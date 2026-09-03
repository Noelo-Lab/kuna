//! Unit tests for the `varargstackargs` option surface and its section-split
//! predicate.

use super::*;
use kuna_base::address::Address;
use kuna_base::space::AddrSpace;
use std::rc::Rc;

fn reg_space() -> Rc<AddrSpace> {
    Rc::new(AddrSpace::new(spacetype::IPTR_PROCESSOR, "register", false, 8, 1, 1, 0, 0, 0))
}

fn stack_space() -> Rc<AddrSpace> {
    Rc::new(AddrSpace::new(spacetype::IPTR_SPACEBASE, "stack", false, 8, 1, 1, 0, 0, 0))
}

/// The AArch64 shape: one section holding `x0`..`x7` and then the outgoing
/// stack area, with an entry attached to every trial.
fn aarch64_section(variadic: bool) -> ParamActive {
    let reg = reg_space();
    let stk = stack_space();
    let mut active = ParamActive::new(true);
    for i in 0..8 {
        active.register_trial(&Address::new(Rc::clone(&reg), 0x4000 + i * 8), 8);
    }
    for i in 0..4 {
        active.register_trial(&Address::new(Rc::clone(&stk), i * 8), 8);
    }
    for i in 0..active.get_num_trials() {
        active.get_trial_mut(i).set_entry(Some(i as usize), 0);
    }
    if variadic {
        active.set_vararg_stack_split(true);
    }
    active
}

#[test]
fn option_name_and_apply() {
    assert_eq!(OptionVarargStackArgs::NAME, "varargstackargs");
    let (on, msg) = OptionVarargStackArgs.apply("on").expect("on");
    assert!(on);
    assert!(msg.contains("turned on"), "{msg}");
    let (off, msg) = OptionVarargStackArgs.apply("off").expect("off");
    assert!(!off);
    assert!(msg.contains("turned off"), "{msg}");
    // Empty defaults to on, as every other on/off option does.
    assert!(OptionVarargStackArgs.apply("").expect("empty").0);
    assert!(OptionVarargStackArgs.apply("bogus").is_err());
}

/// The whole feature: with the flag set the section is cut at the first stack
/// trial, and with it clear the section is scored whole (upstream).
#[test]
fn the_cut_is_the_first_stack_trial_and_only_when_enabled() {
    let mut on = aarch64_section(true);
    assert_eq!(stack_section_split(&on, 0, 12), Some(8));
    // The stack tail itself has no register prefix, so it is never cut again --
    // this is what stops the caller looping.
    assert_eq!(stack_section_split(&on, 8, 12), None);
    on.set_vararg_stack_split(false);
    assert_eq!(stack_section_split(&on, 0, 12), None);

    let off = aarch64_section(false);
    assert_eq!(stack_section_split(&off, 0, 12), None);
}

/// i386 `__cdecl`: every argument, fixed ones included, is already on the
/// stack.  There is no register hole to discount, so there is nothing to cut.
#[test]
fn an_all_stack_section_is_left_alone() {
    let stk = stack_space();
    let mut active = ParamActive::new(true);
    for i in 0..4 {
        active.register_trial(&Address::new(Rc::clone(&stk), i * 4), 4);
    }
    for i in 0..active.get_num_trials() {
        active.get_trial_mut(i).set_entry(Some(i as usize), 0);
    }
    active.set_vararg_stack_split(true);
    assert_eq!(stack_section_split(&active, 0, 4), None);
}

/// The cut trial becomes the new section's `groupstart` through
/// `ParamTrial::slot_group`, which panics on a trial with no `ParamEntry`.
#[test]
fn a_stack_trial_with_no_entry_is_never_the_cut() {
    let mut active = aarch64_section(true);
    active.get_trial_mut(8).set_entry(None, 0);
    assert_eq!(stack_section_split(&active, 0, 12), None);
}

/// A register-only section (the AArch64 float resource, say) never splits.
#[test]
fn a_register_only_section_is_left_alone() {
    let active = aarch64_section(true);
    assert_eq!(stack_section_split(&active, 0, 8), None);
}

/// MIPS o32: `mips32be.cspec` starts the stack parameter area at offset 16,
/// which is "backup storage space for register params" in its own comment, and
/// which is where PIC code saves `$gp` across a call.  A section whose stack
/// area does not begin at the outgoing stack pointer is never cut.
#[test]
fn a_stack_area_behind_a_home_or_return_slot_is_left_alone() {
    let reg = reg_space();
    let stk = stack_space();
    let mut active = ParamActive::new(true);
    for i in 0..4 {
        active.register_trial(&Address::new(Rc::clone(&reg), 0x4000 + i * 8), 4);
    }
    for i in 0..3 {
        active.register_trial(&Address::new(Rc::clone(&stk), 16 + i * 4), 4);
    }
    for i in 0..active.get_num_trials() {
        active.get_trial_mut(i).set_entry(Some(i as usize), 0);
    }
    active.set_vararg_stack_split(true);
    assert_eq!(stack_section_split(&active, 0, 7), None);
}
