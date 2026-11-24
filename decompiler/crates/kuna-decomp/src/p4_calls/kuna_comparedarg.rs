//! P4 input-trial scoring: an argument compared with another argument of the
//! same call is still that call's argument.
//!
//! `Funcdata::only_op_use` (C++ `Funcdata::onlyOpUse`, `funcdata_varnode.cc:1851`)
//! rejects a trial whose value also reaches a branch. A caller that skips the
//! call when its two arguments are equal loads both into their argument
//! registers, compares them there and makes the call:
//!
//! ```text
//!   mov  (%rdi),%rdi
//!   mov  (%rsi),%rsi
//!   cmp  %rsi,%rdi
//!   je   done
//!   call elapsed@plt          ; elapsed(s->sec, ref->sec)
//! ```
//!
//! Both trials reach the `CBRANCH` through the compare, both score inactive, and
//! the call printed `elapsed()`. With the callee an import kuna has no
//! prototype for, no body and no sibling call speak for the list either, so
//! the rule applies only to a direct call whose callee has no body to read
//! ([`bodiless_callee`]).
//!
//! A comparison between the value and another Varnode the same call reads as an
//! argument does not compete with the call: it reads nothing the call does not
//! read. The walk skips that op and does not follow its result, provided both
//! values are computed by the function itself on the way to the call, not its
//! own inputs and not joins of several paths ([`computed_here`]), and every
//! argument register before the pair holds such a value too ([`filled_before`]). A comparison
//! with anything else, a constant above all (`test %rcx,%rcx; jz; call f`, where
//! `rcx` may be no argument of `f`), still rejects the trial.

use kuna_base::space::spacetype;
use kuna_num::opcodes::OpCode;

use kuna_base::types::int4;

use crate::context::{OpId, VarnodeId};
use crate::funcdata::Funcdata;

/// Is `op`, a use of `vn` met while scoring a trial of the call `opmatch`, a
/// comparison of `vn` with another Varnode `opmatch` reads at an argument
/// slot, directly or through the COPY into a temporary the compare
/// instruction makes of it? The ops a flag-setting compare is lifted to count: the equality and
/// ordering tests, the borrow test, and a subtraction whose result is a
/// temporary of the instruction (`cmp`), not a register it writes (`sub`).
pub fn compares_other_argument(data: &Funcdata, opmatch: OpId, op: OpId, vn: VarnodeId) -> bool {
    let Some(call) = data.obank().get(opmatch) else { return false };
    if call.code() != OpCode::CPUI_CALL || !bodiless_callee(data, opmatch) {
        return false;
    }
    let Some(o) = data.obank().get(op) else { return false };
    match o.code() {
        OpCode::CPUI_INT_EQUAL
        | OpCode::CPUI_INT_NOTEQUAL
        | OpCode::CPUI_INT_LESS
        | OpCode::CPUI_INT_SLESS
        | OpCode::CPUI_INT_LESSEQUAL
        | OpCode::CPUI_INT_SLESSEQUAL
        | OpCode::CPUI_INT_SBORROW => {}
        OpCode::CPUI_INT_SUB
            if o
                .get_out()
                .and_then(|v| data.vbank().get(v))
                .is_some_and(|v| v.get_space().get_type() == spacetype::IPTR_INTERNAL) => {}
        _ => return false,
    }
    if o.num_input() != 2 {
        return false;
    }
    let other = match (o.get_in(0), o.get_in(1)) {
        (Some(a), Some(b)) if a == vn && b != vn => b,
        (Some(a), Some(b)) if b == vn && a != vn => a,
        _ => return false,
    };
    if !computed_here(data, vn) || !computed_here(data, other) {
        return false;
    }
    let copied = data
        .vbank()
        .get(other)
        .and_then(|v| v.get_def())
        .and_then(|d| data.obank().get(d))
        .filter(|d| d.code() == OpCode::CPUI_COPY)
        .and_then(|d| d.get_in(0));
    let Some(other_slot) = (1..call.num_input()).find(|&slot| {
        call.get_in(slot).is_some_and(|a| a == other || Some(a) == copied)
    }) else {
        return false;
    };
    let Some(own_slot) = (1..call.num_input()).find(|&slot| call.get_in(slot) == Some(vn)) else { return false };
    filled_before(data, opmatch, own_slot, other_slot)
}

/// Is every general-purpose argument register the convention puts before
/// the two call inputs at `a` and `b` one the call reads a value the function
/// computed into ([`computed_here`])? The compared pair then extends a list
/// the caller filled from the first register: `elapsed(s->sec, ref->sec)` is
/// `rdi` and `rsi`. A pair behind a register nobody loaded is leftovers, and
/// so is any pair at a variadic call: the format, not the registers, says how
/// many arguments `printf` takes, and U-Boot's `printf("No ext_csd found!\n")`
/// sits after a compare of two registers it never reads.
fn filled_before(data: &Funcdata, opmatch: OpId, a: int4, b: int4) -> bool {
    let Some(idx) = data.get_call_specs_index(opmatch) else { return false };
    let fc = data.get_call_specs(idx);
    if !fc.proto().has_model() || fc.is_dotdotdot() {
        return false;
    }
    let Some(call) = data.obank().get(opmatch) else { return false };
    let regs = crate::p4_calls::kuna_passthrough::general_arg_registers(fc.proto().model());
    let active = fc.active_input();
    let trial_at = |slot: int4| (0..active.get_num_trials()).map(|i| active.get_trial(i)).find(|t| t.get_slot() == slot);
    let position = |slot: int4| {
        let t = trial_at(slot)?;
        regs.iter().position(|(r, _)| r == t.get_address())
    };
    let (Some(pa), Some(pb)) = (position(a), position(b)) else { return false };
    regs[..pa.min(pb)].iter().all(|(r, _)| {
        (0..active.get_num_trials()).map(|i| active.get_trial(i)).any(|t| {
            t.get_address() == r
                && t.get_slot() >= 1
                && call.get_in(t.get_slot()).is_some_and(|v| computed_here(data, v))
        })
    })
}

/// Is `vn` a value the function computes itself, not its own input and not a
/// join of values from several paths (a loop counter, a bound carried around a
/// loop), through a COPY? A Thumb loop `for (i = 0; i < n; i++) acc += f(i);`
/// compares `i` with `n` in two argument registers and calls with them still
/// there; neither is `f`'s argument.
fn computed_here(data: &Funcdata, vn: VarnodeId) -> bool {
    let mut cur = vn;
    for _ in 0..4 {
        let Some(v) = data.vbank().get(cur) else { return false };
        let Some(def) = v.get_def().and_then(|d| data.obank().get(d)) else { return false };
        match def.code() {
            OpCode::CPUI_MULTIEQUAL | OpCode::CPUI_INDIRECT => return false,
            OpCode::CPUI_COPY => match def.get_in(0) {
                Some(next) => cur = next,
                None => return false,
            },
            _ => return true,
        }
    }
    false
}

/// Is the direct call `opmatch` to a callee whose body the entry walk could
/// not read through: an import, a PLT entry that jumps through its GOT slot
/// (a trampoline), an address with no code? Such a
/// call has no body evidence for `calleearitybody` or `passthrough` to read,
/// and the compare is the one witness left. An indirect call, whose target a
/// search loop may have just compared its way to, is left alone.
fn bodiless_callee(data: &Funcdata, opmatch: OpId) -> bool {
    let Some(idx) = data.get_call_specs_index(opmatch) else { return false };
    let fc = data.get_call_specs(idx);
    let entry = fc.get_entry_address();
    !entry.is_invalid()
        && (fc.plt_stub().is_some()
            || data.kuna_callee_entry_dead(entry).is_none_or(|d| !d.is_complete() || d.is_trampoline()))
}
