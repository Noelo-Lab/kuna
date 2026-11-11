//! The fixes of ktvalid2 on their kuna-fixes cases, each with only its own fix turned off
//! (`KunaFixes::without`).

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

/// The case's text by default and with `fix` alone off: each `fixed` piece is in the first and not in the
/// second, each `without` piece in the second and not in the first.
fn check(id: &str, fix: KunaFix, fixed: &[&str], without: &[&str]) {
    let on = text_with(id, KunaFixes::DEFAULT);
    let off = text_with(id, KunaFixes::DEFAULT.without(fix));
    for piece in fixed {
        assert!(on.contains(piece), "{piece}\n{on}");
        assert!(!off.contains(piece), "{piece}\n{off}");
    }
    for piece in without {
        assert!(off.contains(piece), "{piece}\n{off}");
        assert!(!on.contains(piece), "{piece}\n{on}");
    }
}

#[test]
fn locals_written_again_are_var() {
    check(
        "pkg/TestLocalFinalWrites",
        KunaFix::LocalFinalWrites,
        &["var count: Int = 0", "var j: Int = 0"],
        &["val count: Int = 0", "val j: Int = 0"],
    );
}

#[test]
fn not_null_checks_are_assertions() {
    check(
        "pkg/TestNotNullAssertion",
        KunaFix::KtNotNullAssertion,
        &[
            "return this.name!!.length",
            "node = node.next!!",
            "return this.head!!.next!!.label!!",
            "return other.head!!.value - other.head!!.next!!.value",
        ],
        &["val var10000: String? = this.name"],
    );
}

#[test]
fn legacy_not_null_checks_are_assertions() {
    check(
        "pkg/TestNotNullAssertionLegacy",
        KunaFix::KtNotNullAssertion,
        &["var node: TestNotNullAssertionLegacy.Node = this.head!!", "node = node.next!!"],
        &["Intrinsics.throwNpe()"],
    );
}

#[test]
fn stored_platform_values_are_initializers() {
    check(
        "pkg/TestExpressionValueCheck",
        KunaFix::KtExpressionValueCheck,
        &["public final val DEFAULT: String = JavaLib.version()"],
        &["DEFAULT = var10000"],
    );
    check(
        "pkg/TestExpressionValueCheckLegacy",
        KunaFix::KtExpressionValueCheck,
        &["public final val DEFAULT: String = JavaLib.version()"],
        &["Intrinsics.checkExpressionValueIsNotNull(var10000, \"version(...)\")"],
    );
}

#[test]
fn static_initializer_remainders_are_init_blocks() {
    check(
        "pkg/TestStaticInitializer",
        KunaFix::KtStaticInitializer,
        &["   init {\n      val t: IntArray = IntArray(16)", "         init {\n            var var0: Boolean"],
        &["@JvmStatic\n   fun {", "@JvmStatic\n      fun {"],
    );
}

#[test]
fn object_instance_local_reads_instance() {
    check(
        "pkg/TestObjectInstanceLocal",
        KunaFix::KtObjectInstanceLocal,
        &["public final val first: Int = TestObjectInstanceLocal.compute(3)", "TestObjectInstanceLocal.register(i)"],
        &["val var0: TestObjectInstanceLocal = TestObjectInstanceLocal()"],
    );
}

#[test]
fn property_initializers_are_found_by_field() {
    check(
        "pkg/TestPropertyInitializerKey",
        KunaFix::KtPropertyInitializerKey,
        &["val items: Array<Int> = arrayOf(1, 2, 3)", "val EMPTY: Array<Any?> = arrayOfNulls(0)"],
        &["val items: Array<Int>\n", "val EMPTY: Array<Any?>\n"],
    );
}

#[test]
fn synchronized_accessors_are_annotated() {
    check(
        "pkg/TestSynchronizedAccessor",
        KunaFix::KtSynchronizedAccessor,
        &["@get:Synchronized\n   public final var a: Int = 1", "@get:Synchronized\n   @set:Synchronized\n   public final var b: Int = 2"],
        &[],
    );
}
