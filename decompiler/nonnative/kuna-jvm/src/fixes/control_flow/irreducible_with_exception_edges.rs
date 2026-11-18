//! kuna fix `irreducible_with_exception_edges` (spec 10.16): reducing an
//! irreducible level of `DomHelper` that holds exception edges.
//!
//! `IrreducibleCFGDeobfuscator::is_statement_irreducible` answers "reducible"
//! for any level where a statement has an exception edge, so such a level is
//! never split; and where the regular flow is irreducible no statement
//! dominates the loop, so the extended postdominators that would isolate a
//! `try` inside it are filtered out. A Kotlin suspend function whose state
//! machine resumes inside a loop that holds a `try` fails there with
//! `parsing failure!`. Where Vineflower gives up on such a level (irreducible
//! over its regular and exception edges together, a handler being entered from
//! its range), [`IrreducibleWithExceptionEdges::isolate`] collapses the
//! smallest single-entry, single-exit region that holds a handler's whole
//! range, the region `findGeneralStatement` would build for that head and
//! exit, so its `try` is built inside it; when there is none,
//! [`IrreducibleWithExceptionEdges::split`] splits as Java does, choosing among
//! the statements no exception edge touches, whose copies run the same code
//! under the same (absent) protection.

use crate::dom_helper::DomHelper;
use crate::general_statement::GeneralStatement;
use crate::irreducible_cfg_deobfuscator::IrreducibleCFGDeobfuscator;
use crate::rt::{JHashMap, JHashSet};
use crate::stat_edge::StatEdge;
use crate::statement::{EdgeDirection, Statement};

pub struct IrreducibleWithExceptionEdges;

impl IrreducibleWithExceptionEdges {
    /// Collapses the smallest single-entry region of `general` that holds the
    /// whole range of one of its exception handlers and leaves it through one
    /// node, into a new general statement; returns it.
    pub fn isolate(general: Statement) -> Option<Statement> {
        let stats: Vec<Statement> = general.get_stats().borrow().iter().copied().collect();
        if !Self::is_irreducible(&stats) {
            return None;
        }
        let first = general.get_first();
        let order = general.get_reverse_post_order_list();

        let mut best: Option<(Statement, Statement, Vec<Statement>)> = None;
        for &handler in &stats {
            let range = handler.get_neighbours(StatEdge::TYPE_EXCEPTION, EdgeDirection::Backward);
            if range.is_empty() || !range.iter().all(|r| stats.contains(r)) {
                continue;
            }
            let entries: Vec<Statement> = range
                .iter()
                .copied()
                .filter(|&r| {
                    Some(r) == first
                        || r.get_neighbours(StatEdge::TYPE_REGULAR, EdgeDirection::Backward)
                            .iter()
                            .any(|p| !range.contains(p))
                })
                .collect();
            let [head] = entries[..] else {
                continue;
            };

            for &post in &order {
                if post == head || range.contains(&post) {
                    continue;
                }
                let Some(nodes) = DomHelper::collect_region(general, head, post) else {
                    continue;
                };
                if !range.iter().all(|r| nodes.contains(r)) {
                    continue;
                }
                if best.as_ref().is_none_or(|(_, _, b)| nodes.len() < b.len()) {
                    best = Some((head, post, nodes));
                }
            }
        }

        let (head, post, nodes) = best?;
        let res = GeneralStatement::new(head, nodes, Some(post));
        general.collapse_nodes_to_statement(res.0);
        Some(res.0)
    }

    /// Splits one node of `statement` when the level has exception edges and
    /// is irreducible over its regular and exception edges; whether it split.
    pub fn split(statement: Statement) -> bool {
        let stats: Vec<Statement> = statement.get_stats().borrow().iter().copied().collect();
        let has_exception_edges = stats
            .iter()
            .any(|s| !s.get_successor_edges(StatEdge::TYPE_EXCEPTION).is_empty());
        if !has_exception_edges || !Self::is_irreducible(&stats) {
            return false;
        }

        let Some(splitnode) =
            IrreducibleCFGDeobfuscator::get_candidate_for_splitting_among(statement, |s| {
                s.get_successor_edges(StatEdge::TYPE_EXCEPTION).is_empty()
                    && s.get_predecessor_edges(StatEdge::TYPE_EXCEPTION).is_empty()
            })
        else {
            return false;
        };

        IrreducibleCFGDeobfuscator::split_node(statement, splitnode);
        true
    }

    /// The T1/T2 reduction of `isStatementIrreducible`, over regular and
    /// exception edges.
    fn is_irreducible(stats: &[Statement]) -> bool {
        let index: JHashMap<Statement, usize> = {
            let mut m = JHashMap::new();
            for (i, &s) in stats.iter().enumerate() {
                m.put(s, i);
            }
            m
        };
        let mut preds: Vec<JHashSet<usize>> = vec![JHashSet::new(); stats.len()];
        let mut succs: Vec<JHashSet<usize>> = vec![JHashSet::new(); stats.len()];
        for (i, &s) in stats.iter().enumerate() {
            let mut dests = s.get_neighbours(StatEdge::TYPE_REGULAR, EdgeDirection::Forward);
            dests.extend(s.get_neighbours(StatEdge::TYPE_EXCEPTION, EdgeDirection::Forward));
            for d in dests {
                if let Some(j) = index.get_cloned(&d) {
                    succs[i].add(j);
                    preds[j].add(i);
                }
            }
        }

        let mut alive: Vec<usize> = (0..stats.len()).collect();
        loop {
            let mut found = None;
            for &n in &alive {
                if succs[n].contains(&n) {
                    found = Some((n, 1));
                    break;
                } else if preds[n].len() == 1 {
                    found = Some((n, 2));
                    break;
                }
            }
            match found {
                Some((n, 1)) => {
                    succs[n].remove(&n);
                    preds[n].remove(&n);
                }
                Some((n, _)) => {
                    let pred = *preds[n].iter().next().expect("one predecessor");
                    let node_succs = succs[n].to_vec();
                    for &succ in &node_succs {
                        succs[pred].add(succ);
                        preds[succ].remove(&n);
                        preds[succ].add(pred);
                    }
                    succs[pred].remove(&n);
                    alive.retain(|&a| a != n);
                }
                None => return alive.len() > 1,
            }
        }
    }
}
