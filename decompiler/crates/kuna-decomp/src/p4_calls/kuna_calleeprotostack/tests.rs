//! Unit tests for [`super`] — the declared-callee stack contract.

use super::*;

#[test]
fn above_callee_frame_reads_the_offset_as_signed() {
    // A 32-bit stack space: 0xffffffec is -20, not 4294967276.
    assert!(!above_callee_frame(0xffffffec, 4, 4, 4));
    assert!(above_callee_frame(0x4, 4, 4, 4));
    assert!(above_callee_frame(0x14, 4, 4, 4));
}

#[test]
fn above_callee_frame_keeps_the_return_address_and_parameter_area() {
    // floor 20 == a four-parameter __stdcall: slots 0 (return address) through
    // 16 (the last parameter) stay guarded, 20 and up do not.
    assert!(!above_callee_frame(0x0, 4, 4, 20));
    assert!(!above_callee_frame(0x10, 4, 4, 20));
    assert!(above_callee_frame(0x14, 4, 4, 20));
}

#[test]
fn declared_caller_frame_floor_declines_without_evidence() {
    let mut fp = FuncProto::new();
    // Unlocked, no model: no claim.
    assert_eq!(declared_caller_frame_floor(true, &fp), None);
    fp.set_extra_pop(20);
    // Still unlocked.
    assert_eq!(declared_caller_frame_floor(true, &fp), None);
    fp.set_input_lock(true);
    assert_eq!(declared_caller_frame_floor(true, &fp), Some(20));
    // The option is the outermost gate.
    assert_eq!(declared_caller_frame_floor(false, &fp), None);
    // A variadic callee reads above its declared parameters.
    fp.set_dotdotdot(true);
    assert_eq!(declared_caller_frame_floor(true, &fp), None);
}

#[test]
fn resolve_declared_extra_pop_leaves_a_model_that_states_its_own() {
    let mut fp = FuncProto::new();
    fp.set_input_lock(true);
    fp.set_extra_pop(4); // as a __cdecl model would
    resolve_declared_extra_pop(true, &mut fp);
    assert_eq!(fp.get_extra_pop(), 4);
}

#[test]
fn resolve_declared_extra_pop_needs_the_lock() {
    let mut fp = FuncProto::new();
    fp.set_extra_pop(EXTRAPOP_UNKNOWN);
    resolve_declared_extra_pop(true, &mut fp);
    assert_eq!(fp.get_extra_pop(), EXTRAPOP_UNKNOWN);
}
