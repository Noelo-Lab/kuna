//! Port of `org/jetbrains/java/decompiler/struct/attr/StructSourceFileAttribute.java`.

use crate::byte_cursor::ByteCursor;
use crate::bytecode_version::BytecodeVersion;
use crate::constant_pool::ConstantPool;
use crate::struct_general_attribute::{
    attribute_view, StructGeneralAttribute, StructGeneralAttributeKind,
};

#[derive(Default)]
pub struct StructSourceFileAttributeData {
    pub const_pool_index: i32,
}

attribute_view!(
    StructSourceFileAttribute,
    SourceFile,
    StructSourceFileAttributeData,
    is_source_file,
    as_source_file,
    try_as_source_file
);

impl StructSourceFileAttribute {
    pub fn new() -> StructSourceFileAttribute {
        StructSourceFileAttribute::alloc_view(StructSourceFileAttributeData::default())
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

    pub fn get_source_file(self, pool: ConstantPool) -> String {
        let index = self.d(|d| d.const_pool_index);
        pool.get_primitive_constant(index)
            .expect("constant")
            .get_string()
    }

    pub fn get_const_pool_index(self) -> i32 {
        self.d(|d| d.const_pool_index)
    }

    pub fn set_const_pool_index(self, const_pool_index: i32) {
        self.d_mut(|d| d.const_pool_index = const_pool_index)
    }
}
