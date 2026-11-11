//! Port of `org/jetbrains/java/decompiler/util/token/VariableTextToken.java`.

use crate::method_descriptor::MethodDescriptor;
use crate::text_range::TextRange;
use crate::text_token::{TextToken, TextTokenKind};
use crate::text_token_visitor::TextTokenVisitor;

#[derive(Clone)]
pub struct VariableTextToken {
    pub parameter: bool,
    pub class_name: String,
    pub method_name: String,
    pub method_descriptor: MethodDescriptor,
    pub index: i32,
    pub name: String,
}

impl VariableTextToken {
    #[allow(clippy::too_many_arguments, clippy::new_ret_no_self)]
    pub fn new(
        start: i32,
        length: i32,
        declaration: bool,
        parameter: bool,
        class_name: String,
        method_name: String,
        method_descriptor: MethodDescriptor,
        index: i32,
        name: String,
    ) -> TextToken {
        TextToken::new(
            start,
            length,
            declaration,
            TextTokenKind::Variable(VariableTextToken {
                parameter,
                class_name,
                method_name,
                method_descriptor,
                index,
                name,
            }),
        )
    }

    pub fn visit(&self, token: &TextToken, visitor: &mut dyn TextTokenVisitor) {
        let range = TextRange::new(token.start, token.length);
        if self.parameter {
            visitor.visit_parameter(
                range,
                token.declaration,
                &self.class_name,
                &self.method_name,
                &self.method_descriptor,
                self.index,
                &self.name,
            );
        } else {
            visitor.visit_local(
                range,
                token.declaration,
                &self.class_name,
                &self.method_name,
                &self.method_descriptor,
                self.index,
                &self.name,
            );
        }
    }
}
