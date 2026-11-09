//! Port of `org/vineflower/kotlin/stat/KDoStatement.java`.

use crate::do_statement::DoStatement;
use crate::text_buffer::TextBuffer;
use crate::{kotlin_statement, unported};

kotlin_statement!(pub KDoStatement: DoStatement, Do,
                  is_k_do, as_k_do, try_as_k_do);

impl KDoStatement {
    pub fn to_java(self, _indent: i32) -> TextBuffer {
        unported!("KDoStatement.toJava")
    }
}
