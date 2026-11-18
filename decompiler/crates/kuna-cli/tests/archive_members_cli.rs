//! `ar` archive inputs through the built CLI: member listing, selection and
//! the symbol-driven member lookup.
//!
//! `fixtures/archive_members/libm2.a` holds `sq.o`, a text member `notes.txt`
//! and `cube.o`, built from the sources beside it:
//! `clang -O2 -c sq.c cube.c && llvm-ar rcsD libm2.a sq.o notes.txt cube.o`.

use std::path::PathBuf;
use std::process::Command;

use crate::common;

fn archive() -> String {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/archive_members/libm2.a")
        .to_string_lossy()
        .into_owned()
}

fn kuna(args: &[&str]) -> (String, String, i32) {
    let output = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args(args)
        .env("SLEIGHHOME", common::repo_root().join("specs"))
        .output()
        .expect("spawn kuna");
    (
        String::from_utf8(output.stdout).expect("UTF-8 stdout"),
        String::from_utf8(output.stderr).expect("UTF-8 stderr"),
        output.status.code().expect("kuna exited without a signal"),
    )
}

#[test]
fn functions_lists_every_object_member_under_its_name() {
    let (stdout, stderr, code) = kuna(&["functions", &archive()]);
    assert_eq!(code, 0, "{stderr}");
    assert_eq!(stdout, "0x400000\tsq.o:sq\n0x400000\tcube.o:cube\n");

    let (stdout, stderr, code) = kuna(&["functions", &archive(), "--json"]);
    assert_eq!(code, 0, "{stderr}");
    let doc: serde_json::Value = serde_json::from_str(&stdout).expect("one JSON document");
    let members = doc["members"].as_array().unwrap();
    let named: Vec<_> = members
        .iter()
        .map(|m| (m["member"].as_str().unwrap(), m["functions"]["functions"][0]["name"].as_str().unwrap()))
        .collect();
    assert_eq!(named, [("sq.o", "sq"), ("cube.o", "cube")]);
}

#[test]
fn member_without_a_name_lists_the_members() {
    let (stdout, stderr, code) = kuna(&["functions", &archive(), "--member"]);
    assert_eq!(code, 0, "{stderr}");
    let names: Vec<_> = stdout.lines().map(|l| l.split('\t').next().unwrap()).collect();
    assert_eq!(names, ["sq.o", "notes.txt", "cube.o"]);

    let (stdout, _, code) = kuna(&["decompile", &archive(), "--member", "--json"]);
    assert_eq!(code, 0);
    let doc: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    let objects: Vec<_> = doc["members"]
        .as_array()
        .unwrap()
        .iter()
        .map(|m| m["object"].as_bool().unwrap())
        .collect();
    assert_eq!(objects, [true, false, true]);
}

#[test]
fn a_symbol_selects_the_member_that_defines_it() {
    let scratch = tempfile::tempdir().unwrap();
    let bytes = std::fs::read(archive()).unwrap();
    let parsed = object::read::archive::ArchiveFile::parse(&*bytes).unwrap();
    let cube = parsed
        .members()
        .map(|m| m.unwrap())
        .find(|m| m.name() == b"cube.o")
        .unwrap();
    let extracted = scratch.path().join("cube.o");
    std::fs::write(&extracted, cube.data(&*bytes).unwrap()).unwrap();
    let (expected, stderr, code) = kuna(&["decompile", extracted.to_str().unwrap(), "cube"]);
    assert_eq!(code, 0, "{stderr}");
    assert!(expected.contains("return a0 * a0 * a0;"), "{expected}");

    let (found, stderr, code) = kuna(&["decompile", &archive(), "cube"]);
    assert_eq!(code, 0, "{stderr}");
    assert_eq!(found, expected);
    assert!(stderr.contains("using member cube.o"), "{stderr}");

    for selected in [
        kuna(&["decompile", &archive(), "--member", "cube.o", "cube"]),
        kuna(&["decompile", &format!("{}:cube.o", archive()), "cube"]),
        kuna(&["decompile", "--member=cube.o", &archive(), "cube"]),
    ] {
        assert_eq!(selected.2, 0, "{}", selected.1);
        assert_eq!(selected.0, expected);
    }
}

#[test]
fn an_unselected_member_is_refused_with_the_member_names() {
    let (stdout, stderr, code) = kuna(&["decompile-all", &archive()]);
    assert_eq!(code, 2);
    assert!(stdout.is_empty());
    assert!(stderr.contains("--member NAME (sq.o, cube.o)"), "{stderr}");

    let (_, stderr, code) = kuna(&["decompile", &archive(), "--member", "absent.o", "sq"]);
    assert_eq!(code, 2);
    assert!(stderr.contains("no member named absent.o"), "{stderr}");

    let object = common::fixture("entry_selectors_x86_64.o");
    let (_, stderr, code) = kuna(&["functions", &object, "--member", "sq.o"]);
    assert_eq!(code, 2);
    assert!(stderr.contains("--member applies to an ar archive input"), "{stderr}");
}

#[test]
fn a_selected_member_leaves_no_staging_files() {
    let staging = tempfile::tempdir().unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args(["decompile", &archive(), "sq"])
        .env("SLEIGHHOME", common::repo_root().join("specs"))
        .env("TMPDIR", staging.path())
        .env("TMP", staging.path())
        .env("TEMP", staging.path())
        .output()
        .expect("spawn kuna");
    assert_eq!(output.status.code(), Some(0));
    assert!(String::from_utf8_lossy(&output.stdout).contains("return a0 * a0;"));
    assert_eq!(std::fs::read_dir(staging.path()).unwrap().count(), 0);
}
