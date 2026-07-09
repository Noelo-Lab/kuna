//! The x86 / x64 reference-kind dispatch — the single place the MSVC-RTTI
//! pointer/displacement ABI diverges between 32-bit and 64-bit PE images.
//!
//! This is §3.1f of `docs/metadata-analyzers-design.md`, the one place a faithful
//! port of `RttiAnalyzer` goes wrong if conflated. On **x86** every inter-structure
//! reference inside the RTTI graph is a full **raw virtual address** the width of a
//! pointer (4 bytes). On **x64** those same references are **32-bit image-base
//! relative displacements** (`IBO32`: the real target is `image_base + disp32`),
//! NOT 8-byte pointers — so a naive 64-bit-pointer read of a COL's
//! `pClassHierarchyDescriptor` field reads garbage. Two further x64 deltas: the
//! RTTI0 `TypeDescriptor`'s class-name string sits at offset **16** (past the 8-byte
//! `pVFTable` + 8-byte spare) instead of **8** (4 + 4) on x86; and the RTTI2
//! `BaseClassArray` entries are 4-byte references on **both** arches (they are
//! `IBO32` on x64, raw `u32` VAs on x86 — same width, different meaning).
//!
//! The COL "meta" pointer stored in memory immediately *before* a vftable (the
//! `vftable_addr - ptr_size` slot Ghidra reads to find the COL) is a *full VA* on
//! both arches (it is a real loaded pointer the runtime dereferences), so it is read
//! at the native pointer width here — only the COL's *internal* fields are `IBO32`.
//!
//! Origin (upstream Ghidra, the tree kuna was ported from):
//! `Ghidra/Features/MicrosoftCodeAnalyzer/.../rtti/RttiUtil.java` +
//! `Rtti0Model`/`Rtti1Model`/`Rtti4Model` (`getRtti0Address`/`getReferencedAddress`
//! switch on `program.getDefaultPointerSize()`).

/// The endianness of the loaded image (PE is always little-endian on every arch
/// kuna's SLEIGH specs support, but the reader is endian-explicit for symmetry with
/// the rest of the analysis tier — see [`crate::pclntab`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum End {
    Little,
    Big,
}

/// The pointer/displacement ABI of an MSVC RTTI graph, selected by `is64`.
///
/// Carries the image base (needed to resolve the x64 `IBO32` displacements) and the
/// endianness; the per-field readers below dispatch on [`Self::is64`].
#[derive(Clone, Copy, Debug)]
pub struct RefKind {
    /// 64-bit (x64) image ⇒ `IBO32` inter-structure refs + name offset 16; else
    /// (x86) raw-VA refs + name offset 8.
    pub is64: bool,
    /// The PE image base (`ImageBase`), the addend for an x64 `IBO32` displacement.
    /// Unused on x86 (raw VAs are absolute).
    pub image_base: u64,
    /// Image endianness (always [`End::Little`] for a real PE).
    pub end: End,
}

impl RefKind {
    /// Build the ref-kind for a PE image.
    pub fn new(is64: bool, image_base: u64, end: End) -> Self {
        RefKind { is64, image_base, end }
    }

    /// Pointer width in bytes (8 on x64, 4 on x86).
    #[inline]
    pub fn ptr_size(&self) -> usize {
        if self.is64 {
            8
        } else {
            4
        }
    }

    /// The byte offset of the class-name string inside an RTTI0 `TypeDescriptor`:
    /// 16 on x64 (8-byte `pVFTable` + 8-byte spare), 8 on x86 (4 + 4).
    #[inline]
    pub fn type_descriptor_name_offset(&self) -> u64 {
        if self.is64 {
            16
        } else {
            8
        }
    }

    /// Read a raw little/big-endian `u32` from the front of `b` (caller guarantees
    /// `b.len() >= 4`).
    #[inline]
    pub fn read_u32(&self, b: &[u8]) -> u32 {
        let a = [b[0], b[1], b[2], b[3]];
        match self.end {
            End::Little => u32::from_le_bytes(a),
            End::Big => u32::from_be_bytes(a),
        }
    }

    /// Read a raw little/big-endian `u64` from the front of `b` (caller guarantees
    /// `b.len() >= 8`).
    #[inline]
    pub fn read_u64(&self, b: &[u8]) -> u64 {
        let mut a = [0u8; 8];
        a.copy_from_slice(&b[..8]);
        match self.end {
            End::Little => u64::from_le_bytes(a),
            End::Big => u64::from_be_bytes(a),
        }
    }

    /// Read a full-width **pointer** (a real loaded VA) from the front of `b`:
    /// 8 bytes on x64, 4 bytes on x86. Used for the COL meta-pointer stored before a
    /// vftable and the object vptr (both are absolute pointers on every arch).
    #[inline]
    pub fn read_ptr(&self, b: &[u8]) -> u64 {
        if self.is64 {
            self.read_u64(b)
        } else {
            self.read_u32(b) as u64
        }
    }

    /// Resolve an **inter-structure reference** field (COL→TD/CHD, CHD→BaseArray,
    /// BaseArray→RTTI1, RTTI1→TD) to its absolute target VA. On x64 this is a 4-byte
    /// `IBO32` displacement added to the image base; on x86 it is a full raw 4-byte
    /// VA (which on a 32-bit image IS pointer-width). `b` must hold ≥ 4 bytes.
    ///
    /// A zero displacement/VA returns `None` (an absent/optional reference — e.g. a
    /// COL `pSelf` that some toolchains leave null), so callers treat it as a
    /// non-edge rather than dereferencing the image base.
    #[inline]
    pub fn resolve_ref(&self, b: &[u8]) -> Option<u64> {
        let disp = self.read_u32(b);
        if disp == 0 {
            return None;
        }
        if self.is64 {
            Some(self.image_base.wrapping_add(disp as u64))
        } else {
            Some(disp as u64)
        }
    }

    /// The byte width of one inter-structure reference field: **4 on both arches**
    /// (x64 `IBO32`, x86 raw `u32` VA). The COL/CHD/RTTI1 layouts are therefore the
    /// SAME field count + width on both arches; only the *meaning* of the 4 bytes
    /// (image-relative vs absolute) and the `TypeDescriptor` name offset differ.
    #[inline]
    pub fn ref_size(&self) -> usize {
        4
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn x86_refs_are_raw_va_name_at_8() {
        let rk = RefKind::new(false, 0x00400000, End::Little);
        assert_eq!(rk.ptr_size(), 4);
        assert_eq!(rk.type_descriptor_name_offset(), 8);
        // A raw-VA ref is the literal 4-byte value (no image base added).
        let bytes = 0x0040_2024u32.to_le_bytes();
        assert_eq!(rk.resolve_ref(&bytes), Some(0x00402024));
        // A full pointer is 4 bytes wide on x86.
        assert_eq!(rk.read_ptr(&0x0040_200cu32.to_le_bytes()), 0x0040200c);
        // Zero is the no-edge sentinel.
        assert_eq!(rk.resolve_ref(&0u32.to_le_bytes()), None);
    }

    #[test]
    fn x64_refs_are_ibo32_name_at_16() {
        let rk = RefKind::new(true, 0x1_4000_0000, End::Little);
        assert_eq!(rk.ptr_size(), 8);
        assert_eq!(rk.type_descriptor_name_offset(), 16);
        // An IBO32 disp32 resolves to image_base + disp.
        let disp = 0x0000_2038u32.to_le_bytes();
        assert_eq!(rk.resolve_ref(&disp), Some(0x1_4000_2038));
        // A full pointer is 8 bytes wide on x64.
        let p = 0x1_4000_2010u64.to_le_bytes();
        assert_eq!(rk.read_ptr(&p), 0x1_4000_2010);
        // Ref fields are 4 bytes on both arches.
        assert_eq!(rk.ref_size(), 4);
    }
}
