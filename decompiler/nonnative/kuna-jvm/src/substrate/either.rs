//! Port of `org/jetbrains/java/decompiler/util/Either.java`.

use crate::jthrow;

#[derive(Clone, Debug)]
pub struct Either<L, R> {
    left: Option<L>,
    right: Option<R>,
}

impl<L, R> Either<L, R> {
    pub fn left(left: L) -> Either<L, R> {
        Either {
            left: Some(left),
            right: None,
        }
    }

    pub fn right(right: R) -> Either<L, R> {
        Either {
            left: None,
            right: Some(right),
        }
    }

    pub fn map(&self, if_left: impl FnOnce(&L), if_right: impl FnOnce(&R)) {
        self.check_invariants();
        if let Some(left) = &self.left {
            if_left(left);
        }
        if let Some(right) = &self.right {
            if_right(right);
        }
    }

    fn check_invariants(&self) {
        if self.left.is_none() && self.right.is_none() {
            jthrow!(IllegalStateException; "Either is empty!");
        }
        if self.left.is_some() && self.right.is_some() {
            jthrow!(IllegalStateException; "Either is full!");
        }
    }
}
