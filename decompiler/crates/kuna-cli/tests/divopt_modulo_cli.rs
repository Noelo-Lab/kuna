//! Regression gate for the shared reciprocal multiply used by GCC for `x / 3`
//! and `x % 3` in the same expression.

use std::path::PathBuf;
use std::process::Command;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..").canonicalize().unwrap()
}

#[test]
fn shared_div3_reciprocal_recovers_the_modulo() {
    let root = repo_root();
    let specs = std::env::var_os("KUNA_SPECS").map(PathBuf::from)
        .unwrap_or_else(|| root.join("specs"));
    if !specs.join("Ghidra/Processors/x86/data/languages/x86-64.sla").exists() {
        eprintln!("divopt modulo: skipping (no `.sla`; run `make specs`)");
        return;
    }
    let output = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args(["decompile", root.join("tests/bug-repro/sort").to_str().unwrap(),
               "--addr", "0xd040", "--sleighpath", specs.to_str().unwrap()])
        .output()
        .expect("spawn kuna");
    let code = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "decompile failed: {stderr}");
    // The modulo must land in a local; that local's NUMBER is not part of the
    // contract -- it shifts whenever a declaration ahead of it is added or
    // removed (`mulblob`, for one).
    let modulo_to_local = code.lines().any(|l| {
        let l = l.trim();
        l.ends_with(" = a0 % 3;")
            && l.strip_suffix(" = a0 % 3;")
                .is_some_and(|lhs| lhs.starts_with('v') && lhs[1..].chars().all(|c| c.is_ascii_digit()))
    });
    assert!(modulo_to_local, "modulo was not recovered:\n{code}");
    assert!(!code.contains("0xfffffffffffffffe"), "raw reciprocal term survived:\n{code}");
}
