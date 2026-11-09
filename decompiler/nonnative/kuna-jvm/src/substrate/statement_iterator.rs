//! Port of `org/jetbrains/java/decompiler/util/StatementIterator.java`.
//!
//! The `DirectGraph.ExprentIterator` callback is any `FnMut(Exprent) -> i32`.

use crate::exprent::Exprent;
use crate::statement::Statement;

pub struct StatementIterator;

impl StatementIterator {
    pub fn iterate(stat: Statement, itr: &mut impl FnMut(Exprent) -> i32) {
        for exp in stat.get_var_definitions().iter() {
            StatementIterator::iterate_exprent(exp, itr);
        }

        match stat.get_exprents() {
            None => {
                let stats = stat.get_stats().borrow().to_vec();
                for st in stats {
                    StatementIterator::iterate(st, itr);
                }

                for expr in stat.get_stat_exprents() {
                    StatementIterator::iterate_exprent(expr, itr);
                }
            }
            Some(exprents) => {
                for exp in exprents.iter() {
                    StatementIterator::iterate_exprent(exp, itr);
                }
            }
        }
    }

    /// Java: iterate(Exprent, ExprentIterator)
    fn iterate_exprent(exp: Exprent, itr: &mut impl FnMut(Exprent) -> i32) {
        let mut lst = exp.get_all_exprents_rec(true);
        lst.push(exp);
        for exprent in lst {
            itr(exprent);
        }
    }
}
