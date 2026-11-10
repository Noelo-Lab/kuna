//! The kuna option `jvm_javac_layout` (`kuna_javac_layout`), for comparisons
//! with a boolean literal: `SecondaryFunctionsHelper` keeps the comparison
//! where javac's bytecode shows the source had one.
//!
//! javac compiles `x == false`, `x != false` and `!x` alike (`ifeq`/`ifne` on
//! the value: a comparison with the constant 0 on the right is one jump), but
//! not a literal on the left (`false == x` pushes `iconst_0` first and jumps
//! with `if_icmpne`), a comparison with `true` (`iconst_1`, `if_icmp..`), or a
//! condition compared with `false` (`(a || b) == false` first computes the
//! condition's value, 0 or 1, then tests it). Vineflower folds all of them
//! into `x`, `!x` or a negated condition. With the option on, a comparison of
//! two booleans whose constant is on the left, or is `true`, or whose other
//! operand is a condition, stays a comparison.

use crate::exprent::Exprent;
use crate::function_exprent::FunctionType;

/// Whether `left == right` / `left != right` (both booleans, one a constant)
/// is written as a comparison rather than as its operand or its negation.
pub fn keeps(left: Exprent, right: Exprent) -> bool {
    match (left.try_as_const(), right.try_as_const()) {
        (Some(_), None) => true,
        (None, Some(c)) => c.get_int_value() != 0 || is_condition(left),
        _ => false,
    }
}

/// Whether `e` is a condition whose value javac computes as 0 or 1 before a
/// comparison tests it: a relational comparison, `&&` or `||`.
fn is_condition(e: Exprent) -> bool {
    e.try_as_function().is_some_and(|f| {
        matches!(
            f.get_func_type(),
            FunctionType::Eq
                | FunctionType::Ne
                | FunctionType::Lt
                | FunctionType::Ge
                | FunctionType::Gt
                | FunctionType::Le
                | FunctionType::BooleanAnd
                | FunctionType::BooleanOr
        )
    })
}
