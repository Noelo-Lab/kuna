#![cfg(unix)]

use std::os::unix::fs::PermissionsExt;
use std::process::{Command, Output};

mod common;

fn run(document: &str, save_first: bool) -> Output {
    let baseline = common::scratch_file("baseline", "json");
    let harness = common::scratch_file("passing-harness", "sh");
    std::fs::write(&baseline, document).unwrap();
    std::fs::write(
        &harness,
        "#!/bin/sh\nprintf 'Success -- example\\nTotal tests applied = 1\\nTotal passing tests = 1\\n'\n",
    )
    .unwrap();
    std::fs::set_permissions(&harness, std::fs::Permissions::from_mode(0o700)).unwrap();
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_kuna"));
    cmd.args(["test", "--datatests", "--binary"])
        .arg(&harness)
        .arg("--baseline")
        .arg(&baseline);
    if save_first {
        cmd.arg("--save-baseline").arg(&baseline);
    }
    cmd.output().expect("run parity command")
}

#[test]
fn malformed_baselines_cannot_report_parity() {
    for document in [
        "{}",
        "null",
        r#"{"passing":"data:example"}"#,
        r#"{"passing":["data:example",1]}"#,
        r#"{"passing":[],"passing":["data:example"]}"#,
        r#"{"passing":["data:example"]} false"#,
    ] {
        let out = run(document, false);
        let stderr = String::from_utf8(out.stderr).unwrap();
        assert_eq!(out.status.code(), Some(2), "{document}: {stderr}");
        assert!(stderr.contains("could not parse baseline JSON"), "{stderr}");
        assert!(
            out.stdout.is_empty(),
            "invalid baseline produced a parity report"
        );
    }
}

#[test]
fn valid_and_explicitly_empty_baselines_still_pass() {
    for document in [r#"{"passing":[]}"#, r#"{"passing":["data:example"]}"#] {
        let out = run(document, false);
        assert!(
            out.status.success(),
            "{}",
            String::from_utf8_lossy(&out.stderr)
        );
        assert!(String::from_utf8(out.stdout).unwrap().contains("PARITY OK"));
    }
}

#[test]
fn missing_expected_passkeys_remain_regressions() {
    let out = run(r#"{"passing":["data:missing"]}"#, false);
    assert_eq!(out.status.code(), Some(1));
    assert!(String::from_utf8(out.stdout)
        .unwrap()
        .contains("PARITY FAIL"));
}

#[test]
fn saving_and_comparing_the_same_path_uses_the_new_record() {
    let out = run("{}", true);
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(String::from_utf8(out.stdout).unwrap().contains("PARITY OK"));
}
