//! Unit tests for the `calleearityfwd` option surface.
//!
//! The retry's *decision* is [`plan_promotions`], which is
//! [`calleearity`](crate::p4_calls::kuna_calleearity)'s and is covered by that
//! module's tests; what is new here is the option and the shape of the candidate
//! list the retry hands it.  The end-to-end behaviour — a call site that
//! finalizes before its witness — is `tests/stages/kuna-calleearityfwd.xml`.

use super::*;

#[test]
fn option_name_and_apply() {
    assert_eq!(OptionCalleeArityFwd::NAME, "calleearityfwd");
    let (on, msg) = OptionCalleeArityFwd.apply("on").expect("on");
    assert!(on);
    assert!(msg.contains("turned on"), "{msg}");
    let (off, msg) = OptionCalleeArityFwd.apply("off").expect("off");
    assert!(!off);
    assert!(msg.contains("turned off"), "{msg}");
    assert!(OptionCalleeArityFwd.apply("").expect("empty").0);
    assert!(OptionCalleeArityFwd.apply("bogus").is_err());
}

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
