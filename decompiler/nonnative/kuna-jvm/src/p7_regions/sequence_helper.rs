//! Port of `org/jetbrains/java/decompiler/modules/decompiler/SequenceHelper.java`.
#![allow(unused_variables, dead_code)]

use crate::basic_block_statement::BasicBlockStatement;
use crate::sequence_statement::SequenceStatement;
use crate::stat_edge::StatEdge;
use crate::statement::{Statement, StatementType};
use crate::unported;
use crate::validation_helper::ValidationHelper;

pub struct SequenceHelper;

impl SequenceHelper {
    pub fn condense_sequences(root: Statement) {
        ValidationHelper::validate_statement(root.get_top_parent());

        Self::condense_sequences_rec(root);

        ValidationHelper::validate_statement(root.get_top_parent());
    }

    fn condense_sequences_rec(stat: Statement) {
        unported!("SequenceHelper.condenseSequencesRec")
    }

    pub fn clear_closure(stat: Statement, clear: Statement) {
        for edge in stat.get_all_successor_edges() {
            let closure = edge.with(|d| d.closure);
            if closure == Some(clear) {
                closure
                    .unwrap()
                    .get_label_edges()
                    .borrow_mut()
                    .remove(&edge);
                edge.with_mut(|d| d.closure = None);
            }
        }

        let stats = stat.get_stats().borrow().to_vec();

        for st in stats {
            Self::clear_closure(st, clear);
        }
    }

    fn is_sequence_disbandable(block: Statement, next: Statement) -> bool {
        let last = block.get_stats().borrow().get_last();
        if last.has_any_successor() && last.get_first_successor().get_destination() != next {
            return false;
        }

        for edge in next.get_predecessor_edges(StatEdge::TYPE_BREAK) {
            if last != edge.get_source() && !last.contains_statement_strict(edge.get_source()) {
                return false;
            }
        }

        true
    }

    fn remove_empty_statements(sequence: SequenceStatement) {
        unported!("SequenceHelper.removeEmptyStatements")
    }

    fn merge_flat_statements(sequence: SequenceStatement) {
        unported!("SequenceHelper.mergeFlatStatements")
    }

    fn get_first_exprentlist(stat: Statement) -> Option<Statement> {
        if stat.get_exprents().is_some() {
            return Some(stat);
        }

        match stat.with(|d| d.type_) {
            StatementType::If
            | StatementType::Sequence
            | StatementType::Switch
            | StatementType::Synchronized => Self::get_first_exprentlist(stat.get_first().unwrap()),
            _ => None,
        }
    }

    pub fn destroy_and_flatten_statement(stat: Statement) -> BasicBlockStatement {
        unported!("SequenceHelper.destroyAndFlattenStatement")
    }

    pub fn destroy_statement_content(stat: Statement, self_: bool) {
        let stats = stat.get_stats().borrow().to_vec();
        for st in stats {
            Self::destroy_statement_content(st, true);
        }

        stat.get_stats().borrow_mut().clear();

        if self_ {
            for edge in stat.get_all_successor_edges() {
                stat.remove_successor(edge);
            }

            for edge in stat.get_all_predecessor_edges() {
                edge.get_source().remove_successor(edge);
            }
        }
    }

    pub fn first_non_seq(stat: Statement) -> Statement {
        let Some(mut first) = stat.get_first() else {
            return stat;
        };

        while first.is_sequence() {
            first = first.get_first().unwrap();
        }

        first
    }
}
