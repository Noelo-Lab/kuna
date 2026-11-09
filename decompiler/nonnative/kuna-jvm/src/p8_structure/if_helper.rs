//! Port of `org/jetbrains/java/decompiler/modules/decompiler/IfHelper.java`.
#![allow(unused_variables, dead_code)]

use crate::exprent::Exprent;
use crate::if_node::IfNode;
use crate::if_statement::IfStatement;
use crate::root_statement::RootStatement;
use crate::rt::JHashSet;
use crate::sequence_helper::SequenceHelper;
use crate::sequence_statement::SequenceStatement;
use crate::stat_edge::StatEdge;
use crate::statement::{Statement, StatementType};
use crate::unported;

pub struct IfHelper;

impl IfHelper {
    pub fn merge_all_ifs(root: RootStatement) -> bool {
        let res = Self::merge_all_ifs_rec(root.0, &mut JHashSet::new());
        if res {
            SequenceHelper::condense_sequences(root.0);
        }
        res
    }

    fn merge_all_ifs_rec(stat: Statement, set_reordered_ifs: &mut JHashSet<i32>) -> bool {
        let mut res = false;

        if stat.get_exprents().is_none() {
            loop {
                let mut changed = false;

                let stats = stat.get_stats().borrow().to_vec();
                for st in stats {
                    res |= Self::merge_all_ifs_rec(st, set_reordered_ifs);

                    if Self::merge_ifs(st, set_reordered_ifs) {
                        changed = true;
                        break;
                    }
                }

                res |= changed;

                if !changed {
                    break;
                }
            }
        }

        res
    }

    pub fn merge_ifs(statement: Statement, set_reordered_ifs: &mut JHashSet<i32>) -> bool {
        unported!("IfHelper.mergeIfs")
    }

    fn collapse_if_if(rtnode: &IfNode) -> bool {
        unported!("IfHelper.collapseIfIf")
    }

    fn collapse_if_else(rtnode: &IfNode) -> bool {
        unported!("IfHelper.collapseIfElse")
    }

    fn collapse_else(rtnode: &IfNode) -> bool {
        unported!("IfHelper.collapseElse")
    }

    fn collapse_ternary(rtnode: &IfNode) -> bool {
        unported!("IfHelper.collapseTernary")
    }

    fn if_else_chain_denesting(rtnode: &IfNode) -> bool {
        unported!("IfHelper.ifElseChainDenesting")
    }

    fn reorder_if(ifstat: IfStatement) -> bool {
        unported!("IfHelper.reorderIf")
    }

    fn swap_branches(ifstat: IfStatement, noifstat: bool, parent: SequenceStatement) {
        unported!("IfHelper.swapBranches")
    }

    fn has_direct_end_edge(stat: Statement, from: Statement) -> bool {
        unported!("IfHelper.hasDirectEndEdge")
    }

    fn get_next_statement(stat: Statement) -> Statement {
        let parent = stat.get_parent().unwrap();
        match parent.type_() {
            StatementType::Root => return parent.as_root().get_dummy_exit().0,
            StatementType::Do => return parent,
            StatementType::Sequence => {
                let stats = parent.as_sequence().get_stats().borrow().to_vec();
                if stats[stats.len() - 1] != stat {
                    for i in (0..stats.len()).rev() {
                        if stats[i] == stat {
                            return stats[i + 1];
                        }
                    }
                }
            }
            _ => {}
        }

        Self::get_next_statement(parent)
    }

    fn exists_path(from: Statement, to: Statement) -> bool {
        for edge in to.get_all_predecessor_edges() {
            if from.contains_statement_strict(edge.get_source()) {
                return true;
            }
        }

        false
    }

    pub fn prettify_ifs(stat: Statement) -> bool {
        let mut changed = false;
        let stats = stat.get_stats().borrow().to_vec();
        for st in stats {
            if st.is_switch() {
                continue;
            }

            Self::prettify_ifs(st);

            if let Some(if_st) = st.try_as_if() {
                changed |= Self::prettify_if(if_st);
            }
        }

        changed
    }

    fn prettify_if(if_st: IfStatement) -> bool {
        unported!("IfHelper.prettifyIf")
    }

    fn is_exit(stat: Statement) -> bool {
        if !stat.has_any_direct_successor() {
            return false;
        }

        let suc = stat.get_first_direct_successor();

        if !(suc.get_type() == StatEdge::TYPE_BREAK && suc.get_destination().is_dummy_exit()) {
            return false;
        }

        match stat.try_as_basic_block() {
            Some(bb) => {
                let exprents = bb.get_exprents().unwrap();
                !exprents.is_empty() && exprents.get(exprents.len() - 1).is_exit()
            }
            None => false,
        }
    }

    fn has_assert_field(expr: Exprent) -> bool {
        unported!("IfHelper.hasAssertField")
    }
}
