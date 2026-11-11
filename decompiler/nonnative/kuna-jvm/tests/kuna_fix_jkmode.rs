//! The fixes `constructor_signature_alignment` and `split_constructor_call`
//! on their kuna-fixes cases, each alone off (`KunaFixes::without`) and on,
//! and `nested_param_rename` keeping a parameter whose table name is not a
//! Java identifier (`<set-?>`) under the name its uses are written by.

use std::path::Path;

use kuna_jvm::kuna_corpus::{count_failed_methods, load_cases, run_case, with_engine_stack, Case};
use kuna_jvm::kuna_fixes::{KunaFix, KunaFixes};

const MARKER: &str = "$VF: Unable to resugar constructor";

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

#[test]
fn an_outer_only_constructor_signature_is_aligned() {
    let id = "pkg/TestOuterOnlySignature";
    let fixed = text_with(id, KunaFixes::DEFAULT);
    assert_eq!(count_failed_methods(&fixed), 0, "{fixed}");
    assert!(
        fixed.contains("Labelled(TestOuterOnlySignature.Box<String> var2) {"),
        "{fixed}"
    );
    assert!(
        fixed.contains("return new TestOuterOnlySignature.Forwarding(var1) {"),
        "{fixed}"
    );

    let broken = text_with(
        id,
        KunaFixes::DEFAULT.without(KunaFix::ConstructorSignatureAlignment),
    );
    assert_eq!(count_failed_methods(&broken), 3, "{broken}");
}

#[test]
fn a_split_constructor_call_is_joined() {
    for (id, markers, new) in [
        (
            "pkg/TestSplitConstructorCall",
            6,
            "return new SplitEntry(this.index, var10003);",
        ),
        (
            "pkg/TestSplitConstructorCallKotlin",
            2,
            "return SplitPair(this.count, this.label)",
        ),
    ] {
        let fixed = text_with(id, KunaFixes::DEFAULT);
        assert!(!fixed.contains(MARKER), "{fixed}");
        assert!(!fixed.contains(".<init>("), "{fixed}");
        assert!(fixed.contains(new), "{fixed}");

        let broken = text_with(
            id,
            KunaFixes::DEFAULT
                .without(KunaFix::SplitConstructorCall)
                .without(KunaFix::FieldReadReuse),
        );
        assert_eq!(broken.matches(MARKER).count(), markers, "{broken}");
    }
}

#[test]
fn an_invalid_table_name_is_not_declared() {
    let fixed = text_with("pkg/TestSplitConstructorCall", KunaFixes::DEFAULT);
    assert!(
        fixed.contains("public final void setName(@Nullable String var1) {"),
        "{fixed}"
    );
    assert!(
        fixed.contains("this.name = var1/* $VF was: <set-?> */;"),
        "{fixed}"
    );
    assert!(!fixed.contains(" <set-?>)"), "{fixed}");
}
