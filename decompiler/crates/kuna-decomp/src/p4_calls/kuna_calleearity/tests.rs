//! Unit tests for the `calleearity` option surface and its promotion plan.

use super::*;
use kuna_base::space::AddrSpace;
use std::rc::Rc;

thread_local! {
    /// One shared space per test thread: two separately constructed `AddrSpace`
    /// values are not the same space, and `Address` equality is what the
    /// promotion plan matches on.
    static REG: Rc<AddrSpace> =
        Rc::new(AddrSpace::new(spacetype::IPTR_PROCESSOR, "register", false, 8, 1, 1, 0, 0, 0));
}

fn at(off: u64) -> Address {
    REG.with(|r| Address::new(Rc::clone(r), off))
}

fn live(off: u64) -> TrialView {
    TrialView {
        addr: at(off),
        size: 8,
        used: false,
        definitely_not_used: false,
        unref: false,
        has_value: true,
    }
}

#[test]
fn option_name_and_apply() {
    assert_eq!(OptionCalleeArity::NAME, "calleearity");
    let (on, msg) = OptionCalleeArity.apply("on").expect("on");
    assert!(on);
    assert!(msg.contains("turned on"), "{msg}");
    let (off, msg) = OptionCalleeArity.apply("off").expect("off");
    assert!(!off);
    assert!(msg.contains("turned off"), "{msg}");
    assert!(OptionCalleeArity.apply("").expect("empty").0);
    assert!(OptionCalleeArity.apply("bogus").is_err());
}

/// The witness case: the sibling passes one register argument this call site
/// dropped, and this call site still holds the Varnode for it.
#[test]
fn a_dropped_register_argument_is_promoted() {
    let witness = vec![(at(0x8), 8)];
    let trials = vec![live(0x8), live(0x10)];
    assert_eq!(plan_promotions(&witness, &trials), Some(vec![0]));
}

/// A call that recovered ANY argument is left alone: `Sleep(200)` must not
/// become `Sleep(200,0)` because a sibling site over-recovered `rdx`.
#[test]
fn a_call_that_already_recovered_an_argument_is_left_alone() {
    let witness = vec![(at(0x8), 8), (at(0x10), 8)];
    let mut trials = vec![live(0x8), live(0x10)];
    trials[0].used = true;
    assert_eq!(plan_promotions(&witness, &trials), None);
}

/// All or nothing: parameters are positional, so a witness whose first location
/// cannot be covered must not have its second one promoted alone.
#[test]
fn an_uncoverable_location_abandons_the_whole_rescue() {
    let mut first = live(0x8);
    first.definitely_not_used = true;
    let trials = vec![first, live(0x10)];
    assert_eq!(plan_promotions(&[(at(0x8), 8), (at(0x10), 8)], &trials), None);

    let mut unref = live(0x8);
    unref.unref = true;
    assert_eq!(plan_promotions(&[(at(0x8), 8)], &[unref]), None);

    let mut mismatched = live(0x8);
    mismatched.has_value = false;
    assert_eq!(plan_promotions(&[(at(0x8), 8)], &[mismatched]), None);

    // A trial whose op input slot no longer carries a value, and a witness
    // location this call has no trial for at all.
    assert_eq!(plan_promotions(&[(at(0x8), 8)], &[live(0x10)]), None);
}

/// A size mismatch is a different storage, not the same one: a 4-byte trial does
/// not answer for an 8-byte witness argument.
#[test]
fn storage_must_match_in_size_as_well_as_address() {
    let mut narrow = live(0x8);
    narrow.size = 4;
    assert_eq!(plan_promotions(&[(at(0x8), 8)], &[narrow]), None);
}
