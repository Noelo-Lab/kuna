//! kuna fix `kt_when_break` (`kuna_fixes`): an unlabelled `break` out of a
//! `switch`, which Kotlin output writes as a `when`, is not written as
//! `break`.
//!
//! In Java an unlabelled `break` inside a `switch` leaves the `switch`; a
//! Kotlin `when` branch never falls through and has no `break` of its own:
//! `break` in a `when` is rejected outside a loop and leaves the enclosing
//! loop inside one. `KSwitchStatement` drops the `break` that follows a case
//! statement, but a `break` that ends a statement inside the case (the last
//! statement of the case's sequence, an `if` branch, a `try` block) is
//! written by `KExprProcessor.jmpWrapper` or `KIfStatement` as `break`. kuna
//! leaves such a `break` out where the branch ends after it anyway: the
//! statement it follows is the last of each enclosing sequence, an `if`
//! branch, a `try` or `catch` block or the `try` block of a `try`/`finally`,
//! up to the case, and none of those statements writes a jump of its own.
//! Any other `break` out of the `when` is `return@labelN` from a
//! `run labelN@{ .. }` written around the `when` (`run` is inline; the
//! lambda's end is the `when`'s end).

use crate::kuna_fix_kt_jump_to_non_loop_label::{self, Lambda};
use crate::stat_edge::StatEdge;
use crate::statement::{Statement, StatementType};

/// How the jump along `edge`, written after `site`, is written when it is an
/// unlabelled `break` out of a `when`: `Some("")` when it is left out,
/// `Some("return@labelN")` when the `when` is written in `run labelN@{ .. }`.
pub fn jump(edge: StatEdge, site: Statement) -> Option<String> {
    let switch = when_closure(edge)?;
    if ends_branch(edge, site, switch) {
        return Some(String::new());
    }
    kuna_fix_kt_jump_to_non_loop_label::written_as(switch, Lambda::Run)
        .then(|| format!("return@label{}", switch.id()))
}

/// Whether the unlabelled `when` `switch` must be written in a
/// `run labelN@{ .. }` for a `break` out of it that does not end its branch.
pub fn needs_run(switch: Statement) -> bool {
    let mut stack: Vec<(Statement, bool)> = switch
        .as_k_switch()
        .get_case_statements()
        .to_vec()
        .into_iter()
        .map(|c| (c, true))
        .collect();
    while let Some((stat, wrapped)) = stack.pop() {
        let mut jumps = Vec::new();
        if wrapped {
            let succs = stat.get_successor_edges(Statement::STATEDGE_DIRECT_ALL);
            if succs.len() == 1 && !succs[0].get_destination().is_dummy_exit() {
                jumps.push(succs[0]);
            }
        }
        if let Some(stat_if) = stat.try_as_if() {
            if stat_if.get_ifstat().is_none() {
                jumps.extend(stat_if.get_if_edge());
            }
        }
        if jumps
            .into_iter()
            .any(|edge| when_closure(edge) == Some(switch) && !ends_branch(edge, stat, switch))
        {
            return true;
        }
        let head = matches!(
            stat.type_(),
            StatementType::If | StatementType::Switch | StatementType::Synchronized
        )
        .then(|| stat.get_first())
        .flatten();
        let children = stat.get_stats().borrow().to_vec();
        stack.extend(children.into_iter().map(|c| (c, Some(c) != head)));
    }
    false
}

fn when_closure(edge: StatEdge) -> Option<Statement> {
    if edge.get_type() != StatEdge::TYPE_BREAK || edge.labeled() || !edge.explicit() {
        return None;
    }
    edge.closure().filter(|c| c.is_k_switch())
}

/// Whether `edge` is the jump `KIfStatement` writes as the only statement of
/// the `if` `stat`.
fn is_if_edge(edge: StatEdge, stat: Statement) -> bool {
    stat.try_as_if()
        .is_some_and(|stat| stat.get_if_edge() == Some(edge) && stat.get_ifstat().is_none())
}

/// Whether control at the jump along `edge`, written after `site` (or as the
/// only statement of the `if` `site`), reaches the end of its branch of
/// `switch` and nothing else when the jump is left out.
fn ends_branch(edge: StatEdge, site: Statement, switch: Statement) -> bool {
    let cases = switch.as_k_switch().get_case_statements().to_vec();
    if is_if_edge(edge, site) && writes_jump(site, cases.contains(&site)) {
        return false;
    }
    let mut stack: Vec<(Statement, bool)> = cases.into_iter().map(|c| (c, true)).collect();
    while let Some((stat, case)) = stack.pop() {
        if stat == site {
            return true;
        }
        if writes_jump(stat, case) {
            continue;
        }
        match stat.type_() {
            StatementType::Sequence => {
                let stats = stat.get_stats();
                let last = stats.borrow().to_vec().last().copied();
                stack.extend(last.map(|s| (s, false)));
            }
            StatementType::If => {
                let stat_if = stat.try_as_if().expect("if");
                stack.extend(stat_if.get_ifstat().map(|s| (s, false)));
                stack.extend(stat_if.get_elsestat().map(|s| (s, false)));
            }
            StatementType::TryCatch => {
                stack.extend(
                    stat.get_stats()
                        .borrow()
                        .to_vec()
                        .into_iter()
                        .map(|s| (s, false)),
                );
            }
            StatementType::CatchAll => stack.extend(stat.get_first().map(|s| (s, false))),
            _ => {}
        }
    }
    false
}

/// Whether `jmpWrapper` writes a jump after `stat` (in a case, an unlabelled
/// `break` is not written).
fn writes_jump(stat: Statement, case: bool) -> bool {
    let succs = stat.get_successor_edges(Statement::STATEDGE_DIRECT_ALL);
    succs.len() == 1 && {
        let edge = succs[0];
        edge.get_type() != StatEdge::TYPE_REGULAR
            && edge.explicit()
            && !edge.get_destination().is_dummy_exit()
            && !(case && edge.get_type() == StatEdge::TYPE_BREAK && !edge.labeled())
    }
}
