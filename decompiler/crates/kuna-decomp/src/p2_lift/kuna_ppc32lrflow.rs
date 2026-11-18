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
    jumptable::JumpTable,
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

const MAX_CONTEXTS: usize = 8;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Value {
    Constant(u64),
    Bits { value: u64, known: u64 },
    Stack(u32),
    ReturnAddress,
}

fn width_mask(size: i32) -> Option<u64> {
    (1..=8).contains(&size).then(|| u64::MAX >> (64 - size * 8))
}

fn known_bits(value: u64, known: u64, width: u64) -> Option<Value> {
    let known = known & width;
    let value = value & known;
    if known == width {
        Some(Value::Constant(value))
    } else if known == 0 {
        None
    } else {
        Some(Value::Bits { value, known })
    }
}

#[derive(Clone, Default, PartialEq, Eq)]
struct State {
    values: BTreeMap<Key, Value>,
    stack: BTreeMap<(u32, i32), Value>,
    image_unmodified: bool,
    return_address_observed: bool,
}

/// Keep the facts from each incoming path together. If the bound is reached,
/// retain only facts shared by all paths.
#[derive(Clone, Default)]
struct Contexts {
    states: Vec<State>,
    widened: bool,
}

impl Contexts {
    fn join<'a>(&mut self, incoming: impl Iterator<Item = &'a State>) -> bool {
        let mut changed = false;
        for state in incoming {
            if self.widened {
                changed |= intersect(&mut self.states[0], state);
            } else if !self.states.contains(state) {
                if self.states.len() < MAX_CONTEXTS {
                    self.states.push(state.clone());
                } else {
                    let (first, rest) = self.states.split_first_mut().unwrap();
                    for other in rest {
                        intersect(first, other);
                    }
                    intersect(first, state);
                    self.states.truncate(1);
                    self.widened = true;
                }
                changed = true;
            }
        }
        changed
    }
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
        let mask = width_mask(wanted.2)?;
        match stored {
            Value::Constant(n) => Some(Value::Constant((n >> shift) & mask)),
            Value::Bits { value, known } => known_bits(value >> shift, known >> shift, mask),
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
    let size = data.vbank().get(o.get_in(0)?)?.get_size();
    let behavior = data.get_arch().op_behavior(o.code())?;
    let width = out.get_size();
    match inputs.as_slice() {
        [Value::Constant(a)] => behavior.evaluate_unary(width, size, *a),
        [Value::Constant(a), Value::Constant(b)] => behavior.evaluate_binary(width, size, *a, *b),
        [Value::Constant(a), Value::Constant(b), Value::Constant(c)] => {
            behavior.evaluate_ternary(width, size, *a, *b, *c)
        }
        _ => return None,
    }
    .ok()
    .map(Value::Constant)
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
        .is_some_and(|(&(at_space, at, width), value)| {
            *value == Value::ReturnAddress
                && space == at_space
                && offset < at.saturating_add(width as u64)
        })
        || state
            .values
            .range((space, offset, 0)..(space, end, 0))
            .any(|(_, value)| *value == Value::ReturnAddress)
}

/// Preserve known condition bits when SLEIGH packs an unknown status bit
/// into the same register. Address identities still require exact copies.
fn partial_bitwise(data: &Funcdata, op: OpId, state: &State) -> Option<Value> {
    let o = data.obank().get(op)?;
    use OpCode::{CPUI_INT_AND, CPUI_INT_OR, CPUI_INT_RIGHT};
    if !matches!(o.code(), CPUI_INT_AND | CPUI_INT_OR | CPUI_INT_RIGHT) {
        return None;
    }
    let size = key(data, o.get_out()?).2;
    let width = width_mask(size)?;
    let bits = |id| match value(data, id, state) {
        Some(Value::Constant(value)) => Some((value & width, width)),
        Some(Value::Bits { value, known }) => Some((value, known)),
        None if !reads_return(data, id, state) => Some((0, 0)),
        _ => None,
    };
    let (a, a_mask) = bits(o.get_in(0)?)?;
    let right = o.get_in(1)?;
    if o.code() == CPUI_INT_RIGHT {
        let Value::Constant(shift) = value(data, right, state)? else {
            return None;
        };
        if shift >= (size * 8) as u64 {
            return Some(Value::Constant(0));
        }
        return known_bits(a >> shift, (a_mask >> shift) | !(width >> shift), width);
    }
    let (b, b_mask) = bits(right)?;
    let (a_zero, b_zero) = (a_mask & !a, b_mask & !b);
    let (ones, zeros) = if o.code() == CPUI_INT_AND {
        (a & b, a_zero | b_zero)
    } else {
        (a | b, a_zero & b_zero)
    };
    known_bits(ones, ones | zeros, width)
}

fn transfer(data: &Funcdata, op: OpId, state: &mut State) {
    let o = data.obank().get(op).expect("raw flow op");
    match o.code() {
        OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => {
            state.return_address_observed |= state
                .values
                .values()
                .chain(state.stack.values())
                .any(|value| *value == Value::ReturnAddress);
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
                        .any(|value| *value == Value::ReturnAddress),
                ..State::default()
            }
        }
        OpCode::CPUI_STORE => {
            let stored = o.get_in(2).expect("STORE value");
            let size = key(data, stored).2;
            if let Some(offset) = stack_address(data, op, state) {
                let stored_value = value(data, stored, state);
                state.return_address_observed |=
                    stored_value != Some(Value::ReturnAddress) && reads_return(data, stored, state);
                state.stack.retain(|&(at, width), v| {
                    let from_store = at.wrapping_sub(offset);
                    let overlaps =
                        from_store < size as u32 || offset.wrapping_sub(at) < width as u32;
                    let covered = width <= size && from_store <= (size - width) as u32;
                    // A partial overwrite can leave bytes of the saved LR behind.
                    state.return_address_observed |=
                        *v == Value::ReturnAddress && overlaps && !covered;
                    !overlaps
                });
                if let Some(v) = stored_value {
                    state.stack.insert((offset, size), v);
                }
            } else {
                state.return_address_observed |= reads_return(data, stored, state);
                state.return_address_observed |=
                    state.stack.values().any(|v| *v == Value::ReturnAddress);
                state.stack.clear();
                state.image_unmodified = false;
            }
        }
        _ => {
            if let Some(out) = o.get_out() {
                let v = if o.code() == OpCode::CPUI_LOAD {
                    if let Some(at) = stack_address(data, op, state) {
                        if state.stack.iter().any(|(&(offset, width), value)| {
                            *value == Value::ReturnAddress
                                && (at.wrapping_sub(offset) < width as u32
                                    || offset.wrapping_sub(at) < key(data, out).2 as u32)
                                && (at != offset || key(data, out).2 != width)
                        }) {
                            state.return_address_observed = true;
                        }
                        state.stack.get(&(at, key(data, out).2)).copied()
                    } else {
                        let loaded = image_value(data, op, state);
                        // An unresolved address can alias a saved return address.
                        state.return_address_observed |= loaded.is_none()
                            && state.stack.values().any(|v| *v == Value::ReturnAddress);
                        loaded
                    }
                } else {
                    pure(data, op, state).or_else(|| partial_bitwise(data, op, state))
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

fn intersect(prior: &mut State, incoming: &State) -> bool {
    let before = (
        prior.image_unmodified,
        prior.return_address_observed,
        prior.values.len(),
        prior.stack.len(),
    );
    prior.image_unmodified &= incoming.image_unmodified;
    prior.return_address_observed |= incoming.return_address_observed;
    if !prior.return_address_observed {
        prior.return_address_observed = prior
            .values
            .iter()
            .any(|(k, v)| *v == Value::ReturnAddress && incoming.values.get(k) != Some(v))
            || incoming
                .values
                .iter()
                .any(|(k, v)| *v == Value::ReturnAddress && prior.values.get(k) != Some(v))
            || prior
                .stack
                .iter()
                .any(|(k, v)| *v == Value::ReturnAddress && incoming.stack.get(k) != Some(v))
            || incoming
                .stack
                .iter()
                .any(|(k, v)| *v == Value::ReturnAddress && prior.stack.get(k) != Some(v));
    }
    prior
        .values
        .retain(|k, v| incoming.values.get(k) == Some(v));
    prior.stack.retain(|k, v| incoming.stack.get(k) == Some(v));
    before
        != (
            prior.image_unmodified,
            prior.return_address_observed,
            prior.values.len(),
            prior.stack.len(),
        )
}

#[allow(clippy::mutable_key_type)]
fn proven_targets(
    data: &Funcdata,
    contexts: &Contexts,
    input: Option<VarnodeId>,
    direct_lr: Option<Key>,
) -> Option<Vec<Address>> {
    let space = data.get_address().get_space()?;
    let mut addresses = BTreeSet::new();
    for state in &contexts.states {
        if state.return_address_observed {
            return None;
        }
        let target = if let Some(lr) = direct_lr {
            // A recovered tail call carries the callee operand instead of LR.
            state.values.get(&lr).copied()
        } else {
            value(data, input?, state)
        };
        let Value::Constant(raw) = target? else {
            return None;
        };
        addresses.insert(Address::new(space.clone(), raw & 0xffff_fffc));
    }
    (!addresses.is_empty()).then(|| addresses.into_iter().collect())
}

#[derive(Clone)]
pub(crate) struct BranchPlan {
    pub(crate) targets: Vec<Address>,
    pub(crate) use_proven_targets: bool,
    pub(crate) allow_tail_call: bool,
}

pub(crate) type BranchPlans = BTreeMap<Address, BranchPlan>;

#[derive(Clone, Copy)]
enum EdgeRule<'a> {
    Conditional(VarnodeId, bool),
    Dispatch(VarnodeId, &'a JumpTable),
}

fn edge_rule<'a>(
    data: &'a Funcdata,
    ops: &[OpId],
    automatic: &BranchPlans,
    successors: i32,
) -> Option<EdgeRule<'a>> {
    let &tail = ops.last()?;
    let op = data.obank().get(tail)?;
    match op.code() {
        OpCode::CPUI_CBRANCH if successors == 2 => {
            Some(EdgeRule::Conditional(op.get_in(1)?, op.is_boolean_flip()))
        }
        OpCode::CPUI_BRANCHIND if automatic.contains_key(op.get_addr()) => Some(
            EdgeRule::Dispatch(op.get_in(0)?, data.find_jump_table(tail)?),
        ),
        _ => None,
    }
}

/// A newly proved destination can be absent from the current table.
/// It participates in revalidation but does not reach any existing edge.
fn reaches_edge(data: &Funcdata, rule: Option<EdgeRule<'_>>, state: &State, edge: i32) -> bool {
    match rule {
        Some(EdgeRule::Conditional(input, flipped)) => match value(data, input, state) {
            Some(Value::Constant(test)) => edge == i32::from((test != 0) != flipped),
            _ => true,
        },
        Some(EdgeRule::Dispatch(input, table)) => match value(data, input, state) {
            Some(Value::Constant(raw)) => {
                let space = data.get_address().get_space().expect("PPC32 code space");
                let dest = Address::new(space.clone(), raw & 0xffff_fffc);
                table.out_edge_for_address(&dest) == Some(edge)
            }
            Some(Value::ReturnAddress) => false,
            _ => true,
        },
        None => true,
    }
}

pub(crate) enum Classification {
    Sites(BranchPlans),
    Changed,
}

/// Recheck temporary decisions against the complete graph before publishing them.
#[allow(clippy::mutable_key_type)]
pub(crate) fn classify(
    data: &mut Funcdata,
    arch: &Architecture,
    automatic: &BranchPlans,
    work: &mut usize,
) -> KunaResult<Classification> {
    match sites(data, arch, automatic, work) {
        Some(result) => result,
        None if automatic.is_empty() => Ok(Classification::Sites(BranchPlans::new())),
        None => Err(KunaError::lowlevel("Cannot revalidate PPC32 LR flow")),
    }
}

#[allow(clippy::mutable_key_type)]
fn sites(
    data: &mut Funcdata,
    arch: &Architecture,
    automatic: &BranchPlans,
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
    let mut inputs = vec![Contexts::default(); n];
    inputs[0].states.push(initial);
    let mut queue = VecDeque::from([0usize]);
    let mut queued = vec![false; n];
    queued[0] = true;
    while let Some(i) = queue.pop_front() {
        queued[i] = false;
        let mut state = inputs[i].clone();
        let ops = data.bb_ops(blocks[i]);
        for &op in &ops {
            step(data, op, &mut state, work)?;
        }
        let block = data.bblocks_ref().block(blocks[i]);
        let rule = edge_rule(data, &ops, automatic, block.size_out());
        for j in 0..block.size_out() {
            let next = *indices.get(&block.get_out(j))?;
            let incoming = state
                .states
                .iter()
                .filter(|s| reaches_edge(data, rule, s, j));
            let changed = inputs[next].join(incoming);
            if changed && !queued[next] {
                queued[next] = true;
                queue.push_back(next);
            }
        }
    }
    let mut result = BranchPlans::new();
    for (i, &block) in blocks.iter().enumerate() {
        if inputs[i].states.is_empty() {
            continue;
        }
        let mut state = inputs[i].clone();
        let ops = data.bb_ops(block);
        for op in ops {
            let o = data.obank().get(op)?;
            let plan = automatic
                .get(o.get_addr())
                .filter(|_| matches!(o.code(), OpCode::CPUI_BRANCHIND | OpCode::CPUI_CALL));
            let direct = o.code() == OpCode::CPUI_CALL;
            let targets = if plan.is_some() || o.code() == OpCode::CPUI_RETURN {
                proven_targets(data, &state, o.get_in(0), direct.then_some(lr))
            } else {
                None
            };
            if let Some(plan) = plan {
                if targets.is_none() {
                    let return_input = if direct {
                        Some(lr)
                    } else {
                        o.get_in(0).map(|vn| key(data, vn))
                    }?;
                    for edge in 0..data.bblocks_ref().block(block).size_in() {
                        let pred = data.bblocks_ref().block(block).get_in(edge);
                        let mut incoming = inputs[*indices.get(&pred)?].clone();
                        replay_edge(data, block, edge, op, automatic, &mut incoming, work)?;
                        if !incoming.states.is_empty()
                            && incoming.states.iter().all(|state| {
                                state.values.get(&return_input) == Some(&Value::ReturnAddress)
                            })
                        {
                            let raw_lr = direct
                                .then(|| arch.manage().get_space(lr.0))
                                .flatten()
                                .map(|space| (Address::new(space.clone(), lr.1), lr.2));
                            if direct && raw_lr.is_none() {
                                return None;
                            }
                            return Some(
                                split_return_edge(data, block, edge, op, raw_lr)
                                    .map(|()| Classification::Changed),
                            );
                        }
                    }
                }
                if direct {
                    let callee = data.get_call_specs_index(op).map(|idx| {
                        let call = data.get_call_specs(idx);
                        call.plt_stub().unwrap_or_else(|| call.get_entry_address())
                    });
                    let valid = targets.as_deref().is_some_and(
                        |addresses| matches!(addresses, [dest] if Some(dest) == callee),
                    );
                    if !valid {
                        if !plan.allow_tail_call {
                            return Some(Err(KunaError::lowlevel(
                                "Cannot revalidate a rewritten PPC32 LR transfer",
                            )));
                        }
                        let mut retry = plan.clone();
                        retry.allow_tail_call = false;
                        result.insert(o.get_addr().clone(), retry);
                    }
                } else if let Some(targets) = targets {
                    let covered = data.find_jump_table(op).is_some_and(|jt| {
                        let actual = jt.address_table();
                        targets.iter().all(|dest| actual.contains(dest))
                            && actual.iter().all(|dest| targets.contains(dest))
                    });
                    if !covered {
                        if plan.use_proven_targets && plan.targets == targets {
                            return Some(Err(KunaError::lowlevel(
                                "Cannot recover the proven PPC32 LR destinations",
                            )));
                        }
                        let mut retry = plan.clone();
                        retry.targets = targets;
                        retry.use_proven_targets = true;
                        result.insert(o.get_addr().clone(), retry);
                    } else if plan.use_proven_targets && targets.len() > 1 {
                        let basic = data.bblocks_ref().block(block);
                        if (0..basic.size_in()).any(|edge| basic.is_loop_in(edge)) {
                            let mut incoming_targets = Vec::new();
                            for edge in 0..basic.size_in() {
                                let pred = basic.get_in(edge);
                                let mut incoming = inputs[*indices.get(&pred)?].clone();
                                replay_edge(data, block, edge, op, automatic, &mut incoming, work)?;
                                let targets = proven_targets(data, &incoming, o.get_in(0), None);
                                let Some([dest]) = targets.as_deref() else {
                                    return Some(Err(dispatch_loop_error()));
                                };
                                let slot = data.find_jump_table(op)?.out_edge_for_address(dest)?;
                                incoming_targets.push((dest.clone(), basic.get_out(slot)));
                            }
                            return Some(
                                specialize_dispatch_loop(data, block, op, &incoming_targets)
                                    .map(|()| Classification::Changed),
                            );
                        }
                    }
                } else {
                    return Some(Err(KunaError::lowlevel(
                        "Cannot revalidate PPC32 LR transfer after discovering additional paths",
                    )));
                }
            } else if o.code() == OpCode::CPUI_RETURN
                && o.get_halt_type() == 0
                && data.get_override().get_flow_override(o.get_addr())
                    == crate::overrides::flow_type::NONE
                && o.get_in(0).is_some_and(|v| key(data, v) == lr)
            {
                if let Some(targets) = targets {
                    result.insert(
                        o.get_addr().clone(),
                        BranchPlan {
                            targets,
                            use_proven_targets: false,
                            allow_tail_call: true,
                        },
                    );
                }
            }
            step(data, op, &mut state, work)?;
        }
    }
    Some(Ok(Classification::Sites(result)))
}

fn step(data: &Funcdata, op: OpId, contexts: &mut Contexts, work: &mut usize) -> Option<()> {
    for state in &mut contexts.states {
        *work += 1;
        if *work > 1_000_000 || state.values.len() > 4096 || state.stack.len() > 4096 {
            return None;
        }
        transfer(data, op, state);
    }
    Some(())
}

fn replay_edge(
    data: &Funcdata,
    block: BlockId,
    edge: i32,
    tail: OpId,
    automatic: &BranchPlans,
    state: &mut Contexts,
    work: &mut usize,
) -> Option<()> {
    let block_info = data.bblocks_ref().block(block);
    let pred = block_info.get_in(edge);
    let ops = data.bb_ops(pred);
    for &op in &ops {
        step(data, op, state, work)?;
    }
    let rule = edge_rule(
        data,
        &ops,
        automatic,
        data.bblocks_ref().block(pred).size_out(),
    );
    let outgoing = block_info.get_in_rev_index(edge);
    state
        .states
        .retain(|s| reaches_edge(data, rule, s, outgoing));
    for op in data.bb_ops(block).into_iter().take_while(|&op| op != tail) {
        step(data, op, state, work)?;
    }
    Some(())
}

fn cloneable(data: &Funcdata, ops: &[OpId]) -> bool {
    ops.iter().all(|&id| {
        data.obank().get(id).is_some_and(|op| {
            !op.is_branch()
                && !op.is_call()
                && !matches!(
                    op.code(),
                    OpCode::CPUI_CALLOTHER | OpCode::CPUI_INDIRECT | OpCode::CPUI_MULTIEQUAL
                )
        })
    })
}

fn dispatch_loop_error() -> KunaError {
    KunaError::lowlevel("Cannot separate PPC32 LR destinations on a dispatch loop")
}

/// Lower each proven incoming path to its own straight-line branch. This keeps
/// the backedge when the switch itself would become the head of its loop.
fn specialize_dispatch_loop(
    data: &mut Funcdata,
    block: BlockId,
    tail: OpId,
    incoming: &[(Address, BlockId)],
) -> KunaResult<()> {
    let basic = data.bblocks_ref().block(block);
    let n = basic.size_in();
    let preds: BTreeSet<_> = (0..n).map(|i| basic.get_in(i)).collect();
    let mut prefix = data.bb_ops(block);
    if !(2..=16).contains(&n)
        || preds.len() != n as usize
        || preds.contains(&block)
        || prefix.pop() != Some(tail)
        || incoming.iter().any(|(_, dest)| *dest == block)
        || !cloneable(data, &prefix)
    {
        return Err(dispatch_loop_error());
    }
    let site = data.obank().get(tail).unwrap().get_addr().clone();
    let table = data
        .find_jump_table_index(tail)
        .ok_or_else(dispatch_loop_error)?;
    for edge in (1..n).rev() {
        let duplicate = data.node_split_block_edge(block, edge);
        copy_lr_ops(data, duplicate, &prefix, None)?;
        let branch = data.new_op(1, site.clone());
        set_direct_exit(data, duplicate, branch, &incoming[edge as usize])?;
        data.op_insert_end(branch, duplicate);
    }
    set_direct_exit(data, block, tail, &incoming[0])?;
    data.remove_jump_table(table);
    data.structure_reset();
    data.sort_call_specs();
    Ok(())
}

fn set_direct_exit(
    data: &mut Funcdata,
    block: BlockId,
    op: OpId,
    (address, dest): &(Address, BlockId),
) -> KunaResult<()> {
    let basic = data.bblocks_ref().block(block);
    let remove: Vec<_> = (0..basic.size_out())
        .map(|i| basic.get_out(i))
        .filter(|out| out != dest)
        .collect();
    for out in remove {
        data.bblocks_mut().remove_edge(block, out);
    }
    debug_assert_eq!(data.bblocks_ref().block(block).size_out(), 1);
    data.bblocks_mut()
        .block_mut(block)
        .clear_flag(crate::block::block_flags::f_switch_out);
    data.op_set_opcode_code(op, OpCode::CPUI_BRANCH);
    let target = data.new_code_ref(address);
    data.op_set_input(op, target, 0)
}

/// The duplicate is a closed returning path, so no cloned output escapes it.
fn split_return_edge(
    data: &mut Funcdata,
    block: BlockId,
    edge: i32,
    tail: OpId,
    raw_lr: Option<(Address, i32)>,
) -> KunaResult<()> {
    let mut ops = data.bb_ops(block);
    let tail_index = ops
        .iter()
        .position(|&op| op == tail)
        .expect("LR transfer in block");
    let site = data.obank().get(tail).unwrap().get_addr().clone();
    let remainder = &ops[tail_index + 1..];
    let terminal = if raw_lr.is_some() {
        data.bblocks_ref().block(block).size_out() == 0
            && remainder.iter().all(|&op| {
                let o = data.obank().get(op).unwrap();
                o.code() == OpCode::CPUI_RETURN && o.get_addr() == &site
            })
    } else {
        remainder.is_empty()
    };
    ops.truncate(tail_index + 1);
    if data.bblocks_ref().block(block).size_in() < 2
        || !terminal
        || !cloneable(data, &ops[..tail_index])
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
    if let Some((address, size)) = raw_lr {
        copy_lr_ops(data, duplicate, &ops[..tail_index], None)?;
        let ret = data.new_op(1, site);
        let input = data.new_varnode(size, &address, None);
        data.op_set_opcode_code(ret, OpCode::CPUI_RETURN);
        data.op_set_input(ret, input, 0)?;
        data.op_insert_end(ret, duplicate);
    } else {
        copy_lr_ops(data, duplicate, &ops, Some(tail))?;
    }
    data.structure_reset();
    data.sort_call_specs();
    Ok(())
}

fn copy_lr_ops(
    data: &mut Funcdata,
    duplicate: BlockId,
    ops: &[OpId],
    return_tail: Option<OpId>,
) -> KunaResult<()> {
    let mut copied = BTreeMap::new();
    for &old in ops {
        let o = data.obank().get(old).expect("raw LR context op");
        let code = if Some(old) == return_tail {
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
    Ok(())
}
