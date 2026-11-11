//! Port of `org/jetbrains/java/decompiler/modules/decompiler/LoopExtractHelper.java`.
//!
//! kuna option `jvm_javac_layout` (`kuna_javac_layout`): the body of a
//! loop's first or last if stays in the loop when javac placed it there, an
//! if of the body whose body javac placed after the loop is moved after it,
//! and a first if whose other path continues the loop is not made its exit.
//!
//! kuna fix `loop_first_if_exit` (`kuna_fix_loop_first_if_exit`): a loop's
//! first if becomes its exit test only when the path past it leaves the loop.

use crate::do_statement::{DoStatement, Type as LoopType};
use crate::function_exprent::{FunctionExprent, FunctionType};
use crate::if_statement::IfStatement;
use crate::kuna_fixes::{self, KunaFix};
use crate::p8_structure::{kuna_fix_loop_first_if_exit, kuna_javac_layout};
use crate::sequence_helper::SequenceHelper;
use crate::sequence_statement::SequenceStatement;
use crate::stat_edge::StatEdge;
use crate::statement::{EdgeDirection, Statement};
use crate::validation_helper::ValidationHelper;

pub struct LoopExtractHelper;

impl LoopExtractHelper {
    pub fn extract_loops(root: Statement) -> bool {
        let res = Self::extract_loops_rec(root) != 0;

        if res {
            SequenceHelper::condense_sequences(root);
        }

        res
    }

    fn extract_loops_rec(stat: Statement) -> i32 {
        let mut res = false;

        loop {
            let mut updated = false;

            let stats = stat.get_stats().borrow().to_vec();
            for st in stats {
                let extr = Self::extract_loops_rec(st);
                res |= extr != 0;

                if extr == 2 {
                    updated = true;
                    break;
                }
            }

            if !updated {
                break;
            }
        }

        if let Some(dostat) = stat.try_as_do() {
            if Self::extract_loop(dostat) {
                ValidationHelper::validate_statement(stat.get_top_parent());
                return 2;
            }
        }

        if res {
            1
        } else {
            0
        }
    }

    fn extract_loop(stat: DoStatement) -> bool {
        if stat.get_looptype() != LoopType::Infinite {
            return false;
        }

        let mut stats: Vec<Statement> = Vec::new();
        let label_edges = stat.get_label_edges().borrow().to_vec();
        for edge in label_edges {
            if edge.get_type() != StatEdge::TYPE_CONTINUE && !edge.get_destination().is_dummy_exit()
            {
                if edge.get_type() == StatEdge::TYPE_BREAK
                    && Self::is_extern_statement(stat, edge.get_source(), edge.get_source())
                {
                    stats.push(edge.get_source());
                } else {
                    return false;
                }
            }
        }

        if !stats.is_empty() {
            let last = stat
                .get_parent()
                .expect("parent")
                .get_stats()
                .borrow()
                .get_last();
            if last != stat.0 {
                return false;
            }
        }

        if !Self::extract_first_if(stat, &stats) {
            Self::extract_last_if(stat, &stats) || Self::extract_tail_if(stat, &stats)
        } else {
            true
        }
    }

    /// kuna option `jvm_javac_layout`: moves after `stat` the body of an if
    /// of its body that javac placed after the loop's code, and that leaves
    /// the loop without coming back: the code after the loop that the
    /// source's `break` reaches.
    fn extract_tail_if(stat: DoStatement, stats: &[Statement]) -> bool {
        if !kuna_javac_layout::active() {
            return false;
        }
        let Some(ifstat) = kuna_javac_layout::tail_if(stat) else {
            return false;
        };
        let target = ifstat.get_ifstat().expect("IfStatement.ifstat");
        if !Self::is_extern_statement(stat, target, target)
            || stats.iter().any(|s| !target.contains_statement(*s))
        {
            return false;
        }
        Self::extract_if_block(stat, ifstat);
        true
    }

    fn extract_last_if(stat: DoStatement, stats: &[Statement]) -> bool {
        let mut last = stat.get_first();
        while let Some(seq) = last.filter(|l| l.is_sequence()) {
            last = Some(seq.get_stats().borrow().get_last());
        }

        let Some(lastif) = last.and_then(|l| l.try_as_if()) else {
            return false;
        };
        if lastif.iftype() != IfStatement::IFTYPE_IF {
            return false;
        }
        let Some(ifstat) = lastif.get_ifstat() else {
            return false;
        };

        let succs = lastif.get_all_successor_edges();
        if succs.is_empty() {
            return false;
        }
        let elseedge = succs[0];

        if elseedge.get_type() != StatEdge::TYPE_CONTINUE || elseedge.closure() != Some(stat.0) {
            return false;
        }

        let mut set = stat.get_neighbours_set(StatEdge::TYPE_CONTINUE, EdgeDirection::Backward);
        set.remove(&lastif.0);

        if set.is_empty()
            && Self::is_extern_statement(stat, ifstat, ifstat)
            && !Self::kept_inside(stat, lastif)
        {
            let mut first = stat.get_first();
            while let Some(seq) = first.filter(|f| f.is_sequence()) {
                first = seq.get_first();
            }
            if first
                .and_then(|f| f.try_as_do())
                .is_some_and(|d| d.get_looptype() == LoopType::Infinite)
            {
                return false;
            }

            for &s in stats {
                if !ifstat.contains_statement(s) {
                    return false;
                }
            }
            Self::extract_if_block(stat, lastif);
            return true;
        }
        false
    }

    fn extract_first_if(stat: DoStatement, stats: &[Statement]) -> bool {
        let mut first = stat.get_first();
        while let Some(seq) = first.filter(|f| f.is_sequence()) {
            first = seq.get_first();
        }

        let Some(firstif) = first.and_then(|f| f.try_as_if()) else {
            return false;
        };

        if !firstif
            .get_first()
            .expect("IfStatement.first")
            .get_exprents()
            .expect("if head exprents")
            .is_empty()
        {
            return false;
        }

        if firstif.iftype() != IfStatement::IFTYPE_IF {
            return false;
        }
        let Some(ifstat) = firstif.get_ifstat() else {
            return false;
        };

        if Self::is_extern_statement(stat, ifstat, ifstat) && !Self::kept_inside(stat, firstif) {
            for &s in stats {
                if !ifstat.contains_statement(s) {
                    return false;
                }
            }

            Self::extract_if_block(stat, firstif);

            return true;
        }

        if stat.get_stats().borrow().len() == 1 {
            let mut continues = ifstat.get_successor_edges(StatEdge::TYPE_CONTINUE);
            if ifstat.is_sequence() {
                let last = ifstat.get_stats().borrow().get_last();
                continues.extend(last.get_successor_edges(StatEdge::TYPE_CONTINUE));
            }

            if continues.len() == 1
                && continues[0].get_destination() == stat.0
                && !Self::else_continues(stat, firstif)
            {
                let mut check = stat.0;

                while check.get_successor_edges(StatEdge::TYPE_REGULAR).is_empty() {
                    match check.get_parent() {
                        Some(parent) => check = parent,
                        None => return false,
                    }
                }

                let destination =
                    check.get_successor_edges(StatEdge::TYPE_REGULAR)[0].get_destination();
                if kuna_fixes::enabled(KunaFix::LoopFirstIfExit)
                    && !kuna_fix_loop_first_if_exit::exits_to(firstif, destination)
                {
                    return false;
                }

                Self::extract_if_block_into_loop(stat, firstif, destination);

                return true;
            }
        }

        false
    }

    /// kuna option `jvm_javac_layout`: whether the path past `ifstat`, the
    /// first if of `loop_`, goes back to the loop's start. Making the if a
    /// `break` out of the loop (`extract_if_block_into_loop`) would end the
    /// loop where it goes on (an empty `while` sharing its start, javac's
    /// `while (c(++i)) {}` in a `for (;;)`): the destination is the regular
    /// successor of the nearest statement around that has one, which a loop
    /// whose tail [`Self::extract_tail_if`] moved after it has.
    fn else_continues(loop_: DoStatement, ifstat: IfStatement) -> bool {
        kuna_javac_layout::active()
            && ifstat
                .get_all_successor_edges()
                .iter()
                .any(|e| e.get_type() == StatEdge::TYPE_CONTINUE && e.get_destination() == loop_.0)
    }

    /// kuna option `jvm_javac_layout`: whether javac placed the body of
    /// `ifstat` inside `loop_`, so it is not moved after the loop.
    fn kept_inside(loop_: DoStatement, ifstat: IfStatement) -> bool {
        kuna_javac_layout::active() && kuna_javac_layout::inside_loop(loop_, ifstat)
    }

    fn is_extern_statement(loop_: DoStatement, block: Statement, stat: Statement) -> bool {
        for edge in stat.get_all_successor_edges() {
            if loop_.contains_statement(edge.get_destination())
                && !block.contains_statement(edge.get_destination())
            {
                return false;
            }
        }

        let stats = stat.get_stats().borrow().to_vec();

        for st in stats {
            if !Self::is_extern_statement(loop_, block, st) {
                return false;
            }
        }

        true
    }

    fn extract_if_block(loop_: DoStatement, ifstat: IfStatement) {
        let target = ifstat.get_ifstat().expect("IfStatement.ifstat");
        let ifedge = ifstat.get_if_edge().expect("IfStatement.ifedge");

        ifstat.set_ifstat(None);
        ifedge
            .get_source()
            .change_edge_type(EdgeDirection::Forward, ifedge, StatEdge::TYPE_BREAK);
        ifedge.with_mut(|d| d.closure = Some(loop_.0));
        ifstat
            .get_stats()
            .borrow_mut()
            .remove_with_key(&target.id());

        loop_.add_labeled_edge(ifedge);

        let block = SequenceStatement::new(vec![loop_.0, target]);
        loop_
            .get_parent()
            .expect("parent")
            .replace_statement(loop_.0, block.0);
        block.set_all_parent();

        loop_.add_successor(StatEdge::new(StatEdge::TYPE_REGULAR, loop_.0, target));

        let block_label_edges = block.get_label_edges().borrow().to_vec();
        for edge in block_label_edges {
            if edge.get_type() == StatEdge::TYPE_CONTINUE || edge == ifedge {
                loop_.add_labeled_edge(edge);
            }
        }

        for edge in block.get_predecessor_edges(StatEdge::TYPE_CONTINUE) {
            if loop_.contains_statement_strict(edge.get_source()) {
                block.remove_predecessor(edge);
                edge.get_source()
                    .change_edge_node(EdgeDirection::Forward, edge, loop_.0);
                loop_.add_predecessor(edge);
            }
        }

        let link = target.get_predecessor_edges(StatEdge::TYPE_BREAK);
        if link.len() == 1 {
            link[0].with_mut(|d| d.can_inline = false);
        }
    }

    fn extract_if_block_into_loop(
        loop_: DoStatement,
        if_stat: IfStatement,
        destination: Statement,
    ) {
        let target = if_stat.get_ifstat().expect("IfStatement.ifstat");
        let ifedge = if_stat.get_if_edge().expect("IfStatement.ifedge");

        if_stat.set_ifstat(None);
        ifedge.get_destination().remove_predecessor(ifedge);
        ifedge
            .get_source()
            .change_edge_type(EdgeDirection::Forward, ifedge, StatEdge::TYPE_BREAK);
        ifedge.with_mut(|d| d.closure = Some(loop_.0));
        if_stat
            .get_stats()
            .borrow_mut()
            .remove_with_key(&target.id());

        ifedge.set_destination(destination);
        destination.add_predecessor(ifedge);

        loop_.add_labeled_edge(ifedge);
        if_stat.set_if_edge(Some(ifedge));

        let seq = SequenceStatement::new(vec![if_stat.0, target]);
        if_stat.replace_with(seq.0);
        seq.set_all_parent();

        if_stat.add_successor(StatEdge::new(StatEdge::TYPE_REGULAR, if_stat.0, target));

        let expr = if_stat.get_headexprent();
        expr.set_condition(
            FunctionExprent::new_single(FunctionType::BoolNot, expr.get_condition(), None).0,
        );
    }
}
