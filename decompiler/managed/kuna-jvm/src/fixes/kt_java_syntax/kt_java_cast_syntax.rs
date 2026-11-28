//! Fix `kt_java_cast_syntax`: a cast Vineflower writes in Java's syntax is
//! written with the Kotlin construct that compiles to it: a numeric conversion
//! call (`x.toLong()`, `c.code`), `value as T` for a type variable.

use crate::code_type::CodeType;
use crate::const_exprent::ConstValue;
use crate::exprent::Exprent;
use crate::fixes::kt_java_syntax::kt_java_reference_cast;
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
    if let Some(cast) = syntax_form(to, from) {
        return Some(cast);
    }
    kt_java_reference_cast::applies(to, from)
        .then(|| KotlinCast::Reference(kt_java_reference_cast::nullable(to, from)))
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
        return kt_java_reference_cast::write(operand, to, nullable, indent);
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
