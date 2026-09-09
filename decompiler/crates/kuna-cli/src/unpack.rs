//! `kuna unpack` -- statically unpack a UPX-packed executable so the rest of the
//! CLI has a real program to analyze.
//!
//! ```text
//!   kuna unpack <binary> [-o OUT] [--json]
//!   kuna unpack <binary> --raw-lzma START:END [--lzma-props P] [--raw-offsets]
//! ```
//!
//! A packed binary is the one input on which every other kuna surface is
//! honestly useless: `kuna functions` finds nothing because there is nothing
//! there yet -- the file holds a loader stub and a compressed blob. This
//! subcommand is the missing first step, and it runs entirely in-process
//! (`kuna_analysis::upx`): a release `kuna` cannot assume `upx -d` is installed,
//! and handing a hostile binary to a packer to look at it is not an option.
//!
//! Default output path is `<binary>.unpacked`, overwritten if it exists: the
//! name is unambiguously this command's own artifact and an agent-driven CLI
//! that fails its second invocation is worse than one that rewrites a file it
//! wrote itself.
//!
//! The failure contract is the point of the feature as much as the success one:
//! a method, filter or layout this build does not implement exits `1` naming it,
//! and never writes a file. See `kuna_analysis::upx` for why.
//!
//! ## The `--raw-lzma` override
//!
//! Discovery is not always possible. A repacker that strips the `PackHeader`,
//! or a private packer that merely borrows UPX's codec, leaves an image whose
//! payload is perfectly decodable and whose *metadata* is gone -- and the honest
//! `no UPX PackHeader found` is then a wall rather than an answer, because the
//! one thing that could get past it is knowledge the analyst already has and the
//! CLI has no way to accept.
//!
//! `--raw-lzma START:END` is that way: it skips discovery entirely and decodes
//! the given range as one raw LZMA1 stream. The range is virtual addresses --
//! what a reader copies out of `kuna disassemble` or `kuna xrefs` -- unless
//! `--raw-offsets` says they are file offsets, which is also the only reading
//! available for an image no object parser recognises. Nothing declares the
//! uncompressed size in this situation, so the decode runs to the end of the
//! input rather than to a length (`upx::lzma::decompress_exhaustive`); the size
//! is a *result* here, not a parameter.
//!
//! By default the range's first two bytes are read as UPX's parameter prefix.
//! `--lzma-props` overrides that for a stream that carries no prefix at all, and
//! then the range is stream body from its first byte.

use std::fmt::Write as _;
use std::path::PathBuf;

use kuna_analysis::neolite::{self, NeoliteError};
use kuna_analysis::upx::lzma;
use kuna_analysis::upx::{self, UpxError};
use object::{Object, ObjectSection};

use crate::jsonfmt::{dumps_indent2, Json};

/// The output ceiling a length-less decode runs under when the caller names no
/// other. Large enough for any payload an analyst is plausibly carving out by
/// hand, small enough that a range that is not really a stream fails instead of
/// eating the machine.
const DEFAULT_RAW_MAX: usize = 512 << 20;

/// UPX's parameter prefix, which `--lzma-props` supplies by hand instead.
const LZMA_PREFIX_LEN: usize = 2;

struct Args {
    binary: String,
    output: Option<String>,
    json: bool,
    raw: Option<RawLzma>,
}

/// The analyst-supplied stream description `--raw-lzma` and friends spell.
struct RawLzma {
    /// Half-open `[start, end)`, in the units `offsets` selects.
    start: u64,
    end: u64,
    /// The endpoints are file offsets rather than virtual addresses.
    offsets: bool,
    /// `(lc, lp, pb)` when the caller named them, meaning the range carries no
    /// two-byte prefix and starts at the stream's first byte.
    props: Option<(u8, u8, u8)>,
    max: usize,
}

pub fn run(argv: &[String]) -> i32 {
    let args = match parse_args(argv) {
        Ok(Some(a)) => a,
        Ok(None) => {
            usage();
            return 0;
        }
        Err(e) => {
            eprintln!("error: {e}");
            usage();
            return 2;
        }
    };

    if let Some(raw) = &args.raw {
        return run_raw(&args, raw);
    }

    let image = match std::fs::read(&args.binary) {
        Ok(bytes) => bytes,
        Err(e) => {
            eprintln!("error: could not read {}: {e}", args.binary);
            return 1;
        }
    };

    // NEOLite is recognized before UPX is even asked, on its own loader section
    // rather than on UPX's failure, so nothing about the UPX arm -- including
    // what it says about a file that is not packed at all -- changes here.
    if neolite::detect(&image) {
        return run_neolite(&args, &image);
    }

    let unpacked = match upx::unpack(&image) {
        Ok(u) => u,
        Err(e) => {
            eprintln!("error: {}: {e}", args.binary);
            if matches!(e, UpxError::NotPacked) {
                eprintln!("note: kuna unpack handles UPX-packed executables; this file is not one");
            }
            return 1;
        }
    };

    let out_path = args
        .output
        .clone()
        .unwrap_or_else(|| default_output(&args.binary).to_string_lossy().into_owned());
    if let Err(e) = std::fs::write(&out_path, &unpacked.bytes) {
        eprintln!("error: could not write {out_path}: {e}");
        return 1;
    }

    let text = if args.json {
        format!(
            "{}\n",
            dumps_indent2(&result_json(&args.binary, &out_path, &unpacked, image.len()))
        )
    } else {
        render_text(&args.binary, &out_path, &unpacked, image.len())
    };
    crate::output::emit_with_status(&text, 0)
}

/// The NEOLite arm. The original section table survives packing, so the
/// rebuilt image is the packed one with each section's stream decoded back to
/// the virtual size that table already declares, the entry point moved off the
/// stub, and the import directory pointed back at the program's own.
fn run_neolite(args: &Args, image: &[u8]) -> i32 {
    let unpacked = match neolite::unpack(image) {
        Ok(u) => u,
        Err(e) => {
            eprintln!("error: {}: {e}", args.binary);
            if let NeoliteError::Unsupported(_) = e {
                eprintln!(
                    "note: the image carries a NEOLite loader section but not a layout this \
                     build can rebuild"
                );
            }
            return 1;
        }
    };

    let out_path = args
        .output
        .clone()
        .unwrap_or_else(|| default_output(&args.binary).to_string_lossy().into_owned());
    if let Err(e) = std::fs::write(&out_path, &unpacked.bytes) {
        eprintln!("error: could not write {out_path}: {e}");
        return 1;
    }

    let text = if args.json {
        format!(
            "{}\n",
            dumps_indent2(&neolite_json(&args.binary, &out_path, &unpacked, image.len()))
        )
    } else {
        render_neolite(&args.binary, &out_path, &unpacked, image.len())
    };
    crate::output::emit_with_status(&text, 0)
}

fn render_neolite(
    binary: &str,
    out_path: &str,
    u: &neolite::Unpacked,
    packed_size: usize,
) -> String {
    let mut s = String::new();
    let _ = writeln!(s, "{binary}: NEOLite-packed");
    let _ = writeln!(
        s,
        "  entry point   {:#x} (stub entry {:#x})",
        u.entry, u.stub_entry
    );
    match u.imports {
        Some(rva) => {
            let _ = writeln!(s, "  imports       {rva:#x} (recovered)");
        }
        None => {
            let _ = writeln!(s, "  imports       not found -- the stub's own table is kept");
        }
    }
    let _ = writeln!(
        s,
        "  {} of {} sections decompressed, {} -> {} bytes",
        u.compressed_sections(),
        u.sections.len(),
        packed_size,
        u.bytes.len()
    );
    for sec in &u.sections {
        let _ = writeln!(
            s,
            "    {:<10} {:#010x}  {:>9} -> {:>9}{}",
            sec.name,
            sec.va,
            sec.packed,
            sec.unpacked,
            if sec.compressed { "" } else { "  (stored)" }
        );
    }
    let _ = writeln!(s, "  wrote {out_path}");
    s
}

fn neolite_json(
    binary: &str,
    out_path: &str,
    u: &neolite::Unpacked,
    packed_size: usize,
) -> Json {
    let sections: Vec<Json> = u
        .sections
        .iter()
        .map(|sec| {
            Json::Object(vec![
                ("name".into(), Json::Str(sec.name.clone())),
                ("va".into(), num(u64::from(sec.va))),
                ("va_hex".into(), Json::Str(format!("{:#x}", sec.va))),
                ("packed_size".into(), num(u64::from(sec.packed))),
                ("unpacked_size".into(), num(u64::from(sec.unpacked))),
                ("compressed".into(), Json::Bool(sec.compressed)),
            ])
        })
        .collect();
    Json::Object(vec![
        ("binary".into(), Json::Str(binary.into())),
        ("output".into(), Json::Str(out_path.into())),
        ("packer".into(), Json::Str("neolite".into())),
        ("entry".into(), num(u64::from(u.entry))),
        ("entry_hex".into(), Json::Str(format!("{:#x}", u.entry))),
        ("stub_entry".into(), num(u64::from(u.stub_entry))),
        ("stub_entry_hex".into(), Json::Str(format!("{:#x}", u.stub_entry))),
        (
            "import_directory".into(),
            match u.imports {
                Some(rva) => num(u64::from(rva)),
                None => Json::Null,
            },
        ),
        ("packed_size".into(), num(packed_size as u64)),
        ("compressed_size".into(), num(u.compressed_bytes())),
        ("unpacked_size".into(), num(u.bytes.len() as u64)),
        ("count".into(), num(u.compressed_sections() as u64)),
        ("sections".into(), Json::Array(sections)),
    ])
}

/// The `--raw-lzma` arm: no discovery, no `PackHeader`, no declared length --
/// the caller's range and the decoder, and whatever comes out.
fn run_raw(args: &Args, raw: &RawLzma) -> i32 {
    // The loader's reader, not `std::fs::read`: a Mach-O fat wrapper has to be
    // peeled before a virtual address means anything, and the addresses the
    // caller is quoting came out of a kuna surface that had already peeled it.
    let image = match kuna_analysis::loader::elf_shdr::read_image(&args.binary) {
        Ok(bytes) => bytes,
        Err(e) => {
            eprintln!("error: could not read {}: {e}", args.binary);
            return 1;
        }
    };

    let (from, to, kind) = match map_range(&image, raw) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("error: {}: {e}", args.binary);
            return 1;
        }
    };

    // With explicit properties the range is body from its first byte, so the
    // prefix the decoder wants is synthesized rather than read.
    let slice = &image[from..to];
    let owned;
    let (stream, synthesized): (&[u8], usize) = match raw.props {
        None => (slice, 0),
        Some((lc, lp, pb)) => {
            owned = [&[pb, (lp << 4) | lc][..], slice].concat();
            (&owned, LZMA_PREFIX_LEN)
        }
    };

    let decoded = match lzma::decompress_exhaustive(stream, raw.max) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("error: {}: raw LZMA1 decode failed: {e}", args.binary);
            eprintln!(
                "note: the range is decoded as one raw LZMA1 stream; check the endpoints, \
                 and pass --lzma-props if the stream carries no two-byte parameter prefix"
            );
            return 1;
        }
    };

    let out_path = args
        .output
        .clone()
        .unwrap_or_else(|| default_output(&args.binary).to_string_lossy().into_owned());
    if let Err(e) = std::fs::write(&out_path, &decoded.bytes) {
        eprintln!("error: could not write {out_path}: {e}");
        return 1;
    }

    let report = RawReport {
        binary: &args.binary,
        out_path: &out_path,
        raw,
        kind,
        from,
        to,
        props: lzma_props(stream),
        // Reported against the caller's range, so it is comparable with the
        // range's own length whether or not the prefix was synthesized.
        consumed: decoded.consumed - synthesized,
        decoded: &decoded,
        packed_size: image.len(),
    };
    let text = if args.json {
        format!("{}\n", dumps_indent2(&raw_json(&report)))
    } else {
        render_raw(&report)
    };
    crate::output::emit_with_status(&text, 0)
}

/// Everything both renderings of a `--raw-lzma` run report.
struct RawReport<'a> {
    binary: &'a str,
    out_path: &'a str,
    raw: &'a RawLzma,
    /// [`KIND_VMA`] or [`KIND_FILE`] -- how the endpoints were read.
    kind: &'a str,
    /// The file bytes the range resolved to.
    from: usize,
    to: usize,
    props: (u8, u8, u8),
    /// Bytes of the caller's range the range coder actually read.
    consumed: usize,
    decoded: &'a lzma::Exhausted,
    packed_size: usize,
}

/// `(lc, lp, pb)` as the two prefix bytes spell them. The decode already
/// succeeded, so the slice is long enough and the values are in range.
fn lzma_props(stream: &[u8]) -> (u8, u8, u8) {
    (stream[1] & 0x0f, stream[1] >> 4, stream[0] & 0x07)
}

/// Which units the endpoints were read in -- reported, because a range that
/// resolved as a file offset when the caller meant an address is the failure
/// mode that produces a plausible-looking wrong answer.
const KIND_VMA: &str = "vma";
const KIND_FILE: &str = "file-offset";

/// Resolve the caller's range to a half-open byte range of `image`.
///
/// Virtual addresses are mapped through the object's own section table -- the
/// same table the decompiler read the address out of. The end is clamped to the
/// section's *raw* data, since a section's virtual size routinely runs past what
/// the file stores and those bytes are zero-fill, not stream.
fn map_range<'a>(image: &'a [u8], raw: &RawLzma) -> Result<(usize, usize, &'a str), String> {
    if raw.end <= raw.start {
        return Err(format!(
            "range {:#x}:{:#x} is empty -- the end is exclusive and must be above the start",
            raw.start, raw.end
        ));
    }
    if raw.offsets {
        let (from, to) = (raw.start as usize, raw.end as usize);
        if to > image.len() {
            return Err(format!(
                "file offset range {:#x}:{:#x} runs past the end of the {}-byte file",
                raw.start,
                raw.end,
                image.len()
            ));
        }
        return Ok((from, to, KIND_FILE));
    }

    let file = object::File::parse(image).map_err(|e| {
        format!(
            "{e}: the range cannot be read as virtual addresses without a section table -- \
             pass --raw-offsets to read {:#x}:{:#x} as file offsets instead",
            raw.start, raw.end
        )
    })?;
    for section in file.sections() {
        let Some((file_off, file_len)) = section.file_range() else { continue };
        let addr = section.address();
        let span = section.size().max(file_len);
        if span == 0 || raw.start < addr || raw.start - addr >= span {
            continue;
        }
        let from = file_off + (raw.start - addr);
        let stored_end = file_off + file_len;
        let want_end = file_off + (raw.end - addr);
        let to = want_end.min(stored_end).min(image.len() as u64);
        if to <= from {
            return Err(format!(
                "{:#x} is inside {} but the section stores no bytes there",
                raw.start,
                section.name().unwrap_or("<unnamed>")
            ));
        }
        return Ok((from as usize, to as usize, KIND_VMA));
    }
    Err(format!(
        "{:#x} is not inside any section with file-backed contents -- \
         pass --raw-offsets if it is a file offset",
        raw.start
    ))
}

fn render_raw(r: &RawReport<'_>) -> String {
    let (lc, lp, pb) = r.props;
    let mut s = String::new();
    let _ = writeln!(s, "unpacked {} -> {}", r.binary, r.out_path);
    let _ = writeln!(s, "  codec:       raw LZMA1 (analyst-supplied range)");
    let _ = writeln!(s, "  range:       {:#x}:{:#x} ({})", r.raw.start, r.raw.end, r.kind);
    let _ = writeln!(s, "  file bytes:  {:#x}..{:#x}", r.from, r.to);
    let _ = writeln!(s, "  properties:  lc={lc} lp={lp} pb={pb}");
    let _ = writeln!(
        s,
        "  compressed:  {} bytes ({} read)",
        r.to - r.from,
        r.consumed
    );
    let _ = writeln!(s, "  unpacked:    {} bytes", r.decoded.bytes.len());
    let _ = writeln!(s, "  end marker:  {}", r.decoded.end_marker);
    s
}

fn raw_json(r: &RawReport<'_>) -> Json {
    let (lc, lp, pb) = r.props;
    Json::Object(vec![
        ("binary".into(), Json::Str(r.binary.into())),
        ("output".into(), Json::Str(r.out_path.into())),
        ("packer".into(), Json::Str("raw-lzma".into())),
        ("codec".into(), Json::Str("lzma1".into())),
        ("range_kind".into(), Json::Str(r.kind.into())),
        ("range_start".into(), num(r.raw.start)),
        ("range_start_hex".into(), Json::Str(format!("{:#x}", r.raw.start))),
        ("range_end".into(), num(r.raw.end)),
        ("range_end_hex".into(), Json::Str(format!("{:#x}", r.raw.end))),
        ("file_offset".into(), num(r.from as u64)),
        ("file_offset_hex".into(), Json::Str(format!("{:#x}", r.from))),
        ("lc".into(), num(u64::from(lc))),
        ("lp".into(), num(u64::from(lp))),
        ("pb".into(), num(u64::from(pb))),
        ("packed_size".into(), num(r.packed_size as u64)),
        ("compressed_size".into(), num((r.to - r.from) as u64)),
        ("compressed_read".into(), num(r.consumed as u64)),
        ("unpacked_size".into(), num(r.decoded.bytes.len() as u64)),
        ("end_marker".into(), Json::Bool(r.decoded.end_marker)),
    ])
}

/// `<binary>.unpacked` -- an extra suffix rather than a replaced one, so the
/// packed original stays identifiable next to it.
fn default_output(binary: &str) -> PathBuf {
    let mut name = binary.to_string();
    name.push_str(".unpacked");
    PathBuf::from(name)
}

fn render_text(binary: &str, out_path: &str, u: &upx::Unpacked, packed_size: usize) -> String {
    let info = &u.info;
    let mut s = String::new();
    let _ = writeln!(s, "unpacked {binary} -> {out_path}");
    let _ = writeln!(s, "  packer:      UPX (loader version {})", info.version);
    let _ = writeln!(s, "  format:      {} ({})", info.format, info.format_name());
    let _ = writeln!(s, "  method:      {} ({})", info.method, info.method_name());
    let _ = writeln!(s, "  level:       {}", info.level);
    let _ = writeln!(s, "  filter:      {:#04x}", info.filter);
    let _ = writeln!(s, "  blocks:      {}", u.blocks.len());
    let _ = writeln!(
        s,
        "  compressed:  {} bytes (packed file {packed_size} bytes)",
        u.compressed_bytes()
    );
    let _ = writeln!(s, "  unpacked:    {} bytes", u.bytes.len());
    s
}

fn result_json(binary: &str, out_path: &str, u: &upx::Unpacked, packed_size: usize) -> Json {
    let info = &u.info;
    let blocks: Vec<Json> = u
        .blocks
        .iter()
        .map(|b| {
            Json::Object(vec![
                ("offset".into(), num(b.offset as u64)),
                ("offset_hex".into(), Json::Str(format!("{:#x}", b.offset))),
                ("u_len".into(), num(u64::from(b.u_len))),
                ("c_len".into(), num(u64::from(b.c_len))),
                ("method".into(), num(u64::from(b.method))),
                ("method_name".into(), Json::Str(upx::method_name(b.method).into())),
                ("filter".into(), num(u64::from(b.filter))),
                ("filter_hex".into(), Json::Str(format!("{:#04x}", b.filter))),
                ("stored".into(), Json::Bool(b.stored)),
            ])
        })
        .collect();
    Json::Object(vec![
        ("binary".into(), Json::Str(binary.into())),
        ("output".into(), Json::Str(out_path.into())),
        ("packer".into(), Json::Str("upx".into())),
        ("loader_version".into(), num(u64::from(info.version))),
        ("format".into(), num(u64::from(info.format))),
        ("format_name".into(), Json::Str(info.format_name().into())),
        ("method".into(), num(u64::from(info.method))),
        ("method_name".into(), Json::Str(info.method_name().into())),
        ("level".into(), num(u64::from(info.level))),
        ("filter".into(), num(u64::from(info.filter))),
        ("filter_hex".into(), Json::Str(format!("{:#04x}", info.filter))),
        ("pack_header_offset".into(), num(info.pack_header_offset as u64)),
        (
            "pack_header_offset_hex".into(),
            Json::Str(format!("{:#x}", info.pack_header_offset)),
        ),
        ("packed_size".into(), num(packed_size as u64)),
        ("compressed_size".into(), num(u.compressed_bytes())),
        ("unpacked_size".into(), num(u.bytes.len() as u64)),
        ("count".into(), num(u.blocks.len() as u64)),
        ("blocks".into(), Json::Array(blocks)),
    ])
}

fn num(v: u64) -> Json {
    Json::Number(v.to_string())
}

/// `Ok(None)` means `--help` was asked for -- a request, not an error.
fn parse_args(argv: &[String]) -> Result<Option<Args>, String> {
    let mut binary: Option<String> = None;
    let mut output: Option<String> = None;
    let mut json = false;
    let mut range: Option<(u64, u64)> = None;
    let mut offsets = false;
    let mut props: Option<(u8, u8, u8)> = None;
    let mut max = DEFAULT_RAW_MAX;
    let mut i = 0;
    while i < argv.len() {
        match argv[i].as_str() {
            "--json" => json = true,
            "--raw-lzma" => {
                i += 1;
                let spec = argv.get(i).ok_or_else(|| {
                    "--raw-lzma requires START:END (or START+LENGTH)".to_string()
                })?;
                range = Some(parse_range(spec)?);
            }
            "--raw-offsets" => offsets = true,
            "--lzma-props" => {
                i += 1;
                let spec = argv
                    .get(i)
                    .ok_or_else(|| "--lzma-props requires pb,lp,lc or a two-byte prefix".to_string())?;
                props = Some(parse_props(spec)?);
            }
            "--raw-max-size" => {
                i += 1;
                let spec =
                    argv.get(i).ok_or_else(|| "--raw-max-size requires a byte count".to_string())?;
                max = usize::try_from(parse_size(spec)?)
                    .map_err(|_| format!("--raw-max-size {spec:?} does not fit in memory"))?;
            }
            "-o" | "--output" => {
                i += 1;
                output = Some(
                    argv.get(i).cloned().ok_or_else(|| "-o requires a path".to_string())?,
                );
            }
            "-h" | "--help" => return Ok(None),
            s if s.starts_with('-') && s.len() > 1 => {
                return Err(format!("unknown option {s}"));
            }
            other => {
                if binary.is_some() {
                    return Err(format!("unexpected argument {other:?}"));
                }
                binary = Some(other.to_string());
            }
        }
        i += 1;
    }
    let binary = binary.ok_or_else(|| "unpack requires <binary>".to_string())?;
    // Every raw-mode modifier is inert without a range, and an analyst who
    // spelled the properties but forgot the range wants to be told so, not to
    // get a silent PackHeader search.
    if range.is_none() {
        for (flag, set) in [("--raw-offsets", offsets), ("--lzma-props", props.is_some())] {
            if set {
                return Err(format!("{flag} only applies with --raw-lzma START:END"));
            }
        }
        if max != DEFAULT_RAW_MAX {
            return Err("--raw-max-size only applies with --raw-lzma START:END".to_string());
        }
    }
    let raw = range.map(|(start, end)| RawLzma { start, end, offsets, props, max });
    Ok(Some(Args { binary, output, json, raw }))
}

/// `START:END` (end exclusive) or `START+LENGTH`. Endpoints are hexadecimal by
/// default, as everywhere else an address is typed at this CLI; the `+LENGTH`
/// form is a count and so reads decimal unless it says `0x`.
fn parse_range(spec: &str) -> Result<(u64, u64), String> {
    if let Some((lo, hi)) = spec.split_once(':') {
        return Ok((parse_vma(lo)?, parse_vma(hi)?));
    }
    if let Some((lo, len)) = spec.split_once('+') {
        let start = parse_vma(lo)?;
        let len = parse_size(len)?;
        return start
            .checked_add(len)
            .map(|end| (start, end))
            .ok_or_else(|| format!("range {spec:?} overflows"));
    }
    Err(format!("range {spec:?} is not START:END or START+LENGTH"))
}

fn parse_vma(token: &str) -> Result<u64, String> {
    let t = token.trim();
    let body = t.strip_prefix("0x").or_else(|| t.strip_prefix("0X")).unwrap_or(t);
    u64::from_str_radix(body, 16).map_err(|_| format!("invalid address {token:?}"))
}

fn parse_size(token: &str) -> Result<u64, String> {
    let t = token.trim();
    match t.strip_prefix("0x").or_else(|| t.strip_prefix("0X")) {
        Some(hex) => u64::from_str_radix(hex, 16),
        None => t.parse::<u64>(),
    }
    .map_err(|_| format!("invalid byte count {token:?}"))
}

/// `pb,lp,lc` -- the order the two UPX prefix bytes lay the fields out in -- or
/// those two bytes themselves as four hex digits, which is what a reader who
/// copied them out of the packer's setup code already has.
fn parse_props(spec: &str) -> Result<(u8, u8, u8), String> {
    let t = spec.trim();
    if !t.contains(',') {
        let body = t.strip_prefix("0x").or_else(|| t.strip_prefix("0X")).unwrap_or(t);
        let pair = u16::from_str_radix(body, 16)
            .map_err(|_| format!("invalid LZMA properties {spec:?}"))?;
        let (hi, lo) = ((pair >> 8) as u8, pair as u8);
        return Ok((lo & 0x0f, lo >> 4, hi & 0x07));
    }
    let f: Vec<&str> = t.split(',').map(str::trim).collect();
    if f.len() != 3 {
        return Err(format!("LZMA properties {spec:?} are not pb,lp,lc"));
    }
    let mut v = [0u8; 3];
    for (slot, (name, tok)) in v.iter_mut().zip(["pb", "lp", "lc"].into_iter().zip(f)) {
        *slot = tok.parse::<u8>().map_err(|_| format!("invalid {name} {tok:?}"))?;
    }
    let (pb, lp, lc) = (v[0], v[1], v[2]);
    // The prefix bytes are 3 / 4 / 4 bits wide, so anything wider is a typo the
    // synthesized header would silently truncate. LZMA1's own tighter limits
    // (lc <= 8, lc + lp <= 8) are the decoder's to enforce, and it does.
    if pb > 7 || lp > 15 || lc > 15 {
        return Err(format!("LZMA properties pb={pb} lp={lp} lc={lc} do not fit the prefix bytes"));
    }
    Ok((lc, lp, pb))
}

fn usage() {
    eprintln!("usage: kuna unpack <binary> [-o OUT] [--json]");
    eprintln!(
        "       kuna unpack <binary> --raw-lzma START:END [--lzma-props pb,lp,lc] \\\n                                    [--raw-offsets] [--raw-max-size N] [-o OUT] [--json]"
    );
    eprintln!();
    eprintln!("  --raw-lzma START:END   decode that range as one raw LZMA1 stream instead of");
    eprintln!("                         searching for a UPX PackHeader; END is exclusive and");
    eprintln!("                         START+LENGTH is accepted too. Endpoints are virtual");
    eprintln!("                         addresses, read in hex.");
    eprintln!("  --lzma-props P         the stream carries no two-byte UPX parameter prefix:");
    eprintln!("                         take it as pb,lp,lc (or as those two bytes, in hex).");
    eprintln!("  --raw-offsets          the range endpoints are file offsets, not addresses.");
    eprintln!("  --raw-max-size N       cap a length-less decode at N bytes (default 512 MiB).");
}
