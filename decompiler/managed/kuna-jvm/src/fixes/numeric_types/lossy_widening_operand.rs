//! Fix `lossy_widening_operand`: a widening conversion that rounds (`int` or
//! `long` to `float`, `long` to `double`) stays written under another
//! conversion, which would otherwise convert its operand directly.

use crate::code_type::CodeType;
use crate::function_exprent::{FunctionExprent, FunctionType};

/// Run on the conversion `fexpr` after its operands: writes its operand's
/// conversion when that one rounds, since `(long) (float) i` written
/// `(long) i` converts `i` to `long` without rounding it to a `float`.
pub fn keep_operand(fexpr: FunctionExprent) {
    use FunctionType::*;
    if fexpr.get_func_type().cast_type().is_none() {
        return;
    }
    let Some(inner) = fexpr.get_lst_operands().get(0).try_as_function() else {
        return;
    };
    let exact = match inner.get_func_type() {
        I2f => inner.get_lst_operands().get(0).get_expr_type_opt().is_some_and(|t| {
            t.array_dim == 0 && matches!(t.type_, CodeType::Byte | CodeType::Short | CodeType::Char)
        }),
        L2f | L2d => false,
        _ => true,
    };
    if !exact && !inner.does_cast() {
        inner.set_needs_cast(true);
    }
}
