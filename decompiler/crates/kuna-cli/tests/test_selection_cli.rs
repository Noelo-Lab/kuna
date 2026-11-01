//! `kuna test` names a requested test that does not exist instead of scoring
//! the rest as complete (issue #1375): a missing `--name`, an empty corpus or an
//! unreadable corpus directory is reported in `selection` and exits 2.

#![cfg(unix)]

use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Output;

use serde_json::Value;

use crate::common;

/// A stand-in `decomp_test_dbg` with the real one's argv and stream grammar:
/// each requested datatest file that exists passes one test, and the unit
/// registry is empty.
const HARNESS: &str = r#"#!/bin/sh
dir=$4
mode=$5
shift 5
if [ "$mode" = unittests ]; then
  printf '==============================\n0/0 tests passed.\n' >&2
  exit 0
fi
[ $# -eq 0 ] && set -- $(cd "$dir" 2>/dev/null && ls *.xml 2>/dev/null)
n=0
for f in "$@"; do
  if [ -f "$dir/$f" ]; then echo "Success -- $f"; n=$((n+1)); fi
done
printf 'Total tests applied = %d\nTotal passing tests = %d\n' $n $n
"#;

fn harness() -> PathBuf {
    let path = common::scratch_file("selection-harness", "sh");
    std::fs::write(&path, HARNESS).unwrap();
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o700)).unwrap();
    path
}

fn corpus(files: &[&str]) -> PathBuf {
    let dir = common::scratch_file("selection-corpus", "d");
    std::fs::create_dir_all(&dir).unwrap();
    for f in files {
        std::fs::write(dir.join(f), "<datatest/>").unwrap();
    }
    dir
}

fn kuna_test(mode: &str, dir: &Path, extra: &[&str]) -> Output {
    std::process::Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args(["test", mode, "--binary"])
        .arg(harness())
        .arg("--datatests-dir")
        .arg(dir)
        .args(extra)
        .output()
        .expect("run kuna test")
}

fn report(out: &Output) -> Value {
    serde_json::from_slice(&out.stdout)
        .unwrap_or_else(|e| panic!("{e}: {}", String::from_utf8_lossy(&out.stdout)))
}

fn stderr(out: &Output) -> String {
    String::from_utf8_lossy(&out.stderr).into_owned()
}

#[test]
fn a_missing_name_beside_a_real_one_is_named_and_exits_2() {
    let dir = corpus(&["a.xml", "b.xml"]);
    let out = kuna_test(
        "--datatests",
        &dir,
        &["--name", "a", "--name", "gone", "--json"],
    );
    assert_eq!(out.status.code(), Some(2), "{}", stderr(&out));
    assert!(
        stderr(&out).contains(&format!(
            "error: datatest 'gone.xml' not found in {}",
            dir.display()
        )),
        "{}",
        stderr(&out)
    );
    let r = report(&out);
    assert_eq!(r["passing"], serde_json::json!(["data:a.xml"]));
    let sel = &r["selection"];
    assert_eq!(sel["complete"], false);
    assert_eq!(
        sel["data"]["requested"],
        serde_json::json!(["a.xml", "gone.xml"])
    );
    assert_eq!(sel["data"]["matched"], serde_json::json!(["a.xml"]));
    assert_eq!(sel["data"]["missing"], serde_json::json!(["gone.xml"]));
    assert_eq!(sel["errors"].as_array().unwrap().len(), 1);
    assert_eq!(sel["unit"], Value::Null);

    let text = kuna_test("--datatests", &dir, &["--name", "a", "--name", "gone"]);
    assert_eq!(text.status.code(), Some(2));
    assert!(
        String::from_utf8_lossy(&text.stdout).contains("[selection] datatest 'gone.xml' not found")
    );
}

#[test]
fn a_fully_matched_or_whole_corpus_selection_is_complete() {
    let dir = corpus(&["b.xml", "a.xml", "notes.txt"]);
    let one = kuna_test("--datatests", &dir, &["--name", "b.xml", "--json"]);
    assert_eq!(one.status.code(), Some(0), "{}", stderr(&one));
    assert_eq!(stderr(&one), "");
    let sel = &report(&one)["selection"];
    assert_eq!(sel["complete"], true);
    assert_eq!(sel["errors"], serde_json::json!([]));
    assert_eq!(sel["data"]["matched"], serde_json::json!(["b.xml"]));

    let all = kuna_test("--datatests", &dir, &["--json"]);
    assert_eq!(all.status.code(), Some(0), "{}", stderr(&all));
    let r = report(&all);
    assert_eq!(r["data_footer"], serde_json::json!([2, 2]));
    assert_eq!(r["selection"]["data"]["requested"], serde_json::json!([]));
    assert_eq!(
        r["selection"]["data"]["matched"],
        serde_json::json!(["a.xml", "b.xml"])
    );
}

#[test]
fn an_empty_or_unreadable_corpus_is_a_selection_error() {
    let empty = corpus(&["notes.txt"]);
    let absent = empty.join("absent");
    for (dir, said) in [
        (&empty, "error: no datatests (*.xml) in"),
        (&absent, "error: cannot read datatest directory"),
    ] {
        let out = kuna_test("--datatests", dir, &["--json"]);
        assert_eq!(out.status.code(), Some(2), "{}", stderr(&out));
        assert!(stderr(&out).contains(said), "{}", stderr(&out));
        let r = report(&out);
        assert_eq!(r["data_footer"], serde_json::json!([0, 0]));
        assert_eq!(r["selection"]["complete"], false);
        assert_eq!(r["selection"]["data"]["matched"], serde_json::json!([]));
    }
}

/// The harness's unit registry is empty by design: running it is a complete
/// selection, but naming a unit test it does not have is not.
#[test]
fn the_empty_unit_registry_stays_complete_until_a_name_is_asked_of_it() {
    let dir = corpus(&["a.xml"]);
    let plain = kuna_test("--unittests", &dir, &["--json"]);
    assert_eq!(plain.status.code(), Some(0), "{}", stderr(&plain));
    let r = report(&plain);
    assert_eq!(r["unit_footer"], serde_json::json!([0, 0]));
    assert_eq!(r["selection"]["complete"], true);
    assert_eq!(r["selection"]["data"], Value::Null);
    assert_eq!(r["selection"]["unit"]["matched"], serde_json::json!([]));

    let named = kuna_test("--unittests", &dir, &["--name", "nosuch", "--json"]);
    assert_eq!(named.status.code(), Some(2), "{}", stderr(&named));
    assert!(
        stderr(&named).contains("error: unit test 'nosuch' not found"),
        "{}",
        stderr(&named)
    );
    assert_eq!(
        report(&named)["selection"]["unit"]["missing"],
        serde_json::json!(["nosuch"])
    );
}

#[test]
fn an_incomplete_selection_never_records_a_baseline() {
    let dir = corpus(&["a.xml"]);
    let baseline = common::scratch_file("selection-baseline", "json");
    for extra in [&["--name", "a", "--name", "gone"][..], &[]] {
        let target = if extra.is_empty() {
            corpus(&[])
        } else {
            dir.clone()
        };
        let mut args = extra.to_vec();
        args.extend(["--save-baseline", baseline.to_str().unwrap()]);
        let out = kuna_test("--datatests", &target, &args);
        assert_eq!(out.status.code(), Some(2), "{}", stderr(&out));
        assert!(
            stderr(&out).contains("error: not writing baseline"),
            "{}",
            stderr(&out)
        );
        assert!(
            !baseline.exists(),
            "a partial run was recorded as a baseline"
        );
    }
    let out = kuna_test(
        "--datatests",
        &dir,
        &["--save-baseline", baseline.to_str().unwrap()],
    );
    assert_eq!(out.status.code(), Some(0), "{}", stderr(&out));
    assert!(baseline.exists());
}

/// The issue's own repro, through the real harness and the vendored corpus.
#[test]
fn the_real_harness_reports_the_missing_half_of_a_mixed_request() {
    let root = common::repo_root();
    let (stdout, stderr, code) = common::run_kuna(&[
        "test",
        "--datatests",
        "--datatests-dir",
        root.join("tests/datatests").to_str().unwrap(),
        "--sleighpath",
        root.join("specs").to_str().unwrap(),
        "--name",
        "forloop1",
        "--name",
        "rea_missing_selection",
        "--json",
    ]);
    assert_eq!(code, 2, "{stderr}");
    assert!(
        stderr.contains("error: datatest 'rea_missing_selection.xml' not found"),
        "{stderr}"
    );
    let r: Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(r["data_footer"], serde_json::json!([1, 1]), "{stderr}");
    assert_eq!(r["passing"], serde_json::json!(["data:For-loop #1"]));
    assert_eq!(
        r["selection"]["data"]["matched"],
        serde_json::json!(["forloop1.xml"])
    );
    assert_eq!(
        r["selection"]["data"]["missing"],
        serde_json::json!(["rea_missing_selection.xml"])
    );
}
