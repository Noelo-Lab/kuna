//! Logic-level tests for the `switchmodbound` ArchOption gate (GH-9191).
//!
//! The recovery walk `JumpBasic::kunaTryModuloBoundTable` lives in
//! `jumptable.cc` and ports with the W4 jumptable wave (see the module docs);
//! this file covers the gate this module owns.

use super::*;

#[test]
fn option_default_is_off() {
    // Shipped default: option switchmodbound off (upstream byte-identical;
    // architecture.cc:1440 sets switch_modulo_bound = false).
    let opt = SwitchModBoundOption::default();
    assert!(!opt.is_enabled());
}

#[test]
fn apply_on_then_off() {
    let mut opt = SwitchModBoundOption::default();
    let msg = opt.apply(true);
    assert!(opt.is_enabled());
    assert!(msg.contains("on"));
    let msg = opt.apply(false);
    assert!(!opt.is_enabled());
    assert!(msg.contains("off"));
}
