//! A floating conversion can quiet a signaling NaN even when it changes no
//! finite value. Keep that effect unless NaNs are ignored or an earlier
//! operation has already quieted the input.

use crate::context::VarnodeId;
use crate::funcdata::Funcdata;
use kuna_num::opcodes::OpCode;

pub(crate) fn can_elide(data: &Funcdata, input: VarnodeId) -> bool {
    data.get_arch().nan_ignore_all || known_quiet(data, input, &mut 32)
}

fn known_quiet(data: &Funcdata, input: VarnodeId, budget: &mut usize) -> bool {
    if *budget == 0 {
        return false;
    }
    *budget -= 1;
    let Some(value) = data.vbank().get(input) else {
        return false;
    };
    if value.is_constant() {
        return data
            .get_arch()
            .get_float_format(value.get_size())
            .is_some_and(|format| !format.get_host_float(value.get_offset()).0.is_nan());
    }
    let Some(op) = value.get_def().and_then(|id| data.obank().get(id)) else {
        return false;
    };
    match op.code() {
        OpCode::CPUI_FLOAT_ADD
        | OpCode::CPUI_FLOAT_SUB
        | OpCode::CPUI_FLOAT_MULT
        | OpCode::CPUI_FLOAT_DIV
        | OpCode::CPUI_FLOAT_SQRT
        | OpCode::CPUI_FLOAT_INT2FLOAT
        | OpCode::CPUI_FLOAT_CEIL
        | OpCode::CPUI_FLOAT_FLOOR
        | OpCode::CPUI_FLOAT_ROUND => true,
        OpCode::CPUI_FLOAT_FLOAT2FLOAT => op
            .get_in(0)
            .and_then(|id| data.vbank().get(id))
            .is_some_and(|source| source.get_size() != value.get_size()),
        OpCode::CPUI_COPY | OpCode::CPUI_CAST => op.get_in(0).is_some_and(|source| {
            data.vbank()
                .get(source)
                .is_some_and(|v| v.get_size() == value.get_size())
                && known_quiet(data, source, budget)
        }),
        OpCode::CPUI_MULTIEQUAL => {
            op.num_input() > 0
                && (0..op.num_input()).all(|slot| {
                    op.get_in(slot)
                        .is_some_and(|source| known_quiet(data, source, budget))
                })
        }
        _ => false,
    }
}
