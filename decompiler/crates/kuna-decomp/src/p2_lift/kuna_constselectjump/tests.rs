//! Logic-level tests for the `constselectjump` gate and its destination filter.
//!
//! The end-to-end recovery (a `CMOVZ`-selected `JMP reg` becoming a switch) is
//! covered by `tests/stages/re-constselectjump.xml`; this file covers the two
//! decisions this module owns on its own.

use super::*;

#[test]
fn option_default_is_reentry() {
    assert_eq!(ConstSelectMode::default(), ConstSelectMode::Reentry);
    assert!(ConstSelectMode::Reentry.admits_reentry() && ConstSelectMode::On.admits_reentry());
    assert!(!ConstSelectMode::Off.admits_reentry());
}

#[test]
fn every_value_parses_and_others_are_refused() {
    for mode in [ConstSelectMode::Off, ConstSelectMode::Reentry, ConstSelectMode::On] {
        let (parsed, msg) = OptionConstSelectJump.apply(mode.as_str()).unwrap();
        assert_eq!(parsed, mode);
        assert!(msg.ends_with(mode.as_str()));
    }
    assert!(OptionConstSelectJump.apply("true").is_err());
}

#[test]
fn destination_filter_accepts_a_nearby_target_either_side() {
    let branch = 0x10003ca23u64;
    assert!(kuna_destination_is_local(branch, 0x10003ca26));
    assert!(kuna_destination_is_local(branch, 0x10003cb26));
    assert!(kuna_destination_is_local(branch, branch - MAX_INTRAPROCEDURAL_SPAN));
    assert!(kuna_destination_is_local(branch, branch + MAX_INTRAPROCEDURAL_SPAN));
}

#[test]
fn destination_filter_rejects_a_far_target_and_zero() {
    let branch = 0x10003ca23u64;
    // A conditional tail call between two distant function entries keeps the
    // upstream `CALLIND` rendering.
    assert!(!kuna_destination_is_local(branch, 0x10004ca26));
    assert!(!kuna_destination_is_local(branch, branch - MAX_INTRAPROCEDURAL_SPAN - 1));
    assert!(!kuna_destination_is_local(branch, branch + MAX_INTRAPROCEDURAL_SPAN + 1));
    assert!(!kuna_destination_is_local(branch, 0));
}

#[test]
fn empty_model_reports_no_table() {
    let m = JumpModelConstSelect::new();
    assert_eq!(m.get_table_size(), 0);
    assert!(m.destinations().is_empty());
    assert!(!m.is_override());
}
