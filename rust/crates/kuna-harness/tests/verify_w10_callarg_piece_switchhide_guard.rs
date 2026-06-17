//! [w10-callarg-piece] disclosure pin for the `switchhide.xml` render delta.
//!
//! The call-argument PIECE recovery makes a *declared callee*'s prototype
//! visible to the call site (its parsed `extern` signature is now retyped onto
//! the callee FunctionSymbol, then copied into the call's `FuncCallSpecs` at
//! flow-construction time — `FlowInfo::build_call_specs`, the relocated C++
//! `ActionDefaultParams::apply` copy arm).  `switchhide.xml`'s `<script>` runs
//!     parse line extern void glob2struct(mystruct *ptr);
//! so the `glob2struct` call now recovers its pointer argument and the call
//! renders WITH an argument instead of as a bare `glob2struct()`.
//!
//! This is a render DELTA, not a scored regression:
//!
//!   - baseline (rust-port): `glob2struct();`
//!   - this item:            `glob2struct(PTRSUB(v1,0xffffffffffffffe8));`
//!
//! `switchhide.xml`'s four `<stringmatch>` assertions key on `case`/`default:`/
//! `switch(v1.b)`/`v1.b = 2;` — switch *structuring*, which is NOT ported — so
//! all four FAIL on BOTH baseline and this item; the pass/fail set does not move.
//! Directionally the new render is CLOSER to the C++ oracle (`glob2struct(&v1)`)
//! — it now passes the argument; it is still a raw `PTRSUB(v1,...)` rather than
//! `&v1` only because stack-var typing is incomplete on the Rust side (a separate
//! seam).  This test PINS the new render so the delta is disclosed, not silent:
//! if the call-arg recovery ever stops firing here (regressing the argument off
//! the call) OR if any of the four assertions starts passing (switch structuring
//! landing), this guard fires loudly.

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

/// The four `switchhide.xml` `<stringmatch>` assertion names.  All FAIL on both
/// baseline and this item (switch structuring un-ported); none may flip to
/// `Success` here without a separate, disclosed change.
const SWITCHHIDE_NAMES: &[&str] =
    &["Switch Hide #1", "Switch Hide #2", "Switch Hide #3", "Switch Hide #4"];

#[test]
fn switchhide_callarg_render_delta_pinned() {
    if !x86_64_sla().exists() {
        eprintln!("SKIP: x86-64.sla not built (run `make specs`)");
        return;
    }
    let path = repo_root().join("decompiler/datatests/switchhide.xml");
    let path = path.to_str().expect("utf8 path").to_string();

    let specs = specs_dir_string();
    let files = vec![path];
    let mut out = String::new();
    // The rendered `print C` only lands in `out` via the KUNA_DUMP triage hook
    // (`FunctionTestCollection::run_tests`).  This is the only test in its own
    // test binary, so the process-global env set is race-free here.
    std::env::set_var("KUNA_DUMP", "1");
    run_test_files_with_specs(&files, std::slice::from_ref(&specs), &mut out);
    std::env::remove_var("KUNA_DUMP");

    // The file must still run (a whole-file EXEC-FAILURE is its own regression).
    assert!(
        !out.contains("Execution failed") && !out.contains("Did not apply tests in"),
        "switchhide.xml EXEC-FAILED under call-arg-piece:\n{out}"
    );

    // DISCLOSED DELTA: the `glob2struct` call now recovers its pointer argument.
    // The exact argument form is `PTRSUB(v1,0xffffffffffffffe8)` (raw stack-var,
    // not yet `&v1`).  Pin that the call renders WITH an argument and that the
    // bare `glob2struct();` is gone — so this item's render churn is recorded.
    assert!(
        out.contains("glob2struct(PTRSUB(v1,0xffffffffffffffe8));"),
        "EXPECTED the disclosed call-arg-piece render `glob2struct(PTRSUB(v1,\
         0xffffffffffffffe8));` (the callee proto is recovered, so the call now \
         passes its pointer arg). If this changed, re-disclose the delta.\n\
         Full output:\n{out}"
    );
    assert!(
        !out.contains("  glob2struct();"),
        "The bare `glob2struct();` (no argument) should be GONE — the call-arg \
         recovery makes the callee proto visible and the call passes its arg.\n\
         Full output:\n{out}"
    );

    // NO SCORED MOVEMENT: all four assertions still FAIL (switch structuring is
    // un-ported); none may flip to Success here.
    for name in SWITCHHIDE_NAMES {
        let fail_line = format!("FAIL -- {name}\n");
        let success_line = format!("Success -- {name}\n");
        assert!(
            out.contains(&fail_line),
            "EXPECTED `FAIL -- {name}` (switch structuring un-ported; the render \
             delta does not move the scored set).\nFull output:\n{out}"
        );
        assert!(
            !out.contains(&success_line),
            "`{name}` unexpectedly PASSES — that would be a separate, undisclosed \
             change (switch structuring landing).\nFull output:\n{out}"
        );
    }
}
