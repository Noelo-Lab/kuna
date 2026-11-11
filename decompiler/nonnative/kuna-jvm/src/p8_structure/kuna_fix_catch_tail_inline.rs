//! kuna fix `catch_tail_inline` (`kuna_fixes`): `InlineSingleBlockHelper`
//! moves the statements that follow a statement into the place the only break
//! to them comes from, but not when that break is implicit and they break out
//! of their sequence. A `try` whose body cannot complete normally is followed
//! by the code only its handler reaches; left after the `try`, code that reads
//! the handler's variable refers to it out of its scope and does not compile.
//! With the fix such code is moved into the handler regardless.

use crate::rt::JHashSet;
use crate::sequence_statement::SequenceStatement;
use crate::stat_edge::StatEdge;
use crate::statement_iterator::StatementIterator;
use crate::var_version_pair::VarVersionPair;

/// Whether the statements of `seq` from `index` on read the variable of a
/// catch clause that `edge`, the only break to them, leaves.
pub fn reads_left_catch_var(seq: SequenceStatement, index: usize, edge: StatEdge) -> bool {
    let to = edge.get_destination();
    let mut from = edge.get_source();
    let mut vars: JHashSet<VarVersionPair> = JHashSet::new();
    while let Some(parent) = from.get_parent() {
        if parent.contains_statement_strict(to) {
            break;
        }
        if let Some(catch) = parent.try_as_catch() {
            let stats = parent.get_stats().borrow().to_vec();
            if let Some(i) = stats.iter().position(|s| *s == from).filter(|&i| i > 0) {
                if let Some(var) = catch.get_vars().to_vec().get(i - 1) {
                    vars.add(var.get_var_version_pair());
                }
            }
        }
        from = parent;
    }
    if vars.is_empty() {
        return false;
    }
    let stats = seq.get_stats().borrow().to_vec();
    let mut found = false;
    for st in &stats[index..] {
        StatementIterator::iterate(*st, &mut |e| {
            if let Some(var) = e.try_as_var() {
                found |= vars.contains(&var.get_var_version_pair());
            }
            0
        });
    }
    found
}
