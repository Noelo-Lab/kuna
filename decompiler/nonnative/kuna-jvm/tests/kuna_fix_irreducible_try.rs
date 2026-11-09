//! Fix `irreducible_with_exception_edges` on its two kuna-fixes cases: in both
//! modes, and with only this fix turned off (`KunaFixes::without`).

use std::path::Path;

use kuna_jvm::kuna_corpus::{count_failed_methods, load_cases, run_case, with_engine_stack};
use kuna_jvm::kuna_fixes::{KunaFix, KunaFixes};

const JASM: &str = "pkg/TestIrreducibleTry";
const KOTLIN: &str = "pkg/TestSuspendLoopTry";

/// The text of the `<set>/<id>` case, with `fixes` when given.
fn decompile(set: &str, id: &str, fixes: Option<KunaFixes>) -> String {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../tests/jvm/cases.tsv");
    let id = format!("{set}/{id}");
    let mut case = load_cases(&manifest)
        .unwrap()
        .into_iter()
        .find(|c| c.id == id)
        .unwrap_or_else(|| panic!("no case {id}"));
    if fixes.is_some() {
        case.fixes = fixes;
    }
    with_engine_stack(move || run_case(&case))
        .actual
        .expect("output")
}

#[test]
fn irreducible_loops_holding_a_try_decompile() {
    for (id, methods) in [(JASM, 2), (KOTLIN, 3)] {
        let fixed = decompile("kuna-fixes", id, None);
        assert_eq!(count_failed_methods(&fixed), 0, "{fixed}");
        assert_eq!(
            fixed.matches("Irreducible bytecode was duplicated").count(),
            methods,
            "{fixed}"
        );

        let compat = decompile("kuna-fixes-compat", id, None);
        assert_eq!(count_failed_methods(&compat), methods, "{compat}");
    }
}

#[test]
fn without_the_fix_the_loops_fail() {
    let without = KunaFixes::DEFAULT.without(KunaFix::IrreducibleWithExceptionEdges);
    for (id, methods) in [(JASM, 2), (KOTLIN, 3)] {
        let failed = decompile("kuna-fixes", id, Some(without));
        assert_eq!(count_failed_methods(&failed), methods, "{failed}");
    }
}

#[test]
fn every_exit_of_the_reproducer_is_kept() {
    let fixed = decompile("kuna-fixes", JASM, None);
    for exit in [
        "return var2;",
        "return 10;",
        "return 20;",
        "return 30;",
        "return -1;",
    ] {
        assert!(fixed.contains(exit), "{exit}: {fixed}");
    }
}
