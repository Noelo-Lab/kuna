//! Verifier end-to-end real-parity tests for `rport/w10-console-family`.
//!
//! The wave's claim is that 8 of 10 EXEC-FAILURE datatest files now *decode +
//! apply* and that 7 specific assertions newly pass.  A passing assertion is
//! only meaningful if it is a **real** oracle match, not a coincidence of a
//! divergent listing.  These tests drive the live datatest engine
//! (`run_test_files_with_specs`, the exact runner `decomp_test_dbg` uses) on the
//! cluster files and pin:
//!
//!   1. the file no longer EXEC-FAILs (no `Error executing` / `Did not apply`),
//!      so the console map/print/retype/noreturn commands actually ran, and
//!   2. each *named* assertion the wave claims (positive `min=1` lines such as
//!      `ptrrel = &ptr->c;`, `ptr->d = 0;`, and the negative `min=0` guards) is
//!      reported `Success` — i.e. the produced body genuinely contains (or, for
//!      the absence assertions, genuinely lacks) the oracle pattern.
//!
//! The `.sla` specs are build artifacts; if they are absent (a fresh tree that
//! never ran `make specs`) the architecture cannot be built and the test
//! `eprintln!`s a skip note and returns — it never silently green-lights a
//! broken engine, because the assertions below require an explicit `Success`.

use std::path::PathBuf;

use kuna_harness::testfunction::run_test_files_with_specs;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn specs_dir() -> PathBuf {
    repo_root().join("specs")
}

fn datatest(name: &str) -> String {
    repo_root().join("decompiler/datatests").join(name).to_str().unwrap().to_string()
}

/// Returns `Some(results_text)` when the x86-64 spec is available, else `None`
/// (skip).  We probe for a `.sla` under the x86 language dir.
fn run_one(file_name: &str) -> Option<String> {
    let sla = repo_root().join("specs/Ghidra/Processors/x86/data/languages/x86-64.sla");
    if !sla.exists() {
        eprintln!("SKIP: {file_name}: x86-64.sla not built (run `make specs`)");
        return None;
    }
    let specs = specs_dir().to_str().unwrap().to_string();
    let files = vec![datatest(file_name)];
    let mut out = String::new();
    run_test_files_with_specs(&files, std::slice::from_ref(&specs), &mut out);
    Some(out)
}

/// Assert the file ran (no execution failure) and every named assertion is a
/// `Success -- <name>` line in the results.
fn assert_passes(file_name: &str, names: &[&str]) {
    let Some(out) = run_one(file_name) else { return };
    assert!(
        !out.contains("Error executing") && !out.contains("Error parsing"),
        "{file_name} EXEC/PARSE-FAILED — the console command family did not run:\n{out}"
    );
    assert!(
        !out.contains("Did not apply tests in"),
        "{file_name} produced no output (engine bailed before `print`):\n{out}"
    );
    for n in names {
        let needle = format!("Success -- {n}");
        assert!(
            out.contains(&needle),
            "expected `{needle}` (real oracle parity) — not found in:\n{out}"
        );
    }
}

/// pointerrel #4/#5/#6 are POSITIVE (`min=1`) assertions — the exact lines
/// `myptroff ptrrel;`, `ptrrel = &ptr->c;`, `ptrrel = &ptrrel[0x14];`.  They
/// pass only because the `retype v4 myptroff ptrrel` console command genuinely
/// re-typed the symbol and the body printed those identical lines.
#[test]
fn pointerrel_retype_lines_are_real_parity() {
    assert_passes(
        "pointerrel.xml",
        &["Relative pointers #4", "Relative pointers #5", "Relative pointers #6"],
    );
}

/// partialsplit #4 is a POSITIVE (`min=1`) assertion `ptr->d = 0;` — it appears
/// only because `option noreturn __stack_chk_fail` planted the artificial halt
/// so `storeconst` decoded past the noreturn call instead of running into
/// unmapped bytes (the EXEC-FAILURE the wave removed).
#[test]
fn partialsplit_storeconst_line_is_real_parity() {
    assert_passes("partialsplit.xml", &["Partial splitting #4"]);
}

/// revisit #8/#9 are NEGATIVE (`min=0`) guards over the `print raw` SSA listing;
/// they pass only if `print raw` actually produced a non-empty listing (the
/// `Funcdata::printRaw` family this wave ported) AND that listing does not
/// contain the forbidden aliased-revisit lines.  An EXEC-FAILURE (the prior
/// state) would also "lack" the pattern, so the no-EXEC-FAIL check in
/// `assert_passes` is what makes this a meaningful pass.
#[test]
fn revisit_printraw_negative_guards_pass_with_real_listing() {
    assert_passes("revisit.xml", &["Revisit SSA #8", "Revisit SSA #9"]);
}

/// Guard against silent over-claim: the wave does NOT claim full parity on these
/// files.  Pin that the divergent positive assertions (pointerrel #1/#2/#3, the
/// `ADJ(ptrrel)->c` relative-pointer macro the engine has not yet ported) are
/// still reported `FAIL`, so a future regression that fakes them would be caught.
#[test]
fn pointerrel_unported_body_still_fails_honestly() {
    let Some(out) = run_one("pointerrel.xml") else { return };
    for n in ["Relative pointers #1", "Relative pointers #2", "Relative pointers #3"] {
        assert!(
            out.contains(&format!("FAIL -- {n}")),
            "{n} unexpectedly Success — the ADJ relative-pointer body is not yet ported; \
             a Success here would mean the listing was faked:\n{out}"
        );
    }
}
