//! kuna fix `kt_operator_line_break`: in Kotlin output, a long binary
//! operation is broken after its operator, not before it.
//!
//! Vineflower breaks a line that grows too long before a binary operator
//! (`a` / `+ b` on the next line), which Java reads as one expression. Kotlin
//! ends a statement at a line break unless the expression cannot end there:
//! inside parentheses or brackets, after an operator, or before `&&`, `||`,
//! `?:` and `.`. So `return a` / `+ b` returns `a` and evaluates `+b` as a
//! statement of its own, and `x = a` / `- b` assigns `a`, without an error
//! (a break before `*`, `/`, `%` or a comparison does not compile). With the
//! fix the break of every binary operator but `&&` and `||` goes after the
//! operator (`a +` / `b`), where Kotlin continues the expression; the text
//! is the same apart from where the line breaks.

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
