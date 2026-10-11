use super::*;
use crate::{context::OpId, heritage::LoadGuard};
use kuna_base::space::{spacetype, AddrSpace};

fn partial_guard() -> LoadGuard {
    let space = Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "stack",
        false,
        4,
        1,
        1,
        0,
        0,
        0,
    ));
    let mut guard = LoadGuard::set(OpId::default(), &space, 0xffff_f000);
    guard.minimum_offset = 0xfffe_f000;
    guard.maximum_offset = 0xffff_ffff;
    guard.analysis_state = 1;
    guard
}

#[test]
fn partial_signed_range_guards_the_array_base() {
    let guard = partial_guard();
    for offset in [0, 4, 8, 12] {
        assert!(window_overlaps(
            &guard,
            4,
            None,
            guard.pointer_base + offset,
            4
        ));
    }
    assert!(window_overlaps(&guard, 4, None, guard.pointer_base - 4, 8));
    assert!(!window_overlaps(&guard, 4, None, guard.minimum_offset, 4));
    assert!(!window_overlaps(
        &guard,
        4,
        None,
        guard.pointer_base + 16,
        4
    ));
}

#[test]
fn provisional_minimum_above_the_base_keeps_four_elements() {
    let mut guard = partial_guard();
    for minimum in [guard.pointer_base + 4, guard.pointer_base + 16] {
        guard.minimum_offset = minimum;
        for offset in [0, 4, 8, 12] {
            assert!(window_overlaps(&guard, 4, None, minimum + offset, 4));
        }
        assert!(!window_overlaps(&guard, 4, None, minimum - 4, 4));
        assert!(!window_overlaps(&guard, 4, None, minimum + 16, 4));
    }
    guard.maximum_offset = guard.minimum_offset + 3;
    assert!(window_overlaps(
        &guard,
        4,
        None,
        guard.minimum_offset + 4,
        3
    ));
    assert!(!window_overlaps(
        &guard,
        4,
        None,
        guard.minimum_offset + 7,
        1
    ));
}

#[test]
fn provisional_range_before_the_base_keeps_a_nonempty_window() {
    let mut guard = partial_guard();
    guard.minimum_offset = guard.pointer_base - 0x2000;
    guard.maximum_offset = guard.minimum_offset + 0xfff;
    for width in [1, 4, 8] {
        for element in 0..4 {
            assert!(window_overlaps(
                &guard,
                width,
                None,
                guard.minimum_offset + element * width as u64,
                width
            ));
        }
        assert!(!window_overlaps(
            &guard,
            width,
            None,
            guard.minimum_offset + 4 * width as u64,
            width
        ));
        assert!(!window_overlaps(
            &guard,
            width,
            None,
            guard.pointer_base,
            width
        ));
    }
}

#[test]
fn locked_or_bounded_ranges_retain_negative_indices() {
    let mut guard = partial_guard();
    guard.minimum_offset = guard.pointer_base - 8;
    guard.maximum_offset = guard.pointer_base + 7;
    guard.analysis_state = 2;
    assert!(window_overlaps(&guard, 4, None, guard.pointer_base - 8, 4));
    assert!(!window_overlaps(
        &guard,
        4,
        None,
        guard.pointer_base - 12,
        4
    ));
    guard.analysis_state = 1;
    let bound = Some((guard.pointer_base - 8, guard.pointer_base + 7));
    assert!(window_overlaps(&guard, 4, bound, guard.pointer_base - 8, 4));
    assert!(!window_overlaps(
        &guard,
        4,
        bound,
        guard.pointer_base + 8,
        4
    ));
}
