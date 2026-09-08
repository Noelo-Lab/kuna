//! (kuna `utf8strings`) The multi-byte reading of the 1-byte width, for the
//! [`inventory`](super::kuna_stringinv::inventory) query.
//!
//! # The gap
//!
//! [`super::is_string_char`] is `AsciiCharSetRecognizer.contains`, so every byte
//! `>= 0x80` ends a run. A literal that opens with any non-ASCII character is
//! therefore reported starting at the byte AFTER its last multi-byte sequence:
//! `_φ( °-°)/ so what was the magical keycombination? ` at 0x2000 is reported as
//! `)/ so what was the magical keycombination? ` at 0x200c. That address is not
//! an address anything in the image refers to, so the row also arrives with zero
//! references and no owning function while `kuna xrefs --to 0x2000` answers one —
//! the reference machinery already knows the literal's true start, and only the
//! reported address is wrong.
//!
//! # The width
//!
//! [`scan_utf8_runs`] is the 1-byte `MinLengthCharSequenceMatcher` with one
//! addition: where the recognizer rejects a byte `>= 0x80`, a well-formed UTF-8
//! sequence starting at that byte is decoded and, if its scalar is not a control
//! character, taken as ONE character of the run. Everything else is the 1-byte
//! matcher's — the same [`super::is_string_char`] recognizer for single bytes,
//! the same [`Termination`] rule, the same minimum length counted in characters.
//!
//! It is a *superset* of the 1-byte scan, not a rival to it: a continuation byte
//! is never in the 1-byte charset, so no accepted sequence can swallow a byte the
//! 1-byte matcher would have taken, and an ill-formed sequence advances one byte
//! and lets the scan continue. Every 1-byte run is therefore a subrange of some
//! run here, which is why [`inventory`](super::kuna_stringinv::inventory) runs
//! one 1-byte scan or the other rather than both, and labels each row by what its
//! bytes actually hold.
//!
//! Scope: the **inventory** only. [`super::StringLiteralPass`] still commits
//! exactly the `char[N]` facts it always did, so no emitted C moves.

use super::{is_string_char, Run, Termination};

/// The character at `data[i]`: one in-charset ASCII byte, or one well-formed
/// multi-byte UTF-8 sequence whose scalar is not a control character.
///
/// Returns the byte width consumed. `None` means the byte closes a run — an
/// out-of-charset ASCII byte, a lead byte that no valid sequence follows, a
/// continuation byte on its own, or a scalar the recognizer declines.
fn char_at(data: &[u8], i: usize) -> Option<usize> {
    let b = *data.get(i)?;
    if is_string_char(b) {
        return Some(1);
    }
    // The lead byte fixes the sequence length; `from_utf8` then rejects a bad
    // continuation byte, an overlong encoding, a surrogate and anything past
    // U+10FFFF. 0xc0/0xc1 are overlong lead bytes and never start one.
    let width = match b {
        0xc2..=0xdf => 2,
        0xe0..=0xef => 3,
        0xf0..=0xf4 => 4,
        _ => return None,
    };
    let seq = data.get(i..i.checked_add(width)?)?;
    let c = std::str::from_utf8(seq).ok()?.chars().next()?;
    if c.is_control() {
        return None;
    }
    Some(width)
}

/// Mirror of the 1-byte [`super::scan_runs`] with UTF-8 sequences admitted as
/// characters: accumulate a run of [`char_at`] characters, closed by the first
/// byte that is neither, and emit it when it holds at least `min_len`
/// **characters**. `term` is the same require-NUL-end rule at this reading.
///
/// A [`Run`]'s `visible_len` is in BYTES at every width, so a run here spans more
/// bytes than it holds characters; the caller reads the character count off the
/// decoded text.
pub(crate) fn scan_utf8_runs(
    data: &[u8],
    vma: u64,
    min_len: usize,
    term: Termination,
) -> Vec<Run> {
    let mut out = Vec::new();
    let mut run_start: Option<(usize, usize)> = None;
    let close = |start: usize, chars: usize, end: usize, nul: bool, out: &mut Vec<Run>| {
        if chars >= min_len && (nul || term == Termination::Any) {
            out.push(Run { addr: vma + start as u64, visible_len: end - start, nul_terminated: nul });
        }
    };
    let mut i = 0usize;
    while i < data.len() {
        if let Some(width) = char_at(data, i) {
            match &mut run_start {
                Some((_, chars)) => *chars += 1,
                None => run_start = Some((i, 1)),
            }
            i += width;
            continue;
        }
        if let Some((start, chars)) = run_start.take() {
            close(start, chars, i, data[i] == 0, &mut out);
        }
        i += 1;
    }
    if let Some((start, chars)) = run_start {
        close(start, chars, data.len(), false, &mut out);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::strings::scan_runs;

    /// The filing image's literal, verbatim: `_φ( °-°)/ so what was the magical
    /// keycombination? ` — a fullwidth low line, a phi and two degree signs.
    const KAOMOJI: &[u8] = b"\xef\xbc\xbf\xcf\x86\x28\x20\xc2\xb0\x2d\xc2\xb0\x29\x2f so what \
was the magical keycombination? \x00";

    #[test]
    fn a_multi_byte_prefix_no_longer_moves_the_start() {
        // The 1-byte matcher starts the run after the LAST multi-byte sequence.
        let ascii = scan_runs(KAOMOJI, 0x2000, 5, Termination::Any);
        assert_eq!(ascii.len(), 1);
        assert_eq!(ascii[0].addr, 0x200c, "the 1-byte reading loses the prefix");

        // The UTF-8 reading starts it at the literal's first byte, and its extent
        // runs to the NUL: 0x37 bytes of text, 50 characters.
        let utf8 = scan_utf8_runs(KAOMOJI, 0x2000, 5, Termination::Any);
        assert_eq!(
            utf8,
            vec![Run { addr: 0x2000, visible_len: 0x37, nul_terminated: true }],
            "the whole literal is one run at its true address"
        );
        let text = std::str::from_utf8(&KAOMOJI[..0x37]).expect("valid UTF-8 by construction");
        assert_eq!(text.chars().count(), 50);
        assert!(text.starts_with('\u{ff3f}'));
    }

    #[test]
    fn pure_ascii_reads_exactly_as_the_1_byte_matcher() {
        // The guarantee the inventory relies on to run one 1-byte scan instead of
        // two: with no multi-byte sequence present the two matchers agree
        // run-for-run, under either ending policy.
        let data = b"\x00Hi\x00Username: \x00\x01\x02abcdefgh\x00trailing";
        for term in [Termination::Nul, Termination::Any] {
            assert_eq!(
                scan_utf8_runs(data, 0x1000, 5, term),
                scan_runs(data, 0x1000, 5, term),
                "{term:?} disagrees on pure ASCII"
            );
        }
    }

    #[test]
    fn an_ill_formed_sequence_closes_the_run_without_eating_ascii() {
        // 0xc3 is a lead byte, but '(' is not a continuation byte: the sequence is
        // rejected, the run closes, and the scan resumes ON the '(' rather than
        // past it — so no 1-byte run is ever swallowed by a bad decode.
        let data = b"hello\xc3(world!\x00";
        let runs = scan_utf8_runs(data, 0, 5, Termination::Any);
        assert_eq!(
            runs,
            vec![
                Run { addr: 0, visible_len: 5, nul_terminated: false },
                Run { addr: 6, visible_len: 7, nul_terminated: true },
            ]
        );
        assert_eq!(runs, scan_runs(data, 0, 5, Termination::Any), "same as the 1-byte matcher");
    }

    #[test]
    fn overlongs_surrogates_and_controls_are_declined() {
        // C0 80 (overlong NUL), ED A0 80 (a surrogate), F5 80 80 80 (past
        // U+10FFFF) and C2 85 (U+0085 NEXT LINE, a C1 control) are all rejected,
        // so none of them joins two neighbouring runs into one.
        for bad in [&b"\xc0\x80"[..], b"\xed\xa0\x80", b"\xf5\x80\x80\x80", b"\xc2\x85"] {
            let mut data = b"aaaaa".to_vec();
            data.extend_from_slice(bad);
            data.extend_from_slice(b"bbbbb\x00");
            let runs = scan_utf8_runs(&data, 0, 5, Termination::Any);
            assert_eq!(runs.len(), 2, "{bad:x?} must not join the two runs: {runs:?}");
            assert_eq!(runs[0].addr, 0);
        }
    }

    #[test]
    fn the_minimum_length_counts_characters_not_bytes() {
        // Five characters, ten bytes: accepted at min 5, rejected at min 6.
        let data = b"\xc2\xb0\xc2\xb0\xc2\xb0\xc2\xb0\xc2\xb0\x00";
        assert_eq!(
            scan_utf8_runs(data, 0, 5, Termination::Nul),
            vec![Run { addr: 0, visible_len: 10, nul_terminated: true }]
        );
        assert!(scan_utf8_runs(data, 0, 6, Termination::Nul).is_empty());
    }

    #[test]
    fn a_sequence_truncated_by_the_end_of_a_region_is_not_a_character() {
        // The last byte of a section is a lead byte with nothing after it.
        let data = b"aaaaa\xc2";
        assert_eq!(
            scan_utf8_runs(data, 0, 5, Termination::Any),
            vec![Run { addr: 0, visible_len: 5, nul_terminated: false }]
        );
    }
}
