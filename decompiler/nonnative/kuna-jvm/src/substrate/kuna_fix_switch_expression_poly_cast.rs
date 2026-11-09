//! kuna fix `switch_expression_poly_cast` (`kuna_fixes`): a switch
//! expression whose results javac would promote is not cast to its target type.
//!
//! `ExprProcessor.getCastedExprent` casts a value whose type is not known to
//! fit the target; a switch expression's type is the join of its results,
//! `Object` when their common superclass is not loaded. A switch expression
//! in an assignment or invocation context is a poly expression: each result
//! is converted to the target type on its own (JLS 15.28.1). Inside a cast it
//! is standalone, and results that are all numeric are promoted together:
//! `(Number)(switch (x) { case 1 -> 1; default -> 2L; })` yields `Long` 1 for
//! the bytecode's `Integer` 1. With the fix such a switch is written without
//! the cast; each result already fits the target, which the bytecode assigns
//! from every branch.

use crate::exprent::Exprent;
use crate::statement::Statement;
use crate::substrate::kuna_fix_ternary_numeric_arms::{printed_type, promotes};

/// Whether `expr` is a switch expression whose results a standalone switch
/// would unbox or promote together.
pub fn results_promote(expr: Exprent) -> bool {
    let Some(switch) = expr.try_as_switch() else {
        return false;
    };
    let mut results = Vec::new();
    for stat in switch.get_backing().get_case_statements().iter() {
        collect_yields(stat, &mut results);
    }
    let types: Vec<_> = results.into_iter().map(printed_type).collect();
    promotes(&types)
}

fn collect_yields(stat: Statement, out: &mut Vec<Exprent>) {
    match stat.get_exprents() {
        Some(exprents) => {
            for e in exprents.iter() {
                if let Some(y) = e.try_as_yield() {
                    out.push(y.get_content());
                }
            }
        }
        None => {
            for st in stat.get_stats().borrow().iter() {
                collect_yields(*st, out);
            }
        }
    }
}
