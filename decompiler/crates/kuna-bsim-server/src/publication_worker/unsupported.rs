use std::io;
use std::process::Child;

use tempfile::TempDir;

fn unsupported() -> io::Error {
    io::Error::new(io::ErrorKind::Unsupported, "remote publication has no process resource guard on this platform")
}

pub(super) struct Limits;

impl Limits {
    pub(super) fn new() -> io::Result<Self> {
        Err(unsupported())
    }

    pub(super) fn attach(&self, _: &Child) -> io::Result<()> {
        Err(unsupported())
    }
}

pub(super) fn private_dir() -> io::Result<TempDir> {
    Err(unsupported())
}

pub(super) fn limit_worker() -> io::Result<()> {
    Err(unsupported())
}
