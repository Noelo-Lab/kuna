//! A PPC64 ELFv2 global entry derives its TOC pointer from `r12`, which the
//! ABI sets to the entry address, so TOC-relative globals resolve by name.

use crate::common;
use std::process::Command;

/// `publish(p) { escaped = p; }` from clang -O1 for powerpc64le:
/// `addis r2,r12,2; addi r2,r2,-32736; addis r4,r2,1; std r3,-32760(r4)`.
#[test]
fn a_global_entry_resolves_its_toc_relative_global() {
    let binary = common::fixture("publish_toc_ppc64le");
    let output = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .env("KUNA_SPECS", common::repo_root().join("specs"))
        .args(["decompile", &binary, "publish"])
        .output()
        .unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    let code = String::from_utf8(output.stdout).unwrap();
    assert!(code.contains("escaped = a0;"), "{code}");
}
