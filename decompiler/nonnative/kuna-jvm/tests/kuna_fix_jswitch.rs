//! The switch layout fixes in isolation: each case of `tests/jvm/kuna-fixes/`
//! for them prints Vineflower's text when every fix but the ones it
//! demonstrates is on.

use std::path::Path;

use kuna_jvm::kuna_corpus::{load_cases, run_case, with_engine_stack, CaseStatus};
use kuna_jvm::kuna_fixes::{KunaFix, KunaFixes};

const CASES: &[(&str, &[KunaFix])] = &[
    ("pkg/TestSwitchCaseOrder", &[KunaFix::SwitchCaseOrder]),
    (
        "pkg/TestSwitchTableFillers",
        &[KunaFix::SwitchTableFillers, KunaFix::SwitchCaseOrder],
    ),
    (
        "pkg/TestSwitchSyntheticDefault",
        &[KunaFix::SwitchSyntheticDefault],
    ),
];

#[test]
fn each_jswitch_case_is_its_own_fixs_change() {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../tests/jvm/cases.tsv");
    let all = load_cases(&manifest).unwrap();
    let runs: Vec<_> = CASES
        .iter()
        .map(|(id, fixes)| {
            let compat_id = format!("kuna-fixes-compat/{id}");
            let mut case = all
                .iter()
                .find(|c| c.id == compat_id)
                .unwrap_or_else(|| panic!("{compat_id} is not in cases.tsv"))
                .clone();
            case.fixes = Some(
                fixes
                    .iter()
                    .fold(KunaFixes::DEFAULT, |set, fix| set.without(*fix)),
            );
            (compat_id, case)
        })
        .collect();
    let results = with_engine_stack(move || {
        runs.iter()
            .map(|(id, case)| (id.clone(), run_case(case)))
            .collect::<Vec<_>>()
    });
    for (id, result) in &results {
        assert_eq!(
            result.status,
            CaseStatus::Match,
            "{id} without its fixes: {:?}",
            result.first_diff
        );
    }
}
