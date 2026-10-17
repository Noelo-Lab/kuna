//! Do not infer an x87 result when every returning path explicitly freed ST0.
use crate::{context::OpId, fspec::ParamActive, funcdata::Funcdata};
use kuna_base::address::Address;
use kuna_num::{opcodes::OpCode, pcoderaw::VarnodeData};
use std::collections::{HashMap, VecDeque};

#[derive(Clone)]
pub struct Storage {
    free: u64,
    top: VarnodeData,
    tags: VarnodeData,
    bank: Vec<VarnodeData>,
}

pub fn storage(arch: &crate::architecture::Architecture) -> Option<Storage> {
    let free = arch.userops.get_op_by_name(b"ffree")?;
    if free.get_inject_id().is_some() {
        return None;
    }
    let top = arch.get_register_varnode(b"ST0").ok()?;
    if top.size != 10 {
        return None;
    }
    Some(Storage {
        free: free.get_index() as u64,
        top,
        tags: arch.get_register_varnode(b"FPUTagWord").ok()?,
        bank: (0..8)
            .map(|i| arch.get_register_varnode(format!("ST{i}").as_bytes()).ok())
            .collect::<Option<Vec<_>>>()?,
    })
}

#[derive(Clone, Copy)]
enum Effect {
    Preserve,
    Empty,
    Unknown,
}

/// The greatest fixed point proves emptiness across loops only after entry paths agree.
fn states(preds: &[Vec<usize>], effects: &[Effect]) -> (Vec<bool>, Vec<bool>) {
    let mut successors = vec![Vec::new(); preds.len()];
    for (to, incoming) in preds.iter().enumerate() {
        for &from in incoming {
            successors[from].push(to);
        }
    }
    let mut reachable = vec![false; preds.len()];
    if preds.is_empty() {
        return (reachable.clone(), reachable);
    }
    let mut pending = VecDeque::from([0]);
    reachable[0] = true;
    while let Some(block) = pending.pop_front() {
        for &next in &successors[block] {
            if !reachable[next] {
                reachable[next] = true;
                pending.push_back(next);
            }
        }
    }
    let transfer = |effect, input| match effect {
        Effect::Preserve => input,
        Effect::Empty => true,
        Effect::Unknown => false,
    };
    let mut output = vec![true; preds.len()];
    loop {
        let mut changed = false;
        for block in 0..preds.len() {
            if !reachable[block] {
                continue;
            }
            let input = block != 0
                && preds[block]
                    .iter()
                    .filter(|&&p| reachable[p])
                    .all(|&p| output[p]);
            let next = transfer(effects[block], input);
            if output[block] != next {
                output[block] = next;
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
    let input = (0..preds.len())
        .map(|block| {
            reachable[block]
                && block != 0
                && preds[block]
                    .iter()
                    .filter(|&&p| reachable[p])
                    .all(|&p| output[p])
        })
        .collect();
    (reachable, input)
}

fn overlaps(data: &Funcdata, id: crate::context::VarnodeId, top: &VarnodeData) -> bool {
    let Some(v) = data.vbank().get(id) else {
        return false;
    };
    v.get_addr()
        .get_space()
        .zip(top.space.as_ref())
        .is_some_and(|(a, b)| a.get_index() == b.get_index())
        && v.get_offset() < top.offset.saturating_add(u64::from(top.size))
        && top.offset < v.get_offset().saturating_add(v.get_size() as u64)
}

fn mmx_access(data: &Funcdata, id: crate::context::VarnodeId, storage: &Storage) -> bool {
    let Some(v) = data.vbank().get(id) else {
        return false;
    };
    storage.bank.iter().any(|reg| {
        overlaps(data, id, reg) && (v.get_size() != reg.size as i32 || v.get_offset() != reg.offset)
    })
}

fn effect(data: &Funcdata, id: OpId, storage: &Storage) -> Effect {
    let Some(op) = data.obank().get(id) else {
        return Effect::Unknown;
    };
    if op.code() == OpCode::CPUI_CALLOTHER {
        let free = op
            .get_in(0)
            .and_then(|v| data.vbank().get(v))
            .is_some_and(|v| v.is_constant() && v.get_offset() == storage.free);
        if free {
            return if op
                .get_in(1)
                .and_then(|v| data.vbank().get(v))
                .is_some_and(|v| {
                    v.get_size() == storage.top.size as i32
                        && v.get_offset() == storage.top.offset
                        && v.get_addr()
                            .get_space()
                            .zip(storage.top.space.as_ref())
                            .is_some_and(|(a, b)| a.get_index() == b.get_index())
                }) {
                Effect::Empty
            } else {
                Effect::Preserve
            };
        }
        return Effect::Unknown;
    }
    if matches!(op.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND)
        || op.get_out().is_some_and(|v| {
            overlaps(data, v, &storage.top)
                || overlaps(data, v, &storage.tags)
                || mmx_access(data, v, storage)
        })
        || (0..op.num_input())
            .filter_map(|i| op.get_in(i))
            .any(|v| mmx_access(data, v, storage))
    {
        Effect::Unknown
    } else {
        Effect::Preserve
    }
}

/// Record raw register identities before SSA and copy propagation erase them.
pub fn capture(data: &mut Funcdata) {
    data.kuna_x87_empty_returns.clear();
    let Some(storage) = data.get_arch().x87_empty_storage.clone() else {
        return;
    };
    let has_free = data.obank().iter_code(OpCode::CPUI_CALLOTHER).any(|id| {
        data.obank()
            .get(id)
            .and_then(|op| op.get_in(0))
            .and_then(|v| data.vbank().get(v))
            .is_some_and(|v| v.is_constant() && v.get_offset() == storage.free)
    });
    if !has_free {
        return;
    }

    let blocks: Vec<_> = (0..data.bblocks_get_size())
        .map(|i| data.bblocks_get_block(i))
        .collect();
    let indices: HashMap<_, _> = blocks.iter().enumerate().map(|(i, &b)| (b, i)).collect();
    let mut preds = Vec::new();
    let mut effects = Vec::new();
    for &block in &blocks {
        let b = data.bblocks_ref().block(block);
        let incoming: Option<Vec<_>> = (0..b.size_in())
            .map(|i| indices.get(&b.get_in(i)).copied())
            .collect();
        let Some(incoming) = incoming else {
            return;
        };
        preds.push(incoming);
        let mut last = Effect::Preserve;
        for op in data.bb_ops(block) {
            let e = effect(data, op, &storage);
            if !matches!(e, Effect::Preserve) {
                last = e;
            }
        }
        effects.push(last);
    }
    let (reachable, input) = states(&preds, &effects);
    let mut returns = std::collections::BTreeMap::<Address, bool>::new();
    for (i, &block) in blocks.iter().enumerate() {
        if !reachable[i] {
            continue;
        }
        let mut empty = input[i];
        for id in data.bb_ops(block) {
            match effect(data, id, &storage) {
                Effect::Empty => empty = true,
                Effect::Unknown => empty = false,
                Effect::Preserve => {}
            }
            if let Some(op) = data.obank().get(id) {
                if op.code() == OpCode::CPUI_RETURN && op.get_halt_type() == 0 {
                    returns
                        .entry(op.get_addr().clone())
                        .and_modify(|all| *all &= empty)
                        .or_insert(empty);
                }
            }
        }
    }
    data.kuna_x87_empty_returns = returns
        .into_iter()
        .filter_map(|(addr, empty)| empty.then_some(addr))
        .collect();
}

pub fn drop_empty(data: &Funcdata, active: &mut ParamActive, returns: &[OpId]) {
    if data.get_func_proto().is_output_locked() || data.kuna_x87_empty_returns.is_empty() {
        return;
    }
    let Some(storage) = data.get_arch().x87_empty_storage.as_ref() else {
        return;
    };
    let ordinary: Vec<_> = returns
        .iter()
        .filter_map(|&id| data.obank().get(id).map(|o| (id, o)))
        .filter(|&(id, o)| {
            !o.is_dead() && o.get_halt_type() == 0 && !crate::kuna_bejoin::never_reached(data, id)
        })
        .collect();
    if ordinary.is_empty()
        || !ordinary
            .iter()
            .all(|(_, o)| data.kuna_x87_empty_returns.contains(o.get_addr()))
    {
        return;
    }
    for i in 0..active.get_num_trials() {
        let trial = active.get_trial(i);
        let addr = Address::new(
            storage.top.space.as_ref().unwrap().clone(),
            storage.top.offset,
        );
        if trial.get_address() == &addr
            && trial.get_size() == storage.top.size as i32
            && !crate::p4_calls::kuna_voidret::forced(data, trial.get_address(), trial.get_size())
        {
            active.get_trial_mut(i).mark_inactive();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn loops_require_an_empty_entry_path() {
        use Effect::*;
        let graph = vec![vec![], vec![0, 1], vec![1]];
        assert_eq!(
            states(&graph, &[Empty, Preserve, Preserve]).1,
            vec![false, true, true]
        );
        assert_eq!(
            states(&graph, &[Preserve, Preserve, Preserve]).1,
            vec![false, false, false]
        );
        assert_eq!(
            states(&graph, &[Empty, Unknown, Preserve]).1,
            vec![false, false, false]
        );
    }
    #[test]
    fn joins_require_all_reachable_predecessors() {
        use Effect::*;
        let graph = vec![vec![], vec![0], vec![0], vec![1, 2, 4], vec![4]];
        let (reachable, input) = states(&graph, &[Preserve, Empty, Empty, Preserve, Unknown]);
        assert_eq!(reachable, vec![true, true, true, true, false]);
        assert!(input[3]);
        assert!(!states(&graph, &[Preserve, Empty, Unknown, Preserve, Unknown]).1[3]);
    }
}
