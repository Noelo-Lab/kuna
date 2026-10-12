//! Recover binary64 sign operations performed on the high binary32 word.

use crate::context::{OpId, VarnodeId};
use crate::dtype::type_metatype;
use crate::funcdata::Funcdata;
use kuna_num::opcodes::OpCode;

fn slice(fd: &Funcdata, vn: VarnodeId, offset: u64) -> Option<VarnodeId> {
    let value = fd.vbank().get(vn)?;
    if value.get_size() != 4 {
        return None;
    }
    let op = fd.obank().get(value.get_def()?)?;
    if op.code() != OpCode::CPUI_SUBPIECE {
        return None;
    }
    let shift = fd.vbank().get(op.get_in(1)?)?;
    if !shift.is_constant() || shift.get_offset() != offset {
        return None;
    }
    op.get_in(0)
}

/// High-word negation or absolute value changes only the original double’s sign bit.
pub(crate) fn recover(fd: &mut Funcdata, piece: OpId) -> bool {
    let matched = (|| {
        let op = fd.obank().get(piece)?;
        if op.code() != OpCode::CPUI_PIECE || fd.vbank().get(op.get_out()?)?.get_size() != 8 {
            return None;
        }
        let high = fd.vbank().get(op.get_in(0)?)?;
        if high.get_size() != 4 {
            return None;
        }
        let unary = fd.obank().get(high.get_def()?)?;
        if !matches!(
            unary.code(),
            OpCode::CPUI_FLOAT_NEG | OpCode::CPUI_FLOAT_ABS
        ) {
            return None;
        }
        let source = slice(fd, unary.get_in(0)?, 4)?;
        if slice(fd, op.get_in(1)?, 0)? != source {
            return None;
        }
        let whole = fd.vbank().get(source)?;
        (whole.get_size() == 8
            && whole.get_type().get_size() == 8
            && whole.get_type().get_metatype() == type_metatype::TYPE_FLOAT)
            .then_some((source, unary.code()))
    })();
    let Some((source, opcode)) = matched else {
        return false;
    };
    if fd.op_set_input(piece, source, 0).is_err() {
        return false;
    }
    fd.op_remove_input(piece, 1);
    fd.op_set_opcode_code(piece, opcode);
    true
}

#[cfg(test)]
mod tests;
