use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet};
use std::ops::Range;

use object::elf;
use object::read::elf::{ElfFile32, SectionHeader, Sym};
use object::{Architecture, Object, ObjectKind, SectionIndex, SymbolIndex};

use super::{KunaResult, bad, elf_sections::SectionMap, file_bytes::SavedMemory};

const ADDRESS_LIMIT: u64 = 1_u64 << 32;
const LINKAGE_ALIGNMENT: u64 = 0x1000;
const PREFERRED_EXTERNAL_SPACE: u64 = 0x20000;
const SYMBOL_ALIGNMENT: u64 = 4;

// The file-backed path in SectionMap remains responsible for ordinary symbols.
pub(super) fn targets(
    file: &object::File<'_>, original: &[u8], sections: &SectionMap, saved: &SavedMemory,
    image_base: u64,
) -> KunaResult<BTreeMap<usize, u64>> {
    let object::File::Elf32(raw) = file else { return Ok(BTreeMap::new()) };
    if file.architecture() != Architecture::Arm || file.kind() != ObjectKind::Relocatable
        || !file.is_little_endian() || !raw.elf_program_headers().is_empty()
        || image_base >= ADDRESS_LIMIT
        || original.last_chunk::<4>().is_some_and(|magic| trim_elf_bytes(magic) == b"PRE")
    {
        return Ok(BTreeMap::new());
    }
    let endian = raw.endian();
    let headers = raw.elf_section_table();
    let symbols = raw.elf_symbol_table();
    // Ghidra visits every symbol table, including optional embedded debug data.
    // Our relocation indices are identities in a single regular symbol table.
    if symbols.is_empty() || headers.enumerate().any(|(index, header)| {
        let kind = header.sh_type(endian);
        kind == elf::SHT_DYNAMIC
            || (matches!(kind, elf::SHT_SYMTAB | elf::SHT_DYNSYM) && index != symbols.section())
            || headers.section_name(endian, header).is_ok_and(|name| name == b".gnu_debugdata")
    }) {
        return Ok(BTreeMap::new());
    }
    let symbol_header = headers.section(symbols.section()).map_err(|_| bad("missing ARM ELF symbol table"))?;
    if symbol_header.sh_entsize(endian) != 16 || symbol_header.sh_size(endian) % 16 != 0 {
        return Ok(BTreeMap::new());
    }

    let layout = if saved.external.is_some() || !saved.uninitialized.is_empty() {
        verified_layout(raw, original.len(), sections, saved, image_base)
    } else {
        None
    };
    let mut targets = saved.external.as_ref().zip(layout.as_ref())
        .and_then(|(expected, layout)| external_targets(raw, layout, expected, image_base))
        .unwrap_or_default();
    for (index, symbol) in symbols.enumerate().skip(1) {
        let kind = symbol.st_type();
        if matches!(kind, elf::STT_FILE | elf::STT_TLS) {
            continue;
        }
        let value = u64::from(symbol.st_value(endian));
        let address = if symbol.st_shndx(endian) == elf::SHN_ABS {
            defined_address(raw, symbol, value)
        } else if let Some(range) = symbols.symbol_section(endian, symbol, index).ok().flatten()
            .filter(|section| headers.section(*section).is_ok_and(|header| header.sh_type(endian) == elf::SHT_NOBITS))
            .and_then(|section| layout.as_ref()?.by_section.get(section.0)?.as_ref())
        {
            defined_address(raw, symbol, range.start + value)
                .filter(|&address| range.start <= address && address <= range.end)
        } else {
            None
        };
        if let Some(address) = address {
            targets.insert(index.0, address);
        }
    }
    Ok(targets)
}

struct Layout {
    by_section: Vec<Option<Range<u64>>>,
    external_space: Option<Range<u64>>,
}

fn verified_layout(
    file: &ElfFile32<'_>, file_size: usize, sections: &SectionMap, saved: &SavedMemory,
    image_base: u64,
) -> Option<Layout> {
    let endian = file.endian();
    let headers = file.elf_section_table();
    // ElfProgramBuilder.computeRelocationStartAddress and processSectionHeader:
    // reserve past fixed-address sections, then pack relocatable sections in
    // header order; each allocation is aligned before adding the image base.
    let mut cursor = headers.iter().filter(|header| header.sh_flags(endian) & elf::SHF_ALLOC != 0
        && header.sh_addr(endian) != 0)
        .map(|header| u64::from(header.sh_addr(endian)) + u64::from(header.sh_size(endian)))
        .max().filter(|&end| end < ADDRESS_LIMIT / 2).unwrap_or(0);
    let mut by_section = vec![None; headers.len()];
    let mut loaded = Vec::new();
    let mut backed = false;
    let mut has_nobits = false;
    for (index, header) in headers.enumerate() {
        let kind = header.sh_type(endian);
        let flags = header.sh_flags(endian);
        let len = u64::from(header.sh_size(endian));
        if kind == elf::SHT_NULL || flags & elf::SHF_ALLOC == 0 || len == 0 {
            continue;
        }
        if flags & elf::SHF_COMPRESSED != 0 {
            return None;
        }
        let fixed = u64::from(header.sh_addr(endian));
        let relative = if fixed == 0 {
            let alignment = u64::from(header.sh_addralign(endian)).max(1);
            let offset = align(cursor, alignment);
            cursor = offset + len;
            offset
        } else {
            fixed
        };
        let start = image_base + relative;
        let end = start + len;
        if end > ADDRESS_LIMIT {
            return None;
        }
        if kind != elf::SHT_NOBITS {
            let offset = header.sh_offset(endian) as usize;
            let file_end = offset.checked_add(len as usize).filter(|&end| end <= file_size)?;
            if len as usize >= file_size || sections.relocation_address(offset, offset, file_end) != Some(start) {
                return None;
            }
            backed = true;
        } else {
            has_nobits = true;
        }
        by_section[index.0] = Some(start..end);
        loaded.push(start..end);
    }
    loaded.sort_unstable_by_key(|range| range.start);
    if !backed || loaded.windows(2).any(|ranges| ranges[0].end > ranges[1].start) {
        return None;
    }
    if has_nobits {
        verify_nobits_names(file, &by_section, saved)?;
    }
    let external_space = linkage_space(loaded.into_iter());
    Some(Layout { by_section, external_space })
}

fn verify_nobits_names(
    file: &ElfFile32<'_>, layout: &[Option<Range<u64>>], saved: &SavedMemory,
) -> Option<()> {
    let endian = file.endian();
    let headers = file.elf_section_table();
    let saved_by_start: BTreeMap<_, _> = saved.uninitialized.iter().map(|block| (block.range.start, block)).collect();
    let mut counts = BTreeMap::new();
    let mut used = BTreeSet::new();
    // MemorySectionResolver resolves sections in reverse order. Whole blocks
    // keep the base name on first use and gain .2, .3, ... on repeated use.
    // Only initialized blocks enter its set of occupied chunk names.
    for (index, header) in headers.iter().enumerate().rev() {
        let Some(range) = layout[index].as_ref() else { continue };
        let raw_name = headers.section_name(endian, header).ok()?;
        if raw_name.is_empty() {
            return None; // Stock synthesizes an indexed name for this section.
        }
        let bytes = trim_elf_bytes(raw_name);
        let name = if bytes.is_empty() { "NO-NAME" } else { std::str::from_utf8(bytes).ok()? };
        let count = counts.entry(name).or_insert(0);
        *count += 1;
        let mut suffix = if *count == 1 { 0 } else { *count };
        let chunk_name = loop {
            let candidate = if suffix == 0 { Cow::Borrowed(name) } else { Cow::Owned(format!("{name}.{suffix}")) };
            if !used.contains(&candidate) {
                break candidate;
            }
            suffix += 1;
        };
        if header.sh_type(endian) == elf::SHT_NOBITS {
            let block = saved_by_start.get(&range.start)?;
            if block.artificial || block.range != *range || block.name != chunk_name {
                return None;
            }
        } else {
            used.insert(chunk_name);
        }
    }
    Some(())
}

fn linkage_space(loaded: impl Iterator<Item = Range<u64>>) -> Option<Range<u64>> {
    let mut largest = (0, 0, 0..0);
    let mut preferred = None;
    let mut start = LINKAGE_ALIGNMENT;
    // ElfProgramBuilder.allocateLinkageBlock prefers the last gap with at least
    // 128 KiB after alignment, otherwise the first largest free range.
    for occupied in loaded.chain(std::iter::once(ADDRESS_LIMIT..ADDRESS_LIMIT)) {
        if start < occupied.start {
            let length = occupied.start - start;
            let aligned = align(start, LINKAGE_ALIGNMENT);
            if length > largest.0 {
                largest = (length, start, aligned..occupied.start);
            }
            if occupied.start >= aligned + PREFERRED_EXTERNAL_SPACE {
                preferred = Some((start, aligned..occupied.start));
            }
        }
        start = start.max(occupied.end);
    }
    let range = preferred.filter(|(start, _)| *start > largest.1)
        .map_or(largest.2, |(_, range)| range);
    (!range.is_empty()).then_some(range)
}

fn external_targets(
    file: &ElfFile32<'_>, layout: &Layout, saved: &Range<u64>, image_base: u64,
) -> Option<BTreeMap<usize, u64>> {
    let free = layout.external_space.as_ref()?;
    if saved.start != free.start || saved.end > free.end {
        return None;
    }
    let endian = file.endian();
    let symbols = file.elf_symbol_table();
    let mut next = saved.start;
    let mut labels = BTreeMap::new();
    let mut allocated = BTreeMap::new();
    for (index, symbol) in symbols.enumerate().skip(1) {
        let kind = symbol.st_type();
        if matches!(kind, elf::STT_FILE | elf::STT_TLS) {
            continue;
        }
        let shndx = symbol.st_shndx(endian);
        let value = u64::from(symbol.st_value(endian));
        if shndx != elf::SHN_COMMON && (shndx != elf::SHN_UNDEF || value != 1 && (value != 0 || kind == elf::STT_SECTION)) {
            // A definition actually placed in this otherwise-free range can
            // affect Ghidra's duplicate lookup. Do not guess the lookup result.
            if symbol.st_name(endian) != 0 && kind != elf::STT_SECTION
                && ordinary_address(file, layout, index, symbol, value, image_base)
                    .is_some_and(|address| saved.contains(&address) || saved.contains(&(address & !1)))
            {
                return None;
            }
            continue;
        }
        let name = symbol_name(file, symbol)?;
        if name == "_GLOBAL_OFFSET_TABLE_" || name.chars().all(char::is_whitespace) {
            continue;
        }
        if kind == elf::STT_SECTION || name.encode_utf16().count() > 2000
            || name.bytes().any(|byte| byte <= b' ' || byte == 0x7f)
        {
            // Such a name need not produce a matching imported label.
            return None;
        }
        // A prior unversioned label wins over an exact versioned-name match.
        let prior = name.find("@@").filter(|&index| index > 0).or_else(|| name.find('@'))
            .and_then(|split| labels.get(&name[..split])).or_else(|| labels.get(name));
        if let Some(&address) = prior {
            allocated.insert(index.0, address);
            continue;
        }
        let size = u64::from(symbol.st_size(endian));
        let size = if kind == elf::STT_OBJECT && size > 0 && size < i32::MAX as u64 {
            align(size, SYMBOL_ALIGNMENT)
        } else {
            SYMBOL_ALIGNMENT
        };
        // ElfProgramBuilder.allocateExternalSymbol ignores COMMON st_value
        // alignment. It rounds a sized STT_OBJECT's size to four, and gives
        // all other symbols one four-byte slot in table order.
        if size > i32::MAX as u64 || next + size > saved.end {
            return None;
        }
        labels.insert(name, next);
        allocated.insert(index.0, next);
        next += size;
    }
    (next == saved.end).then_some(allocated)
}

fn ordinary_address(
    file: &ElfFile32<'_>, layout: &Layout, index: SymbolIndex, symbol: &elf::Sym32<object::Endianness>,
    value: u64, image_base: u64,
) -> Option<u64> {
    let endian = file.endian();
    let symbols = file.elf_symbol_table();
    match symbol.st_shndx(endian) {
        elf::SHN_ABS => Some(value),
        elf::SHN_UNDEF => Some((image_base + value) % ADDRESS_LIMIT),
        _ => {
            let SectionIndex(index) = symbols.symbol_section(endian, symbol, index).ok().flatten()?;
            let header = file.elf_section_table().section(SectionIndex(index)).ok()?;
            if header.sh_flags(endian) & elf::SHF_ALLOC == 0 {
                return None;
            }
            // Skipped empty sections still fall back to their preferred address.
            let start = layout.by_section.get(index)?.as_ref().map_or_else(
                || (image_base + u64::from(header.sh_addr(endian))) % ADDRESS_LIMIT, |range| range.start,
            );
            start.checked_add(value).filter(|&address| address < ADDRESS_LIMIT)
        }
    }
}

fn defined_address(file: &ElfFile32<'_>, symbol: &elf::Sym32<object::Endianness>, address: u64) -> Option<u64> {
    defined_symbol_address(address, symbol.st_type(), || {
        file.elf_symbol_table().symbol_name(file.endian(), symbol).ok()
    })
}

pub(super) fn defined_symbol_address<'a>(
    mut address: u64, kind: u8, name: impl FnOnce() -> Option<&'a [u8]>,
) -> Option<u64> {
    if kind == elf::STT_FUNC && address & 1 != 0 {
        let name = std::str::from_utf8(trim_elf_bytes(name()?)).ok()?;
        if !name.chars().all(char::is_whitespace) && !["$a", "$d", "$t"].iter().any(|tag| {
            name.strip_prefix(*tag).is_some_and(|suffix| suffix.is_empty() || suffix.starts_with('.'))
        }) {
            address &= !1;
        }
    }
    (address < ADDRESS_LIMIT).then_some(address)
}

fn symbol_name<'data>(file: &ElfFile32<'data>, symbol: &elf::Sym32<object::Endianness>) -> Option<&'data str> {
    let name = file.elf_symbol_table().symbol_name(file.endian(), symbol).ok()?;
    std::str::from_utf8(trim_elf_bytes(name)).ok()
}

fn trim_elf_bytes(bytes: &[u8]) -> &[u8] {
    // Ghidra reads UTF-8 and applies Java String.trim().
    let start = bytes.iter().position(|&byte| byte > b' ').unwrap_or(bytes.len());
    let end = bytes.iter().rposition(|&byte| byte > b' ').map_or(start, |end| end + 1);
    &bytes[start..end]
}

fn align(offset: u64, alignment: u64) -> u64 {
    offset.next_multiple_of(alignment)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linkage_space_prefers_only_a_later_sufficient_gap() {
        let smaller_last = [0x50000..0xfffc_0000];
        assert_eq!(linkage_space(smaller_last.into_iter()), Some(0xfffc_0000..ADDRESS_LIMIT));

        let largest_later = [0x21000..0x22001, 0x42002..0xffff_f000];
        assert_eq!(linkage_space(largest_later.into_iter()), Some(0x23000..0x42002));
    }
}
