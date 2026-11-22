//! Batch import from a closed/copied local Ghidra 12.1.2 project or GZF.

mod index;
mod project;

use std::borrow::Cow;
use std::ffi::{OsStr, OsString};
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

use kuna_analysis::bsim::lshweights::LshProfile;
use kuna_analysis::ghidra_program;
use kuna_bsim_server::{ImportOutcome, MAX_ARTIFACT_BYTES, Store};
use serde::Serialize;

use crate::import_sign::sign_import;
use project::{Candidate, Database, Kind, at, check_kind};

const USAGE: &str = "kuna-bsim-server import-ghidra <project.gpr|project.rep|archive.gzf> --store DIR [--profile PROFILE] [--json]";
const MAX_DATABASE_BYTES: usize = 1024 * 1024 * 1024;

pub(super) fn run(argv: &[OsString]) -> i32 {
    if matches!(argv, [arg] if arg == "--help" || arg == "-h") {
        println!("Usage: {USAGE}\n\nImport a closed or copied local Ghidra 12.1.2 snapshot without changing it.\nMigrates named functions from backed linked ELF, PE, Mach-O, and relocatable ELF inputs.\nProfiles: 32, 64, 64_32, nosize, jvm, dalvik (default: derive per binary).\nImport continues after errors; incomplete signing or any error exits nonzero.");
        return 0;
    }
    let options = match Options::parse(argv) {
        Ok(options) => options,
        Err(error) => {
            eprintln!("import-ghidra: {error}\nUsage: {USAGE}");
            return 2;
        }
    };
    let items = match import(&options) {
        Ok(items) => items,
        Err(error) => vec![Item { item: "source or store".into(), status: Status::Error { error } }],
    };
    let (mut imported, mut incomplete, mut skipped, mut failed) = (0, 0, 0, 0);
    for item in &items {
        match &item.status {
            Status::Imported { snapshot_complete, .. } => {
                imported += 1;
                incomplete += usize::from(!snapshot_complete);
            }
            Status::Skipped { .. } => skipped += 1,
            Status::Error { .. } => failed += 1,
        }
    }
    if options.json {
        let report = Report {
            source: options.source.to_string_lossy(),
            store: options.store.to_string_lossy(),
            imported,
            incomplete,
            skipped,
            failed,
            items: &items,
        };
        let mut out = io::stdout().lock();
        if let Err(error) = serde_json::to_writer(&mut out, &report) {
            eprintln!("import-ghidra: writing result: {error}");
            return 1;
        }
        if let Err(error) = writeln!(out) {
            eprintln!("import-ghidra: writing result: {error}");
            return 1;
        }
    } else {
        for Item { item, status } in items {
            match status {
                Status::Imported {
                    display_name, profile, sha256, outcome, version, signed, considered, skipped_unnamed,
                    inserted, updated, unchanged, kept_published, first_failure, ..
                } => {
                    let version = version.map_or_else(String::new, |v| format!(" (saved version {v})"));
                    let outcome = match outcome {
                        ImportOutcome::Inserted => "inserted",
                        ImportOutcome::Updated => "updated",
                        ImportOutcome::AlreadyPresent => "already_present",
                    };
                    println!("{item:?}{version}: {outcome} {display_name:?}, {profile}, SHA-256 {sha256}, \
                        {signed}/{considered} named function(s) signed, {skipped_unnamed} unnamed skipped; \
                        store: {inserted} added, {updated} updated, \
                        {unchanged} unchanged, {kept_published} published kept");
                    if let Some(failure) = first_failure {
                        eprintln!("import-ghidra: {item:?}: incomplete snapshot signing coverage; first failure: {failure}");
                    }
                }
                Status::Skipped { skipped_unnamed } => {
                    println!("{item:?}: skipped (no eligible named functions; {skipped_unnamed} unnamed skipped)");
                }
                Status::Error { error } => eprintln!("import-ghidra: {item:?}: {error}"),
            }
        }
        println!("import-ghidra: {imported} program(s) accepted, {incomplete} incomplete, {skipped} skipped, {failed} error(s)");
    }
    if failed == 0 && incomplete == 0 { 0 } else { 1 }
}

struct Options<'a> {
    source: &'a Path,
    store: &'a Path,
    profile: Option<LshProfile>,
    json: bool,
}

impl<'a> Options<'a> {
    fn parse(argv: &'a [OsString]) -> Result<Self, String> {
        let (mut source, mut store, mut profile, mut json) = (None, None, None, false);
        let mut parse_flags = true;
        let mut args = argv.iter().map(OsString::as_os_str);
        while let Some(arg) = args.next() {
            match arg.to_str().filter(|_| parse_flags) {
                Some("--store") if store.is_none() => {
                    let value = take_value(args.next(), "--store")?;
                    store = Some(Path::new(value));
                }
                Some("--profile") if profile.is_none() => {
                    let value = take_value(args.next(), "--profile")?;
                    profile = Some(value.to_str().and_then(LshProfile::from_name)
                        .ok_or_else(|| format!("unknown profile {value:?}; use 32, 64, 64_32, nosize, jvm, or dalvik"))?);
                }
                Some("--json") => json = true,
                Some("--") => parse_flags = false,
                _ if parse_flags && arg.as_encoded_bytes().starts_with(b"-") => {
                    return Err(format!("unknown or repeated flag {arg:?}"));
                }
                _ if source.is_none() => source = Some(Path::new(arg)),
                _ => return Err(format!("expected one source snapshot, got extra argument {arg:?}")),
            }
        }
        Ok(Self {
            source: source.ok_or("a source snapshot is required")?,
            store: store.ok_or("--store DIR is required")?,
            profile,
            json,
        })
    }
}

fn take_value<'a>(value: Option<&'a OsStr>, flag: &str) -> Result<&'a OsStr, String> {
    value.filter(|s| !s.is_empty() && !s.as_encoded_bytes().starts_with(b"--"))
        .ok_or_else(|| format!("{flag} requires a value"))
}

#[derive(Serialize)]
struct Report<'a> {
    source: Cow<'a, str>,
    store: Cow<'a, str>,
    imported: usize,
    incomplete: usize,
    skipped: usize,
    failed: usize,
    items: &'a [Item],
}

#[derive(Serialize)]
struct Item {
    item: String,
    #[serde(flatten)]
    status: Status,
}

#[derive(Serialize)]
#[serde(tag = "status", rename_all = "snake_case")]
enum Status {
    Imported {
        display_name: String,
        version: Option<i32>,
        profile: &'static str,
        sha256: String,
        outcome: ImportOutcome,
        signed: usize,
        considered: usize,
        skipped_unnamed: usize,
        snapshot_complete: bool,
        executable_complete: bool,
        inserted: usize,
        updated: usize,
        unchanged: usize,
        kept_published: usize,
        #[serde(skip_serializing_if = "Option::is_none")]
        first_failure: Option<String>,
    },
    Skipped { skipped_unnamed: usize },
    Error { error: String },
}

fn import(options: &Options<'_>) -> Result<Vec<Item>, String> {
    let source = Source::open(options.source)?;
    let store_dir = resolve_destination(options.store).map_err(|e| at(options.store, e))?;
    if store_dir.starts_with(source.path()) {
        return Err(at(&store_dir, "--store must be outside the source snapshot"));
    }
    let candidates = source.candidates();
    if candidates.is_empty() {
        return Err(at(options.source, "the snapshot has no indexed items"));
    }

    // Do not create a store for an input that contains no usable candidates.
    let store = candidates.iter().any(|(_, result)| result.is_ok())
        .then(|| Store::open(&store_dir).map_err(|e| at(&store_dir, e)));
    let mut items = Vec::with_capacity(candidates.len());
    for (item, candidate) in candidates {
        let status = candidate.and_then(|database| {
            let store = store.as_ref().expect("completed store selection for usable candidates")
                .as_ref().map_err(String::clone)?;
            import_one(database, store, &store_dir, options.profile)
        }).unwrap_or_else(|error| Status::Error { error });
        items.push(Item { item, status });
    }
    Ok(items)
}

fn import_one(
    database: Database,
    store: &Store,
    store_dir: &Path,
    profile: Option<LshProfile>,
) -> Result<Status, String> {
    let bytes = read_database(&database.path, MAX_DATABASE_BYTES)?;
    let ghidra_program::ProgramSnapshot { original, functions, skipped_unnamed, language_id } =
        ghidra_program::read(&bytes, MAX_DATABASE_BYTES, MAX_ARTIFACT_BYTES as usize)
            .map_err(|e| at(&database.path, e.explain()))?;
    drop(bytes);
    if functions.is_empty() {
        return Ok(Status::Skipped { skipped_unnamed });
    }

    // The signer takes a pathname. Keep a uniquely created private file alive
    // through signing, and never derive its on-disk name from snapshot metadata.
    let mut executable = tempfile::NamedTempFile::new_in(store_dir)
        .map_err(|e| at(store_dir, format!("creating signing input: {e}")))?;
    executable.write_all(&original)
        .map_err(|e| at(executable.path(), format!("writing signing input: {e}")))?;
    let binary = executable.path().to_str().ok_or("signing input path is not UTF-8; use a UTF-8 store path")?;
    let (profile, artifact, coverage) = sign_import(binary, &language_id, functions, profile, database.name.clone())
        .map_err(|e| at(&database.path, e))?;
    drop(executable);
    let signed = artifact.functions.len();
    let (result, merge) = store.import_project(profile, artifact, &original)
        .map_err(|e| at(&database.path, format!("publishing to store: {e}")))?;
    Ok(Status::Imported {
        display_name: database.name,
        version: database.version,
        profile: profile.name(),
        sha256: result.sha256,
        outcome: result.outcome,
        signed,
        considered: coverage.considered,
        skipped_unnamed,
        snapshot_complete: coverage.first_failure.is_none(),
        executable_complete: merge.executable_complete,
        inserted: merge.inserted,
        updated: merge.updated,
        unchanged: merge.unchanged,
        kept_published: merge.kept_published,
        first_failure: coverage.first_failure,
    })
}

fn read_database(path: &Path, max_bytes: usize) -> Result<Vec<u8>, String> {
    check_kind(path, Kind::File)?;
    let mut file = File::open(path).map_err(|e| at(path, e))?;
    let meta = file.metadata().map_err(|e| at(path, e))?;
    if !meta.is_file() {
        return Err(at(path, "database must be a regular file"));
    }
    let len = meta.len();
    if len > max_bytes as u64 {
        return Err(at(path, format!("database exceeds the {max_bytes}-byte input limit")));
    }
    let mut bytes = Vec::with_capacity(len as usize);
    Read::by_ref(&mut file).take(len).read_to_end(&mut bytes).map_err(|e| at(path, e))?;
    if bytes.len() as u64 != len || file.read(&mut [0]).map_err(|e| at(path, e))? != 0 {
        return Err(at(path, "database size changed while reading; use a closed or copied snapshot"));
    }
    Ok(bytes)
}

enum Source {
    Project(PathBuf),
    Archive(PathBuf, String),
}

impl Source {
    fn open(path: &Path) -> Result<Self, String> {
        let repository = match path.extension().and_then(OsStr::to_str) {
            Some("gzf") => {
                check_kind(path, Kind::File)?;
                let name = path.file_stem().ok_or_else(|| at(path, "missing archive basename"))?
                    .to_string_lossy().into_owned();
                return Ok(Self::Archive(path.canonicalize().map_err(|e| at(path, e))?, name));
            }
            Some("gpr") => {
                check_kind(path, Kind::File)?;
                path.with_extension("rep")
            }
            Some("rep") => path.to_owned(),
            _ => return Err(at(path, "expected a .gpr project marker, .rep directory, or .gzf archive")),
        };
        check_kind(&repository, Kind::Directory)?;
        let repository = repository.canonicalize().map_err(|e| at(&repository, e))?;
        let lock = repository.with_extension("lock");
        match fs::symlink_metadata(&lock) {
            Ok(_) => return Err(at(&lock, "project lock is present; close Ghidra or use a consistent copied .rep snapshot")),
            Err(err) if err.kind() == io::ErrorKind::NotFound => {}
            Err(err) => return Err(at(&lock, err)),
        }
        Ok(Self::Project(repository))
    }

    fn path(&self) -> &Path {
        match self {
            Self::Project(root) | Self::Archive(root, _) => root,
        }
    }

    fn candidates(self) -> Vec<Candidate> {
        match self {
            Self::Project(root) => project::discover(&root),
            Self::Archive(path, name) => {
                let item = path.display().to_string();
                vec![(item, Ok(Database { path, name, version: None }))]
            }
        }
    }
}

/// Canonicalize as far as possible without creating directories. In particular,
/// an existing symlink alias into the source must not bypass the containment test.
fn resolve_destination(path: &Path) -> io::Result<PathBuf> {
    let abs = if path.is_absolute() { path.to_owned() } else { std::env::current_dir()?.join(path) };
    let mut unresolved = abs.as_path();
    let mut missing = Vec::new();
    loop {
        match unresolved.canonicalize() {
            Ok(mut resolved) => {
                resolved.extend(missing.into_iter().rev());
                return Ok(resolved);
            }
            Err(err) if err.kind() == io::ErrorKind::NotFound => {
                match fs::symlink_metadata(unresolved) {
                    Err(e) if e.kind() == io::ErrorKind::NotFound => {}
                    Err(e) => return Err(e),
                    Ok(_) => return Err(err),
                }
                missing.push(unresolved.file_name().ok_or(err)?);
                unresolved = unresolved.parent().expect("absolute path with filename has a parent");
            }
            Err(err) => return Err(err),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_only_regular_databases_within_the_input_limit() {
        let scratch = tempfile::tempdir().unwrap();
        assert!(read_database(scratch.path(), 4).is_err());
        let db = scratch.path().join("input.gzf");
        fs::write(&db, b"data").unwrap();
        assert_eq!(read_database(&db, 4).unwrap(), b"data");
        assert!(read_database(&db, 3).is_err_and(|e| e.contains("3-byte input limit")));
    }

    #[test]
    fn rejects_a_store_inside_the_snapshot_before_creating_it() {
        let scratch = tempfile::tempdir().unwrap();
        let source = scratch.path().join("project.rep");
        fs::create_dir(&source).unwrap();
        let store = source.join("new/store");
        let options = Options { source: &source, store: &store, profile: None, json: false };
        assert!(import(&options).is_err_and(|e| e.contains("outside the source")));
        assert!(!source.join("new").exists());
    }

    #[cfg(unix)]
    #[test]
    fn rejects_store_alias_into_the_snapshot() {
        let scratch = tempfile::tempdir().unwrap();
        let source = scratch.path().join("project.rep");
        fs::create_dir(&source).unwrap();
        let alias = scratch.path().join("alias");
        std::os::unix::fs::symlink(&source, &alias).unwrap();
        let store = alias.join("subdir");
        let options = Options { source: &source, store: &store, profile: None, json: false };
        assert!(import(&options).is_err_and(|e| e.contains("outside the source")));
        assert!(!store.exists());
    }
}
