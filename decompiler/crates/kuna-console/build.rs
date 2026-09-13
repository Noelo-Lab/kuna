//! The source fingerprint behind the `kuna` <-> engine-binary build handshake
//! (`src/kuna_buildstamp.rs`).
//!
//! Hashes the sources of this crate and of every workspace crate it links
//! (followed through `[dependencies]`), plus `Cargo.lock`, into
//! `KUNA_SOURCE_FINGERPRINT`. It hashes content, not mtimes or git state: a
//! checkout's debug and release builds agree, two checkouts of one tree agree, a
//! tarball with no `.git` still has one, and a binary left behind by an earlier
//! source state does not match. `tests`/`benches`/`examples` are neither hashed
//! nor watched, so editing a test does not rerun this script or recompile the
//! crate; every watched path is one whose change recompiles this crate anyway.
//! CR bytes are skipped so a CRLF checkout hashes like an LF one.

use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const UNHASHED_DIRS: [&str; 4] = ["tests", "benches", "examples", "target"];

fn main() {
    let manifest = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap_or_default());
    let crates = manifest.parent().map(Path::to_path_buf).unwrap_or_default();
    let mut files: Vec<(String, PathBuf)> = Vec::new();
    for name in linked_crates(&crates, "kuna-console") {
        collect(&crates.join(&name), &name, true, &mut files);
    }
    if let Some(workspace) = crates.parent() {
        let lock = workspace.join("Cargo.lock");
        if lock.is_file() {
            println!("cargo:rerun-if-changed={}", lock.display());
            files.push(("Cargo.lock".to_string(), lock));
        }
    }
    files.sort();

    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    let mut feed = |b: u8| hash = (hash ^ u64::from(b)).wrapping_mul(0x0000_0100_0000_01b3);
    for (rel, path) in &files {
        rel.bytes().for_each(&mut feed);
        feed(0);
        if let Ok(data) = fs::read(path) {
            data.iter()
                .copied()
                .filter(|&b| b != b'\r')
                .for_each(&mut feed);
        }
        feed(0);
    }
    println!("cargo:rustc-env=KUNA_SOURCE_FINGERPRINT={hash:016x}");
}

/// `root` and the workspace crates reachable from it through `[dependencies]`
/// (and `[target.*.dependencies]`) -- what a binary linking `root` is built from.
fn linked_crates(crates: &Path, root: &str) -> BTreeSet<String> {
    let mut seen = BTreeSet::new();
    let mut todo = vec![root.to_string()];
    while let Some(name) = todo.pop() {
        if !seen.insert(name.clone()) {
            continue;
        }
        let Ok(manifest) = fs::read_to_string(crates.join(&name).join("Cargo.toml")) else {
            continue;
        };
        let mut in_deps = false;
        for line in manifest.lines().map(str::trim) {
            if line.starts_with('[') {
                in_deps = line == "[dependencies]"
                    || (line.starts_with("[target.") && line.ends_with(".dependencies]"));
                continue;
            }
            let key: String = line
                .chars()
                .take_while(|c| c.is_ascii_alphanumeric() || *c == '-' || *c == '_')
                .collect();
            if in_deps && key.starts_with("kuna-") && crates.join(&key).join("Cargo.toml").is_file()
            {
                todo.push(key);
            }
        }
    }
    seen
}

/// Every regular file under `dir`, keyed by its `/`-joined path relative to the
/// crates directory. At a crate's top level the unhashed directories are skipped
/// and each kept entry is registered with cargo as a rerun trigger.
fn collect(dir: &Path, rel: &str, top: bool, files: &mut Vec<(String, PathBuf)>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().into_owned();
        if name.starts_with('.') || (top && UNHASHED_DIRS.contains(&name.as_str())) {
            continue;
        }
        let path = entry.path();
        if top {
            println!("cargo:rerun-if-changed={}", path.display());
        }
        let rel = format!("{rel}/{name}");
        if path.is_dir() {
            collect(&path, &rel, false, files);
        } else if path.is_file() {
            files.push((rel, path));
        }
    }
}
