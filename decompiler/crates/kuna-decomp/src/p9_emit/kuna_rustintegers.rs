//! Explicit same-width primitive integer conversions for Rust expressions.

use std::rc::Rc;

use crate::architecture::Architecture;
use crate::context::{OpId, VarnodeId};
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;
use crate::printc::PrintC;
use crate::printlanguage::{OpToken, ReversePolish, TokenType};
use kuna_num::opcodes::OpCode;

/// Find the token following a cast through enclosing right-hand expressions.
pub(crate) fn cast_before_angle(stack: &[ReversePolish]) -> bool {
    for entry in stack.iter().rev() {
        match entry.tok.token_type {
            TokenType::Binary if entry.visited == 0 => {
                return entry.tok.print1.starts_with('<') || entry.tok.print1.starts_with('>');
            }
            TokenType::Binary if entry.visited == 1 && !entry.paren => {}
            TokenType::UnaryPrefix if !entry.paren => {}
            _ => return false,
        }
    }
    false
}

fn shape(ty: &Datatype) -> Option<(i32, bool)> {
    if !matches!(ty.get_size(), 1 | 2 | 4 | 8) || ty.is_enum_type() {
        return None;
    }
    let signed = match ty.get_metatype() {
        type_metatype::TYPE_INT => !(ty.is_char_print() && ty.is_core_type()),
        type_metatype::TYPE_UINT | type_metatype::TYPE_UNKNOWN => false,
        _ => return None,
    };
    Some((ty.get_size(), signed))
}

pub(crate) fn needs_integer_context(fd: &Funcdata, value: VarnodeId, op: OpId, depth: u8) -> Option<bool> {
    if depth == 0 { return None; }
    let node = fd.vbank().get(value)?;
    if node.is_constant() {
        let ty = fd.vn_type_read_facing(value, op);
        return Some(shape(&ty).is_some() && !ty.is_char_print());
    }
    if !node.is_implied() { return Some(false); }
    let Some(def) = node.get_def() else { return Some(false); };
    let operation = fd.obank().get(def)?;
    match operation.code() {
        OpCode::CPUI_INT_LEFT | OpCode::CPUI_INT_RIGHT | OpCode::CPUI_INT_SRIGHT
        | OpCode::CPUI_INT_NEGATE | OpCode::CPUI_INT_2COMP =>
            needs_integer_context(fd, operation.get_in(0)?, def, depth - 1),
        OpCode::CPUI_INT_ADD | OpCode::CPUI_INT_SUB | OpCode::CPUI_INT_MULT
        | OpCode::CPUI_INT_AND | OpCode::CPUI_INT_OR | OpCode::CPUI_INT_XOR
        | OpCode::CPUI_INT_DIV | OpCode::CPUI_INT_SDIV
        | OpCode::CPUI_INT_REM | OpCode::CPUI_INT_SREM => {
            let left = needs_integer_context(fd, operation.get_in(0)?, def, depth - 1)?;
            let right = needs_integer_context(fd, operation.get_in(1)?, def, depth - 1)?;
            Some(left && right)
        }
        _ => Some(false),
    }
}

pub(crate) struct IntegerPlan {
    inputs: [VarnodeId; 2],
    target: Rc<Datatype>,
}

impl PrintC {
    fn rust_integer_declared_type(
        &self,
        fd: &Funcdata,
        arch: &Architecture,
        value: VarnodeId,
    ) -> Option<Rc<Datatype>> {
        let node = fd.vbank().get(value)?;
        if node.is_implied() || node.is_constant() {
            return None;
        }
        if let Some(ty) = node.get_high()
            .and_then(|high| self.declared_local(high))
            .and_then(|local| local.ty.as_ref())
            .filter(|ty| ty.get_size() == node.get_size())
        {
            return Some(Rc::clone(ty));
        }
        crate::printc::declared_variable_type(fd, value)
    }

    pub(crate) fn rust_integer_value_type(
        &self,
        fd: &Funcdata,
        arch: &Architecture,
        value: VarnodeId,
        op: OpId,
    ) -> Rc<Datatype> {
        self.rust_integer_declared_type(fd, arch, value)
            .unwrap_or_else(|| fd.vn_type_read_facing(value, op))
    }

    pub(crate) fn rust_integer_output_type(
        &self,
        fd: &Funcdata,
        arch: &Architecture,
        value: VarnodeId,
    ) -> Rc<Datatype> {
        self.rust_integer_declared_type(fd, arch, value)
            .unwrap_or_else(|| fd.vn_type_def_facing(value))
    }

    pub(crate) fn push_rust_integer_as(
        &mut self,
        fd: &Funcdata,
        arch: &Architecture,
        value: VarnodeId,
        op: OpId,
        target: &Rc<Datatype>,
    ) -> bool {
        self.push_rust_integer_conversion(fd, arch, value, op, target, false)
    }

    pub(crate) fn push_rust_integer_conversion(
        &mut self,
        fd: &Funcdata,
        arch: &Architecture,
        value: VarnodeId,
        op: OpId,
        target: &Rc<Datatype>,
        explicit_cast: bool,
    ) -> bool {
        let Some(target_shape) = shape(target) else { return false; };
        let Some(node) = fd.vbank().get(value) else { return false; };
        if node.get_size() != target_shape.0 { return false; }
        let source = self.rust_integer_value_type(fd, arch, value, op);
        let Some(source_shape) = shape(&source) else { return false; };
        if source_shape.0 != target_shape.0 { return false; }
        let context = if node.is_implied() {
            let Some(context) = needs_integer_context(fd, value, op, 8) else { return false; };
            context
        } else { false };
        if source_shape == target_shape && !(explicit_cast && context) { return false; }
        self.push_cast_open(target, op);
        if context {
            self.push_op(&crate::printc::tokens::FUNCTION_CALL, Some(crate::printc::op_key(op)));
            self.push_atom(&crate::printlanguage::Atom::with_op(
                format!("::core::convert::identity::<{}{}>", if source_shape.1 { "i" } else { "u" }, source_shape.0 * 8),
                crate::printlanguage::TagType::OpToken,
                crate::printlanguage::SyntaxHighlight::no_color,
                crate::printc::op_key(op),
            ));
        }
        if node.is_constant() { self.push_cast_open(&source, op); }
        self.push_vn_ir(fd, arch, value, op);
        if node.is_constant() { self.push_cast_close(&source); }
        self.push_cast_close(target);
        true
    }

    pub(crate) fn rust_integer_binary_plan(
        &self,
        fd: &Funcdata,
        arch: &Architecture,
        op: OpId,
    ) -> Option<IntegerPlan> {
        let operation = fd.obank().get(op)?;
        if !matches!(operation.code(), OpCode::CPUI_INT_ADD | OpCode::CPUI_INT_SUB
            | OpCode::CPUI_INT_MULT | OpCode::CPUI_INT_AND | OpCode::CPUI_INT_OR
            | OpCode::CPUI_INT_XOR) || operation.num_input() != 2
        {
            return None;
        }
        let out = operation.get_out()?;
        let target = self.rust_integer_output_type(fd, arch, out);
        let target_shape = shape(&target)?;
        if fd.vbank().get(out)?.get_size() != target_shape.0 { return None; }
        let inputs = [operation.get_in(0)?, operation.get_in(1)?];
        let mut needed = false;
        for input in inputs {
            let source_shape = shape(&self.rust_integer_value_type(fd, arch, input, op))?;
            if source_shape.0 != target_shape.0 || fd.vbank().get(input)?.get_size() != target_shape.0 {
                return None;
            }
            needed |= source_shape != target_shape;
        }
        needed.then_some(IntegerPlan { inputs, target })
    }

    pub(crate) fn op_rust_integer_binary(
        &mut self,
        fd: &Funcdata,
        arch: &Architecture,
        token: &'static OpToken,
        op: OpId,
    ) -> bool {
        let Some(plan) = self.rust_integer_binary_plan(fd, arch, op) else { return false; };
        self.push_op(token, Some(crate::printc::op_key(op)));
        for input in plan.inputs {
            if !self.push_rust_integer_as(fd, arch, input, op, &plan.target) {
                self.push_vn_ir(fd, arch, input, op);
            }
        }
        true
    }
}
