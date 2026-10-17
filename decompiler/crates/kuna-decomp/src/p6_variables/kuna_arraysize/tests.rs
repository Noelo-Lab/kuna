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

fn with(start: intb, ev: Evidence, unproven: Vec<intb>, reaches: Vec<Reach>) -> MapState {
    let space = stack_space();
    let mut rn = RangeList::new();
    rn.insert_range(Rc::clone(&space), 0, 0xffff);
    let mut st = MapState::new(Rc::clone(&space), &rn, &RangeList::new(), base(1, type_metatype::TYPE_UNKNOWN));
    let mut sizes = BTreeMap::new();
    sizes.insert(start, ev);
    st.set_stated_sizes(sizes, unproven, reaches);
    st
}

fn counts(c: &[(intb, Says)]) -> Evidence {
    Evidence { counts: c.iter().map(|&(b, s)| (b, s, false)).collect(), ..Default::default() }
}

fn state(start: intb, c: &[(intb, Says)]) -> MapState {
    with(start, counts(c), Vec::new(), Vec::new())
}

fn reach(offset: intb, lo: intb, hi: intb, open_up: bool, open_down: bool) -> Reach {
    Reach { offset, lo, hi, open_up, open_down }
}

fn open(start: intb, elem: int4, highind: int4) -> RangeHint {
    RangeHint::new(start as u64, elem, start, base(elem, type_metatype::TYPE_INT), 0, RangeType::Open, highind)
}

fn fits(st: &MapState, gap: intb) -> Option<int4> {
    fit(&open(0x100, 1, -1), 0x101, gap, st)
}

#[test]
fn a_library_length_inside_the_padding_ends_the_array() {
    assert_eq!(fits(&state(0x100, &[(0x2000, Says::Fills)]), 0x2008), Some(0x2000));
    assert_eq!(fits(&state(0x100, &[(0x100, Says::Object)]), 0x108), Some(0x100));
}

#[test]
fn a_length_further_short_than_the_padding_keeps_the_gap() {
    assert_eq!(fits(&state(0x100, &[(0x10, Says::Fills)]), 0x28), None);
}

#[test]
fn the_largest_count_decides_and_a_full_length_keeps_the_gap() {
    assert_eq!(fits(&state(0x100, &[(0x1a, Says::Fills), (0x20, Says::Fills)]), 0x28), Some(0x20));
    assert_eq!(fits(&state(0x100, &[(0x10, Says::Fills), (0x18, Says::Fills)]), 0x18), None);
    assert_eq!(fits(&state(0x100, &[(0x20, Says::Fills), (0x28, Says::Reads)]), 0x28), None);
    assert_eq!(fits(&state(0x100, &[(0x27, Says::Fills)]), 0x28), None);
}

#[test]
fn a_read_only_or_exact_write_length_is_a_lower_bound_only() {
    assert_eq!(fits(&state(0x100, &[(0x40, Says::Reads)]), 0x48), None);
    assert_eq!(fits(&state(0x100, &[(0x40, Says::Covers)]), 0x48), None);
    assert_eq!(fits(&state(0x100, &[(0x40, Says::Covers), (0x40, Says::Fills)]), 0x48), Some(0x40));
    assert_eq!(fits(&state(0x100, &[(0x40, Says::Fills), (0x3c, Says::Reads)]), 0x48), Some(0x40));
    assert_eq!(fits(&state(0x100, &[(0x3c, Says::Fills), (0x40, Says::Reads)]), 0x48), None);
}

#[test]
fn a_library_length_one_short_of_sixteen_keeps_the_terminator() {
    assert_eq!(fits(&state(0x100, &[(0xfff, Says::Fills)]), 0x1008), Some(0x1000));
    let terminated = Evidence { counts: vec![(0xfff, Says::Fills, true)], ..Default::default() };
    assert_eq!(fits(&with(0x100, terminated, Vec::new(), Vec::new()), 0x1008), Some(0xfff));
    assert_eq!(fits(&state(0x100, &[(0xff, Says::Object)]), 0x108), Some(0xff));
    assert_eq!(fits(&state(0x100, &[(0x1f, Says::Constant)]), 0x28), Some(0x1f));
}

#[test]
fn an_unproven_index_near_the_buffer_keeps_the_gap() {
    let ev = || counts(&[(0x3c, Says::Fills)]);
    assert_eq!(fits(&with(0x100, ev(), vec![0x110], Vec::new()), 0x48), None);
    assert_eq!(fits(&with(0x100, ev(), vec![0xf8], Vec::new()), 0x48), None);
    assert_eq!(fits(&with(0x100, ev(), vec![0xe0, 0x148], Vec::new()), 0x48), Some(0x3c));
}

#[test]
fn an_unbounded_walk_into_the_buffer_keeps_the_gap() {
    let ev = || counts(&[(0x3c, Says::Fills)]);
    assert_eq!(fits(&with(0x100, ev(), Vec::new(), vec![reach(0x100, 0x100, 0x13c, true, false)]), 0x48), None);
    assert_eq!(fits(&with(0x100, ev(), Vec::new(), vec![reach(0x13f, 0x13f, 0x140, false, true)]), 0x3f), None);
    assert_eq!(fits(&with(0x100, ev(), Vec::new(), vec![reach(0x148, 0x148, 0x149, true, false)]), 0x48), Some(0x3c));
    assert_eq!(fits(&with(0x100, ev(), Vec::new(), vec![reach(0x80, 0x80, 0x81, true, false)]), 0x48), Some(0x3c));
}

#[test]
fn constant_accesses_past_the_count_keep_the_gap() {
    let ev = || counts(&[(0x40, Says::Fills)]);
    assert_eq!(fits(&with(0x100, ev(), Vec::new(), vec![reach(0x100, 0x100, 0x144, false, false)]), 0x48), None);
    assert_eq!(fits(&with(0x100, ev(), Vec::new(), vec![reach(0x148, 0x142, 0x150, false, false)]), 0x48), None);
    assert_eq!(fits(&with(0x100, ev(), Vec::new(), vec![reach(0x100, 0x100, 0x140, false, false)]), 0x48), Some(0x40));
    let st = state(0x100, &[(0x20, Says::Fills)]);
    assert_eq!(fit(&open(0x100, 1, -1), 0x124, 0x28, &st), None);
    assert_eq!(fit(&open(0x100, 1, 0x22), 0x101, 0x28, &st), None);
    assert_eq!(fit(&open(0x100, 1, 0x1f), 0x120, 0x28, &st), Some(0x20));
}

#[test]
fn small_counts_and_partial_elements_are_not_sizes() {
    assert_eq!(fits(&state(0x100, &[(0xc, Says::Fills)]), 0x10), None);
    let st = state(0x100, &[(0x22, Says::Fills)]);
    assert_eq!(fit(&open(0x100, 4, -1), 0x104, 0x28, &st), None);
}

#[test]
fn only_the_exact_start_counts() {
    assert_eq!(fit(&open(0x108, 1, -1), 0x109, 0x20, &state(0x100, &[(0x20, Says::Fills)])), None);
}

#[test]
fn the_option_levels_parse() {
    assert_eq!(OptionArraySize.apply("off").unwrap().0, LEVEL_OFF);
    assert_eq!(OptionArraySize.apply("on").unwrap().0, LEVEL_ON);
    assert_eq!(OptionArraySize.apply("calls").unwrap().0, LEVEL_CALLS);
    assert!(OptionArraySize.apply("bogus").is_err());
}
