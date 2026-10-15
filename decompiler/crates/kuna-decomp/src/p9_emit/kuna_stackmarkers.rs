//! Physical memory versions and their merge copies are not additional frame writes.

use crate::context::OpId;
use crate::funcdata::Funcdata;
use kuna_num::opcodes::OpCode;

pub(crate) fn nonprinting(fd: &Funcdata, id: OpId) -> bool {
    if !fd.get_arch().stack_views {
        return false;
    }
    if physical_effect(fd, id) {
        return true;
    }
    let Some(op) = fd.obank().get(id) else {
        return false;
    };
    if !matches!(op.code(), OpCode::CPUI_COPY | OpCode::CPUI_CAST) {
        return false;
    }
    let Some(output) = op.get_out().and_then(|id| fd.vbank().get(id)) else {
        return false;
    };
    let Some(unique) = fd.get_arch().manage().get_unique_space() else {
        return false;
    };
    output.get_space().get_index() == unique.get_index()
        && !output.is_stack_store()
        && output.num_descend() != 0
        && output
            .descend_iter()
            .all(|consumer| physical_effect(fd, consumer))
}

fn physical_effect(fd: &Funcdata, id: OpId) -> bool {
    let Some(op) = fd.obank().get(id) else {
        return false;
    };
    if op.code() != OpCode::CPUI_INDIRECT || op.is_indirect_creation() {
        return false;
    }
    let Some(output) = op.get_out().and_then(|id| fd.vbank().get(id)) else {
        return false;
    };
    let Some(stack) = fd.get_arch().manage().get_stack_space() else {
        return false;
    };
    output.get_space().get_index() == stack.get_index()
}
