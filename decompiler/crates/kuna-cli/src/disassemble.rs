//! `kuna disassemble` — the instruction listing.
//!
//! ```text
//!   kuna disassemble <binary> <name|0xaddr|0xstart-0xend> [--addr] [--count N]
//!                    [--bytes N] [--json] [--mode MODE] [--option N V]..
//!                    [--slice ARCH] [--target T] [--sleighpath D]
//! ```
//!
//! The floor an RE agent falls back to when the ceiling gives way. Three
//! independent testers in the RE loop (`docs/re-pipeline.md`) reached for
//! `kuna disassemble` **after** decompilation had already failed them — a
//! function with no recovered body, a `switch(0)` dispatcher, an indirect call
//! through a decrypted stack buffer — and got `unknown subcommand`. Every one of
//! them then left kuna for `objdump -d`, which is also why the bundled
//! `outlining` skill tells agents to shell out for addresses.
//!
//! ## Expose, do not reinvent
//!
//! The engine has disassembled every instruction it ever lifted; nothing here
//! decodes anything itself. Each row comes from
//! [`ConsoleProgram::disassemble_at_into`] — the same
//! `Translate::print_assembly` seam the console's own `disassemble` command
//! (`IfcPrintdisasm`, `kuna-console/src/ifacedecomp.rs`) and the
//! `decompile-project` `.asm` export already print, so a mnemonic that differs
//! from theirs is a lifter difference, not a formatting one. The bytes are the
//! load image's own (`ConsoleProgram::read_bytes_into`).
//!
//! What the console command could not supply is the *range*. Its no-argument
//! form asks the loaded `Funcdata` for its size, which is `0` until a decompile
//! has run, so `load function main; disassemble` prints a header and no
//! instructions; its two-address form needs both ends spelled out, and the CLI
//! never reached it at all. This command resolves the extent instead: a name (or
//! a discovered entry address) lists that function's inventory extent
//! (`ConsoleProgram::function_extent_at`, the same clip `kuna functions` reports
//! as `size`), and a raw address that no function owns lists a fixed window —
//! which is the case that matters for a data blob or a region decompilation
//! refused to enter.
//!
//! ## Not an engine change
//!
//! Like `kuna xrefs`, this is a **query**: it loads the binary once through the
//! in-process seam `decompile-all` uses ([`crate::decompile_all::load_program`],
//! i.e. `bootstrap_from_object` → `commit_pending_analysis`) with the inventory
//! driver defaults, decodes, and prints. Nothing is committed into the engine,
//! no decompilation runs, and no emitted C changes.
//!
//! Bytes that do not decode are not skipped and not guessed at: they are listed
//! as `.byte 0x<nn>` rows, one byte at a time, so a listing that walked into
//! data says so in place rather than silently stopping.

use kuna_console::engine::ConsoleProgram;

use crate::decompile::looks_like_addr;
use crate::decompile_all::{load_program, mode_options_for_binary, Args, DriverDefaults};
use crate::jsonfmt::{dumps_indent2, Json};

/// How much to list from an address that lies inside no known function extent:
/// an unmapped-by-the-inventory blob, a decrypted payload dumped to a file, a
/// region the decompiler refused. Enough to see what is there, small enough to
/// read; `--count`/`--bytes`/an explicit range override it.
const DEFAULT_WINDOW_BYTES: u64 = 64;

/// Safety stop for a listing whose length was DERIVED from the inventory rather
/// than asked for.
///
/// The extent is an upper bound clipped at the next discovered entry or the end
/// of the containing CODE section (`kuna-console/src/funcextent.rs`), so where
/// discovery is thin the "function" runs to the end of `.text`: `main` in the
/// unpacked `Sh4ll6` crackme clips to 19,106 instructions, about 1.2 MB of
/// listing for what the caller asked to see one function of. Truncating there is
/// the useful answer — the header and the JSON `truncated` flag say so, and
/// `end` is the address to resume from. An explicit `--count`, `--bytes` or
/// address range is honored verbatim, however long.
const DERIVED_INSTRUCTION_CAP: usize = 1024;

/// The mnemonic given to a byte the translator would not decode.
const BAD_BYTE_MNEMONIC: &str = ".byte";

/// The parsed command line.
pub(crate) struct DisArgs {
    pub(crate) binary: String,
    /// The target operand: a symbol name, an address, or a `start-end` range.
    pub(crate) spec: String,
    /// `--addr`: read `spec` as an address even when it is bare hex.
    pub(crate) by_address: bool,
    /// `--count N`: stop after N instructions.
    pub(crate) count: Option<usize>,
    /// `--bytes N`: stop after N bytes.
    pub(crate) bytes: Option<u64>,
    pub(crate) json: bool,
    pub(crate) options: Vec<(String, String)>,
    /// `--define-function <start[-end][=name] | @file>` (repeatable): declared
    /// function boundaries, applied at load so `disassemble <name>` resolves a
    /// name the image never carried and the walk stops at the declared end.
    pub(crate) func_decls: Vec<crate::funcdecl::FuncDecl>,
    pub(crate) mode: Option<String>,
    pub(crate) slice: Option<String>,
    pub(crate) target: Option<String>,
    pub(crate) sleighpath: Option<String>,
}

/// Where the walk starts, where it stops, and what the program calls the start.
struct Region {
    start: u64,
    /// Exclusive stop. `None` when only `--count` bounds the walk.
    end: Option<u64>,
    name: Option<String>,
    /// Was the stop derived from the inventory rather than asked for?
    derived: bool,
}

/// One listed instruction.
struct Row {
    addr: u64,
    size: u64,
    bytes: Vec<u8>,
    mnemonic: String,
    operands: String,
}

impl Row {
    /// `MNEMONIC operands` with ONE space — the same instruction spelling
    /// `kuna xrefs` puts in its `instruction` field, so the two surfaces are
    /// greppable with one pattern (`CALL 0x140002490`). The console's own
    /// listing pads the mnemonic to a fixed column instead; that padding is a
    /// display choice, not part of the instruction.
    fn text(&self) -> String {
        if self.operands.is_empty() {
            self.mnemonic.clone()
        } else {
            format!("{} {}", self.mnemonic, self.operands)
        }
    }

    fn hex(&self) -> String {
        let mut s = String::with_capacity(self.bytes.len() * 2);
        for b in &self.bytes {
            s.push_str(&format!("{b:02x}"));
        }
        s
    }
}

/// `kuna disassemble` entry point.
pub fn run(argv: &[String]) -> i32 {
    let args = match parse_args(argv) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("error: {e}");
            usage();
            return 2;
        }
    };
    match render(&args) {
        Ok(text) => crate::output::emit_with_status(&text, 0),
        Err(e) => {
            eprintln!("error: {e}");
            1
        }
    }
}

/// Load, resolve, walk, and render — the whole command, minus the stdout
/// boundary, so the listing is testable without a subprocess.
pub(crate) fn render(args: &DisArgs) -> Result<String, String> {
    let options = mode_options_for_binary(args.mode.as_deref(), &args.binary, args.options.clone())?;
    // The inventory bundle: this surface enumerates entries to resolve a name
    // and to bound a function, and decompiles nothing.
    let load = Args {
        binary: args.binary.clone(),
        json: args.json,
        names: None,
        addrs: Vec::new(),
        no_vars: true,
        max_fn_seconds: 0,
        options,
        func_decls: args.func_decls.clone(),
        slice: args.slice.clone(),
        target: args.target.clone(),
        sleighpath: args.sleighpath.clone(),
    };
    let prog = load_program(&load, DriverDefaults::Inventory)?;

    let region = resolve_region(&prog, args)?;
    // A packed image is the common way to hold an address that is real in the
    // program and absent from the file, so the failure names the move that fixes
    // it rather than leaving the caller to guess.
    if !prog.vma_bytes_mapped(region.start) {
        return Err(format!(
            "no bytes mapped at 0x{:x} in {}: no loaded segment covers it \
             (a packed image maps none of its original addresses -- `kuna unpack` first)",
            region.start, args.binary
        ));
    }
    let (rows, truncated) = walk(&prog, &region, args.count);
    Ok(if args.json {
        format!("{}\n", dumps_indent2(&result_json(args, &region, &rows, truncated)))
    } else {
        render_text(&region, &rows, truncated)
    })
}

// --- the walk ----------------------------------------------------------------

/// Decode forward from `region.start` until a stop is reached: the region's end,
/// the instruction budget, the derived-length cap, or memory that will not read.
///
/// An address the translator rejects is listed as a single `.byte` row rather
/// than ending the listing, because the common reason for one is a listing that
/// ran into inline data and there is usually code again after it.
fn walk(prog: &ConsoleProgram, region: &Region, count: Option<usize>) -> (Vec<Row>, bool) {
    let cap = if region.derived { Some(DERIVED_INSTRUCTION_CAP) } else { None };
    let mut rows: Vec<Row> = Vec::new();
    let mut truncated = false;
    let mut addr = region.start;
    let (mut mnem, mut body, mut raw) = (String::new(), String::new(), Vec::new());
    loop {
        if count.is_some_and(|n| rows.len() >= n) {
            break;
        }
        if region.end.is_some_and(|end| addr >= end) {
            break;
        }
        if cap.is_some_and(|c| rows.len() >= c) {
            truncated = true;
            break;
        }
        let decoded = prog.disassemble_at_into(addr, &mut mnem, &mut body).ok().filter(|&n| n > 0);
        // The bytes are read back separately, so a row is only reported as an
        // instruction when BOTH the decode and the read succeeded — a row can
        // never claim a length it cannot show the bytes for.
        match decoded {
            Some(len) if prog.read_bytes_into(addr, len as usize, &mut raw) => {
                rows.push(Row {
                    addr,
                    size: len as u64,
                    bytes: raw.clone(),
                    mnemonic: mnem.clone(),
                    operands: body.clone(),
                });
                addr = addr.saturating_add(len as u64);
            }
            _ => {
                if !prog.read_bytes_into(addr, 1, &mut raw) {
                    break;
                }
                rows.push(Row {
                    addr,
                    size: 1,
                    bytes: raw.clone(),
                    mnemonic: BAD_BYTE_MNEMONIC.to_string(),
                    operands: format!("0x{:02x}", raw[0]),
                });
                addr = addr.saturating_add(1);
            }
        }
    }
    (rows, truncated)
}

// --- target resolution -------------------------------------------------------

/// Resolve the target operand into a start, a stop, and a display name.
///
/// A `0x`-prefixed operand (or any operand under `--addr`) is an address; a
/// `start-end` / `start..end` pair is an explicit range. Anything else is looked
/// up as a symbol FIRST — a function really can be called `abc`, and reading
/// that as `0xabc` would list somewhere nobody asked about — and only falls back
/// to a bare-hex reading when no symbol carries the name (the rule
/// [`crate::xrefs`] resolves targets by).
///
/// The stop is the tightest bound given: an explicit range end and `--bytes`
/// clip each other, `--count` alone leaves no byte stop at all, and with none of
/// them the stop is derived — the function's inventory extent, or
/// [`DEFAULT_WINDOW_BYTES`] for an address no CODE-section function owns.
fn resolve_region(prog: &ConsoleProgram, args: &DisArgs) -> Result<Region, String> {
    let spec = args.spec.trim();
    let addressy = args.by_address || looks_like_addr(spec);

    if let Some((lo, hi)) = split_range(spec) {
        let (start, end) = (parse_addr(lo)?, parse_addr(hi)?);
        if end <= start {
            return Err(format!("empty range {spec:?}: the end must be above the start"));
        }
        // Both bound the walk, so the tighter one wins — the same "first limit
        // reached" rule `--count` follows.
        let end = args.bytes.map_or(end, |n| end.min(start.saturating_add(n)));
        return Ok(Region { start, end: Some(end), name: name_at(prog, start), derived: false });
    }

    let (start, name) = if addressy {
        let addr = parse_addr(spec)?;
        // An ARM caller legitimately holds an odd `entry|1` Thumb address; the
        // inventory folds the mode bit, so resolve through it when it knows the
        // entry and list where the instructions actually are.
        match prog.find_entry_at(addr) {
            Some(e) => (e.addr.get_offset(), Some(e.name)),
            None => (addr, name_at(prog, addr)),
        }
    } else if let Some(e) = prog.find_entry_by_name(spec) {
        (e.addr.get_offset(), Some(e.name))
    } else if let Some(a) = prog.lookup_symbol(spec) {
        let addr = a.get_offset();
        (addr, name_at(prog, addr).or_else(|| Some(spec.to_string())))
    } else if let Some((n, addr, _)) =
        prog.global_data_symbols().into_iter().find(|(n, _, _)| n == spec)
    {
        (addr, Some(n))
    } else if let Ok(addr) = u64::from_str_radix(spec, 16) {
        (addr, name_at(prog, addr))
    } else {
        return Err(format!(
            "no symbol named {spec:?} in {} (and it is not an address; pass --addr \
             for a bare hex address, or `kuna unpack` if the image is packed)",
            args.binary
        ));
    };

    Ok(match (args.bytes, args.count) {
        (Some(n), _) => {
            Region { start, end: Some(start.saturating_add(n)), name, derived: false }
        }
        (None, Some(_)) => Region { start, end: None, name, derived: false },
        (None, None) => {
            let extent = prog.function_extent_at(start);
            let span = if extent > 0 { extent } else { DEFAULT_WINDOW_BYTES };
            Region { start, end: Some(start.saturating_add(span)), name, derived: true }
        }
    })
}

/// Split an explicit `start-end` / `start..end` range operand.
///
/// Both halves must read as addresses, and a single `-` additionally needs a
/// `0x`-prefixed left half, so a symbol that happens to contain a dash stays a
/// symbol and reaches the lookup below.
fn split_range(spec: &str) -> Option<(&str, &str)> {
    let both_parse =
        |lo: &str, hi: &str| parse_addr(lo).is_ok() && parse_addr(hi).is_ok();
    if let Some((lo, hi)) = spec.split_once("..") {
        return both_parse(lo, hi).then_some((lo, hi));
    }
    let (lo, hi) = spec.split_once('-')?;
    (looks_like_addr(lo) && both_parse(lo, hi)).then_some((lo, hi))
}

fn parse_addr(token: &str) -> Result<u64, String> {
    let t = token.trim();
    let body = t.strip_prefix("0x").or_else(|| t.strip_prefix("0X")).unwrap_or(t);
    u64::from_str_radix(body, 16).map_err(|_| format!("invalid address {token:?}"))
}

/// The program's best name for `vma`: the canonical function entry there, then a
/// function symbol, then a named global. `None` when nothing names it.
fn name_at(prog: &ConsoleProgram, vma: u64) -> Option<String> {
    prog.find_entry_at(vma)
        .map(|e| e.name)
        .or_else(|| prog.function_named_at(vma))
        .or_else(|| {
            prog.global_data_symbols()
                .into_iter()
                .find(|(_, addr, _)| *addr == vma)
                .map(|(name, _, _)| name)
        })
}

// --- rendering ---------------------------------------------------------------

/// Build the `disassemble --json` document.
///
/// `end` is one past the last instruction actually listed, not the requested
/// stop, so a caller resuming a truncated listing has its next start in hand.
fn result_json(args: &DisArgs, region: &Region, rows: &[Row], truncated: bool) -> Json {
    let end = rows.last().map_or(region.start, |r| r.addr + r.size);
    let instructions = Json::Array(
        rows.iter()
            .map(|r| {
                Json::Object(vec![
                    ("address".into(), Json::Number(r.addr.to_string())),
                    ("address_hex".into(), Json::Str(format!("0x{:x}", r.addr))),
                    ("size".into(), Json::Number(r.size.to_string())),
                    ("bytes".into(), Json::Str(r.hex())),
                    ("mnemonic".into(), Json::Str(r.mnemonic.clone())),
                    ("operands".into(), Json::Str(r.operands.clone())),
                    ("text".into(), Json::Str(r.text())),
                ])
            })
            .collect(),
    );
    Json::Object(vec![
        ("binary".into(), Json::Str(args.binary.clone())),
        (
            "target".into(),
            Json::Object(vec![
                ("name".into(), region.name.clone().map(Json::Str).unwrap_or(Json::Null)),
                ("address".into(), Json::Number(region.start.to_string())),
                ("address_hex".into(), Json::Str(format!("0x{:x}", region.start))),
            ]),
        ),
        ("start".into(), Json::Number(region.start.to_string())),
        ("start_hex".into(), Json::Str(format!("0x{:x}", region.start))),
        ("end".into(), Json::Number(end.to_string())),
        ("end_hex".into(), Json::Str(format!("0x{end:x}"))),
        ("count".into(), Json::Number(rows.len().to_string())),
        ("bytes".into(), Json::Number((end - region.start).to_string())),
        ("truncated".into(), Json::Bool(truncated)),
        ("instructions".into(), instructions),
    ])
}

/// The human surface: a `#` header naming what was listed, then one column-aligned
/// row per instruction — address, raw bytes, text. The columns are padded, not
/// tab-separated, because a disassembly listing is read down its mnemonics; the
/// instruction text itself still carries exactly one space, so `grep 'CALL 0x'`
/// matches here and in `--json` alike.
fn render_text(region: &Region, rows: &[Row], truncated: bool) -> String {
    use std::fmt::Write as _;
    let end = rows.last().map_or(region.start, |r| r.addr + r.size);
    let mut out = String::new();
    let label = match &region.name {
        Some(name) => format!("{name} @ 0x{:x}", region.start),
        None => format!("0x{:x}", region.start),
    };
    let plural = if rows.len() == 1 { "instruction" } else { "instructions" };
    let _ = writeln!(
        out,
        "# {} {plural} at {label} (0x{:x}..0x{end:x}, {} bytes){}",
        rows.len(),
        region.start,
        end - region.start,
        if truncated {
            " [truncated: --count N, --bytes N or a 0xstart-0xend range lists more]"
        } else {
            ""
        }
    );
    for r in rows {
        let line = format!("{:<14}{:<22}{}", format!("0x{:x}", r.addr), r.hex(), r.text());
        let _ = writeln!(out, "{}", line.trim_end());
    }
    out
}

// --- argument parsing --------------------------------------------------------

pub(crate) fn parse_args(argv: &[String]) -> Result<DisArgs, String> {
    let mut positional: Vec<String> = Vec::new();
    let mut by_address = false;
    let mut count: Option<usize> = None;
    let mut bytes: Option<u64> = None;
    let mut json = false;
    let mut options: Vec<(String, String)> = Vec::new();
    let mut func_decls: Vec<crate::funcdecl::FuncDecl> = Vec::new();
    let mut mode: Option<String> = None;
    let mut slice: Option<String> = None;
    let mut target: Option<String> = None;
    let mut sleighpath: Option<String> = None;

    let mut i = 0;
    while i < argv.len() {
        let a = argv[i].as_str();
        match a {
            "--addr" => by_address = true,
            "--json" => json = true,
            "--count" => count = Some(parse_positive(&take(argv, &mut i, a)?, a)? as usize),
            "--bytes" => bytes = Some(parse_positive(&take(argv, &mut i, a)?, a)?),
            "--option" => {
                if i + 2 >= argv.len() {
                    return Err("--option requires NAME VALUE".into());
                }
                options.push((argv[i + 1].clone(), argv[i + 2].clone()));
                i += 2;
            }
            "--mode" => mode = Some(take(argv, &mut i, "--mode")?),
            "--define-function" => {
                let v = take(argv, &mut i, "--define-function")?;
                func_decls.extend(crate::funcdecl::parse_flag(&v)?);
            }
            "--slice" => slice = Some(take(argv, &mut i, "--slice")?),
            "--target" => target = Some(take(argv, &mut i, "--target")?),
            "--sleighpath" => sleighpath = Some(take(argv, &mut i, "--sleighpath")?),
            "-h" | "--help" => {
                usage();
                std::process::exit(0);
            }
            s if s.starts_with("--") => return Err(format!("unknown option {s}")),
            _ => positional.push(a.to_string()),
        }
        i += 1;
    }

    if positional.len() > 2 {
        return Err(format!("unexpected argument {:?}", positional[2]));
    }
    let mut it = positional.into_iter();
    let binary = it.next().ok_or("disassemble requires <binary>")?;
    let spec = it.next().ok_or("disassemble requires <name|0xaddr|0xstart-0xend>")?;
    Ok(DisArgs {
        binary,
        spec,
        by_address,
        count,
        bytes,
        json,
        options,
        func_decls,
        mode,
        slice,
        target,
        sleighpath,
    })
}

fn parse_positive(value: &str, flag: &str) -> Result<u64, String> {
    match value.parse::<u64>() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(format!("{flag} takes a positive integer, got {value:?}")),
    }
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
        "usage: kuna disassemble <binary> <name|0xaddr|0xstart-0xend> [--addr] [--count N] \\\n\
         \x20                    [--bytes N] [--json] [--mode auto|reliable|aggressive|fast] \\\n\
         \x20                    [--define-function S[-E][=N]|@FILE].. \\\n\
         \x20                    [--option N V].. [--slice ARCH] [--target T] [--sleighpath D]\n\
         \n\
         The target is a function name, an address (--addr for a bare hex one), or an\n\
         explicit range (0x1000-0x1040 / 0x1000..0x1040) for bytes no function owns.\n\
         A named function lists its whole extent; a raw address lists 64 bytes unless\n\
         --count / --bytes / a range says otherwise.\n\
         \n\
         --define-function <start[-end][=name] | @file> (repeatable) declares a\n\
         boundary first, so a name the image never carried becomes a valid target.\n\
         \n\
         --json emits {{binary,target,start,end,count,bytes,truncated,instructions:\n\
         [{{address,address_hex,size,bytes,mnemonic,operands,text}}]}}; without it, a\n\
         header line and one address / bytes / text row per instruction."
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn row(addr: u64, bytes: &[u8], mnemonic: &str, operands: &str) -> Row {
        Row {
            addr,
            size: bytes.len() as u64,
            bytes: bytes.to_vec(),
            mnemonic: mnemonic.into(),
            operands: operands.into(),
        }
    }

    #[test]
    fn instruction_text_carries_exactly_one_space() {
        assert_eq!(row(0, &[0x55], "PUSH", "RBP").text(), "PUSH RBP");
        assert_eq!(row(0, &[0xc3], "RET", "").text(), "RET");
    }

    #[test]
    fn bytes_render_as_contiguous_lowercase_hex() {
        assert_eq!(row(0, &[0x48, 0x89, 0xe5], "MOV", "RBP,RSP").hex(), "4889e5");
    }

    #[test]
    fn a_range_operand_needs_both_ends() {
        assert_eq!(split_range("0x1000-0x1040"), Some(("0x1000", "0x1040")));
        assert_eq!(split_range("0x1000..0x1040"), Some(("0x1000", "0x1040")));
        // A dash inside a name is a name, not a range: only a 0x-prefixed left
        // half opens one.
        assert_eq!(split_range("foo-bar"), None);
        assert_eq!(split_range("main"), None);
        assert_eq!(split_range("0x1000-"), None);
        // Both halves must read as addresses, so a dotted symbol stays a symbol.
        assert_eq!(split_range("std::vector..end"), None);
    }

    #[test]
    fn addresses_parse_with_or_without_the_prefix() {
        assert_eq!(parse_addr("0x401000"), Ok(0x401000));
        assert_eq!(parse_addr("401000"), Ok(0x401000));
        assert!(parse_addr("main").is_err());
    }

    #[test]
    fn the_target_and_the_binary_are_the_only_positionals() {
        let argv: Vec<String> =
            ["a.out", "main", "--json", "--count", "4"].iter().map(|s| s.to_string()).collect();
        let args = parse_args(&argv).expect("a well-formed command line");
        assert_eq!(args.binary, "a.out");
        assert_eq!(args.spec, "main");
        assert_eq!(args.count, Some(4));
        assert!(args.json && !args.by_address);

        assert!(parse_args(&["a.out".into()]).is_err(), "a missing target is a usage error");
        assert!(
            parse_args(&["a.out".into(), "main".into(), "extra".into()]).is_err(),
            "a third positional is a usage error"
        );
        assert!(
            parse_args(&["a.out".into(), "main".into(), "--count".into(), "0".into()]).is_err(),
            "--count 0 lists nothing and is a usage error"
        );
    }

    #[test]
    fn the_header_names_the_target_and_the_extent() {
        let region = Region { start: 0x1000, end: Some(0x1004), name: Some("main".into()), derived: true };
        let rows = vec![row(0x1000, &[0x55], "PUSH", "RBP"), row(0x1001, &[0x48, 0x89, 0xe5], "MOV", "RBP,RSP")];
        let text = render_text(&region, &rows, false);
        let mut lines = text.lines();
        assert_eq!(
            lines.next().unwrap(),
            "# 2 instructions at main @ 0x1000 (0x1000..0x1004, 4 bytes)"
        );
        assert_eq!(lines.next().unwrap(), "0x1000        55                    PUSH RBP");
        assert!(render_text(&region, &rows, true).contains("[truncated:"));
    }
}
