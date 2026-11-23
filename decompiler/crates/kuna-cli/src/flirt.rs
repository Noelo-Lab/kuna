//! `kuna flirt make`: write FLIRT `.pat` and `.sig` files from the functions
//! of object files or static libraries.
//!
//! ```text
//!   kuna flirt make <lib.a|*.o ...> -o <out.pat> [--sig <out.sig>] [--libname NAME]
//! ```
//!
//! The patterns come from `kuna_analysis::flirt::kuna_flirtpat` and the `.sig`
//! tree from `kuna_analysis::flirt::kuna_flirtsig`. Functions the `.sig` cannot
//! tell apart are listed on stderr and left out of it.

use std::collections::BTreeSet;
use std::path::Path;

use kuna_analysis::flirt::kuna_flirtpat::{patterns_from_object, write_pat, FuncPattern, Target};
use kuna_analysis::flirt::kuna_flirtsig::build_sig;

use crate::args::take_value;

/// What the inputs of one run add up to.
#[derive(Default)]
struct Library {
    target: Option<Target>,
    patterns: Vec<FuncPattern>,
    data_names: BTreeSet<String>,
    short: usize,
}

/// Dispatch `kuna flirt <subcommand>`.
pub fn run(argv: &[String]) -> i32 {
    match argv.first().map(String::as_str) {
        Some("make") => match make(&argv[1..]) {
            Ok(()) => 0,
            Err((status, e)) => {
                eprintln!("kuna flirt make: {e}");
                if status == 2 {
                    usage();
                }
                status
            }
        },
        Some("-h" | "--help" | "help") => {
            usage();
            0
        }
        other => {
            match other {
                Some(other) => eprintln!("kuna flirt: unknown subcommand {other:?}"),
                None => eprintln!("kuna flirt: missing subcommand (try `kuna flirt make ...`)"),
            }
            usage();
            2
        }
    }
}

fn usage() {
    eprintln!(
        "usage: kuna flirt make <lib.a|*.o ...> -o <out.pat> [--sig <out.sig>] [--libname NAME]\n\
         \n\
         Write FLIRT patterns for the functions of object files or static libraries.\n\
         -o is the `.pat` text: one line per function with a global name, bytes a\n\
         relocation rewrites left variable. --sig also writes the `.sig` tree (format\n\
         version 10) that `--option flirt on` applies through KUNA_FLIRT_SIGS;\n\
         --libname is the library name in its header (default: the first input's file\n\
         stem). Functions that share their leading 32 bytes and CRC are told apart by\n\
         a tail byte, then by a referenced name; those nothing tells apart are listed\n\
         on stderr and left out of the `.sig`."
    );
}

fn make(argv: &[String]) -> Result<(), (i32, String)> {
    let usage = |e: String| (2, e);
    let (mut inputs, mut out, mut sig, mut libname) = (Vec::new(), None, None, None);
    let mut i = 0;
    while i < argv.len() {
        let tok = argv[i].as_str();
        match tok {
            "-o" | "--output" => out = Some(take_value(argv, &mut i, tok).map_err(usage)?),
            "--sig" => sig = Some(take_value(argv, &mut i, tok).map_err(usage)?),
            "--libname" => libname = Some(take_value(argv, &mut i, tok).map_err(usage)?),
            flag if flag.starts_with('-') => return Err(usage(format!("unknown flag {flag:?}"))),
            input => inputs.push(input.to_string()),
        }
        i += 1;
    }
    let out = out.ok_or_else(|| usage("-o <out.pat> is required".into()))?;
    let first = inputs.first().ok_or_else(|| usage("at least one input object or archive is required".into()))?;
    let libname = libname.unwrap_or_else(|| {
        Path::new(first).file_stem().map_or_else(String::new, |s| s.to_string_lossy().into_owned())
    });

    let mut lib = Library::default();
    for input in &inputs {
        let before = lib.patterns.len();
        add_input(&mut lib, input).map_err(|e| (1, format!("{input}: {e}")))?;
        eprintln!("kuna flirt make: {input}: {} pattern(s)", lib.patterns.len() - before);
    }
    lib.patterns.sort();
    lib.patterns.dedup();
    if lib.short > 0 {
        eprintln!("kuna flirt make: {} function(s) with too few fixed bytes left out", lib.short);
    }
    let target = lib.target.filter(|_| !lib.patterns.is_empty());
    let target = target.ok_or_else(|| (1, "the inputs define no function with a global name".to_string()))?;
    std::fs::write(&out, write_pat(&lib.patterns)).map_err(|e| (1, format!("cannot write {out}: {e}")))?;
    eprintln!("kuna flirt make: wrote {out} ({} pattern(s))", lib.patterns.len());

    let Some(sig) = sig else { return Ok(()) };
    let built = build_sig(&lib.patterns, &lib.data_names, target, &libname).map_err(|e| (1, e))?;
    for c in &built.collisions {
        eprintln!("kuna flirt make: collision: {} are not told apart; left out of {sig}", c.names.join(", "));
    }
    std::fs::write(&sig, &built.bytes).map_err(|e| (1, format!("cannot write {sig}: {e}")))?;
    eprintln!(
        "kuna flirt make: wrote {sig} ({} module(s), {} unresolved collision(s))",
        built.modules,
        built.collisions.len()
    );
    Ok(())
}

/// Add the patterns of one input: an object, or every object member of an
/// archive. Objects of another architecture, format or byte order than the
/// first are refused.
fn add_input(lib: &mut Library, input: &str) -> Result<(), String> {
    let bytes = std::fs::read(input).map_err(|e| format!("cannot read: {e}"))?;
    if !bytes.starts_with(b"!<arch>\n") {
        return add_object(lib, &bytes);
    }
    let archive = object::read::archive::ArchiveFile::parse(&*bytes).map_err(|e| format!("not a valid archive: {e}"))?;
    for member in archive.members() {
        let member = member.map_err(|e| format!("archive member error: {e}"))?;
        let data = member.data(&*bytes).map_err(|e| format!("archive member data error: {e}"))?;
        if object::File::parse(data).is_ok() {
            add_object(lib, data).map_err(|e| format!("{}: {e}", String::from_utf8_lossy(member.name())))?;
        }
    }
    Ok(())
}

fn add_object(lib: &mut Library, data: &[u8]) -> Result<(), String> {
    let mut object = patterns_from_object(data)?;
    match lib.target {
        Some(target) if target != object.target => {
            return Err(format!("a {:?} object among {:?} objects", object.target, target));
        }
        _ => lib.target = Some(object.target),
    }
    lib.patterns.append(&mut object.patterns);
    lib.data_names.append(&mut object.data_names);
    lib.short += object.short;
    Ok(())
}
