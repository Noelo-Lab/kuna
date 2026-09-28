//! Validate the embedded manual through the CLI, including a relocated binary.

use std::path::Path;
use std::process::{Command, Output, Stdio};

use serde_json::Value;

mod common;
use common::repo_root;

const REQUIRED: [&str; 5] = ["cli", "options", "agents", "phases", "modes"];

fn run_docs(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_kuna"))
        .arg("docs")
        .args(args)
        .output()
        .expect("spawn kuna docs")
}

fn docs(args: &[&str]) -> String {
    let output = run_docs(args);
    assert!(
        output.status.success(),
        "kuna docs {args:?}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).expect("UTF-8 document")
}

fn index() -> Vec<Value> {
    serde_json::from_str(&docs(&["--json"])).expect("JSON topic list")
}

fn source(topic: &str) -> String {
    std::fs::read_to_string(repo_root().join(format!("docs/{topic}.md"))).unwrap()
}

#[test]
fn embedded_docs_match_the_files_on_disk() {
    for topic in REQUIRED {
        assert_eq!(
            docs(&[topic]),
            source(topic),
            "stale embedded {topic}; rebuild kuna-cli"
        );
    }
}

#[test]
fn the_topics_an_agent_needs_are_embedded_in_priority_order() {
    let rows = index();
    let topics: Vec<_> = rows
        .iter()
        .map(|row| row["topic"].as_str().unwrap())
        .collect();
    assert!(topics.starts_with(&REQUIRED), "{topics:?}");
    for row in rows {
        assert!(!row["title"].as_str().unwrap().is_empty());
        assert!(!row["summary"].as_str().unwrap().is_empty());
        assert!(row["bytes"].as_u64().unwrap() > 1024);
    }
}

#[test]
fn the_option_catalog_arrives_whole() {
    let catalog = docs(&["options"]);
    assert!(catalog.contains("## Symptom index"));
    assert!(catalog.lines().count() > 900);
}

#[test]
fn the_list_is_one_line_per_topic() {
    let list = docs(&[]);
    let lines: Vec<_> = list.lines().collect();
    let rows = index();
    assert_eq!(lines.len(), rows.len());
    for (line, row) in lines.iter().zip(rows) {
        assert_eq!(line.split_whitespace().next(), row["topic"].as_str());
        assert!(line.contains(row["summary"].as_str().unwrap()), "{line}");
    }
}

#[test]
fn the_json_list_is_the_documented_shape() {
    for row in index() {
        let object = row.as_object().expect("topic object");
        let keys: Vec<_> = object.keys().map(String::as_str).collect();
        assert_eq!(keys, ["topic", "title", "summary", "bytes"]);
        let topic = row["topic"].as_str().unwrap();
        assert_eq!(row["bytes"].as_u64(), Some(docs(&[topic]).len() as u64));
    }
}

#[test]
fn all_concatenates_every_document_verbatim() {
    let all = docs(&["--all"]);
    for topic in REQUIRED {
        assert!(
            all.contains(source(topic).trim_end_matches('\n')),
            "missing {topic}"
        );
        assert!(
            all.contains(&format!("docs/{topic}.md")),
            "missing {topic} label"
        );
    }
}

#[test]
fn an_unknown_topic_is_a_usage_error_that_names_the_real_ones() {
    let output = run_docs(&["xrefs"]);
    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8(output.stderr).unwrap();
    for topic in REQUIRED {
        assert!(stderr.contains(topic), "{stderr}");
    }
}

#[test]
fn the_binary_carries_its_docs_out_of_the_repo() {
    let sandbox = std::env::temp_dir().join(format!("kuna_docs_norepo_{}", std::process::id()));
    std::fs::create_dir(&sandbox).expect("create sandbox");
    let exe = sandbox.join("kuna");
    std::fs::copy(env!("CARGO_BIN_EXE_kuna"), &exe).expect("copy kuna");
    make_executable(&exe);
    let output = Command::new(&exe)
        .current_dir(&sandbox)
        .args(["docs", "cli"])
        .env_remove("KUNA_SPECS")
        .env_remove("SLEIGHHOME")
        .env_remove("KUNA_DECOMP_DBG")
        .output()
        .expect("run relocated kuna");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(output.stdout, source("cli").as_bytes());
    assert_eq!(
        std::fs::read_dir(&sandbox).unwrap().count(),
        1,
        "docs left files behind"
    );
    std::fs::remove_dir_all(&sandbox).unwrap();
}

#[cfg(unix)]
fn make_executable(path: &Path) {
    use std::os::unix::fs::PermissionsExt;
    let mut perms = std::fs::metadata(path).unwrap().permissions();
    perms.set_mode(0o755);
    std::fs::set_permissions(path, perms).unwrap();
}

#[cfg(not(unix))]
fn make_executable(_path: &Path) {}

#[test]
fn a_reader_that_walks_away_is_not_a_panic() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args(["docs", "--all"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn kuna");
    drop(child.stdout.take().expect("stdout pipe"));
    let output = child.wait_with_output().expect("wait for kuna");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(!stderr.contains("panicked"), "{stderr}");
    assert!(
        output.status.success(),
        "exited {:?}: {stderr}",
        output.status.code()
    );
}
