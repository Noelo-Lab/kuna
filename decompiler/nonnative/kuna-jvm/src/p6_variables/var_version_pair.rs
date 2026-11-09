//! Port of `org/jetbrains/java/decompiler/modules/decompiler/vars/VarVersionPair.java`.

use crate::rt::{JavaEq, JavaHash};
use crate::var_exprent::VarExprent;

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub struct VarVersionPair {
    pub var: i32,
    pub version: i32,
}

impl VarVersionPair {
    pub const fn new(var: i32, version: i32) -> VarVersionPair {
        VarVersionPair { var, version }
    }

    /// Java: VarVersionPair(VarExprent)
    pub fn from_var(var: VarExprent) -> VarVersionPair {
        VarVersionPair {
            var: var.get_index(),
            version: var.get_version(),
        }
    }

    pub fn is(&self, var: i32, version: i32) -> bool {
        self.var == var && self.version == version
    }

    pub fn equals(&self, o: &VarVersionPair) -> bool {
        self.var == o.var && self.version == o.version
    }

    pub fn hash_code(&self) -> i32 {
        self.var.wrapping_mul(31).wrapping_add(self.version)
    }
}

impl JavaHash for VarVersionPair {
    fn java_hash(&self) -> i32 {
        self.hash_code()
    }
}

impl JavaEq for VarVersionPair {
    fn java_equals(&self, other: &Self) -> bool {
        self.equals(other)
    }
}

impl std::fmt::Display for VarVersionPair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.var, self.version)
    }
}
