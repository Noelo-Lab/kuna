//! Port of `org/jetbrains/java/decompiler/struct/gen/CodeType.java`.

use crate::rt::{JavaEq, JavaHash};

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum CodeType {
    Byte,
    Char,
    Double,
    Float,
    Int,
    Long,
    Short,
    Boolean,
    Object,
    Address,
    Void,
    Any,
    Group2empty,
    Null,
    Notinitialized,
    Bytechar,
    Shortchar,
    Unknown,
    Genvar,
}

impl CodeType {
    pub const VALUES: [CodeType; 19] = [
        CodeType::Byte,
        CodeType::Char,
        CodeType::Double,
        CodeType::Float,
        CodeType::Int,
        CodeType::Long,
        CodeType::Short,
        CodeType::Boolean,
        CodeType::Object,
        CodeType::Address,
        CodeType::Void,
        CodeType::Any,
        CodeType::Group2empty,
        CodeType::Null,
        CodeType::Notinitialized,
        CodeType::Bytechar,
        CodeType::Shortchar,
        CodeType::Unknown,
        CodeType::Genvar,
    ];

    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// Java `name()`.
    pub fn name(self) -> &'static str {
        match self {
            CodeType::Byte => "BYTE",
            CodeType::Char => "CHAR",
            CodeType::Double => "DOUBLE",
            CodeType::Float => "FLOAT",
            CodeType::Int => "INT",
            CodeType::Long => "LONG",
            CodeType::Short => "SHORT",
            CodeType::Boolean => "BOOLEAN",
            CodeType::Object => "OBJECT",
            CodeType::Address => "ADDRESS",
            CodeType::Void => "VOID",
            CodeType::Any => "ANY",
            CodeType::Group2empty => "GROUP2EMPTY",
            CodeType::Null => "NULL",
            CodeType::Notinitialized => "NOTINITIALIZED",
            CodeType::Bytechar => "BYTECHAR",
            CodeType::Shortchar => "SHORTCHAR",
            CodeType::Unknown => "UNKNOWN",
            CodeType::Genvar => "GENVAR",
        }
    }
}

impl std::fmt::Display for CodeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

impl JavaHash for CodeType {
    fn java_hash(&self) -> i32 {
        *self as i32
    }
}

impl JavaEq for CodeType {
    fn java_equals(&self, other: &Self) -> bool {
        self == other
    }
}
