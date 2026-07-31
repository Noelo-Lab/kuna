use super::*;

#[test]
fn apply_parses_on_off_and_reports() {
    let (val, msg) = OptionBraceElide.apply("on").unwrap();
    assert!(val);
    assert_eq!(msg, "Single-statement if-body brace elision turned on");
    let (val, msg) = OptionBraceElide.apply("off").unwrap();
    assert!(!val);
    assert_eq!(msg, "Single-statement if-body brace elision turned off");
    assert!(OptionBraceElide.apply("maybe").is_err());
}

#[test]
fn elem_id_is_in_the_kuna_range() {
    assert_eq!(ELEM_BRACEELIDE.get_id(), 4116);
    assert_eq!(OptionBraceElide::NAME, "braceelide");
}
