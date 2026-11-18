//! Port of `org/jetbrains/java/decompiler/modules/decompiler/LowBreakHelper.java`.

use crate::stat_edge::StatEdge;
use crate::statement::{Statement, StatementType};

pub struct LowBreakHelper;

impl LowBreakHelper {

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
}
