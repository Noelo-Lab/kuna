//! P3 heritage-refinement store mark — the `splitstorekeep` decision point.
//!
//! # The gap
//!
//! A frame store lowers to `STORE(stack_ptr, value)`, and `RuleStoreVarnode`
//! (`ruleaction.cc:4396`) rewrites it to a direct `stackvn = COPY(value)`,
//! marking the new output `stack_store` — "this Varnode originally came from a
//! CPUI_STORE".  That mark is the whole reason the store survives:
//! `ActionDirectWrite` (`coreaction.cc:1366`) calls a `COPY` into the frame a
//! *direct write* only when its output `isStackStore()`, and `ActionDeadCode`
//! (`coreaction.cc:4146`) drops the `addrforce` mark of anything that is not a
//! direct write — `if (isAddrForce() && !isDirectWrite()) clearAddrForce()`.
//! Without `addrforce` the store's output is no longer auto-live, nothing
//! consumes it, and the defining `COPY` is swept.
//!
//! `Heritage::refinement` (`heritage.cc:1891`) breaks that chain. When two
//! accesses to one stack range overlap, refinement cuts every access in the
//! range to a common partition: `Heritage::refineWrite` (`heritage.cc:1807`)
//! re-points the store's defining op at a fresh `unique` and rebuilds the
//! original storage out of `SUBPIECE`s of it, one per partition cell. The cells
//! are built by `Heritage::splitByRefinement` with plain `Funcdata::newVarnode`,
//! so none of them carries the `stack_store` mark the store had. Every refined
//! piece is therefore a non-direct-write, loses its `addrforce`, and is deleted.
//!
//! The shape that hits this is the compiler's own idiom for copying a buffer
//! whose size is not a multiple of the word: gcc copies a 31-byte array with
//! four 8-byte moves at offsets 0, 8, 15 and 23, so the middle two overlap on
//! byte 15. Refinement splits exactly those two and no others, and the emitted C
//! copies bytes 0..7 and 23..30 while the fifteen bytes in between are silently
//! never written:
//!
//! ```text
//! v7[0] = v6[0];
//! v7[7] = v6[7];
//! v7[0x17] = v6[0x17];      // 8..22 are simply absent
//! ```
//!
//! An analyst reading that concludes the second half of the buffer is
//! uninitialised, which is the opposite of what the binary does.
//!
//! # The mark
//!
//! [`keep_store_mark`] carries `stack_store` from the write being refined onto
//! each of its refinement pieces. It claims nothing new: a piece of a store IS a
//! store, and the mark's own definition — "originally came from a CPUI_STORE" —
//! is as true of the cells as of the whole. Nothing else about refinement
//! changes, and a write that was not a stack store propagates nothing.
//!
//! It is inert on any function with no overlapping stack accesses, because
//! `Heritage::refinement` returns without splitting when every access in a range
//! already agrees on its boundaries.

use crate::context::VarnodeId;
use crate::funcdata::Funcdata;

/// Carry the `stack_store` mark from a refined write onto its pieces (kuna
/// `splitstorekeep`).
///
/// Called from [`Heritage::refine_write`](crate::p3_dataflow::heritage) with the
/// write Varnode about to be replaced and the partition cells that will carry
/// its storage.  Returns whether the mark was propagated.
pub fn keep_store_mark(fd: &mut Funcdata, vn: VarnodeId, pieces: &[VarnodeId]) -> bool {
    if !fd.get_arch().split_store_keep {
        return false;
    }
    if !fd.vbank().get(vn).map(|v| v.is_stack_store()).unwrap_or(false) {
        return false;
    }
    for &piece in pieces {
        if let Some(p) = fd.vbank_mut().get_mut(piece) {
            p.set_stack_store();
        }
    }
    !pieces.is_empty()
}
