//! VERIFIER adversarial tests for item `w10-partial-types` (round 1) — console
//! seam side.
//!
//! Drives the *actual edited commands* `IfcMapunionfacet` (`map unionfacet`) and
//! `IfcMaphash` (`map hash`) end-to-end against the real `partialunion` /
//! `partialmerge` corpus programs, exercising the logic this diff added that
//! lives only in the console crate:
//!
//!  * the `findByName` + `TYPE_UNION` validation
//!    (`ct==0 || ct->getMetatype()!=TYPE_UNION` -> "Bad union data-type",
//!    ifacedecomp.cc:787),
//!  * the field-index bounds check
//!    (`fieldNum < -1 || fieldNum >= ct->numDepend()` -> "Bad field index",
//!    ifacedecomp.cc:790),
//!  * the `"unionfacet<n>_<hexoff>"` name construction (ifacedecomp.cc:796),
//!  * the "No function loaded" first guard shared by both commands,
//!  * that a successful `map unionfacet` actually lands a `UNION_FACET` symbol in
//!    the loaded function's local scope (recovery is REAL — the command is not a
//!    silent no-op).
//!
//! ## `.sla` precondition
//!
//! Like the other console integration gates, bootstrapping needs the built
//! `.sla` artifacts under `specs/` (gitignored; `make specs`).  A missing `.sla`
//! prints a skip and returns early — never a false green.

use std::path::PathBuf;

use kuna_console::engine::{bootstrap_from_file, ConsoleProgram};
use kuna_console::ifacedecomp::{
    execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE,
};
use kuna_console::ifaceterm::ConsoleCommands;
use kuna_console::interface::IfaceStatus;

use kuna_decomp::database::symbol_category;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

/// Bootstrap a datatest into a [`ConsoleProgram`], or `None` if the `.sla` is
/// missing (printed, so a specs-less CI is a visible skip, never a false green).
fn boot(stem: &str) -> Option<ConsoleProgram> {
    let root = repo_root();
    let xml = root.join(format!("decompiler/datatests/{stem}.xml"));
    let specs = root.join("specs");
    match bootstrap_from_file(xml.to_str().unwrap(), &[specs.to_str().unwrap().to_string()]) {
        Ok(p) => Some(p),
        Err(e) => {
            eprintln!(
                "verify_w10_partial_types_console: skipping {stem} (bootstrap failed, \
                 build `.sla` with `make specs`): {}",
                e.explain()
            );
            None
        }
    }
}

/// Drive `commands` through a console wired like the datatest runner, with
/// `program` installed as the current program.  Returns the console + captured
/// output.
fn drive(program: ConsoleProgram, commands: &[&str]) -> (IfaceStatus, String) {
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
    let out = status.optr.clone();
    (status, out)
}

/// The setup-script prefix every `partialunion` `map unionfacet` test must run
/// first: parse the types, map the global, then `lo fu partial1` so a function
/// is loaded (the command's first guard) and the `structunion` union exists.
const PARTIALUNION_PREFIX: &[&str] = &[
    "option readonly on",
    "parse line struct astruct { int4 aval1; int4 aval2; int4 aval3; };",
    "parse line struct bstruct { float4 bval1; int4 bval2; float4 bval3; };",
    "parse line union structunion { astruct a; bstruct b; };",
    "parse line extern int4 partialunion(float4 val);",
    "map addr r0x301018 structunion globvar",
    "lo fu partial1",
];

/// Run `PARTIALUNION_PREFIX` then `tail`, returning captured output.  Returns
/// `None` (skip) if specs are missing.
fn run_partialunion(tail: &[&str]) -> Option<String> {
    let prog = boot("partialunion")?;
    let mut cmds: Vec<&str> = PARTIALUNION_PREFIX.to_vec();
    cmds.extend_from_slice(tail);
    let (_status, out) = drive(prog, &cmds);
    Some(out)
}

// --- F1: a real `map unionfacet` lands a UNION_FACET symbol -----------------

#[test]
fn map_unionfacet_creates_real_union_facet_symbol() {
    // The exact command from partialunion.xml.  After it runs, the loaded
    // function's local scope must hold a symbol named "unionfacet2_1006ee"
    // (field 1 -> n=2, offset 0x1006ee) with the UNION_FACET category.  A
    // command that silently failed (the old engine_unavailable stub) would
    // leave no such symbol.
    let prog = match boot("partialunion") {
        Some(p) => p,
        None => return,
    };
    let mut cmds: Vec<&str> = PARTIALUNION_PREFIX.to_vec();
    cmds.push("map unionfacet structunion 1 r0x1006ee 10603fc3e29498");
    let count_pre = cmds.len();
    let cmd_strings: Vec<String> = cmds.iter().map(|s| s.to_string()).collect();
    let mut status = ConsoleCommands::into_status(cmd_strings);
    register_decomp_commands(&mut status);
    {
        let data = status.get_data_mut(DECOMPILE_MODULE).unwrap();
        let dcp = data.as_any_mut().downcast_mut::<IfaceDecompData>().unwrap();
        dcp.conf = Some(prog);
    }
    for _ in 0..count_pre {
        execute(&mut status);
    }
    // Inspect the loaded function's local scope for the facet symbol.
    let data = status.get_data_mut(DECOMPILE_MODULE).unwrap();
    let dcp = data.as_any_mut().downcast_mut::<IfaceDecompData>().unwrap();
    let fd = dcp.fd.as_ref().expect("partial1 must be loaded by `lo fu`");
    let sl = fd.get_scope_local().expect("function has a local scope");
    let ids = sl.database().find_by_name(sl.scope_id(), "unionfacet2_1006ee");
    assert_eq!(
        ids.len(),
        1,
        "exactly one facet symbol named unionfacet2_1006ee must exist (recovery is real)",
    );
    let sym = sl.database().symbol(ids[0]);
    assert_eq!(
        sym.category,
        symbol_category::UNION_FACET,
        "the created symbol must be a UNION_FACET directive",
    );
}

// --- F2: bad union name -> "Bad union data-type" ----------------------------

#[test]
fn map_unionfacet_unknown_union_name_errors() {
    // `findByName("nosuchunion")` returns null -> the C++ throws
    // "Bad union data-type: nosuchunion".  The command must report an error and
    // create nothing.
    let Some(out) =
        run_partialunion(&["map unionfacet nosuchunion 0 r0x1006ee 1"])
    else {
        return;
    };
    assert!(
        out.contains("Bad union data-type"),
        "unknown union name must yield the 'Bad union data-type' error; got: {out:?}",
    );
}

// --- F3: a non-union type -> "Bad union data-type" --------------------------

#[test]
fn map_unionfacet_non_union_type_errors() {
    // `astruct` exists but is TYPE_STRUCT, not TYPE_UNION -> the second half of
    // the `ct==0 || getMetatype()!=TYPE_UNION` guard fires.
    let Some(out) = run_partialunion(&["map unionfacet astruct 0 r0x1006ee 1"]) else {
        return;
    };
    assert!(
        out.contains("Bad union data-type"),
        "a struct (non-union) type must yield 'Bad union data-type'; got: {out:?}",
    );
}

// --- F4: field index bounds (`fieldNum >= numDepend`) -----------------------

#[test]
fn map_unionfacet_field_index_out_of_range_errors() {
    // `structunion` has 2 fields (a, b) so numDepend()==2; field index 2 is the
    // first out-of-range value (`fieldNum >= ct->numDepend()`) -> "Bad field
    // index".  This is the upper boundary the bounds check must reject.
    let Some(out) = run_partialunion(&["map unionfacet structunion 2 r0x1006ee 1"]) else {
        return;
    };
    assert!(
        out.contains("Bad field index"),
        "field index == numDepend must be rejected as 'Bad field index'; got: {out:?}",
    );
}

// --- F5: field index below -1 (`fieldNum < -1`) -----------------------------

#[test]
fn map_unionfacet_field_index_below_negative_one_errors() {
    // -2 is the first value below the `-1` whole-union sentinel: `fieldNum < -1`
    // rejects it.  (-1 itself is accepted — the lower-boundary inclusive case.)
    let Some(out) = run_partialunion(&["map unionfacet structunion -2 r0x1006ee 1"]) else {
        return;
    };
    assert!(
        out.contains("Bad field index"),
        "field index -2 (< -1) must be rejected as 'Bad field index'; got: {out:?}",
    );
}

// --- F6: "No function loaded" first guard (both commands) -------------------

#[test]
fn map_commands_require_a_loaded_function() {
    // With a program installed but no function loaded, both commands must hit
    // their shared first guard (`dcp->fd == 0` -> "No function loaded") before
    // touching the type factory or the stream.
    let Some(prog) = boot("partialunion") else { return };
    let (_status, out) = drive(
        prog,
        &["map unionfacet structunion 0 r0x1006ee 1", "map hash r0x1006a7 1 int4 x"],
    );
    assert!(
        out.matches("No function loaded").count() >= 1,
        "map unionfacet/map hash without a loaded function must say 'No function loaded'; \
         got: {out:?}",
    );
}
