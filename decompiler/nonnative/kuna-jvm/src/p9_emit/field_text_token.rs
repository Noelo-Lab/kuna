//! Port of `org/jetbrains/java/decompiler/util/token/FieldTextToken.java`.

use crate::field_descriptor::FieldDescriptor;
use crate::text_token::{TextToken, TextTokenKind};

#[derive(Clone)]
pub struct FieldTextToken {
    pub class_name: String,
    pub name: String,
    pub descriptor: FieldDescriptor,
}

impl FieldTextToken {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(
        start: i32,
        length: i32,
        declaration: bool,
        class_name: String,
        name: String,
        descriptor: FieldDescriptor,
    ) -> TextToken {
        TextToken::new(
            start,
            length,
            declaration,
            TextTokenKind::Field(FieldTextToken {
                class_name,
                name,
                descriptor,
            }),
        )
    }
}
