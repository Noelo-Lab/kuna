//! A single-job signer subprocess. The HTTP process owns the upload and the
//! store; the child gets no credentials, sockets, store path, or load options.

use std::ffi::OsString;
use std::fs::{self, File};
use std::io::{self, BufWriter, Read, Seek, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, ExitStatus, Stdio};
use std::time::{Duration, Instant};

use kuna_analysis::bsim::corpus::CorpusExecutable;
use kuna_analysis::bsim::lshweights::LshProfile;
use kuna_analysis::bsim::signature::BsimSignature;
use kuna_bsim_server::{MAX_ARTIFACT_BYTES, MAX_PUBLISH_METADATA_BYTES, PublishRequest};
use kuna_decomp::kuna_memoryspecs::SpecRoot;
use serde::{Deserialize, Serialize};
use tempfile::TempDir;

use crate::import_sign;

#[cfg(any(target_os = "linux", target_os = "macos"))]
#[path = "publication_worker/unix.rs"]
mod platform;
#[cfg(windows)]
#[path = "publication_worker/windows.rs"]
mod platform;
#[cfg(not(any(target_os = "linux", target_os = "macos", windows)))]
#[path = "publication_worker/unsupported.rs"]
mod platform;

pub(super) const COMMAND: &str = "--publication-worker";
const USE_EMBEDDED_SPECS: &str = "--embedded-specs";
const IMAGE: &str = "image";
const METADATA: &str = "metadata.json";
const READY: u8 = 1;
const REJECTED: i32 = 65;
const UNAVAILABLE: i32 = 69;
const WORKER_WALL_SECONDS: u32 = 120;
const WORKER_TIMEOUT: Duration = Duration::from_secs(WORKER_WALL_SECONDS as u64);
#[cfg(any(target_os = "linux", target_os = "macos", windows))]
const MAX_WORKER_MEMORY: usize = 2 * 1024 * 1024 * 1024;
const MAX_RESULT_BYTES: u64 = 1024 * 1024;

pub(super) struct Publisher {
    executable: PathBuf,
    disk_specs: Vec<PathBuf>,
    embedded_specs: bool,
}

pub(super) struct Upload {
    pub(super) image: File,
    stage: TempDir,
}

pub(super) enum WorkerError {
    Rejected,
    TimedOut,
    Unavailable(io::Error),
}

impl Publisher {
    pub(super) fn new() -> io::Result<Self> {
        // Fail explicitly before listening on an unsupported platform. Install
        // and verify each actual worker's limits again for every publication.
        let _limits = platform::Limits::new()?;
        let (disk_specs, embedded_specs) = import_sign::spec_locations().map_err(io::Error::other)?;
        Ok(Self {
            executable: std::env::current_exe()?,
            disk_specs: disk_specs.into_iter().map(std::path::absolute).collect::<io::Result<_>>()?,
            embedded_specs,
        })
    }

    pub(super) fn stage(metadata: &[u8]) -> io::Result<Upload> {
        let stage = platform::private_dir()?;
        File::create_new(stage.path().join(METADATA))?.write_all(metadata)?;
        let image = File::options().read(true).write(true).create_new(true).open(stage.path().join(IMAGE))?;
        Ok(Upload { image, stage })
    }

    pub(super) fn sign(&self, upload: &Upload) -> Result<(LshProfile, CorpusExecutable), WorkerError> {
        let until = Instant::now() + WORKER_TIMEOUT;
        let limits = platform::Limits::new().map_err(WorkerError::Unavailable)?;
        // The parent keeps the original anonymous result handle; it never
        // opens a path written by the child. All diagnostics go to the sink.
        let mut result = tempfile::tempfile_in(upload.stage.path()).map_err(WorkerError::Unavailable)?;
        let output = result.try_clone().map_err(WorkerError::Unavailable)?;
        let child = Command::new(&self.executable)
            .arg(COMMAND)
            .args(&self.disk_specs)
            .args(self.embedded_specs.then_some(USE_EMBEDDED_SPECS))
            .env_clear()
            .current_dir(upload.stage.path())
            .stdin(Stdio::piped())
            .stdout(Stdio::from(output))
            .stderr(Stdio::null())
            .spawn()
            .map_err(WorkerError::Unavailable)?;
        let mut child = ChildGuard(child);
        limits.attach(&child.0).map_err(WorkerError::Unavailable)?;
        // On Windows the process cannot open the image before the parent
        // assigns the job. A closed pipe never grants permission to start.
        let mut gate = child.0.stdin.take().expect("worker stdin is piped");
        let permission = gate.write_all(&[READY]);
        drop(gate);
        let status = child.wait_until(until)?;
        if status.code() == Some(UNAVAILABLE) {
            return Err(WorkerError::Unavailable(io::Error::other("publication worker resource limits or I/O unavailable")));
        }
        if !status.success() {
            return Err(WorkerError::Rejected);
        }
        permission.map_err(WorkerError::Unavailable)?;

        result.rewind().map_err(WorkerError::Unavailable)?;
        let bytes = read_regular(result, MAX_RESULT_BYTES).map_err(WorkerError::Unavailable)?;
        serde_json::from_slice::<SignedPublication>(&bytes)
            .map_err(io::Error::from)
            .and_then(SignedPublication::into_record)
            .map_err(WorkerError::Unavailable)
    }
}

struct ChildGuard(Child);

impl ChildGuard {
    fn wait_until(&mut self, until: Instant) -> Result<ExitStatus, WorkerError> {
        loop {
            let remaining = until.saturating_duration_since(Instant::now());
            if remaining.is_zero() {
                self.stop();
                return Err(WorkerError::TimedOut);
            }
            if let Some(status) = self.0.try_wait().map_err(WorkerError::Unavailable)? {
                return Ok(status);
            }
            std::thread::sleep(remaining.min(Duration::from_millis(10)));
        }
    }

    fn stop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

impl Drop for ChildGuard {
    fn drop(&mut self) {
        // try_wait/wait cache a reaped exit status. On all other exits, stop
        // the child before its staging directory is removed or the slot reused.
        self.stop();
    }
}

pub(super) fn run(args: &[OsString]) -> i32 {
    let until = Instant::now() + WORKER_TIMEOUT;
    let mut gate = [0u8];
    if io::stdin().read_exact(&mut gate).is_err() || gate != [READY] {
        return UNAVAILABLE;
    }
    if platform::limit_worker().is_err() {
        return UNAVAILABLE;
    }
    // Only the parent resolves disk paths. The embedded marker is last so
    // operator-provided roots keep their precedence over the bundled table.
    let (embedded, paths) = match args.split_last() {
        Some((flag, paths)) if flag == USE_EMBEDDED_SPECS => (true, paths),
        _ => (false, args),
    };
    let mut roots = Vec::with_capacity(paths.len() + usize::from(embedded));
    for arg in paths {
        let path = Path::new(arg);
        if !path.is_absolute() || arg.to_str().is_none() {
            return UNAVAILABLE;
        }
        roots.push(SpecRoot::Dir(path.to_owned()));
    }
    if embedded {
        let Some(root) = import_sign::bundled_spec_root() else { return UNAVAILABLE };
        roots.push(root);
    }
    let load = |path, limit| -> io::Result<_> {
        if !fs::symlink_metadata(path)?.is_file() {
            return Err(io::Error::other("worker input is not a regular file"));
        }
        read_regular(File::open(path)?, limit)
    };
    let Ok(metadata) = load(METADATA, MAX_PUBLISH_METADATA_BYTES as u64) else { return UNAVAILABLE };
    let Ok(request) = serde_json::from_slice::<PublishRequest>(&metadata) else { return REJECTED };
    let Ok(image) = load(IMAGE, MAX_ARTIFACT_BYTES) else { return UNAVAILABLE };
    let Ok((profile, mut record)) = import_sign::sign_publication(image, &request, &roots, until) else { return REJECTED };
    let Some(signature) = record.functions.pop() else { return UNAVAILABLE };
    let signed = SignedPublication {
        profile: profile.name().into(),
        executable: record.name,
        architecture: record.architecture,
        compiler: record.compiler,
        md5: record.md5,
        name: signature.name,
        address: signature.address,
        settings: signature.settings,
        features: signature.features,
        has_unimplemented: signature.has_unimplemented,
        has_bad_data: signature.has_bad_data,
    };
    let stdout = io::stdout();
    let mut result = BufWriter::new(LimitedWriter { inner: stdout.lock(), remaining: MAX_RESULT_BYTES });
    if serde_json::to_writer(&mut result, &signed).is_err() {
        return REJECTED;
    }
    if result.flush().is_err() { UNAVAILABLE } else { 0 }
}

fn read_regular(file: File, limit: u64) -> io::Result<Vec<u8>> {
    let metadata = file.metadata()?;
    if !metadata.is_file() || !(1..=limit).contains(&metadata.len()) {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "invalid publication worker file size or type"));
    }
    let mut bytes = Vec::with_capacity(metadata.len() as usize);
    file.take(metadata.len()).read_to_end(&mut bytes)?;
    if bytes.len() as u64 != metadata.len() {
        return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "truncated publication worker file"));
    }
    Ok(bytes)
}

struct LimitedWriter<W> {
    inner: W,
    remaining: u64,
}

impl<W: Write> Write for LimitedWriter<W> {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        if bytes.len() as u64 > self.remaining {
            return Err(io::Error::other("publication signer result exceeds its size limit"));
        }
        let len = self.inner.write(bytes)?;
        self.remaining -= len as u64;
        Ok(len)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct SignedPublication {
    profile: String,
    executable: String,
    architecture: String,
    compiler: String,
    md5: String,
    name: String,
    address: u64,
    settings: u32,
    features: Vec<u32>,
    has_unimplemented: bool,
    has_bad_data: bool,
}

impl SignedPublication {
    fn into_record(self) -> io::Result<(LshProfile, CorpusExecutable)> {
        let profile = LshProfile::from_name(&self.profile)
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "publication signer returned an unknown profile"))?;
        Ok((profile, CorpusExecutable {
            name: self.executable,
            architecture: self.architecture,
            compiler: self.compiler,
            md5: self.md5,
            complete: false,
            functions: vec![BsimSignature {
                name: self.name,
                address: self.address,
                settings: self.settings,
                features: self.features,
                has_unimplemented: self.has_unimplemented,
                has_bad_data: self.has_bad_data,
            }],
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn result_writer_never_writes_beyond_configured_bound() {
        let mut writer = LimitedWriter { inner: Vec::new(), remaining: 4 };
        writer.write_all(b"12").unwrap();
        assert!(writer.write_all(b"345").is_err());
        writer.write_all(b"34").unwrap();
        assert!(writer.write_all(b"5").is_err());
        assert_eq!(writer.inner.as_slice(), b"1234");
    }

    #[test]
    fn blocked_test_child() {
        if std::env::var_os("KUNA_TEST_SIGNER_BLOCK").is_some() {
            let _ = io::stdin().read(&mut [0]);
        }
    }

    #[test]
    fn deadline_kills_and_reaps_blocked_worker() {
        let child = Command::new(std::env::current_exe().unwrap())
            .args(["--exact", "publication_worker::tests::blocked_test_child"])
            .env("KUNA_TEST_SIGNER_BLOCK", "1")
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn().unwrap();
        let mut child = ChildGuard(child);
        assert!(child.0.try_wait().unwrap().is_none());
        assert!(matches!(child.wait_until(Instant::now()), Err(WorkerError::TimedOut)));
        assert!(child.0.try_wait().unwrap().is_some());
    }
}
