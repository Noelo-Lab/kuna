//! (kuna `rettriangle`) Decline a `returndup` split at the join of a one-block
//! `if`.
//!
//! [`returndup`](crate::p8_structure::kuna_returndup) gives every predecessor of
//! a shared constant-return block its own copy of the return, which recovers a
//! source's early-return guards.  When the two predecessors are the two arms of a
//! one-block `if` -- a condition `C` that goes either straight to the return `R`
//! or through one block `T` that runs into `R` -- there is no guard to recover:
//! the split turns
//!
//! ```c
//! if (c)
//!   f();
//! return 0;
//! ```
//!
//! into `if (!c) return 0; f(); return 0;`, one more statement and the return
//! written twice, where IDA, Ghidra and the source all print the triangle.
//!
//! With `option rettriangle on`,
//! [`Funcdata::returndup_apply`](crate::funcdata::Funcdata) drops such a return
//! from its candidates: it has exactly two predecessors, one of them a two-way
//! block whose other successor is the second predecessor, and that block is
//! entered only from the condition and leaves only to the return.  Other
//! candidates of the same function still split.  The option can only make
//! `returndup` fire less, and is inert with `option returndup off`.

use kuna_base::error::KunaResult;

use crate::context::BlockId;
use crate::funcdata::Funcdata;
use crate::options::on_or_off;

/// Whether shared return block `r` is the join of a one-block `if`: two
/// predecessors, one a two-way condition whose other successor is the other
/// predecessor, which has that condition as its only predecessor and `r` as its
/// only successor.
pub(crate) fn is_triangle_join(data: &Funcdata, r: BlockId) -> bool {
    let g = data.bblocks_ref();
    let rb = g.block(r);
    if rb.size_in() != 2 {
        return false;
    }
    let (a, b) = (rb.get_in(0), rb.get_in(1));
    let arm_of = |cond: BlockId, arm: BlockId| -> bool {
        if cond == arm || cond == r || arm == r {
            return false;
        }
        let c = g.block(cond);
        let t = g.block(arm);
        c.size_out() == 2
            && c.get_out_index(r) >= 0
            && c.get_out_index(arm) >= 0
            && t.size_in() == 1
            && t.get_in(0) == cond
            && t.size_out() == 1
            && t.get_out(0) == r
    };
    arm_of(a, b) || arm_of(b, a)
}

/// The `rettriangle on|off` option.
pub struct OptionRetTriangle;

impl OptionRetTriangle {
    pub const NAME: &'static str = "rettriangle";

    pub fn apply(&self, p1: &str) -> KunaResult<(bool, String)> {
        let val = on_or_off(p1)?;
        let prop = if val { "on" } else { "off" };
        Ok((val, format!("One-block-if return duplication decline turned {prop}")))
    }
}
