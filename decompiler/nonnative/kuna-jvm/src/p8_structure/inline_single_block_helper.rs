//! Port of `org/jetbrains/java/decompiler/modules/decompiler/InlineSingleBlockHelper.java`.
//!
//! kuna fix `catch_tail_inline` (`kuna_fixes`): in `isInlineable`, statements
//! that read the variable of a catch clause the break to them leaves are
//! inlined even when the break is implicit and they break out of the sequence.
//!
//! kuna option `jvm_javac_layout` (`kuna_javac_layout::keeps_tail`): code javac
//! placed after a loop or switch and reached by a `break` stays there.

use crate::if_statement::IfStatement;
use crate::jthrow;
use crate::kuna_fixes::{self, KunaFix};
use crate::merge_helper::MergeHelper;
use crate::p8_structure::kuna_fix_catch_tail_inline;
use crate::p8_structure::kuna_javac_layout;
use crate::root_statement::RootStatement;
use crate::sequence_helper::SequenceHelper;
use crate::sequence_statement::SequenceStatement;
use crate::stat_edge::StatEdge;
use crate::statement::{EdgeDirection, Statement};

pub struct InlineSingleBlockHelper;

impl InlineSingleBlockHelper {
    pub fn inline_single_blocks(root: RootStatement) -> bool {
        let mut res = false;
        let mut progress = true;
        while progress {
            progress = Self::inline_single_blocks_rec(root.0);
            res |= progress;
        }

        if res {
            SequenceHelper::condense_sequences(root.0);
        }

        res
    }

    fn inline_single_blocks_rec(stat: Statement) -> bool {
        let mut res = false;

        let stats = stat.get_stats().borrow().to_vec();

        for st in stats {
            res |= Self::inline_single_blocks_rec(st);
        }

        if let Some(seq) = stat.try_as_sequence() {
            let mut i = 1;
            while i < seq.get_stats().borrow().len() {
                if Self::is_inlineable(seq, i as i32) {
                    Self::inline_block(seq, i as i32);
                    return true;
                }
                i += 1;
            }
        }

        res
    }

    fn inline_block(seq: SequenceStatement, index: i32) {
        let index = index as usize;
        let first = seq.get_stats().borrow().get(index);
        let pre = seq.get_stats().borrow().get(index - 1);
        pre.remove_successor(pre.get_first_successor());

        let edge = first.get_predecessor_edges(StatEdge::TYPE_BREAK)[0];
        let source = edge.get_source();
        let parent = source.get_parent();
        source.remove_successor(edge);

        let mut lst: Vec<Statement> = Vec::new();
        let mut i = seq.get_stats().borrow().len() as i32 - 1;
        while i >= index as i32 {
            let removed = seq.get_stats().borrow_mut().remove_at(i as usize);
            lst.insert(0, removed);
            i -= 1;
        }

        let ifparent = parent.and_then(|p| p.try_as_if()).filter(|ifparent| {
            ifparent.iftype() == IfStatement::IFTYPE_IF && Some(source) == ifparent.get_first()
        });
        if let Some(ifparent) = ifparent {
            let block = SequenceStatement::new(lst);
            block.set_all_parent();

            let newedge = StatEdge::new(StatEdge::TYPE_REGULAR, source, block.0);
            source.add_successor(newedge);
            ifparent.set_if_edge(Some(newedge));
            ifparent.set_ifstat(Some(block.0));

            let block_id = block.id();
            ifparent
                .get_stats()
                .borrow_mut()
                .add_with_key(block.0, block_id);
            block.set_parent(Some(ifparent.0));
        } else {
            lst.insert(0, source);

            let block = SequenceStatement::new(lst);
            block.set_all_parent();

            let parent = parent.expect("parent");
            parent.replace_statement(source, block.0);

            for prededge in block.get_predecessor_edges(StatEdge::TYPE_CONTINUE) {
                block.remove_predecessor(prededge);
                prededge
                    .get_source()
                    .change_edge_node(EdgeDirection::Forward, prededge, source);
                source.add_predecessor(prededge);

                source.add_labeled_edge(prededge);
            }

            if let Some(swst) = parent.try_as_switch() {
                swst.sort_edges_and_nodes();
            }

            source.add_successor(StatEdge::new(StatEdge::TYPE_REGULAR, source, first));

            let block_stats = block.get_stats();
            let last = {
                let stats = block_stats.borrow();
                stats.get(stats.len() - 1)
            };
            if !last.has_any_successor() {
                let label_edges = last.get_label_edges().borrow().to_vec();
                for label_edge in label_edges {
                    if label_edge.get_type() == StatEdge::TYPE_BREAK
                        && MergeHelper::is_direct_path(last, label_edge.get_destination())
                    {
                        let mut closure = parent;
                        while !MergeHelper::is_direct_path(closure, label_edge.get_destination()) {
                            match closure.get_parent() {
                                Some(closure_parent) => closure = closure_parent,
                                None => {
                                    jthrow!(IllegalStateException; "Where have we been inlined to?")
                                }
                            }
                        }
                        label_edge.with_mut(|d| d.closure = Some(closure));
                        last.get_label_edges().borrow_mut().remove(&label_edge);
                    }
                }
            }
        }
    }

    fn is_inlineable(seq: SequenceStatement, index: i32) -> bool {
        let index = index as usize;
        let first = seq.get_stats().borrow().get(index);
        let pre = seq.get_stats().borrow().get(index - 1);

        if pre.has_basic_succ_edge() {
            return false;
        }

        let lst = first.get_predecessor_edges(StatEdge::TYPE_BREAK);

        if lst.len() == 1 {
            let edge = lst[0];

            if kuna_javac_layout::active()
                && kuna_javac_layout::keeps_tail(pre, first, edge.get_source())
            {
                return false;
            }

            if Self::same_catch_ranges(edge) {
                if !edge.can_inline() {
                    return false;
                }

                if !edge.explicit()
                    && !(kuna_fixes::enabled(KunaFix::CatchTailInline)
                        && kuna_fix_catch_tail_inline::reads_left_catch_var(seq, index, edge))
                {
                    let mut i = index;
                    while i < seq.get_stats().borrow().len() {
                        let st = seq.get_stats().borrow().get(i);
                        if !Self::no_exit_labels(st, seq.0) {
                            return false;
                        }
                        i += 1;
                    }
                }

                if let Some(swst) = edge
                    .get_source()
                    .get_parent()
                    .and_then(|p| p.try_as_switch())
                {
                    if swst.get_case_statements().is_empty() {
                        return false;
                    }
                }

                let no_pre_successors = !pre.has_any_successor();

                if no_pre_successors {
                    return false;
                }

                return true;
            }
        }

        false
    }

    fn same_catch_ranges(edge: StatEdge) -> bool {
        let mut from = edge.get_source();
        let to = edge.get_destination();

        loop {
            let parent = from.get_parent().expect("parent");
            if parent.contains_statement_strict(to) {
                break;
            }

            if parent.is_catch() || parent.is_catch_all() {
                if parent.get_first() == Some(from) {
                    return false;
                }
            } else if parent.is_synchronized() && parent.get_stats().borrow().get(1) == from {
                return false;
            }

            from = parent;
        }

        true
    }

    fn no_exit_labels(block: Statement, sequence: Statement) -> bool {
        for edge in block.get_all_successor_edges() {
            if edge.get_type() != StatEdge::TYPE_REGULAR
                && !edge.get_destination().is_dummy_exit()
                && !sequence.contains_statement_strict(edge.get_destination())
            {
                return false;
            }
        }

        let stats = block.get_stats().borrow().to_vec();

        for st in stats {
            if !Self::no_exit_labels(st, sequence) {
                return false;
            }
        }

        true
    }

    pub fn is_break_edge_labeled(source: Statement, closure: Statement) -> bool {
        if closure.is_do() || closure.is_switch() {
            let parent = source.get_parent();
            parent != Some(closure)
                && (parent.is_some_and(|p| p.is_do())
                    || parent.is_some_and(|p| p.is_switch())
                    || Self::is_break_edge_labeled(parent.unwrap(), closure))
        } else {
            true
        }
    }
}
