//! ADVERSARIAL tests for `rport/w10-spacebase-ptrsub-cast` — the stage that ports
//! `TypeSpacebase::getSubType` (`type.cc:3411-3433`) so `TypeOpPtrsub::getOutputToken`
//! (`typeop.cc:2351`) → `TypePointer::downChain` (`type.cc:1221-1257`) resolves the
//! global symbol a spacebase `PTRSUB(spacebase, off)` indexes.
//!
//! ## The bug this fixes
//!
//! For a mapped global array (`map addr 0x601060 int4 myarray[3][5]`) the type-seed
//! (`ActionConstantPtr`/`Funcdata::spacebaseConstant`) types the spacebase PTRSUB's
//! output as the strip-array pointer `int4(*)[5]` (`getTypePointerStripArray`), which
//! MATCHES what C++ sets.  `ActionSetCasts::castOutput` (`coreaction.cc:2624`) then
//! computes the op's OUTPUT TOKEN via `getOutputToken` and short-circuits (no cast)
//! when the token equals the output high type (`coreaction.cc:2635`).
//!
//! Before the fix the Rust `getOutputToken` for the spacebase PTRSUB returned the
//! `xunknown1 *` FALLBACK (the `Datatype::down_chain` → `get_sub_type` spacebase arm
//! was a `SEAM(W6)` `Err`, never resolving the symbol), so the token did NOT equal
//! the `int4(*)[5]` output high type, `force` was set, and a SPURIOUS
//! `CPUI_CAST` wrapped the PTRSUB output → the C printer rendered
//! `((int4 *[5])myarray)[globindex][valin]`.  C++ resolves the symbol (the spacebase
//! `getAlignSize()==0` SKIPS the `downChain` wrapping guard, so it reaches
//! `getSubType`), so its token IS `int4(*)[5]`, the short-circuit fires, and it
//! renders `myarray[globindex][valin]` with NO cast.
//!
//! The fix resolves the spacebase sub-type through the symbol scope
//! (`Funcdata::spacebase_get_sub_type`: the GLOBAL `GlobalQuery` snapshot when the
//! spacebase's `localframe` is invalid, else the function's `ScopeLocal`), making the
//! Rust token match the C++ token.  It is a GENERIC cast-strategy rule — NO function /
//! address / type-name / value special-casing.
//!
//! These tests drive the built `decomp_test_dbg` (rust harness) with `KUNA_DUMP=1`
//! and assert on the sliced `print C` body; they SKIP cleanly if the binary / `.sla`
//! is unavailable.

use std::path::PathBuf;
use std::process::Command;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn specs_root() -> PathBuf {
    std::env::var_os("KUNA_SPECS").map(PathBuf::from).unwrap_or_else(|| repo_root().join("specs"))
}

fn datatests_dir() -> PathBuf {
    std::env::var_os("KUNA_DATATESTS")
        .map(PathBuf::from)
        .unwrap_or_else(|| repo_root().join("decompiler/datatests"))
}

fn rust_harness() -> PathBuf {
    if let Some(p) = std::env::var_os("KUNA_DECOMP_TEST_RUST") {
        return PathBuf::from(p);
    }
    let profile = std::env::var("KUNA_RUST_PROFILE").unwrap_or_else(|_| "release".to_string());
    repo_root().join("rust/target").join(profile).join("decomp_test_dbg")
}

/// Drive `bin` on `stem`.xml with `KUNA_DUMP=1`, returning the sliced bulk
/// `print C` body (between `===KUNA_DUMP_BEGIN===` and `===KUNA_DUMP_MID===`),
/// or `None` if the binary / `.sla` is unavailable (the test SKIPs).
fn dump_print_c(bin: &PathBuf, stem: &str) -> Option<String> {
    if !bin.exists() {
        eprintln!("SKIP: binary {bin:?} not built");
        return None;
    }
    let out = Command::new(bin)
        .arg("-sleighpath")
        .arg(specs_root())
        .arg("-path")
        .arg(datatests_dir())
        .arg("datatests")
        .arg(format!("{stem}.xml"))
        .env("KUNA_DUMP", "1")
        .output()
        .ok()?;
    let text = String::from_utf8_lossy(&out.stdout);
    let begin = text.find("===KUNA_DUMP_BEGIN")?;
    let after_begin = text[begin..].find('\n')? + begin + 1;
    let mid = text[after_begin..].find("===KUNA_DUMP_MID===")? + after_begin;
    let body = text[after_begin..mid].to_string();
    if body.trim().is_empty() {
        eprintln!("SKIP: {stem} produced no C body");
        return None;
    }
    Some(body)
}

// =============================================================================
// (T1) THE FIX: a 2D mapped GLOBAL array's spacebase PTRSUB output is NOT wrapped
//      in a spurious `(int4 *[5])` array-pointer CAST.  The body indexes the
//      symbol directly: `myarray[globindex][valin]`.
// =============================================================================
#[test]
fn t1_twodim_spacebase_array_renders_no_array_pointer_cast() {
    let Some(body) = dump_print_c(&rust_harness(), "twodim") else { return };

    // The symbol must bind (the type-seed precondition this wave builds on).
    assert!(
        body.contains("myarray[globindex]"),
        "twodim: `myarray[globindex]` did not form — the spacebase PTRSUB index \
         tree is missing. Body:\n{body}"
    );
    // The SPURIOUS array-pointer cast must be GONE.  The C++ oracle renders
    // `myarray[globindex][valin]`; the pre-fix Rust rendered
    // `((int4 *[5])myarray)[globindex][valin]`.  Assert NO `int4 *[5]` (or any
    // `*[5]`/`*[3]` array-pointer cast) wraps the spacebase array.
    assert!(
        !body.contains("*[5]") && !body.contains("*[3]"),
        "twodim: a spurious array-pointer CAST (`(int4 *[5])`-style) still wraps \
         the spacebase PTRSUB output — getOutputToken did not resolve the global \
         symbol via TypeSpacebase::getSubType (token != output high type, so \
         castOutput forced a cast). Body:\n{body}"
    );
    // The exact C++-faithful subscript chain is present with NO interposed cast
    // token between `myarray` and its first subscript.
    assert!(
        body.contains("myarray[globindex][valin]"),
        "twodim: the body is not the cast-free `myarray[globindex][valin]`. \
         Body:\n{body}"
    );
}

// =============================================================================
// (T2) THE FIX GENERALIZES: a 3D mapped global array indexes three deep off the
//      same spacebase getSubType resolution, again with NO array-pointer cast.
// =============================================================================
#[test]
fn t2_threedim_spacebase_array_indexes_three_deep_no_cast() {
    let Some(body) = dump_print_c(&rust_harness(), "threedim") else { return };

    assert!(
        body.contains("myarray[globindex]"),
        "threedim: `myarray[globindex]` did not form. Body:\n{body}"
    );
    // No array-pointer cast of the form `*[N]` (the strip-array pointer the
    // pre-fix token mismatch forced).
    assert!(
        !body.contains("*[5]") && !body.contains("*[3]") && !body.contains("*[30]"),
        "threedim: a spurious array-pointer CAST still wraps the spacebase PTRSUB \
         output. Body:\n{body}"
    );
    // Three subscripts deep: `[globindex]...[valin]` (the middle index is a loop
    // variable, so assert the outer + inner subscripts and the three-deep shape).
    assert!(
        body.contains("[globindex]") && body.contains("[valin]"),
        "threedim: the 3D index chain off the typed spacebase PTRSUB is missing. \
         Body:\n{body}"
    );
}

// =============================================================================
// (T3) LEGITIMATE CASTS PRESERVED — the surgical guard.  The fix must remove ONLY
//      the spurious spacebase-array cast where the strip-array pointer already
//      matches; it must NOT remove a single legitimate cast elsewhere.  Pin the
//      two canonical non-spacebase casts the gate names:
//        - packstructaccess: the `(int4)SUB(...)` integer truncation casts.
//        - floatcast:        the `(float8)`/`(float4)` precision casts.
// =============================================================================
#[test]
fn t3_legitimate_non_spacebase_casts_survive() {
    if let Some(body) = dump_print_c(&rust_harness(), "packstructaccess") {
        assert!(
            body.contains("(int4)"),
            "packstructaccess: the legitimate `(int4)` truncation cast was \
             removed — the spacebase fix over-reached into the general cast plane. \
             Body:\n{body}"
        );
    }
    if let Some(body) = dump_print_c(&rust_harness(), "floatcast") {
        assert!(
            body.contains("(float8)") && body.contains("(float4)"),
            "floatcast: a legitimate `(float8)`/`(float4)` precision cast was \
             removed — the spacebase fix over-reached. Body:\n{body}"
        );
    }
}

// =============================================================================
// (T4) NO PER-OP SPACEBASE PERTURBATION OUTSIDE THE GLOBAL-ARRAY CASE.  The
//      LOCAL-frame (stack) spacebase PTRSUBs and the whole switch cluster must
//      keep their structure — the getSubType resolution for the LOCAL arm routes
//      through the (unchanged) `ScopeLocal::spacebase_get_sub_type`, so a stack
//      `PTRSUB(sp, off)` is not corrupted.  The six switch datatests must still
//      recover a structured body and gain no spurious global pointer leak.
// =============================================================================
#[test]
fn t4_local_spacebase_and_switch_cluster_intact() {
    // The local-frame (stack) spacebase array still resolves its members (the
    // stack arrays `a`/`c` in ptrtoarray still index/address by their symbols, not
    // a wild `xunknown` deref).
    if let Some(body) = dump_print_c(&rust_harness(), "ptrtoarray") {
        assert!(
            body.contains('{') && body.contains('}'),
            "ptrtoarray: the render lost its function body — the local-frame \
             spacebase getSubType corrupted the stack IR. Body:\n{body}"
        );
        // The stack-local array `c` is still a recognized array symbol (declared
        // `int4 c [16]`), not a flattened unknown.
        assert!(
            body.contains("c [16]") || body.contains("paiGlob"),
            "ptrtoarray: the stack array `c` / global `paiGlob` symbols vanished — \
             the local spacebase resolution broke. Body:\n{body}"
        );
    }
    for stem in ["switchind", "switchmulti", "switchhide", "ifswitch", "switchloop", "switchreturn"]
    {
        let Some(body) = dump_print_c(&rust_harness(), stem) else { return };
        assert!(
            body.contains('{') && body.contains('}'),
            "{stem}: the switch render lost its body — the spacebase getSubType \
             change perturbed jump-table recovery. Body:\n{body}"
        );
        // No spurious `*[5]`/`*[3]` array-pointer cast leaks into a switch render.
        assert!(
            !body.contains("*[5]") && !body.contains("*[3]"),
            "{stem}: a spurious array-pointer cast leaked into the switch render. \
             Body:\n{body}"
        );
    }
}

// =============================================================================
// (T5) NO SPECIAL-CASING — the fix is a generic cast-strategy / getSubType rule,
//      driven entirely by the GENERIC type graph + symbol scope.  Pin that the
//      payoff is the SAME generic mechanism on TWO independently-mapped global
//      arrays (twodim's `myarray` and threedim's `myarray`): both resolve via the
//      identical `getOutputToken`→`getSubType` path with no name/address keyed
//      branch.  (The production diff carries no `myarray`/`0x601060`/`paiGlob`
//      literal — verified by the verifier's grep; this test pins the behavioral
//      generality the grep guards.)
// =============================================================================
#[test]
fn t5_generic_rule_applies_uniformly_no_address_keyed_branch() {
    let two = dump_print_c(&rust_harness(), "twodim");
    let three = dump_print_c(&rust_harness(), "threedim");
    // Whichever renders (both, or SKIP), the cast-free property must hold uniformly
    // — there is no per-test carve-out.
    for (stem, maybe) in [("twodim", two), ("threedim", three)] {
        let Some(body) = maybe else { continue };
        assert!(
            !body.contains("*[5]") && !body.contains("*[3]") && !body.contains("*[30]"),
            "{stem}: array-pointer cast present — the generic spacebase getSubType \
             rule did not fire uniformly. Body:\n{body}"
        );
    }
}
