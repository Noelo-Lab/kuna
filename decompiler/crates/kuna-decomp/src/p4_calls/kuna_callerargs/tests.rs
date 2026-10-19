//! Unit tests for the agreement rule. The end-to-end witness is the CLI probe
//! `tests/cli/arm-parameter-every-caller-extends-is-narrow.json` and its
//! `narrowargs on` twin.

use super::*;

use kuna_base::space::{spacetype, AddrSpace};

fn r1() -> Address {
    let space = Rc::new(AddrSpace::new(spacetype::IPTR_PROCESSOR, "register", false, 4, 1, 1, 0, 0, 0));
    Address::new(space, 0x24)
}

fn arg(unsigned: int4, signed: int4, zext: bool, sext: bool, truth: bool, constant: bool) -> Arg {
    Arg { addr: r1(), size: 4, unsigned, signed, zext, sext, truth, constant }
}

#[test]
fn a_zero_extension_and_a_fitting_constant_agree_on_unsigned() {
    let (a, b) = (arg(1, 0, true, false, false, false), arg(1, 1, false, false, false, true));
    assert_eq!(agree(&[&a, &b]), Some((1, Ext::Unsigned)));
}

#[test]
fn the_widest_call_sets_the_width() {
    let (a, b) = (arg(1, 0, true, false, false, false), arg(2, 0, false, false, false, true));
    assert_eq!(agree(&[&a, &b]), Some((2, Ext::Unsigned)));
}

#[test]
fn a_sign_extension_and_a_negative_constant_agree_on_signed() {
    let (a, b) = (arg(0, 2, false, true, false, false), arg(0, 1, false, false, false, true));
    assert_eq!(agree(&[&a, &b]), Some((2, Ext::Signed)));
}

#[test]
fn constants_alone_state_nothing() {
    let (a, b) = (arg(1, 1, false, false, false, true), arg(2, 0, false, false, false, true));
    assert_eq!(agree(&[&a, &b]), None);
}

#[test]
fn a_zero_and_a_sign_extension_state_nothing() {
    let (a, b) = (arg(1, 2, true, false, false, false), arg(0, 1, false, true, false, false));
    assert_eq!(agree(&[&a, &b]), None);
}

#[test]
fn a_wide_call_states_nothing() {
    let (a, b) = (arg(1, 0, true, false, false, false), arg(0, 0, false, false, false, false));
    assert_eq!(agree(&[&a, &b]), None);
}

#[test]
fn truth_values_with_one_computed_are_bool() {
    let (a, b) = (arg(1, 1, false, false, true, false), arg(1, 1, false, false, true, true));
    assert_eq!(agree(&[&a, &b]), Some((1, Ext::Bool)));
    assert_eq!(agree(&[&b, &b]), None);
}
