//! A switch on an expression narrower than `int`.
//!
//! The p-code computes a one- or two-byte selector at its own width, so a
//! byte `v1 + 1` is 0 when `v1` is 255. C promotes the operands to `int`
//! first, and `switch(v1 + 1)` switches on 256, which no byte label matches.
//! The upstream integer-promotion checks cast such an expression where an
//! extension or a comparison reads it, but a `BRANCHIND` reads it as it is.
//! The selector prints cast to its own type, `switch((unsigned char)(v1 + 1))`,
//! unless `intPromotionType` shows its promoted value extends the way that
//! type does.

use std::rc::Rc;

use crate::architecture::Architecture;
use crate::cast::{CastContext, CastStrategy, CastStrategyC, IntPromotionCode, VnRef};
use crate::context::OpId;
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;

/// The type the selector of the `BRANCHIND` `op`, Varnode `vn` behind `vr`,
/// prints cast to, or `None`.
pub(crate) fn cast(
    fd: &Funcdata,
    arch: &Architecture,
    strat: &CastStrategyC,
    ctx: &dyn CastContext,
    vr: VnRef,
    op: OpId,
) -> Option<Rc<Datatype>> {
    let vn = fd.obank().get(op)?.get_in(0)?;
    let v = fd.vbank().get(vn)?;
    if v.is_constant() || v.is_explicit() || !v.is_written() || v.get_size() >= strat.promote_size() {
        return None;
    }
    let t = fd.vn_type_read_facing(vn, op);
    if t.is_enum_type() {
        return None;
    }
    let natural = match t.get_metatype() {
        type_metatype::TYPE_INT => IntPromotionCode::SIGNED_EXTENSION,
        type_metatype::TYPE_UINT | type_metatype::TYPE_UNKNOWN => IntPromotionCode::UNSIGNED_EXTENSION,
        _ => return None,
    };
    let ext = strat.int_promotion_type(ctx, vr);
    if ext == IntPromotionCode::NO_PROMOTION
        || (ext != IntPromotionCode::UNKNOWN_PROMOTION && ext.as_i32() & natural.as_i32() != 0)
    {
        return None;
    }
    if t.get_metatype() == type_metatype::TYPE_UNKNOWN {
        return arch.types().get_base(v.get_size(), type_metatype::TYPE_UINT).ok();
    }
    Some(t)
}
