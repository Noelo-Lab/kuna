//! kuna fix `implicit_widening_arithmetic` (spec 10.16): a widening
//! conversion is written wherever leaving it out makes an operation compute
//! in a narrower type than the bytecode's.
//!
//! `SecondaryFunctionsHelper` leaves every widening conversion (`i2l`,
//! `i2f`, `i2d`, `l2f`, `l2d`, `f2d`) unwritten, since Java widens on its
//! own, and writes one back only for a `+ - * /` whose two operands are both
//! such conversions, or for a shift's left operand. Java widens an operand
//! only as far as the other operand's type: when that operand is itself
//! narrower as written (`-(long) a` written `-a`, a conversion under `%`,
//! `&` or a conditional), the operation runs in the narrower type and can
//! overflow (`-(long) size * n` came out `-size * n`, an `int` product), and
//! a negation alone differs at the type's minimum. With the fix, an
//! operation whose result depends on the type it computes in (`+ - * /`,
//! negation, shifts, and `%` of floating types) and whose operands, as
//! written, promote to a narrower type than its own gets the conversion
//! nearest its left operand written, as javac's source had it.

use crate::code_type::CodeType;
use crate::exprent::Exprent;
use crate::function_exprent::{FunctionExprent, FunctionType};

/// Java's binary numeric promotion of two written numeric types.
pub fn promoted(a: CodeType, b: CodeType) -> CodeType {
    for wide in [CodeType::Double, CodeType::Float, CodeType::Long] {
        if a == wide || b == wide {
            return wide;
        }
    }
    CodeType::Int
}

fn numeric(t: CodeType) -> bool {
    matches!(
        t,
        CodeType::Byte
            | CodeType::Short
            | CodeType::Char
            | CodeType::Int
            | CodeType::Long
            | CodeType::Float
            | CodeType::Double
    )
}

/// The primitive type of `expr` (`Object` for references and arrays).
pub fn own_type(expr: Exprent) -> CodeType {
    match expr.get_expr_type_opt() {
        Some(t) if t.array_dim == 0 => t.type_,
        _ => CodeType::Object,
    }
}

fn unwritten_conversion(f: FunctionExprent) -> bool {
    f.get_func_type().cast_type().is_some() && !f.does_cast()
}

/// The type `expr` has as Java reads its text: its own type, except where an
/// unwritten conversion inside it narrows what the operators compute in.
pub fn written_type(expr: Exprent) -> CodeType {
    use FunctionType::*;
    let own = own_type(expr);
    let Some(f) = expr.try_as_function() else {
        return own;
    };
    let ops = f.get_lst_operands();
    if unwritten_conversion(f) {
        return written_type(ops.get(0));
    }
    if !numeric(own) {
        return own;
    }
    match f.get_func_type() {
        Neg | BitNot | Shl | Shr | Ushr => promoted(written_type(ops.get(0)), CodeType::Int),
        Add | Sub | Mul | Div | Rem | And | Or | Xor => {
            promoted(written_type(ops.get(0)), written_type(ops.get(1)))
        }
        Ternary => {
            let (a, b) = (written_type(ops.get(1)), written_type(ops.get(2)));
            if a == b {
                a
            } else if numeric(a) && numeric(b) {
                promoted(a, b)
            } else {
                own
            }
        }
        _ => own,
    }
}

/// Writes the unwritten conversion nearest the left of `expr` that keeps its
/// written type below its own; whether there was one.
fn widen(expr: Exprent) -> bool {
    use FunctionType::*;
    let Some(f) = expr.try_as_function() else {
        return false;
    };
    let ops = f.get_lst_operands();
    if unwritten_conversion(f) {
        f.set_needs_cast(true);
        return true;
    }
    let narrow = |e: Exprent| written_type(e) != own_type(e);
    match f.get_func_type() {
        Neg | BitNot | Shl | Shr | Ushr => widen(ops.get(0)),
        Add | Sub | Mul | Div | Rem | And | Or | Xor => {
            (narrow(ops.get(0)) && widen(ops.get(0))) || (narrow(ops.get(1)) && widen(ops.get(1)))
        }
        Ternary => {
            (narrow(ops.get(1)) && widen(ops.get(1))) || (narrow(ops.get(2)) && widen(ops.get(2)))
        }
        _ => false,
    }
}

/// Whether `func`'s result depends on the type it computes in.
fn type_sensitive(func: FunctionType, own: CodeType) -> bool {
    use FunctionType::*;
    match func {
        Add | Sub | Mul | Div | Neg | Shl | Shr | Ushr => true,
        Rem => matches!(own, CodeType::Float | CodeType::Double),
        _ => false,
    }
}

/// Run on `fexpr` after its operands: writes the conversion that makes it
/// compute in its own type, if its written operands promote to a narrower one.
pub fn keep_widening(fexpr: FunctionExprent) {
    let own = own_type(fexpr.0);
    if !numeric(own) || !type_sensitive(fexpr.get_func_type(), own) {
        return;
    }
    if written_type(fexpr.0) != own {
        widen(fexpr.0);
    }
}
