//! Unit tests for the `zerofillreturn` lane boundary and option.

use super::*;

#[test]
fn scalar_double_fills_the_upper_doubleword() {
    // zext_zd: d0 written, q0+8 zeroed.
    assert_eq!(lane_end(&[(0, 8, false), (8, 8, true)]), Some(8));
    // movi d0,#0: the low lane itself is a zero.
    assert_eq!(lane_end(&[(0, 8, true), (8, 8, true)]), Some(8));
}

#[test]
fn scalar_float_fills_from_its_fourth_byte() {
    // zext_zs: s0 written, q0+4 and q0+8 zeroed.
    assert_eq!(lane_end(&[(0, 4, false), (4, 4, true), (8, 8, true)]), Some(4));
    // zext_zb: b0 written, q0+1 (seven bytes) and q0+8 zeroed.
    assert_eq!(lane_end(&[(0, 1, false), (1, 7, true), (8, 8, true)]), Some(1));
}

#[test]
fn a_conditional_select_fills_on_each_path() {
    // fcsel d0: the write and its fill on both paths of the select.
    assert_eq!(lane_end(&[(0, 8, false), (8, 8, true), (0, 8, false), (8, 8, true)]), Some(8));
}

#[test]
fn a_64_bit_vector_lane_by_lane_keeps_both_lanes() {
    // dup v0.2s: two 4-byte lanes, then the zext_zd fill.
    assert_eq!(lane_end(&[(0, 4, false), (4, 4, false), (8, 8, true)]), Some(8));
}

#[test]
fn a_128_bit_write_is_no_fill() {
    // movi v0.2d,#0 / ldr q0: one 16-byte write.
    assert_eq!(lane_end(&[(0, 16, true)]), None);
    assert_eq!(lane_end(&[(0, 16, false)]), None);
    // movi v0.4s,#0: four 4-byte lanes of zero.
    assert_eq!(lane_end(&[(0, 4, true), (4, 4, true), (8, 4, true), (12, 4, true)]), None);
}

#[test]
fn a_value_above_the_lane_is_no_fill() {
    // mov v0.d[1],x1 next to a low-lane write.
    assert_eq!(lane_end(&[(0, 8, false), (8, 8, false)]), None);
    // a write that straddles the lane's end.
    assert_eq!(lane_end(&[(0, 4, false), (2, 4, true), (8, 8, true)]), None);
}

#[test]
fn no_low_lane_means_no_fill() {
    // ins v0.d[1],xzr: only the upper doubleword, a zero.
    assert_eq!(lane_end(&[(8, 8, true)]), None);
    // a write above the fill.
    assert_eq!(lane_end(&[(0, 8, false), (8, 8, true), (12, 4, false)]), None);
    assert_eq!(lane_end(&[]), None);
}

#[test]
fn option_parses_on_and_off() {
    assert!(OptionZeroFillReturn.apply("on").unwrap().0);
    assert!(!OptionZeroFillReturn.apply("off").unwrap().0);
    assert!(OptionZeroFillReturn.apply("maybe").is_err());
}
