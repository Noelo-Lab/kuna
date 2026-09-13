//! Unit tests for (kuna) `nulterminator`.

use std::rc::Rc;

use kuna_base::address::RangeList;
use kuna_base::space::{addrspace_flags, spacetype, AddrSpace};
use kuna_base::types::{int4, intb, uintb};

use super::*;
use crate::dtype::{Datatype, TypeFactory, TypeFactoryImpl};

fn stack_space() -> Rc<AddrSpace> {
    Rc::new(AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "stack",
        false,
        8,
        1,
        4,
        addrspace_flags::hasphysical,
        1,
        1,
    ))
}

fn base(size: int4, meta: type_metatype) -> Rc<Datatype> {
    Rc::new(Datatype::new_with_align(size, size, meta))
}

fn state() -> MapState {
    let space = stack_space();
    let factory = TypeFactoryImpl::new();
    factory.set_default_alignment_map();
    factory.set_max_basetype_size(8);
    let dflt = factory.get_base(1, type_metatype::TYPE_UNKNOWN).unwrap();
    let mut rn = RangeList::new();
    rn.insert_range(Rc::clone(&space), 0, 0x100);
    MapState::new(space, &rn, &RangeList::new(), dflt)
}

fn open_array(start: uintb, elem: Rc<Datatype>) -> RangeHint {
    let size = elem.get_size();
    RangeHint::new(start, size, start as intb, elem, 0, RangeType::Open, 3)
}

fn zero_store(start: uintb, size: int4) -> RangeHint {
    RangeHint::new(
        start,
        size,
        start as intb,
        base(size, type_metatype::TYPE_UNKNOWN),
        COPY_CONSTANT,
        RangeType::Fixed,
        -1,
    )
}

#[test]
fn an_unread_zero_past_the_index_floor_is_absorbed() {
    let mut st = state();
    st.note_terminator_store(0x6f, 1, true);
    let cur = open_array(0x30, base(1, type_metatype::TYPE_UINT));
    assert!(absorbs_terminator(&cur, &zero_store(0x6f, 1), &st));
}

#[test]
fn nothing_is_absorbed_with_the_option_off() {
    let st = state();
    let cur = open_array(0x30, base(1, type_metatype::TYPE_UINT));
    assert!(!absorbs_terminator(&cur, &zero_store(0x6f, 1), &st));
}

#[test]
fn one_read_or_nonzero_write_disqualifies_the_slot() {
    let mut st = state();
    st.note_terminator_store(0x6f, 1, true);
    st.note_terminator_store(0x6f, 1, false);
    let cur = open_array(0x30, base(1, type_metatype::TYPE_INT));
    assert!(!absorbs_terminator(&cur, &zero_store(0x6f, 1), &st));
}

#[test]
fn the_store_must_be_one_element_at_a_whole_element_distance() {
    let mut st = state();
    st.note_terminator_store(0x6c, 4, true);
    st.note_terminator_store(0x6f, 1, true);
    st.note_terminator_store(0x6e, 2, true);
    let bytes = open_array(0x30, base(1, type_metatype::TYPE_UINT));
    assert!(!absorbs_terminator(&bytes, &zero_store(0x6c, 4), &st));
    let wide = open_array(0x30, base(2, type_metatype::TYPE_UINT));
    assert!(!absorbs_terminator(&wide, &zero_store(0x6f, 1), &st));
    assert!(absorbs_terminator(&wide, &zero_store(0x6e, 2), &st));
    let mut odd = state();
    odd.note_terminator_store(0x6f, 2, true);
    assert!(!absorbs_terminator(&wide, &zero_store(0x6f, 2), &odd));
}

#[test]
fn a_zeroed_word_after_a_word_buffer_is_not_a_terminator() {
    let mut st = state();
    st.note_terminator_store(0x20, 8, true);
    st.note_terminator_store(0x20, 4, true);
    let words = open_array(0x10, base(8, type_metatype::TYPE_INT));
    assert!(!absorbs_terminator(&words, &zero_store(0x20, 8), &st));
    let ints = open_array(0x10, base(4, type_metatype::TYPE_UINT));
    assert!(!absorbs_terminator(&ints, &zero_store(0x20, 4), &st));
}

#[test]
fn only_an_open_unlocked_integer_array_absorbs() {
    let mut st = state();
    st.note_terminator_store(0x6f, 1, true);
    let next = zero_store(0x6f, 1);

    let mut fixed = open_array(0x30, base(1, type_metatype::TYPE_UINT));
    fixed.range_type = RangeType::Fixed;
    assert!(!absorbs_terminator(&fixed, &next, &st));

    let mut locked = open_array(0x30, base(1, type_metatype::TYPE_UINT));
    locked.flags = crate::varmap::TYPELOCK;
    assert!(!absorbs_terminator(&locked, &next, &st));

    let unknown = open_array(0x30, base(1, type_metatype::TYPE_UNKNOWN));
    assert!(!absorbs_terminator(&unknown, &next, &st));

    let flags = open_array(0x30, base(1, type_metatype::TYPE_BOOL));
    assert!(!absorbs_terminator(&flags, &next, &st));
}

#[test]
fn the_follower_must_be_a_plain_constant_copy() {
    let mut st = state();
    st.note_terminator_store(0x6f, 1, true);
    let cur = open_array(0x30, base(1, type_metatype::TYPE_UINT));

    let mut read = zero_store(0x6f, 1);
    read.flags = 0;
    assert!(!absorbs_terminator(&cur, &read, &st));

    let mut open = zero_store(0x6f, 1);
    open.range_type = RangeType::Open;
    assert!(!absorbs_terminator(&cur, &open, &st));

    let mut typed = zero_store(0x6f, 1);
    typed.type_ = base(1, type_metatype::TYPE_STRUCT);
    assert!(!absorbs_terminator(&cur, &typed, &st));
}
