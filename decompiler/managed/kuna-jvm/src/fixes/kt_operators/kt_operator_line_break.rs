//! Fix `kt_operator_line_break`: a long binary operation other than `&&` and
//! `||` breaks its line after the operator, where Kotlin continues the
//! expression.

use crate::function_exprent::{FunctionExprent, FunctionType};

/// Whether the possible line break of the binary operator `operation` goes
/// after the operator: in Kotlin output, unless it is `&&` or `||`.
pub fn breaks_after(operation: FunctionExprent) -> bool {
    operation.0.is_k_function()
        && !matches!(
            operation.get_func_type(),
            FunctionType::BooleanAnd | FunctionType::BooleanOr
        )
}
