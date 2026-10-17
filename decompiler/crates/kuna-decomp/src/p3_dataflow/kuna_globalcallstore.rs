//! Preserve a memory-loaded global store when only its own marker reads it
//! (`option globalcallstore`). Later reload folding or load splitting must not
//! move the assignment or interleave a snapshot's reads with writes to
//! overlapping memory. The kept COPY prints its LOAD inline unless
//! `kuna_loadorder` keeps the load ahead of a store to a global it may read,
//! which it does only at `option indexaliasguard global`; at any other level
//! the refusal stays off so the load cannot print after such a store.

use crate::context::{OpId, VarnodeId};
use crate::funcdata::Funcdata;
use crate::kuna_indexaliasguard::LEVEL_GLOBAL;
use kuna_num::opcodes::OpCode;

pub fn declines(data: &Funcdata, op: OpId, global: VarnodeId, source: VarnodeId) -> bool {
    if !data.get_arch().global_call_store || !load_order_enabled(data) {
        return false;
    }
    let Some(g) = data.vbank().get(global) else {
        return false;
    };
    if !g.is_persist() {
        return false;
    }
    let Some(reader) = data.obank().get(op) else {
        return false;
    };
    if !reader.is_marker() || data.lone_descend(global) != Some(op) {
        return false;
    }
    let Some(out) = reader.get_out().and_then(|v| data.vbank().get(v)) else {
        return false;
    };
    if out.get_addr() != g.get_addr() || out.get_size() != g.get_size() {
        return false;
    }
    data.vbank()
        .get(source)
        .and_then(|v| v.get_def())
        .and_then(|o| data.obank().get(o))
        .is_some_and(|load| load.code() == OpCode::CPUI_LOAD)
}

/// The level at which a retained COPY can safely print its LOAD input inline.
pub(crate) fn load_order_enabled(data: &Funcdata) -> bool {
    data.get_arch().index_alias_guard == LEVEL_GLOBAL
}
