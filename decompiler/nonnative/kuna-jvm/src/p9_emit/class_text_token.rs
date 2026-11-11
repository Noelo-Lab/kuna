//! Port of `org/jetbrains/java/decompiler/util/token/ClassTextToken.java`.

use crate::text_range::TextRange;
use crate::text_token::{TextToken, TextTokenKind};
use crate::text_token_visitor::TextTokenVisitor;

#[derive(Clone, Debug)]
pub struct ClassTextToken {
    pub qualified_name: String,
}

impl ClassTextToken {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(start: i32, length: i32, declaration: bool, qualified_name: String) -> TextToken {
        TextToken::new(
            start,
            length,
            declaration,
            TextTokenKind::Class(ClassTextToken { qualified_name }),
        )
    }

    pub fn visit(&self, token: &TextToken, visitor: &mut dyn TextTokenVisitor) {
        visitor.visit_class(
            TextRange::new(token.start, token.length),
            token.declaration,
            &self.qualified_name,
        );
    }
}
