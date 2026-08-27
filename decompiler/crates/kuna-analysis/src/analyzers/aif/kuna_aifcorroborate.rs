//! (kuna) `aifcorroborate` — the AIF accept needs corroboration, not just a
//! 4-shared prologue (P1 code/data partition).
//!
//! # The defect (GH-313)
//!
//! Upstream's `AggressiveInstructionFinderAnalyzer` rejects a gap candidate on
//! **two** fingerprint tests, applied at different points of the same loop:
//!
//! ```text
//! AggressiveInstructionFinderAnalyzer.java:278   if (startCount < 4) { continue; }
//! AggressiveInstructionFinderAnalyzer.java:367   // don't allow two instruction routines
//!                                               // if the routine doesn't add good info, it
//!                                               // must start like a large numbe of other functions
//!                                               if (numInstr <= 2 || (!addsInfo && startCount < 50)) { continue; }
//! ```
//!
//! kuna ported the first test ([`super::FINGERPRINT_THRESHOLD`]) and the
//! `numInstr <= 2` half of the second ([`super::MIN_SUBROUTINE_INSNS`]), and
//! dropped `(!addsInfo && startCount < 50)` entirely. Both of its inputs exist —
//! `check_valid_subroutine_with_policy` walks the candidate's flows, and
//! `probe_gap_start` has the histogram hit — but the flag was discarded at the
//! return and the count never left its frame, so the test was never applied.
//!
//! The consequence is the accept upstream refuses: a self-contained routine that
//! calls nothing, jumps nowhere known, and merely reaches a `ret` is accepted at
//! `startCount == 4` with **no corroboration at all**.
//!
//! # The guard
//!
//! [`accepts`] is upstream's disjunction, verbatim: an accept must EITHER add
//! information OR match a prologue that [`UNCORROBORATED_THRESHOLD`] discovered
//! functions share.
//!
//! Two details make it upstream's guard rather than a rhyme of it.
//!
//! **The corroborating fact is computed upstream's way.** Ghidra's `addsInfo` is
//! set only from `instr.getFlows()` — unconditionally on a call ("calls always add
//! info"), and on a jump only when `listing.getInstructionAt(flows[0]) != null`
//! ("jumps must jump to existing code"). kuna's own `adds_info`, which feeds the
//! separate `didTerminate || didCallValidSubroutine` gate, is looser: its
//! out-of-gap arm counts any flow target that is a known instruction start,
//! including a plain FALL-THROUGH off the end of the hole into decoded code. That
//! is precisely the signature of a mid-body phantom — the walk left a hole inside a
//! real body, and a candidate planted in the hole flows out of it into the rest of
//! that same body — so reusing it selects backwards. This module therefore reads a
//! second flag, `corroborated`, computed strictly upstream-style, and leaves
//! `adds_info` and the gate it feeds untouched.
//!
//! **A refusal must not release the cursor.** [`super::run_aif`] advances past an
//! ACCEPTED body and one byte past a rejected one, so refusing a candidate hands
//! the cursor back to the interior of the same hole, where it accepts something
//! worse. Measured on the 3.4 MB PE witness, the accept-side guard alone turned a
//! 361-entry mid-body cut into a 222-entry mid-body RISE. An uncorroborated
//! candidate is therefore refused as an ENTRY but still CONSUMES its body
//! (`Probe::Uncorroborated`).
//!
//! It is applied at ONE call site, the gap-walk accept in
//! [`super::probe_gap_start`]. The three other users of the validity predicate
//! (`raw_thumb_prologue_seeds`, `code_pointer_table_seeds`,
//! `validate_pointer_targets`) each carry their own corroboration already — a
//! literal LR-saving Thumb prologue byte-pattern, or a code pointer stored in the
//! image that points at the candidate — so upstream's fingerprint-only fallback
//! reasoning does not apply to them and they are deliberately untouched.
//!
//! # Measured: the guard does NOT transfer, and that is why it is opt-in
//!
//! 110 stripped non-x86-64 decbench binaries, each scored against its own
//! unstripped twin's symbol table, `--mode aggressive` (so ON TOP OF `aifstrict`,
//! which the preset carries — the shipped default path for this corpus):
//!
//! ```text
//!                    AIF-marginal mid-body     ground-truth recall
//!   shipped                          6,728        44,957 / 50,724
//!   aifcorroborate on                4,653        44,107 / 50,724
//!                                   -30.8%                   -850
//! ```
//!
//! Every mid-body entry it removes costs about half a real function, and **not one
//! of the 110 binaries gains recall**. The two u-boot A32 images — the images
//! DIV-20 exists for, and 66% of AIF's corpus-wide true recall — lose 84 (O2) and
//! 141 (O2-noinline) real functions. On the two PE witnesses the trade is worse
//! still: a 590 KB i386 crackme gives up 24 real entries for 16 mid-body ones, and
//! the 3.4 MB stripped i386 PE of GH-299 gives up 1,100 IDA-agreed entries for 361.
//! A threshold grid over {4, 8, 12, 20, 50, 100} finds no escape: 4 is the existing
//! floor and therefore inert, and u-boot is already down 79 functions at 8.
//!
//! So this ships **default-off and in no preset**, and GH-313's G1 is answered with
//! a number rather than a hypothesis. What upstream's guard assumes — that a
//! function worth finding calls something the analyzer already knows — does not
//! hold for the bare-metal ARM firmware that is AIF's remaining justification in
//! kuna, where the routines only AIF can find are exactly the leaf helpers that
//! call nothing.

/// Ghidra's `startCount < 50` (`AggressiveInstructionFinderAnalyzer.java:367`):
/// the number of discovered functions that must share a candidate's prologue
/// fingerprint before it is accepted with NO other corroboration.
///
/// The upstream literal, kept as-is. A grid over {4, 8, 12, 20, 50, 100} on the
/// corpus tripwires buys nothing by diverging: 4 is [`super::FINGERPRINT_THRESHOLD`]
/// and so inert, u-boot O2-noinline is already down 79 real functions at 8, and 100
/// only deepens the loss. There is no threshold at which this guard is free.
pub(crate) const UNCORROBORATED_THRESHOLD: usize = 50;

/// Upstream's `!(!addsInfo && startCount < 50)`: accept when the routine adds
/// information (a call, or a jump into already-discovered code) OR its prologue is
/// shared by at least [`UNCORROBORATED_THRESHOLD`] discovered functions.
pub(crate) fn accepts(corroborated: bool, start_count: usize) -> bool {
    corroborated || start_count >= UNCORROBORATED_THRESHOLD
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn corroboration_or_a_large_shared_prologue() {
        // Adds info: accepted at any fingerprint count that cleared the first gate.
        assert!(accepts(true, 4));
        assert!(accepts(true, 0));
        // No corroboration: the prologue must be a *large* shared start.
        assert!(!accepts(false, 4));
        assert!(!accepts(false, UNCORROBORATED_THRESHOLD - 1));
        assert!(accepts(false, UNCORROBORATED_THRESHOLD));
        assert!(accepts(false, UNCORROBORATED_THRESHOLD + 1));
    }

    #[test]
    fn threshold_is_the_upstream_literal() {
        // `AggressiveInstructionFinderAnalyzer.java:367` — `startCount < 50`.
        assert_eq!(UNCORROBORATED_THRESHOLD, 50);
        // And it is strictly above the first gate's threshold, which is the whole
        // point: the two tests are not the same test. At 4 this guard is inert.
        assert!(UNCORROBORATED_THRESHOLD > super::super::FINGERPRINT_THRESHOLD);
    }
}
