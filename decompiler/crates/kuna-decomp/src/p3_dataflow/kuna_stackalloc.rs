//! Recover runtime stack allocation and bound its outgoing stack area over SSA.

use crate::action::{Action, ActionBase, ActionContext, ActionGroupList, ApplyResult};
use crate::context::{OpId, VarnodeId};
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
    let (base_a, offset_a) = canonical_base(fd, a.base, &mut BTreeSet::new());
    let (base_b, offset_b) = canonical_base(fd, b.base, &mut BTreeSet::new());
    if base_a != base_b || !equivalent(fd, a.amount, b.amount, &mut BTreeSet::new()) {
        return None;
    }
    offset_b.checked_sub(offset_a)
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
            let pop = o.get_in(1).and_then(|v| fd.vbank().get(v)).and_then(|v| {
                let call = crate::funcdata_varnode::op_iop_decode(v.get_offset());
                let fc = fd.get_call_specs_index(call)?;
                let pop = fd.get_call_specs(fc).get_extra_pop();
                (pop != crate::fspec::EXTRAPOP_UNKNOWN).then_some(pop as i128)
            });
            if o.get_out().is_some_and(|v| stack_register(fd, v)) {
                shift(input(0), pop)
            } else {
                shift(input(0), None)
            }
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

/// A compiler alloca survives until return. A loop that restores an older SP
/// requires a shorter lifetime, which this lowering does not emit.
fn loop_restores_stack(fd: &Funcdata, allocation: &Allocation) -> bool {
    fn derives(
        fd: &Funcdata,
        value: VarnodeId,
        root: &Allocation,
        seen: &mut BTreeSet<VarnodeId>,
        budget: &mut usize,
    ) -> bool {
        if *budget == 0 {
            return false;
        }
        *budget -= 1;
        if value == root.output {
            return true;
        }
        if !seen.insert(value) || seen.len() > 256 {
            return false;
        }
        let Some(id) = fd.vbank().get(value).and_then(|v| v.get_def()) else {
            return false;
        };
        let Some(op) = fd.obank().get(id) else {
            return false;
        };
        // The carried SP may be a separately computed, displaced copy of the
        // same allocation after recovery. Restoring its pre-allocation base
        // does not satisfy this affine-origin proof.
        if fd.vbank().get(value).unwrap().get_size()
            == fd.vbank().get(root.output).unwrap().get_size()
            && self::allocation(fd, id)
                .is_some_and(|candidate| origin_delta(fd, root, &candidate).is_some())
        {
            return true;
        }
        match op.code() {
            OpCode::CPUI_COPY
            | OpCode::CPUI_CAST
            | OpCode::CPUI_INDIRECT
            | OpCode::CPUI_INT_SUB
            | OpCode::CPUI_PTRADD
            | OpCode::CPUI_PTRSUB => op
                .get_in(0)
                .is_some_and(|v| derives(fd, v, root, seen, budget)),
            OpCode::CPUI_INT_ADD => (0..2).any(|slot| {
                op.get_in(slot)
                    .is_some_and(|v| derives(fd, v, root, &mut seen.clone(), budget))
            }),
            OpCode::CPUI_INT_AND => {
                alignment(fd, fd.vbank().get(value).unwrap().get_def().unwrap())
                    .and_then(|(slot, _)| op.get_in(slot))
                    .is_some_and(|v| derives(fd, v, root, seen, budget))
            }
            OpCode::CPUI_MULTIEQUAL => (0..op.num_input()).all(|slot| {
                op.get_in(slot)
                    .is_some_and(|v| derives(fd, v, root, &mut seen.clone(), budget))
            }),
            _ => false,
        }
    }
    let Some(block) = fd.obank().get(allocation.op).and_then(|o| o.get_parent()) else {
        return true;
    };
    let graph = fd.bblocks_ref();
    let mut reachable = BTreeSet::new();
    let mut todo = vec![block];
    let mut budget = 65536usize;
    while let Some(b) = todo.pop() {
        if !reachable.insert(b) {
            continue;
        }
        let b = graph.block(b);
        for slot in 0..b.size_out() {
            if budget == 0 {
                return true;
            }
            budget -= 1;
            todo.push(b.get_out(slot));
        }
    }
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
                let o = fd.obank().get(id).unwrap();
                o.code() == OpCode::CPUI_MULTIEQUAL
                    && o.get_out().is_some_and(|v| stack_register(fd, v))
                    && o.get_in(incoming).is_some_and(|v| {
                        derives(
                            fd,
                            v,
                            allocation,
                            &mut BTreeSet::new(),
                            &mut proof_budget,
                        )
                    })
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
}
impl ActionStackAlloc {
    pub fn boxed() -> Box<dyn Action> {
        Box::new(Self {
            base: ActionBase::new(0, "stackalloc", "stackvars"),
            stack_definitions: Vec::new(),
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
        let (allocations, aliases) =
            identify_aliases(fd, found.into_values().collect(), &self.stack_definitions);
        if allocations.is_empty() || allocations.iter().any(|a| loop_restores_stack(fd, a)) {
            return 0;
        }
        if materialize_allocations(fd, &allocations, &aliases, width) {
            self.base.count += allocations.len() as int4;
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
