//! Port of `org/jetbrains/java/decompiler/modules/decompiler/EliminateLoopsHelper.java`.

use crate::do_statement::{DoStatement, Type as LoopType};
use crate::low_break_helper::LowBreakHelper;
use crate::rt::{JHashMap, JHashSet, JavaVecExt};
use crate::sequence_helper::SequenceHelper;
use crate::simplify_exprents_helper::SimplifyExprentsHelper;
use crate::stat_edge::StatEdge;
use crate::statement::Statement;
use crate::struct_class::StructClass;
use crate::try_with_resources_processor::TryWithResourcesProcessor;

pub struct EliminateLoopsHelper;

impl EliminateLoopsHelper {
    pub fn eliminate_loops(root: Statement, cl: StructClass) -> bool {
        let ret = Self::eliminate_loops_rec(root);

        if ret {
            SequenceHelper::condense_sequences(root);

            let mut set_reordered_ifs: JHashSet<i32> = JHashSet::new();

            while SimplifyExprentsHelper::simplify_stack_vars_statement(
                root,
                &mut set_reordered_ifs,
                None,
                cl,
                false,
            ) {
                SequenceHelper::condense_sequences(root);
            }
        }

        ret
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
        if loop_.get_looptype() != LoopType::Infinite {
            return false;
        }

        let mut parentloop = loop_.get_parent();
        while let Some(p) = parentloop {
            if p.is_do() {
                break;
            }
            parentloop = p.get_parent();
        }

        let Some(parentloop) = parentloop else {
            return false;
        };
        if parentloop.get_basichead() != loop_.get_basichead() {
            return false;
        }

        let mut lst_break_edges: Vec<StatEdge> = Vec::new();
        let label_edges = loop_.get_label_edges().borrow().to_vec();
        for edge in label_edges {
            if edge.get_type() == StatEdge::TYPE_BREAK {
                lst_break_edges.push(edge);
            }
        }

        let loopcontent = loop_.get_first().expect("NullPointerException");

        let mut firstok = !loopcontent.has_any_successor();
        if !firstok {
            let edge = loopcontent.get_first_successor();
            firstok = edge.closure() == Some(loop_.0) && edge.get_type() == StatEdge::TYPE_BREAK;
            if firstok {
                lst_break_edges.j_remove(&edge);
            }
        }

        if !lst_break_edges.is_empty() {
            if firstok {
                let mut stat_labeled: JHashMap<i32, bool> = JHashMap::new();
                let mut lst_edge_closures: Vec<Statement> = Vec::new();

                for edge in &lst_break_edges {
                    let minclosure =
                        LowBreakHelper::get_min_closure(loopcontent, edge.get_source());
                    lst_edge_closures.push(minclosure);
                }

                let mut precount = if loop_.is_labeled() { 1 } else { 0 };
                for st in &lst_edge_closures {
                    if !stat_labeled.contains_key(&st.id()) {
                        let btemp = st.is_labeled();
                        precount += if btemp { 1 } else { 0 };
                        stat_labeled.put(st.id(), btemp);
                    }
                }

                for i in 0..lst_break_edges.len() {
                    let st = lst_edge_closures[i];
                    let labeled =
                        LowBreakHelper::is_break_edge_labeled(lst_break_edges[i].get_source(), st)
                            | *stat_labeled.get(&st.id()).expect("NullPointerException");
                    stat_labeled.put(st.id(), labeled);
                }

                for i in 0..lst_break_edges.len() {
                    lst_edge_closures[i] = Self::get_max_break_lift(
                        lst_edge_closures[i],
                        lst_break_edges[i],
                        &stat_labeled,
                        loop_.0,
                    );
                }

                stat_labeled.clear();
                for st in &lst_edge_closures {
                    stat_labeled.put(st.id(), st.is_labeled());
                }

                for i in 0..lst_break_edges.len() {
                    let st = lst_edge_closures[i];
                    let labeled =
                        LowBreakHelper::is_break_edge_labeled(lst_break_edges[i].get_source(), st)
                            | *stat_labeled.get(&st.id()).expect("NullPointerException");
                    stat_labeled.put(st.id(), labeled);
                }

                let postcount = stat_labeled.values().filter(|&&b| b).count() as i64;

                if precount <= postcount {
                    return false;
                } else {
                    for i in 0..lst_break_edges.len() {
                        lst_edge_closures[i].add_labeled_edge(lst_break_edges[i]);
                    }
                }
            } else {
                return false;
            }
        }

        let mut continues: JHashSet<StatEdge> = JHashSet::new();
        TryWithResourcesProcessor::find_edges_leaving(loopcontent, loop_.0, &mut continues);

        let edges: Vec<StatEdge> = continues
            .iter()
            .copied()
            .filter(|edge| edge.get_type() == StatEdge::TYPE_CONTINUE)
            .collect();

        if !edges.is_empty() {
            for edge in &edges {
                if edge.get_destination() == parentloop {
                    return false;
                }
            }
        }

        Self::eliminate_loop(loop_.0, parentloop);

        true
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
        let mut closure = stat.get_parent();

        while let Some(c) = closure {
            if c == max || c.contains_statement_strict(edge.get_destination()) {
                break;
            }

            let edge_labeled = LowBreakHelper::is_break_edge_labeled(edge.get_source(), c);
            let stat_labeled = match stat_labeled.get(&c.id()) {
                Some(&labeled) => labeled,
                None => c.is_labeled(),
            };

            if stat_labeled || !edge_labeled {
                return Some(c);
            }

            closure = c.get_parent();
        }

        None
    }

    fn eliminate_loop(loop_: Statement, parentloop: Statement) {
        let loopcontent = loop_.get_first().expect("NullPointerException");
        if loopcontent.has_successor(StatEdge::TYPE_BREAK) {
            let edge = loopcontent.get_successor_edges(StatEdge::TYPE_BREAK)[0];

            if edge.get_destination().is_if() {
                edge.change_closure(Some(parentloop));
            } else {
                loopcontent.remove_successor(edge);
            }
        }

        let lst = loop_.get_label_edges().borrow().to_vec();

        for edge in lst {
            edge.change_destination(parentloop);

            parentloop.add_labeled_edge(edge);
        }

        loop_.replace_with(loopcontent);
    }
}
