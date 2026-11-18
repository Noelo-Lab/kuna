//! kuna fix `assert_throw_after_if` (`kuna_fixes`): `AssertProcessor`
//! recognises the `throw new AssertionError(...)` of an assert as the body of
//! the `if` that tests `$assertionsDisabled`, as its else, or as the
//! statement its body breaks to. When the code after the assert cannot
//! complete normally (it returns, or is the end of a loop body), javac's
//! `ifne` past the throw makes the structurer write the `if` the other way
//! round: `if ($assertionsDisabled || cond) { rest; return; } throw new
//! AssertionError();` (or `if (...) continue;` with no body), the throw being
//! the next statement of the sequence, reached only when the condition is
//! false. Vineflower leaves that shape alone and prints a reference to the
//! synthetic `$assertionsDisabled`, which javac cannot resolve. With the fix
//! the shape is read as `if (!(...)) throw new AssertionError(); rest`,
//! which is the assert followed by the `if`'s body.

use crate::if_statement::IfStatement;
use crate::sequence_statement::SequenceStatement;
use crate::stat_edge::StatEdge;
use crate::statement::Statement;

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
