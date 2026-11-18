//! kuna fix `pattern_variable_scope` (`kuna_fixes`): a pattern variable is in
//! scope only where its `instanceof` is known to have matched (JLS 6.3.1):
//! for `if (x instanceof T v) S else E`, in `S` (and in the rest of the
//! condition); for `if (!(x instanceof T v)) S else E`, in `E`; and when an
//! `if` without `else` whose condition is false on a match cannot complete
//! normally, in the rest of the enclosing block. `IfPatternMatchProcessor`
//! turns `if (x instanceof T) { v = (T) x; ... }` into a pattern after checking
//! only that no predecessor assigns `v`, so a local that the other branch
//! assigns, or that code after the `if` reads (`T v; if (x instanceof T) v =
//! (T) x; else v = new T(); v.use();`), becomes a pattern variable used out of
//! its scope. With the fix the pattern is made only when every other use of
//! the variable lies inside the pattern's scope; otherwise the cast stays.

use crate::exprent::Exprent;
use crate::if_statement::IfStatement;
use crate::statement::{Statement, StatementType};
use crate::var_exprent::VarExprent;

/// Whether `index` is read or written by `exp` or one of its sub-exprents,
/// other than by `except`.
fn uses(exp: Exprent, index: i32, except: Exprent) -> bool {
    let mut all = exp.get_all_exprents_rec(true);
    all.push(exp);
    all.into_iter()
        .any(|e| e != except && e.try_as_var().is_some_and(|v| v.get_index() == index))
}

/// Whether a statement of `stat`'s subtree outside `scope` (and outside the
/// head condition of `if_stat`) uses `index`.
fn used_outside(stat: Statement, scope: &[Statement], if_stat: Statement, index: i32, except: Exprent) -> bool {
    if scope.contains(&stat) {
        return false;
    }
    if stat.get_var_definitions().to_vec().into_iter().any(|e| uses(e, index, except)) {
        return true;
    }
    match stat.get_exprents() {
        Some(exprents) => exprents.to_vec().into_iter().any(|e| uses(e, index, except)),
        None => {
            if stat != if_stat && stat.get_stat_exprents().into_iter().any(|e| uses(e, index, except)) {
                return true;
            }
            let stats = stat.get_stats().borrow().to_vec();
            stats.into_iter().any(|st| used_outside(st, scope, if_stat, index, except))
        }
    }
}

/// Whether the pattern variable `var`, defined by moving its assignment from
/// `branch` into `statement`'s condition, would be used only in its scope:
/// `branch` when it is the `if`'s own branch; the statements after the `if`
/// in its block when `branch` is the `if`'s successor.
pub fn in_scope(statement: IfStatement, branch: Statement, var: VarExprent, root: Statement) -> bool {
    let if_stat = statement.0;
    let scope: Vec<Statement> = if statement.get_ifstat() == Some(branch) || statement.get_elsestat() == Some(branch) {
        vec![branch]
    } else {
        match if_stat.get_parent() {
            Some(parent) if parent.type_() == StatementType::Sequence => {
                let siblings = parent.get_stats().borrow().to_vec();
                let at = siblings.iter().position(|&s| s == if_stat).expect("the if in its sequence");
                siblings[at + 1..].to_vec()
            }
            _ => Vec::new(),
        }
    };
    !used_outside(root, &scope, if_stat, var.get_index(), var.0)
}
