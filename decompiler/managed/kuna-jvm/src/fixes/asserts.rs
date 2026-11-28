//! Fix family `asserts`: `assert_dead_throw` removes the throw an assert was
//! built from when nothing else reaches it; `assert_throw_after_if` reads
//! `if (c) { rest } throw new AssertionError()` as the assert followed by
//! `rest`; `statement_as_expression` keeps the exprents Java accepts only as
//! statements out of a lambda's expression body and a `for` update.

use crate::code_type::CodeType;
use crate::do_statement;
use crate::exit_exprent::Type as ExitType;
use crate::exprent::{Exprent, ExprentType};
use crate::if_statement::IfStatement;
use crate::sequence_statement::SequenceStatement;
use crate::stat_edge::StatEdge;
use crate::statement::Statement;

/// After `replace_assertion` built an assert from `throw_stat`, which the
/// `if` reached through `if_edge`: removes `throw_stat` from its sequence when
/// every other edge into it is the structural edge from the statement before
/// it, a sequence or an infinite loop, which completes normally only through
/// break edges, and there is none left.
pub fn remove_consumed_throw(throw_stat: Statement, if_edge: StatEdge) {
    let Some(parent) = throw_stat.get_parent().filter(|p| p.is_sequence()) else {
        return;
    };
    let stats = parent.get_stats().borrow().to_vec();
    let Some(pos) = stats.iter().position(|s| *s == throw_stat) else {
        return;
    };
    if pos == 0 || throw_stat.has_any_successor() {
        return;
    }
    let previous = stats[pos - 1];
    let edges = throw_stat.get_all_predecessor_edges();
    let structural = |edge: &StatEdge| {
        *edge == if_edge
            || (edge.get_type() == StatEdge::TYPE_REGULAR
                && edge.get_source() == previous
                && (previous.is_sequence()
                    || previous
                        .try_as_do()
                        .is_some_and(|d| d.get_looptype() == do_statement::Type::Infinite)))
    };
    if !edges.iter().all(structural) {
        return;
    }
    for edge in edges {
        edge.get_source().remove_successor(edge);
    }
    parent
        .get_stats()
        .borrow_mut()
        .remove_with_key(&throw_stat.id());
}

/// The throw statement that follows `stat` in `parent` when `stat` is an
/// `if` without else and the throw, the last statement of the sequence
/// `parent`, is reached only by falling out of `stat`.
pub fn throw_after(parent: Statement, stat: IfStatement) -> Option<Statement> {
    if !parent.is_sequence() || stat.iftype() != IfStatement::IFTYPE_IF {
        return None;
    }
    let stats = parent.get_stats().borrow().to_vec();
    let pos = stats.iter().position(|s| *s == stat.0)?;
    if pos + 2 != stats.len() {
        return None;
    }
    let next = stats[pos + 1];
    let into_next = next.get_all_predecessor_edges();
    let out_of_if = stat.get_all_successor_edges();
    let falls_out = |edges: &[StatEdge]| {
        edges.len() == 1
            && edges[0].get_type() == StatEdge::TYPE_REGULAR
            && edges[0].get_source() == stat.0
            && edges[0].get_destination() == next
    };
    (falls_out(&into_next) && falls_out(&out_of_if) && next.get_label_edges().borrow().is_empty())
        .then_some(next)
}

/// Replaces `stat` and the throw `next` after it in `parent` by `assertion`
/// (a block holding the assert) followed by `stat`'s body, and returns the
/// statement that took `stat`'s place.
pub fn replace(parent: Statement, stat: IfStatement, next: Statement, assertion: Statement) -> Statement {
    for edge in next.get_all_successor_edges() {
        edge.remove();
    }
    for edge in stat.get_all_successor_edges() {
        edge.remove();
    }
    parent.get_stats().borrow_mut().remove_with_key(&next.id());

    let first = stat.get_first().expect("if head");
    let ifstat = stat.get_ifstat();
    let if_edge = stat.get_if_edge().expect("if edge");
    if ifstat.is_some() {
        first.remove_successor(if_edge);
    } else {
        if_edge.change_source(assertion);
    }

    let mut lst_statements: Vec<Statement> = Vec::new();
    if first.get_exprents().is_some_and(|e| !e.is_empty()) {
        lst_statements.push(first);
    }
    lst_statements.push(assertion);
    lst_statements.extend(ifstat);

    let newstat = if lst_statements.len() == 1 {
        assertion
    } else {
        let sequence = SequenceStatement::new(lst_statements);
        sequence.set_all_parent();
        let seq_stats = sequence.get_stats().borrow().to_vec();
        for pair in seq_stats.windows(2) {
            pair[0].add_successor(StatEdge::new(StatEdge::TYPE_REGULAR, pair[0], pair[1]));
        }
        sequence.0
    };

    newstat
        .get_var_definitions()
        .add_all(stat.get_var_definitions().to_vec());
    parent.replace_statement(stat.0, newstat);
    newstat
}

/// Whether `expr` is written as a statement that Java accepts in no
/// expression position: an `assert`, a `return` or `throw`, a `yield`, a
/// monitor (`synchronized`), a statement head, or a local variable declaration.
pub fn is_statement_only(expr: Exprent) -> bool {
    match expr.get_type() {
        ExprentType::Assert
        | ExprentType::Exit
        | ExprentType::Yield
        | ExprentType::Monitor
        | ExprentType::If
        | ExprentType::SwitchHead => true,
        ExprentType::Assignment => expr
            .try_as_assignment()
            .and_then(|a| a.get_left().try_as_var())
            .is_some_and(|v| v.is_definition()),
        ExprentType::Var => expr.try_as_var().is_some_and(|v| v.is_definition()),
        _ => false,
    }
}

/// Whether a lambda whose body is the lone exprent `expr` can be written as
/// `params -> body`: a `return` that has a value is written as that value;
/// every other statement-only exprent needs a block body.
pub fn is_lambda_expression_body(expr: Exprent) -> bool {
    if let Some(exit) = expr.try_as_exit() {
        return exit.get_exit_type() == ExitType::Return
            && exit.get_ret_type().is_none_or(|t| t.type_ != CodeType::Void);
    }
    !is_statement_only(expr)
}
