//! (kuna) The writes of the variables laid over a variable.
//!
//! `Merge::checkCopyPair` (`merge.cc:1112`) calls the later of two COPYs from
//! one Varnode into a variable redundant when nothing writes the variable
//! between them, and looks only at the variable's own instances. A variable
//! that is a piece of a larger symbol shares its storage with the pieces laid
//! over it: with a two-byte slot copied to, its two bytes written one at a
//! time and the slot copied to again, the second COPY restores the slot and
//! was hidden as redundant. The writes of every overlapping piece count as
//! well. An op that prints nothing writes nothing: it is a PIECE or SUBPIECE
//! view of storage its inputs already occupy.

use crate::context::{HighVariableId, OpId};
use crate::merge::MergeContext;

/// The defining ops of the instances of every variable whose piece overlaps
/// the piece of `high`.  The intersections of the piece must be current.
pub(crate) fn overlapping_writes<C: MergeContext + ?Sized>(ctx: &C, high: HighVariableId) -> Vec<OpId> {
    let Some(piece) = ctx.high_piece(high) else {
        return Vec::new();
    };
    let mut writes = Vec::new();
    for i in 0..ctx.piece_num_intersection(piece) {
        let other = ctx.piece_high(ctx.piece_intersection(piece, i));
        if other == high {
            continue;
        }
        for k in 0..ctx.high_num_instances(other) {
            let vn = ctx.high_get_instance(other, k);
            if let Some(op) = ctx.vn_def(vn).filter(|&op| ctx.vn_is_written(vn) && !ctx.op_not_printed(op)) {
                writes.push(op);
            }
        }
    }
    writes
}
