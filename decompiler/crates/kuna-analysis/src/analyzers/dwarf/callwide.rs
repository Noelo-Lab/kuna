//! Bounded, explicit C scalar contracts for callwide's complete-image preflight.

use std::collections::{BTreeMap, BTreeSet};

use object::{Object, ObjectSection};

use kuna_base::types::uint4;
use kuna_decomp::architecture::Architecture;
use kuna_decomp::dtype::{type_metatype, Datatype, TypeFactory};
use kuna_decomp::fspec::PrototypePieces;

use super::{build_pieces, kuna_dwarfbyref, snapshot_unit_for_abi, DieSnap, Reader};

const MAX_BYTES: usize = 8 * 1024 * 1024;
const MAX_ABBREV_BYTES: usize = 128 * 1024;
const MAX_UNIT_BYTES: usize = 1024 * 1024;
const MAX_PARSE_BYTES: usize = 32 * 1024 * 1024;
const MAX_UNITS: usize = 256;
const MAX_RECORDS: usize = 16_384;
const MAX_ATTRIBUTES: usize = 262_144;
const MAX_NAME_BYTES: usize = 1024 * 1024;
const MAX_NAME: usize = 1024;
const MAX_DEPTH: isize = 64;
const MAX_FORMALS: usize = 32;
const MAX_TYPE_CHAIN: usize = 32;

type Names<'a> = BTreeMap<&'a str, (u64, &'a str)>;

fn names(entries: &[(u64, String, Vec<String>)]) -> Option<Names<'_>> {
    if entries.len() > MAX_RECORDS {
        return None;
    }
    let mut result = BTreeMap::new();
    let mut addresses = BTreeSet::new();
    let mut bytes = 0usize;
    let mut count = 0usize;
    for (address, name, aliases) in entries {
        if !addresses.insert(*address) {
            return None;
        }
        for alias in std::iter::once(name).chain(aliases) {
            count = count.checked_add(1)?;
            bytes = bytes.checked_add(alias.len())?;
            if count > MAX_RECORDS
                || bytes > MAX_NAME_BYTES
                || alias.is_empty()
                || alias.len() > MAX_NAME
            {
                return None;
            }
            if let Some((previous, _)) = result.insert(alias.as_str(), (*address, name.as_str())) {
                if previous != *address {
                    return None;
                }
            }
        }
    }
    Some(result)
}

/// Read only complete, unambiguous scalar contracts against established entries.
/// Definitions must match their low PC; declarations need one exact name/alias.
/// Unsupported contracts supply no fact. Invalid input or any exhausted budget
/// refuses the whole extraction. Ordinary DwarfPass recovery is unchanged.
pub fn declarations(
    file: &object::File<'_>,
    arch: &Architecture,
    entries: &[(u64, String, Vec<String>)],
) -> Option<Vec<(u64, PrototypePieces)>> {
    let names = names(entries)?;
    if names.is_empty() {
        return Some(Vec::new());
    }
    let mut total = 0usize;
    let sections = gimli::DwarfSections::load(|id| -> Result<Vec<u8>, ()> {
        let Some(section) = file.section_by_name(id.name()) else {
            return Ok(Vec::new());
        };
        let data = section.compressed_data().map_err(|_| ())?;
        if data.format != object::CompressionFormat::None {
            return Err(());
        }
        total = total.checked_add(data.data.len()).ok_or(())?;
        if total > MAX_BYTES
            || (id == gimli::SectionId::DebugAbbrev && data.data.len() > MAX_ABBREV_BYTES)
        {
            return Err(());
        }
        Ok(data.data.to_vec())
    })
    .ok()?;
    let endian = if file.is_little_endian() {
        gimli::RunTimeEndian::Little
    } else {
        gimli::RunTimeEndian::Big
    };
    let dwarf = sections.borrow(|bytes| gimli::EndianSlice::new(bytes, endian));
    let (_, word_size) = arch.data_org();
    collect(
        &dwarf,
        &names,
        arch.types(),
        word_size,
        kuna_dwarfbyref::argument_abi(file.format(), file.architecture()),
        total,
    )
}

struct Selected {
    offset: usize,
    entry: u64,
    name: String,
}

#[derive(Default)]
struct UnitPlan {
    records: usize,
    c_language: bool,
    selected: Vec<Selected>,
    unsupported: BTreeSet<usize>,
}

fn unsupported(entry: &gimli::DebuggingInformationEntry<Reader<'_>>) -> bool {
    if entry
        .attr_value(gimli::DW_AT_calling_convention)
        .is_some_and(|value| {
            !matches!(
                value,
                gimli::AttributeValue::CallingConvention(gimli::DW_CC_normal)
            )
        })
    {
        return true;
    }
    if [
        gimli::DW_AT_specification,
        gimli::DW_AT_abstract_origin,
        gimli::DW_AT_object_pointer,
        gimli::DW_AT_linkage_name,
        gimli::DW_AT_trampoline,
        gimli::DW_AT_virtuality,
        gimli::DW_AT_inline,
        gimli::DW_AT_ranges,
        gimli::DW_AT_default_value,
        gimli::DW_AT_address_class,
        gimli::DW_AT_endianity,
        gimli::DW_AT_alignment,
        gimli::DW_AT_bit_size,
        gimli::DW_AT_bit_offset,
        gimli::DW_AT_data_bit_offset,
    ]
    .iter()
    .any(|attribute| entry.attr_value(*attribute).is_some())
    {
        return true;
    }
    if matches!(
        entry.attr_value(gimli::DW_AT_artificial),
        Some(gimli::AttributeValue::Flag(true))
    ) {
        return true;
    }
    entry
        .attr_value(gimli::DW_AT_type)
        .is_some_and(|value| !matches!(value, gimli::AttributeValue::UnitRef(_)))
}

fn prewalk(
    dwarf: &gimli::Dwarf<Reader<'_>>,
    names: &Names<'_>,
    section_bytes: usize,
) -> Option<Vec<UnitPlan>> {
    let mut units = dwarf.units();
    let mut plans = Vec::new();
    let mut seen_entries = BTreeSet::new();
    let mut records = 0usize;
    let mut attributes = 0usize;
    let mut name_bytes = 0usize;
    while let Some(header) = units.next().ok()? {
        if plans.len() >= MAX_UNITS
            || header.unit_length() > MAX_UNIT_BYTES
            || section_bytes.checked_mul(plans.len() + 1)? > MAX_PARSE_BYTES
        {
            return None;
        }
        let unit = dwarf.unit(header).ok()?;
        let mut plan = UnitPlan::default();
        let mut cursor = unit.entries();
        while let Some(entry) = cursor.next_dfs().ok()? {
            records += 1;
            plan.records += 1;
            attributes = attributes.checked_add(entry.attrs().len())?;
            if records > MAX_RECORDS
                || entry.attrs().len() > 64
                || attributes > MAX_ATTRIBUTES
                || !(0..=MAX_DEPTH).contains(&entry.depth())
            {
                return None;
            }
            if plan.records == 1 {
                if entry.depth() != 0 || entry.tag() != gimli::DW_TAG_compile_unit {
                    return None;
                }
                plan.c_language = matches!(
                    entry.attr_value(gimli::DW_AT_language),
                    Some(gimli::AttributeValue::Language(
                        gimli::DW_LANG_C
                            | gimli::DW_LANG_C89
                            | gimli::DW_LANG_C99
                            | gimli::DW_LANG_C11
                    ))
                );
            } else if entry.depth() == 0 {
                return None;
            }
            let mut attribute_names = BTreeSet::new();
            for attribute in entry.attrs() {
                if !attribute_names.insert(attribute.name().0) {
                    return None;
                }
            }
            if unsupported(entry) {
                plan.unsupported.insert(entry.offset().0);
            }
            if matches!(
                entry.tag(),
                gimli::DW_TAG_subprogram | gimli::DW_TAG_formal_parameter
            ) {
                for attribute in [gimli::DW_AT_specification, gimli::DW_AT_abstract_origin] {
                    if let Some(value) = entry.attr_value(attribute) {
                        let gimli::AttributeValue::UnitRef(offset) = value else {
                            return None;
                        };
                        plan.unsupported.insert(offset.0);
                    }
                }
            }
            let Some(value) = entry.attr_value(gimli::DW_AT_name) else {
                continue;
            };
            let value = dwarf.attr_string(&unit, value).ok()?;
            let name = std::str::from_utf8(value.slice()).ok()?;
            name_bytes = name_bytes.checked_add(name.len())?;
            if name.len() > MAX_NAME || name_bytes > MAX_NAME_BYTES {
                return None;
            }
            if entry.tag() != gimli::DW_TAG_subprogram {
                continue;
            }
            let Some(&(address, canonical)) = names.get(name) else {
                continue;
            };
            if !seen_entries.insert(address) {
                return None;
            }
            if let Some(value) = entry.attr_value(gimli::DW_AT_low_pc) {
                if dwarf.attr_address(&unit, value).ok()?? != address {
                    return None;
                }
            }
            plan.selected.push(Selected {
                offset: entry.offset().0,
                entry: address,
                name: canonical.to_owned(),
            });
        }
        if plan.records == 0 {
            return None;
        }
        plans.push(plan);
    }
    Some(plans)
}

#[derive(Default)]
struct TypeBudget {
    steps: usize,
    exhausted: bool,
}

#[derive(Clone, Copy)]
struct Scalar {
    size: i32,
    meta: type_metatype,
}

fn scalar(
    mut offset: usize,
    dies: &BTreeMap<usize, DieSnap>,
    unsupported: &BTreeSet<usize>,
    budget: &mut TypeBudget,
) -> Option<Scalar> {
    let mut seen = BTreeSet::new();
    for _ in 0..MAX_TYPE_CHAIN {
        budget.steps += 1;
        if budget.steps > MAX_RECORDS {
            budget.exhausted = true;
            return None;
        }
        if unsupported.contains(&offset) || !seen.insert(offset) {
            return None;
        }
        let die = dies.get(&offset)?;
        match die.tag {
            gimli::DW_TAG_typedef
            | gimli::DW_TAG_const_type
            | gimli::DW_TAG_volatile_type
            | gimli::DW_TAG_restrict_type => {
                offset = die.type_ref?;
            }
            gimli::DW_TAG_base_type => {
                let size = i32::try_from(die.byte_size?).ok()?;
                if !matches!(size, 1 | 2 | 4 | 8) {
                    return None;
                }
                let meta = match die.encoding? {
                    gimli::DW_ATE_signed | gimli::DW_ATE_signed_char => type_metatype::TYPE_INT,
                    gimli::DW_ATE_unsigned | gimli::DW_ATE_unsigned_char | gimli::DW_ATE_UTF => {
                        type_metatype::TYPE_UINT
                    }
                    gimli::DW_ATE_boolean if size == 1 => type_metatype::TYPE_BOOL,
                    _ => return None,
                };
                return Some(Scalar { size, meta });
            }
            _ => return None,
        }
    }
    budget.exhausted = true;
    None
}

fn matches_scalar(actual: &Datatype, expected: Option<Scalar>) -> bool {
    match expected {
        Some(expected) => {
            actual.get_size() == expected.size && actual.get_metatype() == expected.meta
        }
        None => actual.get_size() == 0 && actual.get_metatype() == type_metatype::TYPE_VOID,
    }
}

fn pieces(
    selected: &Selected,
    plan: &UnitPlan,
    dies: &BTreeMap<usize, DieSnap>,
    types: &dyn TypeFactory,
    word_size: uint4,
    budget: &mut TypeBudget,
) -> Option<PrototypePieces> {
    let sub = dies.get(&selected.offset)?;
    if !plan.c_language
        || !sub.prototyped
        || sub.depth != 1
        || plan.unsupported.contains(&selected.offset)
        || (sub.declaration && sub.low_pc.is_some())
        || (!sub.declaration && sub.low_pc.is_none())
    {
        return None;
    }
    let expected_output = match sub.type_ref {
        Some(offset) => Some(scalar(offset, dies, &plan.unsupported, budget)?),
        None => None,
    };
    let mut expected_inputs = Vec::new();
    for offset in &sub.children {
        let child = dies.get(offset)?;
        if child.tag == gimli::DW_TAG_unspecified_parameters {
            return None;
        }
        if child.tag != gimli::DW_TAG_formal_parameter {
            continue;
        }
        if expected_inputs.len() >= MAX_FORMALS {
            budget.exhausted = true;
            return None;
        }
        if plan.unsupported.contains(offset)
            || (sub.declaration && child.has_location)
            || child.indirect_parameter
            || child.register_indirect_parameter.is_some()
        {
            return None;
        }
        expected_inputs.push(scalar(child.type_ref?, dies, &plan.unsupported, budget)?);
    }
    let pieces = build_pieces(&selected.name, sub, dies, types, word_size)?;
    if !matches_scalar(pieces.outtype.as_ref()?, expected_output)
        || pieces.intypes.len() != expected_inputs.len()
        || !pieces
            .intypes
            .iter()
            .zip(expected_inputs)
            .all(|(actual, expected)| matches_scalar(actual, Some(expected)))
    {
        return None;
    }
    Some(pieces)
}

fn collect(
    dwarf: &gimli::Dwarf<Reader<'_>>,
    names: &Names<'_>,
    types: &dyn TypeFactory,
    word_size: uint4,
    abi: kuna_dwarfbyref::ArgumentAbi,
    section_bytes: usize,
) -> Option<Vec<(u64, PrototypePieces)>> {
    let plans = prewalk(dwarf, names, section_bytes)?;
    let mut units = dwarf.units();
    let mut result = Vec::new();
    let mut budget = TypeBudget::default();
    for plan in plans {
        let header = units.next().ok()??;
        if plan.selected.is_empty() || !plan.c_language {
            continue;
        }
        let unit = dwarf.unit(header).ok()?;
        let dies = snapshot_unit_for_abi(dwarf, &unit, abi);
        if dies.len() != plan.records {
            return None;
        }
        for selected in &plan.selected {
            if let Some(pieces) = pieces(selected, &plan, &dies, types, word_size, &mut budget) {
                result.push((selected.entry, pieces));
            }
            if budget.exhausted {
                return None;
            }
        }
    }
    result.sort_by_key(|(entry, _)| *entry);
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use kuna_decomp::dtype::TypeFactoryImpl;

    struct Function {
        name: &'static str,
        address: Option<u32>,
        convention: u8,
        prototyped: bool,
        formals: usize,
        missing_type: bool,
        locations: bool,
    }

    impl Function {
        fn declaration(name: &'static str) -> Self {
            Self {
                name,
                address: None,
                convention: 1,
                prototyped: true,
                formals: 1,
                missing_type: false,
                locations: false,
            }
        }
    }

    fn uleb(bytes: &mut Vec<u8>, mut value: u64) {
        loop {
            let part = value as u8 & 0x7f;
            value >>= 7;
            bytes.push(part | if value == 0 { 0 } else { 0x80 });
            if value == 0 {
                return;
            }
        }
    }

    fn abbreviation(
        bytes: &mut Vec<u8>,
        code: u64,
        tag: gimli::DwTag,
        children: bool,
        attributes: &[(gimli::DwAt, gimli::DwForm)],
    ) {
        uleb(bytes, code);
        uleb(bytes, tag.0 as u64);
        bytes.push(u8::from(children));
        for (name, form) in attributes {
            uleb(bytes, name.0 as u64);
            uleb(bytes, form.0 as u64);
        }
        bytes.extend([0, 0]);
    }

    fn fixture(
        functions: &[Function],
        padding: usize,
        representation: Option<gimli::DwAt>,
        encoding: gimli::DwAte,
        size: u8,
    ) -> (Vec<u8>, Vec<u8>) {
        let mut abbreviations = Vec::new();
        abbreviation(
            &mut abbreviations,
            1,
            gimli::DW_TAG_compile_unit,
            true,
            &[(gimli::DW_AT_language, gimli::DW_FORM_data2)],
        );
        let mut scalar_attributes = vec![
            (gimli::DW_AT_name, gimli::DW_FORM_string),
            (gimli::DW_AT_byte_size, gimli::DW_FORM_data1),
            (gimli::DW_AT_encoding, gimli::DW_FORM_data1),
        ];
        if let Some(attribute) = representation {
            scalar_attributes.push((attribute, gimli::DW_FORM_data1));
        }
        abbreviation(
            &mut abbreviations,
            2,
            gimli::DW_TAG_base_type,
            false,
            &scalar_attributes,
        );
        let mut signature = vec![
            (gimli::DW_AT_name, gimli::DW_FORM_string),
            (gimli::DW_AT_type, gimli::DW_FORM_ref4),
            (gimli::DW_AT_declaration, gimli::DW_FORM_flag),
            (gimli::DW_AT_prototyped, gimli::DW_FORM_flag),
            (gimli::DW_AT_calling_convention, gimli::DW_FORM_data1),
        ];
        abbreviation(
            &mut abbreviations,
            3,
            gimli::DW_TAG_subprogram,
            true,
            &signature,
        );
        signature.push((gimli::DW_AT_low_pc, gimli::DW_FORM_addr));
        abbreviation(
            &mut abbreviations,
            4,
            gimli::DW_TAG_subprogram,
            true,
            &signature,
        );
        abbreviation(
            &mut abbreviations,
            5,
            gimli::DW_TAG_formal_parameter,
            false,
            &[(gimli::DW_AT_type, gimli::DW_FORM_ref4)],
        );
        abbreviation(
            &mut abbreviations,
            6,
            gimli::DW_TAG_unspecified_type,
            false,
            &[],
        );
        abbreviation(
            &mut abbreviations,
            7,
            gimli::DW_TAG_formal_parameter,
            false,
            &[
                (gimli::DW_AT_type, gimli::DW_FORM_ref4),
                (gimli::DW_AT_location, gimli::DW_FORM_exprloc),
            ],
        );
        abbreviations.push(0);
        let mut info = vec![0; 11];
        info.push(1);
        info.extend(gimli::DW_LANG_C99.0.to_le_bytes());
        let scalar = info.len() as u32;
        info.push(2);
        info.extend(b"u64\0");
        info.extend([size, encoding.0]);
        if representation.is_some() {
            info.push(32);
        }
        for function in functions {
            info.push(if function.address.is_some() { 4 } else { 3 });
            info.extend(function.name.as_bytes());
            info.push(0);
            info.extend(scalar.to_le_bytes());
            info.extend([
                u8::from(function.address.is_none()),
                u8::from(function.prototyped),
                function.convention,
            ]);
            if let Some(address) = function.address {
                info.extend(address.to_le_bytes());
            }
            for _ in 0..function.formals {
                info.push(if function.locations { 7 } else { 5 });
                info.extend(
                    if function.missing_type {
                        u32::MAX
                    } else {
                        scalar
                    }
                    .to_le_bytes(),
                );
                if function.locations {
                    info.extend([1, gimli::DW_OP_reg4.0]);
                }
            }
            info.push(0);
        }
        info.extend(std::iter::repeat_n(6, padding));
        info.push(0);
        let length = info.len() as u32 - 4;
        info[..4].copy_from_slice(&length.to_le_bytes());
        info[4..6].copy_from_slice(&4u16.to_le_bytes());
        info[10] = 4;
        (info, abbreviations)
    }

    fn extract(
        functions: &[Function],
        entries: &[(u64, String, Vec<String>)],
        padding: usize,
    ) -> Option<Vec<(u64, PrototypePieces)>> {
        extract_representation(functions, entries, padding, None, gimli::DW_ATE_unsigned, 8)
    }

    fn extract_representation(
        functions: &[Function],
        entries: &[(u64, String, Vec<String>)],
        padding: usize,
        representation: Option<gimli::DwAt>,
        encoding: gimli::DwAte,
        size: u8,
    ) -> Option<Vec<(u64, PrototypePieces)>> {
        let names = names(entries)?;
        let (info, abbreviations) = fixture(functions, padding, representation, encoding, size);
        let bytes = info.len() + abbreviations.len();
        let sections = gimli::DwarfSections::load(|id| -> Result<Vec<u8>, ()> {
            Ok(match id {
                gimli::SectionId::DebugInfo => info.clone(),
                gimli::SectionId::DebugAbbrev => abbreviations.clone(),
                _ => Vec::new(),
            })
        })
        .ok()?;
        let dwarf =
            sections.borrow(|bytes| gimli::EndianSlice::new(bytes, gimli::RunTimeEndian::Little));
        let types = TypeFactoryImpl::new();
        types.set_default_alignment_map();
        types.set_max_basetype_size(8);
        types.setup_sizes(Some(8), 8, 4);
        types.cache_core_types().ok()?;
        collect(
            &dwarf,
            &names,
            &types,
            1,
            kuna_dwarfbyref::ArgumentAbi::default(),
            bytes,
        )
    }

    #[test]
    fn explicit_definitions_and_declarations_keep_exact_entries_and_aliases() {
        let caller = Function::declaration("caller_alias");
        let mut sink = Function::declaration("consume64");
        sink.address = Some(0x2000);
        sink.locations = true;
        let result = extract(
            &[caller, sink],
            &[
                (0x1000, "caller64".into(), vec!["caller_alias".into()]),
                (0x2000, "consume64".into(), vec![]),
                (0x3000, "produce_sum".into(), vec![]),
            ],
            0,
        )
        .expect("complete explicit contracts");
        assert_eq!(result.len(), 2);
        for (index, name) in ["caller64", "consume64"].into_iter().enumerate() {
            assert_eq!(result[index].0, (index as u64 + 1) * 0x1000);
            assert_eq!(result[index].1.name, name);
            assert_eq!(result[index].1.outtype.as_ref().unwrap().get_size(), 8);
            assert_eq!(result[index].1.intypes.len(), 1);
            assert_eq!(result[index].1.intypes[0].get_size(), 8);
        }
    }

    #[test]
    fn ambiguous_names_and_mismatched_definition_addresses_refuse_every_fact() {
        let entries = [(0x1000, "caller64".into(), vec![])];
        assert!(extract(
            &[
                Function::declaration("caller64"),
                Function::declaration("caller64")
            ],
            &entries,
            0
        )
        .is_none());
        let mut definition = Function::declaration("caller64");
        definition.address = Some(0x2000);
        assert!(extract(&[definition], &entries, 0).is_none());
        assert!(names(&[
            (0x1000, "one".into(), vec!["same".into()]),
            (0x2000, "two".into(), vec!["same".into()])
        ])
        .is_none());
    }

    #[test]
    fn custom_abi_incomplete_types_and_unprototyped_dies_supply_no_origin() {
        for invalid in 0..4 {
            let mut function = Function::declaration("caller64");
            match invalid {
                0 => function.convention = gimli::DW_CC_nocall.0,
                1 => function.prototyped = false,
                2 => function.missing_type = true,
                _ => function.locations = true,
            }
            assert!(
                extract(&[function], &[(0x1000, "caller64".into(), vec![])], 0)
                    .unwrap()
                    .is_empty()
            );
        }
    }

    #[test]
    fn nonstandard_scalar_representations_supply_no_width_evidence() {
        let entries = [(0x1000, "caller64".into(), vec![])];
        for attribute in [
            gimli::DW_AT_bit_size,
            gimli::DW_AT_bit_offset,
            gimli::DW_AT_data_bit_offset,
            gimli::DW_AT_endianity,
            gimli::DW_AT_address_class,
            gimli::DW_AT_alignment,
        ] {
            assert!(
                extract_representation(
                    &[Function::declaration("caller64")],
                    &entries,
                    0,
                    Some(attribute),
                    gimli::DW_ATE_unsigned,
                    8,
                )
                .unwrap()
                .is_empty(),
                "{attribute:?}"
            );
        }
        assert!(extract_representation(
            &[Function::declaration("caller64")],
            &entries,
            0,
            None,
            gimli::DW_ATE_boolean,
            8,
        )
        .unwrap()
        .is_empty());

        for size in [0, 3, 5, 6, 7, 9, 16, 32, 255] {
            assert!(
                extract_representation(
                    &[Function::declaration("caller64")],
                    &entries,
                    0,
                    None,
                    gimli::DW_ATE_unsigned,
                    size,
                )
                .unwrap()
                .is_empty(),
                "noncanonical integer size {size}"
            );
        }
        for size in [1, 2, 4, 8] {
            let result = extract_representation(
                &[Function::declaration("caller64")],
                &entries,
                0,
                None,
                gimli::DW_ATE_unsigned,
                size,
            )
            .unwrap();
            assert_eq!(result.len(), 1);
            assert_eq!(
                result[0].1.outtype.as_ref().unwrap().get_size(),
                i32::from(size)
            );
        }

        let mut base = DieSnap::new(gimli::DW_TAG_base_type, 1);
        base.byte_size = Some(8);
        base.encoding = Some(gimli::DW_ATE_unsigned);
        let mut alias = DieSnap::new(gimli::DW_TAG_typedef, 1);
        alias.type_ref = Some(1);
        let dies = BTreeMap::from([(1, base), (2, alias)]);
        for offset in [1, 2] {
            assert!(scalar(
                2,
                &dies,
                &BTreeSet::from([offset]),
                &mut TypeBudget::default(),
            )
            .is_none());
        }
    }

    #[test]
    fn record_and_formal_limits_refuse_without_publishing_a_prefix() {
        let entries = [
            (0x1000, "first".into(), vec![]),
            (0x2000, "second".into(), vec![]),
        ];
        assert!(extract(&[Function::declaration("first")], &entries, MAX_RECORDS).is_none());
        let mut oversized = Function::declaration("second");
        oversized.formals = MAX_FORMALS + 1;
        assert!(extract(&[Function::declaration("first"), oversized], &entries, 0).is_none());
        assert!(names(&[(0x1000, "x".repeat(MAX_NAME + 1), vec![])]).is_none());
    }
}
