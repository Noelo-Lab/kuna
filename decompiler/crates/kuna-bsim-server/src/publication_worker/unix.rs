use std::fs::Permissions;
use std::io;
use std::os::unix::fs::PermissionsExt;
use std::process::Child;
use std::ptr;

use tempfile::TempDir;

use super::{MAX_RESULT_BYTES, MAX_WORKER_MEMORY, WORKER_WALL_SECONDS};

pub(super) struct Limits;

impl Limits {
    pub(super) fn new() -> io::Result<Self> {
        Ok(Self)
    }

    pub(super) fn attach(&self, _: &Child) -> io::Result<()> {
        Ok(())
    }
}

pub(super) fn private_dir() -> io::Result<TempDir> {
    tempfile::Builder::new().permissions(Permissions::from_mode(0o700)).tempdir()
}

pub(super) fn limit_worker() -> io::Result<()> {
    // Safety: these syscalls operate on the worker itself, with valid rlimit
    // structures. Respect tighter inherited limits instead of raising them.
    let limit = |resource, max: libc::rlim_t| unsafe {
        let mut rlim = libc::rlimit { rlim_cur: 0, rlim_max: 0 };
        if libc::getrlimit(resource, &mut rlim) != 0 {
            return Err(io::Error::last_os_error());
        }
        let bound = max.min(rlim.rlim_cur).min(rlim.rlim_max);
        rlim = libc::rlimit { rlim_cur: bound, rlim_max: bound };
        if libc::setrlimit(resource, &rlim) != 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(bound)
    };
    let address_space = limit(libc::RLIMIT_AS, MAX_WORKER_MEMORY as libc::rlim_t)?;
    limit(libc::RLIMIT_CORE, 0)?;
    limit(libc::RLIMIT_FSIZE, MAX_RESULT_BYTES as libc::rlim_t)?;

    // Some OS versions accept setrlimit but don't enforce address-space
    // limits on mappings. Probe with one over-limit, untouched mapping.
    let len = address_space as usize + 4096;
    // Safety: the mapping is anonymous, no memory is accessed, and any
    // successful result is immediately unmapped using the same range.
    unsafe {
        let probe = libc::mmap(ptr::null_mut(), len, libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_PRIVATE | libc::MAP_ANON, -1, 0);
        if probe != libc::MAP_FAILED {
            libc::munmap(probe, len);
            return Err(io::Error::new(io::ErrorKind::Unsupported, "worker address-space limit is not enforced"));
        }
        if io::Error::last_os_error().raw_os_error() != Some(libc::ENOMEM) {
            return Err(io::Error::last_os_error());
        }
        // Backstop if the supervising server exits after opening the gate.
        let mut alarms = std::mem::zeroed::<libc::sigset_t>();
        if libc::signal(libc::SIGALRM, libc::SIG_DFL) == libc::SIG_ERR
            || libc::sigemptyset(&mut alarms) != 0
            || libc::sigaddset(&mut alarms, libc::SIGALRM) != 0
        {
            return Err(io::Error::last_os_error());
        }
        let status = libc::pthread_sigmask(libc::SIG_UNBLOCK, &alarms, ptr::null_mut());
        if status != 0 {
            return Err(io::Error::from_raw_os_error(status));
        }
        libc::alarm(WORKER_WALL_SECONDS);
    }
    Ok(())
}
