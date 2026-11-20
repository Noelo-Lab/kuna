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

/// A return kept in a temporary is compared at the register the model
/// returns its type in.
#[test]
fn return_in_a_temporary_is_compared_at_the_model_register() {
    let v = verdicts("kuna-gobool");
    assert_eq!(of(&v, &["is_one"]), ["AGREES(256)"; 2]);
}

/// A switch row goes to the out-edge its table entry reaches, never silently to
/// the default.
#[test]
fn switch_rows_follow_their_table_entries() {
    let v = verdicts("regionstructure-switch");
    assert_eq!(of(&v, &["dispatch"]), ["AGREES(256)"; 2]);
}

/// A `bool` passed on the stack is 0 or 1 in its low byte, as one in a register.
#[test]
fn stack_bool_parameters_are_zero_or_one() {
    let v = verdicts("kuna-stackobjects");
    let checked = of(&v, &["param_reuse", "param_pair", "param_scalar_escaped"]);
    assert!(!checked.is_empty() && checked.iter().all(|l| *l == "AGREES(256)"), "{checked:?}");
}

/// A return kuna folded to a constant is compared at the register the model
/// returns it in.
#[test]
fn constant_returns_are_compared_at_the_model_register() {
    let v = verdicts("kuna-bejoin-sparc");
    let checked = of(&v, &["same", "one_after", "zero_after", "keep_zero"]);
    assert!(!checked.is_empty() && checked.iter().all(|l| *l == "AGREES(256)"), "{checked:?}");
}

/// A call whose prototype has no output keeps its value in a temporary that a
/// copy moves to a register: the value is the register's after the call.
#[test]
fn a_call_value_cast_into_a_register_is_the_registers() {
    let v = verdicts("ghdec-realtypes-pointee");
    let checked = of(&v, &["acl_calc_mask"]);
    assert!(!checked.is_empty() && checked.iter().all(|l| *l == "AGREES(256)"), "{checked:?}");
}

/// Reading past an array into a callee-saved register's spill is undefined in
/// C: kuna never stores the spill, so its side reads uninitialized stack, and
/// the run is not a difference.
#[test]
fn reading_a_register_spill_past_an_array_is_undefined() {
    let v = verdicts("kuna-arrayextent");
    let checked = of(&v, &["q_clang", "w_clang"]);
    assert!(!checked.is_empty() && checked.iter().all(|l| l.starts_with("UNKNOWN")), "{checked:?}");
}
