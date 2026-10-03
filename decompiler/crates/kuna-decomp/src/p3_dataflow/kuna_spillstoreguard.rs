//! Keep reads across spilled stack pointers until their constant destinations resolve.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::context::{OpId, VarnodeId};
use crate::funcdata::Funcdata;
use crate::op::pcodeop_flags;

pub(super) fn build(
    fd: &mut Funcdata,
    addr: &Address,
    size: int4,
    stores: &[OpId],
    existing: &[OpId],
    bounds: &std::collections::HashMap<OpId, (u64, int4)>,
    guards: &mut Vec<(OpId, OpId)>,
    writes: &mut Vec<VarnodeId>,
) {
    if stores.is_empty() {
        return;
    }
    let Some(stack) = fd.get_arch().manage().get_stack_space().cloned() else {
        return;
    };
    if stack.get_word_size() != 1 {
        return;
    }
    if addr
        .get_space()
        .is_none_or(|space| !Rc::ptr_eq(space, &stack))
    {
        return;
    }
    for &store in stores {
        let space = fd
            .obank()
            .get(store)
            .and_then(|op| op.get_in(0))
            .and_then(|vn| fd.vbank().get(vn))
            .map(|vn| vn.get_offset())
            .filter(|&index| index < fd.get_arch().manage().num_spaces() as u64)
            .and_then(|index| fd.get_arch().manage().get_space(index as int4));
        if space.is_none_or(|space| {
            !Rc::ptr_eq(space, &stack)
                && stack
                    .get_contain()
                    .is_none_or(|container| !Rc::ptr_eq(container, space))
        }) {
            continue;
        }
        if fd
            .obank()
            .get(store)
            .is_none_or(|op| op.is_dead() || op.code() != OpCode::CPUI_STORE)
        {
            continue;
        }
        if existing.contains(&store) {
            continue;
        }
        if let Some(&(start, written_size)) = bounds.get(&store) {
            if !overlaps(&stack, start, written_size, addr.get_offset(), size) {
                continue;
            }
        }
        let indirect = fd.new_indirect_op(store, addr, size, pcodeop_flags::indirect_store);
        let op = fd.obank().get(indirect).expect("spill store guard");
        let input = op.get_in(0).expect("spill store input");
        let output = op.get_out().expect("spill store output");
        fd.vbank_mut()
            .get_mut(input)
            .expect("spill store input")
            .set_active_heritage();
        fd.vbank_mut()
            .get_mut(output)
            .expect("spill store output")
            .set_active_heritage();
        writes.push(output);
        guards.push((indirect, store));
    }
}

/// Limit guard construction only in a closed frame with immutable pointer slots.
pub(super) fn bounds(
    fd: &Funcdata,
    stores: &[OpId],
) -> std::collections::HashMap<OpId, (u64, int4)> {
    use std::collections::HashMap;
    let empty = HashMap::new();
    if stores.len() < 16
        || fd.obank().iter_alive().any(|id| {
            fd.obank().get(id).is_some_and(|op| {
                matches!(
                    op.code(),
                    OpCode::CPUI_CALL | OpCode::CPUI_CALLIND | OpCode::CPUI_CALLOTHER
                )
            })
        })
    {
        return empty;
    }
    let Some(stack) = fd.get_arch().manage().get_stack_space() else {
        return empty;
    };
    if stack.get_word_size() != 1 {
        return empty;
    }
    let mut slots = HashMap::new();
    let mut targets = HashMap::new();
    for id in fd.obank().iter_code(OpCode::CPUI_STORE) {
        let Some(op) = fd.obank().get(id).filter(|op| !op.is_dead()) else {
            continue;
        };
        let Some(pointer) = op.get_in(1) else {
            return empty;
        };
        let Some(start) = preliminary_offset(fd, pointer, stack, &mut slots, 64) else {
            return empty;
        };
        let Some(size) = op
            .get_in(2)
            .and_then(|vn| fd.vbank().get(vn))
            .map(|v| v.get_size())
        else {
            return empty;
        };
        if size <= 0 || stack.wrap_offset(start.wrapping_add(size as u64 - 1)) < start {
            return empty;
        }
        targets.insert(id, (start, size));
    }
    if targets.values().any(|&(start, size)| {
        slots
            .keys()
            .any(|&(slot, width)| overlaps(stack, start, size, slot, width))
    }) {
        return empty;
    }
    if fd
        .vbank()
        .iter_loc()
        .filter_map(|id| fd.vbank().get(id))
        .any(|value| {
            Rc::ptr_eq(value.get_space(), stack)
                && (value.is_written() || value.is_input())
                && slots.keys().any(|&(slot, width)| {
                    overlaps(stack, value.get_offset(), value.get_size(), slot, width)
                        && (value.get_offset() != slot || value.get_size() != width)
                })
        })
    {
        return empty;
    }
    targets.retain(|id, _| stores.contains(id));
    targets
}

/// Admit the whole provisional guard set or leave the existing policy in charge.
pub(super) fn within_budget(
    fd: &Funcdata,
    stores: &[OpId],
    bounds: &std::collections::HashMap<OpId, (u64, int4)>,
) -> bool {
    if stores.is_empty() || bounds.len() == stores.len() {
        return true;
    }
    let Some(stack) = fd.get_arch().manage().get_stack_space() else {
        return false;
    };
    let mut ranges = std::collections::HashSet::new();
    for id in fd.vbank().iter_loc() {
        let Some(value) = fd
            .vbank()
            .get(id)
            .filter(|value| Rc::ptr_eq(value.get_space(), stack))
        else {
            continue;
        };
        ranges.insert((value.get_offset(), value.get_size()));
        if ranges.len().saturating_mul(stores.len()) > 4096 {
            return false;
        }
    }
    true
}

fn preliminary_offset(
    fd: &Funcdata,
    vn: VarnodeId,
    stack: &Rc<kuna_base::space::AddrSpace>,
    slots: &mut std::collections::HashMap<(u64, int4), Option<u64>>,
    depth: usize,
) -> Option<u64> {
    if depth == 0 {
        return None;
    }
    let value = fd.vbank().get(vn)?;
    if value.get_size() != stack.get_addr_size() as int4 {
        return None;
    }
    if value.is_free() && Rc::ptr_eq(value.get_space(), stack) {
        let key = (value.get_offset(), value.get_size());
        if let Some(result) = slots.get(&key) {
            return *result;
        }
        slots.insert(key, None);
        let mut result = None;
        for writer in fd
            .vbank()
            .iter_loc_size_addr(value.get_size(), value.get_addr())
        {
            let candidate = fd.vbank().get(writer)?;
            if candidate.is_input() {
                return None;
            }
            if !candidate.is_written() {
                continue;
            }
            let current = preliminary_offset(fd, writer, stack, slots, depth - 1)?;
            if result.is_some_and(|old| old != current) {
                return None;
            }
            result = Some(current);
        }
        slots.insert(key, result);
        return result;
    }
    if value.is_input() && value.is_spacebase() {
        return offset(fd, vn, stack, depth);
    }
    let op = fd.obank().get(value.get_def()?)?;
    match op.code() {
        OpCode::CPUI_COPY => preliminary_offset(fd, op.get_in(0)?, stack, slots, depth - 1),
        OpCode::CPUI_INT_ADD => {
            for slot in 0..2 {
                let constant = fd.vbank().get(op.get_in(slot)?)?;
                if constant.is_constant() {
                    let base =
                        preliminary_offset(fd, op.get_in(1 - slot)?, stack, slots, depth - 1)?;
                    return Some(stack.wrap_offset(base.wrapping_add(constant.get_offset())));
                }
            }
            None
        }
        _ => None,
    }
}

fn overlaps(
    stack: &kuna_base::space::AddrSpace,
    a: u64,
    a_size: int4,
    b: u64,
    b_size: int4,
) -> bool {
    stack.wrap_offset(a.wrapping_sub(b)) < b_size as u64
        || stack.wrap_offset(b.wrapping_sub(a)) < a_size as u64
}

pub(super) fn prune(fd: &mut Funcdata, guards: &[(OpId, OpId)]) {
    if guards.is_empty() {
        return;
    }
    let Some(stack) = fd.get_arch().manage().get_stack_space().cloned() else {
        return;
    };
    let mut stores = Vec::new();
    let mut grouped = std::collections::HashMap::<OpId, Vec<OpId>>::new();
    for &(indirect, store) in guards {
        let group = grouped.entry(store).or_insert_with(|| {
            stores.push(store);
            Vec::new()
        });
        group.push(indirect);
    }
    let mut resolved = std::collections::HashMap::new();
    loop {
        let mut progress = false;
        for &store in &stores {
            if resolved.contains_key(&store) {
                continue;
            }
            let Some(op) = fd
                .obank()
                .get(store)
                .filter(|op| op.code() == OpCode::CPUI_STORE)
            else {
                continue;
            };
            let Some(pointer) = op.get_in(1) else {
                continue;
            };
            let Some(start) = offset(fd, pointer, &stack, 64) else {
                continue;
            };
            let Some(size) = op
                .get_in(2)
                .and_then(|vn| fd.vbank().get(vn))
                .map(|vn| vn.get_size())
            else {
                continue;
            };
            if size <= 0 || stack.wrap_offset(start.wrapping_add(size as u64 - 1)) < start {
                continue;
            }
            resolved.insert(store, (start, size));
            progress = true;
            for &indirect in &grouped[&store] {
                let Some(out) = fd
                    .obank()
                    .get(indirect)
                    .and_then(|op| op.get_out())
                    .and_then(|vn| fd.vbank().get(vn))
                else {
                    continue;
                };
                if !overlaps(&stack, start, size, out.get_offset(), out.get_size()) {
                    remove(fd, indirect);
                }
            }
        }
        if !progress {
            break;
        }
    }
    for &(indirect, store) in guards {
        if !resolved.contains_key(&store) {
            remove(fd, indirect);
        }
    }
}

fn remove(fd: &mut Funcdata, indirect: OpId) {
    let Some(op) = fd.obank().get(indirect) else {
        return;
    };
    if op.is_dead() || op.code() != OpCode::CPUI_INDIRECT {
        return;
    }
    let input = op.get_in(0).expect("spill store input");
    let output = op.get_out().expect("spill store output");
    fd.total_replace(output, input)
        .expect("remove spill store guard");
    fd.op_destroy(indirect);
}

fn offset(
    fd: &Funcdata,
    vn: VarnodeId,
    stack: &Rc<kuna_base::space::AddrSpace>,
    depth: usize,
) -> Option<u64> {
    if depth == 0 {
        return None;
    }
    let value = fd.vbank().get(vn)?;
    if stack.get_word_size() != 1 || value.get_size() != stack.get_addr_size() as int4 {
        return None;
    }
    if value.is_input() && value.is_spacebase() {
        let space = fd
            .get_arch()
            .manage()
            .get_space_by_spacebase(value.get_addr(), value.get_size())?;
        return Rc::ptr_eq(&space, stack).then_some(0);
    }
    let op = fd.obank().get(value.get_def()?)?;
    match op.code() {
        OpCode::CPUI_COPY => offset(fd, op.get_in(0)?, stack, depth - 1),
        OpCode::CPUI_INT_ADD => {
            for slot in 0..2 {
                let constant = fd.vbank().get(op.get_in(slot)?)?;
                if constant.is_constant() {
                    let base = offset(fd, op.get_in(1 - slot)?, stack, depth - 1)?;
                    return Some(stack.wrap_offset(base.wrapping_add(constant.get_offset())));
                }
            }
            None
        }
        _ => None,
    }
}

#[cfg(test)]
#[path = "kuna_spillstoreguard/tests.rs"]
mod tests;
