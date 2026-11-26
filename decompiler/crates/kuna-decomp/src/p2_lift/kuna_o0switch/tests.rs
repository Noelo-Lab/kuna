use super::*;

fn t(addr: u64, jumps: Option<bool>, root: u64) -> (u64, Test) {
    (addr, Test { jumps_on_match: jumps, root: Some(root) })
}

#[test]
fn every_test_jumping_on_its_match_is_a_switch() {
    let tests = vec![t(0x10, Some(true), 0x0c), t(0x16, Some(true), 0x12)];
    assert_eq!(form(tests), Some(Form::Switch));
}

#[test]
fn a_test_falling_into_its_body_before_the_last_is_an_if_chain() {
    let tests = vec![t(0x10, Some(false), 0x0c), t(0x20, Some(false), 0x1c)];
    assert_eq!(form(tests), Some(Form::IfChain));
    let or_then_else_if = vec![t(0x10, Some(true), 0x0c), t(0x16, Some(false), 0x12), t(0x30, Some(false), 0x2c)];
    assert_eq!(form(or_then_else_if), Some(Form::IfChain));
}

#[test]
fn the_last_test_may_fall_into_the_first_case_body() {
    let three = vec![t(0x10, Some(true), 0x0c), t(0x16, None, 0x12), t(0x1c, Some(true), 0x18), t(0x22, Some(false), 0x1e)];
    assert_eq!(form(three), Some(Form::Switch));
    let two_reloaded = vec![t(0x10, Some(true), 0x0c), t(0x16, Some(false), 0x12)];
    assert_eq!(form(two_reloaded), None);
    let two_one_load = vec![t(0x10, Some(true), 0x0c), t(0x16, Some(false), 0x0c)];
    assert_eq!(form(two_one_load), Some(Form::Switch));
}

#[test]
fn the_order_is_the_address_order() {
    let tests = vec![t(0x22, Some(false), 0x1e), t(0x10, Some(true), 0x0c), t(0x1c, Some(true), 0x18)];
    assert_eq!(form(tests), Some(Form::Switch));
    let tests = vec![t(0x22, Some(true), 0x1e), t(0x10, Some(false), 0x0c), t(0x1c, Some(true), 0x18)];
    assert_eq!(form(tests), Some(Form::IfChain));
}

#[test]
fn fewer_than_two_equality_tests_decide_nothing() {
    assert_eq!(form(vec![t(0x10, Some(true), 0x0c), t(0x16, None, 0x12)]), None);
}
