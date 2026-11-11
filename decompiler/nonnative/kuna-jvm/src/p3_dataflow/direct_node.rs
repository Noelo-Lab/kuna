//! Port of `org/jetbrains/java/decompiler/modules/decompiler/flow/DirectNode.java`.
//!
//! Handle `==` is Java identity; `equals`/`JavaEq`/`JavaHash` are the Java
//! overrides (node type and id). `exprents` aliases the list of the statement
//! the node was flattened from. The edge maps are keyed by the two
//! `DirectEdgeType`s and never iterated, so they are a two-slot array; the
//! hash is fixed at construction because type and id never change.

use std::rc::Rc;

use crate::arena_object;
use crate::basic_block_statement::BasicBlockStatement;
use crate::direct_edge::DirectEdge;
use crate::direct_edge_type::DirectEdgeType;
use crate::direct_node_type::DirectNodeType;
use crate::exprent::Exprent;
use crate::rt::{objects_hash, JList, JavaEq, JavaHash};
use crate::statement::Statement;
use crate::validation_helper::ValidationHelper;

pub struct DirectNodeData {
    pub type_: DirectNodeType,
    pub id: Rc<str>,
    pub block: Option<BasicBlockStatement>,
    pub statement: Statement,
    pub exprents: JList<Exprent>,
    successors: [Option<JList<DirectEdge>>; 2],
    predecessors: [Option<JList<DirectEdge>>; 2],
    pub try_finally: Option<DirectNode>,
    hash: i32,
}

arena_object!(pub DirectNode => DirectNodeData, Class);

impl DirectNode {
    fn new(
        type_: DirectNodeType,
        statement: Statement,
        try_finally: Option<DirectNode>,
    ) -> DirectNode {
        let id: Rc<str> = type_.make_id(statement.with(|d| d.id)).into();
        let hash = objects_hash(&[type_.java_hash(), id.java_hash()]);
        DirectNode::alloc(DirectNodeData {
            type_,
            id,
            block: None,
            statement,
            exprents: JList::new(),
            successors: [None, None],
            predecessors: [None, None],
            try_finally,
            hash,
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
        self.with(|d| d.id.to_string())
    }

    /// The id as a shared string: a key for the id-keyed maps without a copy.
    pub fn id_key(self) -> Rc<str> {
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
        self.with(|d| d.successors[type_ as usize].as_ref().is_some_and(|l| !l.is_empty()))
    }

    pub fn get_successors(self, type_: DirectEdgeType) -> JList<DirectEdge> {
        self.with_mut(|d| d.successors[type_ as usize].get_or_insert_with(JList::new).clone())
    }

    pub fn has_predecessors(self, type_: DirectEdgeType) -> bool {
        self.with(|d| d.predecessors[type_ as usize].as_ref().is_some_and(|l| !l.is_empty()))
    }

    pub fn get_predecessors(self, type_: DirectEdgeType) -> JList<DirectEdge> {
        self.with_mut(|d| d.predecessors[type_ as usize].get_or_insert_with(JList::new).clone())
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

    /// Every node this one references: its try-finally node and the ends of its edges.
    pub(crate) fn linked_nodes(self) -> Vec<DirectNode> {
        self.with(|d| {
            let mut out: Vec<DirectNode> = d.try_finally.into_iter().collect();
            for edges in d.successors.iter().chain(&d.predecessors).flatten() {
                for edge in edges.borrow().iter() {
                    out.push(edge.get_source());
                    out.push(edge.get_destination());
                }
            }
            out
        })
    }

    pub fn equals(self, that: DirectNode) -> bool {
        if self == that {
            return true;
        }
        that.with(|t| self.with(|d| d.type_ == t.type_ && d.id == t.id))
    }

    pub fn hash_code(self) -> i32 {
        self.with(|d| d.hash)
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
