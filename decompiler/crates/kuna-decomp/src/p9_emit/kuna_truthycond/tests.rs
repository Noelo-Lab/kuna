use super::*;

#[test]
fn apply_parses_on_off_and_reports() {
    let (val, msg) = OptionTruthyCond.apply("on").unwrap();
    assert!(val);
    assert_eq!(msg, "Truthy condition rendering turned on");
    let (val, msg) = OptionTruthyCond.apply("off").unwrap();
    assert!(!val);
    assert_eq!(msg, "Truthy condition rendering turned off");
    assert!(OptionTruthyCond.apply("sideways").is_err());
}

#[test]
fn elem_id_is_in_the_kuna_range() {
    assert_eq!(ELEM_TRUTHYCOND.get_id(), 4115);
    assert_eq!(OptionTruthyCond::NAME, "truthycond");
}
