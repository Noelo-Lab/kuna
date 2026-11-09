//! Port of `org/jetbrains/java/decompiler/modules/decompiler/InlineSingleBlockHelper.java`.
#![allow(unused_variables, dead_code)]

use crate::root_statement::RootStatement;
use crate::sequence_helper::SequenceHelper;
use crate::sequence_statement::SequenceStatement;
use crate::stat_edge::StatEdge;
use crate::statement::Statement;
use crate::unported;

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
        unported!("InlineSingleBlockHelper.inlineBlock")
    }

    fn is_inlineable(seq: SequenceStatement, index: i32) -> bool {
        unported!("InlineSingleBlockHelper.isInlineable")
    }

    fn same_catch_ranges(edge: StatEdge) -> bool {
        unported!("InlineSingleBlockHelper.sameCatchRanges")
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
