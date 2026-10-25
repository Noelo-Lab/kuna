//! End-to-end gate for golang compiler-spec selection (`goabi`,
//! `docs/multilang.md` W4 — the Go ABI plane).
//!
//! The proof, on a REAL stripped Go build: under the `auto` default the image
//! loads with the vendored golang cspec, so a two-argument multi-return
//! function recovers BOTH register parameters and keeps BOTH result
//! computations; with `goabi off` it loads under the platform `gcc` model,
//! where the same function renders `(void)` with never-assigned register
//! locals and the second result's computation is dead-coded away.
//!
//! Its own `[[test]]` target: the off arm travels through `KUNA_GOABI`, a
//! process-global environment variable read inside `load file`, and the house
//! rule is that env-mutating tests never share the `integration` binary.

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

/// `pair` is the witness: two register arguments in, two results out — the
/// shape the platform C model cannot see at all.
const MAIN_GO: &str = r#"package main

import (
	"fmt"
	"os"
)

//go:noinline
func pair(a, b int) (int, int) {
	return a + b, a * b
}

func main() {
	s, p := pair(len(os.Args), 3)
	fmt.Println(s, p)
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

/// Bootstrap (the language id is composed inside), commit, decompile
/// `main.pair`, return the transcript.
fn decompile_pair(bin: &str) -> String {
    let specs = repo_root().join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];
    let mut prog: ConsoleProgram = bootstrap_from_object(bin, "", &spec_roots)
        .expect("bootstrap go fixture with built processor specs");
    prog.commit_pending_analysis().expect("analysis commit succeeds");
    let cmds: Vec<String> = ["load function main.pair", "decompile", "print C"]
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

/// RAII env guard: restores the inherited value on drop (unwind included).
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
fn go_image_takes_the_golang_cspec_under_auto_and_gcc_under_off() {
    if !go_available() {
        eprintln!("verify_goabi: skipping (`go` not on PATH)");
        return;
    }
    let dir = std::env::temp_dir().join(format!("kuna_goabi_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let out_bin = dir.join("go_stripped");
    if !build_go(&dir, &out_bin) {
        let _ = std::fs::remove_dir_all(&dir);
        panic!("installed Go compiler failed to build the linux/amd64 fixture");
    }
    let bin = out_bin.to_str().expect("utf8 path");

    // auto (the default): the golang cspec sees both register arguments and
    // keeps both results — the sum and the product both reach the output.
    let auto = decompile_pair(bin);
    assert!(
        auto.contains("main.pair(") && !auto.contains("main.pair(void)"),
        "auto must recover main.pair's register parameters; transcript:\n{auto}"
    );
    assert!(
        auto.contains("a0 + a1") && auto.contains("* a0"),
        "auto must keep both results (sum and product); transcript:\n{auto}"
    );

    // off: the platform model reads RAX/RBX as scratch — no parameters, and
    // the product (dead under one 8-byte return) is deleted with its multiply.
    let _guard = EnvGuard::set("KUNA_GOABI", "off");
    let off = decompile_pair(bin);
    assert!(
        off.contains("main.pair(void)"),
        "off must restore the platform-model reading (no visible parameters); transcript:\n{off}"
    );
    assert!(
        !off.contains("* a0") && !off.contains("a1 * a0"),
        "off deletes the product computation (the documented baseline defect); transcript:\n{off}"
    );

    let _ = std::fs::remove_dir_all(&dir);
}
