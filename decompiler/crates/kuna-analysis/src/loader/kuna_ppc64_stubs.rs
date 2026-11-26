//! (kuna) The PowerPC64 ELFv2 PLT call stubs GNU ld and lld write into `.text`.
//!
//! A stub loads its import's `.plt` slot into r12 and ends `mtctr r12; bctr`,
//! after an optional `std r2,24(r1)` TOC save. The slot address comes from the
//! TOC pointer (`[addis r12,r2,ha;] ld r12,lo(r2|r12)`), from the program
//! counter (`mflr r12; bcl 20,31,.+4; mflr r11; mtlr r12; [addis r12,r11,ha;]
//! ld r12,lo(r11|r12)`, GNU ld's stub for pc-relative callers) or from a
//! Power10 `pld r12,off` (`--power10-stubs`, lld). GNU ld's stub for both TOC
//! and pc-relative callers is the TOC save followed by a pc-relative stub, a
//! `pld` one after a `nop` that keeps it 8-byte aligned, and pc-relative
//! callers enter it one word in. Only a slot an
//! `R_PPC64_JMP_SLOT` relocation names identifies an import. The sequence is a
//! jump through that slot whoever wrote it, but a compiler writes the `pld`
//! form itself for a `-fno-plt` sibling call, so no stub is taken at or inside
//! a defined function, whose own name would otherwise be replaced.

use kuna_decomp::kuna_fasthash::FxHashSet;
use object::read::{Object, ObjectSymbol};
use object::{RelocationFlags, SymbolKind};

const STD_R2_24_R1: u32 = 0xf841_0018;
const MFLR_R12: u32 = 0x7d88_02a6;
const BCL_NEXT: u32 = 0x429f_0005;
const MFLR_R11: u32 = 0x7d68_02a6;
const MTLR_R12: u32 = 0x7d88_03a6;
const MTCTR_R12: u32 = 0x7d89_03a6;
const BCTR: u32 = 0x4e80_0420;
const NOP: u32 = 0x6000_0000;

fn ha(w: u32, ra: u32) -> Option<i64> {
    (w >> 26 == 15 && (w >> 21) & 0x1f == 12 && (w >> 16) & 0x1f == ra).then(|| i64::from(w as i16) << 16)
}

fn ld_r12(w: u32, ra: u32) -> Option<i64> {
    (w >> 26 == 58 && w & 3 == 0 && (w >> 21) & 0x1f == 12 && (w >> 16) & 0x1f == ra)
        .then(|| i64::from((w & 0xfffc) as i16))
}

/// `[addis r12,base,ha;] ld r12,lo(base|r12)`: the displacement and the words used.
fn based_load(word: &impl Fn(usize) -> Option<u32>, at: usize, base: u32) -> Option<(i64, usize)> {
    let w = word(at)?;
    if let Some(high) = ha(w, base) {
        return Some((high + ld_r12(word(at + 4)?, 12)?, 8));
    }
    Some((ld_r12(w, base)?, 4))
}

/// `pld r12,d(0),1`: the 34-bit displacement from the prefix.
fn pld_r12(prefix: u32, suffix: u32) -> Option<i64> {
    let pcrel = prefix >> 26 == 1 && (prefix >> 24) & 3 == 0 && (prefix >> 20) & 1 == 1;
    let load = suffix >> 26 == 57 && (suffix >> 21) & 0x1f == 12 && (suffix >> 16) & 0x1f == 0;
    (pcrel && load).then(|| {
        let raw = (i64::from(prefix & 0x3_ffff) << 16) | i64::from(suffix & 0xffff);
        (raw << 30) >> 30
    })
}

/// Whether `w` can begin a stub: the TOC save, or the first word of a load.
fn opens(w: u32) -> bool {
    w == STD_R2_24_R1 || w == MFLR_R12 || w >> 26 == 1 || w >> 16 == 0x3d82 || w & 0xffff_0003 == 0xe982_0000
}

/// The `.plt` slot a stub at `off` loads, the stub's length in bytes, and
/// whether pc-relative callers enter it one word in, past its TOC save.
/// `toc_base` is the TOC pointer, if the image has a TOC.
#[inline]
pub(super) fn stub_at(word: impl Fn(usize) -> Option<u32>, vma: u64, off: usize, toc_base: Option<u64>) -> Option<(u64, usize, bool)> {
    let mut at = off;
    let first = word(at)?;
    if !opens(first) {
        return None;
    }
    let saves = first == STD_R2_24_R1;
    if saves {
        at += 4;
    }
    let pc = |at: usize| vma.wrapping_add(at as u64);
    let toc = toc_base.and_then(|toc| Some((toc, based_load(&word, at, 2)?)));
    let (slot, len) = if let Some((toc, (d, n))) = toc {
        (toc.wrapping_add(d as u64), n)
    } else if [MFLR_R12, BCL_NEXT, MFLR_R11, MTLR_R12].iter().enumerate().all(|(i, &w)| word(at + 4 * i) == Some(w)) {
        let (d, n) = based_load(&word, at + 16, 11)?;
        (pc(at + 8).wrapping_add(d as u64), 16 + n)
    } else {
        let p = if saves && word(at)? == NOP { at + 4 } else { at };
        let d = pld_r12(word(p)?, word(p + 4)?)?;
        (pc(p).wrapping_add(d as u64), p + 8 - at)
    };
    at += len;
    (word(at)? == MTCTR_R12 && word(at + 4)? == BCTR).then_some((slot, at + 8 - off, saves && toc.is_none()))
}

/// The slots `R_PPC64_JMP_SLOT` relocations name.
pub(super) fn jump_slots(file: &object::File) -> FxHashSet<u64> {
    let Some(relocations) = file.dynamic_relocations() else {
        return FxHashSet::default();
    };
    relocations
        .filter(|(_, r)| matches!(r.flags(), RelocationFlags::Elf { r_type } if r_type == object::elf::R_PPC64_JMP_SLOT))
        .map(|(slot, _)| slot)
        .collect()
}

/// The disjoint, sorted address ranges the image's defined functions cover,
/// lld's own `__plt_*` stub symbols aside.
pub(super) fn function_extents(file: &object::File) -> Vec<(u64, u64)> {
    let mut extents: Vec<(u64, u64)> = file
        .symbols()
        .chain(file.dynamic_symbols())
        .filter(|s| s.kind() == SymbolKind::Text && s.is_definition())
        .filter(|s| !(s.is_local() && s.name_bytes().is_ok_and(|n| n.starts_with(b"__plt_"))))
        .map(|s| (s.address(), s.address().saturating_add(s.size().max(1))))
        .collect();
    extents.sort_unstable();
    let mut merged: Vec<(u64, u64)> = Vec::with_capacity(extents.len());
    for (start, end) in extents {
        match merged.last_mut() {
            Some(last) if start <= last.1 => last.1 = last.1.max(end),
            _ => merged.push((start, end)),
        }
    }
    merged
}

/// Whether `addr` lies at or inside one of the disjoint `extents`.
pub(super) fn in_function(extents: &[(u64, u64)], addr: u64) -> bool {
    let after = extents.partition_point(|&(start, _)| start <= addr);
    after > 0 && addr < extents[after - 1].1
}
