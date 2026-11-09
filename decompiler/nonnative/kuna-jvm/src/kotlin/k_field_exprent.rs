//! Port of `org/vineflower/kotlin/expr/KFieldExprent.java`.

use crate::field_exprent::FieldExprent;
use crate::text_buffer::TextBuffer;
use crate::{kotlin_exprent, unported};

kotlin_exprent!(pub KFieldExprent: FieldExprent, Field,
                is_k_field, as_k_field, try_as_k_field);

impl KFieldExprent {
    pub fn to_java(self, _indent: i32) -> TextBuffer {
        unported!("KFieldExprent.toJava")
    }
}
