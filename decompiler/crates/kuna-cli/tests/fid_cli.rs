//! FID archive ingestion through the built CLI, without an external archive tool.

use std::process::Command;

use crate::common;

fn archive_member(archive: &mut Vec<u8>, name: &str, bytes: &[u8]) {
    let header = format!(
        "{name:<16}{:<12}{:<6}{:<6}{:<8}{:<10}`\n",
        0,
        0,
        0,
        "100644",
        bytes.len()
    );
    assert_eq!(header.len(), 60);
    archive.extend_from_slice(header.as_bytes());
    archive.extend_from_slice(bytes);
    if bytes.len() % 2 != 0 {
        archive.push(b'\n');
    }
}

#[test]
fn archive_members_keep_the_vendored_database_and_leave_no_staging_files() {
    let scratch = tempfile::tempdir().unwrap();
    let staging = scratch.path().join("staging");
    std::fs::create_dir(&staging).unwrap();
    let object = std::fs::read(common::fixture("fid_lib_x86_64.o")).unwrap();
    let expected = std::fs::read(common::fixture("fid_lib_x86_64.fid")).unwrap();
    let mut archive = b"!<arch>\n".to_vec();
    archive_member(&mut archive, "first.o/", &object);
    archive_member(&mut archive, "second.o/", &object);
    archive_member(&mut archive, "readme/", b"not an object");
    let input = scratch.path().join("library.a");
    let output = scratch.path().join("library.fid");
    std::fs::write(&input, archive).unwrap();
    let result = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args(["fid", "build"])
        .arg(&input)
        .arg("-o")
        .arg(&output)
        .args(["--lang", "x86:LE:64:default", "--cspec", "gcc"])
        .env("SLEIGHHOME", common::repo_root().join("specs"))
        .env("TMPDIR", &staging)
        .env("TMP", &staging)
        .env("TEMP", &staging)
        .output()
        .expect("run kuna fid build");
    assert_eq!(
        result.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    assert!(result.stdout.is_empty());
    assert!(String::from_utf8_lossy(&result.stderr).contains("6 record(s)"));
    assert_eq!(std::fs::read(output).unwrap(), expected);
    assert!(std::fs::read_dir(staging).unwrap().next().is_none());
}
