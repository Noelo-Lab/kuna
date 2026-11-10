//! The Kotlin writer's kuna fixes `kt_return_outside_lambda`, `kt_failed_method_hidden` and
//! `kt_object_super_call` on their kuna-fixes cases, each with only its own fix turned off
//! (`KunaFixes::without`).

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

const RETURNS: &str = "pkg/TestReturnOutsideLambdaKt";
const HIDDEN: &str = "pkg/TestFailedMethodHidden";
const SUPER: &str = "pkg/TestObjectSuperCall";

#[test]
fn a_method_without_metadata_returns_its_value() {
    let fixed = text_with(RETURNS, KunaFixes::DEFAULT);
    assert!(
        fixed.contains("\n   return `scale$default`(x, 0, 2, null)\n"),
        "{fixed}"
    );
    assert!(fixed.contains("\n               return v\n"), "{fixed}");
    assert!(fixed.contains("\n               return `$k`\n"), "{fixed}");

    let broken = text_with(
        RETURNS,
        KunaFixes::DEFAULT.without(KunaFix::KtReturnOutsideLambda),
    );
    assert!(
        broken.contains("\n   `scale$default`(x, 0, 2, null)\n"),
        "{broken}"
    );
    assert!(broken.contains("\n               v\n"), "{broken}");
}

#[test]
fn a_lambda_still_ends_in_its_value() {
    let fixed = text_with(RETURNS, KunaFixes::DEFAULT);
    assert!(fixed.contains("   return { \n      object {\n"), "{fixed}");
    assert!(!fixed.contains("return object"), "{fixed}");
}

#[test]
fn a_failed_method_keeps_its_place() {
    let failing = KunaFixes::DEFAULT.without(KunaFix::KtSyntheticObjectSuperCall);
    let fixed = text_with(HIDDEN, failing);
    assert_eq!(count_failed_methods(&fixed), 1, "{fixed}");
    assert!(
        fixed.contains("      fun twice(`$this`: TestFailedMethodHidden, `$completion`"),
        "{fixed}"
    );

    let hidden = text_with(HIDDEN, failing.without(KunaFix::KtFailedMethodHidden));
    assert_eq!(count_failed_methods(&hidden), 0, "{hidden}");
    assert!(!hidden.contains("fun twice(`$this`"), "{hidden}");
    assert!(hidden.contains("fun reset(`$this`"), "{hidden}");
}

#[test]
fn an_object_passes_its_superclass_arguments() {
    let fixed = text_with(SUPER, KunaFixes::DEFAULT);
    for header in [
        "object : pkg.TestObjectSuperCall.Base(k + 1, s), pkg.TestObjectSuperCall.Marker {",
        "object : pkg.TestObjectSuperCall.Plain() {",
        "object : pkg.TestObjectSuperCall.Base(this.q, \"c\") {",
    ] {
        assert!(fixed.contains(header), "{header}\n{fixed}");
    }

    let bare = text_with(
        SUPER,
        KunaFixes::DEFAULT.without(KunaFix::KtObjectSuperCall),
    );
    assert!(
        bare.contains("object : pkg.TestObjectSuperCall.Base, pkg.TestObjectSuperCall.Marker {"),
        "{bare}"
    );
    assert!(
        bare.contains("object : pkg.TestObjectSuperCall.Plain {"),
        "{bare}"
    );
}
