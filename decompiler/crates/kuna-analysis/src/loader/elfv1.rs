//! Validated PowerPC64 ELFv1 function descriptors and per-entry TOC values.
use object::{Architecture, BinaryFormat, FileFlags, Object, ObjectKind, ObjectSection, ObjectSymbol, RelocationFlags, RelocationTarget, SectionFlags, SymbolKind};
use std::collections::{BTreeMap, HashMap};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Descriptor {
    pub address: u64,
    pub entry: u64,
    pub toc: Option<u64>,
}

#[derive(Clone, Debug, Default)]
pub struct Descriptors(pub Vec<Descriptor>);

impl Descriptors {
    /// Only an ABI-v1/unspecified linked ELF with a file-backed .opd qualifies.
    pub fn read(file: &object::File<'_>) -> Self {
        if file.format() != BinaryFormat::Elf || file.architecture() != Architecture::PowerPc64
            || !matches!(file.kind(), ObjectKind::Executable | ObjectKind::Dynamic)
            || !matches!(file.flags(), FileFlags::Elf { e_flags, .. } if e_flags & 3 <= 1)
        { return Self::default(); }
        let Some(opd) = file.section_by_name(".opd") else { return Self::default(); };
        let Ok(bytes) = opd.data() else { return Self::default(); };
        let mut addresses = vec![file.entry()];
        addresses.extend(file.symbols().chain(file.dynamic_symbols())
            .filter(|s| s.kind() == SymbolKind::Text && !s.is_undefined() && s.section_index() == Some(opd.index()))
            .map(|s| s.address()));
        addresses.sort_unstable();addresses.dedup();
        let Some(opd_end) = opd.address().checked_add(bytes.len() as u64) else { return Self::default(); };
        let relocations = descriptor_relocations(file, opd.address(), opd_end);
        let mut out = Vec::new();
        for address in addresses {
            if address & 7 != 0 { continue; }
            let Some(offset) = address.checked_sub(opd.address()).and_then(|n| usize::try_from(n).ok()) else { continue; };
            let Some(end) = offset.checked_add(24) else { continue; };
            let Some(record) = bytes.get(offset..end) else { continue; };
            let word = |start| {
                let b: [u8;8] = record[start..start+8].try_into().unwrap();
                let mut value = if file.is_little_endian() { u64::from_le_bytes(b) } else { u64::from_be_bytes(b) };
                let begin = address.checked_add(start as u64)?;
                let end = begin.checked_add(8)?;
                for (&slot, &(width, resolved)) in relocations.range(begin.saturating_sub(23)..end) {
                    if slot.checked_add(width)? <= begin { continue; }
                    if slot != begin || width != 8 { return None; }
                    value = resolved?;
                }
                Some(value)
            };
            let Some(entry) = word(0) else { continue; };
            let toc = word(8);
            let mapped_code = entry & 3 == 0 && file.sections().any(|s| {
                if !matches!(s.flags(), SectionFlags::Elf { sh_flags } if sh_flags & 6 == 6) { return false; }
                let Some(n) = entry.checked_sub(s.address()).and_then(|n| usize::try_from(n).ok()) else { return false; };
                s.data().ok().is_some_and(|data| n.checked_add(4).is_some_and(|end| end <= data.len()))
            });
            if mapped_code { out.push(Descriptor { address, entry, toc }); }
        }
        Self(out)
    }

    pub fn code_address(&self, address: u64) -> u64 {
        self.0.binary_search_by_key(&address, |d| d.address).ok().map_or(address, |i| self.0[i].entry)
    }

    /// Aliases may share code; conflicting or unresolved TOCs cannot safely seed that entry.
    pub fn entry_tocs(&self) -> BTreeMap<u64,u64> {
        let mut values: HashMap<u64,Option<u64>> = HashMap::new();
        for d in &self.0 {
            values.entry(d.entry).and_modify(|old| { if *old != d.toc { *old = None; } }).or_insert(d.toc);
        }
        values.into_iter().filter_map(|(entry,toc)| toc.map(|toc| (entry,toc))).collect()
    }

    pub fn adjust_vma(&mut self, delta: u64) {
        for d in &mut self.0 {
            d.address = d.address.wrapping_add(delta);
            d.entry = d.entry.wrapping_add(delta);
            d.toc = d.toc.map(|toc| toc.wrapping_add(delta));
        }
        self.0.sort_by_key(|d| d.address);
    }
}

/// Resolve RELA relative words at kuna's initial zero load bias; uncertain writes invalidate affected words.
fn descriptor_relocations(file: &object::File<'_>, begin: u64, end: u64) -> BTreeMap<u64, (u64, Option<u64>)> {
    use object::elf;
    let mut values: BTreeMap<u64, (u64, Option<u64>)> = BTreeMap::new();
    if let Some(relocations) = file.dynamic_relocations() {
        for (slot, relocation) in relocations {
            let RelocationFlags::Elf { r_type } = relocation.flags() else { continue; };
            if r_type == elf::R_PPC64_NONE { continue; }
            let width = match r_type {
                elf::R_PPC64_RELATIVE | elf::R_PPC64_ADDR64 | elf::R_PPC64_UADDR64
                | elf::R_PPC64_GLOB_DAT | elf::R_PPC64_TOC => 8,
                _ => match relocation.size() {
                    1..=64 => u64::from(relocation.size()).div_ceil(8),
                    _ => 24,
                },
            };
            if slot >= end || slot.checked_add(width).is_some_and(|stop| stop <= begin) { continue; }
            let value = (r_type == elf::R_PPC64_RELATIVE && !relocation.has_implicit_addend()
                && matches!(relocation.target(), RelocationTarget::Absolute))
                .then_some(relocation.addend() as u64);
            values.entry(slot).and_modify(|old| { old.0 = old.0.max(width); old.1 = None; })
                .or_insert((width, value));
        }
    }
    values
}

#[cfg(test)]
#[path = "elfv1_tests.rs"]
mod tests;
