//! PPC32 BSS-PLT entries and secure-PLT call stubs, backed by JMP_SLOT records.
use super::elf_plt::PltSym;
use object::{
    Architecture, Object, ObjectKind, ObjectSection, ObjectSymbol, RelocationFlags,
    RelocationTarget, SectionFlags, SectionKind, SymbolKind,
};
use std::collections::{BTreeMap, HashSet};

pub(super) fn resolve(file: &object::File<'_>) -> Vec<PltSym> {
    if file.architecture() != Architecture::PowerPc
        || !matches!(file.kind(), ObjectKind::Executable | ObjectKind::Dynamic)
    {
        return Vec::new();
    }
    let Some(plt) = file.section_by_name(".plt") else {
        return Vec::new();
    };
    let SectionFlags::Elf { sh_flags } = plt.flags() else {
        return Vec::new();
    };
    if sh_flags & 3 != 3 {
        return Vec::new();
    }
    let bss = plt.kind() == SectionKind::UninitializedData && sh_flags & 4 != 0;
    if !bss && sh_flags & 4 != 0 {
        return Vec::new();
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
    if bss {
        return slots
            .into_iter()
            .map(|(addr, name)| PltSym { addr, name })
            .collect();
    }
    if slots.is_empty() {
        return Vec::new();
    }
    let base = pic_base(file);
    let got2_base = file
        .section_by_name(".got2")
        .filter(|s| s.size() == 0)
        .map(|s| (s.address() as u32).wrapping_add(0x8000));
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
            let slot = match first & 0xffff0000 {
                0x3d600000 | 0x3d7e0000
                    if word(file, &code[4..]) & 0xffff0000 == 0x816b0000
                        && word(file, &code[8..]) == 0x7d6903a6
                        && word(file, &code[12..]) == 0x4e800420 =>
                {
                    let anchor = if first & 0x001f0000 == 0 {
                        Some(0)
                    } else {
                        base
                    };
                    anchor.map(|b| {
                        b.wrapping_add(first << 16)
                            .wrapping_add(word(file, &code[4..]) as u16 as i16 as u32)
                    })
                }
                0x817e0000
                    if word(file, &code[4..]) == 0x7d6903a6
                        && word(file, &code[8..]) == 0x4e800420
                        && word(file, &code[12..]) == 0x60000000 =>
                {
                    base.map(|b| b.wrapping_add(first as u16 as i16 as u32))
                }
                _ => None,
            };
            if first & 0xffff0000 != 0x3d600000 {
                if let (Some(slot), Some(base), Some(other)) = (slot, base, got2_base) {
                    if other != base
                        && slots
                            .contains_key(&u64::from(slot.wrapping_sub(base).wrapping_add(other)))
                    {
                        continue;
                    }
                }
            }
            if let Some(name) = slot.and_then(|slot| slots.get(&u64::from(slot))) {
                out.push(PltSym {
                    addr: section.address() + off as u64,
                    name: name.clone(),
                });
            }
        }
    }
    out
}

fn word(file: &object::File<'_>, bytes: &[u8]) -> u32 {
    let bytes = bytes[..4].try_into().unwrap();
    if file.is_little_endian() {
        u32::from_le_bytes(bytes)
    } else {
        u32::from_be_bytes(bytes)
    }
}

/// DT_PPC_GOT identifies the small-PIC anchor, unlike DT_PLTGOT (the slots).
/// A nonempty .got2 permits per-input-object r30 bases, which this decoder cannot
/// distinguish after linking. Decline those PIC stubs rather than guess a base.
fn pic_base(file: &object::File<'_>) -> Option<u32> {
    if file
        .sections()
        .any(|s| s.name() == Ok(".got2") && s.size() != 0)
    {
        return None;
    }
    let dynamic = file.section_by_name(".dynamic")?;
    let mut base = None;
    for entry in dynamic.data().ok()?.chunks_exact(8) {
        let tag = word(file, entry);
        if tag == 0 {
            break;
        }
        if tag == object::elf::DT_PPC_GOT as u32 {
            if base.is_some() {
                return None;
            }
            base = Some(word(file, &entry[4..]));
        }
    }
    let base = base?;
    let got = file.section_by_name(".got")?;
    if !matches!(got.flags(), SectionFlags::Elf { sh_flags } if sh_flags & 3 == 3) {
        return None;
    }
    let off = usize::try_from(u64::from(base).checked_sub(got.address())?).ok()?;
    let header = got.data().ok()?.get(off..off.checked_add(12)?)?;
    if u64::from(word(file, header)) != dynamic.address() {
        return None;
    }
    for sym in file.symbols().chain(file.dynamic_symbols()) {
        if sym.name() == Ok("_GLOBAL_OFFSET_TABLE_")
            && (sym.is_undefined() || sym.address() != u64::from(base))
        {
            return None;
        }
    }
    Some(base)
}
