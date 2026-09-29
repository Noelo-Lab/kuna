//! Recover a forwarded return only when a direct caller consumes it.
use crate::{
    architecture::Architecture,
    context::{OpId, VarnodeId},
    funcdata::Funcdata,
};
use kuna_base::address::Address;
use kuna_num::opcodes::OpCode;
use std::collections::{BTreeMap, BTreeSet, HashSet};

pub type Key = (i32, u64);
pub type Storage = (i32, u64, i32);
#[derive(Clone, Debug, Default)]
pub struct Ledger {
    pub recording: bool,
    pub generation: usize,
    pub demands: BTreeMap<Key, BTreeSet<Storage>>,
    pub calls: BTreeMap<Key, BTreeSet<Key>>,
    pub outputs: BTreeMap<Key, Option<Storage>>,
    pub forwarded_inputs: BTreeMap<Key, BTreeSet<Storage>>,
}
fn key(addr: &Address) -> Option<Key> {
    Some((addr.get_space()?.get_index(), addr.get_offset()))
}
fn storage(addr: &Address, size: i32) -> Option<Storage> {
    let (space, offset) = key(addr)?;
    (size > 0).then_some((space, offset, size))
}
impl Ledger {
    fn demand(&mut self, entry: Key, value: Storage) {
        if self.demands.entry(entry).or_default().insert(value) {
            self.generation += 1;
        }
    }
}

fn consumed(data: &Funcdata, start: VarnodeId) -> bool {
    let mut pending = vec![start];
    let mut seen = HashSet::new();
    while let Some(vn) = pending.pop() {
        if !seen.insert(vn) {
            continue;
        }
        if seen.len() > 128 {
            return false;
        }
        let Some(v) = data.vbank().get(vn) else {
            return false;
        };
        for id in v.descend_iter() {
            let Some(op) = data.obank().get(id).filter(|o| !o.is_dead()) else {
                continue;
            };
            match op.code() {
                OpCode::CPUI_COPY
                | OpCode::CPUI_CAST
                | OpCode::CPUI_MULTIEQUAL
                | OpCode::CPUI_SUBPIECE
                | OpCode::CPUI_PIECE
                | OpCode::CPUI_INT_ZEXT
                | OpCode::CPUI_INT_SEXT => {
                    if let Some(out) = op.get_out() {
                        pending.push(out);
                    }
                }
                OpCode::CPUI_INDIRECT
                | OpCode::CPUI_RETURN
                | OpCode::CPUI_CALL
                | OpCode::CPUI_CALLIND
                | OpCode::CPUI_CALLOTHER => {}
                _ => return true,
            }
        }
    }
    false
}

/// Record actual uses after recovery; artificial return trials are not callers.
pub fn record(arch: &mut Architecture, data: &Funcdata) {
    if !arch.wrapper_return
        || !arch.archid.starts_with("ARM:")
        || !arch.kuna_wrapperreturn.borrow().recording
    {
        return;
    }
    let Some(entry) = key(data.get_address()) else {
        return;
    };
    let mut demands = Vec::new();
    let mut calls = BTreeSet::new();
    for i in 0..data.num_calls() {
        let fc = data.get_call_specs(i);
        let Some(op) = data
            .obank()
            .get(fc.get_op())
            .filter(|o| !o.is_dead() && o.code() == OpCode::CPUI_CALL)
        else {
            continue;
        };
        if let Some(target) = key(fc.get_entry_address()) {
            calls.insert(target);
        }
        let Some(out) = op.get_out() else { continue };
        if !consumed(data, out) {
            continue;
        }
        let Some(ret) = data.vbank().get(out) else {
            continue;
        };
        if !crate::kuna_calleearitybody::is_register(ret.get_addr()) {
            continue;
        }
        let Some(target) = key(fc.get_entry_address()) else {
            continue;
        };
        let Some(value) = storage(ret.get_addr(), ret.get_size()) else {
            continue;
        };
        demands.push((target, value));
    }
    let out = data.get_func_proto().get_output();
    let output = storage(&out.get_address(), out.get_size());
    let mut forwarded: BTreeSet<_> = data
        .kuna_passthrough_claims()
        .iter()
        .filter(|c| !c.arg_owners.is_empty())
        .filter_map(|c| storage(&c.addr, c.size))
        .collect();
    for i in 0..data.num_calls() {
        let fc = data.get_call_specs(i);
        if !fc.is_input_locked() {
            continue;
        }
        let Some(op) = data
            .obank()
            .get(fc.get_op())
            .filter(|o| !o.is_dead() && o.code() == OpCode::CPUI_CALL)
        else {
            continue;
        };
        for slot in 1..op.num_input().min(fc.proto().num_params() + 1) {
            if let Some(value) = op.get_in(slot).and_then(|v| forwarded_origin(data, v)) {
                forwarded.insert(value);
            }
        }
    }
    let mut ledger = arch.kuna_wrapperreturn.borrow_mut();
    if !ledger.recording {
        return;
    }
    if ledger.outputs.get(&entry) != Some(&output) {
        ledger.generation += 1;
    }
    ledger.calls.insert(entry, calls);
    ledger.outputs.insert(entry, output);
    ledger.forwarded_inputs.insert(entry, forwarded);
    for (target, value) in demands {
        ledger.demand(target, value);
    }
}

pub fn forwarded_input(data: &Funcdata, entry: &Address, addr: &Address, size: i32) -> bool {
    if !data.get_arch().wrapper_return {
        return false;
    }
    let (Some(entry), Some(value)) = (key(entry), storage(addr, size)) else {
        return false;
    };
    data.get_arch()
        .kuna_wrapperreturn
        .borrow()
        .forwarded_inputs
        .get(&entry)
        .is_some_and(|set| set.contains(&value))
}

/// Walk each exit back to its producing call, refusing writes or uncertain joins.
fn preserved_call(data: &Funcdata, ret: OpId, addr: &Address, size: i32) -> Option<OpId> {
    let mut current = data.op_previous_op(ret);
    let mut block = data.obank().get(ret)?.get_parent()?;
    let mut visited = HashSet::new();
    let mut budget = 256;
    for _ in 0..16 {
        if !visited.insert(block) {
            return None;
        }
        while let Some(id) = current {
            if budget == 0 {
                return None;
            }
            budget -= 1;
            let op = data.obank().get(id)?;
            match op.code() {
                OpCode::CPUI_CALL => return Some(id),
                OpCode::CPUI_CALLIND | OpCode::CPUI_CALLOTHER => return None,
                _ => {}
            }
            if let Some(out) = op.get_out().and_then(|v| data.vbank().get(v)) {
                let identity = op.code() == OpCode::CPUI_COPY
                    && op
                        .get_in(0)
                        .and_then(|id| data.vbank().get(id))
                        .is_some_and(|v| {
                            v.get_addr() == out.get_addr() && v.get_size() == out.get_size()
                        });
                if !identity
                    && out.get_addr().get_space().map(|s| s.get_index())
                        == addr.get_space().map(|s| s.get_index())
                    && out.get_offset() < addr.get_offset().checked_add(size as u64)?
                    && addr.get_offset() < out.get_offset().checked_add(out.get_size() as u64)?
                {
                    return None;
                }
            }
            current = data.op_previous_op(id);
        }
        let b = data.bblocks_ref().block(block);
        if b.size_in() != 1 {
            return None;
        }
        block = b.get_in(0);
        current = data.bb_op_tail(block);
    }
    None
}

/// Forward a demanded register only once every producer has a proven output.
pub fn claim(data: &Funcdata) -> Option<(Address, i32, Vec<OpId>)> {
    if !data.get_arch().wrapper_return
        || data.get_func_proto().is_output_locked()
        || data.get_active_output().is_none()
    {
        return None;
    }
    let entry = key(data.get_address())?;
    let demand = {
        let ledger = data.get_arch().kuna_wrapperreturn.borrow();
        let demands = ledger.demands.get(&entry)?;
        if demands.len() != 1 {
            return None;
        }
        *demands.iter().next()?
    };
    let (space, offset, size) = demand;
    let manager = data.get_arch().manage();
    if space < 0 || space >= manager.num_spaces() {
        return None;
    }
    let addr = Address::new(manager.get_space(space)?.clone(), offset);
    if !crate::kuna_calleearitybody::is_register(&addr)
        || data.get_func_proto().characterize_as_output(&addr, size)
            == crate::fspec::Containment::NoContainment
    {
        return None;
    }
    let rets: Vec<_> = data
        .obank()
        .iter_code(OpCode::CPUI_RETURN)
        .filter(|&id| {
            data.obank()
                .get(id)
                .is_some_and(|o| !o.is_dead() && o.get_halt_type() == 0)
        })
        .collect();
    if rets.is_empty() {
        return None;
    }
    let mut producers = Vec::new();
    let mut unknown = Vec::new();
    let mut return_type = None;
    for ret in rets {
        let call = preserved_call(data, ret, &addr, size)?;
        let fc = (0..data.num_calls())
            .map(|i| data.get_call_specs(i))
            .find(|fc| fc.get_op() == call)?;
        let declared;
        let output = if fc.proto().is_output_locked() {
            let out = fc.proto().get_output();
            let ty = out.get_type()?;
            if ty.get_metatype() == crate::dtype::type_metatype::TYPE_VOID {
                return None;
            }
            declared = (out.get_address(), out.get_size(), ty.clone());
            Some(&declared)
        } else {
            data.kuna_protoorder_types(fc.get_entry_address())
                .and_then(|s| s.output.as_ref())
        };
        match output {
            Some((a, s, ty)) if *a == addr && *s == size => {
                let signature = (ty.get_metatype(), ty.get_id());
                if return_type.as_ref().is_some_and(|old| *old != signature) {
                    return None;
                }
                return_type = Some(signature);
            }
            Some(_) => return None,
            None => unknown.push(key(fc.get_entry_address())?),
        }
        if !producers.contains(&call) {
            producers.push(call);
        }
    }
    if !unknown.is_empty() {
        let mut ledger = data.get_arch().kuna_wrapperreturn.borrow_mut();
        for target in unknown {
            ledger.demand(target, demand);
        }
        return None;
    }
    Some((addr, size, producers))
}

/// A forwarding claim was made from caller evidence, not from a call clobber.
pub fn proven_output(data: &Funcdata) -> bool {
    if !data.get_arch().wrapper_return {
        return false;
    }
    let out = data.get_func_proto().get_output();
    let Some(value) = storage(&out.get_address(), out.get_size()) else {
        return false;
    };
    if !caller_demands(data, &out.get_address(), out.get_size()) {
        return false;
    }
    data.kuna_passthrough_claims()
        .iter()
        .any(|c| !c.ret_owners.is_empty() && storage(&c.addr, c.size) == Some(value))
}

/// An argument can be touched after a call while still arriving unchanged at it.
/// Refuse loops, earlier calls, and writes on any incoming path.
pub fn input_reaches_call(
    data: &Funcdata,
    call: crate::context::OpId,
    addr: &Address,
    size: i32,
) -> bool {
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

/// An actual argument to a declared callee may forward the wrapper's input.
fn forwarded_origin(data: &Funcdata, mut vn: VarnodeId) -> Option<Storage> {
    for _ in 0..16 {
        let value = data.vbank().get(vn)?;
        if value.is_input() && crate::kuna_calleearitybody::is_register(value.get_addr()) {
            return storage(value.get_addr(), value.get_size());
        }
        let op = data.obank().get(value.get_def()?)?;
        if !matches!(op.code(), OpCode::CPUI_COPY | OpCode::CPUI_CAST) {
            return None;
        }
        let input = op.get_in(0)?;
        if data.vbank().get(input)?.get_size() != value.get_size() {
            return None;
        }
        vn = input;
    }
    None
}

/// Trace exact bytes through identity copies and a split register's PIECE.
fn returned_by(
    data: &Funcdata,
    vn: VarnodeId,
    addr: &Address,
    size: i32,
    budget: &mut usize,
) -> Option<OpId> {
    if *budget == 0 {
        return None;
    }
    *budget -= 1;
    let value = data.vbank().get(vn)?;
    if value.get_size() != size {
        return None;
    }
    let id = value.get_def()?;
    let op = data.obank().get(id)?;
    match op.code() {
        OpCode::CPUI_COPY | OpCode::CPUI_CAST => {
            returned_by(data, op.get_in(0)?, addr, size, budget)
        }
        OpCode::CPUI_PIECE => {
            let hi = op.get_in(0)?;
            let lo = op.get_in(1)?;
            let hs = data.vbank().get(hi)?.get_size();
            let ls = data.vbank().get(lo)?.get_size();
            if hs <= 0 || ls <= 0 || hs.checked_add(ls)? != size {
                return None;
            }
            let space = addr.get_space()?.clone();
            let (ho, loff) = if space.is_big_endian() {
                (0, hs)
            } else {
                (ls, 0)
            };
            let ha = Address::new(space.clone(), addr.get_offset().checked_add(ho as u64)?);
            let la = Address::new(space, addr.get_offset().checked_add(loff as u64)?);
            let owner = returned_by(data, hi, &ha, hs, budget)?;
            (returned_by(data, lo, &la, ls, budget)? == owner).then_some(owner)
        }
        OpCode::CPUI_INDIRECT if op.is_indirect_creation() && value.get_addr() == addr => {
            let iop = data.vbank().get(op.get_in(1)?)?;
            Some(OpId::from(slotmap::KeyData::from_ffi(iop.get_offset())))
        }
        OpCode::CPUI_CALL if value.get_addr() == addr => Some(id),
        _ => None,
    }
}

pub fn returns_claimed_result(data: &Funcdata, vn: VarnodeId, addr: &Address, size: i32) -> bool {
    let owner = returned_by(data, vn, addr, size, &mut 64);
    let Some(owner) = owner else { return false };
    data.kuna_passthrough_claims()
        .iter()
        .any(|c| c.addr == *addr && c.size == size && c.ret_owners.contains(&owner))
}

/// Only an unambiguous consuming caller can override the call-clobber heuristic.
pub fn caller_demands(data: &Funcdata, addr: &Address, size: i32) -> bool {
    if !data.get_arch().wrapper_return {
        return false;
    }
    let (Some(entry), Some(value)) = (key(data.get_address()), storage(addr, size)) else {
        return false;
    };
    data.get_arch()
        .kuna_wrapperreturn
        .borrow()
        .demands
        .get(&entry)
        .is_some_and(|set| set.len() == 1 && set.contains(&value))
}

#[cfg(test)]
mod tests;
