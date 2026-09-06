//! Locate the built engine binaries and the compiled SLEIGH tree — the Rust port
//! of `kuna/paths.py`.
//!
//! Two installations are first-class, and neither is a degraded form of the other:
//!
//! * **a repo checkout** — `kuna` is built to
//!   `<root>/decompiler/target/[<triple>/]<profile>/kuna`, so the repo root is the
//!   directory above the `decompiler`/`target` pair and `specs/` sits under it;
//! * **a release archive** — `kuna`, `decomp_dbg` and `slacomp` are siblings in one
//!   extracted directory and the separately downloaded `specs/` tree sits beside or
//!   inside it. There is no repo root there, so nothing may be derived from one:
//!   popping three parents off the archive directory lands on a path that exists
//!   nowhere.
//!
//! Every binary probe tries the bare name and the name plus the platform's
//! executable suffix, so the Windows archive's `decomp_dbg.exe` resolves through
//! the same code that resolves a unix `decomp_dbg`.  The environment overrides
//! (`KUNA_ROOT`, `KUNA_SPECS`, `KUNA_DECOMP_DBG`, `KUNA_DECOMP_TEST`,
//! `KUNA_SLACOMP`, `KUNA_RUST_PROFILE` — all documented in `docs/cli.md`) win over
//! both layouts.

use std::env;
use std::path::{Path, PathBuf};

/// A non-empty environment variable read as a path (`OsString`, so a Windows
/// path that is not valid UTF-8 still resolves).
fn env_path(var: &str) -> Option<PathBuf> {
    env::var_os(var).filter(|v| !v.is_empty()).map(PathBuf::from)
}

/// The directory the running `kuna` binary lives in (its built-binary siblings).
fn exe_dir() -> Option<PathBuf> {
    let exe = env::current_exe().ok()?;
    exe.parent().map(|p| p.to_path_buf())
}

/// The repo root when `dir` really is an in-tree build directory.
///
/// In-tree the binary sits at `<root>/decompiler/target/[<triple>/]<profile>/kuna`,
/// so the marker is a `target` directory whose parent is `decompiler`; the walk
/// covers the cross-compiled (`--target <triple>`) and `deps/` forms too.  Any
/// other directory — an extracted release archive, `/usr/local/bin` — has no repo
/// root, and this returns `None` rather than inventing one.
fn in_tree_root(dir: &Path) -> Option<PathBuf> {
    let mut cur = Some(dir);
    while let Some(d) = cur {
        let is_target = d.file_name().is_some_and(|n| n == "target");
        let parent_name = d.parent().and_then(Path::file_name);
        if is_target && parent_name.is_some_and(|n| n == "decompiler") {
            return d.parent().and_then(Path::parent).map(Path::to_path_buf);
        }
        cur = d.parent();
    }
    None
}

/// The kuna repository root (override with `KUNA_ROOT`), or the current directory
/// when the running binary is not an in-tree build.
pub fn repo_root() -> PathBuf {
    if let Some(root) = env_path("KUNA_ROOT") {
        return root;
    }
    if let Some(root) = exe_dir().as_deref().and_then(in_tree_root) {
        return root;
    }
    env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

/// A directory holding a SLEIGH tree (`Ghidra/Processors/...`).
fn is_spec_tree(dir: &Path) -> bool {
    dir.join("Ghidra").join("Processors").is_dir()
}

/// Where `specs_dir()` looks when neither `KUNA_SPECS` nor `KUNA_ROOT` pins it:
/// the checkout's `specs/`, then the two places a release archive's separately
/// downloaded `specs/` tree lands (inside the extracted directory, or beside it),
/// then the current directory's.
fn specs_search_dirs() -> Vec<PathBuf> {
    let mut dirs: Vec<PathBuf> = Vec::new();
    let mut push = |d: PathBuf| {
        if !dirs.contains(&d) {
            dirs.push(d);
        }
    };
    if let Some(dir) = exe_dir() {
        if let Some(root) = in_tree_root(&dir) {
            push(root.join("specs"));
        }
        push(dir.join("specs"));
        if let Some(parent) = dir.parent() {
            push(parent.join("specs"));
        }
    }
    if let Ok(cwd) = env::current_dir() {
        push(cwd.join("specs"));
    }
    if dirs.is_empty() {
        dirs.push(PathBuf::from("specs"));
    }
    dirs
}

/// The first candidate that actually holds a SLEIGH tree.
fn first_spec_tree(dirs: &[PathBuf]) -> Option<PathBuf> {
    dirs.iter().find(|d| is_spec_tree(d)).cloned()
}

/// Root passed as `-sleighpath` / `SLEIGHHOME` (contains `Ghidra/Processors/...`).
///
/// `KUNA_SPECS` and `KUNA_ROOT` pin it outright; otherwise the first
/// [`specs_search_dirs`] entry that holds a tree wins, falling back to the first
/// candidate so the caller still has something to name.
pub fn specs_dir() -> PathBuf {
    if let Some(specs) = env_path("KUNA_SPECS") {
        return specs;
    }
    if let Some(root) = env_path("KUNA_ROOT") {
        return root.join("specs");
    }
    let dirs = specs_search_dirs();
    first_spec_tree(&dirs).unwrap_or_else(|| dirs[0].clone())
}

/// The resolved SLEIGH root, or a diagnostic naming every directory probed.
pub fn require_specs_dir() -> Result<PathBuf, String> {
    let dir = specs_dir();
    if is_spec_tree(&dir) {
        return Ok(dir);
    }
    for var in ["KUNA_SPECS", "KUNA_ROOT"] {
        if env_path(var).is_some() {
            return Err(format!(
                "no SLEIGH specs at {} ({var}) -- it holds no Ghidra/Processors tree",
                dir.display()
            ));
        }
    }
    Err(format!("no SLEIGH specs found in {} -- {}", joined(&specs_search_dirs()), SPECS_HINT))
}

/// How to point kuna at a SLEIGH tree, for the failures that surface after the
/// directory itself resolved (a checkout that never ran `make specs`).
pub const SPECS_HINT: &str =
    "set KUNA_SPECS to a specs/ directory (from the kuna-v<ver>-specs release \
     archive), or run `make specs` in a checkout";

/// The directories [`binary`] probes, in order.
fn binary_search_dirs() -> Vec<PathBuf> {
    let mut dirs: Vec<PathBuf> = Vec::new();
    let exe = exe_dir();
    if let Some(dir) = exe.clone() {
        dirs.push(dir);
    }
    let root = env_path("KUNA_ROOT").or_else(|| exe.as_deref().and_then(in_tree_root));
    if let Some(root) = root {
        let profile = env::var("KUNA_RUST_PROFILE").unwrap_or_else(|_| "release".to_string());
        let built = root.join("decompiler").join("target").join(profile);
        if !dirs.contains(&built) {
            dirs.push(built);
        }
    }
    dirs
}

/// The file names an engine binary can carry in `dir`: the bare name, and the
/// name plus `suffix` — `.exe` on Windows, where the release archive ships
/// `decomp_dbg.exe` and `Path::exists()` applies no suffix of its own.
fn probe_binary_with(dir: &Path, name: &str, suffix: &str) -> Option<PathBuf> {
    let bare = dir.join(name);
    if bare.is_file() {
        return Some(bare);
    }
    if suffix.is_empty() {
        return None;
    }
    let suffixed = dir.join(format!("{name}{suffix}"));
    suffixed.is_file().then_some(suffixed)
}

/// [`probe_binary_with`] against the host platform's executable suffix.
fn probe_binary(dir: &Path, name: &str) -> Option<PathBuf> {
    probe_binary_with(dir, name, env::consts::EXE_SUFFIX)
}

/// A comma-separated rendering of a probe list, for a diagnostic.
fn joined(dirs: &[PathBuf]) -> String {
    dirs.iter().map(|d| d.display().to_string()).collect::<Vec<_>>().join(", ")
}

/// The XML datatest corpus directory.
pub fn datatests_dir() -> PathBuf {
    repo_root().join("tests").join("datatests")
}

/// Resolve a built engine binary, honoring a per-binary env override.
///
/// An explicit `<env_var>` always wins; otherwise the binary is looked for in
/// each [`binary_search_dirs`] entry under both its bare and suffixed names.  When
/// no probe hits, the returned path is the one a checkout would build to, so the
/// caller can still test it for existence — the diagnostic to print in that case
/// is [`missing_binary`], which names what was actually probed.
pub fn binary(name: &str, env_var: &str) -> PathBuf {
    if let Some(over) = env_path(env_var) {
        return over;
    }
    let dirs = binary_search_dirs();
    for dir in &dirs {
        if let Some(found) = probe_binary(dir, name) {
            return found;
        }
    }
    match dirs.last() {
        Some(dir) => dir.join(name),
        None => PathBuf::from(name),
    }
}

/// The diagnostic for an engine binary no probe found: the directories actually
/// looked in, the override that pins one, and how to build it.
pub fn missing_binary(name: &str, env_var: &str, cargo_package: &str) -> String {
    if let Some(pinned) = env_path(env_var) {
        return format!("{name} not found at {} ({env_var})", pinned.display());
    }
    format!(
        "{name} not found in {} -- set {env_var} to point at it, or run `make binaries` \
         (or `cargo build --release -p {cargo_package}`)",
        joined(&binary_search_dirs())
    )
}

pub fn decomp_dbg() -> PathBuf {
    binary("decomp_dbg", "KUNA_DECOMP_DBG")
}

/// The [`missing_binary`] diagnostic for [`decomp_dbg`].
pub fn missing_decomp_dbg() -> String {
    missing_binary("decomp_dbg", "KUNA_DECOMP_DBG", "kuna-console")
}

pub fn decomp_test_dbg() -> PathBuf {
    binary("decomp_test_dbg", "KUNA_DECOMP_TEST")
}

pub fn slacomp() -> PathBuf {
    binary("slacomp", "KUNA_SLACOMP")
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A scratch directory, named after the calling test so parallel cases do not
    /// collide.
    fn sandbox(tag: &str) -> PathBuf {
        let dir = env::temp_dir().join(format!("kuna_paths_{}_{tag}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("create sandbox");
        dir
    }

    /// The Windows release archive ships `decomp_dbg.exe`; the probe must find it.
    /// Pinned on every platform by passing the suffix explicitly, because
    /// `EXE_SUFFIX` is empty on the box this usually runs on.
    #[test]
    fn the_probe_finds_a_windows_named_sibling() {
        let dir = sandbox("winexe");
        std::fs::write(dir.join("decomp_dbg.exe"), b"").expect("write the shipped name");
        assert_eq!(
            probe_binary_with(&dir, "decomp_dbg", ".exe"),
            Some(dir.join("decomp_dbg.exe")),
            "the Windows archive's decomp_dbg.exe is invisible to the sibling probe"
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// The bare name still wins where both exist, and a suffix-less platform is
    /// unaffected by the new candidate.
    #[test]
    fn the_bare_name_is_still_preferred() {
        let dir = sandbox("bare");
        std::fs::write(dir.join("decomp_dbg"), b"").expect("write");
        std::fs::write(dir.join("decomp_dbg.exe"), b"").expect("write");
        assert_eq!(probe_binary_with(&dir, "decomp_dbg", ".exe"), Some(dir.join("decomp_dbg")));
        assert_eq!(probe_binary_with(&dir, "decomp_dbg", ""), Some(dir.join("decomp_dbg")));
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// A directory is not a binary, and an absent one is not a hit.
    #[test]
    fn a_directory_is_not_a_binary() {
        let dir = sandbox("notafile");
        std::fs::create_dir_all(dir.join("decomp_dbg")).expect("create the decoy directory");
        assert_eq!(probe_binary_with(&dir, "decomp_dbg", ".exe"), None);
        assert_eq!(probe_binary_with(&dir, "slacomp", ".exe"), None);
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// The three-parent pop is only trusted where the layout it assumes is real.
    #[test]
    fn only_an_in_tree_layout_yields_a_repo_root() {
        assert_eq!(
            in_tree_root(Path::new("/r/decompiler/target/release")),
            Some(PathBuf::from("/r"))
        );
        assert_eq!(
            in_tree_root(Path::new("/r/decompiler/target/x86_64-pc-windows-msvc/release")),
            Some(PathBuf::from("/r"))
        );
        assert_eq!(
            in_tree_root(Path::new("/r/decompiler/target/debug/deps")),
            Some(PathBuf::from("/r"))
        );
    }

    /// The extracted release archive: three parents up is `C:\`, which is why the
    /// pop must be refused rather than reported.
    #[test]
    fn an_extracted_archive_has_no_repo_root() {
        assert_eq!(in_tree_root(Path::new("/c/kuna-v1.329-windows-x86_64")), None);
        assert_eq!(in_tree_root(Path::new("/usr/local/bin")), None);
        assert_eq!(in_tree_root(Path::new("/")), None);
        // `target` alone is not the marker -- its parent must be `decompiler`.
        assert_eq!(in_tree_root(Path::new("/r/other/target/release")), None);
    }

    /// The specs tree an archive user extracts beside the binaries is found.
    #[test]
    fn the_specs_probe_accepts_an_archive_layout() {
        let root = sandbox("specs");
        let bindir = root.join("kuna-v1.329-linux-x86_64");
        std::fs::create_dir_all(&bindir).expect("create the extracted binary dir");
        std::fs::create_dir_all(root.join("specs").join("Ghidra").join("Processors"))
            .expect("create the extracted specs tree");
        let candidates =
            vec![bindir.join("specs"), root.join("specs"), root.join("nowhere").join("specs")];
        assert_eq!(first_spec_tree(&candidates), Some(root.join("specs")));
        assert!(!is_spec_tree(&bindir.join("specs")));
        let _ = std::fs::remove_dir_all(&root);
    }
}
