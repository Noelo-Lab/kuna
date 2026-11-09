//! Port of `org/jetbrains/java/decompiler/util/token/MethodTextToken.java`.

use crate::method_descriptor::MethodDescriptor;
use crate::text_range::TextRange;
use crate::text_token::{TextToken, TextTokenKind};
use crate::text_token_visitor::TextTokenVisitor;

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

    pub fn visit(&self, token: &TextToken, visitor: &mut dyn TextTokenVisitor) {
        visitor.visit_method(
            TextRange::new(token.start, token.length),
            token.declaration,
            &self.class_name,
            &self.name,
            &self.descriptor,
        );
    }
}
