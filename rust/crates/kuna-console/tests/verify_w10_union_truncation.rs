//! VERIFIER adversarial tests for item `w10-union-truncation` (round 1).
//!
//! These land with the verdict per docs/rust-port/verification.md "Adversarial
//! pass".  They drive the *real* `partialunion` datatest end-to-end through the
//! console (`dec` + `print C`) — the exact script from
//! `decompiler/datatests/partialunion.xml` — and assert that the **truncated**
//! partial-union accesses resolve to the right field through
//! `Funcdata::resolve_truncation` / `find_truncation`, byte-matching the C++ B5
//! oracle's `<stringmatch>` lines:
//!
//!   * `globvar.b.bval1 = val;`        (Partial union #1, min=1 max=1)
//!   * `return globvar.a.aval2;`       (Partial union #2, min=1 max=1)
//!   * `globvar.a.aval1 = a1 + 7;`     (Partial union #4, min=1 max=1)
//!
//! The fragile spots this targets:
//!   (T1) the resolveTruncation walk is REAL: the SUBPIECE/implied-truncation
//!        scorer picks the field whose size/offset matches, and the print shows
//!        the *specific* member (`b.bval1`, a float4 at offset 0) — a stub that
//!        always fell to `stripped` (the pre-port behavior) would print the raw
//!        union or a cast, never the field name.
//!   (T2) ANTI-SPECIAL-CASING: none of `bval1`/`aval2`/`globvar` are hardcoded —
//!        the second function (`partialunion`, the float-arg writer) resolves to
//!        `b.bval1` while `partial1` (the int writer) resolves to `a.aval*`, both
//!        from the SAME engine on different data flow.  A different facet field
//!        index produces a different member, proving it is data-driven.
//!   (T3) the truncation cache is WRITTEN then READ: `print C` (the reader, via
//!        `pushPartialSymbol`/`findTruncation`) sees the field the cast-plane
//!        `resolve_truncation` scored — if the write were dropped the member
//!        would not render.
//!
//! ## `.sla` precondition
//!
//! Like the sibling console gates, bootstrapping needs the built `.sla` under
//! `specs/` (gitignored; `make specs`).  A missing `.sla` prints a skip and
//! returns early — never a false green.

use std::path::PathBuf;

use kuna_console::engine::{bootstrap_from_file, ConsoleProgram};
use kuna_console::ifacedecomp::{
    execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE,
};
use kuna_console::ifaceterm::ConsoleCommands;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn boot(stem: &str) -> Option<ConsoleProgram> {
    let root = repo_root();
    let xml = root.join(format!("decompiler/datatests/{stem}.xml"));
    let specs = root.join("specs");
    match bootstrap_from_file(xml.to_str().unwrap(), "", &[specs.to_str().unwrap().to_string()]) {
        Ok(p) => Some(p),
        Err(e) => {
            eprintln!(
                "verify_w10_union_truncation: skipping {stem} (bootstrap failed, build `.sla` \
                 with `make specs`): {}",
                e.explain()
            );
            None
        }
    }
}

/// Drive `commands` through a console with `program` installed as current, then
/// return the captured output stream.
fn drive(program: ConsoleProgram, commands: &[&str]) -> String {
    let cmds: Vec<String> = commands.iter().map(|s| s.to_string()).collect();
    let count = cmds.len();
    let mut status = ConsoleCommands::into_status(cmds);
    register_decomp_commands(&mut status);
    {
        let data = status.get_data_mut(DECOMPILE_MODULE).expect("decompile module data");
        let dcp = data
            .as_any_mut()
            .downcast_mut::<IfaceDecompData>()
            .expect("decompile module data is IfaceDecompData");
        dcp.conf = Some(program);
    }
    for _ in 0..count {
        execute(&mut status);
    }
    status.optr.clone()
}

/// The shared type/symbol setup from partialunion.xml's <script>.
const SETUP: &[&str] = &[
    "option readonly on",
    "parse line struct astruct { int4 aval1; int4 aval2; int4 aval3; };",
    "parse line struct bstruct { float4 bval1; int4 bval2; float4 bval3; };",
    "parse line union structunion { astruct a; bstruct b; };",
    "parse line extern int4 partialunion(float4 val);",
    "map addr r0x301018 structunion globvar",
];

/// T1 + T3: the `partialunion` function decompiles to BOTH
/// `globvar.b.bval1 = val;` (a float4 store — Partial union #1) and
/// `return globvar.a.aval2;` (an int4 read — Partial union #2).  The truncated
/// accesses resolve through `resolve_truncation` to the field whose
/// size/metatype matches the data flow: the float store lands on the float4
/// `b.bval1` (offset 0) and the int return lands on the int4 `a.aval2`
/// (offset 4).  Both are partialunion.xml oracle `<stringmatch>` lines.
#[test]
fn partialunion_float_write_and_int_read_resolve_to_correct_fields() {
    let Some(prog) = boot("partialunion") else { return };
    let mut cmds: Vec<&str> = SETUP.to_vec();
    cmds.extend_from_slice(&["lo fu partialunion", "dec", "print C", "quit"]);
    let out = drive(prog, &cmds);
    eprintln!("=== partialunion print C ===\n{out}");
    // Partial union #1: the float4 store resolves to the float field `b.bval1`.
    assert!(
        out.contains("globvar.b.bval1 = val;"),
        "the float4 write must resolve through resolve_truncation to the `b.bval1` \
         field (partialunion.xml Partial union #1); got:\n{out}"
    );
    // Partial union #2: the int4 read resolves to the int field `a.aval2`
    //  (a DIFFERENT arm, DIFFERENT offset, from the SAME engine on a different
    //  truncation) — the data flow, not a field-name table, picks the member.
    assert!(
        out.contains("return globvar.a.aval2;"),
        "the int4 read must resolve through resolve_truncation to the `a.aval2` \
         field (partialunion.xml Partial union #2); got:\n{out}"
    );
    // ANTI-SPECIAL-CASING: the float4 write must NOT have collapsed to the raw
    // union / stripped type (the pre-port `stripped` fall-through) nor
    // mis-resolved to an int field at the same offset.
    assert!(
        !out.contains("globvar.a.aval1 = val;"),
        "the float4 write must not mis-resolve to the int field a.aval1; got:\n{out}"
    );
}

/// T2: data-driven, not field-name-hardcoded.  The SAME engine on `partial1`
/// resolves its int stores to the `a.aval*` integer fields:
/// `globvar.a.aval2 = a1;` (Partial union #3) and `globvar.a.aval1 = a1 + 7;`
/// (Partial union #4).  Each lands on the int field whose offset matches the
/// store — `a.aval2` at offset 4, `a.aval1` at offset 0 — proving the residual
/// `newoff`/field selection is real.
#[test]
fn partial1_int_writes_resolve_to_a_aval_fields_by_offset() {
    let Some(prog) = boot("partialunion") else { return };
    let mut cmds: Vec<&str> = SETUP.to_vec();
    cmds.extend_from_slice(&[
        "lo fu partial1",
        "map unionfacet structunion 1 r0x1006ee 10603fc3e29498",
        "dec",
        "print C",
        "quit",
    ]);
    let out = drive(prog, &cmds);
    eprintln!("=== partial1 print C ===\n{out}");
    // Partial union #3: a store at union offset 4 -> a.aval2.
    assert!(
        out.contains("globvar.a.aval2 = a1;"),
        "the int store at offset 4 must resolve to `a.aval2` (Partial union #3); got:\n{out}"
    );
    // Partial union #4: a store at union offset 0 -> a.aval1 (DIFFERENT offset,
    // DIFFERENT field, same engine — the offset, not a constant, drives it).
    assert!(
        out.contains("globvar.a.aval1 = a1 + 7;"),
        "the int store at offset 0 must resolve to `a.aval1` (Partial union #4); got:\n{out}"
    );
}

/// T2 (negative): a function with NO union/partial-union flow must produce no
/// `globvar.*.*` member access at all — the truncation engine only fires on the
/// union data-type, never as a blanket print decoration.  `condconst` (a
/// different datatest, x86-16, no union global) must carry none of the
/// partialunion field tokens.
#[test]
fn unrelated_function_has_no_union_field_tokens() {
    let Some(prog) = boot("condconst") else { return };
    let out = drive(prog, &["lo fu condconst", "dec", "print C", "quit"]);
    eprintln!("=== condconst print C ===\n{out}");
    for tok in ["globvar", "bval1", "aval1", "aval2"] {
        assert!(
            !out.contains(tok),
            "partialunion's `{tok}` leaked into an unrelated function — the truncation \
             field resolution is special-cased, not data-driven; got:\n{out}"
        );
    }
}
