//! The fixes of kttypes2 on their kuna-fixes cases, each with only its own fix turned off
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

fn text(id: &str, fixes: KunaFixes) -> String {
    let mut case = fix_case(id);
    case.fixes = Some(fixes);
    with_engine_stack(move || run_case(&case).actual).expect("the compared unit")
}

/// Each `(fixed, lost)` pair: `fixed` is in the default text and not `lost`, the reverse with
/// `fix` alone off.
fn check(id: &str, fix: KunaFix, pairs: &[(&str, &str)]) {
    let on = text(id, KunaFixes::DEFAULT);
    let off = text(id, KunaFixes::DEFAULT.without(fix));
    for (fixed, lost) in pairs {
        assert!(on.contains(fixed) && !on.contains(lost), "{fixed}\n{on}");
        assert!(off.contains(lost) && !off.contains(fixed), "{lost}\n{off}");
    }
}

#[test]
fn class_headers_declare_what_the_metadata_does() {
    check(
        "pkg/TestKtClassHeaderMetadata",
        KunaFix::KtClassHeaderMetadata,
        &[
            ("interface Source<out T> {", "interface Source<T> {"),
            ("interface Sink<in T> {", "interface Sink<T> {"),
            (
                "object Empty : TestKtClassHeaderMetadata.Source<Nothing> {",
                "object Empty : TestKtClassHeaderMetadata.Source {",
            ),
            (
                "class Recorder : TestKtClassHeaderMetadata.Sink<Any?> {",
                "class Recorder : TestKtClassHeaderMetadata.Sink<Any> {",
            ),
            (
                "class Countdown(left: Int) : Iterator<Int> {",
                "class Countdown(left: Int) : java.util.Iterator<Int>, KMappedMarker {",
            ),
            (
                "class Sorted<T : Comparable<T>>(",
                "class Sorted<T : java.lang.Comparable<in T>>(",
            ),
            (
                "where T : CharSequence, T : Comparable<T> {",
                "where T : CharSequence, T : java.lang.Comparable<in T> {",
            ),
        ],
    );
    let on = text("pkg/TestKtClassHeaderMetadata", KunaFixes::DEFAULT);
    assert!(!on.contains("read-only collection"), "{on}");
}

#[test]
fn object_expressions_name_the_enclosing_type_parameters() {
    check(
        "pkg/TestKtEnclosingTypeParameter",
        KunaFix::KtEnclosingTypeParameter,
        &[
            (
                "return object : pkg.TestKtEnclosingTypeParameter.Source<K> {\n         public override fun next(): K {",
                "return object : pkg.TestKtEnclosingTypeParameter.Source<Any> {\n         public override fun next(): Any {\n            return this@",
            ),
            (
                "return object : Iterator<Pair<K, T>> {",
                "return object : Iterator<Pair<Any, Any>> {",
            ),
            (
                "public override fun drainTo(sink: MutableCollection<in T>, count: Int) {\n            var target: MutableCollection<in T> = sink",
                "public override fun drainTo(sink: MutableCollection<in Any>, count: Int) {\n            var target: java.util.Collection = sink",
            ),
        ],
    );
    let on = text("pkg/TestKtEnclosingTypeParameter", KunaFixes::DEFAULT);
    assert!(
        on.contains("fun name(): pkg.TestKtEnclosingTypeParameter.Source<String> {\n      return object : pkg.TestKtEnclosingTypeParameter.Source<Any> {"),
        "{on}"
    );
}

#[test]
fn default_impls_delegates_are_left_out() {
    let id = "pkg/TestKtDefaultImplsDelegate";
    let on = text(id, KunaFixes::DEFAULT);
    let off = text(
        id,
        KunaFixes::DEFAULT.without(KunaFix::KtDefaultImplsDelegate),
    );
    let delegate = "      override fun isTriangle(): Boolean {\n         return TestKtDefaultImplsDelegate.Polygon.DefaultImpls.isTriangle(this)\n      }";
    let inherited = "fun describe(`$this`: TestKtDefaultImplsDelegate.Polygon): String {\n            return TestKtDefaultImplsDelegate.Shape.DefaultImpls.describe(`$this`)";
    for gone in [delegate, inherited] {
        assert!(off.contains(gone) && !on.contains(gone), "{gone}\n{on}");
    }
    assert_eq!(off.matches(".DefaultImpls.").count(), 14, "{off}");
    assert_eq!(on.matches(".DefaultImpls.").count(), 1, "{on}");
    for kept in [
        "public override fun describe(): String {\n         return \"square\"",
        "return TestKtDefaultImplsDelegate.Polygon.DefaultImpls.scaled(this, factor) + 1",
        "fun isTriangle(`$this`: TestKtDefaultImplsDelegate.Polygon): Boolean {\n            return `$this`.sides == 3",
    ] {
        assert!(on.contains(kept), "{kept}\n{on}");
    }
}

#[test]
fn the_fixes_are_off_in_compat_mode() {
    for id in [
        "pkg/TestKtClassHeaderMetadata",
        "pkg/TestKtEnclosingTypeParameter",
    ] {
        let compat = text(id, KunaFixes::COMPAT);
        assert!(compat.contains("interface Source<T> {"), "{compat}");
        assert!(!compat.contains("Source<out T>"), "{compat}");
    }
    let compat = text("pkg/TestKtDefaultImplsDelegate", KunaFixes::COMPAT);
    assert!(compat.contains("DefaultImpls.isTriangle(this)"), "{compat}");
}
