//! Unit tests for the `calleedeadarg` callee entry-liveness probe.

use super::*;
use crate::p0_knowledge::options::KUNA_OPTION_NAMES;

#[test]
fn option_parses_on_and_off() {
    assert!(OptionCalleeDeadArg.apply("on").unwrap().0);
    assert!(!OptionCalleeDeadArg.apply("off").unwrap().0);
    assert!(OptionCalleeDeadArg.apply("maybe").is_err());
}

#[test]
fn option_is_registered() {
    assert!(KUNA_OPTION_NAMES.contains(&OptionCalleeDeadArg::NAME));
}

#[test]
fn incomplete_summary_proves_nothing() {
    let d = CalleeEntryDead::default();
    assert!(!d.is_complete());
}
