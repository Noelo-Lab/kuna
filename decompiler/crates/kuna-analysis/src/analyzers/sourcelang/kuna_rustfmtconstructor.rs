//! Complete image evidence for the first Rust Arguments constructor profile.

use std::collections::{BTreeMap, BTreeSet};

use kuna_decomp::kuna_rustfmtconstructor::{BODY, NAME};
use kuna_decomp::kuna_rustfmtio::ImageSpan;
use object::{Object, ObjectSection, ObjectSymbol, SymbolKind};

pub(super) fn collect(
    file: &object::File<'_>,
    bytes: &[u8],
    symbols: &[(u64, Option<String>)],
) -> BTreeMap<u64, ImageSpan> {
    if file.format() != object::BinaryFormat::Elf
        || file.architecture() != object::Architecture::X86_64
        || !file.is_64()
        || !file.is_little_endian()
        || !matches!(
            file.kind(),
            object::ObjectKind::Executable | object::ObjectKind::Dynamic
        )
    {
        return BTreeMap::new();
    }
    let candidates: BTreeSet<_> = symbols
        .iter()
        .filter_map(|(address, name)| (name.as_deref() == Some(NAME)).then_some(*address))
        .collect();
    if candidates.is_empty() {
        return BTreeMap::new();
    }
    let mut definitions = Vec::new();
    for symbol in file.symbols().chain(file.dynamic_symbols()) {
        if symbol.is_definition() {
            let Some(stop) = symbol.address().checked_add(symbol.size()) else {
                return BTreeMap::new();
            };
            definitions.push((symbol.address(), stop, symbol));
        }
    }
    definitions.sort_by_key(|(start, _, _)| *start);
    let symbol_reach = prefix_reach(definitions.iter().map(|(_, stop, _)| *stop));
    let mut relocations = Vec::new();
    for section in file.sections() {
        if !matches!(section.flags(), object::SectionFlags::Elf { sh_flags }
            if sh_flags & u64::from(object::elf::SHF_ALLOC) != 0)
        {
            continue;
        }
        for (start, relocation) in section.relocations() {
            let stop = (relocation.size() != 0)
                .then(|| start.checked_add(u64::from(relocation.size()).div_ceil(8)))
                .flatten();
            let Some(stop) = stop else {
                return BTreeMap::new();
            };
            relocations.push((start, stop));
        }
    }
    relocations.sort_unstable();
    let relocation_reach = prefix_reach(relocations.iter().map(|(_, stop)| *stop));
    let image = super::kuna_rustfmtio::Image::new(file, bytes);
    candidates
        .into_iter()
        .filter_map(|address| {
            let end = address.checked_add(BODY.len() as u64)?;
            let first = definitions.partition_point(|(start, _, _)| *start < address);
            if first > 0 && symbol_reach[first - 1] > address {
                return None;
            }
            let last = definitions.partition_point(|(start, _, _)| *start < end);
            if first == last {
                return None;
            }
            for (start, _, symbol) in &definitions[first..last] {
                if *start != address
                    || symbol.kind() != SymbolKind::Text
                    || symbol.size() != BODY.len() as u64
                    || symbol
                        .name()
                        .ok()
                        .and_then(|name| rustc_demangle::try_demangle(name).ok())
                        .map(|name| format!("{name:#}"))
                        .as_deref()
                        != Some(NAME)
                {
                    return None;
                }
            }
            let before = relocations.partition_point(|(start, _)| *start < end);
            if before > 0 && relocation_reach[before - 1] > address {
                return None;
            }
            let span = image.unrelocated_code(address, BODY.len())?;
            (span.bytes == BODY).then_some((address, span))
        })
        .collect()
}

/// Running maximum of interval ends, in start order.
fn prefix_reach(stops: impl Iterator<Item = u64>) -> Vec<u64> {
    stops
        .scan(0u64, |reach, stop| {
            *reach = (*reach).max(stop);
            Some(*reach)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use object::ObjectSegment;

    const NATIVE: &[u8] = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/rust_fmt_constructor_route_x86_64.elfdata"
    ));

    fn profiles(bytes: &[u8]) -> BTreeMap<u64, ImageSpan> {
        let Ok(file) = object::File::parse(bytes) else {
            return BTreeMap::new();
        };
        let symbols = file
            .symbols()
            .chain(file.dynamic_symbols())
            .filter(|symbol| symbol.is_definition() && symbol.kind() == SymbolKind::Text)
            .map(|symbol| {
                (
                    symbol.address(),
                    symbol
                        .name()
                        .ok()
                        .and_then(|name| rustc_demangle::try_demangle(name).ok())
                        .map(|name| format!("{name:#}")),
                )
            })
            .collect::<Vec<_>>();
        collect(&file, bytes, &symbols)
    }

    fn profile() -> ImageSpan {
        let mut found = profiles(NATIVE);
        assert_eq!(found.len(), 1);
        found.pop_first().unwrap().1
    }

    fn offset(address: u64) -> usize {
        object::File::parse(NATIVE)
            .unwrap()
            .segments()
            .find_map(|segment| {
                let delta = address.checked_sub(segment.address())?;
                let (offset, size) = segment.file_range();
                (delta < size).then_some((offset + delta) as usize)
            })
            .unwrap()
    }

    fn u16_at(bytes: &[u8], offset: usize) -> usize {
        u16::from_le_bytes(bytes[offset..offset + 2].try_into().unwrap()) as usize
    }

    fn u64_at(bytes: &[u8], offset: usize) -> u64 {
        u64::from_le_bytes(bytes[offset..offset + 8].try_into().unwrap())
    }

    fn symbol_record(name: &str) -> usize {
        let file = object::File::parse(NATIVE).unwrap();
        let symbol = file
            .symbols()
            .find(|symbol| {
                symbol
                    .name()
                    .ok()
                    .and_then(|name| rustc_demangle::try_demangle(name).ok())
                    .map(|name| format!("{name:#}"))
                    .as_deref()
                    == Some(name)
                    || symbol.name().ok() == Some(name)
            })
            .unwrap();
        let shoff = u64_at(NATIVE, 40) as usize;
        let shsize = u16_at(NATIVE, 58);
        let header = (0..u16_at(NATIVE, 60))
            .map(|i| shoff + i * shsize)
            .find(|offset| NATIVE[offset + 4..offset + 8] == 2u32.to_le_bytes())
            .unwrap();
        u64_at(NATIVE, header + 24) as usize
            + symbol.index().0 * u64_at(NATIVE, header + 56) as usize
    }

    #[test]
    fn constructor_profile_requires_complete_exact_native_body() {
        let profile = profile();
        assert_eq!(profile.bytes, BODY);
        let start = offset(profile.address);
        for index in 0..BODY.len() {
            let mut changed = NATIVE.to_vec();
            changed[start + index] ^= 1;
            assert!(profiles(&changed).is_empty(), "changed byte {index}");
        }
    }

    #[test]
    fn constructor_profile_rejects_extra_tail_and_conflicting_alias() {
        let original = symbol_record(NAME);
        for size in [0u64, 16, 18] {
            let mut changed = NATIVE.to_vec();
            changed[original + 16..original + 24].copy_from_slice(&size.to_le_bytes());
            assert!(profiles(&changed).is_empty(), "size {size}");
        }
        let mut changed = NATIVE.to_vec();
        let alias = symbol_record("r13_constructor_stdout");
        changed[alias + 8..alias + 16].copy_from_slice(&profile().address.to_le_bytes());
        changed[alias + 16..alias + 24].copy_from_slice(&(BODY.len() as u64).to_le_bytes());
        assert!(profiles(&changed).is_empty());
        let mut covering = NATIVE.to_vec();
        covering[alias + 8..alias + 16].copy_from_slice(&(profile().address - 1).to_le_bytes());
        covering[alias + 16..alias + 24].copy_from_slice(&2u64.to_le_bytes());
        assert!(profiles(&covering).is_empty());
        covering[alias + 16..alias + 24].copy_from_slice(&1u64.to_le_bytes());
        assert_eq!(profiles(&covering).len(), 1);
    }

    #[test]
    fn constructor_profile_rejects_mutable_code_and_foreign_architecture() {
        let address = profile().address;
        let phoff = u64_at(NATIVE, 32) as usize;
        let phsize = u16_at(NATIVE, 54);
        let mut changed = NATIVE.to_vec();
        for header in (0..u16_at(NATIVE, 56)).map(|i| phoff + i * phsize) {
            let start = u64_at(NATIVE, header + 16);
            let size = u64_at(NATIVE, header + 40);
            if start <= address && address < start + size {
                changed[header + 4] |= object::elf::PF_W as u8;
            }
        }
        assert!(profiles(&changed).is_empty());
        let mut foreign = NATIVE.to_vec();
        foreign[18..20].copy_from_slice(&object::elf::EM_AARCH64.to_le_bytes());
        assert!(profiles(&foreign).is_empty());
    }

    #[test]
    fn constructor_profile_rejects_code_relocation() {
        let file = object::File::parse(NATIVE).unwrap();
        let section = file.section_by_name(".rela.dyn").unwrap();
        let (offset, size) = section.file_range().unwrap();
        assert!(size >= 24);
        let mut changed = NATIVE.to_vec();
        changed[offset as usize..offset as usize + 8]
            .copy_from_slice(&profile().address.to_le_bytes());
        assert!(profiles(&changed).is_empty());
    }

    #[test]
    fn constructor_profile_rejects_retained_relocation_absolute_address() {
        let file = object::File::parse(NATIVE).unwrap();
        let metadata = file.section_by_name(".debug_aranges").unwrap();
        let text = file.section_by_name(".text").unwrap();
        let symtab = file.section_by_name(".symtab").unwrap();
        let (offset, size) = metadata.file_range().unwrap();
        assert!(size >= 24);
        let header = u64_at(NATIVE, 40) as usize + metadata.index().0 * u16_at(NATIVE, 58);
        let address = profile().address;
        let mut changed = NATIVE.to_vec();
        changed[header + 4..header + 8].copy_from_slice(&object::elf::SHT_RELA.to_le_bytes());
        changed[header + 8..header + 24].fill(0);
        changed[header + 32..header + 40].copy_from_slice(&24u64.to_le_bytes());
        changed[header + 40..header + 44].copy_from_slice(&(symtab.index().0 as u32).to_le_bytes());
        changed[header + 44..header + 48].copy_from_slice(&(text.index().0 as u32).to_le_bytes());
        changed[header + 48..header + 56].copy_from_slice(&8u64.to_le_bytes());
        changed[header + 56..header + 64].copy_from_slice(&24u64.to_le_bytes());
        let offset = offset as usize;
        changed[offset..offset + 8].copy_from_slice(&address.to_le_bytes());
        changed[offset + 8..offset + 16]
            .copy_from_slice(&u64::from(object::elf::R_X86_64_64).to_le_bytes());
        changed[offset + 16..offset + 24].fill(0);
        let altered = object::File::parse(changed.as_slice()).unwrap();
        let target = altered.section_by_name(".text").unwrap();
        assert!(target
            .relocations()
            .any(|(offset, relocation)| offset == address && relocation.size() == 64));
        assert!(profiles(&changed).is_empty());
    }
}
