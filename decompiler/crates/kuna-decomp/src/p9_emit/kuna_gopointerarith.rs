//! Preserve a live Go pointer when byte arithmetic survives as an integer add.

use std::rc::Rc;

use crate::architecture::Architecture;
use crate::context::{OpId, VarnodeId};
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;
use crate::printc::{op_key, tokens, vn_key, PrintC};
use crate::printlanguage::{Atom, SyntaxHighlight, TagType};
use kuna_num::opcodes::OpCode;

pub(crate) struct Operand {
    pub(crate) value: VarnodeId,
    pub(crate) reader: OpId,
}

pub(crate) struct Addition {
    pub(crate) base: Operand,
    pub(crate) offset: Operand,
    pub(crate) operation: OpId,
    negative_constant: Option<i64>,
}

fn native_pointer(ty: &Datatype, size: i32) -> bool {
    matches!(size, 4 | 8)
        && ty.get_size() == size
        && matches!(
            ty.get_metatype(),
            type_metatype::TYPE_PTR | type_metatype::TYPE_PTRREL
        )
        && ty.get_word_size() == Some(1)
        && !crate::kuna_typequal::is_volatile(ty)
}

fn peel(fd: &Funcdata, mut value: VarnodeId, mut reader: OpId, size: i32) -> Option<Operand> {
    for _ in 0..8 {
        let node = fd.vbank().get(value)?;
        if node.get_size() != size
            || node.is_volatile()
            || node.is_annotation()
            || node.is_persist()
        {
            return None;
        }
        let ty = fd.vn_type_read_facing(value, reader);
        if crate::kuna_typequal::is_volatile(&ty)
            || !matches!(
                ty.get_metatype(),
                type_metatype::TYPE_PTR
                    | type_metatype::TYPE_PTRREL
                    | type_metatype::TYPE_INT
                    | type_metatype::TYPE_UINT
                    | type_metatype::TYPE_UNKNOWN
            )
        {
            return None;
        }
        if !node.is_implied() || node.is_constant() {
            return Some(Operand { value, reader });
        }
        let id = node.get_def()?;
        let operation = fd.obank().get(id)?;
        if !matches!(operation.code(), OpCode::CPUI_COPY | OpCode::CPUI_CAST)
            || operation.num_input() != 1
        {
            return Some(Operand { value, reader });
        }
        value = operation.get_in(0)?;
        reader = id;
    }
    None
}

pub(crate) fn addition(
    fd: &Funcdata,
    value: VarnodeId,
    reader: OpId,
    value_type: impl FnMut(VarnodeId, OpId) -> Rc<Datatype>,
) -> Option<Addition> {
    let size = fd.vbank().get(value)?.get_size();
    let root = peel(fd, value, reader, size)?;
    let node = fd.vbank().get(root.value)?;
    if !node.is_implied() {
        return None;
    }
    operation(fd, node.get_def()?, value_type)
}

fn operation(
    fd: &Funcdata,
    id: OpId,
    mut value_type: impl FnMut(VarnodeId, OpId) -> Rc<Datatype>,
) -> Option<Addition> {
    let operation = fd.obank().get(id)?;
    if operation.code() != OpCode::CPUI_INT_ADD || operation.num_input() != 2 {
        return None;
    }
    let size = fd.vbank().get(operation.get_out()?)?.get_size();
    if !matches!(size, 4 | 8) {
        return None;
    }
    let a = peel(fd, operation.get_in(0)?, id, size)?;
    let b = peel(fd, operation.get_in(1)?, id, size)?;
    let pointer = |operand: &Operand, ty: &Datatype| {
        let node = fd.vbank().get(operand.value).unwrap();
        !node.is_implied() && !node.is_constant() && native_pointer(ty, size)
    };
    let at = value_type(a.value, a.reader);
    let bt = value_type(b.value, b.reader);
    let (base, offset, offset_type) = match (pointer(&a, &at), pointer(&b, &bt)) {
        (true, false) => (a, b, bt),
        (false, true) => (b, a, at),
        _ => return None,
    };
    let offset_node = fd.vbank().get(offset.value)?;
    if offset_node.is_implied()
        || offset_type.get_size() != size
        || crate::kuna_typequal::is_volatile(&offset_type)
        || !matches!(
            offset_type.get_metatype(),
            type_metatype::TYPE_INT | type_metatype::TYPE_UINT | type_metatype::TYPE_UNKNOWN
        )
    {
        return None;
    }
    let shift = 64 - size * 8;
    let signed = ((offset_node.get_offset() << shift) as i64) >> shift;
    Some(Addition {
        base,
        offset,
        operation: id,
        negative_constant: (offset_node.is_constant() && signed < 0).then_some(signed),
    })
}

impl PrintC {
    pub(crate) fn push_go_pointer_add(
        &mut self,
        fd: &Funcdata,
        arch: &Architecture,
        value: VarnodeId,
        reader: OpId,
        target: &Rc<Datatype>,
    ) -> bool {
        if self.out_lang != crate::kuna_lang::OutLang::Go
            || self.options.nocasts
            || !native_pointer(target, arch.types().get_size_of_pointer())
        {
            return false;
        }
        let Some(plan) = addition(fd, value, reader, |value, op| {
            self.go_value_type(fd, arch, value, op)
        }) else {
            return false;
        };
        if fd.vbank().get(value).unwrap().get_size() != target.get_size() {
            return false;
        }
        self.push_go_pointer_addition(fd, arch, plan, reader, target);
        true
    }

    pub(crate) fn push_go_pointer_operation(
        &mut self,
        fd: &Funcdata,
        arch: &Architecture,
        op: OpId,
        target: &Rc<Datatype>,
    ) -> bool {
        if self.out_lang != crate::kuna_lang::OutLang::Go
            || self.options.nocasts
            || !native_pointer(target, arch.types().get_size_of_pointer())
        {
            return false;
        }
        let Some(plan) = operation(fd, op, |value, reader| {
            self.go_value_type(fd, arch, value, reader)
        }) else {
            return false;
        };
        if fd.vbank().get(plan.base.value).unwrap().get_size() != target.get_size() {
            return false;
        }
        self.push_go_pointer_addition(fd, arch, plan, op, target);
        true
    }

    fn push_go_pointer_addition(
        &mut self,
        fd: &Funcdata,
        arch: &Architecture,
        plan: Addition,
        reader: OpId,
        target: &Rc<Datatype>,
    ) {
        self.push_go_pointer_conversion(target, true, reader);
        self.push_go_call("unsafe.Add", plan.operation);
        self.push_op(&tokens::COMMA, Some(op_key(plan.operation)));
        self.push_go_call("unsafe.Pointer", plan.base.reader);
        self.push_vn_ir(fd, arch, plan.base.value, plan.base.reader);
        if let Some(offset) = plan.negative_constant {
            self.push_op(&tokens::UNARY_MINUS, Some(op_key(plan.offset.reader)));
            self.push_atom(&Atom::with_op_vn(
                format!("0x{:x}", offset.unsigned_abs()),
                TagType::VarToken,
                SyntaxHighlight::const_color,
                op_key(plan.offset.reader),
                vn_key(plan.offset.value),
            ));
        } else {
            self.push_vn_ir(fd, arch, plan.offset.value, plan.offset.reader);
        }
    }
}

#[cfg(test)]
mod tests;
