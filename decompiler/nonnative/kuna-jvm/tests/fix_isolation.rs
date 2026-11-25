//! Each kuna fix and form option alone, over the cases of `tests/jvm/kuna-fixes/cases.json` (case
//! -> fix or option): with only its own fix off (and the form options off) a Java case prints
//! Vineflower's text, the one its compat run's `vineflower` digest pins; with only its own switch
//! off a Kotlin case or a form-option case changes. The two texts themselves are pinned by
//! `kuna_fixes::every_kuna_fixes_case_matches_in_both_modes`.

use std::path::{Path, PathBuf};

use kuna_jvm::kuna_corpus::{load_cases, run_case, with_engine_stack, Case, CaseStatus};
use kuna_jvm::kuna_fixes::{KunaFix, KunaFixes};
use kuna_jvm::kuna_options::{rows, KUNA_FORM};

/// Java cases another fix also changes: it goes off with the case's own.
const PARTNERS: &[(&str, KunaFix)] = &[
    (
        "pkg/TestArgumentGenericCast",
        KunaFix::StaticTypeVariableScope,
    ),
    (
        "pkg/TestBoundedInferenceCast",
        KunaFix::StaticTypeVariableScope,
    ),
    ("pkg/TestCompoundAssignmentShape", KunaFix::FieldReadReuse),
    ("pkg/TestFinallyExitVersions", KunaFix::VarMergeNarrowing),
    ("pkg/TestGenericSlotReuse", KunaFix::UncheckedGenericCast),
    ("pkg/TestSplitConstructorCall", KunaFix::FieldReadReuse),
    ("pkg/TestSuspendLoopFinally", KunaFix::HandlerCopyOffsets),
    ("pkg/TestSwitchFillerFloor", KunaFix::SwitchCaseOrder),
    ("pkg/TestSwitchTableFillers", KunaFix::SwitchCaseOrder),
    (
        "pkg/TestTwrLibraryResource",
        KunaFix::CompoundAssignmentShape,
    ),
    ("pkg/TestVarMergeLvtWidening", KunaFix::VarMergeDeclaredType),
];

/// Kotlin cases whose own fix another one masks (all of it, or, for the inlined source copy, the
/// creating method's reads): that one goes off first.
const MASKED_BY: &[(&str, KunaFix)] = &[
    ("pkg/TestCapturedReceiverKt", KunaFix::KtClassHeaderMetadata),
    ("pkg/TestCastCaptureKt", KunaFix::KtSuspendStateMachine),
    (
        "pkg/TestFailedMethodHidden",
        KunaFix::KtSyntheticObjectSuperCall,
    ),
    ("pkg/TestForRangeUnmatchedBound", KunaFix::KtForRangeCounter),
    (
        "pkg/TestInlinedObjectCaptureKt$dropFirst$$inlined$source$1",
        KunaFix::UnlistedAnonymousClass,
    ),
    (
        "pkg/TestKtFinallySemaphore",
        KunaFix::InlineMarkerFinallyCopy,
    ),
    ("pkg/TestKtInlineLockLocal", KunaFix::KtPartialInline),
    (
        "pkg/TestMembersWithoutMetadataKt$counting$$inlined$feed$1$1",
        KunaFix::KtSuspendStateMachine,
    ),
    (
        "pkg/TestMethodObjectReturnType",
        KunaFix::KtGeneratedOverloadHidden,
    ),
    (
        "pkg/TestSuspendConditionalFinally",
        KunaFix::KtSuspendStateMachine,
    ),
    (
        "pkg/TestSuspendContinuationKt",
        KunaFix::KtSuspendStateMachine,
    ),
    ("pkg/TestUnboxingCastType", KunaFix::KtMappedJavaType),
];

fn corpus() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../tests/jvm")
}

/// What a case is for: a fix, or a form option (`None`).
type Owner = Option<KunaFix>;

/// Every case of `cases.json` with the switch it is for, and the switch's name.
fn owned_cases() -> Vec<(String, Owner, String)> {
    let json = std::fs::read_to_string(corpus().join("kuna-fixes/cases.json")).unwrap();
    let field = |line: &str, key: &str| {
        let rest = &line[line.find(&format!("\"{key}\": \""))? + key.len() + 5..];
        rest.find('"').map(|end| rest[..end].to_string())
    };
    let mut owned = Vec::new();
    for line in json.lines() {
        if let (Some(id), Some(fix)) = (field(line, "id"), field(line, "fix")) {
            owned.push((id, KunaFix::from_id(&fix), fix));
        }
    }
    assert!(owned.len() > 280, "{} cases", owned.len());
    owned
}

fn other(table: &[(&str, KunaFix)], id: &str) -> Option<KunaFix> {
    table
        .iter()
        .find(|(case, _)| *case == id)
        .map(|(_, fix)| *fix)
}

fn case(all: &[Case], id: &str) -> Case {
    all.iter().find(|c| c.id == id).unwrap().clone()
}

fn is_kotlin(case: &Case) -> bool {
    case.compare[0].ends_with(".kt")
}

#[test]
fn a_java_case_without_its_fix_prints_vineflowers_text() {
    let all = load_cases(&corpus().join("cases.tsv")).unwrap();
    let mut runs = Vec::new();
    for (id, owner, _) in owned_cases() {
        let mut compat = case(&all, &format!("kuna-fixes-compat/{id}"));
        let Some(fix) = owner.filter(|_| !is_kotlin(&compat)) else {
            continue;
        };
        let off = KunaFixes::DEFAULT.without(fix);
        compat.fixes = Some(other(PARTNERS, &id).map_or(off, |partner| off.without(partner)));
        for row in rows().iter().filter(|r| r.change_kind == KUNA_FORM) {
            compat
                .options
                .push((row.option.to_string(), "0".to_string()));
        }
        runs.push(compat);
    }
    assert!(runs.len() > 100);
    let failures: Vec<String> = with_engine_stack(move || {
        runs.iter()
            .map(run_case)
            .filter(|result| result.status != CaseStatus::Match)
            .map(|result| format!("{}: {:?}", result.id, result.first_diff))
            .collect()
    });
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[test]
fn a_kotlin_or_form_option_case_changes_without_its_switch() {
    let all = load_cases(&corpus().join("cases.tsv")).unwrap();
    let mut runs = Vec::new();
    for (id, owner, switch) in owned_cases() {
        let mut default = case(&all, &format!("kuna-fixes/{id}"));
        if is_kotlin(&default) || owner.is_none() {
            let base =
                other(MASKED_BY, &id).map_or(KunaFixes::DEFAULT, |m| KunaFixes::DEFAULT.without(m));
            default.fixes = Some(base);
            let mut off = default.clone();
            match owner {
                Some(fix) => off.fixes = Some(base.without(fix)),
                None => off.options.push((switch, "0".to_string())),
            }
            runs.push((default, off));
        }
    }
    assert!(runs.len() > 150);
    let unchanged: Vec<String> = with_engine_stack(move || {
        runs.iter()
            .filter(|(on, off)| run_case(on).actual == run_case(off).actual)
            .map(|(on, _)| on.id.clone())
            .collect()
    });
    assert!(unchanged.is_empty(), "{unchanged:?}");
}
