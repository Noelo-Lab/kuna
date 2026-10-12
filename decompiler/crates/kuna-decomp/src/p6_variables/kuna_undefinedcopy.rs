//! Omit trim copies of call-clobbered values that cannot reach an observable use.

use std::collections::{BTreeMap, BTreeSet};

use crate::context::{BlockId, OpId, VarnodeId};
use crate::funcdata::Funcdata;
use kuna_num::opcodes::OpCode;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct State {
    block: BlockId,
    slot: i32,
    start: usize,
    tainted: BTreeSet<VarnodeId>,
    truth: BTreeMap<VarnodeId, bool>,
}

pub(crate) fn mark(data: &mut Funcdata) {
    let mut copies: BTreeMap<VarnodeId, Vec<OpId>> = BTreeMap::new();
    for id in data.obank().iter_alive() {
        let op = data.obank().get(id).unwrap();
        if op.code() != OpCode::CPUI_COPY || op.not_printed() {
            continue;
        }
        let (Some(input), Some(output)) = (op.get_in(0), op.get_out()) else {
            continue;
        };
        if !data
            .vbank()
            .get(output)
            .map(|v| v.get_space())
            .is_some_and(|s| s.get_type() == kuna_base::space::spacetype::IPTR_INTERNAL)
        {
            continue;
        }
        let Some(def) = data.vbank().get(input).and_then(|v| v.get_def()) else {
            continue;
        };
        let def = data.obank().get(def).unwrap();
        if !def.is_indirect_creation() || exposed(data, output) {
            continue;
        }
        let Some(zero) = def.get_in(0).and_then(|v| data.vbank().get(v)) else {
            continue;
        };
        if zero.get_flags() & crate::varnode::varnode_flags::indirect_creation == 0 {
            continue;
        }
        let Some(effect) = def.get_in(1).and_then(|v| data.vbank().get(v)) else {
            continue;
        };
        let effect = OpId::from(slotmap::KeyData::from_ffi(effect.get_offset()));
        if data.obank().get(effect).is_some_and(|o| o.is_call()) {
            copies.entry(input).or_default().push(id);
        }
    }
    let mut budget = 16384usize;
    for (root, ops) in copies {
        if unobserved(data, root, &mut budget) {
            for op in ops {
                data.op_mark_non_printing_pub(op);
            }
        }
    }
}

fn exposed(data: &Funcdata, id: VarnodeId) -> bool {
    let Some(v) = data.vbank().get(id) else {
        return true;
    };
    if v.is_addr_tied() || v.is_persist() {
        return true;
    }
    let Some(high) = v.get_high().and_then(|h| data.high_bank().get(h)) else {
        return true;
    };
    (0..high.num_instances()).any(|i| {
        data.vbank()
            .get(high.get_instance(i))
            .is_none_or(|v| v.is_addr_tied() || v.is_persist())
    })
}

fn transfers(code: OpCode) -> bool {
    matches!(
        code,
        OpCode::CPUI_COPY | OpCode::CPUI_CAST | OpCode::CPUI_MULTIEQUAL | OpCode::CPUI_INDIRECT
    )
}

fn relevant_blocks(
    data: &Funcdata,
    root: VarnodeId,
    budget: &mut usize,
) -> Option<BTreeSet<BlockId>> {
    let mut values = BTreeSet::new();
    let mut work = vec![root];
    let mut blocks = BTreeSet::new();
    while let Some(vn) = work.pop() {
        *budget = budget.checked_sub(1)?;
        if !values.insert(vn) {
            continue;
        }
        for id in data.vbank().get(vn).unwrap().descend_iter() {
            let op = data.obank().get(id).unwrap();
            if transfers(op.code()) && !op.is_indirect_creation() {
                if let Some(output) = op.get_out() {
                    work.push(output);
                }
            } else if let Some(block) = op.get_parent() {
                blocks.insert(block);
            }
        }
    }
    let mut work: Vec<_> = blocks.iter().copied().collect();
    while let Some(block) = work.pop() {
        *budget = budget.checked_sub(1)?;
        let b = data.bblocks_ref().block(block);
        for slot in 0..b.size_in() {
            let prev = b.get_in(slot);
            if blocks.insert(prev) {
                work.push(prev);
            }
        }
    }
    Some(blocks)
}

/// Interpret zero/nonzero facts along CFG paths, selecting phi inputs on their
/// incoming edges. Revisited states close loops; exhausted budgets retain copies.
fn unobserved(data: &Funcdata, root: VarnodeId, budget: &mut usize) -> bool {
    let def = data.vbank().get(root).unwrap().get_def().unwrap();
    let block = data.obank().get(def).unwrap().get_parent().unwrap();
    let start = data.bb_ops(block).iter().position(|&id| id == def).unwrap();
    let Some(relevant) = relevant_blocks(data, root, budget) else {
        return false;
    };
    let mut work = vec![State {
        block,
        slot: -1,
        start,
        tainted: BTreeSet::new(),
        truth: BTreeMap::new(),
    }];
    let mut seen = BTreeSet::new();
    while let Some(mut state) = work.pop() {
        if !relevant.contains(&state.block) || !seen.insert(state.clone()) {
            continue;
        }
        if seen.len() > 1024 {
            return false;
        }
        let incoming = state.clone();
        for id in data.bb_ops(state.block).into_iter().skip(state.start) {
            let Some(left) = budget.checked_sub(1) else {
                return false;
            };
            *budget = left;
            let op = data.obank().get(id).unwrap();
            let phi = op.code() == OpCode::CPUI_MULTIEQUAL;
            let inputs: Vec<_> = if phi {
                let Some(input) = op.get_in(state.slot) else {
                    return false;
                };
                vec![input]
            } else if op.code() == OpCode::CPUI_INDIRECT {
                op.get_in(0).into_iter().collect()
            } else {
                (0..op.num_input()).filter_map(|i| op.get_in(i)).collect()
            };
            let from = if phi { &incoming } else { &state };
            let tainted =
                !op.is_indirect_creation() && inputs.iter().any(|v| from.tainted.contains(v));
            if tainted && !transfers(op.code()) {
                return false;
            }
            if let Some(output) = op.get_out() {
                let tainted = output == root || tainted;
                if tainted && exposed(data, output) {
                    return false;
                }
                let value = if output == root {
                    None
                } else {
                    evaluate(data, op.code(), &inputs, from)
                };
                state.tainted.remove(&output);
                state.truth.remove(&output);
                if tainted {
                    state.tainted.insert(output);
                }
                if let Some(value) = value {
                    state.truth.insert(output, value);
                }
            }
        }
        let b = data.bblocks_ref().block(state.block);
        let condition = data
            .bb_op_tail(state.block)
            .and_then(|id| data.obank().get(id))
            .filter(|o| o.code() == OpCode::CPUI_CBRANCH)
            .and_then(|o| o.get_in(1));
        let flip = data
            .bb_op_tail(state.block)
            .and_then(|id| data.obank().get(id))
            .is_some_and(|o| o.is_boolean_flip());
        if b.size_out() > 2
            || (b.size_out() == 2 && condition.is_none())
            || (condition.is_some() && b.size_out() != 2)
        {
            return false;
        }
        let known = condition.and_then(|v| truth(data, v, &state));
        for slot in 0..b.size_out() {
            let taken = (slot == 1) != flip;
            if condition.is_some() && known.is_some_and(|v| v != taken) {
                continue;
            }
            let mut next = state.clone();
            next.block = b.get_out(slot);
            next.slot = b.get_out_rev_index(slot);
            next.start = 0;
            if let Some(condition) = condition {
                next.truth.insert(condition, taken);
            }
            forget_redefined(data, &mut next);
            work.push(next);
        }
    }
    true
}

/// A dominated definition executes again before its next use. Preserve the
/// selected phi inputs, which still belong to the preceding loop iteration.
fn forget_redefined(data: &Funcdata, state: &mut State) {
    let inputs: BTreeSet<_> = data
        .bb_ops(state.block)
        .into_iter()
        .filter(|&id| data.obank().get(id).unwrap().code() == OpCode::CPUI_MULTIEQUAL)
        .filter_map(|id| data.obank().get(id).unwrap().get_in(state.slot))
        .collect();
    let keep = |vn: &VarnodeId| {
        inputs.contains(vn)
            || !data
                .vbank()
                .get(*vn)
                .and_then(|v| v.get_def())
                .and_then(|id| data.obank().get(id))
                .and_then(|o| o.get_parent())
                .is_some_and(|bl| data.bblocks_ref().dominates(state.block, Some(bl)))
    };
    state.truth.retain(|vn, _| keep(vn));
    state.tainted.retain(keep);
}

fn truth(data: &Funcdata, id: VarnodeId, state: &State) -> Option<bool> {
    let v = data.vbank().get(id)?;
    if v.is_constant() {
        Some(v.get_offset() != 0)
    } else {
        state.truth.get(&id).copied()
    }
}

fn evaluate(data: &Funcdata, code: OpCode, inputs: &[VarnodeId], state: &State) -> Option<bool> {
    let a = inputs.first().copied()?;
    let av = truth(data, a, state);
    match code {
        OpCode::CPUI_COPY
        | OpCode::CPUI_MULTIEQUAL
        | OpCode::CPUI_INT_ZEXT
        | OpCode::CPUI_INT_SEXT => av,
        OpCode::CPUI_BOOL_NEGATE => av.map(|v| !v),
        OpCode::CPUI_BOOL_AND => {
            let bv = truth(data, *inputs.get(1)?, state);
            if av == Some(false) || bv == Some(false) {
                Some(false)
            } else {
                av.zip(bv).map(|(a, b)| a && b)
            }
        }
        OpCode::CPUI_BOOL_OR => {
            let bv = truth(data, *inputs.get(1)?, state);
            if av == Some(true) || bv == Some(true) {
                Some(true)
            } else {
                av.zip(bv).map(|(a, b)| a || b)
            }
        }
        OpCode::CPUI_BOOL_XOR => av
            .zip(truth(data, *inputs.get(1)?, state))
            .map(|(a, b)| a != b),
        OpCode::CPUI_INT_EQUAL | OpCode::CPUI_INT_NOTEQUAL => {
            let b = *inputs.get(1)?;
            let zero = |v| {
                data.vbank()
                    .get(v)
                    .is_some_and(|v| v.is_constant() && v.get_offset() == 0)
            };
            let nonzero = if zero(a) {
                truth(data, b, state)
            } else if zero(b) {
                av
            } else {
                None
            };
            nonzero.map(|v| {
                if code == OpCode::CPUI_INT_EQUAL {
                    !v
                } else {
                    v
                }
            })
        }
        _ => None,
    }
}
