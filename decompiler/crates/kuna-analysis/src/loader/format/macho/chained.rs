//! Mach-O `LC_DYLD_CHAINED_FIXUPS` resolver — the kuna analog of Ghidra's
//! `DyldChainedFixups` / the dyld `MachOAnalyzer::forEachFixupChainSegment` walk
//! (design §5 PR-O0).
//!
//! ## What it does and why
//!
//! A modern arm64(e) Mach-O does **not** store a real pointer in a `__DATA_CONST`
//! / `__DATA` pointer slot. Instead the static linker packs each slot into a
//! *chained-fixup entry* — a bit-field carrying (a) the pointer's rebase target
//! and (b) the `next` displacement to the following entry in the chain, so dyld
//! can rebase a whole page by hopping the chain rather than consulting a separate
//! relocation list. Read naively, a slot like the ObjC `class_t.isa` word reads
//! `0x0020000100008028` (the `next=2` field leaks into the high bits) instead of
//! the real `class_t*` `0x100008028`. So an ObjC walker that dereferences
//! `__objc_classlist` on such a binary reads **garbage** — this is the
//! prerequisite the design (§3.2 LOSS) flagged for arm64 ObjC.
//!
//! This module parses `LC_DYLD_CHAINED_FIXUPS`, follows every fixup chain, and
//! produces a flat **VMA → resolved-pointer** overlay so a downstream consumer
//! (the ObjC walk; any future Swift/PDB-on-Mach-O reader) can read the *real*
//! address at a slot via [`ChainedFixups::resolved_ptr`].
//!
//! ## Scope
//!
//! - **Rebase**, for every user-space pointer format: `DYLD_CHAINED_PTR_64` (2),
//!   `DYLD_CHAINED_PTR_64_OFFSET` (6, what Apple's `ld64` writes for arm64),
//!   `DYLD_CHAINED_PTR_32` (3), and `DYLD_CHAINED_PTR_ARM64E` (1) /
//!   `_ARM64E_KERNEL` (7, a kext) / `_ARM64E_USERLAND` (9) /
//!   `_ARM64E_USERLAND24` (12), plain and authenticated. Pointer authentication is a run-time concept, so an auth
//!   rebase resolves to `imagebase + target` with its PAC context stripped.
//! - **Bind**: the slot, the import it names (decoded through the
//!   `imports_offset` table and its symbol strings) and the addend. An
//!   import's address is in another image, so a bind resolves to no pointer:
//!   [`ChainedFixups::resolved_ptr`] misses there and a consumer falls back
//!   rather than reading a wrong address.
//!
//! ## Faithfulness / no-op guarantee
//!
//! Pure & total: a non-Mach-O input, a Mach-O **without** `LC_DYLD_CHAINED_FIXUPS`
//! (the classic `LC_DYLD_INFO`/`LC_DYLD_INFO_ONLY` form, whose on-disk pointers are
//! already the unslid values — every existing x86-64 `macho_objc`/`macho_imports`
//! fixture), or an unparsable header yields an **empty** overlay.

use std::collections::HashMap;
use std::sync::Arc;

use object::read::macho::{LoadCommandVariant, MachHeader, Segment};
use object::FileKind;

/// Pointer-format ids from Apple's `mach-o/fixup-chains.h` (`DYLD_CHAINED_PTR_*`).
/// Only the formats kuna resolves are named; any other format leaves its chain
/// unresolved (the inert path).
mod ptr_format {
    /// `DYLD_CHAINED_PTR_ARM64E` — 8-byte stride; a plain rebase target is a
    /// vmaddr, an authenticated one an image-base offset.
    pub const ARM64E: u16 = 1;
    /// `DYLD_CHAINED_PTR_64` — 4-byte stride; the rebase target is a vmaddr.
    pub const PTR_64: u16 = 2;
    /// `DYLD_CHAINED_PTR_32` — 4-byte stride, 32-bit slots.
    pub const PTR_32: u16 = 3;
    /// `DYLD_CHAINED_PTR_64_OFFSET` — like `PTR_64`, but the target is an
    /// image-base offset.
    pub const PTR_64_OFFSET: u16 = 6;
    /// `DYLD_CHAINED_PTR_ARM64E_KERNEL` — `ARM64E` with a 4-byte stride and
    /// image-base-offset targets (a kext).
    pub const ARM64E_KERNEL: u16 = 7;
    /// `DYLD_CHAINED_PTR_ARM64E_USERLAND` — `ARM64E` with image-base-offset
    /// targets.
    pub const ARM64E_USERLAND: u16 = 9;
    /// `DYLD_CHAINED_PTR_ARM64E_USERLAND24` — `ARM64E_USERLAND` with a 24-bit
    /// bind ordinal.
    pub const ARM64E_USERLAND24: u16 = 12;
}

/// One bind fixup: the import a slot is bound to at load time.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChainedBind {
    /// The imported symbol's name as the image spells it (`_printf`), shared
    /// by every bind of that import; empty when the ordinal is out of range,
    /// the names are compressed or the name is not terminated in the pool.
    pub name: Arc<[u8]>,
    /// The bind's addend plus its import record's.
    pub addend: i64,
    /// A weak import: dyld leaves the slot 0 when no image defines it.
    pub weak: bool,
}

/// One decoded chain entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Fixup {
    Rebase(u64),
    Bind { ordinal: u32, addend: i64 },
}

/// A resolved chained-fixup overlay: the real pointer value at each rebased VMA
/// slot, and the import each bound slot names. Built once per Mach-O load by
/// [`resolve_chained_fixups`]; consulted by the loader (which writes the values
/// into the image it maps) and the ObjC walk's `MachoImage::read_ptr`.
///
/// Empty for a Mach-O without `LC_DYLD_CHAINED_FIXUPS` — the resolver is then a
/// strict no-op.
#[derive(Default, Debug, Clone)]
pub struct ChainedFixups {
    /// `(slot_vma, resolved_pointer)` for every plain/auth **rebase** fixup,
    /// sorted by slot.
    rebases: Vec<(u64, u64)>,
    /// `(slot_vma, import)` for every **bind** fixup, sorted by slot.
    binds: Vec<(u64, ChainedBind)>,
    /// The slot width in bytes: 4 for `DYLD_CHAINED_PTR_32`, else 8.
    width: u8,
}

impl ChainedFixups {
    /// Whether this image carried any chained-fixup rebases (a quick "is this a
    /// chained-fixup binary" probe; `false` ⇒ the resolver is a no-op).
    pub fn is_empty(&self) -> bool {
        self.rebases.is_empty()
    }

    /// The resolved pointer at slot `va`, if `va` was a rebase fixup. `None` for a
    /// non-fixup slot (the consumer then reads the raw section word) or a bind.
    pub fn resolved_ptr(&self, va: u64) -> Option<u64> {
        let at = self.rebases.binary_search_by_key(&va, |&(slot, _)| slot).ok()?;
        Some(self.rebases[at].1)
    }

    /// Every resolved rebase target, in slot order.
    pub fn targets(&self) -> impl Iterator<Item = u64> + '_ {
        self.rebases.iter().map(|&(_, target)| target)
    }

    /// Every rebase as `(slot, target)`, in slot order.
    pub fn rebases(&self) -> impl Iterator<Item = (u64, u64)> + '_ {
        self.rebases.iter().copied()
    }

    /// Every bind as `(slot, import)`, in slot order.
    pub fn binds(&self) -> impl Iterator<Item = (u64, &ChainedBind)> + '_ {
        self.binds.iter().map(|(slot, bind)| (*slot, bind))
    }

    /// The import bound at slot `va`, if `va` is a bind fixup.
    pub fn bind_at(&self, va: u64) -> Option<&ChainedBind> {
        let at = self.binds.binary_search_by_key(&va, |(slot, _)| *slot).ok()?;
        Some(&self.binds[at].1)
    }

    /// Sort both lists by slot, keeping one entry per slot (a hostile header
    /// can start two chains on one word).
    fn sort_slots(&mut self) {
        self.rebases.sort_by_key(|&(slot, _)| slot);
        self.rebases.dedup_by_key(|(slot, _)| *slot);
        self.binds.sort_by_key(|(slot, _)| *slot);
        self.binds.dedup_by_key(|(slot, _)| *slot);
    }

    /// The width in bytes of every fixup slot this image carries.
    pub fn slot_width(&self) -> u8 {
        if self.width == 0 { 8 } else { self.width }
    }
}

/// A segment's `(vmaddr, fileoff, filesize)` — the address arithmetic the chained
/// walk needs (slot VMA = segment vmaddr + offset; raw word read at fileoff +
/// offset). Captured for every `LC_SEGMENT_64`/`LC_SEGMENT`.
#[derive(Clone, Copy, Debug)]
struct SegInfo {
    vmaddr: u64,
    fileoff: u64,
    filesize: u64,
}

/// Resolve the `LC_DYLD_CHAINED_FIXUPS` rebases of a Mach-O `file` from its raw
/// `bytes`, producing the [`ChainedFixups`] overlay.
///
/// Empty for any non-Mach-O / no-`LC_DYLD_CHAINED_FIXUPS` / unparsable input — the
/// no-op path that keeps every classic (non-chained) fixture byte-identical.
pub fn resolve_chained_fixups(file: &object::File, bytes: &[u8]) -> ChainedFixups {
    if file.format() != object::BinaryFormat::MachO {
        return ChainedFixups::default();
    }
    match FileKind::parse(bytes) {
        Ok(FileKind::MachO64) => {
            collect::<object::macho::MachHeader64<object::Endianness>>(bytes)
        }
        Ok(FileKind::MachO32) => {
            collect::<object::macho::MachHeader32<object::Endianness>>(bytes)
        }
        // Fat input is peeled to a thin slice before this runs (the engine
        // dispatch); a leftover fat / non-Mach-O here yields the empty no-op.
        _ => ChainedFixups::default(),
    }
}

/// Walk a typed Mach-O header's load commands for `LC_DYLD_CHAINED_FIXUPS` and the
/// segment table, then resolve every chain. Generic over the header width.
fn collect<Mach>(bytes: &[u8]) -> ChainedFixups
where
    Mach: MachHeader<Endian = object::Endianness>,
{
    let empty = ChainedFixups::default();
    let Ok(header) = Mach::parse(bytes, 0) else { return empty };
    let Ok(endian) = header.endian() else { return empty };
    let Ok(mut commands) = header.load_commands(endian, bytes, 0) else { return empty };

    // The chained-fixups blob's file range (LC_DYLD_CHAINED_FIXUPS is a
    // LinkeditDataCommand: dataoff/datasize into the file) + the segment table
    // (for VMA↔fileoff arithmetic), harvested in one load-command pass.
    let mut fixups_range: Option<(u64, u64)> = None;
    let mut segs: Vec<SegInfo> = Vec::new();
    // The lowest segment vmaddr that maps file content is the image preferred
    // base (the imagebase the OFFSET formats add). __PAGEZERO (filesize 0) is
    // skipped so it does not win as the minimum.
    let mut imagebase: u64 = u64::MAX;

    while let Ok(Some(command)) = commands.next() {
        // Segment (either width) → SegInfo + imagebase candidate.
        if let Ok(Some((segment, _))) = Mach::Segment::from_command(command) {
            let vmaddr = segment.vmaddr(endian).into();
            let fileoff = segment.fileoff(endian).into();
            let filesize = segment.filesize(endian).into();
            if filesize > 0 {
                segs.push(SegInfo { vmaddr, fileoff, filesize });
                imagebase = imagebase.min(vmaddr);
            }
            continue;
        }
        // LC_DYLD_CHAINED_FIXUPS is a LinkeditData command; match on its cmd id.
        if let Ok(variant) = command.variant() {
            if let LoadCommandVariant::LinkeditData(linkedit) = variant {
                if command.cmd() == object::macho::LC_DYLD_CHAINED_FIXUPS {
                    let off = linkedit.dataoff.get(endian) as u64;
                    let size = linkedit.datasize.get(endian) as u64;
                    fixups_range = Some((off, size));
                }
            }
        }
    }

    let Some((blob_off, blob_size)) = fixups_range else { return empty };
    if imagebase == u64::MAX {
        return empty;
    }
    let Some(blob) = bytes.get(blob_off as usize..(blob_off + blob_size) as usize) else {
        return empty;
    };

    let mut out = ChainedFixups::default();
    let mut walk = Walk::new(&segs, bytes, blob);
    walk_fixups(blob, imagebase, &mut walk, &mut out);
    out.sort_slots();
    out
}

/// The image a chain walk reads, the file words it has decoded, and the page
/// starts it may still read. The counts come from the image, so both ends are
/// bounded by its size: a chain stops at a word it already decoded (the word
/// and its `next` are the ones already followed), so no slot is decoded twice
/// however many pages or starts point into one chain; and an honest image
/// reads each page start once, one 2-byte entry of the blob, so `blob.len() /
/// 2` of them cover it, while segments sharing one start table stop there.
struct Walk<'a> {
    segs: &'a [SegInfo],
    file: &'a [u8],
    seen: Vec<u64>,
    starts: usize,
}

impl<'a> Walk<'a> {
    fn new(segs: &'a [SegInfo], file: &'a [u8], blob: &[u8]) -> Self {
        Walk { segs, file, seen: vec![0; file.len() / 256 + 1], starts: blob.len() / 2 + 1 }
    }

    /// Spend one page start; `false` once none are left.
    fn start(&mut self) -> bool {
        let left = self.starts > 0;
        self.starts = self.starts.saturating_sub(1);
        left
    }

    /// Mark the 4-byte file word at `fo` decoded; `false` if it already was.
    /// Two slots that do not overlap never begin in one word.
    fn first_visit(&mut self, fo: u64) -> bool {
        let word = fo / 4;
        let Some(bits) = usize::try_from(word / 64).ok().and_then(|i| self.seen.get_mut(i)) else {
            return false;
        };
        let bit = 1u64 << (word % 64);
        let fresh = *bits & bit == 0;
        *bits |= bit;
        fresh
    }
}

/// `dyld_chained_fixups_header` (fixup-chains.h): `fixups_version`,
/// `starts_offset`, `imports_offset`, `symbols_offset`, `imports_count`,
/// `imports_format`, `symbols_format` — seven `u32`. Walks every segment's
/// chains, then names each bind through the imports table.
fn walk_fixups(blob: &[u8], imagebase: u64, walk: &mut Walk, out: &mut ChainedFixups) {
    let Some(starts_offset) = rd_u32(blob, 4) else { return };
    let sio = starts_offset as usize;

    // dyld_chained_starts_in_image { uint32 seg_count; uint32 seg_info_offset[]; }
    let Some(seg_count) = rd_u32(blob, sio) else { return };
    let mut binds: Vec<(u64, u32, i64)> = Vec::new();
    for i in 0..seg_count as usize {
        let Some(seg_info_off) = rd_u32(blob, sio + 4 + 4 * i) else { break };
        if seg_info_off == 0 {
            continue;
        }
        walk_segment(blob, sio + seg_info_off as usize, imagebase, walk, out, &mut binds);
    }
    if binds.is_empty() {
        return;
    }
    let mut ordinals: Vec<u32> = binds.iter().map(|&(_, ordinal, _)| ordinal).collect();
    ordinals.sort_unstable();
    ordinals.dedup();
    let imports = read_imports(blob, &ordinals);
    for (slot, ordinal, addend) in binds {
        let bind = match imports.binary_search_by_key(&ordinal, |(at, _)| *at) {
            Ok(i) => {
                let import = &imports[i].1;
                ChainedBind {
                    name: Arc::clone(&import.name),
                    addend: addend.wrapping_add(import.addend),
                    weak: import.weak,
                }
            }
            Err(_) => ChainedBind { name: Arc::from(&[][..]), addend, weak: false },
        };
        out.binds.push((slot, bind));
    }
}

/// One `dyld_chained_import*` record, its name resolved.
struct Import {
    name: Arc<[u8]>,
    addend: i64,
    weak: bool,
}

/// The imports table a bind ordinal indexes: `DYLD_CHAINED_IMPORT` (1, a `u32`
/// `lib_ordinal:8 weak:1 name_offset:23`), `_ADDEND` (2, the same plus an `i32`
/// addend) or `_ADDEND64` (3, a `u64` `lib_ordinal:16 weak:1 reserved:15
/// name_offset:32` plus a `u64` addend). Names are read from the uncompressed
/// symbol pool (`symbols_format` 0); a compressed pool leaves them empty.
///
/// Only the records at `ordinals` (sorted, distinct: the ones a bind uses) are
/// read, as `(ordinal, import)`. An honest pool holds each name once, so the
/// names read never total more than the pool; the scans stop there, and a name
/// with no terminator inside that allowance is empty. The counts and offsets
/// come from the image, and a table of names that all run on to the end of the
/// pool would otherwise copy the pool once per import.
fn read_imports(blob: &[u8], ordinals: &[u32]) -> Vec<(u32, Import)> {
    let (Some(imports_offset), Some(symbols_offset), Some(count), Some(format), Some(symbols_format)) = (
        rd_u32(blob, 8),
        rd_u32(blob, 12),
        rd_u32(blob, 16),
        rd_u32(blob, 20),
        rd_u32(blob, 24),
    ) else {
        return Vec::new();
    };
    let stride = match format {
        1 => 4,
        2 => 8,
        3 => 16,
        _ => return Vec::new(),
    };
    let mut names: HashMap<u64, Arc<[u8]>> = HashMap::new();
    let mut allowance = blob.len().saturating_sub(symbols_offset as usize);
    let mut name_at = |offset: u64| -> Arc<[u8]> {
        if symbols_format != 0 {
            return Arc::from(&[][..]);
        }
        if let Some(name) = names.get(&offset) {
            return Arc::clone(name);
        }
        let start = (symbols_offset as u64).saturating_add(offset);
        let tail = usize::try_from(start).ok().and_then(|s| blob.get(s..)).unwrap_or(&[]);
        let window = &tail[..tail.len().min(allowance)];
        let name: Arc<[u8]> = match window.iter().position(|&b| b == 0) {
            Some(len) => {
                allowance -= len + 1;
                Arc::from(&window[..len])
            }
            None => {
                allowance -= window.len();
                Arc::from(&[][..])
            }
        };
        names.insert(offset, Arc::clone(&name));
        name
    };
    let mut out = Vec::new();
    for &ordinal in ordinals {
        if ordinal >= count {
            break;
        }
        let Some(at) = (ordinal as usize).checked_mul(stride).and_then(|o| o.checked_add(imports_offset as usize))
        else {
            break;
        };
        let record = match format {
            3 => rd_u64(blob, at).map(|raw| Import {
                name: name_at(raw >> 32),
                addend: rd_u64(blob, at + 8).unwrap_or(0) as i64,
                weak: (raw >> 16) & 1 != 0,
            }),
            _ => rd_u32(blob, at).map(|raw| Import {
                name: name_at(u64::from(raw >> 9)),
                addend: if format == 2 {
                    i64::from(rd_u32(blob, at + 4).unwrap_or(0) as i32)
                } else {
                    0
                },
                weak: (raw >> 8) & 1 != 0,
            }),
        };
        let Some(record) = record else { break };
        out.push((ordinal, record));
    }
    out
}

/// `dyld_chained_starts_in_segment` at `p` within `blob`: walk every page's chain.
fn walk_segment(
    blob: &[u8],
    p: usize,
    imagebase: u64,
    walk: &mut Walk,
    out: &mut ChainedFixups,
    binds: &mut Vec<(u64, u32, i64)>,
) {
    // struct dyld_chained_starts_in_segment {
    //   uint32 size; uint16 page_size; uint16 pointer_format;
    //   uint64 segment_offset; uint32 max_valid_pointer;
    //   uint16 page_count; uint16 page_start[page_count]; ... }
    let Some(page_size) = rd_u16(blob, p + 4) else { return };
    let Some(pointer_format) = rd_u16(blob, p + 6) else { return };
    let Some(segment_offset) = rd_u64(blob, p + 8) else { return };
    let Some(max_valid_pointer) = rd_u32(blob, p + 16) else { return };
    let Some(page_count) = rd_u16(blob, p + 20) else { return };
    if page_size == 0 {
        return;
    }

    let stride: u64 = match pointer_format {
        ptr_format::ARM64E | ptr_format::ARM64E_USERLAND | ptr_format::ARM64E_USERLAND24 => 8,
        ptr_format::PTR_64
        | ptr_format::PTR_64_OFFSET
        | ptr_format::PTR_32
        | ptr_format::ARM64E_KERNEL => 4,
        _ => return,
    };
    out.width = if pointer_format == ptr_format::PTR_32 { 4 } else { 8 };
    let format = Format { pointer_format, imagebase, max_valid_pointer };

    // dyld computes the chain head from the **mach_header** pointer, not the
    // per-segment base: `chain = mh + segment_offset + pageIndex*page_size +
    // page_start[pageIndex]`, so `segment_offset` is image-base-relative.
    // A 32-bit page whose chains need several starts flags its entry
    // DYLD_CHAINED_PTR_START_MULTI and lists them in the overflow entries after
    // `page_start[page_count]`, the last one flagged DYLD_CHAINED_PTR_START_LAST.
    const START_NONE: u16 = 0xFFFF;
    const START_MULTI: u16 = 0x8000;
    const START_LAST: u16 = 0x8000;
    for pi in 0..page_count as usize {
        if !walk.start() {
            return;
        }
        let Some(page_start) = rd_u16(blob, p + 22 + 2 * pi) else { continue };
        if page_start == START_NONE {
            continue;
        }
        let page = imagebase
            .wrapping_add(segment_offset)
            .wrapping_add((pi as u64) * (page_size as u64));
        if page_start & START_MULTI == 0 || pointer_format != ptr_format::PTR_32 {
            walk_chain(walk, page.wrapping_add(u64::from(page_start)), stride, &format, out, binds);
            continue;
        }
        let mut overflow = usize::from(page_start & !START_MULTI);
        while let Some(start) = rd_u16(blob, p + 22 + 2 * overflow) {
            if !walk.start() {
                return;
            }
            let first = page.wrapping_add(u64::from(start & !START_LAST));
            if !walk_chain(walk, first, stride, &format, out, binds) || start & START_LAST != 0 {
                break;
            }
            overflow += 1;
        }
    }
}

/// What [`decode_ptr`] needs to know about a segment's pointers.
struct Format {
    pointer_format: u16,
    imagebase: u64,
    max_valid_pointer: u32,
}

/// Follow one fixup chain from `first_va`, hopping `next * stride` until `next ==
/// 0`, recording each rebase and bind. `next` only moves forward, so a chain
/// cannot cycle, and it stops at a word the walk already decoded ([`Walk`]).
/// `false` when its first word is unmapped or already decoded: an honest page's
/// starts each head a chain of their own, so a 32-bit page's list of starts
/// stops there.
fn walk_chain(
    walk: &mut Walk,
    first_va: u64,
    stride: u64,
    format: &Format,
    out: &mut ChainedFixups,
    binds: &mut Vec<(u64, u32, i64)>,
) -> bool {
    let mut va = first_va;
    let mut first = true;
    loop {
        let Some(fo) = va_to_fileoff(walk.segs, va) else { return !first };
        if !walk.first_visit(fo) {
            return !first;
        }
        first = false;
        let raw = if format.pointer_format == ptr_format::PTR_32 {
            rd_u32(walk.file, fo as usize).map(u64::from)
        } else {
            rd_u64(walk.file, fo as usize)
        };
        let Some(raw) = raw else { return true };
        let (next, fixup) = decode_ptr(raw, format);
        match fixup {
            Some(Fixup::Rebase(target)) => out.rebases.push((va, target)),
            Some(Fixup::Bind { ordinal, addend }) => binds.push((va, ordinal, addend)),
            None => {}
        }
        if next == 0 {
            return true;
        }
        va = va.wrapping_add(next * stride);
    }
}

/// Sign-extend the low `bits` of `value`.
fn sign_extend(value: u64, bits: u32) -> i64 {
    let shift = 64 - bits;
    ((value << shift) as i64) >> shift
}

/// Decode one chained-pointer word: `(next, fixup)`, where `next` is the chain
/// hop in stride units and `fixup` is `None` only for an unhandled format.
fn decode_ptr(raw: u64, format: &Format) -> (u64, Option<Fixup>) {
    let imagebase = format.imagebase;
    match format.pointer_format {
        // dyld_chained_ptr_64_rebase { target:36 high8:8 reserved:7 next:12 bind:1 }
        // dyld_chained_ptr_64_bind   { ordinal:24 addend:8 reserved:19 next:12 bind:1 }
        // PTR_64's target is a vmaddr, PTR_64_OFFSET's an image-base offset.
        ptr_format::PTR_64 | ptr_format::PTR_64_OFFSET => {
            let next = (raw >> 51) & 0xFFF;
            if (raw >> 63) & 1 == 1 {
                let ordinal = (raw & 0xFF_FFFF) as u32;
                let addend = ((raw >> 24) & 0xFF) as i64;
                return (next, Some(Fixup::Bind { ordinal, addend }));
            }
            let unpacked = (raw & 0xF_FFFF_FFFF) | (((raw >> 36) & 0xFF) << 56);
            let resolved = if format.pointer_format == ptr_format::PTR_64_OFFSET {
                imagebase.wrapping_add(unpacked)
            } else {
                unpacked
            };
            (next, Some(Fixup::Rebase(resolved)))
        }
        // dyld_chained_ptr_32_rebase { target:26 next:5 bind:1 }
        // dyld_chained_ptr_32_bind   { ordinal:20 addend:6 next:5 bind:1 }
        // A target above max_valid_pointer is a non-pointer stored biased.
        ptr_format::PTR_32 => {
            let next = (raw >> 26) & 0x1F;
            if (raw >> 31) & 1 == 1 {
                let ordinal = (raw & 0xF_FFFF) as u32;
                let addend = ((raw >> 20) & 0x3F) as i64;
                return (next, Some(Fixup::Bind { ordinal, addend }));
            }
            let target = raw & 0x3FF_FFFF;
            let max = u64::from(format.max_valid_pointer);
            let resolved = if max != 0 && target > max {
                target.wrapping_sub((0x0400_0000 + max) / 2)
            } else {
                target
            };
            (next, Some(Fixup::Rebase(resolved & 0xFFFF_FFFF)))
        }
        // dyld_chained_ptr_arm64e_{rebase,bind,auth_rebase,auth_bind}: auth is
        // bit 63, bind bit 62, next bits 51..61. A plain rebase is
        // target:43 high8:8; an auth rebase target:32 (the PAC context in
        // 32..50 is stripped). A bind's ordinal is 16 bits (24 for USERLAND24);
        // a plain bind carries a signed 19-bit addend at 32..50.
        ptr_format::ARM64E
        | ptr_format::ARM64E_KERNEL
        | ptr_format::ARM64E_USERLAND
        | ptr_format::ARM64E_USERLAND24 => {
            let auth = (raw >> 63) & 1 == 1;
            let next = (raw >> 51) & 0x7FF;
            if (raw >> 62) & 1 == 1 {
                let mask = if format.pointer_format == ptr_format::ARM64E_USERLAND24 {
                    0xFF_FFFF
                } else {
                    0xFFFF
                };
                let ordinal = (raw & mask) as u32;
                let addend = if auth { 0 } else { sign_extend((raw >> 32) & 0x7_FFFF, 19) };
                return (next, Some(Fixup::Bind { ordinal, addend }));
            }
            if auth {
                return (next, Some(Fixup::Rebase(imagebase.wrapping_add(raw & 0xFFFF_FFFF))));
            }
            let unpacked = (raw & 0x7FF_FFFF_FFFF) | (((raw >> 43) & 0xFF) << 56);
            let resolved = match format.pointer_format {
                ptr_format::ARM64E => unpacked,
                _ => imagebase.wrapping_add(unpacked),
            };
            (next, Some(Fixup::Rebase(resolved)))
        }
        _ => (0, None),
    }
}

/// Map a VMA to its file offset via the segment table (the inverse of the load
/// arithmetic). `None` for a VMA outside every mapped segment.
fn va_to_fileoff(segs: &[SegInfo], va: u64) -> Option<u64> {
    for s in segs {
        if va >= s.vmaddr && va - s.vmaddr < s.filesize {
            return s.fileoff.checked_add(va - s.vmaddr);
        }
    }
    None
}

// --- little-endian scalar readers over a byte slice (bounds-checked) ---

fn rd_u16(b: &[u8], off: usize) -> Option<u16> {
    b.get(off..off.checked_add(2)?).map(|s| u16::from_le_bytes(s.try_into().unwrap()))
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

    const BASE: u64 = 0x1_0000_0000;

    fn fmt(pointer_format: u16) -> Format {
        Format { pointer_format, imagebase: BASE, max_valid_pointer: 0 }
    }

    /// `DYLD_CHAINED_PTR_64` rebase: the `next` field (bits 62..51, in 4-byte
    /// stride units) is stripped and the low-36-bit target is the vmaddr — the
    /// `macho_objc_arm64` fixture's `class_t.isa` word `0x0020000100008028`.
    #[test]
    fn decode_ptr64_rebase_strips_next() {
        let (next, fixup) = decode_ptr(0x0020_0001_0000_8028, &fmt(ptr_format::PTR_64));
        assert_eq!(next, 4, "next field is bits 62..51 (×4-byte stride ⇒ +0x10 hop)");
        assert_eq!(fixup, Some(Fixup::Rebase(0x1_0000_8028)));
        let (next, fixup) = decode_ptr(0x0000_0001_0000_8000, &fmt(ptr_format::PTR_64));
        assert_eq!((next, fixup), (0, Some(Fixup::Rebase(0x1_0000_8000))));
    }

    /// `DYLD_CHAINED_PTR_64` bind (bit 63): a 24-bit ordinal and an 8-bit addend.
    #[test]
    fn decode_ptr64_bind_names_its_ordinal() {
        let (next, fixup) = decode_ptr(0x8020_0000_0300_0002, &fmt(ptr_format::PTR_64));
        assert_eq!(next, 4, "the bind entry still carries a next hop");
        assert_eq!(fixup, Some(Fixup::Bind { ordinal: 2, addend: 3 }));
    }

    /// `DYLD_CHAINED_PTR_64_OFFSET` (Apple `ld64`'s arm64 format) adds the image
    /// base: GH #1333's function-pointer slot reads `1004000000000000` on disk,
    /// the word `0x410`, which is `audit_target` at `0x100000410`.
    #[test]
    fn decode_ptr64_offset_adds_imagebase() {
        let (_n, fixup) = decode_ptr(0x0000_0000_0000_0410, &fmt(ptr_format::PTR_64_OFFSET));
        assert_eq!(fixup, Some(Fixup::Rebase(0x1_0000_0410)));
    }

    /// `DYLD_CHAINED_PTR_32`: a 26-bit target, a 5-bit next, and a bind with a
    /// 20-bit ordinal and 6-bit addend; a target above `max_valid_pointer` is a
    /// biased non-pointer.
    #[test]
    fn decode_ptr32_rebase_bind_and_non_pointer() {
        let (next, fixup) = decode_ptr((3 << 26) | 0x4010, &fmt(ptr_format::PTR_32));
        assert_eq!((next, fixup), (3, Some(Fixup::Rebase(0x4010))));
        let (next, fixup) = decode_ptr((1 << 31) | (1 << 26) | (5 << 20) | 7, &fmt(ptr_format::PTR_32));
        assert_eq!((next, fixup), (1, Some(Fixup::Bind { ordinal: 7, addend: 5 })));
        let biased = Format { pointer_format: ptr_format::PTR_32, imagebase: 0, max_valid_pointer: 0x10_0000 };
        let (_n, fixup) = decode_ptr(0x300_0000, &biased);
        assert_eq!(fixup, Some(Fixup::Rebase(0x300_0000 - (0x400_0000 + 0x10_0000) / 2)));
    }

    /// arm64e auth rebase: the PAC context (32..50) is stripped and the 32-bit
    /// target rebased off the image base.
    #[test]
    fn decode_arm64e_auth_rebase_strips_pac() {
        let raw = (1u64 << 63) | (3u64 << 51) | (0xABCDu64 << 32) | 0x0000_8028;
        let (next, fixup) = decode_ptr(raw, &fmt(ptr_format::ARM64E));
        assert_eq!(next, 3, "arm64e next is the 11-bit field at 61..51");
        assert_eq!(fixup, Some(Fixup::Rebase(0x1_0000_8028)));
    }

    /// arm64e plain rebase: the 43-bit target is a vmaddr for `ARM64E` and an
    /// image-base offset for `ARM64E_KERNEL` (7) and the `USERLAND` formats (9,
    /// and 12 = `USERLAND24`).
    #[test]
    fn decode_arm64e_plain_and_userland_rebase() {
        let (next, fixup) = decode_ptr((5u64 << 51) | 0x1_0000_8050, &fmt(ptr_format::ARM64E));
        assert_eq!((next, fixup), (5, Some(Fixup::Rebase(0x1_0000_8050))));
        for format in [
            ptr_format::ARM64E_KERNEL,
            ptr_format::ARM64E_USERLAND,
            ptr_format::ARM64E_USERLAND24,
        ] {
            let (_n, fixup) = decode_ptr((2u64 << 51) | 0x8050, &fmt(format));
            assert_eq!(fixup, Some(Fixup::Rebase(0x1_0000_8050)), "format {format}");
        }
    }

    /// arm64e binds: a 16-bit ordinal (24-bit for `USERLAND24`) and, unless
    /// authenticated, a signed 19-bit addend.
    #[test]
    fn decode_arm64e_binds() {
        let raw = (1u64 << 62) | (0x7_FFFFu64 << 32) | 0x12_0003;
        let (_n, fixup) = decode_ptr(raw, &fmt(ptr_format::ARM64E));
        assert_eq!(fixup, Some(Fixup::Bind { ordinal: 3, addend: -1 }));
        let (_n, fixup) = decode_ptr(raw, &fmt(ptr_format::ARM64E_USERLAND24));
        assert_eq!(fixup, Some(Fixup::Bind { ordinal: 0x12_0003, addend: -1 }));
        let (_n, fixup) = decode_ptr(raw | (1 << 63), &fmt(ptr_format::ARM64E));
        assert_eq!(fixup, Some(Fixup::Bind { ordinal: 3, addend: 0 }));
    }

    /// An unhandled pointer format resolves nothing and stops the chain.
    #[test]
    fn decode_unknown_format_is_inert() {
        assert_eq!(decode_ptr(0xDEAD_BEEF_0000_0000, &fmt(999)), (0, None));
    }

    /// The three import-record layouts, names read from the symbol pool.
    #[test]
    fn imports_table_names_every_format() {
        for (format, stride) in [(1u32, 4usize), (2, 8), (3, 16)] {
            let mut blob = vec![0u8; 28];
            let imports_offset = 28u32;
            let symbols_offset = imports_offset + 2 * stride as u32;
            for (i, v) in [0u32, 0, imports_offset, symbols_offset, 2, format, 0].iter().enumerate() {
                blob[i * 4..i * 4 + 4].copy_from_slice(&v.to_le_bytes());
            }
            for (name_offset, weak, addend) in [(1u64, false, 0i64), (9, true, 16)] {
                match format {
                    3 => {
                        let raw = (name_offset << 32) | (u64::from(weak) << 16) | 1;
                        blob.extend_from_slice(&raw.to_le_bytes());
                        blob.extend_from_slice(&(addend as u64).to_le_bytes());
                    }
                    _ => {
                        let raw = ((name_offset as u32) << 9) | (u32::from(weak) << 8) | 1;
                        blob.extend_from_slice(&raw.to_le_bytes());
                        if format == 2 {
                            blob.extend_from_slice(&(addend as i32).to_le_bytes());
                        }
                    }
                }
            }
            blob.extend_from_slice(b"\0_printf\0_environ\0");
            let imports = read_imports(&blob, &[0, 1, 2]);
            assert_eq!(imports.len(), 2, "format {format}");
            assert_eq!((imports[0].0, &*imports[0].1.name), (0, &b"_printf"[..]));
            assert_eq!((imports[1].0, &*imports[1].1.name), (1, &b"_environ"[..]));
            assert!(imports[1].1.weak && !imports[0].1.weak, "format {format}");
            assert_eq!(imports[1].1.addend, if format == 1 { 0 } else { 16 }, "format {format}");
            assert_eq!(read_imports(&blob, &[1]).len(), 1, "only the bound ordinals are read");
        }
    }

    /// A table whose names all run on to the end of the pool reads each byte of
    /// the pool at most once: the unterminated names are empty, and imports
    /// sharing a name share one copy.
    #[test]
    fn imports_table_reads_the_pool_once() {
        const N: u32 = 1000;
        let symbols_offset = 28 + 4 * N;
        let mut blob = vec![0u8; 28];
        for (i, v) in [0u32, 0, 28, symbols_offset, N, 1, 0].iter().enumerate() {
            blob[i * 4..i * 4 + 4].copy_from_slice(&v.to_le_bytes());
        }
        for _ in 0..N {
            blob.extend_from_slice(&1u32.to_le_bytes());
        }
        blob.extend_from_slice(&[b'A'; 4096]);
        let ordinals: Vec<u32> = (0..N).collect();
        let imports = read_imports(&blob, &ordinals);
        assert_eq!(imports.len(), N as usize);
        assert!(imports.iter().all(|(_, import)| import.name.is_empty()));

        let last = blob.len() - 1;
        blob[last] = 0;
        let imports = read_imports(&blob, &ordinals);
        assert_eq!(imports[0].1.name.len(), 4095);
        assert!(imports.iter().all(|(_, import)| Arc::ptr_eq(&import.name, &imports[0].1.name)));
    }

    /// The fixture's chains: two rebases, and one bind named through the
    /// imports table, in both on-disk layouts.
    #[test]
    fn fixture_rebases_and_binds() {
        for name in ["macho_ptrslots_arm64", "macho_ptrslots_offset_arm64"] {
            let path = format!("{}/tests/fixtures/{name}", env!("CARGO_MANIFEST_DIR"));
            let bytes = std::fs::read(&path).expect("fixture");
            let file = object::File::parse(&bytes[..]).expect("parse");
            let fixups = resolve_chained_fixups(&file, &bytes);
            let mut rebases: Vec<(u64, u64)> = fixups.rebases().collect();
            rebases.sort_unstable();
            assert_eq!(rebases, vec![(0x100004000, 0x100000440), (0x100004008, 0x1000003c8)], "{name}");
            let binds: Vec<(u64, &ChainedBind)> = fixups.binds().collect();
            let want = ChainedBind { name: Arc::from(&b"_audit_imported"[..]), addend: 0, weak: false };
            assert_eq!(binds, vec![(0x100004010, &want)], "{name}");
            assert_eq!(fixups.slot_width(), 8);
        }
    }

    /// A chained-fixups blob with one segment's starts at offset 36: `page_size`,
    /// `pointer_format`, `segment_offset` and `page_count` page starts of 0.
    fn starts_blob(page_size: u16, pointer_format: u16, segment_offset: u64, page_count: u16) -> Vec<u8> {
        let mut blob = vec![0u8; 36 + 22 + 2 * page_count as usize];
        blob[4..8].copy_from_slice(&28u32.to_le_bytes());
        blob[28..32].copy_from_slice(&1u32.to_le_bytes());
        blob[32..36].copy_from_slice(&8u32.to_le_bytes());
        blob[40..42].copy_from_slice(&page_size.to_le_bytes());
        blob[42..44].copy_from_slice(&pointer_format.to_le_bytes());
        blob[44..52].copy_from_slice(&segment_offset.to_le_bytes());
        blob[56..58].copy_from_slice(&page_count.to_le_bytes());
        blob
    }

    /// A hostile header: 65535 eight-byte pages, each starting a chain inside
    /// one 4097-word chain. The walk decodes the chain once: every later page
    /// start lands on a word already decoded, or outside the file.
    #[test]
    fn overlapping_page_starts_decode_each_slot_once() {
        let mut file = Vec::new();
        for _ in 0..4096 {
            file.extend_from_slice(&(2u64 << 51).to_le_bytes());
        }
        file.extend_from_slice(&0u64.to_le_bytes());
        let segs = [SegInfo { vmaddr: BASE, fileoff: 0, filesize: file.len() as u64 }];
        let blob = starts_blob(8, ptr_format::PTR_64, 0, 0xFFFF);
        let mut walk = Walk::new(&segs, &file, &blob);
        let mut out = ChainedFixups::default();
        walk_fixups(&blob, BASE, &mut walk, &mut out);
        assert_eq!(out.rebases.len(), 4097, "each slot of the chain is recorded once");
        assert_eq!(walk.starts, blob.len() / 2 + 1 - 0xFFFF, "one start read per page");
    }

    /// A hostile header: four 32-bit pages sharing one list of starts that
    /// never sets `START_LAST` and runs on to the end of the blob. Each page
    /// walks its own chain once and its list stops at the first start that
    /// heads no new chain.
    #[test]
    fn thirty_two_bit_start_lists_stop_at_a_revisited_start() {
        let mut file = vec![0u8; 0x4000];
        for page in 0..4usize {
            let at = page * 0x1000;
            file[at..at + 4].copy_from_slice(&((1u32 << 26) | 0x100).to_le_bytes());
            file[at + 4..at + 8].copy_from_slice(&0x200u32.to_le_bytes());
        }
        let segs = [SegInfo { vmaddr: BASE, fileoff: 0, filesize: file.len() as u64 }];
        let mut blob = starts_blob(0x1000, ptr_format::PTR_32, 0, 4);
        for page in 0..4usize {
            blob[58 + 2 * page..60 + 2 * page].copy_from_slice(&0x8004u16.to_le_bytes());
        }
        blob.extend_from_slice(&[0u8; 2000]);
        let mut walk = Walk::new(&segs, &file, &blob);
        let mut out = ChainedFixups::default();
        walk_fixups(&blob, BASE, &mut walk, &mut out);
        assert_eq!(out.rebases.len(), 8, "two slots on each of the four pages");
        assert_eq!(walk.starts, blob.len() / 2 + 1 - 4 - 4 * 2, "two list entries per page");
    }

    /// A hostile header: 4096 segments sharing one table of 65535 page
    /// starts. The walk reads at most one start per 2-byte entry of the blob,
    /// not 4096 * 65535 of them.
    #[test]
    fn segments_sharing_a_start_table_stop_at_the_blob_size() {
        let segs = [SegInfo { vmaddr: BASE, fileoff: 0, filesize: 0 }];
        let mut blob = starts_blob(0x4000, ptr_format::PTR_64, 0, 0xFFFF);
        let pages = blob.split_off(36);
        blob.truncate(32);
        let seg_count = 4096u32;
        blob[28..32].copy_from_slice(&seg_count.to_le_bytes());
        let table = 4 + 4 * seg_count;
        for _ in 0..seg_count {
            blob.extend_from_slice(&table.to_le_bytes());
        }
        blob.extend_from_slice(&pages);
        let file = blob.clone();
        let mut walk = Walk::new(&segs, &file, &blob);
        let mut out = ChainedFixups::default();
        walk_fixups(&blob, BASE, &mut walk, &mut out);
        assert_eq!(walk.starts, 0, "the shared table spends the blob's starts");
        assert!(out.rebases.is_empty());
    }

    /// A `seg_count` past the end of the blob stops at the blob's end, and a
    /// segment offset or segment table near the top of the address space wraps
    /// into no mapped slot rather than overflowing.
    #[test]
    fn hostile_counts_and_offsets_are_inert() {
        let file = vec![0u8; 64];
        let segs = [SegInfo { vmaddr: u64::MAX - 4, fileoff: u64::MAX, filesize: 16 }];
        let mut blob = vec![0u8; 32];
        blob[4..8].copy_from_slice(&28u32.to_le_bytes());
        blob[28..32].copy_from_slice(&u32::MAX.to_le_bytes());
        let mut walk = Walk::new(&segs, &file, &blob);
        let mut out = ChainedFixups::default();
        walk_fixups(&blob, BASE, &mut walk, &mut out);
        assert!(out.rebases.is_empty() && out.binds.is_empty());

        let blob = starts_blob(0x4000, ptr_format::PTR_64, u64::MAX - 0x10, 4);
        walk_fixups(&blob, BASE, &mut walk, &mut out);
        assert!(out.rebases.is_empty());
        assert_eq!(va_to_fileoff(&segs, u64::MAX - 2), None);
    }

    /// `resolve_chained_fixups` is a strict no-op on a non-Mach-O input.
    #[test]
    fn resolver_noop_on_non_macho() {
        let bytes = b"\x7fELF\x02\x01\x01\x00".to_vec();
        assert!(matches!(FileKind::parse(bytes.as_slice()), Ok(FileKind::Elf64) | Err(_)));
    }
}
