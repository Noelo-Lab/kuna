//! Port of `org/jetbrains/java/decompiler/util/token/ClassTextToken.java`.

use crate::text_token::{TextToken, TextTokenKind};

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
}
