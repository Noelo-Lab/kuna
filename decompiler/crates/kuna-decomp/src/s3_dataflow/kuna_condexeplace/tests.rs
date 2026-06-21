//! Tests for the `condexeplace` option toggle (kuna GH-9203).  The behavioral
//! fix is an anchor edit in `ActionConditionalConst::handlePhiNodes`
//! (coreaction.cc) ported with the W-coreaction wave; this module owns only the
//! gate.

use super::*;

#[test]
fn default_is_on_div3() {
    // Shipped default `option condexeplace on` (kuna DIV-3 default-on).
    assert!(CondExePlaceOption::default().is_enabled());
}

#[test]
fn apply_toggles_and_reports() {
    let mut opt = CondExePlaceOption::default();
    assert_eq!(
        opt.apply(false),
        "Conditional-constant loop-block placement guard turned off"
    );
    assert!(!opt.is_enabled());
    assert_eq!(
        opt.apply(true),
        "Conditional-constant loop-block placement guard turned on"
    );
    assert!(opt.is_enabled());
}
