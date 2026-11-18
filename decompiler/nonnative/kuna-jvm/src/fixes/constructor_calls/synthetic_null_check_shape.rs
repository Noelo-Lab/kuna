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
//! of javac's two shapes, the second only where the checked value's class or a
//! superclass declares an instance field whose constant value is the one loaded
//! next (`Objects.requireNonNull(b); return 5;` for a `b` whose class has
//! `final int K = 3` keeps its check). A class that is not loaded is not
//! known to declare one, so the check is kept: without a JDK the walk ended at
//! `java.lang.String` or `java.lang.Object` and deleted `Objects.requireNonNull(s);
//! return 5;`'s check. A check kept where javac wrote `x.CONST` still means
//! what the bytecode does.

use crate::code_constants::*;
use crate::code_type::CodeType;
use crate::decompiler_context::DecompilerContext;
use crate::exprent::Exprent;
use crate::instruction_sequence::InstructionSequence;
use crate::kuna_static_types::param_type;
use crate::primitive_constant::ConstValue;
use crate::struct_general_attribute::StructGeneralAttribute;
use crate::var_type::VarType;

/// The instructions of a discarded `requireNonNull`/`getClass` call that
/// match one of javac's null checks.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Shape {
    /// The call checks a `dup` of a value left on the stack.
    Dup,
    /// A non-`null` constant is loaded next, as for `x.CONST`.
    Constant(Load),
}

/// The constant an instruction loads: its value, or its pool index (`ldc`)
/// in the current class's pool.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Load {
    Int(i32),
    Long(i64),
    Float(u32),
    Double(u64),
    Pool(i32),
}

/// The shape of the null check whose result the `pop` at `i` in `seq` discards.
pub fn shape(seq: InstructionSequence, i: i32) -> Option<Shape> {
    if i < 1 || !(OPC_INVOKEVIRTUAL..=OPC_INVOKESTATIC).contains(&seq.get_instr(i - 1).opcode()) {
        return None;
    }
    if i >= 2 && seq.get_instr(i - 2).opcode() == OPC_DUP {
        return Some(Shape::Dup);
    }
    if i + 1 >= seq.length() {
        return None;
    }
    let next = seq.get_instr(i + 1);
    let opcode = next.opcode();
    let load = match opcode {
        OPC_BIPUSH | OPC_SIPUSH => Load::Int(next.operand(0)),
        OPC_LCONST_0 | OPC_LCONST_1 => Load::Long((opcode - OPC_LCONST_0) as i64),
        OPC_FCONST_0..=OPC_FCONST_2 => Load::Float(((opcode - OPC_FCONST_0) as f32).to_bits()),
        OPC_DCONST_0 | OPC_DCONST_1 => Load::Double(((opcode - OPC_DCONST_0) as f64).to_bits()),
        OPC_LDC | OPC_LDC_W | OPC_LDC2_W => Load::Pool(next.operand(0)),
        _ => return None,
    };
    Some(Shape::Constant(load))
}

/// Whether a check of `target` in `shape` is javac's: a `Dup` always; a
/// `Constant` where `target`'s class, or a superclass, declares an instance
/// field whose constant value is the one loaded.
pub fn is_javac_check(shape: Option<Shape>, target: Exprent) -> bool {
    match shape {
        None => false,
        Some(Shape::Dup) => true,
        Some(Shape::Constant(load)) => {
            let t = static_type(target);
            t.type_ == CodeType::Object
                && t.array_dim == 0
                && loaded_value(load).is_some_and(|v| constant_field_holds(t.value_str(), &v))
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

/// The value `load` loads, from the current class's pool for an `ldc`.
fn loaded_value(load: Load) -> Option<ConstValue> {
    Some(match load {
        Load::Int(i) => ConstValue::Integer(i),
        Load::Long(l) => ConstValue::Long(l),
        Load::Float(f) => ConstValue::Float(f32::from_bits(f)),
        Load::Double(d) => ConstValue::Double(f64::from_bits(d)),
        Load::Pool(index) => DecompilerContext::get_current_class()?
            .get_pool()
            .get_primitive_constant(index)?
            .value
            .clone()?,
    })
}

/// Whether `class` or a superclass declares an instance field whose
/// constant value is `value`; a class that is not loaded is not known to.
fn constant_field_holds(class: &str, value: &ConstValue) -> bool {
    let context = DecompilerContext::get_struct_context();
    let mut name = class.to_string();
    for _ in 0..64 {
        let Some(cl) = context.get_class(&name) else {
            return false;
        };
        let pool = cl.get_pool();
        let holds = cl.get_fields().to_vec().into_iter().any(|f| {
            !f.has_modifier(ACC_STATIC)
                && f.get_attribute(StructGeneralAttribute::ATTRIBUTE_CONSTANT_VALUE)
                    .and_then(|attr| pool.get_primitive_constant(attr.get_index()))
                    .and_then(|constant| constant.value.clone())
                    .is_some_and(|field_value| field_value.equals(value))
        });
        if holds {
            return true;
        }
        match cl.super_class().and_then(|s| s.get_string_opt()) {
            Some(s) => name = s.to_string(),
            None => return false,
        }
    }
    false
}
