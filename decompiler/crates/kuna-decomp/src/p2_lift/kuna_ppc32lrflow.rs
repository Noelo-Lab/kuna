//! Classify PPC32 LR transfers before prototype recovery discards RETURN's target.
//!
//! SLEIGH spells `blr` (including conditional forms) as RETURN even when LR was
//! assigned a local code address. A bounded raw-p-code fixed point
//! distinguishes those values from the incoming LR and unproved memory reads.
//! Proven computed sites are re-followed as BRANCHIND by the normal jump-table
//! machinery. Saved-LR returns and explicit flow assertions retain their meaning.

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use kuna_base::address::Address;
use kuna_base::error::{KunaError, KunaResult};
use kuna_num::opcodes::OpCode;

use crate::{
    architecture::Architecture,
    context::{BlockId, OpId, VarnodeId},
    funcdata::Funcdata,
};

type Key = (i32, u64, i32);

/// PPC32 branches discard LR's low two bits, including with eight-byte GPRs.
/// Insert before instruction bookkeeping so its first op and CFG remain ordered.
pub(crate) fn align_target(
    data: &mut Funcdata,
    arch: &Architecture,
    site: &Address,
    kind: u32,
) -> KunaResult<()> {
    if kind != crate::overrides::flow_type::BRANCH
        || !arch.archid.starts_with("PowerPC:")
        || !arch.archid.contains(":32:")
    {
        return Ok(());
    }
    let Some(lr) = register(arch, "LR") else {
        return Ok(());
    };
    let returns: Vec<_> = data
        .obank()
        .iter_at(site)
        .map(|(_, op)| op)
        .filter(|&op| {
            data.obank().get(op).is_some_and(|o| {
                o.code() == OpCode::CPUI_RETURN && o.get_in(0).is_some_and(|v| key(data, v) == lr)
            })
        })
        .collect();
    for ret in returns {
        let input = data.obank().get(ret).unwrap().get_in(0).unwrap();
        let mask = data.new_op(2, site.clone());
        data.op_set_opcode_code(mask, OpCode::CPUI_INT_AND);
        let output = data.new_unique_out(lr.2, mask)?;
        data.op_set_input(ret, output, 0)?;
        data.op_set_input(mask, input, 0)?;
        let bits = data.new_constant(lr.2, 0xffff_fffc);
        data.op_set_input(mask, bits, 1)?;
        data.obank_mut().insert_before_dead(mask, ret);
    }
    Ok(())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Value {
    Constant(u64),
    Stack(u32),
    ReturnAddress,
    Computed,
}

#[derive(Clone, Default, PartialEq, Eq)]
struct State {
    values: BTreeMap<Key, Value>,
    stack: BTreeMap<(u32, i32), Value>,
    image_unmodified: bool,
    return_address_observed: bool,
}

fn key(data: &Funcdata, id: VarnodeId) -> Key {
    let v = data.vbank().get(id).expect("raw flow varnode");
    (v.get_space().get_index(), v.get_offset(), v.get_size())
}

fn register(arch: &Architecture, name: &str) -> Option<Key> {
    let r = arch.translate().get_register_data(name).ok()?;
    Some((r.space?.get_index(), r.offset, r.size as i32))
}

fn value(data: &Funcdata, id: VarnodeId, state: &State) -> Option<Value> {
    let v = data.vbank().get(id)?;
    if v.is_constant() {
        Some(Value::Constant(v.get_offset()))
    } else {
        let wanted = key(data, id);
        if let Some(value) = state.values.get(&wanted) {
            return Some(*value);
        }
        let (&(space, offset, size), stored) = state
            .values
            .range(..=(wanted.0, wanted.1, i32::MAX))
            .next_back()?;
        if space != wanted.0
            || size > 8
            || wanted.2 > size
            || wanted.1.checked_add(wanted.2 as u64)? > offset.checked_add(size as u64)?
        {
            return None;
        }
        let displacement = (wanted.1 - offset) as i32;
        let shift = if data.get_address().get_space()?.is_big_endian() {
            size - wanted.2 - displacement
        } else {
            displacement
        } * 8;
        match stored {
            Value::Constant(c) => Some(Value::Constant(
                (c >> shift) & (u64::MAX >> (64 - wanted.2 * 8)),
            )),
            Value::Computed => Some(Value::Computed),
            Value::Stack(offset) if shift == 0 && wanted.2 >= 4 => Some(Value::Stack(*offset)),
            Value::ReturnAddress if shift == 0 && wanted.2 >= 4 => Some(Value::ReturnAddress),
            _ => None,
        }
    }
}

fn write(state: &mut State, key: Key, value: Option<Value>) {
    state.values.retain(|&(space, off, size), _| {
        space != key.0
            || off.saturating_add(size as u64) <= key.1
            || key.1.saturating_add(key.2 as u64) <= off
    });
    if let Some(value) = value {
        state.values.insert(key, value);
    }
}

fn pure(data: &Funcdata, op: OpId, state: &State) -> Option<Value> {
    let o = data.obank().get(op)?;
    let out = data.vbank().get(o.get_out()?)?;
    let inputs: Option<Vec<_>> = (0..o.num_input())
        .map(|i| value(data, o.get_in(i)?, state))
        .collect();
    let inputs = inputs?;
    if o.code() == OpCode::CPUI_COPY {
        return inputs.first().copied();
    }
    if o.code() == OpCode::CPUI_INT_ZEXT
        && out.get_size() == 8
        && data.vbank().get(o.get_in(0)?)?.get_size() == 4
        && matches!(inputs.first(), Some(Value::Stack(_) | Value::ReturnAddress))
    {
        return inputs.first().copied();
    }
    if matches!(o.code(), OpCode::CPUI_INT_OR | OpCode::CPUI_INT_AND)
        && inputs.len() == 2
        && o.get_in(0).map(|v| key(data, v)) == o.get_in(1).map(|v| key(data, v))
    {
        return inputs.first().copied();
    }
    if matches!(
        inputs.as_slice(),
        [Value::ReturnAddress, Value::Constant(0xffff_fffc)]
    ) && o.code() == OpCode::CPUI_INT_AND
    {
        return Some(Value::ReturnAddress);
    }
    if let [base, Value::Constant(c)] = inputs.as_slice() {
        if *c == 0
            && matches!(
                o.code(),
                OpCode::CPUI_INT_ADD | OpCode::CPUI_INT_SUB | OpCode::CPUI_INT_OR
            )
        {
            return Some(*base);
        }
        if let Value::Stack(offset) = base {
            if matches!(out.get_size(), 4 | 8) {
                match o.code() {
                    OpCode::CPUI_INT_ADD => {
                        return Some(Value::Stack(offset.wrapping_add(*c as u32)))
                    }
                    OpCode::CPUI_INT_SUB => {
                        return Some(Value::Stack(offset.wrapping_sub(*c as u32)))
                    }
                    _ => {}
                }
            }
        }
    }
    if inputs
        .iter()
        .any(|v| matches!(v, Value::Stack(_) | Value::ReturnAddress))
    {
        return None;
    }
    if inputs.contains(&Value::Computed) {
        return Some(Value::Computed);
    }
    let constants: Vec<_> = inputs
        .into_iter()
        .filter_map(|v| {
            if let Value::Constant(c) = v {
                Some(c)
            } else {
                None
            }
        })
        .collect();
    let size = data.vbank().get(o.get_in(0)?)?.get_size();
    let behavior = data.get_arch().op_behavior(o.code())?;
    let c = match constants.as_slice() {
        [a] => behavior.evaluate_unary(out.get_size(), size, *a).ok(),
        [a, b] => behavior.evaluate_binary(out.get_size(), size, *a, *b).ok(),
        [a, b, c] => behavior
            .evaluate_ternary(out.get_size(), size, *a, *b, *c)
            .ok(),
        _ => None,
    }?;
    Some(Value::Constant(c))
}

fn stack_address(data: &Funcdata, op: OpId, state: &State) -> Option<u32> {
    let o = data.obank().get(op)?;
    let space = value(data, o.get_in(0)?, state)?;
    if space != Value::Constant(data.get_address().get_space()?.get_index() as u64) {
        return None;
    }
    match value(data, o.get_in(1)?, state)? {
        Value::Stack(offset) => Some(offset),
        _ => None,
    }
}

fn image_value(data: &Funcdata, op: OpId, state: &State) -> Option<Value> {
    if !state.image_unmodified {
        return None;
    }
    let o = data.obank().get(op)?;
    let space = data.get_address().get_space()?;
    if value(data, o.get_in(0)?, state)? != Value::Constant(space.get_index() as u64) {
        return None;
    }
    let Value::Constant(offset) = value(data, o.get_in(1)?, state)? else {
        return None;
    };
    let size = key(data, o.get_out()?).2;
    if !(1..=8).contains(&size) || offset.checked_add(size as u64 - 1)? > u32::MAX as u64 {
        return None;
    }
    let usepoint = Address::new_invalid();
    for at in offset..offset + size as u64 {
        let addr = Address::new(space.clone(), at);
        let flags = data.get_arch().query_global_properties(&addr, 1, &usepoint);
        if flags & crate::varnode::varnode_flags::readonly == 0
            || flags & crate::varnode::varnode_flags::volatil != 0
        {
            return None;
        }
    }
    data.get_arch()
        .get_load_image_value(&Address::new(space.clone(), offset), size)
        .ok()
        .map(Value::Constant)
}

fn reads_return(data: &Funcdata, input: VarnodeId, state: &State) -> bool {
    let (space, offset, size) = key(data, input);
    let end = offset.saturating_add(size as u64);
    state
        .values
        .range(..=(space, offset, i32::MAX))
        .next_back()
        .is_some_and(|(&(at_space, at, width), &value)| {
            value == Value::ReturnAddress
                && space == at_space
                && offset < at.saturating_add(width as u64)
        })
        || state
            .values
            .range((space, offset, 0)..(space, end, 0))
            .any(|(_, &value)| value == Value::ReturnAddress)
}

fn transfer(data: &Funcdata, op: OpId, state: &mut State) {
    let o = data.obank().get(op).expect("raw flow op");
    match o.code() {
        OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => {
            state.return_address_observed |= state
                .values
                .values()
                .chain(state.stack.values())
                .any(|&value| value == Value::ReturnAddress);
            state.stack.clear();
            state.image_unmodified = false;
            let proto = data
                .get_call_specs_index(op)
                .map(|i| data.get_call_specs(i).proto());
            state.values.retain(|&(space, offset, size), _| {
                let Some(space) = data.get_arch().manage().get_space(space) else {
                    return false;
                };
                proto.is_some_and(|p| {
                    p.has_effect(&Address::new(space.clone(), offset), size)
                        == crate::fspec::effect_type::UNAFFECTED
                })
            });
        }
        OpCode::CPUI_CALLOTHER => {
            *state = State {
                return_address_observed: state.return_address_observed
                    || state
                        .values
                        .values()
                        .chain(state.stack.values())
                        .any(|&value| value == Value::ReturnAddress),
                ..State::default()
            }
        }
        OpCode::CPUI_STORE => {
            let stored = o.get_in(2).expect("STORE value");
            let size = key(data, stored).2;
            if let Some(offset) = stack_address(data, op, state) {
                state.return_address_observed |= state.stack.iter().any(|(&(at, width), &v)| {
                    v == Value::ReturnAddress
                        && (at.wrapping_sub(offset) < size as u32
                            || offset.wrapping_sub(at) < width as u32)
                        && (at != offset || width != size || value(data, stored, state) != Some(v))
                });
                state.stack.retain(|&(at, width), _| {
                    at.wrapping_sub(offset) >= size as u32
                        && offset.wrapping_sub(at) >= width as u32
                });
                if let Some(v) = value(data, stored, state) {
                    state.stack.insert((offset, size), v);
                }
            } else {
                state.return_address_observed |= reads_return(data, stored, state);
                state.return_address_observed |=
                    state.stack.values().any(|&v| v == Value::ReturnAddress);
                state.stack.clear();
                state.image_unmodified = false;
            }
        }
        _ => {
            if let Some(out) = o.get_out() {
                let v = if o.code() == OpCode::CPUI_LOAD {
                    if let Some(at) = stack_address(data, op, state) {
                        if state.stack.iter().any(|(&(offset, width), &value)| {
                            value == Value::ReturnAddress
                                && (at.wrapping_sub(offset) < width as u32
                                    || offset.wrapping_sub(at) < key(data, out).2 as u32)
                                && (at != offset || key(data, out).2 != width)
                        }) {
                            state.return_address_observed = true;
                        }
                        state.stack.get(&(at, key(data, out).2)).copied()
                    } else {
                        image_value(data, op, state)
                    }
                } else {
                    pure(data, op, state)
                };
                if v.is_none() && !state.return_address_observed {
                    for i in 0..o.num_input() {
                        if reads_return(data, o.get_in(i).expect("raw op input"), state) {
                            state.return_address_observed = true;
                        }
                    }
                }
                write(state, key(data, out), v);
            }
        }
    }
}

fn merge_value(a: Value, b: Value) -> Option<Value> {
    if a == b {
        return Some(a);
    }
    match (a, b) {
        (Value::Constant(_) | Value::Computed, Value::Constant(_) | Value::Computed) => {
            Some(Value::Computed)
        }
        _ => None,
    }
}

fn merge(prior: &mut State, incoming: &State) -> bool {
    let old = prior.clone();
    prior.image_unmodified &= incoming.image_unmodified;
    prior.return_address_observed |= incoming.return_address_observed;
    if !prior.return_address_observed {
        prior.return_address_observed = prior
            .values
            .iter()
            .any(|(k, &v)| v == Value::ReturnAddress && incoming.values.get(k) != Some(&v))
            || incoming
                .values
                .iter()
                .any(|(k, &v)| v == Value::ReturnAddress && prior.values.get(k) != Some(&v))
            || prior
                .stack
                .iter()
                .any(|(k, &v)| v == Value::ReturnAddress && incoming.stack.get(k) != Some(&v))
            || incoming
                .stack
                .iter()
                .any(|(k, &v)| v == Value::ReturnAddress && prior.stack.get(k) != Some(&v));
    }
    prior.values.retain(|k, v| {
        if let Some(joined) = incoming
            .values
            .get(k)
            .and_then(|other| merge_value(*v, *other))
        {
            *v = joined;
            true
        } else {
            false
        }
    });
    prior.stack.retain(|k, v| {
        if let Some(joined) = incoming
            .stack
            .get(k)
            .and_then(|other| merge_value(*v, *other))
        {
            *v = joined;
            true
        } else {
            false
        }
    });
    *prior != old
}

pub(crate) enum Classification {
    Sites(Vec<Address>),
    Changed,
}

/// Recheck temporary decisions against the complete graph before publishing them.
#[allow(clippy::mutable_key_type)]
pub(crate) fn classify(
    data: &mut Funcdata,
    arch: &Architecture,
    automatic: &BTreeSet<Address>,
    work: &mut usize,
) -> KunaResult<Classification> {
    match sites(data, arch, automatic, work) {
        Some(result) => result,
        None if automatic.is_empty() => Ok(Classification::Sites(Vec::new())),
        None => Err(KunaError::lowlevel("Cannot revalidate PPC32 LR flow")),
    }
}

#[allow(clippy::mutable_key_type)]
fn sites(
    data: &mut Funcdata,
    arch: &Architecture,
    automatic: &BTreeSet<Address>,
    work: &mut usize,
) -> Option<KunaResult<Classification>> {
    if !arch.archid.starts_with("PowerPC:") || !arch.archid.contains(":32:") {
        return None;
    }
    let lr = register(arch, "LR")?;
    if !matches!(lr.2, 4 | 8)
        || (automatic.is_empty()
            && !data.obank().iter_code(OpCode::CPUI_RETURN).any(|r| {
                data.obank()
                    .get(r)
                    .and_then(|o| o.get_in(0))
                    .is_some_and(|v| key(data, v) == lr)
            }))
    {
        return None;
    }
    let n = data.bblocks_get_size() as usize;
    if n == 0 || n > 4096 {
        return None;
    }
    let blocks: Vec<_> = (0..n).map(|i| data.bblocks_get_block(i as i32)).collect();
    let indices: BTreeMap<_, _> = blocks.iter().enumerate().map(|(i, b)| (*b, i)).collect();
    let mut initial = State {
        image_unmodified: true,
        ..State::default()
    };
    initial.values.insert(lr, Value::ReturnAddress);
    initial
        .values
        .insert(register(arch, "r1")?, Value::Stack(0));
    let mut inputs = vec![None; n];
    inputs[0] = Some(initial);
    let mut queue = VecDeque::from([0usize]);
    let mut queued = vec![false; n];
    queued[0] = true;
    while let Some(i) = queue.pop_front() {
        queued[i] = false;
        let mut state = inputs[i].clone()?;
        for op in data.bb_ops(blocks[i]) {
            step(data, op, &mut state, work)?;
        }
        let block = data.bblocks_ref().block(blocks[i]);
        for j in 0..block.size_out() {
            let next = *indices.get(&block.get_out(j))?;
            let changed = if let Some(prior) = &mut inputs[next] {
                merge(prior, &state)
            } else {
                inputs[next] = Some(state.clone());
                true
            };
            if changed && !queued[next] {
                queued[next] = true;
                queue.push_back(next);
            }
        }
    }
    let mut result = Vec::new();
    for (i, &block) in blocks.iter().enumerate() {
        let Some(mut state) = inputs[i].clone() else {
            continue;
        };
        let ops = data.bb_ops(block);
        for op in ops {
            let o = data.obank().get(op)?;
            let automatic_transfer =
                matches!(o.code(), OpCode::CPUI_BRANCHIND | OpCode::CPUI_CALLIND)
                    && automatic.contains(o.get_addr());
            let target = if automatic_transfer || o.code() == OpCode::CPUI_RETURN {
                o.get_in(0).and_then(|v| value(data, v, &state))
            } else {
                None
            };
            let computed = !state.return_address_observed
                && matches!(target, Some(Value::Constant(_) | Value::Computed));
            if automatic_transfer && !computed {
                if o.code() != OpCode::CPUI_BRANCHIND || data.bb_op_tail(block) != Some(op) {
                    return Some(Err(KunaError::lowlevel(
                        "Cannot revalidate PPC32 LR transfer after discovering additional paths",
                    )));
                }
                for edge in 0..data.bblocks_ref().block(block).size_in() {
                    let pred = data.bblocks_ref().block(block).get_in(edge);
                    let Some(mut incoming) = inputs[*indices.get(&pred)?].clone() else {
                        continue;
                    };
                    for prior in data.bb_ops(pred) {
                        step(data, prior, &mut incoming, work)?;
                    }
                    for prior in data.bb_ops(block) {
                        if prior == op {
                            break;
                        }
                        step(data, prior, &mut incoming, work)?;
                    }
                    if o.get_in(0).and_then(|v| value(data, v, &incoming))
                        == Some(Value::ReturnAddress)
                    {
                        return Some(
                            split_return_edge(data, block, edge, op)
                                .map(|()| Classification::Changed),
                        );
                    }
                }
                return Some(Err(KunaError::lowlevel(
                    "Cannot revalidate PPC32 LR transfer after discovering additional paths",
                )));
            }
            if o.code() == OpCode::CPUI_RETURN
                && o.get_halt_type() == 0
                && data.get_override().get_flow_override(o.get_addr())
                    == crate::overrides::flow_type::NONE
                && computed
                && o.get_in(0).is_some_and(|v| key(data, v) == lr)
            {
                result.push(o.get_addr().clone());
            }
            step(data, op, &mut state, work)?;
        }
    }
    Some(Ok(Classification::Sites(result)))
}

fn step(data: &Funcdata, op: OpId, state: &mut State, work: &mut usize) -> Option<()> {
    *work += 1;
    if *work > 1_000_000 || state.values.len() > 4096 || state.stack.len() > 4096 {
        return None;
    }
    transfer(data, op, state);
    Some(())
}

/// The duplicate is a closed returning path, so no cloned output escapes it.
fn split_return_edge(data: &mut Funcdata, block: BlockId, edge: i32, tail: OpId) -> KunaResult<()> {
    let ops = data.bb_ops(block);
    if data.bblocks_ref().block(block).size_in() < 2
        || ops.iter().any(|&op| {
            op != tail
                && data.obank().get(op).is_none_or(|o| {
                    o.is_branch()
                        || o.is_call()
                        || matches!(
                            o.code(),
                            OpCode::CPUI_CALLOTHER
                                | OpCode::CPUI_INDIRECT
                                | OpCode::CPUI_MULTIEQUAL
                        )
                })
        })
    {
        return Err(KunaError::lowlevel(
            "Cannot split mixed PPC32 LR transfer contexts",
        ));
    }
    let duplicate = data.node_split_block_edge(block, edge);
    while data.bblocks_ref().block(duplicate).size_out() != 0 {
        let out = data.bblocks_ref().block(duplicate).get_out(0);
        data.bblocks_mut().remove_edge(duplicate, out);
    }
    let mut copied = BTreeMap::new();
    for old in ops {
        let o = data.obank().get(old).expect("raw LR context op");
        let code = if old == tail {
            OpCode::CPUI_RETURN
        } else {
            o.code()
        };
        let addr = o.get_addr().clone();
        let inputs: Vec<_> = (0..o.num_input()).map(|i| o.get_in(i).unwrap()).collect();
        let output = o.get_out();
        let new = data.new_op(inputs.len() as i32, addr);
        data.op_set_opcode_code(new, code);
        for (i, input) in inputs.into_iter().enumerate() {
            let v = data.vbank().get(input).expect("raw LR context input");
            let (size, addr, constant) = (v.get_size(), v.get_addr().clone(), v.is_constant());
            let value = if constant {
                data.new_constant(size, addr.get_offset())
            } else if let Some(&copy) = copied.get(&input) {
                copy
            } else {
                data.new_varnode(size, &addr, None)
            };
            data.op_set_input(new, value, i as i32)?;
        }
        if let Some(output) = output {
            let v = data.vbank().get(output).expect("raw LR context output");
            let (size, addr) = (v.get_size(), v.get_addr().clone());
            copied.insert(output, data.new_varnode_out(size, &addr, new)?);
        }
        data.op_insert_end(new, duplicate);
    }
    data.structure_reset();
    data.sort_call_specs();
    Ok(())
}
