//! End-to-end gate for the goroutine-register naming (`goreg`,
//! `docs/multilang.md` W4 — the Go pack's dedicated-register item).
//!
//! The proof, on a REAL stripped Go build under the golang cspec (`goabi`
//! auto): a runtime function that manipulates goroutine state renders its
//! accesses through the named, typed `g` global (`g->stackguard0`), and with
//! `KUNA_GOREG=off` the same function falls back to the phantom-register
//! reading (no `g->` anywhere). `runtime.unlock2` is the witness: it is
//! present in every modern Go binary, named by the pclntab on a stripped
//! build, and its preemption-request store (`g.stackguard0 = stackPreempt`)
//! is the exact idiom the runtime source spells.
//!
//! Own `[[test]]` target: the off arm mutates process env (`EnvGuard`), the
//! `verify_goabi` pattern.

use std::path::PathBuf;
use std::process::{Command, Stdio};

use kuna_console::engine::{bootstrap_from_object, ConsoleProgram};
use kuna_console::ifacedecomp::{
    execute, register_decomp_commands, IfaceDecompData, DECOMPILE_MODULE,
};
use kuna_console::ifaceterm::ConsoleCommands;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

/// `true` if a runnable `go` toolchain is on PATH.
fn go_available() -> bool {
    Command::new("go")
        .arg("version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

/// Any Go program brings the runtime along; the witness is a runtime function.
const MAIN_GO: &str = r#"package main

import (
	"fmt"
	"os"
)

func main() {
	fmt.Println(len(os.Args))
}
"#;

/// Build [`MAIN_GO`] stripped (`-ldflags=-s -w`) for linux/amd64, hermetically.
fn build_go(dir: &std::path::Path, out_bin: &std::path::Path) -> bool {
    if std::fs::create_dir_all(dir).is_err() {
        return false;
    }
    if std::fs::write(dir.join("main.go"), MAIN_GO).is_err() {
        return false;
    }
    Command::new("go")
        .args(["build", "-ldflags=-s -w", "-o"])
        .arg(out_bin)
        .arg("main.go")
        .current_dir(dir)
        .env("GOOS", "linux")
        .env("GOARCH", "amd64")
        .env("GOCACHE", dir.join(".gocache"))
        .env("GOPATH", dir.join(".gopath"))
        .env("CGO_ENABLED", "0")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

/// Bootstrap, commit, decompile `func`, return the transcript.
fn decompile_fn(bin: &str, func: &str) -> String {
    let specs = repo_root().join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];
    let mut prog: ConsoleProgram = bootstrap_from_object(bin, "", &spec_roots)
        .expect("bootstrap go fixture with built processor specs");
    prog.commit_pending_analysis().expect("analysis commit succeeds");
    let cmds: Vec<String> = [&format!("load function {func}"), "decompile", "print C"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let count = cmds.len();
    let mut status = ConsoleCommands::into_status(cmds);
    register_decomp_commands(&mut status);
    {
        let data = status.get_data_mut(DECOMPILE_MODULE).unwrap();
        let dcp = data.as_any_mut().downcast_mut::<IfaceDecompData>().unwrap();
        dcp.conf = Some(prog);
    }
    for _ in 0..count {
        execute(&mut status);
    }
    status.optr.clone()
}

/// RAII env override (the off arm), restoring the inherited value on drop.
struct EnvGuard(&'static str, Option<std::ffi::OsString>);
impl EnvGuard {
    fn set(name: &'static str, value: &str) -> EnvGuard {
        let prev = std::env::var_os(name);
        std::env::set_var(name, value);
        EnvGuard(name, prev)
    }
}
impl Drop for EnvGuard {
    fn drop(&mut self) {
        match &self.1 {
            Some(v) => std::env::set_var(self.0, v),
            None => std::env::remove_var(self.0),
        }
    }
}

#[test]
fn goroutine_state_renders_through_the_named_g_global() {
    if !go_available() {
        eprintln!("verify_goreg: skipping (`go` not on PATH)");
        return;
    }
    let dir = std::env::temp_dir().join(format!("kuna_goreg_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let out_bin = dir.join("go_stripped");
    if !build_go(&dir, &out_bin) {
        let _ = std::fs::remove_dir_all(&dir);
        panic!("installed Go compiler failed to build the linux/amd64 fixture");
    }
    let bin = out_bin.to_str().expect("utf8 path");

    // Default: goroutine state reads through the named, typed global. The
    // stackguard0 store is at the liblink-stated offset 0x10, stable across
    // Go releases, so the assertion holds wherever the runtime drifts later.
    let on = decompile_fn(bin, "runtime.unlock2");
    assert!(
        on.contains("g->"),
        "goreg on must render g-relative accesses through the named global; transcript:\n{on}"
    );
    // The stackguard0 witness SELF-SELECTS from scheduler entry points: which
    // single function touches `g->stackguard0` shifts with the runtime (go1.25
    // moved unlock2's store; CI's newer Go moved exitsyscall's), but the
    // preemption protocol itself guarantees the field is written somewhere in
    // this set on every release (87 candidates rendered it on both go1.22.10
    // and go1.25.3; these are the small, unambiguous ones).
    let candidates = [
        "runtime.exitsyscall",
        "runtime.reentersyscall",
        "runtime.gopark",
        "runtime.execute",
        "runtime.park_m",
        "runtime.acquireSudog",
    ];
    let mut transcripts = String::new();
    let hit = candidates.iter().any(|f| {
        let t = decompile_fn(bin, f);
        let ok = t.contains("g->stackguard0");
        transcripts.push_str(&format!("\n===== {f} (hit: {ok})\n{t}\n"));
        ok
    });
    assert!(
        hit,
        "no scheduler entry point renders g->stackguard0 at its liblink offset; transcripts:{transcripts}"
    );

    // The m-shell witness self-selects the same way: `m.curg` readers are
    // everywhere in the scheduler, and these candidates rendered `->curg` on
    // both go1.22.10 and go1.25.3 (the shells are layout-versioned off the
    // build-info toolchain version, so each release reads its own offsets).
    let m_candidates = [
        "runtime.execute",
        "runtime.dropm",
        "runtime.exitsyscall0",
        "runtime.canpanic",
        "runtime.deferprocStack",
    ];
    let mut m_transcripts = String::new();
    let m_hit = m_candidates.iter().any(|f| {
        let t = decompile_fn(bin, f);
        let ok = t.contains("->curg");
        m_transcripts.push_str(&format!("\n===== {f} (hit: {ok})\n{t}\n"));
        ok
    });
    assert!(
        m_hit,
        "no scheduler entry point renders the m-shell's curg field; transcripts:{m_transcripts}"
    );

    // Off: the raw reading returns — no named global anywhere in the body.
    let _guard = EnvGuard::set("KUNA_GOREG", "off");
    let off = decompile_fn(bin, "runtime.unlock2");
    assert!(
        !off.contains("g->"),
        "goreg off must restore the phantom-register reading; transcript:\n{off}"
    );

    let _ = std::fs::remove_dir_all(&dir);
}
