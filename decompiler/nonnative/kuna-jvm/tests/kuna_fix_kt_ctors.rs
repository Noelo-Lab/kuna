//! The Kotlin constructor, parameter and default-value fixes, one at a time on their
//! kuna-fixes cases, each with only its own fix turned off (`KunaFixes::without`).

use std::path::Path;

use kuna_jvm::kuna_corpus::{count_failed_methods, load_cases, run_case, with_engine_stack, Case};
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
fn only_metadata_varargs_are_vararg() {
    let (fixed, without) =
        fixed_and_without("pkg/TestVarargFromMetadata", KunaFix::KtVarargFromMetadata);
    assert!(
        fixed.contains("class TestVarargFromMetadata(names: Array<String>, counts: IntArray)"),
        "{fixed}"
    );
    assert!(
        fixed.contains("constructor(first: Int, vararg rest: Int)"),
        "{fixed}"
    );
    assert!(
        fixed.contains("fun sum(vararg values: Int): Int"),
        "{fixed}"
    );
    assert!(
        fixed.contains("constructor(vararg parts: String)"),
        "{fixed}"
    );
    assert!(
        fixed.contains("fun first(items: Array<String>): String"),
        "{fixed}"
    );
    assert!(
        without.contains("(vararg names: Any, counts: IntArray)"),
        "{without}"
    );
    assert!(
        without.contains("fun sum(values: IntArray): Int"),
        "{without}"
    );
}

#[test]
fn a_default_is_what_the_branch_assigns_to_the_parameter() {
    let (fixed, without) = fixed_and_without(
        "pkg/TestDefaultArgAssignedParam",
        KunaFix::KtDefaultArgAssignedParam,
    );
    assert!(
        fixed.contains("fun chain(s: Int, t: Int = ...): Int"),
        "{fixed}"
    );
    assert!(
        fixed.contains("fun twice(x: Int, y: Int = ...): Int"),
        "{fixed}"
    );
    assert!(
        fixed.contains("fun plain(a: Int, b: Int = this.measure(a) + 1): Int"),
        "{fixed}"
    );
    assert!(
        without.contains("fun chain(s: Int, t: Int = this.measure(s)): Int"),
        "{without}"
    );
}

#[test]
fn an_inferred_constructor_call_has_no_diamond() {
    let (fixed, without) =
        fixed_and_without("pkg/TestConstructorDiamond", KunaFix::KtConstructorDiamond);
    assert!(!fixed.contains("<>"), "{fixed}");
    assert!(
        fixed.contains("return TestConstructorDiamond.Bag(0)"),
        "{fixed}"
    );
    assert!(
        without.contains("return TestConstructorDiamond.Bag<>(0)"),
        "{without}"
    );
}

#[test]
fn a_local_class_note_is_inline() {
    let (fixed, without) = fixed_and_without(
        "pkg/TestLocalVisibilityComment",
        KunaFix::KtLocalVisibilityComment,
    );
    assert!(
        fixed.contains(
            "\n         /* $VF: local visibility outside of method */ internal inner class Sub : "
        ),
        "{fixed}"
    );
    assert!(!fixed.contains("methodSupplier"), "{fixed}");
    assert!(
        without.contains("outside of methodSupplier\ninternal inner class Sub"),
        "{without}"
    );
}

#[test]
fn an_inner_class_finds_its_defaults() {
    let (fixed, without) = fixed_and_without(
        "pkg/TestDefaultArgsCtorDescriptor",
        KunaFix::KtDefaultArgsCtorDescriptor,
    );
    assert!(
        fixed.contains("inner class Sink(finished: Boolean = false, limit: Long = 16L)"),
        "{fixed}"
    );
    assert!(fixed.contains("class Nested(size: Int = 8)"), "{fixed}");
    assert!(
        without.contains("inner class Sink(finished: Boolean = ..., limit: Long = ...)"),
        "{without}"
    );
}

#[test]
fn supertype_calls_name_their_arguments_where_position_is_not_enough() {
    let (fixed, without) = fixed_and_without(
        "pkg/TestDefaultArgsSuperNamed",
        KunaFix::KtDefaultArgsSuperCall,
    );
    assert_eq!(count_failed_methods(&fixed), 0, "{fixed}");
    assert!(
        fixed.contains("Sub : TestDefaultArgsSuperNamed(a = x, c = 9L)"),
        "{fixed}"
    );
    assert!(
        fixed.contains("Sub : TestDefaultArgsSuperNamed.Inner(z)"),
        "{fixed}"
    );
    assert!(
        fixed.contains("Sub : TestDefaultArgsSuperNamed.Overloaded(a = x)"),
        "{fixed}"
    );
    assert!(
        fixed.contains("Sub : TestDefaultArgsSuperNamed.Overloaded(x)"),
        "{fixed}"
    );
    assert_eq!(count_failed_methods(&without), 3, "{without}");
}
