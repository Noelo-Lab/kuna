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
//! adds [`MIN_GROWTH`] on top of it. The trip holds for the rest of the
//! drive, which releases free pages on the way out.
//!
//! A driver given no limit uses the cgroup default instead
//! ([`use_cgroup_default`]) when the process runs under a finite cgroup v2
//! `memory.max`: a drive trips once it has added that cgroup's
//! [`cgroup_margin`] (less under a cap with little room left) AND the cgroup
//! holds more than it can reclaim beyond [`cgroup_limit`] ([`cgroup_trips`]).
//! What it compares is the whole cgroup's usage, so every process in a
//! container shares one budget and none is given a fixed slice of it. The
//! cgroup is read only on a sample that finds the drive grown that much.
//!
//! The allocator keeps a function's freed transients resident, so a batch
//! would hold the high-water of the largest function it has decompiled, and
//! measure the next function's growth from there. A drive therefore starts by
//! handing free heap pages back once the process holds [`ARMED_SLACK`] more
//! than the last release left under an explicit limit, or [`MIN_GROWTH`] more
//! otherwise, spending at most [`RELEASE_SHARE`] of the time on it ([`settle`]).

use std::cell::Cell;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Mutex;
use std::time::{Duration, Instant};

use crate::kuna_hostmem::{capped_cgroups, resident_bytes, unreclaimable_bytes};

/// How often an armed drive reads the resident set.
const SAMPLE_EVERY: Duration = Duration::from_millis(50);

const MIB: u64 = 1 << 20;

/// What a drive must add to the resident set itself before the limit can
/// stop it.
pub const MIN_GROWTH: u64 = 64 * MIB;

static LIMIT_MIB: AtomicU64 = AtomicU64::new(0);

/// Whether the cgroup default is in force, and its levels: each capped
/// cgroup's directory and its cap.
static CGROUP_DEFAULT: AtomicBool = AtomicBool::new(false);
static CGROUP_LEVELS: Mutex<Vec<(String, u64)>> = Mutex::new(Vec::new());

/// The resident set the last release left, or the lowest a drive started at
/// since; `u64::MAX` before the first drive.
static FLOOR: AtomicU64 = AtomicU64::new(u64::MAX);

/// When the last release ended ([`now_ns`]) and how long it took, in
/// nanoseconds.
static RELEASED_AT: AtomicU64 = AtomicU64::new(0);
static RELEASE_COST: AtomicU64 = AtomicU64::new(0);

/// How far over [`FLOOR`] a drive under a limit may start before free pages
/// are handed back: freed pages a function cannot reuse would otherwise be
/// charged to it.
pub const ARMED_SLACK: u64 = 16 * MIB;

/// With no limit armed, a release takes at most one part in this many of the
/// time since the last one.
const RELEASE_SHARE: u64 = 32;

/// Can this platform's allocator hand free pages back ([`release_free_pages`])?
const RELEASES: bool = cfg!(all(target_os = "linux", target_env = "gnu"));

#[derive(Clone, Copy)]
struct Drive {
    start: u64,
    limit: u64,
    next: Instant,
    tripped: bool,
    cgroup: bool,
}

thread_local! {
    static DRIVE: Cell<Option<Drive>> = const { Cell::new(None) };
}

/// Set the process-wide limit in MiB; `0` turns it off. Either way the
/// cgroup default is off.
pub fn set_limit_mib(mib: u64) {
    CGROUP_DEFAULT.store(false, Ordering::Relaxed);
    LIMIT_MIB.store(mib, Ordering::Relaxed);
}

/// Use the cgroup default in place of a per-process limit; it is in force
/// only under a finite cgroup v2 `memory.max` on a platform that reports the
/// resident set. Returns whether it is.
pub fn use_cgroup_default() -> bool {
    LIMIT_MIB.store(0, Ordering::Relaxed);
    let levels = capped_cgroups();
    let on = !levels.is_empty() && supported();
    *CGROUP_LEVELS.lock().unwrap_or_else(|e| e.into_inner()) = levels;
    CGROUP_DEFAULT.store(on, Ordering::Relaxed);
    on
}

/// The cgroup default's margin for a cap: `max(512 MiB, cap / 10)`, never
/// more than the cap.
pub fn cgroup_margin(cap: u64) -> u64 {
    (512 * MIB).max(cap / 10).min(cap)
}

/// What a capped cgroup may hold before a function in it can be stopped: its
/// cap less [`cgroup_margin`].
pub fn cgroup_limit(cap: u64) -> u64 {
    cap - cgroup_margin(cap)
}

/// The process-wide limit in MiB, `0` when off.
pub fn limit_mib() -> u64 {
    LIMIT_MIB.load(Ordering::Relaxed)
}

/// Can this platform read the process's resident set?
pub fn supported() -> bool {
    resident_bytes().is_some()
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
        if CGROUP_DEFAULT.load(Ordering::Relaxed) {
            return Armed::enter_with(0, true);
        }
        Armed::enter_with(limit_mib().saturating_mul(MIB), false)
    }

    /// Arm a drive against `limit` bytes of resident set, or under the cgroup
    /// default. Either way the drive first [`settle`]s.
    fn enter_with(limit: u64, cgroup: bool) -> Armed {
        let armed = limit > 0 || cgroup;
        let start = settle(armed, limit > 0).filter(|_| armed);
        Armed::arm(start.map(|start| Drive {
            start,
            limit,
            next: Instant::now() + SAMPLE_EVERY,
            tripped: false,
            cgroup,
        }))
    }

    fn arm(drive: Option<Drive>) -> Armed {
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
            release();
        }
    }
}

/// Hand free heap pages back before a drive once the process holds
/// [`ARMED_SLACK`] more than [`FLOOR`] under an explicit limit (`explicit`), or
/// otherwise [`MIN_GROWTH`] more when the last release was over
/// [`RELEASE_SHARE`] times its own cost ago. Returns the resident set the drive
/// starts at; `None` when it cannot be read, or when the drive is not
/// `measured` on a platform whose allocator cannot release.
pub fn settle(measured: bool, explicit: bool) -> Option<u64> {
    if !measured && !RELEASES {
        return None;
    }
    let rss = resident_bytes()?;
    let floor = FLOOR.fetch_min(rss, Ordering::Relaxed).min(rss);
    let since = now_ns().saturating_sub(RELEASED_AT.load(Ordering::Relaxed));
    if !RELEASES || !release_due(rss, floor, explicit, since, RELEASE_COST.load(Ordering::Relaxed)) {
        return Some(rss);
    }
    Some(release().unwrap_or(rss))
}

/// The release rule: a process holding `rss` over the `floor` the last release
/// left by [`ARMED_SLACK`] under an explicit limit, or otherwise by
/// [`MIN_GROWTH`] when `since` the last release is over [`RELEASE_SHARE`] times
/// its `cost` (both in nanoseconds).
pub fn release_due(rss: u64, floor: u64, explicit: bool, since: u64, cost: u64) -> bool {
    let grown = rss.saturating_sub(floor);
    if explicit {
        grown >= ARMED_SLACK
    } else {
        grown >= MIN_GROWTH && since >= cost.saturating_mul(RELEASE_SHARE)
    }
}

/// Hand free heap pages back and make what is left the new [`FLOOR`].
fn release() -> Option<u64> {
    let began = now_ns();
    release_free_pages();
    let done = now_ns();
    RELEASED_AT.store(done, Ordering::Relaxed);
    RELEASE_COST.store(done - began, Ordering::Relaxed);
    let after = resident_bytes()?;
    FLOOR.store(after, Ordering::Relaxed);
    Some(after)
}

fn now_ns() -> u64 {
    static EPOCH: std::sync::OnceLock<Instant> = std::sync::OnceLock::new();
    EPOCH.get_or_init(Instant::now).elapsed().as_nanos() as u64
}

/// Is the armed drive over its limit? Reads the resident set at most once per
/// [`SAMPLE_EVERY`] (and, under the cgroup default, a capped cgroup's usage on
/// a sample that finds the drive grown by that cgroup's floor, see
/// [`cgroup_trips`]); once over, stays over for the rest of the drive.
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
            if drive.cgroup {
                let levels = CGROUP_LEVELS.lock().unwrap_or_else(|e| e.into_inner());
                if let Some(limit) = cgroup_trips(rss, drive.start, &levels, unreclaimable_bytes) {
                    drive.tripped = true;
                    drive.limit = limit;
                }
            } else {
                drive.tripped = trips(rss, drive.start, drive.limit);
            }
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

/// The cgroup default's trip rule: a capped cgroup (`levels`: directory and
/// cap) holds more than its [`cgroup_limit`] by `usage`, and the drive that
/// started at `start` has itself grown by that cgroup's floor: its
/// [`cgroup_margin`], or half the room the cap left the process at the start
/// when that is less, and never under [`MIN_GROWTH`]. A function that grew
/// less is left to finish, since it is not what is using the margin up; the
/// room term keeps a small cap from being reached before the floor is.
/// Returns the limit gone over. `usage` is asked only about a cgroup whose
/// floor the drive has grown by.
pub fn cgroup_trips(
    rss: u64,
    start: u64,
    levels: &[(String, u64)],
    usage: impl Fn(&str) -> Option<u64>,
) -> Option<u64> {
    let grown = rss.saturating_sub(start);
    levels.iter().find_map(|(dir, cap)| {
        let limit = cgroup_limit(*cap);
        let floor = cgroup_margin(*cap).min(cap.saturating_sub(start) / 2).max(MIN_GROWTH);
        (grown >= floor && usage(dir)? > limit).then_some(limit)
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
        let guard = Armed::enter_with(0, false);
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
        let guard = Armed::enter_with(MIB, false);
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
    fn free_pages_go_back_once_the_process_holds_more_than_the_last_release_left() {
        let ms = 1_000_000;
        assert!(!release_due(300 * MIB, 290 * MIB, true, 0, 0), "10 MiB over, under the armed slack");
        assert!(release_due(306 * MIB, 290 * MIB, true, 0, 50 * ms), "an explicit limit releases however recent");
        assert!(!release_due(340 * MIB, 290 * MIB, false, 10_000 * ms, 0), "50 MiB over, under MIN_GROWTH");
        assert!(release_due(360 * MIB, 290 * MIB, false, 10_000 * ms, 20 * ms));
        assert!(!release_due(360 * MIB, 290 * MIB, false, 600 * ms, 20 * ms), "too soon after a 20 ms release");
        assert!(release_due(360 * MIB, 290 * MIB, false, 640 * ms, 20 * ms));
        assert!(!release_due(200 * MIB, 290 * MIB, true, 0, 0), "under the floor");
    }

    #[test]
    fn the_cgroup_limit_is_the_cap_less_a_margin() {
        assert_eq!(cgroup_limit(6 << 30) / MIB, 5529);
        assert_eq!(cgroup_limit(2 << 30) / MIB, 1536);
        assert_eq!(cgroup_limit(32 << 30) / MIB, 29491);
        assert_eq!(cgroup_limit(256 << 20), 0);
    }

    #[test]
    fn the_cgroup_default_stops_only_a_drive_that_grew_by_the_margin_in_a_full_cgroup() {
        let (cap, limit) = (6u64 << 30, cgroup_limit(6 << 30));
        assert_eq!(limit / MIB, 5529);
        let one = |dir: &str| vec![(dir.to_string(), cap)];
        let full = |_: &str| Some(5600 * MIB);
        let roomy = |_: &str| Some(5000 * MIB);
        assert_eq!(cgroup_trips(1500 * MIB, 800 * MIB, &one("/a"), full), Some(limit));
        assert_eq!(cgroup_trips(1300 * MIB, 800 * MIB, &one("/a"), full), None, "grew 500 MiB, under the 614 MiB margin");
        assert_eq!(cgroup_trips(800 * MIB, 1500 * MIB, &one("/a"), full), None, "shrank");
        assert_eq!(cgroup_trips(1500 * MIB, 800 * MIB, &one("/a"), roomy), None, "room left");
        assert_eq!(cgroup_trips(1500 * MIB, 800 * MIB, &one("/a"), |_| None), None, "usage unreadable");
        let nested = vec![("/a/b".to_string(), cap), ("/a".to_string(), 32 << 30)];
        let outer_full = |dir: &str| Some(if dir == "/a" { 30000 * MIB } else { 5000 * MIB });
        assert_eq!(cgroup_trips(5000 * MIB, 800 * MIB, &nested, outer_full), Some(cgroup_limit(32 << 30)));
        assert_eq!(cgroup_trips(2000 * MIB, 800 * MIB, &nested, outer_full), None, "under the outer margin");
        assert_eq!(cgroup_margin(256 << 20), 256 << 20, "a cap under 512 MiB is all margin");
        let unread = |_: &str| -> Option<u64> { panic!("usage read before the drive grew by the margin") };
        assert_eq!(cgroup_trips(1300 * MIB, 800 * MIB, &one("/a"), unread), None);
    }

    #[test]
    fn a_small_cap_lowers_the_floor_to_half_the_room_left() {
        let small = vec![("/s".to_string(), 600 * MIB)];
        let full = |_: &str| Some(400 * MIB);
        assert_eq!(cgroup_limit(600 * MIB), 88 * MIB);
        assert_eq!(cgroup_trips(365 * MIB, 165 * MIB, &small, full), None, "grew 200 MiB of a 217 MiB floor");
        assert_eq!(cgroup_trips(385 * MIB, 165 * MIB, &small, full), Some(88 * MIB), "grew 220 MiB");
        let tight = vec![("/t".to_string(), 200 * MIB)];
        assert_eq!(cgroup_trips(230 * MIB, 165 * MIB, &tight, full), Some(0), "the floor never drops under 64 MiB");
        assert_eq!(cgroup_trips(225 * MIB, 165 * MIB, &tight, full), None);
        let big = vec![("/b".to_string(), 6 << 30)];
        assert_eq!(cgroup_trips(1400 * MIB, 800 * MIB, &big, |_| Some(5600 * MIB)), None, "a 6 GiB cap keeps its 614 MiB floor");
    }
}
