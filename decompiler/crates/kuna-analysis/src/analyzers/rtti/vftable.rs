//! The MSVC vftable walk — the kuna port of Ghidra's
//! `VfTableModel.getVfTableCount` (`Ghidra/Features/MicrosoftCodeAnalyzer/.../rtti/`).
//!
//! A validated [`CompleteObjectLocator`](super::models::CompleteObjectLocator) (COL)
//! sits in memory immediately *before* its class's vftable: the pointer-width word
//! at `vftable_base − ptr_size` (the "meta" slot) holds the COL's own VA, so the
//! vftable starts one pointer after the slot that references the COL. From that base
//! this module walks the vftable's function-pointer slots and bounds the array.
//!
//! # Slot ABI — full VAs on BOTH arches (the x64 subtlety, §3.1)
//!
//! Unlike the COL/RTTI inter-structure references — which are `IBO32` image-base
//! displacements on x64 ([`super::refkind::RefKind::resolve_ref`]) — a **vftable
//! slot is an absolute, pointer-width virtual address on every arch** (8 bytes on
//! x64, 4 on x86). The runtime dereferences these as real function pointers, so the
//! walk reads each with [`RefKind::read_ptr`], NOT `resolve_ref`. Conflating the two
//! is the classic x64 RTTI port error; the vftable is the one place inside the graph
//! that is NOT IBO32.
//!
//! # Bounding the array (port of `getVfTableCount`)
//!
//! Walk `ptr_size`-stride slots from the base; a slot is a valid virtual-method
//! entry while its full-VA value is **non-NULL** and **points into an executable
//! (`.text`) section** of the image. The first slot that fails either gate — NULL,
//! or a target outside `.text` (e.g. the next COL/RTTI struct in `.rdata`, or the
//! object's data) — terminates the table (faithful to Ghidra's `isPointer` +
//! function-start boundary check). A vftable with zero valid slots is rejected
//! (`None`): a real polymorphic class has at least one virtual method.
//!
//! Each surviving slot VA is a virtual-method **function** the pass names
//! `<Class>::vfunc_<i>` (the MSVC metadata graph carries the class name in the
//! RTTI0 `TypeDescriptor` but NOT per-method names, so the slot index is the faithful
//! disambiguator — the documented scope vs Ghidra's script-tier method demangling).
//! The `vfunc_` stem, not `vftable_`: the slot's target is code, and the vftable data
//! object at `base` is what carries the unindexed `<Class>::vftable` label.

use object::read::{Object, ObjectSection};
use object::SectionKind;

use super::models::ImageBytes;
use super::refkind::RefKind;

/// The executable (`.text`) address ranges of a loaded image, used to gate a
/// vftable slot: a slot is a virtual-method entry only if its target VA lands in
/// one of these ranges (a function pointer into code, not a data/RTTI pointer).
///
/// Built once per pass run from the parsed [`object::File`] — every section that is
/// `SectionKind::Text` or carries the PE `IMAGE_SCN_MEM_EXECUTE` characteristic
/// (the same executable-section test [`crate::loader::pe_iat`] uses).
pub struct TextRanges {
    /// `(start, end)` half-open VMA ranges of every executable section, sorted.
    ranges: Vec<(u64, u64)>,
}

impl TextRanges {
    /// Collect the executable-section ranges from `file`. Empty (matches nothing) if
    /// the image exposes no executable section.
    pub fn new(file: &object::File) -> Self {
        let mut ranges: Vec<(u64, u64)> = Vec::new();
        for sec in file.sections() {
            let exec = sec.kind() == SectionKind::Text
                || matches!(
                    sec.flags(),
                    object::SectionFlags::Coff { characteristics }
                        if characteristics & object::pe::IMAGE_SCN_MEM_EXECUTE != 0
                );
            if !exec {
                continue;
            }
            let start = sec.address();
            let size = sec.size();
            if size == 0 {
                continue;
            }
            ranges.push((start, start.saturating_add(size)));
        }
        ranges.sort_by_key(|&(s, _)| s);
        TextRanges { ranges }
    }

    /// Is `vma` inside any executable section? (A vftable slot must point into code.)
    pub fn contains(&self, vma: u64) -> bool {
        self.ranges.iter().any(|&(s, e)| vma >= s && vma < e)
    }

    /// Build a `TextRanges` from explicit `(start, end)` ranges — the synthetic-image
    /// constructor the sibling passes' unit tests use in place of a parsed object.
    #[cfg(test)]
    pub fn for_test(mut ranges: Vec<(u64, u64)>) -> Self {
        ranges.sort_by_key(|&(s, _)| s);
        TextRanges { ranges }
    }
}

/// A recovered vftable: its base VMA and the per-slot virtual-method function VAs
/// (in table order; each points into `.text`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VfTable {
    /// VMA of the vftable's first slot (the value an object's vptr holds).
    pub base: u64,
    /// The virtual-method function VAs, slot 0..N (each points into `.text`).
    pub slots: Vec<u64>,
}

impl VfTable {
    /// The half-open VMA range the slot array occupies: `[base, base + len*ptr)`.
    /// Used to mark the table read-only (`out.readonly`).
    pub fn range(&self, rk: &RefKind) -> (u64, u64) {
        let end = self.base + (self.slots.len() as u64) * (rk.ptr_size() as u64);
        (self.base, end)
    }
}

/// Walk the vftable that begins at `base`, bounding it at the first NULL / non-`.text`
/// slot, and return it. `None` if the table has zero valid virtual-method slots (the
/// candidate `base` is not really a vftable). A generous slot cap (`MAX_SLOTS`) guards
/// a runaway walk over a malformed / coincidental base.
///
/// Each slot is read as a full pointer-width VA ([`RefKind::read_ptr`]) — vftable
/// slots are absolute pointers on both x86 and x64 (NOT IBO32, unlike the COL/RTTI
/// inter-structure refs); see the module docs.
pub fn walk_vftable(
    img: &ImageBytes,
    rk: &RefKind,
    text: &TextRanges,
    base: u64,
) -> Option<VfTable> {
    /// Upper bound on virtual-method slots walked from one base — far beyond any real
    /// class's vtable, a guard against a coincidental base with no NULL terminator.
    const MAX_SLOTS: usize = 4096;

    let ptr = rk.ptr_size() as u64;
    let mut slots = Vec::new();
    let mut slot_addr = base;
    for _ in 0..MAX_SLOTS {
        let Some(bytes) = img.read(slot_addr, rk.ptr_size()) else {
            break; // ran off the end of the loaded image
        };
        let target = rk.read_ptr(bytes);
        // Boundary: NULL slot, or a target that is not code. Either terminates the
        // table (the next word is the following struct / NULL pad, not a method).
        if target == 0 || !text.contains(target) {
            break;
        }
        slots.push(target);
        slot_addr += ptr;
    }
    if slots.is_empty() {
        return None;
    }
    Some(VfTable { base, slots })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rtti::refkind::End;

    /// A synthetic x64 image: a `.text` at 0x140001000 (0x80 bytes) and a `.rdata`
    /// holding a 2-slot vftable @0x140002010 → [0x140001040, 0x140001050], then a
    /// NULL terminator, then a non-text pointer.
    fn x64_img() -> (Vec<u8>, RefKind, TextRanges) {
        let ib = 0x1_4000_0000u64;
        let rk = RefKind::new(true, ib, End::Little);
        let mut rdata = vec![0u8; 0x100]; // .rdata @ 0x140002000
        let put_u64 =
            |v: &mut [u8], off: usize, x: u64| v[off..off + 8].copy_from_slice(&x.to_le_bytes());
        // vftable @ 0x2010: slot0 -> 0x140001040, slot1 -> 0x140001050, slot2 NULL.
        put_u64(&mut rdata, 0x10, ib + 0x1040);
        put_u64(&mut rdata, 0x18, ib + 0x1050);
        // 0x20 stays NULL (terminator).
        // A non-text pointer right after (into .rdata) — also a boundary if reached.
        put_u64(&mut rdata, 0x28, ib + 0x2000);
        let ranges = TextRanges { ranges: vec![(ib + 0x1000, ib + 0x1080)] };
        (rdata, rk, ranges)
    }

    #[test]
    fn walks_to_null_boundary() {
        let (rdata, rk, ranges) = x64_img();
        let ib = 0x1_4000_0000u64;
        let img = ImageBytes::from_spans(vec![(ib + 0x2000, &rdata)]);
        let vt = walk_vftable(&img, &rk, &ranges, ib + 0x2010).unwrap();
        assert_eq!(vt.base, ib + 0x2010);
        assert_eq!(vt.slots, vec![ib + 0x1040, ib + 0x1050]);
        assert_eq!(vt.range(&rk), (ib + 0x2010, ib + 0x2020));
    }

    #[test]
    fn rejects_base_with_no_code_slot() {
        let (rdata, rk, ranges) = x64_img();
        let ib = 0x1_4000_0000u64;
        let img = ImageBytes::from_spans(vec![(ib + 0x2000, &rdata)]);
        // 0x2028 holds a .rdata pointer (not into .text) -> zero valid slots.
        assert_eq!(walk_vftable(&img, &rk, &ranges, ib + 0x2028), None);
    }

    #[test]
    fn one_slot_table() {
        let ib = 0x1_4000_0000u64;
        let rk = RefKind::new(true, ib, End::Little);
        let mut rdata = vec![0u8; 0x40];
        rdata[0x00..0x08].copy_from_slice(&(ib + 0x1004).to_le_bytes());
        // 0x08 NULL terminator.
        let ranges = TextRanges { ranges: vec![(ib + 0x1000, ib + 0x1080)] };
        let img = ImageBytes::from_spans(vec![(ib + 0x2010, &rdata)]);
        let vt = walk_vftable(&img, &rk, &ranges, ib + 0x2010).unwrap();
        assert_eq!(vt.slots, vec![ib + 0x1004]);
    }
}
