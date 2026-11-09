//! kuna fix `kt_for_range_counter` (`kuna_fixes`): `KDoStatement` rewrites a
//! Java `for (V = A; L < R; V++/--)` into `for (V in A..B)` or `repeat(N)` only
//! when the loop really counts `V` from `A` to the bound: the update and the
//! comparison name `V` (on the side the update moves it towards the bound),
//! the bound is a constant or a variable, the body assigns neither `V` nor
//! the bound, and the initializer declares `V` (so nothing after the loop
//! reads it). Vineflower takes the initializer's variable for the loop
//! variable without checking any of this.

use crate::do_statement::DoStatement;
use crate::exprent::Exprent;
use crate::function_exprent::FunctionExprent;
use crate::statement::Statement;
use crate::var_exprent::VarExprent;

/// Whether `stat`'s `for` header counts `var` (the initializer's left side)
/// with `inc` against `condition` (a `<`), as a Kotlin range loop does.
pub fn is_counted_loop(
    stat: DoStatement,
    var: VarExprent,
    inc: FunctionExprent,
    condition: FunctionExprent,
) -> bool {
    if !var.is_definition() {
        return false;
    }
    let counted = inc.get_lst_operands().get(0);
    if !counted.try_as_var().is_some_and(|v| v.equals(var.0)) {
        return false;
    }
    let operands = condition.get_lst_operands();
    let (counter, bound) = if inc.get_func_type().is_pp() {
        (operands.get(0), operands.get(1))
    } else {
        (operands.get(1), operands.get(0))
    };
    if !counter.try_as_var().is_some_and(|v| v.equals(var.0)) {
        return false;
    }
    let bound_var = match bound.try_as_var() {
        Some(b) if b.equals(var.0) => return false,
        Some(b) => Some(b),
        None if bound.is_const() => None,
        None => return false,
    };
    let Some(body) = stat.get_first() else {
        return false;
    };
    !assigns(body, var) && !bound_var.is_some_and(|b| assigns(body, b))
}

/// Whether any exprent of `stat`'s tree assigns `var` (`=`, a compound
/// assignment, `++` or `--`).
fn assigns(stat: Statement, var: VarExprent) -> bool {
    let own = match stat.get_exprents() {
        Some(exprents) => exprents.to_vec(),
        None => stat.get_stat_exprents(),
    };
    if own.into_iter().any(|e| assigns_in(e, var)) {
        return true;
    }
    let stats = stat.get_stats().borrow().to_vec();
    stats.into_iter().any(|s| assigns(s, var))
}

fn assigns_in(exprent: Exprent, var: VarExprent) -> bool {
    exprent
        .get_all_exprents_rec_self(true, true)
        .into_iter()
        .any(|e| {
            let target = if let Some(assign) = e.try_as_assignment() {
                Some(assign.get_left())
            } else if let Some(func) = e.try_as_function() {
                func.get_func_type()
                    .is_ppmm()
                    .then(|| func.get_lst_operands().get(0))
            } else {
                None
            };
            target
                .and_then(|t| t.try_as_var())
                .is_some_and(|t| t.equals_versions(var.0))
        })
}
