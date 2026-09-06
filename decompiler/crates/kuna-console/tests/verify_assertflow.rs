//! The `--assert flow` directive end-to-end — `docs/re-needs/no-cli-structuring-override.md`.
//!
//! `override flow <addr> branch|call|callreturn|return` is the one structuring
//! override whose engine path is fully ported, and it was reachable only from
//! `decomp_dbg`: the `kuna` binary's `--assert` plane carried eleven directives
//! and `flow` was not among them, so an agent driving the CLI could not correct
//! a misclassified call, branch or return at all.
//!
//! Every case here asserts the **emitted C changed**, against a measured
//! baseline — the bar this family is held to, because `override prototype` has
//! printed "Successfully added override" and changed nothing since it was
//! ported.
//!
//! Fixture: `kuna-analysis/tests/fixtures/aif_gap_x86_64`. `sub_13c9` reaches an
//! indirect `call *%rdx` at `0x1405` and then twenty-four more calls; declaring
//! that instruction a `return` cuts the body to its first statement.
//!
//! ## `.sla` precondition
//!
//! Bootstrapping needs the built x86 `.sla` under `specs/` (gitignored; `make
//! specs`). When it is absent the bootstrap fails; the test prints that and
//! returns early (a specs-less CI is a visible skip, never a false green).

use std::path::PathBuf;

use kuna_console::assertions::{self, Body, Directive, Outcome};
use kuna_console::engine::{bootstrap_from_object, ConsoleProgram, EntrySelector};
use kuna_console::project::decompile_targets;

/// The function whose flow the directives below reclassify.
const SUB_13C9: u64 = 0x13c9;
/// The indirect `call *%rdx` inside it.
const INDIRECT_CALL: u64 = 0x1405;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

/// Bootstrap the fixture and run the analysis commit.  `None` ⇒ specs-less skip.
fn load() -> Option<ConsoleProgram> {
    let root = repo_root();
    let spec_roots = vec![root.join("specs").to_str().unwrap().to_string()];
    let bin = root.join("decompiler/crates/kuna-analysis/tests/fixtures/aif_gap_x86_64");
    let mut prog = match bootstrap_from_object(bin.to_str()?, "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_assertflow: skipping (bootstrap failed, build `.sla` with \
                 `make specs`): {}",
                e.explain()
            );
            return None;
        }
    };
    prog.commit_pending_analysis().expect("analysis commit");
    Some(prog)
}

fn flow(spec: &str, func: Option<&str>, addr: u64, kind: &str) -> Directive {
    Directive {
        raw: spec.to_string(),
        body: Body::Flow {
            func: func.map(str::to_string),
            addr,
            kind: kind.to_string(),
        },
    }
}

/// Decompile the entries at `at` under `directives`, returning `(C of the first,
/// its pipeline error, report)`.  Mirrors what the CLI's in-process `--json`
/// surface does.
fn run(at: &[u64], directives: Vec<Directive>) -> Option<(String, Option<String>, Vec<Outcome>)> {
    let mut prog = load()?;
    if !directives.is_empty() {
        prog.set_assertions(directives);
        assertions::apply_program_scoped(&mut prog);
    }
    let targets = at
        .iter()
        .map(|vma| {
            prog.resolve_entry(&EntrySelector::Numeric(*vma))
                .expect("the fixture has a function at this address")
        })
        .collect();
    let funcs = decompile_targets(&mut prog, targets, false, false, false);
    let code = funcs[0].code.clone().unwrap_or_default();
    Some((code, funcs[0].error.clone(), prog.assertion_outcomes()))
}

/// The common case: a run that is expected to decompile cleanly.
fn decompile_with(at: &[u64], directives: Vec<Directive>) -> Option<(String, Vec<Outcome>)> {
    let (code, error, report) = run(at, directives)?;
    assert_eq!(error, None, "the pipeline aborted:\n{code}");
    Some((code, report))
}

/// Every outcome is `applied`; panics naming the offender otherwise.
fn all_applied(report: &[Outcome]) {
    for outcome in report {
        assert_eq!(
            outcome.status, "applied",
            "{:?} was rejected: {:?}",
            outcome.directive, outcome.detail
        );
    }
}

/// The un-asserted baseline every case below is measured against: flow follows
/// the indirect call and the twenty-four that follow it, so the body carries
/// `v25` and ends in a twenty-five-term sum.
#[test]
fn the_baseline_follows_the_indirect_call_into_twenty_five_temporaries() {
    let Some((code, report)) = decompile_with(&[SUB_13C9], Vec::new()) else { return };
    assert!(report.is_empty(), "no directives ⇒ no report rows");
    assert!(code.contains("v25"), "baseline no longer reaches v25:\n{code}");
    assert!(
        !code.contains("return dat_4014;"),
        "baseline already stops at the indirect call:\n{code}"
    );
}

/// The need's own case: `flow <addr> return` collapses the body.
#[test]
fn declaring_the_indirect_call_a_return_cuts_the_body_to_one_statement() {
    let Some((code, report)) =
        decompile_with(&[SUB_13C9], vec![flow("flow 0x1405 return", None, INDIRECT_CALL, "return")])
    else {
        return;
    };
    all_applied(&report);
    assert_eq!(report.len(), 1);
    assert_eq!(report[0].kind, "flow");
    assert_eq!((report[0].phase, report[0].subphase), ("P2", "flow-classification"));
    assert!(code.contains("return dat_4014;"), "the override did not take:\n{code}");
    assert!(!code.contains("v25"), "the body still runs past the override:\n{code}");
}

/// The other three spellings are the console's own vocabulary
/// (`Override::string_to_type`), and each lands as a DIFFERENT flow type — the
/// point being that the directive carries the type through, not merely that it
/// is accepted.  Measured on this fixture: `branch` re-reads the indirect call
/// as a computed jump and recovers its two-case table; `callreturn` prunes the
/// fall-through, leaving the call and nothing after it.
#[test]
fn branch_and_callreturn_land_as_their_own_flow_types() {
    let Some((branch, _)) =
        decompile_with(&[SUB_13C9], vec![flow("flow 0x1405 branch", None, INDIRECT_CALL, "branch")])
    else {
        return;
    };
    assert!(branch.contains("switch"), "branch did not become a computed jump:\n{branch}");

    let Some((callret, _)) = decompile_with(
        &[SUB_13C9],
        vec![flow("flow 0x1405 callreturn", None, INDIRECT_CALL, "callreturn")],
    ) else {
        return;
    };
    assert!(!callret.contains("v25"), "callreturn did not prune the fall-through:\n{callret}");
    assert!(callret.contains("("), "callreturn dropped the call itself:\n{callret}");
    assert_ne!(branch, callret, "two flow types produced the same C");
}

/// `call` at this site is the one the ENGINE refuses — an indirect call has no
/// destination to make direct, so `Funcdata::overrideFlow` raises "Could not
/// apply flowoverride".
///
/// The refusal REJECTS the directive; it does not delete the function.  Nothing
/// is mutated before `overrideFlow` gives up, so the IR that follows is the one
/// the same run without the directive would have produced — and discarding it
/// used to cost the caller a 55-line body plus, on the text CLI, any sign that
/// anything had gone wrong at all (`docs/re-needs/rejected-flow-override-exits.md`).
#[test]
fn a_flow_type_the_engine_cannot_apply_is_rejected_and_keeps_the_body() {
    let Some((code, error, report)) =
        run(&[SUB_13C9], vec![flow("flow 0x1405 call", None, INDIRECT_CALL, "call")])
    else {
        return;
    };
    assert_eq!(error, None, "the refusal aborted the function:\n{code}");
    assert_eq!(report.len(), 1);
    assert_eq!(report[0].status, "rejected");
    assert!(report[0].fatal, "a pipeline refusal is fatal: {report:?}");
    assert!(
        report[0].detail.as_deref().unwrap_or_default().contains("flowoverride"),
        "expected the engine's own refusal, got {:?}",
        report[0].detail
    );
    // The body the abort used to throw away.
    assert!(code.contains("v25"), "the recovered body did not come back:\n{code}");
}

/// A directive the pipeline refused is `fatal`; one that merely did not bind is
/// not.  The distinction is what `kuna decompile` reads to decide whether the run
/// failed without `--assert-strict`: a refused `flow` means the C describes a
/// control-flow graph other than the one asked for, and nothing in the C says so,
/// while an unbound directive left a correct body un-annotated.
#[test]
fn only_a_pipeline_refusal_is_fatal() {
    let Some((_code, report)) =
        decompile_with(&[SUB_13C9], vec![flow("flow 0x1405 goto", None, INDIRECT_CALL, "goto")])
    else {
        return;
    };
    assert_eq!(report[0].status, "rejected");
    assert!(!report[0].fatal, "a directive rejected before the pipeline is not fatal");
}

/// A spelling outside the four-word vocabulary is REJECTED with a reason, not
/// accepted and dropped.  (The CLI parser rejects it earlier still, at `--assert`
/// parse time; this is the engine-side guard for every other caller.)
#[test]
fn an_unknown_flow_kind_is_rejected_with_a_reason() {
    let Some((_code, report)) =
        decompile_with(&[SUB_13C9], vec![flow("flow 0x1405 goto", None, INDIRECT_CALL, "goto")])
    else {
        return;
    };
    assert_eq!(report.len(), 1);
    assert_eq!(report[0].status, "rejected");
    assert!(
        report[0].detail.as_deref().unwrap_or_default().contains("Bad override type"),
        "detail: {:?}",
        report[0].detail
    );
}

/// An unqualified directive binds to "the function under decompile", which is
/// only unambiguous when the run selected one.  On a multi-target run it is
/// rejected rather than applied to whichever function happens to span the
/// address — the plane's standing rule.
#[test]
fn an_unqualified_directive_is_rejected_on_a_multi_target_run() {
    let Some((_code, report)) = decompile_with(
        &[SUB_13C9, 0x1129],
        vec![flow("flow 0x1405 return", None, INDIRECT_CALL, "return")],
    ) else {
        return;
    };
    assert_eq!(report.len(), 1);
    assert_eq!(report[0].status, "rejected");
    assert!(
        report[0].detail.as_deref().unwrap_or_default().contains("<func>::<operand>"),
        "detail: {:?}",
        report[0].detail
    );
}

/// ...and the qualified spelling is how a whole-binary run states the same fact.
#[test]
fn a_qualified_directive_binds_on_a_multi_target_run() {
    let Some((code, report)) = decompile_with(
        &[SUB_13C9, 0x1129],
        vec![flow(
            "flow sub_13c9::0x1405 return",
            Some("sub_13c9"),
            INDIRECT_CALL,
            "return",
        )],
    ) else {
        return;
    };
    all_applied(&report);
    assert!(code.contains("return dat_4014;"), "the override did not take:\n{code}");
}
