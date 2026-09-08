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
//!
//! # UTF-8
//!
//! [`super::is_string_char`] is the ASCII recognizer, so every byte `>= 0x80`
//! ends a run and a literal that opens with a non-ASCII character is reported
//! starting after its last multi-byte sequence — an address nothing in the image
//! refers to, which is why such a row also arrives with no references and no
//! owning function. [`scan_utf8_runs`](super::kuna_utf8strings::scan_utf8_runs)
//! is the same 1-byte matcher with well-formed UTF-8 sequences admitted as
//! characters, and it is a strict superset of the ASCII one: no accepted
//! sequence can swallow a byte the ASCII matcher would have taken. So the two
//! never run together — [`Query::utf8`] chooses which reading answers the 1-byte
//! width — and each row is labelled by what its bytes actually hold, so a row
//! with no multi-byte content is reported as [`Encoding::Ascii`] either way and
//! a pure-ASCII image reads identically under both.

use object::read::{Object, ObjectSection, ObjectSegment};

use super::kuna_utf8strings::scan_utf8_runs;
use super::kuna_widestrings::scan_utf16_runs;
use super::{is_loaded_initialized, scan_runs, Run};

pub use super::Termination;

/// The character width a row was found at.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Encoding {
    /// 1-byte characters — [`super::StringLiteralPass`]'s own width.
    Ascii,
    /// 1-byte characters plus multi-byte UTF-8 sequences
    /// (`kuna_utf8strings::scan_utf8_runs`). Only a row that actually holds a
    /// multi-byte sequence is reported at this width; a pure-ASCII row found by
    /// the same scan is [`Encoding::Ascii`].
    Utf8,
    /// 2-byte little-endian code units (`kuna_widestrings::scan_utf16_run`).
    Utf16,
}

impl Encoding {
    /// The wire name (`encoding` in the JSON surface).
    pub fn as_str(self) -> &'static str {
        match self {
            Encoding::Ascii => "ascii",
            Encoding::Utf8 => "utf8",
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
    /// Read the 1-byte width as UTF-8 rather than as ASCII, so a multi-byte
    /// sequence is a character instead of a terminator. Superset of [`ascii`]
    /// and only meaningful with it: the two readings never both run.
    ///
    /// [`ascii`]: Query::ascii
    pub utf8: bool,
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

/// Read a UTF-8 run back as text. Every character was validated by
/// [`scan_utf8_runs`], so the decode cannot fail; the lossy form is the
/// no-panic spelling of that, not a second policy.
fn utf8_text(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
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
    // One reading answers the 1-byte width. The UTF-8 one is a superset of the
    // ASCII one — every ASCII run is a subrange of a UTF-8 run — so running both
    // would report the same text twice, once at a truncated address.
    if q.ascii && q.utf8 {
        runs.extend(
            regions
                .iter()
                .flat_map(|r| scan_utf8_runs(r.data, r.vma, q.min_len, q.termination))
                .map(|r| (r, Encoding::Utf8)),
        );
    } else if q.ascii {
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
            Encoding::Utf8 => utf8_text(bytes),
            Encoding::Utf16 => utf16_text(bytes),
        };
        // A UTF-8 run that holds no multi-byte sequence is a row the ASCII
        // reading found too, so it is reported at that width and the label keeps
        // meaning "what is in these bytes".
        let encoding = match encoding {
            Encoding::Utf8 if bytes.is_ascii() => Encoding::Ascii,
            other => other,
        };
        // The extent an xref may land in stops at the last visible byte when
        // nothing terminated the run.
        let terminator = match (run.nul_terminated, encoding) {
            (false, _) => 0,
            (true, Encoding::Ascii | Encoding::Utf8) => 1,
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
            utf8: false,
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
                utf8: false,
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

    /// The reading that keeps a multi-byte prefix, at the inventory level: the
    /// vendored `prompt` at 0x101000 opens with a fullwidth low line, so the
    /// ASCII reading reports it from 0x10100c — twelve bytes past the address
    /// `prompt_user` actually loads.
    #[test]
    fn the_utf8_reading_recovers_a_multi_byte_prefix() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/utf8prompt_x86_64");
        let bytes = std::fs::read(path).expect("read utf8prompt_x86_64 fixture");
        let file = object::File::parse(bytes.as_slice()).expect("parse utf8prompt_x86_64");
        let q = |utf8: bool| Query {
            min_len: 5,
            ascii: true,
            utf8,
            utf16: false,
            section: None,
            termination: Termination::Any,
        };

        let ascii = inventory(&file, &q(false));
        let truncated =
            ascii.strings.iter().find(|s| s.addr == 0x10100c).expect("the ASCII reading");
        assert_eq!(truncated.char_len, 43);
        assert_eq!(truncated.encoding, Encoding::Ascii);
        assert!(!ascii.strings.iter().any(|s| s.addr == 0x101000));

        let utf8 = inventory(&file, &q(true));
        let whole = utf8.strings.iter().find(|s| s.addr == 0x101000).expect("the UTF-8 reading");
        assert_eq!(whole.encoding, Encoding::Utf8);
        assert_eq!(whole.char_len, 50, "fifty characters");
        assert_eq!(whole.byte_len, 56, "in fifty-five bytes plus the NUL");
        assert!(whole.text.starts_with('\u{ff3f}'));
        assert!(whole.text.ends_with("magical keycombination? "));
        assert!(
            !utf8.strings.iter().any(|s| s.addr == 0x10100c),
            "one reading answers the 1-byte width, so the truncated row is gone"
        );

        // The ASCII-only control literal is the same row under both readings.
        let control = |inv: &Inventory| {
            inv.strings.iter().find(|s| s.addr == 0x101038).cloned().expect("the control literal")
        };
        assert_eq!(control(&ascii), control(&utf8));
        assert_eq!(control(&ascii).encoding, Encoding::Ascii);
    }

    /// The property the one-scan-or-the-other choice rests on: every row the
    /// ASCII reading finds lies inside a row the UTF-8 reading finds, so nothing
    /// is lost by not running both. Checked over both endings on an image that
    /// holds multi-byte sequences and on one that holds none.
    #[test]
    fn the_utf8_reading_is_a_superset_of_the_ascii_one() {
        for name in ["utf8prompt_x86_64", "fauxware"] {
            let path =
                std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures"))
                    .join(name);
            let bytes = std::fs::read(&path).unwrap_or_else(|e| panic!("read {name}: {e}"));
            let file = object::File::parse(bytes.as_slice()).expect("parse the fixture");
            for termination in [Termination::Nul, Termination::Any] {
                let q = |utf8: bool| Query {
                    min_len: 5,
                    ascii: true,
                    utf8,
                    utf16: false,
                    section: None,
                    termination,
                };
                let ascii = inventory(&file, &q(false));
                let utf8 = inventory(&file, &q(true));
                for row in &ascii.strings {
                    let end = row.addr + u64::from(row.byte_len);
                    assert!(
                        utf8.strings.iter().any(|w| {
                            w.addr <= row.addr && w.addr + u64::from(w.byte_len) >= end
                        }),
                        "{name} {termination:?}: 0x{:x} {:?} is covered by no UTF-8 row",
                        row.addr,
                        row.text
                    );
                }
            }
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
