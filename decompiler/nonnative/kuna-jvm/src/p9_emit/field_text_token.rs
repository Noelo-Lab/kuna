//! Port of `org/jetbrains/java/decompiler/util/token/FieldTextToken.java`.

use crate::field_descriptor::FieldDescriptor;
use crate::text_range::TextRange;
use crate::text_token::{TextToken, TextTokenKind};
use crate::text_token_visitor::TextTokenVisitor;

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

    pub fn visit(&self, token: &TextToken, visitor: &mut dyn TextTokenVisitor) {
        visitor.visit_field(
            TextRange::new(token.start, token.length),
            token.declaration,
            &self.class_name,
            &self.name,
            &self.descriptor,
        );
    }
}
