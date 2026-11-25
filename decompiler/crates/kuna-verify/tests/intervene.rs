//! CSIC: interventions on a run that differs (`intervene`).

use std::path::PathBuf;

use kuna_verify::tools::intervene::{search_source, Do};
use kuna_verify::localize::Source;
use kuna_verify::mutate::Operator;
use kuna_verify::Options;

fn repo() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn stage(test: &str) -> (String, Vec<String>) {
    let specs = repo().join("specs").to_string_lossy().into_owned();
    (repo().join("tests/stages").join(format!("{test}.xml")).to_string_lossy().into_owned(), vec![specs])
}

/// `narrowext off` zero-extends a signed `int` argument: the one witness that
/// holds on every run is the extension turned into a sign extension.
#[test]
fn narrowext_witness_is_the_extension() {
    let (path, roots) = stage("kuna-narrowext");
    let (decompile, c) = search_source(&Source::Datatest(&path), &roots, "pu", None, &Options::default()).unwrap();
    assert_eq!(decompile, 0);
    assert!(c.run.is_some());
    assert_eq!(c.causal_status(), "WITNESS_FOUND");
    let w = c.witnesses.iter().find(|w| matches!(w.change, Do::Opcode { .. })).expect("an opcode witness");
    assert_eq!(w.op, "INT_ZEXT");
    assert_eq!(w.what, "do(INT_ZEXT -> INT_SEXT)");
    assert_eq!(w.all_runs.as_deref(), Some("AGREES(256)"));
    let j = c.json();
    assert_eq!(j["minimality"], "NOT_PROVEN");
    assert_eq!(j["witnesses"][0]["counterfactual_replay"], "AGREES");
}

/// A decompile that agrees has nothing to search.
#[test]
fn agreeing_decompile_has_no_search() {
    let (path, roots) = stage("kuna-narrowext");
    let (_, c) = search_source(&Source::Datatest(&path), &roots, "pu", Some(2), &Options::default()).unwrap();
    assert_eq!(c.run, None);
    assert!(c.witnesses.is_empty());
    assert_eq!(c.causal_status(), "NONE");
}

/// A caught opcode mutant: some witness is on the op it changed.
#[test]
fn opcode_mutants_are_found_where_planted() {
    let (path, roots) = stage("kuna-narrowext");
    let opts = Options { intervene: true, runs: 64, ..Options::default() };
    let mut caught = Vec::new();
    kuna_verify::mutate_datatest(&path, &roots, &opts, 6, |o| {
        if o.operator == Operator::Opcode {
            if let Some(h) = o.csic {
                caught.push((o.what, h.any));
            }
        }
    })
    .unwrap();
    assert!(!caught.is_empty());
    assert!(caught.iter().all(|(_, any)| *any), "{caught:?}");
}
