//! The `ObjectFormat` boundary — the single funnel for every piece of
//! format-specific knowledge the loader needs (the kuna analog of Ghidra's
//! per-format `Loader`/`*ProgramBuilder` hierarchy, here distilled to the four
//! things kuna's ELF-faithful load path actually keys off).
//!
//! ## Why a trait
//!
//! Before this boundary the load path (`crate::loadimage_object::from_bytes`) was
//! ELF-only by construction: `section_kind_flags`, `resolve_plt_imports`, the
//! `:gcc` compiler model, and the MIPS GOT const-ranges were all hard-wired to
//! the ELF case. This module lifts those four chokepoints behind one trait so
//! that ELF becomes *an implementation* rather than the privileged default, and
//! a future PE / Mach-O / COFF impl is "write one [`ObjectFormat`] + add a
//! [`detect`] arm," touching no shared pass and no engine code.
//!
//! ## Faithfulness (PR-1)
//!
//! This is a **pure refactor**: [`elf::ElfFormat`] is today's logic moved
//! verbatim (the `section_kind_flags` body, `elf_plt::resolve_plt_imports`, the
//! `:gcc`/`:default` compiler model, `elf_plt::mips_got_const_ranges`). Only ELF
//! is reachable — the dispatch in `kuna-console`'s engine still routes only
//! `\x7fELF` to the object loader — so the existing ELF fixtures + the 675
//! datatests prove the lift is byte-identical. PE/Mach-O/COFF are *named* in
//! [`FormatKind`] and rejected by [`detect`] (their `object` features are not
//! even enabled yet); their impls land in PR-2+.

use object::{Architecture, BinaryFormat, SectionFlags, SectionKind};

use kuna_base::error::{KunaError, KunaResult};

pub mod coff;
pub mod elf;
pub mod macho;
pub mod pe;

/// One resolved imported symbol: the address a CALL to this import resolves to
/// (a code stub the disassembler sees, or a data slot the engine constant-folds)
/// and the clean imported name.
///
/// Structurally identical to today's [`elf::PltSymCompat`] /
/// `elf_plt::PltSym` — the universal currency of the import/symbol boundary, so a
/// PE IAT slot, a Mach-O `__stubs` entry, and an ELF PLT stub all flow through
/// the same downstream commit path (`seen`-dedup → `FuncSym` → `FunctionSymbol`).
pub struct ImportSym {
    /// The address a CALL to this import resolves to (`FunctionSymbol` address).
    pub addr: u64,
    /// Imported function name (raw object-string bytes, version-suffix stripped).
    pub name: Vec<u8>,
}

/// The file-backed **header page** a format maps ahead of its first section:
/// the bytes at file offset 0 that appear at virtual address `vma` at run time.
///
/// Only PE publishes one today (`SizeOfHeaders` bytes at `ImageBase`, which
/// Windows maps `PAGE_READONLY`). An ELF's `PT_LOAD` program headers already
/// describe the whole mapping, including any header bytes that are in it, so
/// there is nothing left over to add.
pub struct HeaderRegion {
    /// Virtual address the header bytes are mapped at.
    pub vma: u64,
    /// How many bytes from file offset 0 are mapped there.
    pub len: usize,
}

/// Which object format an [`ObjectFormat`] implements.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum FormatKind {
    Elf,
    Pe,
    MachO,
    Coff,
}

/// Everything kuna genuinely needs to know that differs per object format.
///
/// ELF is one implementation ([`elf::ElfFormat`]); PE/Mach-O/COFF are siblings
/// added in later PRs. The format-neutral body of `ObjectLoadImage`
/// (segments/sections/`load_fill`/symbol dedup) is unchanged — it rides the
/// `object` crate's neutral `ObjectSegment`/`ObjectSection`/`ObjectSymbol`
/// traits and asks an `ObjectFormat` only for the four things below.
pub trait ObjectFormat {
    /// Which format this is.
    fn kind(&self) -> FormatKind;

    /// SLEIGH compiler-model id for this format's default ABI, per arch.
    ///
    /// ELF/SysV → `gcc` (x86) / `default` (others); PE → `windows`; Mach-O →
    /// `gcc` (x86-64 SysV) / `default` (arm64). `None` = no ABI opinion (use the
    /// arch default). MUST return a token the vendored `.ldefs` actually declares
    /// (validated by the PR-1 console test) — an invented token yields the
    /// existing "No sleigh specification" error.
    fn compiler_model(&self, arch: Architecture) -> Option<&'static str>;

    /// Per-format arm of today's `section_kind_flags` (translate an `object`
    /// section kind + flags into the kuna `section_flags` bitset).
    fn section_bits(&self, kind: SectionKind, flags: SectionFlags) -> u32;

    /// Format-dispatching replacement for `elf_plt::resolve_plt_imports`.
    ///
    /// ELF: PLT/GOT/`.dynamic`. PE: IAT/INT (PR-2+). Mach-O: `__stubs` /
    /// indirect-symbols (PR-2+). COFF object: none. Pure & total: never
    /// panics/errors; an unknown layout yields an empty `Vec`.
    ///
    /// `bytes` is the raw image (some formats — PE/Mach-O — need a typed
    /// re-parse the neutral `object::File` view does not expose); the ELF impl
    /// ignores it.
    fn resolve_imports(&self, file: &object::File, bytes: &[u8]) -> Vec<ImportSym>;

    /// The image's header page, if the format maps one outside its sections.
    ///
    /// PE only (`SizeOfHeaders` bytes at `ImageBase`); every other format
    /// inherits the `None` default and keeps a section/segment-derived map
    /// byte-for-byte. See [`crate::loader::pe_headers`].
    fn header_region(&self, _file: &object::File, _bytes: &[u8]) -> Option<HeaderRegion> {
        None
    }

    /// Read-only VMA ranges to constant-fold beyond the section-flag scan (the
    /// MIPS GOT external slots today; usually empty).
    fn const_ranges(&self, _file: &object::File, _bytes: &[u8]) -> Vec<(u64, u64)> {
        Vec::new()
    }

    /// Whether this file is a **pre-link relocatable object** whose sections must
    /// be laid out synthetically ([`crate::loader::reloc_object`]) instead of read
    /// off the linked image's own mapping.
    ///
    /// The condition is per-format because "the image tells you where its bytes
    /// live" fails differently in each: an `ET_REL` ELF simply has no `PT_LOAD`
    /// program headers, while a COFF `.obj` *does* present its sections as
    /// segments — every one of them at VMA 0, all overlapping (design §3.6). Only
    /// a relocatable object ever answers `true`; a linked image of any format
    /// keeps the faithful mapped-image path.
    fn relocatable_layout(&self, _file: &object::File) -> bool {
        false
    }

    /// Whether a section occupies memory at run time, and so takes a load VMA in
    /// the synthetic layout ([`ObjectFormat::relocatable_layout`]).
    ///
    /// The ELF `SHF_ALLOC` bit and its COFF `Characteristics` analog. Only
    /// consulted for a relocatable object, so the formats that never claim one
    /// inherit the `false` default.
    fn is_alloc_section(&self, _kind: SectionKind, _flags: SectionFlags) -> bool {
        false
    }
}

/// Select the [`ObjectFormat`] for a parsed object.
///
/// All four formats are constructible — `object`'s `pe`/`macho`/`coff` readers
/// are enabled, and the engine dispatch ([`is_object_binary`]) routes ELF / PE /
/// Mach-O / COFF magics here unconditionally (multi-format is the default since
/// increment 46). The XML/datatest corpus never carries an object-format magic,
/// so in practice `detect` is only ever called on a real object binary.
///
/// [`is_object_binary`]: ../../../../../kuna_console/engine/fn.is_object_binary.html
pub fn detect(file: &object::File) -> KunaResult<Box<dyn ObjectFormat>> {
    match file.format() {
        BinaryFormat::Elf => Ok(Box::new(elf::ElfFormat)),
        BinaryFormat::Pe => Ok(Box::new(pe::PeFormat)),
        BinaryFormat::MachO => Ok(Box::new(macho::MachOFormat)),
        BinaryFormat::Coff => Ok(Box::new(coff::CoffFormat)),
        other => Err(KunaError::lowlevel(format!(
            "unsupported object format {other:?} \
             (kuna supports ELF/PE/Mach-O/COFF)"
        ))),
    }
}

/// Free dispatch over [`detect`] + [`ObjectFormat::resolve_imports`], for the
/// call sites that only carry a parsed `file` (and the raw `bytes`) and have no
/// `ObjectFormat` in hand (`entry::existing_function_addrs`,
/// `loader::noreturn::scan_noreturn`).
///
/// They need *no* format branch: this one function does the right thing per
/// format. A format `detect` rejects (today: anything non-ELF) yields an empty
/// `Vec` — additive, never fails, matching the `elf_plt` contract.
pub fn resolve_imports(file: &object::File, bytes: &[u8]) -> Vec<ImportSym> {
    match detect(file) {
        Ok(fmt) => fmt.resolve_imports(file, bytes),
        Err(_) => Vec::new(),
    }
}
