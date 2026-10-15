//! Scalar declarations for inferred sixteen-byte register temporaries.

use crate::context::HighVariableId;
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
        let Some(v) = fd.vbank().get(h.get_instance(i)) else {
            return false;
        };
        let ty = v.get_type();
        if v.get_size() != 16
            || v.is_input()
            || v.is_persist()
            || v.is_addr_tied()
            || v.is_type_lock()
            || ty.get_metatype() != type_metatype::TYPE_ARRAY
            || !ty.get_array_base().is_some_and(|e| {
                e.get_size() == 1 && e.get_metatype() == type_metatype::TYPE_UNKNOWN
            })
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
