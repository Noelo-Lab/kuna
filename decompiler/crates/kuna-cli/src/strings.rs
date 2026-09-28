//! `kuna strings` — the string inventory.
//!
//! ```text
//!   kuna strings <binary> [--json] [--min-length N] [--filter REGEX]
//!                         [--encoding ascii|utf8|utf16|all] [--termination nul|any]
//!                         [--section NAME] [--no-xrefs]
//!                         [--mode MODE] [--option N V].. [--isa auto|arm|thumb] [--slice ARCH] [--target T]
//!                         [--sleighpath D]
//! ```
//!
//! This is the CLI face of the analyzer tier's **existing** string detection: the
//! rows come from [`kuna_analysis::strings::kuna_stringinv`], whose ASCII half is
//! the very `StringLiteralPass` scan the engine already runs at load to plant the
//! `char[N]` literals `kuna decompile` prints. Nothing is re-detected here and
//! nothing is committed, so no emitted C changes.
//!
//! What makes it worth running instead of `strings(1)` is the last two columns.
//! `strings(1)` answers "what text is in this file"; the question an analyst
//! actually has is "**which function uses this string**", and kuna can answer it
//! because it already has the reference edges — the same
//! [`kuna_analysis::listing::xrefs`] index behind `kuna xrefs`. Every row carries
//! how many references land anywhere in the literal's extent and the functions
//! they come from, so the triage hop (find the prompt → open its checker) is one
//! command instead of `strings | xxd | grep`.
//!
//! `--encoding utf16` is the second thing `strings(1)` needs a second invocation
//! for and the decompiler needs outright: a UTF-16LE literal read at 1-byte width
//! ends at the NUL after its first character, which is why `LoadLibraryW` renders
//! with a one-character argument.

//! `--encoding utf8` is the other reading of the 1-byte width. The matcher's
//! recognizer is ASCII, so any byte `>= 0x80` ends a run and a literal that opens
//! with a non-ASCII character is reported starting after its last multi-byte
//! sequence — a `_φ( °-°)/ so what was the magical keycombination? ` prompt at
//! 0x2000 came back as `)/ so what was the magical keycombination? ` at 0x200c,
//! with zero references and no owning function, while `kuna xrefs --to 0x2000`
//! answered one. Only the reported address was wrong; decoding the sequences puts
//! the row back on the address the image refers to and the existing reference
//! walk fills the last two columns. It is a superset of the ASCII reading rather
//! than a rival to it, so `all` takes it in place of that reading and a row with
//! no multi-byte content is still reported as `ascii`.
//!
//! `--termination` is the third. The markup pass takes only NUL-ended runs
//! because it plants a `char[N]`, and reporting only those made the inventory
//! answer **zero** on a length-prefixed name table
//! (`\x0cout.js\x06std\x12_0x8ec6b3`, the shape a bundled JS or bytecode payload
//! carries) that `strings(1)` reads in full. The default is therefore `any`,
//! `strings(1)`'s own rule over kuna's address set; `nul` restores the
//! pass-faithful view, and every row says which ending it had.

mod filter;

use filter::Regex;

use std::collections::BTreeMap;
use std::rc::Rc;

use kuna_analysis::listing::xrefs::XrefIndex;
use kuna_analysis::strings::kuna_stringinv::{self, FoundString, Termination};
use kuna_base::address::Address;
use kuna_console::engine::ConsoleProgram;

use crate::args::take_value as take;
use crate::decompile_all::{load_program, mode_options_for_binary, Args, DriverDefaults};
use crate::jsonfmt::{dumps_indent2, Json};

/// The parsed command line.
pub(crate) struct StringsArgs {
    binary: String,
    json: bool,
    min_length: usize,
    /// The compiled `--filter`, with the pattern it came from (the JSON echoes it).
    filter: Option<(String, Regex)>,
    ascii: bool,
    utf8: bool,
    utf16: bool,
    encoding_label: String,
    section: Option<String>,
    termination: Termination,
    no_xrefs: bool,
    options: Vec<(String, String)>,
    mode: Option<String>,
    slice: Option<String>,
    target: Option<String>,
    sleighpath: Option<String>,
    isa: Option<kuna_console::engine::ArmIsa>,
}

/// A row, ready to render: the recovered literal plus who reaches it.
struct Row {
    found: FoundString,
    xrefs_count: usize,
    /// The functions the references come from, `(entry, name)`, address-ordered.
    functions: Vec<(u64, String)>,
}

/// `kuna strings` entry point. Wire as `"strings" => strings::run(rest)`.
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

/// Scan, attribute, filter, render — the whole command in one pass.
pub(crate) fn query(args: &StringsArgs) -> Result<String, String> {
    let bytes = crate::image::image_bytes(
        &args.binary,
        kuna_analysis::loader::macho_fat::slice_pref(args.slice.as_deref(), args.target.as_deref()),
    )?;
    let file = kuna_analysis::loadimage_object::parse_object(&*bytes)
        .map_err(|e| format!("could not parse {}: {e}", args.binary))?;

    let inv = kuna_stringinv::inventory(
        &file,
        &kuna_stringinv::Query {
            min_len: args.min_length,
            ascii: args.ascii,
            utf8: args.utf8,
            utf16: args.utf16,
            section: args.section.clone(),
            termination: args.termination,
        },
    );
    if let Some(want) = &args.section {
        if !inv.regions.iter().any(|n| n == want || n.strip_prefix('.') == Some(want.as_str())) {
            let mut have: Vec<&str> = inv.regions.iter().map(String::as_str).collect();
            have.dedup();
            let have = if have.is_empty() {
                "the image has no scannable sections".to_string()
            } else {
                format!("have: {}", have.join(", "))
            };
            return Err(format!("no section named {want:?} ({have})"));
        }
    }

    let mut truncated_filter = false;
    let found: Vec<FoundString> = inv
        .strings
        .into_iter()
        .filter(|s| match &args.filter {
            Some((_, re)) => {
                let (hit, gave_up) = re.is_match(&s.text);
                truncated_filter |= gave_up;
                hit
            }
            None => true,
        })
        .collect();

    // The reference edges are the expensive half, so they are skipped when the
    // caller opted out and when nothing survived the filter.
    let rows = if args.no_xrefs || found.is_empty() {
        found
            .into_iter()
            .map(|found| Row { found, xrefs_count: 0, functions: Vec::new() })
            .collect()
    } else {
        attribute(args, &file, found)?
    };

    if truncated_filter {
        eprintln!(
            "warning: --filter hit its backtracking budget on at least one candidate; \
             those rows are reported as non-matching"
        );
    }
    Ok(if args.json {
        format!("{}\n", dumps_indent2(&result_json(args, inv.from_segments, &rows)))
    } else {
        render_text(args, inv.from_segments, &rows)
    })
}

/// Attach the reference edges: load the program once (the same in-process seam
/// `xrefs` and `decompile-all` use), index every reference, and answer "who
/// reaches this literal" per row.
fn attribute(
    args: &StringsArgs,
    file: &object::File,
    found: Vec<FoundString>,
) -> Result<Vec<Row>, String> {
    let options =
        mode_options_for_binary(args.mode.as_deref(), &args.binary, args.options.clone())?;
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
    // The inventory, read ONCE. `ConsoleProgram::find_entry_at` rebuilds and
    // linearly scans this whole vector per call, which is a rounding error while
    // no string has an owner and 0.85 s of a 2.5 s answer on an ARM image where
    // 1,792 of them do.
    let inventory: BTreeMap<u64, String> = prog
        .function_entries_canonical()
        .into_iter()
        .map(|e| (e.addr.get_offset(), e.name))
        .collect();
    let seeds: Vec<u64> = inventory.keys().copied().collect();
    let index = kuna_analysis::listing::xrefs::build(
        file,
        prog.arch(),
        prog.arch().translate(),
        &seeds,
    );

    // A name is looked up once per referencing function, not once per row: a
    // prompt string referenced from twenty sites resolves one entry.
    let mut names: BTreeMap<u64, String> = BTreeMap::new();
    Ok(found
        .into_iter()
        .map(|found| {
            let mut xrefs_count = 0usize;
            let mut functions: Vec<(u64, String)> = Vec::new();
            // A reference may land anywhere in the literal, not only on its first
            // byte — `lea rax,[fmt+4]` is still a use of `fmt`.
            for vma in found.addr..found.addr.saturating_add(u64::from(found.byte_len)) {
                for r in index.refs_to(vma) {
                    xrefs_count += 1;
                    let Some(entry) = owning_function(&prog, &index, r.from) else {
                        continue;
                    };
                    if functions.iter().any(|(e, _)| *e == entry) {
                        continue;
                    }
                    let name = names
                        .entry(entry)
                        .or_insert_with(|| function_name(&prog, &inventory, entry))
                        .clone();
                    functions.push((entry, name));
                }
            }
            functions.sort();
            Row { found, xrefs_count, functions }
        })
        .collect())
}

/// The entry of the function `vma` lies in — the walk's own attribution first
/// (it knows which descent reached the instruction), then the engine's inventory.
pub(crate) fn owning_function(prog: &ConsoleProgram, index: &XrefIndex, vma: u64) -> Option<u64> {
    index.function_containing(vma).or_else(|| prog.find_entry_at(vma).map(|e| e.addr.get_offset()))
}

/// The display name for a function entry, falling back to the engine's own
/// placeholder (`sub_<addr>`) so a row is never nameless.
pub(crate) fn function_name(prog: &ConsoleProgram, inventory: &BTreeMap<u64, String>, entry: u64) -> String {
    inventory
        .get(&entry)
        .cloned()
        .or_else(|| prog.find_entry_at(entry).map(|e| e.name))
        .or_else(|| prog.function_named_at(entry))
        .unwrap_or_else(|| match prog.arch().manage().get_default_code_space() {
            Some(space) => prog.arch().name_function(&Address::new(Rc::clone(space), entry)),
            None => format!("sub_{entry:x}"),
        })
}

// --- rendering ---------------------------------------------------------------

/// Render `text` on one line: the recognizer admits TAB/CR/LF, which would break
/// both the tab-separated rows and any line-oriented reader downstream.
fn escape_text(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for ch in text.chars() {
        match ch {
            '\\' => out.push_str("\\\\"),
            '\t' => out.push_str("\\t"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            c => out.push(c),
        }
    }
    out
}

/// The `{name, address, address_hex}` triple — the house address shape `xrefs`
/// and `decompile-all` already emit.
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

/// Build the `strings --json` document.
fn result_json(args: &StringsArgs, from_segments: bool, rows: &[Row]) -> Json {
    let strings = Json::Array(
        rows.iter()
            .map(|row| {
                Json::Object(vec![
                    ("address".into(), Json::Number(row.found.addr.to_string())),
                    ("address_hex".into(), Json::Str(format!("0x{:x}", row.found.addr))),
                    ("text".into(), Json::Str(row.found.text.clone())),
                    ("length".into(), Json::Number(row.found.char_len.to_string())),
                    ("byte_length".into(), Json::Number(row.found.byte_len.to_string())),
                    ("nul_terminated".into(), Json::Bool(row.found.nul_terminated)),
                    ("encoding".into(), Json::Str(row.found.encoding.as_str().to_string())),
                    ("section".into(), optional_str(row.found.section.as_deref())),
                    ("xrefs_count".into(), Json::Number(row.xrefs_count.to_string())),
                    (
                        "functions".into(),
                        Json::Array(
                            row.functions
                                .iter()
                                .map(|(addr, name)| function_json(name, *addr))
                                .collect(),
                        ),
                    ),
                ])
            })
            .collect(),
    );
    Json::Object(vec![
        ("binary".into(), Json::Str(args.binary.clone())),
        ("encoding".into(), Json::Str(args.encoding_label.clone())),
        ("min_length".into(), Json::Number(args.min_length.to_string())),
        ("filter".into(), optional_str(args.filter.as_ref().map(|(p, _)| p.as_str()))),
        ("section".into(), optional_str(args.section.as_deref())),
        ("termination".into(), Json::Str(termination_label(args.termination).to_string())),
        ("scanned".into(), Json::Str(scanned_label(from_segments).to_string())),
        ("xrefs".into(), Json::Bool(!args.no_xrefs)),
        ("count".into(), Json::Number(rows.len().to_string())),
        ("strings".into(), strings),
    ])
}

/// Which run endings the scan accepted — the other answer to "why is this empty".
fn termination_label(termination: Termination) -> &'static str {
    match termination {
        Termination::Nul => "nul",
        Termination::Any => "any",
    }
}

/// Which address set the scan covered — the answer to "why is this empty".
fn scanned_label(from_segments: bool) -> &'static str {
    if from_segments {
        "segments"
    } else {
        "sections"
    }
}

/// The human surface: a `#` header naming the query, then one tab-separated row
/// per string — address, encoding, length, section, reference count, referencing
/// functions, text. The text is last because it is the only unbounded column.
fn render_text(args: &StringsArgs, from_segments: bool, rows: &[Row]) -> String {
    use std::fmt::Write as _;
    let mut out = String::new();
    let plural = if rows.len() == 1 { "string" } else { "strings" };
    let _ = writeln!(
        out,
        "# {} {plural} in {} ({}, min length {}, termination {}, scanned by {})",
        rows.len(),
        args.binary,
        args.encoding_label,
        args.min_length,
        termination_label(args.termination),
        scanned_label(from_segments)
    );
    for row in rows {
        let functions = if row.functions.is_empty() {
            "-".to_string()
        } else {
            row.functions.iter().map(|(_, n)| n.as_str()).collect::<Vec<_>>().join(",")
        };
        let _ = writeln!(
            out,
            "0x{:x}\t{}\t{}\t{}\t{}\t{}\t{}",
            row.found.addr,
            row.found.encoding.as_str(),
            row.found.char_len,
            row.found.section.as_deref().unwrap_or("-"),
            row.xrefs_count,
            functions,
            escape_text(&row.found.text)
        );
    }
    out
}

// --- argument parsing --------------------------------------------------------

pub(crate) fn parse_args(argv: &[String]) -> Result<StringsArgs, String> {
    let mut binary: Option<String> = None;
    let mut json = false;
    let mut min_length: Option<usize> = None;
    let mut filter: Option<(String, Regex)> = None;
    let mut encoding = "ascii".to_string();
    let mut section: Option<String> = None;
    let mut termination = Termination::Any;
    let mut no_xrefs = false;
    let mut options: Vec<(String, String)> = Vec::new();
    let mut mode: Option<String> = None;
    let mut slice: Option<String> = None;
    let mut target: Option<String> = None;
    let mut sleighpath: Option<String> = None;
    let mut isa = None;

    let mut i = 0;
    while i < argv.len() {
        let a = argv[i].as_str();
        match a {
            "--json" => json = true,
            "--min-length" => {
                let v = take(argv, &mut i, "--min-length")?;
                let n: usize =
                    v.parse().map_err(|_| format!("--min-length wants a number, not {v:?}"))?;
                if n == 0 {
                    return Err("--min-length must be at least 1".into());
                }
                min_length = Some(n);
            }
            "--filter" => {
                // Compiled here, not at query time: a pattern the matcher cannot
                // parse is a malformed command line (exit 2), not a failed query.
                let pattern = take(argv, &mut i, "--filter")?;
                let re = Regex::compile(&pattern).map_err(|e| format!("--filter: {e}"))?;
                filter = Some((pattern, re));
            }
            "--encoding" => encoding = take(argv, &mut i, "--encoding")?.to_ascii_lowercase(),
            "--section" => section = Some(take(argv, &mut i, "--section")?),
            "--termination" => {
                let v = take(argv, &mut i, "--termination")?.to_ascii_lowercase();
                termination = match v.as_str() {
                    "any" => Termination::Any,
                    "nul" | "null" => Termination::Nul,
                    other => return Err(format!("unknown --termination {other:?} (nul, any)")),
                };
            }
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

    let binary = binary.ok_or("strings requires <binary>")?;
    // `--isa` only reaches the reference walk, which `--no-xrefs` skips: refuse
    // the pair rather than accept a decode-mode selection that cannot apply.
    if no_xrefs && isa.is_some() {
        return Err("--isa has no effect with --no-xrefs (no code is decoded)".into());
    }
    // `utf8` implies the 1-byte width, because the UTF-8 reading IS that width
    // with multi-byte sequences decoded — an ASCII-only literal is a UTF-8 one.
    let (ascii, utf8, utf16) = match encoding.as_str() {
        "ascii" => (true, false, false),
        "utf8" => (true, true, false),
        "utf16" => (false, false, true),
        "all" => (true, true, true),
        other => {
            return Err(format!("unknown --encoding {other:?} (ascii, utf8, utf16, all)"))
        }
    };
    Ok(StringsArgs {
        binary,
        json,
        // The analyzer's own `MinStringLen.LEN_5`, so an unflagged run reports
        // exactly the inventory the engine marked up.
        min_length: min_length.unwrap_or(5),
        filter,
        ascii,
        utf8,
        utf16,
        encoding_label: encoding,
        section,
        termination,
        no_xrefs,
        options,
        mode,
        slice,
        target,
        sleighpath,
        isa,
    })
}

fn usage() {
    eprintln!(
        "usage: kuna strings <binary> [--json] [--min-length N] [--filter REGEX] \\\n\
         \x20                    [--encoding ascii|utf8|utf16|all] [--termination nul|any] \\\n\
         \x20                    [--section NAME] [--no-xrefs] \\\n\
         \x20                    [--mode auto|reliable|aggressive|fast] [--option N V].. \\\n\
         \x20                    [--isa auto|arm|thumb] [--slice ARCH] [--target T] [--sleighpath D]\n\
         \n\
         Lists the text the analyzer tier's own matcher finds, each with the\n\
         functions that reference it.  Defaults: ascii, minimum length 5,\n\
         termination any.\n\
         \n\
         --encoding utf16 reads 2-byte little-endian units; a wide Windows literal\n\
         is a one-character string at 1-byte width.\n\
         --encoding utf8 decodes multi-byte sequences at the 1-byte width, so a\n\
         literal opening with a non-ASCII character keeps its true start address\n\
         (and therefore its references) instead of being reported from the byte\n\
         after its last sequence.  `all` uses it in place of the ascii reading;\n\
         a row with no multi-byte content is still reported as ascii.\n\
         --termination nul takes only NUL-ended runs -- exactly the char[N]\n\
         literals the engine marks up.  The default `any` also reports a run\n\
         closed by any other byte, which is what recovers a length-prefixed name\n\
         table; every row carries nul_terminated so the two stay distinguishable.\n\
         --filter takes a POSIX-flavored regex (literals . * + ? | () [] {{n,m}}\n\
         ^ $ \\\\d \\\\w \\\\s and their negations, plus a leading (?i)), matched anywhere\n\
         in the text.\n\
         --no-xrefs skips the reference walk (xrefs_count 0, no functions).\n\
         \n\
         --json emits {{binary,encoding,min_length,termination,count,strings:[{{address,\n\
         address_hex,text,length,byte_length,nul_terminated,encoding,section,xrefs_count,\n\
         functions}}]}}; without it, one tab-separated row each."
    );
}

#[cfg(test)]
mod tests {
    use super::escape_text;

    #[test]
    fn text_is_escaped_onto_one_line() {
        assert_eq!(escape_text("a\tb\nc\\d"), "a\\tb\\nc\\\\d");
    }
}
