//! kuna fix `concat_operand_type` (spec 10.16): a string concatenation
//! operand is written with the type its `append` or
//! `makeConcatWithConstants` argument converted it as.
//!
//! `ConcatenationHelper` rebuilds `new StringBuilder().append(a).append(b)
//! .toString()` and `makeConcatWithConstants(a, b)` as `"" + a + b` from the
//! arguments alone. The argument's type in the descriptor decided how javac
//! turned the value into text, and the rebuilt operand need not have it: a
//! `char` passed to `append(I)` (`+ (int) c`, no instruction) came out `+ c`,
//! a character instead of its code; `append(C)` of `c ? '[' : '('`, whose
//! constants carry no type, came out `+ (c ? 91 : 40)`; and an implicit
//! widening (`+ (double) f`, `+ (float) n`) is left unwritten, so the text of
//! the narrower value came out (`0.1` for `0.10000000149011612`, `7` for
//! `7.0`). With the fix each operand keeps the type of its argument: the
//! constants of a `char` argument are written as characters, and an operand
//! of another kind than its argument (`char`, `int` and narrower, `long`,
//! `float`, `double`) is cast to it, so that javac converts it the same way.
//! Kotlin output (string templates) is unchanged.

use crate::code_type::CodeType;
use crate::const_exprent::ConstExprent;
use crate::expr_processor::{ExprProcessor, NullCastType};
use crate::exprent::Exprent;
use crate::function_exprent::FunctionType;
use crate::invocation_exprent::InvocationExprent;
use crate::kuna_fix_kt_java_cast_syntax;
use crate::kuna_fixes::{self, KunaFix};
use crate::text_buffer::TextBuffer;
use crate::var_type::VarType;

/// Records on the left-deep concatenation `concat` the argument type of each
/// of its `operands` (`None` where the operand was a string constant).
pub fn record(concat: Exprent, slots: &[Option<VarType>]) {
    let mut node = concat;
    for i in (1..slots.len()).rev() {
        let Some(f) = node
            .try_as_function()
            .filter(|f| f.get_func_type() == FunctionType::StrConcat)
        else {
            return;
        };
        let left = if i == 1 { slots[0].clone() } else { None };
        f.d_mut(|d| d.kuna_concat_slots = [left, slots[i].clone()]);
        node = f.get_lst_operands().get(0);
    }
}

/// The argument type of `operand` when `String.valueOf(..)` around it was
/// removed, else `slot`.
pub fn value_of_slot(before: Exprent, after: Exprent, slot: Option<VarType>) -> Option<VarType> {
    if before == after {
        return slot;
    }
    before
        .try_as_invocation()
        .map(|iex| iex.get_descriptor().params[0].clone())
}

/// The argument types of the operands `makeConcatWithConstants` gave:
/// `parameters` are the call's arguments before `String.valueOf` removal.
pub fn indy_slots(
    call: InvocationExprent,
    parameters: &[Exprent],
    operands: &[Exprent],
) -> Vec<Option<VarType>> {
    let descriptor = call.get_descriptor();
    let stripped = call.get_lst_parameters().to_vec();
    operands
        .iter()
        .map(|op| {
            (0..parameters.len())
                .find(|&i| stripped.get(i) == Some(op))
                .and_then(|i| {
                    value_of_slot(
                        parameters[i],
                        stripped[i],
                        descriptor.params.get(i).cloned(),
                    )
                })
        })
        .collect()
}

#[derive(PartialEq)]
enum Kind {
    Boolean,
    Char,
    Int,
    Long,
    Float,
    Double,
    Reference,
}

fn kind(t: &VarType) -> Kind {
    if t.array_dim > 0 {
        return Kind::Reference;
    }
    match t.type_ {
        CodeType::Boolean => Kind::Boolean,
        CodeType::Char => Kind::Char,
        CodeType::Byte
        | CodeType::Short
        | CodeType::Int
        | CodeType::Bytechar
        | CodeType::Shortchar => Kind::Int,
        CodeType::Long => Kind::Long,
        CodeType::Float => Kind::Float,
        CodeType::Double => Kind::Double,
        _ => Kind::Reference,
    }
}

/// The type `e` is written with: an implicit widening is not written.
fn written_type(e: Exprent) -> VarType {
    if let Some(f) = e.try_as_function() {
        if f.get_simple_cast_type().is_some() && !f.does_cast() {
            return written_type(f.get_lst_operands().get(0));
        }
    }
    e.get_expr_type()
}

/// The constants `e` yields, through conditionals; `None` if it yields
/// anything else.
fn constants(e: Exprent, out: &mut Vec<ConstExprent>) -> bool {
    if let Some(c) = e.try_as_const() {
        out.push(c);
        return true;
    }
    match e.try_as_function() {
        Some(f) if f.get_func_type() == FunctionType::Ternary => {
            let operands = f.get_lst_operands();
            constants(operands.get(1), out) && constants(operands.get(2), out)
        }
        _ => false,
    }
}

/// `operand` written with the kind of `slot`, when it is not written so
/// already.
pub fn write(operand: Exprent, slot: &Option<VarType>, indent: i32) -> Option<TextBuffer> {
    let slot = slot.as_ref()?;
    let wanted = kind(slot);
    if wanted == Kind::Reference
        || wanted == Kind::Boolean
        || !kuna_fixes::enabled(KunaFix::ConcatOperandType)
        || kuna_fix_kt_java_cast_syntax::writing_kotlin()
        || kind(&written_type(operand)) == wanted
    {
        return None;
    }
    let mut consts = Vec::new();
    if wanted == Kind::Char
        && constants(operand, &mut consts)
        && consts.iter().all(|c| {
            kind(&c.get_const_type()) == Kind::Int
                && c.get_value()
                    .is_some_and(|v| (0..=0xFFFF).contains(&v.as_int()))
        })
    {
        for c in consts {
            c.set_const_type(VarType::VARTYPE_CHAR());
        }
        return None;
    }
    let mut buf = TextBuffer::new();
    ExprProcessor::get_casted_exprent_full(
        operand,
        slot,
        &mut buf,
        indent,
        NullCastType::DontCast,
        true,
        false,
        false,
    );
    Some(buf)
}
