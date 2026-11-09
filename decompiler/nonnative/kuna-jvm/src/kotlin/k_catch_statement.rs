//! Port of `org/vineflower/kotlin/stat/KCatchStatement.java`.

#![allow(unused_variables)]

use crate::catch_statement::CatchStatement;
use crate::text_buffer::TextBuffer;
use crate::{kotlin_statement, unported};

kotlin_statement!(pub KCatchStatement: CatchStatement, TryCatch,
                  is_k_catch, as_k_catch, try_as_k_catch);

impl KCatchStatement {
    /// Java: KCatchStatement(CatchStatement)
    pub fn new(statement: CatchStatement) -> KCatchStatement {
        unported!("KCatchStatement.<init>")
    }

    pub fn to_java(self, _indent: i32) -> TextBuffer {
        unported!("KCatchStatement.toJava")
    }
}
