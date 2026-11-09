//! Port of `org/jetbrains/java/decompiler/modules/decompiler/LabelHelper.java`.
#![allow(unused_variables, dead_code)]

use crate::do_statement::Type as DoType;
use crate::merge_helper::MergeHelper;
use crate::root_statement::RootStatement;
use crate::rt::{JHashMap, JHashSet, JLinkedHashSet};
use crate::stat_edge::StatEdge;
use crate::statement::{EdgeDirection, Statement, StatementType};
use crate::unported;

pub struct LabelHelper;

struct LabelSets {
    breaks: JHashSet<Statement>,
    continues: JHashSet<Statement>,
}

impl LabelSets {
    fn new() -> LabelSets {
        LabelSets {
            breaks: JHashSet::new(),
            continues: JHashSet::new(),
        }
    }
}

impl LabelHelper {
    pub fn clean_up_edges(root: RootStatement) {
        Self::reset_all_edges(root.0);

        Self::remove_non_immediate_edges(root.0);

        Self::lift_closures(root.0);

        Self::low_continue_labels(root.0, &mut JLinkedHashSet::new());

        Self::low_closures(root.0);
    }

    pub fn identify_labels(root: RootStatement) {
        Self::set_explicit_edges(root.0);

        Self::process_statement_label(root.0);

        Self::set_ret_edges_unlabeled(root);

        Self::lift_sequence_label(root.0);
    }

    fn lift_closures(stat: Statement) {
        unported!("LabelHelper.liftClosures")
    }

    fn remove_non_immediate_edges(stat: Statement) {
        let stats = stat.get_stats().borrow().to_vec();
        for st in stats {
            Self::remove_non_immediate_edges(st);
        }

        if !stat.has_basic_succ_edge() {
            for edge in stat.get_successor_edges(StatEdge::TYPE_CONTINUE | StatEdge::TYPE_BREAK) {
                stat.remove_successor(edge);
            }
        }
    }

    /// Java: lowContinueLabels(Statement, HashSet). Every caller passes a
    /// `LinkedHashSet`, whose insertion order the edge loop depends on.
    pub fn low_continue_labels(stat: Statement, edges: &mut JLinkedHashSet<StatEdge>) {
        let mut ok = !stat.is_do();
        if !ok {
            let dostat = stat.as_do();
            ok = dostat.get_looptype() == DoType::Infinite
                || dostat.get_looptype() == DoType::While
                    && dostat.get_condition_exprent().is_none()
                || (dostat.get_looptype() == DoType::For && dostat.get_inc_exprent().is_none());
        }

        if ok {
            edges.add_all(stat.get_predecessor_edges(StatEdge::TYPE_CONTINUE));
        }

        if ok && stat.is_do() {
            for edge in edges.to_vec() {
                if stat.contains_statement_strict(edge.get_source()) {
                    edge.get_destination().remove_predecessor(edge);
                    edge.get_source()
                        .change_edge_node(EdgeDirection::Forward, edge, stat);
                    stat.add_predecessor(edge);

                    stat.add_labeled_edge(edge);
                }
            }
        }

        let stats = stat.get_stats().borrow().to_vec();
        for st in stats {
            if Some(st) == stat.get_first() {
                Self::low_continue_labels(st, edges);
            } else {
                Self::low_continue_labels(st, &mut JLinkedHashSet::new());
            }
        }
    }

    pub fn low_closures(stat: Statement) {
        let label_edges = stat.get_label_edges().borrow().to_vec();
        for edge in label_edges {
            if edge.get_type() == StatEdge::TYPE_BREAK {
                let stats = stat.get_stats().borrow().to_vec();
                for st in stats {
                    if st.contains_statement_strict(edge.get_source())
                        && MergeHelper::is_direct_path(st, edge.get_destination())
                    {
                        st.add_labeled_edge(edge);
                    }
                }
            }
        }

        let stats = stat.get_stats().borrow().to_vec();
        for st in stats {
            Self::low_closures(st);
        }
    }

    fn lift_sequence_label(stat: Statement) {
        if let Some(if_stat) = stat.get_parent().and_then(|p| p.try_as_if()) {
            if if_stat.get_ifstat() == Some(stat) && stat.is_sequence() {
                let mut edges: JHashSet<StatEdge> = JHashSet::new();

                let label_edges = stat.get_label_edges().borrow().to_vec();
                for edge in label_edges {
                    if MergeHelper::is_direct_path(if_stat.0, edge.get_destination()) {
                        edges.add(edge);
                    }
                }

                if edges.len() == stat.get_label_edges().borrow().len() {
                    let copy = stat.get_label_edges().borrow().clone();
                    for edge in copy.to_vec() {
                        if_stat.add_labeled_edge(edge);
                    }
                }
            }
        }

        let stats = stat.get_stats().borrow().to_vec();
        for st in stats {
            Self::lift_sequence_label(st);
        }
    }

    fn reset_all_edges(stat: Statement) {
        let stats = stat.get_stats().borrow().to_vec();
        for st in stats {
            Self::reset_all_edges(st);
        }

        for edge in stat.get_all_successor_edges() {
            edge.with_mut(|d| {
                d.explicit = true;
                d.labeled = true;
            });
        }
    }

    fn set_ret_edges_unlabeled(root: RootStatement) {
        let exit = root.get_dummy_exit().0;
        for edge in exit.get_all_predecessor_edges() {
            let lst = edge.get_source().get_exprents();
            if edge.get_type() == StatEdge::TYPE_FINALLYEXIT
                || lst
                    .filter(|l| !l.is_empty())
                    .is_some_and(|l| l.get(l.len() - 1).is_exit())
            {
                edge.with_mut(|d| d.labeled = false);
            }
        }
    }

    fn set_explicit_edges(stat: Statement) -> JHashMap<Statement, Vec<StatEdge>> {
        unported!("LabelHelper.setExplicitEdges")
    }

    fn process_edges_with_next(
        stat: Statement,
        map_edges: &mut JHashMap<Statement, Vec<StatEdge>>,
        next: Option<Statement>,
    ) {
        unported!("LabelHelper.processEdgesWithNext")
    }

    pub fn hide_default_switch_edges(stat: Statement) -> bool {
        unported!("LabelHelper.hideDefaultSwitchEdges")
    }

    fn process_statement_label(stat: Statement) -> LabelSets {
        let mut sets = LabelSets::new();

        if stat.get_exprents().is_none() {
            let stats = stat.get_stats().borrow().to_vec();
            for st in stats {
                let nested = Self::process_statement_label(st);
                sets.breaks.add_all(nested.breaks.to_vec());
                sets.continues.add_all(nested.continues.to_vec());
            }

            let shield_type = stat.is_do() || stat.is_switch();
            if shield_type {
                let label_edges = stat.get_label_edges().borrow().to_vec();
                for edge in label_edges {
                    if edge.with(|d| d.explicit)
                        && ((edge.get_type() == StatEdge::TYPE_BREAK
                            && sets.breaks.contains(&edge.get_source()))
                            || (edge.get_type() == StatEdge::TYPE_CONTINUE
                                && sets.continues.contains(&edge.get_source())))
                    {
                        edge.with_mut(|d| d.labeled = false);
                    }
                }
            }

            match stat.with(|d| d.type_) {
                StatementType::Do => {
                    sets.continues.clear();
                    sets.breaks.clear();
                }
                StatementType::Switch => {
                    sets.breaks.clear();
                }
                _ => {}
            }
        }

        sets.breaks.add(stat);
        sets.continues.add(stat);

        sets
    }

    pub fn replace_continue_with_break(stat: Statement) -> bool {
        unported!("LabelHelper.replaceContinueWithBreak")
    }

    fn find_max_jump(enclosing: Statement) -> Statement {
        let mut enclosing = enclosing;
        let mut last = enclosing;
        while enclosing.is_do()
            && !enclosing
                .get_successor_edges(StatEdge::TYPE_BREAK)
                .is_empty()
        {
            last = enclosing;
            enclosing = Self::get_enclosing_shield_type(enclosing.get_parent().unwrap());
        }

        last
    }

    fn get_enclosing_shield_type(stat: Statement) -> Statement {
        let mut res = stat;

        while !res.is_switch() && !res.is_do() {
            match res.get_parent() {
                None => break,
                Some(parent) => res = parent,
            }
        }

        res
    }

    fn get_min_continue_closure(edge: StatEdge) -> Statement {
        let mut closure = edge.with(|d| d.closure).unwrap();
        loop {
            let mut found = false;

            let stats = closure.get_stats().borrow().to_vec();
            for st in stats {
                if st.contains_statement_strict(edge.get_source())
                    && MergeHelper::is_direct_path(st, edge.get_destination())
                {
                    closure = st;
                    found = true;
                    break;
                }
            }

            if !found {
                break;
            }
        }

        closure
    }
}
