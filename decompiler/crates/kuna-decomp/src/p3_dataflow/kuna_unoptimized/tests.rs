use super::*;

fn ev(spilled: u32, stored: u32, live: u32, frame_pointer: u32, callee_saved: u32) -> Evidence {
    Evidence { spilled, stored, live, frame_pointer, callee_saved }
}

#[test]
fn a_parameter_in_its_register_means_optimized() {
    assert!(!ev(3, 0, 1, 9, 0).unoptimized());
    assert!(!ev(0, 0, 1, 0, 0).unoptimized());
}

#[test]
fn parameters_in_the_frame_mean_unoptimized() {
    assert!(ev(2, 0, 0, 11, 0).unoptimized());
    assert!(ev(1, 1, 0, 0, 0).unoptimized());
    assert!(ev(1, 0, 0, 0, 1).unoptimized());
}

#[test]
fn two_reused_callee_saved_registers_mean_optimized() {
    assert!(!ev(2, 0, 0, 0, 2).unoptimized());
}

#[test]
fn without_parameters_a_frame_pointer_decides() {
    assert!(ev(0, 0, 0, 4, 0).unoptimized());
    assert!(!ev(0, 0, 0, 4, 1).unoptimized());
    assert!(!ev(0, 0, 0, 0, 0).unoptimized());
    assert!(!ev(0, 1, 0, 4, 0).unoptimized());
}

#[test]
fn sign_extension_follows_the_constant_width() {
    assert_eq!(sext(0xfffffff8, 4), -8);
    assert_eq!(sext(0x8, 4), 8);
    assert_eq!(sext(0xffff_ffff_ffff_ffe8, 8), -24);
    assert_eq!(sext(0xfc, 1), -4);
}
