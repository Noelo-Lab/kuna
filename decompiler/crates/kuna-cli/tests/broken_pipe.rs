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

fn assert_quiet_success(output: &std::process::Output) {
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        output.status.success(),
        "closed stdout status: {:?}; {stderr}",
        output.status.code()
    );
    assert!(
        !stderr.contains("panicked"),
        "broken pipe triggered a panic: {stderr}"
    );
    assert!(
        !stderr.contains("Broken pipe"),
        "broken pipe leaked to stderr: {stderr}"
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
