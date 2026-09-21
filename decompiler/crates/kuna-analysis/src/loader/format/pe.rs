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
/// The `object` crate's neutral section kind for a COFF `Characteristics`
/// word, the rule its own PE reader applies, for a section table read without
/// a parsed object (a TE's). One derivation, so the same characteristics yield
/// the same kind, the same [`coff_section_bits`], and the same export label on
/// every PE-family container.
pub(crate) fn coff_section_kind(characteristics: u32) -> SectionKind {
    use object::pe::{
        IMAGE_SCN_CNT_CODE, IMAGE_SCN_CNT_INITIALIZED_DATA, IMAGE_SCN_CNT_UNINITIALIZED_DATA,
        IMAGE_SCN_LNK_INFO, IMAGE_SCN_MEM_DISCARDABLE,
    };
    if characteristics & (IMAGE_SCN_CNT_CODE | IMAGE_SCN_MEM_EXECUTE) != 0 {
        SectionKind::Text
    } else if characteristics & IMAGE_SCN_CNT_INITIALIZED_DATA != 0 {
        if characteristics & IMAGE_SCN_MEM_DISCARDABLE != 0 {
            SectionKind::Other
        } else if characteristics & IMAGE_SCN_MEM_WRITE != 0 {
            SectionKind::Data
        } else {
            SectionKind::ReadOnlyData
        }
    } else if characteristics & IMAGE_SCN_CNT_UNINITIALIZED_DATA != 0 {
        SectionKind::UninitializedData
    } else if characteristics & IMAGE_SCN_LNK_INFO != 0 {
        SectionKind::Linker
    } else {
        SectionKind::Unknown
    }
}

pub(crate) fn coff_section_bits(kind: SectionKind, flags: SectionFlags) -> u32 {
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

    fn section_bits(&self, _name: &str, kind: SectionKind, flags: SectionFlags) -> u32 {
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

    fn import_slots(&self, _file: &object::File, bytes: &[u8]) -> Vec<(u64, u64)> {
        // The Import Address Table slot words, from the same INT/IAT lockstep
        // walk `resolve_imports` names them with.
        crate::loader::pe_iat::resolve_pe_import_slots(bytes)
    }

    fn header_region(&self, file: &object::File, bytes: &[u8]) -> Option<HeaderRegion> {
        // The `SizeOfHeaders` bytes Windows maps read-only at `ImageBase`, which
        // the section walk never covers (design: `loader/pe_headers.rs`).
        crate::loader::pe_headers::header_region(file, bytes)
    }
}

/// Is `bytes` a PE image whose optional-header `Subsystem` is Windows GUI (2) or
/// console (3)?
///
/// Those are the images whose code runs as a user-mode Windows thread, with the
/// Thread Environment Block at the `GS` (x86-64) or `FS` (x86) segment base. A
/// native-subsystem driver keeps its KPCR there instead, and an EFI image
/// nothing, so every other value -- and anything that is not a well-formed PE --
/// answers `false`. `Subsystem` sits 68 bytes into both the PE32 and the PE32+
/// optional header.
pub fn is_windows_user_mode_image(bytes: &[u8]) -> bool {
    const SUBSYSTEM_OFFSET: usize = 68;
    let read_u16 = |at: usize| bytes.get(at..at + 2).map(|b| u16::from_le_bytes([b[0], b[1]]));
    let read_u32 = |at: usize| bytes.get(at..at + 4).map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]));
    if bytes.get(0..2) != Some(&b"MZ"[..]) {
        return false;
    }
    let Some(pe) = read_u32(0x3c).map(|v| v as usize) else { return false };
    if bytes.get(pe..pe + 4) != Some(&b"PE\0\0"[..]) {
        return false;
    }
    let optional = pe + 24;
    let Some(opt_size) = read_u16(pe + 20) else { return false };
    if (opt_size as usize) < SUBSYSTEM_OFFSET + 2 || !matches!(read_u16(optional), Some(0x10b | 0x20b)) {
        return false;
    }
    matches!(
        read_u16(optional + SUBSYSTEM_OFFSET),
        Some(object::pe::IMAGE_SUBSYSTEM_WINDOWS_GUI | object::pe::IMAGE_SUBSYSTEM_WINDOWS_CUI)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pe_header(magic: u16, subsystem: u16) -> Vec<u8> {
        let mut b = vec![0u8; 0x200];
        b[0..2].copy_from_slice(b"MZ");
        b[0x3c..0x40].copy_from_slice(&0x40u32.to_le_bytes());
        b[0x40..0x44].copy_from_slice(b"PE\0\0");
        b[0x54..0x56].copy_from_slice(&0xf0u16.to_le_bytes());
        b[0x58..0x5a].copy_from_slice(&magic.to_le_bytes());
        b[0x58 + 68..0x58 + 70].copy_from_slice(&subsystem.to_le_bytes());
        b
    }

    /// GUI and console PE32/PE32+ images are user mode; a native driver, an EFI
    /// application, a truncated header and a non-PE are not.
    #[test]
    fn user_mode_image_is_gui_or_console_pe_only() {
        for magic in [0x10b, 0x20b] {
            assert!(is_windows_user_mode_image(&pe_header(magic, 2)));
            assert!(is_windows_user_mode_image(&pe_header(magic, 3)));
            assert!(!is_windows_user_mode_image(&pe_header(magic, 1)), "native driver");
            assert!(!is_windows_user_mode_image(&pe_header(magic, 10)), "EFI application");
        }
        assert!(!is_windows_user_mode_image(&pe_header(0x107, 3)), "ROM optional header");
        assert!(!is_windows_user_mode_image(&pe_header(0x20b, 3)[..0x90]), "truncated");
        assert!(!is_windows_user_mode_image(b"\x7fELF\x02\x01\x01"), "ELF");
    }
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
        let bits = f.section_bits("", SectionKind::Text, SectionFlags::Coff { characteristics: text });
        assert!(bits & section_flags::CODE != 0, "exec section is CODE");
        assert!(bits & section_flags::READONLY != 0, "non-writable section is READONLY");
        assert!(bits & section_flags::UNALLOC == 0, "PE sections are never UNALLOC");

        let data = IMAGE_SCN_MEM_READ | IMAGE_SCN_MEM_WRITE;
        let bits = f.section_bits("", SectionKind::Data, SectionFlags::Coff { characteristics: data });
        assert!(bits & section_flags::DATA != 0, "data section is DATA");
        assert!(bits & section_flags::READONLY == 0, "writable section is not READONLY");
    }
}
