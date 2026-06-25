//! ELF PLT/GOT import-name resolution — the kuna analog of Ghidra's Java
//! `ElfDefaultGotPltMarkup` + `ElfProgramBuilder::createExternalFunctionLinkage`
//! (`Ghidra/Features/Base/.../elf/ElfDefaultGotPltMarkup.java`,
//! `.../opinion/ElfProgramBuilder.java`).
//!
//! ## Why this lives in kuna at all
//!
//! kuna ports Ghidra's C++ *decompiler*; the resolution of a PLT stub
//! (`0x400510`) to its imported library name (`puts`) is done by Ghidra's Java
//! *loader/analyzer* layer, which the port never carried over.  Without it a CALL
//! to a PLT stub has no symbol, so `FlowInfo::queryCall` (kuna
//! `decompile_drive.rs::query_call`) finds nothing and the printer falls back to
//! `sub_<addr>`.  This module reconstructs the stub→name mapping straight from the
//! ELF, so the loader can register the imports as ordinary `FunctionSymbol`s in
//! the same stream as the `.symtab`/`.dynsym` defined functions (the
//! "correct-names" model — the full external-location/thunk object model is a
//! deferred follow-up).
//!
//! ## Algorithm (matches `ElfDefaultGotPltMarkup`'s correlation)
//!
//! 1. **`got_slot → name`** from the dynamic relocations: each relocation's
//!    `r_offset` is a GOT slot address and its symbol index resolves (in the
//!    *dynamic* symbol table) to the imported name.  See [`build_got_name_map`].
//! 2. **`plt_stub → got_slot`** by decoding each `.plt*` section's indirect jump
//!    per architecture, then matching the decoded GOT target against the map.
//!    Matching the *decoded target* (rather than trusting PLT-entry order) is
//!    self-correcting: the resolver stub (PLT0) and `IRELATIVE`/IFUNC stubs jump
//!    to slots that are not symbol-bearing relocations, so they fall out of the
//!    map and are dropped automatically.
//!
//! Everything degrades gracefully: an unsupported architecture, a missing
//! dynamic symbol table, or an undecodable stub yields fewer (or zero) entries,
//! leaving the pre-existing behavior unchanged.  This module never panics and
//! never errors — it only ever produces *more* names.

use std::collections::{HashMap, HashSet};

use object::read::{Object, ObjectSection, ObjectSymbol};
use object::{Architecture, RelocationTarget};

/// One resolved PLT-stub import: the stub entry address (the CALL target the
/// decompiler sees) and the clean imported name.
pub(crate) struct PltSym {
    /// PLT stub entry address (`FunctionSymbol` address; what a CALL targets).
    pub addr: u64,
    /// Imported function name (raw `.dynstr` bytes, `@VERSION` stripped).
    pub name: Vec<u8>,
}

/// Resolve PLT-stub call targets to their imported function names.
///
/// Returns one `(stub_addr, name)` per decodable PLT entry whose decoded GOT
/// target matches a symbol-bearing dynamic relocation.  Empty when the binary
/// has no dynamic imports, an unsupported architecture, or stubs this decoder
/// does not recognize.
pub(crate) fn resolve_plt_imports(file: &object::File) -> Vec<PltSym> {
    let arch = file.architecture();

    // MIPS has no regular `.plt` code section and emits NO `R_MIPS_JUMP_SLOT`
    // relocations for its lazy imports: the stub→name correspondence is carried by
    // the dynamic table's GOT layout (`DT_MIPS_LOCAL_GOTNO`/`DT_MIPS_GOTSYM`), not
    // by relocations.  It therefore runs its own resolver (`resolve_mips_imports`)
    // *before* the relocation-driven path below, which would otherwise early-return
    // on the empty `build_got_name_map`.  See [`resolve_mips_imports`].
    if matches!(arch, Architecture::Mips | Architecture::Mips64) {
        return resolve_mips_imports(file);
    }

    let got_to_name = build_got_name_map(file);
    if got_to_name.is_empty() {
        return Vec::new();
    }

    let mut out: Vec<PltSym> = Vec::new();
    // GOT slots already named, so an import that appears in both `.plt.sec` (the
    // CET call target) and `.plt` (the lazy-resolution stub) is named once, at
    // the higher-priority section.
    let mut named_got: HashSet<u64> = HashSet::new();

    // Scan PLT-bearing sections, `.plt.sec`/`.plt.got` before `.plt` so the
    // actual call target (CET `.plt.sec`) wins over the lazy `.plt` stub.
    let mut plt_secs: Vec<(u64, &[u8], u8)> = Vec::new();
    for sec in file.sections() {
        let name = match sec.name() {
            Ok(n) => n,
            Err(_) => continue,
        };
        let prio = match plt_section_priority(name) {
            Some(p) => p,
            None => continue,
        };
        let data = match sec.data() {
            Ok(d) => d,
            Err(_) => continue,
        };
        plt_secs.push((sec.address(), data, prio));
    }
    plt_secs.sort_by_key(|&(_, _, prio)| prio);

    // (kuna) i386-PIE PLT stubs are GOT-relative (`jmp *disp(%ebx)`, `FF A3
    // <disp32>`), so decoding them needs the GOT base that `%ebx` holds at the
    // call site.  Derive it once (gated by the default-on `i386_pie_plt` option —
    // angr `test_decompiling_nl_i386_pie`) and thread it to `decode_i386`; `None`
    // off-i386, with the option off, or when no GOT anchor exists (non-PIC i386),
    // in which case `decode_i386` decodes only the non-PIC `FF 25 <abs32>` form
    // exactly as before.
    let i386_got_base = if matches!(arch, Architecture::I386)
        && kuna_decomp::kuna_i386_pie_plt::i386_pie_plt_enabled()
    {
        i386_got_base(file)
    } else {
        None
    };

    for (vma, data, _) in plt_secs {
        decode_plt_section(arch, vma, data, i386_got_base, &got_to_name, &mut out, &mut named_got);
    }

    // PowerPC ELFv2 (and the older `bss-plt` ABI) has no decodable `.plt` *code*
    // section: `.plt` is a NOBITS data table (the runtime GOT for imports) and the
    // linker synthesizes the call stubs inline in `.text`.  Resolve those stubs
    // separately — they are TOC-relative, so they need the TOC base, which the
    // per-section scan above does not carry.  See [`decode_ppc64_text`].
    if matches!(arch, Architecture::PowerPc64 | Architecture::PowerPc) {
        decode_ppc_text(file, arch, &got_to_name, &mut out, &mut named_got);
    }

    out
}

/// Section-scan priority for the PLT-bearing sections, lowest scanned first.
/// `.plt.sec`/`.plt.got` (the real CET call targets / GOT-relative stubs) take
/// precedence over `.plt` (the lazy-resolution stubs) for the same GOT slot.
fn plt_section_priority(name: &str) -> Option<u8> {
    match name {
        ".plt.sec" => Some(0),
        ".plt.got" => Some(1),
        ".plt" => Some(2),
        // `.iplt` (IFUNC) carries IRELATIVE relocations (no symbol) → no names;
        // skip it explicitly rather than scanning for nothing.
        _ => None,
    }
}

/// Build `got_slot_addr → import_name` from the dynamic relocations.
///
/// The relocation's `RelocationTarget::Symbol(idx)` indexes the **dynamic**
/// symbol table (`.dynsym`), NOT `.symtab`; `Object::symbol_by_index` would
/// resolve the wrong table, so we build a `dynidx → name` map from
/// [`object::read::Object::dynamic_symbols`] keyed on each symbol's own index.
///
/// All symbol-bearing dynamic relocations are included (not just `JUMP_SLOT`):
/// it is harmless because only offsets that a decoded PLT stub actually
/// references are ever queried, and a GOT slot has exactly one relocation.
fn build_got_name_map(file: &object::File) -> HashMap<u64, Vec<u8>> {
    let mut dynnames: HashMap<usize, Vec<u8>> = HashMap::new();
    for s in file.dynamic_symbols() {
        if let Ok(n) = s.name_bytes() {
            if !n.is_empty() {
                dynnames.insert(s.index().0, strip_version(n));
            }
        }
    }

    let mut map: HashMap<u64, Vec<u8>> = HashMap::new();
    if let Some(relocs) = file.dynamic_relocations() {
        for (got_off, reloc) in relocs {
            if let RelocationTarget::Symbol(idx) = reloc.target() {
                if let Some(name) = dynnames.get(&idx.0) {
                    map.entry(got_off).or_insert_with(|| name.clone());
                }
            }
        }
    }
    map
}

/// Strip a glibc-style `@VERSION` / `@@VERSION` suffix from a symbol name.  The
/// raw `.dynstr` string is usually already clean (`puts`); this is defensive for
/// symbol-versioned libraries that fold the version into the string.  Shared with
/// [`crate::loadimage_object`] for the `.symtab`/`.dynsym` paths.
///
/// MSVC C++ mangling (multi-format loader PR-9) uses `@` *structurally* as the
/// namespace/type separator (`?foo@Bar@@QEAAHH@Z`), so a leading-`?` name is
/// returned verbatim — truncating at the first `@` would corrupt every MSVC
/// COFF/PE symbol before the demangler ever sees it. No glibc-versioned symbol
/// ever starts with `?`, so this guard never changes the ELF behavior.
pub(crate) fn strip_version(name: &[u8]) -> Vec<u8> {
    if name.first() == Some(&b'?') {
        return name.to_vec();
    }
    match name.iter().position(|&b| b == b'@') {
        Some(p) => name[..p].to_vec(),
        None => name.to_vec(),
    }
}

/// Decode every recognizable PLT stub in one `.plt*` section, pushing a
/// [`PltSym`] for each whose decoded GOT target is in `map` and not yet named.
fn decode_plt_section(
    arch: Architecture,
    vma: u64,
    data: &[u8],
    i386_got_base: Option<u64>,
    map: &HashMap<u64, Vec<u8>>,
    out: &mut Vec<PltSym>,
    named_got: &mut HashSet<u64>,
) {
    match arch {
        Architecture::X86_64 | Architecture::X86_64_X32 => {
            decode_x86_64(vma, data, map, out, named_got)
        }
        Architecture::I386 => decode_i386(vma, data, i386_got_base, map, out, named_got),
        Architecture::Aarch64 | Architecture::Aarch64_Ilp32 => {
            decode_aarch64(vma, data, map, out, named_got)
        }
        Architecture::Arm => decode_arm(vma, data, map, out, named_got),
        Architecture::Riscv64 | Architecture::Riscv32 => {
            decode_riscv(vma, data, map, out, named_got)
        }
        Architecture::Sparc | Architecture::Sparc32Plus | Architecture::Sparc64 => {
            decode_sparc(vma, data, map, out, named_got)
        }
        // PowerPC's stubs are not in a `.plt*` *code* section (`.plt` is a NOBITS
        // data table; the call stubs live in `.text`), so they are resolved out of
        // band in [`resolve_plt_imports`] via [`decode_ppc_text`], not here.
        //
        // MIPS has no `.plt` code section either (`.MIPS.stubs` + `$gp`-relative
        // GOT); it is resolved out of band by [`resolve_mips_imports`] before this
        // point (the GOT layout carries the names).  See docs/missing-analyses.md.
        _ => {}
    }
}

/// Push `(stub, got)`'s name if `got` is a known import not yet named.
fn record(
    stub: u64,
    got: u64,
    map: &HashMap<u64, Vec<u8>>,
    out: &mut Vec<PltSym>,
    named_got: &mut HashSet<u64>,
) {
    if let Some(name) = map.get(&got) {
        if named_got.insert(got) {
            out.push(PltSym { addr: stub, name: name.clone() });
        }
    }
}

// ---------------------------------------------------------------------------
// x86-64 / x86-32
// ---------------------------------------------------------------------------

/// x86-64 PLT stub: `[endbr64] [bnd] FF 25 <disp32>` = RIP-relative
/// `jmp *got(%rip)`.  The GOT slot is `rip + disp32` where `rip` is the address
/// of the byte after the 6-byte `FF 25 <disp32>`.  For CET `.plt.sec` the stub
/// entry (the real call target) starts at the leading `endbr64`, so the recorded
/// address is backed up over the `endbr64` (+ optional `bnd` prefix).
fn decode_x86_64(
    vma: u64,
    data: &[u8],
    map: &HashMap<u64, Vec<u8>>,
    out: &mut Vec<PltSym>,
    named_got: &mut HashSet<u64>,
) {
    let mut i = 0usize;
    while i + 6 <= data.len() {
        if data[i] == 0xFF && data[i + 1] == 0x25 {
            let disp = i32::from_le_bytes([data[i + 2], data[i + 3], data[i + 4], data[i + 5]]);
            let insn_addr = vma + i as u64;
            // rip = insn_addr + 6 (next instruction), GOT = rip + sign(disp32).
            let got = insn_addr.wrapping_add(6).wrapping_add(disp as i64 as u64);
            let stub = vma + x86_stub_start(data, i, /*endbr64*/ &[0xF3, 0x0F, 0x1E, 0xFA]) as u64;
            record(stub, got, map, out, named_got);
            i += 6;
        } else {
            i += 1;
        }
    }
}

/// x86-32 PLT stub.  Two forms:
///
/// * **Non-PIC**: `[endbr32] FF 25 <abs32>` = `jmp *abs32` where the disp32 is
///   the absolute GOT slot address (operand IS the slot).
/// * **PIE / PIC** (`i386_got_base = Some(base)`): `[endbr32] FF A3 <disp32>` =
///   `jmp *disp32(%ebx)`, GOT-relative, where `%ebx` holds the GOT base (the PIC
///   prologue's `call __x86.get_pc_thunk.bx; add $_,%ebx` sets it to
///   `_GLOBAL_OFFSET_TABLE_`).  The slot is `base + sign(disp32)`.  This is the
///   i386-PIE analog of [`decode_x86_64`] (RIP-relative) / [`decode_aarch64`]
///   (adrp/ldr veneer); naming these stubs lets the no-return pass flag `exit`
///   and collapses the spurious fall-through loop (angr
///   `test_decompiling_nl_i386_pie`).  `i386_got_base` is `None` when the GOT
///   base could not be derived or the `i386_pie_plt` option is off, in which case
///   the `FF A3` form is skipped (byte-identical to the pre-fix behavior); the
///   non-PIC `FF 25` arm is unaffected either way.
fn decode_i386(
    vma: u64,
    data: &[u8],
    i386_got_base: Option<u64>,
    map: &HashMap<u64, Vec<u8>>,
    out: &mut Vec<PltSym>,
    named_got: &mut HashSet<u64>,
) {
    let mut i = 0usize;
    while i + 6 <= data.len() {
        if data[i] == 0xFF && data[i + 1] == 0x25 {
            // Absolute (non-PIC): operand IS the GOT slot address.
            let got = u32::from_le_bytes([data[i + 2], data[i + 3], data[i + 4], data[i + 5]]) as u64;
            let stub = vma + x86_stub_start(data, i, /*endbr32*/ &[0xF3, 0x0F, 0x1E, 0xFB]) as u64;
            record(stub, got, map, out, named_got);
            i += 6;
        } else if data[i] == 0xFF && data[i + 1] == 0xA3 {
            // PIE / PIC: `jmp *disp32(%ebx)`, slot = GOT_base + sign(disp32).
            // Only resolvable with the derived GOT base (option-gated); without
            // it the stub is left to the `sub_<addr>` fallback (pre-fix behavior).
            if let Some(base) = i386_got_base {
                let disp =
                    i32::from_le_bytes([data[i + 2], data[i + 3], data[i + 4], data[i + 5]]);
                let got = base.wrapping_add(disp as i64 as u64);
                let stub = vma + x86_stub_start(data, i, /*endbr32*/ &[0xF3, 0x0F, 0x1E, 0xFB]) as u64;
                record(stub, got, map, out, named_got);
            }
            i += 6;
        } else {
            i += 1;
        }
    }
}

/// Derive the i386 PIC GOT base (the value `%ebx` holds) for resolving the PIE
/// `jmp *disp(%ebx)` (`FF A3 <disp32>`) PLT stubs: prefer the
/// `_GLOBAL_OFFSET_TABLE_` symbol value, falling back to the `.got.plt` then
/// `.got` section address.  `None` when no GOT anchor is present (a non-PIC i386
/// binary, where the `FF A3` form does not occur).  The i386 analog of
/// [`decode_ppc_text`]'s `.got + 0x8000` TOC-base derivation.
fn i386_got_base(file: &object::File) -> Option<u64> {
    // `_GLOBAL_OFFSET_TABLE_` is the exact base `%ebx` is set to; it may live in
    // `.symtab` (LOCAL) or `.dynsym`.
    for sym in file.symbols() {
        if sym.name_bytes() == Ok(&b"_GLOBAL_OFFSET_TABLE_"[..]) {
            return Some(sym.address());
        }
    }
    for sym in file.dynamic_symbols() {
        if sym.name_bytes() == Ok(&b"_GLOBAL_OFFSET_TABLE_"[..]) {
            return Some(sym.address());
        }
    }
    // Stripped of the GOT symbol: the section base is the same anchor.
    for name in [".got.plt", ".got"] {
        if let Some(s) = file.sections().find(|s| s.name() == Ok(name)) {
            return Some(s.address());
        }
    }
    None
}

/// Compute the stub entry start (section-relative) for an x86 `FF 25` at index
/// `i`, backing up over a CET `endbr` (`endbr64`/`endbr32`, 4 bytes) and an
/// optional `bnd` prefix (`F2`).  Without CET the stub starts at the `FF`.
fn x86_stub_start(data: &[u8], i: usize, endbr: &[u8; 4]) -> usize {
    // `[endbr] [F2] FF ...`  → start at endbr.
    if i >= 5 && data[i - 1] == 0xF2 && &data[i - 5..i - 1] == endbr {
        return i - 5;
    }
    // `[endbr] FF ...`       → start at endbr.
    if i >= 4 && &data[i - 4..i] == endbr {
        return i - 4;
    }
    // `[F2] FF ...`          → start at the bnd prefix.
    if i >= 1 && data[i - 1] == 0xF2 {
        return i - 1;
    }
    i
}

// ---------------------------------------------------------------------------
// AArch64 / ARM32 / RISC-V  (filled in by the per-arch follow-up; each decodes
// the stub's GOT reference and calls `record(stub, got, …)`).  Until then they
// no-op, so those arches keep the pre-fix behavior.
// ---------------------------------------------------------------------------

/// AArch64 standard GNU `ld` import veneer (16-byte entry):
/// ```text
///     adrp x16, GOT_page          ; PC-relative page of the GOT slot
///     ldr  x17, [x16, #lo12]      ; 64-bit unsigned-offset load (scaled by 8)
///     add  x16, x16, #lo12
///     br   x17
/// ```
/// `got = (adrp_addr & !0xFFF) + (sign_extend_21(immhi:immlo) << 12) + imm12*8`.
/// A64 is fixed 32-bit little-endian, so we scan at 4-byte alignment matching the
/// `adrp x16` + `ldr x17,[x16]` pair; the 32-byte PLT0 header and any leading
/// `bti c` fall out of the pattern, and the GOT-vs-map cross-check drops the rest.
fn decode_aarch64(
    vma: u64,
    data: &[u8],
    map: &HashMap<u64, Vec<u8>>,
    out: &mut Vec<PltSym>,
    named_got: &mut HashSet<u64>,
) {
    let mut off = 0usize;
    while off + 8 <= data.len() {
        let adrp = u32::from_le_bytes([data[off], data[off + 1], data[off + 2], data[off + 3]]);
        let ldr =
            u32::from_le_bytes([data[off + 4], data[off + 5], data[off + 6], data[off + 7]]);

        // adrp Xd, imm: op=1 (bit31), bits[28:24]=10000; immlo=bits[30:29],
        // immhi=bits[23:5], Rd=bits[4:0].
        let is_adrp = (adrp >> 31) & 1 == 1 && (adrp >> 24) & 0x1f == 0b1_0000;
        let adrp_rd = adrp & 0x1f;
        // ldr Xt,[Xn,#imm] 64-bit unsigned offset: size=11(bits[31:30]),
        // bits[29:24]=111001, opc=01(bits[23:22]=LDR), imm12=bits[21:10],
        // Rn=bits[9:5], Rt=bits[4:0].
        let is_ldr = (ldr >> 30) & 0x3 == 0b11
            && (ldr >> 24) & 0x3f == 0b11_1001
            && (ldr >> 22) & 0x3 == 0b01;
        let ldr_rn = (ldr >> 5) & 0x1f;
        let ldr_rt = ldr & 0x1f;

        if is_adrp && adrp_rd == 16 && is_ldr && ldr_rn == 16 && ldr_rt == 17 {
            let immlo = ((adrp >> 29) & 0x3) as i64;
            let immhi = ((adrp >> 5) & 0x7_ffff) as i64; // 19 bits
            let mut imm21 = (immhi << 2) | immlo;
            if imm21 & (1 << 20) != 0 {
                imm21 -= 1 << 21; // sign-extend the 21-bit page offset
            }
            let page_off = imm21 << 12;
            let ldr_off = (((ldr >> 10) & 0xfff) as u64) * 8; // 64-bit scale
            let adrp_addr = vma.wrapping_add(off as u64);
            let got = (adrp_addr & !0xFFF)
                .wrapping_add(page_off as u64)
                .wrapping_add(ldr_off);
            record(adrp_addr, got, map, out, named_got);
        }
        off += 4;
    }
}

/// ARM A32 "modified immediate" (data-processing operand2): an 8-bit value
/// `imm8` rotated right by `2*rot`.  `op2` is the low 12 bits of the instruction.
fn arm_modified_immediate(op2: u32) -> u32 {
    let rot = (op2 >> 8) & 0xF;
    let imm8 = op2 & 0xFF;
    imm8.rotate_right(2 * rot)
}

/// ARM (32-bit, little-endian, ARM-state) classic GNU `ld` import veneer
/// (12-byte entry):
/// ```text
///     add ip, pc, #a        ; pc here = entry+8 (ARM pipeline offset)
///     add ip, ip, #b
///     ldr pc, [ip, #c]!     ; pre-indexed, U=1 (additive)
/// ```
/// `got = (entry + 8) + a + b + c`, where `a`/`b` are modified immediates and `c`
/// is the 12-bit `ldr` offset.  Scanned at 4-byte alignment; the 20-byte PLT0
/// header and Thumb veneers do not match.
fn decode_arm(
    vma: u64,
    data: &[u8],
    map: &HashMap<u64, Vec<u8>>,
    out: &mut Vec<PltSym>,
    named_got: &mut HashSet<u64>,
) {
    let read_word = |off: usize| -> Option<u32> {
        let b = data.get(off..off + 4)?;
        Some(u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
    };
    // `add ip, <Rn>, #imm`: cond=0xE, I=1,S=0,opcode=ADD → bits[27:20]==0x28,
    // Rd field == 12 (ip).
    let is_add_ip = |w: u32, rn: u32| -> bool {
        (w >> 28) == 0xE
            && ((w >> 20) & 0xFF) == 0x28
            && ((w >> 16) & 0xF) == rn
            && ((w >> 12) & 0xF) == 12
    };

    let mut off = 0usize;
    while off + 12 <= data.len() {
        let (w0, w1, w2) = match (read_word(off), read_word(off + 4), read_word(off + 8)) {
            (Some(a), Some(b), Some(c)) => (a, b, c),
            _ => break,
        };
        // `ldr pc, [ip, #imm]!`: cond=0xE, bits[27:20]==0x5B (P=1,U=1,W=1,L=1,
        // immediate), Rn=12(ip), Rt=15(pc).
        let is_ldr_pc_ip = (w2 >> 28) == 0xE
            && ((w2 >> 20) & 0xFF) == 0x5B
            && ((w2 >> 16) & 0xF) == 12
            && ((w2 >> 12) & 0xF) == 15;

        if is_add_ip(w0, 15) && is_add_ip(w1, 12) && is_ldr_pc_ip {
            let entry = vma.wrapping_add(off as u64);
            let a = arm_modified_immediate(w0 & 0xFFF) as u64;
            let b = arm_modified_immediate(w1 & 0xFFF) as u64;
            let c = (w2 & 0xFFF) as u64; // U=1 → additive offset
            let got = entry
                .wrapping_add(8) // ARM pipeline: pc = entry + 8
                .wrapping_add(a)
                .wrapping_add(b)
                .wrapping_add(c);
            record(entry, got, map, out, named_got);
        }
        off += 4;
    }
}

/// RISC-V (RV32/RV64) standard GNU `ld` import veneer (16-byte entry):
/// ```text
///     auipc t3, %pcrel_hi(got)     ; t3 = pc + (hi20 << 12)
///     l[wd] t3, %pcrel_lo(t3)(t3)  ; ld (RV64, funct3=3) / lw (RV32, funct3=2)
///     jalr  t1, t3
///     nop                          ; (c.nop padding, ignored)
/// ```
/// `got = auipc_addr + (hi20 << 12) + sign_extend_12(lo12)` (`t3`=x28, `t1`=x6).
/// Scanned at 4-byte alignment; the 32-byte PLT0 header does not match.
fn decode_riscv(
    vma: u64,
    data: &[u8],
    map: &HashMap<u64, Vec<u8>>,
    out: &mut Vec<PltSym>,
    named_got: &mut HashSet<u64>,
) {
    let word = |off: usize| -> Option<u32> {
        data.get(off..off + 4).map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
    };

    let mut off = 0usize;
    while off + 8 <= data.len() {
        let auipc = match word(off) {
            Some(w) => w,
            None => break,
        };
        // auipc t3, imm: opcode 0x17 (bits[6:0]), rd 28 (bits[11:7]).
        if (auipc & 0x7f) != 0x17 || ((auipc >> 7) & 0x1f) != 28 {
            off += 4;
            continue;
        }
        let load = match word(off + 4) {
            Some(w) => w,
            None => break,
        };
        // l[wd] t3, lo12(t3): opcode 0x03, funct3 ∈ {2 (lw), 3 (ld)}, rd 28,
        // rs1 28.
        let funct3 = (load >> 12) & 0x7;
        let load_ok = (load & 0x7f) == 0x03
            && (funct3 == 2 || funct3 == 3)
            && ((load >> 7) & 0x1f) == 28
            && ((load >> 15) & 0x1f) == 28;
        if !load_ok {
            off += 4;
            continue;
        }

        let hi20 = (auipc >> 12) & 0xf_ffff; // imm[31:12], used as-is
        let imm12 = (load >> 20) & 0xfff;
        let lo12 = ((imm12 as i32) << 20) >> 20; // sign-extend 12-bit
        let auipc_addr = vma.wrapping_add(off as u64);
        let got = auipc_addr
            .wrapping_add((hi20 as u64) << 12)
            .wrapping_add(lo12 as i64 as u64);
        record(auipc_addr, got, map, out, named_got);
        off += 16; // skip the rest of the 16-byte veneer
    }
}

// ---------------------------------------------------------------------------
// PowerPC64 ELFv2 (and PPC32) — TOC-relative `.text` call stubs
// ---------------------------------------------------------------------------

/// Resolve PowerPC PLT-call stubs, which the linker synthesizes inline in
/// `.text` (there is no `.plt` *code* section — `.plt` is a NOBITS data table,
/// the runtime GOT for imports).
///
/// On PPC64 ELFv2 a call to an imported function `bl`s to a stub of the form
/// ```text
///     std   r2,24(r1)        ; save the caller TOC (0xf8410018)
///     addis r12,r2,off@ha    ; r12 = TOC_base + (ha << 16)
///     ld    r12,off@l(r12)   ; r12 = *(.plt slot) = *(TOC_base + (ha<<16) + lo)
///     mtctr r12
///     bctr
/// ```
/// The loaded address is a `.plt` slot whose `R_PPC64_JMP_SLOT` relocation names
/// the import — exactly the `got_slot → name` map [`build_got_name_map`] already
/// built (every symbol-bearing dynamic reloc is included, so the JMP_SLOTs are in
/// it).  We decode the `addis`/`ld` pair, reconstruct the slot
/// `TOC_base + (ha << 16) + sign_extend16(lo)`, and record the stub entry (the
/// `std r2,24(r1)` — the address `bl` targets) against the matched name.
///
/// **TOC base.**  The ELFv2 ABI fixes the TOC pointer (`r2`) at
/// `.got + 0x8000` (equivalently the `.TOC.` symbol); using `.got + 0x8000`
/// avoids relying on a `.TOC.` symbol being present.  If there is no `.got`
/// section the binary has no TOC-relative imports and we resolve nothing.
///
/// PPC32 (`Architecture::PowerPc`, big-endian) uses a different secure-PLT stub
/// shape that this routine does not model, so it no-ops there — left as a seam
/// (`decode_ppc_text` reads words honoring endianness, so a future PPC32 arm can
/// slot in here).
fn decode_ppc_text(
    file: &object::File,
    arch: Architecture,
    map: &HashMap<u64, Vec<u8>>,
    out: &mut Vec<PltSym>,
    named_got: &mut HashSet<u64>,
) {
    // ELFv2 PPC64 is the only PowerPC ABI whose `.text` stubs we decode.
    if arch != Architecture::PowerPc64 {
        return;
    }
    let little = file.is_little_endian();

    // TOC base = `.got` vma + 0x8000 (ELFv2). No `.got` → no TOC-relative imports.
    let got_vma = match file.sections().find(|s| s.name() == Ok(".got")) {
        Some(s) => s.address(),
        None => return,
    };
    let toc_base = got_vma.wrapping_add(0x8000);

    // The stubs are inline in `.text`.
    let text = match file.sections().find(|s| s.name() == Ok(".text")) {
        Some(s) => s,
        None => return,
    };
    let vma = text.address();
    let data = match text.data() {
        Ok(d) => d,
        Err(_) => return,
    };

    decode_ppc64_stubs(vma, data, little, toc_base, map, out, named_got);
}

/// Decode the PPC64-ELFv2 `std/addis/ld/mtctr/bctr` PLT stubs in a `.text` blob.
/// Pulled out from [`decode_ppc_text`] so the arithmetic is unit-testable without
/// an `object::File`.  `toc_base` is the ELFv2 TOC pointer (`.got + 0x8000`).
fn decode_ppc64_stubs(
    vma: u64,
    data: &[u8],
    little: bool,
    toc_base: u64,
    map: &HashMap<u64, Vec<u8>>,
    out: &mut Vec<PltSym>,
    named_got: &mut HashSet<u64>,
) {
    let word = |off: usize| -> Option<u32> {
        let b = data.get(off..off + 4)?;
        let arr = [b[0], b[1], b[2], b[3]];
        Some(if little { u32::from_le_bytes(arr) } else { u32::from_be_bytes(arr) })
    };

    // Instruction fields (big-endian instruction semantics regardless of byte
    // order in the file): opcode = bits[0:6], RT = bits[6:11], RA = bits[11:16].
    let opcode = |w: u32| (w >> 26) & 0x3f;
    let rt = |w: u32| (w >> 21) & 0x1f;
    let ra = |w: u32| (w >> 16) & 0x1f;

    let mut off = 0usize;
    while off + 20 <= data.len() {
        let (w0, w1, w2, w3, w4) = match (
            word(off),
            word(off + 4),
            word(off + 8),
            word(off + 12),
            word(off + 16),
        ) {
            (Some(a), Some(b), Some(c), Some(d), Some(e)) => (a, b, c, d, e),
            _ => break,
        };

        // std r2,24(r1)         = 0xf8410018  (DS-form, opcode 62, RT=2, RA=1)
        // addis r12,r2,off@ha   opcode 15, RT=12, RA=2
        // ld    r12,off@l(r12)  DS-form, opcode 58, XO=0, RT=12, RA=12
        // mtctr r12             = 0x7d8903a6
        // bctr                  = 0x4e800420
        let is_std_r2 = w0 == 0xf8410018;
        let is_addis = opcode(w1) == 15 && rt(w1) == 12 && ra(w1) == 2;
        let is_ld = opcode(w2) == 58 && (w2 & 0x3) == 0 && rt(w2) == 12 && ra(w2) == 12;
        let is_mtctr = w3 == 0x7d8903a6;
        let is_bctr = w4 == 0x4e800420;

        if is_std_r2 && is_addis && is_ld && is_mtctr && is_bctr {
            // addis @ha: sign-extend the 16-bit field, then << 16.
            let ha = ((w1 & 0xffff) as i16) as i64;
            // ld DS-form: the 14-bit DS displacement is the low 16 bits with the
            // bottom two bits forced to the XO (0 for `ld`); sign-extend it.
            let lo = ((w2 & 0xfffc) as i16) as i64;
            let slot = toc_base
                .wrapping_add((ha << 16) as u64)
                .wrapping_add(lo as u64);
            let stub = vma.wrapping_add(off as u64);
            record(stub, slot, map, out, named_got);
            off += 20; // skip past this 5-instruction stub
            continue;
        }
        off += 4;
    }
}

// ---------------------------------------------------------------------------
// SPARC (SPARC v9 / 32-bit, big-endian)
// ---------------------------------------------------------------------------

/// SPARC PLT import veneer (32-byte entry, big-endian fixed 32-bit insns):
/// ```text
///     sethi  %hi(entry_offset), %g1   ; 0x03______  — relocation index marker
///     b,a    %xcc, <PLT0 resolver>    ; 0x30 6f ____ — branch to the header stub
///     nop ; nop ; nop ; nop ; nop ; nop
/// ```
/// Unlike every other arch, SPARC's `.rela.plt` `R_SPARC_JMP_SLOT` relocation
/// `r_offset` **is the PLT entry address itself** (the linker rewrites the
/// in-place stub at resolution time — see Ghidra's `SPARC*_ElfRelocationHandler`),
/// not a separate `.got` slot.  So the "got_slot → name" map ([`build_got_name_map`])
/// is already keyed on the call target the decompiler sees: the decoder just walks
/// the section in 32-byte strides and records any entry whose address is a known
/// symbol-bearing relocation.  The 4-slot (`0x80`-byte) reserved PLT0 header and
/// the non-symbol `__gmon_start__`/IRELATIVE slots are not relocation keys, so they
/// fall out of the map automatically (same self-correcting cross-check as the
/// other arches; here `stub == got`).
fn decode_sparc(
    vma: u64,
    data: &[u8],
    map: &HashMap<u64, Vec<u8>>,
    out: &mut Vec<PltSym>,
    named_got: &mut HashSet<u64>,
) {
    // SPARC `.plt` entries are 32 bytes; the first instruction of an import
    // veneer is `sethi %hi(...), %g1` (top byte 0x03 big-endian).  Stride by the
    // 32-byte entry size, and gate on both the `sethi %g1` opcode and the map
    // membership so a non-PLT section never produces spurious names.
    const ENTRY: usize = 32;
    let mut off = 0usize;
    while off + 4 <= data.len() {
        // First word, big-endian.  `sethi rd, imm22`: op=00 (bits[31:30]),
        // op2=100 (bits[24:22]), rd=bits[29:25]; rd==1 (%g1) and op2==0b100 give
        // the canonical `sethi %hi(...),%g1` = 0x03xxxxxx import-veneer head.
        let w0 = u32::from_be_bytes([data[off], data[off + 1], data[off + 2], data[off + 3]]);
        let is_sethi_g1 = (w0 >> 30) == 0b00 && ((w0 >> 25) & 0x1f) == 1 && ((w0 >> 22) & 0x7) == 0b100;
        if is_sethi_g1 {
            let entry = vma.wrapping_add(off as u64);
            // SPARC: the JMP_SLOT relocation offset == the PLT entry address, so
            // the stub and the GOT-name-map key are one and the same.
            record(entry, entry, map, out, named_got);
        }
        off += ENTRY;
    }
}

// ---------------------------------------------------------------------------
// MIPS (.MIPS.stubs / GOT layout)
// ---------------------------------------------------------------------------

/// Resolve MIPS import names from the dynamic-symbol GOT layout — the kuna analog
/// of Ghidra's `MIPS_ElfExtension.fixupGot` + `processMipsStubsSection`
/// (`Ghidra/Processors/MIPS/.../elf/extend/MIPS_ElfExtension.java`).
///
/// ## Why MIPS is different
///
/// Unlike x86/ARM/RISC-V, the GNU MIPS o32 toolchain emits **no `.plt` code
/// section and no `R_MIPS_JUMP_SLOT` relocations** for its lazy imports.  Instead
/// the dynamic linker resolves a fixed slice of the GOT in *symbol-table order*:
/// the doc anchor is the "MIPS Multi-GOT" layout (the cr0.org
/// `mips.elf.external.resolution.txt` paper that Ghidra cites).  The
/// correspondence is purely positional:
///
/// * `DT_MIPS_LOCAL_GOTNO` (0x7000000a) — the count of *local* GOT entries that
///   precede the first global (external-symbol) GOT entry.
/// * `DT_MIPS_GOTSYM` (0x70000013) — the **first** dynamic-symbol index that has a
///   GOT entry.  Dynamic symbols `[GOTSYM .. dynsym_count)` map 1:1, in order, to
///   GOT entries starting at index `LOCAL_GOTNO`:
///
///   ```text
///     got_index(i) = LOCAL_GOTNO + (i - GOTSYM)        for i >= GOTSYM
///   ```
///
/// * `DT_PLTGOT` (3) — the GOT base address (`gotBaseOffset`).
///
/// For an **undefined FUNC** dynamic symbol (an import), the GOT slot is
/// initialized to the address of that import's lazy-binding stub in `.MIPS.stubs`
/// (and that same stub address is the symbol's `st_value`).  At a MIPS call site
/// the compiler emits `lw $t9, off($gp); jalr $t9` — an indirect call through the
/// GOT slot.  With kuna's `$gp` recovery (Increment 17) the engine constant-folds
/// the GOT load to that slot's contents (the stub address), so naming the **stub
/// address** as a `FunctionSymbol` makes the call render `puts(...)` instead of
/// `sub_<addr>` — exactly mirroring `createExternalFunctionLinkage(symName,
/// refAddr, gotEntryAddr)` where `refAddr` is the GOT-slot contents (the stub).
///
/// Both naming targets are emitted: the **stub address** (`refAddr`, the indirect
/// call target the decompiler sees) and, defensively, the **GOT slot address**
/// itself (in case a future pass names the slot directly).  Everything degrades
/// gracefully: a missing `DT_MIPS_*` tag, a missing GOT, or a 0 slot yields fewer
/// (or zero) entries and leaves behavior unchanged — this never panics or errors.
fn resolve_mips_imports(file: &object::File) -> Vec<PltSym> {
    let mut out: Vec<PltSym> = Vec::new();
    for ext in mips_external_got_entries(file) {
        // The GOT slot's static contents = the `.MIPS.stubs` stub address (the
        // import's `refAddr`); this is the indirect call target the decompiler
        // resolves after $gp recovery.  Skip a 0 slot (unresolvable).
        if ext.stub_addr != 0 {
            out.push(PltSym { addr: ext.stub_addr, name: ext.name });
        }
    }
    out
}

/// The MIPS GOT external slots' `[start, stop]` (inclusive) byte ranges, to be
/// marked **constant** so the engine folds the `lw $t9, off($gp)` indirect-call
/// load to the stub address (the analog of `setConstant` on the GOT pointer
/// entries in Ghidra's `MIPS_ElfExtension.fixupGot`).  Empty off-MIPS or when the
/// `DT_MIPS_*` tags are absent.  One slot per external dynamic symbol (FUNC or
/// not — a constant GOT pointer is harmless for non-FUNC slots and matches
/// Ghidra, which `setConstant`s every external GOT entry).
pub(crate) fn mips_got_const_ranges(file: &object::File) -> Vec<(u64, u64)> {
    if !matches!(file.architecture(), Architecture::Mips | Architecture::Mips64) {
        return Vec::new();
    }
    let ptr = if file.is_64() { 8u64 } else { 4u64 };
    mips_external_got_entries(file)
        .into_iter()
        .map(|e| (e.slot_addr, e.slot_addr.wrapping_add(ptr - 1)))
        .collect()
}

/// One external-symbol GOT entry on MIPS: the dynsym name, the GOT slot address,
/// and the slot's static contents (the `.MIPS.stubs` stub address it points at).
struct MipsExtGot {
    name: Vec<u8>,
    slot_addr: u64,
    stub_addr: u64,
}

/// Walk the MIPS external-symbol GOT window — dynsym indices `[GOTSYM, count)`
/// map 1:1, in order, to GOT entries starting at index `LOCAL_GOTNO`
/// (`got_index(i) = LOCAL_GOTNO + (i - GOTSYM)`).  Yields one [`MipsExtGot`] per
/// **undefined FUNC** import (Ghidra's `isFunction() && sectionHeaderIndex == 0`).
/// Empty when the `DT_MIPS_LOCAL_GOTNO`/`DT_MIPS_GOTSYM`/`DT_PLTGOT` tags are
/// absent.  Shared by [`resolve_mips_imports`] (naming) and
/// [`mips_got_const_ranges`] (constant-folding); never panics or errors.
fn mips_external_got_entries(file: &object::File) -> Vec<MipsExtGot> {
    let mut out: Vec<MipsExtGot> = Vec::new();

    // The dynamic table tags we need, read straight from the `.dynamic` bytes.
    let Some((local_gotno, gotsym, got_base)) = read_mips_dynamic_tags(file) else {
        return out;
    };

    // The dynamic symbols, by index (the positional GOT correspondence is by
    // `.dynsym` index).
    let mut dynsyms: Vec<(usize, object::read::Symbol)> = Vec::new();
    for s in file.dynamic_symbols() {
        dynsyms.push((s.index().0, s));
    }
    if dynsyms.is_empty() {
        return out;
    }
    let dynsym_count = dynsyms.iter().map(|(i, _)| *i).max().unwrap_or(0) + 1;

    let ptr = if file.is_64() { 8usize } else { 4usize };
    let le = file.is_little_endian();

    for &(idx, ref sym) in &dynsyms {
        if idx < gotsym as usize || idx >= dynsym_count {
            continue;
        }
        // Only undefined FUNC imports name a stub.
        if sym.kind() != object::SymbolKind::Text || !sym.is_undefined() {
            continue;
        }
        let name = match sym.name_bytes() {
            Ok(n) if !n.is_empty() => strip_version(n),
            _ => continue,
        };
        let got_index = local_gotno + (idx as u64 - gotsym);
        let slot_addr = got_base.wrapping_add(got_index * ptr as u64);
        let stub_addr = read_word_at_vma(file, slot_addr, ptr, le).unwrap_or(0);
        out.push(MipsExtGot { name, slot_addr, stub_addr });
    }

    out
}

/// Read `(DT_MIPS_LOCAL_GOTNO, DT_MIPS_GOTSYM, DT_PLTGOT)` from the `.dynamic`
/// section, or `None` if any is absent.  Tags/vals are `Elf{32,64}_Dyn` pairs.
fn read_mips_dynamic_tags(file: &object::File) -> Option<(u64, u64, u64)> {
    const DT_NULL: u64 = 0;
    const DT_PLTGOT: u64 = 3;
    const DT_MIPS_LOCAL_GOTNO: u64 = 0x7000_000a;
    const DT_MIPS_GOTSYM: u64 = 0x7000_0013;

    let data = file.section_by_name(".dynamic")?.data().ok()?;
    let is64 = file.is_64();
    let le = file.is_little_endian();
    let entsz = if is64 { 16usize } else { 8usize };

    let mut local_gotno: Option<u64> = None;
    let mut gotsym: Option<u64> = None;
    let mut pltgot: Option<u64> = None;

    let mut off = 0usize;
    while off + entsz <= data.len() {
        let (tag, val) = if is64 {
            (read_u64_le(&data[off..], le), read_u64_le(&data[off + 8..], le))
        } else {
            (read_u32_le(&data[off..], le) as u64, read_u32_le(&data[off + 4..], le) as u64)
        };
        off += entsz;
        if tag == DT_NULL {
            break;
        }
        match tag {
            DT_PLTGOT => pltgot = Some(val),
            DT_MIPS_LOCAL_GOTNO => local_gotno = Some(val),
            DT_MIPS_GOTSYM => gotsym = Some(val),
            _ => {}
        }
    }
    Some((local_gotno?, gotsym?, pltgot?))
}

/// Read a `ptr`-byte little/big-endian word from the section that contains `vma`
/// (the static-image base-0 assumption: the GOT vma is the file vma).  `None` if
/// no section covers it.
fn read_word_at_vma(file: &object::File, vma: u64, ptr: usize, le: bool) -> Option<u64> {
    for sec in file.sections() {
        let addr = sec.address();
        let size = sec.size();
        if size == 0 || vma < addr || vma >= addr.wrapping_add(size) {
            continue;
        }
        let data = sec.data().ok()?;
        let o = (vma - addr) as usize;
        if o + ptr > data.len() {
            return None;
        }
        return Some(if ptr == 8 {
            read_u64_le(&data[o..], le)
        } else {
            read_u32_le(&data[o..], le) as u64
        });
    }
    None
}

/// Read a 4-byte word honoring endianness (`le`); 0 on a short slice.
fn read_u32_le(b: &[u8], le: bool) -> u32 {
    if b.len() < 4 {
        return 0;
    }
    let a = [b[0], b[1], b[2], b[3]];
    if le {
        u32::from_le_bytes(a)
    } else {
        u32::from_be_bytes(a)
    }
}

/// Read an 8-byte word honoring endianness (`le`); 0 on a short slice.
fn read_u64_le(b: &[u8], le: bool) -> u64 {
    if b.len() < 8 {
        return 0;
    }
    let a = [b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]];
    if le {
        u64::from_le_bytes(a)
    } else {
        u64::from_be_bytes(a)
    }
}

#[cfg(test)]
mod tests {
    //! Synthetic per-architecture decoder tests.  Each builds a `.plt`-shaped
    //! byte buffer (a dummy PLT0 header + one real import veneer whose bytes were
    //! emitted by `clang --target=<arch> -c`), inserts the expected GOT slot —
    //! computed *independently* from the assembled operands — into the name map,
    //! and asserts the decoder recovers the planted name at the veneer's entry
    //! address.  The x86 paths are covered end-to-end by the real-binary fixtures
    //! in `loadimage_object`'s tests.
    use super::*;

    #[test]
    fn aarch64_plt_decode() {
        // clang bytes: `adrp x16,0x3000; ldr x17,[x16,#0xff8]; add x16,x16,#0xff8; br x17`.
        let mut data: Vec<u8> = vec![0u8; 32]; // dummy PLT0 header
        data.extend_from_slice(&0xf0000010u32.to_le_bytes()); // adrp x16, +0x3000
        data.extend_from_slice(&0xf947fe11u32.to_le_bytes()); // ldr  x17,[x16,#0xff8]
        data.extend_from_slice(&0x913fe210u32.to_le_bytes()); // add  x16,x16,#0xff8
        data.extend_from_slice(&0xd61f0220u32.to_le_bytes()); // br   x17

        let vma = 0x10000u64;
        let adrp_addr = vma + 32;
        // got = (adrp_addr & !0xFFF) + page(0x3000) + ldr_off(0xff8)
        let expected_got = (adrp_addr & !0xFFF) + 0x3000 + 0xff8;
        assert_eq!(expected_got, 0x13ff8);

        let mut map: HashMap<u64, Vec<u8>> = HashMap::new();
        map.insert(expected_got, b"read".to_vec());
        let mut out: Vec<PltSym> = Vec::new();
        let mut named: HashSet<u64> = HashSet::new();
        decode_aarch64(vma, &data, &map, &mut out, &mut named);

        assert_eq!(out.len(), 1);
        assert_eq!(out[0].addr, adrp_addr);
        assert_eq!(out[0].name, b"read".to_vec());
    }

    #[test]
    fn arm_plt_decode() {
        // clang bytes: `add ip,pc,#0x600000; add ip,ip,#0x3000; ldr pc,[ip,#0xff8]!`.
        let veneer: [u8; 12] = [
            0x06, 0xc6, 0x8f, 0xe2, // add ip, pc, #0x600000
            0x03, 0xca, 0x8c, 0xe2, // add ip, ip, #0x3000
            0xf8, 0xff, 0xbc, 0xe5, // ldr pc, [ip, #0xff8]!
        ];
        let mut data = vec![0u8; 20]; // dummy PLT0 header
        data.extend_from_slice(&veneer);

        let vma = 0x10000u64;
        let entry = vma + 20;
        let (a, b, c): (u64, u64, u64) = (0x600000, 0x3000, 0xff8);
        let expected_got = (entry + 8) + a + b + c; // pipeline +8

        let mut map: HashMap<u64, Vec<u8>> = HashMap::new();
        map.insert(expected_got, b"read".to_vec());
        let mut out: Vec<PltSym> = Vec::new();
        let mut named: HashSet<u64> = HashSet::new();
        decode_arm(vma, &data, &map, &mut out, &mut named);

        assert_eq!(out.len(), 1);
        assert_eq!(out[0].addr, entry);
        assert_eq!(out[0].name, b"read".to_vec());
    }

    #[test]
    fn ppc64_plt_decode() {
        // Real bytes from a `powerpc64le-linux-gnu-gcc -O0` ELFv2 stub (the
        // `puts@plt` call stub at 0x680 of fixtures/plt_ppc64le):
        //   std   r2,24(r1)        f8 41 00 18
        //   addis r12,r2,-1        3d 82 ff ff
        //   ld    r12,32752(r12)   e9 8c 7f f0
        //   mtctr r12              7d 89 03 a6
        //   bctr                   4e 80 04 20
        // File is little-endian (ELFv2 LE), so each word is stored byte-reversed.
        let mut stub: Vec<u8> = Vec::new();
        for w in [0xf8410018u32, 0x3d82ffff, 0xe98c7ff0, 0x7d8903a6, 0x4e800420] {
            stub.extend_from_slice(&w.to_le_bytes());
        }
        let mut data = vec![0u8; 16]; // a little leading `.text` padding
        data.extend_from_slice(&stub);

        let vma = 0x620u64; // `.text` base from the real fixture
        let stub_addr = vma + 16;
        // TOC base = .got(0x1ff00) + 0x8000 = 0x27f00; slot = TOC + (-1<<16) + 32752.
        let toc_base = 0x27f00u64;
        let expected_slot = toc_base.wrapping_add(((-1i64) << 16) as u64) + 32752;
        assert_eq!(expected_slot, 0x1fef0); // matches the R_PPC64_JMP_SLOT offset

        let mut map: HashMap<u64, Vec<u8>> = HashMap::new();
        map.insert(expected_slot, b"puts".to_vec());
        let mut out: Vec<PltSym> = Vec::new();
        let mut named: HashSet<u64> = HashSet::new();
        decode_ppc64_stubs(vma, &data, /*little*/ true, toc_base, &map, &mut out, &mut named);

        assert_eq!(out.len(), 1);
        assert_eq!(out[0].addr, stub_addr);
        assert_eq!(out[0].name, b"puts".to_vec());

        // Big-endian word layout decodes identically (the stub shape is shared by
        // ELFv2 BE); a positive @ha exercises the other sign of the page offset.
        let mut data_be = vec![0u8; 0];
        for w in [0xf8410018u32, 0x3d820002, 0xe98c0010, 0x7d8903a6, 0x4e800420] {
            data_be.extend_from_slice(&w.to_be_bytes());
        }
        let toc_be = 0x40000u64;
        let expected_be = toc_be + (2u64 << 16) + 0x10;
        let mut map_be: HashMap<u64, Vec<u8>> = HashMap::new();
        map_be.insert(expected_be, b"printf".to_vec());
        let mut out_be: Vec<PltSym> = Vec::new();
        let mut named_be: HashSet<u64> = HashSet::new();
        decode_ppc64_stubs(0x1000, &data_be, /*little*/ false, toc_be, &map_be, &mut out_be, &mut named_be);
        assert_eq!(out_be.len(), 1);
        assert_eq!(out_be[0].addr, 0x1000);
        assert_eq!(out_be[0].name, b"printf".to_vec());
    }

    #[test]
    fn riscv_plt_decode() {
        let vma = 0x10000u64;

        // Case A: positive lo12 — `auipc t3,0x3; ld t3,16(t3); jalr t1,t3; nop`.
        let mut data = vec![0u8; 32];
        data.extend_from_slice(&0x00003e17u32.to_le_bytes()); // auipc t3, 0x3
        data.extend_from_slice(&0x010e3e03u32.to_le_bytes()); // ld t3, 16(t3)
        data.extend_from_slice(&0x000e0367u32.to_le_bytes()); // jalr t1, t3
        data.extend_from_slice(&0x0001u16.to_le_bytes()); // c.nop
        let auipc_addr = vma + 32;
        let expected_got = auipc_addr + (0x3u64 << 12) + 0x10;

        let mut map: HashMap<u64, Vec<u8>> = HashMap::new();
        map.insert(expected_got, b"read".to_vec());
        let mut out: Vec<PltSym> = Vec::new();
        let mut named: HashSet<u64> = HashSet::new();
        decode_riscv(vma, &data, &map, &mut out, &mut named);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].addr, auipc_addr);
        assert_eq!(out[0].name, b"read".to_vec());

        // Case B: negative lo12 — `ld t3,-16(t3)` exercises sign extension.
        let mut data2 = vec![0u8; 32];
        data2.extend_from_slice(&0x00003e17u32.to_le_bytes()); // auipc t3, 0x3
        data2.extend_from_slice(&0xff0e3e03u32.to_le_bytes()); // ld t3, -16(t3)
        data2.extend_from_slice(&0x000e0367u32.to_le_bytes()); // jalr t1, t3
        data2.extend_from_slice(&0x0001u16.to_le_bytes()); // c.nop
        let expected_got2 = auipc_addr + (0x3u64 << 12) - 0x10;

        let mut map2: HashMap<u64, Vec<u8>> = HashMap::new();
        map2.insert(expected_got2, b"write".to_vec());
        let mut out2: Vec<PltSym> = Vec::new();
        let mut named2: HashSet<u64> = HashSet::new();
        decode_riscv(vma, &data2, &map2, &mut out2, &mut named2);
        assert_eq!(out2.len(), 1);
        assert_eq!(out2[0].addr, auipc_addr);
        assert_eq!(out2[0].name, b"write".to_vec());
    }

    #[test]
    fn i386_pie_plt_decode() {
        // Real `i386/nl` PIE stub shape: a PLT0 header (`push 0x4(%ebx);
        // jmp *0x8(%ebx)`) then 16-byte entries headed by `jmp *disp(%ebx)`
        // (`FF A3 <disp32>`).  GOT base = `_GLOBAL_OFFSET_TABLE_` = 0x9edc;
        // `_exit@plt` `jmp *0x14(%ebx)` → slot 0x9edc + 0x14 = 0x9ef0 (the
        // R_386_JUMP_SLOT r_offset for `_exit`, verified on the fixture).
        let base = 0x9edcu64;
        let vma = 0xe10u64;
        // PLT0: push 0x4(%ebx) = FF B3 04000000 ; jmp *0x8(%ebx) = FF A3 08000000.
        let mut data: Vec<u8> = vec![0xFF, 0xB3, 0x04, 0x00, 0x00, 0x00, 0xFF, 0xA3, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        // entry @ vma+0x30 (0xe40): `jmp *0x14(%ebx)` then push/jmp padding.
        let entry_off = 0x30usize;
        data.resize(entry_off, 0x00);
        data.extend_from_slice(&[0xFF, 0xA3, 0x14, 0x00, 0x00, 0x00]); // jmp *0x14(%ebx)
        data.extend_from_slice(&[0x68, 0x10, 0x00, 0x00, 0x00]); // push $0x10
        data.extend_from_slice(&[0xE9, 0xc0, 0xff, 0xff, 0xff]); // jmp .plt

        let entry = vma + entry_off as u64; // 0xe40
        let slot = base + 0x14; // 0x9ef0

        let mut map: HashMap<u64, Vec<u8>> = HashMap::new();
        map.insert(slot, b"_exit".to_vec());
        // The PLT0 resolver slot (base+8 = 0x9ee4) is NOT a symbol-bearing reloc,
        // so it must fall out of the map (self-correcting cross-check).

        // With the GOT base (option on): the stub at its entry is named `_exit`.
        let mut out: Vec<PltSym> = Vec::new();
        let mut named: HashSet<u64> = HashSet::new();
        decode_i386(vma, &data, Some(base), &map, &mut out, &mut named);
        assert_eq!(out.len(), 1, "exactly the symbol-bearing PIE stub is named");
        assert_eq!(out[0].addr, entry);
        assert_eq!(out[0].name, b"_exit".to_vec());

        // Option OFF (i386_got_base = None): the `FF A3` form is skipped entirely
        // (byte-identical to the pre-fix behavior — no PIE stub is named).
        let mut out_off: Vec<PltSym> = Vec::new();
        let mut named_off: HashSet<u64> = HashSet::new();
        decode_i386(vma, &data, None, &map, &mut out_off, &mut named_off);
        assert!(out_off.is_empty(), "FF A3 skipped when GOT base is absent");

        // The non-PIC `FF 25 <abs32>` arm is unaffected by the new param: an
        // absolute stub still resolves with either `i386_got_base` value.
        let abs_slot = 0x404000u64;
        let mut abs = abs_slot.to_le_bytes()[..4].to_vec();
        let mut data25 = vec![0xFF, 0x25];
        data25.append(&mut abs);
        let mut map25: HashMap<u64, Vec<u8>> = HashMap::new();
        map25.insert(abs_slot, b"puts".to_vec());
        let mut out25: Vec<PltSym> = Vec::new();
        let mut named25: HashSet<u64> = HashSet::new();
        decode_i386(0x500, &data25, None, &map25, &mut out25, &mut named25);
        assert_eq!(out25.len(), 1);
        assert_eq!(out25[0].name, b"puts".to_vec());
    }

    #[test]
    fn sparc_plt_decode() {
        // Real `plt_sparc64` bytes: a 4-slot (0x80-byte) reserved PLT0 header,
        // then 32-byte import veneers each headed by `sethi %hi(...),%g1` (BE
        // 0x03xxxxxx) + `b,a %xcc,<resolver>`.  On SPARC the JMP_SLOT relocation
        // offset IS the entry address, so the name map is keyed on the entry vma.
        let vma = 0x202100u64;
        let entry = |i: usize| vma + 0x80 + (i as u64) * 32;
        let veneer = |hi: u32, ba: u32| -> [u8; 32] {
            let mut e = [0u8; 32];
            e[0..4].copy_from_slice(&hi.to_be_bytes()); // sethi %hi(...),%g1
            e[4..8].copy_from_slice(&ba.to_be_bytes()); // b,a %xcc,<resolver>
            // remaining 6 words are `nop` (0x01000000); zeros are fine for decode.
            e
        };

        let mut data = vec![0u8; 0x80]; // reserved PLT0 header (4 slots)
        data.extend_from_slice(&veneer(0x03000080, 0x306fffe7)); // __libc_start_main@plt
        data.extend_from_slice(&veneer(0x030000a0, 0x306fffdf)); // printf@plt
        data.extend_from_slice(&veneer(0x030000c0, 0x306fffd7)); // puts@plt

        // Only printf/puts are symbol-bearing relocations; __libc_start_main is
        // present too but here we name just the two libc imports the e2e asserts.
        let mut map: HashMap<u64, Vec<u8>> = HashMap::new();
        map.insert(entry(1), b"printf".to_vec());
        map.insert(entry(2), b"puts".to_vec());

        let mut out: Vec<PltSym> = Vec::new();
        let mut named: HashSet<u64> = HashSet::new();
        decode_sparc(vma, &data, &map, &mut out, &mut named);

        // The PLT0 header and the unmapped entry(0) fall out; printf+puts named
        // at their own entry addresses (stub == reloc offset).
        assert_eq!(out.len(), 2);
        let mut got: Vec<(u64, Vec<u8>)> = out.iter().map(|p| (p.addr, p.name.clone())).collect();
        got.sort();
        assert_eq!(got, vec![(entry(1), b"printf".to_vec()), (entry(2), b"puts".to_vec())]);
    }
}
