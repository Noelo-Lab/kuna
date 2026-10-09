//! Loads cannot move across physical writes to escaped frame storage.

use crate::context::VarnodeId;
use crate::funcdata::Funcdata;
use kuna_num::opcodes::OpCode;

pub(crate) fn crosses_write(fd: &Funcdata, id: VarnodeId) -> bool {
    if !fd.get_arch().stack_views {
        return false;
    }
    let Some(value) = fd.vbank().get(id) else {
        return false;
    };
    let Some(load) = value.get_def().and_then(|op| fd.obank().get(op)) else {
        return false;
    };
    if load.code() != OpCode::CPUI_LOAD {
        return false;
    }
    let Some(cover) = value.cover() else {
        return false;
    };
    let Some(scope) = fd.get_scope_local() else {
        return false;
    };
    let stack = scope.get_space_id();
    let Some(index) = load
        .get_in(0)
        .and_then(|id| fd.vbank().get(id))
        .map(|v| v.get_offset())
    else {
        return false;
    };
    if !stack
        .get_contain()
        .is_some_and(|space| space.get_index() as u64 == index)
    {
        return false;
    }
    let Some(pointer) = load.get_in(1) else {
        return false;
    };
    if fd.vbank().get(pointer).is_some_and(|v| v.is_constant()) {
        return false;
    }
    let extent = crate::kuna_stackranges::frame_address(fd, pointer)
        .and_then(|address| address.extent(value.get_size()));
    for (block, _) in cover.iter() {
        if block < 0 || block >= fd.bblocks_get_size() {
            continue;
        }
        for id in fd.bb_ops(fd.bblocks_get_block(block)) {
            let Some(op) = fd.obank().get(id) else {
                continue;
            };
            if op.is_dead() || op.is_marker() || op.is_return_copy() {
                continue;
            }
            let Some(written) = op.get_out().and_then(|id| fd.vbank().get(id)) else {
                continue;
            };
            if !written.is_stack_store() || written.get_space().get_index() != stack.get_index() {
                continue;
            }
            let overlaps = extent.map_or(!written.has_no_local_alias(), |(start, size)| {
                let at = written.get_offset();
                stack.wrap_offset(at.wrapping_sub(start)) < size as u64
                    || stack.wrap_offset(start.wrapping_sub(at)) < written.get_size() as u64
            });
            if overlaps && cover.contain(block, fd.op_cover_point_pub(id), 2) {
                return true;
            }
        }
    }
    false
}
