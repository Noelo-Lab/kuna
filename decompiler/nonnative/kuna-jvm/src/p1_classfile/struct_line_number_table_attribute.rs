//! Port of `org/jetbrains/java/decompiler/struct/attr/StructLineNumberTableAttribute.java`.

use crate::byte_cursor::ByteCursor;
use crate::bytecode_version::BytecodeVersion;
use crate::constant_pool::ConstantPool;
use crate::struct_general_attribute::{
    attribute_view, StructGeneralAttribute, StructGeneralAttributeKind,
};

#[derive(Default)]
pub struct StructLineNumberTableAttributeData {
    pub my_line_info: Vec<i32>,
}

attribute_view!(
    StructLineNumberTableAttribute,
    LineNumberTable,
    StructLineNumberTableAttributeData,
    is_line_number_table,
    as_line_number_table,
    try_as_line_number_table
);

impl StructLineNumberTableAttribute {
    pub fn new() -> StructLineNumberTableAttribute {
        StructLineNumberTableAttribute::alloc_view(StructLineNumberTableAttributeData::default())
    }

    pub fn init_content(
        self,
        data: &mut ByteCursor,
        _pool: ConstantPool,
        _version: BytecodeVersion,
    ) {
        let len = data.read_unsigned_short() * 2;
        if len > 0 {
            let mut my_line_info = vec![0; len as usize];
            let mut i = 0;
            while i < len as usize {
                my_line_info[i] = data.read_unsigned_short();
                my_line_info[i + 1] = data.read_unsigned_short();
                i += 2;
            }
            self.d_mut(|d| d.my_line_info = my_line_info);
        } else {
            self.d_mut(|d| {
                if !d.my_line_info.is_empty() {
                    d.my_line_info = Vec::new();
                }
            });
        }
    }

    pub fn find_line_number(self, pc: i32) -> i32 {
        self.d(|d| {
            let info = &d.my_line_info;
            if info.len() >= 2 {
                let mut i = info.len() as i32 - 2;
                while i >= 0 {
                    if pc >= info[i as usize] {
                        return info[i as usize + 1];
                    }
                    i -= 2;
                }
            }
            -1
        })
    }

    pub fn get_raw_data(self) -> Vec<i32> {
        self.d(|d| d.my_line_info.clone())
    }

    pub fn set_line_info(self, my_line_info: Vec<i32>) {
        self.d_mut(|d| d.my_line_info = my_line_info)
    }
}
