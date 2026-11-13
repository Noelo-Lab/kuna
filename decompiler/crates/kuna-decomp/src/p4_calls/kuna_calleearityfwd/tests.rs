//! Unit tests for the forward call-site reconciliation.
//!
//! The retry's *decision* is [`plan_promotions`], which is
//! [`kuna_calleearity`](crate::p4_calls::kuna_calleearity)'s and is covered by that
//! module's tests; what is new here is the shape of the candidate
//! list the retry hands it.  The end-to-end behaviour — a call site that
//! finalizes before its witness — is `tests/stages/kuna-calleearityfwd.xml`.

use super::*;

/// The captured candidates are handed to the shared plan as live, unpromoted
/// trials, so the witness decides entirely: a location the retry did not capture
/// aborts it rather than shifting the remaining arguments left.
#[test]
fn an_uncaptured_witness_location_aborts_the_retry() {
    use kuna_base::space::AddrSpace;
    use std::rc::Rc;
    let reg = Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "register",
        false,
        8,
        1,
        1,
        0,
        0,
        0,
    ));
    let at = |off: u64| Address::new(Rc::clone(&reg), off);
    let view = |off: u64| TrialView {
        addr: at(off),
        size: 8,
        used: false,
        definitely_not_used: false,
        unref: false,
        has_value: true,
    };
    let captured = vec![view(0x10)];
    assert_eq!(plan_promotions(&[(at(0x8), 8), (at(0x10), 8)], &captured), None);
    assert_eq!(plan_promotions(&[(at(0x10), 8)], &captured), Some(vec![0]));
}
