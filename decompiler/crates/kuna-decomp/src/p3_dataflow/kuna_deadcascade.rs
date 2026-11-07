//! (kuna `deadcascade`) Dead-op removal takes out a dead chain of joins and
//! guards at once.
//!
//! `RuleEarlyRemoval` removes an op nothing reads when the pool's cursor
//! reaches it. The op defining what it read usually sits earlier in sequence
//! order, where the cursor has already been, so a chain of values nothing reads
//! dies one link per pool pass, and every pass sweeps the whole function. Here
//! the removal of a join (MULTIEQUAL) or guard (INDIRECT) also removes the
//! joins and guards defining what it read that sit behind the cursor and are
//! left unread, in the order the following passes would have: one batch per
//! pass, each in sequence order, a definition that dies behind the op removing
//! it going to the next batch and one ahead of it to the same batch.
//! Definitions at or ahead of the cursor, and dead ops of other kinds, are left
//! to the passes, which remove them when they reach them.

use std::collections::BTreeMap;

use kuna_base::address::SeqNum;

use crate::context::OpId;
use crate::funcdata::Funcdata;

/// The default pass of a pool run from which removal cascades: a run that has
/// gone this long is almost always taking dead chains apart one link a pass.
pub(crate) const DEFAULT_PASSES: u32 = 32;

/// Remove `op`, which `removable` accepted, and when it is a join or a guard
/// the dead chain of joins and guards behind it.
pub(crate) fn remove(data: &mut Funcdata, op: OpId, removable: fn(OpId, &mut Funcdata) -> bool) {
    let o = data.obank().get(op).expect("deadcascade: stale op");
    if !o.is_marker() {
        data.op_destroy(op);
        return;
    }
    let cursor = o.get_seq_num().clone();
    let mut next = BTreeMap::new();
    behind(data, op, &cursor, &cursor, &mut next, &mut BTreeMap::new());
    data.op_destroy(op);
    while !next.is_empty() {
        let mut batch = std::mem::take(&mut next);
        while let Some((seq, cur)) = batch.pop_first() {
            if data.obank().get(cur).is_none_or(|o| o.is_dead()) || !removable(cur, data) {
                continue;
            }
            data.note_dead_removal(cur);
            behind(data, cur, &seq, &cursor, &mut next, &mut batch);
            data.op_destroy(cur);
        }
    }
}

/// File the live definitions of `op`'s inputs that sit before `cursor`: into
/// `later` when they also sit before `op` (`at`), else into `same`.
fn behind(
    data: &Funcdata,
    op: OpId,
    at: &SeqNum,
    cursor: &SeqNum,
    later: &mut BTreeMap<SeqNum, OpId>,
    same: &mut BTreeMap<SeqNum, OpId>,
) {
    let Some(o) = data.obank().get(op) else {
        return;
    };
    for k in 0..o.num_input() {
        let Some(def) = o.get_in(k).and_then(|v| data.vbank().get(v)).and_then(|v| v.get_def()) else {
            continue;
        };
        let Some(d) = data.obank().get(def) else {
            continue;
        };
        let seq = d.get_seq_num();
        if d.is_dead() || !d.is_marker() || seq >= cursor {
            continue;
        }
        if seq < at {
            later.insert(seq.clone(), def);
        } else {
            same.insert(seq.clone(), def);
        }
    }
}
