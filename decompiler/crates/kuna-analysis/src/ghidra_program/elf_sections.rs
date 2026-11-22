use std::collections::BTreeMap;
use std::ops::Range;

use object::{
    Architecture, Object, ObjectSection, ObjectSymbol, Relocation, RelocationFlags, RelocationTarget, SectionFlags,
    SymbolFlags, elf,
};

use super::file_bytes::{SavedFileRange, SavedMemory};
use super::*;

#[derive(Debug)]
pub(super) struct SectionMap {
    by_project: Vec<SectionSpan>,
    by_file: Vec<usize>,
    arm: bool,
}

#[derive(Debug)]
struct SectionSpan {
    project: Range<u64>,
    file: Range<usize>,
    section_index: usize,
    section_offset: u64,
    executable: bool,
}

struct FileSection {
    file: Range<usize>,
    index: usize,
    executable: bool,
}

#[derive(Debug)]
pub(super) struct CodePatch {
    pub(super) offset: usize,
    pub(super) bytes: [u8; 4],
    pub(super) width: usize,
}

impl CodePatch {
    pub(super) fn bytes(&self) -> &[u8] {
        &self.bytes[..self.width]
    }

    pub(super) fn end(&self) -> usize {
        self.offset + self.width
    }
}

struct Candidate {
    patch: CodePatch,
    kind: u32,
    symbol: usize,
    status: Option<i8>,
}

impl Candidate {
    fn verified_patch(self, address: u64, original: &[u8]) -> KunaResult<Option<CodePatch>> {
        // Bits 0–2 store APPLIED=5; the next four store the exact written width.
        const APPLIED: i8 = 5;
        if self.status == Some((self.patch.width as i8) << 3 | APPLIED) {
            return Ok(Some(self.patch));
        }
        if self.status.is_some_and(|status| status & 7 == APPLIED)
            || &original[self.patch.offset..self.patch.end()] != self.patch.bytes()
        {
            return Err(bad(format_args!("missing or invalid applied ELF executable relocation at {address:#x}")));
        }
        Ok(None)
    }
}

impl SectionMap {
    pub(super) fn read(
        original: &object::File<'_>, size: usize, max_loaded_size: usize, saved: &[SavedFileRange],
    ) -> KunaResult<(Self, Vec<Range<usize>>)> {
        let mut sections = Vec::new();
        let mut loaded_size = 0u64;
        for section in original.sections() {
            let SectionFlags::Elf { sh_flags } = section.flags() else { continue };
            if sh_flags & u64::from(elf::SHF_ALLOC) == 0 {
                continue;
            }
            loaded_size = loaded_size.checked_add(section.size()).filter(|&end| end <= max_loaded_size as u64)
                .ok_or_else(|| bad(format_args!("allocated ELF sections exceed the {max_loaded_size}-byte limit")))?;
            let Some((offset, len)) = section.file_range().filter(|&(_, len)| len != 0) else { continue };
            let index = section.index().0;
            let end = offset.checked_add(len).filter(|&end| end <= size as u64)
                .ok_or_else(|| bad(format_args!("ELF section {index} extends beyond original File Bytes")))?;
            if sh_flags & u64::from(elf::SHF_COMPRESSED) != 0 {
                return Err(bad(format_args!("unsupported compressed allocated ELF section {index}")));
            }
            let executable = sh_flags & u64::from(elf::SHF_EXECINSTR) != 0;
            sections.push(FileSection { file: offset as usize..end as usize, index, executable });
        }
        sort_sections(&mut sections)?;
        // Verify all original executable bytes, including any unmapped in the saved project.
        let code = sections.iter().filter(|section| section.executable)
            .map(|section| section.file.clone()).collect();
        let map = Self::from_saved(&sections, saved, original.architecture() == Architecture::Arm)?;
        Ok((map, code))
    }

    fn from_saved(sections: &[FileSection], saved: &[SavedFileRange], arm: bool) -> KunaResult<Self> {
        let mut by_project = Vec::new();
        for block in saved {
            let first = sections.partition_point(|section| section.file.end <= block.file.start);
            if let Some(section) = sections.get(first).filter(|section| {
                section.file.start <= block.file.start && block.file.end <= section.file.end
                    && (!block.executable || section.executable)
            }) {
                by_project.push(SectionSpan {
                    project: block.project.clone(),
                    file: block.file.clone(),
                    section_index: section.index,
                    section_offset: (block.file.start - section.file.start) as u64,
                    executable: section.executable,
                });
            } else if block.executable
                || sections.get(first).is_some_and(|section| section.file.start < block.file.end)
            {
                return Err(bad(format_args!(
                    "saved sub-block at {:#x} has no single matching original allocated ELF section backing \
                     (file bytes {:#x}..{:#x})", block.project.start, block.file.start, block.file.end,
                )));
            }
        }
        by_project.sort_unstable_by_key(|span| span.project.start);
        let mut by_file: Vec<_> = (0..by_project.len()).collect();
        by_file.sort_unstable_by_key(|&index| by_project[index].file.start);
        if by_file.windows(2).any(|pair| by_project[pair[0]].file.end > by_project[pair[1]].file.start) {
            return Err(bad("saved memory mappings alias original ELF section backing"));
        }
        Ok(Self { by_project, by_file, arm })
    }

    pub(super) fn entry(&self, address: u64) -> Option<ProgramEntry> {
        let index = self.by_project.partition_point(|span| span.project.end <= address);
        let span = self.by_project.get(index).filter(|span| span.executable && span.project.contains(&address))?;
        let offset = span.section_offset + (address - span.project.start);
        if self.arm && !offset.is_multiple_of(2) {
            return None;
        }
        Some(ProgramEntry::ElfSection { section_index: span.section_index, offset })
    }

    fn file_span(&self, file: Range<usize>) -> Option<&SectionSpan> {
        let index = self.by_file.partition_point(|&index| self.by_project[index].file.end <= file.start);
        let span = &self.by_project[*self.by_file.get(index)?];
        (span.file.start <= file.start && file.end <= span.file.end).then_some(span)
    }

    pub(super) fn relocation_address(&self, base: usize, start: usize, end: usize) -> Option<u64> {
        let head = self.file_span(base..base + 1)?;
        let span = self.file_span(start..end)?;
        let section_base = head.project.start + (base - head.file.start) as u64;
        let place = span.project.start + (start - span.file.start) as u64;
        // Stock ELF obtains the relocation base from the saved mapping of byte zero.
        // Do not assume that a separately moved fragment still has that placement.
        (head.section_index == span.section_index
            && section_base.checked_add((start - base) as u64) == Some(place)).then_some(place)
    }

    #[expect(clippy::too_many_arguments, reason = "cross-checks saved relocation records and memory against the original ELF")]
    pub(super) fn relocations(
        &self, db: &Database<'_>, map: &AddressMap, used: &mut HashSet<i32>,
        file: &object::File<'_>, original: &[u8], metadata: &Metadata,
        saved: &SavedMemory,
    ) -> KunaResult<Vec<CodePatch>> {
        let language = metadata.language_id.as_str();
        let x86 = file.architecture() == Architecture::X86_64 && file.is_little_endian()
            && matches!(language, "x86:LE:64:default" | "x86:LE:64:compat32");
        let arm = self.arm && file.is_little_endian() && language.starts_with("ARM:LE:32:");
        if !x86 && !arm {
            return Ok(Vec::new());
        }
        let pc_bias = arm && file.section_by_name(".ghsinfo").is_some();
        let targets = if arm {
            super::elf_arm_targets::targets(file, original, self, saved, metadata.image_base)?
        } else {
            BTreeMap::new()
        };
        let mut candidates = BTreeMap::new();
        for section in file.sections() {
            if !matches!(section.flags(), SectionFlags::Elf { sh_flags }
                if sh_flags & u64::from(elf::SHF_ALLOC | elf::SHF_EXECINSTR) == u64::from(elf::SHF_ALLOC | elf::SHF_EXECINSTR))
            {
                continue;
            }
            let Some((base, len)) = section.file_range() else { continue };
            for (offset, reloc) in section.relocations() {
                let Some((place, candidate)) = self.candidate(file, original, x86, pc_bias, &targets, (base, len), offset, &reloc)
                else {
                    continue;
                };
                if candidates.insert(place, candidate).is_some() {
                    return Err(bad("ambiguous original ELF executable relocations at the same address"));
                }
            }
        }
        if candidates.is_empty() {
            return Ok(Vec::new());
        }
        let table = require_table(db, "Relocations", 6, (Long, "Index"), &[
            (Long, "Address"), (Byte, "Status"), (Int, "Type"), (Binary, "Values"),
            (Binary, "Bytes"), (Str, "Symbol Name"),
        ])?;
        db.visit_records(table, used, |record| {
            let [FieldValue::Long(key), FieldValue::Byte(flags), FieldValue::Int(kind),
                FieldValue::Binary(values), FieldValue::Binary(bytes), FieldValue::Str(_)] = record.columns.as_slice()
            else {
                return Err(bad("invalid Relocations record"));
            };
            let Some(address) = map.decode(*key)? else { return Ok(()) };
            let Some(candidate) = candidates.get_mut(&address) else { return Ok(()) };
            // Ghidra's stock ELF importer stores the symbol index in a single
            // BinaryCodedField long and uses the underlying original FileBytes.
            if candidate.status.replace(*flags).is_some() || *kind as u32 != candidate.kind
                || values.as_deref().and_then(saved_symbol_index) != Some(candidate.symbol) || bytes.is_some()
            {
                return Err(bad(format_args!("unverified saved ELF executable relocation at {address:#x}")));
            }
            Ok(())
        })?;
        let mut patches = Vec::new();
        for (address, candidate) in candidates {
            if let Some(patch) = candidate.verified_patch(address, original)? {
                patches.push(patch);
            }
        }
        patches.sort_unstable_by_key(|patch| patch.offset);
        if patches.windows(2).any(|pair| pair[0].end() > pair[1].offset) {
            return Err(bad("overlapping verified ELF executable relocation fields"));
        }
        Ok(patches)
    }

    #[expect(clippy::too_many_arguments, reason = "validate one original relocation against both saved section mappings")]
    fn candidate(
        &self, file: &object::File<'_>, original: &[u8], x86: bool, pc_bias: bool,
        targets: &BTreeMap<usize, u64>,
        (base, len): (u64, u64), offset: u64, reloc: &Relocation,
    ) -> Option<(u64, Candidate)> {
        let RelocationFlags::Elf { r_type: kind } = reloc.flags() else { return None };
        let width = if x86 {
            if !matches!(kind, elf::R_X86_64_PC32 | elf::R_X86_64_PLT32) || reloc.has_implicit_addend() {
                return None;
            }
            4
        } else {
            super::elf_relocation_arm::width(kind)?
        };
        let RelocationTarget::Symbol(index) = reloc.target() else { return None };
        if offset.checked_add(width as u64)? > len {
            return None;
        }
        let base = usize::try_from(base).ok()?;
        let start = base.checked_add(usize::try_from(offset).ok()?)?;
        let end = start.checked_add(width)?;
        let place = self.relocation_address(base, start, end)?;
        let symbol = targets.get(&index.0).copied()
            .or_else(|| self.file_symbol_address(file, index))?;
        let bytes = if x86 {
            // Ghidra 12.1.2 X86_64_ElfRelocationHandler uses S + A - P,
            // stored modulo 2^32, for both explicit-addend PC32 and PLT32.
            (symbol.wrapping_add_signed(reloc.addend()).wrapping_sub(place) as u32).to_le_bytes()
        } else {
            super::elf_relocation_arm::encode(reloc, original.get(start..end)?, symbol, place, pc_bias)?
        };
        Some((place, Candidate { patch: CodePatch { offset: start, bytes, width }, kind, symbol: index.0, status: None }))
    }

    fn file_symbol_address(&self, file: &object::File<'_>, index: object::SymbolIndex) -> Option<u64> {
        let symbol = file.symbol_by_index(index).ok()?;
        let target = file.section_by_index(symbol.section_index()?).ok()?;
        if !matches!(target.flags(), SectionFlags::Elf { sh_flags } if sh_flags & u64::from(elf::SHF_ALLOC) != 0) {
            return None;
        }
        let (target_file, target_len) = target.file_range()?;
        let offset = symbol.address();
        if offset > target_len || target_len == 0 {
            return None;
        }
        let backed_offset = offset.min(target_len - 1);
        let base = usize::try_from(target_file).ok()?;
        let backed = usize::try_from(target_file.checked_add(backed_offset)?).ok()?;
        let address = self.relocation_address(base, backed, backed.checked_add(1)?)?
            .checked_add(offset - backed_offset)?;
        if self.arm {
            let SymbolFlags::Elf { st_info, .. } = symbol.flags() else { return None };
            let kind = st_info & 0xf;
            if matches!(kind, elf::STT_FILE | elf::STT_TLS) {
                return None;
            }
            super::elf_arm_targets::defined_symbol_address(address, kind, || symbol.name_bytes().ok())
        } else {
            Some(address)
        }
    }
}

fn saved_symbol_index(bytes: &[u8]) -> Option<usize> {
    let index = i64::from_be_bytes(bytes.strip_prefix(&[5, 0])?.try_into().ok()?);
    usize::try_from(index).ok()
}

fn sort_sections(sections: &mut [FileSection]) -> KunaResult<()> {
    sections.sort_unstable_by_key(|section| section.file.start);
    if sections.windows(2).any(|pair| pair[0].file.end > pair[1].file.start) {
        return Err(bad("ambiguous original ELF section backing: allocated sections overlap in the file"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn saved(project: u64, file: Range<usize>, executable: bool) -> SavedFileRange {
        SavedFileRange { project: project..project + file.len() as u64, file, executable }
    }

    #[test]
    fn replay_requires_matching_status_and_width_for_a_changed_field() {
        let candidate = |status| Candidate {
            patch: CodePatch { offset: 1, bytes: [3, 4, 0, 0], width: 2 },
            kind: elf::R_ARM_THM_PC8, symbol: 2, status,
        };
        let original = [9, 1, 2, 8];
        for status in [None, Some(4), Some((4 << 3) | 5)] {
            assert!(candidate(status).verified_patch(0x1000, &original).is_err());
        }
        let patch = candidate(Some((2 << 3) | 5)).verified_patch(0x1000, &original).unwrap().unwrap();
        assert_eq!(patch.bytes(), &[3, 4]);
        assert!(candidate(None).verified_patch(0x1000, &[9, 3, 4, 8]).unwrap().is_none());
    }

    #[test]
    fn elf_symbol_replay_preserves_raw_type_and_bounds_loaded_bytes() {
        use object::write::{Object as Writer, Symbol, SymbolSection};
        use object::{BinaryFormat, Endianness, SectionKind, SymbolKind, SymbolScope};

        let mut writer = Writer::new(BinaryFormat::Elf, Architecture::Arm, Endianness::Little);
        let text = writer.add_section(Vec::new(), b".text".to_vec(), SectionKind::Text);
        writer.append_section_data(text, &[0; 8], 4);
        let bss = writer.add_section(Vec::new(), b".bss".to_vec(), SectionKind::UninitializedData);
        writer.append_section_bss(bss, 64, 4);
        for (name, value, kind) in [
            ("thumb", 3, elf::STT_FUNC), ("", 3, elf::STT_FUNC),
            ("ifunc", 3, elf::STT_GNU_IFUNC), ("end", 8, elf::STT_NOTYPE),
        ] {
            writer.add_symbol(Symbol {
                name: name.as_bytes().to_vec(), value, size: 0,
                kind: SymbolKind::Text, scope: SymbolScope::Linkage, weak: false,
                section: SymbolSection::Section(text),
                flags: SymbolFlags::Elf { st_info: (elf::STB_GLOBAL << 4) | kind, st_other: 0 },
            });
        }
        let data = writer.write().unwrap();
        let file = object::File::parse(&*data).unwrap();
        let (offset, len) = file.section_by_name(".text").unwrap().file_range().unwrap();
        let saved = [saved(0x10000, offset as usize..(offset + len) as usize, true)];
        let (map, _) = SectionMap::read(&file, data.len(), 72, &saved).unwrap();
        let resolved: BTreeMap<_, _> = file.symbols().filter(|sym| sym.address() != 0).map(|sym| {
            (sym.name().unwrap(), map.file_symbol_address(&file, sym.index()).unwrap())
        }).collect();
        assert_eq!(resolved, BTreeMap::from([("thumb", 0x10002), ("", 0x10003), ("ifunc", 0x10003), ("end", 0x10008)]));
        assert!(matches!(SectionMap::read(&file, data.len(), 71, &saved), Err(error)
            if error.explain().contains("allocated ELF sections exceed")));
    }

    #[test]
    fn elf_sections_use_independent_section_placements_and_half_open_ranges() {
        let sections = [
            FileSection { file: 0x10..0x20, index: 4, executable: true },
            FileSection { file: 0x30..0x40, index: 2, executable: true },
            FileSection { file: 0x40..0x50, index: 3, executable: false },
        ];
        let blocks = [
            saved(0xa000, 0x10..0x18, true),
            saved(0xb100, 0x18..0x20, true),
            saved(0x9000, 0x30..0x40, false),
            saved(0xc000, 0x40..0x50, false),
        ];
        let map = SectionMap::from_saved(&sections, &blocks, false).unwrap();
        assert_eq!(map.entry(0xa000), Some(ProgramEntry::ElfSection { section_index: 4, offset: 0 }));
        assert_eq!(map.entry(0x9000), Some(ProgramEntry::ElfSection { section_index: 2, offset: 0 }));
        assert_eq!(map.entry(0xb100), Some(ProgramEntry::ElfSection { section_index: 4, offset: 8 }));
        assert_eq!(map.entry(0xb107), Some(ProgramEntry::ElfSection { section_index: 4, offset: 15 }));
        for address in [0x8fff, 0x9010, 0xa008, 0xb108, 0xc000] {
            assert_eq!(map.entry(address), None);
        }
        assert_eq!(map.relocation_address(0x10, 0x14, 0x18), Some(0xa004));
        assert_eq!(map.relocation_address(0x10, 0x16, 0x1a), None);
        assert_eq!(map.relocation_address(0x10, 0x18, 0x1c), None);
        let arm = SectionMap::from_saved(&sections, &blocks, true).unwrap();
        assert_eq!(arm.entry(0xb107), None);
        assert_eq!(arm.entry(0xb106), Some(ProgramEntry::ElfSection { section_index: 4, offset: 14 }));
    }

    #[test]
    fn elf_sections_reject_ambiguous_original_and_aliased_project_backing() {
        let mut sections = [
            FileSection { file: 0x10..0x20, index: 4, executable: true },
            FileSection { file: 0x1f..0x30, index: 2, executable: false },
        ];
        assert!(sort_sections(&mut sections).unwrap_err().explain().contains("ambiguous original ELF section backing"));
        sections[1].file.start = 0x20;
        sort_sections(&mut sections).unwrap();
        let alias = [saved(0x8000, 0x10..0x20, true), saved(0x9000, 0x12..0x13, false)];
        assert!(SectionMap::from_saved(&sections, &alias, false).unwrap_err().explain().contains("alias original ELF"));
        let crossing = [saved(0x8000, 0x1c..0x24, true)];
        assert!(SectionMap::from_saved(&sections, &crossing, false).is_err());
        let mislabeled_data = [saved(0x8000, 0x20..0x24, true)];
        assert!(SectionMap::from_saved(&sections, &mislabeled_data, false).is_err());
    }
}
