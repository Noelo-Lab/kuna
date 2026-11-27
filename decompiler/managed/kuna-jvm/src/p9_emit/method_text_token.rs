//! Port of `org/jetbrains/java/decompiler/util/token/MethodTextToken.java`.

use crate::method_descriptor::MethodDescriptor;
use crate::text_token::{TextToken, TextTokenKind};

#[derive(Clone)]
pub struct MethodTextToken {
    pub class_name: String,
    pub name: String,
    pub descriptor: MethodDescriptor,
}

impl MethodTextToken {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(
        start: i32,
        length: i32,
        declaration: bool,
        class_name: String,
        name: String,
        descriptor: MethodDescriptor,
    ) -> TextToken {
        TextToken::new(
            start,
            length,
            declaration,
            TextTokenKind::Method(MethodTextToken {
                class_name,
                name,
                descriptor,
            }),
        )
    }
}
