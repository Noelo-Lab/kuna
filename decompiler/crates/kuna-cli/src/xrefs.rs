//! `kuna xrefs` — the cross-reference query.
//!
//! ```text
//!   kuna xrefs <binary> --to <name|0xaddr>   [--json] [--kind k,k..] [--mode MODE]
//!   kuna xrefs <binary> --from <name|0xaddr> [--json] [--kind k,k..] [--mode MODE]
//!              [--option N V].. [--slice ARCH] [--target T] [--sleighpath D]
//! ```
//!
//! `--to` answers *what references this?* — every call site, branch, and data
//! reference that lands on the target. `--from` answers *what does this
//! reference?* — the target function's callees, its tail-jump targets, and the
//! data it touches. The two directions and the per-row `kind` field mirror the
//! DecLib CLI's `xref_to` / `xref_from`, which is the contract an RE agent
//! already knows.
//!
//! This is a **query, not an engine change**: it loads the binary once through
//! the same in-process seam `decompile-all` uses
//! ([`crate::decompile_all::load_program`], i.e. `bootstrap_from_object` →
//! `commit_pending_analysis`), then reads the references straight out of the
//! p-code the SLEIGH lifter already emits for every discovered function
//! ([`kuna_analysis::listing::xrefs`]). Nothing is committed into the engine, no
//! analysis pass runs that would not otherwise, and no emitted C changes.
//!
//! `--json` emits the machine-readable document; without it, one tab-separated
//! row per reference under a `#` header line.

use std::rc::Rc;

use kuna_analysis::listing::xrefs::{Xref, XrefIndex, XrefKind};
use kuna_base::address::Address;
use kuna_console::engine::{ConsoleProgram, EntryLookupError, EntrySelector};

use crate::decompile_all::{load_program, mode_options_for_binary, Args, DriverDefaults};
use crate::jsonfmt::{dumps_indent2, Json};

/// Which way the query runs.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    /// Everything that references the target.
    To,
    /// Everything the target references.
    From,
}

impl Direction {
    fn as_str(self) -> &'static str {
        match self {
            Direction::To => "to",
            Direction::From => "from",
        }
    }
}

/// The parsed command line.
struct XrefArgs {
    binary: String,
    /// The `--to` / `--from` operand: a symbol name or an address.
    spec: String,
    direction: Direction,
    json: bool,
    /// `--kind call,data`: restrict to these kinds; empty means every kind.
    kinds: Vec<XrefKind>,
    options: Vec<(String, String)>,
    mode: Option<String>,
    slice: Option<String>,
    target: Option<String>,
    sleighpath: Option<String>,
}

/// A resolved query target: an address, plus the best name the program has for it.
struct Target {
    addr: u64,
    name: Option<String>,
}

/// The operand resolved to an address, before the walk that names it.
///
/// The address is what the walk needs (it is pointed at it, `build_with_focus`);
/// the naming half runs afterwards because it reads the walk's own discovered
/// functions. `name` is a name the lookup itself established, `fallback` one to
/// use only if nothing else names the address.
struct TargetSpec {
    addr: u64,
    name: Option<String>,
    fallback: Option<String>,
}

/// `kuna xrefs` entry point.
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

/// Load, index, resolve, and render — the whole command in one pass.
fn query(args: &XrefArgs) -> Result<String, String> {
    // A reference query is not a decompile, so `--mode` is NOT resolved through
    // `auto` here: `auto` picks `aggressive` under 500 KiB, and `aggressive` is a
    // preset for the QUALITY of emitted C. Two of the passes it turns on cost a
    // whole extra decode of the program apiece and answer nothing this command
    // reads — the analysis-tier Listing walk (which `xrefs` re-walks itself) and
    // `operand_refs` (whose scalar markup `xrefs` recomputes from the p-code it
    // already has). On a 466 KB obfuscated i386 image they were 1.08 s and 0.58 s
    // of a 3.2 s answer that is byte-identical without them. `--mode aggressive`
    // still asks for the full bundle explicitly.
    let mode = Some(args.mode.as_deref().unwrap_or("reliable"));
    let options = mode_options_for_binary(mode, &args.binary, args.options.clone())?;
    // The query driver bundle: `xrefs` enumerates entries and walks them itself,
    // so it takes no Listing-tier injection at all — the discovery seeds the
    // injection existed to produce go straight into the reference walk below.
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
    };
    let prog = load_program(&load, DriverDefaults::Query)?;

    let bytes = kuna_analysis::loader::elf_shdr::read_image(&args.binary)
        .map_err(|e| format!("{}: {e}", args.binary))?;
    let file = object::File::parse(&*bytes)
        .map_err(|e| format!("could not parse {}: {e}", args.binary))?;

    let entries = prog.function_entries_canonical();
    let inventory: Vec<u64> = entries.iter().map(|e| e.addr.get_offset()).collect();
    let seeds = kuna_analysis::listing::xrefs::discovery_seeds(
        &file,
        &inventory,
        prog.arch().analysis_funcstart_patterns,
    );
    // The operand resolves to an address BEFORE the walk so the walk can be
    // pointed at it: a function whose only inbound edge is an indirect call
    // through a table is in no seed set, and a query about it must not answer
    // "no references" about the very address the caller named.
    let spec = target_address(&prog, &args.spec)?;
    let index = kuna_analysis::listing::xrefs::build_with_focus(
        &file,
        prog.arch(),
        prog.arch().translate(),
        &seeds,
        &[spec.addr],
    );

    let target = resolve_target(&prog, &index, spec);
    let rows = match args.direction {
        // `--to` answers for the callable, not the literal address: an import
        // reached through a veneer and an IAT/GOT slot is one thing under two
        // names, and which of the two a call site happens to reference is not a
        // distinction the caller asked about (`XrefIndex::refs_to_unified`).
        Direction::To => index.refs_to_unified(target.addr),
        Direction::From => {
            if index.is_function_entry(target.addr) || prog.find_entry_at(target.addr).is_some() {
                index.refs_from_function(target.addr)
            } else {
                index.refs_from_instruction(target.addr).iter().collect()
            }
        }
    };
    let rows: Vec<&Xref> = rows
        .into_iter()
        .filter(|r| args.kinds.is_empty() || args.kinds.contains(&r.kind))
        .collect();

    Ok(if args.json {
        format!("{}\n", dumps_indent2(&result_json(args, &prog, &index, &target, &rows)))
    } else {
        render_text(args, &prog, &index, &target, &rows)
    })
}

// --- target resolution -------------------------------------------------------

/// Resolve the `--to` / `--from` operand to an address plus a display name.
///
/// A `0x`-prefixed operand is always an address. Anything else is looked up as a
/// symbol FIRST — a function really can be called `abc`, and silently reading
/// that as `0xabc` would answer a question nobody asked — and only falls back to
/// a bare-hex reading when no symbol carries the name.
///
/// A name that identifies several entries is an ERROR naming all of them, not a
/// miss: falling through to the symbol table would answer for whichever one it
/// happens to hold first, which is the guess the selector model exists to refuse.
fn target_address(prog: &ConsoleProgram, spec: &str) -> Result<TargetSpec, String> {
    let spec = spec.trim();
    if let Some(body) = spec.strip_prefix("0x").or_else(|| spec.strip_prefix("0X")) {
        let addr = u64::from_str_radix(body, 16)
            .map_err(|_| format!("invalid address {spec:?}"))?;
        return Ok(TargetSpec { addr, name: None, fallback: None });
    }
    match prog.resolve_entry(&EntrySelector::Name(spec.to_string())) {
        Ok(entry) => {
            return Ok(TargetSpec {
                addr: entry.addr.get_offset(),
                name: Some(entry.name),
                fallback: None,
            })
        }
        Err(error @ EntryLookupError::Ambiguous { .. }) => return Err(error.to_string()),
        Err(_) => {}
    }
    if let Some(addr) = prog.lookup_symbol(spec) {
        return Ok(TargetSpec {
            addr: addr.get_offset(),
            name: None,
            fallback: Some(spec.to_string()),
        });
    }
    if let Some((name, addr, _)) = prog
        .global_data_symbols()
        .into_iter()
        .find(|(name, _, _)| name == spec)
    {
        return Ok(TargetSpec { addr, name: Some(name), fallback: None });
    }
    if let Ok(addr) = u64::from_str_radix(spec, 16) {
        return Ok(TargetSpec { addr, name: None, fallback: None });
    }
    Err(format!("no symbol named {spec:?} (and it is not an address)"))
}

/// Attach the display name to an address the spec already resolved: whatever the
/// lookup itself knew, else the program's best name for the address, else the
/// spec's own fallback (a symbol names its address even when nothing else does).
fn resolve_target(prog: &ConsoleProgram, index: &XrefIndex, spec: TargetSpec) -> Target {
    let TargetSpec { addr, name, fallback } = spec;
    Target { addr, name: name.or_else(|| name_at(prog, index, addr)).or(fallback) }
}

/// The program's best name for `vma`: the canonical function entry there, then a
/// function symbol, then a named global data object. `None` when nothing names it.
fn name_at(prog: &ConsoleProgram, index: &XrefIndex, vma: u64) -> Option<String> {
    prog.find_entry_at(vma)
        .map(|e| e.name)
        .or_else(|| prog.function_named_at(vma))
        // The walk discovers functions the engine's inventory does not carry (it
        // follows the call graph out of its seeds), and a row that names one must
        // still name it rather than answer `null`.
        .or_else(|| {
            index.is_function_entry(vma).then(|| {
                match prog.arch().manage().get_default_code_space() {
                    Some(space) => {
                        prog.arch().name_function(&Address::new(Rc::clone(space), vma))
                    }
                    None => format!("sub_{vma:x}"),
                }
            })
        })
        .or_else(|| {
            prog.global_data_symbols()
                .into_iter()
                .find(|(_, addr, _)| *addr == vma)
                .map(|(name, _, _)| name)
        })
}

/// The function `vma` lies in, as `(entry, name)`: the walk's own attribution
/// first (it knows which entry's descent reached the instruction), then the
/// engine's inventory for an address the walk never decoded.
fn owning_function(prog: &ConsoleProgram, index: &XrefIndex, vma: u64) -> Option<(u64, String)> {
    let entry = index
        .function_containing(vma)
        .or_else(|| prog.find_entry_at(vma).map(|e| e.addr.get_offset()))?;
    Some((entry, function_name(prog, index, entry)))
}

/// The display name for a function entry, falling back to the engine's own
/// placeholder (`sub_<addr>`) so a row is never nameless.
fn function_name(prog: &ConsoleProgram, index: &XrefIndex, entry: u64) -> String {
    name_at(prog, index, entry).unwrap_or_else(|| {
        match prog.arch().manage().get_default_code_space() {
            Some(space) => prog.arch().name_function(&Address::new(Rc::clone(space), entry)),
            None => format!("sub_{entry:x}"),
        }
    })
}

// --- rendering ---------------------------------------------------------------

/// An `{name, address, address_hex}` triple — the house address shape, used for
/// the query target and for each row's owning function.
fn function_json(name: &str, addr: u64) -> Json {
    Json::Object(vec![
        ("name".into(), Json::Str(name.to_string())),
        ("address".into(), Json::Number(addr.to_string())),
        ("address_hex".into(), Json::Str(format!("0x{addr:x}"))),
    ])
}

fn optional_function_json(f: Option<(u64, String)>) -> Json {
    match f {
        Some((addr, name)) => function_json(&name, addr),
        None => Json::Null,
    }
}

/// Build the `xrefs --json` document.
///
/// Each row carries BOTH ends of the edge explicitly (`from_address` /
/// `to_address` and their functions) plus `address`, which is the end the query
/// did not already name: the referencing site for `--to`, the referenced
/// location for `--from`. A consumer can read either shape without knowing the
/// direction it asked for.
fn result_json(
    args: &XrefArgs,
    prog: &ConsoleProgram,
    index: &XrefIndex,
    target: &Target,
    rows: &[&Xref],
) -> Json {
    let xrefs = Json::Array(
        rows.iter()
            .map(|r| {
                let other = match args.direction {
                    Direction::To => r.from,
                    Direction::From => r.to,
                };
                Json::Object(vec![
                    ("address".into(), Json::Number(other.to_string())),
                    ("address_hex".into(), Json::Str(format!("0x{other:x}"))),
                    ("kind".into(), Json::Str(r.kind.as_str().to_string())),
                    ("from_address".into(), Json::Number(r.from.to_string())),
                    ("from_address_hex".into(), Json::Str(format!("0x{:x}", r.from))),
                    ("to_address".into(), Json::Number(r.to.to_string())),
                    ("to_address_hex".into(), Json::Str(format!("0x{:x}", r.to))),
                    (
                        "from_function".into(),
                        optional_function_json(owning_function(prog, index, r.from)),
                    ),
                    (
                        "to_function".into(),
                        optional_function_json(owning_function(prog, index, r.to)),
                    ),
                    ("instruction".into(), Json::Str(r.instruction.clone())),
                ])
            })
            .collect(),
    );
    Json::Object(vec![
        ("binary".into(), Json::Str(args.binary.clone())),
        (
            "target".into(),
            Json::Object(vec![
                (
                    "name".into(),
                    target.name.clone().map(Json::Str).unwrap_or(Json::Null),
                ),
                ("address".into(), Json::Number(target.addr.to_string())),
                ("address_hex".into(), Json::Str(format!("0x{:x}", target.addr))),
                (
                    "aliases".into(),
                    Json::Array(
                        aliases(index, target)
                            .into_iter()
                            .map(|a| {
                                function_json(
                                    &name_at(prog, index, a).unwrap_or_else(|| format!("0x{a:x}")),
                                    a,
                                )
                            })
                            .collect(),
                    ),
                ),
            ]),
        ),
        ("direction".into(), Json::Str(args.direction.as_str().to_string())),
        ("count".into(), Json::Number(rows.len().to_string())),
        ("xrefs".into(), xrefs),
    ])
}

/// The human surface: a `#` header naming the query, then one tab-separated row
/// per reference.  `--to` rows lead with the referencing site and name the
/// function it sits in; `--from` rows lead with the referenced location and name
/// the site the reference was made from.
fn render_text(
    args: &XrefArgs,
    prog: &ConsoleProgram,
    index: &XrefIndex,
    target: &Target,
    rows: &[&Xref],
) -> String {
    use std::fmt::Write as _;
    let mut out = String::new();
    let label = match &target.name {
        Some(name) => format!("{name} @ 0x{:x}", target.addr),
        None => format!("0x{:x}", target.addr),
    };
    let plural = if rows.len() == 1 { "reference" } else { "references" };
    let _ = writeln!(
        out,
        "# {} {plural} {} {label}",
        rows.len(),
        args.direction.as_str()
    );
    // Say which other address the answer was taken over, so a count that does not
    // match a raw disassembly grep of the target explains itself on the spot.
    for a in aliases(index, target) {
        let _ = writeln!(
            out,
            "# same import at 0x{a:x} ({}) - a forwarding veneer and the pointer slot it jumps through",
            name_at(prog, index, a).unwrap_or_else(|| "-".into())
        );
    }
    for r in rows {
        match args.direction {
            Direction::To => {
                let _ = writeln!(
                    out,
                    "0x{:x}\t{}\t{}\t{}",
                    r.from,
                    r.kind.as_str(),
                    site_label(prog, index, r.from),
                    r.instruction
                );
            }
            Direction::From => {
                let _ = writeln!(
                    out,
                    "0x{:x}\t{}\t{}\t@0x{:x}\t{}",
                    r.to,
                    r.kind.as_str(),
                    name_at(prog, index, r.to).unwrap_or_else(|| "-".into()),
                    r.from,
                    r.instruction
                );
            }
        }
    }
    out
}

/// The addresses other than the target's own that name the same callable: the
/// pointer slot a forwarding veneer jumps through, or the veneers that jump
/// through a slot. Empty for everything else, which is almost everything.
fn aliases(index: &XrefIndex, target: &Target) -> Vec<u64> {
    index.alias_class(target.addr).into_iter().filter(|&a| a != target.addr).collect()
}

/// `name+0xoff` for an address inside a known function; the bare address when
/// nothing owns it.
fn site_label(prog: &ConsoleProgram, index: &XrefIndex, vma: u64) -> String {
    match owning_function(prog, index, vma) {
        Some((entry, name)) if entry == vma => name,
        Some((entry, name)) if entry < vma => format!("{name}+0x{:x}", vma - entry),
        Some((entry, name)) => format!("{name}-0x{:x}", entry - vma),
        None => format!("0x{vma:x}"),
    }
}

// --- argument parsing --------------------------------------------------------

fn parse_args(argv: &[String]) -> Result<XrefArgs, String> {
    let mut binary: Option<String> = None;
    let mut spec: Option<(Direction, String)> = None;
    let mut json = false;
    let mut kinds: Vec<XrefKind> = Vec::new();
    let mut options: Vec<(String, String)> = Vec::new();
    let mut mode: Option<String> = None;
    let mut slice: Option<String> = None;
    let mut target: Option<String> = None;
    let mut sleighpath: Option<String> = None;

    let mut i = 0;
    while i < argv.len() {
        let a = argv[i].as_str();
        match a {
            "--json" => json = true,
            "--to" | "--from" => {
                let dir = if a == "--to" { Direction::To } else { Direction::From };
                let v = take(argv, &mut i, a)?;
                if spec.is_some() {
                    return Err("--to and --from are mutually exclusive".into());
                }
                spec = Some((dir, v));
            }
            "--kind" => {
                let v = take(argv, &mut i, "--kind")?;
                for k in v.split(',').map(str::trim).filter(|s| !s.is_empty()) {
                    kinds.push(parse_kind(k)?);
                }
            }
            "--option" => {
                if i + 2 >= argv.len() {
                    return Err("--option requires NAME VALUE".into());
                }
                options.push((argv[i + 1].clone(), argv[i + 2].clone()));
                i += 2;
            }
            "--mode" => mode = Some(take(argv, &mut i, "--mode")?),
            "--slice" => slice = Some(take(argv, &mut i, "--slice")?),
            "--target" => target = Some(take(argv, &mut i, "--target")?),
            "--sleighpath" => sleighpath = Some(take(argv, &mut i, "--sleighpath")?),
            "-h" | "--help" => {
                usage();
                std::process::exit(0);
            }
            s if s.starts_with("--") => return Err(format!("unknown option {s}")),
            _ => {
                if binary.is_none() {
                    binary = Some(a.to_string());
                } else {
                    return Err(format!("unexpected argument {a:?}"));
                }
            }
        }
        i += 1;
    }

    let binary = binary.ok_or("xrefs requires <binary>")?;
    let (direction, spec) = spec.ok_or("xrefs requires --to <target> or --from <target>")?;
    Ok(XrefArgs {
        binary,
        spec,
        direction,
        json,
        kinds,
        options,
        mode,
        slice,
        target,
        sleighpath,
    })
}

fn parse_kind(k: &str) -> Result<XrefKind, String> {
    match k.to_ascii_lowercase().as_str() {
        "call" => Ok(XrefKind::Call),
        "jump" => Ok(XrefKind::Jump),
        "data" => Ok(XrefKind::Data),
        "read" => Ok(XrefKind::Read),
        "write" => Ok(XrefKind::Write),
        other => Err(format!("unknown --kind {other:?} (call, jump, data, read, write)")),
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
        "usage: kuna xrefs <binary> (--to <name|0xaddr> | --from <name|0xaddr>) [--json] \\\n\
         \x20                  [--kind call,jump,data,read,write] [--mode auto|reliable|aggressive|fast] \\\n\
         \x20                  [--option N V].. [--slice ARCH] [--target T] [--sleighpath D]\n\
         \n\
         --to    everything that references the target (call sites, branches, data references).\n\
         \x20       An import is one callable under two addresses -- a forwarding veneer and the\n\
         \x20       IAT/GOT slot it jumps through -- and both answer the same; target.aliases\n\
         \x20       names the other one.\n\
         --from  everything the target references (its callees and the data it touches)\n\
         \n\
         --json emits {{binary,target:{{name,address,address_hex,aliases}},direction,count,\n\
         xrefs:[{{address,address_hex,kind,from_function,to_function,instruction,..}}]}};\n\
         without it, one tab-separated row each."
    );
}
