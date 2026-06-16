//! Independent-verifier adversarial end-to-end tests for `rport/w10-indproto`
//! (the `getTypeCode(PrototypePieces)` / `TypeCode::setPrototype` keystone).
//!
//! The item ports the nested function-pointer `buildType` path so a struct field
//! like `void (*peek)(int4 a)` parses and the resulting `TypeCode` interns,
//! compares, and decodes.  Before the item, `indproto.xml` EXEC-FAILed with
//!   `Error in C syntax: ... getTypeCode(PrototypePieces) not yet ported`
//! and was counted as not-applied (corpus 672 applied / 271 passing); after, all
//! 3 of its assertions apply (675 applied / 272 passing, +1 = "Indirect
//! prototype #1") with the passing SET a strict superset (zero regression).
//!
//! These tests drive the *real* datatest engine (`run_test_files_with_specs`,
//! the exact runner `decomp_test_dbg` uses) so the whole
//! `FunctionModifier::modType -> getTypeCode(PrototypePieces) -> setPrototype ->
//! updateAllTypes -> TypeCode::compare/compareDependency` chain runs over real
//! interned types.  They pin the spots the hunt list flagged as most fragile for
//! this item:
//!
//!   T1 (regression fence): the real `indproto.xml` no longer EXEC-FAILs and
//!      "Indirect prototype #1" (the negative `Exceeded maximum restarts` guard)
//!      is a genuine `Success`.  This is the exec-failure-closed claim; a future
//!      regression that re-breaks the path would flip this red.
//!
//!   T2 (varargs nested fn-pointer): a struct field `void (*peek)(int4 a, ...)`.
//!      The diff hardcodes `first_var_arg_slot: -1` in the `modType` arm with the
//!      claim that the C++ `paramlist.back() == 0` check is dead code there
//!      (the trailing null is popped in `CParse::newFunc` into `dotdotdot`
//!      *before* `modType` runs).  This asserts the varargs declarator parses and
//!      DECODES without an EXEC-FAILURE — exactly the faithful C++ behavior (C++
//!      also keeps `firstVarArgSlot == -1` for this nested path and does not
//!      crash).
//!
//!   T3 (multi-param, non-void return): a struct field `int4 (*get)(int4 a,
//!      char *b)`.  This exercises `getInTypes` over *two* params, the non-void
//!      output-type assignment in `setPrototype`/`updateAllTypes`, and (because a
//!      second distinct method field is present) the newly-activated
//!      `TypeCode::compare` / `compareDependency` per-parameter + output-type
//!      recursion that were `SEAM(W6)` `Err` stubs before this item.  Asserts the
//!      whole thing DECODES (no EXEC-FAILURE / parse error).
//!
//! The `.sla` specs are build artifacts; if the x86-64 spec is absent the test
//! `eprintln!`s a skip note and returns (it never silently green-lights, because
//! the positive assertions require an explicit `Success`).

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

/// Run a single datatest file path through the real engine, returning the
/// results text, or `None` when the spec is unavailable (skip).
fn run_file(path: &str) -> Option<String> {
    if !x86_64_sla().exists() {
        eprintln!("SKIP: {path}: x86-64.sla not built (run `make specs`)");
        return None;
    }
    let specs = specs_dir_string();
    let files = vec![path.to_string()];
    let mut out = String::new();
    run_test_files_with_specs(&files, std::slice::from_ref(&specs), &mut out);
    Some(out)
}

/// The shared `<binaryimage>` from `indproto.xml` (the two indirect calls + the
/// `Peek\0Get\0` rodata).  We reuse the real bytes so the adversarial variants
/// actually decode a real function; only the `<script>` declarations differ.
const BINARYIMAGE: &str = r#"<binaryimage arch="x86:LE:64:default:gcc">
<bytechunk space="ram" offset="0x100771" readonly="true">
  554889e54883ec10897dfc8975f848
8955f08b45fc89c79090909090837df8
647517488b45f0488b00ffd0488d3d53
010000e838feffffeb16488b45f0488b
4008ffd0488d3d40010000e820feffff
90c9c3
</bytechunk>
<bytechunk space="ram" offset="0x1008f6" readonly="true">
  5065656b0047657400
</bytechunk>
<symbol space="ram" offset="0x100771" name="indproto"/>
<symbol space="ram" offset="0x1005e0" name="puts"/>
</binaryimage>"#;

/// Build a full `<decompilertest>` document for the adversarial variants from a
/// custom `struct methods {...}` body, reusing indproto's image + driver.
fn build_doc(struct_body: &str, stringmatches: &str) -> String {
    format!(
        "<decompilertest>\n{BINARYIMAGE}\n<script>\n  \
         <com>parse line struct methods {{ {struct_body} }};</com>\n  \
         <com>parse line extern void indproto(int4 a,int4 b,methods *ptr);</com>\n  \
         <com>lo fu indproto</com>\n  \
         <com>decompile</com>\n  \
         <com>print C</com>\n  \
         <com>quit</com>\n</script>\n{stringmatches}\n</decompilertest>\n"
    )
}

/// Write `content` to a uniquely-named temp `.xml` and return its path.  Kept in
/// the OS temp dir (never the vendored datatests dir, so the upstream corpus is
/// untouched and `decomp_test_dbg`'s `gatherDataTests` glob never sees it).
fn write_temp_xml(tag: &str, content: &str) -> PathBuf {
    let mut p = std::env::temp_dir();
    let pid = std::process::id();
    p.push(format!("kuna_w10_indproto_{tag}_{pid}.xml"));
    std::fs::write(&p, content).expect("write temp xml");
    p
}

/// Assert the produced results show the file actually ran: no engine-level
/// execution/parse failure, and the body was produced (`print C` reached).
fn assert_decoded(tag: &str, out: &str) {
    assert!(
        !out.contains("Error executing") && !out.contains("Error parsing"),
        "[{tag}] engine EXEC/PARSE-FAILED — the nested function-pointer type did \
         not decode:\n{out}"
    );
    // The pre-item failure mode printed exactly this on the unported path.
    assert!(
        !out.contains("getTypeCode(PrototypePieces)") && !out.contains("not yet ported"),
        "[{tag}] hit the un-ported getTypeCode(PrototypePieces) seam:\n{out}"
    );
    assert!(
        !out.contains("Did not apply tests in"),
        "[{tag}] engine bailed before applying tests (no body printed):\n{out}"
    );
}

/// T1 — regression fence on the real vendored `indproto.xml`: it decodes and the
/// negative `Exceeded maximum restarts` assertion (`Indirect prototype #1`) is a
/// genuine `Success`.  Closing the exec-failure is the item's whole claim.
#[test]
fn w10_indproto_real_file_decodes_and_first_assertion_passes() {
    let path = repo_root()
        .join("decompiler/datatests/indproto.xml")
        .to_str()
        .unwrap()
        .to_string();
    let Some(out) = run_file(&path) else { return };
    assert_decoded("indproto.xml", &out);
    assert!(
        out.contains("Success -- Indirect prototype #1"),
        "expected `Success -- Indirect prototype #1` (exec-failure closed, the \
         function decodes with no restart blow-up) in:\n{out}"
    );
    // The file's three assertions must all be *applied* (not skipped): the engine
    // ran them, so each name appears as either Success or FAIL.
    for n in ["Indirect prototype #1", "Indirect prototype #2", "Indirect prototype #3"] {
        let applied =
            out.contains(&format!("Success -- {n}")) || out.contains(&format!("FAIL -- {n}"));
        assert!(applied, "assertion `{n}` was not applied (file skipped?):\n{out}");
    }
}

/// T2 — adversarial varargs nested function pointer `void (*peek)(int4 a, ...)`.
/// Exercises the `modType` `first_var_arg_slot: -1` hardcode: the trailing-null
/// varargs marker is popped into `dotdotdot` by `new_func` before `modType`
/// runs, so the C++ `paramlist.back()==0` branch is dead and `firstVarArgSlot`
/// faithfully stays -1.  Must DECODE, not crash, not EXEC-FAIL.
#[test]
fn w10_indproto_varargs_nested_fnptr_decodes() {
    let body = "void (*peek)(int4 a, ...); void (*get)(int4 b);";
    let matches =
        "<stringmatch name=\"VARARGS nested fnptr #1\" min=\"0\" max=\"0\">getTypeCode</stringmatch>";
    let doc = build_doc(body, matches);
    let path = write_temp_xml("varargs", &doc);
    let Some(out) = run_file(path.to_str().unwrap()) else {
        let _ = std::fs::remove_file(&path);
        return;
    };
    let _ = std::fs::remove_file(&path);
    assert_decoded("varargs", &out);
    // The negative guard (the un-ported seam string must be absent) is a real
    // Success only because the file actually ran.
    assert!(
        out.contains("Success -- VARARGS nested fnptr #1"),
        "varargs nested fn-pointer did not decode cleanly:\n{out}"
    );
}

/// T3 — adversarial multi-param + non-void return nested function pointer
/// `int4 (*get)(int4 a, char *b)` alongside a second distinct method field.
/// Exercises `getInTypes` over two params, the non-void output-type assignment
/// in `setPrototype`/`updateAllTypes`, and the newly-activated
/// `TypeCode::compare`/`compareDependency` per-parameter + output recursion
/// (two distinct method types are interned and compared during `findAdd`).
/// Must DECODE.
#[test]
fn w10_indproto_multiparam_nonvoid_return_fnptr_decodes() {
    let body = "void (*peek)(int4 a); int4 (*get)(int4 a, char *b);";
    let matches = "<stringmatch name=\"MULTIPARAM nested fnptr #1\" min=\"0\" max=\"0\">not yet ported</stringmatch>";
    let doc = build_doc(body, matches);
    let path = write_temp_xml("multiparam", &doc);
    let Some(out) = run_file(path.to_str().unwrap()) else {
        let _ = std::fs::remove_file(&path);
        return;
    };
    let _ = std::fs::remove_file(&path);
    assert_decoded("multiparam", &out);
    assert!(
        out.contains("Success -- MULTIPARAM nested fnptr #1"),
        "multi-param non-void-return nested fn-pointer did not decode cleanly:\n{out}"
    );
}
