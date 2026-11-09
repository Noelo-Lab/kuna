//! Port of `org/jetbrains/java/decompiler/struct/attr/StructRecordAttribute.java`.

use crate::byte_cursor::ByteCursor;
use crate::bytecode_version::BytecodeVersion;
use crate::constant_pool::ConstantPool;
use crate::struct_general_attribute::{
    attribute_view, StructGeneralAttribute, StructGeneralAttributeKind,
};
use crate::struct_record_component::StructRecordComponent;

#[derive(Default)]
pub struct StructRecordAttributeData {
    pub components: Vec<StructRecordComponent>,
}

attribute_view!(
    StructRecordAttribute,
    Record,
    StructRecordAttributeData,
    is_record,
    as_record,
    try_as_record
);

impl StructRecordAttribute {
    pub fn new() -> StructRecordAttribute {
        StructRecordAttribute::alloc_view(StructRecordAttributeData::default())
    }

    pub fn init_content(self, data: &mut ByteCursor, pool: ConstantPool, version: BytecodeVersion) {
        let component_count = data.read_unsigned_short();
        let mut components = Vec::with_capacity(component_count as usize);
        for _ in 0..component_count {
            components.push(StructRecordComponent::create(data, pool, version));
        }
        self.d_mut(|d| d.components = components);
    }

    pub fn get_components(self) -> Vec<StructRecordComponent> {
        self.d(|d| d.components.clone())
    }

    pub fn set_components(self, components: Vec<StructRecordComponent>) {
        self.d_mut(|d| d.components = components)
    }
}
