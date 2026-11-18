//! kuna fix `protected_back_edge` (spec 10.16): a loop whose back edge lies
//! in a `try` (or `synchronized`) range is built inside that statement.
//!
//! `DomHelper` collapses the region of a method bottom-up. When the region
//! of a loop body ends at an exit that lies outside a `finally`'s range (the
//! copy of the `finally` javac puts before a `return`), the body keeps a
//! regular edge to that exit, `DoStatement.isHead` refuses it, and
//! `CatchAllStatement.isHead` wraps the `try` around the body alone: the loop
//! then surrounds the `try`, so the back edge leaves it and Java runs the
//! `finally` on every iteration, which the bytecode does not
//! (`try { while (true) { .. } } finally { .. }` came out
//! `while (true) { try { .. } finally { .. } }`). With the fix, a `try` is
//! never built around a body whose back edge returns to its own first block,
//! and `findGeneralStatement` takes such a body alone as a region ending at
//! its exit, inside which the exit is a `break`, so the loop is built first
//! and the `try` around it.

use crate::statement::Statement;

/// Whether `head` holds a back edge to its own first block: the body of a
/// loop that is not built yet.
pub fn is_open_loop_body(head: Statement) -> bool {
    let basichead = head.get_basichead().0;
    head.get_continue_set().borrow().contains(&basichead)
}
