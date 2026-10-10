use super::*;

#[test]
fn option_name_and_message() {
    assert_eq!(OptionCalleePopSlot::NAME, "calleepopslot");
    let (on, msg) = OptionCalleePopSlot.apply("on").unwrap();
    assert!(on);
    assert_eq!(msg, "Callee-pop slot tracking turned on");
    let (off, msg) = OptionCalleePopSlot.apply("off").unwrap();
    assert!(!off);
    assert_eq!(msg, "Callee-pop slot tracking turned off");
    assert!(OptionCalleePopSlot.apply("maybe").is_err());
}
