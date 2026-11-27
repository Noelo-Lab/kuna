//! Fix `kt_semaphore_store_liveness`: `EliminateDeadVarsPass` keeps the stores
//! to a `finally` semaphore, whose read is a field of the catch-all statement.

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
