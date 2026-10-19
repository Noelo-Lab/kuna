//! `kuna decompile` honours a per-function wall-clock budget (#1157): the
//! text surface hands it to `decomp_dbg`, whose drive stops at the deadline
//! and names the pass it was in, instead of running unbounded.

#![cfg(unix)]

use crate::common;

use std::os::unix::fs::PermissionsExt;
use std::process::Command;
use std::time::{Duration, Instant};

fn specs() -> String {
    common::repo_root().join("specs").to_str().unwrap().to_string()
}

/// A `decomp_dbg` stand-in that records its arguments in `log` and answers
/// with a clean one-function transcript.
fn recording_stub(log: &std::path::Path) -> std::path::PathBuf {
    let path = common::scratch_file("budget-stub", "sh");
    let script = format!(
        "#!/bin/sh\n\
         printf '%s\\n' \"$@\" > '{}'\n\
         out=\n\
         while IFS= read -r line; do\n\
         \x20 case \"$line\" in 'openfile write '*) out=${{line#openfile write }} ;; esac\n\
         done\n\
         [ -n \"$out\" ] && printf 'void main(void)\\n{{\\n  return;\\n}}\\n' > \"$out\"\n\
         printf '[decomp]> decompile\\nDecompiling main\\nDecompilation complete\\n'\n",
        log.display()
    );
    std::fs::write(&path, script).unwrap();
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o755)).unwrap();
    path
}

fn engine_budget(extra: &[&str]) -> Option<String> {
    let log = common::scratch_file("budget-args", "txt");
    let stub = recording_stub(&log);
    let fauxware = common::fixture("fauxware");
    let mut args = vec!["decompile", fauxware.as_str(), "main", "--decomp-dbg", stub.to_str().unwrap()];
    args.extend_from_slice(extra);
    let out = Command::new(env!("CARGO_BIN_EXE_kuna")).args(&args).output().unwrap();
    assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
    let argv = std::fs::read_to_string(&log).unwrap();
    let _ = std::fs::remove_file(&stub);
    let _ = std::fs::remove_file(&log);
    let argv: Vec<&str> = argv.lines().collect();
    argv.iter().position(|a| *a == "--max-fn-seconds").map(|i| argv[i + 1].to_string())
}

#[test]
fn decompile_hands_its_budget_to_the_engine() {
    assert_eq!(engine_budget(&[]).as_deref(), Some("120"));
    assert_eq!(engine_budget(&["--max-fn-seconds", "7"]).as_deref(), Some("7"));
    assert_eq!(engine_budget(&["--max-fn-seconds", "0"]), None);
}

#[test]
fn decompile_refuses_a_malformed_budget() {
    let out = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args(["decompile", &common::fixture("fauxware"), "main", "--max-fn-seconds", "soon"])
        .output()
        .unwrap();
    assert!(!out.status.success());
    assert!(String::from_utf8_lossy(&out.stderr).contains("invalid --max-fn-seconds value \"soon\""));
}

/// `big` in `tests/hang-repro/escaped_locals` runs well past one second; the
/// text surface now stops it there and says which pass the time ran out in.
#[test]
fn decompile_stops_at_its_budget_and_names_the_pass() {
    let bin = common::repo_root().join("tests/hang-repro/escaped_locals");
    let start = Instant::now();
    let out = common::process::output_with_timeout(
        Command::new(env!("CARGO_BIN_EXE_kuna")).args([
            "decompile",
            bin.to_str().unwrap(),
            "big",
            "--max-fn-seconds",
            "1",
            "--max-fn-memory",
            "0",
            "--sleighpath",
            &specs(),
        ]),
        Duration::from_secs(120),
        Duration::from_millis(50),
    )
    .expect("kuna decompile did not stop within the 120 s outer bound");
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_eq!(out.status.code(), Some(1), "{stderr}");
    assert!(
        stderr.contains("decompilation failed for big")
            && stderr.contains("per-function decompile budget exceeded (1 s) in ")
            && stderr.contains("note: --max-fn-seconds N sets the budget"),
        "{stderr}"
    );
    assert!(start.elapsed() < Duration::from_secs(60), "took {:?}", start.elapsed());
}
