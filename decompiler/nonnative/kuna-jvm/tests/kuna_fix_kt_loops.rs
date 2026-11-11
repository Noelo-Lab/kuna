//! The Kotlin `for`-loop fixes, `var_merge_liveness` and `kt_partial_array_dims` on their
//! kuna-fixes cases, each with only its own fix turned off (`KunaFixes::without`).

use std::path::Path;

use kuna_jvm::kuna_corpus::{count_failed_methods, load_cases, run_case, with_engine_stack, Case};
use kuna_jvm::kuna_fixes::{KunaFix, KunaFixes};

fn fix_case(id: &str) -> Case {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../tests/jvm/cases.tsv");
    load_cases(&manifest)
        .unwrap()
        .into_iter()
        .find(|c| c.id == format!("kuna-fixes/{id}"))
        .unwrap_or_else(|| panic!("no case kuna-fixes/{id}"))
}

fn text_with(id: &str, fixes: KunaFixes) -> String {
    let mut case = fix_case(id);
    case.fixes = Some(fixes);
    with_engine_stack(move || run_case(&case).actual).expect("the compared unit")
}

/// The case's text with every fix, and with all but `fix`.
fn with_and_without(id: &str, fix: KunaFix) -> (String, String) {
    (
        text_with(id, KunaFixes::DEFAULT),
        text_with(id, KunaFixes::DEFAULT.without(fix)),
    )
}

#[test]
fn an_unresugared_loop_goes_on_while_its_condition_holds() {
    let (fixed, broken) = with_and_without(
        "pkg/TestForLoopExitCondition",
        KunaFix::KtForLoopExitCondition,
    );
    assert!(fixed.contains("      while (i >= 0) {\n"), "{fixed}");
    assert!(fixed.contains("      while (node != null) {\n"), "{fixed}");
    assert!(!fixed.contains(") break\n"), "{fixed}");
    assert!(broken.contains("         if (i >= 0) break\n"), "{broken}");
}

#[test]
fn a_range_counts_the_initializers_variable() {
    let (fixed, broken) = with_and_without("pkg/TestForRangeCounter", KunaFix::KtForRangeCounter);
    assert!(
        fixed.contains("      val n: Int = a.size\n      while (i < n) {\n"),
        "{fixed}"
    );
    assert!(fixed.contains("      while (var5 < limit) {\n"), "{fixed}");
    assert!(fixed.contains("      while (total < 100) {\n"), "{fixed}");
    assert!(!fixed.contains("repeat("), "{fixed}");
    assert!(broken.contains("for (n in a.size until i) {"), "{broken}");
    assert!(broken.contains("repeat(100) { i ->"), "{broken}");
}

#[test]
fn a_range_keeps_the_exclusive_bound() {
    let (fixed, broken) = with_and_without(
        "pkg/TestForRangeExclusiveBound",
        KunaFix::KtForRangeExclusiveBound,
    );
    assert!(fixed.contains("for (i in from until to) {"), "{fixed}");
    assert!(fixed.contains("repeat(10) { i ->"), "{fixed}");
    assert!(broken.contains("for (i in from..to) {"), "{broken}");
    assert!(broken.contains("repeat(9) { i ->"), "{broken}");
}

/// `kt_for_range_counter` alone also keeps such a loop out of the range rewrite.
#[test]
fn a_loop_with_an_unmatched_bound_is_written() {
    let id = "pkg/TestForRangeUnmatchedBound";
    let without_counter = KunaFixes::DEFAULT.without(KunaFix::KtForRangeCounter);
    let fixed = text_with(id, without_counter);
    assert!(fixed.contains("      while (i < a.size) {\n"), "{fixed}");
    let broken = text_with(
        id,
        without_counter.without(KunaFix::KtForRangeUnmatchedBound),
    );
    assert!(!broken.contains("while"), "{broken}");
}

#[test]
fn an_unresugared_loop_declares_its_counter() {
    let (fixed, broken) =
        with_and_without("pkg/TestForLoopDefinitions", KunaFix::KtForLoopDefinitions);
    assert!(fixed.contains("      var var4: Int\n"), "{fixed}");
    assert!(!broken.contains("var var4"), "{broken}");
}

#[test]
fn continue_runs_the_update() {
    let (fixed, broken) = with_and_without(
        "pkg/TestForLoopContinueUpdate",
        KunaFix::KtForLoopContinueUpdate,
    );
    assert!(
        fixed.contains("      if (i < limit) {\n         label35@ do {\n"),
        "{fixed}"
    );
    assert!(
        fixed.contains("         } while (run { i++; i < limit })\n"),
        "{fixed}"
    );
    assert!(broken.contains("run label35@{"), "{broken}");
    assert!(broken.contains("continue@label35"), "{broken}");
}

#[test]
fn a_live_variable_is_not_merged() {
    let (fixed, broken) = with_and_without("pkg/TestVarMergeLiveness", KunaFix::VarMergeLiveness);
    assert!(
        fixed.contains("return if (var10000.size > var9.size) var10000 else var9"),
        "{fixed}"
    );
    assert!(
        broken.contains("return if (var10000.size > var10000.size) var10000 else var10000"),
        "{broken}"
    );
}

#[test]
fn an_array_sized_only_outside_is_written() {
    let (fixed, broken) = with_and_without("pkg/TestArrayOfArrays", KunaFix::KtPartialArrayDims);
    assert_eq!(count_failed_methods(&fixed), 0, "{fixed}");
    assert_eq!(fixed.matches("arrayOfNulls(n)").count(), 3, "{fixed}");
    assert_eq!(count_failed_methods(&broken), 3, "{broken}");
}
