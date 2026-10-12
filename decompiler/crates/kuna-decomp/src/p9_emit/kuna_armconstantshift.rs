//! Defined C for a signed operand of a fixed, in-range 32-bit ARM LSL.

use std::rc::Rc;

use crate::architecture::Architecture;
use crate::context::{OpId, VarnodeId};
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;
use kuna_num::opcodes::OpCode;

pub(crate) struct ArmConstantLslPlan {
    pub(crate) lhs: VarnodeId,
    pub(crate) count: VarnodeId,
    pub(crate) unsigned_type: Rc<Datatype>,
    pub(crate) signed_result_type: Option<Rc<Datatype>>,
}

/// Convert before shifting to compute ARM's low 32 bits without signed C UB.
/// Restore the output's signed type so implied signed consumers keep their sign.
pub(crate) fn plan_arm_constant_lsl(
    fd: &Funcdata,
    arch: &Architecture,
    op: OpId,
) -> Option<ArmConstantLslPlan> {
    if !arch.arm_constant_shift || !arch.archid.starts_with("ARM:") {
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
    if !count_v.is_constant()
        || count_v.get_offset() >= 32
        || lhs_v.get_size() != 4
        || out_v.get_size() != 4
        || !signed_shiftee(fd, arch, lhs, op)
    {
        return None;
    }
    let unsigned_type = arch.types().get_base(4, type_metatype::TYPE_UINT).ok()?;
    let signed_result_type = match out_v.get_type_def_facing().get_metatype() {
        type_metatype::TYPE_INT | type_metatype::TYPE_ENUM_INT => {
            Some(arch.types().get_base(4, type_metatype::TYPE_INT).ok()?)
        }
        _ => None,
    };
    Some(ArmConstantLslPlan {
        lhs,
        count,
        unsigned_type,
        signed_result_type,
    })
}

/// A volatile annotation prints its mapped symbol's type, not the userop's
/// unsigned output metadata. Accept only a whole four-byte signed symbol.
fn signed_shiftee(fd: &Funcdata, arch: &Architecture, lhs: VarnodeId, reader: OpId) -> bool {
    let Some(v) = fd.vbank().get(lhs) else {
        return false;
    };
    let signed = |t: &Datatype| {
        matches!(
            t.get_metatype(),
            type_metatype::TYPE_INT | type_metatype::TYPE_ENUM_INT
        )
    };
    if signed(v.get_type_read_facing(reader)) {
        return true;
    }
    if !v.is_implied() {
        return false;
    }
    let Some(def) = v.get_def().and_then(|op| fd.obank().get(op)) else {
        return false;
    };
    if def.code() != OpCode::CPUI_CALLOTHER || def.num_input() != 2 {
        return false;
    }
    let Some(index) = def.get_in(0).and_then(|vn| fd.vbank().get(vn)) else {
        return false;
    };
    if !arch
        .userops
        .get_op(index.get_offset() as u32)
        .is_some_and(|u| u.get_display() == crate::userop::userop_flags::NO_OPERATOR)
    {
        return false;
    }
    let Some(annotation) = def.get_in(1).and_then(|vn| fd.vbank().get(vn)) else {
        return false;
    };
    if !annotation.is_annotation() {
        return false;
    }
    let Some((_, offset, Some(ty))) =
        fd.get_arch()
            .name_for_global_varnode(annotation.get_addr(), 4, def.get_addr())
    else {
        return false;
    };
    offset == 0 && ty.get_size() == 4 && signed(&ty)
}
