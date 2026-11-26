//! Batch import of native executables and relocatable object files.

use std::ffi::{OsStr, OsString};
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;

use kuna_analysis::bsim::lshweights::LshProfile;
use kuna_bsim_server::{ImportOutcome, MachOSlice, MAX_ARTIFACT_BYTES, Store};
use serde::Serialize;

use crate::import_sign::{check_artifact, sign_binary};

const USAGE: &str = "kuna-bsim-server import <file>... --store DIR [--profile PROFILE] [--json]";

pub(super) fn run(argv: &[OsString]) -> i32 {
    if matches!(argv, [arg] if arg == "--help" || arg == "-h") {
        println!("Usage: {USAGE}\n\nSign native binaries, including relocatable object files, and store the original bytes.\nProfiles: 32, 64, 64_32, nosize, jvm, dalvik (default: derive per binary).\nImport continues after errors; incomplete signing or any error exits nonzero.");
        return 0;
    }
    let options = match Options::parse(argv) {
        Ok(options) => options,
        Err(error) => {
            eprintln!("import: {error}\nUsage: {USAGE}");
            return 2;
        }
    };
    let store = Store::open(options.store).map_err(|e| format!("open store {:?}: {e}", options.store));
    let mut report = Report {
        store: options.store.to_string_lossy(),
        imported: 0,
        incomplete: 0,
        failed: 0,
        items: if options.json { Vec::with_capacity(options.files.len()) } else { Vec::new() },
    };
    for path in options.files {
        let status = store.as_ref().map_err(String::clone)
            .and_then(|store| import_one(path, store, options.profile))
            .unwrap_or_else(|error| Status::Error { error });
        match &status {
            Status::Imported { complete, .. } => {
                report.imported += 1;
                if !complete {
                    report.incomplete += 1;
                }
            }
            Status::Error { .. } => report.failed += 1,
        }
        if options.json {
            report.items.push(Item { item: path.to_string_lossy(), status });
        } else {
            print_status(path, &status);
        }
    }
    if options.json {
        let mut out = io::stdout().lock();
        if let Err(error) = serde_json::to_writer(&mut out, &report) {
            eprintln!("import: writing result: {error}");
            return 1;
        }
        if let Err(error) = writeln!(out) {
            eprintln!("import: writing result: {error}");
            return 1;
        }
    } else {
        println!("import: {} file(s) accepted, {} incomplete, {} error(s)", report.imported, report.incomplete, report.failed);
    }
    if report.failed == 0 && report.incomplete == 0 { 0 } else { 1 }
}

struct Options<'a> {
    files: Vec<&'a Path>,
    store: &'a Path,
    profile: Option<LshProfile>,
    json: bool,
}

impl<'a> Options<'a> {
    fn parse(argv: &'a [OsString]) -> Result<Self, String> {
        let (mut files, mut store, mut profile, mut json) = (Vec::new(), None, None, false);
        let mut parse_flags = true;
        let mut args = argv.iter().map(OsString::as_os_str);
        while let Some(arg) = args.next() {
            match arg.to_str().filter(|_| parse_flags) {
                Some("--store") if store.is_none() => {
                    store = Some(Path::new(take_value(args.next(), "--store")?));
                }
                Some("--profile") if profile.is_none() => {
                    let value = take_value(args.next(), "--profile")?;
                    profile = Some(value.to_str().and_then(LshProfile::from_name)
                        .ok_or_else(|| format!("unknown BSim profile {value:?}"))?);
                }
                Some("--json") => json = true,
                Some("--") => parse_flags = false,
                _ if parse_flags && arg.as_encoded_bytes().starts_with(b"-") => {
                    return Err(format!("unknown or repeated flag {arg:?}"));
                }
                _ => files.push(Path::new(arg)),
            }
        }
        if files.is_empty() {
            return Err("at least one source file is required".into());
        }
        Ok(Self { files, store: store.ok_or("--store DIR is required")?, profile, json })
    }
}

fn take_value<'a>(value: Option<&'a OsStr>, flag: &str) -> Result<&'a OsStr, String> {
    value.filter(|s| !s.is_empty() && !s.as_encoded_bytes().starts_with(b"--"))
        .ok_or_else(|| format!("{flag} requires a value"))
}

#[derive(Serialize)]
struct Report<'a> {
    store: std::borrow::Cow<'a, str>,
    imported: usize,
    incomplete: usize,
    failed: usize,
    items: Vec<Item<'a>>,
}

#[derive(Serialize)]
struct Item<'a> {
    item: std::borrow::Cow<'a, str>,
    #[serde(flatten)]
    status: Status,
}

#[derive(Serialize)]
#[serde(tag = "status", rename_all = "snake_case")]
enum Status {
    Imported {
        display_name: String,
        profile: &'static str,
        sha256: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        macho_slice: Option<MachOSlice>,
        outcome: ImportOutcome,
        signed: usize,
        considered: usize,
        complete: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        first_failure: Option<String>,
    },
    Error { error: String },
}

fn print_status(path: &Path, status: &Status) {
    match status {
        Status::Imported { display_name, profile, sha256, outcome, signed, considered, first_failure, .. } => {
            let outcome = match outcome {
                ImportOutcome::Inserted => "inserted",
                ImportOutcome::Updated => "updated",
                ImportOutcome::AlreadyPresent => "already_present",
            };
            println!("{path:?}: {outcome} {display_name:?}, {profile}, SHA-256 {sha256}, {signed}/{considered} function(s) signed");
            if let Some(failure) = first_failure {
                eprintln!("import: {path:?}: incomplete signing coverage; first failure: {failure}");
            }
        }
        Status::Error { error } => eprintln!("import: {path:?}: {error}"),
    }
}

fn import_one(path: &Path, store: &Store, profile: Option<LshProfile>) -> Result<Status, String> {
    // Freeze the input so signing and publication read exactly the same bytes
    // even if the original path changes during import.
    let mut snapshot = snapshot(path)?;
    let display_name = path.file_name().ok_or("binary path has no filename")?.to_string_lossy().into_owned();
    let (profile, executable, coverage, macho_slice) =
        sign_binary(snapshot.path(), display_name.clone(), profile)?;
    let signed = executable.functions.len();
    let complete = executable.complete;

    let result = store.import_file(profile, executable, snapshot.as_file_mut(), macho_slice)
        .map_err(|e| format!("publishing to store: {e}"))?;
    Ok(Status::Imported {
        display_name,
        profile: profile.name(),
        sha256: result.sha256,
        macho_slice,
        outcome: result.outcome,
        signed,
        considered: coverage.considered,
        complete,
        first_failure: coverage.first_failure,
    })
}

fn snapshot(path: &Path) -> Result<tempfile::NamedTempFile, String> {
    // Reject FIFOs and devices before opening a path, then validate the open
    // handle and bound the copy independently of the reported file size.
    check_artifact(&fs::metadata(path).map_err(|e| format!("inspect original binary: {e}"))?)?;
    let file = File::open(path).map_err(|e| format!("open original binary: {e}"))?;
    check_artifact(&file.metadata().map_err(|e| format!("inspect original binary: {e}"))?)?;
    let mut snapshot = tempfile::NamedTempFile::new().map_err(|e| format!("create signing input: {e}"))?;
    io::copy(&mut file.take(MAX_ARTIFACT_BYTES + 1), snapshot.as_file_mut())
        .map_err(|e| format!("copy original binary: {e}"))?;
    check_artifact(&snapshot.as_file().metadata().map_err(|e| format!("inspect signing input: {e}"))?)?;
    Ok(snapshot)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn options_accept_a_batch_and_literal_paths() {
        let argv = ["one.o", "--profile", "64", "--store", "refs", "--json", "--", "-two.o"]
            .map(OsString::from);
        let options = Options::parse(&argv).unwrap();
        assert_eq!(options.files, [Path::new("one.o"), Path::new("-two.o")]);
        assert_eq!(options.store, Path::new("refs"));
        assert_eq!(options.profile, LshProfile::from_name("64"));
        assert!(options.json);
    }

    #[test]
    fn snapshot_rejects_directories_and_oversized_binaries() {
        let dir = tempfile::tempdir().unwrap();
        assert!(snapshot(dir.path()).unwrap_err().contains("regular file"));
        let file = tempfile::NamedTempFile::new_in(dir.path()).unwrap();
        file.as_file().set_len(MAX_ARTIFACT_BYTES + 1).unwrap();
        assert!(snapshot(file.path()).unwrap_err().contains("artifact limit"));
    }
}
