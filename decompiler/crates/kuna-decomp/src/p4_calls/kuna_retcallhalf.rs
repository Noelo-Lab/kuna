//! (kuna) Keep the first register of a returned pair when it is a call's
//! result handed back untouched and the function computes the second.
//!
//! `u64 f(unsigned a) { return full(a) | 0xff00000000ULL; }` is, on 32-bit ARM,
//! `bl full; orr r1,r1,#255; bx lr`: `r0` comes back from `full` as it is and
//! only `r1` is written. Return recovery scores each return register on its
//! own. `r1` passes, but `r0` at the RETURN is the call's INDIRECT creation,
//! which upstream's `ancestorOpUse` refuses outright, and the output model
//! never returns the second register of a pair without the first. The function
//! printed as `void f(int a0) { full(a0); }`.
//!
//! A register that is not the first of its class is never a return value of
//! its own, so a function that computes one and hands it only to the RETURN
//! returns the pair it belongs to. When the first register of that pair is, at
//! every live RETURN, the untouched result of a call ([`is_call_result`]), the
//! function hands that result back as the pair's first half, and [`accept`]
//! makes the trial active.
//!
//! The second register has to have been computed on purpose
//! ([`computed_on_purpose`]), because four kinds of write reach a RETURN in a
//! function that returns one register or none:
//!
//! * the other output of an instruction that writes two registers, such as
//!   the low word of an ARM `smull` whose high word is a division by a
//!   constant;
//! * a restore from the frame, such as gcc's `pop {r1,r2,pc}` that releases
//!   stack space, or an `-O0` reload of a spilled argument;
//! * an argument set up for a later call or `svc` whose p-code does not show
//!   it reading the register;
//! * the zero `-fzero-call-used-regs` leaves in every call-used register a
//!   function does not return in.
//!
//! The rest stays as upstream decides it. The model must return nothing
//! without the call's register and exactly the two registers with it, so a
//! value the function returns in another class is never joined to a leftover.
//! A function that writes the call's own register and leaves the second
//! untouched is not changed: `bl g; orr r0,r0,#255` is byte for byte
//! `int f(void) { return (int)g() | 255; }` as well as a 64-bit return. And a
//! big-endian pair is left alone, because a pair is still joined in
//! little-endian order there.

use kuna_base::address::Address;
use kuna_base::space::spacetype;
use kuna_num::opcodes::OpCode;

use crate::context::{OpId, VarnodeId};
use crate::fspec::ParamActive;
use crate::funcdata::Funcdata;

/// How many move-only ops and phis the walks follow.
const MAX_DEPTH: u32 = 8;

/// How many defining ops the zero test follows.
const ZERO_DEPTH: u32 = 8;

/// Mark active the inactive return trial that is a call's untouched result
/// beside a register the function computes, when together they are the pair
/// the output model returns.
///
/// Runs once `ActionReturnRecovery` has scored every trial for the last time,
/// before the output map is derived.
pub fn accept(data: &Funcdata, active: &mut ParamActive, return_ops: &[OpId]) {
    let active_trials: Vec<i32> = (0..active.get_num_trials()).filter(|&i| active.get_trial(i).is_active()).collect();
    let [second] = active_trials[..] else { return };
    let rets: Vec<OpId> = return_ops
        .iter()
        .copied()
        .filter(|&r| data.obank().get(r).is_some_and(|o| !o.is_dead() && o.get_halt_type() == 0))
        .collect();
    let value_at = |r: OpId, slot: i32| data.obank().get(r).and_then(|o| o.get_in(slot));
    let (second_addr, second_size, second_slot) = {
        let t = active.get_trial(second);
        (t.get_address().clone(), t.get_size(), t.get_slot())
    };
    let candidates: Vec<i32> = (0..active.get_num_trials())
        .filter(|&i| {
            let t = active.get_trial(i);
            !t.is_active()
                && !t.is_definitely_not_used()
                && t.get_size() == second_size
                && !t.get_address().is_big_endian()
                && rets.iter().all(|&r| value_at(r, t.get_slot()).is_some_and(|vn| is_call_result(data, vn, MAX_DEPTH)))
        })
        .collect();
    let values: Vec<(OpId, VarnodeId)> =
        rets.iter().filter_map(|&r| value_at(r, second_slot).map(|vn| (r, vn))).collect();
    if candidates.is_empty()
        || rets.is_empty()
        || values.len() != rets.len()
        || derives_any(data, active)
        || values.iter().all(|&(_, vn)| is_zero(data, vn, ZERO_DEPTH))
        || !values.iter().all(|&(r, vn)| computed_on_purpose(data, vn, r, MAX_DEPTH))
    {
        return;
    }
    let manager = data.get_arch().manage.clone();
    for i in candidates {
        let first_addr = active.get_trial(i).get_address().clone();
        let mut probe = active.clone();
        probe.get_trial_mut(i).mark_active();
        if data.get_func_proto().derive_output_map(&mut probe, &manager).is_err() {
            continue;
        }
        let used: Vec<Address> = (0..probe.get_num_trials())
            .map(|k| probe.get_trial(k))
            .filter(|t| t.is_used())
            .map(|t| t.get_address().clone())
            .collect();
        if used == [first_addr, second_addr.clone()] {
            active.get_trial_mut(i).mark_active();
            return;
        }
    }
}

/// Does the output model return anything from the trials as they stand?
fn derives_any(data: &Funcdata, active: &ParamActive) -> bool {
    let mut probe = active.clone();
    let manager = data.get_arch().manage.clone();
    if data.get_func_proto().derive_output_map(&mut probe, &manager).is_err() {
        return true;
    }
    (0..probe.get_num_trials()).any(|k| probe.get_trial(k).is_used())
}

/// Is `vn` the value a call left in its register, untouched since: the call's
/// INDIRECT creation, read directly, through the injected no-op of a return's
/// mode switch, or through a phi of such values?
fn is_call_result(data: &Funcdata, vn: VarnodeId, depth: u32) -> bool {
    if depth == 0 {
        return false;
    }
    let Some(def) = data.vbank().get(vn).and_then(|v| v.get_def()) else { return false };
    let Some(op) = data.obank().get(def) else { return false };
    match op.code() {
        OpCode::CPUI_COPY if crate::kuna_passthrough::is_injected_noop(data, def) => {
            op.get_in(0).is_some_and(|i| is_call_result(data, i, depth - 1))
        }
        OpCode::CPUI_MULTIEQUAL => (0..op.num_input())
            .all(|k| op.get_in(k).is_some_and(|i| i != vn && is_call_result(data, i, depth - 1))),
        OpCode::CPUI_INDIRECT => op.is_indirect_creation() && created_by_call(data, op),
        _ => false,
    }
}

/// Is the INDIRECT creation `op` a call's clobber of its output?
fn created_by_call(data: &Funcdata, op: &crate::op::PcodeOp) -> bool {
    let Some(iop) = op.get_in(1).and_then(|i| data.vbank().get(i)) else { return false };
    let call = OpId::from(slotmap::KeyData::from_ffi(iop.get_offset()));
    data.obank().get(call).is_some_and(|c| matches!(c.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND))
}

/// Was `vn`, read by the RETURN `ret`, written by an instruction of the
/// function on purpose, on every path?
///
/// Every producing instruction must write no register but its own and not the
/// stack pointer, must not reload the value from the frame, and must not be
/// followed on the way to `ret` by a call or a CALLOTHER, which can read the
/// register as an argument its p-code does not show. A call's result merged in
/// on one path is a value too. An unwritten Varnode counts only when it is not
/// a frame slot.
fn computed_on_purpose(data: &Funcdata, vn: VarnodeId, ret: OpId, depth: u32) -> bool {
    if depth == 0 {
        return false;
    }
    let Some(v) = data.vbank().get(vn) else { return false };
    if v.is_constant() {
        return true;
    }
    let Some(def) = v.get_def() else { return !in_frame(v.get_addr()) };
    let Some(op) = data.obank().get(def) else { return false };
    match op.code() {
        OpCode::CPUI_MULTIEQUAL => (0..op.num_input()).all(|k| {
            op.get_in(k).is_some_and(|i| {
                i != vn && (is_call_result(data, i, depth - 1) || computed_on_purpose(data, i, ret, depth - 1))
            })
        }),
        OpCode::CPUI_INDIRECT if op.is_indirect_creation() => false,
        OpCode::CPUI_INDIRECT => op.get_in(0).is_some_and(|i| computed_on_purpose(data, i, ret, depth - 1)),
        _ if writes_elsewhere(data, op.get_addr(), v.get_addr(), v.get_size()) => false,
        _ if crate::kuna_retinputhalf::call_between(data, def, ret) => false,
        OpCode::CPUI_COPY => op.get_in(0).and_then(|i| data.vbank().get(i)).is_some_and(|i| !in_frame(i.get_addr())),
        OpCode::CPUI_LOAD => op.get_in(1).is_some_and(|p| !frame_pointer(data, p, MAX_DEPTH)),
        _ => true,
    }
}

/// Does the instruction at `pc` write the stack pointer, or a register wider
/// than a flag outside `[addr, addr+size)`?
fn writes_elsewhere(data: &Funcdata, pc: &Address, addr: &Address, size: i32) -> bool {
    if crate::kuna_retinputhalf::writes_stack_pointer(data, pc) {
        return true;
    }
    data.obank().iter_at(pc).any(|(_, op)| {
        let Some(out) = data.obank().get(op).and_then(|o| o.get_out()).and_then(|o| data.vbank().get(o)) else {
            return false;
        };
        let a = out.get_addr();
        let register = a.get_space().is_some_and(|s| s.get_type() == spacetype::IPTR_PROCESSOR);
        register && out.get_size() > 1 && a.overlap(0, addr, size) < 0 && addr.overlap(0, a, out.get_size()) < 0
    })
}

/// Is `addr` in the function's stack frame?
fn in_frame(addr: &Address) -> bool {
    addr.get_space().is_some_and(|s| s.get_type() == spacetype::IPTR_SPACEBASE)
}

/// Is the pointer `vn` the stack pointer, or the stack pointer plus or minus
/// a constant?
fn frame_pointer(data: &Funcdata, vn: VarnodeId, depth: u32) -> bool {
    let Some(v) = data.vbank().get(vn) else { return false };
    if v.is_spacebase() || is_stack_pointer(data, v.get_addr()) {
        return true;
    }
    let Some(op) = v.get_def().and_then(|d| data.obank().get(d)).filter(|_| depth > 0) else { return false };
    match op.code() {
        OpCode::CPUI_COPY
        | OpCode::CPUI_INT_ADD
        | OpCode::CPUI_INT_SUB
        | OpCode::CPUI_PTRSUB
        | OpCode::CPUI_PTRADD
        | OpCode::CPUI_INDIRECT => op.get_in(0).is_some_and(|i| frame_pointer(data, i, depth - 1)),
        _ => false,
    }
}

/// Is `addr` the stack pointer register?
fn is_stack_pointer(data: &Funcdata, addr: &Address) -> bool {
    let Some(stack) = data.get_arch().manage().get_stack_space().cloned() else { return false };
    let Ok(sp) = stack.get_spacebase(0) else { return false };
    sp.space.as_ref().is_some_and(|s| addr.get_space().is_some_and(|a| a.get_index() == s.get_index()))
        && addr.get_offset() == sp.offset
}

/// Is `vn` zero whatever the function's inputs: a zero constant, `x ^ x` or
/// `x - x`, or a copy, extension, piece or concatenation of zeros?
fn is_zero(data: &Funcdata, vn: VarnodeId, depth: u32) -> bool {
    let Some(v) = data.vbank().get(vn) else { return false };
    if v.is_constant() {
        return v.get_offset() == 0;
    }
    let Some(o) = v.get_def().filter(|_| depth > 0).and_then(|d| data.obank().get(d)) else { return false };
    let zero = |k: i32| o.get_in(k).is_some_and(|i| is_zero(data, i, depth - 1));
    match o.code() {
        OpCode::CPUI_COPY | OpCode::CPUI_INT_ZEXT | OpCode::CPUI_INT_SEXT => zero(0),
        OpCode::CPUI_SUBPIECE => zero(0) || piece_of_zero(data, o, depth - 1),
        OpCode::CPUI_PIECE | OpCode::CPUI_INT_OR => zero(0) && zero(1),
        OpCode::CPUI_INT_XOR | OpCode::CPUI_INT_SUB => {
            same_value(data, o.get_in(0), o.get_in(1)) || (zero(0) && zero(1))
        }
        OpCode::CPUI_INT_AND | OpCode::CPUI_INT_MULT => zero(0) || zero(1),
        _ => false,
    }
}

/// Is the SUBPIECE `o` exactly one half of a PIECE, and that half zero?
fn piece_of_zero(data: &Funcdata, o: &crate::op::PcodeOp, depth: u32) -> bool {
    let size = |v: Option<VarnodeId>| v.and_then(|v| data.vbank().get(v)).map(|v| v.get_size());
    let Some(whole) = o.get_in(0).and_then(|w| data.vbank().get(w)).and_then(|w| w.get_def()) else { return false };
    let Some(p) = data.obank().get(whole).filter(|p| p.code() == OpCode::CPUI_PIECE) else { return false };
    let off = o.get_in(1).and_then(|c| data.vbank().get(c)).map(|c| c.get_offset() as i32);
    let out = size(o.get_out());
    let (hi, lo) = (p.get_in(0), p.get_in(1));
    match off {
        Some(0) if out == size(lo) => lo.is_some_and(|l| is_zero(data, l, depth)),
        Some(k) if Some(k) == size(lo) && out == size(hi) => hi.is_some_and(|h| is_zero(data, h, depth)),
        _ => false,
    }
}

/// Are `a` and `b` the same value: one Varnode, or the same piece of one?
fn same_value(data: &Funcdata, a: Option<VarnodeId>, b: Option<VarnodeId>) -> bool {
    let (Some(a), Some(b)) = (a, b) else { return false };
    if a == b {
        return true;
    }
    let def = |v: VarnodeId| data.vbank().get(v).and_then(|v| v.get_def()).and_then(|d| data.obank().get(d));
    let (Some(x), Some(y)) = (def(a), def(b)) else { return false };
    let offset = |o: &crate::op::PcodeOp| o.get_in(1).and_then(|c| data.vbank().get(c)).map(|c| c.get_offset());
    x.code() == OpCode::CPUI_SUBPIECE
        && y.code() == OpCode::CPUI_SUBPIECE
        && x.get_in(0) == y.get_in(0)
        && offset(x) == offset(y)
}
