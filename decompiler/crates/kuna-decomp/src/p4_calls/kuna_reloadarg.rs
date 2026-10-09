//! (kuna) A register reloaded from the caller's frame is no argument when the
//! slot only gave back a scratch register no prototype passes: option
//! `reloadarg`.
//!
//! clang -O2 keeps the stack aligned around a single call with `push %rax` and
//! takes the slot back with `pop %rcx` before tail-calling a variadic function:
//! `push %rax; call h; lea fmt,%rdi; mov %eax,%esi; xor %eax,%eax; pop %rcx;
//! jmp pr`. `ActionActiveParam` scores register trials on its first pass,
//! before `ActionStackPtrFlow` and the stack heritage, and a trial is never
//! scored again once checked. At that point `rcx` is a raw `LOAD [rsp]`, which
//! `AncestorRealistic` takes as solid movement, so the trial was active and
//! `fillinMap` filled `rdx` in below it: `return pr("%d",(ulong)v1,v3,v2);`.
//!
//! [`note`] marks an active register trial whose walk stopped at a LOAD from
//! the caller's frame and asks for a final check. [`recheck`] runs when the
//! call is finalized, after the stack heritage has turned the LOAD into the
//! slot's own data flow, and follows that flow back through copies and the
//! guards calls put on a slot whose address never escapes ([`junk`]). The
//! value is junk only when every leaf is a register input the function's own
//! prototype kills across calls and cannot take as a parameter (`rax`, `r10`,
//! `r11` under SysV): a callee-saved register such as a saved frame pointer
//! (`__builtin_frame_address(1)`) is a real value. Junk trials are dropped from
//! the top of the call's register order only: a dropped trial is a
//! definitely-unused one, and `fillinMap` cuts every trial above such a trial,
//! so a junk register below a live argument (`pop %rdx` under `mov $10,%ecx`)
//! stays a hole-filled argument. A Go image passes arguments in registers the
//! cspec calls scratch, so the rule stands down there.

use kuna_base::address::Address;
use kuna_base::space::spacetype;
use kuna_base::types::int4;

use kuna_num::opcodes::OpCode;

use crate::context::{OpId, VarnodeId};
use crate::fspec::effect_type;
use crate::funcdata::Funcdata;
use crate::varmap::AliasChecker;

/// Flag trial `i` of call spec `idx` for [`recheck`] when one of `loads`, the
/// LOADs its walk took as solid, reads the caller's frame.
pub(crate) fn note(data: &mut Funcdata, idx: int4, i: int4, loads: &[OpId]) {
    if loads.is_empty() || !data.get_arch().reload_arg || data.get_arch().source_is_go {
        return;
    }
    let Some(sp) = super::kuna_spillargtrial::stack_pointer_storage(data) else { return };
    let from_frame = loads.iter().any(|&load| {
        data.obank()
            .get(load)
            .and_then(|o| o.get_in(1))
            .is_some_and(|ptr| super::kuna_spillargtrial::frame_slot(data, ptr, &sp).is_some())
    });
    if from_frame {
        let active = data.get_call_specs_mut(idx).get_active_input();
        active.get_trial_mut(i).set_frame_reload();
        active.mark_needs_final_check();
    }
}

/// Drop the flagged trials of call spec `idx` that [`junk`] rejects and that
/// no other active trial sits above in the callee model's order, unless the
/// call's format string names them.
pub(crate) fn recheck(data: &mut Funcdata, idx: int4, mut alias: Option<&mut AliasChecker>) {
    if !data.get_call_specs(idx).proto().has_model() {
        return;
    }
    let op = data.get_call_specs(idx).get_op();
    let mut order: Vec<(int4, int4, bool)> = Vec::new();
    let mut flagged = false;
    for i in 0..data.get_call_specs(idx).active_input().get_num_trials() {
        let t = data.get_call_specs(idx).active_input().get_trial(i);
        if !t.is_active() {
            continue;
        }
        let (addr, size, reload) = (t.get_address().clone(), t.get_size(), t.has_frame_reload());
        let (mut slot, mut slotsize) = (0, 0);
        let model = data.get_call_specs(idx).proto().model();
        if !model.possible_input_param_with_slot(&addr, size, &mut slot, &mut slotsize) {
            slot = int4::MAX;
        }
        order.push((slot, i, reload));
        flagged |= reload;
    }
    if !flagged {
        return;
    }
    order.sort_by(|a, b| b.cmp(a));
    let mut format_arguments = None;
    for (_, i, reload) in order {
        if !reload {
            return;
        }
        let slot = data.get_call_specs(idx).active_input().get_trial(i).get_slot();
        let Some(vn) = data.obank().get(op).and_then(|o| o.get_in(slot)) else { return };
        if !junk(data, vn, 16, &mut alias) {
            return;
        }
        let args = format_arguments.get_or_insert_with(|| {
            super::kuna_varargformat::arguments(data, data.get_call_specs(idx))
        });
        if super::kuna_varargformat::activate_trial(data, idx, i, args) {
            return;
        }
        data.get_call_specs_mut(idx).get_active_input().get_trial_mut(i).mark_no_use();
        let width = data.vbank().get(vn).map(|v| v.get_size()).unwrap_or(0);
        let zero = data.new_constant(width, 0);
        let _ = data.op_set_input(op, zero, slot);
    }
}

/// Is the register input at `addr` one the function's prototype kills across
/// calls and cannot take as a parameter?
fn scratch_input(data: &Funcdata, addr: &Address, size: int4) -> bool {
    let proto = data.get_func_proto();
    proto.has_effect(addr, size) == effect_type::KILLEDBYCALL && !proto.possible_input_param(addr, size)
}

/// Does `vn` hold, through copies, truncations, concatenations and call
/// guards on a stack slot whose address never escapes, only register inputs
/// [`scratch_input`] accepts?
fn junk(data: &Funcdata, vn: VarnodeId, depth: u32, alias: &mut Option<&mut AliasChecker>) -> bool {
    let Some(v) = data.vbank().get(vn) else { return false };
    if v.is_input() {
        return v.get_space().get_type() == spacetype::IPTR_PROCESSOR
            && !v.is_persist()
            && !v.is_unaffected()
            && !v.is_direct_write()
            && scratch_input(data, v.get_addr(), v.get_size());
    }
    let Some(op) = v.get_def().and_then(|d| data.obank().get(d)).filter(|_| depth > 0) else { return false };
    let next = |i: int4| op.get_in(i);
    match op.code() {
        OpCode::CPUI_COPY | OpCode::CPUI_SUBPIECE => next(0).is_some_and(|n| junk(data, n, depth - 1, alias)),
        OpCode::CPUI_PIECE => {
            next(0).is_some_and(|n| junk(data, n, depth - 1, alias))
                && next(1).is_some_and(|n| junk(data, n, depth - 1, alias))
        }
        OpCode::CPUI_INDIRECT if !op.is_indirect_creation() && !op.is_indirect_store() => {
            let space = v.get_space().clone();
            let offset = v.get_offset();
            space.get_type() == spacetype::IPTR_SPACEBASE
                && alias.as_mut().is_some_and(|a| {
                    !a.has_local_alias(Some((space, offset)), &mut data.alias_gather_access())
                })
                && next(0).is_some_and(|n| junk(data, n, depth - 1, alias))
        }
        _ => false,
    }
}
