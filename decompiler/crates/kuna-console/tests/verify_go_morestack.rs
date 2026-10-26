//! End-to-end gate for the Go stack-growth prologue fold (`gomorestack`,
//! `docs/multilang.md` W2/W5 — the first entry-guard-loop idiom).
//!
//! The proof, on a REAL stripped Go build (names from the pclntab alone, the
//! language from the buildinfo section alone):
//! - default (`auto`): `main.main` decompiles with NO `morestack` anywhere —
//!   the detected source language is Go, so the entry guard folds and the body
//!   is straight-line;
//! - `option gomorestack off`: the same function renders the guard — a
//!   `runtime.morestack_noctxt` call survives into the C.
//!
//! The hermetic matcher logic is pinned by the `kuna_gomorestack` unit tests
//! and `tests/stages/kuna-gomorestack.xml` (no `go` needed); this gate is the
//! detection-driven `auto` path over a genuine Go image. Built at runtime in
//! an isolated temp dir (the `verify_go_pclntab` pattern), guarded on `go`
//! being on PATH; an installed compiler must build the fixture successfully.

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

/// A `main.main` that calls through `fmt` — a non-leaf frame, so the compiler
/// plants the stack-growth guard this gate exists to fold.
const MAIN_GO: &str = r#"package main

import (
	"fmt"
	"os"
)

//go:noinline
func compute(x int) int {
	return x*x + len(os.Args)
}

func main() {
	fmt.Println(compute(7))
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

/// Bootstrap, optionally flip `gomorestack`, commit the analysis facts, then
/// `load function main.main` → `decompile` → `print C`, returning the console
/// transcript (which carries the C).
fn decompile_main(bin: &str, option: Option<&str>) -> String {
    let specs = repo_root().join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];
    let mut prog: ConsoleProgram = bootstrap_from_object(bin, "", &spec_roots)
        .expect("bootstrap go fixture with built processor specs");
    if let Some(v) = option {
        prog.arch_mut().set_kuna_option("gomorestack", v).expect("gomorestack option applies");
    }
    prog.commit_pending_analysis().expect("analysis commit succeeds");
    let cmds: Vec<String> = ["load function main.main", "decompile", "print C"]
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

/// The auto path end to end: detection (buildinfo) → `source_lang` fact →
/// the fold; and `off` restores the guard for auditing.
#[test]
fn stripped_go_main_folds_its_stack_growth_prologue_under_auto() {
    if !go_available() {
        eprintln!("verify_go_morestack: skipping (`go` not on PATH)");
        return;
    }
    let dir = std::env::temp_dir().join(format!("kuna_go_morestack_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let out_bin = dir.join("go_stripped");
    if !build_go(&dir, &out_bin) {
        let _ = std::fs::remove_dir_all(&dir);
        panic!("installed Go compiler failed to build the linux/amd64 fixture");
    }
    let bin = out_bin.to_str().expect("utf8 path");

    let kept = decompile_main(bin, Some("off"));
    assert!(
        kept.contains("morestack"),
        "with gomorestack off the stack-growth call must render; transcript:\n{kept}"
    );

    let folded = decompile_main(bin, None);
    assert!(
        !folded.contains("morestack"),
        "under the auto default a Go binary's entry guard must fold; transcript:\n{folded}"
    );
    // The fold deletes the guard, never the body: the function still prints.
    assert!(
        folded.contains("main.main") || folded.contains("void main"),
        "the folded function still decompiles; transcript:\n{folded}"
    );

    let _ = std::fs::remove_dir_all(&dir);
}
