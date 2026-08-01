//! Unit tests for the `callsitestackargs` option surface and its predicate.

use super::*;
use kuna_base::space::{spacetype, AddrSpace};

fn stack_space() -> Rc<AddrSpace> {
    Rc::new(AddrSpace::new(spacetype::IPTR_SPACEBASE, "stack", false, 8, 1, 1, 0, 0, 0))
}

#[test]
fn option_name_and_apply() {
    assert_eq!(OptionCallsiteStackArgs::NAME, "callsitestackargs");
    let (on, msg) = OptionCallsiteStackArgs.apply("on").expect("on");
    assert!(on);
    assert!(msg.contains("turned on"), "{msg}");
    let (off, msg) = OptionCallsiteStackArgs.apply("off").expect("off");
    assert!(!off);
    assert!(msg.contains("turned off"), "{msg}");
    // Empty defaults to on, as every other on/off option does.
    assert!(OptionCallsiteStackArgs.apply("").expect("empty").0);
    assert!(OptionCallsiteStackArgs.apply("bogus").is_err());
}

/// The predicate is exactly the bug: the outgoing-argument slot sits at a
/// NEGATIVE caller-relative offset (inside the caller's localrange) and a
/// POSITIVE callee-relative one (outside it).  `on` must admit it, `off` must
/// reject it — that difference is the whole feature.
#[test]
fn probe_address_selects_caller_vs_callee_frame() {
    let spc = stack_space();
    // A typical x86-64 gcc localrange: the negative-offset caller frame.
    let mut localrange = RangeList::default();
    localrange.insert_range(Rc::clone(&spc), 0xffff_ffff_ffff_0000, 0xffff_ffff_ffff_ffff);

    // The argument Varnode guard_calls created, caller-relative.
    let vn_offset: uintb = 0xffff_ffff_ffff_fff0;
    // The trial guard_calls registered, callee-relative (vn_offset - stackoffset).
    let trial_addr = Address::new(Rc::clone(&spc), 0x8);

    assert!(
        !outside_caller_local_range(true, &localrange, &spc, vn_offset, &trial_addr),
        "on: the caller-relative slot is inside the caller's frame, so the trial survives"
    );
    assert!(
        outside_caller_local_range(false, &localrange, &spc, vn_offset, &trial_addr),
        "off: the callee-relative address is outside the caller's frame, so the trial is dropped"
    );
}

/// A stack slot genuinely outside the caller's frame is rejected either way —
/// the option only reinterprets which address is probed, it never widens the
/// range itself.
#[test]
fn genuinely_out_of_range_slot_is_rejected_either_way() {
    let spc = stack_space();
    let mut localrange = RangeList::default();
    localrange.insert_range(Rc::clone(&spc), 0xffff_ffff_ffff_0000, 0xffff_ffff_ffff_ffff);

    let vn_offset: uintb = 0x4000; // positive, far outside the caller frame
    let trial_addr = Address::new(Rc::clone(&spc), 0x4000);

    assert!(outside_caller_local_range(true, &localrange, &spc, vn_offset, &trial_addr));
    assert!(outside_caller_local_range(false, &localrange, &spc, vn_offset, &trial_addr));
}
