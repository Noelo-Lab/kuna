//! Port of `org/jetbrains/java/decompiler/struct/attr/StructExceptionsAttribute.java`.

use crate::byte_cursor::ByteCursor;
use crate::bytecode_version::BytecodeVersion;
use crate::constant_pool::ConstantPool;
use crate::struct_general_attribute::{
    attribute_view, StructGeneralAttribute, StructGeneralAttributeKind,
};

#[derive(Default)]
pub struct StructExceptionsAttributeData {
    pub throws_exceptions: Vec<i32>,
}

attribute_view!(
    StructExceptionsAttribute,
    Exceptions,
    StructExceptionsAttributeData,
    is_exceptions,
    as_exceptions,
    try_as_exceptions
);

impl StructExceptionsAttribute {
    pub fn new() -> StructExceptionsAttribute {
        StructExceptionsAttribute::alloc_view(StructExceptionsAttributeData::default())
    }

    pub fn init_content(
        self,
        data: &mut ByteCursor,
        _pool: ConstantPool,
        _version: BytecodeVersion,
    ) {
        let len = data.read_unsigned_short();
        let mut throws_exceptions = Vec::with_capacity(len as usize);
        for _ in 0..len {
            throws_exceptions.push(data.read_unsigned_short());
        }
        self.d_mut(|d| d.throws_exceptions = throws_exceptions);
    }

    pub fn get_exc_classname(self, index: i32, pool: ConstantPool) -> String {
        let cp_index = self.d(|d| d.throws_exceptions[index as usize]);
        pool.get_primitive_constant(cp_index)
            .expect("constant")
            .get_string()
    }

    pub fn get_throws_exceptions(self) -> Vec<i32> {
        self.d(|d| d.throws_exceptions.clone())
    }

    pub fn set_throws_exceptions(self, throws_exceptions: Vec<i32>) {
        self.d_mut(|d| d.throws_exceptions = throws_exceptions)
    }
}
