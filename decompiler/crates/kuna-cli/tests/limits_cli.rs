//! CLI end-to-end gate for the `limits` report of `kuna functions --summary`
//! (and `functions --reachable-from --json`): which functions a decompile would
//! hit the `maxinstruction` / `jumptablemax` budgets on, measured without
//! decompiling.
//!
//! `pe_switchdelta_x86_64.exe` is the switch fixture because its one switch is the
//! MSVC x64 image-base-relative delta table behind a four-case range check, the
//! same shape as the giant state-machine dispatches the report exists for, so a
//! `jumptablemax` of 2 puts it over the cap. The instruction count is pinned on a
//! generated i386 image shaped like a gcc `.cold` fragment.
//!
//! Needs the built `x86` `.sla` under `specs/`; without it each test returns early.

#[path = "common/arm_images.rs"]
#[allow(dead_code)]
mod arm_images;
mod common;

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

fn kuna(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args(args)
        .env("SLEIGHHOME", repo_root().join("specs"))
        .output()
        .expect("failed to spawn the kuna binary")
}

fn is_specs_skip(stderr: &str) -> bool {
    stderr.contains("could not build an architecture") || stderr.contains(".sla")
}

/// Stdout with every whitespace character removed, or `None` on a specs skip.
fn run_kuna(args: &[&str]) -> Option<String> {
    let out = kuna(args);
    let stderr = String::from_utf8_lossy(&out.stderr);
    let code = out.status.code().unwrap_or(-1);
    if code != 0 && is_specs_skip(&stderr) {
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
    assert!(out.contains(r#""instructions":11,"over_maxinstruction":true"#), "counting stops at maxinstruction + 1: {out}");
    assert!(out.contains(OVER_CAP), "{out}");
}

#[test]
fn nothing_is_flagged_under_the_default_budgets() {
    let bin = fixture();
    let Some(out) = run_kuna(&["functions", &bin, "--summary", "--json"]) else { return };
    assert!(out.contains(r#""limits":{"maxinstruction":100000,"jumptablemax":1024,"over":[]}"#), "{out}");
}

/// The plain listing never walks the image for `limits`; a selection that
/// already walked it (`--reachable-from`) carries the same object.
#[test]
fn only_a_listing_that_already_walked_the_image_carries_limits() {
    let bin = fixture();
    let Some(plain) = run_kuna(&["functions", &bin, "--json", "--option", "jumptablemax", "2"])
    else {
        return;
    };
    assert!(!plain.contains(r#""limits""#), "{plain}");

    let reached = run_kuna(&[
        "functions", &bin, "--json", "--reachable-from", "sub_140001040", "--option",
        "jumptablemax", "2",
    ])
    .expect("the reachable-from listing");
    assert!(reached.contains(r#""limits":{"maxinstruction":100000,"jumptablemax":2,"over":[{"name":"sub_140001040""#), "{reached}");
    assert!(reached.contains(r#""over_maxinstruction":false"#), "{reached}");
    assert!(reached.contains(OVER_CAP), "{reached}");
}

/// At a ceiling equal to the case count the table is read whole: the switch is
/// no longer listed, and the two case bodies the lower ceiling left unread now
/// count, taking the body past a budget the truncated read stayed under.
#[test]
fn a_ceiling_at_the_case_count_reads_the_whole_table() {
    let bin = fixture();
    let summary = |cap: &str| {
        run_kuna(&[
            "functions", &bin, "--summary", "--json", "--option", "jumptablemax", cap, "--option",
            "maxinstruction", "16",
        ])
    };
    let Some(truncated) = summary("2") else { return };
    assert!(truncated.contains(r#""instructions":16,"over_maxinstruction":false"#), "{truncated}");
    let whole = summary("4").expect("the whole-table summary");
    assert!(
        whole.contains(r#""instructions":17,"over_maxinstruction":true,"switches_over_jumptablemax":[]"#),
        "{whole}"
    );
}

/// A ceiling below the two entries a table needs reads nothing, and the switch
/// is still reported as over it.
#[test]
fn a_ceiling_below_a_table_still_reports_the_switch() {
    let bin = fixture();
    let Some(out) = run_kuna(&["functions", &bin, "--summary", "--json", "--option", "jumptablemax", "1"])
    else {
        return;
    };
    assert!(out.contains(r#""address_hex":"0x14000105d","cases":4,"read":0"#), "{out}");
}

/// The gcc `.cold` shape: a fragment below its parent jumps into the middle of
/// the parent's body, so the reference walk decodes the parent's tail from the
/// fragment before it walks the parent. Each function is still measured on its
/// own descent: the parent is 45 instructions (4 before the join, 40 after it,
/// and the `ret`), the fragment 43 (its `nop` and `jmp`, then the shared tail).
#[test]
fn a_function_is_measured_on_its_own_descent_whatever_the_walk_reached_first() {
    let mut code = vec![0x90, 0xe9, 0x0e, 0x00, 0x00, 0x00];
    code.resize(0x10, 0xcc);
    code.extend([0x90; 44]);
    code.push(0xc3);
    let image = arm_images::elf_for(
        object::Architecture::I386,
        &code,
        &[],
        &[(0, "fragment", 6), (0x10, "parent", 45)],
    );
    let path = common::scratch_file("limits-cold-fragment", "elf");
    std::fs::write(&path, image).unwrap();
    let bin = path.to_str().unwrap();

    let Some(out) = run_kuna(&["functions", bin, "--summary", "--json", "--option", "maxinstruction", "44"])
    else {
        return;
    };
    assert!(out.contains(r#""over":[{"name":"parent","address":65552,"address_hex":"0x10010","size":45,"instructions":45,"over_maxinstruction":true"#), "{out}");

    let out = run_kuna(&["functions", bin, "--summary", "--json", "--option", "maxinstruction", "42"])
        .expect("the lower budget");
    assert!(out.contains(r#""name":"fragment","address":65536,"#), "{out}");
    assert!(out.contains(r#""instructions":43,"over_maxinstruction":true,"switches_over_jumptablemax":[]},{"name":"parent""#), "{out}");
}

#[test]
fn jumptablemax_is_catalogued_and_refuses_anything_but_a_positive_count() {
    let out = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args(["catalog", "--json"])
        .output()
        .expect("failed to spawn the kuna binary");
    let catalog = String::from_utf8_lossy(&out.stdout);
    assert!(catalog.contains(r#""option": "jumptablemax""#), "{catalog}");

    let bin = fixture();
    for bad in ["wide", "12abc", "1.5", "-5", "0", "4294967296"] {
        let out = kuna(&["functions", &bin, "--json", "--option", "jumptablemax", bad]);
        let stderr = String::from_utf8_lossy(&out.stderr);
        if stderr.contains("could not build an architecture") {
            return;
        }
        assert_ne!(out.status.code(), Some(0), "{bad:?} was accepted");
        assert!(stderr.contains("Must specify integer maximum"), "{bad:?}: {stderr}");
    }
}
