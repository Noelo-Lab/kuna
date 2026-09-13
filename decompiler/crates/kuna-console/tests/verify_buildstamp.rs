//! The child half of the build handshake on the real `decomp_dbg`: it answers a
//! parent of another build with one stderr line carrying its own identity, and
//! says nothing to a matching parent or to a parent that never asked.

use std::io::Write;
use std::process::{Command, Stdio};

use kuna_console::kuna_buildstamp::{identity, PARENT_ENV, REPLY};

fn run(parent: Option<&str>) -> (String, String) {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_decomp_dbg"));
    cmd.args(["-s", "/nonexistent-kuna-specs"])
        .env_remove(PARENT_ENV)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    if let Some(id) = parent {
        cmd.env(PARENT_ENV, id);
    }
    let mut child = cmd.spawn().expect("spawn decomp_dbg");
    child
        .stdin
        .take()
        .expect("stdin")
        .write_all(b"quit\n")
        .expect("write the script");
    let out = child.wait_with_output().expect("wait for decomp_dbg");
    (
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

#[test]
fn decomp_dbg_answers_a_parent_of_another_build() {
    let (stdout, stderr) = run(Some("1.329 (source 0123456789abcdef)"));
    assert_eq!(
        stderr.lines().next(),
        Some(format!("{REPLY}{}", identity()).as_str()),
        "{stderr}"
    );
    assert!(
        !stdout.contains(REPLY),
        "the reply must never reach stdout: {stdout}"
    );
}

#[test]
fn decomp_dbg_is_silent_to_a_matching_parent_and_to_no_parent() {
    for parent in [Some(identity()), None] {
        let (stdout, stderr) = run(parent);
        assert!(
            !stderr.contains(REPLY) && !stdout.contains(REPLY),
            "{parent:?}: {stderr}"
        );
    }
}
