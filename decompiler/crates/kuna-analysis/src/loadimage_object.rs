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
//!   ELF identifies); other ABIs are a seam.
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
//! (PE/Mach-O) are a seam — this loader is ELF-only, matching the W11 task.

use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

use object::read::{Object, ObjectSection, ObjectSegment, ObjectSymbol};
use object::{Architecture, SectionFlags, SectionKind, SymbolKind};

use kuna_base::address::{Address, RangeList};
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::space::AddrSpace;
use kuna_base::types::Wrap;

use kuna_sleigh::loadimage::{section_flags, LoadImage, LoadImageFunc, LoadImageSection};

/// Default read-buffer size (C++ `LoadImageBfd::bufsize`, `loadimage_bfd.cc:36`).
const BUFSIZE: usize = 512;

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
    /// Loadable regions, in *ascending vma order* (the BFD section list, used by
    /// `find_section`/`loadFill`).
    segments: Vec<Segment>,
    /// ELF sections, in file order (the BFD `asection` list for the info walks).
    sections: Vec<SectionInfo>,
    /// Function symbols, in symbol-table order.
    funcsyms: Vec<FuncSym>,
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
        // bfd_openr(filename): read the image file.
        let bytes = std::fs::read(filename).map_err(|e| {
            KunaError::lowlevel(format!("Unable to open image file: {filename}: {e}"))
        })?;
        Self::from_bytes(filename, &bytes)
    }

    /// Open from an in-memory image (the testable core of [`Self::open`]).
    pub fn from_bytes(filename: &str, bytes: &[u8]) -> KunaResult<ObjectLoadImage> {
        // bfd_check_format(thebfd, bfd_object): parse the object file.
        let file = object::File::parse(bytes).map_err(|e| {
            KunaError::lowlevel(format!(
                "File: {filename} : not in recognized object file format: {e}"
            ))
        })?;
        // ELF-only (the BFD loader handles every BFD target; kuna's substitution
        // is scoped to ELF — the only format the W11 task requires).
        if !matches!(file.format(), object::BinaryFormat::Elf) {
            return Err(KunaError::lowlevel(format!(
                "File: {filename} : not an ELF object (kuna ELF loader is ELF-only)"
            )));
        }

        let archtype = language_id_for(&file, filename)?;

        // Snapshot the loadable segments (PT_LOAD), copying their RAM bytes.
        // `data()` returns only the file-backed bytes; a segment's `size()`
        // (its RAM footprint) may exceed that for `.bss`-style tails, which the
        // BFD loader reports as zeroes via the gap fill — so the copied `data`
        // is the file extent and any RAM tail past it falls into the zero-fill
        // path exactly as an unmapped gap would (BFD `SEC_LOAD`-less sections).
        let mut segments: Vec<Segment> = Vec::new();
        for seg in file.segments() {
            let vma = seg.address();
            let data = seg.data().map_err(|e| {
                KunaError::lowlevel(format!("File: {filename} : unreadable segment data: {e}"))
            })?;
            if data.is_empty() {
                continue;
            }
            segments.push(Segment { vma, data: data.to_vec() });
        }
        // Ascending vma order so find_section's "closest greater" walk is a
        // simple scan (the BFD list is already address-ordered for ELF).
        segments.sort_by_key(|s| s.vma);

        // Snapshot the sections for the info walks (the BFD `asection` list).
        let mut sections: Vec<SectionInfo> = Vec::new();
        for sec in file.sections() {
            let flags = section_kind_flags(sec.kind(), sec.flags());
            sections.push(SectionInfo { vma: sec.address(), size: sec.size(), flags });
        }

        // Snapshot the function symbols.  Three sources, deduped by address so an
        // import that appears in several tables is registered exactly once:
        //   1. `.symtab` defined functions (BFD BSF_FUNCTION with a name),
        //   2. PLT stubs → imported library names (the kuna analog of Ghidra's
        //      `ElfDefaultGotPltMarkup`; see [`crate::s1_loader::elf_plt`]),
        //   3. `.dynsym` defined functions, for stripped-but-dynamic binaries
        //      whose `.symtab` is gone.
        let mut funcsyms: Vec<FuncSym> = Vec::new();
        let mut seen: HashSet<u64> = HashSet::new();

        // 1. `.symtab` defined functions.  Skip UND import entries (`st_value == 0`,
        //    e.g. `puts@@GLIBC_2.2.5`), which are not real code addresses, and
        //    strip any `@VERSION` suffix.
        for sym in file.symbols() {
            if sym.kind() != SymbolKind::Text {
                continue;
            }
            let addr = sym.address();
            if addr == 0 {
                continue; // UND / absolute import stub, not a code address
            }
            let name = match sym.name_bytes() {
                Ok(n) if !n.is_empty() => crate::s1_loader::elf_plt::strip_version(n),
                _ => continue, // a->name != (const char *)0
            };
            if name.is_empty() {
                continue;
            }
            if seen.insert(addr) {
                funcsyms.push(FuncSym { addr, name });
            }
        }

        // 2. PLT stubs → imported library names.
        for p in crate::s1_loader::elf_plt::resolve_plt_imports(&file) {
            if seen.insert(p.addr) {
                funcsyms.push(FuncSym { addr: p.addr, name: p.name });
            }
        }

        // 3. `.dynsym` defined functions (stripped-but-dynamic fallback): a
        //    dynamic binary stripped of `.symtab` still exports its defined
        //    functions in `.dynsym`.
        for sym in file.dynamic_symbols() {
            if sym.kind() != SymbolKind::Text {
                continue;
            }
            let addr = sym.address();
            if addr == 0 {
                continue;
            }
            let name = match sym.name_bytes() {
                Ok(n) if !n.is_empty() => crate::s1_loader::elf_plt::strip_version(n),
                _ => continue,
            };
            if name.is_empty() {
                continue;
            }
            if seen.insert(addr) {
                funcsyms.push(FuncSym { addr, name });
            }
        }

        Ok(ObjectLoadImage {
            filename: filename.to_string(),
            archtype,
            segments,
            sections,
            funcsyms,
            spaceid: None,
            buffer: RefCell::new(vec![0u8; BUFSIZE]),
            bufoffset: RefCell::new(!0u64), // ~((uintb)0)
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
            // (offset>=start)&&(offset<stop)  — C++ uses raw `<`/`>=`; a
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
        // if (addr.getSpace() != spaceid) throw DataUnavailError(...)
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

        // if ((curaddr>=bufoffset)&&(curaddr+size<bufoffset+bufsize)) { ... }
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
                // p == 0
                if offset == 0 {
                    break; // Initial address not mapped
                }
                // memset(buffer+offset,0,cursize): fill the rest with zero.
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
                // memset(buffer+offset,0,readsize): zeroes to the next section.
                for b in &mut buffer[offset..offset + rs as usize] {
                    *b = 0;
                }
                readsize = rs;
            } else {
                let mut rs = cursize as u64;
                // if (curaddr+readsize>p->vma+secsize) readsize = p->vma+secsize-curaddr;
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
            let mut errmsg =
                format!("Unable to load {} bytes at {}", cursize, addr.get_shortcut());
            addr.print_raw(&mut errmsg)?;
            return Err(KunaError::data_unavail(errmsg));
        }
        // memcpy(ptr,buffer,size): copy the requested bytes out.
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
        // record.address = Address(spaceid,val)
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

    fn get_readonly(&self, list: &mut RangeList) {
        // List all ranges that are read only (C++ `LoadImageBfd::getReadonly`).
        let Some(space) = self.spaceid.as_ref() else {
            return;
        };
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
        // adjust = AddrSpace::addressToByte(adjust,wordsize) (C++
        // `LoadImageBfd::adjustVma`, `loadimage_bfd.cc:67`).
        let spaceid = self
            .spaceid
            .as_ref()
            .expect("ObjectLoadImage::adjustVma before attachToSpace (C++ null space deref)");
        let badjust = AddrSpace::address_to_byte(adjust as u64, spaceid.get_word_size());
        for s in &mut self.segments {
            s.vma = s.vma.wadd(badjust); // s->vma += adjust
        }
        for s in &mut self.sections {
            s.vma = s.vma.wadd(badjust);
        }
        for s in &mut self.funcsyms {
            s.addr = s.addr.wadd(badjust);
        }
        // A shifted segment set may no longer be vma-sorted only if `badjust`
        // wraps a subset past the top of the space; for a real ELF every vma
        // shifts by the same amount so the order is preserved (matching BFD,
        // which never re-sorts).
    }
}

/// Translate an [`object`] section kind + flags into the kuna
/// [`section_flags`] bitset, mirroring the BFD `SEC_*` -> `LoadImageSection`
/// translation in `LoadImageBfd::getNextSection` (`loadimage_bfd.cc:261`).
fn section_kind_flags(kind: SectionKind, flags: SectionFlags) -> u32 {
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

/// Resolve the SLEIGH **language id** for an ELF object (the `getArchType`
/// payload).  This is the kuna substitution for the Ghidra Java-side
/// BFD-name -> language map: it reads the ELF machine + endianness + class
/// directly and returns the id `SleighArchitecture::resolveArchitecture`
/// consumes (e.g. `x86:LE:64:default:gcc`).
///
/// PARTIAL: covers the common Linux/SysV machines kuna ships a `.sla` for.  An
/// unmapped machine is a `LowlevelError` naming it (the caller falls back to an
/// explicit `--target` language id).
fn language_id_for(file: &object::File, filename: &str) -> KunaResult<Vec<u8>> {
    let little = file.is_little_endian();
    let endian = if little { "LE" } else { "BE" };
    // The Linux/SysV ELF ABI default compiler model is gcc; this is the only
    // ABI a bare ELF identifies (Windows PE / golang are seams).
    let id: String = match file.architecture() {
        Architecture::X86_64 => "x86:LE:64:default:gcc".to_string(),
        Architecture::I386 => "x86:LE:32:default:gcc".to_string(),
        Architecture::Aarch64 => format!("AARCH64:{endian}:64:v8A:default"),
        Architecture::Arm => format!("ARM:{endian}:32:v8:default"),
        Architecture::Mips => format!("MIPS:{endian}:32:default:default"),
        Architecture::PowerPc => format!("PowerPC:{endian}:32:default:default"),
        Architecture::PowerPc64 => format!("PowerPC:{endian}:64:default:default"),
        Architecture::Riscv64 => format!("RISCV:{endian}:64:RV64GC:gcc"),
        Architecture::Riscv32 => format!("RISCV:{endian}:32:RV32GC:gcc"),
        Architecture::Sparc | Architecture::Sparc32Plus => {
            format!("sparc:{endian}:32:default:default")
        }
        Architecture::Sparc64 => format!("sparc:{endian}:64:default:default"),
        other => {
            return Err(KunaError::lowlevel(format!(
                "File: {filename} : unsupported ELF machine {other:?} \
                 (no kuna SLEIGH language; pass an explicit --target language id)"
            )));
        }
    };
    Ok(id.into_bytes())
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
}
