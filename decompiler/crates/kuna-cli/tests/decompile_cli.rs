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
