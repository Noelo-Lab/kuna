//! Port of `org/vineflower/kotlin/expr/KSwitchHeadExprent.java`.

use crate::k_dispatch::KExprentExt;
use crate::kotlin_exprent;
use crate::switch_head_exprent::SwitchHeadExprent;
use crate::text_buffer::TextBuffer;

kotlin_exprent!(pub KSwitchHeadExprent: SwitchHeadExprent, SwitchHead,
                is_k_switch_head, as_k_switch_head, try_as_k_switch_head);

impl KSwitchHeadExprent {
    /// Java: KSwitchHeadExprent(SwitchHeadExprent)
    pub fn new(ex: SwitchHeadExprent) -> KSwitchHeadExprent {
        let bytecode = ex.get_bytecode();
        let head = SwitchHeadExprent::new(ex.get_value(), bytecode.as_ref());
        head.0.set_kt(Some(KExprentExt::SwitchHead));
        head.set_case_values(ex.get_case_values());
        KSwitchHeadExprent(head)
    }

    pub fn to_java(self, indent: i32) -> TextBuffer {
        let mut buf = TextBuffer::new();

        buf.append_buffer(&self.get_value().to_java(indent))
            .enclose("when (", ")");
        buf.add_start_bytecode_mapping(self.get_bytecode().as_ref());

        buf
    }
}
