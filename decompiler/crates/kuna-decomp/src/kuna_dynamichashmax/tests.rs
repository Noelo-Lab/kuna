// Tests for the `kuna_dynamichashmax` port (included into `mod tests` in
// kuna_dynamichashmax.rs).
//
// Covered:
//   - the option decision body (`OptionDynamicHashMax::apply`): on/off flips
//     `enabled` and returns the confirmation message.
//   - the resolved same-address collision budget: 8 off / 16 on (the
//     `maxduplicates` value `DynamicHash::unique_hash` reads, dynamic.cc:433).
//   - default state is `off` (the neutral option-object default).

use super::*;

#[test]
fn default_is_off_budget_eight() {
    let opt = DynamicHashMaxOption::default();
    assert!(!opt.is_enabled());
    assert_eq!(opt.max_duplicates(), 8);
}

#[test]
fn apply_on_raises_budget_to_sixteen() {
    let mut opt = DynamicHashMaxOption::default();
    let msg = opt.apply(true);
    assert!(opt.is_enabled());
    assert_eq!(opt.max_duplicates(), 16);
    assert_eq!(msg, "DynamicHash wide collision budget turned on");
}

#[test]
fn apply_off_restores_budget_eight() {
    let mut opt = DynamicHashMaxOption { enabled: true };
    let msg = opt.apply(false);
    assert!(!opt.is_enabled());
    assert_eq!(opt.max_duplicates(), 8);
    assert_eq!(msg, "DynamicHash wide collision budget turned off");
}

#[test]
fn budget_is_exactly_eight_or_sixteen() {
    // The 3-bit total/position encoding fits up to 16 collisions exactly; the
    // option toggles only between the two documented budgets.
    let off = DynamicHashMaxOption { enabled: false };
    let on = DynamicHashMaxOption { enabled: true };
    assert_eq!(off.max_duplicates(), 8);
    assert_eq!(on.max_duplicates(), 16);
}
