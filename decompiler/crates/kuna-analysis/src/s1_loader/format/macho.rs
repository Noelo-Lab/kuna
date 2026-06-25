//! The Mach-O [`ObjectFormat`] — the Apple object/exe arm of the loader seam.
//!
//! This wires Mach-O through the seam so a Mach-O image **parses, maps its
//! sections, selects the right SLEIGH spec** (`gcc` for x86-64 — macOS x86-64
//! follows the System V AMD64 ABI, the same cspec Ghidra labels `gcc`;
//! `default` for arm64), and **names its imports** (PR-7): a `bl`/`callq`
//! targeting a `__TEXT,__stubs` entry renders `printf(` rather than
//! `sub_<addr>`, via the `LC_DYSYMTAB` indirect-symbol walk in
//! [`crate::s1_loader::macho_stubs`] (design §3.3).
//!
//! ## Section flags
//!
//! Mach-O does **not** carry per-section R/W/X permission in the section header
//! `flags` field — those permissions live in the enclosing segment's
//! `initprot`/`maxprot`. The section `flags` low byte is the section *type*
//! (`S_ZEROFILL`/`S_SYMBOL_STUBS`/…) and the high bits are *attributes*
//! (`S_ATTR_PURE_INSTRUCTIONS`/…). So [`MachOFormat::section_bits`] keys
//! primarily off the neutral [`SectionKind`] (which `object` derives from
//! segment+section), using the section-type/attributes for the NOLOAD/CODE
//! refinements. This is the faithful Mach-O analog of the BFD `SEC_*` mapping.
//!
//! All non-ELF magics (Mach-O's `0xfeedfac*` / fat `0xcafebabe` included) are
//! gated behind `--experimental-formats` at the engine dispatch, so this code is
//! unreachable on the default path.

use object::macho::{S_ATTR_PURE_INSTRUCTIONS, S_GB_ZEROFILL, S_ZEROFILL, SECTION_TYPE};
use object::{Architecture, SectionFlags, SectionKind};

use kuna_sleigh::loadimage::section_flags;

use super::{FormatKind, ImportSym, ObjectFormat};

/// The Mach-O (Apple) object format.
pub struct MachOFormat;

impl ObjectFormat for MachOFormat {
    fn kind(&self) -> FormatKind {
        FormatKind::MachO
    }

    fn compiler_model(&self, arch: Architecture) -> Option<&'static str> {
        // macOS x86-64 follows the System V AMD64 ABI — the *same* cspec Ghidra
        // labels `gcc` for x86-64 (design §2.1 footnote ¹): so this returns
        // `gcc`, NOT an invented `macho` token. arm64 uses the AArch64 `default`
        // cspec (`default`, not `windows`). The model must always be one the
        // vendored `.ldefs` declares.
        match arch {
            Architecture::X86_64 | Architecture::I386 => Some("gcc"),
            // Aarch64 / Arm and anything else: the arch default cspec.
            _ => Some("default"),
        }
    }

    fn section_bits(&self, kind: SectionKind, flags: SectionFlags) -> u32 {
        let mflags = match flags {
            SectionFlags::MachO { flags } => flags,
            _ => 0,
        };
        let sec_type = mflags & SECTION_TYPE;
        let pure_instr = mflags & S_ATTR_PURE_INSTRUCTIONS != 0;
        // Zero-fill section types are the `.bss` analog (no file content).
        let zerofill = sec_type == S_ZEROFILL || sec_type == S_GB_ZEROFILL;

        let mut out = 0u32;
        // NOLOAD: zero-fill (uninitialized) sections, or whatever the neutral
        // `SectionKind` already classifies as uninitialized.
        if zerofill || matches!(kind, SectionKind::UninitializedData) {
            out |= section_flags::NOLOAD;
        }
        // READONLY: Mach-O section flags carry no per-section write bit (it lives
        // in the segment initprot). Use the neutral `SectionKind`: a read-only
        // data / text section is readonly; writable data is not.
        if matches!(kind, SectionKind::ReadOnlyData | SectionKind::Text) {
            out |= section_flags::READONLY;
        }
        // CODE: a `__text`-kind section or a pure-instructions section.
        if pure_instr || matches!(kind, SectionKind::Text) {
            out |= section_flags::CODE;
        }
        if matches!(kind, SectionKind::Data | SectionKind::ReadOnlyData) {
            out |= section_flags::DATA;
        }
        out
    }

    fn resolve_imports(&self, file: &object::File, bytes: &[u8]) -> Vec<ImportSym> {
        // Mach-O import naming: walk the `LC_DYSYMTAB` indirect-symbol table,
        // naming each `__stubs` entry a `bl`/`callq` targets directly + each
        // symbol-pointer slot (the GOT analog) a folded indirect call resolves,
        // and register the exports — all in `s1_loader/macho_stubs.rs` (design
        // §3.3). Arch-independent (section metadata, no instruction decode).
        // Pure & total: a non-Mach-O / no-LC_DYSYMTAB / unparsable layout yields
        // an empty `Vec`.
        crate::s1_loader::macho_stubs::resolve_macho_imports(file, bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `MachOFormat::compiler_model` is `gcc` for x86-64 (System V AMD64, the
    /// macOS ABI) and `default` for arm64 — never an invented token.
    #[test]
    fn macho_compiler_model_per_arch() {
        let f = MachOFormat;
        assert_eq!(f.compiler_model(Architecture::X86_64), Some("gcc"), "x86-64 macOS is SysV → gcc");
        assert_eq!(f.compiler_model(Architecture::I386), Some("gcc"));
        assert_eq!(f.compiler_model(Architecture::Aarch64), Some("default"), "arm64 macOS → default");
    }

    /// A `__text` Mach-O section (`SectionKind::Text`, `S_ATTR_PURE_INSTRUCTIONS`)
    /// is CODE | READONLY; a zero-fill section is NOLOAD.
    #[test]
    fn macho_section_bits_text_and_bss() {
        let f = MachOFormat;
        let bits = f.section_bits(
            SectionKind::Text,
            SectionFlags::MachO { flags: S_ATTR_PURE_INSTRUCTIONS },
        );
        assert!(bits & section_flags::CODE != 0, "__text is CODE");
        assert!(bits & section_flags::READONLY != 0, "__text is READONLY");

        let bss = f.section_bits(
            SectionKind::UninitializedData,
            SectionFlags::MachO { flags: S_ZEROFILL },
        );
        assert!(bss & section_flags::NOLOAD != 0, "zerofill section is NOLOAD");
    }
}
