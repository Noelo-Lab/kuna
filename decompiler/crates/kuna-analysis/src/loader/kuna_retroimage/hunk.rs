//! AmigaDOS hunk load file: `HUNK_HEADER` with the memory size of every
//! hunk, then each hunk's blocks (code, data or bss; relocations; symbols)
//! up to its `HUNK_END`.
//!
//! AmigaDOS allocates each hunk separately and patches every `HUNK_RELOC32`
//! word with the address its target hunk landed at. The loader places the
//! hunks one after another from [`BASE`] and applies the same patches, so
//! references between hunks read as addresses.

use kuna_base::error::KunaResult;
use object::SectionKind;

use super::{add_word, invalid, symbol_name, Bytes, RetroImage, RetroSection, RetroSymbol, MAX_MODULE};

const FORMAT: &str = "Amiga hunk";
pub const LANGUAGE: &str = "68000:BE:32:default";
/// Address of the first hunk.
pub const BASE: u64 = 0x1_0000;

const HUNK_NAME: u32 = 0x3e8;
const HUNK_CODE: u32 = 0x3e9;
const HUNK_DATA: u32 = 0x3ea;
const HUNK_BSS: u32 = 0x3eb;
const HUNK_RELOC32: u32 = 0x3ec;
const HUNK_RELOC16: u32 = 0x3ed;
const HUNK_RELOC8: u32 = 0x3ee;
const HUNK_SYMBOL: u32 = 0x3f0;
const HUNK_DEBUG: u32 = 0x3f1;
const HUNK_END: u32 = 0x3f2;
const HUNK_HEADER: u32 = 0x3f3;
const HUNK_OVERLAY: u32 = 0x3f5;
const HUNK_BREAK: u32 = 0x3f6;
const HUNK_DREL32: u32 = 0x3f7;
const HUNK_DREL16: u32 = 0x3f8;
const HUNK_DREL8: u32 = 0x3f9;
const HUNK_RELOC32SHORT: u32 = 0x3fc;
const HUNK_RELRELOC32: u32 = 0x3fd;
const HUNK_ABSRELOC16: u32 = 0x3fe;
const HUNK_PPC_CODE: u32 = 0x4e9;

/// `HUNK_HEADER`, no resident library names, and a hunk table whose bounds agree.
pub fn probe(prefix: &[u8]) -> bool {
    let file = Bytes { data: prefix, big: true };
    let (Some(id), Some(names), Some(size), Some(first), Some(last)) =
        (file.u32(0), file.u32(4), file.u32(8), file.u32(12), file.u32(16))
    else {
        return false;
    };
    id == HUNK_HEADER && names == 0 && size != 0 && size < 0x1_0000 && first <= last && last - first < size
}

struct Cursor<'a> {
    file: Bytes<'a>,
    at: u64,
}

impl Cursor<'_> {
    fn u32(&mut self) -> Option<u32> {
        let value = self.file.u32(self.at)?;
        self.at += 4;
        Some(value)
    }

    fn u16(&mut self) -> Option<u16> {
        let value = self.file.u16(self.at)?;
        self.at += 2;
        Some(value)
    }

    fn skip_longs(&mut self, longs: u32) -> Option<()> {
        let end = self.at.checked_add(u64::from(longs) * 4)?;
        (end <= self.file.data.len() as u64).then(|| self.at = end)
    }
}

struct Hunk {
    offset: u64,
    size: u64,
    kind: SectionKind,
    file_range: Option<(u64, u64)>,
}

pub fn parse(filename: &str, bytes: &[u8]) -> KunaResult<RetroImage> {
    let bad = |detail: &str| invalid(filename, FORMAT, detail);
    let truncated = || bad("truncated hunk stream");
    let mut cursor = Cursor { file: Bytes { data: bytes, big: true }, at: 4 };
    if cursor.u32().ok_or_else(truncated)? != 0 {
        return Err(bad("resident library names are not supported"));
    }
    let _table_size = cursor.u32().ok_or_else(truncated)?;
    let first = cursor.u32().ok_or_else(truncated)?;
    let last = cursor.u32().ok_or_else(truncated)?;
    let count = last.checked_sub(first).filter(|&span| span < 0x1_0000).ok_or_else(|| bad("bad hunk table bounds"))? + 1;

    let mut hunks = Vec::with_capacity(count as usize);
    let mut total = 0u64;
    for _ in 0..count {
        let word = cursor.u32().ok_or_else(truncated)?;
        if word >> 30 == 3 {
            cursor.u32().ok_or_else(truncated)?;
        }
        let size = u64::from(word & 0x3fff_ffff) * 4;
        hunks.push(Hunk { offset: total, size, kind: SectionKind::UninitializedData, file_range: None });
        total += size;
        if total > MAX_MODULE {
            return Err(bad("hunks exceed the image size limit"));
        }
    }
    if total == 0 {
        return Err(bad("every hunk is empty"));
    }

    let mut module = vec![0u8; total as usize];
    let mut relocs: Vec<(u64, u64)> = Vec::new();
    let mut symbols = Vec::new();
    let mut notes = Vec::new();
    let mut skipped = 0usize;
    'hunks: for index in 0..hunks.len() {
        let (offset, size) = (hunks[index].offset, hunks[index].size);
        loop {
            let Some(block) = cursor.u32() else { break 'hunks };
            match block & 0x3fff_ffff {
                HUNK_END => break,
                HUNK_NAME | HUNK_DEBUG => {
                    let longs = cursor.u32().ok_or_else(truncated)?;
                    cursor.skip_longs(longs).ok_or_else(truncated)?;
                }
                kind @ (HUNK_CODE | HUNK_DATA | HUNK_PPC_CODE) => {
                    let stored = u64::from(cursor.u32().ok_or_else(truncated)?) * 4;
                    let data = cursor.file.slice(cursor.at, stored).ok_or_else(truncated)?;
                    let kept = stored.min(size) as usize;
                    module[offset as usize..offset as usize + kept].copy_from_slice(&data[..kept]);
                    hunks[index].kind = if kind == HUNK_CODE { SectionKind::Text } else { SectionKind::Data };
                    hunks[index].file_range = Some((cursor.at, kept as u64));
                    cursor.at += stored;
                    if kind == HUNK_PPC_CODE {
                        notes.push(format!("hunk {index} holds PowerPC code, which is mapped as data"));
                    }
                }
                HUNK_BSS => {
                    cursor.u32().ok_or_else(truncated)?;
                }
                kind @ (HUNK_RELOC32 | HUNK_RELOC16 | HUNK_RELOC8 | HUNK_DREL16 | HUNK_DREL8 | HUNK_RELRELOC32
                | HUNK_ABSRELOC16) => loop {
                    let entries = cursor.u32().ok_or_else(truncated)?;
                    if entries == 0 {
                        break;
                    }
                    let target = cursor.u32().ok_or_else(truncated)?;
                    let target = target.checked_sub(first).and_then(|target| hunks.get(target as usize));
                    let target = target.map(|hunk| BASE + hunk.offset);
                    for _ in 0..entries {
                        let at = u64::from(cursor.u32().ok_or_else(truncated)?);
                        match target.filter(|_| kind == HUNK_RELOC32 && at + 4 <= size) {
                            Some(target) => relocs.push((offset + at, target)),
                            None if kind == HUNK_RELOC32 => skipped += 1,
                            None => {}
                        }
                    }
                },
                HUNK_RELOC32SHORT | HUNK_DREL32 => {
                    loop {
                        let entries = cursor.u16().ok_or_else(truncated)?;
                        if entries == 0 {
                            break;
                        }
                        let target = u32::from(cursor.u16().ok_or_else(truncated)?);
                        let target = target.checked_sub(first).and_then(|target| hunks.get(target as usize));
                        let target = target.map(|hunk| BASE + hunk.offset);
                        for _ in 0..entries {
                            let at = u64::from(cursor.u16().ok_or_else(truncated)?);
                            match target.filter(|_| at + 4 <= size) {
                                Some(target) => relocs.push((offset + at, target)),
                                None => skipped += 1,
                            }
                        }
                    }
                    cursor.at = cursor.at.next_multiple_of(4);
                }
                HUNK_SYMBOL => loop {
                    let longs = cursor.u32().ok_or_else(truncated)? & 0xff_ffff;
                    if longs == 0 {
                        break;
                    }
                    let raw = cursor.file.slice(cursor.at, u64::from(longs) * 4).ok_or_else(truncated)?;
                    cursor.at += u64::from(longs) * 4;
                    let value = u64::from(cursor.u32().ok_or_else(truncated)?);
                    let raw = &raw[..raw.iter().position(|&byte| byte == 0).unwrap_or(raw.len())];
                    if let Some(name) = symbol_name(raw).filter(|_| value < size) {
                        symbols.push((index, RetroSymbol { offset: offset + value, name, code: false }));
                    }
                },
                HUNK_OVERLAY | HUNK_BREAK => {
                    notes.push("the overlay tree and the hunks it loads on demand are not mapped".into());
                    break 'hunks;
                }
                other => return Err(bad(&format!("unknown block type {other:#x}"))),
            }
        }
    }
    for (at, target) in relocs {
        add_word(&mut module, at, target as u32, true);
    }
    if skipped != 0 {
        notes.push(format!("{skipped} relocation(s) name a word outside their hunk and were not applied"));
    }
    if hunks[0].size == 0 || hunks[0].kind != SectionKind::Text {
        return Err(bad("the first hunk holds no code"));
    }
    let symbols = symbols
        .into_iter()
        .map(|(index, symbol)| RetroSymbol { code: hunks[index].kind == SectionKind::Text, ..symbol })
        .collect();
    let sections = hunks
        .iter()
        .enumerate()
        .filter(|(_, hunk)| hunk.size != 0)
        .map(|(index, hunk)| RetroSection {
            name: format!("hunk{index}"),
            offset: hunk.offset,
            size: hunk.size,
            kind: hunk.kind,
            file_range: hunk.file_range,
        })
        .collect();
    Ok(RetroImage {
        format: FORMAT,
        language: LANGUAGE,
        base: BASE,
        module,
        entry: BASE,
        sections,
        symbols,
        tracked: Vec::new(),
        notes,
    })
}
