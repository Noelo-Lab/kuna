//! Unit tests for the typed assertion core (`kuna_assert.rs`).
//!
//! Covers the validation precedence, the strength resolution, the
//! minimal-rewind computation matrix (`stage-model.md` section 12), the
//! sub-stage -> store dispatch routing, and the `kassert list` rendering.

use super::*;
use crate::kuna_stages::KunaStage;
use kuna_base::error::KunaError;

// --- validate_assertion: precedence of the C++ IfaceParseError throws --------

#[test]
fn validate_bad_stage_code_first() {
    let err = validate_assertion("ZZ", "type-propagation", None).unwrap_err();
    match err {
        KunaError::Parse { explain } => {
            assert_eq!(explain, "Bad stage code (expecting P0 or S1..S9): ZZ");
        }
        other => panic!("expected Parse, got {other:?}"),
    }
}

#[test]
fn validate_missing_substage() {
    let err = validate_assertion("S5", "", None).unwrap_err();
    match err {
        KunaError::Parse { explain } => {
            assert_eq!(explain, "Missing sub-stage name (see `stage list`)");
        }
        other => panic!("expected Parse, got {other:?}"),
    }
}

#[test]
fn validate_unknown_substage() {
    let err = validate_assertion("S5", "not-real", None).unwrap_err();
    match err {
        KunaError::Parse { explain } => {
            assert_eq!(explain, "Unknown sub-stage: not-real (see `stage list`)");
        }
        other => panic!("expected Parse, got {other:?}"),
    }
}

#[test]
fn validate_wrong_stage_for_substage() {
    // type-propagation is S5; asserting it under S3 must fail with the C++ msg.
    let err = validate_assertion("S3", "type-propagation", None).unwrap_err();
    match err {
        KunaError::Parse { explain } => {
            assert_eq!(
                explain,
                "Sub-stage type-propagation belongs to stage S5, not S3"
            );
        }
        other => panic!("expected Parse, got {other:?}"),
    }
}

#[test]
fn validate_ok_uses_catalog_strength_by_default() {
    let v = validate_assertion("S5", "type-propagation", None).unwrap();
    assert_eq!(v.stage, KunaStage::S5);
    assert_eq!(v.substage, "type-propagation");
    // type-propagation is HARD in the catalog -> requested defaults to HARD.
    assert_eq!(v.requested, KunaStrength::Hard);
    assert_eq!(v.applied, KunaStrength::Hard);
    assert_eq!(v.rewind, KunaStage::S5);
    assert_eq!(v.dispatch, Dispatch::Retype);
}

#[test]
fn validate_strength_override_replaces_requested_not_applied() {
    // Override hard->hint: requested becomes HINT, applied stays the catalog HARD.
    let v = validate_assertion("S5", "type-propagation", Some(KunaStrength::Hint)).unwrap();
    assert_eq!(v.requested, KunaStrength::Hint);
    assert_eq!(v.applied, KunaStrength::Hard);
}

// --- Minimal-rewind computation matrix (stage-model.md section 12) -----------

#[test]
fn minimal_rewind_matrix() {
    // (stage, substage, expected minimal rewind) transcribed from the catalog:
    // format->S9, force-goto->S7, typelock->S5, proto->S4, jumptable/flow/
    // context->S2, deadcode-delay->S3.
    let cases = [
        ("S9", "literal-format", KunaStage::S9),
        ("S9", "naming-policy", KunaStage::S9),
        ("S7", "edge-virtualization", KunaStage::S7),
        ("S5", "type-propagation", KunaStage::S5),
        ("S4", "prototype-source", KunaStage::S4),
        ("S2", "switch-model", KunaStage::S2),
        ("S2", "flow-classification", KunaStage::S2),
        ("S2", "decode-context", KunaStage::S2),
        ("S3", "dead-definition-gate", KunaStage::S3),
        // explicit-implied is the one cross-stage rewind: S6 decision -> S9 rewind.
        ("S6", "explicit-implied", KunaStage::S9),
        // external-refinement rewinds back to P0.
        ("S9", "external-refinement", KunaStage::P0),
        // pipeline-variant: a P0 decision whose rewind is S1.
        ("P0", "pipeline-variant", KunaStage::S1),
    ];
    for (stagecode, sub, expect) in cases {
        let v = validate_assertion(stagecode, sub, None)
            .unwrap_or_else(|e| panic!("{stagecode}/{sub} should validate: {e:?}"));
        assert_eq!(v.rewind, expect, "rewind mismatch for {stagecode}/{sub}");
    }
}

// --- Dispatch routing (the C++ if/else chain) --------------------------------

#[test]
fn dispatch_routes_each_store() {
    // (substage, latent?, expected dispatch) -- the C++ IfcKunaAssert::execute order.
    assert_eq!(Dispatch::for_substage("edge-virtualization", false), Dispatch::ForceGoto);
    assert_eq!(Dispatch::for_substage("dead-definition-gate", false), Dispatch::DeadcodeDelay);
    assert_eq!(Dispatch::for_substage("switch-model", false), Dispatch::MultistageJump);
    assert_eq!(Dispatch::for_substage("flow-classification", false), Dispatch::FlowOverride);
    assert_eq!(Dispatch::for_substage("prototype-source", false), Dispatch::ProtoLock);
    assert_eq!(Dispatch::for_substage("type-propagation", false), Dispatch::Retype);
    assert_eq!(Dispatch::for_substage("merge-aggressiveness", false), Dispatch::Isolate);
    assert_eq!(Dispatch::for_substage("naming-policy", false), Dispatch::Rename);
    assert_eq!(
        Dispatch::for_substage("comparison-canonicalization", false),
        Dispatch::Option("compareform")
    );
    assert_eq!(
        Dispatch::for_substage("pointer-notation", false),
        Dispatch::Option("arraynotation")
    );
    assert_eq!(
        Dispatch::for_substage("simplification-quiescence", false),
        Dispatch::Option("togglerule")
    );
    assert_eq!(
        Dispatch::for_substage("readability-rewrites", false),
        Dispatch::Option("analyzeforloops")
    );
    assert_eq!(Dispatch::for_substage("extrapop", false), Dispatch::Option("extrapop"));
    assert_eq!(Dispatch::for_substage("pipeline-variant", false), Dispatch::Option("setaction"));
}

#[test]
fn dispatch_latent_vs_unroutable() {
    // The C++ else branch distinguishes latent (no surface yet) from merely-
    // unroutable (use the native surface). assertion-admission is LATENT.
    assert_eq!(Dispatch::for_substage("assertion-admission", true), Dispatch::Latent);
    // A non-latent sub-stage with no kassert route is Unroutable.
    assert_eq!(Dispatch::for_substage("const-pointer", false), Dispatch::Unroutable);
}

#[test]
fn dispatch_decision_via_validate() {
    // The validated assertion carries the dispatch decision.
    let v = validate_assertion("S7", "edge-virtualization", None).unwrap();
    assert_eq!(v.dispatch, Dispatch::ForceGoto);
    let v = validate_assertion("S3", "comparison-canonicalization", None).unwrap();
    assert_eq!(v.dispatch, Dispatch::Option("compareform"));
    // assertion-admission (P0, latent) routes to Latent.
    let v = validate_assertion("P0", "assertion-admission", None).unwrap();
    assert_eq!(v.dispatch, Dispatch::Latent);
}

// --- AssertLog rendering (kassert list) --------------------------------------

#[test]
fn empty_log_message() {
    let log = AssertLog::new();
    assert!(log.is_empty());
    assert_eq!(
        log.render_list(),
        "No kuna assertions recorded this session\n"
    );
}

#[test]
fn list_renders_records_with_rewind() {
    let mut log = AssertLog::new();
    log.push(KunaAssertion {
        func_name: "main".to_string(),
        stage: KunaStage::S5,
        substage: "type-propagation".to_string(),
        args: "x int".to_string(),
        strength: KunaStrength::Hard,
        applied: KunaStrength::Hard,
        rewind: KunaStage::S5,
    });
    let out = log.render_list();
    assert!(out.starts_with(
        "Recorded assertions (rewind scope is REPORTED per stage-model.md s12; \
         Ghidra-actual re-run remains whole-function):\n"
    ));
    assert!(out.contains("  0: [main] S5/type-propagation x int  strength=HARD  rewind->S5\n"));
}

#[test]
fn list_notes_hint_via_hard_mechanism() {
    let mut log = AssertLog::new();
    log.push(KunaAssertion {
        func_name: "(global)".to_string(),
        stage: KunaStage::S5,
        substage: "type-propagation".to_string(),
        args: String::new(),
        strength: KunaStrength::Hint,
        applied: KunaStrength::Hard,
        rewind: KunaStage::S5,
    });
    let out = log.render_list();
    // strength=HINT but applied=HARD -> the "(applied via HARD mechanism)" note.
    assert!(out.contains("strength=HINT (applied via HARD mechanism)  rewind->S5\n"));
    // No args -> no trailing space before "  strength".
    assert!(out.contains("[(global)] S5/type-propagation  strength=HINT"));
}
