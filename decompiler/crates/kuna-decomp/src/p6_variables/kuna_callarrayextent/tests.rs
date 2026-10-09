use super::*;
use crate::varmap::TYPELOCK;

#[test]
fn equivalent_integer_parameter_types_can_share_storage() {
    let first = Datatype::new_with_align(4, 4, type_metatype::TYPE_UINT);
    let second = Datatype::new_with_align(4, 4, type_metatype::TYPE_UINT);
    let signed = Datatype::new_with_align(4, 4, type_metatype::TYPE_INT);
    assert!(same_integer_type(&first, &second));
    assert!(!same_integer_type(&first, &signed));
}

#[test]
fn frame_capacity_rejects_wrapping_and_nonlocal_ranges() {
    assert_eq!(capacity(-48, 0, 4), Some(12));
    assert_eq!(capacity(-88, -48, 4), Some(10));
    assert_eq!(capacity(-48, -8, 4), Some(10));
    for (start, end, width) in [
        (0, 48, 4),
        (-48, 4, 4),
        (-4, 0, 4),
        (-4097, 0, 1),
        (-47, 0, 4),
        (i64::MIN, 0, 4),
        (-48, 0, 0),
    ] {
        assert_eq!(capacity(start, end, width), None, "{start}..{end}, {width}");
    }
}

#[test]
fn declared_types_and_crossing_accesses_block_coalescing() {
    let int = Rc::new(Datatype::new_with_align(4, 4, type_metatype::TYPE_UINT));
    let mut hint = RangeHint::new((-40i64) as u64, 4, -40, int, 0, RangeType::Fixed, -1);
    assert!(compatible(&hint, -48, 0, 4));
    hint.flags |= TYPELOCK;
    assert!(!compatible(&hint, -48, 0, 4));
    hint.flags = 0;
    assert!(!compatible(&hint, -38, 0, 4));
    assert!(!compatible(&hint, -48, -38, 4));
    hint.range_type = RangeType::Open;
    hint.highind = 3;
    assert!(compatible(&hint, -48, 0, 4));
    assert_eq!(minimum_end(&hint), Some(-24));
    assert!(!compatible(&hint, -48, -28, 4));
    hint.range_type = RangeType::Fixed;
    hint.type_ = Rc::new(Datatype::new_with_align(4, 4, type_metatype::TYPE_FLOAT));
    assert!(!compatible(&hint, -48, 0, 4));
    hint.type_ = Rc::new(Datatype::new_with_align(4, 4, type_metatype::TYPE_STRUCT));
    assert!(!compatible(&hint, -48, 0, 4));
}

#[test]
fn indexed_parent_keeps_its_entire_minimum_extent() {
    let ty = Rc::new(Datatype::new_with_align(4, 4, type_metatype::TYPE_UINT));
    let hint = RangeHint::new((-92i64) as u64, 4, -92, ty, 0, RangeType::Open, 12);
    assert_eq!(minimum_end(&hint), Some(-40));
    assert!(!compatible(&hint, -48, 0, 4));
    assert!(!compatible(&hint, -92, -48, 4));
    assert!(compatible(&hint, -92, 0, 4));
}

#[test]
fn settled_array_parent_can_end_at_the_call_output() {
    let ty = Rc::new(Datatype::new_with_align(4, 4, type_metatype::TYPE_UINT));
    let hint = RangeHint::new((-92i64) as u64, 44, -92, ty, 0, RangeType::Fixed, -1);
    assert_eq!(minimum_end(&hint), Some(-48));
    assert!(compatible(&hint, -92, -48, 4));

    let ty = Rc::new(Datatype::new_with_align(4, 4, type_metatype::TYPE_UINT));
    let hint = RangeHint::new((-92i64) as u64, 4, -92, ty, 0, RangeType::Open, 10);
    assert_eq!(minimum_end(&hint), Some(-48));
    assert!(compatible(&hint, -92, -48, 4));

    let ty = Rc::new(Datatype::new_with_align(4, 4, type_metatype::TYPE_UINT));
    let hint = RangeHint::new((-92i64) as u64, 4, -92, ty, 0, RangeType::Open, 12);
    assert_eq!(minimum_end(&hint), Some(-40));
}

#[test]
fn inferred_parent_stops_at_the_passed_address() {
    let ty = || Rc::new(Datatype::new_with_align(4, 4, type_metatype::TYPE_UINT));
    let mut hints = vec![RangeHint::new(
        (-92i64) as u64,
        4,
        -92,
        ty(),
        0,
        RangeType::Open,
        -1,
    )];
    for start in (-88..=-40).step_by(4) {
        hints.push(RangeHint::new(
            start as u64,
            4,
            start,
            ty(),
            0,
            RangeType::Fixed,
            -1,
        ));
    }
    assert_eq!(joined_parent_end(&hints, 0, 4, -48), Some(-48));
    hints[5].flags |= TYPELOCK;
    assert_eq!(joined_parent_end(&hints, 0, 4, -48), Some(-72));

    let unknown8 = Rc::new(Datatype::new_with_align(8, 8, type_metatype::TYPE_UNKNOWN));
    let hints = vec![
        RangeHint::new((-88i64) as u64, 4, -88, ty(), 0, RangeType::Open, 3),
        RangeHint::new(
            (-72i64) as u64,
            8,
            -72,
            Rc::clone(&unknown8),
            COPY_CONSTANT,
            RangeType::Fixed,
            -1,
        ),
        RangeHint::new(
            (-64i64) as u64,
            8,
            -64,
            unknown8,
            COPY_CONSTANT,
            RangeType::Fixed,
            -1,
        ),
    ];
    assert_eq!(joined_parent_end(&hints, 0, 4, -64), Some(-64));
    assert!(compatible_parent_hint(&hints[1], -88, -56, 4));
    assert!(!compatible(&hints[1], -88, -56, 4));
}

#[test]
fn a_single_unindexed_open_hint_is_not_a_parent() {
    let ty = Rc::new(Datatype::new_with_align(4, 4, type_metatype::TYPE_UINT));
    let mut hint = RangeHint::new((-52i64) as u64, 4, -52, ty, 0, RangeType::Open, -1);
    assert!(!is_parent_root(&hint, false));
    assert!(is_parent_root(&hint, true));
    hint.highind = 1;
    assert!(is_parent_root(&hint, false));
}
