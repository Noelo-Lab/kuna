//! The fixes `finally_exit_versions`, `var_merge_lost_definition` and
//! `head_definition_scope` on their kuna-fixes cases: each alone off
//! (`KunaFixes::without`) prints Vineflower's text, and the cases of the
//! other two do not depend on it.

use std::path::Path;

use kuna_jvm::kuna_corpus::{load_cases, run_case, with_engine_stack, Case};
use kuna_jvm::kuna_fixes::{KunaFix, KunaFixes};

fn text_with(id: &str, fixes: KunaFixes) -> String {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../tests/jvm/cases.tsv");
    let mut case: Case = load_cases(&manifest)
        .unwrap()
        .into_iter()
        .find(|c| c.id == id)
        .expect("the case");
    case.fixes = Some(fixes);
    with_engine_stack(move || run_case(&case).actual).expect("the compared unit")
}

const FINALLY_JAVA: &str = "kuna-fixes/pkg/TestFinallyExitVersions";
const FINALLY_KT: &str = "kuna-fixes/pkg/TestKtFinallyExitVersions";
const LOST: &str = "kuna-fixes/pkg/TestVarMergeLostDefinition";
const HEAD: &str = "kuna-fixes/pkg/TestKtHeadDefinitionScope";

#[test]
fn a_finally_body_write_reaches_the_code_after_the_try() {
    let fixed = text_with(FINALLY_JAVA, KunaFixes::DEFAULT);
    for kept in ["var1.use(var6);", "var1.use(var12);", "var3 = \"s\";"] {
        assert!(fixed.contains(kept), "{kept}\n{fixed}");
    }
    assert!(!fixed.contains("Object var7 = null;"), "{fixed}");
    let broken = text_with(
        FINALLY_JAVA,
        KunaFixes::DEFAULT.without(KunaFix::FinallyExitVersions),
    );
    assert!(broken.contains("Object var7 = null;"), "{broken}");
    assert!(!broken.contains("var1.use(var12);"), "{broken}");

    let fixed = text_with(FINALLY_KT, KunaFixes::DEFAULT);
    for kept in [
        "x = k + 7",
        "src.use(x)",
        "var6 = 2",
        "src.use(var6)",
        "src.use(var10)",
    ] {
        assert!(fixed.contains(kept), "{kept}\n{fixed}");
    }
    let broken = text_with(
        FINALLY_KT,
        KunaFixes::DEFAULT.without(KunaFix::FinallyExitVersions),
    );
    for inlined in ["src.use(1)", "src.use(k)", "src.use(0)"] {
        assert!(broken.contains(inlined), "{inlined}\n{broken}");
    }
}

#[test]
fn a_refused_merge_keeps_the_declaration() {
    let fixed = text_with(LOST, KunaFixes::DEFAULT);
    for kept in [
        "TestVarMergeLostDefinition.Item var6;",
        "TestVarMergeLostDefinition.Item var3;",
    ] {
        assert!(fixed.contains(kept), "{kept}\n{fixed}");
    }
    let broken = text_with(
        LOST,
        KunaFixes::DEFAULT.without(KunaFix::VarMergeLostDefinition),
    );
    assert!(!broken.contains("Item var6;"), "{broken}");
    assert!(broken.contains("var6 = var1.item();"), "{broken}");
    for other in [KunaFix::FinallyExitVersions, KunaFix::HeadDefinitionScope] {
        assert_eq!(text_with(LOST, KunaFixes::DEFAULT.without(other)), fixed);
    }
}

#[test]
fn a_switch_head_declaration_stays_in_scope() {
    let fixed = text_with(HEAD, KunaFixes::DEFAULT);
    assert!(
        fixed.contains("TestKtHeadDefinitionScope.Cursor e;"),
        "{fixed}"
    );
    assert!(fixed.contains("int ex = src.count();"), "{fixed}");
    let broken = text_with(
        HEAD,
        KunaFixes::DEFAULT.without(KunaFix::HeadDefinitionScope),
    );
    assert!(broken.contains("int e = src.count();"), "{broken}");
    for other in [
        KunaFix::FinallyExitVersions,
        KunaFix::VarMergeLostDefinition,
    ] {
        assert_eq!(text_with(HEAD, KunaFixes::DEFAULT.without(other)), fixed);
    }
}
