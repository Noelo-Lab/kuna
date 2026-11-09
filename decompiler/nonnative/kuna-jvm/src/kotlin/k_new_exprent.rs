//! Port of `org/vineflower/kotlin/expr/KNewExprent.java`.

#![allow(dead_code, unused_variables)]

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
    const REFERENCE_CTOR_DESC: &'static str = "(Ljava/lang/Object;)V";

    /// Java: KNewExprent(NewExprent)
    pub fn new(expr: NewExprent) -> KNewExprent {
        unported!("KNewExprent.<init>")
    }

    pub fn get_precedence(self) -> i32 {
        0
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
