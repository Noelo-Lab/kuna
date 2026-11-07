//! Spell an exact-width zero store as a value of its integer byte-array type.

use crate::context::VarnodeId;
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;
use crate::printc::PrintC;
use crate::printlanguage::{Atom, SyntaxHighlight, TagType};

impl PrintC {
    pub(crate) fn push_rust_array_zero(
        &mut self,
        fd: &Funcdata,
        value: VarnodeId,
        target: &Datatype,
    ) -> bool {
        if target.get_metatype() != type_metatype::TYPE_ARRAY || target.get_size() <= 0 {
            return false;
        }
        let Some(node) = fd.vbank().get(value) else {
            return false;
        };
        if !node.is_constant() || node.get_offset() != 0 || node.get_size() != target.get_size() {
            return false;
        }
        let Some(element) = target.get_array_base() else {
            return false;
        };
        if element.get_size() != 1
            || element.get_align_size() != 1
            || element.is_enum_type()
            || !matches!(
                element.get_metatype(),
                type_metatype::TYPE_INT | type_metatype::TYPE_UINT | type_metatype::TYPE_UNKNOWN
            )
            || target.num_elements() != Some(target.get_size())
        {
            return false;
        }
        self.push_atom(&Atom::syntax(
            format!("[0; {}]", target.get_size()),
            TagType::Syntax,
            SyntaxHighlight::const_color,
        ));
        true
    }
}
