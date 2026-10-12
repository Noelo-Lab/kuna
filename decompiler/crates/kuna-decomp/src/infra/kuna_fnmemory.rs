//! (kuna decompile-all memory limit) The per-function resident-memory limit.
//!
//! `--max-fn-memory N` is driver policy beside `--max-fn-seconds`: a driver
//! sets one process-wide limit, every decompile drive arms it ([`Armed`]), and
//! the watchdog's cooperative probes ask [`over`] whether the drive must stop.
//! A drive reads the process's resident set when it starts and then at most
//! once per [`SAMPLE_EVERY`], so a probe between samples is one thread-local
//! read. It trips once the process is over the limit AND the drive itself has
//! added at least [`MIN_GROWTH`] since it started ([`trips`]): what the batch
//! built up before a function began stops that function only if the function
//! adds [`MIN_GROWTH`] on top of it. A drive that starts within a tenth of
//! the limit first hands free heap pages back to the system and measures
//! again. The trip holds for the rest of the drive, which releases free pages
//! on the way out. [`default_limit_mib`] is the limit a driver uses when none
//! was given: a share of the process's cgroup memory cap, or off.

use std::cell::Cell;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

/// Test-only: the directory standing in for `/sys/fs/cgroup`.
pub const CGROUP_ROOT_ENV: &str = "KUNA_CGROUP_ROOT";

/// How often an armed drive reads the resident set.
const SAMPLE_EVERY: Duration = Duration::from_millis(50);

const MIB: u64 = 1 << 20;

/// What a drive must add to the resident set itself before the limit can
/// stop it.
pub const MIN_GROWTH: u64 = 64 * MIB;

static LIMIT_MIB: AtomicU64 = AtomicU64::new(0);

/// The resident set right after the last release at a drive's start, so a
/// process whose live data sits near the limit does not release again for
/// every function.
static RELEASED_AT: AtomicU64 = AtomicU64::new(0);

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
        let limit = limit_mib.saturating_mul(MIB);
        let start = (limit > 0).then(resident_bytes).flatten().map(|rss| {
            if rss > limit / 10 * 9 && rss >= RELEASED_AT.load(Ordering::Relaxed) + MIN_GROWTH {
                release_free_pages();
                let after = resident_bytes().unwrap_or(rss);
                RELEASED_AT.store(after, Ordering::Relaxed);
                after
            } else {
                rss
            }
        });
        let drive = start.map(|start| Drive {
            start,
            limit,
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
            RELEASED_AT.store(resident_bytes().unwrap_or(0), Ordering::Relaxed);
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
            drive.tripped = trips(rss, drive.start, drive.limit);
        }
        d.set(Some(drive));
        drive.tripped
    })
}

/// The trip rule: the process is over `limit` and the drive that started at
/// `start` has itself added at least [`MIN_GROWTH`].
pub fn trips(rss: u64, start: u64, limit: u64) -> bool {
    rss > limit && rss.saturating_sub(start) >= MIN_GROWTH
}

/// The limit (MiB) a driver uses when the caller gave none: the cgroup memory
/// cap the process runs under, less `max(512 MiB, cap / 10)`, split evenly
/// across the `processes` that decompile in the run. `0` (off) outside a
/// capped cgroup.
pub fn default_limit_mib(processes: u64) -> u64 {
    cgroup_memory_cap().map_or(0, |cap| limit_for_cap(cap, processes))
}

/// [`default_limit_mib`] for a known cap in bytes.
pub fn limit_for_cap(cap: u64, processes: u64) -> u64 {
    let usable = cap.saturating_sub((512 * MIB).max(cap / 10));
    usable / processes.max(1) / MIB
}

/// The smallest finite cgroup v2 `memory.max` on the path from this process's
/// cgroup to the root, in bytes.
pub fn cgroup_memory_cap() -> Option<u64> {
    let root = std::env::var(CGROUP_ROOT_ENV).unwrap_or_else(|_| "/sys/fs/cgroup".into());
    let membership = std::fs::read_to_string("/proc/self/cgroup").ok()?;
    cap_from(&membership, |dir| std::fs::read_to_string(format!("{root}{dir}/memory.max")).ok())
}

/// [`cgroup_memory_cap`] over `/proc/self/cgroup` text, with `read_max`
/// answering one cgroup directory's `memory.max` (`""` is the root, which has
/// one only inside a container's cgroup namespace).
pub fn cap_from(membership: &str, read_max: impl Fn(&str) -> Option<String>) -> Option<u64> {
    let mut dir = membership.lines().find_map(|l| l.strip_prefix("0::"))?.trim_end_matches('/');
    let mut cap: Option<u64> = None;
    loop {
        if let Some(bytes) = read_max(dir).and_then(|v| v.trim().parse::<u64>().ok()) {
            cap = Some(cap.map_or(bytes, |c| c.min(bytes)));
        }
        if dir.is_empty() {
            return cap;
        }
        dir = &dir[..dir.rfind('/').unwrap_or(0)];
    }
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
        let grown: Vec<u8> = vec![1; 256 << 20];
        let later = Instant::now() + 2 * SAMPLE_EVERY;
        assert!(over(later), "256 MiB of new pages past a 1 MiB limit");
        assert!(over(Instant::now()), "the trip holds for the rest of the drive");
        drop(grown);
        drop(guard);
        assert!(!tripped(), "dropping the guard restores the enclosing drive");
    }

    #[test]
    fn only_a_drive_that_grew_by_min_growth_trips() {
        let limit = 250 * MIB;
        assert!(!trips(240 * MIB, 100 * MIB, limit), "under the limit");
        assert!(!trips(260 * MIB, 240 * MIB, limit), "over the limit, but the batch put it there");
        assert!(!trips(260 * MIB, 270 * MIB, limit), "under its own start");
        assert!(trips(260 * MIB, 196 * MIB, limit));
        assert!(trips(300 * MIB, 100 * MIB, limit));
    }

    #[test]
    fn the_default_is_the_smallest_cgroup_cap_less_a_margin_per_process() {
        let membership = "0::/user.slice/wave.slice/run-1.scope\n";
        let caps = |dir: &str| match dir {
            "/user.slice" => Some("max\n".to_string()),
            "/user.slice/wave.slice" => Some("6442450944\n".to_string()),
            "/user.slice/wave.slice/run-1.scope" => Some("34359738368\n".to_string()),
            _ => None,
        };
        assert_eq!(cap_from(membership, caps), Some(6 << 30));
        assert_eq!(cap_from(membership, |_| Some("max".into())), None);
        assert_eq!(cap_from("1:memory:/user.slice\n", caps), None, "cgroup v1");
        let container = |dir: &str| (dir.is_empty()).then(|| "2147483648".to_string());
        assert_eq!(cap_from("0::/\n", container), Some(2 << 30), "a namespaced container's own cap");
        assert_eq!(limit_for_cap(6 << 30, 1), 5529);
        assert_eq!(limit_for_cap(2 << 30, 1), 1536);
        assert_eq!(limit_for_cap(32 << 30, 3), 9830);
        assert_eq!(limit_for_cap(256 << 20, 1), 0);
    }
}
