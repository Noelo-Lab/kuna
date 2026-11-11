//! kuna fix `loop_first_if_exit` (spec 10.16): the first `if` of an infinite
//! loop becomes the loop's exit test only when the path past the `if`
//! leaves the loop.
//!
//! `LoopExtractHelper.extractFirstIf` rewrites `while (true) { if (c) { X;
//! continue; } }` as `while (true) { if (!c) break; X; continue; }`, the
//! shape `enhanceLoops` turns into `while (c) { X }`. The `break` it adds
//! goes to the first regular successor of the loop or of a statement around
//! it, on the assumption that the `if`'s own successor, the path taken when
//! `c` is false, is that exit. When it is not, the rewrite inverts the loop:
//! after an empty inner loop sharing the head of the infinite loop was
//! removed (`while (true) { while (v[++i] < p) {} .. }`), the `if` that
//! remained continues the loop when `c` is false, and the rewrite made
//! that the loop's exit (`while (v[++i] >= p) { .. }`, leaving where the
//! bytecode loops). With the fix the rewrite is made only when every
//! successor of the `if` is a `break` to that exit.

use crate::if_statement::IfStatement;
use crate::stat_edge::StatEdge;
use crate::statement::Statement;

/// Whether the path past `firstif` already leaves for `destination`.
pub fn exits_to(firstif: IfStatement, destination: Statement) -> bool {
    let succs = firstif.get_successor_edges(Statement::STATEDGE_DIRECT_ALL);
    !succs.is_empty()
        && succs
            .iter()
            .all(|e| e.get_type() == StatEdge::TYPE_BREAK && e.get_destination() == destination)
}
