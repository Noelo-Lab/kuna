//! `kuna` alone is the toolchain: copied into an empty directory, it runs its
//! built-in decompiler console and SLEIGH compiler with no sibling binaries.

use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::sync::Mutex;

mod common;
use common::{fixture, repo_root};

// A concurrent fork can inherit the copy's writable descriptor until exec,
// causing the relocated executable to fail with ETXTBSY even after copy returns.
static PROCESS_SETUP: Mutex<()> = Mutex::new(());

/// A directory holding nothing but a copy of `kuna`.
fn lone_kuna(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("kuna_single_{}_{tag}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("create sandbox");
    dir
}

/// Run the copy in `dir` with `args`, the specs pinned to the checkout's tree and
/// no separate engine binary named.
fn run_alone(dir: &Path, args: &[&str]) -> Output {
    let exe = dir.join(format!("kuna{}", std::env::consts::EXE_SUFFIX));
    let child = {
        let _setup = PROCESS_SETUP.lock().unwrap();
        std::fs::copy(env!("CARGO_BIN_EXE_kuna"), &exe).expect("copy kuna");
        Command::new(&exe)
            .current_dir(dir)
            .args(args)
            .env("KUNA_SPECS", repo_root().join("specs"))
            .env_remove("SLEIGHHOME")
            .env_remove("KUNA_ROOT")
            .env_remove("KUNA_DECOMP_DBG")
            .env_remove("KUNA_SLACOMP")
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("run the relocated kuna")
    };
    child.wait_with_output().expect("wait for the relocated kuna")
}

#[test]
fn a_lone_kuna_decompiles_with_its_built_in_console() {
    let dir = lone_kuna("decompile");
    let out = run_alone(&dir, &["decompile", &fixture("fauxware"), "main"]);
    let entries = std::fs::read_dir(&dir).unwrap().count();
    let _ = std::fs::remove_dir_all(&dir);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
    assert!(stdout.contains("main("), "{stdout}");
    assert_eq!(entries, 1, "decompile left files beside kuna");
}

#[test]
fn a_lone_kuna_compiles_a_slaspec_with_its_built_in_compiler() {
    let dir = lone_kuna("specs");
    let languages = repo_root().join("specs/Ghidra/Processors/Toy/data/languages");
    let out_sla = dir.join("toy_be.sla");
    let out = run_alone(
        &dir,
        &[
            "specs",
            languages.join("toy_be.slaspec").to_str().unwrap(),
            out_sla.to_str().unwrap(),
        ],
    );
    let compiled = std::fs::read(&out_sla);
    let _ = std::fs::remove_dir_all(&dir);
    assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
    let shipped = std::fs::read(languages.join("toy_be.sla")).expect("make specs built toy_be.sla");
    assert!(compiled.expect("kuna specs wrote no .sla") == shipped, "the built-in compiler's .sla differs");
}

#[test]
fn the_hidden_console_subcommand_is_decomp_dbg() {
    let out = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .arg("decomp_dbg")
        .env_remove("SLEIGHHOME")
        .stdin(Stdio::null())
        .output()
        .expect("run kuna decomp_dbg");
    assert_eq!(out.status.code(), Some(1));
    assert!(
        String::from_utf8_lossy(&out.stderr).contains("Could not discover root of Ghidra installation"),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
}
