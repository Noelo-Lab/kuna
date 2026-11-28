//! Port of `org/jetbrains/java/decompiler/struct/attr/StructLocalVariableTypeTableAttribute.java`.

use crate::byte_cursor::ByteCursor;
use crate::bytecode_version::BytecodeVersion;
use crate::constant_pool::ConstantPool;
use crate::struct_general_attribute::{
    attribute_view, StructGeneralAttribute, StructGeneralAttributeKind,
};
use crate::struct_local_variable_table_attribute::StructLocalVariableTableAttribute;

pub struct StructLocalVariableTypeTableAttributeData {
    /// Holds signatures in its descriptor slots.
    pub backing_attribute: StructLocalVariableTableAttribute,
}

attribute_view!(
    StructLocalVariableTypeTableAttribute,
    LocalVariableTypeTable,
    StructLocalVariableTypeTableAttributeData,
    is_local_variable_type_table,
    as_local_variable_type_table,
    try_as_local_variable_type_table
);

impl StructLocalVariableTypeTableAttribute {
    pub fn new() -> StructLocalVariableTypeTableAttribute {
        StructLocalVariableTypeTableAttribute::alloc_view(
            StructLocalVariableTypeTableAttributeData {
                backing_attribute: StructLocalVariableTableAttribute::new(),
            },
        )
    }

    /// The public field `backingAttribute`.
    pub fn backing_attribute(self) -> StructLocalVariableTableAttribute {
        self.d(|d| d.backing_attribute)
    }

    pub fn init_content(self, data: &mut ByteCursor, pool: ConstantPool, version: BytecodeVersion) {
        StructLocalVariableTableAttribute::init_content(
            self.backing_attribute(),
            data,
            pool,
            version,
        );
    }

    pub fn add(self, attr: StructLocalVariableTypeTableAttribute) {
        self.backing_attribute().add(attr.backing_attribute());
    }

    pub fn get_signature(self, index: i32, visible_offset: i32) -> Option<String> {
        self.backing_attribute()
            .get_descriptor(index, visible_offset)
    }
}
