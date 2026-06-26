//! Logic-level tests for the `switchguardbound` ArchOption gate (angr port).
//!
//! The recovery walk `JumpBasic::kuna_try_guard_bound_table` lives in
//! `jumptable.rs` and ports with the W4 jumptable wave (see the module docs);
//! this file covers the gate this module owns. End-to-end behaviour is exercised
//! by `tests/stages/ghangr-missing-function-call-1101b1.xml`.

use super::*;

#[test]
fn option_default_is_off() {
    // Shipped default: option switchguardbound off (upstream byte-identical).
    let opt = SwitchGuardBoundOption::default();
    assert!(!opt.is_enabled());
}

#[test]
fn apply_on_then_off() {
    let mut opt = SwitchGuardBoundOption::default();
    let msg = opt.apply(true);
    assert!(opt.is_enabled());
    assert!(msg.contains("on"));
    let msg = opt.apply(false);
    assert!(!opt.is_enabled());
    assert!(msg.contains("off"));
}

#[test]
fn elem_id_in_kuna_range() {
    assert!(ELEM_SWITCHGUARDBOUND.get_id() >= 4000);
}
