//! The fix `kt_semaphore_store_liveness` on its kuna-fixes case, with only that fix turned off
//! (`KunaFixes::without`) for the Vineflower shape.

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

fn text_with(fixes: KunaFixes) -> String {
    let mut case = fix_case("pkg/TestKtFinallySemaphore");
    case.fixes = Some(fixes);
    with_engine_stack(move || run_case(&case).actual).expect("the compared unit")
}

#[test]
fn a_finally_semaphore_keeps_its_stores() {
    let fixed = text_with(KunaFixes::DEFAULT);
    let broken = text_with(KunaFixes::DEFAULT.without(KunaFix::KtSemaphoreStoreLiveness));
    for var in ["var8", "var13"] {
        assert!(
            fixed.contains(&format!(
                "var {var}: Boolean = false /* VF: Semaphore variable */\n"
            )),
            "{fixed}"
        );
        assert!(
            fixed.contains(&format!("            {var} = true\n")),
            "{fixed}"
        );
        assert!(fixed.contains(&format!("{var} = false\n")), "{fixed}");
        assert!(fixed.contains(&format!("if ({var}) {{\n")), "{fixed}");
    }
    assert!(!fixed.contains("<unknown>"), "{fixed}");
    assert!(!fixed.contains(": Boolean = true\n"), "{fixed}");
    assert!(broken.contains("         val var8: Boolean\n"), "{broken}");
    assert!(
        broken.contains("         val var13: <unknown>\n"),
        "{broken}"
    );
    assert!(broken.contains("val var11: Boolean = true\n"), "{broken}");
}
