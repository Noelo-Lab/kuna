//! The Kotlin constructor-reading and labelled-jump fixes of ktprint4, one at a time on their
//! kuna-fixes cases, each with only its own fix turned off (`KunaFixes::without`).

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

#[test]
fn an_abstract_class_has_constructors() {
    let (fixed, without) = fixed_and_without(
        "pkg/TestAbstractClassConstructor",
        KunaFix::KtAbstractClassConstructor,
    );
    assert!(fixed.contains("abstract class Shape(name: String) {"), "{fixed}");
    assert!(fixed.contains("abstract class Counter protected constructor() {"), "{fixed}");
    assert!(fixed.contains("public constructor(arity: Int) : this(arity, null)"), "{fixed}");
    assert!(
        fixed.contains("abstract class Checked(limit: Int) : TestAbstractClassConstructor.Shape(\"checked\")"),
        "{fixed}"
    );
    assert!(!fixed.contains("open fun "), "{fixed}");
    assert!(without.contains("open fun Shape(name: java.lang.String) {"), "{without}");
    assert!(without.contains("open fun Tagged(arity: Int) {"), "{without}");
}

#[test]
fn a_constructor_body_after_its_first_block_is_kept() {
    let (fixed, without) = fixed_and_without(
        "pkg/TestConstructorBodyBeyondFirstBlock",
        KunaFix::KtConstructorBodyAfterFirstBlock,
    );
    assert!(fixed.contains("throw IllegalStateException(\"big\")"), "{fixed}");
    assert!(fixed.contains("throw IllegalArgumentException(\"negative\")"), "{fixed}");
    assert!(!without.contains("IllegalStateException"), "{without}");
    assert!(!without.contains("IllegalArgumentException"), "{without}");
    assert!(without.contains("public constructor(a: Int) : this(a, a)\n"), "{without}");
}

#[test]
fn an_inline_class_parameter_constructor_is_the_real_one() {
    let (fixed, without) = fixed_and_without(
        "pkg/TestInlineClassConstructorAccessor",
        KunaFix::KtInlineClassConstructorAccessor,
    );
    assert!(fixed.contains(": TestInlineClassConstructorAccessor.Range(\n         start, end, 1\n      )"), "{fixed}");
    assert!(!fixed.contains("fun Closed("), "{fixed}");
    assert!(!fixed.contains("fun Timed("), "{fixed}");
    assert!(without.contains("fun Closed(start: Int, end: Int) {"), "{without}");
    assert!(without.contains("super(start, end, 1, null)"), "{without}");
}

#[test]
fn a_constructor_without_a_descriptor_is_found() {
    let (fixed, without) = fixed_and_without(
        "pkg/TestConstructorDefaultSignature",
        KunaFix::KtConstructorDefaultSignature,
    );
    assert!(!fixed.contains("fun TestConstructorDefaultSignature()"), "{fixed}");
    assert!(without.contains("fun TestConstructorDefaultSignature() {"), "{without}");
}

#[test]
fn a_jump_out_of_a_lambda_is_a_return() {
    let (fixed, without) =
        fixed_and_without("pkg/TestJumpToNonLoopLabel", KunaFix::KtJumpToNonLoopLabel);
    assert!(fixed.contains("run label19@{\n         if (index < this.size) {"), "{fixed}");
    assert!(fixed.contains("return@label19\n"), "{fixed}");
    assert!(fixed.contains("return@label32\n"), "{fixed}");
    assert!(!fixed.contains("break@"), "{fixed}");
    assert!(!fixed.contains("continue@"), "{fixed}");
    assert!(without.contains("label19@\n      if (index < this.size) {"), "{without}");
    assert!(without.contains("break@label19"), "{without}");
    assert!(without.contains("continue@label32"), "{without}");
}
