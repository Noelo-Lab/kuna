use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::process::Command;
use std::time::Duration;

use super::{common, process};

struct ProbeDir(PathBuf);

impl Drop for ProbeDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

enum Probe {
    Missing,
    Failed,
    Unusable,
}

fn check(probe: Probe) {
    let directory = ProbeDir(common::scratch_file("compiler-probe", "dir"));
    std::fs::create_dir(&directory.0).unwrap();
    if !matches!(probe, Probe::Missing) {
        for name in ["cc", "gcc", "clang"] {
            let path = directory.0.join(name);
            if matches!(probe, Probe::Failed) {
                let fixture = common::repo_root()
                    .join("decompiler/crates/kuna-cli/tests/common/failed-compiler.sh");
                std::os::unix::fs::symlink(fixture, path).unwrap();
            } else {
                std::fs::write(&path, b"#!/bin/sh\nexit 7\n").unwrap();
                std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o600)).unwrap();
            }
        }
    }
    for test in [
        "a_float_pointee_keeps_the_callers_integer_stores_round_trip",
        "an_implied_cast_round_trips_through_the_printed_c",
        "a_float_register_return_and_a_read_void_result_round_trip",
        "a_nan_returned_in_s0_round_trips",
        "a_wrapper_returns_its_callees_result_round_trip",
    ] {
        let output = process::output_with_timeout(
            Command::new(std::env::current_exe().unwrap())
                .args(["--exact", test, "--nocapture"])
                .env("PATH", &directory.0),
            Duration::from_secs(60),
            Duration::from_millis(10),
        )
        .expect("the isolated round-trip check timed out");
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stdout.contains("running 1 test"),
            "{test}: {stdout}\n{stderr}"
        );
        match probe {
            Probe::Missing => assert!(output.status.success(), "{test}: {stdout}\n{stderr}"),
            Probe::Failed | Probe::Unusable => {
                assert!(
                    !output.status.success(),
                    "{test} accepted a broken compiler:\n{stdout}\n{stderr}"
                );
                let message = if matches!(probe, Probe::Failed) {
                    "exit status: 7"
                } else {
                    "cannot run"
                };
                assert!(stderr.contains(message), "{test}: {stdout}\n{stderr}");
            }
        }
    }
}

#[test]
fn missing_compilers_leave_the_spelling_checks_enabled() {
    check(Probe::Missing);
}

#[test]
fn failed_compiler_probes_cannot_skip_round_trips() {
    check(Probe::Failed);
}

#[test]
fn unusable_compilers_are_not_reported_as_missing() {
    check(Probe::Unusable);
}
