//! The Kotlin reference fixes and the anonymous-class failure message, one at a time, on their
//! kuna-fixes cases: `kt_reference_anonymous_super_call`, `kt_property_reference` and
//! `kt_anonymous_class_failure_reason`, each with only its own fix turned off (`KunaFixes::without`).

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

/// The case's text with `fixes` active, and with the exception in failure comments when `dump`.
fn text_with(id: &str, fixes: KunaFixes, dump: bool) -> String {
    let mut case = if id == UNWRITABLE {
        unwritable_case()
    } else {
        fix_case(id)
    };
    case.fixes = Some(fixes);
    if dump {
        case.options.retain(|(k, _)| k != "dump-exception-on-error");
        case.options
            .push(("dump-exception-on-error".into(), "1".into()));
    }
    with_engine_stack(move || run_case(&case).actual).expect("the compared unit")
}

/// `tests/data/kt_anonymous_class_failure_reason/` (kotlinc 2.0.0, `-jvm-target 17`) run as a
/// kuna-fixes Kotlin case: it has no case of its own, because what its fix changes is printed
/// only with `dump-exception-on-error` on, where the compat text cannot be Vineflower's (its
/// stack trace).
fn unwritable_case() -> Case {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/data/kt_anonymous_class_failure_reason/pkg");
    let mut class_files: Vec<_> = std::fs::read_dir(&dir)
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.extension().is_some_and(|x| x == "class"))
        .collect();
    class_files.sort();
    let main = class_files
        .iter()
        .position(|p| p.ends_with("TestUnwritableObjectsKt.class"))
        .unwrap();
    let main = class_files.remove(main);
    class_files.insert(0, main);
    let mut case = fix_case(FUNCTIONS);
    case.id = format!("kuna-fixes/{UNWRITABLE}");
    case.class_files = class_files;
    case.compare = vec![
        "TestUnwritableObjectsKt.kt".into(),
        "TestUnwritableObjectsKt.java".into(),
    ];
    case
}

const FUNCTIONS: &str = "pkg/TestReferenceClassesKt";
const PROPERTIES: &str = "pkg/TestPropertyReferencesKt";
const UNWRITABLE: &str = "pkg/TestUnwritableObjectsKt";
const UNREPRESENTABLE: &str = "<unrepresentable>.INSTANCE";

#[test]
fn function_references_read_the_kept_super_call() {
    let fixed = text_with(
        FUNCTIONS,
        KunaFixes::DEFAULT.without(KunaFix::KtPropertyReference),
        false,
    );
    assert_eq!(count_failed_methods(&fixed), 0, "{fixed}");
    for reference in [
        "(adder::add as KFunction)",
        "(Adder::add as KFunction)",
        "(::Adder as KFunction)",
        "(::twice as KFunction)",
    ] {
        assert!(fixed.contains(reference), "{reference}: {fixed}");
    }
    assert!(!fixed.contains(UNREPRESENTABLE), "{fixed}");

    let unfixed = text_with(
        FUNCTIONS,
        KunaFixes::DEFAULT.without(KunaFix::KtReferenceAnonymousSuperCall),
        false,
    );
    assert_eq!(count_failed_methods(&unfixed), 1, "{unfixed}");
    assert_eq!(unfixed.matches(UNREPRESENTABLE).count(), 3, "{unfixed}");
}

#[test]
fn property_references_are_written_as_references() {
    let fixed = text_with(
        PROPERTIES,
        KunaFixes::DEFAULT.without(KunaFix::KtReferenceAnonymousSuperCall),
        false,
    );
    assert_eq!(count_failed_methods(&fixed), 0, "{fixed}");
    for reference in [
        "valueOf(box::size)",
        "(box::count as KMutableProperty0)",
        "(Box::size as KProperty1)",
        "(Box::count as KMutableProperty1)",
        "(::answer as KProperty0)",
        "(Box::doubled as KProperty1)",
    ] {
        assert!(fixed.contains(reference), "{reference}: {fixed}");
    }
    assert!(!fixed.contains(UNREPRESENTABLE), "{fixed}");

    let unfixed = text_with(
        PROPERTIES,
        KunaFixes::DEFAULT.without(KunaFix::KtPropertyReference),
        false,
    );
    assert_eq!(count_failed_methods(&unfixed), 2, "{unfixed}");
    assert_eq!(unfixed.matches(UNREPRESENTABLE).count(), 4, "{unfixed}");
}

#[test]
fn an_unwritable_anonymous_class_says_what_it_is() {
    let fixed = text_with(UNWRITABLE, KunaFixes::DEFAULT, true);
    assert_eq!(count_failed_methods(&fixed), 3, "{fixed}");
    assert!(
        fixed.contains(
            "Anonymous class does not have Class Kotlin metadata: pkg.TestUnwritableObjectsKt$twoSteps$1 \
             is the continuation of a suspend function: the method is the function's coroutine state \
             machine, which has no Kotlin form\n"
        ),
        "{fixed}"
    );
    assert!(
        fixed.contains(
            "Anonymous class does not have Class Kotlin metadata: pkg.TestUnwritableObjectsKt$localReference$1 \
             is a reference to a local function that captures variables\n"
        ),
        "{fixed}"
    );

    let unfixed = text_with(
        UNWRITABLE,
        KunaFixes::DEFAULT.without(KunaFix::KtAnonymousClassFailureReason),
        true,
    );
    assert_eq!(count_failed_methods(&unfixed), 3, "{unfixed}");
    assert_eq!(
        unfixed
            .matches("IllegalStateException: Anonymous class does not have Class Kotlin metadata\n")
            .count(),
        3,
        "{unfixed}"
    );
}
