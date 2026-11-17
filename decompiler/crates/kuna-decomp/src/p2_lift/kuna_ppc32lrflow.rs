//! Classify PPC32 LR transfers before prototype recovery discards RETURN's target.
//!
//! SLEIGH spells `blr` (including conditional forms) as RETURN even when LR was
//! assigned a local code address or an argument. A bounded raw-p-code fixed point
//! distinguishes those values from the incoming LR and unproved memory reads.
//! Proven computed sites are re-followed as BRANCHIND by the normal jump-table
//! machinery. Saved-LR returns and explicit flow assertions retain their meaning.

use std::collections::{BTreeMap, VecDeque};

use kuna_base::address::Address;
use kuna_base::error::KunaResult;
use kuna_num::opcodes::OpCode;

use crate::{
    architecture::Architecture,
    context::{OpId, VarnodeId},
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

fn transfer(data: &Funcdata, op: OpId, state: &mut State) {
    let o = data.obank().get(op).expect("raw flow op");
    match o.code() {
        OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => {
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
        OpCode::CPUI_CALLOTHER => *state = State::default(),
        OpCode::CPUI_STORE => {
            let stored = o.get_in(2).expect("STORE value");
            let size = key(data, stored).2;
            if let Some(offset) = stack_address(data, op, state) {
                state.stack.retain(|&(at, width), _| {
                    at.wrapping_sub(offset) >= size as u32
                        && offset.wrapping_sub(at) >= width as u32
                });
                if let Some(v) = value(data, stored, state) {
                    state.stack.insert((offset, size), v);
                }
            } else {
                state.stack.clear();
                state.image_unmodified = false;
            }
        }
        _ => {
            if let Some(out) = o.get_out() {
                let v = if o.code() == OpCode::CPUI_LOAD {
                    if let Some(at) = stack_address(data, op, state) {
                        state.stack.get(&(at, key(data, out).2)).copied()
                    } else {
                        image_value(data, op, state)
                    }
                } else {
                    pure(data, op, state)
                };
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

/// Sites whose target is computed in this invocation, rather than its incoming LR.
/// No claim survives unknown writes, aliasing stores, calls or the work bound.
pub(crate) fn computed_sites(data: &Funcdata, arch: &Architecture) -> Vec<Address> {
    sites(data, arch).unwrap_or_default()
}

fn sites(data: &Funcdata, arch: &Architecture) -> Option<Vec<Address>> {
    if !arch.archid.starts_with("PowerPC:") || !arch.archid.contains(":32:") {
        return None;
    }
    let lr = register(arch, "LR")?;
    if !matches!(lr.2, 4 | 8)
        || !data.obank().iter_code(OpCode::CPUI_RETURN).any(|r| {
            data.obank()
                .get(r)
                .and_then(|o| o.get_in(0))
                .is_some_and(|v| key(data, v) == lr)
        })
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
    for r in 0..32 {
        initial
            .values
            .insert(register(arch, &format!("r{r}"))?, Value::Computed);
    }
    initial.values.insert(lr, Value::ReturnAddress);
    initial
        .values
        .insert(register(arch, "r1")?, Value::Stack(0));
    let mut inputs = vec![None; n];
    inputs[0] = Some(initial);
    let mut queue = VecDeque::from([0usize]);
    let mut queued = vec![false; n];
    queued[0] = true;
    let mut operations = 0usize;
    while let Some(i) = queue.pop_front() {
        queued[i] = false;
        let mut state = inputs[i].clone()?;
        for op in data.bb_ops(blocks[i]) {
            operations += 1;
            if operations > 1_000_000 || state.values.len() > 4096 || state.stack.len() > 4096 {
                return None;
            }
            transfer(data, op, &mut state);
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
    for (i, block) in blocks.into_iter().enumerate() {
        let Some(mut state) = inputs[i].take() else {
            continue;
        };
        for op in data.bb_ops(block) {
            let o = data.obank().get(op)?;
            if o.code() == OpCode::CPUI_RETURN
                && o.get_halt_type() == 0
                && data.get_override().get_flow_override(o.get_addr())
                    == crate::overrides::flow_type::NONE
                && o.get_in(0).is_some_and(|v| {
                    key(data, v) == lr
                        && matches!(
                            value(data, v, &state),
                            Some(Value::Constant(_) | Value::Computed)
                        )
                })
            {
                result.push(o.get_addr().clone());
            }
            transfer(data, op, &mut state);
        }
    }
    Some(result)
}
