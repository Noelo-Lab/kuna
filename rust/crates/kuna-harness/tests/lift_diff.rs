//! GATE for item `w5-infra-lift-diff`.
//!
//! The B2-grade STRUCTURAL differential the W5 gate needs: for each corpus
//! function, bootstrap the Rust `XmlArchitecture` frontend, run the real
//! `FlowInfo` follow-flow + the basic-block builder to the SAME boundary the C++
//! `break start heritage` (B2) snapshot represents, and compare the structural
//! skeleton (op count, per-op address+class+output, block partition, edges)
//! against the committed C++ B2 fixture.
//!
//! The driver lives in [`kuna_harness::corpus`] (the pinned case table, the
//! bootstrap, the extractor, and `run_self_test()`); the SAME driver backs the
//! `kuna-lift-diff` self-test binary (the declared item gate
//! `cargo run -p kuna-lift-diff -- --self-test`), so this test and that bin
//! exercise identical code.  See [`kuna_harness::liftdiff`] for the grammar, the
//! index-keyed structural model, and the precise pre-heritage-action accounting.
//!
//! ## Why the model is keyed by block INDEX, not cover-start
//!
//! Round-1 verification (`docs/rust-port/reviews/w5-infra-lift-diff.md`, F1)
//! rejected a `BTreeMap<u64,BlockRec>` keyed by cover-start: that key is NOT
//! unique — one machine instruction can decode into several basic blocks at one
//! address (SPARC register-window `save`/`restore`, conditional-execution
//! lattices, any decode-time inject).  The repaired model is an ordered
//! `Vec<BlockRec>` matched across engines by POSITION (== block index in print
//! order); edges are keyed by target-block index.  `gh6990-returnpair` (a 5-block
//! SPARC CFG that previously collapsed to a 2-block / 31-op spurious PASS) is now a
//! genuine 5-block / 209-op PASS.

use kuna_harness::corpus::{self, Expect, Outcome};

/// The gate: run the full corpus differential and assert the documented
/// expectations + the >= 8 PASS threshold.  Identical to the
/// `kuna-lift-diff --self-test` predicate (`SelfTestReport::passed`).
#[test]
fn lift_diff_b2_structural() {
    let report = corpus::run_self_test().expect("lift-diff harness error");

    // --- Report the per-test table (visible with --nocapture) ---------------
    eprintln!("\n{}", report.render());

    // When specs are unbuilt (a bare checkout) every case skips and the gate is a
    // structural pass.
    if report.all_skipped() {
        eprintln!("all cases skipped (no specs built) — structural pass");
        return;
    }

    // Every case's outcome must match its documented expectation (the WHY of each
    // divergence/exclusion is a checked invariant, not just a comment).
    assert!(
        report.mismatches.is_empty(),
        "case outcomes diverge from their documented expectations:\n  {}",
        report.mismatches.join("\n  ")
    );

    // >= 12 corpus cases span the architectures; the gate requires >= 8 PASS.
    assert!(
        corpus::CASES.len() >= 12,
        "gate requires >= 12 corpus cases spanning architectures (have {})",
        corpus::CASES.len()
    );
    assert!(
        report.npass >= 8,
        "gate requires >= 8 structurally-equal corpus tests (got {} of {}); \
         exclusions/divergences are reported in the table above",
        report.npass,
        corpus::CASES.len()
    );
    assert!(report.passed(), "the aggregate self-test predicate must hold");
}

/// F3 (review): the per-DivergentBy op-count delta is a CHECKED invariant, not a
/// decorative attribution string.  For each case whose documented expectation is
/// `DivergentBy(..)`, pin the exact `cpp_ops - rust_ops` (or block-count) delta the
/// unported pre-heritage action(s) account for, so a Rust `generate_ops` regression
/// that changed the count would be caught here instead of silently re-satisfying
/// the coarse `DivergentBy` bucket.
#[test]
fn divergent_cases_have_pinned_count_deltas() {
    // Map each DivergentBy case to the FIRST-divergence signature its attributed
    // action must produce.  These are read off the committed C++ B2 fixtures and
    // the Rust extractor; a change in either side's count flips the assertion.
    //
    // `(stem, expected first-divergence substring)`.
    const PINS: &[(&str, &str)] = &[
        // AArch64: the call block (block#3 @0x42cb04) gains 2 FuncLink ops: 2 -> 4.
        ("ccmp", "block#3 @0x42cb04: op count differs rust=2 cpp=4"),
        // x86-64 convert: the entry block (block#0) gains 51 call placeholders:
        // 79 -> 130.
        ("convert", "block#0 @0x1006b9: op count differs rust=79 cpp=130"),
        // x86-64 nan: the call block (block#3 @0x1011d8) gains 6 ops: 39 -> 45.
        ("nan", "block#3 @0x1011d8: op count differs rust=39 cpp=45"),
        // 8051: the non-entry block#2 @0xb gains the write_volatile_1 inject: 28 -> 29.
        ("gh1243-8051-addc", "block#2 @0xb: op count differs rust=28 cpp=29"),
        // ARM condexe: the recovered CFG splits differently — block COUNT diverges
        // (Rust 1 block vs the C++ 9-block condexe lattice).
        ("condexesub", "block count differs: rust=1 cpp=9"),
        // MIPS gp: the entry block#0 @0x410040 gains 4 call placeholders: 72 -> 76.
        ("gp", "block#0 @0x410040: op count differs rust=72 cpp=76"),
    ];

    let report = corpus::run_self_test().expect("lift-diff harness error");
    if report.all_skipped() {
        eprintln!("all cases skipped (no specs built) — count-delta pins not exercised");
        return;
    }

    for (stem, expected) in PINS {
        let (_, expect, outcome) = report
            .rows
            .iter()
            .find(|(s, _, _)| s == stem)
            .unwrap_or_else(|| panic!("case {stem} not found in the corpus run"));
        assert!(
            matches!(expect, Expect::DivergentBy(_)),
            "{stem} must be a DivergentBy case"
        );
        match outcome {
            Outcome::Divergent { detail, .. } => assert!(
                detail.contains(expected),
                "{stem}: first divergence must be `{expected}`, got `{detail}`"
            ),
            other => panic!("{stem}: expected a pinned DIVERGENT, got {other:?}"),
        }
    }
}

/// The single validated 5-block PASS that F1 was about: `gh6990-returnpair` must
/// PASS over ALL FIVE blocks (not a collapsed projection).  Pins the block/op
/// counts the diff actually compared.
#[test]
fn gh6990_is_a_validated_five_block_pass() {
    let report = corpus::run_self_test().expect("lift-diff harness error");
    if report.all_skipped() {
        eprintln!("all cases skipped (no specs built) — gh6990 pass not exercised");
        return;
    }
    let (_, _, outcome) =
        report.rows.iter().find(|(s, _, _)| *s == "gh6990-returnpair").expect("gh6990 case");
    match outcome {
        Outcome::Pass { blocks, ops, excluded_tracked } => {
            assert_eq!(*blocks, 5, "gh6990 must compare all 5 blocks (F1 fix)");
            assert_eq!(*ops, 209, "gh6990 compares 209 ops (211 lines - 2 entry-head COPYs)");
            assert_eq!(*excluded_tracked, 2, "the two entry-head tracked COPYs are excluded");
        }
        other => panic!("gh6990 must be a 5-block PASS, got {other:?}"),
    }
}
