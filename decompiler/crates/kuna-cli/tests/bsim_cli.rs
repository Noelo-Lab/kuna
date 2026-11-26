//! `kuna bsim` end to end: build a corpus from a real binary and query it.
//!
//! Exercise fixed profiles, retryable/serialized appends, and self-matches.

use std::path::Path;
use std::process::{Command, Output};

use serde_json::Value;

use crate::common;

/// A small stripped x86-64 ELF; the fixture README pins its function addresses.
fn binary() -> String {
    common::repo_root()
        .join("tests/fixtures/fid/callees/prog")
        .to_str()
        .unwrap()
        .to_owned()
}

fn weights(dir: &Path, profile: &str, settings: &str) {
    // A deterministic scoring fixture in the stock XML shape. The tests do
    // not skip on machines that lack an installed Ghidra.
    let xml = format!(
        "<weights settings=\"{settings}\"><weightfactory scale=\"1\" addend=\"1\">\
         <weightnorm>1</weightnorm><probflip0>1</probflip0><probflip1>1</probflip1>\
         <probdiff0>1</probdiff0><probdiff1>1</probdiff1>{}{}</weightfactory>\
         <idflookup/></weights>",
        "<idf>1</idf>".repeat(512),
        "<tf>1</tf>".repeat(64),
    );
    std::fs::write(dir.join(format!("lshweights_{profile}.xml")), xml).unwrap();
}

fn kuna_command() -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_kuna"));
    command
        .env("SLEIGHHOME", common::repo_root().join("specs"))
        .env_remove("KUNA_BSIM_WEIGHTS");
    command
}

fn successful(output: Output) -> Output {
    assert_eq!(output.status.code(), Some(0), "{}", String::from_utf8_lossy(&output.stderr));
    output
}

fn info(corpus: &str) -> Value {
    let output = successful(kuna(&["bsim", "info", corpus, "--json"]));
    serde_json::from_slice(&output.stdout).expect("info JSON")
}

fn build_capped(bin: &str, corpus: &str, cap: &str) -> Output {
    kuna(&["bsim", "build", bin, "-o", corpus, "--max-functions", cap])
}

fn query(bin: &str, corpus: &str, addr: &str, floor: &str) -> Output {
    kuna(&[
        "bsim", "query", bin, "--corpus", corpus, "--address", addr,
        "--min-similarity", floor, "--json",
    ])
}

fn assert_error(output: Output, message: &str) {
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains(message), "{stderr}");
}

fn kuna(args: &[&str]) -> Output {
    kuna_command().args(args)
        .output()
        .expect("run kuna bsim")
}

#[test]
fn a_capped_ingest_can_be_retried_without_duplicate_signatures() {
    let scratch = tempfile::tempdir().unwrap();
    weights(scratch.path(), "64", "0x49");
    let path = scratch.path().join("prog.kbsm");
    let corpus = path.to_str().unwrap();
    let bin = binary();
    successful(build_capped(&bin, corpus, "1"));
    let doc = info(corpus);
    assert_eq!(doc["profile"], "64");
    assert_eq!(doc["settings"], "0x49");
    assert_eq!(doc["functions"], 1);
    assert_eq!(doc["executables"].as_array().unwrap().len(), 1);
    assert_eq!(doc["executables"][0]["name"], "prog");
    assert_eq!(doc["executables"][0]["architecture"], "x86:LE:64:default");
    assert_eq!(doc["executables"][0]["compiler"], "gcc");
    assert_eq!(doc["executables"][0]["md5"], "59377642fb56e661dd49b2076b60f8c8");
    assert_eq!(doc["executables"][0]["complete"], false);

    let previous = std::fs::read(&path).unwrap();
    successful(build_capped(&bin, corpus, "1"));
    assert_eq!(std::fs::read(&path).unwrap(), previous);

    successful(build_capped(&bin, corpus, "2"));
    let doc = info(corpus);
    assert_eq!(doc["functions"], 2);
    assert_eq!(doc["executables"].as_array().unwrap().len(), 1);
    assert_eq!(doc["executables"][0]["functions"], 2);

    let i386 = common::fixture("x86listingswitch_i386");
    let previous = std::fs::read(&path).unwrap();
    assert_error(
        build_capped(&i386, corpus, "1"),
        "incompatible with corpus profile 64",
    );
    assert_error(
        query(&i386, corpus, "0x100000", "0.7"),
        "incompatible with corpus profile 64",
    );
    assert_eq!(std::fs::read(&path).unwrap(), previous);
}

#[test]
fn a_nosize_corpus_signs_and_queries_mixed_widths_with_its_own_weights() {
    let scratch = tempfile::tempdir().unwrap();
    weights(scratch.path(), "nosize", "0x4d");
    let path = scratch.path().join("prog.kbsm");
    let corpus = path.to_str().unwrap();
    let bin = binary();
    successful(kuna(&["bsim", "build", &bin, "-o", corpus, "--profile", "nosize"]));
    let i386 = common::fixture("x86listingswitch_i386");
    successful(build_capped(&i386, corpus, "1"));
    let stored = info(corpus);
    assert_eq!(stored["profile"], "nosize");
    assert_eq!(stored["settings"], "0x4d");
    assert_eq!(stored["executables"].as_array().unwrap().len(), 2);
    assert_eq!(stored["executables"][1]["architecture"], "x86:LE:32:default");

    let queried = successful(query(&bin, corpus, "0x401000", "0.99999999"));
    let doc: Value = serde_json::from_slice(&queried.stdout).expect("query JSON");
    assert_eq!(doc["function"]["address"], "0x401000");
    assert_eq!(doc["profile"], "nosize");
    assert_eq!(doc["settings"], "0x4d");
    assert_eq!(doc["considered"], stored["functions"], "{doc}");
    let exact = doc["matches"].as_array().unwrap().iter()
        .find(|m| m["address"] == "0x401000" && m["executable"] == "prog")
        .unwrap_or_else(|| panic!("missing self-match: {doc}"));
    assert!((exact["similarity"].as_f64().unwrap() - 1.0).abs() < 1e-9, "{exact}");
    assert!(exact["significance"].as_f64().unwrap() > 0.0, "{exact}");

    let strict = successful(query(&bin, corpus, "0x401000", "1.5"));
    let doc: Value = serde_json::from_slice(&strict.stdout).expect("query JSON");
    assert!(doc["matches"].as_array().unwrap().is_empty());
    let reason = doc["reason"].as_str().expect("a reason");
    assert!(reason.contains("1.50"), "{reason}");
    assert_error(query(&bin, corpus, "0x401000", "NaN"), "needs a finite number");

    let previous = std::fs::read(&path).unwrap();
    weights(scratch.path(), "nosize", "0x49");
    assert_error(query(&bin, corpus, "0x401000", "0.7"), "stored settings 0x4d");
    assert_error(build_capped(&i386, corpus, "2"), "stored settings 0x4d");
    assert_eq!(std::fs::read(&path).unwrap(), previous);
}

#[test]
fn concurrent_appends_preserve_both_inputs() {
    let scratch = tempfile::tempdir().unwrap();
    weights(scratch.path(), "64", "0x49");
    let path = scratch.path().join("corpus.kbsm");
    let corpus = path.to_str().unwrap();
    let bin = binary();
    let other = scratch.path().join("other");
    let mut bytes = std::fs::read(&bin).unwrap();
    bytes.push(0); // Keep identical executable code but give the file a distinct MD5.
    std::fs::write(&other, bytes).unwrap();

    let first = std::thread::scope(|scope| {
        let first = scope.spawn(|| build_capped(&bin, corpus, "1"));
        successful(build_capped(other.to_str().unwrap(), corpus, "1"));
        first.join().unwrap()
    });
    successful(first);
    let doc = info(corpus);
    assert_eq!(doc["functions"], 2);
    let names: Vec<&str> = doc["executables"].as_array().unwrap().iter()
        .map(|e| e["name"].as_str().unwrap()).collect();
    assert_eq!(names.len(), 2);
    assert!(names.contains(&"prog") && names.contains(&"other"), "{doc}");
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

    let old = scratch.path().join("old.kbsm");
    std::fs::write(&old, b"KBSM\x01\0\0\0").unwrap();
    assert_error(kuna(&["bsim", "info", old.to_str().unwrap()]), "rebuild");
}

#[test]
fn local_bsim_builds_and_queries_without_external_weights() {
    let scratch = tempfile::tempdir().unwrap();
    let path = scratch.path().join("standalone.kbsm");
    let corpus = path.to_str().unwrap();
    let bin = binary();
    let invoke = |argv: &[&str]| {
        successful(kuna_command().env("PATH", scratch.path()).args(argv).output().unwrap())
    };

    invoke(&["bsim", "build", &bin, "-o", corpus, "--max-functions", "1"]);
    let result = invoke(&["bsim", "query", &bin, "--corpus", corpus, "--address",
        "0x401000", "--json"]);
    let doc: Value = serde_json::from_slice(&result.stdout).unwrap();
    let top = &doc["matches"][0];
    assert_eq!(top["address"], "0x401000");
    assert!((top["similarity"].as_f64().unwrap() - 1.0).abs() < 1e-9, "{doc}");
    assert!(top["significance"].as_f64().unwrap() > 0.0, "{doc}");
}

#[test]
fn offline_compare_decompiles_both_entries_without_database() {
    let scratch = tempfile::tempdir().unwrap();
    let bin = binary();
    let result = successful(kuna_command().env("PATH", scratch.path())
        .env_remove("KUNA_BSIM_POSTGRES")
        .args(["bsim", "compare", &bin, "--address", "0x401000", "--reference", &bin,
            "--reference-address", "0x401020", "--json"])
        .output().unwrap());
    let doc: Value = serde_json::from_slice(&result.stdout).unwrap();
    assert!(doc["match"].is_null(), "{doc}");
    let diff = doc["diff"].as_str().unwrap();
    assert!(diff.lines().any(|line| line.starts_with("- ") && line.contains("return")), "{doc}");
    assert!(diff.lines().any(|line| line.starts_with("+ ") && line.contains("return")), "{doc}");
}
