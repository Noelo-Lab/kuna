//! Preserve a memory-loaded global store when only its own marker reads it.
//! Later reload folding or load splitting must not move the assignment or
//! interleave a snapshot's reads with writes to overlapping memory.

use crate::context::{OpId, VarnodeId};
use crate::funcdata::Funcdata;
use kuna_num::opcodes::OpCode;

pub fn declines(data: &Funcdata, op: OpId, global: VarnodeId, source: VarnodeId) -> bool {
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
