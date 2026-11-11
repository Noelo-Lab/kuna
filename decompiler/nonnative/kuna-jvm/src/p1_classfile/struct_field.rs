//! Port of `org/jetbrains/java/decompiler/struct/StructField.java`.

use crate::byte_cursor::ByteCursor;
use crate::bytecode_version::BytecodeVersion;
use crate::constant_pool::ConstantPool;
use crate::decompiler_context::DecompilerContext;
use crate::generic_field_descriptor::GenericFieldDescriptor;
use crate::generic_main::GenericMain;
use crate::i_fernflower_preferences::IFernflowerPreferences;
use crate::java_subclass;
use crate::rt::{JavaEq, JavaHash};
use crate::struct_general_attribute::StructGeneralAttribute;
use crate::struct_member::{AttributeMap, StructMember, StructMemberKind};

pub struct StructFieldData {
    pub name: String,
    pub descriptor: String,
    pub signature: Option<GenericFieldDescriptor>,
    pub version: BytecodeVersion,
    /// Set for the `StructRecordComponent` subclass.
    pub record_component: bool,
}

java_subclass!(pub StructField: StructMember, StructMemberKind::Field(StructFieldData), is_field, as_field, try_as_field);

impl JavaEq for StructField {
    fn java_equals(&self, other: &Self) -> bool {
        self == other
    }
}

impl JavaHash for StructField {
    fn java_hash(&self) -> i32 {
        self.index() as i32
    }
}

impl StructField {
    pub fn create(
        in_: &mut ByteCursor,
        pool: ConstantPool,
        cl_qualified_name: &str,
        version: BytecodeVersion,
    ) -> StructField {
        let access_flags = in_.read_unsigned_short();
        let name_index = in_.read_unsigned_short();
        let descriptor_index = in_.read_unsigned_short();

        let values = pool.get_class_element(
            ConstantPool::FIELD,
            cl_qualified_name,
            name_index,
            descriptor_index,
        );

        let attributes = StructMember::read_attributes(in_, pool, version);
        let signature = signature_of(&attributes);

        StructField::new_signature(
            access_flags,
            attributes,
            values[0].clone(),
            values[1].clone(),
            signature,
            version,
        )
    }

    /// Java: `StructField(int, Map, String, String, BytecodeVersion)`.
    pub fn new(
        access_flags: i32,
        attributes: AttributeMap,
        name: String,
        descriptor: String,
        version: BytecodeVersion,
    ) -> StructField {
        StructField::new_signature(access_flags, attributes, name, descriptor, None, version)
    }

    /// Java: `StructField(int, Map, String, String, GenericFieldDescriptor, BytecodeVersion)`.
    pub fn new_signature(
        access_flags: i32,
        attributes: AttributeMap,
        name: String,
        descriptor: String,
        signature: Option<GenericFieldDescriptor>,
        version: BytecodeVersion,
    ) -> StructField {
        StructField::new_member(
            access_flags,
            attributes,
            name,
            descriptor,
            signature,
            version,
            false,
        )
    }

    pub(crate) fn new_member(
        access_flags: i32,
        attributes: AttributeMap,
        name: String,
        descriptor: String,
        signature: Option<GenericFieldDescriptor>,
        version: BytecodeVersion,
        record_component: bool,
    ) -> StructField {
        StructField(StructMember::new_member(
            access_flags,
            attributes,
            StructMemberKind::Field(StructFieldData {
                name,
                descriptor,
                signature,
                version,
                record_component,
            }),
        ))
    }

    pub fn get_name(self) -> String {
        self.d(|d| d.name.clone())
    }

    pub fn get_descriptor(self) -> String {
        self.d(|d| d.descriptor.clone())
    }

    pub fn get_signature(self) -> Option<GenericFieldDescriptor> {
        self.d(|d| d.signature.clone())
    }

    pub fn get_version(self) -> BytecodeVersion {
        self.d(|d| d.version)
    }
}

/// The `Signature` parse `StructField.create` and `StructRecordComponent.create` share.
pub(crate) fn signature_of(attributes: &AttributeMap) -> Option<GenericFieldDescriptor> {
    if DecompilerContext::get_option(IFernflowerPreferences::DECOMPILE_GENERIC_SIGNATURES) {
        let signature_attr = attributes
            .get_cloned(&StructGeneralAttribute::ATTRIBUTE_SIGNATURE.name.to_string())
            .map(|a| a.as_generic_signature());
        if let Some(signature_attr) = signature_attr {
            return GenericMain::parse_field_signature(&signature_attr.get_signature());
        }
    }
    None
}

impl std::fmt::Display for StructField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.get_name())
    }
}
