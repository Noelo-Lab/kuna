//! End-to-end coverage for automatic entry RET-call dispatch recognition.

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
        .join("decompiler/crates/kuna-analysis/tests/fixtures/entry_ret_dispatch_i386")
        .to_string_lossy()
        .into_owned()
}

fn specs_missing(stderr: &str) -> bool {
    stderr.contains("could not build an architecture")
        || stderr.contains("SLEIGH")
        || stderr.contains("Could not discover")
        || stderr.contains(".sla")
}

fn decompile_with(function: &str, extra: &[&str]) -> Option<String> {
    let binary = fixture();
    let sleigh = repo_root().join("specs").to_string_lossy().into_owned();
    let mut command = Command::new(env!("CARGO_BIN_EXE_kuna"));
    command.args(["decompile", &binary, function]);
    command.args(extra);
    let out = command
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

fn decompile(function: &str) -> Option<String> {
    decompile_with(function, &[])
}

const CALLS: [&str; 3] = ["(*dat_804a000)()", "(*dat_804a004)()", "(*dat_804a008)()"];

#[test]
fn entry_dispatch_recovers_every_call_and_continuation() {
    let Some(code) = decompile("entry_dispatch") else {
        return;
    };
    for call in CALLS {
        assert!(
            code.contains(call),
            "entry dispatch stopped before {call}:\n{code}"
        );
    }
    assert!(
        !code.contains("return;"),
        "entry dispatch stayed an empty return:\n{code}"
    );
}

#[test]
fn ordinary_and_immediate_returns_keep_return_semantics() {
    let Some(ordinary) = decompile("ordinary_ret") else {
        return;
    };
    assert!(
        ordinary.contains("return 0x1111;"),
        "ordinary RET changed:\n{ordinary}"
    );

    let Some(immediate) = decompile("immediate_ret") else {
        return;
    };
    assert!(
        immediate.contains("return 0x2222;"),
        "RET-immediate changed:\n{immediate}"
    );
}

#[test]
fn incoming_return_address_is_not_a_dispatch_target() {
    let Some(code) = decompile("incoming_return_ret") else {
        return;
    };
    assert!(
        code.contains("return;"),
        "incoming return address became a call:\n{code}"
    );
    assert!(
        !code.contains("(*"),
        "incoming return address became a call:\n{code}"
    );
}

#[test]
fn unrelated_computed_returns_are_not_calls() {
    for function in [
        "computed_ret",
        "constant_ret",
        "adjusted_fallthrough_ret",
        "negative_displacement_ret",
        "conditional_bypass_ret",
        "partial_sp_ret",
    ] {
        let Some(code) = decompile(function) else {
            return;
        };
        assert!(
            code.contains("return;"),
            "{function} became a call:\n{code}"
        );
        assert!(
            !code.contains("first_link("),
            "{function} called a computed target:\n{code}"
        );
        for call in CALLS {
            assert!(
                !code.contains(call),
                "{function} recovered unrelated {call}:\n{code}"
            );
        }
    }

    let Some(code) = decompile("unrelated_fallthrough_store_ret") else {
        return;
    };
    assert!(
        code.contains("dat_804b00c = 0x8049058;"),
        "unrelated store disappeared:\n{code}"
    );
    assert!(
        !code.contains("(*"),
        "unrelated store made the incoming return address callable:\n{code}"
    );
}

#[test]
fn option_off_restores_the_first_ret_termination() {
    let Some(code) = decompile_with("entry_dispatch", &["--option", "entryretdispatch", "off"])
    else {
        return;
    };
    assert!(code.contains("return;"), "option off did not restore the return:\n{code}");
    for call in CALLS {
        assert!(!code.contains(call), "option off still derived {call}:\n{code}");
    }
}

#[test]
fn explicit_return_cleanly_vetoes_the_automatic_call() {
    let Some(code) = decompile_with("entry_dispatch", &["--assert", "flow 0x804900b return"])
    else {
        return;
    };
    assert!(code.contains("return;"), "explicit return did not win:\n{code}");
    for call in CALLS {
        assert!(!code.contains(call), "explicit return was overridden by {call}:\n{code}");
    }
}
