//! Fix `synthetic_null_check_shape`: a discarded `requireNonNull`/`getClass`
//! call is removed only when its instructions are one of javac's two null-check
//! shapes; a check of `x` before `x.CONST` only when a loaded class declares
//! that constant field, and the constant is then written as that field's read.

use crate::code_constants::*;
use crate::code_type::CodeType;
use crate::decompiler_context::DecompilerContext;
use crate::exprent::Exprent;
use crate::field_descriptor::FieldDescriptor;
use crate::field_exprent::FieldExprent;
use crate::instruction_sequence::InstructionSequence;
use crate::kuna_static_types::param_type;
use crate::primitive_constant::ConstValue;
use crate::struct_field::StructField;
use crate::struct_general_attribute::StructGeneralAttribute;
use crate::var_type::VarType;

/// The instructions of a discarded `requireNonNull`/`getClass` call that
/// match one of javac's null checks.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Shape {
    /// The call checks a `dup` of a value left on the stack.
    Dup,
    /// A non-`null` constant is loaded next, at the bytecode offset given, as
    /// for `x.CONST`.
    Constant(Load, i32),
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

/// The shape of the null check whose result the `pop` at `i` in `seq` discards;
/// `next_offset` is the bytecode offset of the instruction after the `pop`.
pub fn shape(seq: InstructionSequence, i: i32, next_offset: i32) -> Option<Shape> {
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
    Some(Shape::Constant(load, next_offset))
}

/// Whether a check of `target` in `shape` is javac's: a `Dup` always; a
/// `Constant` where `target`'s class, or a superclass, declares an instance
/// field whose constant value is the one loaded.
pub fn is_javac_check(shape: Option<Shape>, target: Exprent) -> bool {
    match shape {
        None => false,
        Some(Shape::Dup) => true,
        Some(Shape::Constant(load, _)) => constant_field_read(load, target).is_some(),
    }
}

/// The constant instance field of `target`'s class (or a superclass) whose
/// value `load` loads: its declaring class and the field.
fn constant_field_read(load: Load, target: Exprent) -> Option<(String, StructField)> {
    let t = static_type(target);
    if t.type_ != CodeType::Object || t.array_dim != 0 {
        return None;
    }
    constant_field(t.value_str(), &loaded_value(load)?)
}

/// After javac's check of `target` in `shape` is removed: true when the shape
/// is not `Constant`; else whether the constant it guards in `next` was found
/// and written as the field read javac compiled, `target.CONST`.
pub fn restore_field_read(shape: Option<Shape>, target: Exprent, next: Exprent) -> bool {
    let Some(Shape::Constant(load, offset)) = shape else {
        return true;
    };
    let Some((_, field)) = constant_field_read(load, target) else {
        return false;
    };
    if offset < 0 {
        return false;
    }
    let mut stack = vec![next];
    while let Some(parent) = stack.pop() {
        for child in parent.get_all_exprents() {
            let loaded = child.try_as_const().is_some()
                && child.get_bytecode().is_some_and(|b| b.get(offset as usize));
            if loaded {
                let read = FieldExprent::new(
                    field.get_name(),
                    static_type(target).value_str().to_string(),
                    false,
                    Some(target),
                    FieldDescriptor::parse_descriptor(&field.get_descriptor()),
                    child.get_bytecode().as_ref(),
                );
                parent.replace_exprent(child, read.0);
                return true;
            }
            stack.push(child);
        }
    }
    false
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

/// The first instance field of `class` or a superclass whose constant value
/// is `value`, with its declaring class; a class that is not loaded is not
/// known to declare one.
fn constant_field(class: &str, value: &ConstValue) -> Option<(String, StructField)> {
    let context = DecompilerContext::get_struct_context();
    let mut name = class.to_string();
    for _ in 0..64 {
        let cl = context.get_class(&name)?;
        let pool = cl.get_pool();
        let found = cl.get_fields().to_vec().into_iter().find(|f| {
            !f.has_modifier(ACC_STATIC)
                && f.get_attribute(StructGeneralAttribute::ATTRIBUTE_CONSTANT_VALUE)
                    .and_then(|attr| pool.get_primitive_constant(attr.get_index()))
                    .and_then(|constant| constant.value.clone())
                    .is_some_and(|field_value| field_value.equals(value))
        });
        if let Some(field) = found {
            return Some((name, field));
        }
        name = cl.super_class().and_then(|s| s.get_string_opt())?.to_string();
    }
    None
}
