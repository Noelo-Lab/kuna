use super::*;

#[test]
fn the_option_takes_on_and_off() {
    assert!(OptionGotTailCall.apply("on").unwrap().0);
    assert!(!OptionGotTailCall.apply("off").unwrap().0);
    assert!(OptionGotTailCall.apply("lock").is_err());
}

#[test]
fn a_caller_is_due_once_the_callee_it_copied_in_is_filed() {
    let mut ledger = Ledger { recording: true, ..Ledger::default() };
    let (caller, other, callee) = ((1, 0x10), (1, 0x20), (1, 0x30));
    ledger.waiting.entry(callee).or_default().extend([caller, other]);
    assert!(ledger.take_due().is_empty());
    ledger.callable.insert(callee, Callee { stated: true, returns: true });
    assert_eq!(ledger.take_due(), BTreeSet::from([caller, other]));
    assert!(ledger.waiting.is_empty());
    assert!(ledger.take_due().is_empty());
}

#[test]
fn nothing_is_due_outside_a_callee_first_run() {
    let mut ledger = Ledger::default();
    ledger.waiting.entry((1, 0x30)).or_default().insert((1, 0x10));
    ledger.callable.insert((1, 0x30), Callee { stated: false, returns: false });
    assert!(ledger.take_due().is_empty());
    assert_eq!(ledger.waiting.len(), 1);
}

#[test]
fn a_refused_caller_is_due_again_once() {
    let mut ledger = Ledger { recording: true, ..Ledger::default() };
    let (caller, callee) = ((1, 0x10), (1, 0x30));
    ledger.callable.insert(callee, Callee { stated: true, returns: true });
    ledger.refused.insert((caller, callee));
    ledger.waiting.entry(callee).or_default().insert(caller);
    assert_eq!(ledger.take_due(), BTreeSet::from([caller]));
    assert!(ledger.take_due().is_empty());
    assert!(ledger.refused.contains(&(caller, callee)));
}

#[test]
fn a_caller_waits_for_every_planned_callee_it_copied_in() {
    let (caller, a, b, unplanned) = ((1, 0x10), (1, 0x20), (1, 0x30), (1, 0x40));
    let mut ledger = Ledger { recording: true, ..Ledger::default() };
    ledger.planned.extend([caller, a, b]);
    ledger.seen.insert(caller);
    for k in [a, b, unplanned] {
        ledger.waiting.entry(k).or_default().insert(caller);
    }
    ledger.seen.insert(a);
    ledger.callable.insert(a, Callee { stated: true, returns: true });
    assert!(ledger.take_due().is_empty());
    ledger.seen.insert(b);
    assert_eq!(ledger.take_due(), BTreeSet::from([caller]));
    assert!(ledger.waiting.is_empty());
}

#[test]
fn a_refusal_is_due_before_the_other_callees_are_decompiled() {
    let (caller, callee, later) = ((1, 0x10), (1, 0x30), (1, 0x50));
    let mut ledger = Ledger { recording: true, ..Ledger::default() };
    ledger.planned.extend([caller, callee, later]);
    ledger.seen.extend([caller, callee]);
    ledger.callable.insert(callee, Callee { stated: true, returns: true });
    ledger.refused.insert((caller, callee));
    ledger.waiting.entry(callee).or_default().insert(caller);
    ledger.waiting.entry(later).or_default().insert(caller);
    assert_eq!(ledger.take_due(), BTreeSet::from([caller]));
    assert!(ledger.waiting.is_empty());
}
