//! The fixes `field_read_all_paths` and `kt_inline_evaluation_order` on their
//! kuna-fixes cases: each alone off (`KunaFixes::without`) prints Vineflower's
//! inlining, and neither changes the other's case.

use std::path::Path;

use kuna_jvm::kuna_corpus::{load_cases, run_case, with_engine_stack, Case};
use kuna_jvm::kuna_fixes::{KunaFix, KunaFixes};

fn text_with(id: &str, fixes: KunaFixes) -> String {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../tests/jvm/cases.tsv");
    let mut case: Case = load_cases(&manifest)
        .unwrap()
        .into_iter()
        .find(|c| c.id == id)
        .expect("the case");
    case.fixes = Some(fixes);
    with_engine_stack(move || run_case(&case).actual).expect("the compared unit")
}

const FIELDS: &str = "kuna-fixes/pkg/TestKtFieldReadPaths";
const ORDER: &str = "kuna-fixes/pkg/TestKtInlineEvaluationOrder";

#[test]
fn a_field_read_crossing_a_call_stays_a_local() {
    let fixed = text_with(FIELDS, KunaFixes::DEFAULT);
    for kept in [
        "val v: Int = h.count",
        "val saved: Int = h.count",
        "h.count = saved",
        "src.use(v + x)",
        "src.use(if (c && src.flag()) v else 0)",
    ] {
        assert!(fixed.contains(kept), "{kept}\n{fixed}");
    }
    assert!(!fixed.contains("src.use(h.count"), "{fixed}");

    let broken = text_with(FIELDS, KunaFixes::DEFAULT.without(KunaFix::FieldReadAllPaths));
    for moved in [
        "src.use(h.count)",
        "h.count = h.count",
        "src.use(h.count + x)",
        "src.use(if (c && src.flag()) h.count else 0)",
    ] {
        assert!(broken.contains(moved), "{moved}\n{broken}");
    }
    let other = text_with(FIELDS, KunaFixes::DEFAULT.without(KunaFix::KtInlineEvaluationOrder));
    assert_eq!(other, fixed);
}

#[test]
fn a_value_with_effects_keeps_its_place() {
    let fixed = text_with(ORDER, KunaFixes::DEFAULT);
    for kept in [
        "val first: Int = src.next()",
        "src.take1(if (k > 0) v else 0)",
        "if (enabled && v < 0)",
        "val x: Int = a[0]",
        "src.leave(token)",
        "src.take(src.next(), a[0])",
    ] {
        assert!(fixed.contains(kept), "{kept}\n{fixed}");
    }

    let broken = text_with(ORDER, KunaFixes::DEFAULT.without(KunaFix::KtInlineEvaluationOrder));
    for moved in [
        "src.take(src.next(), src.next())",
        "src.take1(if (k > 0) src.next() else 0)",
        "if (enabled && src.next() < 0)",
        "src.take(src.next(), a[0])",
        "src.leave(src.enter())",
    ] {
        assert!(broken.contains(moved), "{moved}\n{broken}");
    }
    let other = text_with(ORDER, KunaFixes::DEFAULT.without(KunaFix::FieldReadAllPaths));
    assert_eq!(other, fixed);
}
