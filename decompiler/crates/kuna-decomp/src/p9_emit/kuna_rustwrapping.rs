//! Print fixed-width p-code arithmetic without Rust overflow checks.

use crate::architecture::Architecture;
use crate::context::{OpId, VarnodeId};
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;
use crate::kuna_rusttypes::int_name;
use crate::printc::{op_key, tokens, PrintC};
use crate::printlanguage::{Atom, SyntaxHighlight, TagType};
use kuna_base::address::calc_mask;
use kuna_num::opcodes::OpCode;

pub(crate) struct WrappingPlan {
    method: &'static str,
    width: i32,
    signed: bool,
    target: &'static str,
    inputs: [Option<Operand>; 2],
}

enum Operand {
    Constant(u64),
    Value {
        id: VarnodeId,
        convert: bool,
        context: Option<&'static str>,
    },
}

fn signedness(ty: &Datatype, width: i32) -> Option<bool> {
    if ty.get_size() != width || ty.is_enum_type() {
        return None;
    }
    match ty.get_metatype() {
        type_metatype::TYPE_INT => Some(!(ty.is_char_print() && ty.is_core_type())),
        type_metatype::TYPE_UINT | type_metatype::TYPE_UNKNOWN => Some(false),
        _ => None,
    }
}

impl PrintC {
    pub(crate) fn rust_wrapping_plan(
        &self,
        fd: &Funcdata,
        arch: &Architecture,
        op: OpId,
    ) -> Option<WrappingPlan> {
        let operation = fd.obank().get(op)?;
        let (method, count) = match operation.code() {
            OpCode::CPUI_INT_ADD => ("wrapping_add", 2),
            OpCode::CPUI_INT_SUB => ("wrapping_sub", 2),
            OpCode::CPUI_INT_MULT => ("wrapping_mul", 2),
            OpCode::CPUI_INT_2COMP => ("wrapping_neg", 1),
            _ => return None,
        };
        if operation.num_input() != count {
            return None;
        }
        let out = operation.get_out()?;
        let width = fd.vbank().get(out)?.get_size();
        let signed = signedness(&self.rust_integer_output_type(fd, arch, out), width)?;
        let target = int_name(width, signed)?;
        let mut inputs = [None, None];
        for (slot, input) in inputs.iter_mut().take(count as usize).enumerate() {
            let id = operation.get_in(slot as i32)?;
            let node = fd.vbank().get(id)?;
            if node.get_size() != width || node.is_annotation() {
                return None;
            }
            let source_signed = signedness(&self.rust_integer_value_type(fd, arch, id, op), width)?;
            *input = Some(if node.is_constant() {
                Operand::Constant(node.get_offset() & calc_mask(width))
            } else {
                let convert = source_signed != signed;
                let context = if convert
                    && node.is_implied()
                    && crate::kuna_rustintegers::needs_integer_context(fd, id, op, 8) != Some(false)
                {
                    int_name(width, source_signed)
                } else {
                    None
                };
                Operand::Value {
                    id,
                    convert,
                    context,
                }
            });
        }
        Some(WrappingPlan {
            method,
            width,
            signed,
            target,
            inputs,
        })
    }

    pub(crate) fn op_rust_wrapping(
        &mut self,
        fd: &Funcdata,
        arch: &Architecture,
        op: OpId,
    ) -> bool {
        let Some(plan) = self.rust_wrapping_plan(fd, arch, op) else {
            return false;
        };
        let key = Some(op_key(op));
        self.push_op(&tokens::FUNCTION_CALL, key);
        self.push_atom(&Atom::syntax(
            format!("{}::{}", plan.target, plan.method),
            TagType::FuncToken,
            SyntaxHighlight::funcname_color,
        ));
        if plan.inputs[1].is_some() {
            self.push_op(&tokens::COMMA, key);
        }
        for input in plan.inputs.into_iter().flatten() {
            match input {
                Operand::Constant(bits) => {
                    self.push_constant_ir_fmt_sign(
                        bits,
                        plan.width,
                        op,
                        0,
                        plan.signed && plan.width <= 8,
                    );
                }
                Operand::Value {
                    id,
                    convert,
                    context,
                } => {
                    if convert {
                        self.push_op(self.lang().tok_typecast, key);
                    }
                    if let Some(source) = context {
                        self.push_op(&tokens::FUNCTION_CALL, key);
                        self.push_atom(&Atom::with_op(
                            format!("::core::convert::identity::<{source}>"),
                            TagType::OpToken,
                            SyntaxHighlight::no_color,
                            op_key(op),
                        ));
                    }
                    self.push_vn_ir(fd, arch, id, op);
                    if convert {
                        self.push_atom(&Atom::syntax(
                            plan.target,
                            TagType::TypeToken,
                            SyntaxHighlight::type_color,
                        ));
                    }
                }
            }
        }
        true
    }
}
