//! End-to-end gate for the Go GC write-barrier fold (`gowritebarrier`,
//! `docs/multilang.md` W2/W5 — the guarded-cold-call idiom).
//!
//! The proof, on a REAL stripped Go build (names from the pclntab alone, the
//! language from the buildinfo section alone):
//! - default (`auto`): a function with guarded pointer stores decompiles with
//!   NO `gcWriteBarrier` anywhere, and the program's own global store is still
//!   in the C;
//! - `option gowritebarrier off`: the same function renders the thunk call.
//!
//! The hermetic matcher logic is pinned by the `kuna_gowritebarrier` unit
//! tests and `tests/stages/kuna-gowritebarrier.xml`; this gate is the
//! detection-driven `auto` path over a genuine Go image (the
//! `verify_go_morestack` pattern, same hermetic build recipe).

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

/// `main.push` writes two heap pointers (`n.next = head; head = n`), so the
/// compiler plants a write-barrier diamond in front of each store group.
const MAIN_GO: &str = r#"package main

import "os"

type Node struct {
	next *Node
	val  int
}

var head *Node

//go:noinline
func push(n *Node) {
	n.next = head
	head = n
}

func main() {
	for i := 0; i < len(os.Args); i++ {
		push(&Node{val: i})
	}
	if head != nil {
		os.Exit(head.val)
	}
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

/// Bootstrap, optionally flip `gowritebarrier`, commit the analysis facts,
/// then decompile `main.push` and return the console transcript.
fn decompile_push(bin: &str, option: Option<&str>) -> String {
    let specs = repo_root().join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];
    let mut prog: ConsoleProgram = bootstrap_from_object(bin, "", &spec_roots)
        .expect("bootstrap go fixture with built processor specs");
    if let Some(v) = option {
        prog.arch_mut().set_kuna_option("gowritebarrier", v).expect("gowritebarrier applies");
    }
    prog.commit_pending_analysis().expect("analysis commit succeeds");
    let cmds: Vec<String> = ["load function main.push", "decompile", "print C"]
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
/// the fold; and `off` restores the barrier for auditing.
#[test]
fn stripped_go_pointer_store_folds_its_write_barrier_under_auto() {
    if !go_available() {
        eprintln!("verify_go_writebarrier: skipping (`go` not on PATH)");
        return;
    }
    let dir = std::env::temp_dir().join(format!("kuna_go_wb_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let out_bin = dir.join("go_stripped");
    if !build_go(&dir, &out_bin) {
        let _ = std::fs::remove_dir_all(&dir);
        panic!("installed Go compiler failed to build the linux/amd64 fixture");
    }
    let bin = out_bin.to_str().expect("utf8 path");

    let kept = decompile_push(bin, Some("off"));
    assert!(
        kept.contains("gcWriteBarrier"),
        "with gowritebarrier off the thunk call must render; transcript:\n{kept}"
    );

    let folded = decompile_push(bin, None);
    assert!(
        !folded.contains("gcWriteBarrier") && !folded.contains("writeBarrier"),
        "under the auto default a Go binary's write barrier must fold; transcript:\n{folded}"
    );
    // The fold deletes GC bookkeeping, never the program's store: the global
    // head-pointer assignment (a `dat_<addr> =` store) survives.
    assert!(
        folded.contains("dat_") && folded.contains(" = "),
        "the program's own stores survive the fold; transcript:\n{folded}"
    );

    let _ = std::fs::remove_dir_all(&dir);
}
