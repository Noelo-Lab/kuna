//! Port of `org/jetbrains/java/decompiler/struct/gen/TypeFamily.java`.

use crate::rt::{JavaEq, JavaHash};

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum TypeFamily {
    Unknown,
    Boolean,
    Integer,
    Float,
    Long,
    Double,
    Object,
}

impl TypeFamily {
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    /// Java `name()`.
    pub fn name(self) -> &'static str {
        match self {
            TypeFamily::Unknown => "UNKNOWN",
            TypeFamily::Boolean => "BOOLEAN",
            TypeFamily::Integer => "INTEGER",
            TypeFamily::Float => "FLOAT",
            TypeFamily::Long => "LONG",
            TypeFamily::Double => "DOUBLE",
            TypeFamily::Object => "OBJECT",
        }
    }

    pub fn is_numeric(self) -> bool {
        matches!(
            self,
            TypeFamily::Integer | TypeFamily::Float | TypeFamily::Long | TypeFamily::Double
        )
    }

    pub fn int_or_bool(self) -> bool {
        self == TypeFamily::Integer || self == TypeFamily::Boolean
    }

    pub fn is_greater(self, other: TypeFamily) -> bool {
        self.ordinal() > other.ordinal()
    }

    pub fn is_lesser(self, other: TypeFamily) -> bool {
        self.ordinal() < other.ordinal()
    }
}

impl std::fmt::Display for TypeFamily {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

impl JavaHash for TypeFamily {
    fn java_hash(&self) -> i32 {
        *self as i32
    }
}

impl JavaEq for TypeFamily {
    fn java_equals(&self, other: &Self) -> bool {
        self == other
    }
}
