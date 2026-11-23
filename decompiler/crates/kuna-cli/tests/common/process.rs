//! Process handling shared by CLI integration tests.

use std::collections::HashMap;
use std::io::{self, ErrorKind, Read};
use std::path::Path;
use std::process::{Child, Command, ExitStatus, Output, Stdio};
use std::sync::{Mutex, OnceLock};
use std::thread;
use std::time::{Duration, Instant};

/// The cap every test-spawned process runs under unless it names its own.
pub const DEFAULT_CAP: Duration = Duration::from_secs(300);

const ASAN_PROBE_CAP: Duration = Duration::from_secs(30);

/// A required command must exist and exit successfully.
pub fn required_output(command: &mut Command) -> Output {
    optional_output(command).unwrap_or_else(|| panic!("required command not found: {command:?}"))
}

/// Treat a spawn NotFound as optional; require successful execution otherwise.
pub fn optional_output(command: &mut Command) -> Option<Output> {
    match command.capped_output() {
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
    let child = command
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap_or_else(|error| panic!("cannot run {command:?}: {error}"));
    collect(child, cap, poll_interval)
}

/// `Command::output`/`Command::status` with a kill-and-reap cap, so a child that
/// never exits fails its test instead of stalling the suite.
pub trait Capped {
    fn capped_output(&mut self) -> io::Result<Output>;
    fn capped_status(&mut self) -> io::Result<ExitStatus>;
}

impl Capped for Command {
    fn capped_output(&mut self) -> io::Result<Output> {
        let child = self
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;
        collect(child, DEFAULT_CAP, Duration::from_millis(5))
            .ok_or_else(|| timed_out(self, DEFAULT_CAP))
    }

    fn capped_status(&mut self) -> io::Result<ExitStatus> {
        let mut child = self.spawn()?;
        reap(&mut child, DEFAULT_CAP, Duration::from_millis(5))
            .ok_or_else(|| timed_out(self, DEFAULT_CAP))
    }
}

fn timed_out(command: &Command, cap: Duration) -> io::Error {
    io::Error::new(
        ErrorKind::TimedOut,
        format!("{command:?} was killed after running {cap:?}"),
    )
}

fn collect(mut child: Child, cap: Duration, poll_interval: Duration) -> Option<Output> {
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
    let status = reap(&mut child, cap, poll_interval);
    let stdout = stdout.join().expect("stdout reader");
    let stderr = stderr.join().expect("stderr reader");
    status.map(|status| Output {
        status,
        stdout,
        stderr,
    })
}

fn reap(child: &mut Child, cap: Duration, poll_interval: Duration) -> Option<ExitStatus> {
    let deadline = Instant::now() + cap;
    let mut delay = Duration::from_millis(1).min(poll_interval);
    loop {
        match child.try_wait().expect("poll child status") {
            Some(status) => return Some(status),
            None if Instant::now() >= deadline => {
                child.kill().expect("kill timed-out child");
                child.wait().expect("reap timed-out child");
                return None;
            }
            None => {
                thread::sleep(delay);
                delay = (delay * 2).min(poll_interval);
            }
        }
    }
}

/// Whether a trivial program `compiler` builds with `flags`, which carry its
/// `-fsanitize=` selection, gets past start-up; probed once per process. Some
/// ASan runtimes (Apple clang 17 on Darwin 25) hang before `main`, so such a
/// compiler is as unusable for a sanitizer round trip as a missing one. Only
/// that hang skips: a build that fails or a program that exits non-zero keeps
/// the round trip, which then reports the failure itself.
pub fn sanitizer_runs(compiler: &str, flags: &[&str]) -> bool {
    static PROBES: OnceLock<Mutex<HashMap<(String, Vec<String>), bool>>> = OnceLock::new();
    let key = (
        compiler.to_owned(),
        flags.iter().map(|flag| (*flag).to_owned()).collect(),
    );
    let mut probes = PROBES.get_or_init(Default::default).lock().unwrap();
    if let Some(&usable) = probes.get(&key) {
        return usable;
    }
    let usable = probe_sanitizer(Path::new(compiler), flags, ASAN_PROBE_CAP);
    if !usable {
        eprintln!("skipping {compiler} {flags:?}: a trivial program built with it hangs at start-up");
    }
    probes.insert(key, usable);
    usable
}

/// One uncached `sanitizer_runs` probe: false only when the trivial program
/// built and was still running after `cap`.
pub fn probe_sanitizer(compiler: &Path, flags: &[&str], cap: Duration) -> bool {
    let source = super::scratch_file("sanitizer-probe", "c");
    let executable = source.with_extension("exe");
    std::fs::write(&source, "int main(void) { return 0; }\n").unwrap();
    let built = Command::new(compiler)
        .args(flags)
        .arg(&source)
        .arg("-o")
        .arg(&executable)
        .capped_output()
        .is_ok_and(|output| output.status.success());
    let hung = built
        && output_with_timeout(
            Command::new(&executable)
                .env("ASAN_OPTIONS", "detect_leaks=0")
                .stdin(Stdio::null()),
            cap,
            Duration::from_millis(20),
        )
        .is_none();
    let _ = std::fs::remove_file(&source);
    let _ = std::fs::remove_file(&executable);
    !hung
}
