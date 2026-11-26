use std::io;
use std::mem::size_of;
use std::os::windows::ffi::OsStrExt;
use std::os::windows::io::AsRawHandle;
use std::process::Child;
use std::ptr::{null, null_mut};

use tempfile::TempDir;
use windows_sys::Win32::Foundation::{CloseHandle, HANDLE, LocalFree};
use windows_sys::Win32::Security::Authorization::{
    ConvertStringSecurityDescriptorToSecurityDescriptorW, SetNamedSecurityInfoW, SE_FILE_OBJECT,
};
use windows_sys::Win32::Security::{
    GetSecurityDescriptorDacl, DACL_SECURITY_INFORMATION, PROTECTED_DACL_SECURITY_INFORMATION,
};
use windows_sys::Win32::System::JobObjects::{
    AssignProcessToJobObject, CreateJobObjectW, JobObjectExtendedLimitInformation,
    JOBOBJECT_EXTENDED_LIMIT_INFORMATION, JOB_OBJECT_LIMIT_ACTIVE_PROCESS,
    JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE, JOB_OBJECT_LIMIT_PROCESS_MEMORY,
    QueryInformationJobObject, SetInformationJobObject,
};

use super::MAX_WORKER_MEMORY;

pub(super) struct Limits(HANDLE);

impl Limits {
    pub(super) fn new() -> io::Result<Self> {
        // Safety: create an unnamed, non-inheritable handle and set limits
        // using the documented extended limit structure for that handle.
        unsafe {
            let handle = CreateJobObjectW(null(), null());
            if handle.is_null() {
                return Err(io::Error::last_os_error());
            }
            let job = Self(handle);
            let mut limits = JOBOBJECT_EXTENDED_LIMIT_INFORMATION::default();
            limits.BasicLimitInformation.LimitFlags = JOB_OBJECT_LIMIT_ACTIVE_PROCESS
                | JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE | JOB_OBJECT_LIMIT_PROCESS_MEMORY;
            limits.BasicLimitInformation.ActiveProcessLimit = 1;
            limits.ProcessMemoryLimit = MAX_WORKER_MEMORY;
            if SetInformationJobObject(handle, JobObjectExtendedLimitInformation,
                (&limits as *const JOBOBJECT_EXTENDED_LIMIT_INFORMATION).cast(),
                size_of::<JOBOBJECT_EXTENDED_LIMIT_INFORMATION>() as u32) == 0
            {
                return Err(io::Error::last_os_error());
            }
            Ok(job)
        }
    }

    pub(super) fn attach(&self, child: &Child) -> io::Result<()> {
        // Safety: both job and process handles remain owned and alive.
        if unsafe { AssignProcessToJobObject(self.0, child.as_raw_handle()) } == 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(())
    }
}

impl Drop for Limits {
    fn drop(&mut self) {
        // Safety: this is the single owner of the job handle. Closing it also
        // kills any worker still associated with the job.
        unsafe { CloseHandle(self.0) };
    }
}

pub(super) fn limit_worker() -> io::Result<()> {
    let mut limits = JOBOBJECT_EXTENDED_LIMIT_INFORMATION::default();
    // Safety: a null job selects the current process's job. The output
    // points to a correctly sized writable extended limit structure.
    let ok = unsafe { QueryInformationJobObject(null_mut(), JobObjectExtendedLimitInformation,
        (&mut limits as *mut JOBOBJECT_EXTENDED_LIMIT_INFORMATION).cast(),
        size_of::<JOBOBJECT_EXTENDED_LIMIT_INFORMATION>() as u32, null_mut()) };
    let required = JOB_OBJECT_LIMIT_ACTIVE_PROCESS | JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE
        | JOB_OBJECT_LIMIT_PROCESS_MEMORY;
    if ok == 0 || limits.BasicLimitInformation.LimitFlags & required != required
        || limits.BasicLimitInformation.ActiveProcessLimit != 1
        || !(1..=MAX_WORKER_MEMORY).contains(&limits.ProcessMemoryLimit)
    {
        return Err(io::Error::new(io::ErrorKind::Unsupported, "worker has no bounded Windows job"));
    }
    Ok(())
}

pub(super) fn private_dir() -> io::Result<TempDir> {
    let dir = tempfile::tempdir()?;
    let path: Vec<_> = dir.path().as_os_str().encode_wide().chain(Some(0)).collect();
    // Set a protected, inheritable ACL on the empty directory before creating
    // any files. Only its owner, system and administrators may access it.
    let acl: Vec<_> = "D:P(A;OICI;FA;;;OW)(A;OICI;FA;;;SY)(A;OICI;FA;;;BA)\0".encode_utf16().collect();
    // Safety: both strings are NUL-terminated, descriptor points to writable
    // storage, and a successful conversion transfers a LocalFree allocation.
    unsafe {
        let mut descriptor = null_mut();
        if ConvertStringSecurityDescriptorToSecurityDescriptorW(acl.as_ptr(), 1, &mut descriptor, null_mut()) == 0 {
            return Err(io::Error::last_os_error());
        }
        let mut present = 0;
        let mut acl = null_mut();
        let mut defaulted = 0;
        let result = if GetSecurityDescriptorDacl(descriptor, &mut present, &mut acl, &mut defaulted) == 0 {
            Err(io::Error::last_os_error())
        } else if present == 0 || acl.is_null() {
            Err(io::Error::other("private staging directory is missing its access control list"))
        } else {
            let status = SetNamedSecurityInfoW(path.as_ptr(), SE_FILE_OBJECT,
                DACL_SECURITY_INFORMATION | PROTECTED_DACL_SECURITY_INFORMATION,
                null_mut(), null_mut(), acl, null());
            if status == 0 { Ok(dir) } else { Err(io::Error::from_raw_os_error(status as i32)) }
        };
        LocalFree(descriptor);
        result
    }
}
