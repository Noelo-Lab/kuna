//! `kuna blobs` — the embedded-file inventory.
//!
//! ```text
//!   kuna blobs <binary> [--json] [--extract DIR] [--kind LIST] [--section NAME] [--no-xrefs]
//!                       [--mode MODE] [--option N V].. [--isa auto|arm|thumb] [--slice ARCH]
//!                       [--target T] [--sleighpath D]
//! ```
//!
//! Scans the image's mapped sections with [`kuna_analysis::blobs`] for whole
//! PNG, JPEG, GIF, BMP, ICO, zip, gzip, xz and zstd files and reports each with
//! its address, exact length, section, and the functions that reference any of
//! its bytes (the reference index `kuna strings` and `kuna xrefs` use).
//! `--extract DIR` also writes each file out. Read-only on the image.

use std::collections::BTreeMap;
use std::path::Path;

use kuna_analysis::blobs::{self, Blob, BlobKind};

use crate::args::take_value as take;
use crate::decompile_all::{load_program, mode_options_for_binary, Args, DriverDefaults};
use crate::function_info::function_json;
use crate::jsonfmt::{dumps_indent2, Json};

pub(crate) struct BlobArgs {
    binary: String,
    json: bool,
    extract: Option<String>,
    kinds: Option<Vec<BlobKind>>,
    section: Option<String>,
    no_xrefs: bool,
    options: Vec<(String, String)>,
    mode: Option<String>,
    slice: Option<String>,
    target: Option<String>,
    sleighpath: Option<String>,
    isa: Option<kuna_console::engine::ArmIsa>,
}

struct Row<'d> {
    addr: u64,
    blob: Blob,
    section: Option<String>,
    bytes: &'d [u8],
    xrefs_count: usize,
    functions: Vec<(u64, String)>,
    file: Option<String>,
}

/// `kuna blobs` entry point.
pub fn run(argv: &[String]) -> i32 {
    let args = match parse_args(argv) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("error: {e}");
            usage();
            return 2;
        }
    };
    match query(&args) {
        Ok(text) => crate::output::emit_with_status(&text, 0),
        Err(e) => {
            eprintln!("error: {e}");
            1
        }
    }
}

fn section_matches(name: Option<&str>, want: &str) -> bool {
    matches!(name, Some(n) if n == want || n.strip_prefix('.') == Some(want))
}

pub(crate) fn query(args: &BlobArgs) -> Result<String, String> {
    let bytes = crate::image::image_bytes(
        &args.binary,
        kuna_analysis::loader::macho_fat::slice_pref(args.slice.as_deref(), args.target.as_deref()),
    )?;
    let options = mode_options_for_binary(args.mode.as_deref(), &args.binary, args.options.clone())?;
    let _loadtime_env = crate::loadtime::apply_to_process(&options, args.slice.as_deref(), 1, false);
    let raw = kuna_analysis::loadimage_object::parse_object(&bytes)
        .map_err(|e| format!("could not parse {}: {e}", args.binary))?;
    let view = kuna_analysis::loader::kuna_relocrebase::rebased_view(&raw, &bytes);
    let (file, _) = kuna_analysis::loader::kuna_relocrebase::select(raw, &bytes, &view);
    let (mut regions, from_segments) = blobs::mapped_regions(&file);
    if let Some(want) = &args.section {
        let names: Vec<String> = regions.iter().filter_map(|r| r.name.clone()).collect();
        regions.retain(|r| section_matches(r.name.as_deref(), want));
        if regions.is_empty() {
            let have = if names.is_empty() {
                "the image has no named sections".into()
            } else {
                format!("have: {}", names.join(", "))
            };
            return Err(format!("no section named {want:?} ({have})"));
        }
    }
    let mut rows: Vec<Row> = Vec::new();
    for region in &regions {
        for blob in blobs::scan(region.data) {
            if args.kinds.as_ref().is_some_and(|kinds| !kinds.contains(&blob.kind)) {
                continue;
            }
            rows.push(Row {
                addr: region.vma.wrapping_add(blob.offset as u64),
                blob,
                section: region.name.clone(),
                bytes: &region.data[blob.offset..blob.offset + blob.len],
                xrefs_count: 0,
                functions: Vec::new(),
                file: None,
            });
        }
    }
    rows.sort_by_key(|row| row.addr);
    if !args.no_xrefs && !rows.is_empty() {
        attribute(args, options, &file, &mut rows)?;
    }
    if let Some(dir) = &args.extract {
        extract(Path::new(dir), &mut rows)?;
    }
    Ok(if args.json {
        format!("{}\n", dumps_indent2(&result_json(args, from_segments, &rows)))
    } else {
        render_text(args, from_segments, &rows)
    })
}

/// Name the functions that reference any byte of each blob.
fn attribute(
    args: &BlobArgs,
    options: Vec<(String, String)>,
    file: &object::File,
    rows: &mut [Row],
) -> Result<(), String> {
    let load = Args {
        binary: args.binary.clone(),
        json: args.json,
        names: None,
        addrs: Vec::new(),
        no_vars: true,
        max_fn_seconds: 0,
        options,
        func_decls: Vec::new(),
        assertions: Vec::new(),
        assert_strict: false,
        slice: args.slice.clone(),
        target: args.target.clone(),
        sleighpath: args.sleighpath.clone(),
        isa: args.isa,
        raw_image: false,
        base: None,
        ..Args::serial()
    };
    let prog = load_program(&load, DriverDefaults::Inventory)?;
    let inventory: BTreeMap<u64, String> =
        prog.function_entries_canonical().into_iter().map(|e| (e.addr.get_offset(), e.name)).collect();
    let seeds: Vec<u64> = inventory.keys().copied().collect();
    let index = kuna_analysis::listing::xrefs::build(file, prog.arch(), prog.arch().translate(), &seeds);
    for row in rows {
        let mut owners: BTreeMap<u64, String> = BTreeMap::new();
        for xref in index.refs_into(row.addr..row.addr.saturating_add(row.blob.len as u64)) {
            row.xrefs_count += 1;
            if let Some(entry) = crate::function_info::owning_function(&prog, &index, xref.from) {
                owners
                    .entry(entry)
                    .or_insert_with(|| crate::function_info::function_name(&prog, &inventory, entry));
            }
        }
        row.functions = owners.into_iter().collect();
    }
    Ok(())
}

/// Write each blob to `dir` as `<kind>_<addr>.<ext>`.
fn extract(dir: &Path, rows: &mut [Row]) -> Result<(), String> {
    std::fs::create_dir_all(dir).map_err(|e| format!("could not create {}: {e}", dir.display()))?;
    for row in rows {
        let name = format!("{}_{:x}.{}", row.blob.kind.as_str(), row.addr, row.blob.kind.extension());
        let path = dir.join(name);
        std::fs::write(&path, row.bytes).map_err(|e| format!("could not write {}: {e}", path.display()))?;
        row.file = Some(path.to_string_lossy().into_owned());
    }
    Ok(())
}

fn optional_str(value: Option<&str>) -> Json {
    value.map_or(Json::Null, |s| Json::Str(s.to_string()))
}

fn result_json(args: &BlobArgs, from_segments: bool, rows: &[Row]) -> Json {
    let blobs = Json::Array(
        rows.iter()
            .map(|row| {
                Json::Object(vec![
                    ("address".into(), Json::Number(row.addr.to_string())),
                    ("address_hex".into(), Json::Str(format!("0x{:x}", row.addr))),
                    ("kind".into(), Json::Str(row.blob.kind.as_str().to_string())),
                    ("length".into(), Json::Number(row.blob.len.to_string())),
                    ("section".into(), optional_str(row.section.as_deref())),
                    ("file".into(), optional_str(row.file.as_deref())),
                    ("xrefs_count".into(), Json::Number(row.xrefs_count.to_string())),
                    (
                        "functions".into(),
                        Json::Array(row.functions.iter().map(|(a, n)| function_json(n, *a)).collect()),
                    ),
                ])
            })
            .collect(),
    );
    Json::Object(vec![
        ("binary".into(), Json::Str(args.binary.clone())),
        ("section".into(), optional_str(args.section.as_deref())),
        ("scanned".into(), Json::Str(if from_segments { "segments" } else { "sections" }.into())),
        ("xrefs".into(), Json::Bool(!args.no_xrefs)),
        ("count".into(), Json::Number(rows.len().to_string())),
        ("blobs".into(), blobs),
    ])
}

/// One tab-separated row per blob: address, kind, length, section, reference
/// count, functions, and the extracted file when `--extract` wrote one.
fn render_text(args: &BlobArgs, from_segments: bool, rows: &[Row]) -> String {
    use std::fmt::Write as _;
    let mut out = String::new();
    let plural = if rows.len() == 1 { "blob" } else { "blobs" };
    let _ = writeln!(
        out,
        "# {} embedded {plural} in {} (scanned by {})",
        rows.len(),
        args.binary,
        if from_segments { "segments" } else { "sections" }
    );
    for row in rows {
        let functions = if row.functions.is_empty() {
            "-".to_string()
        } else {
            row.functions.iter().map(|(_, n)| n.as_str()).collect::<Vec<_>>().join(",")
        };
        let _ = write!(
            out,
            "0x{:x}\t{}\t{}\t{}\t{}\t{}",
            row.addr,
            row.blob.kind.as_str(),
            row.blob.len,
            row.section.as_deref().unwrap_or("-"),
            row.xrefs_count,
            functions
        );
        if let Some(file) = &row.file {
            let _ = write!(out, "\t{file}");
        }
        out.push('\n');
    }
    out
}

fn parse_kinds(list: &str) -> Result<Vec<BlobKind>, String> {
    list.split(',')
        .map(|name| {
            BlobKind::ALL.into_iter().find(|k| k.as_str() == name).ok_or_else(|| {
                let known: Vec<&str> = BlobKind::ALL.iter().map(|k| k.as_str()).collect();
                format!("unknown --kind {name:?} (one of {})", known.join(", "))
            })
        })
        .collect()
}

pub(crate) fn parse_args(argv: &[String]) -> Result<BlobArgs, String> {
    let mut binary: Option<String> = None;
    let mut json = false;
    let mut extract = None;
    let mut kinds = None;
    let mut section = None;
    let mut no_xrefs = false;
    let mut options = Vec::new();
    let mut mode = None;
    let mut slice = None;
    let mut target = None;
    let mut sleighpath = None;
    let mut isa = None;
    let mut i = 0;
    while i < argv.len() {
        let a = argv[i].as_str();
        match a {
            "--json" => json = true,
            "--extract" => extract = Some(take(argv, &mut i, "--extract")?),
            "--kind" => kinds = Some(parse_kinds(&take(argv, &mut i, "--kind")?)?),
            "--section" => section = Some(take(argv, &mut i, "--section")?),
            "--no-xrefs" => no_xrefs = true,
            "--option" => options.push(crate::args::take_option(argv, &mut i)?),
            "--mode" => mode = Some(take(argv, &mut i, "--mode")?),
            "--isa" => isa = kuna_console::engine::ArmIsa::parse(&take(argv, &mut i, "--isa")?)?,
            "--slice" => slice = Some(take(argv, &mut i, "--slice")?),
            "--target" => target = Some(take(argv, &mut i, "--target")?),
            "--sleighpath" => sleighpath = Some(take(argv, &mut i, "--sleighpath")?),
            "-h" | "--help" => {
                usage();
                std::process::exit(0);
            }
            s if s.starts_with("--") => return Err(format!("unknown option {s}")),
            _ if binary.is_none() => binary = Some(a.to_string()),
            _ => return Err(format!("unexpected argument {a:?}")),
        }
        i += 1;
    }
    let binary = binary.ok_or("blobs requires <binary>")?;
    if no_xrefs && isa.is_some() {
        return Err("--isa has no effect with --no-xrefs (no code is decoded)".into());
    }
    Ok(BlobArgs { binary, json, extract, kinds, section, no_xrefs, options, mode, slice, target, sleighpath, isa })
}

fn usage() {
    eprintln!(
        "usage: kuna blobs <binary> [--json] [--extract DIR] [--kind LIST] [--section NAME] [--no-xrefs] \\\n\
         \x20                  [--mode auto|reliable|aggressive|fast] [--option N V].. \\\n\
         \x20                  [--isa auto|arm|thumb] [--slice ARCH] [--target T] [--sleighpath D]\n\
         \n\
         Lists the whole image and archive files stored in the image's mapped sections,\n\
         each with its exact length and the functions that reference it:\n\
         \x20 png   chunks to IEND, every CRC-32 checked     zip   end record and central directory\n\
         \x20 jpeg  marker segments to EOI                   gzip  inflated to the trailer's CRC-32\n\
         \x20 gif   blocks to the trailer                    xz    footer, index and their CRC-32s\n\
         \x20 bmp   header size against the pixel array      zstd  frame and block headers\n\
         \x20 ico   directory of PNG or DIB images\n\
         \n\
         A file that does not walk to its end (a truncated PNG) is not listed, and a\n\
         file stored inside a listed one (a PNG in a zip) is not listed separately.\n\
         --extract DIR writes each file to DIR as <kind>_<address>.<ext>.\n\
         --kind keeps the named kinds (comma-separated).\n\
         --no-xrefs skips the reference walk (xrefs_count 0, no functions).\n\
         \n\
         --json emits {{binary,section,scanned,xrefs,count,blobs:[{{address,address_hex,\n\
         kind,length,section,file,xrefs_count,functions}}]}}; without it, one\n\
         tab-separated row each."
    );
}
