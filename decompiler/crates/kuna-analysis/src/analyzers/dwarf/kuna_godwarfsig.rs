//! Go's cross-unit type references and input/result formals, committed under godwarfsig.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::rc::Rc;

use kuna_decomp::architecture::Architecture;
use kuna_decomp::dtype::{type_metatype, Datatype};
use kuna_decomp::fspec::{ParameterPieces, PrototypePieces};
use kuna_decomp::kuna_symbolnamechars::{sanitize_symbol_name, NameChars};

use super::{build_datatype, kuna_dwarfbyref, DieSnap, Reader, TypeWalk};
use object::{Object, ObjectSection};

const INTEGER_REGISTERS: &[(u16, &str)] = &[
    (0, "RAX"), (3, "RBX"), (2, "RCX"), (5, "RDI"), (4, "RSI"),
    (8, "R8"), (9, "R9"), (10, "R10"), (11, "R11"),
];
const FLOAT_REGISTERS: &[(u16, &str)] = &[
    (17, "XMM0_Qa"), (18, "XMM1_Qa"), (19, "XMM2_Qa"), (20, "XMM3_Qa"),
    (21, "XMM4_Qa"), (22, "XMM5_Qa"), (23, "XMM6_Qa"), (24, "XMM7_Qa"),
];

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum Location {
    #[default]
    Unsupported,
    Unavailable,
    Register(u16),
    EmptyEntryRegister(u16),
}

#[derive(Clone, Default)]
struct Extra {
    type_ref: Option<usize>,
    origin_ref: Option<usize>,
    result: Option<bool>,
    kind: Option<u64>,
    location: Location,
    regabi: bool,
    trampoline: bool,
    closure: bool,
    invalid: bool,
    code_size: Option<u64>,
}

#[derive(Default)]
pub(super) struct UnitExtras {
    base: Option<usize>,
    go: bool,
    regabi: bool,
    entries: BTreeMap<usize, Extra>,
    pub(super) complete: bool,
}

impl UnitExtras {
    pub(super) fn enabled(&self) -> bool {
        self.go
    }

    pub(super) fn capture(
        &mut self,
        dwarf: &gimli::Dwarf<Reader<'_>>,
        unit: &gimli::Unit<Reader<'_>>,
        entry: &gimli::DebuggingInformationEntry<Reader<'_>>,
        function_entry: Option<u64>,
    ) {
        if entry.depth() == 0 {
            self.base = unit.header.debug_info_offset().map(|offset| offset.0);
            self.go = unit.encoding().address_size == 8 && self.base.is_some()
                && matches!(entry.attr_value(gimli::DW_AT_language),
                    Some(gimli::AttributeValue::Language(gimli::DW_LANG_Go)));
            self.regabi = entry.attr_value(gimli::DW_AT_producer)
                .and_then(|value| dwarf.attr_string(unit, value).ok())
                .and_then(|value| std::str::from_utf8(value.slice()).ok().map(str::to_owned))
                .is_some_and(|producer| producer.starts_with("Go cmd/compile ")
                    && producer.split_whitespace().any(|word| word == "regabi"));
        }
        if !self.go {
            return;
        }
        let reference = |attribute| match entry.attr_value(attribute) {
            Some(gimli::AttributeValue::UnitRef(offset)) => self.base?.checked_add(offset.0),
            Some(gimli::AttributeValue::DebugInfoRef(offset)) => Some(offset.0),
            _ => None,
        };
        let location = if entry.tag() == gimli::DW_TAG_formal_parameter {
            entry.attr_value(gimli::DW_AT_location)
                .map(|location| {
                    if let Some(expression) = kuna_dwarfbyref::location_at_entry(dwarf, unit, &location, function_entry) {
                        simple_location(expression, unit.encoding())
                    } else {
                        empty_entry_location(dwarf, unit, &location, function_entry).unwrap_or_default()
                    }
                }).unwrap_or_default()
        } else {
            Location::Unsupported
        };
        self.entries.insert(entry.offset().0, Extra {
            type_ref: reference(gimli::DW_AT_type),
            origin_ref: reference(gimli::DW_AT_abstract_origin),
            result: match entry.attr_value(gimli::DW_AT_variable_parameter) {
                Some(gimli::AttributeValue::Flag(value)) => Some(value),
                _ => None,
            },
            kind: entry.attr_value(gimli::DwAt(0x2900)).and_then(|value| value.udata_value()),
            location,
            regabi: self.regabi,
            trampoline: entry.attr_value(gimli::DW_AT_trampoline).is_some(),
            closure: entry.attr_value(gimli::DwAt(0x2907)).is_some(),
            invalid: [gimli::DW_AT_type, gimli::DW_AT_abstract_origin].iter()
                .any(|attribute| entry.attr_value(*attribute).is_some() && reference(*attribute).is_none())
                || entry.attr_value(gimli::DW_AT_specification).is_some(),
            code_size: (entry.tag() == gimli::DW_TAG_subprogram).then(|| {
                let low = dwarf.attr_address(unit, entry.attr_value(gimli::DW_AT_low_pc)?).ok()??;
                let high = entry.attr_value(gimli::DW_AT_high_pc)?;
                match dwarf.attr_address(unit, high.clone()).ok()? {
                    Some(address) => address.checked_sub(low),
                    None => high.udata_value(),
                }
            }).flatten(),
        });
    }
}

fn simple_location(expression: gimli::Expression<Reader<'_>>, encoding: gimli::Encoding) -> Location {
    let mut operations = expression.operations(encoding);
    match (operations.next(), operations.next()) {
        (Ok(None), Ok(None)) => Location::Unavailable,
        (Ok(Some(gimli::Operation::Register { register })), Ok(None)) => Location::Register(register.0),
        _ => Location::Unsupported,
    }
}

/// Go 1.27 can give RET-only inputs an empty entry range; materialization also checks the body.
fn empty_entry_location(
    dwarf: &gimli::Dwarf<Reader<'_>>,
    unit: &gimli::Unit<Reader<'_>>,
    location: &gimli::AttributeValue<Reader<'_>>,
    entry: Option<u64>,
) -> Option<Location> {
    if unit.encoding().version != 5 { return None; }
    let entry = entry?;
    let mut locations = dwarf.attr_locations(unit, location.clone()).ok()??;
    let base = match locations.next_raw().ok()?? {
        gimli::RawLocListEntry::BaseAddress { addr } => addr,
        gimli::RawLocListEntry::BaseAddressx { addr } => dwarf.address(unit, addr).ok()?,
        _ => return None,
    };
    if base != entry { return None; }
    let gimli::RawLocListEntry::OffsetPair { begin: 0, end: 0, data } = locations.next_raw().ok()?? else {
        return None;
    };
    if locations.next_raw().ok()?.is_some() { return None; }
    match simple_location(data, unit.encoding()) {
        Location::Register(register) => Some(Location::EmptyEntryRegister(register)),
        _ => None,
    }
}

/// Raw Go metadata; type interning is deferred until the option is enabled.
pub struct Facts {
    dies: BTreeMap<usize, DieSnap>,
    extras: BTreeMap<usize, Extra>,
    names: NameChars,
    compressed: Option<Deferred>,
}

impl Default for Facts {
    fn default() -> Self {
        Self {
            dies: BTreeMap::new(), extras: BTreeMap::new(), names: NameChars::Off,
            compressed: None,
        }
    }
}

impl fmt::Debug for Facts {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GoDwarfFacts").field("entries", &self.dies.len()).finish()
    }
}

impl Facts {
    pub fn sanitize_names(&mut self, mode: NameChars) {
        self.names = mode;
    }

    pub(super) fn append_unit(&mut self, dies: BTreeMap<usize, DieSnap>, extra: UnitExtras) {
        if !extra.go || !extra.complete {
            return;
        }
        let Some(base) = extra.base else { return };
        for (offset, mut die) in dies {
            let Some(absolute) = base.checked_add(offset) else { continue };
            let Some(attributes) = extra.entries.get(&offset) else { continue };
            die.type_ref = attributes.type_ref;
            die.origin_ref = attributes.origin_ref;
            die.origin_is_spec = false;
            die.parent = die.parent.and_then(|parent| base.checked_add(parent));
            die.discr_ref = die.discr_ref.and_then(|value| base.checked_add(value));
            let Some(children) = die.children.iter().map(|child| base.checked_add(*child))
                .collect::<Option<Vec<_>>>() else { continue };
            die.children = children;
            self.dies.insert(absolute, die);
            self.extras.insert(absolute, attributes.clone());
        }
    }

    pub(super) fn is_empty(&self) -> bool {
        self.dies.is_empty()
    }

    pub fn materialize(&self, arch: &Architecture) -> Vec<(u64, PrototypePieces)> {
        if !arch.archid.starts_with("x86:LE:64:") || arch.data_org().0 != 8
            || !arch.default_fp().is_some_and(|model| model.get_name() == "abi-internal")
        {
            return Vec::new();
        }
        if let Some(compressed) = &self.compressed {
            return compressed.decode(self.names).map(|facts| facts.materialize(arch)).unwrap_or_default();
        }
        self.dies.iter().filter_map(|(offset, die)| {
            if die.tag != gimli::DW_TAG_subprogram || die.declaration {
                return None;
            }
            Some((die.low_pc?, self.prototype(*offset, arch)?))
        }).collect()
    }

    fn prototype(&self, offset: usize, arch: &Architecture) -> Option<PrototypePieces> {
        let sub = self.dies.get(&offset)?;
        let extra = self.extras.get(&offset)?;
        if !extra.regabi || extra.trampoline || extra.closure || extra.invalid || sub.type_ref.is_some() {
            return None;
        }
        let origin = match sub.origin_ref {
            Some(offset) => {
                let attributes = self.extras.get(&offset)?;
                if !attributes.regabi || attributes.trampoline || attributes.closure || attributes.invalid {
                    return None;
                }
                Some(self.dies.get(&offset)?)
            }
            None => None,
        };
        if origin.is_some_and(|origin| origin.tag != gimli::DW_TAG_subprogram
            || origin.low_pc.is_some() || origin.origin_ref.is_some() || origin.type_ref.is_some()
            || origin.children.iter().any(|child| self.dies.get(child)
                .is_some_and(|die| die.tag == gimli::DW_TAG_unspecified_parameters)))
        {
            return None;
        }
        let name = if sub.name.is_empty() { &origin?.name } else { &sub.name };
        if name.is_empty() || hidden_context(name) {
            return None;
        }
        let inherited = origin.map(|origin| origin.children.iter().copied().filter(|offset| {
            self.dies.get(offset).is_some_and(|die| die.tag == gimli::DW_TAG_formal_parameter)
        }).collect::<Vec<_>>()).unwrap_or_default();
        let mut seen = Vec::new();
        let mut parameters = Vec::new();
        for child in &sub.children {
            let die = self.dies.get(child)?;
            if die.tag == gimli::DW_TAG_unspecified_parameters {
                return None;
            }
            if die.tag != gimli::DW_TAG_formal_parameter {
                continue;
            }
            let attributes = self.extras.get(child)?;
            if attributes.closure || attributes.invalid {
                return None;
            }
            let (source, result) = if let Some(parent) = die.origin_ref {
                let source = self.dies.get(&parent)?;
                let inherited_attributes = self.extras.get(&parent)?;
                if inherited_attributes.invalid || inherited_attributes.closure
                    || source.tag != gimli::DW_TAG_formal_parameter || source.origin_ref.is_some()
                    || source.parent != sub.origin_ref || !inherited.contains(&parent)
                    || (!die.name.is_empty() && die.name != source.name)
                    || die.type_ref.is_some_and(|ty| Some(ty) != source.type_ref)
                    || attributes.result.is_some_and(|result| Some(result) != inherited_attributes.result)
                {
                    return None;
                }
                seen.push(parent);
                (source, inherited_attributes.result?)
            } else {
                let result = attributes.result?;
                if origin.is_some() && !result {
                    return None;
                }
                (die, result)
            };
            parameters.push((source, attributes.location, result));
        }
        if parameters.is_empty() || seen != inherited {
            return None;
        }
        if parameters.iter().any(|(_, location, _)| matches!(location, Location::EmptyEntryRegister(_))) {
            if extra.code_size != Some(1) { return None; }
            let address = kuna_base::address::Address::new(arch.manage().get_default_code_space()?.clone(), sub.low_pc?);
            let mut opcode = [0u8];
            arch.translate().loader_rc().borrow_mut().load_fill(&mut opcode, &address).ok()?;
            if opcode != [0xc3] { return None; }
        }
        let mut expected = Vec::new();
        let mut inputs = Vec::new();
        let mut names = Vec::new();
        let mut output = None;
        let (mut integers, mut floats) = (0, 0);
        for (parameter, location, result) in parameters {
            let location = match location {
                Location::EmptyEntryRegister(register) if !result => Location::Register(register),
                Location::EmptyEntryRegister(_) => return None,
                location => location,
            };
            let scalar = self.scalar(parameter.type_ref?, &mut BTreeSet::new(), &mut 256)?;
            let registers = if scalar.float { FLOAT_REGISTERS } else { INTEGER_REGISTERS };
            if result {
                if output.is_some() || (location != Location::Unavailable
                    && location != Location::Register(registers[0].0)) { return None; }
                output = Some((parameter.type_ref?, scalar, registers[0].1));
            } else {
                if output.is_some() { return None; }
                let index = if scalar.float { &mut floats } else { &mut integers };
                let &(dwarf_register, register) = registers.get(*index)?;
                *index += 1;
                if !matches!(location, Location::Unavailable)
                    && location != Location::Register(dwarf_register)
                {
                    return None;
                }
                inputs.push((parameter.type_ref?, scalar));
                names.push(sanitize_symbol_name(&parameter.name, self.names).into_owned());
                expected.push(register);
            }
        }
        let types = arch.types();
        let mut walk = TypeWalk::new();
        let mut materialize = |offset, scalar: Scalar| {
            let ty = build_datatype(Some(offset), &self.dies, types, 1, &mut walk, false)?;
            (ty.get_size() == scalar.size && ty.get_metatype() == scalar.meta
                && self.built_matches(offset, &ty, 64)).then_some(ty)
        };
        let intypes = inputs.into_iter().map(|(offset, scalar)| materialize(offset, scalar))
            .collect::<Option<Vec<_>>>()?;
        let outtype = match output {
            Some((offset, scalar, _)) => materialize(offset, scalar)?,
            None => types.get_type_void().ok()?,
        };
        let pieces = PrototypePieces {
            name: sanitize_symbol_name(name, self.names).into_owned(),
            outtype: Some(outtype), intypes, innames: names, first_var_arg_slot: -1,
            output_storage: None, input_storage: Vec::new(), kuna_library_names: None,
        };
        let mut assigned = Vec::new();
        arch.default_fp()?.assign_parameter_storage(&pieces, &mut assigned, false, types, arch.manage()).ok()?;
        if assigned.len() != expected.len() + 1 { return None; }
        for ((slot, register), ty) in assigned.iter().skip(1).zip(expected).zip(&pieces.intypes) {
            if !storage_matches(arch, slot, register, ty) { return None; }
        }
        if let Some((_, _, register)) = output {
            if !storage_matches(arch, &assigned[0], register, pieces.outtype.as_ref()?) { return None; }
        }
        Some(pieces)
    }

    fn scalar(&self, offset: usize, visited: &mut BTreeSet<usize>, budget: &mut usize) -> Option<Scalar> {
        if *budget == 0 || visited.len() >= 64 || !visited.insert(offset) { return None; }
        *budget -= 1;
        let die = self.dies.get(&offset)?;
        let extra = self.extras.get(&offset)?;
        if extra.invalid { return None; }
        let kind = extra.kind;
        let size = i32::try_from(die.byte_size.unwrap_or(
            if die.tag == gimli::DW_TAG_pointer_type { 8 } else { 0 }
        )).ok()?;
        let meta = match die.tag {
            gimli::DW_TAG_typedef | gimli::DW_TAG_const_type | gimli::DW_TAG_volatile_type => {
                let scalar = self.scalar(die.type_ref?, visited, budget)?;
                return (kind.is_none_or(|kind| kind_matches(kind, scalar.meta, scalar.size))
                    && die.byte_size.is_none_or(|size| size == scalar.size as u64)).then_some(scalar);
            }
            gimli::DW_TAG_base_type => match die.encoding? {
                gimli::DW_ATE_signed if matches!(size, 1 | 2 | 4 | 8) => type_metatype::TYPE_INT,
                gimli::DW_ATE_unsigned if matches!(size, 1 | 2 | 4 | 8) => type_metatype::TYPE_UINT,
                gimli::DW_ATE_boolean if size == 1 => type_metatype::TYPE_BOOL,
                gimli::DW_ATE_float if matches!(size, 4 | 8) => type_metatype::TYPE_FLOAT,
                _ => return None,
            },
            gimli::DW_TAG_pointer_type if size == 8 && matches!(kind, None | Some(22 | 26)) => {
                match die.type_ref {
                    Some(pointee) => {
                        if self.scalar(pointee, &mut visited.clone(), budget).is_none() {
                            self.record_pointee(pointee, visited, budget)?;
                        }
                    }
                    None if kind == Some(26) || (kind.is_none() && die.name == "unsafe.Pointer") => {}
                    None => return None,
                }
                type_metatype::TYPE_PTR
            }
            _ => return None,
        };
        if let Some(kind) = kind {
            if !kind_matches(kind, meta, size) { return None; }
        }
        Some(Scalar { size, meta, float: meta == type_metatype::TYPE_FLOAT })
    }

    fn record_pointee(&self, offset: usize, visited: &mut BTreeSet<usize>, budget: &mut usize) -> Option<()> {
        if *budget == 0 || visited.len() >= 64 || !visited.insert(offset) { return None; }
        *budget -= 1;
        let die = self.dies.get(&offset)?;
        let extra = self.extras.get(&offset)?;
        if extra.invalid || !matches!(extra.kind, None | Some(25)) { return None; }
        if die.tag == gimli::DW_TAG_typedef {
            return self.record_pointee(die.type_ref?, visited, budget);
        }
        let size = die.byte_size?;
        if die.tag != gimli::DW_TAG_structure_type || die.name.is_empty()
            || size == 0 || size > i32::MAX as u64
            || !matches!(self.extras.get(&offset)?.kind, None | Some(25))
        {
            return None;
        }
        let mut end = 0;
        for child in &die.children {
            let member = self.dies.get(child)?;
            if member.tag != gimli::DW_TAG_member || member.name.is_empty()
                || member.bit_size.is_some() || member.bit_offset.is_some()
                || member.data_bit_offset.is_some()
            {
                return None;
            }
            let field = self.scalar(member.type_ref?, &mut visited.clone(), budget)?;
            let begin = u64::try_from(member.data_member_location?).ok()?;
            if begin < end { return None; }
            end = begin.checked_add(field.size as u64)?;
            if end > size { return None; }
        }
        (!die.children.is_empty()).then_some(())
    }

    fn built_matches(&self, offset: usize, ty: &Datatype, depth: usize) -> bool {
        if depth == 0 { return false; }
        let Some(die) = self.dies.get(&offset) else { return false };
        match die.tag {
            gimli::DW_TAG_typedef | gimli::DW_TAG_const_type | gimli::DW_TAG_volatile_type => {
                die.type_ref.is_some_and(|next| self.built_matches(next, ty, depth - 1))
            }
            gimli::DW_TAG_pointer_type => ty.get_metatype() == type_metatype::TYPE_PTR
                && ty.get_size() == 8 && ty.get_ptr_to().is_some_and(|inner| match die.type_ref {
                    Some(next) => self.built_matches(next, &inner, depth - 1),
                    None => inner.get_metatype() == type_metatype::TYPE_VOID,
                }),
            gimli::DW_TAG_structure_type => ty.get_metatype() == type_metatype::TYPE_STRUCT
                && die.byte_size == Some(ty.get_size() as u64)
                && ty.get_field(die.children.len() as i32).is_none()
                && die.children.iter().enumerate().all(|(index, child)| {
                    let Some(member) = self.dies.get(child) else { return false };
                    let Some(field) = ty.get_field(index as i32) else { return false };
                    member.data_member_location == Some(i64::from(field.offset))
                        && member.type_ref.is_some_and(|next| self.built_matches(next, &field.field_type, depth - 1))
                }),
            gimli::DW_TAG_base_type => self.scalar(offset, &mut BTreeSet::new(), &mut 256)
                .is_some_and(|scalar| scalar.meta == ty.get_metatype() && scalar.size == ty.get_size()),
            _ => false,
        }
    }
}

const DEBUG_LIMIT: usize = 64 * 1024 * 1024;

struct StoredSection {
    format: object::CompressionFormat,
    bytes: Vec<u8>,
    size: usize,
}

struct Deferred {
    sections: Vec<(gimli::SectionId, StoredSection)>,
}

impl Deferred {
    fn decode(&self, names: NameChars) -> Option<Facts> {
        let sections = gimli::DwarfSections::load(|id| -> Result<Vec<u8>, ()> {
            let Some((_, stored)) = self.sections.iter().find(|(section, _)| *section == id) else {
                return Ok(Vec::new());
            };
            if stored.format == object::CompressionFormat::None {
                return Ok(stored.bytes.clone());
            }
            if stored.size > DEBUG_LIMIT { return Err(()); }
            let mut decoder = flate2::Decompress::new(true);
            let mut bytes = Vec::new();
            bytes.try_reserve_exact(stored.size + 1).map_err(|_| ())?;
            let status = decoder.decompress_vec(&stored.bytes, &mut bytes, flate2::FlushDecompress::Finish)
                .map_err(|_| ())?;
            if status != flate2::Status::StreamEnd || bytes.len() != stored.size
                || decoder.total_in() != stored.bytes.len() as u64
            {
                return Err(());
            }
            Ok(bytes)
        }).ok()?;
        let dwarf = sections.borrow(|bytes| gimli::EndianSlice::new(bytes, gimli::RunTimeEndian::Little));
        let mut facts = Facts { names, ..Facts::default() };
        let mut units = dwarf.units();
        while let Some(header) = units.next().ok()? {
            let unit = dwarf.unit(header).ok()?;
            let mut extra = UnitExtras::default();
            let dies = super::snapshot_unit_with_go(&dwarf, &unit,
                kuna_dwarfbyref::ArgumentAbi::default(), Some(&mut extra));
            facts.append_unit(dies, extra);
        }
        Some(facts)
    }
}

pub(super) fn capture_compressed(ctx: &crate::pass::AnalysisCtx<'_>) -> Option<Facts> {
    if !ctx.file.is_little_endian() || ctx.file.architecture() != object::Architecture::X86_64 {
        return None;
    }
    let section = |id: gimli::SectionId| ctx.file.section_by_name(id.name())
        .or_else(|| ctx.file.section_by_name(&id.name().replacen(".debug_", ".zdebug_", 1)));
    if section(gimli::SectionId::DebugInfo)?.compressed_data().ok()?.format != object::CompressionFormat::Zlib
        || crate::sourcelang::detect_compiler(ctx.file, ctx.bytes) != crate::sourcelang::Compiler::Go
    {
        return None;
    }
    Some(Facts { compressed: Some(capture_sections(ctx.file)?), ..Facts::default() })
}

fn capture_sections(file: &object::File<'_>) -> Option<Deferred> {
    let section = |id: gimli::SectionId| file.section_by_name(id.name())
        .or_else(|| file.section_by_name(&id.name().replacen(".debug_", ".zdebug_", 1)));
    let mut captured = Vec::new();
    let mut total = 0usize;
    gimli::DwarfSections::load(|id| -> Result<(), ()> {
        let Some(section) = section(id) else { return Ok(()) };
        let data = section.compressed_data().map_err(|_| ())?;
        if !matches!(data.format, object::CompressionFormat::None | object::CompressionFormat::Zlib) {
            return Err(());
        }
        let size = usize::try_from(data.uncompressed_size).map_err(|_| ())?;
        total = total.checked_add(size.max(data.data.len())).ok_or(())?;
        if total > DEBUG_LIMIT { return Err(()); }
        captured.push((id, StoredSection { format: data.format, bytes: data.data.to_vec(), size }));
        Ok(())
    }).ok()?;
    Some(Deferred { sections: captured })
}

#[derive(Clone, Copy)]
struct Scalar {
    size: i32,
    meta: type_metatype,
    float: bool,
}

fn kind_matches(kind: u64, meta: type_metatype, size: i32) -> bool {
    let expected = match kind {
        1 => (type_metatype::TYPE_BOOL, 1),
        2 | 6 => (type_metatype::TYPE_INT, 8),
        3 => (type_metatype::TYPE_INT, 1),
        4 => (type_metatype::TYPE_INT, 2),
        5 => (type_metatype::TYPE_INT, 4),
        7 | 11 | 12 => (type_metatype::TYPE_UINT, 8),
        8 => (type_metatype::TYPE_UINT, 1),
        9 => (type_metatype::TYPE_UINT, 2),
        10 => (type_metatype::TYPE_UINT, 4),
        13 => (type_metatype::TYPE_FLOAT, 4),
        14 => (type_metatype::TYPE_FLOAT, 8),
        22 | 26 => (type_metatype::TYPE_PTR, 8),
        _ => return false,
    };
    expected == (meta, size)
}

fn storage_matches(arch: &Architecture, slot: &ParameterPieces, register: &str, ty: &Rc<Datatype>) -> bool {
    let Ok(expected) = arch.get_register_varnode(register.as_bytes()) else { return false };
    slot.flags == 0 && slot.type_.as_ref().is_some_and(|actual| actual.get_size() == ty.get_size())
        && slot.addr.get_offset() == expected.offset
        && slot.addr.get_space().zip(expected.space.as_ref())
            .is_some_and(|(space, expected)| space.get_index() == expected.get_index())
}

fn hidden_context(name: &str) -> bool {
    name.ends_with("-fm") || name.ends_with(".abi0")
        || [".func", ".gowrap", ".deferwrap", "-range"].iter().any(|marker| {
            name.rsplit_once(marker).is_some_and(|(_, tail)| tail.split('.').all(|part| {
                !part.is_empty() && part.bytes().all(|byte| byte.is_ascii_digit())
            }))
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn insert(facts: &mut Facts, offset: usize, die: DieSnap, kind: Option<u64>) {
        facts.dies.insert(offset, die);
        facts.extras.insert(offset, Extra { kind, ..Extra::default() });
    }
    fn scalar_facts() -> Facts {
        let mut facts = Facts::default();
        let mut scalar = DieSnap::new(gimli::DW_TAG_base_type, 1);
        scalar.byte_size = Some(8);
        scalar.encoding = Some(gimli::DW_ATE_signed);
        insert(&mut facts, 1, scalar, Some(6));
        facts
    }
    fn accepts(facts: &Facts, offset: usize) -> bool {
        facts.scalar(offset, &mut BTreeSet::new(), &mut 256).is_some()
    }

    #[test]
    fn malformed_scalar_graphs_are_refused() {
        let mut facts = scalar_facts();
        assert!(accepts(&facts, 1));
        let mut alias = DieSnap::new(gimli::DW_TAG_typedef, 1);
        alias.type_ref = Some(1);
        insert(&mut facts, 2, alias, Some(6));
        assert!(accepts(&facts, 2));
        facts.extras.get_mut(&2).unwrap().kind = Some(21);
        assert!(!accepts(&facts, 2), "a map kind cannot alias an integer");
        facts.extras.get_mut(&2).unwrap().kind = None;
        facts.dies.get_mut(&2).unwrap().type_ref = Some(2);
        assert!(!accepts(&facts, 2), "typedef cycle");
        let mut pointer = DieSnap::new(gimli::DW_TAG_pointer_type, 1);
        pointer.type_ref = Some(3);
        insert(&mut facts, 3, pointer, Some(22));
        assert!(!accepts(&facts, 3), "pointer cycle");
        facts.dies.get_mut(&3).unwrap().type_ref = Some(9999);
        assert!(!accepts(&facts, 3), "missing target");
        facts.dies.get_mut(&3).unwrap().type_ref = None;
        assert!(!accepts(&facts, 3), "untyped pointer without unsafe.Pointer evidence");
        facts.dies.get_mut(&3).unwrap().name = "unsafe.Pointer".into();
        facts.extras.get_mut(&3).unwrap().kind = None;
        assert!(accepts(&facts, 3));
        facts.extras.get_mut(&3).unwrap().invalid = true;
        assert!(!accepts(&facts, 3));
        facts.dies.get_mut(&1).unwrap().byte_size = Some(16);
        assert!(!accepts(&facts, 1));
    }

    #[test]
    fn record_pointees_require_grounded_nonoverlapping_fields() {
        let mut facts = scalar_facts();
        let mut member = DieSnap::new(gimli::DW_TAG_member, 2);
        member.name = "value".into();
        member.type_ref = Some(1);
        member.data_member_location = Some(0);
        insert(&mut facts, 3, member, None);
        let mut record = DieSnap::new(gimli::DW_TAG_structure_type, 1);
        record.name = "main.Record".into();
        record.byte_size = Some(8);
        record.children = vec![3];
        insert(&mut facts, 2, record, Some(25));
        let mut pointer = DieSnap::new(gimli::DW_TAG_pointer_type, 1);
        pointer.type_ref = Some(2);
        insert(&mut facts, 4, pointer, Some(22));
        assert!(accepts(&facts, 4));
        assert!(!accepts(&facts, 2), "by-value records remain unsupported");
        facts.dies.get_mut(&3).unwrap().data_member_location = Some(1);
        assert!(!accepts(&facts, 4), "field outside the record");
        facts.dies.get_mut(&3).unwrap().data_member_location = Some(0);
        facts.dies.get_mut(&3).unwrap().bit_size = Some(2);
        assert!(!accepts(&facts, 4), "bitfield");
        facts.dies.get_mut(&3).unwrap().bit_size = None;
        facts.dies.get_mut(&2).unwrap().children.push(3);
        assert!(!accepts(&facts, 4), "overlapping fields");
        facts.dies.get_mut(&2).unwrap().children.pop();
        facts.dies.get_mut(&3).unwrap().type_ref = Some(4);
        assert!(!accepts(&facts, 4), "recursive record terminates");
    }

    fn compressed(bytes: &[u8]) -> Deferred {
        let mut encoder = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::default());
        encoder.write_all(bytes).unwrap();
        Deferred { sections: vec![(gimli::SectionId::DebugInfo, StoredSection {
            bytes: encoder.finish().unwrap(), size: bytes.len(), format: object::CompressionFormat::Zlib,
        })] }
    }

    #[test]
    fn compressed_debug_requires_exact_size_checksum_and_stream() {
        let empty = compressed(&[]);
        assert!(empty.decode(NameChars::Off).unwrap().is_empty());
        let mut larger = compressed(&[]);
        larger.sections[0].1.size = 1;
        assert!(larger.decode(NameChars::Off).is_none());
        let mut smaller = compressed(&[0; 4096]);
        smaller.sections[0].1.size = 1;
        assert!(smaller.decode(NameChars::Off).is_none());
        let mut trailing = compressed(&[]);
        trailing.sections[0].1.bytes.push(0);
        assert!(trailing.decode(NameChars::Off).is_none());
        let mut corrupt = compressed(&[]);
        *corrupt.sections[0].1.bytes.last_mut().unwrap() ^= 1;
        assert!(corrupt.decode(NameChars::Off).is_none());
        let mut truncated = compressed(&[]);
        truncated.sections[0].1.bytes.pop();
        assert!(truncated.decode(NameChars::Off).is_none());
        assert!(compressed(&[1, 2, 3]).decode(NameChars::Off).is_none());
    }

    #[test]
    fn compressed_section_limits_precede_decompression() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../../tests/fixtures/go-dwarf-signatures/compressed");
        let mut bytes = std::fs::read(path).unwrap();
        let file = object::File::parse(bytes.as_slice()).unwrap();
        let captured = capture_sections(&file).unwrap();
        let facts = captured.decode(NameChars::Off).unwrap();
        assert!(facts.dies.values().any(|die| die.name == "main.fromWord"));
        let offset = file.section_by_name(".debug_info").unwrap().file_range().unwrap().0 as usize;
        bytes[offset + 8..offset + 16].copy_from_slice(&(DEBUG_LIMIT as u64 + 1).to_le_bytes());
        let oversized = object::File::parse(bytes.as_slice()).unwrap();
        assert!(capture_sections(&oversized).is_none());
    }

    #[test]
    fn hidden_context_suffixes_do_not_match_ordinary_names() {
        for name in ["main.f.func1", "main.f.func1.2", "main.f.gowrap2", "main.f.deferwrap1", "main.f-range1", "main.f-fm", "main.f.abi0"] {
            assert!(hidden_context(name), "{name}");
        }
        for name in ["main.function", "main.func1x", "main.f-range", "main.f.func1.", "main.Func1"] {
            assert!(!hidden_context(name), "{name}");
        }
    }
}
