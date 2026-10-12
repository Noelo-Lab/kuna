use super::*;

#[test]
fn option_name_and_message() {
    assert_eq!(OptionCalleePopFrame::NAME, "calleepopframe");
    let (on, msg) = OptionCalleePopFrame.apply("on").unwrap();
    assert!(on);
    assert_eq!(msg, "Fixed-frame callee-pop reading turned on");
    let (off, msg) = OptionCalleePopFrame.apply("off").unwrap();
    assert!(!off);
    assert_eq!(msg, "Fixed-frame callee-pop reading turned off");
    assert!(OptionCalleePopFrame.apply("maybe").is_err());
}
