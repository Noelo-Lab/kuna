//! The fix `kt_synthetic_object_super_call` on its kuna-fixes case, alone off
//! (`KunaFixes::without`): suspend functions whose continuations are anonymous
//! `ContinuationImpl` subclasses without Class metadata. Both run with fix
//! `kt_suspend_state_machine` off, which rebuilds the functions' state
//! machines into their direct bodies.

use std::path::Path;

use kuna_jvm::kuna_corpus::{load_cases, run_case, with_engine_stack, Case};
use kuna_jvm::kuna_fixes::{KunaFix, KunaFixes};

const CASE: &str = "pkg/TestSuspendContinuationKt";
const MARKER: &str = "// $VF: Couldn't be decompiled";

fn fix_case() -> Case {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../tests/jvm/cases.tsv");
    load_cases(&manifest)
        .unwrap()
        .into_iter()
        .find(|c| c.id == format!("kuna-fixes/{CASE}"))
        .unwrap_or_else(|| panic!("no case kuna-fixes/{CASE}"))
}

fn text_with(fixes: KunaFixes) -> String {
    let mut case = fix_case();
    case.fixes = Some(fixes);
    with_engine_stack(move || run_case(&case).actual).expect("the compared unit")
}

#[test]
fn a_continuation_is_an_object_passing_the_completion() {
    let fixed = text_with(KunaFixes::DEFAULT.without(KunaFix::KtSuspendStateMachine));
    assert!(!fixed.contains(MARKER), "{fixed}");
    assert_eq!(
        fixed
            .matches("= object : ContinuationImpl(`$completion`) {")
            .count(),
        4,
        "{fixed}"
    );
    assert!(
        fixed.contains(
            "return sumOf(null, 0, this as Continuation<in Int>)"
        ),
        "{fixed}"
    );
    assert!(
        fixed.contains("return put(0, this as Continuation<in Unit>)"),
        "{fixed}"
    );
    assert!(fixed.contains("`$continuation`.`I$2` = total"), "{fixed}");
}

#[test]
fn without_the_fix_each_suspend_function_fails() {
    let without = text_with(
        KunaFixes::DEFAULT
            .without(KunaFix::KtSyntheticObjectSuperCall)
            .without(KunaFix::KtSuspendStateMachine),
    );
    assert_eq!(without.matches(MARKER).count(), 4, "{without}");
    assert!(
        without.contains("public suspend fun sumOf(src: SuspendSource, n: Int): Int {\n   // $VF"),
        "{without}"
    );
    assert!(!without.contains("object : ContinuationImpl"), "{without}");
}
