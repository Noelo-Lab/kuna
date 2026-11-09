//! Port of `org/jetbrains/java/decompiler/modules/decompiler/LoopExtractHelper.java`.
#![allow(unused_variables, dead_code)]

use crate::do_statement::DoStatement;
use crate::if_statement::IfStatement;
use crate::sequence_helper::SequenceHelper;
use crate::statement::Statement;
use crate::unported;

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
        unported!("LoopExtractHelper.extractLoopsRec")
    }

    fn extract_loop(stat: DoStatement) -> bool {
        unported!("LoopExtractHelper.extractLoop")
    }

    fn extract_last_if(stat: DoStatement, stats: &[Statement]) -> bool {
        unported!("LoopExtractHelper.extractLastIf")
    }

    fn extract_first_if(stat: DoStatement, stats: &[Statement]) -> bool {
        unported!("LoopExtractHelper.extractFirstIf")
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
        unported!("LoopExtractHelper.extractIfBlock")
    }

    fn extract_if_block_into_loop(
        loop_: DoStatement,
        if_stat: IfStatement,
        destination: Statement,
    ) {
        unported!("LoopExtractHelper.extractIfBlockIntoLoop")
    }
}
