//! Port of `org/jetbrains/java/decompiler/struct/StructRecordComponent.java`.
//!
//! `StructRecordComponent extends StructField`: a view over a `StructField`
//! whose data is flagged `record_component`; it derefs to the field.

use crate::byte_cursor::ByteCursor;
use crate::bytecode_version::BytecodeVersion;
use crate::constant_pool::ConstantPool;
use crate::generic_field_descriptor::GenericFieldDescriptor;
use crate::jthrow;
use crate::pooled_constant::PooledConstant;
use crate::rt::{JavaEq, JavaHash};
use crate::struct_field::{signature_of, StructField};
use crate::struct_member::{AttributeMap, StructMember};

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct StructRecordComponent(pub StructField);

impl std::ops::Deref for StructRecordComponent {
    type Target = StructField;
    fn deref(&self) -> &StructField {
        &self.0
    }
}

impl From<StructRecordComponent> for StructField {
    fn from(view: StructRecordComponent) -> StructField {
        view.0
    }
}

impl From<StructRecordComponent> for StructMember {
    fn from(view: StructRecordComponent) -> StructMember {
        view.0 .0
    }
}

impl std::fmt::Debug for StructRecordComponent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "StructRecordComponent#{}", self.index())
    }
}

impl std::fmt::Display for StructRecordComponent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

impl JavaEq for StructRecordComponent {
    fn java_equals(&self, other: &Self) -> bool {
        self == other
    }
}

impl JavaHash for StructRecordComponent {
    fn java_hash(&self) -> i32 {
        self.index() as i32
    }
}

impl StructMember {
    pub fn is_record_component(self) -> bool {
        self.try_as_field()
            .is_some_and(|f| f.d(|d| d.record_component))
    }

    pub fn try_as_record_component(self) -> Option<StructRecordComponent> {
        self.is_record_component()
            .then(|| StructRecordComponent(self.as_field()))
    }

    #[track_caller]
    pub fn as_record_component(self) -> StructRecordComponent {
        match self.try_as_record_component() {
            Some(rc) => rc,
            None => crate::rt::throw(
                "ClassCastException",
                "StructMember is not a StructRecordComponent",
            ),
        }
    }
}

impl StructRecordComponent {
    pub fn create(
        in_: &mut ByteCursor,
        pool: ConstantPool,
        version: BytecodeVersion,
    ) -> StructRecordComponent {
        let name_index = in_.read_unsigned_short();
        let descriptor_index = in_.read_unsigned_short();

        let name = primitive_string(pool, name_index);
        let descriptor = primitive_string(pool, descriptor_index);

        let attributes = StructMember::read_attributes(in_, pool, version);
        let signature = signature_of(&attributes);

        StructRecordComponent::new(0, attributes, name, descriptor, signature, version)
    }

    fn new(
        flags: i32,
        attributes: AttributeMap,
        name: String,
        descriptor: String,
        signature: Option<GenericFieldDescriptor>,
        version: BytecodeVersion,
    ) -> StructRecordComponent {
        StructRecordComponent(StructField::new_member(
            flags, attributes, name, descriptor, signature, version, true,
        ))
    }
}

/// `((PrimitiveConstant)pool.getConstant(index)).getString()`.
fn primitive_string(pool: ConstantPool, index: i32) -> String {
    match pool.get_constant(index).expect("constant") {
        PooledConstant::Primitive(p) => p.get_string().to_string(),
        PooledConstant::Link(_) => {
            jthrow!(ClassCastException; "LinkConstant cannot be cast to PrimitiveConstant")
        }
    }
}
