//! A downstream reader that closes the pipe is a normal terminal condition, not
//! the `println!` panic (exit 101) every stdout-writing subcommand used to take.
//!
//! Two properties, and the second is the one that is easy to get backwards: the
//! panic is gone, and the command's own verdict still reaches the caller.  A
//! failing run must not go green just because nobody was reading it.

use std::io::Read;
use std::path::PathBuf;
use std::process::{Command, Stdio};

fn run_with_closed_stdout(args: &[&str]) -> std::process::Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn kuna");

    let mut stdout = child.stdout.take().expect("stdout pipe");
    let mut prefix = [0u8; 64];
    stdout.read_exact(&mut prefix).expect("read output prefix");
    drop(stdout);

    child.wait_with_output().expect("wait for kuna")
}

/// Close the read end before the child writes anything, so the very first write
/// takes the EPIPE — no dependence on the payload outgrowing the pipe buffer.
fn run_with_stdout_closed_immediately(args: &[&str]) -> std::process::Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn kuna");
    drop(child.stdout.take().expect("stdout pipe"));
    child.wait_with_output().expect("wait for kuna")
}

fn assert_no_panic(output: &std::process::Output) {
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !stderr.contains("panicked"),
        "broken pipe triggered a panic: {stderr}"
    );
    assert!(
        !stderr.contains("Broken pipe"),
        "broken pipe leaked to stderr: {stderr}"
    );
}

fn assert_quiet_success(output: &std::process::Output) {
    assert_no_panic(output);
    assert!(
        output.status.success(),
        "closed stdout status: {:?}; {}",
        output.status.code(),
        String::from_utf8_lossy(&output.stderr)
    );
}

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .unwrap()
}

#[test]
fn closed_stdout_is_a_quiet_terminal_condition() {
    assert_quiet_success(&run_with_closed_stdout(&["catalog", "--markdown"]));
}

#[test]
fn functions_json_tolerates_a_closed_reader() {
    let root = repo_root();
    let specs = root.join("specs");
    if !specs
        .join("Ghidra/Processors/x86/data/languages/x86-64.sla")
        .exists()
    {
        eprintln!("broken_pipe: skipping functions test (no built x86-64.sla)");
        return;
    }
    let binary = root.join("tests/hang-repro/ssh-sk-helper");
    let output = run_with_closed_stdout(&[
        "functions",
        binary.to_str().unwrap(),
        "--json",
        "--sleighpath",
        specs.to_str().unwrap(),
        "--mode",
        "fast",
    ]);
    assert_quiet_success(&output);
}

/// A closed pipe must not turn a red gate green.  `kuna test` scores the stages
/// corpus against the *main* corpus baseline: the two record disjoint passkeys by
/// construction, so every baseline entry reads as REGRESSED and the run is a
/// guaranteed exit 1 — which has to survive nobody reading the report.
#[test]
fn a_closed_reader_does_not_turn_a_failing_gate_green() {
    let root = repo_root();
    let output = run_with_stdout_closed_immediately(&[
        "test",
        "--datatests",
        "--datatests-dir",
        root.join("tests/stages").to_str().unwrap(),
        "--baseline",
        root.join("docs/baseline.json").to_str().unwrap(),
        "--name",
        "kuna-catalog",
    ]);
    assert_no_panic(&output);
    assert_eq!(
        output.status.code(),
        Some(1),
        "a REGRESSED parity run reported {:?} once its reader closed; {}",
        output.status.code(),
        String::from_utf8_lossy(&output.stderr)
    );
}
