//! Shared **decompile-project core**: the whole-binary decompile loop
//! ([`decompile_targets`] / [`FuncResult`] / [`render_c`]) and the four
//! project-export artifact builders behind `kuna decompile-project`
//! (`<name>.c` / `<name>.h` / `<name>.asm` / `README.md`).
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

use kuna_decomp::decompile_drive::{
    extract_variables, print_c, print_c_prototype, print_c_with_provenance, LineMapping, VarInfo,
};
// `ConsoleProgram::sections` documents its `flags` word as exactly the
// `kuna_sleigh::loadimage::section_flags` constant set (UNALLOC=1, NOLOAD=2,
// CODE=4, DATA=8, READONLY=16).  kuna-console depends on kuna-sleigh, so the
// real constants are used directly here (kuna-cli, which deliberately does not
// depend on kuna-sleigh, used to mirror the stable, upstream-fixed CODE bit).
use kuna_sleigh::loadimage::section_flags;
use object::{Object, ObjectSection};

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
    pub address: u64,
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
}

/// Decompile each `(name, entry)` target in turn against the already-loaded
/// program, returning one [`FuncResult`] per target (success or per-function
/// `error` — a bad function never aborts the batch).  `want_proto`
/// additionally captures the function's prototype line
/// ([`print_c_prototype`]) inside the same panic guard as the C render — the
/// `decompile-project` `.h` surface. `want_provenance` runs the markup emitter
/// after the plain render and resolves its token references against the IR.
pub fn decompile_targets(
    prog: &mut ConsoleProgram,
    targets: Vec<FunctionEntry>,
    no_vars: bool,
    want_proto: bool,
    want_provenance: bool,
) -> Vec<FuncResult> {
    let mut out = Vec::with_capacity(targets.len());
    // (kuna `--assert`) An unqualified directive binds to "the function under
    // decompile", which is only unambiguous when the run selected exactly one.
    let single_target = targets.len() == 1;
    for FunctionEntry {
        name,
        addr: entry,
        aliases,
        size,
        object_location,
        provenance,
        ..
    } in targets
    {
        let address = entry.get_offset();
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
            out.push(FuncResult {
                code: Some(format!(
                    "// {name}: external symbol -- no code at this address in this module\n"
                )),
                name,
                address,
                size: size as i64,
                error: None,
                proto: None,
                variables: Vec::new(),
                line_mappings: Vec::new(),
                aliases,
                object_location,
            });
            continue;
        }
        if !prog.entry_bytes_mapped(&entry) {
            out.push(FuncResult {
                name,
                address,
                size: size as i64,
                code: None,
                error: Some("entry address is not mapped in this input".into()),
                proto: None,
                variables: Vec::new(),
                line_mappings: Vec::new(),
                aliases,
                object_location,
            });
            continue;
        }
        // Mirror IfcDecompile: re-seed this function's DWARF stack locals (so a
        // `-g` binary renders DWARF names/types) and decompile.  The drive itself
        // catches un-ported-seam panics and returns Err, so a single bad function
        // degrades to an `error` record instead of aborting the binary.
        let mapped = prog.dwarf_locals_for(address);
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
        let declared = prog.declared_extent(address);
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
                    crate::decompile_step::decompile_one(
                        prog.arch_mut(),
                        &name,
                        entry,
                        declared,
                        &crate::decompile_step::DecompileSeed {
                            mapped_symbols: &carried,
                            usepoint_symbols: &[],
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
                    // The prototype must be captured HERE (fd is dropped at the
                    // end of the iteration) and inside the same guard (the
                    // declarator walk shares the printer's fail-fast invariants).
                    let proto = if want_proto {
                        Some(print_c_prototype(prog.arch_mut(), &fd))
                    } else {
                        None
                    };
                    (code, variables, proto, provenance.line_mappings)
                }));
                match rendered {
                    Ok((code, variables, proto, line_mappings)) => out.push(FuncResult {
                        name,
                        address,
                        size: size as i64,
                        code: Some(code),
                        error: None,
                        proto,
                        variables,
                        line_mappings,
                        aliases,
                        object_location,
                    }),
                    Err(_) => out.push(FuncResult {
                        name,
                        address,
                        size: size as i64,
                        code: None,
                        error: Some("panic while rendering C / extracting variables".into()),
                        proto: None,
                        variables: Vec::new(),
                        line_mappings: Vec::new(),
                        aliases,
                        object_location,
                    }),
                }
            }
            Err(e) => out.push(FuncResult {
                name,
                address,
                size: size as i64,
                code: None,
                error: Some(e.explain().to_string()),
                proto: None,
                variables: Vec::new(),
                line_mappings: Vec::new(),
                aliases,
                object_location,
            }),
        }
    }
    out
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
/// construction).  Returns the parsed VMAs.
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

/// A function label in the sweep: the (possibly several — alias names at one
/// address survive `resolve_targets`) results anchored at a normalized VMA.
type LabelMap<'a> = BTreeMap<u64, Vec<&'a FuncResult>>;

/// Reusable storage for the full-section disassembly sweep.
struct AssemblyScratch {
    mnem: String,
    body: String,
    raw: Vec<u8>,
    line: String,
    db_start: Option<u64>,
    db: Vec<u8>,
}

impl AssemblyScratch {
    fn new() -> Self {
        Self {
            mnem: String::with_capacity(16),
            body: String::with_capacity(64),
            raw: Vec::with_capacity(16),
            line: String::with_capacity(128),
            db_start: None,
            db: Vec::with_capacity(8),
        }
    }

    fn emit_instruction(&mut self, addr: u64, out: &mut String) {
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

    fn push_db(&mut self, addr: u64, byte: u8, out: &mut String) {
        if self.db_start.is_none() {
            self.db_start = Some(addr);
        }
        self.db.push(byte);
        if self.db.len() == 8 {
            self.flush_db(out);
        }
    }

    fn flush_db(&mut self, out: &mut String) {
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

    fn emit_unreadable(&mut self, addr: u64, out: &mut String) {
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
pub fn build_asm(
    prog: &ConsoleProgram,
    results: &[FuncResult],
    dat_addrs: &BTreeSet<u64>,
    file_name: &str,
) -> String {
    // On ARM-family specs a Thumb function's entry VMA carries the mode bit;
    // the sweep walks even byte addresses, so labels are keyed on `vma & !1`.
    let arm = prog.description().starts_with("ARM");
    let normalize = |vma: u64| if arm { vma & !1 } else { vma };

    let mut labels: LabelMap = BTreeMap::new();
    for r in results {
        labels.entry(normalize(r.address)).or_default().push(r);
    }

    let sections = prog.sections();
    let mut out = String::new();
    out.push_str(&format!("; kuna decompile-project — {file_name}\n"));
    out.push_str(&format!("; arch: {}\n", prog.description()));

    let mut code_secs: Vec<(u64, u64)> = sections
        .iter()
        .filter(|(_, _, flags)| flags & section_flags::CODE != 0)
        .map(|&(vma, size, _)| (vma, size))
        .collect();
    code_secs.sort_unstable();
    let mut scratch = AssemblyScratch::new();
    for (vma, size) in code_secs {
        let end = vma.saturating_add(size);
        out.push_str(&format!("\n; --- code section 0x{vma:x}..0x{end:x} ---\n"));
        sweep_code(prog, &labels, vma, end, &mut scratch, &mut out);
    }

    emit_data_tail(prog, &sections, dat_addrs, &mut out);
    out
}

/// Disassemble `[start, end)` linearly: labels + stack-var headers at function
/// entries, one instruction line per decode, `db` byte lines (coalesced up to
/// 8, clamped at the next label) where decoding fails.
fn sweep_code(
    prog: &ConsoleProgram,
    labels: &LabelMap,
    start: u64,
    end: u64,
    scratch: &mut AssemblyScratch,
    out: &mut String,
) {
    let mut addr = start;
    while addr < end {
        if let Some(rs) = labels.get(&addr) {
            scratch.flush_db(out);
            for r in rs {
                out.push('\n');
                out.push_str(&format!("{}:  ; 0x{:x}\n", r.name, r.address));
                emit_var_header(r, out);
            }
        }
        // Never decode across the next label (a mis-sync would otherwise
        // swallow a function label into an instruction's byte extent) or the
        // section end.
        let next_stop =
            labels.range(addr + 1..).next().map(|(&a, _)| a.min(end)).unwrap_or(end);
        match prog.disassemble_at_into(addr, &mut scratch.mnem, &mut scratch.body) {
            Ok(len) if len > 0 && addr + len as u64 <= next_stop => {
                scratch.flush_db(out);
                prog.read_bytes_into(addr, len as usize, &mut scratch.raw);
                scratch.emit_instruction(addr, out);
                addr += len as u64;
            }
            _ => {
                // Decode failure (or a decode that would cross the next label):
                // one raw byte, coalesced into up-to-8-byte `db` lines.
                if prog.read_bytes_into(addr, 1, &mut scratch.raw) {
                    scratch.push_db(addr, scratch.raw[0], out);
                } else {
                    scratch.flush_db(out);
                    scratch.emit_unreadable(addr, out);
                }
                addr += 1;
            }
        }
    }
    scratch.flush_db(out);
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
}

/// `; --- data ---`: address-sorted deduped labels (named globals ∪ `dat_`
/// tokens), each with a raw-byte dump (16/line + printable-ASCII column) or an
/// `?? (uninitialized/unmapped)` line for image-backed-less ranges (.bss).
fn emit_data_tail(
    prog: &ConsoleProgram,
    sections: &[(u64, u64, u32)],
    dat_addrs: &BTreeSet<u64>,
    out: &mut String,
) {
    let mut data: BTreeMap<u64, DataLabel> = BTreeMap::new();
    for (name, vma, type_size) in prog.global_data_symbols() {
        // First named symbol at a VMA wins (global_data_symbols is
        // (vma, name)-sorted; duplicates at one address are aliases).
        data.entry(vma).or_insert(DataLabel { name, type_size: Some(type_size), dat_alias: false });
    }
    for &vma in dat_addrs {
        data.entry(vma)
            .and_modify(|l| l.dat_alias = true)
            .or_insert_with(|| DataLabel {
                name: format!("dat_{vma:x}"),
                type_size: None,
                dat_alias: false,
            });
    }
    if data.is_empty() {
        return;
    }

    out.push_str("\n; --- data ---\n");
    let addrs: Vec<u64> = data.keys().copied().collect();
    for (idx, (&vma, label)) in data.iter().enumerate() {
        // Size: a typed symbol's datatype size; a bare `dat_` gets
        // min(gap to the next label / containing-section end, 32), floor 1.
        let size = match label.type_size {
            Some(s) if s > 0 => s as u64,
            _ => {
                let next_label = addrs.get(idx + 1).copied();
                let sec_end = sections
                    .iter()
                    .find(|&&(sv, ss, _)| vma >= sv && vma < sv.saturating_add(ss))
                    .map(|&(sv, ss, _)| sv + ss);
                let bound = [next_label, sec_end]
                    .into_iter()
                    .flatten()
                    .filter(|&b| b > vma)
                    .map(|b| b - vma)
                    .min()
                    .unwrap_or(DAT_SIZE_CAP);
                bound.clamp(1, DAT_SIZE_CAP)
            }
        };
        out.push('\n');
        if label.dat_alias {
            out.push_str(&format!("{}:  ; 0x{vma:x} = dat_{vma:x}\n", label.name));
        } else {
            out.push_str(&format!("{}:  ; 0x{vma:x}\n", label.name));
        }
        match prog.read_bytes(vma, size as usize) {
            Some(bytes) => {
                for (row, chunk) in bytes.chunks(16).enumerate() {
                    let hex =
                        chunk.iter().map(|b| format!("{b:02x}")).collect::<Vec<_>>().join(" ");
                    let ascii: String = chunk
                        .iter()
                        .map(|&b| if (0x20..0x7f).contains(&b) { b as char } else { '.' })
                        .collect();
                    out.push_str(&format!(
                        "  {:08x}: {hex:<47}  |{ascii}|\n",
                        vma + row as u64 * 16
                    ));
                }
            }
            None => {
                out.push_str(&format!(
                    "  {vma:08x}: ?? (uninitialized/unmapped, {size} bytes)\n"
                ));
            }
        }
    }
}

// --- README.md ---------------------------------------------------------------

/// `README.md`: binary metadata (file size, arch id, entry point + sections
/// re-parsed via the `object` crate — the engine's LoadImage sections carry no
/// names), function counts, and the artifact inventory / labeling conventions.
///
/// `path_label` is the path string printed in the `| Path |` row (the CLI
/// passes `binary_path.display()`; the wasm front-end a virtual name);
/// `binary_path` itself is still read for the file size and the `object`
/// re-parse.
pub fn build_readme(
    binary_path: &Path,
    path_label: &str,
    file_name: &str,
    prog: &ConsoleProgram,
    results: &[FuncResult],
) -> String {
    let file_size =
        std::fs::metadata(binary_path).map(|m| m.len().to_string()).unwrap_or_else(|_| "?".into());
    let ok = results.iter().filter(|r| r.error.is_none()).count();
    let failed = results.len() - ok;

    let mut out = String::new();
    out.push_str(&format!("# {file_name} — kuna project export\n\n"));
    out.push_str("Generated by `kuna decompile-project`.\n\n");
    out.push_str("## Binary\n\n");
    out.push_str("| Field | Value |\n|---|---|\n");
    out.push_str(&format!("| Path | `{path_label}` |\n"));
    out.push_str(&format!("| File size | {file_size} bytes |\n"));
    out.push_str(&format!("| Architecture | `{}` |\n", prog.description()));

    // Entry point + named sections come from an `object` re-parse (the engine's
    // section iterator has no name field).
    let raw = std::fs::read(binary_path).unwrap_or_default();
    let parsed = object::File::parse(&*raw).ok();
    match &parsed {
        // `image_entry_vma`, not `entry()`: a Mach-O `LC_MAIN` states a
        // `__TEXT`-relative file offset where every other format states a VMA.
        Some(f) => out.push_str(&format!(
            "| Entry point | `0x{:x}` |\n",
            kuna_analysis::analyzers::entry::image_entry_vma(f, &raw).unwrap_or(0)
        )),
        None => out.push_str("| Entry point | unavailable |\n"),
    }
    out.push_str(&format!("| Functions | {} total, {ok} decompiled, {failed} failed |\n", results.len()));

    if let Some(f) = &parsed {
        out.push_str("\n## Sections\n\n");
        out.push_str("| Name | Address | Size | Kind |\n|---|---|---|---|\n");
        for s in f.sections() {
            out.push_str(&format!(
                "| `{}` | `0x{:x}` | `0x{:x}` | {:?} |\n",
                s.name().unwrap_or("?"),
                s.address(),
                s.size(),
                s.kind()
            ));
        }
    }

    out.push_str(&format!(
        "\n## Files\n\n\
         - `{file_name}.c` — every decompiled function, address-ordered, each under a\n\
         \x20 `// Function: <name> @ <addr>` header (failures appear as\n\
         \x20 `// Function: <name> @ <addr>  (error: ...)` comments). Includes `{file_name}.h`.\n\
         - `{file_name}.h` — recompilation aid: the generated core-typedef prelude (the\n\
         \x20 Ghidra/kuna `undefined` family included), the user-defined type definitions\n\
         \x20 recovered during decompilation, and one prototype per decompiled function\n\
         \x20 (token-identical to its `.c` definition line).\n\
         - `{file_name}.asm` — labeled linear disassembly of every code section. Function\n\
         \x20 labels match the `.c` names exactly (`main:`, `sub_<addr>:`); under each label\n\
         \x20 a comment block maps the decompiled variables to their storage\n\
         \x20 (`; arg: <name> (<type>)`, `; stack: <name> @ [stack-0x18] (<type>)`).\n\
         \x20 Undecodable bytes appear as `db` lines. The `; --- data ---` tail labels the\n\
         \x20 named globals and every `dat_<hex>` address the `.c` references, with raw\n\
         \x20 bytes (`??` for unmapped/.bss ranges).\n\
         - `README.md` — this file.\n\
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
