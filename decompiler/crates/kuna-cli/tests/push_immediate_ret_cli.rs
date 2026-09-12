//! End-to-end coverage for one-store PUSH-immediate/RET tail recovery.

use std::path::PathBuf;
use std::process::Command;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .unwrap()
}

fn fixture() -> String {
    repo_root()
        .join("decompiler/crates/kuna-analysis/tests/fixtures/push_immediate_ret_i386")
        .to_string_lossy()
        .into_owned()
}

fn pe_fixture() -> String {
    repo_root()
        .join("decompiler/crates/kuna-analysis/tests/fixtures/push_immediate_ret_i386.exe")
        .to_string_lossy()
        .into_owned()
}

fn specs_missing(stderr: &str) -> bool {
    stderr.contains("could not build an architecture")
        || stderr.contains("SLEIGH")
        || stderr.contains("Could not discover")
        || stderr.contains(".sla")
}

fn decompile_binary(binary: String, function: &str, extra: &[&str]) -> Option<String> {
    let sleigh = repo_root().join("specs").to_string_lossy().into_owned();
    let out = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args(["decompile", &binary, function])
        .args(extra)
        .args(["--sleighpath", &sleigh])
        .output()
        .expect("spawn kuna");
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    if !out.status.success() {
        if specs_missing(&stderr) {
            eprintln!("skipping: specs not built ({stderr})");
            return None;
        }
        panic!("kuna decompile failed: {stderr}");
    }
    Some(stdout)
}

fn decompile(function: &str, extra: &[&str]) -> Option<String> {
    decompile_binary(fixture(), function, extra)
}

fn decompile_pe(extra: &[&str]) -> Option<String> {
    decompile_binary(pe_fixture(), "0x402000", extra)
}

#[test]
fn option_on_restores_the_terminal_transfer() {
    let Some(code) = decompile_pe(&[
        "--define-function",
        "0x402000-0x40200b=push_immediate_ret",
        "--option",
        "pushimmediateret",
        "on",
    ]) else {
        return;
    };
    assert!(
        code.contains("sub_402010();"),
        "unpacker call disappeared:\n{code}"
    );
    assert!(
        code.contains("switch(0x401000)") && code.contains("case 0x401000:"),
        "terminal transfer stayed hidden:\n{code}"
    );
}

#[test]
fn option_off_keeps_the_original_return_classification() {
    let Some(code) = decompile_pe(&[
        "--define-function",
        "0x402000-0x40200b=push_immediate_ret",
        "--option",
        "pushimmediateret",
        "off",
    ]) else {
        return;
    };
    assert!(
        code.contains("sub_402010();"),
        "unpacker call disappeared:\n{code}"
    );
    assert!(
        !code.contains("401000"),
        "option off retained the derived target:\n{code}"
    );
}

#[test]
fn ordinary_argument_adjust_overwrite_computed_and_conditional_returns_decline() {
    for function in [
        "ordinary_ret",
        "argument_push_later_ret",
        "stack_adjust_ret",
        "stack_overwrite_ret",
        "computed_target_ret",
        "conditional_bypass_ret",
    ] {
        let Some(off) = decompile(function, &["--option", "pushimmediateret", "off"]) else {
            return;
        };
        let Some(on) = decompile(function, &["--option", "pushimmediateret", "on"]) else {
            return;
        };
        assert_eq!(on, off, "negative {function} changed under the option");
    }
}

#[test]
fn two_push_call_dispatch_belongs_to_entryretdispatch() {
    let Some(off) = decompile(
        "two_push_dispatch",
        &["--option", "pushimmediateret", "off"],
    ) else {
        return;
    };
    let Some(on) = decompile("two_push_dispatch", &["--option", "pushimmediateret", "on"]) else {
        return;
    };
    assert_eq!(on, off, "one-store option stole the two-store call form");
}

#[test]
fn explicit_return_vetoes_the_automatic_branch() {
    let Some(code) = decompile_pe(&[
        "--define-function",
        "0x402000-0x40200b=push_immediate_ret",
        "--option",
        "pushimmediateret",
        "on",
        "--assert",
        "flow 0x40200a return",
    ]) else {
        return;
    };
    assert!(
        code.contains("sub_402010();"),
        "unpacker call disappeared:\n{code}"
    );
    assert!(
        !code.contains("401000"),
        "explicit return lost precedence:\n{code}"
    );
}
