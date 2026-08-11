//! Unit tests for the `kuna_funcboundflow` fall-through-bound decision.

use super::kuna_should_bound_at_entry;

#[test]
fn gate_off_never_bounds() {
    assert!(!kuna_should_bound_at_entry(false, true, false));
    assert!(!kuna_should_bound_at_entry(false, false, false));
}

#[test]
fn bounds_only_at_a_foreign_function_entry() {
    // gate on, target is a known foreign function entry -> bound (the merge fix).
    assert!(kuna_should_bound_at_entry(true, true, false));
    // gate on, but the target is not a function entry -> ordinary fall-through.
    assert!(!kuna_should_bound_at_entry(true, false, false));
}

#[test]
fn self_entry_is_never_a_bound() {
    // A fall-through onto the current function's own entry must not truncate it.
    assert!(!kuna_should_bound_at_entry(true, true, true));
}
