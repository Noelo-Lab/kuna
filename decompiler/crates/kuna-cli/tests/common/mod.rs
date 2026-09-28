#![allow(dead_code)]

pub mod process;

use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_SCRATCH_ID: AtomicU64 = AtomicU64::new(0);

pub fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .unwrap()
}

pub fn fixture(name: &str) -> String {
    repo_root()
        .join("decompiler/crates/kuna-analysis/tests/fixtures")
        .join(name)
        .to_str()
        .unwrap()
        .to_owned()
}

pub fn run_kuna(args: &[&str]) -> (String, String, i32) {
    let output = Command::new(env!("CARGO_BIN_EXE_kuna"))
        .args(args)
        .output()
        .expect("spawn kuna");
    (
        String::from_utf8(output.stdout).expect("UTF-8 stdout"),
        String::from_utf8(output.stderr).expect("UTF-8 stderr"),
        output.status.code().expect("kuna exited without a signal"),
    )
}

/// A unique generated-fixture path under cargo's own per-target scratch
/// directory, which is ignored and cleaned with the build.
pub fn scratch_file(stem: &str, extension: &str) -> PathBuf {
    let scratch_root = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("kuna-cli");
    std::fs::create_dir_all(&scratch_root).expect("create test scratch directory");
    let id = NEXT_SCRATCH_ID.fetch_add(1, Ordering::Relaxed);
    scratch_root.join(format!("{stem}-{}-{id}.{extension}", std::process::id()))
}
