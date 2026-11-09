//! Port of `org/jetbrains/java/decompiler/modules/decompiler/flow/DirectNode.java`.
//!
//! Handle `==` is Java identity; `equals`/`JavaEq`/`JavaHash` are the Java
//! overrides (node type and id). `exprents` aliases the list of the statement
//! the node was flattened from.

use crate::arena_object;
use crate::basic_block_statement::BasicBlockStatement;
use crate::direct_edge::DirectEdge;
use crate::direct_edge_type::DirectEdgeType;
use crate::direct_node_type::DirectNodeType;
use crate::exprent::Exprent;
use crate::rt::{objects_hash, JHashMap, JList, JavaEq, JavaHash};
use crate::statement::Statement;
use crate::validation_helper::ValidationHelper;

pub struct DirectNodeData {
    pub type_: DirectNodeType,
    pub id: String,
    pub block: Option<BasicBlockStatement>,
    pub statement: Statement,
    pub exprents: JList<Exprent>,
    successors: JHashMap<DirectEdgeType, JList<DirectEdge>>,
    predecessors: JHashMap<DirectEdgeType, JList<DirectEdge>>,
    pub try_finally: Option<DirectNode>,
}

arena_object!(pub DirectNode => DirectNodeData, Class);

impl DirectNode {
    fn new(
        type_: DirectNodeType,
        statement: Statement,
        try_finally: Option<DirectNode>,
    ) -> DirectNode {
        let id = type_.make_id(statement.with(|d| d.id));
        DirectNode::alloc(DirectNodeData {
            type_,
            id,
            block: None,
            statement,
            exprents: JList::new(),
            successors: JHashMap::new(),
            predecessors: JHashMap::new(),
            try_finally,
        })
    }

    pub fn for_stat(
        type_: DirectNodeType,
        statement: Statement,
        try_finally: Option<DirectNode>,
    ) -> DirectNode {
        DirectNode::new(type_, statement, try_finally)
    }

    pub fn type_(self) -> DirectNodeType {
        self.with(|d| d.type_)
    }

    pub fn id(self) -> String {
        self.with(|d| d.id.clone())
    }

    pub fn block(self) -> Option<BasicBlockStatement> {
        self.with(|d| d.block)
    }

    pub fn set_block(self, block: Option<BasicBlockStatement>) {
        self.with_mut(|d| d.block = block)
    }

    pub fn statement(self) -> Statement {
        self.with(|d| d.statement)
    }

    /// The node's exprent list (an alias, like the Java field).
    pub fn exprents(self) -> JList<Exprent> {
        self.with(|d| d.exprents.clone())
    }

    pub fn set_exprents(self, exprents: JList<Exprent>) {
        self.with_mut(|d| d.exprents = exprents)
    }

    pub fn try_finally(self) -> Option<DirectNode> {
        self.with(|d| d.try_finally)
    }

    pub fn has_successors(self, type_: DirectEdgeType) -> bool {
        self.with(|d| d.successors.get(&type_).is_some_and(|l| !l.is_empty()))
    }

    pub fn get_successors(self, type_: DirectEdgeType) -> JList<DirectEdge> {
        self.with_mut(|d| d.successors.compute_if_absent(type_, JList::new).clone())
    }

    pub fn has_predecessors(self, type_: DirectEdgeType) -> bool {
        self.with(|d| d.predecessors.get(&type_).is_some_and(|l| !l.is_empty()))
    }

    pub fn get_predecessors(self, type_: DirectEdgeType) -> JList<DirectEdge> {
        self.with_mut(|d| d.predecessors.compute_if_absent(type_, JList::new).clone())
    }

    pub fn add_successor(self, edge: DirectEdge) {
        ValidationHelper::validate_true(edge.get_source() == self, "Source node mismatch");
        if !self.get_successors(edge.get_type()).contains(&edge) {
            self.get_successors(edge.get_type()).add(edge);
        }

        if !edge
            .get_destination()
            .get_predecessors(edge.get_type())
            .contains(&edge)
        {
            edge.get_destination()
                .get_predecessors(edge.get_type())
                .add(edge);
        }
    }

    pub fn equals(self, that: DirectNode) -> bool {
        if self == that {
            return true;
        }
        let (type_, id) = that.with(|d| (d.type_, d.id.clone()));
        self.with(|d| d.type_ == type_ && d.id == id)
    }

    pub fn hash_code(self) -> i32 {
        self.with(|d| objects_hash(&[d.type_.java_hash(), d.id.java_hash()]))
    }
}

impl JavaEq for DirectNode {
    fn java_equals(&self, other: &Self) -> bool {
        self.equals(*other)
    }
}

impl JavaHash for DirectNode {
    fn java_hash(&self) -> i32 {
        self.hash_code()
    }
}

impl std::fmt::Display for DirectNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.with(|d| f.write_str(&d.id))
    }
}
