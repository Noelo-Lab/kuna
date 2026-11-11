//! The jgeneric2 fixes in isolation: each case of `tests/jvm/kuna-fixes/`
//! for them prints its reviewed default text with every fix on, and no
//! longer does when only its own fix is off, so that fix makes the change.

use std::path::Path;

use kuna_jvm::kuna_corpus::{load_cases, run_case, with_engine_stack, CaseStatus};
use kuna_jvm::kuna_fixes::{KunaFix, KunaFixes};

const CASES: &[(&str, KunaFix)] = &[
    (
        "pkg/TestExternalSupertypeArgs",
        KunaFix::ExternalSupertypeArguments,
    ),
    ("pkg/TestBoundedInferenceCast", KunaFix::BoundedInferenceCast),
    (
        "pkg/TestReceiverTypeArguments",
        KunaFix::ReceiverTypeArguments,
    ),
    ("pkg/TestArgumentGenericCast", KunaFix::ArgumentGenericCast),
];

#[test]
fn each_jgeneric2_case_needs_its_own_fix() {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../tests/jvm/cases.tsv");
    let all = load_cases(&manifest).unwrap();
    let runs: Vec<_> = CASES
        .iter()
        .flat_map(|(id, fix)| {
            let default_id = format!("kuna-fixes/{id}");
            let case = all
                .iter()
                .find(|c| c.id == default_id)
                .unwrap_or_else(|| panic!("{default_id} is not in cases.tsv"))
                .clone();
            let mut without = case.clone();
            without.fixes = Some(KunaFixes::DEFAULT.without(*fix));
            [(default_id.clone(), true, case), (default_id, false, without)]
        })
        .collect();
    let results = with_engine_stack(move || {
        runs.iter()
            .map(|(id, with_fix, case)| (id.clone(), *with_fix, run_case(case)))
            .collect::<Vec<_>>()
    });
    for (id, with_fix, result) in &results {
        assert_eq!(
            result.status == CaseStatus::Match,
            *with_fix,
            "{id} (its fix {}): {:?}",
            if *with_fix { "on" } else { "off" },
            result.first_diff
        );
    }
}
