//! Port of `org/vineflower/kotlin/stat/KSwitchStatement.java`.

#![allow(dead_code, unused_variables)]

use crate::statement::Statement;
use crate::switch_statement::SwitchStatement;
use crate::text_buffer::TextBuffer;
use crate::{kotlin_statement, unported};

kotlin_statement!(pub KSwitchStatement: SwitchStatement, Switch,
                  is_k_switch, as_k_switch, try_as_k_switch);

impl KSwitchStatement {
    /// Java: KSwitchStatement(SwitchStatement)
    pub fn new(statement: SwitchStatement) -> KSwitchStatement {
        unported!("KSwitchStatement.<init>")
    }

    pub fn to_java(self, _indent: i32) -> TextBuffer {
        unported!("KSwitchStatement.toJava")
    }

    fn write_case(self, indent: i32, buf: &mut TextBuffer, stat: Statement) {
        unported!("KSwitchStatement.writeCase")
    }
}
