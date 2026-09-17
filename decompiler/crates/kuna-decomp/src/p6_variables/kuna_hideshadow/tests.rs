//! Unit tests for the `hideshadow` option parse.  The pass itself is covered
//! end-to-end by `tests/stages/kuna-hideshadow.xml` (two-pass: off = the two
//! copies, on = one variable).

use super::*;

#[test]
fn option_parses_on_and_off() {
    let (v, msg) = OptionHideShadow.apply("on").unwrap();
    assert!(v);
    assert!(msg.contains("on"), "{msg}");
    let (v, msg) = OptionHideShadow.apply("off").unwrap();
    assert!(!v);
    assert!(msg.contains("off"), "{msg}");
}

#[test]
fn option_rejects_a_bad_value() {
    assert!(OptionHideShadow.apply("sometimes").is_err());
}

#[test]
fn option_name_is_hideshadow() {
    assert_eq!(OptionHideShadow::NAME, "hideshadow");
}
