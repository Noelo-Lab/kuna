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

fn decompile_binary(binary: String, function: &str, extra: &[&str]) -> String {
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
        panic!("kuna decompile failed: {stderr}");
    }
    stdout
}

fn decompile(function: &str, extra: &[&str]) -> String {
    decompile_binary(fixture(), function, extra)
}

fn decompile_pe(extra: &[&str]) -> String {
    decompile_binary(pe_fixture(), "0x402000", extra)
}

/// A recovered one-store transfer renders as a terminal branch (`switch(0x...)`
/// over the constant, or an indirect tail call); a RET left alone renders none.
fn keeps_return_classification(code: &str) -> bool {
    !code.contains("switch(") && !code.contains("(*(code *)")
}

#[test]
fn the_terminal_transfer_is_recovered() {
    let code = decompile_pe(&["--define-function", "0x402000-0x40200b=push_immediate_ret"]);
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
fn ordinary_argument_adjust_overwrite_computed_and_conditional_returns_decline() {
    for function in [
        "ordinary_ret",
        "argument_push_later_ret",
        "stack_adjust_ret",
        "stack_overwrite_ret",
        "computed_target_ret",
        "conditional_bypass_ret",
    ] {
        let code = decompile(function, &[]);
        assert!(keeps_return_classification(&code), "negative {function} became a transfer:\n{code}");
    }
}

#[test]
fn two_push_call_dispatch_belongs_to_entryretdispatch() {
    let code = decompile("two_push_dispatch", &[]);
    assert!(code.contains("dispatch_helper();"), "the two-store call form was lost:\n{code}");
    assert!(keeps_return_classification(&code), "the one-store rule stole the two-store call form:\n{code}");
}

#[test]
fn explicit_return_vetoes_the_automatic_branch() {
    let code = decompile_pe(&[
        "--define-function",
        "0x402000-0x40200b=push_immediate_ret",
        "--assert",
        "flow 0x40200a return",
    ]);
    assert!(
        code.contains("sub_402010();"),
        "unpacker call disappeared:\n{code}"
    );
    assert!(
        !code.contains("401000"),
        "explicit return lost precedence:\n{code}"
    );
}
