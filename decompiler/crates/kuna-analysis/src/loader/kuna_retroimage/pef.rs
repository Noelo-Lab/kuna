//! PEF (Preferred Executable Format, classic Mac OS on PowerPC): a container
//! header, section headers, and a loader section holding the main symbol, the
//! import tables, a relocation program per section and the hashed exports.
//!
//! The Code Fragment Manager instantiates each code and data section,
//! expanding pattern-initialized data, then runs the relocation programs,
//! which add section addresses and imported symbol addresses to words of the
//! data. The loader does the same over one contiguous module from [`BASE`]:
//! every instantiated section in header order, then one transition-vector
//! slot per imported symbol. A PowerPC main symbol is a transition vector
//! (code address, TOC); its second word fixes `r2` for the whole fragment.
//!
//! Names come from four places: the export table, the import table (given to
//! the import's slot and to the cross-fragment glue stub that calls through
//! it), and the traceback table that follows a function's code.

use std::collections::BTreeMap;

use kuna_base::error::KunaResult;
use object::SectionKind;

use super::{add_word, invalid, symbol_name, word, Bytes, RetroImage, RetroSection, RetroSymbol, MAX_MODULE};

const FORMAT: &str = "PEF";
pub const LANGUAGE: &str = "PowerPC:BE:32:default";
/// Address of the first instantiated section.
pub const BASE: u64 = 0x1000_0000;
const CONTAINER_HEADER: u64 = 40;
const SECTION_HEADER: u64 = 28;
const LOADER_HEADER: u64 = 56;
/// Bytes of one import's slot: a transition vector that points at itself.
const IMPORT_SLOT: u64 = 8;

const KIND_CODE: u8 = 0;
const KIND_DATA: u8 = 1;
const KIND_PATTERN_DATA: u8 = 2;
const KIND_CONSTANT: u8 = 3;
const KIND_LOADER: u8 = 4;
const KIND_EXECUTABLE_DATA: u8 = 6;

const CLASS_CODE: u8 = 0;
const CLASS_TVECTOR: u8 = 2;
const CLASS_GLUE: u8 = 4;

pub fn probe(prefix: &[u8]) -> bool {
    prefix.starts_with(b"Joy!peff")
}

/// Expand a pattern-initialized data section into `out`.
pub(super) fn unpack_pattern(packed: &[u8], out: &mut [u8]) -> Option<()> {
    let mut input = packed.iter().copied();
    let mut at = 0usize;
    let limit = out.len();
    let argument = |input: &mut dyn Iterator<Item = u8>| -> Option<usize> {
        let mut value = 0u32;
        for _ in 0..5 {
            let byte = input.next()?;
            value = (value << 7) | u32::from(byte & 0x7f);
            if byte & 0x80 == 0 {
                return Some(value as usize);
            }
        }
        None
    };
    while let Some(instruction) = input.next() {
        let count = match instruction & 0x1f {
            0 => argument(&mut input)?,
            count => usize::from(count),
        };
        let mut emit = |bytes: &mut dyn Iterator<Item = u8>, at: &mut usize, len: usize| -> Option<()> {
            let end = at.checked_add(len)?;
            for slot in out.get_mut(*at..end)? {
                *slot = bytes.next()?;
            }
            *at = end;
            Some(())
        };
        match instruction >> 5 {
            0 => emit(&mut std::iter::repeat(0), &mut at, count)?,
            1 => emit(&mut input, &mut at, count)?,
            2 => {
                let repeats = argument(&mut input)?.checked_add(1)?;
                let block: Vec<u8> = input.by_ref().take(count).collect();
                (block.len() == count && count != 0).then_some(())?;
                for _ in 0..repeats {
                    emit(&mut block.iter().copied(), &mut at, count)?;
                }
            }
            opcode @ (3 | 4) => {
                let custom = argument(&mut input)?;
                let repeats = argument(&mut input)?;
                let common: Vec<u8> = if opcode == 3 { input.by_ref().take(count).collect() } else { vec![0; count.min(limit)] };
                (common.len() == count && count + custom != 0).then_some(())?;
                for _ in 0..repeats {
                    emit(&mut common.iter().copied(), &mut at, count)?;
                    emit(&mut input, &mut at, custom)?;
                }
                emit(&mut common.iter().copied(), &mut at, count)?;
            }
            _ => return None,
        }
    }
    Some(())
}

#[derive(Default)]
pub(super) struct Section {
    name: Option<String>,
    default_address: u64,
    total: u64,
    unpacked: u64,
    packed: u64,
    container_offset: u64,
    kind: u8,
    /// Module offset, for an instantiated section.
    placed: Option<u64>,
}

impl Section {
    fn section_kind(&self) -> SectionKind {
        match self.kind {
            KIND_CODE => SectionKind::Text,
            KIND_CONSTANT => SectionKind::ReadOnlyData,
            _ => SectionKind::Data,
        }
    }

    /// An instantiated section of `total` bytes at module offset `placed`.
    #[cfg(test)]
    pub(super) fn placed_at(placed: u64, total: u64, default_address: u64) -> Self {
        Self { total, default_address, placed: Some(placed), ..Self::default() }
    }

    /// What a relocation adds for this section: where it is, less where the
    /// linker assumed it would be.
    fn delta(&self) -> Option<u32> {
        Some((BASE + self.placed?).wrapping_sub(self.default_address) as u32)
    }
}

/// The relocation program of one section, run over the module.
pub(super) struct Relocator<'a> {
    module: &'a mut [u8],
    sections: &'a [Section],
    imports: u64,
    import_count: u64,
    /// Module offsets of the section's start and end, and of the next word to patch.
    start: u64,
    end: u64,
    at: u64,
    import_index: u64,
    section_c: u32,
    section_d: u32,
}

impl<'a> Relocator<'a> {
    /// The relocator of section `index`, with imports placed at module offset
    /// `imports`; nothing when the section is not instantiated.
    pub(super) fn new(
        module: &'a mut [u8],
        sections: &'a [Section],
        index: usize,
        imports: u64,
        import_count: u64,
    ) -> Option<Self> {
        let section = sections.get(index)?;
        let start = section.placed?;
        let delta = |index: usize| sections.get(index).and_then(Section::delta).unwrap_or(0);
        Some(Self {
            module,
            sections,
            imports,
            import_count,
            start,
            end: start + section.total,
            at: start,
            import_index: 0,
            section_c: delta(0),
            section_d: delta(1),
        })
    }

    fn add(&mut self, delta: u32) -> Option<()> {
        (self.at + 4 <= self.end && add_word(self.module, self.at, delta, true)).then(|| self.at += 4)
    }

    fn skip(&mut self, bytes: u64) -> Option<()> {
        self.at = self.at.checked_add(bytes).filter(|&at| at <= self.end)?;
        Some(())
    }

    fn import(&mut self, index: u64) -> Option<()> {
        (index < self.import_count).then_some(())?;
        self.import_index = index + 1;
        self.add((BASE + self.imports + index * IMPORT_SLOT) as u32)
    }

    fn section(&self, index: u64) -> Option<u32> {
        self.sections.get(usize::try_from(index).ok()?)?.delta()
    }

    /// Run one instruction; `next` is the word after it, for the two-word
    /// forms. Returns how many words it took.
    fn step(&mut self, instruction: u16, next: Option<u16>) -> Option<usize> {
        let low = u64::from(instruction);
        let wide = |bits: u64| Some(((low & bits) << 16) | u64::from(next?));
        match instruction >> 9 {
            0x00..=0x1f => {
                self.skip(((low >> 6) & 0xff) * 4)?;
                for _ in 0..low & 0x3f {
                    self.add(self.section_d)?;
                }
            }
            opcode @ 0x20..=0x25 => {
                for _ in 0..(low & 0x1ff) + 1 {
                    match opcode {
                        0x20 => self.add(self.section_c)?,
                        0x21 => self.add(self.section_d)?,
                        0x22 | 0x23 => {
                            self.add(self.section_c)?;
                            self.add(self.section_d)?;
                            if opcode == 0x22 {
                                self.skip(4)?;
                            }
                        }
                        0x24 => {
                            self.add(self.section_d)?;
                            self.skip(4)?;
                        }
                        _ => self.import(self.import_index)?,
                    }
                }
            }
            0x30 => self.import(low & 0x1ff)?,
            0x31 => self.section_c = self.section(low & 0x1ff)?,
            0x32 => self.section_d = self.section(low & 0x1ff)?,
            0x33 => self.add(self.section(low & 0x1ff)?)?,
            0x40..=0x47 => self.skip((low & 0xfff) + 1)?,
            0x50 | 0x51 => {
                self.at = self.start.checked_add(wide(0x3ff)?).filter(|&at| at <= self.end)?;
                return Some(2);
            }
            0x52 | 0x53 => {
                self.import(wide(0x3ff)?)?;
                return Some(2);
            }
            0x5a | 0x5b => {
                let delta = self.section(wide(0x3f)?)?;
                match (low >> 6) & 0xf {
                    0 => self.add(delta)?,
                    1 => self.section_c = delta,
                    2 => self.section_d = delta,
                    _ => return None,
                }
                return Some(2);
            }
            _ => return None,
        }
        Some(1)
    }

    /// Run the whole program. A repeat instruction replays the blocks before
    /// it; the step budget bounds a program that repeats without patching.
    pub(super) fn run(&mut self, program: &[u16]) -> Option<()> {
        let mut budget = program.len() as u64 * 4 + (self.end - self.at) + 0x1_0000;
        let mut pc = 0usize;
        while let Some(&instruction) = program.get(pc) {
            let (blocks, repeats, width) = match instruction >> 9 {
                0x48..=0x4f => ((usize::from(instruction >> 8) & 0xf) + 1, u64::from(instruction & 0xff) + 1, 1),
                0x58 | 0x59 => {
                    let next = u64::from(*program.get(pc + 1)?);
                    ((usize::from(instruction >> 6) & 0xf) + 1, ((u64::from(instruction) & 0x3f) << 16) | next, 2)
                }
                _ => {
                    budget = budget.checked_sub(1)?;
                    pc += self.step(instruction, program.get(pc + 1).copied())?;
                    continue;
                }
            };
            let body = program.get(pc.checked_sub(blocks)?..pc)?;
            for _ in 0..repeats {
                let mut inner = 0;
                while let Some(&instruction) = body.get(inner) {
                    budget = budget.checked_sub(1)?;
                    inner += self.step(instruction, body.get(inner + 1).copied())?;
                }
            }
            pc += width;
        }
        Some(())
    }
}

/// The function names of the traceback tables in a code section: a zero word
/// after the function's last instruction, eight bytes of flags, the offset
/// back to the function's first instruction, and its name.
fn traceback_names(code: &[u8], offset: u64, symbols: &mut BTreeMap<u64, String>) {
    let file = Bytes { data: code, big: true };
    let mut at = 0u64;
    while let Some(marker) = file.u32(at) {
        at += 4;
        let (Some(version), Some(flags), Some(flags2), Some(parms)) =
            (file.u8(at), file.u8(at + 2), file.u8(at + 3), file.u16(at + 6))
        else {
            break;
        };
        const HAS_TBOFF: u8 = 0x20;
        const HAS_CTL: u8 = 0x08;
        const INT_HNDL: u8 = 0x80;
        const NAME_PRESENT: u8 = 0x40;
        if marker != 0 || version != 0 || flags & HAS_TBOFF == 0 || flags2 & NAME_PRESENT == 0 {
            continue;
        }
        let mut field = at + 8;
        if (parms >> 8) + ((parms & 0xff) >> 1) != 0 {
            field += 4;
        }
        let Some(length) = file.u32(field).map(u64::from) else { continue };
        field += 4;
        if flags2 & INT_HNDL != 0 {
            field += 4;
        }
        if flags & HAS_CTL != 0 {
            let Some(anchors) = file.u32(field) else { continue };
            field += 4 + u64::from(anchors) * 4;
        }
        let Some(start) = (at - 4).checked_sub(length).filter(|start| length != 0 && start % 4 == 0) else {
            continue;
        };
        let name = file.u16(field).and_then(|len| file.slice(field + 2, u64::from(len))).and_then(symbol_name);
        if let Some(name) = name {
            let name = name.strip_prefix('.').filter(|rest| !rest.is_empty()).map(str::to_owned).unwrap_or(name);
            symbols.entry(offset + start).or_insert(name);
        }
    }
}

/// The cross-fragment glue a linker emits for each imported routine:
/// load the transition vector's address from the TOC, save the caller's TOC,
/// load the code address and callee TOC, and jump.
const GLUE_TAIL: [u8; 20] = [
    0x90, 0x41, 0x00, 0x14, 0x80, 0x0c, 0x00, 0x00, 0x80, 0x4c, 0x00, 0x04, 0x7c, 0x09, 0x03, 0xa6, 0x4e, 0x80, 0x04,
    0x20,
];

fn glue_names(
    module: &[u8],
    code: (u64, u64),
    toc: u64,
    import_names: &BTreeMap<u64, String>,
    symbols: &mut BTreeMap<u64, String>,
) {
    let (start, size) = code;
    for at in (start..(start + size).saturating_sub(23)).step_by(4) {
        let stub = &module[at as usize..at as usize + 24];
        if stub[..2] != [0x81, 0x82] || stub[4..] != GLUE_TAIL {
            continue;
        }
        let displacement = i64::from(i16::from_be_bytes([stub[2], stub[3]]));
        let slot = (toc as i64 + displacement - BASE as i64) as u64;
        let Some(name) = word(module, slot, true).and_then(|vector| import_names.get(&u64::from(vector))) else {
            continue;
        };
        symbols.insert(at, name.clone());
    }
}

pub fn parse(filename: &str, bytes: &[u8]) -> KunaResult<RetroImage> {
    let bad = |detail: &str| invalid(filename, FORMAT, detail);
    let file = Bytes { data: bytes, big: true };
    match file.slice(8, 4) {
        Some(b"pwpc") => {}
        Some(b"m68k") => return Err(bad("CFM-68K fragments are not supported")),
        _ => return Err(bad("unknown architecture tag")),
    }
    let section_count = u64::from(file.u16(32).ok_or_else(|| bad("truncated container header"))?);
    let instantiated = u64::from(file.u16(34).ok_or_else(|| bad("truncated container header"))?);
    let names_at = CONTAINER_HEADER + section_count * SECTION_HEADER;

    let mut sections = Vec::new();
    let mut total = 0u64;
    for index in 0..section_count {
        let at = CONTAINER_HEADER + index * SECTION_HEADER;
        let field = |offset| file.u32(at + offset).map(u64::from);
        let (Some(name), Some(default_address), Some(size), Some(unpacked), Some(packed), Some(container_offset)) =
            (field(0), field(4), field(8), field(12), field(16), field(20))
        else {
            return Err(bad("truncated section headers"));
        };
        let kind = file.u8(at + 24).unwrap_or(0);
        let alignment = file.u8(at + 26).unwrap_or(0).min(12);
        let mapped = index < instantiated
            && size != 0
            && matches!(kind, KIND_CODE | KIND_DATA | KIND_PATTERN_DATA | KIND_CONSTANT | KIND_EXECUTABLE_DATA);
        let placed = mapped.then(|| total.next_multiple_of(1 << alignment.max(4)));
        if let Some(placed) = placed {
            total = placed + size;
            if total > MAX_MODULE {
                return Err(bad("sections exceed the image size limit"));
            }
        }
        sections.push(Section {
            name: (name != 0xffff_ffff).then(|| file.cstr(names_at + name)).flatten().and_then(symbol_name),
            default_address,
            total: size,
            unpacked: unpacked.min(size),
            packed,
            container_offset,
            kind,
            placed,
        });
    }
    if !sections.iter().any(|section| section.placed.is_some() && section.kind == KIND_CODE) {
        return Err(bad("no code section"));
    }

    let loader = sections.iter().find(|section| section.kind == KIND_LOADER);
    let loader = loader.and_then(|section| file.slice(section.container_offset, section.packed));
    let loader = Bytes { data: loader.ok_or_else(|| bad("missing loader section"))?, big: true };
    let head = |at| loader.u32(at).map(u64::from).ok_or_else(|| bad("truncated loader header"));
    let (main_section, main_offset) = (head(0)? as u32 as i32, head(4)?);
    let (library_count, import_count, reloc_sections) = (head(24)?, head(28)?, head(32)?);
    let (reloc_at, strings_at, hash_at, hash_power, export_count) = (head(36)?, head(40)?, head(44)?, head(48)?, head(52)?);
    let imports_at = LOADER_HEADER + library_count.checked_mul(24).ok_or_else(|| bad("bad library count"))?;
    let reloc_headers_at = import_count.checked_mul(4).and_then(|size| imports_at.checked_add(size));
    let reloc_headers_at = reloc_headers_at.filter(|&at| at <= loader.data.len() as u64);
    let reloc_headers_at = reloc_headers_at.ok_or_else(|| bad("import tables extend past the loader section"))?;

    let imports = total.next_multiple_of(16);
    total = imports + import_count * IMPORT_SLOT;
    if total > MAX_MODULE {
        return Err(bad("imports exceed the image size limit"));
    }
    let mut module = vec![0u8; total as usize];
    let mut notes = Vec::new();
    for (index, section) in sections.iter().enumerate() {
        let Some(placed) = section.placed else { continue };
        let out = &mut module[placed as usize..(placed + section.unpacked) as usize];
        let stored = file.slice(section.container_offset, section.packed);
        let stored = stored.ok_or_else(|| bad("section contents extend past end of file"))?;
        if section.kind == KIND_PATTERN_DATA {
            if unpack_pattern(stored, out).is_none() {
                notes.push(format!("the pattern-initialized data of section {index} is malformed and was cut short"));
            }
        } else {
            let kept = stored.len().min(out.len());
            out[..kept].copy_from_slice(&stored[..kept]);
        }
    }
    let mut import_names = BTreeMap::new();
    for index in 0..import_count {
        let slot = imports + index * IMPORT_SLOT;
        add_word(&mut module, slot, (BASE + slot) as u32, true);
        let entry = loader.u32(imports_at + index * 4).unwrap_or(0);
        let name = loader.cstr(strings_at + u64::from(entry & 0xff_ffff)).and_then(symbol_name);
        if let Some(name) = name {
            import_names.insert(BASE + slot, name);
        }
    }

    for header in (0..reloc_sections).map(|index| reloc_headers_at + index * 12) {
        let (Some(index), Some(count), Some(first)) = (loader.u16(header), loader.u32(header + 4), loader.u32(header + 8))
        else {
            notes.push("the relocation headers extend past the loader section".into());
            break;
        };
        let relocator = Relocator::new(&mut module, &sections, usize::from(index), imports, import_count);
        let program = loader.slice(reloc_at + u64::from(first), u64::from(count) * 2);
        let (Some(mut relocator), Some(program)) = (relocator, program) else {
            notes.push(format!("the relocations of section {index} are out of range and were not applied"));
            continue;
        };
        let program: Vec<u16> = program.chunks_exact(2).map(|pair| u16::from_be_bytes([pair[0], pair[1]])).collect();
        if relocator.run(&program).is_none() {
            notes.push(format!("the relocation program of section {index} is malformed and was cut short"));
        }
    }

    let place = |section: i64, offset: u64| -> Option<u64> {
        let section = sections.get(usize::try_from(section).ok()?)?;
        (offset < section.total).then_some(section.placed? + offset)
    };
    let is_code = |offset: u64| {
        sections.iter().any(|section| {
            section.kind == KIND_CODE && section.placed.is_some_and(|placed| offset >= placed && offset - placed < section.total)
        })
    };
    let vector_target = |module: &[u8], offset: u64| -> Option<u64> {
        u64::from(word(module, offset, true)?).checked_sub(BASE).filter(|&target| is_code(target))
    };

    let main = place(i64::from(main_section), main_offset).ok_or_else(|| bad("no main symbol"))?;
    let (entry, toc) = if is_code(main) {
        (main, None)
    } else {
        let entry = vector_target(&module, main).ok_or_else(|| bad("the main transition vector does not point at code"))?;
        (entry, word(&module, main + 4, true).map(u64::from))
    };

    let mut code_names = BTreeMap::new();
    let mut data_names = BTreeMap::new();
    for section in sections.iter().filter(|section| section.kind == KIND_CODE) {
        if let Some(placed) = section.placed {
            traceback_names(&module[placed as usize..(placed + section.unpacked) as usize], placed, &mut code_names);
        }
    }
    if let Some(toc) = toc {
        for section in sections.iter().filter(|section| section.kind == KIND_CODE) {
            if let Some(placed) = section.placed {
                glue_names(&module, (placed, section.unpacked), toc, &import_names, &mut code_names);
            }
        }
    }
    let keys_at = hash_at + (4u64 << hash_power.min(30));
    let exports_at = keys_at + export_count.saturating_mul(4);
    for index in 0..export_count {
        let record = exports_at + index * 10;
        let (Some(key), Some(class_and_name), Some(value), Some(section)) =
            (loader.u32(keys_at + index * 4), loader.u32(record), loader.u32(record + 4), loader.u16(record + 8))
        else {
            break;
        };
        let name = loader.slice(strings_at + u64::from(class_and_name & 0xff_ffff), u64::from(key >> 16));
        let (Some(name), Some(offset)) = (name.and_then(symbol_name), place(i64::from(section as i16), u64::from(value)))
        else {
            continue;
        };
        match (class_and_name >> 24) as u8 & 0xf {
            CLASS_CODE | CLASS_GLUE if is_code(offset) => {
                code_names.insert(offset, name);
            }
            CLASS_TVECTOR => {
                if let Some(target) = vector_target(&module, offset) {
                    code_names.insert(target, name);
                }
            }
            _ => {
                data_names.insert(offset, name);
            }
        }
    }
    for (address, name) in import_names {
        data_names.insert(address - BASE, name);
    }
    let symbols = code_names
        .into_iter()
        .map(|(offset, name)| RetroSymbol { offset, name, code: true })
        .chain(data_names.into_iter().map(|(offset, name)| RetroSymbol { offset, name, code: false }))
        .collect();

    let mut mapped: Vec<RetroSection> = sections
        .iter()
        .enumerate()
        .filter_map(|(index, section)| {
            Some(RetroSection {
                name: section.name.clone().unwrap_or_else(|| format!("section{index}")),
                offset: section.placed?,
                size: section.total,
                kind: section.section_kind(),
                file_range: (section.kind != KIND_PATTERN_DATA)
                    .then(|| (section.container_offset, section.packed.min(section.unpacked))),
            })
        })
        .collect();
    if import_count != 0 {
        mapped.push(RetroSection {
            name: "imports".into(),
            offset: imports,
            size: import_count * IMPORT_SLOT,
            kind: SectionKind::Data,
            file_range: None,
        });
    }
    Ok(RetroImage {
        format: FORMAT,
        language: LANGUAGE,
        base: BASE,
        module,
        entry: BASE + entry,
        sections: mapped,
        symbols,
        tracked: toc.map(|toc| ("r2", toc)).into_iter().collect(),
        notes,
    })
}
