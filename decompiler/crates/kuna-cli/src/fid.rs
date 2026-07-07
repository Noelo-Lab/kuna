//! `kuna fid build` — generate a kuna-native `.fid` fingerprint database from a
//! static library (`.a`) or object file(s) (`.o`).
//!
//! This is the §5.2 `FidServiceLibraryIngest` analog (FID PR3): for each input
//! object it bootstraps the architecture in-process via
//! [`kuna_console::engine::bootstrap_from_object`] (which gives the live SLEIGH
//! decoder the FID generator's `instruction_mask` walk needs), disassembles every
//! named function through the Listing path, runs the byte-exact FID hasher, and
//! writes the deduplicated records to a `.fid` file.
//!
//! ```text
//!   kuna fid build <lib.a|*.o ...> -o <out.fid> --lang <id> --cspec <id>
//! ```
//!
//! `--lang` / `--cspec` are authoritative: they are written verbatim into the
//! `.fid` header (the matching pass refuses a program whose language id differs).
//! A `.a` archive is unpacked member-by-member (each member is bootstrapped as a
//! standalone object); a `.o`/linked image is bootstrapped directly.

use std::path::Path;

use kuna_analysis::loadimage_object::ObjectLoadImage;
use kuna_analysis::s1_fid::build::build_records;
use kuna_analysis::s1_fid::db::{FidDb, FidRecord};
use kuna_console::engine::bootstrap_from_object;

use crate::paths;

/// Parsed `kuna fid build` arguments.
struct BuildArgs {
    inputs: Vec<String>,
    out: Option<String>,
    lang: Option<String>,
    cspec: Option<String>,
}

/// Dispatch `kuna fid <subcommand>`. Currently only `build` exists.
pub fn run(argv: &[String]) -> i32 {
    let Some(sub) = argv.first() else {
        eprintln!("kuna fid: missing subcommand (try `kuna fid build ...`)");
        usage();
        return 2;
    };
    match sub.as_str() {
        "build" => cmd_build(&argv[1..]),
        "-h" | "--help" | "help" => {
            usage();
            0
        }
        other => {
            eprintln!("kuna fid: unknown subcommand {other:?}");
            usage();
            2
        }
    }
}

fn usage() {
    eprintln!(
        "usage: kuna fid build <lib.a|*.o ...> -o <out.fid> --lang <id> --cspec <id>\n\
         \n\
         Generate a kuna `.fid` fingerprint database from a static library or\n\
         object files. --lang is the SLEIGH language id (e.g. x86:LE:64:default),\n\
         --cspec the compiler-spec id (e.g. gcc); both are written into the header."
    );
}

fn cmd_build(argv: &[String]) -> i32 {
    let args = match parse_build_args(argv) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("kuna fid build: {e}");
            usage();
            return 2;
        }
    };

    let out = match &args.out {
        Some(o) => o.clone(),
        None => {
            eprintln!("kuna fid build: -o <out.fid> is required");
            usage();
            return 2;
        }
    };
    let lang = match &args.lang {
        Some(l) => l.clone(),
        None => {
            eprintln!("kuna fid build: --lang <id> is required");
            usage();
            return 2;
        }
    };
    let cspec = args.cspec.clone().unwrap_or_default();
    if args.inputs.is_empty() {
        eprintln!("kuna fid build: at least one input object/archive is required");
        usage();
        return 2;
    }

    // The spec root the engine resolves the architecture against (the same
    // `SLEIGHHOME`/`specs/` mechanism `decomp_dbg` uses).
    let spec_roots = spec_roots();

    let mut all: Vec<FidRecord> = Vec::new();
    for input in &args.inputs {
        match records_for_input(input, &spec_roots) {
            Ok(mut recs) => {
                eprintln!("kuna fid build: {input}: {} record(s)", recs.len());
                all.append(&mut recs);
            }
            Err(e) => {
                eprintln!("kuna fid build: {input}: {e}");
                return 1;
            }
        }
    }

    // Cross-input dedup of identical (full, specific, name) rows (a function
    // defined in two members hashes identically).
    let mut seen = std::collections::HashSet::new();
    all.retain(|r| seen.insert((r.full_hash, r.specific_hash, r.name.clone())));

    let db = FidDb::from_records(lang, cspec, all);
    let bytes = db.serialize();
    if let Err(e) = std::fs::write(&out, &bytes) {
        eprintln!("kuna fid build: cannot write {out}: {e}");
        return 1;
    }
    eprintln!(
        "kuna fid build: wrote {out} ({} record(s), {} bytes)",
        db.records().len(),
        bytes.len()
    );
    0
}

/// Build the FID records for one input path: a `.a` archive (each member
/// bootstrapped separately) or a single object/linked image.
fn records_for_input(input: &str, spec_roots: &[String]) -> Result<Vec<FidRecord>, String> {
    let bytes = std::fs::read(input).map_err(|e| format!("cannot read: {e}"))?;
    if is_archive(&bytes) {
        records_for_archive(input, &bytes, spec_roots)
    } else {
        records_for_object_path(input, spec_roots)
    }
}

/// Bootstrap a single object file (at a real path) and build its records.
fn records_for_object_path(path: &str, spec_roots: &[String]) -> Result<Vec<FidRecord>, String> {
    // Bootstrap the architecture from the object (resolves the SLEIGH language,
    // builds the engine, attaches the loadimage). `target = ""` ⇒ derive the
    // language from the object's machine.
    let prog = bootstrap_from_object(path, "", spec_roots)
        .map_err(|e| format!("bootstrap failed: {e}"))?;
    let arch = prog.arch();

    // Re-read the bytes and open a fresh `ObjectLoadImage` for the generator (the
    // same throwaway-loader pattern `commit_pending_analysis` uses for the
    // deferred Listing build — its `image` arg is only used to satisfy the
    // contract; the decode reads through `arch.translate()`).
    let bytes = std::fs::read(path).map_err(|e| format!("cannot re-read: {e}"))?;
    let image = ObjectLoadImage::from_bytes(path, &bytes)
        .map_err(|e| format!("loadimage failed: {e}"))?;

    Ok(build_records(
        &bytes,
        &image,
        arch,
        arch.translate().as_sleigh().expect("fid build: standalone Sleigh engine"),
    ))
}

/// Unpack a `.a` archive and build records from every object member. Each member
/// is written to a temp file (the bootstrap reads from a path) and bootstrapped
/// as a standalone object.
fn records_for_archive(
    archive_path: &str,
    bytes: &[u8],
    spec_roots: &[String],
) -> Result<Vec<FidRecord>, String> {
    use object::read::archive::ArchiveFile;

    let archive = ArchiveFile::parse(bytes).map_err(|e| format!("not a valid archive: {e}"))?;
    let stem = Path::new(archive_path)
        .file_stem()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_else(|| "archive".to_string());

    let mut out: Vec<FidRecord> = Vec::new();
    let mut idx = 0usize;
    for member in archive.members() {
        let member = member.map_err(|e| format!("archive member error: {e}"))?;
        let data = member.data(bytes).map_err(|e| format!("archive member data error: {e}"))?;
        // Skip the archive's symbol-index / string-table members (not objects).
        if !is_object(data) {
            continue;
        }
        // Materialize the member so the bootstrap (which reads a path) can open it.
        let name = String::from_utf8_lossy(member.name());
        let tmp = std::env::temp_dir().join(format!("kuna_fid_{stem}_{idx}.o"));
        idx += 1;
        std::fs::write(&tmp, data)
            .map_err(|e| format!("cannot stage member {name}: {e}"))?;
        let tmp_str = tmp.to_string_lossy().into_owned();
        let res = records_for_object_path(&tmp_str, spec_roots);
        let _ = std::fs::remove_file(&tmp);
        match res {
            Ok(mut recs) => out.append(&mut recs),
            // A member that fails to bootstrap (e.g. a non-code object) is skipped
            // with a warning, not fatal — the rest of the archive still ingests.
            Err(e) => eprintln!("kuna fid build: {archive_path}({name}): skipped: {e}"),
        }
    }
    Ok(out)
}

/// Is `bytes` a Unix `ar` archive (the `.a` magic `!<arch>\n`)?
fn is_archive(bytes: &[u8]) -> bool {
    bytes.starts_with(b"!<arch>\n")
}

/// Does `bytes` look like a single object file (ELF/Mach-O/PE/COFF magic)?
fn is_object(bytes: &[u8]) -> bool {
    bytes.starts_with(b"\x7fELF")            // ELF
        || bytes.starts_with(&[0xcf, 0xfa, 0xed, 0xfe]) // Mach-O 64 LE
        || bytes.starts_with(&[0xce, 0xfa, 0xed, 0xfe]) // Mach-O 32 LE
        || bytes.starts_with(b"MZ")          // PE
        || (bytes.len() >= 2 && bytes[0] == 0x4c && bytes[1] == 0x01) // COFF x86 (IMAGE_FILE_MACHINE_I386)
        || (bytes.len() >= 2 && bytes[0] == 0x64 && bytes[1] == 0x86) // COFF x86-64
}

/// The SLEIGH spec roots (`SLEIGHHOME` then the repo `specs/` dir), matching how
/// `decomp_dbg` resolves them.
fn spec_roots() -> Vec<String> {
    let mut roots: Vec<String> = Vec::new();
    if let Ok(home) = std::env::var("SLEIGHHOME") {
        if !home.is_empty() {
            roots.push(home);
        }
    }
    let specs = paths::specs_dir().to_string_lossy().into_owned();
    if !roots.contains(&specs) {
        roots.push(specs);
    }
    roots
}

/// Parse the `build` flags: positional inputs + `-o`/`--lang`/`--cspec`.
fn parse_build_args(argv: &[String]) -> Result<BuildArgs, String> {
    let mut a = BuildArgs { inputs: Vec::new(), out: None, lang: None, cspec: None };
    let mut i = 0;
    while i < argv.len() {
        let tok = argv[i].as_str();
        match tok {
            "-o" | "--output" => a.out = Some(take_value(argv, &mut i, tok)?),
            "--lang" => a.lang = Some(take_value(argv, &mut i, tok)?),
            "--cspec" => a.cspec = Some(take_value(argv, &mut i, tok)?),
            other if other.starts_with('-') && other != "-" => {
                return Err(format!("unknown flag {other:?}"));
            }
            other => a.inputs.push(other.to_string()),
        }
        i += 1;
    }
    Ok(a)
}

/// Consume the value following a flag at `argv[*i]`, advancing `*i` past it.
fn take_value(argv: &[String], i: &mut usize, flag: &str) -> Result<String, String> {
    *i += 1;
    argv.get(*i).cloned().ok_or_else(|| format!("{flag} requires a value"))
}
