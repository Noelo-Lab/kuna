//! Port of `org/jetbrains/java/decompiler/modules/decompiler/decompose/DomHelper.java`.
//!
//! `GraphParser` is not ported (plugin API), so `createStatement` is inherent.

use std::collections::VecDeque;

use crate::catch_all_statement::CatchAllStatement;
use crate::catch_statement::CatchStatement;
use crate::control_flow_graph::ControlFlowGraph;
use crate::do_statement::DoStatement;
use crate::dom_tracer::DomTracer;
use crate::if_statement::IfStatement;
use crate::root_statement::RootStatement;
use crate::rt::{JHashMap, JHashSet, JLinkedHashMap, JLinkedHashSet, Shared};
use crate::sequence_statement::SequenceStatement;
use crate::stat_edge::StatEdge;
use crate::statement::{EdgeDirection, Statement};
use crate::struct_method::StructMethod;
use crate::switch_statement::SwitchStatement;
use crate::synchronized_statement::SynchronizedStatement;
use crate::unported;
use crate::validation_helper::ValidationHelper;
use crate::vb_style_collection::VBStyleCollection;

/// `HashMap<Integer, Set<Integer>> mapExtPost`, whose runtime class is a
/// `LinkedHashMap` (the first one) or a `HashMap` (every later one); the
/// difference reaches iteration order. Passed around as a shared reference.
pub enum ExtPostMap {
    Linked(JLinkedHashMap<i32, JLinkedHashSet<i32>>),
    Hash(JHashMap<i32, JLinkedHashSet<i32>>),
}

impl ExtPostMap {
    /// `new LinkedHashMap<>()`.
    pub fn new_linked() -> Shared<ExtPostMap> {
        Shared::new(ExtPostMap::Linked(JLinkedHashMap::new()))
    }

    /// `new HashMap<>()`.
    pub fn new_hash() -> Shared<ExtPostMap> {
        Shared::new(ExtPostMap::Hash(JHashMap::new()))
    }

    pub fn is_empty(&self) -> bool {
        match self {
            ExtPostMap::Linked(m) => m.is_empty(),
            ExtPostMap::Hash(m) => m.is_empty(),
        }
    }
}

pub struct DomHelper;

impl DomHelper {
    pub fn create_statement(&self, graph: ControlFlowGraph, mt: StructMethod) -> RootStatement {
        DomHelper::parse_graph(graph, mt, 0)
    }

    #[allow(dead_code)]
    fn graph_to_statement(_graph: ControlFlowGraph, _mt: StructMethod) -> RootStatement {
        unported!("DomHelper.graphToStatement")
    }

    pub fn calc_post_dominators(_general: Statement) -> VBStyleCollection<Vec<i32>, i32> {
        unported!("DomHelper.calcPostDominators")
    }

    pub fn parse_graph(
        _graph: ControlFlowGraph,
        _mt: StructMethod,
        _iteration: i32,
    ) -> RootStatement {
        unported!("DomHelper.parseGraph")
    }

    pub fn remove_synchronized_handler(stat: Statement) -> bool {
        let mut res = false;

        let stats = stat.get_stats().borrow().to_vec();
        for st in stats {
            res |= DomHelper::remove_synchronized_handler(st);
        }

        if let Some(sync) = stat.try_as_synchronized() {
            sync.remove_exc();
            res = true;
        }

        res
    }

    pub fn build_synchronized(stat: Statement) -> bool {
        let mut res = false;
        let stats = stat.get_stats().borrow().to_vec();
        for st in stats {
            res |= DomHelper::build_synchronized(st);
        }

        if stat.is_sequence() {
            loop {
                let mut found = false;

                let lst = stat.get_stats();
                let mut i = 0;
                while i + 1 < lst.borrow().len() {
                    let current = lst.borrow().get(i);

                    if current.is_monitor_enter() {
                        let mut next = lst.borrow().get(i + 1);
                        let next_direct = next;

                        while next.is_sequence() {
                            next = next.get_first().expect("first");
                        }

                        if let Some(ca) = next.try_as_catch_all() {
                            let ca_first = ca.get_first().expect("first");
                            let mut head_ok = ca_first.contains_monitor_exit_or_athrow();

                            if !head_ok {
                                head_ok = DomHelper::has_no_exits(ca_first);
                            }

                            if !head_ok {
                                head_ok = ca.is_finally();
                            }

                            if head_ok && ca.get_handler().contains_monitor_exit() {
                                ca_first.mark_monitorexit_dead();
                                ca.get_handler().mark_monitorexit_dead();

                                for edge in
                                    ca.get_successor_edge_view(StatEdge::TYPE_REGULAR).iter()
                                {
                                    edge.get_destination().mark_monitorexit_dead();
                                }

                                let ca_parent = ca.get_parent().expect("parent");
                                for edge in ca_parent
                                    .get_successor_edge_view(StatEdge::TYPE_REGULAR)
                                    .iter()
                                {
                                    edge.get_destination().mark_monitorexit_dead();
                                }

                                current.remove_successor(
                                    current.get_successor_edges(Statement::STATEDGE_DIRECT_ALL)[0],
                                );

                                for edge in
                                    current.get_predecessor_edges(Statement::STATEDGE_DIRECT_ALL)
                                {
                                    current.remove_predecessor(edge);
                                    edge.get_source().change_edge_node(
                                        EdgeDirection::Forward,
                                        edge,
                                        next_direct,
                                    );
                                    next_direct.add_predecessor(edge);
                                }

                                stat.get_stats().borrow_mut().remove_with_key(&current.id());
                                let new_first = stat.get_stats().borrow().get(0);
                                stat.set_first(new_first);

                                let sync =
                                    SynchronizedStatement::new(current, ca_first, ca.get_handler());
                                sync.set_all_parent();

                                let label_edges = JHashSet::from_collection(
                                    ca.get_label_edges().borrow().to_vec(),
                                );
                                for edge in label_edges.to_vec() {
                                    sync.add_labeled_edge(edge);
                                }

                                current.add_successor(StatEdge::new(
                                    StatEdge::TYPE_REGULAR,
                                    current,
                                    ca_first,
                                ));

                                ca.get_parent()
                                    .expect("parent")
                                    .replace_statement(ca.0, sync.0);
                                found = true;
                                res = true;
                                break;
                            }
                        }
                    }
                    i += 1;
                }

                if !found {
                    break;
                }
            }
        }

        res
    }

    fn has_no_exits(head: Statement) -> bool {
        let mut stack: VecDeque<Statement> = VecDeque::new();
        stack.push_back(head);

        while let Some(stat) = stack.pop_front() {
            let sucs = stat.get_successor_edges(Statement::STATEDGE_DIRECT_ALL);
            for suc in sucs {
                if !head.contains_statement(suc.get_destination()) {
                    return false;
                }
            }

            stack.extend(stat.get_stats().borrow().to_vec());
        }

        true
    }

    #[allow(dead_code)]
    fn process_statement(
        _general: Statement,
        _root: RootStatement,
        _map_ext_post: Shared<ExtPostMap>,
        _tracer: &mut DomTracer,
    ) -> bool {
        unported!("DomHelper.processStatement")
    }

    #[allow(dead_code)]
    fn find_general_statement(
        _stat: Statement,
        _forceall: bool,
        _map_ext_post: Shared<ExtPostMap>,
    ) -> Option<Statement> {
        unported!("DomHelper.findGeneralStatement")
    }

    /// `st`: the statement whose postdominance is checked; `s`: the candidate.
    #[allow(dead_code)]
    fn does_finally_exit_match(st: Statement, s: Statement) -> bool {
        let finally_exits = s.get_successor_edge_view(StatEdge::TYPE_FINALLYEXIT);
        ValidationHelper::validate_true(
            finally_exits.len() == 1,
            "Must have just one exit from a finally end block",
        );
        for edge in finally_exits.iter() {
            let finally_end = edge.get_source();

            for pred in finally_end.get_all_predecessor_edges() {
                if pred.get_source() == st {
                    return true;
                }
            }
        }

        false
    }

    #[allow(dead_code)]
    fn check_synchronized_completeness(set_nodes: &JLinkedHashSet<Statement>) -> bool {
        for &stat in set_nodes.iter() {
            if stat.is_monitor_enter() {
                let lst_succs = stat.get_successor_edges(Statement::STATEDGE_DIRECT_ALL);
                if lst_succs.len() != 1 || lst_succs[0].get_type() != StatEdge::TYPE_REGULAR {
                    return false;
                }

                if !set_nodes.contains(&lst_succs[0].get_destination()) {
                    return false;
                }
            }
        }

        true
    }

    #[allow(dead_code)]
    fn find_simple_statements(
        _stat: Statement,
        _map_ext_post: Shared<ExtPostMap>,
        _tracer: &mut DomTracer,
    ) -> bool {
        unported!("DomHelper.findSimpleStatements")
    }

    #[allow(dead_code)]
    fn detect_statement(head: Statement) -> Option<Statement> {
        if let Some(res) = DoStatement::is_head(head) {
            return Some(res);
        }

        if let Some(res) = SwitchStatement::is_head(head) {
            return Some(res);
        }

        if let Some(res) = IfStatement::is_head(head) {
            return Some(res);
        }

        if let Some(res) = SequenceStatement::is_head2_block(head) {
            return Some(res);
        }

        if let Some(res) = CatchStatement::is_head(head) {
            return Some(res);
        }

        if let Some(res) = CatchAllStatement::is_head(head) {
            return Some(res);
        }

        None
    }
}
