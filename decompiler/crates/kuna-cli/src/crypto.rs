//! `kuna crypto` — the crypto-constant inventory (findcrypt's question).
//!
//! ```text
//!   kuna crypto <binary> [--json] [--algorithm SUBSTR] [--section NAME] [--no-xrefs]
//!                        [--mode MODE] [--option N V].. [--isa auto|arm|thumb] [--slice ARCH]
//!                        [--target T] [--sleighpath D]
//! ```
//!
//! Scans the image's loaded, initialized sections for the well-known cipher and
//! hash constants in [`crate::cryptosig`] and reports each with its algorithm,
//! address, section, and — through the same reference index `kuna strings` and
//! `kuna xrefs` use — the functions that reference a table, or the function an
//! immediate sits in. Read-only: nothing is committed, so no emitted C changes.

use std::collections::BTreeMap;

use object::{Object, ObjectSection, ObjectSegment, SectionKind};

use crate::cryptosig::{self, Hit, Kind, Region};
use crate::decompile_all::{load_program, mode_options_for_binary, Args, DriverDefaults};
use crate::jsonfmt::{dumps_indent2, Json};

pub(crate) struct CryptoArgs {
    binary: String,
    json: bool,
    algorithm: Option<String>,
    section: Option<String>,
    no_xrefs: bool,
    options: Vec<(String, String)>,
    mode: Option<String>,
    slice: Option<String>,
    target: Option<String>,
    sleighpath: Option<String>,
    isa: Option<kuna_console::engine::ArmIsa>,
}

struct Row {
    hit: Hit,
    xrefs_count: usize,
    functions: Vec<(u64, String)>,
}

/// `kuna crypto` entry point.
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

fn is_mapped(sec: &object::Section) -> bool {
    if matches!(sec.kind(), SectionKind::UninitializedData) {
        return false;
    }
    match sec.flags() {
        object::SectionFlags::Elf { sh_flags } => sh_flags & u64::from(object::elf::SHF_ALLOC) != 0,
        object::SectionFlags::Coff { characteristics } => {
            characteristics & object::pe::IMAGE_SCN_CNT_UNINITIALIZED_DATA == 0
        }
        object::SectionFlags::MachO { flags } => {
            let t = flags & object::macho::SECTION_TYPE;
            t != object::macho::S_ZEROFILL && t != object::macho::S_GB_ZEROFILL
        }
        _ => false,
    }
}

fn is_exec_section(sec: &object::Section) -> bool {
    if sec.kind() == SectionKind::Text {
        return true;
    }
    match sec.flags() {
        object::SectionFlags::Elf { sh_flags } => sh_flags & u64::from(object::elf::SHF_EXECINSTR) != 0,
        object::SectionFlags::Coff { characteristics } => {
            characteristics & (object::pe::IMAGE_SCN_MEM_EXECUTE | object::pe::IMAGE_SCN_CNT_CODE) != 0
        }
        _ => false,
    }
}

/// The loaded+initialized sections, or the loadable segments of an image with
/// no section table.
fn regions<'d>(file: &'d object::File<'d>) -> (Vec<Region<'d>>, bool) {
    let mut out = Vec::new();
    for sec in file.sections() {
        if !is_mapped(&sec) {
            continue;
        }
        let Ok(data) = sec.data() else { continue };
        if data.is_empty() {
            continue;
        }
        out.push(Region {
            name: sec.name().ok().filter(|n| !n.is_empty()).map(str::to_string),
            vma: sec.address(),
            data,
            executable: is_exec_section(&sec),
        });
    }
    if !out.is_empty() {
        return (out, false);
    }
    for seg in file.segments() {
        let Ok(data) = seg.data() else { continue };
        if data.is_empty() {
            continue;
        }
        let executable = match seg.flags() {
            object::SegmentFlags::Elf { p_flags } => p_flags & object::elf::PF_X != 0,
            object::SegmentFlags::MachO { initprot, .. } => initprot & object::macho::VM_PROT_EXECUTE != 0,
            _ => true,
        };
        out.push(Region { name: None, vma: seg.address(), data, executable });
    }
    (out, true)
}

fn section_matches(name: Option<&str>, want: &str) -> bool {
    matches!(name, Some(n) if n == want || n.strip_prefix('.') == Some(want))
}

pub(crate) fn query(args: &CryptoArgs) -> Result<String, String> {
    let bytes = crate::decompile_all::image_bytes(
        &args.binary,
        kuna_analysis::loader::macho_fat::slice_pref(args.slice.as_deref(), args.target.as_deref()),
    )?;
    let file = kuna_analysis::loadimage_object::parse_object(&*bytes)
        .map_err(|e| format!("could not parse {}: {e}", args.binary))?;
    let (mut regs, from_segments) = regions(&file);
    if let Some(want) = &args.section {
        let names: Vec<String> = regs.iter().filter_map(|r| r.name.clone()).collect();
        regs.retain(|r| section_matches(r.name.as_deref(), want));
        if regs.is_empty() {
            let have = if names.is_empty() { "the image has no named sections".into() } else { format!("have: {}", names.join(", ")) };
            return Err(format!("no section named {want:?} ({have})"));
        }
    }
    let mut hits = cryptosig::scan(&regs, file.is_little_endian());
    if let Some(want) = &args.algorithm {
        let want = want.to_ascii_lowercase();
        hits.retain(|h| h.algorithm.to_ascii_lowercase().contains(&want));
    }
    let rows = if args.no_xrefs || hits.is_empty() {
        hits.into_iter().map(|hit| Row { hit, xrefs_count: 0, functions: Vec::new() }).collect()
    } else {
        attribute(args, &file, hits)?
    };
    Ok(if args.json {
        format!("{}\n", dumps_indent2(&result_json(args, from_segments, &rows)))
    } else {
        render_text(args, from_segments, &rows)
    })
}

/// Name who uses each constant: the functions referencing any byte of a table
/// (or of a literal-pool word), or else the function whose decoded instruction
/// encodes the immediate (the nearest instruction start up to 15 bytes before it).
fn attribute(args: &CryptoArgs, file: &object::File, hits: Vec<Hit>) -> Result<Vec<Row>, String> {
    let options = mode_options_for_binary(args.mode.as_deref(), &args.binary, args.options.clone())?;
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
    let inventory: BTreeMap<u64, String> = prog
        .function_entries_canonical()
        .into_iter()
        .map(|e| (e.addr.get_offset(), e.name))
        .collect();
    let seeds: Vec<u64> = inventory.keys().copied().collect();
    let index = kuna_analysis::listing::xrefs::build(file, prog.arch(), prog.arch().translate(), &seeds);
    let mut names: BTreeMap<u64, String> = BTreeMap::new();
    let mut name_of = |entry: u64| {
        names
            .entry(entry)
            .or_insert_with(|| crate::strings::function_name(&prog, &inventory, entry))
            .clone()
    };
    Ok(hits
        .into_iter()
        .map(|hit| {
            let mut xrefs_count = 0usize;
            let mut functions: Vec<(u64, String)> = Vec::new();
            let inline = hit.kind == Kind::Immediate && index.refs_to(hit.addr).is_empty();
            if inline {
                if let Some(entry) = (1..=15u64).find_map(|k| index.function_containing(hit.addr.checked_sub(k)?)) {
                    functions.push((entry, name_of(entry)));
                }
            } else {
                for vma in hit.addr..hit.addr + hit.byte_len as u64 {
                    for r in index.refs_to(vma) {
                        xrefs_count += 1;
                        let Some(entry) = crate::strings::owning_function(&prog, &index, r.from) else {
                            continue;
                        };
                        if !functions.iter().any(|(e, _)| *e == entry) {
                            functions.push((entry, name_of(entry)));
                        }
                    }
                }
                functions.sort();
            }
            Row { hit, xrefs_count, functions }
        })
        .collect())
}

fn function_json(name: &str, addr: u64) -> Json {
    Json::Object(vec![
        ("name".into(), Json::Str(name.to_string())),
        ("address".into(), Json::Number(addr.to_string())),
        ("address_hex".into(), Json::Str(format!("0x{addr:x}"))),
    ])
}

fn optional_str(value: Option<&str>) -> Json {
    value.map_or(Json::Null, |s| Json::Str(s.to_string()))
}

fn result_json(args: &CryptoArgs, from_segments: bool, rows: &[Row]) -> Json {
    let constants = Json::Array(
        rows.iter()
            .map(|row| {
                let h = &row.hit;
                Json::Object(vec![
                    ("address".into(), Json::Number(h.addr.to_string())),
                    ("address_hex".into(), Json::Str(format!("0x{:x}", h.addr))),
                    ("algorithm".into(), Json::Str(h.algorithm.to_string())),
                    ("name".into(), Json::Str(h.name.clone())),
                    ("kind".into(), Json::Str(h.kind.as_str().to_string())),
                    ("layout".into(), Json::Str(h.layout.to_string())),
                    ("byte_length".into(), Json::Number(h.byte_len.to_string())),
                    ("section".into(), optional_str(h.section.as_deref())),
                    ("text".into(), optional_str(h.text.as_deref())),
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
        ("algorithm".into(), optional_str(args.algorithm.as_deref())),
        ("section".into(), optional_str(args.section.as_deref())),
        ("scanned".into(), Json::Str(if from_segments { "segments" } else { "sections" }.into())),
        ("xrefs".into(), Json::Bool(!args.no_xrefs)),
        ("count".into(), Json::Number(rows.len().to_string())),
        ("constants".into(), constants),
    ])
}

/// One tab-separated row per constant: address, algorithm, name, kind, layout,
/// byte length, section, reference count, functions.
fn render_text(args: &CryptoArgs, from_segments: bool, rows: &[Row]) -> String {
    use std::fmt::Write as _;
    let mut out = String::new();
    let plural = if rows.len() == 1 { "constant" } else { "constants" };
    let _ = writeln!(
        out,
        "# {} crypto {plural} in {} (scanned by {})",
        rows.len(),
        args.binary,
        if from_segments { "segments" } else { "sections" }
    );
    for row in rows {
        let h = &row.hit;
        let functions = if row.functions.is_empty() {
            "-".to_string()
        } else {
            row.functions.iter().map(|(_, n)| n.as_str()).collect::<Vec<_>>().join(",")
        };
        let name = match &h.text {
            Some(t) if h.kind == Kind::Alphabet => format!("{} {t}", h.name),
            _ => h.name.clone(),
        };
        let _ = writeln!(
            out,
            "0x{:x}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            h.addr,
            h.algorithm,
            name,
            h.kind.as_str(),
            h.layout,
            h.byte_len,
            h.section.as_deref().unwrap_or("-"),
            row.xrefs_count,
            functions
        );
    }
    out
}

pub(crate) fn parse_args(argv: &[String]) -> Result<CryptoArgs, String> {
    let mut binary: Option<String> = None;
    let mut json = false;
    let mut algorithm = None;
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
            "--algorithm" => algorithm = Some(take(argv, &mut i, "--algorithm")?),
            "--section" => section = Some(take(argv, &mut i, "--section")?),
            "--no-xrefs" => no_xrefs = true,
            "--option" => {
                if i + 2 >= argv.len() {
                    return Err("--option requires NAME VALUE".into());
                }
                crate::optname::check(&argv[i + 1])?;
                options.push((argv[i + 1].clone(), argv[i + 2].clone()));
                i += 2;
            }
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
    let binary = binary.ok_or("crypto requires <binary>")?;
    if no_xrefs && isa.is_some() {
        return Err("--isa has no effect with --no-xrefs (no code is decoded)".into());
    }
    Ok(CryptoArgs { binary, json, algorithm, section, no_xrefs, options, mode, slice, target, sleighpath, isa })
}

fn take(argv: &[String], i: &mut usize, flag: &str) -> Result<String, String> {
    if *i + 1 < argv.len() {
        *i += 1;
        Ok(argv[*i].clone())
    } else {
        Err(format!("{flag} requires a value"))
    }
}

fn usage() {
    eprintln!(
        "usage: kuna crypto <binary> [--json] [--algorithm SUBSTR] [--section NAME] [--no-xrefs] \\\n\
         \x20                   [--mode auto|reliable|aggressive|fast] [--option N V].. \\\n\
         \x20                   [--isa auto|arm|thumb] [--slice ARCH] [--target T] [--sleighpath D]\n\
         \n\
         Lists well-known crypto constants in the image's mapped sections, each with\n\
         the functions that use it:\n\
         \x20 tables     AES S-box / inverse S-box / Te0-3 / Td0-3 / rcon, DES S1 / IP / PC-1,\n\
         \x20            MD5 T, SHA-1 IV, SHA-224/256/384/512 IV and K (SHA-256/512 IV are\n\
         \x20            also the BLAKE2s/2b IV), CRC-32 / CRC-32C tables, Blowfish P-array and\n\
         \x20            S-box heads, ChaCha/Salsa20 \"expand 32-byte k\" / \"expand 16-byte k\";\n\
         \x20            matched as bytes, bytes widened to u32, and u32/u64 words in either\n\
         \x20            byte order\n\
         \x20 immediates TEA/XTEA delta 0x9E3779B9 (and 0x61C88647, 0xC6EF3720), RC5/RC6 P32,\n\
         \x20            CRC-32/CRC-32C polynomials, SHA-1 K0-K3, MD5/SHA-1/SHA-2 first IV\n\
         \x20            words, ChaCha \"expa\"; read in executable sections only\n\
         \x20 alphabets  Base64 alphabets: 64 distinct printable characters (optionally\n\
         \x20            followed by '='), labelled standard, URL-safe or custom\n\
         \n\
         A table row names every function that references any byte of it; an\n\
         immediate row names the function its instruction lies in.\n\
         --algorithm keeps rows whose algorithm contains SUBSTR (case-insensitive).\n\
         --no-xrefs skips the reference walk (xrefs_count 0, no functions).\n\
         \n\
         --json emits {{binary,algorithm,section,scanned,xrefs,count,constants:[{{address,\n\
         address_hex,algorithm,name,kind,layout,byte_length,section,text,xrefs_count,\n\
         functions}}]}}; without it, one tab-separated row each."
    );
}
