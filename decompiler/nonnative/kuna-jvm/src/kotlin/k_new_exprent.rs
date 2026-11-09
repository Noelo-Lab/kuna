//! Port of `org/vineflower/kotlin/expr/KNewExprent.java`.

use crate::exprent::Exprent;
use crate::new_exprent::NewExprent;
use crate::text_buffer::TextBuffer;
use crate::{kotlin_exprent, unported};

pub struct KNewExprentData {
    pub in_annotation: bool,
}

kotlin_exprent!(pub KNewExprent: NewExprent, New(KNewExprentData),
                is_k_new, as_k_new, try_as_k_new);

impl KNewExprent {
    pub fn get_precedence(self) -> i32 {
        unported!("KNewExprent.getPrecedence")
    }

    pub fn to_java(self, _indent: i32) -> TextBuffer {
        unported!("KNewExprent.toJava")
    }

    pub fn copy(self) -> Exprent {
        unported!("KNewExprent.copy")
    }

    pub fn set_in_annotation(self, in_annotation: bool) {
        self.kd_mut(|d| d.in_annotation = in_annotation)
    }
}
