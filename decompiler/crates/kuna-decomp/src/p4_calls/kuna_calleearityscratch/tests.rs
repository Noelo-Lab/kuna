//! Unit tests for the `calleearityscratch` option surface and the boundary
//! test it widens.  Every case runs through
//! [`plan_from_body`](crate::p4_calls::kuna_calleearitybody::plan_from_body)
//! with `scratch` both ways, so each one also states what `calleearitycut`
//! alone answers.  The end-to-end behaviour is
//! `tests/cli/hint-printing-call-loses.json` and
//! `tests/stages/kuna-calleearityscratch.xml`.

use super::*;

use kuna_base::address::Address;
use kuna_base::space::{spacetype, AddrSpace};
use kuna_base::types::int4;
use std::rc::Rc;

use crate::context::VarnodeId;
use crate::p4_calls::kuna_calleearitybody::plan_from_body;
use crate::p4_calls::kuna_calleearitycut::accepts_cut_run;
use crate::p4_calls::kuna_calleedeadarg::CalleeEntryDead;

const REG: int4 = 3;

fn reg_space() -> Rc<AddrSpace> {
    Rc::new(AddrSpace::new(spacetype::IPTR_PROCESSOR, "register", false, 8, 1, REG, 0, 0, 0))
}

fn vid(n: u64) -> VarnodeId {
    slotmap::KeyData::from_ffi((1u64 << 32) | n).into()
}

/// A trial holding a value the caller PASSED THROUGH — quiet already, so the
/// widening has nothing to add.
fn quiet(sp: &Rc<AddrSpace>, off: u64, vn: u64) -> BodyTrial {
    BodyTrial {
        addr: Address::new(Rc::clone(sp), off),
        size: 8,
        vn: Some(vid(vn)),
        caller_quiet: true,
        caller_checked: true,
        caller_active: false,
        caller_constant: false,
    }
}

/// A trial the caller wrote and then USED ITSELF: `checkInputTrialUse` scored it
/// inactive, so it is scratch.
fn scratch(sp: &Rc<AddrSpace>, off: u64, vn: u64) -> BodyTrial {
    BodyTrial {
        addr: Address::new(Rc::clone(sp), off),
        size: 8,
        vn: Some(vid(vn)),
        caller_quiet: false,
        caller_checked: true,
        caller_active: false,
        caller_constant: false,
    }
}

/// A trial the caller wrote whose value reaches the CALL and nothing else —
/// upstream's own reading of a further argument.
fn active(sp: &Rc<AddrSpace>, off: u64, vn: u64) -> BodyTrial {
    BodyTrial { caller_active: true, ..scratch(sp, off, vn) }
}

/// A constant the caller materialized into the register, used by the caller too.
fn constant(sp: &Rc<AddrSpace>, off: u64, vn: u64) -> BodyTrial {
    BodyTrial { caller_constant: true, ..scratch(sp, off, vn) }
}

fn loc(sp: &Rc<AddrSpace>, off: u64) -> (Address, int4) {
    (Address::new(Rc::clone(sp), off), 8)
}

/// The SysV integer argument registers, in prototype order.
fn sysv(reg: &Rc<AddrSpace>) -> Vec<(Address, int4)> {
    vec![loc(reg, 0x38), loc(reg, 0x30), loc(reg, 0x10), loc(reg, 0x8), loc(reg, 0x80), loc(reg, 0x88)]
}

/// A summary that proves `reads` are inputs and nothing dead anywhere — what a
/// body whose every path is cut at a nested call answers.
fn cut_body(reads: &[u64]) -> CalleeEntryDead {
    CalleeEntryDead::from_parts(REG, reads.iter().map(|r| (REG, *r, 8)).collect(), vec![vec![]], true)
}

/// The witness's trials: rdi holds the buffer, rsi holds the XOR key the caller
/// consumed itself, and the rest are untouched.
fn witness(reg: &Rc<AddrSpace>, bound: BodyTrial) -> Vec<BodyTrial> {
    vec![
        quiet(reg, 0x38, 1),
        bound,
        quiet(reg, 0x10, 3),
        quiet(reg, 0x8, 4),
        quiet(reg, 0x80, 5),
        quiet(reg, 0x88, 6),
    ]
}

#[test]
fn option_name_and_apply() {
    assert_eq!(OptionCalleeArityScratch::NAME, "calleearityscratch");
    let (on, msg) = OptionCalleeArityScratch.apply("on").expect("on");
    assert!(on);
    assert!(msg.contains("turned on"), "{msg}");
    let (off, msg) = OptionCalleeArityScratch.apply("off").expect("off");
    assert!(!off);
    assert!(msg.contains("turned off"), "{msg}");
    assert!(OptionCalleeArityScratch.apply("").expect("empty").0);
    assert!(OptionCalleeArityScratch.apply("bogus").is_err());
}

/// The witness: the callee reads rdi and its decode is cut before it can say
/// anything about rsi, which the caller wrote as an XOR key and consumed.
#[test]
fn a_scratch_boundary_bounds_the_run() {
    let reg = reg_space();
    let trials = witness(&reg, scratch(&reg, 0x30, 2));
    let live = cut_body(&[0x38]);
    assert_eq!(plan_from_body(&trials, &sysv(&reg), &live, true, false), None);
    assert_eq!(plan_from_body(&trials, &sysv(&reg), &live, true, true), Some(vec![0]));
}

/// A boundary whose value reaches the CALL and nothing else is a further
/// argument, and truncating there would hide it.
#[test]
fn an_active_boundary_is_still_refused() {
    let reg = reg_space();
    let trials = witness(&reg, active(&reg, 0x30, 2));
    let live = cut_body(&[0x38]);
    assert_eq!(plan_from_body(&trials, &sysv(&reg), &live, true, true), None);
}

/// Materializing a constant into the next argument register is argument setup
/// in its most literal form, however the trial scored.
#[test]
fn a_constant_boundary_is_never_scratch() {
    let reg = reg_space();
    let trials = witness(&reg, constant(&reg, 0x30, 2));
    let live = cut_body(&[0x38]);
    assert_eq!(plan_from_body(&trials, &sysv(&reg), &live, true, true), None);
    assert!(!boundary_is_caller_scratch(&constant(&reg, 0x30, 2)));
}

/// An unscored trial has no verdict to borrow.
#[test]
fn an_unchecked_boundary_is_refused() {
    let reg = reg_space();
    let unchecked = BodyTrial { caller_checked: false, ..scratch(&reg, 0x30, 2) };
    let trials = witness(&reg, unchecked);
    let live = cut_body(&[0x38]);
    assert_eq!(plan_from_body(&trials, &sysv(&reg), &live, true, true), None);
}

/// The widening lives inside `calleearitycut`; with the cut rule off there is no
/// boundary test to widen.
#[test]
fn it_is_inert_with_the_cut_rule_off() {
    let reg = reg_space();
    let trials = witness(&reg, scratch(&reg, 0x30, 2));
    let live = cut_body(&[0x38]);
    assert_eq!(plan_from_body(&trials, &sysv(&reg), &live, false, true), None);
}

/// `calleearitycut`'s own guards still answer first: a run that reaches the last
/// argument register has no boundary at all, scratch or not.
#[test]
fn a_run_that_reaches_the_last_argument_register_is_still_refused() {
    let reg = reg_space();
    let mut trials = witness(&reg, scratch(&reg, 0x30, 2));
    trials[5] = scratch(&reg, 0x88, 6);
    let live = cut_body(&[0x38, 0x30, 0x10, 0x8, 0x80, 0x88]);
    assert_eq!(plan_from_body(&trials, &sysv(&reg), &live, true, true), None);
}

/// ...and a hole inside the run would move a later argument forward, so the run
/// must still be contiguous.
#[test]
fn a_hole_inside_the_run_is_still_refused() {
    let reg = reg_space();
    let mut trials = witness(&reg, scratch(&reg, 0x30, 2));
    trials[1] = BodyTrial { vn: None, ..scratch(&reg, 0x30, 2) };
    let live = cut_body(&[0x38, 0x10]); // rsi skipped, rdx read
    assert_eq!(plan_from_body(&trials, &sysv(&reg), &live, true, true), None);
}

/// The predicate reads only the boundary trial, and only in the one combination.
#[test]
fn the_predicate_is_checked_and_inactive_and_not_constant() {
    let reg = reg_space();
    assert!(boundary_is_caller_scratch(&scratch(&reg, 0x30, 2)));
    assert!(!boundary_is_caller_scratch(&active(&reg, 0x30, 2)));
    assert!(!boundary_is_caller_scratch(&constant(&reg, 0x30, 2)));
    assert!(!boundary_is_caller_scratch(&BodyTrial {
        caller_checked: false,
        ..scratch(&reg, 0x30, 2)
    }));
}

/// A quiet boundary is `calleearitycut`'s own case and does not need the
/// widening.
#[test]
fn a_quiet_boundary_still_bounds_the_run_without_it() {
    let reg = reg_space();
    let trials = witness(&reg, quiet(&reg, 0x30, 2));
    let live = cut_body(&[0x38]);
    assert!(accepts_cut_run(&trials, &[0], &sysv(&reg), false));
    assert_eq!(plan_from_body(&trials, &sysv(&reg), &live, true, false), Some(vec![0]));
}
