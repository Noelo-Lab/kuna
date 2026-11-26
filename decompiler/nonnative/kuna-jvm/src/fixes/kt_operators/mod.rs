//! Fix family `kt_operators` (Kotlin output): operators: precedence, equality,
//! bitwise operators, numeric promotion. Here: equality written as Kotlin's
//! (`!==` for a reference inequality, `!=` for a negated `==`, a copy keeping
//! its operator) and a narrow bitwise operand widened to `Int`.

pub(crate) mod kt_binary_numeric_promotion;
pub(crate) mod kt_compound_bitwise_assignment;
pub(crate) mod kt_implicit_widening;
pub(crate) mod kt_operator_line_break;
pub(crate) mod kt_operator_precedence;

use crate::code_type::CodeType;
use crate::exprent::Exprent;
use crate::fixes::kt_java_syntax::kt_java_cast_syntax;
use crate::function_exprent::FunctionType;
use crate::k_function_exprent::{AnyFunctionType, KFunctionExprent, KFunctionType};
use crate::text_buffer::TextBuffer;
use crate::type_family::TypeFamily;
use crate::var_type::VarType;

/// Marks `comparison`, made from a Java comparison, as an identity test when
/// it compares two operands neither of which is the `null` literal.
pub fn mark_identity(comparison: KFunctionExprent) {
    if !matches!(
        comparison.get_func_type(),
        FunctionType::Eq | FunctionType::Ne
    ) && comparison.get_any_function_type() != AnyFunctionType::Kotlin(KFunctionType::Equals3)
    {
        return;
    }
    let operands = comparison.get_lst_operands();
    let not_null = |i: usize| {
        operands
            .get(i)
            .get_expr_type_opt()
            .is_some_and(|t| !t.ptr_eq(&VarType::VARTYPE_NULL()))
    };
    if not_null(0) && not_null(1) {
        comparison.kd_mut(|d| d.identity = true);
    }
}

/// Whether `comparison` (`==` or `!=`) is an identity test of two references.
pub fn identity(comparison: KFunctionExprent) -> bool {
    let operands = comparison.get_lst_operands();
    let reference = |i: usize| {
        operands
            .get(i)
            .get_expr_type_opt()
            .is_some_and(|t| t.type_family == TypeFamily::Object)
    };
    comparison.kd(|d| d.identity) && reference(0) && reference(1)
}

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
            let identity = comparison.kd(|d| d.identity);
            (!references || identity).then_some(FunctionType::Ne)
        }
        _ => None,
    }
}

/// The comparison that `xor`, an `xor` with `true`, negates, where the fix
/// writes `xor` as the negated comparison.
pub fn negated(xor: Exprent) -> Option<KFunctionExprent> {
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
pub fn write_negated(mut buf: TextBuffer, comparison: KFunctionExprent, indent: i32) -> TextBuffer {
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

/// Gives `copy` the function type of `original` where the constructor
/// changed it: an `==` it made `===`.
pub fn keep_function_type(original: KFunctionExprent, copy: KFunctionExprent) {
    if original.get_func_type() == FunctionType::Eq
        && original.kd(|d| d.k_type) == KFunctionType::None
        && copy.get_func_type() != FunctionType::Eq
    {
        copy.set_func_type(FunctionType::Eq);
    }
}

/// The conversion that widens a value of `type_` to `Int` in Kotlin, if it
/// is `Byte`, `Short` or `Char`.
pub fn widening(type_: &VarType) -> Option<&'static str> {
    if type_.array_dim != 0 {
        return None;
    }
    match type_.type_ {
        CodeType::Byte | CodeType::Short => Some(".toInt()"),
        CodeType::Char => Some(".code"),
        _ => None,
    }
}

/// The type of the value Kotlin gives `operand`, where it is a `Byte`,
/// `Short` or `Char` value: not the result of an arithmetic, bitwise or
/// negation operator, which Kotlin computes as `Int` like Java, nor a number
/// constant, written as an `Int` literal.
fn narrow_type(operand: Exprent) -> Option<VarType> {
    let mut value = operand;
    while let Some(function) = value.try_as_function() {
        if function.does_cast() || function.get_func_type().cast_type().is_none() {
            break;
        }
        value = function.get_lst_operands().get(0);
    }
    if let Some(function) = value.try_as_function() {
        if matches!(
            function.get_func_type(),
            FunctionType::Add
                | FunctionType::Sub
                | FunctionType::Mul
                | FunctionType::Div
                | FunctionType::Rem
                | FunctionType::And
                | FunctionType::Or
                | FunctionType::Xor
                | FunctionType::Shl
                | FunctionType::Shr
                | FunctionType::Ushr
                | FunctionType::Neg
                | FunctionType::BitNot
        ) {
            return None;
        }
    }
    let type_ = kt_java_cast_syntax::written_type(operand, operand.get_expr_type_opt()?);
    if value.try_as_const().is_some() && type_.type_ != CodeType::Char {
        return None;
    }
    widening(&type_).map(|_| type_)
}

/// `operand` written widened to the type Java promotes it to (`Int`, or
/// `Long` through an implicit conversion), if its Kotlin type is `Byte`,
/// `Short` or `Char`.
pub fn write_widened(operand: Exprent, indent: i32) -> Option<TextBuffer> {
    let narrow = narrow_type(operand)?;
    let long = operand.try_as_function().is_some_and(|f| {
        !f.does_cast()
            && f.get_func_type()
                .cast_type()
                .is_some_and(|t| t.type_ == CodeType::Long)
    });
    let suffix = if long {
        kt_java_cast_syntax::conversion(narrow.type_, CodeType::Long)
    } else {
        widening(&narrow)?
    };
    let mut buf = operand.to_java(indent);
    if !kt_operator_precedence::binds_as_postfix(operand) {
        buf.enclose("(", ")");
    }
    buf.append(suffix);
    Some(buf)
}
