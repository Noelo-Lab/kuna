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
//! The commit seam types the entry a `char[N]` and pre-seeds the `StringManager`
//! at the entry address with the resolved bytes, so a reference renders the
//! literal.
//!
//! # Gating & faithfulness
//!
//! Registered only for a Mach-O target (`passes.rs`), default-OFF (`--option
//! cfstrings`), output-changing. The whole pass output is dropped at commit when
//! the gate is off, so a default run is byte-identical. A non-Mach-O image, an
//! image with no `__cfstring`, or an unreadable entry yields nothing — additive,
//! never wrong.

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

/// The pass body over a parsed image and its raw bytes.
fn collect(file: &object::File, bytes: &[u8]) -> AnalysisOutput {
    let mut out = AnalysisOutput::default();
    if file.format() != object::BinaryFormat::MachO {
        return out;
    }
    // The chained-fixup overlay resolves each entry's `data` pointer on an
    // arm64(e) image; empty (raw reads) on a classic LC_DYLD_INFO image.
    let fixups = resolve_chained_fixups(file, bytes);
    let img = MachoImage::with_fixups(file, fixups);
    let Some(sec) = file.section_by_name(SEC_CFSTRING) else {
        return out;
    };
    let base = sec.address();
    // A header can declare a size far past the file; no entry lies outside it.
    let size = sec.size().min(bytes.len() as u64);
    let ps = img.ptr_size();
    // One entry is four pointer-width words (isa, flags, data, length).
    let stride = ps.saturating_mul(4);
    if stride == 0 {
        return out;
    }

    let mut off: u64 = 0;
    while off.saturating_add(stride) <= size {
        let entry = base.wrapping_add(off);
        off += stride;
        // `flags` and `length` are scalars (never a fixup), so `read_ptr`'s overlay
        // misses and returns the raw word; `data` is a rebase/auth-rebase the
        // overlay resolves on a chained image and a raw pointer on a classic one.
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
        if byte_len == 0 || byte_len > MAX_CFSTRING_BYTES {
            continue;
        }
        let Some(raw) = img.read(data, byte_len as usize) else {
            continue;
        };
        let text = if unicode {
            match decode_utf16(raw, file.is_little_endian()) {
                Some(b) => b,
                None => continue,
            }
        } else {
            // 8-bit storage is the literal byte sequence; render it only when it is
            // UTF-8 (the printer would cut an invalid one short).
            match std::str::from_utf8(raw) {
                Ok(_) => raw.to_vec(),
                Err(_) => continue,
            }
        };
        out.cfstrings.push(CFStringFact { addr: entry, bytes: text, size: stride as u32 });
        // A CFString constant is immutable; marking the struct read-only lets the
        // printer's constant-string route fire even on the historical writable
        // `__DATA,__cfstring` layout (it is already read-only in `__DATA_CONST`).
        out.readonly.push((entry, entry.wrapping_add(stride)));
    }
    out
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

    #[test]
    fn an_8bit_entry_that_is_not_utf8_is_skipped() {
        let mut bytes = fixture();
        let at = bytes.windows(14).position(|w| w == b"cfstring hello").unwrap();
        bytes[at + 3] = 0xff;
        assert_eq!(texts(&bytes), ["a constant string longer than one entry", "after the long one"]);
    }
}
