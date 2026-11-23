//! Unit tests for the variadic register-save prologue fold.

use super::*;

#[test]
fn full_save_area_from_xmm0() {
    let saves: Vec<(u32, i64)> = (0..8).map(|j| (j, -0x88 + 16 * j as i64)).collect();
    assert_eq!(save_area(&saves), Some(-0x88 - 48));
}

#[test]
fn partial_save_area_after_named_doubles() {
    let saves: Vec<(u32, i64)> = (1..8).map(|j| (j, -0xb0 + 48 + 16 * j as i64)).collect();
    assert_eq!(save_area(&saves), Some(-0xb0));
    assert_eq!(save_area(&[(0, 0x20)]), Some(0x20 - 48));
}

#[test]
fn save_area_refuses_holes_and_strays() {
    assert_eq!(save_area(&[(0, 0x50), (2, 0x70)]), None);
    assert_eq!(save_area(&[(0, 0x50), (1, 0x68)]), None);
    assert_eq!(save_area(&[]), None);
}

#[test]
fn general_registers_must_be_a_run() {
    assert!(contiguous([1u32, 2, 3, 4, 5].into_iter()));
    assert!(contiguous(std::iter::empty()));
    assert!(!contiguous([1u32, 3].into_iter()));
}

#[test]
fn slot_overlap_is_half_open() {
    let slots = [(0x28, 0x30), (0x50, 0x60)];
    assert!(overlaps(0x28, 4, &slots));
    assert!(overlaps(0x2c, 8, &slots));
    assert!(overlaps(0x58, 8, &slots));
    assert!(!overlaps(0x20, 8, &slots));
    assert!(!overlaps(0x30, 0x20, &slots));
}

#[test]
fn offsets_shift_and_merge() {
    let sp = Offsets {
        at: vec![0],
        wide: false,
    };
    let frame = sp.shifted(-0xd8);
    assert_eq!(frame.single(), Some(-0xd8));
    let mut phi = frame.clone();
    assert!(!phi.merge(&frame));
    assert!(phi.merge(&frame.shifted(8)));
    assert_eq!(phi.single(), None);
    assert_eq!(phi.at, vec![-0xd8, -0xd0]);
}

#[test]
fn offsets_widen_past_the_cap_and_stay_wide() {
    let mut walk = Offsets {
        at: vec![0],
        wide: false,
    };
    for step in 1..=OFFSET_CAP as i64 {
        walk.merge(&Offsets {
            at: vec![8 * step],
            wide: false,
        });
    }
    assert!(walk.wide);
    let seen = walk.at.clone();
    assert!(!walk.merge(&Offsets {
        at: vec![0x1000],
        wide: false
    }));
    assert_eq!(walk.at, seen);
    let mut other = Offsets {
        at: vec![0],
        wide: false,
    };
    assert!(other.merge(&walk));
    assert!(other.wide);
}
