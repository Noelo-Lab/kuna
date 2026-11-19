//! Several entries of a kernel collection in one thin Mach-O.
//!
//! The chosen entries' segment commands are concatenated (each keeps its
//! collection address), the collection's `__BRANCH_STUBS` / `__BRANCH_GOTS`
//! segments are added, and one symbol table, indirect-symbol table and
//! function-starts blob are rebuilt behind the collection: every entry's
//! symbols renumbered into one local / external / undefined order, their
//! section ordinals moved to the merged section list, and each stub or
//! pointer section's `reserved1` moved to its entry's part of the merged
//! indirect table. The rebuilt header goes in front of the collection, so every
//! file offset moves by the header's page-rounded size.

use super::{load_commands, rd_u32, rd_u64, FilesetEntry, HEADER_LEN, LC_SEGMENT_64, LC_SYMTAB, NLIST_LEN};

const LC_DYSYMTAB: u32 = 0xb;
const LC_UNIXTHREAD: u32 = 0x5;
const LC_UUID: u32 = 0x1b;
const LC_BUILD_VERSION: u32 = 0x32;
const LC_SOURCE_VERSION: u32 = 0x2a;
const LC_FUNCTION_STARTS: u32 = 0x26;
const MH_EXECUTE: u32 = 0x2;
const MH_KEXT_BUNDLE: u32 = 0xb;
const PAGE: usize = 0x4000;
const INDIRECT_SYMBOL_LOCAL: u32 = 0x8000_0000;
const INDIRECT_SYMBOL_ABS: u32 = 0x4000_0000;
const SEGMENT_LEN: usize = 72;
const SECTION_LEN: usize = 80;
const BRANCH_SEGMENTS: [&[u8]; 2] = [b"__BRANCH_STUBS", b"__BRANCH_GOTS"];

fn seg_name(bytes: &[u8], cmd: usize) -> &[u8] {
    let raw = &bytes[cmd + 8..cmd + 24];
    &raw[..raw.iter().position(|&b| b == 0).unwrap_or(16)]
}

/// The collection's `__BRANCH_STUBS` / `__BRANCH_GOTS` segment commands.
pub(super) fn branch_segments(bytes: &[u8]) -> Vec<usize> {
    load_commands(bytes, 0)
        .unwrap_or_default()
        .into_iter()
        .filter(|&(cmd, p)| cmd == LC_SEGMENT_64 && BRANCH_SEGMENTS.contains(&seg_name(bytes, p)))
        .map(|(_, p)| p)
        .collect()
}

struct EntrySymbols {
    symoff: usize,
    nsyms: usize,
    stroff: u32,
    strsize: u32,
    local: std::ops::Range<usize>,
    extdef: std::ops::Range<usize>,
    undef: std::ops::Range<usize>,
    indirect: Vec<u32>,
}

fn entry_symbols(bytes: &[u8], cmds: &[(u32, usize)]) -> Option<EntrySymbols> {
    let &(_, st) = cmds.iter().find(|&&(c, _)| c == LC_SYMTAB)?;
    let symoff = rd_u32(bytes, st + 8)? as usize;
    let nsyms = (rd_u32(bytes, st + 12)? as usize).min(bytes.len().saturating_sub(symoff) / NLIST_LEN);
    let mut out = EntrySymbols {
        symoff,
        nsyms,
        stroff: rd_u32(bytes, st + 16)?,
        strsize: rd_u32(bytes, st + 20)?,
        local: 0..nsyms,
        extdef: nsyms..nsyms,
        undef: nsyms..nsyms,
        indirect: Vec::new(),
    };
    if let Some(&(_, dy)) = cmds.iter().find(|&&(c, _)| c == LC_DYSYMTAB) {
        let range = |i: usize| -> Option<std::ops::Range<usize>> {
            let first = (rd_u32(bytes, dy + i)? as usize).min(nsyms);
            Some(first..first.saturating_add(rd_u32(bytes, dy + i + 4)? as usize).min(nsyms))
        };
        out.local = range(8)?;
        out.extdef = range(16)?;
        out.undef = range(24)?;
        let off = rd_u32(bytes, dy + 56)? as usize;
        let n = (rd_u32(bytes, dy + 60)? as usize).min(bytes.len().saturating_sub(off) / 4);
        out.indirect = (0..n).filter_map(|k| rd_u32(bytes, off + 4 * k)).collect();
    }
    Some(out)
}

fn function_starts(bytes: &[u8], cmds: &[(u32, usize)]) -> Vec<u64> {
    let Some(&(_, fs)) = cmds.iter().find(|&&(c, _)| c == LC_FUNCTION_STARTS) else { return Vec::new() };
    let Some(base) = cmds
        .iter()
        .filter(|&&(c, p)| c == LC_SEGMENT_64 && seg_name(bytes, p) == b"__TEXT")
        .find_map(|&(_, p)| rd_u64(bytes, p + 24))
    else {
        return Vec::new();
    };
    let (Some(off), Some(size)) = (rd_u32(bytes, fs + 8), rd_u32(bytes, fs + 12)) else { return Vec::new() };
    let Some(data) = bytes.get(off as usize..off as usize + size as usize) else { return Vec::new() };
    let mut out = Vec::new();
    let (mut addr, mut i) = (base, 0);
    while i < data.len() {
        let (mut delta, mut shift) = (0u64, 0u32);
        while i < data.len() {
            let b = data[i];
            i += 1;
            if shift < 64 {
                delta |= ((b & 0x7f) as u64) << shift;
            }
            shift = shift.saturating_add(7);
            if b & 0x80 == 0 {
                break;
            }
        }
        if delta == 0 {
            break;
        }
        addr = addr.wrapping_add(delta);
        out.push(addr);
    }
    out
}

fn uleb(mut v: u64, out: &mut Vec<u8>) {
    loop {
        let b = (v & 0x7f) as u8;
        v >>= 7;
        if v == 0 {
            out.push(b);
            return;
        }
        out.push(b | 0x80);
    }
}

/// Build one image from `chosen` (indexes into `all`).
pub(super) fn merge(bytes: Vec<u8>, all: &[FilesetEntry], chosen: &[usize]) -> Result<Vec<u8>, String> {
    let mut segs: Vec<Vec<u8>> = Vec::new();
    let mut extra: Vec<Vec<u8>> = Vec::new();
    let mut nlocal: Vec<u8> = Vec::new();
    let mut nextdef: Vec<u8> = Vec::new();
    let mut nundef: Vec<u8> = Vec::new();
    let mut indirect: Vec<Result<(usize, u32), u32>> = Vec::new();
    let mut starts: Vec<u64> = Vec::new();
    let mut strtab: Option<(u32, u32)> = None;
    let mut have_linkedit = false;
    let mut sections_seen = 0usize;
    let mut kind_ordinal: [Option<u8>; 2] = [None, None];
    let has_kernel = chosen.iter().any(|&i| all[i].filetype == MH_EXECUTE);

    for &i in chosen {
        let at = all[i].fileoff as usize;
        let cmds = load_commands(&bytes, at).ok_or_else(|| format!("malformed load commands in {}", all[i].id))?;
        let syms = entry_symbols(&bytes, &cmds);
        let mut section_map: Vec<u8> = vec![0];
        let indirect_base = indirect.len() as u32;
        for &(cmd, p) in &cmds {
            let size = rd_u32(&bytes, p + 4).unwrap_or(0) as usize;
            let raw = bytes.get(p..p + size).ok_or("truncated load command")?.to_vec();
            match cmd {
                LC_SEGMENT_64 => {
                    if seg_name(&bytes, p) == b"__LINKEDIT" {
                        if have_linkedit {
                            continue;
                        }
                        have_linkedit = true;
                    }
                    let mut seg = raw;
                    let nsects = rd_u32(&seg, 64).unwrap_or(0) as usize;
                    for k in 0..nsects {
                        let s = SEGMENT_LEN + k * SECTION_LEN;
                        let Some(flags) = rd_u32(&seg, s + 64) else { break };
                        if matches!(flags & 0xff, 0x6 | 0x7 | 0x8 | 0x10 | 0x14) {
                            let r1 = rd_u32(&seg, s + 68).unwrap_or(0).wrapping_add(indirect_base);
                            seg[s + 68..s + 72].copy_from_slice(&r1.to_le_bytes());
                        }
                        sections_seen += 1;
                        let code = flags & 0x8000_0400 != 0;
                        let ordinal = match u8::try_from(sections_seen) {
                            Ok(o) => {
                                kind_ordinal[code as usize].get_or_insert(o);
                                o
                            }
                            Err(_) => kind_ordinal[code as usize].unwrap_or(1),
                        };
                        section_map.push(ordinal);
                    }
                    segs.push(seg);
                }
                LC_UNIXTHREAD | LC_UUID | LC_BUILD_VERSION | LC_SOURCE_VERSION
                    if all[i].filetype == MH_EXECUTE || !has_kernel =>
                {
                    if !extra.iter().any(|e| rd_u32(e, 0) == Some(cmd)) {
                        extra.push(raw);
                    }
                }
                _ => {}
            }
        }
        starts.extend(function_starts(&bytes, &cmds));
        let Some(syms) = syms else { continue };
        strtab.get_or_insert((syms.stroff, syms.strsize));
        let mut index: Vec<u32> = vec![u32::MAX; syms.nsyms];
        for (range, out) in [(&syms.local, &mut nlocal), (&syms.extdef, &mut nextdef), (&syms.undef, &mut nundef)] {
            for s in range.clone() {
                index[s] = (out.len() / NLIST_LEN) as u32;
                let mut n = bytes[syms.symoff + s * NLIST_LEN..syms.symoff + (s + 1) * NLIST_LEN].to_vec();
                if n[5] != 0 {
                    n[5] = section_map.get(n[5] as usize).copied().unwrap_or(0);
                }
                out.extend_from_slice(&n);
            }
        }
        let tags = [(&syms.local, 0u32), (&syms.extdef, 1), (&syms.undef, 2)];
        for k in 0..syms.indirect.len() {
            let raw = syms.indirect[k];
            if raw & (INDIRECT_SYMBOL_LOCAL | INDIRECT_SYMBOL_ABS) != 0 {
                indirect.push(Err(raw));
                continue;
            }
            let s = raw as usize;
            let new = index.get(s).copied().filter(|&v| v != u32::MAX);
            let tag = tags.iter().find(|(r, _)| r.contains(&s)).map(|&(_, t)| t as usize);
            indirect.push(match (tag, new) {
                (Some(t), Some(v)) => Ok((t, v)),
                _ => Err(INDIRECT_SYMBOL_LOCAL),
            });
        }
    }
    for p in branch_segments(&bytes) {
        let size = rd_u32(&bytes, p + 4).unwrap_or(0) as usize;
        segs.push(bytes.get(p..p + size).ok_or("truncated branch segment")?.to_vec());
    }
    let (nl, ne, nu) = (nlocal.len() / NLIST_LEN, nextdef.len() / NLIST_LEN, nundef.len() / NLIST_LEN);
    let indirect: Vec<u32> = indirect
        .into_iter()
        .map(|v| match v {
            Ok((tag, idx)) => idx + [0, nl as u32, (nl + ne) as u32][tag],
            Err(raw) => raw,
        })
        .collect();

    let starts_base = segs.iter().rev().find(|s| seg_name(s, 0) == b"__TEXT").and_then(|s| rd_u64(s, 24));
    let ranges: Vec<(u64, u64)> = segs
        .iter()
        .filter_map(|s| Some((rd_u64(s, 24)?, rd_u64(s, 24)?.saturating_add(rd_u64(s, 32)?))))
        .collect();
    starts.retain(|&a| ranges.iter().any(|&(lo, hi)| (lo..hi).contains(&a)));
    starts.sort_unstable();
    starts.dedup();
    let mut starts_blob = Vec::new();
    if let Some(base) = starts_base.filter(|&b| starts.first().is_some_and(|&f| f > b)) {
        let mut prev = base;
        for &s in &starts {
            uleb(s - prev, &mut starts_blob);
            prev = s;
        }
        starts_blob.push(0);
    }

    let ncmds_fixed = segs.len() + extra.len() + 2 + usize::from(!starts_blob.is_empty());
    let sizeofcmds: usize = segs.iter().map(Vec::len).sum::<usize>()
        + extra.iter().map(Vec::len).sum::<usize>()
        + 24
        + 80
        + if starts_blob.is_empty() { 0 } else { 16 };
    let shift = (HEADER_LEN + sizeofcmds).div_ceil(PAGE) * PAGE;
    let shift32 = u32::try_from(shift).map_err(|_| "the merged image does not fit 32-bit file offsets")?;

    let tail_start = shift + bytes.len();
    let symoff = tail_start.div_ceil(8) * 8;
    let indirectoff = symoff + nlocal.len() + nextdef.len() + nundef.len();
    let startsoff = indirectoff + indirect.len() * 4;

    let mut out = Vec::with_capacity(startsoff + starts_blob.len());
    let first = &all[*chosen.first().ok_or("no entries to merge")?];
    let at = first.fileoff as usize;
    let filetype = if has_kernel { MH_EXECUTE } else { MH_KEXT_BUNDLE };
    out.extend_from_slice(&bytes[at..at + 12]);
    out.extend_from_slice(&filetype.to_le_bytes());
    out.extend_from_slice(&(ncmds_fixed as u32).to_le_bytes());
    out.extend_from_slice(&(sizeofcmds as u32).to_le_bytes());
    out.extend_from_slice(&rd_u32(&bytes, at + 24).unwrap_or(0).to_le_bytes());
    out.extend_from_slice(&0u32.to_le_bytes());
    for mut seg in segs {
        let fileoff = rd_u64(&seg, 40).unwrap_or(0).checked_add(shift as u64).ok_or("segment offset overflows")?;
        seg[40..48].copy_from_slice(&fileoff.to_le_bytes());
        let nsects = rd_u32(&seg, 64).unwrap_or(0) as usize;
        for k in 0..nsects {
            let s = SEGMENT_LEN + k * SECTION_LEN;
            if let Some(off) = rd_u32(&seg, s + 48).filter(|&o| o != 0) {
                let moved = off.checked_add(shift32).ok_or("section offset overflows the merged image")?;
                seg[s + 48..s + 52].copy_from_slice(&moved.to_le_bytes());
            }
        }
        out.extend_from_slice(&seg);
    }
    for e in extra {
        out.extend_from_slice(&e);
    }
    let (stroff, strsize) = strtab.unwrap_or((0, 0));
    let w = |out: &mut Vec<u8>, vals: &[u32]| vals.iter().for_each(|v| out.extend_from_slice(&v.to_le_bytes()));
    let stroff = stroff.checked_add(shift32).ok_or("string table offset overflows the merged image")?;
    w(&mut out, &[LC_SYMTAB, 24, symoff as u32, (nl + ne + nu) as u32, stroff, strsize]);
    w(
        &mut out,
        &[
            LC_DYSYMTAB, 80, 0, nl as u32, nl as u32, ne as u32, (nl + ne) as u32, nu as u32,
            0, 0, 0, 0, 0, 0, indirectoff as u32, indirect.len() as u32, 0, 0, 0, 0,
        ],
    );
    if !starts_blob.is_empty() {
        w(&mut out, &[LC_FUNCTION_STARTS, 16, startsoff as u32, starts_blob.len() as u32]);
    }
    if u32::try_from(startsoff + starts_blob.len()).is_err() {
        return Err("the merged image does not fit 32-bit file offsets".into());
    }
    out.resize(shift, 0);
    out.extend_from_slice(&bytes);
    out.resize(symoff, 0);
    out.extend_from_slice(&nlocal);
    out.extend_from_slice(&nextdef);
    out.extend_from_slice(&nundef);
    indirect.iter().for_each(|v| out.extend_from_slice(&v.to_le_bytes()));
    out.extend_from_slice(&starts_blob);
    Ok(out)
}
