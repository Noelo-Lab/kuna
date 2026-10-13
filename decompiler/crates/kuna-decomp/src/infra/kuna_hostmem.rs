//! (kuna decompile-all memory) What this process and its machine hold in
//! memory, per platform: the process's resident set and its peak, and what a
//! new process could still take ([`available`]). Linux reads `/proc` and the
//! cgroup v2 tree, Windows asks `kernel32`, macOS asks `sysctl`/`getrusage`;
//! an answer a platform cannot give is `None`.

/// Test-only: the directory standing in for `/sys/fs/cgroup`.
pub const CGROUP_ROOT_ENV: &str = "KUNA_CGROUP_ROOT";

/// Memory a new process could take, and whether a cgroup's `memory.max`
/// rather than the machine is what bounds it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Available {
    pub bytes: u64,
    pub cgroup: bool,
}

#[cfg(target_os = "linux")]
use linux as sys;
#[cfg(target_os = "macos")]
use mac as sys;
#[cfg(not(any(target_os = "linux", windows, target_os = "macos")))]
use other as sys;
#[cfg(windows)]
use win as sys;

/// The process's resident set in bytes: Linux `VmRSS`; on Windows the larger
/// of the working set and the private commit ([`footprint`]), since commit is
/// what an allocation is refused on.
pub fn resident_bytes() -> Option<u64> {
    sys::resident()
}

/// The process's peak resident set in bytes: Linux `VmHWM`, the Windows peaks
/// of [`resident_bytes`]' two counters, macOS `ru_maxrss`.
pub fn peak_resident_bytes() -> Option<u64> {
    sys::peak()
}

/// Memory a new process could take without swapping or a refused allocation.
/// Linux: `MemAvailable`, or the cgroup headroom ([`cgroup_headroom`]) when
/// that is smaller. Windows: free physical memory or the remaining commit,
/// whichever is smaller ([`commit_bound`]). macOS: `hw.memsize` times
/// `kern.memorystatus_level` ([`level_share`]).
pub fn available() -> Option<Available> {
    sys::available()
}

/// The machine's free memory or the cgroup headroom, whichever is smaller.
pub fn tighter(machine: Option<u64>, cgroup: Option<u64>) -> Option<Available> {
    match (machine, cgroup) {
        (Some(m), Some(c)) if c < m => Some(Available { bytes: c, cgroup: true }),
        (Some(m), _) => Some(Available { bytes: m, cgroup: false }),
        (None, c) => c.map(|bytes| Available { bytes, cgroup: true }),
    }
}

/// A Windows process's footprint: the larger of its working set and its
/// private commit.
pub fn footprint(working_set: u64, commit: u64) -> u64 {
    working_set.max(commit)
}

/// What Windows can still hand out: free physical memory, or the commit left
/// under the commit limit when that is smaller.
pub fn commit_bound(avail_phys: u64, avail_page_file: u64) -> u64 {
    avail_phys.min(avail_page_file)
}

/// macOS's available memory: `memsize` times the `level` percent the memory
/// status reports free.
pub fn level_share(memsize: u64, level: u32) -> u64 {
    memsize / 100 * u64::from(level.min(100))
}

fn cgroup_root() -> String {
    std::env::var(CGROUP_ROOT_ENV).unwrap_or_else(|_| "/sys/fs/cgroup".into())
}

/// What the cgroups on this process's path still leave a new process: for
/// each with a finite `memory.max`, that cap less what the cgroup holds that
/// reclaim cannot hand back; the smallest of those.
pub fn cgroup_headroom() -> Option<u64> {
    let root = cgroup_root();
    let membership = std::fs::read_to_string("/proc/self/cgroup").ok()?;
    headroom_from(&membership, |dir, file| {
        std::fs::read_to_string(format!("{root}{dir}/{file}")).ok()
    })
}

/// [`cgroup_headroom`] over `/proc/self/cgroup` text, with `read` answering
/// one file of one cgroup directory. A cgroup's usage is
/// [`unreclaimable_bytes`]'s, `0` when it reports none.
pub fn headroom_from(membership: &str, read: impl Fn(&str, &str) -> Option<String>) -> Option<u64> {
    cgroup_dirs(membership)?
        .filter_map(|dir| {
            let max = parse_max(read(dir, "memory.max"))?;
            let used = unreclaimable(read(dir, "memory.current"), read(dir, "memory.stat"));
            Some(max.saturating_sub(used.unwrap_or(0)))
        })
        .min()
}

/// The cgroups on this process's path with a finite `memory.max`, innermost
/// first: each one's directory (under the cgroup root) and that cap in bytes.
pub fn capped_cgroups() -> Vec<(String, u64)> {
    let root = cgroup_root();
    let Ok(membership) = std::fs::read_to_string("/proc/self/cgroup") else {
        return Vec::new();
    };
    capped_from(&membership, |dir| std::fs::read_to_string(format!("{root}{dir}/memory.max")).ok())
        .into_iter()
        .map(|(dir, max)| (format!("{root}{dir}"), max))
        .collect()
}

/// [`capped_cgroups`] over `/proc/self/cgroup` text, with `read_max`
/// answering one cgroup directory's `memory.max` (`""` is the root, which has
/// one only inside a container's cgroup namespace); directories are relative
/// to the cgroup root.
pub fn capped_from(membership: &str, read_max: impl Fn(&str) -> Option<String>) -> Vec<(String, u64)> {
    cgroup_dirs(membership)
        .into_iter()
        .flatten()
        .filter_map(|dir| Some((dir.to_string(), parse_max(read_max(dir))?)))
        .collect()
}

/// What the cgroup directory `dir` holds that reclaim cannot hand back:
/// `memory.current` less its file-backed LRU pages (`active_file` +
/// `inactive_file` in `memory.stat`). tmpfs and shared memory sit on the
/// anonymous lists and stay counted. `None` when `memory.current` is
/// unreadable.
pub fn unreclaimable_bytes(dir: &str) -> Option<u64> {
    let read = |file: &str| std::fs::read_to_string(format!("{dir}/{file}")).ok();
    unreclaimable(read("memory.current"), read("memory.stat"))
}

/// [`unreclaimable_bytes`] over the two files' text.
pub fn unreclaimable(current: Option<String>, stat: Option<String>) -> Option<u64> {
    let current = current?.trim().parse::<u64>().ok()?;
    let reclaimable = stat.map_or(0, |stat| {
        stat_field(&stat, "active_file").saturating_add(stat_field(&stat, "inactive_file"))
    });
    Some(current.saturating_sub(reclaimable))
}

/// Each cgroup directory from the process's own to the root (`""`), innermost
/// first; `None` outside a cgroup v2 hierarchy.
fn cgroup_dirs(membership: &str) -> Option<impl Iterator<Item = &str>> {
    let leaf = membership.lines().find_map(|l| l.strip_prefix("0::"))?.trim_end_matches('/');
    Some(std::iter::successors(Some(leaf), |&dir| {
        (!dir.is_empty()).then(|| &dir[..dir.rfind('/').unwrap_or(0)])
    }))
}

fn parse_max(text: Option<String>) -> Option<u64> {
    text?.trim().parse::<u64>().ok()
}

fn stat_field(stat: &str, key: &str) -> u64 {
    stat.lines()
        .find_map(|l| l.strip_prefix(key)?.strip_prefix(' '))
        .and_then(|v| v.trim().parse().ok())
        .unwrap_or(0)
}

#[cfg(target_os = "linux")]
mod linux {
    pub fn resident() -> Option<u64> {
        proc_bytes("/proc/self/status", "VmRSS:")
    }

    pub fn peak() -> Option<u64> {
        proc_bytes("/proc/self/status", "VmHWM:")
    }

    pub fn available() -> Option<super::Available> {
        super::tighter(proc_bytes("/proc/meminfo", "MemAvailable:"), super::cgroup_headroom())
    }

    fn proc_bytes(path: &str, key: &str) -> Option<u64> {
        let text = std::fs::read_to_string(path).ok()?;
        let line = text.lines().find(|l| l.starts_with(key))?;
        Some(line.split_whitespace().nth(1)?.parse::<u64>().ok()? * 1024)
    }
}

#[cfg(not(any(target_os = "linux", windows, target_os = "macos")))]
mod other {
    pub fn resident() -> Option<u64> {
        None
    }

    pub fn peak() -> Option<u64> {
        None
    }

    pub fn available() -> Option<super::Available> {
        None
    }
}

#[cfg(windows)]
mod win {
    use std::ffi::c_void;

    #[repr(C)]
    struct MemoryStatusEx {
        length: u32,
        memory_load: u32,
        total_phys: u64,
        avail_phys: u64,
        total_page_file: u64,
        avail_page_file: u64,
        total_virtual: u64,
        avail_virtual: u64,
        avail_extended_virtual: u64,
    }

    #[repr(C)]
    struct ProcessMemoryCounters {
        cb: u32,
        page_fault_count: u32,
        peak_working_set: usize,
        working_set: usize,
        quota_peak_paged_pool: usize,
        quota_paged_pool: usize,
        quota_peak_non_paged_pool: usize,
        quota_non_paged_pool: usize,
        pagefile_usage: usize,
        peak_pagefile_usage: usize,
    }

    #[link(name = "kernel32")]
    unsafe extern "system" {
        fn GlobalMemoryStatusEx(buffer: *mut MemoryStatusEx) -> i32;
        fn GetCurrentProcess() -> *mut c_void;
        fn K32GetProcessMemoryInfo(
            process: *mut c_void,
            counters: *mut ProcessMemoryCounters,
            cb: u32,
        ) -> i32;
    }

    pub fn resident() -> Option<u64> {
        counters().map(|c| super::footprint(c.working_set as u64, c.pagefile_usage as u64))
    }

    pub fn peak() -> Option<u64> {
        counters().map(|c| super::footprint(c.peak_working_set as u64, c.peak_pagefile_usage as u64))
    }

    pub fn available() -> Option<super::Available> {
        status().map(|s| super::Available {
            bytes: super::commit_bound(s.avail_phys, s.avail_page_file),
            cgroup: false,
        })
    }

    fn status() -> Option<MemoryStatusEx> {
        // SAFETY: all-zero is a valid value of this plain-integer struct.
        let mut s: MemoryStatusEx = unsafe { std::mem::zeroed() };
        s.length = std::mem::size_of::<MemoryStatusEx>() as u32;
        // SAFETY: `s` is a MEMORYSTATUSEX with `dwLength` set, as the call requires.
        (unsafe { GlobalMemoryStatusEx(&mut s) } != 0).then_some(s)
    }

    fn counters() -> Option<ProcessMemoryCounters> {
        // SAFETY: all-zero is a valid value of this plain-integer struct.
        let mut c: ProcessMemoryCounters = unsafe { std::mem::zeroed() };
        let cb = std::mem::size_of::<ProcessMemoryCounters>() as u32;
        c.cb = cb;
        // SAFETY: the pseudo-handle needs no closing, and `c` is `cb` bytes.
        (unsafe { K32GetProcessMemoryInfo(GetCurrentProcess(), &mut c, cb) } != 0).then_some(c)
    }
}

#[cfg(target_os = "macos")]
mod mac {
    use std::ffi::{c_char, c_int, c_void, CStr};

    #[repr(C)]
    struct Rusage {
        times: [i64; 4],
        maxrss: i64,
        rest: [i64; 13],
    }

    unsafe extern "C" {
        fn sysctlbyname(
            name: *const c_char,
            oldp: *mut c_void,
            oldlenp: *mut usize,
            newp: *mut c_void,
            newlen: usize,
        ) -> c_int;
        fn getrusage(who: c_int, usage: *mut Rusage) -> c_int;
    }

    fn sysctl<T: Copy + Default>(name: &CStr) -> Option<T> {
        let mut value = T::default();
        let mut len = std::mem::size_of::<T>();
        // SAFETY: `value` is `len` writable bytes and the name is NUL-terminated.
        let rc = unsafe {
            sysctlbyname(name.as_ptr(), (&raw mut value).cast(), &mut len, std::ptr::null_mut(), 0)
        };
        (rc == 0 && len == std::mem::size_of::<T>()).then_some(value)
    }

    pub fn resident() -> Option<u64> {
        None
    }

    pub fn available() -> Option<super::Available> {
        let memsize = sysctl::<u64>(c"hw.memsize")?;
        let level = sysctl::<u32>(c"kern.memorystatus_level")?;
        Some(super::Available { bytes: super::level_share(memsize, level), cgroup: false })
    }

    pub fn peak() -> Option<u64> {
        // SAFETY: all-zero is a valid value of this plain-integer struct.
        let mut usage: Rusage = unsafe { std::mem::zeroed() };
        // SAFETY: RUSAGE_SELF (0) fills one `struct rusage`, which `Rusage` lays out.
        (unsafe { getrusage(0, &mut usage) } == 0).then(|| usage.maxrss.max(0) as u64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_headroom_is_each_cap_less_what_reclaim_cannot_free() {
        const GIB: u64 = 1 << 30;
        let membership = "0::/user.slice/app.slice/run-7.scope\n";
        let tree = |dir: &str, file: &str| -> Option<String> {
            let v = match (dir, file) {
                ("/user.slice/app.slice/run-7.scope", "memory.max") => "6442450944",
                ("/user.slice/app.slice/run-7.scope", "memory.current") => "3221225472",
                ("/user.slice/app.slice/run-7.scope", "memory.stat") => {
                    "anon 1073741824\nfile 2147483648\nactive_file 536870912\n\
                     inactive_file 1073741824\nshmem 536870912\n"
                }
                ("/user.slice", "memory.max") => "34359738368",
                ("/user.slice", "memory.current") => "33285996544",
                _ => return None,
            };
            Some(v.to_string())
        };
        assert_eq!(headroom_from(membership, tree), Some(GIB), "the parent slice is the tighter one");
        let scope_only = |dir: &str, file: &str| tree(dir, file).filter(|_| dir.ends_with(".scope"));
        assert_eq!(
            headroom_from(membership, scope_only),
            Some(6 * GIB - (3 * GIB - 3 * GIB / 2)),
            "the scope's file LRU is reclaimable, its shmem is not"
        );
        let no_usage = |dir: &str, file: &str| (file == "memory.max").then(|| tree(dir, file)).flatten();
        assert_eq!(headroom_from(membership, no_usage), Some(6 * GIB), "no usage files: the cap");
        assert_eq!(headroom_from(membership, |_, _| Some("max".into())), None, "uncapped");
        let over = |_: &str, file: &str| Some(if file == "memory.max" { "100" } else { "200" }.into());
        assert_eq!(headroom_from("0::/\n", over), Some(0), "over its cap: nothing left");
    }

    #[test]
    fn every_capped_cgroup_on_the_path_is_listed_innermost_first() {
        let membership = "0::/user.slice/wave.slice/run-1.scope\n";
        let caps = |dir: &str| match dir {
            "/user.slice" => Some("max\n".to_string()),
            "/user.slice/wave.slice" => Some("6442450944\n".to_string()),
            "/user.slice/wave.slice/run-1.scope" => Some("34359738368\n".to_string()),
            _ => None,
        };
        assert_eq!(
            capped_from(membership, caps),
            vec![
                ("/user.slice/wave.slice/run-1.scope".to_string(), 32 << 30),
                ("/user.slice/wave.slice".to_string(), 6 << 30),
            ]
        );
        assert!(capped_from("1:memory:/user.slice\n", caps).is_empty(), "cgroup v1");
        assert!(capped_from(membership, |_| Some("max".into())).is_empty());
        let container = |dir: &str| (dir.is_empty()).then(|| "2147483648".to_string());
        assert_eq!(capped_from("0::/\n", container), vec![(String::new(), 2 << 30)], "a namespaced container's own cap");
        let stat = "anon 9\nfile 7\nactive_file 2\ninactive_file 3\nshmem 2\n".to_string();
        assert_eq!(unreclaimable(Some("100\n".into()), Some(stat)), Some(95));
        assert_eq!(unreclaimable(Some("100".into()), None), Some(100));
        assert_eq!(unreclaimable(None, Some("active_file 1".into())), None);
    }

    #[test]
    fn the_tighter_bound_wins_and_says_which_it_is() {
        assert_eq!(tighter(Some(200), Some(6)), Some(Available { bytes: 6, cgroup: true }));
        assert_eq!(tighter(Some(5), Some(6)), Some(Available { bytes: 5, cgroup: false }));
        assert_eq!(tighter(Some(5), None), Some(Available { bytes: 5, cgroup: false }));
        assert_eq!(tighter(None, Some(6)), Some(Available { bytes: 6, cgroup: true }));
        assert_eq!(tighter(None, None), None);
    }

    #[test]
    fn the_windows_and_macos_arithmetic() {
        assert_eq!(footprint(3, 7), 7);
        assert_eq!(footprint(9, 7), 9);
        assert_eq!(commit_bound(16 << 30, 2 << 30), 2 << 30, "a full commit limit binds");
        assert_eq!(commit_bound(4 << 30, 40 << 30), 4 << 30);
        assert_eq!(level_share(16 << 30, 25), (16 << 30) / 100 * 25);
        assert_eq!(level_share(16 << 30, 250), (16 << 30) / 100 * 100);
    }

    #[test]
    fn this_platform_reports_what_the_pool_needs() {
        if cfg!(any(target_os = "linux", windows)) {
            assert!(resident_bytes().is_some_and(|b| b > 0));
        }
        if cfg!(any(target_os = "linux", windows, target_os = "macos")) {
            assert!(peak_resident_bytes().is_some_and(|b| b > 0));
            assert!(available().is_some());
        }
    }
}
