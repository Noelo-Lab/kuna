//! (kuna) Keep a narrowed call argument's truncation in the C.
//!
//! Dead-code analysis counts only the possibly-nonzero bits of a call input as
//! consumed, so when a caller zero-extends a sub-`int` value into the argument
//! register (`movzwl %di,%edi`) the subvariable-flow rules trim the argument to
//! its narrow value: `RDI = ZEXT(x:2)` becomes the 2-byte `x`.  The trimmed p-code
//! is right, but C promotes a sub-`int` expression to `int` before it becomes an
//! argument.  `sink(a0 * 3)` with a 2-byte `a0` passes up to seventeen significant
//! bits, or a negative number, where the binary passed `zext16(a0 * 3)`; and the
//! callee's own definition reads the whole register (`void sink(unsigned int)`),
//! so nothing in the printed program truncates the value again.  Upstream Ghidra
//! prints the same promoted form.
//!
//! A trimmed argument therefore gets an explicit cast to the unsigned integer of
//! its own size whenever its C value after promotion is not already that
//! zero-extension: an arithmetic expression promotion can widen, or a signed
//! variable promotion sign-extends.  Zero-extension is the right conversion for
//! every trimmed input, because the bits the trim dropped were either known zero
//! or never read by the callee.  A slot whose parameter type a declared prototype
//! locks is left alone: C already converts the argument to that narrow type.

use std::rc::Rc;

use kuna_base::types::{int4, uint4};
use kuna_num::opcodes::OpCode;

use crate::cast::{CastStrategy, CastStrategyC, IntPromotionCode};
use crate::context::{OpId, VarnodeId};
use crate::coreaction_casts::FuncdataCastContext;
use crate::dtype::{type_metatype, Datatype, TypeFactory};
use crate::funcdata::Funcdata;

/// The cast a narrowed argument in `slot` of the CALL/CALLIND `op` needs so its C
/// value matches the zero-extended value the binary passes, or `None`.
pub(crate) fn narrowed_arg_cast(
    data: &mut Funcdata,
    strat: &CastStrategyC,
    op: OpId,
    slot: int4,
) -> Option<Rc<Datatype>> {
    if slot < 1 {
        return None;
    }
    let o = data.obank().get(op)?;
    if !matches!(o.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND) {
        return None;
    }
    let invn = o.get_in(slot)?;
    let vn = data.vbank().get(invn)?;
    let size = vn.get_size();
    if size >= strat.promote_size() || vn.is_annotation() {
        return None;
    }
    if declared_param(data, op, slot) {
        return None;
    }
    let curtype = data.vn_high_type_read_facing(invn, op);
    let meta = curtype.get_metatype();
    if !matches!(
        meta,
        type_metatype::TYPE_INT | type_metatype::TYPE_UINT | type_metatype::TYPE_UNKNOWN
    ) || curtype.is_enum_type()
    {
        return None;
    }
    if promotes_as_zext(data, strat, op, invn, meta) {
        return None;
    }
    let tlst = data.get_arch().types_rc()?;
    let ct = tlst.get_base(size, type_metatype::TYPE_UINT).ok()?;
    if retype_truncation(data, strat, op, invn, &ct) {
        return None;
    }
    Some(ct)
}

/// Retype an argument that already prints as a truncating cast (`(char)v2`) to
/// `ct`, so it reads `(unsigned char)v2` rather than stacking a second cast.
fn retype_truncation(
    data: &mut Funcdata,
    strat: &CastStrategyC,
    op: OpId,
    invn: VarnodeId,
    ct: &Rc<Datatype>,
) -> bool {
    let Some(vn) = data.vbank().get(invn) else {
        return false;
    };
    if !vn.is_implied() || data.lone_descend(invn) != Some(op) {
        return false;
    }
    let Some(def) = vn.get_def() else {
        return false;
    };
    let Some(d) = data.obank().get(def) else {
        return false;
    };
    if d.code() != OpCode::CPUI_SUBPIECE || d.does_special_printing() {
        return false;
    }
    let (Some(in0), Some(in1)) = (d.get_in(0), d.get_in(1)) else {
        return false;
    };
    let off = data.vbank().get(in1).map(|v| v.get_offset()).unwrap_or(0) as uint4;
    let intype = data.vn_high_type_read_facing(in0, def);
    let outtype = data.vn_high_type_def_facing(invn);
    if !strat.is_subpiece_cast(&outtype, &intype, off) || !strat.is_subpiece_cast(ct, &intype, off) {
        return false;
    }
    data.vn_update_type(invn, Rc::clone(ct));
    data.vbank().get(invn).is_some_and(|v| Rc::ptr_eq(v.get_type(), ct))
}

/// Is parameter `slot` of the call fixed by a declared prototype?
fn declared_param(data: &Funcdata, op: OpId, slot: int4) -> bool {
    let Some(i) = data.get_call_specs_index(op) else {
        return false;
    };
    let fc = data.get_call_specs(i);
    match fc.proto().get_param(slot - 1) {
        Some(p) => p.is_type_locked(),
        None => fc.is_input_locked() && !fc.is_dotdotdot(),
    }
}

/// Does C's integer promotion of the argument expression already produce the
/// zero-extension of its p-code value?
fn promotes_as_zext(
    data: &mut Funcdata,
    strat: &CastStrategyC,
    op: OpId,
    invn: VarnodeId,
    meta: type_metatype,
) -> bool {
    use IntPromotionCode::*;
    let def_code = data
        .vbank()
        .get(invn)
        .and_then(|v| v.get_def())
        .and_then(|d| data.obank().get(d))
        .map(|d| d.code());
    if def_code == Some(OpCode::CPUI_INT_ZEXT) {
        return true;
    }
    let ctx = FuncdataCastContext::new(data);
    let vr = ctx.vn_ref(invn);
    let mut ext = strat.int_promotion_type(&ctx, vr);
    if ext == NO_PROMOTION {
        ext = strat.local_extension_type(&ctx, vr, Some(ctx.op_ref(op)));
        if ext == UNKNOWN_PROMOTION {
            ext = if meta == type_metatype::TYPE_INT {
                SIGNED_EXTENSION
            } else {
                UNSIGNED_EXTENSION
            };
        }
    }
    (ext.as_i32() & UNSIGNED_EXTENSION.as_i32()) != 0
}
