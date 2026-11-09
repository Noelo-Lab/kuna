//! Port of `org/vineflower/kotlin/stat/KIfStatement.java`.

use crate::if_statement::IfStatement;
use crate::text_buffer::TextBuffer;
use crate::{kotlin_statement, unported};

kotlin_statement!(pub KIfStatement: IfStatement, If,
                  is_k_if, as_k_if, try_as_k_if);

impl KIfStatement {
    pub fn to_java(self, _indent: i32) -> TextBuffer {
        unported!("KIfStatement.toJava")
    }
}
