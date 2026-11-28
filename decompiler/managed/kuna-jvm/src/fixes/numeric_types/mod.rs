//! Fix family `numeric_types`: the types of numeric and boolean constants and
//! operands, widening and boxing. In this file, `mixed_type_reassociation`
//! keeps the parentheses of `a op (b op c)` where regrouping changes the type a
//! step computes in; `reference_unboxing` writes an unboxing call where leaving
//! it out hands the box to a context that takes a reference;
//! `unboxed_assignment_precedence` gives an unwritten unboxing of an assignment
//! the assignment's precedence.

pub(crate) mod compound_assignment_shape;
pub(crate) mod concat_operand_type;
pub(crate) mod constant_local_int;
pub(crate) mod implicit_widening_arithmetic;
pub(crate) mod lossy_widening_operand;
pub(crate) mod ternary_numeric_arms;

use crate::assignment_exprent::AssignmentExprent;
use crate::exprent::Exprent;
use crate::fixes::numeric_types::implicit_widening_arithmetic::{own_type, written_type};
use crate::fixes::util::promoted;
use crate::function_exprent::{FunctionExprent, FunctionType};
use crate::invocation_exprent::InvocationExprent;

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

/// `expr` as an unboxing call that would be written as its bare operand.
fn unwritten_unboxing(expr: Exprent) -> Option<InvocationExprent> {
    if expr.is_kotlin() {
        return None;
    }
    expr.try_as_invocation()
        .filter(|inv| inv.is_unboxing_call() && !inv.should_force_unboxing())
}

/// The assignment an unboxing call `inv` is written as when the call is left
/// unwritten (`++a` on an `Integer` is `(a = a + 1).intValue()`): the call then
/// has the assignment's precedence, so an operator around it parenthesizes it.
pub fn bare_unboxed_assignment(inv: InvocationExprent) -> Option<AssignmentExprent> {
    unwritten_unboxing(inv.0)?.get_instance()?.try_as_assignment()
}

/// `argument`, the argument of a boxing call left unwritten: its unboxing
/// call is written.
pub fn keep_under_boxing(argument: Exprent) {
    if let Some(inv) = unwritten_unboxing(argument) {
        inv.force_unboxing(true);
    }
}

/// The operands of `==` or `!=`: if both are unboxing calls left unwritten,
/// the left one is written.
pub fn keep_in_comparison(left: Exprent, right: Exprent) {
    if let (Some(inv), Some(_)) = (unwritten_unboxing(left), unwritten_unboxing(right)) {
        inv.force_unboxing(true);
    }
}
