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
