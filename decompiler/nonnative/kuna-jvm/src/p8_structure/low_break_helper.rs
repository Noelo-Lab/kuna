//! Port of `org/jetbrains/java/decompiler/modules/decompiler/LowBreakHelper.java`.

use crate::stat_edge::StatEdge;
use crate::statement::{Statement, StatementType};

pub struct LowBreakHelper;

impl LowBreakHelper {
    pub fn low_break_labels(root: Statement) {
        Self::low_break_labels_rec(root);

        Self::lift_break_labels(root);
    }

    fn low_break_labels_rec(stat: Statement) {
        loop {
            let mut found = false;

            let label_edges = stat.get_label_edges().borrow().to_vec();
            for edge in label_edges {
                if edge.get_type() == StatEdge::TYPE_BREAK {
                    let minclosure = Self::get_min_closure(stat, edge.get_source());
                    if minclosure != stat {
                        minclosure.add_labeled_edge(edge);
                        let labeled = Self::is_break_edge_labeled(edge.get_source(), minclosure);
                        edge.with_mut(|d| d.labeled = labeled);
                        found = true;
                        break;
                    }
                }
            }

            if !found {
                break;
            }
        }

        let stats = stat.get_stats().borrow().to_vec();
        for st in stats {
            Self::low_break_labels_rec(st);
        }
    }

    pub fn is_break_edge_labeled(source: Statement, closure: Statement) -> bool {
        if closure.is_do() || closure.is_switch() {
            let parent = source.get_parent().unwrap();

            if parent == closure {
                false
            } else {
                Self::is_break_edge_labeled(parent, closure)
                    || (parent.is_do() || parent.is_switch())
            }
        } else {
            true
        }
    }

    pub fn get_min_closure(closure: Statement, source: Statement) -> Statement {
        let mut closure = closure;
        loop {
            let mut newclosure: Option<Statement> = None;

            match closure.with(|d| d.type_) {
                StatementType::Sequence => {
                    let last = closure.get_stats().borrow().get_last();

                    if Self::is_ok_closure(closure, source, Some(last)) {
                        newclosure = Some(last);
                    }
                }
                StatementType::If => {
                    let ifclosure = closure.as_if();
                    if Self::is_ok_closure(closure, source, ifclosure.get_ifstat()) {
                        newclosure = ifclosure.get_ifstat();
                    } else if Self::is_ok_closure(closure, source, ifclosure.get_elsestat()) {
                        newclosure = ifclosure.get_elsestat();
                    }
                }
                StatementType::TryCatch => {
                    let stats = closure.get_stats().borrow().to_vec();
                    for st in stats {
                        if Self::is_ok_closure(closure, source, Some(st)) {
                            newclosure = Some(st);
                            break;
                        }
                    }
                }
                StatementType::Synchronized => {
                    let body = closure.as_synchronized().get_body();

                    if Self::is_ok_closure(closure, source, Some(body)) {
                        newclosure = Some(body);
                    }
                }
                _ => {}
            }

            match newclosure {
                None => break,
                Some(newclosure) => closure = newclosure,
            }
        }

        closure
    }

    fn is_ok_closure(closure: Statement, source: Statement, stat: Option<Statement>) -> bool {
        let mut ok = false;

        if let Some(stat) = stat.filter(|s| s.contains_statement_strict(source)) {
            let lst = stat.get_all_successor_edges();

            ok = lst.is_empty();
            if !ok {
                let edge = lst[0];
                ok = edge.with(|d| d.closure) == Some(closure)
                    && edge.get_type() == StatEdge::TYPE_BREAK;
            }
        }

        ok
    }

    fn lift_break_labels(stat: Statement) {
        let stats = stat.get_stats().borrow().to_vec();
        for st in stats {
            Self::lift_break_labels(st);
        }

        loop {
            let mut found = false;

            let label_edges = stat.get_label_edges().borrow().to_vec();
            for edge in label_edges {
                let (explicit, labeled) = edge.with(|d| (d.explicit, d.labeled));
                if explicit && labeled && edge.get_type() == StatEdge::TYPE_BREAK {
                    let newclosure = Self::get_max_break_lift(stat, edge);

                    if let Some(newclosure) = newclosure {
                        newclosure.add_labeled_edge(edge);
                        let labeled = Self::is_break_edge_labeled(edge.get_source(), newclosure);
                        edge.with_mut(|d| d.labeled = labeled);

                        found = true;
                        break;
                    }
                }
            }

            if !found {
                break;
            }
        }
    }

    fn get_max_break_lift(stat: Statement, edge: StatEdge) -> Option<Statement> {
        let mut closure: Option<Statement> = None;
        let mut newclosure = stat;

        while let Some(next) = Self::get_next_break_lift(newclosure, edge) {
            newclosure = next;
            closure = Some(newclosure);
        }

        closure
    }

    fn get_next_break_lift(stat: Statement, edge: StatEdge) -> Option<Statement> {
        let mut closure = stat.get_parent();

        while let Some(c) = closure.filter(|c| !c.contains_statement_strict(edge.get_destination()))
        {
            let labeled = Self::is_break_edge_labeled(edge.get_source(), c);
            if c.is_labeled() || !labeled {
                return Some(c);
            }

            closure = c.get_parent();
        }

        None
    }
}
