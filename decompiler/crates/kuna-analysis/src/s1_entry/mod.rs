//! Function-entry / function-start discovery for stripped binaries — the kuna
//! analog of Ghidra's entry-point + function-start analyzers, fused with the
//! `.eh_frame` FDE oracle into ONE additive discovery pass.
//!
//! **Multi-format (PR-12+13):** the ELF oracles below are the original core;
//! [`run`](EntryDiscoveryPass::run) / [`collect_entries`] now **dispatch on the
//! object format** (the `ObjectFormat` seam), running the PE analogs
//! (`.pdata`/TLS/entry — [`pe_entry`]) for a PE and the Mach-O analogs
//! (`LC_FUNCTION_STARTS`/`LC_MAIN`/`__mod_init_func` — [`macho_entry`]) for a
//! Mach-O, so a stripped PE/Mach-O recovers its functions too. The ELF oracles
//! stay unchanged and the PE/Mach-O oracles are no-ops on ELF; the arch-specific
//! oracles (4: libc-start, 5: prologue patterns) are reused where applicable.
//!
//! Ghidra recovers function entries with several cooperating analyzers; this
//! pass ports the **feasible subset** of each (the analyzer tier has only the
//! parsed object — no disassembled Listing / PseudoDisassembler — so the deeply
//! Listing-coupled parts are a documented LOSS, mirroring the same wall
//! `s1_loader/noreturn.rs` documents for the "Discovered" no-return analyzer):
//!
//! - `EntryPointAnalyzer.java` ("Disassemble Entry Points") — disassembles every
//!   *external entry point* the ELF loader seeded (the ELF `e_entry`, `DT_INIT`/
//!   `DT_FINI`, and the `INIT_ARRAY`/`FINI_ARRAY` pointer tables). We extract
//!   those entry addresses directly from the ELF (oracles 1+2 below) — the byte
//!   that Ghidra disassembles into a function.
//! - `ExternalEntryFunctionAnalyzer.java` ("External Entry References") — turns
//!   each external-entry-point-with-code into a function. The commit seam's
//!   `out.entries` path (`engine.rs::commit_analysis_output` step 2: `name_function`
//!   + `add_function` + `register_symbol`) is exactly this step, so the pass need
//!   only emit the VMAs.
//! - `FunctionStartAnalyzer.java` ("Function Start Search") — the prologue
//!   byte-pattern matcher (`DittedBitSequence`). We port the bit matcher
//!   (`DittedBitSequence.initFromDittedStringData`/`isMatch`,
//!   DittedBitSequence.java:365,218) and a *minimal* vendored set of the bare
//!   `<funcstart/>` x86-64 gcc prologue sequences (oracle 5). The `after="defined"`
//!   / `validcode="N"` post-rules need a PseudoDisassembler we do not have —
//!   dropped as a documented LOSS.
//! - `GccExceptionAnalyzer.java` + `ehFrame/{Cie,FrameDescriptionEntry}.java`
//!   (the `.eh_frame` FDE `pcBegin` decode, scoped to FDE-start extraction —
//!   NOT full CFI/LSDA) — oracle 3, the highest-value oracle for C/C++ binaries:
//!   every FDE's initial-location is a function start.
//!
//! The pure core is [`collect_entries`]: it unions five oracles, dedups, and
//! skips any VMA already covered by a real funcsym (`.symtab`/`.dynsym` defined
//! FUNC + PLT stubs) so the pass only ever *adds* unnamed function starts.
//! Every emitted VMA is validated to fall inside an executable section.
//!
//! ## Oracles (unioned, deduped, funcsym-skipped)
//!
//! 1. **ELF entry point** (`e_entry`) — `EntryPointAnalyzer` external entry.
//! 2. **`DT_INIT`/`DT_FINI` + `DT_INIT_ARRAY`/`DT_FINI_ARRAY`** pointer tables —
//!    the loader-seeded external entry points (`ElfProgramBuilder`). These carry
//!    Ghidra-faithful **names** through the `entry_names` overlay (`_INIT_<i>` /
//!    `_FINI_<i>` per array element, `_DT_INIT`/`_DT_FINI` for the single tags) so
//!    the commit seam names them like `ElfProgramBuilder.createDynamicEntryPoints`
//!    instead of the generic `sub_<addr>`; the naming is additive and never changes
//!    which VMAs are discovered.
//! 3. **`.eh_frame` FDE `pcBegin`** addresses — [`scan_eh_frame_starts`].
//! 4. **`_start`→`main` libc-start idiom** (x86-64 / AArch64 / ARM / RISC-V): the
//!    arg-setup instructions that load `main` into the platform's first integer-arg
//!    register right before the `__libc_start_main` call. x86-64 carries `main` as
//!    a PC-relative immediate (`lea rdi,[rip+disp]`); the PIE crt1 of AArch64/ARM/
//!    RISC-V loads it *indirectly* from a GOT slot bearing an `R_*_RELATIVE`
//!    relocation whose target is `main`. The disassembly-free stand-in for the
//!    general call-target sweep, which is infeasible at the analyzer tier (no
//!    Listing) — we recover the single highest-value call target.
//! 5. **Prologue byte patterns** (x86-64 gcc): the `FunctionStartAnalyzer` port,
//!    a conservative subset.
//!
//! ## Scope / LOSS
//!
//! - General undirected call-target sweep is infeasible at the analyzer tier (no
//!   Listing) — substituted by the `_start`→`main` idiom (oracle 4) + prologue
//!   patterns (oracle 5).
//! - The `after="defined"` / `validcode` pattern post-rules are dropped (no
//!   PseudoDisassembler); only bare `<funcstart/>` patterns are ported.
//! - Oracle 4 (`_start`→`main`) covers x86-64 + AArch64 + ARM/Thumb + RISC-V
//!   (Increment 23). MIPS/PPC `_start` idioms are a follow-up (those arches no-op).
//!   Oracle 5 (prologue patterns) remains **x86-64-only** in v1 (the
//!   patternconstraints.xml for ARM/AARCH64/MIPS/PPC are a follow-up).
//!   Oracles 1–3 are arch-independent.
//! - Static-image base-0 PIE assumption for array-pointer / absptr decode: kuna's
//!   `ObjectLoadImage` loads at the file's native vmas and never rebases, so the
//!   AbstractDwarfEHDecoder image-base adjustment is identically 0.

use object::read::{Object, ObjectSection, ObjectSymbol};
use object::{SectionKind, SymbolKind};

use crate::pass::{AnalysisCtx, AnalysisOutput, AnalysisPass, ContextPaint, Stage};
use crate::s1_loader::format::FormatKind;

mod macho_entry;
mod pe_entry;

// ===========================================================================
// The pass
// ===========================================================================

/// Port of the entry-point + function-start + `.eh_frame`-FDE analyzers, fused:
/// emit discovered function-entry VMAs into [`AnalysisOutput::entries`].
pub struct EntryDiscoveryPass;

impl AnalysisPass for EntryDiscoveryPass {
    fn stage(&self) -> Stage {
        Stage::S1
    }

    fn id(&self) -> &'static str {
        "entry_disc"
    }

    fn run(&self, ctx: &AnalysisCtx) -> AnalysisOutput {
        let mut out = AnalysisOutput::default();
        // Format-dispatched (PR-12/13): ELF runs the eh_frame/dynamic oracles, PE
        // the `.pdata`/TLS/entry oracles, Mach-O the `LC_FUNCTION_STARTS`/`LC_MAIN`
        // oracles. An unsupported format yields an empty output. The oracle
        // *selection* lives in `collect_entries`; here we only gate the whole pass
        // off for formats with no entry oracle at all. Additive contract: never
        // fail — an empty output on any anomaly.
        if !matches!(
            crate::s1_loader::format::detect(ctx.file).map(|f| f.kind()),
            Ok(FormatKind::Elf | FormatKind::Pe | FormatKind::MachO)
        ) {
            return out;
        }
        out.entries = collect_entries(ctx.file, ctx.bytes);
        // ARM/Thumb: a discovered `main` whose libc-start GOT pointer had the
        // Thumb LSB set needs a `TMode=1` decode-mode paint at its (even) entry —
        // this stripped binary carries no `$t` mapping symbol for `arm_markers` to
        // paint from, so without it the engine decodes the Thumb body as A32 and
        // emits a degenerate function. The exact analog of `arm_markers`'
        // STT_FUNC-LSB → `TMode=1` paint, derived here from the GOT pointer LSB.
        out.context_paints = thumb_entry_paints(ctx.file);
        // Ghidra-faithful names for the dynamic INIT/FINI entries (oracle 2 only),
        // restricted to the VMAs that actually survived into `out.entries` (a named
        // entry already covered by a funcsym is filtered out above, so its name is
        // moot). The commit seam consults this overlay; entries absent from it keep
        // the generic `sub_<addr>` name. See `AnalysisOutput::entry_names`.
        out.entry_names = collect_entry_names(ctx.file, &out.entries);
        out
    }
}

/// (kuna) `.eh_frame` LSDA landing-pad discovery — the GccExceptionAnalyzer full
/// `.gcc_except_table` markup, factored out of the always-on entry-discovery pass
/// as its OWN gated pass (id `eh_frame_full`, default-OFF). It emits each
/// exception-handler landing pad (catch/cleanup block, reached only by the
/// unwinder) as a discovered function entry — net-new code targets the FDE-pcBegin
/// / prologue / libc-start oracles never see (a landing pad sits mid-function).
///
/// Like every other analysis pass it RUNS at load (the facts are cheap to compute
/// and are stashed per-pass); the **commit** is gated by `--option eh_frame_full
/// on` (`engine.rs::analysis_pass_enabled` → `arch.analysis_eh_frame_full`). So a
/// default run computes but never COMMITS the landing pads, and the discovery set
/// is byte-identical to the FDE-pcBegin-only behavior — every parity gate is
/// structurally untouched. Output-changing (adds entries) ⇒ default-off.
///
/// CFI (the `DW_CFA_*` call-frame instructions) is INHERITED, not rebuilt: kuna's
/// engine recovers the stack frame from the code (S5 type inference + S7 frame
/// analysis), so the CFA/saved-register rules add nothing at the decompiler tier.
pub struct EhFrameLsdaPass;

impl AnalysisPass for EhFrameLsdaPass {
    fn stage(&self) -> Stage {
        Stage::S1
    }

    fn id(&self) -> &'static str {
        "eh_frame_full"
    }

    fn run(&self, ctx: &AnalysisCtx) -> AnalysisOutput {
        let mut out = AnalysisOutput::default();
        // ELF-only (the `.eh_frame`/`.gcc_except_table` exception model). Additive
        // contract: an empty output on any anomaly, never a failure.
        if !matches!(
            crate::s1_loader::format::detect(ctx.file).map(|f| f.kind()),
            Ok(FormatKind::Elf)
        ) {
            return out;
        }
        // The filtered landing-pad entries (exec-section + funcsym-skip + dedup,
        // the SAME gates every other oracle applies). Empty on a non-exception
        // binary (no `.gcc_except_table`), so the pass is a strict no-op there.
        out.entries = collect_landing_pad_entries(ctx.file, ctx.bytes);
        out
    }
}

// ===========================================================================
// The pure core
// ===========================================================================

/// Discover function-entry VMAs from the parsed object: union the five oracles,
/// keep only addresses inside an executable section, drop any already covered by
/// a real funcsym, and dedup. The returned vec is sorted (stable output).
///
/// This is the testable seam (drive it over fixture bytes).
pub fn collect_entries(file: &object::File, bytes: &[u8]) -> Vec<u64> {
    let execs = executable_sections(file);
    let funcsyms = existing_function_addrs(file, bytes);

    let kind = crate::s1_loader::format::detect(file).map(|f| f.kind()).ok();
    let mut cand: Vec<u64> = Vec::new();

    match kind {
        // ELF: e_entry, the dynamic INIT/FINI tables, .eh_frame FDEs, and the
        // libc-start idiom (the original oracles, unchanged).
        Some(FormatKind::Elf) => {
            // Oracle 1: ELF entry point (e_entry). EntryPointAnalyzer external entry.
            let entry = file.entry();
            if entry != 0 {
                cand.push(entry);
            }
            // Oracle 2: DT_INIT/DT_FINI + INIT_ARRAY/FINI_ARRAY pointer tables.
            cand.extend(dynamic_entry_points(file));
            // Oracle 3: .eh_frame FDE pcBegin addresses.
            cand.extend(scan_eh_frame_starts(file));
            // Oracle 4: _start -> main via the libc-start idiom (arch-dispatched).
            if let Some(main) = libc_start_main_target(file, entry) {
                cand.push(main);
            }
        }
        // PE: entry (AddressOfEntryPoint+ImageBase), `.pdata` RUNTIME_FUNCTION
        // begins (the `.eh_frame` analog), TLS callbacks, and exports (PR-12).
        Some(FormatKind::Pe) => {
            cand.extend(pe_entry::pe_entry_candidates(file, bytes));
        }
        // Mach-O: entry (`LC_MAIN`/`LC_UNIXTHREAD`), `LC_FUNCTION_STARTS` (the
        // richest, stripped-surviving source), `__mod_init_func`, and exports
        // (PR-13).
        Some(FormatKind::MachO) => {
            cand.extend(macho_entry::macho_entry_candidates(file, bytes));
        }
        // COFF objects (pre-link) and unknown formats: no entry oracle.
        _ => {}
    }

    // Oracle 5: prologue byte patterns — x86-64-only, format-neutral (it scans
    // the executable-section bytes). The vendored gcc pattern subset; the
    // ARM/AARCH64/MIPS/PPC patternconstraints are a follow-up.
    if file.architecture() == object::Architecture::X86_64 {
        cand.extend(prologue_pattern_starts(&execs));
    }

    // Keep only plausible code addresses (inside an executable section), drop any
    // already named by a funcsym, dedup, sort.
    let mut out: Vec<u64> = Vec::new();
    for vma in cand {
        if vma == 0 {
            continue;
        }
        if !in_executable_section(&execs, vma) {
            continue;
        }
        if funcsyms.binary_search(&vma).is_ok() {
            continue;
        }
        out.push(vma);
    }
    out.sort_unstable();
    out.dedup();
    out
}

/// Ghidra-faithful names for the *dynamic* INIT/FINI entries (oracle 2), as an
/// `(addr, name)` overlay the commit seam consults. Faithful to
/// `ElfProgramBuilder.createDynamicEntryPoints`:
///
/// - single `DT_INIT` / `DT_FINI` → `_DT_INIT` / `_DT_FINI` (Ghidra's
///   `"_" + dynamicEntryType.name`),
/// - each `DT_INIT_ARRAY` element `i` → `_INIT_<i>`,
/// - each `DT_FINI_ARRAY` element `i` → `_FINI_<i>`
///
/// (and `DT_PREINIT_ARRAY` element `i` → `_PREINIT_<i>` — the constant is wired
/// for faithfulness, but kuna's discovery does not currently emit PREINIT_ARRAY
/// entries, so none are produced here; adding PREINIT_ARRAY *discovery* is a
/// separate follow-up that would change the discovery set).
///
/// Only oracle 2 names anything (the other four oracles leave their entries
/// generic `sub_<addr>`). The result is filtered to `kept` — the VMAs that
/// actually survived [`collect_entries`]'s funcsym-skip/dedup — so a name for an
/// entry that was dropped (e.g. it duplicates a real funcsym) is never emitted.
/// This is purely additive: it never changes WHICH entries are discovered.
pub fn collect_entry_names(file: &object::File, kept: &[u64]) -> Vec<(u64, String)> {
    let mut out: Vec<(u64, String)> = Vec::new();
    for (addr, name) in dynamic_entry_names(file) {
        if kept.contains(&addr) {
            out.push((addr, name));
        }
    }
    // A VMA can be named by at most one dynamic source (INIT vs FINI tables are
    // disjoint pointer arrays); dedup defensively to keep the overlay a clean map.
    out.sort_by(|a, b| a.0.cmp(&b.0));
    out.dedup_by(|a, b| a.0 == b.0);
    out
}

// ===========================================================================
// Section / funcsym helpers
// ===========================================================================

/// `(address, address+size, data)` for every executable section. SHF_EXECINSTR
/// or the high-level `SectionKind::Text` (`.text`/`.init`/`.fini`/`.plt`). Used
/// both as the prologue-sweep target and the "is this VMA plausible code?" oracle.
pub(crate) fn executable_sections(file: &object::File) -> Vec<(u64, u64, Vec<u8>)> {
    // ELF section header flag: SHF_EXECINSTR (the section holds machine code).
    const SHF_EXECINSTR: u64 = 0x4;
    // Mach-O section attribute: S_ATTR_PURE_INSTRUCTIONS / S_ATTR_SOME_INSTRUCTIONS.
    const S_ATTR_PURE_INSTRUCTIONS: u32 = 0x8000_0000;
    const S_ATTR_SOME_INSTRUCTIONS: u32 = 0x0000_0400;

    let mut out = Vec::new();
    for sec in file.sections() {
        // Per-format executable test (PR-12/13): ELF SHF_EXECINSTR, PE COFF
        // IMAGE_SCN_MEM_EXECUTE, Mach-O instruction attributes — each falling
        // back to the neutral `SectionKind::Text` (`.text`/`__text`/`.plt`/…).
        let exec = match sec.flags() {
            object::SectionFlags::Elf { sh_flags } => sh_flags & SHF_EXECINSTR != 0,
            object::SectionFlags::Coff { characteristics } => {
                characteristics & object::pe::IMAGE_SCN_MEM_EXECUTE != 0
            }
            object::SectionFlags::MachO { flags } => {
                flags & (S_ATTR_PURE_INSTRUCTIONS | S_ATTR_SOME_INSTRUCTIONS) != 0
                    || sec.kind() == SectionKind::Text
            }
            _ => sec.kind() == SectionKind::Text,
        };
        if !exec {
            continue;
        }
        let addr = sec.address();
        let size = sec.size();
        if size == 0 {
            continue;
        }
        let data = sec.data().map(|d| d.to_vec()).unwrap_or_default();
        out.push((addr, addr.saturating_add(size), data));
    }
    out
}

/// True if `vma` lands inside any executable section's `[address, address+size)`.
pub(crate) fn in_executable_section(execs: &[(u64, u64, Vec<u8>)], vma: u64) -> bool {
    execs.iter().any(|&(lo, hi, _)| vma >= lo && vma < hi)
}

/// Sorted VMAs of every already-named function: `.symtab`/`.dynsym` *defined*
/// FUNC symbols (UND imports have `st_value == 0`) plus PLT import stubs. The
/// commit seam's `find_function` already no-ops a covered address, but skipping
/// these here keeps the emitted set to genuinely *new* starts.
pub(crate) fn existing_function_addrs(file: &object::File, bytes: &[u8]) -> Vec<u64> {
    let mut out: Vec<u64> = Vec::new();
    for sym in file.symbols().chain(file.dynamic_symbols()) {
        if sym.kind() != SymbolKind::Text {
            continue;
        }
        let addr = sym.address();
        if addr != 0 {
            out.push(addr);
        }
    }
    // Import stubs, through the format seam (ELF: PLT/GOT). `bytes` is unused for
    // ELF; PE/Mach-O re-parse it inside their `resolve_imports`.
    for p in crate::s1_loader::format::resolve_imports(file, bytes) {
        out.push(p.addr);
    }
    out.sort_unstable();
    out.dedup();
    out
}

// ===========================================================================
// Oracle 2: dynamic INIT/FINI + INIT_ARRAY/FINI_ARRAY
// ===========================================================================

// DT_* tags (elf.h). Values are vmas.
const DT_NULL: u64 = 0;
const DT_INIT: u64 = 12;
const DT_FINI: u64 = 13;
const DT_INIT_ARRAY: u64 = 25;
const DT_FINI_ARRAY: u64 = 26;
const DT_INIT_ARRAYSZ: u64 = 27;
const DT_FINI_ARRAYSZ: u64 = 28;
// Ghidra also seeds `DT_PREINIT_ARRAY` (named `_PREINIT_<i>`), but kuna does not
// currently *discover* preinit-array elements as code entries. Adding that would
// change the discovery set (which VMAs are found), out of scope for this purely
// additive naming pass — wired here as the faithful follow-up seam (the
// `read_pointer_table` `base_name` would just be `"_PREINIT_"`).
#[allow(dead_code)]
const DT_PREINIT_ARRAY: u64 = 32;
#[allow(dead_code)]
const DT_PREINIT_ARRAYSZ: u64 = 33;

/// The loader-seeded external entry-point VMAs from the `.dynamic` table:
/// `DT_INIT`/`DT_FINI` (one each) plus every pointer in the `DT_INIT_ARRAY` /
/// `DT_FINI_ARRAY` tables (faithful to `ElfProgramBuilder` marking these as
/// external entry points). The address-only projection of
/// [`dynamic_entry_points_named`] (which also documents the table decode); the
/// Ghidra-faithful names ride the additive overlay (`dynamic_entry_names`).
fn dynamic_entry_points(file: &object::File) -> Vec<u64> {
    // The address-only projection of the named oracle. Discovery (WHICH VMAs) is
    // defined here; the names are an additive overlay (see `dynamic_entry_names`).
    dynamic_entry_points_named(file).into_iter().map(|(addr, _name)| addr).collect()
}

/// The dynamic INIT/FINI names overlay (oracle 2): the `(addr, name)` pairs for
/// every entry [`dynamic_entry_points_named`] could name. See
/// [`collect_entry_names`] for the Ghidra naming.
fn dynamic_entry_names(file: &object::File) -> Vec<(u64, String)> {
    dynamic_entry_points_named(file)
        .into_iter()
        .filter_map(|(addr, name)| name.map(|n| (addr, n)))
        .collect()
}

/// The loader-seeded external entry points from the `.dynamic` table, each paired
/// with its Ghidra-faithful name (or `None` for the unnamed single-`DT_INIT`/
/// `DT_FINI` shape — see below). The *addresses* are the single source of truth
/// for discovery (`dynamic_entry_points` is exactly `.map(|(a,_)| a)`); the names
/// feed the additive overlay (`dynamic_entry_names`).
///
/// Faithful to `ElfProgramBuilder.createDynamicEntryPoints`:
///   - `DT_INIT`/`DT_FINI` → one entry each, named `_DT_INIT`/`_DT_FINI`
///     (Ghidra's single-entry `"_" + dynamicEntryType.name`),
///   - `DT_INIT_ARRAY`/`DT_FINI_ARRAY` → every nonzero pointer in the table, the
///     `i`-th named `_INIT_<i>`/`_FINI_<i>` (Ghidra's `baseName + i`).
///
/// The dynamic table is read from the `.dynamic` section bytes as `Elf{32,64}_Dyn`
/// (tag/val) pairs; the array bytes come from whichever section contains the array
/// vma (the static-image base-0 assumption: a PIE array pointer is already the
/// file vma, no load bias).
fn dynamic_entry_points_named(file: &object::File) -> Vec<(u64, Option<String>)> {
    let mut out: Vec<(u64, Option<String>)> = Vec::new();

    let Some(dynsec) = file.section_by_name(".dynamic") else {
        return out;
    };
    let Ok(data) = dynsec.data() else {
        return out;
    };
    let is64 = file.is_64();
    let le = file.is_little_endian();
    let entsz = if is64 { 16usize } else { 8usize };

    // Collect tag→val first (we need the *SZ partners for the array tags).
    let mut init_array: Option<u64> = None;
    let mut init_array_sz: u64 = 0;
    let mut fini_array: Option<u64> = None;
    let mut fini_array_sz: u64 = 0;

    let mut off = 0usize;
    while off + entsz <= data.len() {
        let (tag, val) = if is64 {
            (read_u64(&data[off..], le), read_u64(&data[off + 8..], le))
        } else {
            (read_u32(&data[off..], le) as u64, read_u32(&data[off + 4..], le) as u64)
        };
        off += entsz;
        if tag == DT_NULL {
            break;
        }
        match tag {
            // Single DT_INIT/DT_FINI: Ghidra names these `_DT_INIT`/`_DT_FINI`
            // (createDynamicEntryPoints's `"_" + dynamicEntryType.name`). The VMA
            // is pushed unconditionally (byte-identical to the prior `out.push(val)`);
            // the name rides as the overlay.
            DT_INIT => out.push((val, Some("_DT_INIT".to_string()))),
            DT_FINI => out.push((val, Some("_DT_FINI".to_string()))),
            DT_INIT_ARRAY => init_array = Some(val),
            DT_INIT_ARRAYSZ => init_array_sz = val,
            DT_FINI_ARRAY => fini_array = Some(val),
            DT_FINI_ARRAYSZ => fini_array_sz = val,
            _ => {}
        }
    }

    let ptr = if is64 { 8usize } else { 4usize };
    // Order preserved from the prior implementation: INIT_ARRAY pointers then
    // FINI_ARRAY pointers. The element index `i` drives the `_INIT_<i>`/`_FINI_<i>`
    // name (Ghidra's `baseName + i`).
    for (base, sz, base_name) in
        [(init_array, init_array_sz, "_INIT_"), (fini_array, fini_array_sz, "_FINI_")]
    {
        let Some(base) = base else { continue };
        out.extend(read_pointer_table(file, base, sz, ptr, le, base_name));
    }

    out
}

/// Read `sz / ptr` pointers from the array at vma `base` by slicing the section
/// that contains `base`. Each nonzero decoded pointer is itself a function entry;
/// the `i`-th surviving element is named `<base_name><i>` (Ghidra's `baseName + i`,
/// where `i` is the element's index in the array — note Ghidra increments `i` over
/// ALL array slots, but a zero pointer is `continue`d, so the named survivors carry
/// their original array index, which we mirror).
fn read_pointer_table(
    file: &object::File,
    base: u64,
    sz: u64,
    ptr: usize,
    le: bool,
    base_name: &str,
) -> Vec<(u64, Option<String>)> {
    let mut out = Vec::new();
    let Some((sec_addr, data)) = section_bytes_containing(file, base) else {
        return out;
    };
    let start = (base - sec_addr) as usize;
    let n = (sz as usize) / ptr;
    for i in 0..n {
        let o = start + i * ptr;
        if o + ptr > data.len() {
            break;
        }
        let p = if ptr == 8 { read_u64(&data[o..], le) } else { read_u32(&data[o..], le) as u64 };
        if p != 0 {
            // Ghidra names the element by its array index `i` (`baseName + i`).
            out.push((p, Some(format!("{base_name}{i}"))));
        }
    }
    out
}

/// `(section_vma, section_data)` for the section whose `[address, address+size)`
/// contains `vma`. Used to resolve an array vma to its bytes.
fn section_bytes_containing(file: &object::File, vma: u64) -> Option<(u64, Vec<u8>)> {
    for sec in file.sections() {
        let addr = sec.address();
        let size = sec.size();
        if size == 0 {
            continue;
        }
        if vma >= addr && vma < addr.saturating_add(size) {
            if let Ok(d) = sec.data() {
                return Some((addr, d.to_vec()));
            }
        }
    }
    None
}

// ===========================================================================
// Oracle 4: _start -> main via the libc-start idiom (per-architecture)
// ===========================================================================

/// Recover `main` from the `_start`→`__libc_start_main(main, …)` idiom — the
/// disassembly-free stand-in for the general call-target sweep (a pre-decompile
/// Listing is unavailable at the analyzer tier — the same wall `noreturn.rs`
/// documents): kuna recovers the *one* highest-value call target by byte-decoding
/// the instructions that load `main` into the platform's first integer-arg
/// register right before the `__libc_start_main` call.
///
/// The decode is **architecture-dispatched** (each path gated on
/// `file.architecture()`, additive — an unrecognized arch returns `None`):
///
/// - **x86-64** (`main` in `rdi`): `lea rdi,[rip+disp]` (`48 8d 3d <disp32>`)
///   carries `main` as a PC-relative *immediate* — [`x86_64_main_target`].
/// - **AArch64** (`main` in `x0`) / **RISC-V** (`main` in `a0`): PIE crt1 loads
///   `main` *indirectly* from a GOT slot (`adrp x0; ldr x0,[x0,#off]` /
///   `auipc a0; ld a0,off(a0)`). The slot carries an `R_*_RELATIVE` relocation
///   whose target is `main`'s VMA — [`aarch64_main_target`] /
///   [`riscv_main_target`] decode the slot, then resolve it through the
///   RELATIVE-target map ([`relative_targets`]).
/// - **ARM/Thumb** (`main` in `r0`): GOT-relative load
///   (`ldr r0,[GOT_base, #off]`); the GOT slot is `.got + off`, again carrying an
///   `R_ARM_RELATIVE` whose in-place target is `main` (with the Thumb LSB set,
///   masked off) — [`arm_main_target`].
///
/// All non-x86 paths cross-check the decoded slot against the RELATIVE-target map
/// and validate the resolved VMA is inside an executable section *before*
/// emitting, so a misdecode yields `None` rather than a bogus entry.
fn libc_start_main_target(file: &object::File, entry: u64) -> Option<u64> {
    if entry == 0 {
        return None;
    }
    use object::Architecture as A;
    match file.architecture() {
        A::X86_64 | A::X86_64_X32 => x86_64_main_target(file, entry),
        A::Aarch64 | A::Aarch64_Ilp32 => aarch64_main_target(file, entry),
        A::Arm => arm_main_target(file, entry),
        A::Riscv64 | A::Riscv32 => riscv_main_target(file, entry),
        _ => None,
    }
}

/// x86-64 SysV `_start` idiom: `main` is loaded into `rdi` (`lea rdi,[rip+disp]`,
/// bytes `48 8d 3d <disp32>`) immediately before the `call *__libc_start_main@GOT`.
/// Scan a small window at `e_entry` for that `lea rdi` and compute
/// `main = (lea_addr + 7) + sign_extend(disp32)`.
fn x86_64_main_target(file: &object::File, entry: u64) -> Option<u64> {
    let (sec_addr, data) = section_bytes_containing(file, entry)?;
    let start = (entry - sec_addr) as usize;
    // Scan a 64-byte window from _start for `48 8d 3d <disp32>` (lea rdi,[rip+d]).
    let window = data.get(start..(start + 64).min(data.len()))?;
    let mut i = 0usize;
    while i + 7 <= window.len() {
        if window[i] == 0x48 && window[i + 1] == 0x8d && window[i + 2] == 0x3d {
            let disp = read_i32(&window[i + 3..]);
            let lea_addr = entry + i as u64;
            // rip points past the 7-byte instruction.
            let main = (lea_addr.wrapping_add(7)).wrapping_add(disp as i64 as u64);
            return Some(main);
        }
        i += 1;
    }
    None
}

/// AArch64 PIE `_start` idiom: `main` is loaded into `x0` from a GOT slot via the
/// `adrp x0, page ; ldr x0,[x0,#lo12]` pair before the `bl __libc_start_main@plt`.
/// Decode the slot VMA `= (adrp_addr & !0xFFF) + page_off + lo12` (the same A64
/// `adrp`/`ldr` decode `elf_plt::decode_aarch64` uses, here keyed to `x0`), then
/// resolve it through the RELATIVE-target map → `main`.
fn aarch64_main_target(file: &object::File, entry: u64) -> Option<u64> {
    let (sec_addr, data) = section_bytes_containing(file, entry)?;
    let start = (entry - sec_addr) as usize;
    let win_end = (start + 0x80).min(data.len());
    let window = data.get(start..win_end)?;
    let rel = relative_targets(file);

    // A64 is fixed 32-bit LE; scan the `adrp x0` + `ldr x0,[x0,#imm]` pair.
    let mut off = 0usize;
    while off + 8 <= window.len() {
        let adrp = u32::from_le_bytes([
            window[off],
            window[off + 1],
            window[off + 2],
            window[off + 3],
        ]);
        let ldr = u32::from_le_bytes([
            window[off + 4],
            window[off + 5],
            window[off + 6],
            window[off + 7],
        ]);
        // adrp Xd, imm: bit31=1, bits[28:24]=10000; immlo=bits[30:29],
        // immhi=bits[23:5], Rd=bits[4:0].
        let is_adrp = (adrp >> 31) & 1 == 1 && (adrp >> 24) & 0x1f == 0b1_0000;
        let adrp_rd = adrp & 0x1f;
        // ldr Xt,[Xn,#imm] 64-bit unsigned offset: size=11(bits[31:30]),
        // bits[29:24]=111001, opc=01(bits[23:22]), imm12=bits[21:10], Rn=bits[9:5].
        let is_ldr = (ldr >> 30) & 0x3 == 0b11
            && (ldr >> 24) & 0x3f == 0b11_1001
            && (ldr >> 22) & 0x3 == 0b01;
        let ldr_rn = (ldr >> 5) & 0x1f;
        let ldr_rt = ldr & 0x1f;
        // The pair must target x0 (the SysV first integer arg = `main`).
        if is_adrp && adrp_rd == 0 && is_ldr && ldr_rn == 0 && ldr_rt == 0 {
            let immlo = ((adrp >> 29) & 0x3) as i64;
            let immhi = ((adrp >> 5) & 0x7_ffff) as i64; // 19 bits
            let mut imm21 = (immhi << 2) | immlo;
            if imm21 & (1 << 20) != 0 {
                imm21 -= 1 << 21; // sign-extend the 21-bit page offset
            }
            let page_off = imm21 << 12;
            let ldr_off = (((ldr >> 10) & 0xfff) as u64) * 8; // 64-bit scale
            let adrp_addr = sec_addr + (start + off) as u64;
            let slot = (adrp_addr & !0xFFF).wrapping_add(page_off as u64).wrapping_add(ldr_off);
            if let Some(&target) = rel.get(&slot) {
                return Some(target & !1); // mask any thumb-style LSB (defensive)
            }
        }
        off += 4;
    }
    None
}

/// RISC-V PIE `_start` idiom: `main` is loaded into `a0` from a GOT slot via the
/// `auipc a0, hi20 ; ld a0, lo12(a0)` pair before the `jal __libc_start_main@plt`.
/// Decode the slot VMA `= auipc_addr + (hi20<<12) + sign_extend(lo12)` (the same
/// `auipc`/`ld` decode `elf_plt::decode_riscv` uses, here keyed to `a0`=x10), then
/// resolve it through the RELATIVE-target map → `main`.
fn riscv_main_target(file: &object::File, entry: u64) -> Option<u64> {
    let (sec_addr, data) = section_bytes_containing(file, entry)?;
    let start = (entry - sec_addr) as usize;
    let win_end = (start + 0x80).min(data.len());
    let window = data.get(start..win_end)?;
    let rel = relative_targets(file);

    // RISC-V mixes 2- and 4-byte (compressed) insns; the auipc/ld pair is a
    // 32-bit `auipc` immediately followed by a 32-bit `ld`. Scan at 2-byte steps
    // (the minimal insn granularity) so the pair is found regardless of any
    // preceding compressed insns.
    let mut off = 0usize;
    while off + 8 <= window.len() {
        let auipc = u32::from_le_bytes([
            window[off],
            window[off + 1],
            window[off + 2],
            window[off + 3],
        ]);
        // auipc a0, imm: opcode 0x17 (bits[6:0]), rd 10/a0 (bits[11:7]).
        if (auipc & 0x7f) == 0x17 && ((auipc >> 7) & 0x1f) == 10 {
            let load = u32::from_le_bytes([
                window[off + 4],
                window[off + 5],
                window[off + 6],
                window[off + 7],
            ]);
            // ld a0, lo12(a0): opcode 0x03, funct3 3 (ld) [or 2 (lw, RV32)], rd 10,
            // rs1 10.
            let funct3 = (load >> 12) & 0x7;
            let load_ok = (load & 0x7f) == 0x03
                && (funct3 == 2 || funct3 == 3)
                && ((load >> 7) & 0x1f) == 10
                && ((load >> 15) & 0x1f) == 10;
            if load_ok {
                let hi20 = (auipc >> 12) & 0xf_ffff;
                let imm12 = (load >> 20) & 0xfff;
                let lo12 = ((imm12 as i32) << 20) >> 20; // sign-extend 12-bit
                let auipc_addr = sec_addr + (start + off) as u64;
                let slot = auipc_addr
                    .wrapping_add((hi20 as u64) << 12)
                    .wrapping_add(lo12 as i64 as u64);
                if let Some(&target) = rel.get(&slot) {
                    return Some(target & !1);
                }
            }
        }
        off += 2;
    }
    None
}

/// ARM/Thumb PIE `_start` idiom: `main` is loaded into `r0` GOT-*relatively*
/// (`ldr.w r0,[GOT_base, r0]`, the GOT base computed from the literal pool, the
/// index a small per-symbol GOT offset). Rather than fully simulate the
/// two-load+add GOT-base computation (toolchain-fragile), we use the invariant
/// that the GOT base **is** the `.got` section address: for each PC-relative
/// literal-pool word the `_start` window references that is a *small* offset, the
/// candidate slot `= .got_addr + off`; if that slot carries a RELATIVE relocation
/// whose target is in an executable section, that target is `main` (the Thumb LSB
/// masked off). The RELATIVE-map + exec-section cross-check makes the heuristic
/// self-validating (a wrong offset simply misses the map).
fn arm_main_target(file: &object::File, entry: u64) -> Option<u64> {
    // The public oracle masks the Thumb LSB; the raw helper keeps it so the
    // decode-mode paint can tell Thumb (`main|1`) from A32 (`main`).
    arm_main_target_raw(file, entry).map(|raw| raw & !1)
}

/// As [`arm_main_target`] but returns the GOT pointer **un-masked** — the ARM
/// libc-start `main` pointer carries the Thumb mode bit in bit 0 (`main|1` for a
/// Thumb function). The caller masks it for the entry VMA; [`thumb_entry_paints`]
/// inspects it to decide whether to paint `TMode=1`.
fn arm_main_target_raw(file: &object::File, entry: u64) -> Option<u64> {
    // _start's entry LSB is the Thumb mode bit; the bytes live at the even VMA.
    let start_vma = entry & !1;
    let (sec_addr, data) = section_bytes_containing(file, start_vma)?;
    let start = (start_vma - sec_addr) as usize;
    let win_end = (start + 0x80).min(data.len());
    let window = data.get(start..win_end)?;

    let got_addr = file.section_by_name(".got").map(|s| s.address())?;
    let rel = relative_targets(file);
    let execs = executable_sections(file);

    // Collect every aligned 32-bit word in the _start window that, read as a
    // little-endian u32, is a plausible *small* GOT offset (< the .got size, or a
    // modest cap). Each is a candidate `off` for slot = got_addr + off.
    let got_size = file.section_by_name(".got").map(|s| s.size()).unwrap_or(0);
    let cap = got_size.max(0x1000);
    let mut best: Option<u64> = None;
    let mut found = 0usize;
    let mut w = 0usize;
    while w + 4 <= window.len() {
        if (start + w) % 4 == 0 {
            let off = read_u32(&window[w..], true) as u64;
            if off != 0 && off < cap {
                let slot = got_addr.wrapping_add(off);
                if let Some(&target) = rel.get(&slot) {
                    // Validate the *masked* (even) VMA is in an exec section, but
                    // keep the RAW target so the LSB (Thumb mode) survives.
                    if in_executable_section(&execs, target & !1) {
                        // Unique winner: only emit if the GOT-offset literal
                        // resolves to exactly one exec-section RELATIVE target, so
                        // an ambiguous decode is a clean miss (None).
                        if best.is_none() {
                            best = Some(target);
                        }
                        found += 1;
                    }
                }
            }
        }
        w += 2; // Thumb literal pools are halfword-stepped; align-check gates it.
    }
    if found == 1 {
        best
    } else {
        None
    }
}

/// Decode-mode (`TMode`) paints for ARM-discovered Thumb entries — the analog of
/// `arm_markers`' STT_FUNC-LSB → `TMode=1` paint, but derived from the libc-start
/// `main` GOT pointer's Thumb LSB (a stripped binary has no `$t`/FUNC-LSB symbol
/// for `arm_markers`). Emits `TMode=1` at the discovered `main`'s even VMA when
/// its pointer had bit 0 set (a Thumb function). Empty on every non-ARM arch and
/// when `main` is A32 (the default `TMode=0` already decodes it).
fn thumb_entry_paints(file: &object::File) -> Vec<ContextPaint> {
    let mut out = Vec::new();
    if file.architecture() != object::Architecture::Arm {
        return out;
    }
    let entry = file.entry();
    if entry == 0 {
        return out;
    }
    if let Some(raw) = arm_main_target_raw(file, entry) {
        if raw & 1 == 1 {
            // Thumb `main`: paint TMode=1 at the even (decode) VMA. `end: None` is
            // the point-set shape `arm_markers`/`ArmSymbolAnalyzer` use.
            out.push(ContextPaint { addr: raw & !1, end: None, var: "TMode", value: 1 });
        }
    }
    out
}

/// Build `got_slot_vma → relative_target_vma` from the dynamic `R_*_RELATIVE`
/// relocations. The "RELATIVE" relocs are the ones `object` surfaces as
/// [`object::RelocationTarget::Absolute`] (no symbol): their *target* is a fixed
/// in-image VMA (an init/fini pointer, a vtable slot, or — for our purposes — the
/// `main` pointer the PIE `_start` loads). The target value is the RELA `addend`
/// when present (AArch64/RISC-V), else the in-place value stored at the slot (ARM
/// REL, `has_implicit_addend`), read from the containing section's bytes.
fn relative_targets(file: &object::File) -> std::collections::HashMap<u64, u64> {
    use object::read::Object;
    use object::RelocationTarget;
    let mut map: std::collections::HashMap<u64, u64> = std::collections::HashMap::new();
    let le = file.is_little_endian();
    let is64 = file.is_64();
    let Some(relocs) = file.dynamic_relocations() else {
        return map;
    };
    for (slot, reloc) in relocs {
        // RELATIVE = a non-symbolic (Absolute) dynamic reloc.
        if !matches!(reloc.target(), RelocationTarget::Absolute) {
            continue;
        }
        let target: u64 = if reloc.has_implicit_addend() {
            // REL (e.g. ARM): the addend is the value already stored at the slot.
            let Some((sec_addr, bytes)) = section_bytes_containing(file, slot) else {
                continue;
            };
            let o = (slot - sec_addr) as usize;
            if is64 {
                if o + 8 > bytes.len() {
                    continue;
                }
                read_u64(&bytes[o..], le)
            } else {
                if o + 4 > bytes.len() {
                    continue;
                }
                read_u32(&bytes[o..], le) as u64
            }
        } else {
            // RELA: the addend carries the target VMA directly.
            let a = reloc.addend();
            if a < 0 {
                continue;
            }
            a as u64
        };
        if target != 0 {
            map.insert(slot, target);
        }
    }
    map
}

// ===========================================================================
// Oracle 5: prologue byte patterns (FunctionStartAnalyzer port, x86-64 gcc)
// ===========================================================================

/// A ditted bit sequence — the matcher core of Ghidra's `DittedBitSequence`
/// (`DittedBitSequence.java`): `bits[i]` is the required value, `dits[i]` the
/// care-mask, so `isMatch(pos,val) == (val & dits[pos]) == bits[pos]`
/// (DittedBitSequence.java:218). A `.` bit is don't-care (`dits` bit 0).
struct DittedSeq {
    /// Required byte values (already masked by `dits`).
    bits: Vec<u8>,
    /// Care mask per byte (`1` = the bit must match).
    dits: Vec<u8>,
}

impl DittedSeq {
    /// Parse a ditted binary string like `"11111111 ........ 01010101"` (space-
    /// separated bytes; `.` = don't-care bit). Faithful to
    /// `DittedBitSequence.initFromDittedStringData` (DittedBitSequence.java:365):
    /// one byte per 8-bit group.
    fn from_binary(s: &str) -> DittedSeq {
        let mut bits = Vec::new();
        let mut dits = Vec::new();
        for tok in s.split_whitespace() {
            let mut b = 0u8;
            let mut d = 0u8;
            for (k, c) in tok.chars().enumerate() {
                let shift = 7 - k;
                match c {
                    '0' => d |= 1 << shift,
                    '1' => {
                        d |= 1 << shift;
                        b |= 1 << shift;
                    }
                    '.' => {}
                    _ => {}
                }
            }
            bits.push(b);
            dits.push(d);
        }
        DittedSeq { bits, dits }
    }

    /// Length in bytes.
    #[cfg(test)]
    fn len(&self) -> usize {
        self.bits.len()
    }

    /// `(val & dits[i]) == bits[i]` for every byte (DittedBitSequence.isMatch).
    fn matches(&self, window: &[u8]) -> bool {
        if window.len() < self.bits.len() {
            return false;
        }
        for i in 0..self.bits.len() {
            if window[i] & self.dits[i] != self.bits[i] {
                return false;
            }
        }
        true
    }
}

/// The vendored x86-64 gcc *bare-`<funcstart/>`* prologue sequences (the subset
/// of `x86-64gcc_patterns.xml` whose post-rule is an unconditional `<funcstart/>`,
/// no `after`/`validcode` — those need a PseudoDisassembler we lack). Kept
/// minimal in v1: the common gcc frame-setup shapes, plain and ENDBR64-prefixed.
fn prologue_patterns() -> Vec<DittedSeq> {
    vec![
        // endbr64 ; push rbp ; mov rbp,rsp   (f3 0f 1e fa 55 48 89 e5)
        DittedSeq::from_binary(
            "11110011 00001111 00011110 11111010 01010101 01001000 10001001 11100101",
        ),
        // push rbp ; mov rbp,rsp             (55 48 89 e5)
        DittedSeq::from_binary("01010101 01001000 10001001 11100101"),
        // endbr64 ; sub rsp, imm8/32         (f3 0f 1e fa 48 83 ec ..)
        DittedSeq::from_binary(
            "11110011 00001111 00011110 11111010 01001000 10000011 11101100",
        ),
    ]
}

/// Scan every executable section's bytes for a prologue pattern hit at an aligned
/// offset, emitting each hit VMA. Faithful to `FunctionStartAnalyzer.applyActionToSet`
/// minus the disassembly post-rules. Conservative: 16-byte function alignment (the
/// x86-64 gcc default; the commit's `find_function` overlap check squashes any hit
/// landing inside an already-named function, so the residual risk is a spurious
/// *new* start in a gap — the small v1 pattern list keeps that minimal).
fn prologue_pattern_starts(execs: &[(u64, u64, Vec<u8>)]) -> Vec<u64> {
    const FUNC_ALIGN: u64 = 16;
    let pats = prologue_patterns();
    let mut out = Vec::new();
    for (addr, _hi, data) in execs {
        let mut off = 0usize;
        while off < data.len() {
            let vma = addr + off as u64;
            if vma % FUNC_ALIGN == 0 && pats.iter().any(|p| p.matches(&data[off..])) {
                out.push(vma);
            }
            off += 1;
        }
    }
    out
}

// ===========================================================================
// Oracle 3: .eh_frame FDE pcBegin (GccExceptionAnalyzer, FDE-start scope)
// ===========================================================================

// DWARF exception-handling pointer-encoding constants (DW_EH_PE_*), the modes
// DwarfDecoderFactory.getDecoder splits on (format = enc&0x0f, appl = enc&0x70,
// indirect = enc&0x80).
const DW_EH_PE_ABSPTR: u8 = 0x00;
const DW_EH_PE_ULEB128: u8 = 0x01;
const DW_EH_PE_UDATA2: u8 = 0x02;
const DW_EH_PE_UDATA4: u8 = 0x03;
const DW_EH_PE_UDATA8: u8 = 0x04;
const DW_EH_PE_SLEB128: u8 = 0x09;
const DW_EH_PE_SDATA2: u8 = 0x0a;
const DW_EH_PE_SDATA4: u8 = 0x0b;
const DW_EH_PE_SDATA8: u8 = 0x0c;

const DW_EH_PE_PCREL: u8 = 0x10;
const DW_EH_PE_DATAREL: u8 = 0x30;
const DW_EH_PE_OMIT: u8 = 0xff;

/// Scan `.eh_frame` and return every FDE `pcBegin` (each is a function start) —
/// the byproduct of Ghidra's `FrameDescriptionEntry.create`'s
/// `CreateFunctionCmd(pcBeginAddr)` (FrameDescriptionEntry.java:473), scoped to
/// the FDE-start decode (NOT pcRange/CFI/LSDA).
///
/// Walk (faithful to `EhFrameSection.analyzeSection`): each record is
/// `length:u32`, then `cieId:u32`. `cieId == 0` ⇒ CIE (the `.eh_frame`
/// convention) — parse its augmentation to extract the FDE pointer-encoding
/// byte. `cieId != 0` ⇒ FDE — its CIE is at `(o+4) - cieId`
/// (`createCiePointer`:225); decode `pcBegin` at `o+8` with that CIE's encoding.
///
/// Default ptr size 8 (x86-64) for the absptr format — fixtures are all
/// `pcrel|sdata4`, so it is unused there.
pub fn scan_eh_frame_starts(file: &object::File) -> Vec<u64> {
    scan_eh_frame_starts_sized(file, 8)
}

/// As [`scan_eh_frame_starts`] but with an explicit absptr pointer size.
fn scan_eh_frame_starts_sized(file: &object::File, ptr_size: usize) -> Vec<u64> {
    let mut out: Vec<u64> = Vec::new();
    if !matches!(file.format(), object::BinaryFormat::Elf) {
        return out;
    }
    let Some(sec) = file.section_by_name(".eh_frame") else {
        return out;
    };
    let sec_vma = sec.address();
    let Ok(data) = sec.data() else {
        return out;
    };
    if data.is_empty() {
        return out;
    }
    let le = file.is_little_endian();

    // CIE-offset -> FDE pointer-encoding byte. A flat Vec lookup (no HashMap),
    // resolved by the FDE's `(o+4) - cieId` back-pointer; faithful given CIEs
    // precede their FDEs in gcc output (and all vendored fixtures). A forward-
    // referencing CIE would be missed — documented LOSS.
    let mut cie_enc: Vec<(usize, u8)> = Vec::new();

    let mut o = 0usize;
    while o + 4 <= data.len() {
        let length = read_u32(&data[o..], le) as usize;
        if length == 0 {
            break; // zero-length record = end-of-frame
        }
        if length == 0xffff_ffff {
            // 64-bit extended length record. Ghidra throws "ExtLength not
            // completely implemented" (Cie.java:520 / FDE :499). SCOPE: read the
            // 8-byte length and skip the record.
            if o + 12 > data.len() {
                break;
            }
            let ext = read_u64(&data[o + 4..], le) as usize;
            o = match o.checked_add(12).and_then(|v| v.checked_add(ext)) {
                Some(v) => v,
                None => break,
            };
            continue;
        }
        let next = o + 4 + length;
        if next > data.len() || o + 8 > data.len() {
            break;
        }
        let cie_id = read_u32(&data[o + 4..], le);

        if cie_id == 0 {
            // CIE: extract the FDE pointer encoding byte from its augmentation.
            let enc = parse_cie_fde_encoding(&data[o..next], ptr_size).unwrap_or(DW_EH_PE_ABSPTR);
            cie_enc.push((o, enc));
        } else {
            // FDE: locate its CIE by the relative back-pointer.
            let cie_ptr_field = o + 4;
            if (cie_id as usize) <= cie_ptr_field {
                let cie_off = cie_ptr_field - cie_id as usize;
                let enc = cie_enc.iter().find(|&&(co, _)| co == cie_off).map(|&(_, e)| e);
                if let Some(enc) = enc {
                    // pcBegin field starts at o+8.
                    let field_vma = sec_vma + (o + 8) as u64;
                    if let Some(pc) =
                        decode_eh_pointer(enc, field_vma, sec_vma, data, o + 8, ptr_size)
                    {
                        out.push(pc);
                    }
                }
            }
        }
        o = next;
    }

    out.sort_unstable();
    out.dedup();
    out
}

// ===========================================================================
// `.eh_frame` LSDA landing pads (GccExceptionAnalyzer, full markup) — GATED
// ===========================================================================
//
// The `--option eh_frame_full on` product: for each FDE whose CIE augmentation
// carries an `L` char, follow the FDE's augmentation-data LSDA pointer into
// `.gcc_except_table`, decode the call-site table, and emit each non-zero
// landing pad (`lpStart + cs_landing_pad`) as a discovered code entry. A landing
// pad is the catch/cleanup block the unwinder jumps to — a real code target that
// the FDE-pcBegin / prologue / call-idiom oracles never see (it sits mid-function).
//
// Faithful to Ghidra's `ghidra.app.plugin.exceptionhandlers.gcc.*`:
//   - `Cie.processAugmentationInfo` records the `L` char's LSDA-encoding byte
//     (and the `P` personality, `R` FDE-encoding) — `getLSDAEncoding()`.
//   - `FrameDescriptionEntry.createAugmentationInfo`/`createLsda` decodes the FDE's
//     own augmentation-data LSDA pointer with that CIE encoding.
//   - `LSDAHeader.create` reads `[lpStartEnc][lpStart?][ttypeEnc][ttypeOff?]
//     [callSiteEnc][callSiteTableLen]`; when `lpStartEnc == omit`, `lpStart`
//     defaults to the FDE's pcBegin (the region/function start).
//   - `LSDACallSiteRecord.create` reads `[cs_start][cs_len][cs_lp][cs_action]`;
//     `getLandingPad()` is `lpStart + cs_lp`, and `cs_lp == 0` ⇒ NO landing pad
//     (cleanup-less call site). `GccExceptionAnalyzer.processCallSiteRecord`
//     disassembles each non-zero landing pad (marks it code) — our entry fact.
//
// CFI (the `DW_CFA_*` call-frame instructions giving CFA/register-save rules) is
// deliberately NOT recovered here: kuna's own engine recovers the stack frame from
// the code (S5 type inference + S7 frame analysis), so the CFA/saved-register
// rules add nothing at the decompiler tier — CFI is INHERITED, not rebuilt. See
// `docs/analysis-port-log.md`.

/// Decoded CIE augmentation fields relevant to the LSDA walk: the FDE pointer
/// encoding (`R`), and — when the augmentation carries an `L` — the LSDA pointer
/// encoding (`L`). `has_lsda == false` ⇒ this CIE's FDEs carry no LSDA pointer.
#[derive(Clone, Copy)]
struct CieAug {
    /// The FDE `pcBegin`/`pcRange` pointer encoding (the `R` char payload).
    fde_enc: u8,
    /// True iff the augmentation string contains an `L` char (FDEs carry an LSDA
    /// pointer in their augmentation data).
    has_lsda: bool,
    /// The LSDA pointer encoding (the `L` char payload), valid iff `has_lsda`.
    lsda_enc: u8,
    /// The byte offset, within the FDE augmentation-data block, at which the LSDA
    /// pointer sits. It is the FIRST aug-data field for a `zL`/`zPL`/`zPLR` CIE
    /// (the `L` data is the LSDA pointer; `P`/`R` consume CIE aug-data, not FDE
    /// aug-data) — so always 0 in practice; kept explicit for faithfulness.
    fde_lsda_off: usize,
}

/// As [`scan_eh_frame_starts`] but, for `--option eh_frame_full on`, return the
/// exception-handler **landing-pad** PCs decoded from each FDE's
/// `.gcc_except_table` LSDA (the GccExceptionAnalyzer full-markup product). The
/// raw decode (NOT exec-section/funcsym filtered — see
/// [`collect_landing_pad_entries`] for the filtered entry set).
pub fn scan_eh_frame_landing_pads(file: &object::File) -> Vec<u64> {
    scan_eh_frame_landing_pads_sized(file, 8)
}

fn scan_eh_frame_landing_pads_sized(file: &object::File, ptr_size: usize) -> Vec<u64> {
    let mut out: Vec<u64> = Vec::new();
    if !matches!(file.format(), object::BinaryFormat::Elf) {
        return out;
    }
    let Some(sec) = file.section_by_name(".eh_frame") else {
        return out;
    };
    let sec_vma = sec.address();
    let Ok(data) = sec.data() else {
        return out;
    };
    if data.is_empty() {
        return out;
    }
    // The `.gcc_except_table` bytes, read by VMA (the LSDA + call-site table live
    // here, not in `.eh_frame`). Absent ⇒ no landing pads.
    let Some(gcc) = file.section_by_name(".gcc_except_table") else {
        return out;
    };
    let gcc_vma = gcc.address();
    let Ok(gcc_data) = gcc.data() else {
        return out;
    };
    let le = file.is_little_endian();

    // CIE-offset -> decoded augmentation (FDE encoding + LSDA encoding). A flat Vec
    // lookup (no HashMap), resolved by the FDE's `(o+4) - cieId` back-pointer —
    // faithful given CIEs precede their FDEs in gcc output. A forward-referencing
    // CIE is missed (documented LOSS, same as the FDE-start scan).
    let mut cie_aug: Vec<(usize, CieAug)> = Vec::new();

    let mut o = 0usize;
    while o + 4 <= data.len() {
        let length = read_u32(&data[o..], le) as usize;
        if length == 0 {
            break; // zero-length record = end-of-frame
        }
        if length == 0xffff_ffff {
            // 64-bit extended length: read + skip (same scope as the FDE-start scan).
            if o + 12 > data.len() {
                break;
            }
            let ext = read_u64(&data[o + 4..], le) as usize;
            o = match o.checked_add(12).and_then(|v| v.checked_add(ext)) {
                Some(v) => v,
                None => break,
            };
            continue;
        }
        let next = o + 4 + length;
        if next > data.len() || o + 8 > data.len() {
            break;
        }
        let cie_id = read_u32(&data[o + 4..], le);

        if cie_id == 0 {
            // CIE: decode the FDE + LSDA encodings from its augmentation.
            if let Some(aug) = parse_cie_aug(&data[o..next], ptr_size) {
                cie_aug.push((o, aug));
            }
        } else {
            // FDE: locate its CIE by the relative back-pointer.
            let cie_ptr_field = o + 4;
            if (cie_id as usize) > cie_ptr_field {
                o = next;
                continue;
            }
            let cie_off = cie_ptr_field - cie_id as usize;
            let Some(&(_, aug)) = cie_aug.iter().find(|&&(co, _)| co == cie_off) else {
                o = next;
                continue;
            };
            // This FDE carries an LSDA only if its CIE augmentation had an `L`.
            if !aug.has_lsda {
                o = next;
                continue;
            }
            // FDE layout: [length:u32][ciePtr:u32][pcBegin][pcRange][augLen(uleb)]
            //             [augData: lsdaPtr ...][cfi...]. pcBegin starts at o+8.
            let pc_begin_off = o + 8;
            let pc_begin_vma = sec_vma + pc_begin_off as u64;
            let Some(pc_begin) =
                decode_eh_pointer(aug.fde_enc, pc_begin_vma, sec_vma, data, pc_begin_off, ptr_size)
            else {
                o = next;
                continue;
            };
            // Skip pcBegin + pcRange (both the FDE `R` encoding's size), then the
            // augmentation-data length (uleb), to reach the FDE aug-data block.
            let enc_sz = encoded_size(aug.fde_enc, ptr_size, data.get(pc_begin_off..).unwrap_or(&[]));
            let pc_range_off = pc_begin_off + enc_sz;
            let aug_len_off = pc_range_off + enc_sz;
            let Some((_aug_len, aug_data_off)) = read_uleb128(data, aug_len_off) else {
                o = next;
                continue;
            };
            // The LSDA pointer field within the FDE aug-data, decoded with the
            // CIE's `L` encoding. Its VMA is needed for a pcrel `L` encoding.
            let lsda_field_off = aug_data_off + aug.fde_lsda_off;
            let lsda_field_vma = sec_vma + lsda_field_off as u64;
            let Some(lsda_ptr) = decode_eh_pointer(
                aug.lsda_enc,
                lsda_field_vma,
                sec_vma,
                data,
                lsda_field_off,
                ptr_size,
            ) else {
                o = next;
                continue;
            };
            if lsda_ptr == 0 {
                o = next; // no LSDA for this FDE.
                continue;
            }
            // Decode the `.gcc_except_table` LSDA at `lsda_ptr`. lpStart defaults to
            // the FDE's pcBegin when the LSDA omits its own lpStart.
            decode_lsda_landing_pads(
                gcc_data, gcc_vma, lsda_ptr, pc_begin, ptr_size, &mut out,
            );
        }
        o = next;
    }

    out.sort_unstable();
    out.dedup();
    out
}

/// Decode the `.gcc_except_table` LSDA at VMA `lsda_vma` and push each non-zero
/// landing pad (`lpStart + cs_landing_pad`) into `out`. `func_pc_begin` is the
/// FDE's pcBegin — the default `lpStart` when the LSDA's `lpStartEncoding` is
/// `omit`. Faithful to `LSDAHeader.create` + `LSDACallSiteRecord.create`.
fn decode_lsda_landing_pads(
    gcc_data: &[u8],
    gcc_vma: u64,
    lsda_vma: u64,
    func_pc_begin: u64,
    ptr_size: usize,
    out: &mut Vec<u64>,
) {
    // Section-relative offset of the LSDA header.
    let Some(base) = lsda_vma.checked_sub(gcc_vma) else {
        return;
    };
    let base = base as usize;
    if base >= gcc_data.len() {
        return;
    }
    let mut p = base;

    // lpStartEncoding (1 byte). When omit, lpStart = func_pc_begin; else decode a
    // pointer of that encoding (the rebased landing-pad base).
    let Some(&lp_enc) = gcc_data.get(p) else {
        return;
    };
    p += 1;
    let lp_start: u64 = if lp_enc == DW_EH_PE_OMIT {
        func_pc_begin
    } else {
        let field_vma = gcc_vma + p as u64;
        let Some(v) = decode_eh_pointer(lp_enc, field_vma, gcc_vma, gcc_data, p, ptr_size) else {
            return;
        };
        p += encoded_size(lp_enc, ptr_size, gcc_data.get(p..).unwrap_or(&[]));
        v
    };

    // ttypeEncoding (1 byte) + ttypeOffset (uleb) iff not omit. We do not consume
    // the types table (only the call-site landing pads matter for code discovery),
    // but we must SKIP the ttypeOffset uleb to reach the call-site header.
    let Some(&tt_enc) = gcc_data.get(p) else {
        return;
    };
    p += 1;
    if tt_enc != DW_EH_PE_OMIT {
        let Some((_tt_off, np)) = read_uleb128(gcc_data, p) else {
            return;
        };
        p = np;
    }

    // callSiteEncoding (1 byte) + callSiteTableLength (uleb).
    let Some(&cs_enc) = gcc_data.get(p) else {
        return;
    };
    p += 1;
    let Some((cs_table_len, np)) = read_uleb128(gcc_data, p) else {
        return;
    };
    p = np;
    let table_end = match p.checked_add(cs_table_len as usize) {
        Some(e) if e <= gcc_data.len() => e,
        _ => return,
    };

    // Call-site records: [cs_start][cs_len][cs_landing_pad] (all `cs_enc`-encoded),
    // then [cs_action] (uleb). cs_landing_pad == 0 ⇒ no landing pad.
    let mut guard = 0usize;
    while p < table_end {
        // Bound the loop: each record consumes ≥4 bytes; cap iterations defensively.
        guard += 1;
        if guard > 1 << 20 {
            break;
        }
        // cs_start (skipped — the try-block start, not a code entry we emit).
        let Some(np) = skip_eh_pointer(cs_enc, gcc_data, p, ptr_size) else {
            break;
        };
        p = np;
        // cs_len (skipped — try-block length).
        let Some(np) = skip_eh_pointer(cs_enc, gcc_data, p, ptr_size) else {
            break;
        };
        p = np;
        // cs_landing_pad (the offset from lpStart). Decoded as a DW_EH_PE value —
        // in practice `udata4`/`uleb128` with `appl == absptr`, so this is a plain
        // offset; `decode_eh_pointer` applies any `appl` exactly as for a pointer.
        let lp_field_vma = gcc_vma + p as u64;
        let Some(cs_lp) = decode_eh_pointer(cs_enc, lp_field_vma, gcc_vma, gcc_data, p, ptr_size)
        else {
            break;
        };
        let Some(np) = skip_eh_pointer(cs_enc, gcc_data, p, ptr_size) else {
            break;
        };
        p = np;
        // cs_action (uleb, skipped — index into the action table).
        let Some((_action, np)) = read_uleb128(gcc_data, p) else {
            break;
        };
        p = np;

        if cs_lp != 0 {
            out.push(lp_start.wrapping_add(cs_lp));
        }
    }
}

/// Parse a CIE record's augmentation and return its decoded [`CieAug`] (the FDE
/// `R` encoding plus the LSDA `L` encoding / presence), or `None` on a malformed
/// record. The walk mirrors [`parse_cie_fde_encoding`] but tracks the `L` char's
/// 1-byte LSDA encoding payload (`Cie.processLsdaEncoding`) alongside the `R`
/// char's FDE encoding (`Cie.processFdeEncoding`).
fn parse_cie_aug(rec: &[u8], ptr_size: usize) -> Option<CieAug> {
    let mut p = 8usize; // skip length + cieId
    let version = *rec.get(p)?;
    p += 1;

    let aug_start = p;
    while p < rec.len() && rec[p] != 0 {
        p += 1;
    }
    if p >= rec.len() {
        return None;
    }
    let aug = &rec[aug_start..p];
    p += 1; // skip NUL

    if version >= 4 {
        p += 2; // address_size, segment_selector_size
    }
    let (_, np) = read_uleb128(rec, p)?; // code_alignment_factor
    p = np;
    let (_, np) = read_sleb128(rec, p)?; // data_alignment_factor
    p = np;
    if version == 1 {
        p += 1; // return_address_register (u8)
    } else {
        let (_, np) = read_uleb128(rec, p)?; // return_address_register (uleb)
        p = np;
    }

    // Defaults: no `L`, FDE encoding absptr. Only a `z`-augmentation carries the
    // aug-data block where the `R`/`L`/`P` payloads live.
    let mut fde_enc = DW_EH_PE_ABSPTR;
    let mut has_lsda = false;
    let mut lsda_enc = DW_EH_PE_ABSPTR;
    if aug.first() != Some(&b'z') {
        return Some(CieAug { fde_enc, has_lsda, lsda_enc, fde_lsda_off: 0 });
    }
    let (aug_len, np) = read_uleb128(rec, p)?;
    p = np;
    let aug_data_start = p;
    let aug_data_end = (aug_data_start + aug_len as usize).min(rec.len());

    let mut dp = aug_data_start;
    for &c in &aug[1..] {
        match c {
            b'R' => {
                fde_enc = *rec.get(dp)?;
                dp += 1;
            }
            b'L' => {
                has_lsda = true;
                lsda_enc = *rec.get(dp)?;
                dp += 1;
            }
            b'P' => {
                let enc = *rec.get(dp)?;
                dp += 1;
                dp += encoded_size(enc, ptr_size, rec.get(dp..).unwrap_or(&[]));
            }
            b'S' => {}
            _ => {}
        }
        if dp > aug_data_end {
            break;
        }
    }
    // The FDE aug-data LSDA pointer is the FIRST FDE aug-data field for the
    // `zL`/`zPL`/`zPLR` augmentations gcc emits (the `R` char only sizes the FDE
    // pcBegin/pcRange; `P`'s personality is CIE aug-data). Offset 0.
    Some(CieAug { fde_enc, has_lsda, lsda_enc, fde_lsda_off: 0 })
}

/// Advance past a DW_EH_PE-encoded value at `bytes[off..]`, returning the next
/// offset (or `None` past end-of-buffer).
fn skip_eh_pointer(enc: u8, bytes: &[u8], off: usize, ptr_size: usize) -> Option<usize> {
    let sz = encoded_size(enc, ptr_size, bytes.get(off..)?);
    let next = off.checked_add(sz)?;
    if next > bytes.len() {
        return None;
    }
    Some(next)
}

/// The filtered landing-pad entry set: [`scan_eh_frame_landing_pads`] put through
/// the SAME exec-section + funcsym-skip + dedup filter every entry oracle uses, so
/// a landing pad that coincides with a real funcsym (or falls outside an
/// executable section) is dropped. Purely additive — only ever ADDS entries.
pub fn collect_landing_pad_entries(file: &object::File, bytes: &[u8]) -> Vec<u64> {
    let pads = scan_eh_frame_landing_pads(file);
    if pads.is_empty() {
        return Vec::new();
    }
    let execs = executable_sections(file);
    let funcsyms = existing_function_addrs(file, bytes);
    let mut out: Vec<u64> = Vec::new();
    for vma in pads {
        if vma == 0 || !in_executable_section(&execs, vma) {
            continue;
        }
        if funcsyms.binary_search(&vma).is_ok() {
            continue;
        }
        out.push(vma);
    }
    out.sort_unstable();
    out.dedup();
    out
}

/// Parse a CIE record's augmentation and return its FDE pointer-encoding byte
/// (the `R` char's payload), or `None` if the augmentation does not specify one.
/// Faithful to `Cie.processAugmentationString`/`processAugmentationInfo`
/// (Cie.java:204-222, 559-633): walk version / aug-string / [v4: ptrsize,segsize]
/// / code-align(ULEB) / data-align(SLEB) / return-addr-reg / if aug starts with
/// 'z': aug-data-len(ULEB) then the aug-data, in which each aug-string char after
/// 'z' is handled — 'R' → next byte is fdeEncoding; 'L' → 1 byte; 'P' → 1 enc
/// byte + a personality pointer of that enc's size; 'S' → 0.
fn parse_cie_fde_encoding(rec: &[u8], ptr_size: usize) -> Option<u8> {
    // rec = [length:u32][cieId:u32][version:u8][aug-string\0]...
    let mut p = 8usize; // skip length + cieId
    let version = *rec.get(p)?;
    p += 1;

    // Augmentation string (NUL-terminated).
    let aug_start = p;
    while p < rec.len() && rec[p] != 0 {
        p += 1;
    }
    if p >= rec.len() {
        return None;
    }
    let aug = &rec[aug_start..p];
    p += 1; // skip NUL

    if version >= 4 {
        // address_size (u8), segment_selector_size (u8).
        p += 2;
    }

    // code_alignment_factor (ULEB), data_alignment_factor (SLEB).
    let (_, np) = read_uleb128(rec, p)?;
    p = np;
    let (_, np) = read_sleb128(rec, p)?;
    p = np;

    // return_address_register: v1 → u8, else ULEB.
    if version == 1 {
        p += 1;
    } else {
        let (_, np) = read_uleb128(rec, p)?;
        p = np;
    }

    // Only a 'z' augmentation carries aug-data (and thus an encoding byte).
    if aug.first() != Some(&b'z') {
        return None;
    }
    // aug-data length (ULEB), then the aug-data block.
    let (aug_len, np) = read_uleb128(rec, p)?;
    p = np;
    let aug_data_start = p;
    let aug_data_end = (aug_data_start + aug_len as usize).min(rec.len());

    // Walk the aug-string chars after 'z' against the aug-data block.
    let mut dp = aug_data_start;
    for &c in &aug[1..] {
        match c {
            b'R' => {
                // FDE pointer encoding byte.
                return rec.get(dp).copied();
            }
            b'L' => {
                dp += 1; // LSDA encoding byte (ignored).
            }
            b'P' => {
                // personality: 1 encoding byte + a pointer of that enc's size.
                let enc = *rec.get(dp)?;
                dp += 1;
                dp += encoded_size(enc, ptr_size, rec.get(dp..).unwrap_or(&[]));
            }
            b'S' => {} // signal frame: consumes nothing.
            _ => {}
        }
        if dp > aug_data_end {
            break;
        }
    }
    None
}

/// Byte size of a DW_EH_PE-encoded value (for skipping the personality pointer).
/// LEB128 sizes are measured from the trailing bytes.
fn encoded_size(enc: u8, ptr_size: usize, rest: &[u8]) -> usize {
    if enc == DW_EH_PE_OMIT {
        return 0;
    }
    match enc & 0x0f {
        DW_EH_PE_ABSPTR => ptr_size,
        DW_EH_PE_UDATA2 | DW_EH_PE_SDATA2 => 2,
        DW_EH_PE_UDATA4 | DW_EH_PE_SDATA4 => 4,
        DW_EH_PE_UDATA8 | DW_EH_PE_SDATA8 => 8,
        DW_EH_PE_ULEB128 | DW_EH_PE_SLEB128 => read_uleb128(rest, 0).map(|(_, n)| n).unwrap_or(1),
        _ => ptr_size,
    }
}

/// Decode a DW_EH_PE-encoded FDE `pcBegin` pointer at `bytes[field_off..]` whose
/// field lives at `field_vma`. Faithful to `DwarfDecoderFactory` +
/// `AbstractDwarfEHDecoder.resolveRelativeOffset`: read the raw value by `format`,
/// then apply `appl` (pcrel = field_vma + raw; datarel = section_vma + raw;
/// absptr = raw as-is — kuna loads at the file vmas so the image-base adjustment
/// is 0). `indirect` (enc & 0x80) is unresolvable without a runtime relocation —
/// skipped as a documented LOSS (never in the fixtures).
fn decode_eh_pointer(
    enc: u8,
    field_vma: u64,
    section_vma: u64,
    bytes: &[u8],
    field_off: usize,
    ptr_size: usize,
) -> Option<u64> {
    if enc == DW_EH_PE_OMIT {
        return None;
    }
    if enc & 0x80 != 0 {
        return None; // indirect — needs the runtime relocated pointer.
    }
    let format = enc & 0x0f;
    let appl = enc & 0x70;
    let slice = bytes.get(field_off..)?;

    let raw: u64 = match format {
        DW_EH_PE_ABSPTR => {
            if ptr_size == 8 {
                read_u64_opt(slice)?
            } else {
                read_u32_opt(slice)? as u64
            }
        }
        DW_EH_PE_UDATA2 => read_u16_opt(slice)? as u64,
        DW_EH_PE_SDATA2 => read_u16_opt(slice)? as i16 as i64 as u64,
        DW_EH_PE_UDATA4 => read_u32_opt(slice)? as u64,
        DW_EH_PE_SDATA4 => read_u32_opt(slice)? as i32 as i64 as u64,
        DW_EH_PE_UDATA8 => read_u64_opt(slice)?,
        DW_EH_PE_SDATA8 => read_u64_opt(slice)?,
        DW_EH_PE_ULEB128 => read_uleb128(bytes, field_off)?.0,
        DW_EH_PE_SLEB128 => read_sleb128(bytes, field_off)?.0 as u64,
        _ => return None,
    };

    let val = match appl {
        DW_EH_PE_PCREL => field_vma.wrapping_add(raw),
        DW_EH_PE_DATAREL => section_vma.wrapping_add(raw),
        0x00 => raw, // absptr (image base is 0 — no rebase).
        _ => raw,    // funcrel/aligned/textrel unused for FDE pcBegin → treat as absptr.
    };
    Some(val)
}

// ===========================================================================
// Little/big-endian + LEB128 byte readers
// ===========================================================================

fn read_u16_opt(b: &[u8]) -> Option<u16> {
    Some(u16::from_le_bytes([*b.first()?, *b.get(1)?]))
}

fn read_u32(b: &[u8], le: bool) -> u32 {
    let a = [b[0], b[1], b[2], b[3]];
    if le {
        u32::from_le_bytes(a)
    } else {
        u32::from_be_bytes(a)
    }
}

fn read_u32_opt(b: &[u8]) -> Option<u32> {
    Some(u32::from_le_bytes([*b.first()?, *b.get(1)?, *b.get(2)?, *b.get(3)?]))
}

fn read_u64(b: &[u8], le: bool) -> u64 {
    let a = [b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]];
    if le {
        u64::from_le_bytes(a)
    } else {
        u64::from_be_bytes(a)
    }
}

fn read_u64_opt(b: &[u8]) -> Option<u64> {
    if b.len() < 8 {
        return None;
    }
    Some(u64::from_le_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]]))
}

fn read_i32(b: &[u8]) -> i32 {
    i32::from_le_bytes([b[0], b[1], b[2], b[3]])
}

/// Read an unsigned LEB128 at `bytes[off..]`, returning `(value, next_off)`.
fn read_uleb128(bytes: &[u8], off: usize) -> Option<(u64, usize)> {
    let mut result: u64 = 0;
    let mut shift = 0u32;
    let mut p = off;
    loop {
        let b = *bytes.get(p)?;
        p += 1;
        result |= ((b & 0x7f) as u64) << shift;
        if b & 0x80 == 0 {
            break;
        }
        shift += 7;
        if shift >= 64 {
            break;
        }
    }
    Some((result, p))
}

/// Read a signed LEB128 at `bytes[off..]`, returning `(value, next_off)`.
fn read_sleb128(bytes: &[u8], off: usize) -> Option<(i64, usize)> {
    let mut result: i64 = 0;
    let mut shift = 0u32;
    let mut p = off;
    let mut byte;
    loop {
        byte = *bytes.get(p)?;
        p += 1;
        result |= ((byte & 0x7f) as i64) << shift;
        shift += 7;
        if byte & 0x80 == 0 {
            break;
        }
        if shift >= 64 {
            break;
        }
    }
    if shift < 64 && byte & 0x40 != 0 {
        result |= -1i64 << shift;
    }
    Some((result, p))
}

// ===========================================================================
// Tests
// ===========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture(name: &str) -> Vec<u8> {
        let path = format!("{}/tests/fixtures/{}", env!("CARGO_MANIFEST_DIR"), name);
        std::fs::read(&path).unwrap_or_else(|_| panic!("read fixture {path}"))
    }

    // -- Oracle 3: .eh_frame FDE starts (fauxware, the s1-eh-frame headline) ---

    #[test]
    fn eh_frame_starts_fauxware() {
        let bytes = fixture("fauxware");
        let file = object::File::parse(bytes.as_slice()).expect("parse fauxware");
        let starts = scan_eh_frame_starts(&file);
        // readelf --debug-dump=frames ground truth.
        for want in [0x400500u64, 0x400664, 0x4006ed, 0x4006fd, 0x40071d, 0x4007e0, 0x400870] {
            assert!(starts.contains(&want), "FDE start {want:#x} missing from {starts:#x?}");
        }
        // The known funcsyms (authenticate=0x400664, accepted=0x4006ed,
        // rejected=0x4006fd, main=0x40071d) are a subset — the oracle property.
        assert!(!starts.contains(&0), "no spurious 0 start");
    }

    // First FDE decode by hand: pcBegin field at vma 0x400990 holds `70 fb ff ff`
    // = -1168 (sdata4), pcrel: 0x400990 + (-1168) = 0x400500.
    #[test]
    fn eh_frame_first_fde_pcrel_math() {
        let bytes = fixture("fauxware");
        let file = object::File::parse(bytes.as_slice()).expect("parse fauxware");
        let starts = scan_eh_frame_starts(&file);
        assert!(starts.contains(&0x400500), "first FDE pcBegin should be 0x400500");
    }

    // -- `.eh_frame` LSDA landing pads (eh_lsda_x86_64, the gated headline) ----

    // The C++ try/catch fixture (stripped of `.symtab`): each FDE's `zPLR` CIE
    // carries an `L` LSDA encoding, so `scan_eh_frame_landing_pads` follows the
    // FDE aug-data LSDA pointer into `.gcc_except_table`, decodes the call-site
    // table, and yields the exception-handler landing pads. Ground truth (decoded
    // by hand + cross-checked against `objdump -d`, see fixture README):
    //   may_throw LSDA @0x40218c → landing 0x4012bf
    //   guarded   LSDA @0x402198 → landings 0x4012e2, 0x401352, 0x401366
    // Every landing pad lands on an `endbr64` (a real unwinder-only code target).
    #[test]
    fn eh_frame_landing_pads_eh_lsda() {
        let bytes = fixture("eh_lsda_x86_64");
        let file = object::File::parse(bytes.as_slice()).expect("parse eh_lsda_x86_64");
        let pads = scan_eh_frame_landing_pads(&file);
        for want in [0x4012bfu64, 0x4012e2, 0x401352, 0x401366] {
            assert!(pads.contains(&want), "landing pad {want:#x} missing from {pads:#x?}");
        }
        assert!(!pads.contains(&0), "no spurious 0 landing pad");
    }

    // The landing pads are NOT FDE pcBegins (function starts) — they sit
    // mid-function inside `may_throw`/`guarded` and are reached only by the
    // unwinder, so the FDE-start oracle never finds them. This is the property
    // that makes the LSDA oracle net-new (it adds entries no other oracle has).
    #[test]
    fn landing_pads_are_not_fde_starts() {
        let bytes = fixture("eh_lsda_x86_64");
        let file = object::File::parse(bytes.as_slice()).expect("parse eh_lsda_x86_64");
        let starts = scan_eh_frame_starts(&file);
        let pads = scan_eh_frame_landing_pads(&file);
        for p in &pads {
            assert!(
                !starts.contains(p),
                "landing pad {p:#x} must NOT be an FDE start (it is mid-function)"
            );
        }
        // And the filtered entry set keeps them (none coincide with a funcsym in
        // this stripped binary) — the additive product the gated option commits.
        let entries = collect_landing_pad_entries(&file, &bytes);
        for want in [0x4012bfu64, 0x4012e2, 0x401352, 0x401366] {
            assert!(entries.contains(&want), "filtered entry {want:#x} missing from {entries:#x?}");
        }
    }

    // A binary with no `.gcc_except_table` (fauxware: C, no exceptions) yields no
    // landing pads — the oracle is inert there, so the gated option is a strict
    // no-op on a non-exception binary.
    #[test]
    fn landing_pads_empty_without_except_table() {
        let bytes = fixture("fauxware");
        let file = object::File::parse(bytes.as_slice()).expect("parse fauxware");
        assert!(file.section_by_name(".gcc_except_table").is_none(), "fauxware has no LSDA");
        assert!(scan_eh_frame_landing_pads(&file).is_empty(), "no landing pads without LSDA");
    }

    // Unit-test the LSDA call-site-table parse over SYNTHETIC bytes (no fixture):
    // exactly the `guarded` LSDA layout (lpStart=omit → func pcBegin, callSiteEnc=
    // uleb128) decoded in isolation. Proves the header walk (lpStartEnc/ttypeEnc/
    // ttypeOff/callSiteEnc/callSiteLen) + the record walk (cs_start/cs_len/cs_lp/
    // cs_action) + the `lpStart + cs_lp` math + the `cs_lp == 0 ⇒ none` rule.
    #[test]
    fn lsda_call_site_table_parse_synthetic() {
        // `.gcc_except_table` content (just the one LSDA at section offset 0):
        //   lpStartEnc = 0xff (omit)              → lpStart = func_pc_begin
        //   ttypeEnc   = 0x03 (udata4, present)
        //   ttypeOff   = 0x25 (uleb)
        //   callSiteEnc= 0x01 (uleb128)
        //   csTableLen = 0x16 (22 bytes)
        //   records (uleb cs_start, cs_len, cs_lp, action):
        //     05 05 0c 03  → cs_lp=0x0c → landing pc_begin+0x0c
        //     1f 05 00 00  → cs_lp=0    → NONE
        //     44 05 7c 00  → cs_lp=0x7c → landing pc_begin+0x7c
        //     6b 05 90 01 00 → cs_lp uleb(90 01)=0x90 → +0x90
        //     8b 01 19 00 00 → cs_start uleb(8b 01)=0x8b, cs_lp=0 → NONE
        // csTableLen = 0x16 (22) = 4+4+4+5+5. (mirrors the real fixture bytes,
        // README-pinned; the action/types tables AFTER the call-site table are
        // unread, so they are omitted here.)
        let gcc: &[u8] = &[
            0xff, 0x03, 0x25, 0x01, 0x16, // header: lpEnc, ttEnc, ttOff, csEnc, csLen=22
            0x05, 0x05, 0x0c, 0x03, // record 1 (4) → +0x0c
            0x1f, 0x05, 0x00, 0x00, // record 2 (4) → none
            0x44, 0x05, 0x7c, 0x00, // record 3 (4) → +0x7c
            0x6b, 0x05, 0x90, 0x01, 0x00, // record 4 (5) → +0x90 (cs_lp uleb 0x90,0x01)
            0x8b, 0x01, 0x19, 0x00, 0x00, // record 5 (5) → none (cs_start uleb 0x8b,0x01)
        ];
        let func_pc_begin = 0x4012d6u64;
        let mut out: Vec<u64> = Vec::new();
        // gcc_vma=0, lsda_vma=0 (section offset 0), ptr_size 8.
        decode_lsda_landing_pads(gcc, 0, 0, func_pc_begin, 8, &mut out);
        out.sort_unstable();
        out.dedup();
        assert_eq!(
            out,
            vec![
                func_pc_begin + 0x0c, // 0x4012e2
                func_pc_begin + 0x7c, // 0x401352
                func_pc_begin + 0x90, // 0x401366
            ],
            "call-site table landing pads (zero cs_lp records dropped)"
        );
    }

    // -- Oracle 1+2: entry / init / fini (stripped_dynamic) -------------------

    #[test]
    fn dynamic_entry_points_stripped() {
        let bytes = fixture("stripped_dynamic_x86_64");
        let file = object::File::parse(bytes.as_slice()).expect("parse stripped_dynamic");
        let eps = dynamic_entry_points(&file);
        assert!(eps.contains(&0x1000), "DT_INIT 0x1000 missing from {eps:#x?}");
        assert!(eps.contains(&0x1464), "DT_FINI 0x1464 missing from {eps:#x?}");
        // INIT_ARRAY (1 ptr @0x3d78 → 0x1240 frame_dummy), FINI_ARRAY (→ 0x1200).
        assert!(eps.contains(&0x1240), "INIT_ARRAY ptr 0x1240 missing from {eps:#x?}");
        assert!(eps.contains(&0x1200), "FINI_ARRAY ptr 0x1200 missing from {eps:#x?}");
    }

    // -- Oracle 2 naming overlay: Ghidra `_INIT_<i>`/`_FINI_<i>`/`_DT_INIT` --------

    #[test]
    fn dynamic_entry_names_stripped() {
        let bytes = fixture("stripped_dynamic_x86_64");
        let file = object::File::parse(bytes.as_slice()).expect("parse stripped_dynamic");
        let names = dynamic_entry_names(&file);
        // Single DT_INIT/DT_FINI → `_DT_INIT`/`_DT_FINI` ("_" + ElfDynamicType.name).
        assert!(
            names.contains(&(0x1000, "_DT_INIT".to_string())),
            "DT_INIT 0x1000 should be named _DT_INIT, got {names:#x?}"
        );
        assert!(
            names.contains(&(0x1464, "_DT_FINI".to_string())),
            "DT_FINI 0x1464 should be named _DT_FINI, got {names:#x?}"
        );
        // Array element 0 → `_INIT_0` / `_FINI_0` (baseName + i).
        assert!(
            names.contains(&(0x1240, "_INIT_0".to_string())),
            "INIT_ARRAY[0] 0x1240 should be named _INIT_0, got {names:#x?}"
        );
        assert!(
            names.contains(&(0x1200, "_FINI_0".to_string())),
            "FINI_ARRAY[0] 0x1200 should be named _FINI_0, got {names:#x?}"
        );
    }

    // The names overlay is filtered to entries that actually survive collection,
    // and pairs every name with a discovered VMA (the headline e2e fact).
    #[test]
    fn collect_entry_names_matches_collected_entries() {
        let bytes = fixture("stripped_dynamic_x86_64");
        let file = object::File::parse(bytes.as_slice()).expect("parse stripped_dynamic");
        let entries = collect_entries(&file, bytes.as_slice());
        let names = collect_entry_names(&file, &entries);
        // The array-element starts survive into the entry set and carry their names.
        assert!(names.contains(&(0x1240, "_INIT_0".to_string())), "_INIT_0 missing");
        assert!(names.contains(&(0x1200, "_FINI_0".to_string())), "_FINI_0 missing");
        // Every named VMA is a genuinely-discovered entry (overlay never invents).
        for (addr, _name) in &names {
            assert!(entries.contains(addr), "named entry {addr:#x} not in discovered set");
        }
    }

    // The address-only projection is byte-identical to the named oracle's addrs
    // (the byte-identical-discovery contract: naming is purely additive).
    #[test]
    fn dynamic_entry_points_is_named_projection() {
        let bytes = fixture("stripped_dynamic_x86_64");
        let file = object::File::parse(bytes.as_slice()).expect("parse stripped_dynamic");
        let addrs = dynamic_entry_points(&file);
        let named: Vec<u64> = dynamic_entry_points_named(&file).into_iter().map(|(a, _)| a).collect();
        assert_eq!(addrs, named, "dynamic_entry_points must equal the named oracle's addresses");
    }

    // -- Oracle 4: _start -> main idiom (stripped_dynamic) --------------------

    #[test]
    fn libc_start_main_idiom_stripped() {
        let bytes = fixture("stripped_dynamic_x86_64");
        let file = object::File::parse(bytes.as_slice()).expect("parse stripped_dynamic");
        let main = libc_start_main_target(&file, file.entry());
        // lea rdi at 0x1178, disp 0x286 → 0x117f + 0x286 = 0x1405 = main.
        assert_eq!(main, Some(0x1405), "libc-start idiom should recover main at 0x1405");
    }

    // -- Oracle 4 cross-arch: _start -> main idiom (Increment 23) --------------
    //
    // Each fixture is a tiny `int main(int,char**){return argc;}` built DYNAMIC
    // (real crt1 `_start` → `__libc_start_main(main,…)`), `-fno-*-unwind-tables`,
    // `-fvisibility=hidden` (so `main` is NOT in `.dynsym`), and stripped — so
    // `main` is discoverable ONLY via the `_start`→`main` idiom, never via a
    // symbol. The load-bearing VMAs are pinned (read via the container objdump/
    // readelf at build time; see the fixtures README provenance).

    /// AArch64: `_start` (0x600) loads `main` (0x714) into `x0` via
    /// `adrp x0,0x10000 ; ldr x0,[x0,#4080]` → GOT slot 0x10ff0, whose
    /// `R_AARCH64_RELATIVE` addend is 0x714.
    #[test]
    fn libc_start_main_idiom_aarch64() {
        let bytes = fixture("entrymain_aarch64");
        let file = object::File::parse(bytes.as_slice()).expect("parse entrymain_aarch64");
        assert_eq!(file.architecture(), object::Architecture::Aarch64);
        assert_eq!(file.entry(), 0x600, "aarch64 _start (e_entry)");
        // The GOT slot 0x10ff0 resolves to main (0x714) via the RELATIVE map.
        let rel = relative_targets(&file);
        assert_eq!(rel.get(&0x10ff0).copied(), Some(0x714), "RELATIVE @0x10ff0 → main");
        let main = libc_start_main_target(&file, file.entry());
        assert_eq!(main, Some(0x714), "aarch64 idiom should recover main at 0x714");
    }

    /// ARM/Thumb: `_start` (0x3dd, Thumb @0x3dc) loads `main` (0x4d8) into `r0`
    /// GOT-relatively (`.got`@0x10fd0 + 0x28 = slot 0x10ff8, whose `R_ARM_RELATIVE`
    /// in-place value is 0x4d9 — main with the Thumb LSB set, masked to 0x4d8).
    #[test]
    fn libc_start_main_idiom_arm() {
        let bytes = fixture("entrymain_arm");
        let file = object::File::parse(bytes.as_slice()).expect("parse entrymain_arm");
        assert_eq!(file.architecture(), object::Architecture::Arm);
        assert_eq!(file.entry(), 0x3dd, "arm _start (Thumb, LSB set)");
        // The RELATIVE map carries the GOT slot 0x10ff8 → 0x4d9 (Thumb LSB).
        let rel = relative_targets(&file);
        assert_eq!(rel.get(&0x10ff8).copied(), Some(0x4d9), "RELATIVE @0x10ff8 → main|1");
        let main = libc_start_main_target(&file, file.entry());
        assert_eq!(main, Some(0x4d8), "arm idiom should recover main at 0x4d8 (LSB masked)");
    }

    /// RISC-V: `_start` (0x550) loads `main` (0x608) into `a0` via
    /// `auipc a0,0x2 ; ld a0,-1318(a0)` → GOT slot 0x2030, whose
    /// `R_RISCV_RELATIVE` addend is 0x608.
    #[test]
    fn libc_start_main_idiom_riscv() {
        let bytes = fixture("entrymain_riscv64");
        let file = object::File::parse(bytes.as_slice()).expect("parse entrymain_riscv64");
        assert_eq!(file.architecture(), object::Architecture::Riscv64);
        assert_eq!(file.entry(), 0x550, "riscv _start (e_entry)");
        let rel = relative_targets(&file);
        assert_eq!(rel.get(&0x2030).copied(), Some(0x608), "RELATIVE @0x2030 → main");
        let main = libc_start_main_target(&file, file.entry());
        assert_eq!(main, Some(0x608), "riscv idiom should recover main at 0x608");
    }

    /// The fused core proves oracle 4 SPECIFICALLY contributes `main` on each
    /// arch: `main`'s VMA is in `collect_entries`, is in an executable section,
    /// and is NOT covered by any pre-existing funcsym (the discovery property).
    #[test]
    fn collect_entries_crossarch_includes_main() {
        // `want_entry` is the raw `e_entry` oracle-1 emits — ARM's is Thumb-odd
        // (0x3dd); the `main` recovery still uses the even bytes.
        for (name, want_entry, want_main) in [
            ("entrymain_aarch64", 0x600u64, 0x714u64),
            ("entrymain_arm", 0x3dd, 0x4d8),
            ("entrymain_riscv64", 0x550, 0x608),
        ] {
            let bytes = fixture(name);
            let file = object::File::parse(bytes.as_slice()).expect("parse fixture");
            let entries = collect_entries(&file, bytes.as_slice());
            let funcsyms = existing_function_addrs(&file, bytes.as_slice());
            // main is genuinely never named in this stripped, hidden-visibility build.
            assert!(
                funcsyms.binary_search(&want_main).is_err(),
                "{name}: main {want_main:#x} unexpectedly already a funcsym"
            );
            assert!(
                entries.contains(&want_main),
                "{name}: oracle 4 should contribute main {want_main:#x}, got {entries:#x?}"
            );
            // The ELF entry (_start) is also discovered (oracle 1), at the even VMA.
            assert!(
                entries.contains(&want_entry),
                "{name}: _start {want_entry:#x} should be discovered, got {entries:#x?}"
            );
            let execs = executable_sections(&file);
            assert!(
                in_executable_section(&execs, want_main),
                "{name}: main {want_main:#x} must be inside an exec section"
            );
        }
    }

    // -- The fused core: collect_entries (stripped_dynamic, the headline) -----

    #[test]
    fn collect_entries_stripped_includes_entry_and_main() {
        let bytes = fixture("stripped_dynamic_x86_64");
        let file = object::File::parse(bytes.as_slice()).expect("parse stripped_dynamic");
        let entries = collect_entries(&file, bytes.as_slice());
        assert!(entries.contains(&0x1160), "e_entry (_start) 0x1160 missing");
        assert!(entries.contains(&0x1000), "DT_INIT 0x1000 missing");
        assert!(entries.contains(&0x1464), "DT_FINI 0x1464 missing");
        assert!(entries.contains(&0x1405), "main 0x1405 missing");
        // Every emitted entry is inside an executable section and non-zero.
        let execs = executable_sections(&file);
        for &e in &entries {
            assert!(e != 0, "no zero entry");
            assert!(in_executable_section(&execs, e), "entry {e:#x} outside exec section");
        }
    }

    // -- Dedup vs funcsyms: fauxware (symboled) -------------------------------

    #[test]
    fn collect_entries_fauxware_skips_named_functions() {
        let bytes = fixture("fauxware");
        let file = object::File::parse(bytes.as_slice()).expect("parse fauxware");
        let entries = collect_entries(&file, bytes.as_slice());
        let named = existing_function_addrs(&file, bytes.as_slice());
        // No emitted entry coincides with an already-named function.
        for &e in &entries {
            assert!(named.binary_search(&e).is_err(), "entry {e:#x} duplicates a funcsym");
        }
        // An FDE-derived start that is NOT a funcsym (e.g. _start 0x400500) is
        // recovered; main/authenticate (funcsyms) are correctly skipped here.
        assert!(entries.contains(&0x400500), "_start 0x400500 should be discovered");
    }

    // -- The fused core, PE (the PR-12 headline): a stripped PE finds functions ---

    /// A stripped PE32+ (`pe_imports_stripped.exe`, no symbols/exports) recovers
    /// its functions through the format-dispatched PE oracles: the entry and the
    /// `.pdata` RUNTIME_FUNCTION begins. `main` (0x140001592) and the entry
    /// (0x1400014f0) survive into the discovered set, both inside an exec section
    /// and non-zero, even though the binary has no `.symtab`.
    #[test]
    fn collect_entries_pe_stripped_finds_functions() {
        let bytes = fixture("pe_imports_stripped.exe");
        let file = object::File::parse(bytes.as_slice()).expect("parse stripped PE");
        let entries = collect_entries(&file, bytes.as_slice());
        assert!(entries.contains(&0x1400014f0), "entry 0x1400014f0 missing from {entries:#x?}");
        assert!(entries.contains(&0x140001592), "main 0x140001592 missing (.pdata)");
        // A bare load (no oracles) finds nothing in a stripped PE; the oracles
        // recover dozens.
        assert!(entries.len() >= 50, ".pdata should recover many functions, got {}", entries.len());
        let execs = executable_sections(&file);
        for &e in &entries {
            assert!(e != 0, "no zero entry");
            assert!(in_executable_section(&execs, e), "entry {e:#x} outside exec section");
        }
    }

    // -- The fused core, Mach-O (the PR-13 headline) ---------------------------

    /// A linked Mach-O recovers `_compute`/`_main` via `LC_FUNCTION_STARTS` +
    /// `LC_MAIN` — the source that survives stripping. On `macho_imports` the two
    /// are also symboled, so `collect_entries` skips them as funcsyms (correct:
    /// the discovery is additive); we assert the *function-starts oracle itself*
    /// found them (via the candidate set before the funcsym-skip), proving a
    /// stripped Mach-O would discover them.
    #[test]
    fn collect_entries_macho_function_starts_oracle() {
        for (name, compute, main) in [
            ("macho_imports", 0x1000005a0u64, 0x1000005b0u64),
            ("macho_imports_arm64", 0x100000560, 0x10000056c),
        ] {
            let bytes = fixture(name);
            let file = object::File::parse(bytes.as_slice()).expect("parse macho fixture");
            // The raw candidate set (the oracle output before the funcsym-skip)
            // carries both starts — the stripped-survivable discovery fact.
            let cands = macho_entry::macho_entry_candidates(&file, bytes.as_slice());
            assert!(cands.contains(&compute), "{name}: _compute {compute:#x} not in function-starts");
            assert!(cands.contains(&main), "{name}: _main {main:#x} not in function-starts");
            // Every emitted entry (post-filter) lands in an exec section, non-zero.
            let entries = collect_entries(&file, bytes.as_slice());
            let execs = executable_sections(&file);
            for &e in &entries {
                assert!(e != 0, "{name}: no zero entry");
                assert!(in_executable_section(&execs, e), "{name}: entry {e:#x} outside exec section");
            }
        }
    }

    // -- The matcher core ------------------------------------------------------

    #[test]
    fn ditted_matcher_basics() {
        // "11111111 ........ 01010101" matches 0xff ?? 0x55.
        let seq = DittedSeq::from_binary("11111111 ........ 01010101");
        assert_eq!(seq.len(), 3);
        assert!(seq.matches(&[0xff, 0x00, 0x55]));
        assert!(seq.matches(&[0xff, 0xab, 0x55]));
        assert!(!seq.matches(&[0xfe, 0x00, 0x55]));
        assert!(!seq.matches(&[0xff, 0x00, 0x54]));
        assert!(!seq.matches(&[0xff, 0x00])); // too short
    }

    #[test]
    fn prologue_pattern_matches_endbr64_frame() {
        // f3 0f 1e fa 55 48 89 e5 at vma 0x1000 (aligned) → a hit.
        let data = vec![0xf3, 0x0f, 0x1e, 0xfa, 0x55, 0x48, 0x89, 0xe5, 0x00, 0x00];
        let execs = vec![(0x1000u64, 0x1000 + data.len() as u64, data)];
        let hits = prologue_pattern_starts(&execs);
        assert!(hits.contains(&0x1000), "endbr64-frame prologue should match at 0x1000");
    }

    // -- LEB128 readers --------------------------------------------------------

    #[test]
    fn leb128_roundtrip() {
        // ULEB 0x80 0x01 = 128.
        assert_eq!(read_uleb128(&[0x80, 0x01], 0), Some((128, 2)));
        assert_eq!(read_uleb128(&[0x7f], 0), Some((127, 1)));
        // SLEB 0x7f = -1.
        assert_eq!(read_sleb128(&[0x7f], 0), Some((-1, 1)));
        assert_eq!(read_sleb128(&[0x01], 0), Some((1, 1)));
    }
}
