//! Unit tests for the call-site stack-argument gap veto installed in
//! `ParamListStandard::fillin_map`.
//!
//! The model mirrors the Win64 input list the witness is scored against: four
//! exclusion (register) groups followed by a non-exclusion stack resource, which
//! is the boundary the whole rule is about.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{AddrSpace, AddrSpaceManager};
use kuna_base::types::int4;

use crate::dtype::type_class;
use crate::fspec::{ParamEntry, ParamListKind, ParamListStandard};

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

fn excl(
    grp: int4,
    space: &Rc<AddrSpace>,
    base: u64,
    prev: &[ParamEntry],
    mgr: &AddrSpaceManager,
) -> ParamEntry {
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

fn stack_res(
    grp: int4,
    space: &Rc<AddrSpace>,
    prev: &[ParamEntry],
    mgr: &AddrSpaceManager,
) -> ParamEntry {
    ParamEntry::seed(
        grp,
        type_class::TYPECLASS_GENERAL,
        Rc::clone(space),
        0,
        128,
        1,
        8, // alignment != 0 -> non-exclusion resource
        0,
        true,
        false,
        prev,
        mgr,
    )
    .expect("seed stack entry")
}

/// Four register groups (0..3) then one stack resource, as Win64 has.
fn win64_like_model() -> (ParamListStandard, Rc<AddrSpace>, Rc<AddrSpace>, AddrSpaceManager) {
    let mgr = AddrSpaceManager::new();
    let reg = reg_space();
    let stk = stack_space();
    let mut model = ParamListStandard::new(ParamListKind::Standard);
    for g in 0..4 {
        let e = excl(g, &reg, 0x10 + (g as u64) * 0x10, model.get_entry(), &mgr);
        model.push_entry(e);
    }
    let s = stack_res(4, &stk, model.get_entry(), &mgr);
    model.push_entry(s);
    model.finish_decode();
    (model, reg, stk, mgr)
}

/// The witness's call site: the caller loads register groups `regs`, never
/// touches the rest, and leaves a value in the stack slot at `stack_off`.  Every
/// registered trial is ACTIVE, which is what `checkInputTrialUse` scores a
/// written-then-passed Varnode as; the missing register group gets its
/// UNREFERENCED filler from `buildTrialMap`.
fn call_site(
    reg: &Rc<AddrSpace>,
    stk: &Rc<AddrSpace>,
    regs: &[u64],
    stack_off: Option<u64>,
) -> ParamActive {
    let mut active = ParamActive::new(true); // recoversubcall
    let mut n = 0;
    for g in regs {
        active.register_trial(&addr(reg, 0x10 + g * 0x10), 4);
        active.get_trial_mut(n).mark_active();
        n += 1;
    }
    if let Some(off) = stack_off {
        active.register_trial(&addr(stk, off), 8);
        active.get_trial_mut(n).mark_active();
    }
    active.set_stack_arg_gap(true);
    active
}

fn used_groups(active: &ParamActive, model: &ParamListStandard) -> Vec<int4> {
    (0..active.get_num_trials())
        .filter(|&i| active.get_trial(i).is_used())
        .map(|i| active.get_trial(i).slot_group(model.get_entry()))
        .collect()
}

/// The witness: RCX/RDX/R8 loaded, R9 never written, a leftover in the Win64
/// fifth-argument slot.  The call site stops at the register the caller never
/// wrote: the stack slot behind it is not an argument, so neither is the hole.
#[test]
fn unwritten_register_ends_the_argument_list() {
    let (model, reg, stk, mgr) = win64_like_model();
    let mut active = call_site(&reg, &stk, &[0, 1, 2], Some(0));
    model.fillin_map(&mut active, &mgr).expect("fillin_map");
    assert_eq!(used_groups(&active, &model), vec![0, 1, 2]);
}

/// Clause 3: the veto is about the register/stack boundary.  A hole in the
/// middle of the register file with a written register behind it keeps
/// upstream's fill.
#[test]
fn a_register_hole_behind_a_written_register_still_fills() {
    let (model, reg, stk, mgr) = win64_like_model();
    let mut active = call_site(&reg, &stk, &[0, 2], None);
    model.fillin_map(&mut active, &mgr).expect("fillin_map");
    assert_eq!(used_groups(&active, &model), vec![0, 1, 2]);
}

/// Clause 4: a call whose whole register file IS written keeps its stack
/// argument: the veto is inert on the shape the ABI actually permits.
#[test]
fn a_full_register_file_keeps_its_stack_argument() {
    let (model, reg, stk, mgr) = win64_like_model();
    let mut active = call_site(&reg, &stk, &[0, 1, 2, 3], Some(0));
    model.fillin_map(&mut active, &mgr).expect("fillin_map");
    assert_eq!(used_groups(&active, &model), vec![0, 1, 2, 3, 4]);
}

/// Clause 1: the same trials scored for the function's OWN inputs are untouched.
/// `ParamActive::new(false)` is what `ActionInputPrototype` builds, and the flag
/// alone must not be enough to fire the veto.
#[test]
fn the_functions_own_input_recovery_is_untouched() {
    let (model, reg, stk, mgr) = win64_like_model();
    let mut active = ParamActive::new(false); // not a call site
    for (i, g) in [0u64, 1, 2].iter().enumerate() {
        active.register_trial(&addr(&reg, 0x10 + g * 0x10), 4);
        active.get_trial_mut(i as int4).mark_active();
    }
    active.register_trial(&addr(&stk, 0), 8);
    active.get_trial_mut(3).mark_active();
    active.set_stack_arg_gap(true);
    model.fillin_map(&mut active, &mgr).expect("fillin_map");
    assert_eq!(used_groups(&active, &model), vec![0, 1, 2, 3, 4]);
}

/// The predicate itself answers true at the unreferenced register of a call site.
#[test]
fn ends_argument_list_fires_at_the_unreferenced_register() {
    let (model, reg, stk, mgr) = win64_like_model();
    let mut on = call_site(&reg, &stk, &[0, 1, 2], Some(0));
    // fillin_map's own build_trial_map is what synthesizes the unref filler, so
    // score first and then re-ask the predicate about the R9 slot.
    model.fillin_map(&mut on, &mgr).expect("on");
    let idx = (0..on.get_num_trials()).find(|&i| on.get_trial(i).is_unref()).expect("an unref filler");
    assert!(ends_argument_list(&on, idx, on.get_num_trials(), |j| {
        address_is_spacebase(on.get_trial(j).get_address())
    }));
}

