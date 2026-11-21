//! (kuna) `calleefilledarg` — an argument register the callee loads itself is
//! not a parameter slot, so it says nothing about the slots after it.
//!
//! # The symptom
//!
//! An Objective-C image linked with `-objc_stubs_fast` sends every message
//! through a `__objc_stubs` entry that loads the selector into `x1` and jumps
//! to `objc_msgSend`:
//!
//! ```text
//!   objc_msgSend$countByEnumeratingWithState:objects:count:
//!     adrp x1,…; ldr x1,[x1,#off]; adrp x16,…; ldr x16,[x16,#got]; br x16
//! ```
//!
//! The caller leaves `x1` alone and stages the real arguments in `x2`, `x3`
//! and `x4`:
//!
//! ```text
//!   mov x2,sp; add x3,sp,#0x40; mov w4,#0x10; bl objc_msgSend$countBy…
//! ```
//!
//! kuna printed `objc_msgSend$countByEnumeratingWithState:objects:count:(v3)`,
//! one argument, at every `for (x in …)` loop of a 72 MB app; with the address
//! of the enumeration state gone from the call, the state struct never escaped,
//! its zero-initialised slots folded through the call, and the loop body read
//! `*(objc_object **)(v6 * 8)` and compared a global at address 0.
//!
//! # Why the three arguments are lost
//!
//! `FuncCallSpecs::checkInputTrialUse` (`fspec.cc:5592`) scores each of them
//! active — a value the caller moved into the register with no other reader.
//! `x1` is scored definitely-not-used: the last call before this one killed
//! it, so what the register holds at the `bl` is an INDIRECT creation, nothing
//! the caller placed. `ParamListStandard::forceNoUse` (`fspec.cc:1623`) then
//! reads the positional rule of every register convention — the first slot
//! nobody fills ends the argument list — and marks every trial after `x1`
//! inactive. That rule is right for a C callee. It is wrong for a callee that
//! fills the slot itself: `x1` is not a parameter of the stub, and the stub's
//! own prototype, when the image declares the selector, is laid out as
//! `x0, x2, x3, …` for exactly that reason.
//!
//! # The evidence
//!
//! The callee's own body, read by the entry-liveness walk
//! [`crate::p4_calls::kuna_calleedeadarg`] already caches for every direct
//! call. Two things have to hold. The callee is a TRAMPOLINE
//! ([`CalleeEntryDead::is_trampoline`]): every path it has leaves through an
//! indirect transfer the walk cannot follow (`br x16` through a GOT slot, `jmp
//! *%rax`), none returns or calls a named target, and it reads no register on
//! entry -- so whatever it does not write goes to the real target exactly as
//! the caller staged it, and the slots after the filled one are the caller's
//! evidence alone. And the register is written on every path from the entry
//! before anything could read it ([`CalleeEntryDead::proves_dead`]). The
//! second alone would admit `void sink(long x) { printf("%ld\n", x); }`, which
//! writes `rsi` before reading it: at clang -O0 `sink((a - e) / 3)` leaves the
//! `idiv` remainder in `rdx`, and with `rsi` killed by the `strrchr` before
//! it, the positional cascade is what keeps `rdx` off the call. A trampoline
//! has no body of its own to leave such a remainder for.
//!
//! The pass acts only on a trial the caller's own evidence has ALREADY scored
//! definitely-not-used. A slot that is both unfilled by the caller and
//! overwritten by the trampoline carries nothing either way: it is marked
//! [`ParamTrial::is_callee_filled`], and `forceNoUse` steps over it as it
//! steps over a trial with no entry, so the slots after it are scored on their
//! own evidence. A hole `buildTrialMap` backfills on such a register is the
//! same kind of slot ([`ParamActive::is_callee_filled`]), and
//! `forceInactiveChain` never materialises either as an argument. The chain
//! rule itself is untouched: a filled hole still counts against the run of
//! unfilled slots the convention tolerates between two active ones, so a
//! caller that stages `rdi` and `r8` around three empty registers still
//! prints one argument whether or not the callee loads `rdx`.
//!
//! A trial that is anything but definitely-not-used, a non-register trial, an
//! indirect call, a callee that returns or calls on, and one the walk could
//! not cover keep the positional rule. Default-on: the only call sites it can
//! change are calls to a trampoline that loads an argument register the
//! caller left unset, with actively staged registers after it, and on both
//! parity corpora none is.

use crate::fspec::FuncCallSpecs;
use crate::funcdata::Funcdata;

/// Before `fillinMap`, hand `call`'s trials the callee's entry-liveness probe
/// and mark every register trial the caller scored definitely-not-used that
/// the callee writes before any read on every path from its entry.
///
/// A direct call to a trampoline only ([`CalleeEntryDead::is_trampoline`]):
/// with the option off, no probe for the callee, or a callee that returns,
/// calls a named target or reads a register on entry, the trials keep the
/// positional rule.
pub fn mark(call: &mut FuncCallSpecs, data: &Funcdata) {
    let dead = (data.get_arch().callee_filled_arg)
        .then(|| data.kuna_callee_entry_dead_shared(call.get_entry_address()))
        .flatten()
        .filter(|d| d.is_trampoline());
    let active = call.get_active_input();
    if let Some(dead) = &dead {
        for i in 0..active.get_num_trials() {
            let t = active.get_trial(i);
            let register = t.get_address().get_space().is_some_and(|s| s.get_name() == "register");
            if t.is_definitely_not_used() && register && dead.proves_dead(t.get_address(), t.get_size()) {
                active.get_trial_mut(i).set_callee_filled();
            }
        }
    }
    active.set_callee_filled(dead);
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use kuna_base::address::Address;
    use kuna_base::space::{spacetype, AddrSpace, AddrSpaceManager};

    use crate::dtype::type_class;
    use crate::fspec::{ParamActive, ParamEntry, ParamListKind, ParamListStandard};

    /// Five register slots; the caller fills slot 0, nothing fills slot 1, and
    /// slots 2 and 3 are actively staged.
    fn model() -> (AddrSpaceManager, Rc<AddrSpace>, ParamListStandard) {
        let mgr = AddrSpaceManager::new();
        let reg = Rc::new(AddrSpace::new(spacetype::IPTR_PROCESSOR, "register", false, 8, 1, 3, 0, 0, 0));
        let mut model = ParamListStandard::new(ParamListKind::Standard);
        for group in 0..5 {
            let e = ParamEntry::seed(
                group, type_class::TYPECLASS_GENERAL, Rc::clone(&reg), group as u64 * 8, 8, 1, 0, 0, true, false,
                model.get_entry(), &mgr,
            )
            .unwrap();
            model.push_entry(e);
        }
        model.finish_decode();
        (mgr, reg, model)
    }

    fn used(active: &ParamActive) -> Vec<u64> {
        (0..active.get_num_trials())
            .filter(|&i| active.get_trial(i).is_used())
            .map(|i| active.get_trial(i).get_address().get_offset())
            .collect()
    }

    #[test]
    fn an_unfilled_slot_ends_the_list_unless_the_callee_fills_it() {
        for (filled, expected) in [(false, vec![0]), (true, vec![0, 16, 24])] {
            let (mgr, reg, model) = model();
            let mut active = ParamActive::new(true);
            for slot in 0..4u64 {
                active.register_trial(&Address::new(Rc::clone(&reg), slot * 8), 8);
            }
            active.get_trial_mut(0).mark_active();
            active.get_trial_mut(1).mark_no_use();
            if filled {
                active.get_trial_mut(1).set_callee_filled();
            }
            active.get_trial_mut(2).mark_active();
            active.get_trial_mut(3).mark_active();
            model.fillin_map(&mut active, &mgr).unwrap();
            assert_eq!(used(&active), expected, "filled={filled}");
        }
    }

    #[test]
    fn a_filled_hole_still_counts_against_the_chain() {
        // Three unfilled slots before the active one exceed the chain the
        // convention tolerates, filled or not: the list ends at slot 0.
        let (mgr, reg, model) = model();
        let mut active = ParamActive::new(true);
        for slot in 0..5u64 {
            active.register_trial(&Address::new(Rc::clone(&reg), slot * 8), 8);
        }
        active.get_trial_mut(0).mark_active();
        active.get_trial_mut(2).set_callee_filled();
        active.get_trial_mut(4).mark_active();
        model.fillin_map(&mut active, &mgr).unwrap();
        assert_eq!(used(&active), vec![0]);
    }

    #[test]
    fn a_filled_slot_is_never_hole_filled() {
        let (mgr, reg, model) = model();
        let mut active = ParamActive::new(true);
        for slot in 0..3u64 {
            active.register_trial(&Address::new(Rc::clone(&reg), slot * 8), 8);
        }
        active.get_trial_mut(0).mark_inactive();
        active.get_trial_mut(1).mark_no_use();
        active.get_trial_mut(1).set_callee_filled();
        active.get_trial_mut(2).mark_active();
        model.fillin_map(&mut active, &mgr).unwrap();
        assert_eq!(used(&active), vec![0, 16]);
    }
}
