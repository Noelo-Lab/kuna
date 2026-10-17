//! Unit tests for (kuna) `arraysize`.

use std::collections::BTreeMap;
use std::rc::Rc;

use kuna_base::address::RangeList;
use kuna_base::space::{addrspace_flags, spacetype, AddrSpace};
use kuna_base::types::{int4, intb};

use super::*;
use crate::dtype::{type_metatype, Datatype};
use crate::varmap::RangeType;

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

fn state(start: intb, counts: &[(intb, bool)]) -> MapState {
    let space = stack_space();
    let mut rn = RangeList::new();
    rn.insert_range(Rc::clone(&space), 0, 0xffff);
    let mut st = MapState::new(Rc::clone(&space), &rn, &RangeList::new(), base(1, type_metatype::TYPE_UNKNOWN));
    let mut sizes = BTreeMap::new();
    sizes.insert(start, counts.iter().map(|&(bytes, library)| Stated { bytes, library }).collect());
    st.set_stated_sizes(sizes);
    st
}

fn open(start: intb, elem: int4, highind: int4) -> RangeHint {
    RangeHint::new(start as u64, elem, start, base(elem, type_metatype::TYPE_INT), 0, RangeType::Open, highind)
}

#[test]
fn a_library_length_inside_the_padding_ends_the_array() {
    let st = state(0x100, &[(0x2000, true)]);
    assert_eq!(fit(&open(0x100, 1, -1), 0x101, 0x2008, &st), Some(0x2000));
}

#[test]
fn a_length_further_short_than_the_padding_keeps_the_gap() {
    let st = state(0x100, &[(0x10, true)]);
    assert_eq!(fit(&open(0x100, 1, -1), 0x101, 0x28, &st), None);
}

#[test]
fn a_length_at_or_past_the_gap_keeps_the_gap() {
    let st = state(0x100, &[(0x28, true), (0x30, true)]);
    assert_eq!(fit(&open(0x100, 1, -1), 0x101, 0x28, &st), None);
}

#[test]
fn a_length_one_short_of_the_gap_keeps_the_terminator() {
    let st = state(0x100, &[(0x27, true)]);
    assert_eq!(fit(&open(0x100, 1, -1), 0x101, 0x28, &st), None);
}

#[test]
fn a_library_length_one_short_of_sixteen_keeps_the_terminator() {
    let st = state(0x100, &[(0xfff, true)]);
    assert_eq!(fit(&open(0x100, 1, -1), 0x101, 0x1008, &st), Some(0x1000));
    let st = state(0x100, &[(0x1f, false)]);
    assert_eq!(fit(&open(0x100, 1, -1), 0x101, 0x28, &st), Some(0x1f));
}

#[test]
fn small_counts_and_partial_elements_are_not_sizes() {
    let st = state(0x100, &[(0xc, true)]);
    assert_eq!(fit(&open(0x100, 1, -1), 0x101, 0x10, &st), None);
    let st = state(0x100, &[(0x22, true)]);
    assert_eq!(fit(&open(0x100, 4, -1), 0x104, 0x28, &st), None);
}

#[test]
fn absorbed_hints_and_index_bounds_are_kept() {
    let st = state(0x100, &[(0x20, true)]);
    assert_eq!(fit(&open(0x100, 1, -1), 0x124, 0x28, &st), None);
    assert_eq!(fit(&open(0x100, 1, 0x22), 0x101, 0x28, &st), None);
    assert_eq!(fit(&open(0x100, 1, 0x1f), 0x120, 0x28, &st), Some(0x20));
}

#[test]
fn the_largest_count_wins_and_only_at_the_exact_start() {
    let st = state(0x100, &[(0x1a, true), (0x20, true)]);
    assert_eq!(fit(&open(0x100, 1, -1), 0x101, 0x28, &st), Some(0x20));
    assert_eq!(fit(&open(0x108, 1, -1), 0x109, 0x20, &st), None);
}

#[test]
fn the_option_levels_parse() {
    assert_eq!(OptionArraySize.apply("off").unwrap().0, LEVEL_OFF);
    assert_eq!(OptionArraySize.apply("on").unwrap().0, LEVEL_ON);
    assert_eq!(OptionArraySize.apply("calls").unwrap().0, LEVEL_CALLS);
    assert!(OptionArraySize.apply("bogus").is_err());
}
