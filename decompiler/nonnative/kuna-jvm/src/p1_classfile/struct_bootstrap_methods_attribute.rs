//! Port of `org/jetbrains/java/decompiler/struct/attr/StructBootstrapMethodsAttribute.java`.
//!
//! Pool lookups keep Java's `null` for an empty slot.

use crate::byte_cursor::ByteCursor;
use crate::bytecode_version::BytecodeVersion;
use crate::constant_pool::ConstantPool;
use crate::link_constant::LinkConstant;
use crate::pooled_constant::PooledConstant;
use crate::struct_general_attribute::{
    attribute_view, StructGeneralAttribute, StructGeneralAttributeKind,
};

#[derive(Default)]
pub struct StructBootstrapMethodsAttributeData {
    pub method_refs: Vec<Option<LinkConstant>>,
    pub method_arguments: Vec<Vec<Option<PooledConstant>>>,
}

attribute_view!(
    StructBootstrapMethodsAttribute,
    BootstrapMethods,
    StructBootstrapMethodsAttributeData,
    is_bootstrap_methods,
    as_bootstrap_methods,
    try_as_bootstrap_methods
);

impl StructBootstrapMethodsAttribute {
    pub fn new() -> StructBootstrapMethodsAttribute {
        StructBootstrapMethodsAttribute::alloc_view(StructBootstrapMethodsAttributeData::default())
    }

    pub fn init_content(
        self,
        data: &mut ByteCursor,
        pool: ConstantPool,
        _version: BytecodeVersion,
    ) {
        let method_number = data.read_unsigned_short();

        for _ in 0..method_number {
            let bootstrap_method_ref = data.read_unsigned_short();
            let num_bootstrap_arguments = data.read_unsigned_short();

            let mut list_arguments = Vec::new();

            for _ in 0..num_bootstrap_arguments {
                let bootstrap_argument_ref = data.read_unsigned_short();

                list_arguments.push(pool.get_constant(bootstrap_argument_ref));
            }

            let method_ref = pool.get_link_constant(bootstrap_method_ref);
            self.d_mut(|d| {
                d.method_refs.push(method_ref);
                d.method_arguments.push(list_arguments);
            });
        }
    }

    /// The public field `methodRefs`.
    pub fn method_refs(self) -> Vec<Option<LinkConstant>> {
        self.d(|d| d.method_refs.clone())
    }

    /// The public field `methodArguments`.
    pub fn method_arguments(self) -> Vec<Vec<Option<PooledConstant>>> {
        self.d(|d| d.method_arguments.clone())
    }

    pub fn get_methods_number(self) -> i32 {
        self.d(|d| d.method_refs.len() as i32)
    }

    pub fn get_method_reference(self, index: i32) -> Option<LinkConstant> {
        self.d(|d| d.method_refs[index as usize].clone())
    }

    pub fn get_method_arguments(self, index: i32) -> Vec<Option<PooledConstant>> {
        self.d(|d| d.method_arguments[index as usize].clone())
    }
}
