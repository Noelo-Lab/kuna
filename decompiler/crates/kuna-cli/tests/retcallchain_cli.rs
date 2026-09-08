//! End-to-end gate for the RET-call chain a `--assert flow <addr> call` extends
//! along — RE-need `flow-call-override-retain`.
//!
//! Fixture: `retcallchain_i386` (`retcallchain_i386.s` beside it is the source),
//! a 4.7 KiB ELF32 whose `chain_entry` is three `push <continuation>;
//! push <target>; ret` links — the idiom the round-9 crackme builds its whole
//! self-unpacking body out of, 22 links deep.
//!
//! This case is the promoted acceptance probe of that need (`tests/cli/`)
//! executed by `make rust-test`: overriding the FIRST link recovers all three
//! calls.  The `flow <the last ret> call` pass proves the clauses discriminate —
//! the plain epilogue is not a link, so nothing is extended and the body stays
//! empty.

use std::path::PathBuf;
use std::process::Command;

/// The `ret` of each link in `chain_entry`, and the plain epilogue after them.
const FIRST_LINK_RET: &str = "0x8049019";
const EPILOGUE_RET: &str = "0x8049032";

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

fn fixture() -> String {
    repo_root()
        .join("decompiler/crates/kuna-analysis/tests/fixtures/retcallchain_i386")
        .to_string_lossy()
        .into_owned()
}

fn specs_missing(stderr: &str) -> bool {
    stderr.contains("could not build an architecture")
        || stderr.contains("SLEIGH")
        || stderr.contains("Could not discover")
        || stderr.contains(".sla")
}

/// Decompile `chain_entry`, optionally under a `flow` assertion.  `None` ⇒ the
/// `.sla` tree is not built (a visible skip, never a false green).
fn decompile(assertion: Option<&str>) -> Option<String> {
    let binary = fixture();
    let sleigh = repo_root().join("specs").to_string_lossy().into_owned();
    let mut args: Vec<&str> =
        vec!["decompile", &binary, "chain_entry", "--sleighpath", &sleigh];
    if let Some(spec) = assertion {
        args.push("--assert");
        args.push(spec);
    }
    let out = Command::new(env!("CARGO_BIN_EXE_kuna")).args(&args).output().expect("spawn kuna");
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

/// The three callees the chain reaches, by the global each `ret` pops.
const CALLS: [&str; 3] = ["(*dat_804a000)()", "(*dat_804a004)()", "(*dat_804a008)()"];

/// Un-asserted, every `ret` is a return, so the body is empty.
#[test]
fn the_baseline_reads_the_whole_chain_as_one_return() {
    let Some(code) = decompile(None) else { return };
    for call in CALLS {
        assert!(!code.contains(call), "baseline already recovered {call}:\n{code}");
    }
}

/// The need's own case: one override on the first link recovers all three calls.
#[test]
fn overriding_the_first_link_recovers_the_whole_chain() {
    let Some(code) = decompile(Some(&format!("flow {FIRST_LINK_RET} call"))) else { return };
    for call in CALLS {
        assert!(code.contains(call), "the chain stopped before {call}:\n{code}");
    }
}

/// The clauses discriminate: the plain epilogue pushes no continuation, so the
/// override applies to it alone and extends to nothing.
#[test]
fn overriding_the_plain_epilogue_extends_to_nothing() {
    let Some(code) = decompile(Some(&format!("flow {EPILOGUE_RET} call"))) else { return };
    for call in CALLS {
        assert!(!code.contains(call), "a non-link override recovered {call}:\n{code}");
    }
}
