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

use std::sync::Arc;

use kuna_sleigh::loadimage::ImageBytes;
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
    /// When present, the slices above are empty and reads come from the
    /// loaded image, including any applied object-file relocations.
    loaded: Option<Arc<dyn ImageBytes>>,
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
        Some(PoolImage { ro, loaded: None, ptr, little_endian })
    }

    /// Keep only read-only range metadata; read loaded words on demand.
    pub(super) fn from_loaded(
        ranges: Vec<(u64, u64)>,
        loaded: Arc<dyn ImageBytes>,
        ptr: u32,
        little_endian: bool,
    ) -> Option<Self> {
        let ro = ranges.into_iter().map(|(lo, hi)| (lo, hi, &[][..])).collect();
        let mut pool = Self::from_ranges(ro, ptr, little_endian)?;
        pool.loaded = Some(loaded);
        Some(pool)
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

    /// Every pointer-aligned, pointer-sized word of the read-only image.
    pub(super) fn words(&self) -> impl Iterator<Item = u64> + '_ {
        let ptr = self.ptr as usize;
        self.ro.iter().flat_map(move |&(lo, hi, data)| {
            let len = ((hi - lo) as usize).min(data.len());
            let lead = (ptr - (lo as usize % ptr)) % ptr;
            data[..len].get(lead..).unwrap_or(&[]).chunks_exact(ptr).map(move |b| match (ptr, self.little_endian) {
                (8, true) => u64::from_le_bytes(b.try_into().unwrap()),
                (8, false) => u64::from_be_bytes(b.try_into().unwrap()),
                (_, true) => u64::from(u32::from_le_bytes(b.try_into().unwrap())),
                (_, false) => u64::from(u32::from_be_bytes(b.try_into().unwrap())),
            })
        })
    }

    /// Every pointer-aligned read-only address a 32-bit window of `file`'s code
    /// spells: what an instruction names as a displacement or a pool word,
    /// whether or not anything decoded it.
    pub(super) fn code_named(&self, file: &object::File) -> std::collections::HashSet<u64> {
        let ptr = u64::from(self.ptr);
        let (lo, hi) = (self.ro[0].0, self.ro[self.ro.len() - 1].1);
        let mut out = std::collections::HashSet::new();
        for sec in file.sections().filter(|s| s.kind() == SectionKind::Text) {
            let Ok(data) = sec.data() else { continue };
            for b in data.windows(4) {
                let b: [u8; 4] = b.try_into().unwrap();
                let v = u64::from(if self.little_endian { u32::from_le_bytes(b) } else { u32::from_be_bytes(b) });
                if v % ptr == 0 && v >= lo && v < hi && self.contains_bytes(v, ptr) {
                    out.insert(v);
                }
            }
        }
        out
    }

    /// The target's pointer width in bytes, which is also the stride of any
    /// array of pointers in the image (`kuna_switchtable`'s table).
    pub(super) fn ptr_width(&self) -> u32 {
        self.ptr
    }

    /// Whether a complete read stays inside the declared read-only file range.
    pub(super) fn contains_bytes(&self, vma: u64, width: u64) -> bool {
        let Some(end) = vma.checked_add(width) else { return false };
        let Some(i) = self.ro.partition_point(|&(lo, _, _)| lo <= vma).checked_sub(1) else {
            return false;
        };
        let (lo, hi, data) = self.ro[i];
        width != 0 && vma < hi && end <= hi && match &self.loaded {
            Some(image) => image.mapped_covers(vma, end),
            None => end - lo <= data.len() as u64,
        }
    }

    /// The signed 32-bit word at `vma`, when `vma` is 4-byte aligned and lies in
    /// a read-only section — the entry width of a delta-encoded jump table
    /// ([`super::kuna_switchtable`]), which is 32 bits on every target that
    /// emits one whatever the pointer width is.
    pub(super) fn word32_at(&self, vma: u64) -> Option<i32> {
        if vma % 4 != 0 {
            return None;
        }
        let b = self.read_word(vma)?;
        Some(if self.little_endian { i32::from_le_bytes(b) } else { i32::from_be_bytes(b) })
    }

    /// The pointer-sized word at `vma`, when `vma` is pointer-aligned and lies in
    /// a read-only section.
    pub(super) fn word_at(&self, vma: u64) -> Option<u64> {
        let ptr = u64::from(self.ptr);
        if vma % ptr != 0 {
            return None;
        }
        Some(match (self.ptr, self.little_endian) {
            (8, true) => u64::from_le_bytes(self.read_word(vma)?),
            (8, false) => u64::from_be_bytes(self.read_word(vma)?),
            (_, true) => u64::from(u32::from_le_bytes(self.read_word(vma)?)),
            (_, false) => u64::from(u32::from_be_bytes(self.read_word(vma)?)),
        })
    }

    /// The unsigned `width`-byte (1 or 2) word at `vma` in a read-only section.
    pub(super) fn uword_at(&self, vma: u64, width: u32) -> Option<u64> {
        Some(match (width, self.little_endian) {
            (1, _) => u64::from(self.read_word::<1>(vma)?[0]),
            (2, true) => u64::from(u16::from_le_bytes(self.read_word(vma)?)),
            (2, false) => u64::from(u16::from_be_bytes(self.read_word(vma)?)),
            _ => return None,
        })
    }

    fn read_word<const N: usize>(&self, vma: u64) -> Option<[u8; N]> {
        let Some(image) = &self.loaded else {
            return self.bytes_at(vma, N as u32)?.try_into().ok();
        };
        if !self.contains_bytes(vma, N as u64) {
            return None;
        }
        let mut bytes = [0; N];
        (image.fill_span(&mut bytes, vma) == 0).then_some(bytes)
    }

    /// The `width`-byte (1, 2 or 4) word at `vma` in a read-only section, sign-
    /// or zero-extended: an entry of a relative jump table.
    pub(super) fn sword_at(&self, vma: u64, width: u32, signed: bool) -> Option<i64> {
        let raw = match (width, self.little_endian) {
            (1 | 2, _) => self.uword_at(vma, width)?,
            (4, true) => u64::from(u32::from_le_bytes(self.read_word(vma)?)),
            (4, false) => u64::from(u32::from_be_bytes(self.read_word(vma)?)),
            _ => return None,
        };
        let bits = 8 * width;
        Some(if signed && raw >> (bits - 1) == 1 { (raw | (u64::MAX << bits)) as i64 } else { raw as i64 })
    }

    /// The `[lo, hi)` range of the read-only section holding `vma`.
    pub(super) fn section_of(&self, vma: u64) -> Option<(u64, u64)> {
        let i = self.ro.partition_point(|&(lo, _, _)| lo <= vma).checked_sub(1)?;
        let (lo, hi, _) = self.ro[i];
        (vma < hi).then_some((lo, hi))
    }

    /// The `width` bytes of read-only image content at `vma`.
    pub(super) fn bytes_at(&self, vma: u64, width: u32) -> Option<&'a [u8]> {
        let i = self.ro.partition_point(|&(lo, _, _)| lo <= vma).checked_sub(1)?;
        let (lo, hi, data) = self.ro[i];
        if vma >= hi {
            return None;
        }
        let off = (vma - lo) as usize;
        let end = off.checked_add(width as usize)?;
        data.get(off..end)
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
    fn relative_words_use_loaded_bytes_and_stay_in_the_read_only_range() {
        use crate::loadimage_object::ObjectLoadImage;
        use kuna_sleigh::loadimage::LoadImage;

        let bytes = include_bytes!("../../tests/fixtures/arm_thumb_abi_linked.elf");
        let image = ObjectLoadImage::from_bytes_silent("fixture", bytes).unwrap();
        let pool = PoolImage::from_loaded(vec![(0x400000, 0x400024)], image.shared_bytes().unwrap(), 4, true).unwrap();
        assert_eq!(pool.sword_at(0x400020, 2, true), Some(-8184));
        assert_eq!(pool.sword_at(0x400020, 2, false), Some(0xe008));
        assert_eq!(pool.sword_at(0x400022, 4, true), None);
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
