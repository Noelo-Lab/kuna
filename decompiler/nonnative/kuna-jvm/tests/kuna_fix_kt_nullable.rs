//! The fixes of ktnullable on their kuna-fixes cases, each with only its own fix turned off
//! (`KunaFixes::without`).

use std::path::Path;

use kuna_jvm::kotlin::kuna_fix_kt_nullable_local::nullable;
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

/// The case's text by default and with `fixes` off: each `fixed` piece is in the first and not in the
/// second, each `without` piece in the second and not in the first.
fn check(id: &str, fixes: &[KunaFix], fixed: &[&str], without: &[&str]) {
    let on = text_with(id, KunaFixes::DEFAULT);
    let off = text_with(
        id,
        fixes.iter().fold(KunaFixes::DEFAULT, |f, x| f.without(*x)),
    );
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
fn nullable_type_text() {
    assert_eq!(nullable("String"), "String?");
    assert_eq!(nullable("Node?"), "Node?");
    assert_eq!(nullable("MutableList<String>"), "MutableList<String>?");
    assert_eq!(nullable("(Int) -> Unit"), "((Int) -> Unit)?");
}

#[test]
fn locals_holding_null_are_nullable() {
    check(
        "pkg/TestNullableLocal",
        &[KunaFix::KtNullableLocal],
        &[
            "var node: TestNullableLocal.Node? = this.head",
            "var h: String? = host",
            "var found: String? = null",
        ],
        &[
            "var node: TestNullableLocal.Node = this.head",
            "var h: String = host",
            "var found: String = null",
        ],
    );
}

#[test]
fn checked_elvis_value_stays_non_null() {
    let on = text_with("pkg/TestNullableLocal", KunaFixes::DEFAULT);
    assert!(
        on.contains("val var10000: String? = this.cache.get(key) as String?"),
        "{on}"
    );
    let on = text_with("pkg/TestKtFieldReadOnce", KunaFixes::DEFAULT);
    assert!(
        on.contains("val h: TestKtFieldReadOnce.Segment = var10000"),
        "{on}"
    );
}

#[test]
fn casts_of_nullable_values_let_null_through() {
    check(
        "pkg/TestNullableCast",
        &[KunaFix::KtNullableCast],
        &[
            "this.names.get(key) as String?",
            "this.describeName(this.names.get(key) as String?)",
        ],
        &["this.describeName(this.names.get(key) as String)"],
    );
}

#[test]
fn field_read_once_for_elvis_and_safe_call() {
    check(
        "pkg/TestKtFieldReadOnce",
        &[KunaFix::KtFieldReadReuse],
        &[
            "val var10000: TestKtFieldReadOnce.Segment? = this.head",
            "return var10000.pos",
            "val h: TestKtFieldReadOnce.Segment = var10000",
        ],
        &["return this.head.pos", "if (this.head != null) {"],
    );
}

#[test]
fn kept_variable_is_read_at_each_use() {
    check(
        "pkg/TestPartialInline",
        &[KunaFix::KtPartialInline],
        &[
            "if (var10000 == null) {",
            "if (h == null) {",
            "synchronized (box) {",
        ],
        &[
            "if (text == null) {",
            "if ((state as? TestPartialInline.Box) == null) {",
            "if (host == null) {",
        ],
    );
}

#[test]
fn file_initializers_replace_the_remainder() {
    check(
        "pkg/TestFileStaticInitializer",
        &[KunaFix::KtFileStaticInitializer],
        &[
            "public val loaded: Boolean = run {",
            "public val squares: IntArray = run {",
            "   var a: IntArray = var7",
        ],
        &["fun {", "public val loaded: Boolean\n"],
    );
}
