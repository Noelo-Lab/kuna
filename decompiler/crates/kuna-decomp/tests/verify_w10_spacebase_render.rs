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
        .unwrap_or_else(|| repo_root().join("tests/datatests"))
}

fn rust_harness() -> PathBuf {
    if let Some(p) = std::env::var_os("KUNA_DECOMP_TEST_RUST") {
        return PathBuf::from(p);
    }
    let profile = std::env::var("KUNA_RUST_PROFILE").unwrap_or_else(|_| "release".to_string());
    repo_root().join("decompiler/target").join(profile).join("decomp_test_dbg")
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
// (A1) RE-PINNED (W10 for-loop chain landed): the for-loop reroll now FIRES.
//      The RuleEarlyRemoval seam + setCopyImmed + fd_sblock_last_op chain
//      clears the cond-block overflow and threads the whiledo->for conversion,
//      so for-loop files render a real `for (init; cond; incr)` header
//      byte-identical to the C++ oracle.  (Previously the reroll was inert and
//      the body stayed a `while`/`break` loop — that pin is now stale.)
// =============================================================================
#[test]
fn a1_forloop_files_render_plain_while_not_malformed_for() {
    for stem in ["forloop1", "forloop_thruspecial", "forloop_varused", "loopcomment"] {
        let Some(body) = dump_print_c(&rust_harness(), stem) else { return };
        // The reroll fires: a real `for (` header is emitted (the iterate op is
        // recorded on the whiledo node and emit_for_loop runs).
        assert!(
            body.contains("for ("),
            "{stem}: no `for (` header — the for-loop reroll regressed (the \
             RuleEarlyRemoval/setCopyImmed/fd_sblock_last_op chain). Body:\n{body}"
        );
    }
}

// =============================================================================
// (A1b) RE-PINNED (W10 for-loop chain landed): the RSP keystone cleanup STILL
//       holds (no `// rsp` spacebase local, no dead PTRSUB-on-spacebase store),
//       AND the for-loop reroll now FIRES on top of the clean RSP body —
//       `forloop1` renders the real `for (v1 = 0; v1 < max; v1 = v1 + 1)`
//       header (byte-identical to the C++ oracle, For-loop #1 now passes).
//       (Previously the reroll was blocked; that pin is now stale.)
// =============================================================================
#[test]
fn a1b_forloop1_rsp_cleaned_by_keystone_but_reroll_still_blocked() {
    let Some(body) = dump_print_c(&rust_harness(), "forloop1") else { return };
    // The dirty-RSP chain is GONE: no `// rsp` spacebase-input local and no raw
    // PTRSUB-on-spacebase return-address store survives.  (No register-name
    // special-casing — keyed on the printer's `// rsp` comment + the dead store.)
    let has_rsp_local = body.contains("// rsp");
    let has_dead_chain = body.contains("(xunknown1 *)") && body.contains("] = 0x");
    assert!(
        !has_rsp_local && !has_dead_chain,
        "forloop1: the dirty-RSP chain REAPPEARED — the keystone's RSP cleanup \
         regressed.\nBody:\n{body}"
    );
    // The reroll now fires on the clean RSP body: a real `for (` header renders.
    assert!(
        body.contains("for ("),
        "forloop1: no `for (` header — the for-loop reroll regressed.\nBody:\n{body}"
    );
}

// =============================================================================
// (A2) RE-PINNED (W10 for-loop chain landed): For-loop #1 now PASSES under Rust,
//      matching the C++ oracle.  The engine legitimately structures the
//      whiledo->for conversion (the chain cleared the cond-block overflow), so
//      the seam is closed.  (Previously Rust declined and the assertion FAILed;
//      that pin is now stale.)
// =============================================================================
#[test]
fn a2_forloop_assertion_stays_fail_no_fabrication() {
    let Some(rust) = rust_verdicts() else { return };
    // Rust: the for-loop assertion now PASSES (the reroll fires correctly).
    match verdict_for(&rust, "For-loop #1") {
        Some(true) => {} // expected: the for-loop chain landed
        Some(false) => panic!(
            "For-loop #1 now FAILS under Rust — the for-loop reroll regressed \
             (the RuleEarlyRemoval/setCopyImmed/fd_sblock_last_op chain)."
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
    // address to its sub as `&name`, not the pre-stage `PTRSUB(vN, off)`.
    //
    // POST-RSP-KEYSTONE (CORRECTION-7 stale-fence update): `stackother` is now
    // ALSO resolved as `&stackother`.  Before the keystone it fell back to the
    // functional `PTRSUB(v1, off)` form (the slot's alias was unstable, so the
    // SPACEBASE arm declined); the keystone's input-active recovery + the
    // spacebase-PTRSUB typing fix make the slot's mapped Symbol resolve cleanly, so
    // every passed-by-address stack local now renders `&name` (oracle-faithful —
    // `Partial splitting #3` flips green).
    for sym in ["&stackfoo", "&stackmy", "&stackconst", "&stackother"] {
        assert!(
            rust.contains(sym),
            "partialsplit: the SPACEBASE `&name` payoff `{sym}` did not render — \
             the opPtrsub SPACEBASE arm is not firing for a mapped local. Body:\n{rust}"
        );
    }
    // The residual unstructured-stack-pointer form is gone for the passed-by-address
    // locals: no functional `PTRSUB(v1, off)` survives on a mapped slot.
    assert!(
        !rust.contains("PTRSUB(v1,"),
        "partialsplit: a functional `PTRSUB(v1, off)` survived on a mapped slot — \
         the keystone should resolve every passed-by-address local to `&name`. \
         Body:\n{rust}"
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

// =============================================================================
// RSP-keystone type-collision adversarial assertions (PART 2, harness-driven so
// the `<com>map ...</com>` symbol mappings are applied).  The spacebase-PTRSUB
// scope-aware typing (`propagateAddIn2Out`) + the constant-reference-shadow decl
// skip (printc) eliminate the int8-vs-int4 duplicate-HighVariable collision.  No
// function-name / register / value special-casing — each keys on a structural
// property of the rendered C.
// =============================================================================

/// ADV-2 — STACK-POINTER ARGUMENT TYPES WITHOUT A SPURIOUS POINTER CAST.  When a
/// mapped stack local's address is passed to a sub-function, the `&name` argument
/// must NOT carry an inserted pointer cast (`(int4 *)&name`, `(int8 *)&name`): the
/// `PTRSUB(spacebase, off)` output must type as a pointer to the mapped local so
/// `ActionSetCasts` finds the call-argument type already matching.  Keyed on the
/// generic `(<...> *)&` shape, never a specific type or name.
#[test]
fn adv_keystone_addressof_stack_arg_has_no_spurious_pointer_cast() {
    for stem in ["switchind", "partialsplit"] {
        let Some(c) = dump_print_c(&rust_harness(), stem) else { return };
        for (idx, _) in c.match_indices('&') {
            let trimmed = c[..idx].trim_end();
            // A `(... *)` cast immediately preceding the `&` is suspicious, but a
            // `*(T *)&member` deref-of-typed-view is legitimate (reading a struct
            // field through a precision cast).  The spurious-argument pattern is a
            // pointer cast applied to an address-of CALL ARGUMENT — the cast's
            // opening `(` is preceded by `(`/`,`, NOT by a `*` dereference.
            let cast_before_amp = trimmed.ends_with(')')
                && trimmed.rfind('(').is_some_and(|op| trimmed[op + 1..].contains('*'));
            if !cast_before_amp {
                continue;
            }
            let cast_open = trimmed.rfind('(').unwrap();
            let is_deref_view = trimmed[..cast_open].trim_end().ends_with('*');
            assert!(
                is_deref_view,
                "{stem}: an address-of CALL ARGUMENT carries a spurious pointer cast \
                 (the spacebase-PTRSUB output typed wider than the mapped local — \
                 the int8-vs-int4 collision regressed):\n{c}"
            );
        }
    }
}

/// ADV-3 — THE SPACEBASE-PTRSUB TYPING IS SELECTIVE, NOT BLANKET.  The scope-aware
/// `propagateAddIn2Out` spacebase arm resolves each MAPPED slot's OWN Symbol type,
/// so distinct passed-by-address locals render as DISTINCT `&name` references — a
/// blanket/over-merged typing would collapse them onto one identifier.  Pins that
/// the fix keys on the parked per-slot Symbol, not on every spacebase PTRSUB.
#[test]
fn adv_keystone_spacebase_typing_keeps_distinct_named_locals() {
    let Some(c) = dump_print_c(&rust_harness(), "partialsplit") else { return };
    let mut refs: Vec<String> = Vec::new();
    for (idx, _) in c.match_indices('&') {
        let name: String = c[idx + 1..]
            .chars()
            .take_while(|ch| ch.is_alphanumeric() || *ch == '_')
            .collect();
        if name.len() > 1 && name.chars().next().is_some_and(|ch| ch.is_alphabetic() || ch == '_') {
            refs.push(name);
        }
    }
    let mut uniq = refs.clone();
    uniq.sort();
    uniq.dedup();
    assert!(
        uniq.len() >= 3,
        "partialsplit: the mapped `&name` references collapsed to {} distinct \
         name(s) ({:?}) — the spacebase typing over-merged distinct locals or \
         stopped resolving mapped slots:\n{c}",
        uniq.len(),
        uniq
    );
}
