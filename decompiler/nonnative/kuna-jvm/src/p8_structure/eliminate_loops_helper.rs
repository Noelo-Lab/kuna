//! Port of `org/jetbrains/java/decompiler/modules/decompiler/EliminateLoopsHelper.java`.
#![allow(unused_variables, dead_code)]

use crate::do_statement::DoStatement;
use crate::rt::JHashMap;
use crate::stat_edge::StatEdge;
use crate::statement::Statement;
use crate::struct_class::StructClass;
use crate::unported;

pub struct EliminateLoopsHelper;

impl EliminateLoopsHelper {
    pub fn eliminate_loops(root: Statement, cl: StructClass) -> bool {
        unported!("EliminateLoopsHelper.eliminateLoops")
    }

    fn eliminate_loops_rec(stat: Statement) -> bool {
        let stats = stat.get_stats().borrow().to_vec();
        for st in stats {
            if Self::eliminate_loops_rec(st) {
                return true;
            }
        }

        if let Some(loop_) = stat.try_as_do() {
            if Self::is_loop_redundant(loop_) {
                return true;
            }
        }

        false
    }

    fn is_loop_redundant(loop_: DoStatement) -> bool {
        unported!("EliminateLoopsHelper.isLoopRedundant")
    }

    fn get_max_break_lift(
        stat: Statement,
        edge: StatEdge,
        stat_labeled: &JHashMap<i32, bool>,
        max: Statement,
    ) -> Statement {
        let mut closure = stat;
        let mut newclosure = stat;

        while let Some(next) = Self::get_next_break_lift(newclosure, edge, stat_labeled, max) {
            newclosure = next;
            closure = newclosure;
        }

        closure
    }

    fn get_next_break_lift(
        stat: Statement,
        edge: StatEdge,
        stat_labeled: &JHashMap<i32, bool>,
        max: Statement,
    ) -> Option<Statement> {
        unported!("EliminateLoopsHelper.getNextBreakLift")
    }

    fn eliminate_loop(loop_: Statement, parentloop: Statement) {
        unported!("EliminateLoopsHelper.eliminateLoop")
    }
}
