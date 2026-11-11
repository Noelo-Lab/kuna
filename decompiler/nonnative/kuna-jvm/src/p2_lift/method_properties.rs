//! Port of `org/jetbrains/java/decompiler/code/MethodProperties.java`.
//!
//! A plain value: `ClassWrapper` owns one per method and
//! `MethodPropertiesProcessor::parse` fills it through `&mut`.

use crate::struct_field::StructField;
use crate::struct_method::StructMethod;

#[derive(Clone, Debug)]
pub struct MethodProperties {
    pub mt: StructMethod,
    pub is_synthetic_reference_lambda: bool,
    pub assert_field: Option<StructField>,
}

impl MethodProperties {
    pub fn new(mt: StructMethod) -> MethodProperties {
        MethodProperties {
            mt,
            is_synthetic_reference_lambda: false,
            assert_field: None,
        }
    }
}
