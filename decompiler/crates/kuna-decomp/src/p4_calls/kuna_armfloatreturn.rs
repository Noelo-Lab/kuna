//! Preserve full VFP parameter/return storage when the ARM container states that ABI.
use crate::{
    context::VarnodeId,
    dtype::{type_class, type_metatype},
    fspec::{ParamActive, ParamEntry, ParamListStandard, ProtoModel},
    funcdata::Funcdata,
};
use kuna_base::{address::Address, space::AddrSpaceManager};
use kuna_num::opcodes::OpCode;
use std::rc::Rc;

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
        if let Ok(entry) = ParamEntry::seed(
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

/// Preserve the narrow return width when every exit only writes its low word.
pub fn narrow_returns(data: &mut Funcdata, active: &mut ParamActive) {
    if !data.get_arch().arm_float_return || data.get_func_proto().is_output_locked() {
        return;
    }
    let single = data
        .get_func_proto()
        .model()
        .output()
        .get_entry()
        .iter()
        .find(|e| e.get_type() == type_class::TYPECLASS_FLOAT && e.get_size() == 4)
        .map(|e| Address::new(e.get_space().clone(), e.get_base()));
    let Some(single) = single else { return };
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

/// An argument can be touched after a call while still arriving unchanged at it.
/// Refuse loops, earlier calls, and writes on any incoming path.
pub fn input_reaches_call(
    data: &Funcdata,
    call: crate::context::OpId,
    addr: &Address,
    size: i32,
) -> bool {
    if !data
        .get_func_proto()
        .model()
        .input()
        .get_entry()
        .iter()
        .any(|e| {
            e.get_type() == type_class::TYPECLASS_FLOAT && e.justified_contain(addr, size) == 0
        })
    {
        return false;
    }
    let Some(block) = data.obank().get(call).and_then(|o| o.get_parent()) else {
        return false;
    };
    let mut pending = vec![(block, data.op_previous_op(call))];
    let mut seen = std::collections::HashSet::new();
    let mut budget = 256;
    while let Some((block, mut current)) = pending.pop() {
        if !seen.insert(block) || seen.len() > 16 {
            return false;
        }
        while let Some(id) = current {
            if budget == 0 {
                return false;
            }
            budget -= 1;
            let Some(op) = data.obank().get(id) else {
                return false;
            };
            if matches!(
                op.code(),
                OpCode::CPUI_CALL | OpCode::CPUI_CALLIND | OpCode::CPUI_CALLOTHER
            ) {
                return false;
            }
            if let Some(out) = op.get_out().and_then(|v| data.vbank().get(v)) {
                if out.get_addr().get_space().map(|s| s.get_index())
                    == addr.get_space().map(|s| s.get_index())
                    && out.get_offset() < addr.get_offset().saturating_add(size as u64)
                    && addr.get_offset() < out.get_offset().saturating_add(out.get_size() as u64)
                {
                    return false;
                }
            }
            current = data.op_previous_op(id);
        }
        let incoming = data.bblocks_ref().block(block);
        for i in 0..incoming.size_in() {
            let parent = incoming.get_in(i);
            pending.push((parent, data.bb_op_tail(parent)));
        }
    }
    true
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
