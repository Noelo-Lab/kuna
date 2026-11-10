//! The Kotlin writer's kuna fixes, one at a time: `kt_synthetic_anonymous_object` and
//! `kt_local_as_extension_receiver` on their kuna-fixes cases, each with only its own fix
//! turned off (`KunaFixes::without`).

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

/// The case's text with `fixes` active.
fn text_with(id: &str, fixes: KunaFixes) -> String {
    let mut case = fix_case(id);
    case.fixes = Some(fixes);
    with_engine_stack(move || run_case(&case).actual).expect("the compared unit")
}

fn text_without(id: &str, fix: KunaFix) -> String {
    text_with(id, KunaFixes::DEFAULT.without(fix))
}

const SAM: &str = "pkg/TestSamWrapperObjectKt";
const RECEIVER: &str = "pkg/TestCapturedReceiverKt";
const SYNTHETIC: KunaFix = KunaFix::KtSyntheticAnonymousObject;
const LOCAL: KunaFix = KunaFix::KtLocalAsExtensionReceiver;

#[test]
fn a_synthetic_anonymous_object_needs_its_own_fix() {
    let fixed = text_without(SAM, LOCAL);
    assert_eq!(count_failed_methods(&fixed), 0, "{fixed}");
    assert_eq!(
        fixed.matches("return object : Comparator {").count(),
        3,
        "{fixed}"
    );
    assert!(
        fixed.contains("return object : Function1<Integer, Integer> {"),
        "{fixed}"
    );

    let failed = text_without(SAM, SYNTHETIC);
    assert_eq!(count_failed_methods(&failed), 4, "{failed}");
}

#[test]
fn the_methods_of_a_synthetic_object_return_their_values() {
    let fixed = text_with(SAM, KunaFixes::DEFAULT);
    assert!(
        fixed.contains("            return if (y == null) 0 else -1\n"),
        "{fixed}"
    );
    assert!(
        fixed.contains("            return if (y == null) 1 else x.length - y.length\n"),
        "{fixed}"
    );
    assert!(
        !fixed.contains("Class flags could not be determined"),
        "{fixed}"
    );
}

#[test]
fn a_local_is_no_extension_receiver() {
    let fixed = text_without(RECEIVER, SYNTHETIC);
    assert_eq!(count_failed_methods(&fixed), 0, "{fixed}");
    assert!(fixed.contains("\n      fun remove() {\n"), "{fixed}");

    let failed = text_without(RECEIVER, LOCAL);
    assert_eq!(count_failed_methods(&failed), 1, "{failed}");
}
