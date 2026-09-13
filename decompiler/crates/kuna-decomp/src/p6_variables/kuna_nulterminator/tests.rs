//! Unit tests for (kuna) `nulterminator`.

use std::rc::Rc;

use kuna_base::address::RangeList;
use kuna_base::space::{addrspace_flags, spacetype, AddrSpace};
use kuna_base::types::{int4, intb, uintb};

use super::*;
use crate::dtype::{Datatype, TypeFactoryImpl};
use crate::varmap::{ScopeLocal, TYPELOCK};

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

fn factory() -> TypeFactoryImpl {
    let factory = TypeFactoryImpl::new();
    factory.set_default_alignment_map();
    factory.set_max_basetype_size(8);
    factory
}

fn window(space: &Rc<AddrSpace>) -> RangeList {
    let mut rn = RangeList::new();
    rn.insert_range(Rc::clone(space), 0, 0xff);
    rn
}

fn state() -> MapState {
    let space = stack_space();
    MapState::new(Rc::clone(&space), &window(&space), &RangeList::new(), base(1, type_metatype::TYPE_UNKNOWN))
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

fn fixed(start: uintb, size: int4) -> RangeHint {
    RangeHint::new(start, size, start as intb, base(size, type_metatype::TYPE_INT), 0, RangeType::Fixed, -1)
}

fn char_array() -> RangeHint {
    open_array(0x30, base(1, type_metatype::TYPE_UINT))
}

#[test]
fn a_zero_at_the_array_end_closes_the_array_one_element_later() {
    let mut st = state();
    st.note_terminator_store(0x6f, 1, true);
    let closed = close_at_terminator(&char_array(), &zero_store(0x6f, 1), Some(&fixed(0x70, 8)), &st)
        .expect("an unread zero at the end of an open char array terminates it");
    assert_eq!(closed.sstart, 0x30);
    assert_eq!(closed.size, 0x40);
    assert_eq!(closed.range_type, RangeType::Fixed);
}

#[test]
fn the_closed_array_never_reaches_the_padding_after_its_terminator() {
    let mut st = state();
    st.note_terminator_store(0x51, 1, true);
    let closed = close_at_terminator(&char_array(), &zero_store(0x51, 1), Some(&fixed(0x58, 8)), &st)
        .expect("terminator");
    assert_eq!(closed.size, 0x22, "34 elements, not the 40 the following hint would give");
}

#[test]
fn a_utf16_unit_terminates_a_utf16_array() {
    let mut st = state();
    st.note_terminator_store(0x4e, 2, true);
    let wide = open_array(0x30, base(2, type_metatype::TYPE_UINT));
    let closed =
        close_at_terminator(&wide, &zero_store(0x4e, 2), Some(&fixed(0x50, 4)), &st).expect("terminator");
    assert_eq!(closed.size, 0x20);
}

#[test]
fn nothing_closes_without_terminator_evidence() {
    let st = state();
    assert!(close_at_terminator(&char_array(), &zero_store(0x6f, 1), Some(&fixed(0x70, 8)), &st).is_none());
}

#[test]
fn one_read_or_nonzero_write_disqualifies_the_slot() {
    let mut st = state();
    st.note_terminator_store(0x6f, 1, true);
    st.note_terminator_store(0x6f, 1, false);
    assert!(close_at_terminator(&char_array(), &zero_store(0x6f, 1), Some(&fixed(0x70, 8)), &st).is_none());
}

/// A single non-terminator write to a slot poisons it for good, regardless of
/// which write came last: `is_terminator_store` folds with AND, not last-wins.
/// A terminator (an unread zero) followed by a non-terminator write (a later
/// non-zero, or a read) must NOT read back as a terminator, and neither must
/// the reverse order.
#[test]
fn any_non_terminator_write_poisons_the_slot_in_either_order() {
    let mut early_bad = state();
    early_bad.note_terminator_store(0x6f, 1, false);
    early_bad.note_terminator_store(0x6f, 1, true);
    assert!(!early_bad.is_terminator_store(0x6f, 1), "a later zero cannot rescue a poisoned slot");

    let mut late_bad = state();
    late_bad.note_terminator_store(0x6f, 1, true);
    late_bad.note_terminator_store(0x6f, 1, false);
    assert!(!late_bad.is_terminator_store(0x6f, 1), "a later non-terminator write poisons the slot");

    let mut clean = state();
    clean.note_terminator_store(0x6f, 1, true);
    clean.note_terminator_store(0x6f, 1, true);
    assert!(clean.is_terminator_store(0x6f, 1), "only if every write is an unread zero");
}

#[test]
fn another_hint_over_the_slot_disqualifies_it() {
    let mut st = state();
    st.note_terminator_store(0x6f, 1, true);
    let next = zero_store(0x6f, 1);
    let address_taken = RangeHint::new(0x6f, 1, 0x6f, base(1, type_metatype::TYPE_INT), 0, RangeType::Open, -1);
    assert!(close_at_terminator(&char_array(), &next, Some(&address_taken), &st).is_none());
    assert!(close_at_terminator(&char_array(), &next, Some(&fixed(0x6f, 4)), &st).is_none());
    assert!(close_at_terminator(&char_array(), &next, None, &st).is_none());
    let mut wide_st = state();
    wide_st.note_terminator_store(0x4e, 2, true);
    let wide = open_array(0x30, base(2, type_metatype::TYPE_UINT));
    assert!(close_at_terminator(&wide, &zero_store(0x4e, 2), Some(&fixed(0x4f, 1)), &wide_st).is_none());
}

#[test]
fn the_store_must_be_exactly_one_element_at_a_whole_element_distance() {
    let mut st = state();
    st.note_terminator_store(0x6c, 4, true);
    st.note_terminator_store(0x6e, 2, true);
    st.note_terminator_store(0x6f, 1, true);
    st.note_terminator_store(0x6f, 2, true);
    let after = fixed(0x78, 8);
    let bytes = char_array();
    assert!(close_at_terminator(&bytes, &zero_store(0x6c, 4), Some(&after), &st).is_none());
    assert!(close_at_terminator(&bytes, &zero_store(0x6e, 2), Some(&after), &st).is_none());
    let wide = open_array(0x30, base(2, type_metatype::TYPE_UINT));
    assert!(close_at_terminator(&wide, &zero_store(0x6f, 1), Some(&after), &st).is_none());
    assert!(close_at_terminator(&wide, &zero_store(0x6f, 2), Some(&after), &st).is_none());
    assert!(close_at_terminator(&wide, &zero_store(0x6e, 2), Some(&after), &st).is_some());
}

#[test]
fn a_zeroed_slot_after_a_wider_array_is_not_a_terminator() {
    let mut st = state();
    st.note_terminator_store(0x20, 8, true);
    st.note_terminator_store(0x20, 4, true);
    let after = fixed(0x28, 8);
    let words = open_array(0x10, base(8, type_metatype::TYPE_INT));
    assert!(close_at_terminator(&words, &zero_store(0x20, 8), Some(&after), &st).is_none());
    let ints = open_array(0x10, base(4, type_metatype::TYPE_UINT));
    assert!(close_at_terminator(&ints, &zero_store(0x20, 4), Some(&after), &st).is_none());
}

#[test]
fn only_an_open_unlocked_integer_array_is_terminated() {
    let mut st = state();
    st.note_terminator_store(0x6f, 1, true);
    let next = zero_store(0x6f, 1);
    let after = fixed(0x70, 8);

    let mut closed_already = char_array();
    closed_already.range_type = RangeType::Fixed;
    assert!(close_at_terminator(&closed_already, &next, Some(&after), &st).is_none());

    let mut locked = char_array();
    locked.flags = TYPELOCK;
    assert!(close_at_terminator(&locked, &next, Some(&after), &st).is_none());

    let unknown = open_array(0x30, base(1, type_metatype::TYPE_UNKNOWN));
    assert!(close_at_terminator(&unknown, &next, Some(&after), &st).is_none());

    let flags = open_array(0x30, base(1, type_metatype::TYPE_BOOL));
    assert!(close_at_terminator(&flags, &next, Some(&after), &st).is_none());
}

#[test]
fn the_terminator_must_be_a_plain_constant_copy_past_the_array_start() {
    let mut st = state();
    st.note_terminator_store(0x6f, 1, true);
    st.note_terminator_store(0x30, 1, true);
    let after = fixed(0x70, 8);

    let mut read = zero_store(0x6f, 1);
    read.flags = 0;
    assert!(close_at_terminator(&char_array(), &read, Some(&after), &st).is_none());

    let mut locked = zero_store(0x6f, 1);
    locked.flags = COPY_CONSTANT | TYPELOCK;
    assert!(close_at_terminator(&char_array(), &locked, Some(&after), &st).is_none());

    let mut open = zero_store(0x6f, 1);
    open.range_type = RangeType::Open;
    assert!(close_at_terminator(&char_array(), &open, Some(&after), &st).is_none());

    let mut typed = zero_store(0x6f, 1);
    typed.type_ = base(1, type_metatype::TYPE_STRUCT);
    assert!(close_at_terminator(&char_array(), &typed, Some(&after), &st).is_none());

    assert!(close_at_terminator(&char_array(), &zero_store(0x30, 1), Some(&fixed(0x31, 1)), &st).is_none());
}

/// The recovered layout of one `restructure` sweep over `hints`, as sorted
/// `(offset, size)` pairs.  `zeros` are the constant-zero slots nothing reads.
fn layout(hints: &[RangeHint], zeros: &[(uintb, int4)]) -> Vec<(uintb, int4)> {
    let types = factory();
    let space = stack_space();
    let mut scope = ScopeLocal::new(0x1234, Rc::clone(&space), "frame", 8).expect("scope");
    let rn = window(&space);
    scope.reset_local_window(&rn, &RangeList::new(), false);
    let mut st = MapState::new(Rc::clone(&space), &rn, &RangeList::new(), base(1, type_metatype::TYPE_UNKNOWN));
    for h in hints {
        let ct = if h.range_type == RangeType::Open { Some(Rc::clone(&h.type_)) } else { None };
        match h.range_type {
            RangeType::Open => st.add_range_pub(h.start, ct, h.flags, RangeType::Open, h.highind),
            _ => st.add_fixed_type_pub(h.start, Rc::clone(&h.type_), h.flags, &types),
        }
    }
    for &(start, size) in zeros {
        st.note_terminator_store(start, size, true);
    }
    scope.restructure(&mut st, &types).expect("restructure");
    let mut out: Vec<(uintb, int4)> = scope
        .mapped_symbol_specs()
        .into_iter()
        .map(|(_, ct, addr, _)| (addr.get_offset(), ct.get_size()))
        .collect();
    out.sort();
    out
}

fn chars_at(start: uintb) -> RangeHint {
    open_array(start, base(1, type_metatype::TYPE_INT))
}

fn read_char(start: uintb) -> RangeHint {
    RangeHint::new(start, 1, start as intb, base(1, type_metatype::TYPE_INT), 0, RangeType::Fixed, -1)
}

#[test]
fn sweep_the_terminator_joins_the_array_and_the_padding_stays_unmapped() {
    let hints = [chars_at(0x10), zero_store(0x31, 1), fixed(0x38, 4)];
    assert_eq!(layout(&hints, &[]), vec![(0x10, 0x21), (0x31, 1), (0x38, 4)]);
    assert_eq!(layout(&hints, &[(0x31, 1)]), vec![(0x10, 0x22), (0x38, 4)]);
}

#[test]
fn sweep_a_zero_two_elements_past_the_new_end_stays_separate() {
    let hints = [chars_at(0x10), zero_store(0x1f, 1), zero_store(0x21, 1), fixed(0x28, 4)];
    let zeros = [(0x1f, 1), (0x21, 1)];
    assert_eq!(layout(&hints, &zeros), vec![(0x10, 0x10), (0x21, 1), (0x28, 4)]);
}

#[test]
fn sweep_an_adjacent_second_zero_stays_separate() {
    let hints = [chars_at(0x10), zero_store(0x1f, 1), zero_store(0x20, 1), fixed(0x28, 4)];
    let zeros = [(0x1f, 1), (0x20, 1)];
    assert_eq!(layout(&hints, &zeros), vec![(0x10, 0x10), (0x20, 1), (0x28, 4)]);
}

#[test]
fn sweep_a_zero_after_alignment_padding_stays_separate() {
    let hints = [chars_at(0x10), zero_store(0x2d, 1), zero_store(0x30, 1), fixed(0x34, 4)];
    let zeros = [(0x2d, 1), (0x30, 1)];
    assert_eq!(layout(&hints, &zeros), vec![(0x10, 0x1e), (0x30, 1), (0x34, 4)]);
}

#[test]
fn sweep_a_zero_local_that_is_read_later_ends_the_array_before_it() {
    let hints = [chars_at(0x10), read_char(0x1f), zero_store(0x1f, 1), fixed(0x28, 4)];
    assert_eq!(layout(&hints, &[(0x1f, 1)]), vec![(0x10, 0xf), (0x1f, 1), (0x28, 4)]);
}

#[test]
fn sweep_an_address_taken_at_the_slot_keeps_it_separate() {
    let hints = [chars_at(0x10), zero_store(0x1f, 1), chars_at(0x1f), fixed(0x28, 4)];
    let expected = layout(&hints, &[]);
    assert_eq!(layout(&hints, &[(0x1f, 1)]), expected);
    assert!(expected.iter().any(|&(start, _)| start == 0x1f));
}

#[test]
fn sweep_a_wider_element_array_is_unchanged() {
    let ints = open_array(0x10, base(4, type_metatype::TYPE_INT));
    let hints = [ints, zero_store(0x20, 4), fixed(0x28, 4)];
    assert_eq!(layout(&hints, &[(0x20, 4)]), layout(&hints, &[]));
}

#[test]
fn sweep_a_closed_array_that_would_not_fit_falls_back_to_upstream() {
    let types = factory();
    let space = stack_space();
    let mut scope = ScopeLocal::new(0x1234, Rc::clone(&space), "frame", 8).expect("scope");
    let rn = window(&space);
    scope.reset_local_window(&rn, &RangeList::new(), false);
    let at = kuna_base::address::Address::new(Rc::clone(&space), 0x1f);
    scope
        .add_symbol("pinned", base(1, type_metatype::TYPE_INT), &at, &kuna_base::address::Address::new_invalid())
        .expect("pinned symbol");
    let mut st = MapState::new(Rc::clone(&space), &rn, &RangeList::new(), base(1, type_metatype::TYPE_UNKNOWN));
    st.add_range_pub(0x10, Some(base(1, type_metatype::TYPE_INT)), 0, RangeType::Open, 3);
    st.add_fixed_type_pub(0x20, base(1, type_metatype::TYPE_UNKNOWN), COPY_CONSTANT, &types);
    st.add_fixed_type_pub(0x28, base(4, type_metatype::TYPE_INT), 0, &types);
    st.note_terminator_store(0x20, 1, true);
    scope.restructure(&mut st, &types).expect("restructure");
    let mut out: Vec<(uintb, int4)> = scope
        .mapped_symbol_specs()
        .into_iter()
        .map(|(_, ct, addr, _)| (addr.get_offset(), ct.get_size()))
        .collect();
    out.sort();
    assert_eq!(out, vec![(0x10, 0xf), (0x1f, 1), (0x20, 1), (0x28, 4)]);
}

#[test]
fn sweep_an_address_inside_the_array_keeps_upstream_layout() {
    let ch = base(1, type_metatype::TYPE_INT);
    let pointer = |start: uintb, highind: int4| {
        RangeHint::new(start, 1, start as intb, Rc::clone(&ch), 0, RangeType::Open, highind)
    };
    let stored = RangeHint::new(0x12, 1, 0x12, Rc::clone(&ch), 0, RangeType::Fixed, -1);
    let zeros = [(0x11, 1), (0x13, 1)];
    let two_literals = [pointer(0x10, -1), zero_store(0x11, 1), stored.clone(), pointer(0x12, -1), zero_store(0x13, 1), fixed(0x18, 4)];
    assert_eq!(layout(&two_literals, &[]), vec![(0x10, 3), (0x13, 1), (0x18, 4)]);
    assert_eq!(layout(&two_literals, &zeros), vec![(0x10, 3), (0x13, 1), (0x18, 4)]);
    let one_literal = [pointer(0x10, -1), zero_store(0x11, 1), stored, zero_store(0x13, 1), fixed(0x18, 4)];
    assert_eq!(layout(&one_literal, &zeros), vec![(0x10, 4), (0x18, 4)]);
}

#[test]
fn sweep_an_indexed_hint_inside_the_array_is_element_arithmetic() {
    let ch = base(1, type_metatype::TYPE_UINT);
    let pointer = |start: uintb, highind: int4| {
        RangeHint::new(start, 1, start as intb, Rc::clone(&ch), 0, RangeType::Open, highind)
    };
    let zeros = [(0x17, 1)];
    let next_element = [pointer(0x10, 3), pointer(0x11, 3), zero_store(0x17, 1), fixed(0x18, 4)];
    assert_eq!(layout(&next_element, &[]), vec![(0x10, 7), (0x17, 1), (0x18, 4)]);
    assert_eq!(layout(&next_element, &zeros), vec![(0x10, 8), (0x18, 4)]);
    let address = [pointer(0x10, 3), pointer(0x11, -1), zero_store(0x17, 1), fixed(0x18, 4)];
    assert_eq!(layout(&address, &zeros), vec![(0x10, 7), (0x17, 1), (0x18, 4)]);
}
