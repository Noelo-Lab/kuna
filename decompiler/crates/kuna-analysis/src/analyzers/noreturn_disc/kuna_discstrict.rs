//! (kuna) `noreturn_discstrict` — the narrowed no-fall-through predicate for the
//! discovered-no-return evidence tally (P1 external refinement).
//!
//! # The defect (GH-312)
//!
//! [`super::call_site_has_no_fallthrough`] tallies a call site as evidence that its
//! callee never returns when the fall-through VMA "is not a valid instruction".
//! Three arms implement that, tried in order: the successor is not a decoded
//! instruction start, the successor is data, the successor is another function's
//! entry. Three such sites conclude the callee no-return, and the engine then
//! DELETES the code after every call to it.
//!
//! The first arm does not mean what its name says. The Listing walk
//! ([`crate::listing::walk`]) pushes **every** call's fall-through onto the
//! per-function instruction worklist unconditionally, and that worklist drains
//! before the function is left, so a call's successor is always *attempted*. It
//! fails to become an instruction start in exactly three ways: `decode_one`
//! returned `Err`, the decode was zero-length, or the address is outside every
//! executable range. The Listing keeps no record of which — a failed decode and an
//! address nothing ever visited are the same `CodeUnit::Undefined` — so the arm is
//! precisely and only a **decode-failure detector**, and a decoder gap is a fact
//! about kuna, not about the callee.
//!
//! Three undecodable bytes are therefore enough to forge the verdict for a callee
//! that plainly returns, and every caller of that callee loses its live tail. The
//! `noreturn_discstrict_x86_64` fixture is that shape at minimum size: `f` is
//! `mov $7,%eax; ret`, three callers follow their `call f` with one byte that is
//! invalid in 64-bit mode, and a fourth caller whose `return f() + 3` is real code
//! is truncated to `f(); // no-return`.
//!
//! # The narrowing
//!
//! [`successor_is_positive_evidence`] keeps only the arms that observe something
//! about the *program*:
//!
//! - the successor is **data** — it is outside every executable range, so the
//!   compiler emitted no code to fall into (a `call` in the last bytes of a
//!   section);
//! - the successor is **another function's entry** — the compiler placed the next
//!   function immediately after the call, leaving the caller no tail at all.
//!
//! The terminal arm (a call with no fall-through VMA at all — a tail jump lowered
//! to a call) stays in the caller, ahead of this test: it is a property of the call
//! instruction, not of its successor.
//!
//! Dropping the bare arm is what makes the data arm *live*. `is_data(fall)` implies
//! `!is_instruction_start(fall)`, so under the legacy order the data arm was
//! unreachable — every site it would have caught was already consumed by the arm
//! above it. The narrowing does not add a test; it exposes one.
//!
//! # What is deliberately NOT here
//!
//! No decode-outcome record on the Listing. Distinguishing "decode failed here"
//! from "never visited" would make the dropped arm honest, but it would still be a
//! statement about the decoder: a spec gap on Thumb or PPC would keep voting for a
//! no-return conclusion that removes the caller's code. The two positive arms need
//! no such record, and `noreturn_propagate` (no threshold, seeded from the Known
//! list) already covers the padding-tail wrapper shape that motivated the tally.

use crate::listing::Listing;

/// Does the fall-through address `fall` of a call to `callee` carry POSITIVE
/// evidence that the callee never returns?
///
/// True when the successor is data (outside every executable range) or is another
/// function's entry. A successor that merely failed to decode is NOT evidence —
/// that is the GH-312 narrowing; see the module docs.
pub(super) fn successor_is_positive_evidence(listing: &Listing, fall: u64, callee: u64) -> bool {
    if listing.is_data(fall) {
        return true;
    }
    fall != callee && listing.function_at(fall).is_some()
}
