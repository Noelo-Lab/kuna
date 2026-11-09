//! Port of `org/jetbrains/java/decompiler/modules/decompiler/SequenceHelper.java`.

use crate::basic_block_statement::BasicBlockStatement;
use crate::dec_helper::DecHelper;
use crate::jthrow;
use crate::rt::JList;
use crate::sequence_statement::SequenceStatement;
use crate::stat_edge::StatEdge;
use crate::statement::{EdgeDirection, Statement, StatementType};
use crate::validation_helper::ValidationHelper;

pub struct SequenceHelper;

impl SequenceHelper {
    pub fn condense_sequences(root: Statement) {
        ValidationHelper::validate_statement(root.get_top_parent());

        Self::condense_sequences_rec(root);

        ValidationHelper::validate_statement(root.get_top_parent());
    }

    fn condense_sequences_rec(stat: Statement) {
        let mut stat = stat;

        if stat.is_sequence() {
            let mut lst: Vec<Statement> = stat.get_stats().borrow().to_vec();

            let mut unfolded = false;

            let mut i: isize = 0;
            while (i as usize) < lst.len() {
                let iu = i as usize;
                let st = lst[iu];
                if st.is_sequence() {
                    Self::remove_empty_statements(st.as_sequence());

                    if iu == lst.len() - 1 || Self::is_sequence_disbandable(st, lst[iu + 1]) {
                        let first = st.get_first().expect("NullPointerException");
                        for edge in st.get_all_predecessor_edges() {
                            st.remove_predecessor(edge);
                            edge.get_source()
                                .change_edge_node(EdgeDirection::Forward, edge, first);
                            first.add_predecessor(edge);
                        }

                        let last = st.get_stats().borrow().get_last();
                        if last.get_all_successor_edges().is_empty() && iu < lst.len() - 1 {
                            last.add_successor(StatEdge::new(
                                StatEdge::TYPE_REGULAR,
                                last,
                                lst[iu + 1],
                            ));
                        } else {
                            for edge in last.get_all_successor_edges() {
                                if iu == lst.len() - 1 {
                                    if edge.closure() == Some(st) {
                                        stat.add_labeled_edge(edge);
                                    }
                                } else {
                                    edge.get_source().change_edge_type(
                                        EdgeDirection::Forward,
                                        edge,
                                        StatEdge::TYPE_REGULAR,
                                    );
                                    let Some(closure) = edge.closure() else {
                                        jthrow!(
                                            IllegalStateException;
                                            "Closure is null for edge {} in statement {}",
                                            edge, last
                                        );
                                    };

                                    closure.get_label_edges().borrow_mut().remove(&edge);
                                    edge.with_mut(|d| d.closure = None);
                                }
                            }
                        }

                        for edge in st.get_all_successor_edges() {
                            st.remove_successor(edge);
                        }

                        let label_edges = st.get_label_edges().borrow().clone();
                        for edge in label_edges.to_vec() {
                            if edge.get_source() != last {
                                if last.is_basic_block() {
                                    stat.add_labeled_edge(edge);
                                } else {
                                    last.add_labeled_edge(edge);
                                }
                            }
                        }

                        lst.remove(iu);
                        let sub = st.get_stats().borrow().to_vec();
                        lst.splice(iu..iu, sub);
                        i -= 1;

                        Self::clear_closure(st, st);

                        unfolded = true;
                    }
                }
                i += 1;
            }

            if unfolded {
                let sequence = SequenceStatement::new(lst);
                sequence.set_all_parent();

                stat.get_parent()
                    .expect("NullPointerException")
                    .replace_statement(stat, sequence.0);

                stat = sequence.0;

                ValidationHelper::validate_statement(stat.get_top_parent());
            }
        }

        if stat.is_sequence() {
            Self::remove_empty_statements(stat.as_sequence());

            if stat.get_stats().borrow().size() == 1 {
                let st = stat.get_first().expect("NullPointerException");

                let mut ok = !st.has_any_successor();
                if !ok {
                    let edge = st.get_first_successor();

                    ok = !stat.has_any_successor();
                    if !ok {
                        let statedge = stat.get_first_successor();
                        ok = edge.get_destination() == statedge.get_destination();

                        if ok {
                            st.remove_successor(edge);
                        }
                    }
                }

                if ok {
                    stat.get_parent()
                        .expect("NullPointerException")
                        .replace_statement(stat, st);
                    stat = st;
                }
            }
        }

        'outer: loop {
            let stats = stat.get_stats().borrow().to_vec();
            for st in stats {
                if (st.get_stats().borrow().is_empty() || st.get_exprents().is_some())
                    && !st.is_basic_block()
                {
                    Self::destroy_and_flatten_statement(st);
                    continue 'outer;
                }
            }
            break;
        }

        let mut i = 0;
        while i < stat.get_stats().borrow().size() {
            let st = stat.get_stats().borrow().get(i);
            Self::condense_sequences_rec(st);
            i += 1;
        }
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
        if sequence.get_stats().borrow().size() <= 1 {
            return;
        }

        Self::merge_flat_statements(sequence);
        ValidationHelper::validate_statement(sequence.get_top_parent());

        loop {
            let mut found = false;

            let stats = sequence.get_stats().borrow().to_vec();
            for st in stats {
                if st.get_exprents().is_some_and(|e| e.is_empty()) {
                    if !st.has_any_successor() {
                        let lst_breaks = st.get_predecessor_edges(StatEdge::TYPE_BREAK);

                        if lst_breaks.is_empty() {
                            for edge in st.get_all_predecessor_edges() {
                                edge.get_source().remove_successor(edge);
                            }
                            found = true;
                        }
                    } else {
                        let sucedge = st.get_first_successor();
                        if sucedge.get_type() != StatEdge::TYPE_FINALLYEXIT {
                            st.remove_successor(sucedge);

                            for edge in st.get_all_predecessor_edges() {
                                if sucedge.get_type() != StatEdge::TYPE_REGULAR {
                                    edge.get_source().change_edge_type(
                                        EdgeDirection::Forward,
                                        edge,
                                        sucedge.get_type(),
                                    );
                                }

                                st.remove_predecessor(edge);
                                edge.get_source().change_edge_node(
                                    EdgeDirection::Forward,
                                    edge,
                                    sucedge.get_destination(),
                                );
                                sucedge.get_destination().add_predecessor(edge);

                                if let Some(closure) = sucedge.closure() {
                                    closure.add_labeled_edge(edge);
                                }
                            }

                            let label_edges = st.get_label_edges().borrow().to_vec();
                            for edge in label_edges {
                                if edge.closure() == Some(st) {
                                    edge.with_mut(|d| d.closure = None);
                                }
                            }

                            found = true;
                        }
                    }

                    if found {
                        sequence.get_stats().borrow_mut().remove_with_key(&st.id());
                        break;
                    }
                }
            }

            if !found {
                break;
            }
        }

        let first = {
            let stats = sequence.get_stats();
            let stats = stats.borrow();
            (!stats.is_empty()).then(|| stats.get(0))
        };
        if let Some(first) = first {
            sequence.set_first(first);
        }
    }

    fn merge_flat_statements(sequence: SequenceStatement) {
        loop {
            let mut current: Option<Statement> = None;

            let mut found = false;

            let mut i = sequence.get_stats().borrow().size() as isize - 1;
            while i >= 0 {
                let next = current;
                let cur = sequence.get_stats().borrow().get(i as usize);
                current = Some(cur);

                if let Some(next) = next {
                    if cur.get_exprents().is_some_and(|e| !e.is_empty()) {
                        let current_exprents = cur.get_exprents().expect("exprents");
                        if let Some(next_exprents) = next.get_exprents() {
                            next_exprents.add_all_at(0, current_exprents.to_vec());
                            current_exprents.clear();
                            found = true;
                        } else if let Some(first) = Self::get_first_exprentlist(next) {
                            let first_exprents = first.get_exprents().expect("exprents");
                            first_exprents.add_all_at(0, current_exprents.to_vec());
                            current_exprents.clear();
                            found = true;
                        }
                    }
                }
                i -= 1;
            }

            if !found {
                break;
            }
        }
    }

    fn get_first_exprentlist(stat: Statement) -> Option<Statement> {
        if stat.get_exprents().is_some() {
            return Some(stat);
        }

        match stat.with(|d| d.type_) {
            StatementType::If
            | StatementType::Sequence
            | StatementType::Switch
            | StatementType::Synchronized => {
                Self::get_first_exprentlist(stat.get_first().expect("NullPointerException"))
            }
            _ => None,
        }
    }

    pub fn destroy_and_flatten_statement(stat: Statement) -> BasicBlockStatement {
        Self::destroy_statement_content(stat, false);

        let bstat = BasicBlockStatement::create();
        match stat.get_exprents() {
            None => bstat.set_exprents(JList::new()),
            Some(exprents) => bstat.set_exprents(JList::from(DecHelper::copy_exprent_list(
                &exprents.to_vec(),
            ))),
        }

        stat.get_parent()
            .expect("NullPointerException")
            .replace_statement(stat, bstat.0);

        bstat
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
            first = first.get_first().expect("NullPointerException");
        }

        first
    }
}
