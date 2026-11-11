//! kuna fix `kt_java_cast_syntax`: in Kotlin output, the casts Vineflower
//! writes in Java's syntax, `(long)x`, `(int)Math.min(a, b)`, `(T)value`,
//! which is not Kotlin, are written with the Kotlin construct that does the
//! same thing.
//!
//! A cast to a numeric primitive type from a numeric primitive or its box is
//! a numeric conversion, which Kotlin writes as a call of the conversion
//! function: `x.toLong()`, `Math.min(a, b).toInt()`; from or to `char`
//! through its code (`c.code`, `i.toChar()`), and to `char`, `byte` and
//! `short` from a type other than `int` through `toInt()`, as Java converts
//! (`d.toInt().toShort()` for `(short)d`). Kotlin compiles each of these to
//! the same conversion instructions as Java's cast. A cast to a type
//! variable, `(T)value`, is unchecked in both languages and is written
//! `value as T`, which is unchecked too and, like Java's, lets `null` through.
//! An integer constant of a narrow type (`bipush 32` is typed as fitting
//! `byte` and `char`) is written as an `Int` literal, so it converts as one:
//! `32.toLong()`. Casts to class and array types are the fix
//! `kt_java_reference_cast`'s.

use crate::code_type::CodeType;
use crate::const_exprent::ConstValue;
use crate::decompiler_context::DecompilerContext;
use crate::exprent::Exprent;
use crate::kuna_fix_kt_java_reference_cast;
use crate::kuna_fixes::{self, KunaFix};
use crate::plugin_context::PluginContext;
use crate::text_buffer::TextBuffer;
use crate::var_type::VarType;

/// How a cast is written in Kotlin.
pub enum KotlinCast {
    /// A call of conversion functions after the operand, `.toInt()`.
    Convert(&'static str),
    /// An unchecked cast to a type variable, `as T`.
    TypeVariable,
    /// A cast to a class or array type, `as T` (fix `kt_java_reference_cast`);
    /// `true` when the operand may be `null` where its Kotlin type is too.
    Reference(bool),
}

/// Whether the class being written is written as Kotlin.
pub fn writing_kotlin() -> bool {
    DecompilerContext::get_current_class().is_some_and(|cl| {
        PluginContext::get_current_context()
            .get_language_spec(cl)
            .is_some()
    })
}

fn numeric(type_: CodeType) -> bool {
    matches!(
        type_,
        CodeType::Byte
            | CodeType::Short
            | CodeType::Char
            | CodeType::Int
            | CodeType::Long
            | CodeType::Float
            | CodeType::Double
    )
}

/// The conversion from a value of type `from` to `to`, both numeric.
pub fn conversion(from: CodeType, to: CodeType) -> &'static str {
    use CodeType::*;
    match (from, to) {
        (Char, Int) => ".code",
        (Char, Long) => ".code.toLong()",
        (Char, Float) => ".code.toFloat()",
        (Char, Double) => ".code.toDouble()",
        (Char, Byte) => ".code.toByte()",
        (Char, Short) => ".code.toShort()",
        (Char, Char) => "",
        (Int, Char) => ".toChar()",
        (_, Char) => ".toInt().toChar()",
        (Float | Double, Byte) => ".toInt().toByte()",
        (Float | Double, Short) => ".toInt().toShort()",
        (_, Byte) => ".toByte()",
        (_, Short) => ".toShort()",
        (_, Int) => ".toInt()",
        (_, Long) => ".toLong()",
        (_, Float) => ".toFloat()",
        (_, Double) => ".toDouble()",
        _ => "",
    }
}

/// The type of the value `operand` is written as: a conversion that writes
/// only its operand (Java's implicit widening) is written as that operand.
pub fn written_type(operand: Exprent, inferred: VarType) -> VarType {
    if matches!(inferred.type_, CodeType::Bytechar | CodeType::Shortchar)
        && inferred.array_dim == 0
        && operand.try_as_const().is_some()
        && kuna_fixes::enabled(KunaFix::KtJavaCastSyntax)
    {
        return VarType::VARTYPE_INT();
    }
    match operand.try_as_function() {
        Some(func) if func.get_func_type().cast_type().is_some() && !func.does_cast() => {
            let inner = func.get_lst_operands().get(0);
            written_type(inner, inner.get_expr_type())
        }
        _ => inferred,
    }
}

/// The Kotlin form of Java's cast of a value of type `from` to `to`, if it
/// has one with the same meaning.
pub fn form(to: &VarType, from: &VarType) -> Option<KotlinCast> {
    if kuna_fixes::enabled(KunaFix::KtJavaCastSyntax) {
        if let Some(cast) = syntax_form(to, from) {
            return Some(cast);
        }
    }
    kuna_fix_kt_java_reference_cast::applies(to, from)
        .then(|| KotlinCast::Reference(kuna_fix_kt_java_reference_cast::nullable(to, from)))
}

fn syntax_form(to: &VarType, from: &VarType) -> Option<KotlinCast> {
    if to.array_dim != 0 {
        return None;
    }
    if to.type_ == CodeType::Genvar {
        return Some(KotlinCast::TypeVariable);
    }
    if !numeric(to.type_) || from.array_dim != 0 {
        return None;
    }
    let from = VarType::UNBOXING_TYPES().get_or_default(from, from.clone());
    if from.array_dim != 0 || !numeric(from.type_) {
        return None;
    }
    Some(KotlinCast::Convert(conversion(from.type_, to.type_)))
}

/// Whether `operand` is a negative number, which a conversion call would
/// apply to before the minus (`-d.toInt()` is `-(d.toInt())`).
fn negative_literal(operand: Exprent) -> bool {
    operand.try_as_const().is_some_and(|c| {
        c.get_value().is_some_and(|v| match v {
            ConstValue::Int(i) => i < 0,
            ConstValue::Long(l) => l < 0,
            ConstValue::Float(f) => f.is_sign_negative(),
            ConstValue::Double(d) => d.is_sign_negative(),
            _ => false,
        })
    })
}

/// `operand` written with the cast `cast` to `to`.
pub fn write(cast: KotlinCast, operand: Exprent, to: &VarType, indent: i32) -> TextBuffer {
    if let KotlinCast::Reference(nullable) = cast {
        return kuna_fix_kt_java_reference_cast::write(operand, to, nullable, indent);
    }
    let mut buf = operand.to_java(indent);
    match cast {
        KotlinCast::Convert(suffix) => {
            let suffix = if to.type_ == CodeType::Char
                && operand
                    .try_as_const()
                    .is_some_and(|c| c.get_const_type().type_ == CodeType::Char)
            {
                ""
            } else {
                suffix
            };
            if !suffix.is_empty() && (operand.get_precedence() > 0 || negative_literal(operand)) {
                buf.enclose("(", ")");
            }
            buf.append(suffix);
        }
        KotlinCast::TypeVariable => {
            if operand.get_precedence() > 1 {
                buf.enclose("(", ")");
            }
            buf.append(" as ").append_cast_type_name(to);
        }
        KotlinCast::Reference(_) => unreachable!("written before"),
    }
    buf
}
