//! Port of `org/vineflower/kotlin/util/KExprProcessor.java`.

#![allow(unused_variables)]

use crate::statement::Statement;
use crate::text_buffer::TextBuffer;
use crate::unported;

pub struct KExprProcessor;

impl KExprProcessor {
    pub fn jmp_wrapper(stat: Statement, indent: i32, is_switch: bool) -> TextBuffer {
        unported!("KExprProcessor.jmpWrapper")
    }
}
