//! kuna fix `kt_operator_precedence`: in Kotlin output, an operand is
//! parenthesised where Kotlin's operator precedence needs it, not only where
//! Java's does.
//!
//! The Kotlin plugin writes Java's operators with Kotlin's syntax (`x shl 2`,
//! `a ?: b`, `x is T`, `x.inv()`) but decides parentheses with
//! `FunctionExprent.wrapOperandString`, by Java's precedence. Kotlin's grammar
//! orders them differently: postfix, prefix, `as`, multiplicative, additive,
//! range, infix function calls (`shl shr ushr and or xor`, every `infix fun`,
//! all one level, left-associative), elvis, `is`/`in`, comparison, equality,
//! `&&`, `||`. Java's `&`, `^` and `|` sit below comparison and equality, its
//! shifts above them, and each has a level of its own; Java's `?:`-like
//! elvis sits with `||`. So `x | 1 << k` came out `x or 1 shl k`, which
//! Kotlin reads `(x or 1) shl k`, `a | b & c` as `(a or b) and c`, `ok &
//! flags != 0` as `(ok and flags) != 0` and `x ?: (a && b)` as `(x ?: a) &&
//! b`. With the fix an operand gets parentheses also when its Kotlin level
//! binds less tightly than its operator's, or, as a right operand, as
//! tightly unless it is the same associative operator (`a or b or c`).
//! Parentheses Java's rule writes are kept. An unboxing call Vineflower does
//! not write binds as what it is written as: `!(x as Boolean)`, where the
//! unwritten `booleanValue()` came out `!x as Boolean`, which Kotlin reads
//! `(!x) as Boolean`.

use crate::const_exprent::ConstValue;
use crate::exprent::Exprent;
use crate::function_exprent::{FunctionExprent, FunctionType};
use crate::invocation_exprent::UnboxingWrittenAs;
use crate::k_element::KElement;
use crate::k_function_exprent::KFunctionType;
use crate::kotlin::kuna_fix_kt_negated_equality;
use crate::kt_flags::Flags;
use crate::kuna_fix_kt_java_cast_syntax::{self, KotlinCast};
use crate::kuna_fixes::{self, KunaFix};
use crate::var_type::VarType;

const POSTFIX: u8 = 0;
const PREFIX: u8 = 1;
const AS: u8 = 2;
const MULTIPLICATIVE: u8 = 3;
const ADDITIVE: u8 = 4;
const INFIX: u8 = 6;
const ELVIS: u8 = 7;
const NAMED_CHECK: u8 = 8;
const COMPARISON: u8 = 9;
const EQUALITY: u8 = 10;
const CONJUNCTION: u8 = 11;
const DISJUNCTION: u8 = 12;
const LOWEST: u8 = 13;

/// Whether the Kotlin text of `operand`, an operand of `parent`, needs
/// parentheses that Java's precedence did not give it.
pub fn needs_parentheses(parent: FunctionExprent, operand: Exprent) -> bool {
    let Some(kparent) = parent.0.try_as_k_function() else {
        return false;
    };
    let operands = parent.get_lst_operands();
    let level = level(operand);
    let needed = match kparent.kd(|d| d.k_type) {
        KFunctionType::Equals3 => binary(parent, EQUALITY, operand, level),
        KFunctionType::IfNull => binary(parent, ELVIS, operand, level),
        KFunctionType::GetKclass | KFunctionType::StrTemplate => false,
        KFunctionType::None => match parent.get_func_type() {
            FunctionType::And
            | FunctionType::Or
            | FunctionType::Xor
            | FunctionType::Shl
            | FunctionType::Shr
            | FunctionType::Ushr => binary(parent, INFIX, operand, level),
            FunctionType::BitNot | FunctionType::Ipp | FunctionType::Imm => level > POSTFIX,
            FunctionType::Neg | FunctionType::Ppi | FunctionType::Mmi => level > PREFIX,
            FunctionType::BoolNot => {
                let negated_check = operands
                    .get(0)
                    .try_as_k_function()
                    .is_some_and(|f| f.get_func_type() == FunctionType::Instanceof);
                level > if negated_check { NAMED_CHECK } else { PREFIX }
            }
            FunctionType::Instanceof => operands.get(0) == operand && level > NAMED_CHECK,
            FunctionType::Lt | FunctionType::Ge | FunctionType::Gt | FunctionType::Le => {
                binary(parent, COMPARISON, operand, level)
            }
            FunctionType::Eq | FunctionType::Ne => binary(parent, EQUALITY, operand, level),
            FunctionType::BooleanAnd => binary(parent, CONJUNCTION, operand, level),
            FunctionType::BooleanOr => binary(parent, DISJUNCTION, operand, level),
            _ => false,
        },
    };
    needed && kuna_fix_kt_java_cast_syntax::writing_kotlin()
}

/// Whether `operand` of the binary operator `parent` at `parent_level` needs
/// parentheses: it binds less tightly, or as tightly on the right (Kotlin's
/// binary operators associate to the left) unless it is the same
/// associative operation.
fn binary(parent: FunctionExprent, parent_level: u8, operand: Exprent, level: u8) -> bool {
    let operands = parent.get_lst_operands();
    if operands.len() < 2 {
        return false;
    }
    if level > parent_level {
        return true;
    }
    level == parent_level && operands.get(1) == operand && !same_associative(parent, operand)
}

fn same_associative(parent: FunctionExprent, operand: Exprent) -> bool {
    let func_type = parent.get_func_type();
    let same = operand
        .try_as_function()
        .is_some_and(|f| f.get_func_type() == func_type);
    let floating = operand.get_expr_type_opt().is_some_and(|t| {
        t.ptr_eq(&VarType::VARTYPE_FLOAT()) || t.ptr_eq(&VarType::VARTYPE_DOUBLE())
    });
    same && !floating
        && matches!(
            func_type,
            FunctionType::Add
                | FunctionType::Mul
                | FunctionType::And
                | FunctionType::Or
                | FunctionType::Xor
                | FunctionType::BooleanAnd
                | FunctionType::BooleanOr
                | FunctionType::StrConcat
        )
}

/// Whether the Kotlin text of the infix call's operand `operand` needs
/// parentheses; `right` for its argument, which binds to the call's right.
pub fn infix_call_operand(operand: Exprent, right: bool) -> bool {
    kuna_fixes::enabled(KunaFix::KtOperatorPrecedence)
        && if right {
            level(operand) >= INFIX
        } else {
            level(operand) > INFIX
        }
}

/// Whether the Kotlin text of `exprent` is a value or postfix expression, which
/// a postfix call can follow without parentheses.
pub fn binds_as_postfix(exprent: Exprent) -> bool {
    level(exprent) == POSTFIX
}

/// The level of Kotlin's grammar at which the Kotlin text of `exprent`
/// binds; lower binds tighter.
fn level(exprent: Exprent) -> u8 {
    if let Some(constant) = exprent.try_as_const() {
        let negative = constant.get_value().is_some_and(|v| match v {
            ConstValue::Int(i) => i < 0,
            ConstValue::Long(l) => l < 0,
            ConstValue::Float(f) => f.is_sign_negative(),
            ConstValue::Double(d) => d.is_sign_negative(),
            ConstValue::String(_) => false,
        });
        return if exprent.get_precedence() > 0 {
            MULTIPLICATIVE
        } else if negative {
            PREFIX
        } else {
            POSTFIX
        };
    }
    if exprent.is_assignment() {
        return LOWEST;
    }
    if let Some(written) = unboxing_written_as(exprent) {
        return level(written);
    }
    if let Some(invocation) = exprent.try_as_k_invocation() {
        let infix = invocation.kd(|d| match &d.calling_data {
            Some(KElement::Function(function)) => Flags::IS_INFIX.get(function.flags),
            _ => false,
        });
        return if infix { INFIX } else { POSTFIX };
    }
    let Some(function) = exprent.try_as_function() else {
        return POSTFIX;
    };
    if kuna_fix_kt_negated_equality::negated(exprent).is_some() {
        return EQUALITY;
    }
    if let Some(kfunction) = exprent.try_as_k_function() {
        match kfunction.kd(|d| d.k_type) {
            KFunctionType::Equals3 => return EQUALITY,
            KFunctionType::IfNull => return ELVIS,
            KFunctionType::GetKclass | KFunctionType::StrTemplate => return POSTFIX,
            KFunctionType::None => {}
        }
    }
    let kotlin = exprent.try_as_k_function().is_some();
    let operands = function.get_lst_operands();
    let func_type = function.get_func_type();
    if (func_type == FunctionType::Cast || func_type.cast_type().is_some()) && !function.does_cast()
    {
        return level(operands.get(0));
    }
    match func_type {
        FunctionType::Mul | FunctionType::Div | FunctionType::Rem => MULTIPLICATIVE,
        FunctionType::Add | FunctionType::Sub | FunctionType::StrConcat => ADDITIVE,
        FunctionType::And
        | FunctionType::Or
        | FunctionType::Xor
        | FunctionType::Shl
        | FunctionType::Shr
        | FunctionType::Ushr => INFIX,
        FunctionType::BitNot if kotlin => POSTFIX,
        FunctionType::BoolNot
            if kotlin
                && operands
                    .get(0)
                    .try_as_k_function()
                    .is_some_and(|f| f.get_func_type() == FunctionType::Instanceof) =>
        {
            NAMED_CHECK
        }
        FunctionType::BitNot
        | FunctionType::BoolNot
        | FunctionType::Neg
        | FunctionType::Ppi
        | FunctionType::Mmi => PREFIX,
        FunctionType::Ipp | FunctionType::Imm | FunctionType::ArrayLength => POSTFIX,
        FunctionType::Lcmp
        | FunctionType::Fcmpl
        | FunctionType::Fcmpg
        | FunctionType::Dcmpl
        | FunctionType::Dcmpg => POSTFIX,
        FunctionType::Cast if kotlin => AS,
        FunctionType::Cast => PREFIX,
        FunctionType::Instanceof => NAMED_CHECK,
        FunctionType::Lt | FunctionType::Ge | FunctionType::Gt | FunctionType::Le => COMPARISON,
        FunctionType::Eq | FunctionType::Ne => EQUALITY,
        FunctionType::BooleanAnd => CONJUNCTION,
        FunctionType::BooleanOr => DISJUNCTION,
        FunctionType::Ternary if kotlin && safe_cast(operands.to_vec()) => AS,
        FunctionType::Ternary => LOWEST,
        conversion if conversion.cast_type().is_some() => {
            if !kotlin || !kuna_fixes::enabled(KunaFix::KtJavaCastSyntax) {
                return PREFIX;
            }
            let operand = operands.get(0);
            let to = conversion.cast_type().expect("a conversion");
            let from = kuna_fix_kt_java_cast_syntax::written_type(operand, operand.get_expr_type());
            match kuna_fix_kt_java_cast_syntax::form(&to, &from) {
                Some(KotlinCast::Convert(_)) => POSTFIX,
                Some(_) => AS,
                None => PREFIX,
            }
        }
        _ => LOWEST,
    }
}

/// What `exprent`, an unboxing call that is not written, was written as
/// (`x as Boolean`, the cast that is its instance, binds as `as`).
fn unboxing_written_as(exprent: Exprent) -> Option<Exprent> {
    let invocation = exprent.try_as_invocation()?;
    if !invocation.is_unboxing_call() || invocation.should_force_unboxing() {
        return None;
    }
    let instance = invocation.get_instance()?;
    match invocation.d(|d| d.kuna_unboxing_written_as)? {
        UnboxingWrittenAs::Instance => Some(instance),
        UnboxingWrittenAs::CastOperand => {
            Some(instance.try_as_function()?.get_lst_operands().get(0))
        }
    }
}

/// Whether `operand` was written as something else than itself, an unboxing
/// call that is not written, whose parentheses are decided again after it is.
pub fn written_as_other(operand: Exprent) -> bool {
    unboxing_written_as(operand).is_some()
}

/// Whether the conditional `operands` is written as Kotlin's `x as? T`.
fn safe_cast(operands: Vec<Exprent>) -> bool {
    operands[0]
        .try_as_k_function()
        .is_some_and(|c| c.get_func_type() == FunctionType::Instanceof)
        && operands[1]
            .try_as_k_function()
            .is_some_and(|c| c.get_func_type() == FunctionType::Cast)
        && operands[2]
            .get_expr_type_opt()
            .is_some_and(|t| t.ptr_eq(&VarType::VARTYPE_NULL()))
}
