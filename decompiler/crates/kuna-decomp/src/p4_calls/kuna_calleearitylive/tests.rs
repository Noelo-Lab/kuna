//! Unit tests for the `calleearitylive` option surface and its two decisions.
//!
//! The decisions this module owns are which candidate trials cover the witness's
//! tail ([`plan_tail`]) and which argument locations the witness leaves
//! unclaimed ([`uncovered_argument_locations`]) — the second being what refuses
//! a variadic callee.  The end-to-end behaviour is
//! `tests/stages/kuna-calleearitylive.xml`.

use super::*;

use kuna_base::space::AddrSpace;
use std::rc::Rc;

fn reg_space() -> Rc<AddrSpace> {
    Rc::new(AddrSpace::new(spacetype::IPTR_PROCESSOR, "register", false, 8, 1, 1, 0, 0, 0))
}

#[test]
fn option_name_and_apply() {
    assert_eq!(OptionCalleeArityLive::NAME, "calleearitylive");
    let (on, msg) = OptionCalleeArityLive.apply("on").expect("on");
    assert!(on);
    assert!(msg.contains("turned on"), "{msg}");
    let (off, msg) = OptionCalleeArityLive.apply("off").expect("off");
    assert!(!off);
    assert!(msg.contains("turned off"), "{msg}");
    assert!(OptionCalleeArityLive.apply("").expect("empty").0);
    assert!(OptionCalleeArityLive.apply("bogus").is_err());
}

/// All or nothing, and in prototype order: the tail has to be covered by
/// candidates left to right, or the site is left alone rather than having its
/// remaining arguments shifted.
#[test]
fn the_tail_is_covered_in_order_or_not_at_all() {
    let reg = reg_space();
    let at = |off: u64| (Address::new(Rc::clone(&reg), off), 8);
    let candidates = vec![at(0x20), at(0x28)];
    assert_eq!(plan_tail(&[at(0x20), at(0x28)], &candidates), Some(vec![0, 1]));
    assert_eq!(plan_tail(&[at(0x28)], &candidates), Some(vec![1]));
    // Out of prototype order: 0x20 is behind the 0x28 already consumed.
    assert_eq!(plan_tail(&[at(0x28), at(0x20)], &candidates), None);
    // A location this site never captured aborts it.
    assert_eq!(plan_tail(&[at(0x20), at(0x30)], &candidates), None);
    // A different width is a different location.
    let narrow = (Address::new(Rc::clone(&reg), 0x20), 4);
    assert_eq!(plan_tail(&[narrow], &candidates), None);
}

/// The witness claims a location whenever it OVERLAPS it: a four-byte `ESI`
/// trial answers for the eight-byte `RSI` entry it sits in, so a fixed-arity
/// callee leaves only the registers past its own list uncovered.
#[test]
fn a_narrow_trial_covers_the_entry_it_sits_in() {
    let reg = reg_space();
    let entries = vec![
        (Address::new(Rc::clone(&reg), 0x38), 8), // RDI
        (Address::new(Rc::clone(&reg), 0x30), 8), // RSI
        (Address::new(Rc::clone(&reg), 0x80), 8), // R8
        (Address::new(Rc::clone(&reg), 0x88), 8), // R9
    ];
    let witness = vec![
        (Address::new(Rc::clone(&reg), 0x38), 8),
        (Address::new(Rc::clone(&reg), 0x30), 4), // ESI
        (Address::new(Rc::clone(&reg), 0x80), 1), // R8B
    ];
    let left = uncovered_argument_locations(&entries, &witness);
    assert_eq!(left.len(), 1, "{left:?}");
    assert_eq!(left[0].0.get_offset(), 0x88);
}

/// An empty witness claims nothing, so every argument location stays uncovered
/// and the callee reading any of them declines the extension.
#[test]
fn an_empty_witness_covers_nothing() {
    let reg = reg_space();
    let entries = vec![(Address::new(Rc::clone(&reg), 0x38), 8)];
    assert_eq!(uncovered_argument_locations(&entries, &[]).len(), 1);
}

/// A stack location never answers for a register entry, so a witness whose
/// arguments live in another space cannot silently claim the register file.
#[test]
fn a_location_in_another_space_covers_nothing() {
    let reg = reg_space();
    let stack =
        Rc::new(AddrSpace::new(spacetype::IPTR_SPACEBASE, "stack", false, 8, 1, 2, 0, 0, 0));
    let entries = vec![(Address::new(Rc::clone(&reg), 0x38), 8)];
    let witness = vec![(Address::new(Rc::clone(&stack), 0x38), 8)];
    assert_eq!(uncovered_argument_locations(&entries, &witness).len(), 1);
}
