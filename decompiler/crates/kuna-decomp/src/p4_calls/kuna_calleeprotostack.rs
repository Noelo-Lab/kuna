//! (kuna) `calleeprotostack` — read a declared callee's stack contract off its
//! prototype instead of guessing it from the caller's push run.
//!
//! # The gap
//!
//! A prototype that is locked says two things about the stack, and kuna acted on
//! neither.
//!
//! **How much the callee pops.** `FuncProto::resolveExtraPop` (`fspec.cc`) turns
//! a locked parameter list into the `4 + <stack argument bytes>` a callee-cleans
//! convention takes off the stack, and it is the only thing that can answer for
//! a model whose own `<prototype extrapop="unknown">` declines to. Nothing in
//! the tree called it, so a locked prototype left the call at
//! `extrapop_unknown`, `StackSolver` fell through to a guess, and
//! [`crate::p6_variables::kuna_calleepop`]'s guess reads the push run in front of
//! the call. That run is not always the callee's: a Win32 image stages the
//! arguments of one API and calls a second, argument-less one in the middle of
//! the run, and every staged push is then credited to the wrong callee. The
//! error does not stay local — the solver latches its answer into `INT_ADD sp,
//! #c` at every later reference, so the outer call's own stack arguments land at
//! offsets nothing wrote and it renders with the three arguments pushed after the
//! inner call and none of the four pushed before it.
//!
//! **How much of the CALLER's stack it can touch.** `Heritage::guardCalls`
//! (`heritage.cc:1444`) asks the prototype what the call does to every heritaged
//! range and gets `unknown_effect` for the whole stack, so an INDIRECT is planted
//! over each of the caller's slots. That is the honest answer for an unknown
//! callee. For one with a locked, non-variadic prototype it is not: a slot above
//! the callee's own parameter area is the caller's, and the callee can reach it
//! only through a pointer. The planted INDIRECT is what stops the staged value
//! from reaching the call it was staged for, so the argument renders as a local
//! assigned on the line before instead of the constant or `&local` in the source.
//!
//! # The mechanism
//!
//! Both halves are the same claim — *the declared prototype is the truth about
//! this callee's stack* — and both are declined wherever that claim is not
//! evidence.
//!
//! * **The pop.** Only for a prototype that is input-locked, and only where the
//!   MODEL leaves its extrapop unknown. A spec that states its own extrapop
//!   (`x86gcc.cspec`'s `__cdecl extrapop="4"`, x86-64, every RISC spec) already
//!   has the exact answer and is left alone — recomputing it from the parameter
//!   list there would claim a `__cdecl` callee cleans its own arguments.
//! * **The reach.** Only for an input-locked, non-variadic prototype whose
//!   extrapop is known by the rule above; only for a stack range wholly at or
//!   above that extrapop in the CALLEE's frame (the return-address slot and the
//!   parameter area keep the guard); and only for a range no pointer in the
//!   caller can reach, which is [`crate::varmap::AliasChecker`] — the same test
//!   `FuncCallSpecs::checkInputTrialUse` already applies before it will call a
//!   stack slot a parameter. A slot whose address is taken is exactly the
//!   `ReadFile(h,&buf,…)` case: the callee writes it through the pointer, the
//!   guard is what models that write, and it stays.
//!
//! Inert on any target whose callees carry no declared prototype, which is both
//! parity corpora.

use kuna_base::error::KunaResult;
use kuna_base::types::{int4, uintb};

use crate::fspec::{FuncProto, EXTRAPOP_UNKNOWN};
use crate::p0_knowledge::options::on_or_off;

/// `option calleeprotostack on|off` (C++ `ArchOption` shape).
pub struct OptionCalleeProtoStack;

impl OptionCalleeProtoStack {
    /// The option name.
    pub const NAME: &'static str = "calleeprotostack";

    /// Resolve the flag and its confirmation message; the caller writes it into
    /// `Architecture::callee_proto_stack`.
    pub fn apply(&self, p1: &str) -> KunaResult<(bool, String)> {
        let val = on_or_off(p1)?;
        let prop = if val { "on" } else { "off" };
        Ok((val, format!("Declared-callee stack contract turned {prop}")))
    }
}

/// Give a freshly seeded, locked callee prototype the extrapop its parameter
/// list implies (`ActionDefaultParams`).
///
/// A no-op unless the prototype is input-locked and its MODEL declines to state
/// an extrapop; a model that states one already has the exact answer.
pub fn resolve_declared_extra_pop(on: bool, fp: &mut FuncProto) {
    if !on || !fp.is_input_locked() {
        return;
    }
    if fp.get_extra_pop() != EXTRAPOP_UNKNOWN {
        return;
    }
    fp.resolve_extra_pop();
}

/// The callee-frame offset at and above which a declared callee's stack contract
/// leaves the caller's slots alone, or `None` when the prototype is not evidence.
///
/// This is the prototype's extrapop: the return-address slot plus the stack
/// argument bytes the callee owns.
pub fn declared_caller_frame_floor(on: bool, fp: &FuncProto) -> Option<int4> {
    if !on || !fp.is_input_locked() || fp.is_dotdotdot() {
        return None;
    }
    let ep = fp.get_extra_pop();
    if ep == EXTRAPOP_UNKNOWN || ep < 4 {
        return None;
    }
    Some(ep)
}

/// True when `[offset, offset+size)` — a CALLEE-relative stack range — lies
/// wholly above the callee's own frame and parameter area.
///
/// `offset` is the raw (wrapped) space offset; it is read as a signed value in
/// the space's own width so a caller slot below the entry stack pointer does not
/// look like a huge positive one.
pub fn above_callee_frame(offset: uintb, size: int4, addr_size: int4, floor: int4) -> bool {
    let bits = (addr_size * 8) as u32;
    if bits == 0 || bits > 64 {
        return false;
    }
    let signed = if bits == 64 {
        offset as i64
    } else {
        ((offset << (64 - bits)) as i64) >> (64 - bits)
    };
    signed >= floor as i64 && signed.checked_add(size as i64).is_some()
}

#[cfg(test)]
mod tests;
