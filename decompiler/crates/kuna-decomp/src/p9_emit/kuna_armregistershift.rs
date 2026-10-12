//! (kuna) A C-rendering plan for ARM register-controlled logical left shifts.
//!
//! ARM masks a register LSL count to its low eight bits and returns zero when
//! that value is at least the operand width. C leaves an equal-or-larger shift
//! undefined, so the C printer needs a range guard. This module keeps the
//! architecture/opcode selection and the proof that printing both the guard
//! and the shift will not repeat an effect; [`crate::p9_emit::printc::PrintC`]
//! owns only the token emission.

use std::rc::Rc;

use crate::architecture::Architecture;
use crate::context::{OpId, VarnodeId};
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;
use kuna_num::opcodes::OpCode;

/// The operands and types needed to emit one guarded variable ARM LSL.
pub(crate) struct ArmRegisterLslPlan {
    pub(crate) lhs: VarnodeId,
    pub(crate) count: VarnodeId,
    pub(crate) lhs_type: Rc<Datatype>,
    pub(crate) count_type: Rc<Datatype>,
    /// A signed result type when the original p-code output is signed.
    /// Keeping the conditional's arms signed preserves signed consumers such
    /// as `INT_SRIGHT` and `INT_SLESS` after the shift itself is made unsigned.
    pub(crate) signed_result_type: Option<Rc<Datatype>>,
}

/// Select a variable ARM `CPUI_INT_LEFT` whose inputs can safely appear twice
/// in its C conditional expression.
pub(crate) fn plan_arm_register_lsl(
    fd: &Funcdata,
    arch: &Architecture,
    op: OpId,
) -> Option<ArmRegisterLslPlan> {
    if !arch.archid.starts_with("ARM:") {
        return None;
    }
    let o = fd.obank().get(op)?;
    if o.code() != OpCode::CPUI_INT_LEFT || o.num_input() != 2 {
        return None;
    }
    let (lhs, count) = (o.get_in(0)?, o.get_in(1)?);
    let (lhs_v, count_v, out_v) = (
        fd.vbank().get(lhs)?,
        fd.vbank().get(count)?,
        fd.vbank().get(o.get_out()?)?,
    );
    if count_v.is_constant()
        || lhs_v.get_size() != 4
        || out_v.get_size() != 4
        || !operand_is_repeatable(fd, lhs, 16)
        || !operand_is_repeatable(fd, count, 16)
    {
        return None;
    }

    let lhs_type = arch
        .types()
        .get_base(lhs_v.get_size(), type_metatype::TYPE_UINT)
        .ok()?;
    let count_type = arch
        .types()
        .get_base(count_v.get_size(), type_metatype::TYPE_UINT)
        .ok()?;
    let signed_result_type = match out_v.get_type().get_metatype() {
        type_metatype::TYPE_INT | type_metatype::TYPE_ENUM_INT => Some(
            arch.types()
                .get_base(out_v.get_size(), type_metatype::TYPE_INT)
                .ok()?,
        ),
        _ => None,
    };

    Some(ArmRegisterLslPlan {
        lhs,
        count,
        lhs_type,
        count_type,
        signed_result_type,
    })
}

/// Whether printing `vn` twice emits the same side-effect-free C value.
///
/// A volatile leaf must be rejected even when it has no p-code definition:
/// persistent/global values can print as direct C reads. An explicit written
/// value is also safe as a leaf because its defining statement (including a
/// CALL or LOAD) has already materialized it once; implied values instead need
/// a recursive walk and may only contain the pure operations listed below.
fn operand_is_repeatable(fd: &Funcdata, vn: VarnodeId, depth: u8) -> bool {
    let v = match fd.vbank().get(vn) {
        Some(v) => v,
        None => return false,
    };
    if v.is_volatile() {
        return false;
    }
    if v.is_constant() || !v.is_written() {
        return true;
    }
    if v.is_explicit() && !v.is_implied() && v.get_def().is_some() {
        return true;
    }
    if depth == 0 {
        return false;
    }
    let def = match v.get_def().and_then(|op| fd.obank().get(op)) {
        Some(def) => def,
        None => return false,
    };
    if !matches!(
        def.code(),
        OpCode::CPUI_COPY
            | OpCode::CPUI_INT_EQUAL
            | OpCode::CPUI_INT_NOTEQUAL
            | OpCode::CPUI_INT_SLESS
            | OpCode::CPUI_INT_SLESSEQUAL
            | OpCode::CPUI_INT_LESS
            | OpCode::CPUI_INT_LESSEQUAL
            | OpCode::CPUI_INT_ZEXT
            | OpCode::CPUI_INT_SEXT
            | OpCode::CPUI_INT_ADD
            | OpCode::CPUI_INT_SUB
            | OpCode::CPUI_INT_CARRY
            | OpCode::CPUI_INT_SCARRY
            | OpCode::CPUI_INT_SBORROW
            | OpCode::CPUI_INT_2COMP
            | OpCode::CPUI_INT_NEGATE
            | OpCode::CPUI_INT_XOR
            | OpCode::CPUI_INT_AND
            | OpCode::CPUI_INT_OR
            | OpCode::CPUI_INT_LEFT
            | OpCode::CPUI_INT_RIGHT
            | OpCode::CPUI_INT_SRIGHT
            | OpCode::CPUI_INT_MULT
            | OpCode::CPUI_INT_DIV
            | OpCode::CPUI_INT_SDIV
            | OpCode::CPUI_INT_REM
            | OpCode::CPUI_INT_SREM
            | OpCode::CPUI_BOOL_NEGATE
            | OpCode::CPUI_BOOL_XOR
            | OpCode::CPUI_BOOL_AND
            | OpCode::CPUI_BOOL_OR
            | OpCode::CPUI_PIECE
            | OpCode::CPUI_SUBPIECE
            | OpCode::CPUI_PTRADD
            | OpCode::CPUI_PTRSUB
            | OpCode::CPUI_INSERT
            | OpCode::CPUI_ZPULL
            | OpCode::CPUI_POPCOUNT
            | OpCode::CPUI_LZCOUNT
            | OpCode::CPUI_SPULL
    ) {
        return false;
    }
    for slot in 0..def.num_input() {
        let input = match def.get_in(slot) {
            Some(input) => input,
            None => return false,
        };
        if !operand_is_repeatable(fd, input, depth - 1) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests;
