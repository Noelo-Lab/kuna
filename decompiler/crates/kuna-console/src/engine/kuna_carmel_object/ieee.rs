//! Indexed TASKING Carmel MUFOM streams, as emitted by ascarm/lkcarm/lccarm.
//!
//! The assembler manual's IEEE-695 appendix describes ST/AS/LD/LR and the RPN
//! encoding. The Carmel variants of those records (MAU widths, wide issue
//! ordering, and the user functions in LR) are specific to this producer.

use std::collections::BTreeMap;

use kuna_analysis::loader::ieee695::{self as wire, Cursor, Id, err};
use kuna_base::error::KunaResult;

use super::{Image, Region, Space, Symbol};

mod expr;
use expr::{Expression, Variable};

const MAGIC: &[u8] = b"\xe0\x06carmel";
const CODE_LIMIT: u64 = 1 << 24;
const SHORT_LIMIT: u64 = 1 << 16;
const CLIW_BANK: u64 = 4 * 1024;
const CLIW_ROM_BASE: u64 = 0xe80000;

pub(super) fn recognizes(bytes: &[u8]) -> bool {
    bytes.starts_with(MAGIC)
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Kind { Relocatable, TiofAbsolute, IeeeAbsolute }

fn kind(mut part: Cursor<'_>) -> KunaResult<Kind> {
    let at = part.offset;
    if part.peek() == Some(0xea) {
        part.expect(&[0xea, 0x25])?;
        if part.name()? != b"Object version: 1.1" {
            return Err(err(at, "unsupported TIOF revision"));
        }
        part.expect(&[0xea, 0x26])?;
        return match part.name()? {
            b"Object format: Relocatable" => Ok(Kind::Relocatable),
            b"Object format: Absolute" => Ok(Kind::TiofAbsolute),
            _ => Err(err(at, "unsupported TIOF object format")),
        };
    }
    // The debugger layout encodes version and format as attributes of N.
    part.expect(&[0xf0])?;
    let id = part.id()?;
    part.name()?;
    for (attribute, values) in [(0x25, &[1, 1][..]), (0x26, &[1][..])] {
        part.expect(&[0xf1, 0xce])?;
        if part.id()? != id { return Err(err(at, "invalid IEEE module attribute index")); }
        part.expect(&[0, attribute])?;
        part.expect(values)?;
    }
    Ok(Kind::IeeeAbsolute)
}

struct Section<'a> {
    name: &'a [u8],
    space: Space,
    executable: bool,
    readonly: bool,
    align: u64,
    size: Option<u64>,
    fixed: Option<u64>,
    start: u64,
    at: usize,
}

type Sections<'a> = BTreeMap<Id, Section<'a>>;
type Values = BTreeMap<Id, i64>;

fn width(space: Space) -> usize {
    match space { Space::Code | Space::Cliw => 3, _ => 2 }
}

fn limit(space: Space) -> u64 {
    match space { Space::Code | Space::Cliw => CODE_LIMIT, _ => SHORT_LIMIT }
}

fn cluster(name: &[u8], mau: u64, at: usize) -> KunaResult<Space> {
    match (mau, name) {
        (3, b"CODE_cluster") => Ok(Space::Code),
        (3, b"CLIW_cluster") => Ok(Space::Cliw),
        (2, b"IO_cluster" | b"IO_rcluster") => Ok(Space::Io),
        (2, b"EXT_cluster" | b"EXT_rcluster") => Ok(Space::Ext),
        (2, b"DATA_cluster" | b"DATA_rcluster" | b"ADP_cluster" | b"ADP_rcluster"
            | b"ASP_cluster" | b"ASP_rcluster" | b"BDP_cluster" | b"BDP_rcluster"
            | b"BSP_cluster" | b"BSP_rcluster") => Ok(Space::Data),
        _ => Err(err(at, format_args!("unsupported absolute {mau}-byte cluster {:?}", String::from_utf8_lossy(name)))),
    }
}

fn addressing_mode(mode: u64) -> Option<Space> {
    match mode {
        1 | 0x81 => Some(Space::Code),
        2 | 4..=7 => Some(Space::Data),
        3 | 0x83 => Some(Space::Cliw),
        8 => Some(Space::Ext),
        9 => Some(Space::Io),
        _ => None,
    }
}

fn sections(mut part: Cursor<'_>, kind: Kind) -> KunaResult<Sections<'_>> {
    let mut result: Sections<'_> = BTreeMap::new();
    while let Some(tag) = part.peek() {
        let at = part.offset;
        part.byte()?;
        let id = if tag == 0xe2 { 0 } else { part.id()? };
        match tag {
            0xe6 => {
                let mut attrs = 0_u32;
                let mut mode = None;
                while let Some(letter @ 0xc1..=0xda) = part.peek() {
                    part.byte()?;
                    attrs |= 1 << (letter - 0xc1);
                    if letter == 0xd9 { mode = Some(part.num()?); } // Y: addressing mode
                }
                let has = |letter: u8| attrs & (1 << (letter - b'A')) != 0;
                let name = part.name()?;
                let space = match kind {
                    Kind::IeeeAbsolute => cluster(name, part.num()?, at)?,
                    Kind::TiofAbsolute => cluster(name, mode.ok_or_else(|| err(at, "missing absolute MAU width"))?, at)?,
                    Kind::Relocatable => match mode {
                        Some(9) => return Err(err(at, "24-bit TASKING IO MAUs cannot be mapped to the 16-bit Kuna IO space")),
                        Some(mode) => addressing_mode(mode).ok_or_else(|| err(at, format_args!("unsupported addressing mode {mode:#x}")))?,
                        None if has(b'X') => Space::Code,
                        None => return Err(err(at, "relocatable section has no recognized address space")),
                    },
                };
                let executable = space == Space::Code && (kind != Kind::Relocatable || has(b'X'));
                let readonly = match kind {
                    Kind::Relocatable => has(b'R') && !has(b'W'),
                    _ => space == Space::Code || space == Space::Cliw || name.ends_with(b"_rcluster"),
                };
                let align = match space { Space::Code => 2, Space::Cliw => 4, _ => 1 };
                if result.insert(id, Section { name, space, executable, readonly, align, size: None, fixed: None, start: 0, at }).is_some() {
                    return Err(err(at, "duplicate section index"));
                }
            }
            0xe7 => {
                let align = part.num()?;
                if align == 0 { return Err(err(at, "zero section alignment")); }
                let sec = section_mut(&mut result, id, at)?;
                let mut divisor = align;
                let mut other = sec.align;
                while other != 0 { (divisor, other) = (other, divisor % other); }
                sec.align = (sec.align / divisor).checked_mul(align)
                    .ok_or_else(|| err(at, "section alignment overflow"))?;
                if matches!(part.peek(), Some(0x00..=0x88)) {
                    return Err(err(part.offset, "unsupported section page alignment"));
                }
            }
            0xe2 => {
                let variable = part.byte()?;
                let section_id = part.id()?;
                let sec = section_mut(&mut result, section_id, at)?;
                let dest = match variable {
                    0xcc => &mut sec.fixed,
                    0xd3 => &mut sec.size,
                    _ => return Err(err(at, format_args!("unsupported W2 assignment {variable:#04x}"))),
                };
                if dest.replace(part.num()?).is_some() { return Err(err(at, "duplicate section geometry assignment")); }
            }
            0xfb => {
                // Carmel's extension specifies bits per addressable word.
                let sec = result.get(&id).ok_or_else(|| err(at, "unknown MAU section index"))?;
                if part.num()? != (width(sec.space) * 8) as u64 {
                    return Err(err(at, "section MAU conflicts with its address space"));
                }
            }
            0xf0 => {
                // Relocatable overlay-call graph names do not place sections.
                part.name()?;
                while part.rem.starts_with(&[0xe2, 0xcf]) {
                    part.take(2)?;
                    part.id()?;
                    while part.peek() == Some(0xce) { part.byte()?; part.id()?; }
                    part.expect(&[0x9f])?;
                }
            }
            _ => return Err(err(at, format_args!("unsupported W2 record {tag:#04x}"))),
        }
    }
    Ok(result)
}

fn section_mut<'s, 'a>(sections: &'s mut Sections<'a>, id: Id, at: usize) -> KunaResult<&'s mut Section<'a>> {
    sections.get_mut(&id).ok_or_else(|| err(at, format_args!("unknown section index {id:#x}")))
}

// Placement is synthetic for sections without L. Respect explicit positions,
// declaration IDs and SA, but do not try to reproduce a missing locator script.
fn place(sections: &mut Sections<'_>, kind: Kind) -> KunaResult<()> {
    let mut occupied: BTreeMap<(Space, u64), u64> = BTreeMap::new();
    for sec in sections.values_mut().filter(|s| s.fixed.is_some()) {
        sec.start = sec.fixed.unwrap_or_default();
        if kind == Kind::Relocatable && sec.start % sec.align != 0 {
            return Err(err(sec.at, "misaligned fixed section"));
        }
        reserve(&mut occupied, sec)?;
    }
    let mut next = BTreeMap::new();
    for sec in sections.values_mut().filter(|s| s.fixed.is_none()) {
        if kind != Kind::Relocatable { return Err(err(sec.at, "absolute section missing its base")); }
        let mut start = *next.get(&sec.space).unwrap_or(&0_u64);
        let size = sec.size.ok_or_else(|| err(sec.at, "missing section size"))?;
        loop {
            start = start.div_ceil(sec.align).checked_mul(sec.align)
                .ok_or_else(|| err(sec.at, "section placement overflow"))?;
            let end = start.checked_add(size).ok_or_else(|| err(sec.at, "section extent overflow"))?;
            if let Some(&stop) = occupied.range((sec.space, 0)..(sec.space, end)).next_back()
                .map(|(_, stop)| stop).filter(|&&stop| stop > start) {
                start = stop;
            } else {
                next.insert(sec.space, end);
                break;
            }
        }
        sec.start = start;
        reserve(&mut occupied, sec)?;
    }
    Ok(())
}

fn reserve(occupied: &mut BTreeMap<(Space, u64), u64>, sec: &Section<'_>) -> KunaResult<()> {
    let size = sec.size.ok_or_else(|| err(sec.at, "missing section size"))?;
    let end = sec.start.checked_add(size).filter(|&end| end <= limit(sec.space))
        .ok_or_else(|| err(sec.at, "section extent outside its address space"))?;
    if size != 0 {
        if occupied.range((sec.space, 0)..(sec.space, end)).next_back().is_some_and(|(_, &stop)| stop > sec.start) {
            return Err(err(sec.at, "overlapping section extents"));
        }
        occupied.insert((sec.space, sec.start), end);
    }
    Ok(())
}

#[derive(Default)]
struct Names<'a> {
    public: BTreeMap<Id, Name<'a>>,
    external: BTreeMap<Id, Name<'a>>,
}

struct Name<'a> {
    name: &'a [u8],
    space: Option<Space>,
    value: Option<Expression>,
    at: usize,
}

fn names(mut part: Cursor<'_>) -> KunaResult<Names<'_>> {
    let mut names = Names::default();
    while let Some(tag) = part.peek() {
        let at = part.offset;
        part.byte()?;
        match tag {
            0xe8 | 0xe9 => {
                let id = part.id()?;
                let name = part.name()?;
                let table = if tag == 0xe8 { &mut names.public } else { &mut names.external };
                if name.is_empty() || table.insert(id, Name { name, space: None, value: None, at }).is_some() {
                    return Err(err(at, "empty or duplicate symbol declaration"));
                }
            }
            0xf1 => {
                let variable = part.byte()?;
                let id = part.id()?;
                let table = match variable {
                    0xc9 => &mut names.public, 0xd8 => &mut names.external,
                    _ => return Err(err(at, "unsupported W3 symbol kind")),
                };
                let name = table.get_mut(&id).ok_or_else(|| err(at, "attribute for unknown symbol"))?;
                let mut tail = [0; 2];
                let mut count = 0;
                while matches!(part.peek(), Some(0x00..=0x88)) {
                    tail = [tail[1], part.num()?];
                    count += 1;
                }
                // TASKING's context attribute is encoded as a length (one)
                // followed by the addressing mode; bare type IDs aren't addresses.
                if count >= 3 && tail[0] == 1 { name.space = addressing_mode(tail[1]); }
            }
            0xe2 => {
                part.expect(&[0xc9])?;
                let id = part.id()?;
                let name = names.public.get_mut(&id).ok_or_else(|| err(at, "assignment to unknown public symbol"))?;
                if name.value.replace(Expression::read(&mut part, false)?).is_some() {
                    return Err(err(at, "duplicate public symbol assignment"));
                }
            }
            _ => return Err(err(at, format_args!("unsupported W3 record {tag:#04x}"))),
        }
    }
    Ok(names)
}

fn section_value(sections: &Sections<'_>, id: Id, at: usize) -> KunaResult<i64> {
    sections.get(&id).map(|sec| sec.start as i64)
        .ok_or_else(|| err(at, format_args!("unknown section reference {id:#x}")))
}

fn public_symbols(sections: &Sections<'_>, names: &Names<'_>, image: &mut Image) -> KunaResult<Values> {
    let mut values = Values::new();
    for (&id, name) in &names.public {
        let Some(expr) = &name.value else { continue; };
        let value = expr.eval(|var| match var {
            Variable::Base(i) | Variable::Relative(i) => section_value(sections, i, expr.at),
            Variable::Public(i) => known(&values, i, expr.at, "public symbol"),
            Variable::External(_) => Err(err(expr.at, "unsupported external in public symbol assignment")),
        })?;
        values.insert(id, value);
        let from_section = expr.section().and_then(|section| sections.get(&section));
        let Some(space) = name.space.or(from_section.map(|sec| sec.space)) else { continue; };
        let offset = address(value, space, name.at)?;
        if space != Space::Cliw {
            image.symbols.push(Symbol { space, offset: offset * width(space) as u64, name: name.name.to_vec() });
        }
    }
    Ok(values)
}

fn known(values: &Values, id: Id, at: usize, label: &str) -> KunaResult<i64> {
    values.get(&id).copied().ok_or_else(|| err(at, format_args!("absent {label} mapping for index {id:#x}")))
}

fn address(value: i64, space: Space, at: usize) -> KunaResult<u64> {
    u64::try_from(value).ok().filter(|&word| word < limit(space))
        .ok_or_else(|| err(at, format_args!("{} address outside its range: {value:#x}", space.name())))
}

struct Run {
    section: Id,
    start: u64,
    bytes: Vec<u8>,
    copy: bool,
}

struct Fixup {
    run: usize,
    offset: usize,
    bytes: usize,
    swap: bool,
    displacement: Option<(Id, i64, i64)>,
    expr: Expression,
}

struct Loader<'a, 'b> {
    kind: Kind,
    sections: &'b Sections<'a>,
    names: &'b Names<'a>,
    public: &'b Values,
    copy: Option<(u64, u64)>,
    relative: Values,
    position: Values,
    selected: Option<Id>,
    runs: Vec<Run>,
    fixups: Vec<Fixup>,
}

impl Loader<'_, '_> {
    fn resolve(&self, var: Variable, at: usize) -> KunaResult<Option<i64>> {
        Ok(Some(match var {
            Variable::Base(id) => section_value(self.sections, id, at)?,
            Variable::Relative(id) => self.relative.get(&id).copied()
                .map_or_else(|| section_value(self.sections, id, at), Ok)?,
            Variable::Public(id) => known(self.public, id, at, "public symbol")?,
            Variable::External(id) => {
                let ext = self.names.external.get(&id).ok_or_else(|| err(at, "reference to undeclared external"))?;
                if ext.name == b"__lc_b___cliw" {
                    self.sections.values().filter(|s| s.space == Space::Cliw).map(|sec| sec.start).min()
                        .ok_or_else(|| err(at, "absent CLIW section for locator base"))? as i64
                } else {
                    return Ok(None);
                }
            }
        }))
    }

    fn load(&mut self, count: usize, at: usize, copy: bool) -> KunaResult<(usize, usize, bool)> {
        let id = self.selected.ok_or_else(|| err(at, "load without a selected section"))?;
        let sec = &self.sections[&id];
        let unit = width(sec.space);
        // IEEE may end an LD with a partial CODE word in an initialized
        // RAM copy. finish() rejects it unless the section metadata
        // proves that the partial word belongs to a nonexecutable extent.
        let partial_code = self.kind == Kind::IeeeAbsolute && sec.space == Space::Code;
        if count == 0 || (!copy && !partial_code && count % unit != 0) {
            return Err(err(at, "load splits an addressable word"));
        }
        let start = address(known(&self.position, id, at, "section load position")?, sec.space, at)?;
        let end = start.checked_add(count.div_ceil(unit) as u64)
            .filter(|&end| start >= sec.start && end <= sec.start + sec.size.unwrap_or_default())
            .ok_or_else(|| err(at, "load outside its declared section"))?;
        self.position.insert(id, end as i64);
        if !self.runs.last().is_some_and(|run| run.section == id && run.copy == copy
            && run.start * unit as u64 + run.bytes.len() as u64 == start * unit as u64) {
            self.runs.push(Run { section: id, start, bytes: Vec::new(), copy });
        }
        let index = self.runs.len() - 1;
        let offset = self.runs[index].bytes.len();
        let copy = copy || self.copy.is_some_and(|(first, last)| first <= start && start < last);
        let swap = self.kind != Kind::IeeeAbsolute && sec.space == Space::Code
            && sec.executable && !copy && count == 6;
        Ok((index, offset, swap))
    }

    fn literal(&mut self, literal: &[u8], at: usize, copy: bool) -> KunaResult<()> {
        let (run, _, swap) = self.load(literal.len(), at, copy)?;
        let dest = &mut self.runs[run].bytes;
        if swap {
            dest.extend_from_slice(&literal[3..]);
            dest.extend_from_slice(&literal[..3]);
        } else {
            dest.extend_from_slice(literal);
        }
        Ok(())
    }

    // TIOF writes each 16-bit DATA source word in a separate LD, with
    // a pseudo-position increment of one. The debugger absolute variant
    // writes those same bytes packed at the first CODE source position.
    fn tiof_copy(&mut self, part: &mut Cursor<'_>, first: &[u8], at: usize) -> KunaResult<()> {
        let id = self.selected.ok_or_else(|| err(at, "copy without a selected section"))?;
        let mut pos = known(&self.position, id, at, "copy source position")?;
        let mut bytes = first.to_vec();
        while part.rem.starts_with(&[0xe2, 0xd0]) {
            let mut next = *part;
            next.take(2)?;
            if next.id()? != id || next.num()? != (pos + 1) as u64
                || !next.rem.starts_with(&[0xed, 2]) {
                break;
            }
            next.take(2)?;
            bytes.extend_from_slice(next.take(2)?);
            pos += 1;
            *part = next;
        }
        let (run, _, _) = self.load(bytes.len(), at, true)?;
        let dest = &mut self.runs[run].bytes;
        if dest.is_empty() {
            *dest = bytes;
        } else {
            dest.extend(bytes);
        }
        Ok(())
    }

    fn data(&mut self, mut part: Cursor<'_>) -> KunaResult<()> {
        while let Some(tag) = part.peek() {
            let at = part.offset;
            part.byte()?;
            match tag {
                0xe5 => {
                    let id = part.id()?;
                    if !self.sections.contains_key(&id) { return Err(err(at, "load selects unknown section")); }
                    self.selected = Some(id);
                }
                0xe2 => {
                    let variable = part.byte()?;
                    let id = part.id()?;
                    if !self.sections.contains_key(&id) { return Err(err(at, "assignment to unknown load section")); }
                    let expr = Expression::read(&mut part, false)?;
                    let value = expr.eval(|var| self.resolve(var, at)?.ok_or_else(|| err(at, "external in section position")))?;
                    let sec = &self.sections[&id];
                    if value < sec.start as i64 || value > (sec.start + sec.size.unwrap_or_default()) as i64 {
                        return Err(err(at, "section pointer outside its declared extent"));
                    }
                    match variable {
                        0xd0 => { self.position.insert(id, value); }
                        0xd2 => { self.relative.insert(id, value); }
                        _ => return Err(err(at, format_args!("unsupported W5 assignment {variable:#04x}"))),
                    }
                }
                0xed => {
                    let literal = part.name()?;
                    let code = self.selected.is_some_and(|id| self.sections[&id].space == Space::Code);
                    if self.kind == Kind::TiofAbsolute && code && literal.len() == 2
                        && self.sections.values().any(|sec| sec.space == Space::Data
                            && !sec.readonly && sec.size != Some(0)) {
                        self.tiof_copy(&mut part, literal, at)?;
                    } else {
                        self.literal(literal, at, false)?;
                    }
                }
                0xe4 => {
                    if self.kind == Kind::IeeeAbsolute { return Err(err(at, "unexpected LR in debugger absolute")); }
                    let mut empty = true;
                    while let Some(item @ (0x00..=0x88 | 0xbe)) = part.peek() {
                        empty = false;
                        if item == 0xbe {
                            part.byte()?;
                            let mut expr = Expression::read(&mut part, true)?;
                            part.expect(&[0xbf])?;
                            let id = self.selected.ok_or_else(|| err(at, "relocation without a selected section"))?;
                            let unit = width(self.sections[&id].space);
                            let n = expr.take_word_count()?.checked_mul(unit)
                                .ok_or_else(|| err(expr.at, "relocation size overflow"))?;
                            if n > 8 { return Err(err(expr.at, "unsupported relocation field width")); }
                            let at = expr.at;
                            expr.bind(|var| self.resolve(var, at))?;
                            let (run, offset, swap) = self.load(n, expr.at, false)?;
                            if expr.has_external() {
                                let displacement = if unit == 3 && n == 6 && self.sections[&id].space == Space::Code {
                                    expr.displacement()?
                                } else {
                                    None
                                };
                                for ext in expr.externals() {
                                    let name = &self.names.external[&ext];
                                    let is_code = name.space == Some(Space::Code)
                                        || name.space.is_none() && displacement.is_some_and(|(id, _, _)| id == ext);
                                    if !is_code {
                                        return Err(err(expr.at, format_args!(
                                            "absent mapping for external {:?}", String::from_utf8_lossy(name.name),
                                        )));
                                    }
                                }
                                self.runs[run].bytes.resize(offset + n, 0);
                                self.fixups.push(Fixup { run, offset, bytes: n, swap, displacement, expr });
                            } else {
                                let encoded = expr.encoded(n, swap, |_, at| Err(err(at, "unresolved relocation")))?;
                                self.runs[run].bytes.extend_from_slice(&encoded[..n]);
                            }
                        } else {
                            self.literal(part.name()?, at, false)?;
                        }
                    }
                    if empty { return Err(err(at, "unsupported or empty LR operands")); }
                }
                0xf8 if self.kind == Kind::TiofAbsolute => {
                    // The locator may emit a bounded debug scope between
                    // CODE and RAM initializers instead of indexing it in W4.
                    let (tag, body) = scope_body(&mut part, at)?;
                    if !matches!(tag, 0 | 1 | 3 | 5 | 0x0a) || body.rem.last() != Some(&0xf9) {
                        return Err(err(at, "unsupported embedded TIOF scope"));
                    }
                }
                _ => return Err(err(at, format_args!("unsupported W5 record {tag:#04x}"))),
            }
        }
        Ok(())
    }
}

#[derive(Clone, Copy)]
struct ImportRange {
    low: i64,
    high: i64,
    at: usize,
}

fn allocate_imports(
    sections: &Sections<'_>, names: &Names<'_>, fixups: &[Fixup], image: &mut Image,
) -> KunaResult<Values> {
    if fixups.is_empty() { return Ok(Values::new()); }
    let mut ranges = BTreeMap::new();
    for fixup in fixups {
        let expr = &fixup.expr;
        for id in expr.externals() {
            ranges.entry(id).or_insert(ImportRange { low: 0, high: CODE_LIMIT as i64 - 1, at: expr.at });
        }
        if let Some((id, low, high)) = fixup.displacement {
            let range = ranges.get_mut(&id).ok_or_else(|| err(expr.at, "missing import displacement range"))?;
            range.low = range.low.max(low);
            range.high = range.high.min(high);
            range.at = expr.at;
        }
    }

    // Reserve declared CODE, not just loaded bytes: a hole in a section
    // cannot stand in for an external. Keep imports unbacked and even-placed.
    let mut occupied: BTreeMap<u64, u64> = sections.values()
        .filter(|sec| sec.space == Space::Code && sec.size != Some(0))
        .map(|sec| (sec.start, sec.start + sec.size.unwrap_or_default()))
        .collect();
    let mut pending: Vec<_> = ranges.into_iter().collect();
    pending.sort_unstable_by_key(|&(id, range)| (range.high, id));
    let mut allocated = BTreeMap::new();
    let mut names_at = BTreeMap::new();
    for (id, range) in pending {
        let name = names.external[&id].name;
        if let Some(&word) = names_at.get(name) {
            if word < range.low || word > range.high {
                return Err(err(range.at, "relocations for one external have incompatible displacement ranges"));
            }
            allocated.insert(id, word);
            continue;
        }
        let mut candidate = (range.low as u64).next_multiple_of(2);
        while let Some((_, &end)) = occupied.range(..=candidate).next_back() {
            if candidate >= end { break; }
            candidate = end.next_multiple_of(2);
        }
        if candidate as i64 > range.high {
            return Err(err(range.at, format_args!("no unmapped CODE address in the displacement range for {:?}", String::from_utf8_lossy(name))));
        }
        occupied.insert(candidate, candidate + 1);
        allocated.insert(id, candidate as i64);
        names_at.insert(name, candidate as i64);
        image.imports.push(Symbol { space: Space::Code, offset: candidate * 3, name: name.to_vec() });
    }
    Ok(allocated)
}

fn default_copy_bounds(sections: &Sections<'_>, image: &Image) -> Option<(u64, u64)> {
    // The TASKING startup object puts four NOPs at __DUMMYNOPS.
    // The default locator script places CODE ROM copy tables/data after
    // those and before __DUMMYNOPS2, in the same physical CODE cluster.
    let label = |name: &[u8]| image.symbols.iter()
        .find(|sym| sym.space == Space::Code && sym.name == name)
        .map(|sym| sym.offset / 3);
    let marker = label(b"__DUMMYNOPS")?;
    let first = marker + 4;
    let last = label(b"__DUMMYNOPS2")?;
    (first < last && sections.values().any(|sec| sec.space == Space::Code
        && sec.name == b"CODE_cluster" && sec.start <= marker
        && last < sec.start + sec.size.unwrap_or_default())).then_some((first, last))
}

fn scope_body<'a>(part: &mut Cursor<'a>, at: usize) -> KunaResult<(u8, Cursor<'a>)> {
    let tag = part.byte()?;
    let length = part.read_length()?;
    let length = length.checked_sub(part.offset - at).ok_or_else(|| err(at, "invalid scope length"))?;
    let offset = part.offset;
    Ok((tag, Cursor { rem: part.take(length)?, offset }))
}

// The debugger's simple module scopes describe the actual executable
// subsections, excluding synthetic ROM copies added by the locator.
// Do not scan unknown debug subrecords for byte sequences resembling scopes.
fn ieee_copy_tail(mut debug: Cursor<'_>, sections: &Sections<'_>, runs: &[Run]) -> KunaResult<Option<(u64, u64)>> {
    let Some((&code_id, code)) = sections.iter().find(|(_, sec)| sec.space == Space::Code) else { return Ok(None); };
    let mut ram = sections.iter().filter(|(id, sec)| sec.space == Space::Data && !sec.readonly
        && sec.size.is_some_and(|size| size != 0) && runs.iter().all(|run| run.section != **id));
    let Some((_, ram)) = ram.next().filter(|_| ram.next().is_none()) else { return Ok(None); };
    let ram_words = ram.size.unwrap_or_default();
    let mut last_code = 0;
    while let Some(0xf8) = debug.peek() {
        let at = debug.offset;
        debug.byte()?;
        let (tag, mut module) = scope_body(&mut debug, at)?;
        if tag != 0x0a { continue; }
        module.name()?;
        if !module.rem.starts_with(&[0, 0]) { continue; }
        module.take(2)?;
        while let Some(0xf8) = module.peek() {
            let at = module.offset;
            module.byte()?;
            let (tag, mut section) = scope_body(&mut module, at)?;
            if tag != 0x0b { continue; }
            section.name()?;
            let context = section.num()?;
            section.num()?;
            let start = section.num()?;
            section.expect(&[0xf9])?;
            let last = start.checked_add(section.num()?).ok_or_else(|| err(at, "debug section extent overflow"))?;
            section.end()?;
            if context == 1 && start >= code.start && last <= code.start + code.size.unwrap_or_default() {
                last_code = last_code.max(last);
            }
        }
    }
    let words = (ram_words * 2).div_ceil(3);
    let end = code.start + code.size.unwrap_or_default();
    let witness = runs.iter().any(|run| run.section == code_id && run.start < last_code
        && run.start * 3 + run.bytes.len() as u64 == last_code * 3 + ram_words * 2);
    Ok((witness && last_code > code.start && last_code + words <= end
        && end <= last_code + ram_words).then_some((last_code, last_code + words)))
}

fn cliw_origin(sections: &Sections<'_>, kind: Kind) -> Option<u64> {
    let mut templates = sections.values().filter(|sec| sec.space == Space::Cliw);
    let first = templates.next()?;
    if kind != Kind::Relocatable {
        return (first.name == b"CLIW_cluster" && first.start == CLIW_ROM_BASE
            && first.size.is_some_and(|size| size <= CLIW_BANK && size % 4 == 0)
            && templates.next().is_none()).then_some(CLIW_ROM_BASE);
    }
    std::iter::once(first).chain(templates).all(|sec| {
        (sec.name == b"__cliw" || sec.name.starts_with(b"__cliw@")) && sec.fixed.is_none()
            && sec.start % 4 == 0
            && sec.size.is_some_and(|size| size % 4 == 0 && sec.start + size <= CLIW_BANK)
    }).then_some(0)
}

fn push_region(regions: &mut Vec<Region>, region: Region, at: usize) -> KunaResult<()> {
    if let Some(last) = regions.last_mut().filter(|last| last.space == region.space) {
        let end = last.offset + last.bytes.len() as u64;
        if region.offset < end { return Err(err(at, "overlapping load records")); }
        if region.offset == end && last.readonly == region.readonly && last.executable == region.executable {
            last.bytes.extend(region.bytes);
            return Ok(());
        }
    }
    regions.push(region);
    Ok(())
}

impl Loader<'_, '_> {
    fn finish(mut self, cliw_origin: Option<u64>, image: &mut Image) -> KunaResult<()> {
        let imports = allocate_imports(self.sections, self.names, &self.fixups, image)?;
        for fixup in self.fixups {
            let n = fixup.bytes;
            let bytes = fixup.expr.encoded(n, fixup.swap, |id, at| known(&imports, id, at, "external"))?;
            self.runs[fixup.run].bytes[fixup.offset..fixup.offset + n].copy_from_slice(&bytes[..n]);
        }
        let sections = self.sections;
        self.runs.sort_unstable_by_key(|run| (sections[&run.section].space, run.start));
        for Run { section, start, mut bytes, copy } in self.runs {
            let sec = &sections[&section];
            let unit = width(sec.space) as u64;
            let mut start = start;
            let end = start + (bytes.len() as u64).div_ceil(unit);
            let origin = if sec.space == Space::Cliw { cliw_origin.unwrap_or(0) } else { 0 };
            let mut emit = |start, bytes: Vec<u8>, executable| {
                if executable && bytes.len() as u64 % unit != 0 {
                    return Err(err(sec.at, "executable load splits a CODE word"));
                }
                push_region(&mut image.regions, Region {
                    space: sec.space,
                    offset: (start - origin) * unit,
                    bytes,
                    readonly: sec.readonly,
                    executable,
                }, sec.at)
            };
            if let Some((first, last)) = self.copy.filter(|_| sec.executable && !copy) {
                if start < first && first < end {
                    let tail = bytes.split_off(((first - start) * unit) as usize);
                    emit(start, bytes, true)?;
                    bytes = tail;
                    start = first;
                }
                if (first..last).contains(&start) {
                    if end <= last { emit(start, bytes, false)?; continue; }
                    let tail = bytes.split_off(((last - start) * unit) as usize);
                    emit(start, bytes, false)?;
                    bytes = tail;
                    start = last;
                }
            }
            emit(start, bytes, sec.executable && !copy)?;
        }
        Ok(())
    }
}

fn entry(mut part: Cursor<'_>, loader: &Loader<'_, '_>) -> KunaResult<Option<u64>> {
    if part.rem.is_empty() { return Ok(None); }
    part.expect(&[0xe2, 0xc7, 0xbe])?;
    let expr = Expression::read(&mut part, true)?;
    part.expect(&[0xbf])?;
    part.end()?;
    let value = expr.eval(|var| loader.resolve(var, expr.at)?.ok_or_else(|| err(expr.at, "external module entry")))?;
    Ok(Some(address(value, Space::Code, expr.at)? * 3))
}

pub(super) fn parse(bytes: &[u8]) -> KunaResult<Image> {
    let header = wire::module(bytes)?;
    if header.processor != b"carmel" || header.bits_per_mau != 24
        || header.maus_per_address != Some(1) || header.order != Some(0xcc) {
        return Err(err(0, "unsupported Carmel address description"));
    }
    let part = header.parts;
    let kind = kind(part[0])?;
    let mut sections = sections(part[2], kind)?;
    place(&mut sections, kind)?;
    let names = names(part[3])?;
    let mut image = Image::default();
    let public = public_symbols(&sections, &names, &mut image)?;
    let copy = if kind == Kind::Relocatable { None } else { default_copy_bounds(&sections, &image) };
    image.symbols.retain(|sym| sym.space != Space::Code
        || !matches!(sym.name.as_slice(), b"__DUMMYNOPS" | b"__DUMMYNOPS2"));
    let cliw = cliw_origin(&sections, kind);
    let mut loader = Loader {
        kind, sections: &sections, names: &names, public: &public, copy,
        relative: Values::new(), position: Values::new(), selected: None,
        runs: Vec::new(), fixups: Vec::new(),
    };
    loader.data(part[5])?;
    if kind == Kind::IeeeAbsolute && loader.copy.is_none() {
        loader.copy = ieee_copy_tail(part[4], &sections, &loader.runs)?;
        if part[4].rem.is_empty() && loader.copy.is_none()
            && loader.runs.iter().any(|run| sections[&run.section].space == Space::Code)
            && sections.iter().any(|(id, sec)| sec.space == Space::Data && !sec.readonly
                && sec.size != Some(0) && loader.runs.iter().all(|run| run.section != *id)) {
            return Err(err(part[5].offset, "unsupported stripped absolute with possible CODE ROM copies and no section boundaries"));
        }
    }
    image.entry = entry(part[6], &loader)?;
    loader.finish(cliw, &mut image)?;
    image.normalized_cliw = cliw.is_some();
    image.relocatable = kind == Kind::Relocatable;
    Ok(image)
}
