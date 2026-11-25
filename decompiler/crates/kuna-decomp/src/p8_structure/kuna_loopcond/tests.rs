use super::*;

#[test]
fn the_mode_picks_the_rule() {
    assert_eq!(LoopCondHoist::Off.rule(true), None);
    assert_eq!(LoopCondHoist::Auto.rule(false), None);
    assert_eq!(LoopCondHoist::Auto.rule(true), Some(Rule::Layout));
    assert_eq!(LoopCondHoist::On.rule(true), Some(Rule::Layout));
    assert_eq!(LoopCondHoist::On.rule(false), Some(Rule::Prefer));
}

#[test]
fn a_head_laid_out_after_the_body_is_the_condition() {
    let c = condition(Rule::Layout, (Some(0x40), true), &[(Some(0x20), false), (Some(0x30), true)]);
    assert_eq!(c, Some(Cond::Head));
    assert_eq!(condition(Rule::Layout, (Some(0x40), false), &[(Some(0x20), false)]), None);
}

#[test]
fn a_head_laid_out_first_has_its_condition_on_the_latch() {
    assert_eq!(condition(Rule::Layout, (Some(0x10), true), &[(Some(0x30), true)]), Some(Cond::Latch(0)));
    assert_eq!(condition(Rule::Layout, (Some(0x10), true), &[(Some(0x30), false)]), None);
    assert_eq!(condition(Rule::Layout, (Some(0x10), true), &[(Some(0x30), true), (Some(0x38), true)]), None);
    assert_eq!(condition(Rule::Layout, (None, true), &[(Some(0x30), true)]), None);
}

#[test]
fn optimized_code_prefers_a_conditional_latch() {
    assert_eq!(condition(Rule::Prefer, (Some(0x10), true), &[(Some(0x30), true)]), Some(Cond::Latch(0)));
    assert_eq!(condition(Rule::Prefer, (Some(0x10), true), &[(Some(0x30), false)]), Some(Cond::Head));
    assert_eq!(condition(Rule::Prefer, (Some(0x10), false), &[(Some(0x30), false)]), None);
    assert_eq!(condition(Rule::Prefer, (None, true), &[(None, true), (None, true)]), None);
}

#[test]
fn the_option_parses_its_values() {
    assert_eq!(OptionLoopCondHoist.apply("off").unwrap().0, LoopCondHoist::Off);
    assert_eq!(OptionLoopCondHoist.apply("on").unwrap().0, LoopCondHoist::On);
    assert_eq!(OptionLoopCondHoist.apply("auto").unwrap().0, LoopCondHoist::Auto);
    assert!(OptionLoopCondHoist.apply("sideways").is_err());
}
