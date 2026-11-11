//! Port of `org/jetbrains/java/decompiler/struct/attr/StructEnclosingMethodAttribute.java`.

use crate::byte_cursor::ByteCursor;
use crate::bytecode_version::BytecodeVersion;
use crate::constant_pool::ConstantPool;
use crate::struct_general_attribute::{
    attribute_view, StructGeneralAttribute, StructGeneralAttributeKind,
};

#[derive(Default)]
pub struct StructEnclosingMethodAttributeData {
    pub class_name: Option<String>,
    pub method_name: Option<String>,
    pub method_descriptor: Option<String>,
}

attribute_view!(
    StructEnclosingMethodAttribute,
    EnclosingMethod,
    StructEnclosingMethodAttributeData,
    is_enclosing_method,
    as_enclosing_method,
    try_as_enclosing_method
);

impl StructEnclosingMethodAttribute {
    pub fn new() -> StructEnclosingMethodAttribute {
        StructEnclosingMethodAttribute::alloc_view(StructEnclosingMethodAttributeData::default())
    }

    pub fn init_content(
        self,
        data: &mut ByteCursor,
        pool: ConstantPool,
        _version: BytecodeVersion,
    ) {
        let class_index = data.read_unsigned_short();
        let method_index = data.read_unsigned_short();

        let class_name = pool
            .get_primitive_constant(class_index)
            .expect("constant")
            .get_string()
            .to_string();
        self.d_mut(|d| d.class_name = Some(class_name));
        if method_index != 0 {
            let lk = pool.get_link_constant(method_index).expect("constant");
            let (name, descriptor) = (lk.elementname.clone(), lk.descriptor.clone());
            self.d_mut(|d| {
                d.method_name = Some(name);
                d.method_descriptor = Some(descriptor);
            });
        }
    }

    pub fn get_class_name(self) -> Option<String> {
        self.d(|d| d.class_name.clone())
    }

    pub fn get_method_descriptor(self) -> Option<String> {
        self.d(|d| d.method_descriptor.clone())
    }

    pub fn get_method_name(self) -> Option<String> {
        self.d(|d| d.method_name.clone())
    }

    pub fn set_class_name(self, class_name: Option<String>) {
        self.d_mut(|d| d.class_name = class_name)
    }

    pub fn set_method_name(self, method_name: Option<String>) {
        self.d_mut(|d| d.method_name = method_name)
    }

    pub fn set_method_descriptor(self, method_descriptor: Option<String>) {
        self.d_mut(|d| d.method_descriptor = method_descriptor)
    }
}
