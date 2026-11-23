//! Select exact named AArch64 Go signatures without changing generic DWARF imports.

use std::collections::BTreeMap;

use object::{CompressionFormat, Object, ObjectSection, ObjectSymbol};

use super::super::{DieSnap, Reader};
use super::{
    capture_sections, hidden_context, Deferred, Extra, Facts, NameChars, TextSymbols, UnitExtras,
};
use crate::pass::AnalysisCtx;

fn text_entries(file: &object::File<'_>) -> TextSymbols {
    let mut entries = TextSymbols::new();
    for symbol in file
        .symbols()
        .filter(|symbol| symbol.kind() == object::SymbolKind::Text && symbol.is_definition())
    {
        let (entry, size) = (symbol.address(), symbol.size());
        if entry % 4 != 0 || size == 0 || size % 4 != 0 {
            continue;
        }
        let Some(section) = symbol
            .section_index()
            .and_then(|index| file.section_by_index(index).ok())
        else {
            continue;
        };
        if section.kind() != object::SectionKind::Text
            || !entry
                .checked_sub(section.address())
                .and_then(|offset| offset.checked_add(size))
                .is_some_and(|end| end <= section.size())
        {
            continue;
        }
        let Ok(name) = symbol.name() else { continue };
        entries
            .entry(entry)
            .and_modify(|identity| *identity = None)
            .or_insert_with(|| Some((name.to_owned(), size)));
    }
    entries
}

pub(crate) fn capture_compressed(ctx: &AnalysisCtx<'_>) -> Option<Facts> {
    if !ctx.arch.analysis_godwarfsig
        || !ctx.file.is_little_endian()
        || ctx.file.architecture() != object::Architecture::Aarch64
        || ctx.file.format() != object::BinaryFormat::Elf
    {
        return None;
    }
    let section = ctx
        .file
        .section_by_name(gimli::SectionId::DebugInfo.name())?;
    if !matches!(section.compressed_data().ok()?.format, CompressionFormat::Zlib | CompressionFormat::None)
        || ctx.file.symbol_table().is_none()
        || crate::sourcelang::detect_compiler_elf(ctx.file) != crate::sourcelang::Compiler::Go
    {
        return None;
    }
    let arm64_entries = text_entries(ctx.file);
    if arm64_entries.is_empty() {
        return None;
    }
    let mut compressed = capture_sections(ctx.file, false)?;
    compressed.arm64_entries = Some(arm64_entries);
    Some(Facts {
        compressed: Some(Box::new(compressed)),
        ..Facts::default()
    })
}

fn record_definition(definitions: &mut BTreeMap<u64, bool>, entry: u64) {
    definitions
        .entry(entry)
        .and_modify(|repeated| *repeated = true)
        .or_insert(false);
}

pub(super) fn decode_compressed(
    stored: &Deferred,
    entries: &TextSymbols,
    names: NameChars,
) -> Option<Facts> {
    let sections = stored.load()?;
    let dwarf =
        sections.borrow(|bytes| gimli::EndianSlice::new(bytes, gimli::RunTimeEndian::Little));
    let mut definitions = BTreeMap::new();
    let mut selected = Vec::new();
    let mut units = dwarf.units();
    while let Some(header) = units.next().ok()? {
        let unit = dwarf.unit(header).ok()?;
        if let Some((selection, observed)) = select_unit(&dwarf, &unit, entries) {
            for entry in observed {
                record_definition(&mut definitions, entry);
            }
            selected.push(selection);
        }
    }
    for unit in &mut selected {
        unit.definitions
            .retain(|def| definitions.get(&def.entry) == Some(&false));
    }
    decode_selected(&dwarf, &selected, entries, names)
}

fn select_unit(
    dwarf: &gimli::Dwarf<Reader<'_>>,
    unit: &gimli::Unit<Reader<'_>>,
    entries: &TextSymbols,
) -> Option<(SelectedUnit, Vec<u64>)> {
    let mut cursor = unit.entries();
    let root = cursor.next_dfs().ok()??;
    let mut extras = UnitExtras::metadata_only();
    extras.capture(dwarf, unit, &root, None);
    if !extras.go {
        return None;
    }
    let mut selected = SelectedUnit::new(extras.base?, extras.regabi);
    let mut observed = Vec::new();
    let mut active: Vec<OpenDefinition> = Vec::new();
    while let Some(die) = cursor.next_dfs().ok()? {
        while active
            .last()
            .is_some_and(|definition| definition.depth >= die.depth())
        {
            active.pop()?.finish(&mut selected);
        }
        if let Some(definition) = active
            .last_mut()
            .filter(|definition| definition.depth + 1 == die.depth())
        {
            definition.formals = definition.formals.take().and_then(|mut formals| {
                formals.observe(die.tag(), die.offset().0)?;
                Some(formals)
            });
        }
        if type_tag(die.tag()) {
            selected.types.push(die.offset().0);
            continue;
        }
        if die.tag() != gimli::DW_TAG_subprogram
            || matches!(
                die.attr_value(gimli::DW_AT_declaration),
                Some(gimli::AttributeValue::Flag(true))
            )
        {
            continue;
        }
        let Some(entry) = die
            .attr_value(gimli::DW_AT_low_pc)
            .and_then(|value| dwarf.attr_address(unit, value).ok().flatten())
        else {
            continue;
        };
        let Some(identity) = entries.get(&entry) else {
            continue;
        };
        observed.push(entry);
        if extras.regabi && identity.is_some() {
            active.push(OpenDefinition {
                depth: die.depth(),
                offset: die.offset().0,
                entry,
                formals: Some(Formals::default()),
            });
        }
    }
    for definition in active {
        definition.finish(&mut selected);
    }
    Some((selected, observed))
}

struct OpenDefinition {
    depth: isize,
    offset: usize,
    entry: u64,
    formals: Option<Formals>,
}

impl OpenDefinition {
    fn finish(self, selected: &mut SelectedUnit) {
        if let Some(formals) = self.formals.and_then(Formals::finish) {
            selected.definitions.push(Definition {
                offset: self.offset,
                entry: self.entry,
                formals,
            });
        }
    }
}

struct SelectedUnit {
    base: usize,
    regabi: bool,
    types: Vec<usize>,
    definitions: Vec<Definition>,
}

struct Definition {
    offset: usize,
    entry: u64,
    formals: [usize; 2],
}

fn decode_selected(
    dwarf: &gimli::Dwarf<Reader<'_>>,
    selected_units: &[SelectedUnit],
    entries: &TextSymbols,
    names: NameChars,
) -> Option<Facts> {
    let mut facts = Facts {
        names,
        ..Facts::default()
    };
    let mut units = Vec::with_capacity(selected_units.len());
    for selected in selected_units {
        let header = dwarf
            .unit_header(gimli::DebugInfoOffset(selected.base))
            .ok()?;
        let unit = dwarf.unit(header).ok()?;
        let mut extras = UnitExtras {
            base: Some(selected.base),
            regabi: selected.regabi,
            ..UnitExtras::metadata_only()
        };
        for &offset in &selected.types {
            let Some(key) = selected.base.checked_add(offset) else {
                continue;
            };
            let Ok(entry) = unit.entry(gimli::UnitOffset(offset)) else {
                continue;
            };
            let mut die = named_snapshot(dwarf, &unit, &entry);
            die.byte_size = entry
                .attr_value(gimli::DW_AT_byte_size)
                .and_then(|value| value.udata_value());
            if let Some(gimli::AttributeValue::Encoding(encoding)) =
                entry.attr_value(gimli::DW_AT_encoding)
            {
                die.encoding = Some(encoding);
            }
            let extra = extras.read_entry(dwarf, &unit, &entry, None);
            insert(&mut facts, key, die, extra);
        }
        if !selected.definitions.is_empty() {
            units.push((selected, unit, extras));
        }
    }
    for (selected, unit, mut extras) in units {
        for def in &selected.definitions {
            let Some(identity) = entries.get(&def.entry).and_then(Option::as_ref) else {
                continue;
            };
            selected.decode_definition(dwarf, &unit, &mut extras, def, identity, &mut facts);
        }
    }
    Some(facts)
}

impl SelectedUnit {
    fn new(base: usize, regabi: bool) -> Self {
        Self {
            base,
            regabi,
            types: Vec::new(),
            definitions: Vec::new(),
        }
    }

    fn decode_definition(
        &self,
        dwarf: &gimli::Dwarf<Reader<'_>>,
        unit: &gimli::Unit<Reader<'_>>,
        extras: &mut UnitExtras,
        definition: &Definition,
        identity: &(String, u64),
        facts: &mut Facts,
    ) -> Option<()> {
        let [input, output] = definition.formals;
        let input_key = self.base.checked_add(input)?;
        let output_key = self.base.checked_add(output)?;
        let key = self.base.checked_add(definition.offset)?;
        let input = unit.entry(gimli::UnitOffset(input)).ok()?;
        let output = unit.entry(gimli::UnitOffset(output)).ok()?;
        facts.arm64_type_shape(
            extras.reference(&input, gimli::DW_AT_type)?,
            extras.reference(&output, gimli::DW_AT_type)?,
        )?;
        let sub = unit.entry(gimli::UnitOffset(definition.offset)).ok()?;
        let mut die = named_snapshot(dwarf, unit, &sub);
        let (name, extent) = identity;
        if &die.name != name || name.is_empty() || hidden_context(name) {
            return None;
        }
        let extra = extras.read_entry(dwarf, unit, &sub, None);
        if extra.code_size != Some(*extent) {
            return None;
        }
        die.low_pc = Some(definition.entry);
        die.children = vec![input_key, output_key];
        insert(facts, key, die, extra);
        for (key, entry) in [(input_key, input), (output_key, output)] {
            let die = named_snapshot(dwarf, unit, &entry);
            let extra = extras.read_entry(dwarf, unit, &entry, Some(definition.entry));
            insert(facts, key, die, extra);
        }
        Some(())
    }
}

fn named_snapshot(
    dwarf: &gimli::Dwarf<Reader<'_>>,
    unit: &gimli::Unit<Reader<'_>>,
    entry: &gimli::DebuggingInformationEntry<Reader<'_>>,
) -> DieSnap {
    let mut die = DieSnap::new(entry.tag(), 0);
    if let Some(name) = entry
        .attr_value(gimli::DW_AT_name)
        .and_then(|value| dwarf.attr_string(unit, value).ok())
    {
        if let Ok(name) = std::str::from_utf8(name.slice()) {
            die.name = name.to_owned();
        }
    }
    die
}

fn insert(facts: &mut Facts, key: usize, mut die: DieSnap, extra: Extra) {
    die.type_ref = extra.type_ref;
    die.origin_ref = extra.origin_ref;
    facts.dies.insert(key, die);
    facts.extras.insert(key, extra);
}

pub(super) fn type_tag(tag: gimli::DwTag) -> bool {
    matches!(
        tag,
        gimli::DW_TAG_base_type
            | gimli::DW_TAG_pointer_type
            | gimli::DW_TAG_typedef
            | gimli::DW_TAG_const_type
            | gimli::DW_TAG_volatile_type
    )
}

#[derive(Default)]
struct Formals {
    offsets: [usize; 2],
    count: usize,
}

impl Formals {
    fn observe(&mut self, tag: gimli::DwTag, offset: usize) -> Option<()> {
        match tag {
            gimli::DW_TAG_formal_parameter => {
                *self.offsets.get_mut(self.count)? = offset;
                self.count += 1;
            }
            gimli::DW_TAG_unspecified_parameters => return None,
            _ => {}
        }
        Some(())
    }

    fn finish(self) -> Option<[usize; 2]> {
        (self.count == self.offsets.len()).then_some(self.offsets)
    }
}
