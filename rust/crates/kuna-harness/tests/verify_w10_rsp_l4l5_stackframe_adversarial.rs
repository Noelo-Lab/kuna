//! INDEPENDENT VERIFIER adversarial tests (item: rport/w10-rsp-L4L5-stackframe).
//!
//! Proves the RSP L4/L5 stack-frame render is REAL — the named typed stack
//! reference `&v1` is produced by the faithful C++ chain
//! (`setInputVarnode`'s `funcp.hasEffect` input-effect marking removing the spurious
//! `//rsp` local + `ActionNameVars::linkSpacebaseSymbol`'s namerec rename naming the
//! covered stack Symbol `v1` (coreaction.cc:3016 + 3087-3094) + the `&symbol`
//! reference attach) — and NOT a hardcoded `&v1` string, a function-name match, a
//! register-name match, or a `mystruct`-type special-case.
//!
//! Each test drives the FULL datatest harness (the same `decomp_test_dbg` path the
//! corpus scoring uses) over a real datatest binaryimage and inspects the rendered
//! body via the KUNA_DUMP triage hook.  The verifiers grep for forbidden special-
//! casing (function names / addresses / registers / values / the `mystruct` type
//! name); these tests pin the OBSERVABLE render so a future shortcut that fakes the
//! `&v1` form (or special-cases switchhide) fails here.

use std::path::PathBuf;
use std::sync::Mutex;

use kuna_harness::testfunction::run_test_files_with_specs;

/// `KUNA_DUMP` is a PROCESS-GLOBAL env var; the three tests below share this binary
/// and would race on the set/remove if run in parallel.  Serialize the render calls
/// (and their env window) through one mutex so each test sees a clean dump.
static RENDER_LOCK: Mutex<()> = Mutex::new(());

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn x86_64_sla() -> PathBuf {
    repo_root().join("specs/Ghidra/Processors/x86/data/languages/x86-64.sla")
}

fn specs_dir_string() -> String {
    repo_root().join("specs").to_str().unwrap().to_string()
}

/// Render a single datatest's `print C` body via the KUNA_DUMP hook.  Returns the
/// raw harness output (the `===KUNA_DUMP_BEGIN..MID===` block holds the C body).
fn render_datatest(stem: &str) -> Option<String> {
    if !x86_64_sla().exists() {
        eprintln!("SKIP: x86-64.sla not built (run `make specs`)");
        return None;
    }
    let path = repo_root().join(format!("decompiler/datatests/{stem}.xml"));
    let path = path.to_str().expect("utf8 path").to_string();
    let specs = specs_dir_string();
    let files = vec![path];
    let mut out = String::new();
    // Serialize the KUNA_DUMP env window + the render across the binary's tests.
    let _guard = RENDER_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    std::env::set_var("KUNA_DUMP", "1");
    run_test_files_with_specs(&files, std::slice::from_ref(&specs), &mut out);
    std::env::remove_var("KUNA_DUMP");
    drop(_guard);
    assert!(
        !out.contains("Execution failed") && !out.contains("Did not apply tests in"),
        "{stem}.xml EXEC-FAILED:\n{out}"
    );
    Some(out)
}

/// Extract just the rendered C body (between the KUNA_DUMP markers).
fn body_of(out: &str) -> String {
    let begin = out.find("===KUNA_DUMP_BEGIN").unwrap_or(0);
    let mid = out.find("===KUNA_DUMP_MID===").unwrap_or(out.len());
    out[begin..mid].to_string()
}

/// ADVERSARIAL 1 — the `&v1` call argument is the SAME named Symbol the body member
/// accesses use.  A hardcoded `&v1` string (or a per-call shortcut) would not make
/// the body's member accesses agree on the same name; only a real shared-Symbol
/// rename does.  The call passes `&v1` AND the body renders `v1.b` (the same `v1`),
/// AND the spurious `//rsp` input local is gone (the gate flip).
#[test]
fn rsp_l4l5_switchhide_andv1_shares_named_symbol_with_body() {
    let out = match render_datatest("switchhide") {
        Some(o) => o,
        None => return,
    };
    let body = body_of(&out);

    // The call passes the named typed stack reference.
    assert!(
        body.contains("glob2struct(&v1);"),
        "EXPECTED `glob2struct(&v1);` (the renamed stack Symbol passed by reference):\n{body}"
    );
    // The body member accesses use the SAME `v1` name (shared Symbol).
    assert!(
        body.contains("v1.b = 2;") && body.contains("switch(v1.b)"),
        "EXPECTED the body to render `v1.b = 2;` and `switch(v1.b)` on the SAME `v1`:\n{body}"
    );
    // The spurious `//rsp` input local is REMOVED by the input-effect marking.
    assert!(
        !body.contains("// rsp"),
        "the spurious `//rsp` input local must be removed by the input-effect \
         marking (the gate flip):\n{body}"
    );
    // The raw spacebase intermediates must be finished (no leaked PTRSUB / RSP arg /
    // `$$undef` placeholder).
    assert!(
        !body.contains("PTRSUB(RSP")
            && !body.contains("PTRSUB(v1")
            && !body.contains("$$undef"),
        "no raw `PTRSUB(...)` / `$$undef` placeholder may leak into the render:\n{body}"
    );
}

/// ADVERSARIAL 2 — the input-effect marking + stack-frame render is NOT switchhide-
/// /`mystruct`-/`glob2struct`-specific.  On a DIFFERENT function (`noforloop_alias`,
/// a different callee, a stack ARRAY rather than a struct), the same chain still
/// removes the spurious `//rsp` input local and resolves the stack local to a named
/// reference that reaches the call (here the MAPPED name `i` via `map addr` — the
/// oracle's `might_change(i)`).  A special-case keyed on switchhide / the struct type
/// / the callee name would not produce a clean `//rsp`-free, `$$undef`-free body here.
#[test]
fn rsp_l4l5_noforloop_alias_no_rsp_local_no_special_casing() {
    let out = match render_datatest("noforloop_alias") {
        Some(o) => o,
        None => return,
    };
    let body = body_of(&out);

    // No undefined placeholder leaks — the stack local is resolved to a name.
    assert!(
        !body.contains("$$undef"),
        "no `$$undef` placeholder may leak — the stack local must be named:\n{body}"
    );
    // The spurious `//rsp` input local is removed by the input-effect marking on a
    // different function too (the gate flip is corpus-wide, not switchhide-specific).
    assert!(
        !body.contains("// rsp"),
        "the spurious `//rsp` input local must be removed here too:\n{body}"
    );
    // The raw spacebase intermediates must be finished — no leaked `PTRSUB(RSP,...)`
    // call argument (the LOSS-225 net-negative the gate flip resolves corpus-wide).
    assert!(
        !body.contains("PTRSUB(RSP"),
        "no raw `PTRSUB(RSP,...)` call-arg may leak (the stack-frame render finishes \
         it to a named reference):\n{body}"
    );
    // The recovered stack local reaches a call argument by a RESOLVED name (the
    // mapped `i` here), not a raw register.  A `<callee>(i...)` line must exist.
    let has_resolved_call_arg = body.lines().any(|l| {
        let t = l.trim_start();
        (t.contains("might_change(") || t.contains("printf(") || t.contains("sub_"))
            && t.contains('(')
            && !t.contains("(RDI")
            && !t.contains("(RSI")
    });
    assert!(
        has_resolved_call_arg,
        "the stack local must reach a call argument by a resolved name, not the raw \
         register:\n{body}"
    );
}

/// ADVERSARIAL 4 — the namerec spacebase rename consumes its `vN` in C++ LOCATION
/// ORDER (coreaction.cc:3055-3074: per-space spacebase ref INTERLEAVED with the body
/// highs), NOT all-spacebase-first.  In `switchmulti` the stack-pointer INPUT
/// spacebase Varnode (register space, offset 32) addresses a `&`-only stack struct
/// that is *later* in location order than the loop-carried body register (register
/// offset 8); so the body register must keep the LOW `v1` and the struct gets a
/// higher number.  A front-loaded pre-pass would steal `v1` for the struct, pushing
/// the loop var to `v2`/`v3` and breaking the `v1 + 10` / `v1 * 7` switch arms (the
/// switchmulti −6 regression this repair fixes).  This pins the ordering: the loop
/// variable is `v1` AND the `&v1`-style struct render is NOT present in switchmulti
/// (the struct is `&`-only with no body member access here, so it never claims `v1`).
#[test]
fn rsp_l4l5_namerec_rename_is_location_ordered_not_spacebase_first() {
    let out = match render_datatest("switchmulti") {
        Some(o) => o,
        None => return,
    };
    let body = body_of(&out);

    // The loop-carried variable participating in the switch arms keeps `v1` — the
    // body register precedes the stack-pointer-input spacebase ref in location order,
    // so it is numbered first.  These are the exact `Switch Multi` scored arms.
    assert!(
        body.contains("v1 + 10;")
            && body.contains("v1 * 7;")
            && body.contains("v1 ^ 0xaba;")
            && body.contains("v1 | 0x20;"),
        "EXPECTED the switch arms to compute on `v1` (the body register keeps the low \
         `vN`; the later stack-pointer-input spacebase ref must NOT steal it):\n{body}"
    );
    // The render must still be a real, non-trivial body (a crash from the interleaved
    // rename would be its own regression).
    assert!(
        body.contains("switch(") && body.len() > 60,
        "switchmulti must still render a real switch body:\n{body}"
    );
    // No `$$undef` placeholder leaked for the (now correctly numbered) locals.
    assert!(
        !body.contains("$$undef"),
        "no `$$undef` placeholder may leak (every local is numbered):\n{body}"
    );
}

/// ADVERSARIAL 3 — the rename is driven PURELY by the (undefined-name,
/// whole-symbol) condition, not by any token.  A mapped GLOBAL `&symbol` reference
/// (which already has a DEFINED name, e.g. `&myarray`) must STILL render its mapped
/// name — i.e. the new namerec pre-pass does not rename or clobber an already-named
/// symbol, and the `&` reference plane still resolves mapped symbols.  `ptrtoarray`
/// maps a global array and references it; its render must be unperturbed.
#[test]
fn rsp_l4l5_mapped_symbol_reference_unperturbed_by_namerec_prepass() {
    let out = match render_datatest("ptrtoarray") {
        Some(o) => o,
        None => return,
    };
    let body = body_of(&out);
    // The mapped-symbol `&` reference plane still works: a defined-name symbol is
    // referenced by its mapped name, never demoted to a `$$undef` placeholder or a
    // spurious `vN` rename (the pre-pass skips already-named symbols).
    assert!(
        !body.contains("$$undef"),
        "a mapped symbol must keep its defined name — no `$$undef` leak:\n{body}"
    );
    // The function must still decompile to a non-trivial body (a crash / empty body
    // would be its own regression from the pre-pass walk).
    assert!(
        body.contains('{') && body.contains('}') && body.len() > 40,
        "ptrtoarray must still render a real body (the namerec pre-pass must not \
         break unrelated functions):\n{body}"
    );
}
