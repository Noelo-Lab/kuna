//! End-to-end two-pass gate for `picbase` — the i386 PIC base register folded
//! into the cross-reference index (P1 code/data partition, the on-demand xref
//! query).
//!
//! Fixture: `picbase_i386` (`picbase_i386.s` carries the source and the build
//! line), reduced from crackmes.one/5ab77f6333c5d40ad448ca52 where
//! `kuna strings --json --filter Password` reported `xrefs_count: 0` and an
//! empty `functions` list for the prompt at 0x8049127 — as it did for all 23 of
//! that image's strings, because in position-independent i386 code the address
//! is the sum of a GOT pointer the program materialises at run time and a
//! displacement, and the literal is nowhere in the file.
//!
//! ## The two passes
//!
//! * **option OFF (the bug):** every literal in the fixture reports being
//!   referenced by nothing, exactly as before the feature.
//! * **default (the fix):** the prompt is referenced from the function that
//!   establishes the base, the banner from the one that only inherits it, the
//!   trailer from the one that takes its base from `__x86.get_pc_thunk.bx`, and
//!   the global is both read and written.
//!
//! The third and fourth tests are the ones that keep this from inventing
//! ownership, which is the failure mode no parity gate in this repo could see:
//! `secret` is formed by a `lea` off a register the function loaded with
//! `0x11111111`, so folding a base there would attribute a string to a function
//! that never touches it, and `unused` is referenced by nothing at all.
//!
//! ## `.sla` precondition
//!
//! Bootstrapping needs the built `x86` `.sla` under `specs/` (gitignored; `make
//! specs`). When it is absent the bootstrap fails; the test prints that and
//! returns early (a specs-less CI is a visible skip, never a false green).

use std::path::PathBuf;

use kuna_analysis::listing::xrefs::{self, XrefIndex, XrefKind};
use kuna_console::engine::{bootstrap_from_object, ConsoleProgram};

/// `"Password: "` — formed by the function that runs the base idiom itself.
const PROMPT: u64 = 0x804a000;
/// `"kuna picbase fixture"` — formed by a function that only inherits the base.
const BANNER: u64 = 0x804a00b;
/// `"picbase must not claim this"` — formed off a register holding 0x11111111.
const SECRET: u64 = 0x804a020;
/// `"thunked reference"` — formed off `__x86.get_pc_thunk.bx`'s answer.
const TRAILER: u64 = 0x804a03c;
/// `"never referenced anywhere"`.
const UNUSED: u64 = 0x804a04e;
/// The `.data` word `inherits` reads and writes through the base.
const COUNTER: u64 = 0x804b074;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn fixture() -> PathBuf {
    repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/picbase_i386")
}

/// Bootstrap the fixture with `picbase` in the requested state and build the
/// index `kuna xrefs` / `kuna strings` answer out of. `None` is a visible skip
/// when the `.sla` is missing.
fn index(picbase: bool) -> Option<(ConsoleProgram, Vec<u8>, XrefIndex)> {
    let bin = fixture();
    let specs = repo_root().join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];
    let mut prog = match bootstrap_from_object(bin.to_str().unwrap(), "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_picbase: skipping (bootstrap failed, build `.sla` with \
                 `make specs`): {}",
                e.explain()
            );
            return None;
        }
    };
    prog.arch_mut()
        .set_kuna_option("picbase", if picbase { "on" } else { "off" })
        .expect("picbase flips");
    prog.commit_pending_analysis().expect("analysis commit succeeds");

    let bytes = std::fs::read(&bin).expect("fixture readable");
    let file = object::File::parse(&*bytes).expect("fixture parses");
    let seeds: Vec<u64> =
        prog.function_entries_canonical().iter().map(|e| e.addr.get_offset()).collect();
    let idx = xrefs::build(&file, prog.arch(), prog.arch().translate(), &seeds);
    Some((prog, bytes, idx))
}

/// The names of the functions that reference `vma`, as `kuna strings` reports
/// them: the walk's own attribution first, then the engine's inventory.
fn owners(prog: &ConsoleProgram, idx: &XrefIndex, vma: u64) -> Vec<String> {
    let mut out: Vec<String> = idx
        .refs_to(vma)
        .iter()
        .filter_map(|r| idx.function_containing(r.from).or_else(|| prog.find_entry_at(r.from).map(|e| e.addr.get_offset())))
        .filter_map(|e| prog.find_entry_at(e).map(|e| e.name))
        .collect();
    out.sort();
    out.dedup();
    out
}

/// BEFORE (`--option picbase off`): nothing in the image references anything.
#[test]
fn every_literal_is_referenced_by_nothing_with_the_option_off() {
    let Some((_prog, _bytes, idx)) = index(false) else {
        return;
    };
    for (vma, what) in [
        (PROMPT, "the prompt"),
        (BANNER, "the banner"),
        (TRAILER, "the trailer"),
        (COUNTER, "the counter"),
    ] {
        assert!(
            idx.refs_to(vma).is_empty(),
            "with picbase off {what} at {vma:#x} must be referenced by nothing — \
             that is the bug this option restores; got {:?}",
            idx.refs_to(vma)
        );
    }
}

/// AFTER (default): each of the three ways a function comes by the base is
/// resolved, and the global is both read and written.
#[test]
fn base_relative_formations_are_references_by_default() {
    let Some((prog, _bytes, idx)) = index(true) else {
        return;
    };
    assert_eq!(
        owners(&prog, &idx, PROMPT),
        vec!["_start".to_string()],
        "the prompt is formed by the function that runs the base idiom itself"
    );
    assert_eq!(
        owners(&prog, &idx, BANNER),
        vec!["inherits".to_string()],
        "the banner is formed by a function that never writes the base register, \
         so it uses the one its caller left — the case kuna's own function \
         inventory forces on the filing crackme by splitting its prompt routine"
    );
    assert_eq!(
        owners(&prog, &idx, TRAILER),
        vec!["thunked".to_string()],
        "the trailer is formed off __x86.get_pc_thunk.bx, the out-of-line idiom \
         GCC emits instead of the inline call/pop"
    );

    let kinds: Vec<XrefKind> = idx.refs_to(COUNTER).iter().map(|r| r.kind).collect();
    assert!(
        kinds.contains(&XrefKind::Read) && kinds.contains(&XrefKind::Write),
        "the global is loaded from and stored to through the base; got {kinds:?}"
    );
}

/// The property that keeps this from inventing ownership: a function that uses
/// the base register for its own purposes contributes nothing.
#[test]
fn a_function_that_clobbers_the_base_register_claims_nothing() {
    let Some((_prog, _bytes, idx)) = index(true) else {
        return;
    };
    assert!(
        idx.refs_to(SECRET).is_empty(),
        "`clobbers` forms this address off a register it loaded with 0x11111111, \
         so folding the module base there would attribute a string to a function \
         that never touches it; got {:?}",
        idx.refs_to(SECRET)
    );
}

/// And a literal nothing forms the address of stays unreferenced: the pass adds
/// edges it can prove, not edges that would be convenient.
#[test]
fn an_unreferenced_literal_stays_unreferenced() {
    let Some((_prog, _bytes, idx)) = index(true) else {
        return;
    };
    assert!(
        idx.refs_to(UNUSED).is_empty(),
        "nothing forms this address; got {:?}",
        idx.refs_to(UNUSED)
    );
}

/// The two arms differ only by ADDED edges: the option never removes or
/// re-points a reference the constant scan already found.
#[test]
fn the_option_only_ever_adds_edges() {
    let (Some((_, _, off)), Some((_, _, on))) = (index(false), index(true)) else {
        return;
    };
    for vma in 0x8049000u64..0x804b400 {
        let before: Vec<(u64, XrefKind)> =
            off.refs_to(vma).iter().map(|r| (r.from, r.kind)).collect();
        let after: Vec<(u64, XrefKind)> =
            on.refs_to(vma).iter().map(|r| (r.from, r.kind)).collect();
        for edge in &before {
            assert!(
                after.contains(edge),
                "picbase dropped the edge {edge:?} into {vma:#x}; it may only add"
            );
        }
    }
}
