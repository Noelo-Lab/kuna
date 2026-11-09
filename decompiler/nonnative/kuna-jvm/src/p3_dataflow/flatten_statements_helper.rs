//! Port of `org/jetbrains/java/decompiler/modules/decompiler/flow/FlattenStatementsHelper.java`.
//!
//! A plain owned helper (never stored). The try-node lists are `JList`: each
//! is held both by a `flattenStatement` frame and by `tryNodesStack`, and the
//! two are compared by identity.
use crate::direct_edge::DirectEdge;
use crate::direct_graph::DirectGraph;
use crate::direct_node::DirectNode;
use crate::direct_node_type::DirectNodeType;
use crate::do_statement::Type as DoType;
use crate::exprent::Exprent;
use crate::if_statement::IfStatement;
use crate::jthrow;
use crate::list_stack::ListStack;
use crate::root_statement::RootStatement;
use crate::rt::{objects_hash, JHashMap, JList, JavaEq, JavaHash};
use crate::stat_edge::StatEdge;
use crate::statement::{LastBasicType, Statement, StatementType};
use crate::validation_helper::ValidationHelper;

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
        match stat.type_() {
            StatementType::BasicBlock => {
                let node = self.create_direct_node(stat);
                self.add_destination(stat, node);
                if let Some(exprents) = stat.get_exprents() {
                    node.set_exprents(exprents);
                }

                if stat.get_last_basic_type() == LastBasicType::If {
                    if !stat.has_any_direct_successor() {
                        jthrow!(IllegalStateException; "Empty successor list for node {}", node.id());
                    }

                    let dest = stat.get_first_direct_successor().get_destination();
                    self.map_pos_if_branch.put(node, dest);
                }

                let basic_preds = stat.get_all_predecessor_edges();

                if basic_preds.len() == 1 {
                    let pred_edge = basic_preds[0];

                    if pred_edge.get_type() == StatEdge::TYPE_REGULAR
                        && pred_edge.get_source().is_sequence()
                    {
                        let basichead: Statement = pred_edge.get_source().get_basichead().into();
                        self.add_edge_if_possible(basichead, stat);
                    }
                }

                let succs = stat.get_successor_edges(Statement::STATEDGE_DIRECT_ALL);
                self.add_edges(node, &succs);
                node
            }
            StatementType::CatchAll | StatementType::TryCatch => {
                let node = self.create_direct_node_type(stat, DirectNodeType::Try);
                self.add_destination(stat, node);

                let is_finally = stat.try_as_catch_all().is_some_and(|c| c.is_finally());

                let stats = stat.get_stats();

                let end_catch_index = if is_finally {
                    stats.borrow().size() - 1
                } else {
                    stats.borrow().size()
                };

                if let Some(catch_stat) = stat.try_as_catch() {
                    let resources = catch_stat.get_resources();
                    if !resources.is_empty() {
                        node.set_exprents(resources);
                    }
                }

                if is_finally {
                    let finally_node = self.create_direct_node_type(stat, DirectNodeType::Finally);
                    self.finally_nodes_stack.push(finally_node);
                }

                let try_nodes: JList<DirectNode> = JList::new();
                self.try_nodes_stack.push(try_nodes.clone());

                let try_block = self.flatten_statement(stat.get_first().expect("first"));
                node.add_successor(DirectEdge::of(node, try_block));

                let popped = self.try_nodes_stack.pop();
                ValidationHelper::validate_true(
                    try_nodes.ptr_eq(&popped),
                    "tryNodesStack is broken",
                );
                if !self.try_nodes_stack.is_empty() {
                    self.try_nodes_stack.peek().add_all(try_nodes.to_vec());
                }

                let combined_catch_node =
                    self.create_direct_node_type(stat, DirectNodeType::CombinedCatch);

                for inner_try_node in try_nodes.to_vec() {
                    inner_try_node
                        .add_successor(DirectEdge::exception(inner_try_node, combined_catch_node));
                }

                node.add_successor(DirectEdge::of(node, combined_catch_node));

                for i in 1..end_catch_index {
                    let st = stats.borrow().get(i);

                    let catch_node = self.create_direct_node_type(st, DirectNodeType::Catch);
                    let handler_node = self.flatten_statement(st);

                    combined_catch_node
                        .add_successor(DirectEdge::of(combined_catch_node, catch_node));
                    catch_node.add_successor(DirectEdge::of(catch_node, handler_node));
                }

                if is_finally {
                    let st = stats.borrow().get(end_catch_index);
                    let finally_node = self.finally_nodes_stack.pop();
                    ValidationHelper::validate_true(
                        finally_node.statement() == stat
                            && finally_node.type_() == DirectNodeType::Finally,
                        "stackFinally is broken",
                    );
                    combined_catch_node
                        .add_successor(DirectEdge::of(combined_catch_node, finally_node));

                    let finally_end_node =
                        self.create_direct_node_type(stat, DirectNodeType::FinallyEnd);
                    self.finally_nodes_stack.push(finally_end_node);
                    let finally_block_node = self.flatten_statement(st);
                    finally_node.add_successor(DirectEdge::of(finally_node, finally_block_node));

                    let finally_end_node = self.finally_nodes_stack.pop();
                    ValidationHelper::validate_true(
                        finally_end_node.statement() == stat
                            && finally_end_node.type_() == DirectNodeType::FinallyEnd,
                        "stackFinally is broken",
                    );
                }

                node
            }
            StatementType::Do => {
                if !stat.has_basic_succ_edge() && stat.has_successor(StatEdge::TYPE_REGULAR) {
                    let dest =
                        stat.get_successor_edges(StatEdge::TYPE_REGULAR)[0].get_destination();

                    if dest.get_all_predecessor_edges().len() == 1 {
                        let prededges = stat.get_predecessor_edges(StatEdge::TYPE_REGULAR);

                        if !prededges.is_empty() {
                            let prededge = prededges[0];

                            self.add_edge_if_possible(prededge.get_source(), dest);
                        }
                    }
                }

                let body = self.flatten_statement(stat.get_first().expect("first"));

                let do_stat = stat.as_do();
                let loop_type = do_stat.get_looptype();

                match loop_type {
                    DoType::Infinite => {
                        self.add_destination(stat, body);
                        self.add_destination_type(stat, body, Type::Continue);
                        body
                    }
                    DoType::While => {
                        let condition_node = self.create_direct_node_type_exprents(
                            stat,
                            DirectNodeType::Condition,
                            Some(do_stat.get_condition_exprent_list()),
                        );

                        condition_node.add_successor(DirectEdge::of(condition_node, body));

                        self.add_destination(stat, condition_node);
                        self.add_destination_type(stat, condition_node, Type::Continue);

                        let first_successor = stat.get_first_successor();
                        self.add_edge(condition_node, first_successor);
                        condition_node
                    }
                    DoType::DoWhile => {
                        let condition_node = self.create_direct_node_type_exprents(
                            stat,
                            DirectNodeType::Condition,
                            Some(do_stat.get_condition_exprent_list()),
                        );

                        condition_node.add_successor(DirectEdge::of(condition_node, body));

                        self.add_destination(stat, body);
                        self.add_destination_type(stat, condition_node, Type::Continue);

                        self.handle_loop_end(stat, body);

                        let first_successor = stat.get_first_successor();
                        self.add_edge(condition_node, first_successor);
                        body
                    }
                    DoType::For => {
                        let init_node = self.create_direct_node_type(stat, DirectNodeType::Init);
                        if do_stat.get_init_exprent().is_some() {
                            init_node.set_exprents(do_stat.get_init_exprent_list());
                        }

                        let condition_node = self.create_direct_node_type_exprents(
                            stat,
                            DirectNodeType::Condition,
                            Some(do_stat.get_condition_exprent_list()),
                        );
                        let increment_node = self.create_direct_node_type_exprents(
                            stat,
                            DirectNodeType::Increment,
                            Some(do_stat.get_inc_exprent_list()),
                        );

                        self.add_destination(stat, init_node);
                        self.add_destination_type(stat, increment_node, Type::Continue);

                        condition_node.add_successor(DirectEdge::of(condition_node, body));
                        init_node.add_successor(DirectEdge::of(init_node, condition_node));
                        increment_node
                            .add_successor(DirectEdge::of(increment_node, condition_node));

                        self.handle_loop_end(stat, body);

                        let first_successor = stat.get_first_successor();
                        self.add_edge(condition_node, first_successor);
                        init_node
                    }
                    DoType::ForEach => {
                        let inc = self.create_direct_node_type_exprents(
                            stat,
                            DirectNodeType::Increment,
                            Some(do_stat.get_inc_exprent_list()),
                        );

                        let init = self.create_direct_node_type_exprents(
                            stat,
                            DirectNodeType::ForeachVardef,
                            Some(do_stat.get_init_exprent_list()),
                        );

                        self.add_destination(stat, inc);
                        self.add_destination_type(stat, init, Type::Continue);

                        init.add_successor(DirectEdge::of(init, body));
                        inc.add_successor(DirectEdge::of(inc, init));

                        self.handle_loop_end(stat, body);

                        let first_successor = stat.get_first_successor();
                        self.add_edge(init, first_successor);
                        inc
                    }
                }
            }
            StatementType::Synchronized => {
                let tailexprlst = stat.as_synchronized().get_headexprent_list();

                let first = stat.get_first().expect("first");
                let first_node = self.create_direct_node_exprents(first, first.get_exprents());
                self.add_destination(first, first_node);
                self.add_destination(stat, first_node);

                if !tailexprlst.is_empty() {
                    let tail = self.create_direct_node_type_exprents(
                        stat,
                        DirectNodeType::Tail,
                        Some(tailexprlst),
                    );
                    first_node.add_successor(DirectEdge::of(first_node, tail));

                    let succs = first.get_all_direct_successor_edges();
                    self.add_edges(tail, &succs);
                } else {
                    let succs = first.get_all_direct_successor_edges();
                    self.add_edges(first_node, &succs);
                }

                let body = stat.get_stats().borrow().get(1);
                self.flatten_statement(body);

                self.handle_tailed_stat(stat);

                first_node
            }
            StatementType::Switch => {
                let switch_st = stat.as_switch();
                let tailexprlst = switch_st.get_headexprent_list();

                let first = stat.get_first().expect("first");
                let first_node = self.create_direct_node_exprents(first, first.get_exprents());
                let mut out_node = first_node;
                self.add_destination(first, first_node);
                self.add_destination(stat, first_node);

                if !tailexprlst.is_empty() {
                    let tail = self.create_direct_node_type_exprents(
                        stat,
                        DirectNodeType::Tail,
                        Some(tailexprlst),
                    );

                    first_node.add_successor(DirectEdge::of(first_node, tail));

                    out_node = tail;
                }

                self.handle_tailed_stat(stat);

                let mut i = 1;
                while i < stat.get_stats().borrow().size() {
                    let st = stat.get_stats().borrow().get(i);
                    self.flatten_statement(st);
                    i += 1;
                }

                let case_edges = switch_st.get_case_edges();
                let case_values = switch_st.get_case_values();
                let case_statements = switch_st.get_case_statements();

                if case_edges.len() != case_values.len()
                    && (case_edges.len() + 1 != case_values.len() || {
                        let last = case_values.get(case_values.len() - 1);
                        last.len() != 1 || last.get(0).is_some()
                    })
                {
                    jthrow!(IllegalStateException; "Case edges and case values do not match");
                }

                for i in 0..case_edges.len() {
                    let values = case_values.get(i);
                    let this_case_edges = case_edges.get(i).to_vec();
                    let this_case_statement = case_statements.get(i);

                    let mut final_vals: Vec<Exprent> = Vec::new();
                    for value in values.to_vec().into_iter().flatten() {
                        final_vals.push(value);
                    }

                    let case_node = self.create_direct_node_type_exprents(
                        this_case_statement,
                        DirectNodeType::Case,
                        Some(JList::from(final_vals)),
                    );

                    out_node.add_successor(DirectEdge::of(out_node, case_node));

                    self.add_edges(case_node, &this_case_edges);
                }

                if case_edges.len() < case_values.len() {
                    let default_edge = switch_st.get_default_edge().expect("defaultEdge");
                    self.add_edge(out_node, default_edge);
                }

                first_node
            }
            StatementType::If => {
                let if_stat = stat.as_if();

                let tailexprlst = if_stat.get_headexprent_list();

                let first = stat.get_first().expect("first");
                let first_node = self.create_direct_node_exprents(first, first.get_exprents());
                let mut out_node = first_node;
                self.add_destination(first, first_node);
                self.add_destination(stat, first_node);

                if !tailexprlst.is_empty() {
                    let tail = self.create_direct_node_type_exprents(
                        stat,
                        DirectNodeType::Tail,
                        Some(tailexprlst),
                    );

                    first_node.add_successor(DirectEdge::of(first_node, tail));

                    out_node = tail;
                }

                let if_dest = if_stat.get_if_edge().expect("ifEdge").get_destination();
                self.map_pos_if_branch.put(out_node, if_dest);

                self.handle_tailed_stat(stat);

                let mut i = 1;
                while i < stat.get_stats().borrow().size() {
                    let st = stat.get_stats().borrow().get(i);
                    self.flatten_statement(st);
                    i += 1;
                }

                let succs = first.get_all_direct_successor_edges();
                self.add_edges(out_node, &succs);

                if if_stat.iftype() == IfStatement::IFTYPE_IF && stat.has_any_direct_successor() {
                    let first_direct_successor = stat.get_first_direct_successor();
                    self.add_edge(out_node, first_direct_successor);
                }

                first_node
            }
            StatementType::Sequence | StatementType::Root => {
                let first_block = self.flatten_statement(stat.get_first().expect("first"));

                let statsize = stat.get_stats().borrow().size();
                for i in 1..statsize {
                    let st = stat.get_stats().borrow().get(i);
                    self.flatten_statement(st);
                }

                self.add_destination(stat, first_block);
                first_block
            }
            _ => jthrow!("Unexpected statement type"),
        }
    }

    fn handle_tailed_stat(&mut self, stat: Statement) {
        let basic_preds = stat.get_all_predecessor_edges();

        if basic_preds.len() == 1 {
            let pred_edge = basic_preds[0];

            if pred_edge.get_type() == StatEdge::TYPE_REGULAR
                && pred_edge.get_source().is_sequence()
            {
                let basichead: Statement = pred_edge.get_source().get_basichead().into();
                self.add_edge_if_possible(basichead, stat);
            }
        }
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
        let edges = self.indirect_edges.clone();
        for edge in edges {
            let source = edge.source;
            let Some(mut dest) = self.get_destination(&edge) else {
                jthrow!(IllegalStateException;
                    "Could not find destination nodes for stat id {} from source {}", edge.stat, source);
            };

            if edge.type_ == Type::FinallyExit {
                match source.try_finally() {
                    Some(try_finally) if try_finally.type_() == DirectNodeType::FinallyEnd => {
                        dest = try_finally;
                    }
                    _ => continue,
                }
            }

            let diedge = if edge.type_ == Type::Exception {
                DirectEdge::exception(source, dest)
            } else {
                DirectEdge::of(source, dest)
            };

            source.add_successor(diedge);

            if self.map_pos_if_branch.contains_key(&source)
                && Some(edge.stat) != self.map_pos_if_branch.get(&source).copied()
            {
                let (source_id, dest_id) = (source.id(), dest.id());
                self.graph()
                    .with_mut(|d| d.map_neg_if_branch.put(source_id, dest_id));
            }
        }
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
