//! kuna fix `mixed_type_reassociation` (spec 10.16): the parentheses of a
//! `+` or `*` written as the right operand of the same operator stay where
//! regrouping would change the type a step computes in.
//!
//! `FunctionExprent.wrapOperandString` leaves out the parentheses of
//! `a * (b * c)` for an associative operator that is not on `float` or
//! `double`, which writes `a * b * c`, read `(a * b) * c`. Integer `+` and
//! `*` regroup freely only while every step computes in the same type: with
//! `a` and `b` written as `int` (an unwritten widening) and `c` a `long`,
//! `(long) ord * (size * 2L)` came out `ord * size * 2L`, whose `ord * size`
//! is an `int` product that overflows. With the fix the parentheses stay
//! unless the left operand and the first operand of the regrouped group,
//! as written, already promote to the operator's own type.

use crate::exprent::Exprent;
use crate::function_exprent::{FunctionExprent, FunctionType};
use crate::p3_dataflow::kuna_fix_implicit_widening_arithmetic::{own_type, promoted, written_type};

/// Whether writing `right`, the right operand of `parent` and the same
/// operator, without parentheses changes the type a step computes in.
pub fn regroups_type(parent: FunctionExprent, right: Exprent) -> bool {
    let func = parent.get_func_type();
    if !matches!(func, FunctionType::Add | FunctionType::Mul) {
        return false;
    }
    let own = own_type(parent.0);
    if own_type(right) != own {
        return true;
    }
    let mut first = right;
    while let Some(f) = first
        .try_as_function()
        .filter(|f| f.get_func_type() == func)
    {
        first = f.get_lst_operands().get(0);
    }
    let left = parent.get_lst_operands().get(0);
    promoted(written_type(left), written_type(first)) != own
}
