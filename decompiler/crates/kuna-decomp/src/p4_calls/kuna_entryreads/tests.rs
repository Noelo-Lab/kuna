//! Unit tests for the range arithmetic and the statement fold of
//! [`crate::kuna_entryreads`]. The end-to-end behaviour is
//! `tests/stages/kuna-bodyreads.xml` and
//! `tests/cli/passthrough-keeps-an-argument-the-callee-body-takes.json`.

use super::*;

use kuna_base::space::{spacetype, AddrSpace};
use std::rc::Rc;

fn reg_space() -> Rc<AddrSpace> {
    Rc::new(AddrSpace::new(spacetype::IPTR_PROCESSOR, "register", false, 8, 1, 3, 0, 0, 0))
}

fn at(sp: &Rc<AddrSpace>, off: u64) -> Address {
    Address::new(Rc::clone(sp), off)
}

#[test]
fn overlaps_is_byte_range_intersection() {
    let r = reg_space();
    let ranges = vec![(at(&r, 0x30), 4)];
    assert!(overlaps(&ranges, &at(&r, 0x30), 8));
    assert!(overlaps(&ranges, &at(&r, 0x33), 1));
    assert!(!overlaps(&ranges, &at(&r, 0x34), 4));
    assert!(!overlaps(&ranges, &at(&r, 0x28), 8));
    assert!(!overlaps(&[], &at(&r, 0x30), 8));
}

#[test]
fn takes_and_width_cover_reads_and_declared_passes() {
    let r = reg_space();
    let e = EntryReads { read: vec![(at(&r, 0x38), 8)], declared: vec![(at(&r, 0x30), 4)], forwarded: Vec::new() };
    assert!(e.takes(&at(&r, 0x38), 8));
    assert!(e.takes(&at(&r, 0x30), 8));
    assert!(!e.takes(&at(&r, 0x10), 8));
    assert_eq!(e.width(&at(&r, 0x30)), Some(4));
    assert_eq!(e.width(&at(&r, 0x10)), None);
}

#[test]
fn reads_through_adds_only_what_a_callee_takes() {
    let r = reg_space();
    let (callee, other) = (at(&r, 0x1000), at(&r, 0x2000));
    let e = EntryReads {
        read: vec![(at(&r, 0x38), 8)],
        declared: vec![(at(&r, 0x30), 4)],
        forwarded: vec![(at(&r, 0x10), 8, callee.clone()), (at(&r, 0x08), 8, other.clone())],
    };
    let finals = vec![(at(&r, 0x80), 8, callee.clone()), (at(&r, 0x10), 8, callee.clone())];
    let takes = |c: &Address, a: &Address, _: int4| *c == callee && a.get_offset() != 0x08;
    let (all, passes) = reads_through(&e, &finals, takes);
    let offs = |v: &[(Address, int4)]| v.iter().map(|(a, _)| a.get_offset()).collect::<Vec<_>>();
    assert_eq!(offs(&passes), vec![0x30, 0x10, 0x80]);
    assert_eq!(offs(&all), vec![0x38, 0x30, 0x10, 0x80]);
}
