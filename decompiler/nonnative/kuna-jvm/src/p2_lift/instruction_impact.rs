//! Port of `org/jetbrains/java/decompiler/code/interpreter/InstructionImpact.java`.

use crate::code_constants::*;
use crate::code_type::CodeType;
use crate::constant_pool::ConstantPool;
use crate::data_point::DataPoint;
use crate::field_descriptor::FieldDescriptor;
use crate::instruction::Instruction;
use crate::jthrow;
use crate::method_descriptor::MethodDescriptor;
use crate::pooled_constant::PooledConstant;
use crate::var_type::VarType;

type Impact = Option<(Option<&'static [CodeType]>, Option<&'static [CodeType]>)>;

/// `{read, write}` per opcode; `None` marks the opcodes handled one by one.
const STACK_IMPACT: [Impact; 202] = [
    Some((None, None)),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some((None, Some(&[CodeType::Long]))),
    Some((None, Some(&[CodeType::Long]))),
    Some((None, Some(&[CodeType::Float]))),
    Some((None, Some(&[CodeType::Float]))),
    Some((None, Some(&[CodeType::Float]))),
    Some((None, Some(&[CodeType::Double]))),
    Some((None, Some(&[CodeType::Double]))),
    Some((None, Some(&[CodeType::Int]))),
    Some((None, Some(&[CodeType::Int]))),
    None,
    None,
    None,
    Some((None, Some(&[CodeType::Int]))),
    Some((None, Some(&[CodeType::Long]))),
    Some((None, Some(&[CodeType::Float]))),
    Some((None, Some(&[CodeType::Double]))),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some((
        Some(&[CodeType::Object, CodeType::Int]),
        Some(&[CodeType::Int]),
    )),
    Some((
        Some(&[CodeType::Object, CodeType::Int]),
        Some(&[CodeType::Long]),
    )),
    Some((
        Some(&[CodeType::Object, CodeType::Int]),
        Some(&[CodeType::Float]),
    )),
    Some((
        Some(&[CodeType::Object, CodeType::Int]),
        Some(&[CodeType::Double]),
    )),
    None,
    Some((
        Some(&[CodeType::Object, CodeType::Int]),
        Some(&[CodeType::Int]),
    )),
    Some((
        Some(&[CodeType::Object, CodeType::Int]),
        Some(&[CodeType::Int]),
    )),
    Some((
        Some(&[CodeType::Object, CodeType::Int]),
        Some(&[CodeType::Int]),
    )),
    Some((Some(&[CodeType::Int]), None)),
    Some((Some(&[CodeType::Long]), None)),
    Some((Some(&[CodeType::Float]), None)),
    Some((Some(&[CodeType::Double]), None)),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some((
        Some(&[CodeType::Object, CodeType::Int, CodeType::Int]),
        None,
    )),
    Some((
        Some(&[CodeType::Object, CodeType::Int, CodeType::Long]),
        None,
    )),
    Some((
        Some(&[CodeType::Object, CodeType::Int, CodeType::Float]),
        None,
    )),
    Some((
        Some(&[CodeType::Object, CodeType::Int, CodeType::Double]),
        None,
    )),
    Some((
        Some(&[CodeType::Object, CodeType::Int, CodeType::Object]),
        None,
    )),
    Some((
        Some(&[CodeType::Object, CodeType::Int, CodeType::Int]),
        None,
    )),
    Some((
        Some(&[CodeType::Object, CodeType::Int, CodeType::Int]),
        None,
    )),
    Some((
        Some(&[CodeType::Object, CodeType::Int, CodeType::Int]),
        None,
    )),
    Some((Some(&[CodeType::Any]), None)),
    Some((Some(&[CodeType::Any, CodeType::Any]), None)),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some((
        Some(&[CodeType::Int, CodeType::Int]),
        Some(&[CodeType::Int]),
    )),
    Some((
        Some(&[CodeType::Long, CodeType::Long]),
        Some(&[CodeType::Long]),
    )),
    Some((
        Some(&[CodeType::Float, CodeType::Float]),
        Some(&[CodeType::Float]),
    )),
    Some((
        Some(&[CodeType::Double, CodeType::Double]),
        Some(&[CodeType::Double]),
    )),
    Some((
        Some(&[CodeType::Int, CodeType::Int]),
        Some(&[CodeType::Int]),
    )),
    Some((
        Some(&[CodeType::Long, CodeType::Long]),
        Some(&[CodeType::Long]),
    )),
    Some((
        Some(&[CodeType::Float, CodeType::Float]),
        Some(&[CodeType::Float]),
    )),
    Some((
        Some(&[CodeType::Double, CodeType::Double]),
        Some(&[CodeType::Double]),
    )),
    Some((
        Some(&[CodeType::Int, CodeType::Int]),
        Some(&[CodeType::Int]),
    )),
    Some((
        Some(&[CodeType::Long, CodeType::Long]),
        Some(&[CodeType::Long]),
    )),
    Some((
        Some(&[CodeType::Float, CodeType::Float]),
        Some(&[CodeType::Float]),
    )),
    Some((
        Some(&[CodeType::Double, CodeType::Double]),
        Some(&[CodeType::Double]),
    )),
    Some((
        Some(&[CodeType::Int, CodeType::Int]),
        Some(&[CodeType::Int]),
    )),
    Some((
        Some(&[CodeType::Long, CodeType::Long]),
        Some(&[CodeType::Long]),
    )),
    Some((
        Some(&[CodeType::Float, CodeType::Float]),
        Some(&[CodeType::Float]),
    )),
    Some((
        Some(&[CodeType::Double, CodeType::Double]),
        Some(&[CodeType::Double]),
    )),
    Some((
        Some(&[CodeType::Int, CodeType::Int]),
        Some(&[CodeType::Int]),
    )),
    Some((
        Some(&[CodeType::Long, CodeType::Long]),
        Some(&[CodeType::Long]),
    )),
    Some((
        Some(&[CodeType::Float, CodeType::Float]),
        Some(&[CodeType::Float]),
    )),
    Some((
        Some(&[CodeType::Double, CodeType::Double]),
        Some(&[CodeType::Double]),
    )),
    Some((Some(&[CodeType::Int]), Some(&[CodeType::Int]))),
    Some((Some(&[CodeType::Long]), Some(&[CodeType::Long]))),
    Some((Some(&[CodeType::Float]), Some(&[CodeType::Float]))),
    Some((Some(&[CodeType::Double]), Some(&[CodeType::Double]))),
    Some((
        Some(&[CodeType::Int, CodeType::Int]),
        Some(&[CodeType::Int]),
    )),
    Some((
        Some(&[CodeType::Long, CodeType::Int]),
        Some(&[CodeType::Long]),
    )),
    Some((
        Some(&[CodeType::Int, CodeType::Int]),
        Some(&[CodeType::Int]),
    )),
    Some((
        Some(&[CodeType::Long, CodeType::Int]),
        Some(&[CodeType::Long]),
    )),
    Some((
        Some(&[CodeType::Int, CodeType::Int]),
        Some(&[CodeType::Int]),
    )),
    Some((
        Some(&[CodeType::Long, CodeType::Int]),
        Some(&[CodeType::Long]),
    )),
    Some((
        Some(&[CodeType::Int, CodeType::Int]),
        Some(&[CodeType::Int]),
    )),
    Some((
        Some(&[CodeType::Long, CodeType::Long]),
        Some(&[CodeType::Long]),
    )),
    Some((
        Some(&[CodeType::Int, CodeType::Int]),
        Some(&[CodeType::Int]),
    )),
    Some((
        Some(&[CodeType::Long, CodeType::Long]),
        Some(&[CodeType::Long]),
    )),
    Some((
        Some(&[CodeType::Int, CodeType::Int]),
        Some(&[CodeType::Int]),
    )),
    Some((
        Some(&[CodeType::Long, CodeType::Long]),
        Some(&[CodeType::Long]),
    )),
    Some((None, None)),
    Some((Some(&[CodeType::Int]), Some(&[CodeType::Long]))),
    Some((Some(&[CodeType::Int]), Some(&[CodeType::Float]))),
    Some((Some(&[CodeType::Int]), Some(&[CodeType::Double]))),
    Some((Some(&[CodeType::Long]), Some(&[CodeType::Int]))),
    Some((Some(&[CodeType::Long]), Some(&[CodeType::Float]))),
    Some((Some(&[CodeType::Long]), Some(&[CodeType::Double]))),
    Some((Some(&[CodeType::Float]), Some(&[CodeType::Int]))),
    Some((Some(&[CodeType::Float]), Some(&[CodeType::Long]))),
    Some((Some(&[CodeType::Float]), Some(&[CodeType::Double]))),
    Some((Some(&[CodeType::Double]), Some(&[CodeType::Int]))),
    Some((Some(&[CodeType::Double]), Some(&[CodeType::Long]))),
    Some((Some(&[CodeType::Double]), Some(&[CodeType::Float]))),
    Some((Some(&[CodeType::Int]), Some(&[CodeType::Int]))),
    Some((Some(&[CodeType::Int]), Some(&[CodeType::Int]))),
    Some((Some(&[CodeType::Int]), Some(&[CodeType::Int]))),
    Some((
        Some(&[CodeType::Long, CodeType::Long]),
        Some(&[CodeType::Int]),
    )),
    Some((
        Some(&[CodeType::Float, CodeType::Float]),
        Some(&[CodeType::Int]),
    )),
    Some((
        Some(&[CodeType::Float, CodeType::Float]),
        Some(&[CodeType::Int]),
    )),
    Some((
        Some(&[CodeType::Double, CodeType::Double]),
        Some(&[CodeType::Int]),
    )),
    Some((
        Some(&[CodeType::Double, CodeType::Double]),
        Some(&[CodeType::Int]),
    )),
    Some((Some(&[CodeType::Int]), None)),
    Some((Some(&[CodeType::Int]), None)),
    Some((Some(&[CodeType::Int]), None)),
    Some((Some(&[CodeType::Int]), None)),
    Some((Some(&[CodeType::Int]), None)),
    Some((Some(&[CodeType::Int]), None)),
    Some((Some(&[CodeType::Int, CodeType::Int]), None)),
    Some((Some(&[CodeType::Int, CodeType::Int]), None)),
    Some((Some(&[CodeType::Int, CodeType::Int]), None)),
    Some((Some(&[CodeType::Int, CodeType::Int]), None)),
    Some((Some(&[CodeType::Int, CodeType::Int]), None)),
    Some((Some(&[CodeType::Int, CodeType::Int]), None)),
    Some((Some(&[CodeType::Object, CodeType::Object]), None)),
    Some((Some(&[CodeType::Object, CodeType::Object]), None)),
    Some((None, None)),
    Some((None, Some(&[CodeType::Address]))),
    Some((None, None)),
    Some((Some(&[CodeType::Int]), None)),
    Some((Some(&[CodeType::Int]), None)),
    Some((Some(&[CodeType::Int]), None)),
    Some((Some(&[CodeType::Long]), None)),
    Some((Some(&[CodeType::Float]), None)),
    Some((Some(&[CodeType::Double]), None)),
    Some((Some(&[CodeType::Object]), None)),
    Some((None, None)),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some((Some(&[CodeType::Object]), Some(&[CodeType::Int]))),
    None,
    None,
    None,
    Some((Some(&[CodeType::Object]), None)),
    Some((Some(&[CodeType::Object]), None)),
    None,
    None,
    Some((Some(&[CodeType::Object]), None)),
    Some((Some(&[CodeType::Object]), None)),
    Some((None, None)),
    Some((None, Some(&[CodeType::Address]))),
];

const ARR_TYPE: [CodeType; 8] = [
    CodeType::Boolean,
    CodeType::Char,
    CodeType::Float,
    CodeType::Double,
    CodeType::Byte,
    CodeType::Short,
    CodeType::Int,
    CodeType::Long,
];

pub struct InstructionImpact;

impl InstructionImpact {
    pub fn step_types(data: &mut DataPoint, instr: Instruction, pool: ConstantPool) {
        let arr = STACK_IMPACT[instr.opcode() as usize];

        if let Some((read, write)) = arr {
            if let Some(read) = read {
                let mut depth = 0;
                for type_ in read {
                    depth += 1;
                    if *type_ == CodeType::Long || *type_ == CodeType::Double {
                        depth += 1;
                    }
                }

                data.get_stack().remove_multiple(depth);
            }

            if let Some(write) = write {
                for type_ in write {
                    data.get_stack().push(VarType::new(*type_));
                    if *type_ == CodeType::Long || *type_ == CodeType::Double {
                        data.get_stack().push(VarType::new(CodeType::Group2empty));
                    }
                }
            }
        } else {
            InstructionImpact::process_special_instructions(data, instr, pool);
        }
    }

    fn process_special_instructions(data: &mut DataPoint, instr: Instruction, pool: ConstantPool) {
        let opcode = instr.opcode();
        match opcode {
            OPC_ACONST_NULL => {
                data.get_stack()
                    .push(VarType::new_array_dim_value(CodeType::Null, 0, None));
            }
            OPC_LDC | OPC_LDC_W | OPC_LDC2_W => {
                let constant = pool.get_constant(instr.operand(0)).expect("constant");
                let stack = data.get_stack();
                match pooled_type(&constant) {
                    CONSTANT_INTEGER => stack.push(VarType::new(CodeType::Int)),
                    CONSTANT_FLOAT => stack.push(VarType::new(CodeType::Float)),
                    CONSTANT_LONG => {
                        stack.push(VarType::new(CodeType::Long));
                        stack.push(VarType::new(CodeType::Group2empty));
                    }
                    CONSTANT_DOUBLE => {
                        stack.push(VarType::new(CodeType::Double));
                        stack.push(VarType::new(CodeType::Group2empty));
                    }
                    CONSTANT_STRING => stack.push(VarType::new_array_dim_value(
                        CodeType::Object,
                        0,
                        Some("java/lang/String"),
                    )),
                    CONSTANT_CLASS => stack.push(VarType::new_array_dim_value(
                        CodeType::Object,
                        0,
                        Some("java/lang/Class"),
                    )),
                    CONSTANT_METHOD_HANDLE => {
                        let PooledConstant::Link(link) = &constant else {
                            jthrow!(ClassCastException; "PrimitiveConstant cannot be cast to LinkConstant")
                        };
                        stack.push(VarType::new_signature(&link.descriptor));
                    }
                    CONSTANT_DYNAMIC => {
                        let ck = pool.get_link_constant(instr.operand(0)).expect("constant");
                        let fd = FieldDescriptor::parse_descriptor(&ck.descriptor);
                        if fd.type_.type_ != CodeType::Void {
                            stack.push(fd.type_.clone());
                            if fd.type_.stack_size == 2 {
                                stack.push(VarType::new(CodeType::Group2empty));
                            }
                        }
                    }
                    CONSTANT_METHOD_TYPE => stack.push(VarType::new_array_dim_value(
                        CodeType::Object,
                        0,
                        Some("java/lang/invoke/MethodType"),
                    )),
                    other => jthrow!(IllegalStateException; "Unhandled constant type: {}", other),
                }
            }
            OPC_ALOAD => {
                let var1 = data.get_variable(instr.operand(0));
                data.get_stack().push(var1);
            }
            OPC_AALOAD => {
                let var1 = data.get_stack().pop_count(2).expect("stack");
                data.get_stack().push(VarType::new_array_dim_value(
                    var1.type_,
                    var1.array_dim.wrapping_sub(1),
                    var1.value.as_deref(),
                ));
            }
            OPC_ASTORE => {
                let value = data.get_stack().pop();
                data.set_variable(instr.operand(0), value);
            }
            OPC_DUP | OPC_DUP_X1 | OPC_DUP_X2 => {
                let depth1 = 88 - opcode;
                let stack = data.get_stack();
                let top = stack.get_by_offset(-1);
                stack.insert_by_offset(depth1, top);
            }
            OPC_DUP2 | OPC_DUP2_X1 | OPC_DUP2_X2 => {
                let depth2 = 90 - opcode;
                let stack = data.get_stack();
                let second = stack.get_by_offset(-2);
                stack.insert_by_offset(depth2, second);
                let top = stack.get_by_offset(-1);
                stack.insert_by_offset(depth2, top);
            }
            OPC_SWAP => {
                let stack = data.get_stack();
                let var1 = stack.pop();
                stack.insert_by_offset(-1, var1);
            }
            OPC_GETFIELD | OPC_GETSTATIC => {
                if opcode == OPC_GETFIELD {
                    data.get_stack().pop();
                }
                let ck = pool.get_link_constant(instr.operand(0)).expect("constant");
                let var1 = VarType::new_signature(&ck.descriptor);
                let stack = data.get_stack();
                let stack_size = var1.stack_size;
                stack.push(var1);
                if stack_size == 2 {
                    stack.push(VarType::new(CodeType::Group2empty));
                }
            }
            OPC_PUTFIELD | OPC_PUTSTATIC => {
                if opcode == OPC_PUTFIELD {
                    data.get_stack().pop();
                }
                let ck = pool.get_link_constant(instr.operand(0)).expect("constant");
                let var1 = VarType::new_signature(&ck.descriptor);
                data.get_stack().pop_count(var1.stack_size);
            }
            OPC_INVOKEVIRTUAL | OPC_INVOKESPECIAL | OPC_INVOKEINTERFACE | OPC_INVOKESTATIC
            | OPC_INVOKEDYNAMIC => {
                if opcode != OPC_INVOKESTATIC && opcode != OPC_INVOKEDYNAMIC {
                    data.get_stack().pop();
                }
                if opcode != OPC_INVOKEDYNAMIC || instr.bytecode_version().has_invoke_dynamic() {
                    let ck = pool.get_link_constant(instr.operand(0)).expect("constant");
                    let md = MethodDescriptor::parse_descriptor(&ck.descriptor);
                    let stack = data.get_stack();
                    for param in md.params.iter() {
                        stack.pop_count(param.stack_size);
                    }
                    if md.ret.type_ != CodeType::Void {
                        stack.push(md.ret.clone());
                        if md.ret.stack_size == 2 {
                            stack.push(VarType::new(CodeType::Group2empty));
                        }
                    }
                }
            }
            OPC_NEW => {
                let cn = pool
                    .get_primitive_constant(instr.operand(0))
                    .expect("constant");
                data.get_stack().push(VarType::new_array_dim_value(
                    CodeType::Object,
                    0,
                    Some(&cn.get_string()),
                ));
            }
            OPC_NEWARRAY => {
                let stack = data.get_stack();
                stack.pop();
                stack.push(
                    VarType::new_array_dim(ARR_TYPE[(instr.operand(0) - 4) as usize], 1)
                        .resize_array_dim(1),
                );
            }
            OPC_ATHROW => {
                let stack = data.get_stack();
                let var1 = stack.pop();
                stack.clear();
                stack.push(var1);
            }
            OPC_CHECKCAST | OPC_INSTANCEOF => {
                data.get_stack().pop();
                let cn = pool
                    .get_primitive_constant(instr.operand(0))
                    .expect("constant");
                data.get_stack().push(VarType::new_array_dim_value(
                    CodeType::Object,
                    0,
                    Some(&cn.get_string()),
                ));
            }
            OPC_ANEWARRAY | OPC_MULTIANEWARRAY => {
                let dimensions = if opcode == OPC_ANEWARRAY {
                    1
                } else {
                    instr.operand(1)
                };
                data.get_stack().pop_count(dimensions);
                let cn = pool
                    .get_primitive_constant(instr.operand(0))
                    .expect("constant");
                if cn.is_array {
                    let var1 =
                        VarType::new_array_dim_value(CodeType::Object, 0, Some(&cn.get_string()));
                    let var1 = var1.resize_array_dim(var1.array_dim.wrapping_add(dimensions));
                    data.get_stack().push(var1);
                } else {
                    data.get_stack().push(VarType::new_array_dim_value(
                        CodeType::Object,
                        dimensions,
                        Some(&cn.get_string()),
                    ));
                }
            }
            _ => {}
        }
    }
}

/// `PooledConstant.type`.
fn pooled_type(constant: &PooledConstant) -> i32 {
    match constant {
        PooledConstant::Primitive(p) => p.type_,
        PooledConstant::Link(l) => l.type_,
    }
}
