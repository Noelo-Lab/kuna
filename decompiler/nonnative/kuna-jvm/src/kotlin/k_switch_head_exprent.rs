//! Port of `org/vineflower/kotlin/expr/KSwitchHeadExprent.java`.

use crate::switch_head_exprent::SwitchHeadExprent;
use crate::text_buffer::TextBuffer;
use crate::{kotlin_exprent, unported};

kotlin_exprent!(pub KSwitchHeadExprent: SwitchHeadExprent, SwitchHead,
                is_k_switch_head, as_k_switch_head, try_as_k_switch_head);

impl KSwitchHeadExprent {
    pub fn to_java(self, _indent: i32) -> TextBuffer {
        unported!("KSwitchHeadExprent.toJava")
    }
}
