//! The Kotlin naming, accessor-flag and type-syntax fixes of ktprint7, one at a time on their
//! kuna-fixes cases, each with only its own fix turned off (`KunaFixes::without`).

use std::path::Path;

use kuna_jvm::kotlin::kuna_fix_kt_default_accessor_flags::accessor_flags;
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
fn java_keyword_debug_names_name_their_variables() {
    let (fixed, without) = fixed_and_without(
        "pkg/TestJavaKeywordNames",
        KunaFix::KtJavaKeywordVariableName,
    );
    assert!(fixed.contains("return new * 31 + default"), "{fixed}");
    assert!(fixed.contains("old == new"), "{fixed}");
    assert!(fixed.contains("var int: Int = 0"), "{fixed}");
    assert!(without.contains("return var1 * 31 + var2"), "{without}");
    assert!(without.contains("old == var2"), "{without}");
}

#[test]
fn absent_setter_flags_are_the_property_defaults() {
    let (fixed, without) = fixed_and_without(
        "pkg/TestDefaultAccessorFlags",
        KunaFix::KtDefaultAccessorFlags,
    );
    assert!(!fixed.contains("internal set"), "{fixed}");
    assert!(!fixed.contains("final set"), "{fixed}");
    assert!(fixed.contains("private set"), "{fixed}");
    assert!(without.contains("internal final set"), "{without}");
    assert_eq!(without.matches("internal set").count(), 5, "{without}");
}

#[test]
fn accessor_flags_default_to_the_property_visibility_and_modality() {
    let public_final_annotated = 0b0000_0111;
    let protected_open = (2 << 1) | (1 << 4);
    assert_eq!(accessor_flags(true, 0, public_final_annotated), 0);
    assert_eq!(accessor_flags(false, 0, public_final_annotated), 0b0000_0111);
    assert_eq!(accessor_flags(false, 0, protected_open), protected_open);
    assert_eq!(accessor_flags(false, 0, protected_open | (1 << 6)), protected_open);
}

#[test]
fn names_the_shared_writers_print_are_escaped() {
    let (fixed, without) =
        fixed_and_without("pkg/TestUnescapedNames", KunaFix::KtUnescapedName);
    for written in [
        "{ `in` ->",
        "{ _, b ->",
        "{ a, _ ->",
        "return `$this$view`.length",
        "System.`in`",
        "t.`bump$main`()",
    ] {
        assert!(fixed.contains(written), "{written}: {fixed}");
    }
    for written in [
        "{ in ->",
        "<anonymous parameter 0>",
        "return $this$view.length",
        "System.in\n",
        "t.bump$main()",
    ] {
        assert!(without.contains(written), "{written}: {without}");
    }
}

#[test]
fn underscore_names_are_written_in_backticks() {
    let (fixed, without) = fixed_and_without(
        "pkg/TestUnderscoreNames",
        KunaFix::KtReservedUnderscoreName,
    );
    assert!(fixed.contains("fun twice(`_`: Int): Int"), "{fixed}");
    assert!(fixed.contains("for (`_` in xs)"), "{fixed}");
    assert!(fixed.contains("`__` += `_`"), "{fixed}");
    assert!(without.contains("for (_ in xs)"), "{without}");
    assert!(without.contains("__ += _"), "{without}");
}

#[test]
fn signature_types_are_written_in_kotlin_syntax() {
    let (fixed, without) =
        fixed_and_without("pkg/TestJavaTypeSyntax", KunaFix::KtJavaTypeSyntax);
    for written in [
        "interface TestJavaTypeSyntax<T : java.lang.Comparable<in T>> {",
        "class Box<S> : TestJavaTypeSyntax<S> where S : java.lang.CharSequence, S : java.lang.Comparable<in S> {",
        "fun <T : java.lang.Comparable<in T>, R> sink(`$this`: TestJavaTypeSyntax<T>, items: MutableList<in R>",
        "c == IntArray::class.java",
        "c == Array<java.lang.String>::class.java",
    ] {
        assert!(fixed.contains(written), "{written}: {fixed}");
    }
    assert!(!fixed.contains(" extends "), "{fixed}");
    assert!(!fixed.contains("[]"), "{fixed}");
    for written in [
        "<T extends java.lang.Comparable<? super T>>",
        "items: MutableList<R>",
        "c == int[]::class.java",
    ] {
        assert!(without.contains(written), "{written}: {without}");
    }
}
