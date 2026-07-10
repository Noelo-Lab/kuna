//! Tests for the `kuna_arraynotation` port: the `OptionArrayNotation::apply`
//! on/off parse + confirmation message, the invalid-toggle error, and the
//! interaction with the `PrintCOptions` toggle the caller flips.

use super::*;
use crate::printc::PrintCOptions;

#[test]
fn arraynotation_apply_on() {
    let opt = OptionArrayNotation;
    let (val, msg) = opt.apply("on").unwrap();
    assert!(val);
    assert_eq!(msg, "Array notation for pointer arithmetic turned on");
}

#[test]
fn arraynotation_apply_off() {
    let opt = OptionArrayNotation;
    let (val, msg) = opt.apply("off").unwrap();
    assert!(!val);
    assert_eq!(msg, "Array notation for pointer arithmetic turned off");
}

#[test]
fn arraynotation_apply_empty_defaults_on() {
    // onOrOff("") == true (C++ ArchOption::onOrOff empty -> on).
    let opt = OptionArrayNotation;
    let (val, _) = opt.apply("").unwrap();
    assert!(val);
}

#[test]
fn arraynotation_apply_invalid_errors() {
    let opt = OptionArrayNotation;
    let err = opt.apply("maybe").unwrap_err();
    assert!(format!("{err}").contains("Must specify toggle value"));
}

#[test]
fn arraynotation_caller_flips_printc_toggle() {
    // The apply()'s resolved flag is what the caller writes into PrintCOptions
    // via set_array_notation (the C++ lng->setArrayNotation(val)).
    let opt = OptionArrayNotation;
    let mut po = PrintCOptions::new();
    // DIV-2 default-on.
    assert!(po.array_notation());

    let (val, _) = opt.apply("off").unwrap();
    po.set_array_notation(val);
    assert!(!po.array_notation());

    let (val, _) = opt.apply("on").unwrap();
    po.set_array_notation(val);
    assert!(po.array_notation());
}
