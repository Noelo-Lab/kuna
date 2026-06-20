//! `kuna-lift-diff` — the self-test entry point for the W5 lift differential.
//!
//! This binary is the package named in the item gate
//! `cargo run -p kuna-lift-diff -- --self-test` (and the W5 wave gate's second
//! clause).  It is a thin shim: it delegates to
//! [`kuna_harness::corpus::run_self_test`], which runs the B2-grade STRUCTURAL
//! differential (parse the committed C++ `break start heritage` fixture, bootstrap
//! the Rust `XmlArchitecture` frontend, run `FlowInfo` follow-flow + the basic-block
//! builder to the same boundary, and compare the structural model) over the pinned
//! corpus.  The SAME `run_self_test()` backs `cargo test -p kuna-harness --test
//! lift_diff`, so the bin and the integration test exercise identical code.
//!
//! Exit codes:
//!   0  — the gate held (>= 8/12 corpus tests structurally equal end-to-end, every
//!        documented divergence/exclusion attributed; or all cases skipped because
//!        no `.sla` specs are built — a bare checkout, treated as a structural pass).
//!   1  — the gate failed (a mismatch vs the documented expectations, or fewer than
//!        8 PASS), OR an unknown flag was passed.
//!   2  — a harness error (a missing fixture, a bootstrap failure that is not a
//!        spec-missing skip).

use std::process::ExitCode;

use kuna_harness::corpus;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("--self-test") => self_test(),
        Some(other) => {
            eprintln!("kuna-lift-diff: unknown argument {other:?}; expected --self-test");
            ExitCode::from(1)
        }
        None => {
            eprintln!("kuna-lift-diff: the B2 lift differential self-test driver");
            eprintln!("usage: kuna-lift-diff --self-test");
            ExitCode::from(1)
        }
    }
}

fn self_test() -> ExitCode {
    let report = match corpus::run_self_test() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("kuna-lift-diff: harness error: {e}");
            return ExitCode::from(2);
        }
    };
    print!("{}", report.render());

    if report.all_skipped() {
        println!("all cases skipped (no specs built) — structural pass");
        return ExitCode::SUCCESS;
    }
    if report.passed() {
        println!(
            "lift-diff self-test PASS: {}/{} corpus tests structurally equal end-to-end",
            report.npass,
            corpus::CASES.len()
        );
        ExitCode::SUCCESS
    } else {
        eprintln!(
            "lift-diff self-test FAIL: {}/{} PASS (gate requires >= 8); see the table above",
            report.npass,
            corpus::CASES.len()
        );
        ExitCode::from(1)
    }
}
