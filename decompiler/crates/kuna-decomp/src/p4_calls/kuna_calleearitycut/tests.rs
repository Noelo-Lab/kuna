//! Unit tests for the `calleearitycut` option surface and the three conditions
//! that let a callee-body argument run stand without a provably dead boundary
//! register.  The cases run through
//! [`plan_from_body`](crate::p4_calls::kuna_calleearitybody::plan_from_body)
//! with `cut` both ways, so each one also states what `calleearitybody` alone
//! answers.  The end-to-end behaviour is
//! `tests/cli/bytecode-reader-call-site.json`.

use super::*;

use kuna_base::space::{spacetype, AddrSpace};
use std::rc::Rc;

use crate::context::VarnodeId;
use crate::p4_calls::kuna_calleearitybody::plan_from_body;
use crate::p4_calls::kuna_calleedeadarg::CalleeEntryDead;

const REG: int4 = 3;

fn reg_space() -> Rc<AddrSpace> {
    Rc::new(AddrSpace::new(spacetype::IPTR_PROCESSOR, "register", false, 8, 1, REG, 0, 0, 0))
}

fn stack_space() -> Rc<AddrSpace> {
    Rc::new(AddrSpace::new(spacetype::IPTR_SPACEBASE, "stack", false, 8, 1, 4, 0, 0, 0))
}

fn vid(n: u64) -> VarnodeId {
    slotmap::KeyData::from_ffi((1u64 << 32) | n).into()
}

/// A trial holding a value the caller PASSED THROUGH — its own incoming
/// register, untouched.
fn trial(sp: &Rc<AddrSpace>, off: u64, vn: u64) -> BodyTrial {
    BodyTrial {
        addr: Address::new(Rc::clone(sp), off),
        size: 8,
        vn: Some(vid(vn)),
        caller_quiet: true,
    }
}

/// A trial holding a value the caller COMPUTED into the register.
fn written(sp: &Rc<AddrSpace>, off: u64, vn: u64) -> BodyTrial {
    BodyTrial { addr: Address::new(Rc::clone(sp), off), size: 8, vn: Some(vid(vn)), caller_quiet: false }
}

/// A trial with nothing standing at it.
fn barren(sp: &Rc<AddrSpace>, off: u64) -> BodyTrial {
    BodyTrial { addr: Address::new(Rc::clone(sp), off), size: 8, vn: None, caller_quiet: true }
}

fn loc(sp: &Rc<AddrSpace>, off: u64) -> (Address, int4) {
    (Address::new(Rc::clone(sp), off), 8)
}

/// A summary that proves `reads` are inputs and nothing dead anywhere — what a
/// body whose every path is cut at a nested call answers.
fn cut_body(reads: &[u64]) -> CalleeEntryDead {
    CalleeEntryDead::from_parts(REG, reads.iter().map(|r| (REG, *r, 8)).collect(), vec![vec![]], true)
}

/// The SysV integer argument registers, in prototype order.
fn sysv(reg: &Rc<AddrSpace>) -> Vec<(Address, int4)> {
    vec![loc(reg, 0x38), loc(reg, 0x30), loc(reg, 0x10), loc(reg, 0x8), loc(reg, 0x80), loc(reg, 0x88)]
}

/// The witness's trials: rdi/rsi/rdx passed through, rcx written by the caller,
/// r8 and r9 untouched.
fn witness(reg: &Rc<AddrSpace>) -> Vec<BodyTrial> {
    vec![
        trial(reg, 0x38, 1),
        trial(reg, 0x30, 2),
        trial(reg, 0x10, 3),
        written(reg, 0x8, 4),
        trial(reg, 0x80, 5),
        barren(reg, 0x88),
    ]
}

#[test]
fn option_name_and_apply() {
    assert_eq!(OptionCalleeArityCut::NAME, "calleearitycut");
    let (on, msg) = OptionCalleeArityCut.apply("on").expect("on");
    assert!(on);
    assert!(msg.contains("turned on"), "{msg}");
    let (off, msg) = OptionCalleeArityCut.apply("off").expect("off");
    assert!(!off);
    assert!(msg.contains("turned off"), "{msg}");
    assert!(OptionCalleeArityCut.apply("").expect("empty").0);
    assert!(OptionCalleeArityCut.apply("bogus").is_err());
}

/// The witness: the callee reads all four argument registers and its decode is
/// cut before it can clobber r8, so `calleearitybody` alone declines and this
/// rule recovers the four.
#[test]
fn a_run_that_stops_short_at_an_untouched_register_is_the_argument_list() {
    let reg = reg_space();
    let trials = witness(&reg);
    let live = cut_body(&[0x38, 0x30, 0x10, 0x8]);
    assert_eq!(plan_from_body(&trials, &sysv(&reg), &live, false), None);
    assert_eq!(plan_from_body(&trials, &sysv(&reg), &live, true), Some(vec![0, 1, 2, 3]));
}

/// A run that consumes every argument register has no boundary at all — the
/// AArch64 variadic register-save prologue, which reads all of them.
#[test]
fn a_run_that_reaches_the_last_argument_register_is_refused() {
    let reg = reg_space();
    let trials = witness(&reg);
    let live = cut_body(&[0x38, 0x30, 0x10, 0x8, 0x80, 0x88]);
    assert_eq!(plan_from_body(&trials, &sysv(&reg), &live, true), None);
}

/// The caller computed a value into the register the run stops at, so that
/// register is carrying a further argument whatever the cut decode can see.
#[test]
fn a_boundary_register_the_caller_wrote_is_refused() {
    let reg = reg_space();
    let mut trials = witness(&reg);
    trials[4] = written(&reg, 0x80, 5);
    let live = cut_body(&[0x38, 0x30, 0x10, 0x8]);
    assert_eq!(plan_from_body(&trials, &sysv(&reg), &live, true), None);
}

/// A hole inside the run would print the register behind it in the hole's
/// position, so the run must be contiguous.
#[test]
fn a_hole_inside_the_run_is_refused() {
    let reg = reg_space();
    let trials = witness(&reg);
    let live = cut_body(&[0x38, 0x30, 0x8]); // rdx skipped
    assert_eq!(plan_from_body(&trials, &sysv(&reg), &live, true), None);
}

/// The stack section is not a register argument location, so a run that reaches
/// it has no boundary the callee-body decode answers for.
#[test]
fn a_run_bounded_by_the_stack_is_refused() {
    let reg = reg_space();
    let stk = stack_space();
    let trials = vec![trial(&reg, 0x38, 1), trial(&stk, 0x8, 2)];
    let entries = vec![loc(&reg, 0x38), (Address::new(Rc::clone(&stk), 0x8), 500)];
    let live = cut_body(&[0x38]);
    assert_eq!(plan_from_body(&trials, &entries, &live, true), None);
}

/// A register trial the prototype model does not name as an argument location
/// bounds nothing.
#[test]
fn a_boundary_outside_the_model_is_refused() {
    let reg = reg_space();
    let trials = vec![trial(&reg, 0x38, 1), trial(&reg, 0x200, 2)];
    let entries = vec![loc(&reg, 0x38), loc(&reg, 0x30)];
    let live = cut_body(&[0x38]);
    assert_eq!(plan_from_body(&trials, &entries, &live, true), None);
}

/// `calleearitybody`'s own guards still answer first: an argument register the
/// callee reads that is OUTSIDE the run means the run is a prefix of the real
/// list, and no widening of the boundary test changes that.
#[test]
fn a_read_register_outside_the_run_is_still_refused() {
    let reg = reg_space();
    let trials = vec![trial(&reg, 0x38, 1), barren(&reg, 0x30), trial(&reg, 0x10, 3)];
    let live = cut_body(&[0x38, 0x10]);
    assert_eq!(plan_from_body(&trials, &sysv(&reg), &live, true), None);
}

/// An incomplete summary proves nothing in either direction, cut or not.
#[test]
fn an_incomplete_walk_is_still_refused() {
    let reg = reg_space();
    let trials = witness(&reg);
    let live = CalleeEntryDead::from_parts(REG, vec![(REG, 0x38, 8)], vec![vec![]], false);
    assert_eq!(plan_from_body(&trials, &sysv(&reg), &live, true), None);
}

/// The run is what `calleearitybody` picked; an empty one is never widened.
#[test]
fn an_empty_run_is_never_accepted() {
    let reg = reg_space();
    let trials = witness(&reg);
    assert!(!accepts_cut_run(&trials, &[], &sysv(&reg)));
}
