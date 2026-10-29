//! A floating conversion can quiet a signaling NaN even when it changes no
//! finite value. Under `nanignore none` keep that effect unless an earlier
//! operation has already quieted the input; the default `compare` policy and
//! `all` discard it, as upstream does.

use crate::context::{OpId, VarnodeId};
use crate::funcdata::Funcdata;
use kuna_num::opcodes::OpCode;

pub(crate) fn can_elide(data: &Funcdata, input: VarnodeId) -> bool {
    let arch = data.get_arch();
    arch.nan_ignore_all || arch.nan_ignore_compare || known_quiet(data, input, &mut 32)
}

/// Whether a widening's result reaches a conversion back to its source
/// format, the identity pair a C compiler may erase despite NaN quieting.
pub(crate) fn feeds_round_trip(data: &Funcdata, op: OpId) -> bool {
    let Some(convert) = data.obank().get(op) else { return false };
    let (Some(input), Some(output)) = (convert.get_in(0), convert.get_out()) else { return false };
    let Some(source_size) = data.vbank().get(input).map(|v| v.get_size()) else { return false };
    let Some(wide_size) = data.vbank().get(output).map(|v| v.get_size()) else { return false };
    wide_size > source_size && reaches_narrowing(data, output, source_size, &mut 32)
}

fn reaches_narrowing(data: &Funcdata, value: VarnodeId, size: i32, budget: &mut usize) -> bool {
    let Some(vn) = data.vbank().get(value) else { return false };
    let width = vn.get_size();
    vn.descend_iter().any(|id| {
        if *budget == 0 {
            return false;
        }
        *budget -= 1;
        let Some(read) = data.obank().get(id) else { return false };
        let Some(out) = read.get_out().and_then(|o| data.vbank().get(o).map(|v| (o, v.get_size()))) else {
            return false;
        };
        match read.code() {
            OpCode::CPUI_FLOAT_FLOAT2FLOAT => out.1 == size,
            OpCode::CPUI_COPY | OpCode::CPUI_CAST | OpCode::CPUI_MULTIEQUAL => {
                out.1 == width && reaches_narrowing(data, out.0, size, budget)
            }
            OpCode::CPUI_INDIRECT => {
                read.get_in(0) == Some(value) && out.1 == width && reaches_narrowing(data, out.0, size, budget)
            }
            _ => false,
        }
    })
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
