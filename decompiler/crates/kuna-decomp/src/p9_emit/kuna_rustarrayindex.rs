//! Preserve integer expression width before converting a Rust array index.

use crate::architecture::Architecture;
use crate::context::{OpId, VarnodeId};
use crate::dtype::type_metatype;
use crate::funcdata::Funcdata;
use crate::kuna_rustintegers::needs_integer_context;
use crate::printc::{op_key, PrintC};
use crate::printlanguage::{Atom, SyntaxHighlight, TagType};

impl PrintC {
    pub(crate) fn push_rust_array_index(
        &mut self,
        fd: &Funcdata,
        arch: &Architecture,
        index: VarnodeId,
        op: OpId,
        mods: u32,
    ) -> bool {
        let Some(node) = fd.vbank().get(index) else { return false; };
        let source = self.rust_integer_value_type(fd, arch, index, op);
        if node.is_constant()
            || !matches!(source.get_size(), 1 | 2 | 4 | 8)
            || source.get_size() != node.get_size()
            || source.is_enum_type()
            || !matches!(source.get_metatype(), type_metatype::TYPE_INT
                | type_metatype::TYPE_UINT | type_metatype::TYPE_UNKNOWN)
            || node.is_implied() && needs_integer_context(fd, index, op, 8).is_none()
        { return false; }
        self.push_op(self.lang().tok_typecast, Some(op_key(op)));
        let saved = self.context.mods();
        self.context.set_mods(mods);
        if !self.push_rust_integer_conversion(fd, arch, index, op, &source, true) {
            self.push_vn_ir(fd, arch, index, op);
        }
        self.context.set_mods(saved);
        self.push_atom(&Atom::syntax("usize", TagType::TypeToken, SyntaxHighlight::type_color));
        true
    }
}
