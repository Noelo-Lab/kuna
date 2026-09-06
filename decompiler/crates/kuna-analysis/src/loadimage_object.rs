//! Real-ELF [`LoadImage`] backend (W11, item `w11-elf-loader`) — the kuna
//! analog of `decompiler/cpp/loadimage_bfd.{cc,hh}` (`LoadImageBfd`, the GNU
//! BFD-backed loader used by the C++ console for real binaries).
//!
//! ## Why a substitution, not a transcription
//!
//! `LoadImageBfd` links the GNU BFD library (GPL-3, *excluded from the upstream
//! build* — see the `loadimage_bfd.cc` header note).  The kuna Rust port cannot
//! carry a GPL-3 link dependency into an Apache-2.0 tree, so per the plan's
//! dependency-substitution LOSS the BFD object model is replaced by the
//! permissively-licensed [`object`] crate (read-only ELF parsing).  The
//! *semantics* of the C++ `LoadImage` interface are preserved exactly; only the
//! object-file backend differs.  See `docs/rust-port/losses.md`.
//!
//! ## Faithful semantics (vs `loadimage_bfd.cc`)
//!
//! - `loadFill(ptr,size,addr)` reproduces the BFD loader byte-for-byte: a
//!   512-byte read buffer (`bufoffset`/`bufsize`/`buffer`), the same
//!   `findSection`-style "containing segment, else closest-greater segment"
//!   walk, the same gap zero-fill (`memset` to the next segment), the same
//!   "initial address not mapped -> `break` -> DataUnavailError" contract, and
//!   the same final `memcpy` of the requested window out of the buffer.  The
//!   *unit* of mapping is the ELF **loadable segment** (`PT_LOAD`, the bytes
//!   actually present in RAM at run time) rather than a BFD `asection`; this is
//!   the faithful choice for a real process image (BFD's section vmas and the
//!   `PT_LOAD` vmas coincide for the code/data a decompiler reads, and segments
//!   are what the loader maps).
//! - `getArchType()` returns the SLEIGH **language id** for the ELF machine
//!   (e.g. `x86:LE:64:default:gcc`).  C++ `LoadImageBfd::getArchType` returns a
//!   BFD-internal `"<printable>:<target>"` string that the Ghidra Java side
//!   re-maps; kuna has no such map, so the loader resolves the language id
//!   directly off the ELF header (machine + endianness + class), which is
//!   exactly what `SleighArchitecture::resolveArchitecture` consumes.  The
//!   compiler field (`:gcc`) is the System V/Linux default (the only ABI a bare
//!   ELF identifies); other ABIs are a hook.
//! - `adjustVma(adjust)` shifts every segment/section/symbol vma by
//!   `addressToByte(adjust,wordsize)`, exactly as `LoadImageBfd::adjustVma`
//!   walks `thebfd->sections` adding the byte-scaled adjustment.
//! - `openSymbols`/`getNextSymbol` iterate function symbols (BFD `BSF_FUNCTION`
//!   with a non-null name -> [`object::SymbolKind::Text`] with a non-empty
//!   name), reporting `(name, address)` in symbol-table order.
//! - `openSectionInfo`/`getNextSection` and `getReadonly` walk the ELF sections
//!   with the BFD flag translation (`SEC_ALLOC`/`SEC_LOAD`/`SEC_READONLY`/
//!   `SEC_CODE`/`SEC_DATA` -> [`section_flags`]).
//!
//! ## Scope (PARTIAL)
//!
//! Machine -> language-id mapping is wired for the common Linux/SysV ELF
//! machines kuna ships a `.sla` for (x86 32/64, ARM/AArch64, MIPS, PPC, SPARC,
//! RISC-V).  An unmapped machine surfaces a `LowlevelError` naming the machine
//! (the caller falls back to an explicit `--target`).  Non-ELF object formats
//! (PE/Mach-O) are a hook — this loader is ELF-only, matching the W11 task.

use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

use object::read::{Object, ObjectSection, ObjectSegment, ObjectSymbol};
use object::{Architecture, SegmentFlags, SymbolKind};

use kuna_base::address::{Address, RangeList};
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::space::AddrSpace;
use kuna_base::types::Wrap;

use kuna_sleigh::loadimage::{section_flags, LoadImage, LoadImageFunc, LoadImageSection};

use kuna_decomp::kuna_symbolnamechars::{sanitize_symbol_name_bytes, symbolnamechars_mode, NameChars};

/// Default read-buffer size (C++ `LoadImageBfd::bufsize`, `loadimage_bfd.cc:36`).
const BUFSIZE: usize = 512;

/// (kuna) Whether the ET_REL relocatable-object load path is enabled (the
/// `relocobjects` option, default ON).  The loader runs at `load file`, upstream
/// of the per-function option machinery, so the toggle is bridged across layers
/// by the [`RELOC_OBJECTS_ENV`](kuna_decomp::options::RELOC_OBJECTS_ENV) process
/// env var that `Architecture::set_kuna_option("relocobjects", ...)` writes — any
/// of `0`/`off`/`false`/`no` disables it; anything else (or unset) is ON.
pub(crate) fn reloc_objects_enabled() -> bool {
    match std::env::var(kuna_decomp::options::RELOC_OBJECTS_ENV) {
        Ok(v) => !matches!(v.trim(), "0" | "off" | "false" | "no" | "OFF"),
        Err(_) => true,
    }
}

/// Demangle a loader funcsym name (the kuna analog of Ghidra's
/// `GnuDemanglerAnalyzer`; see [`crate::demangle`]).  Applied to every
/// `.symtab` / PLT / `.dynsym` name *after* `@VERSION` stripping and *before* it
/// is installed as a `FunctionSymbol`, so `_ZN3foo3barEv` -> `foo::bar` and
/// kuna's `::`-namespace splitter nests the call.  Operates on the byte-string
/// name: a non-UTF-8 name (or one that is not a recognized mangled symbol) is
/// returned unchanged.  Reduces to the qualified name only (no signature /
/// template args), which is a hard requirement of the scope splitter.
///
/// (kuna `symbolnamechars`, GH-340) The character sanitizer runs LAST, on the
/// reduced name: before it, `safe`'s byte pass would be looking at the mangled
/// `_ZN…` envelope rather than at what actually reaches emitted C, and `ident`
/// would fold the demangler's own output twice.  It still runs before
/// `find_create_scope_from_symbol_name`, so the sanitizer and `symbolnamerepair`
/// never contend over the same empty component.
fn demangle_funcsym_name(name: Vec<u8>, mode: NameChars) -> Vec<u8> {
    let name = match std::str::from_utf8(&name) {
        Ok(s) => match crate::demangle::demangle_name(s) {
            Some(d) => d.into_bytes(),
            None => name,
        },
        Err(_) => name, // not a textual symbol; leave the raw bytes
    };
    match sanitize_symbol_name_bytes(&name, mode) {
        std::borrow::Cow::Borrowed(_) => name,
        std::borrow::Cow::Owned(v) => v,
    }
}

/// One loadable region of the image (the analog of a BFD `asection` for the
/// purpose of [`ObjectLoadImage::find_section`]/`loadFill`): a vma, the bytes
/// that live there, and BFD-style section flags.
#[derive(Debug, Clone)]
struct Segment {
    /// Virtual address the bytes map to (C++ `asection::vma`).
    vma: u64,
    /// The bytes present at `vma` (a copy of the ELF segment's file data; the
    /// region's `size` is `data.len()`, the C++ `secsize`).
    data: Vec<u8>,
}

/// A load segment's [`section_flags`] bits, from the format's own permission
/// word — the segment-level counterpart of the per-format `section_bits`.
///
/// Execute permission is what the CODE bit means here; everything else mapped
/// is DATA, and a segment that cannot be written is READONLY. A permission word
/// in a format this does not model yields no bits, so a reader that keys on
/// CODE ignores the segment rather than guessing.
fn segment_bits(flags: SegmentFlags) -> u32 {
    let (exec, write) = match flags {
        SegmentFlags::Elf { p_flags, .. } => (p_flags & 1 != 0, p_flags & 2 != 0),
        SegmentFlags::MachO { initprot, .. } => (initprot & 4 != 0, initprot & 2 != 0),
        SegmentFlags::Coff { characteristics } => {
            (characteristics & 0x2000_0000 != 0, characteristics & 0x8000_0000 != 0)
        }
        _ => return 0,
    };
    let mut bits = if exec { section_flags::CODE } else { section_flags::DATA };
    if !write {
        bits |= section_flags::READONLY;
    }
    bits
}

/// One ELF section, for the `getNextSection`/`getReadonly` info walks (the BFD
/// `asection` list).
#[derive(Debug, Clone)]
struct SectionInfo {
    /// Section vma (C++ `asection::vma`).
    vma: u64,
    /// Section size (C++ `asection::size`).
    size: u64,
    /// kuna [`section_flags`] (the translated BFD `SEC_*` flags).
    flags: u32,
}

/// One function symbol (the BFD `BSF_FUNCTION` entries the loader iterates).
#[derive(Debug, Clone)]
struct FuncSym {
    /// Symbol vma (C++ `bfd_asymbol_value`).
    addr: u64,
    /// Symbol name (byte string, per the workspace marshal convention).
    name: Vec<u8>,
}

/// One **data** symbol: a defined `.symtab`/`.dynsym` `STT_OBJECT` entry (the BFD
/// `BSF_OBJECT` twin of [`FuncSym`]).
///
/// The function half of the symbol table has always been read (the `funcsyms`
/// stream that names `main`/`fmt`); the data half was dropped, so an imported
/// libc global — `optind`, `stdin`, `stdout`, `optarg` — rendered `dat_20a098`
/// where IDA Pro and Ghidra show its name. Both of those name data objects from
/// the symbol table independently of any debug info, which is what reaches a
/// copy-relocated (`R_X86_64_COPY`) extern: it has a real defined address in the
/// importing binary's `.bss` and a `STT_OBJECT` `.dynsym` entry, but never
/// appears in the program's own `.debug_info`.
///
/// `size` is the symbol's `st_size`, carried so the commit can map a covering
/// `SymbolEntry` of the right extent — a size-1 entry does not contain a 4-/8-byte
/// access, which is exactly the bug the DWARF data globals hit (see
/// [`crate::pass::DataObjectFact`]).
#[derive(Debug, Clone)]
struct DataSym {
    /// Symbol vma.
    addr: u64,
    /// Symbol name, `@VERSION` suffix already stripped.
    name: Vec<u8>,
    /// `st_size` (bytes); `0` when the object declares no size.
    size: u64,
}

/// \brief A [`LoadImage`] over a real ELF executable, backed by the [`object`]
/// crate (the kuna substitution for C++ `LoadImageBfd`).
///
/// The ELF is parsed once at construction into the in-memory model (segments,
/// sections, function symbols, the resolved language id) so the struct owns no
/// borrow of the file buffer — `loadFill` reads from the owned segment bytes,
/// exactly as `LoadImageBfd` reads from BFD's section contents.
#[derive(Debug)]
pub struct ObjectLoadImage {
    /// Name of the loadimage (the `LoadImage` base-class `filename` member).
    filename: String,
    /// The resolved SLEIGH language id (the `getArchType` payload).
    archtype: Vec<u8>,
    /// The per-arch *default-model* fallback language id (design §2.2): the same
    /// arch/endian stem with the compiler model forced to the arch default
    /// (`gcc`/`default`). Used by the engine when [`Self::archtype`] (which may
    /// carry a format-specific model like `:windows`) does not resolve in the
    /// vendored SLEIGH DB — wrong calling-convention details beat no decompile.
    /// `None` when the fallback equals the primary (the common ELF case).
    fallback_archtype: Option<Vec<u8>>,
    /// Loadable regions, in *ascending vma order* (the BFD section list, used by
    /// `find_section`/`loadFill`).
    segments: Vec<Segment>,
    /// ELF sections, in file order (the BFD `asection` list for the info walks).
    sections: Vec<SectionInfo>,
    /// (kuna) The loadable segments as mapping *metadata*, ascending by vma —
    /// the same `(vma, size, flags)` shape as [`Self::sections`], reported by
    /// `getSegments` so a section-keyed reader has a container to fall back on
    /// when the image carries no usable section table. Distinct from
    /// [`Self::segments`], which owns the bytes and covers only what the file
    /// backs; this records each segment's whole RAM footprint. Empty for an
    /// `ET_REL` load, which has no program headers.
    segment_info: Vec<SectionInfo>,
    /// Function symbols, in symbol-table order.
    funcsyms: Vec<FuncSym>,
    /// Relocatable-object section coordinates. Empty for linked images.
    reloc_sections: Vec<crate::loader::reloc_object::RelocSectionInfo>,
    /// Relocatable-object function provenance. Empty for linked images.
    reloc_symbols: Vec<crate::loader::reloc_object::RelocSymbolInfo>,
    /// Defined data symbols (`STT_OBJECT`), in symbol-table order. See
    /// [`DataSym`]; surfaced through [`ObjectLoadImage::data_symbols`].
    datasyms: Vec<DataSym>,
    /// (kuna) Extra `[start, stop]` (inclusive) byte ranges to treat as *constant*
    /// (read-only) beyond the section-flag scan — the MIPS GOT external slots,
    /// whose static contents (the `.MIPS.stubs` stub addresses) the engine folds
    /// so a `lw $t9, off($gp); jalr $t9` indirect call resolves to the import name.
    /// Empty off-MIPS.  See [`crate::loader::elf_plt::mips_got_const_ranges`].
    const_ranges: Vec<(u64, u64)>,
    /// (kuna) `[start, stop]` (inclusive) byte ranges whose contents the engine
    /// may fold to a constant **without** the program-wide `option readonly` —
    /// values that are known by construction rather than by policy. Reported
    /// through `get_readonly` (so the varnodes carry `Varnode::readonly`) *and*
    /// through [`ObjectLoadImage::dynreloc_const_ranges`] (so the fold is enabled
    /// for exactly these ranges without turning on global read-only propagation).
    /// Two producers, each on its own path and never both:
    ///
    /// * `dynrelocs` (linked image): the dynamic-relocation slots this loader
    ///   filled in that `PT_GNU_RELRO` freezes — the GOT entries whose relocated
    ///   value turns `(*dat_e0dc8)(…)` back into a named call. See
    ///   [`crate::loader::kuna_dynrelocs`].
    /// * `msvcfpconst` (relocatable object): the MSVC `__real@` floating-point
    ///   COMDATs, whose value is spelled in the symbol name. See
    ///   [`crate::loader::kuna_msvcfpconst`].
    ///
    /// Empty for a non-ELF linked image, and with both gates off.
    dynreloc_const: Vec<(u64, u64)>,
    /// The address space the file bytes map to (C++ `spaceid`, null until
    /// `attachToSpace`).
    spaceid: Option<Rc<AddrSpace>>,
    /// Read buffer (C++ `buffer`); reused across `loadFill` calls.
    buffer: RefCell<Vec<u8>>,
    /// Starting offset of the buffered bytes (C++ `bufoffset`; `!0` = "nothing
    /// buffered", the C++ sentinel).
    bufoffset: RefCell<u64>,
    /// `openSymbols`/`getNextSymbol` cursor (C++ `mutable cursymbol`).
    cursymbol: RefCell<usize>,
    /// `openSectionInfo`/`getNextSection` cursor (C++ `mutable secinfoptr`).
    cursection: RefCell<usize>,
}

/// Admit one defined `STT_OBJECT` symbol into the data-symbol stream, deduped by
/// address (first table wins, so `.symtab` beats `.dynsym` exactly as the funcsym
/// walk orders them).
///
/// The filters mirror the funcsym walk: an *undefined* entry is an import
/// placeholder with no address of its own, an empty name carries no information,
/// and an `@VERSION` suffix is stripped so a versioned glibc export
/// (`optind@@GLIBC_2.2.5`) installs as `optind`.  A **zero `st_size`** entry is
/// dropped as well — with no extent there is nothing to size a covering
/// `SymbolEntry` against, and the linker's section-boundary markers
/// (`__bss_start`, `_edata`, `_end`) are exactly the zero-size entries that would
/// otherwise plant a spurious name on the first byte of an unrelated object.
fn collect_data_sym<'a>(
    sym: &impl object::ObjectSymbol<'a>,
    out: &mut Vec<DataSym>,
    seen: &mut HashSet<u64>,
    mode: NameChars,
) {
    if sym.is_undefined() {
        return;
    }
    let size = sym.size();
    if size == 0 {
        return;
    }
    let name = match sym.name_bytes() {
        Ok(n) if !n.is_empty() => crate::loader::elf_plt::strip_version(n),
        _ => return,
    };
    if name.is_empty() {
        return;
    }
    // (kuna `symbolnamechars`, GH-340) A data symbol's name reaches emitted C
    // exactly as a function symbol's does, so it is sanitized at the same mint.
    let name = sanitize_symbol_name_bytes(&name, mode).into_owned();
    let addr = sym.address();
    if seen.insert(addr) {
        out.push(DataSym { addr, name, size });
    }
}

/// (kuna `dynrelocs`) Store `value` into the `width`-byte slot at `vma`, in the
/// image's own byte order (`le`), if a loaded segment actually backs it.
///
/// Returns whether the write landed: a slot in a `.bss`-style RAM tail past the
/// segment's file extent has no bytes to patch and is skipped rather than
/// synthesized, so the loader's zero-fill contract is untouched.
fn patch_segments(segments: &mut [Segment], vma: u64, value: u64, width: usize, le: bool) -> bool {
    for seg in segments.iter_mut() {
        if vma < seg.vma {
            continue;
        }
        let off = (vma - seg.vma) as usize;
        let Some(end) = off.checked_add(width) else {
            continue;
        };
        if end > seg.data.len() {
            continue;
        }
        let raw = value.to_le_bytes();
        if le {
            seg.data[off..end].copy_from_slice(&raw[..width]);
        } else {
            for (i, b) in raw[..width].iter().rev().enumerate() {
                seg.data[off + i] = *b;
            }
        }
        return true;
    }
    false
}

impl ObjectLoadImage {
    /// Open an ELF file as a [`LoadImage`] (the analog of
    /// `LoadImageBfd::LoadImageBfd` + `open()`).
    ///
    /// Reads the whole file, parses the ELF, resolves the SLEIGH language id off
    /// the machine/endianness, and snapshots the loadable segments, sections,
    /// and function symbols.  Errors (the C++ `open()` `LowlevelError`s) on an
    /// unreadable file, an unrecognized object format, a non-ELF object, or a
    /// machine with no kuna `.sla`.
    pub fn open(filename: &str) -> KunaResult<ObjectLoadImage> {
        // Read the image file.
        let bytes = std::fs::read(filename).map_err(|e| {
            KunaError::lowlevel(format!("Unable to open image file: {filename}: {e}"))
        })?;
        Self::from_bytes(filename, &bytes)
    }

    /// Open from an in-memory image (the testable core of [`Self::open`]).
    pub fn from_bytes(filename: &str, bytes: &[u8]) -> KunaResult<ObjectLoadImage> {
        Self::from_bytes_with_diagnostics(filename, bytes, true)
    }

    /// Reconstruct a load image for an internal analysis consumer without
    /// repeating diagnostics already emitted by the primary load.
    pub fn from_bytes_silent(filename: &str, bytes: &[u8]) -> KunaResult<ObjectLoadImage> {
        Self::from_bytes_with_diagnostics(filename, bytes, false)
    }

    fn from_bytes_with_diagnostics(
        filename: &str,
        bytes: &[u8],
        emit_diagnostics: bool,
    ) -> KunaResult<ObjectLoadImage> {
        // Parse the object file.
        let file = object::File::parse(bytes).map_err(|e| {
            KunaError::lowlevel(format!(
                "File: {filename} : not in recognized object file format: {e}"
            ))
        })?;
        // (B) Select the per-format `ObjectFormat` behind the boundary. Today only ELF
        // is constructible (`detect` rejects everything else, and the engine
        // dispatch only routes `\x7fELF` here), so this is the verbatim ELF path;
        // PE/Mach-O/COFF impls land behind this same call in later PRs. The error
        // text differs from the old "not an ELF object" string, but the rejected
        // arm is unreachable on the live path (the engine never hands a non-ELF
        // here), so no behavior changes.
        let fmt = crate::loader::format::detect(&file)?;

        let archtype = language_id_for(&file, fmt.as_ref(), bytes, filename)?;
        // (kuna §2.2) The default-model fallback id: the same arch/endian stem
        // with the model dropped to the per-arch default. If the format's chosen
        // model (e.g. PE's `:windows`) isn't vendored for this arch, the engine
        // retries with this before erroring.
        let fallback_archtype = fallback_language_id(&file, &archtype);

        // (kuna) Relocatable-object (`.o` / `.obj`) path: a pre-link object does
        // not say where its bytes live.  An ELF `ET_REL` has no `PT_LOAD` program
        // headers, so `file.segments()` is empty and the linked path below would
        // map zero bytes (every function failing with "Unable to load N bytes"); a
        // COFF object *does* present segments, but stacks every one of them at VMA
        // 0, so the linked path maps only whichever section sorts first and drops
        // every function of the other twelve into a single address-0 collision.
        // Both need the same answer: synthesize a section layout, apply the
        // relocations, and rebase symbols — the angr CLE relocatable backend's job.
        // Gated on the per-format predicate (a linked image of any format is
        // byte-identical — it keeps the mapped path) plus the `relocobjects`
        // off-switch.  See [`crate::loader::reloc_object`].
        if reloc_objects_enabled() && fmt.relocatable_layout(&file) {
            return Self::from_relocatable(
                filename,
                &file,
                fmt.as_ref(),
                archtype,
                emit_diagnostics,
            );
        }

        // Snapshot the loadable segments (PT_LOAD), copying their RAM bytes.
        // `data()` returns only the file-backed bytes; a segment's `size()`
        // (its RAM footprint) may exceed that for `.bss`-style tails, which the
        // BFD loader reports as zeroes via the gap fill — so the copied `data`
        // is the file extent and any RAM tail past it falls into the zero-fill
        // path exactly as an unmapped gap would (BFD `SEC_LOAD`-less sections).
        let mut segments: Vec<Segment> = Vec::new();
        let mut segment_info: Vec<SectionInfo> = Vec::new();
        for seg in file.segments() {
            let vma = seg.address();
            let data = seg.data().map_err(|e| {
                KunaError::lowlevel(format!("File: {filename} : unreadable segment data: {e}"))
            })?;
            // The metadata records the RAM footprint whether or not the file
            // backs it, so a `.bss` tail still reads as mapped.
            if seg.size() != 0 {
                segment_info.push(SectionInfo {
                    vma,
                    size: seg.size(),
                    flags: segment_bits(seg.flags()),
                });
            }
            if data.is_empty() {
                continue;
            }
            segments.push(Segment { vma, data: data.to_vec() });
        }
        // (kuna, `pe-header-entry-mapped`) The header page a format maps ahead
        // of its first section. `file.segments()` enumerates sections, so a PE's
        // `SizeOfHeaders` bytes at `ImageBase` were mapped nowhere and an entry
        // declared inside them read back "not mapped" even with an explicit
        // `--define-function`. Read-only and DATA, as Windows maps it, so the
        // executable-region scans stay out of every PE's MZ/PE bytes.
        // `None` for every other format; see [`crate::loader::pe_headers`].
        let header = fmt.header_region(&file, bytes).map(|region| SectionInfo {
            vma: region.vma,
            size: region.len as u64, // cast: clamped to the file length
            flags: section_flags::DATA | section_flags::READONLY,
        });
        if let Some(region) = &header {
            segments.push(Segment {
                vma: region.vma,
                data: bytes[..region.size as usize].to_vec(), // cast: ditto
            });
            segment_info.push(region.clone());
        }
        segment_info.sort_by_key(|s| s.vma);
        // Ascending vma order so find_section's "closest greater" walk is a
        // simple scan (the BFD list is already address-ordered for ELF).
        segments.sort_by_key(|s| s.vma);

        // Snapshot the sections for the info walks (the BFD `asection` list).
        // (C) The per-format section-flag translation goes through the boundary; for
        // ELF this is the old `section_kind_flags` body, lifted verbatim into
        // `ElfFormat::section_bits`.
        let mut sections: Vec<SectionInfo> = Vec::new();
        for sec in file.sections() {
            let flags = fmt.section_bits(sec.kind(), sec.flags());
            sections.push(SectionInfo { vma: sec.address(), size: sec.size(), flags });
        }
        sections.extend(header);

        // Snapshot the function symbols.  Three sources, deduped by address so an
        // import that appears in several tables is registered exactly once:
        //   1. `.symtab` defined functions (BFD BSF_FUNCTION with a name),
        //   2. PLT stubs → imported library names (the kuna analog of Ghidra's
        //      `ElfDefaultGotPltMarkup`; see [`crate::loader::elf_plt`]),
        //   3. `.dynsym` defined functions, for stripped-but-dynamic binaries
        //      whose `.symtab` is gone.
        // (kuna `symbolnamechars`, GH-340) Read the gate ONCE for the whole walk:
        // a large image carries a few hundred thousand names and the mode is a
        // process env var.
        let namechars = symbolnamechars_mode();
        let mut funcsyms: Vec<FuncSym> = Vec::new();
        let mut seen: HashSet<u64> = HashSet::new();
        // The data half of the same two symbol tables (`STT_OBJECT`), collected in
        // the same walks and deduped on its own address set.  See [`DataSym`].
        let mut datasyms: Vec<DataSym> = Vec::new();
        let mut seen_data: HashSet<u64> = HashSet::new();

        // 1. `.symtab` defined functions.  Skip *undefined* (import-placeholder)
        //    entries — e.g. the ELF UND `puts@@GLIBC_2.2.5` (`st_value == 0`),
        //    whose real address comes from the §3 import resolver, not the symbol
        //    table — and strip any `@VERSION` suffix.
        //
        //    The skip keys off `is_undefined()`, *not* `addr == 0`: a relocatable
        //    COFF object (PR-5) places its first defined function at section-
        //    relative VMA 0 (`compute @ .text+0`), so an `addr == 0` skip would
        //    silently drop it.  `is_undefined()` is the format-faithful predicate —
        //    for every ELF the two agree (UND syms are exactly the `addr == 0`
        //    ones, and no defined ELF/relocatable-ELF symbol sits at VMA 0), so
        //    this is behavior-identical on the ELF arm and additionally admits a
        //    COFF object's defined-at-0 function (design §3.6 object-vs-image).
        for sym in file.symbols() {
            if sym.kind() == SymbolKind::Data {
                collect_data_sym(&sym, &mut datasyms, &mut seen_data, namechars);
                continue;
            }
            if sym.kind() != SymbolKind::Text {
                continue;
            }
            if sym.is_undefined() {
                continue; // UND / import placeholder, not a real code address
            }
            let addr = sym.address();
            let name = match sym.name_bytes() {
                Ok(n) if !n.is_empty() => crate::loader::elf_plt::strip_version(n),
                _ => continue,
            };
            if name.is_empty() {
                continue;
            }
            let name = demangle_funcsym_name(name, namechars);
            if seen.insert(addr) {
                funcsyms.push(FuncSym { addr, name });
            }
        }

        // 2. PLT stubs → imported library names.
        // (F) Per-format import resolution goes through the boundary; for ELF
        // `ElfFormat::resolve_imports` just calls the unchanged
        // `elf_plt::resolve_plt_imports`.
        for p in fmt.resolve_imports(&file, bytes) {
            if seen.insert(p.addr) {
                funcsyms.push(FuncSym { addr: p.addr, name: demangle_funcsym_name(p.name, namechars) });
            }
        }

        // 3. `.dynsym` defined functions (stripped-but-dynamic fallback): a
        //    dynamic binary stripped of `.symtab` still exports its defined
        //    functions in `.dynsym`.
        for sym in file.dynamic_symbols() {
            if sym.kind() == SymbolKind::Data {
                collect_data_sym(&sym, &mut datasyms, &mut seen_data, namechars);
                continue;
            }
            if sym.kind() != SymbolKind::Text {
                continue;
            }
            if sym.is_undefined() {
                continue; // UND import placeholder (mirrors source #1)
            }
            let addr = sym.address();
            let name = match sym.name_bytes() {
                Ok(n) if !n.is_empty() => crate::loader::elf_plt::strip_version(n),
                _ => continue,
            };
            if name.is_empty() {
                continue;
            }
            let name = demangle_funcsym_name(name, namechars);
            if seen.insert(addr) {
                funcsyms.push(FuncSym { addr, name });
            }
        }

        // (kuna) MIPS GOT external slots → constant ranges, so the engine folds the
        // `lw $t9, off($gp)` indirect-call load to the stub address and resolves the
        // call to the import name (the analog of Ghidra's `setConstant` on the GOT
        // pointer entries in `MIPS_ElfExtension.fixupGot`).  Empty off-MIPS.
        // Through the boundary: `ElfFormat::const_ranges` is `mips_got_const_ranges`.
        let const_ranges = fmt.const_ranges(&file, bytes);

        // (kuna `dynrelocs`) A linked image's `PT_LOAD` bytes are the LINKER's,
        // not the run-time loader's: every slot a dynamic relocation fills reads
        // back as 0. Fill them in here, on the owned segment copies, before
        // anything reads them; the RELRO-frozen subset that actually landed is
        // carried separately so the engine can fold those loads.
        let le_image = file.is_little_endian();
        let dynrelocs = crate::loader::kuna_dynrelocs::resolve(&file, bytes);
        let mut applied: HashSet<u64> = HashSet::new();
        for w in &dynrelocs.writes {
            if patch_segments(&mut segments, w.vma, w.value, w.width as usize, le_image) {
                applied.insert(w.vma);
            }
        }
        let dynreloc_const: Vec<(u64, u64)> =
            dynrelocs.const_ranges.into_iter().filter(|(lo, _)| applied.contains(lo)).collect();

        Ok(ObjectLoadImage {
            filename: filename.to_string(),
            archtype,
            fallback_archtype,
            segments,
            sections,
            segment_info,
            funcsyms,
            reloc_sections: Vec::new(),
            reloc_symbols: Vec::new(),
            datasyms,
            const_ranges,
            dynreloc_const,
            spaceid: None,
            buffer: RefCell::new(vec![0u8; BUFSIZE]),
            bufoffset: RefCell::new(!0u64), // ~((uintb)0)
            cursymbol: RefCell::new(0),
            cursection: RefCell::new(0),
        })
    }

    /// (kuna) Build the image from a **relocatable object** (`ET_REL`): lay the
    /// `SHF_ALLOC` sections out above [`reloc_object::RELOC_BASE`], apply the
    /// `.rela.*` relocations, and rebase / extern-bind the symbols — producing
    /// the same `(segments, sections, funcsyms)` triple the linked `PT_LOAD` path
    /// produces.  Funcsym names are demangled + deduped exactly as on the linked
    /// path.  See [`crate::loader::reloc_object`].
    fn from_relocatable(
        filename: &str,
        file: &object::File,
        fmt: &dyn crate::loader::format::ObjectFormat,
        archtype: Vec<u8>,
        emit_diagnostics: bool,
    ) -> KunaResult<ObjectLoadImage> {
        use crate::loader::reloc_object;

        let layout = reloc_object::layout_relocatable(file, fmt);
        let reloc_sections = layout.section_info.clone();
        let reloc_symbols = layout.symbol_info.clone();

        // (kuna `msvcfpconst`) MSVC spells each floating-point literal in the name
        // of the COMDAT that holds it, and COMDAT folding leaves that symbol
        // undefined in every object but one. Decode the name back into the datum:
        // the undefined half gets bytes at its synthetic extern slot (which has no
        // backing at all otherwise), and both halves get a foldable range.
        let fpconst = crate::loader::kuna_msvcfpconst::plan(file, &layout);
        for w in &fpconst.warnings {
            eprintln!("[kuna msvcfpconst] {filename}: {w}");
        }

        // One bounded report per loader construction. Analysis-side consumers
        // may construct another layout for address rebasing, but never print it.
        if emit_diagnostics {
            for line in layout.diagnostics.report_lines() {
                eprintln!("[kuna ET_REL loader] {filename}: {line}");
            }
        }

        let mut segments: Vec<Segment> =
            layout.segments.into_iter().map(|(vma, data)| Segment { vma, data }).collect();
        segments.extend(fpconst.writes.into_iter().map(|(vma, data)| Segment { vma, data }));
        segments.sort_by_key(|s| s.vma);

        let sections: Vec<SectionInfo> = layout
            .sections
            .into_iter()
            .map(|(vma, size, flags)| SectionInfo { vma, size, flags })
            .collect();

        // Defined functions (rebased) + extern call targets, demangled + deduped
        // by address — the same `seen`/`demangle_funcsym_name` discipline the
        // linked path's `.symtab` loop uses.
        let namechars = symbolnamechars_mode();
        let mut funcsyms: Vec<FuncSym> = Vec::new();
        let mut seen: HashSet<u64> = HashSet::new();
        for (addr, name) in layout.funcsyms {
            if addr == 0 {
                continue;
            }
            let name = crate::loader::elf_plt::strip_version(&name);
            if name.is_empty() {
                continue;
            }
            let name = demangle_funcsym_name(name, namechars);
            if seen.insert(addr) {
                funcsyms.push(FuncSym { addr, name });
            }
        }

        // (kuna §2.2) Same default-model fallback id as the linked path: the
        // arch/endian stem with the model dropped to the per-arch default.
        let fallback_archtype = fallback_language_id(file, &archtype);

        Ok(ObjectLoadImage {
            filename: filename.to_string(),
            archtype,
            fallback_archtype,
            segments,
            sections,
            // No program headers on a relocatable object: the section table is
            // the only mapping story it has, and it always has one.
            segment_info: Vec::new(),
            funcsyms,
            reloc_sections,
            reloc_symbols,
            // A relocatable object's symbol addresses are section-relative and are
            // rebased by [`reloc_object::layout_relocatable`], which resolves the
            // function half only (`RelocLayout::funcsyms`).  Data-symbol naming is
            // therefore linked-image-only; an `ET_REL` load keeps today's behavior.
            datasyms: Vec::new(),
            const_ranges: Vec::new(),
            // (kuna) The foldable-range exception list. `dynrelocs` itself is
            // linked-image only (a relocatable object's relocations are already
            // applied by the layout pass); on this path the entries come from
            // `msvcfpconst`, whose ranges qualify for exactly the same reason —
            // a datum that is known by construction, not by policy.
            dynreloc_const: fpconst.const_ranges,
            spaceid: None,
            buffer: RefCell::new(vec![0u8; BUFSIZE]),
            bufoffset: RefCell::new(!0u64),
            cursymbol: RefCell::new(0),
            cursection: RefCell::new(0),
        })
    }

    /// Attach the image to a particular space (C++
    /// `LoadImageBfd::attachToSpace`).
    pub fn attach_to_space(&mut self, id: Rc<AddrSpace>) {
        self.spaceid = Some(id);
    }

    /// The resolved SLEIGH language id (also returned by [`LoadImage::get_arch_type`]).
    pub fn arch_id(&self) -> &[u8] {
        &self.archtype
    }

    /// The per-arch default-model fallback language id (design §2.2), if it
    /// differs from [`Self::arch_id`]. The engine tries this when the primary id
    /// — which may carry a format-specific compiler model (e.g. PE `:windows`) —
    /// does not resolve in the vendored SLEIGH DB.
    pub fn fallback_arch_id(&self) -> Option<&[u8]> {
        self.fallback_archtype.as_deref()
    }

    /// The mapped sections as `(vma, size, section_flags)` triples (the snapshot
    /// the `get_readonly`/`getNextSection` info walks ride). Exposed so a
    /// cross-crate gate can assert the per-format `section_bits` produced the
    /// right exec (`CODE`) / `READONLY` / `NOLOAD` bits on a real PE/Mach-O/ELF
    /// without re-parsing the object. The flags are
    /// `kuna_sleigh::loadimage::section_flags::*`.
    pub fn section_snapshot(&self) -> Vec<(u64, u64, u32)> {
        self.sections.iter().map(|s| (s.vma, s.size, s.flags)).collect()
    }

    /// (kuna) The `[start, stop]` (inclusive) ranges whose contents fold to a
    /// constant with the program-wide `option readonly` still off — the
    /// `PT_GNU_RELRO`-frozen dynamic-relocation slots (`dynrelocs`) on a linked
    /// image, the MSVC `__real@` FP-constant COMDATs (`msvcfpconst`) on a
    /// relocatable object.
    ///
    /// `get_readonly` already reports them, which paints `Varnode::readonly` — but
    /// the constant FOLD of a read-only global is gated globally by `option
    /// readonly` (`Architecture::readonlypropagate`, default off, and turning it on
    /// would fold every `.rodata` read in the program). These ranges are the
    /// narrow exception: values the LINKER computed, in memory the loader
    /// `mprotect`s read-only before `main`. The engine carries them on
    /// `Architecture::dynreloc_const` and `ActionVarnodeProps` folds a read-only
    /// varnode inside one even when global propagation is off.
    pub fn dynreloc_const_ranges(&self) -> &[(u64, u64)] {
        &self.dynreloc_const
    }

    /// The loader's function symbols as `(load_vma, name)` pairs — the SAME list
    /// `getNextSymbol` yields, but already **rebased** to the load VMA (for an
    /// `ET_REL` `.o` the raw `object` symbol address is section-relative; the
    /// loader's layout pass rebases each `SHF_ALLOC` section above
    /// [`RELOC_BASE`](crate::loader::reloc_object::RELOC_BASE) and rebases the
    /// symbols with it). Exposed for the FID generator
    /// ([`crate::fid::build`]), which needs the rebased `(addr, name)` to seed
    /// the Listing and label the hashed functions — the raw `object::File`
    /// addresses would point into the unrebased `.o` and miss every instruction.
    /// Names are lossy-UTF-8 decoded (the marshal convention stores them as bytes).
    pub fn func_symbols(&self) -> Vec<(u64, String)> {
        self.funcsyms
            .iter()
            .map(|s| (s.addr, String::from_utf8_lossy(&s.name).into_owned()))
            .collect()
    }

    /// The original section coordinates retained for a relocatable object.
    /// Empty for a linked image.
    pub fn reloc_sections(&self) -> &[crate::loader::reloc_object::RelocSectionInfo] {
        &self.reloc_sections
    }

    /// Function-symbol provenance retained for a relocatable object. Defined
    /// records carry section coordinates; undefined records carry their
    /// synthetic extern VMA and no object location.
    pub fn reloc_symbols(&self) -> &[crate::loader::reloc_object::RelocSymbolInfo] {
        &self.reloc_symbols
    }

    /// The loader's **data** symbols as `(vma, name, size)` triples — the
    /// `STT_OBJECT` half of the same `.symtab`/`.dynsym` walks [`Self::func_symbols`]
    /// reads, filtered to defined, named, non-zero-size entries (see [`DataSym`]).
    ///
    /// The engine installs each as a named, `undefined<size>`-typed global so a
    /// memory access of the object renders the symbol name (`optind`) rather than
    /// `dat_<addr>`, matching IDA Pro and Ghidra. Empty for a relocatable object.
    pub fn data_symbols(&self) -> Vec<(u64, String, u64)> {
        self.datasyms
            .iter()
            .map(|s| (s.addr, String::from_utf8_lossy(&s.name).into_owned(), s.size))
            .collect()
    }

    /// Find the segment containing `offset`, or the closest segment above it
    /// (C++ `LoadImageBfd::findSection`, `loadimage_bfd.cc:99`).  Returns the
    /// index into [`Self::segments`] and the segment size, or `None` for "no
    /// segment at or above `offset`" (the C++ null `champ`).
    fn find_section(&self, offset: u64) -> Option<(usize, u64)> {
        // First pass: the segment that actually contains `offset`.
        for (i, s) in self.segments.iter().enumerate() {
            let start = s.vma;
            let secsize = s.data.len() as u64; // cast: segment byte count
            let stop = start.wadd(secsize);
            // C++ uses raw `<`/`>=`; a
            // wrapped stop (segment at the top of the space) cannot occur for a
            // real ELF, matching the BFD assumption.
            if offset >= start && offset < stop {
                return Some((i, secsize));
            }
        }
        // Second pass: the closest segment strictly above `offset` (segments are
        // vma-sorted, so the first such is the closest — the C++ `champ` scan).
        for (i, s) in self.segments.iter().enumerate() {
            if s.vma > offset {
                return Some((i, s.data.len() as u64));
            }
        }
        None
    }

    /// Copy `len` bytes out of segment `idx` starting at file-relative
    /// `seg_off` into `dst` (the C++ `bfd_get_section_contents`).  A read past
    /// the segment's file data zero-fills the remainder (a `.bss`-style RAM tail
    /// whose bytes BFD would report as zero).
    fn copy_segment(&self, idx: usize, seg_off: u64, dst: &mut [u8]) {
        let data = &self.segments[idx].data;
        for (i, b) in dst.iter_mut().enumerate() {
            let pos = seg_off.wadd(i as u64); // cast: small loop index
            *b = data.get(pos as usize).copied().unwrap_or(0); // cast: pos within data here
        }
    }
}

impl LoadImage for ObjectLoadImage {
    fn get_file_name(&self) -> &str {
        &self.filename
    }

    fn load_fill(&mut self, ptr: &mut [u8], addr: &Address) -> KunaResult<()> {
        // cast: the C++ `int4 size` parameter (slice length; see trait docs).
        let size: i32 = ptr.len() as i32;
        let space = addr
            .get_space()
            .expect("ObjectLoadImage::loadFill: address with null space (C++ UB)");
        match &self.spaceid {
            Some(sp) if Rc::ptr_eq(sp, space) => {}
            _ => {
                return Err(KunaError::data_unavail(format!(
                    "Trying to get loadimage bytes from space: {}",
                    space.get_name()
                )));
            }
        }

        let curaddr0: u64 = addr.get_offset();
        let mut bufoffset = self.bufoffset.borrow_mut();
        let mut buffer = self.buffer.borrow_mut();

        // The C++ comparison is exact uintb arithmetic (BUFSIZE is 512, so the
        // `+ size` cannot wrap for any real request).
        if curaddr0 >= *bufoffset
            && curaddr0.wadd(size as u64) < (*bufoffset).wadd(BUFSIZE as u64)
        {
            let start = (curaddr0 - *bufoffset) as usize; // cast: in-buffer offset
            ptr.copy_from_slice(&buffer[start..start + ptr.len()]);
            return Ok(());
        }

        // Load the buffer with bytes from the new address.
        *bufoffset = curaddr0;
        let mut offset: usize = 0;
        let mut cursize: i32 = BUFSIZE as i32; // read an entire buffer
        let mut curaddr = curaddr0;

        while cursize > 0 {
            let found = self.find_section(curaddr);
            let Some((idx, secsize)) = found else {
                if offset == 0 {
                    break; // Initial address not mapped
                }
                // Fill the rest with zero.
                for b in &mut buffer[offset..offset + cursize as usize] {
                    *b = 0;
                }
                ptr.copy_from_slice(&buffer[..ptr.len()]);
                return Ok(());
            };
            let seg_vma = self.segments[idx].vma;
            let readsize: u64;
            if seg_vma > curaddr {
                // No section matches at curaddr.
                if offset == 0 {
                    break; // Initial address not mapped
                }
                let mut rs = seg_vma - curaddr;
                if rs > cursize as u64 {
                    rs = cursize as u64;
                }
                // Zeroes to the next section.
                for b in &mut buffer[offset..offset + rs as usize] {
                    *b = 0;
                }
                readsize = rs;
            } else {
                let mut rs = cursize as u64;
                if curaddr.wadd(rs) > seg_vma.wadd(secsize) {
                    rs = seg_vma.wadd(secsize).wsub(curaddr);
                }
                let seg_off = curaddr - seg_vma; // file-relative read offset
                self.copy_segment(idx, seg_off, &mut buffer[offset..offset + rs as usize]);
                readsize = rs;
            }
            offset += readsize as usize; // cast: readsize <= BUFSIZE here
            cursize -= readsize as i32; // cast: readsize <= cursize (an int4) here
            curaddr = curaddr.wadd(readsize);
        }
        if cursize > 0 {
            // (offset==0 break path) Unable to load N bytes at <addr>.
            //
            // (kuna) Restore the "nothing buffered" sentinel first.  `bufoffset`
            // was claimed at the top of the fill, before any byte was read, so
            // leaving it set on the failure path makes the fast path above hand
            // out the 512-byte window starting at an address that is NOT MAPPED —
            // stale bytes, reported as a successful read, for every request
            // within `BUFSIZE` of the one that just failed.  Upstream leaves it
            // claimed (loadimage_bfd.cc throws with `bufoffset` already
            // assigned), which is invisible there only because nothing reads
            // again nearby; kuna's per-entry `entry_bytes_mapped` probe walks a
            // relocatable object's extern slots in address order and so hits
            // exactly that window — the first extern reported unmapped and the
            // next forty "mapped", out of one poisoned buffer.
            *bufoffset = !0u64;
            let mut errmsg =
                format!("Unable to load {} bytes at {}", cursize, addr.get_shortcut());
            addr.print_raw(&mut errmsg)?;
            return Err(KunaError::data_unavail(errmsg));
        }
        // Copy the requested bytes out.
        ptr.copy_from_slice(&buffer[..ptr.len()]);
        let _ = size; // size mirrors ptr.len(); kept for the C++ correspondence
        Ok(())
    }

    fn open_symbols(&self) {
        *self.cursymbol.borrow_mut() = 0;
    }

    fn get_next_symbol(&self, record: &mut LoadImageFunc) -> bool {
        let mut cur = self.cursymbol.borrow_mut();
        if *cur >= self.funcsyms.len() {
            return false;
        }
        let sym = &self.funcsyms[*cur];
        *cur += 1;
        record.name = sym.name.clone();
        let space = self
            .spaceid
            .as_ref()
            .expect("ObjectLoadImage::getNextSymbol before attachToSpace (C++ null space)");
        record.address = Address::new(Rc::clone(space), sym.addr);
        true
    }

    fn open_section_info(&self) {
        *self.cursection.borrow_mut() = 0;
    }

    fn get_next_section(&self, record: &mut LoadImageSection) -> bool {
        let mut cur = self.cursection.borrow_mut();
        if *cur >= self.sections.len() {
            return false;
        }
        let sec = &self.sections[*cur];
        let space = self
            .spaceid
            .as_ref()
            .expect("ObjectLoadImage::getNextSection before attachToSpace (C++ null space)");
        record.address = Address::new(Rc::clone(space), sec.vma);
        record.size = sec.size;
        record.flags = sec.flags;
        *cur += 1;
        // C++ returns whether *another* section follows.
        *cur < self.sections.len()
    }

    fn get_segments(&self) -> Vec<(u64, u64, u32)> {
        self.segment_info.iter().map(|s| (s.vma, s.size, s.flags)).collect()
    }

    fn get_readonly(&self, list: &mut RangeList) {
        // List all ranges that are read only (C++ `LoadImageBfd::getReadonly`).
        let Some(space) = self.spaceid.as_ref() else {
            return;
        };
        // (kuna) The MIPS GOT external slots, marked constant so the engine folds
        // the `lw $t9, off($gp)` indirect-call load to the stub address (the analog
        // of Ghidra's `setConstant` on the GOT pointer entries).  Empty off-MIPS.
        for &(start, stop) in &self.const_ranges {
            list.insert_range(Rc::clone(space), start, stop);
        }
        // (kuna) The constant-by-construction ranges: the dynamic-relocation slots
        // `PT_GNU_RELRO` freezes (read-only in the run-time image, so their
        // relocated contents are trustworthy) and the MSVC `__real@` FP-constant
        // COMDATs (whose value is spelled in the symbol name).
        for &(start, stop) in &self.dynreloc_const {
            list.insert_range(Rc::clone(space), start, stop);
        }
        for sec in &self.sections {
            if sec.flags & section_flags::READONLY != 0 {
                if sec.size == 0 {
                    continue;
                }
                let start = sec.vma;
                let stop = start.wadd(sec.size).wsub(1);
                list.insert_range(Rc::clone(space), start, stop);
            }
        }
    }

    fn get_arch_type(&self) -> Vec<u8> {
        self.archtype.clone()
    }

    fn adjust_vma(&mut self, adjust: i64) {
        // C++ `LoadImageBfd::adjustVma` (`loadimage_bfd.cc:67`): scale the shift by wordsize.
        let spaceid = self
            .spaceid
            .as_ref()
            .expect("ObjectLoadImage::adjustVma before attachToSpace (C++ null space deref)");
        let badjust = AddrSpace::address_to_byte(adjust as u64, spaceid.get_word_size());
        for s in &mut self.segments {
            s.vma = s.vma.wadd(badjust);
        }
        for s in &mut self.sections {
            s.vma = s.vma.wadd(badjust);
        }
        for s in &mut self.segment_info {
            s.vma = s.vma.wadd(badjust);
        }
        for s in &mut self.funcsyms {
            s.addr = s.addr.wadd(badjust);
        }
        for r in &mut self.const_ranges {
            r.0 = r.0.wadd(badjust);
            r.1 = r.1.wadd(badjust);
        }
        for r in &mut self.dynreloc_const {
            r.0 = r.0.wadd(badjust);
            r.1 = r.1.wadd(badjust);
        }
        // A shifted segment set may no longer be vma-sorted only if `badjust`
        // wraps a subset past the top of the space; for a real ELF every vma
        // shifts by the same amount so the order is preserved (matching BFD,
        // which never re-sorts).
    }
}

/// Resolve the SLEIGH **language id** for an object (the `getArchType` payload).
/// This is the kuna substitution for the Ghidra Java-side BFD-name -> language
/// map: it reads the machine + endianness + class directly and returns the id
/// `SleighArchitecture::resolveArchitecture` consumes (e.g.
/// `x86:LE:64:default:gcc`).
///
/// The arch -> language-stem match is format-independent (`object` collapses
/// `e_machine`/`IMAGE_FILE_MACHINE_*`/Mach-O `cputype` into one
/// [`Architecture`]).  The **only** per-format variation is the compiler-model
/// field, which comes from [`ObjectFormat::compiler_model`]: for ELF this is the
/// same `gcc`/`default` token the function baked in before, so every produced id
/// string is **byte-identical to today** — this is a structural change with no
/// output change.  An arch with no `compiler_model` opinion falls back to the
/// per-arch default (`gcc`/`default`) the id strings already used.
///
/// PARTIAL: covers the common machines kuna ships a `.sla` for.  An unmapped
/// machine is a `LowlevelError` naming it (the caller falls back to an explicit
/// `--target` language id).
fn language_id_for(
    file: &object::File,
    fmt: &dyn crate::loader::format::ObjectFormat,
    bytes: &[u8],
    filename: &str,
) -> KunaResult<Vec<u8>> {
    let little = file.is_little_endian();
    let endian = if little { "LE" } else { "BE" };
    let arch = file.architecture();
    // (PR-8 §3.7) Pointer-auth arm64e spec selection, GATED + opt-in: an arm64e
    // Mach-O (`cpusubtype` CPU_SUBTYPE_ARM64E) selects the Apple-Silicon SLEIGH
    // spec (`AARCH64:LE:64:AppleSilicon`) instead of the generic v8A. This is the
    // ONLY thing arm64e changes (import naming / symbols are unaffected). Off by
    // default (the `macho-arm64e` gate); when on and the binary is an arm64e
    // Mach-O, the AppleSilicon id wins over the composed `v8A` id below.
    if let Some(apple_id) = crate::loader::format::macho::apple_silicon_id(fmt, arch, bytes) {
        return Ok(apple_id.into_bytes());
    }
    // The format's default-ABI compiler model for this arch.  For ELF this is
    // `gcc` (x86/RISCV) / `default` (everything else), reproducing the old
    // hard-coded id strings exactly.
    let model = fmt.compiler_model(arch);
    let id: String = match compose_language_id(arch, endian, model) {
        Some(id) => id,
        None => {
            return Err(KunaError::lowlevel(format!(
                "File: {filename} : unsupported machine {arch:?} \
                 (no kuna SLEIGH language; pass an explicit --target language id)"
            )));
        }
    };
    Ok(id.into_bytes())
}

/// The per-arch *default-model* fallback language id (design §2.2): the same
/// arch/endian stem as `primary`, but with the compiler model dropped to the
/// per-arch default (`compose_language_id(arch, endian, None)` — `gcc` for
/// x86/RISCV, `default` elsewhere). Returns `Some` only when it differs from
/// `primary` (so an ELF, whose primary already uses the default model, gets
/// `None` — no behavior change on the established path).
///
/// The engine uses this as a one-step retry: if a format's chosen model (e.g.
/// PE's `:windows`) is not vendored for this arch, falling back to the arch
/// default beats erroring out — wrong calling-convention details still yield a
/// decompile.
fn fallback_language_id(file: &object::File, primary: &[u8]) -> Option<Vec<u8>> {
    let endian = if file.is_little_endian() { "LE" } else { "BE" };
    let fallback = compose_language_id(file.architecture(), endian, None)?;
    if fallback.as_bytes() == primary {
        None
    } else {
        Some(fallback.into_bytes())
    }
}

/// Compose the SLEIGH language id from the format-neutral `arch` + `endian`
/// stem and the per-format compiler `model`. `None` for an arch kuna has no
/// `.sla` stem for. The single source of truth for the id string shape, shared
/// by [`language_id_for`] and [`elf_language_ids`] so the resolves-in-the-DB
/// test cannot drift from the producer.
fn compose_language_id(arch: Architecture, endian: &str, model: Option<&str>) -> Option<String> {
    Some(match arch {
        Architecture::X86_64 => format!("x86:LE:64:default:{}", model.unwrap_or("gcc")),
        Architecture::I386 => format!("x86:LE:32:default:{}", model.unwrap_or("gcc")),
        Architecture::Aarch64 => format!("AARCH64:{endian}:64:v8A:{}", model.unwrap_or("default")),
        Architecture::Arm => format!("ARM:{endian}:32:v8:{}", model.unwrap_or("default")),
        Architecture::Mips => format!("MIPS:{endian}:32:default:{}", model.unwrap_or("default")),
        Architecture::PowerPc => {
            format!("PowerPC:{endian}:32:default:{}", model.unwrap_or("default"))
        }
        Architecture::PowerPc64 => {
            format!("PowerPC:{endian}:64:default:{}", model.unwrap_or("default"))
        }
        Architecture::Riscv64 => format!("RISCV:{endian}:64:RV64GC:{}", model.unwrap_or("gcc")),
        Architecture::Riscv32 => format!("RISCV:{endian}:32:RV32GC:{}", model.unwrap_or("gcc")),
        Architecture::Sparc | Architecture::Sparc32Plus => {
            format!("sparc:{endian}:32:default:{}", model.unwrap_or("default"))
        }
        Architecture::Sparc64 => format!("sparc:{endian}:64:default:{}", model.unwrap_or("default")),
        _ => return None,
    })
}

/// Every SLEIGH language id the **ELF** loader (`ElfFormat` + [`language_id_for`])
/// can produce, for both endiannesses, over the supported ELF machines. This is
/// the exact set a real ELF resolves to today — exposed so the cross-crate
/// console gate (`verify_elf_language_ids`) can assert every one of them resolves
/// in the SLEIGH language database (`scan_language_database`), proving the §2.2
/// `compiler_model` refactor still yields only valid, vendored ids. Derived from
/// the same [`compose_language_id`] + [`crate::loader::format::elf::ElfFormat`]
/// the loader uses, so it cannot drift from the producer.
pub fn elf_language_ids() -> Vec<String> {
    use crate::loader::format::elf::ElfFormat;
    use crate::loader::format::ObjectFormat;
    let fmt = ElfFormat;
    // Per arch, the endiannesses a real ELF actually carries *and* the vendored
    // `.ldefs` declare a stem for (so each enumerated id is one a real binary
    // resolves to): x86 is LE-only; RISC-V is LE-only; SPARC is BE-only; the
    // bi-endian arches (ARM/AArch64/MIPS/PowerPC) ship both. `language_id_for`
    // composes the id from the binary's real endianness, so this is exactly the
    // reachable id set, not a cartesian over-generation.
    let arches: &[(Architecture, &[&str])] = &[
        (Architecture::X86_64, &["LE"]),
        (Architecture::I386, &["LE"]),
        (Architecture::Aarch64, &["LE", "BE"]),
        (Architecture::Arm, &["LE", "BE"]),
        (Architecture::Mips, &["LE", "BE"]),
        (Architecture::PowerPc, &["LE", "BE"]),
        (Architecture::PowerPc64, &["LE", "BE"]),
        (Architecture::Riscv64, &["LE"]),
        (Architecture::Riscv32, &["LE"]),
        (Architecture::Sparc, &["BE"]),
        (Architecture::Sparc64, &["BE"]),
    ];
    let mut out = Vec::new();
    for &(arch, endians) in arches {
        for &endian in endians {
            if let Some(id) = compose_language_id(arch, endian, fmt.compiler_model(arch)) {
                out.push(id);
            }
        }
    }
    out.sort();
    out.dedup();
    out
}

/// Every SLEIGH language id a given [`crate::loader::format::ObjectFormat`]
/// produces over `arches`, paired with the *fallback* id [`fallback_language_id`]
/// would compute for the same arch/endian (design §2.2). The console gate
/// (`verify_object_language_ids`) asserts that for every entry **either the
/// primary or the fallback resolves** in the SLEIGH DB — proving the per-format
/// `compiler_model` tokens are real, vendored ids (or that the fallback rule
/// saves a non-vendored model from erroring). Derived from the same
/// [`compose_language_id`] the loader uses, so it cannot drift from the producer.
///
/// `arches` is `(arch, endians, machine-is-LE/BE-on-disk)`; the third flag is
/// what `is_little_endian()` would report (it drives the fallback's endian), so
/// the enumerated pair is exactly what a real binary of that arch produces.
pub fn format_language_ids(
    fmt: &dyn crate::loader::format::ObjectFormat,
    arches: &[(Architecture, &[&str])],
) -> Vec<(String, Option<String>)> {
    let mut out = Vec::new();
    for &(arch, endians) in arches {
        for &endian in endians {
            if let Some(primary) = compose_language_id(arch, endian, fmt.compiler_model(arch)) {
                // The §2.2 fallback for the same stem: the default-model id, if it
                // differs from the primary.
                let fallback = compose_language_id(arch, endian, None)
                    .filter(|fb| *fb != primary);
                out.push((primary, fallback));
            }
        }
    }
    out.sort();
    out.dedup();
    out
}

/// The arch/endian set a **PE** image realistically carries (the Windows arches
/// kuna ships a `.sla` for): x86 (LE only), ARM/AArch64 (LE only on Windows).
pub fn pe_language_ids() -> Vec<(String, Option<String>)> {
    use crate::loader::format::pe::PeFormat;
    format_language_ids(
        &PeFormat,
        &[
            (Architecture::X86_64, &["LE"]),
            (Architecture::I386, &["LE"]),
            (Architecture::Aarch64, &["LE"]),
            (Architecture::Arm, &["LE"]),
        ],
    )
}

/// The arch/endian set a **Mach-O** image realistically carries: x86-64 (LE) and
/// arm64 (LE) — the two macOS arches.
pub fn macho_language_ids() -> Vec<(String, Option<String>)> {
    use crate::loader::format::macho::MachOFormat;
    format_language_ids(
        &MachOFormat,
        &[
            (Architecture::X86_64, &["LE"]),
            (Architecture::I386, &["LE"]),
            (Architecture::Aarch64, &["LE"]),
        ],
    )
}

/// The arch/endian set a **COFF object** realistically carries (the MSVC/clang
/// Windows-object arches): x86 (LE), ARM/AArch64 (LE).
pub fn coff_language_ids() -> Vec<(String, Option<String>)> {
    use crate::loader::format::coff::CoffFormat;
    format_language_ids(
        &CoffFormat,
        &[
            (Architecture::X86_64, &["LE"]),
            (Architecture::I386, &["LE"]),
            (Architecture::Aarch64, &["LE"]),
            (Architecture::Arm, &["LE"]),
        ],
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use kuna_base::space::{addrspace_flags, spacetype, AddrSpaceManager, ConstantSpace};

    /// const(0) + ram(1) processor space (little endian, 8-byte addresses).
    fn manager() -> AddrSpaceManager {
        let mut m = AddrSpaceManager::new();
        m.insert_space(Rc::new(ConstantSpace::new())).unwrap();
        m.insert_space(Rc::new(AddrSpace::new(
            spacetype::IPTR_PROCESSOR,
            "ram",
            false,
            8,
            1,
            1,
            addrspace_flags::hasphysical,
            1,
            1,
        )))
        .unwrap();
        m.set_default_code_space(1).unwrap();
        m
    }

    /// Build a minimal little-endian ELF64 x86-64 image with one PT_LOAD
    /// segment of `seg_bytes` at vma `seg_vma`, and an optional FUNC symbol.
    /// Hand-assembled so the test needs no external toolchain.
    fn build_elf64(seg_vma: u64, seg_bytes: &[u8], func: Option<(&str, u64)>) -> Vec<u8> {
        // Layout: [Ehdr 64][Phdr 56][segment data][shstrtab][.symtab][.strtab]
        //         [Shdr * n].  Kept deliberately small and explicit.
        let mut buf: Vec<u8> = Vec::new();
        let ehdr_size = 64usize;
        let phdr_size = 56usize;
        let seg_off = (ehdr_size + phdr_size) as u64;

        // --- string tables -------------------------------------------------
        // shstrtab: "\0.shstrtab\0.symtab\0.strtab\0.text\0"
        let mut shstr = vec![0u8];
        let name_off = |s: &mut Vec<u8>, n: &str| {
            let off = s.len() as u32;
            s.extend_from_slice(n.as_bytes());
            s.push(0);
            off
        };
        let off_shstrtab = name_off(&mut shstr, ".shstrtab");
        let off_symtab = name_off(&mut shstr, ".symtab");
        let off_strtab = name_off(&mut shstr, ".strtab");
        let off_text = name_off(&mut shstr, ".text");

        // strtab (symbol names): "\0<func>\0"
        let mut strtab = vec![0u8];
        let func_name_off = match func {
            Some((nm, _)) => {
                let off = strtab.len() as u32;
                strtab.extend_from_slice(nm.as_bytes());
                strtab.push(0);
                off
            }
            None => 0,
        };

        // symtab: [null sym][func sym?]  (Elf64_Sym = 24 bytes)
        let mut symtab: Vec<u8> = vec![0u8; 24];
        if let Some((_, addr)) = func {
            let mut sym = Vec::new();
            sym.extend_from_slice(&func_name_off.to_le_bytes()); // st_name u32
            sym.push(0x02); // st_info: STB_LOCAL<<4 | STT_FUNC(2)
            sym.push(0); // st_other
            sym.extend_from_slice(&1u16.to_le_bytes()); // st_shndx (.text idx=1)
            sym.extend_from_slice(&addr.to_le_bytes()); // st_value u64
            sym.extend_from_slice(&0u64.to_le_bytes()); // st_size u64
            symtab.extend_from_slice(&sym);
        }

        // --- file body offsets --------------------------------------------
        let shstr_off = seg_off + seg_bytes.len() as u64;
        let symtab_off = shstr_off + shstr.len() as u64;
        let strtab_off = symtab_off + symtab.len() as u64;
        let sh_off = strtab_off + strtab.len() as u64;
        // 5 section headers: null, .text, .shstrtab, .symtab, .strtab
        let shnum = 5u16;
        let shstrndx = 2u16;

        // --- Ehdr (Elf64) --------------------------------------------------
        buf.extend_from_slice(&[0x7f, b'E', b'L', b'F']);
        buf.push(2); // EI_CLASS = ELFCLASS64
        buf.push(1); // EI_DATA = ELFDATA2LSB
        buf.push(1); // EI_VERSION
        buf.push(0); // EI_OSABI
        buf.extend_from_slice(&[0u8; 8]); // EI_PAD
        buf.extend_from_slice(&2u16.to_le_bytes()); // e_type = ET_EXEC
        buf.extend_from_slice(&62u16.to_le_bytes()); // e_machine = EM_X86_64
        buf.extend_from_slice(&1u32.to_le_bytes()); // e_version
        buf.extend_from_slice(&seg_vma.to_le_bytes()); // e_entry
        buf.extend_from_slice(&(ehdr_size as u64).to_le_bytes()); // e_phoff
        buf.extend_from_slice(&sh_off.to_le_bytes()); // e_shoff
        buf.extend_from_slice(&0u32.to_le_bytes()); // e_flags
        buf.extend_from_slice(&(ehdr_size as u16).to_le_bytes()); // e_ehsize
        buf.extend_from_slice(&(phdr_size as u16).to_le_bytes()); // e_phentsize
        buf.extend_from_slice(&1u16.to_le_bytes()); // e_phnum
        buf.extend_from_slice(&64u16.to_le_bytes()); // e_shentsize
        buf.extend_from_slice(&shnum.to_le_bytes()); // e_shnum
        buf.extend_from_slice(&shstrndx.to_le_bytes()); // e_shstrndx

        // --- Phdr (one PT_LOAD, R+X) --------------------------------------
        buf.extend_from_slice(&1u32.to_le_bytes()); // p_type = PT_LOAD
        buf.extend_from_slice(&5u32.to_le_bytes()); // p_flags = PF_R|PF_X
        buf.extend_from_slice(&seg_off.to_le_bytes()); // p_offset
        buf.extend_from_slice(&seg_vma.to_le_bytes()); // p_vaddr
        buf.extend_from_slice(&seg_vma.to_le_bytes()); // p_paddr
        buf.extend_from_slice(&(seg_bytes.len() as u64).to_le_bytes()); // p_filesz
        buf.extend_from_slice(&(seg_bytes.len() as u64).to_le_bytes()); // p_memsz
        buf.extend_from_slice(&0x1000u64.to_le_bytes()); // p_align

        // --- bodies --------------------------------------------------------
        debug_assert_eq!(buf.len() as u64, seg_off);
        buf.extend_from_slice(seg_bytes);
        buf.extend_from_slice(&shstr);
        buf.extend_from_slice(&symtab);
        buf.extend_from_slice(&strtab);

        // --- section headers (Elf64_Shdr = 64 bytes each) -----------------
        let push_shdr = |b: &mut Vec<u8>,
                         name: u32,
                         sh_type: u32,
                         sh_flags: u64,
                         addr: u64,
                         offset: u64,
                         size: u64,
                         link: u32,
                         info: u32,
                         entsize: u64| {
            b.extend_from_slice(&name.to_le_bytes());
            b.extend_from_slice(&sh_type.to_le_bytes());
            b.extend_from_slice(&sh_flags.to_le_bytes());
            b.extend_from_slice(&addr.to_le_bytes());
            b.extend_from_slice(&offset.to_le_bytes());
            b.extend_from_slice(&size.to_le_bytes());
            b.extend_from_slice(&link.to_le_bytes());
            b.extend_from_slice(&info.to_le_bytes());
            b.extend_from_slice(&0u64.to_le_bytes()); // sh_addralign
            b.extend_from_slice(&entsize.to_le_bytes());
        };
        debug_assert_eq!(buf.len() as u64, sh_off);
        // 0: null
        push_shdr(&mut buf, 0, 0, 0, 0, 0, 0, 0, 0, 0);
        // 1: .text  (SHT_PROGBITS, ALLOC|EXECINSTR)
        push_shdr(
            &mut buf,
            off_text,
            1,
            0x2 | 0x4,
            seg_vma,
            seg_off,
            seg_bytes.len() as u64,
            0,
            0,
            0,
        );
        // 2: .shstrtab (SHT_STRTAB)
        push_shdr(&mut buf, off_shstrtab, 3, 0, 0, shstr_off, shstr.len() as u64, 0, 0, 0);
        // 3: .symtab (SHT_SYMTAB, link=.strtab(4), entsize=24)
        push_shdr(&mut buf, off_symtab, 2, 0, 0, symtab_off, symtab.len() as u64, 4, 1, 24);
        // 4: .strtab (SHT_STRTAB)
        push_shdr(&mut buf, off_strtab, 3, 0, 0, strtab_off, strtab.len() as u64, 0, 0, 0);

        buf
    }

    #[test]
    fn elf_arch_type_is_x86_64_language_id() {
        let elf = build_elf64(0x401000, &[0x90, 0xc3], None);
        let img = ObjectLoadImage::from_bytes("t.elf", &elf).unwrap();
        assert_eq!(img.get_arch_type(), b"x86:LE:64:default:gcc".to_vec());
        assert_eq!(img.get_file_name(), "t.elf");
    }

    #[test]
    fn elf_load_fill_exact_and_gap_fill() {
        let m = manager();
        let ram = Rc::clone(m.get_space_by_name("ram").unwrap());
        // A 4-byte code segment at 0x401000.
        let elf = build_elf64(0x401000, &[0x55, 0x48, 0x89, 0xe5], None);
        let mut img = ObjectLoadImage::from_bytes("t.elf", &elf).unwrap();
        img.attach_to_space(Rc::clone(&ram));

        // Exact read of the mapped bytes.
        let got = img.load(4, &Address::new(Rc::clone(&ram), 0x401000)).unwrap();
        assert_eq!(got, vec![0x55, 0x48, 0x89, 0xe5]);

        // Read straddling the segment tail: mapped bytes, then zero fill.
        let got = img.load(8, &Address::new(Rc::clone(&ram), 0x401000)).unwrap();
        assert_eq!(got, vec![0x55, 0x48, 0x89, 0xe5, 0, 0, 0, 0]);

        // Initial address entirely unmapped -> DataUnavailError (BFD contract).
        let err = img.load(4, &Address::new(Rc::clone(&ram), 0x1000)).unwrap_err();
        match &err {
            KunaError::DataUnavail { explain } => {
                assert!(explain.starts_with("Unable to load"), "got {explain}");
            }
            other => panic!("expected DataUnavail, got {other:?}"),
        }
    }

    #[test]
    fn elf_symbols_iterate() {
        let m = manager();
        let ram = Rc::clone(m.get_space_by_name("ram").unwrap());
        let elf = build_elf64(0x401000, &[0x90, 0xc3], Some(("add", 0x401000)));
        let mut img = ObjectLoadImage::from_bytes("t.elf", &elf).unwrap();
        img.attach_to_space(Rc::clone(&ram));

        img.open_symbols();
        let mut rec = LoadImageFunc::default();
        assert!(img.get_next_symbol(&mut rec));
        assert_eq!(rec.name, b"add".to_vec());
        assert_eq!(rec.address, Address::new(Rc::clone(&ram), 0x401000));
        assert!(!img.get_next_symbol(&mut rec));
    }

    #[test]
    fn elf_wrong_space_is_data_unavail() {
        let m = manager();
        let ram = Rc::clone(m.get_space_by_name("ram").unwrap());
        let other = Rc::clone(m.get_space_by_name("const").unwrap());
        let elf = build_elf64(0x401000, &[0x90, 0xc3], None);
        let mut img = ObjectLoadImage::from_bytes("t.elf", &elf).unwrap();
        img.attach_to_space(Rc::clone(&ram));
        let err = img.load(2, &Address::new(other, 0)).unwrap_err();
        assert!(matches!(err, KunaError::DataUnavail { .. }));
    }

    #[test]
    fn elf_adjust_vma_shifts_segments_and_symbols() {
        let m = manager();
        let ram = Rc::clone(m.get_space_by_name("ram").unwrap());
        let elf = build_elf64(0x401000, &[0x11, 0x22, 0x33, 0x44], Some(("f", 0x401000)));
        let mut img = ObjectLoadImage::from_bytes("t.elf", &elf).unwrap();
        img.attach_to_space(Rc::clone(&ram));
        img.adjust_vma(0x1000);
        // The bytes now live at 0x402000.
        let got = img.load(4, &Address::new(Rc::clone(&ram), 0x402000)).unwrap();
        assert_eq!(got, vec![0x11, 0x22, 0x33, 0x44]);
        // And the symbol moved too.
        img.open_symbols();
        let mut rec = LoadImageFunc::default();
        assert!(img.get_next_symbol(&mut rec));
        assert_eq!(rec.address, Address::new(Rc::clone(&ram), 0x402000));
    }

    #[test]
    fn non_elf_is_rejected() {
        let err = ObjectLoadImage::from_bytes("x", b"not an object file").unwrap_err();
        assert!(matches!(err, KunaError::Lowlevel { .. }));
    }

    // ---- Real-ELF PLT/GOT import-name resolution (elf_plt) -----------------
    //
    // These load vendored fixture binaries (tests/fixtures/) and check the
    // resolved `addr -> name` function-symbol stream — the exact stream the
    // engine feeds the decompiler via `read_loader_symbols_generic`.  The XML
    // datatest corpus cannot exercise this (it embeds bytechunks + explicit
    // <symbol> defs and never constructs an `ObjectLoadImage`), so the gate lives
    // here in the cargo workspace suite.

    /// Load a vendored fixture ELF and collect its resolved `addr -> name` map.
    fn fixture_funcsyms(name: &str) -> std::collections::HashMap<u64, String> {
        let path = format!("{}/tests/fixtures/{}", env!("CARGO_MANIFEST_DIR"), name);
        let bytes = std::fs::read(&path).unwrap_or_else(|e| panic!("read {path}: {e}"));
        let m = manager();
        let ram = Rc::clone(m.get_space_by_name("ram").unwrap());
        let mut img = ObjectLoadImage::from_bytes(&path, &bytes).unwrap();
        img.attach_to_space(Rc::clone(&ram));
        let mut out = std::collections::HashMap::new();
        img.open_symbols();
        loop {
            let mut rec = LoadImageFunc::default();
            if !img.get_next_symbol(&mut rec) {
                break;
            }
            out.insert(rec.address.get_offset(), String::from_utf8_lossy(&rec.name).into_owned());
        }
        out
    }

    /// (kuna, `zero-function-sizes-make`) `getSegments` reports the program
    /// headers with their permissions translated into section flags — the only
    /// mapping story a sectionless image has left.
    #[test]
    fn segments_carry_the_load_permissions_of_a_sectionless_image() {
        use kuna_sleigh::loadimage::LoadImage;
        let path = format!("{}/tests/fixtures/noshdr_x86_64", env!("CARGO_MANIFEST_DIR"));
        let bytes = std::fs::read(&path).expect("fixture");
        let img = ObjectLoadImage::from_bytes(&path, &bytes).expect("load sectionless PIE");
        assert!(img.section_snapshot().is_empty(), "e_shoff is zero — no section table");
        assert_eq!(
            img.get_segments(),
            vec![
                (0x0, 0xe8, section_flags::DATA | section_flags::READONLY),
                (0x100, 0x16, section_flags::CODE | section_flags::READONLY),
                (0x120, 0x10, section_flags::DATA),
            ],
            "PF_X is the CODE bit, PF_W clears READONLY, and the order is ascending vma"
        );
    }

    /// A linked image with a section table still reports both, independently:
    /// the segments are a second, coarser view, not a replacement.
    #[test]
    fn a_sectioned_image_reports_segments_as_well() {
        use kuna_sleigh::loadimage::LoadImage;
        let path = format!("{}/tests/fixtures/fauxware", env!("CARGO_MANIFEST_DIR"));
        let bytes = std::fs::read(&path).expect("fixture");
        let img = ObjectLoadImage::from_bytes(&path, &bytes).expect("load fauxware");
        let segments = img.get_segments();
        assert!(!img.section_snapshot().is_empty(), "fauxware has sections");
        assert!(
            segments.iter().any(|&(_, _, flags)| flags & section_flags::CODE != 0),
            "one PT_LOAD is executable; got {segments:?}"
        );
        assert!(
            segments.iter().all(|&(vma, size, _)| size > 0 && vma > 0),
            "zero-size records are dropped; got {segments:?}"
        );
    }

    /// (kuna, `pe-header-entry-mapped`) The PE header page reaches the map: an
    /// entry declared inside it has readable bytes, and the region is published
    /// as read-only DATA so the executable-region scans stay out of it.
    #[test]
    fn a_pe_maps_its_header_page_read_only() {
        use kuna_sleigh::loadimage::LoadImage;
        let path = format!(
            "{}/tests/fixtures/pe_headerentry_i386.exe",
            env!("CARGO_MANIFEST_DIR")
        );
        let bytes = std::fs::read(&path).expect("fixture");
        let m = manager();
        let ram = Rc::clone(m.get_space_by_name("ram").unwrap());
        let mut img = ObjectLoadImage::from_bytes(&path, &bytes).expect("load the PE");
        img.attach_to_space(Rc::clone(&ram));

        let header = (0x40_0000u64, 0x200u64, section_flags::DATA | section_flags::READONLY);
        assert!(
            img.get_segments().contains(&header),
            "SizeOfHeaders bytes are mapped at ImageBase; got {:?}",
            img.get_segments()
        );
        assert!(
            img.section_snapshot().contains(&header),
            "and are published to the section walk, so the address resolves as mapped"
        );

        // The declared entry sits at RVA 0x154, one byte past the section table.
        let mut code = [0u8; 3];
        img.load_fill(&mut code, &Address::new(Rc::clone(&ram), 0x40_0154)).expect("entry is mapped");
        assert_eq!(code, [0x55, 0x89, 0xe5], "push ebp; mov ebp,esp");
    }

    /// The clamp in practice: the header page stops where the first section
    /// starts, so nothing it publishes can shadow real content.
    #[test]
    fn the_header_page_never_overlaps_a_section() {
        let path = format!("{}/tests/fixtures/pe_imports.exe", env!("CARGO_MANIFEST_DIR"));
        let bytes = std::fs::read(&path).expect("fixture");
        let img = ObjectLoadImage::from_bytes(&path, &bytes).expect("load the PE");
        let sections = img.section_snapshot();
        let &(hvma, hsize, hflags) = sections
            .iter()
            .min_by_key(|&&(vma, _, _)| vma)
            .expect("a PE has sections");
        assert_eq!(hflags, section_flags::DATA | section_flags::READONLY, "the header page sorts first");
        for &(vma, size, _) in &sections {
            if vma == hvma {
                continue;
            }
            assert!(vma >= hvma + hsize, "section 0x{vma:x}+0x{size:x} overlaps the header page");
        }
    }

    #[test]
    fn fauxware_plt_imports_resolve_to_named_functions() {
        let syms = fixture_funcsyms("fauxware");
        // PLT stubs → imported libc names at their stub entry addresses (classic
        // non-PIE x86-64, no CET: stub start == the `FF 25` jmp).
        for (addr, want) in [
            (0x400510u64, "puts"),
            (0x400520, "printf"),
            (0x400530, "read"),
            (0x400540, "__libc_start_main"),
            (0x400550, "strcmp"),
            (0x400560, "open"),
            (0x400570, "exit"),
        ] {
            assert_eq!(syms.get(&addr).map(String::as_str), Some(want), "PLT import at {addr:#x}");
        }
        // The pre-existing `.symtab` defined-function path still works.
        for (addr, want) in [
            (0x400664u64, "authenticate"),
            (0x4006ed, "accepted"),
            (0x4006fd, "rejected"),
            (0x40071d, "main"),
        ] {
            assert_eq!(syms.get(&addr).map(String::as_str), Some(want), "defined fn at {addr:#x}");
        }
        // No symbol at address 0 (the old UND-import-at-0x0 bug) and no
        // `@VERSION` suffix leaks through.
        assert!(!syms.contains_key(&0), "no function should be registered at 0x0");
        assert!(syms.values().all(|n| !n.contains('@')), "no @VERSION in names");
    }

    /// (kuna `dynrelocs`) A real PIE: the three `R_X86_64_RELATIVE` slots are
    /// filled in with `B + A`, the five `GLOB_DAT` / nine `JUMP_SLOT` entries all
    /// name *undefined* imports and must be left alone, and only the two RELATIVE
    /// slots that `PT_GNU_RELRO` (`0x3d78 + 0x288`) covers are reported constant —
    /// `0x4008` sits in writable `.data` past the RELRO end and must not be.
    #[test]
    fn pie_dynamic_relocations_are_applied_and_relro_slots_are_constant() {
        use kuna_sleigh::loadimage::LoadImage;
        let path = format!("{}/tests/fixtures/cet_pie_x86_64", env!("CARGO_MANIFEST_DIR"));
        let bytes = std::fs::read(&path).expect("fixture");
        let mut img = ObjectLoadImage::from_bytes(&path, &bytes).expect("load PIE");
        let m = manager();
        let ram = Rc::clone(m.get_space_by_name("ram").unwrap());
        img.attach_to_space(Rc::clone(&ram));

        let read_u64 = |img: &mut ObjectLoadImage, vma: u64| -> u64 {
            let mut buf = [0u8; 8];
            img.load_fill(&mut buf, &Address::new(Rc::clone(&ram), vma)).expect("mapped");
            u64::from_le_bytes(buf)
        };
        assert_eq!(read_u64(&mut img, 0x3d78), 0x1240, ".init_array RELATIVE slot");
        assert_eq!(read_u64(&mut img, 0x3d80), 0x1200, ".fini_array RELATIVE slot");
        assert_eq!(read_u64(&mut img, 0x4008), 0x4008, ".data RELATIVE slot");
        // The undefined GLOB_DAT / JUMP_SLOT imports keep the linker's bytes.
        assert_eq!(read_u64(&mut img, 0x3fd8), 0, "__libc_start_main GLOB_DAT untouched");

        assert_eq!(
            img.dynreloc_const_ranges(),
            &[(0x3d78u64, 0x3d7fu64), (0x3d80, 0x3d87)],
            "only the RELRO-covered slots are constant"
        );
    }

    #[test]
    fn cet_plt_sec_imports_resolve_at_call_targets() {
        // PIE + CET: calls target `.plt.sec` (`endbr64; FF 25`).  The stub entry
        // (the call target) is the endbr64, so names must land there, not at the
        // `FF 25` four bytes in.
        let syms = fixture_funcsyms("cet_pie_x86_64");
        assert_eq!(syms.get(&0x10d0).map(String::as_str), Some("free"));
        assert_eq!(syms.get(&0x10e0).map(String::as_str), Some("fread"));
        assert!(syms.values().any(|n| n == "fclose"), "fclose import");
        assert!(syms.values().any(|n| n == "memcmp"), "memcmp import");
        assert!(!syms.contains_key(&0));
        assert!(syms.values().all(|n| !n.contains('@')));
    }

    #[test]
    fn stripped_dynamic_plt_imports_resolve_without_symtab() {
        // No `.symtab`: PLT resolution must work off `.dynsym`/`.rela.plt` alone.
        let syms = fixture_funcsyms("stripped_dynamic_x86_64");
        for want in ["free", "fread", "fclose", "memcmp", "fprintf", "malloc"] {
            assert!(syms.values().any(|n| n == want), "missing import {want}");
        }
        assert!(!syms.contains_key(&0));
    }

    #[test]
    fn mips_stub_imports_resolve_to_named_functions() {
        // MIPS o32 (`plt_mips32`, big-endian, `-O0`): no `.plt`/`R_MIPS_JUMP_SLOT`.
        // The import stub→name correspondence comes from the dynamic-symbol GOT
        // layout (`DT_MIPS_LOCAL_GOTNO`/`DT_MIPS_GOTSYM`); `resolve_mips_imports`
        // names each import's `.MIPS.stubs` stub address (= the dynsym `st_value`).
        let syms = fixture_funcsyms("plt_mips32");
        // `puts`/`printf` are the user-visible imports.  (`__libc_start_main`'s stub
        // at 0x4007e0 == the `.MIPS.stubs` section start, so the `.symtab`
        // `_MIPS_STUBS_` section label is registered there first and wins the dedup
        // — a faithful, harmless overlap with a startup-only import.)
        for (addr, want) in [(0x400800u64, "puts"), (0x4007f0, "printf")] {
            assert_eq!(
                syms.get(&addr).map(String::as_str),
                Some(want),
                "MIPS import at {addr:#x}"
            );
        }
        // The defined `.symtab`/`.dynsym` `main` still resolves.
        assert_eq!(syms.get(&0x400700u64).map(String::as_str), Some("main"));
        // No symbol at 0x0 and no `@VERSION` leakage.
        assert!(!syms.contains_key(&0), "no function should be registered at 0x0");
        assert!(syms.values().all(|n| !n.contains('@')), "no @VERSION in names");
    }

    #[test]
    fn mips_got_const_ranges_cover_external_slots() {
        // The GOT external slots that hold the stub addresses must be reported as
        // constant ranges (so the engine folds the `lw $t9, off($gp)` indirect
        // load).  Layout: PLTGOT=0x411020, LOCAL_GOTNO=6, GOTSYM=5, ptr=4 →
        // got_index(i)=6+(i-5); puts(dynidx7)→slot 0x411040, printf(dynidx8)→0x411044.
        let path = format!("{}/tests/fixtures/plt_mips32", env!("CARGO_MANIFEST_DIR"));
        let bytes = std::fs::read(&path).unwrap();
        let file = object::File::parse(&*bytes).unwrap();
        let ranges = crate::loader::elf_plt::mips_got_const_ranges(&file);
        // Each range is a single 4-byte pointer slot.
        assert!(ranges.iter().all(|&(a, b)| b == a + 3), "4-byte slot ranges: {ranges:?}");
        let starts: std::collections::HashSet<u64> = ranges.iter().map(|&(a, _)| a).collect();
        assert!(starts.contains(&0x411040), "puts GOT slot 0x411040 in {ranges:?}");
        assert!(starts.contains(&0x411044), "printf GOT slot 0x411044 in {ranges:?}");
    }

    #[test]
    fn et_rel_ptx_fix_output_parameters_loads_and_rebases() {
        // (kuna) The ET_REL relocatable-object path on the real testcase binary
        // (angr `test_decompiling_ptx_fix_output_parameters`). Before this
        // feature, `ptx.o` (a `.o` with no PT_LOAD segments) mapped zero bytes
        // and every function failed with "Unable to load N bytes". Now the
        // SHF_ALLOC sections are laid out above RELOC_BASE (0x400000) and the
        // symbols rebased.
        let syms = fixture_funcsyms("ptx.o");
        // `fix_output_parameters` is a LOCAL FUNC at .text+0x660; .text is the
        // first SHF_ALLOC section -> RELOC_BASE, so it rebases to 0x400660
        // (matching angr's CLE default layout).
        assert_eq!(
            syms.get(&0x400660).map(String::as_str),
            Some("fix_output_parameters"),
            "fix_output_parameters must rebase to 0x400660"
        );
        // Undefined externals referenced by the `.rela.text` calls are bound to
        // synthetic call targets and named, so calls render by name.
        for want in ["strlen", "dcgettext", "error"] {
            assert!(syms.values().any(|n| n == want), "missing extern {want}");
        }
        // The bytes at `fix_output_parameters` actually load now (the original
        // bug was the DataUnavailError out of `load_fill`).
        let path = format!("{}/tests/fixtures/ptx.o", env!("CARGO_MANIFEST_DIR"));
        let bytes = std::fs::read(&path).unwrap();
        let m = manager();
        let ram = Rc::clone(m.get_space_by_name("ram").unwrap());
        let mut img = ObjectLoadImage::from_bytes(&path, &bytes).unwrap();
        img.attach_to_space(Rc::clone(&ram));
        let got = img.load(4, &Address::new(Rc::clone(&ram), 0x400660));
        assert!(got.is_ok(), "fix_output_parameters bytes must load: {got:?}");
    }

    #[test]
    fn faillog_data_symbols_carry_the_copy_reloc_externs() {
        // (GH-184) The stripped shadow `faillog` (identical bytes to
        // `tests/bug-repro/faillog`, md5 3bab801d856c5fdb175fecf069c2b4f5) defines
        // exactly four `STT_OBJECT` entries in `.dynsym` — the copy-relocated
        // (`R_X86_64_COPY`) libc externs. Before the data half of the symbol
        // walks was read, all four rendered `dat_<addr>`
        // (`__fprintf_chk(dat_61a0,...)` where every other decompiler prints
        // `stderr`). The `st_size` is load-bearing: an 8-byte load's
        // `queryContainer(addr, 8)` misses a size-1 install.
        let path =
            format!("{}/tests/fixtures/datasyms_faillog_x86_64", env!("CARGO_MANIFEST_DIR"));
        let bytes = std::fs::read(&path).unwrap_or_else(|e| panic!("read {path}: {e}"));
        let img = ObjectLoadImage::from_bytes(&path, &bytes).unwrap();
        let syms = img.data_symbols();
        let by_addr: std::collections::HashMap<u64, (String, u64)> =
            syms.iter().map(|(a, n, s)| (*a, (n.clone(), *s))).collect();
        for (addr, name, size) in [
            (0x6160u64, "stdout", 8u64),
            (0x61a0, "stderr", 8),
            (0x6168, "optind", 4),
            (0x6180, "optarg", 8),
        ] {
            assert_eq!(
                by_addr.get(&addr),
                Some(&(name.to_string(), size)),
                "expected {name} @ {addr:#x} size {size}; got {syms:?}"
            );
        }
        // Exactly the four defined OBJECT entries — no zero-size linker markers
        // (`__bss_start`, `_edata`, `_end` are sizeless and must be dropped), no
        // UND import placeholders, no `@VERSION` leakage (`stderr@GLIBC_2.2.5`
        // strips to `stderr`).
        assert_eq!(syms.len(), 4, "exactly the four copy-reloc externs: {syms:?}");
        assert!(syms.iter().all(|(_, n, _)| !n.contains('@')), "no @VERSION in names");
        assert!(syms.iter().all(|(_, _, s)| *s > 0), "no zero-size entries");
    }

    #[test]
    fn cpp_mangled_symbol_is_demangled_name_only() {
        // The kuna `GnuDemanglerAnalyzer` analog (`demangle`): a defined C++
        // method `_ZN3foo3Bar3bazEi` must surface in the funcsym stream as the
        // demangled NAME-ONLY form `foo::Bar::baz` — not the raw mangled symbol,
        // and not the full c++filt signature `foo::Bar::baz(int)` (the trailing
        // `(int)` would corrupt kuna's `::` scope splitter).
        let syms = fixture_funcsyms("cpp_mangled_x86_64");
        assert!(
            syms.values().any(|n| n == "foo::Bar::baz"),
            "demangled name-only `foo::Bar::baz` expected; got {syms:?}"
        );
        // The raw mangled symbol must NOT survive into the stream...
        assert!(
            !syms.values().any(|n| n.starts_with("_ZN3foo3Bar3baz")),
            "raw mangled symbol must be demangled"
        );
        // ...and no signature tail leaked through the name-only reduction.
        assert!(
            !syms.values().any(|n| n.contains('(') || n.contains('<')),
            "no signature / template leakage in funcsym names"
        );
        // `main` is a plain C-ABI name: not mangled, passes through unchanged.
        assert!(syms.values().any(|n| n == "main"), "main passes through");
    }

    #[test]
    fn msvc_mangled_coff_symbols_are_demangled_name_only() {
        // Multi-format loader PR-9: the loader's `demangle_funcsym_name` now feeds
        // the MSVC arm, so a `?`-prefixed `cl.exe` symbol from a COFF object
        // surfaces in the funcsym stream as the demangled NAME-ONLY form — not the
        // raw `?foo@Bar@@QEAAHH@Z`, and not the full `Bar::foo(int)` signature.
        // (`msvc_mangled.obj`: clang `-target x86_64-pc-windows-msvc`; cl.exe is
        // unavailable on Linux but the windows-msvc target emits real MSVC
        // mangling — see tests/fixtures/msvc_mangled.cpp.)
        let syms = fixture_funcsyms("msvc_mangled.obj");
        assert!(
            syms.values().any(|n| n == "Bar::foo"),
            "demangled name-only `Bar::foo` (from ?foo@Bar@@QEAAHH@Z) expected; got {syms:?}"
        );
        assert!(
            syms.values().any(|n| n == "ns::g"),
            "demangled name-only `ns::g` (from ?g@ns@@YAHHH@Z) expected; got {syms:?}"
        );
        assert!(
            syms.values().any(|n| n == "freefunc"),
            "demangled name-only `freefunc` (from ?freefunc@@YAHH@Z) expected; got {syms:?}"
        );
        // No raw `?`-mangled symbol survives, and no signature/template tail leaks
        // through the name-only reduction (it would corrupt the `::` scope splitter).
        assert!(
            !syms.values().any(|n| n.starts_with('?')),
            "raw MSVC-mangled symbol must be demangled, not passed through: {syms:?}"
        );
        assert!(
            !syms.values().any(|n| n.contains('(') || n.contains('@')),
            "no signature / raw-`@` leakage in funcsym names: {syms:?}"
        );
    }

    /// §2.2 fallback composition: a PE id (`...:windows`) pairs with the
    /// default-model fallback (`...:gcc`/`...:default`), and a Mach-O x86-64 id
    /// (`...:gcc`, already the arch default) pairs with `None` (primary == the
    /// default, so no fallback is needed).
    #[test]
    fn format_language_ids_pair_with_default_model_fallback() {
        let pe = pe_language_ids();
        // x86-64 PE: primary windows, fallback gcc.
        let x64 = pe
            .iter()
            .find(|(p, _)| p == "x86:LE:64:default:windows")
            .expect("PE x86-64 windows id present");
        assert_eq!(
            x64.1.as_deref(),
            Some("x86:LE:64:default:gcc"),
            "PE windows must fall back to the gcc default model"
        );

        let macho = macho_language_ids();
        // x86-64 Mach-O: primary gcc IS the default → no fallback.
        let mx64 = macho
            .iter()
            .find(|(p, _)| p == "x86:LE:64:default:gcc")
            .expect("Mach-O x86-64 gcc id present");
        assert_eq!(
            mx64.1, None,
            "Mach-O x86-64 gcc is already the default model — no fallback"
        );
    }
}
