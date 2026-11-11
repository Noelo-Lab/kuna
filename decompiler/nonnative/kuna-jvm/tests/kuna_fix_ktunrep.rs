//! The fixes `synchronized_break_exit`, `inline_marker_finally_copy` and
//! `kt_unrepresentable_declared_type` on their kuna-fixes cases, in Kotlin and
//! (the first two) in Java output, each alone off (`KunaFixes::without`) and
//! on; and `kt_jump_to_non_loop_label`'s labelled `synchronized` in Kotlin.

use std::path::Path;

use kuna_jvm::kuna_corpus::{load_cases, run_case, with_engine_stack, Case};
use kuna_jvm::kuna_fixes::{KunaFix, KunaFixes};

fn text_with(id: &str, fixes: KunaFixes, java: bool) -> String {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../tests/jvm/cases.tsv");
    let mut case: Case = load_cases(&manifest)
        .unwrap()
        .into_iter()
        .find(|c| c.id == format!("kuna-fixes/{id}"))
        .expect("the case");
    case.fixes = Some(fixes);
    if java {
        case.options.retain(|(k, _)| k != "jvm_plugins");
        case.options.push(("jvm_plugins".into(), "0".into()));
        case.compare = vec![format!("{}.java", id.rsplit('/').next().unwrap())];
    }
    with_engine_stack(move || run_case(&case).actual).expect("the compared unit")
}

const BREAK_EXIT: &str = "pkg/TestSynchronizedBreakExit";
const MARKER_FINALLY: &str = "pkg/TestInlineMarkerFinally";
const RELEASE: &str = "// $VF: monitorexit";
const UNBUILT: &str = "Could not create synchronized statement";

#[test]
fn a_return_in_synchronized_stays_in_the_block() {
    let fixed = text_with(BREAK_EXIT, KunaFixes::DEFAULT, false);
    assert!(!fixed.contains(RELEASE), "{fixed}");
    for ret in ["return false", "return -1", "return 3", "return node"] {
        assert!(fixed.contains(ret), "{ret}: {fixed}");
    }
    let broken = text_with(
        BREAK_EXIT,
        KunaFixes::DEFAULT.without(KunaFix::SynchronizedBreakExit),
        false,
    );
    assert_eq!(broken.matches(RELEASE).count(), 4, "{broken}");

    let java = text_with(BREAK_EXIT, KunaFixes::DEFAULT, true);
    assert!(!java.contains(RELEASE) && !java.contains(UNBUILT), "{java}");
    assert!(java.contains("return false;"), "{java}");
    let java_broken = text_with(
        BREAK_EXIT,
        KunaFixes::DEFAULT.without(KunaFix::SynchronizedBreakExit),
        true,
    );
    assert!(java_broken.contains(UNBUILT), "{java_broken}");
}

#[test]
fn a_labelled_synchronized_is_a_run_lambda_in_kotlin() {
    let fixed = text_with(BREAK_EXIT, KunaFixes::DEFAULT, false);
    let run = fixed.find("run label").expect("a run lambda");
    let label: String = fixed[run + 4..]
        .chars()
        .take_while(|c| c.is_ascii_alphanumeric())
        .collect();
    assert!(
        fixed[run..].starts_with(&format!("run {label}@{{\n")),
        "{fixed}"
    );
    assert!(fixed.contains(&format!("return@{label}")), "{fixed}");
    assert!(!fixed.contains(&format!("{label}:")), "{fixed}");
    assert!(!fixed.contains(&format!("break@{label}")), "{fixed}");
}

#[test]
fn an_inline_synchronized_holds_its_lock() {
    for java in [false, true] {
        let fixed = text_with(MARKER_FINALLY, KunaFixes::DEFAULT, java);
        assert!(fixed.contains("synchronized (this) {"), "{fixed}");
        assert!(!fixed.contains("$VF: monitor"), "{fixed}");
        assert!(!fixed.contains("Semaphore variable"), "{fixed}");
        assert!(!fixed.contains("finallyStart(2)"), "{fixed}");

        let broken = text_with(
            MARKER_FINALLY,
            KunaFixes::DEFAULT.without(KunaFix::InlineMarkerFinallyCopy),
            java,
        );
        assert!(broken.contains("){} // $VF: monitorenter"), "{broken}");
        assert_eq!(broken.matches("Semaphore variable").count(), 2, "{broken}");
    }
}

#[test]
fn an_unrepresentable_declared_type_is_inferred() {
    let id = "pkg/TestUnrepresentableType";
    let fixed = text_with(id, KunaFixes::DEFAULT, false);
    assert!(!fixed.contains("<unrepresentable>"), "{fixed}");
    for decl in [
        "private final val counter = object : ",
        "val source = object : ",
        "val var10003 = TestUnrepresentableType::register",
    ] {
        assert!(fixed.contains(decl), "{decl}: {fixed}");
    }
    let broken = text_with(
        id,
        KunaFixes::DEFAULT.without(KunaFix::KtUnrepresentableDeclaredType),
        false,
    );
    assert_eq!(
        broken.matches(": <unrepresentable> = ").count(),
        3,
        "{broken}"
    );
}
