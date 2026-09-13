//! The source fingerprints behind the `kuna` <-> engine-binary build handshake
//! (`src/kuna_buildstamp.rs`).
//!
//! The engine fingerprint covers `kuna-console` and its production workspace
//! dependency graph. The harness fingerprint starts at `kuna-harness`, whose
//! `decomp_test_dbg` binary adds the datatest runner above that engine graph.
//! Both include the workspace manifest and lockfile, and both hash contents
//! rather than mtimes or git state so debug and release builds of one source
//! tree agree.

use std::env;
use std::path::PathBuf;

#[path = "src/kuna_buildstamp_fingerprint.rs"]
mod fingerprint;

fn main() {
    let manifest = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap_or_default());
    let workspace = manifest
        .parent()
        .and_then(|p| p.parent())
        .expect("kuna-console must live under <workspace>/crates");
    let engine = fingerprint::calculate(workspace, "kuna-console")
        .expect("failed to fingerprint the decomp_dbg source graph");
    let harness = fingerprint::calculate(workspace, "kuna-harness")
        .expect("failed to fingerprint the decomp_test_dbg source graph");
    for path in engine.watched.union(&harness.watched) {
        println!("cargo:rerun-if-changed={}", path.display());
    }
    println!(
        "cargo:rustc-env=KUNA_ENGINE_SOURCE_FINGERPRINT={:016x}",
        engine.value
    );
    println!(
        "cargo:rustc-env=KUNA_HARNESS_SOURCE_FINGERPRINT={:016x}",
        harness.value
    );
}
