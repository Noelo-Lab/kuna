//! kuna fix `reference_unboxing` (spec 10.16): an unboxing call is written
//! where leaving it out would hand the box itself to a context that takes
//! a reference.
//!
//! `InvocationExprent` leaves every unboxing call (`x.longValue()`) and
//! every boxing call (`Long.valueOf(v)`) unwritten unless an overload needs
//! it, since Java converts between a box and its primitive on its own. That
//! holds only where the context converts: a boxing call whose argument is
//! an unboxing call of the same box came out as the box itself
//! (`Long.valueOf(((Long) o).longValue())`, the bytecode of `return (long) o`
//! from a `Long` method, came out `return (Long) o`), and so did `==` or `!=`
//! between two unboxing calls (`x.intValue() == y.intValue()` came out
//! `x == y`, which compares the boxes). Both pass `null` where the bytecode
//! throws `NullPointerException`, and keep a box the bytecode replaces;
//! `==` compares identities. With the fix the unboxing call is written in
//! these places (the left one of a comparison), as javac's source had it.
//! Java output only.

use crate::exprent::Exprent;
use crate::invocation_exprent::InvocationExprent;

/// `expr` as an unboxing call that would be written as its bare operand.
fn unwritten_unboxing(expr: Exprent) -> Option<InvocationExprent> {
    if expr.is_kotlin() {
        return None;
    }
    expr.try_as_invocation()
        .filter(|inv| inv.is_unboxing_call() && !inv.should_force_unboxing())
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
