//! Recover runtime stack allocation and bound its outgoing stack area over SSA.

use crate::action::{Action, ActionBase, ActionContext, ActionGroupList, ApplyResult};
use crate::context::{OpId, VarnodeId};
use crate::funcdata::Funcdata;
use kuna_base::address::{calc_mask, sign_extend};
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;
use std::collections::{BTreeMap, BTreeSet};

/// Internal allocation result is an address-sized integer, including in C.
pub const BUILTIN_STACK_ALLOC: u32 = 0x10000009;
const UNKNOWN: i128 = i128::MIN;
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
    result: &mut BTreeMap<OpId, Allocation>,
) {
    if !seen.insert(value) {
        return;
    }
    let Some(id) = fd.vbank().get(value).and_then(|v| v.get_def()) else {
        return;
    };
    if let Some(found) = allocation(fd, id) {
        result.insert(id, found);
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
    canonical_base(fd, a.base, &mut BTreeSet::new())
        == canonical_base(fd, b.base, &mut BTreeSet::new())
        && equivalent(fd, a.amount, b.amount, &mut BTreeSet::new())
}
fn identify_aliases(
    fd: &Funcdata,
    found: Vec<Allocation>,
) -> (Vec<Allocation>, Vec<(VarnodeId, VarnodeId)>) {
    let roots: Vec<_> = found
        .iter()
        .copied()
        .filter(|a| {
            !found
                .iter()
                .any(|b| same_origin(fd, a, b) && dominates_op(fd, b.op, a.op))
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
    let mut stable = false;
    let mut changed = BTreeSet::new();
    for _ in 0..=ops.len() {
        changed = update(&mut values);
        if changed.is_empty() {
            stable = true;
            break;
        }
    }
    // A pointer induction cycle need not change the outgoing stack area. Widen
    // unresolved cyclic bounds to unknown and propagate that fact. An unknown
    // bound reaching a physical stack-pointer write still declines below.
    if !stable {
        for value in changed {
            for offset in values
                .get_mut(&value)
                .into_iter()
                .flat_map(|b| b.values_mut())
            {
                *offset = UNKNOWN;
            }
        }
        for _ in 0..=ops.len() {
            if update(&mut values).is_empty() {
                stable = true;
                break;
            }
        }
        if !stable {
            return None;
        }
    }
    Some(values)
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
    let values = propagate_bounds(fd, &ops, &roots, false)?;
    let upper = propagate_bounds(fd, &ops, &roots, true)?;
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
        if let Some((_, a)) = alignment(fd, id) {
            align = align.max(a);
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
pub struct ActionStackAlloc {
    base: ActionBase,
}
impl ActionStackAlloc {
    pub fn boxed() -> Box<dyn Action> {
        Box::new(Self {
            base: ActionBase::new(0, "stackalloc", "stackvars"),
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
        let mut found = BTreeMap::new();
        let mut seen = BTreeSet::new();
        for id in fd.obank().iter_alive() {
            if let Some(out) = fd
                .obank()
                .get(id)
                .and_then(|o| o.get_out())
                .filter(|&v| stack_register(fd, v))
            {
                candidates(fd, out, &mut seen, &mut found);
            }
        }
        let (allocations, aliases) = identify_aliases(fd, found.into_values().collect());
        if allocations.is_empty() {
            return 0;
        }
        let Some(headroom) = reserves(fd, &allocations, &aliases, width) else {
            return 0;
        };
        for (alias, original) in aliases {
            fd.total_replace(alias, original).unwrap();
        }
        self.base.count += allocations.len() as int4;
        for (allocation, reserve) in allocations.into_iter().zip(headroom) {
            lower(fd, allocation, width, reserve);
        }
        0
    }
}
