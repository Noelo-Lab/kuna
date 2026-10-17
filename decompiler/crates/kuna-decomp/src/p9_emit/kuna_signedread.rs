//! A signed read of an expression that prints as unsigned.
//!
//! `CastStrategy::markExplicitUnsigned` prints a constant with a `U` suffix when
//! the constant is unsigned and the other operand is not, and an explicit
//! `unsigned int` variable keeps its own type inside an expression. Either one
//! makes the C expression unsigned (`a0 - 1U`, `v2 + 1`) while the Varnode it
//! computes keeps the signed type the rest of the function gave it. A reader
//! that treats its input as signed -- a sign extension, an arithmetic right
//! shift, a signed compare, divide or remainder, an integer-to-float
//! conversion -- checks that signed type, adds no cast, and the printed C reads
//! the value as unsigned: `(long)(a0 - 1U)` zero-extends where the machine
//! sign-extends. Such a read casts the expression back to the signed integer
//! of its size, `(long)(int)(a0 - 1U)`.

use std::rc::Rc;

use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::context::{OpId, VarnodeId};
use crate::dtype::{type_metatype, Datatype, TypeFactory};
use crate::funcdata::Funcdata;

/// The signed type input `slot` of `op` must be cast to because the expression
/// there prints as unsigned, or `None`.
pub(crate) fn cast(data: &mut Funcdata, op: OpId, slot: int4) -> Option<Rc<Datatype>> {
    let o = data.obank().get(op)?;
    let signed = match o.code() {
        OpCode::CPUI_INT_SEXT | OpCode::CPUI_INT_SRIGHT | OpCode::CPUI_FLOAT_INT2FLOAT => slot == 0,
        OpCode::CPUI_INT_SLESS | OpCode::CPUI_INT_SLESSEQUAL | OpCode::CPUI_INT_SDIV | OpCode::CPUI_INT_SREM => true,
        _ => false,
    };
    if !signed {
        return None;
    }
    let vn = o.get_in(slot)?;
    let v = data.vbank().get(vn)?;
    if v.is_constant() || v.is_explicit() || !v.is_written() {
        return None;
    }
    let size = v.get_size();
    let def = v.get_def()?;
    if !arithmetic_prints_unsigned(data, def, 8) {
        return None;
    }
    data.get_arch().types_rc()?.get_base(size, type_metatype::TYPE_INT).ok()
}

/// Does the implied expression `def` computes print as an unsigned C value: is
/// it an operation whose result takes the sign of its operands, with an
/// operand that prints unsigned at the expression's own width or wider? A
/// narrower unsigned operand (`0x80U` beside a `long`) converts to the signed
/// wider type.
fn arithmetic_prints_unsigned(data: &mut Funcdata, def: OpId, depth: u32) -> bool {
    let Some(o) = data.obank().get(def) else { return false };
    let info = crate::typeop::type_op_info(o.code());
    if !info.inherits_sign() {
        return false;
    }
    let Some(width) = o.get_out().and_then(|out| data.vbank().get(out)).map(|v| v.get_size().max(4)) else {
        return false;
    };
    let n = if info.inherits_sign_first_param_only() { 1 } else { o.num_input() };
    let ins: Vec<VarnodeId> = (0..n).filter_map(|i| o.get_in(i)).collect();
    ins.into_iter().any(|vn| operand_prints_unsigned(data, vn, def, width, depth))
}

/// Does operand `vn` of `reader` print as an unsigned value at least `width`
/// bytes wide: a `U` constant (an `unsigned int` literal unless its value needs
/// more), a variable declared `unsigned` or `undefinedN`, an unsigned
/// arithmetic expression, or another expression of unsigned type?
fn operand_prints_unsigned(data: &mut Funcdata, vn: VarnodeId, reader: OpId, width: int4, depth: u32) -> bool {
    let Some(v) = data.vbank().get(vn) else { return false };
    if v.is_constant() {
        let literal = if v.get_offset() <= 0xffff_ffff { 4 } else { 8 };
        return v.is_unsigned_print() && literal >= width;
    }
    let implied = v.is_written() && !v.is_explicit();
    if implied {
        let def = v.get_def().expect("written varnode has a def");
        let inherits = data.obank().get(def).is_some_and(|d| crate::typeop::type_op_info(d.code()).inherits_sign());
        if inherits {
            return depth > 0 && arithmetic_prints_unsigned(data, def, depth - 1);
        }
    }
    let ty = data.vn_high_type_read_facing(vn, reader);
    let unsigned = match ty.get_metatype() {
        type_metatype::TYPE_UINT => true,
        type_metatype::TYPE_UNKNOWN => !implied,
        _ => false,
    };
    unsigned && ty.get_size() >= width
}
