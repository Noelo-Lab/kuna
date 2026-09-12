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
use std::sync::Arc;

use object::read::{Object, ObjectSection, ObjectSegment, ObjectSymbol};
use object::{Architecture, SegmentFlags, SymbolKind};

use kuna_base::address::{Address, RangeList};
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::space::AddrSpace;
use kuna_base::types::Wrap;

use kuna_sleigh::kuna_sharedbytes::windowed_load_fill;
use kuna_sleigh::loadimage::{
    section_flags, ImageBytes, LoadImage, LoadImageFunc, LoadImageSection, IMAGE_WINDOW_BYTES,
};

use kuna_decomp::kuna_symbolnamechars::{sanitize_symbol_name_bytes, symbolnamechars_mode, NameChars};

/// Default read-buffer size (C++ `LoadImageBfd::bufsize`, `loadimage_bfd.cc:36`).
const BUFSIZE: usize = IMAGE_WINDOW_BYTES;

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
/// purpose of [`SegmentBytes::find_section`]/`loadFill`): a vma, the bytes
/// that live there, and BFD-style section flags.
#[derive(Debug, Clone)]
struct Segment {
    /// Virtual address the bytes map to (C++ `asection::vma`).
    vma: u64,
    /// The bytes present at `vma` (a copy of the ELF segment's file data; the
    /// C++ `secsize` is `data.len()`).
    data: Vec<u8>,
    /// (kuna, `pe-zero-filled-data`) The segment's RAM footprint — ELF
    /// `p_memsz`, PE `VirtualSize`, Mach-O `vmsize` — which may exceed the file
    /// extent in [`Self::data`]. The excess is the `.bss`-style zero-filled
    /// tail: mapped memory the file does not back, which the loader must still
    /// hand out as zeroes rather than as a hole. Recorded only for a segment the
    /// image marks as data, and clamped at construction so a tail never reaches
    /// into the next segment.
    memsz: u64,
}

impl Segment {
    /// A segment whose file data is its whole RAM footprint.
    fn file_backed(vma: u64, data: Vec<u8>) -> Segment {
        let memsz = data.len() as u64; // cast: segment byte count
        Segment { vma, data, memsz }
    }

    /// The mapped extent at [`Self::vma`] — the file data plus any zero-filled
    /// RAM tail. Never shorter than the file extent.
    fn mapped_size(&self) -> u64 {
        self.memsz.max(self.data.len() as u64) // cast: segment byte count
    }
}

/// Trim every segment's zero-filled tail so it stops at the next segment's vma.
///
/// `segments` must already be vma-sorted. Only the tail is trimmed: a segment's
/// own file extent is left alone, so a layout that stacks several segments at
/// one address (a COFF object read through the linked path) maps exactly what it
/// mapped before.
fn clamp_virtual_tails(segments: &mut [Segment]) {
    for i in 0..segments.len() {
        let Some(next) = segments.get(i + 1).map(|s| s.vma) else {
            continue;
        };
        let seg = &mut segments[i];
        let room = next.saturating_sub(seg.vma);
        seg.memsz = seg.memsz.min(room).max(seg.data.len() as u64); // cast: byte count
    }
}

/// (kuna) The image's mapped bytes: the vma-sorted segment list and the
/// byte-serving routines over it.
///
/// Split out of [`ObjectLoadImage`] so the bytes can outlive one reader and be
/// read from another thread ([`ImageBytes`]). Every read of image bytes -- the
/// loader's own `load_fill` included -- goes through this one implementation, so
/// a second reader cannot drift from the first.
///
/// Written only at load time, before any share exists; [`ObjectLoadImage`] holds
/// it behind an `Arc` and requires sole ownership at each write site.
#[derive(Debug)]
struct SegmentBytes {
    segments: Vec<Segment>,
}

impl SegmentBytes {
    /// Find the segment containing `offset`, or the closest segment above it
    /// (C++ `LoadImageBfd::findSection`, `loadimage_bfd.cc:99`).  Returns the
    /// index into [`Self::segments`] and the segment size, or `None` for "no
    /// segment at or above `offset`" (the C++ null `champ`).
    ///
    /// The size is the segment's *mapped* extent ([`Segment::mapped_size`]), so
    /// an address in a zero-filled RAM tail resolves to the segment that maps
    /// it instead of falling through to the "closest above" scan.
    fn find_section(&self, offset: u64) -> Option<(usize, u64)> {
        // First pass: the segment that actually contains `offset`.
        for (i, s) in self.segments.iter().enumerate() {
            let start = s.vma;
            let secsize = s.mapped_size();
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
                return Some((i, s.mapped_size()));
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

impl ImageBytes for SegmentBytes {
    /// Fill `dst` with the image bytes starting at address `start`, zero-filling
    /// any part of the span no segment maps (a `.bss`-style RAM tail, or a hole
    /// between two segments).  This is the body of the C++
    /// `LoadImageBfd::loadFill` read loop, lifted out so it can fill either the
    /// staging buffer or the caller's slice directly.
    ///
    /// Returns the number of bytes it could not fill: `0` on success, and
    /// `dst.len()` when the very first byte is unmapped (the C++ `offset == 0`
    /// break, which is the only failure the loop reports).  The loop counter is
    /// a `usize` rather than the C++ `int4`, so a span longer than 2 GiB is
    /// still counted correctly instead of wrapping negative.
    fn fill_span(&self, dst: &mut [u8], start: u64) -> usize {
        let mut offset: usize = 0;
        let mut cursize: usize = dst.len();
        let mut curaddr = start;

        while cursize > 0 {
            let Some((idx, secsize)) = self.find_section(curaddr) else {
                if offset == 0 {
                    break; // Initial address not mapped
                }
                // Fill the rest with zero.
                dst[offset..].fill(0);
                return 0;
            };
            let seg_vma = self.segments[idx].vma;
            let readsize: usize;
            if seg_vma > curaddr {
                // No section matches at curaddr: zeroes to the next section.
                if offset == 0 {
                    break; // Initial address not mapped
                }
                readsize = (seg_vma - curaddr).min(cursize as u64) as usize;
                dst[offset..offset + readsize].fill(0);
            } else {
                let avail = seg_vma.wadd(secsize).wsub(curaddr);
                readsize = avail.min(cursize as u64) as usize;
                let seg_off = curaddr - seg_vma; // file-relative read offset
                self.copy_segment(idx, seg_off, &mut dst[offset..offset + readsize]);
            }
            offset += readsize;
            cursize -= readsize;
            curaddr = curaddr.wadd(readsize as u64);
        }
        cursize
    }


    fn mapped_covers(&self, lo: u64, hi: u64) -> bool {
        let mut cur = lo;
        while cur < hi {
            // The furthest mapped extent of every segment that contains `cur`:
            // segments may stack at one vma (a COFF object read through the
            // linked path), so the first hit is not necessarily the best.
            let mut reach: Option<u64> = None;
            for s in &self.segments {
                let stop = s.vma.wadd(s.mapped_size());
                if cur >= s.vma && cur < stop {
                    reach = Some(match reach {
                        Some(r) => r.max(stop),
                        None => stop,
                    });
                }
            }
            match reach {
                Some(stop) if stop > cur => cur = stop,
                _ => return false,
            }
        }
        true
    }
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

/// One named section as format-neutral export metadata: the project README's
/// section table. `kind` is the parsed object's `SectionKind` rendered with
/// `Debug`, which is what the README printed from its own re-parse before the
/// loader carried the names.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectSectionMetadata {
    pub name: String,
    pub vma: u64,
    pub size: u64,
    pub kind: String,
}

impl ObjectSectionMetadata {
    /// The one place the `SectionKind` is rendered for export.
    pub fn new(name: &str, vma: u64, size: u64, kind: object::SectionKind) -> Self {
        Self { name: name.to_string(), vma, size, kind: format!("{kind:?}") }
    }
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
    /// `find_section`/`loadFill`), behind an `Arc` so a decoder on another thread
    /// can read the same bytes. Every write site requires sole ownership: the
    /// image is fully built -- relocations patched, overlays applied, vmas
    /// adjusted -- before anything shares it.
    bytes: Arc<SegmentBytes>,
    /// ELF sections, in file order (the BFD `asection` list for the info walks).
    sections: Vec<SectionInfo>,
    /// The named sections as export metadata, in file order; see
    /// [`ObjectSectionMetadata`].
    section_metadata: Vec<ObjectSectionMetadata>,
    /// The container's declared entry as a code address (the Thumb bit folded
    /// on ARM), `None` for a relocatable object or an image that declares none.
    image_entry: Option<u64>,
    /// (kuna) The loadable segments as mapping *metadata*, ascending by vma —
    /// the same `(vma, size, flags)` shape as [`Self::sections`], reported by
    /// `getSegments` so a section-keyed reader has a container to fall back on
    /// when the image carries no usable section table. Distinct from
    /// [`Self::segments`], which owns the bytes and covers only what the file
    /// backs; this records each segment's whole RAM footprint. Empty for an
    /// `ET_REL` load, which has no program headers.
    segment_info: Vec<SectionInfo>,
    /// Executable ELF segment extents, used when section headers are absent.
    executable_segments: Vec<(u64, u64)>,
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
    /// (kuna) The image's **import pointer slots** as half-open `[lo, hi)` VMA
    /// ranges — the PE Import Address Table words a run-time loader fills with a
    /// resolved import address. [`Self::funcsyms`] carries a `FunctionSymbol` at
    /// each of them so a `call [slot]` renders the import name; this records that
    /// those addresses hold a POINTER rather than a function body, which is what
    /// a whole-binary enumeration needs to keep them out of the decompile set.
    /// Empty for every non-PE image and for a relocatable object.
    import_slots: Vec<(u64, u64)>,
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
        Self::from_bytes_with_diagnostics(filename, bytes, true, None)
    }

    /// Open an object while letting an explicit SLEIGH language select the
    /// decoder. The recognized container still supplies all image mappings.
    pub fn from_bytes_with_target(
        filename: &str,
        bytes: &[u8],
        target: &str,
    ) -> KunaResult<ObjectLoadImage> {
        Self::from_bytes_with_diagnostics(filename, bytes, true, Some(target))
    }

    /// Reconstruct a load image for an internal analysis consumer without
    /// repeating diagnostics already emitted by the primary load.
    pub fn from_bytes_silent(filename: &str, bytes: &[u8]) -> KunaResult<ObjectLoadImage> {
        Self::from_bytes_with_diagnostics(filename, bytes, false, None)
    }

    /// Silent counterpart of [`Self::from_bytes_with_target`] for an internal
    /// analysis-side reconstruction of the primary load.
    pub fn from_bytes_silent_with_target(
        filename: &str,
        bytes: &[u8],
        target: &str,
    ) -> KunaResult<ObjectLoadImage> {
        Self::from_bytes_with_diagnostics(filename, bytes, false, Some(target))
    }

    fn from_bytes_with_diagnostics(
        filename: &str,
        bytes: &[u8],
        emit_diagnostics: bool,
        target: Option<&str>,
    ) -> KunaResult<ObjectLoadImage> {
        // Parse the object file.
        let file = parse_object(bytes).map_err(|e| {
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

        let target = target.and_then(explicit_language_target);
        if emit_diagnostics {
            if let Some(note) = target.and_then(|t| target_endian_note(&file, t)) {
                kuna_base::notes::say_once(filename, &format!("[kuna target] {filename}: {note}"));
            }
        }
        let effective_arch = effective_architecture(&file, bytes);
        let archtype = language_id_for(
            &file,
            fmt.as_ref(),
            bytes,
            filename,
            effective_arch,
            target,
        )?;
        // (kuna §2.2) The default-model fallback id: the same arch/endian stem
        // with the model dropped to the per-arch default. If the format's chosen
        // model (e.g. PE's `:windows`) isn't vendored for this arch, the engine
        // retries with this before erroring.
        let fallback_archtype = target
            .is_none()
            .then(|| fallback_language_id(&file, effective_arch, &archtype))
            .flatten();

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
                fallback_archtype,
                emit_diagnostics,
            );
        }

        // Snapshot the loadable segments (PT_LOAD), copying their RAM bytes.
        // `data()` returns only the file-backed bytes; a segment's `size()`
        // (its RAM footprint) may exceed that for `.bss`-style tails, so both
        // are kept: `data` is the file extent and `memsz` the mapped extent,
        // and a read past the file extent zero-fills to `memsz` the way BFD
        // reports a `SEC_LOAD`-less section.
        let mut segments: Vec<Segment> = Vec::new();
        let mut segment_info: Vec<SectionInfo> = Vec::new();
        let mut executable_segments = Vec::new();
        for seg in file.segments() {
            let vma = seg.address();
            if matches!(seg.flags(), object::SegmentFlags::Elf { p_flags }
                if p_flags & object::elf::PF_X != 0)
            {
                executable_segments.push((vma, seg.size()));
            }
            let data = seg.data().map_err(|e| {
                KunaError::lowlevel(format!("File: {filename} : unreadable segment data: {e}"))
            })?;
            // The metadata records the RAM footprint whether or not the file
            // backs it, so a `.bss` tail still reads as mapped.
            let bits = segment_bits(seg.flags());
            if seg.size() != 0 {
                segment_info.push(SectionInfo { vma, size: seg.size(), flags: bits });
            }
            // (kuna, `pe-zero-filled-data`) The RAM footprint is what the tail
            // is zero-filled to — but only where the image says the region holds
            // data. An *executable* uninitialized region is a packer's staging
            // area (`UPX0`: `VirtualSize` 0x9000 over `SizeOfRawData` 0), and
            // its file-time contents are not its run-time contents, so handing
            // out zeroes there would put 36 KB of `add [eax],al` into the
            // instruction stream in place of the honest "unpack first".
            let memsz = if bits & section_flags::CODE != 0 { 0 } else { seg.size() };
            if data.is_empty() && memsz == 0 {
                continue;
            }
            segments.push(Segment { vma, data: data.to_vec(), memsz });
        }
        // (kuna, `pe-header-entry-mapped`) The header page a format maps ahead
        // of its first section. `file.segments()` enumerates sections, so a PE's
        // `SizeOfHeaders` bytes at `ImageBase` were mapped nowhere and an entry
        // declared inside them read back "not mapped" even with an explicit
        // `--define-function`. Read-only, as Windows maps it, and DATA unless
        // the image declares its own entry point inside it (a packer stub in the
        // slack after the section table), so the code scans stay out of every
        // PE's MZ/PE bytes without also hiding the one address the image itself
        // calls code.
        // `None` for every other format; see [`crate::loader::pe_headers`].
        let header = fmt.header_region(&file, bytes).map(|region| SectionInfo {
            vma: region.vma,
            size: region.len as u64, // cast: clamped to the file length
            flags: if region.code { section_flags::CODE } else { section_flags::DATA }
                | section_flags::READONLY,
        });
        if let Some(region) = &header {
            segments.push(Segment::file_backed(
                region.vma,
                bytes[..region.size as usize].to_vec(), // cast: ditto
            ));
            segment_info.push(region.clone());
        }
        segment_info.sort_by_key(|s| s.vma);
        // Ascending vma order so find_section's "closest greater" walk is a
        // simple scan (the BFD list is already address-ordered for ELF).
        segments.sort_by_key(|s| s.vma);
        clamp_virtual_tails(&mut segments);

        // Snapshot the sections for the info walks (the BFD `asection` list).
        // (C) The per-format section-flag translation goes through the boundary; for
        // ELF this is the old `section_kind_flags` body, lifted verbatim into
        // `ElfFormat::section_bits`.
        let mut sections: Vec<SectionInfo> = Vec::new();
        let mut section_metadata: Vec<ObjectSectionMetadata> = Vec::new();
        for sec in file.sections() {
            let flags = fmt.section_bits(sec.kind(), sec.flags());
            sections.push(SectionInfo { vma: sec.address(), size: sec.size(), flags });
            if let Ok(name) = sec.name() {
                section_metadata.push(ObjectSectionMetadata::new(
                    name,
                    sec.address(),
                    sec.size(),
                    sec.kind(),
                ));
            }
        }
        sections.extend(header);
        let arm32_decoder = is_arm32_language(&String::from_utf8_lossy(&archtype));
        let image_entry = crate::analyzers::entry::image_entry_vma(&file, bytes)
            .map(|vma| if arm32_decoder { vma & !1 } else { vma });

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

        // (kuna) The import pointer slots the funcsym walk above just registered
        // names at — recorded as addresses so the enumeration can tell a pointer
        // word from a function entry. Empty off-PE.
        let import_slots = fmt.import_slots(&file, bytes);

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
            bytes: Arc::new(SegmentBytes { segments }),
            sections,
            section_metadata,
            image_entry,
            segment_info,
            executable_segments,
            funcsyms,
            reloc_sections: Vec::new(),
            reloc_symbols: Vec::new(),
            datasyms,
            const_ranges,
            dynreloc_const,
            import_slots,
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
        fallback_archtype: Option<Vec<u8>>,
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
            kuna_base::notes::say_once(
                filename,
                &format!("[kuna msvcfpconst] {filename}: {w}"),
            );
        }

        // One bounded report per loader construction. Analysis-side consumers
        // may construct another layout for address rebasing, but never print it.
        if emit_diagnostics {
            for line in layout.diagnostics.report_lines() {
                kuna_base::notes::say_once(
                    filename,
                    &format!("[kuna ET_REL loader] {filename}: {line}"),
                );
            }
        }

        let mut segments: Vec<Segment> = layout
            .segments
            .into_iter()
            .map(|(vma, data)| Segment::file_backed(vma, data))
            .collect();
        segments.extend(
            fpconst.writes.into_iter().map(|(vma, data)| Segment::file_backed(vma, data)),
        );
        segments.sort_by_key(|s| s.vma);

        let sections: Vec<SectionInfo> = layout
            .sections
            .into_iter()
            .map(|(vma, size, flags)| SectionInfo { vma, size, flags })
            .collect();
        // The laid-out sections at their synthetic load VMAs, so the export
        // metadata names the addresses the rest of the export uses.
        let section_metadata: Vec<ObjectSectionMetadata> = file
            .sections()
            .filter_map(|sec| {
                let vma = *layout.section_vma.get(&sec.index())?;
                Some(ObjectSectionMetadata::new(sec.name().ok()?, vma, sec.size(), sec.kind()))
            })
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

        Ok(ObjectLoadImage {
            filename: filename.to_string(),
            archtype,
            fallback_archtype,
            bytes: Arc::new(SegmentBytes { segments }),
            sections,
            section_metadata,
            // A pre-link object declares no entry.
            image_entry: None,
            // No program headers on a relocatable object: the section table is
            // the only mapping story it has, and it always has one.
            segment_info: Vec::new(),
            executable_segments: Vec::new(),
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
            // A pre-link object has no Import Address Table: its external calls
            // are relocations, resolved by the layout pass.
            import_slots: Vec::new(),
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

    /// The named sections as export metadata (see [`ObjectSectionMetadata`]).
    pub fn section_metadata(&self) -> &[ObjectSectionMetadata] {
        &self.section_metadata
    }

    /// The container's declared entry as a code address, if it declares one.
    pub fn image_entry(&self) -> Option<u64> {
        self.image_entry
    }

    /// Executable `(vma, size)` extents. Sectionless ELF images use PF_X
    /// PT_LOAD ranges, including their in-memory tails.
    pub fn executable_ranges(&self) -> Vec<(u64, u64)> {
        if self.sections.is_empty() {
            return self.executable_segments.clone();
        }
        self.sections
            .iter()
            .filter(|s| s.flags & section_flags::CODE != 0)
            .map(|s| (s.vma, s.size))
            .collect()
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

    /// (kuna) The image's import pointer slots as half-open `[lo, hi)` VMA
    /// ranges — the PE Import Address Table words.
    ///
    /// The loader registers a `FunctionSymbol` at each slot so a
    /// `call dword ptr [slot]` renders the import name, and that registration is
    /// what a name-keyed lookup and an explicit `--addr` selection resolve
    /// through. It is not a claim that a function BODY starts there: the slot
    /// holds a pointer the run-time loader writes. The engine reports these
    /// ranges so its whole-binary enumeration can decompile the image's code
    /// without inventing a body for every import.
    pub fn import_slot_ranges(&self) -> &[(u64, u64)] {
        &self.import_slots
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

    /// Write `data` over the mapped bytes at `vma` (the write twin of
    /// [`SegmentBytes::fill_span`], and the loader half of `--assert bytes`).
    ///
    /// Resolved through [`SegmentBytes::find_section`] so an overlay lands in
    /// exactly the segment a read at the same address would come from, and
    /// refused unless that segment maps the whole span: a partial write would
    /// leave half a stated instruction stream in place, which is worse than not
    /// taking the statement at all.  A span reaching into the segment's
    /// zero-filled RAM tail materialises that tail first — the caller is stating
    /// what the running program put there, which is precisely a `.bss`-style
    /// region's content.
    fn overlay_span(&mut self, vma: u64, data: &[u8]) -> Result<(), String> {
        if data.is_empty() {
            return Err("an overlay needs at least one byte".into());
        }
        let end = vma
            .checked_add(data.len() as u64) // cast: overlay byte count
            .ok_or_else(|| "the overlay wraps past the end of the address space".to_string())?;
        let bytes = Arc::get_mut(&mut self.bytes).ok_or_else(|| {
            "cannot overlay image bytes while a shared decode view is active".to_string()
        })?;
        let idx = bytes
            .find_section(vma)
            .filter(|(idx, secsize)| {
                let seg_vma = bytes.segments[*idx].vma;
                vma >= seg_vma && end <= seg_vma.wadd(*secsize)
            })
            .map(|(idx, _)| idx)
            .ok_or_else(|| format!("no loaded segment maps {vma:#x}-{end:#x}"))?;
        let seg = &mut bytes.segments[idx];
        let off = (vma - seg.vma) as usize; // cast: offset within a mapped segment
        let stop = off + data.len();
        if stop > seg.data.len() {
            seg.data.resize(stop, 0);
        }
        seg.data[off..stop].copy_from_slice(data);
        // The 512-byte read window may straddle what just moved; drop it.
        *self.bufoffset.borrow_mut() = !0u64;
        Ok(())
    }

}

impl LoadImage for ObjectLoadImage {
    fn get_file_name(&self) -> &str {
        &self.filename
    }

    fn load_fill(&mut self, ptr: &mut [u8], addr: &Address) -> KunaResult<()> {
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
        windowed_load_fill(&*self.bytes, &self.buffer, &self.bufoffset, ptr, addr)
    }

    fn shared_bytes(&self) -> Option<Arc<dyn ImageBytes>> {
        Some(Arc::clone(&self.bytes) as Arc<dyn ImageBytes>)
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

    fn kuna_overlay_bytes(&mut self, addr: &Address, data: &[u8]) -> KunaResult<()> {
        let space = addr
            .get_space()
            .expect("ObjectLoadImage::kuna_overlay_bytes: address with null space");
        match &self.spaceid {
            Some(sp) if Rc::ptr_eq(sp, space) => {}
            _ => {
                return Err(KunaError::data_unavail(format!(
                    "Trying to overlay loadimage bytes in space: {}",
                    space.get_name()
                )));
            }
        }
        self.overlay_span(addr.get_offset(), data).map_err(KunaError::data_unavail)
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
        let bytes = Arc::get_mut(&mut self.bytes).expect("image bytes already shared");
        for s in &mut bytes.segments {
            s.vma = s.vma.wadd(badjust);
        }
        for s in &mut self.sections {
            s.vma = s.vma.wadd(badjust);
        }
        for s in &mut self.segment_info {
            s.vma = s.vma.wadd(badjust);
        }
        for (vma, _) in &mut self.executable_segments {
            *vma = vma.wadd(badjust);
        }
        for s in &mut self.funcsyms {
            s.addr = s.addr.wadd(badjust);
        }
        for s in &mut self.section_metadata {
            s.vma = s.vma.wadd(badjust);
        }
        if let Some(entry) = &mut self.image_entry {
            *entry = entry.wadd(badjust);
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
    arch: Architecture,
    target: Option<&str>,
) -> KunaResult<Vec<u8>> {
    if let Some(target) = target {
        return Ok(target.as_bytes().to_vec());
    }
    let little = file.is_little_endian();
    let endian = if little { "LE" } else { "BE" };
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
fn fallback_language_id(
    file: &object::File,
    arch: Architecture,
    primary: &[u8],
) -> Option<Vec<u8>> {
    let endian = if file.is_little_endian() { "LE" } else { "BE" };
    let fallback = compose_language_id(arch, endian, None)?;
    if fallback.as_bytes() == primary {
        None
    } else {
        Some(fallback.into_bytes())
    }
}

use object::pe::{IMAGE_FILE_MACHINE_ARM, IMAGE_FILE_MACHINE_ARMNT, IMAGE_FILE_MACHINE_THUMB};

/// Parse an object, including bare THUMB COFF omitted by `object`'s magic dispatch.
pub fn parse_object(bytes: &[u8]) -> object::read::Result<object::File<'_>> {
    if bytes.starts_with(&IMAGE_FILE_MACHINE_THUMB.to_le_bytes()) {
        object::read::coff::CoffFile::parse(bytes).map(object::File::Coff)
    } else {
        object::File::parse(bytes)
    }
}

/// Architecture used for language selection when the neutral parser does not
/// recognize a container's machine value. PE/COFF machines `0x01c0` (`ARM`)
/// and `0x01c2` (`THUMB`) are both documented 32-bit little-endian ARM code
/// values the parser leaves `Unknown`; what each says about the decode mode is
/// [`pe_arm_mode_policy`]'s question, not this one's.
pub fn effective_architecture(file: &object::File, bytes: &[u8]) -> Architecture {
    let parsed = file.architecture();
    if parsed != Architecture::Unknown {
        return parsed;
    }
    match container_machine(file, bytes) {
        Some(IMAGE_FILE_MACHINE_ARM | IMAGE_FILE_MACHINE_THUMB) => Architecture::Arm,
        _ => parsed,
    }
}

/// The PE-family machine words kuna ships a SLEIGH language for. One list,
/// read by the bare-COFF dispatcher, the TE probe, and both loaders' machine
/// tables, so a machine added to one is added to all.
pub const PE_MACHINES: [u16; 6] = [
    object::pe::IMAGE_FILE_MACHINE_I386,
    object::pe::IMAGE_FILE_MACHINE_AMD64,
    IMAGE_FILE_MACHINE_ARM,
    IMAGE_FILE_MACHINE_THUMB,
    IMAGE_FILE_MACHINE_ARMNT,
    object::pe::IMAGE_FILE_MACHINE_ARM64,
];

/// The `object` architecture a PE-family machine word names, for
/// [`PE_MACHINES`]. The three ARM words share one architecture; what each says
/// about the decode mode is [`pe_arm_mode_policy`]'s question.
pub fn pe_machine_architecture(machine: u16) -> Option<Architecture> {
    Some(match machine {
        object::pe::IMAGE_FILE_MACHINE_I386 => Architecture::I386,
        object::pe::IMAGE_FILE_MACHINE_AMD64 => Architecture::X86_64,
        IMAGE_FILE_MACHINE_ARM | IMAGE_FILE_MACHINE_THUMB | IMAGE_FILE_MACHINE_ARMNT => {
            Architecture::Arm
        }
        object::pe::IMAGE_FILE_MACHINE_ARM64 => Architecture::Aarch64,
        _ => return None,
    })
}

/// Whether a SLEIGH language id decodes as 32-bit ARM (processor `ARM`, width
/// `32`) — the property every Thumb-bit rule keys on.
pub fn is_arm32_language(arch_id: &str) -> bool {
    let mut fields = arch_id.split(':');
    fields.next() == Some("ARM") && fields.nth(1) == Some("32")
}

/// What a PE-family machine word says about the ARM decode mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArmModePolicy {
    /// The whole instruction stream is Thumb.
    WholeImageThumb,
    /// Only the entry's low bit is evidence; the `entrythumbflow` walk carries
    /// it along the reachable flow.
    EntryBit,
}

/// The one table both PE-family loaders read. `ARMNT` is Thumb-only by
/// definition on every container and `ARM` may interwork on every container;
/// `THUMB` (`0x1c2`) is the word the two families read differently. Windows
/// names it Thumb and a PE carrying it is painted as such; the UEFI bindings
/// name the same value `ARMTHUMB_MIXED` and its modules interwork, so a TE
/// carrying it relies on the entry bit. That is a container-family decision,
/// and it is made here once rather than in each loader.
pub fn pe_arm_mode_policy(machine: u16, uefi_te: bool) -> Option<ArmModePolicy> {
    match machine {
        IMAGE_FILE_MACHINE_ARMNT => Some(ArmModePolicy::WholeImageThumb),
        IMAGE_FILE_MACHINE_THUMB if !uefi_te => Some(ArmModePolicy::WholeImageThumb),
        IMAGE_FILE_MACHINE_THUMB | IMAGE_FILE_MACHINE_ARM => Some(ArmModePolicy::EntryBit),
        _ => None,
    }
}

/// Container-level whole-image ARM decode-mode evidence for a PE/COFF image:
/// `Some(true)` when [`pe_arm_mode_policy`] says the whole stream is Thumb.
/// Other formats and machines provide no whole-image hint.
pub fn arm_isa_hint(file: &object::File, bytes: &[u8]) -> Option<bool> {
    match pe_arm_mode_policy(container_machine(file, bytes)?, false)? {
        ArmModePolicy::WholeImageThumb => Some(true),
        ArmModePolicy::EntryBit => None,
    }
}

/// The normalized entry of a PE image whose machine word leaves the mode to the
/// entry bit and whose `AddressOfEntryPoint` carries it: the seed for the
/// `entrythumbflow` walk. `None` for every other image.
pub fn arm_entry_thumb_hint(file: &object::File, bytes: &[u8]) -> Option<u64> {
    if pe_arm_mode_policy(container_machine(file, bytes)?, false)? != ArmModePolicy::EntryBit {
        return None;
    }
    let (entry, thumb) = crate::analyzers::entry::image_code_entry(file, bytes)?;
    thumb.then_some(entry)
}

fn container_machine(file: &object::File, bytes: &[u8]) -> Option<u16> {
    match file.format() {
        object::BinaryFormat::Pe => pe_machine(bytes),
        object::BinaryFormat::Coff => bytes
            .get(..2)
            .map(|raw| u16::from_le_bytes([raw[0], raw[1]])),
        _ => None,
    }
}

fn pe_machine(bytes: &[u8]) -> Option<u16> {
    let pe_offset = u32::from_le_bytes(bytes.get(0x3c..0x40)?.try_into().ok()?) as usize;
    if bytes.get(pe_offset..pe_offset.checked_add(4)?)? != b"PE\0\0" {
        return None;
    }
    let machine = bytes.get(pe_offset.checked_add(4)?..pe_offset.checked_add(6)?)?;
    Some(u16::from_le_bytes([machine[0], machine[1]]))
}

/// Empty input and the `default` sentinel request container-derived selection,
/// including the normal compiler-model fallback.
pub fn explicit_language_target(target: &str) -> Option<&str> {
    match target.trim() {
        "" | "default" => None,
        target => Some(target),
    }
}

/// An explicit target whose endian field contradicts the container's. Reported,
/// not refused: `--target` overrides what the container declares, and forcing a
/// byte-swapped decode of a mislabeled image is a legitimate use of it.
fn target_endian_note(file: &object::File, target: &str) -> Option<String> {
    let endian = target
        .split(':')
        .nth(1)
        .filter(|value| matches!(*value, "LE" | "BE"))?;
    let container = if file.is_little_endian() { "LE" } else { "BE" };
    (endian != container).then(|| {
        format!("target {target:?} is {endian}-endian but the container is {container}-endian")
    })
}

/// Compose the SLEIGH language id from the format-neutral `arch` + `endian`
/// stem and the per-format compiler `model`. `None` for an arch kuna has no
/// `.sla` stem for. The single source of truth for the id string shape, shared
/// by [`language_id_for`] and [`elf_language_ids`] so the resolves-in-the-DB
/// test cannot drift from the producer.
pub(crate) fn compose_language_id(arch: Architecture, endian: &str, model: Option<&str>) -> Option<String> {
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

    #[test]
    fn executable_ranges_use_sectionless_load_segments_and_rebase() {
        let mut bytes = build_elf64(0x1000, &[0x90, 0xc3], None);
        bytes[40..48].fill(0); // e_shoff
        bytes[58..64].fill(0); // e_shentsize, e_shnum, e_shstrndx
        bytes[104..112].copy_from_slice(&16u64.to_le_bytes()); // p_memsz, including zero-fill
        let mut image = ObjectLoadImage::from_bytes("synthetic", &bytes).unwrap();
        assert!(image.section_snapshot().is_empty());
        assert_eq!(image.executable_ranges(), vec![(0x1000, 16)]);
        image.attach_to_space(Rc::clone(manager().get_default_code_space().unwrap()));
        image.adjust_vma(0x2000);
        assert_eq!(image.executable_ranges(), vec![(0x3000, 16)]);

        bytes[68..72].copy_from_slice(&object::elf::PF_R.to_le_bytes());
        let image = ObjectLoadImage::from_bytes("synthetic", &bytes).unwrap();
        assert!(
            image.executable_ranges().is_empty(),
            "non-executable PT_LOAD was included"
        );

        bytes[68..72].copy_from_slice(&(object::elf::PF_R | object::elf::PF_X).to_le_bytes());
        bytes[64..68].copy_from_slice(&object::elf::PT_NOTE.to_le_bytes());
        let image = ObjectLoadImage::from_bytes("synthetic", &bytes).unwrap();
        assert!(
            image.executable_ranges().is_empty(),
            "non-loadable segment was included"
        );
    }

    #[test]
    fn executable_ranges_prefer_sections_to_broader_segments() {
        let mut bytes = build_elf64(0x1000, &[0x90, 0xc3], None);
        bytes[104..112].copy_from_slice(&16u64.to_le_bytes());
        let image = ObjectLoadImage::from_bytes("synthetic", &bytes).unwrap();
        assert_eq!(image.executable_ranges(), vec![(0x1000, 2)]);
    }

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

    #[test]
    fn shared_reader_matches_owner_and_blocks_late_overlays() {
        let payload: Vec<u8> = (0..700).map(|i| (i % 251) as u8).collect();
        let object = build_elf64(0x1000, &payload, None);
        let mut owner = ObjectLoadImage::from_bytes("synthetic", &object).unwrap();
        let ram = Rc::clone(manager().get_default_code_space().unwrap());
        owner.attach_to_space(Rc::clone(&ram));

        let shared = owner
            .shared_bytes()
            .expect("object images publish immutable bytes");
        assert!(shared.mapped_covers(0x1000, 0x1000 + payload.len() as u64));
        assert!(!shared.mapped_covers(0x0fff, 0x1001));
        let mut reader = kuna_sleigh::kuna_sharedbytes::SharedBytesImage::new("synthetic", shared);
        reader.attach_to_space(Rc::clone(&ram));

        for (off, len) in [(0x1000, 8), (0x1004, 17), (0x11f8, 40), (0x1000, 600)] {
            let addr = Address::new(Rc::clone(&ram), off);
            let mut want = vec![0; len];
            let mut got = vec![0; len];
            owner.load_fill(&mut want, &addr).unwrap();
            reader.load_fill(&mut got, &addr).unwrap();
            assert_eq!(
                got, want,
                "shared read differs at {off:#x} for {len} bytes"
            );
        }

        let err = owner
            .overlay_span(0x1000, &[0xcc])
            .expect_err("published bytes must stay immutable");
        assert!(err.contains("shared decode view"));
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

    /// (kuna `--assert bytes`) The overlay is a statement about RAM: every later
    /// read serves it, the file on disk is untouched, and an address no segment
    /// maps is refused rather than invented.
    #[test]
    fn an_overlay_replaces_what_every_later_read_returns() {
        use kuna_sleigh::loadimage::LoadImage;
        let m = manager();
        let ram = Rc::clone(m.get_space_by_name("ram").unwrap());
        let elf = build_elf64(0x401000, &[0x55, 0x48, 0x89, 0xe5], None);
        let mut img = ObjectLoadImage::from_bytes("t.elf", &elf).unwrap();
        img.attach_to_space(Rc::clone(&ram));

        // Warm the 512-byte read window first: an overlay that did not drop it
        // would be invisible to exactly the reads most likely to follow one.
        assert_eq!(
            img.load(4, &Address::new(Rc::clone(&ram), 0x401000)).unwrap(),
            vec![0x55, 0x48, 0x89, 0xe5]
        );
        img.kuna_overlay_bytes(&Address::new(Rc::clone(&ram), 0x401001), &[0x31, 0xc0])
            .expect("inside the segment");
        assert_eq!(
            img.load(4, &Address::new(Rc::clone(&ram), 0x401000)).unwrap(),
            vec![0x55, 0x31, 0xc0, 0xe5]
        );

        // Unmapped, and straddling the end of the only segment: both refused,
        // naming the span, because a half-applied overlay is worse than none.
        for (vma, len) in [(0x1000u64, 1usize), (0x401002, 4)] {
            let err = img
                .kuna_overlay_bytes(&Address::new(Rc::clone(&ram), vma), &vec![0x90; len])
                .unwrap_err();
            match &err {
                KunaError::DataUnavail { explain } => {
                    assert!(explain.contains("no loaded segment maps"), "got {explain}");
                }
                other => panic!("expected DataUnavail, got {other:?}"),
            }
        }
        // A wrong-space overlay is the loadFill contract, not a panic.
        let other = Rc::clone(m.get_space_by_name("const").unwrap());
        assert!(img.kuna_overlay_bytes(&Address::new(other, 0), &[0x90]).is_err());
    }

    /// A writable segment's zero-filled RAM tail is real mapped memory, so an
    /// overlay reaching into it materialises the tail rather than refusing: what
    /// the running program left there is exactly what a caller is stating.
    #[test]
    fn an_overlay_may_reach_into_a_zero_filled_tail() {
        use kuna_sleigh::loadimage::LoadImage;
        let mut bytes = build_elf64(0x401000, &[0x55, 0x48, 0x89, 0xe5], None);
        bytes[104..112].copy_from_slice(&64u64.to_le_bytes()); // p_memsz: 4 backed + 60 zero
        bytes[68..72].copy_from_slice(&(object::elf::PF_R | object::elf::PF_W).to_le_bytes());
        let m = manager();
        let ram = Rc::clone(m.get_space_by_name("ram").unwrap());
        let mut img = ObjectLoadImage::from_bytes("t.elf", &bytes).unwrap();
        img.attach_to_space(Rc::clone(&ram));

        img.kuna_overlay_bytes(&Address::new(Rc::clone(&ram), 0x401020), &[0xde, 0xad])
            .expect("inside the p_memsz tail");
        assert_eq!(
            img.load(4, &Address::new(Rc::clone(&ram), 0x40101f)).unwrap(),
            vec![0x00, 0xde, 0xad, 0x00]
        );
        // Past p_memsz is not mapped at all.
        assert!(img
            .kuna_overlay_bytes(&Address::new(Rc::clone(&ram), 0x401040), &[0x90])
            .is_err());
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

    /// The one ARM mode table both PE-family loaders read: the container
    /// family decides how machine `0x1c2` is named, and nothing else differs.
    #[test]
    fn the_arm_mode_policy_is_one_table_read_per_container_family() {
        use object::pe::{IMAGE_FILE_MACHINE_AMD64, IMAGE_FILE_MACHINE_ARMNT};
        for uefi_te in [false, true] {
            assert_eq!(
                pe_arm_mode_policy(IMAGE_FILE_MACHINE_ARMNT, uefi_te),
                Some(ArmModePolicy::WholeImageThumb)
            );
            assert_eq!(pe_arm_mode_policy(IMAGE_FILE_MACHINE_ARM, uefi_te), Some(ArmModePolicy::EntryBit));
            assert_eq!(pe_arm_mode_policy(IMAGE_FILE_MACHINE_AMD64, uefi_te), None);
        }
        assert_eq!(
            pe_arm_mode_policy(IMAGE_FILE_MACHINE_THUMB, false),
            Some(ArmModePolicy::WholeImageThumb),
            "Windows names 0x1c2 THUMB"
        );
        assert_eq!(
            pe_arm_mode_policy(IMAGE_FILE_MACHINE_THUMB, true),
            Some(ArmModePolicy::EntryBit),
            "the UEFI bindings name 0x1c2 ARMTHUMB_MIXED"
        );

        // A PE whose machine word is ARM loads as 32-bit ARM with no whole-image
        // claim, and its odd entry is the walk's seed.
        let path = format!("{}/tests/fixtures/armv4t_thumb_pe.exe", env!("CARGO_MANIFEST_DIR"));
        let mut bytes = std::fs::read(&path).expect("read synthetic ARM PE fixture");
        let pe = u32::from_le_bytes(bytes[0x3c..0x40].try_into().unwrap()) as usize;
        bytes[pe + 4..pe + 6].copy_from_slice(&IMAGE_FILE_MACHINE_ARM.to_le_bytes());
        let file = object::File::parse(bytes.as_slice()).expect("parse the ARM-machine PE");
        assert_eq!(effective_architecture(&file, &bytes), Architecture::Arm);
        assert_eq!(arm_isa_hint(&file, &bytes), None);
        assert_eq!(arm_entry_thumb_hint(&file, &bytes), Some(0x401000));
        let image = ObjectLoadImage::from_bytes("arm-machine.exe", &bytes).expect("loads without a target");
        assert_eq!(image.arch_id(), b"ARM:LE:32:v8:windows");
        assert_eq!(image.image_entry(), Some(0x401000), "the retained entry is a code address");
    }

    #[test]
    fn thumb_machine_pe_maps_with_explicit_language() {
        let path = format!(
            "{}/tests/fixtures/armv4t_thumb_pe.exe",
            env!("CARGO_MANIFEST_DIR")
        );
        let bytes = std::fs::read(&path).expect("read synthetic ARM PE fixture");
        let file = object::File::parse(bytes.as_slice()).expect("parse synthetic ARM PE");
        assert_eq!(file.architecture(), Architecture::Unknown);
        assert_eq!(effective_architecture(&file, &bytes), Architecture::Arm);
        assert_eq!(arm_isa_hint(&file, &bytes), Some(true));

        let mut image = ObjectLoadImage::from_bytes_with_target(
            &path,
            &bytes,
            "ARM:LE:32:v4t:default",
        )
        .expect("explicit language must not discard PE mappings");
        assert_eq!(image.get_arch_type(), b"ARM:LE:32:v4t:default");
        let m = manager();
        let ram = Rc::clone(m.get_space_by_name("ram").unwrap());
        image.attach_to_space(Rc::clone(&ram));
        assert_eq!(
            image.load(4, &Address::new(ram, 0x401000)).unwrap(),
            [0x07, 0x20, 0x70, 0x47]
        );
    }

    #[test]
    fn default_target_preserves_detected_language_and_fallback() {
        let elf = build_elf64(0x1000, &[0xb8, 0x07, 0x00, 0xc3], None);
        let pe = std::fs::read(format!(
            "{}/tests/fixtures/armv4t_thumb_pe.exe",
            env!("CARGO_MANIFEST_DIR")
        ))
        .unwrap();
        for bytes in [&elf, &pe] {
            let automatic = ObjectLoadImage::from_bytes("synthetic", bytes).unwrap();
            for target in ["", "default", " default ", " \t"] {
                for load in [
                    ObjectLoadImage::from_bytes_with_target,
                    ObjectLoadImage::from_bytes_silent_with_target,
                ] {
                    let image = load("synthetic", bytes, target).unwrap();
                    assert_eq!(image.arch_id(), automatic.arch_id());
                    assert_eq!(image.fallback_arch_id(), automatic.fallback_arch_id());
                    assert_eq!(image.section_snapshot(), automatic.section_snapshot());
                }
            }
        }
    }

    #[test]
    fn explicit_decoder_width_preserves_elf_container_mapping() {
        let code = [0xb8, 0x07, 0x00, 0xc3];
        let bytes = build_elf64(0x1000, &code, None);
        let automatic = ObjectLoadImage::from_bytes("synthetic", &bytes).unwrap();
        for target in ["x86:LE:16:Real Mode:default", "x86:LE:32:default:gcc"] {
            let mut image =
                ObjectLoadImage::from_bytes_with_target("synthetic", &bytes, target).unwrap();
            assert_eq!(image.arch_id(), target.as_bytes());
            assert_eq!(image.section_snapshot(), automatic.section_snapshot());
            let ram = Rc::clone(manager().get_default_code_space().unwrap());
            image.attach_to_space(Rc::clone(&ram));
            assert_eq!(
                image
                    .load(code.len() as i32, &Address::new(ram, 0x1000))
                    .unwrap(),
                code
            );
        }
    }

    #[test]
    fn explicit_target_keeps_pe_mapping_across_an_endian_conflict() {
        let path = format!(
            "{}/tests/fixtures/armv4t_thumb_pe.exe",
            env!("CARGO_MANIFEST_DIR")
        );
        let bytes = std::fs::read(&path).expect("read synthetic ARM PE fixture");
        let automatic = ObjectLoadImage::from_bytes(&path, &bytes).unwrap();
        let mut image =
            ObjectLoadImage::from_bytes_with_target(&path, &bytes, "ARM:BE:32:v4t:default")
                .expect("an endian-conflicting target is reported, not refused");
        assert_eq!(image.get_arch_type(), b"ARM:BE:32:v4t:default");
        assert_eq!(image.section_snapshot(), automatic.section_snapshot());
        let ram = Rc::clone(manager().get_default_code_space().unwrap());
        image.attach_to_space(Rc::clone(&ram));
        assert_eq!(
            image.load(4, &Address::new(ram, 0x401000)).unwrap(),
            [0x07, 0x20, 0x70, 0x47]
        );
    }

    #[test]
    fn target_endian_note_fires_only_on_a_real_conflict() {
        let path = format!(
            "{}/tests/fixtures/armv4t_thumb_pe.exe",
            env!("CARGO_MANIFEST_DIR")
        );
        let bytes = std::fs::read(&path).expect("read synthetic ARM PE fixture");
        let file = object::File::parse(bytes.as_slice()).unwrap();
        assert!(target_endian_note(&file, "ARM:BE:32:v4t:default")
            .is_some_and(|note| note.contains("BE-endian") && note.contains("LE-endian")));
        assert!(target_endian_note(&file, "ARM:LE:32:v4t:default").is_none());
        assert!(target_endian_note(&file, "ARM").is_none());
    }

    #[test]
    fn thumb_coff_typed_parser_preserves_machine_and_mapping() {
        use object::{BinaryFormat, Endianness, SectionKind};
        let mut obj = object::write::Object::new(BinaryFormat::Coff, Architecture::Arm, Endianness::Little);
        let text = obj.add_section(Vec::new(), b".text".to_vec(), SectionKind::Text);
        obj.append_section_data(text, &[0x07, 0x20, 0x70, 0x47], 4);
        let mut bytes = obj.write().unwrap();
        bytes[..2].copy_from_slice(&IMAGE_FILE_MACHINE_THUMB.to_le_bytes());
        let file = parse_object(&bytes).unwrap();
        assert_eq!(file.format(), BinaryFormat::Coff);
        assert_eq!(container_machine(&file, &bytes), Some(IMAGE_FILE_MACHINE_THUMB));
        assert_eq!(effective_architecture(&file, &bytes), Architecture::Arm);
        assert_eq!(arm_isa_hint(&file, &bytes), Some(true));
        let mut image = ObjectLoadImage::from_bytes("thumb.obj", &bytes).unwrap();
        assert!(image.get_arch_type().starts_with(b"ARM:LE:32:"));
        let start = image.section_snapshot().iter().find(|(_, _, flags)| flags & section_flags::CODE != 0).unwrap().0;
        let m = manager();
        let ram = Rc::clone(m.get_space_by_name("ram").unwrap());
        image.attach_to_space(Rc::clone(&ram));
        assert_eq!(image.load(4, &Address::new(ram, start)).unwrap(), [0x07, 0x20, 0x70, 0x47]);
    }

    #[test]
    fn thumb_coff_typed_parser_rejects_truncated_headers_and_sections() {
        assert!(parse_object(&[0xc2, 0x01]).is_err());
        let mut header = [0u8; 20];
        header[..2].copy_from_slice(&IMAGE_FILE_MACHINE_THUMB.to_le_bytes());
        header[2..4].copy_from_slice(&1u16.to_le_bytes());
        assert!(parse_object(&header).is_err());
        assert!(parse_object(b"unrecognized object").is_err());
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
    /// entry declared inside it has readable bytes. This fixture declares one,
    /// so the region is read-only CODE — the image itself named those bytes
    /// (`whole-binary-decompilation-treats`). The header page of an image that
    /// declares no such entry stays DATA, which is
    /// `the_header_page_never_overlaps_a_section` below.
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

        let header = (0x40_0000u64, 0x200u64, section_flags::CODE | section_flags::READONLY);
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

    /// PE import slots retain their loader identity independently of section
    /// permissions. The side table, not CODE versus DATA, is what body
    /// selection can rely on while keeping the slot's function symbol.
    #[test]
    fn pe_import_slots_are_reported_in_code_and_data_sections() {
        for (fixture, slot, name, executable) in [
            ("pe_imports.exe", 0x1_4000_d1ecu64, "GetLastError", false),
            ("pe_iatincode_i386.exe", 0x40_1000u64, "VirtualAlloc", true),
        ] {
            let path = format!("{}/tests/fixtures/{fixture}", env!("CARGO_MANIFEST_DIR"));
            let bytes = std::fs::read(&path).expect("fixture");
            let img = ObjectLoadImage::from_bytes(&path, &bytes).expect("load the PE");
            assert!(
                img.import_slot_ranges().iter().any(|&(lo, hi)| slot >= lo && slot < hi),
                "{fixture}: missing import slot at {slot:#x}"
            );
            assert!(
                img.func_symbols().iter().any(|(addr, found)| *addr == slot && found == name),
                "{fixture}: the slot FunctionSymbol must remain installed"
            );
            let section_is_code = img
                .section_snapshot()
                .iter()
                .find(|&&(start, size, _)| slot >= start && slot - start < size)
                .is_some_and(|&(_, _, flags)| flags & section_flags::CODE != 0);
            assert_eq!(section_is_code, executable, "{fixture}: wrong section control");
        }
    }

    /// (kuna, `pe-zero-filled-data`) A PE section that declares more RAM than
    /// the file backs maps its whole `VirtualSize`: the file extent reads back
    /// its own bytes, the tail past `SizeOfRawData` reads back zeroes, and the
    /// first byte past `VirtualSize` is still unmapped.
    #[test]
    fn a_pe_maps_the_zero_filled_tail_of_a_section() {
        use kuna_sleigh::loadimage::LoadImage;
        let path = format!(
            "{}/tests/fixtures/pe_bsstail_x86_64.exe",
            env!("CARGO_MANIFEST_DIR")
        );
        let bytes = std::fs::read(&path).expect("fixture");
        let m = manager();
        let ram = Rc::clone(m.get_space_by_name("ram").unwrap());
        let mut img = ObjectLoadImage::from_bytes(&path, &bytes).expect("load the PE");
        img.attach_to_space(Rc::clone(&ram));

        // .data: RVA 0x2000, VirtualSize 0x7a8, SizeOfRawData 0x200. The bound
        // is probed first, on a cold image: `loadFill` reads a 512-byte window
        // at a time and zero-pads the end of one, so a later read inside a
        // window that started in .data is answered from the buffer.
        let at = |vma: u64| Address::new(Rc::clone(&ram), vma);
        let mut past = [0u8; 1];
        let err = img.load_fill(&mut past, &at(0x1_4000_27a8)).unwrap_err();
        assert!(
            matches!(err, KunaError::DataUnavail { .. }),
            "VirtualSize is still the bound; got {err:?}"
        );

        let mut buf = [0u8; 8];
        img.load_fill(&mut buf, &at(0x1_4000_21f8)).expect("the file extent is mapped");
        assert_eq!(buf, [0xaa; 8], "the last eight bytes SizeOfRawData backs");

        let mut tail = [0xffu8; 64];
        img.load_fill(&mut tail, &at(0x1_4000_2740)).expect("the virtual tail is mapped");
        assert_eq!(tail, [0u8; 64], "past SizeOfRawData the tail reads as zeroes");
    }

    /// The same for an ELF `.bss`: `p_memsz` past `p_filesz` is mapped memory,
    /// not a hole, so a read there answers zeroes instead of DataUnavail — and
    /// the tail of an *executable* segment stays a hole, because there its
    /// file-time zeroes are not what runs.
    #[test]
    fn an_elf_bss_tail_reads_as_zeroes() {
        use kuna_sleigh::loadimage::LoadImage;
        let mut bytes = build_elf64(0x401000, &[0x55, 0x48, 0x89, 0xe5], None);
        bytes[104..112].copy_from_slice(&64u64.to_le_bytes()); // p_memsz, 4 backed + 60 zero
        bytes[68..72].copy_from_slice(&(object::elf::PF_R | object::elf::PF_W).to_le_bytes());
        let m = manager();
        let ram = Rc::clone(m.get_space_by_name("ram").unwrap());
        let mut img = ObjectLoadImage::from_bytes("t.elf", &bytes).expect("load the ELF");
        img.attach_to_space(Rc::clone(&ram));

        // Cold, so the 512-byte read window cannot answer it (see the PE case).
        let err = img.load(4, &Address::new(Rc::clone(&ram), 0x401040)).unwrap_err();
        assert!(matches!(err, KunaError::DataUnavail { .. }), "p_memsz bounds it: {err:?}");
        let got = img.load(8, &Address::new(Rc::clone(&ram), 0x401000)).expect("mapped");
        assert_eq!(got, vec![0x55, 0x48, 0x89, 0xe5, 0, 0, 0, 0]);
        let got = img.load(4, &Address::new(Rc::clone(&ram), 0x401030)).expect("bss tail");
        assert_eq!(got, vec![0, 0, 0, 0], "wholly inside the .bss tail");

        bytes[68..72].copy_from_slice(&(object::elf::PF_R | object::elf::PF_X).to_le_bytes());
        let mut img = ObjectLoadImage::from_bytes("t.elf", &bytes).expect("load the ELF");
        img.attach_to_space(Rc::clone(&ram));
        let err = img.load(4, &Address::new(Rc::clone(&ram), 0x401030)).unwrap_err();
        assert!(
            matches!(err, KunaError::DataUnavail { .. }),
            "an executable segment's tail is not materialised as code; got {err:?}"
        );
    }

    /// (kuna) A read longer than the 512-byte staging buffer is served straight
    /// into the caller's slice.  Upstream copies its answer back with
    /// `memcpy(ptr,buffer,size)` and over-reads the buffer for any such
    /// request; the same copy spelled as a Rust slice panicked ("range end
    /// index 667 out of range for slice of length 512"), which took a whole
    /// `decompile-project` export down at the first global whose declared
    /// datatype was bigger than the buffer.
    #[test]
    fn a_read_larger_than_the_staging_buffer_is_served_whole() {
        use kuna_sleigh::loadimage::LoadImage;
        let payload: Vec<u8> = (0..1024u32).map(|i| (i % 251) as u8).collect();
        let bytes = build_elf64(0x401000, &payload, None);
        let m = manager();
        let ram = Rc::clone(m.get_space_by_name("ram").unwrap());
        let mut img = ObjectLoadImage::from_bytes("t.elf", &bytes).expect("load the ELF");
        img.attach_to_space(Rc::clone(&ram));
        let at = |o: u64| Address::new(Rc::clone(&ram), o);

        // The size out of the crash report, then the buffer seam either side of
        // it, then spans that start and end off the buffer's own boundaries.
        for (off, len) in [(0usize, 667usize), (0, 512), (0, 513), (3, 1021), (500, 524)] {
            let got = img.load(len as i32, &at(0x401000 + off as u64)).expect("mapped");
            assert_eq!(got, payload[off..off + len], "{len} bytes at +{off}");
        }

        // A big read is not cached, so it must leave the window a neighbouring
        // small read is being served from exactly as it found it.
        let warm = img.load(8, &at(0x401000)).expect("mapped");
        img.load(667, &at(0x401100)).expect("mapped");
        assert_eq!(img.load(8, &at(0x401000)).expect("mapped"), warm, "window poisoned");
    }

    /// A big read gets the same answers at the edges of mapped memory that the
    /// buffered path gives: run off the end and the tail reads as zeroes, start
    /// where nothing is mapped and it is `DataUnavail` — not a panic.
    #[test]
    fn a_big_read_off_the_end_zero_fills_and_an_unmapped_one_raises() {
        use kuna_sleigh::loadimage::LoadImage;
        let payload: Vec<u8> = (0..600u32).map(|i| (i % 251) as u8).collect();
        let bytes = build_elf64(0x401000, &payload, None);
        let m = manager();
        let ram = Rc::clone(m.get_space_by_name("ram").unwrap());
        let mut img = ObjectLoadImage::from_bytes("t.elf", &bytes).expect("load the ELF");
        img.attach_to_space(Rc::clone(&ram));
        let at = |o: u64| Address::new(Rc::clone(&ram), o);

        let got = img.load(700, &at(0x401000)).expect("the first byte is mapped");
        assert_eq!(got[..600], payload[..], "the mapped bytes come back verbatim");
        assert!(got[600..].iter().all(|&b| b == 0), "the unmapped tail reads as zeroes");

        let err = img.load(667, &at(0x900000)).unwrap_err();
        assert!(matches!(err, KunaError::DataUnavail { .. }), "unmapped start: {err:?}");
        let err = img.load(667, &at(0x401000 - 8)).unwrap_err();
        assert!(matches!(err, KunaError::DataUnavail { .. }), "starts below the map: {err:?}");
    }

    /// A zero-filled tail is trimmed at the next segment, and only the tail is:
    /// segments stacked at one vma (a COFF object read through the linked path)
    /// keep every byte the file backs.
    #[test]
    fn a_virtual_tail_stops_at_the_next_segment() {
        let mut segs = vec![
            Segment { vma: 0x1000, data: vec![1, 2, 3, 4], memsz: 0x4000 },
            Segment { vma: 0x2000, data: vec![9], memsz: 0x10 },
        ];
        clamp_virtual_tails(&mut segs);
        assert_eq!(segs[0].mapped_size(), 0x1000, "trimmed to where the next segment starts");
        assert_eq!(segs[1].mapped_size(), 0x10, "the last segment keeps its whole tail");

        let mut stacked = vec![
            Segment::file_backed(0, vec![1, 2, 3, 4]),
            Segment::file_backed(0, vec![5, 6]),
        ];
        clamp_virtual_tails(&mut stacked);
        assert_eq!(stacked[0].mapped_size(), 4, "a file extent is never trimmed away");
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
