//! Prevent constant propagation across indexed byte writes to initialized stack slots.

use kuna_base::address::Address;
use std::rc::Rc;

pub(super) fn enabled(
    fd: &crate::funcdata::Funcdata,
    addr: &Address,
    writes: &[crate::context::VarnodeId],
) -> bool {
    fd.get_arch().stack_store_guard
        && addr
            .get_space()
            .zip(fd.get_arch().manage().get_stack_space())
            .is_some_and(|(space, stack)| Rc::ptr_eq(space, stack))
        && writes.iter().any(|&vn| constant_write(fd, vn))
}

fn constant_write(fd: &crate::funcdata::Funcdata, mut vn: crate::context::VarnodeId) -> bool {
    use kuna_num::opcodes::OpCode;
    for _ in 0..16 {
        let Some(value) = fd.vbank().get(vn) else {
            return false;
        };
        if value.is_constant() {
            return true;
        }
        let Some(op) = value.get_def().and_then(|id| fd.obank().get(id)) else {
            return false;
        };
        if !matches!(
            op.code(),
            OpCode::CPUI_COPY
                | OpCode::CPUI_SUBPIECE
                | OpCode::CPUI_INT_ZEXT
                | OpCode::CPUI_INT_SEXT
        ) {
            return false;
        }
        let Some(input) = op.get_in(0) else {
            return false;
        };
        vn = input;
    }
    false
}

/// Keep a stack slot's STORE INDIRECT whole instead of narrowing it to a
/// piece that some read uses as one multi-byte value: that piece would print
/// as a separate local the store never reaches. Byte reads may still narrow,
/// since a byte piece prints as an element of the array the store indexes.
pub(super) fn keeps_store_indirect_whole(
    fd: &crate::funcdata::Funcdata,
    vn: crate::context::VarnodeId,
    store: crate::context::OpId,
) -> bool {
    use kuna_num::opcodes::OpCode;
    if !fd.get_arch().stack_store_guard
        || fd.obank().get(store).is_none_or(|op| op.code() != OpCode::CPUI_STORE)
    {
        return false;
    }
    let Some(slot) = fd.vbank().get(vn) else {
        return false;
    };
    fd.get_arch().manage().get_stack_space().is_some_and(|s| Rc::ptr_eq(s, slot.get_space()))
        && slot.descend_iter().any(|op| {
            let Some(out) = fd.obank().get(op).and_then(|o| o.get_out()).and_then(|o| fd.vbank().get(o)) else {
                return false;
            };
            out.get_size() > 1
                && out.descend_iter().any(|use_op| {
                    fd.obank().get(use_op).is_some_and(|u| u.code() != OpCode::CPUI_INDIRECT)
                })
        })
}
