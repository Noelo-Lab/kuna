//! Unit tests for the `condexeret` retry-pass bookkeeping.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::{spacetype, AddrSpace};
use slotmap::SlotMap;

use super::*;

fn active_with(n: u64, maxpass: int4) -> ParamActive {
    let reg = Rc::new(AddrSpace::new(spacetype::IPTR_PROCESSOR, "register", false, 4, 1, 3, 0, 0, 0));
    let mut active = ParamActive::new(false);
    for i in 0..n {
        active.register_trial(&Address::new(Rc::clone(&reg), 8 * i), 8);
    }
    active.set_max_pass(maxpass);
    active
}

fn blocks() -> (BlockId, BlockId) {
    let mut ids: SlotMap<BlockId, ()> = SlotMap::with_key();
    (ids.insert(()), ids.insert(()))
}

#[test]
fn a_failed_trial_schedules_exactly_one_extra_pass() {
    let (threaded, kept) = blocks();
    let mut active = active_with(3, 0);
    active.get_trial_mut(2).mark_active();
    end_pass(&mut active, true, vec![(1, threaded), (2, threaded), (1, threaded), (0, kept)]);
    assert_eq!(active.cond_exe_retry(), &CondExeRetry::Pending(vec![(0, kept), (1, threaded)]));
    assert_eq!(active.get_max_pass(), 1);
    active.finish_pass();
    assert!(active.get_num_passes() <= active.get_max_pass());

    let gone = |b: BlockId| b == threaded;
    assert!(skips_with(&active, 0, gone));
    assert!(!skips_with(&active, 1, gone));
    assert!(skips_with(&active, 2, gone));

    end_pass(&mut active, true, vec![(1, threaded)]);
    assert_eq!(active.cond_exe_retry(), &CondExeRetry::Done);
    assert_eq!(active.get_max_pass(), 1);
    active.finish_pass();
    assert!(active.get_num_passes() > active.get_max_pass());
    assert!(!skips_with(&active, 0, gone));

    end_pass(&mut active, true, vec![(0, kept), (1, threaded)]);
    assert_eq!(active.cond_exe_retry(), &CondExeRetry::Done);
    assert_eq!(active.get_max_pass(), 1);
}

#[test]
fn a_block_still_in_the_graph_keeps_the_first_verdict() {
    let (a, b) = blocks();
    let mut active = active_with(1, 0);
    end_pass(&mut active, true, vec![(0, a), (0, b)]);
    assert!(skips_with(&active, 0, |_| false));
    assert!(!skips_with(&active, 0, |x| x == b));
}

#[test]
fn nothing_is_scheduled_off_early_or_when_every_failure_was_answered() {
    let (a, _) = blocks();
    let mut off = active_with(2, 0);
    end_pass(&mut off, false, vec![(0, a)]);
    assert_eq!(off.cond_exe_retry(), &CondExeRetry::Idle);
    assert_eq!(off.get_max_pass(), 0);

    let mut early = active_with(2, 3);
    end_pass(&mut early, true, vec![(0, a)]);
    assert_eq!(early.cond_exe_retry(), &CondExeRetry::Idle);
    assert_eq!(early.get_max_pass(), 3);

    let mut answered = active_with(2, 0);
    answered.get_trial_mut(0).mark_active();
    end_pass(&mut answered, true, vec![(0, a)]);
    assert_eq!(answered.cond_exe_retry(), &CondExeRetry::Idle);

    let mut none = active_with(2, 0);
    end_pass(&mut none, true, Vec::new());
    assert_eq!(none.cond_exe_retry(), &CondExeRetry::Idle);
    assert!(!skips_with(&none, 0, |_| true));
}
