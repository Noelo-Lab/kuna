use std::collections::BTreeMap;
use std::ops::Range;

use object::{Architecture, BinaryFormat, Object, ObjectKind, ObjectSection, ObjectSegment, SectionFlags, SegmentFlags};
use sha2::{Digest, Sha256};

use super::address::check_image_base;
use super::elf_sections::{CodePatch, SectionMap};
use super::*;

// ChainedBuffer.XOR_MASK_BYTES in Ghidra 12.1.2. The root length's high bit
// selects this reversible on-disk encoding for each physical data block.
const STORAGE_XOR: [u8; 128] = [
    0x59, 0xea, 0x67, 0x23, 0xda, 0xb8, 0x00, 0xb8, 0xc3, 0x48, 0xdd, 0x8b, 0x21, 0xd6, 0x94, 0x78,
    0x35, 0xab, 0x2b, 0x7e, 0xb2, 0x4f, 0x82, 0x4e, 0x0e, 0x16, 0xc4, 0x57, 0x12, 0x8e, 0x7e, 0xe6,
    0xb6, 0xbd, 0x56, 0x91, 0x57, 0x72, 0xe6, 0x91, 0xdc, 0x52, 0x2e, 0xf2, 0x1a, 0xb7, 0xd6, 0x6f,
    0xda, 0xde, 0xe8, 0x48, 0xb1, 0xbb, 0x50, 0x6f, 0xf4, 0xdd, 0x11, 0xee, 0xf2, 0x67, 0xfe, 0x48,
    0x8d, 0xae, 0x69, 0x1a, 0xe0, 0x26, 0x8c, 0x24, 0x8e, 0x17, 0x76, 0x51, 0xe2, 0x60, 0xd7, 0xe6,
    0x83, 0x65, 0xd5, 0xf0, 0x7f, 0xf2, 0xa0, 0xd6, 0x4b, 0xbd, 0x24, 0xd8, 0xab, 0xea, 0x9e, 0xa6,
    0x48, 0x94, 0x3e, 0x7b, 0x2c, 0xf4, 0xce, 0xdc, 0x69, 0x11, 0xf8, 0x3c, 0xa7, 0x3f, 0x5d, 0x77,
    0x94, 0x3f, 0xe4, 0x8e, 0x48, 0x20, 0xdb, 0x56, 0x32, 0xc1, 0x87, 0x01, 0x2e, 0xe3, 0x7f, 0x40,
];

pub(super) struct Image {
    pub(super) original: Vec<u8>,
    entries: EntryMap,
    external: Option<Range<u64>>,
}

enum EntryMap {
    Linked { slide: i128, code: Vec<Range<u64>> },
    Sections(SectionMap),
}

impl Image {
    pub(super) fn project_entries(&self, functions: &mut Vec<ProgramFunction>) -> KunaResult<()> {
        if let Some(external) = &self.external {
            functions.retain(|function| {
                !matches!(function.location, ProgramEntry::LinkedAddress(addr) if external.contains(&addr))
            });
        }
        for function in &mut *functions {
            let ProgramEntry::LinkedAddress(address) = function.location else {
                return Err(bad("invalid unprojected function entry"));
            };
            let missing = || bad(format_args!(
                "saved function {address:#x} has no verified original executable file backing",
            ));
            function.location = match &self.entries {
                EntryMap::Linked { slide, code } => {
                    let index = code.partition_point(|range| range.end <= address);
                    if code.get(index).is_none_or(|range| !range.contains(&address)) {
                        return Err(missing());
                    }
                    ProgramEntry::LinkedAddress(native_address(address, *slide)?)
                }
                EntryMap::Sections(map) => map.entry(address).ok_or_else(missing)?,
            };
        }
        if matches!(self.entries, EntryMap::Sections(_)) {
            functions.sort_unstable_by_key(|function| function.location);
            if functions.windows(2).any(|pair| pair[0].location == pair[1].location) {
                return Err(bad("duplicate projected ELF section function entries"));
            }
        }
        Ok(())
    }
}

fn native_address(project: u64, slide: i128) -> KunaResult<u64> {
    u64::try_from(i128::from(project) - slide)
        .map_err(|_| bad("project address is outside the original address space"))
}

pub(super) fn read(
    db: &Database<'_>, metadata: &Metadata, map: &AddressMap, used: &mut HashSet<i32>,
    database_size: usize, max_original_size: usize,
) -> KunaResult<Image> {
    let table = require_table(db, "File Bytes", 0, (Long, "Key"), &[
        (Str, "Filename"), (Long, "Offset"), (Long, "Size"), (Binary, "Chain Buffer IDs"),
        (Binary, "Layered Chain Buffer IDs"),
    ])?;
    if table.record_count != 1 {
        return Err(bad("expected exactly one original File Bytes record; missing or ambiguous import"));
    }
    let mut file = None;
    db.visit_records(table, used, |record| {
        file = Some(record);
        Ok(())
    })?;
    let file = file.ok_or_else(|| bad("missing original File Bytes record"))?;
    let [FieldValue::Str(Some(name)), FieldValue::Long(offset), FieldValue::Long(size),
        FieldValue::Binary(Some(original)), FieldValue::Binary(Some(layered))] = file.columns.as_slice()
    else {
        return Err(bad("invalid File Bytes record"));
    };
    if *offset != 0 {
        return Err(bad(format_args!("sliced File Bytes import {name:?} starts at offset {offset}")));
    }
    let size = usize::try_from(*size).ok().filter(|size| *size > 0 && *size < database_size)
        .ok_or_else(|| bad("invalid original File Bytes size"))?;
    if size > max_original_size {
        return Err(bad(format_args!("original File Bytes exceed the {max_original_size}-byte limit")));
    }
    let original_ids = chain_ids(original)?;
    let layered_ids = chain_ids(layered)?;
    if original_ids.len() != layered_ids.len() {
        return Err(bad("original and layered File Bytes chain counts differ"));
    }
    let mut original = Vec::new();
    original.try_reserve_exact(size).map_err(|_| bad("cannot allocate original File Bytes"))?;
    let mut chunk_size = None;
    for (index, id) in original_ids.enumerate() {
        let last = index + 1 == layered_ids.len();
        let len = db.append_file_chain(id, size - original.len(), used, &mut original, &STORAGE_XOR)?;
        if let Some(first) = chunk_size {
            if (last && len > first) || (!last && len != first) {
                return Err(bad("inconsistent File Bytes chain chunk sizes"));
            }
        } else {
            chunk_size = Some(len);
        }
    }
    if original.len() != size {
        return Err(bad("File Bytes chains do not cover the declared original size"));
    }
    let digest = Sha256::digest(&original);
    if !kuna_base::hex_lower(&digest).eq_ignore_ascii_case(&metadata.executable_sha256) {
        return Err(bad("original File Bytes do not match the Executable SHA256 (sliced or corrupt import)"));
    }

    let executable = parse_original_image(&original)?;
    let relocatable = executable.kind() == ObjectKind::Relocatable;
    let track_uninitialized = relocatable && executable.architecture() == Architecture::Arm;
    let saved = read_memory(db, map, used, file.key, size, relocatable, track_uninitialized)?;
    let (entries, ranges, patches) = if relocatable {
        let (sections, code) = SectionMap::read(&executable, size, max_original_size, &saved.file)?;
        let patches = sections.relocations(db, map, used, &executable, &original, metadata, &saved)?;
        (EntryMap::Sections(sections), code, patches)
    } else {
        let slide = linked_slide(&executable, metadata)?;
        let (code, ranges) = linked_ranges(&executable, slide, size, &saved)?;
        (EntryMap::Linked { slide, code }, ranges, Vec::new())
    };
    // Every layered chain overlays the corresponding original chain. Unstored
    // slots inherit original data, and non-executable changes are irrelevant.
    let chunk_size = chunk_size.ok_or_else(|| bad("missing original File Bytes chains"))?;
    let mut start = 0usize;
    for id in layered_ids {
        let end = start.saturating_add(chunk_size).min(size);
        let expected = LayerExpectation {
            original: &original[start..end], start, ranges: &ranges, patches: &patches,
            relocatable: matches!(entries, EntryMap::Sections(_)),
        };
        expected.check(db, id, used)?;
        start = end;
    }
    Ok(Image { original, entries, external: saved.external })
}

struct LayerExpectation<'a> {
    original: &'a [u8],
    start: usize,
    ranges: &'a [Range<usize>],
    patches: &'a [CodePatch],
    relocatable: bool,
}

impl LayerExpectation<'_> {
    fn check(&self, db: &Database<'_>, id: i32, used: &mut HashSet<i32>) -> KunaResult<()> {
        let mut next = self.start;
        db.visit_file_layer(id, self.original.len(), used, &STORAGE_XOR, |offset, stored, xor| {
            let start = self.start + offset;
            self.inherited(next..start)?;
            self.stored(start, stored, xor)?;
            next = start + stored.len();
            Ok(())
        })?;
        self.inherited(next..self.start + self.original.len())
    }

    fn patches(&self, range: Range<usize>) -> impl Iterator<Item = &CodePatch> {
        let index = self.patches.partition_point(|patch| patch.end() <= range.start);
        self.patches[index..].iter().take_while(move |patch| patch.offset < range.end)
    }

    fn inherited(&self, range: Range<usize>) -> KunaResult<()> {
        // An omitted block only equals the expected layer if it does not
        // omit any bytes the stock relocation would have changed.
        for patch in self.patches(range.clone()) {
            let lo = range.start.max(patch.offset);
            let hi = range.end.min(patch.end());
            if self.original[lo - self.start..hi - self.start] != patch.bytes()[lo - patch.offset..hi - patch.offset] {
                return Err(relocation_mismatch(lo..hi));
            }
        }
        Ok(())
    }

    fn stored(&self, start: usize, stored: &[u8], xor: &[u8]) -> KunaResult<()> {
        for range in intersections(self.ranges, &(start..start + stored.len())) {
            let mut next = range.start;
            for patch in self.patches(range.clone()) {
                let lo = range.start.max(patch.offset);
                let hi = range.end.min(patch.end());
                self.unchanged(next..lo, start, stored, xor)?;
                if !equal_layer(stored, xor, lo - start, &patch.bytes()[lo - patch.offset..hi - patch.offset]) {
                    return Err(relocation_mismatch(lo..hi));
                }
                next = hi;
            }
            self.unchanged(next..range.end, start, stored, xor)?;
        }
        Ok(())
    }

    fn unchanged(&self, range: Range<usize>, start: usize, stored: &[u8], xor: &[u8]) -> KunaResult<()> {
        if equal_layer(stored, xor, range.start - start, &self.original[range.start - self.start..range.end - self.start]) {
            return Ok(());
        }
        let reason = if self.relocatable { " without a verified stock ELF relocation" } else { "" };
        Err(bad(format_args!(
            "layered File Bytes changes executable bytes{reason} in file range {:#x}..{:#x}", range.start, range.end,
        )))
    }
}

fn equal_layer(stored: &[u8], xor: &[u8], offset: usize, expected: &[u8]) -> bool {
    let actual = &stored[offset..offset + expected.len()];
    if xor.is_empty() {
        actual == expected
    } else {
        actual.iter().zip(expected).enumerate().all(|(i, (&a, &b))| a ^ xor[(offset + i) % xor.len()] == b)
    }
}

fn relocation_mismatch(range: Range<usize>) -> KunaError {
    bad(format_args!(
        "layered File Bytes disagrees with the verified stock ELF relocation in file range {:#x}..{:#x}",
        range.start, range.end,
    ))
}

fn parse_original_image(bytes: &[u8]) -> KunaResult<object::File<'_>> {
    let unsupported = || bad("unsupported original executable layout; cannot verify file backing");
    let file = object::File::parse(bytes).map_err(|_| unsupported())?;
    if file.format() == BinaryFormat::Elf && file.kind() == ObjectKind::Relocatable {
        if file.segments().next().is_some() {
            return Err(bad("unsupported relocatable ELF (ET_REL) with loadable segments"));
        }
        return Ok(file);
    }
    if !matches!(file.format(), BinaryFormat::Elf | BinaryFormat::Pe | BinaryFormat::MachO)
        || !matches!(file.kind(), ObjectKind::Executable | ObjectKind::Dynamic)
    {
        return Err(unsupported());
    }
    Ok(file)
}

fn linked_slide(executable: &object::File<'_>, metadata: &Metadata) -> KunaResult<i128> {
    match executable.format() {
        BinaryFormat::Pe => check_image_base(executable.relative_address_base(), metadata.image_base)?,
        BinaryFormat::Elf => {
            let native_base = metadata.elf_original_base
                .ok_or_else(|| bad("missing ELF Original Image Base; cannot validate the program layout"))?;
            if executable.kind() == ObjectKind::Dynamic {
                // Stock Ghidra normally maps zero-based PIE and shared objects at
                // 0x100000. Accept only a uniform offset verified below.
                return Ok(i128::from(metadata.image_base) - i128::from(native_base));
            }
            check_image_base(native_base, metadata.image_base)?;
        }
        _ => {}
    }
    Ok(0)
}

fn chain_ids(bytes: &[u8]) -> KunaResult<impl ExactSizeIterator<Item = i32> + '_> {
    // BinaryCodedField(int[]): the type discriminator 4, a nonnull tag 0,
    // and packed big-endian i32 values without a separate count.
    let bytes = bytes.strip_prefix(&[4, 0]).ok_or_else(|| bad("invalid File Bytes chain ID array"))?;
    let (ids, remainder) = bytes.as_chunks::<4>();
    if ids.is_empty() || !remainder.is_empty() {
        return Err(bad("missing or misaligned File Bytes chain IDs"));
    }
    Ok(ids.iter().copied().map(i32::from_be_bytes))
}

pub(super) struct SavedMemory {
    pub(super) file: Vec<SavedFileRange>,
    pub(super) external: Option<Range<u64>>,
    pub(super) uninitialized: Vec<SavedUninitialized>,
}

pub(super) struct SavedUninitialized {
    pub(super) name: String,
    pub(super) range: Range<u64>,
    pub(super) artificial: bool,
}

pub(super) struct SavedFileRange {
    pub(super) project: Range<u64>,
    pub(super) file: Range<usize>,
    pub(super) executable: bool,
}

struct MemoryBlock {
    name: Option<String>,
    artificial: bool,
    unbacked_code: u64,
    range: Range<u64>,
    executable: bool,
    external: bool,
}

const MEMORY_EXECUTE: i8 = 0x01;
const MEMORY_ARTIFICIAL: i8 = 0x10;
const SUB_BLOCK_UNINITIALIZED: i8 = 3;
const SUB_BLOCK_FILE_BYTES: i8 = 4;

// Native executable mappings grouped by virtual-address minus file-offset bias.
// Kuna loads segment bytes; section flags can identify additional code only
// where its file-to-address mapping agrees with a loadable segment.
fn native_code_file_ranges(
    file: &object::File<'_>, size: usize,
) -> KunaResult<BTreeMap<i128, Vec<Range<usize>>>> {
    let mut backing = BTreeMap::<i128, Vec<Range<usize>>>::new();
    let mut code = BTreeMap::<i128, Vec<Range<usize>>>::new();
    for segment in file.segments() {
        let executable = match segment.flags() {
            SegmentFlags::Elf { p_flags } => p_flags & object::elf::PF_X != 0,
            SegmentFlags::Coff { characteristics } => characteristics & object::pe::IMAGE_SCN_MEM_EXECUTE != 0,
            SegmentFlags::MachO { initprot, .. } => initprot & object::macho::VM_PROT_EXECUTE != 0,
            _ => false,
        };
        let (offset, len) = segment.file_range();
        if len == 0 {
            continue;
        }
        let bytes = native_file_range(segment.address(), offset, len, size)?;
        let bias = i128::from(segment.address()) - i128::from(offset);
        if executable {
            code.entry(bias).or_default().push(bytes.clone());
        }
        backing.entry(bias).or_default().push(bytes);
    }
    for ranges in backing.values_mut() {
        merge_ranges(ranges);
    }
    let sections = file.sections().filter_map(|section| {
        if matches!(section.flags(), SectionFlags::Elf { sh_flags }
            if sh_flags & u64::from(object::elf::SHF_ALLOC) == 0)
            || !crate::entry::is_executable(&section)
        {
            return None;
        }
        Some((section.address(), section.file_range()?))
    });
    for (address, (offset, len)) in sections.filter(|(_, (_, len))| *len != 0) {
        let bytes = native_file_range(address, offset, len, size)?;
        let bias = i128::from(address) - i128::from(offset);
        if let Some(ranges) = backing.get(&bias) {
            for range in intersections(ranges, &bytes) {
                code.entry(bias).or_default().push(range);
            }
        }
    }
    for ranges in code.values_mut() {
        merge_ranges(ranges);
    }
    Ok(code)
}

fn native_file_range(address: u64, offset: u64, len: u64, size: usize) -> KunaResult<Range<usize>> {
    let end = offset.checked_add(len).filter(|&end| end <= size as u64)
        .ok_or_else(|| bad("native memory mapping extends beyond original File Bytes"))?;
    address.checked_add(len).ok_or_else(|| bad("native memory mapping address overflows"))?;
    Ok(offset as usize..end as usize)
}

fn intersections<'a>(
    sorted: &'a [Range<usize>], target: &'a Range<usize>,
) -> impl Iterator<Item = Range<usize>> + 'a {
    sorted[sorted.partition_point(|range| range.end <= target.start)..].iter()
        .take_while(move |range| range.start < target.end)
        .map(move |range| range.start.max(target.start)..range.end.min(target.end))
}

fn merge_ranges<T: Copy + Ord>(ranges: &mut Vec<Range<T>>) {
    ranges.sort_unstable_by_key(|range| range.start);
    let mut output = 0;
    for input in 0..ranges.len() {
        if output > 0 && ranges[input].start <= ranges[output - 1].end {
            ranges[output - 1].end = ranges[output - 1].end.max(ranges[input].end);
        } else {
            ranges.swap(output, input);
            output += 1;
        }
    }
    ranges.truncate(output);
}

fn check_disjoint(ranges: &mut [Range<u64>]) -> KunaResult<()> {
    ranges.sort_unstable_by_key(|range| range.start);
    if ranges.windows(2).any(|pair| pair[0].end > pair[1].start) {
        return Err(bad("overlapping saved memory mappings in the default space"));
    }
    Ok(())
}

fn read_memory(
    db: &Database<'_>, map: &AddressMap, used: &mut HashSet<i32>, file_id: i64, size: usize,
    relocatable: bool,
    track_uninitialized: bool,
) -> KunaResult<SavedMemory> {
    let table = require_table(db, "Memory Blocks", 3, (Long, "Key"), &[
        (Str, "Name"), (Str, "Comments"), (Str, "Source Name"), (Byte, "Flags"),
        (Long, "Start Address"), (Long, "Length"), (Int, "Segment"),
    ])?;
    let mut blocks = BTreeMap::new();
    db.visit_records(table, used, |mut record| {
        let [FieldValue::Str(name), _, _, FieldValue::Byte(flags), FieldValue::Long(address),
            FieldValue::Long(length), FieldValue::Int(segment)] = record.columns.as_mut_slice()
        else {
            return Err(bad("invalid Memory Blocks record"));
        };
        if let Some(address) = map.decode(*address)? {
            let end = u64::try_from(*length).ok().filter(|&len| len != 0)
                .and_then(|len| address.checked_add(len))
                .filter(|&end| end - 1 <= map.layout.mask())
                .ok_or_else(|| bad("invalid saved memory block range"))?;
            if *segment != 0 {
                return Err(bad("unsupported segmented memory block"));
            }
            let executable = *flags & MEMORY_EXECUTE != 0;
            let artificial = *flags & MEMORY_ARTIFICIAL != 0;
            let external = !executable && artificial && name.as_deref() == Some("EXTERNAL");
            blocks.insert(record.key, MemoryBlock {
                name: if track_uninitialized && !external { name.take() } else { None },
                range: address..end, executable, artificial, external,
                unbacked_code: if relocatable && executable { end - address } else { 0 },
            });
        }
        Ok(())
    })?;
    check_disjoint(&mut blocks.values().map(|block| block.range.clone()).collect::<Vec<_>>())?;

    let table = require_table(db, "Sub Memory Blocks", 3, (Long, "Key"), &[
        (Long, "Parent ID"), (Byte, "Type"), (Long, "Length"), (Long, "Starting Offset"),
        (Int, "Source ID"), (Long, "Source Address/Offset"),
    ])?;
    let mut mapped = Vec::new();
    let mut external = None;
    let mut uninitialized = Vec::new();
    let mut file = Vec::new();
    db.visit_records(table, used, |record| {
        let [FieldValue::Long(parent), FieldValue::Byte(kind), FieldValue::Long(len),
            FieldValue::Long(sub_offset), FieldValue::Int(source), FieldValue::Long(file_offset)] =
            record.columns.as_slice()
        else {
            return Err(bad("invalid Sub Memory Blocks record"));
        };
        let Some(block) = blocks.get_mut(parent) else { return Ok(()) };
        let len = u64::try_from(*len).ok().filter(|&len| len != 0)
            .ok_or_else(|| bad("invalid saved memory sub-block length"))?;
        let project_start = u64::try_from(*sub_offset).ok().and_then(|offset| block.range.start.checked_add(offset))
            .ok_or_else(|| bad("invalid saved memory sub-block start"))?;
        let project_end = project_start.checked_add(len).filter(|&end| end <= block.range.end)
            .ok_or_else(|| bad("saved memory sub-block exceeds its parent block"))?;
        mapped.push(project_start..project_end);
        if *kind == SUB_BLOCK_UNINITIALIZED && !block.executable && (project_start..project_end) == block.range {
            if let Some(name) = block.name.take() {
                uninitialized.push(SavedUninitialized {
                    name, range: block.range.clone(), artificial: block.artificial,
                });
            }
            if block.external && external.replace(block.range.clone()).is_some() {
                return Err(bad("multiple synthetic EXTERNAL blocks"));
            }
        }
        if *kind != SUB_BLOCK_FILE_BYTES {
            if relocatable && block.executable {
                return Err(bad("saved executable ELF memory has no original File Bytes backing"));
            }
            return Ok(());
        }
        if i64::from(*source) != file_id {
            return Err(bad("saved memory sub-block references an unknown File Bytes record"));
        }
        if relocatable && block.executable {
            block.unbacked_code = block.unbacked_code.checked_sub(len)
                .ok_or_else(|| bad("saved executable ELF memory sub-blocks overlap"))?;
        }
        let start = usize::try_from(*file_offset).map_err(|_| bad("invalid saved memory file offset"))?;
        let end = usize::try_from(len).ok().and_then(|len| start.checked_add(len))
            .filter(|end| *end <= size).ok_or_else(|| bad("saved memory sub-block exceeds File Bytes"))?;
        file.push(SavedFileRange {
            project: project_start..project_end,
            file: start..end,
            executable: block.executable,
        });
        Ok(())
    })?;
    if blocks.values().any(|block| block.unbacked_code != 0) {
        return Err(bad("saved executable ELF memory has incomplete File Bytes backing"));
    }
    check_disjoint(&mut mapped)?;
    Ok(SavedMemory { file, external, uninitialized })
}

fn linked_ranges(
    native: &object::File<'_>, slide: i128, size: usize, saved: &SavedMemory,
) -> KunaResult<(Vec<Range<u64>>, Vec<Range<usize>>)> {
    let native_code = native_code_file_ranges(native, size)?;
    // Check native executable offsets even if the saved block is non-executable,
    // missing, or maps different File Bytes there.
    let mut ranges: Vec<_> = native_code.values().flatten().cloned().collect();
    let mut code = Vec::new();
    for block in &saved.file {
        let project_start = block.project.start;
        let start = block.file.start;
        let end = block.file.end;
        if block.executable {
            if !native_maps_range(native, native_address(project_start, slide)?, start as u64, (end - start) as u64) {
                return Err(bad(format_args!(
                    "unsupported project layout: executable sub-block at {project_start:#x} \
                     maps file bytes {start:#x}..{end:#x} differently from the original"
                )));
            }
            ranges.push(block.file.clone());
        }
        append_verified_ranges(&native_code, slide, project_start, block.file.clone(), &mut code);
    }
    if let Some(external) = &saved.external {
        let overlaps_code = native_code.iter().any(|(bias, ranges)| {
            let project_bias = *bias + slide;
            ranges.iter().any(|range| {
                project_bias + (range.start as i128) < i128::from(external.end)
                    && project_bias + (range.end as i128) > i128::from(external.start)
            })
        });
        if overlaps_code {
            return Err(bad("synthetic EXTERNAL block overlaps original executable memory"));
        }
    }
    merge_ranges(&mut ranges);
    merge_ranges(&mut code);
    Ok((code, ranges))
}

fn append_verified_ranges(
    native_code: &BTreeMap<i128, Vec<Range<usize>>>, slide: i128, project_start: u64,
    backing: Range<usize>, code: &mut Vec<Range<u64>>,
) {
    let bias = i128::from(project_start) - slide - backing.start as i128;
    if let Some(original) = native_code.get(&bias) {
        for range in intersections(original, &backing) {
            let lo = project_start + (range.start - backing.start) as u64;
            let hi = project_start + (range.end - backing.start) as u64;
            code.push(lo..hi);
        }
    }
}

fn native_maps_range(file: &object::File<'_>, project_address: u64, offset: u64, len: u64) -> bool {
    let matches = |(start, size): (u64, u64), address: u64| {
        offset.checked_sub(start).is_some_and(|delta| {
            size.checked_sub(delta).is_some_and(|rest| len <= rest)
                && address.checked_add(delta) == Some(project_address)
        })
    };
    file.segments().any(|segment| matches(segment.file_range(), segment.address()))
        || file.sections().any(|section| {
            section.file_range().is_some_and(|range| matches(range, section.address()))
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn elf_relocatable_original_uses_file_section_provenance() {
        let bytes = include_bytes!("../../tests/fixtures/fid_lib_x86_64.o");
        let original = parse_original_image(bytes).unwrap();
        let text = original.section_by_name(".text").unwrap();
        let (offset, len) = text.file_range().unwrap();
        let base = 0x100100;
        let (sections, _) = SectionMap::read(&original, bytes.len(), bytes.len(), &[SavedFileRange {
            project: base..base + len,
            file: offset as usize..(offset + len) as usize,
            executable: true,
        }]).unwrap();
        let image = Image { original: Vec::new(), entries: EntryMap::Sections(sections), external: None };
        let function = || ProgramFunction { location: ProgramEntry::LinkedAddress(base), name: "f".into() };
        let mut one = vec![function()];
        image.project_entries(&mut one).unwrap();
        assert_eq!(one[0].location, ProgramEntry::ElfSection { section_index: text.index().0, offset: 0 });
        let err = image.project_entries(&mut vec![function(), function()]).unwrap_err();
        assert!(err.explain().contains("duplicate projected ELF section function entries"));
    }

    #[test]
    fn file_bytes_chain_ids_require_the_binary_coded_nonnull_int_array() {
        let ids = [4, 0, 0, 0, 0, 7, 0, 1, 0, 0];
        assert!(chain_ids(&ids).unwrap().eq([7, 65536]));
        for invalid in [&ids[..1], &ids[..2], &ids[..9], &[4, 255][..], &[5, 0][..]] {
            assert!(chain_ids(invalid).is_err());
        }
    }

    #[test]
    fn fixed_base_entries_require_original_bytes_at_the_same_address() {
        let native = BTreeMap::from([(0x40_0000, std::iter::once(0x1000..0x1200).collect())]);
        let saved = 0x40_1000;
        let function = || ProgramFunction { location: ProgramEntry::LinkedAddress(saved), name: "main".into() };
        let image = |code| Image {
            original: Vec::new(), entries: EntryMap::Linked { slide: 0, code }, external: None,
        };
        assert!(image(Vec::new()).project_entries(&mut vec![function()]).is_err());

        let mut code = Vec::new();
        append_verified_ranges(&native, 0, saved, 0x1001..0x1100, &mut code);
        assert!(image(code).project_entries(&mut vec![function()]).is_err());

        let mut code = Vec::new();
        append_verified_ranges(&native, 0, saved, 0x1000..0x1100, &mut code);
        let mut entries = vec![function()];
        image(code).project_entries(&mut entries).unwrap();
        assert_eq!(entries, [function()]);
    }

    #[test]
    fn pie_entry_is_rebased_only_from_a_verified_file_mapping() {
        let image = Image {
            original: Vec::new(),
            entries: EntryMap::Linked { slide: 0x10_0000, code: std::iter::once(0x10_1000..0x10_1200).collect() },
            external: Some(0x10_5000..0x10_5010),
        };
        let function = |address, name: &str| ProgramFunction {
            location: ProgramEntry::LinkedAddress(address), name: name.into(),
        };
        let mut inside = vec![function(0x10_1129, "main"), function(0x10_5000, "__libc_start_main")];
        image.project_entries(&mut inside).unwrap();
        assert_eq!(inside, [function(0x1129, "main")]);
        let mut outside = vec![function(0x10_1200, "patched")];
        assert!(image.project_entries(&mut outside).is_err());
    }

    #[test]
    fn elf_relocatable_layer_requires_exact_relocation_across_stored_blocks() {
        let patch = CodePatch { offset: 0x11, bytes: [4, 5, 0, 6], width: 4 };
        let range = 0x10..0x16;
        let expected = LayerExpectation {
            original: &[9, 0, 0, 0, 0, 8], start: 0x10,
            ranges: std::slice::from_ref(&range), patches: std::slice::from_ref(&patch),
            relocatable: true,
        };
        assert!(expected.stored(0x10, &[9, 4, 5], &[]).is_ok());
        assert!(expected.stored(0x13, &[7, 14, 15], &[7, 8]).is_ok());
        assert!(expected.inherited(0x13..0x14).is_ok());
        assert!(expected.inherited(0x13..0x15).is_err());
        assert!(expected.stored(0x13, &[0, 0, 8], &[]).unwrap_err().explain()
            .contains("disagrees with the verified stock ELF relocation"));
        assert!(expected.stored(0x13, &[0, 6, 7], &[]).unwrap_err().explain()
            .contains("changes executable bytes without a verified stock ELF relocation"));

        let next_chain = LayerExpectation { original: &[0, 0, 8], start: 0x13, ..expected };
        assert!(next_chain.stored(0x13, &[0, 6, 8], &[]).is_ok());
        assert!(next_chain.inherited(0x13..0x15).is_err());
    }

    #[test]
    fn halfword_relocation_does_not_cover_the_next_instruction() {
        let patch = CodePatch { offset: 0x10, bytes: [3, 4, 0, 0], width: 2 };
        let range = 0x10..0x14;
        let expected = LayerExpectation {
            original: &[1, 2, 5, 6], start: 0x10,
            ranges: std::slice::from_ref(&range), patches: std::slice::from_ref(&patch),
            relocatable: true,
        };
        assert!(expected.stored(0x10, &[3, 4, 5, 6], &[]).is_ok());
        assert!(expected.stored(0x10, &[3, 4, 0, 0], &[]).unwrap_err().explain()
            .contains("changes executable bytes without a verified stock ELF relocation"));
    }

    #[test]
    fn elf_relocatable_saved_project_ranges_are_disjoint() {
        let mut adjacent = [0x1080..0x1084, 0x1070..0x1080];
        check_disjoint(&mut adjacent).unwrap();
        assert_eq!(adjacent, [0x1070..0x1080, 0x1080..0x1084]);
        let mut overlap = [0x100..0x120, 0x11f..0x130];
        assert!(check_disjoint(&mut overlap).unwrap_err().explain()
            .contains("overlapping saved memory mappings"));
    }
}
