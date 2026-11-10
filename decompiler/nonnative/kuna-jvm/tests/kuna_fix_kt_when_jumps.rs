//! The Kotlin file-class constructor, jump and supertype-call fixes of ktprint5, one at a time on
//! their kuna-fixes cases, each with only its own fix turned off (`KunaFixes::without`).

use std::path::Path;

use kuna_jvm::kuna_corpus::{load_cases, run_case, with_engine_stack, Case};
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

fn fixed_and_without(id: &str, fix: KunaFix) -> (String, String) {
    (
        text_with(id, KunaFixes::DEFAULT),
        text_with(id, KunaFixes::DEFAULT.without(fix)),
    )
}

fn lines(text: &str, line: &str) -> usize {
    text.lines().filter(|l| l.trim() == line).count()
}

#[test]
fn a_file_class_constructor_is_not_a_function() {
    let (fixed, without) = fixed_and_without(
        "pkg/TestFileClassConstructor__FileClassSizesKt",
        KunaFix::KtFileClassConstructor,
    );
    assert!(
        !fixed.contains("fun TestFileClassConstructor__FileClassSizesKt"),
        "{fixed}"
    );
    assert!(
        fixed.contains("public fun twice(size: Int): Int {"),
        "{fixed}"
    );
    assert!(
        without.contains("open fun TestFileClassConstructor__FileClassSizesKt() {\n}"),
        "{without}"
    );
}

#[test]
fn a_break_out_of_a_when_is_left_out_or_a_run_return() {
    let (fixed, without) = fixed_and_without("pkg/TestWhenBreak", KunaFix::KtWhenBreak);
    assert_eq!(lines(&fixed, "break"), 1, "{fixed}");
    assert!(fixed.contains("run label"), "{fixed}");
    assert_eq!(lines(&fixed, "return@label39"), 1, "{fixed}");
    assert_eq!(lines(&without, "break"), 6, "{without}");
    assert!(!without.contains("run label"), "{without}");
}

#[test]
fn a_try_block_jump_is_kotlin() {
    let (fixed, without) =
        fixed_and_without("pkg/TestJavaStatementJump", KunaFix::KtJavaStatementJump);
    assert_eq!(lines(&fixed, "break;"), 0, "{fixed}");
    assert_eq!(lines(&fixed, "break"), 1, "{fixed}");
    assert_eq!(lines(&without, "break;"), 2, "{without}");
}

#[test]
fn a_catch_block_keeps_its_jump() {
    let (fixed, without) = fixed_and_without("pkg/TestCatchBlockJump", KunaFix::KtCatchBlockJump);
    assert_eq!(lines(&fixed, "continue"), 1, "{fixed}");
    assert_eq!(lines(&fixed, "break"), 1, "{fixed}");
    assert_eq!(lines(&without, "continue"), 0, "{without}");
    assert_eq!(lines(&without, "break"), 0, "{without}");
}

#[test]
fn a_supertype_call_has_type_arguments() {
    let (fixed, without) = fixed_and_without(
        "pkg/TestSupertypeTypeArguments",
        KunaFix::KtSupertypeTypeArguments,
    );
    assert!(
        fixed.contains(": TestSupertypeTypeArguments.Box<Int>(v, \"int\")"),
        "{fixed}"
    );
    assert!(
        fixed.contains(": TestSupertypeTypeArguments.Box<K>(key as K, \"keyed\")"),
        "{fixed}"
    );
    assert!(
        fixed.contains(": TestSupertypeTypeArguments.Holder<E>("),
        "{fixed}"
    );
    assert!(
        without.contains(": TestSupertypeTypeArguments.Box(v, \"int\")"),
        "{without}"
    );
    assert!(
        without.contains(": TestSupertypeTypeArguments.Holder("),
        "{without}"
    );
}
