//! End-to-end gate for the source-language detector's Rust v0 test: a C program
//! whose one ordinary C function is named `Run`/`Read`/`Runtime` must still
//! decompile as C.
//!
//! Mach-O prepends a platform underscore to every symbol, so such a name is
//! spelled `_Runtime` in the image and opens with the same two bytes as the Rust
//! v0 mangling scheme. Detecting the whole image as rustc off that prefix picks
//! the Rust output language, turns `rustabi auto` on and widens the no-return
//! list, so one C helper re-languages the program.
//!
//! Fixtures, all three 16688-byte Mach-O twins or near-twins:
//!
//! * `macho_rustlike_symbol` — the witness: `macho_imports` with `_compute`
//!   renamed `_Runtime` (`macho_rustlike_symbol.py`, a pure string-table edit).
//! * `macho_imports` — the unmodified twin; it and the witness must decompile
//!   identically.
//! * `rust_hello_x86_64` — the positive control: a genuine rustc binary still
//!   reads as Rust, so this is a narrowing of the test and not its removal.
//!
//! ## `.sla` precondition
//!
//! Bootstrapping needs the built `x86` `.sla` under `specs/` (gitignored; `make
//! specs`). When it is absent every arm prints that and returns early — a
//! specs-less environment is a visible skip, never a false green.

use std::path::PathBuf;
use std::process::Command;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .unwrap()
}

fn fixture(name: &str) -> String {
    repo_root()
        .join("decompiler/crates/kuna-analysis/tests/fixtures")
        .join(name)
        .to_str()
        .unwrap()
        .to_string()
}

fn specs() -> String {
    repo_root().join("specs").to_str().unwrap().to_string()
}

fn run(args: &[&str]) -> (String, String, bool) {
    let out = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .env_remove("KUNA_DECOMP_DBG")
        .env_remove("KUNA_DECOMP_TEST")
        .env_remove("KUNA_SLACOMP")
        .args(args)
        .output()
        .expect("run kuna");
    (
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
        out.status.success(),
    )
}

fn is_specs_skip(stderr: &str) -> bool {
    stderr.contains("could not build an architecture")
        || stderr.contains("SLEIGH")
        || stderr.contains("Could not discover")
}

/// `decompile <bin> <fn> [--json]`, or `None` when there are no `.sla` files.
fn decompile(bin: &str, func: &str, json: bool) -> Option<String> {
    let sp = specs();
    let mut args = vec!["decompile", bin, func, "--sleighpath", sp.as_str()];
    if json {
        args.push("--json");
    }
    let (stdout, stderr, ok) = run(&args);
    if !ok && is_specs_skip(&stderr) {
        eprintln!("sourcelang_macho_cli: skipping (no `.sla`): {stderr}");
        return None;
    }
    assert!(ok, "decompile {func} of {bin} failed: {stderr}");
    Some(stdout)
}

/// The witness: a C function named `Runtime` does not make its program Rust.
#[test]
fn a_c_symbol_starting_with_r_does_not_select_the_rust_language() {
    let bin = fixture("macho_rustlike_symbol");
    let (Some(json), Some(body)) = (decompile(&bin, "_main", true), decompile(&bin, "_main", false))
    else {
        return;
    };
    assert!(
        json.contains("\"language\": \"c-language\""),
        "the image is a C program; got:\n{json}"
    );
    assert!(
        !json.contains("rust-language"),
        "`_Runtime` is a C name, not a v0 path; got:\n{json}"
    );
    assert!(
        body.contains("unsigned long _main(int a0)"),
        "the C declaration must be emitted; got:\n{body}"
    );
    for absent in ["unsafe fn", "mut a0", "-> u64"] {
        assert!(!body.contains(absent), "Rust output survived ({absent}):\n{body}");
    }
}

/// The renamed symbol is the ONLY difference from the twin, so the two images
/// must decompile to the same text.
#[test]
fn the_rename_changes_nothing_but_the_symbol() {
    let (Some(witness), Some(twin)) = (
        decompile(&fixture("macho_rustlike_symbol"), "_main", false),
        decompile(&fixture("macho_imports"), "_main", false),
    ) else {
        return;
    };
    assert_eq!(witness, twin, "the string-table edit moved the decompilation");
}

/// The positive control: a real rustc binary still detects as Rust and still
/// renders in the Rust language.
#[test]
fn a_real_rust_binary_still_reads_as_rust() {
    let bin = fixture("rust_hello_x86_64");
    let (Some(json), Some(body)) =
        (decompile(&bin, "black_box", true), decompile(&bin, "black_box", false))
    else {
        return;
    };
    assert!(
        json.contains("\"language\": \"rust-language\""),
        "rustc binaries must still select the Rust language; got:\n{json}"
    );
    assert!(body.contains("unsafe fn black_box"), "got:\n{body}");
}
