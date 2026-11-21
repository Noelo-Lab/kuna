//! Callable PLT ranges whose jump-slot relocation binds to the requested ELF definition.
use std::collections::{HashMap, HashSet};

use object::{
    elf, Architecture, Object, ObjectSection, ObjectSymbol, ObjectSymbolTable, RelocationFlags,
    RelocationTarget,
};

use crate::loader::{elfv1, elfv1_imports, kuna_ppc32_imports};

#[derive(Debug, Clone)]
pub(crate) struct Stub {
    pub definition: u64,
    pub entry: u64,
    pub slot: u64,
    pub end: u64,
}

fn bindings(file: &object::File) -> HashMap<u64, u64> {
    let kind = match file.architecture() {
        Architecture::Aarch64 => elf::R_AARCH64_JUMP_SLOT,
        Architecture::Arm => elf::R_ARM_JUMP_SLOT,
        Architecture::PowerPc => elf::R_PPC_JMP_SLOT,
        Architecture::PowerPc64 => elf::R_PPC64_JMP_SLOT,
        _ => return HashMap::new(),
    };
    let (Some(table), Some(relocs)) = (file.dynamic_symbol_table(), file.dynamic_relocations())
    else {
        return HashMap::new();
    };
    let mut descriptors = None;
    let mut slots = HashMap::new();
    let mut seen = HashSet::new();
    for (slot, reloc) in relocs {
        if !seen.insert(slot) {
            slots.remove(&slot);
            continue;
        }
        if reloc.flags() != (RelocationFlags::Elf { r_type: kind }) || reloc.addend() != 0 {
            continue;
        }
        let RelocationTarget::Symbol(index) = reloc.target() else {
            continue;
        };
        let Ok(symbol) = table.symbol_by_index(index) else {
            continue;
        };
        if !symbol.is_definition()
            || !matches!(symbol.flags(),
            object::SymbolFlags::Elf { st_info, .. } if st_info & 0xf == elf::STT_FUNC)
        {
            continue;
        }
        let address = if file.architecture() == Architecture::PowerPc64 {
            descriptors
                .get_or_insert_with(|| elfv1::Descriptors::read(file))
                .code_address(symbol.address())
        } else {
            symbol.address()
        };
        let Some(section) = symbol
            .section_index()
            .and_then(|i| file.section_by_index(i).ok())
        else {
            continue;
        };
        if address == symbol.address()
            && !matches!(section.flags(),
            object::SectionFlags::Elf { sh_flags } if sh_flags & 6 == 6)
        {
            continue;
        }
        let address = if file.architecture() == Architecture::Arm {
            address & !1
        } else {
            address
        };
        slots.insert(slot, address);
    }
    slots
}

pub(crate) fn stubs(file: &object::File) -> Vec<Stub> {
    let slots = bindings(file);
    if slots.is_empty() {
        return Vec::new();
    }
    let mut out = Vec::new();
    let mut accept = |entry: u64, slot, size| {
        if let Some(&definition) = slots.get(&slot) {
            if let Some(end) = entry.checked_add(size) {
                out.push(Stub {
                    definition,
                    entry,
                    slot,
                    end,
                });
            }
        }
    };
    match file.architecture() {
        Architecture::PowerPc64 => elfv1_imports::visit(file, |entry, _, slot, size| {
            if let Some(slot) = slot {
                accept(entry, slot, size as u64);
            }
        }),
        Architecture::PowerPc => {
            let (bss, imports) = kuna_ppc32_imports::slots(file);
            if bss {
                for slot in imports.keys() {
                    accept(*slot, *slot, 8);
                }
            } else {
                for (entry, form) in kuna_ppc32_imports::stubs(file) {
                    if let kuna_decomp::kuna_ppc32piccalls::Stub::Absolute(slot) = form {
                        if imports.contains_key(&u64::from(slot)) {
                            accept(entry, u64::from(slot), 16);
                        }
                    }
                }
            }
        }
        Architecture::Arm | Architecture::Aarch64 => {
            let arm = file.architecture() == Architecture::Arm;
            let little_code = file.is_little_endian()
                || matches!(file.flags(),
                object::FileFlags::Elf { e_flags, .. } if e_flags & elf::EF_ARM_BE8 != 0);
            for section in file.sections() {
                if !section
                    .name()
                    .ok()
                    .is_some_and(|name| super::plt_section_priority(name).is_some())
                    || !matches!(section.flags(), object::SectionFlags::Elf { sh_flags } if sh_flags & 6 == 6)
                {
                    continue;
                }
                let Ok(bytes) = section.data() else { continue };
                for i in (0..bytes.len()).step_by(4) {
                    let Some(entry) = section.address().checked_add(i as u64) else {
                        break;
                    };
                    let code = &bytes[i..];
                    if arm {
                        if let Some(slot) = super::arm_slot(entry, code, little_code) {
                            accept(entry, slot, 12);
                        }
                    } else if let Some((slot, size)) = aarch64_stub(entry, code) {
                        accept(entry, slot, size);
                    }
                }
            }
        }
        _ => {}
    }
    out
}

fn aarch64_stub(entry: u64, code: &[u8]) -> Option<(u64, u64)> {
    let slot = super::aarch64_slot(entry, code)?;
    let word = |i: usize| Some(u32::from_le_bytes(code.get(i..i + 4)?.try_into().ok()?));
    const BR_X17: u32 = 0xd61f0220;
    match word(8)? {
        BR_X17 => Some((slot, 12)),
        add if add & 0xffc003ff == 0x91000210 && word(12)? == BR_X17 => Some((slot, 16)),
        _ => None,
    }
}

pub(crate) fn slots_for(file: &object::File, entry: u64) -> impl Iterator<Item = u64> {
    bindings(file)
        .into_iter()
        .filter_map(move |(slot, addr)| (addr == entry).then_some(slot))
}
