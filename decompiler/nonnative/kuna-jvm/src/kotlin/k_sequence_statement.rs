//! Port of `org/vineflower/kotlin/stat/KSequenceStatement.java`.

#![allow(unused_variables)]

use crate::sequence_statement::SequenceStatement;
use crate::text_buffer::TextBuffer;
use crate::{kotlin_statement, unported};

kotlin_statement!(pub KSequenceStatement: SequenceStatement, Sequence,
                  is_k_sequence, as_k_sequence, try_as_k_sequence);

impl KSequenceStatement {
    /// Java: KSequenceStatement(SequenceStatement)
    pub fn new(statement: SequenceStatement) -> KSequenceStatement {
        unported!("KSequenceStatement.<init>")
    }

    pub fn to_java(self, _indent: i32) -> TextBuffer {
        unported!("KSequenceStatement.toJava")
    }
}
