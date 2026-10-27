//! Recover runtime stack allocation and bound its outgoing stack area over SSA.

use crate::action::{Action, ActionBase, ActionContext, ActionGroupList, ApplyResult};
use crate::context::{BlockId, OpId, VarnodeId};
use crate::funcdata::Funcdata;
use crate::op::pcodeop_flags;
use kuna_base::address::{calc_mask, sign_extend};
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;
use std::collections::{BTreeMap, BTreeSet};

/// Internal allocation result is an address-sized integer, including in C.
pub const BUILTIN_STACK_ALLOC: u32 = 0x10000009;
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
        value = input;
    }
    value
}
fn literal(fd: &Funcdata, value: VarnodeId, budget: &mut usize) -> Option<u64> {
    *budget = budget.checked_sub(1)?;
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
    let a = literal(fd, o.get_in(0)?, budget)?;
    if matches!(o.code(), OpCode::CPUI_COPY | OpCode::CPUI_CAST) {
        return Some(a & calc_mask(size));
    }
    let behavior = fd.get_arch().op_behavior(o.code())?;
    let input_size = fd.vbank().get(o.get_in(0)?)?.get_size();
    let result = if behavior.is_unary() {
        behavior.evaluate_unary(size, input_size, a).ok()?
    } else {
        let b = literal(fd, o.get_in(1)?, budget)?;
        behavior.evaluate_binary(size, input_size, a, b).ok()?
    };
    Some(result & calc_mask(size))
}
fn number(fd: &Funcdata, value: VarnodeId) -> Option<i128> {
    let width = fd.vbank().get(value)?.get_size();
    let raw = literal(fd, value, &mut 32)?;
    Some(sign_extend(raw as i64, width * 8 - 1) as i128)
}
fn negate(fd: &Funcdata, value: VarnodeId) -> Option<VarnodeId> {
    let value = strip(fd, value);
    let op = fd.obank().get(fd.vbank().get(value)?.get_def()?)?;
    match op.code() {
        OpCode::CPUI_INT_2COMP => op.get_in(0),
        OpCode::CPUI_INT_MULT if number(fd, op.get_in(1)?) == Some(-1) => op.get_in(0),
        // SLEIGH address expressions retain an explicit unit scale before
        // arithmetic simplification, including around a negated index.
        OpCode::CPUI_INT_MULT if number(fd, op.get_in(1)?) == Some(1) => negate(fd, op.get_in(0)?),
        OpCode::CPUI_INT_MULT if number(fd, op.get_in(0)?) == Some(1) => negate(fd, op.get_in(1)?),
        OpCode::CPUI_INT_SUB if number(fd, op.get_in(0)?) == Some(0) => op.get_in(1),
        _ if constant_step(fd, value).is_some() => {
            let (base, offset) = canonical_base(fd, value, &mut BTreeSet::new());
            (base != value && offset == 0)
                .then(|| negate(fd, base))
                .flatten()
        }
        _ => None,
    }
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
fn constant_step(fd: &Funcdata, value: VarnodeId) -> Option<(VarnodeId, i128)> {
    let op = fd.obank().get(fd.vbank().get(value)?.get_def()?)?;
    if op.code() == OpCode::CPUI_INT_ADD {
        if let Some(amount) = number(fd, op.get_in(0)?) {
            return Some((op.get_in(1)?, amount));
        }
    }
    let amount = number(fd, op.get_in(1)?)?;
    match op.code() {
        OpCode::CPUI_INT_ADD | OpCode::CPUI_PTRSUB => Some((op.get_in(0)?, amount)),
        OpCode::CPUI_INT_SUB => Some((op.get_in(0)?, -amount)),
        OpCode::CPUI_PTRADD => Some((
            op.get_in(0)?,
            amount.checked_mul(number(fd, op.get_in(2)?)?)?,
        )),
        _ => None,
    }
}

// Equal steps and predecessor-aligned PHIs preserve p - n. This recognizes
// pointer/counter probe loops, including their zero-iteration exit PHIs, without
// removing the loop or asserting a bound on its iteration count.
fn paired_origins(
    fd: &Funcdata,
    p: VarnodeId,
    n: VarnodeId,
    seen: &mut BTreeSet<(VarnodeId, VarnodeId)>,
) -> Option<BTreeSet<(VarnodeId, VarnodeId)>> {
    let p = strip(fd, p);
    let n = strip(fd, n);
    if !seen.insert((p, n)) {
        return Some(BTreeSet::new());
    }
    if let (Some((a, da)), Some((b, db))) = (constant_step(fd, p), constant_step(fd, n)) {
        if da == db {
            return paired_origins(fd, a, b, seen);
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
                origins.extend(paired_origins(fd, a.get_in(slot)?, b.get_in(slot)?, seen)?);
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
    base: VarnodeId,
}
fn allocation(fd: &Funcdata, op: OpId) -> Option<Allocation> {
    let o = fd.obank().get(op)?;
    let (base, amount) = match o.code() {
        OpCode::CPUI_INT_SUB if number(fd, o.get_in(1)?).is_none() => (o.get_in(0)?, o.get_in(1)?),
        OpCode::CPUI_INT_ADD => {
            if let Some(amount) = negate(fd, o.get_in(1)?) {
                (o.get_in(0)?, amount)
            } else {
                (o.get_in(1)?, negate(fd, o.get_in(0)?)?)
            }
        }
        OpCode::CPUI_PTRADD if number(fd, o.get_in(2)?) == Some(1) => {
            (o.get_in(0)?, negate(fd, o.get_in(1)?)?)
        }
        _ => return None,
    };
    let origins = paired_origins(fd, base, amount, &mut BTreeSet::new())?;
    if origins.len() != 1 {
        return None;
    }
    let (base, amount) = origins.into_iter().next()?;
    if stepped_cycle(fd, base) {
        return None;
    }
    Some(Allocation {
        op,
        output: o.get_out()?,
        amount,
        base,
    })
}
fn candidates(
    fd: &Funcdata,
    value: VarnodeId,
    seen: &mut BTreeSet<VarnodeId>,
    result: &mut impl FnMut(Allocation),
) {
    if !seen.insert(value) {
        return;
    }
    let Some(id) = fd.vbank().get(value).and_then(|v| v.get_def()) else {
        return;
    };
    if let Some(found) = allocation(fd, id) {
        result(found);
        return;
    }
    let Some(op) = fd.obank().get(id) else { return };
    match op.code() {
        OpCode::CPUI_COPY | OpCode::CPUI_CAST | OpCode::CPUI_INDIRECT => {
            if let Some(v) = op.get_in(0) {
                candidates(fd, v, seen, result);
            }
        }
        OpCode::CPUI_INT_AND | OpCode::CPUI_MULTIEQUAL => {
            for slot in 0..op.num_input() {
                if let Some(v) = op.get_in(slot) {
                    candidates(fd, v, seen, result);
                }
            }
        }
        OpCode::CPUI_INT_ADD | OpCode::CPUI_INT_SUB | OpCode::CPUI_PTRADD | OpCode::CPUI_PTRSUB => {
            if let Some(v) = op.get_in(0) {
                candidates(fd, v, seen, result);
            }
        }
        _ => {}
    }
}
/// An expression feeding SP may precede its COPY, but not an observable effect.
fn reservation_point(fd: &Funcdata, point: OpId, stack_write: OpId) -> bool {
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
        let evaluation = op.get_eval_type();
        if evaluation == 0
            || evaluation & pcodeop_flags::special != 0
            || matches!(
                op.code(),
                OpCode::CPUI_INT_DIV
                    | OpCode::CPUI_INT_SDIV
                    | OpCode::CPUI_INT_REM
                    | OpCode::CPUI_INT_SREM
            )
        {
            return false;
        }
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
        if op.code() != OpCode::CPUI_MULTIEQUAL {
            return dominates_op(fd, definition, user);
        }
        let Some(parent) = op.get_parent() else {
            return false;
        };
        let Some(block) = fd.obank().get(definition).and_then(|o| o.get_parent()) else {
            return false;
        };
        (0..op.num_input())
            .filter(|&slot| op.get_in(slot) == Some(value))
            .all(|slot| {
                let predecessor = fd.bblocks_ref().block(parent).get_in(slot);
                fd.bblocks_ref().dominates(block, Some(predecessor))
            })
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
fn same_origin(fd: &Funcdata, a: &Allocation, b: &Allocation) -> bool {
    origin_delta(fd, a, b) == Some(0)
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
        offset_b.checked_sub(offset_a)?
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

fn frame_dependent(fd: &Funcdata, value: VarnodeId, budget: &mut usize) -> bool {
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
        if op.code() != OpCode::CPUI_LOAD {
            todo.extend((0..op.num_input()).filter_map(|slot| op.get_in(slot)));
        }
    }
    false
}

fn may_point_to_slot(
    fd: &Funcdata,
    value: VarnodeId,
    pointer: VarnodeId,
    size: int4,
    extent: int4,
    budget: &mut usize,
) -> bool {
    let Some(v) = fd.vbank().get(value) else {
        return true;
    };
    if v.is_constant() {
        return false;
    }
    let ty = v.get_type();
    let width = extent.max(1);
    if let Some(delta) = relative_offset(fd, value, pointer) {
        return delta < size as i128
            && delta.checked_add(width as i128).is_none_or(|end| end > 0);
    }
    if !matches!(
        ty.get_metatype(),
        crate::dtype::type_metatype::TYPE_PTR | crate::dtype::type_metatype::TYPE_UNKNOWN
    ) && !frame_dependent(fd, value, budget)
    {
        return false;
    }
    let Some(stack) = fd.get_arch().manage().get_stack_space() else {
        return true;
    };
    let Some(input) = fd.find_spacebase_input(&stack) else {
        return true;
    };
    let (base, offset) = canonical_base(fd, value, &mut BTreeSet::new());
    if base == input {
        if let Some(limit) = offset.checked_sub(size as i128) {
            if at_or_below(fd, pointer, input, limit, None, None, budget) {
                return false;
            }
        }
    }
    true
}

/// A saved frame-link address must not have escaped before the guarded call.
fn private_saved_slot(
    fd: &Funcdata,
    call: OpId,
    pointer: VarnodeId,
    size: int4,
    budget: &mut usize,
    active: &mut BTreeSet<OpId>,
) -> Option<()> {
    let stack = fd.get_arch().manage().get_stack_space()?;
    if !stack.stack_grows_negative() {
        return None;
    }
    let input = fd.find_spacebase_input(&stack)?;
    if !at_or_below(fd, pointer, input, -(size as i128), None, None, budget) {
        return None;
    }
    let parent = fd.obank().get(call)?.get_parent()?;
    let graph = fd.bblocks_ref();
    let mut ancestors = BTreeSet::from([parent]);
    let mut todo: Vec<_> = (0..graph.block(parent).size_in())
        .map(|slot| graph.block(parent).get_in(slot))
        .collect();
    while let Some(block) = todo.pop() {
        *budget = budget.checked_sub(1)?;
        if block == parent {
            return None;
        }
        if ancestors.insert(block) {
            let block = graph.block(block);
            todo.extend((0..block.size_in()).map(|slot| block.get_in(slot)));
        }
    }
    for id in fd.obank().iter_alive() {
        let op = fd.obank().get(id)?;
        if !op.get_parent().is_some_and(|block| ancestors.contains(&block))
            || (op.get_parent() == Some(parent) && id != call && !dominates_op(fd, id, call))
        {
            continue;
        }
        *budget = budget.checked_sub(1)?;
        let declaration = if matches!(op.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND) {
            let spec = fd.get_call_specs(fd.get_call_specs_index(id)?);
            if spec.proto().has_effect_override()
                || crate::kuna_calleeprotostack::declared_caller_frame_floor(true, spec.proto()).is_none()
                || op.num_input() != spec.proto().num_params() + 1
            {
                return None;
            }
            Some(spec.proto())
        } else {
            None
        };
        let slots = match op.code() {
            OpCode::CPUI_STORE => 2..3,
            OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => 1..op.num_input(),
            OpCode::CPUI_CALLOTHER => return None,
            OpCode::CPUI_RETURN => 1..op.num_input(),
            _ => continue,
        };
        for slot in slots {
            let value = saved_stack_value(fd, op.get_in(slot)?, budget, active)?;
            let extent = if let Some(proto) = declaration {
                proto.get_param(slot - 1)?.get_type()?.get_ptr_to().map_or(1, |ty| ty.get_size().max(1))
            } else {
                fd.vbank().get(value)?.get_type().get_ptr_to().map_or(1, |ty| ty.get_size().max(1))
            };
            if may_point_to_slot(fd, value, pointer, size, extent, budget) {
                return None;
            }
        }
    }
    Some(())
}

/// Reuse a declared caller-frame contract only for a proved private slot.
fn call_preserves_slot(
    fd: &Funcdata,
    call: OpId,
    pointer: VarnodeId,
    size: int4,
    budget: &mut usize,
    active: &mut BTreeSet<OpId>,
) -> Option<()> {
    let fc = fd.get_call_specs(fd.get_call_specs_index(call)?);
    let proto = fc.proto();
    if proto.has_effect_override() {
        return None;
    }
    let mut floor = crate::kuna_calleeprotostack::declared_caller_frame_floor(true, proto)?;
    let stack = fd.get_arch().manage().get_stack_space()?;
    let call_op = fd.obank().get(call)?;
    if call_op.num_input() != proto.num_params() + 1 {
        return None;
    }
    let mut offset = None;
    for slot in 0..proto.num_params() {
        let param = proto.get_param(slot)?;
        let address = param.get_address();
        if address.get_space()?.get_index() != stack.get_index() {
            continue;
        }
        let at = sign_extend(address.get_offset() as i64, stack.get_addr_size() * 8 - 1);
        if at < 0 || param.get_size() <= 0 {
            return None;
        }
        floor = floor.max(int4::try_from(at.checked_add(param.get_size() as i64)?).ok()?);
        let value = strip(fd, call_op.get_in(slot + 1)?);
        let Some(load) = fd.vbank().get(value)?.get_def().and_then(|id| fd.obank().get(id)) else {
            continue;
        };
        if load.code() != OpCode::CPUI_LOAD {
            continue;
        }
        let here = relative_offset(fd, pointer, load.get_in(1)?)?.checked_add(at as i128)?;
        if offset.is_some_and(|previous| previous != here) {
            return None;
        }
        offset = Some(here);
    }
    if offset.is_none() {
        let ret = fd.get_arch().default_return_addr.as_ref()?;
        if ret.offset != 0
            || ret.space.as_ref()?.get_index() != stack.get_index()
            || ret.size != stack.get_addr_size() as u32
        {
            return None;
        }
        let parent = call_op.get_parent()?;
        let mut previous = call_op.basic_neighbours().0;
        while let Some(id) = previous {
            *budget = budget.checked_sub(1)?;
            let op = fd.obank().get(id)?;
            if op.get_parent() != Some(parent)
                || matches!(op.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND | OpCode::CPUI_CALLOTHER)
            {
                return None;
            }
            if let Some(sp) = op.get_out().filter(|&value| stack_register(fd, value)) {
                // A still-present CALL's own return-address adjustment is not
                // evidence of the pre-instruction stack pointer.
                if op.get_addr() == call_op.get_addr() {
                    return None;
                }
                offset = Some(relative_offset(fd, pointer, sp)?.checked_add(ret.size as i128)?);
                break;
            }
            previous = op.basic_neighbours().0;
        }
    }
    let offset = u64::try_from(offset?).ok()?;
    if !crate::kuna_calleeprotostack::above_callee_frame(offset, size, stack.get_addr_size(), floor) {
        return None;
    }
    private_saved_slot(fd, call, pointer, size, budget, active)
}

fn saved_stack_value(
    fd: &Funcdata,
    value: VarnodeId,
    budget: &mut usize,
    active: &mut BTreeSet<OpId>,
) -> Option<VarnodeId> {
    let value = strip(fd, value);
    let v = fd.vbank().get(value)?;
    let Some(id) = v.get_def().filter(|&id| fd.obank().get(id).is_some_and(|op| op.code() == OpCode::CPUI_LOAD)) else {
        return Some(value);
    };
    if !active.insert(id) {
        return None;
    }
    let result = (|| {
        let load = fd.obank().get(id)?;
        let pointer = load.get_in(1)?;
        let space = number(fd, load.get_in(0)?)?;
        let parent = load.get_parent()?;
        let size = v.get_size();
        let mut previous = load.basic_neighbours().0;
        while let Some(id) = previous {
            *budget = budget.checked_sub(1)?;
            let op = fd.obank().get(id)?;
            if op.get_parent() != Some(parent) {
                return None;
            }
            match op.code() {
                OpCode::CPUI_STORE => {
                    if number(fd, op.get_in(0)?)? == space {
                        let stored = op.get_in(2)?;
                        let width = fd.vbank().get(stored)?.get_size();
                        let delta = relative_offset(fd, op.get_in(1)?, pointer)?;
                        if delta == 0 && width == size {
                            return saved_stack_value(fd, stored, budget, active);
                        }
                        if delta < size as i128 && delta.checked_add(width as i128)? > 0 {
                            return None;
                        }
                    }
                }
                OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => {
                    call_preserves_slot(fd, id, pointer, size, budget, active)?;
                }
                OpCode::CPUI_CALLOTHER | OpCode::CPUI_BRANCH | OpCode::CPUI_CBRANCH
                | OpCode::CPUI_BRANCHIND | OpCode::CPUI_RETURN => return None,
                _ => (),
            }
            previous = op.basic_neighbours().0;
        }
        None
    })();
    active.remove(&id);
    result
}

fn recovered_base(fd: &Funcdata, mut base: VarnodeId, mut offset: i128) -> (VarnodeId, i128) {
    if fd.vbank().get(base).and_then(|v| v.get_def()).and_then(|id| fd.obank().get(id))
        .is_none_or(|op| op.code() != OpCode::CPUI_LOAD)
    {
        return (base, offset);
    }
    let mut budget = 1024;
    let mut active = BTreeSet::new();
    let mut seen = BTreeSet::new();
    while seen.insert(base) {
        let Some(value) = saved_stack_value(fd, base, &mut budget, &mut active) else {
            break;
        };
        if value == base {
            break;
        }
        let (next, delta) = canonical_base(fd, value, &mut BTreeSet::new());
        let Some(total) = offset.checked_add(delta) else {
            break;
        };
        base = next;
        offset = total;
    }
    (base, offset)
}
/// Displaced copies of a root allocation, as `(op, root output, displacement)`.
fn displaced_aliases(
    fd: &Funcdata,
    roots: &[Allocation],
    aliases: &[(VarnodeId, VarnodeId)],
) -> Vec<(OpId, VarnodeId, i128)> {
    fd.obank()
        .iter_alive()
        .filter_map(|op| {
            let candidate = allocation(fd, op)?;
            if roots.iter().any(|r| r.op == op) || aliases.iter().any(|a| a.0 == candidate.output) {
                return None;
            }
            roots.iter().find_map(|r| {
                let delta = origin_delta(fd, r, &candidate).filter(|&d| d != 0)?;
                dominates_op(fd, r.op, op).then_some((op, r.output, delta))
            })
        })
        .collect()
}
fn identify_aliases(
    fd: &Funcdata,
    mut found: Vec<Allocation>,
    stack_definitions: &[OpId],
) -> (Vec<Allocation>, Vec<(VarnodeId, VarnodeId)>) {
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
            if stack_definitions.binary_search(&id).is_err() {
                continue;
            }
            if let Some(candidate) = allocation(fd, id) {
                if fd.vbank().get(candidate.output).unwrap().get_size()
                    == fd.vbank().get(root.output).unwrap().get_size()
                    && origin_delta(fd, root, &candidate).is_some()
                {
                    *root = candidate;
                }
            }
        }
    }
    found.sort_unstable_by_key(|root| root.op);
    found.dedup_by_key(|root| root.op);
    let roots: Vec<_> = found
        .iter()
        .copied()
        .filter(|a| {
            !found
                .iter()
                .any(|b| origin_delta(fd, a, b).is_some() && dominates_op(fd, b.op, a.op))
        })
        .collect();
    let aliases = fd
        .obank()
        .iter_alive()
        .filter_map(|op| {
            let candidate = allocation(fd, op)?;
            let root = roots.iter().find(|r| {
                same_origin(fd, r, &candidate)
                    && r.output != candidate.output
                    && dominates_uses(fd, r.op, candidate.output)
            })?;
            Some((candidate.output, root.output))
        })
        .collect();
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
fn alignment(fd: &Funcdata, op: OpId) -> Option<(int4, u128)> {
    let o = fd.obank().get(op)?;
    if o.code() != OpCode::CPUI_INT_AND {
        return None;
    }
    (0..2).find_map(|slot| {
        let value = o.get_in(slot)?;
        let mask = literal(fd, value, &mut 32)?;
        if mask == 0 {
            return None;
        }
        let width = fd.vbank().get(value)?.get_size();
        let size = (!mask & calc_mask(width)) as u128 + 1;
        size.is_power_of_two().then_some((1 - slot, size))
    })
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

fn evaluate(fd: &Funcdata, id: OpId, values: &Values, upper: bool) -> Bounds {
    let o = fd.obank().get(id).unwrap();
    let input = |slot| {
        o.get_in(slot)
            .and_then(|v| values.get(&v))
            .cloned()
            .unwrap_or_default()
    };
    match o.code() {
        OpCode::CPUI_COPY | OpCode::CPUI_CAST => input(0),
        OpCode::CPUI_MULTIEQUAL => {
            let mut result = Bounds::new();
            for slot in 0..o.num_input() {
                merge(&mut result, input(slot), upper);
            }
            result
        }
        OpCode::CPUI_INT_ADD | OpCode::CPUI_INT_SUB | OpCode::CPUI_PTRSUB => {
            if let Some(by) = o.get_in(1).and_then(|v| number(fd, v)) {
                shift(
                    input(0),
                    Some(if o.code() == OpCode::CPUI_INT_SUB {
                        -by
                    } else {
                        by
                    }),
                )
            } else if o.code() == OpCode::CPUI_INT_ADD {
                if let Some(by) = o.get_in(0).and_then(|v| number(fd, v)) {
                    return shift(input(1), Some(by));
                }
                let mut result = shift(input(0), None);
                merge(&mut result, shift(input(1), None), upper);
                result
            } else {
                shift(input(0), None)
            }
        }
        OpCode::CPUI_PTRADD => shift(
            input(0),
            o.get_in(1)
                .and_then(|v| number(fd, v))
                .zip(o.get_in(2).and_then(|v| number(fd, v)))
                .and_then(|(a, b)| a.checked_mul(b)),
        ),
        OpCode::CPUI_INT_AND => {
            if let Some((slot, amount)) = alignment(fd, id) {
                shift(
                    input(slot),
                    if upper {
                        Some(0)
                    } else {
                        i128::try_from(amount - 1).ok().map(|a| -a)
                    },
                )
            } else {
                let mut result = shift(input(0), None);
                merge(&mut result, shift(input(1), None), upper);
                result
            }
        }
        OpCode::CPUI_INDIRECT => {
            shift(input(0), indirect_stack_pop(fd, id))
        }
        _ => Bounds::new(),
    }
}
fn propagate_bounds(
    fd: &Funcdata,
    ops: &[OpId],
    roots: &BTreeMap<VarnodeId, usize>,
    displaced: &BTreeMap<OpId, (VarnodeId, i128)>,
    upper: bool,
) -> Option<Values> {
    let mut values = Values::new();
    let update = |values: &mut Values| {
        let mut changed = BTreeSet::new();
        for &id in ops {
            let o = fd.obank().get(id).unwrap();
            let Some(output) = o.get_out() else { continue };
            let next = if let Some(&root) = roots.get(&output) {
                Bounds::from([(root, 0)])
            } else if let Some(&(base, offset)) = displaced.get(&id) {
                shift(values.get(&base).cloned().unwrap_or_default(), Some(offset))
            } else {
                evaluate(fd, id, values, upper)
            };
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
        .map(|(lo, hi)| {
            let before = ((bias.checked_sub(lo)? as u128).checked_add(align - 1)?) & !(align - 1);
            let after =
                ((hi.checked_sub(bias)?.max(0) as u128).checked_add(align - 1)?) & !(align - 1);
            let count = before.checked_add(after)?;
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
fn at_or_below(
    fd: &Funcdata,
    value: VarnodeId,
    anchor: VarnodeId,
    ceiling: i128,
    reservation: Option<&Allocation>,
    after: Option<&BTreeSet<BlockId>>,
    budget: &mut usize,
) -> bool {
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
        if indices.contains_key(&value) {
            continue;
        }
        let Some(left) = budget.checked_sub(1) else {
            return false;
        };
        *budget = left;
        if nodes.len() == 256 || fd.vbank().get(value).is_none_or(|v| v.get_size() != width) {
            return false;
        }
        let step = if value == anchor {
            Step::Seed(0)
        } else {
            let Some(id) = fd.vbank().get(value).and_then(|v| v.get_def()) else {
                return false;
            };
            let op = fd.obank().get(id).unwrap();
            let candidate = allocation(fd, id);
            let displacement = reservation.and_then(|root| {
                candidate.and_then(|candidate| {
                    dominates_op(fd, root.op, id)
                        .then(|| origin_delta(fd, root, &candidate))
                        .flatten()
                })
            });
            if let Some(delta) = displacement {
                Step::Seed(delta)
            } else if let Some((input, delta)) = constant_step(fd, value) {
                Step::Offset(input, delta)
            } else {
                match op.code() {
                    OpCode::CPUI_COPY | OpCode::CPUI_CAST => {
                        let Some(input) = op.get_in(0) else {
                            return false;
                        };
                        Step::Offset(input, 0)
                    }
                    OpCode::CPUI_INDIRECT => {
                        let Some(input) = op.get_in(0) else {
                            return false;
                        };
                        let Some(pop) = indirect_stack_pop(fd, id) else {
                            return false;
                        };
                        Step::Offset(input, pop)
                    }
                    OpCode::CPUI_MULTIEQUAL => {
                        if after.is_some()
                            && op.get_parent().is_none_or(|parent| {
                                fd.bblocks_ref().block(parent).size_in() != op.num_input()
                            })
                        {
                            return false;
                        }
                        Step::Phi(id)
                    }
                    OpCode::CPUI_INT_AND => {
                        let Some(input) =
                            alignment(fd, id).and_then(|(slot, _)| op.get_in(slot))
                        else {
                            return false;
                        };
                        Step::Offset(input, 0)
                    }
                    _ => {
                        let Some(candidate) = candidate else {
                            return false;
                        };
                        // SUB by a negated size restores SP upwards.
                        if negate(fd, candidate.amount).is_some() {
                            return false;
                        }
                        Step::Offset(candidate.base, 0)
                    }
                }
            }
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
                        return false;
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
                return false;
            };
            *budget = left;
            let upper = match nodes[index].step {
                Step::Seed(delta) => Some(delta),
                Step::Offset(input, delta) => match nodes[indices[&input]].upper {
                    Some(value) => {
                        let Some(value) = value.checked_add(delta) else {
                            return false;
                        };
                        Some(value)
                    }
                    None => None,
                },
                Step::Phi(id) => {
                    let op = fd.obank().get(id).unwrap();
                    (0..op.num_input())
                        .filter(|&slot| live_input(id, slot))
                        .filter_map(|slot| nodes[indices[&op.get_in(slot).unwrap()]].upper)
                        .max()
                }
            };
            if nodes[index].upper != upper {
                nodes[index].upper = upper;
                changed = true;
            }
        }
        if !changed {
            return nodes.iter().all(|node| node.upper.is_some())
                && nodes[indices[&value]].upper.is_some_and(|upper| upper <= ceiling);
        }
    }
    false
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
                    later.base,
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
                at_or_below(fd, allocation.base, header_sp, 0, None, None, &mut proof_budget)
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

fn lower(fd: &mut Funcdata, allocation: Allocation, width: int4, headroom: Reservation) {
    let op = allocation.op;
    let address = fd.obank().get(op).unwrap().get_addr().clone();
    let amount = if fd.vbank().get(allocation.amount).unwrap().is_free() {
        fd.clone_varnode(allocation.amount)
    } else {
        allocation.amount
    };
    let count_op = fd.new_op(2, address.clone());
    fd.op_set_opcode_code(count_op, OpCode::CPUI_INT_ADD);
    fd.op_set_input(count_op, amount, 0).unwrap();
    let reserve = fd.new_constant(width, headroom.count);
    fd.op_set_input(count_op, reserve, 1).unwrap();
    let count = fd.new_unique_out(width, count_op).unwrap();
    fd.op_insert_before(count_op, op);
    let allocate = fd.new_op(2, address);
    fd.op_set_opcode_code(allocate, OpCode::CPUI_CALLOTHER);
    let builtin = fd.new_constant(4, BUILTIN_STACK_ALLOC as u64);
    fd.op_set_input(allocate, builtin, 0).unwrap();
    fd.op_set_input(allocate, count, 1).unwrap();
    let buffer = fd.new_unique_out(width, allocate).unwrap();
    fd.op_insert_before(allocate, op);
    while fd.obank().get(op).unwrap().num_input() > 2 {
        fd.op_remove_input(op, 2);
    }
    fd.op_set_opcode_code(op, OpCode::CPUI_INT_ADD);
    fd.op_set_input(op, buffer, 0).unwrap();
    let reserve = fd.new_constant(width, headroom.shift);
    fd.op_set_input(op, reserve, 1).unwrap();
}
/// A declined reservation must not leave aliases fighting arithmetic cleanup.
fn materialize_allocations(
    fd: &mut Funcdata,
    allocations: &[Allocation],
    aliases: &[(VarnodeId, VarnodeId)],
    width: int4,
) -> bool {
    let displaced = displaced_aliases(fd, allocations, aliases);
    let Some(headroom) = reserves(fd, allocations, aliases, &displaced, width) else {
        return false;
    };
    for (op, root, delta) in displaced {
        let size = fd
            .vbank()
            .get(fd.obank().get(op).unwrap().get_out().unwrap())
            .unwrap()
            .get_size();
        while fd.obank().get(op).unwrap().num_input() > 2 {
            fd.op_remove_input(op, 2);
        }
        fd.op_set_opcode_code(op, OpCode::CPUI_INT_ADD);
        fd.op_set_input(op, root, 0).unwrap();
        let displacement = fd.new_constant(size, delta as u64 & calc_mask(size));
        fd.op_set_input(op, displacement, 1).unwrap();
    }
    for &(alias, original) in aliases {
        fd.total_replace(alias, original).unwrap();
    }
    for (&allocation, reserve) in allocations.iter().zip(headroom) {
        lower(fd, allocation, width, reserve);
    }
    true
}

pub struct ActionStackAlloc {
    base: ActionBase,
    // Retain provenance, not a stale expression, while call trials converge.
    stack_definitions: Vec<OpId>,
    // Failed proofs must not disappear merely because arithmetic is between
    // normal forms on the next heritage pass.
    deferred_outputs: Vec<VarnodeId>,
}
impl ActionStackAlloc {
    pub fn boxed() -> Box<dyn Action> {
        Box::new(Self {
            base: ActionBase::new(0, "stackalloc", "stackvars"),
            stack_definitions: Vec::new(),
            deferred_outputs: Vec::new(),
        })
    }
}
impl Action for ActionStackAlloc {
    fn base(&self) -> &ActionBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ActionBase {
        &mut self.base
    }
    fn clone_filtered(&self, groups: &ActionGroupList) -> Option<Box<dyn Action>> {
        groups.contains(self.get_group()).then(Self::boxed)
    }
    fn reset(&mut self, _fd: &mut Funcdata) {
        self.base.reset_base();
        self.stack_definitions.clear();
        self.deferred_outputs.clear();
    }
    fn apply(&mut self, fd: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        if !fd.get_arch().stack_alloc || fd.get_heritage_pass() == 0 {
            return 0;
        }
        let Some(stack) = fd.get_arch().manage().get_stack_space() else {
            return 0;
        };
        let Ok(register) = stack.get_spacebase(0) else {
            return 0;
        };
        let width = register.size as int4;
        if !matches!(width, 4 | 8) {
            return 0;
        }
        // Active parameter recovery takes multiple heritage passes. Merely
        // running after its action does not mean its stack trials are final.
        if (0..fd.num_calls()).any(|idx| {
            let call = fd.get_call_specs(idx);
            call.is_input_active()
                && fd.obank().get(call.get_op()).is_some_and(|op| {
                    !op.is_dead()
                        && matches!(op.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND)
                })
        }) {
            let mut seen = BTreeSet::new();
            for id in fd.obank().iter_alive() {
                let Some(out) = fd
                    .obank()
                    .get(id)
                    .and_then(|op| op.get_out())
                    .filter(|&out| stack_register(fd, out))
                else {
                    continue;
                };
                seen.clear();
                candidates(fd, out, &mut seen, &mut |candidate| {
                    if reservation_point(fd, candidate.op, id) {
                        self.stack_definitions.push(candidate.op);
                    }
                });
            }
            self.stack_definitions.sort_unstable();
            self.stack_definitions.dedup();
            return 0;
        }
        let mut found = BTreeMap::new();
        let mut seen = BTreeSet::new();
        for id in fd.obank().iter_alive() {
            if let Some(out) = fd
                .obank()
                .get(id)
                .and_then(|o| o.get_out())
                .filter(|&v| stack_register(fd, v))
            {
                candidates(fd, out, &mut seen, &mut |candidate| {
                    found.insert(candidate.op, candidate);
                });
            }
        }
        let complete = self.deferred_outputs.iter().all(|&output| {
            fd.vbank().get(output).is_none_or(|value| {
                value.descend_iter().next().is_none()
                    || found.values().any(|candidate| candidate.output == output)
            })
        });
        self.deferred_outputs.extend(found.values().map(|candidate| candidate.output));
        self.deferred_outputs.sort_unstable();
        self.deferred_outputs.dedup();
        let (allocations, aliases) =
            identify_aliases(fd, found.into_values().collect(), &self.stack_definitions);
        if !complete
            || allocations.is_empty()
            || allocations.iter().any(|a| loop_restores_stack(fd, a))
            || !sequential_reservations_survive(fd, &allocations)
        {
            return 0;
        }
        if materialize_allocations(fd, &allocations, &aliases, width) {
            self.base.count += allocations.len() as int4;
            self.deferred_outputs.clear();
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::ArchContext;
    use kuna_base::address::Address;
    use kuna_base::space::{
        addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, FspecSpace,
        IopSpace, UniqueSpace,
    };
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
        m
    }

    fn build_fd() -> Funcdata {
        let manage = build_manager();
        let glb = Rc::new(ArchContext::new(manage));
        let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
        let addr = Address::new(ram, 0x1000);
        Funcdata::new("func", "func", glb, addr, 0x10000000, 0x40).unwrap()
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
            assert_eq!(negate(&fd, restored), Some(amount));
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
            &[Allocation {
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

            fd.get_call_specs_mut(index).proto_mut().set_extra_pop(width);
            fd.get_call_specs_mut(index).proto_mut().set_input_lock(true);
            let slot = fd.new_op(2, fd.get_address().clone());
            fd.op_set_opcode_code(slot, OpCode::CPUI_INT_ADD);
            fd.op_set_input(slot, saved_sp, 0).unwrap();
            let distance = fd.new_constant(width, (-2 * width as i64) as u64 & calc_mask(width));
            fd.op_set_input(slot, distance, 1).unwrap();
            let private = fd.new_unique_out(width, slot).unwrap();
            fd.op_insert_end(slot, blocks[3]);
            let call = fd.new_op(1, fd.get_address().clone());
            fd.op_set_opcode_code(call, OpCode::CPUI_CALL);
            let target = fd.new_constant(width, 0x4000);
            fd.op_set_input(call, target, 0).unwrap();
            fd.op_insert_end(call, blocks[7]);
            let mut spec = crate::fspec::FuncCallSpecs::new(call, Address::new_invalid());
            spec.proto_mut().set_extra_pop(width);
            spec.proto_mut().set_input_lock(true);
            fd.push_call_specs(spec);
            assert!(private_saved_slot(&fd, call, private, width, &mut 4096, &mut BTreeSet::new()).is_some());

            let escape = fd.new_op(3, fd.get_address().clone());
            fd.op_set_opcode_code(escape, OpCode::CPUI_STORE);
            let memory = fd.get_arch().manage().get_space_by_name("ram").unwrap().get_index();
            let space = fd.new_constant(4, memory as u64);
            let global = fd.new_constant(width, 0x5000);
            fd.op_set_input(escape, space, 0).unwrap();
            fd.op_set_input(escape, global, 1).unwrap();
            fd.op_set_input(escape, private, 2).unwrap();
            fd.op_insert_before(escape, call);
            assert!(
                private_saved_slot(&fd, call, private, width, &mut 4096, &mut BTreeSet::new()).is_none(),
                "a prior global address escape invalidates a no-argument call's private-slot proof"
            );
        }
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
    fn declined_reservation_does_not_rewrite_displaced_aliases() {
        let mut fd = build_fd();
        let ram = Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap());
        let base = fd.new_varnode(4, &Address::new(Rc::clone(&ram), 0x2000), None);
        let base = fd.set_input_varnode(base).unwrap();
        let amount = fd.new_varnode(4, &Address::new(ram, 0x2004), None);
        let amount = fd.set_input_varnode(amount).unwrap();
        let root = fd.bblocks_root_pub();
        let block = fd.bblocks_mut().new_block_basic(root);
        let op = fd.new_op(2, fd.get_address().clone());
        fd.op_set_opcode_code(op, OpCode::CPUI_INT_SUB);
        fd.op_set_input(op, base, 0).unwrap();
        fd.op_set_input(op, amount, 1).unwrap();
        let output = fd.new_unique_out(4, op).unwrap();
        fd.op_insert_end(op, block);
        let allocation = Allocation {
            op,
            output,
            amount,
            base,
        };
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
        assert!(!materialize_allocations(&mut fd, &[allocation], &[], 4));
        for (alias, shifted_base) in aliases {
            let op = fd.obank().get(alias).unwrap();
            assert_eq!(op.code(), OpCode::CPUI_INT_SUB);
            assert_eq!(op.get_in(0), Some(shifted_base));
            assert_eq!(op.get_in(1), Some(amount));
        }
    }
}
