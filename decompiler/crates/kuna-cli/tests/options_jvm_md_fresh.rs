//! Freshness fence for the generated JVM option catalog: `docs/options-jvm.md`
//! must be byte-identical to what `kuna catalog --markdown --engine jvm` emits.
//! Regenerate after any `decompiler/nonnative/kuna-jvm/options.toml` change:
//!
//! ```sh
//! decompiler/target/release/kuna catalog --markdown --engine jvm > docs/options-jvm.md
//! ```

use std::process::Command;

const COMMITTED: &str = include_str!("../../../../docs/options-jvm.md");

#[test]
fn options_jvm_md_matches_the_generator_byte_for_byte() {
    let out = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args(["catalog", "--markdown", "--engine", "jvm"])
        .output()
        .expect("spawn kuna");
    assert!(out.status.success(), "{}", String::from_utf8_lossy(&out.stderr));
    let generated = String::from_utf8(out.stdout).expect("UTF-8 markdown");
    assert!(
        generated == COMMITTED,
        "docs/options-jvm.md is stale -- regenerate with \
         `decompiler/target/release/kuna catalog --markdown --engine jvm > docs/options-jvm.md` \
         (generated {} bytes, committed {} bytes)",
        generated.len(),
        COMMITTED.len()
    );
}
