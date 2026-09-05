//! CLI contract checks for `kuna graph-export`.

use std::path::PathBuf;
use std::process::Command;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .unwrap()
}

fn fauxware() -> String {
    repo_root()
        .join("decompiler/crates/kuna-analysis/tests/fixtures/fauxware")
        .to_string_lossy()
        .into_owned()
}

fn specs() -> String {
    repo_root().join("specs").to_string_lossy().into_owned()
}

fn missing_specs(stderr: &str) -> bool {
    stderr.contains("could not build an architecture")
        || stderr.contains("SLEIGH")
        || stderr.contains("Could not discover")
}

#[test]
fn graph_export_emits_v2_json_with_ordered_call_edges() {
    let output = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args([
            "graph-export",
            &fauxware(),
            "fixture-version",
            "--sleighpath",
            &specs(),
        ])
        .output()
        .expect("spawn kuna graph-export");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    if !output.status.success() && missing_specs(&stderr) {
        eprintln!("graph_export_cli: skipping (no `.sla`; run `make specs`): {stderr}");
        return;
    }
    assert!(output.status.success(), "graph-export failed: {stderr}");
    assert!(stdout.starts_with("{\n"), "not JSON: {stdout}");
    for key in [
        "\"schemaVersion\": 2",
        "\"version\": \"fixture-version\"",
        "\"analysisImageBase\": ",
        "\"functions\": [",
        "\"edges\": [",
        "\"kind\": ",
        "\"hasIndirectCalls\": ",
        "\"callerAddress\": ",
        "\"calleeAddress\": ",
        "\"calleeOrder\": ",
    ] {
        assert!(stdout.contains(key), "missing {key} from:\n{stdout}");
    }
    assert!(
        !stdout.contains("\"address\": \"0x"),
        "addresses must be JSON numbers"
    );
}

#[test]
fn graph_export_writes_only_to_requested_file() {
    let path = std::env::temp_dir().join(format!("kuna-graph-export-{}.json", std::process::id()));
    let output = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args([
            "graph-export",
            &fauxware(),
            "-o",
            path.to_str().unwrap(),
            "--sleighpath",
            &specs(),
        ])
        .output()
        .expect("spawn kuna graph-export");
    let stderr = String::from_utf8_lossy(&output.stderr);
    if !output.status.success() && missing_specs(&stderr) {
        return;
    }
    assert!(output.status.success(), "graph-export failed: {stderr}");
    assert!(
        output.stdout.is_empty(),
        "file output must not mix JSON into stdout"
    );
    let document = std::fs::read_to_string(&path).expect("exported JSON file");
    let _ = std::fs::remove_file(path);
    assert!(document.contains("\"schemaVersion\": 2"));
}
