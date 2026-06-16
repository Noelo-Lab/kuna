//! [w10-spacebase-typing] verifier no-regression guard for the SWITCH/jumptable
//! datatests.
//!
//! The W10 spacebase-typing keystone makes `castFixupPtrsub` keep a valid
//! stack-frame `PTRSUB(sp, off)` instead of tearing it back into a raw
//! `RSP + off` INT_ADD.  That changes type/cast behavior on a BROAD class of
//! functions, and the jumptable/switch recovery files are the most sensitive to
//! any drift in stack-pointer typing (extrapop / stack-placeholder / jump-table
//! base recovery all read stack types).  This guard runs the real `switchind.xml`
//! datatest through the engine and pins the EXACT set of assertions that pass on
//! the parent `rust-port` baseline — a regression in any one of them (or a
//! whole-file EXEC-FAILURE) fails here, independently of the aggregate 298 count.
//!
//! Baseline (rust-port, parent of this item): `switchind.xml` yields
//! `Success` for #1-7, #9-14 (13 assertions) and `FAIL` for #8, #15, #16.  This
//! test asserts every one of those 13 is still `Success` and the file still runs.

use std::path::PathBuf;

use kuna_harness::testfunction::run_test_files_with_specs;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn x86_64_sla() -> PathBuf {
    repo_root().join("specs/Ghidra/Processors/x86/data/languages/x86-64.sla")
}

fn specs_dir_string() -> String {
    repo_root().join("specs").to_str().unwrap().to_string()
}

/// The assertions that pass on the `rust-port` baseline for `switchind.xml`.
/// Every one of these MUST remain `Success` after the spacebase-typing item.
const SWITCHIND_MUST_PASS: &[&str] = &[
    "Switch Indirect #1",
    "Switch Indirect #2",
    "Switch Indirect #3",
    "Switch Indirect #4",
    "Switch Indirect #5",
    "Switch Indirect #6",
    "Switch Indirect #7",
    "Switch Indirect #9",
    "Switch Indirect #10",
    "Switch Indirect #11",
    "Switch Indirect #12",
    "Switch Indirect #13",
    "Switch Indirect #14",
];

#[test]
fn w10_spacebase_typing_switchind_passing_set_unregressed() {
    if !x86_64_sla().exists() {
        eprintln!("SKIP: x86-64.sla not built (run `make specs`)");
        return;
    }
    let path = repo_root().join("decompiler/datatests/switchind.xml");
    let path = path.to_str().expect("utf8 path").to_string();

    let specs = specs_dir_string();
    let files = vec![path.clone()];
    let mut out = String::new();
    run_test_files_with_specs(&files, std::slice::from_ref(&specs), &mut out);

    // The file must actually run (a whole-file EXEC-FAILURE would silently turn
    // every assertion into a non-Success and is itself a switch-file regression).
    assert!(
        !out.contains("Execution failed") && !out.contains("Did not apply tests in"),
        "switchind.xml EXEC-FAILED under spacebase-typing — switch recovery regressed:\n{out}"
    );

    // Every baseline-passing assertion must still print `Success -- <name>` and
    // must NOT appear as `FAIL -- <name>`.
    for name in SWITCHIND_MUST_PASS {
        let success_line = format!("Success -- {name}\n");
        let fail_line = format!("FAIL -- {name}\n");
        assert!(
            out.contains(&success_line),
            "REGRESSION: `{name}` no longer passes under spacebase-typing.\n\
             Expected `Success -- {name}`. Full output:\n{out}"
        );
        assert!(
            !out.contains(&fail_line),
            "REGRESSION: `{name}` now FAILs under spacebase-typing.\nFull output:\n{out}"
        );
    }
}
