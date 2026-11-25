//! Convert a primitive byte to Boolean without changing its storage width.

use crate::architecture::Architecture;
use crate::context::{OpId, VarnodeId};
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;
use crate::printc::{op_key, tokens, PrintC};
use crate::printlanguage::{Atom, OpToken, SyntaxHighlight, TagType};
use kuna_num::opcodes::OpCode;
use std::rc::Rc;

impl PrintC {
    /// The `!=` of a byte test `x != 0`: C's token, which keeps the parentheses
    /// around a bitwise operand, except under another comparison, where Rust's
    /// own token parenthesises it (Rust comparisons do not chain).
    pub(crate) fn byte_test_token(&self) -> &'static OpToken {
        if self.top_is_comparison() {
            self.lang_token(&tokens::NOT_EQUAL)
        } else {
            &tokens::NOT_EQUAL
        }
    }

    /// Whether constant `vn` read by `op` prints as `false`/`true`: its type is
    /// Boolean and so is what consumes it, the function's return value, the
    /// declared variable a COPY assigns (through implied copies) or the other
    /// side of an equality test.
    pub(crate) fn rust_boolean_constant(&self, fd: &Funcdata, arch: &Architecture, vn: VarnodeId, op: OpId) -> bool {
        let boolean = |ty: &Datatype| ty.get_metatype() == type_metatype::TYPE_BOOL && ty.get_size() == 1;
        boolean(&fd.vn_type_read_facing(vn, op)) && self.rust_boolean_consumer(fd, arch, vn, op, 4)
    }

    fn rust_boolean_consumer(&self, fd: &Funcdata, arch: &Architecture, vn: VarnodeId, op: OpId, depth: u8) -> bool {
        let boolean = |ty: &Datatype| ty.get_metatype() == type_metatype::TYPE_BOOL && ty.get_size() == 1;
        let Some(operation) = fd.obank().get(op) else { return false; };
        match operation.code() {
            OpCode::CPUI_RETURN => fd.get_func_proto().get_output_type().is_some_and(|ty| boolean(ty)),
            OpCode::CPUI_COPY => {
                let Some(out) = operation.get_out() else { return false; };
                if fd.vbank().get(out).is_some_and(|node| node.is_implied()) {
                    depth > 0 && fd.lone_descend(out)
                        .is_some_and(|reader| self.rust_boolean_consumer(fd, arch, out, reader, depth - 1))
                } else {
                    boolean(&self.rust_integer_output_type(fd, arch, out))
                }
            }
            OpCode::CPUI_INT_EQUAL | OpCode::CPUI_INT_NOTEQUAL => (0..2)
                .filter_map(|slot| operation.get_in(slot))
                .filter(|&other| other != vn)
                .any(|other| boolean(&self.rust_integer_value_type(fd, arch, other, op))),
            _ => false,
        }
    }

    fn rust_boolean_expression(
        &self,
        fd: &Funcdata,
        arch: &Architecture,
        value: VarnodeId,
        op: OpId,
        depth: u8,
    ) -> bool {
        let Some(node) = fd.vbank().get(value) else { return false; };
        if depth == 0 || node.get_size() != 1 || node.is_constant() { return false; }
        if !node.is_implied() {
            let ty = self.rust_integer_value_type(fd, arch, value, op);
            return ty.get_metatype() == type_metatype::TYPE_BOOL && ty.get_size() == 1
                && !ty.is_enum_type();
        }
        let Some(def) = node.get_def() else { return false; };
        let Some(operation) = fd.obank().get(def) else { return false; };
        match operation.code() {
            OpCode::CPUI_INT_AND | OpCode::CPUI_INT_OR | OpCode::CPUI_INT_XOR => {
                operation.num_input() == 2 && (0..2).all(|slot| operation.get_in(slot)
                    .is_some_and(|input| self.rust_boolean_expression(fd, arch, input, def, depth - 1)))
            }
            OpCode::CPUI_BOOL_NEGATE => {
                operation.num_input() == 1 && operation.get_in(0)
                    .is_some_and(|input| self.rust_boolean_expression(fd, arch, input, def, depth - 1))
            }
            OpCode::CPUI_BOOL_AND | OpCode::CPUI_BOOL_OR | OpCode::CPUI_BOOL_XOR
            | OpCode::CPUI_INT_EQUAL | OpCode::CPUI_INT_NOTEQUAL
            | OpCode::CPUI_INT_LESS | OpCode::CPUI_INT_LESSEQUAL
            | OpCode::CPUI_INT_SLESS | OpCode::CPUI_INT_SLESSEQUAL
            | OpCode::CPUI_FLOAT_EQUAL | OpCode::CPUI_FLOAT_NOTEQUAL
            | OpCode::CPUI_FLOAT_LESS | OpCode::CPUI_FLOAT_LESSEQUAL => operation.num_input() == 2,
            OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => {
                let Some(index) = fd.get_call_specs_index(def) else { return false; };
                let proto = fd.get_call_specs(index).proto();
                let result = self.call_result_plan(fd, arch, def).map(|plan| plan.target)
                    .or_else(|| proto.has_store().then(|| proto.get_output_type().cloned()).flatten());
                result.is_some_and(|ty| ty.get_metatype() == type_metatype::TYPE_BOOL
                    && ty.get_size() == 1 && !ty.is_enum_type())
            }
            _ => false,
        }
    }

    pub(crate) fn push_rust_truncated_boolean_conversion(
        &mut self,
        fd: &Funcdata,
        arch: &Architecture,
        op: OpId,
        target: &Datatype,
        byte: &Rc<Datatype>,
    ) -> bool {
        if target.get_metatype() != type_metatype::TYPE_BOOL || target.get_size() != 1
            || byte.get_metatype() != type_metatype::TYPE_UINT || byte.get_size() != 1
        {
            return false;
        }
        let Some(value) = fd.obank().get(op).and_then(|operation| operation.get_in(0))
            else { return false; };
        self.push_op(self.byte_test_token(), Some(op_key(op)));
        self.push_cast_open(byte, op);
        self.push_vn_ir(fd, arch, value, op);
        self.push_cast_close(byte);
        self.push_atom(&Atom::with_op(
            "0", TagType::Syntax, SyntaxHighlight::const_color, op_key(op),
        ));
        true
    }

    /// A byte-valued expression converted to `bool` prints as `expr != 0`
    /// (Rust has no integer-to-`bool` cast); a call whose result is not stated
    /// as one byte tests its low byte, `call as u8 != 0`. A value that already
    /// is Boolean is left to the cast.
    pub(crate) fn push_rust_boolean_conversion(
        &mut self,
        fd: &Funcdata,
        arch: &Architecture,
        value: VarnodeId,
        op: OpId,
        target: &Datatype,
    ) -> bool {
        if target.get_metatype() != type_metatype::TYPE_BOOL || target.get_size() != 1 {
            return false;
        }
        let Some(node) = fd.vbank().get(value) else { return false; };
        let source = self.rust_integer_value_type(fd, arch, value, op);
        if node.get_size() != 1 || source.get_size() != 1 || source.is_enum_type()
            || !matches!(source.get_metatype(), type_metatype::TYPE_INT
                | type_metatype::TYPE_UINT | type_metatype::TYPE_UNKNOWN)
        {
            return false;
        }
        if node.is_implied() {
            if self.rust_boolean_expression(fd, arch, value, op, 8) { return false; }
            if let Some(def) = node.get_def() {
                if fd.obank().get(def).is_some_and(|operation|
                    matches!(operation.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND))
                {
                    let Some(index) = fd.get_call_specs_index(def) else { return false; };
                    let call = fd.get_call_specs(index);
                    let proto = call.proto();
                    let result = self.call_result_plan(fd, arch, def).map(|plan| plan.target)
                        .or_else(|| proto.has_store().then(|| proto.get_output_type().cloned()).flatten())
                        .filter(|result| result.get_metatype() != type_metatype::TYPE_VOID)
                        .or_else(|| fd.kuna_protoorder_types(call.get_entry_address())
                            .and_then(|recovered| recovered.output.as_ref())
                            .map(|(_, _, recovered)| Rc::clone(recovered)));
                    if result.as_ref().is_some_and(|result| {
                        result.get_metatype() == type_metatype::TYPE_BOOL && result.get_size() == 1
                    }) {
                        return false;
                    }
                    if result.as_ref().is_some_and(|result| result.is_enum_type()
                        || !matches!(result.get_metatype(), type_metatype::TYPE_INT
                            | type_metatype::TYPE_UINT | type_metatype::TYPE_UNKNOWN))
                    {
                        return false;
                    }
                    if result.is_none_or(|result| result.get_size() != 1) {
                        let Ok(byte) = arch.types().get_base(1, type_metatype::TYPE_UINT) else { return false; };
                        self.push_op(self.byte_test_token(), Some(op_key(op)));
                        self.push_cast_open(&byte, op);
                        self.push_vn_ir(fd, arch, value, op);
                        self.push_cast_close(&byte);
                        self.push_atom(&Atom::with_op(
                            "0", TagType::Syntax, SyntaxHighlight::const_color, op_key(op),
                        ));
                        return true;
                    }
                }
            }
        }
        let context = node.is_implied()
            && crate::kuna_rustintegers::needs_integer_context(fd, value, op, 8).unwrap_or(true);
        self.push_op(self.byte_test_token(), Some(op_key(op)));
        if context {
            let signed = source.get_metatype() == type_metatype::TYPE_INT
                && !(source.is_char_print() && source.is_core_type());
            self.push_op(&tokens::FUNCTION_CALL, Some(op_key(op)));
            self.push_atom(&Atom::with_op(
                format!("::core::convert::identity::<{}8>", if signed { "i" } else { "u" }),
                TagType::OpToken, SyntaxHighlight::no_color, op_key(op),
            ));
        }
        self.push_vn_ir(fd, arch, value, op);
        self.push_atom(&Atom::with_op(
            "0", TagType::Syntax, SyntaxHighlight::const_color, op_key(op),
        ));
        true
    }
}
