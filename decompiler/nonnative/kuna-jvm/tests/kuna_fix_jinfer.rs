//! The generic-type fixes of wrong output in isolation: each case of
//! `tests/jvm/kuna-fixes/` for them prints Vineflower's text when every
//! fix but its own is on (and the option of form `jvm_source_locals` off), so
//! that fix alone makes the case's change.

use std::path::Path;

use kuna_jvm::kuna_corpus::{load_cases, run_case, with_engine_stack, CaseStatus};
use kuna_jvm::kuna_fixes::{KunaFix, KunaFixes};

const CASES: &[(&str, KunaFix)] = &[
    (
        "pkg/TestUncheckedTypePattern",
        KunaFix::UncheckedTypePattern,
    ),
    (
        "pkg/TestInnerMethodArgumentShift",
        KunaFix::InnerMethodArgumentShift,
    ),
    ("pkg/TestRawReceiverErasure", KunaFix::RawReceiverErasure),
    (
        "pkg/TestBoxedTypeVariableCast",
        KunaFix::BoxedTypeVariableCast,
    ),
    (
        "pkg/TestLambdaArgumentOverload",
        KunaFix::LambdaArgumentOverload,
    ),
    (
        "pkg/TestStaticTypeVariableScope",
        KunaFix::StaticTypeVariableScope,
    ),
    (
        "pkg/TestUncheckedGenericCast",
        KunaFix::UncheckedGenericCast,
    ),
];

#[test]
fn each_generic_type_case_is_its_own_fixs_change() {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../tests/jvm/cases.tsv");
    let all = load_cases(&manifest).unwrap();
    let runs: Vec<_> = CASES
        .iter()
        .map(|(id, fix)| {
            let compat_id = format!("kuna-fixes-compat/{id}");
            let mut case = all
                .iter()
                .find(|c| c.id == compat_id)
                .unwrap_or_else(|| panic!("{compat_id} is not in cases.tsv"))
                .clone();
            case.fixes = Some(KunaFixes::DEFAULT.without(*fix));
            case.options
                .push(("jvm_source_locals".to_string(), "0".to_string()));
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
            "{id} without its fix: {:?}",
            result.first_diff
        );
    }
}
