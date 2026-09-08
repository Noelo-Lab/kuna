//! (kuna) The string **inventory** query behind `kuna strings` — the reporting
//! face of the [`StringLiteralPass`](super::StringLiteralPass) that already runs
//! at load.
//!
//! # Why this is not a second scanner
//!
//! The analyzer tier's job is to *mark up* strings: it emits a
//! [`StringFact`] per literal so the commit seam can plant a typelocked
//! `char[N]`, and the printer renders `puts("Username: ")`. It deliberately keeps
//! nothing an analyst would ask for — no text, no section, no encoding — because
//! nothing downstream of the commit needed it.
//!
//! So this module answers those questions **over the pass's own matcher**:
//! [`super::scan_runs`] over the pass's own sections (and, on an image with no
//! sections, over its PT_LOAD segments). What is added is presentation — reading
//! the bytes back as text, attributing each to its section — plus the two things
//! the pass documents as out of scope:
//!
//! # Termination
//!
//! The pass is [`Termination::Nul`] because it plants a `char[N]`, and under that
//! policy every row here is a fact the engine also marked up. An *inventory* is a
//! different question: a length-prefixed name table
//! (`\x0cout.js\x06std\x12_0x8ec6b3`, the shape a bundled JS or bytecode payload
//! carries) holds no NUL at all, so require-NUL-end reports zero strings for a
//! region an analyst can read straight out of a hex dump. [`Termination::Any`] is
//! `strings(1)`'s rule over the same address set, and every row says which ending
//! it had ([`FoundString::nul_terminated`]) so a C literal is still
//! distinguishable from a printable fragment.
//!
//! # UTF-16
//!
//! [`super::StringLiteralPass`] is the 1-byte (`allCharWidths = false`) arm of
//! Ghidra's `StringsAnalyzer` and says so: "UTF-16/32 is a documented seam,
//! skipped". Read as ASCII, a UTF-16LE literal is a one-character string — the
//! `L"ntdll.dll"` argument of `LoadLibraryW` stops at the NUL after `n`. So
//! [`scan_utf16_run`](super::kuna_widestrings::scan_utf16_run) is the missing
//! width, mirroring the 1-byte matcher exactly:
//! the same [`super::is_string_char`] recognizer, the same require-NUL-end rule,
//! the same minimum length — over 2-byte units instead of bytes. It is a widened
//! matcher, not a different one, and it is reported as its own `encoding` so a
//! consumer always knows which width found a row.
//!
//! Scope: UTF-16**LE** whose code units are all in the 1-byte recognizer's
//! charset (the Windows-API case). A big-endian or non-Latin UTF-16 literal is
//! not recovered.

use object::read::{Object, ObjectSection, ObjectSegment};

use super::kuna_widestrings::scan_utf16_runs;
use super::{is_loaded_initialized, scan_runs, Run};

pub use super::Termination;

/// The character width a row was found at.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Encoding {
    /// 1-byte characters — [`super::StringLiteralPass`]'s own width.
    Ascii,
    /// 2-byte little-endian code units (`kuna_widestrings::scan_utf16_run`).
    Utf16,
}

impl Encoding {
    /// The wire name (`encoding` in the JSON surface).
    pub fn as_str(self) -> &'static str {
        match self {
            Encoding::Ascii => "ascii",
            Encoding::Utf16 => "utf16",
        }
    }
}

/// One recovered string literal, at the address it is mapped to.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FoundString {
    /// VMA of the first character byte.
    pub addr: u64,
    /// The decoded text, terminator excluded.
    pub text: String,
    /// Visible character count (`text.chars().count()`, so a UTF-16 row counts
    /// code units, not the bytes they occupy).
    pub char_len: usize,
    /// Bytes occupied including the terminator — the extent an xref can land in.
    /// An unterminated run occupies exactly its visible bytes.
    pub byte_len: u32,
    /// The run ended at a NUL, so it is a C string and not merely printable text.
    /// Always true under [`Termination::Nul`].
    pub nul_terminated: bool,
    /// Which width found it.
    pub encoding: Encoding,
    /// The section it lives in, or `None` on an image scanned by segment.
    pub section: Option<String>,
}

/// What to look for.
pub struct Query {
    /// Minimum visible length, in characters (`StringsAnalyzer.minStringLength`;
    /// the pass default is 5).
    pub min_len: usize,
    /// Scan at 1-byte width (the pass itself).
    pub ascii: bool,
    /// Scan at 2-byte little-endian width.
    pub utf16: bool,
    /// Restrict to this section, by name (a leading `.` is optional).
    pub section: Option<String>,
    /// Which run endings count as a string.
    pub termination: Termination,
}

/// The answer: the rows, plus enough about the scan for a caller to explain an
/// empty one.
pub struct Inventory {
    /// Every match, ordered by address then encoding.
    pub strings: Vec<FoundString>,
    /// The names of the regions that were scannable at all, in image order — the
    /// candidate set a `--section` filter picks from.
    pub regions: Vec<String>,
    /// The image had no loaded+initialized *section*, so the scan fell back to
    /// its PT_LOAD segments (a UPX-packed or section-header-stripped image).
    pub from_segments: bool,
}

/// A contiguous run of mapped, initialized bytes: a section, or — when the image
/// has no usable section table — a loadable segment.
struct Region<'d> {
    name: Option<String>,
    vma: u64,
    data: &'d [u8],
}

impl Region<'_> {
    /// The bytes of `[addr, addr+len)`, if that range lies inside this region.
    fn slice(&self, addr: u64, len: usize) -> Option<&[u8]> {
        let off = addr.checked_sub(self.vma)? as usize;
        self.data.get(off..off.checked_add(len)?)
    }
}

/// The loaded+initialized sections, using the pass's own membership test so the
/// scanned address set is exactly the one the analyzer marks up.
fn section_regions<'d>(file: &'d object::File<'d>) -> Vec<Region<'d>> {
    let mut out = Vec::new();
    for sec in file.sections() {
        if !is_loaded_initialized(&sec) {
            continue;
        }
        let data = match sec.data() {
            Ok(d) if !d.is_empty() => d,
            _ => continue,
        };
        let name = sec.name().ok().filter(|n| !n.is_empty()).map(str::to_string);
        out.push(Region { name, vma: sec.address(), data });
    }
    out
}

/// The PT_LOAD segments — the fallback address set for an image whose section
/// headers are gone (a UPX-packed ELF keeps its program headers and nothing
/// else), which is precisely when an analyst reaches for a string dump.
fn segment_regions<'d>(file: &'d object::File<'d>) -> Vec<Region<'d>> {
    let mut out = Vec::new();
    for seg in file.segments() {
        let data = match seg.data() {
            Ok(d) if !d.is_empty() => d,
            _ => continue,
        };
        out.push(Region { name: None, vma: seg.address(), data });
    }
    out
}

/// The region holding `addr`, if any.
fn region_of<'a, 'd>(regions: &'a [Region<'d>], addr: u64) -> Option<&'a Region<'d>> {
    regions.iter().find(|r| addr >= r.vma && addr - r.vma < r.data.len() as u64)
}

/// Does `name` (a region's own name) satisfy a `--section` operand? The leading
/// `.` is optional, so `--section rdata` finds `.rdata`.
fn section_matches(name: Option<&str>, want: &str) -> bool {
    let want = want.trim();
    match name {
        Some(n) => n == want || n.strip_prefix('.') == Some(want),
        None => false,
    }
}

/// Read a 1-byte fact back as text. Every byte is in the recognizer's charset,
/// so the decode cannot fail.
fn ascii_text(bytes: &[u8]) -> String {
    bytes.iter().map(|&b| b as char).collect()
}

/// Read a 2-byte fact back as text (low bytes only — the units are all in the
/// 1-byte charset by construction).
fn utf16_text(bytes: &[u8]) -> String {
    bytes.chunks_exact(2).map(|u| u[0] as char).collect()
}

/// Enumerate the image's string literals.
///
/// The ASCII half is [`super::StringLiteralPass`]'s own matcher and the UTF-16
/// half the widened one, both over the pass's own address set, so under
/// [`Termination::Nul`] every row is a fact the engine also marked up. Nothing is
/// committed either way: this is a read-only query over the parsed object, so no
/// invocation of the engine can change because of it.
pub fn inventory(file: &object::File, q: &Query) -> Inventory {
    let sections = section_regions(file);
    let from_segments = sections.is_empty();
    let regions = if from_segments { segment_regions(file) } else { sections };

    let mut runs: Vec<(Run, Encoding)> = Vec::new();
    if q.ascii {
        runs.extend(
            regions
                .iter()
                .flat_map(|r| scan_runs(r.data, r.vma, q.min_len, q.termination))
                .map(|r| (r, Encoding::Ascii)),
        );
    }
    if q.utf16 {
        runs.extend(
            regions
                .iter()
                .flat_map(|r| scan_utf16_runs(r.data, r.vma, q.min_len, q.termination))
                .map(|r| (r, Encoding::Utf16)),
        );
    }

    let mut strings: Vec<FoundString> = Vec::new();
    for (run, encoding) in runs {
        let Some(region) = region_of(&regions, run.addr) else {
            continue;
        };
        if let Some(want) = &q.section {
            if !section_matches(region.name.as_deref(), want) {
                continue;
            }
        }
        let Some(bytes) = region.slice(run.addr, run.visible_len) else {
            continue;
        };
        let text = match encoding {
            Encoding::Ascii => ascii_text(bytes),
            Encoding::Utf16 => utf16_text(bytes),
        };
        // The extent an xref may land in stops at the last visible byte when
        // nothing terminated the run.
        let terminator = match (run.nul_terminated, encoding) {
            (false, _) => 0,
            (true, Encoding::Ascii) => 1,
            (true, Encoding::Utf16) => 2,
        };
        strings.push(FoundString {
            addr: run.addr,
            char_len: text.chars().count(),
            text,
            byte_len: (run.visible_len + terminator) as u32,
            nul_terminated: run.nul_terminated,
            encoding,
            section: region.name.clone(),
        });
    }
    strings.sort_by(|a, b| (a.addr, a.encoding).cmp(&(b.addr, b.encoding)));

    let names = regions.iter().filter_map(|r| r.name.clone()).collect();
    Inventory { strings, regions: names, from_segments }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pass::StringFact;
    use crate::strings::kuna_widestrings::scan_utf16_run;
    use crate::strings::{scan_run, scan_strings};

    /// The pass-faithful query, so a test says which policy it is asserting.
    fn nul(min_len: usize, ascii: bool, utf16: bool, section: Option<&str>) -> Query {
        Query {
            min_len,
            ascii,
            utf16,
            section: section.map(str::to_string),
            termination: Termination::Nul,
        }
    }

    #[test]
    fn utf16_matcher_mirrors_the_ascii_one() {
        // "ntdll.dll" as UTF-16LE, NUL-terminated: 9 units + the 0x0000.
        let mut data = Vec::new();
        for ch in "ntdll.dll".chars() {
            data.push(ch as u8);
            data.push(0);
        }
        data.extend_from_slice(&[0, 0]);
        let out = scan_utf16_run(&data, 0x1000, 5);
        assert_eq!(out, vec![StringFact { addr: 0x1000, len: 20 }]);

        // The same bytes read at 1-byte width are a one-character run, which the
        // pass rejects outright — this is the defect the width exists to fix.
        assert!(scan_run(&data, 0x1000, 5).is_empty());
    }

    #[test]
    fn utf16_requires_a_null_end_and_the_minimum() {
        // 4 units + terminator: below the minimum.
        let short = b"a\0b\0c\0d\0\0\0";
        assert!(scan_utf16_run(short, 0, 5).is_empty());
        // 6 units closed by a non-NUL unit: rejected, exactly as requireNullEnd
        // rejects it at 1-byte width.
        let unterminated = b"a\0b\0c\0d\0e\0f\0\x01\x02";
        assert!(scan_utf16_run(unterminated, 0, 5).is_empty());
    }

    #[test]
    fn utf16_units_stay_on_even_addresses() {
        // A region mapped at an odd VMA reads its units from the odd file offset,
        // so `hello` still starts on an even address.
        let mut data = vec![0xffu8];
        for ch in "hello".chars() {
            data.push(ch as u8);
            data.push(0);
        }
        data.extend_from_slice(&[0, 0]);
        let out = scan_utf16_run(&data, 0x1001, 5);
        assert_eq!(out, vec![StringFact { addr: 0x1002, len: 12 }]);
    }

    #[test]
    fn a_length_prefixed_name_table_needs_the_relaxed_ending() {
        // The reported shape: each name preceded by its own length tag, no NUL
        // anywhere. `strings(1)` reads all three; require-NUL-end reads none.
        let data = b"\x0cout.js\x06std\x12_0x8ec6b3\x11".to_vec();
        let nul_runs = scan_runs(&data, 0x1000, 4, Termination::Nul);
        assert!(nul_runs.is_empty(), "no run here ends at a NUL: {nul_runs:?}");

        let any_runs = scan_runs(&data, 0x1000, 4, Termination::Any);
        let texts: Vec<String> = any_runs
            .iter()
            .map(|r| ascii_text(&data[(r.addr - 0x1000) as usize..][..r.visible_len]))
            .collect();
        assert_eq!(texts, vec!["out.js", "_0x8ec6b3"], "\"std\" is below the minimum");
        assert!(any_runs.iter().all(|r| !r.nul_terminated), "none of them is a C string");
    }

    #[test]
    fn a_run_reaching_the_end_of_a_region_is_a_text_run_only() {
        let data = b"trailing".to_vec();
        assert!(scan_runs(&data, 0, 4, Termination::Nul).is_empty());
        assert_eq!(
            scan_runs(&data, 0, 4, Termination::Any),
            vec![Run { addr: 0, visible_len: 8, nul_terminated: false }]
        );
    }

    #[test]
    fn the_relaxed_ending_keeps_every_terminated_row_intact() {
        // Relaxing the ending only ADDS rows: a NUL-terminated literal keeps its
        // address, its text and its terminator-inclusive extent.
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/fauxware");
        let bytes = std::fs::read(path).expect("read fauxware fixture");
        let file = object::File::parse(bytes.as_slice()).expect("parse fauxware");
        let strict = inventory(&file, &nul(5, true, false, None));
        let relaxed = inventory(
            &file,
            &Query {
                min_len: 5,
                ascii: true,
                utf16: false,
                section: None,
                termination: Termination::Any,
            },
        );
        for row in &strict.strings {
            assert!(row.nul_terminated, "the strict policy admits nothing else");
            assert!(
                relaxed.strings.contains(row),
                "0x{:x} {:?} was dropped by the relaxed ending",
                row.addr,
                row.text
            );
        }
        assert!(
            relaxed.strings.len() > strict.strings.len(),
            "fauxware has an unterminated printable run"
        );
        assert!(relaxed.strings.iter().any(|r| !r.nul_terminated));
    }

    #[test]
    fn the_wide_width_takes_the_same_ending_policy() {
        // "abcde" as UTF-16LE, closed by a non-zero unit instead of 0x0000.
        let mut data = Vec::new();
        for ch in "abcde".chars() {
            data.push(ch as u8);
            data.push(0);
        }
        data.extend_from_slice(&[0x01, 0x02]);
        assert!(scan_utf16_run(&data, 0, 5).is_empty());
        assert_eq!(
            scan_utf16_runs(&data, 0, 5, Termination::Any),
            vec![Run { addr: 0, visible_len: 10, nul_terminated: false }]
        );
    }

    #[test]
    fn section_operand_tolerates_a_missing_dot() {
        assert!(section_matches(Some(".rdata"), "rdata"));
        assert!(section_matches(Some(".rdata"), ".rdata"));
        assert!(!section_matches(Some(".rdata"), "data"));
        assert!(!section_matches(None, "rdata"));
    }

    #[test]
    fn inventory_over_fauxware_reads_the_pass_back_as_text() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/fauxware");
        let bytes = std::fs::read(path).expect("read fauxware fixture");
        let file = object::File::parse(bytes.as_slice()).expect("parse fauxware");
        let inv = inventory(&file, &nul(5, true, false, None));
        let row = inv
            .strings
            .iter()
            .find(|s| s.addr == 0x400915)
            .expect("the pass's own \"Username: \" fact must be reported");
        assert_eq!(row.text, "Username: ");
        assert_eq!(row.char_len, 10);
        assert_eq!(row.byte_len, 11);
        assert_eq!(row.encoding, Encoding::Ascii);
        assert_eq!(row.section.as_deref(), Some(".rodata"));
        assert!(!inv.from_segments);

        // Every ASCII row is one the pass itself emitted — no row is invented here.
        let facts = scan_strings(&file, 5);
        for row in inv.strings.iter().filter(|s| s.encoding == Encoding::Ascii) {
            assert!(
                facts.contains(&StringFact { addr: row.addr, len: row.byte_len }),
                "row 0x{:x} is not a StringLiteralPass fact",
                row.addr
            );
        }
    }

    #[test]
    fn a_section_filter_narrows_the_inventory() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/fauxware");
        let bytes = std::fs::read(path).expect("read fauxware fixture");
        let file = object::File::parse(bytes.as_slice()).expect("parse fauxware");
        let inv = inventory(&file, &nul(5, true, false, Some("rodata")));
        assert!(!inv.strings.is_empty(), "fauxware has .rodata strings");
        assert!(inv.strings.iter().all(|s| s.section.as_deref() == Some(".rodata")));
        assert!(inv.regions.iter().any(|n| n == ".rodata"));
    }
}
