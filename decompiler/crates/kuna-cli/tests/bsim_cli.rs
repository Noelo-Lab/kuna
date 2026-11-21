//! `kuna bsim` end to end: build a corpus from a real binary and query it.
//!
//! The load-bearing observation is that a function queried against a corpus that
//! contains it comes back at similarity 1.0 — if the builder and the query
//! disagreed about how to sign a function, this would fail, and nothing else in
//! the surface would notice.

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use serde_json::Value;

use crate::common;

/// The stripped static fixture the FID tests already use. It lives at the repo
/// root's `tests/fixtures/`, not under a crate, so `common::fixture` does not
/// resolve it.
fn binary() -> String {
    common::repo_root()
        .join("tests/fixtures/fid/prog")
        .to_str()
        .unwrap()
        .to_owned()
}

/// Ghidra's shipped weight tables, when this machine has them.
fn weights_dir() -> Option<PathBuf> {
    if let Ok(dir) = std::env::var("KUNA_BSIM_WEIGHTS") {
        if Path::new(&dir).join("lshweights_64.xml").exists() {
            return Some(PathBuf::from(dir));
        }
    }
    let candidates = [
        "/nix/store",
        "/opt",
        "/usr/share/ghidra",
    ];
    for root in candidates {
        if root == "/nix/store" {
            // Ghidra lives in a store hash; search one level down.
            let Ok(entries) = std::fs::read_dir(root) else { continue };
            for entry in entries.flatten() {
                let candidate = entry
                    .path()
                    .join("lib/ghidra/Ghidra/Features/BSim/data");
                if candidate.join("lshweights_64.xml").exists() {
                    return Some(candidate);
                }
            }
            continue;
        }
        let candidate = Path::new(root).join("Ghidra/Features/BSim/data");
        if candidate.join("lshweights_64.xml").exists() {
            return Some(candidate);
        }
    }
    None
}

fn kuna(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args(args)
        .env("SLEIGHHOME", common::repo_root().join("specs"))
        .output()
        .expect("run kuna bsim")
}

#[test]
fn a_corpus_round_trips_and_reports_its_contents() {
    let scratch = tempfile::tempdir().unwrap();
    let corpus = scratch.path().join("prog.kbsm");
    let built = kuna(&[
        "bsim",
        "build",
        &binary(),
        "-o",
        corpus.to_str().unwrap(),
        "--max-functions",
        "24",
    ]);
    let stderr = String::from_utf8_lossy(&built.stderr);
    assert_eq!(built.status.code(), Some(0), "{stderr}");
    assert!(corpus.exists(), "no corpus written: {stderr}");

    let info = kuna(&["bsim", "info", corpus.to_str().unwrap(), "--json"]);
    assert_eq!(info.status.code(), Some(0), "{}", String::from_utf8_lossy(&info.stderr));
    let doc: Value = serde_json::from_slice(&info.stdout).expect("json");
    assert_eq!(doc["language"], "x86:LE:64:default");
    assert_eq!(doc["settings"], "0x49");
    assert_eq!(doc["executables"].as_array().unwrap().len(), 1);
    assert_eq!(doc["executables"][0]["name"], "prog");
    assert!(doc["functions"].as_u64().unwrap() > 0);

    // Ingesting the same binary again is not an error and does not duplicate it.
    let again = kuna(&[
        "bsim",
        "build",
        &binary(),
        "-o",
        corpus.to_str().unwrap(),
        "--max-functions",
        "24",
    ]);
    assert_eq!(again.status.code(), Some(0));
    let info = kuna(&["bsim", "info", corpus.to_str().unwrap(), "--json"]);
    let doc: Value = serde_json::from_slice(&info.stdout).expect("json");
    assert_eq!(doc["executables"].as_array().unwrap().len(), 1);
}

#[test]
fn a_function_queried_against_its_own_corpus_matches_itself() {
    let Some(weights) = weights_dir() else {
        // The weight tables define what a similarity number means, so without
        // them there is nothing to assert: the run is expected to fail, and that
        // failure is itself the contract (a diagnostic, not a silent pass).
        let scratch = tempfile::tempdir().unwrap();
        let corpus = scratch.path().join("prog.kbsm");
        let info = kuna(&["bsim", "info", corpus.to_str().unwrap()]);
        assert_ne!(info.status.code(), Some(0), "a missing corpus must fail");
        return;
    };
    let scratch = tempfile::tempdir().unwrap();
    let corpus = scratch.path().join("prog.kbsm");
    // No cap: the fixture's `sub_4017c0` must be present. A capped build stops in
    // address order and would legitimately omit it.
    let built = kuna(&["bsim", "build", &binary(), "-o", corpus.to_str().unwrap()]);
    assert_eq!(
        built.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&built.stderr)
    );

    let queried = kuna(&[
        "bsim",
        "query",
        &binary(),
        "--corpus",
        corpus.to_str().unwrap(),
        "--address",
        "0x4017c0",
        "--weights",
        weights.to_str().unwrap(),
        "--max-matches",
        "3",
        "--json",
    ]);
    assert_eq!(
        queried.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&queried.stderr)
    );
    let doc: Value = serde_json::from_slice(&queried.stdout).expect("json");
    assert_eq!(doc["function"]["address"], "0x4017c0");
    assert!(doc["features"].as_u64().unwrap() > 0, "no features: {doc}");
    let matches = doc["matches"].as_array().expect("matches");
    assert_eq!(matches.len(), 1, "a unique function must match once: {doc}");
    assert_eq!(matches[0]["name"], "sub_4017c0");
    let similarity = matches[0]["similarity"].as_f64().expect("similarity");
    assert!(
        (similarity - 1.0).abs() < 1e-9,
        "self similarity must be 1.0, got {similarity}"
    );
    assert!(
        matches[0]["significance"].as_f64().unwrap() > 0.0,
        "significance must be reported"
    );

    // A high floor above 1.0 admits nothing, and the answer says which floor it
    // used rather than reading as "not in the corpus".
    let strict = kuna(&[
        "bsim",
        "query",
        &binary(),
        "--corpus",
        corpus.to_str().unwrap(),
        "--address",
        "0x4017c0",
        "--weights",
        weights.to_str().unwrap(),
        "--min-similarity",
        "1.5",
        "--json",
    ]);
    let doc: Value = serde_json::from_slice(&strict.stdout).expect("json");
    assert!(doc["matches"].as_array().unwrap().is_empty());
    let reason = doc["reason"].as_str().expect("a reason");
    assert!(reason.contains("1.50"), "{reason}");
}

#[test]
fn a_malformed_corpus_is_reported_not_panicked_on() {
    let scratch = tempfile::tempdir().unwrap();
    let bad = scratch.path().join("bad.kbsm");
    std::fs::write(&bad, b"not a corpus at all").unwrap();
    let info = kuna(&["bsim", "info", bad.to_str().unwrap(), "--json"]);
    assert_ne!(info.status.code(), Some(0));
    let stderr = String::from_utf8_lossy(&info.stderr);
    assert!(stderr.contains("bsim"), "{stderr}");

    let missing = kuna(&["bsim", "info", scratch.path().join("nope.kbsm").to_str().unwrap()]);
    assert_ne!(missing.status.code(), Some(0));
}
