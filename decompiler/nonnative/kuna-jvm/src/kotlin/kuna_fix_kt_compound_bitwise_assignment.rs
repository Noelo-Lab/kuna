//! kuna fix `kt_compound_bitwise_assignment`: in Kotlin output, a compound
//! assignment with a bitwise or shift operator is written as an assignment
//! of the infix call.
//!
//! The Kotlin plugin writes assignments with Java's `AssignmentExprent`,
//! which writes a compound assignment `x |= y`; Kotlin has `+=`, `-=`, `*=`,
//! `/=` and `%=`, but no `|=`, `&=`, `^=`, `<<=`, `>>=` or `>>>=` (its
//! bitwise operators are the infix functions `or`, `and`, `xor`, `shl`,
//! `shr`, `ushr`), so kotlinc rejects every one (each `suspend` function's
//! continuation has `this.label |= Integer.MIN_VALUE`). Java evaluates the
//! left side's subexpressions once, reads it, evaluates the right side,
//! applies the operator and stores; `x = x or y` evaluates the target's
//! subexpressions, reads `x`, evaluates `y` and stores, which is the same
//! when reading the left side again has no effect: when it is built from
//! variables, constants, fields, array elements and arithmetic that cannot
//! throw (no division) only. Its type must be `Int`, `Long` or `Boolean`,
//! which Kotlin's infix functions take and give back; Java narrows a `byte`,
//! `short` or `char` result again, so those are left as they are. Infix
//! calls bind less tightly than arithmetic and more than comparisons, and
//! associate to the left, so the right side is written in parentheses unless
//! it is a value, a call, or a prefix, postfix, conversion or arithmetic
//! operation (`set or (1 shl id)`, which Java writes `set |= 1 << id`).
//! With the fix `kt_narrow_bitwise_operand` a `Byte`, `Short` or `Char` left
//! side is widened and the result narrowed back, as Java does:
//! `b = (b.toInt() xor k.toInt()).toByte()`.

use crate::code_type::CodeType;
use crate::exprent::Exprent;
use crate::function_exprent::FunctionType;
use crate::k_function_exprent::KFunctionType;
use crate::kotlin::kuna_fix_kt_narrow_bitwise_operand;
use crate::kuna_fixes::{self, KunaFix};
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
    let narrowed = kuna_fixes::enabled(KunaFix::KtNarrowBitwiseOperand)
        && kuna_fix_kt_narrow_bitwise_operand::widening(left_type).is_some();
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
        buffer.append(kuna_fix_kt_narrow_bitwise_operand::widening(left_type).unwrap_or(""));
    }
    buffer.append(" ").append(name).append(" ");
    let widened = narrowed
        .then(|| kuna_fix_kt_narrow_bitwise_operand::write(right, indent))
        .flatten();
    if let Some(widened) = widened {
        buffer.append_buffer(&widened);
    } else if binds_tighter(right) {
        buffer.append_buffer(&right.to_java(indent));
    } else {
        buffer
            .append("(")
            .append_buffer(&right.to_java(indent))
            .append(")");
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

/// Whether `exprent` needs no parentheses as an infix call's right operand.
fn binds_tighter(exprent: Exprent) -> bool {
    if exprent
        .try_as_k_function()
        .is_some_and(|k| k.kd(|d| d.k_type) != KFunctionType::None)
    {
        return false;
    }
    let Some(function) = exprent.try_as_function() else {
        return !exprent.is_assignment();
    };
    let type_ = function.get_func_type();
    matches!(
        type_,
        FunctionType::Add
            | FunctionType::Sub
            | FunctionType::Mul
            | FunctionType::Div
            | FunctionType::Rem
            | FunctionType::BitNot
            | FunctionType::BoolNot
            | FunctionType::Neg
            | FunctionType::Cast
    ) || type_.cast_type().is_some()
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
