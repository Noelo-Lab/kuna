//! kuna fix `kt_negated_equality`: in Kotlin output, `(a == b) xor true` is
//! written `a != b`.
//!
//! kotlinc compiles Kotlin's `a != b` of two objects to
//! `Intrinsics.areEqual(a, b) ^ 1`; `ResugarKotlinMethodsPass` turns the call
//! back into `a == b`, and the Kotlin plugin writes the `^ 1` of a boolean as
//! `xor true`, so `a != b` came out `(a == b) xor true`. `x xor true` is `!x`
//! for a `Boolean` `x`, and the negation of `==` is `!=` (of `!=`, `==`; of
//! the identity test `===`, `!==`), so with the fix a comparison that `xor
//! true` negates is written with the other operator. The comparison then
//! binds as an equality, which `kt_operator_precedence` parenthesises by.

use crate::code_type::CodeType;
use crate::exprent::Exprent;
use crate::function_exprent::FunctionType;
use crate::k_function_exprent::{AnyFunctionType, KFunctionExprent, KFunctionType};
use crate::kuna_fixes::{self, KunaFix};
use crate::text_buffer::TextBuffer;
use crate::type_family::TypeFamily;

fn is_true(operand: Exprent) -> bool {
    operand.try_as_const().is_some_and(|c| {
        c.get_const_type().type_ == CodeType::Boolean
            && c.get_value().is_some()
            && c.get_int_value() == 1
    })
}

/// The function type that writes the negation of `comparison`, if it is an
/// `==`, `!=` or `===` whose negation can be written.
fn negation(comparison: KFunctionExprent) -> Option<FunctionType> {
    match comparison.get_any_function_type() {
        AnyFunctionType::Java(FunctionType::Eq) => Some(FunctionType::Ne),
        AnyFunctionType::Java(FunctionType::Ne) => Some(FunctionType::Eq),
        AnyFunctionType::Kotlin(KFunctionType::Equals3) => {
            let operands = comparison.get_lst_operands();
            let references = (0..2).all(|i| {
                operands
                    .get(i)
                    .get_expr_type_opt()
                    .is_some_and(|t| t.type_family == TypeFamily::Object)
            });
            let identity = kuna_fixes::enabled(KunaFix::KtReferenceInequality)
                && comparison.kd(|d| d.identity);
            (!references || identity).then_some(FunctionType::Ne)
        }
        _ => None,
    }
}

/// The comparison that `xor`, an `xor` with `true`, negates, where the fix
/// writes `xor` as the negated comparison.
pub fn negated(xor: Exprent) -> Option<KFunctionExprent> {
    if !kuna_fixes::enabled(KunaFix::KtNegatedEquality)
        || !kuna_fixes::enabled(KunaFix::KtOperatorPrecedence)
    {
        return None;
    }
    let xor = xor.try_as_k_function()?;
    if xor.get_func_type() != FunctionType::Xor {
        return None;
    }
    let operands = xor.get_lst_operands();
    if operands.len() != 2 {
        return None;
    }
    let comparison = match (is_true(operands.get(0)), is_true(operands.get(1))) {
        (false, true) => operands.get(0),
        (true, false) => operands.get(1),
        _ => return None,
    };
    comparison
        .try_as_k_function()
        .filter(|c| negation(*c).is_some())
}

/// `buf` with `comparison`, which `xor true` negates, written after it with
/// the other operator.
pub fn write(mut buf: TextBuffer, comparison: KFunctionExprent, indent: i32) -> TextBuffer {
    let original = comparison.get_any_function_type();
    let Some(other) = negation(comparison) else {
        return buf;
    };
    comparison.set_func_type(other);
    let text = comparison.0 .0.to_java(indent);
    comparison.set_func_type_typed(original);
    buf.append_buffer(&text);
    buf
}
