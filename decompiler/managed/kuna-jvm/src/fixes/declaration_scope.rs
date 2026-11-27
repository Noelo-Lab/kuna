//! Fix family `declaration_scope`: `local_final_writes` makes `setNonFinal`
//! see the writes in statement heads and repeated by loops;
//! `switch_case_scope` keeps a case's declarations live to the end of the
//! switch, visits cases in the order they are written, and braces the cases
//! whose declarations clash instead of renaming.

use crate::decompiler_context::DecompilerContext;
use crate::exprent::Exprent;
use crate::p6_variables::var_definition_helper::{NameMap, VarDefs, VarInMethod};
use crate::language_spec::LanguageSpec;
use crate::rt::JHashSet;
use crate::statement::{Statement, StatementType};
use crate::struct_method::StructMethod;
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

/// Whether `mt`'s class is written as Java: Kotlin's `when` branches are
/// blocks of their own.
fn java(mt: StructMethod) -> bool {
    let class = DecompilerContext::get_struct_context().get_class(&mt.get_class_qualified_name());
    class.is_none_or(|cl| LanguageSpec::of(cl).is_none())
}

/// Whether `stat` is a statement sequence that is a case body of a switch
/// statement (not a switch expression's phantom switch) of Java method `mt`.
pub fn is_case_block(stat: Statement, mt: StructMethod) -> bool {
    if stat.type_() != StatementType::Sequence {
        return false;
    }
    let Some(switch) = stat.get_parent().and_then(|p| p.try_as_switch()) else {
        return false;
    };
    !switch.is_phantom() && switch.get_case_statements().contains(&stat) && java(mt)
}

/// `stats`, the children of `stat`, in the order they are written when `stat`
/// is a switch statement of Java method `mt`: its head, then its cases in case order, then any
/// other child in its place in `stats`.
pub fn in_text_order(stat: Statement, stats: Vec<Statement>, mt: StructMethod) -> Vec<Statement> {
    let Some(switch) = stat.try_as_switch().filter(|s| !s.is_phantom()) else {
        return stats;
    };
    if !java(mt) {
        return stats;
    }
    let cases = switch.get_case_statements().to_vec();
    let mut ordered: Vec<Statement> = stats.iter().copied().filter(|s| !cases.contains(s)).take(1).collect();
    ordered.extend(cases.iter().copied().filter(|c| stats.contains(c)));
    let rest: Vec<Statement> = stats.iter().copied().filter(|s| !ordered.contains(s)).collect();
    ordered.extend(rest);
    ordered
}

/// Moves the declarations recorded for `child` (a basic block at the top
/// level of the case sequence `case`) to `case`, where they stay live.
pub fn keep_in_case(var_definitions: &mut VarDefs, child: Statement, case: Statement) {
    if let Some(defs) = var_definitions.remove(&Some(child)) {
        var_definitions
            .compute_if_absent(Some(case), Default::default)
            .add_all(defs.to_vec());
    }
}

/// Whether `stat`'s subtree declares the variable `pair`.
fn declares(stat: Statement, pair: VarVersionPair) -> bool {
    let is_def = |e: Exprent| {
        let mut all = e.get_all_exprents_rec(true);
        all.push(e);
        all.into_iter()
            .any(|x| x.try_as_var().is_some_and(|v| v.is_definition() && v.get_var_version_pair() == pair))
    };
    if stat.get_var_definitions().to_vec().into_iter().any(is_def) {
        return true;
    }
    match stat.get_exprents() {
        Some(exprents) => exprents.to_vec().into_iter().any(is_def),
        None => {
            stat.get_stat_exprents().into_iter().any(is_def)
                || stat.get_stats().borrow().to_vec().into_iter().any(|st| declares(st, pair))
        }
    }
}

/// For a declaration of `name` in `stat` (of method `mt`) that clashes with live declarations:
/// when `stat` lies in a case of the nearest enclosing switch statement and
/// every live declaration with that name is made in another case of it, puts
/// those cases in braces (and `stat`'s own case too when `stat` is at its top
/// level, `case_level`) and returns true: the name can stay. Vineflower finds
/// the other cases' declarations only in the set recorded for a case
/// statement, which misses those an `if` heading a case hands to the switch.
pub fn scope_clashing_cases(
    stat: Statement,
    mt: StructMethod,
    case_level: bool,
    name: &str,
    name_map: &NameMap,
    live_var_defs: &JHashSet<VarInMethod>,
) -> bool {
    let mut parent = stat.get_parent();
    let switch = loop {
        match parent {
            None => return false,
            Some(p) => match p.try_as_switch() {
                Some(switch) if !switch.is_phantom() => break switch,
                _ => parent = p.get_parent(),
            },
        }
    };
    let cases = switch.get_case_statements().to_vec();
    let Some(own) = cases.iter().copied().find(|c| c.contains_statement(stat)) else {
        return false;
    };
    if !java(mt) {
        return false;
    }
    let clashing: Vec<VarInMethod> = live_var_defs
        .iter()
        .copied()
        .filter(|d| name_map.get(d).and_then(|n| n.as_deref()) == Some(name))
        .collect();
    if clashing.is_empty() {
        return false;
    }
    let mut owners: Vec<Statement> = Vec::new();
    for def in &clashing {
        if def.mt() != mt {
            return false;
        }
        match cases.iter().copied().find(|&c| c != own && declares(c, def.pair())) {
            Some(owner) => owners.push(owner),
            None => return false,
        }
    }
    for owner in owners {
        switch.scope_case_statement(owner);
    }
    if case_level {
        switch.scope_case_statement(own);
    }
    true
}
