use super::*;

#[test]
fn option_parses_on_and_off_only() {
    assert!(OptionPltLocal.apply("on").unwrap().0);
    assert!(!OptionPltLocal.apply("off").unwrap().0);
    assert!(OptionPltLocal.apply("stub").is_err());
    assert_eq!(OptionPltLocal::NAME, "pltlocal");
}

#[test]
fn a_stub_maps_to_its_definition_and_nothing_else_does() {
    let table = [(0x10380, 0x10344), (0x10390, 0x10354)];
    assert_eq!(target_of(&table, 0x10380), Some(0x10344));
    assert_eq!(target_of(&table, 0x10390), Some(0x10354));
    assert_eq!(target_of(&table, 0x10344), None);
    assert_eq!(target_of(&[], 0x10380), None);
}
