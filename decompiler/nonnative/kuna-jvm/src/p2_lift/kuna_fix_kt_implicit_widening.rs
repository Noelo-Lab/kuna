//! kuna fix `kt_implicit_widening`: in Kotlin output, a number or `Char`
//! that is stored, passed or returned where a wider numeric type is expected
//! is written with its conversion.
//!
//! Java widens a primitive implicitly in assignment, invocation and return
//! contexts (JLS 5.1.2), and Vineflower writes the value as it is: a
//! `byte[]` element, typed `Byte` by its `baload`, stored into a local the
//! LocalVariableTable types `I` (the JVM has no instruction for that
//! widening), and an `i2l` it marks implicit. Kotlin converts only
//! explicitly, so `val b0: Int = this[i++]`, `Integer.toString(b, 16)` and
//! `val n: Long = source.readShortLe()` are rejected. kotlinc compiled the
//! source's `this[i++].toInt()` to the same `baload; istore`, and its
//! `.toLong()` to the `i2l`. With the fix the value takes that conversion
//! (`.code` for a `Char`), the same instructions; the declared types are
//! unchanged.

use crate::code_type::CodeType;
use crate::exprent::Exprent;
use crate::kuna_fix_kt_java_cast_syntax::{self, KotlinCast};
use crate::var_type::VarType;

/// The conversion `value`, written as `written`, takes where `target` is
/// expected, if Java widens it there implicitly. A constant is written in
/// the target's type already.
pub fn conversion(value: Exprent, target: &VarType, written: &VarType) -> Option<KotlinCast> {
    if target.array_dim != 0
        || written.array_dim != 0
        || written_operand(value).try_as_const().is_some()
    {
        return None;
    }
    let widens = target.type_ != CodeType::Char
        && rank(written.type_)
            .zip(rank(target.type_))
            .is_some_and(|(from, to)| from < to && !(written.type_ == CodeType::Char && to < 2));
    widens.then(|| {
        KotlinCast::Convert(kuna_fix_kt_java_cast_syntax::conversion(
            written.type_,
            target.type_,
        ))
    })
}

/// What `value` is written as: the operand of the conversions Vineflower
/// leaves implicit.
fn written_operand(value: Exprent) -> Exprent {
    match value.try_as_function() {
        Some(func) if func.get_func_type().cast_type().is_some() && !func.does_cast() => {
            written_operand(func.get_lst_operands().get(0))
        }
        _ => value,
    }
}

/// The place of a numeric type in Java's widening order; a `Char` widens to
/// `Int` and beyond, and nothing widens to a `Char`.
fn rank(type_: CodeType) -> Option<u8> {
    match type_ {
        CodeType::Byte => Some(0),
        CodeType::Short | CodeType::Char => Some(1),
        CodeType::Int => Some(2),
        CodeType::Long => Some(3),
        CodeType::Float => Some(4),
        CodeType::Double => Some(5),
        _ => None,
    }
}
