//! VERIFIER adversarial tests for `rport/w10-spacebase-render` — the stage that
//! merged the held `rport/w10-forloop-reroll` (while->for conversion) onto the
//! RSP-spacebase-rendered tree.
//!
//! Written by the integration/verifier per docs/rust-port/verification.md
//! "Adversarial pass"; they land with the verdict regardless of outcome.
//!
//! ## The PROVEN state these tests pin (the honest verdict)
//!
//! The stage brief's premise — "with the RSP-input spacebase-pointer typing
//! landed, the for-loop files are RSP-clean, so the held for-loop reroll's
//! overflow-syntax guard no longer declines and for-loops form" — is
//! CONTRADICTED by the running engine.  Diagnosed against the FAITHFUL datatest
//! harness (`decomp_test_dbg` + `KUNA_DUMP=1`, the exact path the parity runner
//! drives), every for-loop datatest (`forloop1`, `forloop_thruspecial`,
//! `forloop_varused`, `loopcomment`) still renders:
//!
//!   * raw RSP locals  `undefined8 v2; // rsp`  /  `undefined8 v3; // rsp`,
//!   * a per-CALL dead stack store of the return address
//!         `*(xunknown1 *)&v3[0xff..f8] = 0x4005ee;`  followed by the CALL, and
//!   * the `v3 = &v3[0xff..f8]` stack-pointer adjust,
//!
//! i.e. the loop body is NOT clean.  The C++ oracle eliminates all of it and
//! emits `for (v1 = 0; v1 < max; v1 = v1 + 1) {...}`.  The blocker is the
//! DEFERRED `ActionExtraPopSetup::apply` (coreaction_protos.rs:470, deferred at
//! commit 10fc2ab): its per-call INT_ADD/INDIRECT spacebase op is only net-safe
//! once `propagateSpacebaseRef` + the spacebase-store dead-code land WITH it;
//! enabling it ALONE regresses jump-table index recovery (`switchind`).  So the
//! reroll's `has_overflow_syntax()` guard (funcdata_block.rs:196) correctly
//! still declines — the for-loop cannot form at this stage without violating the
//! switch-no-regression constraint.
//!
//! What this stage DID establish (and what these tests pin):
//!
//!   (A1) The reroll merge is BYTE-SAFE and INERT, not destructive: the for-loop
//!        datatests still render the plain `while`/`if-break` form byte-for-byte
//!        as before the merge — `emit_block_while_do`'s new `iterate_op` branch
//!        is never taken (no node carries an iterate op), so no `while` render
//!        is corrupted into a malformed `for`.  Regression tripwire for the
//!        merged `emit_for_loop`.
//!
//!   (A2) NO FALSE-POSITIVE for-loop: the For-loop datatest `<stringmatch>`
//!        (`for.*v1 = 0; v1 < max; v1 = v1 + 1`) stays FAIL under the Rust
//!        engine — the engine does NOT fabricate a `for (` header it cannot
//!        legitimately structure.  (This is the negative the brief's "No
//!        for-loop" guard cluster also encodes.)
//!
//!   (A3) THE SWITCH/JUMPTABLE CLUSTER DOES NOT REGRESS: the switch datatest
//!        assertions that pass on the pre-merge tree still pass — the merge did
//!        not perturb jump-table recovery (the very thing enabling ExtraPopSetup
//!        would have broken).  This is the hard brief constraint.
//!
//!   (A4) THE PROTECTED FILES STAY BYTE-IDENTICAL to the C++ oracle: `boolless`,
//!        `readstruct`, `condconst_conn` — even though they may carry stack
//!        locals — render identically Rust-vs-C++.  Render-stage tripwire.
//!
//! These are written to FLIP GREEN the day the ExtraPopSetup+propagateSpacebaseRef
//! co-landing makes the for-loop body clean: (A1)/(A4) stay green throughout,
//! and (A2) inverts (the `for` header appears) — at which point the assertion is
//! updated to require the `for`.  Until then they pin the true, non-fabricated
//! state.

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
    let begin = text.find("===KUNA_DUMP_BEGIN")?;
    let after_begin = text[begin..].find('\n')? + begin + 1;
    let mid = text[after_begin..].find("===KUNA_DUMP_MID===")? + after_begin;
    Some(text[after_begin..mid].to_string())
}

/// Run the Rust harness over the WHOLE corpus once and return the per-line
/// `Success -- <name>` / `FAIL -- <name>` verdict stream (stdout).
fn rust_verdicts() -> Option<String> {
    let bin = rust_harness();
    if !bin.exists() {
        eprintln!("SKIP: rust harness {bin:?} not built");
        return None;
    }
    let out = Command::new(&bin)
        .arg("-sleighpath")
        .arg(specs_root())
        .arg("-path")
        .arg(datatests_dir())
        .arg("datatests")
        .output()
        .ok()?;
    Some(String::from_utf8_lossy(&out.stdout).into_owned())
}

fn verdict_for(stream: &str, name: &str) -> Option<bool> {
    for line in stream.lines() {
        if let Some(rest) = line.strip_prefix("Success -- ") {
            if rest == name {
                return Some(true);
            }
        } else if let Some(rest) = line.strip_prefix("FAIL -- ") {
            if rest == name {
                return Some(false);
            }
        }
    }
    None
}

// =============================================================================
// (A1) The reroll merge is INERT, not destructive: for-loop files still render
//      the plain `while`/`if-break` body and NEVER a malformed `for (` header.
//      `emit_for_loop` is reachable only when a whiledo node carries an iterate
//      op; none does at this stage, so the body must be byte-stable.
// =============================================================================
#[test]
fn a1_forloop_files_render_plain_while_not_malformed_for() {
    for stem in ["forloop1", "forloop_thruspecial", "forloop_varused", "loopcomment"] {
        let Some(body) = dump_print_c(&rust_harness(), stem) else { return };
        // The body must structure as a `while`/`do`/`if`-break loop, never a
        // `for (` header (the reroll is inert: no iterate op is recorded).
        assert!(
            !body.contains("for ("),
            "{stem}: a `for (` header appeared — the reroll fired without a clean \
             RSP body (or fabricated a header). Body:\n{body}"
        );
        // It IS still a loop (sanity: we are looking at the right function): the
        // render contains a loop keyword, confirming the body wasn't dropped.
        assert!(
            body.contains("while") || body.contains("do {") || body.contains("goto"),
            "{stem}: no loop structure at all — the merge corrupted the render. Body:\n{body}"
        );
    }
}

// =============================================================================
// (A1b) The reroll merge did not perturb the for-loop bodies' BYTES: the Rust
//       render is unchanged by the merge.  We pin the *shape* that proves the
//       reroll is inert — the dead RSP return-address store + `// rsp` locals
//       are STILL present (the exact thing the ExtraPopSetup deferral leaves),
//       which is WHY the overflow guard declines.  This is the diagnostic the
//       brief asked for, asserted as a tripwire: the day it flips, RSP got
//       cleaned and the for-loop should form.
// =============================================================================
#[test]
fn a1b_forloop1_still_carries_dirty_rsp_proving_reroll_blocked() {
    let Some(body) = dump_print_c(&rust_harness(), "forloop1") else { return };
    // The proof the for-loop CANNOT form yet: the RSP input + stack-pointer
    // chain survives (deferred ActionExtraPopSetup).  No register-name
    // special-casing — we key on the `// rsp` comment the printer emits for an
    // un-eliminated spacebase-input local and the raw PTRSUB-on-spacebase store.
    let has_rsp_local = body.contains("// rsp");
    let has_dead_chain = body.contains("(xunknown1 *)") && body.contains("] = 0x");
    assert!(
        has_rsp_local && has_dead_chain,
        "forloop1: the dirty-RSP chain is GONE — RSP got cleaned. If so, the \
         for-loop reroll should now FORM: update this test + (A2) to require the \
         `for (v1 = 0; v1 < max; v1 = v1 + 1)` header.\nBody:\n{body}"
    );
}

// =============================================================================
// (A2) NO FALSE-POSITIVE for-loop: the engine does not fabricate the `for`
//      header it cannot legitimately structure.  The For-loop #1 datatest stays
//      FAIL under Rust while it is Success under the C++ oracle (the seam).
// =============================================================================
#[test]
fn a2_forloop_assertion_stays_fail_no_fabrication() {
    let Some(rust) = rust_verdicts() else { return };
    // Rust: the for-loop assertion does NOT pass (the engine declines honestly).
    match verdict_for(&rust, "For-loop #1") {
        Some(false) => {} // expected: the seam is open
        Some(true) => panic!(
            "For-loop #1 now PASSES under Rust — the for-loop reroll activated. \
             Update (A1)/(A1b)/(A2) to lock the new for-loop render in place."
        ),
        None => panic!("For-loop #1 not found in the Rust verdict stream"),
    }
    // The C++ oracle DOES pass it — confirming this is a real seam, not a broken
    // datatest. (SKIP the cross-check if the oracle binary is unavailable.)
    let cpp = cpp_oracle();
    if cpp.exists() {
        let out = Command::new(&cpp)
            .arg("-sleighpath")
            .arg(specs_root())
            .arg("-path")
            .arg(datatests_dir())
            .arg("datatests")
            .output()
            .expect("run cpp oracle");
        let cpp_stream = String::from_utf8_lossy(&out.stdout);
        assert_eq!(
            verdict_for(&cpp_stream, "For-loop #1"),
            Some(true),
            "C++ oracle no longer passes For-loop #1 — the oracle changed; \
             re-baseline before trusting the Rust verdict."
        );
    }
}

// =============================================================================
// (A3) THE SWITCH/JUMPTABLE CLUSTER DOES NOT REGRESS.  Enabling ExtraPopSetup
//      (the only way to clean the for-loop RSP today) would break exactly these;
//      the merge must NOT have touched them.  These names are the switch
//      assertions that pass on the pre-reroll tree (the Stage-A passing set).
// =============================================================================
#[test]
fn a3_switch_jumptable_cluster_does_not_regress() {
    let Some(rust) = rust_verdicts() else { return };
    // A representative, must-stay-green subset across switchind/switchmulti/
    // switchloop (the brief's named protected files).  These pass at Stage A;
    // any FAIL here means the merge perturbed jump-table recovery.
    let must_pass = [
        "Switch Indirect #1",
        "Switch Indirect #2",
        "Switch Indirect #9",
        "Switch Indirect #14",
        "Switch Multi #1",
        "Switch Loop #1",
    ];
    for name in must_pass {
        assert_eq!(
            verdict_for(&rust, name),
            Some(true),
            "REGRESSION: switch assertion `{name}` no longer passes — the \
             for-loop reroll merge perturbed jump-table recovery (the \
             ExtraPopSetup hazard the deferral exists to avoid)."
        );
    }
}

// =============================================================================
// (A4) THE PROTECTED FILES STAY BYTE-IDENTICAL to the C++ oracle.  These carry
//      stack locals; the render stage must not have shifted their bytes.
// =============================================================================
#[test]
fn a4_protected_files_byte_identical_rust_vs_cpp() {
    for stem in ["boolless", "readstruct", "condconst_conn"] {
        let Some(rust) = dump_print_c(&rust_harness(), stem) else { return };
        let Some(cpp) = dump_print_c(&cpp_oracle(), stem) else { return };
        assert_eq!(
            rust, cpp,
            "{stem}: Rust render diverged from the C++ oracle — the \
             spacebase-render / for-loop-reroll stage shifted a protected file's \
             bytes.\n--- RUST ---\n{rust}\n--- CPP ---\n{cpp}"
        );
    }
}

// =============================================================================
// INDEPENDENT VERIFIER (Round 1) adversarial tests.
//
// The porter's `a4_protected_files_byte_identical_rust_vs_cpp` above keys on the
// stems "readstruct" / "condconst_conn", which are NOT datatest *files* (they are
// function names inside `nestedoffset.xml` / `condconst.xml`).  `dump_print_c`
// finds no `===KUNA_DUMP_BEGIN` marker for them and returns None, so that test
// silently checks only `boolless`.  These tests re-pin the byte-identity claim on
// REAL datatest stems that carry stack locals and exercise the new SPACEBASE
// `opPtrsub` arm, plus pin the two stage facts the independent review confirmed:
// the SPACEBASE arm DOES fire and renders `&name` for mapped symbols, and it does
// NOT leak a `$$undef` placeholder for unmapped auto-locals (the
// `is_name_undefined` suppression in `link_symbol_reference`).
// =============================================================================

/// Real render-sensitive stems carrying stack locals: must be byte-identical
/// Rust-vs-C++ at this stage (they pass on the suite; the merge must not shift
/// their bytes).  Unlike the porter's A4 these use stems that actually emit a
/// dump body, so the assertion is not vacuous.
#[test]
fn vfy_r1_real_protected_files_byte_identical() {
    for stem in ["boolless", "nestedoffset", "condconst"] {
        let Some(rust) = dump_print_c(&rust_harness(), stem) else { return };
        let Some(cpp) = dump_print_c(&cpp_oracle(), stem) else { return };
        assert!(!rust.is_empty(), "{stem}: empty Rust dump — stem is not a real datatest file");
        assert_eq!(
            rust, cpp,
            "{stem}: Rust render diverged from the C++ oracle (real protected stem).\n\
             --- RUST ---\n{rust}\n--- CPP ---\n{cpp}"
        );
    }
}

/// The new `opPtrsub` SPACEBASE arm is NOT dead code: for a *mapped* stack symbol
/// it renders the `&name` reference form (the W10 render payoff), replacing the
/// pre-stage functional `PTRSUB(vN, 0xff..)` text.  `partialsplit` maps stack
/// locals (`stackfoo`/`stackmy`/`stackconst`) that are passed by address.
#[test]
fn vfy_r1_spacebase_arm_emits_addressof_name_for_mapped_local() {
    let Some(rust) = dump_print_c(&rust_harness(), "partialsplit") else { return };
    // The reference form appeared for the MAPPED locals (the SPACEBASE arm fired,
    // reading the Symbol parked by link_symbol_reference).  Each is passed by
    // address to sub_101010 as `&name`, not the pre-stage `PTRSUB(vN, off)`.
    for sym in ["&stackfoo", "&stackmy", "&stackconst"] {
        assert!(
            rust.contains(sym),
            "partialsplit: the SPACEBASE `&name` payoff `{sym}` did not render — \
             the opPtrsub SPACEBASE arm is not firing for a mapped local. Body:\n{rust}"
        );
    }
    // Faithful selectivity: an UNMAPPED stack slot in the same function still
    // falls back to the functional `PTRSUB(...)` form (no fabricated name).  This
    // confirms the arm keys on the parked Symbol, not on every spacebase PTRSUB.
    assert!(
        rust.contains("PTRSUB(v1,"),
        "partialsplit: every PTRSUB became a &name — the arm stopped being \
         selective (it must only name MAPPED slots). Body:\n{rust}"
    );
}

/// The `is_name_undefined` suppression (Rust `link_symbol_reference` returns
/// false for an undefined-named composite) must NOT introduce a NEW `&$$undef`
/// leak: an unmapped auto-local reference falls back to the functional form, not
/// to `&$$undefNN`.  The whole corpus carries exactly ONE pre-existing `&$$undef`
/// (in `forloop_thruspecial`, unchanged by this stage); the SPACEBASE arm must
/// not add a second.
#[test]
fn vfy_r1_no_new_addressof_undef_leak_from_spacebase_arm() {
    // Stems where the SPACEBASE arm newly fires on (some) unmapped locals.
    for stem in ["partialmerge", "partialsplit", "varcross", "noforloop_alias", "stackstring"] {
        let Some(rust) = dump_print_c(&rust_harness(), stem) else { return };
        assert!(
            !rust.contains("&$$undef"),
            "{stem}: the SPACEBASE arm leaked an `&$$undef` placeholder — the \
             is_name_undefined suppression failed to keep an unmapped auto-local \
             out of the render. Body:\n{rust}"
        );
    }
}
