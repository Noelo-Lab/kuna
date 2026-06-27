//! Mach-O `__objc_*` section location + a VMA→bytes reader.
//!
//! The Objective-C 2.0 metadata graph is a web of absolute pointers between the
//! `__objc_*` sections (`__objc_classlist` → `__objc_data` `class_t` →
//! `__objc_const` `class_ro_t` → method/selector strings in `__TEXT`). Walking it
//! means dereferencing a VMA into whichever section covers it, so this module
//! builds a flat `[ (lo, hi, &bytes) ]` map of every *mapped* section and reads
//! little/big-endian scalars + C strings off it (the kuna analog of Ghidra's
//! `Memory`/`MemBuffer` reads in `Objc2*`).
//!
//! It is the port of the section-locating half of Ghidra's `Objc2Constants` /
//! `ObjcTypeMetadataAnalyzer.checkExecutableFormat` (the `getSection("__objc_*")`
//! lookups). Pure & total: a non-Mach-O / no-`__objc_*` input yields an empty
//! [`MachoImage::classlist`], so the pass is an additive no-op.

use object::read::{Object, ObjectSection};
use object::{Endianness, SectionKind};

/// The `__objc_classlist` section name (the array of `class_t*` the walk starts
/// from). Lives in `__DATA_CONST,__objc_classlist` on a modern toolchain,
/// `__DATA,__objc_classlist` historically — `object` exposes the bare section
/// short-name, so we match either.
pub const SEC_CLASSLIST: &str = "__objc_classlist";

/// A read-only view over a Mach-O image's mapped sections, keyed by VMA range, so
/// an absolute pointer in the ObjC metadata can be dereferenced to bytes.
pub struct MachoImage<'a> {
    /// `(vma_lo, vma_hi, data)` for every mapped, non-zero-length section, sorted
    /// by `vma_lo`. `data` is the section's file bytes (a zero-fill/`.bss` section
    /// is skipped — it carries no metadata pointers).
    spans: Vec<(u64, u64, &'a [u8])>,
    /// The image endianness (`__objc_*` scalars are read with it).
    endian: Endianness,
    /// Pointer size in bytes (8 for the 64-bit Mach-O this pass targets; 4 for a
    /// 32-bit image).
    ptr_size: u64,
}

impl<'a> MachoImage<'a> {
    /// Build the VMA→bytes map from a parsed Mach-O `file`. Skips zero-fill
    /// sections (no file content) and zero-length sections. `None` is never
    /// returned — an image with no usable sections yields an empty map (every read
    /// then misses, the inert path).
    pub fn new(file: &'a object::File<'a>) -> MachoImage<'a> {
        let mut spans: Vec<(u64, u64, &'a [u8])> = Vec::new();
        for sec in file.sections() {
            if matches!(sec.kind(), SectionKind::UninitializedData) {
                continue;
            }
            let addr = sec.address();
            let size = sec.size();
            if size == 0 {
                continue;
            }
            if let Ok(data) = sec.data() {
                spans.push((addr, addr.saturating_add(size), data));
            }
        }
        spans.sort_unstable_by_key(|&(lo, _, _)| lo);
        let ptr_size = if file.is_64() { 8 } else { 4 };
        MachoImage { spans, endian: file.endianness(), ptr_size }
    }

    /// Pointer size in bytes for this image.
    pub fn ptr_size(&self) -> u64 {
        self.ptr_size
    }

    /// `n` bytes starting at virtual address `va`, if `[va, va+n)` lies wholly
    /// inside one mapped section. `None` for an unmapped / split read.
    pub fn read(&self, va: u64, n: usize) -> Option<&'a [u8]> {
        let end = va.checked_add(n as u64)?;
        for &(lo, hi, data) in &self.spans {
            if va >= lo && end <= hi {
                let off = (va - lo) as usize;
                return data.get(off..off + n);
            }
        }
        None
    }

    /// A pointer-sized word at `va` (the absolute `class_t*` / `class_ro_t*` /
    /// string-pointer form the large/64-bit ObjC layout uses), honoring endianness
    /// + pointer width.
    pub fn read_ptr(&self, va: u64) -> Option<u64> {
        let bytes = self.read(va, self.ptr_size as usize)?;
        Some(match (self.endian, self.ptr_size) {
            (Endianness::Little, 8) => u64::from_le_bytes(bytes.try_into().ok()?),
            (Endianness::Big, 8) => u64::from_be_bytes(bytes.try_into().ok()?),
            (Endianness::Little, _) => u32::from_le_bytes(bytes.try_into().ok()?) as u64,
            (Endianness::Big, _) => u32::from_be_bytes(bytes.try_into().ok()?) as u64,
        })
    }

    /// A little/big-endian `u32` at `va` (the `method_list_t` header fields +
    /// the relative-method-list `i32` offsets are read through this).
    pub fn read_u32(&self, va: u64) -> Option<u32> {
        let bytes = self.read(va, 4)?;
        Some(match self.endian {
            Endianness::Little => u32::from_le_bytes(bytes.try_into().ok()?),
            Endianness::Big => u32::from_be_bytes(bytes.try_into().ok()?),
        })
    }

    /// A signed little/big-endian `i32` at `va` (a relative-method-list field is a
    /// signed self-relative displacement).
    pub fn read_i32(&self, va: u64) -> Option<i32> {
        self.read_u32(va).map(|v| v as i32)
    }

    /// The NUL-terminated ASCII string at `va` (a selector / class name / type
    /// encoding). `None` for an unmapped address or a string with no terminator in
    /// its section. Reads byte-by-byte across the covering section only.
    pub fn read_cstr(&self, va: u64) -> Option<String> {
        let &(lo, hi, data) = self.spans.iter().find(|&&(lo, hi, _)| va >= lo && va < hi)?;
        let start = (va - lo) as usize;
        let max = (hi - lo) as usize;
        let mut s = String::new();
        let mut i = start;
        while i < max && i < data.len() {
            let b = data[i];
            if b == 0 {
                return Some(s);
            }
            // ObjC selectors / class names / type encodings are printable ASCII.
            if !(0x20..0x7f).contains(&b) {
                return None;
            }
            s.push(b as char);
            i += 1;
        }
        None
    }

    /// The `(vma_lo, bytes)` of the `__objc_classlist` section — the array of
    /// `class_t*` entries the walk iterates (stride = [`Self::ptr_size`]). `None`
    /// when the image carries no ObjC class list (the inert path).
    pub fn classlist(&self, file: &'a object::File<'a>) -> Option<(u64, &'a [u8])> {
        let sec = file.section_by_name(SEC_CLASSLIST)?;
        let addr = sec.address();
        let data = sec.data().ok()?;
        if data.is_empty() {
            return None;
        }
        Some((addr, data))
    }
}
