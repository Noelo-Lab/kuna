//! kuna fix `assert_dead_throw` (`kuna_fixes`): `AssertProcessor`
//! runs before the if bodies are inlined, so the `throw new AssertionError(...)`
//! of an assert is often not the `if`'s body but a statement the `if` breaks
//! to, placed after the loop or labeled block the break leaves. Vineflower
//! builds the `assert` from it, removes the `if`'s edge and the throw's own
//! exit edge, and leaves the throw in the tree: dead code after a statement
//! that cannot complete normally, which javac rejects (`unreachable
//! statement`), and which keeps the variables of its message alive outside
//! their loop (a for-each over them is then written `for (key : ...)`). With
//! the fix, the consumed throw is removed when nothing else reaches it.

use crate::do_statement;
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
