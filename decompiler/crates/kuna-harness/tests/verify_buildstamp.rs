//! The child half of the build handshake on the real `decomp_test_dbg`, whose
//! stderr also carries the unit-test grammar `kuna test` parses.

use std::process::Command;

use kuna_console::kuna_buildstamp::{identity, ChildKind, PARENT_ENV, REPLY};

fn run(parent: Option<&str>) -> String {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_decomp_test_dbg"));
    cmd.args(["-sleighpath", "/nonexistent-kuna-specs", "unittests"])
        .env_remove(PARENT_ENV);
    if let Some(id) = parent {
        cmd.env(PARENT_ENV, id);
    }
    let out = cmd.output().expect("spawn decomp_test_dbg");
    String::from_utf8_lossy(&out.stderr).into_owned()
}

#[test]
fn decomp_test_dbg_answers_only_a_parent_of_another_build() {
    let other = run(Some("1.329 (source 0123456789abcdef)"));
    let quiet = run(Some(identity(ChildKind::Harness)));
    assert_eq!(
        other.lines().next(),
        Some(format!("{REPLY}{}", identity(ChildKind::Harness)).as_str()),
        "{other}"
    );
    assert_eq!(
        other.lines().skip(1).collect::<Vec<_>>(),
        quiet.lines().collect::<Vec<_>>()
    );
    assert!(
        !quiet.contains(REPLY) && !run(None).contains(REPLY),
        "{quiet}"
    );
}
