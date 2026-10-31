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
//! - a position-independent ELF carries a dynamic relocation for each, whose
//!   value [`crate::loader::kuna_dynrelocs::resolve`] computes;
//! - a PE lists them in its `.reloc` base relocations, the slot holding the
//!   preferred-base target.
//!
//! [`data_pointers`] reads that list and keeps each word whose target lies in an
//! allocated section of the image. [`Coverage`] says which list it was, and
//! [`Coverage::None`] that the image keeps none: a non-PIE ELF or a PE without
//! base relocations stores its pointers as plain words, which no metadata tells
//! apart from integers. Imports are not here: a bound slot names a symbol of
//! another image, not an address of this one.

use object::pe::{IMAGE_REL_BASED_DIR64, IMAGE_REL_BASED_HIGHLOW};
use object::read::macho::{LoadCommandVariant, MachHeader, Segment as MachSegment};
use object::read::pe::{ImageNtHeaders, PeFile};
use object::read::{Object, ObjectSection, ObjectSegment};
use object::{BinaryFormat, FileKind, ObjectKind};

/// Which relocation metadata listed an image's data pointers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Coverage {
    /// `LC_DYLD_CHAINED_FIXUPS` rebases.
    ChainedFixups,
    /// `LC_DYLD_INFO` rebase opcodes.
    RebaseOpcodes,
    /// ELF dynamic relocations.
    DynamicRelocations,
    /// PE base relocations.
    BaseRelocations,
    /// The image lists no pointer relocations, so its data pointers are not
    /// enumerable.
    None,
}

impl Coverage {
    /// The name the CLI reports.
    pub fn as_str(self) -> &'static str {
        match self {
            Coverage::ChainedFixups => "chained-fixups",
            Coverage::RebaseOpcodes => "rebase-opcodes",
            Coverage::DynamicRelocations => "dynamic-relocations",
            Coverage::BaseRelocations => "base-relocations",
            Coverage::None => "none",
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

/// Every data pointer an image's relocation metadata lists, by slot.
#[derive(Debug, Clone)]
pub struct DataPointers {
    pub coverage: Coverage,
    pub pointers: Vec<DataPointer>,
}

/// The data pointers `file` lists (`bytes` is its raw image), sorted by slot,
/// each target inside an allocated section. A relocatable object has no load
/// addresses yet, so it lists none.
pub fn data_pointers(file: &object::File, bytes: &[u8]) -> DataPointers {
    let none = DataPointers { coverage: Coverage::None, pointers: Vec::new() };
    if file.kind() == ObjectKind::Relocatable {
        return none;
    }
    let (coverage, mut pointers) = match file.format() {
        BinaryFormat::MachO => macho(file, bytes),
        BinaryFormat::Elf if file.kind() == ObjectKind::Dynamic => elf(file, bytes),
        BinaryFormat::Pe => match file {
            object::File::Pe32(pe) => pe_relocated(file, pe),
            object::File::Pe64(pe) => pe_relocated(file, pe),
            _ => (Coverage::None, Vec::new()),
        },
        _ => (Coverage::None, Vec::new()),
    };
    if coverage == Coverage::None {
        return none;
    }
    let mapped: Vec<(u64, u64)> = file
        .sections()
        .filter(|sec| sec.size() != 0 && allocated(sec.flags()))
        .map(|sec| (sec.address(), sec.address().saturating_add(sec.size())))
        .collect();
    pointers.retain(|p| mapped.iter().any(|&(lo, hi)| p.target >= lo && p.target < hi));
    pointers.sort_unstable_by_key(|p| p.slot);
    pointers.dedup_by_key(|p| p.slot);
    DataPointers { coverage, pointers }
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
fn macho(file: &object::File, bytes: &[u8]) -> (Coverage, Vec<DataPointer>) {
    let fixups = crate::loader::format::macho::resolve_chained_fixups(file, bytes);
    if !fixups.is_empty() {
        let width = fixups.slot_width();
        let pointers = fixups
            .rebases()
            .map(|(slot, target)| DataPointer { slot, target, width })
            .collect();
        return (Coverage::ChainedFixups, pointers);
    }
    let width: u8 = if file.is_64() { 8 } else { 4 };
    let slots = match FileKind::parse(bytes) {
        Ok(FileKind::MachO64) => rebase_slots::<object::macho::MachHeader64<object::Endianness>>(bytes),
        Ok(FileKind::MachO32) => rebase_slots::<object::macho::MachHeader32<object::Endianness>>(bytes),
        _ => None,
    };
    let Some(slots) = slots else { return (Coverage::None, Vec::new()) };
    let pointers = slots
        .into_iter()
        .filter_map(|slot| {
            let target = read_word(file, slot, width as usize)?;
            Some(DataPointer { slot, target, width })
        })
        .collect();
    (Coverage::RebaseOpcodes, pointers)
}

/// Interpret the `LC_DYLD_INFO` rebase opcodes into the slot addresses they
/// rebase. `None` when the image carries no rebase information.
fn rebase_slots<Mach>(bytes: &[u8]) -> Option<Vec<u64>>
where
    Mach: MachHeader<Endian = object::Endianness>,
{
    let header = Mach::parse(bytes, 0).ok()?;
    let endian = header.endian().ok()?;
    let mut commands = header.load_commands(endian, bytes, 0).ok()?;
    let mut segments: Vec<u64> = Vec::new();
    let mut rebase: Option<(usize, usize)> = None;
    while let Ok(Some(command)) = commands.next() {
        if let Ok(Some((segment, _))) = Mach::Segment::from_command(command) {
            segments.push(segment.vmaddr(endian).into());
            continue;
        }
        if let Ok(LoadCommandVariant::DyldInfo(info)) = command.variant() {
            let off = info.rebase_off.get(endian) as usize;
            let size = info.rebase_size.get(endian) as usize;
            if size != 0 {
                rebase = Some((off, size));
            }
        }
    }
    let (off, size) = rebase?;
    let ops = bytes.get(off..off.checked_add(size)?)?;
    let ptr: u64 = if header.is_type_64() { 8 } else { 4 };
    Some(interpret_rebase(ops, &segments, ptr))
}

/// The rebase opcode machine of `mach-o/loader.h` (`REBASE_OPCODE_*`), yielding
/// `segment.vmaddr + offset` for each rebased slot. Bounded: a malformed stream
/// stops, never loops.
fn interpret_rebase(ops: &[u8], segments: &[u64], ptr: u64) -> Vec<u64> {
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
    let mut offset: u64 = 0;
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
        at += 1;
        let imm = byte & 0x0f;
        let emit = |count: u64, step: u64, offset: &mut u64, out: &mut Vec<u64>| {
            for _ in 0..count.min(LIMIT as u64) {
                if let Some(seg) = base {
                    out.push(seg.wrapping_add(*offset));
                }
                *offset = offset.wrapping_add(step);
            }
        };
        match byte & 0xf0 {
            DONE => break,
            SET_TYPE_IMM => {}
            SET_SEGMENT_AND_OFFSET_ULEB => {
                base = segments.get(imm as usize).copied();
                let Some(value) = uleb(&mut at) else { break };
                offset = value;
            }
            ADD_ADDR_ULEB => {
                let Some(value) = uleb(&mut at) else { break };
                offset = offset.wrapping_add(value);
            }
            ADD_ADDR_IMM_SCALED => offset = offset.wrapping_add(u64::from(imm) * ptr),
            DO_REBASE_IMM_TIMES => emit(u64::from(imm), ptr, &mut offset, &mut out),
            DO_REBASE_ULEB_TIMES => {
                let Some(count) = uleb(&mut at) else { break };
                emit(count, ptr, &mut offset, &mut out);
            }
            DO_REBASE_ADD_ADDR_ULEB => {
                let Some(skip) = uleb(&mut at) else { break };
                emit(1, skip.wrapping_add(ptr), &mut offset, &mut out);
            }
            DO_REBASE_ULEB_TIMES_SKIPPING_ULEB => {
                let (Some(count), Some(skip)) = (uleb(&mut at), uleb(&mut at)) else { break };
                emit(count, skip.wrapping_add(ptr), &mut offset, &mut out);
            }
            _ => break,
        }
        if out.len() >= LIMIT {
            break;
        }
    }
    out
}

/// A position-independent ELF's dynamic relocations whose value is an address
/// of this image ([`crate::loader::kuna_dynrelocs::resolve`]).
fn elf(file: &object::File, bytes: &[u8]) -> (Coverage, Vec<DataPointer>) {
    if file.dynamic_relocations().is_none() {
        return (Coverage::None, Vec::new());
    }
    let pointers = crate::loader::kuna_dynrelocs::resolve(file, bytes)
        .writes
        .into_iter()
        .map(|w| DataPointer { slot: w.vma, target: w.value, width: w.width })
        .collect();
    (Coverage::DynamicRelocations, pointers)
}

/// A PE's `HIGHLOW` / `DIR64` base relocations, each slot read for its
/// preferred-base target.
fn pe_relocated<Pe: ImageNtHeaders>(
    file: &object::File,
    pe: &PeFile<'_, Pe>,
) -> (Coverage, Vec<DataPointer>) {
    let sections = pe.section_table();
    let Ok(Some(blocks)) = pe.data_directories().relocation_blocks(pe.data(), &sections) else {
        return (Coverage::None, Vec::new());
    };
    let base = pe.relative_address_base();
    let mut pointers = Vec::new();
    for rel in blocks.flatten().flatten() {
        let width: u8 = match rel.typ {
            IMAGE_REL_BASED_DIR64 => 8,
            IMAGE_REL_BASED_HIGHLOW => 4,
            _ => continue,
        };
        let slot = base + u64::from(rel.virtual_address);
        if let Some(target) = read_word(file, slot, width as usize) {
            pointers.push(DataPointer { slot, target, width });
        }
    }
    if pointers.is_empty() {
        return (Coverage::None, Vec::new());
    }
    (Coverage::BaseRelocations, pointers)
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
