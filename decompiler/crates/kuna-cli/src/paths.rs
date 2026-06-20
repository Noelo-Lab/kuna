//! Locate the kuna repo root and the built engine binaries — the Rust port of
//! `kuna/paths.py`.
//!
//! The `kuna` binary normally lives at `decompiler/target/<profile>/kuna`, so its
//! sibling engine binaries (`decomp_dbg`, `decomp_test_dbg`, `slacomp`) sit in
//! the same directory and the repo root is three levels up.  Environment
//! overrides (`KUNA_ROOT`, `KUNA_SPECS`, `KUNA_DECOMP_DBG`, …) are honored
//! exactly as the Python `paths.py` honored them, so out-of-tree builds and the
//! `/tmp` baseline tree keep working.

use std::env;
use std::path::PathBuf;

/// The directory the running `kuna` binary lives in (its built-binary siblings).
fn exe_dir() -> Option<PathBuf> {
    let exe = env::current_exe().ok()?;
    exe.parent().map(|p| p.to_path_buf())
}

/// The kuna repository root (override with `KUNA_ROOT`).
///
/// Without the env var, derive it from the binary location: `kuna` lives at
/// `<root>/decompiler/target/<profile>/kuna`, so the root is three parents up.
/// Falls back to the current directory if that walk fails.
pub fn repo_root() -> PathBuf {
    if let Ok(env_root) = env::var("KUNA_ROOT") {
        if !env_root.is_empty() {
            return PathBuf::from(env_root);
        }
    }
    if let Some(dir) = exe_dir() {
        // <root>/decompiler/target/<profile>/<exe>  ->  pop profile, target, decompiler.
        if let Some(root) = dir.parent().and_then(|p| p.parent()).and_then(|p| p.parent()) {
            return root.to_path_buf();
        }
    }
    env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

/// Root passed as `-sleighpath` / `SLEIGHHOME` (contains `Ghidra/Processors/...`).
pub fn specs_dir() -> PathBuf {
    if let Ok(env_specs) = env::var("KUNA_SPECS") {
        if !env_specs.is_empty() {
            return PathBuf::from(env_specs);
        }
    }
    repo_root().join("specs")
}

/// The XML datatest corpus directory.
pub fn datatests_dir() -> PathBuf {
    repo_root().join("tests").join("datatests")
}

/// Resolve a built engine binary, honoring a per-binary env override.
///
/// Resolution order (mirroring `paths.binary` in the Rust-only world): an
/// explicit `<env_var>` override always wins; otherwise the binary is the
/// `kuna` binary's sibling (`exe_dir()/<name>`), falling back to
/// `decompiler/target/<KUNA_RUST_PROFILE or release>/<name>` under the repo root.
pub fn binary(name: &str, env_var: &str) -> PathBuf {
    if let Ok(over) = env::var(env_var) {
        if !over.is_empty() {
            return PathBuf::from(over);
        }
    }
    if let Some(dir) = exe_dir() {
        let sibling = dir.join(name);
        if sibling.exists() {
            return sibling;
        }
    }
    let profile = env::var("KUNA_RUST_PROFILE").unwrap_or_else(|_| "release".to_string());
    repo_root().join("decompiler").join("target").join(profile).join(name)
}

pub fn decomp_dbg() -> PathBuf {
    binary("decomp_dbg", "KUNA_DECOMP_DBG")
}

pub fn decomp_test_dbg() -> PathBuf {
    binary("decomp_test_dbg", "KUNA_DECOMP_TEST")
}

pub fn slacomp() -> PathBuf {
    binary("slacomp", "KUNA_SLACOMP")
}
