//! Unit tests for the stack-argument caller-frame predicate.

use super::*;
use kuna_base::space::{spacetype, AddrSpace};

fn stack_space() -> Rc<AddrSpace> {
    Rc::new(AddrSpace::new(spacetype::IPTR_SPACEBASE, "stack", false, 8, 1, 1, 0, 0, 0))
}

/// The predicate is exactly the bug: the outgoing-argument slot sits at a
/// NEGATIVE caller-relative offset (inside the caller's localrange) and a
/// POSITIVE callee-relative one (outside it).  The caller-relative address is
/// what is probed, so the trial survives.
#[test]
fn probe_address_is_the_caller_relative_slot() {
    let spc = stack_space();
    // A typical x86-64 gcc localrange: the negative-offset caller frame.
    let mut localrange = RangeList::default();
    localrange.insert_range(Rc::clone(&spc), 0xffff_ffff_ffff_0000, 0xffff_ffff_ffff_ffff);

    // The argument Varnode guard_calls created, caller-relative.
    let vn_offset: uintb = 0xffff_ffff_ffff_fff0;

    assert!(
        !outside_caller_local_range(&localrange, &spc, vn_offset),
        "the caller-relative slot is inside the caller's frame, so the trial survives"
    );
}

/// A stack slot genuinely outside the caller's frame is rejected: the predicate
/// never widens the range itself.
#[test]
fn genuinely_out_of_range_slot_is_rejected() {
    let spc = stack_space();
    let mut localrange = RangeList::default();
    localrange.insert_range(Rc::clone(&spc), 0xffff_ffff_ffff_0000, 0xffff_ffff_ffff_ffff);

    let vn_offset: uintb = 0x4000; // positive, far outside the caller frame

    assert!(outside_caller_local_range(&localrange, &spc, vn_offset));
}
