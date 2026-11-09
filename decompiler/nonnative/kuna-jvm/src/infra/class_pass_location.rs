//! Port of `org/jetbrains/java/decompiler/api/java/ClassPassLocation.java`.

use crate::rt::{JavaEq, JavaHash};

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum ClassPassLocation {
    BeforeProcessing,
    AfterProcessing,
    BeforeWriting,
}

impl JavaHash for ClassPassLocation {
    fn java_hash(&self) -> i32 {
        *self as i32
    }
}

impl JavaEq for ClassPassLocation {
    fn java_equals(&self, other: &Self) -> bool {
        self == other
    }
}
