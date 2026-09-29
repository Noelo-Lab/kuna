//! Process handling shared by CLI integration tests.

use std::io::{ErrorKind, Read};
use std::process::{Command, Output, Stdio};
use std::thread;
use std::time::{Duration, Instant};

/// A required command must exist and exit successfully.
pub fn required_output(command: &mut Command) -> Output {
    optional_output(command).unwrap_or_else(|| panic!("required command not found: {command:?}"))
}

/// Treat a spawn NotFound as optional; require successful execution otherwise.
pub fn optional_output(command: &mut Command) -> Option<Output> {
    match command.output() {
        Ok(output) => {
            assert!(
                output.status.success(),
                "{command:?} failed ({}):\n{}",
                output.status,
                String::from_utf8_lossy(&output.stderr)
            );
            Some(output)
        }
        Err(error) if error.kind() == ErrorKind::NotFound => None,
        Err(error) => panic!("cannot run {command:?}: {error}"),
    }
}

/// Drain both pipes while polling; kill and reap the child when its cap expires.
pub fn output_with_timeout(
    command: &mut Command,
    cap: Duration,
    poll_interval: Duration,
) -> Option<Output> {
    let mut child = command
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap_or_else(|error| panic!("cannot run {command:?}: {error}"));
    let mut stdout = child.stdout.take().expect("stdout piped");
    let mut stderr = child.stderr.take().expect("stderr piped");
    let stdout = thread::spawn(move || {
        let mut bytes = Vec::new();
        stdout.read_to_end(&mut bytes).expect("read child stdout");
        bytes
    });
    let stderr = thread::spawn(move || {
        let mut bytes = Vec::new();
        stderr.read_to_end(&mut bytes).expect("read child stderr");
        bytes
    });
    let deadline = Instant::now() + cap;
    let status = loop {
        match child.try_wait().expect("poll child status") {
            Some(status) => break Some(status),
            None if Instant::now() >= deadline => {
                child.kill().expect("kill timed-out child");
                child.wait().expect("reap timed-out child");
                break None;
            }
            None => thread::sleep(poll_interval),
        }
    };
    let stdout = stdout.join().expect("stdout reader");
    let stderr = stderr.join().expect("stderr reader");
    status.map(|status| Output {
        status,
        stdout,
        stderr,
    })
}
