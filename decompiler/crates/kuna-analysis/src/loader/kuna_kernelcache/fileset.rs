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
const SEGMENT_LEN: usize = 72;
const SECTION_LEN: usize = 80;
const N_EXT: u8 = 0x01;
const N_ABS: u8 = 0x02;
const S_ATTR_SOME_INSTRUCTIONS: u32 = 0x400;

/// One image of a kernel collection.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct FilesetEntry {
    pub(super) id: String,
    pub(super) fileoff: u64,
    /// `MH_EXECUTE` for the kernel, `MH_KEXT_BUNDLE` for a kext.
    pub(super) filetype: u32,
}

pub(super) fn is_fileset(bytes: &[u8]) -> bool {
    rd_u32(bytes, 0) == Some(MH_MAGIC_64) && rd_u32(bytes, 12) == Some(MH_FILESET)
}

/// Which entry of a kernel collection was loaded, and the addresses every
/// entry's segments map: what a surface reading the loaded entry needs to say
/// that an address belongs to another entry rather than to nothing.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FilesetMap {
    pub loaded: String,
    entries: Vec<(String, Vec<std::ops::Range<u64>>)>,
}

impl FilesetMap {
    /// The entry whose segments map `addr`, the loaded entry first (every entry
    /// maps the one shared `__LINKEDIT`).
    pub fn owner(&self, addr: u64) -> Option<&str> {
        let loaded = self.entries.iter().filter(|(id, _)| *id == self.loaded);
        loaded
            .chain(self.entries.iter())
            .find(|(_, ranges)| ranges.iter().any(|r| r.contains(&addr)))
            .map(|(id, _)| id.as_str())
    }
}

/// The [`FilesetMap`] of a collection with `all[index]` loaded. Each distinct
/// entry header is read once, the loaded one first, within the [`bind_work`]
/// budget.
fn fileset_map(bytes: &[u8], all: &[FilesetEntry], index: usize) -> FilesetMap {
    let mut work = bind_work(bytes);
    let mut seen = std::collections::HashSet::new();
    let mut entries = Vec::new();
    for entry in std::iter::once(&all[index]).chain(all) {
        if work == 0 {
            break;
        }
        if !seen.insert(entry.fileoff) {
            continue;
        }
        let Some(cmds) = load_commands(bytes, entry.fileoff as usize) else { continue };
        work = work.saturating_sub(cmds.len());
        let ranges = cmds
            .iter()
            .filter(|&&(cmd, _)| cmd == LC_SEGMENT_64)
            .filter_map(|&(_, p)| {
                let vmaddr = rd_u64(bytes, p + 24)?;
                Some(vmaddr..vmaddr.saturating_add(rd_u64(bytes, p + 32)?))
            })
            .filter(|r| !r.is_empty())
            .collect();
        entries.push((entry.id.clone(), ranges));
    }
    FilesetMap { loaded: all[index].id.clone(), entries }
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
    let end = at.checked_add(HEADER_LEN)?.checked_add(rd_u32(bytes, at + 20)? as usize)?;
    let mut out = Vec::with_capacity(ncmds.min(4096));
    let mut p = at + HEADER_LEN;
    for _ in 0..ncmds {
        let cmd = rd_u32(bytes, p)?;
        let size = rd_u32(bytes, p + 4)? as usize;
        if size < 8 || p.checked_add(size)? > end {
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
            let name = bytes.get(p.checked_add(name_off)?..p.checked_add(size)?)?;
            let name = name.split(|&b| b == 0).next()?;
            let fileoff = rd_u64(bytes, p + 16)?;
            Some(FilesetEntry {
                id: String::from_utf8_lossy(name).into_owned(),
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
    if name == "list" {
        let ids: Vec<&str> = all.iter().map(|e| e.id.as_str()).collect();
        return Err(format!("MH_FILESET entries ({}):\n{}", all.len(), ids.join("\n")));
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
        "no single MH_FILESET entry matches {name:?} ({} entries; {}: {}; \
         `--option kernelcache list` prints them all)",
        all.len(),
        if hits.len() > 1 { "ambiguous between" } else { "for example" },
        shown.join(", ")
    ))
}

/// Cut the chosen entry out of a kernel collection: rebase every chained-fixup
/// pointer slot, then copy the entry's header and load commands to offset 0.
pub(super) fn extract(
    mut bytes: Vec<u8>,
    choice: &KernelcacheChoice,
) -> Result<(Vec<u8>, String, FilesetMap), String> {
    let all = entries(&bytes).ok_or("malformed MH_FILESET load commands")?;
    let entry = select(&all, choice)?.clone();
    let index = all.iter().position(|e| e.fileoff == entry.fileoff).unwrap_or(0);
    let map = fileset_map(&bytes, &all, index);
    let applied = apply_fixups(&mut bytes)?;

    let at = usize::try_from(entry.fileoff).map_err(|_| "fileset entry offset out of range")?;
    let sizeofcmds = rd_u32(&bytes, at + 20).ok_or("truncated fileset entry header")? as usize;
    let header_len = HEADER_LEN.checked_add(sizeofcmds).ok_or("fileset entry load commands overflow")?;
    if at.checked_add(header_len).is_none_or(|end| end > bytes.len()) {
        return Err(format!("the header of {} runs past the end of the collection", entry.id));
    }
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
    Ok((bytes, note, map))
}

/// An entry's `LC_SYMTAB` and its `LC_DYSYMTAB` external-defined and undefined
/// index ranges, clamped to the symbols the file actually holds.
struct Symbols {
    symoff: usize,
    stroff: usize,
    extdef: std::ops::Range<usize>,
    undef: std::ops::Range<usize>,
}

fn symbols(bytes: &[u8], cmds: &[(u32, usize)]) -> Option<Symbols> {
    let (_, st) = *cmds.iter().find(|&&(cmd, _)| cmd == LC_SYMTAB)?;
    let (_, dy) = *cmds.iter().find(|&&(cmd, _)| cmd == LC_DYSYMTAB)?;
    let symoff = rd_u32(bytes, st + 8)? as usize;
    let nsyms = (rd_u32(bytes, st + 12)? as usize).min(bytes.len().saturating_sub(symoff) / NLIST_LEN);
    let range = |i: usize| -> Option<std::ops::Range<usize>> {
        let first = (rd_u32(bytes, dy + i)? as usize).min(nsyms);
        Some(first..first.saturating_add(rd_u32(bytes, dy + i + 4)? as usize).min(nsyms))
    };
    Some(Symbols {
        symoff,
        stroff: rd_u32(bytes, st + 16)? as usize,
        extdef: range(16)?,
        undef: range(24)?,
    })
}

fn nlist_name<'a>(bytes: &'a [u8], syms: &Symbols, index: usize, work: &mut usize) -> Option<&'a [u8]> {
    let strx = rd_u32(bytes, syms.symoff.checked_add(index.checked_mul(NLIST_LEN)?)?)? as usize;
    let tail = bytes.get(syms.stroff.checked_add(strx)?..)?;
    let window = &tail[..tail.len().min(*work)];
    let Some(len) = window.iter().position(|&b| b == 0) else {
        *work = work.saturating_sub(window.len());
        return None;
    };
    *work = work.saturating_sub(len + 1);
    (len > 0).then(|| &tail[..len])
}

/// The work [`bind_to_collection`] may spend, in commands, sections and symbol
/// name bytes visited: an honest collection costs a fraction of its size, while
/// entry headers sharing load commands or a name table would multiply it.
fn bind_work(bytes: &[u8]) -> usize {
    bytes.len().saturating_mul(4).max(1 << 20)
}

/// Bind a kext's undefined symbols the collection resolved at build time: a kext
/// calls into the kernel and its dependencies with a direct `bl`, and the only
/// record of the callee's name is the kext's undefined `nlist`. Each one that
/// another entry exports as code is rewritten to `N_ABS | N_EXT` at that address
/// (the kernel's definition wins over a kext's), which the Mach-O import walk
/// reads as an external function. Each distinct entry header is read once, the
/// names copied never exceed the file, and collecting and writing each stop at
/// the [`bind_work`] budget. Returns how many were bound.
fn bind_to_collection(bytes: &mut [u8], all: &[FilesetEntry], index: usize) -> usize {
    let mut work = bind_work(bytes);
    let Some(own) = load_commands(bytes, all[index].fileoff as usize).and_then(|c| symbols(bytes, &c)) else {
        return 0;
    };
    let mut wanted: std::collections::HashMap<Vec<u8>, Option<(u64, bool)>> = std::collections::HashMap::new();
    let mut allowance = bytes.len();
    for i in own.undef.clone() {
        let Some(name) = nlist_name(bytes, &own, i, &mut work) else { continue };
        if wanted.contains_key(name) || name.len() > allowance {
            continue;
        }
        allowance -= name.len();
        wanted.insert(name.to_vec(), None);
    }
    let mut seen = std::collections::HashSet::from([all[index].fileoff]);
    for entry in all {
        if work == 0 {
            break;
        }
        if !seen.insert(entry.fileoff) {
            continue;
        }
        let at = entry.fileoff as usize;
        let Some(cmds) = load_commands(bytes, at) else { continue };
        work = work.saturating_sub(cmds.len());
        let (Some(syms), Some(code)) = (symbols(bytes, &cmds), code_ranges(bytes, &cmds, &mut work)) else {
            continue;
        };
        let is_kernel = entry.filetype == MH_EXECUTE;
        for s in syms.extdef.clone() {
            let Some(name) = nlist_name(bytes, &syms, s, &mut work) else { continue };
            let Some(slot) = wanted.get_mut(name) else { continue };
            let Some(addr) = rd_u64(bytes, syms.symoff + s * NLIST_LEN + 8) else { continue };
            work = work.saturating_sub(code.len());
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
    let mut work = bind_work(bytes);
    for i in own.undef.clone() {
        let Some(name) = nlist_name(bytes, &own, i, &mut work) else { continue };
        let Some(&Some((addr, _))) = wanted.get(name) else { continue };
        if addr == 0 {
            continue;
        }
        let at = own.symoff + i * NLIST_LEN;
        let Some(nlist) = bytes.get_mut(at..at + NLIST_LEN) else { continue };
        nlist[4] = N_ABS | N_EXT;
        nlist[5] = 0;
        nlist[8..].copy_from_slice(&addr.to_le_bytes());
        bound += 1;
    }
    bound
}

/// The address ranges of an entry's instruction-bearing sections. A segment's
/// sections are read only as far as its own `cmdsize` holds them.
fn code_ranges(bytes: &[u8], cmds: &[(u32, usize)], work: &mut usize) -> Option<Vec<std::ops::Range<u64>>> {
    let mut out = Vec::new();
    for &(cmd, p) in cmds {
        if cmd != LC_SEGMENT_64 {
            continue;
        }
        let room = (rd_u32(bytes, p + 4)? as usize).saturating_sub(SEGMENT_LEN) / SECTION_LEN;
        let nsects = (rd_u32(bytes, p + 64)? as usize).min(room);
        for k in 0..nsects {
            if *work == 0 {
                return Some(out);
            }
            *work -= 1;
            let s = p + SEGMENT_LEN + k * SECTION_LEN;
            if rd_u32(bytes, s + 64)? & S_ATTR_SOME_INSTRUCTIONS != 0 {
                let addr = rd_u64(bytes, s + 32)?;
                out.push(addr..addr.saturating_add(rd_u64(bytes, s + 40)?));
            }
        }
    }
    Some(out)
}

/// Write every rebase of the collection's `LC_DYLD_CHAINED_FIXUPS` into its slot,
/// with the pointer-authentication bits stripped.
fn apply_fixups(bytes: &mut [u8]) -> Result<usize, String> {
    let mut segs = segments(bytes, 0).ok_or("malformed MH_FILESET segments")?;
    segs.retain(|s| s.filesize > 0);
    segs.sort_by_key(|s| s.vmaddr);
    let rebases: Vec<(u64, u64)> = {
        let file = object::File::parse(&*bytes).map_err(|e| format!("MH_FILESET: {e}"))?;
        resolve_chained_fixups(&file, bytes).rebases().collect()
    };
    let mut applied = 0;
    for (va, target) in rebases {
        let Some(seg) = segs[..segs.partition_point(|s| s.vmaddr <= va)].last() else { continue };
        if va - seg.vmaddr >= seg.filesize.saturating_sub(7) {
            continue;
        }
        let Some(off) = seg.fileoff.checked_add(va - seg.vmaddr).and_then(|o| usize::try_from(o).ok()) else {
            continue;
        };
        if let Some(slot) = bytes.get_mut(off..off.saturating_add(8)) {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture() -> Vec<u8> {
        std::fs::read(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/kernelcache/fileset.macho")).unwrap()
    }

    fn mach_header(filetype: u32, ncmds: u32, cmds: &[u8]) -> Vec<u8> {
        let mut out = Vec::new();
        for v in [MH_MAGIC_64, 0x0100_000c, 0, filetype, ncmds, cmds.len() as u32, 0, 0] {
            out.extend_from_slice(&v.to_le_bytes());
        }
        out.extend_from_slice(cmds);
        out
    }

    fn le_words(words: &[u32]) -> Vec<u8> {
        words.iter().flat_map(|w| w.to_le_bytes()).collect()
    }

    /// Entries aliasing one header are read against it once: ten thousand
    /// entries naming one 50,000-command header cost one parse, not 10,000.
    #[test]
    fn aliased_entries_are_read_once() {
        let mut image = fixture();
        let all = entries(&image).unwrap();
        let kext = all.iter().position(|e| e.filetype != MH_EXECUTE).unwrap();
        let expected = bind_to_collection(&mut image.clone(), &all, kext);
        let ncmds = 50_000u32;
        let at = image.len().next_multiple_of(8);
        image.resize(at, 0);
        for v in [MH_MAGIC_64, 0x0100_000c, 0, MH_EXECUTE, ncmds, ncmds * 8, 0, 0] {
            image.extend_from_slice(&v.to_le_bytes());
        }
        for _ in 0..ncmds {
            image.extend_from_slice(&0x7fff_ffffu32.to_le_bytes());
            image.extend_from_slice(&8u32.to_le_bytes());
        }
        let alias = FilesetEntry { id: "alias".into(), fileoff: at as u64, filetype: MH_EXECUTE };
        let mut aliased = all.clone();
        aliased.extend(std::iter::repeat_n(alias, 10_000));
        let start = std::time::Instant::now();
        let bound = bind_to_collection(&mut image, &aliased, kext);
        let elapsed = start.elapsed();
        assert_eq!(bound, expected);
        assert!(elapsed < std::time::Duration::from_millis(500), "10k aliases took {elapsed:?}");
    }

    /// Distinct entry headers that skip into one shared tail of load commands
    /// stop at the work budget instead of re-reading the tail per header, and
    /// the real entries before them still bind.
    #[test]
    fn headers_sharing_commands_stop_at_the_work_budget() {
        let mut image = fixture();
        let all = entries(&image).unwrap();
        let kext = all.iter().position(|e| e.filetype != MH_EXECUTE).unwrap();
        let expected = bind_to_collection(&mut image.clone(), &all, kext);
        let (headers, tail_cmds) = (2_000usize, 40_000usize);
        let first_header = image.len().next_multiple_of(8);
        image.resize(first_header, 0);
        let tail_at = first_header + headers * 40;
        let mut aliased = all.clone();
        for i in 0..headers {
            let at = first_header + i * 40;
            let skip = (tail_at - (at + 32)) as u32;
            let cmds = (tail_cmds as u32 + 1, skip + tail_cmds as u32 * 8);
            let header = mach_header(MH_EXECUTE, cmds.0, &[]);
            image.extend_from_slice(&header[..20]);
            image.extend_from_slice(&cmds.1.to_le_bytes());
            image.extend_from_slice(&header[24..]);
            image.extend_from_slice(&le_words(&[0x7fff_ffff, skip]));
            aliased.push(FilesetEntry { id: format!("h{i}"), fileoff: at as u64, filetype: MH_EXECUTE });
        }
        for _ in 0..tail_cmds {
            image.extend_from_slice(&le_words(&[0x7fff_ffff, 8]));
        }
        let start = std::time::Instant::now();
        let bound = bind_to_collection(&mut image, &aliased, kext);
        let elapsed = start.elapsed();
        assert_eq!(bound, expected);
        assert!(elapsed < std::time::Duration::from_secs(5), "shared-tail headers took {elapsed:?}");
    }

    /// A segment's sections are read only as far as its own `cmdsize` holds
    /// them, so one 72-byte command claiming a million sections adds none.
    #[test]
    fn sections_stop_at_the_segment_command() {
        let mut image = vec![0u8; 0x1000];
        let at = 0x100;
        let mut seg = le_words(&[LC_SEGMENT_64, SEGMENT_LEN as u32]);
        seg.resize(SEGMENT_LEN, 0);
        seg[64..68].copy_from_slice(&1_000_000u32.to_le_bytes());
        let header = mach_header(MH_EXECUTE, 1, &seg);
        image[at..at + header.len()].copy_from_slice(&header);
        for off in (at + 32 + SEGMENT_LEN..image.len()).step_by(4) {
            image[off..off + 4].copy_from_slice(&S_ATTR_SOME_INSTRUCTIONS.to_le_bytes());
        }
        let cmds = load_commands(&image, at).unwrap();
        let mut work = 1 << 20;
        assert_eq!(code_ranges(&image, &cmds, &mut work), Some(Vec::new()));
        assert_eq!(work, 1 << 20, "no section was visited");
    }

    /// Undefined symbols that all read one long unterminated name copy it at
    /// most once per byte of the file; the loop ends promptly without a blow-up.
    #[test]
    fn undefined_names_copy_at_most_the_file() {
        let count = 30_000u32;
        let name_len = 1usize << 20;
        let symoff = 0x4000usize;
        let stroff = symoff + count as usize * NLIST_LEN;
        let mut cmds = le_words(&[LC_SYMTAB, 24, symoff as u32, count, stroff as u32, name_len as u32 + 2]);
        let mut dysym = le_words(&[LC_DYSYMTAB, 80, 0, 0, 0, 0, 0, count]);
        dysym.resize(80, 0);
        cmds.extend_from_slice(&dysym);
        let header = mach_header(0xb, 2, &cmds);
        let mut image = vec![0u8; stroff + name_len + 2];
        image[0x1000..0x1000 + header.len()].copy_from_slice(&header);
        for i in 0..count as usize {
            let at = symoff + i * NLIST_LEN;
            image[at..at + 4].copy_from_slice(&(1 + i as u32).to_le_bytes());
        }
        image[stroff + 1..stroff + 1 + name_len].fill(b'A');
        let all = [FilesetEntry { id: "own".into(), fileoff: 0x1000, filetype: 0xb }];
        let start = std::time::Instant::now();
        assert_eq!(bind_to_collection(&mut image, &all, 0), 0);
        let elapsed = start.elapsed();
        assert!(elapsed < std::time::Duration::from_secs(5), "undefined-name bomb took {elapsed:?}");
    }
}
