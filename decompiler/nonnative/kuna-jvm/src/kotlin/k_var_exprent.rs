//! Port of `org/vineflower/kotlin/expr/KVarExprent.java`.

use crate::exprent::Exprent;
use crate::text_buffer::TextBuffer;
use crate::var_exprent::VarExprent;
use crate::{kotlin_exprent, unported};

/// `KVarExprent.DeclarationType`.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum DeclarationType {
    Definition,
    Usage,
    ForLoopVariable,
    ExceptionType,
}

impl DeclarationType {
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    pub fn name(self) -> &'static str {
        match self {
            DeclarationType::Definition => "DEFINITION",
            DeclarationType::Usage => "USAGE",
            DeclarationType::ForLoopVariable => "FOR_LOOP_VARIABLE",
            DeclarationType::ExceptionType => "EXCEPTION_TYPE",
        }
    }
}

pub struct KVarExprentData {
    pub declaration_type: Option<DeclarationType>,
}

kotlin_exprent!(pub KVarExprent: VarExprent, Var(KVarExprentData),
                is_k_var, as_k_var, try_as_k_var);

impl KVarExprent {
    pub fn get_name(self) -> String {
        unported!("KVarExprent.getName")
    }

    pub fn to_java(self, _indent: i32) -> TextBuffer {
        unported!("KVarExprent.toJava")
    }

    pub fn set_declaration_type(self, declaration_type: Option<DeclarationType>) {
        self.kd_mut(|d| d.declaration_type = declaration_type)
    }

    pub fn copy(self) -> Exprent {
        unported!("KVarExprent.copy")
    }
}
