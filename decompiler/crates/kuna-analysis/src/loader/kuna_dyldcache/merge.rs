//! Merge several rebuilt cache images into one thin Mach-O.
//!
//! `dyldimage A+B` loads more than one image of a cache as one program, so a
//! call from one into another can be followed without a second run. Each image
//! is first rebuilt on its own ([`DyldCache::extract`]); this module joins the
//! results. The merged file opens with a header page of its own, since the
//! commands of several images need not fit in the padding of one. Every
//! non-`__LINKEDIT` segment follows at its cache address, images in address
//! order, and one `__LINKEDIT` closes the file.
//!
//! That `__LINKEDIT` holds one symbol table (locals, then external
//! definitions, then undefined symbols, each symbol's section ordinal moved
//! past the sections of the images before it), one indirect-symbol table (each
//! section's `reserved1` moved the same way), one function-starts table counted
//! from the lowest `__TEXT`, and one `kuna.dyld.xrefs` note, without the call
//! targets that now lie inside a merged image. The identity commands
//! (`LC_ID_DYLIB`, `LC_UUID`, the dylib list) are those of the image named
//! first. Export tries and data-in-code are not carried: every export of a
//! cache image is also an external definition in its symbol table.

use super::extract::{self, align, put_u32, put_u64, ImageLayout, Linkedit, PAGE};
use super::{u32_at, xrefs, DyldCache};

const N_STAB: u8 = 0xe0;
const N_TYPE: u8 = 0x0e;
const N_SECT: u8 = 0x0e;
const N_EXT: u8 = 0x01;
/// A Mach-O symbol names its section in one byte.
const MAX_SECTIONS: usize = 255;
/// The section types whose `reserved1` indexes the indirect-symbol table: symbol
/// pointers, stubs and thread-local variable pointers.
const USES_INDIRECT: [u32; 5] = [0x6, 0x7, 0x8, 0x10, 0x14];

/// One rebuilt image, parsed back.
struct Part<'a> {
    bytes: &'a [u8],
    layout: ImageLayout,
}

impl<'a> Part<'a> {
    fn parse(address: u64, bytes: &'a [u8]) -> Result<Part<'a>, String> {
        let header = bytes.get(..32).ok_or("rebuilt image is shorter than a Mach-O header")?;
        let cmds = bytes
            .get(32..32usize.saturating_add(u32_at(header, 20) as usize))
            .ok_or("rebuilt image is shorter than its load commands")?;
        Ok(Part { bytes, layout: extract::parse_layout(address, header.to_vec(), cmds.to_vec()) })
    }

    fn text_base(&self) -> u64 {
        self.layout.segments.iter().find(|s| s.name == "__TEXT").map_or(self.layout.address, |s| s.vmaddr)
    }

    fn slice(&self, offset: u64, len: u64) -> Result<&'a [u8], String> {
        usize::try_from(offset)
            .ok()
            .zip(usize::try_from(len).ok())
            .and_then(|(o, l)| self.bytes.get(o..o.checked_add(l)?))
            .ok_or_else(|| "rebuilt image data lies past its end".to_string())
    }

    fn linkedit_data(&self, kind: u32) -> Result<&'a [u8], String> {
        match self.layout.commands().into_iter().find(|c| c.0 == kind) {
            Some((_, at)) => self.slice(self.layout.cmd_u32(at + 8) as u64, self.layout.cmd_u32(at + 12) as u64),
            None => Ok(&[]),
        }
    }
}

fn uleb(data: &[u8], at: &mut usize) -> Option<u64> {
    let mut value = 0u64;
    for shift in (0..64).step_by(7) {
        let byte = *data.get(*at)?;
        *at += 1;
        value |= u64::from(byte & 0x7f) << shift;
        if byte & 0x80 == 0 {
            return Some(value);
        }
    }
    None
}

fn put_uleb(out: &mut Vec<u8>, mut value: u64) {
    loop {
        let byte = (value & 0x7f) as u8;
        value >>= 7;
        out.push(if value == 0 { byte } else { byte | 0x80 });
        if value == 0 {
            return;
        }
    }
}

/// The addresses an `LC_FUNCTION_STARTS` table lists, counted from `base`.
fn function_starts(table: &[u8], base: u64, out: &mut Vec<u64>) {
    let (mut at, mut addr) = (0, base);
    while let Some(delta) = uleb(table, &mut at).filter(|&d| d != 0) {
        addr = addr.wrapping_add(delta);
        out.push(addr);
    }
}

impl DyldCache {
    /// Rebuild the images `indices` as one thin Mach-O. Entries sharing a
    /// header address count once; a single image is [`DyldCache::extract`]
    /// unchanged.
    pub fn extract_merged(&self, indices: &[usize]) -> Result<Vec<u8>, String> {
        let mut chosen: Vec<(u64, usize)> = Vec::new();
        for &index in indices {
            let address = self.images.get(index).ok_or("image index out of range")?.address;
            if !chosen.iter().any(|c| c.0 == address) {
                chosen.push((address, index));
            }
        }
        let primary = chosen.first().ok_or("no image selected")?.0;
        if chosen.len() == 1 {
            return self.extract(chosen[0].1);
        }
        let rebuilt = chosen
            .iter()
            .map(|&(address, index)| Ok((address, self.extract(index)?)))
            .collect::<Result<Vec<_>, String>>()?;
        let mut parts =
            rebuilt.iter().map(|(address, bytes)| Part::parse(*address, bytes)).collect::<Result<Vec<_>, _>>()?;
        parts.sort_by_key(Part::text_base);
        merge(&parts, primary)
    }
}

/// The merged `__LINKEDIT` contents, before they are placed.
#[derive(Default)]
struct Tables {
    /// The nlists by class: locals, external definitions, undefined.
    symbols: [Vec<u8>; 3],
    strings: Vec<u8>,
    indirect: Vec<u8>,
    /// Where each part's entries start in `indirect`.
    indirect_bases: Vec<u32>,
    function_starts: Vec<u8>,
    notes: Vec<xrefs::Xref>,
}

impl Tables {
    fn counts(&self) -> [u32; 3] {
        [0, 1, 2].map(|class| (self.symbols[class].len() / 16) as u32)
    }

    /// The first merged index of each symbol class.
    fn firsts(&self) -> [u32; 3] {
        let [locals, extdefs, _] = self.counts();
        [0, locals, locals + extdefs]
    }

    /// Append `part`'s symbols, each section ordinal moved up by
    /// `section_base`; returns each one's class and index within it.
    fn add_symbols(&mut self, part: &Part, section_base: usize) -> Result<Vec<(usize, u32)>, String> {
        let layout = &part.layout;
        let Some(at) = layout.symtab else {
            return Ok(Vec::new());
        };
        let nlists = part.slice(layout.cmd_u32(at + 8) as u64, (layout.cmd_u32(at + 12) as u64).saturating_mul(16))?;
        let pool = part.slice(layout.cmd_u32(at + 16) as u64, layout.cmd_u32(at + 20) as u64)?;
        let mut placed = Vec::with_capacity(nlists.len() / 16);
        for n in nlists.chunks_exact(16) {
            let name = pool.get(u32_at(n, 0) as usize..).unwrap_or(&[]);
            let name = &name[..name.iter().position(|&b| b == 0).unwrap_or(name.len())];
            let mut strx = 0;
            if !name.is_empty() {
                strx = u32::try_from(self.strings.len()).map_err(|_| "merged string table exceeds 4 GiB")?;
                self.strings.extend(name);
                self.strings.push(0);
            }
            let (n_type, mut n_sect) = (n[4], n[5]);
            if n_type & N_STAB == 0 && n_type & N_TYPE == N_SECT && n_sect != 0 {
                n_sect = (n_sect as usize + section_base).min(MAX_SECTIONS) as u8;
            }
            let class = if n_type & N_STAB != 0 || n_type & N_EXT == 0 {
                0
            } else if n_type & N_TYPE != 0 {
                1
            } else {
                2
            };
            let list = &mut self.symbols[class];
            placed.push((class, (list.len() / 16) as u32));
            list.extend(strx.to_le_bytes());
            list.extend([n_type, n_sect]);
            list.extend(&n[6..16]);
        }
        Ok(placed)
    }

    fn gather(parts: &[Part]) -> Result<Tables, String> {
        let holds = |va: u64| parts.iter().any(|p| p.layout.holds(va));
        let mut tables = Tables { strings: vec![0], ..Tables::default() };
        let mut placed = Vec::new();
        let mut indirect: Vec<(usize, u32)> = Vec::new();
        let mut starts = Vec::new();
        let mut section_base = 0;
        for (k, part) in parts.iter().enumerate() {
            let layout = &part.layout;
            placed.push(tables.add_symbols(part, section_base)?);
            section_base += layout.segments.iter().map(|s| s.sections.len()).sum::<usize>();
            tables.indirect_bases.push(indirect.len() as u32);
            if let Some(at) = layout.dysymtab {
                let (off, count) = (layout.cmd_u32(at + 56) as u64, layout.cmd_u32(at + 60) as u64);
                indirect.extend(part.slice(off, count.saturating_mul(4))?.chunks_exact(4).map(|v| (k, u32_at(v, 0))));
            }
            function_starts(part.linkedit_data(extract::LC_FUNCTION_STARTS)?, part.text_base(), &mut starts);
            tables.notes.extend(
                xrefs::read_note(part.bytes)
                    .into_iter()
                    .filter(|x| x.kind != xrefs::XrefKind::Call || !holds(x.addr)),
            );
        }
        let firsts = tables.firsts();
        for (k, v) in indirect {
            let moved = if v & (extract::INDIRECT_SYMBOL_LOCAL | extract::INDIRECT_SYMBOL_ABS) != 0 {
                v
            } else {
                placed[k].get(v as usize).map_or(extract::INDIRECT_SYMBOL_LOCAL, |&(class, at)| firsts[class] + at)
            };
            tables.indirect.extend(moved.to_le_bytes());
        }
        starts.sort_unstable();
        starts.dedup();
        let mut last = parts.first().map_or(0, Part::text_base);
        for addr in starts {
            if addr > last {
                put_uleb(&mut tables.function_starts, addr - last);
                last = addr;
            }
        }
        tables.function_starts.push(0);
        tables.notes.sort_by_key(|x| (x.addr, x.kind as u32));
        tables.notes.dedup_by(|a, b| a.addr == b.addr && a.kind == b.kind);
        Ok(tables)
    }
}

/// Where the tables landed in the merged `__LINKEDIT`, as file offsets.
struct Placed {
    symoff: u32,
    stroff: u32,
    indirectoff: u32,
    startsoff: u32,
}

/// Point one of the identity image's commands at the merged tables; a command
/// for linkedit data the merge does not carry is zeroed.
fn retarget(cmd: u32, raw: &mut [u8], tables: &Tables, at: &Placed) {
    match cmd {
        extract::LC_SYMTAB => {
            put_u32(raw, 8, at.symoff);
            put_u32(raw, 12, tables.counts().iter().sum());
            put_u32(raw, 16, at.stroff);
            put_u32(raw, 20, tables.strings.len() as u32);
        }
        extract::LC_DYSYMTAB => {
            for class in 0..3 {
                put_u32(raw, 8 + class * 8, tables.firsts()[class]);
                put_u32(raw, 12 + class * 8, tables.counts()[class]);
            }
            for field in (32..80).step_by(4) {
                put_u32(raw, field, 0);
            }
            if !tables.indirect.is_empty() {
                put_u32(raw, 56, at.indirectoff);
                put_u32(raw, 60, (tables.indirect.len() / 4) as u32);
            }
        }
        extract::LC_DYLD_INFO | extract::LC_DYLD_INFO_ONLY => {
            for field in (8..48).step_by(4) {
                put_u32(raw, field, 0);
            }
        }
        extract::LC_FUNCTION_STARTS => {
            put_u32(raw, 8, at.startsoff);
            put_u32(raw, 12, tables.function_starts.len() as u32);
        }
        c if extract::LINKEDIT_DATA_COMMANDS.contains(&c) => {
            put_u32(raw, 8, 0);
            put_u32(raw, 12, 0);
        }
        _ => {}
    }
}

/// Every non-`__LINKEDIT` segment command of `parts`, as `(part, segment,
/// bytes)`, its sections' `reserved1` moved to the merged indirect-symbol table.
fn segment_commands(parts: &[Part], tables: &Tables) -> Result<Vec<(usize, usize, Vec<u8>)>, String> {
    let mut out = Vec::new();
    for (k, part) in parts.iter().enumerate() {
        for (s, seg) in part.layout.segments.iter().enumerate().filter(|(_, s)| s.name != "__LINKEDIT") {
            let mut raw = part.layout.command(seg.cmd_offset)?.to_vec();
            for sec in seg.sections.iter().filter(|sec| USES_INDIRECT.contains(&sec.section_type())) {
                let moved = sec.reserved1.wrapping_add(tables.indirect_bases[k]);
                put_u32(&mut raw, sec.cmd_offset - seg.cmd_offset + 68, moved);
            }
            out.push((k, s, raw));
        }
    }
    Ok(out)
}

/// Join `parts`, sorted by address; `primary` is the header address of the
/// image whose identity commands the result keeps.
fn merge(parts: &[Part], primary: u64) -> Result<Vec<u8>, String> {
    let main = parts.iter().find(|p| p.layout.address == primary).ok_or("no image selected")?;
    let sections: usize = parts.iter().flat_map(|p| &p.layout.segments).map(|s| s.sections.len()).sum();
    if sections > MAX_SECTIONS {
        return Err(format!(
            "the selected images hold {sections} sections and one Mach-O symbol table can index {MAX_SECTIONS}; \
             load fewer images at once"
        ));
    }
    let tables = Tables::gather(parts)?;
    let mut segments = segment_commands(parts, &tables)?;
    let le_seg = main.layout.linkedit().ok_or("image has no __LINKEDIT segment")?;
    let mut le_cmd = main.layout.command(le_seg.cmd_offset)?.to_vec();
    let mut other = Vec::new();
    for (cmd, at) in main.layout.commands() {
        let raw = main.layout.command(at)?;
        let own_note = cmd == extract::LC_NOTE && raw.get(8..24) == Some(xrefs::NOTE_OWNER);
        if cmd != extract::LC_SEGMENT_64 && !own_note {
            other.push((cmd, raw.to_vec()));
        }
    }
    let cmds_len = segments.iter().map(|c| c.2.len()).sum::<usize>()
        + le_cmd.len()
        + other.iter().map(|c| c.1.len()).sum::<usize>()
        + if tables.notes.is_empty() { 0 } else { 40 };

    let too_big = || "merged image exceeds 4 GiB".to_string();
    let mut out = vec![0u8; align(32 + cmds_len as u64, PAGE) as usize];
    for (k, s, raw) in &mut segments {
        let (part, seg) = (&parts[*k], &parts[*k].layout.segments[*s]);
        let offset = out.len() as u64;
        out.extend(part.slice(seg.fileoff, seg.filesize)?);
        out.resize(align(out.len() as u64, PAGE) as usize, 0);
        put_u64(raw, 40, offset);
        for sec in &seg.sections {
            let at = sec.cmd_offset - seg.cmd_offset + 48;
            let old = u32_at(raw, at) as u64;
            if old != 0 {
                let moved = old.checked_sub(seg.fileoff).and_then(|d| u32::try_from(offset + d).ok());
                put_u32(raw, at, moved.ok_or_else(too_big)?);
            }
        }
    }

    let mut le = Linkedit::default();
    let le_offset = out.len() as u64;
    let mut place = |data: &[u8]| u32::try_from(le_offset + le.push(data)).map_err(|_| too_big());
    let placed = Placed {
        symoff: place(&tables.symbols.concat())?,
        indirectoff: place(&tables.indirect)?,
        startsoff: place(&tables.function_starts)?,
        stroff: place(&tables.strings)?,
    };
    let note = match tables.notes.is_empty() {
        true => None,
        false => {
            let blob = xrefs::encode(&tables.notes);
            Some((place(&blob)? as u64, blob.len() as u64))
        }
    };
    put_u64(&mut le_cmd, 32, align(le.bytes.len() as u64, PAGE));
    put_u64(&mut le_cmd, 40, le_offset);
    put_u64(&mut le_cmd, 48, le.bytes.len() as u64);
    out.extend(&le.bytes);

    let mut cmds: Vec<u8> = segments.iter().flat_map(|c| c.2.iter().copied()).collect();
    cmds.extend(&le_cmd);
    let mut ncmds = segments.len() as u32 + 1 + other.len() as u32;
    for (cmd, mut raw) in other {
        retarget(cmd, &mut raw, &tables, &placed);
        cmds.extend(raw);
    }
    if let Some((off, size)) = note {
        cmds.extend(extract::LC_NOTE.to_le_bytes());
        cmds.extend(40u32.to_le_bytes());
        cmds.extend(xrefs::NOTE_OWNER);
        cmds.extend(off.to_le_bytes());
        cmds.extend(size.to_le_bytes());
        ncmds += 1;
    }
    let mut header = main.layout.header.clone();
    put_u32(&mut header, 16, ncmds);
    put_u32(&mut header, 20, cmds.len() as u32);
    out[..32].copy_from_slice(&header);
    out[32..32 + cmds.len()].copy_from_slice(&cmds);
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    const LINKEDIT: u64 = 0x7_0000_0000;
    const STUB: u32 = 0x8000_0408;

    /// A thin Mach-O at `base`: `__TEXT` with `__text` (section 1) and a
    /// 12-byte-stub `__stubs` (section 2) whose two stubs import `imports`, the
    /// function `local` 0x1000 past the base, and a function-starts table.
    fn thin(base: u64, local: &str, imports: [&str; 2], starts: &[u64]) -> Vec<u8> {
        let mut strings = vec![0u8];
        let mut nlists = Vec::new();
        for (name, n_type, n_sect, value) in
            [(local, 0x0fu8, 1u8, base + 0x1000), (imports[0], 0x01, 0, 0), (imports[1], 0x01, 0, 0)]
        {
            nlists.extend((strings.len() as u32).to_le_bytes());
            nlists.extend([n_type, n_sect, 0, 0]);
            nlists.extend(value.to_le_bytes());
            strings.extend(name.as_bytes());
            strings.push(0);
        }
        let indirect: Vec<u8> = [2u32, 1].iter().flat_map(|v| v.to_le_bytes()).collect();
        let mut table = Vec::new();
        let mut last = base;
        for &addr in starts {
            put_uleb(&mut table, addr - last);
            last = addr;
        }
        let mut le = Linkedit::default();
        let (symoff, indirectoff, startsoff, stroff) =
            (le.push(&nlists), le.push(&indirect), le.push(&table), le.push(&strings));

        let name16 = |s: &str| {
            let mut raw = s.as_bytes().to_vec();
            raw.resize(16, 0);
            raw
        };
        let segment = |name: &str, vmaddr: u64, fileoff: u64, size: u64, sections: &[(&str, u64, u64, u32, u32, u32)]| {
            let mut cmd = extract::LC_SEGMENT_64.to_le_bytes().to_vec();
            cmd.extend((72 + 80 * sections.len() as u32).to_le_bytes());
            cmd.extend(name16(name));
            for v in [vmaddr, 0x4000, fileoff, size] {
                cmd.extend(v.to_le_bytes());
            }
            for v in [5u32, 5, sections.len() as u32, 0] {
                cmd.extend(v.to_le_bytes());
            }
            for &(sect, offset, size, flags, reserved1, reserved2) in sections {
                cmd.extend(name16(sect));
                cmd.extend(name16(name));
                cmd.extend((vmaddr + offset).to_le_bytes());
                cmd.extend(size.to_le_bytes());
                for v in [offset as u32, 2, 0, 0, flags, reserved1, reserved2, 0] {
                    cmd.extend(v.to_le_bytes());
                }
            }
            cmd
        };
        let words = |cmd: u32, fields: &[u32]| -> Vec<u8> {
            [cmd, 8 + 4 * fields.len() as u32].iter().chain(fields).flat_map(|v| v.to_le_bytes()).collect()
        };
        let le_at = |off: u64| (0x4000 + off) as u32;
        let mut dysymtab = vec![0u32, 0, 0, 1, 1, 2];
        dysymtab.resize(18, 0);
        (dysymtab[12], dysymtab[13]) = (le_at(indirectoff), 2);
        let cmds = [
            segment("__TEXT", base, 0, 0x4000, &[("__text", 0x1000, 8, 0x8000_0400, 0, 0), ("__stubs", 0x2000, 24, STUB, 0, 12)]),
            segment("__LINKEDIT", LINKEDIT, 0x4000, le.bytes.len() as u64, &[]),
            words(extract::LC_SYMTAB, &[le_at(symoff), 3, le_at(stroff), strings.len() as u32]),
            words(extract::LC_DYSYMTAB, &dysymtab),
            words(extract::LC_FUNCTION_STARTS, &[le_at(startsoff), table.len() as u32]),
        ]
        .concat();
        let mut out: Vec<u8> =
            [0xfeed_facfu32, 0x0100_000c, 2, 6, 5, cmds.len() as u32, 0, 0].iter().flat_map(|v| v.to_le_bytes()).collect();
        out.extend(cmds);
        out.resize(0x4000, 0);
        out.extend(&le.bytes);
        out
    }

    fn merged() -> Vec<u8> {
        let (low, high) = (0x1_0000_0000u64, 0x2_0000_0000u64);
        let a = thin(high, "_high_fn", ["_malloc", "_free"], &[high + 0x1000, high + 0x1004]);
        let b = thin(low, "_low_fn", ["_open", "_close"], &[low + 0x1000]);
        let mut parts = vec![Part::parse(high, &a).unwrap(), Part::parse(low, &b).unwrap()];
        parts.sort_by_key(Part::text_base);
        merge(&parts, high).unwrap()
    }

    #[test]
    fn symbols_keep_their_sections_and_stubs_their_imports() {
        use object::{Object, ObjectSection, ObjectSymbol};
        let bytes = merged();
        let file = object::File::parse(bytes.as_slice()).unwrap();
        for (name, addr) in [("_low_fn", 0x1_0000_1000u64), ("_high_fn", 0x2_0000_1000)] {
            let sym = file.symbols().find(|s| s.name() == Ok(name)).unwrap();
            let section = file.section_by_index(sym.section_index().unwrap()).unwrap();
            assert_eq!((sym.address(), section.name().unwrap(), section.address()), (addr, "__text", addr), "{name}");
        }

        let part = Part::parse(0x2_0000_0000, &bytes).unwrap();
        let layout = &part.layout;
        let (symtab, dysymtab) = (layout.symtab.unwrap(), layout.dysymtab.unwrap());
        let fields: Vec<u32> = (0..6).map(|i| layout.cmd_u32(dysymtab + 8 + i * 4)).collect();
        assert_eq!(fields, [0, 0, 0, 2, 2, 4], "no locals, two definitions, four imports");
        let nlists = part.slice(layout.cmd_u32(symtab + 8) as u64, 6 * 16).unwrap();
        let pool = part.slice(layout.cmd_u32(symtab + 16) as u64, layout.cmd_u32(symtab + 20) as u64).unwrap();
        let name = |index: u32| {
            let at = u32_at(nlists, index as usize * 16) as usize;
            String::from_utf8_lossy(&pool[at..at + pool[at..].iter().position(|&b| b == 0).unwrap()]).into_owned()
        };
        let indirect = part.slice(layout.cmd_u32(dysymtab + 56) as u64, 16).unwrap();
        let stubs: Vec<(u64, Vec<String>)> = layout
            .segments
            .iter()
            .flat_map(|s| &s.sections)
            .filter(|s| s.sectname == "__stubs")
            .map(|s| (s.addr, (0..2).map(|i| name(u32_at(indirect, (s.reserved1 as usize + i) * 4))).collect()))
            .collect();
        assert_eq!(
            stubs,
            [
                (0x1_0000_2000, vec!["_close".to_string(), "_open".to_string()]),
                (0x2_0000_2000, vec!["_free".to_string(), "_malloc".to_string()]),
            ]
        );
    }

    #[test]
    fn function_starts_count_from_the_lowest_text() {
        let bytes = merged();
        let part = Part::parse(0x2_0000_0000, &bytes).unwrap();
        assert_eq!(part.layout.segments.iter().map(|s| s.name.as_str()).collect::<Vec<_>>(), ["__TEXT", "__TEXT", "__LINKEDIT"]);
        let mut starts = Vec::new();
        function_starts(part.linkedit_data(extract::LC_FUNCTION_STARTS).unwrap(), 0x1_0000_0000, &mut starts);
        assert_eq!(starts, [0x1_0000_1000, 0x2_0000_1000, 0x2_0000_1004]);
    }

    #[test]
    fn more_sections_than_a_symbol_can_index_are_refused() {
        let image = thin(0x1_0000_0000, "_f", ["_a", "_b"], &[]);
        let parts: Vec<Part> = (0..128).map(|i| Part::parse(0x1_0000_0000 + i * 0x4000, &image).unwrap()).collect();
        let err = merge(&parts, 0x1_0000_0000).unwrap_err();
        assert!(err.contains("256 sections"), "{err}");
    }
}
