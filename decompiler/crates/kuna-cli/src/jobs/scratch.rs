//! Owned worker scratch directories and the existing orphan-cleanup policy.

use std::path::{Path, PathBuf};
use std::time::Duration;

const SCRATCH_PREFIX: &str = "kuna-jobs-";

pub(super) struct ScratchDir {
    dir: tempfile::TempDir,
}

impl ScratchDir {
    pub(super) fn create() -> Result<Self, String> {
        Self::create_in(&std::env::temp_dir())
    }

    fn create_in(temp: &Path) -> Result<Self, String> {
        sweep_stale_scratch(temp);
        let create_error = |e| format!("cannot create the worker scratch dir in {}: {e}", temp.display());
        std::fs::create_dir_all(temp).map_err(create_error)?;
        let prefix = format!("{SCRATCH_PREFIX}{}-", std::process::id());
        let mut builder = tempfile::Builder::new();
        builder.prefix(&prefix);
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            builder.permissions(std::fs::Permissions::from_mode(0o700));
        }
        let dir = builder.tempdir_in(temp).map_err(create_error)?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(dir.path(), std::fs::Permissions::from_mode(0o700)).map_err(
                |e| {
                    format!(
                        "cannot restrict the worker scratch dir {}: {e}",
                        dir.path().display()
                    )
                },
            )?;
        }
        Ok(Self { dir })
    }

    pub(super) fn path(&self) -> &Path {
        self.dir.path()
    }
}

/// Remove remnants when `/proc` shows their owner is gone, with the existing
/// one-week age fallback on hosts without `/proc`.
fn sweep_stale_scratch(temp: &Path) {
    let Ok(entries) = std::fs::read_dir(temp) else { return };
    for entry in entries.flatten() {
        let name = entry.file_name();
        let Some(pid) = name.to_str().and_then(scratch_owner) else { continue };
        if !owner_is_gone(pid, &entry.path()) {
            continue;
        }
        let _ = std::fs::remove_dir_all(entry.path());
    }
}

/// Read the owner pid from `kuna-jobs-<pid>-<suffix>`.
fn scratch_owner(name: &str) -> Option<u32> {
    name.strip_prefix(SCRATCH_PREFIX)?.split('-').next()?.parse().ok()
}

fn owner_is_gone(pid: u32, path: &Path) -> bool {
    if Path::new("/proc/self/stat").exists() {
        return !Path::new(&format!("/proc/{pid}")).exists();
    }
    const WEEK: Duration = Duration::from_secs(7 * 24 * 3600);
    std::fs::metadata(path)
        .and_then(|m| m.modified())
        .and_then(|t| t.elapsed().map_err(|_| std::io::ErrorKind::Other.into()))
        .is_ok_and(|age| age > WEEK)
}

/// Recognize the scratch-name prefix used by the parent's assignment protocol.
pub(super) fn pool_scratch(dir: &str) -> Option<PathBuf> {
    let path = Path::new(dir);
    path.file_name()?.to_str()?.starts_with(SCRATCH_PREFIX).then(|| path.to_path_buf())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Cleanup is a `Drop`, not a step on the happy path: the directory holds the
    /// whole program's symbol inventory and every worker's decompiled C, and
    /// `run_pool` can also leave by an early `Err` or a panicking pool thread.
    #[test]
    fn the_scratch_dir_is_private_and_removed_on_every_path() {
        let kept = {
            let dir = ScratchDir::create().unwrap();
            let path = dir.path().to_path_buf();
            assert!(path.is_dir());
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let mode = std::fs::metadata(&path).unwrap().permissions().mode() & 0o777;
                assert_eq!(
                    mode, 0o700,
                    "worker C and symbols must not transit a world-readable dir"
                );
            }
            std::fs::write(path.join("inventory.spec"), b"payload").unwrap();
            path
        };
        assert!(!kept.exists(), "a non-empty scratch dir must go with its guard");

        let leaked = std::sync::Mutex::new(PathBuf::new());
        let _ = std::panic::catch_unwind(|| {
            let dir = ScratchDir::create().unwrap();
            *leaked.lock().unwrap() = dir.path().to_path_buf();
            panic!("pool thread died");
        });
        assert!(!leaked.lock().unwrap().exists());
    }

    /// A worker only ever deletes a directory the pool itself named.
    #[test]
    fn a_worker_recognizes_only_its_own_scratch_dir() {
        assert_eq!(
            pool_scratch("/tmp/kuna-jobs-4242-99"),
            Some(PathBuf::from("/tmp/kuna-jobs-4242-99"))
        );
        for foreign in ["/tmp", "/home/u/out", "kuna-jobs-1/nested", "/"] {
            assert_eq!(pool_scratch(foreign), None, "{foreign} is not a pool scratch dir");
        }
        assert_eq!(scratch_owner("kuna-jobs-4242-17384"), Some(4242));
        for bad in ["kuna-jobs-", "kuna-jobs-abc-1", "kunajobs-1-2", "tmpdir"] {
            assert_eq!(scratch_owner(bad), None, "{bad}");
        }
    }

    /// The sweep is the last resort for the one window a dying process cannot
    /// cover, so it must be exact about ownership: this process is alive, so its
    /// own directory is never a candidate.
    #[test]
    fn the_sweep_spares_a_live_owner() {
        let dir = ScratchDir::create().unwrap();
        let mine = dir.path().to_path_buf();
        let temp = mine.parent().unwrap().to_path_buf();
        let dead = temp.join(format!("{SCRATCH_PREFIX}{}-11", u32::MAX));
        let unrelated = temp.join(format!("kuna-not-a-job-{}", std::process::id()));
        std::fs::create_dir_all(&dead).unwrap();
        std::fs::create_dir_all(&unrelated).unwrap();

        sweep_stale_scratch(&temp);

        assert!(mine.is_dir(), "a live run's directory must survive the sweep");
        assert!(unrelated.is_dir(), "the sweep must not touch directories it did not create");
        if Path::new("/proc/self/stat").exists() {
            assert!(!dead.exists(), "a dead owner's directory must be swept");
        }
        let _ = std::fs::remove_dir_all(&dead);
        let _ = std::fs::remove_dir_all(&unrelated);
    }

    #[test]
    fn scratch_directories_have_independent_lifetimes() {
        let base = tempfile::tempdir().unwrap();
        let first = ScratchDir::create_in(base.path()).unwrap();
        let second = ScratchDir::create_in(base.path()).unwrap();
        let first_path = first.path().to_path_buf();
        let second_path = second.path().to_path_buf();
        assert_ne!(first_path, second_path);
        for path in [&first_path, &second_path] {
            assert_eq!(scratch_owner(path.file_name().unwrap().to_str().unwrap()), Some(std::process::id()));
            assert_eq!(pool_scratch(path.to_str().unwrap()), Some(path.clone()));
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                assert_eq!(std::fs::metadata(path).unwrap().permissions().mode() & 0o777, 0o700);
            }
        }
        std::fs::write(first.path().join("inventory.spec"), b"first").unwrap();
        std::fs::write(second.path().join("inventory.spec"), b"second").unwrap();
        drop(first);
        assert!(!first_path.exists());
        assert_eq!(std::fs::read(second.path().join("inventory.spec")).unwrap(), b"second");
        drop(second);
        assert!(!second_path.exists());
        assert_eq!(std::fs::read_dir(base.path()).unwrap().count(), 0);
    }

    #[test]
    fn scratch_creation_keeps_missing_parent_support() {
        let base = tempfile::tempdir().unwrap();
        let parent = base.path().join("missing").join("nested");
        let scratch = ScratchDir::create_in(&parent).unwrap();
        let path = scratch.path().to_path_buf();
        assert_eq!(path.parent(), Some(parent.as_path()));
        assert!(path.is_dir());
        drop(scratch);
        assert!(!path.exists());
        assert!(parent.is_dir());
    }

    #[test]
    fn scratch_creation_failure_preserves_the_parent_file() {
        let base = tempfile::tempdir().unwrap();
        let parent = base.path().join("file");
        std::fs::write(&parent, b"not a directory").unwrap();
        let error = match ScratchDir::create_in(&parent) {
            Ok(_) => panic!("a regular file is not a scratch parent"),
            Err(error) => error,
        };
        assert!(error.contains("cannot create the worker scratch dir"));
        assert_eq!(std::fs::read(parent).unwrap(), b"not a directory");
        assert_eq!(std::fs::read_dir(base.path()).unwrap().count(), 1);
    }
}
