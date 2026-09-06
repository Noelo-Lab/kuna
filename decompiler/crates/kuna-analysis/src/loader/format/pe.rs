//! The PE [`ObjectFormat`] — the Windows portable-executable arm of the loader
//! boundary (PR-2 skeleton).
//!
//! This PR enables `object`'s `pe` reader and wires PE through the boundary so a PE
//! image **parses, maps its sections with the right exec/readonly bits, and
//! selects the Windows SLEIGH spec**. Import naming (IAT/INT) is deliberately
//! out of scope here — [`PeFormat::resolve_imports`] returns empty; the real
//! IAT walk lands in PR-4 (`loader/pe_iat.rs`, design §3.2). Until then a
//! PE's calls render as `sub_<addr>`, which is the correct skeleton behavior.
//!
//! All object-format magics (PE included) are admitted by the engine dispatch
//! (`is_object_binary`) unconditionally; the XML/datatest corpus never carries a
//! PE magic, so the ELF/XML oracles are structurally untouched.

use object::pe::{
    IMAGE_SCN_CNT_CODE, IMAGE_SCN_CNT_UNINITIALIZED_DATA, IMAGE_SCN_MEM_EXECUTE,
    IMAGE_SCN_MEM_WRITE,
};
use object::{Architecture, SectionFlags, SectionKind};

use kuna_sleigh::loadimage::section_flags;

use super::{FormatKind, HeaderRegion, ImportSym, ObjectFormat};

/// The PE (Windows portable-executable) object format.
pub struct PeFormat;

/// Translate a COFF/PE `Characteristics` bitset (+ the neutral [`SectionKind`])
/// into the kuna `section_flags` bitset — the PE/COFF arm of the old
/// `section_kind_flags`. PE images and COFF objects share the same section-flag
/// model (a linked PE *is* a COFF-flavored image), so [`crate::loader::format::coff::CoffFormat`]
/// reuses this. Mirrors the BFD `SEC_*` derivation Ghidra's `CoffLoader` /
/// `PeLoader` perform from `IMAGE_SCN_*`.
pub(super) fn coff_section_bits(kind: SectionKind, flags: SectionFlags) -> u32 {
    let chars = match flags {
        SectionFlags::Coff { characteristics } => characteristics,
        _ => 0,
    };
    let exec = chars & IMAGE_SCN_MEM_EXECUTE != 0;
    let write = chars & IMAGE_SCN_MEM_WRITE != 0;
    let uninit = chars & IMAGE_SCN_CNT_UNINITIALIZED_DATA != 0;
    let code = chars & IMAGE_SCN_CNT_CODE != 0;

    // PE/COFF allocation: every section in a COFF/PE image is allocated (there is
    // no SHF_ALLOC analog — sections are mapped). So `UNALLOC` is never set on
    // the PE arm; the readonly/exec/load bits come straight from Characteristics.
    let mut out = 0u32;

    // NOLOAD: an uninitialized (.bss-style) section has no file content. Mirror
    // BFD's `!SEC_LOAD` allocated section: `SectionKind::UninitializedData` is
    // the neutral signal, with the COFF Characteristics flag as a fallback.
    if matches!(kind, SectionKind::UninitializedData) || uninit {
        out |= section_flags::NOLOAD;
    }
    // READONLY: an allocated, non-writable section (the BFD readonly bit). All
    // PE sections are allocated, so this is simply "not writable".
    if !write {
        out |= section_flags::READONLY;
    }
    // CODE / DATA, from the executable bit / content-type, with the neutral
    // `SectionKind` as the backstop.
    if exec || code || matches!(kind, SectionKind::Text) {
        out |= section_flags::CODE;
    }
    if matches!(kind, SectionKind::Data | SectionKind::ReadOnlyData) {
        out |= section_flags::DATA;
    }
    out
}

impl ObjectFormat for PeFormat {
    fn kind(&self) -> FormatKind {
        FormatKind::Pe
    }

    fn compiler_model(&self, _arch: Architecture) -> Option<&'static str> {
        // PE is overwhelmingly the Windows ABI (Visual Studio / MinGW both pick
        // the `windows` cspec for the calling convention). Per arch the vendored
        // ldefs all declare a `windows` id (x86 `windows`/`clangwindows`,
        // AARCH64/ARM `windows`), so one token covers every PE arch. If a future
        // arch lacks it, `compose_language_id`'s fallback drops to `gcc`/`default`
        // rather than erroring (design §2.2).
        Some("windows")
    }

    fn section_bits(&self, kind: SectionKind, flags: SectionFlags) -> u32 {
        coff_section_bits(kind, flags)
    }

    fn resolve_imports(&self, file: &object::File, bytes: &[u8]) -> Vec<ImportSym> {
        // PE import naming: walk the Import Directory (INT/IAT lockstep), name
        // each IAT slot the engine constant-folds, decode the MinGW `FF 25` thunk
        // veneers that a direct `call` targets, and register the exports — all in
        // `loader/pe_iat.rs` (design §3.2). Pure & total: a non-PE / no-import
        // / unparsable layout yields an empty `Vec`.
        crate::loader::pe_iat::resolve_pe_imports(file, bytes)
    }

    fn header_region(&self, file: &object::File, bytes: &[u8]) -> Option<HeaderRegion> {
        // The `SizeOfHeaders` bytes Windows maps read-only at `ImageBase`, which
        // the section walk never covers (design: `loader/pe_headers.rs`).
        crate::loader::pe_headers::header_region(file, bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use object::pe::{IMAGE_SCN_CNT_CODE, IMAGE_SCN_MEM_EXECUTE, IMAGE_SCN_MEM_READ, IMAGE_SCN_MEM_WRITE};

    /// `PeFormat::compiler_model` returns `windows` for every arch (the PE ABI).
    #[test]
    fn pe_compiler_model_is_windows() {
        let f = PeFormat;
        for a in [
            Architecture::X86_64,
            Architecture::I386,
            Architecture::Aarch64,
            Architecture::Arm,
        ] {
            assert_eq!(f.compiler_model(a), Some("windows"), "{a:?} must be :windows");
        }
    }

    /// A `.text` PE section (`IMAGE_SCN_CNT_CODE | MEM_EXECUTE | MEM_READ`, not
    /// writable) is CODE | READONLY and not UNALLOC; a writable `.data`-style
    /// section is not READONLY.
    #[test]
    fn pe_section_bits_text_is_code_readonly() {
        let f = PeFormat;
        let text = IMAGE_SCN_CNT_CODE | IMAGE_SCN_MEM_EXECUTE | IMAGE_SCN_MEM_READ;
        let bits = f.section_bits(SectionKind::Text, SectionFlags::Coff { characteristics: text });
        assert!(bits & section_flags::CODE != 0, "exec section is CODE");
        assert!(bits & section_flags::READONLY != 0, "non-writable section is READONLY");
        assert!(bits & section_flags::UNALLOC == 0, "PE sections are never UNALLOC");

        let data = IMAGE_SCN_MEM_READ | IMAGE_SCN_MEM_WRITE;
        let bits = f.section_bits(SectionKind::Data, SectionFlags::Coff { characteristics: data });
        assert!(bits & section_flags::DATA != 0, "data section is DATA");
        assert!(bits & section_flags::READONLY == 0, "writable section is not READONLY");
    }
}
