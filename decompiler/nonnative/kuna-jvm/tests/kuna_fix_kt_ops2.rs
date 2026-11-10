//! The Kotlin expression fixes of ktops2 on their kuna-fixes cases, each with only its own fix turned off
//! (`KunaFixes::without`): copied equality, binary numeric promotion, negated equality, operator line
//! breaks, unboxing cast types and the precedence of unwritten unboxing calls.

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

fn check(text: &str, lines: &[&str]) {
    for line in lines {
        assert!(text.contains(line), "{line}\n{text}");
    }
}

#[test]
fn copied_equality_stays_equality() {
    let (fixed, without) =
        fixed_and_without("pkg/TestCopiedEquality", KunaFix::KtCopiedEquality);
    check(
        &fixed,
        &[
            "b: Boolean = a == \"x\")",
            "b: Boolean = a === \"x\")",
            "b: Boolean = a == c)",
            "Base(a == c)",
        ],
    );
    check(&without, &["b: Boolean = a === c)", "Base(a === c)"]);
}

#[test]
fn binary_numeric_promotion_is_written() {
    let (fixed, without) = fixed_and_without(
        "pkg/TestBinaryNumericPromotion",
        KunaFix::KtBinaryNumericPromotion,
    );
    check(
        &fixed,
        &[
            "return c.code + 4",
            "return a - b",
            "return i * 10 + c.code",
            "return (c.code + 1).toChar()",
            "sb.append(c.code + 1)",
            "return c.code < 128",
            "return b.toInt() == 61",
            "return this.section.toInt() == 2",
            "return l != i.toLong()",
            "return i.toDouble() == d",
            "return s.toInt() == b.toInt()",
            "return -c.code",
        ],
    );
    check(
        &without,
        &["return c + 4", "return b == 61", "return l != i", "return -c"],
    );
}

#[test]
fn negated_equality_is_written_with_the_other_operator() {
    let (fixed, without) =
        fixed_and_without("pkg/TestNegatedEquality", KunaFix::KtNegatedEquality);
    check(
        &fixed,
        &[
            "return this.name != other.name",
            "return if (a != b) 1 else 2",
            "return c or (a != b)",
            "return (a != b) == c",
            "return i != j",
            "return a !== b",
        ],
    );
    check(
        &without,
        &["return (this.name == other.name) xor true", "return (a === b) xor true"],
    );
}

#[test]
fn line_breaks_follow_the_operator() {
    let (fixed, without) =
        fixed_and_without("pkg/TestOperatorLineBreak", KunaFix::KtOperatorLineBreak);
    check(
        &fixed,
        &["return 31 *\n", "return this.firstComponentValue.length +\n"],
    );
    check(
        &without,
        &["return this.firstComponentValue.length\n         + this.secondComponentValue"],
    );
    let flat = |t: &str| t.split_whitespace().collect::<Vec<_>>().join(" ");
    assert_eq!(flat(&fixed), flat(&without));
}

#[test]
fn unboxing_casts_take_the_kotlin_type() {
    let (fixed, alone) =
        fixed_and_without("pkg/TestUnboxingCastType", KunaFix::KtUnboxingCastType);
    assert_eq!(alone, fixed, "kt_mapped_java_type names the box classes alike");
    let without = text_with(
        "pkg/TestUnboxingCastType",
        KunaFixes::DEFAULT
            .without(KunaFix::KtUnboxingCastType)
            .without(KunaFix::KtMappedJavaType),
    );
    check(
        &fixed,
        &[
            "return !(p(x) as Boolean) && q() as Boolean",
            "return p(x) as Boolean",
            "return f() as Char",
            "return (f() as Char).code + 1",
            "if (!(p(item) as Boolean)) {",
        ],
    );
    check(
        &without,
        &[
            "return !(p(x) as java.lang.Boolean) && q() as java.lang.Boolean",
            "return f() as Character",
        ],
    );
}

#[test]
fn unwritten_unboxing_binds_as_its_cast() {
    let without = text_with(
        "pkg/TestUnboxingCastType",
        KunaFixes::DEFAULT.without(KunaFix::KtOperatorPrecedence),
    );
    check(
        &without,
        &["return !p(x) as Boolean && q() as Boolean", "if (!p(item) as Boolean) {"],
    );
}
