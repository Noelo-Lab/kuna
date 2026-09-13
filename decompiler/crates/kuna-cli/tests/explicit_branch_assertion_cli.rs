//! End-to-end precedence coverage for an explicit intraprocedural flow assertion
//! at a branch that also targets a discovered function entry.

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

fn decompile(extra: &[&str]) -> Option<(String, String)> {
    let binary = fixture();
    let sleigh = repo_root().join("specs").to_string_lossy().into_owned();
    let mut args = vec![
        "decompile",
        &binary,
        "0x40d120",
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
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    if !out.status.success() {
        if specs_missing(&stderr) {
            eprintln!("skipping: specs not built ({stderr})");
            return None;
        }
        panic!("kuna decompile failed: {stderr}\n{stdout}");
    }
    let code = json_string_field(&stdout, "code")
        .unwrap_or_else(|| panic!("JSON result has no function code: {stdout}"))
        .to_string();
    Some((stdout, code))
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
