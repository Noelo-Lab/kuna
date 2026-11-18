//! **ARM EHABI unwind index** — `.ARM.exidx` function starts and the
//! exception tables `.ARM.extab` holds for them. ARM32 ELF only.
//!
//! An ARM32 ELF carries no `.eh_frame`: the unwinder searches `.ARM.exidx`, a
//! table of 8-byte entries sorted by address. Word 0 is a prel31 offset to a
//! function's first instruction. Word 1 is `1` (`EXIDX_CANTUNWIND`), an inline
//! compact unwind description (bit 31 set), or a prel31 offset to an
//! `.ARM.extab` entry. An `.ARM.extab` entry in the generic model starts with a
//! prel31 offset to its personality routine; the routine's data follows. For
//! the GNU C++ personality that data is one word-aligned run of unwind opcodes,
//! whose first byte counts the additional words, and then a language-specific
//! data area in the `.gcc_except_table` layout.
//!
//! [`scan`] returns every entry that can unwind, with the next entry's start as
//! its end. A `CANTUNWIND` entry only bounds its predecessor: the linker writes
//! one at the end of each code section, so it marks where unwinding stops, not
//! a function.
//!
//! # Which instruction set a start decodes in
//!
//! Bit 0 of a start is the Thumb bit, but a table whose relocations were made
//! against section symbols leaves it clear on Thumb code. So a start is Thumb
//! when the bit is set or the address is not a multiple of four. Any other
//! start is decided by the image: A32 when nothing in the image is known to be
//! Thumb, Thumb when something is and no function symbol is known to be A32,
//! and left out when the image holds both, because decoding it in the wrong
//! set yields a function of garbage.

use object::read::{Object, ObjectSection, ObjectSymbol};
use object::SymbolKind;

use crate::pass::ContextPaint;

/// The most entries read from one table.
const MAX_ENTRIES: usize = 1 << 22;

/// One `.ARM.exidx` entry that can unwind.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ExidxEntry {
    /// The function's first instruction, Thumb bit removed.
    pub start: u64,
    /// The next table entry's start, when it lies above this one.
    pub end: Option<u64>,
    /// The instruction set at `start`, `None` when the image does not decide it.
    pub thumb: Option<bool>,
    /// The language-specific data area of a generic-model `.ARM.extab` entry.
    pub lsda: Option<u64>,
}

fn prel31(word: u32, place: u64) -> u64 {
    let offset = ((word << 1) as i32 >> 1) as i64;
    (place as i64).wrapping_add(offset) as u64 & 0xffff_ffff
}

/// The generic-model data area of the `.ARM.extab` entry at `entry`: past the
/// personality word and the unwind-opcode words.
fn extab_lsda(extab: &[u8], extab_vma: u64, entry: u64, le: bool) -> Option<u64> {
    let off = usize::try_from(entry.checked_sub(extab_vma)?).ok()?;
    let personality = super::read_u32(extab.get(off..off.checked_add(4)?)?, le);
    if personality & 0x8000_0000 != 0 {
        return None;
    }
    let opcodes = super::read_u32(extab.get(off + 4..off.checked_add(8)?)?, le);
    let lsda = off + 8 + 4 * (opcodes >> 24) as usize;
    (lsda < extab.len()).then(|| extab_vma + lsda as u64)
}

/// Does a defined function symbol say the image holds Thumb code, A32 code?
fn symbol_modes(file: &object::File) -> (bool, bool) {
    let (mut thumb, mut arm) = (false, false);
    for sym in file.symbols().chain(file.dynamic_symbols()) {
        if sym.kind() == SymbolKind::Text && sym.is_definition() && sym.size() != 0 {
            if sym.address() & 1 != 0 {
                thumb = true;
            } else {
                arm = true;
            }
        }
    }
    (thumb, arm)
}

/// The unwindable entries of an ARM32 ELF's `.ARM.exidx`, in table order; empty
/// on every other image.
pub fn scan(file: &object::File) -> Vec<ExidxEntry> {
    let mut out = Vec::new();
    if file.format() != object::BinaryFormat::Elf || file.architecture() != object::Architecture::Arm {
        return out;
    }
    let Some(sec) = file.section_by_name(".ARM.exidx") else {
        return out;
    };
    let Ok(data) = sec.data() else {
        return out;
    };
    let extab = file.section_by_name(".ARM.extab").and_then(|s| Some((s.address(), s.data().ok()?)));
    let le = file.is_little_endian();
    let word = |i: usize, half: usize| super::read_u32(&data[8 * i + half..], le);
    let count = (data.len() / 8).min(MAX_ENTRIES);
    let raw_start = |i: usize| prel31(word(i, 0), sec.address() + 8 * i as u64);
    for i in 0..count {
        let content = word(i, 4);
        if content == 1 {
            continue;
        }
        let place = sec.address() + 8 * i as u64;
        let raw = raw_start(i);
        let start = raw & !1;
        let lsda = match extab {
            Some((vma, bytes)) if content & 0x8000_0000 == 0 => {
                extab_lsda(bytes, vma, prel31(content, place + 4), le)
            }
            _ => None,
        };
        let end = (i + 1 < count).then(|| raw_start(i + 1) & !1).filter(|&e| e > start);
        let thumb = (raw & 1 != 0 || start & 2 != 0).then_some(true);
        out.push(ExidxEntry { start, end, thumb, lsda });
    }
    let (thumb_symbol, arm_symbol) = symbol_modes(file);
    let any_thumb = thumb_symbol || out.iter().any(|e| e.thumb == Some(true));
    let rest = if !any_thumb {
        Some(false)
    } else if !arm_symbol {
        Some(true)
    } else {
        None
    };
    for e in &mut out {
        e.thumb = e.thumb.or(rest);
    }
    out
}

/// The starts whose instruction set is decided.
pub fn starts(file: &object::File) -> impl Iterator<Item = u64> {
    scan(file).into_iter().filter(|e| e.thumb.is_some()).map(|e| e.start)
}

/// `TMode=1` over every `.ARM.exidx` function known to be Thumb.
pub fn thumb_paints(file: &object::File) -> Vec<ContextPaint> {
    scan(file)
        .into_iter()
        .filter(|e| e.thumb == Some(true))
        .map(|e| ContextPaint { addr: e.start, end: e.end, var: "TMode", value: 1 })
        .collect()
}

#[cfg(test)]
#[path = "kuna_armexidx/tests.rs"]
mod tests;
