//! CLI end-to-end gate for runtime/packer hints: `functions --summary` carries
//! them in `summary.runtime`, every other binary subcommand prints the
//! actionable ones as a `note:` line on stderr, and stdout stays clean JSON.
//!
//! Fixture: `pyinstaller_cookie_x86_64` (`arraycoverwidth_x86_64` with a
//! PyInstaller CArchive cookie appended). A missing `.sla` is a visible skip.

use std::path::PathBuf;
use std::process::Command;

fn fixture(name: &str) -> String {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .unwrap()
        .join("decompiler/crates/kuna-analysis/tests/fixtures")
        .join(name)
        .to_str()
        .unwrap()
        .to_string()
}

fn run_kuna(args: &[&str]) -> (String, String, i32) {
    let specs = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../specs");
    let out = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args(args)
        .env("SLEIGHHOME", specs)
        .output()
        .expect("failed to spawn the kuna binary");
    (
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
        out.status.code().unwrap_or(-1),
    )
}

fn no_specs(stderr: &str, code: i32) -> bool {
    code != 0 && (stderr.contains("could not build an architecture") || stderr.contains(".sla"))
}

const PYI_HINT: &str = "PyInstaller bundle (Python 3.12): extract with pyinstxtractor-ng";

#[test]
fn summary_json_carries_the_runtime_array() {
    let bin = fixture("pyinstaller_cookie_x86_64");
    let (out, err, code) = run_kuna(&["functions", &bin, "--summary", "--json"]);
    if no_specs(&err, code) {
        eprintln!("skipping: no .sla ({err})");
        return;
    }
    assert_eq!(code, 0, "{err}");
    assert!(out.contains("\"runtime\": ["), "{out}");
    assert!(out.contains("\"id\": \"pyinstaller\""), "{out}");
    assert!(out.contains("\"version\": \"3.12\""), "{out}");
    assert!(out.contains(PYI_HINT), "{out}");
    assert!(!err.contains("note:"), "the summary already says it: {err}");

    let (text, _, _) = run_kuna(&["functions", &bin, "--summary"]);
    assert!(text.contains(&format!("runtime\tpyinstaller\t{PYI_HINT}")), "{text}");
}

#[test]
fn a_plain_image_has_an_empty_runtime_array_and_no_note() {
    let bin = fixture("arraycoverwidth_x86_64");
    let (out, err, code) = run_kuna(&["functions", &bin, "--summary", "--json"]);
    if no_specs(&err, code) {
        eprintln!("skipping: no .sla ({err})");
        return;
    }
    assert!(out.contains("\"runtime\": []"), "{out}");
    let (_, err, _) = run_kuna(&["decompile", &bin, "vm"]);
    assert!(!err.contains("note:"), "{err}");
}

#[test]
fn other_subcommands_note_the_hint_on_stderr_only() {
    let bin = fixture("pyinstaller_cookie_x86_64");
    let (out, err, code) = run_kuna(&["functions", &bin, "--json"]);
    if no_specs(&err, code) {
        eprintln!("skipping: no .sla ({err})");
        return;
    }
    assert_eq!(code, 0, "{err}");
    assert_eq!(err.matches("note:").count(), 1, "{err}");
    assert!(err.contains(PYI_HINT), "{err}");
    assert!(!out.contains("pyinstxtractor"), "stdout stays the document: {out}");

    let (out, err, _) = run_kuna(&["decompile", &bin, "vm", "--json"]);
    assert!(err.contains(PYI_HINT), "{err}");
    assert!(!out.contains("pyinstxtractor"), "{out}");
}
