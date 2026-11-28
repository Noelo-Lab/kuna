//! Fix `implicit_widening_arithmetic`: an operation whose operands, as written,
//! promote to a narrower type than its own gets the widening conversion nearest
//! its left operand written; a variable assigned an integer operation's result
//! with no narrowing conversion is at least an `int`. A comparison computes in
//! the type its two operands have, and a conditional in the type of its arms:
//! `(double) f == i` written `f == i` compares two `float`s, the `int` rounded,
//! and `b ? (float) l : (double) l` written `b ? (float) l : l` rounds the
//! second arm to a `float` too.

use crate::code_type::CodeType;
use crate::exprent::Exprent;
use crate::fixes::util::promoted;
use crate::function_exprent::{FunctionExprent, FunctionType};
use crate::type_family::TypeFamily;

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

/// Whether `expr` is an integer operation (`+`, `&`, `<<`, unary `-`, ...):
/// its value is an `int`, which javac stores into a `byte`, `short` or `char`
/// only through a narrowing conversion (`i2b`, `i2s`, `i2c`).
pub fn computes_int(expr: Exprent) -> bool {
    use FunctionType::*;
    expr.try_as_function().is_some_and(|f| {
        matches!(
            f.get_func_type(),
            Add | Sub | Mul | Div | Rem | And | Or | Xor | Shl | Shr | Ushr | Neg | BitNot
        ) && f
            .get_lst_operands()
            .iter()
            .all(|o| o.get_expr_type_opt().is_some_and(|t| t.type_family == TypeFamily::Integer))
    })
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

/// Writes the conversion that makes the comparison `fexpr` compare in the
/// floating-point type its operands have, if as written they promote to
/// another type (an `int` or `long` operand is then rounded to a `float`
/// where the bytecode converts it to a `double`, or the reverse).
fn keep_compared(fexpr: FunctionExprent) {
    let ops = fexpr.get_lst_operands();
    if ops.len() != 2 {
        return;
    }
    let (left, right) = (ops.get(0), ops.get(1));
    let own = own_type(left);
    if own != own_type(right) || !matches!(own, CodeType::Float | CodeType::Double) {
        return;
    }
    if promoted(written_type(left), written_type(right)) != own {
        let narrow = |e: Exprent| written_type(e) != own_type(e);
        let _ = (narrow(left) && widen(left)) || (narrow(right) && widen(right));
    }
}

/// Writes the conversion that makes the `double` conditional `fexpr` one as
/// written, if its arms as written promote to `float` and one of them is an
/// `int` or a `long`, which that promotion would round where the bytecode
/// converts it to a `double`.
fn keep_conditional(fexpr: FunctionExprent) {
    let ops = fexpr.get_lst_operands();
    if ops.len() != 3 || own_type(fexpr.0) != CodeType::Double {
        return;
    }
    let arms = [written_type(ops.get(1)), written_type(ops.get(2))];
    if promoted(arms[0], arms[1]) == CodeType::Float && arms.iter().any(|t| matches!(t, CodeType::Int | CodeType::Long)) {
        widen(fexpr.0);
    }
}

/// Run on `fexpr` after its operands: writes the conversion that makes it
/// compute in its own type, if its written operands promote to a narrower one.
pub fn keep_widening(fexpr: FunctionExprent) {
    use FunctionType::*;
    match fexpr.get_func_type() {
        Eq | Ne | Lt | Ge | Gt | Le => return keep_compared(fexpr),
        Ternary => return keep_conditional(fexpr),
        _ => {}
    }
    let own = own_type(fexpr.0);
    if !numeric(own) || !type_sensitive(fexpr.get_func_type(), own) {
        return;
    }
    if written_type(fexpr.0) != own {
        widen(fexpr.0);
    }
}
