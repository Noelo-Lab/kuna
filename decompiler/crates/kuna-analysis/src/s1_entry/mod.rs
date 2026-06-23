//! Function-entry / function-start discovery for stripped ELFs — the kuna
//! analog of Ghidra's entry-point + function-start analyzers, fused with the
//! `.eh_frame` FDE oracle into ONE additive discovery pass.
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
//!    the loader-seeded external entry points (`ElfProgramBuilder`).
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
        // ELF-only (the oracles are all ELF/eh_frame structures). Additive
        // contract: never fail — an empty output on any anomaly.
        if !matches!(ctx.file.format(), object::BinaryFormat::Elf) {
            return out;
        }
        out.entries = collect_entries(ctx.file);
        // ARM/Thumb: a discovered `main` whose libc-start GOT pointer had the
        // Thumb LSB set needs a `TMode=1` decode-mode paint at its (even) entry —
        // this stripped binary carries no `$t` mapping symbol for `arm_markers` to
        // paint from, so without it the engine decodes the Thumb body as A32 and
        // emits a degenerate function. The exact analog of `arm_markers`'
        // STT_FUNC-LSB → `TMode=1` paint, derived here from the GOT pointer LSB.
        out.context_paints = thumb_entry_paints(ctx.file);
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
pub fn collect_entries(file: &object::File) -> Vec<u64> {
    let execs = executable_sections(file);
    let funcsyms = existing_function_addrs(file);

    let mut cand: Vec<u64> = Vec::new();

    // Oracle 1: ELF entry point (e_entry). EntryPointAnalyzer external entry.
    let entry = file.entry();
    if entry != 0 {
        cand.push(entry);
    }

    // Oracle 2: DT_INIT/DT_FINI + INIT_ARRAY/FINI_ARRAY pointer tables.
    cand.extend(dynamic_entry_points(file));

    // Oracle 3: .eh_frame FDE pcBegin addresses.
    cand.extend(scan_eh_frame_starts(file));

    // Oracle 4: _start -> main via the libc-start idiom. x86-64 uses the direct
    // `lea rdi,[rip+disp]` immediate; AArch64/ARM/RISC-V PIE crt1 instead loads
    // `main` *indirectly* from a GOT slot that carries an `R_*_RELATIVE`
    // relocation whose target is `main` (Increment 23) — `libc_start_main_target`
    // dispatches on the architecture.
    if let Some(main) = libc_start_main_target(file, entry) {
        cand.push(main);
    }
    // Oracle 5: prologue byte patterns — x86-64-only (the vendored gcc pattern
    // subset; the ARM/AARCH64/MIPS/PPC patternconstraints are a follow-up).
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

// ===========================================================================
// Section / funcsym helpers
// ===========================================================================

/// `(address, address+size, data)` for every executable section. SHF_EXECINSTR
/// or the high-level `SectionKind::Text` (`.text`/`.init`/`.fini`/`.plt`). Used
/// both as the prologue-sweep target and the "is this VMA plausible code?" oracle.
fn executable_sections(file: &object::File) -> Vec<(u64, u64, Vec<u8>)> {
    // ELF section header flag: SHF_EXECINSTR (the section holds machine code).
    const SHF_EXECINSTR: u64 = 0x4;

    let mut out = Vec::new();
    for sec in file.sections() {
        let exec = match sec.flags() {
            object::SectionFlags::Elf { sh_flags } => sh_flags & SHF_EXECINSTR != 0,
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
fn in_executable_section(execs: &[(u64, u64, Vec<u8>)], vma: u64) -> bool {
    execs.iter().any(|&(lo, hi, _)| vma >= lo && vma < hi)
}

/// Sorted VMAs of every already-named function: `.symtab`/`.dynsym` *defined*
/// FUNC symbols (UND imports have `st_value == 0`) plus PLT import stubs. The
/// commit seam's `find_function` already no-ops a covered address, but skipping
/// these here keeps the emitted set to genuinely *new* starts.
fn existing_function_addrs(file: &object::File) -> Vec<u64> {
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
    for p in crate::s1_loader::elf_plt::resolve_plt_imports(file) {
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

/// The loader-seeded external entry points from the `.dynamic` table:
/// `DT_INIT`/`DT_FINI` (one each) plus every pointer in the `DT_INIT_ARRAY` /
/// `DT_FINI_ARRAY` tables (faithful to `ElfProgramBuilder` marking these as
/// external entry points). The dynamic table is read from the `.dynamic` section
/// bytes as `Elf{32,64}_Dyn` (tag/val) pairs — robust without the typed-view
/// plumbing, and the array bytes come from whichever section contains the array
/// vma (the static-image base-0 assumption: a PIE array pointer is already the
/// file vma, no load bias).
fn dynamic_entry_points(file: &object::File) -> Vec<u64> {
    let mut out: Vec<u64> = Vec::new();

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
            DT_INIT | DT_FINI => out.push(val),
            DT_INIT_ARRAY => init_array = Some(val),
            DT_INIT_ARRAYSZ => init_array_sz = val,
            DT_FINI_ARRAY => fini_array = Some(val),
            DT_FINI_ARRAYSZ => fini_array_sz = val,
            _ => {}
        }
    }

    let ptr = if is64 { 8usize } else { 4usize };
    for (base, sz) in [(init_array, init_array_sz), (fini_array, fini_array_sz)] {
        let Some(base) = base else { continue };
        out.extend(read_pointer_table(file, base, sz, ptr, le));
    }

    out
}

/// Read `sz / ptr` pointers from the array at vma `base` by slicing the section
/// that contains `base`. Each decoded pointer is itself a function entry.
fn read_pointer_table(file: &object::File, base: u64, sz: u64, ptr: usize, le: bool) -> Vec<u64> {
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
            out.push(p);
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
            let entries = collect_entries(&file);
            let funcsyms = existing_function_addrs(&file);
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
        let entries = collect_entries(&file);
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
        let entries = collect_entries(&file);
        let named = existing_function_addrs(&file);
        // No emitted entry coincides with an already-named function.
        for &e in &entries {
            assert!(named.binary_search(&e).is_err(), "entry {e:#x} duplicates a funcsym");
        }
        // An FDE-derived start that is NOT a funcsym (e.g. _start 0x400500) is
        // recovered; main/authenticate (funcsyms) are correctly skipped here.
        assert!(entries.contains(&0x400500), "_start 0x400500 should be discovered");
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
