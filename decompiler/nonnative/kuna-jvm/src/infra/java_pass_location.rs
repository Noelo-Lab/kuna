//! Port of `org/jetbrains/java/decompiler/api/java/JavaPassLocation.java`.

use crate::rt::{JavaEq, JavaHash};

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum JavaPassLocation {
    CfgConstruction,
    CfgPreStatement,
    BeforeMain,
    InLoopDecomp,
    MainLoop,
    AfterMain,
    AtEnd,
    LateProcessing,
}

impl JavaPassLocation {
    /// Whether a pass reporting a change at this location restarts the enclosing loop.
    pub fn is_loop(self) -> bool {
        match self {
            JavaPassLocation::CfgConstruction => false,
            JavaPassLocation::CfgPreStatement => false,
            JavaPassLocation::BeforeMain => false,
            JavaPassLocation::InLoopDecomp => true,
            JavaPassLocation::MainLoop => true,
            JavaPassLocation::AfterMain => false,
            JavaPassLocation::AtEnd => false,
            JavaPassLocation::LateProcessing => false,
        }
    }
}

impl JavaHash for JavaPassLocation {
    fn java_hash(&self) -> i32 {
        *self as i32
    }
}

impl JavaEq for JavaPassLocation {
    fn java_equals(&self, other: &Self) -> bool {
        self == other
    }
}
