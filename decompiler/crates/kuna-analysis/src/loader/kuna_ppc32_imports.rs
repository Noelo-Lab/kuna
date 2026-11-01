//! PPC32 relocation slots and absolute import stubs.
//! r30-relative stubs require caller register evidence: DT_PPC_GOT describes
//! a table, not r30, even when the linker has discarded an empty .got2.
use super::elf_plt::PltSym;
use object::{
    Architecture, Object, ObjectKind, ObjectSection, ObjectSymbol, RelocationFlags,
    RelocationTarget, SectionFlags, SectionKind, SymbolKind,
};
use std::collections::{BTreeMap, HashSet};

/// Relocation slots for caller-local PIC recovery. These are data, not executable
/// function symbols; no r30-relative stub receives a global identity.
pub fn pic_slots(file: &object::File<'_>) -> Vec<kuna_decomp::kuna_ppc32piccalls::ImportSlot> {
    let (bss, slots) = slots(file);
    if bss { return Vec::new(); }
    slots.into_iter().filter_map(|(address, name)| {
        let name = String::from_utf8(name).ok()?;
        Some(kuna_decomp::kuna_ppc32piccalls::ImportSlot {
            address, no_return: super::noreturn::elf_name_is_noreturn(&name), name,
        })
    }).collect()
}

fn slots(file: &object::File<'_>) -> (bool, BTreeMap<u64, Vec<u8>>) {
    if file.architecture() != Architecture::PowerPc
        || !matches!(file.kind(), ObjectKind::Executable | ObjectKind::Dynamic)
    {
        return (false, BTreeMap::new());
    }
    let Some(plt) = file.section_by_name(".plt") else {
        return (false, BTreeMap::new());
    };
    let SectionFlags::Elf { sh_flags } = plt.flags() else {
        return (false, BTreeMap::new());
    };
    if sh_flags & 3 != 3 {
        return (false, BTreeMap::new());
    }
    let bss = plt.kind() == SectionKind::UninitializedData && sh_flags & 4 != 0;
    if !bss && sh_flags & 4 != 0 {
        return (false, BTreeMap::new());
    }
    let names: BTreeMap<_, _> = file
        .dynamic_symbols()
        .filter_map(|s| {
            if !matches!(s.kind(), SymbolKind::Text | SymbolKind::Unknown) {
                return None;
            }
            s.name_bytes()
                .ok()
                .filter(|n| !n.is_empty())
                .map(|n| (s.index().0, super::elf_plt::strip_version(n)))
        })
        .collect();
    let mut slots = BTreeMap::new();
    let mut seen = HashSet::new();
    let mut duplicate = HashSet::new();
    if let Some(relocations) = file.dynamic_relocations() {
        for (slot, reloc) in relocations {
            if !seen.insert(slot) {
                duplicate.insert(slot);
            }
            if !matches!(reloc.flags(), RelocationFlags::Elf { r_type } if r_type == object::elf::R_PPC_JMP_SLOT)
                || reloc.has_implicit_addend()
                || reloc.addend() != 0
                || slot > u32::MAX as u64
                || slot & 3 != 0
            {
                continue;
            }
            let Some(off) = slot.checked_sub(plt.address()) else {
                continue;
            };
            if (bss && off < 72)
                || off
                    .checked_add(if bss { 8 } else { 4 })
                    .is_none_or(|end| end > plt.size())
            {
                continue;
            }
            let RelocationTarget::Symbol(index) = reloc.target() else {
                continue;
            };
            if let Some(name) = names.get(&index.0) {
                slots.insert(slot, name.clone());
            }
        }
    }
    slots.retain(|slot, _| !duplicate.contains(slot));
    (bss, slots)
}

/// Resolve global PIC stub identities under the compiler and ABI assumptions
/// documented for `ppc32picimports abi`. This is not the caller-local `on` mode.
pub fn resolve_with_pic_abi(file: &object::File<'_>) -> Vec<super::format::ImportSym> {
    resolve_impl(file, true).into_iter().map(|s| super::format::ImportSym {
        addr: s.addr, name: s.name, kind: super::format::ImportSymKind::Import,
    }).collect()
}

pub(super) fn resolve(file: &object::File<'_>) -> Vec<PltSym> {
    resolve_impl(file, kuna_decomp::kuna_ppc32picimports::abi_enabled())
}

fn resolve_impl(file: &object::File<'_>, prove_pic: bool) -> Vec<PltSym> {
    let (bss, slots) = slots(file);
    if bss {
        return slots
            .into_iter()
            .map(|(addr, name)| PltSym { addr, name, slot: None })
            .collect();
    }
    if slots.is_empty() {
        return Vec::new();
    }
    let mut out = Vec::new();
    for section in file.sections() {
        if !matches!(section.flags(), SectionFlags::Elf { sh_flags } if sh_flags & 6 == 6)
            || section.address() & 3 != 0
        {
            continue;
        }
        let Ok(bytes) = section.data() else {
            continue;
        };
        for off in (0..bytes.len()).step_by(4) {
            let Some(code) = bytes.get(off..off.saturating_add(16)) else {
                break;
            };
            let first = word(file, code);
            if first & 0xffff0000 != 0x3d600000
                || word(file, &code[4..]) & 0xffff0000 != 0x816b0000
                || word(file, &code[8..]) != 0x7d6903a6
                || word(file, &code[12..]) != 0x4e800420
            {
                continue;
            }
            let slot = (first << 16).wrapping_add(word(file, &code[4..]) as u16 as i16 as u32);
            if let Some(name) = slots.get(&u64::from(slot)) {
                out.push(PltSym {
                    addr: section.address() + off as u64,
                    name: name.clone(),
                    slot: None,
                });
            }
        }
    }
    if prove_pic {
        out.extend(super::kuna_ppc32picimports::resolve(file, &slots, &out));
    }
    out
}

pub(super) fn word(file: &object::File<'_>, bytes: &[u8]) -> u32 {
    let bytes = bytes[..4].try_into().unwrap();
    if file.is_little_endian() {
        u32::from_le_bytes(bytes)
    } else {
        u32::from_be_bytes(bytes)
    }
}

/// Recognized PIC stub ranges, independent of import identity or caller state.
/// GNU may merge these into .text under one shared linker-generated FDE.
pub(crate) fn pic_stub_ranges(file: &object::File<'_>) -> Vec<(u64, u64)> {
    if file.architecture() != Architecture::PowerPc {
        return Vec::new();
    }
    let mut ranges = Vec::new();
    for section in file.sections() {
        if !matches!(section.flags(), SectionFlags::Elf { sh_flags } if sh_flags & 6 == 6) {
            continue;
        }
        let Ok(bytes) = section.data() else { continue; };
        let first = (4 - section.address() % 4) as usize % 4;
        for off in (first..bytes.len().saturating_sub(15)).step_by(4) {
            let start = word(file, &bytes[off..]);
            if !matches!(start & 0xffff_0000, 0x817e_0000 | 0x3d7e_0000) {
                continue;
            }
            let words = [start, word(file, &bytes[off + 4..]),
                word(file, &bytes[off + 8..]), word(file, &bytes[off + 12..])];
            if kuna_decomp::kuna_ppc32piccalls::stub_delta(&words).is_some() {
                let start = section.address() + off as u64;
                if let Some(end) = start.checked_add(16) { ranges.push((start, end)); }
            }
        }
    }
    ranges
}

/// Collect metadata even with recovery disabled, so console options remain live.
pub struct PicSlotsPass;
impl crate::pass::AnalysisPass for PicSlotsPass {
    fn phase(&self) -> crate::pass::Phase { crate::pass::Phase::P1 }
    fn id(&self) -> &'static str { "ppc32_pic_slots" }
    fn run(&self, ctx: &crate::pass::AnalysisCtx) -> crate::pass::AnalysisOutput {
        crate::pass::AnalysisOutput { ppc32_pic_slots: pic_slots(ctx.file), ..Default::default() }
    }
}
