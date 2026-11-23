//! Tests for the switch arm printing order.

use super::print_order;

#[test]
fn label_order_is_kept_when_nothing_moves() {
    assert_eq!(print_order(&[false, false, false], &[false, false, false]), vec![0, 1, 2]);
}

#[test]
fn default_prints_after_the_other_arms() {
    assert_eq!(print_order(&[false, false, false], &[true, false, false]), vec![1, 2, 0]);
    assert_eq!(print_order(&[false, false, false], &[false, true, false]), vec![0, 2, 1]);
}

#[test]
fn an_arm_that_does_not_stop_prints_last() {
    assert_eq!(print_order(&[false, true, false], &[false, false, true]), vec![0, 2, 1]);
    assert_eq!(print_order(&[true, false, false], &[true, false, false]), vec![1, 2, 0]);
    assert_eq!(print_order(&[true, false, true], &[true, false, false]), vec![1, 2, 0]);
}
