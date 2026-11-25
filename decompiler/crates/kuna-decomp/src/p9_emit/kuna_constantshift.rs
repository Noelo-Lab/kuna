//! (kuna `armconstshift all`) Defined C for fixed 64-bit left shifts.
use std::rc::Rc;

use crate::architecture::Architecture;
use crate::context::{OpId, VarnodeId};
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;
use kuna_num::opcodes::OpCode;

pub(crate) struct ConstantLslPlan {
    pub(crate) lhs: VarnodeId,
    pub(crate) count: VarnodeId,
    pub(crate) unsigned_type: Rc<Datatype>,
    pub(crate) signed_result_type: Option<Rc<Datatype>>,
}

/// Shift the complete input modulo 64 bits and restore the result type.
pub(crate) fn plan_fixed64_lsl(
    fd: &Funcdata,
    arch: &Architecture,
    op: OpId,
    printed: impl Fn(VarnodeId) -> Rc<Datatype>,
) -> Option<ConstantLslPlan> {
    if arch.arm_constant_shift != crate::p9_emit::kuna_armconstantshift::ConstShiftMode::All {
        return None;
    }
    let operation = fd.obank().get(op)?;
    if operation.code() != OpCode::CPUI_INT_LEFT || operation.num_input() != 2 {
        return None;
    }
    let (lhs, count) = (operation.get_in(0)?, operation.get_in(1)?);
    let (value, amount, output) = (
        fd.vbank().get(lhs)?,
        fd.vbank().get(count)?,
        fd.vbank().get(operation.get_out()?)?,
    );
    if value.get_size() != 8
        || output.get_size() != 8
        || !amount.is_constant()
        || amount.get_offset() >= 64
    {
        return None;
    }
    let signed = |t: &Datatype| {
        matches!(
            t.get_metatype(),
            type_metatype::TYPE_INT | type_metatype::TYPE_ENUM_INT
        )
    };
    let implied_sign_extension = value.is_implied()
        && value
            .get_def()
            .and_then(|def| fd.obank().get(def))
            .is_some_and(|def| def.code() == OpCode::CPUI_INT_SEXT);
    let printed_type = printed(lhs);
    // Keep the compact form when the known bits prove a nonnegative result
    // representable in signed64. This also preserves existing widened indexes.
    if printed_type.get_size() == 8
        && value.get_nz_mask() <= (i64::MAX as u64 >> amount.get_offset())
    {
        return None;
    }
    if !implied_sign_extension && !signed(&printed_type) {
        return None;
    }
    Some(ConstantLslPlan {
        lhs,
        count,
        unsigned_type: arch.types().get_base(8, type_metatype::TYPE_UINT).ok()?,
        signed_result_type: if signed(output.get_type_def_facing()) {
            Some(arch.types().get_base(8, type_metatype::TYPE_INT).ok()?)
        } else {
            None
        },
    })
}

#[cfg(test)]
mod tests;
