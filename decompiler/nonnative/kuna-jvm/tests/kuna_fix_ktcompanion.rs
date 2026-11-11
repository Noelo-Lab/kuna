//! The fixes `kt_companion_static_copy` and `kt_reused_inline_object`, and
//! `kt_extension_call`'s parameter-type test, on their kuna-fixes cases, each
//! fix alone off (`KunaFixes::without`) and on.

use std::path::Path;

use kuna_jvm::kuna_corpus::{load_cases, run_case, with_engine_stack, Case};
use kuna_jvm::kuna_fixes::{KunaFix, KunaFixes};

fn case(id: &str) -> Case {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../tests/jvm/cases.tsv");
    load_cases(&manifest)
        .unwrap()
        .into_iter()
        .find(|c| c.id == format!("kuna-fixes/{id}"))
        .expect("the case")
}

fn text(mut case: Case, fixes: KunaFixes) -> String {
    case.fixes = Some(fixes);
    with_engine_stack(move || run_case(&case).actual).expect("the compared unit")
}

const COMPANION: &str = "pkg/TestKtCompanionOverride";
const MEMBER_TYPE: &str = "pkg/TestKtExtensionMemberType";
const REUSED: &str = "pkg/TestKtReusedInlineObject";

/// The part of a class's text before its companion object.
fn before_companion(text: &str) -> &str {
    &text[..text
        .find("companion object Default")
        .expect("the companion")]
}

#[test]
fn a_class_keeps_the_members_its_companion_overrides() {
    let fixed = text(case(COMPANION), KunaFixes::DEFAULT);
    let own = before_companion(&fixed);
    for member in [
        "public abstract fun nextBits(bits: Int): Int\n",
        "public open fun next(): Int {\n",
        "public open fun next(until: Int): Int {\n",
        "public abstract val size: Int\n",
        "public open val label: String\n",
        "private fun step(x: Int): Int {\n      return x + 1\n",
    ] {
        assert!(own.contains(member), "{member}: {fixed}");
    }
    assert!(!own.contains("fun twice"), "{fixed}");
    let broken = text(
        case(COMPANION),
        KunaFixes::DEFAULT.without(KunaFix::KtCompanionStaticCopy),
    );
    let own = before_companion(&broken);
    assert!(!own.contains("fun next"), "{broken}");
    assert!(!own.contains("fun step("), "{broken}");
}

#[test]
fn an_extension_call_on_a_member_that_cannot_take_its_arguments() {
    let fixed = text(case(MEMBER_TYPE), KunaFixes::DEFAULT);
    for call in [
        "return dice.roll(range)\n",
        "return dice.mark(count)\n",
        "return TestKtExtensionMemberTypeKt.pick(dice, range)\n",
        "return TestKtExtensionMemberTypeKt.scale(dice, step)\n",
    ] {
        assert!(fixed.contains(call), "{call}: {fixed}");
    }
}

#[test]
fn a_reused_inline_object_is_the_inline_call() {
    let fixed = text(case(REUSED), KunaFixes::DEFAULT);
    assert_eq!(fixed.matches("return orderBy(key)\n").count(), 2, "{fixed}");
    assert!(fixed.contains("return orderByDescending(key)\n"), "{fixed}");
    assert!(fixed.contains("return labelBy(name)\n"), "{fixed}");
    let broken = text(
        case(REUSED),
        KunaFixes::DEFAULT.without(KunaFix::KtReusedInlineObject),
    );
    assert_eq!(
        broken.matches("$VF: Couldn't be decompiled").count(),
        4,
        "{broken}"
    );
}

/// With the declaring file class written first, its objects are processed
/// and the writer reaches their bodies: kotlin-stdlib's failure.
#[test]
fn a_reused_inline_object_of_a_class_written_before() {
    let mut first = case(REUSED);
    first.class_files.sort_by_key(|p| {
        !p.file_name()
            .and_then(|n| n.to_str())
            .is_some_and(|n| n.starts_with("TestKtReusedInlineObjectKt"))
    });
    let fixed = text(first.clone(), KunaFixes::DEFAULT);
    assert_eq!(fixed.matches("return orderBy(key)\n").count(), 2, "{fixed}");
    let broken = text(
        first.clone(),
        KunaFixes::DEFAULT.without(KunaFix::KtReusedInlineObject),
    );
    assert!(!broken.contains("return orderBy(key)"), "{broken}");
    let compat = text(
        first,
        KunaFixes::from_options([("jvm_vineflower_compat", "on")]),
    );
    assert!(compat.contains("$VF: Couldn't be decompiled"), "{compat}");
    assert!(!compat.contains("wasn't processed yet"), "{compat}");
}
