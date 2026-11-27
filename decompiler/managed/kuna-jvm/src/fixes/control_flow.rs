//! Fix family `control_flow`: `catch_tail_inline` moves the code only a
//! handler reaches into the handler when it reads the catch variable;
//! `irreducible_with_exception_edges` reduces an irreducible level that holds
//! exception edges; `loop_first_if_exit` makes a loop's first `if` its exit
//! test only when the path past the `if` leaves the loop.

use crate::basic_block_statement::BasicBlockStatement;
use crate::dom_helper::DomHelper;
use crate::exit_exprent::Type as ExitType;
use crate::general_statement::GeneralStatement;
use crate::if_statement::IfStatement;
use crate::irreducible_cfg_deobfuscator::IrreducibleCFGDeobfuscator;
use crate::root_statement::RootStatement;
use crate::rt::{JHashMap, JHashSet};
use crate::sequence_statement::SequenceStatement;
use crate::stat_edge::StatEdge;
use crate::statement::{EdgeDirection, Statement};
use crate::statement_iterator::StatementIterator;
use crate::var_version_pair::VarVersionPair;

/// Whether the statements of `seq` from `index` on read the variable of a
/// catch clause that `edge`, the only break to them, leaves.
pub fn reads_left_catch_var(seq: SequenceStatement, index: usize, edge: StatEdge) -> bool {
    let to = edge.get_destination();
    let mut from = edge.get_source();
    let mut vars: JHashSet<VarVersionPair> = JHashSet::new();
    while let Some(parent) = from.get_parent() {
        if parent.contains_statement_strict(to) {
            break;
        }
        if let Some(catch) = parent.try_as_catch() {
            let stats = parent.get_stats().borrow().to_vec();
            if let Some(i) = stats.iter().position(|s| *s == from).filter(|&i| i > 0) {
                if let Some(var) = catch.get_vars().to_vec().get(i - 1) {
                    vars.add(var.get_var_version_pair());
                }
            }
        }
        from = parent;
    }
    if vars.is_empty() {
        return false;
    }
    let stats = seq.get_stats().borrow().to_vec();
    let mut found = false;
    for st in &stats[index..] {
        StatementIterator::iterate(*st, &mut |e| {
            if let Some(var) = e.try_as_var() {
                found |= vars.contains(&var.get_var_version_pair());
            }
            0
        });
    }
    found
}

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

/// Fix `initializer_return_break`: Java allows no `return` in a static
/// initializer, so each `return;` of `root`, a `<clinit>` body, that is not
/// its last statement becomes a `break` out of the innermost enclosing
/// statement after which the body ends, into an empty block placed after it.
/// A statement qualifies when it is the last of a sequence that ends the
/// body; the `break` is labeled unless that statement is a loop or switch
/// with none between.
pub fn initializer_return_break(root: RootStatement) {
    let mut tails: JHashSet<Statement> = JHashSet::new();
    let mut queue: Vec<Statement> = root.get_first().into_iter().collect();
    while let Some(stat) = queue.pop() {
        if !tails.add(stat) {
            continue;
        }
        if stat.is_sequence() {
            queue.extend(stat.get_stats().borrow().to_vec().last().copied());
        } else if let Some(ifst) = stat.try_as_if() {
            queue.extend(ifst.get_ifstat().into_iter().chain(ifst.get_elsestat()));
        }
    }
    let exit = root.get_dummy_exit().0;
    let mut ends: JHashMap<Statement, Statement> = JHashMap::new();
    for edge in exit.get_all_predecessor_edges() {
        let block = edge.get_source();
        let Some(exprents) = block.get_exprents().filter(|e| !e.is_empty()) else {
            continue;
        };
        let last = exprents.get(exprents.len() - 1);
        if !last
            .try_as_exit()
            .is_some_and(|ex| ex.get_exit_type() == ExitType::Return && ex.get_value().is_none())
            || tails.contains(&block)
        {
            continue;
        }
        let mut target = None;
        let mut between_loop = false;
        let mut scope = block.get_parent();
        while let Some(stat) = scope {
            let parent = stat.get_parent();
            let last_of_sequence = parent.is_some_and(|p| {
                p.is_sequence() && p.get_stats().borrow().to_vec().last() == Some(&stat)
            });
            if tails.contains(&stat) && last_of_sequence {
                target = Some(stat);
                break;
            }
            between_loop |= stat.is_do() || stat.is_switch();
            scope = parent;
        }
        let Some(target) = target else {
            continue;
        };
        let end = match ends.get(&target) {
            Some(end) => *end,
            None => {
                let end = BasicBlockStatement::create();
                let seq = target.get_parent().expect("checked above");
                seq.get_stats().borrow_mut().add_with_key(end.0, end.id());
                end.set_parent(Some(seq));
                for out in target.get_all_successor_edges() {
                    if out.get_destination() == exit {
                        target.remove_successor(out);
                    }
                }
                target.add_successor(StatEdge::new(StatEdge::TYPE_REGULAR, target, end.0));
                let tail = StatEdge::new_closure(StatEdge::TYPE_BREAK, end.0, exit, edge.closure());
                tail.with_mut(|d| {
                    d.labeled = false;
                    d.explicit = false;
                });
                end.add_successor(tail);
                ends.put(target, end.0);
                end.0
            }
        };
        exprents.remove_at(exprents.len() - 1);
        block.remove_successor(edge);
        let jump = StatEdge::new_closure(StatEdge::TYPE_BREAK, block, end, Some(target));
        let labeled = between_loop || !(target.is_do() || target.is_switch());
        jump.with_mut(|d| d.labeled = labeled);
        block.add_successor(jump);
    }
}

/// Whether the path past `firstif` already leaves for `destination`.
pub fn exits_to(firstif: IfStatement, destination: Statement) -> bool {
    let succs = firstif.get_successor_edges(Statement::STATEDGE_DIRECT_ALL);
    !succs.is_empty()
        && succs
            .iter()
            .all(|e| e.get_type() == StatEdge::TYPE_BREAK && e.get_destination() == destination)
}
