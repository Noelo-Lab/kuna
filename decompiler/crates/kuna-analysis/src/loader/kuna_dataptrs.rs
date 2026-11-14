//! (kuna) The pointers an image stores in its data, as its own relocation
//! metadata states them.
//!
//! A word in `.data` that holds `&audit_target` is a reference no instruction
//! makes: nothing decodes it, so the instruction walk behind `kuna xrefs` never
//! sees it. An image that is rebased at load time lists every such word, because
//! the loader has to fix each one up:
//!
//! - a Mach-O with `LC_DYLD_CHAINED_FIXUPS` threads them on its fixup chains,
//!   each rebase carrying its target ([`crate::loader::format::macho::chained`]);
//! - a classic Mach-O lists them in its `LC_DYLD_INFO` rebase opcodes, the slot
//!   already holding the unslid target;
//! - a position-independent ELF carries a dynamic relocation for each: a
//!   `RELATIVE` one (or an `SHT_RELR` bit) for a local target, an absolute word
//!   or `GLOB_DAT` / `JUMP_SLOT` one for a symbol this image defines and
//!   exports, on x86, ARM, AArch64, RISC-V and PowerPC;
//! - a PE lists them in its `.reloc` base relocations, the slot holding the
//!   preferred-base target.
//!
//! [`data_pointers`] reads that list and keeps each word whose target lies in an
//! allocated section of the image. [`Coverage`] says how much of it the list
//! is: all of one [`Source`], part of one (a table that could not be read), or
//! nothing, either because the image keeps no such metadata
//! ([`Coverage::None`]: a non-PIE ELF or a PE without base relocations stores
//! its pointers as plain words, which no metadata tells apart from integers) or
//! because the metadata it keeps is a kind kuna does not read
//! ([`Coverage::Unsupported`]: a relocatable object's, a MIPS image's). Only a
//! complete list is an answer when it is empty. Imports are not here: a bound
//! slot names a symbol of another image, not an address of this one.

use object::pe::{IMAGE_REL_BASED_DIR64, IMAGE_REL_BASED_HIGHLOW};
use object::read::macho::{LoadCommandVariant, MachHeader, Segment as MachSegment};
use object::read::pe::{ImageNtHeaders, PeFile};
use object::read::{Object, ObjectSection, ObjectSegment, ObjectSymbol};
use object::{BinaryFormat, FileKind, ObjectKind, RelocationFlags, RelocationTarget};

/// The relocation metadata a pointer list is read from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Source {
    /// `LC_DYLD_CHAINED_FIXUPS` rebases.
    ChainedFixups,
    /// `LC_DYLD_INFO` rebase opcodes.
    RebaseOpcodes,
    /// ELF dynamic relocations, `SHT_RELR` included.
    DynamicRelocations,
    /// PE base relocations.
    BaseRelocations,
}

impl Source {
    /// The name the CLI reports.
    pub fn as_str(self) -> &'static str {
        match self {
            Source::ChainedFixups => "chained-fixups",
            Source::RebaseOpcodes => "rebase-opcodes",
            Source::DynamicRelocations => "dynamic-relocations",
            Source::BaseRelocations => "base-relocations",
        }
    }
}

/// How much of an image's pointer-relocation metadata a list covers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Coverage {
    /// Every pointer the source lists.
    Complete(Source),
    /// The part of the source that could be read; the rest could not.
    Partial(Source),
    /// The image keeps no pointer-relocation metadata, so its data pointers
    /// are not enumerable.
    None,
    /// The image keeps pointer-relocation metadata of a kind kuna does not
    /// read.
    Unsupported,
}

impl Coverage {
    /// The name the CLI reports: the source's for a complete list.
    pub fn as_str(self) -> &'static str {
        match self {
            Coverage::Complete(source) => source.as_str(),
            Coverage::Partial(_) => "partial",
            Coverage::None => "none",
            Coverage::Unsupported => "unsupported",
        }
    }

    /// The metadata the listed pointers came from.
    pub fn source(self) -> Option<Source> {
        match self {
            Coverage::Complete(source) | Coverage::Partial(source) => Some(source),
            Coverage::None | Coverage::Unsupported => None,
        }
    }
}

/// One pointer word stored in the image's data.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DataPointer {
    /// Address of the word.
    pub slot: u64,
    /// The address it holds.
    pub target: u64,
    /// The word's width in bytes.
    pub width: u8,
}

/// Every data pointer an image's relocation metadata lists, by slot. `reason`
/// says why the list is not complete, for every coverage but
/// [`Coverage::Complete`].
#[derive(Debug, Clone)]
pub struct DataPointers {
    pub coverage: Coverage,
    pub reason: Option<String>,
    pub pointers: Vec<DataPointer>,
}

impl DataPointers {
    fn listed(source: Source, pointers: Vec<DataPointer>, unread: Vec<String>) -> DataPointers {
        if unread.is_empty() {
            return DataPointers { coverage: Coverage::Complete(source), reason: None, pointers };
        }
        DataPointers { coverage: Coverage::Partial(source), reason: Some(unread.join("; ")), pointers }
    }

    fn empty(coverage: Coverage, reason: impl Into<String>) -> DataPointers {
        DataPointers { coverage, reason: Some(reason.into()), pointers: Vec::new() }
    }
}

/// The data pointers `file` lists (`bytes` is its raw image), sorted by slot,
/// each target inside an allocated section, and how much of its metadata they
/// cover.
pub fn data_pointers(file: &object::File, bytes: &[u8]) -> DataPointers {
    let mut listed = match (file.format(), file.kind()) {
        (_, ObjectKind::Relocatable) => relocatable(file),
        (BinaryFormat::MachO, _) => macho(file, bytes),
        (BinaryFormat::Elf, ObjectKind::Dynamic) => elf(file),
        (BinaryFormat::Elf, ObjectKind::Executable) => elf_executable(file),
        (BinaryFormat::Elf, _) => DataPointers::empty(
            Coverage::Unsupported,
            "an ELF image that is neither an executable nor a shared object",
        ),
        (BinaryFormat::Pe, _) => match file {
            object::File::Pe32(pe) => pe_relocated(file, pe),
            object::File::Pe64(pe) => pe_relocated(file, pe),
            _ => DataPointers::empty(Coverage::Unsupported, "a PE image kuna cannot read as PE"),
        },
        (format, _) => {
            DataPointers::empty(Coverage::Unsupported, format!("{format:?} pointer relocations are not read"))
        }
    };
    if listed.coverage.source().is_none() {
        return listed;
    }
    let mapped: Vec<(u64, u64)> = file
        .sections()
        .filter(|sec| sec.size() != 0 && allocated(sec.flags()))
        .map(|sec| (sec.address(), sec.address().saturating_add(sec.size())))
        .collect();
    let pointers = &mut listed.pointers;
    pointers.retain(|p| mapped.iter().any(|&(lo, hi)| p.target >= lo && p.target < hi));
    pointers.sort_unstable_by_key(|p| p.slot);
    pointers.dedup_by_key(|p| p.slot);
    listed
}

/// A relocatable object's relocations name sections, not load addresses, and
/// are not read here.
fn relocatable(file: &object::File) -> DataPointers {
    if file.sections().any(|sec| sec.relocations().next().is_some()) {
        DataPointers::empty(Coverage::Unsupported, "a relocatable object's relocations are not read")
    } else {
        DataPointers::empty(Coverage::None, "a relocatable object with no relocations")
    }
}

/// A non-PIE ELF executable loads at its link addresses, so no relocation
/// lists its pointers, unless the link kept its static relocations
/// (`--emit-relocs`, as a BOLT-ready binary or `vmlinux` does), which are not
/// read.
fn elf_executable(file: &object::File) -> DataPointers {
    let kept = match file {
        object::File::Elf32(elf) => kept_static_relocations(elf),
        object::File::Elf64(elf) => kept_static_relocations(elf),
        _ => false,
    };
    if kept {
        return DataPointers::empty(
            Coverage::Unsupported,
            "an ELF executable's static relocation sections (--emit-relocs) are not read",
        );
    }
    DataPointers::empty(
        Coverage::None,
        "an ELF executable (ET_EXEC) loads at its link addresses, so no relocation lists its pointers",
    )
}

/// Does an unallocated `SHT_REL` / `SHT_RELA` section relocate an allocated
/// section?
fn kept_static_relocations<'data, Elf, R>(elf: &object::read::elf::ElfFile<'data, Elf, R>) -> bool
where
    Elf: object::read::elf::FileHeader,
    R: object::ReadRef<'data>,
{
    use object::read::elf::SectionHeader;
    let endian = elf.endian();
    let sections = elf.elf_section_table();
    let alloc = |flags: u64| flags & u64::from(object::elf::SHF_ALLOC) != 0;
    sections.iter().any(|sec| {
        matches!(sec.sh_type(endian), object::elf::SHT_REL | object::elf::SHT_RELA)
            && !alloc(sec.sh_flags(endian).into())
            && sections
                .section(object::SectionIndex(sec.sh_info(endian) as usize))
                .is_ok_and(|target| sec.sh_info(endian) != 0 && alloc(target.sh_flags(endian).into()))
    })
}

/// Does a section with these flags occupy addresses in the loaded image?
fn allocated(flags: object::SectionFlags) -> bool {
    const SHF_ALLOC: u64 = 0x2;
    match flags {
        object::SectionFlags::Elf { sh_flags } => sh_flags & SHF_ALLOC != 0,
        _ => true,
    }
}

/// A Mach-O image's chained-fixup rebases, else its classic rebase opcodes.
/// A bind-only chained image lists none, completely, and so does a slid image
/// whose `LC_DYLD_INFO` rebases nothing. A chain walk that stopped short
/// ([`ChainedFixups::damage`]) leaves the list partial.
///
/// [`ChainedFixups::damage`]: crate::loader::format::macho::chained::ChainedFixups::damage
fn macho(file: &object::File, bytes: &[u8]) -> DataPointers {
    let fixups = crate::loader::format::macho::resolve_chained_fixups(file, bytes);
    if fixups.is_chained() {
        let width = fixups.slot_width();
        let pointers = fixups.rebases().map(|(slot, target)| DataPointer { slot, target, width }).collect();
        let unread = fixups.damage().map(|why| format!("the chained fixups were not all read: {why}"));
        return DataPointers::listed(Source::ChainedFixups, pointers, unread.into_iter().collect());
    }
    let commands = match FileKind::parse(bytes) {
        Ok(FileKind::MachO64) => macho_commands::<object::macho::MachHeader64<object::Endianness>>(bytes),
        Ok(FileKind::MachO32) => macho_commands::<object::macho::MachHeader32<object::Endianness>>(bytes),
        _ => None,
    };
    let Some(commands) = commands else {
        return DataPointers::empty(Coverage::Unsupported, "a Mach-O image whose load commands cannot be read");
    };
    if commands.chained {
        let why = "the LC_DYLD_CHAINED_FIXUPS data lies outside the file or its segments".to_string();
        return DataPointers::listed(Source::ChainedFixups, Vec::new(), vec![why]);
    }
    if let Some((off, size)) = commands.rebase {
        let Some(ops) = off.checked_add(size).and_then(|end| bytes.get(off..end)) else {
            let why = format!("the rebase opcodes (file offset 0x{off:x}, {size} bytes) lie outside the file");
            return DataPointers::listed(Source::RebaseOpcodes, Vec::new(), vec![why]);
        };
        let width: u8 = if file.is_64() { 8 } else { 4 };
        let (slots, damage) = interpret_rebase(ops, &commands.segments, u64::from(width));
        let mut unread: Vec<String> = damage.into_iter().collect();
        let mut missing = 0usize;
        let pointers = slots
            .into_iter()
            .filter_map(|slot| {
                let target = read_word(file, slot, width as usize);
                missing += usize::from(target.is_none());
                Some(DataPointer { slot, target: target?, width })
            })
            .collect();
        if missing != 0 {
            unread.push(format!("{missing} rebased slots have no file data"));
        }
        return DataPointers::listed(Source::RebaseOpcodes, pointers, unread);
    }
    if commands.local_relocations != 0 {
        return DataPointers::empty(Coverage::Unsupported, "a Mach-O image's local relocations (LC_DYSYMTAB) are not read");
    }
    if commands.dyld_info && commands.slid {
        return DataPointers::listed(Source::RebaseOpcodes, Vec::new(), Vec::new());
    }
    DataPointers::empty(Coverage::None, "a Mach-O image with no rebase information")
}

/// What a Mach-O image's header and load commands say about its rebase
/// metadata.
struct MachoCommands {
    /// dyld slides the image: a PIE executable, a dylib, a bundle.
    slid: bool,
    /// The image carries `LC_DYLD_INFO`, its rebase opcodes empty or not.
    dyld_info: bool,
    /// Every segment's `vmaddr`, in load-command order.
    segments: Vec<u64>,
    /// The `LC_DYLD_INFO` rebase opcodes' file range, when it is not empty.
    rebase: Option<(usize, usize)>,
    /// The image carries `LC_DYLD_CHAINED_FIXUPS`.
    chained: bool,
    /// `LC_DYSYMTAB`'s `nlocrel`.
    local_relocations: u32,
}

fn macho_commands<Mach>(bytes: &[u8]) -> Option<MachoCommands>
where
    Mach: MachHeader<Endian = object::Endianness>,
{
    let header = Mach::parse(bytes, 0).ok()?;
    let endian = header.endian().ok()?;
    let mut commands = header.load_commands(endian, bytes, 0).ok()?;
    let slid = header.flags(endian) & object::macho::MH_PIE != 0
        || matches!(header.filetype(endian), object::macho::MH_DYLIB | object::macho::MH_BUNDLE);
    let mut out = MachoCommands {
        slid,
        dyld_info: false,
        segments: Vec::new(),
        rebase: None,
        chained: false,
        local_relocations: 0,
    };
    while let Ok(Some(command)) = commands.next() {
        if let Ok(Some((segment, _))) = Mach::Segment::from_command(command) {
            out.segments.push(segment.vmaddr(endian).into());
            continue;
        }
        if command.cmd() == object::macho::LC_DYLD_CHAINED_FIXUPS {
            out.chained = true;
            continue;
        }
        match command.variant() {
            Ok(LoadCommandVariant::DyldInfo(info)) => {
                out.dyld_info = true;
                let off = info.rebase_off.get(endian) as usize;
                let size = info.rebase_size.get(endian) as usize;
                if size != 0 {
                    out.rebase = Some((off, size));
                }
            }
            Ok(LoadCommandVariant::Dysymtab(dysymtab)) => out.local_relocations = dysymtab.nlocrel.get(endian),
            _ => {}
        }
    }
    Some(out)
}

/// The rebase opcode machine of `mach-o/loader.h` (`REBASE_OPCODE_*`), yielding
/// `segment.vmaddr + offset` for each rebased slot, and why the stream was not
/// read to its end when it was not. Bounded: a malformed stream stops, never
/// loops.
fn interpret_rebase(ops: &[u8], segments: &[u64], ptr: u64) -> (Vec<u64>, Option<String>) {
    const DONE: u8 = 0x00;
    const SET_TYPE_IMM: u8 = 0x10;
    const SET_SEGMENT_AND_OFFSET_ULEB: u8 = 0x20;
    const ADD_ADDR_ULEB: u8 = 0x30;
    const ADD_ADDR_IMM_SCALED: u8 = 0x40;
    const DO_REBASE_IMM_TIMES: u8 = 0x50;
    const DO_REBASE_ULEB_TIMES: u8 = 0x60;
    const DO_REBASE_ADD_ADDR_ULEB: u8 = 0x70;
    const DO_REBASE_ULEB_TIMES_SKIPPING_ULEB: u8 = 0x80;
    const LIMIT: usize = 1 << 22;

    let mut out = Vec::new();
    let mut at = 0usize;
    let mut base: Option<u64> = None;
    let mut segment = 0u8;
    let mut offset: u64 = 0;
    let mut damage: Option<String> = None;
    let uleb = |at: &mut usize| -> Option<u64> {
        let mut value = 0u64;
        let mut shift = 0u32;
        loop {
            let byte = *ops.get(*at)?;
            *at += 1;
            if shift < 64 {
                value |= u64::from(byte & 0x7f) << shift;
            }
            shift += 7;
            if byte & 0x80 == 0 {
                return Some(value);
            }
        }
    };
    while let Some(&byte) = ops.get(at) {
        let opcode_at = at;
        at += 1;
        let imm = byte & 0x0f;
        let mut emit = |count: u64, step: u64, offset: &mut u64, out: &mut Vec<u64>| {
            if count != 0 && base.is_none() {
                damage.get_or_insert_with(|| format!("the rebase opcodes name segment {segment}, which the image does not have"));
            }
            for _ in 0..count.min(LIMIT as u64) {
                if let Some(seg) = base {
                    out.push(seg.wrapping_add(*offset));
                }
                *offset = offset.wrapping_add(step);
            }
        };
        let truncated = format!("the rebase opcodes end inside the operand of the opcode at byte {opcode_at}");
        match byte & 0xf0 {
            DONE => break,
            SET_TYPE_IMM => {}
            SET_SEGMENT_AND_OFFSET_ULEB => {
                segment = imm;
                base = segments.get(imm as usize).copied();
                let Some(value) = uleb(&mut at) else {
                    damage.get_or_insert(truncated);
                    break;
                };
                offset = value;
            }
            ADD_ADDR_ULEB => {
                let Some(value) = uleb(&mut at) else {
                    damage.get_or_insert(truncated);
                    break;
                };
                offset = offset.wrapping_add(value);
            }
            ADD_ADDR_IMM_SCALED => offset = offset.wrapping_add(u64::from(imm) * ptr),
            DO_REBASE_IMM_TIMES => emit(u64::from(imm), ptr, &mut offset, &mut out),
            DO_REBASE_ULEB_TIMES => {
                let Some(count) = uleb(&mut at) else {
                    damage.get_or_insert(truncated);
                    break;
                };
                emit(count, ptr, &mut offset, &mut out);
            }
            DO_REBASE_ADD_ADDR_ULEB => {
                let Some(skip) = uleb(&mut at) else {
                    damage.get_or_insert(truncated);
                    break;
                };
                emit(1, skip.wrapping_add(ptr), &mut offset, &mut out);
            }
            DO_REBASE_ULEB_TIMES_SKIPPING_ULEB => {
                let (Some(count), Some(skip)) = (uleb(&mut at), uleb(&mut at)) else {
                    damage.get_or_insert(truncated);
                    break;
                };
                emit(count, skip.wrapping_add(ptr), &mut offset, &mut out);
            }
            _ => {
                damage.get_or_insert(format!("unknown rebase opcode 0x{byte:02x} at byte {opcode_at}"));
                break;
            }
        }
        if out.len() >= LIMIT {
            damage.get_or_insert(format!("the rebase opcodes list more than {LIMIT} slots"));
            break;
        }
    }
    (out, damage)
}

/// The dynamic relocation types that store an address of this image in a
/// word, on one machine: `RELATIVE` (`B + A`), the pointer-width absolute word
/// (`S + A`) and its unaligned twin where the machine has one (PowerPC's
/// `UADDR`, which GNU ld emits for a pointer at an unaligned offset), and
/// `GLOB_DAT` / `JUMP_SLOT` where the machine has them and they store a word.
/// A RISC-V GOT slot is an absolute word, and an ELFv1 PowerPC64 (`e_flags`
/// ABI 1) or a BSS-PLT PowerPC (no `DT_PPC_GOT`) PLT slot is not a word at all:
/// the first is a copied function descriptor, the second patched code.
/// `local_addend`: a relocation against a local symbol stores `B + A`, as on
/// 32-bit PowerPC, where binutils folds the symbol's value into the addend and
/// glibc's loader adds only the load bias.
struct PointerRelocs {
    relative: u32,
    absolute: u32,
    unaligned: Option<u32>,
    glob_dat: Option<u32>,
    jump_slot: Option<u32>,
    local_addend: bool,
}

impl PointerRelocs {
    /// Does `r_type` store `S + A` in a word whose addend may be implicit?
    fn word(&self, r_type: u32) -> bool {
        r_type == self.absolute || Some(r_type) == self.unaligned
    }

    /// Does `r_type` store a symbol's address, `S + A`?
    fn symbolic(&self, r_type: u32) -> bool {
        self.word(r_type) || Some(r_type) == self.glob_dat || Some(r_type) == self.jump_slot
    }
}

/// [`PointerRelocs`] for `file`'s machine, or why its dynamic relocations are
/// not read: SPARC's are not verified, and a MIPS image relocates its local GOT
/// entries without any.
fn pointer_relocs(file: &object::File) -> Result<PointerRelocs, String> {
    use object::{elf, Architecture as A};
    let arch = file.architecture();
    if let Some((relative, glob_dat, jump_slot)) = crate::loader::kuna_dynrelocs::reloc_triple(arch) {
        let absolute = match arch {
            A::X86_64 => elf::R_X86_64_64,
            A::Aarch64 => elf::R_AARCH64_ABS64,
            A::I386 => elf::R_386_32,
            _ => elf::R_ARM_ABS32,
        };
        return Ok(PointerRelocs {
            relative,
            absolute,
            unaligned: None,
            glob_dat: Some(glob_dat),
            jump_slot: Some(jump_slot),
            local_addend: false,
        });
    }
    let e_flags = match file.flags() {
        object::FileFlags::Elf { e_flags, .. } => e_flags,
        _ => 0,
    };
    Ok(match arch {
        A::Riscv64 | A::Riscv32 => PointerRelocs {
            relative: elf::R_RISCV_RELATIVE,
            absolute: if arch == A::Riscv64 { elf::R_RISCV_64 } else { elf::R_RISCV_32 },
            unaligned: None,
            glob_dat: None,
            jump_slot: Some(elf::R_RISCV_JUMP_SLOT),
            local_addend: false,
        },
        A::PowerPc64 => PointerRelocs {
            relative: elf::R_PPC64_RELATIVE,
            absolute: elf::R_PPC64_ADDR64,
            unaligned: Some(elf::R_PPC64_UADDR64),
            glob_dat: Some(elf::R_PPC64_GLOB_DAT),
            jump_slot: (e_flags & elf::EF_PPC64_ABI == 2).then_some(elf::R_PPC64_JMP_SLOT),
            local_addend: false,
        },
        A::PowerPc => {
            let secure_plt = crate::loader::kuna_dynrelocs::dynamic_entries(file)
                .iter()
                .any(|&(tag, _)| tag == elf::DT_PPC_GOT);
            PointerRelocs {
                relative: elf::R_PPC_RELATIVE,
                absolute: elf::R_PPC_ADDR32,
                unaligned: Some(elf::R_PPC_UADDR32),
                glob_dat: Some(elf::R_PPC_GLOB_DAT),
                jump_slot: secure_plt.then_some(elf::R_PPC_JMP_SLOT),
                local_addend: true,
            }
        }
        A::Mips | A::Mips64 | A::Mips64_N32 => {
            return Err("MIPS dynamic relocations are not read: its local GOT entries are relocated without one".into())
        }
        _ => return Err(format!("{arch:?} dynamic relocations are not read")),
    })
}

/// A position-independent ELF's dynamic relocations whose value is an address
/// of this image ([`PointerRelocs`]): `RELATIVE` (`B + A`, the bias 0 at the
/// vaddrs kuna maps, and every `SHT_RELR` slot, whose addend is the word it
/// holds), and an absolute word, `GLOB_DAT` or `JUMP_SLOT` against a symbol the
/// image defines (`S + A`). A shared object's table of its own exported
/// functions is the absolute kind (`R_X86_64_64`), the symbol being
/// preemptible. An undefined symbol is an import. Read here rather than through
/// `dynrelocs`, whose writes are an option of the loaded image, not of the
/// metadata. Partial when a table the walk does not read exists
/// ([`crate::loader::kuna_dynrelocs::unread_tables`]).
fn elf(file: &object::File) -> DataPointers {
    let kinds = match pointer_relocs(file) {
        Ok(kinds) => kinds,
        Err(why) => return DataPointers::empty(Coverage::Unsupported, why),
    };
    let width: u8 = if file.is_64() { 8 } else { 4 };
    let mask = if width == 8 { u64::MAX } else { u64::from(u32::MAX) };
    let defined: std::collections::HashMap<usize, (u64, bool)> = file
        .dynamic_symbols()
        .filter(|sym| !sym.is_undefined())
        .map(|sym| (sym.index().0, (sym.address(), sym.is_local())))
        .collect();
    let mut pointers = Vec::new();
    let mut missing = 0usize;
    for (slot, reloc) in file.dynamic_relocations().into_iter().flatten() {
        let RelocationFlags::Elf { r_type } = reloc.flags() else { continue };
        if r_type != kinds.relative && !kinds.symbolic(r_type) {
            continue;
        }
        let symbol = if r_type == kinds.relative {
            0
        } else {
            let RelocationTarget::Symbol(index) = reloc.target() else { continue };
            let Some(&(address, local)) = defined.get(&index.0) else { continue };
            if local && kinds.local_addend { 0 } else { address }
        };
        let addend = if !reloc.has_implicit_addend() {
            reloc.addend() as u64
        } else if r_type == kinds.relative || kinds.word(r_type) {
            let Some(word) = read_word(file, slot, width as usize) else {
                missing += 1;
                continue;
            };
            word
        } else {
            0
        };
        pointers.push(DataPointer { slot, target: symbol.wrapping_add(addend) & mask, width });
    }
    let (relr, relr_unread) = crate::loader::kuna_dynrelocs::elf_relr_slots(file, width as usize);
    for slot in relr {
        match read_word(file, slot, width as usize) {
            Some(word) => pointers.push(DataPointer { slot, target: word & mask, width }),
            None => missing += 1,
        }
    }
    let mut unread = crate::loader::kuna_dynrelocs::unread_tables(file);
    unread.extend(relr_unread);
    if missing != 0 {
        unread.push(format!("{missing} relocated slots have no file data to read the addend from"));
    }
    DataPointers::listed(Source::DynamicRelocations, pointers, unread)
}

/// A PE's `HIGHLOW` / `DIR64` base relocations, each slot read for its
/// preferred-base target. The directory's blocks are walked here, not through
/// `object`'s iterator, which ends silently at a block it rejects, and walked
/// as the Windows loader walks them: every block whose header starts inside the
/// directory, its entries read from the section even past the directory's
/// end, until a zero-size block. A block that does not fit in the section
/// leaves the list partial. A directory of size 0 is no directory, to the
/// loader as here.
fn pe_relocated<Pe: ImageNtHeaders>(file: &object::File, pe: &PeFile<'_, Pe>) -> DataPointers {
    let Some(dir) = pe
        .data_directories()
        .get(object::pe::IMAGE_DIRECTORY_ENTRY_BASERELOC)
        .filter(|dir| dir.size.get(object::LittleEndian) != 0)
    else {
        return DataPointers::empty(
            Coverage::None,
            "a PE without a base-relocation directory loads only at its preferred base",
        );
    };
    let (rva, size) = (dir.virtual_address.get(object::LittleEndian), dir.size.get(object::LittleEndian));
    let sections = pe.section_table();
    let directory = match dir.data(pe.data(), &sections) {
        Ok(directory) => directory,
        Err(err) => {
            let why = format!("the base-relocation directory (RVA 0x{rva:x}, {size} bytes) cannot be read: {err}");
            return DataPointers::listed(Source::BaseRelocations, Vec::new(), vec![why]);
        }
    };
    let data = sections.pe_data_at(pe.data(), rva).unwrap_or(directory);
    let base = pe.relative_address_base();
    let word = |at: usize| u32::from_le_bytes([data[at], data[at + 1], data[at + 2], data[at + 3]]);
    let mut pointers = Vec::new();
    let mut unread = Vec::new();
    let mut missing = 0usize;
    let mut at = 0usize;
    while at < directory.len() && data.len() - at >= 8 {
        let page = u64::from(word(at));
        let size = word(at + 4) as usize;
        if size == 0 {
            break;
        }
        if size < 8 || size > data.len() - at {
            unread.push(format!("the base-relocation block at directory offset 0x{at:x} has size {size}"));
            break;
        }
        for entry in data[at + 8..at + size].chunks_exact(2) {
            let entry = u16::from_le_bytes([entry[0], entry[1]]);
            let width: u8 = match entry >> 12 {
                IMAGE_REL_BASED_DIR64 => 8,
                IMAGE_REL_BASED_HIGHLOW => 4,
                _ => continue,
            };
            let slot = base.wrapping_add(page).wrapping_add(u64::from(entry & 0xfff));
            match read_word(file, slot, width as usize) {
                Some(target) => pointers.push(DataPointer { slot, target, width }),
                None => missing += 1,
            }
        }
        at += size;
    }
    if missing != 0 {
        unread.push(format!("{missing} base relocations name a slot with no file data"));
    }
    DataPointers::listed(Source::BaseRelocations, pointers, unread)
}

/// The `width`-byte word at `vma` in the file's segment bytes, in its byte order.
fn read_word(file: &object::File, vma: u64, width: usize) -> Option<u64> {
    let le = file.is_little_endian();
    file.segments().find_map(|seg| {
        let off = usize::try_from(vma.checked_sub(seg.address())?).ok()?;
        let data = seg.data().ok()?;
        let raw = data.get(off..off.checked_add(width)?)?;
        let mut buf = [0u8; 8];
        Some(if le {
            buf[..width].copy_from_slice(raw);
            u64::from_le_bytes(buf)
        } else {
            buf[8 - width..].copy_from_slice(raw);
            u64::from_be_bytes(buf)
        })
    })
}

#[cfg(test)]
mod tests;
