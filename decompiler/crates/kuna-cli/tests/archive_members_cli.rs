//! `ar` archive inputs through the built CLI: member listing, selection and
//! the symbol-driven member lookup.
//!
//! `fixtures/archive_members/libm2.a` holds `sq.o`, a text member `notes.txt`
//! and `cube.o`, built from the sources beside it (clang 22.1.8, x86-64 Linux):
//! `printf 'not an object\n' > notes.txt && clang -O2 -c sq.c cube.c &&
//! llvm-ar rcsD libm2.a sq.o notes.txt cube.o`.

use std::path::{Path, PathBuf};
use std::process::Command;

use crate::common;

fn archive() -> String {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/archive_members/libm2.a")
        .to_string_lossy()
        .into_owned()
}

fn member(name: &[u8]) -> Vec<u8> {
    let bytes = std::fs::read(archive()).unwrap();
    let parsed = object::read::archive::ArchiveFile::parse(&*bytes).unwrap();
    let found = parsed.members().map(|m| m.unwrap()).find(|m| m.name() == name).unwrap();
    found.data(&*bytes).unwrap().to_vec()
}

/// Write a GNU archive of `members` (short names, `/`-terminated); a thin
/// archive records each member's size but not its bytes.
fn write_archive(path: &Path, magic: &str, members: &[(&str, &[u8])]) {
    let mut bytes = magic.as_bytes().to_vec();
    for (name, data) in members {
        let size = data.len();
        bytes.extend(format!("{name:<16}{:<12}{:<6}{:<6}{:<8}{size:<10}`\n", 0, 0, 0, 644).bytes());
        if magic == "!<arch>\n" {
            bytes.extend(*data);
            if size % 2 == 1 {
                bytes.push(b'\n');
            }
        }
    }
    std::fs::write(path, bytes).unwrap();
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
    let extracted = scratch.path().join("cube.o");
    std::fs::write(&extracted, member(b"cube.o")).unwrap();
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

#[test]
fn blobs_reads_a_selected_member() {
    let dir = tempfile::tempdir().unwrap();
    let image = common::repo_root().join("decompiler/crates/kuna-analysis/tests/fixtures/embedded_blobs_x86_64.so");
    let lib = dir.path().join("libb.a");
    write_archive(&lib, "!<arch>\n", &[("blobs.so/", &std::fs::read(image).unwrap())]);
    let (stdout, stderr, code) = kuna(&["blobs", &format!("{}:blobs.so", lib.display()), "--no-xrefs"]);
    assert_eq!(code, 0, "{stderr}");
    assert!(stdout.starts_with("# 5 embedded blobs in "), "{stdout}");
    assert!(stdout.contains("0x3c0\tpng\t69\t.rodata"), "{stdout}");
}

#[test]
fn a_name_stored_twice_is_selected_by_its_occurrence() {
    let scratch = tempfile::tempdir().unwrap();
    let lib = scratch.path().join("libdup.a");
    write_archive(&lib, "!<arch>\n", &[("x.o/", &member(b"sq.o")), ("x.o/", &member(b"cube.o"))]);
    let lib = lib.to_str().unwrap();

    let (stdout, stderr, code) = kuna(&["functions", lib]);
    assert_eq!(code, 0, "{stderr}");
    assert_eq!(stdout, "0x400000\tx.o#1:sq\n0x400000\tx.o#2:cube\n");

    let (_, stderr, code) = kuna(&["decompile", lib, "--member", "x.o", "sq"]);
    assert_eq!(code, 2);
    assert!(stderr.contains("2 members are named x.o; select one as x.o#1, x.o#2"), "{stderr}");

    let (stdout, stderr, code) = kuna(&["decompile", &format!("{lib}:x.o#2"), "cube"]);
    assert_eq!(code, 0, "{stderr}");
    assert!(stdout.contains("return a0 * a0 * a0;"), "{stdout}");
}

#[test]
fn an_existing_path_with_a_colon_is_not_split() {
    let scratch = tempfile::tempdir().unwrap();
    std::fs::copy(archive(), scratch.path().join("lib.a")).unwrap();
    let real = scratch.path().join("lib.a:sq.o");
    std::fs::write(&real, member(b"cube.o")).unwrap();
    let (stdout, stderr, code) = kuna(&["functions", real.to_str().unwrap()]);
    assert_eq!(code, 0, "{stderr}");
    assert_eq!(stdout, "0x400000\tcube\n");
}

#[test]
fn a_thin_archive_reads_only_regular_member_files() {
    let scratch = tempfile::tempdir().unwrap();
    let cube = member(b"cube.o");
    std::fs::write(scratch.path().join("cube.o"), &cube).unwrap();
    let lib = scratch.path().join("libthin.a");
    write_archive(&lib, "!<thin>\n", &[("cube.o/", &cube)]);
    let (stdout, stderr, code) = kuna(&["functions", lib.to_str().unwrap()]);
    assert_eq!(code, 0, "{stderr}");
    assert_eq!(stdout, "0x400000\tcube.o:cube\n");

    write_archive(&lib, "!<thin>\n", &[("cube.o/", &cube), ("./", b"")]);
    let (_, stderr, code) = kuna(&["functions", lib.to_str().unwrap(), "--member"]);
    assert_eq!(code, 2);
    assert!(stderr.contains("is not a regular file"), "{stderr}");
}
