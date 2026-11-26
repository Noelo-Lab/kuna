//! Preserve a narrow machine product when C would promote its operands to signed `int`.

use crate::architecture::Architecture;
use crate::context::{OpId, VarnodeId};
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;
use kuna_num::opcodes::OpCode;
use std::rc::Rc;

pub(super) struct NarrowMultiplyPlan {
    pub lhs: VarnodeId,
    pub rhs: VarnodeId,
    pub unsigned: Rc<Datatype>,
    pub result: Rc<Datatype>,
}

pub(super) fn plan(
    data: &Funcdata,
    arch: &Architecture,
    id: OpId,
    printed: impl Fn(VarnodeId) -> Rc<Datatype>,
) -> Option<NarrowMultiplyPlan> {
    let op = data.obank().get(id)?;
    if op.code() != OpCode::CPUI_INT_MULT {
        return None;
    }
    let out = data.vbank().get(op.get_out()?)?;
    let size = out.get_size();
    let types = arch.types();
    let int_width = types.get_size_of_int();
    if !matches!(int_width, 2 | 4 | 8) || !matches!(size, 1 | 2) {
        return None;
    }
    let signed_width = [int_width.min(4), 4].into_iter().filter(|&width| size < width).min()?;
    let mask = (1u64 << (size * 8)) - 1;
    let (lhs, rhs) = (op.get_in(0)?, op.get_in(1)?);
    let (left, right) = (data.vbank().get(lhs)?, data.vbank().get(rhs)?);
    let int_max = (1u64 << (signed_width * 8 - 1)) - 1;
    if left.is_unsigned_print()
        || right.is_unsigned_print()
        || (left.get_nz_mask() & mask).saturating_mul(right.get_nz_mask() & mask) <= int_max
    {
        return None;
    }
    if ![lhs, rhs].into_iter().all(|vn| {
        let ty = printed(vn);
        ty.get_size() == size && ty.get_metatype() == type_metatype::TYPE_UINT && !ty.is_enum_type()
    }) {
        return None;
    }
    let result = match out.get_type_def_facing().get_metatype() {
        type_metatype::TYPE_INT => type_metatype::TYPE_INT,
        type_metatype::TYPE_UINT | type_metatype::TYPE_UNKNOWN => type_metatype::TYPE_UINT,
        _ => return None,
    };
    Some(NarrowMultiplyPlan {
        lhs,
        rhs,
        unsigned: types.get_base(int_width.max(4), type_metatype::TYPE_UINT).ok()?,
        result: types.get_base(size, result).ok()?,
    })
}

/// The product `vn` when a C cast to `ty` printed around it already truncates it to its machine width.
pub(super) fn truncating_cast(data: &Funcdata, vn: VarnodeId, ty: &Datatype) -> Option<OpId> {
    let v = data.vbank().get(vn)?;
    if !v.is_implied()
        || v.has_implied_field()
        || ty.get_size() != v.get_size()
        || ty.is_enum_type()
        || !matches!(ty.get_metatype(), type_metatype::TYPE_INT | type_metatype::TYPE_UINT)
    {
        return None;
    }
    let def = v.get_def()?;
    (data.obank().get(def)?.code() == OpCode::CPUI_INT_MULT).then_some(def)
}
