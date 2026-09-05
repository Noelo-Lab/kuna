//! What `XrefIndex::has_indirect_calls` means, pinned on a real image.
//!
//! The field answers one question a reference index otherwise cannot: a
//! computed call files no Call edge, so "this function calls nothing" and "this
//! function's callee is decided at run time" look identical from
//! `refs_from_function`. `kuna decompile-graph` reports it per function for
//! exactly that reason.
//!
//! The claim is narrow — `CALLIND`, not any indirection — and the third test is
//! the one that keeps it narrow: a PLT stub's `JMP qword ptr [0x601000]` is an
//! indirect *branch*, and reading it as a computed call would mark most of a
//! dynamically linked program's import stubs as having unknowable callees when
//! their target is in fact the one thing about them that is known.
//!
//! Fixture: `fauxware` (ELF x86-64), the repo's smallest linked image with all
//! three shapes in it. Bootstrapping needs the built `x86` `.sla` under
//! `specs/` (gitignored; `make specs`); when it is absent the test prints that
//! and returns early.

use std::path::PathBuf;

use kuna_analysis::listing::xrefs::{self, XrefIndex};
use kuna_console::engine::bootstrap_from_object;

/// `__do_global_ctors_aux` — its loop body is one `CALL RAX` (`ff d0`).
const COMPUTED_CALL: u64 = 0x400880;
/// `main` — five call sites, every one of them a direct `CALL <vma>`.
const DIRECT_CALLS_ONLY: u64 = 0x40071d;
/// The `puts` PLT stub — `JMP qword ptr [0x601000]`, a `BRANCHIND`.
const FORWARDING_VENEER: u64 = 0x400510;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

/// The index `kuna xrefs` answers out of, or `None` as a visible skip.
fn index() -> Option<XrefIndex> {
    let bin = repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/fauxware");
    let specs = repo_root().join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];
    let mut prog = match bootstrap_from_object(bin.to_str().unwrap(), "", &spec_roots) {
        Ok(prog) => prog,
        Err(e) => {
            eprintln!(
                "verify_indirect_callers: skipping (bootstrap failed, build `.sla` \
                 with `make specs`): {}",
                e.explain()
            );
            return None;
        }
    };
    prog.commit_pending_analysis().expect("analysis commit succeeds");
    let bytes = std::fs::read(&bin).expect("fixture readable");
    let file = object::File::parse(&*bytes).expect("fixture parses");
    let seeds: Vec<u64> =
        prog.function_entries_canonical().iter().map(|e| e.addr.get_offset()).collect();
    Some(xrefs::build(&file, prog.arch(), prog.arch().translate(), &seeds))
}

#[test]
fn a_computed_call_is_reported() {
    let Some(idx) = index() else { return };
    assert!(
        idx.has_indirect_calls(COMPUTED_CALL),
        "__do_global_ctors_aux's `CALL RAX` was not reported"
    );
    // And it really has no static callee to have been reported as one instead.
    assert!(
        idx.refs_from_function(COMPUTED_CALL).is_empty(),
        "the computed call filed a target edge after all"
    );
}

#[test]
fn direct_calls_alone_are_not_reported() {
    let Some(idx) = index() else { return };
    assert!(
        !idx.has_indirect_calls(DIRECT_CALLS_ONLY),
        "main has five direct call sites and no computed one"
    );
    assert!(!idx.refs_from_function(DIRECT_CALLS_ONLY).is_empty(), "main's callees vanished");
}

#[test]
fn an_indirect_branch_is_not_a_computed_call() {
    let Some(idx) = index() else { return };
    assert!(
        !idx.has_indirect_calls(FORWARDING_VENEER),
        "the puts PLT stub's `JMP qword ptr [...]` was read as a computed call"
    );
    // The veneer's destination is not unknown, it is the slot it reads.
    assert_eq!(idx.veneer_slot(FORWARDING_VENEER), Some(0x601000));
}
