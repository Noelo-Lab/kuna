//! The fix `anonymous_class_created_elsewhere` on its kuna-fixes case, alone
//! off (`KunaFixes::without`): the SAM objects `dropFirst`'s lambdas declare,
//! created only in kotlinc's `$$inlined$source$1` copy, written as Kotlin and
//! as Java. The case's classes are units of their own, so without the fix the
//! creating class is written before the declaring class is processed and the
//! objects have no body (`wasn't processed yet`).

use std::path::Path;

use kuna_jvm::kuna_corpus::{count_failed_methods, load_cases, run_case, with_engine_stack, Case};
use kuna_jvm::kuna_fixes::{KunaFix, KunaFixes};

const CASE: &str = "pkg/TestInlinedObjectCaptureKt$dropFirst$$inlined$source$1";
const CREATOR: &str = "TestInlinedObjectCaptureKt$dropFirst$$inlined$source$1.this";
const KT_CREATOR: &str = "this@`TestInlinedObjectCaptureKt$dropFirst$$inlined$source$1`";

fn text_with(java: bool, fixes: KunaFixes) -> String {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../tests/jvm/cases.tsv");
    let mut case: Case = load_cases(&manifest)
        .unwrap()
        .into_iter()
        .find(|c| c.id == format!("kuna-fixes/{CASE}"))
        .unwrap_or_else(|| panic!("no case kuna-fixes/{CASE}"));
    if java {
        case.options
            .push(("jvm_plugins".to_string(), "0".to_string()));
    }
    case.fixes = Some(fixes);
    with_engine_stack(move || run_case(&case).actual).expect("the compared unit")
}

#[test]
fn kotlin_objects_read_the_creating_method() {
    let fixed = text_with(false, KunaFixes::DEFAULT);
    assert_eq!(count_failed_methods(&fixed), 0, "{fixed}");
    for read in [
        "empty.element = false",
        &format!("if (skipped.element >= {KT_CREATOR}.`$count$inlined`) {{"),
        "val var3: Int = skipped.element++",
        &format!("it + {KT_CREATOR}.`$fallback$inlined`, `$completion`"),
    ] {
        assert!(fixed.contains(read), "{read}\n{fixed}");
    }

    let broken = text_with(
        false,
        KunaFixes::DEFAULT.without(KunaFix::AnonymousClassCreatedElsewhere),
    );
    assert_eq!(count_failed_methods(&broken), 2, "{broken}");
}

#[test]
fn java_objects_read_the_creating_method() {
    let fixed = text_with(true, KunaFixes::DEFAULT);
    assert_eq!(fixed.matches("= new Sink() {").count(), 2, "{fixed}");
    for read in [
        "empty.element = false;",
        &format!("if (skipped.element >= {CREATOR}.$count$inlined) {{"),
        "Object var10000x = $this$dropFirst_u24lambda_u240.put(value, $completionx);",
        &format!("it + {CREATOR}.$fallback$inlined, $completion"),
    ] {
        assert!(fixed.contains(read), "{read}\n{fixed}");
    }
    assert!(
        !fixed.contains("TestInlinedObjectCaptureKt.super"),
        "{fixed}"
    );

    let broken = text_with(
        true,
        KunaFixes::DEFAULT.without(KunaFix::AnonymousClassCreatedElsewhere),
    );
    assert_eq!(
        broken.matches(" wasn't processed yet! */;").count(),
        2,
        "{broken}"
    );
}
