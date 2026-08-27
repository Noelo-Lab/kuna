use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_SCRATCH_ID: AtomicU64 = AtomicU64::new(0);

fn source_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../..")
        .canonicalize()
        .expect("canonical source root")
}

/// Return a unique generated-fixture path without falling back to the system
/// temporary directory. Coordination worktrees use the outer repository's
/// ignored `tmp/`; a standalone clone uses Cargo's ignored target directory.
pub fn scratch_file(stem: &str, extension: &str) -> PathBuf {
    let source = source_root();
    let scratch_root = source
        .ancestors()
        .find(|candidate| candidate.join("TODO.md").is_file() && candidate.join("kuna").is_dir())
        .map(|outer| outer.join("tmp/kuna-tests/kuna-cli"))
        .unwrap_or_else(|| source.join("decompiler/target/test-tmp/kuna-cli"));
    std::fs::create_dir_all(&scratch_root).expect("create repository-local test scratch");
    let id = NEXT_SCRATCH_ID.fetch_add(1, Ordering::Relaxed);
    scratch_root.join(format!("{stem}-{}-{id}.{extension}", std::process::id()))
}
