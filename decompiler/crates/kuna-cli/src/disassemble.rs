//! `kuna disassemble` — the instruction listing.
//!
//! ```text
//!   kuna disassemble <binary> <name|0xaddr|0xstart-0xend> [--addr] [--as VIEW]
//!                    [--count N] [--bytes N] [--json] [--mode MODE]
//!                    [--option N V].. [--isa auto|arm|thumb] [--slice ARCH] [--target T] [--sleighpath D]
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
//! as `.byte` rows, so a listing that walked into data says so in place rather
//! than silently stopping. On an architecture whose instructions must be
//! aligned, one such row runs to the next alignment boundary, because that is
//! the only place code can resume ([`resume_grid`]).
//!
//! ## Where a straight line is the wrong walk
//!
//! Decoding forward from one end is a guess that every byte in the range starts
//! an instruction or is inside one, and hand-written or obfuscated code breaks
//! it deliberately: an `EB 01` jumps over one junk byte, and from that byte on
//! the listing is a decode of instructions the program never executes. It does
//! not merely lose a row — it invents calls and out-of-image jumps that are not
//! in the bytes (`docs/re-needs/linear-disassembly-silently-skips.md`).
//!
//! Two answers. The listing always reports the addresses its OWN branches name
//! that no row of it starts at ([`skipped_targets`]), because a decode a byte
//! out of phase is spelled exactly like one that is not. And `--follow` decodes
//! from those addresses as well as from the start ([`follow_rows`]), filling
//! what flow never reaches with the same straight-line walk, so it never lists
//! less than the plain listing does. Where two reachable paths read the same
//! bytes as different instructions, `--follow` lists both rows: that is what the
//! bytes mean, and dropping either would be a guess.
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

use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap};

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

/// The mnemonic given to bytes the translator would not decode.
const BAD_BYTE_MNEMONIC: &str = ".byte";

/// How many skipped branch targets [`skipped_note`] spells out before it counts
/// the rest. Three is the whole crop on the witness (a 70-instruction listing of
/// `EB 01` jump-over-junk idioms), so a note that names six is a note that names
/// them all on anything but a listing already beyond reading.
const SKIPPED_NOTE_ADDRS: usize = 6;

/// The widest resume grid [`resume_grid`] will infer from a listing's own rows.
/// No instruction set aligns further than this, so a wider shared alignment is
/// a coincidence of a short listing, not a grid.
const MAX_RESUME_GRID: u64 = 16;

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
    /// `--follow`: decode from every address the range's own branches name, not
    /// only from one end ([`follow_rows`]).
    pub(crate) follow: bool,
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
    pub(crate) isa: Option<kuna_console::engine::ArmIsa>,
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
    /// (kuna, `disassembly-fabricates-zero-byte`) Exclusive end of the mapped
    /// run holding [`Self::start`] ([`mapped_run_end`]) — a bound the image
    /// imposes rather than one the caller asked for. `None` where the loader
    /// publishes no segment map and there is nothing to bound the walk with.
    mapped_end: Option<u64>,
}

impl Region {
    /// Where the walk actually stops: the tighter of the caller's own stop and
    /// the end of mapped memory.
    fn stop(&self) -> Option<u64> {
        match (self.end, self.mapped_end) {
            (Some(end), Some(mapped)) => Some(end.min(mapped)),
            (end, mapped) => end.or(mapped),
        }
    }

    /// Could the caller's own request have run past the end of mapped memory?
    /// True for an unbounded `--count` walk, which has no byte stop at all.
    fn reaches_past_image(&self) -> bool {
        self.mapped_end.is_some_and(|mapped| self.end.is_none_or(|end| end > mapped))
    }
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

    // A caller-bounded window is answerable without the program-wide discovery
    // walk, so try it there first and keep the answer only if it is already the
    // one the full inventory would give ([`windowed_answer_is_final`]).
    if window_is_caller_bounded(args) {
        let load = load_args(args, windowed_options(&options, &args.options));
        let prog = load_program(&load, DriverDefaults::Inventory)?;
        if let Ok(region) = resolve_region(&prog, args) {
            let section = section_flags_at(&prog, region.start);
            if windowed_answer_is_final(&region, section, args.view.unwrap_or(ViewRequest::Auto)) {
                return listing_for(args, &prog, region);
            }
        }
    }

    // The inventory bundle: this surface enumerates entries to resolve a name
    // and to bound a function, and decompiles nothing.
    let load = load_args(args, options);
    let prog = load_program(&load, DriverDefaults::Inventory)?;

    let region = resolve_region(&prog, args)?;
    listing_for(args, &prog, region)
}

/// The load arguments this surface takes, with `options` already resolved.
fn load_args(args: &DisArgs, options: Vec<(String, String)>) -> Args {
    Args {
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
        isa: args.isa,
        raw_image: false,
        base: None,
        ..Args::serial()
    }
}

/// Render the listing for an already-resolved target.
fn listing_for(args: &DisArgs, prog: &ConsoleProgram, region: Region) -> Result<Listing, String> {
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
    let (view, mut notes) = choose_view(prog, &region, args.view.unwrap_or(ViewRequest::Auto));
    let text = match view {
        View::Code => {
            let CodeWalk { rows, truncated, folded, skipped, anchored } =
                walk(prog, &region, args.count, args.follow);
            if folded > 0 {
                notes.push(pool_note(folded));
            }
            if anchored > 0 {
                notes.push(follow_note(anchored));
            }
            if !skipped.is_empty() {
                notes.push(skipped_note(&skipped, args.follow));
            }
            let end = rows.last().map_or(region.start, |r| r.addr + r.size);
            if cut_short_by_image(&region, args.count, rows.len(), end) {
                notes.push(image_bound_note(prog, &region));
            }
            if args.json {
                format!("{}\n", dumps_indent2(&result_json(args, &region, &rows, truncated, &notes)))
            } else {
                render_text(&region, &rows, truncated)
            }
        }
        View::Data => {
            let (rows, truncated) = walk_data(prog, &region, args.count);
            let end = rows.last().map_or(region.start, |r| r.addr + r.bytes.len() as u64);
            if cut_short_by_image(&region, args.count, rows.len(), end) {
                notes.push(image_bound_note(prog, &region));
            }
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

// --- the windowed load --------------------------------------------------------

/// The whole-image discovery passes a caller-bounded listing does not need.
///
/// These two are the gates of the analysis tier's deferred Listing build
/// (`kuna_analysis::passes::run_listing_consumers`); every other discovery pass a
/// `--mode` preset turns on — `funcstart_patterns`, `aif`, `ptrentry`,
/// `tailcallentry`, `poolentry` — is a consumer of that build and is inert once
/// they are off.
const WINDOW_SUPPRESSED: [&str; 2] = ["listing", "fast_funcdisc"];

/// Did the caller bound the listing themselves — `--count`, `--bytes`, or an
/// explicit `start-end` range?
///
/// A window the caller did not bound is the function extent the inventory
/// reports (`function_extent_at`), so there is nothing to answer without it.
fn window_is_caller_bounded(args: &DisArgs) -> bool {
    args.count.is_some() || args.bytes.is_some() || split_range(args.spec.trim()).is_some()
}

/// `options` with the whole-image discovery passes turned off, unless the caller
/// named one — then their word stands.
///
/// What turns them on here is the resolved `--mode` preset, and that preset is a
/// whole-binary *decompilation* policy: `auto` selects `fast` from 2 MiB up, and
/// the walk `fast` retains (`fast_funcdisc`) decodes every executable byte in the
/// image. On a 9.4 MB PE whose `.text` is 99.4% of the file that is 20 s of
/// recursive descent to print 40 instructions, and the listing itself reads
/// nothing the walk produces. `explicit` is the caller's own `--option` list,
/// before the preset was merged into it.
fn windowed_options(
    options: &[(String, String)],
    explicit: &[(String, String)],
) -> Vec<(String, String)> {
    let mut out = options.to_vec();
    for name in WINDOW_SUPPRESSED {
        if !explicit.iter().any(|(option, _)| option == name) {
            out.push((name.to_string(), "off".to_string()));
        }
    }
    out
}

/// Is the windowed load's answer already the one the full inventory would give?
///
/// The discovery walk reaches a caller-bounded listing through exactly two
/// values: the target's `name`, and `from_entry`, which forces the instruction
/// view. So the windowed answer stands when the program named the target from a
/// fact it already held, and when the view it picked does not turn on an entry
/// the walk might have found there. Everything else — a name only discovery
/// invents, a bare address in a data section — falls back and pays for the walk,
/// which is why the fallback is what keeps this from being the variant swap that
/// loses `kuna disassemble <generated name>` on a stripped image.
fn windowed_answer_is_final(region: &Region, section: Option<u32>, want: ViewRequest) -> bool {
    region.name.is_some()
        && (region.from_entry
            || decide_view(want, false, section) == decide_view(want, true, section))
}

/// (kuna, `disassembly-fabricates-zero-byte`) The exclusive end of the
/// contiguous mapped run containing `vma`, and the next mapped address above it.
///
/// The load image answers a read that STARTS on mapped memory for its whole
/// length, zero-filling every byte past the last segment it crosses — the
/// upstream BFD contract (`LoadImageBfd::loadFill`, `loadimage_object.rs`). So
/// the start address is the only thing that path checks, and a listing that asks
/// for more bytes than the run holds gets the rest of them invented: `kuna
/// disassemble 0x80d190b` on an unmapped address correctly refuses, while
/// `kuna disassemble 0x80d18b0 --count 30` on the same image walked 23 bytes
/// past the same segment's end and reported eight `ADD byte ptr [EAX],AL` rows
/// that are not in the file. The bound has to be carried here instead: the start
/// is checked against the image, and the length is clipped to the run holding
/// it, so the windowed listing agrees with the direct query.
///
/// Adjacent and overlapping segments merge, because a listing that crosses from
/// one `PT_LOAD` into the next at the very byte the first ends has crossed
/// nothing — only a genuine hole stops it. `None` when the loader publishes no
/// segments at all (the XML `<binaryimage>` corpus, a relocatable object): that
/// is silence, not a bound, and the walk stands as it was.
fn mapped_run_end(prog: &ConsoleProgram, vma: u64) -> Option<(u64, Option<u64>)> {
    mapped_run(&prog.segments(), vma)
}

/// [`mapped_run_end`] over a segment list, so the merge is testable without a
/// loaded program.
fn mapped_run(segments: &[(u64, u64, u32)], vma: u64) -> Option<(u64, Option<u64>)> {
    let mut runs: Vec<(u64, u64)> = segments
        .iter()
        .filter(|&&(_, size, _)| size > 0)
        .map(|&(start, size, _)| (start, start.saturating_add(size)))
        .collect();
    runs.sort_unstable();
    let mut merged: Vec<(u64, u64)> = Vec::with_capacity(runs.len());
    for (start, end) in runs {
        match merged.last_mut() {
            Some(last) if start <= last.1 => last.1 = last.1.max(end),
            _ => merged.push((start, end)),
        }
    }
    let hit = merged.iter().position(|&(start, end)| vma >= start && vma < end)?;
    Some((merged[hit].1, merged.get(hit + 1).map(|&(start, _)| start)))
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
/// An address the translator rejects is listed as a `.byte` row rather than
/// ending the listing, because the common reason for one is a listing that ran
/// into inline data and there is usually code again after it. Where that code
/// can resume is [`resume_grid`]: one byte later on a variable-length
/// architecture, the next alignment boundary on a fixed-width one.
///
/// A word the listing's OWN instructions read at a fixed address is then folded
/// back into a data row rather than left decoded as the instruction its bytes
/// happen to spell — the literal pool an ARM/MIPS/PPC function carries inside
/// its own extent. What is folded, and what refuses a fold, is
/// [`crate::litpool`]; the fold never moves a row, so the listing's addresses
/// are the same either way.
///
/// Under `--follow` the straight line is not the only anchor: the walk also
/// starts at every address the range's own branches name, so a jump over a
/// decoy byte cannot pull the rest of the listing out of phase
/// ([`follow_rows`]).
///
/// Either way the branch targets no row starts at are reported
/// ([`skipped_targets`]) — the fact an agent reading an obfuscated listing
/// cannot infer, because a desynchronized decode looks exactly like an ordinary
/// one.
fn walk(
    prog: &ConsoleProgram,
    region: &Region,
    count: Option<usize>,
    follow: bool,
) -> CodeWalk {
    // What the straight line makes of this window is the baseline either way:
    // without `--follow` it IS the listing, and with it, its skipped targets are
    // the count of what following recovered.
    let (rows, truncated, refs) = decode_rows(prog, region, count, 0);
    let plain = skipped_targets(&rows, &refs);
    let (rows, truncated, refs, anchored) = if follow {
        let (rows, truncated, refs) = follow_rows(prog, region, count);
        (rows, truncated, refs, plain.len())
    } else {
        (rows, truncated, refs, 0)
    };
    // Before the fold, so the targets are tested against the boundaries the
    // decode actually produced rather than against a pool word's merged span.
    let skipped = if follow { skipped_targets(&rows, &refs) } else { plain };
    let (rows, folded) = fold_pool_words(prog, region, rows, &refs);
    CodeWalk { rows, truncated, folded, skipped, anchored }
}

/// What one code-view walk produced: the rows, whether it was cut short, how
/// many literal-pool words were folded, which branch targets it has no row for,
/// and how many of those the straight line would have walked over (0 without
/// `--follow`, where they are reported rather than decoded).
struct CodeWalk {
    rows: Vec<Row>,
    truncated: bool,
    folded: usize,
    skipped: Vec<u64>,
    anchored: usize,
}

/// (kuna, `linear-disassembly-silently-skips`) The addresses this listing's own
/// branches name that no row in it starts at.
///
/// Rows tile their span, so an in-range target no row starts at lies strictly
/// inside one — which means the instruction printed over it is a decode of bytes
/// the program never executes as that instruction, and so is everything after it
/// until the decode happens to re-synchronize. That is the whole `EB 01`
/// jump-over-a-decoy-byte idiom, and a straight-line listing of it is not merely
/// missing a row: it prints calls and jumps the bytes do not contain.
///
/// Only targets INSIDE the listed span are reported. A branch out of the range
/// is not evidence about the range, and the listing is not claiming to have
/// decoded where it points.
fn skipped_targets(rows: &[Row], evidence: &FixedRefs) -> Vec<u64> {
    let (Some(first), Some(last)) = (rows.first(), rows.last()) else {
        return Vec::new();
    };
    let span = first.addr..(last.addr + last.size);
    let starts: BTreeSet<u64> = rows.iter().map(|r| r.addr).collect();
    let mut out: Vec<u64> = evidence
        .flow_targets
        .iter()
        .copied()
        .filter(|t| span.contains(t) && !starts.contains(t))
        .collect();
    out.sort_unstable();
    out.dedup();
    out
}

/// `--follow`: the same window, decoded from every address its own branches name
/// rather than from one end only.
///
/// Two passes. The first is an ordinary recursive descent seeded at
/// `region.start`: each run steps forward while the instruction falls through
/// (`FixedRefs::falls_through`, the `xref_control_flow` last-op rule), stops at
/// an address some earlier run already claimed, and hands every in-window
/// address it branches or calls to back to the worklist. Seeds are taken in
/// address order, so a bounded walk spends its budget on the rows it is going to
/// print. Where two runs contest the same bytes the earlier claim stands, which
/// is what makes the pass terminate: no address is ever decoded twice.
///
/// The second pass fills what flow never reached — the decoy byte a jump was
/// over, the padding after a return, a handler nothing in this window calls — by
/// running the ordinary straight-line [`decode_rows`] across each gap, clipped so
/// that no instruction crosses into a claimed row. So `--follow` never lists
/// LESS than the straight line does: it is the same listing with every branch
/// target forced to start a row, and the bytes that then do not tile an
/// instruction spelled out as `.byte`.
///
/// Returns the rows, whether the listing was cut short, and the fixed-address
/// evidence from both passes.
fn follow_rows(
    prog: &ConsoleProgram,
    region: &Region,
    count: Option<usize>,
) -> (Vec<Row>, bool, FixedRefs) {
    let cap = if region.derived { Some(DERIVED_INSTRUCTION_CAP) } else { None };
    let stop = region.stop();
    let in_window = |a: u64| a >= region.start && stop.is_none_or(|end| a < end);
    let mut evidence = FixedRefs::default();
    let mut claimed: BTreeMap<u64, Row> = BTreeMap::new();
    // Address order, so a listing bounded by `--count` claims the low addresses
    // it will actually print before it spends the budget on a far callee.
    let mut seeds: BinaryHeap<Reverse<u64>> = BinaryHeap::new();
    seeds.push(Reverse(region.start));
    // A row can only be printed once, so the printed budget bounds the claims;
    // an explicit range that asked for neither is bounded by the window itself.
    let claim_budget = count.or(cap).unwrap_or(usize::MAX);
    let (mut mnem, mut body, mut raw) = (String::new(), String::new(), Vec::new());
    while let Some(Reverse(seed)) = seeds.pop() {
        if !in_window(seed) || claimed.contains_key(&seed) {
            continue;
        }
        let mut addr = seed;
        while in_window(addr) && !claimed.contains_key(&addr) && claimed.len() < claim_budget {
            // No instruction may reach past mapped memory or into bytes an
            // earlier run already spelled as an instruction. The window's own
            // end is NOT a ceiling: the straight line lists the instruction
            // straddling it whole, and `--follow` must not list less.
            let ceiling = claimed
                .range(addr.saturating_add(1)..)
                .next()
                .map(|(&a, _)| a)
                .into_iter()
                .chain(region.mapped_end)
                .min();
            let decoded = prog
                .disassemble_at_into(addr, &mut mnem, &mut body)
                .ok()
                .filter(|&n| n > 0)
                .filter(|&n| ceiling.is_none_or(|c| addr.saturating_add(n as u64) <= c));
            let Some(len) = decoded else { break };
            if !prog.read_bytes_into(addr, len as usize, &mut raw) {
                break;
            }
            let known = evidence.flow_targets.len();
            prog.add_fixed_refs_at(addr, &mut evidence);
            for &target in &evidence.flow_targets[known..] {
                if in_window(target) {
                    seeds.push(Reverse(target));
                }
            }
            claimed.insert(
                addr,
                Row {
                    addr,
                    size: len as u64,
                    bytes: raw.clone(),
                    mnemonic: mnem.clone(),
                    operands: body.clone(),
                },
            );
            if !evidence.falls_through {
                break;
            }
            addr = addr.saturating_add(len as u64);
        }
    }
    let (rows, truncated) = fill_between(prog, region, claimed, count, cap, &mut evidence);
    (rows, truncated, evidence)
}

/// Merge the claimed rows with a straight-line decode of everything between
/// them, in address order, until the row budget or the window runs out.
fn fill_between(
    prog: &ConsoleProgram,
    region: &Region,
    claimed: BTreeMap<u64, Row>,
    count: Option<usize>,
    cap: Option<usize>,
    evidence: &mut FixedRefs,
) -> (Vec<Row>, bool) {
    let stop = region.stop();
    // The row budget both bounds impose, whichever is tighter.
    let budget = match (count, cap) {
        (Some(asked), Some(derived)) => Some(asked.min(derived)),
        (asked, derived) => asked.or(derived),
    };
    // Every claimed row is alignment evidence for the gaps between them, which
    // have none of their own: a gap that opens on bytes the translator refuses
    // must still resume on the architecture's grid.
    let witness = claimed.values().fold(0u64, |w, r| w | r.addr | r.size);
    let mut out: Vec<Row> = Vec::new();
    let mut cursor = region.start;
    let mut claims = claimed.into_values().peekable();
    loop {
        let left = match budget {
            // Out of budget with something still to list: the answer is short by
            // the ask, not by the image.
            Some(n) if out.len() >= n => {
                return (out, claims.peek().is_some() || stop.is_none_or(|end| cursor < end));
            }
            Some(n) => Some(n - out.len()),
            // Unbounded only where the window has an end of its own, so the gap
            // decode below still stops.
            None => None,
        };
        match claims.peek() {
            Some(row) if row.addr <= cursor => {
                let row = claims.next().expect("peeked");
                cursor = cursor.max(row.addr.saturating_add(row.size));
                out.push(row);
            }
            next => {
                // The gap runs to the next claim, or to the end of the window.
                let gap_end = next.map(|r| r.addr).or(stop);
                if gap_end.is_some_and(|end| end <= cursor) {
                    return (out, false);
                }
                let gap = Region {
                    start: cursor,
                    end: gap_end,
                    name: None,
                    derived: false,
                    from_entry: false,
                    // The clip: nothing decoded in a gap may reach into a
                    // claimed row or past mapped memory. The window's own end
                    // is not one, so the last row of a `--follow` listing is
                    // the same row the straight line ends on.
                    mapped_end: next.map(|r| r.addr).or(region.mapped_end),
                };
                let (filled, _, refs) = decode_rows(prog, &gap, left, witness);
                let Some(last) = filled.last() else {
                    return (out, false);
                };
                cursor = last.addr.saturating_add(last.size);
                evidence.reads.extend(refs.reads);
                evidence.flow_targets.extend(refs.flow_targets);
                out.extend(filled);
            }
        }
    }
}

/// The straight-line decode itself: rows in address order, whether the walk was
/// truncated, and the fixed-address evidence the rows carry.
///
/// `seed_witness` is the alignment evidence a caller already holds — the OR of
/// the addresses and sizes of rows it decoded elsewhere in this same listing.
/// A walk that starts on undecodable bytes has none of its own, and on a
/// fixed-width architecture that is the difference between resuming on the
/// instruction grid and resuming one byte in ([`resume_grid`]). `0` for a walk
/// that is the whole listing.
fn decode_rows(
    prog: &ConsoleProgram,
    region: &Region,
    count: Option<usize>,
    seed_witness: u64,
) -> (Vec<Row>, bool, FixedRefs) {
    let cap = if region.derived { Some(DERIVED_INSTRUCTION_CAP) } else { None };
    let stop = region.stop();
    let alignment = u64::try_from(prog.arch().translate().get_alignment()).unwrap_or(1);
    let mut rows: Vec<Row> = Vec::new();
    let mut evidence = FixedRefs::default();
    let mut truncated = false;
    let mut addr = region.start;
    // The alignment every decoded row so far shares, folded into one OR of
    // their addresses and sizes; `resume_grid` reads its low zero bits.
    let mut witness = seed_witness;
    let (mut mnem, mut body, mut raw) = (String::new(), String::new(), Vec::new());
    loop {
        if count.is_some_and(|n| rows.len() >= n) {
            break;
        }
        if stop.is_some_and(|end| addr >= end) {
            break;
        }
        if cap.is_some_and(|c| rows.len() >= c) {
            truncated = true;
            break;
        }
        // An instruction that runs off the end of mapped memory was decoded out
        // of the load image's zero fill, not out of the file, so it is not an
        // instruction: the mapped bytes under it are listed as `.byte` instead.
        let decoded = prog
            .disassemble_at_into(addr, &mut mnem, &mut body)
            .ok()
            .filter(|&n| n > 0)
            .filter(|&n| {
                !region.mapped_end.is_some_and(|m| addr.saturating_add(n as u64) > m)
            });
        // The bytes are read back separately, so a row is only reported as an
        // instruction when BOTH the decode and the read succeeded — a row can
        // never claim a length it cannot show the bytes for.
        match decoded {
            Some(len) if prog.read_bytes_into(addr, len as usize, &mut raw) => {
                prog.add_fixed_refs_at(addr, &mut evidence);
                witness |= addr | len as u64;
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
                let want = recovery_span(addr, resume_grid(alignment, witness), stop);
                // A span that will not read whole falls back to the one byte
                // that is always safe; a row never claims bytes it cannot show.
                if !prog.read_bytes_into(addr, want as usize, &mut raw)
                    && !prog.read_bytes_into(addr, 1, &mut raw)
                {
                    break;
                }
                rows.push(Row {
                    addr,
                    size: raw.len() as u64,
                    bytes: raw.clone(),
                    mnemonic: BAD_BYTE_MNEMONIC.to_string(),
                    operands: bad_byte_operand(&raw),
                });
                addr = addr.saturating_add(raw.len() as u64);
            }
        }
    }
    (rows, truncated, evidence)
}

/// The address grid a listing resumes on after bytes the translator refused.
///
/// One byte is the right answer on a variable-length architecture: any address
/// can start an instruction there, and inline data is usually followed by code
/// again. On an architecture whose instructions must be aligned it is the wrong
/// answer, because code can only resume on the grid — stepping one byte puts
/// every row after the data off it, which is how an ARM function's five-word
/// literal pool came back as one `.byte` and four invented instructions.
///
/// `alignment` is the architecture's declared minimum (SLEIGH `define
/// alignment`); `1` means nothing is aligned and the byte-at-a-time recovery
/// stands. Above that the grid is the alignment the rows already decoded all
/// share — `witness`, the OR of each decoded row's address and size — so an ARM
/// listing of 4-byte rows resumes on 4 and a Thumb listing that has decoded a
/// 2-byte row resumes on 2. With nothing decoded yet there is no grid to infer
/// and the walk steps one byte.
fn resume_grid(alignment: u64, witness: u64) -> u64 {
    if alignment <= 1 || witness == 0 {
        return 1;
    }
    (1u64 << witness.trailing_zeros().min(u64::BITS - 1)).min(MAX_RESUME_GRID)
}

/// How many bytes one recovery row covers: forward to the next `grid`
/// boundary — a whole grid step when `addr` is already on one, since an
/// undecodable slot on an aligned architecture is one instruction's worth of
/// bytes — clipped to the region's end and never zero.
fn recovery_span(addr: u64, grid: u64, end: Option<u64>) -> u64 {
    let span = if grid > 1 { grid - addr % grid } else { 1 };
    match end {
        Some(end) => span.min(end.saturating_sub(addr)).max(1),
        None => span,
    }
}

/// The operand of a `.byte` row: every byte it covers, comma-separated in the
/// spelling an assembler takes back.
fn bad_byte_operand(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("0x{b:02x}")).collect::<Vec<_>>().join(",")
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
    let stop = region.stop();
    let mut rows: Vec<DataRow> = Vec::new();
    let mut truncated = false;
    let mut addr = region.start;
    let mut raw: Vec<u8> = Vec::new();
    loop {
        if count.is_some_and(|n| rows.len() >= n) {
            break;
        }
        if stop.is_some_and(|end| addr >= end) {
            break;
        }
        if cap.is_some_and(|c| rows.len() >= c) {
            truncated = true;
            break;
        }
        let want = match stop {
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
        return Ok(bound_to_image(
            prog,
            Region {
                start,
                end: Some(end),
                name: name_at(prog, start),
                derived: false,
                from_entry: false,
                mapped_end: None,
            },
        ));
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

    let region = match (args.bytes, args.count) {
        (Some(n), _) => Region {
            start,
            end: Some(start.saturating_add(n)),
            name,
            derived: false,
            from_entry,
            mapped_end: None,
        },
        (None, Some(_)) => {
            Region { start, end: None, name, derived: false, from_entry, mapped_end: None }
        }
        (None, None) => {
            let extent = prog.function_extent_at(start);
            let span = if extent > 0 { extent } else { DEFAULT_WINDOW_BYTES };
            Region {
                start,
                end: Some(start.saturating_add(span)),
                name,
                derived: true,
                from_entry,
                mapped_end: None,
            }
        }
    };
    Ok(bound_to_image(prog, region))
}

/// (kuna, `disassembly-fabricates-zero-byte`) Record the end of mapped memory
/// on a resolved region, so no walk over it can list a byte the image does not
/// hold ([`mapped_run_end`]).
fn bound_to_image(prog: &ConsoleProgram, region: Region) -> Region {
    let mapped_end = mapped_run_end(prog, region.start).map(|(end, _)| end);
    Region { mapped_end, ..region }
}

/// Did the walk stop at the end of mapped memory with the caller's own request
/// still unmet? Only then is the image the reason, and only then does saying so
/// tell the caller something they could not read off the answer.
fn cut_short_by_image(region: &Region, count: Option<usize>, listed: usize, end: u64) -> bool {
    region.reaches_past_image()
        && region.mapped_end == Some(end)
        && count.is_none_or(|n| listed < n)
}

/// Say that the image, not the ask, is what ended the listing — the fact an
/// agent cannot infer from a short answer, and the one that says `--count` will
/// not buy more.
fn image_bound_note(prog: &ConsoleProgram, region: &Region) -> String {
    let mapped_end = region.mapped_end.unwrap_or(region.start);
    let next = mapped_run_end(prog, region.start).and_then(|(_, next)| next);
    let resume = match next {
        Some(addr) => format!(", and the next mapped address is 0x{addr:x}"),
        None => String::new(),
    };
    format!(
        "the listing stops at 0x{mapped_end:x}, where the segment holding \
         0x{:x} ends -- the bytes above it are not in the image{resume}",
        region.start
    )
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
    let region = bound_to_image(
        prog,
        Region { start, end: Some(end), name: None, derived: false, from_entry: true, mapped_end: None },
    );
    let CodeWalk { rows, .. } = walk(prog, &region, None, false);
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

/// (kuna, `linear-disassembly-silently-skips`) Say that the straight line walked
/// OVER an address this range's own branches name — the one fact an agent
/// reading a listing cannot infer, because a decode that is a byte out of phase
/// is spelled exactly like one that is not.
///
/// It is not a missing row that matters. The instruction covering the target,
/// and every row after it until the decode re-synchronizes, are a reading of
/// bytes the program never executes that way: on the witness the listing prints
/// a `CALL` and a `JMP` to an address outside the image, neither of which is in
/// the bytes. So the note leads with what is wrong with the rows that ARE
/// there, and names the move that fixes them.
fn skipped_note(skipped: &[u64], followed: bool) -> String {
    let shown: Vec<String> =
        skipped.iter().take(SKIPPED_NOTE_ADDRS).map(|a| format!("0x{a:x}")).collect();
    let more = match skipped.len() - shown.len() {
        0 => String::new(),
        n => format!(" and {n} more"),
    };
    let (count, subject) = match skipped.len() {
        1 => ("an address".to_string(), "it"),
        n => (format!("{n} addresses"), "them"),
    };
    // Under --follow whatever is left is out of the window's reach -- named by
    // bytes no flow arrives at, or sitting at the very edge of the extent -- so
    // re-running the same command is not the move.
    let fix = if followed {
        "--follow could not re-anchor these, so disassemble one of them on its own to \
         decode from it"
    } else {
        "re-run with --follow to decode from those addresses too"
    };
    format!(
        "the decode ran across {count} this range's own branches name -- {}{more} -- so no \
         row starts at {subject}; the instruction printed over each one, and the rows after \
         it until the decode re-synchronizes, spell bytes the program never executes that \
         way -- {fix}",
        shown.join(", ")
    )
}

/// (kuna, `linear-disassembly-silently-skips`) Say how much of the listing
/// `--follow` re-anchored, so a caller comparing two runs of the same range
/// knows how many rows moved and why.
fn follow_note(anchored: usize) -> String {
    let n = match anchored {
        1 => "one address".to_string(),
        n => format!("{n} addresses"),
    };
    format!(
        "--follow decoded from {n} the straight line would have walked over; bytes no \
         flow reaches are still listed, straight-line, in between"
    )
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
    let mut follow = false;
    let mut json = false;
    let mut options: Vec<(String, String)> = Vec::new();
    let mut func_decls: Vec<crate::funcdecl::FuncDecl> = Vec::new();
    let mut mode: Option<String> = None;
    let mut slice: Option<String> = None;
    let mut target: Option<String> = None;
    let mut sleighpath: Option<String> = None;
    let mut isa = None;

    let mut i = 0;
    while i < argv.len() {
        let a = argv[i].as_str();
        match a {
            "--addr" => by_address = true,
            "--as" => view = Some(parse_view(&take(argv, &mut i, "--as")?)?),
            "--json" => json = true,
            "--follow" => follow = true,
            "--count" => count = Some(parse_positive(&take(argv, &mut i, a)?, a)? as usize),
            "--bytes" => bytes = Some(parse_positive(&take(argv, &mut i, a)?, a)?),
            "--option" => {
                if i + 2 >= argv.len() {
                    return Err("--option requires NAME VALUE".into());
                }
                crate::optname::check(&argv[i + 1])?;
                options.push((argv[i + 1].clone(), argv[i + 2].clone()));
                i += 2;
            }
            "--mode" => mode = Some(take(argv, &mut i, "--mode")?),
            "--define-function" => {
                let v = take(argv, &mut i, "--define-function")?;
                func_decls.extend(crate::funcdecl::parse_flag(&v)?);
            }
            "--isa" => isa = kuna_console::engine::ArmIsa::parse(&take(argv, &mut i, "--isa")?)?,
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
        follow,
        json,
        options,
        func_decls,
        mode,
        slice,
        target,
        sleighpath,
        isa,
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
         \x20                    [--as code|data|auto] [--count N] [--bytes N] [--follow] [--json] \\\n\
         \x20                    [--mode auto|reliable|aggressive|fast] \\\n\
         \x20                    [--define-function S[-E][=N]|@FILE].. \\\n\
         \x20                    [--option N V].. [--isa auto|arm|thumb] [--slice ARCH] [--target T] [--sleighpath D]\n\
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
         --follow decodes from every address the range's own branches name, not just\n\
         from its start, so a jump over a decoy byte cannot pull the rest of the\n\
         listing out of phase. Bytes no flow reaches are still listed in between.\n\
         Without it, such an address is reported in notes rather than decoded.\n\
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

    /// A byte that will not decode only re-aligns the listing on an
    /// architecture that has a grid to re-align to. `witness` is the OR of the
    /// decoded rows' addresses and sizes.
    /// The predicate the diagnostic rests on: an in-range target no row starts
    /// at. A target that starts a row is fine, and one outside the listed span
    /// says nothing about the span.
    #[test]
    fn a_skipped_target_is_one_inside_the_listing_that_no_row_starts_at() {
        let rows = vec![
            row(0x1000, &[0xeb, 0x04], "JMP", "0x1006"),
            row(0x1002, &[0x90], "NOP", ""),
            row(0x1003, &[0xc2, 0xac, 0xeb], "RET", "0xebac"),
            row(0x1006, &[0x90], "NOP", ""),
        ];
        let mut refs = FixedRefs::default();
        // 0x1004 is covered by the RET; 0x1006 starts a row; 0x2000 is out of
        // the span; 0x1000 is the listing's own first row.
        refs.flow_targets = vec![0x1004, 0x1006, 0x2000, 0x1000, 0x1004];
        assert_eq!(skipped_targets(&rows, &refs), vec![0x1004]);
        assert!(skipped_targets(&[], &refs).is_empty());
    }

    /// The note names the addresses, counts what it cannot name, and says which
    /// move fixes it — a different move once `--follow` is already on.
    #[test]
    fn the_skipped_note_names_the_addresses_and_the_move_that_fixes_them() {
        let one = skipped_note(&[0x43d092], false);
        assert!(one.contains("an address"), "{one}");
        assert!(one.contains("0x43d092"), "{one}");
        assert!(one.contains("--follow"), "{one}");

        let three = skipped_note(&[0x43d092, 0x43d0ae, 0x43d0c1], false);
        assert!(three.contains("3 addresses"), "{three}");
        assert!(three.contains("0x43d092, 0x43d0ae, 0x43d0c1"), "{three}");

        let many: Vec<u64> = (0..SKIPPED_NOTE_ADDRS as u64 + 3).map(|i| 0x1000 + i).collect();
        let note = skipped_note(&many, false);
        assert!(note.contains("and 3 more"), "{note}");
        assert!(!note.contains("0x1006"), "the seventh must not be spelled out: {note}");

        let followed = skipped_note(&[0x43d092], true);
        assert!(!followed.contains("re-run with --follow"), "already on: {followed}");
        assert!(followed.contains("on its own"), "{followed}");
    }

    #[test]
    fn the_resume_grid_is_the_alignment_the_decoded_rows_share() {
        // ARM: 4-byte rows on 4-byte addresses.
        assert_eq!(resume_grid(2, 0x5c0 | 0x5c4 | 4), 4);
        // Thumb: one 2-byte row is enough to say the grid is 2.
        assert_eq!(resume_grid(2, 0x5c0 | 0x5c2 | 2), 2);
        // A variable-length architecture keeps its byte-at-a-time recovery
        // however aligned the rows it happens to have decoded are.
        assert_eq!(resume_grid(1, 0x401000 | 4), 1);
        // Nothing decoded yet: no grid to infer.
        assert_eq!(resume_grid(4, 0), 1);
        // An odd row address says this listing is not on a grid at all.
        assert_eq!(resume_grid(2, 0x695 | 4), 1);
        // A short listing whose rows happen to share more alignment than any
        // instruction set has is clamped.
        assert_eq!(resume_grid(2, 0x10000), MAX_RESUME_GRID);
    }

    /// The row spans TO the boundary, so the next row starts on it — a
    /// one-byte-at-a-time recovery would leave a row at the unaligned address.
    #[test]
    fn a_recovery_row_spans_to_the_next_grid_boundary() {
        assert_eq!(recovery_span(0x695, 4, None), 3);
        // Already on the grid: an undecodable slot is one instruction wide.
        assert_eq!(recovery_span(0x694, 4, None), 4);
        // No grid: one byte, which is what every variable-length listing gets.
        assert_eq!(recovery_span(0x694, 1, None), 1);
        // The region's end clips the span, and never to zero.
        assert_eq!(recovery_span(0x6a0, 4, Some(0x6a2)), 2);
        assert_eq!(recovery_span(0x6a0, 4, Some(0x6a4)), 4);
        assert_eq!(recovery_span(0x6a0, 4, Some(0x6a0)), 1);
    }

    #[test]
    fn a_byte_row_spells_every_byte_it_covers() {
        assert_eq!(bad_byte_operand(&[0xb8]), "0xb8");
        assert_eq!(bad_byte_operand(&[0xb8, 0xfe, 0xff, 0xff]), "0xb8,0xfe,0xff,0xff");
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
            mapped_end: None,
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
            mapped_end: None,
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

    fn dis_args(spec: &str, count: Option<usize>, bytes: Option<u64>) -> DisArgs {
        DisArgs {
            binary: "bin".into(),
            spec: spec.into(),
            by_address: false,
            view: None,
            count,
            bytes,
            follow: false,
            json: false,
            options: Vec::new(),
            func_decls: Vec::new(),
            mode: None,
            slice: None,
            target: None,
            sleighpath: None,
            isa: None,
        }
    }

    #[test]
    fn a_window_is_the_callers_when_count_bytes_or_a_range_bounds_it() {
        for (spec, count, bytes, expect) in [
            ("main", Some(8), None, true),
            ("main", None, Some(64), true),
            ("0x401000", Some(1), None, true),
            ("0x401000-0x401040", None, None, true),
            ("0x401000..0x401040", None, None, true),
            // Nothing bounds these but the inventory's own function extent.
            ("main", None, None, false),
            ("0x401000", None, None, false),
        ] {
            assert_eq!(
                window_is_caller_bounded(&dis_args(spec, count, bytes)),
                expect,
                "{spec:?} / count {count:?} / bytes {bytes:?}"
            );
        }
    }

    /// The image bound is the end of the run holding the start, and adjacent or
    /// overlapping segments are one run: a listing that crosses from one
    /// `PT_LOAD` into the next at the byte the first ends has crossed no hole.
    #[test]
    fn the_image_bound_is_the_end_of_the_run_holding_the_address() {
        let code = section_flags::CODE;
        let data = section_flags::DATA;
        // The witness's own layout: R E [0x8048000,0x80d1904), a hole, then RW.
        let keygenme = [(0x8048000, 0x89904, code), (0x80d2f50, 0x7a9540, data)];
        assert_eq!(mapped_run(&keygenme, 0x80d18b0), Some((0x80d1904, Some(0x80d2f50))));
        assert_eq!(mapped_run(&keygenme, 0x80d2f50), Some((0x887c490, None)));
        // Inside the hole there is no run at all -- the direct query's answer.
        assert_eq!(mapped_run(&keygenme, 0x80d190b), None);
        assert_eq!(mapped_run(&keygenme, 0x8047fff), None);

        // Touching and overlapping segments merge, in either input order.
        let touching = [(0x2000, 0x1000, data), (0x1000, 0x1000, code)];
        assert_eq!(mapped_run(&touching, 0x1800), Some((0x3000, None)));
        let overlapping = [(0x1000, 0x1800, code), (0x2000, 0x1000, data)];
        assert_eq!(mapped_run(&overlapping, 0x1000), Some((0x3000, None)));

        // A zero-size record is not a segment (the `runModel` convention), and
        // a loader that publishes nothing at all is silence, not a bound.
        assert_eq!(mapped_run(&[(0x1000, 0, code)], 0x1000), None);
        assert_eq!(mapped_run(&[], 0x1000), None);
    }

    /// The walk stops at the tighter of the two bounds, and says the image was
    /// the reason only when the image really was.
    #[test]
    fn the_image_bound_clips_the_walk_and_explains_itself_once() {
        let region = |end, mapped_end| Region {
            start: 0x1000,
            end,
            name: None,
            derived: false,
            from_entry: false,
            mapped_end,
        };
        assert_eq!(region(Some(0x1100), Some(0x1040)).stop(), Some(0x1040));
        assert_eq!(region(Some(0x1020), Some(0x1040)).stop(), Some(0x1020));
        assert_eq!(region(None, Some(0x1040)).stop(), Some(0x1040));
        assert_eq!(region(Some(0x1100), None).stop(), Some(0x1100));
        assert_eq!(region(None, None).stop(), None);

        // A `--count` walk has no byte stop, so it always reaches past.
        assert!(region(None, Some(0x1040)).reaches_past_image());
        assert!(region(Some(0x1100), Some(0x1040)).reaches_past_image());
        assert!(!region(Some(0x1040), Some(0x1040)).reaches_past_image());
        assert!(!region(Some(0x1100), None).reaches_past_image());

        // The note is for a walk the image cut short: it stopped on the bound
        // with the caller's count unmet.
        assert!(cut_short_by_image(&region(None, Some(0x1040)), Some(30), 9, 0x1040));
        // Stopped short of the bound -- something else ended it.
        assert!(!cut_short_by_image(&region(None, Some(0x1040)), Some(30), 9, 0x1030));
        // The count was met exactly on the bound: the ask ended it, not the image.
        assert!(!cut_short_by_image(&region(None, Some(0x1040)), Some(9), 9, 0x1040));
        // The caller asked for exactly the mapped bytes and got them.
        assert!(!cut_short_by_image(&region(Some(0x1040), Some(0x1040)), None, 4, 0x1040));
    }

    #[test]
    fn the_windowed_load_turns_the_discovery_walk_off_unless_the_caller_named_it() {
        let preset: Vec<(String, String)> =
            vec![("listing".into(), "off".into()), ("fast_funcdisc".into(), "on".into())];

        let suppressed = windowed_options(&preset, &[]);
        for name in WINDOW_SUPPRESSED {
            assert_eq!(
                suppressed.iter().filter(|(o, _)| o == name).next_back().map(|(_, v)| v.as_str()),
                Some("off"),
                "{name} must end the list off"
            );
        }

        // The caller's own `--option` is the last word: naming one skips it.
        let named: Vec<(String, String)> = vec![("fast_funcdisc".into(), "on".into())];
        let mut merged = preset.clone();
        merged.extend(named.clone());
        let kept = windowed_options(&merged, &named);
        assert_eq!(
            kept.iter().filter(|(o, _)| o == "fast_funcdisc").next_back().map(|(_, v)| v.as_str()),
            Some("on")
        );
        assert_eq!(
            kept.iter().filter(|(o, _)| o == "listing").next_back().map(|(_, v)| v.as_str()),
            Some("off")
        );
    }

    #[test]
    fn the_windowed_answer_stands_only_when_the_walk_could_not_have_changed_it() {
        let data = section_flags::DATA | section_flags::READONLY;
        let code = section_flags::CODE;
        let region = |name: Option<&str>, from_entry: bool| Region {
            start: 0x401000,
            end: Some(0x401040),
            name: name.map(str::to_string),
            derived: false,
            from_entry,
            mapped_end: None,
        };
        use ViewRequest::{Auto, Data as WantData};
        for (name, from_entry, section, want, expect) in [
            // Named from a fact the windowed load already held, in a section
            // whose view does not depend on there being an entry.
            (Some("main"), true, Some(code), Auto, true),
            (Some("sub_401000"), true, Some(data), Auto, true),
            (Some("g_table"), false, Some(code), Auto, true),
            (Some("g_table"), false, None, Auto, true),
            // Only the walk could name this one.
            (None, false, Some(code), Auto, false),
            // A data section renders as bytes here and as instructions if the
            // walk finds an entry, so the answer is not settled yet.
            (Some("g_table"), false, Some(data), Auto, false),
            // ...unless the caller pinned the view, which outranks the entry.
            (Some("g_table"), false, Some(data), WantData, true),
        ] {
            assert_eq!(
                windowed_answer_is_final(&region(name, from_entry), section, want),
                expect,
                "{name:?} / from_entry {from_entry} / section {section:?} / {want:?}"
            );
        }
    }
}
