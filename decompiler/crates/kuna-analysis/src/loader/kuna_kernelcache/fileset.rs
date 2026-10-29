//! `MH_FILESET` kernel collections: list the `LC_FILESET_ENTRY` images and cut
//! one out as a standalone thin Mach-O.

use kuna_decomp::kuna_kernelcache::KernelcacheChoice;

use crate::loader::format::macho::resolve_chained_fixups;

const MH_MAGIC_64: u32 = 0xfeed_facf;
const MH_EXECUTE: u32 = 0x2;
const MH_FILESET: u32 = 0xc;
const LC_SEGMENT_64: u32 = 0x19;
const LC_FILESET_ENTRY: u32 = 0x8000_0035;
const LC_SYMTAB: u32 = 0x2;
const LC_DYSYMTAB: u32 = 0xb;
const HEADER_LEN: usize = 32;
const NLIST_LEN: usize = 16;
const N_EXT: u8 = 0x01;
const N_ABS: u8 = 0x02;
const S_ATTR_SOME_INSTRUCTIONS: u32 = 0x400;

/// One image of a kernel collection.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FilesetEntry {
    pub id: String,
    pub vmaddr: u64,
    pub fileoff: u64,
    /// The entry header's `filetype` (`MH_EXECUTE` for the kernel,
    /// `MH_KEXT_BUNDLE` for a kext).
    pub filetype: u32,
}

pub(super) fn is_fileset(bytes: &[u8]) -> bool {
    rd_u32(bytes, 0) == Some(MH_MAGIC_64) && rd_u32(bytes, 12) == Some(MH_FILESET)
}

struct Segment {
    vmaddr: u64,
    fileoff: u64,
    filesize: u64,
}

/// `(cmd, offset)` of each load command of the 64-bit header at `at`.
fn load_commands(bytes: &[u8], at: usize) -> Option<Vec<(u32, usize)>> {
    if rd_u32(bytes, at)? != MH_MAGIC_64 {
        return None;
    }
    let ncmds = rd_u32(bytes, at + 16)? as usize;
    let end = at + HEADER_LEN + rd_u32(bytes, at + 20)? as usize;
    let mut out = Vec::with_capacity(ncmds.min(4096));
    let mut p = at + HEADER_LEN;
    for _ in 0..ncmds {
        let cmd = rd_u32(bytes, p)?;
        let size = rd_u32(bytes, p + 4)? as usize;
        if size < 8 || p + size > end {
            return None;
        }
        out.push((cmd, p));
        p += size;
    }
    Some(out)
}

fn segments(bytes: &[u8], at: usize) -> Option<Vec<Segment>> {
    load_commands(bytes, at)?
        .into_iter()
        .filter(|&(cmd, _)| cmd == LC_SEGMENT_64)
        .map(|(_, p)| {
            Some(Segment {
                vmaddr: rd_u64(bytes, p + 24)?,
                fileoff: rd_u64(bytes, p + 40)?,
                filesize: rd_u64(bytes, p + 48)?,
            })
        })
        .collect()
}

pub(super) fn entries(bytes: &[u8]) -> Option<Vec<FilesetEntry>> {
    if !is_fileset(bytes) {
        return None;
    }
    load_commands(bytes, 0)?
        .into_iter()
        .filter(|&(cmd, _)| cmd == LC_FILESET_ENTRY)
        .map(|(_, p)| {
            let size = rd_u32(bytes, p + 4)? as usize;
            let name_off = rd_u32(bytes, p + 24)? as usize;
            let name = bytes.get(p + name_off..p + size)?;
            let name = name.split(|&b| b == 0).next()?;
            let fileoff = rd_u64(bytes, p + 16)?;
            Some(FilesetEntry {
                id: String::from_utf8_lossy(name).into_owned(),
                vmaddr: rd_u64(bytes, p + 8)?,
                fileoff,
                filetype: rd_u32(bytes, usize::try_from(fileoff).ok()?.checked_add(12)?)?,
            })
        })
        .collect()
}

fn select<'a>(all: &'a [FilesetEntry], choice: &KernelcacheChoice) -> Result<&'a FilesetEntry, String> {
    let name = match choice {
        KernelcacheChoice::Entry(name) => name,
        _ => {
            return all
                .iter()
                .find(|e| e.filetype == MH_EXECUTE)
                .or_else(|| all.first())
                .ok_or_else(|| "MH_FILESET has no LC_FILESET_ENTRY".to_string());
        }
    };
    if let Some(e) = all.iter().find(|e| e.id == *name) {
        return Ok(e);
    }
    let lower = name.to_ascii_lowercase();
    let suffix = format!(".{lower}");
    let hits: Vec<&FilesetEntry> = all
        .iter()
        .filter(|e| {
            let id = e.id.to_ascii_lowercase();
            id == lower || id.ends_with(&suffix)
        })
        .collect();
    if let [one] = hits[..] {
        return Ok(one);
    }
    let near: Vec<&str> = if hits.is_empty() {
        all.iter().filter(|e| e.id.to_ascii_lowercase().contains(&lower)).map(|e| e.id.as_str()).collect()
    } else {
        hits.iter().map(|e| e.id.as_str()).collect()
    };
    let shown = if near.is_empty() { all.iter().map(|e| e.id.as_str()).take(8).collect() } else { near };
    Err(format!(
        "no single MH_FILESET entry matches {name:?} ({} entries; {}: {})",
        all.len(),
        if hits.len() > 1 { "ambiguous between" } else { "for example" },
        shown.join(", ")
    ))
}

/// Cut the chosen entry out of a kernel collection: rebase every chained-fixup
/// pointer slot, then copy the entry's header and load commands to offset 0.
pub(super) fn extract(mut bytes: Vec<u8>, choice: &KernelcacheChoice) -> Result<(Vec<u8>, String), String> {
    let all = entries(&bytes).ok_or("malformed MH_FILESET load commands")?;
    let entry = select(&all, choice)?.clone();
    let index = all.iter().position(|e| e.fileoff == entry.fileoff).unwrap_or(0);
    let applied = apply_fixups(&mut bytes)?;

    let at = usize::try_from(entry.fileoff).map_err(|_| "fileset entry offset out of range")?;
    let sizeofcmds = rd_u32(&bytes, at + 20).ok_or("truncated fileset entry header")? as usize;
    let header_len = HEADER_LEN + sizeofcmds;
    let segs = segments(&bytes, at).ok_or_else(|| format!("malformed load commands in {}", entry.id))?;
    if segs.iter().any(|s| s.filesize > 0 && s.fileoff < header_len as u64) {
        return Err(format!("cannot cut {} out of its collection: its data starts inside the header", entry.id));
    }
    let bound = if entry.filetype == MH_EXECUTE { 0 } else { bind_to_collection(&mut bytes, &all, index) };
    bytes.copy_within(at..at + header_len, 0);
    let bound = if bound > 0 { format!(", {bound} undefined symbols bound to the collection") } else { String::new() };
    let note = format!(
        "MH_FILESET: loaded {} (entry {} of {}, {applied} chained-fixup pointers rebased{bound}); \
         choose another entry with --option kernelcache <bundle-id>",
        entry.id,
        index + 1,
        all.len()
    );
    Ok((bytes, note))
}

/// An entry's `LC_SYMTAB` (`symoff`, `nsyms`) and `LC_DYSYMTAB` external-defined
/// and undefined index ranges.
struct Symbols {
    symoff: usize,
    nsyms: usize,
    stroff: usize,
    extdef: std::ops::Range<usize>,
    undef: std::ops::Range<usize>,
}

fn symbols(bytes: &[u8], at: usize) -> Option<Symbols> {
    let cmds = load_commands(bytes, at)?;
    let (_, st) = *cmds.iter().find(|&&(cmd, _)| cmd == LC_SYMTAB)?;
    let (_, dy) = *cmds.iter().find(|&&(cmd, _)| cmd == LC_DYSYMTAB)?;
    let range = |i: usize| -> Option<std::ops::Range<usize>> {
        let first = rd_u32(bytes, dy + i)? as usize;
        Some(first..first + rd_u32(bytes, dy + i + 4)? as usize)
    };
    Some(Symbols {
        symoff: rd_u32(bytes, st + 8)? as usize,
        nsyms: rd_u32(bytes, st + 12)? as usize,
        stroff: rd_u32(bytes, st + 16)? as usize,
        extdef: range(16)?,
        undef: range(24)?,
    })
}

fn nlist_name<'a>(bytes: &'a [u8], syms: &Symbols, index: usize) -> Option<&'a [u8]> {
    let strx = rd_u32(bytes, syms.symoff + index * NLIST_LEN)? as usize;
    let tail = bytes.get(syms.stroff.checked_add(strx)?..)?;
    tail.split(|&b| b == 0).next().filter(|n| !n.is_empty())
}

/// Bind a kext's undefined symbols the collection resolved at build time: a kext
/// calls into the kernel and its dependencies with a direct `bl`, and the only
/// record of the callee's name is the kext's undefined `nlist`. Each one that
/// another entry exports as code is rewritten to `N_ABS | N_EXT` at that address
/// (the kernel's definition wins over a kext's), which the Mach-O import walk
/// reads as an external function. Returns how many were bound.
fn bind_to_collection(bytes: &mut [u8], all: &[FilesetEntry], index: usize) -> usize {
    let Some(own) = symbols(bytes, all[index].fileoff as usize) else { return 0 };
    let mut wanted: std::collections::HashMap<Vec<u8>, Option<(u64, bool)>> = own
        .undef
        .clone()
        .filter(|&i| i < own.nsyms)
        .filter_map(|i| nlist_name(bytes, &own, i).map(|n| (n.to_vec(), None)))
        .collect();
    for (i, entry) in all.iter().enumerate() {
        if i == index {
            continue;
        }
        let at = entry.fileoff as usize;
        let (Some(syms), Some(code)) = (symbols(bytes, at), code_ranges(bytes, at)) else { continue };
        let is_kernel = entry.filetype == MH_EXECUTE;
        for s in syms.extdef.clone().filter(|&s| s < syms.nsyms) {
            let Some(name) = nlist_name(bytes, &syms, s) else { continue };
            let Some(slot) = wanted.get_mut(name) else { continue };
            let Some(addr) = rd_u64(bytes, syms.symoff + s * NLIST_LEN + 8) else { continue };
            if !code.iter().any(|r| r.contains(&addr)) {
                continue;
            }
            match slot {
                Some((_, true)) => {}
                Some(_) if !is_kernel => *slot = Some((0, false)),
                _ => *slot = Some((addr, is_kernel)),
            }
        }
    }
    let mut bound = 0;
    for i in own.undef.clone().filter(|&i| i < own.nsyms) {
        let Some(name) = nlist_name(bytes, &own, i) else { continue };
        let Some(&Some((addr, _))) = wanted.get(name) else { continue };
        if addr == 0 {
            continue;
        }
        let at = own.symoff + i * NLIST_LEN;
        bytes[at + 4] = N_ABS | N_EXT;
        bytes[at + 5] = 0;
        bytes[at + 8..at + 16].copy_from_slice(&addr.to_le_bytes());
        bound += 1;
    }
    bound
}

/// The address ranges of an entry's instruction-bearing sections.
fn code_ranges(bytes: &[u8], at: usize) -> Option<Vec<std::ops::Range<u64>>> {
    let mut out = Vec::new();
    for (cmd, p) in load_commands(bytes, at)? {
        if cmd != LC_SEGMENT_64 {
            continue;
        }
        for k in 0..rd_u32(bytes, p + 64)? as usize {
            let s = p + 72 + k * 80;
            if rd_u32(bytes, s + 64)? & S_ATTR_SOME_INSTRUCTIONS != 0 {
                let addr = rd_u64(bytes, s + 32)?;
                out.push(addr..addr + rd_u64(bytes, s + 40)?);
            }
        }
    }
    Some(out)
}

/// Write every rebase of the collection's `LC_DYLD_CHAINED_FIXUPS` into its slot,
/// with the pointer-authentication bits stripped.
fn apply_fixups(bytes: &mut [u8]) -> Result<usize, String> {
    let segs = segments(bytes, 0).ok_or("malformed MH_FILESET segments")?;
    let rebases: Vec<(u64, u64)> = {
        let file = object::File::parse(&*bytes).map_err(|e| format!("MH_FILESET: {e}"))?;
        resolve_chained_fixups(&file, bytes).rebases().collect()
    };
    let mut applied = 0;
    for (va, target) in rebases {
        let Some(seg) = segs.iter().find(|s| va >= s.vmaddr && va - s.vmaddr + 8 <= s.filesize) else {
            continue;
        };
        let off = (seg.fileoff + (va - seg.vmaddr)) as usize;
        if let Some(slot) = bytes.get_mut(off..off + 8) {
            slot.copy_from_slice(&target.to_le_bytes());
            applied += 1;
        }
    }
    Ok(applied)
}

fn rd_u32(b: &[u8], off: usize) -> Option<u32> {
    b.get(off..off.checked_add(4)?).map(|s| u32::from_le_bytes(s.try_into().unwrap()))
}

fn rd_u64(b: &[u8], off: usize) -> Option<u64> {
    b.get(off..off.checked_add(8)?).map(|s| u64::from_le_bytes(s.try_into().unwrap()))
}
