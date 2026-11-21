//! Port of `org/jetbrains/java/decompiler/struct/attr/StructCodeAttribute.java`.

use std::rc::Rc;

use crate::byte_cursor::ByteCursor;
use crate::bytecode_version::BytecodeVersion;
use crate::constant_pool::ConstantPool;
use crate::jthrow;
use crate::struct_general_attribute::{
    attribute_view, StructGeneralAttribute, StructGeneralAttributeKind,
};
use crate::struct_member::{AttributeMap, StructMember};

#[derive(Default)]
pub struct StructCodeAttributeData {
    pub local_variables: i32,
    /// `int code_length; code[]; short exception_table_length; exception_table[]`.
    pub code_and_exception_data: Option<Rc<[u8]>>,
    pub code_attributes: Option<AttributeMap>,
}

attribute_view!(
    StructCodeAttribute,
    Code,
    StructCodeAttributeData,
    is_code,
    as_code,
    try_as_code
);

impl StructCodeAttribute {
    pub fn new() -> StructCodeAttribute {
        StructCodeAttribute::alloc_view(StructCodeAttributeData::default())
    }

    pub fn init_content(self, data: &mut ByteCursor, pool: ConstantPool, version: BytecodeVersion) {
        let predates_java = version.predates_java();
        data.discard(if predates_java { 1 } else { 2 });
        let local_variables = if predates_java {
            data.read_unsigned_byte()
        } else {
            data.read_unsigned_short()
        };
        self.d_mut(|d| d.local_variables = local_variables);
        let code_length = if predates_java {
            data.read_unsigned_short()
        } else {
            data.read_int()
        };
        if code_length < 0 {
            jthrow!(NegativeArraySizeException; "{}", code_length);
        }
        let mut code_data = vec![0u8; code_length as usize];
        data.read_fully(&mut code_data);
        let exc_length = data.read_unsigned_short();
        let mut exception_data = vec![0u8; exc_length as usize * 8];
        data.read_fully(&mut exception_data);
        let mut baos: Vec<u8> = Vec::with_capacity(code_data.len() + 2 + exception_data.len());
        baos.extend_from_slice(&(code_data.len() as i32).to_be_bytes());
        baos.extend_from_slice(&code_data);
        baos.extend_from_slice(&(exc_length as u16).to_be_bytes());
        baos.extend_from_slice(&exception_data);
        self.d_mut(|d| d.code_and_exception_data = Some(Rc::from(baos)));
        let code_attributes = StructMember::read_attributes(data, pool, version);
        self.d_mut(|d| d.code_attributes = Some(code_attributes));
    }

    /// The public field `localVariables`.
    pub fn local_variables(self) -> i32 {
        self.d(|d| d.local_variables)
    }

    /// The public field `codeAndExceptionData`.
    pub fn code_and_exception_data(self) -> Option<Rc<[u8]>> {
        self.d(|d| d.code_and_exception_data.clone())
    }

    /// The entries of the public field `codeAttributes`, in its iteration order.
    pub fn code_attributes(self) -> Vec<(String, StructGeneralAttribute)> {
        self.d(|d| {
            d.code_attributes
                .as_ref()
                .map_or_else(Vec::new, |m| m.entry_vec())
        })
    }
}
