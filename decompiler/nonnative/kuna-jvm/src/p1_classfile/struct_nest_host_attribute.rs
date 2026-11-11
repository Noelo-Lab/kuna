//! Port of `org/jetbrains/java/decompiler/struct/attr/StructNestHostAttribute.java`.

use crate::byte_cursor::ByteCursor;
use crate::bytecode_version::BytecodeVersion;
use crate::constant_pool::ConstantPool;
use crate::struct_general_attribute::{
    attribute_view, StructGeneralAttribute, StructGeneralAttributeKind,
};

#[derive(Default)]
pub struct StructNestHostAttributeData {
    pub const_pool_index: i32,
}

attribute_view!(
    StructNestHostAttribute,
    NestHost,
    StructNestHostAttributeData,
    is_nest_host,
    as_nest_host,
    try_as_nest_host
);

impl StructNestHostAttribute {
    pub fn new() -> StructNestHostAttribute {
        StructNestHostAttribute::alloc_view(StructNestHostAttributeData::default())
    }

    pub fn init_content(
        self,
        data: &mut ByteCursor,
        _pool: ConstantPool,
        _version: BytecodeVersion,
    ) {
        let index = data.read_unsigned_short();
        self.d_mut(|d| d.const_pool_index = index);
    }

    pub fn get_host_class(self, pool: ConstantPool) -> String {
        let index = self.d(|d| d.const_pool_index);
        pool.get_primitive_constant(index)
            .expect("constant")
            .get_string()
            .to_string()
    }

    pub fn set_const_pool_index(self, const_pool_index: i32) {
        self.d_mut(|d| d.const_pool_index = const_pool_index)
    }
}
