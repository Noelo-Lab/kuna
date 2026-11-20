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

#[test]
fn the_pad_is_the_distance_to_the_proven_pool() {
    use super::kuna_pool_pad;
    let extents = [(0x100, 0x108), (0x200, 0x204)];
    assert_eq!(kuna_pool_pad(&extents, |_| true, 0x100), Some(0), "the pool starts at the fall-through");
    assert_eq!(kuna_pool_pad(&extents, |_| true, 0x104), Some(0), "inside a run");
    assert_eq!(kuna_pool_pad(&extents, |_| true, 0xfc), Some(4), "one instruction of alignment padding");
    assert_eq!(kuna_pool_pad(&extents, |_| true, 0xfe), Some(2), "a Thumb halfword of padding");
    assert_eq!(kuna_pool_pad(&extents, |_| true, 0xf8), None, "further than the pad allowance");
    assert_eq!(kuna_pool_pad(&extents, |_| true, 0x108), None, "past the run, no pool near");
    assert_eq!(kuna_pool_pad(&extents, |_| true, 0x1fc), Some(4), "the next run");
    assert_eq!(kuna_pool_pad(&extents, |_| true, 0x204), None, "past every run");
    assert_eq!(kuna_pool_pad(&[], |_| true, 0x100), None, "no pools at all");
    assert_eq!(kuna_pool_pad(&extents, |_| false, 0x100), None, "not executable read-only image bytes");
}

#[test]
fn padding_is_an_instruction_that_does_nothing() {
    use super::kuna_is_padding_run;
    let r8 = (0, 0x20, 4);
    let r9 = (0, 0x24, 4);
    assert!(kuna_is_padding_run(std::iter::empty()), "an instruction with no p-code at all");
    assert!(kuna_is_padding_run([(OpCode::CPUI_COPY, Some(r8), &[r8][..])]), "ARM's mov r8,r8");
    assert!(!kuna_is_padding_run([(OpCode::CPUI_COPY, Some(r8), &[r9][..])]), "a real move");
    assert!(!kuna_is_padding_run([(OpCode::CPUI_CALL, None, &[r8][..])]), "a call");
    assert!(
        !kuna_is_padding_run([(OpCode::CPUI_COPY, Some(r8), &[r8][..]), (OpCode::CPUI_STORE, None, &[r8][..])]),
        "any live op in the run"
    );
}

#[test]
fn a_calls_fall_through_onto_its_pool_word_is_bounded() {
    use super::kuna_should_bound_at_pool;
    assert!(kuna_should_bound_at_pool(Some(0), true), "the pool starts at the fall-through");
    assert!(kuna_should_bound_at_pool(Some(4), true), "one instruction of alignment padding");
}

#[test]
fn each_pool_condition_alone_keeps_the_fall_through() {
    use super::kuna_should_bound_at_pool;
    assert!(!kuna_should_bound_at_pool(None, true), "no pool within the pad");
    assert!(!kuna_should_bound_at_pool(Some(8), true), "further than the pad allowance");
    assert!(!kuna_should_bound_at_pool(Some(4), false), "the gap is not padding");
}
