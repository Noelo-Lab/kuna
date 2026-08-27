use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_SCRATCH_ID: AtomicU64 = AtomicU64::new(0);

/// A unique generated-fixture path under cargo's own per-target scratch
/// directory, which is ignored and cleaned with the build.
pub fn scratch_file(stem: &str, extension: &str) -> PathBuf {
    let scratch_root = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("kuna-console");
    std::fs::create_dir_all(&scratch_root).expect("create test scratch directory");
    let id = NEXT_SCRATCH_ID.fetch_add(1, Ordering::Relaxed);
    scratch_root.join(format!("{stem}-{}-{id}.{extension}", std::process::id()))
}
