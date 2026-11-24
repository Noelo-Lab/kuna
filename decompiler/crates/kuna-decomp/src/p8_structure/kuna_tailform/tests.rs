//! Tests for the `tailform` decision function and its option value.

use super::*;

fn site(shape: TailShape, void: bool, lines: usize) -> TailSite {
    TailSite { shape, void, lines, feats: None }
}

#[test]
fn a_short_guarded_tail_is_wrapped() {
    let p = TailFormPolicy::default();
    assert_eq!(decide(&site(TailShape::Guard, true, 1), &p), TailDecision::Wrap);
    assert_eq!(decide(&site(TailShape::Guard, false, DEFAULT_MAX_LINES as usize), &p), TailDecision::Wrap);
}

#[test]
fn a_long_guarded_tail_stays_a_guard() {
    let p = TailFormPolicy::default();
    assert_eq!(decide(&site(TailShape::Guard, true, DEFAULT_MAX_LINES as usize + 1), &p), TailDecision::Leave);
}

#[test]
fn a_long_wrapped_body_becomes_a_guard() {
    let p = TailFormPolicy::default();
    assert_eq!(decide(&site(TailShape::Wrapped, true, DEFAULT_MAX_LINES as usize + 1), &p), TailDecision::Guard);
    assert_eq!(decide(&site(TailShape::Wrapped, false, 2), &p), TailDecision::Leave);
}

#[test]
fn void_and_valued_tails_use_their_own_threshold() {
    let p = TailFormPolicy { void_max: 1, value_max: 6, ..Default::default() };
    assert_eq!(decide(&site(TailShape::Guard, true, 2), &p), TailDecision::Leave);
    assert_eq!(decide(&site(TailShape::Guard, false, 6), &p), TailDecision::Wrap);
    assert_eq!(decide(&site(TailShape::Wrapped, true, 2), &p), TailDecision::Guard);
}

#[test]
fn the_option_takes_off_on_one_or_two_thresholds() {
    let o = OptionTailForm;
    assert_eq!(o.apply("off").unwrap().0, None);
    assert_eq!(o.apply("on").unwrap().0, Some(TailFormPolicy::default()));
    let one = o.apply("3").unwrap().0.unwrap();
    assert_eq!((one.void_max, one.value_max), (3, 3));
    let two = o.apply("2,5").unwrap().0.unwrap();
    assert_eq!((two.void_max, two.value_max), (2, 5));
    assert!(o.apply("wide").is_err());
    assert!(o.apply("2,").is_err());
    assert_eq!(o.apply("wrap").unwrap().0.unwrap().rule, TailRule::Wrap);
    assert_eq!(o.apply("guard").unwrap().0.unwrap().rule, TailRule::Guard);
    assert_eq!(o.apply("model").unwrap().0.unwrap().rule, TailRule::Model);
}

#[test]
fn forced_rules_wrap_or_guard_every_tail() {
    let wrap = TailFormPolicy { rule: TailRule::Wrap, ..Default::default() };
    let guard = TailFormPolicy { rule: TailRule::Guard, ..Default::default() };
    assert_eq!(decide(&site(TailShape::Guard, true, 40), &wrap), TailDecision::Wrap);
    assert_eq!(decide(&site(TailShape::Wrapped, false, 1), &guard), TailDecision::Guard);
}
