//! (kuna `fatstr`) Rust `&str` RECORD recognition — length-bounded string
//! facts recovered from static `(ptr, len)` records (`docs/multilang.md` W4,
//! the analysis-side half of the `fatstr` option).
//!
//! # Why the NUL scan cannot see these
//!
//! Rust string literals carry their length in the `&str` fat pointer, not a
//! terminator, and rustc packs them back to back in `.rodata`. The NUL-run
//! scan ([`super::StringLiteralPass`]) therefore GLUES adjacent literals into
//! one wrong row: the measured `strops` witness renders its three literals as
//! a single `"hello multilang, !world to decompile \n"`. The lengths are not
//! lost, though — every `format!`/`&str` static materializes a `(ptr, len)`
//! RECORD (the `&[&str]` pieces arrays of `fmt::Arguments`, string constants
//! promoted to statics), and those records are the segmentation oracle.
//!
//! # Where the records live, and the two arms
//!
//! - **PIE (the common rustc default)**: the pointer halves are relocated, so
//!   the records sit in `.data.rel.ro` as `R_*_RELATIVE` entries whose addend
//!   is the string's address, with the LENGTH in the raw image qword right
//!   after the relocated slot (the measured witness: 458 such entries). The
//!   reloc arm walks `dynamic_relocations()`: a RELATIVE entry at `off` whose
//!   addend lands in read-only data, whose companion qword at `off + 8` is
//!   itself unrelocated and a sane length, and whose target bytes are UTF-8
//!   text, is a record.
//! - **Non-PIE / static**: the pointers are absolute in the file bytes, so the
//!   direct arm scans read-only data for qword `(ptr, len)` pairs under the
//!   same target tests, with a stricter minimum length (4, the NUL scan's own
//!   minimum) because no relocation vouches for the slot.
//!
//! Both arms demand: target in read-only data, `1..=4096` bytes (the engine
//! `fatstr` rule's own bound), valid UTF-8 whose characters are >=90%
//! printable-or-whitespace. The facts flow out as `bounded_strings` — exact
//! byte lengths, NO terminator — and the commit arm plants them BEFORE the
//! NUL-run stream, so its occupied-address guard retires the glued rows.
//!
//! # Gating
//!
//! One option, two recognizers: this pass is registered only for rustc- and
//! Go-detected images (`passes_for`), and its commit is gated on the same
//! `Architecture::fat_str` mode the engine rule reads (`off` drops the facts
//! at the read-symbols boundary, exactly like every analysis gate).
//!
//! # The Go arm
//!
//! Go string headers are the same `(ptr, len)` record over the same
//! unterminated `.rodata` blob, with one placement difference: the measured
//! go1.22 toolchain binaries are non-PIE and keep most headers in the
//! WRITABLE data image (`.data`: 480 of 545 records on the hello witness;
//! `.noptrdata` holds the pointer-free statics), with the remaining 65 in
//! `.rodata` itself. A Go registration therefore widens the SLOT regions to
//! those two sections by name ([`StrRecordPass::go`]); the TARGET test is
//! unchanged — the text must sit in genuinely read-only data — which is what
//! keeps a writable slot's mutability irrelevant (the record may move; the
//! literal it currently states cannot). Go also shares literal bytes between
//! headers (substring records): records are deduped longest-first per start
//! address, and a record starting inside an earlier one is retired by the
//! commit arm's occupied-address guard, so prefix/suffix sharing renders
//! through the containing literal.

use object::read::{Object, ObjectSection};
use object::RelocationFlags;

use crate::pass::{AnalysisCtx, AnalysisOutput, AnalysisPass, Phase, StringFact};

/// The engine `fatstr` rule's length ceiling, shared here.
const MAX_LEN: u64 = 4096;
/// Direct-arm minimum (no relocation vouches for the slot): the NUL scan's own.
const DIRECT_MIN_LEN: u64 = 4;
/// Safety cap per image, far above the measured populations (hundreds).
const MAX_RECORDS: usize = 65536;

/// A read-only data region `[vma, vma+len)` with its raw file bytes.
pub(crate) struct RoRegion<'a> {
    pub(crate) vma: u64,
    pub(crate) data: &'a [u8],
}

impl RoRegion<'_> {
    pub(crate) fn contains(&self, p: u64) -> bool {
        p >= self.vma && p < self.vma + self.data.len() as u64
    }
    pub(crate) fn bytes(&self, p: u64, n: u64) -> Option<&[u8]> {
        if !self.contains(p) || p + n > self.vma + self.data.len() as u64 {
            return None;
        }
        let off = (p - self.vma) as usize;
        Some(&self.data[off..off + n as usize])
    }
}

/// The sections whose bytes may hold string text or records: genuinely
/// read-only data, plus `.data.rel.ro` (read-only after relocation, where a
/// PIE keeps its relocated record slots).
pub(crate) fn ro_regions<'a>(file: &'a object::File) -> Vec<RoRegion<'a>> {
    file.sections()
        .filter(|s| {
            matches!(
                s.kind(),
                object::SectionKind::ReadOnlyData | object::SectionKind::ReadOnlyString
            ) || s.name().is_ok_and(|n| n == ".data.rel.ro")
        })
        .filter_map(|s| Some(RoRegion { vma: s.address(), data: s.data().ok()? }))
        .collect()
}

/// The additional SLOT regions a Go image keeps headers in: the writable data
/// image. Slots only — [`record_at`] still demands the target bytes in RO.
fn go_slot_regions<'a>(file: &'a object::File) -> Vec<RoRegion<'a>> {
    file.sections()
        .filter(|s| s.name().is_ok_and(|n| n == ".data" || n == ".noptrdata"))
        .filter_map(|s| Some(RoRegion { vma: s.address(), data: s.data().ok()? }))
        .collect()
}

/// Is this record text? Valid UTF-8 whose chars are >=90% printable or
/// whitespace — the engine rule's text-evidence test, applied to exact bytes.
pub(crate) fn is_text(bytes: &[u8]) -> bool {
    let Ok(s) = core::str::from_utf8(bytes) else { return false };
    let total = s.chars().count();
    if total == 0 {
        return false;
    }
    let good = s.chars().filter(|c| !c.is_control() || c.is_whitespace()).count();
    good * 10 >= total * 9
}

/// A validated record: `len` bytes of text at `ptr` inside read-only data.
fn record_at(regions: &[RoRegion], ptr: u64, len: u64, min_len: u64) -> Option<StringFact> {
    if len < min_len || len > MAX_LEN {
        return None;
    }
    let region = regions.iter().find(|r| r.contains(ptr))?;
    let bytes = region.bytes(ptr, len)?;
    if !is_text(bytes) {
        return None;
    }
    Some(StringFact { addr: ptr, len: len as u32 })
}

/// Read a little-endian pointer-width word.
pub(crate) fn read_word(bytes: &[u8]) -> u64 {
    match bytes.len() {
        8 => u64::from_le_bytes(bytes.try_into().expect("8 bytes")),
        4 => u32::from_le_bytes(bytes.try_into().expect("4 bytes")) as u64,
        _ => 0,
    }
}

/// The reloc arm: every `R_*_RELATIVE` whose slot sits in read-only data and
/// whose companion `len` word is unrelocated raw bytes.
fn scan_reloc_records(file: &object::File, regions: &[RoRegion], width: u64) -> Vec<StringFact> {
    let mut out = Vec::new();
    let Some((r_relative, _, _)) =
        crate::loader::kuna_dynrelocs::reloc_triple(file.architecture())
    else {
        return out;
    };
    let relocs: Vec<(u64, object::Relocation)> =
        file.dynamic_relocations().into_iter().flatten().collect();
    if relocs.is_empty() {
        return out;
    }
    let slots: std::collections::HashSet<u64> = relocs.iter().map(|(off, _)| *off).collect();
    for (off, rel) in &relocs {
        let RelocationFlags::Elf { r_type } = rel.flags() else { continue };
        if r_type != r_relative {
            continue;
        }
        // The slot itself and its companion length word both sit in RO data,
        // and the length slot is NOT relocated (a relocated word is a pointer,
        // and (ptr, ptr) is a record of something else).
        if slots.contains(&(off + width)) {
            continue;
        }
        let Some(region) = regions.iter().find(|r| r.contains(*off)) else { continue };
        let Some(len_bytes) = region.bytes(off + width, width) else { continue };
        let len = read_word(len_bytes);
        let target = rel.addend() as u64;
        if let Some(fact) = record_at(regions, target, len, 1) {
            out.push(fact);
            if out.len() >= MAX_RECORDS {
                break;
            }
        }
    }
    out
}

/// The direct arm: absolute `(ptr, len)` word pairs whose slots sit in
/// `slots` and whose targets sit in `targets` (the non-PIE layout; on a PIE
/// the unrelocated slots read zero and match nothing).
fn scan_direct_records(slots: &[RoRegion], targets: &[RoRegion], width: u64) -> Vec<StringFact> {
    let mut out = Vec::new();
    let w = width as usize;
    for region in slots {
        let data = region.data;
        let mut i = 0usize;
        while i + 2 * w <= data.len() {
            let ptr = read_word(&data[i..i + w]);
            let len = read_word(&data[i + w..i + 2 * w]);
            if let Some(fact) = record_at(targets, ptr, len, DIRECT_MIN_LEN) {
                out.push(fact);
                if out.len() >= MAX_RECORDS {
                    return out;
                }
            }
            i += w;
        }
    }
    out
}

/// (kuna `fatstr`) The string-record pass. Registered for rustc- and
/// Go-detected images; committed under the `fatstr` mode gate.
pub struct StrRecordPass {
    /// Widen the direct arm's SLOT regions to the Go data image
    /// ([`go_slot_regions`]); targets stay read-only either way.
    go_slots: bool,
}

impl StrRecordPass {
    /// The rustc registration: slots in read-only data only.
    pub fn rust() -> StrRecordPass {
        StrRecordPass { go_slots: false }
    }
    /// The Go registration: slots also in `.data`/`.noptrdata`.
    pub fn go() -> StrRecordPass {
        StrRecordPass { go_slots: true }
    }
}

impl AnalysisPass for StrRecordPass {
    fn phase(&self) -> Phase {
        Phase::P1
    }
    fn id(&self) -> &'static str {
        "fatstrrecords"
    }
    fn run(&self, ctx: &AnalysisCtx) -> AnalysisOutput {
        let regions = ro_regions(ctx.file);
        if regions.is_empty() {
            return AnalysisOutput::default();
        }
        let width: u64 = if ctx.file.is_64() { 8 } else { 4 };
        let mut facts = scan_reloc_records(ctx.file, &regions, width);
        if self.go_slots {
            let slots = go_slot_regions(ctx.file);
            facts.extend(scan_direct_records(&slots, &regions, width));
        }
        facts.extend(scan_direct_records(&regions, &regions, width));
        // One fact per start address, LONGEST first: Go headers share bytes
        // (prefix records at one start), and the longer record renders the
        // shorter as a prefix read while the reverse would truncate. A record
        // starting inside an earlier one is left to the commit arm's
        // occupied-address guard.
        facts.sort_by_key(|f| (f.addr, core::cmp::Reverse(f.len)));
        facts.dedup_by_key(|f| f.addr);
        AnalysisOutput { bounded_strings: facts, ..Default::default() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn region(vma: u64, data: &[u8]) -> RoRegion<'_> {
        RoRegion { vma, data }
    }

    #[test]
    fn text_test_accepts_utf8_rejects_binary() {
        assert!(is_text(b"hello multilang, "));
        assert!(is_text("héllo — wörld".as_bytes()));
        assert!(is_text(b"!"));
        assert!(is_text(b"line\nbreak\tok "));
        assert!(!is_text(&[0xff, 0xfe, 0x00, 0x41]));
        assert!(!is_text(&[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0b][..]));
        assert!(!is_text(b""));
    }

    #[test]
    fn record_validation_bounds_and_backing() {
        let text = region(0x1000, b"hello multilang, !world to decompile \n\0junk");
        let regions = vec![text];
        // Exact record: 17 bytes of the glued run.
        let f = record_at(&regions, 0x1000, 17, 1).expect("17-byte record");
        assert_eq!((f.addr, f.len), (0x1000, 17));
        // The 1-byte piece.
        assert!(record_at(&regions, 0x1011, 1, 1).is_some());
        // Out of region, zero, oversize, and spilling past the region all refuse.
        assert!(record_at(&regions, 0x2000, 4, 1).is_none());
        assert!(record_at(&regions, 0x1000, 0, 1).is_none());
        assert!(record_at(&regions, 0x1000, MAX_LEN + 1, 1).is_none());
        assert!(record_at(&regions, 0x1000, 4096, 1).is_none());
        // Direct-arm minimum: a 1-byte record needs a relocation to vouch.
        assert!(record_at(&regions, 0x1011, 1, DIRECT_MIN_LEN).is_none());
    }

    #[test]
    fn direct_arm_finds_absolute_records_and_dedups() {
        // A synthetic RO layout: strings at 0x2000, records at 0x2100.
        let mut data = vec![0u8; 0x200];
        data[..17].copy_from_slice(b"hello multilang, ");
        data[17] = b'!';
        let mut recs = Vec::new();
        for (p, l) in [(0x2000u64, 17u64), (0x2011, 1), (0x2000, 17)] {
            recs.extend_from_slice(&p.to_le_bytes());
            recs.extend_from_slice(&l.to_le_bytes());
        }
        data[0x100..0x100 + recs.len()].copy_from_slice(&recs);
        let regions = vec![region(0x2000, &data)];
        let found = scan_direct_records(&regions, &regions, 8);
        // The 17-byte record is found (once per slot it appears in); the 1-byte
        // record fails the direct-arm minimum. Dedup collapses the repeat.
        let mut facts = found;
        facts.sort_by_key(|f| f.addr);
        facts.dedup_by_key(|f| f.addr);
        assert_eq!(facts.len(), 1);
        assert_eq!((facts[0].addr, facts[0].len), (0x2000, 17));
    }

    /// The Go slot split: records whose SLOTS sit outside the target regions
    /// (the `.data` header area) are found when passed as the slot set, and
    /// the targets are still held to the read-only set.
    #[test]
    fn go_slots_scan_separately_from_targets() {
        let mut ro = vec![0u8; 0x40];
        ro[..17].copy_from_slice(b"hello multilang, ");
        let targets = vec![region(0x2000, &ro)];
        // Header area at 0x5000 ("data"): one record to the RO text, one
        // record whose target points back into the header area itself.
        let mut hdr = vec![0u8; 0x40];
        hdr[..8].copy_from_slice(&0x2000u64.to_le_bytes());
        hdr[8..16].copy_from_slice(&17u64.to_le_bytes());
        hdr[16..24].copy_from_slice(&0x5000u64.to_le_bytes());
        hdr[24..32].copy_from_slice(&17u64.to_le_bytes());
        let slots = vec![region(0x5000, &hdr)];
        let found = scan_direct_records(&slots, &targets, 8);
        assert_eq!(found.len(), 1, "only the RO-backed target is a record");
        assert_eq!((found[0].addr, found[0].len), (0x2000, 17));
    }

    /// Same-start records keep the LONGEST (Go prefix sharing): the shorter
    /// renders as a prefix read of the longer.
    #[test]
    fn same_start_dedup_keeps_the_longest() {
        let mut facts = vec![
            StringFact { addr: 0x2000, len: 5 },
            StringFact { addr: 0x2000, len: 17 },
            StringFact { addr: 0x2020, len: 4 },
        ];
        facts.sort_by_key(|f| (f.addr, core::cmp::Reverse(f.len)));
        facts.dedup_by_key(|f| f.addr);
        assert_eq!(facts.len(), 2);
        assert_eq!((facts[0].addr, facts[0].len), (0x2000, 17));
    }
}
