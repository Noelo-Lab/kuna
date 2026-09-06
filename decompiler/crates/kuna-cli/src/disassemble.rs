//! `kuna disassemble` — the instruction listing.
//!
//! ```text
//!   kuna disassemble <binary> <name|0xaddr|0xstart-0xend> [--addr] [--as VIEW]
//!                    [--count N] [--bytes N] [--json] [--mode MODE]
//!                    [--option N V].. [--slice ARCH] [--target T] [--sleighpath D]
//!   kuna read <binary> <name|0xaddr|0xstart-0xend> ...   # the same, --as data
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
//!
//! ## Two views, because a data address is not an instruction stream
//!
//! Listing a data blob as instructions is worse than useless: an RE agent that
//! asked kuna for the encoded globals at `0x100003f30` got `ADD byte ptr
//! [RCX],AL` / `OR CL,byte ptr [RBX]` back — a decode of `00 01 02 03 ..` that is
//! correct in the translator and a lie about the program. It went to `xxd`, which
//! is the friction this view removes (`docs/re-needs/cli-mode-read-raw.md`).
//!
//! So the target picks its own rendering. `--as auto` (the default) asks the
//! loader which section the start address is in: a section carrying `DATA`
//! without `CODE` renders as a hexdump — address, bytes, ASCII gutter, and a
//! contiguous `hex` string in `--json` — and says on stderr why it did.
//! `--as code` and `--as data` override it in either direction, because a packer
//! puts real code in `.data` and a compiler puts real data in `__TEXT`. Nothing
//! about the decode changes: the same walk, the same bytes, a different view of
//! them.

use kuna_console::engine::{ConsoleProgram, FixedRefs};
use kuna_sleigh::loadimage::section_flags;

use crate::decompile::looks_like_addr;
use crate::decompile_all::{load_program, mode_options_for_binary, Args, DriverDefaults};
use crate::jsonfmt::{dumps_indent2, Json};
use crate::litpool;

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

/// The same safety stop for a byte listing nobody sized, in hexdump rows.
const DERIVED_ROW_CAP: usize = DERIVED_INSTRUCTION_CAP;

/// How many bytes one hexdump row covers — the `xxd` width an RE agent's eye is
/// already calibrated to.
const HEXDUMP_ROW_BYTES: usize = 16;

/// The mnemonic given to a byte the translator would not decode.
const BAD_BYTE_MNEMONIC: &str = ".byte";

/// What to render at the target.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum View {
    /// Decoded instructions.
    Code,
    /// The bytes themselves, as a hexdump.
    Data,
}

/// `--as`: what the caller asked for, before the program gets a say.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum ViewRequest {
    /// Let the loader's section flags choose ([`choose_view`]).
    Auto,
    Code,
    Data,
}

/// The parsed command line.
pub(crate) struct DisArgs {
    pub(crate) binary: String,
    /// The target operand: a symbol name, an address, or a `start-end` range.
    pub(crate) spec: String,
    /// `--addr`: read `spec` as an address even when it is bare hex.
    pub(crate) by_address: bool,
    /// `--as code|data|auto`. `None` is the command's own default — `Auto` for
    /// `kuna disassemble`, `Data` for `kuna read`.
    pub(crate) view: Option<ViewRequest>,
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
    /// Did the target resolve to a discovered function entry? Such a target is
    /// code by definition, whatever section it was linked into.
    from_entry: bool,
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
        hex(&self.bytes)
    }
}

/// One hexdump row: up to [`HEXDUMP_ROW_BYTES`] bytes of the image.
struct DataRow {
    addr: u64,
    bytes: Vec<u8>,
}

impl DataRow {
    /// Contiguous lowercase hex — the spelling that pastes into another tool,
    /// and the one `--json` reports (per row and, concatenated, for the span).
    fn hex(&self) -> String {
        hex(&self.bytes)
    }

    /// The same bytes space-separated (`xxd -g1`), for the human column, where a
    /// 32-character run of hex is unreadable.
    fn grouped(&self) -> String {
        self.bytes.iter().map(|b| format!("{b:02x}")).collect::<Vec<_>>().join(" ")
    }

    /// The printable-ASCII gutter; every other byte is a `.`, `xxd`-style.
    fn ascii(&self) -> String {
        self.bytes
            .iter()
            .map(|&b| if (0x20..0x7f).contains(&b) { b as char } else { '.' })
            .collect()
    }
}

fn hex(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        s.push_str(&format!("{b:02x}"));
    }
    s
}

/// What the command produced: the document for stdout, plus the notes that
/// belong on stderr. A note never enters the document, so `--json` stays
/// machine-readable — it is repeated in the JSON's own `notes` array instead.
pub(crate) struct Listing {
    pub(crate) text: String,
    pub(crate) notes: Vec<String>,
}

/// `kuna disassemble` entry point.
pub fn run(argv: &[String]) -> i32 {
    run_as(argv, ViewRequest::Auto)
}

/// `kuna read` entry point — the same query with the byte view as its default,
/// so an agent that wants the bytes of a data address does not have to know that
/// the command for it is spelled `disassemble`. An explicit `--as code` still
/// wins, because the two spellings are one command.
pub fn run_read(argv: &[String]) -> i32 {
    run_as(argv, ViewRequest::Data)
}

fn run_as(argv: &[String], default_view: ViewRequest) -> i32 {
    let mut args = match parse_args(argv) {
        Ok(a) => a,
        Err(e) => {
            eprintln!("error: {e}");
            usage();
            return 2;
        }
    };
    args.view.get_or_insert(default_view);
    match render(&args) {
        Ok(listing) => {
            for note in &listing.notes {
                eprintln!("note: {note}");
            }
            crate::output::emit_with_status(&listing.text, 0)
        }
        Err(e) => {
            eprintln!("error: {e}");
            1
        }
    }
}

/// Load, resolve, walk, and render — the whole command, minus the stdout
/// boundary, so the listing is testable without a subprocess.
pub(crate) fn render(args: &DisArgs) -> Result<Listing, String> {
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
        assertions: Vec::new(),
        assert_strict: false,
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
    let (view, mut notes) = choose_view(&prog, &region, args.view.unwrap_or(ViewRequest::Auto));
    let text = match view {
        View::Code => {
            let (rows, truncated, folded) = walk(&prog, &region, args.count);
            if folded > 0 {
                notes.push(pool_note(folded));
            }
            if args.json {
                format!("{}\n", dumps_indent2(&result_json(args, &region, &rows, truncated, &notes)))
            } else {
                render_text(&region, &rows, truncated)
            }
        }
        View::Data => {
            let (rows, truncated) = walk_data(&prog, &region, args.count);
            if args.json {
                format!(
                    "{}\n",
                    dumps_indent2(&data_result_json(args, &region, &rows, truncated, &notes))
                )
            } else {
                render_data_text(&region, &rows, truncated)
            }
        }
    };
    Ok(Listing { text, notes })
}

// --- which view ---------------------------------------------------------------

/// (kuna) The loader `section_flags` bits of the section containing `vma`.
///
/// `None` when no section covers it — which includes every loader that publishes
/// no section table at all (the XML `<binaryimage>` corpus, a raw byte blob).
/// There the byte view can only be asked for, never inferred, which is the right
/// way round: an inference from missing evidence is a guess.
fn section_flags_at(prog: &ConsoleProgram, vma: u64) -> Option<u32> {
    prog.sections()
        .into_iter()
        .find(|(start, size, _)| vma >= *start && vma - start < *size)
        .map(|(_, _, flags)| flags)
}

/// Decide between the instruction listing and the hexdump, and say why when the
/// program — not the caller — made the choice.
///
/// `auto` believes the loader's own section classification and nothing else: a
/// section the format marks as data and not as code (`.rdata`, `__TEXT,__const`,
/// `.rodata`) holds bytes, so it is shown as bytes. A discovered function entry
/// is code whatever section it was linked into, and an address in a section that
/// carries `CODE` — or in no section the loader knows about — keeps the
/// instruction listing it has always had.
fn choose_view(prog: &ConsoleProgram, region: &Region, want: ViewRequest) -> (View, Vec<String>) {
    let section = section_flags_at(prog, region.start);
    let view = decide_view(want, region.from_entry, section);
    let inferred = want == ViewRequest::Auto && view == View::Data;
    let notes = if inferred {
        vec![format!(
            "0x{:x} is in a non-executable data section, so these are bytes, not \
             instructions -- pass `--as code` to disassemble them anyway",
            region.start
        )]
    } else {
        Vec::new()
    };
    (view, notes)
}

/// The decision itself, with the program's evidence already gathered: the
/// caller's demand wins outright, a discovered entry is code wherever it was
/// linked, and only a section the loader marks `DATA` without `CODE` flips the
/// view. A loader that published no section covering the address (`None`) is
/// silence, not evidence, and keeps the instruction listing.
fn decide_view(want: ViewRequest, from_entry: bool, section: Option<u32>) -> View {
    match want {
        ViewRequest::Code => View::Code,
        ViewRequest::Data => View::Data,
        ViewRequest::Auto if from_entry => View::Code,
        ViewRequest::Auto => match section {
            Some(flags) if flags & section_flags::DATA != 0 && flags & section_flags::CODE == 0 => {
                View::Data
            }
            _ => View::Code,
        },
    }
}

// --- the walk ----------------------------------------------------------------

/// Decode forward from `region.start` until a stop is reached: the region's end,
/// the instruction budget, the derived-length cap, or memory that will not read.
///
/// An address the translator rejects is listed as a single `.byte` row rather
/// than ending the listing, because the common reason for one is a listing that
/// ran into inline data and there is usually code again after it.
///
/// A word the listing's OWN instructions read at a fixed address is then folded
/// back into a data row rather than left decoded as the instruction its bytes
/// happen to spell — the literal pool an ARM/MIPS/PPC function carries inside
/// its own extent. What is folded, and what refuses a fold, is
/// [`crate::litpool`]; the fold never moves a row, so the listing's addresses
/// are the same either way.
///
/// Returns the rows, whether the walk was truncated, and how many words were
/// folded — which the caller says out loud, because "this is not an
/// instruction" is exactly the fact an agent reading a listing needs and cannot
/// infer from the row.
fn walk(
    prog: &ConsoleProgram,
    region: &Region,
    count: Option<usize>,
) -> (Vec<Row>, bool, usize) {
    let (rows, truncated, refs) = decode_rows(prog, region, count);
    let (rows, folded) = fold_pool_words(prog, region, rows, &refs);
    (rows, truncated, folded)
}

/// The straight-line decode itself: rows in address order, whether the walk was
/// truncated, and the fixed-address evidence the rows carry.
fn decode_rows(
    prog: &ConsoleProgram,
    region: &Region,
    count: Option<usize>,
) -> (Vec<Row>, bool, FixedRefs) {
    let cap = if region.derived { Some(DERIVED_INSTRUCTION_CAP) } else { None };
    let mut rows: Vec<Row> = Vec::new();
    let mut evidence = FixedRefs::default();
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
                prog.add_fixed_refs_at(addr, &mut evidence);
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
    (rows, truncated, evidence)
}

/// Replace each proved literal-pool word with one data row.
///
/// The rows a word covers are folded into a single `.word 0x...` row over the
/// same bytes, so the listing's addresses are untouched — [`crate::litpool`]
/// only proves a word whose width tiles whole decoded rows, which is what makes
/// that true.
fn fold_pool_words(
    prog: &ConsoleProgram,
    region: &Region,
    rows: Vec<Row>,
    evidence: &FixedRefs,
) -> (Vec<Row>, usize) {
    let Some(&Row { addr: first, .. }) = rows.first() else {
        return (rows, 0);
    };
    let last = rows.last().map_or(first, |r| r.addr + r.size);
    // A word only qualifies where the program says data can live and does not
    // say code does: a mapped non-writable section (a GOT slot is read by
    // address too, and a writable `.text` is a packer), and no function symbol
    // installed at that very address. The extent clip already keeps a *named*
    // target's listing off the next function, but an explicit multi-function
    // range walks straight through entries and must not eat one.
    let sections = prog.sections();
    let is_pool_slot = |vma: u64| {
        sections
            .iter()
            .find(|(start, size, _)| vma >= *start && vma - start < *size)
            .is_some_and(|(_, _, flags)| flags & section_flags::READONLY != 0)
            && prog.function_named_at(vma).is_none()
    };
    let boundaries: Vec<litpool::Boundary> = rows.iter().map(|r| (r.addr, r.size)).collect();
    let pool = litpool::pool_words(
        &boundaries,
        &evidence.reads,
        &evidence.flow_targets,
        (region.start.max(first), last),
        &is_pool_slot,
    );
    if pool.is_empty() {
        return (rows, 0);
    }
    let big_endian = prog.arch().translate().is_big_endian();
    let mut out: Vec<Row> = Vec::with_capacity(rows.len());
    let mut folded = 0;
    let mut it = rows.into_iter();
    while let Some(row) = it.next() {
        let Some(&width) = pool.get(&row.addr) else {
            out.push(row);
            continue;
        };
        let (addr, mut bytes) = (row.addr, row.bytes);
        while (bytes.len() as u64) < width {
            match it.next() {
                Some(next) => bytes.extend_from_slice(&next.bytes),
                None => break,
            }
        }
        out.push(Row {
            addr,
            size: bytes.len() as u64,
            mnemonic: litpool::word_mnemonic(width).to_string(),
            operands: litpool::word_operand(&bytes, big_endian),
            bytes,
        });
        folded += 1;
    }
    (out, folded)
}

/// Read the region's bytes forward from `region.start` into hexdump rows, until
/// the region's end, the row budget, the derived-length cap, or memory that will
/// not read.
///
/// A short read ends the listing rather than zero-filling it: the point of this
/// view is that every byte it shows is a byte the image really holds.
fn walk_data(prog: &ConsoleProgram, region: &Region, count: Option<usize>) -> (Vec<DataRow>, bool) {
    let cap = if region.derived { Some(DERIVED_ROW_CAP) } else { None };
    let mut rows: Vec<DataRow> = Vec::new();
    let mut truncated = false;
    let mut addr = region.start;
    let mut raw: Vec<u8> = Vec::new();
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
        let want = match region.end {
            Some(end) => (end - addr).min(HEXDUMP_ROW_BYTES as u64) as usize,
            None => HEXDUMP_ROW_BYTES,
        };
        let got = read_upto(prog, addr, want, &mut raw);
        if got == 0 {
            break;
        }
        rows.push(DataRow { addr, bytes: raw[..got].to_vec() });
        addr = addr.saturating_add(got as u64);
        if got < want {
            break;
        }
    }
    (rows, truncated)
}

/// Read at most `want` bytes at `vma`, shortening at the first byte the image
/// will not hand over — a mapped segment ends where it ends, which is far more
/// often mid-row than on a 16-byte boundary. Returns how many were read.
fn read_upto(prog: &ConsoleProgram, vma: u64, want: usize, out: &mut Vec<u8>) -> usize {
    if want == 0 {
        return 0;
    }
    if prog.read_bytes_into(vma, want, out) {
        return want;
    }
    let mut one: Vec<u8> = Vec::new();
    let mut got: Vec<u8> = Vec::with_capacity(want);
    for i in 0..want {
        if !prog.read_bytes_into(vma.saturating_add(i as u64), 1, &mut one) {
            break;
        }
        got.push(one[0]);
    }
    *out = got;
    out.len()
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
        return Ok(Region {
            start,
            end: Some(end),
            name: name_at(prog, start),
            derived: false,
            from_entry: false,
        });
    }

    let (start, name, from_entry) = if addressy {
        let addr = parse_addr(spec)?;
        // An ARM caller legitimately holds an odd `entry|1` Thumb address; the
        // inventory folds the mode bit, so resolve through it when it knows the
        // entry and list where the instructions actually are.
        match prog.find_entry_at(addr) {
            Some(e) => (e.addr.get_offset(), Some(e.name), true),
            None => (addr, name_at(prog, addr), false),
        }
    } else if let Some(e) = prog.find_entry_by_name(spec) {
        (e.addr.get_offset(), Some(e.name), true)
    } else if let Some(a) = prog.lookup_symbol(spec) {
        let addr = a.get_offset();
        (addr, name_at(prog, addr).or_else(|| Some(spec.to_string())), false)
    } else if let Some((n, addr, _)) =
        prog.global_data_symbols().into_iter().find(|(n, _, _)| n == spec)
    {
        (addr, Some(n), false)
    } else if let Ok(addr) = u64::from_str_radix(spec, 16) {
        (addr, name_at(prog, addr), false)
    } else {
        return Err(format!(
            "no symbol named {spec:?} in {} (and it is not an address; pass --addr \
             for a bare hex address, or `kuna unpack` if the image is packed)",
            args.binary
        ));
    };

    Ok(match (args.bytes, args.count) {
        (Some(n), _) => Region {
            start,
            end: Some(start.saturating_add(n)),
            name,
            derived: false,
            from_entry,
        },
        (None, Some(_)) => Region { start, end: None, name, derived: false, from_entry },
        (None, None) => {
            let extent = prog.function_extent_at(start);
            let span = if extent > 0 { extent } else { DEFAULT_WINDOW_BYTES };
            Region {
                start,
                end: Some(start.saturating_add(span)),
                name,
                derived: true,
                from_entry,
            }
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
fn result_json(
    args: &DisArgs,
    region: &Region,
    rows: &[Row],
    truncated: bool,
    notes: &[String],
) -> Json {
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
    let mut doc = envelope(args, region, end, rows.len(), truncated, notes, "code");
    doc.push(("instructions".into(), instructions));
    Json::Object(doc)
}

/// Build the `--as data` (`kuna read`) document: the same envelope, then the
/// span's bytes as one contiguous hex string plus the hexdump rows.
///
/// `hex` is the whole span in one piece on purpose — an agent comparing kuna's
/// answer with `xxd` or pasting a table into a decoder wants one token, not a
/// re-join of N rows.
fn data_result_json(
    args: &DisArgs,
    region: &Region,
    rows: &[DataRow],
    truncated: bool,
    notes: &[String],
) -> Json {
    let end = rows.last().map_or(region.start, |r| r.addr + r.bytes.len() as u64);
    let mut hex = String::new();
    for r in rows {
        hex.push_str(&r.hex());
    }
    let listed = Json::Array(
        rows.iter()
            .map(|r| {
                Json::Object(vec![
                    ("address".into(), Json::Number(r.addr.to_string())),
                    ("address_hex".into(), Json::Str(format!("0x{:x}", r.addr))),
                    ("size".into(), Json::Number(r.bytes.len().to_string())),
                    ("bytes".into(), Json::Str(r.hex())),
                    ("ascii".into(), Json::Str(r.ascii())),
                ])
            })
            .collect(),
    );
    let mut doc = envelope(args, region, end, rows.len(), truncated, notes, "data");
    doc.push(("hex".into(), Json::Str(hex)));
    doc.push(("rows".into(), listed));
    Json::Object(doc)
}

/// The keys both views share, in one order, so a consumer reads `start`/`end`/
/// `count`/`bytes` the same way whichever view answered. `kind` says which one
/// did; `count` is the number of listed entries (instructions, or hexdump rows).
fn envelope(
    args: &DisArgs,
    region: &Region,
    end: u64,
    count: usize,
    truncated: bool,
    notes: &[String],
    kind: &str,
) -> Vec<(String, Json)> {
    vec![
        ("binary".into(), Json::Str(args.binary.clone())),
        ("kind".into(), Json::Str(kind.to_string())),
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
        ("count".into(), Json::Number(count.to_string())),
        ("bytes".into(), Json::Number((end - region.start).to_string())),
        ("truncated".into(), Json::Bool(truncated)),
        ("notes".into(), Json::Array(notes.iter().cloned().map(Json::Str).collect())),
    ]
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
    let label = label(region);
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

/// The human byte surface: `xxd -g1` with kuna's own address column — a `#`
/// header naming the span, then address, sixteen space-separated bytes, and the
/// printable-ASCII gutter. Space-separated because a 32-character run of hex is
/// not readable; `--json` carries the contiguous spelling for the machine.
fn render_data_text(region: &Region, rows: &[DataRow], truncated: bool) -> String {
    use std::fmt::Write as _;
    let end = rows.last().map_or(region.start, |r| r.addr + r.bytes.len() as u64);
    let mut out = String::new();
    let plural = if end - region.start == 1 { "byte" } else { "bytes" };
    let _ = writeln!(
        out,
        "# {} {plural} at {} (0x{:x}..0x{end:x}){}",
        end - region.start,
        label(region),
        region.start,
        if truncated { " [truncated: --bytes N or a 0xstart-0xend range reads more]" } else { "" }
    );
    for r in rows {
        let _ = writeln!(
            out,
            "{:<14}{:<49}|{}|",
            format!("0x{:x}", r.addr),
            r.grouped(),
            r.ascii()
        );
    }
    out
}

/// The instruction listing for one function body, `<vma>  <MNEMONIC operands>`
/// per line — the compact spelling `kuna decompile-graph` carries per function,
/// with no header and no byte column.
///
/// Walked by [`walk`], so it is the same decode this command lists and an
/// undecodable byte inside the body is a `.byte 0x..` row rather than the end of
/// the listing. `None` only when the extent is empty or nothing decoded.
pub(crate) fn function_listing(prog: &ConsoleProgram, start: u64, end: u64) -> Option<String> {
    use std::fmt::Write as _;
    if end <= start {
        return None;
    }
    let region =
        Region { start, end: Some(end), name: None, derived: false, from_entry: true };
    let (rows, _, _) = walk(prog, &region, None);
    if rows.is_empty() {
        return None;
    }
    let mut out = String::new();
    for r in &rows {
        let _ = writeln!(out, "{:08x}  {}", r.addr, r.text());
    }
    Some(out.trim_end().to_string())
}

/// Why a row in this listing is a data word and not the instruction its bytes
/// spell — the one fact an agent reading a `.word` row cannot infer, and the
/// move that decodes it anyway.
fn pool_note(folded: usize) -> String {
    let tail = "-- disassemble such an address on its own to decode it anyway";
    if folded == 1 {
        format!(
            "one word in this range is read as a constant by the range's own instructions \
             (a literal pool), so it is listed as data rather than decoded {tail}"
        )
    } else {
        format!(
            "{folded} words in this range are read as constants by the range's own \
             instructions (a literal pool), so they are listed as data rather than decoded \
             {tail}"
        )
    }
}

/// What to call the start address in a header: its name and address when the
/// program has a name for it, the bare address otherwise.
fn label(region: &Region) -> String {
    match &region.name {
        Some(name) => format!("{name} @ 0x{:x}", region.start),
        None => format!("0x{:x}", region.start),
    }
}

// --- argument parsing --------------------------------------------------------

pub(crate) fn parse_args(argv: &[String]) -> Result<DisArgs, String> {
    let mut positional: Vec<String> = Vec::new();
    let mut by_address = false;
    let mut view: Option<ViewRequest> = None;
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
            "--as" => view = Some(parse_view(&take(argv, &mut i, "--as")?)?),
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
        view,
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

fn parse_view(value: &str) -> Result<ViewRequest, String> {
    match value {
        "auto" => Ok(ViewRequest::Auto),
        "code" => Ok(ViewRequest::Code),
        "data" => Ok(ViewRequest::Data),
        other => Err(format!("--as takes code|data|auto, got {other:?}")),
    }
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
        "usage: kuna disassemble|read <binary> <name|0xaddr|0xstart-0xend> [--addr] \\\n\
         \x20                    [--as code|data|auto] [--count N] [--bytes N] [--json] \\\n\
         \x20                    [--mode auto|reliable|aggressive|fast] \\\n\
         \x20                    [--define-function S[-E][=N]|@FILE].. \\\n\
         \x20                    [--option N V].. [--slice ARCH] [--target T] [--sleighpath D]\n\
         \n\
         The target is a function name, an address (--addr for a bare hex one), or an\n\
         explicit range (0x1000-0x1040 / 0x1000..0x1040) for bytes no function owns.\n\
         A named function lists its whole extent; a raw address lists 64 bytes unless\n\
         --count / --bytes / a range says otherwise.\n\
         \n\
         --as picks the view. `code` decodes instructions, `data` prints a hexdump,\n\
         and `auto` (the default for `disassemble`) reads bytes when the address is\n\
         in a non-executable data section. `kuna read` is the same command with\n\
         `--as data` as its default.\n\
         \n\
         --define-function <start[-end][=name] | @file> (repeatable) declares a\n\
         boundary first, so a name the image never carried becomes a valid target.\n\
         \n\
         --json emits {{binary,kind,target,start,end,count,bytes,truncated,notes}} plus\n\
         instructions:[{{address,address_hex,size,bytes,mnemonic,operands,text}}] in the\n\
         code view, or hex + rows:[{{address,address_hex,size,bytes,ascii}}] in the data\n\
         view; without it, a header line and one row per instruction or 16 bytes."
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
    fn a_data_row_spells_its_bytes_three_ways() {
        let r = DataRow { addr: 0x1000, bytes: (0u8..16).collect() };
        assert_eq!(r.hex(), "000102030405060708090a0b0c0d0e0f");
        assert_eq!(r.grouped(), "00 01 02 03 04 05 06 07 08 09 0a 0b 0c 0d 0e 0f");
        assert_eq!(r.ascii(), "................");
        let text = DataRow { addr: 0, bytes: b"kuna \x7f\"\\".to_vec() };
        assert_eq!(text.ascii(), "kuna .\"\\", "0x7f is not printable; 0x20 and quotes are");
    }

    #[test]
    fn the_view_flag_takes_three_spellings_and_nothing_else() {
        assert_eq!(parse_view("auto"), Ok(ViewRequest::Auto));
        assert_eq!(parse_view("code"), Ok(ViewRequest::Code));
        assert_eq!(parse_view("data"), Ok(ViewRequest::Data));
        assert!(parse_view("hex").is_err());
        let argv: Vec<String> =
            ["a.out", "0x1000", "--addr", "--as", "data"].iter().map(|s| s.to_string()).collect();
        assert_eq!(parse_args(&argv).unwrap().view, Some(ViewRequest::Data));
        // Unspecified stays unspecified, so each entry point can default it.
        assert_eq!(parse_args(&["a.out".into(), "main".into()]).unwrap().view, None);
        assert!(
            parse_args(&["a.out".into(), "main".into(), "--as".into(), "asm".into()]).is_err(),
            "an unknown view is a usage error"
        );
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
        let region = Region {
            start: 0x1000,
            end: Some(0x1004),
            name: Some("main".into()),
            derived: true,
            from_entry: true,
        };
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

    #[test]
    fn the_byte_header_counts_bytes_and_the_rows_are_xxd_shaped() {
        let region = Region {
            start: 0x400915,
            end: Some(0x400925),
            name: Some("s_400915".into()),
            derived: false,
            from_entry: false,
        };
        let rows = vec![DataRow { addr: 0x400915, bytes: b"Username: \0\x01\x02\x03\x04\x05".to_vec() }];
        let text = render_data_text(&region, &rows, false);
        let mut lines = text.lines();
        assert_eq!(
            lines.next().unwrap(),
            "# 16 bytes at s_400915 @ 0x400915 (0x400915..0x400925)"
        );
        assert_eq!(
            lines.next().unwrap(),
            "0x400915      55 73 65 72 6e 61 6d 65 3a 20 00 01 02 03 04 05  |Username: ......|"
        );
        assert!(render_data_text(&region, &rows, true).contains("[truncated:"));
    }

    /// An explicit `--as` is the caller's, and `auto` only ever flips on the
    /// loader's own classification — never on an entry, never on silence.
    #[test]
    fn the_view_choice_believes_the_caller_then_the_section_flags() {
        let data = section_flags::DATA | section_flags::READONLY;
        let code = section_flags::CODE | section_flags::READONLY;
        use ViewRequest::{Auto, Code as WantCode, Data as WantData};
        for (want, from_entry, flags, expect) in [
            (Auto, false, Some(data), View::Data),
            (Auto, false, Some(code), View::Code),
            // A section marked BOTH (a Mach-O `__text` carrying data attributes)
            // is code: the executable bit is the stronger claim.
            (Auto, false, Some(data | code), View::Code),
            // A discovered function is code wherever it was linked.
            (Auto, true, Some(data), View::Code),
            // No section covering the address: silence, not evidence.
            (Auto, false, None, View::Code),
            // The caller outranks all of it, in either direction.
            (WantCode, false, Some(data), View::Code),
            (WantData, true, Some(code), View::Data),
            (WantData, false, None, View::Data),
        ] {
            assert_eq!(
                decide_view(want, from_entry, flags),
                expect,
                "{want:?} / from_entry {from_entry} / flags {flags:?}"
            );
        }
    }
}
