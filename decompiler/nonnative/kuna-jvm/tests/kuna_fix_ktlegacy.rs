//! The fixes `kt_legacy_parameter_checks`, `kt_inlined_to_typed_array` and
//! `kt_narrow_constant_local` on their kuna-fixes cases, each alone off
//! (`KunaFixes::without`) and on.

use std::path::Path;

use kuna_jvm::kuna_corpus::{load_cases, run_case, with_engine_stack, Case};
use kuna_jvm::kuna_fixes::{KunaFix, KunaFixes};

fn text_with(id: &str, fixes: KunaFixes) -> String {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../tests/jvm/cases.tsv");
    let mut case: Case = load_cases(&manifest)
        .unwrap()
        .into_iter()
        .find(|c| c.id == format!("kuna-fixes/{id}"))
        .expect("the case");
    case.fixes = Some(fixes);
    with_engine_stack(move || run_case(&case).actual).expect("the compared unit")
}

const CHECKS: &str = "pkg/TestKtLegacyFunctionChecks";
const TO_TYPED_ARRAY: &str = "pkg/TestKtLegacyToTypedArray";
const NARROW: &str = "pkg/TestKtNarrowConstantTemps";

#[test]
fn opening_parameter_checks_are_dropped() {
    let fixed = text_with(CHECKS, KunaFixes::DEFAULT);
    assert_eq!(fixed.matches("checkParameterIsNotNull").count(), 1, "{fixed}");
    assert!(
        fixed.contains("val n: Int = first.length\n      Intrinsics.checkParameterIsNotNull(second, \"second\")"),
        "{fixed}"
    );
    assert!(
        fixed.contains("block: () -> T): T {\n      val result"),
        "{fixed}"
    );
    let broken = text_with(
        CHECKS,
        KunaFixes::DEFAULT.without(KunaFix::KtLegacyParameterChecks),
    );
    assert_eq!(broken.matches("checkParameterIsNotNull").count(), 12, "{broken}");
    assert!(
        broken.contains("Intrinsics.checkParameterIsNotNull(block, \"block\")"),
        "{broken}"
    );
}

#[test]
fn inlined_to_typed_array_is_the_call() {
    let fixed = text_with(TO_TYPED_ARRAY, KunaFixes::DEFAULT);
    assert!(!fixed.contains("toArray("), "{fixed}");
    assert!(!fixed.contains("TypeCastException"), "{fixed}");
    assert_eq!(fixed.matches(".toTypedArray()").count(), 5, "{fixed}");
    assert_eq!(fixed.matches(".toTypedArray<Any>()").count(), 2, "{fixed}");
    assert!(
        fixed.contains("val array: Array<Int> = (set as Collection<Int>).toTypedArray()\n      return array.size + array[0]"),
        "{fixed}"
    );
    let broken = text_with(
        TO_TYPED_ARRAY,
        KunaFixes::DEFAULT.without(KunaFix::KtInlinedToTypedArray),
    );
    assert!(!broken.contains("toTypedArray"), "{broken}");
    assert_eq!(broken.matches("TypeCastException(").count(), 4, "{broken}");
}

#[test]
fn constant_locals_used_as_int_are_int() {
    let fixed = text_with(NARROW, KunaFixes::DEFAULT);
    assert!(!fixed.contains(": Short"), "{fixed}");
    assert_eq!(fixed.matches(": Byte\n").count(), 1, "{fixed}");
    assert!(fixed.contains("fun keepByte(kind: Int): Byte {\n      val var10000: Byte"), "{fixed}");
    assert!(fixed.contains("val var2: Int = 3\n"), "{fixed}");
    assert!(!fixed.contains(".toInt()"), "{fixed}");
    let broken = text_with(
        NARROW,
        KunaFixes::DEFAULT.without(KunaFix::KtNarrowConstantLocal),
    );
    assert!(broken.contains("val var2: Byte = 3\n"), "{broken}");
    assert!(broken.contains("val var2: Short = 300\n"), "{broken}");
    assert_eq!(broken.matches("return var10000.toInt()").count(), 2, "{broken}");
}
