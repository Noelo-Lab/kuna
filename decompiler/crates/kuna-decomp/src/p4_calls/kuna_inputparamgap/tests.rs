//! Unit tests for the `inputparamgap` option surface and the register-gap
//! tolerance it installs in `ParamListStandard::fillin_map`.
//!
//! The model mirrors the SysV x86-64 input list the witness is scored against:
//! six exclusion (register) groups followed by a non-exclusion stack resource,
//! so both halves of the narrowing can be exercised.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{spacetype, AddrSpace, AddrSpaceManager};
use kuna_base::types::int4;

use crate::dtype::type_class;
use crate::fspec::{ParamListKind, ParamListStandard};

use super::*;

fn reg_space() -> Rc<AddrSpace> {
    Rc::new(AddrSpace::new(spacetype::IPTR_PROCESSOR, "register", false, 4, 1, 3, 0, 0, 0))
}

fn stack_space() -> Rc<AddrSpace> {
    Rc::new(AddrSpace::new(spacetype::IPTR_SPACEBASE, "stack", false, 4, 1, 4, 0, 0, 0))
}

fn addr(spc: &Rc<AddrSpace>, off: u64) -> Address {
    Address::new(Rc::clone(spc), off)
}

fn excl(grp: int4, space: &Rc<AddrSpace>, base: u64, prev: &[ParamEntry], mgr: &AddrSpaceManager) -> ParamEntry {
    ParamEntry::seed(
        grp,
        type_class::TYPECLASS_GENERAL,
        Rc::clone(space),
        base,
        4,
        1,
        0,
        0,
        true,
        false,
        prev,
        mgr,
    )
    .expect("seed exclusion entry")
}

fn stack_res(grp: int4, space: &Rc<AddrSpace>, prev: &[ParamEntry], mgr: &AddrSpaceManager) -> ParamEntry {
    ParamEntry::seed(
        grp,
        type_class::TYPECLASS_GENERAL,
        Rc::clone(space),
        0,
        128,
        1,
        4, // alignment != 0 -> non-exclusion resource
        0,
        true,
        false,
        prev,
        mgr,
    )
    .expect("seed stack entry")
}

/// Six register groups (0..5) then one stack resource, as SysV x86-64 has.
fn sysv_like_model() -> (ParamListStandard, Rc<AddrSpace>, Rc<AddrSpace>, AddrSpaceManager) {
    let mgr = AddrSpaceManager::new();
    let reg = reg_space();
    let stk = stack_space();
    let mut model = ParamListStandard::new(ParamListKind::Standard);
    for g in 0..6 {
        let e = excl(g, &reg, 0x10 + (g as u64) * 0x10, model.get_entry(), &mgr);
        model.push_entry(e);
    }
    let s = stack_res(6, &stk, model.get_entry(), &mgr);
    model.push_entry(s);
    model.finish_decode();
    (model, reg, stk, mgr)
}

/// Register the given register groups as ACTIVE trials (the shape
/// `ActionInputPrototype` produces: an input Varnode with readers).
fn active_regs(reg: &Rc<AddrSpace>, groups: &[u64], own_input_gap: bool) -> ParamActive {
    let mut active = ParamActive::new(false);
    for (i, g) in groups.iter().enumerate() {
        active.register_trial(&addr(reg, 0x10 + g * 0x10), 4);
        active.get_trial_mut(i as int4).mark_active();
    }
    active.set_own_input_gap(own_input_gap);
    active
}

fn used_groups(active: &ParamActive, model: &ParamListStandard) -> Vec<int4> {
    (0..active.get_num_trials())
        .filter(|&i| active.get_trial(i).is_used())
        .map(|i| active.get_trial(i).slot_group(model.get_entry()))
        .collect()
}

#[test]
fn option_name_and_apply() {
    assert_eq!(OptionInputParamGap::NAME, "inputparamgap");
    let (on, msg) = OptionInputParamGap.apply("on").expect("on");
    assert!(on);
    assert!(msg.contains("turned on"), "{msg}");
    let (off, msg) = OptionInputParamGap.apply("off").expect("off");
    assert!(!off);
    assert!(msg.contains("turned off"), "{msg}");
    // Empty defaults to on, as every other on/off option does.
    assert!(OptionInputParamGap.apply("").expect("empty").0);
    assert!(OptionInputParamGap.apply("bogus").is_err());
}

/// The witness: groups 0, 4 and 5 read live-in with 1/2/3 ignored. Without the
/// option `forceInactiveChain` trips at the third empty slot and drops both far
/// registers, leaving a one-parameter prototype.
#[test]
fn three_register_gap_drops_the_far_registers_without_the_option() {
    let (model, reg, _stk, mgr) = sysv_like_model();
    let mut active = active_regs(&reg, &[0, 4, 5], false);
    model.fillin_map(&mut active, &mgr).expect("fillin_map");
    assert_eq!(used_groups(&active, &model), vec![0]);
}

/// With the option on, the two live-in registers survive and the ignored
/// registers between them are promoted to filler parameters, so the recovered
/// list is the contiguous 0..5 the ABI describes.
#[test]
fn three_register_gap_recovers_the_full_list_with_the_option() {
    let (model, reg, _stk, mgr) = sysv_like_model();
    let mut active = active_regs(&reg, &[0, 4, 5], true);
    model.fillin_map(&mut active, &mgr).expect("fillin_map");
    assert_eq!(used_groups(&active, &model), vec![0, 1, 2, 3, 4, 5]);
}

/// A two-register gap was always tolerated: the option moves the limit, it does
/// not create the behaviour.
#[test]
fn two_register_gap_is_unchanged_by_the_option() {
    for gap_on in [false, true] {
        let (model, reg, _stk, mgr) = sysv_like_model();
        let mut active = active_regs(&reg, &[0, 3], gap_on);
        model.fillin_map(&mut active, &mgr).expect("fillin_map");
        assert_eq!(used_groups(&active, &model), vec![0, 1, 2, 3], "gap_on={gap_on}");
    }
}

/// The measured narrowing: a STACK trial behind a register hole is left to
/// upstream's verdict. Protecting it too let a Win64 function promote eleven
/// scratch slots of the caller's argument area into parameters.
#[test]
fn a_stack_trial_behind_a_register_gap_is_not_protected() {
    let (model, reg, stk, mgr) = sysv_like_model();
    let mut active = ParamActive::new(false);
    active.register_trial(&addr(&reg, 0x10), 4); // group 0, live-in
    active.get_trial_mut(0).mark_active();
    active.register_trial(&addr(&stk, 0), 4); // the stack resource
    active.get_trial_mut(1).mark_active();
    active.set_own_input_gap(true);
    model.fillin_map(&mut active, &mgr).expect("fillin_map");
    // Groups 1..5 are empty, so the chain reaches the stack trial and upstream's
    // veto still applies to it: only the register parameter survives.
    assert_eq!(used_groups(&active, &model), vec![0]);
}

/// The predicate itself: a call site's trials are never protected, whatever the
/// architecture flag says, because nothing sets the flag on them.
#[test]
fn a_call_sites_trials_are_never_protected() {
    let (model, reg, _stk, mgr) = sysv_like_model();
    let mut active = ParamActive::new(true); // recoversubcall
    active.register_trial(&addr(&reg, 0x10), 4);
    active.get_trial_mut(0).mark_active();
    assert!(!active.is_own_input_gap());
    assert!(!trial_is_protected(&active, active.get_trial(0), model.get_entry()));
    let _ = mgr;
}

/// An INACTIVE trial is never protected: the option only stops a run from
/// cancelling evidence, it never manufactures any.
#[test]
fn an_inactive_trial_is_never_protected() {
    let (model, reg, _stk, mgr) = sysv_like_model();
    let mut active = ParamActive::new(false);
    active.register_trial(&addr(&reg, 0x10), 4);
    active.set_own_input_gap(true);
    // No mark_active(): the body never reads this register before writing it.
    model.fillin_map(&mut active, &mgr).expect("fillin_map");
    assert!(!trial_is_protected(&active, active.get_trial(0), model.get_entry()));
    assert!(used_groups(&active, &model).is_empty());
}
