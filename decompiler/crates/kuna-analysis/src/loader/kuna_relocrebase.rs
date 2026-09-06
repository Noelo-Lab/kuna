//! (kuna `relocrebase`) Rebase the LOAD-TIME analysis facts of a relocatable
//! object into the loaded image's address space.
//!
//! ## The bug this closes (GH-289)
//!
//! [`crate::loader::reloc_object`] lays a relocatable object (an ELF `ET_REL`
//! `.o`, a COFF `.obj`) out synthetically above [`RELOC_BASE`], applies its
//! relocations and rebases its symbols, so **every address the engine holds is a
//! post-layout one**. The analyzer tier, however, re-parses the same file through
//! its own `object::File` and therefore computes **pre-link, section-relative**
//! addresses. The two spaces then mix in one inventory:
//!
//! - `kuna functions ptx.o` reported 26 real functions at `0x400000+` beside 27
//!   phantoms at `0x0`, `0x20`, `0x34`, … — the `.eh_frame` FDE oracle's
//!   *unrelocated* `initial_location` fields (a PC-relative field in a `.o` reads
//!   back as its own section offset) plus one DWARF `DW_AT_low_pc` that reads 0
//!   because its relocation was never applied. `decompile-all` filters to the
//!   loader's CODE sections and so reported the correct 26 — the two surfaces
//!   disagreed on a `.o`, violating the invariant DIV-68 established.
//! - Silently, every string literal and DWARF-named global was keyed to a
//!   pre-link address and never attached to the loaded image at all.
//!
//! ## Why "rebase", not "decline"
//!
//! Declining (what the Listing/xref tier does — `run_listing_consumers`) would
//! throw real information away: a `.o` compiled `-g` carries full DWARF. So this
//! module rebases instead.
//!
//! ## How: rebase the INPUT, not each output fact
//!
//! A fact is a bare `u64` by the time it reaches [`AnalysisOutput`], and in a
//! relocatable object **every** section sits at address 0 — `.text`+0x20 and
//! `.rodata`+0x20 are the same number — so a post-hoc rebase cannot tell which
//! section's delta to apply. Worse, the interesting fields (a `.eh_frame`
//! `initial_location`, a `.debug_info` `DW_AT_low_pc`, a `DW_FORM_strp` offset)
//! are not offsets at all until their relocation is applied.
//!
//! So the rebase is applied to the analyzer tier's **input**: a patched copy of
//! the image bytes in which
//!
//! 1. every laid-out section's contents are replaced by the loader's own
//!    relocated bytes (so `.eh_frame` and `.text` read exactly as the engine
//!    decodes them),
//! 2. every *non*-laid-out section that carries relocations (`.debug_info`,
//!    `.debug_line`, …) has them applied here, resolving a target in a laid-out
//!    section to its load VMA and a target in another debug section to its own
//!    section-relative offset (`S = 0`, which is what a single-object link
//!    produces),
//! 3. each laid-out section's address field (ELF `sh_addr`, COFF
//!    `VirtualAddress`) is set to its load VMA, and
//! 4. each ELF symbol defined in a laid-out section has `st_value` shifted by
//!    that section's VMA (a COFF symbol needs no patch — `object` reports it as
//!    `section.virtual_address + value`, so step 3 already moved it).
//!
//! Every pass then reads a coherent, post-layout address space with no source
//! change, and each fact is produced already rebased through its OWN section's
//! delta. Sections are laid out non-contiguously (alignment padding between them,
//! and the empty-section skip), so there is no single global offset — the map is
//! per-section, [`RelocLayout::section_vma`].
//!
//! ## The safety net
//!
//! A field with no relocation (a hand-written `.eh_frame`, a `SHN_COMMON`
//! symbol whose `st_value` is an alignment, a symbol in a discarded section)
//! still yields an address in no laid-out section. [`retain_in_image`] drops
//! exactly those — the phantom class — rather than letting them through
//! unrebased. The one documented exception is a [`NoReturnFact`], which the
//! commit boundary resolves by NAME when its address does not resolve (an
//! undefined `exit` in a `.o` has always had address 0); it is retained with its
//! address zeroed so the name path still fires.
//!
//! Gated by `--option relocrebase on|off` (default **ON**, DIV-79) through the
//! [`kuna_decomp::kuna_relocrebase`] env bridge — the analyzer tier runs inside
//! `load file`, upstream of the per-function option machinery.
//!
//! [`RELOC_BASE`]: crate::loader::reloc_object::RELOC_BASE
//! [`AnalysisOutput`]: crate::pass::AnalysisOutput
//! [`NoReturnFact`]: crate::pass::NoReturnFact
//! [`RelocLayout::section_vma`]: crate::loader::reloc_object::RelocLayout::section_vma

use std::collections::HashMap;

use object::read::{Object, ObjectSection, ObjectSymbol};
use object::{RelocationKind, RelocationTarget, SectionIndex, SymbolSection};

use crate::loader::reloc_object::{self, RelocLayout};
use crate::pass::AnalysisOutput;

/// A relocatable object re-presented in the loaded image's address space.
pub struct RebasedView {
    /// The patched image bytes the analyzer tier parses instead of the original.
    pub bytes: Vec<u8>,
    /// Every `[lo, hi)` extent that exists in the loaded image — one per laid-out
    /// section plus the synthetic extern area. A fact outside all of them is a
    /// phantom (see [`retain_in_image`]).
    pub ranges: Vec<(u64, u64)>,
}

impl RebasedView {
    /// Does `addr` lie in the loaded image?
    pub fn contains(&self, addr: u64) -> bool {
        self.ranges.iter().any(|&(lo, hi)| addr >= lo && addr < hi)
    }
}

/// Build the rebased analyzer-tier view of `file`, or `None` when this object is
/// not synthetically laid out, the `relocrebase` gate is off, or the image's
/// headers cannot be patched (in which case the caller keeps today's behavior).
pub fn rebased_view(file: &object::File, bytes: &[u8]) -> Option<RebasedView> {
    if !kuna_decomp::kuna_relocrebase::relocrebase_enabled() {
        return None;
    }
    if !reloc_object::is_synthetically_laid_out(file) {
        return None;
    }
    let fmt = crate::loader::format::detect(file).ok()?;
    let layout = reloc_object::layout_relocatable(file, fmt.as_ref());
    if layout.section_vma.is_empty() {
        return None; // nothing was laid out; the loader itself has no image
    }

    let mut out = bytes.to_vec();
    splice_laid_sections(file, &layout, &mut out);
    relocate_unlaid_sections(file, &layout, &mut out);
    patch_section_addresses(file, &layout, &mut out)?;

    let mut ranges: Vec<(u64, u64)> =
        layout.sections.iter().map(|&(vma, size, _)| (vma, vma.wrapping_add(size))).collect();
    if let Some((lo, hi)) = layout.extern_range {
        ranges.push((lo, hi));
    }
    Some(RebasedView { bytes: out, ranges })
}

/// Pick the view the analyzer tier reads: the rebased one when [`rebased_view`]
/// produced it (and it still parses), else the raw pre-link one unchanged.
pub fn select<'a>(
    raw: object::File<'a>,
    bytes: &'a [u8],
    view: &'a Option<RebasedView>,
) -> (object::File<'a>, &'a [u8]) {
    match view {
        Some(v) => match crate::loadimage_object::parse_object(&*v.bytes) {
            Ok(file) => (file, &v.bytes[..]),
            Err(_) => (raw, bytes),
        },
        None => (raw, bytes),
    }
}

/// Replace each laid-out section's file bytes with the loader's own relocated
/// copy, so the analyzer tier decodes exactly the bytes the engine does.
/// `NOBITS`/uninitialized sections have no file extent and are skipped.
fn splice_laid_sections(file: &object::File, layout: &RelocLayout, out: &mut [u8]) {
    let by_vma: HashMap<u64, &Vec<u8>> =
        layout.segments.iter().map(|(vma, data)| (*vma, data)).collect();
    for sec in file.sections() {
        let Some(&vma) = layout.section_vma.get(&sec.index()) else { continue };
        let Some(data) = by_vma.get(&vma) else { continue };
        let Some((off, len)) = sec.file_range() else { continue };
        let off = off as usize;
        let len = (len as usize).min(data.len());
        if off.saturating_add(len) > out.len() {
            continue;
        }
        out[off..off + len].copy_from_slice(&data[..len]);
    }
}

/// Apply the relocations of every section the layout did NOT lay out — the
/// `.debug_*` tables above all. Without this a `.o`'s `DW_AT_low_pc` reads 0 and
/// every `DW_FORM_strp` resolves to `.debug_str`+0, so the DWARF pass names one
/// phantom function after whatever string happens to sit at offset 0.
fn relocate_unlaid_sections(file: &object::File, layout: &RelocLayout, out: &mut [u8]) {
    for sec in file.sections() {
        if layout.section_vma.contains_key(&sec.index()) {
            continue; // already spliced with the loader's relocated bytes
        }
        let Some((sec_off, sec_len)) = sec.file_range() else { continue };
        for (offset, reloc) in sec.relocations() {
            let nbytes = (reloc.size() / 8) as usize;
            if nbytes != 4 && nbytes != 8 {
                continue;
            }
            if offset.saturating_add(nbytes as u64) > sec_len {
                continue;
            }
            let at = (sec_off + offset) as usize;
            if at + nbytes > out.len() {
                continue;
            }
            let Some(s) = resolve_target(file, layout, &reloc) else { continue };
            let a = reloc.addend() as i128
                + if reloc.has_implicit_addend() {
                    implicit_addend(&out[at..at + nbytes])
                } else {
                    0
                };
            // A non-laid-out section has no load VMA, so `P` is its own
            // section-relative position — the same convention `S = 0` uses for a
            // debug-to-debug reference.
            let p = offset as i128;
            let value: i128 = match reloc.kind() {
                RelocationKind::Absolute => s as i128 + a,
                RelocationKind::Relative | RelocationKind::PltRelative => s as i128 + a - p,
                _ => continue,
            };
            let le = (value as u64).to_le_bytes();
            out[at..at + nbytes].copy_from_slice(&le[..nbytes]);
        }
    }
}

/// Resolve a relocation target to its address in the rebased space. A symbol in a
/// laid-out section resolves to `section_vma + st_value`; an undefined symbol to
/// its already-assigned extern slot; a symbol in a section the layout skipped
/// (every `.debug_*` table) resolves to `0 + st_value`, which is exactly what a
/// single-object link leaves in place.
fn resolve_target(
    file: &object::File,
    layout: &RelocLayout,
    reloc: &object::Relocation,
) -> Option<u64> {
    match reloc.target() {
        RelocationTarget::Symbol(idx) => {
            let sym = file.symbol_by_index(idx).ok()?;
            match sym.section() {
                SymbolSection::Section(sec_idx) => Some(
                    layout
                        .section_vma
                        .get(&sec_idx)
                        .copied()
                        .unwrap_or(0)
                        .wrapping_add(sym.address()),
                ),
                SymbolSection::Undefined | SymbolSection::Common => {
                    layout.extern_addr.get(&idx).copied()
                }
                SymbolSection::Absolute => Some(sym.address()),
                _ => None,
            }
        }
        RelocationTarget::Section(sidx) => Some(layout.section_vma.get(&sidx).copied().unwrap_or(0)),
        _ => None,
    }
}

/// Read the in-place addend of a REL-style relocation (COFF, 32-bit ELF), the
/// twin of `reloc_object`'s helper.
fn implicit_addend(field: &[u8]) -> i128 {
    match field.len() {
        4 => i32::from_le_bytes([field[0], field[1], field[2], field[3]]) as i128,
        8 => i64::from_le_bytes([
            field[0], field[1], field[2], field[3], field[4], field[5], field[6], field[7],
        ]) as i128,
        _ => 0,
    }
}

/// Write each laid-out section's load VMA into its header address field, and (ELF
/// only) shift each defined symbol's `st_value` by its section's VMA. Returns
/// `None` for a header shape this cannot patch, so the caller falls back to the
/// unrebased view rather than emitting a half-patched image.
fn patch_section_addresses(
    file: &object::File,
    layout: &RelocLayout,
    out: &mut [u8],
) -> Option<()> {
    match file.format() {
        object::BinaryFormat::Elf => patch_elf(file, layout, out),
        object::BinaryFormat::Coff => patch_coff(layout, out),
        _ => None,
    }
}

/// The fixed offsets of the ELF header/section-header/symbol fields this patcher
/// writes, resolved for one ELF class.
struct ElfShape {
    /// `sh_addr`'s offset inside a section header.
    sh_addr: usize,
    /// `st_value`'s offset inside a symbol table entry.
    st_value: usize,
    /// Width of both fields (4 on ELF32, 8 on ELF64).
    width: usize,
}

fn patch_elf(file: &object::File, layout: &RelocLayout, out: &mut [u8]) -> Option<()> {
    if out.len() < 64 || out[..4] != [0x7f, b'E', b'L', b'F'] {
        return None;
    }
    let is64 = match out[4] {
        1 => false,
        2 => true,
        _ => return None,
    };
    if out[5] != 1 {
        return None; // big-endian ELF: the field writes below are little-endian
    }
    let shape = if is64 {
        ElfShape { sh_addr: 16, st_value: 8, width: 8 }
    } else {
        ElfShape { sh_addr: 12, st_value: 4, width: 4 }
    };
    let (e_shoff, e_shentsize, e_shnum) = if is64 {
        (read_u64(out, 0x28)?, read_u16(out, 0x3a)? as usize, read_u16(out, 0x3c)? as usize)
    } else {
        (read_u32(out, 0x20)? as u64, read_u16(out, 0x2e)? as usize, read_u16(out, 0x30)? as usize)
    };
    let shoff = usize::try_from(e_shoff).ok()?;
    if e_shentsize < shape.sh_addr + shape.width {
        return None;
    }
    if shoff.checked_add(e_shnum.checked_mul(e_shentsize)?)? > out.len() {
        return None;
    }

    // (3) sh_addr <- the section's load VMA. `object`'s ELF `SectionIndex` is the
    // raw section-header index, so the header sits at a fixed stride.
    for (&SectionIndex(i), &vma) in &layout.section_vma {
        if i >= e_shnum {
            return None;
        }
        write_uint(out, shoff + i * e_shentsize + shape.sh_addr, shape.width, vma)?;
    }

    // (4) st_value <- section VMA + st_value, for every symbol defined in a
    // laid-out section. An undefined/common/absolute symbol is left alone: the
    // engine addresses an extern through the loader's own synthetic slot, and a
    // pass that reads a zero address already treats it as "no address".
    let (sym_off, sym_entsize) = elf_symtab_extent(out, shoff, e_shentsize, e_shnum, is64)?;
    if sym_entsize < shape.st_value + shape.width {
        return None;
    }
    for sym in file.symbols() {
        let SymbolSection::Section(sec_idx) = sym.section() else { continue };
        let Some(&base) = layout.section_vma.get(&sec_idx) else { continue };
        let at = sym_off.checked_add(sym.index().0.checked_mul(sym_entsize)?)? + shape.st_value;
        if at + shape.width > out.len() {
            return None;
        }
        write_uint(out, at, shape.width, base.wrapping_add(sym.address()))?;
    }
    Some(())
}

/// Locate the `SHT_SYMTAB` section's `(file offset, entry size)` — the table
/// `object::File::symbols()` enumerates for an ELF.
fn elf_symtab_extent(
    out: &[u8],
    shoff: usize,
    shentsize: usize,
    shnum: usize,
    is64: bool,
) -> Option<(usize, usize)> {
    const SHT_SYMTAB: u32 = 2;
    for i in 0..shnum {
        let h = shoff + i * shentsize;
        if read_u32(out, h + 4)? != SHT_SYMTAB {
            continue;
        }
        return if is64 {
            Some((usize::try_from(read_u64(out, h + 24)?).ok()?, usize::try_from(read_u64(out, h + 56)?).ok()?))
        } else {
            Some((read_u32(out, h + 16)? as usize, read_u32(out, h + 36)? as usize))
        };
    }
    // No symbol table at all: nothing to shift, which is not a failure.
    Some((0, usize::MAX))
}

/// COFF: write each laid-out section's load VMA into its `VirtualAddress`.
/// `object` reports a COFF symbol as `section.virtual_address + value`, so this
/// single write rebases the symbols too.
fn patch_coff(layout: &RelocLayout, out: &mut [u8]) -> Option<()> {
    const COFF_HEADER: usize = 20;
    const SECTION_HEADER: usize = 40;
    const VIRTUAL_ADDRESS: usize = 12;
    if out.len() < COFF_HEADER {
        return None;
    }
    let machine = read_u16(out, 0)?;
    let nsections = read_u16(out, 2)? as usize;
    if machine == 0 && nsections == 0xffff {
        return None; // ANON_OBJECT_HEADER_BIGOBJ: a different header shape
    }
    let table = COFF_HEADER + read_u16(out, 16)? as usize;
    if table.checked_add(nsections.checked_mul(SECTION_HEADER)?)? > out.len() {
        return None;
    }
    // `object`'s COFF `SectionIndex` is the 1-based section NUMBER.
    for (&SectionIndex(n), &vma) in &layout.section_vma {
        if n == 0 || n > nsections {
            return None;
        }
        let vma = u32::try_from(vma).ok()?;
        write_uint(out, table + (n - 1) * SECTION_HEADER + VIRTUAL_ADDRESS, 4, vma as u64)?;
    }
    Some(())
}

fn read_u16(b: &[u8], at: usize) -> Option<u16> {
    Some(u16::from_le_bytes(b.get(at..at + 2)?.try_into().ok()?))
}

fn read_u32(b: &[u8], at: usize) -> Option<u32> {
    Some(u32::from_le_bytes(b.get(at..at + 4)?.try_into().ok()?))
}

fn read_u64(b: &[u8], at: usize) -> Option<u64> {
    Some(u64::from_le_bytes(b.get(at..at + 8)?.try_into().ok()?))
}

fn write_uint(b: &mut [u8], at: usize, width: usize, value: u64) -> Option<()> {
    let field = b.get_mut(at..at + width)?;
    field.copy_from_slice(&value.to_le_bytes()[..width]);
    Some(())
}

/// Drop every address-keyed fact that lands outside the loaded image — the
/// phantom class a field with no relocation still produces. See the module docs
/// for the one exception (a [`crate::pass::NoReturnFact`] keeps its name, which
/// is the commit's fallback resolution key, and only loses its address).
pub fn retain_in_image(out: &mut AnalysisOutput, view: &RebasedView) {
    let keep = |a: u64| view.contains(a);
    let keep_range = |&(lo, hi): &(u64, u64)| view.contains(lo) && (hi <= lo || view.contains(hi - 1));

    out.symbols.retain(|s| keep(s.addr));
    out.data_objects.retain(|d| keep(d.addr));
    out.entries.retain(|&a| keep(a));
    out.fde_bodies.retain(keep_range);
    out.entry_names.retain(|(a, _)| keep(*a));
    for fact in &mut out.noreturn {
        if !keep(fact.addr) {
            fact.addr = 0;
        }
    }
    out.no_fallthru_calls.retain(|&a| keep(a));
    out.readonly.retain(keep_range);
    out.externref.retain(keep_range);
    out.strings.retain(|s| keep(s.addr));
    out.context_paints.retain(|p| keep(p.addr));
    out.tracked_regs.retain(|t| keep(t.func_addr));
    out.locals.retain(|l| keep(l.func_addr));
    out.comments.retain(|c| keep(c.func_addr) && keep(c.addr));
    out.fid_names.retain(|f| keep(f.addr));
    out.cpp_dwarf.symbols.retain(|s| keep(s.addr));
    out.cpp_dwarf.locals.retain(|l| keep(l.func_addr));
    out.cpp_dwarf.prototypes.retain(|(a, _)| keep(*a));
    out.cpp_sig.proven.retain(|(a, _)| keep(*a));
    out.cpp_sig.inferred.retain(|(a, _)| keep(*a));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::loader::reloc_object::RELOC_BASE;

    /// The gate is a process-global env var, so the test that toggles it and the
    /// tests that read it must not run concurrently.
    static GATE: std::sync::Mutex<()> = std::sync::Mutex::new(());

    fn serial() -> std::sync::MutexGuard<'static, ()> {
        GATE.lock().unwrap_or_else(|e| e.into_inner())
    }

    fn fixture(name: &str) -> Vec<u8> {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures")
            .join(name);
        std::fs::read(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
    }

    /// Every laid-out section, symbol and `.eh_frame` FDE of an ELF `ET_REL`
    /// lands in the loaded image's address space after the rebase.
    #[test]
    fn et_rel_sections_and_symbols_are_rebased() {
        let _serial = serial();
        let bytes = fixture("ptx.o");
        let file = object::File::parse(&*bytes).expect("parse ptx.o");
        let view = rebased_view(&file, &bytes).expect("ptx.o is synthetically laid out");
        let rebased = object::File::parse(&*view.bytes).expect("patched image still parses");

        let text = rebased.section_by_name(".text").expect(".text");
        assert_eq!(text.address(), RELOC_BASE, ".text at the layout base");
        let startup = rebased.section_by_name(".text.startup").expect(".text.startup");
        assert!(startup.address() > RELOC_BASE, ".text.startup laid out above .text");

        let sym = |name: &str| {
            rebased
                .symbols()
                .find(|s| s.name() == Ok(name))
                .unwrap_or_else(|| panic!("symbol {name}"))
                .address()
        };
        assert_eq!(sym("to_uchar"), RELOC_BASE, ".text+0 symbol rebased");
        assert_eq!(sym("main"), startup.address(), ".text.startup+0 symbol rebased");

        // The `.eh_frame` FDE oracle used to report the FDE's own section offset
        // (0x20, 0x34, …) because `initial_location` is a PC-relative field the
        // linker had not resolved. Every start must now be a real code address.
        let starts = crate::entry::scan_eh_frame_starts(&rebased);
        assert!(!starts.is_empty(), "ptx.o has FDEs");
        for start in &starts {
            assert!(view.contains(*start), "FDE start {start:#x} outside the loaded image");
        }
        assert!(starts.contains(&RELOC_BASE), "the .text+0 FDE resolves to the layout base");
    }

    /// The DWARF pass's `DW_AT_low_pc` and `DW_FORM_strp` fields need their
    /// `.debug_info` relocations applied: unrebased, every subprogram reads
    /// address 0 and every name reads `.debug_str`+0.
    #[test]
    fn et_rel_debug_sections_are_relocated() {
        let _serial = serial();
        let bytes = fixture("ptx.o");
        let file = object::File::parse(&*bytes).expect("parse ptx.o");
        let view = rebased_view(&file, &bytes).expect("synthetically laid out");
        let raw = file.section_by_name(".debug_info").expect(".debug_info").data().unwrap();
        let rebased = object::File::parse(&*view.bytes).unwrap();
        let patched = rebased.section_by_name(".debug_info").unwrap().data().unwrap();
        assert_ne!(raw, patched, ".debug_info relocations applied");
    }

    /// A COFF `.obj` takes the same treatment through its `VirtualAddress` field.
    #[test]
    fn coff_obj_sections_are_rebased() {
        let _serial = serial();
        for name in ["coff_obj.obj", "coff_comdat_i386.obj"] {
            let bytes = fixture(name);
            let file = object::File::parse(&*bytes).expect("parse obj");
            let view = rebased_view(&file, &bytes).expect("obj is synthetically laid out");
            let rebased = object::File::parse(&*view.bytes).expect("patched obj still parses");
            let mut any = false;
            for sec in rebased.sections() {
                if sec.size() == 0 {
                    continue;
                }
                if sec.address() == 0 {
                    continue; // a link-time-only section keeps no address
                }
                any = true;
                assert!(
                    view.contains(sec.address()),
                    "{name}: section {:?} at {:#x} outside the loaded image",
                    sec.name(),
                    sec.address()
                );
            }
            assert!(any, "{name}: no section was rebased");
        }
    }

    /// The gate is honoured: `off` yields no view at all, so the analyzer tier
    /// keeps the pre-fix (pre-link) inputs byte for byte.
    #[test]
    fn gate_off_declines() {
        let _serial = serial();
        let bytes = fixture("ptx.o");
        let file = object::File::parse(&*bytes).expect("parse ptx.o");
        kuna_decomp::kuna_relocrebase::set_relocrebase_env(false);
        let off = rebased_view(&file, &bytes).is_none();
        kuna_decomp::kuna_relocrebase::set_relocrebase_env(true);
        let on = rebased_view(&file, &bytes).is_some();
        std::env::remove_var(kuna_decomp::kuna_relocrebase::RELOCREBASE_ENV);
        assert!(off, "`relocrebase off` must decline");
        assert!(on, "`relocrebase on` must rebase");
    }

    /// A linked executable is never touched (the whole feature is a strict no-op
    /// off the relocatable-object path).
    #[test]
    fn linked_image_is_untouched() {
        let _serial = serial();
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../../tests/bug-repro/grep");
        let Ok(bytes) = std::fs::read(&path) else { return };
        let file = object::File::parse(&*bytes).expect("parse grep");
        assert!(rebased_view(&file, &bytes).is_none(), "a linked ELF is not rebased");
    }
}
