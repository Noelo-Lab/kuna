use super::*;

#[test]
fn option_name_and_message() {
    assert_eq!(OptionCalleePop::NAME, "calleepop");
    let (on, msg) = OptionCalleePop.apply("on").unwrap();
    assert!(on);
    assert_eq!(msg, "Callee-pop argument recovery turned on");
    let (off, msg) = OptionCalleePop.apply("off").unwrap();
    assert!(!off);
    assert_eq!(msg, "Callee-pop argument recovery turned off");
    assert!(OptionCalleePop.apply("maybe").is_err());
}

#[test]
fn upstream_guess_is_the_return_address_only() {
    assert_eq!(RETURN_ADDRESS_ONLY, 4);
}
