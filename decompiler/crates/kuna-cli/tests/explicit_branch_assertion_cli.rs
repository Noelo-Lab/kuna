//! End-to-end precedence coverage for an explicit intraprocedural flow assertion
//! at a branch that also targets a discovered function entry, and for the near
//! misses that must keep tail-call recovery: a refused `branch` fact, and an
//! applied one at a different instruction of the same function.
//!
//! `sub_401090` carries both near misses: `call 0x44a000` at `0x40109a` enters
//! a return-address-discarding fragment that re-enters the function, and the
//! `jmp 0x407b20` at `0x401148` is a tail call to a function the same body also
//! calls directly.

use std::path::PathBuf;
use std::process::Command;

const NEAR_MISS_ENTRY: &str = "0x401090";
const NEAR_MISS_TAIL_CALL: &str = "sub_407b20(); // tail-call";

struct Run {
    success: bool,
    doc: String,
    code: String,
}

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .unwrap()
}

fn fixture() -> String {
    repo_root()
        .join(
            "decompiler/crates/kuna-analysis/tests/fixtures/\
explicit_branch_assertion_pe_i386.exe",
        )
        .to_string_lossy()
        .into_owned()
}

fn specs_missing(stderr: &str) -> bool {
    stderr.contains("could not build an architecture")
        || stderr.contains("SLEIGH")
        || stderr.contains("Could not discover")
        || stderr.contains(".sla")
}

fn json_string_field<'a>(doc: &'a str, key: &str) -> Option<&'a str> {
    let at = doc.find(&format!("\"{key}\":"))? + key.len() + 3;
    let body = doc[at..].trim_start().strip_prefix('"')?;
    let mut escaped = false;
    for (i, ch) in body.char_indices() {
        match ch {
            _ if escaped => escaped = false,
            '\\' => escaped = true,
            '"' => return Some(&body[..i]),
            _ => {}
        }
    }
    None
}

/// Decompile `entry` with `extra` arguments; `None` is a visible skip when the
/// specs are not built. A refused assertion exits nonzero but still prints the
/// function, so the exit status is returned rather than judged here.
fn decompile_at(entry: &str, extra: &[&str]) -> Option<Run> {
    let binary = fixture();
    let sleigh = repo_root().join("specs").to_string_lossy().into_owned();
    let mut args = vec![
        "decompile",
        &binary,
        entry,
        "--addr",
        "--sleighpath",
        &sleigh,
        "--json",
    ];
    args.extend_from_slice(extra);
    let out = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args(args)
        .output()
        .expect("spawn kuna");
    let doc = String::from_utf8_lossy(&out.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    if !out.status.success() && specs_missing(&stderr) {
        eprintln!("skipping: specs not built ({stderr})");
        return None;
    }
    let code = json_string_field(&doc, "code")
        .unwrap_or_else(|| panic!("JSON result has no function code: {stderr}\n{doc}"))
        .to_string();
    Some(Run {
        success: out.status.success(),
        doc,
        code,
    })
}

fn decompile(extra: &[&str]) -> Option<(String, String)> {
    let run = decompile_at("0x40d120", extra)?;
    assert!(run.success, "kuna decompile failed:\n{}", run.doc);
    Some((run.doc, run.code))
}

fn assert_full_body(code: &str) {
    assert!(
        !code.contains("// tail-call"),
        "assertion became a tail call:\n{code}"
    );
    assert!(
        code.contains("0x40d12b"),
        "overlapping return site is absent:\n{code}"
    );
    assert!(
        code.contains("sub_40d21e()"),
        "downstream body is absent:\n{code}"
    );
}

#[test]
fn explicit_branch_owns_precedence_over_tail_call_inference() {
    let Some((doc, code)) = decompile(&["--assert", "flow 0x40d126 branch"]) else {
        return;
    };
    assert_full_body(&code);
    assert!(
        doc.contains("\"status\": \"applied\""),
        "assertion was not applied:\n{doc}"
    );
}

#[test]
fn disabling_tailcalljump_is_an_equivalent_control() {
    let Some((_, precedence)) = decompile(&["--assert", "flow 0x40d126 branch"]) else {
        return;
    };
    let Some((_, option_off)) = decompile(&[
        "--assert",
        "flow 0x40d126 branch",
        "--option",
        "tailcalljump",
        "off",
    ]) else {
        return;
    };
    assert_eq!(precedence, option_off);
}

#[test]
fn no_assertion_control_stays_a_full_body() {
    let Some((_, default)) = decompile(&[]) else {
        return;
    };
    assert_full_body(&default);
}

#[test]
fn applied_branch_at_another_instruction_keeps_tail_call_recovery() {
    let Some(control) = decompile_at(NEAR_MISS_ENTRY, &[]) else {
        return;
    };
    assert!(control.success, "kuna decompile failed:\n{}", control.doc);
    assert!(
        control.code.contains(NEAR_MISS_TAIL_CALL),
        "the control no longer recovers the tail call at 0x401148:\n{}",
        control.code
    );
    let Some(run) = decompile_at(NEAR_MISS_ENTRY, &["--assert", "flow 0x40109a branch"])
    else {
        return;
    };
    assert!(run.success, "kuna decompile failed:\n{}", run.doc);
    assert!(
        run.doc.contains("\"status\": \"applied\""),
        "assertion was not applied:\n{}",
        run.doc
    );
    assert!(
        run.code.contains(NEAR_MISS_TAIL_CALL),
        "a branch assertion at 0x40109a suppressed the tail call at 0x401148:\n{}",
        run.code
    );
}

#[test]
fn refused_branch_fact_keeps_tail_call_recovery() {
    let Some(run) = decompile_at(NEAR_MISS_ENTRY, &["--assert", "flow 0x401148 branch"])
    else {
        return;
    };
    assert!(!run.success, "a refused assertion must fail the command:\n{}", run.doc);
    assert!(
        run.doc.contains("\"status\": \"rejected\""),
        "a branch fact on a jmp must be refused:\n{}",
        run.doc
    );
    assert!(
        run.code.contains(NEAR_MISS_TAIL_CALL),
        "a refused branch fact suppressed the tail call at 0x401148:\n{}",
        run.code
    );
}
