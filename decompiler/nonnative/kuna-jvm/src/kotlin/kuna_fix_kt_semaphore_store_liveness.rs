//! kuna fix `kt_semaphore_store_liveness` (`kuna_fixes`): `EliminateDeadVarsPass`
//! keeps the stores to a `finally` semaphore. When `FinallyProcessor` cannot
//! deduplicate a `finally`, it guards the handler with a semaphore variable that
//! the catch-all reads as its monitor (`if (varN)` at the finally's entry). That
//! read is a field of the statement, not an exprent, so the SSAU graph the pass
//! queries has no use of the variable, and Vineflower drops every store of
//! `false` as dead: the guard then reads an unassigned variable, the versions of
//! the semaphore are no longer merged into one, and variable definition either
//! fails ("Var types not processed!") or declares the guard after its use.

use crate::rt::JHashSet;
use crate::statement::Statement;
use crate::var_exprent::VarExprent;
use crate::var_processor::VarProcessor;

/// The variables read as the monitor of a `finally` in `stat`'s subtree.
pub fn monitor_vars(stat: Statement) -> JHashSet<i32> {
    let mut vars = JHashSet::new();
    let mut stack = vec![stat];
    while let Some(st) = stack.pop() {
        if let Some(fin) = st.try_as_catch_all() {
            if let Some(monitor) = fin.get_monitor().filter(|_| fin.is_finally()) {
                vars.add(monitor.get_index());
            }
        }
        stack.extend(st.get_stats().borrow().to_vec());
    }
    vars
}

/// Whether `var` is a version of one of `monitors`.
pub fn is_monitor_store(var: VarExprent, monitors: &JHashSet<i32>, varproc: VarProcessor) -> bool {
    let index = var.get_index();
    monitors.contains(&index)
        || varproc
            .get_var_original_index(index)
            .is_some_and(|orig| monitors.contains(&orig))
}
