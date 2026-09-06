//! Unit tests for the `calleedeadarg` callee entry-liveness probe.

use super::*;
use crate::p0_knowledge::options::KUNA_OPTION_NAMES;

#[test]
fn option_parses_on_and_off() {
    assert!(OptionCalleeDeadArg.apply("on").unwrap().0);
    assert!(!OptionCalleeDeadArg.apply("off").unwrap().0);
    assert!(OptionCalleeDeadArg.apply("maybe").is_err());
}

#[test]
fn option_is_registered() {
    assert!(KUNA_OPTION_NAMES.contains(&OptionCalleeDeadArg::NAME));
}

#[test]
fn incomplete_summary_proves_nothing() {
    let d = CalleeEntryDead::default();
    assert!(!d.is_complete());
}

#[test]
fn summary_with_no_terminator_proves_nothing() {
    // A walk whose every path closes back onto an already-visited address ends
    // complete, with no reads and no cuts.  The cut test is a conjunction, so
    // over an empty list it holds for every register at once.
    let reg = Rc::new(kuna_base::space::AddrSpace::new(
        spacetype::IPTR_PROCESSOR,
        "register",
        false,
        8,
        1,
        3,
        kuna_base::space::addrspace_flags::hasphysical,
        1,
        1,
    ));
    let complete_no_cuts =
        CalleeEntryDead { reg_idx: 3, reads: Vec::new(), cuts: Vec::new(), complete: true };
    assert!(complete_no_cuts.is_complete());
    assert!(!complete_no_cuts.proves_dead(&Address::new(Rc::clone(&reg), 8), 8));

    // The same summary with one terminator that wrote RCX still proves it dead.
    let mut cut = ByteSet::new();
    for b in 8u64..16 {
        cut.insert((3, b));
    }
    let one_cut =
        CalleeEntryDead { reg_idx: 3, reads: Vec::new(), cuts: vec![cut], complete: true };
    assert!(one_cut.proves_dead(&Address::new(reg, 8), 8));
}
