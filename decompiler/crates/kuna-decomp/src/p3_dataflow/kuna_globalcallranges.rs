//! Exact caller-side memory intervals of a completely decoded leaf callee.

use kuna_base::address::Address;
use kuna_base::space::spacetype;
use kuna_num::opcodes::OpCode;
use slotmap::SecondaryMap;

use crate::context::{OpId, VarnodeId};
use crate::fspec::{FuncCallSpecs, OFFSET_UNKNOWN};
use crate::funcdata::Funcdata;

#[derive(Debug, Default)]
pub(super) struct Calls {
    resolved: SecondaryMap<OpId, Option<Vec<(u64, u64)>>>,
}

impl Calls {
    pub(super) fn clear(&mut self) {
        self.resolved.clear();
    }

    pub(super) fn preserves(
        &mut self,
        fd: &Funcdata,
        call: &FuncCallSpecs,
        address: &Address,
        size: i32,
    ) -> bool {
        let Some(space) = address.get_space() else {
            return false;
        };
        if !fd.get_arch().global_call_ranges
            || fd.callee_memory.is_empty()
            || space.get_type() != spacetype::IPTR_PROCESSOR
            || space.get_word_size() != 1
            || size <= 0
            || fd
                .get_arch()
                .manage()
                .get_default_data_space()
                .map(|s| s.get_index())
                != Some(space.get_index())
        {
            return false;
        }
        let Some(end) = address.get_offset().checked_add(size as u64) else {
            return false;
        };
        if end - 1 > space.get_highest() {
            return false;
        }
        let op = call.get_op();
        if !self.resolved.contains_key(op) {
            self.resolved.insert(op, resolve(fd, call));
        }
        self.resolved
            .get(op)
            .and_then(|ranges| ranges.as_ref())
            .is_some_and(|ranges| {
                ranges
                    .iter()
                    .all(|&(first, last)| end <= first || last <= address.get_offset())
            })
    }
}

pub(super) fn should_seed(fd: &Funcdata) -> bool {
    if fd.num_calls() < 16 {
        return false;
    }
    let Some(space) = fd.get_arch().manage().get_default_data_space() else {
        return false;
    };
    fd.vbank()
        .iter_loc()
        .filter(|&id| {
            fd.vbank()
                .get(id)
                .is_some_and(|v| v.is_written() && v.get_space().get_index() == space.get_index())
        })
        .take(64)
        .count()
        == 64
}

fn constant(fd: &Funcdata, id: VarnodeId, width: i32, remaining: u8) -> Option<u64> {
    let value = fd.vbank().get(id)?;
    if value.get_size() != width || remaining == 0 {
        return None;
    }
    if value.is_constant() {
        return (value.get_offset() <= kuna_base::address::calc_mask(width))
            .then_some(value.get_offset());
    }
    if value.get_space().get_type() != spacetype::IPTR_INTERNAL
        && value.get_space().get_name() != "register"
    {
        return None;
    }
    let def = fd.obank().get(value.get_def()?)?;
    let input = |slot| constant(fd, def.get_in(slot)?, width, remaining - 1);
    let result = match def.code() {
        OpCode::CPUI_COPY | OpCode::CPUI_CAST => input(0)?,
        OpCode::CPUI_INT_ADD => input(0)?.checked_add(input(1)?)?,
        OpCode::CPUI_INT_SUB => input(0)?.checked_sub(input(1)?)?,
        _ => return None,
    };
    (result <= kuna_base::address::calc_mask(width)).then_some(result)
}

fn resolve(fd: &Funcdata, call: &FuncCallSpecs) -> Option<Vec<(u64, u64)>> {
    if call.proto().has_effect_override()
        || fd.obank().get(call.get_op())?.code() != OpCode::CPUI_CALL
    {
        return None;
    }
    let space = fd.get_arch().manage().get_default_data_space()?;
    if space.get_word_size() != 1 || !matches!(space.get_addr_size(), 4 | 8) {
        return None;
    }
    let entry = call.get_entry_address();
    let summary = fd
        .callee_memory
        .get(&(entry.get_space()?.get_index(), entry.get_offset()))?;
    if summary.uses_frame_return() {
        let stack = fd.get_arch().manage().get_stack_space()?;
        let return_address = fd.get_arch().default_return_addr.as_ref()?;
        if call.get_spacebase_offset() == OFFSET_UNKNOWN
            || stack.get_word_size() != 1
            || return_address.offset != 0
            || return_address.size != stack.get_addr_size() as u32
            || return_address.space.as_ref().map(|s| s.get_index()) != Some(stack.get_index())
        {
            return None;
        }
    }
    let mut ranges = Vec::new();
    for (root_space, root_offset, root_size, offset, size) in summary.global_accesses()? {
        if root_size != space.get_addr_size() as u32 {
            return None;
        }
        let mut actual = None;
        if !call.is_input_active() {
            for slot in 0..call.proto().num_params() {
                let param = call.proto().get_param(slot)?;
                let address = param.get_address();
                if address.get_space().map(|s| s.get_index()) == Some(root_space)
                    && address.get_offset() == root_offset
                    && param.get_size() == root_size as i32
                {
                    actual = fd.obank().get(call.get_op())?.get_in(slot + 1);
                    break;
                }
            }
        } else {
            let active = call.active_input();
            for index in 0..active.get_num_trials() {
                let trial = active.get_trial(index);
                let address = trial.get_address();
                if address.get_space().map(|s| s.get_index()) == Some(root_space)
                    && address.get_offset() == root_offset
                    && trial.get_size() == root_size as i32
                    && trial.get_slot() > 0
                    && trial.get_slot() < fd.obank().get(call.get_op())?.num_input()
                    && !trial.is_unref()
                    && !trial.is_definitely_not_used()
                    && !trial.is_ind_create_formed()
                {
                    actual = fd.obank().get(call.get_op())?.get_in(trial.get_slot());
                    break;
                }
            }
        }
        let base = constant(fd, actual?, root_size as i32, 16)?;
        let first = if offset < 0 {
            base.checked_sub(offset.unsigned_abs())?
        } else {
            base.checked_add(offset as u64)?
        };
        let last = first.checked_add(u64::from(size))?;
        if last.checked_sub(1)? > space.get_highest() {
            return None;
        }
        ranges.push((first, last));
    }
    Some(ranges)
}
