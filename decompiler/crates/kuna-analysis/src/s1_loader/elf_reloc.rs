//! (kuna) ET_REL relocatable-object (`.o`) load-layout + relocation engine —
//! the loader-tier capability angr's CLE `ELF` backend has and kuna lacked.
//!
//! ## Why this exists
//!
//! A linked ELF (`ET_EXEC`/`ET_DYN`) carries `PT_LOAD` program headers, so the
//! faithful [`crate::loadimage_object`] backend builds its byte map from
//! `file.segments()`.  A **relocatable object** (`ET_REL`, the `.o` a compiler
//! emits before link) has *no program headers* — `readelf -l ptx.o` →
//! "There are no program headers in this file".  `file.segments()` is therefore
//! empty, the byte map is empty, and **every** function fails to lift
//! (`Unable to load N bytes at ...`).  angr's CLE loader instead lays the
//! `SHF_ALLOC` sections out at a synthetic base, applies the `.rela.*`
//! relocations, and binds the symbols, so a `.o` decompiles fully.  This module
//! reproduces exactly that, for the x86-64 relocations the corpus uses.
//!
//! ## What it does (the angr CLE `ET_REL` path, distilled)
//!
//! 1. **Section layout.** Each `SHF_ALLOC` section (`.text`, `.text.startup`,
//!    `.rodata*`, `.data*`, `.bss`, …) is assigned a non-overlapping load VMA
//!    above [`RELOC_BASE`] (`0x400000`, matching angr's default so the testcase's
//!    `0x400660` lines up), respecting each section's alignment.  `PROGBITS`
//!    bytes are snapshotted; `NOBITS` (`.bss`) is zero-filled.
//! 2. **Relocations.** For each laid-out section, every relocation that applies
//!    to it (`.rela.<sec>`) is resolved and patched into the snapshotted bytes:
//!    `S + A - P` for PC-relative (`R_X86_64_PC32`/`PLT32`), `S + A` for absolute
//!    (`R_X86_64_64`/`32`/`32S`).  An unhandled kind degrades with a warning and a
//!    skip — never a panic or a silent miscompile.
//! 3. **Symbol rebasing + externs.** Each defined function symbol is shifted from
//!    its section-relative `st_value` to `section_load_vma + st_value`; each
//!    *undefined* referenced symbol (an external like `xmalloc`/`strlen`) is
//!    assigned a synthetic address in an "extern" area above the laid-out
//!    sections (the angr extern-object analog) and, when it is a function,
//!    registered as a function symbol so the call renders by name.
//!
//! The result is fed back into [`crate::loadimage_object::ObjectLoadImage`] as the
//! same `(segments, sections, funcsyms)` triple the linked path produces, so the
//! rest of the loader (`load_fill`, `getNextSection`, `getReadonly`,
//! `getNextSymbol`) is unchanged.

use std::collections::HashMap;

use object::read::{Object, ObjectSection, ObjectSymbol};
use object::{
    RelocationKind, RelocationTarget, SectionIndex, SectionKind, SymbolIndex, SymbolKind,
    SymbolSection,
};

use super::format::ObjectFormat;

/// Synthetic load base for the first `SHF_ALLOC` section (angr's CLE default for
/// a relocatable object, so a `.o` lifted by kuna and by angr share addresses).
pub const RELOC_BASE: u64 = 0x40_0000;

/// `SHF_ALLOC` — the section-header flag marking a section that occupies memory
/// at run time (the ones we lay out).
const SHF_ALLOC: u64 = 0x2;

/// The laid-out image of a relocatable object: the same three streams the linked
/// `PT_LOAD` path produces, ready to drop into `ObjectLoadImage`.
pub struct RelocLayout {
    /// Loadable regions `(vma, bytes)` — one per laid-out `SHF_ALLOC` section,
    /// relocations already applied.  Feeds `ObjectLoadImage::segments`.
    pub segments: Vec<(u64, Vec<u8>)>,
    /// `(vma, size, flags)` per laid-out section, for the `getNextSection` /
    /// `getReadonly` info walks.  Feeds `ObjectLoadImage::sections`.
    pub sections: Vec<(u64, u64, u32)>,
    /// `(addr, name)` function symbols — defined functions rebased to their load
    /// VMA plus each external call target.  Feeds `ObjectLoadImage::funcsyms`
    /// (demangle + dedup happen there, as on the linked path).
    pub funcsyms: Vec<(u64, Vec<u8>)>,
    /// Non-fatal diagnostics (an unhandled relocation kind, an unresolved
    /// symbol).  The caller logs these; the load still succeeds.
    pub warnings: Vec<String>,
}

/// One laid-out `SHF_ALLOC` section: its load VMA and a *mutable* byte buffer the
/// relocation pass patches in place.
struct LaidSection {
    index: SectionIndex,
    vma: u64,
    size: u64,
    flags: u32,
    data: Vec<u8>,
}

/// Lay a relocatable object (`ET_REL`) out into a loadable image: synthesize the
/// section layout, apply the `.rela.*` relocations, and rebase / extern-bind the
/// symbols.  The kuna analog of angr CLE's `ELF` relocatable backend.
///
/// `fmt` supplies the per-format section-flag translation (`section_bits`),
/// identical to the linked path, so `.rodata` lands read-only (string literals
/// fold) and `.text` lands as code.
pub fn layout_relocatable(
    file: &object::File,
    fmt: &dyn ObjectFormat,
) -> RelocLayout {
    let mut warnings: Vec<String> = Vec::new();

    // --- Pass 1: assign a load VMA to every SHF_ALLOC section, snapshot bytes.
    let mut laid: Vec<LaidSection> = Vec::new();
    let mut vma_of: HashMap<SectionIndex, u64> = HashMap::new();
    let mut cursor = RELOC_BASE;
    for sec in file.sections() {
        let sh_flags = match sec.flags() {
            object::SectionFlags::Elf { sh_flags } => sh_flags,
            _ => 0,
        };
        if sh_flags & SHF_ALLOC == 0 {
            continue; // not mapped at run time (.rela.*, .symtab, .debug_*, …)
        }
        let align = sec.align().max(1);
        let vma = align_up(cursor, align);
        let size = sec.size();
        // PROGBITS bytes verbatim; NOBITS (.bss) zero-filled to its RAM size.
        let data: Vec<u8> = if matches!(sec.kind(), SectionKind::UninitializedData) {
            vec![0u8; size as usize]
        } else {
            match sec.data() {
                Ok(d) => {
                    let mut v = d.to_vec();
                    // A short file extent (rare) is zero-padded to the RAM size.
                    if (v.len() as u64) < size {
                        v.resize(size as usize, 0);
                    }
                    v
                }
                Err(_) => vec![0u8; size as usize],
            }
        };
        vma_of.insert(sec.index(), vma);
        laid.push(LaidSection {
            index: sec.index(),
            vma,
            size,
            flags: fmt.section_bits(sec.kind(), sec.flags()),
            data,
        });
        cursor = vma.wrapping_add(size);
    }

    // Externs (undefined referenced symbols) live in a synthetic area above the
    // laid-out sections — the angr extern-object analog.  Allocated on demand.
    let mut extern_cursor = align_up(cursor.wrapping_add(0x1000), 0x1000);
    let mut extern_of: HashMap<SymbolIndex, u64> = HashMap::new();
    let mut extern_order: Vec<(SymbolIndex, u64)> = Vec::new();
    // Externs reached through a PLT-relative relocation (`R_X86_64_PLT32`) are
    // definitely *call targets* — even when the undefined symbol is `STT_NOTYPE`
    // (object `SymbolKind::Unknown`), which gcc emits for plain `extern` calls.
    // Used to register them as function symbols so the call renders by name.
    let mut plt_externs: std::collections::HashSet<SymbolIndex> = std::collections::HashSet::new();

    // --- Pass 2: apply relocations, patching each laid-out section's bytes.
    for li in 0..laid.len() {
        let sec_index = laid[li].index;
        let sec_vma = laid[li].vma;
        let Ok(sec) = file.section_by_index(sec_index) else { continue };
        // Collect first so the `laid` buffer can be borrowed mutably below.
        let relocs: Vec<(u64, object::Relocation)> = sec.relocations().collect();
        for (offset, reloc) in relocs {
            let size_bits = reloc.size();
            if size_bits != 32 && size_bits != 64 {
                warnings.push(format!(
                    "reloc at {sec_vma:#x}+{offset:#x}: unhandled size {size_bits} bits (skipped)"
                ));
                continue;
            }
            // S — the resolved value of the relocation's target.
            let s = match reloc.target() {
                RelocationTarget::Symbol(sym_idx) => {
                    // A PLT-relative reloc marks its target as a call target.
                    if matches!(reloc.kind(), RelocationKind::PltRelative) {
                        plt_externs.insert(sym_idx);
                    }
                    resolve_symbol(
                        file,
                        sym_idx,
                        &vma_of,
                        &mut extern_of,
                        &mut extern_order,
                        &mut extern_cursor,
                    )
                }
                RelocationTarget::Section(sidx) => vma_of.get(&sidx).copied(),
                _ => None,
            };
            let Some(s) = s else {
                warnings.push(format!(
                    "reloc at {sec_vma:#x}+{offset:#x}: unresolved target (skipped)"
                ));
                continue;
            };
            let a = reloc.addend() as i128;
            let p = sec_vma.wrapping_add(offset) as i128;
            // `XxxRelative` = S + A - P; `Absolute` = S + A (object's documented
            // semantics, read/common.rs).
            let value: i128 = match reloc.kind() {
                RelocationKind::Absolute => s as i128 + a,
                RelocationKind::Relative | RelocationKind::PltRelative => s as i128 + a - p,
                other => {
                    warnings.push(format!(
                        "reloc at {sec_vma:#x}+{offset:#x}: unhandled kind {other:?} (skipped)"
                    ));
                    continue;
                }
            };
            let nbytes = (size_bits / 8) as usize;
            let off = offset as usize;
            let buf = &mut laid[li].data;
            if off + nbytes > buf.len() {
                warnings.push(format!(
                    "reloc at {sec_vma:#x}+{offset:#x}: past section end (skipped)"
                ));
                continue;
            }
            let le = (value as u64).to_le_bytes();
            buf[off..off + nbytes].copy_from_slice(&le[..nbytes]);
        }
    }

    // --- Function symbols: defined (rebased) + extern call targets.
    let mut funcsyms: Vec<(u64, Vec<u8>)> = Vec::new();
    for sym in file.symbols() {
        if sym.kind() != SymbolKind::Text {
            continue;
        }
        let SymbolSection::Section(sec_idx) = sym.section() else { continue };
        let Some(&base) = vma_of.get(&sec_idx) else { continue };
        let name = match sym.name_bytes() {
            Ok(n) if !n.is_empty() => n.to_vec(),
            _ => continue,
        };
        funcsyms.push((base.wrapping_add(sym.address()), name));
    }
    // Extern functions: name each external *call target* at its synthetic slot
    // so `call ext` renders by name instead of a bare address. A call target is
    // an undefined symbol reached through a PLT-relative reloc (reliable even for
    // the `STT_NOTYPE` symbols gcc emits for `extern` calls) or one typed
    // `STT_FUNC`. Pure data externs (e.g. `stdout`, referenced by PC32 to an
    // `STT_OBJECT`) are addressed but deliberately left unnamed here.
    for (sym_idx, addr) in &extern_order {
        let Ok(sym) = file.symbol_by_index(*sym_idx) else { continue };
        let is_func = plt_externs.contains(sym_idx) || sym.kind() == SymbolKind::Text;
        if !is_func {
            continue;
        }
        if let Ok(n) = sym.name_bytes() {
            if !n.is_empty() {
                funcsyms.push((*addr, n.to_vec()));
            }
        }
    }

    let segments = laid.iter().map(|l| (l.vma, l.data.clone())).collect();
    let sections = laid.iter().map(|l| (l.vma, l.size, l.flags)).collect();
    RelocLayout { segments, sections, funcsyms, warnings }
}

/// Resolve a relocation's target symbol to a load address.  A defined symbol maps
/// to `section_load_vma + st_value`; an *undefined* symbol is bound to a fresh
/// synthetic extern slot (allocated on first reference and reused after).
/// Absolute symbols pass through.  `None` for a symbol in a non-laid section
/// (e.g. a `.debug_*`-only symbol) — the caller warns and skips.
fn resolve_symbol(
    file: &object::File,
    idx: SymbolIndex,
    vma_of: &HashMap<SectionIndex, u64>,
    extern_of: &mut HashMap<SymbolIndex, u64>,
    extern_order: &mut Vec<(SymbolIndex, u64)>,
    extern_cursor: &mut u64,
) -> Option<u64> {
    let sym = file.symbol_by_index(idx).ok()?;
    match sym.section() {
        SymbolSection::Section(sec_idx) => {
            vma_of.get(&sec_idx).map(|base| base.wrapping_add(sym.address()))
        }
        SymbolSection::Undefined | SymbolSection::Common => {
            if let Some(&a) = extern_of.get(&idx) {
                return Some(a);
            }
            let a = *extern_cursor;
            *extern_cursor = extern_cursor.wrapping_add(16);
            extern_of.insert(idx, a);
            extern_order.push((idx, a));
            Some(a)
        }
        SymbolSection::Absolute => Some(sym.address()),
        _ => None,
    }
}

/// Round `value` up to a multiple of `align` (a power of two ≥ 1).
fn align_up(value: u64, align: u64) -> u64 {
    debug_assert!(align >= 1);
    let mask = align - 1;
    value.wrapping_add(mask) & !mask
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::s1_loader::format::elf::ElfFormat;

    /// Hand-assemble a minimal little-endian ELF64 **ET_REL** (no program
    /// headers) with one `.text` (ALLOC|EXEC), one `.data` (ALLOC|WRITE), a
    /// `.symtab` (`myfunc`@.text, `datum`@.data, undefined `ext_call`), and a
    /// `.rela.text` exercising one of each relocation kind the corpus uses:
    ///   off 4  : R_X86_64_PC32  -> datum   (addend -4)
    ///   off 10 : R_X86_64_PLT32 -> ext_call(addend -4)
    ///   off 16 : R_X86_64_64    -> datum   (addend  0)
    ///   off 24 : R_X86_64_32    -> datum   (addend  0)
    /// Needs no external toolchain.
    fn build_et_rel() -> Vec<u8> {
        // Section header indices (must match shndx / link / info below):
        //   0 null, 1 .text, 2 .data, 3 .symtab, 4 .strtab, 5 .rela.text, 6 .shstrtab
        const TEXT: usize = 32;
        let text = vec![0u8; TEXT];
        let data = vec![0u8; 8];

        // --- .strtab (symbol names) -------------------------------------
        let mut strtab = vec![0u8];
        let push_str = |s: &mut Vec<u8>, n: &str| {
            let off = s.len() as u32;
            s.extend_from_slice(n.as_bytes());
            s.push(0);
            off
        };
        let n_myfunc = push_str(&mut strtab, "myfunc");
        let n_datum = push_str(&mut strtab, "datum");
        let n_extcall = push_str(&mut strtab, "ext_call");

        // --- .symtab (Elf64_Sym = 24 bytes) -----------------------------
        let mut symtab: Vec<u8> = vec![0u8; 24]; // [0] null
        let push_sym = |s: &mut Vec<u8>, name: u32, info: u8, shndx: u16, value: u64| {
            s.extend_from_slice(&name.to_le_bytes()); // st_name
            s.push(info); // st_info
            s.push(0); // st_other
            s.extend_from_slice(&shndx.to_le_bytes()); // st_shndx
            s.extend_from_slice(&value.to_le_bytes()); // st_value
            s.extend_from_slice(&0u64.to_le_bytes()); // st_size
        };
        // STB_GLOBAL<<4 | STT_FUNC(2) = 0x12 ; STT_OBJECT(1) = 0x11
        push_sym(&mut symtab, n_myfunc, 0x12, 1, 0); // [1] myfunc @ .text+0
        push_sym(&mut symtab, n_datum, 0x11, 2, 0); // [2] datum  @ .data+0
        push_sym(&mut symtab, n_extcall, 0x12, 0, 0); // [3] ext_call UNDEF

        // --- .rela.text (Elf64_Rela = 24 bytes) -------------------------
        let mut rela: Vec<u8> = Vec::new();
        let push_rela = |s: &mut Vec<u8>, off: u64, sym: u64, ty: u64, addend: i64| {
            s.extend_from_slice(&off.to_le_bytes());
            s.extend_from_slice(&((sym << 32) | ty).to_le_bytes());
            s.extend_from_slice(&addend.to_le_bytes());
        };
        push_rela(&mut rela, 4, 2, 2, -4); // PC32  datum
        push_rela(&mut rela, 10, 3, 4, -4); // PLT32 ext_call
        push_rela(&mut rela, 16, 2, 1, 0); // 64    datum
        push_rela(&mut rela, 24, 2, 10, 0); // 32    datum

        // --- .shstrtab (section names) ----------------------------------
        let mut shstr = vec![0u8];
        let s_text = push_str(&mut shstr, ".text");
        let s_data = push_str(&mut shstr, ".data");
        let s_symtab = push_str(&mut shstr, ".symtab");
        let s_strtab = push_str(&mut shstr, ".strtab");
        let s_rela = push_str(&mut shstr, ".rela.text");
        let s_shstr = push_str(&mut shstr, ".shstrtab");

        // --- file body offsets ------------------------------------------
        let mut buf: Vec<u8> = Vec::new();
        let ehdr_size = 64u64;
        buf.resize(ehdr_size as usize, 0); // Ehdr filled in last
        let off_text = buf.len() as u64;
        buf.extend_from_slice(&text);
        let off_data = buf.len() as u64;
        buf.extend_from_slice(&data);
        let off_symtab = buf.len() as u64;
        buf.extend_from_slice(&symtab);
        let off_strtab = buf.len() as u64;
        buf.extend_from_slice(&strtab);
        let off_rela = buf.len() as u64;
        buf.extend_from_slice(&rela);
        let off_shstr = buf.len() as u64;
        buf.extend_from_slice(&shstr);
        // align section headers to 8
        while buf.len() % 8 != 0 {
            buf.push(0);
        }
        let sh_off = buf.len() as u64;

        // --- section headers (Elf64_Shdr = 64 bytes) --------------------
        #[allow(clippy::too_many_arguments)]
        let push_shdr = |b: &mut Vec<u8>,
                         name: u32,
                         sh_type: u32,
                         sh_flags: u64,
                         addr: u64,
                         offset: u64,
                         size: u64,
                         link: u32,
                         info: u32,
                         align: u64,
                         entsize: u64| {
            b.extend_from_slice(&name.to_le_bytes());
            b.extend_from_slice(&sh_type.to_le_bytes());
            b.extend_from_slice(&sh_flags.to_le_bytes());
            b.extend_from_slice(&addr.to_le_bytes());
            b.extend_from_slice(&offset.to_le_bytes());
            b.extend_from_slice(&size.to_le_bytes());
            b.extend_from_slice(&link.to_le_bytes());
            b.extend_from_slice(&info.to_le_bytes());
            b.extend_from_slice(&align.to_le_bytes());
            b.extend_from_slice(&entsize.to_le_bytes());
        };
        // SHF_WRITE=1, SHF_ALLOC=2, SHF_EXECINSTR=4, SHF_INFO_LINK=0x40
        push_shdr(&mut buf, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0); // [0] null
        push_shdr(&mut buf, s_text, 1, 0x2 | 0x4, 0, off_text, TEXT as u64, 0, 0, 16, 0); // [1] .text
        push_shdr(&mut buf, s_data, 1, 0x2 | 0x1, 0, off_data, 8, 0, 0, 8, 0); // [2] .data
        push_shdr(&mut buf, s_symtab, 2, 0, 0, off_symtab, symtab.len() as u64, 4, 1, 8, 24); // [3] .symtab
        push_shdr(&mut buf, s_strtab, 3, 0, 0, off_strtab, strtab.len() as u64, 0, 0, 1, 0); // [4] .strtab
        push_shdr(&mut buf, s_rela, 4, 0x40, 0, off_rela, rela.len() as u64, 3, 1, 8, 24); // [5] .rela.text
        push_shdr(&mut buf, s_shstr, 3, 0, 0, off_shstr, shstr.len() as u64, 0, 0, 1, 0); // [6] .shstrtab

        // --- Ehdr -------------------------------------------------------
        let mut e: Vec<u8> = Vec::new();
        e.extend_from_slice(&[0x7f, b'E', b'L', b'F']);
        e.push(2); // ELFCLASS64
        e.push(1); // ELFDATA2LSB
        e.push(1); // version
        e.extend_from_slice(&[0u8; 9]); // OSABI + pad
        e.extend_from_slice(&1u16.to_le_bytes()); // e_type = ET_REL
        e.extend_from_slice(&62u16.to_le_bytes()); // e_machine = EM_X86_64
        e.extend_from_slice(&1u32.to_le_bytes()); // e_version
        e.extend_from_slice(&0u64.to_le_bytes()); // e_entry
        e.extend_from_slice(&0u64.to_le_bytes()); // e_phoff = 0 (no phdrs)
        e.extend_from_slice(&sh_off.to_le_bytes()); // e_shoff
        e.extend_from_slice(&0u32.to_le_bytes()); // e_flags
        e.extend_from_slice(&64u16.to_le_bytes()); // e_ehsize
        e.extend_from_slice(&0u16.to_le_bytes()); // e_phentsize
        e.extend_from_slice(&0u16.to_le_bytes()); // e_phnum
        e.extend_from_slice(&64u16.to_le_bytes()); // e_shentsize
        e.extend_from_slice(&7u16.to_le_bytes()); // e_shnum
        e.extend_from_slice(&6u16.to_le_bytes()); // e_shstrndx
        buf[..64].copy_from_slice(&e[..64]);
        buf
    }

    fn le32(b: &[u8]) -> u32 {
        u32::from_le_bytes([b[0], b[1], b[2], b[3]])
    }
    fn le64(b: &[u8]) -> u64 {
        let mut a = [0u8; 8];
        a.copy_from_slice(&b[..8]);
        u64::from_le_bytes(a)
    }

    #[test]
    fn et_rel_layout_relocations_and_symbols() {
        let bytes = build_et_rel();
        let file = object::File::parse(&*bytes).expect("parse ET_REL");
        assert_eq!(file.kind(), object::ObjectKind::Relocatable);
        assert!(file.segments().next().is_none(), "ET_REL has no PT_LOAD");

        let layout = layout_relocatable(&file, &ElfFormat);
        assert!(layout.warnings.is_empty(), "unexpected warnings: {:?}", layout.warnings);

        // .text is the first SHF_ALLOC section -> RELOC_BASE; .data follows it.
        let (text_vma, text) = layout
            .segments
            .iter()
            .find(|(v, _)| *v == RELOC_BASE)
            .map(|(v, d)| (*v, d.clone()))
            .expect(".text at RELOC_BASE");
        assert_eq!(text_vma, 0x40_0000);
        let data_vma = layout
            .segments
            .iter()
            .map(|(v, _)| *v)
            .find(|v| *v != RELOC_BASE)
            .expect(".data laid out");
        assert!(data_vma > text_vma, ".data above .text");

        // Defined function `myfunc` rebased to .text load VMA; `datum` (data) is
        // NOT a function symbol; extern `ext_call` bound above the sections.
        let func = |name: &str| {
            layout
                .funcsyms
                .iter()
                .find(|(_, n)| n == name.as_bytes())
                .map(|(a, _)| *a)
        };
        assert_eq!(func("myfunc"), Some(text_vma), "myfunc rebased");
        assert_eq!(func("datum"), None, "data symbol is not a funcsym");
        let ext = func("ext_call").expect("ext_call bound");
        assert!(ext >= data_vma, "extern above the laid-out sections");

        // PC32 @4 : S + A - P = data_vma - 4 - (text_vma+4)
        assert_eq!(
            le32(&text[4..8]) as i32,
            (data_vma as i64 - 4 - (text_vma as i64 + 4)) as i32
        );
        // PLT32 @10: ext - 4 - (text_vma+10)
        assert_eq!(
            le32(&text[10..14]) as i32,
            (ext as i64 - 4 - (text_vma as i64 + 10)) as i32
        );
        // R_X86_64_64 @16 : S + A = data_vma (8 bytes)
        assert_eq!(le64(&text[16..24]), data_vma);
        // R_X86_64_32 @24 : S + A = data_vma (4 bytes)
        assert_eq!(le32(&text[24..28]) as u64, data_vma);
    }
}
