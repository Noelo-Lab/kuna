//! Checks for the conservative stack-write preservation gate.

use super::*;

#[test]
fn default_is_off_byte_identical() {
    // Shipped default `option stackalias off` (upstream byte-identical).
    assert!(!StackAliasOption::default().is_enabled());
}

#[test]
fn apply_toggles_and_reports() {
    let mut opt = StackAliasOption::default();
    assert_eq!(opt.apply(true), "Stack-alias dead-store preservation turned on");
    assert!(opt.is_enabled());
    assert_eq!(opt.apply(false), "Stack-alias dead-store preservation turned off");
    assert!(!opt.is_enabled());
}
