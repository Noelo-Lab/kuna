//! Convert proven array places at Rust pointer consumers without loading them.

use std::rc::Rc;

use crate::architecture::Architecture;
use crate::context::{OpId, VarnodeId};
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;
use crate::printc::{op_key, PrintC};
use crate::printlanguage::modifiers;
use super::kuna_rustarrayaddr::{array_place, RAW_ADDRESS};

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
            || target.get_word_size() != Some(1) || target.is_formal_pointer_rel()
            || self.context.is_set(modifiers::PRINT_LOAD_VALUE | modifiers::PRINT_STORE_VALUE)
        { return false; }
        let source = self.rust_integer_value_type(fd, arch, value, op);
        if source.get_metatype() != type_metatype::TYPE_PTR
            || source.get_size() != target.get_size()
            || source.get_word_size() != Some(1) || source.is_formal_pointer_rel()
            || !fd.vbank().get(value).is_some_and(|node| node.get_size() == source.get_size())
            || !array_place(fd, value)
        { return false; }
        self.push_cast_open(target, op);
        self.push_op(&RAW_ADDRESS, Some(op_key(op)));
        self.push_vn_ir(fd, arch, value, op);
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
