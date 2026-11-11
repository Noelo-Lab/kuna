//! Port of `org/jetbrains/java/decompiler/modules/decompiler/deobfuscator/IrreducibleCFGDeobfuscator.java`.
//!
//! kuna fix `irreducible_with_exception_edges` reuses the candidate choice and
//! the split (`kuna_fix_irreducible_with_exception_edges.rs`).

use crate::cancelation_manager::CancelationManager;
use crate::rt::{JHashMap, JHashSet};
use crate::stat_edge::StatEdge;
use crate::statement::{EdgeDirection, Statement};

pub struct IrreducibleCFGDeobfuscator;

struct Node {
    id: i32,
    preds: JHashSet<usize>,
    succs: JHashSet<usize>,
}

impl IrreducibleCFGDeobfuscator {
    pub fn is_statement_irreducible(statement: Statement) -> bool {
        let mut nodes: Vec<Node> = Vec::new();
        let mut map_nodes: JHashMap<i32, usize> = JHashMap::new();

        let stats: Vec<Statement> = statement.get_stats().borrow().iter().copied().collect();

        for &stat in &stats {
            if !stat
                .get_successor_edges(StatEdge::TYPE_EXCEPTION)
                .is_empty()
            {
                return false;
            }

            let id = stat.with(|d| d.id);
            nodes.push(Node {
                id,
                preds: JHashSet::new(),
                succs: JHashSet::new(),
            });
            map_nodes.put(id, nodes.len() - 1);
        }

        for &stat in &stats {
            let node = map_nodes
                .get_cloned(&stat.with(|d| d.id))
                .expect("NullPointerException");

            for succ in stat.get_neighbours(StatEdge::TYPE_REGULAR, EdgeDirection::Forward) {
                let node_succ = map_nodes
                    .get_cloned(&succ.with(|d| d.id))
                    .expect("NullPointerException");

                nodes[node].succs.add(node_succ);
                nodes[node_succ].preds.add(node);
            }
        }

        loop {
            CancelationManager::check_canceled();
            let mut ttype = 0;
            let mut node: Option<usize> = None;

            for nd in map_nodes.value_vec() {
                if nodes[nd].succs.contains(&nd) {
                    ttype = 1;
                } else if nodes[nd].preds.len() == 1 {
                    ttype = 2;
                }

                if ttype != 0 {
                    node = Some(nd);
                    break;
                }
            }

            match node {
                Some(node) => {
                    if ttype == 1 {
                        nodes[node].succs.remove(&node);
                        nodes[node].preds.remove(&node);
                    } else {
                        let pred = *nodes[node]
                            .preds
                            .iter()
                            .next()
                            .expect("NoSuchElementException");

                        let node_succs = nodes[node].succs.to_vec();
                        nodes[pred].succs.add_all(node_succs.clone());
                        nodes[pred].succs.remove(&node);

                        for succ in node_succs {
                            nodes[succ].preds.remove(&node);
                            nodes[succ].preds.add(pred);
                        }

                        let id = nodes[node].id;
                        map_nodes.remove(&id);
                    }
                }
                None => return map_nodes.len() > 1,
            }
        }
    }

    fn get_candidate_for_splitting(statement: Statement) -> Option<Statement> {
        Self::get_candidate_for_splitting_among(statement, |_| true)
    }

    /// Java's candidate choice over the statements `accept` admits (fix
    /// `irreducible_with_exception_edges` admits only unprotected ones).
    pub(crate) fn get_candidate_for_splitting_among(
        statement: Statement,
        accept: impl Fn(Statement) -> bool,
    ) -> Option<Statement> {
        let mut candidate_for_splitting: Option<Statement> = None;
        let mut size_candidate_for_splitting = i32::MAX;
        let mut succs_candidate_for_splitting = i32::MAX;

        let stats: Vec<Statement> = statement.get_stats().borrow().iter().copied().collect();
        for stat in stats {
            if !accept(stat) {
                continue;
            }
            let set_preds =
                stat.get_neighbours_set(StatEdge::TYPE_REGULAR, EdgeDirection::Backward);

            if set_preds.len() > 1 {
                let succ_count = stat
                    .get_neighbours_set(StatEdge::TYPE_REGULAR, EdgeDirection::Forward)
                    .len() as i32;
                if succ_count <= succs_candidate_for_splitting {
                    let size =
                        Self::get_statement_size(stat).wrapping_mul(set_preds.len() as i32 - 1);

                    if succ_count < succs_candidate_for_splitting
                        || size < size_candidate_for_splitting
                    {
                        candidate_for_splitting = Some(stat);
                        size_candidate_for_splitting = size;
                        succs_candidate_for_splitting = succ_count;
                    }
                }
            }
        }

        candidate_for_splitting
    }

    pub fn split_irreducible_node(statement: Statement) -> bool {
        let Some(splitnode) = Self::get_candidate_for_splitting(statement) else {
            return false;
        };

        Self::split_node(statement, splitnode);
        true
    }

    /// The body of `splitIrreducibleNode` after the candidate is chosen.
    pub(crate) fn split_node(statement: Statement, splitnode: Statement) {
        let enteredge = *splitnode
            .get_predecessor_edges(StatEdge::TYPE_REGULAR)
            .first()
            .expect("NoSuchElementException");

        let splitcopy = Self::copy_statement(splitnode, None, &mut JHashMap::new());
        Self::init_copied_statement(splitcopy);

        splitcopy.set_parent(Some(statement));
        let splitcopy_id = splitcopy.with(|d| d.id);
        statement
            .get_stats()
            .borrow_mut()
            .add_with_key(splitcopy, splitcopy_id);

        let enter_source = enteredge.get_source();
        for prededge in splitnode.get_predecessor_edges(Statement::STATEDGE_DIRECT_ALL) {
            if prededge.get_source() == enter_source
                || prededge.with(|d| d.closure) == Some(enter_source)
            {
                splitnode.remove_predecessor(prededge);
                prededge
                    .get_source()
                    .change_edge_node(EdgeDirection::Forward, prededge, splitcopy);
                splitcopy.add_predecessor(prededge);
            }
        }

        for succ in splitnode.get_successor_edges(Statement::STATEDGE_DIRECT_ALL) {
            let closure = succ.with(|d| d.closure);
            splitcopy.add_successor(StatEdge::new_closure(
                succ.get_type(),
                splitcopy,
                succ.get_destination(),
                closure,
            ));
        }
    }

    fn get_statement_size(statement: Statement) -> i32 {
        if let Some(block) = statement.try_as_basic_block() {
            block.get_block().get_seq().length()
        } else {
            let stats: Vec<Statement> = statement.get_stats().borrow().iter().copied().collect();
            stats
                .into_iter()
                .map(Self::get_statement_size)
                .fold(0i32, |a, b| a.wrapping_add(b))
        }
    }

    fn copy_statement(
        from: Statement,
        to: Option<Statement>,
        map_alt_to_copies: &mut JHashMap<Statement, Statement>,
    ) -> Statement {
        let to = match to {
            Some(to) => to,
            None => {
                let to = from.get_simple_copy();
                map_alt_to_copies.put(from, to);
                to
            }
        };

        let from_stats: Vec<Statement> = from.get_stats().borrow().iter().copied().collect();
        for &st in &from_stats {
            let stcopy = st.get_simple_copy();

            let stcopy_id = stcopy.with(|d| d.id);
            to.get_stats().borrow_mut().add_with_key(stcopy, stcopy_id);
            map_alt_to_copies.put(st, stcopy);
        }

        let to_stats: Vec<Statement> = to.get_stats().borrow().iter().copied().collect();
        for i in 0..from_stats.len() {
            let stold = from_stats[i];
            let stnew = to_stats[i];

            for edgeold in stold.get_successor_edges(Statement::STATEDGE_DIRECT_ALL) {
                let destination = edgeold.get_destination();
                let closure = edgeold.with(|d| d.closure);
                let edgenew = StatEdge::new_closure(
                    edgeold.get_type(),
                    stnew,
                    map_alt_to_copies
                        .get_cloned(&destination)
                        .unwrap_or(destination),
                    closure.map(|c| map_alt_to_copies.get_cloned(&c).unwrap_or(c)),
                );

                stnew.add_successor(edgenew);
            }
        }

        for i in 0..from_stats.len() {
            let stold = from_stats[i];
            let stnew = to_stats[i];

            Self::copy_statement(stold, Some(stnew), map_alt_to_copies);
        }

        to
    }

    fn init_copied_statement(statement: Statement) {
        statement.init_simple_copy();
        statement.set_copied(true);

        let stats: Vec<Statement> = statement.get_stats().borrow().iter().copied().collect();
        for st in stats {
            st.set_parent(Some(statement));
            Self::init_copied_statement(st);
        }
    }
}
