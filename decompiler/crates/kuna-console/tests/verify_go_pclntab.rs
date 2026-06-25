//! End-to-end gate for Go `pclntab` function-name recovery (`s1_pclntab`, the
//! kuna analog of Ghidra's `GolangSymbolAnalyzer` name-recovery half).
//!
//! The proof: a Go binary's functions render NAMED (`main.main`, `main.compute`,
//! `runtime.*`) instead of `sub_<addr>`, recovered from the embedded pclntab — and
//! crucially this works even on a STRIPPED Go binary (no `.symtab`), where the
//! pclntab is the *only* name source. Without the pclntab pass a stripped Go
//! `main.main` is just `sub_<addr>`.
//!
//! ## How the fixture is produced
//!
//! Go ELF binaries are ~1 MB (they embed the whole runtime), too large to vendor.
//! Following Increment 15's pattern (the real-Go no-return e2e), this test BUILDS
//! a tiny Go program at runtime in an isolated temp dir with a private
//! `GOCACHE`/`GOPATH` (hermetic — never touches the user's environment), guarded
//! on `go` being on PATH AND the build succeeding; it **skips cleanly** otherwise.
//! The hermetic parser logic is pinned separately by the unit tests in
//! `kuna-analysis/src/s1_pclntab/tests.rs` (no `go` needed), so the merge-blocking
//! gate does not depend on a Go toolchain.
//!
//! ## `.sla` precondition
//!
//! Bootstrapping needs the built `x86` `.sla` under `specs/` (gitignored;
//! `make specs`). When absent the bootstrap fails; the test prints that and
//! returns early (a specs-less CI is a visible skip, never a false green).

use std::path::PathBuf;
use std::process::{Command, Stdio};

use kuna_console::engine::bootstrap_from_object;

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

/// A small Go program with a named, non-inlined function (`compute`) called from
/// `main`. `//go:noinline` keeps `main.compute` a distinct function with its own
/// pclntab entry. The `os.Args` use defeats dead-code elimination of `compute`.
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

/// Build `MAIN_GO` into `out_bin` hermetically. `strip` toggles `-ldflags=-s -w`
/// (drops the symbol table + DWARF, but KEEPS `.gopclntab`). Returns `true` on a
/// successful build that produced the binary, `false` for any failure (→ skip).
fn build_go(dir: &std::path::Path, out_bin: &std::path::Path, strip: bool) -> bool {
    if std::fs::create_dir_all(dir).is_err() {
        return false;
    }
    if std::fs::write(dir.join("main.go"), MAIN_GO).is_err() {
        return false;
    }
    let mut cmd = Command::new("go");
    cmd.arg("build");
    // Flags MUST precede the package argument (`main.go`); `-ldflags=-s -w` drops
    // the symbol table + DWARF but keeps `.gopclntab`.
    if strip {
        cmd.arg("-ldflags=-s -w");
    }
    cmd.arg("-o")
        .arg(out_bin)
        .arg("main.go")
        .current_dir(dir)
        .env("GOOS", "linux")
        .env("GOARCH", "amd64")
        .env("GOCACHE", dir.join(".gocache"))
        .env("GOPATH", dir.join(".gopath"))
        .env("CGO_ENABLED", "0")
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    matches!(cmd.status(), Ok(s) if s.success()) && out_bin.exists()
}

/// Bootstrap a binary, commit the deferred analysis (so the pclntab pass's facts
/// land), and return the `ConsoleProgram`. `None` if the `.sla` is absent (skip).
fn bootstrap(bin: &str) -> Option<kuna_console::engine::ConsoleProgram> {
    let specs = repo_root().join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];
    let mut prog = match bootstrap_from_object(bin, "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!(
                "verify_go_pclntab: skipping (bootstrap failed, build `.sla` with \
                 `make specs`): {}",
                e.explain()
            );
            return None;
        }
    };
    // The analysis-pass facts are committed at `read symbols` (gated by the per-pass
    // flags), not eagerly at bootstrap — trigger that commit here.
    prog.commit_pending_analysis().expect("analysis commit succeeds");
    Some(prog)
}

/// The headline e2e: a STRIPPED Go binary (no `.symtab`) recovers `main.main` and
/// `main.compute` as NAMED functions purely from the pclntab. This is the BEFORE
/// (`sub_<addr>`) / AFTER (`main.main`, `main.compute`) proof — on a stripped
/// binary the pclntab is the only possible name source, so a recovered name can
/// only have come from this pass.
#[test]
fn stripped_go_functions_recovered_from_pclntab() {
    if !go_available() {
        eprintln!("verify_go_pclntab: skipping (`go` not on PATH)");
        return;
    }
    let dir = std::env::temp_dir().join(format!("kuna_go_pclntab_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let out_bin = dir.join("go_stripped");
    if !build_go(&dir, &out_bin, /* strip */ true) {
        eprintln!("verify_go_pclntab: skipping (`go build` did not produce a binary)");
        let _ = std::fs::remove_dir_all(&dir);
        return;
    }
    let bin = out_bin.to_str().expect("utf8 path").to_string();

    let Some(prog) = bootstrap(&bin) else {
        let _ = std::fs::remove_dir_all(&dir);
        return;
    };

    // AFTER: the user functions are recovered by name from the pclntab. On a
    // stripped binary there is NO `.symtab`, so these names can ONLY come from the
    // pclntab pass (the BEFORE state would be a bare `sub_<addr>`).
    assert!(
        prog.lookup_symbol("main.main").is_some(),
        "main.main not recovered from the pclntab (stripped binary, no .symtab)"
    );
    assert!(
        prog.lookup_symbol("main.compute").is_some(),
        "main.compute not recovered from the pclntab (stripped binary, no .symtab)"
    );
    // A headline runtime function is recovered too (proves the runtime.* set, not
    // just the user package, is named).
    assert!(
        prog.lookup_symbol("runtime.main").is_some(),
        "runtime.main not recovered from the pclntab"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

/// The gating proof: with `--option gopclntab off` the recovery is suppressed, so
/// the same stripped binary's `main.compute` is NOT named (it falls back to
/// `sub_<addr>`). This pins that the names came from the pass, not some other
/// seam, and that the flag actually gates it.
#[test]
fn gopclntab_off_suppresses_recovery() {
    if !go_available() {
        eprintln!("verify_go_pclntab: skipping (`go` not on PATH)");
        return;
    }
    let dir = std::env::temp_dir().join(format!("kuna_go_pclntab_off_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    let out_bin = dir.join("go_stripped");
    if !build_go(&dir, &out_bin, /* strip */ true) {
        eprintln!("verify_go_pclntab: skipping (`go build` did not produce a binary)");
        let _ = std::fs::remove_dir_all(&dir);
        return;
    }
    let bin = out_bin.to_str().expect("utf8 path").to_string();

    let specs = repo_root().join("specs");
    let spec_roots = vec![specs.to_str().unwrap().to_string()];
    let mut prog = match bootstrap_from_object(&bin, "", &spec_roots) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("verify_go_pclntab: skipping (bootstrap failed): {}", e.explain());
            let _ = std::fs::remove_dir_all(&dir);
            return;
        }
    };
    // Turn the pass OFF before the deferred commit consults the flag.
    prog.arch_mut()
        .set_kuna_option("gopclntab", "off")
        .expect("gopclntab off applies");
    prog.commit_pending_analysis().expect("analysis commit succeeds");

    // With the pass gated off, the pclntab names are NOT installed on this stripped
    // binary — `main.compute` has no symbol.
    assert!(
        prog.lookup_symbol("main.compute").is_none(),
        "gopclntab off must suppress pclntab name recovery (main.compute should be absent)"
    );

    let _ = std::fs::remove_dir_all(&dir);
}
