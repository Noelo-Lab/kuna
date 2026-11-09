//! Port of `org/jetbrains/java/decompiler/struct/attr/StructInnerClassesAttribute.java`.

use crate::byte_cursor::ByteCursor;
use crate::bytecode_version::BytecodeVersion;
use crate::constant_pool::ConstantPool;
use crate::struct_general_attribute::{
    attribute_view, StructGeneralAttribute, StructGeneralAttributeKind,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Entry {
    pub outer_name_idx: i32,
    pub simple_name_idx: i32,
    pub access_flags: i32,
    pub inner_name: String,
    pub enclosing_name: Option<String>,
    pub simple_name: Option<String>,
}

impl Entry {
    pub fn new(
        outer_name_idx: i32,
        simple_name_idx: i32,
        access_flags: i32,
        inner_name: String,
        enclosing_name: Option<String>,
        simple_name: Option<String>,
    ) -> Entry {
        Entry {
            outer_name_idx,
            simple_name_idx,
            access_flags,
            inner_name,
            enclosing_name,
            simple_name,
        }
    }
}

#[derive(Default)]
pub struct StructInnerClassesAttributeData {
    pub entries: Vec<Entry>,
}

attribute_view!(
    StructInnerClassesAttribute,
    InnerClasses,
    StructInnerClassesAttributeData,
    is_inner_classes,
    as_inner_classes,
    try_as_inner_classes
);

impl StructInnerClassesAttribute {
    pub fn new() -> StructInnerClassesAttribute {
        StructInnerClassesAttribute::alloc_view(StructInnerClassesAttributeData::default())
    }

    pub fn init_content(
        self,
        data: &mut ByteCursor,
        pool: ConstantPool,
        _version: BytecodeVersion,
    ) {
        let len = data.read_unsigned_short();
        let mut entries = Vec::with_capacity(len as usize);
        for _ in 0..len {
            let inner_name_idx = data.read_unsigned_short();
            let outer_name_idx = data.read_unsigned_short();
            let simple_name_idx = data.read_unsigned_short();
            let access_flags = data.read_unsigned_short();

            let inner_name = pool
                .get_primitive_constant(inner_name_idx)
                .expect("constant")
                .get_string();
            let outer_name = if outer_name_idx != 0 {
                Some(
                    pool.get_primitive_constant(outer_name_idx)
                        .expect("constant")
                        .get_string(),
                )
            } else {
                None
            };
            let simple_name = if simple_name_idx != 0 {
                Some(
                    pool.get_primitive_constant(simple_name_idx)
                        .expect("constant")
                        .get_string(),
                )
            } else {
                None
            };

            entries.push(Entry::new(
                outer_name_idx,
                simple_name_idx,
                access_flags,
                inner_name,
                outer_name,
                simple_name,
            ));
        }
        self.d_mut(|d| d.entries = entries);
    }

    pub fn get_entries(self) -> Vec<Entry> {
        self.d(|d| d.entries.clone())
    }

    pub fn set_entries(self, entries: Vec<Entry>) {
        self.d_mut(|d| d.entries = entries)
    }
}
