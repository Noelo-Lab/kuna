//! `@FILE` lists for `--functions`, `--addr` and `--entry` (issue #1978): every
//! surface that takes them reads the same selection from a file as from argv.

use crate::common;

use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

fn kuna(args: &[&str], stdin: Option<&str>) -> (String, String, i32) {
    let mut child = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args(args)
        .env("SLEIGHHOME", common::repo_root().join("specs"))
        .stdin(if stdin.is_some() { Stdio::piped() } else { Stdio::null() })
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn kuna");
    if let Some(text) = stdin {
        child.stdin.take().unwrap().write_all(text.as_bytes()).unwrap();
    }
    let out = child.wait_with_output().unwrap();
    (
        String::from_utf8(out.stdout).unwrap(),
        String::from_utf8(out.stderr).unwrap(),
        out.status.code().expect("exit code"),
    )
}

fn ok(args: &[&str], stdin: Option<&str>) -> String {
    let (stdout, stderr, code) = kuna(args, stdin);
    assert_eq!(code, 0, "{args:?}: {stderr}");
    stdout
}

fn refused(args: &[&str], stdin: Option<&str>) -> String {
    let (stdout, stderr, code) = kuna(args, stdin);
    assert_eq!(code, 2, "{args:?}: {stdout}{stderr}");
    stderr
}

/// A list file and the `@path` value naming it.
fn list(text: &str) -> (PathBuf, String) {
    let path = common::scratch_file("selection-list", "txt");
    std::fs::write(&path, text).unwrap();
    let value = format!("@{}", path.display());
    (path, value)
}

fn with<'a>(head: &[&'a str], tail: &[&'a str]) -> Vec<&'a str> {
    head.iter().chain(tail).copied().collect()
}

#[test]
fn decompile_all_reads_names_and_addresses_from_a_file() {
    let bin = common::fixture("fauxware");
    let run = ["decompile-all", bin.as_str(), "--json"];

    let names = ok(&with(&run, &["--functions", "main,authenticate"]), None);
    assert!(names.contains("\"name\": \"authenticate\"") || names.contains("\"name\":\"authenticate\""), "{names}");
    let (_keep, file) = list("# picked by hand\nmain\n\n  authenticate  # the checker\n");
    assert_eq!(ok(&with(&run, &["--functions", &file]), None), names);
    assert_eq!(ok(&with(&run, &["--functions", "main", "--functions", "authenticate"]), None), names);

    let addrs = ok(&with(&run, &["--addr", "0x40071d", "--addr", "0x400664"]), None);
    let (_keep, file) = list("0x40071d\n400664   # bare hex, as --addr takes it\n");
    assert_eq!(ok(&with(&run, &["--addr", &file]), None), addrs);
    let (_keep, half) = list("0x400664\n");
    assert_eq!(ok(&with(&run, &["--addr", "0x40071d", "--addr", &half]), None), addrs);
    assert_eq!(ok(&with(&run, &["--addr", "@-"]), Some("0x40071d\n0x400664\n")), addrs);
}

/// The issue's invocation: the file `--define-function` reads is also the
/// `--addr` list, and the export is the one the same flags written out make,
/// its `.asm` scoped to the two declared functions.
#[test]
fn decompile_project_takes_its_selection_from_the_bounds_file() {
    let bin = common::fixture("fauxware");
    let (_keep, bounds) = list(
        "# worked out by hand\n0x40071d-0x4007e0 = main\n0x400664-0x4006ed=authenticate\n",
    );
    let export = |tag: &str, flags: &[&str]| -> PathBuf {
        let dir = common::scratch_file(tag, "dir");
        let out = dir.to_str().unwrap().to_string();
        ok(&with(&["decompile-project", bin.as_str(), "-o", &out], flags), None);
        dir
    };
    let inline = export(
        "selection-inline",
        &[
            "--define-function", "0x40071d-0x4007e0=main",
            "--define-function", "0x400664-0x4006ed=authenticate",
            "--addr", "0x40071d", "--addr", "0x400664",
        ],
    );
    let listed = export("selection-listed", &["--define-function", &bounds, "--addr", &bounds]);
    let read = |dir: &Path, name: &str| std::fs::read_to_string(dir.join(name)).unwrap();
    for artifact in ["fauxware.c", "fauxware.h", "fauxware.asm", "README.md"] {
        assert_eq!(read(&listed, artifact), read(&inline, artifact), "{artifact}");
    }
    let asm = read(&listed, "fauxware.asm");
    let blocks: Vec<&str> = asm.lines().filter(|line| line.starts_with("; --- code ")).collect();
    assert_eq!(
        blocks,
        ["; --- code range 0x400664..0x4006ed ---", "; --- code range 0x40071d..0x4007e0 ---"]
    );
    for dir in [inline, listed] {
        let _ = std::fs::remove_dir_all(dir);
    }
}

#[test]
fn decompile_graph_reads_its_names_from_a_file() {
    let bin = common::fixture("fauxware");
    let run = ["decompile-graph", bin.as_str()];
    let (_keep, file) = list("main\nauthenticate\n");
    assert_eq!(
        ok(&with(&run, &["--functions", &file]), None),
        ok(&with(&run, &["--functions", "main,authenticate"]), None)
    );
}

/// A raw image's `--entry` seeds come from a file the same way: two `ret`s of
/// AArch64 at 0x10000 and 0x10004.
#[test]
fn raw_image_entries_read_from_a_file() {
    let image = common::scratch_file("selection-raw", "bin");
    std::fs::write(&image, [0xc0, 0x03, 0x5f, 0xd6, 0xc0, 0x03, 0x5f, 0xd6]).unwrap();
    let image = image.to_str().unwrap().to_string();
    let run = [
        "decompile-all", image.as_str(), "--raw-image", "--target", "AARCH64:LE:64:v8A",
        "--base", "0x10000", "--json",
    ];
    let inline = ok(&with(&run, &["--entry", "0x10000", "--entry", "0x10004"]), None);
    let (_keep, file) = list("0x10000\n0x10004\n");
    assert_eq!(ok(&with(&run, &["--entry", &file]), None), inline);
}

#[test]
fn fid_query_reads_its_addresses_from_a_file() {
    let root = common::repo_root();
    let binary = root.join("tests/fixtures/fid/prog");
    let db = root.join("tests/fixtures/fid/lib.fid");
    let run = ["fid", "query", binary.to_str().unwrap(), "--db", db.to_str().unwrap(), "--json"];
    let inline = ok(&with(&run, &["--addr", "0x4017c0", "--addr", "0x401810"]), None);
    let (_keep, file) = list("0x4017c0  # kuna_crc32\n401810\n");
    assert_eq!(ok(&with(&run, &["--addr", &file]), None), inline);
    let (path, file) = list("0x4017c0\nnope\n");
    let stderr = refused(&with(&run, &["--addr", &file]), None);
    assert!(stderr.contains(&format!("--addr @{}:2: invalid address \"nope\"", path.display())), "{stderr}");
}

#[test]
fn a_list_that_cannot_be_read_or_parsed_is_a_usage_error() {
    let bin = common::fixture("fauxware");
    for command in ["decompile-all", "decompile-project", "decompile-graph"] {
        let run = [command, bin.as_str()];
        let stderr = refused(&with(&run, &["--functions", "@/nonexistent/kuna-names.txt"]), None);
        assert!(stderr.contains("error: --functions @/nonexistent/kuna-names.txt: "), "{stderr}");
        assert!(stderr.contains("written @@/nonexistent/kuna-names.txt"), "{stderr}");

        let (_keep, empty) = list("# nothing selected\n\n");
        let stderr = refused(&with(&run, &["--addr", &empty]), None);
        assert!(stderr.contains(&format!("error: --addr {empty}: lists no entries")), "{stderr}");

        let (path, bad) = list("0x40071d\nmain\n");
        let stderr = refused(&with(&run, &["--addr", &bad]), None);
        assert!(
            stderr.contains(&format!("error: --addr @{}:2: invalid address \"main\"", path.display())),
            "{stderr}"
        );

        let stderr = refused(&with(&run, &["--addr", "@-", "--functions", "@-"]), Some("0x40071d\n"));
        assert!(stderr.contains("error: --functions @-: standard input already feeds an earlier @-"), "{stderr}");
    }
}

/// `@@` spells a name that starts with `@`, and `kuna decompile`'s one target is
/// a name even when it starts with `@`: neither reads a file.
#[test]
fn a_name_that_starts_with_at_is_not_a_file() {
    let bin = common::fixture("fauxware");
    let (_, stderr, _) = kuna(&["decompile-all", &bin, "--functions", "@@main", "--json"], None);
    assert!(stderr.contains("no function named \"@main\""), "{stderr}");
    for json in [&[][..], &["--json"]] {
        let (_, stderr, _) = kuna(&with(&["decompile", &bin, "@main"], json), None);
        assert!(stderr.contains("\"@main\""), "{json:?}: {stderr}");
        assert!(!stderr.contains("No such file"), "{json:?}: {stderr}");
    }
}
