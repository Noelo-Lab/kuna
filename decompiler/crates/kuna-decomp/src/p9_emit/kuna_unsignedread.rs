//! An unsigned read of an expression C evaluates as signed.
//!
//! C promotes an operand narrower than `int` to `int` before arithmetic, so
//! `(*a1 ^ 0x80) - 0x80` over an `unsigned short *a1` is an `int` in C, and so
//! is `(int)a0 + 1`. The Varnode such an expression computes can still be typed
//! `unsigned int`: `arithmeticOutputStandard` takes the most ordered operand
//! type and knows no promotion. A reader that treats its input as unsigned --
//! a zero extension, a logical right shift, an unsigned compare, divide or
//! remainder -- checks that unsigned type, finds nothing to cast, and the
//! printed C reads the value as signed: `(unsigned long)((*a1 ^ 0x80) - 0x80)`
//! sign-extends where the machine zero-extends. Such a read makes the
//! expression unsigned: a plain constant operand of its top operation takes
//! a `U` suffix (`v + 1U`), and otherwise the expression is cast to the
//! unsigned integer of its size. Neither happens when the value cannot be
//! negative (its nonzero-bit mask clears the top bit) or, for a compare,
//! divide or remainder, when the other operand already makes C convert it.
//!
//! The C type of the printed expression is computed bottom-up: a constant from
//! its literal, a variable, a cast or a load from the type the text states, an
//! extension from its input when C performs it implicitly, and arithmetic by
//! C's usual conversions over the promoted operands. Only expressions at least
//! as wide as `int` are considered; a narrower one is cast by the upstream
//! integer-promotion checks.

use std::rc::Rc;

use kuna_base::address::signbit_negative;
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::cast::{CastStrategy, CastStrategyC};
use crate::context::{OpId, VarnodeId};
use crate::coreaction_casts::FuncdataCastContext;
use crate::dtype::{type_metatype, Datatype, TypeFactory};
use crate::funcdata::Funcdata;

const DEPTH: u32 = 8;

/// A C integer type after promotion: `(unsigned, bytes)`.
type CInt = (bool, int4);

/// The unsigned type input `slot` of `op` must be cast to because the
/// expression there is a signed `int` or `long` in C, or `None`.
pub(crate) fn cast(data: &mut Funcdata, strat: &CastStrategyC, op: OpId, slot: int4) -> Option<Rc<Datatype>> {
    let o = data.obank().get(op)?;
    let binary = match o.code() {
        OpCode::CPUI_INT_ZEXT | OpCode::CPUI_INT_RIGHT if slot == 0 => false,
        OpCode::CPUI_INT_LESS | OpCode::CPUI_INT_LESSEQUAL | OpCode::CPUI_INT_DIV | OpCode::CPUI_INT_REM => true,
        _ => return None,
    };
    let vn = o.get_in(slot)?;
    let other = if binary { o.get_in(1 - slot) } else { None };
    let v = data.vbank().get(vn)?;
    if v.is_constant() || v.is_explicit() || !v.is_written() {
        return None;
    }
    let size = v.get_size();
    if size < strat.promote_size() || size > 8 || !signbit_negative(v.get_nz_mask(), size) {
        return None;
    }
    let ir = data.vn_type_read_facing(vn, op);
    if !matches!(ir.get_metatype(), type_metatype::TYPE_UINT | type_metatype::TYPE_UNKNOWN) {
        return None;
    }
    if c_int(data, strat, vn, op, DEPTH)? != (false, size) {
        return None;
    }
    if let Some(other) = other {
        if c_int(data, strat, other, op, DEPTH).is_some_and(|(unsigned, width)| unsigned && width >= size) {
            return None;
        }
    }
    if let Some(c) = suffixable_literal(data, vn) {
        if let Some(cv) = data.vbank_mut().get_mut(c) {
            cv.set_unsigned_print();
            if size > 4 {
                cv.set_long_print();
            }
            return None;
        }
    }
    data.get_arch().types_rc()?.get_base(size, type_metatype::TYPE_UINT).ok()
}

/// A constant operand of the operation that computes the implied `vn` whose
/// `U` suffix makes the whole expression unsigned (`v + 1U`): an operand of
/// arithmetic whose result bits do not depend on the sign it computes with
/// (a division, a remainder or a shift would change meaning), printed as a
/// plain non-negative integer.
fn suffixable_literal(data: &Funcdata, vn: VarnodeId) -> Option<VarnodeId> {
    let def = data.vbank().get(vn)?.get_def()?;
    let d = data.obank().get(def)?;
    if !matches!(
        d.code(),
        OpCode::CPUI_INT_ADD
            | OpCode::CPUI_INT_SUB
            | OpCode::CPUI_INT_MULT
            | OpCode::CPUI_INT_AND
            | OpCode::CPUI_INT_OR
            | OpCode::CPUI_INT_XOR
            | OpCode::CPUI_INT_NEGATE
            | OpCode::CPUI_INT_2COMP
    ) {
        return None;
    }
    (0..d.num_input()).filter_map(|i| d.get_in(i)).find(|&c| {
        data.vbank().get(c).is_some_and(|cv| cv.is_constant() && !cv.is_annotation()) && {
            let t = data.vn_type_read_facing(c, def);
            matches!(t.get_metatype(), type_metatype::TYPE_UINT | type_metatype::TYPE_UNKNOWN)
                && !t.is_char_print()
                && !t.is_enum_type()
                && data.vn_high_display_format(c) == 0
        }
    })
}

/// The promoted C type of `vn` as printed where `reader` reads it, or `None`
/// when the text does not settle it.
fn c_int(data: &mut Funcdata, strat: &CastStrategyC, vn: VarnodeId, reader: OpId, depth: u32) -> Option<CInt> {
    let v = data.vbank().get(vn)?;
    if v.is_annotation() || depth == 0 {
        return None;
    }
    if v.is_constant() {
        return literal(data, vn, reader);
    }
    if v.is_explicit() || !v.is_written() {
        return declared(&data.vn_type_read_facing(vn, reader));
    }
    let def = v.get_def()?;
    let d = data.obank().get(def)?;
    let code = d.code();
    let ins: Vec<VarnodeId> = (0..d.num_input()).filter_map(|i| d.get_in(i)).collect();
    let info = crate::typeop::type_op_info(code);
    match code {
        OpCode::CPUI_COPY => c_int(data, strat, *ins.first()?, def, depth - 1),
        OpCode::CPUI_INT_ZEXT | OpCode::CPUI_INT_SEXT => {
            let outtype = data.vn_type_def_facing(vn);
            let intype = data.vn_type_read_facing(*ins.first()?, def);
            let cast = if code == OpCode::CPUI_INT_ZEXT {
                strat.is_zext_cast(&outtype, &intype)
            } else {
                strat.is_sext_cast(&outtype, &intype)
            };
            if !cast {
                return None;
            }
            let hidden = {
                let ctx = FuncdataCastContext::new(data);
                let (d, r) = (ctx.op_ref(def), ctx.op_ref(reader));
                strat.is_extension_cast_implied(&ctx, d, Some(r))
            };
            if hidden {
                c_int(data, strat, ins[0], def, depth - 1)
            } else {
                declared(&outtype)
            }
        }
        _ if d.is_bool_output() => Some((false, 4)),
        _ if info.inherits_sign() => {
            let n = if info.inherits_sign_first_param_only() { 1 } else { ins.len() };
            let mut acc: Option<CInt> = None;
            for &i in &ins[..n.min(ins.len())] {
                let (unsigned, width) = c_int(data, strat, i, def, depth - 1)?;
                acc = Some(match acc {
                    None => (unsigned, width),
                    Some((au, aw)) if aw == width => (au || unsigned, width),
                    Some((au, aw)) if aw > width => (au, aw),
                    Some(_) => (unsigned, width),
                });
            }
            acc
        }
        _ => declared(&data.vn_type_def_facing(vn)),
    }
}

/// The promoted C type of a value the text declares as `t`.
fn declared(t: &Datatype) -> Option<CInt> {
    if t.is_enum_type() {
        return None;
    }
    let size = t.get_size();
    match t.get_metatype() {
        type_metatype::TYPE_BOOL => Some((false, 4)),
        _ if size < 4 && size > 0 => {
            matches!(t.get_metatype(), type_metatype::TYPE_INT | type_metatype::TYPE_UINT | type_metatype::TYPE_UNKNOWN)
                .then_some((false, 4))
        }
        type_metatype::TYPE_INT if size == 4 || size == 8 => Some((false, size)),
        type_metatype::TYPE_UINT | type_metatype::TYPE_UNKNOWN if size == 4 || size == 8 => Some((true, size)),
        _ => None,
    }
}

/// The C type of the literal the constant `vn` prints as where `reader` reads
/// it: a `U` or `L` suffix says so, a character is an `int`, and a
/// hexadecimal magnitude takes the first of `int`, `unsigned int`, `long`,
/// `unsigned long` that holds it (a leading minus keeps that type).
fn literal(data: &Funcdata, vn: VarnodeId, reader: OpId) -> Option<CInt> {
    let v = data.vbank().get(vn)?;
    let t = data.vn_type_read_facing(vn, reader);
    if t.is_enum_type() {
        return None;
    }
    let (size, off) = (v.get_size(), v.get_offset());
    let magnitude = match t.get_metatype() {
        type_metatype::TYPE_BOOL => return Some((false, 4)),
        _ if t.is_char_print() => return Some((false, 4)),
        type_metatype::TYPE_INT if signbit_negative(off, size) => kuna_base::address::uintb_negate(off, size),
        type_metatype::TYPE_INT | type_metatype::TYPE_UINT | type_metatype::TYPE_UNKNOWN => off,
        _ => return None,
    };
    let long = v.is_long_print() || magnitude > 0xffff_ffff;
    if v.is_unsigned_print() {
        return Some((true, if long { 8 } else { 4 }));
    }
    Some(match (long, magnitude) {
        (false, x) if x <= 0x7fff_ffff => (false, 4),
        (false, _) => (true, 4),
        (true, x) if x <= 0x7fff_ffff_ffff_ffff => (false, 8),
        (true, _) => (true, 8),
    })
}
