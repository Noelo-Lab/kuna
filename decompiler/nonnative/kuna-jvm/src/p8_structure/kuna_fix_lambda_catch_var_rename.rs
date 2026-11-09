//! kuna fix `lambda_catch_var_rename` (`kuna_fixes`): `NestedClassProcessor`
//! renames a lambda body's local that clashes with a name of the enclosing
//! method (`e` becomes `ex`) by giving it a renamed local-variable-table entry,
//! and hands that entry to every `VarExprent` of the body's `DirectGraph`. A
//! catch clause's variable is held by its `CatchStatement` (or
//! `CatchAllStatement`) and has no node in the graph, so it keeps the old
//! entry: the clause declares `e` while every use reads `ex`, and javac cannot
//! resolve `ex`. With the fix the catch variables of the body's statement tree
//! take the same entry as the variable's uses.

use crate::struct_local_variable_table_attribute::LocalVariable;
use crate::rt::JHashMap;
use crate::statement::Statement;
use crate::var_exprent::VarExprent;
use crate::var_version_pair::VarVersionPair;

/// Applies the lambda body's renamed entries to the catch variables under
/// `stat`, as the rename applies them to the variables of the body's graph.
pub fn rename_catch_vars(
    stat: Statement,
    lvts: &JHashMap<VarVersionPair, Option<LocalVariable>>,
    renamed: &dyn Fn(VarVersionPair) -> bool,
) {
    let vars: Vec<VarExprent> = if let Some(catch) = stat.try_as_catch() {
        catch.get_vars().to_vec()
    } else if let Some(catch_all) = stat.try_as_catch_all() {
        catch_all.get_vars().to_vec()
    } else {
        Vec::new()
    };
    for var in vars {
        let pair = var.get_var_version_pair();
        if let Some(lv) = lvts.get(&pair).cloned().flatten() {
            var.set_lvt(Some(lv));
        } else if renamed(pair) {
            var.set_lvt(None);
        }
    }
    for child in stat.get_stats().borrow().to_vec() {
        rename_catch_vars(child, lvts, renamed);
    }
}
