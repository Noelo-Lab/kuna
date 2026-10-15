//! The compiled SLEIGH tree a release `kuna` carries inside itself (cargo feature
//! `embed-specs`; `build.rs` generates the table).
//!
//! The engine reads specs only through plain `std::fs` path reads, so the table is
//! unpacked once into the per-user cache directory, under a name derived from its
//! contents, and that directory is handed out like any other spec root. Each unpack
//! is written to a private directory and renamed into place only once complete, so a
//! reader never sees a partial tree and concurrent first runs settle on one copy.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU32, Ordering};

#[cfg(feature = "embed-specs")]
mod generated {
    include!(concat!(env!("OUT_DIR"), "/embedded_specs.rs"));
}

/// Written last into a finished unpack; a tree without it is not used.
const COMPLETE: &str = ".complete";

/// The unpacked tree, or why no cache location would take it. `None` when this
/// build carries no specs.
#[cfg(feature = "embed-specs")]
pub fn tree() -> Option<&'static Result<PathBuf, String>> {
    static TREE: std::sync::OnceLock<Result<PathBuf, String>> = std::sync::OnceLock::new();
    Some(TREE.get_or_init(|| unpack_into_cache(generated::ID, generated::FILES)))
}

#[cfg(not(feature = "embed-specs"))]
pub fn tree() -> Option<&'static Result<PathBuf, String>> {
    None
}

/// Unpack into the first cache location that accepts the tree.
#[cfg_attr(not(feature = "embed-specs"), allow(dead_code))]
fn unpack_into_cache(id: &str, files: &[(&str, &[u8])]) -> Result<PathBuf, String> {
    let mut failures = Vec::new();
    for base in cache_bases() {
        match unpack(&base, id, files) {
            Ok(dir) => return Ok(dir),
            Err(e) => failures.push(format!("{}: {e}", base.display())),
        }
    }
    Err(failures.join("; "))
}

/// Where unpacked trees live, most preferred first: the platform's per-user cache
/// directory, then the system temp directory.
#[cfg_attr(not(feature = "embed-specs"), allow(dead_code))]
fn cache_bases() -> Vec<PathBuf> {
    let var = |name: &str| {
        std::env::var_os(name).filter(|v| !v.is_empty()).map(PathBuf::from)
    };
    let user_cache = if cfg!(windows) {
        var("LOCALAPPDATA")
    } else if cfg!(target_os = "macos") {
        var("HOME").map(|home| home.join("Library").join("Caches"))
    } else {
        var("XDG_CACHE_HOME")
            .filter(|dir| dir.is_absolute())
            .or_else(|| var("HOME").map(|home| home.join(".cache")))
    };
    let user = std::env::var("USER").or_else(|_| std::env::var("USERNAME")).unwrap_or_default();
    let temp = if user.is_empty() { "kuna".to_string() } else { format!("kuna-{user}") };
    let mut bases: Vec<PathBuf> = user_cache.into_iter().map(|dir| dir.join("kuna")).collect();
    bases.push(std::env::temp_dir().join(temp));
    bases
}

/// The tree for `id` under `base`, unpacking `files` there first if no finished
/// copy exists.
pub(crate) fn unpack(base: &Path, id: &str, files: &[(&str, &[u8])]) -> Result<PathBuf, String> {
    let dir = base.join(format!("specs-{id}"));
    if dir.join(COMPLETE).is_file() {
        return Ok(dir);
    }
    fs::create_dir_all(base).map_err(|e| e.to_string())?;
    static STAGINGS: AtomicU32 = AtomicU32::new(0);
    let nonce = STAGINGS.fetch_add(1, Ordering::Relaxed);
    let staging = base.join(format!(".specs-{id}-{}-{nonce}", std::process::id()));
    if let Err(e) = write_tree(&staging, id, files) {
        let _ = fs::remove_dir_all(&staging);
        return Err(e);
    }
    if dir.is_dir() && !dir.join(COMPLETE).is_file() {
        let _ = fs::remove_dir_all(&dir);
    }
    let mut renamed = fs::rename(&staging, &dir);
    // A virus scanner can briefly hold files written a moment ago, which fails a
    // Windows directory rename with access denied.
    for wait_ms in [20, 80, 200] {
        if renamed.is_ok() || dir.join(COMPLETE).is_file() {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(wait_ms));
        renamed = fs::rename(&staging, &dir);
    }
    let _ = fs::remove_dir_all(&staging);
    if dir.join(COMPLETE).is_file() {
        return Ok(dir);
    }
    Err(match renamed {
        Err(e) => e.to_string(),
        Ok(()) => format!("{} has no {COMPLETE} marker", dir.display()),
    })
}

fn write_tree(staging: &Path, id: &str, files: &[(&str, &[u8])]) -> Result<(), String> {
    for (rel, bytes) in files {
        let path = rel.split('/').fold(staging.to_path_buf(), |p, part| p.join(part));
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| format!("{}: {e}", parent.display()))?;
        }
        fs::write(&path, bytes).map_err(|e| format!("{}: {e}", path.display()))?;
    }
    fs::write(staging.join(COMPLETE), id).map_err(|e| format!("{COMPLETE}: {e}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    const FILES: &[(&str, &[u8])] = &[
        ("Ghidra/Processors/x86/data/languages/x86.ldefs", b"<language_definitions/>"),
        ("Ghidra/Processors/x86/data/languages/x86-64.sla", b"\x00sla"),
    ];

    fn sandbox(tag: &str) -> PathBuf {
        let dir = std::env::temp_dir()
            .join(format!("kuna_embedded_specs_{}_{tag}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        dir
    }

    #[test]
    fn an_unpack_lays_out_the_tree_and_leaves_no_staging() {
        let base = sandbox("layout");
        let dir = unpack(&base, "0123", FILES).expect("unpack");
        assert_eq!(dir, base.join("specs-0123"));
        assert!(dir.join("Ghidra").join("Processors").is_dir());
        for (rel, bytes) in FILES {
            assert_eq!(fs::read(dir.join(rel)).unwrap(), *bytes, "{rel}");
        }
        let names: Vec<_> = fs::read_dir(&base).unwrap().map(|e| e.unwrap().file_name()).collect();
        assert_eq!(names, vec![std::ffi::OsString::from("specs-0123")]);
        let _ = fs::remove_dir_all(&base);
    }

    #[test]
    fn a_finished_tree_is_reused_untouched() {
        let base = sandbox("reuse");
        let dir = unpack(&base, "0123", FILES).expect("first unpack");
        fs::write(dir.join("sentinel"), b"kept").unwrap();
        assert_eq!(unpack(&base, "0123", FILES).expect("second unpack"), dir);
        assert!(dir.join("sentinel").is_file(), "a finished tree was rewritten");
        let _ = fs::remove_dir_all(&base);
    }

    #[test]
    fn a_tree_without_its_marker_is_replaced() {
        let base = sandbox("partial");
        let dir = base.join("specs-0123");
        fs::create_dir_all(dir.join("Ghidra")).unwrap();
        assert_eq!(unpack(&base, "0123", FILES).expect("unpack"), dir);
        assert!(dir.join(COMPLETE).is_file());
        assert!(dir.join(FILES[1].0).is_file());
        let _ = fs::remove_dir_all(&base);
    }

    #[test]
    fn concurrent_first_runs_settle_on_one_tree() {
        let base = sandbox("race");
        let dirs: Vec<PathBuf> = std::thread::scope(|s| {
            let runs: Vec<_> = (0..8).map(|_| s.spawn(|| unpack(&base, "0123", FILES))).collect();
            runs.into_iter().map(|r| r.join().unwrap().expect("unpack")).collect()
        });
        assert!(dirs.iter().all(|d| *d == base.join("specs-0123")));
        assert_eq!(fs::read_dir(&base).unwrap().count(), 1, "staging left behind");
        let _ = fs::remove_dir_all(&base);
    }

    #[test]
    fn distinct_contents_get_distinct_trees() {
        let base = sandbox("ids");
        let a = unpack(&base, "aaaa", FILES).expect("unpack a");
        let b = unpack(&base, "bbbb", &FILES[..1]).expect("unpack b");
        assert_ne!(a, b);
        assert!(!b.join(FILES[1].0).exists());
        let _ = fs::remove_dir_all(&base);
    }
}
