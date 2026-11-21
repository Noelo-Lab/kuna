//! Fix `kt_binary_numeric_promotion`: an operand of arithmetic, a comparison or
//! `==` is written with the conversion Java's binary numeric promotion makes
//! (`.code` for a `Char`, `toInt()`/`toLong()`..), which Kotlin's operators do not.

use crate::code_type::CodeType;
use crate::exprent::Exprent;
use crate::function_exprent::FunctionType;
use crate::fixes::kt_operators::kt_operator_precedence;
use crate::fixes::kt_java_syntax::kt_java_cast_syntax;
use crate::fixes::util::promoted;
use crate::text_buffer::TextBuffer;

/// The Kotlin type of the value `operand` is written as, if it is a number
/// or a `Char`.
fn kotlin_type(operand: Exprent) -> Option<CodeType> {
    let written = kt_java_cast_syntax::written_type(operand, operand.get_expr_type_opt()?);
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
                |t: CodeType| (t != to).then(|| kt_java_cast_syntax::conversion(t, to));
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
    if !kt_operator_precedence::binds_as_postfix(operand) {
        buf.enclose("(", ")");
    }
    buf.append(conversion);
    buf
}
