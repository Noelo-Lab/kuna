//! The COFF [`ObjectFormat`] — the relocatable-object arm of the loader seam
//! (PR-2 skeleton).
//!
//! ## COFF object vs COFF image
//!
//! `object` reports two things as [`object::BinaryFormat::Coff`]: a **COFF
//! object** (a pre-link `.obj`/`.o`) and, historically, a COFF-flavored image.
//! A *linked* PE reports as `BinaryFormat::Pe` and routes through
//! [`crate::s1_loader::format::pe::PeFormat`]. So [`CoffFormat`] is specifically
//! the **object-file** case: pre-link, no IAT, externals are unresolved
//! *symbols* (in the COFF symbol table), not addresses. Its value is symbols +
//! strings + (rarely) DWARF, not import naming — so [`CoffFormat::resolve_imports`]
//! returns empty by design (design §3.6).
//!
//! Section flags are identical to PE (both key off the COFF `Characteristics`
//! field), so this impl reuses [`crate::s1_loader::format::pe::coff_section_bits`].
//!
//! All non-ELF magics (COFF's `IMAGE_FILE_MACHINE_*` prefix included) are gated
//! behind `--experimental-formats` at the engine dispatch, so this code is
//! unreachable on the default path.

use object::{Architecture, SectionFlags, SectionKind};

use super::{FormatKind, ImportSym, ObjectFormat};

/// The COFF relocatable-object format.
pub struct CoffFormat;

impl ObjectFormat for CoffFormat {
    fn kind(&self) -> FormatKind {
        FormatKind::Coff
    }

    fn compiler_model(&self, _arch: Architecture) -> Option<&'static str> {
        // COFF objects are overwhelmingly MSVC (the Windows toolchain output), so
        // the Windows ABI cspec is the right default — same token PE uses. The
        // `compose_language_id` fallback drops to `gcc`/`default` if an arch lacks
        // a `windows` id (design §2.2).
        Some("windows")
    }

    fn section_bits(&self, kind: SectionKind, flags: SectionFlags) -> u32 {
        // PE and COFF share the `Characteristics`-based section-flag model.
        crate::s1_loader::format::pe::coff_section_bits(kind, flags)
    }

    fn resolve_imports(&self, _file: &object::File, _bytes: &[u8]) -> Vec<ImportSym> {
        // A COFF *object* is pre-link: no IAT, no resolved imports — externals
        // are unresolved symbols, not addresses. So this is empty by design
        // (design §3.6). Defined symbols still flow through the format-neutral
        // `file.symbols()` source in `from_bytes`.
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `CoffFormat::compiler_model` is `windows` (MSVC default) for every arch.
    #[test]
    fn coff_compiler_model_is_windows() {
        let f = CoffFormat;
        for a in [Architecture::X86_64, Architecture::I386, Architecture::Aarch64] {
            assert_eq!(f.compiler_model(a), Some("windows"), "{a:?} must be :windows");
        }
    }

    /// COFF shares PE's `Characteristics` section-flag model — an exec section is
    /// CODE | READONLY (delegated to `pe::coff_section_bits`).
    #[test]
    fn coff_section_bits_reuse_pe() {
        use kuna_sleigh::loadimage::section_flags;
        use object::pe::{IMAGE_SCN_CNT_CODE, IMAGE_SCN_MEM_EXECUTE, IMAGE_SCN_MEM_READ};
        let f = CoffFormat;
        let text = IMAGE_SCN_CNT_CODE | IMAGE_SCN_MEM_EXECUTE | IMAGE_SCN_MEM_READ;
        let bits = f.section_bits(SectionKind::Text, SectionFlags::Coff { characteristics: text });
        assert!(bits & section_flags::CODE != 0, "exec section is CODE");
        assert!(bits & section_flags::READONLY != 0, "non-writable section is READONLY");
    }
}
