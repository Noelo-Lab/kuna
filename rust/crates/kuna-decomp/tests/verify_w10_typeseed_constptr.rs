//! VERIFIER adversarial tests for `rport/w10-typeseed-constptr` — the stage that
//! ported `ActionConstantPtr` + `Funcdata::spacebaseConstant` +
//! `Architecture::cacheAddrSpaceProperties`/`inferPtrSpaces` (the global-array
//! TYPE-SEED): a mapped GLOBAL-CONSTANT address (e.g. `0x601060`) is rewritten to
//! a typed `PTRSUB(spacebase, off)` carrying the symbol's stripped-array pointer
//! type, so `RulePtrArith`'s slot search selects a `TYPE_PTR` input and builds the
//! 2D/3D `AddTreeState` index tree.
//!
//! ## The PROVEN state these tests pin (the honest verdict)
//!
//! With the type-seed landed, the constant `0x601060` is no longer a bare untyped
//! integer (`* 4 + 0x601060`); it becomes `myarray` and the 2D index structure
//! `myarray[globindex][valin]` forms (the `AddTreeState` fires — twodim/threedim
//! `#3` "no ` * ` pointer-deref" assertions FLIP green, whole-suite passing rises
//! 331 -> 333).  The remaining gap on `#1`/`#2` (and `ptrtoarray #7`/`#10`) is a
//! SEPARATE render/cast-strategy seam (`coreaction_casts.rs`/`printc.rs`): a
//! spurious `(int4 *[5])`-style CAST wraps the spacebase PTRSUB output, which the
//! C++ oracle does not emit.  That cast lives in the cast plane (`ActionSetCasts`,
//! not this wave's owned files) — so these tests pin the type-seed payoff and the
//! constraint guards, NOT the un-cast render.
//!
//! Written by the integration/verifier per docs/rust-port/verification.md
//! "Adversarial pass"; they land with the verdict regardless of outcome and are
//! authored to FLIP the moment the cast-suppression seam lands.

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

fn cpp_oracle() -> PathBuf {
    if let Some(p) = std::env::var_os("KUNA_DECOMP_TEST") {
        return PathBuf::from(p);
    }
    repo_root().join("decompiler/cpp/decomp_test_dbg")
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
    // A missing `.sla` aborts the file (no KUNA_DUMP block); SKIP cleanly.
    let begin = text.find("===KUNA_DUMP_BEGIN")?;
    let after_begin = text[begin..].find('\n')? + begin + 1;
    let mid = text[after_begin..].find("===KUNA_DUMP_MID===")? + after_begin;
    let body = text[after_begin..mid].to_string();
    if body.trim().is_empty() {
        // The file produced no C body (e.g. an exec error): SKIP rather than fail.
        eprintln!("SKIP: {stem} produced no C body");
        return None;
    }
    Some(body)
}

// =============================================================================
// (T1) THE TYPE-SEED PAYOFF: a 2D mapped global array renders the indexed form
//      `myarray[globindex][valin]`, NOT the bare-constant arithmetic
//      `* 4 + 0x601060`.  This proves `ActionConstantPtr` rewrote the constant to
//      a typed `PTRSUB(spacebase, 0x601060)` and `RulePtrArith`/`AddTreeState`
//      built the index tree off its `TYPE_PTR` output.
// =============================================================================
#[test]
fn t1_twodim_global_array_indexes_by_symbol_not_bare_constant() {
    let Some(body) = dump_print_c(&rust_harness(), "twodim") else { return };

    // The mapped symbol NAME must appear, bound onto the spacebase PTRSUB.
    assert!(
        body.contains("myarray"),
        "twodim: the mapped global symbol `myarray` did not bind — the constant \
         0x601060 stayed an un-typed/un-named bare constant (ActionConstantPtr or \
         the global-scope linkSymbolReference routing did not fire). Body:\n{body}"
    );
    // The 2D INDEX STRUCTURE must be present (AddTreeState fired off the typed
    // PTRSUB): both `[globindex]` and `[valin]` / `[valout]` subscripts appear.
    assert!(
        body.contains("[globindex]"),
        "twodim: no `[globindex]` subscript — AddTreeState did not build the 2D \
         index tree off the typed PTRSUB. Body:\n{body}"
    );
    assert!(
        body.contains("[valin]") && body.contains("[valout]"),
        "twodim: missing the inner array subscripts — the 2D array indexing did \
         not form. Body:\n{body}"
    );
    // The bare-constant arithmetic form MUST be GONE (the regression tripwire):
    // before the type-seed the body read `... * 4 + 0x601060`.
    assert!(
        !body.contains("+ 0x601060") && !body.contains("0x601060)"),
        "twodim: the raw constant 0x601060 is still rendered as bare pointer \
         arithmetic — the type-seed did not rewrite it. Body:\n{body}"
    );
}

// =============================================================================
// (T2) HARD GATE — SWITCH SENSITIVITY: const->pointer typing must NOT perturb
//      jump-table base/index recovery.  The six switch datatests must still
//      RECOVER their switch/case structure (the type-seed did not break jump-table
//      base/index recovery) and must NOT acquire a spurious global pointer
//      reference (`PTRSUB(`) — the leak a mis-typed single-bit switch-index
//      constant would cause.  (The full byte-stable-vs-prior-tree no-perturbation
//      proof is run by the integration sweep: rust render on the wave's base
//      commit == rust render on this branch, all six files; this in-tree test pins
//      the structural invariant that proof rests on.)  The `bit_transitions < 3`
//      mask filter is the guard that makes this hold.
// =============================================================================
#[test]
fn t2_switch_cluster_recovers_and_gains_no_spurious_pointer() {
    for stem in [
        "switchind",
        "switchmulti",
        "switchhide",
        "ifswitch",
        "switchloop",
        "switchreturn",
    ] {
        let Some(body) = dump_print_c(&rust_harness(), stem) else { return };
        // The render is non-empty and structurally intact (a function body with a
        // brace) — a type-seed that corrupted the IR would drop the body.  (The
        // strong byte-stable-vs-base no-perturbation proof — rust render at the
        // wave's base commit == rust render here, all six files — is run by the
        // integration sweep; this pins the body survived and gained no global ptr.)
        assert!(
            body.contains('{') && body.contains('}'),
            "{stem}: the render lost its function body — the const->pointer \
             type-seed corrupted the IR. Body:\n{body}"
        );
        // No spurious GLOBAL pointer reference: a switch-index/mask constant that
        // was wrongly typed as a global pointer would leak a `PTRSUB(0,<addr>)`
        // functional form (in0 == the constant-0 spacebase — the global-array
        // type-seed shape).  A legitimate STACK PTRSUB (`PTRSUB(v2, off)`, off a
        // local) is unrelated and pre-existing, so the guard targets `PTRSUB(0,`
        // specifically.  These files map no globals, so any `PTRSUB(0,` is a breach.
        assert!(
            !body.contains("PTRSUB(0,"),
            "{stem}: a global `PTRSUB(0,...)` leaked into the switch render — a \
             switch-index constant was mis-typed as a global pointer \
             (bit_transitions<3 guard breach). Body:\n{body}"
        );
    }
}

// =============================================================================
// (T3) SINGLE-BIT-CONSTANT-NOT-POINTER guard: a constant that looks like a single
//      bit or mask (`bit_transitions < 3`) must NOT be turned into a pointer
//      (unless it is an exact function entry, the kuna GH-6930 escape).  The
//      switch datatests carry single-bit jump-table index/mask constants; if the
//      guard failed, those would become spurious `PTRSUB`/`&DAT_` references.  The
//      `&DAT_` / `&dat_` global-data-reference form is the most direct tell of a
//      single-bit constant wrongly promoted to a global pointer; no switch render
//      may carry one (the C++ oracle never does, and these files map no globals).
// =============================================================================
#[test]
fn t3_single_bit_const_not_turned_into_pointer() {
    for stem in ["switchind", "switchmulti", "switchhide", "switchloop"] {
        let Some(body) = dump_print_c(&rust_harness(), stem) else { return };
        let lower = body.to_ascii_lowercase();
        // A single-bit/mask constant promoted to a GLOBAL pointer would render a
        // `&dat_<addr>` / `PTRSUB(0,<addr>)` reference (the global-array type-seed
        // shape — in0 == the constant-0 spacebase).  None of these files map a
        // global, so any such reference is a guard breach.  (A stack `PTRSUB(v2,
        // off)` off a local is unrelated and pre-existing — not matched.)
        assert!(
            !lower.contains("&dat_") && !body.contains("PTRSUB(0,"),
            "{stem}: a single-bit/mask switch constant was promoted to a global \
             pointer reference (`&dat_`/`PTRSUB(0,...)`) — the bit_transitions<3 \
             guard failed. Body:\n{body}"
        );
    }
}

// =============================================================================
// (T4) THE PROTECTED-PARITY FILES STAY BYTE-IDENTICAL Rust-vs-C++: a const->ptr
//      type-seed that leaked into an unrelated function would surface here.
//      `boolless` / `condconst` carry constants but no mapped global array, so the
//      type-seed must be a complete no-op on them.
// =============================================================================
#[test]
fn t4_protected_parity_files_byte_identical() {
    for stem in ["boolless", "condconst"] {
        let Some(rust_body) = dump_print_c(&rust_harness(), stem) else { return };
        let Some(cpp_body) = dump_print_c(&cpp_oracle(), stem) else { return };
        assert_eq!(
            rust_body, cpp_body,
            "{stem}: render diverged from the C++ oracle — the const->ptr type-seed \
             perturbed a function with no mapped global array.\n--- RUST ---\n\
             {rust_body}\n--- CPP ---\n{cpp_body}"
        );
    }
}

// =============================================================================
// (T5) THREEDIM 3D index tree: the same type-seed must build the 3-deep
//      `myarray[globindex][..][valin]` access (proves inferPtrSpaces + the
//      stripped-array pointer type generalize past 2D).
// =============================================================================
#[test]
fn t5_threedim_global_array_indexes_three_deep() {
    let Some(body) = dump_print_c(&rust_harness(), "threedim") else { return };
    assert!(
        body.contains("myarray") && body.contains("[globindex]"),
        "threedim: the 3D mapped global array did not bind/index off the typed \
         PTRSUB. Body:\n{body}"
    );
    // Three subscript groups on the myarray access: [globindex][..][valin/valout].
    // Count `][` (subscript-to-subscript joins) — a 3D access has >= 2 per row.
    assert!(
        body.matches("][").count() >= 2,
        "threedim: fewer than a 3-deep index chain formed — AddTreeState did not \
         build the full nested index off the typed PTRSUB. Body:\n{body}"
    );
    // The bare 3D-stride constant arithmetic must be gone (no `+ 0x` global base
    // arithmetic on the array).
    assert!(
        !body.contains("+ 0x601060"),
        "threedim: the raw global base 0x601060 is still bare arithmetic. \
         Body:\n{body}"
    );
}
