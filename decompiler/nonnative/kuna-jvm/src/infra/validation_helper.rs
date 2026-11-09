//! Port of `org/jetbrains/java/decompiler/modules/decompiler/ValidationHelper.java`.
//!
//! The graph validators run only when `VALIDATE_DECOMPILED_CODE=true` is in the
//! environment (Java: the system property of the same name); `assert_true` and
//! `not_null` always check. The `DotExporter` dumps on failure are not ported.

use std::collections::VecDeque;
use std::fmt::Display;
use std::sync::OnceLock;

use crate::catch_statement::CatchStatement;
use crate::direct_edge_type::DirectEdgeType;
use crate::direct_graph::DirectGraph;
use crate::direct_node::DirectNode;
use crate::direct_node_type::DirectNodeType;
use crate::exit_exprent::{ExitExprent, Type as ExitType};
use crate::exprent::{Exprent, ExprentType};
use crate::flatten_statements_helper::FlattenStatementsHelper;
use crate::if_statement::IfStatement;
use crate::jthrow;
use crate::list_stack::ListStack;
use crate::merge_helper::MergeHelper;
use crate::root_statement::RootStatement;
use crate::rt::{try_catch, JHashMap, JHashSet, JavaEq, JavaHash};
use crate::stat_edge::StatEdge;
use crate::statement::{Statement, StatementType};
use crate::var_exprent::VarExprent;
use crate::var_type::VarType;
use crate::var_version_node::VarVersionNode;
use crate::var_version_pair::VarVersionPair;
use crate::var_versions_graph::VarVersionsGraph;
use crate::vb_style_collection::VBStyleCollection;

pub struct ValidationHelper;

/// Java `String.valueOf(o)` for a nullable reference.
fn jstr<T: Display>(o: Option<T>) -> String {
    o.map_or_else(|| "null".to_string(), |v| v.to_string())
}

/// Java `AbstractCollection.toString()`.
fn jlist<T: Display>(items: impl IntoIterator<Item = T>) -> String {
    let parts: Vec<String> = items.into_iter().map(|i| i.to_string()).collect();
    format!("[{}]", parts.join(", "))
}

impl ValidationHelper {
    #[allow(non_snake_case)]
    pub fn VALIDATE() -> bool {
        static VALIDATE: OnceLock<bool> = OnceLock::new();
        *VALIDATE
            .get_or_init(|| std::env::var("VALIDATE_DECOMPILED_CODE").is_ok_and(|v| v == "true"))
    }

    pub fn validate_statement(statement: RootStatement) {
        if !Self::VALIDATE() {
            return;
        }

        let statements: VBStyleCollection<Statement, i32> = VBStyleCollection::new();

        let mut stack: VecDeque<Statement> = VecDeque::new();
        stack.push_front(statement.get_dummy_exit().0);
        stack.push_front(statement.0);

        while let Some(stat) = stack.pop_front() {
            statements.put_with_key(stat, stat.with(|d| d.id));
            stack.extend(stat.get_stats().iter());
        }

        for stat in statements.iter() {
            for edge in stat.get_all_successor_edges() {
                Self::validate_edge_context(&statements, stat, edge);
            }

            for edge in stat.get_all_predecessor_edges() {
                Self::validate_edge_context(&statements, stat, edge);
            }

            let label_edges = stat.get_label_edges().borrow().to_vec();
            for edge in label_edges {
                Self::validate_edge_context(&statements, stat, edge);
            }

            if let Some(exprents) = stat.get_exprents() {
                for exprent in exprents.iter() {
                    Self::validate_exprent(exprent);
                }
            }

            let first = stat.get_first();
            if !first.is_some_and(|f| statements.contains(&f))
                && !stat.is_dummy_exit()
                && !stat.is_basic_block()
            {
                jthrow!(IllegalStateException; "Non-existing first statement: [{}] {}", stat, jstr(first));
            }

            if !first.is_some_and(|f| stat.get_stats().contains(&f))
                && !stat.is_dummy_exit()
                && !stat.is_basic_block()
            {
                jthrow!(IllegalStateException; "Stat doesn't contain first statement: [{}] {}", stat, jstr(first));
            }

            for stat_stat in stat.get_stats().iter() {
                if stat_stat.get_parent() != Some(stat) {
                    jthrow!(IllegalStateException;
                        "Statement parent is not set correctly [{}]: Expected {} but was {}",
                        stat_stat, stat, jstr(stat_stat.get_parent()));
                }
            }

            Self::validate_single_statement(stat);
        }

        let direct_graph = match try_catch(|| {
            let mut flatten = FlattenStatementsHelper::new();
            flatten.build_direct_graph(statement)
        }) {
            Ok(graph) => graph,
            Err(_) => jthrow!(IllegalStateException; "Failed to build direct graph"),
        };

        Self::validate_d_graph(direct_graph, statement);
    }

    fn validate_edge_context(
        statements: &VBStyleCollection<Statement, i32>,
        stat: Statement,
        edge: StatEdge,
    ) {
        if !statements.contains(&edge.get_source()) {
            jthrow!(IllegalStateException; "Edge pointing from non-existing statement: [{}] {}", stat, edge);
        }

        if !statements.contains(&edge.get_destination()) {
            jthrow!(IllegalStateException; "Edge pointing to non-existing statement: [{}] {}", stat, edge);
        }

        if let Some(closure) = edge.with(|d| d.closure) {
            if !statements.contains(&closure) {
                jthrow!(IllegalStateException; "Edge with non-existing closure: [{}] {}", stat, edge);
            }
        }

        Self::validate_edge(edge);
    }

    pub fn validate_edge(edge: StatEdge) {
        if !Self::VALIDATE() {
            return;
        }

        if !Self::is_successor(edge.get_source(), edge) {
            jthrow!(IllegalStateException;
                "Edge pointing from statement but it isn't a successor: {} {}", edge.get_source(), edge);
        }

        if !edge
            .get_destination()
            .get_all_predecessor_edges()
            .contains(&edge)
        {
            jthrow!(IllegalStateException; "Edge pointing to statement but it isn't a predecessor: {}", edge);
        }

        let (closure, phantom_continue) = edge.with(|d| (d.closure, d.phantom_continue));
        match edge.get_type() {
            StatEdge::TYPE_BREAK => {
                let Some(closure) = closure else {
                    jthrow!(IllegalStateException; "Break edge with break type, but no closure: {}", edge);
                };

                if closure.with(|d| d.type_) == StatementType::BasicBlock {
                    jthrow!(IllegalStateException; "Break edge closure to basic block: {}", edge);
                }

                if edge.get_source() == closure && !phantom_continue {
                    jthrow!(IllegalStateException; "Break edge with closure pointing to itself: {}", edge);
                }

                if edge.get_destination() == closure {
                    jthrow!(IllegalStateException; "Break edge with closure pointing to itself: {}", edge);
                }

                if edge.get_source() == edge.get_destination() {
                    jthrow!(IllegalStateException; "Break edge pointing to itself: {}", edge);
                }

                if !edge.get_destination().is_dummy_exit()
                    && !MergeHelper::is_direct_path(closure, edge.get_destination())
                {
                    jthrow!(IllegalStateException; "Break edge with closure with invalid direct path: {}", edge);
                }
            }
            StatEdge::TYPE_CONTINUE => {
                let Some(closure) = closure else {
                    jthrow!(IllegalStateException; "Continue edge with continue type, but no closure: {}", edge);
                };

                if closure != edge.get_destination() {
                    jthrow!(IllegalStateException;
                        "Continue edge with closure pointing to different destination: {}", edge);
                }

                if !edge.get_destination().is_do() {
                    jthrow!(IllegalStateException; "Continue edge where closure isn't pointing to a do: {}", edge);
                }
            }
            StatEdge::TYPE_FINALLYEXIT => {
                if closure.is_none() {
                    jthrow!(IllegalStateException;
                        "Finally exit edge with finally exit type, but no closure: {}", edge);
                }

                if !edge.get_destination().is_dummy_exit() {
                    jthrow!(IllegalStateException;
                        "Finally exit edge where closure isn't pointing to the dummy exit: {}", edge);
                }
            }
            _ => {}
        }
    }

    pub fn validate_single_statement(stat: Statement) {
        if !Self::VALIDATE() {
            return;
        }

        let type_ = stat.with(|d| d.type_);
        if type_ == StatementType::BasicBlock && stat.get_all_successor_edges().is_empty() {
            jthrow!(IllegalStateException; "Basic block {} has no edges", stat);
        }

        match type_ {
            StatementType::If => Self::validate_if_statement(stat.as_if()),
            StatementType::TryCatch => Self::validate_trycatch_statement(stat.as_catch()),
            _ => {}
        }
    }

    pub fn validate_trycatch_statement(catch_stat: CatchStatement) {
        if catch_stat.get_stats().len() == 1 && catch_stat.get_resources().is_empty() {
            jthrow!(IllegalStateException; "Try statement with single statement: {}", catch_stat.0);
        }
    }

    pub fn validate_if_statement(if_stat: IfStatement) {
        if !Self::VALIDATE() {
            return;
        }

        let stats = if_stat.get_stats();
        let stat = if_stat.0;

        match if_stat.get_first() {
            None => {
                jthrow!(IllegalStateException; "If statement without a first statement: {}", stat)
            }
            Some(first) if !stats.contains(&first) => {
                jthrow!(IllegalStateException; "If statement does not contain own first statement: {}", stat)
            }
            Some(_) => {}
        }

        let Some(if_edge) = if_stat.get_if_edge() else {
            jthrow!(IllegalStateException; "If statement without an if edge: {}", stat);
        };

        if let Some(ifstat) = if_stat.get_ifstat() {
            if if_edge.get_destination() != ifstat {
                jthrow!(IllegalStateException;
                    "If statement if edge destination is not ifStat: {} (destination is: {} but ifStat is: {})",
                    stat, if_edge.get_destination(), ifstat);
            }

            if !stats.contains(&ifstat) {
                jthrow!(IllegalStateException; "If statement does not contain own ifStat: {}", stat);
            }
        }

        let iftype = if_stat.d(|d| d.iftype);
        if iftype == IfStatement::IFTYPE_IF {
            if if_stat.get_else_edge().is_some() {
                jthrow!(IllegalStateException; "If statement with unexpected else edge: {}", stat);
            }
            let successors = if_stat.get_successor_edges(Statement::STATEDGE_DIRECT_ALL);
            if successors.is_empty() {
                jthrow!(IllegalStateException; "If statement with no else edge and no successors: {}", stat);
            } else if successors.len() > 1 {
                jthrow!(IllegalStateException;
                    "If statement with more than one successor: {} (successors: {})",
                    stat, jlist(if_stat.get_successor_edges(Statement::STATEDGE_DIRECT_ALL)));
            }
        } else if iftype == IfStatement::IFTYPE_IFELSE {
            let Some(else_edge) = if_stat.get_else_edge() else {
                jthrow!(IllegalStateException; "IfElse statement without else edge: {}", stat);
            };

            if if_stat.get_ifstat().is_none() {
                jthrow!(IllegalStateException; "IfElse statement without ifStat: {}", stat);
            }

            let Some(elsestat) = if_stat.get_elsestat() else {
                jthrow!(IllegalStateException; "IfElse statement without elseStat: {}", stat);
            };

            if else_edge.get_destination() != elsestat {
                jthrow!(IllegalStateException;
                    "IfElse statement else edge destination is not elseStat: {}", stat);
            }

            if !stats.contains(&elsestat) {
                jthrow!(IllegalStateException; "IfElse statement does not contain own elseStat: {}", stat);
            }
        } else {
            jthrow!(IllegalStateException; "Unknown if type: {}", stat);
        }

        if Some(if_edge.get_source()) != if_stat.get_first() {
            jthrow!(IllegalStateException;
                "If statement if edge source is not first statement: [{}] {} (source is: {} but first is: {})",
                if_edge, stat, if_edge.get_source(), jstr(if_stat.get_first()));
        }

        if let Some(else_edge) = if_stat.get_else_edge() {
            if Some(else_edge.get_source()) != if_stat.get_first() {
                jthrow!(IllegalStateException;
                    "IfElse statement else edge source is not first statement: {} (elseEdge: {})",
                    stat, else_edge);
            }
        }

        if stats.len() > 3 {
            jthrow!(IllegalStateException; "If statement with more than 3 sub statements: {}", stat);
        }

        for sub in stats.iter() {
            if Some(sub) != if_stat.get_first()
                && Some(sub) != if_stat.get_ifstat()
                && Some(sub) != if_stat.get_elsestat()
            {
                jthrow!(IllegalStateException;
                    "If statement contains unknown sub statement: {} (subStatement: {})", stat, sub);
            }
        }

        let first = if_stat.get_first().unwrap();
        for edge in first.get_successor_edges(Statement::STATEDGE_DIRECT_ALL) {
            if if_stat.get_if_edge() != Some(edge) && if_stat.get_else_edge() != Some(edge) {
                jthrow!(IllegalStateException;
                    "If statement first contains unknown successor edge: {} (edge: {})", stat, edge);
            }
        }
    }

    pub fn validate_d_graph(graph: DirectGraph, _root: RootStatement) {
        if !Self::VALIDATE() {
            return;
        }

        let (nodes, first) = graph.with(|d| (d.nodes.clone(), d.first));
        let first = first.unwrap();
        let mut inaccessible_nodes: JHashSet<DirectNode> =
            JHashSet::from_collection(nodes.to_vec());

        let mut stack: ListStack<DirectNode> = ListStack::new();
        stack.push(first);
        inaccessible_nodes.remove(&first);

        while !stack.is_empty() {
            let node = stack.pop();

            for pred in node.get_predecessors(DirectEdgeType::Regular) {
                if !pred
                    .get_source()
                    .get_successors(DirectEdgeType::Regular)
                    .contains(&pred)
                {
                    jthrow!(IllegalStateException;
                        "Predecessor {} does not have {} as a successor", pred, node);
                }
            }

            for succ in node.get_successors(DirectEdgeType::Regular) {
                if !succ
                    .get_destination()
                    .get_predecessors(DirectEdgeType::Regular)
                    .contains(&succ)
                {
                    jthrow!(IllegalStateException;
                        "Successor {} does not have {} as a predecessor", succ, node);
                }

                if inaccessible_nodes.remove(&succ.get_destination()) {
                    stack.push(succ.get_destination());
                }
            }
        }

        if !inaccessible_nodes.is_empty() {
            for inaccessible_node in inaccessible_nodes.iter() {
                if inaccessible_node.with(|d| d.type_) != DirectNodeType::FinallyEnd {
                    jthrow!(IllegalStateException;
                        "Inaccessible direct graph nodes: {}", jlist(inaccessible_nodes.iter()));
                }
            }
        }

        let mut all_exprents: JHashMap<ID<Exprent>, DirectNode> = JHashMap::new();

        for node in nodes.iter() {
            let exprents = node.with(|d| d.exprents.clone()).unwrap();
            for exprent in exprents.iter() {
                for sub_exprent in exprent.get_all_exprents_rec_self(true, true) {
                    let key = ID::new(sub_exprent);
                    if all_exprents.contains_key(&key) && !sub_exprent.is_var() {
                        let other = all_exprents.get(&key).copied().unwrap();
                        jthrow!(IllegalStateException;
                            "Duplicated exprent: {} (Sub exprent of: {}) in dgraph. Appears in both node {} and node {}!",
                            sub_exprent, exprent, node.with(|d| d.id.clone()), other.with(|d| d.id.clone()));
                    } else {
                        all_exprents.put(key, node);
                    }
                }
            }
        }
    }

    pub fn validate_vars(
        dgraph: DirectGraph,
        _root: RootStatement,
        predicate: impl Fn(VarExprent) -> bool,
        message: &str,
    ) {
        if !Self::VALIDATE() {
            return;
        }

        let nodes = dgraph.with(|d| d.nodes.clone());
        for node in nodes.iter() {
            if let Some(exprents) = node.with(|d| d.exprents.clone()) {
                for exprent in exprents.iter() {
                    for sub in exprent.get_all_exprents_rec_self(true, true) {
                        if let Some(var) = sub.try_as_var() {
                            if !predicate(var) {
                                jthrow!(IllegalStateException; "{}: {}_{} {}",
                                    message, var.get_index(), var.get_version(), var.get_var_type());
                            }
                        }
                    }
                }
            }
        }
    }

    /// Java: notNull(Object)
    pub fn not_null<T>(o: Option<&T>) {
        if o.is_none() {
            jthrow!(NullPointerException; "Null not expected here");
        }
    }

    pub fn validate_exit_exprent(exit: ExitExprent) {
        if !Self::VALIDATE() {
            return;
        }

        if exit.get_exit_type() == ExitType::Return {
            if exit.get_ret_type().equals(&VarType::VARTYPE_VOID()) {
                if exit.get_value().is_some() {
                    jthrow!(IllegalStateException; "Void return with value: {}", exit.0);
                }
            } else if exit.get_value().is_none() {
                jthrow!(IllegalStateException; "Non-void return without value: {}", exit.0);
            }
        }

        for sub_exprents in exit.get_all_exprents() {
            Self::validate_exprent(sub_exprents);
        }
    }

    pub fn validate_exprent(exprent: Exprent) {
        if !Self::VALIDATE() {
            return;
        }

        match exprent.get_type() {
            ExprentType::Exit => Self::validate_exit_exprent(exprent.as_exit()),
            _ => {
                for sub_exprents in exprent.get_all_exprents() {
                    Self::validate_exprent(sub_exprents);
                }
            }
        }
    }

    pub fn successors_exist(stat: Statement) {
        if !Self::VALIDATE() {
            return;
        }

        if stat.get_all_successor_edges().is_empty() {
            jthrow!(IllegalStateException; "Statement has no successors: {}", stat);
        }
    }

    pub fn one_successor(stat: Statement) {
        if !Self::VALIDATE() {
            return;
        }

        if stat.get_all_successor_edges().len() != 1 {
            jthrow!(IllegalStateException; "Statement has more than one successor: [{}] {}",
                stat, jlist(stat.get_all_successor_edges()));
        }
    }

    fn is_successor(source: Statement, edge: StatEdge) -> bool {
        if source.get_all_successor_edges().contains(&edge) {
            return true;
        }

        if let Some(ifstat) = source.get_parent().and_then(|p| p.try_as_if()) {
            if ifstat.get_first() == Some(source)
                && (Some(edge) == ifstat.get_if_edge() || Some(edge) == ifstat.get_else_edge())
            {
                return true;
            }
        }

        false
    }

    pub fn validate_true(condition: bool, message: &str) {
        if Self::VALIDATE() && !condition {
            jthrow!(IllegalStateException; "Validation failed: {}", message);
        }
    }

    pub fn assert_true(condition: bool, message: &str) {
        if !condition {
            jthrow!(IllegalStateException; "Assertion failed: {}", message);
        }
    }

    pub fn validate_var_versions_graph(
        graph: VarVersionsGraph,
        _statement: RootStatement,
        _var_assignment_map: &JHashMap<VarVersionPair, VarVersionPair>,
    ) {
        if !Self::VALIDATE() {
            return;
        }

        let nodes = graph.with(|d| d.nodes.clone());
        let mut roots: JHashSet<VarVersionNode> = JHashSet::new();

        for node in nodes.iter() {
            if node.with(|d| d.predecessors.is_empty()) {
                roots.add(node);
            }
        }

        let reached = VarVersionsGraph::root_reachability(&roots);

        if nodes.len() != reached.len() {
            jthrow!(IllegalStateException; "Highly cyclic varversions graph!");
        }
    }
}

/// `ValidationHelper.ID`: a key compared and hashed by the identity of its object.
#[derive(Clone, Copy)]
struct ID<T> {
    obj: T,
}

impl<T> ID<T> {
    fn new(obj: T) -> ID<T> {
        ID { obj }
    }
}

impl JavaHash for ID<Exprent> {
    fn java_hash(&self) -> i32 {
        self.obj.index() as i32
    }
}

impl JavaEq for ID<Exprent> {
    fn java_equals(&self, other: &Self) -> bool {
        self.obj == other.obj
    }
}
