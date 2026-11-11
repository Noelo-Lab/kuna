//! kuna fix `switch_synthetic_default` (`kuna_fixes`), its second part: a
//! switch over an enum with javac's synthetic default is written as the
//! switch expression it was.
//!
//! javac adds the default `throw new MatchException(null, null)` (before Java
//! 21 `throw new IncompatibleClassChangeError()`) to a switch over an enum
//! only when the switch is an expression (a statement switch over an enum need
//! not be exhaustive). When the expression is returned and one case gives its
//! value, javac lays the case out falling into the return instruction, so the
//! case reads `return v;` and `SwitchExpressionHelper` finds no variable the
//! cases assign: Vineflower prints a switch statement with javac's default.
//! With the fix such a switch, whose every case is one block ending in `return
//! v` or `throw`, becomes `return switch (..) { case .. -> v; .. };`: each
//! return a `yield`, the switch the phantom backing of a switch expression in
//! a new block after it, as `SwitchExpressionHelper` builds one.

use crate::basic_block_statement::BasicBlockStatement;
use crate::exit_exprent::{ExitExprent, Type as ExitType};
use crate::rt::JList;
use crate::sequence_statement::SequenceStatement;
use crate::stat_edge::StatEdge;
use crate::statement::Statement;
use crate::substrate::kuna_fix_switch_synthetic_default;
use crate::switch_exprent::SwitchExprent;
use crate::switch_statement::SwitchStatement;
use crate::yield_exprent::YieldExprent;

/// Turns `stat` into `return switch (..) {..}` when it is a switch over an
/// enum with javac's synthetic default whose cases return or throw.
pub fn process(stat: SwitchStatement) -> bool {
    if stat.is_phantom()
        || stat.is_pattern()
        || stat.get_parent().is_none()
        || !stat
            .0
            .get_successor_edges(StatEdge::TYPE_REGULAR)
            .is_empty()
        || !kuna_fix_switch_synthetic_default::is_javac_default(stat)
    {
        return false;
    }
    let Some(default) = stat.get_default_edge() else {
        return false;
    };
    let mut returns: Vec<(Statement, ExitExprent)> = Vec::new();
    let cases = stat.get_case_statements();
    let case_edges = stat.get_case_edges();
    for i in 0..cases.len() {
        let case = cases.get(i);
        let edges = case_edges.get(i);
        if edges.contains(&default) {
            if edges.len() != 1 {
                return false;
            }
            continue;
        }
        if !case.is_basic_block() {
            return false;
        }
        let Some(exprents) = case.get_exprents().filter(|e| !e.is_empty()) else {
            return false;
        };
        let Some(exit) = exprents.get(exprents.len() - 1).try_as_exit() else {
            return false;
        };
        if exprents
            .iter()
            .take(exprents.len() - 1)
            .any(|e| e.is_exit())
        {
            return false;
        }
        let succs = case.get_all_successor_edges();
        if succs.len() != 1 || !succs[0].get_destination().is_dummy_exit() {
            return false;
        }
        match exit.get_exit_type() {
            ExitType::Throw => {}
            ExitType::Return if exit.get_value().is_some() => returns.push((case, exit)),
            _ => return false,
        }
    }
    let Some(&(_, sample)) = returns.first() else {
        return false;
    };
    let Some(ret_type) = sample.get_ret_type() else {
        return false;
    };
    let exit_edge = returns[0].0.get_all_successor_edges()[0];

    for &(case, exit) in &returns {
        let value = exit.get_value().expect("return value");
        let yield_ = YieldExprent::new(value, ret_type.clone());
        case.replace_exprent(exit.0, yield_.0);
    }
    stat.set_phantom(true);

    let suc = BasicBlockStatement::create();
    let switch = SwitchExprent::new(stat, ret_type.clone(), false, false);
    let ret = ExitExprent::new(
        ExitType::Return,
        Some(switch.0),
        Some(ret_type),
        None,
        sample.get_method_descriptor(),
    );
    suc.set_exprents(JList::from(vec![ret.0]));

    let seq = SequenceStatement::new_varargs(&[stat.0, suc.0]);
    seq.set_parent(stat.get_parent());
    stat.replace_with(seq.0);
    seq.set_all_parent();
    suc.add_successor(StatEdge::new_closure(
        exit_edge.get_type(),
        suc.0,
        exit_edge.get_destination(),
        exit_edge.closure(),
    ));
    true
}
