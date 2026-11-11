//! Port of `org/jetbrains/java/decompiler/struct/attr/StructGenericSignatureAttribute.java`.

use crate::byte_cursor::ByteCursor;
use crate::bytecode_version::BytecodeVersion;
use crate::constant_pool::ConstantPool;
use crate::struct_general_attribute::{
    attribute_view, StructGeneralAttribute, StructGeneralAttributeKind,
};

#[derive(Default)]
pub struct StructGenericSignatureAttributeData {
    pub signature: String,
}

attribute_view!(
    StructGenericSignatureAttribute,
    GenericSignature,
    StructGenericSignatureAttributeData,
    is_generic_signature,
    as_generic_signature,
    try_as_generic_signature
);

impl StructGenericSignatureAttribute {
    pub fn new() -> StructGenericSignatureAttribute {
        StructGenericSignatureAttribute::alloc_view(StructGenericSignatureAttributeData::default())
    }

    pub fn init_content(
        self,
        data: &mut ByteCursor,
        pool: ConstantPool,
        _version: BytecodeVersion,
    ) {
        let index = data.read_unsigned_short();
        let signature = pool
            .get_primitive_constant(index)
            .expect("constant")
            .get_string()
            .to_string();
        self.d_mut(|d| d.signature = signature);
    }

    pub fn get_signature(self) -> String {
        self.d(|d| d.signature.clone())
    }

    pub fn set_signature(self, signature: String) {
        self.d_mut(|d| d.signature = signature)
    }
}
