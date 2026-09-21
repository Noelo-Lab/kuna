//! CLI end-to-end gate for the Rust back-end's fidelity report (GH-668).
//!
//! A jump the structurer could not remove and Rust cannot spell renders as a
//! diverging `panic!`, and the path it sits on is not a translation of the
//! binary. That used to be visible only by reading the text: exit 0, silent
//! stderr, `"error": null`. Each test below fixes one channel — the stderr note,
//! the per-function JSON count, and the two verdicts that deliberately do NOT
//! move — over both the forked `decomp_dbg` surface and the in-process one.
//!
//! The C control is half the gate: a real `goto` in C output is correct, so it
//! must report nothing and count zero.
//!
//! ## `.sla` precondition
//!
//! Every test bootstraps a real x86 architecture, which needs the built `.sla`
//! under `specs/` (gitignored; `make specs`). Without it the command cannot
//! build an architecture; the test says so and returns — a specs-less CI is a
//! visible skip, never a false green.

use std::path::PathBuf;
use std::process::Command;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

/// A checked-in non-PIE x86-64 ELF whose entry point is a two-entry (irreducible)
/// loop: the structurer keeps exactly one residual goto for it.
fn fixture() -> String {
    repo_root()
        .join("decompiler/crates/kuna-analysis/tests/fixtures/irreducible_x86_64")
        .to_str()
        .unwrap()
        .to_string()
}

fn run_kuna(args: &[&str]) -> (String, String, Option<i32>) {
    let out = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .env("SLEIGHHOME", repo_root().join("specs"))
        .args(args)
        .output()
        .expect("failed to spawn the kuna binary");
    (
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
        out.status.code(),
    )
}

fn is_specs_skip(stderr: &str) -> bool {
    stderr.contains("could not build an architecture")
        || stderr.contains("Could not find .sla")
        || stderr.contains("Could not discover")
        || stderr.contains("SLEIGH")
}

/// The marker the printer emits, spelled here exactly once so a drift in
/// `kuna_langrust` fails this file rather than passing it silently.
const MARKER: &str = "panic!(\"kuna: unstructured goto to ";

/// The forked `decomp_dbg` surface: the note names the function and the count,
/// and points at the two ways out.
#[test]
fn the_text_surface_reports_the_marker_on_stderr() {
    let (stdout, stderr, code) = run_kuna(&["decompile", &fixture(), "irreducible", "--language", "rust"]);
    if is_specs_skip(&stderr) {
        eprintln!("SKIP: no built .sla under specs/ ({})", stderr.trim());
        return;
    }
    assert!(stdout.contains(MARKER), "expected the marker in:\n{stdout}");
    assert!(
        stderr.contains("note: 1 unstructured goto in irreducible"),
        "expected the fidelity note, got:\n{stderr}"
    );
    assert!(stderr.contains("--language c"), "the note names the C escape hatch:\n{stderr}");
    // The deliberate half: a lossy render is a warning, not a failed run.
    assert_eq!(code, Some(0), "the verdict does not move; stderr was:\n{stderr}");
}

/// The in-process surface: the same note, plus the machine-readable count that a
/// `decompile-all` consumer filters on. `error` stays `null` on purpose.
#[test]
fn the_json_surface_counts_the_marker_per_function() {
    let (stdout, stderr, code) =
        run_kuna(&["decompile", &fixture(), "irreducible", "--language", "rust", "--json"]);
    if is_specs_skip(&stderr) {
        eprintln!("SKIP: no built .sla under specs/ ({})", stderr.trim());
        return;
    }
    assert!(
        stdout.contains("\"unstructured_gotos\": 1"),
        "expected the per-function count in:\n{stdout}"
    );
    assert!(stdout.contains("\"error\": null"), "a lossy render is not a failed run:\n{stdout}");
    assert!(
        stderr.contains("note: 1 unstructured goto in irreducible"),
        "expected the fidelity note, got:\n{stderr}"
    );
    assert_eq!(code, Some(0));
}

/// The whole-binary surface answers per record, so a consumer can pick out the
/// functions worth re-running without parsing any text.
#[test]
fn decompile_all_carries_the_count_on_every_record() {
    let (stdout, stderr, code) =
        run_kuna(&["decompile-all", &fixture(), "--language", "rust", "--json"]);
    if is_specs_skip(&stderr) {
        eprintln!("SKIP: no built .sla under specs/ ({})", stderr.trim());
        return;
    }
    assert!(stdout.contains("\"unstructured_gotos\": 1"), "expected one lossy record in:\n{stdout}");
    assert!(
        stderr.contains("unstructured goto"),
        "a whole-binary run says it too, got:\n{stderr}"
    );
    assert_eq!(code, Some(0));
}

/// A `goto` in C is a statement, not a defect: nothing to report and nothing to
/// count. Without this the count could be read off any rendered body.
#[test]
fn c_output_reports_nothing_and_counts_zero() {
    let (stdout, stderr, code) = run_kuna(&["decompile", &fixture(), "irreducible"]);
    if is_specs_skip(&stderr) {
        eprintln!("SKIP: no built .sla under specs/ ({})", stderr.trim());
        return;
    }
    assert!(stdout.contains("goto label_"), "C spells the jump for real:\n{stdout}");
    assert!(!stderr.contains("unstructured goto"), "C has nothing to report:\n{stderr}");
    assert_eq!(code, Some(0));

    let (stdout, stderr, code) = run_kuna(&["decompile", &fixture(), "irreducible", "--json"]);
    if is_specs_skip(&stderr) {
        return;
    }
    assert!(stdout.contains("\"unstructured_gotos\": 0"), "expected a zero count in:\n{stdout}");
    assert_eq!(code, Some(0));
}
