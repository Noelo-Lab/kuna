//! Port of `org/jetbrains/java/decompiler/util/StartEndPair.java`.

use crate::rt::{JavaEq, JavaHash};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StartEndPair {
    pub start: i32,
    pub end: i32,
}

impl StartEndPair {
    pub fn new(start: i32, end: i32) -> StartEndPair {
        StartEndPair { start, end }
    }

    /// Java: `join(StartEndPair...)`; `None` entries are skipped like Java's nulls.
    pub fn join(pairs: &[Option<StartEndPair>]) -> StartEndPair {
        let mut start = i32::MAX;
        let mut end = i32::MIN;
        for pair in pairs.iter().flatten() {
            start = start.min(pair.start);
            end = end.max(pair.end);
        }
        StartEndPair::new(start, end)
    }
}

impl JavaEq for StartEndPair {
    fn java_equals(&self, other: &Self) -> bool {
        other.start == self.start && other.end == self.end
    }
}

impl JavaHash for StartEndPair {
    fn java_hash(&self) -> i32 {
        self.start.wrapping_mul(31).wrapping_add(self.end)
    }
}

impl std::fmt::Display for StartEndPair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}->{}", self.start, self.end)
    }
}
