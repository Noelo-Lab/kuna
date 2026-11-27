//! Port of `org/jetbrains/java/decompiler/modules/decompiler/flow/DirectEdge.java`.
//!
//! An immutable value: `==` is the ported `equals`, which compares the end
//! nodes with `DirectNode.equals` (type and id), not by identity.

use crate::direct_edge_type::DirectEdgeType;
use crate::direct_node::DirectNode;
use crate::rt::{objects_hash, JavaEq, JavaHash};

#[derive(Clone, Copy, Debug)]
pub struct DirectEdge {
    source: DirectNode,
    destination: DirectNode,
    type_: DirectEdgeType,
}

impl DirectEdge {
    pub fn new(source: DirectNode, destination: DirectNode, type_: DirectEdgeType) -> DirectEdge {
        DirectEdge {
            source,
            destination,
            type_,
        }
    }

    pub fn of(source: DirectNode, destination: DirectNode) -> DirectEdge {
        DirectEdge::new(source, destination, DirectEdgeType::Regular)
    }

    pub fn exception(source: DirectNode, destination: DirectNode) -> DirectEdge {
        DirectEdge::new(source, destination, DirectEdgeType::Exception)
    }

    pub fn get_source(&self) -> DirectNode {
        self.source
    }

    pub fn get_destination(&self) -> DirectNode {
        self.destination
    }

    pub fn get_type(&self) -> DirectEdgeType {
        self.type_
    }

    pub fn equals(&self, that: &DirectEdge) -> bool {
        self.source.equals(that.source)
            && self.destination.equals(that.destination)
            && self.type_ == that.type_
    }

    pub fn hash_code(&self) -> i32 {
        objects_hash(&[
            self.source.hash_code(),
            self.destination.hash_code(),
            self.type_.java_hash(),
        ])
    }
}

impl PartialEq for DirectEdge {
    fn eq(&self, other: &Self) -> bool {
        self.equals(other)
    }
}

impl Eq for DirectEdge {}

impl JavaEq for DirectEdge {
    fn java_equals(&self, other: &Self) -> bool {
        self.equals(other)
    }
}

impl JavaHash for DirectEdge {
    fn java_hash(&self) -> i32 {
        self.hash_code()
    }
}

impl std::fmt::Display for DirectEdge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({} -> {} | {})",
            self.source, self.destination, self.type_
        )
    }
}
