use super::*;

#[test]
fn apply_parses_each_value_and_reports() {
    let (val, msg) = OptionTruthyCond.apply("on").unwrap();
    assert_eq!(val, TruthyCond::On);
    assert_eq!(msg, "Truthy condition rendering turned on");
    let (val, msg) = OptionTruthyCond.apply("off").unwrap();
    assert_eq!(val, TruthyCond::Off);
    assert_eq!(msg, "Truthy condition rendering turned off");
    let (val, _) = OptionTruthyCond.apply("ptrchain").unwrap();
    assert_eq!(val, TruthyCond::PtrChain);
    assert_eq!(OptionTruthyCond.apply("").unwrap().0, TruthyCond::On);
    assert!(OptionTruthyCond.apply("sideways").is_err());
}

#[test]
fn elem_id_is_in_the_kuna_range() {
    assert_eq!(ELEM_TRUTHYCOND.get_id(), 4115);
    assert_eq!(OptionTruthyCond::NAME, "truthycond");
}

#[test]
fn set_truthy_mode_sets_both_flags() {
    let mut o = crate::printc::PrintCOptions::new();
    o.set_truthy_mode(TruthyCond::On);
    assert!(o.truthy_cond && !o.truthy_ptr_chain);
    o.set_truthy_mode(TruthyCond::Off);
    assert!(!o.truthy_cond && !o.truthy_ptr_chain);
    o.set_truthy_mode(TruthyCond::PtrChain);
    assert!(o.truthy_cond && o.truthy_ptr_chain);
}
