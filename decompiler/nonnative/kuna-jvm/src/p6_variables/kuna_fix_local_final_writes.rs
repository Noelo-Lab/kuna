//! kuna fix `local_final_writes` (spec 10.16): `setNonFinal` sees every write
//! of a local.
//!
//! `VarDefinitionHelper.setNonFinal` decides which locals are written once: a
//! declaration without a value enters a set, the first write takes it out and
//! any other write makes the local non-final. It reads statement bodies only,
//! so a write in a head (`if (++n > max)`, a loop condition, a `switch` or
//! `synchronized` subject) was never seen, and it shares one set between a
//! loop and the code around it, so the one write in a loop body of a local
//! declared before the loop counted as the only one although the loop repeats
//! it. Java output never prints this finality; Kotlin's prints it as `val`,
//! which kotlinc rejects as reassigned. Here heads are read where they run and
//! a loop body starts with a set of its own: a local declared outside the loop
//! and written inside it is non-final.

use crate::rt::JHashSet;
use crate::statement::Statement;
use crate::var_definition_helper::VarDefinitionHelper;
use crate::var_version_pair::VarVersionPair;

/// `setNonFinal(Statement, Set)` with heads and loops.
pub fn set_non_final(
    helper: VarDefinitionHelper,
    stat: Statement,
    un_initialized: &mut JHashSet<VarVersionPair>,
) {
    if let Some(exprents) = stat.get_exprents().filter(|e| !e.is_empty()) {
        for exp in exprents.iter() {
            if let Some(var) = exp.try_as_var() {
                un_initialized.add(VarVersionPair::from_var(var));
            } else {
                helper.set_non_final_exprent(Some(exp), un_initialized);
            }
        }
    }

    let stats = stat.get_stats().borrow().to_vec();
    if let Some(dostat) = stat.try_as_do() {
        for var in stat.get_var_definitions().iter() {
            un_initialized.add(VarVersionPair::from_var(var.as_var()));
        }
        helper.set_non_final_exprent(dostat.get_init_exprent(), un_initialized);
        let mut body: JHashSet<VarVersionPair> = JHashSet::new();
        helper.set_non_final_exprent(dostat.get_condition_exprent(), &mut body);
        helper.set_non_final_exprent(dostat.get_inc_exprent(), &mut body);
        for st in stats {
            set_non_final(helper, st, &mut body);
        }
        return;
    }

    let heads = if let Some(ifstat) = stat.try_as_if() {
        Some(ifstat.get_headexprent_list())
    } else if let Some(switch) = stat.try_as_switch() {
        Some(switch.get_headexprent_list())
    } else {
        stat.try_as_synchronized().map(|sync| sync.get_headexprent_list())
    };
    let Some(heads) = heads else {
        for st in stats {
            set_non_final(helper, st, un_initialized);
        }
        return;
    };

    let first = stat.get_first();
    if let Some(first) = first {
        set_non_final(helper, first, un_initialized);
    }
    for head in heads.iter() {
        helper.set_non_final_exprent(Some(head), un_initialized);
    }
    let rest: Vec<Statement> = stats.into_iter().filter(|st| Some(*st) != first).collect();
    if let Some(ifstat) = stat.try_as_if() {
        if let (Some(if_branch), Some(else_branch)) = (ifstat.get_ifstat(), ifstat.get_elsestat()) {
            set_non_final(helper, if_branch, &mut un_initialized.clone());
            set_non_final(helper, else_branch, un_initialized);
            return;
        }
    }
    for st in rest {
        set_non_final(helper, st, un_initialized);
    }
}
