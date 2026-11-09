//! glibc's heap policy for a JVM engine run, set once the input has been
//! routed to the JVM engine (never on the native path).
//!
//! Every engine thread builds and drops millions of small objects. Under the
//! default policy each thread's heap grows a few pages at a time and is trimmed
//! back when objects die; both are `mprotect`/`madvise` calls under the
//! process's memory-map lock, so at a dozen threads and more they serialize the
//! threads' page faults. A large top pad grows a heap a whole segment at a
//! time, an unreachable trim threshold keeps freed pages for the next objects,
//! and blocks up to 32 MiB come from the heaps instead of fresh mappings.

#[cfg(all(target_os = "linux", target_env = "gnu"))]
unsafe extern "C" {
    fn mallopt(param: std::ffi::c_int, value: std::ffi::c_int) -> std::ffi::c_int;
}

pub(crate) fn tune() {
    #[cfg(all(target_os = "linux", target_env = "gnu"))]
    {
        const M_TRIM_THRESHOLD: std::ffi::c_int = -1;
        const M_TOP_PAD: std::ffi::c_int = -2;
        const M_MMAP_THRESHOLD: std::ffi::c_int = -3;
        // SAFETY: `mallopt` takes two integers and only changes allocator parameters.
        unsafe {
            mallopt(M_TRIM_THRESHOLD, std::ffi::c_int::MAX);
            mallopt(M_TOP_PAD, 256 << 20);
            mallopt(M_MMAP_THRESHOLD, 32 << 20);
        }
    }
}
