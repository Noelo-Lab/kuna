//! (kuna `callerreads`) Where each direct call returns to.
//!
//! A function decompiled alone may return what its callers read after calling
//! it (`kuna_decomp::kuna_callerreads`). The walk already files every direct
//! call as a [`RefKind::Call`] edge, so this reads the completed Listing and
//! pairs each edge's target with the address the calling instruction falls
//! through to. The decompile step decodes the callers from those addresses only
//! for a function it recovered `void`.

use super::model::RefKind;
use super::Listing;

/// Every direct call of the walk as `(callee entry, return address)`, sorted
/// and de-duplicated.
pub fn call_returns(listing: &Listing) -> Vec<(u64, u64)> {
    let mut out: Vec<(u64, u64)> = Vec::new();
    for from in listing.ref_source_iter() {
        let Some(back) = listing.instruction_at(from).and_then(|i| i.fall_through) else { continue };
        for r in listing.refs_from(from) {
            if r.kind == RefKind::Call {
                out.push((r.to, back));
            }
        }
    }
    out.sort_unstable();
    out.dedup();
    out
}
