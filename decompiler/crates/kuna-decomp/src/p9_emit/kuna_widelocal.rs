//! Scalar declarations for inferred sixteen-byte register temporaries.

use crate::context::{HighVariableId, VarnodeId};
use crate::dtype::type_metatype;
use crate::funcdata::Funcdata;
use kuna_num::opcodes::OpCode;

pub(crate) fn scalar_bits(fd: &Funcdata, high: HighVariableId) -> bool {
    let Some(h) = fd.high_bank().get(high) else {
        return false;
    };
    if h.num_instances() == 0 || h.kuna_symbol_type().is_some() {
        return false;
    }
    (0..h.num_instances()).all(|i| {
        let vn = h.get_instance(i);
        let Some(v) = fd.vbank().get(vn) else {
            return false;
        };
        if v.get_size() != 16
            || v.is_input()
            || v.is_persist()
            || v.is_addr_tied()
            || v.is_type_lock()
            || !byte_array(fd, vn)
            || !scalar_source(fd, vn, high)
        {
            return false;
        }
        v.descend_iter().all(|op| {
            fd.obank().get(op).is_some_and(|o| {
                matches!(
                    o.code(),
                    OpCode::CPUI_SUBPIECE
                        | OpCode::CPUI_COPY
                        | OpCode::CPUI_MULTIEQUAL
                        | OpCode::CPUI_INT_AND
                        | OpCode::CPUI_INT_OR
                        | OpCode::CPUI_INT_XOR
                )
            })
        })
    })
}

fn byte_array(fd: &Funcdata, vn: VarnodeId) -> bool {
    fd.vbank().get(vn).is_some_and(|v| {
        let ty = v.get_type();
        ty.get_metatype() == type_metatype::TYPE_ARRAY
            && ty
                .get_array_base()
                .is_some_and(|e| e.get_size() == 1 && e.get_metatype() == type_metatype::TYPE_UNKNOWN)
    })
}

/// False when the definition prints as an array value, which C would convert to
/// a pointer before assigning it to an integer.
fn scalar_source(fd: &Funcdata, vn: VarnodeId, high: HighVariableId) -> bool {
    let Some(def) = fd.vbank().get(vn).and_then(|v| v.get_def()).and_then(|d| fd.obank().get(d)) else {
        return true;
    };
    match def.code() {
        OpCode::CPUI_LOAD => false,
        OpCode::CPUI_COPY => def.get_in(0).and_then(|i| fd.vbank().get(i)).is_none_or(|i| {
            i.is_constant()
                || i.get_high() == Some(high)
                || i.get_type().get_metatype() != type_metatype::TYPE_ARRAY
        }),
        _ => true,
    }
}
