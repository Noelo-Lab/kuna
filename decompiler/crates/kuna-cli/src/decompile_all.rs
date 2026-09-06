//! `kuna decompile-all` / `kuna functions` — in-process **whole-binary**
//! decompilation.
//!
//! ```text
//!   kuna decompile-all <binary> [--json] [--functions a,b,..] [--addr 0xVMA].. \
//!                       [--no-vars] [--max-fn-seconds N] [--mode MODE] \
//!                       [--option N V].. [TRIAGE] \
//!                       [--slice ARCH] [--target T] [--sleighpath D]
//!   kuna functions <binary> [--json] [--summary] [TRIAGE] [--mode MODE] \
//!                  [--slice ARCH] [--target T] [--sleighpath D]
//!
//!   TRIAGE := [--filter REGEX] [--min-size N] [--max-size N]
//!             [--reachable-from <name|0xaddr>] [--sort addr|size|name] [--limit N]
//! ```
//!
//! # Triage
//!
//! A whole-binary answer is only usable if the caller can narrow it BEFORE it is
//! produced. Unfiltered, a 211 KB PE crackme is 1,150 functions and ~5.9 MB of
//! `--json` — more than an agent's whole context for a question it has not asked
//! yet. The [`Filters`] set is therefore applied to the TARGET LIST, not to the
//! output: `--filter`/`--min-size`/`--max-size`/`--reachable-from` choose which
//! entries exist for this run, `--sort` + `--limit` cap it, and
//! `decompile-all` then decompiles only what survived. Narrowing the run is what
//! makes it cheap; narrowing the output would not.
//!
//! `--reachable-from <name|0xaddr>` is the call-graph question a triaging agent
//! asks first — *what does the entry point actually touch* — and it reuses
//! `kuna xrefs`' own edges ([`kuna_analysis::listing::xrefs`]) rather than a
//! second call-graph implementation.
//!
//! `kuna functions --summary` answers *where do I start* without emitting a
//! function list at all: counts by size bucket, the image entry point, how many
//! functions it reaches, how many have no call site, and the N largest
//! functions. One call, a few hundred bytes.
//!
//! Unlike `kuna decompile` (which spawns a fresh `decomp_dbg` subprocess **per
//! function** — re-parsing the SLEIGH spec and re-running the whole-binary
//! analysis tier every time), this loads and analyzes the binary **once**
//! in-process (`bootstrap_from_object` → `commit_pending_analysis`, i.e. the
//! `load file` + `read symbols` seam), then loops `decompile` + `print C` over
//! every executable entry. The marginal per-function cost drops from a full image load
//! to just the IR build + pipeline — the load-once shape benchmark harnesses
//! (decbench) and an LLM driver need.
//!
//! The per-function decompile runs the *same* step as the console `decompile`
//! command (`IfcDecompile`) — one shared
//! `kuna_console::decompile_step::decompile_one`, so the two surfaces cannot
//! drift again (DIV-66; they had, and this one was the weaker). It re-seeds the
//! function's DWARF stack locals via [`ConsoleProgram::dwarf_locals_for`] (so a
//! `-g` binary renders DWARF names), and a per-function pipeline abort (the
//! decompile drive already catches panics / un-ported seams and returns `Err`) is
//! recorded as that function's `error` rather than aborting the whole binary.
//!
//! `--json` emits a machine-readable object (the decbench / LLM surface); without
//! it the command prints concatenated C with `// Function: <name> @ <addr>`
//! headers (the human surface).
//!
//! Omitted `--mode` resolves the size-based `auto` policy. Under its concrete
//! `reliable` preset, the decompile surface injects `--option listing on`
//! (decbench F1, DIV-15) unless the caller names `listing`, so the default-on
//! `noreturn_propagate` consumer fires and an unnamed internal exit/fatal
//! wrapper cannot swallow following functions. `aggressive` names Listing on;
//! `fast` names it off. A later explicit `--option` always wins.
//!
//! Both surfaces share ONE discovery policy ([`DriverDefaults`], DIV-68): the
//! non-x86-64 `funcstart_patterns` + `aif` defaults (DIV-20) and the Listing that
//! gates them are injected for `functions` exactly as for `decompile-all`, so the
//! inventory can never omit an entry the whole-binary run decompiles.
//!
//! An unfiltered run that discovers ZERO functions **fails** — non-zero exit, the
//! reason on stderr and in the document's run-level `error` field — because a
//! silent `count: 0` is indistinguishable from a file that genuinely has no
//! functions, and the caller acts on the difference. [`zero_discovery_error`]
//! draws the line at executable content, so a data-only object still answers
//! with an honest empty inventory and exit 0.
//!
//! [`render_result_json`] and [`decompile_entries`] are also `kuna decompile
//! --json`'s (`decompile.rs`) — one schema and one decompile policy across the
//! single-function and whole-binary surfaces.

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::ffi::{OsStr, OsString};
use std::fmt::Write as _;
use std::rc::Rc;

// The call-graph edges `--reachable-from` walks are `kuna xrefs`' own edges.
use kuna_analysis::listing::xrefs::{Xref, XrefIndex, XrefKind};
use kuna_base::address::Address;
use kuna_console::engine::{
    bootstrap_from_object, ConsoleProgram, EntryLookupError, EntrySelector, FunctionEntry,
    ObjectLocation,
};
// The decompile loop + result shape live in the shared decompile-project core
// (`kuna_console::project` — also reused by the `kuna_wasm` front-end).
use kuna_console::project::{
    decompile_targets, default_fn_budget_seconds, render_c, FuncResult,
};
// `File::architecture()` (the ARM-discovery default, decbench) plus the
// section/segment walks the zero-discovery diagnosis reads.
use object::{Object, ObjectSection, ObjectSegment};
use kuna_decomp::decompile_drive::{LineMapping, VarInfo};
use kuna_decomp::options::{OptionDatabase, KUNA_OPTION_NAMES, RELOC_OBJECTS_ENV};

use regex::Regex;

use crate::jsonfmt::{dumps_indent2, Json};
use crate::paths;

/// Parsed `decompile-all` / `functions` arguments (the two share a loader;
/// `decompile-project` reuses the same parse via its own wrapper).
pub(crate) struct Args {
    pub(crate) binary: String,
    pub(crate) json: bool,
    /// `--functions a,b,c`: restrict to these names (None ⇒ CODE-backed entries).
    pub(crate) names: Option<Vec<String>>,
    /// `--addr 0xVMA|.section+0xOFFSET|SECTION_INDEX:0xOFFSET` (repeatable).
    /// Combined with `--functions` if both are given.
    pub(crate) addrs: Vec<EntrySelector>,
    /// `--no-vars`: skip the per-function variable extraction (faster; drops the
    /// `variables` array used by decbench's `type_match`).
    pub(crate) no_vars: bool,
    /// `--max-fn-seconds N` (decompile-all / decompile-project): per-function
    /// decompile watchdog budget in seconds; `0` disables.  A function that
    /// exceeds it is recorded as that function's `error` (the batch continues)
    /// instead of hanging the whole run — the defensive cap for the known
    /// stripped-ELF non-convergence hang (`tests/hang-repro/`). Defaults to 10
    /// for an unfiltered fast whole-binary run and 120 otherwise.
    pub(crate) max_fn_seconds: u64,
    pub(crate) options: Vec<(String, String)>,
    /// `--define-function <start[-end][=name] | @file>` (repeatable): the
    /// caller-declared function boundaries, applied by [`load_program`] right
    /// after the analysis commit so they outrank discovery. Every surface that
    /// loads through this struct honors them; only the surfaces that parse the
    /// flag can be non-empty.
    pub(crate) func_decls: Vec<crate::funcdecl::FuncDecl>,
    /// `--assert <directive> | @FILE` (repeatable): the caller-supplied
    /// assertions, installed by [`load_program`] right after the analysis commit
    /// and dispatched by the decompile loop (`kuna_console::assertions`). Empty
    /// for every invocation that passed none.
    pub(crate) assertions: Vec<kuna_console::assertions::Directive>,
    /// `--assert-strict`: a rejected directive makes the run exit non-zero
    /// instead of being reported and continuing.
    pub(crate) assert_strict: bool,
    pub(crate) slice: Option<String>,
    pub(crate) target: Option<String>,
    pub(crate) sleighpath: Option<String>,
}

// --- triage: narrowing a whole-binary run before it runs ---------------------

/// How a narrowed inventory is ordered (`--sort`).
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub(crate) enum SortKey {
    /// Entry address, ascending — discovery order, and the historical default of
    /// both surfaces.
    #[default]
    Addr,
    /// Byte extent, DESCENDING. The triage question is "what is big here", so
    /// the answer leads with the biggest rather than making the caller reverse
    /// a list it may have already truncated with `--limit`.
    Size,
    /// Name, ascending.
    Name,
}

impl SortKey {
    fn parse(value: &str) -> Result<SortKey, String> {
        match value.trim().to_ascii_lowercase().as_str() {
            "addr" | "address" => Ok(SortKey::Addr),
            "size" => Ok(SortKey::Size),
            "name" => Ok(SortKey::Name),
            other => Err(format!(
                "unknown --sort key {other:?} (expected addr, size, or name)"
            )),
        }
    }
}

/// The size histogram `--summary` reports, as `(label, min, max)` inclusive.
///
/// Log-ish rather than uniform because a real inventory is: the zero-extent
/// import slots, a mass of thunks and tiny wrappers, and a short tail of large
/// functions — and the tail is what a triaging agent is looking for.
const SIZE_BUCKETS: [(&str, u64, u64); 7] = [
    ("0", 0, 0),
    ("1-15", 1, 15),
    ("16-63", 16, 63),
    ("64-255", 64, 255),
    ("256-1023", 256, 1023),
    ("1024-4095", 1024, 4095),
    ("4096+", 4096, u64::MAX),
];

/// How many functions `--summary` lists under `largest` when `--limit` is absent.
const DEFAULT_SUMMARY_LARGEST: usize = 10;

/// The triage narrowing shared by `kuna functions` and `kuna decompile-all`.
///
/// Deliberately NOT a field of [`Args`]: `Args` is a struct literal in four other
/// command modules, and growing it would break every one of them for a selection
/// none of them make. [`parse_args_with_filters`] returns the pair; the plain
/// [`parse_args`] every other surface calls drops this half.
#[derive(Default)]
pub(crate) struct Filters {
    /// `--filter REGEX`, matched against the canonical name and every alias.
    name: Option<Regex>,
    /// `--min-size N` / `--max-size N`, inclusive, over the inventory `size`.
    min_size: Option<u64>,
    max_size: Option<u64>,
    /// `--reachable-from <name|0xaddr>`, resolved against the loaded program.
    reachable_from: Option<String>,
    /// `--limit N`, applied AFTER `--sort`.
    limit: Option<usize>,
    sort: SortKey,
    /// `--summary`: report the orientation document instead of the inventory.
    pub(crate) summary: bool,
}

impl Filters {
    /// Does this selection narrow the run at all?  An unnarrowed run keeps every
    /// pre-existing behaviour, byte for byte — including the zero-discovery
    /// verdict, which must stay attached to DISCOVERY and never fire because a
    /// filter matched nothing.
    pub(crate) fn narrows(&self) -> bool {
        self.name.is_some()
            || self.min_size.is_some()
            || self.max_size.is_some()
            || self.reachable_from.is_some()
            || self.limit.is_some()
            || self.sort != SortKey::Addr
    }

    /// Narrow, order and cap `entries`, returning the call graph if one had to be
    /// built (the summary reuses it rather than walking the image twice).
    ///
    /// The graph is built only when `--reachable-from` or `--summary` needs it,
    /// so `--filter`/`--min-size`/`--max-size`/`--limit` stay pure inventory
    /// arithmetic with no extra decode.
    pub(crate) fn select(
        &self,
        prog: &ConsoleProgram,
        binary: &str,
        entries: Vec<FunctionEntry>,
    ) -> Result<(Vec<FunctionEntry>, Option<CallGraph>), String> {
        let graph = (self.reachable_from.is_some() || self.summary)
            .then(|| CallGraph::build(prog, binary))
            .transpose()?;
        let reachable = match (&self.reachable_from, &graph) {
            (Some(spec), Some(graph)) => Some(graph.reachable_from(prog, spec)?),
            _ => None,
        };
        let mut kept: Vec<FunctionEntry> = entries
            .into_iter()
            .filter(|e| self.keeps(e, reachable.as_ref()))
            .collect();
        self.order(&mut kept);
        if let Some(limit) = self.limit {
            kept.truncate(limit);
        }
        Ok((kept, graph))
    }

    fn keeps(&self, e: &FunctionEntry, reachable: Option<&BTreeSet<u64>>) -> bool {
        if let Some(re) = &self.name {
            if !re.is_match(&e.name) && !e.aliases.iter().any(|a| re.is_match(a)) {
                return false;
            }
        }
        if self.min_size.is_some_and(|min| e.size < min) {
            return false;
        }
        if self.max_size.is_some_and(|max| e.size > max) {
            return false;
        }
        if reachable.is_some_and(|set| !set.contains(&e.addr.get_offset())) {
            return false;
        }
        true
    }

    /// Order in place. Every key breaks ties on the entry address, so a narrowed
    /// run is reproducible rather than dependent on the discovery order.
    fn order(&self, entries: &mut [FunctionEntry]) {
        match self.sort {
            SortKey::Addr => entries.sort_by_key(|e| e.addr.get_offset()),
            SortKey::Size => entries.sort_by(|a, b| {
                b.size.cmp(&a.size).then(a.addr.get_offset().cmp(&b.addr.get_offset()))
            }),
            SortKey::Name => entries.sort_by(|a, b| {
                a.name.cmp(&b.name).then(a.addr.get_offset().cmp(&b.addr.get_offset()))
            }),
        }
    }

    /// How many functions `--summary` lists under `largest`.
    fn largest_wanted(&self) -> usize {
        self.limit.unwrap_or(DEFAULT_SUMMARY_LARGEST)
    }
}

/// The program's call graph, read out of the same reference edges `kuna xrefs`
/// answers with ([`kuna_analysis::listing::xrefs`]) — one edge model for the
/// whole CLI, not a second one that could disagree with it.
///
/// The walk is seeded with kuna's own canonical inventory, so it explores the
/// call graph out of every discovered entry rather than only out of the image
/// entry point.
pub(crate) struct CallGraph {
    index: XrefIndex,
    /// Canonical `(entry, extent)` pairs, ascending by entry — the containment
    /// ladder that maps a reached address back onto the inventory record owning
    /// it.
    entries: Vec<(u64, u64)>,
}

impl CallGraph {
    pub(crate) fn build(prog: &ConsoleProgram, binary: &str) -> Result<CallGraph, String> {
        let bytes = kuna_analysis::loader::elf_shdr::read_image(binary)
            .map_err(|e| format!("{binary}: {e}"))?;
        let file = object::File::parse(&*bytes)
            .map_err(|e| format!("could not parse {binary}: {e}"))?;
        Ok(CallGraph::build_from(prog, &file))
    }

    /// [`Self::build`] off an already-parsed image, for a caller that holds one.
    pub(crate) fn build_from(prog: &ConsoleProgram, file: &object::File) -> CallGraph {
        let mut entries: Vec<(u64, u64)> = prog
            .function_entries_canonical()
            .iter()
            .map(|e| (e.addr.get_offset(), e.size))
            .collect();
        entries.sort_unstable();
        entries.dedup_by_key(|(addr, _)| *addr);
        let seeds: Vec<u64> = entries.iter().map(|(addr, _)| *addr).collect();
        let index = kuna_analysis::listing::xrefs::build(
            file,
            prog.arch(),
            prog.arch().translate(),
            &seeds,
        );
        CallGraph { index, entries }
    }

    /// Every inventory entry reachable from `spec` through call, tail-jump, and
    /// address-taken-function-pointer edges, `spec`'s own function included.
    ///
    /// Function pointers count: a callback registered with `CreateThread` or an
    /// atexit handler is code the named function reaches, and dropping it would
    /// under-report exactly the indirection an obfuscated crackme leans on. A
    /// materialized address that does NOT land on a known function entry is not
    /// an edge (that is a string or a global, not a callee).
    fn reachable_from(
        &self,
        prog: &ConsoleProgram,
        spec: &str,
    ) -> Result<BTreeSet<u64>, String> {
        let start = resolve_function_spec(prog, spec)?;
        let seed = self.node_at(start).ok_or_else(|| {
            format!("--reachable-from {spec:?}: 0x{start:x} is in no discovered function")
        })?;

        let mut reached: BTreeSet<u64> = BTreeSet::new();
        let mut queue: VecDeque<u64> = VecDeque::new();
        reached.insert(seed);
        queue.push_back(seed);
        while let Some(node) = queue.pop_front() {
            for r in self.index.refs_from_function(node) {
                if let Some(callee) = self.callee_of(r) {
                    if reached.insert(callee) {
                        queue.push_back(callee);
                    }
                }
            }
        }

        // The walk's function set and kuna's inventory are two views of the same
        // program and need not agree entry-for-entry, so fold every reached
        // address onto the inventory record that contains it before answering.
        let mut owners: BTreeSet<u64> = BTreeSet::new();
        for vma in reached {
            owners.insert(vma);
            if let Some(owner) = self.owner_of(vma) {
                owners.insert(owner);
            }
        }
        Ok(owners)
    }

    /// Every callee of the function entered at `entry`, as
    /// `(callee entry, edge kind)` in reference order — the same edge rule
    /// [`Self::reachable_from`] walks ([`Self::callee_of`]), answered one
    /// function at a time rather than as a transitive closure. Address-taken
    /// callees are therefore edges too: without them `main` has no caller in a
    /// glibc program, because `_start` hands it to `__libc_start_main` as a
    /// pointer.
    ///
    /// A callee is always a node the graph knows, named in the inventory's terms
    /// rather than the walk's ([`Self::owner_of`], the fold `reachable_from`
    /// does once at the end of its search): a reference into the middle of a
    /// body resolves to the body, and one that lands in no discovered function
    /// at all — a `CALL 0x0` off a nulled relocation, a branch into a gap, a
    /// materialized address that is a string — is not an edge and is dropped.
    ///
    /// Duplicates collapse on the callee at the first reference's position,
    /// carrying the strongest kind the caller uses: calling a function is a
    /// stronger claim than jumping to it, and both are stronger than mentioning
    /// its address.
    pub(crate) fn callees_of(&self, entry: u64) -> Vec<(u64, XrefKind)> {
        let mut out: Vec<(u64, XrefKind)> = Vec::new();
        let mut seen: BTreeMap<u64, usize> = BTreeMap::new();
        let mut refs: Vec<&Xref> = self.index.refs_from_function(entry);
        refs.sort_by_key(|r| (r.from, r.to));
        for r in refs {
            let Some(callee) = self.callee_of(r).and_then(|c| self.owner_of(c)) else {
                continue;
            };
            // A jump back onto the caller is a loop edge inside one body reached
            // through a second entry symbol, not a call.
            if callee == entry && r.kind == XrefKind::Jump {
                continue;
            }
            match seen.get(&callee) {
                Some(&at) if edge_rank(r.kind) < edge_rank(out[at].1) => out[at].1 = r.kind,
                Some(_) => {}
                None => {
                    seen.insert(callee, out.len());
                    out.push((callee, r.kind));
                }
            }
        }
        out
    }

    /// The call-graph node a reference edge lands on, or `None` when the edge is
    /// not a call-graph edge at all.
    fn callee_of(&self, r: &Xref) -> Option<u64> {
        match r.kind {
            XrefKind::Call | XrefKind::Jump => self.node_at(r.to),
            XrefKind::Data => self.index.is_function_entry(r.to).then_some(r.to),
            XrefKind::Read | XrefKind::Write => None,
        }
    }

    /// The walk's function entry for `vma`: the entry itself, else the function
    /// containing it (a branch into the middle of a body is still that body).
    fn node_at(&self, vma: u64) -> Option<u64> {
        if self.index.is_function_entry(vma) {
            return Some(vma);
        }
        self.index.function_containing(vma)
    }

    /// The inventory entry whose extent contains `vma`.
    ///
    /// Bounded by the extent rather than just "the greatest entry at or below",
    /// so an address in a gap between functions is attributed to neither.
    fn owner_of(&self, vma: u64) -> Option<u64> {
        let at = self.entries.partition_point(|(addr, _)| *addr <= vma);
        let (addr, size) = *self.entries.get(at.checked_sub(1)?)?;
        (vma == addr || vma - addr < size).then_some(addr)
    }

    /// Does the function entered at `entry` make a computed call?
    ///
    /// A caller reading [`Self::callees_of`] needs this to tell "no callees"
    /// from "the callee is computed at run time and has no static target".
    pub(crate) fn has_indirect_calls(&self, entry: u64) -> bool {
        self.index.has_indirect_calls(entry)
    }

    /// The fixed pointer slot a forwarding veneer at `entry` jumps through
    /// (`jmp [slot]`), or `None` when `entry` is not one.
    pub(crate) fn veneer_slot(&self, entry: u64) -> Option<u64> {
        self.index.veneer_slot(entry)
    }

    /// Does anything CALL `vma`?  Data and branch references do not count: the
    /// question a triaging agent asks is which functions no call site reaches.
    fn has_caller(&self, vma: u64) -> bool {
        self.index
            .refs_to(vma)
            .iter()
            .any(|r| r.kind == XrefKind::Call)
    }
}

/// How strong a claim one reference kind makes about a call-graph edge, lowest
/// first: a call, then a tail jump, then a materialized address.
fn edge_rank(kind: XrefKind) -> u8 {
    match kind {
        XrefKind::Call => 0,
        XrefKind::Jump => 1,
        _ => 2,
    }
}

/// Resolve a `<name|0xaddr>` operand against the loaded program.
///
/// A name is looked up FIRST and a bare-hex reading is only the fallback, so a
/// function genuinely called `abc` is not silently read as `0xabc` — the same
/// order `kuna xrefs` resolves its `--to`/`--from` operand in. An address is
/// resolved THROUGH the inventory when it names a known entry, which is what
/// folds the ARM Thumb mode bit out of an odd `--reachable-from 0x3dd`.
fn resolve_function_spec(prog: &ConsoleProgram, spec: &str) -> Result<u64, String> {
    let spec = spec.trim();
    let through_inventory =
        |addr: u64| prog.find_entry_at(addr).map_or(addr, |e| e.addr.get_offset());
    if let Some(body) = spec.strip_prefix("0x").or_else(|| spec.strip_prefix("0X")) {
        return u64::from_str_radix(body, 16)
            .map(through_inventory)
            .map_err(|_| format!("invalid address {spec:?}"));
    }
    if let Some(entry) = prog.find_entry_by_name(spec) {
        return Ok(entry.addr.get_offset());
    }
    if let Some(addr) = prog.lookup_symbol(spec) {
        return Ok(through_inventory(addr.get_offset()));
    }
    u64::from_str_radix(spec, 16)
        .map(through_inventory)
        .map_err(|_| format!("no function named {spec:?} (and it is not an address)"))
}

// --- the `--summary` orientation document ------------------------------------

/// The answer to "where do I start", measured without decompiling anything.
struct Summary {
    /// Functions discovered, before any triage filter.
    total: usize,
    /// The image entry point as `(address, name)`, `None` when the format
    /// declares none.
    entry: Option<(u64, String)>,
    /// How many DISCOVERED functions the entry point reaches.
    reachable_from_entry: Option<usize>,
    /// How many SELECTED functions no call site reaches.
    no_callers: usize,
    /// Total byte extent of the selected functions.
    code_bytes: u64,
    /// `(label, count)` per [`SIZE_BUCKETS`] row, over the selected functions.
    buckets: Vec<(&'static str, usize)>,
    /// The largest selected functions, biggest first.
    largest: Vec<FunctionEntry>,
}

/// Measure the orientation document over `all` (the discovered inventory) and
/// `selected` (what the triage filters kept — the same list when there are none).
fn summarize(
    prog: &ConsoleProgram,
    binary: &str,
    filters: &Filters,
    graph: &CallGraph,
    all: &[FunctionEntry],
    selected: &[FunctionEntry],
) -> Summary {
    let entry = image_entry(prog, binary);
    let reachable_from_entry = entry.as_ref().and_then(|(vma, _)| {
        let reached = graph.reachable_from(prog, &format!("0x{vma:x}")).ok()?;
        Some(all.iter().filter(|e| reached.contains(&e.addr.get_offset())).count())
    });

    let mut buckets: Vec<(&'static str, usize)> =
        SIZE_BUCKETS.iter().map(|(label, _, _)| (*label, 0)).collect();
    for e in selected {
        if let Some(i) = SIZE_BUCKETS.iter().position(|(_, lo, hi)| e.size >= *lo && e.size <= *hi) {
            buckets[i].1 += 1;
        }
    }

    let mut largest = selected.to_vec();
    largest.sort_by(|a, b| {
        b.size.cmp(&a.size).then(a.addr.get_offset().cmp(&b.addr.get_offset()))
    });
    largest.truncate(filters.largest_wanted());

    Summary {
        total: all.len(),
        entry,
        reachable_from_entry,
        no_callers: selected
            .iter()
            .filter(|e| !graph.has_caller(e.addr.get_offset()))
            .count(),
        code_bytes: selected.iter().map(|e| e.size).sum(),
        buckets,
        largest,
    }
}

/// The image's declared entry point, named with kuna's best name for it.
///
/// This is the FORMAT's entry (a PE `AddressOfEntryPoint` is the CRT startup,
/// not `main`) — the one address every image agrees on, and the honest root for
/// "what does this program actually reach". Taken through
/// [`kuna_analysis::analyzers::entry::image_entry_vma`], because a Mach-O
/// `LC_MAIN` states its entry as a `__TEXT`-relative file offset, not a VMA.
fn image_entry(prog: &ConsoleProgram, binary: &str) -> Option<(u64, String)> {
    let bytes = std::fs::read(binary).ok()?;
    let file = object::File::parse(&*bytes).ok()?;
    let vma = kuna_analysis::analyzers::entry::image_entry_vma(&file, &bytes)?;
    // Reported THROUGH the inventory, so an ARM `e_entry` carrying the Thumb mode
    // bit is answered at the even entry the rest of the document uses.
    match prog.find_entry_at(vma) {
        Some(entry) => Some((entry.addr.get_offset(), entry.name)),
        None => Some((vma, prog.function_named_at(vma).unwrap_or_else(|| format!("0x{vma:x}")))),
    }
}

fn summary_json(
    binary: &str,
    summary: &Summary,
    selected: usize,
    error: Option<&str>,
) -> String {
    let entry = match &summary.entry {
        Some((vma, name)) => Json::Object(vec![
            ("name".into(), Json::Str(name.clone())),
            ("address".into(), Json::Number(vma.to_string())),
            ("address_hex".into(), Json::Str(format!("0x{vma:x}"))),
        ]),
        None => Json::Null,
    };
    let buckets = Json::Array(
        summary
            .buckets
            .iter()
            .zip(SIZE_BUCKETS.iter())
            .map(|((label, count), (_, lo, hi))| {
                Json::Object(vec![
                    ("bucket".into(), Json::Str((*label).to_string())),
                    ("min_size".into(), Json::Number(lo.to_string())),
                    (
                        "max_size".into(),
                        if *hi == u64::MAX {
                            Json::Null
                        } else {
                            Json::Number(hi.to_string())
                        },
                    ),
                    ("count".into(), Json::Number(count.to_string())),
                ])
            })
            .collect(),
    );
    format!(
        "{}\n",
        dumps_indent2(&Json::Object(vec![
            ("binary".into(), Json::Str(binary.to_string())),
            ("count".into(), Json::Number(selected.to_string())),
            ("total".into(), Json::Number(summary.total.to_string())),
            ("error".into(), error_json(error)),
            (
                "summary".into(),
                Json::Object(vec![
                    ("entry".into(), entry),
                    (
                        "reachable_from_entry".into(),
                        summary
                            .reachable_from_entry
                            .map(|n| Json::Number(n.to_string()))
                            .unwrap_or(Json::Null),
                    ),
                    ("no_callers".into(), Json::Number(summary.no_callers.to_string())),
                    ("code_bytes".into(), Json::Number(summary.code_bytes.to_string())),
                    ("size_buckets".into(), buckets),
                    ("largest".into(), entries_json(&summary.largest)),
                ])
            ),
        ]))
    )
}

fn summary_text(binary: &str, summary: &Summary, selected: usize) -> String {
    let mut out = String::new();
    let _ = writeln!(out, "binary\t{binary}");
    let _ = writeln!(out, "functions\t{selected} selected / {} discovered", summary.total);
    match &summary.entry {
        Some((vma, name)) => {
            let _ = writeln!(out, "entry\t0x{vma:x}\t{name}");
        }
        None => {
            let _ = writeln!(out, "entry\t(none declared)");
        }
    }
    if let Some(n) = summary.reachable_from_entry {
        let _ = writeln!(out, "reachable from entry\t{n}");
    }
    let _ = writeln!(out, "no callers\t{}", summary.no_callers);
    let _ = writeln!(out, "code bytes\t{}", summary.code_bytes);
    let _ = writeln!(out, "size buckets:");
    for (label, count) in &summary.buckets {
        let _ = writeln!(out, "  {label:<12}\t{count}");
    }
    let _ = writeln!(out, "largest:");
    for e in &summary.largest {
        let _ = writeln!(out, "  0x{:x}\t{}\t{}", e.addr.get_offset(), e.size, e.name);
    }
    out
}

/// `kuna decompile-all` entry point.
pub fn run(argv: &[String]) -> i32 {
    let (args, filters) = match parse_args_with_filters(argv, "decompile-all") {
        Ok(a) => a,
        Err(e) => {
            eprintln!("error: {e}");
            usage_decompile_all();
            return 2;
        }
    };
    // `--summary` is an inventory question, so it never enters the decompile
    // loop: the whole point of asking it is to find out what is worth decompiling.
    if filters.summary {
        return run_summary(&args, &filters);
    }
    match decompile_all(&args, &filters) {
        Ok(run) => {
            // An UNFILTERED run that DISCOVERED nothing is a failed run wearing a
            // successful one's clothes. A run narrowed by `--functions`/`--addr`
            // that matched nothing is a different condition (already warned
            // about, per target) and keeps its status — as is a triage filter
            // that matched nothing, which is an answer, not a failure. So the
            // verdict is read off the pre-filter target set.
            let unfiltered = args.names.is_none() && args.addrs.is_empty();
            let discovery_error = (run.discovered == 0 && unfiltered)
                .then(|| zero_discovery_error(&args.binary))
                .flatten();
            let text = if args.json {
                render_selected_json(
                    &args.binary,
                    &run.funcs,
                    &args.options,
                    discovery_error.as_deref(),
                    filters.narrows().then_some(run.discovered),
                    &run.assertions,
                )
            } else {
                render_c(&run.funcs)
            };
            // A rejected assertion is reported and the run continues (an agent
            // batching forty renames against a re-decompiled binary must not lose
            // all forty to one stale name); `--assert-strict` makes it fatal.
            let rejected = report_rejected_assertions(&run.assertions);
            let refused = any_refused_assertion(&run.assertions);
            let status = emit_with_discovery_error(&text, discovery_error.as_deref());
            if status == 0 && (refused || (args.assert_strict && rejected)) {
                return 1;
            }
            status
        }
        Err(e) => {
            eprintln!("error: {e}");
            1
        }
    }
}

/// Emit `text`, then report a discovery failure (stdout before stderr, as
/// `kuna decompile` orders them) and answer with the run's verdict.
/// Report every rejected assertion on stderr, and say whether there was one.
///
/// The human surface has no `assertions[]` to read, and a directive that was
/// silently dropped is the failure mode this plane exists to end -- so a
/// rejection is always spoken, on both surfaces.
pub(crate) fn report_rejected_assertions(
    outcomes: &[kuna_console::assertions::Outcome],
) -> bool {
    let mut any = false;
    for outcome in outcomes.iter().filter(|o| o.status == "rejected") {
        any = true;
        let reason = outcome.detail.as_deref().unwrap_or("no reason given");
        if outcome.fatal {
            eprintln!(
                "error: --assert {:?} refused by the pipeline: {reason} \
                 (the C below was produced WITHOUT it)",
                outcome.directive
            );
        } else {
            eprintln!("warning: --assert {:?} rejected: {reason}", outcome.directive);
        }
    }
    any
}

/// Did the pipeline REFUSE a directive it had already accepted
/// (`kuna_console::assertions::Outcome::fatal`)?
///
/// That is the verdict of the run whether or not `--assert-strict` was passed:
/// unlike a directive that never bound, the C came back looking healthy while
/// describing a program the directive was never applied to.
pub(crate) fn any_refused_assertion(outcomes: &[kuna_console::assertions::Outcome]) -> bool {
    outcomes.iter().any(|o| o.fatal)
}

fn emit_with_discovery_error(text: &str, discovery_error: Option<&str>) -> i32 {
    let status = crate::output::emit_with_status(text, i32::from(discovery_error.is_some()));
    if let Some(message) = discovery_error {
        eprintln!("error: {message}");
    }
    status
}

/// `kuna functions` entry point (enumeration only — no decompile).
pub fn run_functions(argv: &[String]) -> i32 {
    let (args, filters) = match parse_args_with_filters(argv, "functions") {
        Ok(a) => a,
        Err(e) => {
            eprintln!("error: {e}");
            usage_functions();
            return 2;
        }
    };
    if filters.summary {
        return run_summary(&args, &filters);
    }
    match list_functions(&args) {
        Ok((prog, all)) => {
            // DISCOVERY, not selection, decides the verdict: an empty inventory
            // is a total discovery failure, while a triage filter that matched
            // nothing is an answer.
            let discovery_error = all
                .is_empty()
                .then(|| zero_discovery_error(&args.binary))
                .flatten();
            let total = all.len();
            let entries = match filters.select(&prog, &args.binary, all) {
                Ok((entries, _)) => entries,
                Err(e) => {
                    eprintln!("error: {e}");
                    return 1;
                }
            };
            let text = if args.json {
                functions_json(&args.binary, &entries, total, discovery_error.as_deref())
            } else {
                let mut text = String::new();
                for e in &entries {
                    // Alias names follow the canonical one on the same line, so
                    // the plain listing stays one line per function.
                    let extra = if e.aliases.is_empty() {
                        String::new()
                    } else {
                        format!("\t({})", e.aliases.join(", "))
                    };
                    let _ = writeln!(text, "0x{:x}\t{}{extra}", e.addr.get_offset(), e.name);
                }
                text
            };
            emit_with_discovery_error(&text, discovery_error.as_deref())
        }
        Err(e) => {
            eprintln!("error: {e}");
            1
        }
    }
}

/// `--summary`: the orientation document, on either surface.
///
/// It loads through the INVENTORY driver bundle on both, so the counts a caller
/// orients by are the same numbers `kuna functions` reports, and asking for
/// orientation never pays for a decompile it is trying to avoid.
fn run_summary(args: &Args, filters: &Filters) -> i32 {
    let (prog, all) = match list_functions(args) {
        Ok(loaded) => loaded,
        Err(e) => {
            eprintln!("error: {e}");
            return 1;
        }
    };
    let discovery_error = all
        .is_empty()
        .then(|| zero_discovery_error(&args.binary))
        .flatten();
    let (selected, graph) = match filters.select(&prog, &args.binary, all.clone()) {
        Ok(pair) => pair,
        Err(e) => {
            eprintln!("error: {e}");
            return 1;
        }
    };
    let Some(graph) = graph else {
        eprintln!("error: --summary could not build the program call graph");
        return 1;
    };
    let summary = summarize(&prog, &args.binary, filters, &graph, &all, &selected);
    let text = if args.json {
        summary_json(&args.binary, &summary, selected.len(), discovery_error.as_deref())
    } else {
        summary_text(&args.binary, &summary, selected.len())
    };
    emit_with_discovery_error(&text, discovery_error.as_deref())
}

/// One `decompile-all` run: what it decompiled, and how many entries the
/// selection started from.
///
/// The two are different numbers once a triage filter narrows the run, and the
/// zero-discovery verdict has to be read off the second one.
struct AllRun {
    funcs: Vec<FuncResult>,
    discovered: usize,
    /// One row per `--assert` directive: what became of it.
    assertions: Vec<kuna_console::assertions::Outcome>,
}

/// Load + analyze the binary once, narrow the target list, then decompile what
/// survived.
///
/// The filters run BEFORE the decompile loop on purpose: narrowing the output
/// would still pay for 1,150 decompiles.
fn decompile_all(args: &Args, filters: &Filters) -> Result<AllRun, String> {
    let mut prog = load_program(args, DriverDefaults::Decompile)?;
    let targets = resolve_targets(&prog, args)?;
    let discovered = targets.len();
    let targets = if filters.narrows() {
        filters.select(&prog, &args.binary, targets)?.0
    } else {
        targets
    };
    let funcs = decompile_entries(&mut prog, args, targets);
    Ok(AllRun { funcs, discovered, assertions: prog.assertion_outcomes() })
}

/// Arm the per-function watchdog and run the decompile loop over `targets` — the
/// body `decompile-all` and `kuna decompile --json` share, so the two answer with
/// one policy as well as one schema.
///
/// The watchdog (`--max-fn-seconds`, default 10 for an unfiltered fast batch and
/// 120 otherwise, 0 disables) is driver policy, not a stage-model option: the
/// decompile drive arms a cooperative deadline from this budget for EACH
/// function, so one pathological function becomes a per-function `error` record
/// instead of hanging the whole batch.
pub(crate) fn decompile_entries(
    prog: &mut ConsoleProgram,
    args: &Args,
    targets: Vec<FunctionEntry>,
) -> Vec<FuncResult> {
    if args.max_fn_seconds > 0 {
        prog.arch_mut().kuna_fn_budget =
            Some(std::time::Duration::from_secs(args.max_fn_seconds));
    }

    decompile_targets(
        prog,
        targets,
        args.no_vars,
        /* want_proto= */ false,
        /* want_provenance= */ args.json,
    )
}

/// Enumerate the program's full callable-symbol inventory, one
/// [`FunctionEntry`] per entry address (the `functions` command).
///
/// The loaded program is handed back with it: the triage filters and `--summary`
/// both ask further questions of the same load rather than paying for a second.
fn list_functions(args: &Args) -> Result<(ConsoleProgram, Vec<FunctionEntry>), String> {
    let prog = load_program(args, DriverDefaults::Inventory)?;
    // One record per entry address, address-ordered, alias names carried as data
    // (issue #197 — this used to dedup by (address, name), so a function the
    // loader and an analysis pass both named was listed twice).
    let entries = prog.function_entries_canonical();
    Ok((prog, entries))
}

/// Which bundle of driver defaults a surface takes in [`load_program`].
///
/// Both variants take the same DISCOVERY policy (DIV-20/DIV-68); they differ only
/// in whether the Listing is built on an architecture where it discovers nothing.
pub(crate) enum DriverDefaults {
    /// `kuna functions` — enumeration only.
    Inventory,
    /// `kuna decompile-all` / `kuna decompile-project` — enumeration plus bodies.
    Decompile,
    /// `kuna xrefs` — a reference query that runs its OWN recursive descent, so
    /// the Listing tier would only decode the program a second time over the
    /// same bytes. It takes the discovery bundle (`funcstart_patterns`, `aif`)
    /// but not the Listing: the seeds go straight into the reference walk
    /// (`listing::xrefs::discovery_seeds`) and the gap-walk runs over the
    /// partition that walk leaves behind.
    Query,
}

impl DriverDefaults {
    /// Does this surface decompile the entries it discovers (so the Listing's
    /// no-return facts change its output, not just its inventory)?
    fn decompiles(&self) -> bool {
        matches!(self, DriverDefaults::Decompile)
    }

    /// Does this surface want the program-wide Listing built for it (DIV-15)?
    ///
    /// `Query` does not: it walks the program itself, so the Listing would be a
    /// second decode of the same bytes. The DIV-20/DIV-68 discovery flags it
    /// still takes — the reference walk consumes both of them directly.
    fn wants_listing(&self) -> bool {
        !matches!(self, DriverDefaults::Query)
    }
}

/// The driver-default analysis options a surface injects before the option pass
/// — the shared source of the DIV-15 Listing default and the DIV-20/DIV-68
/// non-x86-64 discovery bundle, returned as `(name, value)` pairs in the order
/// they must be applied.
///
/// One function because the two surfaces that need them apply them differently:
/// the in-process drivers ([`load_program`]) call `set_kuna_option`, while
/// single-function `kuna decompile` emits `option` lines into the `decomp_dbg`
/// script. When only the in-process driver had them, a non-x86-64 entry that
/// exists solely because `funcstart_patterns` found it was listed by `kuna
/// functions` and decompiled by `kuna decompile-all`, yet `kuna decompile
/// <that name>` answered `no function matches` — kuna printed a name it would
/// not then accept (RE-need `analysis-generated-function-name`).
///
/// `decompiles` says whether the surface renders bodies, which is the one axis
/// the bundles differ on (the Listing is entry-neutral on x86-64, so
/// enumeration there does not pay for it). `binary` is read to classify the
/// architecture; anything `object` cannot parse — the corpus `<binaryimage>`
/// XML included — is treated as x86-64 and takes no discovery injection, so
/// those scripts stay byte-identical.
///
/// Every injection yields to the caller: naming the option at all (directly or
/// through a resolved `--mode` preset) skips it.
///
/// (kuna, decbench F1) The program-wide Listing is ON unless the caller set it
/// explicitly (`--option listing on|off` still wins — the injection is skipped
/// whenever the caller names `listing` at all).  Two independent reasons, one
/// per surface:
///
///   * A DECOMPILING surface needs it on every architecture.  The Listing
///     feeds the default-on `noreturn_propagate` consumer (the angr-style
///     call-graph no-return fixpoint, DIV-14): without it the pass is a
///     structural no-op, so a call to an unnamed internal exit/fatal wrapper
///     in a STRIPPED binary is treated as returning and the decompiler runs
///     past it, swallowing every following function into the caller (the
///     decbench `noreturn-propagation-stripped` family, e.g. coreutils
///     `xalloc_die`: 118 LOC / 2 gotos swallowed vs the true 4-instruction
///     body).  See DIV-15.
///   * On a NON-x86-64 image every surface needs it, `kuna functions`
///     included, because the Listing is the master gate of the DIV-20
///     discovery bundle below — `funcstart_patterns` and `aif` both walk the
///     Listing's code units and are inert without it, and those two passes
///     ARE the discovery on a stripped ARM/AArch64/MIPS/PPC/RISC-V binary.
///     See DIV-68.
///
/// x86-64 enumeration keeps the cheap path: the Listing is measured
/// entry-neutral there (identical entry sets on 40 sampled stripped x86-64
/// ELFs), so building it would only make `kuna functions` slower.  Only these
/// drivers change: the engine default (`analysis_listing = false`) and the
/// interactive console / datatest harness are untouched, and a selected mode
/// can still name Listing.
///
/// (kuna, decbench ARM) Oracle 5 — the always-on prologue-pattern scan folded into
/// function discovery — is x86-64-only, so on a STRIPPED **non-x86-64** binary the ELF
/// entry point is the ONLY discovered function (ARM Cortex-M `betaflight`: 1 of ~469;
/// it has no recursive-descent Listing sweep at the analyzer tier).  The
/// `funcstart_patterns` pass IS the primary discovery source there — it applies the
/// full ARM/AArch64/MIPS/PPC/RISC-V `<patternpairs>` (pre/post) prologue matcher over
/// the code — so it is ON for non-x86-64 on every driver surface, the `functions`
/// inventory included (DIV-68), unless the caller named it explicitly.  x86-64 keeps
/// it OFF (oracle 5 + the entry oracles suffice, and the aggressive scan can
/// over-produce there).  See DIV-20 (`docs/divergences.md`).
///
/// (kuna, decbench ARM) `funcstart_patterns` only seeds a candidate when a matching
/// EPILOGUE prepattern (Ghidra `<patternpairs>`) sits immediately before it, so ~70% of a
/// stripped Cortex-M firmware's functions — those preceded by literal pools / data /
/// padding and living in call-graph components reachable only through indirect calls /
/// function-pointer tables — are never seeded, and the recursive-descent walk (direct
/// CALL/BL only) structurally cannot reach them (crazyflie: 87% of the missed functions
/// have NO direct-call edge from what kuna found).  The ported Aggressive Instruction
/// Finder (`aif`, Ghidra `ArmAggressiveInstructionFinderAnalyzer`) gap-walks the UNDEFINED
/// regions the walk left uncovered, gating each candidate on a prologue-fingerprint
/// histogram learned from the already-discovered functions + `check_valid_subroutine`, so
/// it bridges those disconnected components.  It rides alongside `funcstart_patterns`
/// (crazyflie cf2.elf 1430 -> 2700 functions, 45% -> 82% of angr's discovered set),
/// unless the caller named it.  Extra non-ground-truth functions are harmless to the GED
/// benchmark (it scores per ground-truth function, matched by name).  See DIV-20.
pub(crate) fn driver_default_options(
    binary: &str,
    decompiles: bool,
    wants_listing: bool,
    options: &[(String, String)],
) -> Vec<(&'static str, &'static str)> {
    let named = |name: &str| options.iter().any(|(option, _)| option == name);
    let non_x86_64 = kuna_analysis::loader::elf_shdr::read_image(binary)
        .ok()
        .and_then(|bytes| {
            object::File::parse(&*bytes)
                .ok()
                .map(|file| file.architecture() != object::Architecture::X86_64)
        })
        .unwrap_or(false);

    let mut injected = Vec::new();
    // (kuna DIV-120) A function past the instruction budget reports the body kuna
    // DID decode, not nothing.  Upstream's default makes the overrun fatal, so a
    // function larger than `maxinstruction` decompiled to `code: null` and an
    // error naming no remedy; clearing the flag truncates the flow at the budget
    // instead and P3-P9 run on what was decoded, under a warning header that
    // names the knob.  Only the decompiling surfaces take it -- an inventory or
    // query load never follows flow -- and naming the option explicitly (or
    // `--option maxinstruction N` plus it) still restores the hard failure.
    if decompiles && !named("errortoomanyinstructions") {
        injected.push(("errortoomanyinstructions", "off"));
    }
    if wants_listing && (decompiles || non_x86_64) && !named("listing") {
        injected.push(("listing", "on"));
    }
    if non_x86_64 && !named("funcstart_patterns") {
        injected.push(("funcstart_patterns", "on"));
    }
    if non_x86_64 && !named("aif") {
        injected.push(("aif", "on"));
    }
    injected
}

/// Bootstrap the architecture from the binary and run the analysis commit (the
/// in-process `load file` + `read symbols`), applying load-time env gates and
/// `--option`s in the correct order.  `defaults` selects the driver-default
/// bundle (DIV-15/DIV-20/DIV-68): the discovery passes are shared by every
/// surface, the Listing-for-no-return default is the decompiling surfaces'.
pub(crate) fn load_program(
    args: &Args,
    defaults: DriverDefaults,
) -> Result<ConsoleProgram, String> {
    let binary = std::fs::canonicalize(&args.binary)
        .map_err(|_| format!("binary not found: {}", args.binary))?
        .to_string_lossy()
        .into_owned();
    // Load-time loader gates are read by `bootstrap_from_object` itself, so they
    // must be exported BEFORE it runs (the same gates `kuna decompile` threads to
    // the subprocess env). Keep the restoration guard alive through runtime
    // option recording too: `relocobjects`, `i386_pie_plt`, `relocrebase`,
    // `typedepth` and `dwarfstructs` update their env bridges again inside
    // `set_kuna_option` and must not leak into a later load.
    let _loadtime_env = apply_loadtime_env(&args.options, args.slice.as_deref());

    let spec_roots = spec_roots(args.sleighpath.as_deref());
    let target = args.target.as_deref().unwrap_or("");
    let mut prog = bootstrap_from_object(&binary, target, &spec_roots)
        .map_err(|e| format!("could not build an architecture for {binary}: {}", e.explain()))?;

    for (name, value) in driver_default_options(
        &binary,
        defaults.decompiles(),
        defaults.wants_listing(),
        &args.options,
    ) {
        // Through the same dispatch the caller's `--option`s take: a driver
        // default may name an upstream option (`errortoomanyinstructions`) as
        // well as a kuna one, and `set_kuna_option` only knows the kuna table.
        apply_one_option(&mut prog, name, value)?;
    }

    // (kuna `--assert`) A `readonly` range is inert unless read-only propagation
    // is on, and that option is default-off; asserting the range turns it on.
    // Set BEFORE the caller's own `--option`s so an explicit `--option readonly
    // off` still wins -- the same order the script surface emits it in.
    if kuna_console::assertions::implies_readonly_propagation(&args.assertions) {
        prog.arch_mut().readonlypropagate = true;
    }
    // Analysis-/printer-tier `--option`s must be applied to the architecture
    // BEFORE the gated analysis commit (the `option` < `read symbols` ordering
    // the script path enforces), so a per-pass gate takes effect.
    apply_runtime_options(&mut prog, &args.options)?;
    prog.commit_pending_analysis()
        .map_err(|e| format!("read symbols (analysis commit) failed: {}", e.explain()))?;
    // AFTER the commit: a caller-declared boundary is an assertion that outranks
    // whatever discovery decided about the same address.
    crate::funcdecl::apply(&mut prog, &args.func_decls)?;
    // The `--assert` plane, same slot and for the same reason: a caller-declared
    // fact outranks whatever discovery decided about it. The program-scoped
    // directives take effect here; the function- and symbol-scoped ones are
    // dispatched by the decompile loop (`kuna_console::assertions`).
    if !args.assertions.is_empty() {
        prog.set_assertions(args.assertions.clone());
        kuna_console::assertions::apply_program_scoped(&mut prog);
    }
    Ok(prog)
}

/// Build the target [`FunctionEntry`] list from the filters: `--addr` entries
/// (the canonical record at that address, else a record named via the symbol
/// table / `name_function`), `--functions` names or aliases, or — with no filter
/// — every CODE-backed entry, each exactly once.
pub(crate) fn resolve_targets(
    prog: &ConsoleProgram,
    args: &Args,
) -> Result<Vec<FunctionEntry>, String> {
    let mut targets: Vec<FunctionEntry> = Vec::new();

    // Resolve every address form through the program's shared selector model.
    for selector in &args.addrs {
        targets.push(prog.resolve_entry(selector).map_err(|error| error.to_string())?);
    }

    // `--functions a,b,c`: intersect names with the enumerated set.  An ALIAS
    // resolves too — collapsing the enumeration must not make a name that used
    // to select a function stop working (the decbench name-narrowing looks up
    // generated `sub_<addr>` names).
    if let Some(names) = &args.names {
        for want in names {
            match prog.resolve_entry(&EntrySelector::Name(want.clone())) {
                Ok(entry) => targets.push(entry),
                Err(EntryLookupError::NotFound { .. }) => {
                    eprintln!("warning: no function named {want:?} in {}", args.binary)
                }
                Err(error) => return Err(error.to_string()),
            }
        }
    }

    // Dedup the explicitly-selected targets by entry offset: `--addr 0xX` and
    // `--functions f` can resolve to the same function, and decompiling it twice
    // just wastes work + duplicates the JSON entry.
    let mut seen = std::collections::HashSet::new();
    targets.retain(|e| seen.insert(e.addr.get_offset()));

    // No filter at all ⇒ every executable function, exactly once. Import pointer
    // slots remain explicitly selectable, but are data rather than function
    // bodies.
    if args.addrs.is_empty() && args.names.is_none() {
        targets = prog.function_entries_executable();
    }
    Ok(targets)
}

// --- option / env-gate handling ---------------------------------------------

/// Is `name` a load-time loader gate (read inside `bootstrap_from_object`)?  Such
/// gates are exported as env vars BEFORE the bootstrap; the matching `option`
/// line is still applied afterward (for the catalog record), exactly as
/// `kuna decompile` does.
pub(crate) fn is_loadtime_gate(name: &str) -> bool {
    matches!(
        name,
        "relocobjects"
            | "i386_pie_plt"
            | "relocrebase"
            | "dynrelocs"
            | "pdatachained"
            | "macho-arm64e"
            | "typedepth"
            | "dwarfstructs"
            | "ifuncfpret"
            | "symbolnamerepair"
            | "symbolnamechars"
            | "symbolnamebound"
            | "msvcfpconst"
    )
}

fn last_option_value<'a>(options: &'a [(String, String)], name: &str) -> Option<&'a str> {
    options
        .iter()
        .rev()
        .find(|(option_name, _)| option_name == name)
        .map(|(_, value)| value.as_str())
}

#[derive(Default)]
struct LoadtimeEnv {
    previous: Vec<(&'static str, Option<OsString>)>,
}

impl LoadtimeEnv {
    fn set(&mut self, name: &'static str, value: impl AsRef<OsStr>) {
        self.previous.push((name, std::env::var_os(name)));
        std::env::set_var(name, value);
    }

    fn remove(&mut self, name: &'static str) {
        self.previous.push((name, std::env::var_os(name)));
        std::env::remove_var(name);
    }
}

impl Drop for LoadtimeEnv {
    fn drop(&mut self) {
        for (name, previous) in self.previous.drain(..).rev() {
            if let Some(value) = previous {
                std::env::set_var(name, value);
            } else {
                std::env::remove_var(name);
            }
        }
    }
}

/// Export the load-time loader gates (and the Mach-O slice) onto this process's
/// environment before `bootstrap_from_object` reads them — the in-process analog
/// of the `Command::env(...)` calls in `decompile.rs`.
fn apply_loadtime_env(options: &[(String, String)], slice: Option<&str>) -> LoadtimeEnv {
    let mut env = LoadtimeEnv::default();
    if let Some(slice) = slice.filter(|s| !s.trim().is_empty()) {
        env.set("KUNA_MACHO_SLICE", slice);
    }

    if let Some(value) = last_option_value(options, "relocobjects") {
        let off = matches!(
            value.trim().to_ascii_lowercase().as_str(),
            "0" | "off" | "false" | "no"
        );
        env.set(RELOC_OBJECTS_ENV, if off { "0" } else { "1" });
    }
    if let Some(value) = last_option_value(options, "i386_pie_plt") {
        let on = !matches!(
            value.trim().to_ascii_lowercase().as_str(),
            "off" | "0" | "false"
        );
        env.set("KUNA_I386_PIE_PLT", if on { "on" } else { "off" });
    }
    // (kuna, GH-289) The relocatable-object analysis rebase runs inside `load
    // file` (the whole analyzer tier does), so the gate must be exported before
    // `bootstrap_from_object`. Default-on: only an off-token disables it.
    if let Some(value) = last_option_value(options, "relocrebase") {
        let on = !matches!(
            value.trim().to_ascii_lowercase().as_str(),
            "off" | "0" | "false"
        );
        env.set(
            kuna_decomp::kuna_relocrebase::RELOCREBASE_ENV,
            if on { "on" } else { "off" },
        );
    }
    // (kuna, DIV-84) Same timing for the linked-image dynamic relocations: they
    // are applied inside the loader's own snapshot of the image bytes.
    if let Some(value) = last_option_value(options, "dynrelocs") {
        let on = !matches!(
            value.trim().to_ascii_lowercase().as_str(),
            "off" | "0" | "false"
        );
        env.set(
            kuna_decomp::kuna_dynrelocs::DYNRELOCS_ENV,
            if on { "on" } else { "off" },
        );
    }
    // (kuna, DIV-117) Same timing for the PE `.pdata` chained-record skip: the
    // entry oracles run inside `load file`.
    if let Some(value) = last_option_value(options, "pdatachained") {
        let on = !matches!(
            value.trim().to_ascii_lowercase().as_str(),
            "off" | "0" | "false"
        );
        env.set(
            kuna_decomp::kuna_pdatachained::PDATACHAINED_ENV,
            if on { "on" } else { "off" },
        );
    }
    // (kuna, DIV-96) Same timing for the MSVC `__real@` constants: the decoded
    // bytes are materialised while the loader lays the object out.
    if let Some(value) = last_option_value(options, "msvcfpconst") {
        let on = !matches!(
            value.trim().to_ascii_lowercase().as_str(),
            "off" | "0" | "false"
        );
        env.set(
            kuna_decomp::kuna_msvcfpconst::MSVCFPCONST_ENV,
            if on { "on" } else { "off" },
        );
    }
    // (kuna) The symbol table is installed inside `load file`, so the gate must be
    // exported before `bootstrap_from_object` -- turning it off after the fact
    // would arrive long after the load it was meant to fail. Default-on: only an
    // off-token disables it.
    if let Some(value) = last_option_value(options, "symbolnamerepair") {
        let on = !matches!(
            value.trim().to_ascii_lowercase().as_str(),
            "off" | "0" | "false"
        );
        env.set(
            kuna_decomp::kuna_symbolnamerepair::SYMBOLNAMEREPAIR_ENV,
            if on { "on" } else { "off" },
        );
    }
    // (kuna, GH-340) Symbol names are minted inside `load file` (the loader's
    // symbol walks and the analysis passes both run there), so the sanitizer's
    // mode must be exported before `bootstrap_from_object`. An unrecognized
    // token falls back to the shipped `safe` rather than silently to `off`.
    if let Some(value) = last_option_value(options, "symbolnamechars") {
        let mode = kuna_decomp::kuna_symbolnamechars::NameChars::parse(value).unwrap_or_default();
        env.set(
            kuna_decomp::kuna_symbolnamechars::SYMBOLNAMECHARS_ENV,
            mode.as_str(),
        );
    }
    // (kuna) The scope ceiling is spent while the symbol table is installed
    // inside `load file`, so it must be exported before `bootstrap_from_object`.
    // Valued: the token goes through verbatim.
    if let Some(value) = last_option_value(options, "symbolnamebound") {
        env.set(kuna_decomp::kuna_symbolnamebound::SYMBOLNAMEBOUND_ENV, value.trim());
    }
    if let Some(value) = last_option_value(options, "ifuncfpret") {
        // default-off, opt-in: only an on-token enables it.
        let on = matches!(
            value.trim().to_ascii_lowercase().as_str(),
            "on" | "1" | "true" | ""
        );
        env.set("KUNA_IFUNCFPRET", if on { "on" } else { "off" });
    }
    if let Some(value) = last_option_value(options, "typedepth") {
        let on = !matches!(
            value.trim().to_ascii_lowercase().as_str(),
            "off" | "0" | "false"
        );
        env.set(kuna_decomp::kuna_typedepth::TYPEDEPTH_ENV, if on { "on" } else { "off" });
    }
    if let Some(value) = last_option_value(options, "dwarfstructs") {
        let on = !matches!(
            value.trim().to_ascii_lowercase().as_str(),
            "off" | "0" | "false"
        );
        env.set(
            kuna_decomp::kuna_dwarfstructs::DWARFSTRUCTS_ENV,
            if on { "on" } else { "off" },
        );
    }
    if let Some(value) = last_option_value(options, "dwarfvariants") {
        let on = !matches!(
            value.trim().to_ascii_lowercase().as_str(),
            "off" | "0" | "false"
        );
        env.set(
            kuna_decomp::kuna_dwarfvariants::DWARFVARIANTS_ENV,
            if on { "on" } else { "off" },
        );
    }
    if let Some(value) = last_option_value(options, "macho-arm64e") {
        if matches!(
            value.trim().to_ascii_lowercase().as_str(),
            "on" | "true" | "1" | "yes"
        ) {
            env.set("KUNA_MACHO_ARM64E", "1");
        } else {
            env.remove("KUNA_MACHO_ARM64E");
        }
    }
    env
}

/// (kuna outlang) The output language to use when the caller named none.
///
/// A Rust binary rendered as C is strictly worse than the same binary rendered
/// as Rust: the types are wrong in a way a reader has to undo by hand. kuna
/// already knows which it is -- `sourcelang::detect_compiler` is the port of
/// Ghidra's `SourceLanguageAnalyzer`, and it reports `Rustc` from the `.comment`
/// `rustc version` record, a `.rodata` signature, or a Rust-mangled symbol -- so
/// the default follows the binary rather than making every user of a Rust binary
/// remember a flag.
///
/// Detection is high-precision, not heuristic, and `--language c` always wins.
/// Returns `None` when the file is not a Rust binary or cannot be parsed, which
/// leaves the C default in place: this can only ever ADD a language, never take
/// one away.
pub fn detected_output_language(binary: &str) -> Option<&'static str> {
    let bytes = std::fs::read(binary).ok()?;
    let file = object::File::parse(&*bytes).ok()?;
    match kuna_analysis::sourcelang::detect_compiler(&file, &bytes) {
        kuna_analysis::sourcelang::Compiler::Rustc => Some("rust-language"),
        _ => None,
    }
}

/// Resolve a `--language` value, or `None` for the auto policy.
///
/// `auto` is the default and the only value that is not a language name.
pub fn parse_language_flag(v: &str) -> Result<Option<&'static str>, String> {
    if v.eq_ignore_ascii_case("auto") {
        return Ok(None);
    }
    kuna_decomp::kuna_lang::OutLang::from_print_name(v)
        .map(|l| Some(l.print_name()))
        .ok_or_else(|| {
            format!(
                "unknown output language {v:?} (expected auto, or one of: {})",
                kuna_decomp::kuna_lang::OutLang::names().join(", ")
            )
        })
}

/// Apply each `--option NAME VALUE` to the live architecture, mirroring the
/// console `option` command (`IfcOption`): kuna stage-model options route to
/// `set_kuna_option`, upstream options to the `OptionDatabase`.  Load-time gates
/// are still applied here (so they are recorded) after their env export above.
fn apply_runtime_options(prog: &mut ConsoleProgram, options: &[(String, String)]) -> Result<(), String> {
    for (name, value) in options {
        apply_one_option(prog, name, value)?;
    }
    Ok(())
}

/// Apply one `NAME VALUE` pair to the live architecture: the kuna stage-model
/// table first, then the upstream `OptionDatabase`, then the load-time gates the
/// env bridge already handled.
fn apply_one_option(prog: &mut ConsoleProgram, name: &str, value: &str) -> Result<(), String> {
    if KUNA_OPTION_NAMES.contains(&name) {
        return prog
            .arch_mut()
            .set_kuna_option(name, value)
            .map(|_| ())
            .map_err(|e| format!("option {name}: {}", e.explain()));
    }
    let id = prog.registry().find_element(name, 0);
    if id == 0 {
        // A load-time gate may not be a registered upstream option but is a
        // valid kuna gate already handled via env; don't fail on it.
        if is_loadtime_gate(name) {
            return Ok(());
        }
        return Err(format!("unknown option: {name}"));
    }
    let db = OptionDatabase::new();
    db.set(prog.arch_mut(), id, value, "", "")
        .map(|_| ())
        .map_err(|e| format!("option {name}: {}", e.explain()))
}

/// The SLEIGH spec roots (an explicit `--sleighpath` wins, else `SLEIGHHOME` +
/// the repo `specs/`), matching `kuna fid`'s resolution.
fn spec_roots(sleighpath: Option<&str>) -> Vec<String> {
    let mut roots: Vec<String> = Vec::new();
    if let Some(p) = sleighpath.filter(|s| !s.is_empty()) {
        roots.push(p.to_string());
        return roots;
    }
    if let Ok(home) = std::env::var("SLEIGHHOME") {
        if !home.is_empty() {
            roots.push(home);
        }
    }
    let specs = paths::specs_dir().to_string_lossy().into_owned();
    if !roots.contains(&specs) {
        roots.push(specs);
    }
    roots
}

// --- discovery-failure diagnosis --------------------------------------------

/// Why a run that discovered ZERO functions failed, or `None` when an empty
/// inventory is the honest answer for this image.
///
/// A total discovery failure used to be reported in a successful run's voice —
/// `count: 0`, exit 0, silent stderr — which an agent cannot tell apart from
/// "this file genuinely has no functions". The distinction this makes is
/// EXECUTABLE CONTENT: a data-only relocatable object or a resource-only PE has
/// no functions to find, and failing those would turn a correct answer into an
/// error. An image that does carry code and yielded nothing is a failed run, and
/// the message names the cause it can prove, because a packed image is the one
/// an agent can act on.
pub(crate) fn zero_discovery_error(binary: &str) -> Option<String> {
    let bytes = std::fs::read(binary).unwrap_or_default();
    if !bytes.is_empty() && !image_has_executable_content(&bytes) {
        return None;
    }
    Some(match detect_packer(&bytes) {
        Some(packer) => format!(
            "no functions discovered in {binary}: image appears {packer}-packed; \
             try `kuna unpack`"
        ),
        None => format!("no functions discovered in {binary}"),
    })
}

/// The packer whose signature `bytes` carries.
///
/// UPX is the one that matters: it is what `kuna unpack` targets, and every UPX
/// build stamps the `UPX!` magic into its stub and into each packed block
/// header, so a whole-image search is both cheap (this runs only once a run has
/// already failed) and precise enough to name in a diagnostic.
fn detect_packer(bytes: &[u8]) -> Option<&'static str> {
    bytes.windows(4).any(|w| w == b"UPX!").then_some("UPX")
}

/// Does this image carry executable content at all?
///
/// Section flags first (the per-format executable test `kuna-analysis`'s entry
/// analyzers use), then the ELF program headers — a section-header-stripped PIE
/// has no section table at all, and the program header is what the loader obeys.
/// An image `object` cannot parse (a raw blob, a `<binaryimage>` document)
/// answers `true`: nothing there clears the run, so it stays a failure.
fn image_has_executable_content(bytes: &[u8]) -> bool {
    // ELF section header flag SHF_EXECINSTR; the Mach-O instruction attributes.
    const SHF_EXECINSTR: u64 = 0x4;
    const S_ATTR_PURE_INSTRUCTIONS: u32 = 0x8000_0000;
    const S_ATTR_SOME_INSTRUCTIONS: u32 = 0x0000_0400;
    // ELF program header flag PF_X.
    const PF_X: u32 = 0x1;

    let Ok(file) = object::File::parse(bytes) else {
        return true;
    };
    let executable_section = file.sections().any(|sec| {
        sec.size() != 0
            && match sec.flags() {
                object::SectionFlags::Elf { sh_flags } => sh_flags & SHF_EXECINSTR != 0,
                object::SectionFlags::Coff { characteristics } => {
                    characteristics & object::pe::IMAGE_SCN_MEM_EXECUTE != 0
                }
                object::SectionFlags::MachO { flags } => {
                    flags & (S_ATTR_PURE_INSTRUCTIONS | S_ATTR_SOME_INSTRUCTIONS) != 0
                        || sec.kind() == object::SectionKind::Text
                }
                _ => sec.kind() == object::SectionKind::Text,
            }
    });
    executable_section
        || file.segments().any(|seg| {
            seg.size() != 0
                && matches!(
                    seg.flags(),
                    object::SegmentFlags::Elf { p_flags } if p_flags & PF_X != 0
                )
        })
}

// --- output rendering --------------------------------------------------------

/// Render the `--json` document for a decompile run.
///
/// `kuna decompile --json` renders through here too (its `functions` array holds
/// the one function it was asked for), so the single-function and whole-binary
/// surfaces cannot drift into two shapes for one record.
pub(crate) fn render_result_json(
    binary: &str,
    funcs: &[FuncResult],
    options: &[(String, String)],
    error: Option<&str>,
    assertions: &[kuna_console::assertions::Outcome],
) -> String {
    render_selected_json(binary, funcs, options, error, None, assertions)
}

/// [`render_result_json`] plus the inventory `total` a NARROWED whole-binary run
/// answers with.
///
/// `total` is `Some` only when a triage filter chose these functions out of a
/// larger set, so a caller can never mistake a `--limit`-capped answer for the
/// whole program. An unfiltered `decompile-all --json` and every
/// `kuna decompile --json` pass `None` and are byte-identical to before — the
/// decbench backend consumes the first of those.
pub(crate) fn render_selected_json(
    binary: &str,
    funcs: &[FuncResult],
    options: &[(String, String)],
    error: Option<&str>,
    total: Option<usize>,
    assertions: &[kuna_console::assertions::Outcome],
) -> String {
    let language = last_option_value(options, "setlanguage").unwrap_or("c-language");
    format!(
        "{}\n",
        dumps_indent2(&result_json(binary, funcs, language, error, total, assertions))
    )
}

/// The `functions --json` document.
///
/// `count` is what the `functions` array holds and `total` what discovery found,
/// so a triage-narrowed listing says what it was narrowed from. They are equal on
/// an unfiltered run.
fn functions_json(
    binary: &str,
    entries: &[FunctionEntry],
    total: usize,
    error: Option<&str>,
) -> String {
    format!(
        "{}\n",
        dumps_indent2(&Json::Object(vec![
            ("binary".into(), Json::Str(binary.to_string())),
            ("count".into(), Json::Number(entries.len().to_string())),
            ("total".into(), Json::Number(total.to_string())),
            ("error".into(), error_json(error)),
            ("functions".into(), entries_json(entries)),
        ]))
    )
}

/// The inventory-record array shared by the `functions` listing and the
/// `--summary` document's `largest`.
fn entries_json(entries: &[FunctionEntry]) -> Json {
    Json::Array(
        entries
            .iter()
            .map(|e| {
                let a = e.addr.get_offset();
                Json::Object(vec![
                    ("name".into(), Json::Str(e.name.clone())),
                    ("address".into(), Json::Number(a.to_string())),
                    ("address_hex".into(), Json::Str(format!("0x{a:x}"))),
                    ("aliases".into(), aliases_json(&e.aliases)),
                    (
                        "object_location".into(),
                        object_location_json(e.object_location.as_ref()),
                    ),
                    ("size".into(), Json::Number(e.size.to_string())),
                ])
            })
            .collect(),
    )
}

/// The run-level `error` field. Always present (`null` on a healthy run) so a
/// consumer can read it unconditionally, exactly as [`aliases_json`] is.
fn error_json(error: Option<&str>) -> Json {
    error.map(|e| Json::Str(e.to_string())).unwrap_or(Json::Null)
}

/// Build the `decompile-all --json` document.
fn result_json(
    binary: &str,
    funcs: &[FuncResult],
    language: &str,
    error: Option<&str>,
    total: Option<usize>,
    assertions: &[kuna_console::assertions::Outcome],
) -> Json {
    let functions = Json::Array(
        funcs
            .iter()
            .map(|f| {
                let vars = Json::Array(f.variables.iter().map(var_json).collect());
                let line_mappings =
                    Json::Array(f.line_mappings.iter().map(line_mapping_json).collect());
                Json::Object(vec![
                    ("name".into(), Json::Str(f.name.clone())),
                    ("address".into(), Json::Number(f.address.to_string())),
                    ("address_hex".into(), Json::Str(format!("0x{:x}", f.address))),
                    ("aliases".into(), aliases_json(&f.aliases)),
                    (
                        "object_location".into(),
                        object_location_json(f.object_location.as_ref()),
                    ),
                    ("size".into(), Json::Number(f.size.to_string())),
                    (
                        "code".into(),
                        f.code.clone().map(Json::Str).unwrap_or(Json::Null),
                    ),
                    (
                        "error".into(),
                        f.error.clone().map(Json::Str).unwrap_or(Json::Null),
                    ),
                    ("line_mappings".into(), line_mappings),
                    ("variables".into(), vars),
                ])
            })
            .collect(),
    );
    let mut fields = vec![
        ("binary".to_string(), Json::Str(binary.to_string())),
        // (kuna outlang) The auto language policy resolves inside the engine, so
        // the document has to say which language `code` is in -- otherwise a
        // consumer cannot tell a Rust body from a C one without guessing.
        ("language".to_string(), Json::Str(language.to_string())),
        ("count".to_string(), Json::Number(funcs.len().to_string())),
    ];
    if let Some(total) = total {
        fields.push(("total".to_string(), Json::Number(total.to_string())));
    }
    fields.extend([(
        // The RUN-level error channel, set exactly when the command exits
        // non-zero (a total discovery failure here; the aborted function on
        // `kuna decompile --json`). A single function that failed inside a
        // whole-binary run is that record's own `error`, not this one.
        "error".to_string(),
        error_json(error),
    ), ("functions".to_string(), functions)]);
    // (kuna `--assert`) One row per caller-supplied assertion, in the order the
    // caller gave them: what it was, where in the phase model it wrote, and
    // whether it landed. Always present (`[]` when none were passed) so an agent
    // can read it unconditionally — an assertion whose fate is unreported is an
    // assertion an agent has to verify by eye, which is the whole problem.
    fields.push(("assertions".to_string(), assertions_json(assertions)));
    Json::Object(fields)
}

/// The per-directive report (`kuna_console::assertions::Outcome`).
fn assertions_json(outcomes: &[kuna_console::assertions::Outcome]) -> Json {
    Json::Array(
        outcomes
            .iter()
            .map(|o| {
                Json::Object(vec![
                    ("directive".into(), Json::Str(o.directive.clone())),
                    ("kind".into(), Json::Str(o.kind.to_string())),
                    ("phase".into(), Json::Str(o.phase.to_string())),
                    ("subphase".into(), Json::Str(o.subphase.to_string())),
                    ("status".into(), Json::Str(o.status.to_string())),
                    (
                        "detail".into(),
                        o.detail.clone().map(Json::Str).unwrap_or(Json::Null),
                    ),
                    ("fatal".into(), Json::Bool(o.fatal)),
                ])
            })
            .collect(),
    )
}

fn line_mapping_json(mapping: &LineMapping) -> Json {
    Json::Object(vec![
        ("line_number".into(), Json::Number(mapping.line_number.to_string())),
        (
            "addresses".into(),
            Json::Array(
                mapping
                    .addresses
                    .iter()
                    .map(|address| Json::Number(address.to_string()))
                    .collect(),
            ),
        ),
    ])
}

/// (kuna, issue #197) The `aliases` array: every OTHER name the reported entry
/// carries.  Always present (`[]` when the entry has exactly one name) so a
/// consumer can read the field unconditionally.  Additive — no existing field
/// changes shape, and the names that used to appear as extra top-level records
/// are all still here, one level down.
fn aliases_json(aliases: &[String]) -> Json {
    Json::Array(aliases.iter().map(|a| Json::Str(a.clone())).collect())
}

fn object_location_json(location: Option<&ObjectLocation>) -> Json {
    match location {
        Some(location) => Json::Object(vec![
            (
                "section_index".into(),
                Json::Number(location.section_index.to_string()),
            ),
            ("section".into(), Json::Str(location.section.clone())),
            ("offset".into(), Json::Number(location.offset.to_string())),
            (
                "offset_hex".into(),
                Json::Str(format!("0x{:x}", location.offset)),
            ),
        ]),
        None => Json::Null,
    }
}

/// One `VariableInfo`-shaped JSON object (the fields decbench's `type_match`
/// consumes).
fn var_json(v: &VarInfo) -> Json {
    Json::Object(vec![
        ("name".into(), Json::Str(v.name.clone())),
        ("type".into(), Json::Str(v.type_name.clone())),
        (
            "kind".into(),
            Json::Str(if v.is_param { "arg" } else { "stack" }.into()),
        ),
        (
            "arg_index".into(),
            v.arg_index.map(|i| Json::Number(i.to_string())).unwrap_or(Json::Null),
        ),
        (
            "stack_offset".into(),
            v.stack_offset.map(|o| Json::Number(o.to_string())).unwrap_or(Json::Null),
        ),
        ("size".into(), Json::Number(v.size.to_string())),
        (
            "line_numbers".into(),
            Json::Array(
                v.line_numbers
                    .iter()
                    .map(|line| Json::Number(line.to_string()))
                    .collect(),
            ),
        ),
        (
            "addresses".into(),
            Json::Array(
                v.addresses
                    .iter()
                    .map(|address| Json::Number(address.to_string()))
                    .collect(),
            ),
        ),
    ])
}

#[cfg(test)]
mod provenance_json_tests {
    use super::*;

    #[test]
    fn result_schema_adds_line_and_variable_provenance() {
        let function = FuncResult {
            name: "f".into(),
            address: 0x401000,
            size: 12,
            code: Some("int f(int x)\n{\n  return x;\n}".into()),
            error: None,
            proto: None,
            variables: vec![VarInfo {
                name: "x".into(),
                type_name: "int".into(),
                stack_offset: None,
                size: 4,
                is_param: true,
                arg_index: Some(0),
                line_numbers: vec![3],
                addresses: vec![0x401004],
            }],
            line_mappings: vec![LineMapping {
                line_number: 3,
                addresses: vec![0x401004, 0x401008],
            }],
            aliases: Vec::new(),
            object_location: None,
        };

        let rendered = dumps_indent2(&result_json("fixture", &[function], "c-language", None, None, &[]));
        assert!(rendered.contains("\"address\": 4198400"));
        assert!(rendered.contains("\"code\": \"int f(int x)\\n{\\n  return x;\\n}\""));
        assert!(rendered.contains("\"line_mappings\": ["));
        assert!(rendered.contains("\"line_number\": 3"));
        assert!(rendered.contains("\"line_numbers\": [\n            3"));
        assert!(rendered.contains("\"addresses\": [\n            4198404"));
    }
}

// --- argument parsing --------------------------------------------------------

/// Expand a concrete decompiler mode into its owned `(option, value)`
/// overrides. Callers PREPEND these before the user's `--option` pairs so an
/// explicit `--option` still wins (last-write, matching the console's `mode`
/// then `option` ordering). `auto` must first be resolved from binary metadata
/// by [`mode_options_for_binary`].
pub fn mode_override_pairs(mode: &str) -> Result<Vec<(String, String)>, String> {
    if kuna_decomp::modes::mode_is_automatic(mode) {
        return Err("mode `auto` requires input binary size".into());
    }
    match kuna_decomp::modes::mode_overrides(mode) {
        Some(ovr) => Ok(ovr.iter().map(|(o, v)| ((*o).to_string(), (*v).to_string())).collect()),
        None => {
            let known: Vec<&str> = kuna_decomp::modes::mode_names().collect();
            Err(format!("unknown mode {mode:?} (known: {})", known.join(", ")))
        }
    }
}

/// Resolve an omitted or explicit `auto` mode from `binary_size`; explicit
/// concrete modes ignore the size.
#[cfg(test)]
fn mode_override_pairs_for_size(
    mode: Option<&str>,
    binary_size: u64,
) -> Result<Vec<(String, String)>, String> {
    let concrete = kuna_decomp::modes::resolve_mode_for_size(mode, binary_size).ok_or_else(|| {
        let requested = mode.unwrap_or("auto");
        let known: Vec<&str> = kuna_decomp::modes::mode_names().collect();
        format!("unknown mode {requested:?} (known: {})", known.join(", "))
    })?;
    mode_override_pairs(concrete)
}

/// Resolve the frontend mode policy and prepend its overrides to the user's
/// explicit options. Omission is `auto`; file metadata is only read for an
/// omitted or explicitly automatic mode.
pub(crate) fn mode_options_for_binary(
    mode: Option<&str>,
    binary: &str,
    explicit: Vec<(String, String)>,
) -> Result<Vec<(String, String)>, String> {
    Ok(mode_and_options_for_binary(mode, binary, explicit)?.1)
}

fn mode_and_options_for_binary(
    mode: Option<&str>,
    binary: &str,
    explicit: Vec<(String, String)>,
) -> Result<(&'static str, Vec<(String, String)>), String> {
    let concrete = concrete_mode_for_binary(mode, binary)?;
    let mut merged = mode_override_pairs(concrete)?;
    merged.extend(explicit);
    Ok((concrete, merged))
}

fn concrete_mode_for_binary(
    mode: Option<&str>,
    binary: &str,
) -> Result<&'static str, String> {
    let requested = mode.unwrap_or("auto");
    if kuna_decomp::modes::mode_is_automatic(requested) {
        let size = std::fs::metadata(binary)
            .map_err(|e| format!("cannot read input binary metadata for mode auto: {binary}: {e}"))?
            .len();
        kuna_decomp::modes::resolve_mode_for_size(Some(requested), size).ok_or_else(|| {
            let known: Vec<&str> = kuna_decomp::modes::mode_names().collect();
            format!("unknown mode {requested:?} (known: {})", known.join(", "))
        })
    } else {
        kuna_decomp::modes::resolve_mode_for_size(Some(requested), 0).ok_or_else(|| {
            let known: Vec<&str> = kuna_decomp::modes::mode_names().collect();
            format!("unknown mode {requested:?} (known: {})", known.join(", "))
        })
    }
}

/// Parse `argv` for `cmd`, discarding the triage half.
///
/// The surfaces that do not narrow (`decompile-project`, `kuna decompile --json`)
/// call this so their [`Args`] shape and their parse are untouched by triage.
pub(crate) fn parse_args(argv: &[String], cmd: &str) -> Result<Args, String> {
    Ok(parse_args_with_filters(argv, cmd)?.0)
}

/// The full parse: the load/decompile arguments plus the triage selection.
///
/// The triage flags are accepted only on the two surfaces that act on them, so a
/// `decompile-project` run cannot silently swallow a `--filter` it would ignore.
pub(crate) fn parse_args_with_filters(
    argv: &[String],
    cmd: &str,
) -> Result<(Args, Filters), String> {
    let triageable = cmd == "decompile-all" || cmd == "functions";
    let mut filters = Filters::default();
    let mut binary: Option<String> = None;
    let mut json = false;
    let mut names: Option<Vec<String>> = None;
    let mut addrs: Vec<EntrySelector> = Vec::new();
    let mut no_vars = false;
    let mut max_fn_seconds: Option<u64> = None;
    let mut options: Vec<(String, String)> = Vec::new();
    let mut func_decls: Vec<crate::funcdecl::FuncDecl> = Vec::new();
    let mut assertions: Vec<kuna_console::assertions::Directive> = Vec::new();
    let mut assert_strict = false;
    let mut mode: Option<String> = None;
    let mut slice: Option<String> = None;
    let mut target: Option<String> = None;
    let mut sleighpath: Option<String> = None;
    let mut saw_language = false;

    let mut i = 0;
    while i < argv.len() {
        let a = argv[i].as_str();
        match a {
            "--json" => json = true,
            "--no-vars" => no_vars = true,
            "--functions" => {
                let v = take(argv, &mut i, "--functions")?;
                names = Some(v.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect());
            }
            "--addr" => {
                let v = take(argv, &mut i, "--addr")?;
                addrs.push(parse_entry_selector(&v)?);
            }
            "--define-function" => {
                let v = take(argv, &mut i, "--define-function")?;
                func_decls.extend(crate::funcdecl::parse_flag(&v)?);
            }
            "--assert" => {
                let v = take(argv, &mut i, "--assert")?;
                assertions.extend(crate::assertdecl::parse_flag(&v)?);
            }
            "--assert-strict" => assert_strict = true,
            "--max-fn-seconds"
                if cmd == "decompile-all"
                    || cmd == "decompile-project"
                    || cmd == "decompile-graph" =>
            {
                let v = take(argv, &mut i, "--max-fn-seconds")?;
                max_fn_seconds = Some(
                    v.trim()
                        .parse::<u64>()
                        .map_err(|_| format!("invalid --max-fn-seconds value {v:?}"))?,
                );
            }
            "--filter" if triageable => {
                let v = take(argv, &mut i, "--filter")?;
                filters.name = Some(
                    Regex::new(&v).map_err(|e| format!("invalid --filter regex {v:?}: {e}"))?,
                );
            }
            "--min-size" if triageable => {
                let v = take(argv, &mut i, "--min-size")?;
                filters.min_size = Some(parse_count(&v, "--min-size")?);
            }
            "--max-size" if triageable => {
                let v = take(argv, &mut i, "--max-size")?;
                filters.max_size = Some(parse_count(&v, "--max-size")?);
            }
            "--reachable-from" if triageable => {
                filters.reachable_from = Some(take(argv, &mut i, "--reachable-from")?);
            }
            "--limit" if triageable => {
                let v = take(argv, &mut i, "--limit")?;
                filters.limit = Some(parse_count(&v, "--limit")? as usize);
            }
            "--sort" if triageable => {
                let v = take(argv, &mut i, "--sort")?;
                filters.sort = SortKey::parse(&v)?;
            }
            "--summary" if triageable => filters.summary = true,
            "--option" => {
                if i + 2 >= argv.len() {
                    return Err("--option requires NAME VALUE".into());
                }
                crate::optname::check(&argv[i + 1])?;
                options.push((argv[i + 1].clone(), argv[i + 2].clone()));
                i += 2;
            }
            // (kuna outlang) `--language` is the first-class surface for the
            // output language; it lowers to the upstream `setlanguage` option, so
            // it reaches every downstream consumer (the console script here, the
            // in-process option applier in decompile-all) with no new plumbing.
            // Pushed in argv order, so a later `--option setlanguage` still wins.
            "--language" => {
                let v = take(argv, &mut i, "--language")?;
                if let Some(lang) = parse_language_flag(&v)? {
                    options.push(("setlanguage".into(), lang.into()));
                }
                saw_language = true;
            }
            "--mode" => mode = Some(take(argv, &mut i, "--mode")?),
            "--slice" => slice = Some(take(argv, &mut i, "--slice")?),
            "--target" => target = Some(take(argv, &mut i, "--target")?),
            "--sleighpath" => sleighpath = Some(take(argv, &mut i, "--sleighpath")?),
            "-h" | "--help" => {
                if cmd == "functions" {
                    usage_functions();
                } else {
                    usage_decompile_all();
                }
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

    let binary = binary.ok_or_else(|| format!("{cmd} requires <binary>"))?;

    // (kuna outlang, DIV-80) The auto policy: with no `--language` and no
    // explicit `--option setlanguage`, follow the binary. `decompile-project` and
    // `decompile-graph` are excluded -- a `.c`/`.h`/`.asm` export and a `codeC`
    // field are C-shaped by construction and refuse any other language, so
    // auto-selecting one there would turn a working export into an error.
    if !saw_language
        && cmd != "decompile-project"
        && cmd != "decompile-graph"
        && !options.iter().any(|(n, _)| n == "setlanguage")
    {
        if let Some(lang) = detected_output_language(&binary) {
            options.push(("setlanguage".into(), lang.into()));
        }
    }

    let explicit_fast_funcdisc = options.iter().any(|(name, _)| name == "fast_funcdisc");
    // Omitted mode is the size-driven `auto` policy. Mode overrides are
    // PREPENDED so an explicit `--option` still wins (last-write). Every
    // downstream consumer (`apply_loadtime_env`, the listing/funcstart
    // auto-inject skips, `apply_runtime_options`) reads `args.options`, so this
    // is the single wire point for decompile-all, decompile-project, and
    // functions.
    let (concrete_mode, merged) =
        mode_and_options_for_binary(mode.as_deref(), &binary, options)?;
    options = merged;
    if names.is_none() && !addrs.is_empty() && !explicit_fast_funcdisc {
        options.push(("fast_funcdisc".into(), "off".into()));
    }
    let whole_binary = (cmd == "decompile-all"
        || cmd == "decompile-project"
        || cmd == "decompile-graph")
        && names.is_none()
        && addrs.is_empty();
    let max_fn_seconds = max_fn_seconds
        .unwrap_or_else(|| default_fn_budget_seconds(concrete_mode, whole_binary));

    if let (Some(min), Some(max)) = (filters.min_size, filters.max_size) {
        if min > max {
            return Err(format!("--min-size {min} is greater than --max-size {max}"));
        }
    }

    Ok((
        Args {
            binary,
            json,
            names,
            addrs,
            no_vars,
            max_fn_seconds,
            options,
            func_decls,
            assertions,
            assert_strict,
            slice,
            target,
            sleighpath,
        },
        filters,
    ))
}

fn parse_count(v: &str, flag: &str) -> Result<u64, String> {
    v.trim()
        .parse::<u64>()
        .map_err(|_| format!("invalid {flag} value {v:?} (expected a non-negative integer)"))
}

fn take(argv: &[String], i: &mut usize, flag: &str) -> Result<String, String> {
    if *i + 1 < argv.len() {
        *i += 1;
        Ok(argv[*i].clone())
    } else {
        Err(format!("{flag} requires a value"))
    }
}

fn parse_hex(s: &str) -> Result<u64, String> {
    let t = s.trim();
    let body = t.strip_prefix("0x").or_else(|| t.strip_prefix("0X")).unwrap_or(t);
    u64::from_str_radix(body, 16).map_err(|_| format!("invalid address {s:?}"))
}

fn parse_entry_selector(s: &str) -> Result<EntrySelector, String> {
    match EntrySelector::parse(s) {
        selector @ (EntrySelector::Numeric(_)
        | EntrySelector::SectionOffset { .. }
        | EntrySelector::SectionIndexOffset { .. }) => Ok(selector),
        EntrySelector::Name(_) => parse_hex(s).map(EntrySelector::Numeric),
    }
}

fn usage_decompile_all() {
    eprintln!(
        "usage: kuna decompile-all <binary> [--json] [--functions a,b,..] [--addr 0xVMA].. \\\n\
         \x20                   [--no-vars] [--max-fn-seconds N] [--mode auto|reliable|aggressive|fast] \\\n\
         \x20                   [--filter REGEX] [--min-size N] [--max-size N] \\\n\
         \x20                   [--reachable-from <name|0xaddr>] [--sort addr|size|name] [--limit N] \\\n\
         \x20                   [--summary] [--define-function S[-E][=N]|@FILE].. \\\n\
         \x20                   [--option N V].. [--slice ARCH] [--target T] [--sleighpath D]\n\
         \n\
         Decompile every CODE-backed function in one in-process load (load-once,\n\
         decompile-many).  --json emits {{binary,count,functions:[{{name,address,code,variables,..}}]}};\n\
         without it, concatenated C with `// Function:` headers.\n\
         --max-fn-seconds N caps ONE function's decompile at N seconds (default 10\n\
         for unfiltered fast runs, 120 otherwise; 0 disables); a function over\n\
         budget becomes its own `error` record and the batch continues.\n\
         Omitted --mode uses auto: aggressive below 500 KiB, reliable below\n\
         2 MiB, and fast at 2 MiB or larger. Explicit --option values win.\n\
         An unfiltered run that discovers no function at all exits 1 with the\n\
         reason on stderr and in the document's run-level `error` field.\n\
         \n\
         --define-function <start[-end][=name] | @file> (repeatable) declares where a\n\
         function starts and ends: start names an entry discovery missed, the\n\
         exclusive end bounds its flow so it stops swallowing its neighbours.\n\
         \n\
         Triage (narrows the run BEFORE decompiling, so it is also what makes it\n\
         cheap): --filter REGEX matches the name or any alias; --min-size/--max-size\n\
         bound the inventory extent; --reachable-from <name|0xaddr> keeps only what\n\
         that function reaches through the call graph; --sort addr|size|name orders\n\
         (size is largest first) and --limit N caps. A narrowed --json document also\n\
         carries `total`, the count before narrowing. --summary skips the decompile\n\
         entirely and reports the orientation document (see `kuna functions -h`)."
    );
}

fn usage_functions() {
    eprintln!(
        "usage: kuna functions <binary> [--json] [--summary] \\\n\
         \x20               [--filter REGEX] [--min-size N] [--max-size N] \\\n\
         \x20               [--reachable-from <name|0xaddr>] [--sort addr|size|name] [--limit N] \\\n\
         \x20               [--define-function S[-E][=N]|@FILE].. \\\n\
         \x20               [--mode auto|reliable|aggressive|fast] [--slice ARCH] [--target T] [--sleighpath D]\n\
         \n\
         List every function kuna discovers in a binary as `<addr>\\t<name>` (or\n\
         --json: {{binary,count,total,functions:[{{name,address,address_hex,aliases,size}}]}}).\n\
         \n\
         Triage: --filter REGEX matches the name or any alias; --min-size/--max-size\n\
         bound the extent; --reachable-from <name|0xaddr> keeps only what that\n\
         function reaches through the call graph (the `kuna xrefs` edges);\n\
         --sort addr|size|name orders (size is largest first) and --limit N caps.\n\
         \n\
         --summary answers `where do I start` in a few hundred bytes instead of a\n\
         function list: the image entry point, how many functions it reaches, how\n\
         many have no call site, the size histogram, and the --limit largest\n\
         functions (10 by default).\n\
         Shares decompile-all's discovery policy, so the inventory always contains\n\
         every function a whole-binary run would decompile; on a non-x86-64 binary\n\
         that means a full prologue-pattern + gap-walk discovery pass.\n\
         --define-function <start[-end][=name] | @file> (repeatable) declares an entry\n\
         discovery missed and its exclusive extent; it enumerates like any other.\n\
         Discovering no function at all exits 1 with the reason on stderr and in\n\
         the document's `error` field (a packed image is named as such)."
    );
}

#[cfg(test)]
mod discovery_tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    static TEMP_ID: AtomicU64 = AtomicU64::new(0);

    /// A minimal ELF64 executable: one `PF_X` `PT_LOAD` and NO section table —
    /// the section-header-stripped PIE shape of the witness binary, where the
    /// program header is the only evidence the image holds code.
    fn stripped_executable(payload: &[u8]) -> Vec<u8> {
        const EHDR: usize = 64;
        const PHDR: usize = 56;
        let mut out = vec![0u8; EHDR + PHDR + payload.len()];
        out[..4].copy_from_slice(b"\x7fELF");
        out[4] = 2; // ELFCLASS64
        out[5] = 1; // ELFDATA2LSB
        out[6] = 1; // EV_CURRENT
        out[16..18].copy_from_slice(&2u16.to_le_bytes()); // e_type = ET_EXEC
        out[18..20].copy_from_slice(&62u16.to_le_bytes()); // e_machine = EM_X86_64
        out[20..24].copy_from_slice(&1u32.to_le_bytes()); // e_version
        out[32..40].copy_from_slice(&(EHDR as u64).to_le_bytes()); // e_phoff
        out[52..54].copy_from_slice(&(EHDR as u16).to_le_bytes()); // e_ehsize
        out[54..56].copy_from_slice(&(PHDR as u16).to_le_bytes()); // e_phentsize
        out[56..58].copy_from_slice(&1u16.to_le_bytes()); // e_phnum
        let total = (EHDR + PHDR + payload.len()) as u64;
        let p = EHDR;
        out[p..p + 4].copy_from_slice(&1u32.to_le_bytes()); // p_type = PT_LOAD
        out[p + 4..p + 8].copy_from_slice(&5u32.to_le_bytes()); // p_flags = PF_R|PF_X
        out[p + 16..p + 24].copy_from_slice(&0x40_0000u64.to_le_bytes()); // p_vaddr
        out[p + 32..p + 40].copy_from_slice(&total.to_le_bytes()); // p_filesz
        out[p + 40..p + 48].copy_from_slice(&total.to_le_bytes()); // p_memsz
        out[p + 48..p + 56].copy_from_slice(&0x1000u64.to_le_bytes()); // p_align
        out[EHDR + PHDR..].copy_from_slice(payload);
        out
    }

    /// A minimal ET_REL ELF64 whose only allocated section is `.data` — an
    /// object file that legitimately holds no functions at all.
    fn data_only_object() -> Vec<u8> {
        const EHDR: usize = 64;
        const SHDR: usize = 64;
        let names: &[u8] = b"\0.data\0.shstrtab\0";
        let shoff = EHDR;
        let names_off = shoff + 3 * SHDR;
        let data_off = names_off + names.len();
        let mut out = vec![0u8; data_off + 4];
        out[..4].copy_from_slice(b"\x7fELF");
        out[4] = 2;
        out[5] = 1;
        out[6] = 1;
        out[16..18].copy_from_slice(&1u16.to_le_bytes()); // e_type = ET_REL
        out[18..20].copy_from_slice(&62u16.to_le_bytes());
        out[20..24].copy_from_slice(&1u32.to_le_bytes());
        out[40..48].copy_from_slice(&(shoff as u64).to_le_bytes()); // e_shoff
        out[52..54].copy_from_slice(&(EHDR as u16).to_le_bytes());
        out[58..60].copy_from_slice(&(SHDR as u16).to_le_bytes()); // e_shentsize
        out[60..62].copy_from_slice(&3u16.to_le_bytes()); // e_shnum
        out[62..64].copy_from_slice(&2u16.to_le_bytes()); // e_shstrndx
        out[names_off..data_off].copy_from_slice(names);

        fn shdr(out: &mut [u8], at: usize, name: u32, kind: u32, flags: u64, off: u64, size: u64) {
            out[at..at + 4].copy_from_slice(&name.to_le_bytes());
            out[at + 4..at + 8].copy_from_slice(&kind.to_le_bytes());
            out[at + 8..at + 16].copy_from_slice(&flags.to_le_bytes());
            out[at + 24..at + 32].copy_from_slice(&off.to_le_bytes());
            out[at + 32..at + 40].copy_from_slice(&size.to_le_bytes());
        }
        // `.data`: SHT_PROGBITS, SHF_ALLOC|SHF_WRITE — allocated, never executed.
        shdr(&mut out, shoff + SHDR, 1, 1, 0x3, data_off as u64, 4);
        shdr(&mut out, shoff + 2 * SHDR, 7, 3, 0, names_off as u64, names.len() as u64);
        out
    }

    fn temp_image(tag: &str, bytes: &[u8]) -> std::path::PathBuf {
        let id = TEMP_ID.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "kuna-discovery-{tag}-{}-{id}.bin",
            std::process::id()
        ));
        std::fs::write(&path, bytes).expect("write the discovery fixture");
        path
    }

    #[test]
    fn the_upx_magic_is_recognized_and_nothing_else_is() {
        assert_eq!(detect_packer(b"....UPX!....."), Some("UPX"));
        assert_eq!(detect_packer(b"UPX!"), Some("UPX"));
        assert_eq!(detect_packer(b"a plain unpacked image, UPX-free"), None);
        assert_eq!(detect_packer(b"UPX"), None, "the magic is four bytes");
        assert_eq!(detect_packer(b""), None);
    }

    /// A section-header-stripped executable still shows its code through the
    /// program headers, so zero functions there is a failure — and a packed one
    /// is named, because that is the cause the caller can act on.
    #[test]
    fn a_packed_image_names_the_packer_in_the_failure() {
        let packed = temp_image("packed", &stripped_executable(b"UPX!\x00\x00\x00\x00"));
        let message = zero_discovery_error(packed.to_str().unwrap())
            .expect("an executable image that yielded nothing is a failure");
        assert!(message.contains("no functions"), "{message}");
        assert!(message.contains("UPX-packed"), "{message}");
        assert!(message.contains("kuna unpack"), "{message}");
        std::fs::remove_file(packed).expect("remove the discovery fixture");

        let plain = temp_image("plain", &stripped_executable(b"\x55\x48\x89\xe5\x5d\xc3"));
        let message = zero_discovery_error(plain.to_str().unwrap())
            .expect("an executable image that yielded nothing is a failure");
        assert!(message.contains("no functions"), "{message}");
        assert!(!message.contains("packed"), "no packer, no packer claim: {message}");
        std::fs::remove_file(plain).expect("remove the discovery fixture");
    }

    /// The legitimate empty case: an image with no executable content has no
    /// functions to find, so the empty inventory stays a success.
    #[test]
    fn an_image_with_no_code_keeps_its_honest_empty_answer() {
        let bytes = data_only_object();
        assert!(!image_has_executable_content(&bytes));
        let path = temp_image("dataonly", &bytes);
        assert_eq!(zero_discovery_error(path.to_str().unwrap()), None);
        std::fs::remove_file(path).expect("remove the discovery fixture");
    }

    /// Anything `object` cannot parse is not evidence of innocence: a run that
    /// found nothing in it still failed.
    #[test]
    fn an_unparseable_image_is_still_a_failure() {
        assert!(image_has_executable_content(b"not an object file at all"));
        assert!(image_has_executable_content(&[]));
    }

    /// The checked-in x86-64 fixture the acceptance probes use: real code, real
    /// sections — the shape that must fail loudly if discovery ever returns
    /// nothing for it.
    #[test]
    fn a_real_fixture_carries_executable_content() {
        let fixture = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../kuna-analysis/tests/fixtures/aif_gap_x86_64");
        let bytes = std::fs::read(fixture).expect("the fixture is checked in");
        assert!(image_has_executable_content(&bytes));
        assert_eq!(detect_packer(&bytes), None);
    }

    /// The run-level `error` field is present on every document, so a consumer
    /// reads it unconditionally rather than inferring failure from `count`.
    #[test]
    fn the_run_level_error_field_is_always_present() {
        let healthy = functions_json("fixture", &[], 0, None);
        assert!(healthy.contains("\"error\": null"), "{healthy}");
        let failed = functions_json("fixture", &[], 0, Some("no functions discovered in fixture"));
        assert!(
            failed.contains("\"error\": \"no functions discovered in fixture\""),
            "{failed}"
        );
        let decompiled = render_result_json("fixture", &[], &[], Some("boom"), &[]);
        assert!(decompiled.contains("\"error\": \"boom\""), "{decompiled}");
        assert!(
            render_result_json("fixture", &[], &[], None, &[]).contains("\"error\": null")
        );
    }

    /// `--language` reaches the document through the same last-write-wins lookup
    /// every other option uses.
    #[test]
    fn the_document_reports_the_selected_language() {
        let options = vec![
            ("setlanguage".into(), "rust-language".into()),
            ("setlanguage".into(), "c-language".into()),
        ];
        assert!(render_result_json("f", &[], &options, None, &[]).contains("\"c-language\""));
        assert!(render_result_json("f", &[], &[], None, &[]).contains("\"c-language\""));
    }
}

#[cfg(test)]
mod mode_tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    static TEMP_ID: AtomicU64 = AtomicU64::new(0);

    fn sparse_binary(size: u64) -> std::path::PathBuf {
        let id = TEMP_ID.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "kuna-auto-mode-{}-{id}.bin",
            std::process::id()
        ));
        let file = std::fs::File::create(&path).expect("create auto-mode fixture");
        file.set_len(size).expect("size auto-mode fixture");
        path
    }

    #[test]
    fn omitted_and_explicit_auto_select_the_same_concrete_presets() {
        for (size, concrete) in [
            (0, "aggressive"),
            (kuna_decomp::modes::AUTO_RELIABLE_MIN_BYTES, "reliable"),
            (kuna_decomp::modes::AUTO_FAST_MIN_BYTES, "fast"),
        ] {
            let expected = mode_override_pairs(concrete).unwrap();
            assert_eq!(mode_override_pairs_for_size(None, size).unwrap(), expected);
            assert_eq!(mode_override_pairs_for_size(Some("auto"), size).unwrap(), expected);
        }
    }

    #[test]
    fn explicit_concrete_mode_ignores_binary_metadata() {
        let missing = std::env::temp_dir().join(format!(
            "kuna-auto-mode-missing-{}.bin",
            std::process::id()
        ));
        let options =
            mode_options_for_binary(Some("fast"), missing.to_str().unwrap(), Vec::new()).unwrap();
        assert_eq!(options, mode_override_pairs("fast").unwrap());
    }

    #[test]
    fn shared_parsers_default_to_auto_and_keep_user_options_last() {
        let path = sparse_binary(kuna_decomp::modes::AUTO_FAST_MIN_BYTES);
        let binary = path.to_string_lossy().into_owned();
        for cmd in ["decompile-all", "decompile-project", "functions"] {
            let argv = vec![
                binary.clone(),
                "--option".into(),
                "listing".into(),
                "on".into(),
            ];
            let args = parse_args(&argv, cmd).unwrap();
            let listing: Vec<&str> = args
                .options
                .iter()
                .filter(|(name, _)| name == "listing")
                .map(|(_, value)| value.as_str())
                .collect();
            assert_eq!(listing, vec!["off", "on"], "{cmd} precedence");
        }
        std::fs::remove_file(path).expect("remove auto-mode fixture");
    }

    #[test]
    fn unfiltered_fast_batches_default_to_ten_seconds_per_function() {
        let path = sparse_binary(kuna_decomp::modes::AUTO_FAST_MIN_BYTES);
        let binary = path.to_string_lossy().into_owned();
        for cmd in ["decompile-all", "decompile-project"] {
            let args = parse_args(std::slice::from_ref(&binary), cmd).unwrap();
            assert_eq!(
                args.max_fn_seconds,
                kuna_console::project::FAST_WHOLE_BINARY_FN_BUDGET_SECONDS,
                "{cmd}"
            );

            let selected = parse_args(
                &[
                    binary.clone(),
                    "--addr".into(),
                    "0x1234".into(),
                    "--mode".into(),
                    "fast".into(),
                ],
                cmd,
            )
            .unwrap();
            assert_eq!(
                selected.max_fn_seconds,
                kuna_console::project::DEFAULT_FN_BUDGET_SECONDS,
                "{cmd} selected"
            );
            let named = parse_args(
                &[
                    binary.clone(),
                    "--functions".into(),
                    "sub_1234".into(),
                    "--mode".into(),
                    "fast".into(),
                ],
                cmd,
            )
            .unwrap();
            assert_eq!(
                named.max_fn_seconds,
                kuna_console::project::DEFAULT_FN_BUDGET_SECONDS,
                "{cmd} named"
            );

            let disabled = parse_args(
                &[
                    binary.clone(),
                    "--mode".into(),
                    "fast".into(),
                    "--max-fn-seconds".into(),
                    "0".into(),
                ],
                cmd,
            )
            .unwrap();
            assert_eq!(disabled.max_fn_seconds, 0, "{cmd} explicit override");
            let explicit = parse_args(
                &[
                    binary.clone(),
                    "--mode".into(),
                    "fast".into(),
                    "--max-fn-seconds".into(),
                    "17".into(),
                ],
                cmd,
            )
            .unwrap();
            assert_eq!(explicit.max_fn_seconds, 17, "{cmd} explicit budget");
        }

        for mode in ["reliable", "aggressive"] {
            let args = parse_args(
                &[binary.clone(), "--mode".into(), mode.into()],
                "decompile-all",
            )
            .unwrap();
            assert_eq!(
                args.max_fn_seconds,
                kuna_console::project::DEFAULT_FN_BUDGET_SECONDS,
                "{mode}"
            );
        }
        let functions = parse_args(&[binary], "functions").unwrap();
        assert_eq!(
            functions.max_fn_seconds,
            kuna_console::project::DEFAULT_FN_BUDGET_SECONDS
        );
        std::fs::remove_file(path).expect("remove auto-mode fixture");
    }

    #[test]
    fn address_selection_skips_preset_discovery_but_names_do_not() {
        let path = sparse_binary(kuna_decomp::modes::AUTO_FAST_MIN_BYTES);
        let binary = path.to_string_lossy().into_owned();
        let address_args = parse_args(
            &[
                binary.clone(),
                "--addr".into(),
                "0x1234".into(),
                "--mode".into(),
                "fast".into(),
            ],
            "decompile-project",
        )
        .unwrap();
        assert_eq!(
            last_option_value(&address_args.options, "fast_funcdisc"),
            Some("off")
        );

        let explicit_args = parse_args(
            &[
                binary.clone(),
                "--addr".into(),
                "0x1234".into(),
                "--mode".into(),
                "fast".into(),
                "--option".into(),
                "fast_funcdisc".into(),
                "on".into(),
            ],
            "decompile-project",
        )
        .unwrap();
        assert_eq!(
            last_option_value(&explicit_args.options, "fast_funcdisc"),
            Some("on")
        );

        let named_args = parse_args(
            &[
                binary,
                "--functions".into(),
                "sub_1234".into(),
                "--mode".into(),
                "fast".into(),
            ],
            "decompile-project",
        )
        .unwrap();
        assert_eq!(
            last_option_value(&named_args.options, "fast_funcdisc"),
            Some("on")
        );
        std::fs::remove_file(path).expect("remove auto-mode fixture");
    }

    #[test]
    fn loadtime_gates_use_the_last_named_option() {
        let options = vec![
            ("relocobjects".into(), "on".into()),
            ("i386_pie_plt".into(), "on".into()),
            ("macho-arm64e".into(), "on".into()),
            ("relocobjects".into(), "off".into()),
            ("i386_pie_plt".into(), "off".into()),
            ("macho-arm64e".into(), "off".into()),
        ];
        assert_eq!(last_option_value(&options, "relocobjects"), Some("off"));
        assert_eq!(last_option_value(&options, "i386_pie_plt"), Some("off"));
        assert_eq!(last_option_value(&options, "macho-arm64e"), Some("off"));
    }
}

#[cfg(test)]
mod triage_tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    static TEMP_ID: AtomicU64 = AtomicU64::new(0);

    /// `parse_args` reads the file's size to resolve the `auto` mode, so the
    /// triage parse needs a real path — the bytes are never looked at.
    fn sparse_binary() -> std::path::PathBuf {
        let id = TEMP_ID.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir()
            .join(format!("kuna-triage-{}-{id}.bin", std::process::id()));
        std::fs::File::create(&path)
            .expect("create triage fixture")
            .set_len(1024)
            .expect("size triage fixture");
        path
    }

    fn parse(cmd: &str, extra: &[&str]) -> Result<(Args, Filters), String> {
        let path = sparse_binary();
        let mut argv = vec![path.to_string_lossy().into_owned()];
        argv.extend(extra.iter().map(|s| (*s).to_string()));
        let parsed = parse_args_with_filters(&argv, cmd);
        std::fs::remove_file(path).expect("remove triage fixture");
        parsed
    }

    /// `Args`/`Filters` are not `Debug`, so a rejected parse is unwrapped here.
    fn err(parsed: Result<(Args, Filters), String>) -> String {
        match parsed {
            Ok(_) => panic!("expected the parse to be rejected"),
            Err(e) => e,
        }
    }

    #[test]
    fn sort_keys_are_the_three_documented_ones() {
        assert_eq!(SortKey::parse("addr").unwrap(), SortKey::Addr);
        assert_eq!(SortKey::parse("ADDRESS").unwrap(), SortKey::Addr);
        assert_eq!(SortKey::parse(" size ").unwrap(), SortKey::Size);
        assert_eq!(SortKey::parse("name").unwrap(), SortKey::Name);
        assert!(SortKey::parse("entropy").is_err());
        assert_eq!(SortKey::default(), SortKey::Addr);
    }

    /// An unnarrowed run must keep every pre-existing behaviour, so `narrows()`
    /// is what gates the whole feature and has to be false by default.
    #[test]
    fn a_bare_run_narrows_nothing() {
        let (_, filters) = parse("functions", &[]).unwrap();
        assert!(!filters.narrows());
        assert!(!filters.summary);
        for flag in [
            vec!["--filter", "main"],
            vec!["--min-size", "1"],
            vec!["--max-size", "1"],
            vec!["--reachable-from", "main"],
            vec!["--limit", "1"],
            vec!["--sort", "size"],
        ] {
            let (_, filters) = parse("functions", &flag).unwrap();
            assert!(filters.narrows(), "{flag:?}");
        }
    }

    #[test]
    fn the_triage_flags_parse_on_both_whole_binary_surfaces() {
        for cmd in ["functions", "decompile-all"] {
            let (_, filters) = parse(
                cmd,
                &[
                    "--filter", "^auth",
                    "--min-size", "16",
                    "--max-size", "256",
                    "--reachable-from", "0x1000",
                    "--sort", "size",
                    "--limit", "4",
                    "--summary",
                ],
            )
            .unwrap_or_else(|e| panic!("{cmd}: {e}"));
            assert!(filters.name.as_ref().unwrap().is_match("authenticate"), "{cmd}");
            assert!(!filters.name.as_ref().unwrap().is_match("deauth"), "{cmd}");
            assert_eq!(filters.min_size, Some(16), "{cmd}");
            assert_eq!(filters.max_size, Some(256), "{cmd}");
            assert_eq!(filters.reachable_from.as_deref(), Some("0x1000"), "{cmd}");
            assert_eq!(filters.sort, SortKey::Size, "{cmd}");
            assert_eq!(filters.limit, Some(4), "{cmd}");
            assert!(filters.summary, "{cmd}");
            assert_eq!(filters.largest_wanted(), 4, "{cmd}");
        }
    }

    /// `decompile-project` does not act on a narrowing, so it must reject one
    /// rather than silently discard it.
    #[test]
    fn decompile_project_rejects_the_triage_flags() {
        let e = err(parse("decompile-project", &["--filter", "main"]));
        assert!(e.contains("unknown option --filter"), "{e}");
        assert!(parse("decompile-project", &["--summary"]).is_err());
    }

    #[test]
    fn bad_triage_values_are_rejected_at_parse_time() {
        assert!(parse("functions", &["--filter", "("]).is_err());
        assert!(parse("functions", &["--limit", "-1"]).is_err());
        assert!(parse("functions", &["--min-size", "big"]).is_err());
        let e = err(parse("functions", &["--min-size", "10", "--max-size", "9"]));
        assert!(e.contains("greater than --max-size"), "{e}");
        assert!(parse("functions", &["--min-size", "9", "--max-size", "9"]).is_ok());
    }

    /// The histogram has to partition the whole size domain: a function that
    /// falls in no bucket would silently vanish from the summary.
    #[test]
    fn the_size_buckets_partition_every_extent() {
        for size in [0u64, 1, 15, 16, 63, 64, 255, 256, 1023, 1024, 4095, 4096, u64::MAX] {
            let hits = SIZE_BUCKETS.iter().filter(|(_, lo, hi)| size >= *lo && size <= *hi).count();
            assert_eq!(hits, 1, "size {size} lands in {hits} buckets");
        }
        assert_eq!(DEFAULT_SUMMARY_LARGEST, 10);
    }
}
