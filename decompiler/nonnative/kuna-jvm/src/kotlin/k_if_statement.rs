//! Port of `org/vineflower/kotlin/stat/KIfStatement.java`.

#![allow(unused_variables)]

use crate::if_statement::IfStatement;
use crate::text_buffer::TextBuffer;
use crate::{kotlin_statement, unported};

kotlin_statement!(pub KIfStatement: IfStatement, If,
                  is_k_if, as_k_if, try_as_k_if);

impl KIfStatement {
    /// Java: KIfStatement(IfStatement)
    pub fn new(statement: IfStatement) -> KIfStatement {
        unported!("KIfStatement.<init>")
    }

    pub fn to_java(self, _indent: i32) -> TextBuffer {
        unported!("KIfStatement.toJava")
    }
}
