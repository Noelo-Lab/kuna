//! The Kotlin printer fixes (`kt_when_case_type`, `kt_method_object_return_type`,
//! `kt_secondary_constructor_line`, `kt_range_operand_precedence`,
//! `kt_lambda_singleton_returns`, `kt_type_parameter_id`, `kt_java_cast_syntax`), each on its
//! kuna-fixes case with only its own fix turned off, and the conversion table.

use std::path::Path;

use kuna_jvm::code_type::CodeType;
use kuna_jvm::kuna_corpus::{load_cases, run_case, with_engine_stack, Case};
use kuna_jvm::kuna_fix_kt_java_cast_syntax::conversion;
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

/// The default text and the text with only `fix` off.
fn fixed_and_without(id: &str, fix: KunaFix) -> (String, String) {
    (
        text_with(id, KunaFixes::DEFAULT),
        text_with(id, KunaFixes::DEFAULT.without(fix)),
    )
}

#[test]
fn when_labels_take_the_selector_type() {
    let (fixed, without) = fixed_and_without("pkg/TestWhenCaseTypeKt", KunaFix::KtWhenCaseType);
    assert!(fixed.contains("      45679 -> {"), "{fixed}");
    assert!(without.contains("      '뉯' -> {"), "{without}");
}

#[test]
fn an_object_returning_method_has_a_return_type() {
    let (fixed, without) = fixed_and_without(
        "pkg/TestMethodObjectReturnType",
        KunaFix::KtMethodObjectReturnType,
    );
    assert!(
        fixed.contains("fun describe(value: Any?): Any? {"),
        "{fixed}"
    );
    assert!(without.contains("fun describe(value: Any?) {"), "{without}");
}

#[test]
fn a_bodiless_secondary_constructor_ends_its_line() {
    let (fixed, without) = fixed_and_without(
        "pkg/TestSecondaryConstructorLine",
        KunaFix::KtSecondaryConstructorLine,
    );
    assert!(fixed.contains("constructor() : this(0)\n   }\n"), "{fixed}");
    assert!(without.contains("constructor() : this(0)   }"), "{without}");
}

#[test]
fn a_loose_range_bound_is_parenthesised() {
    let (fixed, without) = fixed_and_without(
        "pkg/TestRangeOperandPrecedenceKt",
        KunaFix::KtRangeOperandPrecedence,
    );
    assert!(
        fixed.contains("in (if (s[0] == '-') 1 else 0) until length)"),
        "{fixed}"
    );
    assert!(
        without.contains("in if (s[0] == '-') 1 else 0 until length)"),
        "{without}"
    );
}

#[test]
fn a_singleton_lambda_returns_early_with_its_label() {
    let (fixed, without) = fixed_and_without(
        "pkg/TestLambdaSingletonReturnsKt",
        KunaFix::KtLambdaSingletonReturns,
    );
    assert!(fixed.contains("= lambda_1@{ items, key ->"), "{fixed}");
    assert!(fixed.contains("return@lambda_1 i\n"), "{fixed}");
    assert_eq!(fixed.matches("@{").count(), 1, "{fixed}");
    assert!(without.contains("         i\n"), "{without}");
}

#[test]
fn class_type_parameters_are_named() {
    let (fixed, without) = fixed_and_without("pkg/TestTypeParameterId", KunaFix::KtTypeParameterId);
    assert!(fixed.contains("public fun key(i: Int): K {"), "{fixed}");
    assert!(
        fixed.contains("public fun pairs(values: List<V>): Map<K, V> {"),
        "{fixed}"
    );
    assert!(
        fixed.contains("public fun advance(step: V): Pair<K, V> {"),
        "{fixed}"
    );
    assert!(
        without.contains("public fun key(i: Int): Any {"),
        "{without}"
    );
}

#[test]
fn casts_are_written_in_kotlin() {
    let (fixed, without) = fixed_and_without("pkg/TestJavaCastSyntax", KunaFix::KtJavaCastSyntax);
    assert!(
        fixed.contains("return Math.min(byteCount, (8192 - limit).toLong()).toInt()"),
        "{fixed}"
    );
    assert!(fixed.contains("return d.toInt().toByte()"), "{fixed}");
    assert!(fixed.contains("return this.items[i] as T"), "{fixed}");
    assert!(
        without.contains("return (int)Math.min(byteCount, (long)(8192 - limit))"),
        "{without}"
    );
}

#[test]
fn conversions_follow_java_casts() {
    use CodeType::*;
    assert_eq!(conversion(Long, Int), ".toInt()");
    assert_eq!(conversion(Char, Int), ".code");
    assert_eq!(conversion(Char, Long), ".code.toLong()");
    assert_eq!(conversion(Int, Char), ".toChar()");
    assert_eq!(conversion(Long, Char), ".toInt().toChar()");
    assert_eq!(conversion(Double, Byte), ".toInt().toByte()");
    assert_eq!(conversion(Float, Short), ".toInt().toShort()");
    assert_eq!(conversion(Long, Short), ".toShort()");
    assert_eq!(conversion(Int, Double), ".toDouble()");
}
