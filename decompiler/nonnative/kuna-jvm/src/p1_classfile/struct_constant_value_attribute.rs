//! Port of `org/jetbrains/java/decompiler/struct/attr/StructConstantValueAttribute.java`.

use crate::byte_cursor::ByteCursor;
use crate::bytecode_version::BytecodeVersion;
use crate::constant_pool::ConstantPool;
use crate::struct_general_attribute::{
    attribute_view, StructGeneralAttribute, StructGeneralAttributeKind,
};

#[derive(Default)]
pub struct StructConstantValueAttributeData {
    pub index: i32,
}

attribute_view!(
    StructConstantValueAttribute,
    ConstantValue,
    StructConstantValueAttributeData,
    is_constant_value,
    as_constant_value,
    try_as_constant_value
);

impl StructConstantValueAttribute {
    pub fn new() -> StructConstantValueAttribute {
        StructConstantValueAttribute::alloc_view(StructConstantValueAttributeData::default())
    }

    pub fn init_content(
        self,
        data: &mut ByteCursor,
        _pool: ConstantPool,
        _version: BytecodeVersion,
    ) {
        let index = data.read_unsigned_short();
        self.d_mut(|d| d.index = index);
    }

    pub fn get_index(self) -> i32 {
        self.d(|d| d.index)
    }

    pub fn set_index(self, index: i32) {
        self.d_mut(|d| d.index = index)
    }
}
