//! Address-table discovery — the kuna analog of Ghidra's `AddressTableAnalyzer`
//! ("Create Address Tables").
//!
//! ## What this is (and the three things it is NOT)
//!
//! This pass is a **byte-level data-discovery scan**: it walks read-only/data
//! sections and, wherever it finds a run of ≥ N consecutive pointer-width values
//! that each look like a valid in-memory address landing inside an *executable*
//! section, it lays down an absolute address/jump table (a "function-pointer
//! array": vtables, relocated function-pointer arrays, GCC absolute switch
//! tables). It emits the table head + body as **data facts** only — never
//! function entries and never switch case-ranges.
//!
//! Three distinct concerns the literature conflates; this pass is only the first:
//!
//! 1. **This pass — `AddressTableAnalyzer` (application-layer pointer-table
//!    discovery).** A GAP, but a small, low-payoff, **off-by-default** one (Ghidra
//!    ships it `setDefaultEnablement(false)`). Its output is data symbols + a
//!    read-only range over the table body, NOT prototypes and NOT switch ranges.
//!
//! 2. **switch / jump-table recovery (`JumpTable::recoverAddresses`)** — INHERITED
//!    🟡. The SSA/data-flow-driven, in-function switch recovery in the engine
//!    (`decompiler/cpp/jumptable.{cc,hh}` → kuna `s2_lift/jumptable.rs`, driven
//!    from `s2_lift/flow.rs::recover_jump_tables`). It is the analog of Ghidra's
//!    `DecompilerSwitchAnalyzer`. **This pass does NOT do that** and must not be
//!    confused with it — the switch table the `switchtab_x86_64` fixture also
//!    carries is recovered by that inherited engine machinery, independently.
//!
//! 3. **roadmap #9 "re-run jump-table recovery after type recovery"** — the
//!    decompiler-internal MULTISTAGE re-recovery (`recover_count > 1`, gated behind
//!    the `Override::queryMultistageJumptable` SEAM in jumptable.rs). It is
//!    ENGINE-RESIDENT (S2), **not** an analyzer-tier item. This pass does NOT
//!    attempt it; #9 is a separate, deferred *engine* task (see
//!    `docs/missing-analyses.md` §7 and `docs/analysis-port-log.md`).
//!
//! ## Origin (upstream Ghidra, the tree kuna was ported from)
//!
//! - Analyzer wrapper:
//!   `Ghidra/Features/Base/src/main/java/ghidra/app/plugin/core/disassembler/AddressTableAnalyzer.java`
//!   (`added()`, `checkTable()`, `removeNonSearchableMemory()`, `getDefaultEnablement()`;
//!   note `setDefaultEnablement(false)`).
//! - Core scanner:
//!   `Ghidra/Features/Base/.../disassembler/AddressTable.java` —
//!   `getEntry()` (the pointer-run scanner: read addr-sized value, reject
//!   0 / `< minAddressOffset` / unaligned / out-of-memory / fails the relocation
//!   guard, stop on any ref-in-middle; min-run length check;
//!   `isValidRelocationAddress()`), `getThresholdRunOfValidPointers()` (the
//!   probability-based min-run).
//!
//! ## DISABLED by default
//!
//! This pass is **registered commented-out** in
//! [`crate::passes::default_passes`], exactly like [`crate::s1_strings`], for two
//! reasons: (a) Ghidra parity (off by default upstream); (b) real false-positive
//! risk — a pointer-run scanner over `.rodata`/`.data` over-accepts, since any
//! coincidental run of in-range aligned values reads as a "table". The relocation
//! guard is the main defense and is strong on PIE/relocatable objects (every real
//! pointer is relocated) but *weak on non-PIE* binaries (absolute `.text` pointers
//! carry no relocation), so the non-PIE `switchtab_x86_64` fixture exercises the
//! heuristic path (min-run + executable-target check + string guard), not the
//! relocation-guided path. The scanner is exercised directly by the unit tests
//! below; it is inert in the bootstrap until a future increment enables it.
//!
//! ## Scope / LOSS vs Ghidra (documented divergence)
//!
//! - No listing/Bookmark/`makeTable`/`autoLabel` side effects: Ghidra mutates the
//!   Program listing; kuna has no pre-decompile listing (the same tier limitation
//!   documented for the `FindNoReturnFunctionsAnalyzer` "Discovered" analyzer in
//!   [`crate::s1_loader::noreturn`]). kuna emits *facts only*.
//! - No index-table (secondary byte-index) recovery (`AddressTable.getEntry`'s
//!   `checkForIndex` branch) and no shifted-address tables — deferred; rare on
//!   x86-64 ELF.
//! - The probability-based `getThresholdRunOfValidPointers` is reduced to its
//!   clamped floor (`MIN_RUN_FLOOR`); the spirit (a longer run is more credible)
//!   is preserved by the configurable `min_run` knob.

use object::read::{Object, ObjectSection};
use object::SectionKind;

use crate::pass::{AnalysisCtx, AnalysisOutput, AnalysisPass, Stage, SymFact, SymKind};

// --- Upstream constants (AddressTable.java / AddressTableAnalyzer.java) --------

/// `AddressTable.MINIMUM_SAFE_ADDRESS` — addresses below this look like small
/// integers, not pointers, and are rejected.
const MINIMUM_SAFE_ADDRESS: u64 = 1024;

/// `AddressTableAnalyzer.OPTION_DEFAULT_MIN_POINTER_ADDR` — the default minimum
/// value a slot may hold to be considered an address (`minAddressOffset`).
const DEFAULT_MIN_POINTER_ADDR: u64 = 0x1024;

/// `AddressTable` floor for the minimum run length. The faithful
/// `getThresholdRunOfValidPointers` computes a probability over total mapped
/// bytes vs `2^ptrbits` and clamps `>= 2`; kuna uses the clamped floor and lets
/// the caller raise `min_run`.
const MIN_RUN_FLOOR: usize = 2;

/// ELF section-header flag: `SHF_EXECINSTR` (section holds executable machine
/// instructions) — a table element must point into such a section.
const SHF_EXECINSTR: u64 = 0x4;
/// ELF section-header flag: `SHF_ALLOC` (section occupies memory at runtime).
const SHF_ALLOC: u64 = 0x2;

/// A discovered absolute address table: a run of pointer-width values starting at
/// `addr`, each of which (`elements[i]`) lands inside an executable section.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AddrTable {
    /// Virtual address of the first table slot.
    pub addr: u64,
    /// Number of slots in the table.
    pub len: usize,
    /// The decoded element targets, in order (each is an in-`.text` address).
    pub elements: Vec<u64>,
}

/// `[start, end)` half-open executable-section range.
type ExecRange = (u64, u64);

/// Is `sec` an executable (code) section — the set a table element must point
/// into ("target must be in code memory", `AddressTable.getEntry`)? Per-format
/// (PR-10): ELF `SHF_EXECINSTR` (unchanged), PE/COFF `IMAGE_SCN_MEM_EXECUTE`,
/// Mach-O `S_ATTR_PURE_INSTRUCTIONS` / the neutral `SectionKind::Text`.
fn is_executable_section<'a>(sec: &impl object::read::ObjectSection<'a>) -> bool {
    match sec.flags() {
        object::SectionFlags::Elf { sh_flags } => sh_flags & SHF_EXECINSTR != 0,
        object::SectionFlags::Coff { characteristics } => {
            characteristics & object::pe::IMAGE_SCN_MEM_EXECUTE != 0
        }
        object::SectionFlags::MachO { flags } => {
            flags & object::macho::S_ATTR_PURE_INSTRUCTIONS != 0
                || matches!(sec.kind(), SectionKind::Text)
        }
        _ => matches!(sec.kind(), SectionKind::Text),
    }
}

/// Collect the `[address, address+size)` ranges of every executable section — the
/// set a table element must point into. Mirrors the "target must be in code
/// memory" check of `AddressTable.getEntry`.
fn exec_ranges(file: &object::File) -> Vec<ExecRange> {
    let mut out = Vec::new();
    for sec in file.sections() {
        if !is_executable_section(&sec) {
            continue;
        }
        let a = sec.address();
        let sz = sec.size();
        if sz == 0 {
            continue;
        }
        out.push((a, a.saturating_add(sz)));
    }
    out
}

/// `true` if `addr` lands in any executable section of `file` — the public form
/// of [`exec_ranges`] for the tests' per-element assertion.
pub fn in_exec_section(file: &object::File, addr: u64) -> bool {
    exec_ranges(file).iter().any(|&(s, e)| addr >= s && addr < e)
}

/// Build the sorted list of relocated slot addresses (`isValidRelocationAddress`
/// guide). On a relocatable / PIE object every real absolute pointer slot carries
/// a dynamic relocation; on a non-PIE `EXEC` there are typically none, so this is
/// empty and the scan falls through to the pure heuristic path.
///
/// Mirrors `elf_plt::build_got_name_map`'s use of
/// [`object::read::Object::dynamic_relocations`]; we only need the slot *offsets*
/// here, not the target names. No `HashMap` (per the engine convention): a sorted
/// `Vec` + binary search.
fn relocated_slots(file: &object::File) -> Vec<u64> {
    let mut slots = Vec::new();
    if let Some(relocs) = file.dynamic_relocations() {
        // Any dynamic relocation (symbol, absolute, or base-RELATIVE) pins a real
        // pointer into its slot; the guard only needs the slot address (the first
        // tuple element), not the relocation's target.
        for (off, _reloc) in relocs {
            slots.push(off);
        }
    }
    slots.sort_unstable();
    slots.dedup();
    slots
}

/// Read one little/big-endian pointer-width value from `data` at `off`, returning
/// `None` if the slice is too short. `ptr_size` is 8 or 4.
fn read_ptr(data: &[u8], off: usize, ptr_size: usize, little: bool) -> Option<u64> {
    let end = off.checked_add(ptr_size)?;
    let bytes = data.get(off..end)?;
    let mut v: u64 = 0;
    if little {
        for (i, &b) in bytes.iter().enumerate() {
            v |= (b as u64) << (8 * i);
        }
    } else {
        for &b in bytes {
            v = (v << 8) | b as u64;
        }
    }
    Some(v)
}

/// `getLoadedAndInitializedAddressSet` ∩ `removeNonSearchableMemory`: the
/// sections worth scanning for a pointer table — allocated + initialized
/// read-only/data (`.rodata`, `.data`, `.data.rel.ro`, `.got` analogues). `.bss`
/// (uninitialized) and non-allocated metadata (`.symtab`, `.comment`) are skipped.
fn is_searchable_section<'a>(sec: &impl object::read::ObjectSection<'a>) -> bool {
    if matches!(sec.kind(), SectionKind::UninitializedData) {
        return false;
    }
    // Executable sections hold code, not data tables — the disqualifier on every
    // format.
    if is_executable_section(sec) {
        return false;
    }
    // Must be in the runtime image. Read-only OR writable data both qualify
    // (vtables in .data.rel.ro, GOT, .rodata switch tables). Per-format (PR-10):
    // ELF SHF_ALLOC (unchanged); a PE/COFF section is mapped iff readable+
    // initialized; a Mach-O section is mapped unless it is zero-fill (already
    // excluded by the `SectionKind` guard above).
    match sec.flags() {
        object::SectionFlags::Elf { sh_flags } => sh_flags & SHF_ALLOC != 0,
        object::SectionFlags::Coff { characteristics } => {
            characteristics & object::pe::IMAGE_SCN_CNT_UNINITIALIZED_DATA == 0
                && characteristics & object::pe::IMAGE_SCN_MEM_READ != 0
        }
        object::SectionFlags::MachO { .. } => true,
        _ => false,
    }
}

/// `checkTable` string guard (AddressTable.java): a run that is actually a
/// NUL-terminated printable string is not a pointer table. We reuse the
/// `s1_strings` charset recognizer: if every byte of the candidate slot region is
/// a printable string char, treat the region as text and reject the run.  Kept
/// deliberately minimal (the spirit of the guard, not the full n-gram model).
fn looks_like_string(data: &[u8], off: usize, span: usize) -> bool {
    let end = match off.checked_add(span) {
        Some(e) if e <= data.len() => e,
        _ => return false,
    };
    span > 0 && data[off..end].iter().all(|&b| is_printable_string_byte(b))
}

/// Mirror of `s1_strings::is_string_char` (`AsciiCharSetRecognizer.contains`):
/// printable ASCII + CR/LF/TAB. Duplicated (rather than re-exported) to keep this
/// disabled pass self-contained.
fn is_printable_string_byte(b: u8) -> bool {
    (0x20..=0x7e).contains(&b) || b == 0x0d || b == 0x0a || b == 0x09
}

/// Faithful port of the `AddressTable.getEntry` per-entry loop over one section's
/// bytes, returning every accepted absolute table in the section.
///
/// For each aligned start offset, accumulate consecutive slots while each decoded
/// value is a valid in-`.text` pointer; close the run on the first invalid slot.
/// A run of `>= min_run` slots becomes an [`AddrTable`]. The relocation guard
/// (`isValidRelocationAddress`) is applied **only when `use_reloc_guard` is set** —
/// i.e. for a relocatable / PIE image (`ET_REL`/`ET_DYN`), where *every* real
/// absolute pointer is relocated, so an un-relocated slot is provably not a
/// pointer. On a non-PIE `ET_EXEC` the guard is OFF (absolute `.text` pointers
/// carry no relocation), and only the heuristic checks (in-exec, min-addr,
/// string-guard, min-run) apply — the over-acceptance-prone path that is exactly
/// why this pass ships disabled. See the module docs.
fn scan_section(
    sec_vma: u64,
    data: &[u8],
    ptr_size: usize,
    little: bool,
    min_addr: u64,
    min_run: usize,
    exec: &[ExecRange],
    relocs: &[u64],
    use_reloc_guard: bool,
) -> Vec<AddrTable> {
    let mut tables = Vec::new();
    let in_exec = |a: u64| exec.iter().any(|&(s, e)| a >= s && a < e);
    let is_reloc_slot = |slot_vma: u64| relocs.binary_search(&slot_vma).is_ok();

    let n = data.len();
    let mut off = 0usize;
    while off + ptr_size <= n {
        // The table slot's own virtual address (for the relocation guard).
        let slot_vma = sec_vma + off as u64;
        let mut elements: Vec<u64> = Vec::new();
        let mut cur = off;
        while cur + ptr_size <= n {
            let slot_vma_cur = sec_vma + cur as u64;
            let Some(v) = read_ptr(data, cur, ptr_size, little) else { break };
            // getEntry rejects: 0, small ints (< minAddressOffset / MINIMUM_SAFE_ADDRESS),
            // and targets not inside executable memory.
            if v == 0 || v < min_addr || v < MINIMUM_SAFE_ADDRESS || !in_exec(v) {
                break;
            }
            // Relocation guard (isValidRelocationAddress): on a relocatable image
            // a real pointer slot must be relocated.
            if use_reloc_guard && !is_reloc_slot(slot_vma_cur) {
                break;
            }
            // String guard: a printable, would-be-string region is not a table.
            if looks_like_string(data, cur, ptr_size) {
                break;
            }
            elements.push(v);
            cur += ptr_size;
        }
        if elements.len() >= min_run {
            tables.push(AddrTable { addr: slot_vma, len: elements.len(), elements });
            // Advance past the accepted table (non-overlapping, like getEntry).
            off = cur;
        } else {
            off += ptr_size;
        }
    }
    tables
}

/// Scan every searchable section of `file` for absolute address tables: runs of
/// `>= min_run` consecutive `ptr_size`-byte values that each point inside an
/// executable section. The pure core of the pass; shared with the unit tests.
///
/// `min_run` is clamped up to [`MIN_RUN_FLOOR`] (a 1-element "run" is never a
/// table). `ptr_size` is 8 (ELFCLASS64) or 4 (ELFCLASS32); endianness from the
/// object.
pub fn scan_address_tables(file: &object::File, min_run: usize, ptr_size: usize) -> Vec<AddrTable> {
    let min_run = min_run.max(MIN_RUN_FLOOR);
    let little = file.is_little_endian();
    let min_addr = DEFAULT_MIN_POINTER_ADDR;
    let exec = exec_ranges(file);
    if exec.is_empty() {
        return Vec::new();
    }
    let relocs = relocated_slots(file);
    // The relocation guard is only sound on a relocatable / PIE image, where every
    // real absolute pointer carries a relocation. On a non-PIE ET_EXEC the few
    // dynamic relocations (GOT slots for `__libc_start_main` etc.) do NOT cover the
    // program's absolute switch-table pointers, so applying the guard there would
    // wrongly reject every real table (this is the `switchtab_x86_64` case). Mirror
    // `AddressTable.isValidRelocationAddress`, which is only consulted for objects
    // whose memory is relocatable.
    let use_reloc_guard =
        matches!(file.kind(), object::ObjectKind::Relocatable | object::ObjectKind::Dynamic)
            && !relocs.is_empty();

    let mut out = Vec::new();
    for sec in file.sections() {
        if !is_searchable_section(&sec) {
            continue;
        }
        let data = match sec.data() {
            Ok(d) if !d.is_empty() => d,
            _ => continue,
        };
        out.extend(scan_section(
            sec.address(),
            data,
            ptr_size,
            little,
            min_addr,
            min_run,
            &exec,
            &relocs,
            use_reloc_guard,
        ));
    }
    out
}

/// Choose the pointer width for `file`: 8 for ELFCLASS64, else 4. Faithful to
/// `AddressTable.getByteLength` reading the program's default pointer size.
fn ptr_width(file: &object::File) -> usize {
    if file.is_64() {
        8
    } else {
        4
    }
}

/// Turn the discovered tables into [`AnalysisOutput`] facts: a **Data** symbol at
/// each table head + a read-only range over the body. **Never** an `entries` /
/// Function fact — faithful to `AddressTableAnalyzer` (which labels and types the
/// table but, per its commented-out code, never makes functions from it; real
/// code targets are found by `s1-entry-disc` and the engine's switch recovery).
/// Pure (no engine/context dependency), so the unit tests can assert it directly.
fn emit_facts(tables: &[AddrTable], ptr_size: usize) -> AnalysisOutput {
    let mut out = AnalysisOutput::default();
    for t in tables {
        out.symbols.push(SymFact {
            addr: t.addr,
            name: format!("AddrTable{:08x}", t.addr),
            kind: SymKind::Data,
        });
        let body_end = t.addr + (t.len as u64) * (ptr_size as u64);
        out.readonly.push((t.addr, body_end));
    }
    out
}

/// Port of `AddressTableAnalyzer` ("Create Address Tables"): discover absolute
/// address/jump tables and emit their head + body as **data** facts.
///
/// **Disabled by default** (see the module docs): registered commented-out in
/// [`crate::passes::default_passes`].
pub struct AddrTablePass {
    /// Minimum run length to accept a table (clamped up to [`MIN_RUN_FLOOR`]).
    pub min_run: usize,
}

impl Default for AddrTablePass {
    fn default() -> Self {
        AddrTablePass { min_run: MIN_RUN_FLOOR }
    }
}

impl AnalysisPass for AddrTablePass {
    fn stage(&self) -> Stage {
        Stage::S1
    }

    fn id(&self) -> &'static str {
        "addrtable"
    }

    fn run(&self, ctx: &AnalysisCtx) -> AnalysisOutput {
        // Format-agnostic (PR-10): the scan keys off the neutral pointer-run +
        // executable-target heuristic (per-format exec/searchable section arms),
        // so it works on ELF/PE/COFF/Mach-O. (Still off by default — registered
        // commented-out, matching Ghidra's `setDefaultEnablement(false)`.) The
        // relocation guard already keys on the neutral `file.kind()`.
        let ptr_size = ptr_width(ctx.file);
        let tables = scan_address_tables(ctx.file, self.min_run, ptr_size);
        emit_facts(&tables, ptr_size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The `switchtab_x86_64` fixture's absolute jump table, read off
    /// `readelf -x .rodata` / `objdump -d` at fixture-build time and pinned here
    /// (addresses shift across toolchains — never re-derive). The 8-entry table
    /// sits at vma `0x402008` (the `jmp *0x402008(,%rdi,8)` target); the 8-byte
    /// prefix at `0x402000` is the switch's bounds word, not part of the table.
    const EXPECTED_TABLE_VMA: u64 = 0x402008;
    /// The 8 case targets, all inside `.text` (`0x401020..0x401159`).
    const EXPECTED_ELEMENTS: [u64; 8] = [
        0x401149, 0x401119, 0x40111f, 0x401125, 0x40112b, 0x401131, 0x401137, 0x40113d,
    ];

    fn load(name: &str) -> Vec<u8> {
        let path = format!("{}/tests/fixtures/{}", env!("CARGO_MANIFEST_DIR"), name);
        std::fs::read(&path).unwrap_or_else(|e| panic!("read fixture {path}: {e}"))
    }

    #[test]
    fn finds_the_8_entry_table_in_switchtab() {
        // The headline: the scanner FINDS the 8-entry absolute table at 0x402008
        // when invoked directly (NOT via default_passes — the pass is disabled).
        let bytes = load("switchtab_x86_64");
        let file = object::File::parse(bytes.as_slice()).expect("parse switchtab");
        assert!(file.is_64(), "fixture is ELFCLASS64");

        let tables = scan_address_tables(&file, /*min_run=*/ 2, /*ptr_size=*/ 8);

        let hit = tables
            .iter()
            .find(|t| t.addr == EXPECTED_TABLE_VMA)
            .unwrap_or_else(|| panic!("no table at {EXPECTED_TABLE_VMA:#x}; found: {tables:?}"));
        assert!(hit.len >= 8, "table at {EXPECTED_TABLE_VMA:#x} has {} slots, want >= 8", hit.len);
        // The first 8 decoded targets are the known case bodies, in order.
        assert_eq!(&hit.elements[..8], &EXPECTED_ELEMENTS, "table elements differ");
    }

    #[test]
    fn every_element_points_inside_an_executable_section() {
        let bytes = load("switchtab_x86_64");
        let file = object::File::parse(bytes.as_slice()).expect("parse switchtab");
        let tables = scan_address_tables(&file, 2, 8);
        assert!(!tables.is_empty(), "expected at least one table");
        for t in &tables {
            for &e in &t.elements {
                assert!(in_exec_section(&file, e), "element {e:#x} not in an exec section");
            }
        }
    }

    #[test]
    fn pass_emits_a_data_symbol_and_readonly_range_for_the_table() {
        // Exercise the fact-emission half directly over the scanned tables (the
        // pure `emit_facts`, the same path `AddrTablePass::run` takes — building a
        // full Architecture for an AnalysisCtx needs the SLEIGH specs, so the
        // unit tests assert the pure cores, as every other s1_* pass test does):
        // it must emit a Data symbol at the table head + a readonly range over the
        // body, and NEVER a Function entry from a table (faithful to the analyzer).
        let bytes = load("switchtab_x86_64");
        let file = object::File::parse(bytes.as_slice()).expect("parse switchtab");
        let tables = scan_address_tables(&file, 2, 8);
        let out = emit_facts(&tables, 8);

        assert!(
            out.symbols.iter().any(|s| s.addr == EXPECTED_TABLE_VMA && s.kind == SymKind::Data),
            "expected a Data symbol at {EXPECTED_TABLE_VMA:#x}; got {:?}",
            out.symbols
        );
        assert!(
            out.symbols.iter().all(|s| s.kind == SymKind::Data),
            "address-table pass must emit only Data symbols, never Function"
        );
        assert!(out.entries.is_empty(), "address-table pass must NOT emit code entries");
        assert!(
            out.readonly.iter().any(|&(a, e)| a == EXPECTED_TABLE_VMA && e > a),
            "expected a readonly range starting at the table head"
        );
    }

    #[test]
    fn fauxware_has_no_pointer_table_over_its_real_data() {
        // Negative/over-acceptance guard: fauxware (non-PIE, no dense switch) has no
        // run of consecutive in-.text pointers in .rodata/.data, so the scanner must
        // not invent a table over its real string data. The fauxware string literals
        // ("Username: " @ 0x400915, …) are NOT runs of in-.text 8-byte values, so the
        // in-exec + min-run checks reject them even with the reloc guard off (EXEC).
        let bytes = load("fauxware");
        let file = object::File::parse(bytes.as_slice()).expect("parse fauxware");
        let tables = scan_address_tables(&file, /*min_run=*/ 2, 8);
        // No table may start inside the .rodata string region the s1_strings test
        // pins (0x4008d0..0x400930), which is plain text, not pointers.
        assert!(
            !tables.iter().any(|t| (0x4008d0..0x400930).contains(&t.addr)),
            "spurious table over fauxware string data: {tables:?}"
        );
    }

    #[test]
    fn short_run_below_min_is_rejected() {
        // A run shorter than min_run must not be accepted as a table. With a huge
        // min_run, the real 8-entry table is also rejected (guards the threshold).
        let bytes = load("switchtab_x86_64");
        let file = object::File::parse(bytes.as_slice()).expect("parse switchtab");
        let tables = scan_address_tables(&file, /*min_run=*/ 64, 8);
        assert!(
            !tables.iter().any(|t| t.addr == EXPECTED_TABLE_VMA),
            "an 8-entry table must be rejected when min_run=64"
        );
    }

    #[test]
    fn scan_runs_over_pe_and_macho() {
        // PR-10: the scanner is format-agnostic — `exec_ranges` finds the PE
        // `.text` (`IMAGE_SCN_MEM_EXECUTE`) and the searchable `.rdata`/`.data`,
        // and the scan completes (no ELF early return, no panic). We do not pin a
        // specific table (the pass is off by default and these fixtures carry no
        // dense pointer table); we prove the per-format flag arms resolve.
        let pe = load("pe_imports.exe");
        let pe_file = object::File::parse(pe.as_slice()).expect("parse pe");
        assert_eq!(pe_file.format(), object::BinaryFormat::Pe);
        let pe_exec = exec_ranges(&pe_file);
        assert!(!pe_exec.is_empty(), "PE .text must be found as an exec range");
        // The scan runs to completion over the PE (the old gate returned early).
        let _ = scan_address_tables(&pe_file, 2, ptr_width(&pe_file));

        let mo = load("macho_imports");
        let mo_file = object::File::parse(mo.as_slice()).expect("parse macho");
        assert_eq!(mo_file.format(), object::BinaryFormat::MachO);
        let mo_exec = exec_ranges(&mo_file);
        assert!(!mo_exec.is_empty(), "Mach-O __text must be found as an exec range");
        let _ = scan_address_tables(&mo_file, 2, ptr_width(&mo_file));
    }

    #[test]
    fn read_ptr_endianness() {
        let le = [0x08, 0x20, 0x40, 0, 0, 0, 0, 0];
        assert_eq!(read_ptr(&le, 0, 8, true), Some(0x402008));
        let be = [0, 0, 0, 0, 0, 0x40, 0x20, 0x08];
        assert_eq!(read_ptr(&be, 0, 8, false), Some(0x402008));
        // too-short slice
        assert_eq!(read_ptr(&[0u8; 3], 0, 8, true), None);
    }

    #[test]
    fn string_guard_rejects_printable_runs() {
        // "AAAAAAAA" is 8 printable bytes — guarded as a would-be string, not a
        // table slot.
        let s = b"AAAAAAAA";
        assert!(looks_like_string(s, 0, 8));
        // a slot full of zero bytes is not printable text
        assert!(!looks_like_string(&[0u8; 8], 0, 8));
    }
}
