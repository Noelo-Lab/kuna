//! CLI end-to-end gate for `kuna decompile`'s **failure contract**: a function
//! whose pipeline aborted must be observable — the reason on stderr and a
//! non-zero exit — instead of the silent `exit 0` + un-decompiled shell that
//! used to hide it (the `LOSS-131` swallow).
//!
//! The engine is not needed to assert the contract (and a corpus function that
//! panics is a moving target — the panics themselves are being fixed): the
//! command shells out to `decomp_dbg`, so these tests point `--decomp-dbg` at a
//! stub that replays a real console transcript. That keeps the gate hermetic —
//! no `.sla`, no binary analysis, no dependence on which functions currently
//! abort.
//!
//! The exceptions drive the real engine over a checked-in fixture. A stub cannot
//! notice that the console reordered its own prints — the reorder that would
//! silently return `kuna decompile` to the generic wording — and it cannot
//! notice that the console's own grammar split a path on a space, because the
//! stub is not the parser under test.

#![cfg(unix)]

use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Command;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

/// Any existing binary works: the stub never reads it (the CLI only
/// canonicalizes the path and passes it in the script).
fn fauxware() -> String {
    repo_root()
        .join("decompiler/crates/kuna-analysis/tests/fixtures/fauxware")
        .to_str()
        .unwrap()
        .to_string()
}

/// Write an executable `decomp_dbg` stub that consumes the console script,
/// writes `c_body` to the script's `openfile write` target, replays
/// `transcript` on stdout and `stderr_text` on stderr, and exits 0 (exactly
/// what the real console does: a caught per-function abort is not a process
/// failure).
fn stub_decomp_dbg(tag: &str, c_body: &str, transcript: &str, stderr_text: &str) -> PathBuf {
    let path = std::env::temp_dir().join(format!(
        "kuna_stub_dbg_{tag}_{}_{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0)
    ));
    let script = format!(
        "#!/bin/sh\n\
         out=\n\
         while IFS= read -r line; do\n\
         \x20 case \"$line\" in\n\
         \x20   'openfile write '*) out=${{line#openfile write }} ;;\n\
         \x20 esac\n\
         done\n\
         [ -n \"$out\" ] && cat > \"$out\" <<'KUNA_C_EOF'\n{c_body}\nKUNA_C_EOF\n\
         cat <<'KUNA_OUT_EOF'\n{transcript}\nKUNA_OUT_EOF\n\
         cat >&2 <<'KUNA_ERR_EOF'\n{stderr_text}\nKUNA_ERR_EOF\n\
         exit 0\n"
    );
    let mut fh = std::fs::File::create(&path).expect("create the decomp_dbg stub");
    fh.write_all(script.as_bytes()).expect("write the decomp_dbg stub");
    drop(fh);
    let mut perms = std::fs::metadata(&path).expect("stat the stub").permissions();
    perms.set_mode(0o755);
    std::fs::set_permissions(&path, perms).expect("chmod the stub");
    path
}

fn run_decompile(stub: &Path, target: &str) -> (String, String, Option<i32>) {
    let out = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args(["decompile", &fauxware(), target, "--decomp-dbg", stub.to_str().unwrap()])
        .output()
        .expect("failed to spawn the kuna binary");
    (
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
        out.status.code(),
    )
}

/// The `coreutils comm` shape: the pipeline aborts, the console keeps the
/// session alive, and `print C` renders the un-decompiled shell.  The C is
/// therefore non-empty — only the console's `Skipping` notice distinguishes
/// this from an empty function, and the CLI must act on it.
#[test]
fn pipeline_abort_reports_the_reason_and_exits_nonzero() {
    let stub = stub_decomp_dbg(
        "abort",
        "void sub_3994(void)\n{\n  /* WARNING: decompilation failed: \
         decompile pipeline reached an un-ported seam (LOSS-131) */\n}",
        "[decomp]> decompile\nDecompiling sub_3994\n\
         Skipping sub_3994: decompile pipeline reached an un-ported seam (LOSS-131): \
         called `Option::unwrap()` on a `None` value\n[decomp]> print C",
        "thread 'main' panicked at crates/kuna-decomp/src/p2_lift/jumptable.rs:1811:76:\n\
         called `Option::unwrap()` on a `None` value",
    );
    let (stdout, stderr, code) = run_decompile(&stub, "main");
    let _ = std::fs::remove_file(&stub);

    assert_eq!(code, Some(1), "a failed function must not report success\n{stderr}");
    assert!(
        stderr.contains("decompilation failed for sub_3994"),
        "the reason must reach stderr, got: {stderr}"
    );
    assert!(
        stderr.contains("LOSS-131") && stderr.contains("Option::unwrap()"),
        "the real panic text must survive into the report, got: {stderr}"
    );
    assert!(
        stderr.contains("jumptable.rs:1811"),
        "the console's own stderr (panic location) must be forwarded, got: {stderr}"
    );
    assert!(
        stdout.contains("sub_3994"),
        "the recovered shell still goes to stdout, got: {stdout}"
    );
}

/// A clean run is untouched: C on stdout, nothing on stderr, exit 0.
#[test]
fn clean_run_still_exits_zero() {
    let stub = stub_decomp_dbg(
        "clean",
        "void main(void)\n{\n  return;\n}",
        "[decomp]> decompile\nDecompiling main\nDecompilation complete\n[decomp]> print C",
        "",
    );
    let (stdout, stderr, code) = run_decompile(&stub, "main");
    let _ = std::fs::remove_file(&stub);

    assert_eq!(code, Some(0), "a healthy function stays a success\n{stderr}");
    assert!(stdout.contains("void main(void)"), "got: {stdout}");
    assert!(!stderr.contains("error:"), "no error is reported, got: {stderr}");
}

/// An empty `print C` keeps the pre-existing "no C output" error (that path is
/// unchanged, and still exits non-zero).
#[test]
fn empty_output_still_errors() {
    let stub = stub_decomp_dbg("empty", "", "[decomp]> decompile\nDecompiling main", "");
    let (_stdout, stderr, code) = run_decompile(&stub, "main");
    let _ = std::fs::remove_file(&stub);

    assert_eq!(code, Some(1));
    assert!(stderr.contains("no C output"), "got: {stderr}");
}

/// GH-339's silent half, through the whole CLI: the console reported that the
/// analysis commit failed and then rendered C anyway, from a program whose debug
/// facts were only partially applied.  `kuna decompile` used to print that C and
/// exit 0, which is indistinguishable from a binary that simply has no symbols.
///
/// The stub is what makes this durable: the engine-side clamp that stops
/// `st_size` from producing this particular reason ships separately, and the
/// driver must keep reporting *any* commit failure after it lands.
#[test]
fn analysis_commit_failure_is_reported_and_exits_nonzero() {
    let stub = stub_decomp_dbg(
        "commit",
        "unsigned int main(void)\n{\n  return dat_402000;\n}",
        "[decomp]> load file /x/sz.elf\n/x/sz.elf successfully loaded: x86:LE:64:default:gcc\n\
         [decomp]> read symbols\n\
         Execution error: g_a symbol created with zero size type\n\
         [decomp]> load function main\n[decomp]> decompile\nDecompilation complete",
        "",
    );
    let (_stdout, stderr, code) = run_decompile(&stub, "main");
    let _ = std::fs::remove_file(&stub);

    assert_eq!(code, Some(1), "a dropped analysis commit must not report success\n{stderr}");
    assert!(
        stderr.contains("read symbols (analysis commit) failed: \
                         g_a symbol created with zero size type"),
        "the in-process surfaces' wording must be reproduced verbatim, got: {stderr}"
    );
}

/// A diagnostic belonging to a *different* command is not reported as the
/// analysis-commit failure — the notice is attributed to the command echo above
/// it, so an unrelated `Execution error:` leaves the exit code alone.
#[test]
fn an_unrelated_console_diagnostic_is_not_a_commit_failure() {
    let stub = stub_decomp_dbg(
        "unrelated",
        "void main(void)\n{\n  return;\n}",
        "[decomp]> read symbols\n[decomp]> option nosuchoption on\n\
         Execution error: Unknown option\n[decomp]> decompile\nDecompilation complete",
        "",
    );
    let (stdout, stderr, code) = run_decompile(&stub, "main");
    let _ = std::fs::remove_file(&stub);

    assert_eq!(code, Some(0), "an unrelated diagnostic must not be misattributed\n{stderr}");
    assert!(stdout.contains("void main(void)"), "got: {stdout}");
}

/// A load failure with no reason printed keeps the generic wording — it is the
/// fallback, not the default.
#[test]
fn a_reasonless_load_failure_keeps_the_generic_wording() {
    let stub = stub_decomp_dbg(
        "generic",
        "",
        "[decomp]> load file /x/a.out\nCould not create architecture\n[decomp]> quit",
        "",
    );
    let (_stdout, stderr, code) = run_decompile(&stub, "main");
    let _ = std::fs::remove_file(&stub);

    assert_eq!(code, Some(1));
    assert!(
        stderr.contains("(unsupported/!recognized binary)"),
        "got: {stderr}"
    );
}

fn specs() -> String {
    repo_root().join("specs").to_str().unwrap().to_string()
}

/// The DIV-88 fixture: an ELF whose `.symtab` carries `::b`, a qualified name
/// with an empty scope component.  `symbolnamerepair` (default-ON) skips the
/// degenerate component; `off` restores the hard load failure this test needs.
fn hostile_scope() -> String {
    repo_root()
        .join("decompiler/crates/kuna-analysis/tests/fixtures/hostile_scope_x86_64")
        .to_str()
        .unwrap()
        .to_string()
}

/// `true` only for a genuinely specs-less environment.
///
/// The sibling `decompile_all_cli.rs (is_specs_skip)` treats *any*
/// `could not build an architecture` as a missing-`.sla` skip, which would
/// swallow the exact message this test asserts — hence the narrower predicate.
fn is_specs_skip(stderr: &str) -> bool {
    stderr.contains("No sleigh specification")
        || stderr.contains("Could not discover")
        || stderr.contains("Could not find .sla")
}

/// GH-337: all four surfaces must answer one binary-load failure with one
/// string.  `kuna decompile` drives `decomp_dbg` as a subprocess, so it has to
/// recover the reason from the transcript; the other three hold the
/// `LowlevelError` itself.  This is the test that would catch a console-side
/// print reorder — the recorded-transcript unit tests would not.
#[test]
fn every_surface_reports_the_same_load_failure() {
    let bin = hostile_scope();
    let sp = specs();
    let project_out = std::env::temp_dir().join(format!(
        "kuna_clidiag_proj_{}_{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0)
    ));
    let project_out = project_out.to_str().unwrap().to_string();

    let surfaces: [(&str, Vec<&str>); 4] = [
        ("decompile", vec!["decompile", bin.as_str(), "main"]),
        ("functions", vec!["functions", bin.as_str()]),
        ("decompile-all", vec!["decompile-all", bin.as_str()]),
        ("decompile-project", vec!["decompile-project", bin.as_str(), "-o", project_out.as_str()]),
    ];

    let mut reports: Vec<(&str, String)> = Vec::new();
    for (name, mut argv) in surfaces {
        argv.extend_from_slice(&[
            "--option",
            "symbolnamerepair",
            "off",
            "--sleighpath",
            sp.as_str(),
        ]);
        let out = Command::new(env!("CARGO_BIN_EXE_kuna"))
            .args(&argv)
            .output()
            .expect("failed to spawn the kuna binary");
        let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
        if is_specs_skip(&stderr) {
            eprintln!("skipping: specs-less environment ({name}): {stderr}");
            let _ = std::fs::remove_dir_all(&project_out);
            return;
        }
        assert_eq!(out.status.code(), Some(1), "{name} must fail the load: {stderr}");
        reports.push((name, stderr.trim().to_string()));
    }
    let _ = std::fs::remove_dir_all(&project_out);

    let expected = format!(
        "error: could not build an architecture for {}: Non-global scope has empty name",
        std::fs::canonicalize(&bin).expect("the fixture is checked in").display()
    );
    for (name, stderr) in &reports {
        assert_eq!(
            stderr, &expected,
            "{name} must report the real reason, byte-identically to the others"
        );
    }
}

/// A unique scratch directory whose name contains a space.
///
/// Everything before the first space is unique to this run, so the path a
/// whitespace split would truncate to cannot collide with an unrelated file —
/// the truncation assertion below is then about this test and nothing else.
fn spaced_dir(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "kuna_pathtest_{tag}_{}_{} with space",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0)
    ));
    std::fs::create_dir_all(&dir).expect("create the spaced scratch dir");
    dir
}

/// The console reads a filename with `s >> filename`, so an unquoted path
/// containing a space arrives as two arguments: `load file` took the head as a
/// BFD target and the tail as the filename, and the load failed with
/// `Unable to recognize imagefile <tail>`.
///
/// `kuna decompile` is the only surface that can regress here — it is the one
/// that drives `decomp_dbg` through a text script; the in-process surfaces pass
/// the path as a string.
#[test]
fn a_binary_under_a_spaced_directory_decompiles() {
    let dir = spaced_dir("bin");
    let bin = dir.join("a.out");
    std::fs::copy(fauxware(), &bin).expect("copy the fixture into the spaced dir");

    let out = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args(["decompile", bin.to_str().unwrap(), "main", "--sleighpath", specs().as_str()])
        .output()
        .expect("failed to spawn the kuna binary");
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    let _ = std::fs::remove_dir_all(&dir);

    if is_specs_skip(&stderr) {
        eprintln!("skipping: specs-less environment: {stderr}");
        return;
    }
    assert!(
        !stderr.contains("Unable to recognize imagefile"),
        "the path was split on its space, got: {stderr}"
    );
    assert_eq!(out.status.code(), Some(0), "the load must succeed: {stderr}");
    assert!(stdout.contains("main"), "C for main must reach stdout, got: {stdout}");
}

/// The same defect on the output side, and the one that does not need a spaced
/// *binary* path to bite: the temp file lives in `std::env::temp_dir()`, which
/// on Windows is `C:\Users\First Last\AppData\Local\Temp` by default. An
/// unquoted `openfile write` truncated there, so the C was written to a file
/// named after the first path component — silently, because the console
/// discarded the write error — and the CLI reported "no C output".
#[test]
fn a_spaced_temp_dir_still_yields_c() {
    let dir = spaced_dir("tmp");
    let out = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args(["decompile", fauxware().as_str(), "main", "--sleighpath", specs().as_str()])
        .env("TMPDIR", &dir)
        .output()
        .expect("failed to spawn the kuna binary");
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();

    // The truncation target: the spaced dir's name up to its first space. The
    // prefix is unique to this run, so anything there was written by this run.
    let clobbered = dir.to_str().unwrap().split(' ').next().unwrap().to_string();
    let clobber_existed = Path::new(&clobbered).exists();
    let _ = std::fs::remove_dir_all(&dir);
    let _ = std::fs::remove_file(&clobbered);

    if is_specs_skip(&stderr) {
        eprintln!("skipping: specs-less environment: {stderr}");
        return;
    }
    assert!(!stderr.contains("no C output"), "the redirect was truncated, got: {stderr}");
    assert_eq!(out.status.code(), Some(0), "the run must succeed: {stderr}");
    assert!(stdout.contains("main"), "C for main must reach stdout, got: {stdout}");
    assert!(
        !clobber_existed,
        "the C was written to the truncated path {clobbered}, clobbering whatever was there"
    );
}
