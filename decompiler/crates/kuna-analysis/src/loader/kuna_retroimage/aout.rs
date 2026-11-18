//! a.out: a 32-byte `exec` header, text, data, relocations, an `nlist`
//! symbol table and its strings.
//!
//! Three header dialects exist. NetBSD stores `a_midmag` in network order as
//! `flags<<26 | mid<<16 | magic`; SunOS stores the same word big-endian with
//! its machine type where NetBSD has the low `mid` byte; Linux and FreeBSD on
//! i386 store it in host (little-endian) order. The rest of the header is in
//! the machine's own byte order. Where the text begins in the file and in
//! memory depends on the dialect and the magic, so the loader takes the
//! dialect's address and falls back to the page the entry point sits in when
//! the entry is not inside the text that address gives.

use std::collections::BTreeMap;

use kuna_base::error::KunaResult;
use object::SectionKind;

use super::{invalid, symbol_name, Bytes, RetroImage, RetroSection, RetroSymbol, MAX_MODULE};

const FORMAT: &str = "a.out";
const HEADER: u64 = 32;
const OMAGIC: u32 = 0o407;
const NMAGIC: u32 = 0o410;
const ZMAGIC: u32 = 0o413;
const QMAGIC: u32 = 0o314;

/// How an 8-byte relocation record packs its bit-field word.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Relocs {
    /// `r_symbolnum` in the high 24 bits (m68k).
    Big,
    /// `r_symbolnum` in the low 24 bits (i386).
    Little,
    /// A record layout this loader does not apply.
    Unapplied,
}

/// One machine id: the language that decodes it and its load page size.
pub struct Machine {
    mid: u32,
    pub language: &'static str,
    big: bool,
    page: u64,
    relocs: Relocs,
}

const fn machine(mid: u32, language: &'static str, big: bool, page: u64, relocs: Relocs) -> Machine {
    Machine { mid, language, big, page, relocs }
}

const M68K: &str = "68000:BE:32:MC68020";
const I386: &str = "x86:LE:32:default";
const SPARC: &str = "sparc:BE:32:default";

/// The machine ids of `sys/aout_mids.h`, plus SunOS's SPARC machine type.
pub const MACHINES: &[Machine] = &[
    machine(0x001, M68K, true, 0x2000, Relocs::Big),
    machine(0x002, M68K, true, 0x2000, Relocs::Big),
    machine(0x003, SPARC, true, 0x2000, Relocs::Unapplied),
    machine(0x064, I386, false, 0x1000, Relocs::Little),
    machine(0x086, I386, false, 0x1000, Relocs::Little),
    machine(0x087, M68K, true, 0x2000, Relocs::Big),
    machine(0x088, M68K, true, 0x1000, Relocs::Big),
    machine(0x08a, SPARC, true, 0x2000, Relocs::Unapplied),
    machine(0x08f, "ARM:LE:32:v4", false, 0x1000, Relocs::Unapplied),
    machine(0x090, "68000:BE:32:default", true, 0x800, Relocs::Big),
    machine(0x0c8, M68K, true, 0x1000, Relocs::Big),
    machine(0x12c, M68K, true, 0x1000, Relocs::Big),
];

/// Where a dialect puts the text for one magic.
struct Layout {
    text_offset: u64,
    text_address: u64,
    /// Whether the data's file offset is rounded up to a page.
    page_aligned_file: bool,
    segment: u64,
}

struct Header {
    machine: &'static Machine,
    magic: u32,
    layout: Layout,
}

fn is_magic(magic: u32) -> bool {
    matches!(magic, OMAGIC | NMAGIC | ZMAGIC | QMAGIC)
}

fn header(prefix: &[u8]) -> Option<Header> {
    let raw: [u8; 4] = prefix.get(..4)?.try_into().ok()?;
    let network = u32::from_be_bytes(raw);
    let host = u32::from_le_bytes(raw);
    let find = |mid: u32| MACHINES.iter().find(|machine| machine.mid == mid);
    if is_magic(network & 0xffff) {
        // SunOS keeps a toolchain version above its machine type, 1 to 3.
        let sun = Some((network >> 16) & 0xff).filter(|mid| (1..=3).contains(mid));
        let machine = find((network >> 16) & 0x3ff).or_else(|| find(sun?))?;
        let magic = network & 0xffff;
        let paged = matches!(magic, ZMAGIC | QMAGIC);
        return Some(Header {
            machine,
            magic,
            layout: Layout {
                text_offset: if magic == ZMAGIC { 0 } else { HEADER },
                text_address: machine.page,
                page_aligned_file: paged,
                segment: machine.page,
            },
        });
    }
    if !is_magic(host & 0xffff) {
        return None;
    }
    let machine = find((host >> 16) & 0x3ff).filter(|machine| !machine.big)?;
    let magic = host & 0xffff;
    let linux = machine.mid == 0x064;
    Some(Header {
        machine,
        magic,
        layout: Layout {
            text_offset: match magic {
                ZMAGIC if linux => 1024,
                ZMAGIC => machine.page,
                QMAGIC => 0,
                _ => HEADER,
            },
            text_address: if magic == QMAGIC { machine.page } else { 0 },
            page_aligned_file: !linux && matches!(magic, ZMAGIC | QMAGIC),
            segment: if linux && magic != QMAGIC { 1024 } else { machine.page },
        },
    })
}

/// The magic and a known machine id, and segment sizes a file could hold.
pub fn probe(prefix: &[u8]) -> bool {
    let Some(header) = header(prefix) else { return false };
    let file = Bytes { data: prefix, big: header.machine.big };
    let (Some(text), Some(data), Some(syms)) = (file.u32(4), file.u32(8), file.u32(16)) else {
        return false;
    };
    text != 0 && u64::from(text) + u64::from(data) + u64::from(syms) <= MAX_MODULE
}

fn round_up(value: u64, to: u64) -> u64 {
    value.div_ceil(to) * to
}

pub fn parse(filename: &str, bytes: &[u8]) -> KunaResult<RetroImage> {
    let bad = |detail: &str| invalid(filename, FORMAT, detail);
    let Header { machine, magic, layout } = header(bytes).ok_or_else(|| bad("unknown magic or machine id"))?;
    let big = machine.big;
    let file = Bytes { data: bytes, big };
    let field = |at| file.u32(at).map(u64::from).ok_or_else(|| bad("truncated 32-byte header"));
    let (a_text, a_data, a_bss, a_syms) = (field(4)?, field(8)?, field(12)?, field(16)?);
    let (a_entry, a_trsize, a_drsize) = (field(20)?, field(24)?, field(28)?);
    if a_text == 0 {
        return Err(bad("empty text segment"));
    }
    let text = file
        .slice(layout.text_offset, a_text)
        .ok_or_else(|| bad("text segment extends past end of file"))?;
    let data_offset = if layout.page_aligned_file {
        round_up(layout.text_offset + a_text, machine.page)
    } else {
        layout.text_offset + a_text
    };
    let data = file.slice(data_offset, a_data).ok_or_else(|| bad("data segment extends past end of file"))?;

    let relocatable = a_entry == 0 && (magic == OMAGIC || a_trsize + a_drsize != 0);
    let entry_in = |address: u64| a_entry >= address && a_entry - address < a_text;
    let header_in_text = if layout.text_offset == 0 { HEADER } else { 0 };
    let text_address = if relocatable {
        0
    } else if entry_in(layout.text_address) {
        layout.text_address
    } else if entry_in(0) {
        0
    } else {
        a_entry.saturating_sub(header_in_text) & !(machine.page - 1)
    };
    if !relocatable && !entry_in(text_address) {
        return Err(bad("entry point is outside the text segment"));
    }
    let data_address = if magic == OMAGIC {
        text_address + a_text
    } else {
        round_up(text_address + a_text, layout.segment)
    };
    let bss_address = data_address + a_data;
    let loaded = bss_address + a_bss - text_address;
    if loaded > MAX_MODULE || bss_address + a_bss > 1 << 32 {
        return Err(bad("segments do not fit the address space"));
    }

    let mut module = vec![0u8; loaded as usize];
    module[..text.len()].copy_from_slice(text);
    let data_at = (data_address - text_address) as usize;
    module[data_at..data_at + data.len()].copy_from_slice(data);
    let mut sections = vec![RetroSection {
        name: ".text".into(),
        offset: 0,
        size: a_text,
        kind: SectionKind::Text,
        file_range: Some((layout.text_offset, a_text)),
    }];
    if a_data != 0 {
        sections.push(RetroSection {
            name: ".data".into(),
            offset: data_address - text_address,
            size: a_data,
            kind: SectionKind::Data,
            file_range: Some((data_offset, a_data)),
        });
    }
    if a_bss != 0 {
        sections.push(RetroSection {
            name: ".bss".into(),
            offset: bss_address - text_address,
            size: a_bss,
            kind: SectionKind::UninitializedData,
            file_range: None,
        });
    }

    let mut notes = Vec::new();
    let text_relocs = data_offset + a_data;
    let symbols_at = text_relocs + a_trsize + a_drsize;
    let table = SymbolTable::read(file, symbols_at, a_syms);
    if a_syms != 0 && table.entries.is_empty() {
        notes.push("the symbol table extends past end of file and was not read".into());
    }
    let mut symbols = table.defined(text_address, loaded);
    let mut entry = a_entry;
    if relocatable {
        let externals = Externals::place(&table, &mut module, &mut sections, &mut symbols);
        let relocs = [(text_relocs, a_trsize, 0), (text_relocs + a_trsize, a_drsize, data_address)];
        if a_trsize + a_drsize != 0 && machine.relocs == Relocs::Unapplied {
            notes.push("relocations of this machine are not applied; references to undefined symbols read as zero".into());
        } else {
            let mut skipped = 0usize;
            for (at, size, segment) in relocs {
                skipped += externals.apply(file, at, size, segment, machine.relocs, &mut module);
            }
            if skipped != 0 {
                notes.push(format!("{skipped} relocation(s) name a field outside the image and were not applied"));
            }
        }
        entry = symbols.iter().filter(|symbol| symbol.code).map(|symbol| symbol.offset).min().unwrap_or(0);
    }
    Ok(RetroImage {
        format: FORMAT,
        language: machine.language,
        base: text_address,
        module,
        entry: if relocatable { text_address + entry } else { entry },
        sections,
        symbols,
        tracked: Vec::new(),
        notes,
    })
}

const N_EXT: u8 = 0x01;
const N_TYPE: u8 = 0x1e;
const N_STAB: u8 = 0xe0;
const N_UNDF: u8 = 0x00;
const N_TEXT: u8 = 0x04;
const N_DATA: u8 = 0x06;
const N_BSS: u8 = 0x08;

struct Nlist {
    name: Option<String>,
    kind: u8,
    value: u64,
}

/// A C symbol without the underscore the a.out toolchains prepend.
fn c_name(name: &str) -> &str {
    name.strip_prefix('_').filter(|rest| !rest.is_empty()).unwrap_or(name)
}

struct SymbolTable {
    entries: Vec<Nlist>,
}

impl SymbolTable {
    /// The `nlist` records at `at`; empty when the table or its strings are
    /// not wholly inside the file.
    fn read(file: Bytes, at: u64, size: u64) -> Self {
        let strings = at + size;
        let mut entries = Vec::new();
        if file.slice(at, size).is_none() {
            return Self { entries };
        }
        for record in (0..size / 12).map(|index| at + index * 12) {
            let (Some(strx), Some(kind), Some(value)) = (file.u32(record), file.u8(record + 4), file.u32(record + 8))
            else {
                break;
            };
            let name = (strx >= 4)
                .then(|| file.cstr(strings + u64::from(strx)))
                .flatten()
                .and_then(symbol_name);
            entries.push(Nlist { name, kind, value: u64::from(value) });
        }
        Self { entries }
    }

    /// The text, data and bss symbols that name an address inside the image.
    /// Debugger records, object-file names and assembler-local labels are not
    /// names of the program.
    fn defined(&self, base: u64, loaded: u64) -> Vec<RetroSymbol> {
        let mut by_offset: BTreeMap<(u64, bool), (bool, &str)> = BTreeMap::new();
        for entry in &self.entries {
            let section = entry.kind & N_TYPE;
            let Some(name) = entry.name.as_deref() else { continue };
            if entry.kind & N_STAB != 0
                || !matches!(section, N_TEXT | N_DATA | N_BSS)
                || name.ends_with(".o")
                || name.starts_with('L')
                || name.contains("_compiled")
            {
                continue;
            }
            let Some(offset) = entry.value.checked_sub(base).filter(|&offset| offset < loaded) else { continue };
            let external = entry.kind & N_EXT != 0;
            let name = c_name(name);
            let slot = by_offset.entry((offset, section == N_TEXT)).or_insert((external, name));
            if external && !slot.0 {
                *slot = (external, name);
            }
        }
        by_offset
            .into_iter()
            .map(|((offset, code), (_, name))| RetroSymbol { offset, name: name.to_owned(), code })
            .collect()
    }
}

/// The addresses a relocatable object's undefined and common symbols get: a
/// range appended after the bss, one slot per symbol.
struct Externals {
    /// Symbol-table index to module offset.
    slots: BTreeMap<u32, u64>,
}

impl Externals {
    fn place(
        table: &SymbolTable,
        module: &mut Vec<u8>,
        sections: &mut Vec<RetroSection>,
        symbols: &mut Vec<RetroSymbol>,
    ) -> Self {
        let start = (module.len() as u64).next_multiple_of(4);
        let mut next = start;
        let mut slots = BTreeMap::new();
        for (index, entry) in table.entries.iter().enumerate() {
            if entry.kind & N_TYPE != N_UNDF || entry.kind & (N_STAB | N_EXT) != N_EXT {
                continue;
            }
            let Some(name) = entry.name.as_ref() else { continue };
            let size = entry.value.clamp(4, 0x1_0000).next_multiple_of(4);
            if next + size > MAX_MODULE {
                break;
            }
            slots.insert(index as u32, next);
            symbols.push(RetroSymbol { offset: next, name: c_name(name).to_owned(), code: entry.value == 0 });
            next += size;
        }
        if next != start {
            module.resize(next as usize, 0);
            sections.push(RetroSection {
                name: ".extern".into(),
                offset: start,
                size: next - start,
                kind: SectionKind::UninitializedData,
                file_range: None,
            });
        }
        Self { slots }
    }

    /// Add each external symbol's slot address to the fields its relocation
    /// records name; returns how many records fell outside the image.
    /// Section-relative records need nothing: the object is loaded at the
    /// addresses it was assembled for.
    fn apply(&self, file: Bytes, at: u64, size: u64, segment: u64, relocs: Relocs, module: &mut [u8]) -> usize {
        let mut skipped = 0;
        for record in (0..size / 8).map(|index| at + index * 8) {
            let (Some(address), Some(info)) = (file.u32(record), file.u32(record + 4)) else { break };
            let (symbol, length, external) = match relocs {
                Relocs::Big => (info >> 8, (info >> 5) & 3, info & 0x10 != 0),
                Relocs::Little => (info & 0xff_ffff, (info >> 25) & 3, info & 0x800_0000 != 0),
                Relocs::Unapplied => return 0,
            };
            let Some(&slot) = external.then(|| self.slots.get(&symbol)).flatten() else { continue };
            let width = 1usize << length;
            let field = usize::try_from(segment + u64::from(address)).ok().and_then(|start| {
                module.get_mut(start..start.checked_add(width)?)
            });
            let Some(field) = field.filter(|_| width <= 4) else {
                skipped += 1;
                continue;
            };
            let mut raw = [0u8; 4];
            if file.big {
                raw[4 - width..].copy_from_slice(field);
                let value = u32::from_be_bytes(raw).wrapping_add(slot as u32);
                field.copy_from_slice(&value.to_be_bytes()[4 - width..]);
            } else {
                raw[..width].copy_from_slice(field);
                let value = u32::from_le_bytes(raw).wrapping_add(slot as u32);
                field.copy_from_slice(&value.to_le_bytes()[..width]);
            }
        }
        skipped
    }
}
