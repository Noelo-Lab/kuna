//! Render raw-pointer indexing with the instruction's wrapping byte offset.

use crate::architecture::Architecture;
use crate::context::OpId;
use crate::dtype::type_metatype;
use crate::funcdata::Funcdata;
use crate::printc::{op_key, tokens, PrintC};
use crate::printlanguage::{modifiers, Atom, SyntaxHighlight, TagType};
use kuna_num::opcodes::OpCode;

impl PrintC {
    pub(crate) fn op_rust_ptradd(&mut self, fd: &Funcdata, arch: &Architecture, op: OpId) -> bool {
        let Some(operation) = fd.obank().get(op) else { return false; };
        let (Some(base), Some(index), Some(scale), Some(output)) = (
            operation.get_in(0), operation.get_in(1), operation.get_in(2), operation.get_out(),
        ) else { return false; };
        let source = self.rust_integer_value_type(fd, arch, base, op);
        let target = fd.vn_type_def_facing(output);
        let index_type = self.rust_integer_value_type(fd, arch, index, op);
        if source.get_metatype() != type_metatype::TYPE_PTR
            || target.get_metatype() != type_metatype::TYPE_PTR
            || !matches!(source.get_size(), 4 | 8)
            || target.get_size() != source.get_size()
            || source.get_word_size() != Some(1)
            || target.get_word_size() != Some(1)
            || index_type.get_size() != source.get_size()
            || index_type.is_enum_type()
            || !matches!(index_type.get_metatype(), type_metatype::TYPE_INT
                | type_metatype::TYPE_UINT | type_metatype::TYPE_UNKNOWN)
        { return false; }
        let Some(base_node) = fd.vbank().get(base) else { return false; };
        if base_node.get_size() != source.get_size()
            || !fd.vbank().get(index).is_some_and(|node| node.get_size() == source.get_size())
            || !fd.vbank().get(output).is_some_and(|node| node.get_size() == target.get_size())
            || base_node.is_constant()
            || base_node.is_implied() && base_node.get_def()
                .and_then(|def| fd.obank().get(def))
                .is_some_and(|def| matches!(def.code(), OpCode::CPUI_PTRSUB | OpCode::CPUI_COPY))
        { return false; }
        let Some(scale_node) = fd.vbank().get(scale).filter(|node| node.is_constant()) else {
            return false;
        };
        let scale = scale_node.get_offset();
        let scale = if source.get_size() == 4 { scale as i32 as i64 } else { scale as i64 };
        let value = self.context.is_set(modifiers::PRINT_LOAD_VALUE | modifiers::PRINT_STORE_VALUE);
        let mods = self.context.mods() & !(modifiers::PRINT_LOAD_VALUE | modifiers::PRINT_STORE_VALUE);
        let convert = source.get_ptr_to().unwrap().get_id() != target.get_ptr_to().unwrap().get_id();
        if value { self.push_op(&tokens::DEREFERENCE, Some(op_key(op))); }
        if convert { self.push_cast_open(&target, op); }
        self.push_op(&tokens::FUNCTION_CALL, Some(op_key(op)));
        self.push_op(&tokens::OBJECT_MEMBER, Some(op_key(op)));
        self.push_vn_ir_m(fd, arch, base, op, mods);
        self.push_atom(&Atom::syntax("wrapping_byte_offset", TagType::Syntax, SyntaxHighlight::funcname_color));
        self.push_op(&tokens::FUNCTION_CALL, Some(op_key(op)));
        self.push_op(&tokens::OBJECT_MEMBER, Some(op_key(op)));
        self.push_op(self.lang().tok_typecast, Some(op_key(op)));
        let saved = self.context.mods();
        self.context.set_mods(mods);
        if fd.vbank().get(index).is_some_and(|node| node.is_constant()) {
            self.push_cast_open(&index_type, op);
            self.push_vn_ir(fd, arch, index, op);
            self.push_cast_close(&index_type);
        } else if !self.push_rust_integer_conversion(fd, arch, index, op, &index_type, true) {
            self.push_vn_ir(fd, arch, index, op);
        }
        self.context.set_mods(saved);
        self.push_atom(&Atom::syntax("isize", TagType::TypeToken, SyntaxHighlight::type_color));
        self.push_atom(&Atom::syntax("wrapping_mul", TagType::Syntax, SyntaxHighlight::funcname_color));
        self.push_atom(&Atom::syntax(scale.to_string(), TagType::Syntax, SyntaxHighlight::const_color));
        if convert { self.push_cast_close(&target); }
        true
    }
}
