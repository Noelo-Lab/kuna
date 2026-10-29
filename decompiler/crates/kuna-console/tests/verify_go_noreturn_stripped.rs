//! End-to-end gate for the Go pclntab no-return pass (`gonoreturn`,
//! `docs/multilang.md`, the Go pack's no-return coverage item).
//!
//! The structural gap: the symbol-table no-return pass (`noreturn_known`)
//! emits ZERO facts on a stripped Go binary (`-ldflags=-s -w` drops both
//! symbol tables), so the vendored `GolangFunctionsThatDoNotReturn` went
//! unused on exactly the binaries it was vendored for. This gate proves the
//! pclntab-sourced pass closes it, on a REAL stripped go build:
//!
//! - default (`gonoreturn on`): a variable signed shift's guard calls
//!   `runtime.panicshift`, and the call is TERMINAL — flow truncates, no
//!   fall-through into the INT3 padding after the function;
//! - `option gonoreturn off`: the same call keeps its fall-through edge and
//!   the decompile runs into the padding (`int3-pad` artifacts) — the bug.
//!
//! Built at runtime in an isolated temp dir (the `verify_go_pclntab`
//! pattern), guarded on `go` being on PATH; an installed compiler must build
//! the fixture successfully. The list-matching logic is pinned hermetically
//! by the unit tests in `kuna-analysis` (`pclntab/kuna_gonoreturn.rs`).

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

/// A variable SIGNED shift: the compiler cannot prove `n` non-negative, so it
/// plants the `if n < 0 { runtime.panicshift() }` guard this gate needs.
const MAIN_GO: &str = r#"package main

import (
	"fmt"
	"os"
)

//go:noinline
func sh(x int, n int) int {
	return x << n
}

func main() {
	fmt.Println(sh(len(os.Args), len(os.Args[0])-3))
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

/// Bootstrap, optionally flip `gonoreturn` BEFORE the analysis commit (the
/// per-pass gate is decided there), then decompile `main.sh` and return the
/// console transcript.
fn decompile_sh(bin: &str, option: Option<&str>) -> String {
    let specs = repo_root().join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];
    let mut prog: ConsoleProgram = bootstrap_from_object(bin, "", &spec_roots)
        .expect("bootstrap go fixture with built processor specs");
    if let Some(v) = option {
        prog.arch_mut().set_kuna_option("gonoreturn", v).expect("gonoreturn option applies");
    }
    prog.commit_pending_analysis().expect("analysis commit succeeds");
    let cmds: Vec<String> =
        ["load function main.sh", "decompile", "print C"].iter().map(|s| s.to_string()).collect();
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

/// The headline e2e: on a STRIPPED Go binary the vendored no-return list
/// reaches the pclntab-recovered `runtime.panicshift`, truncating flow at the
/// shift guard's panic call; `off` restores the fall-through bug.
#[test]
fn stripped_go_panicshift_truncates_flow_under_the_default() {
    if !go_available() {
        eprintln!("verify_go_noreturn_stripped: skipping (`go` not on PATH)");
        return;
    }
    let dir = std::env::temp_dir().join(format!("kuna_go_noreturn_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let out_bin = dir.join("go_stripped_shift");
    if !build_go(&dir, &out_bin) {
        let _ = std::fs::remove_dir_all(&dir);
        panic!("installed Go compiler failed to build the linux/amd64 fixture");
    }
    let bin = out_bin.to_str().expect("utf8 path");

    let on = decompile_sh(bin, None);
    assert!(on.contains("panicshift"), "the shift guard's panic call renders; transcript:\n{on}");
    assert!(
        !on.contains("int3-pad"),
        "under the default the panic call is terminal - no fall-through into the \
         padding after the function; transcript:\n{on}"
    );

    let off = decompile_sh(bin, Some("off"));
    assert!(off.contains("panicshift"), "the call itself is untouched by the option; transcript:\n{off}");
    assert!(
        off.contains("int3-pad"),
        "with gonoreturn off the fall-through bug is back (flow runs into the INT3 \
         padding), proving the facts came from this pass; transcript:\n{off}"
    );

    let _ = std::fs::remove_dir_all(&dir);
}
