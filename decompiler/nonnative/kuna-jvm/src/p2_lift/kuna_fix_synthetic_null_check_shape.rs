//! kuna fix `synthetic_null_check_shape` (`kuna_fixes`): javac null-checks a
//! value with `invokestatic Objects.requireNonNull; pop` (`invokevirtual
//! getClass; pop` before Java 9) where the source has no call: on a `dup` of
//! the outer instance of `x.new Inner()`, the receiver of `x::m` or a pattern
//! switch's selector, which stays on the stack for the construct (and the
//! construct checks it again when written back), and on `x` of `x.CONST`, an
//! instance field with a constant value read through `x`, whose constant is
//! loaded next. `SimplifyExprentsHelper.isQualifiedNewGetClass` removes a
//! discarded `requireNonNull`/`getClass` statement when `ExprProcessor`
//! flagged it (a constant, `null` or an `astore` next) or when the next
//! statement creates a nested class taking the checked value (or one of its
//! type) first, so the source's own `Objects.requireNonNull(o); return new
//! Holder(o);`, `Objects.requireNonNull(o); return null;` or
//! `Objects.requireNonNull(bytes); char[] r = new char[2 * bytes.length];` lost
//! its check. With the fix it removes only a call `ExprProcessor` found in one
//! of javac's two shapes, the second only where the checked value's class can
//! have an instance field with a constant value.

use crate::code_constants::*;
use crate::code_type::CodeType;
use crate::decompiler_context::DecompilerContext;
use crate::exprent::Exprent;
use crate::instruction_sequence::InstructionSequence;
use crate::kuna_static_types::param_type;
use crate::struct_general_attribute::StructGeneralAttribute;
use crate::var_type::VarType;

/// The instructions of a discarded `requireNonNull`/`getClass` call that
/// match one of javac's null checks.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Shape {
    /// The call checks a `dup` of a value left on the stack.
    Dup,
    /// A non-`null` constant is loaded next, as for `x.CONST`.
    Constant,
}

/// The shape of the null check whose result the `pop` at `i` in `seq` discards.
pub fn shape(seq: InstructionSequence, i: i32) -> Option<Shape> {
    if i < 1 || !(OPC_INVOKEVIRTUAL..=OPC_INVOKESTATIC).contains(&seq.get_instr(i - 1).opcode()) {
        return None;
    }
    if i >= 2 && seq.get_instr(i - 2).opcode() == OPC_DUP {
        return Some(Shape::Dup);
    }
    (i + 1 < seq.length() && (OPC_ICONST_M1..=OPC_LDC2_W).contains(&seq.get_instr(i + 1).opcode()))
        .then_some(Shape::Constant)
}

/// Whether a check of `target` in `shape` is javac's: a `Dup` always; a
/// `Constant` where `target`'s class, or a superclass, declares an instance
/// field with a constant value, or is outside the input.
pub fn is_javac_check(shape: Option<Shape>, target: Exprent) -> bool {
    match shape {
        None => false,
        Some(Shape::Dup) => true,
        Some(Shape::Constant) => {
            let t = static_type(target);
            t.type_ == CodeType::Object
                && t.array_dim == 0
                && may_have_constant_field(t.value_str())
        }
    }
}

/// The type `e` is declared with: a parameter's, a local's from the
/// LocalVariableTable, a field's; else its expression type.
pub fn static_type(e: Exprent) -> VarType {
    if let Some(var) = e.try_as_var() {
        if let Some(Some(t)) = param_type(var) {
            return t;
        }
        if let Some(lvt) = var.get_lvt() {
            return lvt.get_var_type();
        }
    }
    if let Some(field) = e.try_as_field() {
        return field.get_descriptor().type_.clone();
    }
    e.get_expr_type()
}

fn may_have_constant_field(class: &str) -> bool {
    let context = DecompilerContext::get_struct_context();
    let mut name = class.to_string();
    for _ in 0..64 {
        let Some(cl) = context.get_class(&name) else {
            return true;
        };
        let constant = cl.get_fields().to_vec().into_iter().any(|f| {
            !f.has_modifier(ACC_STATIC)
                && f.get_attribute(StructGeneralAttribute::ATTRIBUTE_CONSTANT_VALUE)
                    .is_some()
        });
        if constant {
            return true;
        }
        match cl.super_class().and_then(|s| s.get_string_opt()) {
            Some(s) => name = s.to_string(),
            None => return false,
        }
    }
    true
}
