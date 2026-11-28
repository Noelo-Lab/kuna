//! Fix `kt_compound_bitwise_assignment`: `x |= y` and the other bitwise and shift
//! compound assignments are written `x = x or y` when reading `x` again has no
//! effect and its type is `Int`, `Long` or `Boolean`, the right side
//! parenthesised by Kotlin's precedence.

use crate::code_type::CodeType;
use crate::exprent::Exprent;
use crate::function_exprent::FunctionType;
use crate::k_function_exprent::KFunctionType;
use crate::fixes::kt_operators::{self, kt_operator_precedence};
use crate::text_buffer::TextBuffer;
use crate::var_type::VarType;

/// ` = left op right` for the compound assignment `left op= right`, or
/// `None` where Kotlin cannot write it that way.
pub fn write(
    left: Exprent,
    left_type: &VarType,
    operator: FunctionType,
    right: Exprent,
    indent: i32,
) -> Option<TextBuffer> {
    let narrowed = kt_operators::widening(left_type).is_some();
    if !matches!(
        operator,
        FunctionType::And
            | FunctionType::Or
            | FunctionType::Xor
            | FunctionType::Shl
            | FunctionType::Shr
            | FunctionType::Ushr
    ) || left_type.array_dim != 0
        || !(matches!(
            left_type.type_,
            CodeType::Int | CodeType::Long | CodeType::Boolean
        ) || narrowed)
        || !rereadable(left)
    {
        return None;
    }
    let name = match operator {
        FunctionType::And => "and",
        FunctionType::Or => "or",
        FunctionType::Xor => "xor",
        FunctionType::Shl => "shl",
        FunctionType::Shr => "shr",
        _ => "ushr",
    };
    let mut buffer = TextBuffer::new();
    buffer.append(" = ");
    if narrowed {
        buffer.append("(");
    }
    buffer.append_buffer(&left.to_java(indent));
    if narrowed {
        buffer.append(kt_operators::widening(left_type).unwrap_or(""));
    }
    buffer.append(" ").append(name).append(" ");
    let widened = narrowed
        .then(|| kt_operators::write_widened(right, indent))
        .flatten();
    if let Some(widened) = widened {
        buffer.append_buffer(&widened);
    } else if kt_operator_precedence::infix_right_operand(right) {
        buffer
            .append("(")
            .append_buffer(&right.to_java(indent))
            .append(")");
    } else {
        buffer.append_buffer(&right.to_java(indent));
    }
    if narrowed {
        buffer.append(")").append(match left_type.type_ {
            CodeType::Byte => ".toByte()",
            CodeType::Short => ".toShort()",
            _ => ".toChar()",
        });
    }
    Some(buffer)
}

/// Whether evaluating `exprent` again reads the same place with no effect.
fn rereadable(exprent: Exprent) -> bool {
    if exprent.try_as_var().is_some() || exprent.try_as_const().is_some() {
        return true;
    }
    if let Some(field) = exprent.try_as_field() {
        return field.get_instance().is_none_or(rereadable);
    }
    if let Some(array) = exprent.try_as_array() {
        return rereadable(array.get_array()) && rereadable(array.get_index());
    }
    exprent.try_as_function().is_some_and(|f| {
        exprent
            .try_as_k_function()
            .is_none_or(|k| k.kd(|d| d.k_type) == KFunctionType::None)
            && (matches!(
                f.get_func_type(),
                FunctionType::Add
                    | FunctionType::Sub
                    | FunctionType::Mul
                    | FunctionType::And
                    | FunctionType::Or
                    | FunctionType::Xor
                    | FunctionType::Shl
                    | FunctionType::Shr
                    | FunctionType::Ushr
                    | FunctionType::Neg
                    | FunctionType::BitNot
            ) || f.get_func_type().cast_type().is_some())
            && f.get_lst_operands().to_vec().into_iter().all(rereadable)
    })
}
