//! Follow one proven immutable ELF relative pointer at a memory-indirect call.

use std::collections::BTreeMap;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::AddrSpace;
use kuna_decomp::architecture::Architecture;
use kuna_num::opcodes::OpCode;
use object::{
    Object, ObjectSection, ObjectSymbol, SectionFlags, SymbolFlags,
    SymbolSection,
};

use super::xrefs::{in_range, FullOp, XrefKind, MAX_SLOT_COPIES};
use crate::loader::kuna_dataptrs::Coverage;
use crate::loader::kuna_dynrelocs::{fixup_sites, mapped_flags, protected, protected_pages, FixupKind};

pub(super) struct GotCalls(BTreeMap<u64, u64>);

impl GotCalls {
    pub(super) fn new(
        file: &object::File,
        arch: &Architecture,
        exec: &[(u64, u64)],
    ) -> Option<Self> {
        let data = arch.manage().get_default_data_space()?;
        Self::collect(file, exec, |slot| {
            arch.read_loadimage_value(&Address::new(Rc::clone(data), slot), 8)
                .ok()
        })
    }

    fn collect(
        file: &object::File,
        exec: &[(u64, u64)],
        read: impl Fn(u64) -> Option<u64>,
    ) -> Option<Self> {
        if file.architecture() != object::Architecture::X86_64
            || !file.is_little_endian()
            || !matches!(
                file.kind(),
                object::ObjectKind::Executable | object::ObjectKind::Dynamic
            )
        {
            return None;
        }
        let object::File::Elf64(elf) = file else {
            return None;
        };
        let mut got = file
            .sections()
            .filter(|section| matches!(section.name(), Ok(".got")));
        let section = got.next()?;
        if got.next().is_some() {
            return None;
        }
        let got_start = section.address();
        let got_end =
            got_start.checked_add(section.size().min(section.data().ok()?.len() as u64))?;
        let fixups = fixup_sites(file, elf.data());
        if !matches!(fixups.coverage, Coverage::Complete(_))
            || fixups
                .sites
                .iter()
                .any(|fixup| fixup.kind == FixupKind::Relocation)
        {
            return None;
        }
        let mut function_entries = BTreeMap::new();
        for symbol in file.symbols().chain(file.dynamic_symbols()) {
            let SymbolSection::Section(index) = symbol.section() else {
                continue;
            };
            let SymbolFlags::Elf { st_info, .. } = symbol.flags() else {
                continue;
            };
            let ordinary = match st_info & 0xf {
                object::elf::STT_FUNC => {
                    let Ok(section) = file.section_by_index(index) else {
                        continue;
                    };
                    if !matches!(section.flags(), SectionFlags::Elf { sh_flags }
                        if sh_flags & u64::from(object::elf::SHF_EXECINSTR) != 0)
                        || !symbol
                            .address()
                            .checked_sub(section.address())
                            .is_some_and(|offset| offset < section.size())
                    {
                        continue;
                    }
                    true
                }
                object::elf::STT_GNU_IFUNC => false,
                _ => continue,
            };
            function_entries
                .entry(symbol.address())
                .and_modify(|entry| *entry &= ordinary)
                .or_insert(ordinary);
        }
        let relro = protected_pages(elf.data());
        let mut targets = BTreeMap::new();
        let mut prior_end = 0;
        for (i, fixup) in fixups.sites.iter().enumerate() {
            let end = fixup.vma.checked_add(u64::from(fixup.width));
            let overlap = prior_end > fixup.vma
                || end.is_none_or(|end| fixups.sites.get(i + 1).is_some_and(|next| next.vma < end));
            prior_end = prior_end.max(end.unwrap_or(u64::MAX));
            let Some(end) = end else { continue };
            let Some(target) = fixup.target else { continue };
            if overlap
                || fixup.kind != FixupKind::Rebase
                || fixup.width != 8
                || fixup.vma % 8 != 0
                || fixup.vma < got_start
                || end > got_end
                || function_entries.get(&target) != Some(&true)
                || !in_range(exec, target)
                || !target
                    .checked_add(1)
                    .and_then(|end| mapped_flags(file, target, end))
                    .is_some_and(|flags| flags & object::elf::PF_X != 0)
                || !protected(file, fixup.vma, end, relro)
            {
                continue;
            }
            if read(fixup.vma) == Some(target) {
                targets.insert(fixup.vma, target);
            }
        }
        (!targets.is_empty()).then_some(Self(targets))
    }

    pub(super) fn target(&self, ops: &[FullOp], data: &Rc<AddrSpace>) -> Option<(u64, u64)> {
        let mut calls = ops
            .iter()
            .enumerate()
            .filter(|(_, op)| op.opcode == OpCode::CPUI_CALLIND);
        let (mut before, call) = calls.next()?;
        if call.ins.len() != 1
            || calls.next().is_some()
            || ops.iter().enumerate().any(|(index, op)| {
                index != before
                    && matches!(
                        op.opcode,
                        OpCode::CPUI_BRANCH
                            | OpCode::CPUI_CBRANCH
                            | OpCode::CPUI_BRANCHIND
                            | OpCode::CPUI_CALL
                            | OpCode::CPUI_CALLOTHER
                            | OpCode::CPUI_RETURN
                    )
            })
        {
            return None;
        }
        let mut value = call.ins.first()?;
        for _ in 0..MAX_SLOT_COPIES {
            if value.size != 8 {
                return None;
            }
            if value
                .space
                .as_ref()
                .is_some_and(|space| Rc::ptr_eq(space, data))
            {
                return self
                    .0
                    .get(&value.offset)
                    .map(|&target| (value.offset, target));
            }
            let index = ops[..before]
                .iter()
                .rposition(|op| op.out.as_ref().is_some_and(|out| overlaps(out, value)))?;
            let def = &ops[index];
            if def.out.as_ref() != Some(value)
                || def.opcode != OpCode::CPUI_COPY
                || def.ins.len() != 1
            {
                return None;
            }
            value = &def.ins[0];
            before = index;
        }
        None
    }
}

/// Leave the legacy listing path at a site with a user-stated callee.
pub(super) fn has_stated_callee(arch: &Architecture, code: &Rc<AddrSpace>, site: u64) -> bool {
    if arch.kuna_callee_overrides.is_empty() {
        return false;
    }
    let site = Address::new(Rc::clone(code), site);
    arch.kuna_callee_overrides
        .values()
        .any(|sites| sites.contains_key(&site))
}

/// Slot operands remain reads after the single proven call gets its real edge.
pub(super) fn resolved_slot_reads(references: &mut Vec<(u64, XrefKind)>, slot: u64) {
    for (_, kind) in references.iter_mut() {
        if *kind == XrefKind::Call {
            *kind = XrefKind::Read;
        }
    }
    if !references.contains(&(slot, XrefKind::Read)) {
        references.push((slot, XrefKind::Read));
    }
}

fn overlaps(a: &kuna_num::pcoderaw::VarnodeData, b: &kuna_num::pcoderaw::VarnodeData) -> bool {
    matches!((&a.space, &b.space), (Some(a), Some(b)) if Rc::ptr_eq(a, b))
        && a.offset < b.offset.saturating_add(u64::from(b.size))
        && b.offset < a.offset.saturating_add(u64::from(a.size))
}

#[cfg(test)]
#[path = "kuna_gotcall_tests.rs"]
mod tests;
