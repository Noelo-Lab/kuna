//! Tests for keeping a re-rolled switch only when the function reads no worse.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{addrspace_flags, spacetype, AddrSpace};

use super::*;

fn at(off: u64) -> Address {
    let ram = Rc::new(AddrSpace::new(spacetype::IPTR_PROCESSOR, "ram", false, 8, 1, 2, addrspace_flags::hasphysical, 1, 1));
    Address::new(ram, off)
}

fn shape(gotos: int4, params: int4, calls: &[(u64, int4)]) -> Shape {
    let mut s = Shape { gotos, params, output: 8, ..Shape::default() };
    for &(a, n) in calls {
        s.calls.entry(at(a)).or_default().insert(n);
    }
    s
}

#[test]
fn the_same_shape_is_kept() {
    let before = shape(1, 1, &[(0x10, 1)]);
    assert!(!before.clone().worse_than(&before, true));
}

#[test]
fn fewer_gotos_are_kept() {
    let before = shape(2, 1, &[]);
    assert!(!shape(1, 1, &[]).worse_than(&before, true));
}

#[test]
fn a_new_goto_is_withdrawn() {
    let before = shape(1, 1, &[]);
    assert!(shape(2, 1, &[]).worse_than(&before, true));
}

#[test]
fn another_parameter_or_call_argument_list_is_withdrawn() {
    let before = shape(0, 1, &[(0x10, 1)]);
    assert!(shape(0, 3, &[(0x10, 1)]).worse_than(&before, true));
    assert!(shape(0, 1, &[(0x10, 4)]).worse_than(&before, true));
    let mut out = before.clone();
    out.output = 16;
    assert!(out.worse_than(&before, true));
}

#[test]
fn a_switch_printed_as_a_condition_is_withdrawn() {
    let before = shape(0, 1, &[]);
    let mut after = before.clone();
    after.switch_conditions = 1;
    assert!(after.worse_than(&before, true));
}

#[test]
fn a_binary_search_tree_switch_is_judged_only_on_what_it_says_about_the_program() {
    let before = shape(1, 1, &[(0x10, 1)]);
    assert!(!shape(2, 1, &[(0x10, 1)]).worse_than(&before, false));
    assert!(shape(1, 2, &[(0x10, 1)]).worse_than(&before, false));
    assert!(shape(1, 1, &[(0x10, 3)]).worse_than(&before, false));
}

#[test]
fn a_return_found_only_with_the_switch_is_kept() {
    let mut before = shape(0, 1, &[]);
    before.output = 0;
    let mut after = before.clone();
    after.output = 4;
    assert!(!after.worse_than(&before, true));
}
