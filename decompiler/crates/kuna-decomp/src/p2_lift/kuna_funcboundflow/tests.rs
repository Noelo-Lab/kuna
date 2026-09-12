//! Unit tests for the `kuna_funcboundflow` fall-through-bound decision.

use kuna_num::opcodes::OpCode;

use super::{kuna_is_unconditional_return, kuna_should_bound_at_entry};

#[test]
fn only_an_unconditional_return_is_safe_to_admit() {
    assert!(kuna_is_unconditional_return([
        OpCode::CPUI_COPY,
        OpCode::CPUI_RETURN,
    ]));
    assert!(!kuna_is_unconditional_return([
        OpCode::CPUI_CBRANCH,
        OpCode::CPUI_RETURN,
    ]));
    assert!(!kuna_is_unconditional_return([
        OpCode::CPUI_BRANCH,
        OpCode::CPUI_RETURN,
    ]));
    assert!(!kuna_is_unconditional_return([
        OpCode::CPUI_BRANCHIND,
        OpCode::CPUI_RETURN,
    ]));
    assert!(!kuna_is_unconditional_return([OpCode::CPUI_COPY]));
}

#[test]
fn gate_off_never_bounds() {
    assert!(!kuna_should_bound_at_entry(false, true, false, false));
    assert!(!kuna_should_bound_at_entry(false, false, false, false));
}

#[test]
fn bounds_only_at_a_foreign_function_entry() {
    // gate on, target is a known foreign function entry -> bound (the merge fix).
    assert!(kuna_should_bound_at_entry(true, true, false, false));
    // gate on, but the target is not a function entry -> ordinary fall-through.
    assert!(!kuna_should_bound_at_entry(true, false, false, false));
}

#[test]
fn self_entry_is_never_a_bound() {
    // A fall-through onto the current function's own entry must not truncate it.
    assert!(!kuna_should_bound_at_entry(true, true, true, false));
}

#[test]
fn foreign_return_entry_is_the_only_admitted_boundary_instruction() {
    // A separately callable one-instruction return can also close the current
    // function: admitting it cannot flow into anything after that function.
    assert!(!kuna_should_bound_at_entry(true, true, false, true));

    // The same foreign entry without a RETURN classification remains bounded.
    assert!(kuna_should_bound_at_entry(true, true, false, false));
}
