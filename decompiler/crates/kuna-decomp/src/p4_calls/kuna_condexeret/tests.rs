//! Unit tests for the `condexeret` retry-pass bookkeeping.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{spacetype, AddrSpace};
use slotmap::SlotMap;

use super::*;

// One shared register space, so `Address` (which compares spaces by pointer)
// is equal wherever the tests build the same storage.
thread_local! {
    static REG: Rc<AddrSpace> =
        Rc::new(AddrSpace::new(spacetype::IPTR_PROCESSOR, "register", false, 4, 1, 3, 0, 0, 0));
}
fn reg() -> Rc<AddrSpace> {
    REG.with(Rc::clone)
}

fn active_with(n: u64, maxpass: int4) -> ParamActive {
    let mut active = ParamActive::new(false);
    for i in 0..n {
        active.register_trial(&Address::new(reg(), 8 * i), 8);
    }
    active.set_max_pass(maxpass);
    active
}

fn blocks() -> (BlockId, BlockId) {
    let mut ids: SlotMap<BlockId, ()> = SlotMap::with_key();
    (ids.insert(()), ids.insert(()))
}

fn rets() -> (OpId, OpId) {
    let mut ids: SlotMap<OpId, ()> = SlotMap::with_key();
    (ids.insert(()), ids.insert(()))
}

fn rem(trial: int4, ret: OpId, merges: Vec<BlockId>) -> Remembered {
    Remembered { trial, ret, merges, inputs: vec![(Address::new(reg(), 0), 8)] }
}

#[test]
fn a_remembered_trial_schedules_exactly_one_extra_pass() {
    let (threaded, kept) = blocks();
    let (r0, r1) = rets();
    let mut active = active_with(3, 0);
    active.get_trial_mut(2).mark_active();
    assert!(remembering(&active, true));
    assert!(!remembering(&active, false));
    end_pass(&mut active, true, vec![rem(1, r0, vec![threaded]), rem(2, r0, vec![threaded]), rem(0, r1, vec![kept])]);
    assert_eq!(
        active.cond_exe_retry(),
        &CondExeRetry::Pending(vec![rem(1, r0, vec![threaded]), rem(0, r1, vec![kept])])
    );
    assert_eq!(active.get_max_pass(), 1);
    active.finish_pass();
    assert!(active.get_num_passes() <= active.get_max_pass());
    assert!(!remembering(&active, true));

    let gone = |b: BlockId| b == threaded;
    assert!(matches!(check_with(&active, 1, r0, gone), Check::Strict(_)));
    assert_eq!(check_with(&active, 1, r1, gone), Check::Skip);
    assert_eq!(check_with(&active, 0, r1, gone), Check::Skip);
    assert_eq!(check_with(&active, 2, r0, gone), Check::Skip);

    end_pass(&mut active, true, vec![rem(1, r0, vec![threaded])]);
    assert_eq!(active.cond_exe_retry(), &CondExeRetry::Done);
    assert_eq!(active.get_max_pass(), 1);
    active.finish_pass();
    assert!(active.get_num_passes() > active.get_max_pass());
    assert_eq!(check_with(&active, 1, r0, gone), Check::Normal);

    end_pass(&mut active, true, vec![rem(0, r0, vec![kept])]);
    assert_eq!(active.cond_exe_retry(), &CondExeRetry::Done);
    assert_eq!(active.get_max_pass(), 1);
}

#[test]
fn a_pair_is_rewalked_only_once_every_merge_is_gone() {
    let (a, b) = blocks();
    let (r0, _) = rets();
    let mut active = active_with(1, 0);
    end_pass(&mut active, true, vec![rem(0, r0, vec![a, b])]);
    assert_eq!(check_with(&active, 0, r0, |_| false), Check::Skip);
    assert_eq!(check_with(&active, 0, r0, |x| x == b), Check::Skip);
    assert_eq!(
        check_with(&active, 0, r0, |_| true),
        Check::Strict(vec![(Address::new(reg(), 0), 8)])
    );
}

#[test]
fn nothing_is_scheduled_off_early_or_when_every_failure_was_answered() {
    let (a, _) = blocks();
    let (r0, _) = rets();
    let mut off = active_with(2, 0);
    end_pass(&mut off, false, vec![rem(0, r0, vec![a])]);
    assert_eq!(off.cond_exe_retry(), &CondExeRetry::Idle);
    assert_eq!(off.get_max_pass(), 0);

    let mut early = active_with(2, 3);
    assert!(!remembering(&early, true));
    end_pass(&mut early, true, vec![rem(0, r0, vec![a])]);
    assert_eq!(early.cond_exe_retry(), &CondExeRetry::Idle);
    assert_eq!(early.get_max_pass(), 3);

    let mut answered = active_with(2, 0);
    answered.get_trial_mut(0).mark_active();
    end_pass(&mut answered, true, vec![rem(0, r0, vec![a])]);
    assert_eq!(answered.cond_exe_retry(), &CondExeRetry::Idle);

    let mut none = active_with(2, 0);
    end_pass(&mut none, true, Vec::new());
    assert_eq!(none.cond_exe_retry(), &CondExeRetry::Idle);
    assert_eq!(check_with(&none, 0, r0, |_| true), Check::Normal);
}
