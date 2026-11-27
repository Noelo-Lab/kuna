//! Fixed void prototypes for undefined MIPS ELF imports. A call-site DIE and
//! the linked PIC call must independently identify the same declaration and
//! symbol. Only witnessed declarations participate in agreement or vetoes.
use std::collections::BTreeMap;
use std::rc::Rc;

use kuna_decomp::dtype::{type_metatype, Datatype};
use kuna_decomp::kuna_gotimportproto::fixed_inputs;

use super::{build_pieces, strip_qualifiers, DieSnap};
use crate::pass::{AnalysisCtx, MipsImportDeclarations};

/// Retained CALL16 and JALR relocations must name the same undefined function
/// at an adjacent GP-relative load of t9 and a classic jalr t9. Key the pair
/// by its return PC, after the delay slot. ET_REL offsets are not linked PCs.
pub(super) fn linked_calls<'a>(file: &object::File<'a>) -> BTreeMap<u64, &'a str> {
    use object::{Object, ObjectSection, ObjectSymbol, RelocationFlags, RelocationTarget};

    let mut loads = BTreeMap::new();
    let mut calls = BTreeMap::new();
    let versioned = std::cell::OnceCell::new();
    if !matches!(
        file.kind(),
        object::ObjectKind::Dynamic | object::ObjectKind::Executable
    ) {
        return BTreeMap::new();
    }
    for section in file
        .sections()
        .filter(|s| s.kind() == object::SectionKind::Text)
    {
        for (pc, rel) in section.relocations() {
            let RelocationFlags::Elf { r_type } = rel.flags() else {
                continue;
            };
            if !matches!(
                r_type,
                object::elf::R_MIPS_CALL16 | object::elf::R_MIPS_JALR
            ) || rel.addend() != 0
                || pc % 4 != 0
            {
                continue;
            }
            let RelocationTarget::Symbol(index) = rel.target() else {
                continue;
            };
            let Ok(symbol) = file.symbol_by_index(index) else {
                continue;
            };
            if !symbol.is_undefined() || symbol.kind() != object::SymbolKind::Text {
                continue;
            }
            let Some(word) = section
                .data_range(pc, 4)
                .ok()
                .flatten()
                .and_then(|s| s.try_into().ok())
            else {
                continue;
            };
            let opcode = if file.is_little_endian() {
                u32::from_le_bytes(word)
            } else {
                u32::from_be_bytes(word)
            };
            let (index, delta) = match r_type {
                object::elf::R_MIPS_CALL16 if matches!(opcode >> 16, 0x8f99 | 0xdf99) => {
                    (&mut loads, 12)
                }
                object::elf::R_MIPS_JALR if opcode == 0x0320f809 => (&mut calls, 8),
                _ => continue,
            };
            if let (Some(ret), Ok(name)) = (pc.checked_add(delta), symbol.name()) {
                index
                    .entry(ret)
                    .and_modify(|old| *old = None)
                    .or_insert(Some((name, symbol.address())));
            }
        }
    }
    calls
        .into_iter()
        .filter_map(|(ret, name)| {
            let target = name?;
            if loads.get(&ret) != Some(&Some(target)) {
                return None;
            }
            let (name, address) = target;
            let Some((base, version)) = name.split_once('@') else {
                return Some((ret, name));
            };
            let imports = versioned.get_or_init(|| match file {
                object::File::Elf32(elf) => versioned_imports(elf),
                object::File::Elf64(elf) => versioned_imports(elf),
                _ => BTreeMap::new(),
            });
            (imports.get(&(base, version)) == Some(&address)).then_some((ret, base))
        })
        .collect()
}

/// GNU ld appends the required version to retained symtab references. Resolve
/// that spelling using the dynamic symbol's version requirement and stub VMA.
fn versioned_imports<'a, Elf: object::read::elf::FileHeader>(
    elf: &object::read::elf::ElfFile<'a, Elf>,
) -> BTreeMap<(&'a str, &'a str), u64> {
    use object::{Object, ObjectSymbol};
    let endian = elf.endian();
    let mut imports = BTreeMap::new();
    let Ok(Some(versions)) = elf.elf_section_table().versions(endian, elf.data()) else {
        return imports;
    };
    for symbol in elf
        .dynamic_symbols()
        .filter(|s| s.is_undefined() && s.kind() == object::SymbolKind::Text)
    {
        let Ok(Some(version)) = versions.version(versions.version_index(endian, symbol.index()))
        else {
            continue;
        };
        if version.file().is_some() {
            if let (Ok(name), Ok(suffix)) = (symbol.name(), std::str::from_utf8(version.name())) {
                imports.insert((name, suffix), symbol.address());
            }
        }
    }
    imports
}

/// Join a call's final relocation to the declaration DIE referenced by its
/// call-site origin. A section-GC tombstone or a source spelling cannot do so.
pub(super) fn witnessed<'a>(
    dwarf: &gimli::Dwarf<super::Reader<'_>>,
    unit: &gimli::Unit<super::Reader<'_>>,
    calls: &BTreeMap<u64, &'a str>,
) -> BTreeMap<usize, Option<&'a str>> {
    let mut result = BTreeMap::new();
    if calls.is_empty() {
        return result;
    }
    let mut cursor = unit.entries();
    let mut owners: Vec<(isize, Vec<gimli::Range>)> = Vec::new();
    while let Ok(Some(entry)) = cursor.next_dfs() {
        while owners
            .last()
            .is_some_and(|(depth, _)| entry.depth() <= *depth)
        {
            owners.pop();
        }
        if entry.tag() == gimli::DW_TAG_subprogram {
            let mut ranges = Vec::new();
            if let Ok(mut iter) = dwarf.die_ranges(unit, entry) {
                while let Ok(Some(r)) = iter.next() {
                    if r.begin != 0 && r.begin < r.end {
                        ranges.push(r);
                    }
                }
            }
            owners.push((entry.depth(), ranges));
            continue;
        }
        let Some((_, ranges)) = owners.last() else {
            continue;
        };
        let (origin, ret) = match entry.tag() {
            gimli::DW_TAG_call_site => (gimli::DW_AT_call_origin, gimli::DW_AT_call_return_pc),
            gimli::DW_TAG_GNU_call_site => (gimli::DW_AT_abstract_origin, gimli::DW_AT_low_pc),
            _ => continue,
        };
        let Some(gimli::AttributeValue::UnitRef(origin)) = entry.attr_value(origin) else {
            continue;
        };
        let Some(ret) = entry
            .attr_value(ret)
            .and_then(|v| dwarf.attr_address(unit, v).ok().flatten())
        else {
            continue;
        };
        if !ranges.iter().any(|r| {
            ret.checked_sub(12)
                .is_some_and(|call| r.begin <= call && ret <= r.end)
        }) {
            continue;
        }
        if let Some(&name) = calls.get(&ret) {
            result
                .entry(origin.0)
                .and_modify(|old| {
                    if *old != Some(name) {
                        *old = None;
                    }
                })
                .or_insert(Some(name));
        }
    }
    result
}

pub(super) fn record(
    sub: &DieSnap,
    dies: &BTreeMap<usize, DieSnap>,
    imports: &BTreeMap<String, Vec<u64>>,
    ctx: &AnalysisCtx,
    complete_unit: bool,
    declarations: &mut MipsImportDeclarations,
) {
    if sub.external != Some(true) || sub.low_pc.is_some() || !sub.has_linkage_name {
        return;
    }
    let name = &sub.linkage_name;
    let Some(entries) = imports.get(name) else {
        return;
    };
    let [entry] = entries.as_slice() else { return };
    declarations.entries.insert(*entry);
    let pointer_size = ctx.arch.types().get_size_of_pointer();
    // ELF symbol versions can give one spelling several different stubs.
    // An unversioned DWARF name cannot choose between their contracts.
    let returns_void = void_result(sub, dies);
    if returns_void {
        declarations.void_results.insert(name.clone());
    }
    let pieces =
        if complete_unit && import_scope(sub, dies) && sub.fixed_declaration && returns_void {
            build_pieces(name, sub, dies, ctx.arch.types(), ctx.arch.data_org().1)
                .filter(fixed_inputs)
                .filter(|p| {
                    super::kuna_cppproto::formal_parameters(sub, dies)
                        .iter()
                        .map(|off| &dies[off])
                        .zip(&p.intypes)
                        .all(|(child, ty)| {
                            modeled_formal(child, dies, ty, sub.cpp_declaration)
                                && matches!(ty.get_size(), 1 | 2 | 4 | 8)
                                && ty.get_size() <= pointer_size
                                && (ty.get_metatype() != type_metatype::TYPE_PTR
                                    || ty.get_size() == pointer_size)
                                && matches!(
                                    ty.get_metatype(),
                                    type_metatype::TYPE_INT
                                        | type_metatype::TYPE_UINT
                                        | type_metatype::TYPE_BOOL
                                        | type_metatype::TYPE_ENUM_INT
                                        | type_metatype::TYPE_ENUM_UINT
                                        | type_metatype::TYPE_PTR
                                )
                        })
                })
                .map(|mut p| {
                    p.void_prototype_source =
                        Some(kuna_decomp::fspec::VoidPrototypeSource::Declaration);
                    (*entry, p)
                })
        } else {
            None
        };

    match declarations.by_name.entry(name.clone()) {
        std::collections::btree_map::Entry::Vacant(slot) => {
            slot.insert(pieces);
        }
        std::collections::btree_map::Entry::Occupied(mut slot) => {
            let existing = slot.get_mut();
            if !existing
                .as_ref()
                .zip(pieces.as_ref())
                .is_some_and(|((_, a), (_, b))| {
                    a.intypes.len() == b.intypes.len()
                        && a.intypes
                            .iter()
                            .zip(&b.intypes)
                            .all(|(x, y)| Rc::ptr_eq(x, y))
                })
            {
                *existing = None;
            }
        }
    }
}

/// The generic DWARF importer can give an unsupported encoding an integer
/// display type of the same size. Only lock by-value arguments whose ABI class
/// survives conversion. A pointer has pointer storage regardless of its referent.
fn modeled_formal(
    child: &DieSnap,
    dies: &BTreeMap<usize, DieSnap>,
    ty: &Datatype,
    cpp: bool,
) -> bool {
    let Some(die) = child.type_ref.and_then(|offset| dies.get(&offset)) else {
        return false;
    };
    // The type converter collapses typedefs and qualifiers. Their explicit
    // alignment can move this argument and every following argument.
    if std::iter::once(child)
        .chain(super::qualifier_chain(die, dies))
        .filter_map(|node| node.alignment)
        .any(|align| align != ty.get_alignment() as u64)
    {
        return false;
    }
    let (die, _) = strip_qualifiers(die, dies);
    match die.tag {
        gimli::DW_TAG_pointer_type => die
            .byte_size
            .is_none_or(|size| size == ty.get_size() as u64),
        gimli::DW_TAG_reference_type | gimli::DW_TAG_rvalue_reference_type => {
            cpp && die
                .byte_size
                .is_none_or(|size| size == ty.get_size() as u64)
        }
        gimli::DW_TAG_enumeration_type => {
            super::enum_signed(die, dies)
                == Some(matches!(
                    ty.get_metatype(),
                    type_metatype::TYPE_INT | type_metatype::TYPE_ENUM_INT
                ))
                && super::enum_width(die, dies) == Some(ty.get_size())
        }
        gimli::DW_TAG_base_type => match die.encoding {
            // A floating-point or multiword input can move every later argument.
            // The declaration alone supplies no evidence for that register layout.
            Some(
                gimli::DW_ATE_signed
                | gimli::DW_ATE_unsigned
                | gimli::DW_ATE_signed_char
                | gimli::DW_ATE_unsigned_char
                | gimli::DW_ATE_boolean,
            ) => die.byte_size == Some(ty.get_size() as u64),
            _ => false,
        },
        _ => false,
    }
}

/// Below compilation-unit scope, only an exact linkage spelling in a namespace
/// identifies the external function. Local and member declarations stay scoped.
fn import_scope<'a>(mut die: &'a DieSnap, dies: &'a BTreeMap<usize, DieSnap>) -> bool {
    if die.depth != 1 && !die.has_linkage_name {
        return false;
    }
    while die.depth > 1 {
        let Some(parent) = die.parent.and_then(|off| dies.get(&off)) else {
            return false;
        };
        if parent.tag != gimli::DW_TAG_namespace {
            return false;
        }
        die = parent;
    }
    die.depth == 1
}

fn void_result(sub: &DieSnap, dies: &BTreeMap<usize, DieSnap>) -> bool {
    if !sub.has_type_attribute {
        return true;
    }
    let Some(die) = sub.type_ref.and_then(|off| dies.get(&off)) else {
        return false;
    };
    let (base, _) = strip_qualifiers(die, dies);
    matches!(
        base.tag,
        gimli::DW_TAG_typedef
            | gimli::DW_TAG_const_type
            | gimli::DW_TAG_volatile_type
            | gimli::DW_TAG_restrict_type
    ) && !base.has_type_attribute
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unresolved_and_cyclic_aliases_do_not_establish_a_void_result() {
        let mut sub = DieSnap::new(gimli::DW_TAG_subprogram, 1);
        sub.has_type_attribute = true;
        sub.type_ref = Some(1);
        let mut alias = DieSnap::new(gimli::DW_TAG_typedef, 1);
        alias.has_type_attribute = true;
        let mut dies = BTreeMap::from([(1, alias)]);
        // An unsupported reference form is present but has no unit offset.
        assert!(!void_result(&sub, &dies));
        dies.get_mut(&1).unwrap().type_ref = Some(2);
        assert!(!void_result(&sub, &dies));
        dies.get_mut(&1).unwrap().type_ref = Some(1);
        assert!(!void_result(&sub, &dies));
    }
}
