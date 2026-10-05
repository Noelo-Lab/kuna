//! Storage transfers preserve bits; FLOAT_* conversion operations change values.

use crate::context::{OpId, VarnodeId};
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;
use kuna_num::opcodes::OpCode;
use std::rc::Rc;

pub(crate) struct Transfer {
    pub input: VarnodeId,
    pub target: Rc<Datatype>,
    pub size: i32,
    pub to_float: bool,
}

/// A same-width `CAST` between an integer and a float, judged by the types the
/// printed C declares: a named variable's declaration, else the value's own type.
pub(crate) fn storage_transfer(fd: &Funcdata, decl_high_type: bool, op: OpId) -> Option<Transfer> {
    let operation = fd.obank().get(op)?;
    if operation.code() != OpCode::CPUI_CAST {
        return None;
    }
    let input = operation.get_in(0)?;
    let out = operation.get_out()?;
    let src = fd.vbank().get(input)?;
    let dst = fd.vbank().get(out)?;
    let size = src.get_size();
    if !matches!(size, 4 | 8) || dst.get_size() != size {
        return None;
    }
    let from_ty = crate::printc::declared_variable_type(fd, decl_high_type, input)
        .unwrap_or_else(|| src.get_type_read_facing(op).clone());
    let to_ty = crate::printc::declared_variable_type(fd, decl_high_type, out)
        .unwrap_or_else(|| dst.get_type_def_facing().clone());
    let from = from_ty.get_metatype();
    let to = to_ty.get_metatype();
    if from == type_metatype::TYPE_UNKNOWN && untyped_call_value(fd, input, 0) {
        return None;
    }
    let integer = |t| {
        matches!(
            t,
            type_metatype::TYPE_INT | type_metatype::TYPE_UINT | type_metatype::TYPE_UNKNOWN
        )
    };
    let to_float = to == type_metatype::TYPE_FLOAT && integer(from);
    if !to_float && !(from == type_metatype::TYPE_FLOAT && integer(to)) {
        return None;
    }
    Some(Transfer {
        input,
        target: to_ty,
        size,
        to_float,
    })
}

/// Whether a same-width `CAST` of `input` to a float prints as a reinterpretation
/// of its bits: the integer side [`storage_transfer`] accepts, judged by the same
/// printed type.
pub(crate) fn reinterprets_to_float(fd: &Funcdata, decl_high_type: bool, input: VarnodeId) -> bool {
    let Some(src) = fd.vbank().get(input) else {
        return false;
    };
    let from = crate::printc::declared_variable_type(fd, decl_high_type, input)
        .unwrap_or_else(|| src.get_type().clone())
        .get_metatype();
    match from {
        type_metatype::TYPE_INT | type_metatype::TYPE_UINT => true,
        type_metatype::TYPE_UNKNOWN => !untyped_call_value(fd, input, 0),
        _ => false,
    }
}

/// An untyped call has no integer C return contract. Its ABI storage alone
/// cannot establish the type of the call expression that the printer emits.
fn untyped_call_value(fd: &Funcdata, vn: VarnodeId, depth: usize) -> bool {
    if depth == 16 {
        return true;
    }
    let Some(v) = fd.vbank().get(vn) else {
        return true;
    };
    if v.get_type().get_metatype() != type_metatype::TYPE_UNKNOWN {
        return false;
    }
    let Some(op) = v.get_def().and_then(|op| fd.obank().get(op)) else {
        return false;
    };
    match op.code() {
        OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => true,
        OpCode::CPUI_COPY | OpCode::CPUI_CAST | OpCode::CPUI_SUBPIECE => op
            .get_in(0)
            .is_none_or(|input| untyped_call_value(fd, input, depth + 1)),
        _ => false,
    }
}
