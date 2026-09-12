//! End-to-end controls for a real function entry which is also a shared RET.
//!
//! The dataset witness is an i386 dietlibc syscall wrapper whose shared RET has
//! a genuine caller.  `function_boundary_return_x86_64` is the redistributable
//! reduction: it exercises the same flow decision on a second architecture and
//! keeps an ordinary adjacent function as the adversarial boundary control.

use std::path::PathBuf;
use std::process::{Command, Output};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .unwrap()
}

fn fixture() -> String {
    repo_root()
        .join("decompiler/crates/kuna-analysis/tests/fixtures/function_boundary_return_x86_64")
        .to_string_lossy()
        .into_owned()
}

fn specs() -> String {
    std::env::var("KUNA_SPECS")
        .unwrap_or_else(|_| repo_root().join("specs").to_string_lossy().into_owned())
}

fn specs_missing(stderr: &str) -> bool {
    stderr.contains("could not build an architecture")
        || stderr.contains("SLEIGH")
        || stderr.contains("Could not discover")
        || stderr.contains(".sla")
}

fn run(args: &[&str]) -> Option<Output> {
    let out = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args(args)
        .output()
        .expect("spawn kuna");
    if !out.status.success() {
        let stderr = String::from_utf8_lossy(&out.stderr);
        if specs_missing(&stderr) {
            eprintln!("skipping: specs not built ({stderr})");
            return None;
        }
        panic!("kuna {:?} failed: {stderr}", args);
    }
    Some(out)
}

fn decompile(function: &str, extra: &[&str]) -> Option<String> {
    let binary = fixture();
    let sleigh = specs();
    let mut args = vec!["decompile", &binary, function, "--sleighpath", &sleigh];
    args.extend_from_slice(extra);
    run(&args).map(|out| String::from_utf8_lossy(&out.stdout).into_owned())
}

#[test]
fn shared_entry_return_is_included_and_stops_the_flow() {
    let Some(c) = decompile("shared_wrapper", &[]) else {
        return;
    };
    assert!(
        c.contains("return 7;"),
        "the success path still loses the shared RET:\n{c}"
    );
    assert!(
        c.contains("return 0xffffffff;"),
        "the error path still loses the shared RET:\n{c}"
    );
    assert!(
        !c.contains("funcboundflow:"),
        "a real entry RET should end flow, not be replaced by a boundary halt:\n{c}"
    );
    assert!(
        !c.contains("0x21"),
        "flow crossed the admitted RET into after_ret's return value:\n{c}"
    );
}

#[test]
fn ordinary_next_function_is_still_a_hard_boundary() {
    let Some(default) = decompile("ordinary_first", &[]) else {
        return;
    };
    assert!(
        default.contains("funcboundflow:"),
        "the ordinary foreign MOV entry was not bounded:\n{default}"
    );
    assert!(
        !default.contains("0x16"),
        "ordinary_first swallowed ordinary_next's return value:\n{default}"
    );

    let Some(off) = decompile("ordinary_first", &["--option", "funcboundflow", "off"]) else {
        return;
    };
    assert!(
        off.contains("return 0x16;"),
        "the option-off control did not expose the neighbouring body:\n{off}"
    );
}

#[test]
fn branch_entries_are_not_safe_shared_terminators() {
    for function in ["before_direct_branch", "before_indirect_branch"] {
        let Some(c) = decompile(function, &[]) else {
            return;
        };
        assert!(
            c.contains("funcboundflow:"),
            "{function} flowed into a branch entry which can reach more code:\n{c}"
        );
    }
}

#[test]
fn separately_callable_return_stays_inventoried_and_callable() {
    let Some(c) = decompile("callable_ret", &[]) else {
        return;
    };
    assert!(
        c.contains("void callable_ret(void)"),
        "RET entry is not callable by name:\n{c}"
    );
    assert!(
        c.contains("return;"),
        "RET entry did not decompile as a return:\n{c}"
    );

    let binary = fixture();
    let sleigh = specs();
    let Some(functions) = run(&["functions", &binary, "--json", "--sleighpath", &sleigh]) else {
        return;
    };
    let functions = String::from_utf8_lossy(&functions.stdout);
    assert!(
        functions.contains("\"name\": \"callable_ret\""),
        "the genuine RET function disappeared from inventory:\n{functions}"
    );

    let Some(xrefs) = run(&[
        "xrefs",
        &binary,
        "--to",
        "callable_ret",
        "--sleighpath",
        &sleigh,
    ]) else {
        return;
    };
    let xrefs = String::from_utf8_lossy(&xrefs.stdout);
    assert!(
        xrefs.contains("# 1 reference"),
        "callable_ret lost its real call reference:\n{xrefs}"
    );
    assert!(
        xrefs.contains("ret_caller"),
        "the direct caller was not retained:\n{xrefs}"
    );
}
