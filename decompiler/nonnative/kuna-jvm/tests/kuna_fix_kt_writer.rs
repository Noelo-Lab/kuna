//! The Kotlin writer's kuna fixes, one at a time: `jvm_fix_kt_synthetic_anonymous_object`
//! and `jvm_fix_kt_local_as_extension_receiver` on their kuna-fixes cases, each with
//! only its own fix turned off.

use std::path::Path;

use kuna_jvm::kuna_corpus::{count_failed_methods, load_cases, run_case, with_engine_stack, Case};

fn fix_case(id: &str) -> Case {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../tests/jvm/cases.tsv");
    load_cases(&manifest)
        .unwrap()
        .into_iter()
        .find(|c| c.id == format!("kuna-fixes/{id}"))
        .unwrap_or_else(|| panic!("no case kuna-fixes/{id}"))
}

/// The case's text with `option` set to `value` on top of kuna's defaults.
fn text_with(id: &str, option: &'static str, value: &'static str) -> String {
    let mut case = fix_case(id);
    case.options.push((option.to_string(), value.to_string()));
    with_engine_stack(move || run_case(&case).actual).expect("the compared unit")
}

const SAM: &str = "pkg/TestSamWrapperObjectKt";
const RECEIVER: &str = "pkg/TestCapturedReceiverKt";
const SYNTHETIC: &str = "jvm_fix_kt_synthetic_anonymous_object";
const LOCAL: &str = "jvm_fix_kt_local_as_extension_receiver";

#[test]
fn a_synthetic_anonymous_object_needs_its_own_fix() {
    let fixed = text_with(SAM, LOCAL, "off");
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

    let failed = text_with(SAM, SYNTHETIC, "off");
    assert_eq!(count_failed_methods(&failed), 4, "{failed}");
}

#[test]
fn the_methods_of_a_synthetic_object_return_their_values() {
    let fixed = text_with(SAM, SYNTHETIC, "on");
    assert!(
        fixed.contains("            return if (y == null) 0 else -1\n"),
        "{fixed}"
    );
    assert!(
        fixed.contains("            return if (y == null) 1 else x.length() - y.length()\n"),
        "{fixed}"
    );
    assert!(
        !fixed.contains("Class flags could not be determined"),
        "{fixed}"
    );
}

#[test]
fn a_local_is_no_extension_receiver() {
    let fixed = text_with(RECEIVER, SYNTHETIC, "off");
    assert_eq!(count_failed_methods(&fixed), 0, "{fixed}");
    assert!(fixed.contains("\n      fun remove() {\n"), "{fixed}");

    let failed = text_with(RECEIVER, LOCAL, "off");
    assert_eq!(count_failed_methods(&failed), 1, "{failed}");
}
