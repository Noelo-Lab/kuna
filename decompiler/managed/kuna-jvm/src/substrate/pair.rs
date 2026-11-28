//! Port of `org/jetbrains/java/decompiler/util/Pair.java`.

use crate::rt::{JavaEq, JavaHash};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Pair<A, B> {
    pub a: A,
    pub b: B,
}

impl<A, B> Pair<A, B> {
    pub fn of(a: A, b: B) -> Pair<A, B> {
        Pair { a, b }
    }
}

impl<A: JavaHash, B: JavaHash> JavaHash for Pair<A, B> {
    fn java_hash(&self) -> i32 {
        self.a.java_hash() ^ self.b.java_hash()
    }
}

impl<A: JavaEq, B: JavaEq> JavaEq for Pair<A, B> {
    fn java_equals(&self, other: &Self) -> bool {
        self.a.java_equals(&other.a) && self.b.java_equals(&other.b)
    }
}

impl<A: std::fmt::Display, B: std::fmt::Display> std::fmt::Display for Pair<A, B> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Pair{{{},{}}}", self.a, self.b)
    }
}
