//! Pre-fileset (prelinked) kernelcaches: one `MH_EXECUTE` kernel that carries
//! its kexts. Up to iOS 11 each kext is a whole Mach-O inside `__PRELINK_TEXT`,
//! its segments spread over the `__PLK_*` / `__PRELINK_DATA` ranges and found
//! through its `_PrelinkExecutableLoadAddr`; from iOS 12 the kexts are merged
//! into the kernel's own segments and `__PRELINK_INFO,__kmod_start` points at
//! each kext's header, which then describes only its code. Pointers are
//! threaded (`__TEXT,__thread_starts`) or chained (`__TEXT,__chain_starts`)
//! and are rebased into the bytes before anything reads them.

use super::prelink::KextInfo;
use super::{
    load_commands, rd_u32, rd_u64, FilesetEntry, HEADER_LEN, LC_DYSYMTAB, LC_SEGMENT_64, LC_SYMTAB, MH_EXECUTE, NLIST_LEN,
};
use crate::loader::format::macho::chained::chain_starts_rebases;

const MH_MAGIC_64: u32 = 0xfeed_facf;
const MH_KEXT_BUNDLE: u32 = 0xb;
const SEGMENT_LEN: usize = 72;
const SECTION_LEN: usize = 80;
const LC_UNIXTHREAD: u32 = 0x5;
const ZEROFILL_TYPES: [u32; 3] = [0x1, 0xc, 0x12];
const CODE_FLAGS: u32 = 0x8000_0400;
const N_TYPE: u8 = 0x0e;
const N_SECT: u8 = 0x0e;
const N_ABS: u8 = 0x02;
const N_STAB: u8 = 0xe0;
const N_EXT: u8 = 0x01;
const S_NON_LAZY_SYMBOL_POINTERS: u32 = 0x6;
const S_LAZY_SYMBOL_POINTERS: u32 = 0x7;
const S_SYMBOL_STUBS: u32 = 0x8;
const INDIRECT_SYMBOL_ABS: u32 = 0x4000_0000;
const KMOD_NAME: u64 = 0x10;
const KMOD_NAME_LEN: usize = 64;
const LINKEDIT_DATA: [u32; 8] = [0x1d, 0x1e, 0x26, 0x29, 0x2b, 0x2e, 0x33, 0x34];
const PRELINK_SEGMENTS: [&[u8]; 6] =
    [b"__PRELINK_TEXT", b"__PLK_TEXT_EXEC", b"__PRELINK_DATA", b"__PLK_DATA_CONST", b"__PLK_LLVM_COV", b"__PLK_LINKEDIT"];

#[derive(Clone)]
struct Seg {
    cmd: usize,
    name: Vec<u8>,
    vmaddr: u64,
    fileoff: u64,
    filesize: u64,
    code: bool,
}

fn name16(raw: &[u8]) -> Vec<u8> {
    raw[..raw.iter().position(|&b| b == 0).unwrap_or(raw.len())].to_vec()
}

fn segs(bytes: &[u8], at: usize) -> Vec<Seg> {
    let mut out = Vec::new();
    for (cmd, p) in load_commands(bytes, at).unwrap_or_default() {
        if cmd != LC_SEGMENT_64 {
            continue;
        }
        let (Some(vmaddr), Some(fileoff), Some(filesize), Some(nsects)) =
            (rd_u64(bytes, p + 24), rd_u64(bytes, p + 40), rd_u64(bytes, p + 48), rd_u32(bytes, p + 64))
        else {
            continue;
        };
        let code = (0..nsects as usize)
            .any(|k| rd_u32(bytes, p + SEGMENT_LEN + k * SECTION_LEN + 64).is_some_and(|f| f & CODE_FLAGS != 0));
        let name = name16(&bytes[p + 8..p + 24]);
        let code = code || name == b"__TEXT_EXEC";
        out.push(Seg { cmd: p, name, vmaddr, fileoff, filesize, code });
    }
    out
}

fn va_to_off(segs: &[Seg], va: u64) -> Option<usize> {
    let s = segs.iter().find(|s| s.filesize > 0 && va >= s.vmaddr && va - s.vmaddr < s.filesize)?;
    usize::try_from(s.fileoff.checked_add(va - s.vmaddr)?).ok()
}

/// `(addr, size, file offset)` of section `seg,sect` of the header at `at`.
fn section(bytes: &[u8], at: usize, seg: &[u8], sect: &[u8]) -> Option<(u64, u64, usize)> {
    for s in segs(bytes, at) {
        for k in 0..rd_u32(bytes, s.cmd + 64)? as usize {
            let p = s.cmd + SEGMENT_LEN + k * SECTION_LEN;
            if name16(&bytes[p..p + 16]) == sect && name16(&bytes[p + 16..p + 32]) == seg {
                return Some((rd_u64(bytes, p + 32)?, rd_u64(bytes, p + 40)?, rd_u32(bytes, p + 48)? as usize));
            }
        }
    }
    None
}

/// Whether `bytes` is a 64-bit `MH_EXECUTE` with threaded or chained pointers,
/// or with a `__PRELINK_INFO` segment.
pub(super) fn is_prelinked(bytes: &[u8]) -> bool {
    rd_u32(bytes, 0) == Some(MH_MAGIC_64)
        && rd_u32(bytes, 12) == Some(MH_EXECUTE)
        && (section(bytes, 0, b"__TEXT", b"__thread_starts").is_some()
            || section(bytes, 0, b"__TEXT", b"__chain_starts").is_some()
            || segs(bytes, 0).iter().any(|s| s.name == b"__PRELINK_INFO"))
}

/// The image base: the address of the segment holding the header.
fn image_base(segs: &[Seg]) -> Option<u64> {
    segs.iter().find(|s| s.fileoff == 0 && s.filesize > 0).map(|s| s.vmaddr)
}

/// Write the rebase target of every threaded (`__thread_starts`) or chained
/// (`__chain_starts`) pointer of the image at offset 0 into its slot. Returns
/// how many slots were written.
pub(super) fn rebase(bytes: &mut [u8]) -> usize {
    let segs = segs(bytes, 0);
    let Some(base) = image_base(&segs) else { return 0 };
    let mut written = 0;
    if let Some((_, size, off)) = section(bytes, 0, b"__TEXT", b"__thread_starts") {
        written += thread_starts(bytes, &segs, base, off, size as usize);
    }
    if let Some((_, size, off)) = section(bytes, 0, b"__TEXT", b"__chain_starts") {
        let Some(blob) = bytes.get(off..off.saturating_add(size as usize)).map(<[u8]>::to_vec) else { return written };
        let table: Vec<(u64, u64, u64)> = segs.iter().map(|s| (s.vmaddr, s.fileoff, s.filesize)).collect();
        for (va, target, width) in chain_starts_rebases(bytes, &table, base, &blob) {
            let Some(at) = va_to_off(&segs, va) else { continue };
            if let Some(slot) = bytes.get_mut(at..at + width) {
                slot.copy_from_slice(&target.to_le_bytes()[..width]);
                written += 1;
            }
        }
    }
    written
}

/// XNU's `rebase_threaded_starts`: the first word's bit 0 picks an 8- or 4-byte
/// hop unit, each further word (up to `0xffffffff`) is a chain's offset from
/// the image base. A chain word with bit 62 clear is a rebase: authenticated
/// (bit 63) it holds a 32-bit offset from the base, else a 43-bit
/// sign-extended address with its top byte in bits 43..50. Bits 51..61 hop to
/// the next word.
fn thread_starts(bytes: &mut [u8], segs: &[Seg], base: u64, off: usize, size: usize) -> usize {
    let Some(header) = rd_u32(bytes, off) else { return 0 };
    let unit = if header & 1 == 1 { 8 } else { 4 };
    let mut written = 0;
    let mut budget = bytes.len();
    for i in 1..size / 4 {
        let Some(start) = rd_u32(bytes, off + 4 * i) else { break };
        if start == u32::MAX {
            break;
        }
        let mut va = base.wrapping_add(start as u64);
        for _ in 0..bytes.len() / unit as usize {
            if budget == 0 {
                return written;
            }
            budget -= 1;
            let Some(at) = va_to_off(segs, va) else { break };
            let Some(v) = rd_u64(bytes, at) else { break };
            if v & (1 << 62) == 0 {
                let target = if v & (1 << 63) != 0 {
                    base.wrapping_add(v & 0xffff_ffff)
                } else {
                    let top8 = v & 0x0007_f800_0000_0000;
                    let low43 = ((((v & 0x7ff_ffff_ffff) << 21) as i64) >> 21) as u64;
                    (top8 << 13) | (low43 & 0x00ff_ffff_ffff_ffff)
                };
                bytes[at..at + 8].copy_from_slice(&target.to_le_bytes());
                written += 1;
            }
            let hop = ((v >> 51) & 0x7ff) * unit;
            if hop == 0 {
                break;
            }
            va = va.wrapping_add(hop);
        }
    }
    written
}

/// The kernel (`com.apple.kernel`, the header at offset 0) and every kext
/// whose header the cache holds, each kext's segment and section offsets made
/// absolute in place. `merged` says whether the kexts live in the kernel's own
/// segments (iOS 12-14) rather than in `__PRELINK_TEXT`.
pub(super) fn entries(bytes: &mut [u8], info: &[KextInfo]) -> (Vec<FilesetEntry>, bool) {
    let outer = segs(bytes, 0);
    let mut out = vec![FilesetEntry { id: "com.apple.kernel".into(), fileoff: 0, filetype: MH_EXECUTE }];
    let kmod_start = section(bytes, 0, b"__PRELINK_INFO", b"__kmod_start").filter(|s| s.1 >= 8);
    let merged = kmod_start.is_some();
    let mut headers: Vec<(String, usize)> = Vec::new();
    if let Some((_, size, off)) = kmod_start {
        let kmod_info = section(bytes, 0, b"__PRELINK_INFO", b"__kmod_info");
        for i in 0..(size as usize / 8).min(bytes.len() / 8) {
            let Some(at) = rd_u64(bytes, off + 8 * i).and_then(|va| va_to_off(&outer, va)) else { continue };
            let id = info
                .iter()
                .find(|k| k.module_index == Some(i as u64))
                .map(|k| k.id.clone())
                .or_else(|| kmod_name(bytes, &outer, kmod_info, i))
                .unwrap_or_else(|| format!("prelink_mod_{i}"));
            headers.push((id, at));
        }
    } else {
        for k in info {
            if let Some(at) = k.load_addr.and_then(|va| va_to_off(&outer, va)) {
                headers.push((k.id.clone(), at));
            }
        }
    }
    for (id, at) in headers {
        if rd_u32(bytes, at) != Some(MH_MAGIC_64) || rd_u32(bytes, at + 12) != Some(MH_KEXT_BUNDLE) {
            continue;
        }
        if !segs(bytes, at).iter().any(|s| s.code) || !absolutize(bytes, &outer, at) {
            continue;
        }
        if !out.iter().any(|e| e.id == id) {
            out.push(FilesetEntry { id, fileoff: at as u64, filetype: MH_KEXT_BUNDLE });
        }
    }
    (out, merged)
}

/// The `name` of the `i`th `kmod_info_t` `__kmod_info` points at.
fn kmod_name(bytes: &[u8], outer: &[Seg], kmod_info: Option<(u64, u64, usize)>, i: usize) -> Option<String> {
    let (_, size, off) = kmod_info?;
    if (i as u64 + 1) * 8 > size {
        return None;
    }
    let at = va_to_off(outer, rd_u64(bytes, off + 8 * i)?.checked_add(KMOD_NAME)?)?;
    let raw = bytes.get(at..at + KMOD_NAME_LEN)?;
    let name = String::from_utf8(name16(raw)).ok()?;
    (!name.is_empty()).then_some(name)
}

/// Rewrite a kext header's file offsets, which are relative to the kext and not
/// always consistent, to the absolute offsets of its addresses in the cache. A
/// `__LINKEDIT`-relative table moves with the kext's `__LINKEDIT`, or is
/// dropped when that has no bytes in the cache.
fn absolutize(bytes: &mut [u8], outer: &[Seg], at: usize) -> bool {
    let own = segs(bytes, at);
    let linkedit = own.iter().find(|s| s.name == b"__LINKEDIT").and_then(|s| {
        let to = va_to_off(outer, s.vmaddr).filter(|_| s.filesize > 0)?;
        Some((s.fileoff, to as u64))
    });
    for s in &own {
        let to = if s.filesize == 0 { Some(0) } else { va_to_off(outer, s.vmaddr).map(|o| o as u64) };
        let Some(to) = to else { return false };
        bytes[s.cmd + 40..s.cmd + 48].copy_from_slice(&to.to_le_bytes());
        for k in 0..rd_u32(bytes, s.cmd + 64).unwrap_or(0) as usize {
            let p = s.cmd + SEGMENT_LEN + k * SECTION_LEN;
            let (Some(addr), Some(flags)) = (rd_u64(bytes, p + 32), rd_u32(bytes, p + 64)) else { return false };
            let off = if ZEROFILL_TYPES.contains(&(flags & 0xff)) || rd_u64(bytes, p + 40) == Some(0) {
                0
            } else {
                match va_to_off(outer, addr).and_then(|o| u32::try_from(o).ok()) {
                    Some(o) => o,
                    None => return false,
                }
            };
            bytes[p + 48..p + 52].copy_from_slice(&off.to_le_bytes());
            if s.name == b"__TEXT_EXEC" && flags == 0 {
                bytes[p + 64..p + 68].copy_from_slice(&CODE_FLAGS.to_le_bytes());
            }
        }
    }
    let rel = |v: u32| -> u32 {
        match linkedit {
            Some((from, to)) if v as u64 >= from => u32::try_from(to + (v as u64 - from)).unwrap_or(0),
            _ => 0,
        }
    };
    for (cmd, p) in load_commands(bytes, at).unwrap_or_default() {
        if cmd == LC_SYMTAB {
            let (symoff, stroff) = (rd_u32(bytes, p + 8).unwrap_or(0), rd_u32(bytes, p + 16).unwrap_or(0));
            let (symoff, stroff) = (rel(symoff), rel(stroff));
            if symoff == 0 || stroff == 0 {
                bytes[p + 8..p + 24].fill(0);
            } else {
                bytes[p + 8..p + 12].copy_from_slice(&symoff.to_le_bytes());
                bytes[p + 16..p + 20].copy_from_slice(&stroff.to_le_bytes());
            }
        } else if LINKEDIT_DATA.contains(&cmd) {
            let dataoff = rel(rd_u32(bytes, p + 8).unwrap_or(0));
            if dataoff == 0 {
                bytes[p + 8..p + 16].fill(0);
            } else {
                bytes[p + 8..p + 12].copy_from_slice(&dataoff.to_le_bytes());
            }
        }
    }
    true
}

/// Remove the segment commands of the header at `at` that `drop` selects, and
/// move its symbols to the surviving section ordinals: a symbol whose section
/// went away becomes absolute (`N_ABS`) at its address.
fn drop_segments(bytes: &mut [u8], at: usize, drop_thread: bool, drop: impl Fn(&Seg) -> bool) -> Result<(), String> {
    let Some(cmds) = load_commands(bytes, at) else { return Err("malformed load commands".into()) };
    let all = segs(bytes, at);
    let mut kept_cmds: Vec<u8> = Vec::new();
    let mut ncmds = 0u32;
    let mut ordinals: Vec<u8> = vec![0];
    let mut next = 1usize;
    for &(cmd, p) in &cmds {
        let size = rd_u32(bytes, p + 4).unwrap_or(0) as usize;
        if cmd == LC_UNIXTHREAD && drop_thread {
            continue;
        }
        if cmd == LC_SEGMENT_64 {
            let Some(s) = all.iter().find(|s| s.cmd == p) else { continue };
            let nsects = rd_u32(bytes, p + 64).unwrap_or(0) as usize;
            if drop(s) {
                ordinals.extend(std::iter::repeat_n(0, nsects));
                continue;
            }
            for _ in 0..nsects {
                ordinals.push(u8::try_from(next).unwrap_or(0));
                next += 1;
            }
        }
        kept_cmds.extend_from_slice(&bytes[p..p + size]);
        ncmds += 1;
    }
    let old_len = rd_u32(bytes, at + 20).unwrap_or(0) as usize;
    let start = at + HEADER_LEN;
    let region_end = start.checked_add(old_len).filter(|&e| e <= bytes.len()).ok_or("load commands run past the file")?;
    bytes[start..start + kept_cmds.len()].copy_from_slice(&kept_cmds);
    bytes[start + kept_cmds.len()..region_end].fill(0);
    bytes[at + 16..at + 20].copy_from_slice(&ncmds.to_le_bytes());
    bytes[at + 20..at + 24].copy_from_slice(&(kept_cmds.len() as u32).to_le_bytes());

    let Some(&(_, st)) = load_commands(bytes, at).unwrap_or_default().iter().find(|&&(c, _)| c == LC_SYMTAB) else {
        return Ok(());
    };
    let symoff = rd_u32(bytes, st + 8).unwrap_or(0) as usize;
    let nsyms = (rd_u32(bytes, st + 12).unwrap_or(0) as usize).min(bytes.len().saturating_sub(symoff) / NLIST_LEN);
    for i in 0..nsyms {
        let n = symoff + i * NLIST_LEN;
        let (ty, sect) = (bytes[n + 4], bytes[n + 5]);
        if ty & N_STAB != 0 || ty & N_TYPE != N_SECT {
            continue;
        }
        match ordinals.get(sect as usize).copied().unwrap_or(0) {
            0 => {
                bytes[n + 4] = (ty & !N_TYPE) | N_ABS;
                bytes[n + 5] = 0;
            }
            o => bytes[n + 5] = o,
        }
    }
    Ok(())
}

/// The image for `chosen` (indexes into `all`, the kernel at 0). The kernel of
/// a merged cache already holds every kext, and kexts alone keep the kernel's
/// data segments, which hold their own data. A kext of an older cache gets
/// the kernel symbols it reaches bound to its stubs and `__got` slots, as a
/// fileset kext's undefined symbols are.
pub(super) fn cut(mut bytes: Vec<u8>, all: &[FilesetEntry], merged: bool, chosen: &[usize]) -> Result<Vec<u8>, String> {
    let prelink = |s: &Seg| PRELINK_SEGMENTS.contains(&s.name.as_slice());
    if merged {
        if chosen.contains(&0) {
            return Ok(bytes);
        }
        drop_segments(&mut bytes, 0, true, |s| prelink(s) || s.code || s.filesize == 0 && s.name != b"__LINKEDIT")?;
        let mut entries = all.to_vec();
        entries[0].filetype = MH_KEXT_BUNDLE;
        let with_kernel: Vec<usize> = std::iter::once(0).chain(chosen.iter().copied()).collect();
        return super::merge::merge(bytes, &entries, &with_kernel);
    }
    let kernel = KernelSymbols::read(&bytes);
    for &i in chosen.iter().filter(|&&i| i != 0) {
        bind_to_kernel(&mut bytes, all[i].fileoff as usize, &kernel);
    }
    if chosen.contains(&0) {
        drop_segments(&mut bytes, 0, false, prelink)?;
        if chosen.len() == 1 {
            return Ok(bytes);
        }
    }
    super::merge::merge(bytes, all, chosen)
}

/// The kernel's defined symbols by address, as `nlist_64` records whose
/// names index the kernel's string table.
struct KernelSymbols {
    by_addr: std::collections::BTreeMap<u64, [u8; NLIST_LEN]>,
    stroff: u32,
    strsize: u32,
}

impl KernelSymbols {
    fn read(bytes: &[u8]) -> Self {
        let mut out = KernelSymbols { by_addr: Default::default(), stroff: 0, strsize: 0 };
        let cmds = load_commands(bytes, 0).unwrap_or_default();
        let Some(&(_, st)) = cmds.iter().find(|&&(c, _)| c == LC_SYMTAB) else { return out };
        let symoff = rd_u32(bytes, st + 8).unwrap_or(0) as usize;
        let nsyms = (rd_u32(bytes, st + 12).unwrap_or(0) as usize).min(bytes.len().saturating_sub(symoff) / NLIST_LEN);
        out.stroff = rd_u32(bytes, st + 16).unwrap_or(0);
        out.strsize = rd_u32(bytes, st + 20).unwrap_or(0);
        for i in 0..nsyms {
            let n: [u8; NLIST_LEN] = bytes[symoff + i * NLIST_LEN..symoff + (i + 1) * NLIST_LEN].try_into().unwrap();
            if n[4] & N_STAB != 0 || n[4] & N_TYPE != N_SECT {
                continue;
            }
            let addr = u64::from_le_bytes(n[8..16].try_into().unwrap());
            let slot = out.by_addr.entry(addr).or_insert(n);
            if slot[4] & N_EXT == 0 && n[4] & N_EXT != 0 {
                *slot = n;
            }
        }
        out
    }
}

/// Give the stripped kext at `at` a symbol table of the kernel symbols its
/// `__got` slots, stubs and `BL`s reach, each absolute at the kernel address
/// (in the undefined range, where a fileset kext's bound imports sit), and an
/// indirect-symbol table naming its pointer and stub sections. The tables go
/// at the end of `bytes`; the names stay in the kernel's string table.
fn bind_to_kernel(bytes: &mut Vec<u8>, at: usize, kernel: &KernelSymbols) {
    if kernel.by_addr.is_empty() {
        return;
    }
    let cmds = load_commands(bytes, at).unwrap_or_default();
    let (Some(&(_, st)), Some(&(_, dy))) =
        (cmds.iter().find(|&&(c, _)| c == LC_SYMTAB), cmds.iter().find(|&&(c, _)| c == LC_DYSYMTAB))
    else {
        return;
    };
    if rd_u32(bytes, st + 12) != Some(0) {
        return;
    }
    let own = segs(bytes, at);
    let read_ptr = |va: u64| va_to_off(&own, va).and_then(|o| rd_u64(bytes, o));
    let mut index: std::collections::BTreeMap<u64, u32> = Default::default();
    let mut nlists: Vec<u8> = Vec::new();
    let mut symbol = |addr: u64, nlists: &mut Vec<u8>| -> Option<u32> {
        if let Some(&i) = index.get(&addr) {
            return Some(i);
        }
        let mut n = *kernel.by_addr.get(&addr)?;
        n[4] = N_ABS | N_EXT;
        n[5] = 0;
        let i = (nlists.len() / NLIST_LEN) as u32;
        nlists.extend_from_slice(&n);
        index.insert(addr, i);
        Some(i)
    };
    let mut indirect: Vec<u32> = Vec::new();
    let mut reserved1: Vec<(usize, u32)> = Vec::new();
    let mut retype: Vec<(usize, u32, u32)> = Vec::new();
    for s in &own {
        for k in 0..rd_u32(bytes, s.cmd + 64).unwrap_or(0) as usize {
            let p = s.cmd + SEGMENT_LEN + k * SECTION_LEN;
            let (Some(addr), Some(size), Some(flags)) = (rd_u64(bytes, p + 32), rd_u64(bytes, p + 40), rd_u32(bytes, p + 64))
            else {
                continue;
            };
            if size > bytes.len() as u64 {
                continue;
            }
            let name = name16(&bytes[p..p + 16]);
            let kind = match (flags & 0xff, name.as_slice()) {
                (0, b"__got" | b"__auth_got" | b"__nl_symbol_ptr") => S_NON_LAZY_SYMBOL_POINTERS,
                (0, b"__la_symbol_ptr") => S_LAZY_SYMBOL_POINTERS,
                (0, b"__stubs" | b"__auth_stubs") => S_SYMBOL_STUBS,
                (t, _) => t,
            };
            let slots: Vec<Option<u64>> = match kind {
                S_NON_LAZY_SYMBOL_POINTERS | S_LAZY_SYMBOL_POINTERS => {
                    retype.push((p, kind, 0));
                    (0..size / 8).map(|j| addr.checked_add(8 * j).and_then(read_ptr)).collect()
                }
                S_SYMBOL_STUBS => {
                    let stub = match rd_u32(bytes, p + 72).unwrap_or(0) {
                        0 if name == b"__auth_stubs" => 16,
                        0 => 12,
                        n => n,
                    };
                    retype.push((p, kind | (flags & !0xff), stub));
                    (0..size / stub as u64)
                        .map(|j| addr.checked_add(stub as u64 * j).and_then(|va| stub_slot(bytes, &own, va)).and_then(read_ptr))
                        .collect()
                }
                _ => {
                    if s.code {
                        for w in (0..size / 4).filter_map(|j| {
                            let va = addr.checked_add(4 * j)?;
                            va_to_off(&own, va).and_then(|o| rd_u32(bytes, o)).map(|ins| (j, ins))
                        }) {
                            let (j, ins) = w;
                            if ins & 0xfc00_0000 == 0x9400_0000 {
                                let delta = ((((ins & 0x03ff_ffff) << 6) as i32) >> 4) as i64;
                                symbol(addr.wrapping_add(4 * j).wrapping_add(delta as u64), &mut nlists);
                            }
                        }
                    }
                    continue;
                }
            };
            reserved1.push((p + 68, indirect.len() as u32));
            indirect.extend(slots.into_iter().map(|v| v.and_then(|v| symbol(v, &mut nlists)).unwrap_or(INDIRECT_SYMBOL_ABS)));
        }
    }
    if nlists.is_empty() {
        return;
    }
    let symoff = bytes.len().div_ceil(8) * 8;
    let (Ok(symoff32), Ok(indirectoff)) = (u32::try_from(symoff), u32::try_from(symoff + nlists.len())) else { return };
    if u32::try_from(symoff + nlists.len() + 4 * indirect.len()).is_err() {
        return;
    }
    let nsyms = (nlists.len() / NLIST_LEN) as u32;
    bytes.resize(symoff, 0);
    bytes.extend_from_slice(&nlists);
    indirect.iter().for_each(|v| bytes.extend_from_slice(&v.to_le_bytes()));
    for (field, v) in [(8, symoff32), (12, nsyms), (16, kernel.stroff), (20, kernel.strsize)] {
        bytes[st + field..st + field + 4].copy_from_slice(&v.to_le_bytes());
    }
    let dysym = [0, 0, 0, 0, 0, nsyms, 0, 0, 0, 0, 0, 0, indirectoff, indirect.len() as u32];
    for (k, v) in dysym.iter().enumerate() {
        bytes[dy + 8 + 4 * k..dy + 12 + 4 * k].copy_from_slice(&v.to_le_bytes());
    }
    for (field, v) in reserved1 {
        bytes[field..field + 4].copy_from_slice(&v.to_le_bytes());
    }
    for (p, flags, stub) in retype {
        bytes[p + 64..p + 68].copy_from_slice(&flags.to_le_bytes());
        if stub != 0 {
            bytes[p + 72..p + 76].copy_from_slice(&stub.to_le_bytes());
        }
    }
}

/// The pointer slot an arm64 stub (`ADRP x16; LDR x16, [x16, #off]; BR x16`)
/// loads.
fn stub_slot(bytes: &[u8], own: &[Seg], va: u64) -> Option<u64> {
    let adrp = rd_u32(bytes, va_to_off(own, va)?)?;
    let ldr = rd_u32(bytes, va_to_off(own, va.checked_add(4)?)?)?;
    if adrp & 0x9f00_0000 != 0x9000_0000 || ldr & 0xffc0_0000 != 0xf940_0000 {
        return None;
    }
    let imm = ((((adrp >> 29) & 3) | (((adrp >> 5) & 0x7ffff) << 2)) as u64) << 44;
    let page = (va & !0xfff).wrapping_add(((imm as i64) >> 32) as u64);
    Some(page.wrapping_add((((ldr >> 10) & 0xfff) as u64) * 8))
}

#[cfg(test)]
mod tests {
    use super::super::prelink;
    use super::*;

    fn fixture(name: &str) -> Vec<u8> {
        std::fs::read(format!("{}/tests/fixtures/kernelcache/{name}", env!("CARGO_MANIFEST_DIR"))).unwrap()
    }

    const BASE: u64 = 0xFFFF_FFF0_0700_4000;

    #[test]
    fn threaded_pointers_are_rebased() {
        let mut bytes = fixture("prelinked_merged.macho");
        assert_eq!(rebase(&mut bytes), 10);
        assert_eq!(rd_u64(&bytes, 0x4000), Some(BASE + 0x2000));
        assert_eq!(rd_u64(&bytes, 0x4008), Some(BASE + 0x2900));
        assert_eq!(rd_u64(&bytes, 0x4010), Some(BASE + 0x1000));
        assert_eq!(rd_u64(&bytes, 0x5008), Some(BASE + 0x2800));
    }

    #[test]
    fn chain_starts_are_rebased() {
        let mut bytes = fixture("chainstarts.macho");
        assert_eq!(rebase(&mut bytes), 2);
        assert_eq!(rd_u64(&bytes, 0x3000), Some(BASE + 0x2020));
        assert_eq!(rd_u64(&bytes, 0x3008), Some(BASE + 0x2020));
    }

    #[test]
    fn merged_kext_is_found_through_kmod_start() {
        let mut bytes = fixture("prelinked_merged.macho");
        rebase(&mut bytes);
        let info = prelink::kexts(&bytes).unwrap();
        assert_eq!(info[0].module_index, Some(0));
        let (all, merged) = entries(&mut bytes, &info);
        assert!(merged);
        assert_eq!(all[1].id, "com.example.driver.Widget");
        assert_eq!(all[1].fileoff, 0x2800);
        let image = cut(bytes, &all, merged, &[1]).unwrap();
        let names: Vec<Vec<u8>> = segs(&image, 0).into_iter().map(|s| s.name).collect();
        assert!(names.contains(&b"__DATA_CONST".to_vec()) && names.contains(&b"__TEXT_EXEC".to_vec()));
        assert!(!names.contains(&b"__KLD".to_vec()));
        let exec = segs(&image, 0).into_iter().find(|s| s.name == b"__TEXT_EXEC").unwrap();
        assert_eq!(exec.vmaddr, BASE + 0x2800);
    }

    #[test]
    fn split_kext_is_cut_out_with_kernel_imports() {
        let mut bytes = fixture("prelinked_split.macho");
        let info = prelink::kexts(&bytes).unwrap();
        assert_eq!(info.len(), 2);
        let (all, merged) = entries(&mut bytes, &info);
        assert!(!merged);
        assert_eq!(all.len(), 2);
        let image = cut(bytes, &all, merged, &[1]).unwrap();
        assert_eq!(rd_u32(&image, 12), Some(MH_KEXT_BUNDLE));
        let file = object::File::parse(image.as_slice()).unwrap();
        use object::read::{Object, ObjectSection, ObjectSymbol};
        let got = file.section_by_name("__got").unwrap();
        assert_eq!(got.address(), BASE + 0xA000);
        let imports = crate::loader::macho_stubs::resolve_macho_imports(&file, &image);
        assert!(imports.iter().any(|i| i.addr == BASE + 0x9100 && i.name == b"kernel_add"));
        let sym = file.symbols().find(|s| s.name() == Ok("_kernel_add")).unwrap();
        assert_eq!(sym.address(), BASE + 0x2000);
    }

    /// A thousand starts that each walk the same hundred-slot chain: the walk
    /// is budgeted by the file, so the rebase writes stay within it.
    #[test]
    fn thread_start_walks_share_one_budget() {
        let n = 1000;
        let mut bytes = vec![0u8; 0x4000];
        let segs = [Seg { cmd: 0, name: b"__TEXT".to_vec(), vmaddr: 0, fileoff: 0, filesize: 0x4000, code: false }];
        for i in 0..99 {
            bytes[0x200 + 8 * i..0x208 + 8 * i].copy_from_slice(&(1u64 << 51).to_le_bytes());
        }
        bytes[0x100..0x104].copy_from_slice(&1u32.to_le_bytes());
        for i in 1..=n {
            bytes[0x100 + 4 * i..0x104 + 4 * i].copy_from_slice(&0x200u32.to_le_bytes());
        }
        let written = thread_starts(&mut bytes, &segs, 0, 0x100, 4 * (n + 1));
        assert!(written <= bytes.len(), "{written} writes: the walks were not budgeted");
    }

    #[test]
    fn drop_segments_refuses_load_commands_past_the_file() {
        let mut bytes = vec![0u8; 0x40];
        bytes[0..4].copy_from_slice(&MH_MAGIC_64.to_le_bytes());
        bytes[16..20].copy_from_slice(&1u32.to_le_bytes());
        bytes[20..24].copy_from_slice(&0x1000u32.to_le_bytes());
        bytes[32..36].copy_from_slice(&0x1bu32.to_le_bytes());
        bytes[36..40].copy_from_slice(&8u32.to_le_bytes());
        assert!(drop_segments(&mut bytes, 0, false, |_| false).is_err());
    }

    /// A `__got` section whose declared size exceeds the file: its slots are
    /// skipped, not collected into a vector sized from the header.
    #[test]
    fn bind_to_kernel_skips_a_section_larger_than_the_file() {
        let mut bytes = vec![0u8; 288];
        bytes[0..4].copy_from_slice(&MH_MAGIC_64.to_le_bytes());
        bytes[12..16].copy_from_slice(&MH_KEXT_BUNDLE.to_le_bytes());
        bytes[16..20].copy_from_slice(&3u32.to_le_bytes());
        bytes[20..24].copy_from_slice(&256u32.to_le_bytes());
        let seg = 32;
        bytes[seg..seg + 4].copy_from_slice(&0x19u32.to_le_bytes());
        bytes[seg + 4..seg + 8].copy_from_slice(&152u32.to_le_bytes());
        bytes[seg + 8..seg + 14].copy_from_slice(b"__DATA");
        bytes[seg + 24..seg + 32].copy_from_slice(&0x1000u64.to_le_bytes());
        bytes[seg + 32..seg + 40].copy_from_slice(&0x100u64.to_le_bytes());
        bytes[seg + 48..seg + 56].copy_from_slice(&0x100u64.to_le_bytes());
        bytes[seg + 64..seg + 68].copy_from_slice(&1u32.to_le_bytes());
        let sect = seg + 72;
        bytes[sect..sect + 5].copy_from_slice(b"__got");
        bytes[sect + 16..sect + 22].copy_from_slice(b"__DATA");
        bytes[sect + 32..sect + 40].copy_from_slice(&0x1000u64.to_le_bytes());
        bytes[sect + 40..sect + 48].copy_from_slice(&0xFFFF_FFFF_FFFF_FFF8u64.to_le_bytes());
        bytes[184..188].copy_from_slice(&0x2u32.to_le_bytes());
        bytes[188..192].copy_from_slice(&24u32.to_le_bytes());
        bytes[208..212].copy_from_slice(&0xbu32.to_le_bytes());
        bytes[212..216].copy_from_slice(&80u32.to_le_bytes());
        let kernel = KernelSymbols {
            by_addr: std::collections::BTreeMap::from([(0x2000u64, [0u8; NLIST_LEN])]),
            stroff: 0,
            strsize: 0,
        };
        bind_to_kernel(&mut bytes, 0, &kernel);
        assert_eq!(bytes.len(), 288);
    }
}
