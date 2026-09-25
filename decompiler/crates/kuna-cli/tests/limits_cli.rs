//! CLI end-to-end gate for the `limits` report of `kuna functions --json` and
//! `kuna functions --summary`: which functions a decompile would hit the
//! `maxinstruction` / `jumptablemax` budgets on, measured without decompiling.
//!
//! `pe_switchdelta_x86_64.exe` is the fixture because its one switch is the MSVC
//! x64 image-base-relative delta table behind a four-case range check, the same
//! shape as the giant state-machine dispatches the report exists for, so a
//! `jumptablemax` of 2 puts it over the cap.
//!
//! Needs the built `x86` `.sla` under `specs/`; without it each test returns early.

use std::path::PathBuf;
use std::process::Command;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn fixture() -> String {
    repo_root()
        .join("decompiler/crates/kuna-analysis/tests/fixtures/pe_switchdelta_x86_64.exe")
        .to_str()
        .unwrap()
        .to_string()
}

fn run_kuna(args: &[&str]) -> Option<String> {
    let out = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args(args)
        .env("SLEIGHHOME", repo_root().join("specs"))
        .output()
        .expect("failed to spawn the kuna binary");
    let stderr = String::from_utf8_lossy(&out.stderr);
    let code = out.status.code().unwrap_or(-1);
    if code != 0 && (stderr.contains("could not build an architecture") || stderr.contains(".sla")) {
        eprintln!("skip: no built x86 .sla");
        return None;
    }
    assert_eq!(code, 0, "kuna {args:?} failed: {stderr}");
    Some(String::from_utf8_lossy(&out.stdout).chars().filter(|c| !c.is_whitespace()).collect())
}

const OVER_CAP: &str = r#""switches_over_jumptablemax":[{"address":5368713309,"address_hex":"0x14000105d","cases":4,"read":2}]"#;

#[test]
fn the_summary_flags_a_switch_over_jumptablemax_and_a_body_over_maxinstruction() {
    let bin = fixture();
    let args = [
        "functions", &bin, "--summary", "--json", "--option", "jumptablemax", "2", "--option",
        "maxinstruction", "10",
    ];
    let Some(out) = run_kuna(&args) else { return };
    assert!(out.contains(r#""limits":{"maxinstruction":10,"jumptablemax":2,"over":[{"name":"sub_140001040""#), "{out}");
    assert!(out.contains(r#""instructions":16,"over_maxinstruction":true"#), "{out}");
    assert!(out.contains(OVER_CAP), "{out}");
}

#[test]
fn nothing_is_flagged_under_the_default_budgets() {
    let bin = fixture();
    let Some(out) = run_kuna(&["functions", &bin, "--summary", "--json"]) else { return };
    assert!(out.contains(r#""limits":{"maxinstruction":100000,"jumptablemax":1024,"over":[]}"#), "{out}");
}

#[test]
fn the_inventory_json_carries_the_same_limits() {
    let bin = fixture();
    let Some(out) = run_kuna(&["functions", &bin, "--json", "--option", "jumptablemax", "2"])
    else {
        return;
    };
    assert!(out.contains(r#""limits":{"maxinstruction":100000,"jumptablemax":2,"over":[{"name":"sub_140001040""#), "{out}");
    assert!(out.contains(r#""over_maxinstruction":false"#), "{out}");
    assert!(out.contains(OVER_CAP), "{out}");
}

#[test]
fn a_ceiling_at_the_case_count_reads_the_whole_table() {
    let bin = fixture();
    let Some(out) = run_kuna(&["functions", &bin, "--json", "--option", "jumptablemax", "4"])
    else {
        return;
    };
    assert!(out.contains(r#""jumptablemax":4,"over":[]"#), "{out}");
}

#[test]
fn jumptablemax_is_catalogued_and_refuses_a_non_integer() {
    let out = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args(["catalog", "--json"])
        .output()
        .expect("failed to spawn the kuna binary");
    let catalog = String::from_utf8_lossy(&out.stdout);
    assert!(catalog.contains(r#""option": "jumptablemax""#), "{catalog}");

    let bin = fixture();
    let out = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args(["functions", &bin, "--json", "--option", "jumptablemax", "wide"])
        .env("SLEIGHHOME", repo_root().join("specs"))
        .output()
        .expect("failed to spawn the kuna binary");
    let stderr = String::from_utf8_lossy(&out.stderr);
    if stderr.contains("could not build an architecture") {
        return;
    }
    assert_ne!(out.status.code(), Some(0), "a non-integer ceiling must be refused");
    assert!(stderr.contains("Must specify integer maximum"), "{stderr}");
}
