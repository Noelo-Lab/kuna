//! kuna fix `kt_jump_to_non_loop_label` (`kuna_fixes`): a jump out of a
//! statement that Kotlin output writes as a lambda is a `return` from that
//! lambda, and a labelled `if` that is jumped out of is written as one.
//!
//! Java labels any statement (`label5: { .. break label5; .. }`); Kotlin's
//! `break@label`/`continue@label` need a loop. Vineflower writes a labelled
//! sequence as `run labelN@{ .. }` and leaves it with `return@labelN`, but it
//! writes a labelled `if` as `labelN@ if (..)` and leaves it with
//! `break@labelN`, and it writes `break@labelN` for a labelled `switch` and
//! for a `for` loop it cannot resugar, which it also wraps in
//! `run labelN@{ .. }`, and `continue@labelN` for a loop it writes as
//! `repeat(n) labelN@{ .. }`. Kotlin rejects each. kuna writes a labelled
//! `if` that a written jump leaves as `run labelN@{ if (..) { .. } }`, a
//! `break` out of a statement written as `run` as `return@labelN` (the
//! lambda's end is the statement's end), and a `continue` of a `repeat`
//! loop as `return@labelN` (or `return@repeat`), which ends the iteration.
//! `run` and `repeat` are inline, so the lambdas cost nothing at run time.

use std::cell::RefCell;

use crate::stat_edge::StatEdge;
use crate::statement::Statement;

/// The lambda form a statement is written in.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Lambda {
    /// `run labelN@{ .. }`: leaving it is `return@labelN`.
    Run,
    /// `repeat(n) labelN@{ .. }`: its next iteration is `return@labelN`.
    Repeat,
}

thread_local! {
    static LAMBDAS: RefCell<Vec<(Statement, Lambda)>> = const { RefCell::new(Vec::new()) };
}

struct Scope;

impl Drop for Scope {
    fn drop(&mut self) {
        LAMBDAS.with(|l| l.borrow_mut().pop());
    }
}

/// Runs `f`, which writes the body of `stat`, written as `lambda`.
pub fn within<R>(stat: Statement, lambda: Lambda, f: impl FnOnce() -> R) -> R {
    LAMBDAS.with(|l| l.borrow_mut().push((stat, lambda)));
    let _scope = Scope;
    f()
}

fn lambda_of(stat: Statement) -> Option<Lambda> {
    LAMBDAS.with(|l| {
        l.borrow()
            .iter()
            .rev()
            .find(|(s, _)| *s == stat)
            .map(|&(_, lambda)| lambda)
    })
}

/// Whether the jump along `edge` is a `return` from the lambda its target
/// statement is written as.
pub fn returns(edge: StatEdge) -> bool {
    let Some(closure) = edge.closure() else {
        return false;
    };
    match (edge.get_type(), lambda_of(closure)) {
        (StatEdge::TYPE_BREAK, Some(Lambda::Run)) => edge.labeled(),
        (StatEdge::TYPE_CONTINUE, Some(Lambda::Repeat)) => true,
        _ => false,
    }
}

/// Whether some jump out of the labelled `if` statement `stat` is written:
/// an explicit labelled `break` that is its `if`'s own edge, or that does not
/// go to the method's exit (`jmpWrapper` writes no jump there).
pub fn jumped_out_of(stat: Statement) -> bool {
    let edges = stat.get_label_edges().borrow().to_vec();
    edges.iter().any(|edge| {
        edge.labeled()
            && edge.explicit()
            && edge.get_type() == StatEdge::TYPE_BREAK
            && edge.closure() == Some(stat)
            && (!edge.get_destination().is_dummy_exit()
                || edge.get_source().try_as_if().is_some_and(|source| {
                    source.get_if_edge() == Some(*edge) && source.get_ifstat().is_none()
                }))
    })
}
