//! Mach-O Core Foundation constant strings (`__cfstring`) — render a `CFSTR("x")`
//! constant as the string literal it holds instead of `dat_<addr>`.
//!
//! # What a `__cfstring` entry is
//!
//! A `CFSTR("launch-report")` in source compiles to a 4-word `CFString` struct
//! in the `__cfstring` section (`__DATA_CONST,__cfstring` on a modern toolchain,
//! `__DATA,__cfstring` historically):
//!
//! ```text
//!   +0   isa      -> ___CFConstantStringClassReference  (a bind/rebase)
//!   +8   flags    (info word; bit 0x10 set => UTF-16 char storage, else 8-bit)
//!   +16  data     -> the char bytes, in __cstring (8-bit) or __ustring (UTF-16)
//!   +24  length   character count (NOT byte count)
//! ```
//!
//! Code references the *struct* address, not the char bytes, so the printer's
//! constant-string route (which reads the bytes at the referenced address) sees
//! the struct and renders `dat_<addr>`. This pass resolves each entry's `data`
//! pointer (through the chained-fixup overlay on an arm64(e) image, a raw word on
//! a classic `LC_DYLD_INFO` image), reads the string, and emits a [`CFStringFact`].
//! The commit seam types the entry an opaque `struct __CFString`, whose pointer
//! is `CFStringRef`, and pre-seeds the `StringManager` at the entry address with
//! the resolved bytes, so a reference renders the literal.
//!
//! # Gating & faithfulness
//!
//! Registered only for a Mach-O target (`passes.rs`), default-on (`--option
//! cfstrings`), output-changing. The whole pass output is dropped at commit when
//! the gate is off, so `cfstrings off` renders each entry as before. A non-Mach-O
//! image, an image with no `__cfstring`, or an unreadable entry yields nothing —
//! additive, never wrong.
//!
//! # Bounds
//!
//! The walk stops at the end of the file, not the section header, and the string
//! bytes read over all entries are capped at the file's length. Entries are 32
//! bytes but each may point at the same 64 KiB run, so without that cap a crafted
//! section would make every entry an owned copy of the run (twice, once more in
//! the `StringManager`), about 4096x the file. Each entry's reads scan the image's
//! section spans, so the walk is entries x sections, both sized by the file; a
//! budget of span checks ends it long before a real image would reach it.

use object::read::{Object, ObjectSection};

use crate::loader::format::macho::resolve_chained_fixups;
use crate::objc::sections::MachoImage;
use crate::pass::{AnalysisCtx, AnalysisOutput, AnalysisPass, CFStringFact, Phase};

/// The `__cfstring` section short name (`object` exposes the bare section name).
const SEC_CFSTRING: &str = "__cfstring";

/// CFString `flags` bit that marks UTF-16 (2-byte) char storage; clear is 8-bit.
const CFSTRING_UNICODE: u64 = 0x10;

/// Upper bound on one constant string's byte length, so a corrupt `length` field
/// cannot drive an unbounded read/allocation.
const MAX_CFSTRING_BYTES: u64 = 1 << 16;

/// Span checks the walk may spend: an entry makes four reads, and each read scans
/// the image's section spans in turn.
const MAX_SPAN_CHECKS: u64 = 1 << 30;

/// The `__cfstring` constant-string recovery pass.
pub struct CFStringPass;

impl AnalysisPass for CFStringPass {
    fn phase(&self) -> Phase {
        Phase::P1
    }

    fn id(&self) -> &'static str {
        "cfstrings"
    }

    fn run(&self, ctx: &AnalysisCtx) -> AnalysisOutput {
        collect(ctx.file, ctx.bytes)
    }
}

/// The pass body over a parsed image and its raw bytes. The walk is bounded by
/// the file, not the section header. `flags` and `length` are scalars, so
/// `read_ptr`'s fixup overlay misses and returns the raw word, while `data` is the
/// rebase the overlay resolves on an arm64(e) chained image (a raw pointer on a
/// classic `LC_DYLD_INFO` one). 8-bit data is kept only when it is UTF-8 (the
/// printer would cut an invalid one short), and a string holding a NUL is skipped
/// because a C literal stops there, trailing NULs included. Every string read is
/// charged to a budget of the file's length. Each entry is marked read-only: a CFString
/// constant is immutable, and the printer's constant-string route needs that on the
/// historical writable `__DATA,__cfstring` layout.
fn collect(file: &object::File, bytes: &[u8]) -> AnalysisOutput {
    collect_bounded(file, bytes, MAX_SPAN_CHECKS)
}

/// [`collect`] with an explicit budget of section-span checks.
fn collect_bounded(file: &object::File, bytes: &[u8], max_span_checks: u64) -> AnalysisOutput {
    let mut out = AnalysisOutput::default();
    if file.format() != object::BinaryFormat::MachO {
        return out;
    }
    let Some(sec) = file.section_by_name(SEC_CFSTRING) else {
        return out;
    };
    let fixups = resolve_chained_fixups(file, bytes);
    let img = MachoImage::with_fixups(file, fixups);
    let base = sec.address();
    let size = sec.size().min(bytes.len() as u64);
    let ps = img.ptr_size();
    let stride = ps.saturating_mul(4);
    if stride == 0 {
        return out;
    }

    let max_entries = max_span_checks / (4 * file.sections().count().max(1) as u64);
    let mut budget = bytes.len() as u64;
    let mut off: u64 = 0;
    let mut walked: u64 = 0;
    while off.saturating_add(stride) <= size && walked < max_entries {
        walked += 1;
        let entry = base.wrapping_add(off);
        off += stride;
        let (Some(flags), Some(data), Some(length)) = (
            img.read_ptr(entry.wrapping_add(ps)),
            img.read_ptr(entry.wrapping_add(ps.wrapping_mul(2))),
            img.read_ptr(entry.wrapping_add(ps.wrapping_mul(3))),
        ) else {
            continue;
        };
        if data == 0 || length == 0 {
            continue;
        }
        let unicode = flags & CFSTRING_UNICODE != 0;
        let Some(byte_len) = (if unicode { length.checked_mul(2) } else { Some(length) }) else {
            continue;
        };
        if byte_len == 0 || byte_len > MAX_CFSTRING_BYTES || byte_len > budget {
            continue;
        }
        let Some(raw) = img.read(data, byte_len as usize) else {
            continue;
        };
        budget -= byte_len;
        let text = if unicode {
            match decode_utf16(raw, file.is_little_endian()) {
                Some(b) => b,
                None => continue,
            }
        } else {
            match std::str::from_utf8(raw) {
                Ok(_) => raw.to_vec(),
                Err(_) => continue,
            }
        };
        if nul_inside(&text) {
            continue;
        }
        out.cfstrings.push(CFStringFact { addr: entry, bytes: text, size: stride as u32 });
        out.readonly.push((entry, entry.wrapping_add(stride)));
    }
    out
}

/// Does `text` hold a NUL? The printed literal would end there, so even a
/// trailing one (`CFSTR("a\0\0")`, length 3) would print as a shorter string.
fn nul_inside(text: &[u8]) -> bool {
    text.contains(&0)
}

/// Decode a UTF-16 char buffer to UTF-8 bytes. `None` on an odd length or an
/// invalid code-unit sequence (the entry is then skipped, never mis-rendered).
fn decode_utf16(raw: &[u8], little_endian: bool) -> Option<Vec<u8>> {
    if raw.len() % 2 != 0 {
        return None;
    }
    let units: Vec<u16> = raw
        .chunks_exact(2)
        .map(|c| {
            if little_endian {
                u16::from_le_bytes([c[0], c[1]])
            } else {
                u16::from_be_bytes([c[0], c[1]])
            }
        })
        .collect();
    String::from_utf16(&units).ok().map(String::into_bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture() -> Vec<u8> {
        std::fs::read(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/macho_cfstring")).unwrap()
    }

    fn texts(bytes: &[u8]) -> Vec<String> {
        let file = object::File::parse(bytes).unwrap();
        collect(&file, bytes).cfstrings.iter().map(|f| String::from_utf8(f.bytes.clone()).unwrap()).collect()
    }

    fn section_size_offset(bytes: &[u8]) -> usize {
        let file = object::File::parse(bytes).unwrap();
        let sec = file.section_by_name(SEC_CFSTRING).unwrap();
        let (file_off, _) = sec.file_range().unwrap();
        // `section_64` header: sectname[16] segname[16] addr size offset ...; find the
        // header whose `offset` field names this section's file offset.
        (0..bytes.len() - 80)
            .find(|&at| bytes[at..at + 16].starts_with(b"__cfstring\0")
                && u32::from_le_bytes(bytes[at + 48..at + 52].try_into().unwrap()) as u64 == file_off)
            .map(|at| at + 40)
            .unwrap()
    }

    #[test]
    fn reads_every_entry_of_the_fixture() {
        assert_eq!(
            texts(&fixture()),
            ["cfstring hello", "a constant string longer than one entry", "after the long one"]
        );
    }

    #[test]
    fn a_section_size_past_the_file_does_not_drive_the_walk() {
        let mut bytes = fixture();
        let at = section_size_offset(&bytes);
        bytes[at..at + 8].copy_from_slice(&0xffff_ffff_ffff_0000u64.to_le_bytes());
        let started = std::time::Instant::now();
        let _ = texts(&bytes);
        assert!(started.elapsed() < std::time::Duration::from_secs(5));
    }

    /// A minimal arm64 Mach-O whose `__cfstring` holds `n` entries that all point
    /// at one `run`-byte `__cstring` run.
    fn shared_run_image(n: usize, run: usize) -> Vec<u8> {
        let (cmds, data_off) = (72 + 2 * 80, 32 + 72 + 2 * 80);
        let cf_off = data_off + run;
        let mut b = vec![0u8; cf_off + n * 32];
        let total = b.len() as u64;
        let base = 0x1_0000_0000u64;
        let put32 = |b: &mut Vec<u8>, at: usize, v: u32| b[at..at + 4].copy_from_slice(&v.to_le_bytes());
        let put64 = |b: &mut Vec<u8>, at: usize, v: u64| b[at..at + 8].copy_from_slice(&v.to_le_bytes());
        let name = |b: &mut Vec<u8>, at: usize, s: &str| b[at..at + s.len()].copy_from_slice(s.as_bytes());
        for (at, v) in [(0, 0xfeed_facf), (4, 0x0100_000c), (12, 2), (16, 1), (20, cmds as u32)] {
            put32(&mut b, at, v);
        }
        put32(&mut b, 32, 0x19);
        put32(&mut b, 36, cmds as u32);
        name(&mut b, 40, "__DATA");
        put64(&mut b, 56, base);
        put64(&mut b, 64, total);
        put64(&mut b, 80, total);
        for at in [88, 92] {
            put32(&mut b, at, 3);
        }
        put32(&mut b, 96, 2);
        for (i, (sect, off, size)) in [("__cstring", data_off, run), ("__cfstring", cf_off, n * 32)].into_iter().enumerate() {
            let at = 104 + i * 80;
            name(&mut b, at, sect);
            name(&mut b, at + 16, "__DATA");
            put64(&mut b, at + 32, base + off as u64);
            put64(&mut b, at + 40, size as u64);
            put32(&mut b, at + 48, off as u32);
        }
        b[data_off..cf_off].fill(b'a');
        for e in 0..n {
            let at = cf_off + e * 32;
            put64(&mut b, at + 8, 0x7c8);
            put64(&mut b, at + 16, base + data_off as u64);
            put64(&mut b, at + 24, run as u64);
        }
        b
    }

    #[test]
    fn entries_sharing_one_run_read_no_more_than_the_file() {
        let bytes = shared_run_image(1024, MAX_CFSTRING_BYTES as usize);
        let file = object::File::parse(&*bytes).unwrap();
        let out = collect(&file, &bytes);
        let total: usize = out.cfstrings.iter().map(|f| f.bytes.len()).sum();
        assert!(!out.cfstrings.is_empty());
        assert!(total <= bytes.len(), "{total} string bytes from a {}-byte file", bytes.len());
    }

    #[test]
    fn the_walk_stops_at_its_budget_of_span_checks() {
        let bytes = shared_run_image(64, 16);
        let file = object::File::parse(&*bytes).unwrap();
        assert_eq!(collect(&file, &bytes).cfstrings.len(), 64);
        let budget = 4 * file.sections().count() as u64 * 10;
        assert_eq!(collect_bounded(&file, &bytes, budget).cfstrings.len(), 10);
    }

    #[test]
    fn a_string_holding_a_nul_is_skipped_not_cut_short() {
        assert!(!nul_inside(b"plain"));
        assert!(nul_inside(b"a\0b"));
        let mut bytes = fixture();
        let at = bytes.windows(14).position(|w| w == b"cfstring hello").unwrap();
        bytes[at + 8] = 0;
        assert_eq!(texts(&bytes), ["a constant string longer than one entry", "after the long one"]);
    }

    #[test]
    fn a_string_ending_in_a_nul_is_skipped_too() {
        assert!(nul_inside(b"trailing\0"));
        let mut bytes = fixture();
        let at = bytes.windows(14).position(|w| w == b"cfstring hello").unwrap();
        bytes[at + 13] = 0;
        assert_eq!(texts(&bytes), ["a constant string longer than one entry", "after the long one"]);
    }

    #[test]
    fn utf16_decodes_in_either_byte_order_and_refuses_a_lone_surrogate() {
        let snowman_le = [0x03, 0x26, 0x20, 0x00];
        let snowman_be = [0x26, 0x03, 0x00, 0x20];
        assert_eq!(decode_utf16(&snowman_le, true).unwrap(), "\u{2603} ".as_bytes());
        assert_eq!(decode_utf16(&snowman_be, false).unwrap(), "\u{2603} ".as_bytes());
        assert_eq!(decode_utf16(&[0x00, 0xd8], true), None);
    }

    #[test]
    fn an_8bit_entry_that_is_not_utf8_is_skipped() {
        let mut bytes = fixture();
        let at = bytes.windows(14).position(|w| w == b"cfstring hello").unwrap();
        bytes[at + 3] = 0xff;
        assert_eq!(texts(&bytes), ["a constant string longer than one entry", "after the long one"]);
    }
}
