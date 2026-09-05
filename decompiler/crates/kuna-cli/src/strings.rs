//! `kuna strings` — the string inventory.
//!
//! ```text
//!   kuna strings <binary> [--json] [--min-length N] [--filter REGEX]
//!                         [--encoding ascii|utf16|all] [--section NAME] [--no-xrefs]
//!                         [--mode MODE] [--option N V].. [--slice ARCH] [--target T]
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

use std::collections::BTreeMap;
use std::rc::Rc;

use kuna_analysis::listing::xrefs::XrefIndex;
use kuna_analysis::strings::kuna_stringinv::{self, FoundString};
use kuna_base::address::Address;
use kuna_console::engine::ConsoleProgram;

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
    utf16: bool,
    encoding_label: String,
    section: Option<String>,
    no_xrefs: bool,
    options: Vec<(String, String)>,
    mode: Option<String>,
    slice: Option<String>,
    target: Option<String>,
    sleighpath: Option<String>,
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
    let bytes = kuna_analysis::loader::elf_shdr::read_image(&args.binary)
        .map_err(|e| format!("{}: {e}", args.binary))?;
    let file = object::File::parse(&*bytes)
        .map_err(|e| format!("could not parse {}: {e}", args.binary))?;

    let inv = kuna_stringinv::inventory(
        &file,
        &kuna_stringinv::Query {
            min_len: args.min_length,
            ascii: args.ascii,
            utf16: args.utf16,
            section: args.section.clone(),
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
    };
    let prog = load_program(&load, DriverDefaults::Inventory)?;
    let seeds: Vec<u64> =
        prog.function_entries_canonical().iter().map(|e| e.addr.get_offset()).collect();
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
                    let name =
                        names.entry(entry).or_insert_with(|| function_name(&prog, entry)).clone();
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
fn owning_function(prog: &ConsoleProgram, index: &XrefIndex, vma: u64) -> Option<u64> {
    index.function_containing(vma).or_else(|| prog.find_entry_at(vma).map(|e| e.addr.get_offset()))
}

/// The display name for a function entry, falling back to the engine's own
/// placeholder (`sub_<addr>`) so a row is never nameless.
fn function_name(prog: &ConsoleProgram, entry: u64) -> String {
    prog.find_entry_at(entry)
        .map(|e| e.name)
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
        ("scanned".into(), Json::Str(scanned_label(from_segments).to_string())),
        ("xrefs".into(), Json::Bool(!args.no_xrefs)),
        ("count".into(), Json::Number(rows.len().to_string())),
        ("strings".into(), strings),
    ])
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
        "# {} {plural} in {} ({}, min length {}, scanned by {})",
        rows.len(),
        args.binary,
        args.encoding_label,
        args.min_length,
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
    let mut no_xrefs = false;
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
            "--no-xrefs" => no_xrefs = true,
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

    let binary = binary.ok_or("strings requires <binary>")?;
    let (ascii, utf16) = match encoding.as_str() {
        "ascii" => (true, false),
        "utf16" => (false, true),
        "all" => (true, true),
        other => return Err(format!("unknown --encoding {other:?} (ascii, utf16, all)")),
    };
    Ok(StringsArgs {
        binary,
        json,
        // The analyzer's own `MinStringLen.LEN_5`, so an unflagged run reports
        // exactly the inventory the engine marked up.
        min_length: min_length.unwrap_or(5),
        filter,
        ascii,
        utf16,
        encoding_label: encoding,
        section,
        no_xrefs,
        options,
        mode,
        slice,
        target,
        sleighpath,
    })
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
        "usage: kuna strings <binary> [--json] [--min-length N] [--filter REGEX] \\\n\
         \x20                    [--encoding ascii|utf16|all] [--section NAME] [--no-xrefs] \\\n\
         \x20                    [--mode auto|reliable|aggressive|fast] [--option N V].. \\\n\
         \x20                    [--slice ARCH] [--target T] [--sleighpath D]\n\
         \n\
         Lists the string literals the analyzer tier already detects, each with the\n\
         functions that reference it.  Defaults: ascii, minimum length 5 (the\n\
         analyzer's own StringsAnalyzer settings).\n\
         \n\
         --encoding utf16 reads 2-byte little-endian units; a wide Windows literal\n\
         is a one-character string at 1-byte width.\n\
         --filter takes a POSIX-flavored regex (literals . * + ? | () [] {{n,m}}\n\
         ^ $ \\\\d \\\\w \\\\s and their negations, plus a leading (?i)), matched anywhere\n\
         in the text.\n\
         --no-xrefs skips the reference walk (xrefs_count 0, no functions).\n\
         \n\
         --json emits {{binary,encoding,min_length,count,strings:[{{address,address_hex,\n\
         text,length,byte_length,encoding,section,xrefs_count,functions}}]}}; without it,\n\
         one tab-separated row each."
    );
}

// --- the --filter matcher ----------------------------------------------------
//
// A small backtracking regex, hand-rolled for the same reason the rest of this
// CLI is (the workspace does not take a dependency for a leaf feature; `regex`
// is dev-only in the engine crates).  It covers the flavor an operator actually
// types at a string filter and REJECTS what it does not implement, so a pattern
// is never silently reinterpreted into a different one.

/// One alternative-free element of a character class.
#[derive(Debug)]
enum ClassItem {
    Char(char),
    Range(char, char),
    Digit(bool),
    Word(bool),
    Space(bool),
}

impl ClassItem {
    fn matches(&self, c: char) -> bool {
        match self {
            ClassItem::Char(x) => *x == c,
            ClassItem::Range(lo, hi) => *lo <= c && c <= *hi,
            ClassItem::Digit(want) => c.is_ascii_digit() == *want,
            ClassItem::Word(want) => (c.is_alphanumeric() || c == '_') == *want,
            ClassItem::Space(want) => c.is_whitespace() == *want,
        }
    }
}

#[derive(Debug)]
enum Node {
    Char(char),
    Any,
    Class { neg: bool, items: Vec<ClassItem> },
    Start,
    End,
    Concat(Vec<Node>),
    Alt(Vec<Node>),
    Repeat { node: Box<Node>, min: u32, max: u32, greedy: bool },
}

/// A compiled `--filter` pattern.
struct Regex {
    root: Node,
    icase: bool,
}

/// How many node visits one candidate string may cost before the match is
/// abandoned. A pathological pattern (`(a*)*b`) must not hang the command; the
/// caller is told when the budget was hit rather than being handed a silent
/// "no match".
const MATCH_BUDGET: i64 = 2_000_000;

impl Regex {
    fn compile(pattern: &str) -> Result<Regex, String> {
        let (body, icase) = match pattern.strip_prefix("(?i)") {
            Some(rest) => (rest, true),
            None => (pattern, false),
        };
        let chars: Vec<char> = body.chars().collect();
        let mut p = Parser { src: &chars, pos: 0, depth: 0 };
        let root = p.alt()?;
        if p.pos != chars.len() {
            return Err(format!("unexpected {:?} at offset {}", chars[p.pos], p.pos));
        }
        Ok(Regex { root, icase })
    }

    /// Does the pattern match anywhere in `text`? The second element is `true`
    /// when the search ran out of backtracking budget (the answer is then a
    /// conservative "no").
    fn is_match(&self, text: &str) -> (bool, bool) {
        let chars: Vec<char> = text.chars().collect();
        let m = Matcher {
            text: &chars,
            icase: self.icase,
            budget: std::cell::Cell::new(MATCH_BUDGET),
        };
        for start in 0..=chars.len() {
            if m.node(&self.root, start, &mut |_| true) {
                return (true, false);
            }
            if m.budget.get() <= 0 {
                return (false, true);
            }
        }
        (false, false)
    }
}

struct Matcher<'t> {
    text: &'t [char],
    /// Fold case on both sides of every character comparison (a leading `(?i)`).
    icase: bool,
    budget: std::cell::Cell<i64>,
}

impl Matcher<'_> {
    /// Does a pattern character match a text character, honoring `(?i)`?
    fn same(&self, pat: char, c: char) -> bool {
        pat == c || (self.icase && pat.to_lowercase().eq(c.to_lowercase()))
    }

    /// The forms of `c` a character class is tried against: itself, plus its
    /// other case under `(?i)`, so `[A-Z]` and `[a-z]` both match either.
    fn variants(&self, c: char) -> [char; 2] {
        if !self.icase {
            return [c, c];
        }
        [c.to_lowercase().next().unwrap_or(c), c.to_uppercase().next().unwrap_or(c)]
    }

    /// Charge one node visit; `false` once the budget is gone.
    fn spend(&self) -> bool {
        let left = self.budget.get() - 1;
        self.budget.set(left);
        left > 0
    }

    /// Match `nodes` in order from `pos`, handing every end position to `k`.
    fn seq(&self, nodes: &[Node], pos: usize, k: &mut dyn FnMut(usize) -> bool) -> bool {
        match nodes.split_first() {
            None => k(pos),
            Some((head, rest)) => self.node(head, pos, &mut |p| self.seq(rest, p, k)),
        }
    }

    fn node(&self, node: &Node, pos: usize, k: &mut dyn FnMut(usize) -> bool) -> bool {
        if !self.spend() {
            return false;
        }
        match node {
            Node::Char(c) => {
                self.text.get(pos).is_some_and(|&t| self.same(*c, t)) && k(pos + 1)
            }
            Node::Any => pos < self.text.len() && k(pos + 1),
            Node::Class { neg, items } => match self.text.get(pos) {
                Some(&c) => {
                    let variants = self.variants(c);
                    let inside = items.iter().any(|i| variants.iter().any(|&v| i.matches(v)));
                    (inside != *neg) && k(pos + 1)
                }
                None => false,
            },
            Node::Start => pos == 0 && k(pos),
            Node::End => pos == self.text.len() && k(pos),
            Node::Concat(nodes) => self.seq(nodes, pos, k),
            Node::Alt(branches) => branches.iter().any(|b| self.node(b, pos, k)),
            Node::Repeat { node, min, max, greedy } => {
                self.repeat(node, *min, *max, *greedy, pos, 0, k)
            }
        }
    }

    fn repeat(
        &self,
        node: &Node,
        min: u32,
        max: u32,
        greedy: bool,
        pos: usize,
        count: u32,
        k: &mut dyn FnMut(usize) -> bool,
    ) -> bool {
        if !self.spend() {
            return false;
        }
        // A zero-width body would repeat forever, so one empty iteration ends the
        // loop rather than extending it.
        let more = |k: &mut dyn FnMut(usize) -> bool| {
            count < max
                && self.node(node, pos, &mut |p| {
                    p != pos && self.repeat(node, min, max, greedy, p, count + 1, k)
                })
        };
        let done = |k: &mut dyn FnMut(usize) -> bool| count >= min && k(pos);
        if greedy {
            more(k) || done(k)
        } else {
            done(k) || more(k)
        }
    }
}

/// Recursive-descent parser for the supported flavor.
struct Parser<'p> {
    src: &'p [char],
    pos: usize,
    depth: u32,
}

/// Nesting cap: a hand-rolled recursive-descent parser must refuse a pattern deep
/// enough to overflow the stack rather than crash on it.
const MAX_DEPTH: u32 = 64;

impl Parser<'_> {
    fn peek(&self) -> Option<char> {
        self.src.get(self.pos).copied()
    }

    fn alt(&mut self) -> Result<Node, String> {
        let mut branches = vec![self.concat()?];
        while self.peek() == Some('|') {
            self.pos += 1;
            branches.push(self.concat()?);
        }
        Ok(if branches.len() == 1 { branches.pop().unwrap() } else { Node::Alt(branches) })
    }

    fn concat(&mut self) -> Result<Node, String> {
        let mut nodes = Vec::new();
        while let Some(c) = self.peek() {
            if c == '|' || c == ')' {
                break;
            }
            nodes.push(self.repeat()?);
        }
        Ok(Node::Concat(nodes))
    }

    fn repeat(&mut self) -> Result<Node, String> {
        let atom = self.atom()?;
        let (min, max) = match self.peek() {
            Some('*') => (0, u32::MAX),
            Some('+') => (1, u32::MAX),
            Some('?') => (0, 1),
            Some('{') => return self.counted(atom),
            _ => return Ok(atom),
        };
        self.pos += 1;
        let greedy = if self.peek() == Some('?') {
            self.pos += 1;
            false
        } else {
            true
        };
        Ok(Node::Repeat { node: Box::new(atom), min, max, greedy })
    }

    /// `{n}` / `{n,}` / `{n,m}`. A `{` that does not open a valid counter is a
    /// literal brace, the way an operator's shell-quoted pattern usually means it.
    fn counted(&mut self, atom: Node) -> Result<Node, String> {
        let save = self.pos;
        self.pos += 1;
        let min = match self.number() {
            Some(n) => n,
            None => {
                self.pos = save;
                return Ok(atom);
            }
        };
        let max = match self.peek() {
            Some('}') => min,
            Some(',') => {
                self.pos += 1;
                self.number().unwrap_or(u32::MAX)
            }
            _ => {
                self.pos = save;
                return Ok(atom);
            }
        };
        if self.peek() != Some('}') {
            self.pos = save;
            return Ok(atom);
        }
        self.pos += 1;
        if min > max {
            return Err(format!("{{{min},{max}}} counts down"));
        }
        let greedy = if self.peek() == Some('?') {
            self.pos += 1;
            false
        } else {
            true
        };
        Ok(Node::Repeat { node: Box::new(atom), min, max, greedy })
    }

    fn number(&mut self) -> Option<u32> {
        let start = self.pos;
        while self.peek().is_some_and(|c| c.is_ascii_digit()) {
            self.pos += 1;
        }
        if self.pos == start {
            return None;
        }
        self.src[start..self.pos].iter().collect::<String>().parse().ok()
    }

    fn atom(&mut self) -> Result<Node, String> {
        let c = self.peek().ok_or("pattern ends where an expression was expected")?;
        self.pos += 1;
        match c {
            '(' => {
                if self.depth >= MAX_DEPTH {
                    return Err("pattern nests too deeply".into());
                }
                // `(?:` is accepted and means the same thing: nothing here
                // captures, so a group is always non-capturing.
                if self.src[self.pos..].starts_with(&['?', ':']) {
                    self.pos += 2;
                } else if self.peek() == Some('?') {
                    return Err("only (?:...) groups and a leading (?i) are supported".into());
                }
                self.depth += 1;
                let inner = self.alt()?;
                self.depth -= 1;
                if self.peek() != Some(')') {
                    return Err("unclosed (".into());
                }
                self.pos += 1;
                Ok(inner)
            }
            ')' => Err("unmatched )".into()),
            '[' => self.class(),
            ']' => Ok(Node::Char(']')),
            '.' => Ok(Node::Any),
            '^' => Ok(Node::Start),
            '$' => Ok(Node::End),
            '*' | '+' | '?' => Err(format!("nothing for {c:?} to repeat")),
            '\\' => {
                let e = self.peek().ok_or("pattern ends in a backslash")?;
                self.pos += 1;
                Ok(match escape_class(e) {
                    Some(item) => Node::Class { neg: false, items: vec![item] },
                    None => Node::Char(escape_char(e)?),
                })
            }
            other => Ok(Node::Char(other)),
        }
    }

    fn class(&mut self) -> Result<Node, String> {
        let neg = self.peek() == Some('^');
        if neg {
            self.pos += 1;
        }
        let mut items = Vec::new();
        // A `]` in first position is a literal, the POSIX convention.
        if self.peek() == Some(']') {
            self.pos += 1;
            items.push(ClassItem::Char(']'));
        }
        loop {
            let c = self.peek().ok_or("unclosed [")?;
            self.pos += 1;
            if c == ']' {
                break;
            }
            let lo = if c == '\\' {
                let e = self.peek().ok_or("pattern ends in a backslash")?;
                self.pos += 1;
                match escape_class(e) {
                    Some(item) => {
                        items.push(item);
                        continue;
                    }
                    None => escape_char(e)?,
                }
            } else {
                c
            };
            // `a-z`, but a trailing `-` before `]` is a literal hyphen.
            if self.peek() == Some('-') && self.src.get(self.pos + 1).is_some_and(|&n| n != ']') {
                self.pos += 1;
                let hi = self.peek().ok_or("unclosed [")?;
                self.pos += 1;
                let hi = if hi == '\\' {
                    let e = self.peek().ok_or("pattern ends in a backslash")?;
                    self.pos += 1;
                    escape_char(e)?
                } else {
                    hi
                };
                if hi < lo {
                    return Err(format!("range [{lo}-{hi}] counts down"));
                }
                items.push(ClassItem::Range(lo, hi));
            } else {
                items.push(ClassItem::Char(lo));
            }
        }
        if items.is_empty() {
            return Err("empty character class".into());
        }
        Ok(Node::Class { neg, items })
    }
}

/// The shorthand classes, or `None` when the escape is a literal character.
fn escape_class(e: char) -> Option<ClassItem> {
    Some(match e {
        'd' => ClassItem::Digit(true),
        'D' => ClassItem::Digit(false),
        'w' => ClassItem::Word(true),
        'W' => ClassItem::Word(false),
        's' => ClassItem::Space(true),
        'S' => ClassItem::Space(false),
        _ => return None,
    })
}

/// The literal an escape stands for. An alphanumeric escape that is not one of
/// the supported ones is REFUSED rather than read as its bare letter: `\b` means
/// a word boundary to everyone who types it, and silently matching a literal `b`
/// would answer a different question.
fn escape_char(e: char) -> Result<char, String> {
    Ok(match e {
        'n' => '\n',
        'r' => '\r',
        't' => '\t',
        '0' => '\0',
        c if c.is_alphanumeric() => return Err(format!("unsupported escape \\{c}")),
        other => other,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hit(pattern: &str, text: &str) -> bool {
        Regex::compile(pattern).expect("pattern compiles").is_match(text).0
    }

    #[test]
    fn literal_and_anchors() {
        assert!(hit("Enter the", "Enter the 128-byte quantum key"));
        assert!(!hit("^the", "Enter the key"));
        assert!(hit("^Enter", "Enter the key"));
        assert!(hit("key$", "Enter the key"));
        assert!(!hit("Enter$", "Enter the key"));
    }

    #[test]
    fn classes_repeats_and_alternation() {
        assert!(hit("%[0-9]*[sd]", "value=%s"));
        assert!(hit("a+b", "aaab"));
        assert!(hit("colou?r", "color"));
        assert!(hit("colou?r", "colour"));
        assert!(hit("cat|dog", "hotdog"));
        assert!(!hit("cat|dog", "hotpony"));
        assert!(hit("\\d{3}-\\d{4}", "call 555-1234 now"));
        assert!(!hit("\\d{3}-\\d{5}", "call 555-1234 now"));
        assert!(hit("[^a-z]+", "ABC"));
        assert!(!hit("^[^a-z]+$", "AbC"));
    }

    #[test]
    fn the_serial_format_is_findable_by_its_own_syntax() {
        // The literal an operator would paste straight out of the decompilation:
        // every metacharacter escaped.
        assert!(hit("%\\[\\^-\\]", "%[^-]-%[^-]-%s"));
        // And the shape query, which must not match an ordinary format string.
        assert!(hit("%\\[\\^.\\]-%\\[\\^.\\]-%s", "%[^-]-%[^-]-%s"));
        assert!(!hit("%\\[\\^.\\]-%\\[\\^.\\]-%s", "%s-%s-%s"));
    }

    #[test]
    fn case_insensitive_prefix() {
        assert!(hit("(?i)PASSWORD", "Enter password:"));
        assert!(!hit("PASSWORD", "Enter password:"));
    }

    #[test]
    fn a_bad_pattern_is_rejected_not_reinterpreted() {
        let bad =
            ["(unclosed", "[a-", "*leading", "a{3,1}", "trailing\\", "(?=x)", "a\\b", "\\x41"];
        for bad in bad {
            assert!(Regex::compile(bad).is_err(), "{bad:?} must not compile");
        }
        // A `{` that is not a counter stays a literal brace.
        assert!(hit("APOCALYPSE\\{", "APOCALYPSE{THE_END_OF_CRACKMES}"));
        assert!(hit("x{not a count}", "x{not a count}"));
    }

    #[test]
    fn a_pathological_pattern_gives_up_instead_of_hanging() {
        // The classic exponential blowup: every split of the a-run is retried
        // before the missing `b` fails the match.
        let re = Regex::compile("(a+)+b").expect("compiles");
        let (hit, gave_up) = re.is_match(&"a".repeat(64));
        assert!(!hit && gave_up, "the budget must stop the search");
    }

    #[test]
    fn case_folding_reaches_classes_and_ranges() {
        assert!(hit("(?i)[a-z]+", "ABC"));
        assert!(hit("(?i)[A-Z]+", "abc"));
        assert!(!hit("[A-Z]+", "abc"));
        // The negated shorthands keep their meaning under (?i).
        assert!(hit("(?i)\\D", "x"));
        assert!(!hit("(?i)^\\d+$", "abc"));
    }

    #[test]
    fn text_is_escaped_onto_one_line() {
        assert_eq!(escape_text("a\tb\nc\\d"), "a\\tb\\nc\\\\d");
    }
}
