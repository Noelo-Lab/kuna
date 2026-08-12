//! The ELF [`ObjectFormat`] — today's ELF-only load logic, lifted verbatim.
//!
//! Every method here is the *current* behavior moved behind the trait with zero
//! change:
//! - [`ElfFormat::section_bits`] is the body of the old
//!   `loadimage_object::section_kind_flags`.
//! - [`ElfFormat::compiler_model`] returns the same `gcc`/`default` tokens the
//!   old `language_id_for` baked into the id string.
//! - [`ElfFormat::resolve_imports`] just calls
//!   [`crate::loader::elf_plt::resolve_plt_imports`] (its internals are
//!   untouched) and re-wraps each `PltSym` as an [`ImportSym`].
//! - [`ElfFormat::const_ranges`] is `elf_plt::mips_got_const_ranges`.
//!
//! The existing ELF fixtures + the 675 datatests are the proof this is
//! byte-identical (see the module doc on [`super`]).

use object::read::Object;
use object::{Architecture, SectionFlags, SectionKind};

use kuna_sleigh::loadimage::section_flags;

use super::{FormatKind, ImportSym, ObjectFormat};

/// The ELF object format.
pub struct ElfFormat;

impl ObjectFormat for ElfFormat {
    fn kind(&self) -> FormatKind {
        FormatKind::Elf
    }

    fn compiler_model(&self, arch: Architecture) -> Option<&'static str> {
        // The Linux/SysV ELF ABI default compiler model. This reproduces exactly
        // what the old `language_id_for` hard-coded into each id string:
        // `x86`/`RISCV` → `gcc`, every other supported machine → `default`.
        // Returned as the explicit token so `language_id_for` stays a structural
        // change with no output change (the produced id strings are unchanged).
        match arch {
            Architecture::X86_64 | Architecture::I386 => Some("gcc"),
            Architecture::Riscv64 | Architecture::Riscv32 => Some("gcc"),
            _ => Some("default"),
        }
    }

    /// Verbatim the old `loadimage_object::section_kind_flags` body, mirroring
    /// the BFD `SEC_*` → `LoadImageSection` translation in
    /// `LoadImageBfd::getNextSection` (`loadimage_bfd.cc:261`).
    fn section_bits(&self, kind: SectionKind, flags: SectionFlags) -> u32 {
        // ELF section header flags (the BFD `SEC_*` bits derive from these).
        const SHF_WRITE: u64 = 0x1;
        const SHF_ALLOC: u64 = 0x2;
        const SHF_EXECINSTR: u64 = 0x4;

        let sh_flags = match flags {
            SectionFlags::Elf { sh_flags } => sh_flags,
            _ => 0,
        };
        let alloc = sh_flags & SHF_ALLOC != 0;
        let exec = sh_flags & SHF_EXECINSTR != 0;
        let write = sh_flags & SHF_WRITE != 0;

        let mut out = 0u32;
        // (SEC_ALLOC)==0 -> unalloc
        if !alloc {
            out |= section_flags::UNALLOC;
        }
        // SEC_LOAD is set for allocated sections with file contents; an
        // uninitialized (.bss-style) section is NOLOAD.  `SectionKind::UninitializedData`
        // is exactly BFD's `!SEC_LOAD` allocated section.
        if matches!(kind, SectionKind::UninitializedData) || !alloc {
            out |= section_flags::NOLOAD;
        }
        // SEC_READONLY: an allocated, non-writable section (the BFD readonly bit).
        if alloc && !write {
            out |= section_flags::READONLY;
        }
        // SEC_CODE / SEC_DATA.
        if exec || matches!(kind, SectionKind::Text) {
            out |= section_flags::CODE;
        }
        if matches!(kind, SectionKind::Data | SectionKind::ReadOnlyData) {
            out |= section_flags::DATA;
        }
        out
    }

    fn resolve_imports(&self, file: &object::File, _bytes: &[u8]) -> Vec<ImportSym> {
        // `elf_plt`'s internals are unchanged; just re-wrap each PltSym.
        crate::loader::elf_plt::resolve_plt_imports(file)
            .into_iter()
            .map(|p| ImportSym { addr: p.addr, name: p.name })
            .collect()
    }

    fn const_ranges(&self, file: &object::File, _bytes: &[u8]) -> Vec<(u64, u64)> {
        crate::loader::elf_plt::mips_got_const_ranges(file)
    }

    /// An `ET_REL` object carries no program headers, so `file.segments()` is
    /// empty and the mapped-image path would map zero bytes — the historical
    /// gate, moved behind the boundary verbatim.
    fn relocatable_layout(&self, file: &object::File) -> bool {
        file.kind() == object::ObjectKind::Relocatable && file.segments().next().is_none()
    }

    fn is_alloc_section(&self, _kind: SectionKind, flags: SectionFlags) -> bool {
        const SHF_ALLOC: u64 = 0x2;
        matches!(flags, SectionFlags::Elf { sh_flags } if sh_flags & SHF_ALLOC != 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `ElfFormat::compiler_model` returns exactly the token the old
    /// `language_id_for` baked into each id string — `gcc` for x86/RISCV,
    /// `default` for every other supported machine. This is the byte-identical
    /// proof for the §2.2 structural change: the compiler-model field is
    /// unchanged per arch.
    #[test]
    fn elf_compiler_model_matches_today() {
        let f = ElfFormat;
        for a in [Architecture::X86_64, Architecture::I386, Architecture::Riscv64, Architecture::Riscv32]
        {
            assert_eq!(f.compiler_model(a), Some("gcc"), "{a:?} must keep :gcc");
        }
        for a in [
            Architecture::Aarch64,
            Architecture::Arm,
            Architecture::Mips,
            Architecture::PowerPc,
            Architecture::PowerPc64,
            Architecture::Sparc,
            Architecture::Sparc64,
        ] {
            assert_eq!(f.compiler_model(a), Some("default"), "{a:?} must keep :default");
        }
    }

    /// `ElfFormat::section_bits` reproduces the old `section_kind_flags` body: an
    /// ALLOC|EXECINSTR `.text` is CODE|READONLY (alloc, non-writable, exec); a
    /// non-ELF flag set falls through to the `SectionKind` arm.
    #[test]
    fn elf_section_bits_text_is_code_readonly() {
        use kuna_sleigh::loadimage::section_flags;
        let f = ElfFormat;
        // SHF_ALLOC|SHF_EXECINSTR, non-writable.
        let bits = f.section_bits(SectionKind::Text, SectionFlags::Elf { sh_flags: 0x2 | 0x4 });
        assert!(bits & section_flags::CODE != 0, "exec section is CODE");
        assert!(bits & section_flags::READONLY != 0, "alloc+!write is READONLY");
        assert!(bits & section_flags::UNALLOC == 0, "alloc section is not UNALLOC");
        // SHF_ALLOC|SHF_WRITE data section: DATA, not READONLY.
        let bits = f.section_bits(SectionKind::Data, SectionFlags::Elf { sh_flags: 0x2 | 0x1 });
        assert!(bits & section_flags::DATA != 0, "data section is DATA");
        assert!(bits & section_flags::READONLY == 0, "writable section is not READONLY");
    }
}
