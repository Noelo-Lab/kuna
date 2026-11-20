//! Verdicts on stage tests that pin harness behavior. Each of these was once a
//! false `DIFFERS` or a wrong `AGREES`, so a change that brings one back fails
//! here instead of reading as a kuna bug.

use std::path::PathBuf;

use kuna_verify::{check_datatest, Options};

fn repo() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

/// `(function, verdict label)` for every function the test's script decompiles.
fn verdicts(test: &str) -> Vec<(String, String)> {
    let specs = repo().join("specs").to_string_lossy().into_owned();
    let path = repo().join("tests/stages").join(format!("{test}.xml"));
    let res = check_datatest(&path.to_string_lossy(), &[specs], &Options::default());
    assert_eq!(res.error, None, "{test}");
    res.reports.iter().map(|r| (r.function.clone(), r.verdict.label())).collect()
}

fn of<'a>(v: &'a [(String, String)], names: &[&str]) -> Vec<&'a str> {
    v.iter().filter(|(f, _)| names.contains(&f.as_str())).map(|(_, l)| l.as_str()).collect()
}

/// The first pass sets `narrowext off`, the bug the test pins; the second is the
/// fix. Inputs keep the documented RV64 extension in both.
#[test]
fn narrowext_bug_pass_differs_and_fix_pass_agrees() {
    let v = verdicts("kuna-narrowext");
    let all = "AGREES(256)";
    assert_eq!(
        of(&v, &["pi", "pus", "pu", "psc"]),
        ["DIFFERS", all, "DIFFERS", "DIFFERS", all, all, all, all]
    );
}

/// A 32-bit `int` argument in a 64-bit RISC-V register arrives sign-extended.
#[test]
fn riscv_arguments_keep_their_abi_extension() {
    let v = verdicts("kuna-constptrdeclared-riscv");
    assert_eq!(of(&v, &["rq", "ru", "rl"]), ["AGREES(256)"; 3]);
}

/// A stack slot kuna writes as a varnode is the memory a later LOAD reads.
#[test]
fn stack_varnode_writes_reach_loads() {
    let v = verdicts("kuna-arrayextent");
    let agree = of(&v, &["h_clang", "h_gcc", "m_clang", "m_gcc"]);
    assert!(!agree.is_empty() && agree.iter().all(|l| *l == "AGREES(256)"), "{agree:?}");
}

/// Cut run-on rows are a declared divergence, and the placeholder label of a
/// table's default row never matches a selector.
#[test]
fn run_on_switch_rows_are_declared() {
    let v = verdicts("kuna-switchrunonguard");
    assert_eq!(of(&v, &["pick", "pick2"]), ["DIVERGES(switchrunon-cut-row)"; 2]);
}

/// A return kept in a temporary is not machine state, so it is not compared.
#[test]
fn return_in_a_temporary_is_unknown() {
    let v = verdicts("kuna-gobool");
    assert!(v.iter().any(|(_, l)| l.starts_with("UNKNOWN(return storage unique:")), "{v:?}");
}

/// A switch row goes to the out-edge its table entry reaches, never silently to
/// the default.
#[test]
fn switch_rows_follow_their_table_entries() {
    let v = verdicts("regionstructure-switch");
    assert_eq!(of(&v, &["dispatch"]), ["AGREES(256)"; 2]);
}
