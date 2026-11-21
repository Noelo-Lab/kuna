//! (kuna) `callresultuse` — a callee's result, read untouched by the next call
//! or by the RETURN, is that call's result although the callee's prototype
//! is unknown.
//!
//! # The symptom
//!
//! `bl allKeys; bl objc_retainAutoreleasedReturnValue; mov x22,x0; mov x2,x21;
//! bl containsObject:; mov x0,x22; bl objc_release` reads each message's `x0`
//! as the next receiver and later releases it; the callees are imports.  kuna
//! printed `allKeys(a); objc_retainAutoreleasedReturnValue();
//! containsObject:()`, receiver and argument gone, and `g2(g(a))` with `g`
//! and `g2` imports as `g(); g2();`.
//!
//! # Why
//!
//! Three things in upstream's recovery conspire.  A register a call to an
//! unknown callee may return is modelled at the call by an INDIRECT creation
//! (`Heritage::guardCalls`, `heritage.cc:1522`), and `Funcdata::ancestorOpUse`
//! (`funcdata_varnode.cc:1963`) refuses such a value as a trial for the next
//! call's input or the function's RETURN: "an indirectCreation is an
//! indication of an output trial, this should not count as an only use".  The
//! refusal is a design choice: `call g; ret` is both `void f(void) { g(); }`
//! and `T f(void) { return g(); }`.  Second, `Funcdata::onlyOpUse`
//! (`funcdata_varnode.cc:1851`) refuses a Varnode two different callees read,
//! and the retained object saved in `x22` is one Varnode after copy
//! propagation.  Third, heritage visits only a range some op reads or writes,
//! so `bl g; bl g2` with `x0` handed straight on is never heritaged there and
//! the second call has no trial to refuse.
//!
//! # The rule
//!
//! `on`: a trial whose root is on every path the value a CALL or CALLIND left
//! in the first general-purpose result register, its output or an INDIRECT
//! creation marked a possible output ([`call_result_root`]), is scored like
//! any other value as the next call's input, and another call reading the same
//! value as an argument does not refuse it ([`scoring_argument`]).  The
//! RETURNs get a read of the register when nothing else names it ([`plant`]),
//! so the hand-off is heritaged at all; that trial is settled by
//! [`settle_planted`].  The RETURN keeps the strict reading: `bl g; cbz x0,L;
//! ...; L: ret` is `long h(long a) { long r = g(a); if (r) *p = 1; return r; }`
//! and `void h(long a) { if (g(a)) *p = 1; }` alike, and only the callers
//! decide it (`voidret`).  `all`: a result that reaches every live RETURN
//! untouched is the return value too ([`scoring_return`]), the reading a
//! reader who takes an untouched `x0` at the `ret` for the result makes.  A root the function computes itself on
//! another path, or beside a value the function returns in another storage
//! class ([`yield_to_own_value`]), keeps the strict reading in every mode.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::space::spacetype;
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::context::{OpId, VarnodeId};
use crate::dtype::{type_class, type_metatype};
use crate::funcdata::Funcdata;

/// (kuna `callresultuse`) Which untouched call results count as the value
/// handed on.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum CallResultUse {
    /// Upstream's reading: a call result is never a trial's only use.
    #[default]
    Off,
    /// The next call's argument.
    On,
    /// `On`, and the return value whenever it reaches the RETURN untouched.
    All,
}

impl CallResultUse {
    pub fn as_str(self) -> &'static str {
        match self {
            CallResultUse::Off => "off",
            CallResultUse::On => "on",
            CallResultUse::All => "all",
        }
    }
}

/// The `callresultuse` option parser.
pub struct OptionCallResultUse;

impl OptionCallResultUse {
    pub fn apply(&self, p1: &str) -> kuna_base::error::KunaResult<(CallResultUse, String)> {
        let mode = match p1 {
            "off" => CallResultUse::Off,
            "on" | "" => CallResultUse::On,
            "all" => CallResultUse::All,
            other => {
                return Err(kuna_base::error::KunaError::parse(format!(
                    "Unknown callresultuse value: {other} (expected off|on|all)"
                )))
            }
        };
        Ok((mode, format!("Unknown callee results read by the next call or the return set to {}", mode.as_str())))
    }
}

/// Is the trial rooted at `root` for `opmatch` (a CALL or CALLIND input, or
/// a RETURN) one a call result satisfies: [`call_result_root`] for a call,
/// [`scoring_return`] for a RETURN?
pub fn scores_trial(data: &Funcdata, root: VarnodeId, opmatch: OpId) -> bool {
    match data.obank().get(opmatch).map(|o| o.code()) {
        Some(OpCode::CPUI_RETURN) => scoring_return(data, opmatch, root),
        Some(OpCode::CPUI_CALL | OpCode::CPUI_CALLIND) => call_result_root(data, root),
        _ => false,
    }
}

/// Is `root` a call result on every path: its definition, through
/// MULTIEQUALs, COPYs and the INDIRECT effects of later calls, is always a
/// CALL's output or creation [`scores_def`] accepts?  A root one path computes
/// in the function itself (`fnstsw ax` on one arm, a call's `ax` on the other)
/// is not, and keeps the strict reading.  Off with the option.
pub fn call_result_root(data: &Funcdata, root: VarnodeId) -> bool {
    data.get_arch().call_result_use != CallResultUse::Off && walk_call_results(data, root)
}

/// Does every definition behind `root` end at a call result [`scores_def`]
/// accepts, and at least one?
fn walk_call_results(data: &Funcdata, root: VarnodeId) -> bool {
    let mut seen = crate::kuna_fasthash::FxHashSet::default();
    let mut stack = vec![root];
    let mut any = false;
    while let Some(vn) = stack.pop() {
        if !seen.insert(vn) {
            continue;
        }
        let Some(def) = data.vbank().get(vn).and_then(|v| v.get_def()) else { return false };
        let Some(op) = data.obank().get(def) else { return false };
        match op.code() {
            OpCode::CPUI_MULTIEQUAL => stack.extend((0..op.num_input()).filter_map(|i| op.get_in(i))),
            OpCode::CPUI_COPY => stack.extend(op.get_in(0)),
            OpCode::CPUI_INDIRECT if !op.is_indirect_creation() => stack.extend(op.get_in(0)),
            _ if scores_def(data, def) => any = true,
            _ => return false,
        }
    }
    any
}

/// Is `def` a call's result in the first result register: the CALL or
/// CALLIND itself when the callee's output is known, or else the INDIRECT
/// creation of a register the callee may have left its result in (a creation
/// whose zero input is not an "indirect zero", the range being a possible
/// output of the callee's model, and whose effect is a CALL or CALLIND)?
fn scores_def(data: &Funcdata, def: OpId) -> bool {
    let Some(op) = data.obank().get(def) else { return false };
    let Some(out) = op.get_out().and_then(|v| data.vbank().get(v)) else { return false };
    let first_register = first_result_register(data, out.get_addr(), out.get_size());
    match op.code() {
        OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => first_register,
        OpCode::CPUI_INDIRECT if op.is_indirect_creation() => {
            let possible_output = op
                .get_in(0)
                .and_then(|v| data.vbank().get(v))
                .is_some_and(|v| !v.is_indirect_zero());
            possible_output
                && first_register
                && crate::kuna_retcallhalf::created_by_call(data, op)
                && !callee_result_elsewhere(data, op, out.get_addr(), out.get_size())
        }
        _ => false,
    }
}

/// Is the call `creation` is the effect of one whose callee's output is
/// settled somewhere other than `[addr, addr+size)`: a prototype with its
/// output locked and no return value, or one returned in other storage (a
/// `double` in `d0` on hard-float ARM, whose `r0` the call also clobbers)?
/// The register the creation stands for is then not a result.
fn callee_result_elsewhere(data: &Funcdata, creation: &crate::op::PcodeOp, addr: &Address, size: int4) -> bool {
    let Some(iop) = creation.get_in(1).and_then(|v| data.vbank().get(v)) else { return false };
    let call = OpId::from(slotmap::KeyData::from_ffi(iop.get_offset()));
    let end = addr.get_offset().wrapping_add(size.max(0) as u64);
    (0..data.num_calls()).any(|i| {
        let fc = data.get_call_specs(i);
        if fc.get_op() != call {
            return false;
        }
        let disjoint = |out_addr: &Address, out_size: int4| {
            let out_end = out_addr.get_offset().wrapping_add(out_size.max(0) as u64);
            out_addr.get_space().map(|s| s.get_index()) != addr.get_space().map(|s| s.get_index())
                || out_end <= addr.get_offset()
                || end <= out_addr.get_offset()
        };
        if fc.proto().is_output_locked() {
            let out = fc.proto().get_output();
            return fc.proto().get_output_type().is_none_or(|t| t.get_metatype() == type_metatype::TYPE_VOID)
                || disjoint(&out.get_address(), out.get_size());
        }
        // `protoorder types` states the callee's recovery without locking it;
        // its result storage is recorded only with `passthrough` on.
        match data.kuna_protoorder_types(fc.get_entry_address()).map(|s| s.result.as_ref()) {
            Some(Some((out_addr, out_size))) => disjoint(out_addr, *out_size),
            Some(None) => data.get_arch().pass_through,
            None => false,
        }
    })
}

/// Does `[addr, addr+size)` lie in the output model's first general-purpose
/// register (`x0`, `rax`, `r0`), the one a pointer or integer result is
/// returned in?  The second register of a pair (`x1`, `rdx`) is never a result
/// on its own, and a value a call left there is not taken for one; a float
/// result (`d0`) is left to the stricter reading, since a receiver or an
/// argument handed on is a pointer.
fn first_result_register(data: &Funcdata, addr: &Address, size: int4) -> bool {
    let Some(list) = data.get_func_proto().model().output_list() else { return false };
    let end = addr.get_offset().wrapping_add(size.max(0) as u64);
    list.get_entry().iter().any(|e| {
        e.is_first_in_class()
            && e.get_type() == type_class::TYPECLASS_GENERAL
            && e.get_join_record().is_none()
            && addr.get_space().is_some_and(|sp| sp.get_index() == e.get_space().get_index())
            && e.get_base() <= addr.get_offset()
            && end <= e.get_base().wrapping_add(e.get_size().max(0) as u64)
    })
}

/// Is `opmatch` a RETURN scoring `root`, a call result on every path
/// ([`call_result_root`]) at this RETURN and at every other live RETURN of
/// the function, under `all`?  A RETURN that hands back the function's own
/// incoming register or a value it computed (`scale_err`: `oflow`'s `r0` on
/// one exit, the untouched `r0` beside the `s0` it converts on the other)
/// keeps the strict reading, where only the value every exit agrees on is a
/// return.  Under `on` the RETURN keeps the strict reading throughout: `bl g;
/// cbz x0,L; ...; L: ret` is `long h(long a) { long r = g(a); if (r) *p = 1;
/// return r; }` and `void h(long a) { if (g(a)) *p = 1; }` alike.
pub fn scoring_return(data: &Funcdata, opmatch: OpId, root: VarnodeId) -> bool {
    if data.get_arch().call_result_use != CallResultUse::All {
        return false;
    }
    let Some(ret) = data.obank().get(opmatch).filter(|o| o.code() == OpCode::CPUI_RETURN) else { return false };
    let Some(slot) = (0..ret.num_input()).find(|&i| ret.get_in(i) == Some(root)) else { return false };
    every_return_hands_back_a_call_result(data, slot)
}

/// Does every live RETURN read, in `slot`, a value that is a call result on
/// every path ([`walk_call_results`])?
fn every_return_hands_back_a_call_result(data: &Funcdata, slot: int4) -> bool {
    let mut rets = data
        .obank()
        .iter_code(OpCode::CPUI_RETURN)
        .filter(|&r| data.obank().get(r).is_some_and(|o| !o.is_dead() && o.get_halt_type() == 0))
        .peekable();
    rets.peek().is_some()
        && rets.all(|r| {
            data.obank().get(r).and_then(|o| o.get_in(slot)).is_some_and(|vn| walk_call_results(data, vn))
        })
}

/// Is `opmatch` a CALL or CALLIND scoring `root`, a call result on every path
/// ([`call_result_root`])?  `only_op_use` then lets another call read the
/// same value as an argument: `x = [a allKeys]; [x containsObject:k];
/// objc_release(x)` passes the retained object to two callees, and upstream
/// refuses a Varnode two different callees read (`checkCallDoubleUse` allows
/// only the same callee), which dropped the receiver of the first.  A value
/// a call produced and the function went on to hand around is a value, not
/// a register two callees happened to share.
pub fn scoring_argument(data: &Funcdata, opmatch: OpId, root: VarnodeId) -> bool {
    data.obank().get(opmatch).is_some_and(|o| matches!(o.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND))
        && call_result_root(data, root)
}

/// Is `op`'s use of `vn` one a returned call result may also have before the
/// RETURN: a branch on it, a store of it as the stored value, or another
/// RETURN? The same uses `voidret` lets a forced return have.
pub fn returned_use(data: &Funcdata, op: OpId, vn: VarnodeId) -> bool {
    crate::p4_calls::kuna_voidret::returned_use(data, op, vn)
}

/// Mark the active return trials that hold a call result on every path
/// ([`call_result_root`]) inactive again when the function returns a value of
/// its own in another storage class (`passthrough`'s `returns_own_value`): in
/// `split`, a tested `eax` left by `sub_401195` beside the `st0` sum the
/// function computes, the convention returns in one class and the sum is the
/// return value, the callee's integer a leftover.  Nothing is done with the
/// option off.
pub fn yield_to_own_value(data: &Funcdata, active: &mut crate::fspec::ParamActive, return_ops: &[OpId]) {
    if data.get_arch().call_result_use == CallResultUse::Off {
        return;
    }
    let live: Vec<OpId> = return_ops
        .iter()
        .copied()
        .filter(|&r| data.obank().get(r).is_some_and(|o| !o.is_dead() && o.get_halt_type() == 0))
        .collect();
    if live.is_empty() {
        return;
    }
    let mine: Vec<int4> = (0..active.get_num_trials())
        .filter(|&i| {
            let t = active.get_trial(i);
            t.is_active()
                && live.iter().all(|&r| {
                    data.obank()
                        .get(r)
                        .and_then(|o| o.get_in(t.get_slot()))
                        .is_some_and(|vn| walk_call_results(data, vn))
                })
        })
        .collect();
    if !mine.is_empty() && crate::p4_calls::kuna_passthrough::returns_own_value(data, active, &mine) {
        for i in mine {
            active.get_trial_mut(i).mark_inactive();
        }
    }
}

/// Give the RETURNs a read of the result register, and the function a return
/// trial there, when the function calls something and no op of it names the
/// register: heritage visits only a range some op reads or writes, and `bl g;
/// bl g2` names none, so without the read `x0` is never heritaged, the first
/// call never gets its output trial and the second never its input trial
/// there, and the hand-off is lost before it can be scored.  The planted
/// trial itself is settled by [`settle_planted`] once the value at each
/// RETURN is known.  The register is the output model's first general-purpose
/// one, whole; nothing is planted with the option off, a locked output, no
/// call, a range some op of the function names ([`named`]), a stated input of
/// a first call ([`first_call_input`]), or a trial `voidret` already planted
/// there.
pub fn plant(data: &mut Funcdata) {
    if data.get_arch().call_result_use == CallResultUse::Off
        || data.get_active_output().is_none()
        || data.get_func_proto().is_output_locked()
        || data.num_calls() == 0
    {
        return;
    }
    let Some((addr, size)) = result_register(data) else { return };
    if named(data, &addr, size)
        || first_call_input(data, &addr, size)
        || crate::p4_calls::kuna_voidret::planted_overlaps(data, &addr, size)
    {
        return;
    }
    crate::p4_calls::kuna_voidret::plant_piece(data, addr.clone(), size);
    if crate::p4_calls::kuna_voidret::planted_overlaps(data, &addr, size) {
        data.kuna_note_callresultuse_planted(addr, size);
    }
}

/// Settle the return trial [`plant`] made: active only under `all`, and only
/// when the value at every RETURN is a call result on every path
/// ([`every_return_hands_back_a_call_result`]); otherwise inactive.  Without this the read an
/// untouched input register reaches would make the trial active, as a read of
/// an input does (`scale_err`: `oflow`'s `r0` on one exit, the function's own
/// incoming `r0` on the other, printed `return a1`).  Under `on` the hand-off
/// was the plant's only purpose.
pub fn settle_planted(data: &Funcdata, active: &mut crate::fspec::ParamActive) {
    let Some((addr, size)) = data.kuna_callresultuse_planted() else { return };
    let i = active.which_trial(addr, *size);
    if i < 0 {
        return;
    }
    let slot = active.get_trial(i).get_slot();
    let keep = data.get_arch().call_result_use == CallResultUse::All
        && every_return_hands_back_a_call_result(data, slot);
    if keep {
        active.get_trial_mut(i).mark_active();
    } else {
        active.get_trial_mut(i).mark_inactive();
    }
}

/// Does some Varnode of the function share a byte with `[addr, addr+size)`?
/// Heritage then visits the range on its own, and the trials it registers
/// are scored as they always were.
fn named(data: &Funcdata, addr: &Address, size: int4) -> bool {
    crate::p4_calls::kuna_passthrough::touched(data, addr, size)
}

/// Is `register` a stated input of a call no other call can execute before
/// (`passthrough`'s `first_calls`)?  Such a call reads the function's own
/// incoming register, not a result, and `passthrough` claims it as the
/// function's parameter handed through; planting a RETURN read there would
/// turn the claim into a trial of the function's own output.
fn first_call_input(data: &Funcdata, register: &Address, size: int4) -> bool {
    let first = crate::p4_calls::kuna_passthrough::first_calls(data);
    (0..data.num_calls()).any(|idx| {
        let fc = data.get_call_specs(idx);
        first.contains(&fc.get_op())
            && data
                .kuna_protoorder_types(fc.get_entry_address())
                .is_some_and(|s| s.inputs.iter().any(|(a, sz, _)| a == register && *sz == size))
    })
}

/// The output model's first general-purpose register (`x0`, `rax`, `r0`), whole.
fn result_register(data: &Funcdata) -> Option<(Address, int4)> {
    let list = data.get_func_proto().model().output_list()?;
    let entry = list.get_entry().iter().find(|e| {
        e.is_first_in_class()
            && e.get_join_record().is_none()
            && e.get_type() == type_class::TYPECLASS_GENERAL
            && e.get_space().get_type() == spacetype::IPTR_PROCESSOR
    })?;
    Some((Address::new(Rc::clone(entry.get_space()), entry.get_base()), entry.get_size()))
}
