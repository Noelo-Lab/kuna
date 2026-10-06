//! Unit tests for the `hiddenretarg` veto.

use std::rc::Rc;

use kuna_base::space::spacetype;

use super::*;
use crate::kuna_calleedeadarg::CalleeEntryDead;
use crate::p0_knowledge::options::KUNA_OPTION_NAMES;

#[test]
fn option_parses_on_and_off() {
    assert!(OptionHiddenRetArg.apply("on").unwrap().0);
    assert!(!OptionHiddenRetArg.apply("off").unwrap().0);
    assert!(OptionHiddenRetArg.apply("maybe").is_err());
}

#[test]
fn option_is_registered() {
    assert!(KUNA_OPTION_NAMES.contains(&OptionHiddenRetArg::NAME));
}

fn x8() -> Address {
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
    Address::new(reg, 0x4040)
}

fn x8_bytes() -> Vec<(int4, u64)> {
    (0x4040u64..0x4048).map(|b| (3, b)).collect()
}

/// `add x0,x0,#1; ret`: x8 is neither read nor written, and the only path
/// returns. `proves_dead` wants a write before the RETURN; `never_takes` does not.
#[test]
fn a_leaf_that_never_touches_x8_does_not_take_it() {
    let leaf = CalleeEntryDead::from_parts(3, Vec::new(), vec![Vec::new()], true);
    assert!(leaf.never_takes(&x8(), 8));
    assert!(!leaf.proves_dead(&x8(), 8));
}

/// `str x0,[x8]; ret` is a real hidden return.
#[test]
fn a_callee_that_reads_x8_takes_it() {
    let reads = vec![(3, 0x4040, 8)];
    let mk = CalleeEntryDead::from_parts(3, reads, vec![Vec::new()], true);
    assert!(!mk.never_takes(&x8(), 8));
    let low_half = CalleeEntryDead::from_parts(3, vec![(3, 0x4040, 4)], vec![Vec::new()], true);
    assert!(!low_half.never_takes(&x8(), 8));
}

/// A path that leaves for unread code with x8 unwritten may hand it on; one
/// that wrote x8 first (`mov x8,#98; svc #0`) cannot.
#[test]
fn a_transfer_needs_x8_written_first() {
    let ret = || CalleeEntryDead::from_parts(3, Vec::new(), vec![Vec::new()], true);
    assert!(!ret().with_opaque_cut(Vec::new()).never_takes(&x8(), 8));
    assert!(ret().with_opaque_cut(x8_bytes()).never_takes(&x8(), 8));
    let target = Address::new(x8().get_space().unwrap().clone(), 0x1000);
    assert!(!ret().with_named_cut(target.clone(), Vec::new()).never_takes(&x8(), 8));
    assert!(ret().with_named_cut(target, x8_bytes()).never_takes(&x8(), 8));
}

#[test]
fn an_uncovered_walk_proves_nothing() {
    assert!(!CalleeEntryDead::default().never_takes(&x8(), 8));
    let no_cuts = CalleeEntryDead::from_parts(3, Vec::new(), Vec::new(), true);
    assert!(!no_cuts.never_takes(&x8(), 8));
    let incomplete = CalleeEntryDead::from_parts(3, Vec::new(), vec![Vec::new()], false);
    assert!(!incomplete.never_takes(&x8(), 8));
    let other_space = CalleeEntryDead::from_parts(4, Vec::new(), vec![Vec::new()], true);
    assert!(!other_space.never_takes(&x8(), 8));
}

/// `helper: svc #0; ret` reaches the kernel with the caller's x8, which it
/// reads as the syscall number; `mov x8,#98; svc #0` brings its own.
#[test]
fn a_system_call_with_x8_unwritten_takes_it() {
    let consumer = CalleeEntryDead::from_parts(3, Vec::new(), vec![Vec::new()], true)
        .with_trap_cut(Vec::new());
    assert!(consumer.traps_unwritten(&x8(), 8));
    assert!(!consumer.never_takes(&x8(), 8));
    let own = CalleeEntryDead::from_parts(3, Vec::new(), vec![Vec::new()], true)
        .with_trap_cut(x8_bytes());
    assert!(!own.traps_unwritten(&x8(), 8));
    assert!(own.never_takes(&x8(), 8));
}
