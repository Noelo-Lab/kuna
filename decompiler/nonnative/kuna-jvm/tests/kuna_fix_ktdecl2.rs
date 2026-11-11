//! The fixes of ktdecl2 on their kuna-fixes cases, each with only its own fix turned off
//! (`KunaFixes::without`), and the class-name escape.

use std::path::Path;

use crate::common;

use kuna_jvm::kotlin::kuna_fix_kt_escaped_class_name::{escape, escape_qualified};
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

/// The case's default text, and its text with `fix` alone off.
fn on_off(id: &str, fix: KunaFix) -> (String, String) {
    let text = |fixes| {
        let mut case = fix_case(id);
        case.fixes = Some(fixes);
        with_engine_stack(move || run_case(&case).actual).expect("the compared unit")
    };
    (
        text(KunaFixes::DEFAULT),
        text(KunaFixes::DEFAULT.without(fix)),
    )
}

fn check(id: &str, fix: KunaFix, pairs: &[(&str, &str)]) {
    let (on, off) = on_off(id, fix);
    for (fixed, lost) in pairs {
        assert!(on.contains(fixed) && !on.contains(lost), "{fixed}\n{on}");
        assert!(off.contains(lost) && !off.contains(fixed), "{lost}\n{off}");
    }
}

#[test]
fn class_names_escape_only_what_kotlin_cannot_name() {
    let _guard = common::install_default_context();
    assert_eq!(escape("-DeprecatedOkio"), "`-DeprecatedOkio`");
    assert_eq!(escape("in"), "`in`");
    assert_eq!(escape("Okio"), "Okio");
    assert_eq!(escape("Foo$Bar"), "Foo$Bar");
    assert_eq!(escape("<unrepresentable>"), "<unrepresentable>");
    assert_eq!(escape("kotlin/RequiresOptIn"), "kotlin/RequiresOptIn");
    assert_eq!(escape_qualified("pkg.Outer.make.1.Sub"), "pkg.Outer.make.1.Sub");
    assert_eq!(escape_qualified("okio.-DeprecatedOkio"), "okio.`-DeprecatedOkio`");
    assert_eq!(escape_qualified("Outer.in.Deep"), "Outer.`in`.Deep");
    assert_eq!(escape_qualified("java.util.List"), "java.util.List");
}

#[test]
fn escaped_class_name_backticks_types_and_expressions() {
    check(
        "pkg/TestKtEscapedClassName",
        KunaFix::KtEscapedClassName,
        &[
            ("legacy: pkg.TestKtEscapedClassName.`-Legacy`", "legacy: pkg.TestKtEscapedClassName.-Legacy"),
            ("return TestKtEscapedClassName.`in`(v)", "return TestKtEscapedClassName.in(v)"),
        ],
    );
}

#[test]
fn inner_class_constructor_uses_the_implicit_receiver() {
    check(
        "pkg/TestKtInnerClassConstructor",
        KunaFix::KtInnerClassConstructor,
        &[
            ("return Part(x)", "return TestKtInnerClassConstructor.Part(x)"),
            ("return Empty()", "return TestKtInnerClassConstructor.Empty()"),
        ],
    );
    let (on, _) = on_off("pkg/TestKtInnerClassConstructor", KunaFix::KtInnerClassConstructor);
    assert!(on.contains("return o.Part(x)"), "{on}");
}

#[test]
fn data_class_constructor_declares_its_properties() {
    check(
        "pkg/TestKtDataClassPropertyParameters",
        KunaFix::KtDataClassPropertyParameters,
        &[
            (
                "data class Header(public final val name: String, public final var value: Int)",
                "data class Header(name: String, value: Int)",
            ),
            ("Defaults(public final val a: Int = 1,", "Defaults(a: Int = 1,"),
            ("init {\n         this.size = ", "init {\n         this.name = name"),
        ],
    );
    let (on, off) = on_off("pkg/TestKtDataClassPropertyParameters", KunaFix::KtDataClassPropertyParameters);
    for member in ["fun component1()", "fun copy(", "fun hashCode()", "fun toString()", "fun equals("] {
        assert!(!on.contains(member) && off.contains(member), "{member}\n{on}");
    }
    assert!(on.contains("fun head(): A"), "{on}");
}

#[test]
fn function_parameter_name_reads_the_header_name() {
    check(
        "pkg/TestKtFunctionParameterName",
        KunaFix::KtFunctionParameterName,
        &[
            ("return this.id + other.id + extra", "return this.id + var1.id + var2"),
            ("return other is TestKtFunctionParameterName", "return var1 is TestKtFunctionParameterName"),
            ("this.id = id", "this.id = var1"),
        ],
    );
}

#[test]
fn contract_imports_contract_and_opts_in() {
    check(
        "pkg/TestKtContractImport",
        KunaFix::KtContractImport,
        &[
            (
                "InvocationKind\nimport kotlin.contracts.contract\n",
                "InvocationKind\nimport kotlin.jvm.internal",
            ),
            ("{\n   @OptIn(ExperimentalContracts::class)\n   public inline fun", "{\n   public inline fun"),
        ],
    );
}

#[test]
fn unlabeled_sequence_break_is_a_break() {
    check(
        "pkg/TestKtUnlabeledSequenceBreak",
        KunaFix::KtUnlabeledSequenceBreak,
        &[("found = var10000\n               break\n", "return@run")],
    );
}
