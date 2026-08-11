//! P3 partial-range call-overlap guards — the `calloverlap` decision point.
//!
//! # The gap
//!
//! [`Heritage::guard_calls`](crate::p3_dataflow::heritage::Heritage) characterizes
//! every heritaged range against each call site's prototype.  When the range is
//! *bigger* than the ABI storage it contains the characterization is
//! [`Containment::ContainedBy`](crate::fspec::Containment), and upstream Ghidra
//! runs one of two dedicated guards — `Heritage::guardCallOverlappingInput`
//! (`heritage.cc:1210`) on the input side and `Heritage::tryOutputOverlapGuard` /
//! `Heritage::guardOutputOverlap` (`heritage.cc:1249`/`:1293`) on the output side.
//! kuna shipped both as comment-only stubs, so the parameter and return entries
//! inside such a range were invisible to trial recovery.
//!
//! This is routine on x86-64: SLEIGH models `PXOR`, `MOVDQA`, `MOVDQU` and `ORPD`
//! as ONE 128-bit write to the whole `XmmReg`, so `Heritage::refinement`'s
//! `size > 4 && maxw < size` gate never fires and the range stays 16 bytes wide
//! while the SysV parameter/return entry inside it is 8.  The observable symptom is
//! a call rendered with missing arguments and a return value read from a stale
//! pre-call definition (`u(); sink = v1;` where `v1` is never assigned).
//!
//! # The levels
//!
//! The two halves are separable and carry different risk, so the option is a
//! tri-state rather than an on/off:
//!
//! * [`LEVEL_OFF`] — both branches inert; byte-identical to what kuna shipped
//!   before this option.
//! * [`LEVEL_IN`] — the input guard only.  Recovers the dropped argument but
//!   leaves the call's return value unrecovered.
//! * [`LEVEL_FULL`] — both guards, which is upstream Ghidra's behavior.
//!
//! The guards themselves are upstream `Heritage` methods and live with their
//! siblings in [`crate::p3_dataflow::heritage`]; this module owns the option's
//! parse surface and the level vocabulary the gate reads.

use kuna_base::error::KunaResult;
use kuna_base::types::int4;

/// `option calloverlap off` — both overlap guards stay inert.
pub const LEVEL_OFF: int4 = 0;

/// `option calloverlap in` — the input guard (`guardCallOverlappingInput`) only.
pub const LEVEL_IN: int4 = 1;

/// `option calloverlap full` — the input guard plus the output guard
/// (`tryOutputOverlapGuard`), i.e. upstream Ghidra's behavior.
pub const LEVEL_FULL: int4 = 2;

/// `option calloverlap off|in|full` (C++ has no equivalent — upstream is
/// unconditionally `full`).
pub struct OptionCallOverlap;

impl OptionCallOverlap {
    /// The option name.
    pub const NAME: &'static str = "calloverlap";

    /// Parse `off`/`in`/`full` into the guard level + a confirmation message.
    pub fn apply(&self, p1: &str) -> KunaResult<(int4, String)> {
        match p1 {
            "off" => Ok((
                LEVEL_OFF,
                "Partial-range call overlap guards turned off".to_string(),
            )),
            "in" => Ok((
                LEVEL_IN,
                "Partial-range call overlap guards turned on, input side only \
                 (guardCallOverlappingInput)"
                    .to_string(),
            )),
            // Empty parameter reads as the full guard, matching `options::on_or_off`.
            "full" | "on" | "" => Ok((
                LEVEL_FULL,
                "Partial-range call overlap guards turned on, input + output \
                 (guardCallOverlappingInput + tryOutputOverlapGuard)"
                    .to_string(),
            )),
            _ => Err(kuna_base::error::KunaError::parse(
                "Must specify one of off, in, full",
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn levels_are_ordered_so_the_gate_can_use_a_threshold() {
        // Both guards read `level >= LEVEL_IN` / `level >= LEVEL_FULL`, so the
        // vocabulary must stay monotone.
        assert!(LEVEL_OFF < LEVEL_IN);
        assert!(LEVEL_IN < LEVEL_FULL);
    }

    #[test]
    fn apply_parses_the_three_spellings() {
        assert_eq!(OptionCallOverlap.apply("off").expect("off").0, LEVEL_OFF);
        assert_eq!(OptionCallOverlap.apply("in").expect("in").0, LEVEL_IN);
        assert_eq!(OptionCallOverlap.apply("full").expect("full").0, LEVEL_FULL);
        // `on` and the empty parameter are the full guard.
        assert_eq!(OptionCallOverlap.apply("on").expect("on").0, LEVEL_FULL);
        assert_eq!(OptionCallOverlap.apply("").expect("empty").0, LEVEL_FULL);
        assert!(OptionCallOverlap.apply("wide").is_err());
    }
}
