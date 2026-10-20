//! Preserve caller stack storage across a complete, memory-write-free helper.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_num::opcodes::OpCode;

use crate::fspec::{FuncCallSpecs, OFFSET_UNKNOWN};
use crate::funcdata::Funcdata;
use crate::kuna_rustabi::CalleeReturnWrites;

fn register_only_body(summary: &CalleeReturnWrites, register: i32, stack: (u64, u32)) -> bool {
    if !summary.is_complete()
        || summary.instruction_count() < 2
        || !summary.store_spaces().is_empty()
        || summary.reads_nonreturn_memory()
        || !summary.stack_pointer_preserved_until_return()
    {
        return false;
    }
    let Some(stack_end) = stack.0.checked_add(u64::from(stack.1)) else {
        return false;
    };
    let mut substantive = false;
    for &(space, offset, size) in summary.written_ranges() {
        if space != register || size <= 0 {
            return false;
        }
        let Some(end) = offset.checked_add(size as u64) else {
            return false;
        };
        if (offset < stack_end && stack.0 < end) || summary.is_return_target(space, offset, size) {
            continue;
        }
        substantive = true;
    }
    substantive
}

/// Reuse the register-write probe; the call's own return-address store stays live.
pub(super) fn preserves(fd: &Funcdata, call: &FuncCallSpecs, address: &Address, size: i32) -> bool {
    if !fd.get_arch().callee_preserves
        || !(1..=16).contains(&size)
        || call.proto().has_effect_override()
        || call.get_spacebase_offset() == OFFSET_UNKNOWN
        || !fd
            .obank()
            .get(call.get_op())
            .is_some_and(|op| op.code() == OpCode::CPUI_CALL)
    {
        return false;
    }
    let manager = fd.get_arch().manage();
    let Some(frame) = manager.get_stack_space() else {
        return false;
    };
    if !frame.stack_grows_negative()
        || frame.get_word_size() != 1
        || !address
            .get_space()
            .is_some_and(|space| Rc::ptr_eq(space, frame))
    {
        return false;
    }
    let width = frame.get_addr_size() as i32;
    if !matches!(width, 4 | 8) {
        return false;
    }
    let Some(return_address) = fd.get_arch().default_return_addr.as_ref() else {
        return false;
    };
    if return_address.offset != 0
        || return_address.size != width as u32
        || !return_address
            .space
            .as_ref()
            .is_some_and(|space| Rc::ptr_eq(space, frame))
    {
        return false;
    }
    let relative = frame.wrap_offset(
        address
            .get_offset()
            .wrapping_sub(call.get_spacebase_offset()),
    );
    if !crate::kuna_calleeprotostack::above_callee_frame(relative, size, width, width) {
        return false;
    }
    let Some(register) = manager.get_space_by_name("register") else {
        return false;
    };
    let Ok(stack) = frame.get_spacebase(0) else {
        return false;
    };
    if !stack
        .space
        .as_ref()
        .is_some_and(|space| Rc::ptr_eq(space, register))
    {
        return false;
    }
    let entry = call.get_entry_address();
    if entry.is_invalid() {
        return false;
    }
    fd.kuna_callee_ret_writes(entry).is_some_and(|summary| {
        register_only_body(summary, register.get_index(), (stack.offset, stack.size))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn complete_register_work_is_required() {
        let body = |writes, stores, complete, count| {
            CalleeReturnWrites::from_parts_sized(writes, stores, complete, count)
        };
        assert!(register_only_body(
            &body(vec![(1, 0, 4), (1, 16, 4)], vec![], true, 2),
            1,
            (16, 4)
        ));
        assert!(!register_only_body(
            &body(vec![(1, 16, 4)], vec![], true, 1),
            1,
            (16, 4)
        ));
        assert!(!register_only_body(
            &body(vec![(1, 16, 4)], vec![], true, 2),
            1,
            (16, 4)
        ));
        assert!(!register_only_body(
            &body(vec![(1, 0, 4)], vec![], false, 2),
            1,
            (16, 4)
        ));
        assert!(!register_only_body(
            &body(vec![(1, 0, 4)], vec![2], true, 2),
            1,
            (16, 4)
        ));
        assert!(!register_only_body(
            &body(vec![(2, 0x1000, 4)], vec![], true, 2),
            1,
            (16, 4)
        ));
        assert!(!register_only_body(
            &body(vec![(1, 0, 4), (1, 16, 4)], vec![], true, 2).reading_memory(),
            1,
            (16, 4)
        ));
        let return_only =
            body(vec![(1, 32, 4), (1, 16, 4)], vec![], true, 2).returning_through(1, 32, 4);
        assert!(!register_only_body(&return_only, 1, (16, 4)));
    }
}
