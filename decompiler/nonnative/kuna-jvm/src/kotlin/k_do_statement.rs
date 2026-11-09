//! Port of `org/vineflower/kotlin/stat/KDoStatement.java`.

#![allow(dead_code, unused_variables)]

use crate::do_statement::DoStatement;
use crate::text_buffer::TextBuffer;
use crate::var_type::VarType;
use crate::{kotlin_statement, unported};

kotlin_statement!(pub KDoStatement: DoStatement, Do,
                  is_k_do, as_k_do, try_as_k_do);

impl KDoStatement {
    /// Java: KDoStatement(DoStatement)
    pub fn new(statement: DoStatement) -> KDoStatement {
        unported!("KDoStatement.<init>")
    }

    fn is_integer_type(type_: &VarType) -> bool {
        VarType::VARTYPE_INT().equals(type_)
            || VarType::VARTYPE_BYTE().equals(type_)
            || VarType::VARTYPE_SHORT().equals(type_)
            || VarType::VARTYPE_CHAR().equals(type_)
            || VarType::VARTYPE_LONG().equals(type_)
    }

    pub fn to_java(self, _indent: i32) -> TextBuffer {
        unported!("KDoStatement.toJava")
    }
}
