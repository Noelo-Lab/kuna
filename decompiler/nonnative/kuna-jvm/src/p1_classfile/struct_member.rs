//! Port of `org/jetbrains/java/decompiler/struct/StructMember.java`.
//!
//! The abstract base is the session-scope arena object; `StructClass`,
//! `StructMethod` and `StructField` are views over [`StructMemberKind`]
//! (`StructRecordComponent` is a flagged `StructField`), so the base methods
//! are callable on every view and a `StructMember` handle is the polymorphic
//! reference Java code passes around. The attribute map is keyed by attribute
//! name, which hashes exactly as Java's `Key` does.

use crate::byte_cursor::ByteCursor;
use crate::bytecode_version::BytecodeVersion;
use crate::code_constants::ACC_SYNTHETIC;
use crate::constant_pool::ConstantPool;
use crate::rt::{JHashMap, Shared};
use crate::struct_class::{StructClass, StructClassData};
use crate::struct_field::{StructField, StructFieldData};
use crate::struct_general_attribute::{AttributeKey, AttributeView, StructGeneralAttribute};
use crate::struct_method::{StructMethod, StructMethodData};
use crate::{arena_identity, arena_object, jthrow};

pub type AttributeMap = JHashMap<String, StructGeneralAttribute>;

pub struct StructMemberData {
    pub access_flags: i32,
    pub attributes: Shared<AttributeMap>,
    pub kind: StructMemberKind,
}

pub enum StructMemberKind {
    Class(StructClassData),
    Method(StructMethodData),
    Field(StructFieldData),
}

arena_object!(pub StructMember => StructMemberData, Session);
arena_identity!(StructMember);

impl StructMember {
    pub(crate) fn new_member(
        access_flags: i32,
        attributes: AttributeMap,
        kind: StructMemberKind,
    ) -> StructMember {
        StructMember::alloc(StructMemberData {
            access_flags,
            attributes: Shared::new(attributes),
            kind,
        })
    }

    pub fn get_access_flags(self) -> i32 {
        self.with(|d| d.access_flags)
    }

    pub fn get_attribute<T: AttributeView>(self, attribute: AttributeKey<T>) -> Option<T> {
        let found = self.with(|d| {
            d.attributes
                .borrow()
                .get_cloned(&attribute.name.to_string())
        });
        found.map(T::from_attribute)
    }

    pub fn has_attribute<T>(self, attribute: AttributeKey<T>) -> bool {
        self.with(|d| {
            d.attributes
                .borrow()
                .contains_key(&attribute.name.to_string())
        })
    }

    /// The live attribute map (Java returns the member's own map).
    pub fn get_attributes(self) -> Shared<AttributeMap> {
        self.with(|d| d.attributes.clone())
    }

    pub fn has_modifier(self, modifier: i32) -> bool {
        (self.get_access_flags() & modifier) == modifier
    }

    pub fn is_synthetic(self) -> bool {
        self.has_modifier(ACC_SYNTHETIC)
            || self.has_attribute(StructGeneralAttribute::ATTRIBUTE_SYNTHETIC)
    }

    /// Java: `readAttributes(DataInputFullStream, ConstantPool, BytecodeVersion)`.
    pub fn read_attributes(
        in_: &mut ByteCursor,
        pool: ConstantPool,
        version: BytecodeVersion,
    ) -> AttributeMap {
        StructMember::read_attributes_code(in_, pool, true, version)
    }

    /// Java: `readAttributes(DataInputFullStream, ConstantPool, boolean, BytecodeVersion)`.
    pub fn read_attributes_code(
        in_: &mut ByteCursor,
        pool: ConstantPool,
        read_code: bool,
        version: BytecodeVersion,
    ) -> AttributeMap {
        let length = in_.read_unsigned_short();
        let mut attributes: AttributeMap = JHashMap::with_capacity(length as usize);

        for _ in 0..length {
            let name_index = in_.read_unsigned_short();
            let name = pool
                .get_primitive_constant(name_index)
                .expect("attribute name")
                .get_string()
                .to_string();

            let attribute = StructGeneralAttribute::create_attribute(&name);
            let att_length = in_.read_int();
            match attribute {
                Some(attribute) if read_code || !attribute.is_code() => {
                    attribute.init_content(in_, pool, version);
                    if StructGeneralAttribute::ATTRIBUTE_LOCAL_VARIABLE_TABLE.name == name
                        && attributes.contains_key(&name)
                    {
                        let table = attributes
                            .get_cloned(&name)
                            .expect("table")
                            .as_local_variable_table();
                        table.add(attribute.as_local_variable_table());
                    } else if StructGeneralAttribute::ATTRIBUTE_LOCAL_VARIABLE_TYPE_TABLE.name
                        == name
                        && attributes.contains_key(&name)
                    {
                        let table = attributes
                            .get_cloned(&name)
                            .expect("table")
                            .as_local_variable_type_table();
                        table.add(attribute.as_local_variable_type_table());
                    } else {
                        attributes.put(name, attribute);
                    }
                }
                _ => discard_bytes(in_, att_length),
            }
        }

        let lvt_name = StructGeneralAttribute::ATTRIBUTE_LOCAL_VARIABLE_TABLE
            .name
            .to_string();
        let lvtt_name = StructGeneralAttribute::ATTRIBUTE_LOCAL_VARIABLE_TYPE_TABLE
            .name
            .to_string();
        if attributes.contains_key(&lvt_name) && attributes.contains_key(&lvtt_name) {
            let lvt = attributes
                .get_cloned(&lvt_name)
                .expect("lvt")
                .as_local_variable_table();
            let lvtt = attributes
                .get_cloned(&lvtt_name)
                .expect("lvtt")
                .as_local_variable_type_table();
            lvt.merge_signatures(lvtt);
        }
        attributes
    }

    pub fn get_version(self) -> BytecodeVersion {
        if let Some(c) = self.try_as_class() {
            StructClass::get_version(c)
        } else if let Some(m) = self.try_as_method() {
            StructMethod::get_version(m)
        } else {
            StructField::get_version(self.as_field())
        }
    }

    pub fn read_attribute(
        self,
        in_: &mut ByteCursor,
        pool: ConstantPool,
        name: &str,
    ) -> Option<StructGeneralAttribute> {
        let attribute = StructGeneralAttribute::create_attribute(name);
        let length = in_.read_int();
        match attribute {
            None => discard_bytes(in_, length),
            Some(attribute) => attribute.init_content(in_, pool, self.get_version()),
        }
        attribute
    }
}

impl std::fmt::Display for StructMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(c) = self.try_as_class() {
            std::fmt::Display::fmt(&c, f)
        } else if let Some(m) = self.try_as_method() {
            std::fmt::Display::fmt(&m, f)
        } else {
            std::fmt::Display::fmt(&self.as_field(), f)
        }
    }
}

/// `discard(int)`: a negative length cannot be skipped, as in `InputStream.skip`.
pub(crate) fn discard_bytes(in_: &mut ByteCursor, length: i32) {
    if length < 0 {
        jthrow!(IOException; "premature end of stream");
    }
    in_.discard(length as usize);
}
