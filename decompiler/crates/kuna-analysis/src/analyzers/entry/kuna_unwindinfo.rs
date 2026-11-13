//! Mach-O `__unwind_info` function-start recovery (`unwindstarts`).
//!
//! The linker folds each object file's `__compact_unwind` entries into one
//! `__TEXT,__unwind_info` section: a two-level lookup whose leaf (second-level)
//! pages list, in ascending order, the start offset of every function the
//! unwinder can describe. Those offsets are function starts by construction —
//! one per `.cfi_startproc` the compiler emitted — and the section lives in the
//! mapped `__TEXT` segment, so it survives the `strip` that removes
//! `LC_FUNCTION_STARTS` from `__LINKEDIT`.
//!
//! On an image that still carries `LC_FUNCTION_STARTS` this oracle adds nothing:
//! every unwind start is already a function-starts entry (the linker folds runs
//! of adjacent same-encoding functions, so it is in fact a strict subset). Its
//! value is the stripped App-Store app whose `LC_FUNCTION_STARTS` is gone: there
//! the symbol stream, `LC_MAIN`, exports and `__mod_init_func` seed only a
//! handful of starts and the arm64 gap walk cannot reach the body, while
//! `__unwind_info` still names every function. Output-changing (it adds entries)
//! and inert when `LC_FUNCTION_STARTS` is present, so it is default-OFF.
//!
//! Only the function **starts** are read. The compact encodings (frame size,
//! saved-register set, frameless stack size) are deliberately ignored: kuna's
//! S5/S7 frame analysis recovers the stack frame from the code itself, so the
//! encoding adds nothing at the decompiler tier — the same reason `eh_frame_full`
//! inherits CFI rather than rebuilding it.

use object::macho::{MachHeader32, MachHeader64};
use object::read::macho::{FatArch, MachHeader, MachOFatFile32, MachOFatFile64, Section, Segment};
use object::{Architecture, Endianness, FileKind};

use crate::loader::format::{detect, FormatKind};
use crate::pass::{AnalysisCtx, AnalysisOutput, AnalysisPass, Phase};

/// (kuna `unwindstarts`) Mach-O `__unwind_info` function-start discovery: read
/// the second-level pages of `__TEXT,__unwind_info` and emit each function start
/// as a discovered entry. Registered always (the facts are cheap to compute and
/// are stashed at load), but the COMMIT is gated by `--option unwindstarts on`
/// (default-OFF, output-changing) via `engine.rs::analysis_pass_enabled` →
/// `arch.analysis_unwindstarts`, so a default run is byte-identical to before.
pub struct UnwindInfoStartsPass;

impl AnalysisPass for UnwindInfoStartsPass {
    fn phase(&self) -> Phase {
        Phase::P1
    }

    fn id(&self) -> &'static str {
        "unwindstarts"
    }

    fn run(&self, ctx: &AnalysisCtx) -> AnalysisOutput {
        let mut out = AnalysisOutput::default();
        // Mach-O only. Additive contract: an empty output on any anomaly, never a
        // failure.
        if !matches!(detect(ctx.file).map(|f| f.kind()), Ok(FormatKind::MachO)) {
            return out;
        }
        out.entries = unwind_info_starts(ctx.file, ctx.bytes);
        out
    }
}

/// The filtered `__unwind_info` function starts — inside an executable section,
/// not already a funcsym, sorted and deduped (the same gates every other oracle
/// applies). The testable seam (drive it over fixture bytes).
pub fn unwind_info_starts(file: &object::File, bytes: &[u8]) -> Vec<u64> {
    let mut raw: Vec<u64> = Vec::new();
    collect_thin(bytes, &mut raw);

    let execs = super::executable_sections(file);
    let funcsyms = super::existing_function_addrs_for_file(file);
    let mut out: Vec<u64> = Vec::new();
    for vma in raw {
        if vma == 0 || !super::in_executable_section(&execs, vma) {
            continue;
        }
        if funcsyms.binary_search(&vma).is_ok() {
            continue;
        }
        out.push(vma);
    }
    out.sort_unstable();
    out.dedup();
    out
}

/// Dispatch on the Mach-O file kind, peeling one fat slice (x86-64 → arm64 →
/// first, as the rest of the loader does) before walking the thin image.
fn collect_thin(bytes: &[u8], out: &mut Vec<u64>) {
    match FileKind::parse(bytes) {
        Ok(FileKind::MachO64) => walk::<MachHeader64<Endianness>>(bytes, out),
        Ok(FileKind::MachO32) => walk::<MachHeader32<Endianness>>(bytes, out),
        Ok(FileKind::MachOFat32) => {
            if let Some(slice) = select_fat_slice_32(bytes) {
                collect_thin(slice, out);
            }
        }
        Ok(FileKind::MachOFat64) => {
            if let Some(slice) = select_fat_slice_64(bytes) {
                collect_thin(slice, out);
            }
        }
        _ => {}
    }
}

/// Walk a thin Mach-O's load commands, find the `__TEXT` base and the
/// `__TEXT,__unwind_info` section bytes, and parse the second-level pages.
/// Generic over the header width so 64- and 32-bit share one body.
fn walk<Mach>(bytes: &[u8], out: &mut Vec<u64>)
where
    Mach: MachHeader<Endian = Endianness>,
{
    let Ok(header) = Mach::parse(bytes, 0) else {
        return;
    };
    let Ok(endian) = header.endian() else {
        return;
    };
    let Ok(mut commands) = header.load_commands(endian, bytes, 0) else {
        return;
    };

    // The function offsets in `__unwind_info` are relative to the image base, the
    // `__TEXT` segment vmaddr (the segment that maps the mach header).
    let mut text_base: Option<u64> = None;
    let mut unwind: Option<(u64, u64)> = None; // (file offset, size) of __unwind_info

    while let Ok(Some(command)) = commands.next() {
        let Ok(Some((segment, section_data))) = Mach::Segment::from_command(command) else {
            continue;
        };
        if trim(segment.name()) == b"__TEXT" {
            text_base = Some(segment.vmaddr(endian).into());
        }
        let Ok(secs) = segment.sections(endian, section_data) else {
            continue;
        };
        for sec in secs {
            if trim(sec.name()) == b"__unwind_info" {
                if let Some((off, size)) = sec.file_range(endian) {
                    unwind = Some((off, size));
                }
            }
        }
    }

    let (Some(base), Some((off, size))) = (text_base, unwind) else {
        return;
    };
    let start = off as usize;
    let Some(end) = start.checked_add(size as usize).filter(|&e| e <= bytes.len()) else {
        return;
    };
    parse_unwind_info(&bytes[start..end], base, out);
}

// `__unwind_info` second-level page kinds (Apple `mach-o/compact_unwind_encoding.h`).
const SECOND_LEVEL_REGULAR: u32 = 2;
const SECOND_LEVEL_COMPRESSED: u32 = 3;
// ld64 writes second-level pages of at most 4 KiB; an entry array is read only
// within its own page, so a malformed count cannot run on into the next page.
const SECOND_LEVEL_PAGE_SIZE: usize = 4096;
// Low 24 bits of a compressed entry are the function offset from the page's first
// function; the top 8 are an encoding index this oracle does not read.
const COMPRESSED_ENTRY_FUNC_OFFSET: u32 = 0x00FF_FFFF;
// The smallest entry (compressed) is 4 bytes, so a section of `n` bytes describes
// at most `n / 4` functions. Capping the output there keeps the work linear in the
// section size when index entries point at the same or overlapping pages.
const MIN_ENTRY_SIZE: usize = 4;

/// Parse a `__unwind_info` blob, pushing `base + functionOffset` for every entry
/// in every second-level page. Total and linear in `buf.len()`: a short or
/// malformed blob yields fewer starts, never a panic or a runaway loop.
fn parse_unwind_info(buf: &[u8], base: u64, out: &mut Vec<u64>) {
    // unwind_info_section_header: version, then three (offset, count) pairs; the
    // index array is the last pair.
    let (Some(index_off), Some(index_count)) = (read_u32(buf, 20), read_u32(buf, 24)) else {
        return;
    };
    let index_off = index_off as usize;
    // Each index entry is 12 bytes: functionOffset, secondLevelPagesSectionOffset,
    // lsdaIndexArraySectionOffset. The last entry is a sentinel (end address, no
    // page), and only entries that lie inside the section are read.
    let fit = buf.len().saturating_sub(index_off) / 12;
    let pages = (index_count as usize).saturating_sub(1).min(fit);
    let limit = out.len().saturating_add(buf.len() / MIN_ENTRY_SIZE);
    for i in 0..pages {
        let eo = index_off + i * 12;
        let (Some(first_func), Some(page_off)) = (read_u32(buf, eo), read_u32(buf, eo + 4)) else {
            break;
        };
        let page_off = page_off as usize;
        if page_off == 0 {
            continue;
        }
        match read_u32(buf, page_off) {
            Some(SECOND_LEVEL_REGULAR) => parse_regular_page(buf, page_off, base, limit, out),
            Some(SECOND_LEVEL_COMPRESSED) => {
                parse_compressed_page(buf, page_off, base, first_func as u64, limit, out)
            }
            _ => {}
        }
        if out.len() >= limit {
            return;
        }
    }
}

/// The entry array of the second-level page at `page_off` as `(offset of the
/// first entry, entry count)`, the count clamped to what fits inside the page's
/// 4 KiB extent and inside the section.
fn page_entries(buf: &[u8], page_off: usize, entry_size: usize) -> Option<(usize, usize)> {
    let entry_page_off = read_u16(buf, page_off.checked_add(4)?)? as usize;
    let entry_count = read_u16(buf, page_off.checked_add(6)?)? as usize;
    let first = page_off.checked_add(entry_page_off)?;
    let end = page_off.saturating_add(SECOND_LEVEL_PAGE_SIZE).min(buf.len());
    Some((first, entry_count.min(end.saturating_sub(first) / entry_size)))
}

/// Regular second-level page: a `(functionOffset, encoding)` pair per entry,
/// functionOffset already image-relative.
fn parse_regular_page(buf: &[u8], page_off: usize, base: u64, limit: usize, out: &mut Vec<u64>) {
    let Some((first, count)) = page_entries(buf, page_off, 8) else {
        return;
    };
    for e in 0..count.min(limit.saturating_sub(out.len())) {
        if let Some(func_off) = read_u32(buf, first + e * 8) {
            out.push(base.wrapping_add(func_off as u64));
        }
    }
}

/// Compressed second-level page: a `u32` per entry whose low 24 bits are the
/// function offset from the page's first function (`first_func`, image-relative).
fn parse_compressed_page(
    buf: &[u8],
    page_off: usize,
    base: u64,
    first_func: u64,
    limit: usize,
    out: &mut Vec<u64>,
) {
    let Some((first, count)) = page_entries(buf, page_off, 4) else {
        return;
    };
    for e in 0..count.min(limit.saturating_sub(out.len())) {
        if let Some(val) = read_u32(buf, first + e * 4) {
            let delta = (val & COMPRESSED_ENTRY_FUNC_OFFSET) as u64;
            out.push(base.wrapping_add(first_func + delta));
        }
    }
}

/// Little-endian `u32` at `buf[off..off+4]`, or `None` if out of range.
fn read_u32(buf: &[u8], off: usize) -> Option<u32> {
    let b = buf.get(off..off.checked_add(4)?)?;
    Some(u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
}

/// Little-endian `u16` at `buf[off..off+2]`, or `None` if out of range.
fn read_u16(buf: &[u8], off: usize) -> Option<u16> {
    let b = buf.get(off..off.checked_add(2)?)?;
    Some(u16::from_le_bytes([b[0], b[1]]))
}

/// Trim a NUL-padded fixed-size Mach-O name field to its content bytes.
fn trim(name: &[u8]) -> &[u8] {
    match name.iter().position(|&b| b == 0) {
        Some(i) => &name[..i],
        None => name,
    }
}

/// Select one slice of a 32-bit fat header (x86-64, then arm64, then first).
fn select_fat_slice_32(bytes: &[u8]) -> Option<&[u8]> {
    let fat = MachOFatFile32::parse(bytes).ok()?;
    select_fat_arch(fat.arches()).and_then(|a| a.data(bytes).ok())
}

/// Select one slice of a 64-bit fat header.
fn select_fat_slice_64(bytes: &[u8]) -> Option<&[u8]> {
    let fat = MachOFatFile64::parse(bytes).ok()?;
    select_fat_arch(fat.arches()).and_then(|a| a.data(bytes).ok())
}

/// Deterministic fat-slice preference (x86-64, then arm64, then first).
fn select_fat_arch<Fat: FatArch>(arches: &[Fat]) -> Option<&Fat> {
    arches
        .iter()
        .find(|a| a.architecture() == Architecture::X86_64)
        .or_else(|| arches.iter().find(|a| a.architecture() == Architecture::Aarch64))
        .or_else(|| arches.first())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture(name: &str) -> Vec<u8> {
        let path = format!("{}/tests/fixtures/{}", env!("CARGO_MANIFEST_DIR"), name);
        std::fs::read(&path).unwrap_or_else(|_| panic!("read fixture {path}"))
    }

    /// The headline: on a stripped arm64 Mach-O (no `LC_FUNCTION_STARTS`, local
    /// symbols removed) the four `uw_*` function starts live only in
    /// `__TEXT,__unwind_info`, and this oracle recovers exactly them.
    #[test]
    fn recovers_the_unwind_only_function_starts() {
        let bytes = fixture("macho_unwind_only_arm64");
        let file = object::File::parse(bytes.as_slice()).expect("parse fixture");
        let starts = unwind_info_starts(&file, &bytes);
        for a in [0x1000002e0u64, 0x100000328, 0x100000370, 0x1000003bc] {
            assert!(starts.contains(&a), "unwind start {a:#x} missing; got {starts:#x?}");
        }
        // `_main` (0x10000040c) is a function symbol, so the funcsym filter drops it
        // (another oracle already names it) — the pass only ADDS new starts.
        assert!(!starts.contains(&0x10000040c), "a funcsym start must be filtered out");
        // The Mach header (0x100000000) is not in an executable section.
        assert!(!starts.contains(&0x100000000), "the mach header is not a function");
    }

    /// Mach-O only: an ELF image yields nothing (the parser finds no section and
    /// `collect_thin` never runs).
    #[test]
    fn non_macho_yields_nothing() {
        let bytes = fixture("cet_pie_x86_64");
        let file = object::File::parse(bytes.as_slice()).expect("parse elf");
        assert!(unwind_info_starts(&file, &bytes).is_empty());
    }

    /// Every read is bounds-checked: a short or empty blob is parsed to no starts,
    /// never a panic.
    #[test]
    fn short_blob_does_not_panic() {
        let mut out = Vec::new();
        parse_unwind_info(&[], 0x1000, &mut out);
        parse_unwind_info(&[1, 0, 0, 0], 0x1000, &mut out);
        parse_unwind_info(&[0u8; 27], 0x1000, &mut out);
        assert!(out.is_empty());
    }

    /// A section header: version 1, no common encodings or personalities, and an
    /// index array of `index_count` entries at `index_off`.
    fn header(index_off: u32, index_count: u32) -> Vec<u8> {
        let mut b = Vec::new();
        for v in [1u32, 0, 0, 0, 0, index_off, index_count] {
            b.extend_from_slice(&v.to_le_bytes());
        }
        b
    }

    /// A compressed second-level page claiming `entry_count` entries, padded with
    /// zero entries to `len` bytes.
    fn compressed_page(entry_count: u16, len: usize) -> Vec<u8> {
        let mut p = Vec::new();
        p.extend_from_slice(&3u32.to_le_bytes());
        p.extend_from_slice(&12u16.to_le_bytes());
        p.extend_from_slice(&entry_count.to_le_bytes());
        p.extend_from_slice(&[0u8; 4]);
        p.resize(len, 0);
        p
    }

    /// An index count near `u32::MAX` over a 28-byte section reads the index
    /// entries that fit (none) and stops, rather than looping over the count.
    #[test]
    fn huge_index_count_is_clamped_to_the_section() {
        let mut out = Vec::new();
        parse_unwind_info(&header(28, u32::MAX), 0x1000, &mut out);
        assert!(out.is_empty());
    }

    /// A page whose entry count runs past its 4 KiB extent yields only the entries
    /// inside the page; the bytes after it are not read as function offsets.
    #[test]
    fn page_entries_stay_inside_their_page() {
        let mut buf = header(28, 2);
        for v in [0x1000u32, 64, 0, 0x2000, 0, 0] {
            buf.extend_from_slice(&v.to_le_bytes());
        }
        buf.resize(64, 0);
        buf.extend_from_slice(&compressed_page(0xFFFF, 2 * SECOND_LEVEL_PAGE_SIZE));
        let mut out = Vec::new();
        parse_unwind_info(&buf, 0, &mut out);
        assert_eq!(out.len(), (SECOND_LEVEL_PAGE_SIZE - 12) / 4);
    }

    /// Many index entries naming the same full page cannot multiply the output:
    /// it is capped at one start per four section bytes.
    #[test]
    fn repeated_pages_are_capped_by_the_section_size() {
        let pages = 200u32;
        let page_off = 28 + (pages + 1) * 12;
        let mut buf = header(28, pages + 1);
        for _ in 0..pages {
            for v in [0x1000u32, page_off, 0] {
                buf.extend_from_slice(&v.to_le_bytes());
            }
        }
        buf.extend_from_slice(&[0u8; 12]);
        buf.extend_from_slice(&compressed_page(0xFFFF, SECOND_LEVEL_PAGE_SIZE));
        let mut out = Vec::new();
        parse_unwind_info(&buf, 0, &mut out);
        assert!(!out.is_empty());
        assert!(out.len() <= buf.len() / MIN_ENTRY_SIZE, "{} starts from {} bytes", out.len(), buf.len());
    }
}
