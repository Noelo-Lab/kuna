//! (kuna) `aifstrict` — structural rejects for the AIF gap cursor (P1 code/data
//! partition).
//!
//! # The defect (GH-299)
//!
//! [`super::run_aif`] walks the undefined-byte partition [`Listing`] leaves behind
//! and probes it **one byte at a time**: on a reject it does
//! `advanced = gap_start + 1`, because [`Listing::first_undefined_after`] is
//! byte-granular by construction. Every byte of every hole is therefore a
//! candidate function start, and a candidate is accepted on two local tests — a
//! 2-mnemonic prologue fingerprint shared by >= 4 discovered functions, and a
//! valid-subroutine decode. Neither test knows where instructions begin.
//!
//! On a large stripped i386 PE that produces ~2,100 entries that are not function
//! starts at all: they sit on an ordinary instruction in the middle of a body IDA
//! decodes as one function, and 35% of them are inside a function kuna itself
//! already has an entry for. The mechanism is not a bad fingerprint — it is that
//! the walk left a hole *inside* a body (an unfollowed jump table, a no-return
//! call, an undecodable byte), and the byte-slide then reads that hole as if it
//! were the interstice between two functions.
//!
//! Upstream Ghidra ships `AggressiveInstructionFinderAnalyzer` **disabled**, with
//! the description warning "IT MAY CREATE A LOT OF BAD CODE!"; kuna turns it on for
//! non-x86-64 (DIV-20) because on stripped ARM firmware it is the difference
//! between finding almost nothing and finding the program. So the trade this option
//! makes has to be measured in both directions, not assumed.
//!
//! # (E) The aligned slide
//!
//! Every architecture kuna decodes has an instruction-alignment floor of at least
//! two bytes, and on every non-x86 target in the corpus a *function* start is
//! 4-byte aligned. The byte-slide spends 3 of every 4 probes on addresses that
//! cannot be an instruction boundary, and those are precisely the probes that
//! succeed for the wrong reason: a candidate that starts mid-instruction reads the
//! tail of one encoding plus the head of the next, and that synthetic pair is as
//! likely to match a common prologue fingerprint as a real one.
//!
//! [`probe_allowed`] keeps a candidate only when it is 4-byte aligned **or** it is
//! the first byte of its hole. The exemption is what keeps the alignment filter
//! honest: a hole boundary is evidence (the walk decoded up to exactly there), an
//! interior byte the cursor slid onto is a guess. It is also what keeps the
//! `aif_gap_x86_64` fixture green — its indirect-only function at `0x13ae` is not
//! 4-byte aligned, but it *is* a hole start.
//!
//! 4 bytes on every architecture, deliberately: 16-byte alignment kills 91% of the
//! bad entries on Cortex-M but takes 80% of the real ones with it. 4 is not free
//! either — a Thumb function start may legally be 2-mod-4, and on `CMSIS_DAP.axf`
//! (73.5% of ground-truth entries 4-aligned) three real functions are lost. Corpus
//! -wide that is paid for many times over: over 110 stripped non-x86-64 binaries
//! the filter *raises* recall by 344 functions while removing 4,282 mid-body
//! entries, because the accepts it prevents are accepts that would have consumed
//! the real entry behind them.
//!
//! # What is deliberately NOT here
//!
//! The issue's first direction — reject a candidate bracketed by a known function —
//! was implemented and **measured out**. On the Listing's entry-ordered function
//! model, "the hole lies inside one body" can only be approximated as "no known
//! entry lies in `(hole_start, next_code]`", and on a sparsely discovered image
//! that approximation swallows whole unexplored regions: u-boot A32 has 314
//! discovered entries for 2,836 real functions, and the predicate costs it 195 of
//! 1,759 recovered functions (O2-noinline) — while cutting the corpus-wide mid-body
//! count to 3,128. Tightening it to "the hole is the first break in the enclosing
//! function's decoded body" still costs 185. A32 recovery is AIF's entire remaining
//! justification (DIV-20), so neither form ships. The suppression needs real
//! per-instruction walk ownership, not an entry interval.

use crate::listing::Listing;

/// The instruction-alignment floor the slide snaps to. 4 bytes on every
/// architecture: A32/MIPS/PPC/SPARC instructions are 4-byte aligned, Thumb
/// *functions* are (only the 2-byte instruction interior is not), and on i386/PE —
/// the witness class, where nothing is guaranteed aligned — the compiler still
/// aligns function starts, so the filter costs no true find there either.
pub(crate) const SLIDE_ALIGN: u64 = 4;

/// True iff `vma` is the first byte of the undefined hole it lies in, i.e. the byte
/// before it is not undefined (decoded code, typed data, or outside the executable
/// image). Address 0 is trivially a hole start.
///
/// A hole start is *evidence*: the recursive-descent walk decoded up to exactly
/// there and stopped. Every other address in the hole is a cursor guess.
pub(crate) fn is_hole_start(listing: &Listing, vma: u64) -> bool {
    match vma.checked_sub(1) {
        None => true,
        Some(prev) => !listing.is_undefined(prev),
    }
}

/// (E) Whether the gap cursor may probe `vma` at all: a 4-byte-aligned address, or
/// the first byte of its hole.
pub(crate) fn probe_allowed(listing: &Listing, vma: u64) -> bool {
    vma % SLIDE_ALIGN == 0 || is_hole_start(listing, vma)
}

/// (E) The address the gap cursor advances to after rejecting `gap_start`: the next
/// 4-byte-aligned address, clamped to `gap_hi` (the exclusive end of this hole) so
/// the slide can never step *over* the start of the following hole.
///
/// Clamping matters because the caller resumes with
/// `first_undefined_after(advanced - 1)`: an unclamped aligned jump could land past
/// a one-byte instruction and skip the next hole's first byte, which is the one
/// address [`probe_allowed`] guarantees is probed.
pub(crate) fn next_probe_after(gap_start: u64, gap_hi: u64) -> u64 {
    let aligned = (gap_start & !(SLIDE_ALIGN - 1)).saturating_add(SLIDE_ALIGN);
    aligned.min(gap_hi).max(gap_start.saturating_add(1))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aligned_slide_snaps_to_the_next_multiple_of_four() {
        // From an aligned start the slide is a full stride.
        assert_eq!(next_probe_after(0x1000, u64::MAX), 0x1004);
        // From an interior byte it snaps up to the next boundary, never past it.
        assert_eq!(next_probe_after(0x1001, u64::MAX), 0x1004);
        assert_eq!(next_probe_after(0x1002, u64::MAX), 0x1004);
        assert_eq!(next_probe_after(0x1003, u64::MAX), 0x1004);
        // Clamped at the hole end so the next hole's first byte is never skipped.
        assert_eq!(next_probe_after(0x1001, 0x1002), 0x1002);
        assert_eq!(next_probe_after(0x1001, 0x1003), 0x1003);
        // Always strictly forward (no cursor stall).
        for a in 0x1000u64..0x1010 {
            assert!(next_probe_after(a, u64::MAX) > a);
            assert!(next_probe_after(a, a + 1) > a);
        }
    }

    #[test]
    fn slide_alignment_is_four_not_sixteen() {
        // 16-byte alignment kills 91% of the bad Cortex-M entries but 80% of the
        // real ones; the measured choice is 4 on every architecture.
        assert_eq!(SLIDE_ALIGN, 4);
    }
}
