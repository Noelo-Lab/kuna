//! Unit tests for [`super`] — the by-value slot arithmetic.

use super::*;

#[test]
fn slots_are_four_byte_aligned() {
    assert_eq!(align4(1), 4);
    assert_eq!(align4(4), 4);
    assert_eq!(align4(5), 8);
    assert_eq!(align4(8), 8);
}

#[test]
fn a_range_inside_the_object_is_built_by_its_constructor() {
    assert!(within(0, 4, 4));
    assert!(within(4, 4, 8));
    assert!(within(0, 8, 8));
}

#[test]
fn a_range_reaching_past_the_object_is_not() {
    assert!(!within(4, 4, 4));
    assert!(!within(0, 8, 4));
    assert!(!within(u64::MAX - 1, 4, 8));
    assert!(!within(0, 0, 4));
}

#[test]
fn the_option_is_on_or_off() {
    assert!(OptionMsvcByval.apply("on").unwrap().0);
    assert!(!OptionMsvcByval.apply("off").unwrap().0);
    assert!(OptionMsvcByval.apply("maybe").is_err());
}
