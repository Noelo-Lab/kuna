//! (kuna decompile-all memory limit) The per-function resident-memory limit.
//!
//! `--max-fn-memory N` is driver policy beside `--max-fn-seconds`: a driver
//! sets one process-wide limit, every decompile drive arms it ([`Armed`]), and
//! the watchdog's cooperative probes ask [`over`] whether the drive must stop.
//! [`over`] reads the process's resident set at most once per
//! [`SAMPLE_EVERY`], so a probe between samples is one thread-local read. A
//! drive trips once the process is over the limit AND has grown since the
//! drive began, so memory an earlier function left resident never fails the
//! next one. The trip holds for the rest of the drive, and the drive hands
//! its freed pages back to the system on the way out.

use std::cell::Cell;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

/// The environment variable `kuna decompile` hands the limit (MiB) to its
/// `decomp_dbg` child in.
pub const ENV_VAR: &str = "KUNA_MAX_FN_MEMORY";

/// How often an armed drive reads the resident set.
const SAMPLE_EVERY: Duration = Duration::from_millis(50);

const MIB: u64 = 1 << 20;

static LIMIT_MIB: AtomicU64 = AtomicU64::new(0);

#[derive(Clone, Copy)]
struct Drive {
    start: u64,
    limit: u64,
    next: Instant,
    tripped: bool,
}

thread_local! {
    static DRIVE: Cell<Option<Drive>> = const { Cell::new(None) };
}

/// Set the process-wide limit in MiB; `0` turns it off.
pub fn set_limit_mib(mib: u64) {
    LIMIT_MIB.store(mib, Ordering::Relaxed);
}

/// The process-wide limit in MiB, `0` when off.
pub fn limit_mib() -> u64 {
    LIMIT_MIB.load(Ordering::Relaxed)
}

/// Can this platform read the process's resident set?
pub fn supported() -> bool {
    resident_bytes().is_some()
}

/// The process's resident set (`VmRSS`) in bytes, or `None` where the
/// platform does not report it.
pub fn resident_bytes() -> Option<u64> {
    let status = std::fs::read_to_string("/proc/self/status").ok()?;
    let line = status.lines().find_map(|l| l.strip_prefix("VmRSS:"))?;
    let kib: u64 = line.trim().strip_suffix("kB")?.trim().parse().ok()?;
    Some(kib * 1024)
}

/// Arms the limit for one drive and restores the enclosing state when dropped,
/// releasing free heap pages first if this drive tripped.
pub struct Armed {
    outer: Option<Drive>,
    armed: bool,
}

impl Armed {
    /// Arm the process-wide limit for the drive about to run: a no-op guard
    /// when no limit is set or the resident set cannot be read.
    pub fn enter() -> Armed {
        Armed::enter_with(limit_mib())
    }

    fn enter_with(limit_mib: u64) -> Armed {
        let drive = (limit_mib > 0).then(resident_bytes).flatten().map(|start| Drive {
            start,
            limit: limit_mib.saturating_mul(MIB),
            next: Instant::now() + SAMPLE_EVERY,
            tripped: false,
        });
        let outer = DRIVE.with(|d| d.replace(drive));
        Armed { outer, armed: drive.is_some() }
    }

    /// Is a limit armed for this drive?
    pub fn is_armed(&self) -> bool {
        self.armed
    }
}

impl Drop for Armed {
    fn drop(&mut self) {
        let tripped = tripped();
        DRIVE.with(|d| d.set(self.outer));
        if tripped {
            release_free_pages();
        }
    }
}

/// Is the armed drive over its limit? Reads the resident set at most once per
/// [`SAMPLE_EVERY`]; once over, stays over for the rest of the drive.
pub fn over(now: Instant) -> bool {
    DRIVE.with(|d| {
        let Some(mut drive) = d.get() else {
            return false;
        };
        if drive.tripped || now < drive.next {
            return drive.tripped;
        }
        drive.next = now + SAMPLE_EVERY;
        if let Some(rss) = resident_bytes() {
            drive.tripped = rss > drive.limit && rss > drive.start;
        }
        d.set(Some(drive));
        drive.tripped
    })
}

/// Did the armed drive go over its limit?
pub fn tripped() -> bool {
    tripped_limit_mib().is_some()
}

/// The limit (MiB) the armed drive went over, if it did.
pub fn tripped_limit_mib() -> Option<u64> {
    DRIVE.with(|d| d.get().filter(|drive| drive.tripped).map(|drive| drive.limit / MIB))
}

#[cfg(all(target_os = "linux", target_env = "gnu"))]
unsafe extern "C" {
    fn malloc_trim(pad: usize) -> std::ffi::c_int;
}

/// Return the free pages a tripped drive left in the heap, so the next
/// function (and every other process under the same memory cap) gets them.
fn release_free_pages() {
    #[cfg(all(target_os = "linux", target_env = "gnu"))]
    // SAFETY: `malloc_trim` takes a byte count and only releases free pages.
    unsafe {
        malloc_trim(0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_unarmed_drive_never_trips() {
        let guard = Armed::enter_with(0);
        assert!(!guard.is_armed());
        assert!(!over(Instant::now() + Duration::from_secs(1)));
        drop(guard);
        assert!(!tripped());
    }

    #[test]
    fn a_drive_trips_once_it_grows_past_the_limit() {
        if !supported() {
            return;
        }
        let guard = Armed::enter_with(1);
        assert!(guard.is_armed());
        let grown: Vec<u8> = vec![1; 128 << 20];
        let later = Instant::now() + 2 * SAMPLE_EVERY;
        assert!(over(later), "128 MiB of new pages past a 1 MiB limit");
        assert!(over(Instant::now()), "the trip holds for the rest of the drive");
        drop(grown);
        drop(guard);
        assert!(!tripped(), "dropping the guard restores the enclosing drive");
    }
}
