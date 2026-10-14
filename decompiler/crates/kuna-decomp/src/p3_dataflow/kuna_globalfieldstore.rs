//! Preserve the storage identity of a global store spanning aggregate fields.
use crate::context::VarnodeId;
use crate::dtype::type_metatype::{TYPE_ARRAY, TYPE_PARTIALSTRUCT};
use crate::funcdata::Funcdata;
use kuna_num::opcodes::OpCode;

/// Keep an aggregate view's global COPY as a marker input. Propagating its
/// value instead lets later field splitting name register pieces as locals.
pub(crate) fn keeps_store(data: &Funcdata, output: VarnodeId, source: VarnodeId) -> bool {
    if !data.get_arch().global_field_store {
        return false;
    }
    let Some(v) = data.vbank().get(output) else {
        return false;
    };
    if !v.is_persist() || !matches!(v.get_type().get_metatype(), TYPE_PARTIALSTRUCT | TYPE_ARRAY) {
        return false;
    }
    let from_load = data
        .vbank()
        .get(source)
        .and_then(|v| v.get_def())
        .and_then(|op| data.obank().get(op))
        .is_some_and(|op| op.code() == OpCode::CPUI_LOAD);
    !from_load || crate::p3_dataflow::kuna_globalcallstore::load_order_enabled(data)
}
