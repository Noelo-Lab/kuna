//! CLI end-to-end gate for `kuna decompile-all` / `kuna functions` — drives the
//! built `kuna` binary over the real vendored `fauxware` ELF and asserts the
//! machine-readable JSON surface decbench and an LLM driver consume.
//!
//! ## `.sla` precondition
//!
//! Bootstrapping needs the built `x86` `.sla` under `specs/` (gitignored;
//! `make specs`).  When it is absent the command fails to build an architecture;
//! the test prints that and returns early (a specs-less CI is a visible skip,
//! never a false green).

use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn fauxware() -> String {
    repo_root()
        .join("decompiler/crates/kuna-analysis/tests/fixtures/fauxware")
        .to_str()
        .unwrap()
        .to_string()
}

/// The checked-in stripped-ELF hang repro (`tests/hang-repro/README.md`): a
/// fully-stripped x86-64 openssh `ssh-sk-helper` whose `sub_1bd04` @ 0x1bd04
/// never converges in the decompile pipeline (the `--max-fn-seconds` watchdog's
/// raison d'être).
fn hang_repro() -> String {
    repo_root().join("tests/hang-repro/ssh-sk-helper").to_str().unwrap().to_string()
}

fn specs() -> String {
    repo_root().join("specs").to_str().unwrap().to_string()
}

/// Run the built `kuna` binary, returning `(stdout, stderr, success)`.
fn run_kuna(args: &[&str]) -> (String, String, bool) {
    let out = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args(args)
        .output()
        .expect("failed to spawn the kuna binary");
    (
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
        out.status.success(),
    )
}

/// Run the built `kuna` binary with a hard outer wall-clock `cap`, returning
/// `Some((stdout, stderr, success))` if it exited in time, `None` if it had to
/// be killed.  The outer cap is the regression guard for the watchdog itself:
/// without `--max-fn-seconds` the hang-repro invocation would spin forever.
fn run_kuna_with_timeout(args: &[&str], cap: Duration) -> Option<(String, String, bool)> {
    use std::io::Read;
    let mut child = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("failed to spawn the kuna binary");
    // Drain the pipes on reader threads so a chatty child can never block on a
    // full pipe while we poll for exit.
    let mut stdout_pipe = child.stdout.take().expect("stdout piped");
    let mut stderr_pipe = child.stderr.take().expect("stderr piped");
    let out_thread = std::thread::spawn(move || {
        let mut buf = Vec::new();
        let _ = stdout_pipe.read_to_end(&mut buf);
        buf
    });
    let err_thread = std::thread::spawn(move || {
        let mut buf = Vec::new();
        let _ = stderr_pipe.read_to_end(&mut buf);
        buf
    });
    let deadline = Instant::now() + cap;
    let status = loop {
        match child.try_wait().expect("try_wait on the kuna binary") {
            Some(st) => break Some(st),
            None if Instant::now() >= deadline => {
                let _ = child.kill();
                let _ = child.wait();
                break None;
            }
            None => std::thread::sleep(Duration::from_millis(200)),
        }
    };
    let stdout = String::from_utf8_lossy(&out_thread.join().expect("stdout reader")).into_owned();
    let stderr = String::from_utf8_lossy(&err_thread.join().expect("stderr reader")).into_owned();
    status.map(|st| (stdout, stderr, st.success()))
}

/// `true` when the failure is a missing-`.sla` bootstrap failure (a legitimate
/// skip), not a real bug.
fn is_specs_skip(stderr: &str) -> bool {
    stderr.contains("could not build an architecture")
        || stderr.contains("SLEIGH")
        || stderr.contains("Could not discover")
}

#[test]
fn decompile_all_emits_json_for_main() {
    let bin = fauxware();
    let (stdout, stderr, ok) = run_kuna(&[
        "decompile-all",
        &bin,
        "--functions",
        "main,authenticate",
        "--json",
        "--sleighpath",
        &specs(),
    ]);
    if !ok {
        if is_specs_skip(&stderr) {
            eprintln!("decompile_all_cli: skipping (no `.sla`; run `make specs`): {stderr}");
            return;
        }
        panic!("kuna decompile-all failed: {stderr}");
    }
    // Shape assertions (no JSON dep): two functions, both with non-null code.
    assert!(stdout.trim_start().starts_with('{'), "output is not a JSON object:\n{stdout}");
    assert!(stdout.contains("\"count\": 2"), "expected count 2:\n{stdout}");
    assert!(stdout.contains("\"name\": \"main\""), "missing function `main`:\n{stdout}");
    assert!(stdout.contains("\"name\": \"authenticate\""), "missing `authenticate`:\n{stdout}");
    assert!(stdout.contains("\"variables\""), "missing variables array:\n{stdout}");
    // `authenticate(const char *, const char *)` ⇒ a parameter with arg_index 0.
    assert!(
        stdout.contains("\"kind\": \"arg\"") && stdout.contains("\"arg_index\": 0"),
        "expected a parameter with arg_index 0:\n{stdout}"
    );
}

/// The per-function watchdog (`--max-fn-seconds`) bounds the known
/// non-converging function of the stripped-ELF hang repro: the invocation that
/// used to spin FOREVER (100% CPU, no output) must now terminate, exit 0, and
/// record the function as a per-function `error` in otherwise-valid JSON.
///
/// The 300s outer cap is deliberately generous (slow debug builds / shared-
/// machine load); the watchdog itself is asked for 10s.
#[test]
fn decompile_all_watchdog_bounds_pathological_function() {
    let bin = hang_repro();
    let res = run_kuna_with_timeout(
        &[
            "decompile-all",
            &bin,
            "--addr",
            "0x1bd04",
            "--json",
            "--max-fn-seconds",
            "10",
            "--sleighpath",
            &specs(),
        ],
        Duration::from_secs(300),
    );
    let (stdout, stderr, ok) = match res {
        Some(t) => t,
        None => panic!(
            "kuna decompile-all did not terminate within the 300s outer bound — \
             the --max-fn-seconds watchdog is not firing on the hang repro"
        ),
    };
    if !ok {
        if is_specs_skip(&stderr) {
            eprintln!("decompile_all_cli: skipping (no `.sla`; run `make specs`): {stderr}");
            return;
        }
        panic!("kuna decompile-all failed: {stderr}");
    }
    // Shape assertions (no JSON dep): a well-formed single-function document
    // whose one record errored out on the budget, with no code emitted.
    let trimmed = stdout.trim();
    assert!(trimmed.starts_with('{') && trimmed.ends_with('}'), "output is not a JSON object:\n{stdout}");
    assert!(stdout.contains("\"count\": 1"), "expected count 1:\n{stdout}");
    assert!(stdout.contains("\"address_hex\": \"0x1bd04\""), "missing the 0x1bd04 record:\n{stdout}");
    assert!(
        stdout.contains("\"error\": \"per-function decompile budget exceeded (10 s)\""),
        "expected the watchdog budget error:\n{stdout}"
    );
    assert!(stdout.contains("\"code\": null"), "a budget-exceeded function must have null code:\n{stdout}");
}

/// Watchdog control: a healthy function in the SAME hang-repro binary
/// decompiles normally under the default budget — `code` non-null, `error`
/// null — so the watchdog demonstrably fires only on pathological input.
#[test]
fn decompile_all_watchdog_quiet_on_healthy_function() {
    let bin = hang_repro();
    // 0x5020 is a tiny PLT-style thunk (`sub_5020`) that decompiles in
    // milliseconds on a release build; the default 120s budget applies.
    let res = run_kuna_with_timeout(
        &["decompile-all", &bin, "--addr", "0x5020", "--json", "--sleighpath", &specs()],
        Duration::from_secs(300),
    );
    let (stdout, stderr, ok) = match res {
        Some(t) => t,
        None => panic!("kuna decompile-all on a healthy function did not terminate in 300s"),
    };
    if !ok {
        if is_specs_skip(&stderr) {
            eprintln!("decompile_all_cli: skipping (no `.sla`; run `make specs`): {stderr}");
            return;
        }
        panic!("kuna decompile-all failed: {stderr}");
    }
    assert!(stdout.contains("\"count\": 1"), "expected count 1:\n{stdout}");
    assert!(stdout.contains("\"error\": null"), "healthy function must have null error:\n{stdout}");
    assert!(stdout.contains("\"code\": \""), "healthy function must emit code:\n{stdout}");
    assert!(
        !stdout.contains("budget exceeded"),
        "watchdog must not fire on a healthy function:\n{stdout}"
    );
}

#[test]
fn functions_lists_main() {
    let bin = fauxware();
    let (stdout, stderr, ok) = run_kuna(&["functions", &bin, "--json", "--sleighpath", &specs()]);
    if !ok {
        if is_specs_skip(&stderr) {
            eprintln!("decompile_all_cli: skipping (no `.sla`; run `make specs`): {stderr}");
            return;
        }
        panic!("kuna functions failed: {stderr}");
    }
    assert!(stdout.contains("\"name\": \"main\""), "enumeration missing `main`:\n{stdout}");
    assert!(stdout.contains("\"address\""), "enumeration missing addresses:\n{stdout}");
}
