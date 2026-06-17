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
//! This is a render DELTA, not a scored regression for the call-arg form:
//!
//!   - baseline (rust-port):       `glob2struct();`
//!   - w10-callarg-piece:          `glob2struct(PTRSUB(v1,0xffffffffffffffe8));`
//!   - w10-rsp-L4L5-stackframe:    `glob2struct(&v1);`  (THIS item — the oracle form)
//!
//! The W10 RSP L4/L5 stack-frame render closes the seam: the input-effect marking
//! (`setInputVarnode`'s `funcp.hasEffect` tail) removes the spurious `//rsp` local,
//! so the call argument resolves to a true spacebase reference `PTRSUB(spacebase,
//! -0x18)`; `ActionNameVars::linkSpacebaseSymbol`'s namerec rename then names the
//! covered stack Symbol `v1` (coreaction.cc:3016 + 3087-3094) and the `&symbol`
//! attach renders it `&v1` — matching the C++ oracle.  The body member accesses on
//! the SAME renamed Symbol now render `v1.b`.
//!
//! `switchhide.xml`'s four `<stringmatch>` assertions key on `case`/`default:`/
//! `switch(v1.b)`/`v1.b = 2;`.  On the integrated tree (guard-fold + RSP L4/L5 +
//! &v1-render, with ActionReturnSplit live):
//!   - #2 (`default:`) PASSES via `JumpBasic::foldInGuards` (w10-rsp-8-guardfold).
//!   - #3 (`switch(v1.b)`) and #4 (`v1.b = 2;`) PASS off the stack-var naming this
//!     item lands.
//!   - #1 (the 9-case count) still FAILs: it needs deeper switch case-arm
//!     structuring (an independent jumptable seam, not this wave).
//! This test PINS that exact #1-fail, #2/#3/#4-pass split plus the oracle-correct
//! `glob2struct(&v1)` call-arg render, so any future drift (the arg regressing off
//! `&v1`, or the structuring landing and flipping #1) fires loudly.

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

/// `switchhide.xml` `<stringmatch>` assertions that still FAIL: #1 (the 9-case
/// count).  This needs deeper switch case-arm structuring that is still un-ported.
const SWITCHHIDE_FAIL_NAMES: &[&str] = &["Switch Hide #1"];

/// `switchhide.xml` assertions that PASS on the integrated tree: #2 (`default:`)
/// via the guard-fold, #3 (`switch(v1.b)`) and #4 (`v1.b = 2;`) off the stack-var
/// naming this item lands.  Disclosed forward movement (not a regression).
const SWITCHHIDE_PASS_NAMES: &[&str] =
    &["Switch Hide #2", "Switch Hide #3", "Switch Hide #4"];

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

    // ORACLE-CORRECT RENDER: the `glob2struct` call passes its recovered pointer
    // argument as the named typed stack reference `&v1` (the C++ oracle form), not
    // the raw `PTRSUB(...)` and not the bare `glob2struct()`.
    assert!(
        out.contains("glob2struct(&v1);"),
        "EXPECTED the oracle call-arg render `glob2struct(&v1);` (the stack Symbol \
         is named `v1` by the linkSpacebaseSymbol namerec rename and the `&symbol` \
         reference is attached). If this changed, re-disclose the delta.\n\
         Full output:\n{out}"
    );
    assert!(
        !out.contains("  glob2struct();"),
        "The bare `glob2struct();` (no argument) should be GONE — the call-arg \
         recovery makes the callee proto visible and the call passes its arg.\n\
         Full output:\n{out}"
    );
    assert!(
        !out.contains("PTRSUB(v1") && !out.contains("PTRSUB(RSP"),
        "The raw `PTRSUB(...)` call-arg intermediate should be GONE — the \
         stack-frame render finishes it as `&v1`.\nFull output:\n{out}"
    );
    // The body member accesses on the SAME renamed Symbol render `v1.b`.
    assert!(
        out.contains("v1.b = 2;") && out.contains("switch(v1.b)"),
        "EXPECTED the body to render `v1.b = 2;` and `switch(v1.b)` (the shared \
         renamed Symbol).\nFull output:\n{out}"
    );

    // #1 (the 9-case count) still FAILs — switch case-arm structuring is independent.
    for name in SWITCHHIDE_FAIL_NAMES {
        let fail_line = format!("FAIL -- {name}\n");
        assert!(
            out.contains(&fail_line),
            "EXPECTED `FAIL -- {name}` (switch case-arm structuring un-ported; this \
             item lands stack-var naming, not structuring).\nFull output:\n{out}"
        );
    }
    // #2 (`default:`) via guard-fold, #3/#4 off stack-var naming now PASS.
    for name in SWITCHHIDE_PASS_NAMES {
        let success_line = format!("Success -- {name}\n");
        let fail_line = format!("FAIL -- {name}\n");
        assert!(
            out.contains(&success_line),
            "EXPECTED `Success -- {name}` (guard-fold default render and/or the \
             stack Symbol named `v1`).  If this regressed, the fold/naming stopped \
             firing here.\nFull output:\n{out}"
        );
        assert!(
            !out.contains(&fail_line),
            "`{name}` unexpectedly FAILs — the guard-fold/naming render regressed.\n\
             Full output:\n{out}"
        );
    }
}
