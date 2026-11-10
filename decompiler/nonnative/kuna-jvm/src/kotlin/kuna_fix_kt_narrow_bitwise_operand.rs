//! kuna fix `kt_narrow_bitwise_operand`: in Kotlin output, a `Byte`, `Short`
//! or `Char` operand of a bitwise or shift operator is widened to `Int` as
//! Java widens it.
//!
//! Java promotes a `byte`, `short` or `char` operand of `&`, `|`, `^`, `~`,
//! `<<`, `>>` and `>>>` to `int` without writing it; the Kotlin plugin writes
//! the operator as Kotlin's infix function (`b and 255`, `x shl b`,
//! `b.inv()`), which `Byte`, `Short` and `Char` do not have and whose shift
//! count is an `Int`, so kotlinc rejects each ("unresolved reference 'and'").
//! With the fix such an operand is written with the conversion Kotlin needs,
//! `b.toInt() and 255`, `c.code xor 2`, `x shl b.toInt()`, which compiles to
//! no instruction, as Java's promotion; one Java widens further to `long`
//! takes that conversion, `b.toLong() and 255L` (`i2l`, as Java). A
//! compound assignment to a `Byte`, `Short` or `Char` place
//! (`buffer[i] ^= key[j]`, which Java narrows back) is written
//! `buffer[i] = (buffer[i].toInt() xor key[j].toInt()).toByte()` when the fix
//! `kt_compound_bitwise_assignment` can reread its left side.

use crate::code_type::CodeType;
use crate::exprent::Exprent;
use crate::function_exprent::FunctionType;
use crate::kotlin::kuna_fix_kt_operator_precedence;
use crate::kuna_fix_kt_java_cast_syntax;
use crate::text_buffer::TextBuffer;
use crate::var_type::VarType;

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
    let type_ = kuna_fix_kt_java_cast_syntax::written_type(operand, operand.get_expr_type_opt()?);
    if value.try_as_const().is_some() && type_.type_ != CodeType::Char {
        return None;
    }
    widening(&type_).map(|_| type_)
}

/// `operand` written widened to the type Java promotes it to (`Int`, or
/// `Long` through an implicit conversion), if its Kotlin type is `Byte`,
/// `Short` or `Char`.
pub fn write(operand: Exprent, indent: i32) -> Option<TextBuffer> {
    let narrow = narrow_type(operand)?;
    let long = operand.try_as_function().is_some_and(|f| {
        !f.does_cast()
            && f.get_func_type()
                .cast_type()
                .is_some_and(|t| t.type_ == CodeType::Long)
    });
    let suffix = if long {
        kuna_fix_kt_java_cast_syntax::conversion(narrow.type_, CodeType::Long)
    } else {
        widening(&narrow)?
    };
    let mut buf = operand.to_java(indent);
    if !kuna_fix_kt_operator_precedence::binds_as_postfix(operand) {
        buf.enclose("(", ")");
    }
    buf.append(suffix);
    Some(buf)
}
