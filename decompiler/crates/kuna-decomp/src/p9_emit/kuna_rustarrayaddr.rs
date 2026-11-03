//! Form raw addresses from proven array places without indexing or borrowing.

use crate::architecture::Architecture;
use crate::context::{OpId, VarnodeId};
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;
use crate::kuna_rustintegers::needs_integer_context;
use crate::printc::{op_key, op_token, tokens, PrintC};
use crate::printlanguage::{modifiers, Atom, OpToken, SyntaxHighlight, TagType, TokenType};
use kuna_num::opcodes::OpCode;

static RAW_ADDRESS: OpToken = op_token("&raw mut ", "", 1, 62, false, TokenType::UnaryPrefix, 0, 0);

fn array_place(fd: &Funcdata, base: VarnodeId) -> bool {
    let Some(node) = fd.vbank().get(base).filter(|node| node.is_implied()) else { return false; };
    let Some(def) = node.get_def() else { return false; };
    let Some(operation) = fd.obank().get(def).filter(|op| op.code() == OpCode::CPUI_PTRSUB) else {
        return false;
    };
    let (Some(owner), Some(offset)) = (operation.get_in(0), operation.get_in(1)) else { return false; };
    let Some(offset) = fd.vbank().get(offset).filter(|node| node.is_constant()) else { return false; };
    let pointer = fd.vn_type_read_facing(owner, def);
    if pointer.get_metatype() != type_metatype::TYPE_PTR
        || pointer.get_word_size() != Some(1) || pointer.is_formal_pointer_rel()
    { return false; }
    let Some(inner) = pointer.get_ptr_to() else { return false; };
    let offset = offset.get_offset();
    if inner.get_metatype() == type_metatype::TYPE_ARRAY {
        return offset == 0 && inner.get_size() > 0;
    }
    if inner.get_metatype() != type_metatype::TYPE_STRUCT
        || inner.get_size() <= 0 || offset >= inner.get_size() as u64
    { return false; }
    inner.find_truncation(offset as i64, 0, def, 0).ok().flatten()
        .filter(|(_, remainder)| *remainder == 0)
        .and_then(|(field, _)| inner.get_field(field))
        .is_some_and(|field| field.field_type.get_metatype() == type_metatype::TYPE_ARRAY
            && field.field_type.get_size() > 0)
}

impl PrintC {
    pub(crate) fn push_rust_array_address_cast(
        &mut self,
        fd: &Funcdata,
        arch: &Architecture,
        value: VarnodeId,
        op: OpId,
        target: &std::rc::Rc<Datatype>,
    ) -> bool {
        let source = self.rust_integer_value_type(fd, arch, value, op);
        if self.context.is_set(modifiers::PRINT_LOAD_VALUE | modifiers::PRINT_STORE_VALUE)
            || source.get_metatype() != type_metatype::TYPE_PTR
            || !matches!(source.get_size(), 4 | 8) || target.get_size() != source.get_size()
            || source.get_word_size() != Some(1) || source.is_formal_pointer_rel()
            || target.is_enum_type()
            || !matches!(target.get_metatype(), type_metatype::TYPE_INT
                | type_metatype::TYPE_UINT | type_metatype::TYPE_UNKNOWN)
            || !fd.vbank().get(value).is_some_and(|node| node.get_size() == source.get_size())
            || !array_place(fd, value)
        { return false; }
        self.push_cast_open(target, op);
        self.push_op(&RAW_ADDRESS, Some(op_key(op)));
        self.push_vn_ir(fd, arch, value, op);
        self.push_cast_close(target);
        true
    }

    pub(crate) fn op_rust_array_address(&mut self, fd: &Funcdata, arch: &Architecture, op: OpId) -> bool {
        if self.context.is_set(modifiers::PRINT_LOAD_VALUE | modifiers::PRINT_STORE_VALUE)
            || !self.options.array_notation()
        { return false; }
        let Some(operation) = fd.obank().get(op) else { return false; };
        let (Some(base), Some(index), Some(scale), Some(output)) = (
            operation.get_in(0), operation.get_in(1), operation.get_in(2), operation.get_out(),
        ) else { return false; };
        let source = self.rust_integer_value_type(fd, arch, base, op);
        let target = fd.vn_type_def_facing(output);
        let index_type = self.rust_integer_value_type(fd, arch, index, op);
        if source.get_metatype() != type_metatype::TYPE_PTR
            || target.get_metatype() != type_metatype::TYPE_PTR
            || !matches!(source.get_size(), 4 | 8) || target.get_size() != source.get_size()
            || source.get_word_size() != Some(1) || target.get_word_size() != Some(1)
            || source.is_formal_pointer_rel() || target.is_formal_pointer_rel()
            || index_type.get_size() != source.get_size() || index_type.is_enum_type()
            || !matches!(index_type.get_metatype(), type_metatype::TYPE_INT
                | type_metatype::TYPE_UINT | type_metatype::TYPE_UNKNOWN)
            || !array_place(fd, base)
        { return false; }
        if ![base, index, output].into_iter().all(|value| fd.vbank().get(value)
            .is_some_and(|node| node.get_size() == source.get_size()))
            || needs_integer_context(fd, index, op, 8).is_none()
        { return false; }
        let Some(scale) = fd.vbank().get(scale).filter(|node| node.is_constant()) else { return false; };
        let scale = scale.get_offset();
        let scale = if source.get_size() == 4 { scale as i32 as i64 } else { scale as i64 };
        self.push_cast_open(&target, op);
        self.push_op(&tokens::FUNCTION_CALL, Some(op_key(op)));
        self.push_op(&tokens::OBJECT_MEMBER, Some(op_key(op)));
        self.push_op(&RAW_ADDRESS, Some(op_key(op)));
        self.push_vn_ir(fd, arch, base, op);
        self.push_atom(&Atom::syntax("wrapping_byte_offset", TagType::Syntax, SyntaxHighlight::funcname_color));
        self.push_op(&tokens::FUNCTION_CALL, Some(op_key(op)));
        self.push_op(&tokens::OBJECT_MEMBER, Some(op_key(op)));
        self.push_op(self.lang().tok_typecast, Some(op_key(op)));
        if fd.vbank().get(index).is_some_and(|node| node.is_constant()) {
            self.push_cast_open(&index_type, op);
            self.push_vn_ir(fd, arch, index, op);
            self.push_cast_close(&index_type);
        } else if !self.push_rust_integer_conversion(fd, arch, index, op, &index_type, true) {
            self.push_vn_ir(fd, arch, index, op);
        }
        self.push_atom(&Atom::syntax("isize", TagType::TypeToken, SyntaxHighlight::type_color));
        self.push_atom(&Atom::syntax("wrapping_mul", TagType::Syntax, SyntaxHighlight::funcname_color));
        self.push_atom(&Atom::syntax(scale.to_string(), TagType::Syntax, SyntaxHighlight::const_color));
        self.push_cast_close(&target);
        true
    }
}
