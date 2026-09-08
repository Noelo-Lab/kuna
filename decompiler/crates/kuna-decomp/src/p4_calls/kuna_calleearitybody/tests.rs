//! Unit tests for the `calleearitybody` option surface and the one decision the
//! module owns: which of a call's trials the callee's body claims as arguments
//! ([`plan_from_body`]), and the two guards that bound the claim.  Every case
//! here passes `cut = false`, so it is this rule alone that answers;
//! [`crate::p4_calls::kuna_calleearitycut`] owns the widened boundary test and
//! its own cases.  The end-to-end behaviour is
//! `tests/cli/defined-payload-function-loses.json`.

use super::*;

use kuna_base::space::{spacetype, AddrSpace};
use std::rc::Rc;

const REG: int4 = 3;

fn reg_space() -> Rc<AddrSpace> {
    Rc::new(AddrSpace::new(spacetype::IPTR_PROCESSOR, "register", false, 8, 1, REG, 0, 0, 0))
}

fn stack_space() -> Rc<AddrSpace> {
    Rc::new(AddrSpace::new(spacetype::IPTR_SPACEBASE, "stack", false, 8, 1, 4, 0, 0, 0))
}

/// A distinct arena handle; the plan only ever compares and carries these.
fn vid(n: u64) -> VarnodeId {
    slotmap::KeyData::from_ffi((1u64 << 32) | n).into()
}

/// A trial with a promotable Varnode standing at it.
fn trial(sp: &Rc<AddrSpace>, off: u64, vn: u64) -> BodyTrial {
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

/// A trial the site could not promote (definitely-not-used, unreferenced, or the
/// wrong width for its Varnode).
fn barren(sp: &Rc<AddrSpace>, off: u64) -> BodyTrial {
    BodyTrial {
        addr: Address::new(Rc::clone(sp), off),
        size: 8,
        vn: None,
        caller_quiet: true,
        caller_checked: true,
        caller_active: false,
        caller_constant: false,
    }
}

fn loc(sp: &Rc<AddrSpace>, off: u64) -> (Address, int4) {
    (Address::new(Rc::clone(sp), off), 8)
}

/// `reads` become read-before-write ranges; `written` is the one path
/// terminator, so every register in it is proven dead.
fn body(reads: &[u64], written: &[u64], complete: bool) -> CalleeEntryDead {
    let mut cut: Vec<(int4, u64)> = Vec::new();
    for w in written {
        for b in *w..*w + 8 {
            cut.push((REG, b));
        }
    }
    CalleeEntryDead::from_parts(
        REG,
        reads.iter().map(|r| (REG, *r, 8)).collect(),
        vec![cut],
        complete,
    )
}

/// The four Win64-shaped argument registers, in prototype order.
fn args(reg: &Rc<AddrSpace>) -> Vec<(Address, int4)> {
    vec![loc(reg, 0x8), loc(reg, 0x10), loc(reg, 0x80), loc(reg, 0x88)]
}

#[test]
fn option_name_and_apply() {
    assert_eq!(OptionCalleeArityBody::NAME, "calleearitybody");
    let (on, msg) = OptionCalleeArityBody.apply("on").expect("on");
    assert!(on);
    assert!(msg.contains("turned on"), "{msg}");
    let (off, msg) = OptionCalleeArityBody.apply("off").expect("off");
    assert!(!off);
    assert!(msg.contains("turned off"), "{msg}");
    assert!(OptionCalleeArityBody.apply("").expect("empty").0);
    assert!(OptionCalleeArityBody.apply("bogus").is_err());
}

/// The witness shape: the callee reads the first argument register and
/// overwrites the second before ever reading it, so the argument list is exactly
/// one long.
#[test]
fn a_read_run_bounded_by_a_dead_register_is_the_argument_list() {
    let reg = reg_space();
    let trials = vec![trial(&reg, 0x8, 1), trial(&reg, 0x10, 2), trial(&reg, 0x80, 3)];
    let live = body(&[0x8], &[0x10, 0x80], true);
    assert_eq!(plan_from_body(&trials, &args(&reg), &live, false, false), Some(vec![0]));
}

/// Two read registers extend the list; the dead third still ends it.
#[test]
fn the_run_is_as_long_as_the_reads_are() {
    let reg = reg_space();
    let trials = vec![trial(&reg, 0x8, 1), trial(&reg, 0x10, 2), trial(&reg, 0x80, 3)];
    let live = body(&[0x8, 0x10], &[0x80], true);
    assert_eq!(plan_from_body(&trials, &args(&reg), &live, false, false), Some(vec![0, 1]));
}

/// A body that proves a read and nothing else has not said where the argument
/// list ends — a forwarder cut at its own nested call looks exactly like this.
/// (`calleearitycut` is what admits this shape; here it is off.)
#[test]
fn a_read_with_no_dead_register_anywhere_is_refused() {
    let reg = reg_space();
    let trials = vec![trial(&reg, 0x8, 1), trial(&reg, 0x10, 2)];
    let live = body(&[0x8], &[], true);
    assert_eq!(plan_from_body(&trials, &args(&reg), &live, false, false), None);
}

/// The variadic register-save prologue: the callee reads EVERY argument
/// register, so none is left dead and the rule declines rather than inventing an
/// argument per register.
#[test]
fn a_callee_that_reads_every_argument_register_is_refused() {
    let reg = reg_space();
    let trials = vec![trial(&reg, 0x8, 1), trial(&reg, 0x10, 2)];
    let live = body(&[0x8, 0x10, 0x80, 0x88], &[], true);
    assert_eq!(plan_from_body(&trials, &args(&reg), &live, false, false), None);
}

/// An argument register the callee reads but this site has no trial for means
/// the run is a PREFIX of the callee's real list, not the whole of it.
#[test]
fn a_read_argument_register_outside_the_run_is_refused() {
    let reg = reg_space();
    let trials = vec![trial(&reg, 0x8, 1)];
    let live = body(&[0x8, 0x80], &[0x10], true);
    assert_eq!(plan_from_body(&trials, &args(&reg), &live, false, false), None);
}

/// Nothing read at all is nothing to say — an empty run is never promoted, even
/// with a dead register right there.
#[test]
fn an_empty_run_is_not_an_argument_list() {
    let reg = reg_space();
    let trials = vec![trial(&reg, 0x8, 1), trial(&reg, 0x10, 2)];
    let live = body(&[], &[0x8, 0x10], true);
    assert_eq!(plan_from_body(&trials, &args(&reg), &live, false, false), None);
}

/// All or nothing: a claimed register with no promotable Varnode at this site
/// aborts the whole list rather than shifting the arguments behind it.
#[test]
fn a_claimed_register_with_no_varnode_aborts() {
    let reg = reg_space();
    let trials = vec![trial(&reg, 0x8, 1), barren(&reg, 0x10), trial(&reg, 0x80, 3)];
    let live = body(&[0x8, 0x10], &[0x80], true);
    assert_eq!(plan_from_body(&trials, &args(&reg), &live, false, false), None);
}

/// A register the callee does not consume is skipped when there is nothing
/// standing at it — the unreferenced float-register fillers `build_trial_map`
/// synthesizes on SysV sit in front of the first integer argument.
#[test]
fn an_empty_register_the_callee_ignores_is_skipped() {
    let reg = reg_space();
    let trials = vec![barren(&reg, 0x1200), trial(&reg, 0x8, 1)];
    let entries = vec![loc(&reg, 0x1200), loc(&reg, 0x8), loc(&reg, 0x10)];
    let live = body(&[0x8], &[0x10], true);
    assert_eq!(plan_from_body(&trials, &entries, &live, false, false), Some(vec![1]));
}

/// ...but a register the caller DID write, sitting in FRONT of the run, is
/// refused: the promoted value would print in its position, and nothing in the
/// output would say the argument it names is not the first one.
#[test]
fn a_live_register_in_front_of_the_run_is_refused() {
    let reg = reg_space();
    let trials = vec![trial(&reg, 0x38, 9), trial(&reg, 0x30, 1)];
    let entries = vec![loc(&reg, 0x38), loc(&reg, 0x30), loc(&reg, 0x10)];
    let live = body(&[0x30], &[0x10], true);
    assert_eq!(plan_from_body(&trials, &entries, &live, false, false), None);
    // Behind the run the same register is just an argument register the callee
    // ignores, and the witness has three of them.
    let trials = vec![trial(&reg, 0x30, 1), trial(&reg, 0x38, 9)];
    let entries = vec![loc(&reg, 0x30), loc(&reg, 0x38), loc(&reg, 0x10)];
    assert_eq!(plan_from_body(&trials, &entries, &live, false, false), Some(vec![0]));
}

/// The register section ends at the stack: a caller-relative slot is not
/// something the callee-body decode answers for, so the walk stops there.
#[test]
fn the_stack_section_ends_the_walk() {
    let reg = reg_space();
    let stk = stack_space();
    let mut trials = vec![trial(&reg, 0x8, 1), trial(&stk, 0x28, 2)];
    let live = body(&[0x8], &[0x10], true);
    assert_eq!(plan_from_body(&trials, &args(&reg), &live, false, false), Some(vec![0]));
    // ...and a stack trial is never itself claimed, whatever the body says.
    trials.push(trial(&reg, 0x10, 3));
    assert_eq!(plan_from_body(&trials, &args(&reg), &live, false, false), Some(vec![0]));
}

/// An import, a thunk or a body past the decode budget leaves the walk
/// incomplete, and an incomplete walk proves nothing in either direction.
#[test]
fn an_incomplete_walk_proves_nothing() {
    let reg = reg_space();
    let trials = vec![trial(&reg, 0x8, 1), trial(&reg, 0x10, 2)];
    let live = body(&[0x8], &[0x10], false);
    assert_eq!(plan_from_body(&trials, &args(&reg), &live, false, false), None);
}

/// No prototype model means no argument locations to bound the claim with.
#[test]
fn no_model_entries_is_refused() {
    let reg = reg_space();
    let trials = vec![trial(&reg, 0x8, 1)];
    let live = body(&[0x8], &[0x10], true);
    assert_eq!(plan_from_body(&trials, &[], &live, false, false), None);
}
