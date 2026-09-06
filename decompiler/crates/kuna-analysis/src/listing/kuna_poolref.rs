//! (kuna) `poolref` — follow a read-only pointer word, so a literal-pool load is
//! a reference to the thing the pool points at.
//!
//! # The gap
//!
//! [`super::xrefs`] reads a reference out of one instruction's p-code, so it
//! answers for whatever address that instruction encodes. On ARM the address of
//! a string is never encoded in the instruction that uses it: the constant does
//! not fit an ARM immediate, so the compiler parks it in a **literal pool** in
//! `.text` and the code loads it PC-relatively.
//!
//! ```text
//! 0x862c  ldr r0,[0x86e4]      ; r0 = the word at 0x86e4
//! 0x8630  bl  0x95cc           ; __libc_fatal(r0)
//! ...
//! 0x86e4  bc 61 06 00          ; = 0x661bc, "FATAL: kernel too old\n"
//! ```
//!
//! The walk files `0x862c -> 0x86e4 read` and stops there, because 0x86e4 is
//! all the instruction says. Nothing references 0x661bc, so `kuna xrefs --to`
//! that string answers zero and `kuna strings --json` reports it with
//! `xrefs_count 0` and no owning function — on a binary whose disassembly plainly
//! shows which function prints it. This is [`super::kuna_picbase`]'s defect one
//! indirection over: there the address is a register plus a displacement, here it
//! is a word in the image.
//!
//! # The rule
//!
//! One dereference, of content that cannot change. A `Read` of a location that
//! is
//!
//!  * in an **allocated, non-writable** section with file content — the word is
//!    the same at run time as it is in the image, so reading it is not a guess;
//!  * **pointer-sized and pointer-aligned** — a literal pool is an array of
//!    words, and a half-word or byte load of one is reading a number, not
//!    following a pointer;
//!  * holding a value that passes the same `ScalarOperandAnalyzer.checkOperands`
//!    filter the constant scan uses and lands in a mapped section
//!
//! files a second edge from the **same instruction** to that value, as
//! [`XrefKind::Data`](super::xrefs::XrefKind::Data) — the address-taken case,
//! which is exactly what the load did. Attributing it to the instruction rather
//! than to the pool word is what gives the reference an owning function; a pool
//! word is data and belongs to nothing.
//!
//! Only one hop, and only through read-only memory. A writable slot (a GOT
//! entry, a `.data` pointer) is whatever the loader or the program last wrote
//! there, and the image's copy of it is not evidence of anything.

use object::read::{Object, ObjectSection};
use object::SectionKind;

/// ELF section-header flag `SHF_WRITE` (the section is writable at run time).
const SHF_WRITE: u64 = 0x1;
/// ELF section-header flag `SHF_ALLOC` (the section occupies memory at runtime).
const SHF_ALLOC: u64 = 0x2;

/// The read-only half of the loaded image, as the dereference sees it: the
/// `[lo, hi)` and bytes of every allocated, non-writable section that has file
/// content, in address order.
pub(super) struct PoolImage<'a> {
    ro: Vec<(u64, u64, &'a [u8])>,
    /// The target's pointer width in bytes (4 or 8).
    ptr: u32,
    little_endian: bool,
}

impl<'a> PoolImage<'a> {
    /// The read-only image of `file`, or `None` when it has no read-only mapped
    /// section with content (nothing could ever be followed, so the walk should
    /// not carry the check at all).
    pub(super) fn new(file: &'a object::File<'a>) -> Option<Self> {
        let mut ro: Vec<(u64, u64, &'a [u8])> = Vec::new();
        for sec in file.sections() {
            let (lo, size) = (sec.address(), sec.size());
            if lo == 0 || size == 0 || !is_read_only(&sec) {
                continue;
            }
            let Ok(data) = sec.data() else { continue };
            if data.is_empty() {
                continue; // NOBITS: allocated but not in the file
            }
            let hi = lo.saturating_add(size.min(data.len() as u64));
            ro.push((lo, hi, data));
        }
        Self::from_ranges(ro, if file.is_64() { 8 } else { 4 }, file.is_little_endian())
    }

    /// [`Self::new`] over an already-collected range list, which is what makes
    /// the dereference testable without an image on disk.
    pub(super) fn from_ranges(mut ro: Vec<(u64, u64, &'a [u8])>, ptr: u32, little_endian: bool) -> Option<Self> {
        if ro.is_empty() {
            return None;
        }
        ro.sort_unstable_by_key(|&(lo, _, _)| lo);
        Some(PoolImage { ro, ptr, little_endian })
    }

    /// The address a `width`-byte read of `at` yields, when that read is a
    /// pointer-sized load of a read-only word holding a mapped address.
    ///
    /// `None` for every read that is not one of those, which is the overwhelming
    /// majority: a narrow load, an unaligned one, one from writable memory, and
    /// one whose word is not plausibly an address all decline here.
    pub(super) fn follow(&self, at: u64, width: u32, mapped: &[(u64, u64)]) -> Option<u64> {
        if width != self.ptr {
            return None;
        }
        let word = self.word_at(at)?;
        if !super::xrefs::looks_like_address(word) || !super::xrefs::in_range(mapped, word) {
            return None;
        }
        Some(word)
    }

    /// The target's pointer width in bytes, which is also the stride of any
    /// array of pointers in the image (`kuna_switchtable`'s table).
    pub(super) fn ptr_width(&self) -> u32 {
        self.ptr
    }

    /// The pointer-sized word at `vma`, when `vma` is pointer-aligned and lies in
    /// a read-only section.
    pub(super) fn word_at(&self, vma: u64) -> Option<u64> {
        let ptr = u64::from(self.ptr);
        if vma % ptr != 0 {
            return None;
        }
        let i = self.ro.partition_point(|&(lo, _, _)| lo <= vma).checked_sub(1)?;
        let (lo, hi, data) = self.ro[i];
        if vma >= hi {
            return None;
        }
        let off = (vma - lo) as usize;
        let end = off.checked_add(self.ptr as usize)?;
        let b = data.get(off..end)?;
        Some(match (self.ptr, self.little_endian) {
            (8, true) => u64::from_le_bytes(b.try_into().ok()?),
            (8, false) => u64::from_be_bytes(b.try_into().ok()?),
            (_, true) => u64::from(u32::from_le_bytes(b.try_into().ok()?)),
            (_, false) => u64::from(u32::from_be_bytes(b.try_into().ok()?)),
        })
    }
}

/// Is `sec` mapped at run time and never written? Per-format, falling back to the
/// neutral [`SectionKind`] when the container carries no flags we understand.
fn is_read_only(sec: &object::Section) -> bool {
    match sec.flags() {
        object::SectionFlags::Elf { sh_flags } => {
            sh_flags & SHF_ALLOC != 0 && sh_flags & SHF_WRITE == 0
        }
        object::SectionFlags::Coff { characteristics } => {
            characteristics & object::pe::IMAGE_SCN_MEM_WRITE == 0
                && characteristics
                    & (object::pe::IMAGE_SCN_MEM_EXECUTE | object::pe::IMAGE_SCN_MEM_READ)
                    != 0
        }
        _ => matches!(
            sec.kind(),
            SectionKind::Text | SectionKind::ReadOnlyData | SectionKind::ReadOnlyString
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A pool at 0x1000 holding, in order: a mapped address, a small number, a
    /// mapped address again. `MAPPED` is what a followed value must land in.
    const POOL: [u8; 12] = [
        0x00, 0x40, 0x00, 0x00, // 0x1000 -> 0x4000 (mapped)
        0x2a, 0x00, 0x00, 0x00, // 0x1004 -> 42      (a number)
        0x10, 0x40, 0x00, 0x00, // 0x1008 -> 0x4010 (mapped)
    ];
    const MAPPED: [(u64, u64); 2] = [(0x1000, 0x2000), (0x4000, 0x4100)];

    fn ro32() -> PoolImage<'static> {
        PoolImage::from_ranges(vec![(0x1000, 0x100c, &POOL[..])], 4, true).unwrap()
    }

    #[test]
    fn a_pointer_sized_read_of_a_read_only_word_follows_it() {
        assert_eq!(ro32().follow(0x1000, 4, &MAPPED), Some(0x4000));
        assert_eq!(ro32().follow(0x1008, 4, &MAPPED), Some(0x4010));
    }

    /// The defect the width gate exists for: `ldrh r0,[pool]` is reading a number
    /// out of the pool, and a `LOAD`'s address varnode is pointer-sized whatever
    /// the access is, so the width has to come from the access itself.
    #[test]
    fn a_narrow_read_of_a_pool_word_is_not_a_pointer_dereference() {
        assert_eq!(ro32().follow(0x1000, 2, &MAPPED), None);
        assert_eq!(ro32().follow(0x1000, 1, &MAPPED), None);
        assert_eq!(ro32().follow(0x1000, 8, &MAPPED), None);
    }

    /// A pool is an array of words; an unaligned read of one is not a slot.
    #[test]
    fn an_unaligned_location_is_not_a_pool_word() {
        assert_eq!(ro32().follow(0x1002, 4, &MAPPED), None);
    }

    /// The same `ScalarOperandAnalyzer.checkOperands` floor the constant scan
    /// uses: a small integer is a number however well it lands.
    #[test]
    fn a_word_that_is_not_plausibly_an_address_is_not_followed() {
        assert_eq!(ro32().follow(0x1004, 4, &MAPPED), None);
    }

    /// Nothing outside the read-only ranges is readable: a writable slot holds
    /// whatever the loader or the program last wrote, not what the image says.
    #[test]
    fn a_location_outside_every_read_only_range_reads_nothing() {
        assert_eq!(ro32().follow(0x3000, 4, &MAPPED), None);
        assert_eq!(ro32().follow(0x1010, 4, &MAPPED), None);
    }

    /// A word whose value lands in no mapped section is not an address.
    #[test]
    fn a_word_that_lands_nowhere_is_not_followed() {
        assert_eq!(ro32().follow(0x1000, 4, &[(0x9000, 0x9100)]), None);
    }

    /// A word that runs off the end of its section is not read at all.
    #[test]
    fn a_word_that_overruns_its_section_reads_nothing() {
        let img = PoolImage::from_ranges(vec![(0x1000, 0x1002, &POOL[..2])], 4, true).unwrap();
        assert_eq!(img.follow(0x1000, 4, &MAPPED), None);
    }

    #[test]
    fn the_word_is_read_in_the_images_own_byte_order() {
        let be = PoolImage::from_ranges(vec![(0x1000, 0x100c, &POOL[..])], 4, false).unwrap();
        assert_eq!(be.follow(0x1000, 4, &[(0x400000, 0x401000)]), Some(0x400000));
        let img = PoolImage::from_ranges(vec![(0x1000, 0x100c, &POOL[..])], 8, true).unwrap();
        assert_eq!(img.follow(0x1000, 8, &MAPPED), None); // 0x2a00004000 lands nowhere
    }

    #[test]
    fn an_image_with_no_read_only_section_declines_to_exist() {
        assert!(PoolImage::from_ranges(Vec::new(), 4, true).is_none());
    }
}
