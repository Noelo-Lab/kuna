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
//!   - this item:            `glob2struct(PTRSUB(v1,-0x18));`
//!
//! (w10-convert-negconst: the PTRSUB byte-offset constant is a signed
//! (TYPE_INT) read-facing constant, so `pushConstant` now renders it as the
//! two's-complement `-0x18` rather than the full unsigned `0xffffffffffffffe8`
//! bit pattern — the faithful C++ `push_integer(sign=true)` form.)
//!
//! `switchhide.xml`'s four `<stringmatch>` assertions key on `case`/`default:`/
//! `switch(v1.b)`/`v1.b = 2;`.  As of w10-rsp-8-guardfold the `JumpBasic::
//! foldInGuards` guard-fold lands, so #2 (`default:`) now passes; #1/#3/#4 still
//! FAIL (the 9-case count + bitfield switch-var typing need deeper structuring).
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

/// `switchhide.xml` `<stringmatch>` assertions that still FAIL: #1 (the 9-case
/// count), #3 (`switch(v1.b)` — bitfield switch-var typing), #4 (`v1.b = 2;`).
/// These need deeper switch/struct structuring that is still un-ported.
const SWITCHHIDE_FAIL_NAMES: &[&str] =
    &["Switch Hide #1", "Switch Hide #3", "Switch Hide #4"];

/// `switchhide.xml` assertions that now PASS post guard-fold: #2 (`default:`).
/// The `JumpBasic::foldInGuards` guard-fold (w10-rsp-8-guardfold) routes the
/// switch's out-of-range path into the table as a folded `default:` case, so the
/// `default:` keyword now renders.  Disclosed forward movement (not a regression).
const SWITCHHIDE_PASS_NAMES: &[&str] = &["Switch Hide #2"];

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
    // The exact argument form is `PTRSUB(v1,-0x18)` (raw stack-var,
    // not yet `&v1`).  Pin that the call renders WITH an argument and that the
    // bare `glob2struct();` is gone — so this item's render churn is recorded.
    assert!(
        out.contains("glob2struct(PTRSUB(v1,-0x18));"),
        "EXPECTED the disclosed call-arg-piece render `glob2struct(PTRSUB(v1,\
         -0x18));` (the callee proto is recovered, so the call now \
         passes its pointer arg). If this changed, re-disclose the delta.\n\
         Full output:\n{out}"
    );
    assert!(
        !out.contains("  glob2struct();"),
        "The bare `glob2struct();` (no argument) should be GONE — the call-arg \
         recovery makes the callee proto visible and the call passes its arg.\n\
         Full output:\n{out}"
    );

    // #1/#3/#4 still FAIL (deeper switch/struct structuring un-ported).
    for name in SWITCHHIDE_FAIL_NAMES {
        let fail_line = format!("FAIL -- {name}\n");
        let success_line = format!("Success -- {name}\n");
        assert!(
            out.contains(&fail_line),
            "EXPECTED `FAIL -- {name}` (switch/struct structuring un-ported).\n\
             Full output:\n{out}"
        );
        assert!(
            !out.contains(&success_line),
            "`{name}` unexpectedly PASSES — re-disclose the delta.\n\
             Full output:\n{out}"
        );
    }
    // #2 (`default:`) now PASSES via the guard-fold (disclosed forward movement).
    for name in SWITCHHIDE_PASS_NAMES {
        let success_line = format!("Success -- {name}\n");
        let fail_line = format!("FAIL -- {name}\n");
        assert!(
            out.contains(&success_line),
            "EXPECTED `Success -- {name}` — the guard-fold now renders `default:`.\n\
             If this regressed, the fold stopped firing here.\nFull output:\n{out}"
        );
        assert!(
            !out.contains(&fail_line),
            "`{name}` unexpectedly FAILs — the guard-fold default render regressed.\n\
             Full output:\n{out}"
        );
    }
}
