//! Unit tests for the `foldcallretphi` option parse.  The self-effect discount
//! [`conflict_is_self_call_effect`](super::conflict_is_self_call_effect) is
//! exercised end-to-end by the stage test
//! `tests/stages/kuna-foldcallretphi.xml` (two-pass: off = the spilled temp,
//! on = the folded call).

use super::*;

#[test]
fn option_parse_on_off() {
    let (v, msg) = OptionFoldCallRetPhi.apply("on").unwrap();
    assert!(v);
    assert!(msg.contains("on"));
    let (v, msg) = OptionFoldCallRetPhi.apply("off").unwrap();
    assert!(!v);
    assert!(msg.contains("off"));
}

#[test]
fn option_parse_rejects_garbage() {
    assert!(OptionFoldCallRetPhi.apply("maybe").is_err());
}

#[test]
fn option_name_is_foldcallretphi() {
    assert_eq!(OptionFoldCallRetPhi::NAME, "foldcallretphi");
}

