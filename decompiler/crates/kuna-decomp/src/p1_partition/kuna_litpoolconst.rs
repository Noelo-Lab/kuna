//! (kuna) `litpoolconst` — a word read out of executable read-only memory is a
//! constant, so an ARM literal pool folds without the program-wide `readonly`.
//!
//! # The gap it closes
//!
//! An ARM immediate that does not fit the instruction encoding is parked in a
//! **literal pool** in `.text` and loaded PC-relatively, so the value a function
//! returns is a word sitting a few bytes past its own last instruction:
//!
//! ```text
//! 0x8440  ldr r3,[0x8458]
//! 0x8444  str r3,[r11,#-0x30]
//! ...
//! 0x8458  39050000            .word 0x00000539
//! ```
//!
//! kuna already paints that word `Varnode::readonly` — `.text` is
//! `SHF_ALLOC` without `SHF_WRITE` — but *folding* a read-only global is gated by
//! `option readonly` (`Architecture::readonlypropagate`), which is off by default
//! because it folds every `.rodata` read in the program. So the default output is
//! `v3 = dat_8458;` and the number the program actually returns is not in the C
//! at all. `kuna disassemble` names the same word `.word 0x00000539` in the same
//! listing, which is what makes this a gap rather than a limit.
//!
//! # The rule
//!
//! Fold a read that lies **entirely inside an allocated, executable,
//! non-writable, file-backed range**. Those bytes are the program's own
//! instruction stream: they are mapped `r-x`, so a store to them faults, and the
//! value in the image is the value at run time. That is the same "known by
//! construction, not by policy" standard `dynrelocs` applies to a
//! `PT_GNU_RELRO`-frozen slot, and it is narrower than `readonly` in the
//! direction that matters: a non-writable *data* section — `.rodata`,
//! `.data.rel.ro` — keeps today's behaviour unless the image maps it executable
//! too, which a single-RX-region firmware image does. The warrant is the
//! permission, not the section name, and a corpus of packers and protectors is
//! exactly where a data section's flags are least trustworthy.
//!
//! Ranges come from the loader's section table, or from the `PF_X` load segments
//! when the image carries no sections. An image whose loader reports neither —
//! the XML datatest corpus, the raw-bytes loader — contributes no ranges and the
//! option is inert.

use kuna_sleigh::loadimage::section_flags;

/// The `(vma, size, flags)` section/segment snapshot shape both
/// `ObjectLoadImage::section_snapshot` and `LoadImage::get_segments` report.
pub type RegionSnapshot = (u64, u64, u32);

/// Whether one snapshot row is executable read-only memory with file content.
fn is_code_const(&(_, size, flags): &RegionSnapshot) -> bool {
    size != 0
        && flags & section_flags::CODE != 0
        && flags & section_flags::READONLY != 0
        && flags & (section_flags::UNALLOC | section_flags::NOLOAD) == 0
}

/// The inclusive `[start, stop]` ranges whose contents fold to a constant with
/// `option readonly` still off: every executable, non-writable, file-backed
/// region of `sections`, or of `segments` when the image has no section table.
///
/// Sorted and merged, so a reader can binary-search.
pub fn code_const_ranges(
    sections: &[RegionSnapshot],
    segments: &[RegionSnapshot],
) -> Vec<(u64, u64)> {
    let rows = if sections.iter().any(is_code_const) { sections } else { segments };
    let mut out: Vec<(u64, u64)> = rows
        .iter()
        .filter(|row| is_code_const(row))
        .map(|&(vma, size, _)| (vma, vma.wrapping_add(size - 1)))
        .filter(|&(lo, hi)| hi >= lo)
        .collect();
    out.sort_unstable();
    out.dedup();
    let mut merged: Vec<(u64, u64)> = Vec::with_capacity(out.len());
    for (lo, hi) in out {
        match merged.last_mut() {
            Some(last) if lo <= last.1.saturating_add(1) => last.1 = last.1.max(hi),
            _ => merged.push((lo, hi)),
        }
    }
    merged
}

/// Whether the `size`-byte read at `offset` lies entirely inside one range of
/// `ranges` (sorted, disjoint, inclusive).
///
/// The whole read, not just its first byte: a word straddling the end of `.text`
/// is half instruction stream and half something else, and neither half is
/// evidence about the other.
pub fn contains(ranges: &[(u64, u64)], offset: u64, size: u32) -> bool {
    if ranges.is_empty() || size == 0 {
        return false;
    }
    let Some(last) = offset.checked_add(u64::from(size) - 1) else {
        return false;
    };
    let idx = ranges.partition_point(|&(lo, _)| lo <= offset);
    idx > 0 && ranges[idx - 1].1 >= last
}

#[cfg(test)]
mod tests;
