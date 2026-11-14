use super::*;

#[test]
fn the_option_reads_on_and_off() {
    assert!(OptionCalleeArgs.apply("on").unwrap().0);
    assert!(!OptionCalleeArgs.apply("off").unwrap().0);
    assert!(OptionCalleeArgs.apply("maybe").is_err());
}
