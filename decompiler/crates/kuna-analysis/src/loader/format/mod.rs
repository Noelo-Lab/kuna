//! Object-format dispatch for ELF, PE, Mach-O and COFF.
//!
//! [`ObjectFormat`] supplies ABI defaults, section flags, imported/exported
//! symbols, header mappings, import-pointer slots and relocatable-object layout.
//! [`detect`] selects the implementation from a parsed object's format. The
//! shared loader handles image mapping and symbol installation through this
//! interface; unsupported formats return an error.

use object::{Architecture, BinaryFormat, SectionFlags, SectionKind};

use kuna_base::error::{KunaError, KunaResult};

pub mod coff;
pub mod elf;
pub mod elf_userland;
pub mod macho;
pub mod pe;

/// Provenance for a symbol emitted by the format resolver.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ImportSymKind {
    /// A genuine imported call target: slot, stub, or veneer.
    Import,
    /// A function exported by the image itself.
    Export,
}

/// One resolved library-facing symbol: the address a CALL resolves to and its
/// clean name. [`ImportSymKind`] distinguishes genuine imported call targets
/// from the defined exports PE and Mach-O append to the same loader stream.
///
/// ELF PLT stubs, PE IAT slots and Mach-O stubs/exports share this shape
/// for the loader's function-symbol commit path.
pub struct ImportSym {
    /// Resolved function-symbol address.
    pub addr: u64,
    /// Resolved function name (raw object-string bytes, ABI decoration stripped).
    pub name: Vec<u8>,
    /// Whether this address represents an imported target or a defined export.
    pub kind: ImportSymKind,
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
    /// Does the region hold code? The header page is data on every image a
    /// compiler produces, and the map records it as such. It is code when the
    /// image itself says so — when `AddressOfEntryPoint` points into it, which
    /// is a packer laying its stub in the slack after the section table. See
    /// [`crate::loader::pe_headers::declared_entry_in_header`].
    pub code: bool,
}

/// Which object format an [`ObjectFormat`] implements.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum FormatKind {
    Elf,
    Pe,
    MachO,
    Coff,
}

/// Format-specific information consumed by the shared object loader.
pub trait ObjectFormat {
    /// Which format this is.
    fn kind(&self) -> FormatKind;

    /// SLEIGH compiler-model id for this format's default ABI, per arch.
    ///
    /// ELF/SysV → `gcc` (x86) / `default` (others); PE → `windows`; Mach-O →
    /// `gcc` (x86-64 SysV) / `default` (arm64). `None` = no ABI opinion (use the
    /// arch default). The token must name a compiler in the vendored `.ldefs`;
    /// an unknown token yields the "No sleigh specification" error.
    fn compiler_model(&self, arch: Architecture) -> Option<&'static str>;

    /// Per-format arm of today's `section_kind_flags` (translate an `object`
    /// section's name, kind and flags into the kuna `section_flags` bitset).
    fn section_bits(&self, name: &str, kind: SectionKind, flags: SectionFlags) -> u32;

    /// Format-dispatching replacement for `elf_plt::resolve_plt_imports`.
    ///
    /// ELF: PLT/GOT/`.dynamic`. PE: IAT/INT. Mach-O: `__stubs` /
    /// indirect-symbols. COFF object: none. Pure & total: never
    /// panics/errors; an unknown layout yields an empty `Vec`. PE and Mach-O
    /// also append image exports, marked distinctly with
    /// [`ImportSymKind::Export`].
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

    /// The **import pointer slots** of the image as half-open `[lo, hi)` VMA
    /// ranges: the words a run-time loader fills with a resolved import address.
    ///
    /// [`ObjectFormat::resolve_imports`] registers a `FunctionSymbol` at each of
    /// them so a `call [slot]` renders the import name, but such an address is a
    /// pointer word, not a function entry. This is the address half of that same
    /// answer, so a consumer that enumerates *bodies* can tell the two apart
    /// without re-deriving the layout. Implemented for PE IAT entries and typed
    /// Mach-O lazy/non-lazy symbol-pointer indirect entries.
    fn import_slots(&self, _file: &object::File, _bytes: &[u8]) -> Vec<(u64, u64)> {
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

/// Select a format implementation for ELF, PE, Mach-O or COFF.
/// Unsupported object formats return an error.
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
/// format. An unsupported format yields an empty `Vec`.
pub fn resolve_imports(file: &object::File, bytes: &[u8]) -> Vec<ImportSym> {
    match detect(file) {
        Ok(fmt) => fmt.resolve_imports(file, bytes),
        Err(_) => Vec::new(),
    }
}

/// Free dispatch over [`detect`] + [`ObjectFormat::import_slots`], the address
/// half of [`resolve_imports`]. Same contract: never panics, and a format
/// `detect` rejects yields an empty `Vec`.
pub fn import_slots(file: &object::File, bytes: &[u8]) -> Vec<(u64, u64)> {
    match detect(file) {
        Ok(fmt) => fmt.import_slots(file, bytes),
        Err(_) => Vec::new(),
    }
}
