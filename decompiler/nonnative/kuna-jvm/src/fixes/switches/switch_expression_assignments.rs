//! kuna fix `switch_expression_assignments` (`kuna_fixes`):
//! `SwitchExpressionHelper` turns a switch statement into `var = switch
//! (...) { ... }` when the statements that break out of its cases end by
//! assigning `var` (or throw), then replaces every assignment to `var` in
//! the cases by a `yield`. It skips the break sources that are not basic
//! blocks (an `if` or a loop whose condition leaves the case) and never asks
//! whether an assignment it replaces is the last thing done before a break:
//! `if (round) { t = t.truncate(); if (up) { t = t.plus(1); } }` becomes
//! `yield t.truncate(); if (up) { yield t.plus(1); }`, which is unreachable
//! code, and a case left by a failed condition yields nothing. With the fix
//! the switch becomes an expression only when every way out of a case is a
//! block ending in an assignment to the variable (or a throw), not an empty
//! block, an `if` without else or a conditional loop, and every assignment to
//! the variable in the cases is one of those.

use crate::do_statement;
use crate::exit_exprent::Type as ExitType;
use crate::exprent::Exprent;
use crate::if_statement::IfStatement;
use crate::statement::Statement;
use crate::switch_statement::SwitchStatement;

/// Whether each assignment to the variable of index `var` in `stat`'s cases
/// is the last exprent of one of `break_jumps`, and each of those that is
/// reached by completing normally ends in such an assignment or a throw.
pub fn every_assignment_yields(stat: SwitchStatement, break_jumps: &[Statement], var: i32) -> bool {
    let mut finals: Vec<Exprent> = Vec::new();
    for jump in break_jumps {
        let Some(exprents) = jump.get_exprents() else {
            if falls_out(*jump) {
                return false;
            }
            continue;
        };
        let Some(last) = exprents.to_vec().last().copied() else {
            return false;
        };
        if last.try_as_exit().is_some_and(|exit| exit.get_exit_type() == ExitType::Throw) {
            continue;
        }
        if !assigns(last, var) {
            return false;
        }
        finals.push(last);
    }
    stat.get_case_statements()
        .iter()
        .all(|case| assignments_in(case, var, &finals))
}

/// Whether the break edge of the compound statement `jump` is taken when
/// it completes by itself: an `if` without else whose condition is false,
/// a loop whose condition fails. (Any other compound statement completes
/// through the edges of the statements inside it.)
fn falls_out(jump: Statement) -> bool {
    jump.try_as_if().is_some_and(|i| i.iftype() == IfStatement::IFTYPE_IF)
        || jump.try_as_do().is_some_and(|d| d.get_looptype() != do_statement::Type::Infinite)
}

fn assigns(exprent: Exprent, var: i32) -> bool {
    exprent
        .try_as_assignment()
        .and_then(|assign| assign.get_left().try_as_var())
        .is_some_and(|left| left.get_index() == var)
}

/// Whether every assignment to `var` among the exprents `findReplacements`
/// visits under `stat` is in `finals`.
fn assignments_in(stat: Statement, var: i32, finals: &[Exprent]) -> bool {
    if let Some(exprents) = stat.get_exprents() {
        if exprents.iter().any(|e| assigns(e, var) && !finals.contains(&e)) {
            return false;
        }
    }
    stat.get_stats()
        .borrow()
        .to_vec()
        .into_iter()
        .all(|st| assignments_in(st, var, finals))
}
