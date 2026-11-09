//! Port of `org/jetbrains/java/decompiler/modules/decompiler/flow/DirectEdgeType.java`.

use crate::rt::{JavaEq, JavaHash};

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum DirectEdgeType {
    Regular,
    Exception,
}

impl DirectEdgeType {
    pub const TYPES: [DirectEdgeType; 2] = [DirectEdgeType::Regular, DirectEdgeType::Exception];

    pub fn values() -> [DirectEdgeType; 2] {
        Self::TYPES
    }

    pub fn ordinal(self) -> i32 {
        self as i32
    }

    pub fn name(self) -> &'static str {
        match self {
            DirectEdgeType::Regular => "REGULAR",
            DirectEdgeType::Exception => "EXCEPTION",
        }
    }
}

impl JavaHash for DirectEdgeType {
    fn java_hash(&self) -> i32 {
        self.ordinal()
    }
}

impl JavaEq for DirectEdgeType {
    fn java_equals(&self, other: &Self) -> bool {
        self == other
    }
}

impl std::fmt::Display for DirectEdgeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
