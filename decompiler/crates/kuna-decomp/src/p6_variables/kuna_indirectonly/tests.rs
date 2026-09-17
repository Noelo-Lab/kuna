//! Unit tests for the `indirectonly` port.

use super::*;
use crate::options::KUNA_OPTION_NAMES;

#[test]
fn option_name_is_registered() {
    assert!(KUNA_OPTION_NAMES.contains(&OptionIndirectOnly::NAME));
}

#[test]
fn option_parses_on_and_off() {
    let (v, msg) = OptionIndirectOnly.apply("on").unwrap();
    assert!(v);
    assert!(msg.contains("on"));
    let (v, msg) = OptionIndirectOnly.apply("off").unwrap();
    assert!(!v);
    assert!(msg.contains("off"));
    assert!(OptionIndirectOnly.apply("maybe").is_err());
}
