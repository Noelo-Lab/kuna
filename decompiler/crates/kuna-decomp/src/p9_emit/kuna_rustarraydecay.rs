//! Convert proven array places at Rust pointer consumers without loading them.

use std::rc::Rc;

use super::kuna_rustarrayaddr::{array_place, RAW_ADDRESS};
use crate::architecture::Architecture;
use crate::context::{OpId, VarnodeId};
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;
use crate::printc::{op_key, PrintC};
use crate::printlanguage::modifiers;
use kuna_num::opcodes::OpCode;

fn array_source(
    fd: &Funcdata,
    mut value: VarnodeId,
    mut reader: OpId,
    size: i32,
) -> Option<(VarnodeId, OpId)> {
    for _ in 0..8 {
        let node = fd.vbank().get(value)?;
        let source = fd.vn_type_read_facing(value, reader);
        if node.get_size() != size
            || source.get_size() != size
            || source.get_metatype() != type_metatype::TYPE_PTR
            || source.get_word_size() != Some(1)
            || source.is_formal_pointer_rel()
        {
            return None;
        }
        if array_place(fd, value) {
            return Some((value, reader));
        }
        if !node.is_implied() {
            return None;
        }
        let def = node.get_def()?;
        let operation = fd.obank().get(def)?;
        if !matches!(operation.code(), OpCode::CPUI_CAST | OpCode::CPUI_COPY) {
            return None;
        }
        value = operation.get_in(0)?;
        reader = def;
    }
    None
}

impl PrintC {
    pub(crate) fn push_rust_array_pointer(
        &mut self,
        fd: &Funcdata,
        arch: &Architecture,
        value: VarnodeId,
        op: OpId,
        target: &Rc<Datatype>,
    ) -> bool {
        if target.get_metatype() != type_metatype::TYPE_PTR
            || !matches!(target.get_size(), 4 | 8)
            || target.get_word_size() != Some(1)
            || target.is_formal_pointer_rel()
            || self
                .context
                .is_set(modifiers::PRINT_LOAD_VALUE | modifiers::PRINT_STORE_VALUE)
        {
            return false;
        }
        let source = self.rust_integer_value_type(fd, arch, value, op);
        if source.get_metatype() != type_metatype::TYPE_PTR
            || source.get_size() != target.get_size()
            || source.get_word_size() != Some(1)
            || source.is_formal_pointer_rel()
        {
            return false;
        }
        let Some((value, reader)) = array_source(fd, value, op, target.get_size()) else {
            return false;
        };
        self.push_cast_open(target, op);
        self.push_op(&RAW_ADDRESS, Some(op_key(op)));
        self.push_vn_ir(fd, arch, value, reader);
        self.push_cast_close(target);
        true
    }

    pub(crate) fn push_rust_value_as(
        &mut self,
        fd: &Funcdata,
        arch: &Architecture,
        value: VarnodeId,
        op: OpId,
        target: &Rc<Datatype>,
    ) -> bool {
        self.push_rust_array_pointer(fd, arch, value, op, target)
            || self.push_rust_integer_as(fd, arch, value, op, target)
    }
}
