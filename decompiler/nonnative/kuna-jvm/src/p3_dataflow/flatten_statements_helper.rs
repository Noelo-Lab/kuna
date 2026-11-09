//! Port of `org/jetbrains/java/decompiler/modules/decompiler/flow/FlattenStatementsHelper.java`.
//!
//! A plain owned helper (never stored). The try-node lists are `JList`: each
//! is held both by a `flattenStatement` frame and by `tryNodesStack`, and the
//! two are compared by identity.
#![allow(dead_code, unused_variables)]

use crate::direct_graph::DirectGraph;
use crate::direct_node::DirectNode;
use crate::direct_node_type::DirectNodeType;
use crate::exprent::Exprent;
use crate::jthrow;
use crate::list_stack::ListStack;
use crate::root_statement::RootStatement;
use crate::rt::{objects_hash, JHashMap, JList, JavaEq, JavaHash};
use crate::stat_edge::StatEdge;
use crate::statement::Statement;
use crate::unported;

pub struct FlattenStatementsHelper {
    map_regular_destination_nodes: JHashMap<Statement, DirectNode>,
    map_continue_destination_nodes: JHashMap<Statement, DirectNode>,
    indirect_edges: Vec<Edge>,
    map_pos_if_branch: JHashMap<DirectNode, Statement>,
    try_nodes_stack: ListStack<JList<DirectNode>>,
    finally_nodes_stack: ListStack<DirectNode>,
    graph: Option<DirectGraph>,
    root: Option<RootStatement>,
}

impl Default for FlattenStatementsHelper {
    fn default() -> Self {
        Self::new()
    }
}

impl FlattenStatementsHelper {
    pub fn new() -> FlattenStatementsHelper {
        FlattenStatementsHelper {
            map_regular_destination_nodes: JHashMap::new(),
            map_continue_destination_nodes: JHashMap::new(),
            indirect_edges: Vec::new(),
            map_pos_if_branch: JHashMap::new(),
            try_nodes_stack: ListStack::new(),
            finally_nodes_stack: ListStack::new(),
            graph: None,
            root: None,
        }
    }

    fn graph(&self) -> DirectGraph {
        self.graph.expect("FlattenStatementsHelper.graph")
    }

    pub fn build(root: RootStatement) -> DirectGraph {
        FlattenStatementsHelper::new().build_direct_graph(root)
    }

    pub fn build_direct_graph(&mut self, root: RootStatement) -> DirectGraph {
        self.root = Some(root);

        self.graph = Some(DirectGraph::new());

        let first = self.flatten_statement(root.into());
        self.graph().set_first(Some(first));

        let dummyexit: Statement = root.get_dummy_exit().into();
        let node = self.create_direct_node(dummyexit);
        self.add_destination(dummyexit, node);

        self.set_edges();

        self.graph().sort_reverse_post_order();

        self.graph()
    }

    /// Java: addDestination(Statement, DirectNode)
    fn add_destination(&mut self, stat: Statement, node: DirectNode) {
        self.add_destination_type(stat, node, Type::Regular);
    }

    /// Java: addDestination(Statement, DirectNode, Edge.Type)
    fn add_destination_type(&mut self, stat: Statement, node: DirectNode, type_: Type) {
        match type_ {
            Type::Regular => {
                self.map_regular_destination_nodes.put(stat, node);
            }
            Type::Continue => {
                self.map_continue_destination_nodes.put(stat, node);
            }
            _ => jthrow!("Unexpected edge type: {}", type_),
        }
    }

    /// Java: createDirectNode(Statement)
    fn create_direct_node(&mut self, stat: Statement) -> DirectNode {
        let direct_node = self.create_direct_node_type(stat, DirectNodeType::Direct);
        if let Some(block) = stat.try_as_basic_block() {
            direct_node.set_block(Some(block));
        }

        direct_node
    }

    /// Java: createDirectNode(Statement, List)
    fn create_direct_node_exprents(
        &mut self,
        stat: Statement,
        exprents: Option<JList<Exprent>>,
    ) -> DirectNode {
        let node = self.create_direct_node(stat);
        if let Some(exprents) = exprents {
            node.set_exprents(exprents);
        }
        node
    }

    /// Java: createDirectNode(Statement, DirectNodeType)
    fn create_direct_node_type(&mut self, stat: Statement, type_: DirectNodeType) -> DirectNode {
        let try_finally = if self.finally_nodes_stack.is_empty() {
            None
        } else {
            Some(self.finally_nodes_stack.peek())
        };
        let node = DirectNode::for_stat(type_, stat, try_finally);
        let id = node.id();
        self.graph().with_mut(|d| d.nodes.add_with_key(node, id));

        if !self.try_nodes_stack.is_empty() {
            self.try_nodes_stack.peek().add(node);
        }

        node
    }

    /// Java: createDirectNode(Statement, DirectNodeType, List)
    fn create_direct_node_type_exprents(
        &mut self,
        stat: Statement,
        type_: DirectNodeType,
        exprents: Option<JList<Exprent>>,
    ) -> DirectNode {
        let node = self.create_direct_node_type(stat, type_);
        if let Some(exprents) = exprents {
            node.set_exprents(exprents);
        }
        node
    }

    fn flatten_statement(&mut self, stat: Statement) -> DirectNode {
        unported!("FlattenStatementsHelper.flattenStatement")
    }

    fn handle_tailed_stat(&mut self, stat: Statement) {
        unported!("FlattenStatementsHelper.handleTailedStat")
    }

    fn handle_loop_end(&mut self, stat: Statement, body: DirectNode) {
        for edge in &self.indirect_edges {
            if edge.stat == stat && edge.type_ == Type::Continue {
                return;
            }
        }

        self.indirect_edges
            .push(Edge::new(body, stat, Type::Continue));
    }

    fn add_edges(&mut self, source_node: DirectNode, lst_succ_edges: &[StatEdge]) {
        for &edge in lst_succ_edges {
            self.add_edge(source_node, edge);
        }
    }

    fn add_edge(&mut self, source_node: DirectNode, edge: StatEdge) {
        let edge_type = edge.get_type();
        let destination = edge.get_destination();
        self.save_edge(source_node, destination, edge_type);
    }

    fn save_edge(&mut self, source_node: DirectNode, destination: Statement, edge_type: i32) {
        self.indirect_edges
            .push(Edge::new_int(source_node, destination, edge_type));
    }

    fn add_edge_if_possible(&mut self, pred_edge: Statement, stat: Statement) {
        let last_basic = self.map_regular_destination_nodes.get(&pred_edge).copied();

        if let Some(last_basic) = last_basic {
            self.indirect_edges
                .push(Edge::new(last_basic, stat, Type::Regular));
        }
    }

    fn set_edges(&mut self) {
        unported!("FlattenStatementsHelper.setEdges")
    }

    pub fn get_direct_node(&self, stat: Statement) -> Option<DirectNode> {
        self.map_regular_destination_nodes.get(&stat).copied()
    }

    fn get_destination(&self, edge: &Edge) -> Option<DirectNode> {
        let map = if edge.type_ == Type::Continue {
            &self.map_continue_destination_nodes
        } else {
            &self.map_regular_destination_nodes
        };
        map.get(&edge.stat).copied()
    }
}

/// Java: `FlattenStatementsHelper.Edge`.
#[derive(Clone, Copy, Debug)]
pub struct Edge {
    pub source: DirectNode,
    pub stat: Statement,
    pub type_: Type,
}

impl Edge {
    /// Java: Edge(DirectNode, Statement, Edge.Type)
    pub fn new(source: DirectNode, stat: Statement, type_: Type) -> Edge {
        Edge {
            source,
            stat,
            type_,
        }
    }

    /// Java: Edge(DirectNode, Statement, int)
    pub fn new_int(source: DirectNode, stat: Statement, edgetype: i32) -> Edge {
        let type_ = if edgetype == StatEdge::TYPE_CONTINUE {
            Type::Continue
        } else if edgetype == StatEdge::TYPE_FINALLYEXIT {
            Type::FinallyExit
        } else if edgetype == StatEdge::TYPE_EXCEPTION {
            Type::Exception
        } else {
            Type::Regular
        };
        Edge {
            source,
            stat,
            type_,
        }
    }

    pub fn equals(&self, edge: &Edge) -> bool {
        self.type_ == edge.type_
            && self.source.equals(edge.source)
            && self.stat.java_equals(&edge.stat)
    }

    pub fn hash_code(&self) -> i32 {
        objects_hash(&[
            self.source.java_hash(),
            self.stat.java_hash(),
            self.type_.java_hash(),
        ])
    }
}

impl JavaEq for Edge {
    fn java_equals(&self, other: &Self) -> bool {
        self.equals(other)
    }
}

impl JavaHash for Edge {
    fn java_hash(&self) -> i32 {
        self.hash_code()
    }
}

impl std::fmt::Display for Edge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Source: {} Dest: {} Edge: {}",
            self.source, self.stat, self.type_
        )
    }
}

/// Java: `FlattenStatementsHelper.Edge.Type`.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub enum Type {
    Regular,
    Continue,
    Exception,
    FinallyExit,
}

impl Type {
    pub fn ordinal(self) -> i32 {
        self as i32
    }

    pub fn name(self) -> &'static str {
        match self {
            Type::Regular => "REGULAR",
            Type::Continue => "CONTINUE",
            Type::Exception => "EXCEPTION",
            Type::FinallyExit => "FINALLY_EXIT",
        }
    }
}

impl JavaHash for Type {
    fn java_hash(&self) -> i32 {
        self.ordinal()
    }
}

impl JavaEq for Type {
    fn java_equals(&self, other: &Self) -> bool {
        self == other
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
