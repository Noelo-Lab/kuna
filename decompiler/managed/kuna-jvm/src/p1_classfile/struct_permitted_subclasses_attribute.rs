//! Port of `org/jetbrains/java/decompiler/struct/attr/StructPermittedSubclassesAttribute.java`.

use crate::byte_cursor::ByteCursor;
use crate::bytecode_version::BytecodeVersion;
use crate::constant_pool::ConstantPool;
use crate::struct_general_attribute::{
    attribute_view, StructGeneralAttribute, StructGeneralAttributeKind,
};

#[derive(Default)]
pub struct StructPermittedSubclassesAttributeData {
    pub classes: Vec<String>,
}

attribute_view!(
    StructPermittedSubclassesAttribute,
    PermittedSubclasses,
    StructPermittedSubclassesAttributeData,
    is_permitted_subclasses,
    as_permitted_subclasses,
    try_as_permitted_subclasses
);

impl StructPermittedSubclassesAttribute {
    pub fn new() -> StructPermittedSubclassesAttribute {
        StructPermittedSubclassesAttribute::alloc_view(
            StructPermittedSubclassesAttributeData::default(),
        )
    }

    pub fn get_classes(self) -> Vec<String> {
        self.d(|d| d.classes.clone())
    }

    pub fn init_content(
        self,
        data: &mut ByteCursor,
        pool: ConstantPool,
        _version: BytecodeVersion,
    ) {
        let count = data.read_unsigned_short();
        for _ in 0..count {
            let class = pool
                .get_primitive_constant(data.read_unsigned_short())
                .expect("constant")
                .get_string()
                .to_string();
            self.d_mut(|d| d.classes.push(class));
        }
    }
}
