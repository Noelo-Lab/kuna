//! Port of `org/jetbrains/java/decompiler/struct/attr/StructMethodParametersAttribute.java`.

use crate::byte_cursor::ByteCursor;
use crate::bytecode_version::BytecodeVersion;
use crate::constant_pool::ConstantPool;
use crate::struct_general_attribute::{
    attribute_view, StructGeneralAttribute, StructGeneralAttributeKind,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Entry {
    pub my_name: Option<String>,
    pub my_access_flags: i32,
}

impl Entry {
    pub fn new(name: Option<String>, access_flags: i32) -> Entry {
        Entry {
            my_name: name,
            my_access_flags: access_flags,
        }
    }
}

#[derive(Default)]
pub struct StructMethodParametersAttributeData {
    pub my_entries: Vec<Entry>,
}

attribute_view!(
    StructMethodParametersAttribute,
    MethodParameters,
    StructMethodParametersAttributeData,
    is_method_parameters,
    as_method_parameters,
    try_as_method_parameters
);

impl StructMethodParametersAttribute {
    pub fn new() -> StructMethodParametersAttribute {
        StructMethodParametersAttribute::alloc_view(StructMethodParametersAttributeData::default())
    }

    pub fn init_content(
        self,
        data: &mut ByteCursor,
        pool: ConstantPool,
        _version: BytecodeVersion,
    ) {
        let len = data.read_unsigned_byte();
        let mut entries = Vec::with_capacity(len as usize);
        for _ in 0..len {
            let name_index = data.read_unsigned_short();
            let name = if name_index != 0 {
                Some(
                    pool.get_primitive_constant(name_index)
                        .expect("constant")
                        .get_string(),
                )
            } else {
                None
            };
            let access_flags = data.read_unsigned_short();
            entries.push(Entry::new(name, access_flags));
        }
        self.d_mut(|d| d.my_entries = entries);
    }

    pub fn get_entries(self) -> Vec<Entry> {
        self.d(|d| d.my_entries.clone())
    }

    pub fn set_entries(self, my_entries: Vec<Entry>) {
        self.d_mut(|d| d.my_entries = my_entries)
    }
}
