//! kuna fix `kt_binary_numeric_promotion`: in Kotlin output, the operands of
//! an arithmetic, comparison or equality operator are converted where Java
//! promotes them and Kotlin does not.
//!
//! Java converts the operands of `+ - * / %`, `< <= > >=` and `== !=` to a
//! common type before it applies the operator (binary numeric promotion): a
//! `byte`, `short` or `char` to `int`, then both to `long`, `float` or
//! `double` if one is. The Kotlin plugin writes these operators as they are,
//! but Kotlin's are member functions that do not promote: `Char + Int` and
//! `Char - Int` give a `Char` (Java computes an `int`), `Char` has no other
//! arithmetic and compares only with a `Char`, and `==` needs both operands
//! of one type (kotlinc rejects `b == 0` for a `Byte` `b`, `c < 128` for a
//! `Char` `c` and `l != i` for a `Long` `l` and an `Int` `i`). With the fix
//! an operand is written with the conversion Java's promotion makes, which
//! compiles to the instructions Java's does: a `Char` operand of arithmetic
//! (other than `Char - Char`, an `Int` in both languages) or of a comparison
//! with another type takes `.code`, and the operand of `==` or `!=` whose
//! type is not the promoted one takes its conversion to it
//! (`b.toInt() == 0`, `l != i.toLong()`); `-c` becomes `-c.code`.

use crate::code_type::CodeType;
use crate::exprent::Exprent;
use crate::function_exprent::FunctionType;
use crate::kotlin::kuna_fix_kt_operator_precedence;
use crate::kuna_fix_kt_java_cast_syntax;
use crate::text_buffer::TextBuffer;

/// The Kotlin type of the value `operand` is written as, if it is a number
/// or a `Char`.
fn kotlin_type(operand: Exprent) -> Option<CodeType> {
    let written = kuna_fix_kt_java_cast_syntax::written_type(operand, operand.get_expr_type_opt()?);
    if written.array_dim != 0 {
        return None;
    }
    matches!(
        written.type_,
        CodeType::Byte
            | CodeType::Short
            | CodeType::Char
            | CodeType::Int
            | CodeType::Long
            | CodeType::Float
            | CodeType::Double
    )
    .then_some(written.type_)
}

/// Java's binary numeric promotion of `a` and `b`.
fn promoted(a: CodeType, b: CodeType) -> CodeType {
    for wide in [CodeType::Double, CodeType::Float, CodeType::Long] {
        if a == wide || b == wide {
            return wide;
        }
    }
    CodeType::Int
}

fn code(type_: CodeType) -> Option<&'static str> {
    (type_ == CodeType::Char).then_some(".code")
}

/// The conversions the operands `left` and `right` of the binary operator
/// `func_type` are written with in Kotlin, if one needs one.
pub fn conversions(
    func_type: FunctionType,
    left: Exprent,
    right: Exprent,
) -> Option<(Option<&'static str>, Option<&'static str>)> {
    let (l, r) = (kotlin_type(left)?, kotlin_type(right)?);
    let both_char = l == CodeType::Char && r == CodeType::Char;
    let pair = match func_type {
        FunctionType::Add
        | FunctionType::Sub
        | FunctionType::Mul
        | FunctionType::Div
        | FunctionType::Rem => {
            if func_type == FunctionType::Sub && both_char {
                return None;
            }
            (code(l), code(r))
        }
        FunctionType::Lt | FunctionType::Ge | FunctionType::Gt | FunctionType::Le => {
            if both_char {
                return None;
            }
            (code(l), code(r))
        }
        FunctionType::Eq | FunctionType::Ne => {
            if l == r {
                return None;
            }
            let to = promoted(l, r);
            let convert =
                |t: CodeType| (t != to).then(|| kuna_fix_kt_java_cast_syntax::conversion(t, to));
            (convert(l), convert(r))
        }
        _ => return None,
    };
    (pair.0.is_some() || pair.1.is_some()).then_some(pair)
}

/// The conversion of the operand of a unary minus, `-c.code`.
pub fn negation(operand: Exprent) -> Option<&'static str> {
    code(kotlin_type(operand)?)
}

/// `operand` written with `conversion` after it.
pub fn write(operand: Exprent, conversion: &str, indent: i32) -> TextBuffer {
    let mut buf = operand.to_java(indent);
    if !kuna_fix_kt_operator_precedence::binds_as_postfix(operand) {
        buf.enclose("(", ")");
    }
    buf.append(conversion);
    buf
}
