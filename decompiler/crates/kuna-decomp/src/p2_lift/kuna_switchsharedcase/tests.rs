//! Logic-level tests for the `switchsharedcase` ArchOption gate (angr port).
//!
//! The recovery walk `JumpBasic::kuna_try_loop_carried_guard_table` lives in
//! `jumptable.rs` next to the rest of the model; this file covers the gate this
//! module owns. End-to-end behaviour is exercised by
//! `tests/stages/switchsharedcase-b2sum.xml`.

use super::*;

#[test]
fn option_default_is_off() {
    // Shipped default: option switchsharedcase off (upstream byte-identical).
    let opt = SwitchSharedCaseOption::default();
    assert!(!opt.is_enabled());
}

#[test]
fn apply_on_then_off() {
    let mut opt = SwitchSharedCaseOption::default();
    let msg = opt.apply(true);
    assert!(opt.is_enabled());
    assert!(msg.contains("on"));
    let msg = opt.apply(false);
    assert!(!opt.is_enabled());
    assert!(msg.contains("off"));
}

#[test]
fn elem_id_in_kuna_range() {
    assert!(ELEM_SWITCHSHAREDCASE.get_id() >= 4000);
}
