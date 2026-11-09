//! (kuna) Recover stack-passed call arguments.
//!
//! `ActionActiveParam` -> `FuncCallSpecs::checkInputTrialUse` (`fspec.cc:5585`)
//! decides whether a call's stack-parameter trial is a real argument.  One of
//! its no-use tests asks whether the outgoing-argument slot lies inside the
//! **caller's** own local stack range, and upstream asks that of the argument
//! Varnode's address (`fspec.cc:5618`, `inRange(vn->getAddr(),1)`).
//!
//! kuna asked it of the *trial's* address instead.  `Heritage::guard_calls`
//! registers the trial at the callee-relative address (`addr - stackoffset`)
//! while creating the argument Varnode at the caller-relative one, so the two
//! differ by the call's stack offset.  On a downward-growing stack the
//! callee-relative offsets are always positive (8, 16, 24, ...) and the caller's
//! `localrange` is negative, so every stack trial at every call site with an
//! unlocked callee prototype failed the test and was scored `no-use` — and a
//! definitely-unused trial has its CALL input replaced with constant 0, handing
//! the argument-producing computation to dead-code elimination.
//!
//! Visible as: calls truncated at the register budget (x86-64: six arguments,
//! i386: none at all), and — where a dropped argument was the sole consumer of a
//! computation — deleted basic blocks.  `docs/features/callsite-stack-args/`
//! carries the two decbench cases, the minimal witnesses and the corpus numbers.

use kuna_base::address::{Address, RangeList};
use kuna_base::marshal::ElementId;
use kuna_base::space::AddrSpace;
use kuna_base::types::uintb;
use std::rc::Rc;


/// Marshaling element `<callsitestackargs>` (kuna 4000+ range; 4117 = warnstyle).
pub const ELEM_CALLSITESTACKARGS: ElementId = ElementId::new("callsitestackargs", 4118);

/// Is this stack trial outside the caller's own local stack frame?
///
/// The `checkInputTrialUse` no-use test (`fspec.cc:5618`), asked of the
/// caller-relative Varnode address as upstream does.
pub fn outside_caller_local_range(
    localrange: &RangeList,
    vn_space: &Rc<AddrSpace>,
    vn_offset: uintb,
) -> bool {
    !localrange.in_range(&Address::new(Rc::clone(vn_space), vn_offset), 1)
}

#[cfg(test)]
#[path = "kuna_callsitestackargs/tests.rs"]
mod tests;
