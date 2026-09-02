//! `kuna unpack` -- statically unpack a UPX-packed executable so the rest of the
//! CLI has a real program to analyze.
//!
//! ```text
//!   kuna unpack <binary> [-o OUT] [--json]
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

use std::fmt::Write as _;
use std::path::PathBuf;

use kuna_analysis::upx::{self, UpxError};

use crate::jsonfmt::{dumps_indent2, Json};

struct Args {
    binary: String,
    output: Option<String>,
    json: bool,
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

    let image = match std::fs::read(&args.binary) {
        Ok(bytes) => bytes,
        Err(e) => {
            eprintln!("error: could not read {}: {e}", args.binary);
            return 1;
        }
    };

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
    let mut i = 0;
    while i < argv.len() {
        match argv[i].as_str() {
            "--json" => json = true,
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
    Ok(Some(Args { binary, output, json }))
}

fn usage() {
    eprintln!("usage: kuna unpack <binary> [-o OUT] [--json]");
}
