//! Preserve full VFP parameter/return storage when the ARM container states that ABI.
use crate::{
    context::VarnodeId,
    dtype::{type_class, type_metatype},
    fspec::{ParamActive, ParamEntry, ParamListStandard, PrototypePieces, ProtoModel},
    funcdata::Funcdata,
    infra::architecture::Architecture,
};
use kuna_base::{address::Address, space::AddrSpaceManager};
use kuna_num::opcodes::OpCode;
use std::borrow::Cow;
use std::rc::Rc;

/// The option is on and the loaded ARM image states the VFP procedure-call standard.
pub fn applies(arch: &Architecture) -> bool {
    arch.arm_float_return
        && arch.archid.starts_with("ARM:")
        && arch.translate().loader_rc().borrow().arm_vfp_args()
}

/// A recovered prototype's open tail read as a closed list for the canonical-storage
/// check: a variadic callee receives even its fixed floats in r0-r3, so a recovered
/// VFP parameter already rules the variadic reading out.
pub fn closed_recovery<'a>(arch: &Architecture, pieces: &'a PrototypePieces) -> Cow<'a, PrototypePieces> {
    if applies(arch) {
        Cow::Owned(PrototypePieces { first_var_arg_slot: -1, ..pieces.clone() })
    } else {
        Cow::Borrowed(pieces)
    }
}

fn add_pairs(list: &mut ParamListStandard, manager: &AddrSpaceManager, limit: usize) {
    let entries: Vec<_> = list
        .get_entry()
        .iter()
        .filter(|e| e.get_type() == type_class::TYPECLASS_FLOAT && e.get_size() == 4)
        .cloned()
        .collect();
    for pair in entries.chunks_exact(2).take(limit) {
        let (lo, hi) = (&pair[0], &pair[1]);
        if lo.get_space().get_index() != hi.get_space().get_index()
            || lo.get_base() + 4 != hi.get_base()
            || lo.get_group() + 1 != hi.get_group()
        {
            continue;
        }
        let addr = Address::new(lo.get_space().clone(), lo.get_base());
        if list
            .get_entry()
            .iter()
            .any(|e| e.justified_contain(&addr, 8) == 0)
        {
            continue;
        }
        if let Ok(mut entry) = ParamEntry::seed(
            lo.get_group(),
            type_class::TYPECLASS_FLOAT,
            lo.get_space().clone(),
            lo.get_base(),
            8,
            5,
            0,
            0,
            true,
            false,
            list.get_entry(),
            manager,
        ) {
            entry.kuna_whole_register(lo.is_first_in_class());
            list.push_entry(entry);
        }
    }
    list.preserve_whole_float_groups();
    list.populate_resolver();
}

pub fn model(model: &Rc<ProtoModel>, manager: &AddrSpaceManager) -> Rc<ProtoModel> {
    if model.is_merged() {
        return model.clone();
    }
    let mut result = (**model).clone();
    add_pairs(result.input_mut(), manager, 8);
    add_pairs(result.output_mut(), manager, 1);
    Rc::new(result)
}

/// Distinguish fresh low words from the unchanged low half of the older double.
fn single_write(
    data: &Funcdata,
    vn: VarnodeId,
    whole: VarnodeId,
    budget: &mut usize,
) -> Option<bool> {
    if *budget == 0 {
        return None;
    }
    *budget -= 1;
    let value = data.vbank().get(vn).filter(|v| v.get_size() == 4)?;
    if value.is_constant() || value.is_input() {
        return Some(true);
    }
    let op = value.get_def().and_then(|id| data.obank().get(id))?;
    match op.code() {
        OpCode::CPUI_COPY | OpCode::CPUI_CAST => single_write(data, op.get_in(0)?, whole, budget),
        OpCode::CPUI_MULTIEQUAL if op.num_input() > 0 => (0..op.num_input())
            .try_fold(false, |written, i| {
                Some(single_write(data, op.get_in(i)?, whole, budget)? | written)
            }),
        OpCode::CPUI_MULTIEQUAL => None,
        OpCode::CPUI_SUBPIECE => Some(
            !(op.get_in(0) == Some(whole)
                && op
                    .get_in(1)
                    .and_then(|v| data.vbank().get(v))
                    .is_some_and(|v| v.is_constant() && v.get_offset() == 0)),
        ),
        _ => Some(true),
    }
}

fn high_source(
    data: &Funcdata,
    vn: VarnodeId,
    addr: &Address,
    budget: &mut usize,
) -> Option<VarnodeId> {
    if *budget == 0 {
        return None;
    }
    *budget -= 1;
    let value = data.vbank().get(vn).filter(|v| v.get_size() == 4)?;
    let op = value.get_def().and_then(|id| data.obank().get(id))?;
    match op.code() {
        OpCode::CPUI_COPY | OpCode::CPUI_CAST => high_source(data, op.get_in(0)?, addr, budget),
        OpCode::CPUI_SUBPIECE => {
            let whole = op.get_in(0)?;
            let value = data.vbank().get(whole)?;
            let offset = data.vbank().get(op.get_in(1)?)?;
            (value.get_size() == 8
                && value.get_addr() == addr
                && offset.is_constant()
                && offset.get_offset() == 4)
                .then_some(whole)
        }
        _ => None,
    }
}

/// Record the d0 values whose high halves survive a later four-byte write.
fn partial_sources(
    data: &Funcdata,
    vn: VarnodeId,
    addr: &Address,
    sources: &mut Vec<VarnodeId>,
    budget: &mut usize,
) -> bool {
    if *budget == 0 {
        return false;
    }
    *budget -= 1;
    let Some(value) = data.vbank().get(vn).filter(|v| v.get_size() == 8) else {
        return false;
    };
    let Some(op) = value.get_def().and_then(|id| data.obank().get(id)) else {
        return true;
    };
    match op.code() {
        OpCode::CPUI_COPY | OpCode::CPUI_CAST => op
            .get_in(0)
            .is_some_and(|v| partial_sources(data, v, addr, sources, budget)),
        OpCode::CPUI_MULTIEQUAL => {
            op.num_input() > 0
                && (0..op.num_input()).all(|i| {
                    op.get_in(i)
                        .is_some_and(|v| partial_sources(data, v, addr, sources, budget))
                })
        }
        OpCode::CPUI_PIECE => {
            let whole = op
                .get_in(0)
                .and_then(|v| high_source(data, v, addr, budget));
            if let Some(whole) = whole {
                if op
                    .get_in(1)
                    .is_some_and(|v| single_write(data, v, whole, budget) == Some(true))
                    && !sources.contains(&whole)
                {
                    sources.push(whole);
                }
            }
            true
        }
        _ => true,
    }
}

fn partial_return(data: &Funcdata, vn: VarnodeId, addr: &Address, budget: &mut usize) -> bool {
    let mut sources = Vec::new();
    partial_sources(data, vn, addr, &mut sources, budget)
        && !sources.is_empty()
        && partial_path(data, vn, addr, &sources, budget)
}

/// Every incoming value must be partial or the older value carried past a predicated write.
fn partial_path(
    data: &Funcdata,
    vn: VarnodeId,
    addr: &Address,
    sources: &[VarnodeId],
    budget: &mut usize,
) -> bool {
    if *budget == 0 {
        return false;
    }
    *budget -= 1;
    let Some(value) = data.vbank().get(vn).filter(|v| v.get_size() == 8) else {
        return false;
    };
    if sources.contains(&vn) {
        return true;
    }
    let Some(op) = value.get_def().and_then(|id| data.obank().get(id)) else {
        return false;
    };
    match op.code() {
        OpCode::CPUI_COPY | OpCode::CPUI_CAST => op
            .get_in(0)
            .is_some_and(|v| partial_path(data, v, addr, sources, budget)),
        OpCode::CPUI_MULTIEQUAL => {
            op.num_input() > 0
                && (0..op.num_input()).all(|i| {
                    op.get_in(i)
                        .is_some_and(|v| partial_path(data, v, addr, sources, budget))
                })
        }
        OpCode::CPUI_PIECE => {
            let whole = op
                .get_in(0)
                .and_then(|v| high_source(data, v, addr, budget));
            whole.is_some_and(|whole| {
                op.get_in(1)
                    .is_some_and(|v| single_write(data, v, whole, budget) == Some(true))
            })
        }
        _ => false,
    }
}

/// The value is written by an op of this function other than a call, and only
/// RETURNs read it: it exists to be returned.
fn computed_result(data: &Funcdata, vn: VarnodeId) -> bool {
    let Some(value) = data.vbank().get(vn) else { return false };
    let Some(op) = value.get_def().and_then(|id| data.obank().get(id)) else {
        return false;
    };
    !matches!(
        op.code(),
        OpCode::CPUI_INDIRECT | OpCode::CPUI_CALL | OpCode::CPUI_CALLIND
    ) && value.descend_iter().all(|id| {
        data.obank()
            .get(id)
            .is_some_and(|o| o.code() == OpCode::CPUI_RETURN)
    })
}

/// The value is what a call left in its register: the call's output or the
/// INDIRECT a call creates.
fn left_by_call(data: &Funcdata, vn: VarnodeId) -> bool {
    data.vbank()
        .get(vn)
        .and_then(|v| v.get_def())
        .and_then(|id| data.obank().get(id))
        .is_some_and(|op| {
            op.is_indirect_creation()
                || matches!(op.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND)
        })
}

/// Retire a VFP return trial that only hands back what a call left in the
/// register when an integer return trial holds a value the function computes
/// to return: `half(x); return k + 1;` leaves `half`'s double in d0 beside
/// `k + 1` in r0, and the wider d0 would otherwise win the fill-in.
fn drop_call_leftovers(data: &Funcdata, active: &mut ParamActive, returns: &[crate::context::OpId]) {
    let entries = data.get_func_proto().model().output().get_entry();
    let vfp = |addr: &Address, size: i32| {
        entries.iter().any(|e| {
            e.get_type() == type_class::TYPECLASS_FLOAT && e.justified_contain(addr, size) >= 0
        })
    };
    let every = |slot: i32, test: &dyn Fn(VarnodeId) -> bool| {
        returns.iter().all(|&id| {
            data.obank()
                .get(id)
                .and_then(|o| o.get_in(slot))
                .is_some_and(test)
        })
    };
    let computed = (0..active.get_num_trials()).any(|i| {
        let t = active.get_trial(i);
        t.is_active()
            && !vfp(t.get_address(), t.get_size())
            && every(t.get_slot(), &|vn| computed_result(data, vn))
    });
    if !computed {
        return;
    }
    for i in 0..active.get_num_trials() {
        let t = active.get_trial(i);
        if t.is_active()
            && vfp(t.get_address(), t.get_size())
            && every(t.get_slot(), &|vn| left_by_call(data, vn))
        {
            active.get_trial_mut(i).mark_inactive();
        }
    }
}

/// Retire VFP return trials a call merely left behind, then preserve the narrow
/// return width when every exit only writes its low word.
pub fn narrow_returns(data: &mut Funcdata, active: &mut ParamActive) {
    if !data.get_arch().arm_float_return || data.get_func_proto().is_output_locked() {
        return;
    }
    let returns: Vec<_> = data
        .obank()
        .iter_code(OpCode::CPUI_RETURN)
        .filter(|&id| {
            data.obank()
                .get(id)
                .is_some_and(|o| !o.is_dead() && o.get_halt_type() == 0)
        })
        .collect();
    if returns.is_empty() {
        return;
    }
    drop_call_leftovers(data, active, &returns);
    let single = data
        .get_func_proto()
        .model()
        .output()
        .get_entry()
        .iter()
        .find(|e| e.get_type() == type_class::TYPECLASS_FLOAT && e.get_size() == 4)
        .map(|e| Address::new(e.get_space().clone(), e.get_base()));
    let Some(single) = single else { return };
    for i in 0..active.get_num_trials() {
        let trial = active.get_trial(i);
        if !trial.is_active()
            || trial.get_size() != 8
            || trial.get_address().justified_contain(8, &single, 4, false) != 0
        {
            continue;
        }
        let addr = trial.get_address().clone();
        let slot = trial.get_slot();
        let proven = returns.iter().all(|&id| {
            data.obank()
                .get(id)
                .and_then(|o| o.get_in(slot))
                .is_some_and(|vn| partial_return(data, vn, &addr, &mut 64))
        });
        if !proven {
            continue;
        }
        for &id in &returns {
            let op = data.obank().get(id).unwrap();
            let vn = op.get_in(slot).unwrap();
            let sub = data.new_op(2, op.get_addr().clone());
            data.op_set_opcode_code(sub, OpCode::CPUI_SUBPIECE);
            let zero = data.new_constant(4, 0);
            let _ = data.op_set_input(sub, vn, 0);
            let _ = data.op_set_input(sub, zero, 1);
            let out = data.new_varnode_out(4, &single, sub).unwrap();
            data.vbank_mut().get_mut(out).unwrap().set_write_mask();
            data.op_insert_before(sub, id);
            let _ = data.op_set_input(id, out, slot);
        }
        active.shrink(i, single.clone(), 4);
    }
}

/// Only a finalized scalar VFP trial supplies a floating-point type.
pub fn type_returns(data: &mut Funcdata, active: &ParamActive) {
    if !data.get_arch().arm_float_return || data.get_func_proto().is_output_locked() {
        return;
    }
    let used: Vec<_> = (0..active.get_num_trials())
        .map(|i| active.get_trial(i))
        .filter(|t| t.is_used())
        .collect();
    if used.len() != 1 {
        return;
    }
    let trial = used[0];
    let size = trial.get_size();
    if !matches!(size, 4 | 8) {
        return;
    }
    let model = data.get_func_proto().model();
    let Some(first) = model
        .output()
        .get_entry()
        .iter()
        .find(|e| e.get_type() == type_class::TYPECLASS_FLOAT)
    else {
        return;
    };
    if trial.get_address().get_space().map(|s| s.get_index()) != Some(first.get_space().get_index())
        || trial.get_address().get_offset() != first.get_base()
    {
        return;
    }
    let Some(types) = data.get_arch().types() else {
        return;
    };
    let Ok(ty) = types.get_base(size, type_metatype::TYPE_FLOAT) else {
        return;
    };
    let returns: Vec<_> = data.obank().iter_code(OpCode::CPUI_RETURN).collect();
    for op in returns {
        let Some(op) = data
            .obank()
            .get(op)
            .filter(|o| !o.is_dead() && o.get_halt_type() == 0 && o.num_input() == 2)
        else {
            continue;
        };
        let Some(vn) = op.get_in(1) else { continue };
        if let Some(value) = data
            .vbank_mut()
            .get_mut(vn)
            .filter(|v| v.get_size() == size)
        {
            value.update_type_locked(ty.clone(), true, false);
        }
    }
}

/// An 8-byte d-register input that no op reads whole: every use takes one
/// 4-byte half, so it holds two floats (s0 and s1), not a double, and must not
/// match the widened d-register entry. Kept when a later VFP input is read, so
/// that parameter keeps its position.
pub fn split_double_input(data: &Funcdata, vn: VarnodeId) -> bool {
    if !data.get_arch().arm_float_return {
        return false;
    }
    let Some(value) = data.vbank().get(vn).filter(|v| v.get_size() == 8) else {
        return false;
    };
    let entries = data.get_func_proto().model().input().get_entry();
    let vfp = |addr: &Address, size: i32| {
        entries.iter().any(|e| {
            e.get_type() == type_class::TYPECLASS_FLOAT && e.justified_contain(addr, size) >= 0
        })
    };
    let addr = value.get_addr();
    if !vfp(addr, 8) || !halves_only(data, vn, 4) {
        return false;
    }
    let end = addr.get_offset() + 8;
    !data
        .vbank()
        .iter_def_flag(crate::varnode::varnode_flags::input)
        .filter_map(|id| data.vbank().get(id))
        .any(|v| {
            !v.has_no_descend()
                && v.get_addr().get_space().map(|s| s.get_index())
                    == addr.get_space().map(|s| s.get_index())
                && v.get_offset() >= end
                && vfp(v.get_addr(), v.get_size())
        })
}

/// Every read of `vn` ends in a 4-byte SUBPIECE, directly or through a CAST,
/// COPY or a right shift by 32 that only such reads consume.
fn halves_only(data: &Funcdata, vn: VarnodeId, depth: u32) -> bool {
    let Some(value) = data.vbank().get(vn) else { return false };
    depth > 0
        && value.descend_iter().all(|id| {
            let Some(op) = data.obank().get(id) else { return false };
            let Some(out) = op.get_out() else { return false };
            let small = data.vbank().get(out).is_some_and(|o| o.get_size() <= 4);
            match op.code() {
                OpCode::CPUI_SUBPIECE => small,
                OpCode::CPUI_CAST | OpCode::CPUI_COPY => halves_only(data, out, depth - 1),
                OpCode::CPUI_INT_RIGHT | OpCode::CPUI_INT_SRIGHT => {
                    op.get_in(0) == Some(vn)
                        && op
                            .get_in(1)
                            .and_then(|c| data.vbank().get(c))
                            .is_some_and(|c| c.is_constant() && c.get_offset() == 32)
                        && halves_only(data, out, depth - 1)
                }
                _ => false,
            }
        })
}

/// The type of an unused parameter that only fills a VFP slot below a used
/// floating-point one: a float of its width, so the printed prototype puts the
/// next parameter in the same register (an integer type would move it to the
/// core registers). Below a VFP parameter recovery typed as an integer, the
/// filler keeps its default type too.
pub fn unused_vfp_type(
    data: &mut Funcdata,
    active: &ParamActive,
    triallist: &[VarnodeId],
    i: i32,
) -> Option<Rc<crate::dtype::Datatype>> {
    let trial = active.get_trial(i);
    if !data.get_arch().arm_float_return || !trial.is_unref() {
        return None;
    }
    let size = trial.get_size();
    let offset = trial.get_address().get_offset();
    let later: Vec<VarnodeId> = {
        let entries = data.get_func_proto().model().input().get_entry();
        let vfp = |addr: &Address, size: i32| {
            entries.iter().any(|e| {
                e.get_type() == type_class::TYPECLASS_FLOAT
                    && e.get_size() == size
                    && e.justified_contain(addr, size) == 0
            })
        };
        if !vfp(trial.get_address(), size) {
            return None;
        }
        (0..active.get_num_trials())
            .map(|j| active.get_trial(j))
            .filter(|t| {
                t.is_used()
                    && !t.is_unref()
                    && t.get_address().get_offset() > offset
                    && vfp(t.get_address(), t.get_size())
            })
            .filter_map(|t| triallist.get((t.get_slot() - 1) as usize).copied())
            .collect()
    };
    let float_after = later.into_iter().any(|vn| {
        data.high_get_type(vn)
            .is_some_and(|ty| ty.get_metatype() == type_metatype::TYPE_FLOAT)
    });
    if !float_after {
        return None;
    }
    data.get_arch().types()?.get_base(size, type_metatype::TYPE_FLOAT).ok()
}

/// A widened call-argument trial needs a callee contract; a live d-register
/// alone does not establish that an unknown call consumes a double.
pub fn call_input_allowed(
    data: &Funcdata,
    call: &crate::fspec::FuncCallSpecs,
    addr: &Address,
    size: i32,
) -> bool {
    if !data.get_arch().arm_float_return
        || size != 8
        || !call.proto().model().input().get_entry().iter().any(|e| {
            e.get_type() == type_class::TYPECLASS_FLOAT && e.justified_contain(addr, size) == 0
        })
    {
        return true;
    }
    if call.is_input_locked() {
        return (0..call.proto().num_params())
            .filter_map(|i| call.proto().get_param(i))
            .any(|p| p.get_address() == *addr && p.get_size() == size);
    }
    data.kuna_protoorder_types(call.get_entry_address())
        .is_some_and(|stated| {
            stated.arity_sound
                && stated.inputs.iter().any(|(a, s, ty)| {
                    *a == *addr && *s == size && ty.get_metatype() == type_metatype::TYPE_FLOAT
                })
        })
}

#[cfg(test)]
mod tests;
