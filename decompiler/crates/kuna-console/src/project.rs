//! Shared **decompile-project core**: the whole-binary decompile loop
//! ([`decompile_targets`] / [`decompile_pulled`] / [`FuncResult`] /
//! [`render_c`]) and the four project-export artifact builders behind
//! `kuna decompile-project` (`<name>.c` / `<name>.h` / `<name>.asm` /
//! `README.md`).
//!
//! A `--stream` export builds the same artifacts incrementally: it pulls targets
//! one at a time ([`decompile_pulled`]), steers the order with
//! [`FuncResult::callee_hints`], renders its README from a snapshot
//! ([`ReadmeFacts`] / [`render_readme`] / [`render_readme_streaming`]) rather
//! than from the program, and drives the disassembly in bounded steps
//! ([`crate::project_stream`]).
//!
//! Used by both the `kuna` CLI (`kuna-cli`'s `decompile-all` /
//! `decompile-project` surfaces) and the `kuna_wasm` in-browser front-end —
//! moved here from `kuna-cli` so a `wasm32-wasip1` build can reach it without
//! `kuna-cli`'s subprocess machinery.  The callers keep their own argument
//! parsing / program loading (`load_program` / `resolve_targets` stay in
//! `kuna-cli`); everything here operates on an already-loaded
//! [`ConsoleProgram`] and the resolved [`FunctionEntry`] target list.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::path::Path;

use kuna_base::address::Address;
use kuna_decomp::decompile_drive::{
    extract_variables, print_c, print_c_prototype, print_c_with_provenance, LineMapping, VarInfo,
};
use kuna_decomp::funcdata::Funcdata;
use kuna_num::opcodes::OpCode;
// `ConsoleProgram::sections` documents its `flags` word as exactly the
// `kuna_sleigh::loadimage::section_flags` constant set (UNALLOC=1, NOLOAD=2,
// CODE=4, DATA=8, READONLY=16).  kuna-console depends on kuna-sleigh, so the
// real constants are used directly here (kuna-cli, which deliberately does not
// depend on kuna-sleigh, used to mirror the stable, upstream-fixed CODE bit).
use kuna_sleigh::loadimage::section_flags;

use crate::engine::{ConsoleProgram, EntryProvenance, FunctionEntry, ObjectLocation};

/// `dat_` blocks larger than this are truncated in the `.asm` data tail (the
/// printer's `dat_<hex>` names carry no size; the label only marks the start).
const DAT_SIZE_CAP: u64 = 32;

pub const DEFAULT_FN_BUDGET_SECONDS: u64 = 120;
pub const FAST_WHOLE_BINARY_FN_BUDGET_SECONDS: u64 = 10;

pub fn default_fn_budget_seconds(mode: &str, whole_binary: bool) -> u64 {
    if mode == "fast" && whole_binary {
        FAST_WHOLE_BINARY_FN_BUDGET_SECONDS
    } else {
        DEFAULT_FN_BUDGET_SECONDS
    }
}

/// One decompiled function's result (success carries `code`; failure carries
/// `error`).
pub struct FuncResult {
    pub name: String,
    /// User-facing address, in the target's address units for a raw image.
    pub address: u64,
    /// Engine address, always stored as a byte offset.
    pub byte_address: u64,
    /// The entry's byte extent, carried through from
    /// [`FunctionEntry::size`](crate::engine::FunctionEntry::size) so this
    /// surface and the `functions` inventory report ONE number with one meaning.
    ///
    /// An inventory fact, not a decompile result: it is reported even when the
    /// function errored, and it does NOT come from the recovered
    /// `Funcdata::get_size()`, which is the caller's requested flow bound and so
    /// is `0` on every whole-binary run (the bound is always "unbounded").
    pub size: i64,
    pub code: Option<String>,
    pub error: Option<String>,
    /// The `.h` prototype line (`<ret> <name>(<params>);`), captured only when
    /// the caller asked for it (`decompile_targets(want_proto=true)` — the
    /// `decompile-project` surface).  Always `None` on the `decompile-all`
    /// path, which does not serialize prototypes.
    pub proto: Option<String>,
    pub variables: Vec<VarInfo>,
    pub line_mappings: Vec<LineMapping>,
    /// (kuna, issue #197) Every OTHER name this entry carries — a generic
    /// `sub_<addr>` placeholder, an ELF weak/strong twin, a PE
    /// decorated/undecorated pair.  Carried through from
    /// [`crate::engine::FunctionEntry`] so collapsing the enumeration to one
    /// record per entry loses no name; empty for a target the caller named
    /// itself (`--addr` on an address the enumeration does not know).
    pub aliases: Vec<String>,
    /// Original object-file coordinate for a relocatable definition.
    pub object_location: Option<ObjectLocation>,
    /// (kuna `--stream`) Byte addresses this function reaches: the entry address
    /// of every direct `CALL`, plus every constant that names a known function
    /// entry living in code (the address-taken case — `_start` hands `main` to
    /// `__libc_start_main` as a pointer).  Thumb-bit normalized, sorted,
    /// deduped, and never containing this function's own entry.
    ///
    /// A SCHEDULING HINT for `kuna decompile-project --stream`, which decompiles
    /// a function's callees before unrelated functions so the export fills in
    /// entry-point-first.  It is NOT the CLI's call-edge model (`CallGraph` in
    /// kuna-cli, which answers reference questions from the xref index over the
    /// whole image) and is never serialized into any exported document.  Empty on
    /// every error record, and on every run that did not ask for it
    /// ([`DecompileOptions::want_callee_hints`]).
    pub callee_hints: Vec<u64>,
}

/// What a decompile batch captures beyond the C itself.
///
/// `single_target` is an explicit input rather than a count: it gates the
/// `--assert` plane's unqualified directives (a directive that names no function
/// binds to "the function under decompile", which is only unambiguous when the
/// run selected exactly one), and a pull-based batch does not know its own
/// length up front.
#[derive(Debug, Clone, Copy, Default)]
pub struct DecompileOptions {
    pub no_vars: bool,
    pub want_proto: bool,
    pub want_provenance: bool,
    /// Collect [`FuncResult::callee_hints`] — the `--stream` scheduler's frontier.
    pub want_callee_hints: bool,
    pub single_target: bool,
}

/// Decompile each `(name, entry)` target in turn against the already-loaded
/// program, returning one [`FuncResult`] per target (success or per-function
/// `error` — a bad function never aborts the batch).  `want_proto`
/// additionally captures the function's prototype line
/// ([`print_c_prototype`]) inside the same panic guard as the C render — the
/// `decompile-project` `.h` surface. `want_provenance` runs the markup emitter
/// after the plain render and resolves its token references against the IR.
///
/// The eager form of [`decompile_pulled`], with `single_target` derived from the
/// target count.
pub fn decompile_targets(
    prog: &mut ConsoleProgram,
    targets: Vec<FunctionEntry>,
    no_vars: bool,
    want_proto: bool,
    want_provenance: bool,
) -> Vec<FuncResult> {
    let opts = DecompileOptions {
        no_vars,
        want_proto,
        want_provenance,
        want_callee_hints: false,
        single_target: targets.len() == 1,
    };
    let mut out = Vec::with_capacity(targets.len());
    let mut pending = targets.into_iter();
    decompile_pulled(prog, &opts, &mut || pending.next(), &mut |r| out.push(r));
    out
}

/// One target's decompile, for a caller holding a single [`FunctionEntry`].
pub fn decompile_entry(
    prog: &mut ConsoleProgram,
    target: FunctionEntry,
    opts: &DecompileOptions,
) -> FuncResult {
    let mut pending = Some(target);
    let mut out = None;
    decompile_pulled(prog, opts, &mut || pending.take(), &mut |r| out = Some(r));
    out.expect("decompile_pulled emits exactly one result per target")
}

/// The pull/push form of [`decompile_targets`]: take targets from `next` until
/// it returns `None`, handing each finished [`FuncResult`] to `sink` before the
/// next target is decompiled.
///
/// Exactly one result per target, in the order `next` produced them — the
/// streaming project export consumes results as they land instead of waiting for
/// the whole batch, and feeds `next` from a scheduler that the results
/// themselves steer ([`FuncResult::callee_hints`]).
///
/// The callee-hint context (the program's function-entry set and its code
/// ranges) is built once per call, on the first target, and only when
/// `opts.want_callee_hints` is set.
pub fn decompile_pulled(
    prog: &mut ConsoleProgram,
    opts: &DecompileOptions,
    next: &mut dyn FnMut() -> Option<FunctionEntry>,
    sink: &mut dyn FnMut(FuncResult),
) {
    let DecompileOptions { no_vars, want_proto, want_provenance, single_target, .. } = *opts;
    let mut hints: Option<CalleeHintContext> = None;
    while let Some(FunctionEntry {
        name,
        addr: entry,
        aliases,
        size,
        object_location,
        provenance,
        ..
    }) = next()
    {
        if opts.want_callee_hints && hints.is_none() {
            hints = Some(CalleeHintContext::new(prog));
        }
        let byte_address = entry.get_offset();
        let address = prog.output_code_offset(byte_address);
        // (kuna) An entry with no mapped bytes is an EXTERNAL, not a decompile
        // failure: a relocatable object's undefined symbols (and a PE import
        // slot) carry an address only so a call to one renders by name, and the
        // definition lives in another module. The whole-binary surfaces never
        // reach one (`function_entries_executable` drops them), but selecting one
        // by name or address — clicking its row in the browser inventory — used to
        // run the lifter against unmapped memory and report the resulting
        // "Unable to load 512 bytes at ..." as if the function had failed to
        // decompile. Say what it actually is instead. See
        // `ConsoleProgram::entry_bytes_mapped`.
        if !prog.entry_bytes_mapped(&entry) && provenance == EntryProvenance::UndefinedExternal {
            sink(FuncResult {
                code: Some(format!(
                    "// {name}: external symbol -- no code at this address in this module\n"
                )),
                name,
                address,
                byte_address,
                size: size as i64,
                error: None,
                proto: None,
                variables: Vec::new(),
                line_mappings: Vec::new(),
                aliases,
                object_location,
                callee_hints: Vec::new(),
            });
            continue;
        }
        if !prog.entry_bytes_mapped(&entry) {
            sink(FuncResult {
                name,
                address,
                byte_address,
                size: size as i64,
                code: None,
                error: Some("entry address is not mapped in this input".into()),
                proto: None,
                variables: Vec::new(),
                line_mappings: Vec::new(),
                aliases,
                object_location,
                callee_hints: Vec::new(),
            });
            continue;
        }
        // Mirror IfcDecompile: re-seed this function's DWARF stack locals (so a
        // `-g` binary renders DWARF names/types) and decompile.  The drive itself
        // catches un-ported-seam panics and returns Err, so a single bad function
        // degrades to an `error` record instead of aborting the binary.
        let mapped = prog.dwarf_locals_for(byte_address);
        // (kuna, Ghidra-gap) `CALL_RETURN` flow overrides for the binary's
        // `call error(nonzero,…)` sites — prune the fall-through so the flow-follower
        // stops at the no-return call (Ghidra "Subroutine does not return") instead of
        // walking into the next function and absorbing it. The whole binary's list is
        // passed; only sites this function's flow actually visits are applied. Empty
        // unless the Listing + `noreturn_error` are on (so `kuna functions`/console are
        // unaffected).
        let flow_ovr: Vec<(kuna_base::address::Address, kuna_base::types::uint4)> =
            match entry.get_space() {
                Some(space) if !prog.arch().error_noreturn_callsites.is_empty() => prog
                    .arch()
                    .error_noreturn_callsites
                    .iter()
                    .map(|&off| {
                        (
                            kuna_base::address::Address::new(std::rc::Rc::clone(space), off),
                            kuna_decomp::overrides::flow_type::CALL_RETURN,
                        )
                    })
                    .collect(),
                _ => Vec::new(),
            };
        // (kuna, DIV-66) The SHARED per-function decompile step — the same one
        // `IfcDecompile` runs. Before it existed this loop called the drive
        // directly and so skipped Ghidra's `FormatStringAnalyzer` half-B loop (and
        // its scoped read-only propagation), which meant `--option formatstring on`
        // — named by `--mode aggressive`, and therefore by `auto` under 500 KiB —
        // was a silent no-op on every whole-binary surface. `discovered` is
        // dropped: each function is decompiled exactly once here, so there is no
        // later re-decompile to persist it for.
        // A caller-declared extent (`function bounds` / `kuna --define-function`)
        // bounds this function's flow follow; 0 — the usual case — is the natural,
        // unbounded extent.
        let declared = prog.declared_extent(byte_address);
        // (kuna `--assert`) The caller-declared facts this function is decompiled
        // AGAINST: a `prototype`/`param`/`return` directive is consumed at flow
        // time, so it has to be seeded here rather than applied afterwards. Every
        // field is empty for a run that passed no directive, which is what makes
        // the plane free (`crate::assertions`).
        let seed = crate::assertions::function_seed(prog, &name, &entry, single_target);
        // (kuna `--assert`) A caller-stated `flow` reclassification is appended
        // AFTER the derived no-return prunes, so it wins the map insert at an
        // address both name (`Override::insertFlowOverride` is a map store):
        // what the caller declared outranks what analysis inferred.
        let mut flow_ovr = flow_ovr;
        flow_ovr.extend(seed.flow_overrides.iter().cloned());
        let step = crate::decompile_step::decompile_one(
            prog.arch_mut(),
            &name,
            entry.clone(),
            declared,
            &crate::decompile_step::DecompileSeed {
                mapped_symbols: &mapped,
                usepoint_symbols: &[],
                dynamic_symbols: &[],
                pending_proto: seed.pending_proto.as_ref(),
                flow_overrides: &flow_ovr,
                mapped_params: &seed.mapped_params,
            },
            &[],
        );
        // (kuna `--assert`) The symbol-scoped second pass. `name`/`type` name a
        // LOCAL, and a local does not exist until a decompile has produced it --
        // so the only order in which they can work is decompile, mutate the local
        // scope, decompile again with the mutation carried across (exactly the
        // console's `decompile` / `rename` / `decompile` sequence). Emitted only
        // when such a directive actually bound to this function, so every other
        // invocation keeps its current cost.
        let result = match step.result {
            Ok(mut fd)
                if crate::assertions::has_symbol_scoped(prog, &name, single_target) =>
            {
                if crate::assertions::apply_symbol_scoped(prog, &mut fd, &name, single_target) {
                    let carried = crate::assertions::carried_symbols(&fd);
                    let carried_usepoint = crate::assertions::carried_usepoint_symbols(&fd);
                    crate::decompile_step::decompile_one(
                        prog.arch_mut(),
                        &name,
                        entry,
                        declared,
                        &crate::decompile_step::DecompileSeed {
                            mapped_symbols: &carried,
                            usepoint_symbols: &carried_usepoint,
                            dynamic_symbols: &[],
                            pending_proto: seed.pending_proto.as_ref(),
                            flow_overrides: &flow_ovr,
                            mapped_params: &seed.mapped_params,
                        },
                        &[],
                    )
                    .result
                } else {
                    Ok(fd)
                }
            }
            other => other,
        };
        match result {
            Ok(fd) => {
                // (kuna `--assert`) The flow overrides the follower REFUSED are only
                // known now: `function_seed` recorded `applied` when it seeded them.
                crate::assertions::record_rejected_flow_overrides(
                    prog,
                    &name,
                    single_target,
                    &fd,
                );
                // Render + extract under `catch_unwind`: the decompile drive only
                // guards the pipeline (decompile_drive.rs), so a fail-fast invariant
                // in the printer / type declarator on an exotic recovered function
                // would otherwise abort the WHOLE binary and discard every function
                // already decompiled. Containing it here honors the per-function
                // isolation contract (one bad function → one `error` record).
                let rendered = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    // Trim the surrounding newlines the same way `kuna decompile`
                    // does (`decompile.rs::trim_newlines`), so the per-function
                    // `code` is byte-identical to the single-shot path.
                    let (untrimmed, provenance) = if want_provenance {
                        print_c_with_provenance(prog.arch_mut(), &fd)
                    } else {
                        (print_c(prog.arch_mut(), &fd), Default::default())
                    };
                    let code = untrimmed.trim_matches('\n').to_string();
                    let mut variables =
                        if no_vars { Vec::new() } else { extract_variables(prog.arch(), &fd) };
                    provenance.apply_to_variables(&fd, &mut variables);
                    for variable in &mut variables {
                        for address in &mut variable.addresses {
                            *address = prog.output_code_offset(*address);
                        }
                    }
                    // The prototype must be captured HERE (fd is dropped at the
                    // end of the iteration) and inside the same guard (the
                    // declarator walk shares the printer's fail-fast invariants).
                    let proto = if want_proto {
                        Some(print_c_prototype(prog.arch_mut(), &fd))
                    } else {
                        None
                    };
                    let mut line_mappings = provenance.line_mappings;
                    for mapping in &mut line_mappings {
                        for address in &mut mapping.addresses {
                            *address = prog.output_code_offset(*address);
                        }
                    }
                    let callee_hints = hints
                        .as_ref()
                        .map(|ctx| ctx.scan(&fd, byte_address))
                        .unwrap_or_default();
                    (code, variables, proto, line_mappings, callee_hints)
                }));
                match rendered {
                    Ok((code, variables, proto, line_mappings, callee_hints)) => sink(FuncResult {
                        name,
                        address,
                        byte_address,
                        size: size as i64,
                        code: Some(code),
                        error: None,
                        proto,
                        variables,
                        line_mappings,
                        aliases,
                        object_location,
                        callee_hints,
                    }),
                    Err(_) => sink(FuncResult {
                        name,
                        address,
                        byte_address,
                        size: size as i64,
                        code: None,
                        error: Some("panic while rendering C / extracting variables".into()),
                        proto: None,
                        variables: Vec::new(),
                        line_mappings: Vec::new(),
                        aliases,
                        object_location,
                        callee_hints: Vec::new(),
                    }),
                }
            }
            Err(e) => sink(FuncResult {
                name,
                address,
                byte_address,
                size: size as i64,
                code: None,
                error: Some(e.explain().to_string()),
                proto: None,
                variables: Vec::new(),
                line_mappings: Vec::new(),
                aliases,
                object_location,
                callee_hints: Vec::new(),
            }),
        }
    }
}

/// The per-batch facts [`FuncResult::callee_hints`] is filtered against: which
/// addresses are function entries, which of them live in code, and how wide a
/// code pointer is.
///
/// Owned rather than borrowed from the program, so the scan can run inside the
/// render guard while the printer holds `prog` mutably.  Built once per
/// [`decompile_pulled`] call: the entry set is the canonical inventory's address
/// set (`ConsoleProgram::find_entry_at` answers from the same set, but rebuilds
/// the whole inventory on every probe, which a per-constant test cannot afford).
struct CalleeHintContext {
    arm: bool,
    /// Every function-entry address, Thumb-bit normalized and sorted — a whole
    /// image's worth, probed once per constant varnode.
    entries: Vec<u64>,
    /// `(start, end)` of each code range, or empty when the container publishes
    /// none — in which case being a known entry is the whole test.
    code: Vec<(u64, u64)>,
    pointer_size: i32,
}

impl CalleeHintContext {
    fn new(prog: &ConsoleProgram) -> Self {
        let arm = prog.description().starts_with("ARM");
        let normalize = |vma: u64| if arm { vma & !1 } else { vma };
        let mut entries: Vec<u64> = prog
            .function_entries()
            .filter(|(_, addr)| addr.get_space().is_some())
            .map(|(_, addr)| normalize(addr.get_offset()))
            .collect();
        entries.sort_unstable();
        entries.dedup();
        // Sections first, segments where the container publishes no section
        // table (a sectionless ELF, the XML corpus loader): the same fallback
        // order the executable-entry filter uses.
        let mut code: Vec<(u64, u64)> = code_ranges(&prog.sections());
        if code.is_empty() {
            code = code_ranges(&prog.segments());
        }
        let pointer_size = prog
            .arch()
            .manage()
            .get_default_code_space()
            .map_or(0, |space| space.get_addr_size() as i32);
        Self { arm, entries, code, pointer_size }
    }

    fn normalize(&self, vma: u64) -> u64 {
        if self.arm {
            vma & !1
        } else {
            vma
        }
    }

    /// Is `vma` a function entry that code could plausibly be reached at?
    fn is_code_entry(&self, vma: u64) -> bool {
        self.entries.binary_search(&vma).is_ok()
            && (self.code.is_empty()
                || self.code.iter().any(|&(start, end)| vma >= start && vma < end))
    }

    /// The hints for one decompiled function: direct-call callees, plus every
    /// constant that names a code entry (an address-taken callee).  `entry` is
    /// the function's own byte address, which is never its own hint.
    fn scan(&self, fd: &Funcdata, entry: u64) -> Vec<u64> {
        let mut out: BTreeSet<u64> = BTreeSet::new();
        for i in 0..fd.num_calls() {
            let spec = fd.get_call_specs(i);
            let Some(op) = fd.obank().get(spec.get_op()) else { continue };
            // A CALLIND's call spec carries a spaceless, invalid entry address.
            if op.code() != OpCode::CPUI_CALL {
                continue;
            }
            let callee = spec.get_entry_address();
            if callee.get_space().is_some() {
                out.insert(self.normalize(callee.get_offset()));
            }
        }
        if self.pointer_size > 0 {
            for id in fd.vbank().iter_loc() {
                let Some(vn) = fd.vbank().get(id) else { continue };
                if !vn.is_constant() || vn.get_size() <= 0 || vn.get_size() > self.pointer_size {
                    continue;
                }
                let vma = self.normalize(vn.get_offset());
                if self.is_code_entry(vma) {
                    out.insert(vma);
                }
            }
        }
        out.remove(&self.normalize(entry));
        out.into_iter().collect()
    }
}

fn code_ranges(spans: &[(u64, u64, u32)]) -> Vec<(u64, u64)> {
    spans
        .iter()
        .filter(|(_, _, flags)| flags & section_flags::CODE != 0)
        .map(|&(vma, size, _)| (vma, vma.saturating_add(size)))
        .collect()
}

/// Render the functions as concatenated C with `// Function:` headers (the human
/// output, mirroring `DecompilationResult.to_c_file`).
pub fn render_c(funcs: &[FuncResult]) -> String {
    let mut out = String::new();
    for f in funcs {
        match (&f.code, &f.error) {
            (Some(code), _) => {
                out.push_str(&format!("// Function: {} @ 0x{:x}\n", f.name, f.address));
                out.push_str(code);
                out.push_str("\n\n");
            }
            (None, Some(err)) => {
                out.push_str(&format!(
                    "// Function: {} @ 0x{:x}  (error: {})\n\n",
                    f.name, f.address, err
                ));
            }
            (None, None) => {}
        }
    }
    out
}

// --- .h ------------------------------------------------------------------

/// The include-guard macro: the file name sanitized to `[A-Z0-9_]` + `_H`
/// (a leading digit gets a `_` prefix — a macro name can't start with one).
pub fn sanitize_guard(file_name: &str) -> String {
    let mut s: String = file_name
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c.to_ascii_uppercase() } else { '_' })
        .collect();
    if s.chars().next().is_none_or(|c| c.is_ascii_digit()) {
        s.insert(0, '_');
    }
    s.push_str("_H");
    s
}

/// `<name>.h`: include guard + recompile prelude + user type definitions +
/// the prototype section (successes in address order; failures as comments).
pub fn build_header(file_name: &str, prelude: &str, types: &str, results: &[FuncResult]) -> String {
    let guard = sanitize_guard(file_name);
    let mut out = String::new();
    out.push_str(&format!("#ifndef {guard}\n#define {guard}\n\n"));
    out.push_str(prelude);
    if !types.is_empty() {
        out.push_str("\n/* user-defined types */\n");
        out.push_str(types);
    }
    out.push_str("\n/* function prototypes */\n");
    for r in results {
        match (&r.proto, &r.error) {
            (Some(proto), None) => {
                out.push_str(proto);
                if !proto.ends_with('\n') {
                    out.push('\n');
                }
            }
            _ => {
                out.push_str(&format!(
                    "/* {} @ 0x{:x}: decompile failed */\n",
                    r.name, r.address
                ));
            }
        }
    }
    out.push_str(&format!("\n#endif /* {guard} */\n"));
    out
}

// --- .c ------------------------------------------------------------------

/// `<name>.c`: the header include + the exact `decompile-all` concatenated-C
/// rendering (`render_c` — `// Function: <name> @ <addr>` blocks, failures as
/// `(error: …)` comments).
pub fn build_c(file_name: &str, results: &[FuncResult]) -> String {
    format!("#include \"{file_name}.h\"\n\n{}", render_c(results))
}

// --- dat_ collection ------------------------------------------------------

/// Scan every decompiled function's C for `dat_<hex>` tokens — the names the
/// printer generates on the fly (`kuna_global_data_name`, `dat_{off:x}`,
/// lowercase hex) for data addresses no symbol covers.  Hand-rolled (no regex
/// dep): the char before `dat_` must not be an identifier char, the token is
/// `dat_` + 1+ lowercase-hex chars, and the char after the hex run must not
/// be an identifier char either (so a user symbol like `dat_foo` or
/// `dat_12x3` never false-positives — the printer's tokens are exact by
/// construction). Returns the presentation coordinates exactly as printed;
/// they are not engine byte offsets on a word-addressed target.
pub fn collect_dat_addrs(results: &[FuncResult]) -> BTreeSet<u64> {
    fn is_ident(b: u8) -> bool {
        b.is_ascii_alphanumeric() || b == b'_'
    }
    let mut out = BTreeSet::new();
    for r in results {
        let Some(code) = &r.code else { continue };
        let b = code.as_bytes();
        let mut i = 0;
        while let Some(pos) = code[i..].find("dat_") {
            let start = i + pos;
            let hex_start = start + 4;
            let mut hex_end = hex_start;
            while hex_end < b.len() && matches!(b[hex_end], b'0'..=b'9' | b'a'..=b'f') {
                hex_end += 1;
            }
            let prev_ok = start == 0 || !is_ident(b[start - 1]);
            let next_ok = hex_end == b.len() || !is_ident(b[hex_end]);
            if prev_ok && next_ok && hex_end > hex_start {
                if let Ok(vma) = u64::from_str_radix(&code[hex_start..hex_end], 16) {
                    out.insert(vma);
                }
            }
            i = hex_end.max(hex_start);
        }
    }
    out
}

// --- .asm ------------------------------------------------------------------

/// Reusable storage for the full-section disassembly sweep.
pub(crate) struct AssemblyScratch {
    pub(crate) mnem: String,
    pub(crate) body: String,
    pub(crate) raw: Vec<u8>,
    line: String,
    db_start: Option<u64>,
    db: Vec<u8>,
}

impl AssemblyScratch {
    pub(crate) fn new() -> Self {
        Self {
            mnem: String::with_capacity(16),
            body: String::with_capacity(64),
            raw: Vec::with_capacity(16),
            line: String::with_capacity(128),
            db_start: None,
            db: Vec::with_capacity(8),
        }
    }

    pub(crate) fn emit_instruction(&mut self, addr: u64, out: &mut String) {
        self.line.clear();
        write!(&mut self.line, "  {addr:08x}: ").unwrap();
        for (idx, &byte) in self.raw.iter().enumerate() {
            if idx != 0 {
                self.line.push(' ');
            }
            push_lower_hex_byte(&mut self.line, byte);
        }
        let raw_width = self.raw.len().saturating_mul(3).saturating_sub(1);
        for _ in raw_width..24 {
            self.line.push(' ');
        }
        self.line.push_str("  ");
        self.line.push_str(&self.mnem);
        for _ in self.mnem.chars().count()..10 {
            self.line.push(' ');
        }
        self.line.push(' ');
        self.line.push_str(&self.body);
        self.line.truncate(self.line.trim_end().len());
        self.line.push('\n');
        out.push_str(&self.line);
    }

    pub(crate) fn push_db(&mut self, addr: u64, byte: u8, out: &mut String) {
        if self.db_start.is_none() {
            self.db_start = Some(addr);
        }
        self.db.push(byte);
        if self.db.len() == 8 {
            self.flush_db(out);
        }
    }

    pub(crate) fn flush_db(&mut self, out: &mut String) {
        let Some(addr) = self.db_start.take() else { return };
        self.line.clear();
        write!(&mut self.line, "  {addr:08x}: db ").unwrap();
        for (idx, &byte) in self.db.iter().enumerate() {
            if idx != 0 {
                self.line.push_str(", ");
            }
            self.line.push_str("0x");
            push_lower_hex_byte(&mut self.line, byte);
        }
        self.line.push('\n');
        out.push_str(&self.line);
        self.db.clear();
    }

    pub(crate) fn emit_unreadable(&mut self, addr: u64, out: &mut String) {
        self.line.clear();
        writeln!(&mut self.line, "  {addr:08x}: db ?? (unreadable)").unwrap();
        out.push_str(&self.line);
    }
}

fn push_lower_hex_byte(out: &mut String, byte: u8) {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    out.push(HEX[(byte >> 4) as usize] as char);
    out.push(HEX[(byte & 0xf) as usize] as char);
}

/// `<name>.asm`: linear sweep of every CODE section with function labels +
/// stack-var comment blocks, then the `; --- data ---` tail (named globals ∪
/// the `dat_<hex>` set, with raw bytes).
///
/// The one-shot form of [`crate::project_stream::AsmSweep`], run to completion
/// with an unbounded budget.
pub fn build_asm(
    prog: &ConsoleProgram,
    results: &[FuncResult],
    dat_addrs: &BTreeSet<u64>,
    file_name: &str,
) -> String {
    let labels = crate::project_stream::asm_labels_from_results(results);
    let mut sweep = crate::project_stream::AsmSweep::new(prog, file_name, labels);
    let mut buf: Vec<u8> = Vec::new();
    buf.extend_from_slice(sweep.header_lines().as_bytes());
    while !sweep.step(prog, u64::MAX, &mut buf).expect("a Vec sink cannot fail") {}
    let mut out = String::from_utf8(buf).expect("the sweep emits UTF-8");
    emit_data_tail(prog, &prog.sections(), dat_addrs, &mut out);
    out
}

#[cfg(test)]
mod tests {
    use super::AssemblyScratch;

    fn legacy_instruction_line(addr: u64, raw: &[u8], mnem: &str, body: &str) -> String {
        let hex = raw.iter().map(|b| format!("{b:02x}")).collect::<Vec<_>>().join(" ");
        let line = format!("  {addr:08x}: {hex:<24}  {mnem:<10} {body}");
        format!("{}\n", line.trim_end())
    }

    #[test]
    fn assembly_scratch_formats_exactly_without_reallocating() {
        let mut scratch = AssemblyScratch::new();
        let capacities = (
            scratch.mnem.capacity(),
            scratch.body.capacity(),
            scratch.raw.capacity(),
            scratch.line.capacity(),
            scratch.db.capacity(),
        );
        let pointers = (
            scratch.mnem.as_ptr(),
            scratch.body.as_ptr(),
            scratch.raw.as_ptr(),
            scratch.line.as_ptr(),
            scratch.db.as_ptr(),
        );
        let mut out = String::new();

        scratch.mnem.push_str("PUSH");
        scratch.body.push_str("RBP");
        scratch.raw.push(0x55);
        scratch.emit_instruction(0x40071d, &mut out);

        scratch.mnem.clear();
        scratch.body.clear();
        scratch.raw.clear();
        scratch.mnem.push_str("RET");
        scratch.raw.push(0xc3);
        scratch.emit_instruction(0x40071e, &mut out);

        for (idx, byte) in [0x00, 0x0f, 0xa5, 0xff].into_iter().enumerate() {
            scratch.push_db(0x400720 + idx as u64, byte, &mut out);
        }
        scratch.flush_db(&mut out);

        assert_eq!(
            out,
            concat!(
                "  0040071d: 55                        PUSH       RBP\n",
                "  0040071e: c3                        RET\n",
                "  00400720: db 0x00, 0x0f, 0xa5, 0xff\n",
            )
        );
        assert_eq!(
            capacities,
            (
                scratch.mnem.capacity(),
                scratch.body.capacity(),
                scratch.raw.capacity(),
                scratch.line.capacity(),
                scratch.db.capacity(),
            )
        );
        assert_eq!(
            pointers,
            (
                scratch.mnem.as_ptr(),
                scratch.body.as_ptr(),
                scratch.raw.as_ptr(),
                scratch.line.as_ptr(),
                scratch.db.as_ptr(),
            )
        );
    }

    #[test]
    fn assembly_scratch_matches_legacy_format_edge_cases() {
        let cases: &[(&[u8], &str, &str)] = &[
            (&[], "", ""),
            (&[0x00, 0x01, 0x7f, 0x80, 0xfe, 0xff], "LOAD", "R0,0xff "),
            (
                &[0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99],
                "TENLETTERS",
                "",
            ),
            (&[0xab], "é", "operand"),
        ];
        let mut scratch = AssemblyScratch::new();
        for (idx, &(raw, mnem, body)) in cases.iter().enumerate() {
            scratch.raw.clear();
            scratch.raw.extend_from_slice(raw);
            scratch.mnem.clear();
            scratch.mnem.push_str(mnem);
            scratch.body.clear();
            scratch.body.push_str(body);
            let mut actual = String::new();
            let addr = 0x400700 + idx as u64;
            scratch.emit_instruction(addr, &mut actual);
            assert_eq!(actual, legacy_instruction_line(addr, raw, mnem, body));
        }
    }
}

/// The per-function variable comment block: `; arg:` lines (ABI order), then
/// `; stack:` lines with the signed frame offset the decompiled name lives at.
/// Empty when the function recovered no variables.
pub(crate) fn var_header_lines(r: &FuncResult) -> String {
    let mut out = String::new();
    emit_var_header(r, &mut out);
    out
}

fn emit_var_header(r: &FuncResult, out: &mut String) {
    for v in r.variables.iter().filter(|v| v.is_param) {
        out.push_str(&format!("; arg:   {} ({})\n", v.name, v.type_name));
    }
    for v in r.variables.iter().filter(|v| !v.is_param) {
        match v.stack_offset {
            Some(off) => {
                let sign = if off < 0 { '-' } else { '+' };
                out.push_str(&format!(
                    "; stack: {} @ [stack{}0x{:x}] ({})\n",
                    v.name,
                    sign,
                    off.unsigned_abs(),
                    v.type_name
                ));
            }
            None => out.push_str(&format!("; stack: {} ({})\n", v.name, v.type_name)),
        }
    }
}

/// One data-tail label: display name, byte size if a typed symbol supplied
/// one, and whether a `dat_<hex>` token also resolves here (the alias case:
/// the named symbol stays the label, the `dat_` spelling is appended so the
/// `.c` token remains greppable — `<name>:  ; 0x<addr> = dat_<hex>`).
struct DataLabel {
    name: String,
    type_size: Option<i64>,
    dat_alias: bool,
    address: Option<Address>,
}

/// `; --- data ---`: address-sorted deduped labels (named globals ∪ `dat_`
/// tokens), each with a raw-byte dump (16/line + printable-ASCII column) or an
/// `?? (uninitialized/unmapped)` line for image-backed-less ranges (.bss).
pub(crate) fn emit_data_tail(
    prog: &ConsoleProgram,
    sections: &[(u64, u64, u32)],
    dat_addrs: &BTreeSet<u64>,
    out: &mut String,
) {
    let mut data: BTreeMap<(u64, i32), DataLabel> = BTreeMap::new();
    for (name, address, type_size) in prog.global_data_symbol_addresses() {
        let display_vma = prog.output_address_offset(&address);
        let space_index = address.get_space().map_or(i32::MAX, |space| space.get_index());
        // First named symbol at a displayed address wins (global_data_symbols
        // is address/name-sorted; duplicates at one address are aliases).
        data.entry((display_vma, space_index)).or_insert(DataLabel {
            name,
            type_size: Some(type_size),
            dat_alias: false,
            address: Some(address),
        });
    }
    let data_space_index = prog
        .arch()
        .manage()
        .get_default_data_space()
        .map_or(i32::MAX, |space| space.get_index());
    for &display_vma in dat_addrs {
        data.entry((display_vma, data_space_index))
            .and_modify(|l| l.dat_alias = true)
            .or_insert_with(|| DataLabel {
                name: format!("dat_{display_vma:x}"),
                type_size: None,
                dat_alias: false,
                address: None,
            });
    }
    if data.is_empty() {
        return;
    }

    out.push_str("\n; --- data ---\n");
    for (&(display_vma, _space_index), label) in &data {
        let byte_vma = label.address.as_ref().map_or(display_vma, Address::get_offset);
        // Size: a typed symbol's datatype size; a bare `dat_` gets
        // min(gap to the next label / containing-section end, 32), floor 1.
        let size = match label.type_size {
            Some(s) if s > 0 => s as u64,
            _ => {
                let next_label = data
                    .range((display_vma.saturating_add(1), i32::MIN)..)
                    .next()
                    .map(|(&(addr, _), _)| addr);
                let sec_end = sections
                    .iter()
                    .find(|&&(sv, ss, _)| byte_vma >= sv && byte_vma < sv.saturating_add(ss))
                    .map(|&(sv, ss, _)| sv.saturating_add(ss))
                    .map(|byte_end| {
                        label
                            .address
                            .as_ref()
                            .and_then(Address::get_space)
                            .cloned()
                            .map_or(byte_end, |space| {
                                prog.output_address_offset(&Address::new(space, byte_end))
                            })
                    });
                let bound = [next_label, sec_end]
                    .into_iter()
                    .flatten()
                    .filter(|&b| b > display_vma)
                    .map(|b| b - display_vma)
                    .min()
                    .unwrap_or(DAT_SIZE_CAP);
                bound.clamp(1, DAT_SIZE_CAP)
            }
        };
        out.push('\n');
        if label.dat_alias {
            out.push_str(&format!(
                "{}:  ; 0x{display_vma:x} = dat_{display_vma:x}\n",
                label.name
            ));
        } else {
            out.push_str(&format!("{}:  ; 0x{display_vma:x}\n", label.name));
        }
        let bytes = match label.address.as_ref() {
            Some(address) => prog.read_bytes_at(address, size as usize),
            None => prog.read_bytes(byte_vma, size as usize),
        };
        match bytes {
            Some(bytes) => {
                for (row, chunk) in bytes.chunks(16).enumerate() {
                    let hex =
                        chunk.iter().map(|b| format!("{b:02x}")).collect::<Vec<_>>().join(" ");
                    let ascii: String = chunk
                        .iter()
                        .map(|&b| if (0x20..0x7f).contains(&b) { b as char } else { '.' })
                        .collect();
                    let byte_offset = byte_vma + row as u64 * 16;
                    let display_offset = match label.address.as_ref() {
                        Some(address) => {
                            let space = address
                                .get_space()
                                .cloned()
                                .expect("mapped data label has an address space");
                            let row_address = Address::new(space, byte_offset);
                            prog.output_address_offset(&row_address)
                        }
                        None => display_vma + row as u64 * 16,
                    };
                    out.push_str(&format!("  {display_offset:08x}: {hex:<47}  |{ascii}|\n"));
                }
            }
            None => {
                out.push_str(&format!(
                    "  {display_vma:08x}: ?? (uninitialized/unmapped, {size} bytes)\n"
                ));
            }
        }
    }
}

// --- README.md ---------------------------------------------------------------

/// One row of the README's section table.
#[derive(Debug, Clone)]
pub struct ReadmeSectionRow {
    pub name: String,
    pub vma: u64,
    pub size: u64,
    pub kind: String,
}

/// Everything the README says about the binary, as plain owned data.
///
/// Snapshotted from the `ConsoleProgram` on the thread that owns it
/// ([`Self::snapshot`]), so a streamed export can keep rewriting its README from
/// a writer thread that never sees the program.
#[derive(Debug, Clone, Default)]
pub struct ReadmeFacts {
    /// The path string printed in the `| Path |` row (the CLI passes the
    /// canonicalized on-disk path; the wasm front-end a virtual name).
    pub path_label: String,
    /// The base name the artifacts are named after.
    pub file_name: String,
    /// `None` when the file could not be stat'd, which prints as `?`.
    pub file_size: Option<u64>,
    /// The loader's description line.  `None` before the program is loaded, and
    /// every program-derived row then prints `pending`.
    pub description: Option<String>,
    /// The image entry point, resolved through the inventory so an ARM entry
    /// carrying the Thumb mode bit prints at the even address the rest of the
    /// export uses.
    pub entry: Option<u64>,
    pub sections: Vec<ReadmeSectionRow>,
}

impl ReadmeFacts {
    /// The facts known before the program is loaded: the path and its size.
    pub fn pending(binary_path: &Path, path_label: &str, file_name: &str) -> Self {
        Self {
            path_label: path_label.to_string(),
            file_name: file_name.to_string(),
            file_size: std::fs::metadata(binary_path).map(|m| m.len()).ok(),
            description: None,
            entry: None,
            sections: Vec::new(),
        }
    }

    /// The facts a loaded program carries: [`Self::pending`] plus the
    /// architecture, entry point and section table.
    pub fn snapshot(
        binary_path: &Path,
        path_label: &str,
        file_name: &str,
        prog: &ConsoleProgram,
    ) -> Self {
        let metadata = prog.image_metadata();
        Self {
            description: Some(prog.description().to_string()),
            entry: metadata
                .and_then(|value| value.entry)
                .map(|vma| prog.find_entry_at(vma).map_or(vma, |entry| entry.addr.get_offset())),
            sections: metadata
                .map(|value| {
                    value
                        .sections
                        .iter()
                        .map(|section| ReadmeSectionRow {
                            name: section.name.clone(),
                            vma: section.vma,
                            size: section.size,
                            kind: section.kind.to_string(),
                        })
                        .collect()
                })
                .unwrap_or_default(),
            ..Self::pending(binary_path, path_label, file_name)
        }
    }
}

/// The README's function tally.
#[derive(Debug, Clone, Copy, Default)]
pub struct ReadmeCounts {
    pub total: usize,
    pub ok: usize,
    pub failed: usize,
}

impl ReadmeCounts {
    pub fn of(results: &[FuncResult]) -> Self {
        let ok = results.iter().filter(|r| r.error.is_none()).count();
        Self { total: results.len(), ok, failed: results.len() - ok }
    }
}

/// Which export the README describes: the artifacts differ, so the file
/// inventory does too.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReadmeLayout {
    /// A whole-binary export written in one go.
    Standard,
    /// A `--stream` export: decompile-ordered `.c`, `.asm` variable comments in
    /// their own section, plus `index.jsonl` and `.streaming`.
    Streamed,
}

/// Which phase a streamed export is in — the `phase` field of `.streaming`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StreamPhase {
    /// Loading and analyzing the binary; nothing but the README exists yet.
    #[default]
    Loading,
    /// Functions are being decompiled and appended.
    Decompiling,
    /// Every function is in; the type block and the `.asm` tails are being written.
    Finalizing,
    /// The run gave up; `error` says why.
    Failed,
}

impl StreamPhase {
    pub fn as_str(self) -> &'static str {
        match self {
            StreamPhase::Loading => "loading",
            StreamPhase::Decompiling => "decompiling",
            StreamPhase::Finalizing => "finalizing",
            StreamPhase::Failed => "failed",
        }
    }
}

/// How far the `.asm` sweep has got — the `asm` field of `.streaming`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AsmPhase {
    #[default]
    Pending,
    Sweeping,
    Complete,
}

impl AsmPhase {
    pub fn as_str(self) -> &'static str {
        match self {
            AsmPhase::Pending => "pending",
            AsmPhase::Sweeping => "sweeping",
            AsmPhase::Complete => "complete",
        }
    }
}

/// A streamed export's progress, as the in-progress README and `.streaming`
/// report it.
#[derive(Debug, Clone, Default)]
pub struct StreamProgress {
    pub phase: StreamPhase,
    /// Worker processes actually in use (1 when the run is serial).
    pub jobs: usize,
    /// Targets selected; `None` until the load resolves them.
    pub total: Option<usize>,
    /// Results written so far, failures included.
    pub done: usize,
    /// How many of `done` are error records.
    pub failed: usize,
    pub asm: AsmPhase,
    pub error: Option<String>,
    pub elapsed_s: u64,
}

impl StreamProgress {
    fn counts(&self) -> Option<ReadmeCounts> {
        self.total.map(|total| ReadmeCounts {
            total,
            ok: self.done.saturating_sub(self.failed),
            failed: self.failed,
        })
    }
}

/// `README.md`: binary metadata (file size, arch id, entry point + sections),
/// function counts, and the artifact inventory / labeling conventions.
///
/// `path_label` is the path string printed in the `| Path |` row (the CLI
/// passes `binary_path.display()`; the wasm front-end a virtual name);
/// `binary_path` itself is read for the file size.
pub fn build_readme(
    binary_path: &Path,
    path_label: &str,
    file_name: &str,
    prog: &ConsoleProgram,
    results: &[FuncResult],
) -> String {
    let facts = ReadmeFacts::snapshot(binary_path, path_label, file_name, prog);
    render_readme(&facts, Some(ReadmeCounts::of(results)), ReadmeLayout::Standard)
}

/// Render a README from snapshotted facts.  `counts` is `None` while the target
/// set is still unknown, which prints the function tally as `pending`.
pub fn render_readme(
    facts: &ReadmeFacts,
    counts: Option<ReadmeCounts>,
    layout: ReadmeLayout,
) -> String {
    render_readme_inner(facts, counts, layout, None)
}

/// The README of an export that is still running: [`render_readme`] with a
/// banner and a streaming-status section around it.
pub fn render_readme_streaming(facts: &ReadmeFacts, progress: &StreamProgress) -> String {
    let mut out = render_readme_inner(
        facts,
        progress.counts(),
        ReadmeLayout::Streamed,
        Some(
            "**This export is still streaming** — `.streaming` exists until every artifact is \
             final; files are appended / rewritten while it does.",
        ),
    );
    out.push_str(&render_streaming_status(facts, progress));
    out
}

fn render_readme_inner(
    facts: &ReadmeFacts,
    counts: Option<ReadmeCounts>,
    layout: ReadmeLayout,
    banner: Option<&str>,
) -> String {
    let file_name = &facts.file_name;
    let file_size = facts.file_size.map_or_else(|| "?".to_string(), |size| size.to_string());

    let mut out = String::new();
    out.push_str(&format!("# {file_name} — kuna project export\n\n"));
    if let Some(banner) = banner {
        out.push_str(banner);
        out.push_str("\n\n");
    }
    match layout {
        ReadmeLayout::Standard => out.push_str("Generated by `kuna decompile-project`.\n\n"),
        ReadmeLayout::Streamed => {
            out.push_str("Generated by `kuna decompile-project --stream`.\n\n")
        }
    }
    out.push_str("## Binary\n\n");
    out.push_str("| Field | Value |\n|---|---|\n");
    out.push_str(&format!("| Path | `{}` |\n", facts.path_label));
    out.push_str(&format!("| File size | {file_size} bytes |\n"));
    match &facts.description {
        Some(description) => out.push_str(&format!("| Architecture | `{description}` |\n")),
        None => out.push_str("| Architecture | pending |\n"),
    }
    match (&facts.description, facts.entry) {
        (None, _) => out.push_str("| Entry point | pending |\n"),
        (Some(_), Some(entry)) => out.push_str(&format!("| Entry point | `0x{entry:x}` |\n")),
        (Some(_), None) => out.push_str("| Entry point | unavailable |\n"),
    }
    match counts {
        Some(counts) => out.push_str(&format!(
            "| Functions | {} total, {} decompiled, {} failed |\n",
            counts.total, counts.ok, counts.failed
        )),
        None => out.push_str("| Functions | pending |\n"),
    }

    if !facts.sections.is_empty() {
        out.push_str("\n## Sections\n\n");
        out.push_str("| Name | Address | Size | Kind |\n|---|---|---|---|\n");
        for section in &facts.sections {
            out.push_str(&format!(
                "| `{}` | `0x{:x}` | `0x{:x}` | {} |\n",
                section.name, section.vma, section.size, section.kind
            ));
        }
    }

    out.push_str("\n## Files\n\n");
    match layout {
        ReadmeLayout::Standard => out.push_str(&format!(
            "- `{file_name}.c` — every decompiled function, address-ordered, each under a\n\
             \x20 `// Function: <name> @ <addr>` header (failures appear as\n\
             \x20 `// Function: <name> @ <addr>  (error: ...)` comments). Includes `{file_name}.h`.\n"
        )),
        ReadmeLayout::Streamed => out.push_str(&format!(
            "- `{file_name}.c` — every decompiled function, each under a\n\
             \x20 `// Function: <name> @ <addr>` header (failures appear as\n\
             \x20 `// Function: <name> @ <addr>  (error: ...)` comments). Includes `{file_name}.h`.\n\
             \x20 The blocks are in DECOMPILE order, not address order: the entry point and the\n\
             \x20 functions it reaches first, then the rest address-ordered. Under `--jobs N`\n\
             \x20 the interleaving is worker completion order and is not reproducible run to\n\
             \x20 run; `index.jsonl` is the address-to-offset index.\n"
        )),
    }
    out.push_str(&format!(
        "- `{file_name}.h` — recompilation aid: the generated core-typedef prelude (the\n\
         \x20 Ghidra/kuna `undefined` family included), the user-defined type definitions\n\
         \x20 recovered during decompilation, and one prototype per decompiled function\n\
         \x20 (token-identical to its `.c` definition line).\n"
    ));
    match layout {
        ReadmeLayout::Standard => out.push_str(&format!(
            "- `{file_name}.asm` — labeled linear disassembly of every code section. Function\n\
             \x20 labels match the `.c` names exactly (`main:`, `sub_<addr>:`); under each label\n\
             \x20 a comment block maps the decompiled variables to their storage\n\
             \x20 (`; arg: <name> (<type>)`, `; stack: <name> @ [stack-0x18] (<type>)`).\n\
             \x20 Undecodable bytes appear as `db` lines. The `; --- data ---` tail labels the\n\
             \x20 named globals and every `dat_<hex>` address the `.c` references, with raw\n\
             \x20 bytes (`??` for unmapped/.bss ranges).\n"
        )),
        ReadmeLayout::Streamed => out.push_str(&format!(
            "- `{file_name}.asm` — labeled linear disassembly of every code section. Function\n\
             \x20 labels match the `.c` names exactly (`main:`, `sub_<addr>:`); undecodable\n\
             \x20 bytes appear as `db` lines. The disassembly is swept before the decompile\n\
             \x20 finishes, so the variable comment blocks a non-stream export prints under\n\
             \x20 each label are appended in a `; --- variables ---` section instead\n\
             \x20 (`; arg: <name> (<type>)`, `; stack: <name> @ [stack-0x18] (<type>)`), ahead\n\
             \x20 of the `; --- data ---` tail. That tail labels the named globals and every\n\
             \x20 `dat_<hex>` address the `.c` references, with raw bytes (`??` for\n\
             \x20 unmapped/.bss ranges).\n"
        )),
    }
    if layout == ReadmeLayout::Streamed {
        out.push_str(
            "- `index.jsonl` — one JSON object per function, appended after that function's\n\
             \x20 `.c` block is written: `{\"seq\", \"name\", \"addr\", \"size\", \"c_offset\",\n\
             \x20 \"c_len\", \"error\"}`. `c_offset`/`c_len` slice the `.c` to exactly that\n\
             \x20 block, so the line is also the proof the block is complete.\n\
             - `.streaming` — present only while the export is running: a JSON status file\n\
             \x20 (`schema`, `phase`, `pid`, timestamps, counts, `asm`, `error`), rewritten as\n\
             \x20 the run progresses and removed when it succeeds. A `failed` phase records\n\
             \x20 why the run stopped; a `.streaming` left behind with a dead `pid` and no\n\
             \x20 `failed` phase means the run was killed.\n",
        );
    }
    out.push_str(&format!(
        "- `README.md` — this file.\n\
         \n\
         ## Labeling conventions\n\n\
         - Functions without a symbol name keep the generated `sub_<addr>` /\n\
         \x20 `FUN_<addr>` name in both `{file_name}.c` and `{file_name}.asm` — the label in the\n\
         \x20 `.asm` is the anchor for the same function in the `.c`.\n\
         - Unnamed data referenced by the C appears as `dat_<hex>` (the hex is the VMA);\n\
         \x20 the same spelling labels the bytes in the `.asm` data tail. When a named\n\
         \x20 global covers that address the symbol name stays the label and the `dat_`\n\
         \x20 spelling is appended: `<name>:  ; 0x<addr> = dat_<hex>`.\n"
    ));
    out
}

fn render_streaming_status(facts: &ReadmeFacts, progress: &StreamProgress) -> String {
    let file_name = &facts.file_name;
    let mut out = String::from("\n## Streaming status\n\n| Field | Value |\n|---|---|\n");
    out.push_str(&format!("| Phase | {} |\n", progress.phase.as_str()));
    out.push_str(&format!("| Elapsed | {}s |\n", progress.elapsed_s));
    out.push_str(&format!("| Workers | {} |\n", progress.jobs));
    match progress.total {
        Some(total) => out.push_str(&format!(
            "| Functions written | {} of {total} ({} failed) |\n",
            progress.done, progress.failed
        )),
        None => out.push_str("| Functions written | pending |\n"),
    }
    out.push_str(&format!("| Disassembly | {} |\n", progress.asm.as_str()));
    if let Some(error) = &progress.error {
        out.push_str(&format!("| Error | {error} |\n"));
    }
    out.push_str(&format!(
        "\nWhile `.streaming` exists the export is incomplete:\n\n\
         - `{file_name}.c` and `{file_name}.asm` are append-only and may end mid-block; a\n\
         \x20 line in `index.jsonl` is what says a `.c` block is whole.\n\
         - `{file_name}.h` and `README.md` are rewritten whole every time, so they are always\n\
         \x20 readable — but the `.h` carries no type definitions until the export completes,\n\
         \x20 and until then it does not compile against the `.c`.\n\
         - Function blocks already written are never rewritten: a `.c` offset stays valid.\n"
    ));
    out
}
