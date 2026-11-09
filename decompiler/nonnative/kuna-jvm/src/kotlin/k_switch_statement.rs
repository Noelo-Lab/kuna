//! Port of `org/vineflower/kotlin/stat/KSwitchStatement.java`.

use crate::switch_statement::SwitchStatement;
use crate::text_buffer::TextBuffer;
use crate::{kotlin_statement, unported};

kotlin_statement!(pub KSwitchStatement: SwitchStatement, Switch,
                  is_k_switch, as_k_switch, try_as_k_switch);

impl KSwitchStatement {
    pub fn to_java(self, _indent: i32) -> TextBuffer {
        unported!("KSwitchStatement.toJava")
    }
}
