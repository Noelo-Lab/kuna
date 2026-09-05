//! (kuna `widestrings`) The 2-byte width of [`super::StringLiteralPass`].
//!
//! # The gap
//!
//! Ghidra's `StringsAnalyzer` has an `allCharWidths` setting; kuna ported only
//! its 1-byte arm and said so ("UTF-16/32 is a documented seam, skipped"). Read
//! at 1-byte width a UTF-16LE literal is a ONE-CHARACTER string: `L"ntdll.dll"`
//! is `6e 00 74 00 …`, whose first NUL closes the run after `n`. Nothing plants
//! data there (a 1-char run is below the minimum length), so the constant keeps
//! whatever `char *` the type lattice gave it and the printer reads to the same
//! NUL — which is why every wide Windows-API argument rendered as its own first
//! character: `LoadLibraryW("n")`, `FindWindowW("O",0)`. On an anti-debugging
//! routine that is the whole content of the check.
//!
//! # The width
//!
//! [`scan_utf16_run`] is the 1-byte `MinLengthCharSequenceMatcher` widened to
//! 2-byte little-endian code units, not a different matcher: the same
//! [`super::is_string_char`] recognizer over each unit's low byte, the same
//! require-NUL-end rule, the same minimum length. Units are read on the image's
//! even addresses, so a byte pair straddling two adjacent literals is never read
//! as one character.
//!
//! The two widths cannot both claim a run: a wide unit demands a zero high byte,
//! so five consecutive 1-byte-charset bytes never occur inside a wide run and a
//! wide run never starts inside an ASCII one. The commit boundary still applies
//! its `occupied` guard to each fact, and commits the ASCII facts first.
//!
//! Scope: UTF-16**LE** whose code units are all in the 1-byte recognizer's
//! charset — the Windows-API case. A big-endian or non-Latin UTF-16 literal is
//! not recovered, exactly as `kuna strings --encoding utf16` does not recover it.

use object::read::{Object, ObjectSection};

use crate::pass::StringFact;

use super::{is_loaded_initialized, is_string_char};

/// Mirror of the 1-byte `MinLengthCharSequenceMatcher` over 2-byte little-endian
/// code units: a run of units whose high byte is zero and whose low byte is in
/// [`super::is_string_char`], closed by a `0x0000` unit (require-NUL-end) and
/// emitted when it holds at least `min_len` units.
///
/// The emitted `len` is the BYTE span including the terminator (`(units+1) * 2`),
/// so the commit boundary derives the array element count as `len / 2`.
pub(crate) fn scan_utf16_run(data: &[u8], vma: u64, min_len: usize) -> Vec<StringFact> {
    let mut out = Vec::new();
    let mut i = (vma % 2) as usize;
    let mut run_start: Option<usize> = None;
    while i + 1 < data.len() {
        let (lo, hi) = (data[i], data[i + 1]);
        if hi == 0 && is_string_char(lo) {
            if run_start.is_none() {
                run_start = Some(i);
            }
            i += 2;
            continue;
        }
        if let Some(start) = run_start.take() {
            let units = (i - start) / 2;
            if lo == 0 && hi == 0 && units >= min_len {
                out.push(StringFact { addr: vma + start as u64, len: (units as u32 + 1) * 2 });
            }
        }
        i += 2;
    }
    out
}

/// The 2-byte analog of [`super::scan_strings`]: walk the loaded+initialized
/// sections and emit a [`StringFact`] per UTF-16LE literal, over exactly the
/// address set the ASCII half scans.
pub fn scan_wide_strings(file: &object::File, min_len: usize) -> Vec<StringFact> {
    let mut out = Vec::new();
    for sec in file.sections() {
        if !is_loaded_initialized(&sec) {
            continue;
        }
        let data = match sec.data() {
            Ok(d) if !d.is_empty() => d,
            _ => continue,
        };
        out.extend(scan_utf16_run(data, sec.address(), min_len));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::{scan_run, DEFAULT_MIN_LEN};

    #[test]
    fn utf16_run_is_the_ascii_matcher_widened() {
        // "ntdll.dll" as UTF-16LE, NUL-terminated: 9 units + the 0x0000.
        let mut data = Vec::new();
        for ch in "ntdll.dll".chars() {
            data.push(ch as u8);
            data.push(0);
        }
        data.extend_from_slice(&[0, 0]);
        assert_eq!(scan_utf16_run(&data, 0x1000, 5), vec![StringFact { addr: 0x1000, len: 20 }]);
        // The defect the width exists to fix: at 1-byte width the same bytes are a
        // one-character run, which the pass rejects outright.
        assert!(scan_run(&data, 0x1000, 5).is_empty());
    }

    #[test]
    fn short_and_unterminated_wide_runs_are_rejected() {
        let wide = |s: &str, term: bool| {
            let mut d = Vec::new();
            for ch in s.chars() {
                d.push(ch as u8);
                d.push(0);
            }
            if term {
                d.extend_from_slice(&[0, 0]);
            }
            d
        };
        // 4 units + terminator: below the minimum.
        assert!(scan_utf16_run(&wide("abcd", true), 0, 5).is_empty());
        // 9 units, no terminator: require-NUL-end rejects it.
        assert!(scan_utf16_run(&wide("ntdll.dll", false), 0, 5).is_empty());
        // Closed by a non-NUL out-of-charset unit: also rejected.
        let mut ended = wide("ntdll.dll", false);
        ended.extend_from_slice(&[0x01, 0x02]);
        assert!(scan_utf16_run(&ended, 0, 5).is_empty());
    }

    #[test]
    fn an_ascii_literal_is_never_read_as_a_wide_one() {
        // The disjointness the two widths rely on: a 1-byte literal's units have a
        // non-zero high byte, so no wide run starts inside it.
        let data = b"NtQueryInformationProcess\x00\x00\x00";
        assert!(scan_utf16_run(data, 0x2000, 5).is_empty());
        assert!(!scan_run(data, 0x2000, 5).is_empty());
    }

    #[test]
    fn units_are_read_on_even_addresses() {
        // A wide run that begins at an ODD image address is not a wide literal:
        // reading it as one would pair bytes across the compiler's alignment.
        let mut data = vec![0xffu8];
        for ch in "ntdll.dll".chars() {
            data.push(ch as u8);
            data.push(0);
        }
        data.extend_from_slice(&[0, 0]);
        // vma even => units start at offset 0, and offset 1 (the real run) is never
        // a unit boundary.
        assert!(scan_utf16_run(&data, 0x1000, 5).is_empty());
    }

    #[test]
    fn scan_wide_strings_over_the_pe_fixture() {
        // End-to-end: the widened scan must surface the wide `ntdll.dll` the 1-byte
        // pass cannot see, at its `.rdata` address (9 units + terminator = 20 bytes).
        let path =
            concat!(env!("CARGO_MANIFEST_DIR"), "/tests/fixtures/widestrings_x86_64.exe");
        let bytes = std::fs::read(path).expect("read widestrings_x86_64.exe");
        let file = object::File::parse(bytes.as_slice()).expect("parse widestrings_x86_64.exe");
        let wide = scan_wide_strings(&file, DEFAULT_MIN_LEN);
        assert!(
            wide.contains(&StringFact { addr: 0x140002100, len: 20 }),
            "wide ntdll.dll @ 0x140002100 len 20 not detected: {wide:?}"
        );
        // And the 1-byte pass still sees nothing there — the gap this closes.
        assert!(
            !super::super::scan_strings(&file, DEFAULT_MIN_LEN)
                .iter()
                .any(|f| f.addr == 0x140002100),
            "the ASCII width must not claim the wide literal"
        );
    }
}
