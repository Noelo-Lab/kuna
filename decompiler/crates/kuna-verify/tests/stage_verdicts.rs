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
    assert!(!checked.is_empty() && checked.iter().all(|l| l.starts_with("AGREES(")), "{checked:?}");
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

/// A system call writes the kernel's result register: `on` agrees, `off` (the
/// bug the test pins) differs, and `auto` on an image that names no operating
/// system leaves the register unwritten, which is declared.
#[test]
fn system_call_result_follows_the_kernel() {
    let v = verdicts("kuna-syscallregs-arm");
    assert_eq!(of(&v, &["pair"]), ["AGREES(256)", "DIFFERS", "DIVERGES(syscall-result)"]);
    let v = verdicts("kuna-x64syscall");
    assert_eq!(of(&v, &["kernel_call"]), ["DIFFERS", "AGREES(256)", "AGREES(256)"]);
    assert_eq!(of(&verdicts("kuna-ownreturn"), &["sys2nr"]), ["DIVERGES(syscall-result)"]);
}

/// A removed Go bounds-check panic and a cut arm64e pointer-authentication
/// trap are declared; the checks kuna keeps agree.
#[test]
fn removed_checks_are_declared() {
    let v = verdicts("kuna-gobounds");
    assert_eq!(of(&v, &["pick"]), ["AGREES(256)", "DIVERGES(panic-check)", "AGREES(256)"]);
    let specs = repo().join("specs").to_string_lossy().into_owned();
    let path = repo().join("tests/stages/kuna-ptrauthtrap.xml");
    let res = check_datatest(&path.to_string_lossy(), &[specs], &Options::default());
    let v: Vec<_> = res.reports.iter().map(|r| (r.function.clone(), r.verdict.label())).collect();
    assert_eq!(of(&v, &["x30check", "x30forged"]), ["DIVERGES(ptrauth-trap)", "AGREES(256)"]);
}

/// C3 names the rule application after which `csc` differs with `narrowext`
/// off: the `propagatecopy` at 0x10036 that carries the unextended byte on.
#[test]
fn localize_names_the_copy_rule() {
    use kuna_verify::localize::{localize, Finding, Source};
    let specs = repo().join("specs").to_string_lossy().into_owned();
    let path = repo().join("tests/stages/kuna-narrowext.xml").to_string_lossy().into_owned();
    let found = localize(&Source::Datatest(&path), &[specs], "csc", None, &Options::default()).unwrap();
    let text = found.render();
    let Finding::Rules { culprits, .. } = found.finding else { panic!("{text}") };
    let latest = culprits.last().unwrap();
    assert_eq!((latest.rule.as_str(), latest.addr), ("propagatecopy", Some(0x10036)), "{text}");
}

/// A call kuna knows does not return only by its callee's name (flow plants a
/// halt after it) ends the run as kuna's does, instead of running on past it.
#[test]
fn a_call_flow_halts_after_does_not_return() {
    let v = verdicts("ghangr-noreturn_extern");
    let victim = of(&v, &["victim"]);
    assert!(!victim.is_empty() && victim.iter().all(|l| *l == "AGREES(256)"), "{v:?}");
}

/// x86's segment bases point at a TLS block, not into the image: the
/// stack-protector guard at `fs:0x28` reads the same before and after a call.
#[test]
fn the_stack_guard_survives_a_call() {
    let specs = repo().join("specs").to_string_lossy().into_owned();
    let path = repo().join("decompiler/crates/kuna-analysis/tests/fixtures/katavm_level1_x86_64");
    let reports =
        kuna_verify::check_binary(&path.to_string_lossy(), &["sub_12d0".into()], &[], &[specs], None, &Options::default()).unwrap();
    let labels: Vec<String> = reports.iter().map(|r| r.verdict.label()).collect();
    assert_eq!(labels, ["AGREES(256)"]);
}

/// A Thumb `bx` to Thumb code switches to the mode the target decodes in
/// anyway, and barriers and the exclusive monitor change nothing on one thread.
#[test]
fn thumb_interworking_and_barriers_run() {
    for (test, names) in [("kuna-veneer-tailcall", &["ret_veneer", "plain_veneer"][..]), ("kuna-syscallregs-arm", &["fence"])] {
        let v = verdicts(test);
        let got = of(&v, names);
        assert!(!got.is_empty() && got.iter().all(|l| *l == "AGREES(256)"), "{test}: {v:?}");
    }
}

/// x87: `fild`, `fld1` and a spill to a `float` slot run in the 80-bit format,
/// and a `double` returned in `ST0` is rounded from it before the compare.
#[test]
fn x87_extended_values_run_and_return() {
    let v = verdicts("kuna-integer-float-rounding-x87");
    assert_eq!(of(&v, &["cmpf", "lt1", "l2d2f"]), ["AGREES(256)", "AGREES(256)", "AGREES(256)"]);
}
