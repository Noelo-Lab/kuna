//! Corrections to `var_merge_narrowing` (a narrowing merge the LocalVariableTable declares as
//! one local is widened to the local's type, not refused) and `kt_reference_anonymous_super_call`
//! (a reference's signature constant split at its `(`, for a mangled JVM name), on their
//! kuna-fixes cases.

use std::path::Path;

use kuna_jvm::kuna_corpus::{count_failed_methods, load_cases, run_case, with_engine_stack, Case};
use kuna_jvm::kuna_fix_kt_reference_anonymous_super_call::split_signature;
use kuna_jvm::kuna_fixes::{KunaFix, KunaFixes};

fn text_with(id: &str, fixes: KunaFixes) -> String {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../tests/jvm/cases.tsv");
    let mut case: Case = load_cases(&manifest)
        .unwrap()
        .into_iter()
        .find(|c| c.id == format!("kuna-fixes/{id}"))
        .unwrap_or_else(|| panic!("no case kuna-fixes/{id}"));
    case.fixes = Some(fixes);
    with_engine_stack(move || run_case(&case).actual).expect("the compared unit")
}

#[test]
fn signature_is_split_at_its_parenthesis() {
    assert_eq!(
        split_signature("mark-1pYyLzM(II)I"),
        Some(("mark-1pYyLzM".to_string(), "(II)I".to_string()))
    );
    assert_eq!(
        split_signature("add(II)I"),
        Some(("add".to_string(), "(II)I".to_string()))
    );
    assert_eq!(
        split_signature("<init>(I)V"),
        Some(("<init>".to_string(), "(I)V".to_string()))
    );
    assert_eq!(split_signature("add"), None);
}

#[test]
fn mangled_references_are_written() {
    let fixed = text_with("pkg/TestReferenceMangledKt", KunaFixes::DEFAULT);
    assert_eq!(count_failed_methods(&fixed), 0, "{fixed}");
    assert!(fixed.contains("(h::mark as KFunction)"), "{fixed}");
    assert!(fixed.contains("(MangledHolder::mark as KFunction)"), "{fixed}");
}

#[test]
fn declared_local_is_merged_with_its_type() {
    const ID: &str = "pkg/TestVarMergeLvtWidening";
    let fixed = text_with(ID, KunaFixes::DEFAULT);
    assert!(
        fixed.contains("      mods = new TestVarMergeLvtWidening.LinkedMods(mods);\n"),
        "{fixed}"
    );
    assert!(!fixed.contains("var2"), "{fixed}");
    assert!(fixed.contains("String s = (String)src.next();"), "{fixed}");
    let without = text_with(
        ID,
        KunaFixes::DEFAULT
            .without(KunaFix::VarMergeNarrowing)
            .without(KunaFix::VarMergeDeclaredType),
    );
    assert!(without.contains("o = (String)src.next();"), "{without}");
}
