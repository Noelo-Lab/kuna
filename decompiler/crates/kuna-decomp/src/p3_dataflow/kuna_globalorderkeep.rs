//! Preserve a global's COPY when a forced marker merge would move the store
//! to its input's earlier computation.

use crate::context::{OpId, VarnodeId};
use crate::funcdata::Funcdata;
use crate::kuna_indexaliasguard::LEVEL_GLOBAL;
use kuna_num::opcodes::OpCode;

pub fn declines(data: &Funcdata, op: OpId, vn: VarnodeId, input: VarnodeId) -> bool {
    if data.get_arch().index_alias_guard != LEVEL_GLOBAL {
        return false;
    }
    let (Some(global), Some(value), Some(reader)) = (
        data.vbank().get(vn),
        data.vbank().get(input),
        data.obank().get(op),
    ) else {
        return false;
    };
    if !global.is_persist()
        || !data.global_store_guarded(global)
        || global.is_global_load()
        || value.is_global_load()
        || value.is_persist()
        || value.is_addr_tied()
        || value.is_constant()
        || value.is_input()
        || !reader.is_marker()
    {
        return false;
    }
    let Some(out) = reader.get_out().and_then(|v| data.vbank().get(v)) else {
        return false;
    };
    if out.get_addr() != global.get_addr() || out.get_size() != global.get_size() {
        return false;
    }
    let Some(copy) = global
        .get_def()
        .and_then(|d| data.obank().get(d).map(|o| (d, o)))
    else {
        return false;
    };
    copy.1.code() == OpCode::CPUI_COPY
        && !copy.1.is_return_copy()
        && crate::p6_variables::kuna_globalorder::write_moves(data, copy.0, input)
        && !crate::kuna_globalstorekeep::old_value_read_after(data, vn)
}
