//! Port of `org/jetbrains/java/decompiler/struct/consts/PooledConstant.java`.
//!
//! The base class has exactly two subclasses, so it is a closed enum.

use crate::constant_pool::ConstantPool;
use crate::link_constant::LinkConstant;
use crate::primitive_constant::PrimitiveConstant;
use crate::rt::JavaEq;

#[derive(Clone, Debug)]
pub enum PooledConstant {
    Primitive(PrimitiveConstant),
    Link(LinkConstant),
}

impl From<PrimitiveConstant> for PooledConstant {
    fn from(c: PrimitiveConstant) -> PooledConstant {
        PooledConstant::Primitive(c)
    }
}

impl From<LinkConstant> for PooledConstant {
    fn from(c: LinkConstant) -> PooledConstant {
        PooledConstant::Link(c)
    }
}

impl PooledConstant {
    /// The `type` field.
    pub fn type_(&self) -> i32 {
        match self {
            PooledConstant::Primitive(c) => c.type_,
            PooledConstant::Link(c) => c.type_,
        }
    }

    pub fn is_primitive(&self) -> bool {
        matches!(self, PooledConstant::Primitive(_))
    }

    pub fn try_as_primitive(&self) -> Option<PrimitiveConstant> {
        match self {
            PooledConstant::Primitive(c) => Some(c.clone()),
            PooledConstant::Link(_) => None,
        }
    }

    #[track_caller]
    pub fn as_primitive(&self) -> PrimitiveConstant {
        match self {
            PooledConstant::Primitive(c) => c.clone(),
            PooledConstant::Link(_) => crate::rt::throw(
                "ClassCastException",
                "LinkConstant cannot be cast to PrimitiveConstant",
            ),
        }
    }

    pub fn is_link(&self) -> bool {
        matches!(self, PooledConstant::Link(_))
    }

    pub fn try_as_link(&self) -> Option<LinkConstant> {
        match self {
            PooledConstant::Link(c) => Some(c.clone()),
            PooledConstant::Primitive(_) => None,
        }
    }

    #[track_caller]
    pub fn as_link(&self) -> LinkConstant {
        match self {
            PooledConstant::Link(c) => c.clone(),
            PooledConstant::Primitive(_) => crate::rt::throw(
                "ClassCastException",
                "PrimitiveConstant cannot be cast to LinkConstant",
            ),
        }
    }

    pub fn resolve_constant(&mut self, pool: ConstantPool) {
        match self {
            PooledConstant::Primitive(c) => c.resolve_constant(pool),
            PooledConstant::Link(c) => c.resolve_constant(pool),
        }
    }

    /// Java `==`.
    pub fn ptr_eq(&self, other: &PooledConstant) -> bool {
        match (self, other) {
            (PooledConstant::Primitive(a), PooledConstant::Primitive(b)) => a.ptr_eq(b),
            (PooledConstant::Link(a), PooledConstant::Link(b)) => a.ptr_eq(b),
            _ => false,
        }
    }

    pub fn equals(&self, other: &PooledConstant) -> bool {
        match (self, other) {
            (PooledConstant::Primitive(a), PooledConstant::Primitive(b)) => a.equals(b),
            (PooledConstant::Link(a), PooledConstant::Link(b)) => a.equals(b),
            _ => false,
        }
    }
}

impl JavaEq for PooledConstant {
    fn java_equals(&self, other: &Self) -> bool {
        self.equals(other)
    }
}
