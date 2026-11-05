//! Convert a primitive byte to Boolean without changing its storage width.

use crate::architecture::Architecture;
use crate::context::{OpId, VarnodeId};
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;
use crate::printc::{op_key, tokens, PrintC};
use crate::printlanguage::{Atom, SyntaxHighlight, TagType};
use kuna_num::opcodes::OpCode;
use std::rc::Rc;

impl PrintC {
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
        self.push_op(&tokens::NOT_EQUAL, Some(op_key(op)));
        self.push_cast_open(byte, op);
        self.push_vn_ir(fd, arch, value, op);
        self.push_cast_close(byte);
        self.push_atom(&Atom::with_op(
            "0", TagType::Syntax, SyntaxHighlight::const_color, op_key(op),
        ));
        true
    }

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
            if let Some(def) = node.get_def() {
                if fd.obank().get(def).is_some_and(|operation|
                    matches!(operation.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND))
                {
                    let Some(index) = fd.get_call_specs_index(def) else { return false; };
                    let Some(result) = fd.get_call_specs(index).proto().get_output_type() else { return false; };
                    if result.get_size() != 1 || result.is_enum_type()
                        || !matches!(result.get_metatype(), type_metatype::TYPE_INT
                            | type_metatype::TYPE_UINT | type_metatype::TYPE_UNKNOWN)
                    {
                        return false;
                    }
                }
            }
        }
        let context = if node.is_implied() {
            let Some(context) = crate::kuna_rustintegers::needs_integer_context(fd, value, op, 8)
                else { return false; };
            context
        } else { false };
        self.push_op(&tokens::NOT_EQUAL, Some(op_key(op)));
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
