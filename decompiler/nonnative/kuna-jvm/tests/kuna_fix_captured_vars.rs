//! The fixes `captured_var_clash_name` and `captured_var_through_cast` on
//! their kuna-fixes cases, each alone off (`KunaFixes::without`).

use std::path::Path;

use kuna_jvm::kuna_corpus::{count_failed_methods, load_cases, run_case, with_engine_stack, Case};
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
fn captured_variables_read_the_renamed_variable() {
    let fixed = text_with("pkg/TestCapturedClashName", KunaFixes::DEFAULT);
    for read in [
        "final int firstx = other[0];",
        "return firstx;",
        "int totalx = other.length * 2;",
        "apply(base -> base + totalx, 1);",
    ] {
        assert!(fixed.contains(read), "{read}\n{fixed}");
    }

    let broken = text_with(
        "pkg/TestCapturedClashName",
        KunaFixes::DEFAULT.without(KunaFix::CapturedVarClashName),
    );
    for read in ["return first;", "apply(base -> base + total, 1);"] {
        assert!(broken.contains(read), "{read}\n{broken}");
    }
}

#[test]
fn a_cast_copy_is_captured_as_its_variable() {
    let fixed = text_with("pkg/TestCastCaptureKt", KunaFixes::DEFAULT);
    assert_eq!(count_failed_methods(&fixed), 0, "{fixed}");
    for read in ["impl$iv.fire(code + 1)", "impl$iv.fire(\"closed $code\")"] {
        assert!(fixed.contains(read), "{read}\n{fixed}");
    }
    assert!(!fixed.contains("$handle"), "{fixed}");

    let broken = text_with(
        "pkg/TestCastCaptureKt",
        KunaFixes::DEFAULT.without(KunaFix::CapturedVarThroughCast),
    );
    for read in ["this.$handle.fire(code + 1)", "this.$handle.fire(\"closed $code\")"] {
        assert!(broken.contains(read), "{read}\n{broken}");
    }
}
