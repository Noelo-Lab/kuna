//! Return freed whole-image analysis pages before CLI function processing.
//! glibc can retain gigabytes of small Listing allocations after their owners
//! drop. This one phase-boundary trim releases only free pages; other targets
//! keep their normal policy. The binding takes a byte count, with no pointers.

#[cfg(all(target_os = "linux", target_env = "gnu"))]
unsafe extern "C" {
    fn malloc_trim(pad: usize) -> std::ffi::c_int;
}

pub(crate) fn after_analysis() {
    #[cfg(all(target_os = "linux", target_env = "gnu"))]
    unsafe {
        malloc_trim(0);
    }
}
