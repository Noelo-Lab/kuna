//! Declared function boundaries end-to-end — `docs/re-needs/no-cli-function-boundary-override.md`.
//!
//! Every function boundary kuna knows is one it DERIVED: discovery finds the
//! entries and the extent is the address-contiguous clip `[entry, next_entry)`
//! over an unbounded flow follow. On an obfuscated or packed image, where
//! discovery is exactly what fails, a caller had no way to correct either half —
//! "function F spans `[start,end)`" was not expressible anywhere in the engine.
//!
//! The two-state proof, on the real-ELF path (the XML datatest oracles never
//! reach it):
//!
//!  - **no declaration** (today's default): the function at `0x13c9` follows flow
//!    to its natural end and swallows the 25 leaf functions laid down after it.
//!  - **`function bounds 0x13c9 0x1420 as stage1`**: the same address decompiles
//!    to the ONE call inside the declared 87 bytes, reports 87 as its extent, and
//!    answers to the declared name.
//!
//! Fixture: `kuna-analysis/tests/fixtures/aif_gap_x86_64` — a stripped x86-64 ELF
//! whose `0x13c9` function calls `sub_1129` … `sub_1393` in sequence, so a bound
//! that lands after the first call is trivially observable in the body.
//!
//! ## `.sla` precondition
//!
//! Bootstrapping needs the built x86 `.sla` under `specs/` (gitignored; `make
//! specs`). When it is absent the bootstrap fails; the test prints that and
//! returns early (a specs-less CI is a visible skip, never a false green).

use std::path::PathBuf;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_console::decompile_step::{decompile_one, DecompileSeed};
use kuna_console::engine::{bootstrap_from_object, ConsoleProgram};

/// The merged blob's entry, and a bound just past its first (indirect) call.
const ENTRY: u64 = 0x13c9;
const DECLARED_END: u64 = 0x1420;
/// The last of the 25 leaf calls the unbounded follow swallows — the witness that
/// the bound really cut the flow rather than just relabelling the record.
const PAST_THE_END: &str = "sub_1393";

/// The second function in the fixture, whose `JZ 0x1098` at `0x1081` lets a
/// declared end land exactly on a conditional branch's target
/// (`docs/re-needs/explicit-function-boundary-aborts.md`).
const BRANCH_ENTRY: u64 = 0x1070;
/// An end the `JZ` targets — the cut edge.
const BRANCH_END: u64 = 0x1098;
/// A leaf whose derived extent `[0x1129,0x1141)` ends in `POP RBP; RET` — the
/// witness that declaring a CORRECT extent must not drop the closing instruction.
const LEAF_ENTRY: u64 = 0x1129;
const LEAF_END: u64 = 0x1141;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

/// Bootstrap the fixture. `None` ⇒ specs-less skip.
fn load() -> Option<ConsoleProgram> {
    let root = repo_root();
    let spec_roots = vec![root.join("specs").to_str().unwrap().to_string()];
    let bin = root.join("decompiler/crates/kuna-analysis/tests/fixtures/aif_gap_x86_64");
    let mut prog = match bootstrap_from_object(bin.to_str()?, "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_funcbounds: skipping (bootstrap failed, build `.sla` with \
                 `make specs`): {}",
                e.explain()
            );
            return None;
        }
    };
    prog.commit_pending_analysis().expect("analysis commit succeeds");
    Some(prog)
}

fn code_addr(prog: &ConsoleProgram, vma: u64) -> Address {
    let space = prog.arch().manage().get_default_code_space().expect("code space").clone();
    Address::new(Rc::clone(&space), vma)
}

/// Decompile `ENTRY` under whatever extent the program has declared for it, and
/// return its rendered C.
fn body_at_entry(prog: &mut ConsoleProgram, name: &str) -> String {
    let entry = code_addr(prog, ENTRY);
    let declared = prog.declared_extent(ENTRY);
    let step = decompile_one(
        prog.arch_mut(),
        name,
        entry,
        declared,
        &DecompileSeed::plain(&[], &[]),
        &[],
    );
    let fd = step.result.expect("the function decompiles");
    kuna_decomp::decompile_drive::print_c(prog.arch_mut(), &fd)
}

/// The inventory extent kuna reports for `ENTRY`.
fn reported_extent(prog: &ConsoleProgram) -> u64 {
    prog.function_entries_canonical()
        .iter()
        .find(|e| e.addr.get_offset() == ENTRY)
        .map(|e| e.size)
        .expect("the entry is enumerated")
}

/// With nothing declared, both halves keep the derived answer — the default this
/// feature must not disturb.
#[test]
fn an_undeclared_function_keeps_its_derived_extent_and_unbounded_flow() {
    let Some(mut prog) = load() else { return };
    assert_eq!(prog.declared_extent(ENTRY), 0, "nothing is declared yet");
    // The derived clip: `.text` ends at 0x1673 and `_DT_FINI` is the first entry
    // of the NEXT code section, so the section end wins over the neighbour.
    assert_eq!(reported_extent(&prog), 0x1673 - ENTRY);
    let body = body_at_entry(&mut prog, "sub_13c9");
    assert!(
        body.contains(PAST_THE_END),
        "the unbounded follow must still reach {PAST_THE_END}, got:\n{body}"
    );
}

/// A declared `[start,end)` bounds the flow follow, replaces the reported extent,
/// and names the entry.
#[test]
fn a_declared_boundary_bounds_the_flow_and_the_reported_extent() {
    let Some(mut prog) = load() else { return };
    let addr = code_addr(&prog, ENTRY);
    let name = prog
        .declare_function(addr, Some("stage1"), (DECLARED_END - ENTRY) as i32)
        .expect("the declaration is accepted");
    assert_eq!(name, "stage1");
    assert_eq!(prog.declared_extent(ENTRY), (DECLARED_END - ENTRY) as i32);
    assert_eq!(reported_extent(&prog), DECLARED_END - ENTRY);

    let body = body_at_entry(&mut prog, &name);
    assert!(body.contains("stage1"), "the declared name renders, got:\n{body}");
    assert!(
        !body.contains(PAST_THE_END),
        "flow past the declared end must be cut, got:\n{body}"
    );
}

/// A declared end that cuts a *branch* rather than a fall-through still yields C.
///
/// `sub_1070` opens `CMP RAX,RDI; JZ 0x1098` at `0x1081`, so declaring the extent
/// `[0x1070,0x1098)` puts the conditional's target one byte past the last in-body
/// byte. The walk deliberately never decodes it, and until this was fixed
/// `collect_edges` then asked `target` for the edge head and the whole function
/// died with `Could not find op at target address: (ram,0x00001098)` -- in every
/// mode, so declaring a boundary that cut any real edge produced nothing at all.
#[test]
fn a_declared_end_a_branch_targets_clips_instead_of_aborting() {
    let Some(mut prog) = load() else { return };
    let addr = code_addr(&prog, BRANCH_ENTRY);
    prog.declare_function(addr.clone(), Some("deregister"), (BRANCH_END - BRANCH_ENTRY) as i32)
        .expect("the declaration is accepted");
    let step = decompile_one(
        prog.arch_mut(),
        "deregister",
        addr,
        (BRANCH_END - BRANCH_ENTRY) as i32,
        &DecompileSeed::plain(&[], &[]),
        &[],
    );
    let fd = step.result.expect("the clipped function decompiles");
    let body = kuna_decomp::decompile_drive::print_c(prog.arch_mut(), &fd);
    assert!(body.contains("deregister"), "the declared name renders, got:\n{body}");
    assert!(
        body.contains("Function flows out of bounds"),
        "the cut is reported, not hidden, got:\n{body}"
    );
    assert!(
        body.contains("flows to r0x00001098"),
        "the warning names the cut edge's target, got:\n{body}"
    );
}

/// Declaring the extent kuna itself derived must change nothing — the whole point
/// of an assertion is that asserting the truth is free.
///
/// It was not: the fall-through bound treated the LAST in-body byte as already
/// out of range, so a declared `[entry,end)` never decoded the instruction that
/// starts at `end - 1`. For `sub_1129`, whose 24 bytes end in `POP RBP; RET`,
/// that was the return and everything the structurer needed it for: the body came
/// out as an empty `void sub_1129(void)` carrying a bogus `Function flows out of
/// bounds` warning.
#[test]
fn declaring_the_derived_extent_reproduces_the_undeclared_body() {
    let Some(mut prog) = load() else { return };
    let addr = code_addr(&prog, LEAF_ENTRY);
    let seed = DecompileSeed::plain(&[], &[]);
    let natural = decompile_one(prog.arch_mut(), "sub_1129", addr.clone(), 0, &seed, &[])
        .result
        .expect("the undeclared function decompiles");
    let undeclared = kuna_decomp::decompile_drive::print_c(prog.arch_mut(), &natural);
    assert!(
        undeclared.contains("return"),
        "the fixture's leaf really does return a value, got:\n{undeclared}"
    );

    let size = (LEAF_END - LEAF_ENTRY) as i32;
    prog.declare_function(addr.clone(), None, size).expect("the declaration is accepted");
    let bounded = decompile_one(prog.arch_mut(), "sub_1129", addr, size, &seed, &[])
        .result
        .expect("the declared function decompiles");
    let declared = kuna_decomp::decompile_drive::print_c(prog.arch_mut(), &bounded);

    assert_eq!(undeclared, declared, "asserting the derived extent is inert");
    assert!(
        !declared.contains("out of bounds"),
        "nothing left the extent, so nothing is warned about, got:\n{declared}"
    );
}

/// The declaration is what `load function <name>` and the enumeration resolve
/// against, so an entry declared at an address the image never named is reachable
/// by name afterwards.
#[test]
fn a_declared_entry_becomes_resolvable_by_name() {
    let Some(mut prog) = load() else { return };
    // 0x1500 is interior to the merged blob: discovery does not know it.
    let interior = 0x1500u64;
    assert!(prog.lookup_symbol("hidden_stage").is_none());
    let before = prog.function_entries_canonical().len();
    let addr = code_addr(&prog, interior);
    prog.declare_function(addr, Some("hidden_stage"), 0x80).expect("declared");
    assert_eq!(
        prog.lookup_symbol("hidden_stage").map(|a| a.get_offset()),
        Some(interior),
        "the declared name resolves to its address"
    );
    assert_eq!(prog.function_entries_canonical().len(), before + 1);
    // The neighbour's derived clip tightens against the new entry.
    assert_eq!(reported_extent(&prog), interior - ENTRY);
}

/// Declaring an already-named entry without naming it keeps the name the image
/// gave it: a boundary assertion must not silently rename `main` to `sub_<addr>`.
#[test]
fn an_unnamed_declaration_does_not_overwrite_an_existing_name() {
    let Some(mut prog) = load() else { return };
    let named = prog
        .function_entries_canonical()
        .iter()
        .find(|e| e.name == "_DT_FINI")
        .map(|e| e.addr.get_offset())
        .expect("the fixture carries _DT_FINI");
    let addr = code_addr(&prog, named);
    let back = prog.declare_function(addr, None, 4).expect("declared");
    assert_eq!(back, "_DT_FINI");
    assert_eq!(prog.declared_extent(named), 4);
    assert!(prog
        .function_entries_canonical()
        .iter()
        .any(|e| e.addr.get_offset() == named && e.name == "_DT_FINI"));
}
