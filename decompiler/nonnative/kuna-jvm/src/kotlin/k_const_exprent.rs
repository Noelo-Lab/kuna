//! Port of `org/vineflower/kotlin/expr/KConstExprent.java`.

use crate::const_exprent::ConstExprent;
use crate::text_buffer::TextBuffer;
use crate::{kotlin_exprent, unported};

kotlin_exprent!(pub KConstExprent: ConstExprent, Const,
                is_k_const, as_k_const, try_as_k_const);

impl KConstExprent {
    pub fn to_java(self, _indent: i32) -> TextBuffer {
        unported!("KConstExprent.toJava")
    }
}
