//! The Kotlin operator fixes of ktops on their kuna-fixes cases, each with only its own fix turned off
//! (`KunaFixes::without`): operator precedence, narrow bitwise operands, reference inequality.

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
fn operands_are_parenthesised_by_kotlin_precedence() {
    let (fixed, without) =
        fixed_and_without("pkg/TestOperatorPrecedence", KunaFix::KtOperatorPrecedence);
    for line in [
        "return x or (1 shl k)",
        "return b2 xor 3968 xor (b1 shl 6)",
        "return a or (b and c)",
        "return a shl 1 or b",
        "return (a or b) shl 1",
        "return a + b shl 2",
        "return (x and (a shl b)) != 0 && (x or 4) > 3",
        "return ok and (flags != 0)",
        "return (a == b) and c",
        "return (x is TestOperatorPrecedence) or y",
        "return x ?: (a && b)",
        "return x ?: (a == b)",
        "return a with (b with c)",
        "return a with (b or c)",
        "return a or (b with c)",
    ] {
        assert!(fixed.contains(line), "{line}\n{fixed}");
    }
    for line in [
        "return x or 1 shl k",
        "return a == b and c",
        "return x ?: a && b",
        "return a with b or c",
    ] {
        assert!(without.contains(line), "{line}\n{without}");
    }
}

#[test]
fn narrow_bitwise_operands_are_widened() {
    let (fixed, without) = fixed_and_without(
        "pkg/TestNarrowBitwiseOperand",
        KunaFix::KtNarrowBitwiseOperand,
    );
    for line in [
        "return b.toInt() and 255",
        "return c.code xor 32",
        "return x shl n.toInt()",
        "return this.flags.toInt() and 15",
        "buffer[i] = (buffer[i].toInt() xor key[j].toInt()).toByte()",
    ] {
        assert!(fixed.contains(line), "{line}\n{fixed}");
    }
    for line in ["return b and 255", "return x shl n", "buffer[i] ^= key[j]"] {
        assert!(without.contains(line), "{line}\n{without}");
    }
}

#[test]
fn reference_inequality_is_an_identity_test() {
    let (fixed, without) = fixed_and_without(
        "pkg/TestReferenceInequality",
        KunaFix::KtReferenceInequality,
    );
    for line in [
        "return a === b",
        "return a !== b",
        "return !(a == b)",
        "return if (a !== b) 1 else 2",
        "while (x !== b) {",
    ] {
        assert!(fixed.contains(line), "{line}\n{fixed}");
    }
    assert!(!without.contains("!=="), "{without}");
    assert!(without.contains("return a != b"), "{without}");
}
