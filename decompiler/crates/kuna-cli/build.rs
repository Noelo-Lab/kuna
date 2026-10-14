//! With the `embed-specs` feature, generate `$OUT_DIR/embedded_specs.rs`: an
//! `include_bytes!` table of the repo's compiled SLEIGH tree (every file under
//! `specs/` except the SLEIGH sources) and a content id naming that table
//! (`src/embedded_specs.rs`). Without the feature this does nothing, so a
//! checkout builds before `make specs` has produced any `.sla`.

use std::env;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};

const SKIPPED_EXTENSIONS: &[&str] = &["slaspec", "sinc", "lock"];

const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;

fn fnv(mut hash: u64, bytes: &[u8]) -> u64 {
    for &b in bytes {
        hash ^= u64::from(b);
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    hash
}

/// Every embeddable file under `dir`, as (`/`-joined path relative to `root`, path).
fn collect(root: &Path, dir: &Path, out: &mut Vec<(String, PathBuf)>) {
    let entries = fs::read_dir(dir)
        .unwrap_or_else(|e| panic!("embed-specs: cannot read {}: {e}", dir.display()));
    for entry in entries {
        let path = entry.expect("embed-specs: directory entry").path();
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or_default();
        if name.starts_with('.') {
            continue;
        }
        if path.is_dir() {
            collect(root, &path, out);
            continue;
        }
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or_default();
        if !path.is_file() || SKIPPED_EXTENSIONS.contains(&ext) {
            continue;
        }
        let rel = path.strip_prefix(root).expect("embed-specs: path under the root");
        let rel = rel.iter().map(|c| c.to_str().expect("embed-specs: UTF-8 path")).collect::<Vec<_>>();
        out.push((rel.join("/"), path));
    }
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    if env::var_os("CARGO_FEATURE_EMBED_SPECS").is_none() {
        return;
    }
    let manifest = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    let root = manifest.join("..").join("..").join("..").join("specs");
    println!("cargo:rerun-if-changed={}", root.display());

    let mut files = Vec::new();
    collect(&root, &root, &mut files);
    files.sort();
    assert!(
        files.iter().any(|(rel, _)| rel.ends_with(".sla")),
        "embed-specs: no compiled .sla under {} -- run `make specs` first",
        root.display()
    );

    let mut hash = FNV_OFFSET;
    let mut table = String::new();
    for (rel, path) in &files {
        let bytes = fs::read(path)
            .unwrap_or_else(|e| panic!("embed-specs: cannot read {}: {e}", path.display()));
        hash = fnv(hash, rel.as_bytes());
        hash = fnv(hash, &(bytes.len() as u64).to_le_bytes());
        hash = fnv(hash, &bytes);
        let source = path.to_str().expect("embed-specs: UTF-8 path");
        writeln!(table, "    ({rel:?}, include_bytes!({source:?})),").unwrap();
    }
    let generated = format!(
        "pub const ID: &str = \"{hash:016x}\";\n\
         pub static FILES: &[(&str, &[u8])] = &[\n{table}];\n"
    );
    let out = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR")).join("embedded_specs.rs");
    fs::write(&out, generated)
        .unwrap_or_else(|e| panic!("embed-specs: cannot write {}: {e}", out.display()));
}
