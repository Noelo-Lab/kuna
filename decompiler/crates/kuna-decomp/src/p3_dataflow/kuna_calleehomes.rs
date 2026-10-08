//! Conservative proof for pointer spills retained in a model-declared caller home gap.

use std::collections::BTreeSet;
use std::rc::Rc;

use kuna_base::address::calc_mask;
use kuna_base::space::{spacetype, AddrSpace};
use kuna_num::opcodes::OpCode;

use crate::context::{OpId, VarnodeId};
use crate::fspec::{FuncCallSpecs, OFFSET_UNKNOWN};
use crate::funcdata::Funcdata;

const HOME_START: i64 = 8;
const HOME_END: i64 = 40;
/// Maximum p-code operations inspected for one deferred-home proof.
const MAX_CALLER_SCAN_OPS: usize = 4096;

fn signed(value: u64, size: u32) -> Option<i64> {
    let bits = size.checked_mul(8)?;
    match bits {
        1..=63 => Some(((value << (64 - bits)) as i64) >> (64 - bits)),
        64 => Some(value as i64),
        _ => None,
    }
}

pub(crate) fn is_home_slot(offset: i64, size: u32) -> bool {
    size > 0
        && offset >= HOME_START
        && offset
            .checked_add(i64::from(size))
            .is_some_and(|end| end <= HOME_END)
}

fn stack_space(fd: &Funcdata) -> Option<Rc<AddrSpace>> {
    let space = fd.get_scope_local()?.get_space_id();
    (space.stack_grows_negative() && space.get_word_size() == 1).then(|| Rc::clone(space))
}

fn verified_home_gap(fd: &Funcdata, call: &FuncCallSpecs) -> Option<Rc<AddrSpace>> {
    let proto = call.proto();
    if !proto.has_model()
        || !proto.has_store()
        || !proto.is_input_locked()
        || !proto.is_model_locked()
        || proto.is_dotdotdot()
        || proto.is_model_unknown()
        || proto.has_custom_storage()
    {
        return None;
    }
    let model = proto.model();
    if model.is_merged() || model.is_unknown() || !model.is_stack_grows_negative() {
        return None;
    }
    let stack = stack_space(fd)?;
    if stack.get_addr_size() != 8 {
        return None;
    }
    let return_address = fd.get_arch().default_return_addr.as_ref()?;
    let return_space = return_address.space.as_ref()?;
    let return_offset = signed(return_address.offset, stack.get_addr_size())?;
    if !Rc::ptr_eq(return_space, &stack)
        || return_address.size != stack.get_addr_size() as u32
        || return_offset.checked_add(i64::from(return_address.size))? != HOME_START
    {
        return None;
    }
    let input = model.input_opt()?;
    if !input
        .get_spacebase()
        .is_some_and(|space| Rc::ptr_eq(space, &stack))
    {
        return None;
    }
    let stack_entries: Vec<_> = input
        .get_entry()
        .iter()
        .filter(|entry| entry.get_space().get_type() == spacetype::IPTR_SPACEBASE)
        .collect();
    if stack_entries.len() != 1 || input.get_stack_entry().is_none() {
        return None;
    }
    let entry = stack_entries[0];
    if !Rc::ptr_eq(entry.get_space(), &stack)
        || signed(entry.get_base(), stack.get_addr_size())? != HOME_END
    {
        return None;
    }
    Some(stack)
}

/// Translate a callee home cell only when the architecture declares an 8-byte
/// stack return slot and the locked model starts its sole stack input at byte 40.
/// The caller-side proof must still place the translated cell in a negative local.
/// Kuna does not currently expose the cspec `stackshift` as a model property.
pub(crate) fn caller_home_cell(
    fd: &Funcdata,
    call: &FuncCallSpecs,
    offset: i64,
    size: u32,
) -> Option<(u64, i32)> {
    let stack = verified_home_gap(fd, call)?;
    if !is_home_slot(offset, size) {
        return None;
    }
    let (start, size) = callee_frame_cell(fd, call, offset, size)?;
    let start = signed(start, stack.get_addr_size() as u32)?;
    let end = start.checked_add(i64::from(size))?;
    if start >= 0 || end > 0 {
        return None;
    }
    Some((start as u64 & calc_mask(stack.get_addr_size() as i32), size))
}

/// Map a callee-frame byte interval through a resolved call stack offset. This
/// proves physical aliasing only; it does not imply caller ownership.
pub(crate) fn callee_frame_cell(
    fd: &Funcdata,
    call: &FuncCallSpecs,
    offset: i64,
    size: u32,
) -> Option<(u64, i32)> {
    let stack = stack_space(fd)?;
    let addr_size = u32::try_from(stack.get_addr_size()).ok()?;
    let bits = addr_size.checked_mul(8)?;
    if !(1..=64).contains(&bits)
        || size == 0
        || size > i32::MAX as u32
        || call.get_spacebase_offset() == OFFSET_UNKNOWN
    {
        return None;
    }
    let base = signed(call.get_spacebase_offset(), addr_size)?;
    let first = i128::from(base).checked_add(i128::from(offset))?;
    let end = first.checked_add(i128::from(size))?;
    let limit = 1i128.checked_shl(bits - 1)?;
    if first < -limit || first >= limit || end > limit {
        return None;
    }
    let first = i64::try_from(first).ok()?;
    Some((
        first as u64 & calc_mask(stack.get_addr_size() as i32),
        size as i32,
    ))
}

fn overlaps(start: i64, size: i32, other_start: i64, other_size: i32) -> bool {
    size > 0
        && other_size > 0
        && start < other_start.saturating_add(i64::from(other_size))
        && other_start < start.saturating_add(i64::from(size))
}

fn frame_range(fd: &Funcdata, pointer: VarnodeId, width: i32) -> Option<(i64, i64)> {
    let address = crate::p6_variables::kuna_stackranges::frame_address(fd, pointer)?;
    let (start, size) = address.extent(width)?;
    let start = signed(start, u32::try_from(address.pointer_size).ok()?)?;
    Some((start, start.checked_add(i64::from(size))?))
}

fn may_be_stack_derived(
    fd: &Funcdata,
    vn: VarnodeId,
    budget: &mut usize,
    seen: &mut BTreeSet<VarnodeId>,
) -> bool {
    let Some(left) = (*budget).checked_sub(1) else {
        return true;
    };
    *budget = left;
    if !seen.insert(vn) {
        return false;
    }
    let Some(value) = fd.vbank().get(vn) else {
        return true;
    };
    if value.is_spacebase() && value.is_input() {
        return true;
    }
    let Some(def) = value.get_def().and_then(|id| fd.obank().get(id)) else {
        return false;
    };
    if def.code() == OpCode::CPUI_LOAD {
        return false;
    }
    (0..def.num_input()).any(|slot| {
        def.get_in(slot)
            .is_some_and(|input| may_be_stack_derived(fd, input, budget, seen))
    })
}

fn stack_derived(fd: &Funcdata, vn: VarnodeId) -> bool {
    may_be_stack_derived(fd, vn, &mut 128, &mut BTreeSet::new())
}

fn stack_access_width(fd: &Funcdata, op: OpId, code: OpCode) -> Option<i32> {
    let pcode = fd.obank().get(op)?;
    match code {
        OpCode::CPUI_LOAD => pcode
            .get_out()
            .and_then(|vn| fd.vbank().get(vn))
            .map(|vn| vn.get_size()),
        OpCode::CPUI_STORE => pcode
            .get_in(2)
            .and_then(|vn| fd.vbank().get(vn))
            .map(|vn| vn.get_size()),
        _ => None,
    }
}

fn frame_value_overlaps(fd: &Funcdata, vn: VarnodeId, start: i64, size: i32) -> Option<bool> {
    let address = crate::p6_variables::kuna_stackranges::frame_address(fd, vn)?;
    let (frame_start, frame_size) = address.extent(1)?;
    let frame_start = signed(frame_start, u32::try_from(address.pointer_size).ok()?)?;
    Some(overlaps(frame_start, frame_size, start, size))
}

fn pointer_propagation(code: OpCode) -> bool {
    matches!(
        code,
        OpCode::CPUI_COPY
            | OpCode::CPUI_CAST
            | OpCode::CPUI_INT_ADD
            | OpCode::CPUI_INT_SUB
            | OpCode::CPUI_INT_2COMP
            | OpCode::CPUI_PTRADD
            | OpCode::CPUI_PTRSUB
    )
}

fn pointer_temporary(fd: &Funcdata, vn: VarnodeId) -> bool {
    let Some(value) = fd.vbank().get(vn) else {
        return false;
    };
    let space = value.get_space();
    let register = fd.get_arch().manage().get_space_by_name("register");
    (register.is_some_and(|register| Rc::ptr_eq(register, space))
        || space.get_type() == spacetype::IPTR_INTERNAL)
        && !value.is_persist()
        && !value.is_stack_store()
}

fn stack_copy_overlaps(
    fd: &Funcdata,
    vn: VarnodeId,
    stack: &AddrSpace,
    start: i64,
    size: i32,
) -> bool {
    fd.vbank().get(vn).is_some_and(|value| {
        value.get_space().get_index() == stack.get_index()
            && signed(value.get_offset(), stack.get_addr_size())
                .is_some_and(|offset| overlaps(offset, value.get_size(), start, size))
    })
}

fn strict_other_call_unobserved(fd: &Funcdata, id: OpId, start: u64, size: i32) -> bool {
    if !(1..=16).contains(&size) {
        return false;
    }
    let Some(mask) = 1u32.checked_shl(size as u32).map(|bits| bits - 1) else {
        return false;
    };
    crate::p3_dataflow::kuna_calleememory::unobserved_strict(fd, id, start, size, mask)
        == Some(mask)
}

/// Check caller operations for reads, forwarding, or unresolved aliases of a
/// mapped home cell. Direct calls need complete strict summaries; indirect and
/// user calls remain barriers.
pub(crate) fn caller_home_unobserved(
    fd: &Funcdata,
    current_call: OpId,
    start: u64,
    size: i32,
) -> bool {
    if !(1..=16).contains(&size) {
        return false;
    }
    let Some(stack) = stack_space(fd) else {
        return false;
    };
    let Some(start_signed) = signed(start, stack.get_addr_size()) else {
        return false;
    };
    let Some(end_signed) = start_signed.checked_add(i64::from(size)) else {
        return false;
    };
    if start_signed >= 0 || end_signed > 0 {
        return false;
    }

    let mut scanned_ops = 0;
    for (_, id) in fd.obank().iter_all() {
        if scanned_ops == MAX_CALLER_SCAN_OPS {
            return false;
        }
        scanned_ops += 1;
        let Some(op) = fd.obank().get(id) else {
            return false;
        };
        if op.is_dead() {
            continue;
        }
        match op.code() {
            OpCode::CPUI_CALL if id != current_call => {
                if !strict_other_call_unobserved(fd, id, start, size) {
                    return false;
                }
            }
            OpCode::CPUI_CALLIND | OpCode::CPUI_CALLOTHER if id != current_call => return false,
            _ => {}
        }

        for slot in 0..op.num_input() {
            let Some(input) = op.get_in(slot) else {
                continue;
            };
            if stack_copy_overlaps(fd, input, &stack, start_signed, size) {
                return false;
            }
        }

        if matches!(op.code(), OpCode::CPUI_LOAD | OpCode::CPUI_STORE) {
            let Some(pointer) = op.get_in(1) else {
                return false;
            };
            let Some(width) = stack_access_width(fd, id, op.code()) else {
                return false;
            };
            let derived = stack_derived(fd, pointer);
            let pointer_range = frame_range(fd, pointer, width);
            let access_space =
                crate::p3_dataflow::ruleaction_4::RuleLoadVarnode::check_spacebase(fd, id)
                    .map(|(space, _)| space);
            if let Some((first, end)) = pointer_range {
                if overlaps(first, (end - first) as i32, start_signed, size) {
                    if op.code() == OpCode::CPUI_LOAD {
                        return false;
                    }
                    if access_space
                        .as_ref()
                        .map_or(true, |space| !Rc::ptr_eq(space, &stack))
                    {
                        return false;
                    }
                }
            } else if derived {
                return false;
            }
            if derived
                && access_space
                    .as_ref()
                    .map_or(true, |space| !Rc::ptr_eq(space, &stack))
            {
                return false;
            }
        }

        for slot in 0..op.num_input() {
            let Some(input) = op.get_in(slot) else {
                continue;
            };
            let derived = stack_derived(fd, input);
            if !derived {
                continue;
            }
            let possible_home = match frame_value_overlaps(fd, input, start_signed, size) {
                Some(overlaps) => overlaps,
                None => true,
            };
            match op.code() {
                OpCode::CPUI_CALL => {}
                OpCode::CPUI_LOAD if slot == 1 && possible_home => return false,
                OpCode::CPUI_LOAD if slot == 1 => {}
                OpCode::CPUI_STORE if slot == 1 => {}
                OpCode::CPUI_STORE => return false,
                OpCode::CPUI_RETURN => return false,
                code if pointer_propagation(code) => {
                    let Some(out_id) = op.get_out() else {
                        return false;
                    };
                    if !pointer_temporary(fd, out_id)
                        || crate::p6_variables::kuna_stackranges::frame_address(fd, out_id)
                            .is_none()
                    {
                        return false;
                    }
                }
                _ => return false,
            }
        }
    }
    true
}

#[cfg(test)]
#[path = "kuna_calleehomes/tests.rs"]
pub(crate) mod tests;
