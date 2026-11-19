//! Recover runtime stack allocation and bound its outgoing stack area over SSA.

use crate::action::{Action, ActionBase, ActionContext, ActionGroupList, ApplyResult};
use crate::context::{BlockId, OpId, VarnodeId};
use crate::dtype::{type_metatype, Datatype, TypeFactory};
use crate::funcdata::Funcdata;
use crate::kuna_fasthash::FxHashMap;
use crate::op::pcodeop_flags;
use kuna_base::address::{calc_mask, sign_extend, Address};
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;
use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

/// Internal allocation result is an address-sized integer, including in C.
pub const BUILTIN_STACK_ALLOC: u32 = crate::userop::BUILTIN_STACK_ALLOC;
const UNKNOWN: i128 = i128::MIN;
const SWEEPS: usize = 64;
const WIDENINGS: usize = 8;
type Bounds = BTreeMap<usize, i128>;
type Values = BTreeMap<VarnodeId, Bounds>;

fn strip(fd: &Funcdata, mut value: VarnodeId) -> VarnodeId {
    let mut seen = BTreeSet::new();
    while seen.insert(value) {
        let Some(op) = fd
            .vbank()
            .get(value)
            .and_then(|v| v.get_def())
            .and_then(|d| fd.obank().get(d))
        else {
            break;
        };
        if !matches!(op.code(), OpCode::CPUI_COPY | OpCode::CPUI_CAST) {
            break;
        }
        let Some(input) = op.get_in(0) else { break };
        if fd.vbank().get(input).unwrap().get_size() != fd.vbank().get(value).unwrap().get_size() { break; }
        value = input;
    }
    value
}
fn literal(fd: &Funcdata, value: VarnodeId, budget: &mut usize) -> Option<u64> {
    literal_using(fd, value, budget, &|value| (value, false))
}
fn literal_using<F: Fn(VarnodeId) -> (VarnodeId, bool)>(
    fd: &Funcdata, value: VarnodeId, budget: &mut usize, values: &F,
) -> Option<u64> {
    *budget = budget.checked_sub(1)?;
    let (value, opaque) = values(value);
    if opaque { return None; }
    let v = fd.vbank().get(value)?;
    let size = v.get_size();
    if !(1..=8).contains(&size) {
        return None;
    }
    if v.is_constant() {
        return Some(v.get_offset() & calc_mask(size));
    }
    let o = fd.obank().get(v.get_def()?)?;
    if !matches!(
        o.code(),
        OpCode::CPUI_COPY
            | OpCode::CPUI_CAST
            | OpCode::CPUI_INT_ADD
            | OpCode::CPUI_INT_SUB
            | OpCode::CPUI_INT_AND
            | OpCode::CPUI_INT_OR
            | OpCode::CPUI_INT_XOR
            | OpCode::CPUI_INT_NEGATE
            | OpCode::CPUI_INT_2COMP
            | OpCode::CPUI_INT_LEFT
            | OpCode::CPUI_INT_RIGHT
            | OpCode::CPUI_INT_SRIGHT
            | OpCode::CPUI_INT_ZEXT
            | OpCode::CPUI_INT_SEXT
    ) {
        return None;
    }
    let a = literal_using(fd, o.get_in(0)?, budget, values)?;
    if matches!(o.code(), OpCode::CPUI_COPY | OpCode::CPUI_CAST) {
        return Some(a & calc_mask(size));
    }
    let behavior = fd.get_arch().op_behavior(o.code())?;
    let input_size = fd.vbank().get(o.get_in(0)?)?.get_size();
    let result = if behavior.is_unary() {
        behavior.evaluate_unary(size, input_size, a).ok()?
    } else {
        let b = literal_using(fd, o.get_in(1)?, budget, values)?;
        behavior.evaluate_binary(size, input_size, a, b).ok()?
    };
    Some(result & calc_mask(size))
}
fn number_using<F: Fn(VarnodeId) -> (VarnodeId, bool)>(
    fd: &Funcdata, value: VarnodeId, values: &F,
) -> Option<i128> {
    let width = fd.vbank().get(value)?.get_size();
    let raw = literal_using(fd, value, &mut 32, values)?;
    Some(sign_extend(raw as i64, width * 8 - 1) as i128)
}

fn native_stack_sum_using<F: Fn(VarnodeId) -> (VarnodeId, bool)>(
    fd: &Funcdata, width: int4, mut todo: Vec<(VarnodeId, i128)>,
    mut pops: Option<&mut BTreeMap<OpId, int4>>, values: &F,
) -> Option<i128> {
    let mut budget = 256usize;
    let mut terms = BTreeMap::<VarnodeId, i128>::new();
    let mut constant = 0i128;
    while let Some((value, sign)) = todo.pop() {
        budget = budget.checked_sub(1)?;
        let (value, opaque) = values(value);
        if fd.vbank().get(value)?.get_size() != width {
            return None;
        }
        if let Some(number) = (!opaque).then(|| number_using(fd, value, values)).flatten() {
            constant = constant.checked_add(sign.checked_mul(number)?)?;
            continue;
        }
        let op = (!opaque).then(|| fd.vbank().get(value)?.get_def().and_then(|id| fd.obank().get(id))).flatten();
        if let Some(op) = op {
            match op.code() {
                OpCode::CPUI_COPY | OpCode::CPUI_CAST => {
                    todo.push((op.get_in(0)?, sign));
                    continue;
                }
                OpCode::CPUI_INT_OR if op.get_in(0) == op.get_in(1) => {
                    todo.push((op.get_in(0)?, sign));
                    continue;
                }
                OpCode::CPUI_INDIRECT => {
                    if let Some(pop) = indirect_stack_pop(fd, fd.vbank().get(value)?.get_def()?) {
                        if let Some(dependencies) = pops.as_deref_mut() {
                            let effect = fd.vbank().get(op.get_in(1)?)?;
                            let call = crate::funcdata_varnode::op_iop_decode(effect.get_offset());
                            dependencies.insert(call, pop as int4);
                        }
                        constant = constant.checked_add(sign.checked_mul(pop)?)?;
                        todo.push((op.get_in(0)?, sign));
                        continue;
                    }
                }
                OpCode::CPUI_INT_ADD | OpCode::CPUI_INT_SUB | OpCode::CPUI_PTRSUB => {
                    todo.push((op.get_in(0)?, sign));
                    todo.push((op.get_in(1)?, if op.code() == OpCode::CPUI_INT_SUB { sign.checked_neg()? } else { sign }));
                    continue;
                }
                OpCode::CPUI_PTRADD => {
                    if let Some(scale) = number_using(fd, op.get_in(2)?, values) {
                        todo.push((op.get_in(0)?, sign));
                        todo.push((op.get_in(1)?, sign.checked_mul(scale)?));
                        continue;
                    }
                }
                OpCode::CPUI_INT_2COMP => {
                    todo.push((op.get_in(0)?, sign.checked_neg()?));
                    continue;
                }
                OpCode::CPUI_INT_MULT => {
                    if let Some(scale) = number_using(fd, op.get_in(1)?, values) {
                        todo.push((op.get_in(0)?, sign.checked_mul(scale)?));
                        continue;
                    }
                    if let Some(scale) = number_using(fd, op.get_in(0)?, values) {
                        todo.push((op.get_in(1)?, sign.checked_mul(scale)?));
                        continue;
                    }
                }
                _ => {}
            }
        }
        let coefficient = terms.entry(value).or_default();
        *coefficient = coefficient.checked_add(sign)?;
    }
    if terms.values().any(|&coefficient| coefficient != 0) {
        return None;
    }
    Some(sign_extend((constant as u64 & calc_mask(width)) as i64, width * 8 - 1) as i128)
}
fn constant_step_using<F: Fn(VarnodeId) -> (VarnodeId,bool)>(
    fd: &Funcdata, value: VarnodeId, values: &F,
) -> Option<(VarnodeId,i128)> {
    let (value, opaque) = values(value);
    if opaque { return None; }
    let op = fd.obank().get(fd.vbank().get(value)?.get_def()?)?;
    if op.code() == OpCode::CPUI_INT_ADD {
        if let Some(amount) = number_using(fd, op.get_in(0)?, values) {
            return Some((op.get_in(1)?, amount));
        }
    }
    let amount = number_using(fd, op.get_in(1)?, values)?;
    match op.code() {
        OpCode::CPUI_INT_ADD | OpCode::CPUI_PTRSUB => Some((op.get_in(0)?, amount)),
        OpCode::CPUI_INT_SUB => Some((op.get_in(0)?, -amount)),
        OpCode::CPUI_PTRADD => Some((
            op.get_in(0)?,
            amount.checked_mul(number_using(fd, op.get_in(2)?, values)?)?,
        )),
        _ => None,
    }
}
fn alignment_using<F: Fn(VarnodeId) -> (VarnodeId,bool)>(
    fd: &Funcdata, op: OpId, values: &F,
) -> Option<(int4,u128)> {
    let o = fd.obank().get(op)?;
    if o.code() != OpCode::CPUI_INT_AND {
        return None;
    }
    (0..2).find_map(|slot| {
        let value = o.get_in(slot)?;
        let mask = literal_using(fd,value,&mut 32,values)?;
        if mask == 0 {
            return None;
        }
        let width = fd.vbank().get(value)?.get_size();
        let size = (!mask & calc_mask(width)) as u128 + 1;
        size.is_power_of_two().then_some((1 - slot, size))
    })
}
fn stack_upper_using<F: Fn(VarnodeId) -> (VarnodeId,bool)>(
    fd: &Funcdata,
    value: VarnodeId,
    anchor: VarnodeId,
    reservation: Option<&Allocation>,
    after: Option<&BTreeSet<BlockId>>,
    budget: &mut usize, values: &F,
) -> Option<i128> {
    let (value, opaque) = values(value);
    if opaque { return None; }
    let (anchor, opaque) = values(anchor);
    if opaque { return None; }
    #[derive(Clone, Copy)]
    enum Step {
        Seed(i128),
        Offset(VarnodeId, i128),
        Phi(OpId),
    }
    struct Node {
        step: Step,
        upper: Option<i128>,
    }
    let live_input = |id: OpId, slot: int4| {
        after.is_none_or(|reachable| {
            let parent = fd.obank().get(id).unwrap().get_parent().unwrap();
            reachable.contains(&fd.bblocks_ref().block(parent).get_in(slot))
        })
    };
    let width = fd.vbank().get(anchor).unwrap().get_size();
    let mut indices = BTreeMap::new();
    let mut nodes = Vec::<Node>::new();
    let mut todo = vec![value];
    while let Some(value) = todo.pop() {
        let (value, opaque) = values(value);
        if opaque { return None; }
        if indices.contains_key(&value) {
            continue;
        }
        let Some(left) = budget.checked_sub(1) else {
            return None;
        };
        *budget = left;
        if nodes.len() == 256 || fd.vbank().get(value).is_none_or(|v| v.get_size() != width) {
            return None;
        }
        let step = if value == anchor {
            Step::Seed(0)
        } else {
            let Some(id) = fd.vbank().get(value).and_then(|v| v.get_def()) else {
                return None;
            };
            let op = fd.obank().get(id).unwrap();
            let mut lookup = None;
            let displacement = reservation.and_then(|root| {
                let candidate = *lookup.insert(allocation(fd, id));
                let candidate = candidate?;
                dominates_op(fd, root.op, id)
                    .then(|| origin_delta(fd, root, &candidate))
                    .flatten()
            });
            if let Some(delta) = displacement {
                Step::Seed(delta)
            } else if let Some((input, delta)) = constant_step_using(fd,value,values) {
                Step::Offset(input, delta)
            } else {
                match op.code() {
                    OpCode::CPUI_COPY | OpCode::CPUI_CAST => {
                        let Some(input) = op.get_in(0) else {
                            return None;
                        };
                        Step::Offset(input, 0)
                    }
                    OpCode::CPUI_INDIRECT => {
                        let Some(input) = op.get_in(0) else {
                            return None;
                        };
                        let Some(pop) = indirect_stack_pop(fd, id) else {
                            return None;
                        };
                        Step::Offset(input, pop)
                    }
                    OpCode::CPUI_MULTIEQUAL => {
                        if after.is_some()
                            && op.get_parent().is_none_or(|parent| {
                                fd.bblocks_ref().block(parent).size_in() != op.num_input()
                            })
                        {
                            return None;
                        }
                        Step::Phi(id)
                    }
                    OpCode::CPUI_INT_AND => {
                        let Some(input) =
                            alignment_using(fd,id,values).and_then(|(slot, _)| op.get_in(slot))
                        else {
                            return None;
                        };
                        Step::Offset(input, 0)
                    }
                    _ => {
                        let Some(candidate) = lookup.unwrap_or_else(|| allocation(fd, id)) else {
                            return None;
                        };
                        // SUB by a negated size restores SP upwards.
                        if negate(fd, candidate.amount).is_some() {
                            return None;
                        }
                        Step::Offset(candidate.base, 0)
                    }
                }
            }
        };
        let step = match step {
            Step::Offset(input,delta) => {
                let (input,opaque) = values(input);
                if opaque { return None; }
                Step::Offset(input,delta)
            }
            other => other,
        };
        indices.insert(value, nodes.len());
        nodes.push(Node { step, upper: None });
        match step {
            Step::Seed(_) => {}
            Step::Offset(input, _) => todo.push(input),
            Step::Phi(id) => {
                let op = fd.obank().get(id).unwrap();
                for slot in 0..op.num_input() {
                    if !live_input(id, slot) {
                        continue;
                    }
                    let Some(input) = op.get_in(slot) else {
                        return None;
                    };
                    todo.push(input);
                }
            }
        }
    }
    // Bottom is temporary: every relevant ancestor must become anchored.
    for _ in 0..SWEEPS {
        let mut changed = false;
        for index in 0..nodes.len() {
            let Some(left) = budget.checked_sub(1) else {
                return None;
            };
            *budget = left;
            let upper = match nodes[index].step {
                Step::Seed(delta) => Some(delta),
                Step::Offset(input, delta) => match nodes[indices[&input]].upper {
                    Some(value) => {
                        let Some(value) = value.checked_add(delta) else {
                            return None;
                        };
                        Some(value)
                    }
                    None => None,
                },
                Step::Phi(id) => {
                    let op = fd.obank().get(id).unwrap();
                    (0..op.num_input())
                        .filter(|&slot| live_input(id, slot))
                        .filter_map(|slot| nodes[indices[&values(op.get_in(slot).unwrap()).0]].upper)
                        .max()
                }
            };
            if nodes[index].upper != upper {
                nodes[index].upper = upper;
                changed = true;
            }
        }
        if !changed {
            return if nodes.iter().all(|node| node.upper.is_some()) {
                nodes[indices[&value]].upper
            } else { None };
        }
    }
    None
}
fn number(fd: &Funcdata, value: VarnodeId) -> Option<i128> {
    let width = fd.vbank().get(value)?.get_size();
    let raw = literal(fd, value, &mut 32)?;
    Some(sign_extend(raw as i64, width * 8 - 1) as i128)
}
fn negate(fd: &Funcdata, value: VarnodeId) -> Option<(VarnodeId, i128)> {
    let width = fd.vbank().get(value)?.get_size();
    if !matches!(width, 2 | 4 | 8) { return None; }
    let mut value = carried(fd, value);
    let mut factor = 1_i128;
    for _ in 0..64 {
        let node = fd.vbank().get(value)?;
        if node.get_size() != width { return None; }
        let operation = node.get_def().and_then(|id| fd.obank().get(id));
        let step = operation.and_then(|op| match op.code() {
            OpCode::CPUI_INT_2COMP => Some((op.get_in(0)?, -1)),
            OpCode::CPUI_INT_MULT => {
                if let Some(constant) = number(fd, op.get_in(1)?) { Some((op.get_in(0)?, constant)) }
                else { Some((op.get_in(1)?, number(fd, op.get_in(0)?)?)) }
            }
            OpCode::CPUI_INT_SUB if number(fd, op.get_in(0)?) == Some(0) => Some((op.get_in(1)?, -1)),
            _ => None,
        });
        if let Some((input, coefficient)) = step {
            factor = factor.checked_mul(coefficient)?;
            value = strip(fd, input);
            continue;
        }
        if constant_step(fd, value).is_some() {
            let (base, offset) = canonical_base(fd, value, &mut BTreeSet::new());
            if base != value && offset == 0 { value = strip(fd, base); continue; }
        }
        return (factor < 0 && factor > -(1_i128 << (8 * width - 1))).then_some((value, -factor));
    }
    None
}
fn stack_register(fd: &Funcdata, value: VarnodeId) -> bool {
    let Some(stack) = fd.get_arch().manage().get_stack_space() else {
        return false;
    };
    let Ok(register) = stack.get_spacebase(0) else {
        return false;
    };
    fd.vbank().get(value).is_some_and(|v| {
        v.get_size() == register.size as int4
            && v.get_offset() == register.offset
            && register
                .space
                .as_ref()
                .is_some_and(|s| std::rc::Rc::ptr_eq(s, v.get_space()))
    })
}
fn constant_step(fd: &Funcdata, value: VarnodeId) -> Option<(VarnodeId,i128)> {
    constant_step_using(fd,value,&|value|(value,false))
}

// Equal steps and predecessor-aligned PHIs preserve p - n. This recognizes
// pointer/counter probe loops, including their zero-iteration exit PHIs, without
// removing the loop or asserting a bound on its iteration count.
/// Follow copies and loop-head PHIs whose backedges preserve the input.
/// An INDIRECT is an effect, not an identity.
fn carried(fd: &Funcdata, mut value: VarnodeId) -> VarnodeId {
    for _ in 0..16 {
        value = strip(fd, value);
        let Some(op) = fd.vbank().get(value).and_then(|v| v.get_def()).and_then(|id| fd.obank().get(id)) else { break };
        let next = match op.code() {
            OpCode::CPUI_COPY => op.get_in(0),
            OpCode::CPUI_INDIRECT => preserved_carried_input(fd, value),
            // A loop-head phi whose other inputs carry back to itself is the
            // value it enters with.
            OpCode::CPUI_MULTIEQUAL => {
                let mut entry = None;
                for slot in 0..op.num_input() {
                    let Some(input) = op.get_in(slot) else { return value };
                    let input = carried_step(fd, input, value);
                    if input == value { continue; }
                    match entry { None => entry = Some(input), Some(seen) if seen == input => {}, Some(_) => return value }
                }
                entry
            }
            _ => None,
        };
        let Some(next) = next else { break };
        value = next;
    }
    value
}

/// Follow value-preserving transitions, stopping at the loop-head PHI.
fn carried_step(fd: &Funcdata, mut value: VarnodeId, phi: VarnodeId) -> VarnodeId {
    for _ in 0..16 {
        value = strip(fd, value);
        if value == phi { break; }
        let Some(op) = fd.vbank().get(value).and_then(|v| v.get_def()).and_then(|id| fd.obank().get(id)) else { break };
        let next = match op.code() {
            OpCode::CPUI_COPY => op.get_in(0),
            OpCode::CPUI_INDIRECT => preserved_carried_input(fd, value),
            _ => None,
        };
        let Some(next) = next else { break };
        value = next;
    }
    value
}

thread_local! {
    static CARRY_PROOFS: std::cell::RefCell<Option<FxHashMap<VarnodeId, Option<VarnodeId>>>> =
        const { std::cell::RefCell::new(None) };
}

/// Reuse independent carry proofs only while the function's IR is immutable.
fn with_carry_proofs<T>(fd: &Funcdata, query: impl FnOnce(&Funcdata) -> T) -> T {
    struct Reset;
    impl Drop for Reset {
        fn drop(&mut self) {
            CARRY_PROOFS.with(|cache| { cache.borrow_mut().take(); });
        }
    }
    CARRY_PROOFS.with(|cache| {
        let mut cache = cache.borrow_mut();
        assert!(cache.is_none());
        *cache = Some(FxHashMap::default());
    });
    let _reset = Reset;
    query(fd)
}

fn preserved_carried_input(fd: &Funcdata, value: VarnodeId) -> Option<VarnodeId> {
    let stack = fd.get_arch().manage().get_stack_space()?;
    let node = fd.vbank().get(value)?;
    if node.is_volatile() || node.get_space().get_index() != stack.get_index() { return None; }
    let id = node.get_def()?;
    let op = fd.obank().get(id)?;
    if op.is_indirect_creation() { return None; }
    let source = op.get_in(0)?;
    let first = sign_extend(node.get_offset() as i64, stack.get_addr_size() as int4 * 8 - 1) as i128;
    // A nested address query must not assume the carry transition it is proving.
    thread_local! { static PROVING_CARRY: std::cell::Cell<bool> = const { std::cell::Cell::new(false) }; }
    PROVING_CARRY.with(|active| {
        if active.replace(true) { return None; }
        struct Reset<'a>(&'a std::cell::Cell<bool>);
        impl Drop for Reset<'_> {
            fn drop(&mut self) { self.0.set(false); }
        }
        let _reset = Reset(active);
        if let Some(result) = CARRY_PROOFS.with(|cache| cache.borrow().as_ref()?.get(&value).copied()) {
            return result;
        }
        let result = (|| {
            let effect = frame_word_effect(fd, id)?;
            private_frame_slot(fd, first, node.get_size(), FrameRead::Operation(effect),
                Some(id), Some(source), &mut 4096, &mut SavedFrameQuery::default())?;
            Some(source)
        })();
        CARRY_PROOFS.with(|cache| {
            if let Some(cache) = cache.borrow_mut().as_mut() {
                cache.insert(value, result);
            }
        });
        result
    })
}

/// `(-y) & -m` for a power of two `m` is `-((y + m - 1) & -m)`: a reservation
/// rounded to a larger alignment (`alloca_with_align`, a realigned VLA) is the
/// negation of a rounded-up size. Rewriting the mask that feeds a stack-pointer
/// update in that form exposes the size to allocation recognition.
/// Returns whether any reservation was rewritten.
fn unmask_reservations(fd: &mut Funcdata) -> bool {
    let masks: Vec<(OpId, VarnodeId, int4, i128)> = fd.obank().iter_alive().filter_map(|id| {
        let op = fd.obank().get(id)?;
        if !op.get_out().is_some_and(|o| stack_register(fd, o)) { return None; }
        let op = if op.code() == OpCode::CPUI_COPY {
            fd.vbank().get(op.get_in(0)?)?.get_def().and_then(|d| fd.obank().get(d))?
        } else { op };
        if !matches!(op.code(), OpCode::CPUI_INT_ADD | OpCode::CPUI_PTRADD) { return None; }
        (0..2).find_map(|slot| {
            let masked = strip(fd, op.get_in(slot)?);
            let and_id = fd.vbank().get(masked)?.get_def()?;
            let and = fd.obank().get(and_id)?;
            if and.code() != OpCode::CPUI_INT_AND { return None; }
            let width = fd.vbank().get(masked)?.get_size();
            let (value, mask) = match (number(fd, and.get_in(1)?), number(fd, and.get_in(0)?)) {
                (Some(m), _) => (and.get_in(0)?, m),
                (None, Some(m)) => (and.get_in(1)?, m),
                _ => return None,
            };
            let m = mask.checked_neg()?;
            if m < 2 || m & (m - 1) != 0 { return None; }
            let (size, scale) = negate(fd, value)?;
            (scale == 1 && fd.vbank().get(size)?.get_size() == width).then_some((and_id, size, width, m))
        })
    }).collect();
    let rewritten = !masks.is_empty();
    for (and_id, size, width, m) in masks {
        let address = fd.obank().get(and_id).unwrap().get_addr().clone();
        let round = fd.new_op(2, address.clone());
        fd.op_set_opcode_code(round, OpCode::CPUI_INT_ADD);
        let bias = fd.new_constant(width, (m - 1) as u64);
        fd.op_set_all_input(round, &[size, bias]).unwrap();
        let rounded = fd.new_unique_out(width, round).unwrap();
        fd.op_insert_before(round, and_id);
        let trunc = fd.new_op(2, address);
        fd.op_set_opcode_code(trunc, OpCode::CPUI_INT_AND);
        let mask = fd.new_constant(width, (-m) as u64 & if width == 8 { u64::MAX } else { (1u64 << (8 * width)) - 1 });
        fd.op_set_all_input(trunc, &[rounded, mask]).unwrap();
        let aligned = fd.new_unique_out(width, trunc).unwrap();
        fd.op_insert_before(trunc, and_id);
        fd.op_remove_input(and_id, 1);
        fd.op_set_opcode_code(and_id, OpCode::CPUI_INT_2COMP);
        fd.op_set_input(and_id, aligned, 0).unwrap();
    }
    rewritten
}
/// Reservations whose copies into SP were folded away: copy propagation
/// rewrites a later SP update (an outgoing-argument adjustment) as the entry
/// frame plus every reserved amount, and the intermediate SP copies die. An
/// allocation earlier in the block whose amount node is one of that update's
/// terms, with no call or branch between, is the reservation it witnesses.
fn folded_reservations(
    fd: &Funcdata,
    write: OpId,
    out: VarnodeId,
    lookup: &mut impl FnMut(OpId) -> Option<Allocation>,
) -> Vec<Allocation> {
    let mut terms = BTreeSet::new();
    let mut todo = vec![out];
    let mut budget = 64;
    while let Some(value) = todo.pop() {
        budget -= 1;
        if budget == 0 { return Vec::new(); }
        let Some(op) = fd.vbank().get(strip(fd, value)).and_then(|node| node.get_def()).and_then(|id| fd.obank().get(id)) else { continue };
        match op.code() {
            OpCode::CPUI_INT_ADD => {
                // A constant-stepped SP base is walked on rather than negated:
                // negating it rescans the whole SP chain at each step.
                for slot in 0..2 {
                    let Some(input) = op.get_in(slot) else { continue };
                    if number(fd, input).is_some() { continue; }
                    if constant_step(fd, input).is_none() {
                        if let Some((amount, _)) = negate(fd, input) { terms.insert(amount); continue; }
                    }
                    todo.push(input);
                }
            }
            OpCode::CPUI_INT_SUB => {
                if let Some(input) = op.get_in(1) { terms.insert(strip(fd, input)); }
                if let Some(input) = op.get_in(0) { todo.push(input); }
            }
            // An aligned reservation masks the reduced pointer.
            OpCode::CPUI_COPY => { if let Some(input) = op.get_in(0) { todo.push(input); } }
            // An aligned reservation masks the reduced pointer by a constant.
            OpCode::CPUI_INT_AND if op.get_in(1).and_then(|v| number(fd, v)).is_some_and(|mask| mask < 0) => {
                if let Some(input) = op.get_in(0) { todo.push(input); }
            }
            OpCode::CPUI_INT_OR | OpCode::CPUI_INT_XOR if op.get_in(1).and_then(|v| number(fd, v)) == Some(0) => {
                if let Some(input) = op.get_in(0) { todo.push(input); }
            }
            _ => {}
        }
    }
    if terms.len() < 1 { return Vec::new(); }
    let Some(block) = fd.obank().get(write).and_then(|op| op.get_parent()) else { return Vec::new() };
    let mut found = Vec::new();
    let mut previous = fd.obank().get(write).and_then(|op| op.basic_neighbours().0);
    while let Some(id) = previous {
        let Some(op) = fd.obank().get(id) else { break };
        if op.get_parent() != Some(block) || matches!(op.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND
            | OpCode::CPUI_CALLOTHER | OpCode::CPUI_RETURN | OpCode::CPUI_BRANCH | OpCode::CPUI_CBRANCH | OpCode::CPUI_BRANCHIND) { break; }
        previous = op.basic_neighbours().0;
        if let Some(candidate) = lookup(id) {
            if terms.contains(&candidate.amount) || negate(fd, candidate.amount).is_none() && terms.iter().any(|&term| strip(fd, term) == strip(fd, candidate.amount)) {
                found.push(candidate);
            }
        }
    }
    found
}

fn paired_origins(
    fd: &Funcdata,
    p: VarnodeId,
    n: VarnodeId,
    scale: i128,
    seen: &mut BTreeSet<(VarnodeId, VarnodeId)>,
) -> Option<BTreeSet<(VarnodeId, VarnodeId)>> {
    let p = carried(fd, p);
    let n = carried(fd, n);
    if !seen.insert((p, n)) {
        return Some(BTreeSet::new());
    }
    if let (Some((a, da)), Some((b, db))) = (constant_step(fd, p), constant_step(fd, n)) {
        if Some(da) == db.checked_mul(scale) {
            return paired_origins(fd, a, b, scale, seen);
        }
    }
    let phi = |v| {
        fd.vbank()
            .get(v)
            .and_then(|v| v.get_def())
            .and_then(|d| fd.obank().get(d))
    };
    if let (Some(a), Some(b)) = (phi(p), phi(n)) {
        if a.code() == OpCode::CPUI_MULTIEQUAL
            && b.code() == OpCode::CPUI_MULTIEQUAL
            && a.get_parent() == b.get_parent()
            && a.num_input() == b.num_input()
        {
            let mut origins = BTreeSet::new();
            for slot in 0..a.num_input() {
                origins.extend(paired_origins(fd, a.get_in(slot)?, b.get_in(slot)?, scale, seen)?);
            }
            return Some(origins);
        }
    }
    // Discovery starts at physical stack-pointer definitions. The opt-in
    // allocation model also covers a base restored through memory; requiring
    // its complete provenance would miss ordinary saved-frame allocations.
    Some(BTreeSet::from([(p, n)]))
}
/// A base carried around a loop by a nonzero constant step has moved by an
/// unbounded amount, as in a stack-clash probe walking the stack pointer down a
/// page at a time; the residual subtraction is then not the whole buffer.
fn stepped_cycle(fd: &Funcdata, base: VarnodeId) -> bool {
    fn walk(
        fd: &Funcdata,
        value: VarnodeId,
        at: i128,
        active: &mut BTreeMap<VarnodeId, (usize, i128)>,
        done: &mut BTreeSet<VarnodeId>,
        pending: &mut Vec<VarnodeId>,
    ) -> Option<usize> {
        let value = strip(fd, value);
        if let Some(&(index, entered)) = active.get(&value) {
            return (entered == at).then_some(index);
        }
        if !done.insert(value) {
            return Some(usize::MAX);
        }
        if active.len() >= 256 {
            return None;
        }
        let index = done.len() - 1;
        active.insert(value, (index, at));
        pending.push(value);
        let mut next = Vec::new();
        if let Some((origin, step)) = constant_step(fd, value) {
            next.push((origin, step));
        } else if let Some(phi) = fd
            .vbank()
            .get(value)
            .and_then(|v| v.get_def())
            .and_then(|d| fd.obank().get(d))
            .filter(|o| o.code() == OpCode::CPUI_MULTIEQUAL)
        {
            next.extend((0..phi.num_input()).filter_map(|slot| Some((phi.get_in(slot)?, 0))));
        }
        // Keep a completed branch active until its whole strongly connected
        // component closes. A later PHI input can re-enter that branch with a
        // different displacement even after a zero-step cycle visited it.
        let mut low = index;
        for (origin, step) in next {
            let to = at.checked_add(step)?;
            low = low.min(walk(fd, origin, to, active, done, pending)?);
        }
        if low == index {
            while let Some(member) = pending.pop() {
                active.remove(&member);
                if member == value {
                    break;
                }
            }
        }
        Some(low)
    }
    walk(
        fd,
        base,
        0,
        &mut BTreeMap::new(),
        &mut BTreeSet::new(),
        &mut Vec::new(),
    )
    .is_none()
}
#[derive(Clone, Copy)]
struct Allocation {
    op: OpId,
    output: VarnodeId,
    amount: VarnodeId,
    scale: i128,
    base: VarnodeId,
    // Actual pre-reservation SP when witnessed; the algebraic base otherwise.
    before: VarnodeId,
}

impl Allocation {
    // The physical reservation can extend below a source object whose
    // algebraic base is above the old SP. Neither region ends at `before`
    // in every case; use their higher boundary for disjointness proofs.
    fn upper_base(&self, fd: &Funcdata) -> VarnodeId {
        if self.before == self.base
            || native_stack_delta(fd, self.before, self.base).is_some_and(|prefix| prefix >= 0)
        {
            self.before
        } else {
            self.base
        }
    }
}
fn allocation(fd: &Funcdata, op: OpId) -> Option<Allocation> {
    let o = fd.obank().get(op)?;
    let (base, amount, scale) = match o.code() {
        OpCode::CPUI_INT_SUB if number(fd, o.get_in(1)?).is_none() => (o.get_in(0)?, o.get_in(1)?, 1),
        // A constant adjustment reserves no runtime amount.
        OpCode::CPUI_INT_ADD if (0..2).any(|slot| o.get_in(slot).and_then(|v| number(fd, v)).is_some()) => return None,
        OpCode::CPUI_INT_ADD => {
            if let Some((amount, scale)) = negate(fd, o.get_in(1)?) {
                (o.get_in(0)?, amount, scale)
            } else {
                { let (amount, scale) = negate(fd, o.get_in(0)?)?; (o.get_in(1)?, amount, scale) }
            }
        }
        OpCode::CPUI_PTRADD if number(fd, o.get_in(2)?) == Some(1) => {
            { let (amount, scale) = negate(fd, o.get_in(1)?)?; (o.get_in(0)?, amount, scale) }
        }
        _ => return None,
    };
    if number(fd, amount).is_some() {
        return None;
    }
    let origins = paired_origins(fd, base, amount, scale, &mut BTreeSet::new())?;
    // If the live operands vary together but have no unique entry pair,
    // keep the subtraction at its actual execution point. Do not hoist it
    // to an arbitrary predecessor.
    let (base, amount) = if origins.len() == 1 {
        origins.into_iter().next()?
    } else if origins.len() > 1 && stack_register(fd, base)
        && fd.vbank().get(base).and_then(|node| node.get_def()).and_then(|id| fd.obank().get(id))
            .is_some_and(|op| op.code() == OpCode::CPUI_MULTIEQUAL)
    {
        (base, amount)
    } else {
        return None;
    };
    if !runtime_amount(fd, amount) || stepped_cycle(fd, base) {
        return None;
    }
    Some(Allocation { before: base,
        op,
        output: o.get_out()?,
        amount,
        scale,
        base,
    })
}
fn has_runtime_stack_reservation(fd: &Funcdata) -> bool {
    let mut seen = BTreeSet::new();
    let mut tested = FxHashMap::default();
    let mut lookup = |id| *tested.entry(id).or_insert_with(|| allocation(fd, id));
    fd.obank().iter_alive().any(|id| {
        let Some(out) = fd.obank().get(id).and_then(|op| op.get_out()).filter(|&out| stack_register(fd, out))
        else { return false };
        let mut found = false;
        visit_candidates(fd, out, &mut |v| seen.insert(v), &mut lookup, &mut |_| found = true);
        found || !folded_reservations(fd, id, out, &mut lookup).is_empty()
    })
}

/// Does `op` reserve a runtime amount below its stack base? An address formed
/// from its result lies in the reservation or below it, never in the static
/// frame above, so the stack alias gather does not extend the frame's alias
/// boundary through it.
pub(crate) fn reserves_runtime_amount(fd: &Funcdata, op: OpId) -> bool {
    if !fd.stack_alloc_on() { return false; }
    let Some(stack) = fd.get_arch().manage().get_stack_space() else { return false };
    if !stack.stack_grows_negative() { return false; }
    if !fd.stack_alloc_settled() && fd.kuna_stack_scope_holds.is_empty()
        && fd.find_spacebase_input(stack).is_none()
    {
        return false;
    }
    let reserves = allocation(fd, op).is_some() || {
        // Copy propagation rewrites a pointer below the reservation as the static
        // base plus the negated amount, off the reservation op itself.
        fd.obank().get(op).is_some_and(|o| match o.code() {
            OpCode::CPUI_INT_SUB => o.get_in(1).is_some_and(|amount| runtime_amount(fd, amount)),
            OpCode::CPUI_INT_ADD | OpCode::CPUI_PTRADD => (0..2).any(|slot| o.get_in(slot)
                .and_then(|value| negate(fd, value))
                .is_some_and(|(amount, scale)| scale > 0 && runtime_amount(fd, amount))),
            _ => false,
        })
    };
    if reserves && !fd.stack_alloc_settled() && fd.kuna_stack_scope_holds.is_empty() && !has_runtime_stack_reservation(fd) {
        return false;
    }
    if reserves { fd.touch_stack_alloc(); }
    reserves
}

/// `value` as a base plus a constant, through copies and constant adds.
fn affine_parts(fd: &Funcdata, mut value: VarnodeId) -> (VarnodeId, i128) {
    let mut offset = 0_i128;
    for _ in 0..16 {
        let Some(op) = fd.vbank().get(value).and_then(|v| v.get_def()).and_then(|id| fd.obank().get(id)) else { break };
        let next = match op.code() {
            OpCode::CPUI_COPY => op.get_in(0),
            OpCode::CPUI_INT_ADD => match (op.get_in(0), op.get_in(1)) {
                (Some(a), Some(b)) if number(fd, b).is_some() => { offset += number(fd, b).unwrap(); Some(a) }
                (Some(a), Some(b)) if number(fd, a).is_some() => { offset += number(fd, a).unwrap(); Some(b) }
                _ => None,
            },
            _ => None,
        };
        let Some(next) = next else { break };
        value = next;
    }
    (value, offset)
}

/// A call made with an allocation live passes its stack arguments through the
/// lowered storage, where input recovery cannot see them: past the reservation
/// the call has no stack offset, so heritage creates no trials. A variadic
/// callee whose last named argument is read from that storage takes the words
/// stored right above it in the call's block, contiguously, as its variadic
/// arguments. When no argument is on the stack yet, the call's prototype model
/// places the next variadic argument: an ABI that sends variadic arguments to
/// the stack (Apple arm64) names the slot at the call's stack pointer.
fn bind_variadic_tails(fd: &mut Funcdata, shifts: &[(VarnodeId, u64)]) {
    let calls: Vec<OpId> = (0..fd.num_calls()).map(|i| fd.get_call_specs(i).get_op()).collect();
    for call in calls {
        let Some(index) = fd.get_call_specs_index(call) else { continue };
        if !fd.get_call_specs(index).proto().is_dotdotdot() { continue; }
        let Some(op) = fd.obank().get(call) else { continue };
        if op.is_dead() || op.num_input() < 2 { continue; }
        let Some(block) = op.get_parent() else { continue };
        let ops = fd.bb_ops(block);
        let Some(position) = ops.iter().position(|&id| id == call) else { continue };
        // Arguments are stored after the previous call in the block.
        let first = ops[..position].iter().rposition(|&id| fd.obank().get(id)
            .is_some_and(|o| matches!(o.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND))).map_or(0, |i| i + 1);
        // The latest write to the slot in the span must be one whole STORE
        // starting at it: a later overlapping write leaves no single value.
        let stored = |fd: &Funcdata, space: Option<i128>, at: (VarnodeId, i128), slot: int4, exact: bool| {
            for &id in ops[first..position].iter().rev() {
                let Some(s) = fd.obank().get(id).filter(|s| s.code() == OpCode::CPUI_STORE && !s.is_dead()) else { continue };
                let Some((base, offset)) = s.get_in(1).map(|p| affine_parts(fd, p)) else { continue };
                let size = s.get_in(2).and_then(|v| fd.vbank().get(v)).map_or(0, |v| v.get_size());
                if base != at.0 || offset >= at.1 + slot as i128 || offset + size as i128 <= at.1 { continue; }
                let whole = offset == at.1 && if exact { size == slot } else { size <= slot }
                    && space.is_none_or(|space| s.get_in(0).and_then(|v| number(fd, v)) == Some(space));
                return whole.then_some((id, size));
            }
            None
        };
        let last = op.get_in(op.num_input() - 1).unwrap();
        let sp = fd.kuna_stack_call_sp.get(&call).copied().flatten().map(|sp| affine_parts(fd, sp));
        // Only a named argument read from the call's own outgoing storage
        // anchors a tail above it.
        let load = fd.vbank().get(last).and_then(|v| v.get_def()).and_then(|id| fd.obank().get(id))
            .filter(|l| l.code() == OpCode::CPUI_LOAD).and_then(|l| Some((l.get_in(0)?, l.get_in(1)?)))
            .filter(|&(_, pointer)| sp.is_some_and(|(base, _)| affine_parts(fd, pointer).0 == base));
        let mut slots = Vec::new();
        if let Some((space, pointer)) = load {
            let width = fd.vbank().get(last).map_or(0, |v| v.get_size());
            let (base, start) = affine_parts(fd, pointer);
            let space_id = number(fd, space);
            let mut slot = start + width as i128;
            while let Some((store, _)) = stored(fd, space_id, (base, slot), width, true) {
                slots.push((store, width));
                slot += width as i128;
            }
        } else if let Some((base, at)) = sp {
            // The outgoing argument area is the compiler backing below the
            // recovered object; a word above it belongs to the object.
            // Lowering left each object pointer as `backing + shift`.
            let Some(&(_, shift)) = shifts.iter().find(|&&(object, _)| fd.vbank().get(object).and_then(|v| v.get_def())
                .and_then(|id| fd.obank().get(id)).is_some_and(|o| o.code() == OpCode::CPUI_INT_ADD && o.get_in(0) == Some(base)))
            else { continue };
            let mut space_id = None;
            // A slot is the model's stride; a promoted int fills only part of it.
            for (offset, width) in variadic_stack_slots(fd, index, op.num_input() as usize - 1) {
                if at + offset + width as i128 > shift as i128 { break; }
                let Some((store, size)) = stored(fd, space_id, (base, at + offset), width, false) else { break };
                space_id = fd.obank().get(store).unwrap().get_in(0).and_then(|v| number(fd, v));
                slots.push((store, size));
            }
        }
        for (store, width) in slots {
            let Some((space, address)) = fd.obank().get(store).and_then(|s| Some((s.get_in(0)?, s.get_in(1)?))) else { break };
            let pc = fd.obank().get(call).unwrap().get_addr().clone();
            let read = fd.new_op(2, pc);
            fd.op_set_opcode_code(read, OpCode::CPUI_LOAD);
            if fd.op_set_input(read, space, 0).is_err() || fd.op_set_input(read, address, 1).is_err() { break; }
            let Ok(value) = fd.new_unique_out(width, read) else { break };
            fd.op_insert_before(read, call);
            let inputs = fd.obank().get(call).unwrap().num_input();
            if fd.op_insert_input(call, value, inputs).is_err() { break; }
        }
    }
}

/// Where the call's variadic arguments from position `from` on sit, as
/// `(offset, size)` from the call's stack pointer. A call that already fills
/// the model's register slots continues on the stack where the model says. A
/// call short of them takes its tail from the stack's first word only on a
/// target that passes every variadic argument on the stack (Apple arm64, by
/// image family: the generic AArch64 model a Mach-O loads with still says
/// registers), and only when that model's stack area begins at the stack
/// pointer. The model's stack offsets are from the stack pointer at the call
/// operation, after any return-address push, so a return address or home area
/// in front of the stack arguments is already part of them.
fn variadic_stack_slots(fd: &Funcdata, index: int4, from: usize) -> Vec<(i128, int4)> {
    const LIMIT: usize = 32;
    let arch = fd.get_arch();
    let proto = fd.get_call_specs(index).proto();
    let model = Rc::clone(proto.model());
    let Some(stack) = arch.manage().get_stack_space() else { return Vec::new() };
    let stack_base = model.input().get_entry().iter()
        .filter(|e| Rc::ptr_eq(e.get_space(), stack)).map(|e| e.get_base()).min();
    if stack_base.is_none() { return Vec::new(); }
    let mut pieces = crate::p4_calls::fspec::PrototypePieces::default();
    proto.get_pieces(&mut pieces);
    let Ok(named) = usize::try_from(pieces.first_var_arg_slot) else { return Vec::new() };
    if from < named || pieces.intypes.len() != named { return Vec::new(); }
    let Some(types) = arch.types() else { return Vec::new() };
    let size = types.get_size_of_pointer();
    let Ok(word) = types.get_base(size, type_metatype::TYPE_UINT) else { return Vec::new() };
    while pieces.intypes.len() < from + LIMIT {
        pieces.intypes.push(Rc::clone(&word));
        pieces.innames.push(String::new());
    }
    let mut storage = Vec::new();
    if model.assign_parameter_storage(&pieces, &mut storage, true, types, arch.manage()).is_err() { return Vec::new(); }
    let on_stack = |piece: &crate::p4_calls::fspec::ParameterPieces|
        piece.addr.get_space().is_some_and(|space| Rc::ptr_eq(space, stack));
    let Some(first_stack) = storage.iter().skip(1).position(|piece| on_stack(piece)) else { return Vec::new() };
    if from < first_stack {
        // Short of the register slots: only a target whose every variadic
        // argument goes to the stack (Apple arm64; the loader records the
        // image family) lays the tail from the stack's first word, and only
        // when no variadic argument was recovered in a register.
        if !arch.stack_alloc_stacked_varargs || from != named || stack_base != Some(0) { return Vec::new(); }
        return (0..LIMIT as i128).map(|k| (k * size as i128, size)).collect();
    }
    let bits = stack.get_addr_size() as i32 * 8;
    storage.iter().skip(1 + from)
        .take_while(|piece| on_stack(piece))
        .map(|piece| (sign_extend(piece.addr.get_offset() as i64, bits - 1) as i128, size))
        .collect()
}

fn candidates(
    fd: &Funcdata,
    value: VarnodeId,
    seen: &mut BTreeSet<VarnodeId>,
    result: &mut impl FnMut(Allocation),
) {
    visit_candidates(fd, value, &mut |v| seen.insert(v), &mut |id| allocation(fd, id), result);
}

fn visit_candidates(
    fd: &Funcdata,
    value: VarnodeId,
    first_visit: &mut impl FnMut(VarnodeId) -> bool,
    lookup: &mut impl FnMut(OpId) -> Option<Allocation>,
    result: &mut impl FnMut(Allocation),
) {
    if !first_visit(value) {
        return;
    }
    let Some(id) = fd.vbank().get(value).and_then(|v| v.get_def()) else {
        return;
    };
    if let Some(found) = lookup(id) {
        result(found);
        return;
    }
    let Some(op) = fd.obank().get(id) else { return };
    match op.code() {
        OpCode::CPUI_COPY | OpCode::CPUI_CAST | OpCode::CPUI_INDIRECT => {
            if let Some(v) = op.get_in(0) {
                visit_candidates(fd, v, first_visit, lookup, result);
            }
        }

        OpCode::CPUI_INT_AND | OpCode::CPUI_MULTIEQUAL => {
            for slot in 0..op.num_input() {
                if let Some(v) = op.get_in(slot) {
                    visit_candidates(fd, v, first_visit, lookup, result);
                }
            }
        }
        OpCode::CPUI_INT_ADD | OpCode::CPUI_INT_SUB | OpCode::CPUI_PTRADD | OpCode::CPUI_PTRSUB => {
            if let Some(v) = op.get_in(0) {
                visit_candidates(fd, v, first_visit, lookup, result);
            }
        }
        _ => {}
    }
}
/// An expression feeding SP may precede its COPY, but not an observable effect.
fn reservation_step(fd: &Funcdata, allocation: &Allocation, id: OpId) -> bool {
    let Some(op)=fd.obank().get(id) else {return false};
    let writes_result=op.code()==OpCode::CPUI_STORE
        && fd.get_arch().manage().get_stack_space().and_then(|stack|stack.get_contain())
            .is_some_and(|space|op.get_in(0).and_then(|value|number(fd,value))==Some(space.get_index() as i128))
        && op.get_in(1).is_some_and(|pointer|native_stack_delta(fd,pointer,allocation.output)==Some(0));
    if writes_result || independent_frame_store(fd,op,allocation).is_some() || op.code()==OpCode::CPUI_INDIRECT {return true;}
    let evaluation=op.get_eval_type();
    evaluation!=0 && evaluation&pcodeop_flags::special==0 && !matches!(op.code(),
        OpCode::CPUI_INT_DIV|OpCode::CPUI_INT_SDIV|OpCode::CPUI_INT_REM|OpCode::CPUI_INT_SREM)
}

fn reservation_point(fd: &Funcdata, allocation: &Allocation, stack_write: OpId) -> bool {
    let point = allocation.op;
    if point == stack_write {
        return true;
    }
    let Some(op) = fd.obank().get(point) else {
        return false;
    };
    if op.get_parent().is_none()
        || op.get_parent() != fd.obank().get(stack_write).and_then(|op| op.get_parent())
    {
        return false;
    }
    let mut next = op.basic_neighbours().1;
    while let Some(id) = next {
        if id == stack_write {
            return true;
        }
        let Some(op) = fd.obank().get(id) else {
            return false;
        };
        if !reservation_step(fd,allocation,id) {return false;}
        next = op.basic_neighbours().1;
    }
    false
}
fn dominates_op(fd: &Funcdata, definition: OpId, user: OpId) -> bool {
    if definition == user {
        return false;
    }
    let Some(a) = fd.obank().get(definition).and_then(|o| o.get_parent()) else {
        return false;
    };
    let Some(b) = fd.obank().get(user).and_then(|o| o.get_parent()) else {
        return false;
    };
    if a != b {
        return fd.bblocks_ref().dominates(a, Some(b));
    }
    let mut cursor = fd
        .obank()
        .get(definition)
        .and_then(|o| o.basic_neighbours().1);
    while let Some(op) = cursor {
        if op == user {
            return true;
        }
        cursor = fd.obank().get(op).and_then(|o| o.basic_neighbours().1);
    }
    return false;
}
fn dominates_uses(fd: &Funcdata, definition: OpId, value: VarnodeId) -> bool {
    fd.vbank().get(value).unwrap().descend_iter().all(|user| {
        let op = fd.obank().get(user).unwrap();
        (0..op.num_input())
            .filter(|&slot| op.get_in(slot) == Some(value))
            .all(|slot| dominates_input(fd, definition, user, slot))
    })
}
fn canonical_base(
    fd: &Funcdata,
    value: VarnodeId,
    seen: &mut BTreeSet<VarnodeId>,
) -> (VarnodeId, i128) {
    let value = strip(fd, value);
    if !seen.insert(value) {
        return (value, 0);
    }
    if let Some((base, offset)) = constant_step(fd, value) {
        let (base, previous) = canonical_base(fd, base, seen);
        if let Some(total) = previous.checked_add(offset) {
            return (base, total);
        }
    }
    (value, 0)
}
fn equivalent(
    fd: &Funcdata,
    a: VarnodeId,
    b: VarnodeId,
    seen: &mut BTreeSet<(VarnodeId, VarnodeId)>,
) -> bool {
    let a = strip(fd, a);
    let b = strip(fd, b);
    if a == b {
        return true;
    }
    if !seen.insert((a, b)) {
        return false;
    }
    let va = fd.vbank().get(a).unwrap();
    let vb = fd.vbank().get(b).unwrap();
    if va.get_size() != vb.get_size() {
        return false;
    }
    if va.is_constant() && vb.is_constant() {
        return (va.get_offset() & calc_mask(va.get_size()))
            == (vb.get_offset() & calc_mask(vb.get_size()));
    }
    let (Some(da), Some(db)) = (va.get_def(), vb.get_def()) else {
        return false;
    };
    let oa = fd.obank().get(da).unwrap();
    let ob = fd.obank().get(db).unwrap();
    if oa.code() != ob.code() || oa.num_input() != ob.num_input() {
        return false;
    }
    if !matches!(
        oa.code(),
        OpCode::CPUI_INT_ADD
            | OpCode::CPUI_INT_SUB
            | OpCode::CPUI_INT_MULT
            | OpCode::CPUI_INT_AND
            | OpCode::CPUI_INT_OR
            | OpCode::CPUI_INT_XOR
            | OpCode::CPUI_INT_ZEXT
            | OpCode::CPUI_INT_SEXT
            | OpCode::CPUI_INT_2COMP
            | OpCode::CPUI_INT_LEFT
            | OpCode::CPUI_INT_RIGHT
            | OpCode::CPUI_INT_SRIGHT
    ) {
        return false;
    }
    let mut direct = seen.clone();
    if (0..oa.num_input()).all(|i| {
        equivalent(
            fd,
            oa.get_in(i).unwrap(),
            ob.get_in(i).unwrap(),
            &mut direct,
        )
    }) {
        return true;
    }
    matches!(
        oa.code(),
        OpCode::CPUI_INT_ADD
            | OpCode::CPUI_INT_MULT
            | OpCode::CPUI_INT_AND
            | OpCode::CPUI_INT_OR
            | OpCode::CPUI_INT_XOR
    ) && equivalent(fd, oa.get_in(0).unwrap(), ob.get_in(1).unwrap(), seen)
        && equivalent(fd, oa.get_in(1).unwrap(), ob.get_in(0).unwrap(), seen)
}
/// Constant displacement of `b` from `a` when both subtract the same amount from one base.
fn origin_delta(fd: &Funcdata, a: &Allocation, b: &Allocation) -> Option<i128> {
    allocation_delta(fd, a, b, true)
}

fn allocation_delta(
    fd: &Funcdata,
    a: &Allocation,
    b: &Allocation,
    recover_saved: bool,
) -> Option<i128> {
    if a.scale != b.scale { return None; }
    let base = |value| {
        let (base, offset) = canonical_base(fd, value, &mut BTreeSet::new());
        if recover_saved {
            recovered_base(fd, base, offset)
        } else {
            (base, offset)
        }
    };
    let (base_a, offset_a) = base(a.base);
    let (base_b, offset_b) = base(b.base);
    if base_a != base_b {
        return None;
    }
    let amount_delta = if equivalent(fd, a.amount, b.amount, &mut BTreeSet::new()) {
        0
    } else {
        let (amount_a, offset_a) = canonical_base(fd, a.amount, &mut BTreeSet::new());
        let (amount_b, offset_b) = canonical_base(fd, b.amount, &mut BTreeSet::new());
        if !equivalent(fd, amount_a, amount_b, &mut BTreeSet::new()) {
            return None;
        }
        offset_b.checked_sub(offset_a)?.checked_mul(a.scale)?
    };
    offset_b.checked_sub(offset_a)?.checked_sub(amount_delta)
}

/// A constant address displacement, without assuming anything about memory.
fn relative_offset(fd: &Funcdata, value: VarnodeId, anchor: VarnodeId) -> Option<i128> {
    let (value, delta) = canonical_base(fd, value, &mut BTreeSet::new());
    let (anchor, baseline) = canonical_base(fd, anchor, &mut BTreeSet::new());
    let extra = delta.checked_sub(baseline)?;
    if value == anchor {
        return Some(extra);
    }
    let value = allocation(fd, fd.vbank().get(value)?.get_def()?)?;
    let anchor = allocation(fd, fd.vbank().get(anchor)?.get_def()?)?;
    allocation_delta(fd, &anchor, &value, false)?.checked_add(extra)
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum FrameWalk {
    Values,
    OpaqueLoads,
    OpaqueAddresses,
}

fn frame_dependent(fd: &Funcdata, value: VarnodeId, budget: &mut usize, walk: FrameWalk, proof: &mut SavedFrameQuery) -> bool {
    let Some(v) = fd.vbank().get(value) else {
        return true;
    };
    if v.is_input() {
        return v.is_spacebase() || stack_register(fd, value);
    }
    let mut seen = BTreeSet::new();
    let mut todo = vec![value];
    while let Some(value) = todo.pop() {
        if !seen.insert(value) {
            continue;
        }
        let Some(left) = budget.checked_sub(1) else {
            return true;
        };
        *budget = left;
        let Some(v) = fd.vbank().get(value) else {
            return true;
        };
        if v.is_input() && (v.is_spacebase() || stack_register(fd, value)) {
            return true;
        }
        let Some(op) = v.get_def().and_then(|id| fd.obank().get(id)) else {
            continue;
        };
        // A LOAD can reveal a stored frame address during value resolution.
        // Other saved-value roots are already in this expression's ancestry.
        if op.code() == OpCode::CPUI_LOAD {
            if walk != FrameWalk::Values { return true; }
            let id = v.get_def().unwrap();
            if load_frame_sources(fd, id, &mut todo, budget, proof).unwrap_or(true) { return true; }
            continue;
        }
        if walk == FrameWalk::OpaqueAddresses
            && matches!(op.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND | OpCode::CPUI_CALLOTHER)
        { return true; }
        if !matches!(op.code(), OpCode::CPUI_LOAD | OpCode::CPUI_CALL | OpCode::CPUI_CALLIND) {
            todo.extend((0..op.num_input()).filter_map(|slot| op.get_in(slot)));
        }
    }
    false
}

/// SP before this native instruction, not a second SSA expression of its result.
/// SP before this native instruction, following only a unique incoming path.
/// A join needs its existing SP phi; this does not invent a merged SP value.
fn preceding_stack(fd: &Funcdata, point: OpId, budget: &mut usize) -> Option<VarnodeId> {
    let point = fd.obank().get(point)?;
    preceding_stack_from(fd, point.get_addr(), point.get_parent()?, point.basic_neighbours().0,
        &mut BTreeSet::new(), budget)?
}
fn same_instruction_stack(fd: &Funcdata, point: OpId) -> Option<VarnodeId> {
    let point = fd.obank().get(point)?;
    let mut previous = point.basic_neighbours().0;
    while let Some(id) = previous {
        let op = fd.obank().get(id)?;
        if op.get_parent() != point.get_parent() || op.get_addr() != point.get_addr() { return None; }
        if let Some(output) = op.get_out().filter(|&value| stack_register(fd, value)) { return Some(output); }
        previous = op.basic_neighbours().0;
    }
    None
}
// The stack value reaching a merge is defined only when every incoming path
// supplies the same native SP. A path back into a block already on the walk
// cannot introduce a different value. `Some(None)` marks such a path.
fn preceding_stack_from(
    fd: &Funcdata, at: &Address, mut parent: BlockId, mut previous: Option<OpId>,
    active: &mut BTreeSet<BlockId>, budget: &mut usize,
) -> Option<Option<VarnodeId>> {
    loop {
        while let Some(id) = previous {
            *budget = budget.checked_sub(1)?;
            let op = fd.obank().get(id)?;
            if op.get_parent() != Some(parent) || op.code() == OpCode::CPUI_CALLOTHER { return None; }
            if matches!(op.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND) {
                let call = fd.get_call_specs(fd.get_call_specs_index(id)?);
                if call.get_extra_pop() != 0 || call.proto().has_effect_override() { return None; }
                return Some(Some(fd.kuna_stack_call_sp.get(&id).copied().flatten()?));
            }
            if op.get_addr() != at || op.code() == OpCode::CPUI_MULTIEQUAL {
                if let Some(output) = op.get_out().filter(|&value| stack_register(fd, value)) {
                    return Some(Some(output));
                }
            }
            previous = op.basic_neighbours().0;
        }
        *budget = budget.checked_sub(1)?;
        let block = fd.bblocks_ref().block(parent);
        match block.size_in() {
            0 => return Some(Some(fd.find_spacebase_input(fd.get_arch().manage().get_stack_space()?)?)),
            1 => { parent = block.get_in(0); previous = fd.bb_op_tail(parent); }
            count => {
                if !active.insert(parent) { return Some(None); }
                let mut values = Vec::new();
                for edge in 0..count {
                    let source = block.get_in(edge);
                    if let Some(value) = preceding_stack_from(fd, at, source, fd.bb_op_tail(source), active, budget)? {
                        values.push(value);
                    }
                }
                active.remove(&parent);
                let Some(&first) = values.first() else { return Some(None) };
                if values.iter().any(|&value| native_stack_delta(fd, value, first) != Some(0)) { return None; }
                // Use a representative available on every path into the merge.
                let available = |value: VarnodeId| fd.vbank().get(value).is_some_and(|node| {
                    node.get_def().and_then(|definition| fd.obank().get(definition)).and_then(|op| op.get_parent())
                        .is_none_or(|block| block != parent && fd.bblocks_ref().dominates(block, Some(parent)))
                });
                return Some(Some(values.into_iter().find(|&value| available(value))?));
            }
        }
    }
}















/// Displaced copies of a root allocation, as `(op, root output, displacement)`.
fn displaced_aliases(
    fd: &Funcdata, roots: &[Allocation], aliases: &[(VarnodeId, VarnodeId)],
) -> Vec<(OpId, VarnodeId, i128)> {
    let mut ends = RestorationCache::new();
    fd.obank().iter_alive().filter_map(|op| {
        let candidate = allocation(fd, op)?;
        if roots.iter().any(|root| root.op == op) || aliases.iter().any(|alias| alias.0 == candidate.output) { return None; }
        let (root, delta) = alias_origin(fd, roots, &candidate, &mut ends)?;
        (delta != 0).then_some((op, root, delta))
    }).collect()
}
fn identify_aliases(
    fd: &Funcdata, found: Vec<Allocation>, stack_definitions: &[OpId],
) -> (Vec<Allocation>, Vec<(VarnodeId, VarnodeId)>) {
    let mut ends = RestorationCache::new();
    let roots = logical_allocations(fd, found, stack_definitions, &mut ends);
    let aliases = fd.obank().iter_alive().filter_map(|op| {
        if roots.iter().any(|root| root.op == op) { return None; }
        let candidate = allocation(fd, op)?;
        let (root, delta) = alias_origin(fd, &roots, &candidate, &mut ends)?;
        (delta == 0).then_some((candidate.output, root))
    }).collect();
    (roots, aliases)
}
fn merge(out: &mut Bounds, from: Bounds, upper: bool) {
    for (root, offset) in from {
        out.entry(root)
            .and_modify(|old| {
                *old = if *old == UNKNOWN || offset == UNKNOWN {
                    UNKNOWN
                } else if upper {
                    (*old).max(offset)
                } else {
                    (*old).min(offset)
                }
            })
            .or_insert(offset);
    }
}
fn shift(mut value: Bounds, by: Option<i128>) -> Bounds {
    for offset in value.values_mut() {
        *offset = if *offset == UNKNOWN {
            UNKNOWN
        } else {
            by.and_then(|by| offset.checked_add(by)).unwrap_or(UNKNOWN)
        };
    }
    value
}
fn alignment(fd: &Funcdata, op: OpId) -> Option<(int4,u128)> {
    alignment_using(fd,op,&|value|(value,false))
}
fn indirect_stack_pop(fd: &Funcdata, id: OpId) -> Option<i128> {
    let op = fd.obank().get(id)?;
    if !op.get_out().is_some_and(|out| stack_register(fd, out)) {
        return None;
    }
    let effect = fd.vbank().get(op.get_in(1)?)?;
    let call = crate::funcdata_varnode::op_iop_decode(effect.get_offset());
    let fc = fd.get_call_specs_index(call)?;
    let pop = fd.get_call_specs(fc).get_extra_pop();
    (pop != crate::fspec::EXTRAPOP_UNKNOWN).then_some(pop as i128)
}

#[derive(Clone, Copy)]
enum BoundTransfer {
    Empty,
    Root(usize),
    Copy(Option<VarnodeId>),
    Shift(Option<VarnodeId>, Option<i128>),
    Merge(OpId),
    UnknownPair([Option<VarnodeId>; 2]),
}

fn bound_transfer(fd: &Funcdata, id: OpId, upper: bool) -> BoundTransfer {
    let o = fd.obank().get(id).unwrap();
    match o.code() {
        OpCode::CPUI_COPY | OpCode::CPUI_CAST => BoundTransfer::Copy(o.get_in(0)),
        OpCode::CPUI_MULTIEQUAL => BoundTransfer::Merge(id),
        OpCode::CPUI_INT_ADD | OpCode::CPUI_INT_SUB | OpCode::CPUI_PTRSUB => {
            if let Some(by) = o.get_in(1).and_then(|v| number(fd, v)) {
                BoundTransfer::Shift(
                    o.get_in(0),
                    Some(if o.code() == OpCode::CPUI_INT_SUB { -by } else { by }),
                )
            } else if o.code() == OpCode::CPUI_INT_ADD {
                if let Some(by) = o.get_in(0).and_then(|v| number(fd, v)) {
                    BoundTransfer::Shift(o.get_in(1), Some(by))
                } else {
                    BoundTransfer::UnknownPair([o.get_in(0), o.get_in(1)])
                }
            } else {
                BoundTransfer::Shift(o.get_in(0), None)
            }
        }
        OpCode::CPUI_PTRADD => BoundTransfer::Shift(
            o.get_in(0),
            o.get_in(1)
                .and_then(|v| number(fd, v))
                .zip(o.get_in(2).and_then(|v| number(fd, v)))
                .and_then(|(a, b)| a.checked_mul(b)),
        ),
        OpCode::CPUI_INT_AND => {
            if let Some((slot, amount)) = alignment(fd, id) {
                BoundTransfer::Shift(
                    o.get_in(slot),
                    if upper {
                        Some(0)
                    } else {
                        i128::try_from(amount - 1).ok().map(|a| -a)
                    },
                )
            } else {
                BoundTransfer::UnknownPair([o.get_in(0), o.get_in(1)])
            }
        }
        OpCode::CPUI_INDIRECT => {
            BoundTransfer::Shift(o.get_in(0), indirect_stack_pop(fd, id))
        }
        _ => BoundTransfer::Empty,
    }
}

fn evaluate(fd: &Funcdata, transfer: BoundTransfer, values: &Values, upper: bool) -> Bounds {
    let input = |value: Option<VarnodeId>| {
        value.and_then(|v| values.get(&v)).cloned().unwrap_or_default()
    };
    match transfer {
        BoundTransfer::Empty => Bounds::new(),
        BoundTransfer::Root(root) => Bounds::from([(root, 0)]),
        BoundTransfer::Copy(value) => input(value),
        BoundTransfer::Shift(value, by) => shift(input(value), by),
        BoundTransfer::Merge(id) => {
            let o = fd.obank().get(id).unwrap();
            let mut result = Bounds::new();
            for slot in 0..o.num_input() {
                merge(&mut result, input(o.get_in(slot)), upper);
            }
            result
        }
        BoundTransfer::UnknownPair([a, b]) => {
            let mut result = shift(input(a), None);
            merge(&mut result, shift(input(b), None), upper);
            result
        }
    }
}
fn propagate_bounds(
    fd: &Funcdata,
    ops: &[OpId],
    roots: &BTreeMap<VarnodeId, usize>,
    displaced: &BTreeMap<OpId, (VarnodeId, i128)>,
    upper: bool,
) -> Option<Values> {
    // Scalar folds and call-pop facts depend on IR, not on the evolving bounds.
    // Rebuild this plan for each immutable invocation: no facts survive an IR
    // mutation, and every fold retains its original independent node budget.
    let mut transfers = Vec::with_capacity(ops.len());
    for &id in ops {
        let Some(output) = fd.obank().get(id).unwrap().get_out() else { continue };
        let transfer = if let Some(&root) = roots.get(&output) {
            BoundTransfer::Root(root)
        } else if let Some(&(base, offset)) = displaced.get(&id) {
            BoundTransfer::Shift(Some(base), Some(offset))
        } else {
            bound_transfer(fd, id, upper)
        };
        transfers.push((output, transfer));
    }
    let mut values = Values::new();
    let update = |values: &mut Values| {
        let mut changed = BTreeSet::new();
        for &(output, transfer) in &transfers {
            let next = evaluate(fd, transfer, values, upper);
            if values.get(&output) != Some(&next) {
                values.insert(output, next);
                changed.insert(output);
            }
        }
        changed
    };
    // A pointer induction cycle need not change the outgoing stack area. Widen
    // bounds still changing after a bounded number of sweeps to unknown and
    // propagate that fact. An unknown bound reaching a physical stack-pointer
    // write still declines below.
    for _ in 0..WIDENINGS {
        let mut changed = BTreeSet::new();
        for _ in 0..SWEEPS {
            changed = update(&mut values);
            if changed.is_empty() {
                return Some(values);
            }
        }
        for value in changed {
            for offset in values
                .get_mut(&value)
                .into_iter()
                .flat_map(|b| b.values_mut())
            {
                *offset = UNKNOWN;
            }
        }
    }
    None
}
#[derive(Clone, Copy, Debug)]
struct Reservation {
    count: u64,
    shift: u64,
}

// The SPARC V9 window ABI includes the 2047-byte bias in its local-range header.
fn stack_bias(fd: &Funcdata, width: int4) -> u64 {
    if width != 8 || !fd.get_func_proto().has_model() {
        return 0;
    }
    let arch = fd.get_arch();
    let Some(stack) = arch.manage().get_stack_space() else {
        return 0;
    };
    let sparc = arch
        .manage()
        .register_lookup()
        .and_then(|r| r.probe_register("didrestore"))
        .is_some();
    if sparc
        && fd
            .get_func_proto()
            .model()
            .get_local_range()
            .iter()
            .any(|r| {
                r.get_space().get_index() == stack.get_index()
                    && r.get_first() == 0
                    && r.get_last() == 0x8ae
            })
    {
        0x7ff
    } else {
        0
    }
}
fn reserves(
    fd: &Funcdata,
    allocations: &[Allocation],
    aliases: &[(VarnodeId, VarnodeId)],
    displaced: &[(OpId, VarnodeId, i128)],
    width: int4,
) -> Option<Vec<Reservation>> {
    let ops: Vec<_> = fd.obank().iter_alive().collect();
    let mut roots: BTreeMap<_, _> = allocations
        .iter()
        .enumerate()
        .map(|(i, a)| (a.output, i))
        .collect();
    for &(alias, original) in aliases {
        if let Some(&root) = roots.get(&original) {
            roots.insert(alias, root);
        }
    }
    let mut virtual_adds = BTreeMap::new();
    for &(op, base, delta) in displaced {
        let output = fd.obank().get(op)?.get_out()?;
        let size = fd.vbank().get(output)?.get_size();
        let offset = sign_extend((delta as u64 & calc_mask(size)) as i64, size * 8 - 1) as i128;
        virtual_adds.insert(op, (base, offset));
    }
    let values = propagate_bounds(fd, &ops, &roots, &virtual_adds, false)?;
    let upper = propagate_bounds(fd, &ops, &roots, &virtual_adds, true)?;
    let bias = stack_bias(fd, width) as i128;
    let mut low = vec![0_i128; allocations.len()];
    let mut high = vec![bias; allocations.len()];
    for bounds in upper.values() {
        for (&root, &offset) in bounds {
            if offset != UNKNOWN {
                high[root] = high[root].max(offset);
            }
        }
    }
    let mut align = 16_u128;
    for &id in &ops {
        let o = fd.obank().get(id)?;
        if let Some((slot, a)) = alignment(fd, id) {
            if o.get_in(slot)
                .and_then(|v| values.get(&v))
                .is_some_and(|v| !v.is_empty())
            {
                align = align.max(a);
            }
        }
        let mut references = Vec::new();
        if let Some(out) = o.get_out().filter(|&v| stack_register(fd, v)) {
            if upper
                .get(&out)
                .is_some_and(|v| v.values().any(|&x| x == UNKNOWN))
            {
                return None;
            }
            references.push((out, true));
        }
        if matches!(o.code(), OpCode::CPUI_LOAD | OpCode::CPUI_STORE) {
            if let Some(ptr) = o.get_in(1) {
                references.push((ptr, false));
                let accessed = if o.code() == OpCode::CPUI_STORE {
                    o.get_in(2)
                } else {
                    o.get_out()
                };
                let size = accessed
                    .and_then(|v| fd.vbank().get(v))
                    .map_or(0, |v| v.get_size()) as i128;
                for (&root, &offset) in upper.get(&ptr).into_iter().flatten() {
                    if offset != UNKNOWN {
                        high[root] = high[root].max(offset.checked_add(size)?);
                    }
                }
            }
        }
        if matches!(
            o.code(),
            OpCode::CPUI_CALL | OpCode::CPUI_CALLIND | OpCode::CPUI_CALLOTHER
        ) {
            for slot in 1..o.num_input() {
                if let Some(v) = o.get_in(slot) {
                    references.push((v, false));
                }
            }
        }
        for (value, exact_stack) in references {
            for (&root, &offset) in values.get(&value).into_iter().flatten() {
                if offset == UNKNOWN {
                    if exact_stack {
                        return None;
                    }
                } else {
                    low[root] = low[root].min(offset);
                }
            }
        }
    }
    low.into_iter()
        .zip(high)
        .zip(allocations)
        .map(|((lo, hi), allocation)| {
            let before = ((bias.checked_sub(lo)? as u128).checked_add(align - 1)?) & !(align - 1);
            let after =
                ((hi.checked_sub(bias)?.max(0) as u128).checked_add(align - 1)?) & !(align - 1);
            let prefix = native_stack_delta(fd, allocation.before, allocation.base)?;
            let prefix=if prefix<0 {
                // The witnessed native commit is below the source pointer.
                // Its observed low bound must already provide that headroom.
                let shift=before.checked_sub(u128::try_from(bias).ok()?)?;
                if shift<prefix.unsigned_abs() {return None;}
                0
            } else {prefix as u128};
            let prefix = prefix.checked_add(align - 1)? & !(align - 1);
            let count = before.checked_add(after)?.max(prefix);
            if count > calc_mask(width) as u128 {
                return None;
            }
            Some(Reservation {
                count: count as u64,
                shift: (before as u64).checked_sub(bias as u64)?,
            })
        })
        .collect()
}

/// Prove a stack address never exceeds an anchor on the relevant execution paths.
fn stack_upper(
    fd: &Funcdata, value: VarnodeId, anchor: VarnodeId, reservation: Option<&Allocation>,
    after: Option<&BTreeSet<BlockId>>, budget: &mut usize,
) -> Option<i128> {
    stack_upper_using(fd,value,anchor,reservation,after,budget,&|value|(value,false))
}

fn at_or_below(
    fd: &Funcdata, value: VarnodeId, anchor: VarnodeId, ceiling: i128,
    reservation: Option<&Allocation>, after: Option<&BTreeSet<BlockId>>, budget: &mut usize,
) -> bool {
    stack_upper(fd,value,anchor,reservation,after,budget).is_some_and(|upper|upper<=ceiling)
}

fn reachable_blocks(
    fd: &Funcdata,
    start: BlockId,
    budget: &mut usize,
) -> Option<BTreeSet<BlockId>> {
    let mut reached = BTreeSet::new();
    let mut todo = vec![start];
    while let Some(block) = todo.pop() {
        if !reached.insert(block) {
            continue;
        }
        let block = fd.bblocks_ref().block(block);
        for slot in 0..block.size_out() {
            *budget = budget.checked_sub(1)?;
            todo.push(block.get_out(slot));
        }
    }
    Some(reached)
}

/// Independent restored frames cannot become simultaneously live C allocas.
fn sequential_reservations_survive(fd: &Funcdata, allocations: &[Allocation]) -> bool {
    if allocations.len() < 2 {
        return true;
    }
    let mut graph_budget = 65536;
    let mut proof_budget = 4096;
    for earlier in allocations {
        let Some(block) = fd.obank().get(earlier.op).and_then(|op| op.get_parent()) else {
            return false;
        };
        let Some(reachable) = reachable_blocks(fd, block, &mut graph_budget) else {
            return false;
        };
        for later in allocations {
            if earlier.op == later.op {
                continue;
            }
            let Some(next) = fd.obank().get(later.op).and_then(|op| op.get_parent()) else {
                return false;
            };
            // Other iterations are already covered by the loop proof.
            if next == block && !dominates_op(fd, earlier.op, later.op) {
                continue;
            }
            if reachable.contains(&next)
                && !at_or_below(
                    fd,
                    later.before,
                    earlier.output,
                    0,
                    Some(earlier),
                    Some(&reachable),
                    &mut proof_budget,
                )
            {
                return false;
            }
        }
    }
    true
}

/// A compiler alloca survives until return, unlike restored loop storage.
fn loop_restores_stack(fd: &Funcdata, allocation: &Allocation) -> bool {
    if negate(fd, allocation.amount).is_some() {
        return true;
    }
    let Some(block) = fd.obank().get(allocation.op).and_then(|o| o.get_parent()) else {
        return true;
    };
    let mut budget = 65536;
    let Some(reachable) = reachable_blocks(fd, block, &mut budget) else {
        return true;
    };
    let graph = fd.bblocks_ref();
    let cyclic = reachable.iter().any(|&b| {
        let b = graph.block(b);
        (0..b.size_out()).any(|i| b.get_out(i) == block)
    });
    if !cyclic {
        return false;
    }
    let mut proved = false;
    let mut proof_budget = 4096;
    for &latch in &reachable {
        let b = graph.block(latch);
        for slot in 0..b.size_out() {
            let header = b.get_out(slot);
            if !graph.dominates(header, Some(latch)) || !graph.dominates(header, Some(block)) {
                continue;
            }
            let mut natural = BTreeSet::from([header]);
            let mut todo = vec![latch];
            while let Some(b) = todo.pop() {
                if !natural.insert(b) {
                    continue;
                }
                let b = graph.block(b);
                for i in 0..b.size_in() {
                    if budget == 0 {
                        return true;
                    }
                    budget -= 1;
                    todo.push(b.get_in(i));
                }
            }
            if !natural.contains(&block) {
                continue;
            }
            proved = true;
            let incoming = graph.block(header).get_in_index(latch);
            let carried = fd.bb_ops(header).into_iter().any(|id| {
                let op = fd.obank().get(id).unwrap();
                if op.code() != OpCode::CPUI_MULTIEQUAL {
                    return false;
                }
                let Some(header_sp) = op.get_out().filter(|&v| stack_register(fd, v)) else {
                    return false;
                };
                let Some(carried_sp) = op.get_in(incoming) else {
                    return false;
                };
                at_or_below(fd, allocation.before, header_sp, 0, None, None, &mut proof_budget)
                    && at_or_below(
                        fd,
                        carried_sp,
                        allocation.output,
                        0,
                        Some(allocation),
                        Some(&reachable),
                        &mut proof_budget,
                    )
            });
            if !carried {
                return true;
            }
        }
    }
    !proved
}

/// Preserve unambiguous fixed-frame write widths before SP aliases are erased.
fn fixed_frame_writes(fd: &Funcdata) -> Vec<(i64, int4)> {
    let Some(space) = fd.get_arch().manage().get_stack_space() else {
        return Vec::new();
    };
    let at = |off| kuna_base::address::Address::new(std::rc::Rc::clone(&space), off);
    let bias = stack_bias(fd, space.get_addr_size() as int4) as i64;
    let bank = fd.vbank();
    let mut writes: BTreeMap<i64, (int4, bool)> = BTreeMap::new();
    for id in bank.iter_loc_addr_range(&at(0), &at(1))
        .chain(bank.iter_loc_addr_range(&at(1), &at(0)))
    {
        let Some(value) = bank.get(id) else { continue };
        let Some(op) = value.get_def().and_then(|id| fd.obank().get(id)) else { continue };
        if value.is_free()
            || matches!(op.code(), OpCode::CPUI_INDIRECT | OpCode::CPUI_MULTIEQUAL)
        {
            continue;
        }
        let off = sign_extend(value.get_offset() as i64, space.get_addr_size() as i32 * 8 - 1);
        if off >= bias {
            continue;
        }
        let size = value.get_size();
        writes.entry(off)
            .and_modify(|(prior, valid)| {
                *valid &= *prior == size;
                *prior = (*prior).max(size);
            })
            .or_insert((size, true));
    }
    let mut writes = writes.into_iter().peekable();
    let mut prior_end = i64::MIN;
    let mut out = Vec::new();
    while let Some((off, (size, valid))) = writes.next() {
        let end = off + i64::from(size);
        let overlap = off < prior_end || writes.peek().is_some_and(|(next, _)| *next < end);
        prior_end = prior_end.max(end);
        if valid && !overlap {
            out.push((off, size));
        }
    }
    out
}









pub(crate) fn is_scoped_allocation(fd: &Funcdata, id: OpId) -> bool {
    if fd.kuna_stack_scopes.is_empty() {
        return false;
    }
    fd.obank().get(id).is_some_and(|op| op.code() == OpCode::CPUI_CALLOTHER
        && op.num_input() == 3
        && op.get_in(0).and_then(|value| fd.vbank().get(value))
            .is_some_and(|value| value.is_constant() && value.get_offset() == BUILTIN_STACK_ALLOC as u64))
}

fn runtime_amount(fd: &Funcdata, amount: VarnodeId) -> bool {
    let Some(value) = fd.vbank().get(amount) else { return false };
    number(fd, amount).is_none()
        && matches!(value.get_size(), 2 | 4 | 8)
        && native_stack_sum(fd, value.get_size(), vec![(amount, 1)], None).is_none()
}

fn independent_frame_store(fd: &Funcdata, op: &crate::op::PcodeOp, allocation: &Allocation) -> Option<()> {
    if op.code() != OpCode::CPUI_STORE { return None; }
    let stack = fd.get_arch().manage().get_stack_space()?;
    let memory = stack.get_contain()?;
    let space = fd.vbank().get(op.get_in(0)?)?;
    if stack.get_word_size() != 1 || memory.get_word_size() != 1 || !space.is_constant()
        || space.get_offset() != memory.get_index() as u64
    {
        return None;
    }
    let size = fd.vbank().get(op.get_in(2)?)?.get_size();
    if size <= 0 { return None; }
    let input = fd.find_spacebase_input(&stack)?;
    let bias = stack_bias(fd, stack.get_addr_size() as int4) as i128;
    let base = native_stack_delta(fd, allocation.before, input)?.checked_add(bias)?;
    let target = native_stack_delta(fd, op.get_in(1)?, input)?;
    (base <= target && target.checked_add(size as i128)? <= bias).then_some(())
}

fn dominates_input(fd: &Funcdata, definition: OpId, user: OpId, slot: int4) -> bool {
    let Some(op) = fd.obank().get(user) else { return false };
    if op.code() != OpCode::CPUI_MULTIEQUAL {
        return dominates_op(fd, definition, user);
    }
    let Some(parent) = op.get_parent() else { return false };
    let Some(block) = fd.obank().get(definition).and_then(|op| op.get_parent()) else {
        return false;
    };
    let parent = fd.bblocks_ref().block(parent);
    if parent.size_in() != op.num_input() || slot < 0 || slot >= op.num_input() {
        return false;
    }
    fd.bblocks_ref().dominates(block, Some(parent.get_in(slot)))
}

type RestorationCache = BTreeMap<OpId, Vec<OpId>>;

fn reservation_active_at(fd: &Funcdata, root: &Allocation, point: OpId, ends: &mut RestorationCache) -> bool {
    let restores = ends.entry(root.op).or_insert_with(|| fd.kuna_stack_scope_holds.iter()
        .filter_map(|&value| proved_restore(fd, root, value).map(|end| end.restore)).collect());
    !restores.iter().any(|&restore| dominates_op(fd, restore, point))
}

fn alias_origin(fd: &Funcdata, roots: &[Allocation], candidate: &Allocation, ends: &mut RestorationCache) -> Option<(VarnodeId, i128)> {
    let mut selected: Option<(VarnodeId, i128, bool)> = None;
    for root in roots {
        if root.output == candidate.output { continue; }
        let Some(delta) = origin_delta(fd, root, candidate) else { continue };
        if !(if delta == 0 { dominates_uses(fd, root.op, candidate.output) } else { dominates_op(fd, root.op, candidate.op) }) { continue; }
        let active = dominates_op(fd, root.op, candidate.op)
            && reservation_active_at(fd, root, candidate.op, ends);
        // Prefer the currently reserved object when sibling scopes reuse the
        // same native addresses. Otherwise retain an expired-address alias:
        // scope_captures must preserve its bits, not pretend it is still live.
        if selected.as_ref().is_none_or(|&(_, _, prior_active)| active && !prior_active) {
            selected = Some((root.output, delta, active));
        }
    }
    selected.map(|(output, delta, _)| (output, delta))
}

fn logical_allocations(
    fd: &Funcdata,
    mut found: Vec<Allocation>,
    stack_definitions: &[OpId],
    ends: &mut RestorationCache,
) -> Vec<Allocation> {
    // Recovery can remove the COPY of an allocation expression into SP.
    // Only expressions witnessed to reserve SP before any intervening effect
    // can become an earlier root; affine equality alone is insufficient.
    for root in &mut found {
        let Some(block) = fd.obank().get(root.op).and_then(|op| op.get_parent()) else {
            continue;
        };
        let mut previous = fd.obank().get(root.op).unwrap().basic_neighbours().0;
        while let Some(id) = previous {
            let Some(op) = fd.obank().get(id) else {
                break;
            };
            if op.get_parent() != Some(block)
                || matches!(
                    op.code(),
                    OpCode::CPUI_CALL
                        | OpCode::CPUI_CALLIND
                        | OpCode::CPUI_CALLOTHER
                        | OpCode::CPUI_RETURN
                        | OpCode::CPUI_BRANCH
                        | OpCode::CPUI_CBRANCH
                        | OpCode::CPUI_BRANCHIND
                )
            {
                break;
            }
            previous = op.basic_neighbours().0;
            // A value the root stack pointer is formed from by a constant
            // adjustment witnesses the reservation even when the copy of it
            // into SP was removed as dead (an outgoing-argument area follows).
            let witnessed = fd.obank().get(id).and_then(|op| op.get_out()).is_some_and(|out|
                fd.obank().get(root.op).is_some_and(|root_op| root_op.code() == OpCode::CPUI_INT_ADD
                    && (0..2).any(|slot| root_op.get_in(slot).map(|input| strip(fd, input)) == Some(out)
                        && root_op.get_in(1 - slot).and_then(|other| number(fd, other)).is_some())));
            // The same amount node in both: the stack pointer was formed from
            // this reservation and a constant frame displacement.
            let shared = allocation(fd, id).is_some_and(|candidate| candidate.amount == root.amount);
            if stack_definitions.binary_search(&id).is_err() && !witnessed && !shared {
                continue;
            }
            if let Some(mut candidate) = allocation(fd, id) {
                normalize_reservation(fd, &mut candidate);
                if fd.vbank().get(candidate.output).unwrap().get_size()
                    == fd.vbank().get(root.output).unwrap().get_size()
                    && origin_delta(fd, root, &candidate).is_some()
                    && reservation_active_at(fd, &candidate, root.op, ends)
                {
                    *root = candidate;
                }
            }
        }
    }
    found.sort_unstable_by_key(|root| root.op);
    found.dedup_by_key(|root| root.op);
    found
        .iter()
        .copied()
        .filter(|a| {
            !found
                .iter()
                .any(|b| origin_delta(fd, a, b).is_some() && dominates_op(fd, b.op, a.op)
                    && reservation_active_at(fd, b, a.op, ends))
        })
        .collect()
}

/// Preserve the native reservation top when constant frame arithmetic was
/// folded into the subtraction base. Only native-witnessed roots call this.
fn normalize_reservation(fd: &Funcdata, allocation: &mut Allocation) {
    let Some(before) = preceding_stack(fd, allocation.op, &mut 4096) else { return };
    if fd.vbank().get(before).and_then(|value| value.get_def())
        .is_some_and(|definition| !dominates_op(fd, definition, allocation.op)) { return; }
    let Some(prefix)=native_stack_delta(fd,before,allocation.base) else {return};
    if prefix<0 {
        // A source address may sit above SP (for example a reserved ABI
        // window). Its witnessed native commit must reserve at least the
        // source expression's full extent, not adopt the source pointer.
        let mut current=Some(allocation.op);
        let mut witnessed=false;
        while let Some(id)=current {
            let Some(op)=fd.obank().get(id) else {return};
            if op.get_parent()!=fd.obank().get(allocation.op).and_then(|op|op.get_parent()) {return;}
            if id!=allocation.op && !reservation_step(fd,allocation,id) {break;}
            if let Some(value)=op.get_out().filter(|&value|stack_register(fd,value)) {
                witnessed=stack_upper_using(fd,value,allocation.output,None,None,&mut 4096,&|value|(value,false))
                    .is_some_and(|shift|shift<=prefix);
            }
            current=op.basic_neighbours().1;
        }
        if !witnessed {return;}
    }
    let width = fd.vbank().get(allocation.output).unwrap().get_size();
    if native_stack_sum(fd, width,
        vec![(before, 1), (allocation.output, -1), (allocation.amount, -allocation.scale)], None) == Some(prefix)
    {
        allocation.before = before;
    }
}

fn native_stack_delta(fd: &Funcdata, value: VarnodeId, anchor: VarnodeId) -> Option<i128> {
    let width = fd.vbank().get(anchor)?.get_size();
    if fd.vbank().get(value)?.get_size() != width { return None; }
    let normalize = |delta: i128| sign_extend((delta as u64 & calc_mask(width)) as i64, width * 8 - 1) as i128;
    if let Some(delta) = relative_offset(fd, value, anchor) {
        return Some(normalize(delta));
    }
    native_stack_sum(fd, width, vec![(value, 1), (anchor, -1)], None)
}

fn native_stack_sum(
    fd: &Funcdata, width: int4, todo: Vec<(VarnodeId, i128)>,
    pops: Option<&mut BTreeMap<OpId, int4>>,
) -> Option<i128> {
    native_stack_sum_using(fd, width, todo, pops, &|value| (value, false))
}

fn mapped_static_access(
    fd: &Funcdata, op: &crate::op::PcodeOp, space: i128, budget: &mut usize,
) -> bool {
    let Some(address) = op.get_in(1) else { return false };
    let Some(offset) = literal(fd, address, budget) else { return false };
    let Some(data_space) = fd.get_arch().manage().get_default_data_space() else { return false };
    if space != data_space.get_index() as i128 { return false; }
    let data = if op.code() == OpCode::CPUI_STORE { op.get_in(2) } else { op.get_out() };
    let Some(size) = data.and_then(|value| fd.vbank().get(value)).map(|value| value.get_size()) else {
        return false;
    };
    let ranges = &fd.get_arch().elem_ptr_ranges;
    let after = ranges.partition_point(|&(first, _)| first <= offset);
    if size <= 0 || after == 0
        || offset.checked_add(size as u64 - 1).is_none_or(|last| last > ranges[after - 1].1)
    { return false; }
    let location = Address::new(Rc::clone(data_space), offset);
    let flags = fd.get_arch().query_global_properties(&location, size, op.get_addr());
    let range_flags = fd.get_arch().query_global_range_properties(&location);
    flags & crate::varnode::varnode_flags::mapped != 0
        && (flags | range_flags) & crate::varnode::varnode_flags::volatil == 0
}

fn observes_after_restore(fd: &Funcdata, restore: OpId, allocation_top: VarnodeId, budget: &mut usize) -> bool {
    restored_storage_observed(fd, restore, allocation_top, budget, true)
}

fn restored_storage_observed(
    fd: &Funcdata, restore: OpId, allocation_top: VarnodeId, budget: &mut usize, calls_observe: bool,
) -> bool {
    let op = fd.obank().get(restore).unwrap();
    let Some(parent) = op.get_parent() else { return true };
    let restored = op.get_out().unwrap();
    let register_space = fd.get_arch().manage().get_stack_space()
        .and_then(|space| space.get_spacebase(0).ok().and_then(|register| {
            register.space.as_ref().map(|space| space.get_index())
        }))
        .filter(|&index| fd.get_arch().manage().get_default_data_space()
            .is_some_and(|space| space.get_index() != index));
    let mut todo = vec![(parent, Some(restore))];
    let mut seen = BTreeSet::new();
    while let Some((block, start)) = todo.pop() {
        if start.is_none() && !seen.insert(block) {
            continue;
        }
        let mut active = start.is_none();
        for id in fd.bb_ops(block) {
            let Some(left) = budget.checked_sub(1) else { return true };
            *budget = left;
            if !active {
                active = Some(id) == start;
                continue;
            }
            let op = fd.obank().get(id).unwrap();
            match op.code() {
                OpCode::CPUI_CALL | OpCode::CPUI_CALLIND if calls_observe => return true,
                OpCode::CPUI_CALLOTHER | OpCode::CPUI_BRANCHIND => return true,
                OpCode::CPUI_LOAD | OpCode::CPUI_STORE => {
                    // Register-bank accesses are not accesses to released stack RAM.
                    let Some(space) = op.get_in(0).and_then(|value| number(fd, value)) else { return true };
                    if register_space.is_some_and(|index| space == index as i128) {
                        continue;
                    }
                    let Some(address) = op.get_in(1) else { return true };
                    if mapped_static_access(fd, op, space, budget) { continue; }
                    // Post-indexed epilogues can update SP before loading fixed
                    // frame words. Those words are above the runtime reservation.
                    let outside = native_stack_delta(fd, address, restored).is_some_and(|offset| offset >= 0)
                        || native_stack_delta(fd, address, allocation_top).is_some_and(|offset| offset >= 0)
                        || {
                            let (anchor, displacement) = canonical_base(fd, address, &mut BTreeSet::new());
                            at_or_below(fd, allocation_top, anchor, displacement, None, None, budget)
                        };
                    if !outside {
                        return true;
                    }
                }
                _ => {}
            }
        }
        let block = fd.bblocks_ref().block(block);
        for edge in 0..block.size_out() {
            todo.push((block.get_out(edge), None));
        }
    }
    false
}

pub(crate) fn scoped_region(fd: &Funcdata, start: OpId, last: BlockId) -> Result<Vec<BlockId>, &'static str> {
    let first = fd.obank().get(start).and_then(|op| op.get_parent())
        .ok_or("Native stack allocation lost its basic block")?;
    let graph = fd.bblocks_ref();
    graph.arena.get(first).ok_or("Native stack allocation block was removed")?;
    graph.arena.get(last).ok_or("Native stack lifetime boundary was removed")?;
    if first == last {
        return Ok(vec![first]);
    }
    let mut todo = vec![first];
    let mut region = BTreeSet::new();
    let mut reaches_end = false;
    let mut budget = 65536usize;
    while let Some(block) = todo.pop() {
        budget = budget.checked_sub(1).ok_or("Native stack lifetime region exceeded its proof budget")?;
        if block == last {
            reaches_end = true;
            continue;
        }
        if !region.insert(block) {
            continue;
        }
        if fd.bb_ops(block).into_iter().any(|id| fd.obank().get(id)
            .is_some_and(|op| op.code() == OpCode::CPUI_BRANCHIND))
        {
            return Err("Native stack lifetime contains an indirect branch");
        }
        if graph.block(block).size_out() == 0 {
            if !fd.bb_op_tail(block).and_then(|id| fd.obank().get(id)).is_some_and(|op| op.code() == OpCode::CPUI_RETURN) {
                return Err("Native stack lifetime has an exit before restoration");
            }
            continue;
        }
        for edge in 0..graph.block(block).size_out() {
            if graph.block(block).is_irreducible_out(edge) {
                return Err("Native stack lifetime contains an irreducible edge");
            }
            let next = graph.block(block).get_out(edge);
            if next == first {
                return Err("Native stack lifetime re-enters its allocation");
            }
            todo.push(next);
        }
    }
    if !reaches_end {
        return Err("Native stack lifetime no longer reaches its restoration");
    }
    if !graph.dominates(first, Some(last)) {
        return Err("Native stack allocation no longer dominates its restoration");
    }
    if region.iter().copied().filter(|&block| block != first).any(|block|
        (0..graph.block(block).size_in()).any(|edge| !region.contains(&graph.block(block).get_in(edge))))
    {
        return Err("Native stack lifetime acquired an outside entry");
    }
    let mut result = vec![first];
    result.extend(region.into_iter().filter(|&block| block != first));
    Ok(result)
}

fn after_scope_end(fd: &Funcdata, id: OpId, end: &LifetimeEnd) -> bool {
    id == end.restore || match end.boundary {
        Some(block) => fd.obank().get(id).and_then(|op| op.get_parent())
            .is_some_and(|parent| fd.bblocks_ref().dominates(block, Some(parent))),
        None => dominates_op(fd, end.restore, id),
    }
}

fn restoration_failure_before(fd: &Funcdata, allocation: &Allocation, native_roots: &[OpId], end: Option<&LifetimeEnd>, facts: &FrameFacts) -> Option<(OpId, String)> {
    let site = fd.obank().get(allocation.op).unwrap().get_addr().get_offset();
    let incomplete = || format!("Stack storage lifetime at {site:#x} could not be proved; allocation recovery declined.");
    let Some(parent) = fd.obank().get(allocation.op).and_then(|op| op.get_parent()) else {
        return Some((allocation.op, incomplete()));
    };
    let mut graph_budget = 65536;
    let Some(mut reachable) = reachable_blocks(fd, parent, &mut graph_budget) else {
        return Some((allocation.op, incomplete()));
    };
    if let Some(end) = end {
        let Ok(region) = scoped_region(fd, allocation.op, end.boundary.unwrap_or(end.join)) else {
            return Some((allocation.op, incomplete()));
        };
        reachable.retain(|block| region.contains(block) || (end.boundary.is_none() && *block == end.join));
    }
    for id in fd.obank().iter_alive() {
        if end.is_some_and(|end| after_scope_end(fd, id, end)) {
            continue;
        }
        let op = fd.obank().get(id).unwrap();
        if op.code() == OpCode::CPUI_MULTIEQUAL {
            continue;
        }
        if !op.get_parent().is_some_and(|block| reachable.contains(&block))
            || (op.get_parent() == Some(parent) && !dominates_op(fd, allocation.op, id))
        {
            continue;
        }
        let Some(output) = op.get_out().filter(|&value| stack_register(fd, value) || native_roots.binary_search(&id).is_ok()) else {
            continue;
        };
        // New reservations are checked by the whole-set lifetime/alias proof.
        // Private saved-frame origins may resolve after input recovery finishes.
        // A proved upward alias is still a restore, never a fresh reservation.
        let mut deferred = false;
        candidates(fd, output, &mut BTreeSet::new(), &mut |next| {
            deferred = deferred || (next.op != allocation.op
                && dominates_op(fd, allocation.op, next.op)
                && negate(fd, next.amount).is_none()
                && native_roots.binary_search(&next.op).is_ok()
                && (next.op == id || dominates_op(fd, next.op, id))
                && facts.equations.delta(fd,next.output,allocation.output).is_none_or(|delta| delta <= 0)
                && (facts.equations.delta(fd,output,next.output).is_some_and(|delta| delta <= 0)
                    || {
                        let mut budget = graph_budget.min(4096);
                        let before = budget;
                        let below = stack_upper_using(fd,output,next.output,None,Some(&reachable),&mut budget,&|value|facts.equations.value(fd,value)).is_some_and(|upper|upper<=0);
                        graph_budget -= before - budget;
                        below
                    }));
        });
        if deferred {
            continue;
        }
        let mut proof_budget = graph_budget.min(4096);
        let before = proof_budget;
        let retained=facts.equations.delta(fd,output,allocation.output).is_some_and(|offset|offset<=0)
            || stack_upper_using(fd,output,allocation.output,None,Some(&reachable),&mut proof_budget,
                &|value|facts.equations.value(fd,value)).is_some_and(|upper|upper<=0);
        graph_budget -= before - proof_budget;
        if !retained && observes_after_restore(fd, id, allocation.before, &mut graph_budget)
        {
            return Some((id, format!("Stack storage lifetime at {site:#x}: retention after native SP write at {:#x} is not proved before a later observer; allocation recovery declined.",
                op.get_addr().get_offset())));
        }
    }
    None
}

pub(crate) fn is_scoped_address_capture(fd: &Funcdata, id: OpId) -> bool {
    !fd.kuna_stack_scopes.is_empty() && fd.obank().get(id).and_then(|op| op.get_out())
        .is_some_and(|value| fd.kuna_stack_scope_holds.contains(&value))
}

#[derive(Clone, Copy)]
enum FrameRead {
    Operation(OpId),
    Operand(OpId,int4),
    Incoming(OpId, int4),
}

fn frame_word_uses(
    fd: &Funcdata, read: FrameRead, birth: Option<OpId>, budget: &mut usize,
) -> Option<BTreeMap<OpId, bool>> {
    frame_word_uses_where(fd, read, birth, budget, |_| true)
}

/// [`frame_word_uses`], recording only the operations `keep` accepts. The walk
/// and its budget are the same; only the result is smaller.
fn frame_word_uses_where(
    fd: &Funcdata, read: FrameRead, birth: Option<OpId>, budget: &mut usize, keep: impl Fn(OpId) -> bool,
) -> Option<BTreeMap<OpId, bool>> {
    let mut uses = BTreeMap::new();
    frame_word_uses_summarized(fd, read, birth, budget, keep, None, |id, live| {
        *uses.entry(id).or_insert(false) |= live;
    })?;
    Some(uses)
}

/// Each block's [`span_effect`] operations, last first, and its operation
/// count. Built on first use; the IR is fixed while one inference runs.
#[derive(Default)]
struct BlockEffects(std::cell::RefCell<std::collections::HashMap<BlockId, std::rc::Rc<(usize, Vec<OpId>)>>>);

impl BlockEffects {
    fn get(&self, fd: &Funcdata, block: BlockId) -> Option<std::rc::Rc<(usize, Vec<OpId>)>> {
        if let Some(found) = self.0.borrow().get(&block) { return Some(found.clone()); }
        let mut count = 0;
        let mut effects = Vec::new();
        let mut previous = fd.bb_op_tail(block);
        while let Some(id) = previous {
            count += 1;
            if span_effect(fd, id) { effects.push(id); }
            previous = fd.obank().get(id)?.basic_neighbours().0;
        }
        let summary = std::rc::Rc::new((count, effects));
        self.0.borrow_mut().insert(block, summary.clone());
        Some(summary)
    }
}

/// [`frame_word_uses_where`] keeping [`span_effect`] operations. A whole
/// block without the restore or the birth is taken from `effects`, charged
/// the same budget as walking its operations one by one.
fn frame_word_spans(
    fd: &Funcdata, read: FrameRead, birth: Option<OpId>, budget: &mut usize, effects: &BlockEffects,
) -> Option<Vec<(OpId, bool)>> {
    let mut uses = Vec::new();
    frame_word_uses_summarized(fd, read, birth, budget, |id| span_effect(fd, id), Some(effects),
        |id, live| uses.push((id, live)))?;
    // Preserve operation order and the liveness union across paths.
    uses.sort_unstable_by_key(|&(id, _)| id);
    uses.dedup_by(|later, earlier| {
        if later.0 != earlier.0 { return false; }
        earlier.1 |= later.1;
        true
    });
    Some(uses)
}

fn frame_word_uses_summarized(
    fd: &Funcdata, read: FrameRead, birth: Option<OpId>, budget: &mut usize, keep: impl Fn(OpId) -> bool,
    effects: Option<&BlockEffects>, mut record: impl FnMut(OpId, bool),
) -> Option<()> {
    let (restore, incoming) = match read {
        FrameRead::Operation(id) | FrameRead::Operand(id,_) => (id, None),
        FrameRead::Incoming(id, slot) => (id, Some(slot)),
    };
    let op = fd.obank().get(restore)?;
    let parent = op.get_parent()?;
    let start = if let Some(slot) = incoming {
        let block = fd.bblocks_ref().block(parent);
        if op.code() != OpCode::CPUI_MULTIEQUAL || slot < 0 || slot >= op.num_input()
            || block.size_in() != op.num_input() { return None; }
        (block.get_in(slot), None, true)
    } else { (parent, Some(restore), true) };
    let mut todo = vec![start];
    // A whole block is walked once per liveness; only the restore's own block
    // is entered mid-way, and a path back to it stops at the restore.
    let mut entries = std::collections::HashSet::new();
    let birth_block = birth.and_then(|id| fd.obank().get(id)).and_then(|op| op.get_parent());
    while let Some((block, before, mut live)) = todo.pop() {
        *budget = budget.checked_sub(1)?;
        let mut previous = if let Some(id) = before {
            fd.obank().get(id)?.basic_neighbours().0
        } else {
            if !entries.insert((block, live)) { continue; }
            if let Some(effects) = effects.filter(|_| block != parent && Some(block) != birth_block) {
                let summary = effects.get(fd, block)?;
                *budget = budget.checked_sub(summary.0)?;
                for &id in &summary.1 { record(id, live); }
                let node = fd.bblocks_ref().block(block);
                for index in 0..node.size_in() { todo.push((node.get_in(index), None, live)); }
                continue;
            }
            fd.bb_op_tail(block)
        };
        let mut stop = false;
        while let Some(id) = previous {
            if incoming.is_none() && id == restore {
                stop = true;
                break;
            }
            *budget = budget.checked_sub(1)?;
            if Some(id) == birth { live = false; }
            if keep(id) { record(id, live); }
            previous = fd.obank().get(id)?.basic_neighbours().0;
        }
        if !stop {
            let node = fd.bblocks_ref().block(block);
            for index in 0..node.size_in() { todo.push((node.get_in(index), None, live)); }
        }
    }
    Some(())
}

#[derive(Clone, Copy)]
struct SavedFrameValue {
    value: Option<VarnodeId>,
    birth: Option<OpId>,
    pending: Option<OpId>,
}

/// State for one immutable saved-value query. Completed facts never outlive it,
/// and a subtree that read an active assumption is never memoized.
#[derive(Default)]
struct SavedFrameQuery {
    active: BTreeMap<OpId, SavedFrameValue>,
    completed: BTreeMap<OpId, SavedFrameValue>,
    assumption_reads: usize,
}

fn resolved_base(
    fd: &Funcdata, value: VarnodeId, budget: &mut usize, proof: &mut SavedFrameQuery,
) -> (VarnodeId, i128) {
    let original = value;
    let mut value = value;
    let mut offset = 0_i128;
    loop {
        let Some(left) = budget.checked_sub(1) else { return (original, 0) };
        *budget = left;
        if let Some(node) = fd.vbank().get(value) {
            if let Some(op) = node.get_def().and_then(|id| fd.obank().get(id))
                .filter(|op| matches!(op.code(), OpCode::CPUI_COPY | OpCode::CPUI_CAST))
            {
                if let Some(source) = op.get_in(0).and_then(|source| fd.vbank().get(source).map(|node|(source,node))) {
                    let stack_read = fd.get_arch().manage().get_stack_space()
                        .is_some_and(|stack| source.1.get_space().get_index() == stack.get_index());
                    if source.1.get_size() == node.get_size() && !node.is_volatile()
                        && !source.1.is_volatile() && !source.1.is_addr_tied() && !stack_read
                    {
                        // SSA aliases have no read obligation of their own.
                        // Peel them even while their memory-reading producer's
                        // conditional value proof is still in progress.
                        value = source.0;
                        continue;
                    }
                }
            }
        }
        if let Some((base, step)) = constant_step(fd, value) {
            let Some(total) = offset.checked_add(step) else { return (value, offset) };
            value = base;
            offset = total;
            continue;
        }
        // Keep each read's evaluation point, including reads below affine
        // arithmetic. A failed read must not fall back to an older marker.
        let Some(known) = saved_stack_value(fd, value, budget, proof).and_then(|saved| saved.value)
            else { return (value, offset) };
        if known == value { return (value, offset); }
        value = known;
    }
}

fn frame_offset_saved(
    fd: &Funcdata, value: VarnodeId, budget: &mut usize, proof: &mut SavedFrameQuery,
) -> (Option<i128>, VarnodeId) {
    let Some(left) = budget.checked_sub(1) else { return (None, value) };
    *budget = left;
    let Some(stack) = fd.get_arch().manage().get_stack_space() else { return (None, value) };
    let Some(input) = fd.find_spacebase_input(&stack) else { return (None, value) };
    if let Some(delta) = native_stack_delta(fd, value, input) { return (Some(delta), value); }
    let (known, displacement) = resolved_base(fd, value, budget, proof);
    let offset = if known == value { None } else {
        frame_offset(fd, known, budget, proof).and_then(|offset| offset.checked_add(displacement))
    };
    (offset, known)
}

fn frame_offset(
    fd: &Funcdata, value: VarnodeId, budget: &mut usize, proof: &mut SavedFrameQuery,
) -> Option<i128> {
    frame_offset_saved(fd, value, budget, proof).0
}

/// The callee's real incoming storage, including finalized stack arguments.
fn call_frame_floor(fd: &Funcdata, call: &crate::fspec::FuncCallSpecs) -> Option<i128> {
    let proto = call.proto();
    if call.is_input_active() || !proto.has_model() || proto.is_model_unknown()
        || proto.has_effect_override() || !proto.is_stack_grows_negative()
        || call.get_extra_pop() == crate::fspec::EXTRAPOP_UNKNOWN || call.get_extra_pop() < 0
    { return None; }
    let stack = fd.get_arch().manage().get_stack_space()?;
    if proto.get_spacebase()?.get_index() != stack.get_index() { return None; }
    let width = stack.get_addr_size() as int4;
    let signed = |value: u64| sign_extend(value as i64, width * 8 - 1) as i128;
    let mut floor = call.get_extra_pop() as i128;
    for range in proto.get_local_range().iter().filter(|r| r.get_space().get_index() == stack.get_index()) {
        // The bundled biased-window ABI spells this negative tail at 32 bits;
        // its separate positive register-window range still counts in full.
        if width == 8 && stack_bias(fd, width) == 0x7ff
            && range.get_first() == 0xfff0bdc1 && range.get_last() == u32::MAX as u64
        { continue; }
        let first = signed(range.get_first());
        let last = signed(range.get_last());
        if first > last { return None; }
        if last >= 0 { floor = floor.max(last.checked_add(1)?); }
    }
    if let Some(first) = proto.get_param_range().iter()
        .filter(|r| r.get_space().get_index() == stack.get_index())
        .map(|r| signed(r.get_first())).filter(|&first| first >= 0).min()
    { floor = floor.max(first); }
    let mut argument = |address: &Address, size: int4| -> Option<()> {
        if address.get_space()?.get_index() == stack.get_index() {
            let offset = signed(address.get_offset());
            if offset < 0 || size <= 0 { return None; }
            floor = floor.max(offset.checked_add(size as i128)?);
        }
        Some(())
    };
    for index in 0..proto.num_params() {
        let param = proto.get_param(index)?;
        argument(&param.get_address(), param.get_size())?;
    }
    for (address, size) in call.final_input_storage() { argument(address, *size)?; }
    Some(floor)
}

/// Native SP definitions stay available until stack-input recovery is complete.
fn unaligned_stack_expression(fd: &Funcdata, mut value: VarnodeId, budget: &mut usize) -> Option<(VarnodeId, i128)> {
    let mut offset = 0_i128;
    loop {
        *budget = budget.checked_sub(1)?;
        let (base, delta) = canonical_base(fd, value, &mut BTreeSet::new());
        offset = offset.checked_add(delta)?;
        let Some(id) = fd.vbank().get(base)?.get_def() else { return Some((base, offset)) };
        let Some((slot, _)) = alignment(fd, id) else { return Some((base, offset)) };
        // Clearing low address bits rounds down; this is an upper bound,
        // never a claim that the aligned and unaligned pointers are equal.
        value = fd.obank().get(id)?.get_in(slot)?;
    }
}

fn call_stack_upper(
    fd: &Funcdata, call: OpId, budget: &mut usize, proof: &mut SavedFrameQuery,
) -> Option<i128> {
    let original = fd.kuna_stack_call_sp.get(&call).copied().flatten()?;
    let value = saved_stack_value(fd, original, budget, proof)
        .and_then(|saved| saved.value).unwrap_or(original);
    if let Some(offset) = frame_offset(fd, value, budget, proof) { return Some(offset); }
    // Reuse the relational solver on the actual native witness. Unlike exact
    // frame offsets, this also covers calls below chained reservations.
    let mut local = (*budget).min(4096);
    let available = local;
    let relational = fd.get_arch().manage().get_stack_space()
        .and_then(|stack|fd.find_spacebase_input(stack))
        .and_then(|entry|stack_upper(fd,original,entry,None,None,&mut local));
    *budget -= available-local;
    let Some((value, extra)) = unaligned_stack_expression(fd, value, budget) else { return relational };
    let mut upper = relational;
    let mut seen = BTreeSet::new();
    for pointer in std::iter::once(value).chain(fd.kuna_stack_scope_holds.iter().copied().filter(|&v| stack_register(fd, v))) {
        candidates(fd, pointer, &mut seen, &mut |root| {
            if negate(fd, root.amount).is_some() { return; }
            let Some(delta) = native_stack_delta(fd, value, root.output) else { return };
            let base = unaligned_stack_expression(fd, root.base, budget)
                .and_then(|(base, offset)| frame_offset(fd, base, budget, proof)?.checked_add(offset));
            if let Some(bound) = base.and_then(|base| base.checked_add(delta)?.checked_add(extra)) {
                upper = Some(upper.map_or(bound, |previous: i128| previous.min(bound)));
            }
        });
    }
    upper
}

fn source_allocation_address(
    fd: &Funcdata, value: VarnodeId, root: VarnodeId,
    budget: &mut usize, proof: &mut SavedFrameQuery,
) -> bool {
    let Some(left) = budget.checked_sub(1) else { return false };
    *budget = left;
    if fd.vbank().get(value).zip(fd.vbank().get(root))
        .is_none_or(|(value, root)| value.get_size() != root.get_size()) { return false; }
    if native_stack_delta(fd, value, root).is_some() { return true; }
    let (known, _) = resolved_base(fd, value, budget, proof);
    if known != value { return source_allocation_address(fd, known, root, budget, proof); }
    let Some(id) = fd.vbank().get(value).and_then(|v| v.get_def()) else { return false };
    let op = fd.obank().get(id).unwrap();
    if let Some((slot, _)) = alignment(fd, id) {
        return source_allocation_address(fd, op.get_in(slot).unwrap(), root, budget, proof);
    }
    let slots: &[int4] = match op.code() {
        OpCode::CPUI_INT_ADD => &[0, 1],
        OpCode::CPUI_INT_SUB | OpCode::CPUI_PTRADD => &[0],
        _ => return false,
    };
    slots.iter().any(|&slot| {
        op.get_in(slot).zip(op.get_in(1 - slot)).is_some_and(|(base, index)|
            source_allocation_address(fd, base, root, budget, proof)
                && !frame_dependent(fd, index, budget, FrameWalk::Values, proof))
    })
}

fn owned_source_address(fd: &Funcdata, value: VarnodeId, budget: &mut usize, proof: &mut SavedFrameQuery) -> bool {
    for &stack in &fd.kuna_stack_scope_holds {
        if !stack_register(fd, stack) { continue; }
        let mut owned = false;
        candidates(fd, stack, &mut BTreeSet::new(), &mut |root| {
            let mut remaining = (*budget).min(256);
            let available = remaining;
            owned |= source_allocation_address(fd, value, root.output, &mut remaining, proof);
            *budget -= available - remaining;
        });
        if owned { return true; }
    }
    false
}

/// Possible frame provenance is not an equality proof. Inspect every path
/// after the nearest full overwrite, including conditional memory writers.
/// Possible frame provenance is not an equality proof. Inspect every path
/// after the nearest full overwrite, including conditional memory writers.
fn source_address_inputs(fd: &Funcdata, value: VarnodeId, root: VarnodeId, budget: &mut usize, inputs: &mut Vec<VarnodeId>, proof: &mut SavedFrameQuery) -> bool {
    let Some(left) = budget.checked_sub(1) else { return false };
    *budget = left;
    if fd.vbank().get(value).zip(fd.vbank().get(root))
        .is_none_or(|(value, root)| value.get_size() != root.get_size()) { return false; }
    if native_stack_delta(fd, value, root).is_some() { return true; }
    // Reuse existing value equations only. Starting another spill proof here
    // would restart the provenance walk whose scalar indices we are deferring.
    if let Some(id) = fd.vbank().get(value).and_then(|node| node.get_def()) {
        let known = if let Some(saved) = proof.active.get(&id).copied() {
            if saved.value.is_some() { proof.assumption_reads += 1; }
            saved.value
        } else { proof.completed.get(&id).and_then(|saved| saved.value) };
        if let Some(known) = known.filter(|&known| known != value) {
            return source_address_inputs(fd, known, root, budget, inputs, proof);
        }
    }
    let (known, _) = canonical_base(fd, value, &mut BTreeSet::new());
    if known != value { return source_address_inputs(fd, known, root, budget, inputs, proof); }
    let Some(id) = fd.vbank().get(value).and_then(|node| node.get_def()) else { return false };
    let op = fd.obank().get(id).unwrap();
    if let Some((slot, _)) = alignment(fd, id) {
        return source_address_inputs(fd, op.get_in(slot).unwrap(), root, budget, inputs, proof);
    }
    let slots: &[int4] = match op.code() {
        OpCode::CPUI_INT_ADD => &[0,1],
        OpCode::CPUI_INT_SUB | OpCode::CPUI_PTRADD => &[0],
        _ => return false,
    };
    for &slot in slots {
        let Some((base,index)) = op.get_in(slot).zip(op.get_in(1-slot)) else { continue };
        if source_address_inputs(fd, base, root, budget, inputs, proof) {
            inputs.push(index);
            return true;
        }
    }
    false
}

fn source_frame_inputs(
    fd: &Funcdata, value: VarnodeId, sources: &mut Vec<VarnodeId>, budget: &mut usize, proof: &mut SavedFrameQuery,
) -> bool {
    for &stack in &fd.kuna_stack_scope_holds {
        if !stack_register(fd, stack) { continue; }
        let mut owned = false;
        candidates(fd, stack, &mut BTreeSet::new(), &mut |root| {
            if owned { return; }
            let mut remaining = (*budget).min(256);
            let available = remaining;
            owned = source_address_inputs(fd, value, root.output, &mut remaining, sources, proof);
            *budget -= available - remaining;
        });
        if owned { return true; }
    }
    false
}

fn load_frame_sources(
    fd: &Funcdata, load: OpId, sources: &mut Vec<VarnodeId>, budget: &mut usize, proof: &mut SavedFrameQuery,
) -> Option<bool> {
    let op = fd.obank().get(load)?;
    let pointer = op.get_in(1)?;
    let memory = number(fd, op.get_in(0)?)?;
    let size = fd.vbank().get(op.get_out()?)?.get_size();
    let stack = fd.get_arch().manage().get_stack_space()?;
    let entry = fd.find_spacebase_input(stack)?;
    let first = frame_offset(fd, pointer, budget, proof);
    let writer = raw_word_writer(fd, load, pointer, memory, first, WordQuery::Origin, budget, proof);
    let add_source = |value, sources: &mut Vec<VarnodeId>, budget: &mut usize, proof: &mut SavedFrameQuery| {
        if !frame_dependent(fd, value, budget, FrameWalk::OpaqueLoads, proof) { return; }
        // An owned base contributes no compiler-frame provenance, but its
        // indices still do. Resolve those in this walk, not a fresh recursive
        // saved-value proof that loses the walk's visited set.
        if source_frame_inputs(fd, value, sources, budget, proof) { return; }
        sources.push(value);
    };
    if let Some(writer) = writer { add_source(writer.value, sources, budget, proof); }
    for (id, live) in frame_word_uses(fd, FrameRead::Operation(load), writer.map(|writer| writer.op), budget)? {
        if !live { continue; }
        let op = fd.obank().get(id)?;
        if op.is_dead() { continue; }
        *budget = budget.checked_sub(1)?;
        let payload = if op.code() == OpCode::CPUI_STORE {
            if number(fd, op.get_in(0)?)? != memory { continue; }
            let target = op.get_in(1)?;
            let payload = op.get_in(2)?;
            let bytes = fd.vbank().get(payload)?.get_size() as i128;
            let delta = native_stack_delta(fd, target, pointer).or_else(||
                first.and_then(|first| frame_offset(fd, target, budget, proof)
                    .and_then(|target| target.checked_sub(first))));
            if let Some(delta) = delta {
                if delta >= size as i128 || delta.checked_add(bytes)? <= 0 { continue; }
            } else {
                if at_or_below(fd, target, pointer, -bytes, None, None, budget)
                    || at_or_below(fd, pointer, target, -(size as i128), None, None, budget)
                { continue; }
                // This is a may-origin walk, not a must-alias/value proof.
                // Keep every remaining possibly frame-based store's payload;
                // precise ownership would recursively ask this same provenance.
                if first.is_some() && !frame_dependent(fd, target, budget, FrameWalk::OpaqueAddresses, proof)
                { continue; }
            }
            payload
        } else if matches!(op.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND | OpCode::CPUI_CALLOTHER) {
            if let Some(first) = first {
                if op.code() != OpCode::CPUI_CALLOTHER {
                    let spec = fd.get_call_specs(fd.get_call_specs_index(id)?);
                    let limit = call_stack_upper(fd, id, budget, proof)?.checked_add(call_frame_floor(fd, spec)?)?;
                    if first < limit { return Some(true); }
                }
            }
            let begin = int4::from(op.code() != OpCode::CPUI_CALLIND);
            for slot in begin..op.num_input() {
                let argument = op.get_in(slot)?;
                if source_frame_inputs(fd, argument, sources, budget, proof) { continue; }
                let aliases = if let Some(first) = first {
                    frame_pointer_reaches(fd, argument, first, size, budget, proof)
                } else {
                    frame_dependent(fd, argument, budget, FrameWalk::OpaqueLoads, proof)
                        && !at_or_below(fd, argument, pointer, -(size as i128), None, None, budget)
                        && !at_or_below(fd, pointer, argument, -(size as i128), None, None, budget)
                };
                if aliases { add_source(argument, sources, budget, proof); }
            }
            continue;
        } else {
            if op.is_marker() || stack.get_contain().is_none_or(|space| space.get_index() as i128 != memory) { continue; }
            let Some(value) = op.get_out().and_then(|out| fd.vbank().get(out))
                .filter(|out| out.get_space().get_index() == stack.get_index()) else { continue };
            let offset = sign_extend(value.get_offset() as i64, stack.get_addr_size() as int4 * 8 - 1) as i128;
            let end = offset.checked_add(value.get_size() as i128)?;
            if let Some(first) = first {
                if offset >= first + size as i128 || end <= first { continue; }
            } else if at_or_below(fd, pointer, entry, offset - size as i128, None, None, budget)
                || at_or_below(fd, entry, pointer, -end, None, None, budget)
            { continue; }
            op.get_out()?
        };
        add_source(payload, sources, budget, proof);
    }
    (*budget > 0).then_some(false)
}


/// A value built only from narrower-than-address scalars: a narrow load or
/// constant, a zero/sign extension or a truncation of a non-frame value, and
/// one such value combined with constants. Two variable operands are never
/// combined at full width, so bytes recombined into an address are not numbers.
fn number_tree(fd: &Funcdata, value: VarnodeId, budget: &mut usize, proof: &mut SavedFrameQuery) -> bool {
    let Some(stack) = fd.get_arch().manage().get_stack_space() else { return false };
    let width = stack.get_addr_size() as int4;
    let Some(left) = budget.checked_sub(1) else { return false };
    *budget = left;
    let Some(node) = fd.vbank().get(value) else { return false };
    if node.is_constant() { return true; }
    let Some(op) = node.get_def().and_then(|id| fd.obank().get(id)) else { return false };
    // Arithmetic whose result is narrower than an address, over numbers, is a
    // number: an address reaches narrow width only by truncation (SUBPIECE,
    // checked below). Combining two variable operands can reassemble byte
    // pieces of a stored address, so that needs non-frame provenance.
    if node.get_size() < width && !op.is_marker()
        && !matches!(op.code(), OpCode::CPUI_LOAD | OpCode::CPUI_SUBPIECE | OpCode::CPUI_CALL
            | OpCode::CPUI_CALLIND | OpCode::CPUI_CALLOTHER | OpCode::CPUI_PIECE | OpCode::CPUI_SEGMENTOP)
        && op.num_input() > 0
    {
        let variables = (0..op.num_input()).filter(|&slot| op.get_in(slot)
            .and_then(|input| fd.vbank().get(input)).is_some_and(|v| !v.is_constant())).count();
        if (0..op.num_input()).all(|slot| op.get_in(slot).is_some_and(|input|
            fd.vbank().get(input).is_some_and(|v| v.is_constant()) || number_tree(fd, input, budget, proof)))
            && (variables < 2 || !frame_dependent(fd, value, budget, FrameWalk::Values, proof))
        { return true; }
    }
    match op.code() {
        // A single loaded byte is a number: rebuilding an address from it needs
        // the remaining bytes as a constant, i.e. code that knows its own stack
        // address. A wider narrow load must not carry a stored address piece.
        OpCode::CPUI_LOAD => node.get_size() < width
            && (node.get_size() == 1 || !frame_dependent(fd, value, budget, FrameWalk::Values, proof)),
        OpCode::CPUI_INT_ZEXT | OpCode::CPUI_INT_SEXT =>
            op.get_in(0).is_some_and(|input| number_tree(fd, input, budget, proof)),
        OpCode::CPUI_SUBPIECE => node.get_size() < width
            && op.get_in(0).is_some_and(|input| number_tree(fd, input, budget, proof)
                || !frame_dependent(fd, input, budget, FrameWalk::Values, proof)),
        // A merge selects one of its inputs and recombines none of them, so
        // it is a number when each input is.
        OpCode::CPUI_MULTIEQUAL => op.num_input() > 0
            && (0..op.num_input()).all(|slot| op.get_in(slot).is_some_and(|input|
                fd.vbank().get(input).is_some_and(|v| v.is_constant()) || number_tree(fd, input, budget, proof))),
        // A predicate is a Boolean whatever it compares; it carries no address.
        OpCode::CPUI_INT_EQUAL | OpCode::CPUI_INT_NOTEQUAL | OpCode::CPUI_INT_LESS
        | OpCode::CPUI_INT_SLESS | OpCode::CPUI_INT_LESSEQUAL | OpCode::CPUI_INT_SLESSEQUAL
        | OpCode::CPUI_INT_CARRY | OpCode::CPUI_INT_SCARRY | OpCode::CPUI_INT_SBORROW
        | OpCode::CPUI_BOOL_NEGATE | OpCode::CPUI_BOOL_AND | OpCode::CPUI_BOOL_OR | OpCode::CPUI_BOOL_XOR
        | OpCode::CPUI_FLOAT_EQUAL | OpCode::CPUI_FLOAT_NOTEQUAL | OpCode::CPUI_FLOAT_LESS
        | OpCode::CPUI_FLOAT_LESSEQUAL | OpCode::CPUI_FLOAT_NAN => true,
        OpCode::CPUI_INT_2COMP | OpCode::CPUI_INT_NEGATE =>
            op.get_in(0).is_some_and(|input| number_tree(fd, input, budget, proof)),
        OpCode::CPUI_INT_ADD | OpCode::CPUI_INT_SUB | OpCode::CPUI_INT_AND | OpCode::CPUI_INT_OR
        | OpCode::CPUI_INT_XOR | OpCode::CPUI_INT_MULT | OpCode::CPUI_INT_LEFT | OpCode::CPUI_INT_RIGHT
        | OpCode::CPUI_INT_SRIGHT | OpCode::CPUI_INT_DIV | OpCode::CPUI_INT_REM => {
            let constant = |slot| op.get_in(slot).and_then(|input| fd.vbank().get(input)).is_some_and(|v| v.is_constant());
            let variables = (0..op.num_input()).filter(|&slot| !constant(slot)).count();
            // Two variable numbers can reassemble address pieces; only
            // non-frame provenance rules that out.
            variables >= 1
                && (0..op.num_input()).all(|slot| constant(slot)
                    || op.get_in(slot).is_some_and(|input| number_tree(fd, input, budget, proof)))
                && (variables == 1 || !frame_dependent(fd, value, budget, FrameWalk::Values, proof))
        }
        OpCode::CPUI_COPY | OpCode::CPUI_CAST => op.get_in(0).is_some_and(|input| number_tree(fd, input, budget, proof)),
        _ => false,
    }
}

fn frame_pointer_reaches(
    fd: &Funcdata, value: VarnodeId, first: i128, size: int4,
    budget: &mut usize, proof: &mut SavedFrameQuery,
) -> bool {
    // A number built from narrow scalars is not an address into the frame.
    // Draw from the caller's budget: number_tree can reach frame_dependent,
    // which can come back here through load_frame_sources.
    let start = (*budget).min(256);
    let mut local = start;
    let number = number_tree(fd, value, &mut local, proof);
    *budget -= start - local;
    if number { return false; }
    if !frame_dependent(fd, value, budget, FrameWalk::OpaqueLoads, proof) { return false; }
    let (offset, value) = frame_offset_saved(fd, value, budget, proof);
    if let Some(offset) = offset {
        return offset >= first && offset < first + size as i128;
    }
    // Prove exact object ownership before expanding conservative memory
    // origins, which can encounter this same pointer through its spill.
    // Source objects cannot legitimately expose a fresh compiler checkpoint.
    if owned_source_address(fd, value, budget, proof) { return false; }
    frame_dependent(fd, value, budget, FrameWalk::Values, proof)
}

fn private_frame_slot(
    fd: &Funcdata, first: i128, size: int4, stop: FrameRead, birth: Option<OpId>, expected: Option<VarnodeId>,
    budget: &mut usize, proof: &mut SavedFrameQuery,
) -> Option<()> {
    let stack = fd.get_arch().manage().get_stack_space()?;
    if stack.get_word_size() != 1 || stack.get_contain().is_none_or(|space| space.get_word_size() != 1) {
        return None;
    }
    let bias = stack_bias(fd, stack.get_addr_size() as int4) as i128;
    if !matches!(size, 1 | 2 | 4 | 8) || first.checked_add(size as i128)? > bias { return None; }
    // An address escaped before assignment can still alias the new value.
    // Earlier writes, unlike earlier escapes, do not invalidate that value.
    let mut live = frame_word_uses(fd, stop, birth, budget)?;
    if let FrameRead::Operation(stop) = stop {
        if fd.obank().get(stop).is_some_and(|op| matches!(op.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND | OpCode::CPUI_STORE)) {
            live.insert(stop, true);
        }
    }
    for (id, active_value) in live {
        *budget = budget.checked_sub(1)?;
        let op = fd.obank().get(id)?;
        if op.is_dead() { continue; }
        if active_value && (!matches!(op.code(), OpCode::CPUI_MULTIEQUAL | OpCode::CPUI_INDIRECT)
            || op.is_indirect_creation()) {
            if let Some(out) = op.get_out().and_then(|value| fd.vbank().get(value))
                .filter(|out| out.get_space().get_index() == stack.get_index())
            {
                let offset = sign_extend(out.get_offset() as i64, stack.get_addr_size() as int4 * 8 - 1) as i128;
                if offset < first + size as i128 && offset + out.get_size() as i128 > first {
                    if op.is_indirect_creation() { return None; }
                    if offset == first && out.get_size() == size && expected.is_some_and(|value|
                        native_stack_delta(fd, op.get_out().unwrap(), value) == Some(0))
                    { continue; }
                    return None;
                }
            }
        }
        let inputs = match op.code() {
            OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => {
                if active_value {
                    let call = fd.get_call_specs(fd.get_call_specs_index(id)?);
                    let limit = call_stack_upper(fd, id, budget, proof)?.checked_add(call_frame_floor(fd, call)?)?;
                    if first < limit { return None; }
                }
                int4::from(op.code() != OpCode::CPUI_CALLIND)..op.num_input()
            }
            OpCode::CPUI_CALLOTHER => {
                if active_value { return None; }
                0..op.num_input()
            },
            OpCode::CPUI_STORE => {
                let pointer = op.get_in(1)?;
                if active_value {
                if let Some(offset) = frame_offset(fd, pointer, budget, proof) {
                    let bytes = fd.vbank().get(op.get_in(2)?)?.get_size() as i128;
                    if offset == first && bytes == size as i128 && expected.is_some_and(|value|
                        native_stack_delta(fd, op.get_in(2).unwrap(), value) == Some(0))
                    { continue; }
                    if offset < first + size as i128 && offset.checked_add(bytes)? > first { return None; }
                } else if frame_pointer_reaches(fd, pointer, first, size, budget, proof) { return None; }
                }
                // A certified native backchain payload is bookkeeping, not
                // exposure of the compiler's saved-word storage. The actual
                // destination overlap check above still applies.
                if native_frame_link_payload(fd, id, stack.get_addr_size() as int4)
                    || ppc_backchain_restore_store(fd, id, stack.get_addr_size() as int4, budget, proof)
                        .is_some_and(|(root, _, restore)| {
                            let Some(value) = fd.obank().get(restore).and_then(|op| op.get_out()) else { return false };
                            // This is conditional on the word proof currently
                            // being discharged; the planner proves the actual read.
                            let (known, offset) = resolved_base(fd, value, budget, proof);
                            let (before, before_offset) = resolved_base(fd, root.before, budget, proof);
                            native_stack_delta(fd, known, before)
                                .and_then(|delta| delta.checked_add(offset)?.checked_sub(before_offset))
                                .is_some_and(|delta| sign_extend(delta as i64, stack.get_addr_size() as int4 * 8 - 1) == 0)
                        })
                { 2..2 } else { 2..3 }
            }
            // Heritage markers describe memory state, not a source assignment
            // or an address escape. Their real STORE/CALL is checked separately.
            OpCode::CPUI_MULTIEQUAL | OpCode::CPUI_INDIRECT => 0..0,
            _ if op.get_out().and_then(|value| fd.vbank().get(value)).is_some_and(|out|
                out.is_addr_tied() && stack.get_contain().is_some_and(|memory|
                    out.get_space().get_index() == memory.get_index())) => 0..op.num_input(),
            _ => 0..0,
        };
        for slot in inputs {
            if frame_pointer_reaches(fd, op.get_in(slot)?, first, size, budget, proof) { return None; }
        }
    }
    Some(())
}

fn anchored_word_escape(
    fd: &Funcdata, value: VarnodeId, pointer: VarnodeId, size: int4,
    budget: &mut usize, proof: &mut SavedFrameQuery,
) -> bool {
    let start = (*budget).min(256);
    let mut local = start;
    let number = number_tree(fd, value, &mut local, proof);
    *budget -= start - local;
    if number { return false; }
    if let Some(delta) = native_stack_delta(fd, value, pointer) {
        return delta < size as i128;
    }
    let (anchor, offset) = resolved_base(fd, pointer, budget, proof);
    let (value_anchor, value_offset) = resolved_base(fd, value, budget, proof);
    if stack_upper(fd, anchor, value_anchor, None, None, budget)
        .and_then(|upper| upper.checked_add(offset)?.checked_add(size as i128)?.checked_sub(value_offset))
        .is_some_and(|end| end <= 0)
    { return false; }
    if !frame_dependent(fd, value, budget, FrameWalk::OpaqueLoads, proof) { return false; }
    if owned_source_address(fd, value, budget, proof) { return false; }
    frame_dependent(fd, value, budget, FrameWalk::Values, proof)
}

/// Resolve one saved value through raw memory or its ordinary stack-SSA version.
/// Calls need a caller-private slot, not read-only/non-escape contracts on buffers.
fn unchanged_raw_slot(
    fd: &Funcdata, pointer: VarnodeId, space: i128, size: int4,
    read: OpId, birth: OpId, budget: &mut usize, proof: &mut SavedFrameQuery,
) -> Option<()> {
    let uses = frame_word_uses(fd, FrameRead::Operation(read), Some(birth), budget)?;
    // Before the write, publication matters only when a later call can use it
    // while the saved word is live. A load already captured in SSA is independent.
    let crosses_call = uses.iter().any(|(&id, &live)| live && fd.obank().get(id)
        .is_some_and(|op| !op.is_dead() && matches!(op.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND)));
    for (id, live) in uses {
        if !crosses_call && !live { continue; }
        *budget = budget.checked_sub(1)?;
        let op = fd.obank().get(id)?;
        if op.is_dead() { continue; }
        match op.code() {
            OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => {
                let call = fd.get_call_specs(fd.get_call_specs_index(id)?);
                let floor = call_frame_floor(fd, call)?;
                if live {
                    let sp = fd.kuna_stack_call_sp.get(&id).copied().flatten()?;
                    if native_stack_delta(fd, pointer, sp)? < floor { return None; }
                }
                for slot in int4::from(op.code() != OpCode::CPUI_CALLIND)..op.num_input() {
                    if anchored_word_escape(fd, op.get_in(slot)?, pointer, size, budget, proof) { return None; }
                }
            }
            OpCode::CPUI_CALLOTHER => return None,
            OpCode::CPUI_STORE => {
                if live && number(fd, op.get_in(0)?)? == space {
                    let delta = native_stack_delta(fd, op.get_in(1)?, pointer)?;
                    let bytes = fd.vbank().get(op.get_in(2)?)?.get_size() as i128;
                    if delta < size as i128 && delta.checked_add(bytes)? > 0 { return None; }
                }
                if crosses_call && id != birth && anchored_word_escape(fd, op.get_in(2)?, pointer, size, budget, proof) { return None; }
            }
            _ if live && !matches!(op.code(), OpCode::CPUI_MULTIEQUAL | OpCode::CPUI_INDIRECT)
                && op.get_out().and_then(|value| fd.vbank().get(value)).is_some_and(|out|
                    (out.is_addr_tied() && out.get_space().get_index() as i128 == space)
                    || fd.get_arch().manage().get_stack_space().is_some_and(|stack|
                        out.get_space().get_index() == stack.get_index())) => return None,
            _ if !live && op.get_out().and_then(|value| fd.vbank().get(value)).is_some_and(|out|
                out.is_addr_tied() && out.get_space().get_index() as i128 == space) => {
                for slot in 0..op.num_input() {
                    if anchored_word_escape(fd, op.get_in(slot)?, pointer, size, budget, proof) { return None; }
                }
            }
            _ => (),
        }
    }
    Some(())
}

#[derive(Clone, Copy)]
struct RawWordWriter { op: OpId, value: VarnodeId }

/// Nearest dominating full-width write. Marker versions may supply equality
/// facts, but only physical writes kill older provenance for the may query.
#[derive(Clone, Copy)]
enum WordQuery { Value, Origin }

fn frame_word_effect(fd: &Funcdata, marker: OpId) -> Option<OpId> {
    let op = fd.obank().get(marker)?;
    let effect = fd.vbank().get(op.get_in(1)?)?;
    let id = crate::funcdata_varnode::op_iop_decode(effect.get_offset());
    if fd.get_call_specs_index(id).is_some() { return Some(id); }
    let effect = fd.obank().get(id)?;
    let stack = fd.get_arch().manage().get_stack_space()?;
    (effect.code() == OpCode::CPUI_STORE
        && effect.get_in(0).and_then(|value| number(fd, value))
            == stack.get_contain().map(|space| space.get_index() as i128)).then_some(id)
}

fn incoming_frame_word(
    fd: &Funcdata, mut value: VarnodeId, budget: &mut usize,
) -> Option<VarnodeId> {
    loop {
        let node = fd.vbank().get(value)?;
        let Some(op) = node.get_def().and_then(|id| fd.obank().get(id)) else { return Some(value) };
        if op.code() != OpCode::CPUI_INDIRECT || op.is_indirect_creation() || node.is_volatile()
            || frame_word_effect(fd, node.get_def()?).is_none()
        { return Some(value); }
        let Some(input) = op.get_in(0).filter(|&input| fd.vbank().get(input)
            .is_some_and(|input| input.get_size() == node.get_size() && input.get_addr() == node.get_addr()))
            else { return Some(value) };
        *budget = budget.checked_sub(1)?;
        value = input;
    }
}

fn raw_word_writer(
    fd: &Funcdata, read: OpId, pointer: VarnodeId, memory: i128, first: Option<i128>,
    query: WordQuery, budget: &mut usize, proof: &mut SavedFrameQuery,
) -> Option<RawWordWriter> {
    let size = fd.vbank().get(fd.obank().get(read)?.get_out()?)?.get_size();
    if size <= 0 { return None; }
    let covers = |delta: i128, bytes: int4| bytes > 0 && match query {
        WordQuery::Value => delta == 0 && bytes == size,
        WordQuery::Origin => delta <= 0 && delta.checked_add(bytes as i128).is_some_and(|end| end >= size as i128),
    };
    let stack = fd.get_arch().manage().get_stack_space();
    let mut block = fd.obank().get(read)?.get_parent()?;
    let mut previous = fd.obank().get(read)?.basic_neighbours().0;
    loop {
        *budget = budget.checked_sub(1)?;
        let Some(write) = previous else {
            let parent = fd.bblocks_ref().block(block).get_immed_dom()?;
            if parent == block { return None; }
            block = parent;
            previous = fd.bb_op_tail(block);
            continue;
        };
        let stored = fd.obank().get(write)?;
        previous = stored.basic_neighbours().0;
        if stored.get_parent() != Some(block) { return None; }
        if stored.code() == OpCode::CPUI_STORE {
            if number(fd, stored.get_in(0)?)? != memory { continue; }
            let value = stored.get_in(2)?;
            let bytes = fd.vbank().get(value)?.get_size();
            let target = stored.get_in(1)?;
            let delta = native_stack_delta(fd, target, pointer).or_else(||
                frame_offset(fd, target, budget, proof).zip(first).and_then(|(target, first)| target.checked_sub(first)));
            if delta.is_some_and(|delta| covers(delta, bytes)) {
                return Some(RawWordWriter { op: write, value });
            }
        } else if (matches!(query, WordQuery::Value) || !stored.is_marker()) && stack.is_some_and(|stack|
            stack.get_contain().is_some_and(|space| space.get_index() as i128 == memory))
        {
            if let Some(value) = stored.get_out().and_then(|out| fd.vbank().get(out))
                .filter(|out| out.get_space().get_index() == stack.unwrap().get_index())
            {
                let offset = sign_extend(value.get_offset() as i64, stack.unwrap().get_addr_size() as int4 * 8 - 1) as i128;
                if first.is_some_and(|first| covers(offset - first, value.get_size())) {
                    let candidate = incoming_frame_word(fd, stored.get_out().unwrap(), budget)?;
                    return Some(RawWordWriter { op: write, value: candidate });
                }
            }
        }
        // A candidate is not a value equation. The existing all-path
        // preservation check still validates every intervening write/effect.
    }
}

/// Dense index of the values one [`FrameEquations`] covers, by arena slot.
struct ValueIndex(slotmap::SecondaryMap<VarnodeId, usize>);

impl ValueIndex {
    fn get(&self, value: &VarnodeId) -> Option<&usize> { self.0.get(*value) }
}

/// Candidate SSA value equations. Memory aliases are supplied only by the
/// word-use graph; they remain provisional until its effects are validated.
/// Removing a failed memory alias rebuilds these classes, rather than caching
/// an assumption-dependent answer in a recursive query.
struct FrameEquations {
    values: Vec<VarnodeId>,
    index: ValueIndex,
    parent: Vec<usize>,
    rank: Vec<u8>,
    anchor: Vec<usize>,
    opaque: Vec<bool>,
}

impl FrameEquations {
    fn new(fd: &Funcdata) -> Self {
        let mut values = BTreeSet::new();
        for id in fd.obank().iter_alive() {
            let Some(op) = fd.obank().get(id) else { continue };
            values.extend(op.get_out());
            values.extend((0..op.num_input()).filter_map(|slot| op.get_in(slot)));
        }
        values.extend(fd.kuna_stack_scope_holds.iter().copied());
        let values: Vec<_> = values.into_iter().collect();
        let index = ValueIndex(values.iter().enumerate().map(|(index, &value)| (value, index)).collect());
        let count = values.len();
        Self { values, index, parent: (0..count).collect(), rank: vec![0; count], anchor: (0..count).collect(), opaque: vec![false;count] }
    }

    fn class(&self, mut index: usize) -> usize {
        while self.parent[index] != index { index = self.parent[index]; }
        index
    }

    fn representative(&self, value: VarnodeId) -> VarnodeId {
        self.index.get(&value).map_or(value, |&index| self.values[self.anchor[self.class(index)]])
    }

    fn anchor_priority(fd: &Funcdata, value: VarnodeId) -> (u8,VarnodeId) {
        let priority = fd.vbank().get(value).map_or(4, |node| {
            node.get_def().and_then(|id|fd.obank().get(id)).map_or(0, |op|match op.code() {
                OpCode::CPUI_MULTIEQUAL => 1,
                OpCode::CPUI_LOAD => 2,
                OpCode::CPUI_COPY | OpCode::CPUI_CAST => {
                    if op.get_in(0).and_then(|v|fd.vbank().get(v)).is_some_and(|source|
                        fd.get_arch().manage().get_stack_space().is_some_and(|stack|
                            source.get_space().get_index()==stack.get_index()
                            || (source.is_addr_tied() && stack.get_contain().is_some_and(|memory|
                                source.get_space().get_index()==memory.get_index())))) {2} else {3}
                }
                OpCode::CPUI_INDIRECT => 2,
                OpCode::CPUI_STORE => 3,
                _ => 0,
            })
        });
        (priority,value)
    }

    fn equate(&mut self, fd: &Funcdata, first: VarnodeId, second: VarnodeId) -> bool {
        if fd.vbank().get(first).zip(fd.vbank().get(second))
            .is_none_or(|(first,second)|first.get_size()!=second.get_size()) {return false;}
        let Some((&first, &second)) = self.index.get(&first).zip(self.index.get(&second)) else { return false };
        let (mut first, mut second) = (self.class(first), self.class(second));
        if first == second { return false; }
        if self.rank[first] < self.rank[second] { std::mem::swap(&mut first, &mut second); }
        self.parent[second] = first;
        self.opaque[first] |= self.opaque[second];
        if self.rank[first] == self.rank[second] { self.rank[first] += 1; }
        let left = self.anchor[first];
        let right = self.anchor[second];
        if Self::anchor_priority(fd, self.values[right]) < Self::anchor_priority(fd, self.values[left]) {
            self.anchor[first] = right;
        }
        true
    }

    fn pure_aliases(&mut self, fd: &Funcdata) {
        let stack = fd.get_arch().manage().get_stack_space();
        for id in fd.obank().iter_alive() {
            let Some(op) = fd.obank().get(id) else { continue };
            let Some(output) = op.get_out() else { continue };
            let Some(source) = op.get_in(0) else { continue };
            let Some((out, input)) = fd.vbank().get(output).zip(fd.vbank().get(source)) else { continue };
            if out.get_size() != input.get_size() || out.is_volatile() || input.is_volatile() { continue; }
            let read = stack.is_some_and(|stack| input.get_space().get_index() == stack.get_index());
            if matches!(op.code(), OpCode::CPUI_COPY | OpCode::CPUI_CAST) && !read && !input.is_addr_tied() {
                self.equate(fd, output, source);
            }
        }
    }

    /// Collapse only trivial PHIs with an external seed. A PHI
    /// selecting different values stays a symbolic PHI, not a failed constant
    /// query. Its predecessor structure remains available to may-address uses.
    fn trivial_phis<F: Fn(VarnodeId)->bool>(&mut self, fd: &Funcdata, eligible: F) {
        let phis: Vec<_> = fd.obank().iter_alive().filter(|&id|
            fd.obank().get(id).is_some_and(|op| op.code() == OpCode::CPUI_MULTIEQUAL)).collect();
        let mut users = vec![Vec::<OpId>::new(); self.values.len()];
        for &id in &phis {
            let op = fd.obank().get(id).unwrap();
            for value in op.get_out().into_iter().chain((0..op.num_input()).filter_map(|slot| op.get_in(slot))) {
                if let Some(&index) = self.index.get(&value) { users[self.class(index)].push(id); }
            }
        }
        let mut queue: std::collections::VecDeque<_> = phis.iter().copied().collect();
        let mut queued: BTreeSet<_> = phis.into_iter().collect();
        while let Some(id) = queue.pop_front() {
            queued.remove(&id);
            let op = fd.obank().get(id).unwrap();
            let Some(output) = op.get_out().filter(|&output|eligible(output)) else { continue };
            if !op.get_parent().is_some_and(|parent| fd.bblocks_ref().block(parent).size_in() == op.num_input()) { continue; }
            let Some(node) = fd.vbank().get(output).filter(|node| !node.is_volatile()) else { continue };
            let Some(&output_index) = self.index.get(&output) else { continue };
            let output_class = self.class(output_index);
            let mut seed = None;
            let mut conflicting = false;
            for slot in 0..op.num_input() {
                let Some(input) = op.get_in(slot) else { conflicting = true; break };
                if fd.vbank().get(input).is_none_or(|input| input.get_size() != node.get_size() || input.is_volatile()) {
                    conflicting = true; break;
                }
                let Some(&input_index) = self.index.get(&input) else { conflicting = true; break };
                let input_class = self.class(input_index);
                if input_class == output_class { continue; }
                if seed.is_some_and(|previous| previous != input_class) { conflicting = true; break; }
                seed = Some(input_class);
            }
            if conflicting { continue; }
            let Some(seed) = seed else { continue };
            if !self.equate(fd, output, self.values[self.anchor[seed]]) { continue; }
            let winner = self.class(output_index);
            let absorbed = if winner == output_class { seed } else { output_class };
            let affected = std::mem::take(&mut users[absorbed]);
            // Only users of the absorbed class changed their input classes.
            // Union by rank bounds each dependency's moves logarithmically.
            for &user in &affected {
                if queued.insert(user) { queue.push_back(user); }
            }
            users[winner].extend(affected);
        }
    }
    fn value(&self, fd: &Funcdata, value: VarnodeId) -> (VarnodeId, bool) {
        let value = self.representative(value);
        let opaque = self.index.get(&value).is_some_and(|&index|self.opaque[self.class(index)])
            || fd.vbank().get(value).and_then(|node| node.get_def())
            .and_then(|id| fd.obank().get(id)).is_some_and(|op| match op.code() {
                OpCode::CPUI_LOAD => true,
                OpCode::CPUI_COPY | OpCode::CPUI_CAST => op.get_in(0).and_then(|input| fd.vbank().get(input))
                    .is_some_and(|input| fd.get_arch().manage().get_stack_space().is_some_and(|stack|
                        input.get_space().get_index() == stack.get_index()
                        || (input.is_addr_tied() && stack.get_contain().is_some_and(|memory|
                            input.get_space().get_index() == memory.get_index())))),
                OpCode::CPUI_INDIRECT => op.is_indirect_creation()
                    || op.get_out().and_then(|out|fd.vbank().get(out)).is_some_and(|out|
                        fd.get_arch().manage().get_stack_space().is_some_and(|stack|
                            out.get_space().get_index() == stack.get_index())),
                _ => false,
            });
        (value, opaque)
    }

    fn delta(&self, fd: &Funcdata, value: VarnodeId, anchor: VarnodeId) -> Option<i128> {
        let width = fd.vbank().get(anchor)?.get_size();
        if fd.vbank().get(value)?.get_size() != width { return None; }
        native_stack_sum_using(fd, width, vec![(value,1),(anchor,-1)], None, &|value| self.value(fd,value))
    }

    fn bounded_value(&self, fd: &Funcdata, roots: &[Allocation], value: VarnodeId) -> (VarnodeId,bool) {
        let (known,opaque)=self.value(fd,value);
        (known,opaque && !roots.iter().any(|root|negate(fd,root.amount).is_none()
            && self.representative(root.output)==known))
    }
}

#[derive(Clone, Copy)]
struct FrameWordUse {
    origin_only: bool,
    result: Option<VarnodeId>,
    source: VarnodeId,
    first: Option<i128>,
    pointer: Option<VarnodeId>,
    size: int4,
    // None is an intrinsic full-word definition, not a memory read to prove.
    read: Option<FrameRead>,
    birth: Option<OpId>,
    alias: bool,
}

impl FrameWordUse {
    fn equation_result(self, fd: &Funcdata) -> Option<VarnodeId> {
        if self.origin_only {return None;}
        // Arguments are evaluated before a call; a stale argument does not
        // invalidate the independent SSA value returned by that call.
        if matches!(self.read,Some(FrameRead::Operand(id,_)) if fd.obank().get(id)
            .is_some_and(|op|matches!(op.code(),OpCode::CPUI_CALL|OpCode::CPUI_CALLIND|OpCode::CPUI_CALLOTHER)))
        { None } else { self.result }
    }
}

struct FrameWordGraph {
    uses: Vec<FrameWordUse>,
    blocked: BTreeSet<VarnodeId>,
}

fn frame_word_birth(fd: &Funcdata, value: VarnodeId) -> Option<OpId> {
    let id = fd.vbank().get(value)?.get_def()?;
    if fd.obank().get(id)?.code() == OpCode::CPUI_INDIRECT { frame_word_effect(fd, id) }
    else { Some(id) }
}

impl FrameWordGraph {
    fn new(fd: &Funcdata) -> Self {
        let mut graph = Self { uses: Vec::new(), blocked: BTreeSet::new() };
        let Some(stack) = fd.get_arch().manage().get_stack_space() else { return graph };
        let offset = |value: VarnodeId| {
            fd.vbank().get(value).filter(|node| node.get_space().get_index() == stack.get_index())
                .map(|node| sign_extend(node.get_offset() as i64, stack.get_addr_size() as int4 * 8 - 1) as i128)
        };
        let entry = fd.find_spacebase_input(stack);
        let memory = stack.get_contain().map(|space| space.get_index() as i128);
        for id in fd.obank().iter_alive() {
            let Some(op) = fd.obank().get(id) else { continue };
            let Some(output) = op.get_out() else { continue };
            let Some(node) = fd.vbank().get(output) else { graph.blocked.insert(output); continue };
            if node.is_volatile() { graph.blocked.insert(output); continue; }
            match op.code() {
                OpCode::CPUI_COPY | OpCode::CPUI_CAST => {
                    let Some(source) = op.get_in(0) else { graph.blocked.insert(output); continue };
                    let Some(first) = offset(source) else { continue };
                    let Some(input) = fd.vbank().get(source).filter(|input|
                        input.get_size() == node.get_size() && !input.is_volatile())
                    else { graph.blocked.insert(output); continue };
                    graph.uses.push(FrameWordUse {origin_only: false,
                        result: Some(output), source, first: Some(first), pointer: None, size: input.get_size(),
                        read: Some(FrameRead::Operand(id,0)), birth: frame_word_birth(fd, source), alias: true,
                    });
                }
                OpCode::CPUI_STORE => {
                    let Some(first) = offset(output) else { continue };
                    let Some(source) = op.get_in(2).filter(|&source| fd.vbank().get(source).is_some_and(|input|
                        input.get_size() == node.get_size() && !input.is_volatile()))
                    else { graph.blocked.insert(output); continue };
                    graph.uses.push(FrameWordUse {origin_only: false,
                        result: Some(output), source, first: Some(first), pointer: None, size: node.get_size(),
                        read: None, birth: Some(id), alias: true,
                    });
                }
                OpCode::CPUI_INDIRECT => {
                    let Some(first) = offset(output) else { continue };
                    let source = op.get_in(0).filter(|&source| fd.vbank().get(source).is_some_and(|input|
                        input.get_size() == node.get_size() && !input.is_volatile()));
                    let effect = (!op.is_indirect_creation()).then(|| frame_word_effect(fd, id)).flatten();
                    let (Some(source), Some(effect)) = (source, effect)
                    else { graph.blocked.insert(output); continue };
                    graph.uses.push(FrameWordUse {origin_only: false,
                        result: Some(output), source, first: Some(first), pointer: None, size: node.get_size(),
                        read: Some(FrameRead::Operation(effect)), birth: frame_word_birth(fd, source), alias: true,
                    });
                }
                OpCode::CPUI_MULTIEQUAL => {
                    let Some(first) = offset(output) else { continue };
                    if !op.get_parent().is_some_and(|parent|
                        fd.bblocks_ref().block(parent).size_in() == op.num_input())
                    { graph.blocked.insert(output); continue; }
                    for slot in 0..op.num_input() {
                        let Some(source) = op.get_in(slot).filter(|&source| fd.vbank().get(source).is_some_and(|input|
                            input.get_size() == node.get_size() && !input.is_volatile()))
                        else { graph.blocked.insert(output); continue };
                        graph.uses.push(FrameWordUse {origin_only: false,
                            result: Some(output), source, first: Some(first), pointer: None, size: node.get_size(),
                            read: Some(FrameRead::Incoming(id, slot)), birth: frame_word_birth(fd, source), alias: false,
                        });
                    }
                }
                OpCode::CPUI_LOAD => {
                    if op.get_in(0).and_then(|value| number(fd, value)) != memory {
                        continue;
                    }
                    let Some(pointer) = op.get_in(1) else { continue };
                    let first = entry.and_then(|entry| native_stack_delta(fd, pointer, entry));
                    let writer = memory.and_then(|memory|
                        frame_word_source_using(fd,id,pointer,memory,first,&mut 65536,&|value|(value,false)));
                    let source = writer.and_then(|writer| {
                        let birth = if fd.obank().get(writer.op)?.code() == OpCode::CPUI_STORE {
                            Some(writer.op)
                        } else { frame_word_birth(fd, writer.value) };
                        Some(FrameWordUse {origin_only: false,
                            result: Some(output), source: writer.value, first, pointer: Some(pointer), size: node.get_size(),
                            read: Some(FrameRead::Operation(id)), birth,
                            alias: fd.vbank().get(writer.value).is_some_and(|source|source.get_size()==node.get_size()),
                        })
                    });
                    // An unresolved external read contributes provenance but no
                    // compiler-frame value equation.
                    graph.uses.push(source.unwrap_or(FrameWordUse {
                        origin_only: true, result: Some(output), source: output, first,
                        pointer: Some(pointer), size: node.get_size(),
                        read: Some(FrameRead::Operation(id)), birth: None, alias: false,
                    }));
                }
                _ => {}
            }
        }
        for id in fd.obank().iter_alive() {
            let Some(op)=fd.obank().get(id).filter(|op|!op.is_dead()&&!op.is_marker()) else {continue};
            for slot in 0..op.num_input() {
                let Some(source)=op.get_in(slot) else {continue};
                let Some(first)=offset(source) else {continue};
                if slot==0 && matches!(op.code(),OpCode::CPUI_COPY|OpCode::CPUI_CAST) {continue;}
                let Some(node)=fd.vbank().get(source) else {continue};
                graph.uses.push(FrameWordUse {origin_only: false,result:op.get_out(),source,first:Some(first),pointer:None,
                    size:node.get_size(),read:Some(FrameRead::Operand(id,slot)),birth:frame_word_birth(fd,source),alias:false});
            }
        }
        graph
    }
    fn refine_reads(&mut self, fd: &Funcdata, equations: &FrameEquations,
        bad_inputs: &BTreeSet<(OpId,int4)>, rejected: &BTreeSet<VarnodeId>) -> Vec<usize> {
        let Some(stack)=fd.get_arch().manage().get_stack_space() else {return Vec::new()};
        let Some(entry)=fd.find_spacebase_input(stack) else {return Vec::new()};
        let Some(memory)=stack.get_contain().map(|space|space.get_index() as i128) else {return Vec::new()};
        let mut changed=Vec::new();
        for (index,word) in self.uses.iter_mut().enumerate() {
            if !word.origin_only {continue;}
            let Some(FrameRead::Operation(read))=word.read else {continue};
            if bad_inputs.contains(&(read,1)) || word.result.is_some_and(|result|rejected.contains(&result)) {continue;}
            let Some(pointer)=word.pointer else {continue};
            let first=equations.delta(fd,pointer,entry);
            let Some(writer)=frame_word_source_using(fd,read,pointer,memory,first,&mut 65536,
                &|value|equations.value(fd,value)) else {continue};
            if rejected.contains(&writer.value) || self.blocked.contains(&writer.value)
                || fd.vbank().get(writer.value).is_none_or(|node|node.is_volatile() || node.get_size()!=word.size)
                || fd.obank().get(writer.op).is_some_and(|op|op.code()==OpCode::CPUI_STORE
                    && (bad_inputs.contains(&(writer.op,1)) || op.get_in(1)
                        .is_none_or(|target|equations.delta(fd,pointer,target)!=Some(0)))) {continue;}
            word.source=writer.value;
            word.first=first;
            word.birth=if fd.obank().get(writer.op).is_some_and(|op|op.code()==OpCode::CPUI_STORE) {
                Some(writer.op)
            } else {frame_word_birth(fd,writer.value)};
            word.alias=fd.vbank().get(writer.value).is_some_and(|node|node.get_size()==word.size);
            word.origin_only=false;
            changed.push(index);
        }
        changed
    }
}

fn frame_word_source_using<F: Fn(VarnodeId) -> (VarnodeId,bool)>(
    fd: &Funcdata, read: OpId, pointer: VarnodeId, memory: i128,
    first: Option<i128>, budget: &mut usize, values: &F,
) -> Option<RawWordWriter> {
    let size = fd.vbank().get(fd.obank().get(read)?.get_out()?)?.get_size();
    if !matches!(size, 1 | 2 | 4 | 8) { return None; }
    let stack = fd.get_arch().manage().get_stack_space();
    let mut block = fd.obank().get(read)?.get_parent()?;
    let mut previous = fd.obank().get(read)?.basic_neighbours().0;
    loop {
        *budget = budget.checked_sub(1)?;
        let Some(write) = previous else {
            let parent = fd.bblocks_ref().block(block).get_immed_dom()?;
            if parent == block { return None; }
            block = parent;
            previous = fd.bb_op_tail(block);
            continue;
        };
        let op = fd.obank().get(write)?;
        previous = op.basic_neighbours().0;
        if op.get_parent() != Some(block) { return None; }
        if op.code() == OpCode::CPUI_STORE {
            if number(fd, op.get_in(0)?) != Some(memory) { continue; }
            let value = op.get_in(2)?;
            let bytes=fd.vbank().get(value)?.get_size();
            if bytes>=size && native_stack_sum_using(fd,fd.vbank().get(pointer)?.get_size(),
                vec![(op.get_in(1)?,1),(pointer,-1)],None,values)
                .is_some_and(|delta|delta<=0 && delta.checked_add(bytes as i128).is_some_and(|end|end>=size as i128))
            {return Some(RawWordWriter {op:write,value});}
        } else if stack.is_some_and(|stack|
            stack.get_contain().is_some_and(|space| space.get_index() as i128 == memory))
        {
            let Some(value) = op.get_out().and_then(|value| fd.vbank().get(value)).filter(|value|
                value.get_space().get_index() == stack.unwrap().get_index() && value.get_size() == size)
            else { continue };
            let at = sign_extend(value.get_offset() as i64, stack.unwrap().get_addr_size() as int4 * 8 - 1) as i128;
            if first == Some(at) { return Some(RawWordWriter { op: write, value: op.get_out()? }); }
        }
    }
}
fn declared_scalar_output(fd: &Funcdata, call: OpId, size: int4) -> Option<&Rc<crate::dtype::Datatype>> {
    let proto=fd.get_call_specs(fd.get_call_specs_index(call)?).proto();
    proto.get_output_type().filter(|ty|proto.is_output_locked() && ty.get_size()==size
        && matches!(ty.get_metatype(),type_metatype::TYPE_INT|type_metatype::TYPE_UINT
            |type_metatype::TYPE_BOOL|type_metatype::TYPE_FLOAT))
}

/// May-origin facts are independent of invariant numeric values. A changing
/// scalar PHI can remain non-native; a newly discovered memory effect can only
/// add native-frame provenance, never remove it or leave old ownership cached.
#[derive(Clone, Copy)]
enum FrameAccessRule {
    Join,
    Manipulated,
    Source,
    Index { base: usize, index: usize, commutative: bool },
    Align(usize),
    ExternalRead(usize),
}

struct FrameOrigins {
    reads: BTreeMap<(OpId,int4),usize>,
    word_targets: Vec<Option<usize>>,
    inputs: Vec<Vec<usize>>,
    users: Vec<Vec<usize>>,
    phi: Vec<bool>,
    grounded: Vec<bool>,
    native: Vec<bool>,
    address_sized: Vec<bool>,
    private: Vec<bool>,
    // May fail compiler ownership; private alone is only a may-frame fact.
    non_compiler: Vec<bool>,
    rule: Vec<FrameAccessRule>,
    queue: std::collections::VecDeque<usize>,
    queued: Vec<bool>,
}

fn frame_read_initialized(fd: &Funcdata, word: FrameWordUse, equations: &FrameEquations,
    memory: i128, bad_inputs: &BTreeSet<(OpId,int4)>) -> bool {
    let Some(FrameRead::Operation(read))=word.read else {return false};
    let Some(pointer)=word.pointer else {return false};
    let Some(stack)=fd.get_arch().manage().get_stack_space() else {return false};
    let Some(entry)=fd.find_spacebase_input(stack) else {return false};
    let Some(parent)=fd.obank().get(read).and_then(|op|op.get_parent()) else {return false};
    if word.size<=0 {return false;}
    let first=equations.delta(fd,pointer,entry);
    let full=if word.size>=128 {u128::MAX} else {(1_u128<<word.size)-1};
    let cover=|missing:u128,delta:i128,bytes:int4| {
        if bytes<=0 {return missing;}
        let Some(end)=delta.checked_add(bytes as i128) else {return missing};
        if word.size>128 {
            return if delta<=0 && end>=word.size as i128 {0} else {missing};
        }
        let start=delta.max(0).min(word.size as i128) as u32;
        let end=end.max(0).min(word.size as i128) as u32;
        if start>=end {return missing;}
        let low=if start==128 {u128::MAX} else {(1_u128<<start)-1};
        let high=if end==128 {u128::MAX} else {(1_u128<<end)-1};
        missing & !(high & !low)
    };
    let mut todo=vec![(parent,Some(read),full)];
    let mut seen=BTreeSet::new();
    let mut budget=65536_usize;
    let mut witness=false;
    while let Some((block,before,mut missing))=todo.pop() {
        if !seen.insert((block,before,missing)) {continue;}
        let Some(left)=budget.checked_sub(1) else {return false};budget=left;
        let mut previous=before.and_then(|id|fd.obank().get(id).and_then(|op|op.basic_neighbours().0))
            .or_else(||before.is_none().then(||fd.bb_op_tail(block)).flatten());
        while let Some(id)=previous {
            let Some(left)=budget.checked_sub(1) else {return false};budget=left;
            let Some(op)=fd.obank().get(id) else {return false};
            previous=op.basic_neighbours().0;
            if op.is_dead() || op.is_marker() {continue;}
            let valid_store=op.code()!=OpCode::CPUI_STORE || !bad_inputs.contains(&(id,1))
                && op.get_in(0).and_then(|value|number(fd,value))==Some(memory);
            if !valid_store {continue;}
            if op.code()==OpCode::CPUI_STORE {
                if let Some((delta,bytes))=op.get_in(1).zip(op.get_in(2)).and_then(|(target,data)|
                    equations.delta(fd,target,pointer).zip(fd.vbank().get(data).map(|node|node.get_size())))
                {missing=cover(missing,delta,bytes);}
            }
            if let Some((node,first))=op.get_out().and_then(|value|fd.vbank().get(value))
                .filter(|node|node.get_space().get_index()==stack.get_index()).zip(first)
            {
                let at=sign_extend(node.get_offset() as i64,stack.get_addr_size() as int4*8-1) as i128;
                missing=cover(missing,at-first,node.get_size());
            }
            if missing==0 {witness=true;break;}
        }
        if missing==0 {continue;}
        let node=fd.bblocks_ref().block(block);
        if node.size_in()==0 {return false;}
        for slot in 0..node.size_in() {todo.push((node.get_in(slot),None,missing));}
    }
    witness
}


impl FrameOrigins {

    fn add_edge(&mut self, target: usize, source: usize) -> bool {
        if self.inputs[target].contains(&source) {return false;}
        self.inputs[target].push(source);
        self.users[source].push(target);
        self.enqueue(target);
        true
    }
    fn input_index(&self, fd: &Funcdata, equations: &FrameEquations, op: OpId, slot: int4) -> Option<usize> {
        self.reads.get(&(op,slot)).copied().or_else(||fd.obank().get(op)?.get_in(slot)
            .and_then(|value|equations.index.get(&value).copied()))
    }
    fn private_input(&self, fd: &Funcdata, equations: &FrameEquations, op: OpId, slot: int4) -> Option<bool> {
        self.input_index(fd,equations,op,slot).and_then(|index|self.grounded[index].then_some(self.private[index]))
    }
    fn non_native_input(&self, fd: &Funcdata, equations: &FrameEquations, op: OpId, slot: int4) -> bool {
        self.input_index(fd,equations,op,slot).is_some_and(|index|self.grounded[index]&&!self.native[index])
    }
    fn add_word_source(&mut self, equations: &FrameEquations, word: usize, value: VarnodeId) -> bool {
        self.word_targets[word].zip(equations.index.get(&value).copied())
            .is_some_and(|(target,source)|self.add_edge(target,source))
    }
    fn add_word_input(&mut self, fd: &Funcdata, equations: &FrameEquations, word: usize, op: OpId, slot: int4) -> bool {
        self.word_targets[word].zip(self.input_index(fd,equations,op,slot))
            .is_some_and(|(target,source)|self.add_edge(target,source))
    }
    fn unknown_word(&mut self, word: usize) -> bool {
        self.word_targets[word].is_some_and(|target|self.unknown_index(target))
    }
    fn manipulated_word(&mut self, word: usize) -> bool {
        let Some(target)=self.word_targets[word] else {return false};
        if matches!(self.rule[target],FrameAccessRule::Manipulated) {return false;}
        self.rule[target]=FrameAccessRule::Manipulated;
        self.enqueue(target);
        true
    }
    fn new(fd: &Funcdata, equations: &FrameEquations, words: &FrameWordGraph, roots: &[Allocation], bad_inputs: &BTreeSet<(OpId,int4)>) -> Self {
        let address_width=fd.get_arch().manage().get_stack_space().map_or(0,|stack|stack.get_addr_size() as int4);
        let value_count = equations.values.len();
        let mut reads=BTreeMap::new();
        for word in &words.uses {
            if let Some(FrameRead::Operand(op,slot))=word.read {
                let next=value_count+reads.len();
                reads.entry((op,slot)).or_insert(next);
            }
        }
        let count=value_count+reads.len();
        let word_targets=words.uses.iter().map(|word|match word.read {
            Some(FrameRead::Operand(op,slot))=>reads.get(&(op,slot)).copied(),
            _=>word.result.and_then(|value|equations.index.get(&value).copied()),
        }).collect();
        let mut origins = Self {
            reads,word_targets,
            inputs: vec![Vec::new(); count], users: vec![Vec::new(); count],
            phi: vec![false; count], grounded: vec![false; count], native: vec![false; count], address_sized: vec![true;count],
            private: vec![false; count], non_compiler: vec![false; count], rule: vec![FrameAccessRule::Join; count],
            queue: (0..count).collect(), queued: vec![true; count],
        };
        for (index,&value) in equations.values.iter().enumerate() {
            origins.address_sized[index]=fd.vbank().get(value).is_none_or(|node|node.get_size()>=address_width);
        }
        for (index,word) in words.uses.iter().enumerate() {
            if matches!(word.read,Some(FrameRead::Operand(..))) {
                if let Some(target)=origins.word_targets[index] {origins.address_sized[target]=word.size>=address_width;}
            }
        }
        for index in 0..value_count {
            let value=equations.values[index];
            let address=fd.vbank().get(value).and_then(|node|node.get_def()).and_then(|id|fd.obank().get(id))
                .is_some_and(|op|matches!(op.code(),OpCode::CPUI_INT_ADD|OpCode::CPUI_INT_SUB
                    |OpCode::CPUI_PTRADD|OpCode::CPUI_PTRSUB|OpCode::CPUI_INT_AND));
            if roots.iter().filter(|root|negate(fd,root.amount).is_none()).any(|root|
                value==root.output || (address && equations.delta(fd,value,root.output).is_some()))
            {
                origins.rule[index]=FrameAccessRule::Source;
                origins.non_compiler[index]=true;
                origins.native[index]=true;
            }
        }
        let memory: BTreeMap<_,_>=words.uses.iter().enumerate().filter(|(_,word)|word.alias || matches!(word.read,
            Some(FrameRead::Operation(id)) if fd.obank().get(id).is_some_and(|op|op.code()==OpCode::CPUI_LOAD))).filter_map(|(word_index,word)| {
            let result=word.result?;
            let source=word.birth.and_then(|birth|fd.obank().get(birth).filter(|op|
                op.code()==OpCode::CPUI_STORE && op.get_in(2)==Some(word.source)).map(|_|birth))
                .and_then(|birth|origins.reads.get(&(birth,2)).copied())
                .or_else(||equations.index.get(&word.source).copied())?;
            Some((result,(source,word.origin_only.then_some(word_index))))
        }).collect();
        for (index,word) in words.uses.iter().enumerate() {
            if matches!(word.read,Some(FrameRead::Operand(..))) {
                if let Some((target,source))=origins.word_targets[index]
                    .zip(equations.index.get(&word.source).copied()) {origins.add_edge(target,source);}
            }
        }
        let stack = fd.get_arch().manage().get_stack_space();
        let compiler_floor=stack.and_then(|stack|fd.find_spacebase_input(stack)).and_then(|entry|
            roots.iter().try_fold(i128::MIN,|floor,root|
                stack_upper_using(fd,root.upper_base(fd),entry,None,None,&mut 65536,&|value|equations.value(fd,value))
                    .map(|bound|floor.max(bound))));
        for index in 0..value_count {
            let value = equations.values[index];
            let Some(node) = fd.vbank().get(value) else {
                origins.unknown_index(index);
                continue;
            };
            if node.is_volatile() || words.blocked.contains(&value) {
                origins.unknown_index(index);
                continue;
            }
            if node.is_constant() || node.is_input() {
                origins.grounded[index] = true;
                let matches_base = |space: &kuna_base::space::AddrSpace| {
                    (0..space.num_spacebase()).any(|base|space.get_spacebase(base).is_ok_and(|storage|
                        storage.size as int4==node.get_size() && storage.offset==node.get_offset()
                            && storage.space.as_ref().is_some_and(|space|
                                space.get_index()==node.get_space().get_index())))
                };
                let stack_base=stack.is_some_and(|space|matches_base(space));
                let foreign_base=node.is_spacebase() && !stack_base
                    && (0..fd.get_arch().manage().num_spaces()).any(|index|
                        fd.get_arch().manage().get_space(index).is_some_and(|space|
                            stack.is_none_or(|stack|space.get_index()!=stack.get_index()) && matches_base(space)));
                origins.native[index] |= node.is_spacebase() || stack_base;
                // A declared foreign base denotes another source address
                // region. An unclassified spacebase remains frame-capable.
                origins.private[index] = stack_base || (node.is_spacebase() && !foreign_base);
                origins.non_compiler[index] |= !stack_base;
                continue;
            }
            let Some(op) = node.get_def().and_then(|id| fd.obank().get(id)) else {
                origins.unknown_index(index);
                continue;
            };
            if op.is_indirect_creation() {
                origins.unknown_index(index);
                continue;
            }
            origins.phi[index] = op.code() == OpCode::CPUI_MULTIEQUAL;
            if !matches!(origins.rule[index], FrameAccessRule::Source) {
                origins.rule[index] = match op.code() {
                    OpCode::CPUI_COPY | OpCode::CPUI_CAST | OpCode::CPUI_LOAD
                        | OpCode::CPUI_INDIRECT | OpCode::CPUI_MULTIEQUAL | OpCode::CPUI_STORE
                        | OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => FrameAccessRule::Join,
                    _ => FrameAccessRule::Manipulated,
                };
                if let Some((base,_))=alignment_using(fd,node.get_def().unwrap(),&|value|equations.value(fd,value))
                    .filter(|&(base,_)|!bad_inputs.contains(&(node.get_def().unwrap(),1-base))) {
                    if let Some(base) = origins.input_index(fd,equations,node.get_def().unwrap(),base) {
                        origins.rule[index] = FrameAccessRule::Align(base);
                    }
                } else if matches!(op.code(), OpCode::CPUI_INT_ADD | OpCode::CPUI_INT_SUB | OpCode::CPUI_PTRADD | OpCode::CPUI_PTRSUB)
                    && (op.code() != OpCode::CPUI_PTRADD || !bad_inputs.contains(&(node.get_def().unwrap(),2))
                        && op.get_in(2).and_then(|value|number_using(fd,value,&|value|equations.value(fd,value))).is_some())
                {
                    if let Some((base,element))=origins.input_index(fd,equations,node.get_def().unwrap(),0)
                        .zip(origins.input_index(fd,equations,node.get_def().unwrap(),1))
                    {
                        origins.rule[index] = FrameAccessRule::Index {
                            base, index: element, commutative: op.code() == OpCode::CPUI_INT_ADD,
                        };
                    }
                }
            }
            if matches!(origins.rule[index],FrameAccessRule::Manipulated|FrameAccessRule::Align(_)|FrameAccessRule::Index {..})
                && stack.and_then(|stack|fd.find_spacebase_input(stack))
                    .and_then(|entry|equations.delta(fd,value,entry))
                    .zip(compiler_floor).is_none_or(|(offset,floor)|offset<floor)
            { origins.non_compiler[index]=true; }
            let stack_output = stack.is_some_and(|stack| node.get_space().get_index() == stack.get_index());
            let stack_source = op.get_in(0).and_then(|value| fd.vbank().get(value)).is_some_and(|input|
                stack.is_some_and(|stack| input.get_space().get_index() == stack.get_index()));
            let read = op.code() == OpCode::CPUI_LOAD
                || (matches!(op.code(), OpCode::CPUI_COPY | OpCode::CPUI_CAST) && stack_source)
                || (matches!(op.code(), OpCode::CPUI_INDIRECT | OpCode::CPUI_STORE) && stack_output);
            if read {
                if matches!(op.code(),OpCode::CPUI_COPY|OpCode::CPUI_CAST) && stack_source {
                    if let Some(source)=origins.input_index(fd,equations,node.get_def().unwrap(),0) {origins.add_edge(index,source);}
                    else {origins.unknown_effect(equations,value);}
                } else if let Some(&(source,origin_only))=memory.get(&value) {
                    if let Some(word)=origin_only {
                        let initialized=stack.and_then(|stack|stack.get_contain())
                            .is_some_and(|space|frame_read_initialized(fd,words.uses[word],equations,
                                space.get_index() as i128,bad_inputs));
                        if initialized {
                            origins.grounded[index]=true;
                            origins.non_compiler[index]=true;
                        } else {origins.unknown_index(index);}
                    } else {origins.add_edge(index,source);}
                }
                else if op.code() == OpCode::CPUI_LOAD {
                    if !origins.address_sized[index] {origins.unknown_index(index);}
                    else if let Some(pointer)=origins.input_index(fd,equations,node.get_def().unwrap(),1) {
                        origins.rule[index]=FrameAccessRule::ExternalRead(pointer);
                        origins.non_compiler[index]=true;
                        origins.add_edge(index,pointer);
                    } else { origins.unknown_effect(equations, value); }
                } else { origins.unknown_effect(equations, value); }
            } else if matches!(op.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND) {
                // Preserve the existing scalar-return origin contract. Actual
                // call effects on frame words are separate memory transitions.
                origins.grounded[index] = true;
                origins.private[index] = origins.address_sized[index] && declared_scalar_output(fd,node.get_def().unwrap(),node.get_size()).is_none();
                origins.non_compiler[index] = true;
            } else if op.code() == OpCode::CPUI_INDIRECT {
                if let Some(source)=origins.input_index(fd,equations,node.get_def().unwrap(),0) {origins.add_edge(index,source);}
            } else {
                for slot in 0..op.num_input() {
                    if let Some(source)=origins.input_index(fd,equations,node.get_def().unwrap(),slot) {origins.add_edge(index,source);}
                }
            }
        }
        origins
    }

    fn enqueue(&mut self, index: usize) {
        if !self.queued[index] {
            self.queued[index] = true;
            self.queue.push_back(index);
        }
    }

    fn unknown_effect(&mut self, equations: &FrameEquations, value: VarnodeId) -> bool {
        let Some(&index) = equations.index.get(&value) else { return false };
        self.unknown_index(index)
    }

    fn unknown_index(&mut self, index: usize) -> bool {
        // Losing an exact byte/short value does not invent a full native
        // pointer. Explicit native-address fragments still flow through inputs.
        let native=self.native[index] || self.address_sized[index];
        let private=self.private[index] || self.address_sized[index];
        if native!=self.native[index] || private!=self.private[index] || !self.non_compiler[index] || !self.grounded[index] {
            self.non_compiler[index]=true;
            self.native[index]=native;
            self.private[index]=private;
            self.grounded[index]=true;
            for position in 0..self.users[index].len() {self.enqueue(self.users[index][position]);}
            return true;
        }
        false
    }

    fn settle(&mut self) {
        while let Some(index) = self.queue.pop_front() {
            self.queued[index] = false;
            let inputs = &self.inputs[index];
            let grounded = self.grounded[index] || if self.phi[index] {
                inputs.iter().any(|&input| self.grounded[input])
            } else {
                !inputs.is_empty() && inputs.iter().all(|&input| self.grounded[input])
            };
            let native = self.native[index] || inputs.iter().any(|&input| self.native[input])
                || matches!(self.rule[index], FrameAccessRule::ExternalRead(pointer) if self.private[pointer]);
            let joined_private = inputs.iter().any(|&input| self.private[input]);
            let source = |base: usize, element: usize| self.grounded[base] && self.grounded[element]
                && self.native[base] && !self.private[base] && !self.native[element];
            let private = self.private[index] || (grounded && match self.rule[index] {
                FrameAccessRule::Source => false,
                FrameAccessRule::Join => joined_private,
                FrameAccessRule::Manipulated => native || joined_private,
                FrameAccessRule::Align(base) => self.private[base],
                FrameAccessRule::ExternalRead(pointer) => self.native[pointer] || self.private[pointer],
                FrameAccessRule::Index {base,index:element,commutative} => {
                    if source(base,element) || (commutative && source(element,base)) { false }
                    else { native || joined_private }
                }
            });
            let non_compiler=self.non_compiler[index] || match self.rule[index] {
                FrameAccessRule::Align(base)=>self.non_compiler[base],
                FrameAccessRule::Index {base,index:element,..} if self.native[base] && !self.native[element]=>self.non_compiler[base],
                FrameAccessRule::Index {base,index:element,commutative:true} if self.native[element] && !self.native[base]=>self.non_compiler[element],
                _=>inputs.iter().any(|&input|self.non_compiler[input]),
            };
            let changed = non_compiler!=self.non_compiler[index] || grounded != self.grounded[index] || native != self.native[index] || private != self.private[index];
            self.non_compiler[index]=non_compiler;
            self.grounded[index] = grounded;
            self.native[index] = native;
            self.private[index] = private;
            if changed {
                for position in 0..self.users[index].len() { self.enqueue(self.users[index][position]); }
            }
        }
    }

    fn non_native(&self, equations: &FrameEquations, value: VarnodeId) -> bool {
        equations.index.get(&value).is_some_and(|&index| self.grounded[index] && !self.native[index])
    }

    fn must_private(&self, equations: &FrameEquations, value: VarnodeId) -> bool {
        equations.index.get(&value).is_some_and(|&index|
            self.grounded[index] && self.private[index] && !self.non_compiler[index])
    }
    fn private_address(&self, equations: &FrameEquations, value: VarnodeId) -> Option<bool> {
        equations.index.get(&value).and_then(|&index| self.grounded[index].then_some(self.private[index]))
    }

    fn seal_unknowns(&mut self) {
        for index in 0..self.grounded.len() {
            if !self.grounded[index] { self.unknown_index(index); }
        }
        self.settle();
    }

    
}

/// An operation a frame word's live span can be affected by or escape
/// through: a write into the stack space, a STORE, a call, a return, or a
/// write of an address-tied memory location. Every span consumer ignores the
/// rest (plain arithmetic and heritage markers), so spans keep only these;
/// a whole-frame word set times its live range otherwise grows quadratically.
fn span_effect(fd: &Funcdata, id: OpId) -> bool {
    let Some(op) = fd.obank().get(id) else { return false };
    if matches!(op.code(), OpCode::CPUI_STORE | OpCode::CPUI_CALL | OpCode::CPUI_CALLIND
        | OpCode::CPUI_CALLOTHER | OpCode::CPUI_RETURN) { return true; }
    if op.is_marker() && !op.is_indirect_creation() { return false; }
    let Some(stack) = fd.get_arch().manage().get_stack_space() else { return true };
    op.get_out().and_then(|value| fd.vbank().get(value)).is_some_and(|out|
        out.get_space().get_index() == stack.get_index()
            || (out.is_addr_tied() && stack.get_contain().is_some_and(|memory| out.get_space().get_index() == memory.get_index())))
}

struct FrameFacts {
    equations: FrameEquations,
    origins: FrameOrigins,
    words: FrameWordGraph,
    spans: Vec<Vec<(OpId, bool)>>,
    roots: Vec<Allocation>,
    entry: VarnodeId,
    memory: i128,
    calls: BTreeMap<OpId, Option<i128>>,
    transports: Vec<(OpId,usize,OpId,OpId,OpId)>,
    bad_inputs: BTreeSet<(OpId,int4)>,
    invalid_reads: BTreeSet<usize>,
    /// [`Self::upper`] at full budget, by value. Equations and roots are
    /// fixed for one epoch, and every effect pass asks the same pointers.
    full_uppers: std::cell::RefCell<BTreeMap<VarnodeId, Option<i128>>>,
}

impl FrameFacts {
    fn check_word(&self, fd: &Funcdata, word: FrameWordUse, span: &[(OpId,bool)]) -> bool {
        if word.origin_only {return self.binding(fd,word);}
        if fd.vbank().get(word.source).is_none_or(|node|node.is_volatile()) {return false;}
        if word.birth.and_then(|id|fd.obank().get(id).filter(|op|op.code()==OpCode::CPUI_STORE
            && op.get_in(2)==Some(word.source)).map(|_|id)).is_some_and(|id|!self.input_valid(id,2)) {return false;}
        if !self.binding(fd,word) {return false;}
        if word.read.is_none() {return true;}
        let Some(stack) = fd.get_arch().manage().get_stack_space() else { return false };
        if self.escaped_in_span(fd,word,span) { return false; }
        for &(id,live) in span {
            if !live { continue; }
            let Some(op) = fd.obank().get(id).filter(|op|!op.is_dead()) else { continue };
            if !op.is_marker() || op.is_indirect_creation() {
                if let Some(out) = op.get_out().and_then(|v|fd.vbank().get(v)).filter(|out|
                    out.get_space().get_index()==stack.get_index())
                {
                    let at = sign_extend(out.get_offset() as i64,stack.get_addr_size() as int4*8-1) as i128;
                    if self.stack_write_reaches(fd,at,out.get_size(),word) {
                        if op.is_indirect_creation() || self.first(fd,word)!=Some(at) || out.get_size()!=word.size
                            || self.equations.delta(fd,op.get_out().unwrap(),word.source)!=Some(0) { return false; }
                    }
                }
            }
            match op.code() {
                OpCode::CPUI_STORE if op.get_in(0).and_then(|v|number(fd,v))==Some(self.memory) => {
                    let Some((pointer,data)) = op.get_in(1).zip(op.get_in(2)) else { return false };
                    let bytes = fd.vbank().get(data).map_or(0,|v|v.get_size());
                    if self.store_reaches(fd,pointer,bytes,word,Some((id,1))).unwrap_or(true)
                        && !self.identical_store(fd,id,word) { return false; }
                }
                OpCode::CPUI_CALL | OpCode::CPUI_CALLIND if !self.call_preserves(fd,id,word) => return false,
                OpCode::CPUI_CALLOTHER => return false,
                _ => {}
            }
        }
        true
    }

    fn link_store(&self, fd: &Funcdata, root: &Allocation, id: OpId, width: int4) -> bool {
        if !ppc_backchain_store(fd,root,id,width) || !self.input_valid(id,1) || !self.input_valid(id,2) {return false;}
        let Some(op)=fd.obank().get(id) else {return false};
        let Some(pointer)=op.get_in(1) else {return false};
        let Some(data)=op.get_in(2) else {return false};
        if self.equations.delta(fd,pointer,root.output)!=Some(0) {return false;}
        let (known,_)=self.equations.value(fd,data);
        let parent=fd.vbank().get(known).and_then(|node|node.get_def()).and_then(|load|
            fd.obank().get(load).filter(|op|op.code()==OpCode::CPUI_LOAD).map(|op|(load,op)));
        let payload=self.equations.delta(fd,data,self.entry)==Some(0)
            || self.equations.delta(fd,data,root.before)==Some(0)
            || parent.is_some_and(|(id,load)|self.input_valid(id,1)
                && load.get_in(0).and_then(|value|number_using(fd,value,&|value|self.equations.value(fd,value)))==Some(self.memory)
                && load.get_in(1).is_some_and(|pointer|self.equations.delta(fd,pointer,root.before)==Some(0)));
        if !payload {return false;}
        let mut next=op.basic_neighbours().1;
        while let Some(id)=next {
            let Some(commit)=fd.obank().get(id) else {return false};
            if commit.get_addr()!=op.get_addr() || commit.get_parent()!=op.get_parent() {return false;}
            if commit.code()==OpCode::CPUI_COPY && commit.get_out().is_some_and(|value|stack_register(fd,value))
                && self.input_valid(id,0) && commit.get_in(0).is_some_and(|value|self.equations.delta(fd,value,root.output)==Some(0)) {return true;}
            next=commit.basic_neighbours().1;
        }
        false
    }
    fn header_live(&self, fd: &Funcdata, birth: OpId, read: FrameRead, stack: VarnodeId, budget: &mut usize) -> bool {
        let Some(span)=frame_word_uses(fd,read,Some(birth),budget) else {return false};
        for (id,live) in span {
            if !live {continue;}
            let Some(op)=fd.obank().get(id) else {return false};
            if op.is_dead() || op.is_marker() {continue;}
            if op.code()==OpCode::CPUI_CALLOTHER {return false;}
            if matches!(op.code(),OpCode::CPUI_CALL|OpCode::CPUI_CALLIND) {
                let Some(index)=fd.get_call_specs_index(id) else {return false};
                let call=fd.get_call_specs(index);
                if call_frame_floor(fd,call).is_none() {return false;}
                let Some(sp)=fd.kuna_stack_call_sp.get(&id).copied().flatten() else {return false};
                if !stack_upper_using(fd,sp,stack,None,None,budget,&|value|self.equations.value(fd,value))
                    .and_then(|upper|upper.checked_add(call.get_extra_pop() as i128)).is_some_and(|upper|upper<=0) {return false;}
            }
            if let Some(output)=op.get_out().filter(|&value|stack_register(fd,value)) {
                if !self.equations.delta(fd,output,stack).is_some_and(|delta|delta<=0)
                    && !stack_upper_using(fd,output,stack,None,None,budget,&|value|self.equations.value(fd,value))
                        .is_some_and(|upper|upper<=0) {return false;}
            }
        }
        true
    }
    fn header_address(&self, fd: &Funcdata, value: VarnodeId, read: FrameRead,
        links: &[(OpId,usize)], width: int4, budget: &mut usize) -> bool {
        enum Work {Visit(VarnodeId,FrameRead),Leave(VarnodeId)}
        let mut todo=vec![Work::Visit(value,read)];
        let mut active=BTreeSet::new();
        while let Some(work)=todo.pop() {
            let (value,read)=match work {Work::Visit(value,read)=>(value,read),Work::Leave(value)=>{active.remove(&value);continue;}};
            let Some(left)=budget.checked_sub(1) else {return false}; *budget=left;
            let (value,opaque)=self.equations.value(fd,value);
            let Some(node)=fd.vbank().get(value).filter(|node|node.get_size()==width && !node.is_volatile()) else {return false};
            if opaque {return false;}
            let matches=links.iter().any(|&(store,index)|{
                let root=&self.roots[index];
                let dominates=match read {FrameRead::Operation(id)|FrameRead::Operand(id,_)=>dominates_op(fd,store,id),
                    FrameRead::Incoming(id,slot)=>dominates_input(fd,store,id,slot)};
                dominates && self.equations.delta(fd,value,root.output)==Some(0)
                    && self.header_live(fd,store,read,root.output,budget)
            });
            if matches {continue;}
            let Some(id)=node.get_def() else {return false};
            let Some(op)=fd.obank().get(id).filter(|op|op.code()==OpCode::CPUI_MULTIEQUAL && op.num_input()>0
                && op.get_parent().is_some_and(|parent|fd.bblocks_ref().block(parent).size_in()==op.num_input())) else {return false};
            if !active.insert(value) || !self.header_live(fd,id,read,value,budget) {return false;}
            todo.push(Work::Leave(value));
            for slot in 0..op.num_input() {
                let Some(input)=op.get_in(slot) else {return false};
                todo.push(Work::Visit(input,FrameRead::Incoming(id,slot)));
            }
        }
        true
    }
    fn header_sp_sink(&self, fd: &Funcdata, id: OpId, native: &[(OpId,VarnodeId,bool)], budget: &mut usize) -> bool {
        let Some(op)=fd.obank().get(id).filter(|op|op.code()==OpCode::CPUI_LOAD) else {return false};
        if op.get_in(0).and_then(|value|number(fd,value))!=Some(self.memory) {return false;}
        let Some(output)=op.get_out() else {return false};
        let mut native_sink=false;
        let Some(pointer)=op.get_in(1).filter(|_|self.input_valid(id,1)) else {return false};
        // The header must be read at the SP live at this p-code point. An SP
        // write earlier in the same native instruction already applies here.
        let Some(current)=same_instruction_stack(fd,id).or_else(||preceding_stack(fd,id,budget)) else {return false};
        if self.equations.delta(fd,pointer,current)!=Some(0) {return false;}
        let mut todo=vec![output]; let mut seen=BTreeSet::new();
        while let Some(value)=todo.pop() {
            if !seen.insert(value) {continue;}
            let Some(left)=budget.checked_sub(1) else {return false}; *budget=left;
            let Some(node)=fd.vbank().get(value) else {return false};
            native_sink|=stack_register(fd,value);
            if node.is_volatile() || node.is_persist() || crate::kuna_stackalias::holds_store(fd,value)
                || crate::kuna_localstackviews::native_store(fd,value) {return false;}
            if node.is_auto_live_hold() && !native.iter().any(|&(_,held,owned)|held==value && owned) {return false;}
            if fd.kuna_stack_scope_holds.contains(&value) && !native.iter().any(|&(_,held,owned)|held==value && owned) {return false;}
            if fd.kuna_stack_call_sp.values().flatten().any(|&sp|
                self.equations.representative(sp)==self.equations.representative(value))
                || self.roots.iter().any(|root|[root.before,root.base,root.amount].contains(&value)) {return false;}
            for user in node.descend_iter() {
                let Some(op)=fd.obank().get(user) else {return false};
                if op.is_dead() {continue;}
                if !matches!(op.code(),OpCode::CPUI_COPY|OpCode::CPUI_CAST|OpCode::CPUI_INDIRECT|OpCode::CPUI_MULTIEQUAL)
                    || op.is_indirect_creation() {return false;}
                let Some(output)=op.get_out().filter(|&out|fd.vbank().get(out).is_some_and(|out|out.get_size()==node.get_size())) else {return false};
                todo.push(output);
            }
        }
        native_sink
    }

    fn input_value(&self, fd: &Funcdata, id: OpId, slot: int4) -> Option<VarnodeId> {
        if !self.input_valid(id,slot) {return None;}
        let input=fd.obank().get(id)?.get_in(slot)?;
        if fd.vbank().get(input)?.is_volatile() {return None;}
        let (value,opaque)=self.equations.value(fd,input);
        (!opaque).then_some(value)
    }

    fn affine_base(&self, fd: &Funcdata, mut value: VarnodeId, budget: &mut usize) -> Option<(VarnodeId,i128)> {
        let mut offset=0_i128;
        loop {
            *budget=budget.checked_sub(1)?;
            let (known,opaque)=self.equations.value(fd,value);
            if opaque {return None;}
            value=known;
            let Some((base,step))=constant_step_using(fd,value,&|value|self.equations.value(fd,value))
                else {return Some((value,offset))};
            offset=offset.checked_add(step)?;
            value=base;
        }
    }
    fn header_reaches(&self, fd: &Funcdata, value: VarnodeId, bytes: i128, allocation: &Allocation,
        width: int4, site: Option<(OpId,int4)>, budget: &mut usize) -> bool {
        self.header_reaches_at(fd,value,bytes,allocation.output,allocation.before,width,site,budget)
    }

    /// A restore of the pre-allocation SP: proved by the frame equations, or,
    /// for a reload the equations cannot follow (the SP saved in a frame slot),
    /// by the saved-word proof.
    fn restore_end(&self, fd: &Funcdata, allocation: &Allocation, value: VarnodeId) -> Option<LifetimeEnd> {
        self.restoration(fd, allocation, value).or_else(|| proved_restore(fd, allocation, value))
    }

    fn restoration(&self, fd: &Funcdata, allocation: &Allocation, value: VarnodeId) -> Option<LifetimeEnd> {
        if !fd.get_arch().stack_alloc_scopes || !stack_register(fd,value) {return None;}
        let id=fd.vbank().get(value)?.get_def()?;
        let op=fd.obank().get(id)?;
        if op.code()==OpCode::CPUI_MULTIEQUAL || !dominates_op(fd,allocation.op,id)
            || self.equations.delta(fd,value,allocation.before)!=Some(0) {return None;}
        Some(LifetimeEnd {restore:id,join:op.get_parent()?,boundary:None,
            address:op.get_addr().get_offset(),value})
    }


    fn input_valid(&self, op: OpId, slot: int4) -> bool { !self.bad_inputs.contains(&(op,slot)) }
    fn private_at(&self, fd: &Funcdata, value: VarnodeId, site: Option<(OpId,int4)>) -> Option<bool> {
        if let Some((op,slot))=site {self.origins.private_input(fd,&self.equations,op,slot)}
        else {self.origins.private_address(&self.equations,value)}
    }
    fn non_native_at(&self, fd: &Funcdata, value: VarnodeId, site: Option<(OpId,int4)>) -> bool {
        if let Some((op,slot))=site {self.origins.non_native_input(fd,&self.equations,op,slot)}
        else {self.origins.non_native(&self.equations,value)}
    }
    fn infer(fd: &Funcdata, mut roots: Vec<Allocation>) -> Option<Self> {
        let stack = fd.get_arch().manage().get_stack_space()?;
        if stack.get_word_size()!=1 || stack.get_contain()?.get_word_size()!=1 { return None; }
        let entry = fd.find_spacebase_input(stack)?;
        let memory = stack.get_contain()?.get_index() as i128;
        let mut words = FrameWordGraph::new(fd);
        let effects = BlockEffects::default();
        let mut spans = Vec::with_capacity(words.uses.len());
        let mut invalid_intervals = BTreeSet::new();
        let mut invalid_reads=BTreeSet::new();
        // Effect passes rescan these spans. Stop constructing them as soon as
        // the existing whole-function limit is exceeded.
        const SPAN_LIMIT: usize = 4_000_000;
        let mut total_span = 0;
        for (index,word) in words.uses.iter().enumerate() {
            let mut span = if let Some(read) = word.read {
                match frame_word_spans(fd,read,word.birth,&mut 65536,&effects) {
                    Some(uses) => uses,
                    None => {
                        invalid_reads.insert(index);
                        if let Some(result)=word.equation_result(fd) {invalid_intervals.insert(result);}
                        Vec::new()
                    }
                }
            } else { Vec::new() };
            if let Some(FrameRead::Operation(stop)) = word.read {
                if fd.obank().get(stop).is_some_and(|op|matches!(op.code(),
                    OpCode::CPUI_CALL|OpCode::CPUI_CALLIND|OpCode::CPUI_STORE))
                { span.push((stop,true)); }
            }
            total_span += span.len();
            if total_span > SPAN_LIMIT { return None; }
            spans.push(span);
        }
        words.blocked.extend(invalid_intervals);
        let mut rejected = BTreeSet::new();
        let mut bad_inputs=BTreeSet::new();
        loop {
            let mut equations = FrameEquations::new(fd);
            for &value in words.blocked.iter().chain(rejected.iter()) {
                if let Some(&index) = equations.index.get(&value) { equations.opaque[index]=true; }
            }
            equations.pure_aliases(fd);
            for word in &words.uses {
                if let Some(result)=word.result.filter(|result|word.alias
                    && !words.blocked.contains(result) && !words.blocked.contains(&word.source)
                    && !rejected.contains(result))
                {equations.equate(fd,result,word.source);}
            }
            equations.trivial_phis(fd,|value|!words.blocked.contains(&value)&&!rejected.contains(&value));
            let refined=words.refine_reads(fd,&equations,&bad_inputs,&rejected);
            if !refined.is_empty() {
                for index in refined {
                    let word=words.uses[index];
                    match word.read.and_then(|read|frame_word_spans(fd,read,word.birth,&mut 65536,&effects)) {
                        Some(uses)=>spans[index]=uses,
                        None=>{
                            invalid_reads.insert(index);
                            if let Some(result)=word.equation_result(fd) {words.blocked.insert(result);}
                        }
                    }
                }
                continue;
            }
            let origins = FrameOrigins::new(fd,&equations,&words,&roots,&bad_inputs);
            let mut facts = Self { equations,origins,words,spans,roots,entry,memory,calls:BTreeMap::new(),transports:Vec::new(),bad_inputs,invalid_reads,full_uppers:Default::default() };
            for (index,word) in facts.words.uses.iter().enumerate() {
                if facts.invalid_reads.contains(&index) || !facts.binding(fd,*word) {
                    facts.origins.unknown_word(index);
                }
            }
            for index in 0..fd.num_calls() {
                let call = fd.get_call_specs(index).get_op();
                let snapshot=fd.kuna_stack_call_sp.get(&call).copied().flatten()
                    .or_else(||fd.get_call_specs(index).get_spacebase_relative(fd));
                let upper=snapshot.and_then(|value|facts.upper(fd,value,&mut 65536))
                    .or_else(||preceding_stack(fd,call,&mut 65536)
                        .and_then(|value|facts.upper(fd,value,&mut 65536)));
                facts.calls.insert(call,upper);
            }
            // Structural ABI roles depend only on this equation/read epoch,
            // not on partially settled MAY origins. Rebuild after rejection.
            let width=stack.get_addr_size() as int4;
            for id in fd.obank().iter_alive() {
                if let Some((root,header,restore,load))=facts.backchain_transport(fd,id,width) {
                    facts.transports.push((id,root,header,restore,load));
                }
            }
            loop {
                facts.origins.settle();
                if !facts.feed_effects(fd) { break; }
            }
            facts.origins.seal_unknowns();
            while facts.feed_effects(fd) { facts.origins.settle(); }
            let mut changed = false;
            for index in 0..facts.words.uses.len() {
                let word=facts.words.uses[index];
                if !facts.preserved(fd,index) {
                    if let Some(FrameRead::Operand(op,slot)|FrameRead::Incoming(op,slot))=word.read {changed|=facts.bad_inputs.insert((op,slot));}
                    if let Some(result)=word.equation_result(fd) {changed|=rejected.insert(result);}
                }
            }
            if !changed { return Some(facts); }
            // Each unresolved read is promoted at most once; rejected value
            // candidates never return. Rebuild all equivalences and effects
            // after either change, without retaining assumption-dependent facts.
            let Self {words: next_words,spans: next_spans,roots:next_roots,bad_inputs:next_bad_inputs,invalid_reads:next_invalid_reads,..} = facts;
            words = next_words;
            spans = next_spans;
            roots = next_roots;
            bad_inputs=next_bad_inputs;
            invalid_reads=next_invalid_reads;
        }
    }

    fn unaligned(&self, fd: &Funcdata, mut value: VarnodeId, budget: &mut usize) -> Option<(VarnodeId,i128)> {
        let mut offset = 0_i128;
        loop {
            *budget = budget.checked_sub(1)?;
            let (current,opaque) = self.equations.value(fd,value);
            if opaque { return None; }
            value = current;
            if self.roots.iter().any(|root|self.equations.representative(root.output)==value) {
                return Some((value,offset));
            }
            if let Some((base,delta)) = constant_step_using(fd,value,&|v|self.equations.value(fd,v)) {
                offset = offset.checked_add(delta)?;
                value = base;
                continue;
            }
            if let Some(op) = fd.vbank().get(value)?.get_def() {
                if let Some((slot,_)) = alignment_using(fd,op,&|v|self.equations.value(fd,v)) {
                    value = fd.obank().get(op)?.get_in(slot)?;
                    continue;
                }
            }
            return Some((value,offset));
        }
    }

    fn full_upper(&self, fd: &Funcdata, value: VarnodeId) -> Option<i128> {
        if let Some(&upper) = self.full_uppers.borrow().get(&value) { return upper; }
        let upper = self.upper(fd,value,&mut 65536);
        self.full_uppers.borrow_mut().insert(value,upper);
        upper
    }
    fn upper(&self, fd: &Funcdata, value: VarnodeId, budget: &mut usize) -> Option<i128> {
        if let Some(offset) = self.equations.delta(fd,value,self.entry) { return Some(offset); }
        // A validated downward reservation has a stack upper bound even
        // when its count's exact value is opaque. This is not a value alias.
        let bounded=|value|self.equations.bounded_value(fd,&self.roots,value);
        let mut upper=stack_upper_using(fd,value,self.entry,None,None,budget,&bounded);
        for root in &self.roots {
            if negate(fd,root.amount).is_some() {continue;}
            let Some(delta)=self.equations.delta(fd,value,root.output) else {continue};
            if let Some(bound)=stack_upper_using(fd,root.upper_base(fd),self.entry,None,None,budget,&bounded)
                .and_then(|before|before.checked_add(delta))
            {upper=Some(upper.map_or(bound,|previous|previous.min(bound)));}
        }
        let Some((value,extra)) = self.unaligned(fd,value,budget) else { return upper };
        for root in &self.roots {
            if negate(fd,root.amount).is_some() { continue; }
            let Some(delta) = self.equations.delta(fd,value,root.output) else { continue };
            let base = self.unaligned(fd,root.base,budget).and_then(|(base,offset)|
                self.equations.delta(fd,base,self.entry)?.checked_add(offset));
            if let Some(bound) = base.and_then(|base|base.checked_add(delta)?.checked_add(extra)) {
                upper = Some(upper.map_or(bound,|previous|previous.min(bound)));
            }
        }
        upper
    }

    fn first(&self, fd: &Funcdata, word: FrameWordUse) -> Option<i128> {
        if let Some(pointer)=word.pointer {
            if matches!(word.read,Some(FrameRead::Operation(op)) if !self.input_valid(op,1)) {return None;}
            self.equations.delta(fd,pointer,self.entry)
        } else {word.first}
    }

    fn binding(&self, fd: &Funcdata, word: FrameWordUse) -> bool {
        if let Some((id,store))=word.birth.and_then(|id|fd.obank().get(id)
            .filter(|op|op.code()==OpCode::CPUI_STORE).map(|op|(id,op))) {
            if !self.input_valid(id,1) || store.get_in(0).and_then(|value|number(fd,value))!=Some(self.memory) {return false;}
        }
        let Some(pointer) = word.pointer else { return true };
        if matches!(word.read,Some(FrameRead::Operation(op)) if !self.input_valid(op,1)) {return false;}
        if word.origin_only {return true;}
        if let Some(target) = word.birth.and_then(|id|fd.obank().get(id))
            .filter(|op|op.code()==OpCode::CPUI_STORE).and_then(|op|op.get_in(1))
        { return word.birth.is_some_and(|birth|self.input_valid(birth,1))
            && self.equations.delta(fd,pointer,target)==Some(0); }
        let Some(stack)=fd.get_arch().manage().get_stack_space() else {return false};
        fd.vbank().get(word.source).filter(|node|node.get_space().get_index()==stack.get_index())
            .is_some_and(|source|self.equations.delta(fd,pointer,self.entry)==Some(
                sign_extend(source.get_offset() as i64,stack.get_addr_size() as int4*8-1) as i128))
    }

    fn store_reaches(&self, fd: &Funcdata, pointer: VarnodeId, bytes: int4, word: FrameWordUse, site: Option<(OpId,int4)>) -> Option<bool> {
        if bytes <= 0 { return Some(true); }
        let delta = if site.is_some_and(|(op,slot)|!self.input_valid(op,slot)) {None} else if let Some(first) = self.first(fd,word) {
            self.equations.delta(fd,pointer,self.entry).and_then(|at|at.checked_sub(first))
        } else {
            word.pointer.and_then(|target|self.equations.delta(fd,pointer,target))
        };
        if let Some(delta) = delta {
            return Some(delta < word.size as i128 && delta.checked_add(bytes as i128).is_none_or(|end|end>0));
        }
        if let Some(first)=self.first(fd,word) {
            if site.is_none_or(|(op,slot)|self.input_valid(op,slot)) && self.full_upper(fd,pointer).and_then(|upper|upper.checked_add(bytes as i128))
                .is_some_and(|end|end<=first) {return Some(false);}
            return self.private_at(fd,pointer,site);
        }
        // Source ownership separates source objects from fixed compiler words,
        // not two potentially overlapping accesses within source storage.
        match self.private_at(fd,pointer,site) {
            Some(false) if word.pointer.is_some_and(|target|
                self.origins.must_private(&self.equations,target)) => Some(false),
            Some(_) => Some(true),
            None => None,
        }
    }

    fn stack_write_reaches(&self, fd: &Funcdata, at: i128, bytes: int4, word: FrameWordUse) -> bool {
        if let Some(first) = self.first(fd,word) {
            return at < first + word.size as i128 && at.checked_add(bytes as i128).is_none_or(|end|end>first);
        }
        if let Some(pointer) = word.pointer {
            if self.full_upper(fd,pointer).and_then(|upper|upper.checked_add(word.size as i128))
                .is_some_and(|end|end<=at) { return false; }
        }
        true
    }

    fn escapes(&self, fd: &Funcdata, value: VarnodeId, word: FrameWordUse, site: Option<(OpId,int4)>) -> bool {
        if self.non_native_at(fd,value,site)
            && self.private_at(fd,value,site)==Some(false) { return false; }
        // A pointer into a runtime reservation addresses a source object below
        // the static frame: reaching a frame word at or above the reservation's
        // base would step outside that object.
        if site.is_none_or(|(op,slot)|self.input_valid(op,slot)) && self.first(fd,word).is_some_and(|first|
            self.roots.iter().any(|root|self.equations.delta(fd,value,root.output).is_some_and(|d|d>=0)
                && self.equations.delta(fd,root.upper_base(fd),self.entry).is_some_and(|base|first>=base)))
        { return false; }
        if site.is_some_and(|(op,slot)|!self.input_valid(op,slot)) {return self.private_at(fd,value,site).unwrap_or(true);}
        if let Some(first) = self.first(fd,word) {
            if let Some(at) = self.equations.delta(fd,value,self.entry) {
                return at>=first && at<first+word.size as i128;
            }
        } else if let Some(pointer) = word.pointer {
            if let Some(delta) = self.equations.delta(fd,value,pointer) {
                return delta>=0 && delta<word.size as i128;
            }
        }
        self.private_at(fd,value,site).unwrap_or(true)
    }

    fn call_preserves(&self, fd: &Funcdata, call: OpId, word: FrameWordUse) -> bool {
        let Some(index) = fd.get_call_specs_index(call) else { return false };
        let Some(floor) = call_frame_floor(fd,fd.get_call_specs(index)) else { return false };
        // Below a runtime reservation the call's SP is the reservation's
        // output plus a known displacement, and the output never lies above
        // the reservation's base (the amount may be zero): the callee's area
        // ends by base + d + floor.
        if let Some(first) = self.first(fd,word) {
            let snapshot=fd.kuna_stack_call_sp.get(&call).copied().flatten()
                .or_else(||fd.get_call_specs(index).get_spacebase_relative(fd));
            if snapshot.is_some_and(|sp|self.roots.iter().any(|root|dominates_op(fd,root.op,call)
                && self.equations.delta(fd,sp,root.output).zip(self.equations.delta(fd,root.upper_base(fd),self.entry))
                    .and_then(|(d,base)|base.checked_add(d)?.checked_add(floor)).is_some_and(|limit|first>=limit)))
            {return true;}
        }
        if let Some(first) = self.first(fd,word) {
            return self.calls.get(&call).copied().flatten().and_then(|sp|sp.checked_add(floor))
                .is_some_and(|limit|first>=limit);
        }
        let Some(pointer) = word.pointer else { return false };
        if self.lower(fd,pointer,&mut 65536).zip(self.calls.get(&call).copied().flatten())
            .and_then(|(first,sp)|sp.checked_add(floor).map(|limit|first>=limit))==Some(true)
        {return true;}
        if !self.origins.must_private(&self.equations,pointer) {return false;}
        fd.kuna_stack_call_sp.get(&call).copied().flatten()
            .and_then(|sp|self.equations.delta(fd,pointer,sp)).is_some_and(|delta|delta>=floor)
    }

    fn identical_store(&self, fd: &Funcdata, id: OpId, word: FrameWordUse) -> bool {
        if !self.input_valid(id,1) || !self.input_valid(id,2) {return false;}
        let Some(op) = fd.obank().get(id).filter(|op|op.code()==OpCode::CPUI_STORE) else { return false };
        let Some(value) = op.get_in(2).filter(|&value|fd.vbank().get(value).is_some_and(|v|v.get_size()==word.size)) else { return false };
        let Some(pointer) = op.get_in(1) else { return false };
        let delta = if let Some(first) = self.first(fd,word) {
            self.equations.delta(fd,pointer,self.entry).and_then(|at|at.checked_sub(first))
        } else { word.pointer.and_then(|target|self.equations.delta(fd,pointer,target)) };
        delta==Some(0) && self.equations.delta(fd,value,word.source)==Some(0)
    }

    fn private_spill_in_span(&self, fd: &Funcdata, store: OpId, span: &[(OpId,bool)]) -> bool {
        let Some(stack)=fd.get_arch().manage().get_stack_space() else {return false};
        let Some(op)=fd.obank().get(store) else {return false};
        if op.code()!=OpCode::CPUI_STORE || op.get_in(0).and_then(|v|number(fd,v))!=Some(self.memory)
            || !self.input_valid(store,1) || !self.input_valid(store,2) {return false;}
        let Some(first)=self.input_value(fd,store,1).and_then(|v|self.equations.delta(fd,v,self.entry)) else {return false};
        let Some(source)=op.get_in(2) else {return false};
        let Some(size)=fd.vbank().get(source).filter(|v|!v.is_volatile()).map(|v|v.get_size()) else {return false};
        let Some(last)=first.checked_add(size as i128) else {return false};
        let bias=stack_bias(fd,stack.get_addr_size() as int4) as i128;
        if size<=0 || last>bias || !self.roots.iter().any(|root|
            self.equations.delta(fd,root.upper_base(fd),self.entry).is_some_and(|floor|first>=floor+bias)) {return false;}
        let word=FrameWordUse {origin_only:false,result:None,source,first:Some(first),pointer:None,size,
            read:None,birth:None,alias:false};
        // One flat address-privacy check, not a recursive saved-value proof.
        // A store into this same private cell does not publish its contents.
        for &(id,_) in span {
            let Some(op)=fd.obank().get(id).filter(|op|!op.is_dead()&&!op.is_marker()) else {continue};
            if op.code()==OpCode::CPUI_STORE && op.get_in(0).and_then(|v|number(fd,v))==Some(self.memory) {
                if let Some(at)=self.input_value(fd,id,1).and_then(|v|self.equations.delta(fd,v,self.entry)) {
                    if op.get_in(2).and_then(|v|fd.vbank().get(v)).is_some_and(|v|
                        at>=first && at.checked_add(v.get_size() as i128).is_some_and(|end|end<=last)) {continue;}
                }
            }
            let slots=match op.code() {
                OpCode::CPUI_CALL=>{if !self.call_preserves(fd,id,word) {return false;} 1..op.num_input()},
                OpCode::CPUI_CALLIND=>{if !self.call_preserves(fd,id,word) {return false;} 0..op.num_input()},
                OpCode::CPUI_CALLOTHER=>return false,
                OpCode::CPUI_RETURN=>0..op.num_input(),
                OpCode::CPUI_STORE=>2..3,
                _ if op.get_out().and_then(|v|fd.vbank().get(v)).is_some_and(|v|
                    v.is_addr_tied()&&v.get_space().get_index() as i128==self.memory)=>0..op.num_input(),
                _=>0..0,
            };
            if slots.into_iter().any(|slot|op.get_in(slot).is_none_or(|v|self.escapes(fd,v,word,Some((id,slot))))) {return false;}
        }
        true
    }

    fn escaped_in_span(&self, fd: &Funcdata, word: FrameWordUse, span: &[(OpId,bool)]) -> bool {
        let mut private_spills=BTreeMap::new();
        for &(id,_) in span {
            let Some(op)=fd.obank().get(id).filter(|op|!op.is_dead()&&!op.is_marker()) else {continue};
            let inputs=match op.code() {
                OpCode::CPUI_CALL=>1..op.num_input(),
                OpCode::CPUI_CALLIND|OpCode::CPUI_CALLOTHER|OpCode::CPUI_RETURN=>0..op.num_input(),
                OpCode::CPUI_STORE if !self.identical_store(fd,id,word)
                    && !self.transports.iter().any(|&(store,_,_,_,_)|store==id)
                    && !(self.input_valid(id,2) && native_frame_link_payload(fd,id,
                        fd.get_arch().manage().get_stack_space().map_or(word.size,|stack|stack.get_addr_size() as int4)))=>2..3,
                _ if op.get_out().and_then(|value|fd.vbank().get(value)).is_some_and(|value|
                    value.is_addr_tied() && value.get_space().get_index() as i128==self.memory)=>0..op.num_input(),
                _=>0..0,
            };
            // A locked, fixed prototype reads only its declared parameters;
            // trial inputs beyond them are not arguments the callee receives.
            let declared=(op.code()==OpCode::CPUI_CALL).then(||fd.get_call_specs_index(id)).flatten()
                .map(|index|fd.get_call_specs(index)).filter(|spec|spec.is_input_locked() && !spec.is_dotdotdot())
                .map(|spec|spec.proto().num_params());
            for slot in inputs {
                if declared.is_some_and(|count|slot>count) {continue;}
                if op.get_in(slot).is_none_or(|value|self.escapes(fd,value,word,Some((id,slot)))) {
                    if op.code()==OpCode::CPUI_STORE && slot==2 {
                        let key=self.input_value(fd,id,1).and_then(|v|self.equations.delta(fd,v,self.entry))
                            .zip(op.get_in(2).and_then(|v|fd.vbank().get(v)).map(|v|v.get_size()));
                        if let Some((first,size))=key {
                            let data=op.get_in(2).unwrap();
                            let disjoint=self.first(fd,word).is_some_and(|word_first|
                                first>=word_first+word.size as i128 || first.checked_add(size as i128).is_some_and(|last|last<=word_first));
                            let explicit=self.first(fd,word).zip(self.equations.delta(fd,data,self.entry))
                                .is_some_and(|(first,at)|at>=first && at<first+word.size as i128);
                            if disjoint && !explicit && self.input_valid(id,2)
                                && fd.vbank().get(data).is_some_and(|node|!node.is_volatile())
                                && *private_spills.entry((first,size)).or_insert_with(||self.private_spill_in_span(fd,id,span)) {continue;}
                        }
                    }
                    return true;
                }
            }
        }
        false
    }

    fn feed_effects(&mut self, fd: &Funcdata) -> bool {
        let Some(stack) = fd.get_arch().manage().get_stack_space() else { return false };
        let mut changed = false;
        for index in 0..self.words.uses.len() {
            let word = self.words.uses[index];
            let span = &self.spans[index];
            let escaped = self.escaped_in_span(fd,word,span);
            for &(id,live) in span {
                if !live { continue; }
                let Some(op) = fd.obank().get(id).filter(|op|!op.is_dead()) else { continue };
                if (!op.is_marker() || op.is_indirect_creation()) && op.get_out().and_then(|v|fd.vbank().get(v))
                    .is_some_and(|out|out.get_space().get_index()==stack.get_index())
                {
                    let value = op.get_out().unwrap();
                    let out = fd.vbank().get(value).unwrap();
                    let at = sign_extend(out.get_offset() as i64,stack.get_addr_size() as int4*8-1) as i128;
                    if self.stack_write_reaches(fd,at,out.get_size(),word) {
                        let data = if op.code()==OpCode::CPUI_STORE {op.get_in(2).unwrap_or(value)} else {value};
                        changed |= if op.code()==OpCode::CPUI_STORE {
                            self.origins.add_word_input(fd,&self.equations,index,id,2)
                        } else {self.origins.add_word_source(&self.equations,index,data)};
                        if self.first(fd,word)!=Some(at) || out.get_size()!=word.size {
                            changed |= self.origins.manipulated_word(index);
                        }
                    }
                }
                match op.code() {
                    OpCode::CPUI_STORE if op.get_in(0).and_then(|v|number(fd,v))==Some(self.memory) => {
                        if let Some((pointer,data)) = op.get_in(1).zip(op.get_in(2)) {
                            let bytes = fd.vbank().get(data).map_or(0,|v|v.get_size());
                            if self.store_reaches(fd,pointer,bytes,word,Some((id,1))).unwrap_or(true) {
                                changed |= self.origins.add_word_input(fd,&self.equations,index,id,2);
                                let delta=self.first(fd,word).and_then(|first|
                                    self.equations.delta(fd,pointer,self.entry)?.checked_sub(first))
                                    .or_else(||word.pointer.and_then(|target|self.equations.delta(fd,pointer,target)));
                                if delta!=Some(0) || bytes!=word.size {
                                    changed |= self.origins.manipulated_word(index);
                                }
                            }
                        }
                    }
                    OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => {
                        if escaped || !self.call_preserves(fd,id,word) {
                            changed |= self.origins.unknown_word(index);
                        }
                    }
                    OpCode::CPUI_CALLOTHER => changed |= self.origins.unknown_word(index),
                    _ => {}
                }
            }
        }
        changed
    }

    fn preserved(&self, fd: &Funcdata, index: usize) -> bool {
        !self.invalid_reads.contains(&index) && self.check_word(fd,self.words.uses[index],&self.spans[index])
    }
    fn lower(&self, fd: &Funcdata, mut value: VarnodeId, budget: &mut usize) -> Option<i128> {
        let mut displacement=0_i128;
        loop {
            *budget=budget.checked_sub(1)?;
            if let Some(offset)=self.equations.delta(fd,value,self.entry) {
                return offset.checked_add(displacement);
            }
            let (known,opaque)=self.equations.value(fd,value);
            if opaque {return None;}
            value=known;
            if let Some((base,step))=constant_step_using(fd,value,&|v|self.equations.value(fd,v)) {
                displacement=displacement.checked_add(step)?;
                value=base;
            } else {
                let id=fd.vbank().get(value)?.get_def()?;
                let (slot,alignment)=alignment_using(fd,id,&|v|self.equations.value(fd,v))?;
                displacement=displacement.checked_sub((alignment-1) as i128)?;
                value=fd.obank().get(id)?.get_in(slot)?;
            }
        }
    }
    fn backchain_transport(&self, fd: &Funcdata, id: OpId, width: int4) -> Option<(usize,OpId,OpId,OpId)> {
        let store=fd.obank().get(id).filter(|op|!op.is_dead())?;
        // Heritage may turn the relocation into a direct write of the restored
        // frame's link word once SP has a constant frame offset there.
        let word=direct_frame_word(fd,id,width);
        let (target,mut value)=if let Some(offset)=word {
            if !self.input_valid(id,0) {return None;}
            (None,store.get_in(0).map(|value|(value,offset))?.0)
        } else {
            if store.code()!=OpCode::CPUI_STORE || store.get_in(0).and_then(|value|number(fd,value))!=Some(self.memory)
                || !self.input_valid(id,1) || !self.input_valid(id,2) {return None;}
            let target=store.get_in(1)?;
            if fd.vbank().get(target)?.is_volatile() {return None;}
            (Some(target),store.get_in(2)?)
        };
        let mut consumer=id;
        let mut budget=65536_usize;
        let load_id=loop {
            budget=budget.checked_sub(1)?;
            let node=fd.vbank().get(value)?;
            if node.get_size()!=width || node.is_volatile() || node.is_persist() || node.is_auto_live_hold()
                || fd.kuna_stack_scope_holds.contains(&value)
                || fd.kuna_stack_call_sp.values().flatten().any(|&sp|sp==value)
                || self.roots.iter().any(|root|[root.before,root.base,root.amount].contains(&value))
                || node.descend_iter().any(|user|user!=consumer && fd.obank().get(user).is_some_and(|op|!op.is_dead()))
            {return None;}
            let definition=node.get_def()?;
            let op=fd.obank().get(definition)?;
            if op.get_parent()!=store.get_parent() {return None;}
            match op.code() {
                OpCode::CPUI_LOAD=>break definition,
                OpCode::CPUI_COPY|OpCode::CPUI_CAST if self.input_valid(definition,0)
                    && !node.is_addr_tied()=>{value=op.get_in(0)?;consumer=definition;}
                _=>return None,
            }
        };
        let load=fd.obank().get(load_id)?;
        if load.get_in(0).and_then(|value|number(fd,value))!=Some(self.memory) || !self.input_valid(load_id,1) {return None;}
        let pointer=load.get_in(1)?;
        if fd.vbank().get(pointer)?.is_volatile() {return None;}
        let (root,header)=self.roots.iter().enumerate().find_map(|(index,root)| {
            if self.equations.delta(fd,pointer,root.output)!=Some(0) {return None;}
            fd.obank().iter_alive().find(|&writer|self.link_store(fd,root,writer,width)
                && dominates_op(fd,writer,load_id)
                && fd.obank().get(writer).and_then(|op|op.get_in(2))
                    .is_some_and(|payload|self.equations.delta(fd,payload,root.before)!=Some(0))
                && self.header_live(fd,writer,FrameRead::Operation(load_id),root.output,&mut budget))
                .map(|writer|(index,writer))
        })?;
        let mut next=load.basic_neighbours().1;
        let mut restore=None;
        while let Some(current)=next {
            if current==id {break;}
            budget=budget.checked_sub(1)?;
            let op=fd.obank().get(current)?;
            if op.get_parent()!=store.get_parent() {return None;}
            next=op.basic_neighbours().1;
            if op.is_marker() {continue;}
            if let Some(output)=op.get_out().filter(|&value|stack_register(fd,value)) {
                let at=match target {
                    Some(target)=>self.equations.delta(fd,target,output)==Some(0),
                    None=>self.equations.delta(fd,output,self.entry)==word,
                };
                if op.code()!=OpCode::CPUI_COPY || !self.input_valid(current,0) || !at {return None;}
                if let Some((previous,value))=restore {
                    if fd.obank().get(previous)?.get_addr()!=op.get_addr()
                        || self.equations.delta(fd,value,output)!=Some(0) {return None;}
                }
                restore=Some((current,output));
            } else if matches!(op.code(),OpCode::CPUI_STORE|OpCode::CPUI_CALL|OpCode::CPUI_CALLIND
                |OpCode::CPUI_CALLOTHER|OpCode::CPUI_BRANCH|OpCode::CPUI_CBRANCH|OpCode::CPUI_BRANCHIND|OpCode::CPUI_RETURN)
                || op.get_out().and_then(|value|fd.vbank().get(value)).is_some_and(|node|
                    fd.get_arch().manage().get_stack_space().is_some_and(|stack|node.get_space().get_index()==stack.get_index())
                        || node.is_addr_tied() && node.get_space().get_index() as i128==self.memory) {return None;}
        }
        if next!=Some(id) {return None;}
        let (restore,_)=restore?;
        dominates_op(fd,self.roots[root].op,restore).then_some((root,header,restore,load_id))
    }
fn header_reaches_at(&self, fd: &Funcdata, value: VarnodeId, bytes: i128, header: VarnodeId, boundary: VarnodeId,
        width: int4, site: Option<(OpId,int4)>, budget: &mut usize) -> bool {
        // A value narrower than an address cannot point at the header: a use as a
        // pointer widens it first, and that result is checked where it is used.
        if fd.vbank().get(value).is_some_and(|node|node.get_size()<width) {return false;}
        if site.is_some_and(|(op,slot)|!self.input_valid(op,slot)) {return true;}
        enum Work {Visit(VarnodeId,i128),Leave(VarnodeId)}
        let mut todo=vec![Work::Visit(value,0)];
        let mut active=BTreeSet::new();
        while let Some(work)=todo.pop() {
            let (value,extra)=match work {
                Work::Visit(value,extra)=>(value,extra),
                Work::Leave(value)=>{active.remove(&value);continue;}
            };
            let Some(left)=budget.checked_sub(1) else {return true};
            *budget=left;
            let overlap=|delta:i128|{
                let delta=sign_extend(delta as i64,width*8-1) as i128;
                delta<width as i128 && delta.checked_add(bytes).is_none_or(|end|end>0)
            };
            if let Some(delta)=self.equations.delta(fd,value,header) {
                let Some(delta)=delta.checked_add(extra) else {return true};
                if overlap(delta) {return true;}
                continue;
            }
            let Some((known,displacement))=self.affine_base(fd,value,budget) else {
                if self.origins.non_native(&self.equations,value)
                    && self.origins.private_address(&self.equations,value)==Some(false) {continue;}
                return true;
            };
            let Some(displacement)=displacement.checked_add(extra) else {return true};
            if let Some(delta)=self.equations.delta(fd,known,header) {
                let Some(delta)=delta.checked_add(displacement) else {return true};
                if overlap(delta) {return true;}
                continue;
            }
            if let Some(op)=fd.vbank().get(known).and_then(|node|node.get_def()).and_then(|id|fd.obank().get(id))
                .filter(|op|op.code()==OpCode::CPUI_MULTIEQUAL) {
                if self.origins.non_native(&self.equations,known)
                    && self.origins.private_address(&self.equations,known)==Some(false) {continue;}
                if op.num_input()==0 || !active.insert(known) {return true;}
                todo.push(Work::Leave(known));
                for slot in 0..op.num_input() {
                    let Some(input)=op.get_in(slot) else {return true};
                    todo.push(Work::Visit(input,displacement));
                }
                continue;
            }
            if self.equations.delta(fd,value,boundary).and_then(|delta|delta.checked_add(extra))
                .is_some_and(|delta|delta>=0) {continue;}
            if extra!=0 {return true;}
            if self.origins.private_address(&self.equations,value)!=Some(false) {return true;}
        }
        false
    }
}

fn saved_stack_value(
    fd: &Funcdata, value: VarnodeId, budget: &mut usize, proof: &mut SavedFrameQuery,
) -> Option<SavedFrameValue> {
    *budget = budget.checked_sub(1)?;
    let node = fd.vbank().get(value)?;
    if node.is_volatile() { return None; }
    let Some(id) = node.get_def() else {
        return Some(SavedFrameValue { value: Some(value), birth: None, pending: None });
    };
    if let Some(mut saved) = proof.active.get(&id).copied() {
        if saved.value.is_none() {
            let op = fd.obank().get(id)?;
            if matches!(op.code(), OpCode::CPUI_COPY | OpCode::CPUI_CAST) {
                if let Some(source) = op.get_in(0).and_then(|value| fd.vbank().get(value).map(|node|(value,node))) {
                    let stack_read = fd.get_arch().manage().get_stack_space()
                        .is_some_and(|stack| source.1.get_space().get_index() == stack.get_index());
                    if stack_read && source.1.get_size() == node.get_size() {
                        if let Some(candidate) = incoming_frame_word(fd, source.0, budget)
                            .and_then(|value| fd.vbank().get(value)?.get_def())
                            .and_then(|candidate| proof.active.get(&candidate).or_else(|| proof.completed.get(&candidate)))
                            .copied().filter(|candidate| candidate.value.is_some() && candidate.pending.is_none())
                        {
                            // The suspended reader still checks its actual read
                            // after its producer finishes. Propagate the producer's
                            // initialized equation, not a storage-preservation fact.
                            saved = candidate;
                            saved.birth = saved.birth.filter(|&birth| dominates_op(fd, birth, id));
                            proof.active.insert(id, saved);
                        }
                    }
                }
            }
        }
        proof.assumption_reads += 1;
        return Some(saved);
    }
    if let Some(&saved) = proof.completed.get(&id) { return Some(saved); }
    let assumption_reads = proof.assumption_reads;
    proof.active.insert(id, SavedFrameValue { value: None, birth: None, pending: Some(id) });
    let result = (|| {
        let op = fd.obank().get(id)?;
        let stack = fd.get_arch().manage().get_stack_space().cloned();
        let in_stack = stack.as_ref().is_some_and(|stack| node.get_space().get_index() == stack.get_index());
        match op.code() {
            OpCode::CPUI_COPY | OpCode::CPUI_CAST => {
                let source = op.get_in(0)?;
                if fd.vbank().get(source)?.get_size() != node.get_size() {
                    return Some(SavedFrameValue { value: Some(value), birth: in_stack.then_some(id), pending: None });
                }
                let source_node = fd.vbank().get(source)?;
                let stack_read = stack.as_ref().is_some_and(|stack| source_node.get_space().get_index() == stack.get_index());
                let candidate = if stack_read { incoming_frame_word(fd, source, budget)? } else { source };
                let mut saved = saved_stack_value(fd, candidate, budget, proof)?;
                if saved.value.is_none() { return Some(saved); }
                if stack_read {
                    let stack = stack.as_ref()?;
                    let first = sign_extend(source_node.get_offset() as i64, stack.get_addr_size() as int4 * 8 - 1) as i128;
                    // A direct stack-word read has the same conditional value
                    // obligation as a LOAD or an INDIRECT transition.
                    if let Some(expected) = proof.active.get(&id).and_then(|saved| saved.value) {
                        if native_stack_delta(fd, saved.value?, expected) != Some(0) { return None; }
                    }
                    let mut assumed = saved;
                    assumed.birth = assumed.birth.filter(|&birth| dominates_op(fd, birth, id));
                    proof.active.insert(id, assumed);
                    private_frame_slot(fd, first, source_node.get_size(), FrameRead::Operation(id), saved.birth, saved.value, budget, proof)?;
                }
                if in_stack { saved.birth = Some(id); }
                Some(saved)
            }
            OpCode::CPUI_MULTIEQUAL => {
                let mut saved: Option<SavedFrameValue> = None;
                let mut deferred = Vec::new();
                let mut pending = None;
                for pass in 0..2 {
                    if pass == 1 && saved.is_none() {
                        return Some(SavedFrameValue { value: None, birth: None, pending });
                    }
                    let count = if pass == 0 { op.num_input() as usize } else { deferred.len() };
                    for slot in 0..count {
                        let input = if pass == 0 { op.get_in(slot as int4)? } else { deferred[slot] };
                        let mut next = saved_stack_value(fd, input, budget, proof)?;
                        if pass == 1 {
                            while next.value.is_none() {
                                let anchor = next.pending?;
                                if anchor == id { return None; }
                                let previous = proof.active.get(&anchor)?;
                                // Each retry must seed an active ancestor, not
                                // waive a deferred call or read obligation.
                                if previous.value.is_some() { return None; }
                                let mut assumed = saved?;
                                assumed.birth = assumed.birth.filter(|&birth| dominates_op(fd, birth, anchor));
                                proof.active.insert(anchor, assumed);
                                next = saved_stack_value(fd, input, budget, proof)?;
                            }
                        }
                        let Some(source) = next.value else {
                            let anchor = next.pending?;
                            if pending.is_none() || pending == Some(id) { pending = Some(anchor); }
                            deferred.push(input);
                            continue;
                        };
                        if let Some(previous) = saved {
                            if native_stack_delta(fd, source, previous.value?) != Some(0) { return None; }
                            let birth = match (previous.birth, next.birth) {
                                (a, b) if a == b => a,
                                (Some(a), Some(b)) if dominates_op(fd, a, b) => Some(a),
                                (Some(a), Some(b)) if dominates_op(fd, b, a) => Some(b),
                                _ => None,
                            };
                            saved.as_mut()?.birth = birth;
                        } else { saved = Some(next); }
                        let mut assumed = saved?;
                        assumed.birth = assumed.birth.filter(|&birth| dominates_op(fd, birth, id));
                        proof.active.insert(id, assumed);
                    }
                }
                saved
            }
            OpCode::CPUI_INDIRECT if in_stack => {
                let stack = stack.as_ref()?;
                if op.is_indirect_creation() || node.is_volatile() { return None; }
                let source = op.get_in(0)?;
                let mut saved = saved_stack_value(fd, source, budget, proof)?;
                if saved.value.is_none() { return Some(saved); }
                // A different source is a store folded into this marker: by the
                // marker's meaning, its input is the slot's value just before
                // the effect, so the slot took that value here, not where the
                // source was computed.
                if fd.vbank().get(source)?.get_addr() != node.get_addr() {
                    saved.birth = Some(id);
                }
                let effect_op = frame_word_effect(fd, id)?;
                // Check the transition from its initialized incoming value.
                // A store of this value is not an escape of its storage address.
                let mut assumed = saved;
                assumed.birth = assumed.birth.filter(|&birth| dominates_op(fd, birth, id));
                proof.active.insert(id, assumed);
                let first = sign_extend(node.get_offset() as i64, stack.get_addr_size() as int4 * 8 - 1) as i128;
                private_frame_slot(fd, first, node.get_size(), FrameRead::Operation(effect_op), saved.birth, saved.value, budget, proof)?;
                saved.birth = Some(effect_op);
                Some(saved)
            }
            OpCode::CPUI_INDIRECT if indirect_stack_pop(fd, id) == Some(0) => {
                saved_stack_value(fd, op.get_in(0)?, budget, proof)
            }
            OpCode::CPUI_LOAD => {
                let pointer = op.get_in(1)?;
                if !matches!(node.get_size(), 1 | 2 | 4 | 8) { return None; }
                let memory = number(fd, op.get_in(0)?)?;
                let first = stack.as_ref().filter(|stack|
                    stack.get_contain().is_some_and(|space| space.get_index() as i128 == memory))
                    .and_then(|_| frame_offset(fd, pointer, budget, proof));
                let writer = raw_word_writer(fd, id, pointer, memory, first, WordQuery::Value, budget, proof)?;
                let mut saved = saved_stack_value(fd, writer.value, budget, proof)?;
                if saved.value.is_none() { return Some(saved); }
                let birth = if fd.obank().get(writer.op)?.code() == OpCode::CPUI_STORE {
                    Some(writer.op)
                } else if fd.obank().get(writer.op)?.code() == OpCode::CPUI_INDIRECT {
                    // The incoming candidate may have no common birth. Starting
                    // at its later marker would miss the marker's real effect.
                    saved.birth
                } else { saved.birth.or(Some(writer.op)) };
                // As for an INDIRECT transition, the initialized candidate is
                // conditional until every intervening write/read obligation
                // succeeds. Re-entrant alias checks may use that same value.
                let mut assumed = saved;
                assumed.birth = birth.filter(|&birth| dominates_op(fd, birth, id));
                proof.active.insert(id, assumed);
                if let Some(first) = first {
                    private_frame_slot(fd, first, node.get_size(), FrameRead::Operation(id), birth, saved.value, budget, proof)?;
                } else {
                    unchanged_raw_slot(fd, pointer, memory, node.get_size(), id, writer.op, budget, proof)?;
                }
                saved.birth = Some(id);
                Some(saved)
            }
            _ => Some(SavedFrameValue { value: Some(value), birth: in_stack.then_some(id), pending: None }),
        }
    })();
    let assumed = proof.active.remove(&id)?;
    let result = result?;
    if let Some(expected) = assumed.value {
        if native_stack_delta(fd, result.value?, expected) != Some(0) { return None; }
    }
    if result.value.is_none() && result.pending == Some(id) { return None; }
    if *budget != 0 && result.value.is_some() && result.pending.is_none()
        && proof.assumption_reads == assumption_reads
    {
        proof.completed.insert(id, result);
    }
    Some(result)
}

fn recovered_base(fd: &Funcdata, base: VarnodeId, offset: i128) -> (VarnodeId, i128) {
    let (known, delta) = resolved_base(fd, base, &mut 4096, &mut SavedFrameQuery::default());
    let (base, extra) = canonical_base(fd, known, &mut BTreeSet::new());
    (base, offset + delta + extra)
}

#[derive(Clone, Copy)]
struct LifetimeEnd {
    restore: OpId,
    join: BlockId,
    boundary: Option<BlockId>,
    address: u64,
    value: VarnodeId,
}

fn proved_restore(fd: &Funcdata, allocation: &Allocation, value: VarnodeId) -> Option<LifetimeEnd> {
    if !fd.get_arch().stack_alloc_scopes || !stack_register(fd, value) { return None; }
    let id = fd.vbank().get(value)?.get_def()?;
    if !dominates_op(fd, allocation.op, id) { return None; }
    let op = fd.obank().get(id)?;
    if op.code() == OpCode::CPUI_MULTIEQUAL { return None; }
    let saved = saved_stack_value(fd, value, &mut 65536, &mut SavedFrameQuery::default())?.value?;
    if native_stack_delta(fd, saved, allocation.before) != Some(0)
        && !frame_offset(fd, saved, &mut 4096, &mut SavedFrameQuery::default())
            .zip(frame_offset(fd, allocation.before, &mut 4096, &mut SavedFrameQuery::default()))
            .is_some_and(|(left, right)| left == right)
    { return None; }
    Some(LifetimeEnd { restore: id, join: op.get_parent()?, boundary: None,
        address: op.get_addr().get_offset(), value })
}

fn source_lifetime(fd: &Funcdata, allocation: &Allocation, facts: &FrameFacts) -> Result<Option<LifetimeEnd>, &'static str> {
    let mut end: Option<LifetimeEnd> = None;
    for next in fd.kuna_stack_scope_holds.iter().filter_map(|&value| facts.restore_end(fd, allocation, value)) {
        if !observes_after_restore(fd, next.restore, allocation.before, &mut 65536) { continue; }
        if let Some(previous) = end {
            if dominates_op(fd, previous.restore, next.restore) { continue; }
            if !dominates_op(fd, next.restore, previous.restore) {
                return Err("Native stack lifetime has no single lexical exit; allocation recovery declined.");
            }
        }
        end = Some(next);
    }
    Ok(end)
}

fn lifetime_body(fd: &Funcdata, allocation: &Allocation, end: &LifetimeEnd) -> Result<Vec<BlockId>, &'static str> {
    let mut blocks = scoped_region(fd, allocation.op, end.join)?;
    if !blocks.contains(&end.join) { blocks.push(end.join); }
    Ok(blocks)
}

fn within_lifetime(fd: &Funcdata, id: OpId, allocation: &Allocation, end: &LifetimeEnd, blocks: &[BlockId]) -> bool {
    let Some(block) = fd.obank().get(id).and_then(|op| op.get_parent()) else { return false };
    blocks.contains(&block) && !after_scope_end(fd, id, end)
        && (id == allocation.op || dominates_op(fd, allocation.op, id))
}

struct ScopeCapture {
    source: VarnodeId,
    ty: Rc<Datatype>,
    uses: Vec<(OpId, int4)>,
}

/// Materialize values at their existing definitions, never replay an expired
/// array expression at an outside use. Ordinary stores and call escapes stay.
fn scope_captures(
    fd: &Funcdata, facts: &FrameFacts, aliases: &[(VarnodeId, VarnodeId)],
    ends: &[Option<LifetimeEnd>], displaced: &[(OpId, VarnodeId, i128)], metadata: &[(OpId,usize)],
) -> Result<Vec<ScopeCapture>, &'static str> {
    let allocations=&facts.roots;
    let mut captures = BTreeMap::<VarnodeId, ScopeCapture>::new();
    let memory = fd.get_arch().manage().get_stack_space().and_then(|s| s.get_contain()).map(|s| s.get_index());
    for (allocation, end) in allocations.iter().zip(ends) {
        let Some(end) = end else { continue };
        let blocks = lifetime_body(fd, allocation, end)?;
        let mut pending = vec![allocation.output];
        pending.extend(aliases.iter().filter_map(|&(alias, root)| (root == allocation.output).then_some(alias)));
        pending.extend(displaced.iter().copied().filter_map(|(id, root, _)|
            (root == allocation.output).then(|| fd.obank().get(id).unwrap().get_out().unwrap())));
        for id in fd.obank().iter_alive() {
            let op = fd.obank().get(id).unwrap();
            if op.is_call() && within_lifetime(fd, id, allocation, end, &blocks) { pending.extend(op.get_out()); }
        }
        let mut derived = BTreeSet::new();
        while let Some(value) = pending.pop() {
            if !derived.insert(value) { continue; }
            for id in fd.vbank().get(value).unwrap().descend_iter() {
                let op = fd.obank().get(id).unwrap();
                // INDIRECTs have no C evaluation point. Track their versions,
                // but use the saved-value proof before bypassing their effect.
                if !op.is_dead() && (within_lifetime(fd, id, allocation, end, &blocks)
                    || op.code() == OpCode::CPUI_INDIRECT)
                { pending.extend(op.get_out()); }
            }
        }
        for id in fd.obank().iter_alive() {
            let op = fd.obank().get(id).unwrap();
            if metadata.iter().any(|&(store,_)|store==id)
                || (op.is_marker() && op.code() != OpCode::CPUI_MULTIEQUAL) || within_lifetime(fd, id, allocation, end, &blocks)
                || op.get_out().is_some_and(|out| stack_register(fd, out))
            { continue; }
            for slot in 0..op.num_input() {
                let Some(input) = op.get_in(slot) else { continue };
                if fd.vbank().get(input).is_some_and(|node| node.is_addr_tied()
                    && Some(node.get_space().get_index()) == memory) { continue; }
                let resolved=facts.input_value(fd,id,slot)
                    .filter(|&value|fd.vbank().get(value).unwrap().get_size()==fd.vbank().get(input).unwrap().get_size());
                let source = match resolved {
                    Some(source) if derived.contains(&source) => source,
                    Some(_) => continue,
                    None if derived.contains(&input) && !fd.vbank().get(input).unwrap().is_volatile()
                        && fd.vbank().get(input).and_then(|node| node.get_def()).is_some_and(|producer|
                            fd.obank().get(producer).is_some_and(|op| !op.is_indirect_creation())
                                && within_lifetime(fd, producer, allocation, end, &blocks)) => input,
                    None if derived.contains(&input) => return Err("A scoped value's saved storage is unproved; allocation recovery declined."),
                    None => continue,
                };
                // Zero-displacement aliases are replaced with their root
                // before captures are applied. Capture that actual consumer,
                // not the now-dead outside recomputation.
                let source = aliases.iter().find_map(|&(alias, root)|
                    (alias == source && root == allocation.output).then_some(root)).unwrap_or(source);
                if let Some(stack) = fd.get_arch().manage().get_stack_space() {
                    let input_node = fd.vbank().get(input).unwrap();
                    if input_node.get_space().get_index() == stack.get_index() {
                        if !facts.input_valid(id,slot)
                        {
                            return Err("A scoped address's storage changed before its outside read; allocation recovery declined.");
                        }
                    }
                }
                // Global escapes remain explicit source stores. Their later
                // memory versions are not private scalar snapshot candidates.
                if fd.vbank().get(source).is_some_and(|node| node.is_addr_tied()
                    && Some(node.get_space().get_index()) == memory) { continue; }
                let (source, consumer, slot) = if let Some(producer) = fd.vbank().get(source).unwrap().get_def()
                    .filter(|&producer| !within_lifetime(fd, producer, allocation, end, &blocks))
                {
                    let root = displaced.iter().find_map(|&(alias, root, _)|
                        (alias == producer && root == allocation.output).then_some(root))
                        .ok_or("A scoped value has an unproved outside memory effect; allocation recovery declined.")?;
                    // Materialization rewrites this existing affine alias to
                    // root + delta before replacing its root input with bits.
                    (root, producer, 0)
                } else { (source, id, slot) };
                let node = fd.vbank().get(source).unwrap();
                let producer = node.get_def().ok_or("A scoped value has no producer; allocation recovery declined.")?;
                if !within_lifetime(fd, producer, allocation, end, &blocks) {
                    return Err("A scoped value has an unproved outside memory effect; allocation recovery declined.");
                }
                if !dominates_input(fd, producer, consumer, slot) {
                    return Err("A scoped value does not dominate its outside use; allocation recovery declined.");
                }
                let meta = node.get_type().get_metatype();
                if !matches!(node.get_size(), 1 | 2 | 4 | 8)
                    || !matches!(meta, type_metatype::TYPE_UNKNOWN | type_metatype::TYPE_PTR
                        | type_metatype::TYPE_INT | type_metatype::TYPE_UINT | type_metatype::TYPE_BOOL | type_metatype::TYPE_FLOAT)
                {
                    return Err("A scoped value has an unsupported scalar representation; allocation recovery declined.");
                }
                let declared = declared_scalar_output(fd,producer,node.get_size());
                let ty = if let Some(ty) = declared { Rc::clone(ty) }
                    else if matches!(meta, type_metatype::TYPE_PTR | type_metatype::TYPE_UNKNOWN)
                        && matches!(node.get_size(), 1 | 2 | 4 | 8)
                    {
                        fd.get_arch().types_rc().and_then(|factory| factory.get_base(node.get_size(), type_metatype::TYPE_UINT).ok())
                            .ok_or("A scoped value has no integer representation; allocation recovery declined.")?
                    } else { Rc::clone(node.get_type()) };
                captures.entry(source).or_insert_with(|| ScopeCapture { source, ty, uses: Vec::new() }).uses.push((consumer, slot));
            }
        }
    }
    for capture in captures.values_mut() { capture.uses.sort_unstable(); capture.uses.dedup(); }
    Ok(captures.into_values().collect())
}

fn masked_alignment(fd: &Funcdata, value: VarnodeId) -> Option<(u128, i128)> {
    let (base, offset) = canonical_base(fd, value, &mut BTreeSet::new());
    let (_, align) = alignment(fd, fd.vbank().get(base)?.get_def()?)?;
    Some((align, offset))
}
fn alignment_from_base(fd: &Funcdata, allocation: &Allocation, base_align: u128, base_offset: u64) -> Option<(u64, u64)> {
    let (amount_align, amount_offset) = masked_alignment(fd, allocation.amount).unwrap_or((1, 0));
    let scale = allocation.scale as u128;
    let amount_align = amount_align.checked_mul(scale & scale.wrapping_neg()).unwrap_or(u128::MAX);
    let align = base_align.min(amount_align) as u64;
    if align < 2 { return None; }
    let residue = base_offset.wrapping_sub((amount_offset as u64).wrapping_mul(scale as u64));
    Some((align, residue & (align - 1)))
}
/// A rounded count alone says nothing about its native base's alignment.
fn allocation_alignment(fd: &Funcdata, allocation: &Allocation) -> Option<(u64, u64)> {
    let (align, offset) = masked_alignment(fd, allocation.base)?;
    alignment_from_base(fd, allocation, align, offset as u64)
}
fn witnessed_alignment(
    fd: &Funcdata, allocation: &Allocation, equations: &FrameEquations, witnesses: &[(VarnodeId, u64)],
) -> Result<Option<(u64, u64)>, &'static str> {
    let mut result = allocation_alignment(fd, allocation);
    for &(value, align) in witnesses {
        let next = if let Some(delta) = equations.delta(fd, value, allocation.output) {
            Some((align, (0_u64.wrapping_sub(delta as u64)) & (align - 1)))
        } else {
            equations.delta(fd, value, allocation.base).and_then(|delta|
                alignment_from_base(fd, allocation, align as u128, 0_u64.wrapping_sub(delta as u64)))
        };
        let Some(next) = next else { continue };
        if let Some(previous) = result {
            if (previous.1 ^ next.1) & (previous.0.min(next.0) - 1) != 0 {
                return Err("Native stack alignment witnesses disagree; allocation recovery declined.");
            }
        }
        if result.is_none_or(|previous| previous.0 < next.0) { result = Some(next); }
    }
    Ok(result)
}

fn scaled_amount(fd: &mut Funcdata, allocation: &Allocation, width: int4, ty: &Rc<crate::dtype::Datatype>) -> VarnodeId {
    if allocation.scale == 1 { return allocation.amount; }
    let address = fd.obank().get(allocation.op).unwrap().get_addr().clone();
    let cast = fd.new_op(1, address.clone());
    fd.op_set_opcode_code(cast, OpCode::CPUI_CAST);
    fd.op_set_input(cast, allocation.amount, 0).unwrap();
    let unsigned = fd.new_unique_out(width, cast).unwrap();
    fd.vn_update_type_locked(unsigned, Rc::clone(ty), true, true);
    fd.op_insert_before(cast, allocation.op);
    let product = fd.new_op(2, address);
    fd.op_set_opcode_code(product, OpCode::CPUI_INT_MULT);
    let scale = fd.new_constant(width, allocation.scale as u64);
    fd.op_set_all_input(product, &[unsigned, scale]).unwrap();
    let amount = fd.new_unique_out(width, product).unwrap();
    fd.vn_update_type_locked(amount, Rc::clone(ty), true, true);
    fd.op_insert_before(product, allocation.op);
    amount
}

fn lower(fd: &mut Funcdata, allocation: Allocation, amount: VarnodeId, width: int4, headroom: Reservation, alignment: Option<(u64, u64)>, end: Option<BlockId>) {
    let op = allocation.op;
    let address = fd.obank().get(op).unwrap().get_addr().clone();
    let amount = if fd.vbank().get(amount).unwrap().is_free() {
        fd.clone_varnode(amount)
    } else { amount };
    let align = alignment.map(|(align, _)| align);
    // The native decrement wraps at address width; compiler-only backing
    // padding must be added after that truncation, not wrapped with it.
    let count_width = if width < 8 && (headroom.count != 0 || align.is_some()) { width * 2 } else { width };
    let amount = if count_width != width {
        let cast = fd.new_op(1, address.clone());
        fd.op_set_opcode_code(cast, OpCode::CPUI_CAST);
        fd.op_set_input(cast, amount, 0).unwrap();
        let native = fd.new_unique_out(width, cast).unwrap();
        let ty = fd.get_arch().types_rc().unwrap().get_base(width, type_metatype::TYPE_UINT).unwrap();
        fd.vn_update_type_locked(native, ty, true, true);
        fd.op_insert_before(cast, op);
        let extend = fd.new_op(1, address.clone());
        fd.op_set_opcode_code(extend, OpCode::CPUI_INT_ZEXT);
        fd.op_set_input(extend, native, 0).unwrap();
        let widened = fd.new_unique_out(count_width, extend).unwrap();
        fd.op_insert_before(extend, op);
        widened
    } else { amount };
    let count_op = fd.new_op(2, address.clone());
    fd.op_set_opcode_code(count_op, OpCode::CPUI_INT_ADD);
    fd.op_set_input(count_op, amount, 0).unwrap();
    let reserve = fd.new_constant(count_width, headroom.count + align.map_or(0, |align| align as u64 - 1));
    fd.op_set_input(count_op, reserve, 1).unwrap();
    let count = fd.new_unique_out(count_width, count_op).unwrap();
    fd.op_insert_before(count_op, op);
    let allocate = fd.new_op(if end.is_some() { 3 } else { 2 }, address);
    fd.op_set_opcode_code(allocate, OpCode::CPUI_CALLOTHER);
    let builtin = fd.new_constant(4, BUILTIN_STACK_ALLOC as u64);
    fd.op_set_input(allocate, builtin, 0).unwrap();
    fd.op_set_input(allocate, count, 1).unwrap();
    let buffer = fd.new_unique_out(width, allocate).unwrap();
    if end.is_some() {
        let scoped = fd.new_constant(4, 1);
        fd.op_set_input(allocate, scoped, 2).unwrap();
        fd.obank_mut().get_mut(allocate).unwrap().set_hold_output();
        fd.vbank_mut().get_mut(buffer).unwrap().set_explicit();
    }
    fd.op_insert_before(allocate, op);
    let buffer = if let Some((align, residue)) = alignment {
        let residue = residue.wrapping_sub(headroom.shift) & (align - 1);
        let bump = fd.new_op(2, fd.obank().get(op).unwrap().get_addr().clone());
        fd.op_set_opcode_code(bump, OpCode::CPUI_INT_ADD);
        let extra = fd.new_constant(width, align - 1 - residue);
        fd.op_set_all_input(bump, &[buffer, extra]).unwrap();
        let bumped = fd.new_unique_out(width, bump).unwrap();
        fd.op_insert_before(bump, op);
        let mask = fd.new_op(2, fd.obank().get(op).unwrap().get_addr().clone());
        fd.op_set_opcode_code(mask, OpCode::CPUI_INT_AND);
        let bits = fd.new_constant(width, (align as u64).wrapping_neg() & if width == 8 { u64::MAX } else { (1u64 << (8 * width)) - 1 });
        fd.op_set_all_input(mask, &[bumped, bits]).unwrap();
        let aligned = fd.new_unique_out(width, mask).unwrap();
        fd.op_insert_before(mask, op);
        let adjust = fd.new_op(2, fd.obank().get(op).unwrap().get_addr().clone());
        fd.op_set_opcode_code(adjust, OpCode::CPUI_INT_ADD);
        let residue = fd.new_constant(width, residue);
        fd.op_set_all_input(adjust, &[aligned, residue]).unwrap();
        let congruent = fd.new_unique_out(width, adjust).unwrap();
        fd.op_insert_before(adjust, op);
        congruent
    } else { buffer };
    fd.op_set_opcode_code(op, OpCode::CPUI_INT_ADD);
    let reserve = fd.new_constant(width, headroom.shift);
    fd.op_set_all_input(op, &[buffer, reserve]).unwrap();
    if let Some(end) = end {
        fd.split_stack_scope_before(allocate);
        fd.kuna_stack_scopes.push((allocate, end));
        fd.structure_reset();
    }
}

struct CheckpointPlan {
    address: Address,
    size: int4,
    source: VarnodeId,
    stores: Vec<OpId>,
    /// Exact reloads proved to return `source`; release forwards them.
    reloads: Vec<OpId>,
}

fn checkpoint_value_unused(fd: &Funcdata, value: VarnodeId, address: Option<&Address>, size: int4, allocations: &[Allocation], native_restores: &[(OpId, usize)], planned_reads: &[(OpId,int4,i128)]) -> bool {
    let mut todo = vec![value];
    let mut seen = BTreeSet::new();
    while let Some(value) = todo.pop() {
        if !seen.insert(value) { continue; }
        let Some(node) = fd.vbank().get(value) else { return false };
        if node.is_volatile() || crate::kuna_stackalias::holds_store(fd, value)
            || crate::kuna_localstackviews::native_store(fd, value) { return false; }
        for user in node.descend_iter() {
            let op = fd.obank().get(user).unwrap();
            if op.is_dead() { continue; }
            if (0..op.num_input()).filter(|&slot|op.get_in(slot)==Some(value))
                .all(|slot|planned_reads.iter().any(|&(id,input,_)|id==user && input==slot)) {continue;}
            if op.code() == OpCode::CPUI_STORE && op.get_in(2) == Some(value)
                && op.get_in(1) != Some(value)
                && native_restores.iter().any(|&(store, _)| store == user) { continue; }
            // A relocation heritage made a direct link-word COPY: the commit
            // rewrites its input, so the transported value has no use there.
            if op.code() == OpCode::CPUI_COPY && native_restores.iter().any(|&(store, _)| store == user) { continue; }
            if allocations.iter().any(|a| a.op == user && op.get_in(0) == Some(value)) { continue; }
            let Some(output) = op.get_out() else { return false };
            let out = fd.vbank().get(output).unwrap();
            if op.code() == OpCode::CPUI_COPY && stack_register(fd, output)
                && !out.descend_iter().any(|id| fd.obank().get(id).is_some_and(|op| !op.is_dead())) { continue; }
            if !matches!(op.code(), OpCode::CPUI_COPY | OpCode::CPUI_CAST | OpCode::CPUI_INT_ADD
                | OpCode::CPUI_INT_SUB | OpCode::CPUI_INT_AND | OpCode::CPUI_INT_OR | OpCode::CPUI_INT_XOR
                | OpCode::CPUI_INT_2COMP | OpCode::CPUI_INT_ZEXT | OpCode::CPUI_INT_SEXT
                | OpCode::CPUI_SUBPIECE | OpCode::CPUI_PIECE | OpCode::CPUI_PTRADD | OpCode::CPUI_PTRSUB
                | OpCode::CPUI_MULTIEQUAL | OpCode::CPUI_INDIRECT) { return false; }
            if out.is_persist() { return false; }
            if let Some(address) = address {
                if (out.is_addr_tied() && out.get_space().get_index() != address.get_space().unwrap().get_index())
                    || (out.get_space().get_index() == address.get_space().unwrap().get_index()
                        && (out.get_addr() != address || out.get_size() != size)) { return false; }
            } else if out.is_addr_tied() || fd.get_arch().manage().get_stack_space()
                .is_some_and(|stack| out.get_space().get_index() == stack.get_index()) { return false; }
            todo.push(output);
        }
    }
    true
}

// OR only sets bits, so an unsigned address `p | c` lies in `[p, p + c]`.
// Masked frame reads therefore keep a bounded byte range without assuming
// stack alignment.
fn masked_frame_range(fd: &Funcdata, facts: &FrameFacts, pointer: VarnodeId, entry: VarnodeId) -> Option<(i128, i128)> {
    let op = fd.obank().get(fd.vbank().get(strip(fd, pointer))?.get_def()?)?;
    if op.code() != OpCode::CPUI_INT_OR { return None; }
    (0..2).find_map(|slot| {
        let mask = number(fd, op.get_in(1 - slot)?).filter(|mask| (0..=0xffff).contains(mask))?;
        let offset = facts.equations.delta(fd, op.get_in(slot)?, entry)?;
        Some((offset, offset + mask))
    })
}
/// A reload of a private checkpoint slot that the frame facts prove equal to
/// the slot's single stored value, which is available at the reload.
fn checkpoint_reload_forwards(fd: &Funcdata, facts: &FrameFacts, plan: &CheckpointPlan, load: OpId) -> bool {
    let Some(output) = fd.obank().get(load).and_then(|op| op.get_out()) else { return false };
    let stored = |store: &OpId| fd.obank().get(*store).and_then(|op| op.get_in(2));
    plan.stores.iter().all(|store| stored(store).is_some_and(|value| facts.equations.delta(fd, value, plan.source) == Some(0)))
        && facts.equations.delta(fd, output, plan.source) == Some(0)
        && fd.vbank().get(plan.source).is_some_and(|source| source.get_def()
            .is_none_or(|definition| dominates_op(fd, definition, load)))
}
fn unused_checkpoint_slots(fd: &Funcdata, facts: &FrameFacts, width: int4, planned_reads: &[(OpId,int4,i128)]) -> Vec<CheckpointPlan> {
    let allocations=&facts.roots;
    if fd.get_arch().stack_alias_deadstore || fd.get_arch().stack_views { return Vec::new(); }
    let Some(space) = fd.get_arch().manage().get_stack_space() else { return Vec::new() };
    let Some(entry) = fd.find_spacebase_input(space) else { return Vec::new() };
    let bias = stack_bias(fd, width) as i128;
    let bank = fd.vbank();
    let at = |off| Address::new(Rc::clone(space), off);
    let eligible = |first: i128, source| first + width as i128 <= bias && allocations.iter().any(|a|
        facts.equations.delta(fd,a.upper_base(fd),entry).is_some_and(|origin| first >= bias + origin)
            && facts.equations.delta(fd,source,a.before) == Some(0));
    let mut candidates = BTreeMap::<u64, CheckpointPlan>::new();
    for id in fd.obank().iter_alive() {
        let op = fd.obank().get(id).unwrap();
        if op.code() != OpCode::CPUI_STORE || op.is_dead() { continue; }
        if op.get_in(0).and_then(|v| number(fd, v)) != space.get_contain().map(|s| s.get_index() as i128) { continue; }
        let Some(source) = op.get_in(2).filter(|&v| bank.get(v).is_some_and(|v| v.get_size() == width)) else { continue };
        let Some(first) = facts.input_value(fd,id,1).and_then(|pointer|facts.equations.delta(fd,pointer,entry)) else { continue };
        if !eligible(first, source) { continue; }
        candidates.entry(first as u64 & calc_mask(width)).or_insert_with(|| CheckpointPlan {
            address: at(first as u64 & calc_mask(width)), size: width, source, stores: Vec::new(), reloads: Vec::new(),
        }).stores.push(id);
    }
    for id in bank.iter_loc_addr_range(&at(0), &at(1)).chain(bank.iter_loc_addr_range(&at(1), &at(0))) {
        let value = bank.get(id).unwrap();
        if value.get_size() != width { continue; }
        let Some(op) = value.get_def().and_then(|id| fd.obank().get(id)) else { continue };
        if op.code() != OpCode::CPUI_INDIRECT || op.is_indirect_creation() { continue; }
        let Some(source) = op.get_in(0) else { continue };
        let first = sign_extend(value.get_offset() as i64, width * 8 - 1) as i128;
        if eligible(first, source) {
            candidates.entry(value.get_offset()).or_insert_with(|| CheckpointPlan {
                address: value.get_addr().clone(), size: width, source, stores: Vec::new(), reloads: Vec::new(),
            });
        }
    }
    candidates.into_values().filter(|plan| {
        if crate::kuna_localstackviews::preserves_range(fd, &plan.address, width) { return false; }
        let first = sign_extend(plan.address.get_offset() as i64, width * 8 - 1) as i128;
        if bank.iter_loc_size_addr(width, &plan.address).any(|id| bank.get(id).unwrap().is_auto_live_hold()
            || !checkpoint_value_unused(fd, id, Some(&plan.address), width, allocations, &[], planned_reads)) { return false; }
        let word=FrameWordUse {origin_only: false,result:None,source:plan.source,first:Some(first),pointer:None,size:width,
            read:None,birth:None,alias:false};
        for id in fd.obank().iter_alive() {
            let op = fd.obank().get(id).unwrap();
            if op.is_dead() || plan.stores.contains(&id) { continue; }
            if op.code() == OpCode::CPUI_LOAD && op.get_in(0).and_then(|v| number(fd, v)) == space.get_contain().map(|s| s.get_index() as i128) {
                let Some(output) = op.get_out() else { return false };
                let size = bank.get(output).unwrap().get_size();
                let pointer = op.get_in(1).unwrap();
                if let Some(offset) = facts.input_value(fd,id,1).and_then(|pointer|facts.equations.delta(fd,pointer,entry)) {
                    if offset >= first + width as i128 || offset + size as i128 <= first { continue; }
                    if offset == first && size == width && (checkpoint_value_unused(fd, output, Some(&plan.address), width, allocations, &[], planned_reads)
                        || checkpoint_reload_forwards(fd, facts, plan, id)) { continue; }
                    return false;
                }
                if let Some((low, high)) = facts.input_value(fd,id,1).and_then(|pointer|masked_frame_range(fd,facts,pointer,entry)) {
                    if low >= first + width as i128 || high + size as i128 <= first { continue; }
                    return false;
                }
                if facts.store_reaches(fd,pointer,size,word,Some((id,1))).unwrap_or(true) { return false; }
                continue;
            }
            // Pure address construction is not an escape. Check the effects
            // which can publish a word address or observe it externally.
            if matches!(op.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND | OpCode::CPUI_CALLOTHER | OpCode::CPUI_STORE | OpCode::CPUI_RETURN)
                || op.get_out().and_then(|out| bank.get(out)).is_some_and(|out| out.is_addr_tied()
                    && space.get_contain().is_some_and(|s| s.get_index() == out.get_space().get_index()))
            {
                // An INDIRECT's second input names the effecting op; it is not a value.
                let values = if op.code()==OpCode::CPUI_INDIRECT {1} else {op.num_input()};
                if ((if op.code()==OpCode::CPUI_CALL {1} else {0})..values).any(|slot| facts.escapes(fd,op.get_in(slot).unwrap(),word,Some((id,slot)))) { return false; }
            }
        }
        let mut exits = false;
        let private = fd.obank().iter_alive().filter(|&id| fd.obank().get(id).is_some_and(|op| !op.is_dead() && op.code() == OpCode::CPUI_RETURN)).all(|exit| {
            exits = true;
            frame_word_uses(fd,FrameRead::Operation(exit),None,&mut 65536).is_some_and(|span|
                facts.check_word(fd,FrameWordUse {origin_only: false,read:Some(FrameRead::Operation(exit)),..word},&span.into_iter().collect::<Vec<_>>()))
        });
        private && exits
    }).map(|mut plan| {
        let first = sign_extend(plan.address.get_offset() as i64, width * 8 - 1) as i128;
        plan.reloads = fd.obank().iter_alive().filter(|&id| fd.obank().get(id).is_some_and(|op| !op.is_dead()
            && op.code() == OpCode::CPUI_LOAD
            && op.get_out().and_then(|out| bank.get(out)).is_some_and(|out| out.get_size() == width)
            && facts.input_value(fd,id,1).and_then(|pointer| facts.equations.delta(fd,pointer,entry)) == Some(first)
            && !checkpoint_value_unused(fd, op.get_out().unwrap(), Some(&plan.address), width, allocations, &[], planned_reads)
            && checkpoint_reload_forwards(fd, facts, &plan, id))).collect();
        plan
    }).collect()
}

fn release_checkpoint_slots(fd: &mut Funcdata, slots: &[CheckpointPlan]) {
    for plan in slots {
        for &load in &plan.reloads {
            let output = fd.obank().get(load).and_then(|op| op.get_out()).unwrap();
            fd.total_replace(output, plan.source).unwrap();
            fd.op_destroy(load);
        }
        for &store in &plan.stores { fd.op_destroy(store); }
        let values: Vec<_> = fd.vbank().iter_loc_size_addr(plan.size, &plan.address).collect();
        for value in values { if let Some(node) = fd.vbank_mut().get_mut(value) { node.clear_addr_force(); } }
        fd.scope_local_mark_not_mapped(plan.address.get_space().unwrap(), plan.address.get_offset(), plan.size, false);
    }
}




/// Rebase exact fixed-frame memory addresses without replaying stale spill reads.


/// Expose preserved fixed-frame pointer reads to ordinary stack-variable recovery.
fn fixed_frame_reads(fd: &Funcdata, facts: &FrameFacts, ends: &[Option<LifetimeEnd>], width: int4) -> Vec<(OpId, int4, i128)> {
    let allocations=&facts.roots;
    let Some(stack) = fd.get_arch().manage().get_stack_space() else { return Vec::new() };
    if stack.get_word_size() != 1 || stack.get_contain().is_none_or(|space| space.get_word_size() != 1) {
        return Vec::new();
    }
    let Some(entry) = fd.find_spacebase_input(stack) else { return Vec::new() };
    let bias = stack_bias(fd, width) as i128;
    let Some((first, rest)) = allocations.split_first() else { return Vec::new() };
    let Some(first) = facts.equations.delta(fd,first.upper_base(fd),entry).and_then(|offset| bias.checked_add(offset))
    else { return Vec::new() };
    let Some(floor) = rest.iter().try_fold(first, |floor, allocation|
        facts.equations.delta(fd,allocation.upper_base(fd),entry)
            .and_then(|offset| bias.checked_add(offset)).map(|offset| floor.max(offset)))
    else { return Vec::new() };
    let mut reads = Vec::new();
    for id in fd.obank().iter_alive() {
        let Some(op) = fd.obank().get(id) else { continue };
        if (op.is_marker() && op.code() != OpCode::CPUI_MULTIEQUAL)
            || op.get_out().is_some_and(|out| stack_register(fd, out)) { continue; }
        let outside=allocations.iter().zip(ends).all(|(root,end)|
            dominates_op(fd,id,root.op) || end.as_ref().is_some_and(|end|after_scope_end(fd,id,end)));
        for slot in 0..op.num_input() {
            let Some(input) = op.get_in(slot) else { continue };
            // Normalize the evaluated memory address, not just stack-word
            // carriers. Outgoing ABI slots can be addressed through a GPR.
            if slot==1 && matches!(op.code(),OpCode::CPUI_LOAD|OpCode::CPUI_STORE)
                && op.get_in(0).and_then(|value|number_using(fd,value,&|value|facts.equations.value(fd,value)))==Some(facts.memory)
                && fd.vbank().get(input).is_some_and(|node|node.get_size()==width && !node.is_volatile())
            {
                if let Some(offset)=facts.input_value(fd,id,slot).and_then(|value|facts.equations.delta(fd,value,entry))
                    .filter(|&offset|offset<bias && (offset>=floor || outside))
                {reads.push((id,slot,offset));continue;}
            }
            let Some(_) = fd.vbank().get(input).filter(|node| !node.is_volatile()
                && node.get_space().get_index() == stack.get_index() && node.get_size() == width
                && matches!(node.get_type().get_metatype(), type_metatype::TYPE_UNKNOWN
                    | type_metatype::TYPE_PTR | type_metatype::TYPE_INT | type_metatype::TYPE_UINT))
            else { continue };
            let Some(value)=facts.input_value(fd,id,slot) else {continue};
            let Some(offset)=facts.equations.delta(fd,value,entry) else {continue};
            if offset < floor || offset >= bias { continue; }
            reads.push((id, slot, offset));
        }
    }
    reads
}

struct AllocationPlan {
    unsigned_count: Option<Rc<Datatype>>,
    links: Vec<(OpId,usize)>,
    native_values: Vec<OpId>,
    displaced: Vec<(OpId,VarnodeId,i128)>,
    ends: Vec<Option<LifetimeEnd>>,
    captures: Vec<ScopeCapture>,
    merges: BTreeMap<VarnodeId,Rc<Datatype>>,
    headroom: Vec<Reservation>,
    alignments: Vec<Option<(u64, u64)>>,
    reads: Vec<(OpId,int4,i128)>,
    frame: Vec<(i64,int4)>,
    checkpoints: Vec<CheckpointPlan>,
}

impl AllocationPlan {
fn prepare(
    fd: &Funcdata, facts: &FrameFacts, aliases: &[(VarnodeId, VarnodeId)],
    width: int4, native_roots: &[OpId], native: &[(OpId,VarnodeId,bool)], witnesses: &[(VarnodeId,u64)],
) -> Result<Self, &'static str> {
    let allocations=&facts.roots;
    let unsigned_count = if allocations.iter().any(|allocation| allocation.scale != 1) {
        Some(fd.get_arch().types_rc().and_then(|factory| factory.get_base(width, type_metatype::TYPE_UINT).ok())
            .ok_or("Native scaled stack count has no unsigned representation; allocation recovery declined.")?)
    } else { None };
    let links = native_frame_links(fd,facts,width,native)?;
    let mut native_values=Vec::new();
    for &(store,_,_,_,load) in &facts.transports {
        if !links.iter().any(|&(id,_)|id==store) {continue;}
        let mut value=fd.obank().get(store).and_then(|op|op.get_in(if direct_frame_word(fd,store,width).is_some() {0} else {2}))
            .ok_or("Native transport lost its payload; allocation recovery declined.")?;
        loop {
            let producer=fd.vbank().get(value).and_then(|node|node.get_def())
                .ok_or("Native transport lost its producer; allocation recovery declined.")?;
            native_values.push(producer);
            if producer==load {break;}
            let op=fd.obank().get(producer).ok_or("Native transport producer is absent; allocation recovery declined.")?;
            if !matches!(op.code(),OpCode::CPUI_COPY|OpCode::CPUI_CAST) {
                return Err("Native transport chain changed; allocation recovery declined.");
            }
            value=op.get_in(0).ok_or("Native transport chain is incomplete; allocation recovery declined.")?;
        }
    }
    let displaced = displaced_aliases(fd, allocations, aliases);
    let mut ends = Vec::with_capacity(allocations.len());
    for allocation in allocations {
        let end = source_lifetime(fd, allocation, facts)?;
        if restoration_failure_before(fd, allocation, native_roots, end.as_ref(), facts).is_some() {
            return Err("Native stack lifetime has no representable source boundary; allocation recovery declined.");
        }
        if end.is_none() && loop_restores_stack(fd, allocation) {
            return Err("Native stack lifetime requires a lexical scope; allocation recovery declined.");
        }
        ends.push(end);
    }
    let unscoped: Vec<_> = allocations.iter().zip(&ends).filter_map(|(&allocation, end)| end.is_none().then_some(allocation)).collect();
    if !sequential_reservations_survive(fd, &unscoped) {
        return Err("Native stack reservations do not share a function lifetime; allocation recovery declined.");
    }
    let captures = scope_captures(fd, facts, aliases, &ends, &displaced, &links)?;
    let mut merges = BTreeMap::new();
    for capture in &captures {
        for &(consumer, _) in &capture.uses {
            let op = fd.obank().get(consumer).unwrap();
            if op.code() != OpCode::CPUI_MULTIEQUAL { continue; }
            let output = op.get_out().ok_or("A scoped merge has no value; allocation recovery declined.")?;
            let node = fd.vbank().get(output).unwrap();
            if node.is_type_lock() && node.get_type().get_metatype() == type_metatype::TYPE_PTR {
                return Err("A scoped merge has a locked pointer representation; allocation recovery declined.");
            }
            for slot in 0..op.num_input() {
                if captures.iter().any(|capture| capture.uses.contains(&(consumer, slot))) { continue; }
                let input = fd.vbank().get(op.get_in(slot).unwrap()).unwrap();
                if input.get_size() != node.get_size() || (!input.is_constant()
                    && !matches!(input.get_type().get_metatype(), type_metatype::TYPE_UNKNOWN
                        | type_metatype::TYPE_INT | type_metatype::TYPE_UINT | type_metatype::TYPE_BOOL))
                {
                    return Err("A scoped merge has an unsupported incoming representation; allocation recovery declined.");
                }
            }
            merges.insert(output, if node.is_type_lock() { Rc::clone(node.get_type()) } else { Rc::clone(&capture.ty) });
        }
    }
    let headroom = reserves(fd, allocations, aliases, &displaced, width)
        .ok_or("Native stack backing extent is unresolved; allocation recovery declined.")?;
    let alignments = allocations.iter().map(|allocation|
        witnessed_alignment(fd, allocation, &facts.equations, witnesses)).collect::<Result<Vec<_>, _>>()?;
    if alignments.iter().zip(&headroom).any(|(alignment, reserve)|
        alignment.is_some_and(|(align, _)|
            reserve.count.checked_add(align - 1).is_none_or(|count| count > calc_mask(width))))
    {
        return Err("Native stack alignment exceeds the backing address width; allocation recovery declined.");
    }
    let reads = fixed_frame_reads(fd,facts,&ends,width);
    let frame = fixed_frame_writes(fd);
    let checkpoints = unused_checkpoint_slots(fd, facts, width, &reads);
    Ok(Self {unsigned_count,links,native_values,displaced,ends,captures,merges,headroom,alignments,reads,frame,checkpoints})
}
fn commit(self, fd: &mut Funcdata, allocations: &[Allocation], aliases: &[(VarnodeId,VarnodeId)], width: int4) {
    let Self {unsigned_count,links,native_values,displaced,mut ends,captures,merges,headroom,alignments,reads,frame,checkpoints}=self;
    for (((allocation, reserve), end), alignment) in allocations.iter().zip(&headroom).zip(&ends).zip(&alignments) {
        let mut note = allocation_origin_note(fd, allocation, reserve);
        if width == 2 && (reserve.count != 0 || alignment.is_some()) {
            note.push_str(" Recompilation requires a representable backing size and room for the compiler's own stack frame.");
        }
        fd.note_stack_alloc(note);
        if let Some(end) = end {
            fd.note_stack_alloc(format!("Stack storage ends at native SP restoration {:#x}; compiler backing is scoped.", end.address));
        }
    }
    for (output, ty) in merges {
        fd.vn_update_type_locked(output, ty, true, true);
        fd.vbank_mut().get_mut(output).unwrap().set_explicit();
    }
    if !reads.is_empty() {
        let stack = fd.get_arch().manage().get_stack_space().cloned().unwrap();
        let entry = fd.find_spacebase_input(&stack).unwrap();
        for (id, slot, offset) in reads {
            let address = fd.new_op(2, fd.obank().get(id).unwrap().get_addr().clone());
            fd.op_set_opcode_code(address, OpCode::CPUI_INT_ADD);
            fd.op_set_input(address, entry, 0).unwrap();
            let displacement = fd.new_constant(width, offset as u64 & calc_mask(width));
            fd.op_set_input(address, displacement, 1).unwrap();
            let pointer = fd.new_unique_out(width, address).unwrap();
            let input = fd.obank().get(id).unwrap().get_in(slot).unwrap();
            if fd.vbank().get(input).unwrap().is_type_lock() {
                let ty = Rc::clone(fd.vbank().get(input).unwrap().get_type());
                fd.vn_update_type_locked(pointer, ty, true, true);
            }
            if fd.obank().get(id).unwrap().code() == OpCode::CPUI_MULTIEQUAL {
                let join = fd.obank().get(id).unwrap().get_parent().unwrap();
                let predecessor = fd.bblocks_ref().block(join).get_in(slot);
                let mut before = fd.bb_op_head(predecessor);
                while let Some(next) = before {
                    let op = fd.obank().get(next).unwrap();
                    if op.code() != OpCode::CPUI_MULTIEQUAL { break; }
                    before = op.basic_neighbours().1;
                }
                if let Some(before) = before { fd.op_insert_before(address, before); }
                else { fd.op_insert_end(address, predecessor); }
            } else { fd.op_insert_before(address, id); }
            fd.op_set_input(id, pointer, slot).unwrap();
        }
    }
    // CFG splitting preserves native semantics and happens only after all value
    // and backing plans have succeeded.
    for end in ends.iter_mut().flatten() {
        // Keep the inside exit join as well as the outside boundary. Removing
        // a marker-only join would move conditional exits across the scope
        // graph boundary, where edge grouping loses their conditions.
        let inside=fd.obank().get(end.restore).unwrap().get_parent().unwrap();
        fd.bblocks_mut().block_mut(inside).set_flag(crate::block::block_flags::f_stack_scope_boundary);
        end.boundary = Some(fd.split_stack_scope_after(end.restore));
    }
    for (op, root, delta) in displaced {
        let size = fd.vbank().get(fd.obank().get(op).unwrap().get_out().unwrap()).unwrap().get_size();
        fd.op_set_opcode_code(op, OpCode::CPUI_INT_ADD);
        let displacement = fd.new_constant(size, delta as u64 & calc_mask(size));
        fd.op_set_all_input(op, &[root, displacement]).unwrap();
    }
    for &(alias, original) in aliases { fd.total_replace(alias, original).unwrap(); }
    for capture in captures {
        let def = fd.vbank().get(capture.source).unwrap().get_def().unwrap();
        let size = fd.vbank().get(capture.source).unwrap().get_size();
        let snapshot = fd.new_op(1, fd.obank().get(def).unwrap().get_addr().clone());
        fd.op_set_opcode_code(snapshot, OpCode::CPUI_CAST);
        fd.op_set_input(snapshot, capture.source, 0).unwrap();
        let value = fd.new_unique_out(size, snapshot).unwrap();
        fd.vn_update_type_locked(value, capture.ty, true, true);
        fd.vbank_mut().get_mut(value).unwrap().set_explicit();
        fd.vbank_mut().get_mut(value).unwrap().set_auto_live_hold();
        fd.obank_mut().get_mut(snapshot).unwrap().set_hold_output();
        fd.kuna_stack_scope_holds.push(value);
        let mut point = if fd.obank().get(def).unwrap().code() == OpCode::CPUI_INDIRECT {
            let effect = fd.obank().get(def).unwrap().get_in(1).unwrap();
            crate::funcdata_varnode::op_iop_decode(fd.vbank().get(effect).unwrap().get_offset())
        } else { def };
        while let Some(next) = fd.obank().get(point).unwrap().basic_neighbours().1 {
            if fd.obank().get(next).unwrap().code() != OpCode::CPUI_MULTIEQUAL { break; }
            point = next;
        }
        fd.op_insert_after(snapshot, point);
        for (consumer, slot) in capture.uses { fd.op_set_input(consumer, value, slot).unwrap(); }
    }
    for (((&allocation, reserve), alignment), end) in allocations.iter().zip(headroom).zip(alignments).zip(ends) {
        let amount = if let Some(ty) = unsigned_count.as_ref() {
            scaled_amount(fd, &allocation, width, ty)
        } else { allocation.amount };
        lower(fd, allocation, amount, width, reserve, alignment, end.and_then(|end| end.boundary));
    }
    release_checkpoint_slots(fd, &checkpoints);
    // A relocation heritage turned into a direct link-word COPY keeps its word:
    // it now writes the link payload the header held, without the header read.
    let direct: Vec<(OpId, VarnodeId)> = links.iter().filter(|&&(store, _)| direct_frame_word(fd, store, width).is_some())
        .filter_map(|&(store, index)| links.iter().find(|&&(link, other)| other == index && direct_frame_word(fd, link, width).is_none())
            .and_then(|&(link, _)| fd.obank().get(link)?.get_in(2)).map(|payload| (store, payload))).collect();
    for &(store, payload) in &direct { fd.op_set_input(store, payload, 0).unwrap(); }
    for (store, _) in links { if !direct.iter().any(|&(id, _)| id == store) { fd.op_destroy(store); } }
    for value in native_values { fd.op_destroy(value); }
    fd.record_frame_writes(frame);
}
}

fn materialize_allocations(fd: &mut Funcdata, facts: &FrameFacts, aliases: &[(VarnodeId,VarnodeId)],
    width: int4, native_roots: &[OpId], native: &[(OpId,VarnodeId,bool)], witnesses: &[(VarnodeId,u64)]) -> Result<Vec<(VarnodeId,u64)>, &'static str>
{
    let plan=AllocationPlan::prepare(fd,facts,aliases,width,native_roots,native,witnesses)?;
    let shifts=plan.headroom.iter().zip(&facts.roots).map(|(reserve,allocation)|(allocation.output,reserve.shift)).collect();
    plan.commit(fd,&facts.roots,aliases,width);
    Ok(shifts)
}

/// Raw zero rotates must keep their native identity before the frame pool folds them.
fn alignment_zero_rotate_input(fd: &Funcdata, value: VarnodeId) -> Option<VarnodeId> {
    let value = strip(fd, value);
    let width = fd.vbank().get(value)?.get_size();
    if !matches!(width, 2 | 4 | 8) { return None; }
    let op = fd.obank().get(fd.vbank().get(value)?.get_def()?)?;
    if op.code() != OpCode::CPUI_INT_OR { return None; }
    (0..2).find_map(|slot| {
        let identity = strip(fd, op.get_in(slot)?);
        let zero = strip(fd, op.get_in(1 - slot)?);
        if fd.vbank().get(identity)?.get_size() != width || fd.vbank().get(zero)?.get_size() != width {
            return None;
        }
        let identity = fd.obank().get(fd.vbank().get(identity)?.get_def()?)?;
        let zero = fd.obank().get(fd.vbank().get(zero)?.get_def()?)?;
        if !matches!((identity.code(), zero.code()),
            (OpCode::CPUI_INT_LEFT, OpCode::CPUI_INT_RIGHT) | (OpCode::CPUI_INT_RIGHT, OpCode::CPUI_INT_LEFT))
        { return None; }
        let source = strip(fd, identity.get_in(0)?);
        if source != strip(fd, zero.get_in(0)?) || fd.vbank().get(source)?.get_size() != width
            || literal(fd, identity.get_in(1)?, &mut 32) != Some(0)
            || literal(fd, zero.get_in(1)?, &mut 32) != Some(width as u64 * 8)
        { return None; }
        Some(source)
    })
}

pub struct ActionStackAlloc {
    base: ActionBase,
    stack_definitions: Vec<OpId>,
    native: Vec<(OpId, VarnodeId, bool)>,
    alignments: Vec<(VarnodeId, u64)>,
    finalized_pass: Option<u64>,
    finished: bool,
}
impl ActionStackAlloc {
    pub fn boxed() -> Box<dyn Action> {
        Box::new(Self { base: ActionBase::new(0, "stackalloc", "stackvars"),
            stack_definitions: Vec::new(), native: Vec::new(), alignments: Vec::new(), finalized_pass: None, finished: false })
    }
    fn release_native(&mut self, fd: &mut Funcdata) {
        fd.kuna_stack_call_sp.clear();
        self.alignments.clear();
        for (_, value, owned) in self.native.drain(..) {
            if let Some(index) = fd.kuna_stack_scope_holds.iter().position(|&held| held == value) {
                fd.kuna_stack_scope_holds.swap_remove(index);
            }
            if owned {
                if let Some(value) = fd.vbank_mut().get_mut(value) { value.clear_auto_live_hold(); }
            }
        }
    }
    /// Keep explicit native masks as alignment facts, not as a different frame coordinate model.
    fn retain_alignments(&mut self, fd: &mut Funcdata, roots: &[Allocation]) {
        if roots.is_empty() { return; }
        let mut masks: Vec<_> = fd.obank().iter_alive().filter_map(|id| {
            let (slot, align) = alignment(fd, id).filter(|&(_, align)| align > 1)?;
            let op = fd.obank().get(id)?;
            let output = op.get_out()?;
            let input = op.get_in(slot)?;
            let related = roots.iter().any(|root|
                source_allocation_address(fd, input, root.output, &mut 256, &mut SavedFrameQuery::default())
                    || source_allocation_address(fd, root.base, output, &mut 256, &mut SavedFrameQuery::default()));
            related.then_some((id, output, align as u64))
        }).collect();
        let low_masks: Vec<_> = fd.obank().iter_alive().filter_map(|id| {
            let op = fd.obank().get(id)?;
            if op.code() != OpCode::CPUI_INT_AND { return None; }
            (0..2).find_map(|slot| {
                let output = op.get_out()?;
                let width = fd.vbank().get(output)?.get_size();
                let input = op.get_in(1 - slot)?;
                let input = alignment_zero_rotate_input(fd, input).unwrap_or(input);
                let mask_node = op.get_in(slot)?;
                if fd.vbank().get(input)?.get_size() != width || fd.vbank().get(mask_node)?.get_size() != width {
                    return None;
                }
                let mask = literal(fd, mask_node, &mut 32)?;
                let align = mask.checked_add(1)?;
                let entry = fd.find_spacebase_input(fd.get_arch().manage().get_stack_space()?)?;
                (align > 1 && align.is_power_of_two() && align <= calc_mask(width)
                    && source_allocation_address(fd, input, entry, &mut 256, &mut SavedFrameQuery::default()))
                    .then_some((input, output, align))
            })
        }).collect();
        // With no low mask, there is nothing to test against the native SP.
        let native = if low_masks.is_empty() { &[][..] } else { self.native.as_slice() };
        for &(id, value, _) in native {
            if !stack_register(fd, value) || !roots.iter().any(|root| native_stack_delta(fd, root.base, value).is_some()) {
                continue;
            }
            let width = fd.vbank().get(value).unwrap().get_size();
            for &(input, low, align) in &low_masks {
                if native_stack_sum(fd, width, vec![(value, 1), (low, 1), (input, -1)], None)
                    .is_some_and(|delta| delta % align as i128 == 0)
                {
                    masks.push((id, value, align));
                }
            }
        }
        for (id, output, align) in masks {
            if let Some((_, previous)) = self.alignments.iter_mut().find(|(value, _)| *value == output) {
                *previous = (*previous).max(align);
                continue;
            }
            self.alignments.push((output, align));
            if self.native.iter().any(|&(_, held, _)| held == output) { continue; }
            let owned = !fd.vbank().get(output).unwrap().is_auto_live_hold();
            fd.vbank_mut().get_mut(output).unwrap().set_auto_live_hold();
            fd.kuna_stack_scope_holds.push(output);
            self.native.push((id, output, owned));
        }
    }
    fn retain_native(&mut self, fd: &mut Funcdata) -> bool {
        if self.native.is_empty() && !has_runtime_stack_reservation(fd) { return false; }
        let definitions: Vec<_> = fd.obank().iter_alive().filter_map(|id|
            fd.obank().get(id).and_then(|op| op.get_out()).filter(|&out| stack_register(fd, out)).map(|out| (id, out)))
            .collect();
        for (id, out) in definitions {
            if self.native.iter().any(|&(_, held, _)| held == out) { continue; }
            let owned = !fd.vbank().get(out).unwrap().is_auto_live_hold();
            fd.vbank_mut().get_mut(out).unwrap().set_auto_live_hold();
            fd.kuna_stack_scope_holds.push(out);
            self.native.push((id, out, owned));
        }
        let calls: Vec<_> = (0..fd.num_calls()).map(|index| fd.get_call_specs(index).get_op()).collect();
        for call in calls {
            let Some(out) = capture_call_stack(fd, call) else { continue };
            if self.native.iter().any(|&(_, held, _)| held == out) { continue; }
            if let Some(id) = fd.vbank().get(out).and_then(|value| value.get_def()) {
                let owned = !fd.vbank().get(out).unwrap().is_auto_live_hold();
                fd.vbank_mut().get_mut(out).unwrap().set_auto_live_hold();
                fd.kuna_stack_scope_holds.push(out);
                self.native.push((id, out, owned));
            }
        }
        let mut roots = Vec::new();
        let mut seen = BTreeSet::new();
        let mut tested = FxHashMap::default();
        let mut lookup = |id| *tested.entry(id).or_insert_with(|| allocation(fd, id));
        for &(id, value, _) in &self.native {
            if !stack_register(fd, value) { continue; }
            visit_candidates(fd, value, &mut |v| seen.insert(v), &mut lookup, &mut |root| roots.push(root));
            roots.extend(folded_reservations(fd, id, value, &mut lookup));
        }
        self.retain_alignments(fd, &roots);
        true
    }
    fn reservations(&mut self, fd: &Funcdata) -> (Vec<Allocation>, Vec<(VarnodeId, VarnodeId)>) {
        let mut found = BTreeMap::new();
        // Native SP definitions often reach the same long, loop-carried
        // chain. Reuse the immutable allocation query, but walk it anew for
        // each write: reservation_point depends on the particular write.
        let mut tested = FxHashMap::default();
        let mut lookup = |id| *tested.entry(id).or_insert_with(|| allocation(fd, id));
        let mut visited = slotmap::SecondaryMap::new();
        for (visit, &(write, out, _)) in self.native.iter().enumerate() {
            if !stack_register(fd, out) { continue; }
            if fd.obank().get(write).is_none_or(|op| op.is_dead() || op.get_out() != Some(out)) { continue; }
            let mut direct = false;
            visit_candidates(fd, out, &mut |v| visited.insert(v, visit) != Some(visit),
                &mut lookup, &mut |candidate| {
                direct = true;
                if reservation_point(fd, &candidate, write) {
                    let mut candidate = candidate;
                    normalize_reservation(fd, &mut candidate);
                    self.stack_definitions.push(candidate.op);
                    found.insert(candidate.op, candidate);
                }
            });
            // Only an SP update that is no reservation itself witnesses folded ones.
            if direct { continue; }
            for mut candidate in folded_reservations(fd, write, out, &mut lookup) {
                normalize_reservation(fd, &mut candidate);
                self.stack_definitions.push(candidate.op);
                found.entry(candidate.op).or_insert(candidate);
            }
        }
        self.stack_definitions.sort_unstable();
        self.stack_definitions.dedup();
        identify_aliases(fd, found.into_values().collect(), &self.stack_definitions)
    }
}
impl Action for ActionStackAlloc {
    fn base(&self) -> &ActionBase { &self.base }
    fn base_mut(&mut self) -> &mut ActionBase { &mut self.base }
    fn clone_filtered(&self, groups: &ActionGroupList) -> Option<Box<dyn Action>> {
        groups.contains(self.get_group()).then(Self::boxed)
    }
    fn reset(&mut self, fd: &mut Funcdata) {
        self.release_native(fd);
        self.base.reset_base();
        self.stack_definitions.clear();
        self.finalized_pass = None;
        self.finished = false;
    }
    fn apply(&mut self, fd: &mut Funcdata, ctx: &mut ActionContext) -> ApplyResult {
        // The jump-table probe discovers targets, not source storage. Leave
        // allocation lowering to the full analysis of the recovered flow.
        if !fd.stack_alloc_on() || fd.is_jumptable_recovery_on()
            || fd.get_heritage_pass() == 0 || self.finished { return 0; }
        let Some(stack) = fd.get_arch().manage().get_stack_space() else { return 0 };
        if !stack.stack_grows_negative() { return 0; }
        let Ok(register) = stack.get_spacebase(0) else { return 0 };
        let width = register.size as int4;
        if !matches!(width, 2 | 4 | 8) || stack.get_addr_size() as int4 != width || stack.get_word_size() != 1
            || stack.get_contain().is_none_or(|space| space.get_word_size() != 1) { return 0; }
        // Frame facts require an incoming SP. Until heritage has one, do not
        // alter native reservations or retain definitions that force a retry.
        if self.native.is_empty() && fd.find_spacebase_input(stack).is_none() { return 0; }
        if unmask_reservations(fd) { fd.touch_stack_alloc(); }
        if !self.retain_native(fd) { return 0; }
        // Holding native values changes the analysis; only a materialized
        // allocation settles it.
        fd.touch_stack_alloc();
        if (0..fd.num_calls()).any(|idx| {
            let call = fd.get_call_specs(idx);
            call.is_input_active() && fd.obank().get(call.get_op()).is_some_and(|op|
                !op.is_dead() && matches!(op.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND))
        }) { return 0; }
        let pass = fd.get_heritage_pass() as u64;
        // Let the ordinary stack-SSA pool finish once after call inputs settle.
        // Held native definitions preserve where storage was actually reserved.
        if self.finalized_pass == Some(pass) { return 0; }
        let (facts, mut aliases) = with_carry_proofs(fd, |fd| {
            let (allocations, aliases) = self.reservations(fd);
            let facts = if allocations.is_empty() && self.finalized_pass.is_some() {
                None
            } else {
                FrameFacts::infer(fd, allocations)
            };
            (facts, aliases)
        });
        let Some(facts)=facts else {
            self.finished=true;
            self.release_native(fd);
            decline(fd, ctx, None);
            return 0;
        };
        // Freeze every equality before mutating the IR. No query can observe
        // an earlier substitution from the same canonicalization batch.
        let mut substitutions=Vec::new();
        for &(_,value,_) in &self.native {
            let Some((allocation,end))=facts.roots.iter().find_map(|allocation|
                facts.restore_end(fd,allocation,value).map(|end|(allocation,end))) else {continue};
            if fd.obank().get(end.restore).unwrap().code()!=OpCode::CPUI_COPY
                || end.value!=allocation.before && fd.vbank().get(end.value).unwrap().descend_iter().next().is_some()
            {substitutions.push((end.restore,end.value,allocation.before));}
        }
        let canonicalized=!substitutions.is_empty();
        for (restore,value,before) in substitutions {
            if fd.obank().get(restore).unwrap().code()!=OpCode::CPUI_COPY {
                fd.op_set_opcode_code(restore,OpCode::CPUI_COPY);
                fd.op_set_all_input(restore,&[before]).unwrap();
            }
            if value!=before && fd.vbank().get(value).unwrap().descend_iter().next().is_some() {
                fd.total_replace(value,before).unwrap();
            }
        }
        // First settle native call-point reads; then propagate each newly
        // proved restore before recording frame writes or raising allocations.
        // An unchanged proof never requests another pass.
        if self.finalized_pass.is_none() || canonicalized {
            self.finalized_pass = Some(pass);
            self.base.count += 1;
            return 1;
        }
        self.finished = true;
        // Recomputed reservation addresses may use spilled operands. Only
        // use their proven equivalence after frame-word effects have settled.
        let live_roots: Vec<_> = facts.roots.iter().filter(|root|
            paired_origins(fd, root.base, root.amount, root.scale, &mut BTreeSet::new())
                .is_some_and(|origins| origins.len() > 1)).collect();
        let mut ends = RestorationCache::new();
        for id in fd.obank().iter_alive().filter(|_| !live_roots.is_empty()) {
            let Some(output) = fd.obank().get(id).filter(|op| matches!(op.code(),
                OpCode::CPUI_INT_ADD | OpCode::CPUI_INT_SUB | OpCode::CPUI_PTRADD | OpCode::CPUI_PTRSUB))
                .and_then(|op| op.get_out()) else { continue };
            if facts.roots.iter().any(|root| root.op == id)
                || aliases.iter().any(|&(value, _)| value == output) { continue; }
            let mut matching = facts.roots.iter().filter(|root|
                dominates_op(fd, root.op, id)
                    && facts.equations.delta(fd, output, root.output) == Some(0)
                    && reservation_active_at(fd, root, id, &mut ends));
            let Some(root) = matching.next() else { continue };
            if matching.next().is_none() && live_roots.iter().any(|live| live.op == root.op)
                && dominates_uses(fd, root.op, output)
            {
                aliases.push((output, root.output));
            }
        }
        match materialize_allocations(fd, &facts, &aliases, width, &self.stack_definitions, &self.native, &self.alignments) {
            Ok(shifts) => {
                self.base.count += facts.roots.len() as int4;
                bind_variadic_tails(fd, &shifts);
                fd.settle_stack_alloc();
            }
            Err(note) => { fd.note_stack_alloc(note.to_string()); decline(fd, ctx, Some(note)); }
        }
        self.release_native(fd);
        0
    }
}

/// Recovery changed this analysis and materializes nothing: the decompile
/// drive analyzes the function again without it, so this analysis stops here
/// (as for a spoiled store guard), keeping only `note`.
fn decline(fd: &mut Funcdata, ctx: &mut ActionContext, note: Option<&str>) {
    fd.spoil_stack_alloc(note);
    if !fd.is_jumptable_recovery_on() { ctx.abandon = true; }
}

fn capture_call_stack(fd: &mut Funcdata, call: OpId) -> Option<VarnodeId> {
    if let Some(&value) = fd.kuna_stack_call_sp.get(&call) { return value; }
    let value = (|| {
        let index = fd.get_call_specs_index(call)?;
        if let Some(value) = fd.get_call_specs(index).get_spacebase_relative(fd) { return Some(value); }
        let mut previous = fd.obank().get(call)?.basic_neighbours().0;
        while let Some(id) = previous {
            let op = fd.obank().get(id)?;
            if op.code() == OpCode::CPUI_INDIRECT && op.get_in(1).and_then(|input| fd.vbank().get(input))
                .is_some_and(|effect| crate::funcdata_varnode::op_iop_decode(effect.get_offset()) == call)
            {
                previous = op.basic_neighbours().0;
                continue;
            }
            if let Some(out) = op.get_out().filter(|&out| stack_register(fd, out)) { return Some(out); }
            if matches!(op.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND) {
                let call = fd.get_call_specs(fd.get_call_specs_index(id)?);
                if call.get_extra_pop() != 0 || call.proto().has_effect_override() { return None; }
                if let Some(&value) = fd.kuna_stack_call_sp.get(&id) { return value; }
                if let Some(value) = call.get_spacebase_relative(fd) { return Some(value); }
            }
            previous = op.basic_neighbours().0;
        }
        None
    })().or_else(|| {
        // Register-only calls need no argument-space placeholder. Read the
        // actual native SP here instead of guessing it from a predecessor:
        // heritage supplies the right incoming value even through a loop.
        let stack = fd.get_arch().manage().get_stack_space()?.clone();
        let source = fd.new_spacebase_ptr(&stack).ok()?;
        let size = fd.vbank().get(source)?.get_size();
        let mut before = call;
        while let Some(previous) = fd.obank().get(before)?.basic_neighbours().0 {
            let op = fd.obank().get(previous)?;
            if op.code() != OpCode::CPUI_INDIRECT || !op.get_in(1).and_then(|input| fd.vbank().get(input))
                .is_some_and(|effect| crate::funcdata_varnode::op_iop_decode(effect.get_offset()) == call)
            { break; }
            before = previous;
        }
        let read = fd.new_op(1, fd.obank().get(call)?.get_addr().clone());
        fd.op_set_opcode_code(read, OpCode::CPUI_COPY);
        fd.op_set_input(read, source, 0).unwrap();
        let value = fd.new_unique_out(size, read).unwrap();
        fd.op_insert_before(read, before);
        Some(value)
    });
    if value.is_some() { fd.kuna_stack_call_sp.insert(call, value); }
    value
}


fn native_stack_offset(fd: &Funcdata, value: VarnodeId, entry: VarnodeId, budget: &mut usize) -> Option<i128> {
    *budget = budget.checked_sub(1)?;
    if value == entry {
        return Some(0);
    }
    if fd.vbank().get(value)?.get_size() != fd.vbank().get(entry)?.get_size() {
        return None;
    }
    let op = fd.obank().get(fd.vbank().get(value)?.get_def()?)?;
    if let Some((base, delta)) = constant_step(fd, value) {
        return native_stack_offset(fd, base, entry, budget)?.checked_add(delta);
    }
    match op.code() {
        OpCode::CPUI_COPY | OpCode::CPUI_CAST => native_stack_offset(fd, op.get_in(0)?, entry, budget),
        OpCode::CPUI_INDIRECT => native_stack_offset(fd, op.get_in(0)?, entry, budget)?
            .checked_add(indirect_stack_pop(fd, fd.vbank().get(value)?.get_def()?)?),
        OpCode::CPUI_MULTIEQUAL => {
            let first = native_stack_offset(fd, op.get_in(0)?, entry, budget)?;
            for slot in 1..op.num_input() {
                if native_stack_offset(fd, op.get_in(slot)?, entry, budget)? != first {
                    return None;
                }
            }
            Some(first)
        }
        _ => None,
    }
}

fn allocation_origin_note(fd: &Funcdata, allocation: &Allocation, reserve: &Reservation) -> String {
    let entry = fd.get_arch().manage().get_stack_space().and_then(|space| fd.find_spacebase_input(space));
    let origin = entry.and_then(|entry| native_stack_offset(fd, allocation.base, entry, &mut 256));
    let origin = origin.map_or_else(|| "unresolved native SP".to_string(), |offset| format!("entry SP {offset:+}"));
    format!("Stack allocation at {:#x}: native SP = ({origin}) - (backing size - {:#x}); compiler backing shift {:#x}. Native frame offsets are not C object boundaries.",
        fd.obank().get(allocation.op).unwrap().get_addr().get_offset(), reserve.count, reserve.shift)
}

// PPC ELF linkage word zero is the ABI backchain, outside C stack objects.
// A conforming compiler recreates it when lowering the recovered alloca/VLA.
// https://refspecs.linuxfoundation.org/ELF/ppc64/PPC-elf64abi.html
// Recognize only a native backchain relocation around a separately saved SP.
// Existing conditional saved-word facts are reused, never seeded here.
fn ppc_backchain_restore_store(
    fd: &Funcdata, id: OpId, width: int4, budget: &mut usize, proof: &mut SavedFrameQuery,
) -> Option<(Allocation, OpId, OpId)> {
    *budget = budget.checked_sub(1)?;
    let store = fd.obank().get(id)?;
    if store.code() != OpCode::CPUI_STORE { return None; }
    let stack = fd.get_arch().manage().get_stack_space()?;
    let memory = stack.get_contain()?.get_index() as i128;
    if number(fd, store.get_in(0)?) != Some(memory) { return None; }
    let data = store.get_in(2)?;
    let node = fd.vbank().get(data)?;
    let target = store.get_in(1)?;
    if node.get_size() != width || node.is_volatile() || fd.vbank().get(target)?.is_volatile() { return None; }
    let load_id = node.get_def()?;
    let load = fd.obank().get(load_id)?;
    if load.code() != OpCode::CPUI_LOAD || load.get_parent() != store.get_parent()
        || number(fd, load.get_in(0)?) != Some(memory) { return None; }
    let pointer = load.get_in(1)?;
    if fd.vbank().get(pointer)?.is_volatile() { return None; }
    let origin = strip(fd, pointer);
    let root = allocation(fd, fd.vbank().get(origin)?.get_def()?)?;
    if native_stack_delta(fd, pointer, root.output) != Some(0) { return None; }
    let writer = raw_word_writer(fd, load_id, pointer, memory, None, WordQuery::Value, budget, proof)?;
    if !ppc_backchain_store(fd, &root, writer.op, width) { return None; }
    let payload = fd.obank().get(writer.op)?.get_in(2)?;
    // Moving a header containing the saved SP itself would create a self-link
    // in the restored frame. Relocation copies a parent value, not that SP.
    if native_stack_delta(fd, payload, root.before) == Some(0) { return None; }
    let entry = fd.find_spacebase_input(stack)?;
    if native_stack_delta(fd, payload, entry) != Some(0) {
        let parent = fd.vbank().get(strip(fd, payload))?;
        if parent.is_volatile() { return None; }
        let parent = fd.obank().get(parent.get_def()?)?;
        if parent.code() != OpCode::CPUI_LOAD || number(fd, parent.get_in(0)?) != Some(memory)
            || native_stack_delta(fd, parent.get_in(1)?, root.before) != Some(0)
        { return None; }
    }
    if node.descend_iter().any(|user| user != id && fd.obank().get(user).is_some_and(|op| !op.is_dead())) {
        return None;
    }
    let mut next = load.basic_neighbours().1;
    let mut restore = None;
    while let Some(current) = next {
        if current == id { break; }
        *budget = budget.checked_sub(1)?;
        let op = fd.obank().get(current)?;
        if op.get_parent() != store.get_parent() { return None; }
        next = op.basic_neighbours().1;
        if op.is_marker() { continue; }
        if let Some(output) = op.get_out().filter(|&value| stack_register(fd, value)) {
            if op.code() != OpCode::CPUI_COPY || native_stack_delta(fd, target, output) != Some(0) { return None; }
            if let Some((previous, previous_value)) = restore {
                if fd.obank().get(previous)?.get_addr() != op.get_addr()
                    || native_stack_delta(fd, output, previous_value) != Some(0) { return None; }
            }
            restore = Some((current, output));
        } else if matches!(op.code(), OpCode::CPUI_STORE | OpCode::CPUI_CALL | OpCode::CPUI_CALLIND
            | OpCode::CPUI_CALLOTHER | OpCode::CPUI_BRANCH | OpCode::CPUI_CBRANCH
            | OpCode::CPUI_BRANCHIND | OpCode::CPUI_RETURN)
            || op.get_out().and_then(|value| fd.vbank().get(value)).is_some_and(|out|
                out.is_addr_tied() && out.get_space().get_index() as i128 == memory)
        { return None; }
    }
    if next != Some(id) { return None; }
    let (restore, _) = restore?;
    if !dominates_op(fd, root.op, restore) { return None; }
    Some((root, writer.op, restore))
}

fn native_frame_link_payload(fd: &Funcdata, id: OpId, width: int4) -> bool {
    let Some(op) = fd.obank().get(id) else { return false };
    if op.code() != OpCode::CPUI_STORE { return false; }
    let Some(pointer) = op.get_in(1) else { return false };
    let pointer = strip(fd, pointer);
    let Some(root) = fd.vbank().get(pointer).and_then(|node| node.get_def())
        .and_then(|id| allocation(fd, id)) else { return false };
    ppc_backchain_store(fd, &root, id, width)
}

fn ppc_backchain_store(fd: &Funcdata, allocation: &Allocation, id: OpId, width: int4) -> bool {
    let arch = fd.get_arch();
    let Some(stack) = arch.manage().get_stack_space() else { return false };
    let Some(memory) = stack.get_contain() else { return false };
    let Some(registers) = arch.manage().register_lookup() else { return false };
    let Some(r1) = registers.probe_register("r1") else { return false };
    let Some(lr) = registers.probe_register("LR") else { return false };
    let Ok(sp) = stack.get_spacebase(0) else { return false };
    let same_storage = |a: &kuna_base::space::VarnodeStorage, b: &kuna_base::space::VarnodeStorage|
        a.offset == b.offset && a.size == b.size
            && a.space.as_ref().zip(b.space.as_ref()).is_some_and(|(a,b)| a.get_index() == b.get_index());
    if !same_storage(&r1, &sp) || r1.size != width as u32
        || registers.probe_register("XER").is_none() || registers.probe_register("CTR").is_none()
        || !stack.stack_grows_negative() || stack.get_word_size() != 1 || memory.get_word_size() != 1
    { return false; }
    if arch.default_return_addr.as_ref().is_some_and(|address|
        address.offset != lr.offset || address.size != lr.size
            || address.space.as_ref().zip(lr.space.as_ref()).is_none_or(|(a,b)| a.get_index() != b.get_index()))
    { return false; }
    let proto = fd.get_func_proto();
    if !proto.has_model() || proto.is_model_unknown() || proto.has_effect_override()
        || !proto.is_stack_grows_negative() || proto.get_extra_pop() != 0
        || proto.get_spacebase().is_none_or(|space| space.get_index() != stack.get_index())
    { return false; }
    let Some(op) = fd.obank().get(id) else { return false };
    let Some(pointer) = op.get_in(1) else { return false };
    let Some(data) = op.get_in(2) else { return false };
    if op.code() != OpCode::CPUI_STORE
        || op.get_in(0).and_then(|value| number(fd, value)) != Some(memory.get_index() as i128)
        || fd.vbank().get(data).is_none_or(|value| value.get_size() != width || value.is_volatile())
        || fd.vbank().get(pointer).is_none_or(|value| value.is_volatile())
        || native_stack_delta(fd, pointer, allocation.output) != Some(0)
    { return false; }
    let origin = strip(fd, data);
    if fd.vbank().get(origin).is_some_and(|value| value.is_free() && !value.is_input()) { return false; }
    let entry = fd.find_spacebase_input(stack);
    let mut payload = entry.is_some_and(|entry| native_stack_delta(fd, data, entry) == Some(0))
        || native_stack_delta(fd, data, allocation.before) == Some(0);
    if !payload {
        if let Some(load) = fd.vbank().get(origin).and_then(|value| value.get_def()).and_then(|id| fd.obank().get(id)) {
            payload = load.code() == OpCode::CPUI_LOAD
                && load.get_in(0).and_then(|value| number(fd, value)) == Some(memory.get_index() as i128)
                && load.get_in(1).is_some_and(|pointer| native_stack_delta(fd, pointer, allocation.before) == Some(0));
        }
    }
    if !payload { return false; }
    let mut next = op.basic_neighbours().1;
    while let Some(id) = next {
        let Some(commit) = fd.obank().get(id) else { return false };
        if commit.get_addr() != op.get_addr() || commit.get_parent() != op.get_parent() { return false; }
        if commit.code() == OpCode::CPUI_COPY && commit.get_out().is_some_and(|value| stack_register(fd, value))
            && commit.get_in(0).is_some_and(|value| native_stack_delta(fd, value, allocation.output) == Some(0))
        { return true; }
        next = commit.basic_neighbours().1;
    }
    false
}



/// A pointer a call returned: its provenance is whatever the callee legally
/// held, which after the allocation's scope is at most an object the source
/// passed it, never the native frame-link word the source cannot name.
fn returned_pointer(fd: &Funcdata, value: VarnodeId) -> bool {
    fd.vbank().get(strip(fd, value)).and_then(|node| node.get_def()).and_then(|id| fd.obank().get(id))
        .is_some_and(|op| matches!(op.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND))
}

/// The call whose INDIRECT defines `value`, if any.
fn carried_from(fd: &Funcdata, value: VarnodeId) -> Option<OpId> {
    let id = fd.vbank().get(strip(fd, value))?.get_def()?;
    (fd.obank().get(id)?.code() == OpCode::CPUI_INDIRECT).then(|| frame_word_effect(fd, id)).flatten()
}

/// The frame offset of the link word a COPY writes directly, when `id` is a
/// COPY into an address-sized stack word.
fn direct_frame_word(fd: &Funcdata, id: OpId, width: int4) -> Option<i128> {
    let op = fd.obank().get(id).filter(|op| op.code() == OpCode::CPUI_COPY)?;
    let node = fd.vbank().get(op.get_out()?)?;
    let stack = fd.get_arch().manage().get_stack_space()?;
    (node.get_space().get_index() == stack.get_index() && node.get_size() == width && !node.is_volatile())
        .then(|| sign_extend(node.get_offset() as i64, width * 8 - 1) as i128)
}

/// Is `value`, through copies and call INDIRECTs, a definition that
/// dominates `op` (or a function input)?
fn predates(fd: &Funcdata, mut value: VarnodeId, op: OpId) -> bool {
    for _ in 0..256 {
        let Some(def) = fd.vbank().get(value).and_then(|node| node.get_def()) else { return true };
        let Some(defop) = fd.obank().get(def) else { return false };
        if matches!(defop.code(), OpCode::CPUI_COPY | OpCode::CPUI_INDIRECT) && !dominates_op(fd, def, op) {
            let Some(next) = defop.get_in(0) else { return false };
            value = next;
            continue;
        }
        return dominates_op(fd, def, op);
    }
    false
}

/// Is every SSA value `value` can hold, through copies, call INDIRECTs and
/// merges, either a relocation's rewritten link word or one defined before
/// `op`? Each is the frame link the source frame holds.
fn link_word_value(fd: &Funcdata, value: VarnodeId, op: OpId, relocated: &[VarnodeId], seen: &mut BTreeSet<VarnodeId>) -> bool {
    if relocated.contains(&value) || !seen.insert(value) { return true; }
    if seen.len() > 256 { return false; }
    let Some(def) = fd.vbank().get(value).and_then(|node| node.get_def()) else { return true };
    let Some(defop) = fd.obank().get(def) else { return false };
    match defop.code() {
        OpCode::CPUI_COPY | OpCode::CPUI_INDIRECT if !dominates_op(fd, def, op) =>
            defop.get_in(0).is_some_and(|input| link_word_value(fd, input, op, relocated, seen)),
        OpCode::CPUI_MULTIEQUAL if !dominates_op(fd, def, op) =>
            (0..defop.num_input()).all(|slot| defop.get_in(slot).is_some_and(|input| link_word_value(fd, input, op, relocated, seen))),
        _ => dominates_op(fd, def, op),
    }
}

fn native_frame_links(fd: &Funcdata, facts: &FrameFacts, width: int4, native: &[(OpId,VarnodeId,bool)]) -> Result<Vec<(OpId, usize)>, &'static str> {
    let allocations=&facts.roots;
    let refused = "Native frame-link data has no source allocation representation; allocation recovery declined.";
    let Some(stack) = fd.get_arch().manage().get_stack_space() else { return Ok(Vec::new()) };
    let Some(memory) = stack.get_contain() else { return Ok(Vec::new()) };
    let entry = fd.find_spacebase_input(stack);
    let mut links = Vec::new();
    for id in fd.obank().iter_alive() {
        let op = fd.obank().get(id).unwrap();
        if op.code() != OpCode::CPUI_STORE || op.is_dead()
            || op.get_in(0).and_then(|value| number(fd, value)) != Some(memory.get_index() as i128) { continue; }
        let Some(data) = op.get_in(2).filter(|&value| fd.vbank().get(value).is_some_and(|value| value.get_size() == width && !value.is_constant())) else { continue };
        // Sibling scopes reuse the native address: the header belongs to the
        // nearest dominating reservation that writes it.
        // Only a reservation live at the store owns it; the dominating ones form
        // a chain, whose deepest member is the nearest.
        let owners:Vec<usize>=(0..allocations.len()).filter(|&index|facts.link_store(fd,&allocations[index],id,width)
            && dominates_op(fd,allocations[index].op,id)).collect();
        let nearest=owners.iter().copied().find(|&index|owners.iter().all(|&other|other==index
            || !dominates_op(fd,allocations[index].op,allocations[other].op)
            || !dominates_op(fd,allocations[other].op,id)));
        if let Some(index) = nearest {
            links.push((id,index));
            continue;
        }
        let origin = strip(fd, data);
        let native = fd.vbank().get(origin).is_some_and(|value| (value.is_input() || value.is_free()) && stack_register(fd, origin))
            || entry.is_some_and(|entry| native_stack_delta(fd, data, entry) == Some(0));
        if native && op.get_in(1).is_some_and(|pointer| allocations.iter().any(|allocation|
            native_stack_delta(fd, pointer, allocation.output).is_some())) { return Err(refused); }
    }
    let mut native_restores = Vec::new();
    if !links.is_empty() {
        for &(store,index,header,restore,_) in &facts.transports {
            if !links.contains(&(header,index)) {continue;}
            let Some(value)=fd.obank().get(restore).and_then(|op|op.get_out()) else {continue};
            if facts.restore_end(fd,&allocations[index],value).is_some_and(|end|end.restore==restore) {
                native_restores.push((store,index));
            }
        }
    }
    let metadata:Vec<_>=links.iter().chain(&native_restores).copied().collect();
    for &(store,index) in &links {
        let allocation = &allocations[index];
        let parent = fd.obank().get(store).unwrap().get_parent().ok_or(refused)?;
        let reachable = reachable_blocks(fd, parent, &mut 65536).ok_or(refused)?;
        // A predecessor in this set can revisit the store's block. Its earlier
        // operations may observe the previous iteration's backchain word.
        let block = fd.bblocks_ref().block(parent);
        let revisited = (0..block.size_in()).any(|slot| reachable.contains(&block.get_in(slot)));
        let released: Vec<OpId> = fd.obank().iter_alive().filter(|&id| fd.obank().get(id)
            .and_then(|op| op.get_out()).is_some_and(|out| facts.restore_end(fd, allocation, out)
                .is_some_and(|end| end.restore == id))).collect();
        let mut budget = 65536;
        for id in fd.obank().iter_alive() {
            let op = fd.obank().get(id).unwrap();
            if op.is_dead() || op.is_marker() || id == store
                || metadata.iter().any(|&(store,_)|store==id)
                || !op.get_parent().is_some_and(|block| reachable.contains(&block))
                || (op.get_parent() == Some(parent) && !revisited && !dominates_op(fd, allocation.op, id)) { continue; }
            if matches!(op.code(), OpCode::CPUI_LOAD | OpCode::CPUI_STORE)
                && op.get_in(0).and_then(|value| number(fd, value)) == Some(memory.get_index() as i128)
            {
                let pointer = op.get_in(1).ok_or(refused)?;
                let value = if op.code() == OpCode::CPUI_STORE { op.get_in(2) } else { op.get_out() }.ok_or(refused)?;
                let bytes = fd.vbank().get(value).ok_or(refused)?.get_size();
                if facts.header_reaches(fd,pointer,bytes as i128,allocation,width,Some((id,1)),&mut budget) {
                    if op.code()!=OpCode::CPUI_LOAD || bytes!=width {return Err(refused);}
                    // The decoder may load into a temporary before copying
                    // into SP. Follow the same native-only consumer closure
                    // in either representation; never give SP an ordinary
                    // unused-value fallback after its native proof fails.
                    if !facts.header_sp_sink(fd,id,native,&mut budget)
                        && (stack_register(fd,value)
                            || !facts.header_address(fd,pointer,FrameRead::Operation(id),&links,width,&mut budget)
                            || !checkpoint_value_unused(fd,value,None,width,&[],&metadata,&[])) {return Err(refused);}
                }
            }
            let inputs = match op.code() {
                OpCode::CPUI_STORE if !links.iter().any(|&(link,_)| link == id) => 2..3,
                OpCode::CPUI_CALL => 1..op.num_input(),
                // A RETURN pops this frame, the header with it: a returned value
                // cannot reach the word without reading a dead frame.
                OpCode::CPUI_CALLIND | OpCode::CPUI_CALLOTHER => 0..op.num_input(),
                _ if op.get_out().and_then(|value| fd.vbank().get(value)).is_some_and(|out|
                    out.is_addr_tied() && out.get_space().get_index() == memory.get_index()) => 0..op.num_input(),
                _ => 0..0,
            };
            let expired = op.code() == OpCode::CPUI_CALL
                && released.iter().any(|&restore| dominates_op(fd, restore, id));
            for slot in inputs {
                let input = op.get_in(slot).ok_or(refused)?;
                if expired && returned_pointer(fd, input) { continue; }
                // A frame word a call's INDIRECT carries past that call holds
                // what was stored there when the call received only returned
                // pointers; check the stored value.
                let quiet = expired && carried_from(fd, input).and_then(|call| fd.obank().get(call))
                    .is_some_and(|call| call.code() == OpCode::CPUI_CALL && (1..call.num_input()).all(|slot|
                        call.get_in(slot).is_some_and(|value| returned_pointer(fd, value)
                            || !facts.header_reaches(fd, value, 1, allocation, width, None, &mut budget))));
                let input = if quiet { carried(fd, input) } else { input };
                if facts.header_reaches(fd,input,1,allocation,width,Some((id,slot)),&mut budget) { return Err(refused); }
            }
            if budget == 0 { return Err(refused); }
        }
    }
    let relocated:Vec<VarnodeId>=native_restores.iter().filter_map(|&(store,_)|
        direct_frame_word(fd,store,width).and(fd.obank().get(store)?.get_out())).collect();
    for &(store,index) in &native_restores {
        let op=fd.obank().get(store).ok_or(refused)?;
        let target=if direct_frame_word(fd,store,width).is_some() {
            facts.transports.iter().find(|&&(id,..)|id==store).and_then(|&(_,_,_,restore,_)|fd.obank().get(restore)?.get_out())
        } else {op.get_in(1)}.ok_or(refused)?;
        let parent=op.get_parent().ok_or(refused)?;
        let reachable=reachable_blocks(fd,parent,&mut 65536).ok_or(refused)?;
        let block=fd.bblocks_ref().block(parent);
        let revisited=(0..block.size_in()).any(|slot|reachable.contains(&block.get_in(slot)));
        let first=facts.equations.delta(fd,target,facts.entry);
        let mut budget=65536;
        for id in fd.obank().iter_alive() {
            let op=fd.obank().get(id).ok_or(refused)?;
            if op.is_dead() || op.is_marker() || metadata.iter().any(|&(store,_)|store==id)
                || !op.get_parent().is_some_and(|block|reachable.contains(&block))
                || op.get_parent()==Some(parent) && !revisited && !dominates_op(fd,store,id) {continue;}
            if op.code()==OpCode::CPUI_CALLOTHER {return Err(refused);}
            if matches!(op.code(),OpCode::CPUI_LOAD|OpCode::CPUI_STORE)
                && op.get_in(0).and_then(|value|number(fd,value))==Some(memory.get_index() as i128)
            {
                let pointer=op.get_in(1).ok_or(refused)?;
                let value=if op.code()==OpCode::CPUI_STORE {op.get_in(2)} else {op.get_out()}.ok_or(refused)?;
                let bytes=fd.vbank().get(value).ok_or(refused)?.get_size();
                if facts.header_reaches_at(fd,pointer,bytes as i128,target,target,width,Some((id,1)),&mut budget)
                    && (op.code()!=OpCode::CPUI_LOAD || bytes!=width
                        || !facts.header_sp_sink(fd,id,native,&mut budget)
                            && (stack_register(fd,value)
                                || !facts.header_live(fd,store,FrameRead::Operation(id),target,&mut budget)
                                || !checkpoint_value_unused(fd,value,None,width,&[],&metadata,&[]))) {return Err(refused);}
            }
            for slot in 0..op.num_input() {
                // A direct relocation's own word keeps its value: the commit
                // rewrites that COPY to the payload the header held.
                // Any relocation's own word: each commit rewrites its COPY to the
                // payload its header held, the same frame link.
                if op.get_in(slot).is_some_and(|value|link_word_value(fd,value,allocations[index].op,&relocated,&mut BTreeSet::new())) {continue;}
                // A read whose SSA value predates the allocation sees the frame
                // link the source frame holds as well: the relocation rewrites
                // the word with the payload the header took from it.
                if op.get_in(slot).is_some_and(|value|
                    predates(fd,value,allocations[index].op)) {continue;}
                if let Some(node)=op.get_in(slot).and_then(|value|fd.vbank().get(value))
                    .filter(|node|node.get_space().get_index()==stack.get_index()) {
                    let at=sign_extend(node.get_offset() as i64,width*8-1) as i128;
                    if first.is_none_or(|first|at<first+width as i128 && at+node.get_size() as i128>first) {return Err(refused);}
                }
            }
            if let Some(node)=op.get_out().and_then(|value|fd.vbank().get(value))
                .filter(|node|node.get_space().get_index()==stack.get_index()) {
                let at=sign_extend(node.get_offset() as i64,width*8-1) as i128;
                if first.is_none_or(|first|at<first+width as i128 && at+node.get_size() as i128>first) {return Err(refused);}
            }
            let inputs=match op.code() {
                OpCode::CPUI_STORE=>2..3,
                OpCode::CPUI_CALL=>1..op.num_input(),
                OpCode::CPUI_CALLIND=>0..op.num_input(),
                _ if op.get_out().and_then(|value|fd.vbank().get(value)).is_some_and(|node|
                    node.is_addr_tied() && node.get_space().get_index()==memory.get_index())=>0..op.num_input(),
                _=>0..0,
            };
            for slot in inputs {
                let input=op.get_in(slot).ok_or(refused)?;
                // The relocation rewrote the frame link with the word it held;
                // a pointer a later call returned names no source object there.
                let after=op.code()==OpCode::CPUI_CALL && !revisited && dominates_op(fd,store,id);
                if after && returned_pointer(fd,input) {continue;}
                let quiet=after && carried_from(fd,input).and_then(|call|fd.obank().get(call))
                    .is_some_and(|call|call.code()==OpCode::CPUI_CALL && (1..call.num_input()).all(|slot|
                        call.get_in(slot).is_some_and(|value|returned_pointer(fd,value)
                            || !facts.header_reaches_at(fd,value,1,target,target,width,None,&mut budget))));
                let input=if quiet {carried(fd,input)} else {input};
                if facts.header_reaches_at(fd,input,1,target,target,width,
                    Some((id,slot)),&mut budget) {return Err(refused);}
            }
        }
    }
    // Keep relocations out of the certified allocation-header set used above.
    // Both kinds of bookkeeping stores are removed only at successful commit.
    links.extend(native_restores);
    Ok(links)
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::ArchContext;
    use kuna_base::address::Address;
    use kuna_base::space::{addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, FspecSpace, IopSpace, UniqueSpace};
    use std::rc::Rc;
fn build_manager() -> AddrSpaceManager {
        let mut m = AddrSpaceManager::new();
        m.insert_space(Rc::new(ConstantSpace::new())).unwrap();
        m.insert_space(Rc::new(UniqueSpace::new(1, 0, false)))
            .unwrap();
        m.insert_space(Rc::new(IopSpace::new(2))).unwrap();
        m.insert_space(Rc::new(FspecSpace::new(3))).unwrap();
        m.insert_space(Rc::new(AddrSpace::new(
            spacetype::IPTR_PROCESSOR,
            "ram",
            false,
            8,
            1,
            4,
            addrspace_flags::hasphysical,
            1,
            1,
        )))
        .unwrap();
        m.set_default_code_space(4).unwrap();
        m
    }
struct FrameRegisters {
    space: Rc<AddrSpace>, width: int4, ppc: bool,
}
impl kuna_base::space::RegisterLookup for FrameRegisters {
    fn get_register(&self, name: &str) -> Result<kuna_base::space::VarnodeStorage, kuna_base::error::KunaError> {
        let index = match name {
            "r1" => 0, "LR" => 4, "XER" if self.ppc => 5, "CTR" if self.ppc => 6,
            _ => return Err(kuna_base::error::KunaError::lowlevel("unknown register")),
        };
        Ok(kuna_base::space::VarnodeStorage { space: Some(Rc::clone(&self.space)), offset: index * self.width as u64, size: self.width as u32 })
    }
    fn get_register_name(&self, _: &Rc<AddrSpace>, _: u64, _: i32) -> String { String::new() }
    fn get_exact_register_name(&self, _: &Rc<AddrSpace>, _: u64, _: i32) -> String { String::new() }
}
fn scoped_lifetime_fixture(width: int4, stack_return: bool) -> (Funcdata, Allocation, LifetimeEnd, BlockId) {
    scoped_lifetime_fixture_abi(width, stack_return, 0)
}
fn scoped_lifetime_fixture_abi(width: int4, stack_return: bool, abi: u8) -> (Funcdata, Allocation, LifetimeEnd, BlockId) {
        use kuna_base::space::{SpacebaseSpace, VarnodeStorage};
        let mut manager = build_manager();
        manager.set_default_data_space(4).unwrap();
        let ram = Rc::clone(manager.get_space_by_name("ram").unwrap());
        let register = Rc::new(AddrSpace::new(
            spacetype::IPTR_PROCESSOR, "register", false, width as u32, 1, 5,
            addrspace_flags::hasphysical, 0, 0,
        ));
        manager.insert_space(Rc::clone(&register)).unwrap();
        if abi != 0 {
            manager.set_register_lookup(Rc::new(FrameRegisters { space: Rc::clone(&register), width, ppc: abi == 1 }));
        }
        let stack = Rc::new(SpacebaseSpace::new("stack", 6, width as u32, &ram, 1, true, false));
        manager.add_spacebase_pointer(&stack, &VarnodeStorage {
            space: Some(Rc::clone(&register)), offset: 0, size: width as u32,
        }, width, true).unwrap();
        manager.insert_space(stack).unwrap();
        let mut context = ArchContext::new(manager);
        context.stack_alloc_scopes = true;
        let types = Rc::new(crate::dtype::TypeFactoryImpl::new());
        types.set_max_basetype_size(8);
        types.setup_sizes(Some(width), width, 4);
        context.types = Some(types);
        if stack_return {
            context.default_return_addr = Some(kuna_num::pcoderaw::VarnodeData {
                space: context.manage().get_stack_space().cloned(), offset: 0, size: width as u32,
            });
        }
        if abi != 0 {
            context.default_return_addr = Some(kuna_num::pcoderaw::VarnodeData {
                space: Some(Rc::clone(&register)), offset: 4 * width as u64, size: width as u32,
            });
        }
        let context = Rc::new(context);
        let mut fd = Funcdata::new("scope", "scope", context,
            Address::new(Rc::clone(&ram), 0x1000), 0x10000000, 0x40).unwrap();
        if abi != 0 {
            let mut model = crate::fspec::ProtoModel::new(fd.get_arch().manage());
            model.build_param_list("standard").unwrap();
            let stack = Rc::clone(fd.get_arch().manage().get_stack_space().unwrap());
            let entry = crate::fspec::ParamEntry::seed(0, crate::dtype::type_class::TYPECLASS_GENERAL,
                stack, if width == 4 { 8 } else { 112 }, 4096, 1, width, 0, true, false,
                &[], fd.get_arch().manage()).unwrap();
            model.input_mut().push_entry(entry);
            model.input_mut().finish_decode();
            fd.get_func_proto_mut().set_model(Some(Rc::new(model)));
            fd.get_func_proto_mut().set_extra_pop(0);
        }
        let graph = fd.bblocks_root_pub();
        let blocks: Vec<_> = (0..4).map(|_| fd.bblocks_mut().new_block_basic(graph)).collect();
        for (index, &block) in blocks.iter().enumerate() {
            fd.bblocks_mut().block_mut(block).set_index(index as int4);
        }
        for (from, to) in [(0, 1), (1, 1), (1, 2), (2, 3)] {
            fd.bblocks_mut().add_edge(blocks[from], blocks[to]);
        }
        fd.bblocks_mut().calc_forward_dominator(graph, &[blocks[0]]);
        let sp = Address::new(register, 0);
        let input = fd.new_varnode(width, &sp, None);
        let input = fd.set_input_varnode(input).unwrap();
        let frame = fd.new_op(2, Address::new(Rc::clone(&ram), 0x1004));
        fd.op_set_opcode_code(frame, OpCode::CPUI_INT_ADD);
        fd.op_set_input(frame, input, 0).unwrap();
        let offset = fd.new_constant(width, (-64_i64) as u64 & calc_mask(width));
        fd.op_set_input(frame, offset, 1).unwrap();
        let base = fd.new_varnode_out(width, &sp, frame).unwrap();
        fd.op_insert_end(frame, blocks[0]);
        let amount = fd.new_varnode(width, &Address::new(Rc::clone(&ram), 0x2000), None);
        let amount = fd.set_input_varnode(amount).unwrap();
        let op = fd.new_op(2, Address::new(Rc::clone(&ram), 0x1010));
        fd.op_set_opcode_code(op, OpCode::CPUI_INT_SUB);
        fd.op_set_input(op, base, 0).unwrap();
        fd.op_set_input(op, amount, 1).unwrap();
        let output = fd.new_varnode_out(width, &sp, op).unwrap();
        fd.op_insert_end(op, blocks[0]);
        let restore = fd.new_op(1, Address::new(ram, 0x1030));
        fd.op_set_opcode_code(restore, OpCode::CPUI_COPY);
        fd.op_set_input(restore, base, 0).unwrap();
        let value = fd.new_varnode_out(width, &sp, restore).unwrap();
        fd.op_insert_end(restore, blocks[2]);
        let allocation = Allocation { scale: 1, before: base, op, output, amount, base };
        let end = LifetimeEnd {
            restore, join: blocks[2], boundary: Some(blocks[3]), address: 0x1030, value,
        };
        fd.kuna_stack_scope_holds.extend([output, value]);
        (fd, allocation, end, blocks[1])
    }
fn checkpoint_call_fixture(width: int4, variadic: bool, stack_return: bool) -> (Funcdata, Allocation, LifetimeEnd, BlockId, int4, VarnodeId) {
        use crate::dtype::{type_class, type_metatype, Datatype};
        use crate::fspec::{FuncCallSpecs, ParamEntry, ProtoModel};
        let (mut fd, allocation, end, block) = scoped_lifetime_fixture(width, stack_return);
        let stack = Rc::clone(fd.get_arch().manage().get_stack_space().unwrap());
        let register = Rc::clone(fd.get_arch().manage().get_space_by_name("register").unwrap());
        let slot = Address::new(Rc::clone(&stack), (-32_i64) as u64 & calc_mask(width));
        let save = fd.new_op(1, fd.get_address().clone());
        fd.op_set_opcode_code(save, OpCode::CPUI_COPY);
        fd.op_set_input(save, allocation.base, 0).unwrap();
        let saved = fd.new_varnode_out(width, &slot, save).unwrap();
        fd.op_insert_before(save, allocation.op);
        let call = fd.new_op(2, fd.get_address().clone());
        fd.op_set_opcode_code(call, OpCode::CPUI_CALL);
        let target = fd.new_constant(width, 0x3000);
        fd.op_set_input(call, target, 0).unwrap();
        fd.op_set_input(call, allocation.output, 1).unwrap();
        fd.op_insert_end(call, block);
        let mut model = ProtoModel::new(fd.get_arch().manage());
        model.build_param_list("standard").unwrap();
        let entry = ParamEntry::seed(0, type_class::TYPECLASS_GENERAL,
            stack, 0, 4096, 1, width, 0, true, false, &[], fd.get_arch().manage()).unwrap();
        model.input_mut().push_entry(entry);
        model.input_mut().finish_decode();
        let mut spec = FuncCallSpecs::new(call, Address::new_invalid());
        spec.proto_mut().attach_internal_store(Rc::new(Datatype::new(0, type_metatype::TYPE_VOID)));
        spec.proto_mut().set_model(Some(Rc::new(model)));
        spec.proto_mut().set_extra_pop(0);
        spec.proto_mut().set_dotdotdot(variadic);
        spec.set_final_input_storage(vec![(Address::new(register, 2 * width as u64), width)]);
        let index = fd.push_call_specs(spec);
        let indirect = fd.new_op(2, fd.get_address().clone());
        fd.op_set_opcode_code(indirect, OpCode::CPUI_INDIRECT);
        fd.op_set_input(indirect, saved, 0).unwrap();
        let effect = fd.new_varnode_iop(call);
        fd.op_set_input(indirect, effect, 1).unwrap();
        let restored = fd.new_varnode_out(width, &slot, indirect).unwrap();
        fd.op_insert_end(indirect, block);
        fd.op_set_input(end.restore, restored, 0).unwrap();
        fd.kuna_stack_call_sp.insert(call, Some(allocation.output));
        (fd, allocation, end, block, index, saved)
    }

    #[test]
    fn dynamic_saved_words_require_private_call_storage_and_no_earlier_publication() {
        for width in [4, 8] {
            for control in ["preserved", "local-only", "argument", "below-slot", "published-before", "inside-arguments", "unknown-call"] {
                let (mut fd, allocation, end, _, index, saved) = checkpoint_call_fixture(width, false, false);
                let call = fd.get_call_specs(index).get_op();
                let old = fd.obank().get(end.restore).unwrap().get_in(0).unwrap();
                let marker = fd.vbank().get(old).unwrap().get_def().unwrap();
                let save = fd.vbank().get(saved).unwrap().get_def().unwrap();
                let point = fd.get_address().clone();
                let memory = fd.get_arch().manage().get_stack_space().unwrap().get_contain().unwrap().get_index();
                let space = fd.new_constant(width, memory as u64);
                let pointer = allocation.output;
                let birth = fd.new_op(3, point.clone());
                fd.op_set_opcode_code(birth, OpCode::CPUI_STORE);
                fd.op_set_all_input(birth, &[space, pointer, allocation.base]).unwrap();
                fd.op_insert_before(birth, call);
                let read = fd.new_op(2, point.clone());
                fd.op_set_opcode_code(read, OpCode::CPUI_LOAD);
                fd.op_set_all_input(read, &[space, pointer]).unwrap();
                let value = fd.new_unique_out(width, read).unwrap();
                fd.op_insert_before(read, if control == "local-only" { call } else { end.restore });
                fd.op_set_input(end.restore, value, 0).unwrap();
                fd.op_destroy(marker);
                fd.op_destroy(save);
                let number = fd.new_constant(width, 7);
                fd.op_set_input(call, number, 1).unwrap();
                if control == "local-only" {
                    let add = fd.new_op(2, point.clone());
                    fd.op_set_opcode_code(add, OpCode::CPUI_INT_ADD);
                    let offset = fd.new_constant(width, width as u64);
                    fd.op_set_all_input(add, &[pointer, offset]).unwrap();
                    let adjacent = fd.new_unique_out(width, add).unwrap();
                    fd.op_insert_before(add, birth);
                    let earlier = fd.new_op(3, point.clone());
                    fd.op_set_opcode_code(earlier, OpCode::CPUI_STORE);
                    fd.op_set_all_input(earlier, &[space, adjacent, allocation.base]).unwrap();
                    fd.op_insert_before(earlier, birth);
                }
                if control == "argument" { fd.op_set_input(call, pointer, 1).unwrap(); }
                if control == "below-slot" {
                    let add = fd.new_op(2, point.clone());
                    fd.op_set_opcode_code(add, OpCode::CPUI_INT_ADD);
                    let offset = fd.new_constant(width, calc_mask(width));
                    fd.op_set_all_input(add, &[pointer, offset]).unwrap();
                    let below = fd.new_unique_out(width, add).unwrap();
                    fd.op_insert_before(add, call);
                    fd.op_set_input(call, below, 1).unwrap();
                }
                if control == "published-before" {
                    let publish = fd.new_op(3, point);
                    fd.op_set_opcode_code(publish, OpCode::CPUI_STORE);
                    let global = fd.new_constant(width, 0x9000);
                    fd.op_set_all_input(publish, &[space, global, pointer]).unwrap();
                    fd.op_insert_before(publish, birth);
                }
                if control == "inside-arguments" {
                    let stack = Rc::clone(fd.get_arch().manage().get_stack_space().unwrap());
                    fd.get_call_specs_mut(index).set_final_input_storage(vec![(Address::new(stack, 0), width)]);
                }
                if control == "unknown-call" {
                    fd.get_call_specs_mut(index).proto_mut().set_extra_pop(crate::fspec::EXTRAPOP_UNKNOWN);
                }
                let restored = saved_stack_value(&fd, value, &mut 65536, &mut SavedFrameQuery::default())
                    .and_then(|saved| saved.value).and_then(|value| native_stack_delta(&fd, value, allocation.base));
                assert_eq!(restored == Some(0), matches!(control, "preserved" | "local-only"), "width={width}, control={control}");
            }
        }
    }

    #[test]
    fn loop_checkpoint_requires_a_consistent_initialized_seed() {
        for width in [4, 8] {
            for change in ["preserved", "different_backedge", "unknown_entry"] {
                let (mut fd, allocation, end, block, _, saved) = checkpoint_call_fixture(width, false, false);
                let stack = Rc::clone(fd.get_arch().manage().get_stack_space().unwrap());
                let address = fd.vbank().get(saved).unwrap().get_addr().clone();
                let carried = fd.obank().get(end.restore).unwrap().get_in(0).unwrap();
                let marker = fd.vbank().get(carried).unwrap().get_def().unwrap();
                let phi = fd.new_op(2, fd.get_address().clone());
                fd.op_set_opcode_code(phi, OpCode::CPUI_MULTIEQUAL);
                fd.op_set_all_input(phi, &[saved, carried]).unwrap();
                let merged = fd.new_varnode_out(width, &address, phi).unwrap();
                fd.op_insert_begin(phi, block);
                fd.op_set_input(marker, merged, 0).unwrap();
                if change == "different_backedge" {
                    let add = fd.new_op(2, fd.get_address().clone());
                    fd.op_set_opcode_code(add, OpCode::CPUI_INT_ADD);
                    let delta = fd.new_constant(width, width as u64);
                    fd.op_set_all_input(add, &[allocation.base, delta]).unwrap();
                    let other = fd.new_unique_out(width, add).unwrap();
                    fd.op_insert_end(add, block);
                    let write = fd.new_op(1, fd.get_address().clone());
                    fd.op_set_opcode_code(write, OpCode::CPUI_COPY);
                    fd.op_set_input(write, other, 0).unwrap();
                    let changed = fd.new_varnode_out(width, &address, write).unwrap();
                    fd.op_insert_end(write, block);
                    fd.op_set_input(phi, changed, 1).unwrap();
                } else if change == "unknown_entry" {
                    let input = fd.new_varnode(width, &Address::new(stack, address.get_offset()), None);
                    let input = fd.set_input_varnode(input).unwrap();
                    fd.op_set_input(phi, input, 0).unwrap();
                }
                let resolved = saved_stack_value(&fd, end.value, &mut 65536, &mut SavedFrameQuery::default())
                    .and_then(|saved| saved.value)
                    .and_then(|value| native_stack_delta(&fd, value, allocation.base));
                assert_eq!(resolved == Some(0), change == "preserved", "width={width}, change={change}");
            }
        }
    }

    #[test]
    fn dominated_raw_restore_checks_partial_writes_on_every_path() {
        for width in [4, 8] {
            for corrupt in [false, true] {
                let (mut fd, allocation, end, block, _, _) = checkpoint_call_fixture(width, false, false);
                let stack = Rc::clone(fd.get_arch().manage().get_stack_space().unwrap());
                let input = fd.find_spacebase_input(&stack).unwrap();
                let point = fd.get_address().clone();
                let graph = fd.bblocks_root_pub();
                let alternate = fd.bblocks_mut().new_block_basic(graph);
                fd.bblocks_mut().block_mut(alternate).set_index(4);
                fd.bblocks_mut().add_edge(block, alternate);
                fd.bblocks_mut().add_edge(alternate, end.join);
                if corrupt {
                    let write = fd.new_op(1, point.clone());
                    fd.op_set_opcode_code(write, OpCode::CPUI_COPY);
                    let byte = fd.new_constant(1, 7);
                    fd.op_set_input(write, byte, 0).unwrap();
                    let address = Address::new(Rc::clone(&stack), (-31_i64) as u64 & calc_mask(width));
                    fd.new_varnode_out(1, &address, write).unwrap();
                    fd.op_insert_end(write, alternate);
                }
                let entry = fd.obank().get(allocation.op).unwrap().get_parent().unwrap();
                fd.bblocks_mut().calc_forward_dominator(graph, &[entry]);
                let add = fd.new_op(2, point.clone());
                fd.op_set_opcode_code(add, OpCode::CPUI_INT_ADD);
                let offset = fd.new_constant(width, (-32_i64) as u64 & calc_mask(width));
                fd.op_set_all_input(add, &[input, offset]).unwrap();
                let pointer = fd.new_unique_out(width, add).unwrap();
                fd.op_insert_before(add, end.restore);
                let load = fd.new_op(2, point);
                fd.op_set_opcode_code(load, OpCode::CPUI_LOAD);
                let memory = fd.new_constant(4, stack.get_contain().unwrap().get_index() as u64);
                fd.op_set_all_input(load, &[memory, pointer]).unwrap();
                let read = fd.new_unique_out(width, load).unwrap();
                fd.op_insert_before(load, end.restore);
                fd.op_set_input(end.restore, read, 0).unwrap();
                let resolved = saved_stack_value(&fd, end.value, &mut 65536, &mut SavedFrameQuery::default())
                    .and_then(|saved| saved.value)
                    .and_then(|value| native_stack_delta(&fd, value, allocation.base));
                assert_eq!(resolved == Some(0), !corrupt, "width={width}, corrupt={corrupt}");
            }
        }
    }

    #[test]
    fn cyclic_checkpoint_validates_every_preserving_call() {
        use crate::fspec::FuncCallSpecs;
        for width in [4, 8] {
            for change in ["preserved", "clobbered", "escaped"] {
                let (mut fd, allocation, end, block, index, saved) = checkpoint_call_fixture(width, false, false);
                let stack = Rc::clone(fd.get_arch().manage().get_stack_space().unwrap());
                let address = fd.vbank().get(saved).unwrap().get_addr().clone();
                let first = fd.obank().get(end.restore).unwrap().get_in(0).unwrap();
                let first_marker = fd.vbank().get(first).unwrap().get_def().unwrap();
                let phi = fd.new_op(2, fd.get_address().clone());
                fd.op_set_opcode_code(phi, OpCode::CPUI_MULTIEQUAL);
                fd.op_set_all_input(phi, &[saved, first]).unwrap();
                let merged = fd.new_varnode_out(width, &address, phi).unwrap();
                fd.op_insert_begin(phi, block);
                fd.op_set_input(first_marker, merged, 0).unwrap();
                let call = fd.new_op(if change == "escaped" { 2 } else { 1 }, fd.get_address().clone());
                fd.op_set_opcode_code(call, OpCode::CPUI_CALL);
                let target = fd.new_constant(width, 0x9000);
                fd.op_set_input(call, target, 0).unwrap();
                if change == "escaped" {
                    let input = fd.find_spacebase_input(&stack).unwrap();
                    let add = fd.new_op(2, fd.get_address().clone());
                    fd.op_set_opcode_code(add, OpCode::CPUI_INT_ADD);
                    let offset = fd.new_constant(width, (-32_i64) as u64 & calc_mask(width));
                    fd.op_set_all_input(add, &[input, offset]).unwrap();
                    let pointer = fd.new_unique_out(width, add).unwrap();
                    fd.op_insert_end(add, block);
                    fd.op_set_input(call, pointer, 1).unwrap();
                }
                fd.op_insert_end(call, block);
                let mut model = fd.get_call_specs(index).proto().model().as_ref().clone();
                if change == "clobbered" {
                    let mut locals = kuna_base::address::RangeList::new();
                    locals.insert_range(Rc::clone(&stack), 0, 32);
                    model.set_local_range(locals);
                }
                let mut spec = FuncCallSpecs::new(call, Address::new_invalid());
                spec.proto_mut().attach_internal_store(Rc::new(Datatype::new(0, type_metatype::TYPE_VOID)));
                spec.proto_mut().set_model(Some(Rc::new(model)));
                spec.proto_mut().set_extra_pop(0);
                spec.set_final_input_storage(Vec::new());
                fd.push_call_specs(spec);
                fd.kuna_stack_call_sp.insert(call, Some(allocation.output));
                let marker = fd.new_op(2, fd.get_address().clone());
                fd.op_set_opcode_code(marker, OpCode::CPUI_INDIRECT);
                let effect = fd.new_varnode_iop(call);
                fd.op_set_all_input(marker, &[first, effect]).unwrap();
                let second = fd.new_varnode_out(width, &address, marker).unwrap();
                fd.op_insert_after(marker, call);
                fd.op_set_input(phi, second, 1).unwrap();
                let resolved = saved_stack_value(&fd, end.value, &mut 65536, &mut SavedFrameQuery::default())
                    .and_then(|saved| saved.value)
                    .and_then(|value| native_stack_delta(&fd, value, allocation.base));
                assert_eq!(resolved == Some(0), change == "preserved", "width={width}, change={change}");
            }
        }
    }

    #[test]
    fn saved_words_cross_store_alias_markers_only_after_full_preservation_checks() {
        for width in [4,8] {
            for mode in ["disjoint","clobber","partial","escape"] {
                let (mut fd,root,end,block,_,saved) = checkpoint_call_fixture(width,false,false);
                let point = fd.get_address().clone();
                let stack = Rc::clone(fd.get_arch().manage().get_stack_space().unwrap());
                let entry = fd.find_spacebase_input(&stack).unwrap();
                let address = fd.new_op(2,point.clone());
                fd.op_set_opcode_code(address,OpCode::CPUI_INT_ADD);
                let displacement = if mode=="disjoint" { -16_i64 } else if mode=="partial" { -32_i64+width as i64-1 } else { -32_i64 };
                let offset = fd.new_constant(width,displacement as u64 & calc_mask(width));
                fd.op_set_all_input(address,&[entry,offset]).unwrap();
                let pointer = fd.new_unique_out(width,address).unwrap();
                fd.op_insert_begin(address,block);
                let store = fd.new_op(3,point.clone());
                fd.op_set_opcode_code(store,OpCode::CPUI_STORE);
                let memory = fd.new_constant(4,stack.get_contain().unwrap().get_index() as u64);
                let target = if mode=="escape" { fd.new_constant(width,0x8000) } else { pointer };
                let payload = if mode=="escape" { pointer } else { fd.new_constant(if mode=="partial" {1} else {width},0) };
                fd.op_set_all_input(store,&[memory,target,payload]).unwrap();
                fd.op_insert_after(store,address);
                let alias = fd.new_op(2,point);
                fd.op_set_opcode_code(alias,OpCode::CPUI_INDIRECT);
                let effect = fd.new_varnode_iop(store);
                fd.op_set_all_input(alias,&[saved,effect]).unwrap();
                let slot = fd.vbank().get(saved).unwrap().get_addr().clone();
                let value = fd.new_varnode_out(width,&slot,alias).unwrap();
                fd.op_insert_before(alias,store);
                let call_marker = fd.vbank().get(fd.obank().get(end.restore).unwrap().get_in(0).unwrap()).unwrap().get_def().unwrap();
                fd.op_set_input(call_marker,value,0).unwrap();
                let actual = saved_stack_value(&fd,end.value,&mut 65536,&mut SavedFrameQuery::default())
                    .and_then(|saved|saved.value).and_then(|value|native_stack_delta(&fd,value,root.before));
                assert_eq!(actual==Some(0),mode=="disjoint","width={width}, mode={mode}");
            }
        }
    }


    #[test]
    fn checkpoint_calls_below_chained_allocations_reuse_relational_bounds() {
        for width in [4,8] {
            for downward in [false,true] {
                let (mut fd,root,end,_,index,_) = checkpoint_call_fixture(width,false,false);
                let call = fd.get_call_specs(index).get_op();
                let amount = fd.new_varnode(width,&Address::new(Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap()),0x7000),None);
                let amount = fd.set_input_varnode(amount).unwrap();
                let next = fd.new_op(2,fd.get_address().clone());
                fd.op_set_opcode_code(next,if downward { OpCode::CPUI_INT_SUB } else { OpCode::CPUI_INT_ADD });
                fd.op_set_all_input(next,&[root.output,amount]).unwrap();
                let address = fd.vbank().get(root.output).unwrap().get_addr().clone();
                let value = fd.new_varnode_out(width,&address,next).unwrap();
                fd.op_insert_before(next,call);
                fd.kuna_stack_scope_holds.push(value);
                fd.kuna_stack_call_sp.insert(call,Some(value));
                let restored = saved_stack_value(&fd,end.value,&mut 65536,&mut SavedFrameQuery::default())
                    .and_then(|saved|saved.value).and_then(|value|native_stack_delta(&fd,value,root.before));
                assert_eq!(restored==Some(0),downward,"width={width}, downward={downward}");
            }
        }
    }


fn build_fd() -> Funcdata {
        let manage = build_manager();
        let glb = Rc::new(ArchContext::new(manage));
        let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
        let addr = Address::new(ram, 0x1000);
        Funcdata::new("func", "func", glb, addr, 0x10000000, 0x40).unwrap()
    }
fn frame_slot(size: int4, meta: crate::dtype::type_metatype) -> crate::funcdata::FrameSlot {
        crate::funcdata::FrameSlot {
            name: String::new(),
            dtype: Rc::new(crate::dtype::Datatype::new(size, meta)),
            size,
        }
    }
fn copied_mask(fd: &mut Funcdata, width: int4, mask: u64, reversed: bool) -> OpId {
        let copy = fd.new_op(1, fd.get_address().clone());
        fd.op_set_opcode_code(copy, OpCode::CPUI_COPY);
        let mask = fd.new_constant(width, mask);
        fd.op_set_input(copy, mask, 0).unwrap();
        let mask = fd.new_unique_out(width, copy).unwrap();
        let unknown = fd.new_unique(width, None);
        let op = fd.new_op(2, fd.get_address().clone());
        fd.op_set_opcode_code(op, OpCode::CPUI_INT_AND);
        let (left, right) = if reversed {
            (mask, unknown)
        } else {
            (unknown, mask)
        };
        fd.op_set_input(op, left, 0).unwrap();
        fd.op_set_input(op, right, 1).unwrap();
        fd.new_unique_out(width, op).unwrap();
        op
    }
fn merged_step(
        fd: &mut Funcdata,
        width: int4,
        cyclic: bool,
        step: i64,
        reversed: bool,
    ) -> VarnodeId {
        let join = fd.new_op(2, fd.get_address().clone());
        fd.op_set_opcode_code(join, OpCode::CPUI_MULTIEQUAL);
        let output = fd.new_unique_out(width, join).unwrap();
        let header = fd.new_op(if cyclic { 2 } else { 1 }, fd.get_address().clone());
        fd.op_set_opcode_code(header, OpCode::CPUI_MULTIEQUAL);
        let initial = fd.new_unique(width, None);
        fd.op_set_input(header, initial, 0).unwrap();
        if cyclic {
            fd.op_set_input(header, output, 1).unwrap();
        }
        let base = fd.new_unique_out(width, header).unwrap();
        let add = fd.new_op(2, fd.get_address().clone());
        fd.op_set_opcode_code(add, OpCode::CPUI_INT_ADD);
        fd.op_set_input(add, base, 0).unwrap();
        let amount = fd.new_constant(width, step as u64 & calc_mask(width));
        fd.op_set_input(add, amount, 1).unwrap();
        let advanced = fd.new_unique_out(width, add).unwrap();
        let (left, right) = if reversed {
            (advanced, base)
        } else {
            (base, advanced)
        };
        fd.op_set_input(join, left, 0).unwrap();
        fd.op_set_input(join, right, 1).unwrap();
        output
    }
#[test]
fn restored_storage_observers_require_a_possible_stack_access() {
        for width in [4, 8] {
            for (code, volatile) in [
                (OpCode::CPUI_LOAD, false), (OpCode::CPUI_STORE, false),
                (OpCode::CPUI_LOAD, true), (OpCode::CPUI_STORE, true),
            ] {
                for (offset, expected) in [(Some(0), false), (Some(-1), true), (None, true)] {
                    let mut manager = build_manager();
                    manager.set_default_data_space(4).unwrap();
                    let mut context = ArchContext::new(manager);
                    let ram = Rc::clone(context.manage().get_space_by_name("ram").unwrap());
                    let mut globals = crate::context::GlobalQuery::default();
                    globals.owned.insert_range(Rc::clone(&ram), 0x8000, 0x900f);
                    context.elem_ptr_ranges = Rc::new(vec![(0x8000, 0x800f)]);
                    globals.flagbase = kuna_base::partmap::PartMap::new(
                        if volatile { crate::varnode::varnode_flags::volatil } else { 0 },
                    );
                    context.global_query = Some(Rc::new(globals));
                    let mut fd = Funcdata::new("func", "func", Rc::new(context),
                        Address::new(Rc::clone(&ram), 0x1000), 0x10000000, 0x40).unwrap();
                    let input = fd.new_varnode(width, &Address::new(ram, 0x4000), None);
                    let top = fd.set_input_varnode(input).unwrap();
                    let root = fd.bblocks_root_pub();
                    let block = fd.bblocks_mut().new_block_basic(root);
                    let restore = fd.new_op(1, fd.get_address().clone());
                    fd.op_set_opcode_code(restore, OpCode::CPUI_COPY);
                    fd.op_set_input(restore, top, 0).unwrap();
                    fd.new_unique_out(width, restore).unwrap();
                    fd.op_insert_end(restore, block);
                    let address = if let Some(offset) = offset {
                        let add = fd.new_op(2, fd.get_address().clone());
                        fd.op_set_opcode_code(add, OpCode::CPUI_INT_ADD);
                        fd.op_set_input(add, top, 0).unwrap();
                        let delta = fd.new_constant(width, offset as u64 & calc_mask(width));
                        fd.op_set_input(add, delta, 1).unwrap();
                        let out = fd.new_unique_out(width, add).unwrap();
                        fd.op_insert_end(add, block);
                        out
                    } else {
                        let ram = Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap());
                        let value = fd.new_varnode(width, &Address::new(ram, 0x5000), None);
                        fd.set_input_varnode(value).unwrap()
                    };
                    let access = fd.new_op(if code == OpCode::CPUI_STORE { 3 } else { 2 }, fd.get_address().clone());
                    fd.op_set_opcode_code(access, code);
                    let space = fd.new_constant(4, 4);
                    fd.op_set_input(access, space, 0).unwrap();
                    fd.op_set_input(access, address, 1).unwrap();
                    if code == OpCode::CPUI_STORE {
                        let value = fd.new_constant(2, 7);
                        fd.op_set_input(access, value, 2).unwrap();
                    } else {
                        fd.new_unique_out(2, access).unwrap();
                    }
                    fd.op_insert_end(access, block);
                    assert_eq!(observes_after_restore(&fd, restore, top, &mut 4096), expected);
                    let global = fd.new_constant(width, 0x8000);
                    fd.op_set_input(access, global, 1).unwrap();
                    assert_eq!(observes_after_restore(&fd, restore, top, &mut 4096), volatile);
                    let unmapped = fd.new_constant(width, 0x9000);
                    fd.op_set_input(access, unmapped, 1).unwrap();
                    assert!(observes_after_restore(&fd, restore, top, &mut 4096));
                    let crossing = fd.new_constant(width, 0x800f);
                    fd.op_set_input(access, crossing, 1).unwrap();
                    assert!(observes_after_restore(&fd, restore, top, &mut 4096));
                    fd.op_set_input(access, global, 1).unwrap();
                    let call = fd.new_op(1, fd.get_address().clone());
                    fd.op_set_opcode_code(call, OpCode::CPUI_CALL);
                    let target = fd.new_constant(width, 0x1000);
                    fd.op_set_input(call, target, 0).unwrap();
                    fd.op_insert_end(call, block);
                    assert!(observes_after_restore(&fd, restore, top, &mut 4096));
                }
            }
        }
    }
#[test]
fn native_frame_delta_cancels_adjustments_without_equating_distinct_values() {
        for width in [4, 8] {
            let mut fd = build_fd();
            let ram = Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap());
            let mut input = |offset| {
                let value = fd.new_varnode(width, &Address::new(Rc::clone(&ram), offset), None);
                fd.set_input_varnode(value).unwrap()
            };
            let entry = input(0x4000);
            let amount = input(0x5000);
            let other = input(0x6000);
            let expression = |fd: &mut Funcdata, code, inputs: &[VarnodeId]| {
                let op = fd.new_op(inputs.len() as int4, fd.get_address().clone());
                fd.op_set_opcode_code(op, code);
                for (slot, &value) in inputs.iter().enumerate() {
                    fd.op_set_input(op, value, slot as int4).unwrap();
                }
                fd.new_unique_out(width, op).unwrap()
            };
            let allocated = expression(&mut fd, OpCode::CPUI_INT_SUB, &[entry, amount]);
            let restored = expression(&mut fd, OpCode::CPUI_INT_ADD, &[allocated, amount]);
            assert_eq!(native_stack_delta(&fd, restored, entry), Some(0));
            let different = expression(&mut fd, OpCode::CPUI_INT_ADD, &[allocated, other]);
            assert_eq!(native_stack_delta(&fd, different, entry), None);
            let sixteen = fd.new_constant(width, 16);
            let partial = expression(&mut fd, OpCode::CPUI_INT_ADD, &[allocated, sixteen]);
            assert_eq!(native_stack_delta(&fd, partial, allocated), Some(16));
            let high_bit = fd.new_constant(width, 1u64 << (width * 8 - 1));
            let once = expression(&mut fd, OpCode::CPUI_INT_ADD, &[allocated, high_bit]);
            let wrapped = expression(&mut fd, OpCode::CPUI_INT_ADD, &[once, high_bit]);
            assert_eq!(native_stack_delta(&fd, wrapped, allocated), Some(0));
        }
    }

    #[test]
    fn compiler_padding_does_not_wrap_with_the_native_count() {
        fn evaluate(fd: &Funcdata, value: VarnodeId, scalar: VarnodeId, n: u64) -> u64 {
            let node = fd.vbank().get(value).unwrap();
            let mask = calc_mask(node.get_size());
            if value == scalar { return n & mask; }
            if let Some(value) = number(fd, value) { return value as u64 & mask; }
            let op = fd.obank().get(node.get_def().unwrap()).unwrap();
            let first = evaluate(fd, op.get_in(0).unwrap(), scalar, n);
            match op.code() {
                OpCode::CPUI_CAST | OpCode::CPUI_COPY | OpCode::CPUI_INT_ZEXT => first & mask,
                OpCode::CPUI_INT_ADD => first.wrapping_add(evaluate(fd, op.get_in(1).unwrap(), scalar, n)) & mask,
                code => panic!("unexpected backing count operation {code:?}"),
            }
        }
        for width in [2, 4] {
            for (reserve, alignment, slack) in [
                (Reservation { count: 16, shift: 16 }, None, 16),
                (Reservation { count: 0, shift: 0 }, Some((16, 0)), 15),
            ] {
                let (mut fd, root, _, _) = scoped_lifetime_fixture(width, false);
                lower(&mut fd, root, root.amount, width, reserve, alignment, None);
                let allocate = fd.obank().iter_alive().find(|&op| {
                    let op = fd.obank().get(op).unwrap();
                    op.code() == OpCode::CPUI_CALLOTHER && op.get_in(0)
                        .and_then(|v| number(&fd, v)) == Some(BUILTIN_STACK_ALLOC as i128)
                }).unwrap();
                let count = fd.obank().get(allocate).unwrap().get_in(1).unwrap();
                let mask = calc_mask(width);
                for n in [0, 1, mask - slack, mask - slack + 1, mask] {
                    assert_eq!(evaluate(&fd, count, root.amount, n), n + slack,
                        "width={width} slack={slack} native_count={n:#x}");
                }
            }
        }
    }

    #[test]
    fn scaled_decrements_preserve_modular_backing_counts_and_distinct_origins() {
        fn evaluate(fd: &Funcdata, value: VarnodeId, before: VarnodeId, top: u64, scalar: VarnodeId, n: u64, mask: u64) -> u64 {
            if value == before { return top & mask; }
            if value == scalar { return n & mask; }
            if let Some(number) = number(fd, value) { return number as u64 & mask; }
            let op = fd.obank().get(fd.vbank().get(value).unwrap().get_def().unwrap()).unwrap();
            let a = evaluate(fd, op.get_in(0).unwrap(), before, top, scalar, n, mask);
            match op.code() {
                OpCode::CPUI_CAST | OpCode::CPUI_COPY => a,
                OpCode::CPUI_INT_ADD => a.wrapping_add(evaluate(fd, op.get_in(1).unwrap(), before, top, scalar, n, mask)) & mask,
                OpCode::CPUI_INT_MULT => a.wrapping_mul(evaluate(fd, op.get_in(1).unwrap(), before, top, scalar, n, mask)) & mask,
                _ => panic!("unexpected count expression"),
            }
        }
        for width in [4, 8] {
            for scale in [2_i128, 4, 16] {
                for commuted in [false, true] {
                    let (mut fd, old, end, _) = scoped_lifetime_fixture(width, false);
                    let point = fd.obank().get(old.op).unwrap().get_addr().clone();
                    let product = fd.new_op(2, point.clone());
                    fd.op_set_opcode_code(product, OpCode::CPUI_INT_MULT);
                    let factor = fd.new_constant(width, (-scale) as u64 & calc_mask(width));
                    let inputs = if commuted { [factor, old.amount] } else { [old.amount, factor] };
                    fd.op_set_all_input(product, &inputs).unwrap();
                    let step = fd.new_unique_out(width, product).unwrap();
                    fd.op_insert_before(product, old.op);
                    fd.op_set_opcode_code(old.op, OpCode::CPUI_INT_ADD);
                    fd.op_set_all_input(old.op, &[old.base, step]).unwrap();
                    let root = allocation(&fd, old.op).unwrap();
                    let other = fd.new_op(2, point.clone());
                    fd.op_set_opcode_code(other, OpCode::CPUI_INT_SUB);
                    fd.op_set_all_input(other, &[old.base, old.amount]).unwrap();
                    fd.new_unique_out(width, other).unwrap();
                    fd.op_insert_before(other, end.restore);
                    assert_eq!(origin_delta(&fd, &root, &allocation(&fd, other).unwrap()), None);
                    let ty = fd.get_arch().types_rc().unwrap().get_base(width, type_metatype::TYPE_UINT).unwrap();
                    let count = scaled_amount(&mut fd, &root, width, &ty);
                    for n in [0, 1, 15, 16, 17, (calc_mask(width) >> 4) - 1, calc_mask(width) >> 4, calc_mask(width)] {
                        let top = 0x10000;
                        let native = evaluate(&fd, root.output, root.before, top, root.amount, n, calc_mask(width));
                        let backing = evaluate(&fd, count, root.before, top, root.amount, n, calc_mask(width));
                        assert_eq!(top.wrapping_sub(native) & calc_mask(width), backing, "width={width} scale={scale} n={n:#x} commuted={commuted}");
                    }
                }
            }
        }
    }

    #[test]
    fn scaled_origin_pairing_uses_byte_steps_not_scalar_steps() {
        for width in [4, 8] {
            for base_step in [1, 16] {
                let (mut fd, root, end, _) = scoped_lifetime_fixture(width, false);
                let expression = |fd: &mut Funcdata, code, a, b| {
                    let op = fd.new_op(2, fd.get_address().clone());
                    fd.op_set_opcode_code(op, code);
                    fd.op_set_all_input(op, &[a,b]).unwrap();
                    let value = fd.new_unique_out(width, op).unwrap();
                    fd.op_insert_before(op, end.restore);
                    value
                };
                let step = fd.new_constant(width, base_step);
                let one = fd.new_constant(width, 1);
                let p = expression(&mut fd, OpCode::CPUI_INT_ADD, root.base, step);
                let n = expression(&mut fd, OpCode::CPUI_INT_ADD, root.amount, one);
                let result = paired_origins(&fd, p, n, 16, &mut BTreeSet::new()).unwrap();
                let &(p,n) = result.iter().next().unwrap();
                let (pbase,poffset) = canonical_base(&fd, p, &mut BTreeSet::new());
                let (nbase,noffset) = canonical_base(&fd, n, &mut BTreeSet::new());
                let (base,bias) = canonical_base(&fd, root.base, &mut BTreeSet::new());
                assert!(equivalent(&fd, pbase, base, &mut BTreeSet::new()));
                assert!(equivalent(&fd, nbase, root.amount, &mut BTreeSet::new()));
                assert_eq!(poffset - bias - 16 * noffset, base_step as i128 - 16);
            }
        }
    }

    #[test]
    fn scaled_decrements_fold_constant_chains_without_accepting_upward_steps() {
        for width in [4, 8] {
            for (inner, outer, expected) in [(2_i64,-8_i64,Some(16)),(-2,8,Some(16)),(-2,-8,None),(2,8,None),(0,-8,None),(-2,-1,None)] {
                let (mut fd, root, end, _) = scoped_lifetime_fixture(width, false);
                let product = |fd: &mut Funcdata, value, factor| {
                    let op = fd.new_op(2, fd.get_address().clone());
                    fd.op_set_opcode_code(op, OpCode::CPUI_INT_MULT);
                    let factor = fd.new_constant(width, factor as u64 & calc_mask(width));
                    fd.op_set_all_input(op, &[factor, value]).unwrap();
                    let value = fd.new_unique_out(width, op).unwrap();
                    fd.op_insert_before(op, end.restore);
                    value
                };
                let inside = product(&mut fd, root.amount, inner);
                let outside = product(&mut fd, inside, outer);
                let op = fd.new_op(2, fd.get_address().clone());
                fd.op_set_opcode_code(op, OpCode::CPUI_INT_ADD);
                fd.op_set_all_input(op, &[root.before, outside]).unwrap();
                fd.new_unique_out(width, op).unwrap();
                fd.op_insert_before(op, end.restore);
                let parsed = allocation(&fd, op);
                assert_eq!(parsed.map(|root| root.scale), expected);
                if let Some(parsed) = parsed {
                    assert_eq!(native_stack_sum(&fd, width, vec![(parsed.output,1),(parsed.before,-1),(parsed.amount,parsed.scale)], None), Some(0));
                }
            }
        }
    }


#[test]
fn recovered_write_refines_only_filler_and_preserves_neighbor_bounds() {
        use crate::dtype::type_metatype::{TYPE_INT, TYPE_UNKNOWN};
        for width in [2, 8, 12] {
            let fd = build_fd();
            fd.record_frame_slots([
                (-48, frame_slot(12, TYPE_UNKNOWN)),
                (-36, frame_slot(4, TYPE_INT)),
            ]);
            fd.record_frame_writes([(-48, width), (-36, 2)]);
            let slots = fd.frame_slots();
            let expected = if width == 12 {
                vec![(-48, 12), (-36, 4)]
            } else {
                vec![(-48, width), (-48 + i64::from(width), 12 - width), (-36, 4)]
            };
            assert_eq!(slots.iter().map(|(off, slot)| (*off, slot.size)).collect::<Vec<_>>(), expected);
            assert_eq!(slots.first().unwrap().1.dtype.get_metatype(), TYPE_UNKNOWN);
            assert_eq!(slots.last().unwrap().1.dtype.get_metatype(), TYPE_INT);
        }
    }
#[test]
fn recovered_write_cannot_widen_cross_a_boundary_or_invent_a_slot() {
        use crate::dtype::type_metatype::{TYPE_INT, TYPE_UNKNOWN};
        for (off, width) in [(-48, 16), (-48, 20), (-48, 0), (-40, 4)] {
            let fd = build_fd();
            fd.record_frame_slots([
                (-48, frame_slot(12, TYPE_UNKNOWN)),
                (-36, frame_slot(4, TYPE_INT)),
            ]);
            fd.record_frame_writes([(off, width)]);
            let slots = fd.frame_slots();
            assert_eq!(slots.iter().map(|(off, slot)| (*off, slot.size)).collect::<Vec<_>>(),
                vec![(-48, 12), (-36, 4)]);
            assert_eq!(slots.first().unwrap().1.dtype.get_metatype(), TYPE_UNKNOWN);
            assert_eq!(slots.last().unwrap().1.dtype.get_metatype(), TYPE_INT);
        }
        let fd = build_fd();
        fd.record_frame_slots([
            (-48, frame_slot(16, TYPE_UNKNOWN)),
            (-36, frame_slot(4, TYPE_INT)),
        ]);
        fd.record_frame_writes([(-48, 2)]);
        assert_eq!(fd.frame_slots().iter().map(|(off, slot)| (*off, slot.size)).collect::<Vec<_>>(),
            vec![(-48, 16), (-36, 4)]);
    }
#[test]
fn cancelling_offsets_do_not_hide_a_stack_reservation() {
        for width in [4, 8] {
            let mut fd = build_fd();
            let ram = Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap());
            let input = fd.new_varnode(width, &Address::new(ram, 0x2000), None);
            let amount = fd.set_input_varnode(input).unwrap();
            let negative = fd.new_op(1, fd.get_address().clone());
            fd.op_set_opcode_code(negative, OpCode::CPUI_INT_2COMP);
            fd.op_set_input(negative, amount, 0).unwrap();
            let negated = fd.new_unique_out(width, negative).unwrap();
            let add = fd.new_op(2, fd.get_address().clone());
            fd.op_set_opcode_code(add, OpCode::CPUI_INT_ADD);
            fd.op_set_input(add, negated, 0).unwrap();
            let minus = fd.new_constant(width, (-20_i64) as u64 & calc_mask(width));
            fd.op_set_input(add, minus, 1).unwrap();
            let displaced = fd.new_unique_out(width, add).unwrap();
            let restore = fd.new_op(2, fd.get_address().clone());
            fd.op_set_opcode_code(restore, OpCode::CPUI_INT_ADD);
            let plus = fd.new_constant(width, 20);
            fd.op_set_input(restore, plus, 0).unwrap();
            fd.op_set_input(restore, displaced, 1).unwrap();
            let restored = fd.new_unique_out(width, restore).unwrap();
            let input = fd.new_varnode(width, &Address::new(Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap()), 0x3000), None);
            let before = fd.set_input_varnode(input).unwrap();
            let wrapped = fd.new_op(2, fd.get_address().clone());
            fd.op_set_opcode_code(wrapped, OpCode::CPUI_INT_ADD);
            fd.op_set_all_input(wrapped, &[before, restored]).unwrap();
            fd.new_unique_out(width, wrapped).unwrap();
            let direct = fd.new_op(2, fd.get_address().clone());
            fd.op_set_opcode_code(direct, OpCode::CPUI_INT_SUB);
            fd.op_set_all_input(direct, &[before, amount]).unwrap();
            fd.new_unique_out(width, direct).unwrap();
            assert_eq!(origin_delta(&fd, &allocation(&fd, wrapped).unwrap(), &allocation(&fd, direct).unwrap()), Some(0));
        }
    }
#[test]
fn saved_frame_alias_requires_an_unmodified_slot() {
        for width in [4, 8] {
            let mut fd = build_fd();
            let ram = Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap());
            let root = fd.bblocks_root_pub();
            let block = fd.bblocks_mut().new_block_basic(root);
            let input = fd.new_varnode(width, &Address::new(Rc::clone(&ram), 0x2000), None);
            let base = fd.set_input_varnode(input).unwrap();
            let input = fd.new_varnode(width, &Address::new(Rc::clone(&ram), 0x2010), None);
            let amount = fd.set_input_varnode(input).unwrap();
            let append = |fd: &mut Funcdata, code, inputs: &[VarnodeId], output: bool| {
                let op = fd.new_op(inputs.len() as int4, fd.get_address().clone());
                fd.op_set_opcode_code(op, code);
                for (slot, &value) in inputs.iter().enumerate() {
                    fd.op_set_input(op, value, slot as int4).unwrap();
                }
                let value = output.then(|| fd.new_unique_out(width, op).unwrap());
                fd.op_insert_end(op, block);
                (op, value)
            };
            let (first, output) = append(&mut fd, OpCode::CPUI_INT_SUB, &[base, amount], true);
            let output = output.unwrap();
            let negative = fd.new_constant(width, (-2 * width as i64) as u64 & calc_mask(width));
            let pointer = append(&mut fd, OpCode::CPUI_INT_ADD, &[output, negative], true).1.unwrap();
            let negative = fd.new_constant(width, (-(width as i64)) as u64 & calc_mask(width));
            let other = append(&mut fd, OpCode::CPUI_INT_ADD, &[output, negative], true).1.unwrap();
            let distance = fd.new_constant(width, (4 * width) as u64);
            let saved = append(&mut fd, OpCode::CPUI_INT_ADD, &[base, distance], true).1.unwrap();
            let space = fd.new_constant(4, ram.get_index() as u64);
            append(&mut fd, OpCode::CPUI_STORE, &[space, pointer, saved], false);
            let (load, loaded) = append(&mut fd, OpCode::CPUI_LOAD, &[space, pointer], true);
            let loaded = loaded.unwrap();
            let restored = append(&mut fd, OpCode::CPUI_INT_SUB, &[loaded, distance], true).1.unwrap();
            let (second, _) = append(&mut fd, OpCode::CPUI_INT_SUB, &[restored, amount], true);
            let first = allocation(&fd, first).unwrap();
            let second = allocation(&fd, second).unwrap();
            assert_eq!(origin_delta(&fd, &first, &second), Some(0));

            let interference = fd.new_op(3, fd.get_address().clone());
            fd.op_set_opcode_code(interference, OpCode::CPUI_STORE);
            fd.op_set_input(interference, space, 0).unwrap();
            fd.op_set_input(interference, pointer, 1).unwrap();
            let byte = fd.new_constant(1, 7);
            fd.op_set_input(interference, byte, 2).unwrap();
            fd.op_insert_before(interference, load);
            assert_eq!(origin_delta(&fd, &first, &second), None);
            fd.op_set_input(interference, other, 1).unwrap();
            assert_eq!(origin_delta(&fd, &first, &second), Some(0));

            let call = fd.new_op(1, fd.get_address().clone());
            fd.op_set_opcode_code(call, OpCode::CPUI_CALL);
            let target = fd.new_constant(width, 0x3000);
            fd.op_set_input(call, target, 0).unwrap();
            fd.op_insert_before(call, load);
            let spec = crate::fspec::FuncCallSpecs::new(call, Address::new_invalid());
            fd.push_call_specs(spec);
            assert_eq!(origin_delta(&fd, &first, &second), None);
        }
    }
#[test]
fn reconverging_phi_does_not_hide_a_stepped_cycle() {
        for width in [4, 8] {
            for reversed in [false, true] {
                let mut fd = build_fd();
                let base = merged_step(&mut fd, width, true, -4096, reversed);
                assert!(stepped_cycle(&fd, base));
            }
        }
    }
#[test]
fn reconverging_zero_step_cycle_is_not_a_probe() {
        for width in [4, 8] {
            for reversed in [false, true] {
                let mut fd = build_fd();
                let base = merged_step(&mut fd, width, true, 0, reversed);
                assert!(!stepped_cycle(&fd, base));
            }
        }
    }
#[test]
fn acyclic_phi_offsets_are_not_a_stepped_cycle() {
        for width in [4, 8] {
            for reversed in [false, true] {
                let mut fd = build_fd();
                let base = merged_step(&mut fd, width, false, -4096, reversed);
                assert!(!stepped_cycle(&fd, base));
            }
        }
    }

#[test]
fn negation_does_not_use_the_value_before_a_call_clobber() {
    let mut fd = build_fd();
    let at = fd.get_address().clone();
    let amount = fd.new_unique(4, None);
    let neg = fd.new_op(1, at.clone());
    fd.op_set_opcode_code(neg, OpCode::CPUI_INT_2COMP);
    fd.op_set_input(neg, amount, 0).unwrap();
    let previous = fd.new_unique_out(4, neg).unwrap();
    let call = fd.new_op(1, at.clone());
    fd.op_set_opcode_code(call, OpCode::CPUI_CALL);
    let marker = fd.new_op(2, at);
    fd.op_set_opcode_code(marker, OpCode::CPUI_INDIRECT);
    fd.op_set_input(marker, previous, 0).unwrap();
    let effect = fd.new_varnode_iop(call);
    fd.op_set_input(marker, effect, 1).unwrap();
    let current = fd.new_unique_out(4, marker).unwrap();
    assert_eq!(negate(&fd, previous), Some((amount, 1)));
    assert_eq!(negate(&fd, current), None);
}
#[test]
fn copied_zero_mask_is_not_pointer_alignment() {
        for width in [4, 8] {
            for reversed in [false, true] {
                let mut fd = build_fd();
                let op = copied_mask(&mut fd, width, 0, reversed);
                assert_eq!(alignment(&fd, op), None);
            }
        }
    }
#[test]
fn copied_alignment_mask_preserves_operand_and_width() {
        for width in [4, 8] {
            for reversed in [false, true] {
                let mut fd = build_fd();
                let op = copied_mask(&mut fd, width, calc_mask(width) & !63, reversed);
                assert_eq!(alignment(&fd, op), Some((if reversed { 1 } else { 0 }, 64)));
            }
        }
    }
fn alignment_fd(width: int4, enabled: bool) -> (Funcdata, VarnodeId, BlockId) {
        use kuna_base::space::{SpacebaseSpace, VarnodeStorage};
        let mut manager = build_manager();
        let ram = Rc::clone(manager.get_space_by_name("ram").unwrap());
        let register = Rc::new(AddrSpace::new(spacetype::IPTR_PROCESSOR, "register", false,
            width as u32, 1, 5, addrspace_flags::hasphysical, 0, 0));
        manager.insert_space(Rc::clone(&register)).unwrap();
        let stack = Rc::new(SpacebaseSpace::new("stack", 6, width as u32, &ram, 1, true, false));
        manager.add_spacebase_pointer(&stack, &VarnodeStorage {
            space: Some(Rc::clone(&register)), offset: 0, size: width as u32,
        }, width, true).unwrap();
        manager.insert_space(stack).unwrap();
        let mut context = ArchContext::new(manager);
        context.stack_alloc = enabled;
        let types = Rc::new(crate::dtype::TypeFactoryImpl::new());
        types.set_max_basetype_size(8);
        types.setup_sizes(Some(width), width, 4);
        context.types = Some(types);
        context.opbehaviors = vec![None; OpCode::CPUI_MAX as usize];
        context.opbehaviors[OpCode::CPUI_INT_ADD as usize] = Some(Rc::new(kuna_num::opbehavior::OpBehaviorIntAdd));
        context.opbehaviors[OpCode::CPUI_INT_ZEXT as usize] = Some(Rc::new(kuna_num::opbehavior::OpBehaviorIntZext));
        context.opbehaviors[OpCode::CPUI_INT_AND as usize] = Some(Rc::new(kuna_num::opbehavior::OpBehaviorIntAnd));
        let mut fd = Funcdata::new("alignment", "alignment", Rc::new(context),
            Address::new(ram, 0x1000), 0x10000000, 0x40).unwrap();
        let input = fd.new_varnode(width, &Address::new(register, 0), None);
        let input = fd.set_input_varnode(input).unwrap();
        fd.vbank_mut().get_mut(input).unwrap().set_spacebase();
        let graph = fd.bblocks_root_pub();
        let block = fd.bblocks_mut().new_block_basic(graph);
        (fd, input, block)
    }
fn alignment_op(fd: &mut Funcdata, block: BlockId, width: int4, code: OpCode, inputs: &[VarnodeId]) -> (OpId, VarnodeId) {
        let op = fd.new_op(inputs.len() as int4, fd.get_address().clone());
        fd.op_set_opcode_code(op, code);
        fd.op_set_all_input(op, inputs).unwrap();
        let output = fd.new_unique_out(width, op).unwrap();
        fd.op_insert_end(op, block);
        (op, output)
    }
#[test]
fn native_alignment_witnesses_do_not_change_abstract_frame_coordinates() {
        for width in [2, 4, 8] {
            for (enabled, touched, settled) in [(false, false, false), (false, true, false),
                (true, false, false), (true, true, false), (true, true, true)] {
                let (mut fd, input, block) = alignment_fd(width, enabled);
                if touched { fd.touch_stack_alloc(); }
                if settled { fd.settle_stack_alloc(); }
                let mask = fd.new_constant(width, calc_mask(width) & !63);
                let (_, aligned) = alignment_op(&mut fd, block, width, OpCode::CPUI_INT_AND, &[input, mask]);
                let register = Rc::clone(fd.vbank().get(input).unwrap().get_space());
                let other = fd.new_varnode(width, &Address::new(register, 64), None);
                let other = fd.set_input_varnode(other).unwrap();
                fd.vbank_mut().get_mut(other).unwrap().set_spacebase();
                alignment_op(&mut fd, block, width, OpCode::CPUI_COPY, &[other]);
                fd.calc_nz_mask();
                assert_eq!(fd.vbank().get(input).unwrap().get_nz_mask() & 255, 0);
                assert_eq!(fd.vbank().get(other).unwrap().get_nz_mask() & 255, 0);
                assert_eq!(fd.vbank().get(aligned).unwrap().get_nz_mask() & 63, 0);
            }
        }
    }
#[test]
fn native_sp_low_mask_cancellation_requires_exact_full_width_identity() {
        for width in [2, 4, 8] {
            for (same, mask_bits, constant, mask_width, rotated) in [(true, 63, -192_i64, width, false),
                (true, 63, -192, width, true), (false, 63, -192, width, false),
                (true, 62, -192, width, false), (true, 63, -191, width, false),
                (true, 63, -192, width * 2, false)] {
                let (mut fd, input, block) = alignment_fd(width, true);
                let other = if same { input } else { fd.new_unique(width, None) };
                let other = if rotated {
                    let zero = fd.new_constant(1, 0);
                    let bits = fd.new_constant(1, width as u64 * 8);
                    let (_, left) = alignment_op(&mut fd, block, width, OpCode::CPUI_INT_LEFT, &[other, zero]);
                    let (_, right) = alignment_op(&mut fd, block, width, OpCode::CPUI_INT_RIGHT, &[other, bits]);
                    alignment_op(&mut fd, block, width, OpCode::CPUI_INT_OR, &[left, right]).1
                } else { other };
                let mask = fd.new_constant(mask_width, mask_bits);
                let (_, low) = alignment_op(&mut fd, block, width, OpCode::CPUI_INT_AND, &[other, mask]);
                let constant_node = fd.new_constant(width, constant as u64 & calc_mask(width));
                let (_, adjustment) = alignment_op(&mut fd, block, width, OpCode::CPUI_INT_SUB, &[constant_node, low]);
                let base_op = fd.new_op(2, fd.get_address().clone());
                fd.op_set_opcode_code(base_op, OpCode::CPUI_INT_ADD);
                fd.op_set_all_input(base_op, &[input, adjustment]).unwrap();
                let address = fd.vbank().get(input).unwrap().get_addr().clone();
                let base = fd.new_varnode_out(width, &address, base_op).unwrap();
                fd.op_insert_end(base_op, block);
                let amount = fd.new_unique(width, None);
                let (op, output) = alignment_op(&mut fd, block, width, OpCode::CPUI_INT_SUB, &[base, amount]);
                let allocation = Allocation { op, output, amount, scale: 1, base, before: base };
                fd.vbank_mut().get_mut(base).unwrap().set_auto_live_hold();
                fd.kuna_stack_scope_holds.push(base);
                let mut action = ActionStackAlloc {
                    base: ActionBase::new(0, "stackalloc", "stackvars"), stack_definitions: Vec::new(),
                    native: vec![(base_op, base, true)], alignments: Vec::new(), finalized_pass: None, finished: false,
                };
                action.retain_alignments(&mut fd, &[allocation]);
                assert_eq!(action.alignments, if same && mask_bits == 63 && constant == -192 && mask_width == width {
                    vec![(base, 64)]
                } else { Vec::new() });
                action.release_native(&mut fd);
            }
        }
    }
#[test]
fn native_alignment_zero_rotate_requires_same_full_width_source() {
        for width in [2, 4, 8] {
            for swapped in [false, true] {
                for (same, rotation, narrow, arithmetic) in [(true, 0, false, false),
                    (false, 0, false, false), (true, 1, false, false),
                    (true, 0, true, false), (true, 0, false, true)] {
                    let (mut fd, input, block) = alignment_fd(width, true);
                    let other = if same { input } else { fd.new_unique(width, None) };
                    let count = fd.new_constant(1, rotation);
                    let complement = fd.new_constant(1, width as u64 * 8 - rotation);
                    let (_, left) = alignment_op(&mut fd, block, if narrow { width / 2 } else { width },
                        OpCode::CPUI_INT_LEFT, &[input, count]);
                    let (_, right) = alignment_op(&mut fd, block, width,
                        if arithmetic { OpCode::CPUI_INT_SRIGHT } else { OpCode::CPUI_INT_RIGHT }, &[other, complement]);
                    let pair = if swapped { [right, left] } else { [left, right] };
                    let (_, value) = alignment_op(&mut fd, block, width, OpCode::CPUI_INT_OR, &pair);
                    assert_eq!(alignment_zero_rotate_input(&fd, value),
                        (same && rotation == 0 && !narrow && !arithmetic).then_some(input));
                }
            }
        }
    }
#[test]
fn native_interior_alignment_survives_abstract_mask_simplification() {
        for width in [2, 4, 8] {
            let (mut fd, input, block) = alignment_fd(width, true);
            let negative = fd.new_constant(width, (-24_i64) as u64 & calc_mask(width));
            let (_, base) = alignment_op(&mut fd, block, width, OpCode::CPUI_INT_ADD, &[input, negative]);
            let source = fd.new_unique(width, None);
            let mask = fd.new_constant(width, calc_mask(width) & !63);
            let (_, amount) = alignment_op(&mut fd, block, width, OpCode::CPUI_INT_AND, &[source, mask]);
            let (op, output) = alignment_op(&mut fd, block, width, OpCode::CPUI_INT_SUB, &[base, amount]);
            let allocation = Allocation { op, output, amount, scale: 1, base, before: base };
            let bump = fd.new_constant(width, 63);
            let (_, pointer) = alignment_op(&mut fd, block, width, OpCode::CPUI_INT_ADD, &[output, bump]);
            let (masked, pointer) = alignment_op(&mut fd, block, width, OpCode::CPUI_INT_AND, &[pointer, mask]);
            let unrelated = copied_mask(&mut fd, width, calc_mask(width) & !127, false);
            fd.obank_mut().mark_alive(unrelated);
            let mut action = ActionStackAlloc {
                base: ActionBase::new(0, "stackalloc", "stackvars"), stack_definitions: Vec::new(),
                native: Vec::new(), alignments: Vec::new(), finalized_pass: None, finished: false,
            };
            action.retain_alignments(&mut fd, &[allocation]);
            assert_eq!(action.alignments, vec![(pointer, 64)]);
            assert!(fd.kuna_stack_scope_holds.contains(&pointer));
            fd.touch_stack_alloc();
            fd.calc_nz_mask();
            assert_eq!(fd.vbank().get(input).unwrap().get_nz_mask() & 255, 0);
            // The abstract frame pool folds this native mask to root + 24.
            fd.op_set_opcode_code(masked, OpCode::CPUI_INT_ADD);
            let delta = fd.new_constant(width, 24);
            fd.op_set_all_input(masked, &[output, delta]).unwrap();
            let equations = FrameEquations::new(&fd);
            let alignment = witnessed_alignment(&fd, &allocation, &equations, &action.alignments).unwrap();
            assert_eq!(alignment, Some((64, 40)));
            lower(&mut fd, allocation, amount, width, Reservation { count: 128, shift: 16 }, alignment, None);
            let backing = fd.obank().iter_alive().find_map(|id| {
                let op = fd.obank().get(id).unwrap();
                (op.code() == OpCode::CPUI_CALLOTHER).then(|| op.get_out().unwrap())
            }).unwrap();
            let size = fd.new_constant(width, 128);
            for low in [0, 8, 16, 63] {
                let address = fd.new_constant(width, 0x2000 + low);
                let result = literal_using(&fd, pointer, &mut 256, &|value|
                    (if value == backing { address } else if value == source { size } else { value }, false)).unwrap();
                assert_eq!(result & 63, 0);
            }
            action.release_native(&mut fd);
            assert!(!fd.kuna_stack_scope_holds.contains(&pointer));
            assert!(!fd.vbank().get(pointer).unwrap().is_auto_live_hold());
        }
    }
#[test]
fn native_static_alignment_survives_abstract_frame_simplification() {
        for width in [2, 4, 8] {
            let (mut fd, input, block) = alignment_fd(width, true);
            let mask = fd.new_constant(width, calc_mask(width) & !63);
            let (masked, aligned) = alignment_op(&mut fd, block, width, OpCode::CPUI_INT_AND, &[input, mask]);
            let negative = fd.new_constant(width, (-24_i64) as u64 & calc_mask(width));
            let (_, base) = alignment_op(&mut fd, block, width, OpCode::CPUI_INT_ADD, &[aligned, negative]);
            let source = fd.new_unique(width, None);
            let (_, amount) = alignment_op(&mut fd, block, width, OpCode::CPUI_INT_AND, &[source, mask]);
            let (op, output) = alignment_op(&mut fd, block, width, OpCode::CPUI_INT_SUB, &[base, amount]);
            let allocation = Allocation { op, output, amount, scale: 1, base, before: base };
            let mut action = ActionStackAlloc {
                base: ActionBase::new(0, "stackalloc", "stackvars"), stack_definitions: Vec::new(),
                native: Vec::new(), alignments: Vec::new(), finalized_pass: None, finished: false,
            };
            action.retain_alignments(&mut fd, &[allocation]);
            assert_eq!(action.alignments, vec![(aligned, 64)]);
            fd.op_set_opcode_code(masked, OpCode::CPUI_COPY);
            fd.op_set_all_input(masked, &[input]).unwrap();
            assert_eq!(native_stack_delta(&fd, base, input), Some(-24));
            assert_eq!(allocation_alignment(&fd, &allocation), None);
            assert_eq!(witnessed_alignment(&fd, &allocation, &FrameEquations::new(&fd), &action.alignments).unwrap(),
                Some((64, 40)));
            action.release_native(&mut fd);
        }
    }
#[test]
fn masked_native_base_preserves_shifted_congruence_and_footprint() {
        for width in [2, 4, 8] {
            for align in [16_u64, 64, 4096, 65536, 131072] {
                if align * 4 > calc_mask(width) { continue; }
                for base_offset in [-24_i64, 0, 24] {
                    for amount_offset in [-16_i64, 0, 16] {
                        for shift in [0, 8, 16, 65] {
                            for scoped in [false, true] {
                                let (mut fd, input, block) = alignment_fd(width, true);
                                let mask = fd.new_constant(width, align.wrapping_neg() & calc_mask(width));
                                let (_, base) = alignment_op(&mut fd, block, width, OpCode::CPUI_INT_AND, &[input, mask]);
                                let offset = fd.new_constant(width, base_offset as u64 & calc_mask(width));
                                let (_, base) = alignment_op(&mut fd, block, width, OpCode::CPUI_INT_ADD, &[base, offset]);
                                let source = fd.new_unique(width, None);
                                let (_, amount) = alignment_op(&mut fd, block, width, OpCode::CPUI_INT_AND, &[source, mask]);
                                let offset = fd.new_constant(width, amount_offset as u64 & calc_mask(width));
                                let (_, amount) = alignment_op(&mut fd, block, width, OpCode::CPUI_INT_ADD, &[amount, offset]);
                                let (op, output) = alignment_op(&mut fd, block, width, OpCode::CPUI_INT_SUB, &[base, amount]);
                                let allocation = Allocation { op, output, amount, scale: 1, base, before: base };
                                let residue = (base_offset - amount_offset) as u64 & (align - 1);
                                assert_eq!(allocation_alignment(&fd, &allocation), Some((align, residue)));
                                let graph = fd.bblocks_root_pub();
                                let end = scoped.then(|| fd.bblocks_mut().new_block_basic(graph));
                                let alignment = allocation_alignment(&fd, &allocation);
                                lower(&mut fd, allocation, amount, width, Reservation { count: shift + 32, shift }, alignment, end);
                                let allocate = fd.obank().iter_alive().find(|&id|
                                    fd.obank().get(id).unwrap().code() == OpCode::CPUI_CALLOTHER).unwrap();
                                let backing = fd.obank().get(allocate).unwrap().get_out().unwrap();
                                let size = fd.new_constant(width, align * 2);
                                let count = fd.obank().get(allocate).unwrap().get_in(1).unwrap();
                                let observe_mask = fd.new_constant(width, align - 1);
                                let (_, observed) = alignment_op(&mut fd, block, width, OpCode::CPUI_INT_AND, &[output, observe_mask]);
                                fd.touch_stack_alloc();
                                fd.settle_stack_alloc();
                                fd.calc_nz_mask();
                                for low in [0, 1, 15, 16, align - 2, align - 1] {
                                    let start = 0x2000 + low;
                                    let pointer = fd.new_constant(width, start);
                                    let values = |value| (if value == source { size } else if value == backing { pointer } else { value }, false);
                                    let result = literal_using(&fd, output, &mut 256, &values).unwrap();
                                    let observed = literal_using(&fd, observed, &mut 256, &values).unwrap();
                                    let count = literal_using(&fd, count, &mut 256, &values).unwrap();
                                    let native_size = (align * 2).wrapping_add(amount_offset as u64);
                                    assert_eq!(observed, residue, "width={width}, align={align}, shift={shift}");
                                    assert!(result >= start + shift);
                                    assert!(result + native_size + 32 <= start + count);
                                }
                                assert_eq!(fd.kuna_stack_scopes.len(), usize::from(scoped));
                            }
                        }
                    }
                }
            }
        }
    }
#[test]
fn masked_native_base_preserves_scaled_count_alignment() {
        for width in [2, 4, 8] {
            for (amount_align, scale) in [(1_u64, 64_i128), (16, 4), (16, 12), (32, 2)] {
                let (mut fd, input, block) = alignment_fd(width, true);
                let mask = fd.new_constant(width, calc_mask(width) & !63);
                let (_, base) = alignment_op(&mut fd, block, width, OpCode::CPUI_INT_AND, &[input, mask]);
                let source = fd.new_unique(width, None);
                let mask = fd.new_constant(width, amount_align.wrapping_neg() & calc_mask(width));
                let (_, amount) = alignment_op(&mut fd, block, width, OpCode::CPUI_INT_AND, &[source, mask]);
                let offset = fd.new_constant(width, 3);
                let (_, amount) = alignment_op(&mut fd, block, width, OpCode::CPUI_INT_ADD, &[amount, offset]);
                let (op, output) = alignment_op(&mut fd, block, width, OpCode::CPUI_INT_SUB, &[base, amount]);
                let allocation = Allocation { op, output, amount, scale, base, before: base };
                assert_eq!(allocation_alignment(&fd, &allocation), Some((64, (0_u64.wrapping_sub(3 * scale as u64)) & 63)));
            }
        }
    }
#[test]
fn rounded_count_does_not_invent_native_address_alignment() {
        let (mut fd, base, block) = alignment_fd(8, true);
        let source = fd.new_unique(8, None);
        let mask = fd.new_constant(8, !63);
        let (_, amount) = alignment_op(&mut fd, block, 8, OpCode::CPUI_INT_AND, &[source, mask]);
        let (op, output) = alignment_op(&mut fd, block, 8, OpCode::CPUI_INT_SUB, &[base, amount]);
        let allocation = Allocation { op, output, amount, scale: 1, base, before: base };
        assert_eq!(allocation_alignment(&fd, &allocation), None);
        lower(&mut fd, allocation, amount, 8, Reservation { count: 32, shift: 8 }, None, None);
        let backing = fd.obank().iter_alive().find_map(|id| {
            let op = fd.obank().get(id).unwrap();
            (op.code() == OpCode::CPUI_CALLOTHER).then(|| op.get_out().unwrap())
        }).unwrap();
        let pointer = fd.new_constant(8, 0x2010);
        assert_eq!(literal_using(&fd, output, &mut 256, &|value|
            (if value == backing { pointer } else { value }, false)), Some(0x2018));
    }
#[test]
fn unrelated_integer_mask_does_not_align_an_allocation() {
        let mut fd = build_fd();
        let zero = fd.new_constant(4, 0);
        let op = fd.new_op(1, fd.get_address().clone());
        fd.op_set_opcode_code(op, OpCode::CPUI_COPY);
        fd.op_set_input(op, zero, 0).unwrap();
        let root = fd.new_unique_out(4, op).unwrap();
        fd.obank_mut().mark_alive(op);
        let offset = fd.new_constant(4, u32::MAX as u64);
        let add = fd.new_op(2, fd.get_address().clone());
        fd.op_set_opcode_code(add, OpCode::CPUI_INT_ADD);
        fd.op_set_input(add, root, 0).unwrap();
        fd.op_set_input(add, offset, 1).unwrap();
        let ptr = fd.new_unique_out(4, add).unwrap();
        fd.obank_mut().mark_alive(add);
        let space = fd.new_constant(4, 4);
        let load = fd.new_op(2, fd.get_address().clone());
        fd.op_set_opcode_code(load, OpCode::CPUI_LOAD);
        fd.op_set_input(load, space, 0).unwrap();
        fd.op_set_input(load, ptr, 1).unwrap();
        fd.new_unique_out(1, load).unwrap();
        fd.obank_mut().mark_alive(load);
        let mask = copied_mask(&mut fd, 4, 0xffffffc0, false);
        fd.obank_mut().mark_alive(mask);
        let reservations = reserves(
            &fd,
            &[Allocation { scale: 1, before: zero,
                op,
                output: root,
                amount: zero,
                base: zero,
            }],
            &[],
            &[],
            4,
        )
        .unwrap();
        assert_eq!((reservations[0].count, reservations[0].shift), (16, 16));
    }
#[test]
fn sequential_reservations_require_retained_storage() {
        for width in [4, 8] {
            let mut fd = build_fd();
            let ram = Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap());
            let base = fd.new_varnode(width, &Address::new(Rc::clone(&ram), 0x2000), None);
            let base = fd.set_input_varnode(base).unwrap();
            let first = fd.new_varnode(width, &Address::new(Rc::clone(&ram), 0x2010), None);
            let first = fd.set_input_varnode(first).unwrap();
            let second = fd.new_varnode(width, &Address::new(ram, 0x2020), None);
            let second = fd.set_input_varnode(second).unwrap();
            let root = fd.bblocks_root_pub();
            let block = fd.bblocks_mut().new_block_basic(root);
            let mut reservations = Vec::new();
            for amount in [first, second] {
                let op = fd.new_op(2, fd.get_address().clone());
                fd.op_set_opcode_code(op, OpCode::CPUI_INT_SUB);
                fd.op_set_input(op, base, 0).unwrap();
                fd.op_set_input(op, amount, 1).unwrap();
                fd.new_unique_out(width, op).unwrap();
                fd.op_insert_end(op, block);
                reservations.push(allocation(&fd, op).unwrap());
            }
            assert!(
                !sequential_reservations_survive(&fd, &reservations),
                "independent sizes from a restored base do not have C alloca lifetimes"
            );
            fd.op_set_input(reservations[1].op, reservations[0].output, 0)
                .unwrap();
            reservations[1] = allocation(&fd, reservations[1].op).unwrap();
            assert!(
                sequential_reservations_survive(&fd, &reservations),
                "genuine consecutive allocations retain the earlier backing"
            );
        }
    }
#[test]
fn bounds_transfers_preserve_operand_order_and_refresh_after_mutation() {
        for width in [4, 8] {
            let mut fd = build_fd();
            let ram = Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap());
            let input = fd.new_varnode(width, &Address::new(ram, 0x2000), None);
            let input = fd.set_input_varnode(input).unwrap();
            let root_block = fd.bblocks_root_pub();
            let block = fd.bblocks_mut().new_block_basic(root_block);
            let append = |fd: &mut Funcdata, code, inputs: &[VarnodeId]| {
                let op = fd.new_op(inputs.len() as int4, fd.get_address().clone());
                fd.op_set_opcode_code(op, code);
                for (slot, &value) in inputs.iter().enumerate() {
                    fd.op_set_input(op, value, slot as int4).unwrap();
                }
                let value = fd.new_unique_out(width, op).unwrap();
                fd.op_insert_end(op, block);
                (op, value)
            };
            let (seed, root) = append(&mut fd, OpCode::CPUI_COPY, &[input]);
            let four = fd.new_constant(width, 4);
            let (fold, folded) = append(&mut fd, OpCode::CPUI_COPY, &[four]);
            let (add, added) = append(&mut fd, OpCode::CPUI_INT_ADD, &[folded, root]);
            let (sub, subtracted) = append(&mut fd, OpCode::CPUI_INT_SUB, &[root, folded]);
            let (ptrsub, shifted) = append(&mut fd, OpCode::CPUI_PTRSUB, &[root, folded]);
            let two = fd.new_constant(width, 2);
            let (ptradd, indexed) =
                append(&mut fd, OpCode::CPUI_PTRADD, &[root, folded, two]);
            let mask = fd.new_constant(width, (-16_i64) as u64 & calc_mask(width));
            let (align, aligned) = append(&mut fd, OpCode::CPUI_INT_AND, &[mask, root]);
            let zero = fd.new_constant(width, 0);
            let (clear, cleared) = append(&mut fd, OpCode::CPUI_INT_AND, &[root, zero]);
            let (phi, joined) =
                append(&mut fd, OpCode::CPUI_MULTIEQUAL, &[added, subtracted]);
            let (dynamic, unknown) =
                append(&mut fd, OpCode::CPUI_INT_ADD, &[root, input]);
            let ops = [seed, fold, add, sub, ptrsub, ptradd, align, clear, phi, dynamic];
            let roots = BTreeMap::from([(root, 0)]);
            for amount in [4, 8] {
                let constant = fd.new_constant(width, amount);
                fd.op_set_input(fold, constant, 0).unwrap();
                let lower = propagate_bounds(&fd, &ops, &roots, &BTreeMap::new(), false)
                    .unwrap();
                let upper = propagate_bounds(&fd, &ops, &roots, &BTreeMap::new(), true)
                    .unwrap();
                for values in [&lower, &upper] {
                    assert_eq!(values[&root][&0], 0);
                    assert!(values[&folded].is_empty());
                    assert_eq!(values[&added][&0], amount as i128);
                    assert_eq!(values[&subtracted][&0], -(amount as i128));
                    assert_eq!(values[&shifted][&0], amount as i128);
                    assert_eq!(values[&indexed][&0], 2 * amount as i128);
                    assert_eq!(values[&cleared][&0], UNKNOWN);
                    assert_eq!(values[&unknown][&0], UNKNOWN);
                }
                assert_eq!(lower[&aligned][&0], -15);
                assert_eq!(upper[&aligned][&0], 0);
                assert_eq!(lower[&joined][&0], -(amount as i128));
                assert_eq!(upper[&joined][&0], amount as i128);
            }
        }
    }
#[test]
fn precomputed_bounds_keep_each_scalar_folds_node_budget() {
        let mut fd = build_fd();
        let ram = Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap());
        let input = fd.new_varnode(4, &Address::new(ram, 0x2000), None);
        let input = fd.set_input_varnode(input).unwrap();
        let root_block = fd.bblocks_root_pub();
        let block = fd.bblocks_mut().new_block_basic(root_block);
        let mut ops = Vec::new();
        let mut copy = |fd: &mut Funcdata, input| {
            let op = fd.new_op(1, fd.get_address().clone());
            fd.op_set_opcode_code(op, OpCode::CPUI_COPY);
            fd.op_set_input(op, input, 0).unwrap();
            let value = fd.new_unique_out(4, op).unwrap();
            fd.op_insert_end(op, block);
            ops.push(op);
            value
        };
        let root = copy(&mut fd, input);
        let mut constant = fd.new_constant(4, 4);
        for _ in 0..31 {
            constant = copy(&mut fd, constant);
        }
        let within_budget = constant;
        let beyond_budget = copy(&mut fd, within_budget);
        let mut outputs = Vec::new();
        for amount in [within_budget, beyond_budget] {
            let op = fd.new_op(2, fd.get_address().clone());
            fd.op_set_opcode_code(op, OpCode::CPUI_INT_ADD);
            fd.op_set_input(op, root, 0).unwrap();
            fd.op_set_input(op, amount, 1).unwrap();
            outputs.push(fd.new_unique_out(4, op).unwrap());
            fd.op_insert_end(op, block);
            ops.push(op);
        }
        let values = propagate_bounds(
            &fd, &ops, &BTreeMap::from([(root, 0)]), &BTreeMap::new(), false,
        ).unwrap();
        assert_eq!(values[&outputs[0]][&0], 4);
        assert_eq!(values[&outputs[1]][&0], UNKNOWN);
    }
#[test]
fn declined_reservation_does_not_rewrite_displaced_aliases() {
        let (mut fd,allocation,_,block)=scoped_lifetime_fixture(4,false);
        let base=allocation.base;
        let amount=allocation.amount;
        let mut aliases = Vec::new();
        for delta in [0x7fffffff_i64, -0x7fffffff] {
            let add = fd.new_op(2, fd.get_address().clone());
            fd.op_set_opcode_code(add, OpCode::CPUI_INT_ADD);
            fd.op_set_input(add, base, 0).unwrap();
            let offset = fd.new_constant(4, delta as u64 & calc_mask(4));
            fd.op_set_input(add, offset, 1).unwrap();
            let shifted_base = fd.new_unique_out(4, add).unwrap();
            fd.op_insert_end(add, block);
            let alias = fd.new_op(2, fd.get_address().clone());
            fd.op_set_opcode_code(alias, OpCode::CPUI_INT_SUB);
            fd.op_set_input(alias, shifted_base, 0).unwrap();
            fd.op_set_input(alias, amount, 1).unwrap();
            let pointer = fd.new_unique_out(4, alias).unwrap();
            fd.op_insert_end(alias, block);
            let load = fd.new_op(2, fd.get_address().clone());
            fd.op_set_opcode_code(load, OpCode::CPUI_LOAD);
            let space = fd.new_constant(4, 4);
            fd.op_set_input(load, space, 0).unwrap();
            fd.op_set_input(load, pointer, 1).unwrap();
            fd.new_unique_out(1, load).unwrap();
            fd.op_insert_end(load, block);
            aliases.push((alias, shifted_base));
        }
        assert_eq!(displaced_aliases(&fd, &[allocation], &[]).len(), 2);
        // Opposite offsets need at least 2^32 bytes after alignment, which
        // cannot be represented by the four-byte allocation count.
        let facts=FrameFacts::infer(&fd,vec![allocation]).unwrap();
        assert!(materialize_allocations(&mut fd, &facts, &[], 4, &[], &[], &[]).is_err());
        for (alias, shifted_base) in aliases {
            let op = fd.obank().get(alias).unwrap();
            assert_eq!(op.code(), OpCode::CPUI_INT_SUB);
            assert_eq!(op.get_in(0), Some(shifted_base));
            assert_eq!(op.get_in(1), Some(amount));
        }
    }
#[test]
fn native_adoption_allows_disjoint_fixed_frame_initialization() {
        for width in [4, 8] {
            for case in ["fixed_frame", "below_sp", "crosses_base", "crosses_entry", "global", "unknown", "different_value", "different_space", "narrow_spill", "byte_initializer", "call"] {
                let (mut fd, allocation, _, _) = scoped_lifetime_fixture(width, false);
                let adopt = fd.new_op(1, fd.get_address().clone());
                fd.op_set_opcode_code(adopt, OpCode::CPUI_COPY);
                fd.op_set_input(adopt, allocation.output, 0).unwrap();
                let sp = fd.vbank().get(allocation.output).unwrap().get_addr().clone();
                fd.new_varnode_out(width, &sp, adopt).unwrap();
                fd.op_insert_after(adopt, allocation.op);
                if case == "call" {
                    let call = fd.new_op(1, fd.get_address().clone());
                    fd.op_set_opcode_code(call, OpCode::CPUI_CALL);
                    let target = fd.new_constant(width, 0x8000);
                    fd.op_set_input(call, target, 0).unwrap();
                    fd.op_insert_before(call, adopt);
                } else {
                    let pointer = if case == "global" {
                        fd.new_constant(width, 0x8000)
                    } else if case == "unknown" {
                        let ram = Rc::clone(fd.get_arch().manage().get_default_code_space().unwrap());
                        let pointer = fd.new_varnode(width, &Address::new(ram, 0x3000), None);
                        fd.set_input_varnode(pointer).unwrap();
                        pointer
                    } else {
                        let stack = fd.get_arch().manage().get_stack_space().unwrap();
                        let input = fd.find_spacebase_input(&stack).unwrap();
                        let address = fd.new_op(2, fd.get_address().clone());
                        fd.op_set_opcode_code(address, OpCode::CPUI_INT_ADD);
                        fd.op_set_input(address, input, 0).unwrap();
                        let offset = match case {
                            "below_sp" => -80_i64,
                            "crosses_base" => -65,
                            "crosses_entry" => -1,
                            _ => -32,
                        };
                        let offset = fd.new_constant(width, offset as u64 & calc_mask(width));
                        fd.op_set_input(address, offset, 1).unwrap();
                        let pointer = fd.new_unique_out(width, address).unwrap();
                        fd.op_insert_before(address, adopt);
                        pointer
                    };
                    let data = if case == "byte_initializer" {
                        fd.new_constant(1, 0)
                    } else if case == "narrow_spill" {
                        fd.new_constant(width / 2, 7)
                    } else if case == "different_value" {
                        allocation.amount
                    } else {
                        allocation.output
                    };
                    let store = fd.new_op(3, fd.get_address().clone());
                    fd.op_set_opcode_code(store, OpCode::CPUI_STORE);
                    let space = fd.new_constant(4, if case == "different_space" { 5 } else { 4 });
                    fd.op_set_input(store, space, 0).unwrap();
                    fd.op_set_input(store, pointer, 1).unwrap();
                    fd.op_set_input(store, data, 2).unwrap();
                    fd.op_insert_before(store, adopt);
                }
                let disjoint = matches!(case, "fixed_frame" | "different_value" | "narrow_spill" | "byte_initializer");
                assert_eq!(reservation_point(&fd, &allocation, adopt), disjoint,
                    "width={width}, case={case}");
            }
        }
    }
#[test]
fn conditional_scope_stops_before_a_shared_continuation() {
        for width in [4, 8] {
            for split in [false, true] {
                for outside in [false, true] {
                    let (mut fd, allocation, mut end, _) = scoped_lifetime_fixture(width, false);
                    let graph = fd.bblocks_root_pub();
                    let first = fd.obank().get(allocation.op).unwrap().get_parent().unwrap();
                    let entry = fd.bblocks_mut().new_block_basic(graph);
                    let continuation = fd.bblocks_mut().new_block_basic(graph);
                    fd.bblocks_mut().block_mut(entry).set_index(4);
                    fd.bblocks_mut().block_mut(continuation).set_index(5);
                    fd.bblocks_mut().add_edge(entry, first);
                    fd.bblocks_mut().add_edge(entry, continuation);
                    fd.bblocks_mut().add_edge(end.boundary.unwrap(), continuation);
                    fd.bblocks_mut().set_start_block(graph, entry);
                    let mut roots = Vec::new();
                    fd.bblocks_mut().structure_loops(graph, &mut roots);
                    fd.bblocks_mut().calc_forward_dominator(graph, &roots);
                    let stack = fd.get_arch().manage().get_stack_space().unwrap();
                    let input = fd.find_spacebase_input(&stack).unwrap();
                    let write = fd.new_op(1, fd.get_address().clone());
                    fd.op_set_opcode_code(write, OpCode::CPUI_COPY);
                    fd.op_set_input(write, input, 0).unwrap();
                    let sp = fd.vbank().get(allocation.output).unwrap().get_addr().clone();
                    fd.new_varnode_out(width, &sp, write).unwrap();
                    let call = fd.new_op(1, fd.get_address().clone());
                    fd.op_set_opcode_code(call, OpCode::CPUI_CALL);
                    let target = fd.new_constant(width, 0x3000);
                    fd.op_set_input(call, target, 0).unwrap();
                    if outside {
                        fd.op_insert_end(write, continuation);
                        fd.op_insert_end(call, continuation);
                    } else {
                        fd.op_insert_before(write, end.restore);
                        fd.op_insert_before(call, end.restore);
                    }
                    if !split { end.boundary = None; }
                    assert_eq!(restoration_failure_before(&fd, &allocation, &[allocation.op], Some(&end), &FrameFacts::infer(&fd,vec![allocation]).unwrap()).is_none(),
                        outside, "width={width}, split={split}, outside={outside}");
                }
            }
        }
    }
#[test]
fn typed_pointer_offsets_preserve_modular_stack_relations() {
        for width in [4, 8] {
            for (scale, offset, expected) in [
                (Some(1), 0, Some(0)), (Some(1), 16, Some(16)),
                (Some(1), (-16_i64) as u64 & calc_mask(width), Some(-16)),
                (Some(1), calc_mask(width), Some(-1)), (Some(2), 0, None), (None, 0, None),
            ] {
                let (mut fd, allocation, end, _) = scoped_lifetime_fixture(width, false);
                let stack = Rc::clone(fd.get_arch().manage().get_stack_space().unwrap());
                let entry = fd.find_spacebase_input(&stack).unwrap();
                let point = fd.get_address().clone();
                let base = fd.new_op(2, point.clone());
                fd.op_set_opcode_code(base, OpCode::CPUI_PTRSUB);
                let distance = fd.new_constant(width, (-64_i64) as u64 & calc_mask(width));
                fd.op_set_all_input(base, &[entry, distance]).unwrap();
                let base_value = fd.new_unique_out(width, base).unwrap();
                fd.op_insert_before(base, end.restore);
                let negate = fd.new_op(1, point.clone());
                fd.op_set_opcode_code(negate, OpCode::CPUI_INT_2COMP);
                fd.op_set_input(negate, allocation.amount, 0).unwrap();
                let index = fd.new_unique_out(width, negate).unwrap();
                fd.op_insert_before(negate, end.restore);
                let add = fd.new_op(3, point.clone());
                fd.op_set_opcode_code(add, OpCode::CPUI_PTRADD);
                let factor = scale.map_or(allocation.amount, |scale| fd.new_constant(4, scale));
                fd.op_set_all_input(add, &[base_value, index, factor]).unwrap();
                let pointer = fd.new_unique_out(width, add).unwrap();
                fd.op_insert_before(add, end.restore);
                let member = fd.new_op(2, point);
                fd.op_set_opcode_code(member, OpCode::CPUI_PTRSUB);
                let distance = fd.new_constant(width, offset);
                fd.op_set_all_input(member, &[pointer, distance]).unwrap();
                let value = fd.new_unique_out(width, member).unwrap();
                fd.op_insert_before(member, end.restore);
                assert_eq!(native_stack_delta(&fd, value, allocation.output), expected,
                    "width={width}, scale={scale:?}, offset={offset:#x}");
            }
        }
    }

    fn restores_checkpoint(fd: &mut Funcdata, allocation: &Allocation, end: &LifetimeEnd) -> bool {
        let calls: Vec<_>=(0..fd.num_calls()).map(|index|fd.get_call_specs(index).get_op()).collect();
        for call in calls {capture_call_stack(fd,call);}
        FrameFacts::infer(fd,vec![*allocation]).is_some_and(|facts|
            facts.equations.delta(fd,end.value,allocation.base)==Some(0))
    }

    #[test]
fn native_origin_preserves_signed_offsets_and_requires_agreeing_paths() {
        for width in [4, 8] {
            let mut fd = build_fd();
            let ram = Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap());
            let entry = fd.new_varnode(width, &Address::new(ram, 0x4000), None);
            let entry = fd.set_input_varnode(entry).unwrap();
            let expression = |fd: &mut Funcdata, code, inputs: &[VarnodeId]| {
                let op = fd.new_op(inputs.len() as int4, fd.get_address().clone());
                fd.op_set_opcode_code(op, code);
                for (slot, &input) in inputs.iter().enumerate() {
                    fd.op_set_input(op, input, slot as int4).unwrap();
                }
                fd.new_unique_out(width, op).unwrap()
            };
            let delta = fd.new_constant(width, (-24i64 as u64) & calc_mask(width));
            let before = expression(&mut fd, OpCode::CPUI_INT_ADD, &[entry, delta]);
            let copied = expression(&mut fd, OpCode::CPUI_COPY, &[before]);
            let joined = expression(&mut fd, OpCode::CPUI_MULTIEQUAL, &[before, copied]);
            assert_eq!(native_stack_offset(&fd, joined, entry, &mut 256), Some(-24));
            let mixed = expression(&mut fd, OpCode::CPUI_MULTIEQUAL, &[before, entry]);
            assert_eq!(native_stack_offset(&fd, mixed, entry, &mut 256), None);
        }
    }

    #[test]
fn native_origin_does_not_guess_loaded_or_cyclic_frame_bases() {
        let mut fd = build_fd();
        let ram = Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap());
        let entry = fd.new_varnode(8, &Address::new(Rc::clone(&ram), 0x4000), None);
        let entry = fd.set_input_varnode(entry).unwrap();
        let load = fd.new_op(2, fd.get_address().clone());
        fd.op_set_opcode_code(load, OpCode::CPUI_LOAD);
        let space = fd.new_constant(8, ram.get_index() as u64);
        fd.op_set_input(load, space, 0).unwrap();
        fd.op_set_input(load, entry, 1).unwrap();
        let loaded = fd.new_unique_out(8, load).unwrap();
        assert_eq!(native_stack_offset(&fd, loaded, entry, &mut 256), None);
        let phi = fd.new_op(2, fd.get_address().clone());
        fd.op_set_opcode_code(phi, OpCode::CPUI_MULTIEQUAL);
        let cyclic = fd.new_unique_out(8, phi).unwrap();
        fd.op_set_input(phi, entry, 0).unwrap();
        fd.op_set_input(phi, cyclic, 1).unwrap();
        assert_eq!(native_stack_offset(&fd, cyclic, entry, &mut 256), None);
    }

    #[test]
fn allocation_header_data_does_not_invent_a_native_frame_link() {
        for width in [4, 8] {
            for kind in ["data", "input", "free_input", "entry_copy", "allocation"] {
                let (mut fd, allocation, end, _) = scoped_lifetime_fixture(width, false);
                let stack = fd.get_arch().manage().get_stack_space().unwrap();
                let input = fd.find_spacebase_input(&stack).unwrap();
                let space = stack.get_contain().unwrap().get_index() as u64;
                let store = fd.new_op(3, fd.get_address().clone());
                fd.op_set_opcode_code(store, OpCode::CPUI_STORE);
                let space = fd.new_constant(4, space);
                let data = match kind {
                    "input" => input,
                    "entry_copy" => {
                        let copy = fd.new_op(2, fd.get_address().clone());
                        fd.op_set_opcode_code(copy, OpCode::CPUI_INT_OR);
                        fd.op_set_all_input(copy, &[allocation.base, allocation.base]).unwrap();
                        let copied = fd.new_unique_out(width, copy).unwrap();
                        fd.op_insert_before(copy, end.restore);
                        let add = fd.new_op(2, fd.get_address().clone());
                        fd.op_set_opcode_code(add, OpCode::CPUI_INT_ADD);
                        let frame = fd.new_constant(width, 64);
                        fd.op_set_all_input(add, &[copied, frame]).unwrap();
                        let value = fd.new_unique_out(width, add).unwrap();
                        fd.op_insert_before(add, end.restore);
                        value
                    }
                    "allocation" => allocation.output,
                    "free_input" => {
                        let address = fd.vbank().get(input).unwrap().get_addr().clone();
                        fd.new_varnode(width, &address, None)
                    }
                    _ => fd.new_constant(width, 7),
                };
                fd.op_set_all_input(store, &[space, allocation.output, data]).unwrap();
                fd.op_insert_before(store, end.restore);
                assert_eq!(native_frame_links(&fd,&FrameFacts::infer(&fd,vec![allocation]).unwrap(),width,&[]).is_err(),
                    matches!(kind, "input" | "free_input" | "entry_copy"),
                    "width={width}, kind={kind}");
            }
        }
    }

    #[test]
fn saved_checkpoint_distinguishes_its_address_from_adjacent_objects() {
        for width in [4, 8] {
            for delta in [-33, -32, -32 + width - 1, -32 + width] {
                let (mut fd, allocation, end, block) = scoped_lifetime_fixture(width, false);
                let stack = fd.get_arch().manage().get_stack_space().unwrap();
                let address = Address::new(Rc::clone(&stack), (-32_i64) as u64 & calc_mask(width));
                let input = fd.find_spacebase_input(&stack).unwrap();
                let alias = fd.new_op(2, fd.get_address().clone());
                fd.op_set_opcode_code(alias, OpCode::CPUI_INT_ADD);
                fd.op_set_input(alias, input, 0).unwrap();
                let offset = fd.new_constant(width, delta as i64 as u64 & calc_mask(width));
                fd.op_set_input(alias, offset, 1).unwrap();
                let pointer = fd.new_unique_out(width, alias).unwrap();
                fd.op_insert_end(alias, block);
                let store = fd.new_op(3, fd.get_address().clone());
                fd.op_set_opcode_code(store, OpCode::CPUI_STORE);
                let space = fd.new_constant(4, 4);
                let global = fd.new_constant(width, 0x8000);
                fd.op_set_input(store, space, 0).unwrap();
                fd.op_set_input(store, global, 1).unwrap();
                fd.op_set_input(store, pointer, 2).unwrap();
                fd.op_insert_end(store, block);
                assert_eq!(private_frame_slot(&fd, -32, width, FrameRead::Operation(end.restore), None, Some(allocation.base), &mut 65536, &mut SavedFrameQuery::default()).is_some(),
                    delta < -32 || delta >= -32 + width, "width={width}, alias={delta}");
            }
        }
    }

    #[test]
fn restored_phi_requires_a_checkpoint_seed_on_every_entry() {
        for width in [4, 8] {
            for known in [true, false] {
                let (mut fd, allocation, end, block) = scoped_lifetime_fixture(width, false);
                let phi = fd.new_op(2, fd.get_address().clone());
                fd.op_set_opcode_code(phi, OpCode::CPUI_MULTIEQUAL);
                let output = fd.new_unique_out(width, phi).unwrap();
                let entry = if known { allocation.base } else { allocation.amount };
                fd.op_set_input(phi, entry, 0).unwrap();
                fd.op_set_input(phi, output, 1).unwrap();
                fd.op_insert_begin(phi, block);
                fd.op_set_input(end.restore, output, 0).unwrap();
                assert_eq!(restores_checkpoint(&mut fd, &allocation, &end), known);
            }
        }
    }

    #[test]
fn saved_checkpoint_rejects_unbounded_indexed_frame_stores() {
        for width in [4, 8] {
            for bytes in [1, width] {
                let (mut fd, allocation, end, block) = scoped_lifetime_fixture(width, false);
                let stack = fd.get_arch().manage().get_stack_space().unwrap();
                let input = fd.find_spacebase_input(&stack).unwrap();
                let address = Address::new(Rc::clone(&stack), (-32_i64) as u64 & calc_mask(width));
                let base = fd.new_op(2, fd.get_address().clone());
                fd.op_set_opcode_code(base, OpCode::CPUI_INT_ADD);
                fd.op_set_input(base, input, 0).unwrap();
                let offset = fd.new_constant(width, (-16_i64) as u64 & calc_mask(width));
                fd.op_set_input(base, offset, 1).unwrap();
                let fixed = fd.new_unique_out(width, base).unwrap();
                fd.op_insert_end(base, block);
                let indexed = fd.new_op(2, fd.get_address().clone());
                fd.op_set_opcode_code(indexed, OpCode::CPUI_INT_ADD);
                fd.op_set_input(indexed, fixed, 0).unwrap();
                fd.op_set_input(indexed, allocation.amount, 1).unwrap();
                let pointer = fd.new_unique_out(width, indexed).unwrap();
                fd.op_insert_end(indexed, block);
                let store = fd.new_op(3, fd.get_address().clone());
                fd.op_set_opcode_code(store, OpCode::CPUI_STORE);
                let space = fd.new_constant(4, 4);
                let value = fd.new_constant(bytes, 7);
                fd.op_set_input(store, space, 0).unwrap();
                fd.op_set_input(store, pointer, 1).unwrap();
                fd.op_set_input(store, value, 2).unwrap();
                fd.op_insert_end(store, block);
                assert!(private_frame_slot(&fd, -32, width, FrameRead::Operation(end.restore), None, Some(allocation.base), &mut 65536, &mut SavedFrameQuery::default()).is_none(),
                    "width={width}, store_bytes={bytes}");
            }
        }
    }

    #[test]
fn checkpoint_calls_use_final_abi_storage_not_argument_values() {
        for width in [4, 8] {
            for variadic in [false, true] {
                for (offset, expected) in [(None, true), (Some(16), true), (Some(32), false)] {
                    let (mut fd, allocation, end, _, index, _) = checkpoint_call_fixture(width, variadic, false);
                    if let Some(offset) = offset {
                        let stack = Rc::clone(fd.get_arch().manage().get_stack_space().unwrap());
                        fd.get_call_specs_mut(index).set_final_input_storage(
                            vec![(Address::new(stack, offset), width)]);
                    }
                    assert_eq!(restores_checkpoint(&mut fd, &allocation, &end), expected,
                        "width={width}, variadic={variadic}, outgoing={offset:?}");
                }
                for (last, expected) in [(31, true), (32, false)] {
                    let (mut fd, allocation, end, _, index, _) = checkpoint_call_fixture(width, variadic, false);
                    let stack = Rc::clone(fd.get_arch().manage().get_stack_space().unwrap());
                    let mut model = fd.get_call_specs(index).proto().model().as_ref().clone();
                    let mut locals = kuna_base::address::RangeList::new();
                    locals.insert_range(stack, 0, last);
                    model.set_local_range(locals);
                    fd.get_call_specs_mut(index).proto_mut().set_model(Some(Rc::new(model)));
                    assert_eq!(restores_checkpoint(&mut fd, &allocation, &end), expected,
                        "width={width}, variadic={variadic}, home last={last}");
                }
            }
        }
    }

    #[test]
fn checkpoint_slot_can_be_reused_outside_its_live_version() {
        for width in [4, 8] {
            for before in [false, true] {
                let (mut fd, allocation, end, _, _, saved) = checkpoint_call_fixture(width, false, false);
                let stack = fd.get_arch().manage().get_stack_space().unwrap();
                let input = fd.find_spacebase_input(&stack).unwrap();
                let address = fd.new_op(2, fd.get_address().clone());
                fd.op_set_opcode_code(address, OpCode::CPUI_INT_ADD);
                fd.op_set_input(address, input, 0).unwrap();
                let offset = fd.new_constant(width, (-32_i64) as u64 & calc_mask(width));
                fd.op_set_input(address, offset, 1).unwrap();
                let pointer = fd.new_unique_out(width, address).unwrap();
                let store = fd.new_op(3, fd.get_address().clone());
                fd.op_set_opcode_code(store, OpCode::CPUI_STORE);
                let space = fd.new_constant(4, 4);
                let byte = fd.new_constant(1, 7);
                fd.op_set_input(store, space, 0).unwrap();
                fd.op_set_input(store, pointer, 1).unwrap();
                fd.op_set_input(store, byte, 2).unwrap();
                if before {
                    let save = fd.vbank().get(saved).unwrap().get_def().unwrap();
                    fd.op_insert_before(address, save);
                    fd.op_insert_after(store, address);
                } else {
                    let boundary = end.boundary.unwrap();
                    fd.op_insert_end(address, boundary);
                    fd.op_insert_end(store, boundary);
                }
                assert!(restores_checkpoint(&mut fd, &allocation, &end), "width={width}, before={before}");
            }
        }
    }

    #[test]
fn every_live_call_must_leave_the_checkpoint_outside_its_abi_frame() {
        for width in [4, 8] {
            for (last, expected) in [(31, true), (128, false)] {
                let (mut fd, allocation, end, _, index, _) = checkpoint_call_fixture(width, false, false);
                let call = fd.new_op(1, fd.get_address().clone());
                fd.op_set_opcode_code(call, OpCode::CPUI_CALL);
                let target = fd.new_constant(width, 0x4000);
                fd.op_set_input(call, target, 0).unwrap();
                fd.op_insert_after(call, allocation.op);
                let mut spec = fd.get_call_specs(index).clone_for_op(call);
                let mut model = spec.proto().model().as_ref().clone();
                let stack = Rc::clone(fd.get_arch().manage().get_stack_space().unwrap());
                let mut locals = kuna_base::address::RangeList::new();
                locals.insert_range(stack, 0, last);
                model.set_local_range(locals);
                spec.proto_mut().set_model(Some(Rc::new(model)));
                fd.push_call_specs(spec);
                assert_eq!(restores_checkpoint(&mut fd, &allocation, &end), expected,
                    "width={width}, other call home last={last}");
            }
        }
    }

    #[test]
fn an_undeclared_return_register_does_not_erase_positive_abi_areas() {
        use crate::dtype::type_class;
        use crate::fspec::ParamEntry;
        for width in [4, 8] {
            let (mut fd, _, _, _, index, _) = checkpoint_call_fixture(width, false, false);
            let stack = Rc::clone(fd.get_arch().manage().get_stack_space().unwrap());
            let mut model = fd.get_call_specs(index).proto().model().as_ref().clone();
            model.build_param_list("standard").unwrap();
            model.input_mut().push_entry(ParamEntry::seed(0, type_class::TYPECLASS_GENERAL,
                Rc::clone(&stack), 112, 3984, 1, width, 0, true, false, &[], fd.get_arch().manage()).unwrap());
            model.input_mut().finish_decode();
            let mut locals = kuna_base::address::RangeList::new();
            locals.insert_range(Rc::clone(&stack), 0, 111);
            model.set_local_range(locals);
            let mut params = kuna_base::address::RangeList::new();
            params.insert_range(stack, 112, 4095);
            model.set_param_range(params);
            fd.get_call_specs_mut(index).proto_mut().set_model(Some(Rc::new(model)));
            assert_eq!(call_frame_floor(&fd, fd.get_call_specs(index)), Some(112));
        }
    }

    #[test]
fn reassociated_checkpoint_seeds_still_require_slot_privacy() {
        for width in [4, 8] {
            for change in ["reassociated", "earlier_global_escape", "escaped_to_call"] {
                let (mut fd, allocation, end, _, index, saved) = checkpoint_call_fixture(width, false, false);
                let restored = fd.obank().get(end.restore).unwrap().get_in(0).unwrap();
                let marker = fd.vbank().get(restored).unwrap().get_def().unwrap();
                fd.op_set_input(marker, allocation.base, 0).unwrap();
                if change != "reassociated" {
                    let stack = Rc::clone(fd.get_arch().manage().get_stack_space().unwrap());
                    let entry = fd.find_spacebase_input(&stack).unwrap();
                    let point = fd.get_address().clone();
                    let pointer = fd.new_op(2, point.clone());
                    fd.op_set_opcode_code(pointer, OpCode::CPUI_INT_ADD);
                    let offset = fd.new_constant(width, fd.vbank().get(saved).unwrap().get_offset());
                    fd.op_set_all_input(pointer, &[entry, offset]).unwrap();
                    let pointer_value = fd.new_unique_out(width, pointer).unwrap();
                    fd.op_insert_before(pointer, allocation.op);
                    if change == "escaped_to_call" {
                        let call = fd.get_call_specs(index).get_op();
                        fd.op_set_input(call, pointer_value, 1).unwrap();
                    } else {
                        let publish = fd.new_op(3, point);
                        fd.op_set_opcode_code(publish, OpCode::CPUI_STORE);
                        let memory = stack.get_contain().unwrap().get_index();
                        let space = fd.new_constant(4, memory as u64);
                        let global = fd.new_constant(width, 0x9000);
                        fd.op_set_all_input(publish, &[space, global, pointer_value]).unwrap();
                        fd.op_insert_before(publish, allocation.op);
                        let load = fd.new_op(2, fd.get_address().clone());
                        fd.op_set_opcode_code(load, OpCode::CPUI_LOAD);
                        fd.op_set_all_input(load, &[space, global]).unwrap();
                        let alias = fd.new_unique_out(width, load).unwrap();
                        fd.op_insert_before(load, end.restore);
                        let write = fd.new_op(3, fd.get_address().clone());
                        fd.op_set_opcode_code(write, OpCode::CPUI_STORE);
                        let zero = fd.new_constant(width, 0);
                        fd.op_set_all_input(write, &[space, alias, zero]).unwrap();
                        fd.op_insert_before(write, end.restore);
                    }
                }
                assert_eq!(restores_checkpoint(&mut fd, &allocation, &end),
                    change == "reassociated", "width={width}, change={change}");
            }
        }
    }

    #[test]
fn conditional_scalar_merges_use_predecessor_dominance() {
        for width in [4, 8] {
            for kind in ["constant_else", "unknown_else", "wrong_edge"] {
                let (mut fd, allocation, end, _) = scoped_lifetime_fixture(width, false);
                let graph = fd.bblocks_root_pub();
                let first = fd.obank().get(allocation.op).unwrap().get_parent().unwrap();
                let entry = fd.bblocks_mut().new_block_basic(graph);
                let merge = fd.bblocks_mut().new_block_basic(graph);
                fd.bblocks_mut().block_mut(entry).set_index(4);
                fd.bblocks_mut().block_mut(merge).set_index(5);
                fd.bblocks_mut().add_edge(entry, first);
                fd.bblocks_mut().add_edge(entry, merge);
                fd.bblocks_mut().add_edge(end.boundary.unwrap(), merge);
                fd.bblocks_mut().set_start_block(graph, entry);
                let mut roots = Vec::new();
                fd.bblocks_mut().structure_loops(graph, &mut roots);
                fd.bblocks_mut().calc_forward_dominator(graph, &roots);
                let point = fd.get_address().clone();
                let call = fd.new_op(2, point.clone());
                fd.op_set_opcode_code(call, OpCode::CPUI_CALL);
                let target = fd.new_constant(width, 0x8000);
                fd.op_set_all_input(call, &[target, allocation.output]).unwrap();
                let result = fd.new_unique_out(width, call).unwrap();
                fd.op_insert_before(call, end.restore);
                let mut spec = crate::fspec::FuncCallSpecs::new(call, Address::new_invalid());
                spec.proto_mut().attach_internal_store(Rc::new(Datatype::new(width, type_metatype::TYPE_UINT)));
                spec.proto_mut().set_output_lock(true);
                fd.push_call_specs(spec);
                let phi = fd.new_op(2, point);
                fd.op_set_opcode_code(phi, OpCode::CPUI_MULTIEQUAL);
                let other = if kind == "unknown_else" { allocation.amount }
                    else { fd.new_constant(width, 19) };
                let mut inputs = [0, 1].map(|slot|
                    if fd.bblocks_ref().block(merge).get_in(slot) == entry { other } else { result });
                if kind == "wrong_edge" { inputs.swap(0, 1); }
                fd.op_set_all_input(phi, &inputs).unwrap();
                fd.new_unique_out(width, phi).unwrap();
                fd.op_insert_begin(phi, merge);
                assert!(!dominates_op(&fd, allocation.op, phi));
                let captures = scope_captures(&fd, &FrameFacts::infer(&fd,vec![allocation]).unwrap(), &[], &[Some(end)], &[], &[]);
                if kind == "wrong_edge" {
                    assert!(captures.is_err(), "a capture cannot be evaluated on a predecessor it does not dominate");
                } else {
                    let captures = captures.unwrap();
                    assert!(captures.iter().any(|capture| capture.source == result
                        && capture.uses.iter().any(|&(consumer, _)| consumer == phi)),
                        "the scoped producer must be saved on its own incoming edge: width={width}, kind={kind}");
                }
            }
        }
    }

    #[test]
fn reassociated_scope_addresses_include_their_integer_callees() {
        for width in [4, 8] {
            for kind in ["callee_result", "direct_address", "outside_definition"] {
                let (mut fd, allocation, end, _) = scoped_lifetime_fixture(width, false);
                let point = fd.get_address().clone();
                let shifted = fd.new_op(2, point.clone());
                fd.op_set_opcode_code(shifted, OpCode::CPUI_INT_ADD);
                let one = fd.new_constant(width, 1);
                fd.op_set_all_input(shifted, &[allocation.base, one]).unwrap();
                let base = fd.new_unique_out(width, shifted).unwrap();
                fd.op_insert_before(shifted, end.restore);
                let recomputed = fd.new_op(2, point.clone());
                fd.op_set_opcode_code(recomputed, OpCode::CPUI_INT_SUB);
                fd.op_set_all_input(recomputed, &[base, allocation.amount]).unwrap();
                let address = fd.new_unique_out(width, recomputed).unwrap();
                if kind == "outside_definition" {
                    fd.op_insert_begin(recomputed, end.boundary.unwrap());
                } else { fd.op_insert_before(recomputed, end.restore); }
                let mut value = address;
                if kind == "callee_result" {
                    let call = fd.new_op(2, point.clone());
                    fd.op_set_opcode_code(call, OpCode::CPUI_CALL);
                    let target = fd.new_constant(width, 0x8000);
                    fd.op_set_all_input(call, &[target, address]).unwrap();
                    value = fd.new_unique_out(width, call).unwrap();
                    fd.op_insert_before(call, end.restore);
                    let mut spec = crate::fspec::FuncCallSpecs::new(call, Address::new_invalid());
                    spec.proto_mut().attach_internal_store(Rc::new(Datatype::new(width, type_metatype::TYPE_UINT)));
                    spec.proto_mut().set_output_lock(true);
                    fd.push_call_specs(spec);
                }
                let consumer = fd.new_op(2, point);
                fd.op_set_opcode_code(consumer, OpCode::CPUI_CALL);
                let target = fd.new_constant(width, 0x9000);
                fd.op_set_all_input(consumer, &[target, value]).unwrap();
                fd.op_insert_end(consumer, end.boundary.unwrap());
                let displaced = displaced_aliases(&fd, &[allocation], &[]);
                let planned = displaced.iter().find(|&&(id, _, _)| id == recomputed).unwrap();
                assert_eq!((planned.1, planned.2), (allocation.output, 1));
                let captures = scope_captures(&fd, &FrameFacts::infer(&fd,vec![allocation]).unwrap(), &[], &[Some(end)], &displaced, &[]).unwrap();
                let (source, user, slot) = match kind {
                    "callee_result" => (value, consumer, 1),
                    "outside_definition" => (allocation.output, recomputed, 0),
                    _ => (value, consumer, 1),
                };
                assert!(captures.iter().any(|capture| capture.source == source
                    && capture.uses.contains(&(user, slot))), "width={width}, kind={kind}");
            }
        }
    }

    #[test]
    fn opaque_loaded_arguments_do_not_hide_an_explicit_checkpoint_escape() {
        for width in [4, 8] {
            for kind in ["scalar", "pointer", "encoded_pointer"] {
                let expose = kind != "scalar";
                let (mut fd, allocation, end, _, index, _) = checkpoint_call_fixture(width, false, false);
                let stack = fd.get_arch().manage().get_stack_space().unwrap();
                let entry = fd.find_spacebase_input(&stack).unwrap();
                let call = fd.get_call_specs(index).get_op();
                let address = |fd: &mut Funcdata, offset: i64| {
                    let op = fd.new_op(2, fd.get_address().clone());
                    fd.op_set_opcode_code(op, OpCode::CPUI_INT_ADD);
                    let offset = fd.new_constant(width, offset as u64 & calc_mask(width));
                    fd.op_set_all_input(op, &[entry, offset]).unwrap();
                    let value = fd.new_unique_out(width, op).unwrap();
                    fd.op_insert_before(op, call);
                    value
                };
                let temporary = address(&mut fd, -48);
                let mut payload = if expose { address(&mut fd, -32) } else { allocation.amount };
                if kind == "encoded_pointer" {
                    let narrow = fd.new_op(2, fd.get_address().clone());
                    fd.op_set_opcode_code(narrow, OpCode::CPUI_SUBPIECE);
                    let zero = fd.new_constant(4, 0);
                    fd.op_set_all_input(narrow, &[payload, zero]).unwrap();
                    payload = fd.new_unique_out(1, narrow).unwrap();
                    fd.op_insert_before(narrow, call);
                }
                let store = fd.new_op(3, fd.get_address().clone());
                fd.op_set_opcode_code(store, OpCode::CPUI_STORE);
                let memory = fd.new_constant(4, 4);
                fd.op_set_all_input(store, &[memory, temporary, payload]).unwrap();
                fd.op_insert_before(store, call);
                let load = fd.new_op(2, fd.get_address().clone());
                fd.op_set_opcode_code(load, OpCode::CPUI_LOAD);
                fd.op_set_all_input(load, &[memory, temporary]).unwrap();
                let argument = fd.new_unique_out(width, load).unwrap();
                fd.op_insert_before(load, call);
                fd.op_set_input(call, argument, 1).unwrap();
                if kind != "encoded_pointer" {
                    assert_eq!(frame_pointer_reaches(&fd, argument, -32, width, &mut 65536,
                        &mut SavedFrameQuery::default()), expose,
                        "width={width}, loaded payload={kind}");
                }
                assert_eq!(private_frame_slot(&fd, -32, width, FrameRead::Operation(end.restore), None, Some(allocation.base),
                    &mut 65536, &mut SavedFrameQuery::default()).is_some(), !expose,
                    "width={width}, checkpoint payload={kind}");
            }
        }
    }


    #[test]
    fn raw_scalar_spills_use_value_width_not_native_address_width() {
        for width in [4, 8] {
            for size in [1, 2, 4, 8] {
                for corrupt in [false, true] {
                    let (mut fd, _, _, _, index, _) = checkpoint_call_fixture(width, false, false);
                    let stack = fd.get_arch().manage().get_stack_space().unwrap();
                    let entry = fd.find_spacebase_input(&stack).unwrap();
                    let call = fd.get_call_specs(index).get_op();
                    let address = fd.new_op(2, fd.get_address().clone());
                    fd.op_set_opcode_code(address, OpCode::CPUI_INT_ADD);
                    let offset = fd.new_constant(width, (-48_i64) as u64 & calc_mask(width));
                    fd.op_set_all_input(address, &[entry, offset]).unwrap();
                    let pointer = fd.new_unique_out(width, address).unwrap();
                    fd.op_insert_before(address, call);
                    let memory = fd.new_constant(4, 4);
                    let payload = fd.new_constant(size, 0x5a);
                    let store = fd.new_op(3, fd.get_address().clone());
                    fd.op_set_opcode_code(store, OpCode::CPUI_STORE);
                    fd.op_set_all_input(store, &[memory, pointer, payload]).unwrap();
                    fd.op_insert_before(store, call);
                    let mut point = call;
                    let zero = fd.new_constant(1, 0);
                    if corrupt {
                        let write = fd.new_op(3, fd.get_address().clone());
                        fd.op_set_opcode_code(write, OpCode::CPUI_STORE);
                        fd.op_set_all_input(write, &[memory, pointer, zero]).unwrap();
                        fd.op_insert_after(write, point);
                        point = write;
                    }
                    let load = fd.new_op(2, fd.get_address().clone());
                    fd.op_set_opcode_code(load, OpCode::CPUI_LOAD);
                    fd.op_set_all_input(load, &[memory, pointer]).unwrap();
                    let value = fd.new_unique_out(size, load).unwrap();
                    fd.op_insert_after(load, point);
                    let actual = saved_stack_value(&fd, value, &mut 65536, &mut SavedFrameQuery::default()).and_then(|saved| saved.value);
                    if corrupt && size > 1 {
                        assert!(actual.is_none(), "width={width}, size={size}: partial overwrite is not the original value");
                    } else {
                        let expected = if corrupt { zero } else { payload };
                        assert!(actual.is_some_and(|actual| number(&fd, actual) == number(&fd, expected)),
                            "width={width}, size={size}, overwritten={corrupt}");
                    }
                }
            }
        }
    }


    #[test]
fn scoped_scalar_liveouts_preserve_actual_integer_producers() {
        for width in [4, 8] {
            for (kind, size, meta, locked, expected) in [
                ("narrow", 1, type_metatype::TYPE_INT, true, true),
                ("signed", 4, type_metatype::TYPE_INT, true, true),
                ("unsigned", 8, type_metatype::TYPE_UINT, true, true),
                ("sum", 4, type_metatype::TYPE_UINT, true, true),
                ("mixed", width, type_metatype::TYPE_UINT, true, true),
                ("unlocked", 4, type_metatype::TYPE_UINT, false, true),
                ("float", 8, type_metatype::TYPE_FLOAT, true, true),
                ("wide", 16, type_metatype::TYPE_UINT, true, false),
            ] {
                let (mut fd, allocation, end, _) = scoped_lifetime_fixture(width, false);
                let point = fd.get_address().clone();
                let call = fd.new_op(2, point.clone());
                fd.op_set_opcode_code(call, OpCode::CPUI_CALL);
                let target = fd.new_constant(width, 0x8000);
                fd.op_set_all_input(call, &[target, allocation.output]).unwrap();
                let result = fd.new_unique_out(size, call).unwrap();
                fd.op_insert_before(call, end.restore);
                let mut spec = crate::fspec::FuncCallSpecs::new(call, Address::new_invalid());
                spec.proto_mut().attach_internal_store(Rc::new(Datatype::new(size, meta)));
                spec.proto_mut().set_output_lock(locked);
                fd.push_call_specs(spec);
                let mut value = result;
                if matches!(kind, "sum" | "mixed") {
                    let sum = fd.new_op(2, point.clone());
                    fd.op_set_opcode_code(sum, OpCode::CPUI_INT_ADD);
                    let other = if kind == "mixed" { allocation.output }
                        else { fd.new_constant(size, 19) };
                    fd.op_set_all_input(sum, &[result, other]).unwrap();
                    value = fd.new_unique_out(size, sum).unwrap();
                    fd.op_insert_before(sum, end.restore);
                }
                let consumer = fd.new_op(2, point);
                fd.op_set_opcode_code(consumer, OpCode::CPUI_CALL);
                let target = fd.new_constant(width, 0x9000);
                fd.op_set_all_input(consumer, &[target, value]).unwrap();
                fd.op_insert_end(consumer, end.boundary.unwrap());
                let captures = scope_captures(&fd, &FrameFacts::infer(&fd,vec![allocation]).unwrap(), &[], &[Some(end)], &[], &[]);
                assert_eq!(captures.is_ok(), expected, "width={width}, kind={kind}");
                if expected {
                    let captures = captures.unwrap();
                    let saved = captures.iter().find(|capture| capture.uses.contains(&(consumer, 1))).unwrap();
                    assert_eq!(saved.source, value, "the original scalar producer must be evaluated, not replayed");
                    assert_eq!((saved.ty.get_size(), saved.ty.get_metatype()), (size, meta),
                        "width={width}, kind={kind}");
                }
            }
        }
    }
    #[test]
    fn restored_siblings_keep_distinct_objects_when_native_addresses_match() {
        for width in [4, 8] {
            for displacement in [0, 1] {
                let (mut fd, first, first_end, _) = scoped_lifetime_fixture(width, false);
                let graph = fd.bblocks_root_pub();
                let second_block = first_end.boundary.unwrap();
                let restore_block = fd.bblocks_mut().new_block_basic(graph);
                let boundary = fd.bblocks_mut().new_block_basic(graph);
                fd.bblocks_mut().block_mut(restore_block).set_index(4);
                fd.bblocks_mut().block_mut(boundary).set_index(5);
                fd.bblocks_mut().add_edge(second_block, restore_block);
                fd.bblocks_mut().add_edge(restore_block, boundary);
                let ram = Rc::clone(fd.get_arch().manage().get_default_data_space().unwrap());
                let address = Address::new(Rc::clone(&ram), 0x1040);
                let mut amount = first.amount;
                if displacement != 0 {
                    let add = fd.new_op(2, address.clone());
                    fd.op_set_opcode_code(add, OpCode::CPUI_INT_ADD);
                    fd.op_set_input(add, amount, 0).unwrap();
                    let one = fd.new_constant(width, displacement);
                    fd.op_set_input(add, one, 1).unwrap();
                    amount = fd.new_unique_out(width, add).unwrap();
                    fd.op_insert_end(add, second_block);
                }
                let op = fd.new_op(2, address.clone());
                fd.op_set_opcode_code(op, OpCode::CPUI_INT_SUB);
                fd.op_set_input(op, first.base, 0).unwrap();
                fd.op_set_input(op, amount, 1).unwrap();
                let sp = fd.vbank().get(first.output).unwrap().get_addr().clone();
                let output = fd.new_varnode_out(width, &sp, op).unwrap();
                fd.op_insert_end(op, second_block);
                let second = Allocation { scale: 1, before: first.base, op, output, amount, base: first.base };
                let restore = fd.new_op(1, address.clone());
                fd.op_set_opcode_code(restore, OpCode::CPUI_COPY);
                fd.op_set_input(restore, first.base, 0).unwrap();
                let restored = fd.new_varnode_out(width, &sp, restore).unwrap();
                fd.op_insert_end(restore, restore_block);
                fd.kuna_stack_scope_holds.extend([output, restored]);
                for block in [second_block, boundary] {
                    let call = fd.new_op(1, address.clone());
                    fd.op_set_opcode_code(call, OpCode::CPUI_CALL);
                    let target = fd.new_constant(width, 0x9000);
                    fd.op_set_input(call, target, 0).unwrap();
                    fd.op_insert_end(call, block);
                }
                let mut recomputed = Vec::new();
                for (block, source_amount, expected) in [
                    (fd.obank().get(first_end.restore).unwrap().get_parent().unwrap(), first.amount, first.output),
                    (second_block, amount, second.output),
                ] {
                    let expression = fd.new_op(2, address.clone());
                    fd.op_set_opcode_code(expression, OpCode::CPUI_INT_SUB);
                    fd.op_set_all_input(expression, &[first.base, source_amount]).unwrap();
                    let value = fd.new_unique_out(width, expression).unwrap();
                    if block == second_block { fd.op_insert_after(expression, second.op); }
                    else { fd.op_insert_before(expression, first_end.restore); }
                    let use_value = fd.new_op(2, address.clone());
                    fd.op_set_opcode_code(use_value, OpCode::CPUI_CALL);
                    let target = fd.new_constant(width, 0xa000);
                    fd.op_set_all_input(use_value, &[target, value]).unwrap();
                    fd.op_insert_end(use_value, second_block);
                    recomputed.push((value, expected));
                }
                let entry = fd.obank().get(first.op).unwrap().get_parent().unwrap();
                fd.bblocks_mut().calc_forward_dominator(graph, &[entry]);
                let mut definitions = vec![first.op, second.op];
                definitions.sort_unstable();
                let (roots, aliases) = identify_aliases(&fd, vec![first, second], &definitions);
                assert_eq!(roots.iter().map(|root| root.op).collect::<Vec<_>>(), definitions,
                    "width={width}, displacement={displacement}");
                for (value, expected) in recomputed {
                    assert!(aliases.contains(&(value, expected)),
                        "address recomputations must retain their available native object");
                }
            }
        }
    }

    #[test]
    fn scoped_integer_loop_merges_keep_the_actual_accumulator() {
        for width in [4, 8] {
            for kind in ["constant_seed", "address_seed", "unknown_seed"] {
                let (mut fd, allocation, end, body) = scoped_lifetime_fixture(width, false);
                let point = fd.get_address().clone();
                let call = fd.new_op(2, point.clone());
                fd.op_set_opcode_code(call, OpCode::CPUI_CALL);
                let target = fd.new_constant(width, 0x8000);
                fd.op_set_all_input(call, &[target, allocation.output]).unwrap();
                let result = fd.new_unique_out(width, call).unwrap();
                fd.op_insert_end(call, body);
                let mut spec = crate::fspec::FuncCallSpecs::new(call, Address::new_invalid());
                spec.proto_mut().attach_internal_store(Rc::new(Datatype::new(width, type_metatype::TYPE_UINT)));
                spec.proto_mut().set_output_lock(true);
                fd.push_call_specs(spec);
                let phi = fd.new_op(2, point.clone());
                fd.op_set_opcode_code(phi, OpCode::CPUI_MULTIEQUAL);
                let accumulator = fd.new_unique_out(width, phi).unwrap();
                fd.op_insert_begin(phi, body);
                let sum = fd.new_op(2, point.clone());
                fd.op_set_opcode_code(sum, OpCode::CPUI_INT_ADD);
                fd.op_set_all_input(sum, &[accumulator, result]).unwrap();
                let next = fd.new_unique_out(width, sum).unwrap();
                fd.op_insert_end(sum, body);
                let seed = match kind {
                    "address_seed" => allocation.output,
                    "unknown_seed" => allocation.amount,
                    _ => fd.new_constant(width, 0),
                };
                fd.op_set_all_input(phi, &[seed, next]).unwrap();
                let consumer = fd.new_op(2, point);
                fd.op_set_opcode_code(consumer, OpCode::CPUI_CALL);
                let target = fd.new_constant(width, 0x9000);
                fd.op_set_all_input(consumer, &[target, accumulator]).unwrap();
                fd.op_insert_end(consumer, end.boundary.unwrap());
                let captures = scope_captures(&fd, &FrameFacts::infer(&fd,vec![allocation]).unwrap(), &[], &[Some(end)], &[], &[])
                    .unwrap_or_else(|note| panic!("width={width}, kind={kind}: {note}"));
                let saved = captures.iter().find(|capture| capture.uses.contains(&(consumer, 1))).unwrap();
                assert_eq!(saved.source, accumulator, "capture the changing accumulator, not one invariant arm");
                assert_eq!(saved.ty.get_size(), width);
                assert_eq!(saved.ty.get_metatype(), type_metatype::TYPE_UINT);
            }
        }
    }
    #[test]
    fn derived_scalar_expressions_cannot_inline_an_expired_backing() {
        for width in [4, 8] {
            for code in [OpCode::CPUI_SUBPIECE, OpCode::CPUI_INT_EQUAL, OpCode::CPUI_LOAD] {
                for outside in [false, true] {
                    let (mut fd, allocation, end, _) = scoped_lifetime_fixture(width, false);
                    let point = fd.get_address().clone();
                    let expression = fd.new_op(2, point.clone());
                    fd.op_set_opcode_code(expression, code);
                    let (first, second, size) = if code == OpCode::CPUI_LOAD {
                        let space = fd.get_arch().manage().get_stack_space().unwrap()
                            .get_contain().unwrap().get_index() as u64;
                        (fd.new_constant(4, space), allocation.output, width)
                    } else {
                        (allocation.output, fd.new_constant(width, 0),
                            if code == OpCode::CPUI_INT_EQUAL { 1 } else { width / 2 })
                    };
                    fd.op_set_all_input(expression, &[first, second]).unwrap();
                    let value = fd.new_unique_out(size, expression).unwrap();
                    fd.op_insert_before(expression, end.restore);
                    let call = fd.new_op(2, point);
                    fd.op_set_opcode_code(call, OpCode::CPUI_CALL);
                    let target = fd.new_constant(width, 0x8000);
                    fd.op_set_all_input(call, &[target, value]).unwrap();
                    if outside { fd.op_insert_end(call, end.boundary.unwrap()); }
                    else { fd.op_insert_before(call, end.restore); }
                    let captures = scope_captures(&fd, &FrameFacts::infer(&fd,vec![allocation]).unwrap(), &[], &[Some(end)], &[], &[]).unwrap();
                    if outside {
                        // Preserve the evaluated scalar; never defer its load,
                        // truncation or comparison until after the backing dies.
                        let saved = captures.iter().find(|capture| capture.uses.contains(&(call, 1))).unwrap();
                        assert_eq!(saved.source, value);
                        assert_eq!(saved.ty.get_size(), size);
                        assert_eq!(saved.ty.get_metatype(), type_metatype::TYPE_UINT);
                    } else {
                        assert!(!captures.iter().any(|capture| capture.uses.contains(&(call, 1))));
                    }
                }
            }
        }
    }
    #[test]
    fn outside_address_copies_do_not_become_scalar_phi_roots() {
        for width in [4, 8] {
            let (mut fd, allocation, end, _) = scoped_lifetime_fixture(width, false);
            let graph = fd.bblocks_root_pub();
            let first = fd.obank().get(allocation.op).unwrap().get_parent().unwrap();
            let left = fd.bblocks_mut().new_block_basic(graph);
            let right = fd.bblocks_mut().new_block_basic(graph);
            let merge = fd.bblocks_mut().new_block_basic(graph);
            for (index, block) in [(4, left), (5, right), (6, merge)] {
                fd.bblocks_mut().block_mut(block).set_index(index);
            }
            for (from, to) in [(end.boundary.unwrap(), left), (end.boundary.unwrap(), right),
                (left, merge), (right, merge)]
            {
                fd.bblocks_mut().add_edge(from, to);
            }
            fd.bblocks_mut().calc_forward_dominator(graph, &[first]);
            let point = fd.get_address().clone();
            let call = fd.new_op(2, point.clone());
            fd.op_set_opcode_code(call, OpCode::CPUI_CALL);
            let target = fd.new_constant(width, 0x8000);
            fd.op_set_all_input(call, &[target, allocation.output]).unwrap();
            let scalar = fd.new_unique_out(width, call).unwrap();
            fd.op_insert_before(call, end.restore);
            let mut spec = crate::fspec::FuncCallSpecs::new(call, Address::new_invalid());
            spec.proto_mut().attach_internal_store(Rc::new(Datatype::new(width, type_metatype::TYPE_UINT)));
            spec.proto_mut().set_output_lock(true);
            fd.push_call_specs(spec);
            let copy = fd.new_op(1, point.clone());
            fd.op_set_opcode_code(copy, OpCode::CPUI_COPY);
            fd.op_set_input(copy, allocation.output, 0).unwrap();
            let address = fd.new_unique_out(width, copy).unwrap();
            fd.op_insert_end(copy, left);
            let phi = fd.new_op(2, point.clone());
            fd.op_set_opcode_code(phi, OpCode::CPUI_MULTIEQUAL);
            fd.op_set_all_input(phi, &[address, scalar]).unwrap();
            let joined = fd.new_unique_out(width, phi).unwrap();
            fd.op_insert_begin(phi, merge);
            let consumer = fd.new_op(2, point);
            fd.op_set_opcode_code(consumer, OpCode::CPUI_CALL);
            let target = fd.new_constant(width, 0x9000);
            fd.op_set_all_input(consumer, &[target, joined]).unwrap();
            fd.op_insert_end(consumer, merge);
            let captures = scope_captures(&fd, &FrameFacts::infer(&fd,vec![allocation]).unwrap(), &[], &[Some(end)], &[], &[]).unwrap();
            assert!(!captures.iter().any(|capture| capture.source == address || capture.source == joined));
            assert!(captures.iter().any(|capture| capture.source == allocation.output
                && capture.uses.contains(&(phi, 0))));
            assert!(captures.iter().any(|capture| capture.source == scalar
                && capture.uses.contains(&(phi, 1))));
        }
    }
    #[test]
    fn an_unobserved_effect_marker_does_not_extend_an_address_lifetime() {
        for width in [4, 8] {
            for observed in [false, true] {
                let (mut fd, allocation, end, _) = scoped_lifetime_fixture(width, false);
                let after = end.boundary.unwrap();
                let point = fd.get_address().clone();
                let call = fd.new_op(1, point.clone());
                fd.op_set_opcode_code(call, OpCode::CPUI_CALL);
                let target = fd.new_constant(width, 0x8000);
                fd.op_set_input(call, target, 0).unwrap();
                fd.op_insert_end(call, after);
                let marker = fd.new_op(2, point.clone());
                fd.op_set_opcode_code(marker, OpCode::CPUI_INDIRECT);
                let effect = fd.new_varnode_iop(call);
                fd.op_set_all_input(marker, &[allocation.output, effect]).unwrap();
                let value = fd.new_unique_out(width, marker).unwrap();
                fd.op_insert_after(marker, call);
                if observed {
                    let consumer = fd.new_op(2, point);
                    fd.op_set_opcode_code(consumer, OpCode::CPUI_CALL);
                    let target = fd.new_constant(width, 0x9000);
                    fd.op_set_all_input(consumer, &[target, value]).unwrap();
                    fd.op_insert_end(consumer, after);
                    assert!(scope_captures(&fd, &FrameFacts::infer(&fd,vec![allocation]).unwrap(), &[], &[Some(end)], &[], &[]).is_err());
                } else {
                    assert!(scope_captures(&fd, &FrameFacts::infer(&fd,vec![allocation]).unwrap(), &[], &[Some(end)], &[], &[]).unwrap().is_empty());
                }
            }
        }
    }    #[test]
    fn a_later_reservation_is_not_an_alias_of_expired_scoped_storage() {
        for width in [4, 8] {
            for kind in ["reservation", "pointer", "epilogue"] {
                let (mut fd, allocation, end, _) = scoped_lifetime_fixture(width, false);
                let address = fd.get_address().clone();
                let mut amount = fd.new_constant(width, 16);
                let after = end.boundary.unwrap();
                if kind != "epilogue" {
                    let add = fd.new_op(2, address.clone());
                    fd.op_set_opcode_code(add, OpCode::CPUI_INT_ADD);
                    fd.op_set_input(add, allocation.amount, 0).unwrap();
                    fd.op_set_input(add, amount, 1).unwrap();
                    amount = fd.new_unique_out(width, add).unwrap();
                    fd.op_insert_end(add, after);
                }
                let op = fd.new_op(2, address);
                fd.op_set_opcode_code(op, if kind == "epilogue" { OpCode::CPUI_INT_ADD } else { OpCode::CPUI_INT_SUB });
                fd.op_set_input(op, allocation.base, 0).unwrap();
                fd.op_set_input(op, amount, 1).unwrap();
                if kind == "pointer" {
                    fd.new_unique_out(width, op).unwrap();
                } else {
                    let sp = fd.vbank().get(allocation.output).unwrap().get_addr().clone();
                    fd.new_varnode_out(width, &sp, op).unwrap();
                }
                fd.op_insert_end(op, after);
                let mut found = vec![allocation];
                let mut definitions = vec![allocation.op];
                if kind == "reservation" {
                    found.push(super::allocation(&fd, op).unwrap());
                    definitions.push(op);
                    definitions.sort_unstable();
                }
                let (roots, aliases) = identify_aliases(&fd, found, &definitions);
                assert_eq!(roots.len(), if kind == "reservation" { 2 } else { 1 },
                    "width={width}, kind={kind}");
                if kind == "pointer" {
                    // A non-reserving recomputation is an expired address,
                    // not a second object. Preserve it through the old root.
                    assert!(displaced_aliases(&fd, &roots, &aliases).contains(&(op, allocation.output, -16)));
                }
            }
        }
    }
    #[test]
    fn address_spills_do_not_guess_unresolved_memory_destinations() {
        for width in [4, 8] {
          for clobbered in [false, true] {
            let (mut fd, allocation, end, _) = scoped_lifetime_fixture(width, false);
            let ram = Rc::clone(fd.get_arch().manage().get_default_data_space().unwrap());
            let pointer = fd.new_varnode(width, &Address::new(ram, 0x4000), None);
            let pointer = fd.set_input_varnode(pointer).unwrap();
            let space = fd.get_arch().manage().get_stack_space().unwrap()
                .get_contain().unwrap().get_index() as u64;
            let space = fd.new_constant(4, space);
            let store = fd.new_op(3, fd.get_address().clone());
            fd.op_set_opcode_code(store, OpCode::CPUI_STORE);
            fd.op_set_all_input(store, &[space, pointer, allocation.output]).unwrap();
            fd.op_insert_before(store, end.restore);
            let after = end.boundary.unwrap();
            if clobbered {
                let ram = Rc::clone(fd.get_arch().manage().get_default_data_space().unwrap());
                let other = fd.new_varnode(width, &Address::new(ram, 0x5000), None);
                let other = fd.set_input_varnode(other).unwrap();
                let overwrite = fd.new_op(3, fd.get_address().clone());
                fd.op_set_opcode_code(overwrite, OpCode::CPUI_STORE);
                let zero = fd.new_constant(width, 0);
                fd.op_set_all_input(overwrite, &[space, other, zero]).unwrap();
                fd.op_insert_end(overwrite, after);
            }
            let load = fd.new_op(2, fd.get_address().clone());
            fd.op_set_opcode_code(load, OpCode::CPUI_LOAD);
            fd.op_set_all_input(load, &[space, pointer]).unwrap();
            let reloaded = fd.new_unique_out(width, load).unwrap();
            fd.op_insert_end(load, after);
            let consumer = fd.new_op(2, fd.get_address().clone());
            fd.op_set_opcode_code(consumer, OpCode::CPUI_CALL);
            let target = fd.new_constant(width, 0x9000);
            fd.op_set_all_input(consumer, &[target, reloaded]).unwrap();
            fd.op_insert_end(consumer, after);
            let captures = scope_captures(&fd, &FrameFacts::infer(&fd,vec![allocation]).unwrap(), &[], &[Some(end)], &[], &[]).unwrap();
            let saved = captures.iter().find(|capture| capture.uses.contains(&(consumer, 1)));
            if clobbered {
                // The second unknown destination may alias the first. Keep the
                // real memory read; do not invent its old pointer value.
                assert!(saved.is_none());
            } else {
                // Exact address identity plus an unchanged slot proves the
                // readback even though its absolute destination is unknown.
                assert_eq!(saved.unwrap().source, allocation.output);
            }
          }
        }
    }

    fn call_word_is_above_frame(fd: &Funcdata, call: OpId, first: i128) -> bool {
        let mut budget = 65536;
        call_stack_upper(fd, call, &mut budget, &mut SavedFrameQuery::default())
            .zip(call_frame_floor(fd, fd.get_call_specs(fd.get_call_specs_index(call).unwrap())))
            .and_then(|(upper, floor)| upper.checked_add(floor))
            .is_some_and(|limit| first >= limit)
    }
    #[test]
    fn captured_call_frames_count_the_return_header_once_at_the_word_boundary() {
        for width in [4, 8] {
            for (last, expected) in [(47 + width, true), (48 + width, false)] {
                let (mut fd, allocation, end, _, index, _) = checkpoint_call_fixture(width, false, true);
                let call = fd.get_call_specs(index).get_op();
                fd.kuna_stack_call_sp.remove(&call);
                let stack = Rc::clone(fd.get_arch().manage().get_stack_space().unwrap());
                let mut model = fd.get_call_specs(index).proto().model().as_ref().clone();
                let mut locals = kuna_base::address::RangeList::new();
                locals.insert_range(Rc::clone(&stack), 0, last as u64);
                model.set_local_range(locals);
                fd.get_call_specs_mut(index).proto_mut().set_model(Some(Rc::new(model)));
                fd.get_call_specs_mut(index).proto_mut().set_extra_pop(width);
                let reserve = fd.new_op(2, fd.get_address().clone());
                fd.op_set_opcode_code(reserve, OpCode::CPUI_INT_ADD);
                fd.op_set_input(reserve, allocation.output, 0).unwrap();
                let offset = fd.new_constant(width, (-(16_i64 + width as i64)) as u64 & calc_mask(width));
                fd.op_set_input(reserve, offset, 1).unwrap();
                let sp = fd.vbank().get(allocation.output).unwrap().get_addr().clone();
                let reserved = fd.new_varnode_out(width, &sp, reserve).unwrap();
                fd.op_insert_before(reserve, call);
                let mut specs = std::mem::replace(fd.get_call_specs_mut(index),
                    crate::fspec::FuncCallSpecs::new(call, Address::new_invalid()));
                specs.create_placeholder(&mut fd, &stack).unwrap();
                let pointer = specs.get_spacebase_relative(&fd).unwrap();
                let reference = fd.vbank().get(pointer).unwrap().get_def().unwrap();
                fd.op_set_input(reference, reserved, 0).unwrap();
                *fd.get_call_specs_mut(index) = specs;
                capture_call_stack(&mut fd, call).unwrap();
                let mut specs = std::mem::replace(fd.get_call_specs_mut(index),
                    crate::fspec::FuncCallSpecs::new(call, Address::new_invalid()));
                specs.abort_spacebase_relative(&mut fd);
                *fd.get_call_specs_mut(index) = specs;
                assert_eq!(restores_checkpoint(&mut fd, &allocation, &end), expected,
                    "width={width}, last={last}");
            }
        }
    }    #[test]
    fn symbolic_call_space_requires_a_storage_witness_not_an_argument_value() {
        for width in [4, 8] {
            for (probe,consumed,initialized) in [(false,false,false),(true,false,false),(true,true,false),
                (false,false,true),(true,false,true),(true,true,true)] {
                let (mut fd, allocation, end, _, index, _) = checkpoint_call_fixture(width, false, false);
                let call = fd.get_call_specs(index).get_op();
                fd.kuna_stack_call_sp.remove(&call);
                let mut model = fd.get_call_specs(index).proto().model().as_ref().clone();
                let stack = Rc::clone(fd.get_arch().manage().get_stack_space().unwrap());
                let mut locals = kuna_base::address::RangeList::new();
                locals.insert_range(Rc::clone(&stack), 0, 128);
                model.set_local_range(locals);
                fd.get_call_specs_mut(index).proto_mut().set_model(Some(Rc::new(model)));
                fd.get_call_specs_mut(index).set_final_input_storage(vec![(Address::new(Rc::clone(&stack), 8), width)]);
                let mut call_sp = allocation.output;
                if probe {
                    let reserve = fd.new_op(2, fd.get_address().clone());
                    fd.op_set_opcode_code(reserve, OpCode::CPUI_INT_ADD);
                    fd.op_set_input(reserve, allocation.output, 0).unwrap();
                    let offset = fd.new_constant(width, (-128_i64) as u64 & calc_mask(width));
                    fd.op_set_input(reserve, offset, 1).unwrap();
                    let sp = fd.vbank().get(allocation.output).unwrap().get_addr().clone();
                    call_sp = fd.new_varnode_out(width, &sp, reserve).unwrap();
                    fd.op_insert_before(reserve, call);
                }
                let address = fd.new_op(2, fd.get_address().clone());
                fd.op_set_opcode_code(address, OpCode::CPUI_INT_ADD);
                fd.op_set_input(address, allocation.output, 0).unwrap();
                let offset = fd.new_constant(width, (-120_i64) as u64 & calc_mask(width));
                fd.op_set_input(address, offset, 1).unwrap();
                let pointer = fd.new_unique_out(width, address).unwrap();
                fd.op_insert_before(address, call);
                let load = fd.new_op(2, fd.get_address().clone());
                fd.op_set_opcode_code(load, OpCode::CPUI_LOAD);
                let space = fd.new_constant(4, 4);
                fd.op_set_input(load, space, 0).unwrap();
                fd.op_set_input(load, pointer, 1).unwrap();
                // Define the scalar argument on every loop iteration; this
                // control checks the SP witness, not uninitialized ABI bytes.
                if initialized {
                    let store=fd.new_op(3,fd.get_address().clone());
                    fd.op_set_opcode_code(store,OpCode::CPUI_STORE);
                    fd.op_set_all_input(store,&[space,pointer,allocation.amount]).unwrap();
                    fd.op_insert_before(store,call);
                }
                let value = fd.new_unique_out(width, load).unwrap();
                fd.vbank_mut().get_mut(value).unwrap().set_spacebase_placeholder();
                fd.op_insert_before(load, call);
                fd.op_set_input(call, value, 1).unwrap();
                if probe {
                    let mut specs = std::mem::replace(fd.get_call_specs_mut(index),
                        crate::fspec::FuncCallSpecs::new(call, Address::new_invalid()));
                    specs.create_placeholder(&mut fd, &stack).unwrap();
                    let pointer = specs.get_spacebase_relative(&fd).unwrap();
                    let reference = fd.vbank().get(pointer).unwrap().get_def().unwrap();
                    fd.op_set_input(reference, call_sp, 0).unwrap();
                    *fd.get_call_specs_mut(index) = specs;
                    if consumed {
                        capture_call_stack(&mut fd, call).unwrap();
                        let mut specs = std::mem::replace(fd.get_call_specs_mut(index),
                            crate::fspec::FuncCallSpecs::new(call, Address::new_invalid()));
                        specs.abort_spacebase_relative(&mut fd);
                        *fd.get_call_specs_mut(index) = specs;
                    }
                }
                assert_eq!(restores_checkpoint(&mut fd, &allocation, &end),probe && initialized,
                    "width={width},probe={probe},consumed={consumed},initialized={initialized}");
            }
        }
    }    #[test]
    fn captured_call_frames_recheck_folded_cleanup_effects() {
        for width in [4, 8] {
            let (mut fd, allocation, _, _, index, saved) = checkpoint_call_fixture(width, false, false);
            let call = fd.get_call_specs(index).get_op();
                fd.kuna_stack_call_sp.remove(&call);
            let stack = Rc::clone(fd.get_arch().manage().get_stack_space().unwrap());
            let mut model = fd.get_call_specs(index).proto().model().as_ref().clone();
            let mut locals = kuna_base::address::RangeList::new();
            locals.insert_range(Rc::clone(&stack), 0, 128);
            model.set_local_range(locals);
            fd.get_call_specs_mut(index).proto_mut().set_model(Some(Rc::new(model)));
            let address = fd.get_address().clone();
            let sp = fd.vbank().get(allocation.output).unwrap().get_addr().clone();
            let reserve = fd.new_op(2, address.clone());
            fd.op_set_opcode_code(reserve, OpCode::CPUI_INT_ADD);
            fd.op_set_input(reserve, allocation.output, 0).unwrap();
            let offset = fd.new_constant(width, (-128_i64) as u64 & calc_mask(width));
            fd.op_set_input(reserve, offset, 1).unwrap();
            let reserved = fd.new_varnode_out(width, &sp, reserve).unwrap();
            fd.op_insert_before(reserve, call);
            let previous = fd.new_op(1, address.clone());
            fd.op_set_opcode_code(previous, OpCode::CPUI_CALL);
            let target = fd.new_constant(width, 0x4000);
            fd.op_set_input(previous, target, 0).unwrap();
            fd.op_insert_before(previous, call);
            let mut spec = crate::fspec::FuncCallSpecs::new(previous, Address::new_invalid());
            spec.proto_mut().set_extra_pop(0);
            let previous_index = fd.push_call_specs(spec);
            let indirect = fd.new_op(2, address);
            fd.op_set_opcode_code(indirect, OpCode::CPUI_INDIRECT);
            fd.op_set_input(indirect, reserved, 0).unwrap();
            let effect = fd.new_varnode_iop(previous);
            fd.op_set_input(indirect, effect, 1).unwrap();
            let after_call = fd.new_varnode_out(width, &sp, indirect).unwrap();
            fd.op_insert_after(indirect, previous);
            let mut specs = std::mem::replace(fd.get_call_specs_mut(index),
                crate::fspec::FuncCallSpecs::new(call, Address::new_invalid()));
            specs.create_placeholder(&mut fd, &stack).unwrap();
            let pointer = specs.get_spacebase_relative(&fd).unwrap();
            let reference = fd.vbank().get(pointer).unwrap().get_def().unwrap();
            fd.op_set_input(reference, after_call, 0).unwrap();
            *fd.get_call_specs_mut(index) = specs;
            capture_call_stack(&mut fd, call).unwrap();
            let mut specs = std::mem::replace(fd.get_call_specs_mut(index),
                crate::fspec::FuncCallSpecs::new(call, Address::new_invalid()));
            specs.abort_spacebase_relative(&mut fd);
            *fd.get_call_specs_mut(index) = specs;
            let first = sign_extend(fd.vbank().get(saved).unwrap().get_offset() as i64, width * 8 - 1) as i128;
            assert!(call_word_is_above_frame(&fd, call, first));
            // This cleanup raises the later call's SP and invalidates its old bound.
            fd.get_call_specs_mut(previous_index).proto_mut().set_extra_pop(64);
            assert!(!call_word_is_above_frame(&fd, call, first));
        }
    }    #[test]
    fn checkpoint_casts_require_native_width_preservation() {
        for width in [4, 8] {
            for stored_width in [1, 2, 4, 8].into_iter().filter(|&size| size <= width) {
                let (mut fd, allocation, end, body) = scoped_lifetime_fixture(width, false);
                let first = fd.new_op(1, fd.get_address().clone());
                fd.op_set_opcode_code(first, OpCode::CPUI_CAST);
                fd.op_set_input(first, allocation.base, 0).unwrap();
                let narrow = fd.new_unique_out(stored_width, first).unwrap();
                fd.op_insert_end(first, body);
                let second = fd.new_op(1, fd.get_address().clone());
                fd.op_set_opcode_code(second, OpCode::CPUI_CAST);
                fd.op_set_input(second, narrow, 0).unwrap();
                let widened = fd.new_unique_out(width, second).unwrap();
                fd.op_insert_end(second, body);
                fd.op_set_input(end.restore, widened, 0).unwrap();
                assert_eq!(native_stack_delta(&fd, widened, allocation.base),
                    (stored_width == width).then_some(0), "width={width}, stored={stored_width}");
                assert_eq!(proved_restore(&fd, &allocation, end.value).is_some(),
                    stored_width == width, "width={width}, stored={stored_width}");
            }
        }
    }

    #[test]
    fn loaded_frame_links_do_not_hide_reassociated_checkpoint_addresses() {
        for width in [4, 8] {
            for frame_link in [false, true] {
              for (aligned_load,initialized_target) in [(false,false),(true,false),(true,true)] {
                let (mut fd, allocation, end, _, index, _) = checkpoint_call_fixture(width, false, false);
                let call = fd.get_call_specs(index).get_op();
                let stack = Rc::clone(fd.get_arch().manage().get_stack_space().unwrap());
                let entry = fd.find_spacebase_input(&stack).unwrap();
                let address = fd.new_op(2, fd.get_address().clone());
                fd.op_set_opcode_code(address, OpCode::CPUI_INT_ADD);
                let offset = fd.new_constant(width, (-48_i64) as u64 & calc_mask(width));
                fd.op_set_all_input(address, &[entry, offset]).unwrap();
                let slot = fd.new_unique_out(width, address).unwrap();
                fd.op_insert_before(address, call);
                let target = if aligned_load {
                    let align = fd.new_op(2, fd.get_address().clone());
                    fd.op_set_opcode_code(align, OpCode::CPUI_INT_AND);
                    let mask = fd.new_constant(width, (!7_u64) & calc_mask(width));
                    fd.op_set_all_input(align, &[slot, mask]).unwrap();
                    let target = fd.new_unique_out(width, align).unwrap();
                    fd.op_insert_before(align, call);
                    target
                } else { slot };
                let memory = fd.new_constant(4, stack.get_contain().unwrap().get_index() as u64);
                let store = fd.new_op(3, fd.get_address().clone());
                fd.op_set_opcode_code(store, OpCode::CPUI_STORE);
                fd.op_set_all_input(store, &[memory, if initialized_target {target} else {slot}, if frame_link { entry } else { allocation.amount }]).unwrap();
                fd.op_insert_before(store, call);
                let load = fd.new_op(2, fd.get_address().clone());
                fd.op_set_opcode_code(load, OpCode::CPUI_LOAD);
                fd.op_set_all_input(load, &[memory, target]).unwrap();
                let value = fd.new_unique_out(width, load).unwrap();
                fd.op_insert_before(load, call);
                let reach = fd.new_op(2, fd.get_address().clone());
                fd.op_set_opcode_code(reach, OpCode::CPUI_INT_ADD);
                let offset = fd.new_constant(width, (-32_i64) as u64 & calc_mask(width));
                fd.op_set_all_input(reach, &[value, offset]).unwrap();
                let pointer = fd.new_unique_out(width, reach).unwrap();
                fd.op_insert_before(reach, call);
                fd.op_set_input(call, pointer, 1).unwrap();
                // Storing entry SP did not expose the checkpoint at -32.
                // Reloading it and adding -32 does: guard the actual use.
                assert_eq!(restores_checkpoint(&mut fd, &allocation, &end), !frame_link && (!aligned_load || initialized_target),
                    "width={width},frame_link={frame_link},aligned_load={aligned_load},initialized_target={initialized_target}");
            }
              }
        }
    }

    #[test]
    fn scoped_restore_keeps_expired_object_accesses_explicit() {
        for width in [4, 8] {
            for store in [false, true] {
                for location in ["fixed_above", "fixed_below", "recomputed", "unknown"] {
                    let (mut fd, allocation, end, _) = scoped_lifetime_fixture(width, false);
                    let boundary = end.boundary.unwrap();
                    let pointer = if location == "unknown" {
                        allocation.amount
                    } else {
                        let expression = fd.new_op(2, fd.get_address().clone());
                        fd.op_set_opcode_code(expression, if location == "recomputed" {
                            OpCode::CPUI_INT_SUB
                        } else {
                            OpCode::CPUI_INT_ADD
                        });
                        fd.op_set_input(expression, allocation.base, 0).unwrap();
                        let offset = match location {
                            "recomputed" => allocation.amount,
                            "fixed_below" => fd.new_constant(width, calc_mask(width)),
                            _ => fd.new_constant(width, 8),
                        };
                        fd.op_set_input(expression, offset, 1).unwrap();
                        let pointer = fd.new_unique_out(width, expression).unwrap();
                        fd.op_insert_end(expression, boundary);
                        pointer
                    };
                    let access = fd.new_op(if store { 3 } else { 2 }, fd.get_address().clone());
                    fd.op_set_opcode_code(access, if store { OpCode::CPUI_STORE } else { OpCode::CPUI_LOAD });
                    let space = fd.new_constant(4, 4);
                    fd.op_set_input(access, space, 0).unwrap();
                    fd.op_set_input(access, pointer, 1).unwrap();
                    if store {
                        let byte = fd.new_constant(1, 7);
                        fd.op_set_input(access, byte, 2).unwrap();
                    } else {
                        fd.new_unique_out(1, access).unwrap();
                    }
                    fd.op_insert_end(access, boundary);
                    let entry = fd.obank().get(allocation.op).unwrap().get_parent().unwrap();
                    let (roots, aliases) = identify_aliases(&fd, vec![allocation], &[allocation.op]);
                    let facts=FrameFacts::infer(&fd,roots).unwrap();
                    materialize_allocations(&mut fd, &facts, &aliases, width, &[allocation.op], &[], &[])
                        .unwrap_or_else(|note| panic!("width={width}, store={store}, location={location}: {note}"));
                    let graph = fd.bblocks_root_pub();
                    fd.bblocks_mut().calc_forward_dominator(graph, &[entry]);
                    if location == "fixed_above" {
                        assert!(fd.kuna_stack_scopes.is_empty());
                    } else {
                        let &(_, boundary) = fd.kuna_stack_scopes.first().unwrap();
                        let parent = fd.obank().get(access).unwrap().get_parent().unwrap();
                        assert!(boundary == parent || fd.bblocks_ref().dominates(boundary, Some(parent)),
                            "the expired access must remain after the source scope");
                    }
                    assert_eq!(fd.obank().get(access).unwrap().code(),
                        if store { OpCode::CPUI_STORE } else { OpCode::CPUI_LOAD });
                }
            }
        }
    }
    #[test]
    fn scoped_address_liveouts_require_direct_width_preserving_consumers() {
        for width in [4, 8] {
            for kind in ["direct", "copy", "narrow", "phi", "arithmetic"] {
                let (mut fd, allocation, end, _) = scoped_lifetime_fixture(width, false);
                let point = fd.get_address().clone();
                let address = fd.new_op(2, point.clone());
                fd.op_set_opcode_code(address, OpCode::CPUI_INT_ADD);
                let offset = fd.new_constant(width, 16);
                fd.op_set_all_input(address, &[allocation.output, offset]).unwrap();
                let pointer = fd.new_unique_out(width, address).unwrap();
                fd.op_insert_before(address, end.restore);
                let after = end.boundary.unwrap();
                let mut value = pointer;
                if kind != "direct" {
                    let op = fd.new_op(if matches!(kind, "copy" | "phi") { 1 } else { 2 }, point.clone());
                    let code = match kind {
                        "copy" => OpCode::CPUI_COPY,
                        "narrow" => OpCode::CPUI_SUBPIECE,
                        "phi" => OpCode::CPUI_MULTIEQUAL,
                        _ => OpCode::CPUI_INT_ADD,
                    };
                    fd.op_set_opcode_code(op, code);
                    fd.op_set_input(op, pointer, 0).unwrap();
                    if !matches!(kind, "copy" | "phi") {
                        let offset = fd.new_constant(width, if kind == "narrow" { 0 } else { 16 });
                        fd.op_set_input(op, offset, 1).unwrap();
                    }
                    value = fd.new_unique_out(if kind == "narrow" { width / 2 } else { width }, op).unwrap();
                    fd.op_insert_begin(op, after);
                }
                let call = fd.new_op(2, point);
                fd.op_set_opcode_code(call, OpCode::CPUI_CALL);
                let target = fd.new_constant(width, 0x8000);
                fd.op_set_all_input(call, &[target, value]).unwrap();
                fd.op_insert_end(call, after);
                let captures = scope_captures(&fd, &FrameFacts::infer(&fd,vec![allocation]).unwrap(), &[], &[Some(end)], &[], &[])
                    .unwrap_or_else(|note| panic!("width={width}, kind={kind}: {note}"));
                let saved = captures.iter().find(|capture| capture.source == pointer).unwrap();
                assert_eq!(saved.ty.get_size(), width, "capture before the consumer narrows the address");
                assert_eq!(saved.ty.get_metatype(), type_metatype::TYPE_UINT);
                let use_site = if kind == "direct" { (call, 1) }
                    else { (fd.vbank().get(value).unwrap().get_def().unwrap(), 0) };
                assert!(saved.uses.contains(&use_site));
            }
        }
    }
    #[test]
    fn private_return_spills_do_not_publish_saved_word_addresses() {
        for width in [4,8] {
            for mode in ["private","global","exposed","explicit","overlap","consumed"] {
                let (mut fd,allocation,end,body,index,_)=checkpoint_call_fixture(width,false,false);
                let stack=Rc::clone(fd.get_arch().manage().get_stack_space().unwrap());
                let memory=Rc::clone(stack.get_contain().unwrap());
                let entry=fd.find_spacebase_input(&stack).unwrap();
                let point=fd.get_address().clone();
                let call=fd.get_call_specs(index).get_op();
                let integer=Rc::new(crate::dtype::Datatype::new(width,type_metatype::TYPE_UINT));
                fd.get_call_specs_mut(index).proto_mut().attach_internal_store(integer);
                let returned=fd.new_unique_out(width,call).unwrap();
                let address=fd.new_op(2,point.clone());
                fd.op_set_opcode_code(address,OpCode::CPUI_INT_ADD);
                let offset=fd.new_constant(width,(-16_i64) as u64&calc_mask(width));
                fd.op_set_all_input(address,&[entry,offset]).unwrap();
                let local=fd.new_unique_out(width,address).unwrap();
                fd.op_insert_before(address,allocation.op);
                if mode=="exposed" {
                    let expose=fd.new_op(1,point.clone());
                    fd.op_set_opcode_code(expose,OpCode::CPUI_COPY);
                    fd.op_set_input(expose,local,0).unwrap();
                    let global=fd.new_varnode_out(width,&Address::new(Rc::clone(&memory),0x9000),expose).unwrap();
                    fd.vbank_mut().get_mut(global).unwrap().mark_mapped_addr_tied();
                    fd.op_insert_before(expose,allocation.op);
                }
                let target=if mode=="global" {fd.new_constant(width,0x9000)} else if mode=="overlap" {
                    let address=fd.new_op(2,point.clone());
                    fd.op_set_opcode_code(address,OpCode::CPUI_INT_ADD);
                    let offset=fd.new_constant(width,(-32_i64) as u64&calc_mask(width));
                    fd.op_set_all_input(address,&[entry,offset]).unwrap();
                    let pointer=fd.new_unique_out(width,address).unwrap();
                    fd.op_insert_before(address,allocation.op);pointer
                } else {local};
                let payload=if mode=="explicit" {
                    let address=fd.new_op(2,point.clone());
                    fd.op_set_opcode_code(address,OpCode::CPUI_INT_ADD);
                    let offset=fd.new_constant(width,(-32_i64) as u64&calc_mask(width));
                    fd.op_set_all_input(address,&[entry,offset]).unwrap();
                    let pointer=fd.new_unique_out(width,address).unwrap();
                    fd.op_insert_before(address,allocation.op);pointer
                } else {returned};
                let space=fd.new_constant(4,memory.get_index() as u64);
                let write=fd.new_op(3,point.clone());
                fd.op_set_opcode_code(write,OpCode::CPUI_STORE);
                fd.op_set_all_input(write,&[space,target,payload]).unwrap();
                fd.op_insert_end(write,body);
                if mode=="consumed" {
                    let expose=fd.new_op(1,point.clone());
                    fd.op_set_opcode_code(expose,OpCode::CPUI_COPY);
                    fd.op_set_input(expose,returned,0).unwrap();
                    let global=fd.new_varnode_out(width,&Address::new(Rc::clone(&memory),0x9000),expose).unwrap();
                    fd.vbank_mut().get_mut(global).unwrap().mark_mapped_addr_tied();
                    fd.op_insert_end(expose,body);
                }
                let saved_address=fd.new_op(2,point.clone());
                fd.op_set_opcode_code(saved_address,OpCode::CPUI_INT_ADD);
                let offset=fd.new_constant(width,(-32_i64) as u64&calc_mask(width));
                fd.op_set_all_input(saved_address,&[entry,offset]).unwrap();
                let saved_pointer=fd.new_unique_out(width,saved_address).unwrap();
                fd.op_insert_before(saved_address,allocation.op);
                let load=fd.new_op(2,point);
                fd.op_set_opcode_code(load,OpCode::CPUI_LOAD);
                fd.op_set_all_input(load,&[space,saved_pointer]).unwrap();
                let value=fd.new_unique_out(width,load).unwrap();
                fd.op_insert_end(load,body);
                fd.op_set_input(end.restore,value,0).unwrap();
                assert_eq!(restores_checkpoint(&mut fd,&allocation,&end),mode=="private",
                    "width={width}, mode={mode}");
            }
        }
    }

    #[test]
    fn checkpoint_liveness_release_preserves_readers_escapes_and_owned_holds() {
        for width in [4, 8] {
            for mode in ["unused", "direct", "direct_address", "data", "address", "partial", "held", "masked_disjoint", "masked_overlap", "reload", "reload_clobbered", "reload_before_save"] {
                let (mut fd, allocation, end, body, index, saved) = checkpoint_call_fixture(width, false, false);
                let version = fd.obank().get(end.restore).unwrap().get_in(0).unwrap();
                let address = fd.vbank().get(saved).unwrap().get_addr().clone();
                for id in [saved, version] { fd.vbank_mut().get_mut(id).unwrap().set_addr_force(); }
                // Equality substitution has consumed the restore's memory read.
                fd.op_set_input(end.restore, allocation.base, 0).unwrap();
                let stack = Rc::clone(fd.get_arch().manage().get_stack_space().unwrap());
                let point = fd.get_address().clone();
                if mode.starts_with("direct") {
                    let ram=Rc::clone(stack.get_contain().unwrap());
                    let target=fd.new_varnode(width,&Address::new(ram,0x3000),None);
                    let call=fd.get_call_specs(index).get_op();
                    fd.op_set_input(call,target,0).unwrap();
                }
                if mode == "data" {
                    let store = fd.new_op(3, point.clone());
                    fd.op_set_opcode_code(store, OpCode::CPUI_STORE);
                    let memory = fd.new_constant(4, stack.get_contain().unwrap().get_index() as u64);
                    let global = fd.new_constant(width, 0x8000);
                    fd.op_set_all_input(store, &[memory, global, version]).unwrap();
                    fd.op_insert_end(store, body);
                } else if matches!(mode,"address"|"direct_address") {
                    let pointer = fd.new_op(2, point.clone());
                    fd.op_set_opcode_code(pointer, OpCode::CPUI_INT_ADD);
                    let entry = fd.find_spacebase_input(&stack).unwrap();
                    let offset = fd.new_constant(width, address.get_offset());
                    fd.op_set_all_input(pointer, &[entry, offset]).unwrap();
                    let value = fd.new_unique_out(width, pointer).unwrap();
                    let call = fd.get_call_specs(index).get_op();
                    fd.op_insert_before(pointer, call);
                    fd.op_set_input(call, value, 1).unwrap();
                } else if mode == "partial" {
                    let write = fd.new_op(1, point.clone());
                    fd.op_set_opcode_code(write, OpCode::CPUI_COPY);
                    let zero = fd.new_constant(1, 0);
                    fd.op_set_input(write, zero, 0).unwrap();
                    let partial = Address::new(Rc::clone(&stack), address.get_offset().wrapping_add(1) & calc_mask(width));
                    fd.new_varnode_out(1, &partial, write).unwrap();
                    fd.op_insert_before(write, end.restore);
                } else if mode.starts_with("masked") {
                    // A byte read through `(frame + k) | 3`, as compilers emit
                    // for the low byte of a big-endian spill.
                    let entry = fd.find_spacebase_input(&stack).unwrap();
                    let base = if mode == "masked_disjoint" { address.get_offset().wrapping_add(width as u64) }
                        else { address.get_offset().wrapping_sub(2) };
                    let add = fd.new_op(2, point.clone());
                    fd.op_set_opcode_code(add, OpCode::CPUI_INT_ADD);
                    let offset = fd.new_constant(width, base & calc_mask(width));
                    fd.op_set_all_input(add, &[entry, offset]).unwrap();
                    let sum = fd.new_unique_out(width, add).unwrap();
                    let or = fd.new_op(2, point.clone());
                    fd.op_set_opcode_code(or, OpCode::CPUI_INT_OR);
                    let three = fd.new_constant(width, 3);
                    fd.op_set_all_input(or, &[sum, three]).unwrap();
                    let masked = fd.new_unique_out(width, or).unwrap();
                    let load = fd.new_op(2, point.clone());
                    fd.op_set_opcode_code(load, OpCode::CPUI_LOAD);
                    let memory = fd.new_constant(4, stack.get_contain().unwrap().get_index() as u64);
                    fd.op_set_all_input(load, &[memory, masked]).unwrap();
                    fd.new_unique_out(1, load).unwrap();
                    for op in [add, or, load] { fd.op_insert_before(op, end.restore); }
                } else if mode.starts_with("reload") {
                    // The scope's restore reloads the checkpoint word and the
                    // restored SP is observed afterwards.
                    let entry = fd.find_spacebase_input(&stack).unwrap();
                    if mode == "reload_clobbered" {
                        let clobber = fd.new_op(1, point.clone());
                        fd.op_set_opcode_code(clobber, OpCode::CPUI_COPY);
                        let other = fd.new_constant(width, 0x1234);
                        fd.op_set_input(clobber, other, 0).unwrap();
                        fd.new_varnode_out(width, &address, clobber).unwrap();
                        fd.op_insert_before(clobber, end.restore);
                    }
                    let add = fd.new_op(2, point.clone());
                    fd.op_set_opcode_code(add, OpCode::CPUI_INT_ADD);
                    let offset = fd.new_constant(width, address.get_offset());
                    fd.op_set_all_input(add, &[entry, offset]).unwrap();
                    let pointer = fd.new_unique_out(width, add).unwrap();
                    let load = fd.new_op(2, point.clone());
                    fd.op_set_opcode_code(load, OpCode::CPUI_LOAD);
                    let memory = fd.new_constant(4, stack.get_contain().unwrap().get_index() as u64);
                    fd.op_set_all_input(load, &[memory, pointer]).unwrap();
                    let reloaded = fd.new_unique_out(width, load).unwrap();
                    // A read before the save does not observe the checkpoint.
                    let at = if mode == "reload_before_save" { fd.vbank().get(saved).unwrap().get_def().unwrap() } else { end.restore };
                    for op in [add, load] { fd.op_insert_before(op, at); }
                    fd.op_set_input(end.restore, reloaded, 0).unwrap();
                    let sp = fd.vbank().get(allocation.output).unwrap().get_addr().clone();
                    let restored_sp = fd.obank().get(end.restore).unwrap().get_out().unwrap();
                    assert_eq!(fd.vbank().get(restored_sp).unwrap().get_addr(), &sp);
                    let publish = fd.new_op(3, point.clone());
                    fd.op_set_opcode_code(publish, OpCode::CPUI_STORE);
                    let global = fd.new_constant(width, 0x8000);
                    fd.op_set_all_input(publish, &[memory, global, restored_sp]).unwrap();
                    fd.op_insert_after(publish, end.restore);
                } else if mode == "held" {
                    fd.vbank_mut().get_mut(version).unwrap().set_auto_live_hold();
                }
                let ret = fd.new_op(1, point);
                fd.op_set_opcode_code(ret, OpCode::CPUI_RETURN);
                let zero = fd.new_constant(4, 0);
                fd.op_set_input(ret, zero, 0).unwrap();
                fd.op_insert_end(ret, end.boundary.unwrap());
                let slots = unused_checkpoint_slots(&fd,&FrameFacts::infer(&fd,vec![allocation]).unwrap(),width,&[]);
                assert_eq!(slots.iter().any(|plan| plan.address == address && plan.size == width), matches!(mode,"unused"|"direct"|"masked_disjoint"|"reload"), "width={width}, mode={mode}");
                release_checkpoint_slots(&mut fd, &slots);
                assert_eq!(fd.vbank().get(version).unwrap().is_addr_force(), !matches!(mode,"unused"|"direct"|"masked_disjoint"|"reload"),
                    "observable or externally owned physical writes must remain live");
            }
        }
    }
    #[test]
    fn pure_copy_checkpoints_still_require_an_unmodified_unexposed_word() {
        for width in [4, 8] {
            for case in ["clean", "full_store", "full_copy", "same_value", "before_seed",
                "address_escape", "saved_data", "global_copy", "basis_poison"] {
                let (mut fd, mut allocation, end, block) = scoped_lifetime_fixture(width, false);
                let stack = Rc::clone(fd.get_arch().manage().get_stack_space().unwrap());
                let slot = Address::new(Rc::clone(&stack), (-32_i64) as u64 & calc_mask(width));
                let save = fd.new_op(1, fd.get_address().clone());
                fd.op_set_opcode_code(save, OpCode::CPUI_COPY);
                fd.op_set_input(save, allocation.base, 0).unwrap();
                let saved = fd.new_varnode_out(width, &slot, save).unwrap();
                fd.op_insert_before(save, allocation.op);
                fd.op_set_input(end.restore, saved, 0).unwrap();
                if case == "basis_poison" {
                    allocation.base = saved;
                    fd.op_set_input(allocation.op, saved, 0).unwrap();
                }
                if matches!(case, "full_copy" | "global_copy") {
                    let copy = fd.new_op(1, fd.get_address().clone());
                    fd.op_set_opcode_code(copy, OpCode::CPUI_COPY);
                    let data = if case == "global_copy" { saved } else { fd.new_constant(width, 0) };
                    fd.op_set_input(copy, data, 0).unwrap();
                    let address = if case == "global_copy" {
                        Address::new(Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap()), 0x8000)
                    } else { slot.clone() };
                    fd.new_varnode_out(width, &address, copy).unwrap();
                    fd.op_insert_end(copy, block);
                } else if case != "clean" {
                    let input = fd.find_spacebase_input(&stack).unwrap();
                    let address = fd.new_op(2, fd.get_address().clone());
                    fd.op_set_opcode_code(address, OpCode::CPUI_INT_ADD);
                    fd.op_set_input(address, input, 0).unwrap();
                    let offset = fd.new_constant(width, (-32_i64) as u64 & calc_mask(width));
                    fd.op_set_input(address, offset, 1).unwrap();
                    let pointer = fd.new_unique_out(width, address).unwrap();
                    if case == "before_seed" { fd.op_insert_before(address, save); }
                    else { fd.op_insert_end(address, block); }
                    let store = fd.new_op(3, fd.get_address().clone());
                    fd.op_set_opcode_code(store, OpCode::CPUI_STORE);
                    let space = fd.new_constant(4, 4);
                    fd.op_set_input(store, space, 0).unwrap();
                    let data = match case {
                        "same_value" => allocation.base,
                        "address_escape" => pointer,
                        "saved_data" => saved,
                        _ => fd.new_constant(width, 0),
                    };
                    let destination = if matches!(case, "address_escape" | "saved_data") {
                        fd.new_constant(width, 0x8000)
                    } else { pointer };
                    fd.op_set_input(store, destination, 1).unwrap();
                    fd.op_set_input(store, data, 2).unwrap();
                    if case == "before_seed" { fd.op_insert_after(store, address); }
                    else { fd.op_insert_end(store, block); }
                }
                // Observing the saved native VALUE does not expose the WORD's
                // address or invalidate equality. Liveness release separately
                // preserves real readers and exported data.
                assert_eq!(restores_checkpoint(&mut fd, &allocation, &end),
                    matches!(case, "clean" | "same_value" | "before_seed" | "saved_data" | "global_copy"), "width={width}, case={case}");
            }
        }
    }    #[test]
    fn saved_word_data_checks_continue_through_restored_sp_carriers() {
        for width in [4, 8] {
            for case in ["adjust_data", "restore_data", "adjust_copy", "address_only"] {
                let (mut fd, allocation, end, _, _, _) = checkpoint_call_fixture(width, false, false);
                let boundary = end.boundary.unwrap();
                let value = if case == "restore_data" { end.value } else {
                    let restored = fd.obank().get(end.restore).unwrap().get_in(0).unwrap();
                    let adjust = fd.new_op(2, fd.get_address().clone());
                    fd.op_set_opcode_code(adjust, OpCode::CPUI_INT_ADD);
                    fd.op_set_input(adjust, restored, 0).unwrap();
                    let amount = fd.new_constant(width, width as u64);
                    fd.op_set_input(adjust, amount, 1).unwrap();
                    let sp = fd.vbank().get(allocation.output).unwrap().get_addr().clone();
                    let value = fd.new_varnode_out(width, &sp, adjust).unwrap();
                    fd.op_insert_end(adjust, boundary);
                    value
                };
                if case == "adjust_copy" {
                    let copy = fd.new_op(1, fd.get_address().clone());
                    fd.op_set_opcode_code(copy, OpCode::CPUI_COPY);
                    fd.op_set_input(copy, value, 0).unwrap();
                    let ram = Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap());
                    fd.new_varnode_out(width, &Address::new(ram, 0x8000), copy).unwrap();
                    fd.op_insert_end(copy, boundary);
                } else {
                    let store = fd.new_op(3, fd.get_address().clone());
                    fd.op_set_opcode_code(store, OpCode::CPUI_STORE);
                    let space = fd.new_constant(4, 4);
                    fd.op_set_input(store, space, 0).unwrap();
                    let address = if case == "address_only" { value } else { fd.new_constant(width, 0x8000) };
                    let data = if case == "address_only" { fd.new_constant(width, 7) } else { value };
                    fd.op_set_input(store, address, 1).unwrap();
                    fd.op_set_input(store, data, 2).unwrap();
                    fd.op_insert_end(store, boundary);
                }
                // Later source-visible SP data must remain visible, but it
                // cannot retroactively change the earlier saved-word read.
                assert_eq!(restores_checkpoint(&mut fd, &allocation, &end), true,
                    "width={width}, case={case}");
            }
        }
    }    #[test]
    fn checkpoint_recognizes_reassociated_allocation_addresses() {
        for width in [4, 8] {
            for case in ["same_object", "unresolved_frame", "checkpoint"] {
                let (mut fd, allocation, end, _, index, _) = checkpoint_call_fixture(width, false, false);
                let call = fd.get_call_specs(index).get_op();
                let base = fd.new_op(2, fd.get_address().clone());
                fd.op_set_opcode_code(base, OpCode::CPUI_INT_ADD);
                fd.op_set_input(base, allocation.base, 0).unwrap();
                let offset = if case == "checkpoint" { 32_i64 } else { -16_i64 };
                let offset = fd.new_constant(width, offset as u64 & calc_mask(width));
                fd.op_set_input(base, offset, 1).unwrap();
                let base_value = fd.new_unique_out(width, base).unwrap();
                fd.op_insert_before(base, call);
                let amount = if case == "checkpoint" {
                    fd.new_constant(width, 0)
                } else if case == "unresolved_frame" {
                    let ram = Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap());
                    let input = fd.new_varnode(width, &Address::new(ram, 0x2800), None);
                    fd.set_input_varnode(input).unwrap()
                } else {
                    allocation.amount
                };
                let address = fd.new_op(2, fd.get_address().clone());
                fd.op_set_opcode_code(address, OpCode::CPUI_INT_SUB);
                fd.op_set_input(address, base_value, 0).unwrap();
                fd.op_set_input(address, amount, 1).unwrap();
                let pointer = fd.new_unique_out(width, address).unwrap();
                fd.op_insert_before(address, call);
                fd.op_set_input(call, pointer, 1).unwrap();
                assert_eq!(restores_checkpoint(&mut fd, &allocation, &end), case == "same_object",
                    "width={width}, case={case}");
            }
        }
    }        fn read_address_delta(fd: &mut Funcdata, root: VarnodeId, read: OpId) -> Option<i128> {
        let calls: Vec<_>=(0..fd.num_calls()).map(|index|fd.get_call_specs(index).get_op()).collect();
        for call in calls {capture_call_stack(fd,call);}
        let mut reservation=allocation(fd,fd.vbank().get(root)?.get_def()?)?;
        normalize_reservation(fd,&mut reservation);
        let facts=FrameFacts::infer(fd,vec![reservation])?;
        facts.equations.delta(fd,fd.obank().get(read)?.get_out()?,root)
    }

    #[test]
    fn private_address_spills_require_one_non_escaping_whole_word() {
        for width in [4, 8] {
            for change in ["private", "reassociated", "creation", "zero", "escaped_before",
                "indirect_target", "raw_load", "partial", "late_partial", "other_slot", "unknown_pop",
                "positive_delta", "negative_delta"]
            {
                let (mut fd, allocation, end, block, index, _) = checkpoint_call_fixture(width, false, false);
                let stack = Rc::clone(fd.get_arch().manage().get_stack_space().unwrap());
                let entry = fd.find_spacebase_input(&stack).unwrap();
                let point = fd.get_address().clone();
                let address = Address::new(Rc::clone(&stack), (-40_i64) as u64 & calc_mask(width));
                let call = fd.get_call_specs(index).get_op();
                let init = fd.new_op(1, point.clone());
                fd.op_set_opcode_code(init, OpCode::CPUI_COPY);
                let delta = match change { "positive_delta" => 16_i128, "negative_delta" => -16, _ => 0 };
                let initial = if change == "zero" {
                    fd.new_constant(width, 0)
                } else if delta != 0 {
                    let add = fd.new_op(2, point.clone());
                    fd.op_set_opcode_code(add, OpCode::CPUI_INT_ADD);
                    let offset = fd.new_constant(width, delta as u64 & calc_mask(width));
                    fd.op_set_all_input(add, &[allocation.output, offset]).unwrap();
                    let result = fd.new_unique_out(width, add).unwrap();
                    fd.op_insert_before(add, call);
                    result
                } else { allocation.output };
                fd.op_set_input(init, initial, 0).unwrap();
                let saved = fd.new_varnode_out(width, &address, init).unwrap();
                fd.op_insert_before(init, call);
                let marker = fd.new_op(2, point.clone());
                fd.op_set_opcode_code(marker, OpCode::CPUI_INDIRECT);
                let effect = fd.new_varnode_iop(call);
                let source = if change == "reassociated" { allocation.output } else { saved };
                fd.op_set_all_input(marker, &[source, effect]).unwrap();
                let mut value = fd.new_varnode_out(width, &address, marker).unwrap();
                fd.op_insert_after(marker, call);
                if change == "creation" {
                    let unknown = fd.new_constant(width, 0);
                    fd.op_set_input(marker, unknown, 0).unwrap();
                    fd.mark_indirect_creation(marker, false).unwrap();
                }
                if change == "unknown_pop" {
                    fd.get_call_specs_mut(index).proto_mut().set_extra_pop(crate::fspec::EXTRAPOP_UNKNOWN);
                }
                if matches!(change, "escaped_before" | "indirect_target" | "raw_load") {
                    let pointer = fd.new_op(2, point.clone());
                    fd.op_set_opcode_code(pointer, OpCode::CPUI_INT_ADD);
                    let offset = fd.new_constant(width, address.get_offset());
                    fd.op_set_all_input(pointer, &[entry, offset]).unwrap();
                    let pointer_value = fd.new_unique_out(width, pointer).unwrap();
                    fd.op_insert_before(pointer, allocation.op);
                    let access = fd.new_op(if change == "indirect_target" { 1 } else { 2 }, point.clone());
                    if change == "indirect_target" {
                        fd.op_set_opcode_code(access, OpCode::CPUI_CALLIND);
                        fd.op_set_input(access, pointer_value, 0).unwrap();
                    } else if change == "escaped_before" {
                        fd.op_set_opcode_code(access, OpCode::CPUI_CALL);
                        let target = fd.new_constant(width, 0x8000);
                        fd.op_set_all_input(access, &[target, pointer_value]).unwrap();
                    } else {
                        fd.op_set_opcode_code(access, OpCode::CPUI_LOAD);
                        let space = fd.new_constant(4, stack.get_contain().unwrap().get_index() as u64);
                        fd.op_set_all_input(access, &[space, pointer_value]).unwrap();
                        fd.new_unique_out(width, access).unwrap();
                    }
                    fd.op_insert_before(access, allocation.op);
                }
                if change == "other_slot" {
                    let copy = fd.new_op(1, point.clone());
                    fd.op_set_opcode_code(copy, OpCode::CPUI_COPY);
                    fd.op_set_input(copy, value, 0).unwrap();
                    let other = Address::new(Rc::clone(&stack), (-48_i64) as u64 & calc_mask(width));
                    value = fd.new_varnode_out(width, &other, copy).unwrap();
                    fd.op_insert_end(copy, block);
                }
                let after = end.boundary.unwrap();
                let mut reads = Vec::new();
                for _ in 0..2 {
                    let read = fd.new_op(1, point.clone());
                    fd.op_set_opcode_code(read, OpCode::CPUI_COPY);
                    fd.op_set_input(read, value, 0).unwrap();
                    fd.new_unique_out(width, read).unwrap();
                    fd.op_insert_end(read, after);
                    reads.push((read, 0));
                }
                if matches!(change, "partial" | "late_partial") {
                    let write = fd.new_op(1, point);
                    fd.op_set_opcode_code(write, OpCode::CPUI_COPY);
                    let byte = fd.new_constant(1, 0);
                    fd.op_set_input(write, byte, 0).unwrap();
                    let partial = Address::new(stack, address.get_offset().wrapping_add(1) & calc_mask(width));
                    fd.new_varnode_out(1, &partial, write).unwrap();
                    fd.op_insert_before(write, reads[usize::from(change == "late_partial")].0);
                }
                let expected = matches!(change, "private" | "reassociated" | "positive_delta" | "negative_delta" | "other_slot" | "raw_load");
                for &(read, _) in &reads {
                    assert_eq!(read_address_delta(&mut fd, allocation.output, read),
                        (expected || (change == "late_partial" && read == reads[0].0)).then_some(delta),
                        "width={width}, change={change}, read={read:?}");
                }
                if change == "late_partial" {
                    assert_eq!(read_address_delta(&mut fd, allocation.output, reads[0].0), Some(0),
                        "the first read precedes the partial overwrite");
                }
                let captures = scope_captures(&fd, &FrameFacts::infer(&fd,vec![allocation]).unwrap(), &[], &[Some(end)], &[], &[]);
                if expected {
                    let captures = captures.unwrap();
                    for &(consumer, slot) in &reads {
                        assert!(captures.iter().any(|capture| capture.uses.contains(&(consumer, slot))));
                    }
                } else if matches!(change, "partial" | "late_partial") {
                    assert!(captures.is_err(), "a later read must not be replaced with the pre-overwrite address");
                }
            }
        }
    }
    #[test]
    fn address_spills_use_the_actual_post_restore_call_frame() {
        use crate::fspec::FuncCallSpecs;
        for width in [4, 8] {
            for change in ["fixed_frame", "entry_frame", "unknown_frame", "after_both_reads",
                "fixed_consumed", "consumed_canonical", "consumed_overlap"]
            {
                let (mut fd, allocation, end, _, index, _) = checkpoint_call_fixture(width, false, false);
                let stack = Rc::clone(fd.get_arch().manage().get_stack_space().unwrap());
                let entry = fd.find_spacebase_input(&stack).unwrap();
                let point = fd.get_address().clone();
                let address = Address::new(Rc::clone(&stack), (-40_i64) as u64 & calc_mask(width));
                let save = fd.new_op(1, point.clone());
                fd.op_set_opcode_code(save, OpCode::CPUI_COPY);
                fd.op_set_input(save, allocation.output, 0).unwrap();
                let saved = fd.new_varnode_out(width, &address, save).unwrap();
                fd.op_insert_before(save, end.restore);
                let after = end.boundary.unwrap();
                let mut reads = Vec::new();
                for _ in 0..2 {
                    let read = fd.new_op(1, point.clone());
                    fd.op_set_opcode_code(read, OpCode::CPUI_COPY);
                    fd.op_set_input(read, saved, 0).unwrap();
                    fd.new_unique_out(width, read).unwrap();
                    fd.op_insert_end(read, after);
                    reads.push((read, 0));
                }
                let call = fd.new_op(1, point.clone());
                fd.op_set_opcode_code(call, OpCode::CPUI_CALL);
                let target = fd.new_constant(width, 0x9000);
                fd.op_set_input(call, target, 0).unwrap();
                if change == "after_both_reads" { fd.op_insert_after(call, reads[1].0); }
                else { fd.op_insert_before(call, reads[1].0); }
                let mut spec = FuncCallSpecs::new(call, Address::new_invalid());
                spec.proto_mut().attach_internal_store(Rc::new(Datatype::new(0, type_metatype::TYPE_VOID)));
                spec.proto_mut().set_model(Some(fd.get_call_specs(index).proto().model().clone()));
                spec.proto_mut().set_extra_pop(0);
                spec.set_final_input_storage(Vec::new());
                if matches!(change, "fixed_frame" | "entry_frame" | "fixed_consumed"
                    | "consumed_canonical" | "consumed_overlap")
                {
                    spec.create_placeholder(&mut fd, &stack).unwrap();
                    let placeholder = fd.obank().get(call).unwrap().get_in(spec.get_stack_placeholder_slot()).unwrap();
                    let load = fd.vbank().get(placeholder).unwrap().get_def().unwrap();
                    let pointer = fd.obank().get(load).unwrap().get_in(1).unwrap();
                    let add = fd.vbank().get(pointer).unwrap().get_def().unwrap();
                    let sp = match change {
                        "entry_frame" => entry,
                        "fixed_frame" | "fixed_consumed" => allocation.base,
                        _ => end.value,
                    };
                    fd.op_set_input(add, sp, 0).unwrap();
                }
                let post_index = fd.push_call_specs(spec);
                if change=="unknown_frame" {
                    // No argument-space proxy and an actual opaque SP effect:
                    // a known native definition is otherwise a valid witness.
                    let unknown=fd.new_op(1,fd.get_address().clone());
                    fd.op_set_opcode_code(unknown,OpCode::CPUI_CALLOTHER);
                    let userop=fd.new_constant(4,0);
                    fd.op_set_input(unknown,userop,0).unwrap();
                    let sp=fd.vbank().get(allocation.output).unwrap().get_addr().clone();
                    fd.new_varnode_out(width,&sp,unknown).unwrap();
                    fd.op_insert_before(unknown,call);
                }

                if matches!(change, "fixed_consumed" | "consumed_canonical" | "consumed_overlap") {
                    capture_call_stack(&mut fd, call).unwrap();
                    let mut spec = std::mem::replace(fd.get_call_specs_mut(post_index),
                        FuncCallSpecs::new(call, Address::new_invalid()));
                    spec.abort_spacebase_relative(&mut fd);
                    fd.total_replace(end.value, allocation.base).unwrap();
                    if change == "consumed_overlap" {
                        spec.set_final_input_storage(vec![(Address::new(Rc::clone(&stack), 24), width)]);
                    }
                    *fd.get_call_specs_mut(post_index) = spec;
                }
                let last = if change == "after_both_reads" {
                    saved
                } else {
                    let marker = fd.new_op(2, point);
                    fd.op_set_opcode_code(marker, OpCode::CPUI_INDIRECT);
                    let effect = fd.new_varnode_iop(call);
                    fd.op_set_all_input(marker, &[saved, effect]).unwrap();
                    let output = fd.new_varnode_out(width, &address, marker).unwrap();
                    fd.op_insert_before(marker, call);
                    fd.op_set_input(reads[1].0, output, 0).unwrap();
                    output
                };
                assert_eq!(read_address_delta(&mut fd, allocation.output, reads[0].0), Some(0),
                    "the first read precedes the later call");
                let expected = matches!(change, "fixed_frame" | "after_both_reads" | "fixed_consumed" | "consumed_canonical");
                assert_eq!(read_address_delta(&mut fd, allocation.output, reads[1].0), expected.then_some(0),
                    "width={width}, change={change}");
                if !expected {
                    assert!(scope_captures(&fd, &FrameFacts::infer(&fd,vec![allocation]).unwrap(), &[], &[Some(end)], &[], &[]).is_err(),
                        "a later ABI frame may overwrite the saved address");
                }
                let _ = last;
            }
        }
    }

    #[test]
    fn conditional_stack_word_reads_validate_the_selected_incoming_edge() {
        for width in [4, 8] {
            for change in ["preserved", "partial_tail", "same_word", "disjoint_tail"] {
                let (mut fd, allocation, end, _) = scoped_lifetime_fixture(width, false);
                let stack = Rc::clone(fd.get_arch().manage().get_stack_space().unwrap());
                let entry = fd.find_spacebase_input(&stack).unwrap();
                let entry_block = fd.obank().get(allocation.op).unwrap().get_parent().unwrap();
                let join = end.boundary.unwrap();
                fd.bblocks_mut().add_edge(entry_block, join);
                let graph = fd.bblocks_root_pub();
                fd.bblocks_mut().calc_forward_dominator(graph, &[entry_block]);
                let word = Address::new(Rc::clone(&stack), (-24_i64) as u64 & calc_mask(width));
                let initial = fd.new_op(1, fd.get_address().clone());
                fd.op_set_opcode_code(initial, OpCode::CPUI_COPY);
                let scalar = fd.new_constant(width, 77);
                fd.op_set_input(initial, scalar, 0).unwrap();
                fd.new_varnode_out(width, &word, initial).unwrap();
                fd.op_insert_before(initial, allocation.op);
                let save = fd.new_op(1, fd.get_address().clone());
                fd.op_set_opcode_code(save, OpCode::CPUI_COPY);
                fd.op_set_input(save, allocation.base, 0).unwrap();
                let saved = fd.new_varnode_out(width, &word, save).unwrap();
                fd.op_insert_end(save, end.join);
                if change != "preserved" {
                    let address = fd.new_op(2, fd.get_address().clone());
                    fd.op_set_opcode_code(address, OpCode::CPUI_INT_ADD);
                    fd.op_set_input(address, entry, 0).unwrap();
                    let first = if change == "partial_tail" { -23_i64 }
                        else if change == "disjoint_tail" { -24 + width as i64 } else { -24 };
                    let offset = fd.new_constant(width, first as u64 & calc_mask(width));
                    fd.op_set_input(address, offset, 1).unwrap();
                    let pointer = fd.new_unique_out(width, address).unwrap();
                    fd.op_insert_end(address, end.join);
                    let store = fd.new_op(3, fd.get_address().clone());
                    fd.op_set_opcode_code(store, OpCode::CPUI_STORE);
                    let memory = fd.new_constant(4, stack.get_contain().unwrap().get_index() as u64);
                    fd.op_set_input(store, memory, 0).unwrap();
                    fd.op_set_input(store, pointer, 1).unwrap();
                    let data = if change == "same_word" { allocation.base } else { fd.new_constant(1, 0xaa) };
                    fd.op_set_input(store, data, 2).unwrap();
                    fd.op_insert_end(store, end.join);
                }
                let phi = fd.new_op(2, fd.get_address().clone());
                fd.op_set_opcode_code(phi, OpCode::CPUI_MULTIEQUAL);
                fd.op_set_input(phi, saved, 0).unwrap();
                let zero = fd.new_constant(width, 0);
                fd.op_set_input(phi, zero, 1).unwrap();
                fd.new_unique_out(width, phi).unwrap();
                fd.op_insert_end(phi, join);
                assert_eq!(private_frame_slot(&fd, -24, width, FrameRead::Incoming(phi, 0),
                    Some(save), Some(saved), &mut 65536, &mut SavedFrameQuery::default()).is_some(),
                    change != "partial_tail", "width={width}, change={change}");
                assert!(private_frame_slot(&fd, -24, width, FrameRead::Operation(phi),
                    Some(save), Some(saved), &mut 65536, &mut SavedFrameQuery::default()).is_none(),
                    "The bypass edge contains a different word: width={width}, change={change}");
            }
        }
    }

    #[test]
    fn incoming_loop_words_check_preheader_clobbers_and_earlier_escapes() {
        for width in [4, 8] {
            for change in ["preserved", "clobbered", "escaped"] {
                let (mut fd, allocation, _, body) = scoped_lifetime_fixture(width, false);
                let stack = Rc::clone(fd.get_arch().manage().get_stack_space().unwrap());
                let entry = fd.find_spacebase_input(&stack).unwrap();
                let word = Address::new(Rc::clone(&stack), (-24_i64) as u64 & calc_mask(width));
                let save = fd.new_op(1, fd.get_address().clone());
                fd.op_set_opcode_code(save, OpCode::CPUI_COPY);
                fd.op_set_input(save, allocation.base, 0).unwrap();
                let saved = fd.new_varnode_out(width, &word, save).unwrap();
                fd.op_insert_before(save, allocation.op);
                if change != "preserved" {
                    let address = fd.new_op(2, fd.get_address().clone());
                    fd.op_set_opcode_code(address, OpCode::CPUI_INT_ADD);
                    fd.op_set_input(address, entry, 0).unwrap();
                    let offset = fd.new_constant(width, (-24_i64) as u64 & calc_mask(width));
                    fd.op_set_input(address, offset, 1).unwrap();
                    let pointer = fd.new_unique_out(width, address).unwrap();
                    fd.op_insert_before(address, save);
                    let store = fd.new_op(3, fd.get_address().clone());
                    fd.op_set_opcode_code(store, OpCode::CPUI_STORE);
                    let memory = fd.new_constant(4, stack.get_contain().unwrap().get_index() as u64);
                    fd.op_set_input(store, memory, 0).unwrap();
                    if change == "escaped" {
                        let global = fd.new_constant(width, 0x8000);
                        fd.op_set_input(store, global, 1).unwrap();
                        fd.op_set_input(store, pointer, 2).unwrap();
                        fd.op_insert_before(store, save);
                    } else {
                        fd.op_set_input(store, pointer, 1).unwrap();
                        let byte = fd.new_constant(1, 0xaa);
                        fd.op_set_input(store, byte, 2).unwrap();
                        fd.op_insert_after(store, save);
                    }
                }
                let phi = fd.new_op(2, fd.get_address().clone());
                fd.op_set_opcode_code(phi, OpCode::CPUI_MULTIEQUAL);
                let carried = fd.new_unique_out(width, phi).unwrap();
                fd.op_set_input(phi, saved, 0).unwrap();
                fd.op_set_input(phi, carried, 1).unwrap();
                fd.op_insert_begin(phi, body);
                assert_eq!(private_frame_slot(&fd, -24, width, FrameRead::Incoming(phi, 1),
                    Some(save), Some(saved), &mut 65536, &mut SavedFrameQuery::default()).is_some(),
                    change == "preserved", "width={width}, change={change}");
            }
        }
    }

    #[test]
    fn scalar_subword_arguments_do_not_inherit_overwritten_frame_origins() {
        for width in [4, 8] {
            for displacement in [0, width / 2] {
                for payload in ["scalar", "frame", "partial"] {
                    let (mut fd, allocation, end, _, index, _) = checkpoint_call_fixture(width, false, false);
                    let call = fd.get_call_specs(index).get_op();
                    let stack = Rc::clone(fd.get_arch().manage().get_stack_space().unwrap());
                    let entry = fd.find_spacebase_input(&stack).unwrap();
                    let address = fd.new_op(2, fd.get_address().clone());
                    fd.op_set_opcode_code(address, OpCode::CPUI_INT_ADD);
                    fd.op_set_input(address, entry, 0).unwrap();
                    let offset = fd.new_constant(width, (-48_i64) as u64 & calc_mask(width));
                    fd.op_set_input(address, offset, 1).unwrap();
                    let pointer = fd.new_unique_out(width, address).unwrap();
                    fd.op_insert_before(address, call);
                    let read_address = fd.new_op(2, fd.get_address().clone());
                    fd.op_set_opcode_code(read_address, OpCode::CPUI_INT_ADD);
                    fd.op_set_input(read_address, pointer, 0).unwrap();
                    let offset = fd.new_constant(width, displacement as u64);
                    fd.op_set_input(read_address, offset, 1).unwrap();
                    let field = fd.new_unique_out(width, read_address).unwrap();
                    fd.op_insert_before(read_address, call);
                    let memory = fd.new_constant(4, stack.get_contain().unwrap().get_index() as u64);
                    let prior = fd.new_op(3, fd.get_address().clone());
                    fd.op_set_opcode_code(prior, OpCode::CPUI_STORE);
                    fd.op_set_input(prior, memory, 0).unwrap();
                    fd.op_set_input(prior, pointer, 1).unwrap();
                    fd.op_set_input(prior, entry, 2).unwrap();
                    fd.op_insert_before(prior, call);
                    let overwrite = fd.new_op(3, fd.get_address().clone());
                    fd.op_set_opcode_code(overwrite, OpCode::CPUI_STORE);
                    fd.op_set_input(overwrite, memory, 0).unwrap();
                    fd.op_set_input(overwrite, if payload == "partial" { field } else { pointer }, 1).unwrap();
                    let value = if payload == "frame" { entry } else if payload == "partial" {
                        fd.new_constant(1, 0)
                    } else {
                        let ram = Rc::clone(fd.get_arch().manage().get_default_code_space().unwrap());
                        let value = fd.new_varnode(width, &Address::new(ram, 0x4000), None);
                        fd.set_input_varnode(value).unwrap()
                    };
                    fd.op_set_input(overwrite, value, 2).unwrap();
                    fd.op_insert_before(overwrite, call);
                    let load = fd.new_op(2, fd.get_address().clone());
                    fd.op_set_opcode_code(load, OpCode::CPUI_LOAD);
                    fd.op_set_input(load, memory, 0).unwrap();
                    fd.op_set_input(load, field, 1).unwrap();
                    let argument = fd.new_unique_out(width / 2, load).unwrap();
                    fd.op_insert_before(load, call);
                    fd.op_set_input(call, argument, 1).unwrap();
                    assert_eq!(restores_checkpoint(&mut fd, &allocation, &end), payload == "scalar",
                        "width={width}, displacement={displacement}, payload={payload}");
                }
            }
        }
    }
    #[test]
    fn large_positive_abi_ranges_are_not_legacy_negative_tails() {
        let (mut fd, allocation, end, _, index, _) = checkpoint_call_fixture(8, false, false);
        let stack = Rc::clone(fd.get_arch().manage().get_stack_space().unwrap());
        let mut model = fd.get_call_specs(index).proto().model().as_ref().clone();
        let mut locals = kuna_base::address::RangeList::new();
        // This is positive in this 64-bit, unbiased ABI, even though the same
        // spelling is the legacy negative tail in the bundled biased-window ABI.
        locals.insert_range(stack, 0xfff0bdc1, u32::MAX as u64);
        model.set_local_range(locals);
        fd.get_call_specs_mut(index).proto_mut().set_model(Some(Rc::new(model)));
        assert_eq!(call_frame_floor(&fd, fd.get_call_specs(index)), Some(1_i128 << 32));
        assert!(!restores_checkpoint(&mut fd, &allocation, &end));
    }

    #[test]
    fn loop_lifetime_distinguishes_restoration_from_carried_storage() {
        use kuna_base::space::{SpacebaseSpace, VarnodeStorage};
        for width in [4, 8] {
            let mut manager = build_manager();
            let ram = Rc::clone(manager.get_space_by_name("ram").unwrap());
            let register = Rc::new(AddrSpace::new(
                spacetype::IPTR_PROCESSOR,
                "register",
                false,
                8,
                1,
                5,
                0,
                0,
                0,
            ));
            manager.insert_space(Rc::clone(&register)).unwrap();
            let stack = Rc::new(SpacebaseSpace::new(
                "stack", 6, width as u32, &ram, 1, true, false,
            ));
            manager
                .add_spacebase_pointer(
                    &stack,
                    &VarnodeStorage {
                        space: Some(Rc::clone(&register)),
                        offset: 0,
                        size: width as u32,
                    },
                    width,
                    true,
                )
                .unwrap();
            manager.insert_space(stack).unwrap();
            let arch = Rc::new(ArchContext::new(manager));
            let mut fd = Funcdata::new(
                "restored",
                "restored",
                arch,
                Address::new(Rc::clone(&ram), 0x1000),
                0x10000000,
                0x40,
            )
            .unwrap();
            let root = fd.bblocks_root_pub();
            let blocks: Vec<_> = (0..8)
                .map(|_| fd.bblocks_mut().new_block_basic(root))
                .collect();
            for (index, &block) in blocks.iter().enumerate() {
                fd.bblocks_mut().block_mut(block).set_index(index as int4);
            }
            for (from, to) in [
                (0, 1),
                (0, 2),
                (1, 3),
                (2, 3),
                (3, 4),
                (4, 5),
                (4, 7),
                (5, 6),
                (6, 4),
            ] {
                fd.bblocks_mut().add_edge(blocks[from], blocks[to]);
            }
            fd.bblocks_mut().calc_forward_dominator(root, &[blocks[0]]);
            let sp_address = Address::new(register, 0);
            let base = fd.new_varnode(width, &sp_address, None);
            let base = fd.set_input_varnode(base).unwrap();
            let amount = fd.new_varnode(width, &Address::new(ram, 0x2000), None);
            let amount = fd.set_input_varnode(amount).unwrap();
            let mut older = Vec::new();
            let mut current = None;
            for block in [blocks[1], blocks[2], blocks[5]] {
                let op = fd.new_op(2, fd.get_address().clone());
                fd.op_set_opcode_code(op, OpCode::CPUI_INT_SUB);
                fd.op_set_input(op, base, 0).unwrap();
                fd.op_set_input(op, amount, 1).unwrap();
                let out = fd.new_varnode_out(width, &sp_address, op).unwrap();
                fd.op_insert_end(op, block);
                if block == blocks[5] {
                    current = Some(op);
                } else {
                    older.push(out);
                }
            }
            let saved = fd.new_op(2, fd.get_address().clone());
            fd.op_set_opcode_code(saved, OpCode::CPUI_MULTIEQUAL);
            for (slot, &value) in older.iter().enumerate() {
                fd.op_set_input(saved, value, slot as int4).unwrap();
            }
            let saved_sp = fd.new_varnode_out(width, &sp_address, saved).unwrap();
            fd.op_insert_end(saved, blocks[3]);
            let restore = fd.new_op(1, fd.get_address().clone());
            fd.op_set_opcode_code(restore, OpCode::CPUI_COPY);
            fd.op_set_input(restore, saved_sp, 0).unwrap();
            let restored_sp = fd.new_varnode_out(width, &sp_address, restore).unwrap();
            fd.op_insert_end(restore, blocks[6]);
            let carried = fd.new_op(2, fd.get_address().clone());
            fd.op_set_opcode_code(carried, OpCode::CPUI_MULTIEQUAL);
            fd.op_set_input(carried, saved_sp, 0).unwrap();
            fd.op_set_input(carried, restored_sp, 1).unwrap();
            let header_sp = fd.new_varnode_out(width, &sp_address, carried).unwrap();
            fd.op_insert_end(carried, blocks[4]);
            let current = allocation(&fd, current.unwrap()).unwrap();
            // Neither predecessor reservation dominates this loop. Restoring
            // their PHI is not carrying the current reservation's lifetime,
            // even though all three calculate the same stack address.
            assert!(loop_restores_stack(&fd, &current));
            fd.op_set_input(restore, current.output, 0).unwrap();
            assert!(
                loop_restores_stack(&fd, &current),
                "recomputing from a fixed older base restores storage before the next allocation"
            );
            fd.op_set_input(current.op, header_sp, 0).unwrap();
            let current = allocation(&fd, current.op).unwrap();
            assert!(!loop_restores_stack(&fd, &current));

            let add = fd.new_op(2, fd.get_address().clone());
            fd.op_set_opcode_code(add, OpCode::CPUI_INT_ADD);
            fd.op_set_input(add, current.output, 0).unwrap();
            fd.op_set_input(add, amount, 1).unwrap();
            let restored = fd.new_unique_out(width, add).unwrap();
            fd.op_insert_before(add, restore);
            fd.op_set_input(restore, restored, 0).unwrap();
            assert!(
                loop_restores_stack(&fd, &current),
                "adding the allocation amount ends its lifetime"
            );
            let negative = fd.new_op(1, fd.get_address().clone());
            fd.op_set_opcode_code(negative, OpCode::CPUI_INT_2COMP);
            fd.op_set_input(negative, amount, 0).unwrap();
            let negative_amount = fd.new_unique_out(width, negative).unwrap();
            fd.op_insert_before(negative, restore);
            let subtract = fd.new_op(2, fd.get_address().clone());
            fd.op_set_opcode_code(subtract, OpCode::CPUI_INT_SUB);
            fd.op_set_input(subtract, current.output, 0).unwrap();
            fd.op_set_input(subtract, negative_amount, 1).unwrap();
            let restored = fd.new_unique_out(width, subtract).unwrap();
            fd.op_insert_before(subtract, restore);
            fd.op_set_input(restore, restored, 0).unwrap();
            assert!(
                loop_restores_stack(&fd, &current),
                "subtracting a negated amount also restores storage"
            );

            let copy = fd.new_op(2, fd.get_address().clone());
            fd.op_set_opcode_code(copy, OpCode::CPUI_INT_SUB);
            fd.op_set_input(copy, header_sp, 0).unwrap();
            fd.op_set_input(copy, amount, 1).unwrap();
            let copied = fd.new_unique_out(width, copy).unwrap();
            fd.op_insert_before(copy, restore);
            fd.op_set_input(restore, copied, 0).unwrap();
            assert!(
                !loop_restores_stack(&fd, &current),
                "a dominated recomputation still carries the current reservation"
            );

            fd.bblocks_mut().add_edge(blocks[6], blocks[6]);
            let cycle = fd.new_op(2, fd.get_address().clone());
            fd.op_set_opcode_code(cycle, OpCode::CPUI_MULTIEQUAL);
            fd.op_set_input(cycle, current.output, 0).unwrap();
            fd.op_set_input(cycle, restored_sp, 1).unwrap();
            let cyclic = fd.new_unique_out(width, cycle).unwrap();
            fd.op_insert_before(cycle, restore);
            fd.op_set_input(restore, cyclic, 0).unwrap();
            assert!(
                !loop_restores_stack(&fd, &current),
                "an anchored nested zero-step cycle retains storage"
            );
            let unknown = fd.new_op(2, fd.get_address().clone());
            fd.op_set_opcode_code(unknown, OpCode::CPUI_MULTIEQUAL);
            let unanchored = fd.new_unique_out(width, unknown).unwrap();
            fd.op_set_input(unknown, unanchored, 0).unwrap();
            fd.op_set_input(unknown, unanchored, 1).unwrap();
            fd.op_insert_before(unknown, cycle);
            fd.op_set_input(cycle, unanchored, 1).unwrap();
            assert!(
                loop_restores_stack(&fd, &current),
                "an unanchored cyclic PHI arm is not harmless bottom"
            );

            fd.bblocks_mut().add_edge(blocks[4], blocks[6]);
            fd.bblocks_mut().calc_forward_dominator(root, &[blocks[0]]);
            let bypass = fd.new_op(3, fd.get_address().clone());
            fd.op_set_opcode_code(bypass, OpCode::CPUI_MULTIEQUAL);
            let bypassed = fd.new_unique_out(width, bypass).unwrap();
            fd.op_set_input(bypass, current.output, 0).unwrap();
            fd.op_set_input(bypass, bypassed, 1).unwrap();
            fd.op_set_input(bypass, header_sp, 2).unwrap();
            fd.op_insert_before(bypass, restore);
            fd.op_set_input(restore, bypassed, 0).unwrap();
            assert!(
                !loop_restores_stack(&fd, &current),
                "iterations bypassing a reservation retain storage from earlier iterations"
            );
            fd.op_set_input(bypass, saved_sp, 2).unwrap();
            assert!(
                loop_restores_stack(&fd, &current),
                "a bypass restoring the preallocation value still ends storage"
            );

            fd.op_set_input(bypass, header_sp, 2).unwrap();
            let call = fd.new_op(1, fd.get_address().clone());
            fd.op_set_opcode_code(call, OpCode::CPUI_CALL);
            let target = fd.new_constant(width, 0x3000);
            fd.op_set_input(call, target, 0).unwrap();
            fd.op_insert_before(call, restore);
            let mut spec = crate::fspec::FuncCallSpecs::new(call, Address::new_invalid());
            spec.proto_mut().attach_internal_store(Rc::new(crate::dtype::Datatype::new(
                0, crate::dtype::type_metatype::TYPE_VOID,
            )));
            spec.proto_mut().set_extra_pop(0);
            let index = fd.push_call_specs(spec);
            let effect = fd.new_varnode_iop(call);
            let indirect = fd.new_op(2, fd.get_address().clone());
            fd.op_set_opcode_code(indirect, OpCode::CPUI_INDIRECT);
            fd.op_set_input(indirect, bypassed, 0).unwrap();
            fd.op_set_input(indirect, effect, 1).unwrap();
            let after_call = fd.new_varnode_out(width, &sp_address, indirect).unwrap();
            fd.op_insert_before(indirect, restore);
            fd.op_set_input(restore, after_call, 0).unwrap();
            assert!(
                !loop_restores_stack(&fd, &current),
                "a known zero stack-pop preserves the allocation"
            );
            fd.get_call_specs_mut(index).proto_mut().set_extra_pop(width);
            assert!(
                loop_restores_stack(&fd, &current),
                "callee stack cleanup can restore storage above the allocation"
            );
            fd.get_call_specs_mut(index)
                .proto_mut()
                .set_extra_pop(crate::fspec::EXTRAPOP_UNKNOWN);
            assert!(
                loop_restores_stack(&fd, &current),
                "unknown call stack cleanup cannot prove storage retention"
            );

        }
    }
    #[test]
    fn checkpoint_carriers_distinguish_later_sp_adjustments_from_data_uses() {
        for width in [4, 8] {
            for case in ["outside_sp", "inside_sp", "outside_data"] {
                let (mut fd, allocation, end, block, _, _) = checkpoint_call_fixture(width, false, false);
                let restored = fd.obank().get(end.restore).unwrap().get_in(0).unwrap();
                let adjust = fd.new_op(2, fd.get_address().clone());
                fd.op_set_opcode_code(adjust, OpCode::CPUI_INT_ADD);
                fd.op_set_input(adjust, restored, 0).unwrap();
                let amount = fd.new_constant(width, width as u64);
                fd.op_set_input(adjust, amount, 1).unwrap();
                let result = if case == "outside_data" {
                    fd.new_unique_out(width, adjust).unwrap()
                } else {
                    let sp = fd.vbank().get(allocation.output).unwrap().get_addr().clone();
                    fd.new_varnode_out(width, &sp, adjust).unwrap()
                };
                let parent = if case == "inside_sp" { block } else { end.boundary.unwrap() };
                fd.op_insert_end(adjust, parent);
                if case == "outside_data" {
                    let store = fd.new_op(3, fd.get_address().clone());
                    fd.op_set_opcode_code(store, OpCode::CPUI_STORE);
                    let space = fd.new_constant(4, 4);
                    let address = fd.new_constant(width, 0x8000);
                    fd.op_set_input(store, space, 0).unwrap();
                    fd.op_set_input(store, address, 1).unwrap();
                    fd.op_set_input(store, result, 2).unwrap();
                    fd.op_insert_end(store, parent);
                }
                assert!(restores_checkpoint(&mut fd, &allocation, &end),
                    "width={width}, case={case}");
                // Value equality survives a later adjustment, but an adjustment
                // before the boundary must still pass the lifetime preflight.
                assert_eq!(restoration_failure_before(&fd, &allocation, &[], Some(&end), &FrameFacts::infer(&fd,vec![allocation]).unwrap()).is_some(),
                    case == "inside_sp", "width={width}, case={case}");
            }
        }
    }
    #[test]
    fn raw_pointer_spills_validate_their_own_loop_uses() {
        for width in [4, 8] {
            for change in ["owned", "word_clobber", "partial", "early_partial", "conditional_same", "conditional_clobber", "escaped_before", "uninitialized"] {
                let (mut fd, root, _, body) = scoped_lifetime_fixture(width, false);
                let stack = Rc::clone(fd.get_arch().manage().get_stack_space().unwrap());
                let memory = Rc::clone(stack.get_contain().unwrap());
                let entry = fd.find_spacebase_input(&stack).unwrap();
                let point = fd.get_address().clone();
                let address = fd.new_op(2, point.clone());
                fd.op_set_opcode_code(address, OpCode::CPUI_INT_ADD);
                let offset = fd.new_constant(width, (-32_i64) as u64 & calc_mask(width));
                fd.op_set_all_input(address, &[entry, offset]).unwrap();
                let pointer = fd.new_unique_out(width, address).unwrap();
                fd.op_insert_before(address, root.op);
                let space = fd.new_constant(4, memory.get_index() as u64);
                if change == "escaped_before" {
                    let copy = fd.new_op(1, point.clone());
                    fd.op_set_opcode_code(copy, OpCode::CPUI_COPY);
                    fd.op_set_input(copy, pointer, 0).unwrap();
                    let value = fd.new_varnode_out(width, &Address::new(Rc::clone(&memory), 0x9000), copy).unwrap();
                    fd.vbank_mut().get_mut(value).unwrap().mark_mapped_addr_tied();
                    fd.op_insert_before(copy, root.op);
                }
                if change != "uninitialized" {
                    let save = fd.new_op(3, point.clone());
                    fd.op_set_opcode_code(save, OpCode::CPUI_STORE);
                    fd.op_set_all_input(save, &[space, pointer, root.output]).unwrap();
                    fd.op_insert_after(save, root.op);
                }
                let load = fd.new_op(2, point.clone());
                fd.op_set_opcode_code(load, OpCode::CPUI_LOAD);
                fd.op_set_all_input(load, &[space, pointer]).unwrap();
                let value = fd.new_unique_out(width, load).unwrap();
                fd.op_insert_begin(load, body);
                let write = fd.new_op(3, point.clone());
                fd.op_set_opcode_code(write, OpCode::CPUI_STORE);
                let byte = fd.new_constant(1, 7);
                fd.op_set_all_input(write, &[space, value, byte]).unwrap();
                fd.op_insert_end(write, body);
                if matches!(change, "conditional_same" | "conditional_clobber") {
                    let graph = fd.bblocks_root_pub();
                    let branch = fd.bblocks_mut().new_block_basic(graph);
                    fd.bblocks_mut().block_mut(branch).set_index(4);
                    fd.bblocks_mut().add_edge(body, branch);
                    fd.bblocks_mut().add_edge(branch, body);
                    let entry_block = fd.obank().get(root.op).unwrap().get_parent().unwrap();
                    fd.bblocks_mut().calc_forward_dominator(graph, &[entry_block]);
                    let store = fd.new_op(3, point.clone());
                    fd.op_set_opcode_code(store, OpCode::CPUI_STORE);
                    let payload = if change == "conditional_same" { root.output } else { fd.new_constant(width, 0) };
                    fd.op_set_all_input(store, &[space, pointer, payload]).unwrap();
                    fd.op_insert_end(store, branch);
                }
                if matches!(change, "word_clobber" | "partial" | "early_partial") {
                    let target = if matches!(change, "partial" | "early_partial") {
                        let add = fd.new_op(2, point.clone());
                        fd.op_set_opcode_code(add, OpCode::CPUI_INT_ADD);
                        let one = fd.new_constant(width, 1);
                        fd.op_set_all_input(add, &[pointer, one]).unwrap();
                        let target = fd.new_unique_out(width, add).unwrap();
                        if change == "early_partial" { fd.op_insert_before(add, load); }
                        else { fd.op_insert_end(add, body); }
                        target
                    } else { pointer };
                    let clobber = fd.new_op(3, point);
                    fd.op_set_opcode_code(clobber, OpCode::CPUI_STORE);
                    let zero = fd.new_constant(if matches!(change, "partial" | "early_partial") { 1 } else { width }, 0);
                    fd.op_set_all_input(clobber, &[space, target, zero]).unwrap();
                    if change == "early_partial" { fd.op_insert_before(clobber, load); }
                    else { fd.op_insert_end(clobber, body); }
                }
                let restored = saved_stack_value(&fd, value, &mut 65536, &mut SavedFrameQuery::default())
                    .and_then(|saved| saved.value).and_then(|value| native_stack_delta(&fd, value, root.output));
                assert_eq!(restored == Some(0), matches!(change, "owned" | "conditional_same"), "width={width}, change={change}");
            }
        }
    }

    #[test]
    fn memory_state_markers_are_not_source_pointer_escapes() {
        for width in [4, 8] {
            for change in ["markers", "global_escape", "call_escape", "clobber", "creation"] {
                let (mut fd, allocation, end, _, index, _) = checkpoint_call_fixture(width, false, false);
                let stack = Rc::clone(fd.get_arch().manage().get_stack_space().unwrap());
                let memory = Rc::clone(stack.get_contain().unwrap());
                let point = fd.get_address().clone();
                let call = fd.get_call_specs(index).get_op();
                let global = Address::new(Rc::clone(&memory), 0x8000);
                let save_global = fd.new_op(1, point.clone());
                fd.op_set_opcode_code(save_global, OpCode::CPUI_COPY);
                fd.op_set_input(save_global, allocation.output, 0).unwrap();
                let initial = fd.new_varnode_out(width, &global, save_global).unwrap();
                fd.op_insert_before(save_global, call);
                fd.vbank_mut().get_mut(initial).unwrap().mark_mapped_addr_tied();
                assert!(fd.vbank().get(initial).unwrap().is_addr_tied());
                let store = fd.new_op(3, point.clone());
                fd.op_set_opcode_code(store, OpCode::CPUI_STORE);
                let space = fd.new_constant(4, memory.get_index() as u64);
                let byte = fd.new_constant(1, 7);
                fd.op_set_all_input(store, &[space, allocation.output, byte]).unwrap();
                fd.op_insert_before(store, call);
                let mut previous = initial;
                for effect in [store, call] {
                    let marker = fd.new_op(2, point.clone());
                    fd.op_set_opcode_code(marker, OpCode::CPUI_INDIRECT);
                    let reference = fd.new_varnode_iop(effect);
                    fd.op_set_all_input(marker, &[previous, reference]).unwrap();
                    previous = fd.new_varnode_out(width, &global, marker).unwrap();
                    fd.op_insert_before(marker, effect);
                    fd.vbank_mut().get_mut(previous).unwrap().mark_mapped_addr_tied();
                }
                if change != "markers" {
                    let address = fd.new_op(2, point.clone());
                    fd.op_set_opcode_code(address, OpCode::CPUI_INT_ADD);
                    let entry = fd.find_spacebase_input(&stack).unwrap();
                    let offset = fd.new_constant(width, (-32_i64) as u64 & calc_mask(width));
                    fd.op_set_all_input(address, &[entry, offset]).unwrap();
                    let pointer = fd.new_unique_out(width, address).unwrap();
                    fd.op_insert_before(address, call);
                    match change {
                        "global_escape" => {
                            let escape = fd.new_op(1, point.clone());
                            fd.op_set_opcode_code(escape, OpCode::CPUI_COPY);
                            fd.op_set_input(escape, pointer, 0).unwrap();
                            let escaped = fd.new_varnode_out(width, &global, escape).unwrap();
                            fd.vbank_mut().get_mut(escaped).unwrap().mark_mapped_addr_tied();
                            fd.op_insert_before(escape, call);
                        }
                        "call_escape" => { fd.op_set_input(call, pointer, 1).unwrap(); }
                        "creation" => {
                            let marker = fd.new_op(2, point.clone());
                            fd.op_set_opcode_code(marker, OpCode::CPUI_INDIRECT);
                            let zero = fd.new_constant(width, 0);
                            let effect = fd.new_varnode_iop(call);
                            fd.op_set_all_input(marker, &[zero, effect]).unwrap();
                            let address = Address::new(Rc::clone(&stack), (-32_i64) as u64 & calc_mask(width));
                            fd.new_varnode_out(width, &address, marker).unwrap();
                            fd.mark_indirect_creation(marker, false).unwrap();
                            fd.op_insert_before(marker, call);
                        }
                        "clobber" => {
                            let overwrite = fd.new_op(3, point.clone());
                            fd.op_set_opcode_code(overwrite, OpCode::CPUI_STORE);
                            let zero = fd.new_constant(width, 0);
                            fd.op_set_all_input(overwrite, &[space, pointer, zero]).unwrap();
                            fd.op_insert_before(overwrite, call);
                        }
                        _ => unreachable!(),
                    }
                }
                let facts=FrameFacts::infer(&fd,vec![allocation]).unwrap();
                assert_eq!(facts.equations.delta(&fd,end.value,allocation.before)==Some(0), change == "markers",
                    "width={width}, change={change}");
            }
        }
    }

    #[test]
    fn checkpoint_rejects_clobbers_but_keeps_observed_saved_values() {
        for width in [4, 8] {
            for case in ["different_seed", "partial_write", "unrelated_store", "source_data", "later_source_data",
                "address_escape", "unknown_pop", "active_inputs", "unknown_effect"] {
                let (mut fd, allocation, end, block, index, saved) = checkpoint_call_fixture(width, false, false);
                let restored = fd.obank().get(end.restore).unwrap().get_in(0).unwrap();
                let indirect = fd.vbank().get(restored).unwrap().get_def().unwrap();
                match case {
                    "different_seed" => {
                        let save = fd.vbank().get(saved).unwrap().get_def().unwrap();
                        fd.op_set_input(save, allocation.amount, 0).unwrap();
                    }
                    "partial_write" | "unrelated_store" | "source_data" | "later_source_data" => {
                        let store = fd.new_op(3, fd.get_address().clone());
                        fd.op_set_opcode_code(store, OpCode::CPUI_STORE);
                        let space = fd.new_constant(4, 4);
                        fd.op_set_input(store, space, 0).unwrap();
                        let pointer = if case == "partial_write" {
                            let stack = fd.get_arch().manage().get_stack_space().unwrap();
                            let input = fd.find_spacebase_input(&stack).unwrap();
                            let address = fd.new_op(2, fd.get_address().clone());
                            fd.op_set_opcode_code(address, OpCode::CPUI_INT_ADD);
                            fd.op_set_input(address, input, 0).unwrap();
                            let offset = fd.new_constant(width, (-33_i64) as u64 & calc_mask(width));
                            fd.op_set_input(address, offset, 1).unwrap();
                            let pointer = fd.new_unique_out(width, address).unwrap();
                            fd.op_insert_before(address, indirect);
                            pointer
                        } else {
                            fd.new_constant(width, 0x8000)
                        };
                        fd.op_set_input(store, pointer, 1).unwrap();
                        let data = if matches!(case, "source_data" | "later_source_data") {
                            restored
                        } else {
                            fd.new_constant(width, 0)
                        };
                        fd.op_set_input(store, data, 2).unwrap();
                        if case == "later_source_data" {
                            fd.op_insert_end(store, end.boundary.unwrap());
                        } else {
                            fd.op_insert_after(store, indirect);
                        }
                        if case == "unrelated_store" {
                            let effect = fd.new_varnode_iop(store);
                            fd.op_set_input(indirect, effect, 1).unwrap();
                        }
                    }
                    "unknown_effect" => {
                        let effect=fd.new_op(1,fd.get_address().clone());
                        fd.op_set_opcode_code(effect,OpCode::CPUI_CALLOTHER);
                        let userop=fd.new_constant(4,9);
                        fd.op_set_input(effect,userop,0).unwrap();
                        fd.op_insert_after(effect,indirect);
                        let reference=fd.new_varnode_iop(effect);
                        fd.op_set_input(indirect,reference,1).unwrap();
                    }
                    "address_escape" => {
                        let stack = fd.get_arch().manage().get_stack_space().unwrap();
                        let input = fd.find_spacebase_input(&stack).unwrap();
                        let address = fd.new_op(2, fd.get_address().clone());
                        fd.op_set_opcode_code(address, OpCode::CPUI_INT_ADD);
                        fd.op_set_input(address, input, 0).unwrap();
                        let offset = fd.new_constant(width, (-32_i64) as u64 & calc_mask(width));
                        fd.op_set_input(address, offset, 1).unwrap();
                        let pointer = fd.new_unique_out(width, address).unwrap();
                        fd.op_insert_begin(address, block);
                        let call = fd.get_call_specs(index).get_op();
                        fd.op_set_input(call, pointer, 1).unwrap();
                    }
                    "unknown_pop" => fd.get_call_specs_mut(index).proto_mut().set_extra_pop(crate::fspec::EXTRAPOP_UNKNOWN),
                    "active_inputs" => fd.get_call_specs_mut(index).init_active_input(),
                    _ => unreachable!(),
                }
                assert_eq!(restores_checkpoint(&mut fd, &allocation, &end),
                    matches!(case, "source_data" | "later_source_data" | "unrelated_store"), "width={width}, case={case}");

            }
        }
    }
    #[test]
    fn provisional_checkpoint_values_do_not_discharge_writes_or_address_escapes() {
        for width in [4, 8] {
            for change in ["whole_write", "partial_write", "address_escape", "adjacent_write"] {
                let (mut fd, allocation, end, block, _, _) = checkpoint_call_fixture(width, false, false);
                let restored = fd.obank().get(end.restore).unwrap().get_in(0).unwrap();
                let marker = fd.vbank().get(restored).unwrap().get_def().unwrap();
                let address = fd.new_op(2, fd.get_address().clone());
                fd.op_set_opcode_code(address, OpCode::CPUI_INT_ADD);
                let delta = match change {
                    "partial_write" => 31,
                    "adjacent_write" => 32 + width as u64,
                    _ => 32,
                };
                let offset = fd.new_constant(width, delta);
                fd.op_set_all_input(address, &[restored, offset]).unwrap();
                let pointer = fd.new_unique_out(width, address).unwrap();
                fd.op_insert_after(address, marker);
                let store = fd.new_op(3, fd.get_address().clone());
                fd.op_set_opcode_code(store, OpCode::CPUI_STORE);
                let memory = fd.new_constant(4, 4);
                let (target, data) = if change == "address_escape" {
                    (fd.new_constant(width, 0x8000), pointer)
                } else {
                    (pointer, fd.new_constant(width, 0))
                };
                fd.op_set_all_input(store, &[memory, target, data]).unwrap();
                fd.op_insert_after(store, address);
                assert_eq!(restores_checkpoint(&mut fd, &allocation, &end), change == "adjacent_write",
                    "width={width}, change={change}, block={block:?}");
            }
        }
    }

    #[test]
    fn shared_checkpoint_renamings_do_not_exhaust_the_value_proof() {
        for width in [4, 8] {
            let (mut fd, allocation, _, _) = scoped_lifetime_fixture(width, false);
            let graph = fd.bblocks_root_pub();
            let mut parent = fd.obank().get(allocation.op).unwrap().get_parent().unwrap();
            let mut value = allocation.base;
            for depth in 0..20 {
                let blocks: Vec<_> = (0..3).map(|index| {
                    let block = fd.bblocks_mut().new_block_basic(graph);
                    fd.bblocks_mut().block_mut(block).set_index(4 + depth * 3 + index);
                    block
                }).collect();
                for (from, to) in [(parent, blocks[0]), (parent, blocks[1]),
                    (blocks[0], blocks[2]), (blocks[1], blocks[2])]
                { fd.bblocks_mut().add_edge(from, to); }
                let mut incoming = Vec::new();
                for &block in &blocks[..2] {
                    let copy = fd.new_op(1, fd.get_address().clone());
                    fd.op_set_opcode_code(copy, OpCode::CPUI_COPY);
                    fd.op_set_input(copy, value, 0).unwrap();
                    incoming.push(fd.new_unique_out(width, copy).unwrap());
                    fd.op_insert_end(copy, block);
                }
                let phi = fd.new_op(2, fd.get_address().clone());
                fd.op_set_opcode_code(phi, OpCode::CPUI_MULTIEQUAL);
                fd.op_set_all_input(phi, &incoming).unwrap();
                value = fd.new_unique_out(width, phi).unwrap();
                fd.op_insert_begin(phi, blocks[2]);
                parent = blocks[2];
            }
            let restored = saved_stack_value(&fd, value, &mut 65536, &mut SavedFrameQuery::default())
                .and_then(|saved| saved.value);
            assert_eq!(restored.and_then(|value| native_stack_delta(&fd, value, allocation.base)), Some(0),
                "width={width}: branch-specific renamings preserve the checkpoint");
        }
    }

    #[test]
    fn failed_checkpoint_cycles_do_not_leave_provisional_child_values() {
        for width in [4, 8] {
            for changed in [false, true] {
                let (mut fd, allocation, _, _) = scoped_lifetime_fixture(width, false);
                let graph = fd.bblocks_root_pub();
                let entry = fd.obank().get(allocation.op).unwrap().get_parent().unwrap();
                let blocks: Vec<_> = (0..3).map(|index| {
                    let block = fd.bblocks_mut().new_block_basic(graph);
                    fd.bblocks_mut().block_mut(block).set_index(4 + index);
                    block
                }).collect();
                let (header, alternate, join) = (blocks[0], blocks[1], blocks[2]);
                for (from, to) in [(entry, header), (header, join), (header, alternate),
                    (alternate, join), (join, header)]
                { fd.bblocks_mut().add_edge(from, to); }
                let first = fd.new_op(2, fd.get_address().clone());
                fd.op_set_opcode_code(first, OpCode::CPUI_MULTIEQUAL);
                let first_value = fd.new_unique_out(width, first).unwrap();
                fd.op_insert_begin(first, header);
                let copy = fd.new_op(1, fd.get_address().clone());
                fd.op_set_opcode_code(copy, OpCode::CPUI_COPY);
                fd.op_set_input(copy, first_value, 0).unwrap();
                let child = fd.new_unique_out(width, copy).unwrap();
                fd.op_insert_end(copy, header);
                let shift = fd.new_op(2, fd.get_address().clone());
                fd.op_set_opcode_code(shift, OpCode::CPUI_INT_ADD);
                let offset = fd.new_constant(width, if changed { width as u64 } else { 0 });
                fd.op_set_all_input(shift, &[allocation.base, offset]).unwrap();
                let other = fd.new_unique_out(width, shift).unwrap();
                fd.op_insert_end(shift, alternate);
                let second = fd.new_op(2, fd.get_address().clone());
                fd.op_set_opcode_code(second, OpCode::CPUI_MULTIEQUAL);
                fd.op_set_all_input(second, &[child, other]).unwrap();
                let second_value = fd.new_unique_out(width, second).unwrap();
                fd.op_insert_begin(second, join);
                fd.op_set_all_input(first, &[allocation.base, second_value]).unwrap();
                let mut proof = SavedFrameQuery::default();
                let mut budget = 65536;
                let first_result = saved_stack_value(&fd, first_value, &mut budget, &mut proof)
                    .and_then(|saved| saved.value);
                let child_result = saved_stack_value(&fd, child, &mut budget, &mut proof)
                    .and_then(|saved| saved.value);
                for result in [first_result, child_result] {
                    assert_eq!(result.and_then(|value| native_stack_delta(&fd, value, allocation.base)),
                        (!changed).then_some(0), "width={width}, changed={changed}");
                }
            }
        }
    }


    #[test]
    fn folded_reservation_prefix_keeps_the_real_restore_and_full_backing() {
        for width in [4, 8] {
            for prefix in [8_u64, 24, 272] {
                let (mut fd, mut allocation, end, _) = scoped_lifetime_fixture(width, false);
                let access = fd.new_op(2, fd.get_address().clone());
                fd.op_set_opcode_code(access, OpCode::CPUI_INT_ADD);
                let offset = fd.new_constant(width, 32);
                fd.op_set_all_input(access, &[allocation.output, offset]).unwrap();
                let pointer = fd.new_unique_out(width, access).unwrap();
                fd.op_insert_before(access, end.restore);
                let store = fd.new_op(3, fd.get_address().clone());
                fd.op_set_opcode_code(store, OpCode::CPUI_STORE);
                let space = fd.new_constant(4, fd.get_arch().manage().get_space_by_name("ram").unwrap().get_index() as u64);
                let byte = fd.new_constant(1, 0x5a);
                fd.op_set_all_input(store, &[space, pointer, byte]).unwrap();
                fd.op_insert_before(store, end.restore);
                let baseline = reserves(&fd, &[allocation], &[], &[], width).unwrap()[0].count;
                let old_base = allocation.base;
                let folded = fd.new_op(2, fd.obank().get(allocation.op).unwrap().get_addr().clone());
                fd.op_set_opcode_code(folded, OpCode::CPUI_INT_ADD);
                let delta = fd.new_constant(width, (0_u64.wrapping_sub(prefix)) & calc_mask(width));
                fd.op_set_all_input(folded, &[old_base, delta]).unwrap();
                let base = fd.new_unique_out(width, folded).unwrap();
                fd.op_insert_before(folded, allocation.op);
                fd.op_set_input(allocation.op, base, 0).unwrap();
                allocation.base = base;
                allocation.before = base;
                assert!(proved_restore(&fd, &allocation, end.value).is_none());
                normalize_reservation(&fd, &mut allocation);
                assert_eq!(proved_restore(&fd, &allocation, end.value).map(|end| end.restore), Some(end.restore));
                let reserve = reserves(&fd, &[allocation], &[], &[], width).unwrap()[0];
                assert_eq!(reserve.count, baseline.max((prefix + 15) & !15));
            }
        }
    }

    #[test]
    fn unrelated_or_upward_arithmetic_does_not_adopt_a_saved_native_top() {
        for width in [4, 8] {
            for unrelated in [false, true] {
                let (mut fd, mut allocation, end, _) = scoped_lifetime_fixture(width, false);
                let old_base = allocation.base;
                let folded = fd.new_op(2, fd.obank().get(allocation.op).unwrap().get_addr().clone());
                fd.op_set_opcode_code(folded, OpCode::CPUI_INT_ADD);
                let delta = fd.new_constant(width, if unrelated { calc_mask(width) - 7 } else { 8 });
                fd.op_set_all_input(folded, &[old_base, delta]).unwrap();
                let base = fd.new_unique_out(width, folded).unwrap();
                fd.op_insert_before(folded, allocation.op);
                fd.op_set_input(allocation.op, base, 0).unwrap();
                allocation.base = base;
                allocation.before = base;
                if unrelated {
                    let address = Address::new(Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap()), 0x2400);
                    let other = fd.new_varnode(width, &address, None);
                    allocation.amount = fd.set_input_varnode(other).unwrap();
                }
                normalize_reservation(&fd, &mut allocation);
                assert!(proved_restore(&fd, &allocation, end.value).is_none());
            }
        }
    }

    #[test]
    fn folded_reservation_recovers_across_empty_single_predecessors() {
        for width in [4, 8] {
            let (mut fd, mut allocation, end, transit) = scoped_lifetime_fixture(width, false);
            fd.bblocks_mut().remove_edge(transit, transit);
            fd.op_uninsert(allocation.op);
            fd.op_insert_before(allocation.op, end.restore);
            let graph = fd.bblocks_root_pub();
            let entry = fd.obank().get(fd.vbank().get(allocation.base).unwrap().get_def().unwrap())
                .unwrap().get_parent().unwrap();
            fd.bblocks_mut().calc_forward_dominator(graph, &[entry]);
            let folded = fd.new_op(2, fd.obank().get(allocation.op).unwrap().get_addr().clone());
            fd.op_set_opcode_code(folded, OpCode::CPUI_INT_ADD);
            let delta = fd.new_constant(width, calc_mask(width) - 23);
            fd.op_set_all_input(folded, &[allocation.base, delta]).unwrap();
            let base = fd.new_unique_out(width, folded).unwrap();
            fd.op_insert_before(folded, allocation.op);
            fd.op_set_input(allocation.op, base, 0).unwrap();
            allocation.base = base;
            allocation.before = base;
            normalize_reservation(&fd, &mut allocation);
            assert_eq!(proved_restore(&fd, &allocation, end.value).map(|end| end.restore), Some(end.restore));
            assert_eq!(reserves(&fd, &[allocation], &[], &[], width).unwrap()[0].count, 32);
        }
    }

    #[test]
    fn folded_later_reservations_cannot_hide_partial_restoration() {
        for width in [4, 8] {
            for partial_restore in [false, true] {
                let (mut fd, first, _, _) = scoped_lifetime_fixture(width, false);
                let address = fd.obank().get(first.op).unwrap().get_addr().clone();
                let top = fd.new_op(2, address.clone());
                fd.op_set_opcode_code(top, OpCode::CPUI_INT_ADD);
                let delta = fd.new_constant(width, if partial_restore { 64 } else { 0 });
                fd.op_set_all_input(top, &[first.output, delta]).unwrap();
                let before = fd.new_unique_out(width, top).unwrap();
                fd.op_insert_after(top, first.op);
                let fold = fd.new_op(2, address.clone());
                fd.op_set_opcode_code(fold, OpCode::CPUI_INT_ADD);
                let delta = fd.new_constant(width, calc_mask(width) - 271);
                fd.op_set_all_input(fold, &[before, delta]).unwrap();
                let base = fd.new_unique_out(width, fold).unwrap();
                fd.op_insert_after(fold, top);
                let op = fd.new_op(2, address);
                fd.op_set_opcode_code(op, OpCode::CPUI_INT_SUB);
                fd.op_set_all_input(op, &[base, first.amount]).unwrap();
                let output = fd.new_unique_out(width, op).unwrap();
                fd.op_insert_after(op, fold);
                let later = Allocation { scale: 1, op, output, amount: first.amount, base, before };
                assert_eq!(sequential_reservations_survive(&fd, &[first, later]), !partial_restore);
            }
        }
    }

    #[test]
    fn reservation_tops_cross_only_declared_zero_pop_call_witnesses() {
        use crate::fspec::{EffectRecord, FuncCallSpecs};
        for width in [4, 8] {
            for mode in 0..4 {
                let (mut fd, mut allocation, end, _) = scoped_lifetime_fixture(width, false);
                let old_base = allocation.base;
                let address = Address::new(Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap()), 0x1008);
                let call = fd.new_op(1, address);
                fd.op_set_opcode_code(call, if mode == 3 { OpCode::CPUI_CALLOTHER } else { OpCode::CPUI_CALL });
                let target = fd.new_constant(width, 0x3000);
                fd.op_set_input(call, target, 0).unwrap();
                fd.op_insert_before(call, allocation.op);
                if mode != 3 {
                    let mut spec = FuncCallSpecs::new(call, Address::new_invalid());
                    spec.proto_mut().set_extra_pop(if mode == 1 { width } else { 0 });
                    if mode == 2 {
                        spec.proto_mut().push_effect_override(EffectRecord::new_unknown(fd.vbank().get(old_base).unwrap().get_addr(), width));
                    }
                    fd.push_call_specs(spec);
                    fd.kuna_stack_call_sp.insert(call, Some(old_base));
                }
                let folded = fd.new_op(2, fd.obank().get(allocation.op).unwrap().get_addr().clone());
                fd.op_set_opcode_code(folded, OpCode::CPUI_INT_ADD);
                let delta = fd.new_constant(width, calc_mask(width) - 23);
                fd.op_set_all_input(folded, &[old_base, delta]).unwrap();
                let base = fd.new_unique_out(width, folded).unwrap();
                fd.op_insert_before(folded, allocation.op);
                fd.op_set_input(allocation.op, base, 0).unwrap();
                allocation.base = base;
                allocation.before = base;
                normalize_reservation(&fd, &mut allocation);
                assert_eq!(proved_restore(&fd, &allocation, end.value).is_some(), mode == 0);
            }
        }
    }

    #[test]
    fn scalar_memory_cycles_do_not_become_native_frame_sources() {
        for (width, scalar_width) in [(4, 1), (4, 4), (8, 1), (8, 4), (8, 8)] {
            for mode in 0..3 {
                let (mut fd, allocation, _, block) = scoped_lifetime_fixture(width, false);
                let point = fd.get_address().clone();
                let address = fd.new_op(2, point.clone());
                fd.op_set_opcode_code(address, OpCode::CPUI_INT_ADD);
                let offset = fd.new_constant(width, 24);
                fd.op_set_all_input(address, &[allocation.base, offset]).unwrap();
                let pointer = fd.new_unique_out(width, address).unwrap();
                fd.op_insert_before(address, allocation.op);
                let space = fd.new_constant(4, fd.get_arch().manage().get_space_by_name("ram").unwrap().get_index() as u64);
                let initial = fd.new_op(3, point.clone());
                fd.op_set_opcode_code(initial, OpCode::CPUI_STORE);
                let zero = fd.new_constant(scalar_width, 0);
                fd.op_set_all_input(initial, &[space, pointer, zero]).unwrap();
                fd.op_insert_before(initial, allocation.op);
                let load = fd.new_op(2, point.clone());
                fd.op_set_opcode_code(load, OpCode::CPUI_LOAD);
                fd.op_set_all_input(load, &[space, pointer]).unwrap();
                let value = fd.new_unique_out(scalar_width, load).unwrap();
                fd.op_insert_end(load, block);
                let step = fd.new_op(2, point.clone());
                fd.op_set_opcode_code(step, OpCode::CPUI_INT_ADD);
                let one = fd.new_constant(scalar_width, 1);
                fd.op_set_all_input(step, &[value, one]).unwrap();
                let next = fd.new_unique_out(scalar_width, step).unwrap();
                fd.op_insert_end(step, block);
                let payload = if mode == 2 {
                    let truncate = fd.new_op(2, point.clone());
                    fd.op_set_opcode_code(truncate, OpCode::CPUI_SUBPIECE);
                    let offset = fd.new_constant(4, 0);
                    fd.op_set_all_input(truncate, &[allocation.before, offset]).unwrap();
                    let byte = fd.new_unique_out(1, truncate).unwrap();
                    fd.op_insert_end(truncate, block);
                    byte
                } else if mode == 1 { allocation.before } else { next };
                let store = fd.new_op(3, point);
                fd.op_set_opcode_code(store, OpCode::CPUI_STORE);
                fd.op_set_all_input(store, &[space, pointer, payload]).unwrap();
                fd.op_insert_end(store, block);
                assert_eq!(frame_dependent(&fd, value, &mut 4096, FrameWalk::Values, &mut SavedFrameQuery::default()), mode != 0);
            }
        }
    }

    #[test]
    fn spilled_source_pointers_keep_scalar_indices_and_expose_native_indices() {
        for width in [4,8] {
            for spilled_base in [false,true] {
                for native_index in [false,true] {
                    let (mut fd, root, _, body) = scoped_lifetime_fixture(width, false);
                    let point = fd.get_address().clone();
                    let memory = fd.new_constant(4, fd.get_arch().manage().get_stack_space().unwrap().get_contain().unwrap().get_index() as u64);
                    let address = |fd: &mut Funcdata, offset| {
                        let op = fd.new_op(2, fd.get_address().clone());
                        fd.op_set_opcode_code(op, OpCode::CPUI_INT_ADD);
                        let displacement = fd.new_constant(width, offset);
                        fd.op_set_all_input(op, &[root.before, displacement]).unwrap();
                        let pointer = fd.new_unique_out(width, op).unwrap();
                        fd.op_insert_before(op, root.op);
                        pointer
                    };
                    let index_slot = address(&mut fd, 16);
                    let base_slot = address(&mut fd, 32);
                    let result_slot = address(&mut fd, 48);
                    let store = |fd: &mut Funcdata, pointer, value, before| {
                        let op = fd.new_op(3, fd.get_address().clone());
                        fd.op_set_opcode_code(op, OpCode::CPUI_STORE);
                        fd.op_set_all_input(op, &[memory,pointer,value]).unwrap();
                        if before { fd.op_insert_before(op, root.op); }
                        else { fd.op_insert_end(op, body); }
                    };
                    let initial = if native_index { root.before } else { fd.new_constant(width,0) };
                    store(&mut fd, index_slot, initial, true);
                    let load = |fd: &mut Funcdata, pointer| {
                        let op = fd.new_op(2, fd.get_address().clone());
                        fd.op_set_opcode_code(op, OpCode::CPUI_LOAD);
                        fd.op_set_all_input(op, &[memory,pointer]).unwrap();
                        let value = fd.new_unique_out(width, op).unwrap();
                        fd.op_insert_end(op, body);
                        value
                    };
                    let index = load(&mut fd,index_slot);
                    let base = if spilled_base {
                        store(&mut fd,base_slot,root.output,false);
                        load(&mut fd,base_slot)
                    } else { root.output };
                    let add = fd.new_op(3,point.clone());
                    fd.op_set_opcode_code(add,OpCode::CPUI_PTRADD);
                    let stride = fd.new_constant(4,1);
                    fd.op_set_all_input(add,&[base,index,stride]).unwrap();
                    let pointer = fd.new_unique_out(width,add).unwrap();
                    fd.op_insert_end(add,body);
                    store(&mut fd,result_slot,pointer,false);
                    let observed = load(&mut fd,result_slot);
                    if !native_index {
                        let step = fd.new_op(2,point);
                        fd.op_set_opcode_code(step,OpCode::CPUI_INT_ADD);
                        let one = fd.new_constant(width,1);
                        fd.op_set_all_input(step,&[index,one]).unwrap();
                        let next = fd.new_unique_out(width,step).unwrap();
                        fd.op_insert_end(step,body);
                        store(&mut fd,index_slot,next,false);
                    }
                    let mut budget = 65536;
                    assert_eq!(frame_dependent(&fd,observed,&mut budget,FrameWalk::Values,&mut SavedFrameQuery::default()),native_index,
                        "width={width}, spilled_base={spilled_base}, native_index={native_index}, budget={budget}");
                    assert!(budget>0,"source-origin traversal must close its scalar cycle");
                }
            }
        }
    }

    #[test]
    fn indexed_source_writes_close_the_scalar_provenance_cycle() {
        for width in [4,8] {
            for spilled_base in [false,true] {
                for native_index in [false,true] {
                    let (mut fd, root, _, body) = scoped_lifetime_fixture(width, false);
                    let point = fd.get_address().clone();
                    let stack = fd.get_arch().manage().get_stack_space().unwrap();
                    let entry = fd.find_spacebase_input(stack).unwrap();
                    let memory = fd.new_constant(4, fd.get_arch().manage().get_stack_space().unwrap().get_contain().unwrap().get_index() as u64);
                    let address = |fd: &mut Funcdata, offset| {
                        let op = fd.new_op(2, fd.get_address().clone());
                        fd.op_set_opcode_code(op, OpCode::CPUI_INT_ADD);
                        let displacement = fd.new_constant(width, offset);
                        fd.op_set_all_input(op, &[entry, displacement]).unwrap();
                        let pointer = fd.new_unique_out(width, op).unwrap();
                        fd.op_insert_before(op, root.op);
                        pointer
                    };
                    let index_slot = address(&mut fd, (-16_i64) as u64 & calc_mask(width));
                    let base_slot = address(&mut fd, (-24_i64) as u64 & calc_mask(width));
                    let result_slot = address(&mut fd, (-32_i64) as u64 & calc_mask(width));
                    let store = |fd: &mut Funcdata, pointer, value, before| {
                        let op = fd.new_op(3, fd.get_address().clone());
                        fd.op_set_opcode_code(op, OpCode::CPUI_STORE);
                        fd.op_set_all_input(op, &[memory,pointer,value]).unwrap();
                        if before { fd.op_insert_before(op, root.op); }
                        else { fd.op_insert_end(op, body); }
                    };
                    let initial = if native_index { root.before } else { fd.new_constant(width,0) };
                    store(&mut fd, index_slot, initial, true);
                    let load = |fd: &mut Funcdata, pointer| {
                        let op = fd.new_op(2, fd.get_address().clone());
                        fd.op_set_opcode_code(op, OpCode::CPUI_LOAD);
                        fd.op_set_all_input(op, &[memory,pointer]).unwrap();
                        let value = fd.new_unique_out(width, op).unwrap();
                        fd.op_insert_end(op, body);
                        value
                    };
                    let index = load(&mut fd,index_slot);
                    let base = if spilled_base {
                        store(&mut fd,base_slot,root.output,false);
                        load(&mut fd,base_slot)
                    } else { root.output };
                    let add = fd.new_op(3,point.clone());
                    fd.op_set_opcode_code(add,OpCode::CPUI_PTRADD);
                    let stride = fd.new_constant(4,1);
                    fd.op_set_all_input(add,&[base,index,stride]).unwrap();
                    let pointer = fd.new_unique_out(width,add).unwrap();
                    fd.op_insert_end(add,body);
                    let byte = fd.new_constant(1, 7);
                    store(&mut fd, pointer, byte, false);
                    store(&mut fd,result_slot,pointer,false);
                    let observed = load(&mut fd,result_slot);
                    if !native_index {
                        let step = fd.new_op(2,point);
                        fd.op_set_opcode_code(step,OpCode::CPUI_INT_ADD);
                        let one = fd.new_constant(width,1);
                        fd.op_set_all_input(step,&[index,one]).unwrap();
                        let next = fd.new_unique_out(width,step).unwrap();
                        fd.op_insert_end(step,body);
                        store(&mut fd,index_slot,next,false);
                    }
                    let mut budget = 65536;
                    assert_eq!(frame_dependent(&fd,observed,&mut budget,FrameWalk::Values,&mut SavedFrameQuery::default()),native_index,
                        "width={width}, spilled_base={spilled_base}, native_index={native_index}, budget={budget}");
                    assert!(budget>0,"source-origin traversal must close its scalar cycle");
                }
            }
        }
    }

    #[test]
    fn source_pointer_spills_in_substantial_blocks_use_the_shared_proof_budget() {
        for width in [4,8] {
            for mode in ["preserved","clobbered","escaped","header"] {
                let (mut fd,root,end,body,index,saved) = checkpoint_call_fixture(width,false,false);
                let point = fd.get_address().clone();
                let save = fd.vbank().get(saved).unwrap().get_def().unwrap();
                let pointer = fd.new_op(2,point.clone());
                fd.op_set_opcode_code(pointer,OpCode::CPUI_INT_ADD);
                let offset = fd.new_constant(width,if mode=="header" {0} else {16});
                fd.op_set_all_input(pointer,&[root.output,offset]).unwrap();
                let source = fd.new_unique_out(width,pointer).unwrap();
                fd.op_insert_after(pointer,root.op);
                fd.op_uninsert(save);
                fd.op_insert_after(save,pointer);
                fd.op_set_input(save,source,0).unwrap();
                let memory = fd.new_constant(4,fd.get_arch().manage().get_stack_space().unwrap().get_contain().unwrap().get_index() as u64);
                let call = fd.get_call_specs(index).get_op();
                for offset in 0..96 {
                    let store=fd.new_op(3,point.clone());
                    fd.op_set_opcode_code(store,OpCode::CPUI_STORE);
                    let target=fd.new_constant(width,0x8000+offset*width as u64);
                    let value=fd.new_constant(width,offset);
                    fd.op_set_all_input(store,&[memory,target,value]).unwrap();
                    fd.op_insert_before(store,call);
                }
                if mode=="clobbered" || mode=="escaped" {
                    let address=fd.new_op(2,point.clone());
                    fd.op_set_opcode_code(address,OpCode::CPUI_INT_ADD);
                    let stack=fd.get_arch().manage().get_stack_space().unwrap();
                    let entry=fd.find_spacebase_input(stack).unwrap();
                    let offset=fd.new_constant(width,(-32_i64) as u64 & calc_mask(width));
                    fd.op_set_all_input(address,&[entry,offset]).unwrap();
                    let target=fd.new_unique_out(width,address).unwrap();
                    fd.op_insert_before(address,call);
                    if mode=="escaped" { fd.op_set_input(call,target,1).unwrap(); }
                    else {
                        let store=fd.new_op(3,point.clone());
                        fd.op_set_opcode_code(store,OpCode::CPUI_STORE);
                        let zero=fd.new_constant(width,0);
                        fd.op_set_all_input(store,&[memory,target,zero]).unwrap();
                        fd.op_insert_before(store,call);
                    }
                }
                let carried=fd.obank().get(end.restore).unwrap().get_in(0).unwrap();
                let read=fd.new_op(1,point);
                fd.op_set_opcode_code(read,OpCode::CPUI_COPY);
                fd.op_set_input(read,carried,0).unwrap();
                let value=fd.new_unique_out(width,read).unwrap();
                fd.op_insert_end(read,body);
                fd.op_set_input(end.restore,root.before,0).unwrap();
                let mut budget=65536;
                assert_eq!(FrameFacts::infer(&fd,vec![root]).unwrap().header_reaches(&fd,value,1,&root,width,None,&mut budget),mode!="preserved",
                    "width={width}, mode={mode}");
            }
        }
    }



    #[test]
    fn additive_reassociation_preserves_observed_native_intermediates() {
        use crate::action::Rule;
        for width in [4, 8] {
            for frame in [12_u64, 28, 80] {
                for preserve in [false, true] {
                    let mut arch = ArchContext::new(build_manager());
                    arch.stack_alloc = true;
                    let arch = Rc::new(arch);
                    let ram = Rc::clone(arch.manage().get_space_by_name("ram").unwrap());
                    let point = Address::new(Rc::clone(&ram), 0x1000);
                    let mut fd = Funcdata::new("terms", "terms", arch, point.clone(), 0x10000000, 0x40).unwrap();
                    let root = fd.bblocks_root_pub();
                    let block = fd.bblocks_mut().new_block_basic(root);
                    let input = fd.new_varnode(width, &Address::new(ram, 0x2000), None);
                    let input = fd.set_input_varnode(input).unwrap();
                    let inner = fd.new_op(2, point.clone());
                    fd.op_set_opcode_code(inner, OpCode::CPUI_INT_ADD);
                    let displacement = fd.new_constant(width, 0_u64.wrapping_sub(frame) & calc_mask(width));
                    fd.op_set_all_input(inner, &[input, displacement]).unwrap();
                    let native = fd.new_unique_out(width, inner).unwrap();
                    fd.op_insert_end(inner, block);
                    let outer = fd.new_op(2, point);
                    fd.op_set_opcode_code(outer, OpCode::CPUI_INT_ADD);
                    let adjustment = fd.new_constant(width, 8);
                    fd.op_set_all_input(outer, &[native, adjustment]).unwrap();
                    let result = fd.new_unique_out(width, outer).unwrap();
                    fd.op_insert_end(outer, block);
                    if preserve { fd.kuna_stack_scope_holds.push(native); }
                    crate::ruleaction_1::RuleCollectTerms::new("analysis").apply_op(outer, &mut fd);
                    assert_eq!(native_stack_delta(&fd, result, input), Some(8 - frame as i128));
                    if preserve {
                        assert_eq!(native_stack_delta(&fd, native, input), Some(-(frame as i128)));
                    }
                }
            }
        }
    }

    #[test]
    fn ppc_linkage_recovery_requires_abi_identity_and_no_source_observation() {
        for width in [4, 8] {
            for mode in ["native", "entry_copy", "before", "parent", "dead_read", "body_observer", "adjacent_write", "other_isa", "split", "partial", "displaced", "free", "scalar", "live_read", "address_escape", "clobber", "merged_body", "merged_header", "merged_shifted_header", "merged_cycle", "merged_scalar_cycle", "merged_reconverging", "global_address_escape", "global_body_escape", "merged_dead_read_bad_arm", "merged_dead_read_cycle", "global_address_gap"] {
                let (mut fd, allocation, end, _) = scoped_lifetime_fixture_abi(width, false, if mode == "other_isa" { 2 } else { 1 });
                let stack = fd.get_arch().manage().get_stack_space().unwrap();
                let entry = fd.find_spacebase_input(stack).unwrap();
                let memory = fd.new_constant(4, stack.get_contain().unwrap().get_index() as u64);
                let point = fd.obank().get(allocation.op).unwrap().get_addr().clone();
                let prologue = fd.new_op(3, point.clone());
                fd.op_set_opcode_code(prologue, OpCode::CPUI_STORE);
                fd.op_set_all_input(prologue, &[memory, allocation.before, entry]).unwrap();
                fd.op_insert_before(prologue, allocation.op);
                let data = match mode {
                    "entry_copy" => {
                        let copy = fd.new_op(1, point.clone());
                        fd.op_set_opcode_code(copy, OpCode::CPUI_COPY);
                        fd.op_set_input(copy, entry, 0).unwrap();
                        let value = fd.new_unique_out(width, copy).unwrap();
                        fd.op_insert_before(copy, allocation.op);
                        value
                    }
                    "before" => allocation.before,
                    "parent" => {
                        let load = fd.new_op(2, point.clone());
                        fd.op_set_opcode_code(load, OpCode::CPUI_LOAD);
                        fd.op_set_all_input(load, &[memory, allocation.before]).unwrap();
                        let value = fd.new_unique_out(width, load).unwrap();
                        fd.op_insert_before(load, allocation.op);
                        value
                    }
                    "free" => {
                        let address = fd.vbank().get(entry).unwrap().get_addr().clone();
                        fd.new_varnode(width, &address, None)
                    }
                    "scalar" => fd.new_constant(width, 7),
                    "partial" => {
                        let cut = fd.new_op(2, point.clone());
                        fd.op_set_opcode_code(cut, OpCode::CPUI_SUBPIECE);
                        let zero = fd.new_constant(4, 0);
                        fd.op_set_all_input(cut, &[entry, zero]).unwrap();
                        let byte = fd.new_unique_out(1, cut).unwrap();
                        fd.op_insert_before(cut, allocation.op);
                        byte
                    }
                    _ => entry,
                };
                let pointer = if mode == "displaced" {
                    let add = fd.new_op(2, point.clone());
                    fd.op_set_opcode_code(add, OpCode::CPUI_INT_ADD);
                    let offset = fd.new_constant(width, 16);
                    fd.op_set_all_input(add, &[allocation.output, offset]).unwrap();
                    let pointer = fd.new_unique_out(width, add).unwrap();
                    fd.op_insert_after(add, allocation.op);
                    pointer
                } else { allocation.output };
                let store = fd.new_op(3, point.clone());
                fd.op_set_opcode_code(store, OpCode::CPUI_STORE);
                fd.op_set_all_input(store, &[memory, pointer, data]).unwrap();
                let previous = fd.vbank().get(pointer).and_then(|value| value.get_def()).unwrap();
                fd.op_insert_after(store, previous);
                let commit_point = if mode == "split" { Address::new(Rc::clone(point.get_space().unwrap()), point.get_offset() + 4) } else { point.clone() };
                let commit = fd.new_op(1, commit_point);
                fd.op_set_opcode_code(commit, OpCode::CPUI_COPY);
                fd.op_set_input(commit, allocation.output, 0).unwrap();
                let address = fd.vbank().get(allocation.output).unwrap().get_addr().clone();
                fd.new_varnode_out(width, &address, commit).unwrap();
                fd.op_insert_after(commit, store);
                let merged = mode.starts_with("merged_");
                let certifies = merged || mode.starts_with("global_") || matches!(mode, "native" | "entry_copy" | "before" | "parent" | "dead_read" | "body_observer" | "adjacent_write" | "live_read" | "address_escape" | "clobber");
                assert_eq!(ppc_backchain_store(&fd, &allocation, store, width), certifies, "{width}: {mode}");
                if merged || mode.starts_with("global_") || matches!(mode, "live_read" | "dead_read" | "body_observer" | "address_escape") {
                    let value = if merged {
                        let mut arms = Vec::new();
                        for offset in [16, 32] {
                            let add = fd.new_op(2, point.clone());
                            fd.op_set_opcode_code(add, OpCode::CPUI_INT_ADD);
                            let offset = fd.new_constant(width, offset);
                            fd.op_set_all_input(add, &[allocation.output, offset]).unwrap();
                            arms.push(fd.new_unique_out(width, add).unwrap());
                            fd.op_insert_before(add, end.restore);
                        }
                        if mode == "merged_header" || mode.starts_with("merged_dead_read_") { arms[0] = allocation.output; }
                        if mode == "merged_scalar_cycle" { arms[0] = allocation.amount; }
                        let phi = fd.new_op(2, point.clone());
                        fd.op_set_opcode_code(phi, OpCode::CPUI_MULTIEQUAL);
                        fd.op_set_all_input(phi, &arms).unwrap();
                        let mut value = fd.new_unique_out(width, phi).unwrap();
                        fd.op_insert_before(phi, end.restore);
                        if matches!(mode, "merged_shifted_header" | "merged_cycle" | "merged_scalar_cycle" | "merged_dead_read_cycle") {
                            let add = fd.new_op(2, point.clone());
                            fd.op_set_opcode_code(add, OpCode::CPUI_INT_ADD);
                            let offset = fd.new_constant(width, if mode == "merged_shifted_header" { (-16_i64) as u64 & calc_mask(width) } else { 16 });
                            fd.op_set_all_input(add, &[value, offset]).unwrap();
                            let shifted = fd.new_unique_out(width, add).unwrap();
                            fd.op_insert_before(add, end.restore);
                            if mode == "merged_shifted_header" { value = shifted; }
                            else { fd.op_set_input(phi, shifted, 1).unwrap(); }
                        }
                        if mode == "merged_reconverging" {
                            let join = fd.new_op(2, point.clone());
                            fd.op_set_opcode_code(join, OpCode::CPUI_MULTIEQUAL);
                            fd.op_set_all_input(join, &[value, value]).unwrap();
                            value = fd.new_unique_out(width, join).unwrap();
                            fd.op_insert_before(join, end.restore);
                        }
                        value
                    } else if matches!(mode, "live_read" | "dead_read") {
                        let load = fd.new_op(2, point.clone());
                        fd.op_set_opcode_code(load, OpCode::CPUI_LOAD);
                        fd.op_set_all_input(load, &[memory, allocation.output]).unwrap();
                        let value = fd.new_unique_out(width, load).unwrap();
                        fd.op_insert_before(load, end.restore);
                        value
                    } else if matches!(mode, "body_observer" | "global_body_escape") {
                        let add = fd.new_op(2, point.clone());
                        fd.op_set_opcode_code(add, OpCode::CPUI_INT_ADD);
                        let offset = fd.new_constant(width, 16);
                        fd.op_set_all_input(add, &[allocation.output, offset]).unwrap();
                        let pointer = fd.new_unique_out(width, add).unwrap();
                        fd.op_insert_before(add, end.restore);
                        pointer
                    } else { allocation.output };
                    if mode.starts_with("global_") {
                        let copy = fd.new_op(1, point.clone());
                        fd.op_set_opcode_code(copy, OpCode::CPUI_COPY);
                        fd.op_set_input(copy, value, 0).unwrap();
                        let memory = Rc::clone(fd.get_arch().manage().get_stack_space().unwrap().get_contain().unwrap());
                        let address = Address::new(memory, 0x9000);
                        let output = fd.new_varnode_out(width, &address, copy).unwrap();
                        fd.vbank_mut().get_mut(output).unwrap().mark_mapped_addr_tied();
                        fd.op_insert_before(copy, if mode == "global_address_gap" { store } else { end.restore });
                    } else if mode.starts_with("merged_dead_read_") {
                        let load = fd.new_op(2, point.clone());
                        fd.op_set_opcode_code(load, OpCode::CPUI_LOAD);
                        fd.op_set_all_input(load, &[memory, value]).unwrap();
                        let loaded = fd.new_unique_out(width, load).unwrap();
                        fd.op_insert_before(load, end.restore);
                        let copy = fd.new_op(1, point.clone());
                        fd.op_set_opcode_code(copy, OpCode::CPUI_COPY);
                        fd.op_set_input(copy, loaded, 0).unwrap();
                        fd.new_varnode_out(width, &address, copy).unwrap();
                        fd.op_insert_before(copy, end.restore);
                    } else if mode == "dead_read" {
                        let copy = fd.new_op(1, point.clone());
                        fd.op_set_opcode_code(copy, OpCode::CPUI_COPY);
                        fd.op_set_input(copy, value, 0).unwrap();
                        fd.new_varnode_out(width, &address, copy).unwrap();
                        fd.op_insert_before(copy, end.restore);
                    } else {
                    let call = fd.new_op(2, point.clone());
                    fd.op_set_opcode_code(call, OpCode::CPUI_CALL);
                    let target = fd.new_constant(width, 0x4000);
                    fd.op_set_all_input(call, &[target, value]).unwrap();
                    fd.op_insert_before(call, end.restore);
                    let mut spec = crate::fspec::FuncCallSpecs::new(call, Address::new_invalid());
                    spec.proto_mut().set_model(Some(Rc::clone(fd.get_func_proto().model())));
                    spec.proto_mut().set_extra_pop(0);
                    fd.push_call_specs(spec);
                    }
                }
                if matches!(mode, "clobber" | "adjacent_write") {
                    let add = fd.new_op(2, point.clone());
                    fd.op_set_opcode_code(add, OpCode::CPUI_INT_ADD);
                    let offset = fd.new_constant(width, if mode == "clobber" { (width - 1) as u64 } else { width as u64 });
                    fd.op_set_all_input(add, &[allocation.output, offset]).unwrap();
                    let pointer = fd.new_unique_out(width, add).unwrap();
                    fd.op_insert_before(add, end.restore);
                    let write = fd.new_op(3, point.clone());
                    fd.op_set_opcode_code(write, OpCode::CPUI_STORE);
                    let byte = fd.new_constant(1, 0);
                    fd.op_set_all_input(write, &[memory, pointer, byte]).unwrap();
                    fd.op_insert_before(write, end.restore);
                }
                if certifies {
                    assert_eq!(native_frame_links(&fd,&FrameFacts::infer(&fd,vec![allocation]).unwrap(),width,&[]).is_err(), matches!(mode, "live_read" | "address_escape" | "clobber" | "merged_header" | "merged_shifted_header" | "merged_cycle" | "global_address_escape" | "merged_dead_read_bad_arm" | "merged_dead_read_cycle" | "global_address_gap"), "{width}: {mode}");
                }
            }
        }
    }

    #[test]
    fn native_backchain_restore_keeps_every_source_observation() {
        for width in [4, 8] {
            for change in ["native", "parent", "copied", "destination_read", "destination_write", "self_link", "entry_self_link", "call_use", "global_use", "destination", "partial", "volatile",
                "missing_restore", "source_word", "header_clobber", "partial_header_clobber", "other_isa"] {
                let (mut fd, root, end, _) = scoped_lifetime_fixture_abi(width, false, if change == "other_isa" { 2 } else { 1 });
                let stack = Rc::clone(fd.get_arch().manage().get_stack_space().unwrap());
                let memory = Rc::clone(stack.get_contain().unwrap());
                let entry = fd.find_spacebase_input(&stack).unwrap();
                let space = fd.new_constant(4, memory.get_index() as u64);
                let point = fd.obank().get(root.op).unwrap().get_addr().clone();
                let header = fd.new_op(3, point.clone());
                fd.op_set_opcode_code(header, OpCode::CPUI_STORE);
                let initial = if change == "parent" {
                    let prologue = fd.new_op(3, point.clone());
                    fd.op_set_opcode_code(prologue, OpCode::CPUI_STORE);
                    fd.op_set_all_input(prologue, &[space, root.before, entry]).unwrap();
                    fd.op_insert_before(prologue, root.op);
                    let load = fd.new_op(2, point.clone());
                    fd.op_set_opcode_code(load, OpCode::CPUI_LOAD);
                    fd.op_set_all_input(load, &[space, root.before]).unwrap();
                    let value = fd.new_unique_out(width, load).unwrap();
                    fd.op_insert_before(load, root.op);
                    value
                } else if change == "self_link" { root.before } else { entry };
                if change == "entry_self_link" {
                    let frame = fd.vbank().get(root.before).unwrap().get_def().unwrap();
                    let zero = fd.new_constant(width, 0);
                    fd.op_set_input(frame, zero, 1).unwrap();
                }
                fd.op_set_all_input(header, &[space, root.output, initial]).unwrap();
                fd.op_insert_after(header, root.op);
                let commit = fd.new_op(1, point.clone());
                fd.op_set_opcode_code(commit, OpCode::CPUI_COPY);
                fd.op_set_input(commit, root.output, 0).unwrap();
                let native_sp = fd.vbank().get(root.output).unwrap().get_addr().clone();
                fd.new_varnode_out(width, &native_sp, commit).unwrap();
                fd.op_insert_after(commit, header);
                let pointer = if change == "source_word" {
                    let add = fd.new_op(2, point.clone());
                    fd.op_set_opcode_code(add, OpCode::CPUI_INT_ADD);
                    let offset = fd.new_constant(width, width as u64);
                    fd.op_set_all_input(add, &[root.output, offset]).unwrap();
                    let pointer = fd.new_unique_out(width, add).unwrap();
                    fd.op_insert_before(add, end.restore);
                    pointer
                } else { root.output };
                let load = fd.new_op(2, point.clone());
                fd.op_set_opcode_code(load, OpCode::CPUI_LOAD);
                fd.op_set_all_input(load, &[space, pointer]).unwrap();
                let payload = fd.new_unique_out(if change == "partial" { width / 2 } else { width }, load).unwrap();
                fd.op_insert_before(load, end.restore);
                let payload=if change=="copied" {
                    let copy=fd.new_op(1,point.clone());
                    fd.op_set_opcode_code(copy,OpCode::CPUI_COPY);
                    fd.op_set_input(copy,payload,0).unwrap();
                    let value=fd.new_unique_out(width,copy).unwrap();
                    fd.op_insert_after(copy,load);
                    value
                } else {payload};
                if change == "volatile" {
                    fd.vbank_mut().get_mut(payload).unwrap().set_flags_pub(crate::varnode::varnode_flags::volatil);
                }
                let pointer = if change == "destination" {
                    let add = fd.new_op(2, point.clone());
                    fd.op_set_opcode_code(add, OpCode::CPUI_INT_ADD);
                    let offset = fd.new_constant(width, width as u64);
                    fd.op_set_all_input(add, &[end.value, offset]).unwrap();
                    let pointer = fd.new_unique_out(width, add).unwrap();
                    fd.op_insert_after(add, end.restore);
                    pointer
                } else { end.value };
                let restore_store = fd.new_op(3, point.clone());
                fd.op_set_opcode_code(restore_store, OpCode::CPUI_STORE);
                fd.op_set_all_input(restore_store, &[space, pointer, payload]).unwrap();
                let previous = fd.vbank().get(pointer).unwrap().get_def().unwrap();
                fd.op_insert_after(restore_store, previous);
                if change=="destination_read" {
                    let read=fd.new_op(2,point.clone());
                    fd.op_set_opcode_code(read,OpCode::CPUI_LOAD);
                    fd.op_set_all_input(read,&[space,root.before]).unwrap();
                    let value=fd.new_unique_out(width,read).unwrap();
                    fd.op_insert_after(read,restore_store);
                    let expose=fd.new_op(1,point.clone());
                    fd.op_set_opcode_code(expose,OpCode::CPUI_COPY);
                    fd.op_set_input(expose,value,0).unwrap();
                    let global=Address::new(Rc::clone(&memory),0x9000);
                    let output=fd.new_varnode_out(width,&global,expose).unwrap();
                    fd.vbank_mut().get_mut(output).unwrap().mark_mapped_addr_tied();
                    fd.op_insert_after(expose,read);
                }
                if change=="destination_write" {
                    let write=fd.new_op(3,point.clone());
                    fd.op_set_opcode_code(write,OpCode::CPUI_STORE);
                    let value=fd.new_constant(width,0);
                    fd.op_set_all_input(write,&[space,root.before,value]).unwrap();
                    fd.op_insert_after(write,restore_store);
                }
                if change == "missing_restore" { fd.op_set_input(end.restore, root.amount, 0).unwrap(); }
                if matches!(change, "call_use" | "global_use") {
                    let user = fd.new_op(if change == "call_use" { 2 } else { 1 }, point.clone());
                    if change == "call_use" {
                        fd.op_set_opcode_code(user, OpCode::CPUI_CALL);
                        let target = fd.new_constant(width, 0x7000);
                        fd.op_set_all_input(user, &[target, payload]).unwrap();
                    } else {
                        fd.op_set_opcode_code(user, OpCode::CPUI_COPY);
                        fd.op_set_input(user, payload, 0).unwrap();
                        let global = Address::new(Rc::clone(&memory), 0x9000);
                        let value = fd.new_varnode_out(width, &global, user).unwrap();
                        fd.vbank_mut().get_mut(value).unwrap().mark_mapped_addr_tied();
                    }
                    fd.op_insert_before(user, end.restore);
                }
                if matches!(change, "header_clobber" | "partial_header_clobber") {
                    let write = fd.new_op(3, point);
                    fd.op_set_opcode_code(write, OpCode::CPUI_STORE);
                    let zero = fd.new_constant(if change == "partial_header_clobber" { 1 } else { width }, 0);
                    fd.op_set_all_input(write, &[space, root.output, zero]).unwrap();
                    fd.op_insert_before(write, load);
                }
                let plan = native_frame_links(&fd,&FrameFacts::infer(&fd,vec![root]).unwrap(),width,&[]);
                if matches!(change, "native" | "parent" | "copied") {
                    assert!(plan.unwrap().iter().any(|&(store, _)| store == restore_store),
                        "only native relocation may be removed, width={width}");
                    let facts=FrameFacts::infer(&fd,vec![root]).unwrap();
                    let whole=AllocationPlan::prepare(&fd,&facts,&[],width,&[],&[],&[]).unwrap();
                    assert!(whole.native_values.contains(&load));
                    assert!(whole.captures.iter().all(|capture|capture.uses.iter().all(|&(op,_)|op!=restore_store)));
                    whole.commit(&mut fd,&[root],&[],width);
                    assert!(fd.obank().get(load).is_none_or(|op|op.is_dead()),"native transport read survived commit");
                    assert!(fd.obank().get(restore_store).is_none_or(|op|op.is_dead()));
                } else if change == "source_word" {
                    assert!(plan.is_err() || !plan.unwrap().iter().any(|&(store, _)| store == restore_store),
                        "a source payload read must not become removable bookkeeping, width={width}");
                } else {
                    assert!(plan.is_err(), "width={width}, change={change}");
                }
            }
        }
    }

    #[test]
    fn ppc_backchain_loop_checks_preceding_iteration_observers() {
        for width in [4, 8] {
            let (mut fd, allocation, _, body) = scoped_lifetime_fixture_abi(width, false, 1);
            let point = fd.obank().get(allocation.op).unwrap().get_addr().clone();
            let stack = fd.get_arch().manage().get_stack_space().unwrap();
            let entry = fd.find_spacebase_input(stack).unwrap();
            let memory = fd.new_constant(4, stack.get_contain().unwrap().get_index() as u64);
            fd.op_uninsert(allocation.op);
            fd.op_insert_end(allocation.op, body);
            let phi = fd.new_op(2, point.clone());
            fd.op_set_opcode_code(phi, OpCode::CPUI_MULTIEQUAL);
            fd.op_set_all_input(phi, &[entry, allocation.output]).unwrap();
            let pointer = fd.new_unique_out(width, phi).unwrap();
            fd.op_insert_before(phi, allocation.op);
            let load = fd.new_op(2, point.clone());
            fd.op_set_opcode_code(load, OpCode::CPUI_LOAD);
            fd.op_set_all_input(load, &[memory, pointer]).unwrap();
            let value = fd.new_unique_out(width, load).unwrap();
            fd.op_insert_before(load, allocation.op);
            let call = fd.new_op(2, point.clone());
            fd.op_set_opcode_code(call, OpCode::CPUI_CALL);
            let target = fd.new_constant(width, 0x4000);
            fd.op_set_all_input(call, &[target, value]).unwrap();
            fd.op_insert_before(call, allocation.op);
            let store = fd.new_op(3, point.clone());
            fd.op_set_opcode_code(store, OpCode::CPUI_STORE);
            fd.op_set_all_input(store, &[memory, allocation.output, entry]).unwrap();
            fd.op_insert_after(store, allocation.op);
            let commit = fd.new_op(1, point);
            fd.op_set_opcode_code(commit, OpCode::CPUI_COPY);
            fd.op_set_input(commit, allocation.output, 0).unwrap();
            let address = fd.vbank().get(allocation.output).unwrap().get_addr().clone();
            fd.new_varnode_out(width, &address, commit).unwrap();
            fd.op_insert_after(commit, store);
            assert!(native_frame_links(&fd,&FrameFacts::infer(&fd,vec![allocation]).unwrap(),width,&[]).is_err(), "{width}: previous iteration's header is observed");
        }
    }

    #[test]
    fn restored_frame_offsets_use_the_actual_read_point() {
        for width in [4, 8] {
            for partial in [false, true] {
                let (mut fd, allocation, end, body, _, saved) = checkpoint_call_fixture(width, false, false);
                let point = fd.get_address().clone();
                let native = fd.vbank().get(end.value).unwrap().get_addr().clone();
                let phi = fd.new_op(2, point.clone());
                fd.op_set_opcode_code(phi, OpCode::CPUI_MULTIEQUAL);
                fd.op_set_input(phi, allocation.before, 0).unwrap();
                let before = fd.new_varnode_out(width, &native, phi).unwrap();
                fd.op_insert_begin(phi, body);
                let save = fd.vbank().get(saved).unwrap().get_def().unwrap();
                fd.op_uninsert(save);
                fd.op_insert_after(save, phi);
                fd.op_set_input(save, before, 0).unwrap();
                fd.op_uninsert(allocation.op);
                fd.op_insert_after(allocation.op, save);
                fd.op_set_input(allocation.op, before, 0).unwrap();
                let incoming = fd.obank().get(end.restore).unwrap().get_in(0).unwrap();
                let read = fd.new_op(1, point.clone());
                fd.op_set_opcode_code(read, OpCode::CPUI_COPY);
                fd.op_set_input(read, incoming, 0).unwrap();
                let value = fd.new_unique_out(width, read).unwrap();
                fd.op_insert_end(read, body);
                fd.op_set_input(phi, value, 1).unwrap();
                fd.op_uninsert(end.restore);
                fd.op_insert_after(end.restore, read);
                fd.op_set_input(end.restore, value, 0).unwrap();
                if partial {
                    let write = fd.new_op(1, point);
                    fd.op_set_opcode_code(write, OpCode::CPUI_COPY);
                    let byte = fd.new_constant(1, 7);
                    fd.op_set_input(write, byte, 0).unwrap();
                    let stack = Rc::clone(fd.get_arch().manage().get_stack_space().unwrap());
                    let address = Address::new(stack, (-31_i64) as u64 & calc_mask(width));
                    fd.new_varnode_out(1, &address, write).unwrap();
                    fd.op_insert_before(write, read);
                }
                let actual = saved_stack_value(&fd, end.value, &mut 65536, &mut SavedFrameQuery::default())
                    .and_then(|saved| saved.value).and_then(|value|native_stack_delta(&fd,value,allocation.before));
                assert_eq!(actual, (!partial).then_some(0), "width={width}, partial={partial}, actual read");
                let offset = frame_offset(&fd, end.value, &mut 65536, &mut SavedFrameQuery::default());
                assert_eq!(offset, (!partial).then_some(-64), "width={width}, partial={partial}, composed frame offset");
            }
        }
    }
    #[test]
    fn native_frame_offsets_do_not_strip_an_invalid_stack_read() {
        for width in [4, 8] {
            let (mut fd, _, end, _, _, _) = checkpoint_call_fixture(width, false, false);
            let write = fd.new_op(1, fd.get_address().clone());
            fd.op_set_opcode_code(write, OpCode::CPUI_COPY);
            let byte = fd.new_constant(1, 7);
            fd.op_set_input(write, byte, 0).unwrap();
            let stack = Rc::clone(fd.get_arch().manage().get_stack_space().unwrap());
            let address = Address::new(stack, (-31_i64) as u64 & calc_mask(width));
            fd.new_varnode_out(1, &address, write).unwrap();
            fd.op_insert_before(write, end.restore);
            assert!(saved_stack_value(&fd, end.value, &mut 65536, &mut SavedFrameQuery::default()).is_none());
            assert_eq!(frame_offset(&fd, end.value, &mut 65536, &mut SavedFrameQuery::default()), None,
                "width={width}: an earlier memory version is not the restored register's value");
            let add = fd.new_op(2, fd.get_address().clone());
            fd.op_set_opcode_code(add, OpCode::CPUI_INT_ADD);
            let eight = fd.new_constant(width, 8);
            fd.op_set_all_input(add, &[end.value, eight]).unwrap();
            let derived = fd.new_unique_out(width, add).unwrap();
            fd.op_insert_after(add, end.restore);
            assert_eq!(frame_offset(&fd, derived, &mut 65536, &mut SavedFrameQuery::default()), None,
                "width={width}: affine arithmetic cannot validate a failed read");
        }
    }
    #[test]
    fn call_returned_store_addresses_keep_the_stored_frame_origin() {
        for width in [4, 8] {
            for code in [OpCode::CPUI_CALL, OpCode::CPUI_CALLIND, OpCode::CPUI_CALLOTHER] {
                let (mut fd, root, _, body, index, _) = checkpoint_call_fixture(width, false, false);
                let point = fd.get_address().clone();
                let stack = Rc::clone(fd.get_arch().manage().get_stack_space().unwrap());
                let memory = Rc::clone(stack.get_contain().unwrap());
                let entry = fd.find_spacebase_input(&stack).unwrap();
                let address = fd.new_op(2, point.clone());
                fd.op_set_opcode_code(address, OpCode::CPUI_INT_ADD);
                let offset = fd.new_constant(width, (-16_i64) as u64 & calc_mask(width));
                fd.op_set_all_input(address, &[entry, offset]).unwrap();
                let pointer = fd.new_unique_out(width, address).unwrap();
                fd.op_insert_before(address, root.op);
                // A callee may obtain this public local's address from a global,
                // not only from its explicit argument list.
                let expose = fd.new_op(1, point.clone());
                fd.op_set_opcode_code(expose, OpCode::CPUI_COPY);
                fd.op_set_input(expose, pointer, 0).unwrap();
                let global = fd.new_varnode_out(width, &Address::new(Rc::clone(&memory), 0x9000), expose).unwrap();
                fd.vbank_mut().get_mut(global).unwrap().mark_mapped_addr_tied();
                fd.op_insert_before(expose, root.op);
                let space = fd.new_constant(4, memory.get_index() as u64);
                let init = fd.new_op(3, point.clone());
                fd.op_set_opcode_code(init, OpCode::CPUI_STORE);
                let zero = fd.new_constant(width, 0);
                fd.op_set_all_input(init, &[space, pointer, zero]).unwrap();
                fd.op_insert_before(init, root.op);
                let call = fd.get_call_specs(index).get_op();
                fd.op_set_opcode_code(call, code);
                let integer = Rc::new(crate::dtype::Datatype::new(width, type_metatype::TYPE_UINT));
                fd.get_call_specs_mut(index).proto_mut().attach_internal_store(integer);
                let returned = fd.new_unique_out(width, call).unwrap();
                let write = fd.new_op(3, point.clone());
                fd.op_set_opcode_code(write, OpCode::CPUI_STORE);
                fd.op_set_all_input(write, &[space, returned, root.before]).unwrap();
                fd.op_insert_end(write, body);
                let load = fd.new_op(2, point);
                fd.op_set_opcode_code(load, OpCode::CPUI_LOAD);
                fd.op_set_all_input(load, &[space, pointer]).unwrap();
                let value = fd.new_unique_out(width, load).unwrap();
                fd.op_insert_end(load, body);
                let mut budget = 65536;
                assert!(frame_dependent(&fd, value, &mut budget, FrameWalk::Values, &mut SavedFrameQuery::default()),
                    "width={width}, code={code:?}: a returned address must not hide the native pointer payload");
                assert!(budget > 0);
            }
        }
    }
    #[test]
    fn spilled_fixed_store_addresses_do_not_taint_disjoint_scalar_words() {
        for width in [4, 8] {
            for raw in [true, false] {
                for aliases in [0, 1, 3] {
            let (mut fd, root, end, body, _, _) = checkpoint_call_fixture(width, false, false);
            let point = fd.get_address().clone();
            let stack = Rc::clone(fd.get_arch().manage().get_stack_space().unwrap());
            let entry = fd.find_spacebase_input(&stack).unwrap();
            let space = fd.new_constant(4, stack.get_contain().unwrap().get_index() as u64);
            let address = |fd: &mut Funcdata, displacement: i64| {
                let op = fd.new_op(2, fd.get_address().clone());
                fd.op_set_opcode_code(op, OpCode::CPUI_INT_ADD);
                let offset = fd.new_constant(width, displacement as u64 & calc_mask(width));
                fd.op_set_all_input(op, &[entry, offset]).unwrap();
                let value = fd.new_unique_out(width, op).unwrap();
                fd.op_insert_before(op, root.op);
                value
            };
            let word = address(&mut fd, -16);
            let saved_pointer = address(&mut fd, -32);
            let init = fd.new_op(3, point.clone());
            fd.op_set_opcode_code(init, OpCode::CPUI_STORE);
            let zero = fd.new_constant(width, 0);
            fd.op_set_all_input(init, &[space, word, zero]).unwrap();
            fd.op_insert_before(init, root.op);
            let load = fd.new_op(if raw { 2 } else { 1 }, point.clone());
            if raw {
                fd.op_set_opcode_code(load, OpCode::CPUI_LOAD);
                fd.op_set_all_input(load, &[space, saved_pointer]).unwrap();
            } else {
                fd.op_set_opcode_code(load, OpCode::CPUI_COPY);
                let source = fd.obank().get(end.restore).unwrap().get_in(0).unwrap();
                fd.op_set_input(load, source, 0).unwrap();
            }
            let pointer = fd.new_unique_out(width, load).unwrap();
            fd.op_insert_end(load, body);
            let mut pointer = pointer;
            for _ in 0..aliases {
                let copy = fd.new_op(1, point.clone());
                fd.op_set_opcode_code(copy, OpCode::CPUI_COPY);
                fd.op_set_input(copy, pointer, 0).unwrap();
                pointer = fd.new_unique_out(width, copy).unwrap();
                fd.op_insert_end(copy, body);
            }
            let write = fd.new_op(3, point.clone());
            fd.op_set_opcode_code(write, OpCode::CPUI_STORE);
            fd.op_set_all_input(write, &[space, pointer, root.before]).unwrap();
            fd.op_insert_end(write, body);
            let read = fd.new_op(2, point);
            fd.op_set_opcode_code(read, OpCode::CPUI_LOAD);
            fd.op_set_all_input(read, &[space, word]).unwrap();
            let value = fd.new_unique_out(width, read).unwrap();
            fd.op_insert_end(read, body);
            let mut budget = 65536;
            assert!(!frame_dependent(&fd, value, &mut budget, FrameWalk::Values, &mut SavedFrameQuery::default()),
                "width={width}, raw={raw}, aliases={aliases}: a store through a proved entry-64 pointer cannot change entry-16");
            assert!(budget > 0);
        }
        }
        }
    }
    #[test]
    fn indirect_read_candidates_keep_effect_and_storage_guards() {
        for width in [4, 8] {
            for mode in ["call", "unknown_effect", "foreign_store", "creation", "volatile"] {
                let (mut fd, root, end, _, _, _) = checkpoint_call_fixture(width, false, false);
                let point = fd.get_address().clone();
                let stack = Rc::clone(fd.get_arch().manage().get_stack_space().unwrap());
                let marker_value = fd.obank().get(end.restore).unwrap().get_in(0).unwrap();
                let marker = fd.vbank().get(marker_value).unwrap().get_def().unwrap();
                if matches!(mode, "unknown_effect" | "foreign_store") {
                    let effect = fd.new_op(if mode == "foreign_store" { 3 } else { 2 }, point.clone());
                    let zero = fd.new_constant(width, 0);
                    if mode == "foreign_store" {
                        fd.op_set_opcode_code(effect, OpCode::CPUI_STORE);
                        let register = fd.get_arch().manage().get_space_by_name("register").unwrap().get_index();
                        let space = fd.new_constant(4, register as u64);
                        fd.op_set_all_input(effect, &[space, root.output, zero]).unwrap();
                    } else {
                        fd.op_set_opcode_code(effect, OpCode::CPUI_INT_ADD);
                        fd.op_set_all_input(effect, &[zero, zero]).unwrap();
                        fd.new_unique_out(width, effect).unwrap();
                    }
                    fd.op_insert_before(effect, marker);
                    let reference = fd.new_varnode_iop(effect);
                    fd.op_set_input(marker, reference, 1).unwrap();
                } else if mode == "creation" {
                    let zero = fd.new_constant(width, 0);
                    fd.op_set_input(marker, zero, 0).unwrap();
                    fd.mark_indirect_creation(marker, false).unwrap();
                } else if mode == "volatile" {
                    fd.vbank_mut().get_mut(marker_value).unwrap().set_flags_pub(crate::varnode::varnode_flags::volatil);
                }
                let entry = fd.find_spacebase_input(&stack).unwrap();
                let address = fd.new_op(2, point.clone());
                fd.op_set_opcode_code(address, OpCode::CPUI_INT_ADD);
                let offset = fd.new_constant(width, (-32_i64) as u64 & calc_mask(width));
                fd.op_set_all_input(address, &[entry, offset]).unwrap();
                let pointer = fd.new_unique_out(width, address).unwrap();
                fd.op_insert_before(address, root.op);
                let load = fd.new_op(2, point);
                fd.op_set_opcode_code(load, OpCode::CPUI_LOAD);
                let space = fd.new_constant(4, stack.get_contain().unwrap().get_index() as u64);
                fd.op_set_all_input(load, &[space, pointer]).unwrap();
                let value = fd.new_unique_out(width, load).unwrap();
                fd.op_insert_before(load, end.restore);
                fd.op_set_input(end.restore, value, 0).unwrap();
                assert_eq!(proved_restore(&fd, &root, end.value).is_some(), mode == "call",
                    "width={width}, mode={mode}: unsupported or overwritten saved words cannot prove a restoration");
            }
        }
    }
    #[test]
    fn initialized_loop_words_propagate_to_suspended_readers_without_waiving_reads() {
        for width in [4, 8] {
            for clobber in [-1, 0, 1, width, width + 1] {
                for after_read in [false, true] {
                    for escaped in [false, true] {
                let (mut fd, root, end, _body, index, saved) = checkpoint_call_fixture(width, false, false);
                let point = fd.get_address().clone();
                let stack = Rc::clone(fd.get_arch().manage().get_stack_space().unwrap());
                let initial = fd.vbank().get(saved).unwrap().get_def().unwrap();
                fd.op_set_input(initial, root.output, 0).unwrap();
                // Keep the initialized pointer assignment after its allocation.
                fd.op_uninsert(initial);
                fd.op_insert_after(initial, root.op);
                let after_call = fd.obank().get(end.restore).unwrap().get_in(0).unwrap();
                let marker = fd.vbank().get(after_call).unwrap().get_def().unwrap();
                let merge = fd.new_op(2, point.clone());
                fd.op_set_opcode_code(merge, OpCode::CPUI_MULTIEQUAL);
                fd.op_set_all_input(merge, &[saved, after_call]).unwrap();
                let slot = fd.vbank().get(saved).unwrap().get_addr().clone();
                let current = fd.new_varnode_out(width, &slot, merge).unwrap();
                let call = fd.get_call_specs(index).get_op();
                fd.op_insert_before(merge, call);
                fd.op_set_input(marker, current, 0).unwrap();
                let read = fd.new_op(1, point.clone());
                fd.op_set_opcode_code(read, OpCode::CPUI_COPY);
                fd.op_set_input(read, current, 0).unwrap();
                let pointer = fd.new_unique_out(width, read).unwrap();
                fd.op_insert_before(read, call);
                let memory = fd.new_constant(4, stack.get_contain().unwrap().get_index() as u64);
                let write = fd.new_op(3, point.clone());
                fd.op_set_opcode_code(write, OpCode::CPUI_STORE);
                let byte = fd.new_constant(1, 7);
                fd.op_set_all_input(write, &[memory, pointer, byte]).unwrap();
                fd.op_insert_before(write, call);
                if clobber != 0 || escaped {
                    let entry = fd.find_spacebase_input(&stack).unwrap();
                    let address = fd.new_op(2, point.clone());
                    fd.op_set_opcode_code(address, OpCode::CPUI_INT_ADD);
                    let displacement = fd.new_constant(width, (-32_i64) as u64 & calc_mask(width));
                    fd.op_set_all_input(address, &[entry, displacement]).unwrap();
                    let address_value = fd.new_unique_out(width, address).unwrap();
                    fd.op_insert_before(address, read);
                    let corrupt = fd.new_op(3, point.clone());
                    fd.op_set_opcode_code(corrupt, OpCode::CPUI_STORE);
                    let changed = if clobber == -1 { root.output }
                        else if clobber == width + 1 { root.before }
                        else { fd.new_constant(clobber.max(1), 0) };
                    fd.op_set_all_input(corrupt, &[memory, address_value, changed]).unwrap();
                    if clobber != 0 {
                        fd.op_insert_before(corrupt, if after_read { call } else { read });
                    } else { fd.op_destroy(corrupt); }
                    if escaped { fd.op_set_input(call, address_value, 1).unwrap(); }
                }
                let mut budget = 65536;
                let result = saved_stack_value(&fd, pointer, &mut budget, &mut SavedFrameQuery::default())
                    .and_then(|saved| saved.value);
                assert_eq!(result, (clobber <= 0 && !escaped).then_some(root.output),
                    "width={width}, clobber={clobber}, after_read={after_read}, escaped={escaped}: a conditional loop value does not waive the actual read");
                assert!(budget > 0);
                    }
                }
            }
        }
    }
    #[test]
    fn finite_frame_equations_distinguish_invariant_and_changing_loop_values() {
        for width in [4, 8] {
            for changing in [false, true] {
                let (mut fd, root, _, body) = scoped_lifetime_fixture(width, false);
                let point = fd.get_address().clone();
                let phi = fd.new_op(2, point.clone());
                fd.op_set_opcode_code(phi, OpCode::CPUI_MULTIEQUAL);
                let current = fd.new_unique_out(width, phi).unwrap();
                let copy = fd.new_op(1, point.clone());
                fd.op_set_opcode_code(copy, OpCode::CPUI_COPY);
                fd.op_set_input(copy, current, 0).unwrap();
                let copied = fd.new_unique_out(width, copy).unwrap();
                let step = fd.new_op(2, point);
                fd.op_set_opcode_code(step, OpCode::CPUI_INT_ADD);
                let one = fd.new_constant(width, 1);
                fd.op_set_all_input(step, &[copied, one]).unwrap();
                let next = fd.new_unique_out(width, step).unwrap();
                fd.op_set_all_input(phi, &[root.before, if changing { next } else { copied }]).unwrap();
                fd.op_insert_end(phi, body);
                fd.op_insert_end(copy, body);
                fd.op_insert_end(step, body);
                let mut equations = FrameEquations::new(&fd);
                equations.pure_aliases(&fd);
                equations.trivial_phis(&fd, |_|true);
                let actual = equations.representative(current);
                let initial = equations.representative(root.before);
                assert_eq!(actual == initial, !changing,
                    "width={width}: only a loop-invariant value may become its entry checkpoint");
                assert_eq!(equations.representative(copied), actual,
                    "a pure SSA copy retains the current iteration's value even for a changing PHI");
            }
        }
    }
    #[test]
    fn frame_word_choices_remain_symbolic_instead_of_becoming_constant_queries() {
        for width in [4, 8] {
            let (mut fd, root, _, body) = scoped_lifetime_fixture(width, false);
            let point = fd.get_address().clone();
            let stack = Rc::clone(fd.get_arch().manage().get_stack_space().unwrap());
            let address = Address::new(stack, (-32_i64) as u64 & calc_mask(width));
            let phi = fd.new_op(2, point.clone());
            fd.op_set_opcode_code(phi, OpCode::CPUI_MULTIEQUAL);
            fd.op_set_all_input(phi, &[root.before, root.output]).unwrap();
            let word = fd.new_varnode_out(width, &address, phi).unwrap();
            fd.op_insert_end(phi, body);
            let copy = fd.new_op(1, point);
            fd.op_set_opcode_code(copy, OpCode::CPUI_COPY);
            fd.op_set_input(copy, word, 0).unwrap();
            let current = fd.new_unique_out(width, copy).unwrap();
            fd.op_insert_end(copy, body);
            let uses = FrameWordGraph::new(&fd);
            let mut equations = FrameEquations::new(&fd);
            equations.pure_aliases(&fd);
            for usage in &uses.uses {
                let Some(result)=usage.result else {continue};
                if usage.alias && !uses.blocked.contains(&result) && !uses.blocked.contains(&usage.source) {
                        equations.equate(&fd, result, usage.source);
                    }
            }
            equations.trivial_phis(&fd, |_|true);
            let value = equations.representative(current);
            assert_eq!(value, word, "the read retains the current PHI's value and predecessor choices");
            assert_ne!(value, equations.representative(root.before));
            assert_ne!(value, equations.representative(root.output));
        }
    }
    #[test]
    fn changing_spilled_scalar_origin_does_not_require_an_invariant_value() {
        for width in [4, 8] {
            for mode in ["scalar", "native", "uninitialized"] {
                let (mut fd, root, _, body) = scoped_lifetime_fixture(width, false);
                let point = fd.get_address().clone();
                let stack = Rc::clone(fd.get_arch().manage().get_stack_space().unwrap());
                let address = Address::new(stack, (-32_i64) as u64 & calc_mask(width));
                let phi = fd.new_op(2, point.clone());
                fd.op_set_opcode_code(phi, OpCode::CPUI_MULTIEQUAL);
                let word = fd.new_varnode_out(width, &address, phi).unwrap();
                let read = fd.new_op(1, point.clone());
                fd.op_set_opcode_code(read, OpCode::CPUI_COPY);
                fd.op_set_input(read, word, 0).unwrap();
                let current = fd.new_unique_out(width, read).unwrap();
                let step = fd.new_op(2, point);
                fd.op_set_opcode_code(step, OpCode::CPUI_INT_ADD);
                let one = fd.new_constant(width, 1);
                fd.op_set_all_input(step, &[current, one]).unwrap();
                let next = fd.new_unique_out(width, step).unwrap();
                let initial = if mode == "scalar" { fd.new_constant(width, 0) }
                    else if mode == "native" { root.before }
                    else {
                        let ram = Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap());
                        fd.new_varnode(width, &Address::new(ram, 0x9800), None)
                    };
                fd.op_set_all_input(phi, &[initial, next]).unwrap();
                fd.op_insert_end(phi, body);
                fd.op_insert_end(read, body);
                fd.op_insert_end(step, body);
                let address = fd.new_op(3, fd.get_address().clone());
                fd.op_set_opcode_code(address, OpCode::CPUI_PTRADD);
                let stride = fd.new_constant(4, 1);
                fd.op_set_all_input(address, &[root.output, current, stride]).unwrap();
                let pointer = fd.new_unique_out(width, address).unwrap();
                fd.op_insert_end(address, body);
                let words = FrameWordGraph::new(&fd);
                let mut equations = FrameEquations::new(&fd);
                equations.pure_aliases(&fd);
                for usage in &words.uses {
                    let Some(result)=usage.result else {continue};
                    if usage.alias && !words.blocked.contains(&result) && !words.blocked.contains(&usage.source) {
                        equations.equate(&fd, result, usage.source);
                    }
                }
                equations.trivial_phis(&fd, |_|true);
                assert_ne!(equations.representative(current), equations.representative(initial));
                let mut origins = FrameOrigins::new(&fd, &equations, &words, &[root], &BTreeSet::new());
                origins.settle();
                origins.seal_unknowns();
                assert_eq!(origins.non_native(&equations, current), mode == "scalar", "width={width}, mode={mode}");
                assert_eq!(origins.non_native(&equations, next), mode == "scalar", "width={width}, mode={mode}");
                assert!(!origins.non_native(&equations, pointer), "a source object still has native address bits");
                assert_eq!(origins.private_address(&equations, pointer), Some(mode != "scalar"),
                    "only a scalar-origin index preserves source-object address ownership");
            }
        }
    }
    #[test]
    fn affine_word_comparison_does_not_bypass_removed_read_equations() {
        for width in [4, 8] {
            let (mut fd, root, end, body, _, _) = checkpoint_call_fixture(width, false, false);
            let carried = fd.obank().get(end.restore).unwrap().get_in(0).unwrap();
            let read = fd.new_op(1, fd.get_address().clone());
            fd.op_set_opcode_code(read, OpCode::CPUI_COPY);
            fd.op_set_input(read, carried, 0).unwrap();
            let value = fd.new_unique_out(width, read).unwrap();
            fd.op_insert_end(read, body);
            let graph = FrameWordGraph::new(&fd);
            for mode in ["none", "complete", "removed_effect"] {
                let mut equations = FrameEquations::new(&fd);
                equations.pure_aliases(&fd);
                for usage in &graph.uses {
                    let Some(result)=usage.result else {continue};
                    if mode == "none" || (mode == "removed_effect" && result == carried)
                        || !usage.alias || graph.blocked.contains(&result) || graph.blocked.contains(&usage.source)
                    { continue; }
                    equations.equate(&fd, result, usage.source);
                }
                equations.trivial_phis(&fd, |_|true);
                assert_eq!(equations.delta(&fd, value, root.before), (mode == "complete").then_some(0),
                    "width={width}, mode={mode}: affine helpers must not re-read an unproved spill");
                assert_eq!(equations.delta(&fd, value, value), Some(0),
                    "an opaque SSA read is still identical to itself, not to its unproved producer");
            }
        }
    }
    #[test]
    fn finite_word_facts_preserve_call_checkpoint_and_reject_real_clobbers() {
        for width in [4,8] {
            for clobber in [0,1,width] {
                let (mut fd,root,end,_,_,_) = checkpoint_call_fixture(width,false,false);
                let carried = fd.obank().get(end.restore).unwrap().get_in(0).unwrap();
                if clobber != 0 {
                    let point=fd.get_address().clone();
                    let stack=Rc::clone(fd.get_arch().manage().get_stack_space().unwrap());
                    let entry=fd.find_spacebase_input(&stack).unwrap();
                    let address=fd.new_op(2,point.clone());
                    fd.op_set_opcode_code(address,OpCode::CPUI_INT_ADD);
                    let offset=fd.new_constant(width,(-32_i64) as u64 & calc_mask(width));
                    fd.op_set_all_input(address,&[entry,offset]).unwrap();
                    let pointer=fd.new_unique_out(width,address).unwrap();
                    fd.op_insert_before(address,end.restore);
                    let write=fd.new_op(3,point);
                    fd.op_set_opcode_code(write,OpCode::CPUI_STORE);
                    let memory=fd.new_constant(4,stack.get_contain().unwrap().get_index() as u64);
                    let zero=fd.new_constant(clobber,0);
                    fd.op_set_all_input(write,&[memory,pointer,zero]).unwrap();
                    fd.op_insert_before(write,end.restore);
                    // The read occurs after this physical write, not merely
                    // after the call's logical INDIRECT checkpoint.
                    let read=fd.new_op(1,fd.get_address().clone());
                    fd.op_set_opcode_code(read,OpCode::CPUI_COPY);
                    fd.op_set_input(read,carried,0).unwrap();
                    let loaded=fd.new_unique_out(width,read).unwrap();
                    fd.op_insert_before(read,end.restore);
                    fd.op_set_input(end.restore,loaded,0).unwrap();
                }
                let roots=[root];
                let facts=FrameFacts::infer(&fd,roots.to_vec()).expect("finite frame facts");
                let restored=fd.obank().get(end.restore).unwrap().get_in(0).unwrap();
                assert_eq!(facts.equations.delta(&fd,restored,root.before),
                    (clobber==0).then_some(0),"width={width}, clobber={clobber}");
            }
        }
    }

    #[test]
    fn finite_word_facts_close_loop_effects_without_waiving_actual_reads() {
        for width in [4, 8] {
            for clobber in [-1, 0, 1, width, width + 1] {
                for after_read in [false, true] {
                    for escaped in [false, true] {
                let (mut fd, root, end, _body, index, saved) = checkpoint_call_fixture(width, false, false);
                let point = fd.get_address().clone();
                let stack = Rc::clone(fd.get_arch().manage().get_stack_space().unwrap());
                let initial = fd.vbank().get(saved).unwrap().get_def().unwrap();
                fd.op_set_input(initial, root.output, 0).unwrap();
                // Keep the initialized pointer assignment after its allocation.
                fd.op_uninsert(initial);
                fd.op_insert_after(initial, root.op);
                let after_call = fd.obank().get(end.restore).unwrap().get_in(0).unwrap();
                let marker = fd.vbank().get(after_call).unwrap().get_def().unwrap();
                let merge = fd.new_op(2, point.clone());
                fd.op_set_opcode_code(merge, OpCode::CPUI_MULTIEQUAL);
                fd.op_set_all_input(merge, &[saved, after_call]).unwrap();
                let slot = fd.vbank().get(saved).unwrap().get_addr().clone();
                let current = fd.new_varnode_out(width, &slot, merge).unwrap();
                let call = fd.get_call_specs(index).get_op();
                fd.op_insert_before(merge, call);
                fd.op_set_input(marker, current, 0).unwrap();
                let read = fd.new_op(1, point.clone());
                fd.op_set_opcode_code(read, OpCode::CPUI_COPY);
                fd.op_set_input(read, current, 0).unwrap();
                let pointer = fd.new_unique_out(width, read).unwrap();
                fd.op_insert_before(read, call);
                let memory = fd.new_constant(4, stack.get_contain().unwrap().get_index() as u64);
                let write = fd.new_op(3, point.clone());
                fd.op_set_opcode_code(write, OpCode::CPUI_STORE);
                let byte = fd.new_constant(1, 7);
                fd.op_set_all_input(write, &[memory, pointer, byte]).unwrap();
                fd.op_insert_before(write, call);
                if clobber != 0 || escaped {
                    let entry = fd.find_spacebase_input(&stack).unwrap();
                    let address = fd.new_op(2, point.clone());
                    fd.op_set_opcode_code(address, OpCode::CPUI_INT_ADD);
                    let displacement = fd.new_constant(width, (-32_i64) as u64 & calc_mask(width));
                    fd.op_set_all_input(address, &[entry, displacement]).unwrap();
                    let address_value = fd.new_unique_out(width, address).unwrap();
                    fd.op_insert_before(address, read);
                    let corrupt = fd.new_op(3, point.clone());
                    fd.op_set_opcode_code(corrupt, OpCode::CPUI_STORE);
                    let changed = if clobber == -1 { root.output }
                        else if clobber == width + 1 { root.before }
                        else { fd.new_constant(clobber.max(1), 0) };
                    fd.op_set_all_input(corrupt, &[memory, address_value, changed]).unwrap();
                    if clobber != 0 {
                        fd.op_insert_before(corrupt, if after_read { call } else { read });
                    } else { fd.op_destroy(corrupt); }
                    if escaped { fd.op_set_input(call, address_value, 1).unwrap(); }
                }
                let roots=[root];
                let facts=FrameFacts::infer(&fd,roots.to_vec()).expect("finite loop facts");
                assert_eq!(facts.equations.delta(&fd,pointer,root.output),
                    (clobber<=0 && !escaped).then_some(0),
                    "width={width}, clobber={clobber}, after_read={after_read}, escaped={escaped}");
                    }
                }
            }
        }
    }
    #[test]
    fn finite_raw_word_reads_bind_to_the_store_target_not_its_payload_slot() {
        for width in [4,8] {
            let (mut fd,root,end,_,_,saved)=checkpoint_call_fixture(width,false,false);
            let point=fd.get_address().clone();
            let stack=Rc::clone(fd.get_arch().manage().get_stack_space().unwrap());
            let entry=fd.find_spacebase_input(&stack).unwrap();
            let address=fd.new_op(2,point.clone());
            fd.op_set_opcode_code(address,OpCode::CPUI_INT_ADD);
            let offset=fd.new_constant(width,(-48_i64) as u64 & calc_mask(width));
            fd.op_set_all_input(address,&[entry,offset]).unwrap();
            let pointer=fd.new_unique_out(width,address).unwrap();
            fd.op_insert_before(address,end.restore);
            let write=fd.new_op(3,point.clone());
            fd.op_set_opcode_code(write,OpCode::CPUI_STORE);
            let memory=fd.new_constant(4,stack.get_contain().unwrap().get_index() as u64);
            fd.op_set_all_input(write,&[memory,pointer,saved]).unwrap();
            fd.op_insert_before(write,end.restore);
            let load=fd.new_op(2,point);
            fd.op_set_opcode_code(load,OpCode::CPUI_LOAD);
            fd.op_set_all_input(load,&[memory,pointer]).unwrap();
            let value=fd.new_unique_out(width,load).unwrap();
            fd.op_insert_before(load,end.restore);
            let roots=[root];
            let facts=FrameFacts::infer(&fd,roots.to_vec()).unwrap();
            assert_eq!(facts.equations.delta(&fd,value,root.before),Some(0),
                "width={width}: memory location is the STORE target, not the stored stack value's location");
        }
    }
    #[test]
    fn finite_source_ownership_does_not_prove_dynamic_source_accesses_disjoint() {
        for width in [4,8] {
            let (mut fd,root,_,body)=scoped_lifetime_fixture(width,false);
            let point=fd.get_address().clone();
            let address=fd.new_op(2,point.clone());
            fd.op_set_opcode_code(address,OpCode::CPUI_INT_ADD);
            let four=fd.new_constant(width,4);
            fd.op_set_all_input(address,&[root.output,four]).unwrap();
            let target=fd.new_unique_out(width,address).unwrap();
            fd.op_insert_end(address,body);
            let indexed=fd.new_op(3,fd.get_address().clone());
            fd.op_set_opcode_code(indexed,OpCode::CPUI_PTRADD);
            let stride=fd.new_constant(4,2);
            fd.op_set_all_input(indexed,&[root.output,root.amount,stride]).unwrap();
            let indexed_pointer=fd.new_unique_out(width,indexed).unwrap();
            fd.op_insert_end(indexed,body);
            let roots=[root];
            let facts=FrameFacts::infer(&fd,roots.to_vec()).unwrap();
            let word=FrameWordUse {origin_only: false,result:Some(target),source:target,first:None,pointer:Some(target),
                size:width,read:None,birth:Some(address),alias:false};
            assert_eq!(facts.store_reaches(&fd,root.output,8,word,None),Some(true));
            assert_eq!(facts.store_reaches(&fd,root.output,1,word,None),Some(false));
            // Two symbolic source objects are not disjoint merely because
            // neither is a fixed compiler-private frame pointer.
            let mut symbolic=word;
            symbolic.pointer=Some(indexed_pointer);
            assert_ne!(facts.store_reaches(&fd,root.output,8,symbolic,None),Some(false));
        }
    }

    #[test]
    fn finite_scalar_word_clobbers_invalidate_identity_without_inventing_frame_bits() {
        for width in [4,8] {
            for native in [false,true] {
                let (mut fd,root,end,_,_,saved)=checkpoint_call_fixture(width,false,false);
                let initial=fd.vbank().get(saved).unwrap().get_def().unwrap();
                let zero=fd.new_constant(width,0);
                fd.op_set_input(initial,zero,0).unwrap();
                let stack=Rc::clone(fd.get_arch().manage().get_stack_space().unwrap());
                let entry=fd.find_spacebase_input(&stack).unwrap();
                let address=fd.new_op(2,fd.get_address().clone());
                fd.op_set_opcode_code(address,OpCode::CPUI_INT_ADD);
                let offset=fd.new_constant(width,(-32_i64) as u64 & calc_mask(width));
                fd.op_set_all_input(address,&[entry,offset]).unwrap();
                let pointer=fd.new_unique_out(width,address).unwrap();
                fd.op_insert_before(address,end.restore);
                let write=fd.new_op(3,fd.get_address().clone());
                fd.op_set_opcode_code(write,OpCode::CPUI_STORE);
                let memory=fd.new_constant(4,stack.get_contain().unwrap().get_index() as u64);
                let replacement=if native {root.before} else {fd.new_constant(width,1)};
                fd.op_set_all_input(write,&[memory,pointer,replacement]).unwrap();
                fd.op_insert_before(write,end.restore);
                let roots=[root];
                let facts=FrameFacts::infer(&fd,roots.to_vec()).unwrap();
                assert_eq!(facts.equations.delta(&fd,end.value,zero),None,
                    "a physical write invalidates the old exact scalar value");
                assert_eq!(facts.origins.non_native(&facts.equations,end.value),!native,
                    "width={width}: a known scalar clobber remains scalar; an actual native pointer does not");
            }
        }
    }

    #[test]
    fn finite_partial_pointer_clobbers_lose_source_ownership_but_scalar_clobbers_do_not() {
        for width in [4,8] {
            for source in [false,true] {
            for bytes in [1,width] {
                let (mut fd,root,end,_,_,saved)=checkpoint_call_fixture(width,false,false);
                let initial=fd.vbank().get(saved).unwrap().get_def().unwrap();
                let zero=fd.new_constant(width,0);
                let original=if source {root.output} else {zero};
                fd.op_set_input(initial,original,0).unwrap();
                fd.op_uninsert(initial);
                fd.op_insert_after(initial,root.op);
                let stack=Rc::clone(fd.get_arch().manage().get_stack_space().unwrap());
                let entry=fd.find_spacebase_input(&stack).unwrap();
                let address=fd.new_op(2,fd.get_address().clone());
                fd.op_set_opcode_code(address,OpCode::CPUI_INT_ADD);
                let offset=fd.new_constant(width,(-32_i64) as u64 & calc_mask(width));
                fd.op_set_all_input(address,&[entry,offset]).unwrap();
                let pointer=fd.new_unique_out(width,address).unwrap();
                fd.op_insert_before(address,end.restore);
                let write=fd.new_op(3,fd.get_address().clone());
                fd.op_set_opcode_code(write,OpCode::CPUI_STORE);
                let memory=fd.new_constant(4,stack.get_contain().unwrap().get_index() as u64);
                let replacement=fd.new_constant(bytes,1);
                fd.op_set_all_input(write,&[memory,pointer,replacement]).unwrap();
                fd.op_insert_before(write,end.restore);
                let roots=[root];
                let facts=FrameFacts::infer(&fd,roots.to_vec()).unwrap();
                assert_eq!(facts.equations.delta(&fd,end.value,original),None,
                    "a physical write invalidates the old exact scalar value");
                assert_eq!(facts.origins.non_native(&facts.equations,end.value),!source);
                assert_eq!(facts.origins.private_address(&facts.equations,end.value),Some(source && bytes<width),
                    "width={width}, bytes={bytes}, source={source}: a partial pointer rewrite can redirect into a private frame word");
            }
            }
        }
    }
    #[test]
    fn finite_call_return_addresses_do_not_become_scalar_escape_exemptions() {
        for width in [4,8] {
            for kind in ["scalar","known_scalar_return","return","copy_return"] {
                let (mut fd,root,end,_,index,saved)=checkpoint_call_fixture(width,false,false);
                let initial=fd.vbank().get(saved).unwrap().get_def().unwrap();
                let mut argument=root.amount;
                if kind!="scalar" {
                    let provider=fd.new_op(1,fd.get_address().clone());
                    fd.op_set_opcode_code(provider,OpCode::CPUI_CALL);
                    let target=fd.new_constant(width,0x7000);
                    fd.op_set_input(provider,target,0).unwrap();
                    argument=fd.new_unique_out(width,provider).unwrap();
                    fd.op_insert_before(provider,initial);
                    if kind=="known_scalar_return" {
                        let mut spec=crate::fspec::FuncCallSpecs::new(provider,Address::new_invalid());
                        spec.proto_mut().attach_internal_store(Rc::new(crate::dtype::Datatype::new(width,type_metatype::TYPE_UINT)));
                        spec.proto_mut().set_output_lock(true);
                        fd.push_call_specs(spec);
                    }
                    if kind=="copy_return" {
                        for _ in 0..3 {
                            let copy=fd.new_op(1,fd.get_address().clone());
                            fd.op_set_opcode_code(copy,OpCode::CPUI_COPY);
                            fd.op_set_input(copy,argument,0).unwrap();
                            argument=fd.new_unique_out(width,copy).unwrap();
                            fd.op_insert_before(copy,initial);
                        }
                    }
                }
                let call=fd.get_call_specs(index).get_op();
                fd.op_set_input(call,argument,1).unwrap();
                let roots=[root];
                let facts=FrameFacts::infer(&fd,roots.to_vec()).unwrap();
                assert_eq!(facts.equations.delta(&fd,end.value,root.before),
                    matches!(kind,"scalar"|"known_scalar_return").then_some(0),"width={width}, kind={kind}: an opaque address return is not a definite scalar");
            }
        }
    }

    #[test]
    fn finite_declared_foreign_bases_do_not_alias_private_stack_words() {
        use kuna_base::space::{SpacebaseSpace,VarnodeStorage};
        for width in [4,8] {
            for escaped in [false,true] {
                let mut manager=build_manager();
                manager.set_default_data_space(4).unwrap();
                let ram=Rc::clone(manager.get_space_by_name("ram").unwrap());
                let register=Rc::new(AddrSpace::new(spacetype::IPTR_PROCESSOR,"register",false,
                    width as u32,1,5,addrspace_flags::hasphysical,0,0));
                manager.insert_space(Rc::clone(&register)).unwrap();
                let stack=Rc::new(SpacebaseSpace::new("stack",6,width as u32,&ram,1,true,false));
                manager.add_spacebase_pointer(&stack,&VarnodeStorage {space:Some(Rc::clone(&register)),
                    offset:0,size:width as u32},width,true).unwrap();
                manager.insert_space(Rc::clone(&stack)).unwrap();
                let globals=Rc::new(SpacebaseSpace::new("globals",7,width as u32,&ram,1,true,false));
                manager.add_spacebase_pointer(&globals,&VarnodeStorage {space:Some(Rc::clone(&register)),
                    offset:4*width as u64,size:width as u32},width,true).unwrap();
                manager.insert_space(globals).unwrap();
                let arch=Rc::new(ArchContext::new(manager));
                let point=Address::new(Rc::clone(&ram),0x1000);
                let mut fd=Funcdata::new("foreign_base","foreign_base",arch,point.clone(),0x10000000,0x40).unwrap();
                let graph=fd.bblocks_root_pub();
                let body=fd.bblocks_mut().new_block_basic(graph);
                let sp_addr=Address::new(Rc::clone(&register),0);
                let input=fd.new_varnode(width,&sp_addr,None);
                let entry=fd.set_input_varnode(input).unwrap();
                fd.vbank_mut().get_mut(entry).unwrap().set_spacebase();
                let input=fd.new_varnode(width,&Address::new(register,4*width as u64),None);
                let global=fd.set_input_varnode(input).unwrap();
                fd.vbank_mut().get_mut(global).unwrap().set_spacebase();
                let save=fd.new_op(1,point.clone());
                fd.op_set_opcode_code(save,OpCode::CPUI_COPY);
                fd.op_set_input(save,entry,0).unwrap();
                let saved=fd.new_varnode_out(width,&Address::new(stack,(-16_i64) as u64 & calc_mask(width)),save).unwrap();
                fd.op_insert_end(save,body);
                let add=fd.new_op(2,point.clone());
                fd.op_set_opcode_code(add,OpCode::CPUI_INT_ADD);
                let offset=fd.new_constant(width,4);
                fd.op_set_all_input(add,&[global,offset]).unwrap();
                let destination=fd.new_unique_out(width,add).unwrap();
                fd.op_insert_end(add,body);
                let payload=if escaped {
                    let address=fd.new_op(2,point.clone());
                    fd.op_set_opcode_code(address,OpCode::CPUI_INT_ADD);
                    let offset=fd.new_constant(width,(-16_i64) as u64 & calc_mask(width));
                    fd.op_set_all_input(address,&[entry,offset]).unwrap();
                    let value=fd.new_unique_out(width,address).unwrap();
                    fd.op_insert_end(address,body);
                    value
                } else {fd.new_constant(width,0)};
                let write=fd.new_op(3,point.clone());
                fd.op_set_opcode_code(write,OpCode::CPUI_STORE);
                let memory=fd.new_constant(4,ram.get_index() as u64);
                fd.op_set_all_input(write,&[memory,destination,payload]).unwrap();
                fd.op_insert_end(write,body);
                let restore=fd.new_op(1,point);
                fd.op_set_opcode_code(restore,OpCode::CPUI_COPY);
                fd.op_set_input(restore,saved,0).unwrap();
                let value=fd.new_varnode_out(width,&sp_addr,restore).unwrap();
                fd.op_insert_end(restore,body);
                let facts=FrameFacts::infer(&fd,Vec::new()).unwrap();
                assert_eq!(facts.equations.delta(&fd,value,entry),(!escaped).then_some(0),
                    "width={width}, escaped={escaped}: a global destination is disjoint, but leaking the private word's address is not");
            }
        }
    }

    #[test]
    fn finite_recomputed_reservation_addresses_keep_source_object_ownership() {
        for width in [4,8] {
            for corrupt in [false,true] {
                let (mut fd,root,end,_,_,_)=checkpoint_call_fixture(width,false,false);
                let point=fd.get_address().clone();
                let stack=Rc::clone(fd.get_arch().manage().get_stack_space().unwrap());
                let recompute=fd.new_op(2,point.clone());
                fd.op_set_opcode_code(recompute,OpCode::CPUI_INT_SUB);
                fd.op_set_all_input(recompute,&[root.base,root.amount]).unwrap();
                let alternate=fd.new_unique_out(width,recompute).unwrap();
                fd.op_insert_before(recompute,end.restore);
                let address=fd.new_op(2,point.clone());
                fd.op_set_opcode_code(address,OpCode::CPUI_INT_ADD);
                let base=if corrupt {fd.find_spacebase_input(&stack).unwrap()} else {alternate};
                let offset=if corrupt {fd.new_constant(width,(-32_i64) as u64 & calc_mask(width))} else {
                    let register=Rc::clone(stack.get_spacebase(0).unwrap().space.as_ref().unwrap());
                    let index=fd.new_varnode(width,&Address::new(register,3*width as u64),None);
                    fd.set_input_varnode(index).unwrap()
                };
                fd.op_set_all_input(address,&[base,offset]).unwrap();
                let pointer=fd.new_unique_out(width,address).unwrap();
                fd.op_insert_before(address,end.restore);
                let store=fd.new_op(3,point);
                fd.op_set_opcode_code(store,OpCode::CPUI_STORE);
                let memory=fd.new_constant(4,stack.get_contain().unwrap().get_index() as u64);
                let byte=fd.new_constant(1,7);
                fd.op_set_all_input(store,&[memory,pointer,byte]).unwrap();
                fd.op_insert_before(store,end.restore);
                let roots=[root];
                let facts=FrameFacts::infer(&fd,roots.to_vec()).unwrap();
                assert_eq!(facts.origins.private_address(&facts.equations,pointer),Some(corrupt),
                    "width={width}, corrupt={corrupt}: algebraic address identity does not require an SSA ancestry through the SP commit");
                assert_eq!(facts.equations.delta(&fd,end.value,root.before),(!corrupt).then_some(0));
            }
        }
    }

    #[test]
    fn finite_direct_stack_operands_validate_the_arithmetic_read_point() {
        for width in [4,8] {
            for (code,amount) in [(OpCode::CPUI_COPY,0_i128),(OpCode::CPUI_INT_ADD,0),
                (OpCode::CPUI_INT_ADD,4),(OpCode::CPUI_INT_SUB,4)] {
                for clobber in [false,true] {
                    let (mut fd,root,end,_,_,_)=checkpoint_call_fixture(width,false,false);
                    let carried=fd.obank().get(end.restore).unwrap().get_in(0).unwrap();
                    if code!=OpCode::CPUI_COPY {
                        fd.op_set_opcode_code(end.restore,code);
                        let offset=fd.new_constant(width,amount as u64);
                        fd.op_set_all_input(end.restore,&[carried,offset]).unwrap();
                    }
                    if clobber {
                        let stack=Rc::clone(fd.get_arch().manage().get_stack_space().unwrap());
                        let entry=fd.find_spacebase_input(&stack).unwrap();
                        let add=fd.new_op(2,fd.get_address().clone());
                        fd.op_set_opcode_code(add,OpCode::CPUI_INT_ADD);
                        let offset=fd.new_constant(width,(-32_i64) as u64 & calc_mask(width));
                        fd.op_set_all_input(add,&[entry,offset]).unwrap();
                        let pointer=fd.new_unique_out(width,add).unwrap();
                        fd.op_insert_before(add,end.restore);
                        let write=fd.new_op(3,fd.get_address().clone());
                        fd.op_set_opcode_code(write,OpCode::CPUI_STORE);
                        let memory=fd.new_constant(4,stack.get_contain().unwrap().get_index() as u64);
                        let zero=fd.new_constant(width,0);
                        fd.op_set_all_input(write,&[memory,pointer,zero]).unwrap();
                        fd.op_insert_before(write,end.restore);
                    }
                    let facts=FrameFacts::infer(&fd,vec![root]).unwrap();
                    assert_eq!(facts.equations.delta(&fd,end.value,root.before),(!clobber).then_some(if code==OpCode::CPUI_INT_SUB {-amount} else {amount}),
                        "width={width}, code={code:?}, amount={amount}, clobber={clobber}: ALU operands read memory at the consumer, not at the prior CALL marker");
                }
            }
        }
    }

    #[test]
    fn finite_read_sites_keep_earlier_gpr_snapshots_after_later_clobbers() {
        for width in [4,8] {
            let (mut fd,root,end,_,_,_)=checkpoint_call_fixture(width,false,false);
            let carried=fd.obank().get(end.restore).unwrap().get_in(0).unwrap();
            let copy=fd.new_op(1,fd.get_address().clone());
            fd.op_set_opcode_code(copy,OpCode::CPUI_COPY);
            fd.op_set_input(copy,carried,0).unwrap();
            let snapshot=fd.new_unique_out(width,copy).unwrap();
            fd.op_insert_before(copy,end.restore);
            let stack=Rc::clone(fd.get_arch().manage().get_stack_space().unwrap());
            let entry=fd.find_spacebase_input(&stack).unwrap();
            let address=fd.new_op(2,fd.get_address().clone());
            fd.op_set_opcode_code(address,OpCode::CPUI_INT_ADD);
            let offset=fd.new_constant(width,(-32_i64) as u64 & calc_mask(width));
            fd.op_set_all_input(address,&[entry,offset]).unwrap();
            let pointer=fd.new_unique_out(width,address).unwrap();
            fd.op_insert_before(address,end.restore);
            let write=fd.new_op(3,fd.get_address().clone());
            fd.op_set_opcode_code(write,OpCode::CPUI_STORE);
            let memory=fd.new_constant(4,stack.get_contain().unwrap().get_index() as u64);
            let zero=fd.new_constant(width,0);
            fd.op_set_all_input(write,&[memory,pointer,zero]).unwrap();
            fd.op_insert_before(write,end.restore);
            let late=fd.new_op(2,fd.get_address().clone());
            fd.op_set_opcode_code(late,OpCode::CPUI_INT_ADD);
            fd.op_set_all_input(late,&[carried,zero]).unwrap();
            let late_value=fd.new_unique_out(width,late).unwrap();
            fd.op_insert_before(late,end.restore);
            fd.op_set_opcode_code(end.restore,OpCode::CPUI_INT_ADD);
            fd.op_set_all_input(end.restore,&[snapshot,zero]).unwrap();
            let facts=FrameFacts::infer(&fd,vec![root]).unwrap();
            assert_eq!(facts.equations.delta(&fd,snapshot,root.before),Some(0));
            assert_eq!(facts.equations.delta(&fd,end.value,root.before),Some(0));
            assert_eq!(facts.equations.delta(&fd,late_value,root.before),None);
        }
    }
    #[test]
    fn finite_direct_index_reads_preserve_scalar_but_not_pointer_origins() {
        for width in [4,8] {
            for kind in ["scalar","source","private"] {
                let (mut fd,root,end,_,_,saved)=checkpoint_call_fixture(width,false,false);
                let initial=fd.vbank().get(saved).unwrap().get_def().unwrap();
                let zero=fd.new_constant(width,0);
                fd.op_set_input(initial,zero,0).unwrap();
                let carried=fd.obank().get(end.restore).unwrap().get_in(0).unwrap();
                let stack=Rc::clone(fd.get_arch().manage().get_stack_space().unwrap());
                let entry=fd.find_spacebase_input(&stack).unwrap();
                let address=fd.new_op(2,fd.get_address().clone());
                fd.op_set_opcode_code(address,OpCode::CPUI_INT_ADD);
                let offset=fd.new_constant(width,(-32_i64) as u64 & calc_mask(width));
                fd.op_set_all_input(address,&[entry,offset]).unwrap();
                let pointer=fd.new_unique_out(width,address).unwrap();
                fd.op_insert_before(address,end.restore);
                let write=fd.new_op(3,fd.get_address().clone());
                fd.op_set_opcode_code(write,OpCode::CPUI_STORE);
                let memory=fd.new_constant(4,stack.get_contain().unwrap().get_index() as u64);
                let data=match kind {"source"=>root.output,"private"=>root.before,_=>fd.new_constant(width,1)};
                fd.op_set_all_input(write,&[memory,pointer,data]).unwrap();
                fd.op_insert_before(write,end.restore);
                let indexed=fd.new_op(3,fd.get_address().clone());
                fd.op_set_opcode_code(indexed,OpCode::CPUI_PTRADD);
                let stride=fd.new_constant(4,1);
                fd.op_set_all_input(indexed,&[root.output,carried,stride]).unwrap();
                let value=fd.new_unique_out(width,indexed).unwrap();
                fd.op_insert_before(indexed,end.restore);
                let facts=FrameFacts::infer(&fd,vec![root]).unwrap();
                assert_eq!(facts.origins.non_native_input(&fd,&facts.equations,indexed,1),kind=="scalar");
                assert_eq!(facts.origins.private_address(&facts.equations,value),Some(kind!="scalar"),
                    "width={width}, kind={kind}: a pointer-valued overwrite cannot retain scalar-index ownership");
                assert_eq!(facts.origins.private_address(&facts.equations,root.output),Some(false));
            }
        }
    }
    #[test]
    fn finite_store_addresses_use_their_post_clobber_read_origin() {
        for width in [4,8] {
            let (mut fd,root,end,_,_,saved)=checkpoint_call_fixture(width,false,false);
            let initial=fd.vbank().get(saved).unwrap().get_def().unwrap();
            fd.op_set_input(initial,root.output,0).unwrap();
            fd.op_uninsert(initial);fd.op_insert_after(initial,root.op);
            let carried=fd.obank().get(end.restore).unwrap().get_in(0).unwrap();
            let stack=Rc::clone(fd.get_arch().manage().get_stack_space().unwrap());
            let entry=fd.find_spacebase_input(&stack).unwrap();
            let mut pointers=Vec::new();
            for first in [-32_i64,-48] {
                let address=fd.new_op(2,fd.get_address().clone());
                fd.op_set_opcode_code(address,OpCode::CPUI_INT_ADD);
                let offset=fd.new_constant(width,first as u64 & calc_mask(width));
                fd.op_set_all_input(address,&[entry,offset]).unwrap();
                pointers.push(fd.new_unique_out(width,address).unwrap());
                fd.op_insert_before(address,end.restore);
            }
            let memory=fd.new_constant(4,stack.get_contain().unwrap().get_index() as u64);
            for (pointer,data) in [(pointers[1],root.before),(pointers[0],pointers[1])] {
                let write=fd.new_op(3,fd.get_address().clone());
                fd.op_set_opcode_code(write,OpCode::CPUI_STORE);
                fd.op_set_all_input(write,&[memory,pointer,data]).unwrap();
                fd.op_insert_before(write,end.restore);
            }
            let write=fd.new_op(3,fd.get_address().clone());
            fd.op_set_opcode_code(write,OpCode::CPUI_STORE);
            let one=fd.new_constant(width,1);
            fd.op_set_all_input(write,&[memory,carried,one]).unwrap();
            fd.op_insert_before(write,end.restore);
            let load=fd.new_op(2,fd.get_address().clone());
            fd.op_set_opcode_code(load,OpCode::CPUI_LOAD);
            fd.op_set_all_input(load,&[memory,pointers[1]]).unwrap();
            let value=fd.new_unique_out(width,load).unwrap();
            fd.op_insert_before(load,end.restore);
            let facts=FrameFacts::infer(&fd,vec![root]).unwrap();
            assert_eq!(facts.equations.delta(&fd,value,root.before),None,
                "a stale source-array pointer must not hide a later write to a private saved word");
            assert_eq!(facts.origins.private_input(&fd,&facts.equations,write,1),Some(true));
        }
    }
    #[test]
    fn finite_call_argument_reads_precede_the_calls_memory_effect() {
        for width in [4,8] {
            let (mut fd,root,end,body,index,saved)=checkpoint_call_fixture(width,false,false);
            fd.bblocks_mut().remove_edge(body,body);
            let initial=fd.vbank().get(saved).unwrap().get_def().unwrap();
            let stack=Rc::clone(fd.get_arch().manage().get_stack_space().unwrap());
            let entry=fd.find_spacebase_input(&stack).unwrap();
            let address=fd.new_op(2,fd.get_address().clone());
            fd.op_set_opcode_code(address,OpCode::CPUI_INT_ADD);
            let offset=fd.new_constant(width,(-32_i64) as u64 & calc_mask(width));
            fd.op_set_all_input(address,&[entry,offset]).unwrap();
            let pointer=fd.new_unique_out(width,address).unwrap();
            fd.op_insert_before(address,initial);
            fd.op_set_input(initial,pointer,0).unwrap();
            let call=fd.get_call_specs(index).get_op();
            fd.op_set_input(call,saved,1).unwrap();
            let after=fd.obank().get(end.restore).unwrap().get_in(0).unwrap();
            let facts=FrameFacts::infer(&fd,vec![root]).unwrap();
            assert!(facts.input_valid(call,1),"argument is read before the callee can overwrite its slot");
            assert_eq!(facts.equations.delta(&fd,after,pointer),None,"the post-call version is not the pre-call argument snapshot");
        }
    }

    #[test]
    fn finite_store_data_reads_do_not_reuse_a_clobbered_birth_value() {
        for width in [4,8] {
            let (mut fd,root,end,_,_,saved)=checkpoint_call_fixture(width,false,false);
            let initial=fd.vbank().get(saved).unwrap().get_def().unwrap();
            let zero=fd.new_constant(width,0);
            fd.op_set_input(initial,zero,0).unwrap();
            let carried=fd.obank().get(end.restore).unwrap().get_in(0).unwrap();
            let stack=Rc::clone(fd.get_arch().manage().get_stack_space().unwrap());
            let entry=fd.find_spacebase_input(&stack).unwrap();
            let mut pointers=Vec::new();
            for first in [-32_i64,-48] {
                let address=fd.new_op(2,fd.get_address().clone());
                fd.op_set_opcode_code(address,OpCode::CPUI_INT_ADD);
                let offset=fd.new_constant(width,first as u64 & calc_mask(width));
                fd.op_set_all_input(address,&[entry,offset]).unwrap();
                pointers.push(fd.new_unique_out(width,address).unwrap());
                fd.op_insert_before(address,end.restore);
            }
            let memory=fd.new_constant(4,stack.get_contain().unwrap().get_index() as u64);
            let one=fd.new_constant(width,1);
            let clobber=fd.new_op(3,fd.get_address().clone());
            fd.op_set_opcode_code(clobber,OpCode::CPUI_STORE);
            fd.op_set_all_input(clobber,&[memory,pointers[0],one]).unwrap();
            fd.op_insert_before(clobber,end.restore);
            let store=fd.new_op(3,fd.get_address().clone());
            fd.op_set_opcode_code(store,OpCode::CPUI_STORE);
            fd.op_set_all_input(store,&[memory,pointers[1],carried]).unwrap();
            fd.op_insert_before(store,end.restore);
            let load=fd.new_op(2,fd.get_address().clone());
            fd.op_set_opcode_code(load,OpCode::CPUI_LOAD);
            fd.op_set_all_input(load,&[memory,pointers[1]]).unwrap();
            let value=fd.new_unique_out(width,load).unwrap();
            fd.op_insert_before(load,end.restore);
            let facts=FrameFacts::infer(&fd,vec![root]).unwrap();
            assert_eq!(facts.equations.delta(&fd,value,zero),None,
                "the value copied after an overwrite is not the earlier scalar zero");
            assert!(facts.origins.non_native(&facts.equations,value),"both possible scalar writers remain non-native");
        }
    }

    #[test]
    fn finite_bad_call_arguments_do_not_invalidate_independent_scalar_returns() {
        for width in [4,8] {
            let (mut fd,root,end,body,index,saved)=checkpoint_call_fixture(width,false,false);
            fd.bblocks_mut().remove_edge(body,body);
            let stack=Rc::clone(fd.get_arch().manage().get_stack_space().unwrap());
            let entry=fd.find_spacebase_input(&stack).unwrap();
            let call=fd.get_call_specs(index).get_op();
            let address=fd.new_op(2,fd.get_address().clone());
            fd.op_set_opcode_code(address,OpCode::CPUI_INT_ADD);
            let offset=fd.new_constant(width,(-32_i64) as u64 & calc_mask(width));
            fd.op_set_all_input(address,&[entry,offset]).unwrap();
            let pointer=fd.new_unique_out(width,address).unwrap();
            fd.op_insert_before(address,call);
            let write=fd.new_op(3,fd.get_address().clone());
            fd.op_set_opcode_code(write,OpCode::CPUI_STORE);
            let memory=fd.new_constant(4,stack.get_contain().unwrap().get_index() as u64);
            let zero=fd.new_constant(width,0);
            fd.op_set_all_input(write,&[memory,pointer,zero]).unwrap();
            fd.op_insert_before(write,call);
            fd.op_set_input(call,saved,1).unwrap();
            let returned=fd.new_unique_out(width,call).unwrap();
            fd.get_call_specs_mut(index).proto_mut().attach_internal_store(Rc::new(crate::dtype::Datatype::new(width,type_metatype::TYPE_UINT)));
            fd.get_call_specs_mut(index).proto_mut().set_output_lock(true);
            let sum=fd.new_op(2,fd.get_address().clone());
            fd.op_set_opcode_code(sum,OpCode::CPUI_INT_ADD);
            fd.op_set_all_input(sum,&[root.output,returned]).unwrap();
            let indexed=fd.new_unique_out(width,sum).unwrap();
            fd.op_insert_before(sum,end.restore);
            let difference=fd.new_op(2,fd.get_address().clone());
            fd.op_set_opcode_code(difference,OpCode::CPUI_INT_SUB);
            fd.op_set_all_input(difference,&[indexed,returned]).unwrap();
            let recovered=fd.new_unique_out(width,difference).unwrap();
            fd.op_insert_before(difference,end.restore);
            let facts=FrameFacts::infer(&fd,vec![root]).unwrap();
            assert!(!facts.input_valid(call,1));
            assert!(!facts.equations.value(&fd,returned).1,"argument clobber does not invalidate a call's independent return value");
            assert_eq!(facts.equations.delta(&fd,recovered,root.output),Some(0));
        }
    }

    #[test]
    fn finite_source_stores_may_alias_mixed_frame_pointer_phis() {
        for width in [4,8] {
            for mixed in [false,true] {
                let (mut fd,root,end,body,_,_)=checkpoint_call_fixture(width,false,false);
                let stack=Rc::clone(fd.get_arch().manage().get_stack_space().unwrap());
                let entry=fd.find_spacebase_input(&stack).unwrap();
                let address=fd.new_op(2,fd.get_address().clone());
                fd.op_set_opcode_code(address,OpCode::CPUI_INT_ADD);
                let offset=fd.new_constant(width,(-48_i64) as u64 & calc_mask(width));
                fd.op_set_all_input(address,&[entry,offset]).unwrap();
                let local=fd.new_unique_out(width,address).unwrap();
                fd.op_insert_before(address,root.op);
                let phi=fd.new_op(2,fd.get_address().clone());
                fd.op_set_opcode_code(phi,OpCode::CPUI_MULTIEQUAL);
                fd.op_set_all_input(phi,&[if mixed {root.output} else {local},root.before]).unwrap();
                let pointer=fd.new_unique_out(width,phi).unwrap();
                fd.op_insert_begin(phi,body);
                let memory=fd.new_constant(4,stack.get_contain().unwrap().get_index() as u64);
                let one=fd.new_constant(width,1);
                let two=fd.new_constant(width,2);
                let first=fd.new_op(3,fd.get_address().clone());
                fd.op_set_opcode_code(first,OpCode::CPUI_STORE);
                fd.op_set_all_input(first,&[memory,pointer,one]).unwrap();
                fd.op_insert_end(first,body);
                let indexed=fd.new_op(3,fd.get_address().clone());
                fd.op_set_opcode_code(indexed,OpCode::CPUI_PTRADD);
                let stride=fd.new_constant(4,1);
                let register=Rc::clone(fd.get_arch().manage().get_space_by_name("register").unwrap());
                let input=fd.new_varnode(width,&Address::new(register,0x180),None);
                let index=fd.set_input_varnode(input).unwrap();
                fd.op_set_all_input(indexed,&[root.output,index,stride]).unwrap();
                let other=fd.new_unique_out(width,indexed).unwrap();
                fd.op_insert_end(indexed,body);
                let second=fd.new_op(3,fd.get_address().clone());
                fd.op_set_opcode_code(second,OpCode::CPUI_STORE);
                fd.op_set_all_input(second,&[memory,other,two]).unwrap();
                fd.op_insert_end(second,body);
                let load=fd.new_op(2,fd.get_address().clone());
                fd.op_set_opcode_code(load,OpCode::CPUI_LOAD);
                fd.op_set_all_input(load,&[memory,pointer]).unwrap();
                let value=fd.new_unique_out(width,load).unwrap();
                fd.op_insert_before(load,end.restore);
                let facts=FrameFacts::infer(&fd,vec![root]).unwrap();
                assert_eq!(facts.equations.delta(&fd,value,one),(!mixed).then_some(0),
                    "width={width}, mixed={mixed}: may-private is not definite compiler ownership");
            }
        }
    }

    #[test]
    fn finite_noncompiler_phi_arms_cannot_prove_private_ownership() {
        for width in [4,8] {
            for kind in ["scalar","unknown_return","scalar_return","arithmetic"] {
                let (mut fd,root,end,body,_,_)=checkpoint_call_fixture(width,false,false);
                let stack=Rc::clone(fd.get_arch().manage().get_stack_space().unwrap());
                let entry=fd.find_spacebase_input(&stack).unwrap();
                let address=fd.new_op(2,fd.get_address().clone());
                fd.op_set_opcode_code(address,OpCode::CPUI_INT_ADD);
                let offset=fd.new_constant(width,(-48_i64) as u64 & calc_mask(width));
                fd.op_set_all_input(address,&[entry,offset]).unwrap();
                let local=fd.new_unique_out(width,address).unwrap();
                fd.op_insert_before(address,root.op);
                let register=Rc::clone(fd.get_arch().manage().get_space_by_name("register").unwrap());
                let input=fd.new_varnode(width,&Address::new(register,0x200),None);
                let scalar=fd.set_input_varnode(input).unwrap();
                let arm=match kind {
                    "scalar"=>scalar,
                    "arithmetic"=>{
                        let add=fd.new_op(2,fd.get_address().clone());
                        fd.op_set_opcode_code(add,OpCode::CPUI_INT_ADD);
                        fd.op_set_all_input(add,&[entry,scalar]).unwrap();
                        let value=fd.new_unique_out(width,add).unwrap();
                        fd.op_insert_after(add,root.op);
                        value
                    }
                    _=>{
                        let call=fd.new_op(2,fd.get_address().clone());
                        fd.op_set_opcode_code(call,OpCode::CPUI_CALL);
                        let target=fd.new_constant(width,0x8000);
                        fd.op_set_all_input(call,&[target,root.output]).unwrap();
                        let value=fd.new_unique_out(width,call).unwrap();
                        fd.op_insert_after(call,root.op);
                        if kind=="scalar_return" {
                            let mut spec=crate::fspec::FuncCallSpecs::new(call,Address::new_invalid());
                            spec.proto_mut().attach_internal_store(Rc::new(crate::dtype::Datatype::new(width,type_metatype::TYPE_UINT)));
                            spec.proto_mut().set_output_lock(true);
                            fd.push_call_specs(spec);
                        }
                        value
                    }
                };
                let phi=fd.new_op(2,fd.get_address().clone());
                fd.op_set_opcode_code(phi,OpCode::CPUI_MULTIEQUAL);
                fd.op_set_all_input(phi,&[arm,root.before]).unwrap();
                let pointer=fd.new_unique_out(width,phi).unwrap();
                fd.op_insert_begin(phi,body);
                let memory=fd.new_constant(4,stack.get_contain().unwrap().get_index() as u64);
                let one=fd.new_constant(width,1);
                let two=fd.new_constant(width,2);
                let first=fd.new_op(3,fd.get_address().clone());
                fd.op_set_opcode_code(first,OpCode::CPUI_STORE);
                fd.op_set_all_input(first,&[memory,pointer,one]).unwrap();
                fd.op_insert_end(first,body);
                let indexed=fd.new_op(3,fd.get_address().clone());
                fd.op_set_opcode_code(indexed,OpCode::CPUI_PTRADD);
                let stride=fd.new_constant(4,1);
                let register=Rc::clone(fd.get_arch().manage().get_space_by_name("register").unwrap());
                let input=fd.new_varnode(width,&Address::new(register,0x180),None);
                let index=fd.set_input_varnode(input).unwrap();
                fd.op_set_all_input(indexed,&[root.output,index,stride]).unwrap();
                let other=fd.new_unique_out(width,indexed).unwrap();
                fd.op_insert_end(indexed,body);
                let second=fd.new_op(3,fd.get_address().clone());
                fd.op_set_opcode_code(second,OpCode::CPUI_STORE);
                fd.op_set_all_input(second,&[memory,other,two]).unwrap();
                fd.op_insert_end(second,body);
                let load=fd.new_op(2,fd.get_address().clone());
                fd.op_set_opcode_code(load,OpCode::CPUI_LOAD);
                fd.op_set_all_input(load,&[memory,pointer]).unwrap();
                let value=fd.new_unique_out(width,load).unwrap();
                fd.op_insert_before(load,end.restore);
                let facts=FrameFacts::infer(&fd,vec![root]).unwrap();
                assert_eq!(facts.equations.delta(&fd,value,one),None,
                    "width={width}, kind={kind}: may-private is not definite compiler ownership");
            }
        }
    }

    #[test]
    fn finite_opaque_reservations_keep_bounds_without_guessing_counts() {
        for width in [4,8] {
            let (mut fd,mut root,_,_,_,saved)=checkpoint_call_fixture(width,false,false);
            let initial=fd.vbank().get(saved).unwrap().get_def().unwrap();
            let zero=fd.new_constant(width,0);
            fd.op_set_input(initial,zero,0).unwrap();
            fd.op_set_input(root.op,saved,1).unwrap();
            root.amount=saved;
            let stack=Rc::clone(fd.get_arch().manage().get_stack_space().unwrap());
            let entry=fd.find_spacebase_input(&stack).unwrap();
            let address=fd.new_op(2,fd.get_address().clone());
            fd.op_set_opcode_code(address,OpCode::CPUI_INT_ADD);
            let offset=fd.new_constant(width,(-32_i64) as u64 & calc_mask(width));
            fd.op_set_all_input(address,&[entry,offset]).unwrap();
            let pointer=fd.new_unique_out(width,address).unwrap();
            fd.op_insert_before(address,root.op);
            let write=fd.new_op(3,fd.get_address().clone());
            fd.op_set_opcode_code(write,OpCode::CPUI_STORE);
            let memory=fd.new_constant(4,stack.get_contain().unwrap().get_index() as u64);
            let one=fd.new_constant(width,1);
            fd.op_set_all_input(write,&[memory,pointer,one]).unwrap();
            fd.op_insert_before(write,root.op);
            let copy=fd.new_op(1,fd.get_address().clone());
            fd.op_set_opcode_code(copy,OpCode::CPUI_COPY);
            fd.op_set_input(copy,root.output,0).unwrap();
            let snapshot=fd.new_unique_out(width,copy).unwrap();
            fd.op_insert_after(copy,root.op);
            let facts=FrameFacts::infer(&fd,vec![root]).unwrap();
            assert!(!facts.input_valid(root.op,1));
            assert!(facts.equations.value(&fd,root.output).1);
            assert_eq!(facts.equations.delta(&fd,root.output,root.output),Some(0));
            assert_eq!(facts.equations.delta(&fd,snapshot,root.output),Some(0));
            assert_eq!(facts.equations.delta(&fd,root.output,root.before),None);
            let bound=facts.equations.delta(&fd,root.before,entry).unwrap();
            assert_eq!(facts.upper(&fd,root.output,&mut 65536),Some(bound),
                "an unknown exact size does not erase a witnessed downward reservation's upper bound");
        }
    }

    #[test]
    fn native_header_sinks_reject_snapshots_escapes_and_expired_headers() {
        for width in [4,8] {
            for kind in ["native","snapshot","copy_snapshot","foreign_hold","ordinary_use","after_restore","mutated"] {
                let (mut fd,root,end,_)=scoped_lifetime_fixture_abi(width,false,1);
                let stack=Rc::clone(fd.get_arch().manage().get_stack_space().unwrap());
                let entry=fd.find_spacebase_input(&stack).unwrap();
                let point=fd.obank().get(root.op).unwrap().get_addr().clone();
                let memory=fd.new_constant(4,stack.get_contain().unwrap().get_index() as u64);
                let prologue=fd.new_op(3,point.clone());
                fd.op_set_opcode_code(prologue,OpCode::CPUI_STORE);
                fd.op_set_all_input(prologue,&[memory,root.before,entry]).unwrap();
                fd.op_insert_before(prologue,root.op);
                let link=fd.new_op(3,point.clone());
                fd.op_set_opcode_code(link,OpCode::CPUI_STORE);
                fd.op_set_all_input(link,&[memory,root.output,entry]).unwrap();
                fd.op_insert_after(link,root.op);
                let commit=fd.new_op(1,point.clone());
                fd.op_set_opcode_code(commit,OpCode::CPUI_COPY);
                fd.op_set_input(commit,root.output,0).unwrap();
                let sp=fd.vbank().get(root.output).unwrap().get_addr().clone();
                fd.new_varnode_out(width,&sp,commit).unwrap();
                fd.op_insert_after(commit,link);
                let load=fd.new_op(2,fd.obank().get(end.restore).unwrap().get_addr().clone());
                fd.op_set_opcode_code(load,OpCode::CPUI_LOAD);
                fd.op_set_all_input(load,&[memory,root.output]).unwrap();
                let value=fd.new_varnode_out(width,&sp,load).unwrap();
                if kind=="after_restore" {fd.op_insert_after(load,end.restore);}
                else {fd.op_insert_before(load,end.restore);}
                if kind!="after_restore" {
                    let current=fd.new_op(1,point.clone());
                    fd.op_set_opcode_code(current,OpCode::CPUI_COPY);
                    fd.op_set_input(current,root.output,0).unwrap();
                    fd.new_varnode_out(width,&sp,current).unwrap();
                    fd.op_insert_before(current,load);
                }
                fd.vbank_mut().get_mut(value).unwrap().set_auto_live_hold();
                fd.kuna_stack_scope_holds.push(value);
                let native=[(load,value,kind!="foreign_hold")];
                if matches!(kind,"snapshot"|"copy_snapshot") {
                    let mut snapshot=value;
                    let mut previous=load;
                    if kind=="copy_snapshot" {
                        let copy=fd.new_op(1,point.clone());
                        fd.op_set_opcode_code(copy,OpCode::CPUI_COPY);
                        fd.op_set_input(copy,value,0).unwrap();
                        snapshot=fd.new_unique_out(width,copy).unwrap();
                        fd.op_insert_after(copy,load);previous=copy;
                    }
                    let call=fd.new_op(1,point.clone());
                    fd.op_set_opcode_code(call,OpCode::CPUI_CALL);
                    let target=fd.new_constant(width,0x8000);
                    fd.op_set_input(call,target,0).unwrap();
                    fd.op_insert_after(call,previous);
                    fd.kuna_stack_call_sp.insert(call,Some(snapshot));
                }
                if matches!(kind,"ordinary_use"|"mutated") {
                    let write=fd.new_op(3,point.clone());
                    fd.op_set_opcode_code(write,OpCode::CPUI_STORE);
                    let (pointer,data)=if kind=="mutated" {(root.output,fd.new_constant(width,7))}
                        else {(fd.new_constant(width,0x9000),value)};
                    fd.op_set_all_input(write,&[memory,pointer,data]).unwrap();
                    if kind=="mutated" {fd.op_insert_before(write,load);}
                    else {fd.op_insert_after(write,load);}
                }
                let facts=FrameFacts::infer(&fd,vec![root]).unwrap();
                assert_eq!(native_frame_links(&fd,&facts,width,&native).is_ok(),kind=="native",
                    "width={width}, kind={kind}: a metadata-only stack use is not dead ABI bookkeeping");
            }
        }
    }

    #[test]
    fn unknown_narrow_scalar_writes_do_not_publish_full_frame_addresses() {
        for width in [4,8] {
            for bytes in [1,width] {
                let (mut fd,root,end,body,index,_)=checkpoint_call_fixture(width,false,false);
                let stack=Rc::clone(fd.get_arch().manage().get_stack_space().unwrap());
                let entry=fd.find_spacebase_input(&stack).unwrap();
                let slot=Address::new(Rc::clone(&stack),(-48_i64) as u64 & calc_mask(width));
                let initial=fd.new_op(1,fd.get_address().clone());
                fd.op_set_opcode_code(initial,OpCode::CPUI_COPY);
                let zero=fd.new_constant(bytes,0);
                fd.op_set_input(initial,zero,0).unwrap();
                let saved=fd.new_varnode_out(bytes,&slot,initial).unwrap();
                fd.op_insert_before(initial,root.op);
                let pointer=fd.new_op(2,fd.get_address().clone());
                fd.op_set_opcode_code(pointer,OpCode::CPUI_INT_ADD);
                let offset=fd.new_constant(width,slot.get_offset());
                fd.op_set_all_input(pointer,&[entry,offset]).unwrap();
                let address=fd.new_unique_out(width,pointer).unwrap();
                let call=fd.get_call_specs(index).get_op();
                fd.op_insert_before(pointer,call);
                fd.op_set_input(call,address,1).unwrap();
                let marker=fd.new_op(2,fd.get_address().clone());
                fd.op_set_opcode_code(marker,OpCode::CPUI_INDIRECT);
                let effect=fd.new_varnode_iop(call);
                fd.op_set_all_input(marker,&[saved,effect]).unwrap();
                let value=fd.new_varnode_out(bytes,&slot,marker).unwrap();
                fd.op_insert_after(marker,call);
                let write=fd.new_op(3,fd.get_address().clone());
                fd.op_set_opcode_code(write,OpCode::CPUI_STORE);
                let memory=fd.new_constant(4,stack.get_contain().unwrap().get_index() as u64);
                let global=fd.new_constant(width,0x9000);
                fd.op_set_all_input(write,&[memory,global,value]).unwrap();
                fd.op_insert_end(write,body);
                let facts=FrameFacts::infer(&fd,vec![root]).unwrap();
                assert_eq!(facts.equations.delta(&fd,end.value,root.before),(bytes<width).then_some(0),
                    "width={width}, bytes={bytes}: uncertain character data is not a published native pointer");
                assert_eq!(facts.equations.delta(&fd,value,zero),None,"the callee may still change the scalar's value");
            }
        }
    }

    #[test]
    fn partial_loads_do_not_adopt_a_wider_stores_numeric_value() {
        for width in [4,8] {
            let (mut fd,root,end,_,_,_)=checkpoint_call_fixture(width,false,false);
            let stack=Rc::clone(fd.get_arch().manage().get_stack_space().unwrap());
            let entry=fd.find_spacebase_input(&stack).unwrap();
            let address=fd.new_op(2,fd.get_address().clone());
            fd.op_set_opcode_code(address,OpCode::CPUI_INT_ADD);
            let offset=fd.new_constant(width,(-48_i64) as u64 & calc_mask(width));
            fd.op_set_all_input(address,&[entry,offset]).unwrap();
            let pointer=fd.new_unique_out(width,address).unwrap();
            fd.op_insert_before(address,end.restore);
            let memory=fd.new_constant(4,stack.get_contain().unwrap().get_index() as u64);
            let data=fd.new_constant(width,0x1234);
            let store=fd.new_op(3,fd.get_address().clone());
            fd.op_set_opcode_code(store,OpCode::CPUI_STORE);
            fd.op_set_all_input(store,&[memory,pointer,data]).unwrap();
            fd.op_insert_before(store,end.restore);
            let mut values=Vec::new();
            for bytes in [1,width] {
                let load=fd.new_op(2,fd.get_address().clone());
                fd.op_set_opcode_code(load,OpCode::CPUI_LOAD);
                fd.op_set_all_input(load,&[memory,pointer]).unwrap();
                values.push(fd.new_unique_out(bytes,load).unwrap());
                fd.op_insert_before(load,end.restore);
            }
            let facts=FrameFacts::infer(&fd,vec![root]).unwrap();
            assert_ne!(number_using(&fd,values[0],&|value|facts.equations.value(&fd,value)),Some(0x1234),
                "one byte cannot acquire an entire native word's numeric value");
            assert_eq!(number_using(&fd,values[1],&|value|facts.equations.value(&fd,value)),Some(0x1234));
        }
    }

    #[test]
    fn native_call_bounds_reuse_physical_sp_without_overriding_known_snapshots() {
        for width in [4,8] {
            for kind in ["physical","exact_snapshot","higher_snapshot","unknown"] {
                let (mut fd,root,end,_,index,_)=checkpoint_call_fixture(width,false,false);
                let call=fd.get_call_specs(index).get_op();
                fd.kuna_stack_call_sp.remove(&call);
                let current=fd.new_op(1,fd.obank().get(root.op).unwrap().get_addr().clone());
                fd.op_set_opcode_code(current,if kind=="unknown" {OpCode::CPUI_CALLOTHER} else {OpCode::CPUI_COPY});
                let input=if kind=="unknown" {fd.new_constant(4,0)} else {root.output};
                fd.op_set_input(current,input,0).unwrap();
                let sp=fd.vbank().get(root.output).unwrap().get_addr().clone();
                let value=fd.new_varnode_out(width,&sp,current).unwrap();
                fd.op_insert_before(current,call);
                if kind=="exact_snapshot" {fd.kuna_stack_call_sp.insert(call,Some(value));}
                if kind=="higher_snapshot" {
                    let stack=fd.get_arch().manage().get_stack_space().unwrap();
                    let entry=fd.find_spacebase_input(stack).unwrap();
                    fd.kuna_stack_call_sp.insert(call,Some(entry));
                }
                let facts=FrameFacts::infer(&fd,vec![root]).unwrap();
                assert_eq!(facts.equations.delta(&fd,end.value,root.before),
                    matches!(kind,"physical"|"exact_snapshot").then_some(0),"width={width}, kind={kind}");
            }
        }
    }

    #[test]
    fn dead_native_sp_selectors_do_not_require_every_arm_to_be_a_source_reservation() {
        for width in [4,8] {
            for (visible,copied) in [(false,false),(true,false),(false,true),(true,true)] {
                let (mut fd,mut root,end,header)=scoped_lifetime_fixture_abi(width,false,1);
                let stack=Rc::clone(fd.get_arch().manage().get_stack_space().unwrap());
                let entry_sp=fd.find_spacebase_input(&stack).unwrap();
                let entry=fd.obank().get(root.op).unwrap().get_parent().unwrap();
                let graph=fd.bblocks_root_pub();
                let body=fd.bblocks_mut().new_block_basic(graph);
                fd.bblocks_mut().remove_edge(header,header);
                fd.bblocks_mut().add_edge(header,body);
                fd.bblocks_mut().add_edge(body,header);
                let sp=fd.vbank().get(root.output).unwrap().get_addr().clone();
                let phi=fd.new_op(2,fd.get_address().clone());
                fd.op_set_opcode_code(phi,OpCode::CPUI_MULTIEQUAL);
                fd.op_set_all_input(phi,&[root.before,root.output]).unwrap();
                let selected=fd.new_varnode_out(width,&sp,phi).unwrap();
                fd.op_insert_begin(phi,header);
                fd.op_set_input(root.op,selected,0).unwrap();
                root.before=selected;root.base=selected;
                fd.op_uninsert(root.op);fd.op_insert_end(root.op,body);
                let point=fd.obank().get(root.op).unwrap().get_addr().clone();
                let memory=fd.new_constant(4,stack.get_contain().unwrap().get_index() as u64);
                let link=fd.new_op(3,point.clone());
                fd.op_set_opcode_code(link,OpCode::CPUI_STORE);
                fd.op_set_all_input(link,&[memory,root.output,entry_sp]).unwrap();
                fd.op_insert_after(link,root.op);
                let commit=fd.new_op(1,point.clone());
                fd.op_set_opcode_code(commit,OpCode::CPUI_COPY);
                fd.op_set_input(commit,root.output,0).unwrap();
                fd.new_varnode_out(width,&sp,commit).unwrap();
                fd.op_insert_after(commit,link);
                let load=fd.new_op(2,fd.obank().get(end.restore).unwrap().get_addr().clone());
                fd.op_set_opcode_code(load,OpCode::CPUI_LOAD);
                fd.op_set_all_input(load,&[memory,selected]).unwrap();
                let value=if copied {fd.new_unique_out(width,load).unwrap()}
                    else {fd.new_varnode_out(width,&sp,load).unwrap()};
                fd.op_insert_before(load,end.restore);
                let (held,holder)=if copied {
                    let copy=fd.new_op(1,point.clone());
                    fd.op_set_opcode_code(copy,OpCode::CPUI_COPY);
                    fd.op_set_input(copy,value,0).unwrap();
                    let output=fd.new_varnode_out(width,&sp,copy).unwrap();
                    fd.op_insert_after(copy,load);
                    (output,copy)
                } else {(value,load)};
                fd.vbank_mut().get_mut(held).unwrap().set_auto_live_hold();
                fd.kuna_stack_scope_holds.push(held);
                let native=[(holder,held,true)];
                if visible {
                    let write=fd.new_op(3,point);
                    fd.op_set_opcode_code(write,OpCode::CPUI_STORE);
                    let global=fd.new_constant(width,0x9000);
                    fd.op_set_all_input(write,&[memory,global,value]).unwrap();
                    fd.op_insert_after(write,load);
                }
                fd.bblocks_mut().calc_forward_dominator(graph,&[entry]);
                let facts=FrameFacts::infer(&fd,vec![root]).unwrap();
                let result=native_frame_links(&fd,&facts,width,&native);
                assert_eq!(result.is_ok(),!visible,"width={width},visible={visible},copied={copied}");
                if let Ok(stores)=result {assert_eq!(stores,vec![(link,0)]);}
            }
        }
    }

    #[test]
    fn merged_raw_reads_keep_value_origins_without_inheriting_storage_origins() {
        for width in [4,8] {
            for (native,complete,whole) in [
                (false,true,true),(true,true,true),(false,false,true),(true,false,true),
                (false,true,false),(false,false,false),
            ] {
                let (mut fd,root,end,left,index,_)=checkpoint_call_fixture(width,false,false);
                let stack=Rc::clone(fd.get_arch().manage().get_stack_space().unwrap());
                let entry_sp=fd.find_spacebase_input(&stack).unwrap();
                let entry=fd.obank().get(root.op).unwrap().get_parent().unwrap();
                let join=fd.obank().get(end.restore).unwrap().get_parent().unwrap();
                let graph=fd.bblocks_root_pub();
                let right=fd.bblocks_mut().new_block_basic(graph);
                fd.bblocks_mut().remove_edge(left,left);
                fd.bblocks_mut().add_edge(entry,right);
                fd.bblocks_mut().add_edge(right,join);
                let call=fd.get_call_specs(index).get_op();
                let marker=fd.vbank().get(fd.obank().get(end.restore).unwrap().get_in(0).unwrap()).unwrap().get_def().unwrap();
                fd.op_uninsert(call);fd.op_insert_before(call,end.restore);
                fd.op_uninsert(marker);fd.op_insert_after(marker,call);
                let address=fd.new_op(2,fd.get_address().clone());
                fd.op_set_opcode_code(address,OpCode::CPUI_INT_ADD);
                let displacement=fd.new_constant(width,(-48_i64) as u64 & calc_mask(width));
                fd.op_set_all_input(address,&[entry_sp,displacement]).unwrap();
                let pointer=fd.new_unique_out(width,address).unwrap();
                fd.op_insert_before(address,root.op);
                let memory=fd.new_constant(4,stack.get_contain().unwrap().get_index() as u64);
                for (block,value) in [(left,if native {entry_sp} else {root.amount}),(right,root.amount)] {
                    if block==right && !complete {continue;}
                    let value=if whole {value} else {fd.new_constant(width/2,0)};
                    let write=fd.new_op(3,fd.get_address().clone());
                    fd.op_set_opcode_code(write,OpCode::CPUI_STORE);
                    fd.op_set_all_input(write,&[memory,pointer,value]).unwrap();
                    fd.op_insert_end(write,block);
                }
                let load=fd.new_op(2,fd.get_address().clone());
                fd.op_set_opcode_code(load,OpCode::CPUI_LOAD);
                fd.op_set_all_input(load,&[memory,pointer]).unwrap();
                let loaded=fd.new_unique_out(width,load).unwrap();
                fd.op_insert_before(load,call);
                let reach=fd.new_op(2,fd.get_address().clone());
                fd.op_set_opcode_code(reach,OpCode::CPUI_INT_ADD);
                let displacement=fd.new_constant(width,(-32_i64) as u64 & calc_mask(width));
                fd.op_set_all_input(reach,&[loaded,displacement]).unwrap();
                let argument=fd.new_unique_out(width,reach).unwrap();
                fd.op_insert_before(reach,call);
                fd.op_set_input(call,argument,1).unwrap();
                fd.bblocks_mut().calc_forward_dominator(graph,&[entry]);
                assert_eq!(restores_checkpoint(&mut fd,&root,&end),!native && complete && whole,
                    "width={width},native={native},complete={complete},whole={whole}");
            }
        }
    }

    #[test]
    fn incoming_abi_words_without_a_writer_keep_native_pointer_origins() {
        for width in [4,8] {
            for offset in [0,16] {
                let (mut fd,root,end,_,index,_)=checkpoint_call_fixture(width,false,false);
                let stack=Rc::clone(fd.get_arch().manage().get_stack_space().unwrap());
                let entry=fd.find_spacebase_input(&stack).unwrap();
                let call=fd.get_call_specs(index).get_op();
                let address=fd.new_op(2,fd.get_address().clone());
                fd.op_set_opcode_code(address,OpCode::CPUI_INT_ADD);
                let offset=fd.new_constant(width,offset);
                fd.op_set_all_input(address,&[entry,offset]).unwrap();
                let pointer=fd.new_unique_out(width,address).unwrap();
                fd.op_insert_before(address,call);
                let load=fd.new_op(2,fd.get_address().clone());
                fd.op_set_opcode_code(load,OpCode::CPUI_LOAD);
                let memory=fd.new_constant(4,stack.get_contain().unwrap().get_index() as u64);
                fd.op_set_all_input(load,&[memory,pointer]).unwrap();
                let value=fd.new_unique_out(width,load).unwrap();
                fd.op_insert_before(load,call);
                let reach=fd.new_op(2,fd.get_address().clone());
                fd.op_set_opcode_code(reach,OpCode::CPUI_INT_ADD);
                let offset=fd.new_constant(width,(-32_i64) as u64 & calc_mask(width));
                fd.op_set_all_input(reach,&[value,offset]).unwrap();
                let argument=fd.new_unique_out(width,reach).unwrap();
                fd.op_insert_before(reach,call);
                fd.op_set_input(call,argument,1).unwrap();
                assert!(!restores_checkpoint(&mut fd,&root,&end));
            }
        }
    }

    #[test]
    fn checkpoint_reads_follow_preserved_frame_pointer_spills() {
        for width in [4,8] {
            for damage in 0..5 {
                let (mut fd,root,end,_,index,_)=checkpoint_call_fixture(width,false,false);
                let stack=Rc::clone(fd.get_arch().manage().get_stack_space().unwrap());
                let entry=fd.find_spacebase_input(&stack).unwrap();
                let call=fd.get_call_specs(index).get_op();
                let point=fd.get_address().clone();
                let address=fd.new_op(2,point.clone());
                fd.op_set_opcode_code(address,OpCode::CPUI_INT_ADD);
                let offset=fd.new_constant(width,(-48_i64) as u64 & calc_mask(width));
                fd.op_set_all_input(address,&[entry,offset]).unwrap();
                let fp_slot=fd.new_unique_out(width,address).unwrap();
                fd.op_insert_before(address,root.op);
                let memory=fd.new_constant(4,stack.get_contain().unwrap().get_index() as u64);
                let store=fd.new_op(3,point.clone());
                fd.op_set_opcode_code(store,OpCode::CPUI_STORE);
                fd.op_set_all_input(store,&[memory,fp_slot,root.before]).unwrap();
                fd.op_insert_before(store,root.op);
                let fp_load=fd.new_op(2,point.clone());
                fd.op_set_opcode_code(fp_load,OpCode::CPUI_LOAD);
                fd.op_set_all_input(fp_load,&[memory,fp_slot]).unwrap();
                let frame=fd.new_unique_out(width,fp_load).unwrap();
                fd.op_insert_before(fp_load,end.restore);
                let locate=fd.new_op(2,point.clone());
                fd.op_set_opcode_code(locate,OpCode::CPUI_INT_ADD);
                let offset=fd.new_constant(width,32);
                fd.op_set_all_input(locate,&[frame,offset]).unwrap();
                let checkpoint=fd.new_unique_out(width,locate).unwrap();
                fd.op_insert_before(locate,end.restore);
                if damage==2 {
                    let write=fd.new_op(3,point.clone());
                    fd.op_set_opcode_code(write,OpCode::CPUI_STORE);
                    let byte=fd.new_constant(1,1);
                    fd.op_set_all_input(write,&[memory,fp_slot,byte]).unwrap();
                    fd.op_insert_before(write,fp_load);
                }
                if damage==3 {
                    let index=fd.new_op(2,point.clone());
                    fd.op_set_opcode_code(index,OpCode::CPUI_INT_ADD);
                    fd.op_set_all_input(index,&[root.before,root.amount]).unwrap();
                    let target=fd.new_unique_out(width,index).unwrap();
                    fd.op_insert_before(index,fp_load);
                    let write=fd.new_op(3,point.clone());
                    fd.op_set_opcode_code(write,OpCode::CPUI_STORE);
                    let value=fd.new_constant(width,0);
                    fd.op_set_all_input(write,&[memory,target,value]).unwrap();
                    fd.op_insert_before(write,fp_load);
                }
                if damage==4 {
                    let left=fd.obank().get(call).unwrap().get_parent().unwrap();
                    let entry_block=fd.obank().get(root.op).unwrap().get_parent().unwrap();
                    let join=fd.obank().get(end.restore).unwrap().get_parent().unwrap();
                    let graph=fd.bblocks_root_pub();
                    let right=fd.bblocks_mut().new_block_basic(graph);
                    fd.bblocks_mut().remove_edge(left,left);
                    fd.bblocks_mut().add_edge(entry_block,right);
                    fd.bblocks_mut().add_edge(right,join);
                    let index=fd.new_op(2,point.clone());
                    fd.op_set_opcode_code(index,OpCode::CPUI_INT_ADD);
                    let offset=fd.new_constant(width,(-32_i64) as u64 & calc_mask(width));
                    fd.op_set_all_input(index,&[entry,offset]).unwrap();
                    let target=fd.new_unique_out(width,index).unwrap();
                    fd.op_insert_end(index,left);
                    let write=fd.new_op(3,point.clone());
                    fd.op_set_opcode_code(write,OpCode::CPUI_STORE);
                    let value=fd.new_constant(width,0);
                    fd.op_set_all_input(write,&[memory,target,value]).unwrap();
                    fd.op_insert_end(write,left);
                    fd.bblocks_mut().calc_forward_dominator(graph,&[entry_block]);
                }
                if damage==1 {
                    let write=fd.new_op(3,point.clone());
                    fd.op_set_opcode_code(write,OpCode::CPUI_STORE);
                    let byte=fd.new_constant(1,1);
                    fd.op_set_all_input(write,&[memory,checkpoint,byte]).unwrap();
                    fd.op_insert_before(write,end.restore);
                }
                let read=fd.new_op(2,point);
                fd.op_set_opcode_code(read,OpCode::CPUI_LOAD);
                fd.op_set_all_input(read,&[memory,checkpoint]).unwrap();
                let value=fd.new_unique_out(width,read).unwrap();
                fd.op_insert_before(read,end.restore);
                fd.op_set_input(end.restore,value,0).unwrap();
                assert_eq!(restores_checkpoint(&mut fd,&root,&end),damage==0,"width={width},damage={damage}");
                let _=call;
            }
        }
    }

    #[test]
    fn source_addresses_above_sp_require_a_matching_physical_reservation() {
        for width in [4,8] {
            for offset in [8,88,128] {
                for (extra,source_sp,temporary) in [0,offset-1,offset,offset+1,offset+16].into_iter()
                    .flat_map(|extra|[(extra,false,false),(extra,true,false),(extra,false,true),(extra,true,true)]) {
                    let (mut fd,physical,end,_)=scoped_lifetime_fixture(width,false);
                    let point=fd.obank().get(physical.op).unwrap().get_addr().clone();
                    let address=fd.new_op(2,point.clone());
                    fd.op_set_opcode_code(address,OpCode::CPUI_INT_ADD);
                    let displacement=fd.new_constant(width,offset);
                    fd.op_set_all_input(address,&[physical.before,displacement]).unwrap();
                    let base=fd.new_unique_out(width,address).unwrap();
                    fd.op_insert_before(address,physical.op);
                    let reserve=fd.new_op(2,point);
                    fd.op_set_opcode_code(reserve,OpCode::CPUI_INT_SUB);
                    fd.op_set_all_input(reserve,&[base,physical.amount]).unwrap();
                    let sp=fd.vbank().get(physical.output).unwrap().get_addr().clone();
                    let output=if source_sp {fd.new_varnode_out(width,&sp,reserve).unwrap()}
                        else {fd.new_unique_out(width,reserve).unwrap()};
                    fd.op_insert_before(reserve,physical.op);
                    if temporary {
                        let scratch=fd.new_op(2,fd.obank().get(physical.op).unwrap().get_addr().clone());
                        fd.op_set_opcode_code(scratch,OpCode::CPUI_INT_ADD);
                        let shift=fd.new_constant(width,(offset+32).wrapping_neg() & calc_mask(width));
                        fd.op_set_all_input(scratch,&[output,shift]).unwrap();
                        fd.new_varnode_out(width,&sp,scratch).unwrap();
                        fd.op_insert_before(scratch,physical.op);
                    }
                    fd.op_set_opcode_code(physical.op,OpCode::CPUI_INT_ADD);
                    let shift=fd.new_constant(width,extra.wrapping_neg() & calc_mask(width));
                    fd.op_set_all_input(physical.op,&[output,shift]).unwrap();
                    let mut source=Allocation {op:reserve,before:base,base,output,..physical};
                    normalize_reservation(&fd,&mut source);
                    let facts=FrameFacts::infer(&fd,vec![source]).unwrap();
                    assert_eq!(facts.restoration(&fd,&source,end.value).is_some(),extra>=offset,
                        "width={width},offset={offset},extra={extra},source_sp={source_sp},temporary={temporary}");
                    if extra>=offset {
                        let before = facts.equations.delta(&fd, physical.before, facts.entry).unwrap();
                        let word = FrameWordUse {
                            origin_only: false, result: None, source: output, first: None,
                            pointer: Some(output), size: 1, read: None, birth: None, alias: false,
                        };
                        // An opaque count of one puts the source byte at base - 1,
                        // even though the committed SP is below the old SP.
                        assert!(facts.stack_write_reaches(&fd, before + offset as i128 - 1, 1, word));
                        let backing=reserves(&fd,&[source],&[],&[],width).unwrap()[0];
                        assert!(backing.shift>=extra && backing.count>=backing.shift,
                            "native and source accesses must lie inside the new backing");
                        assert!(AllocationPlan::prepare(&fd,&facts,&[],width,&[physical.op],&[],&[]).is_ok(),
                            "the witnessed scope must reach a complete lowerable plan");
                    }
                }
            }
        }
    }

    #[test]
    fn logical_store_slots_do_not_override_actual_memory_space_or_pointer_reads() {
        for width in [4,8] {
            for wrong_space in [false,true] {
                for direct in [false,true] {
                    let (mut fd,root,end,_,index,_)=checkpoint_call_fixture(width,false,false);
                    let stack=Rc::clone(fd.get_arch().manage().get_stack_space().unwrap());
                    let entry=fd.find_spacebase_input(&stack).unwrap();
                    let call=fd.get_call_specs(index).get_op();
                    let point=fd.get_address().clone();
                    let address=fd.new_op(2,point.clone());
                    fd.op_set_opcode_code(address,OpCode::CPUI_INT_ADD);
                    let offset=fd.new_constant(width,(-48_i64) as u64 & calc_mask(width));
                    fd.op_set_all_input(address,&[entry,offset]).unwrap();
                    let target=fd.new_unique_out(width,address).unwrap();
                    fd.op_insert_before(address,call);
                    let memory=fd.new_constant(4,stack.get_contain().unwrap().get_index() as u64);
                    let pointer=if wrong_space {target} else {
                        let save=fd.new_op(1,point.clone());
                        fd.op_set_opcode_code(save,OpCode::CPUI_COPY);
                        fd.op_set_input(save,target,0).unwrap();
                        let storage=Address::new(Rc::clone(&stack),(-56_i64) as u64 & calc_mask(width));
                        let value=fd.new_varnode_out(width,&storage,save).unwrap();
                        fd.op_insert_before(save,call);
                        let locate=fd.new_op(2,point.clone());
                        fd.op_set_opcode_code(locate,OpCode::CPUI_INT_ADD);
                        let offset=fd.new_constant(width,(-56_i64) as u64 & calc_mask(width));
                        fd.op_set_all_input(locate,&[entry,offset]).unwrap();
                        let address=fd.new_unique_out(width,locate).unwrap();
                        fd.op_insert_before(locate,call);
                        let clobber=fd.new_op(3,point.clone());
                        fd.op_set_opcode_code(clobber,OpCode::CPUI_STORE);
                        let byte=fd.new_constant(1,1);
                        fd.op_set_all_input(clobber,&[memory,address,byte]).unwrap();
                        fd.op_insert_before(clobber,call);
                        value
                    };
                    let store=fd.new_op(3,point.clone());
                    fd.op_set_opcode_code(store,OpCode::CPUI_STORE);
                    let space=if wrong_space {fd.new_constant(4,stack.get_index() as u64)} else {memory};
                    fd.op_set_all_input(store,&[space,pointer,root.amount]).unwrap();
                    let storage=Address::new(Rc::clone(&stack),(-48_i64) as u64 & calc_mask(width));
                    let nominal=fd.new_varnode_out(width,&storage,store).unwrap();
                    fd.op_insert_before(store,call);
                    let value=if direct {nominal} else {
                        let read=fd.new_op(2,point.clone());
                        fd.op_set_opcode_code(read,OpCode::CPUI_LOAD);
                        fd.op_set_all_input(read,&[memory,target]).unwrap();
                        let value=fd.new_unique_out(width,read).unwrap();
                        fd.op_insert_before(read,call);
                        value
                    };
                    let reach=fd.new_op(2,point);
                    fd.op_set_opcode_code(reach,OpCode::CPUI_INT_ADD);
                    let offset=fd.new_constant(width,(-32_i64) as u64 & calc_mask(width));
                    fd.op_set_all_input(reach,&[value,offset]).unwrap();
                    let argument=fd.new_unique_out(width,reach).unwrap();
                    fd.op_insert_before(reach,call);
                    fd.op_set_input(call,argument,1).unwrap();
                    assert!(!restores_checkpoint(&mut fd,&root,&end),"width={width},wrong_space={wrong_space},direct={direct}");
                }
            }
        }
    }

    #[test]
    fn scalar_byte_ranges_must_cover_each_read_on_every_entry_path() {
        for width in [4,8] {
            for mode in 0..4 {
                let (mut fd,root,end,_,index,_)=checkpoint_call_fixture(width,false,false);
                let stack=Rc::clone(fd.get_arch().manage().get_stack_space().unwrap());
                let entry=fd.find_spacebase_input(&stack).unwrap();
                let call=fd.get_call_specs(index).get_op();
                let point=fd.get_address().clone();
                let memory=fd.new_constant(4,stack.get_contain().unwrap().get_index() as u64);
                let locate=fd.new_op(2,point.clone());
                fd.op_set_opcode_code(locate,OpCode::CPUI_INT_ADD);
                let offset=fd.new_constant(width,(-48_i64) as u64 & calc_mask(width));
                fd.op_set_all_input(locate,&[entry,offset]).unwrap();
                let pointer=fd.new_unique_out(width,locate).unwrap();
                fd.op_insert_before(locate,call);
                let read=fd.new_op(2,point.clone());
                fd.op_set_opcode_code(read,OpCode::CPUI_LOAD);
                fd.op_set_all_input(read,&[memory,pointer]).unwrap();
                let value=fd.new_unique_out(width,read).unwrap();
                fd.op_insert_before(read,call);
                for half in 0..2 {
                    let locate=fd.new_op(2,point.clone());
                    fd.op_set_opcode_code(locate,OpCode::CPUI_INT_ADD);
                    let offset=fd.new_constant(width,(half*(width/2)+int4::from(mode==2 && half==1)) as u64);
                    fd.op_set_all_input(locate,&[pointer,offset]).unwrap();
                    let target=fd.new_unique_out(width,locate).unwrap();
                    fd.op_insert_before(locate,read);
                    let data=if mode==1 && half==0 {
                        let low=fd.new_op(2,point.clone());
                        fd.op_set_opcode_code(low,OpCode::CPUI_SUBPIECE);
                        let zero=fd.new_constant(4,0);
                        fd.op_set_all_input(low,&[entry,zero]).unwrap();
                        let value=fd.new_unique_out(width/2,low).unwrap();
                        fd.op_insert_before(low,read);
                        value
                    } else {fd.new_constant(width/2,0)};
                    let write=fd.new_op(3,point.clone());
                    fd.op_set_opcode_code(write,OpCode::CPUI_STORE);
                    fd.op_set_all_input(write,&[memory,target,data]).unwrap();
                    if mode==3 && half==1 {fd.op_insert_after(write,read);}
                    else {fd.op_insert_before(write,read);}
                }
                let reach=fd.new_op(2,point);
                fd.op_set_opcode_code(reach,OpCode::CPUI_INT_ADD);
                let offset=fd.new_constant(width,(-32_i64) as u64 & calc_mask(width));
                fd.op_set_all_input(reach,&[value,offset]).unwrap();
                let argument=fd.new_unique_out(width,reach).unwrap();
                fd.op_insert_before(reach,call);
                fd.op_set_input(call,argument,1).unwrap();
                assert_eq!(restores_checkpoint(&mut fd,&root,&end),mode==0,"width={width},mode={mode}");
            }
        }
    }

}
