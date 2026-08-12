//! The COFF [`ObjectFormat`] — the relocatable-object arm of the loader boundary
//! (PR-5; the PR-2 skeleton, now exercised end-to-end by `coff_obj.obj`).
//!
//! ## COFF object vs COFF image (design §3.6)
//!
//! `object` reports two things as [`object::BinaryFormat::Coff`]:
//!
//! - a **COFF object** — a pre-link `.obj`/`.o` (the common case, and what
//!   [`CoffFormat`] is for): sections sit at section-relative VMAs, there is no
//!   IAT and no resolved imports, and externals are unresolved *symbols* in the
//!   COFF symbol table, not addresses;
//! - a **COFF image** — a linked binary `object` happens to classify as
//!   COFF-flavored (rare). A *normal* linked PE reports as
//!   [`object::BinaryFormat::Pe`] and routes through
//!   [`crate::loader::format::pe::PeFormat`] (the IAT-naming arm), so it never
//!   reaches here.
//!
//! So [`CoffFormat`]'s value is the COFF symbol table (defined function names) +
//! the format-agnostic passes (strings / demangle / no-return-name / protos)
//! riding a COFF object — **not** import naming. A pre-link object has nothing to
//! resolve imports *to*, so [`CoffFormat::resolve_imports`] returns empty by
//! design (design §3.6); the unresolved external (e.g. a `puts` symbol) is simply
//! absent from the funcsym set.
//!
//! ## Defined-function-at-VMA-0
//!
//! A relocatable `.obj` places its first defined function at section-relative
//! VMA 0 (`compute @ .text+0`). The defined-funcsym source in
//! [`crate::loadimage_object`] therefore skips import placeholders on
//! `is_undefined()` rather than `addr == 0` — the format-faithful predicate that
//! still drops the undefined external while keeping a legitimately-defined COFF
//! function that happens to live at VMA 0 (behavior-identical on every ELF, where
//! the two predicates coincide).
//!
//! Section flags are identical to PE (both key off the COFF `Characteristics`
//! field), so this impl reuses [`crate::loader::format::pe::coff_section_bits`].
//!
//! All object-format magics (COFF's `IMAGE_FILE_MACHINE_*` prefix included) are
//! admitted by the engine dispatch unconditionally; the XML/datatest corpus never
//! carries a COFF prefix, so the ELF/XML oracles are untouched.

use object::read::Object;
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
        crate::loader::format::pe::coff_section_bits(kind, flags)
    }

    fn resolve_imports(&self, _file: &object::File, _bytes: &[u8]) -> Vec<ImportSym> {
        // A COFF *object* is pre-link: no IAT, no resolved imports — externals
        // are unresolved symbols, not addresses. So this is empty by design
        // (design §3.6). Defined symbols still flow through the format-neutral
        // `file.symbols()` source in `from_bytes`.
        Vec::new()
    }

    /// Every COFF *object* needs the synthetic layout — see the
    /// "all sections at VMA 0" note in the module header. A COFF *image* (the
    /// rare linked binary `object` classifies as COFF-flavored) reports a
    /// non-relocatable kind and keeps the mapped-image path.
    fn relocatable_layout(&self, file: &object::File) -> bool {
        file.kind() == object::ObjectKind::Relocatable
    }

    /// The `Characteristics` analog of `SHF_ALLOC`: a section holds code or data
    /// that occupies memory at run time. The link-time-only sections — the
    /// `.drectve` linker-directive blob (`LNK_INFO`), anything marked
    /// `LNK_REMOVE`, and the discardable MSVC CodeView sections (`.debug$S` /
    /// `.debug$T`, which are `CNT_INITIALIZED_DATA` but never mapped) — are
    /// excluded, so they neither consume load VMAs nor draw relocation work.
    fn is_alloc_section(&self, _kind: SectionKind, flags: SectionFlags) -> bool {
        use object::pe::{
            IMAGE_SCN_CNT_CODE, IMAGE_SCN_CNT_INITIALIZED_DATA,
            IMAGE_SCN_CNT_UNINITIALIZED_DATA, IMAGE_SCN_LNK_INFO, IMAGE_SCN_LNK_REMOVE,
            IMAGE_SCN_MEM_DISCARDABLE,
        };
        let SectionFlags::Coff { characteristics } = flags else { return false };
        let content = IMAGE_SCN_CNT_CODE
            | IMAGE_SCN_CNT_INITIALIZED_DATA
            | IMAGE_SCN_CNT_UNINITIALIZED_DATA;
        let link_only =
            IMAGE_SCN_LNK_INFO | IMAGE_SCN_LNK_REMOVE | IMAGE_SCN_MEM_DISCARDABLE;
        characteristics & content != 0 && characteristics & link_only == 0
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
