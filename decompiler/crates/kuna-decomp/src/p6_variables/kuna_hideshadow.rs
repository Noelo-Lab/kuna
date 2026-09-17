//! (kuna) Consolidate redundant copies of one value — the `hideshadow` option.
//!
//! # The gap
//!
//! When two Varnodes are copied from the same ancestor they always hold the
//! same value, so upstream Ghidra calls them *shadows* of one variable.  If the
//! two copy paths are not nested the merge machinery cannot put them in one
//! `HighVariable`, and the emitted C keeps a copy for each:
//!
//! ```text
//! v29 = LONGINT_OVERFLOW;
//! v35 = v29;
//! ...
//! v29 = v34 | v35;
//! v35 = v34 | v35;
//! ```
//!
//! `ActionHideShadow` is the upstream pass that repairs this
//! (`coreaction.cc:4976`): for every `HighVariable` it calls
//! [`Merge::hide_shadows`](crate::merge::Merge::hide_shadows), which re-points
//! the second copy's input at the first so that ancestor -> first -> second is
//! one chain.  Both Varnodes then reach the same `HighVariable`, and the
//! following `ActionCopyMarker` (`mark_internal_copies`) hides the assignment.
//! The engine half has been ported since the merge wave; the action body was a
//! stub returning 0, so the pass never ran and the redundant assignments stayed.
//!
//! # What this does
//!
//! [`hide_shadow_copies`] is the upstream body: walk the written def-set, dedup
//! `HighVariable`s through the bank's mark surface, and call `hide_shadows` on
//! each.  Nothing here decides anything — every acceptance test lives in
//! `Merge::hide_shadows`, which only re-points a COPY when
//!
//! * `Varnode::copy_shadow` proves both Varnodes trace to one common ancestor
//!   through COPYs alone (a COPY never changes the value), and
//! * the surviving Varnode's Cover *strictly contains* the other's definition
//!   point (`contain_varnode_def == 1`, interior, not a boundary) — so the value
//!   is live and unclobbered at the point the copy is re-pointed to it.
//!
//! A value that is conditionally re-defined between the two copies fails the
//! second test: the re-definition splits the Cover, the def point is no longer
//! interior to it, and the pair is declined.
//!
//! # Scheduling
//!
//! `ActionHideShadow` is already in `universal_sched` at its upstream slot,
//! immediately before `ActionCopyMarker` in the merge phalanx, so wiring the
//! body moves no action index.

use crate::action::ApplyResult;
use crate::context::VarnodeId;
use crate::funcdata::Funcdata;
use crate::merge::MergeContext;
use crate::options::on_or_off;
use crate::varnode::varnode_flags;
use kuna_base::error::KunaResult;

/// Run `Merge::hideShadows` over every `HighVariable` with a written instance
/// (C++ `ActionHideShadow::apply`, `coreaction.cc:4976-4995`).  Returns the
/// number of HighVariables whose data-flow changed.
///
/// The upstream walk is `beginDef()`..`endDef(written)`, which covers the input
/// and written classes; only the written class is iterated here because a
/// HighVariable with no written instance yields an empty `find_single_copy`
/// list and cannot change.  The mark bit is set and cleared over the same set,
/// as upstream does, so no mark survives the pass.
pub fn hide_shadow_copies(data: &mut Funcdata) -> ApplyResult {
    data.with_covermerge(|merge, d| {
        let defs: Vec<VarnodeId> = d.vbank().iter_def_flag(varnode_flags::written).collect();
        let mut count: ApplyResult = 0;
        for &vn in &defs {
            let high = match d.vn_high(vn) {
                None => continue,
                Some(h) => h,
            };
            if d.high_is_mark(high) {
                continue;
            }
            if merge.hide_shadows(d, high).unwrap_or(false) {
                count += 1;
            }
            d.bank_set_mark(high);
        }
        for &vn in &defs {
            if let Some(high) = d.vn_high(vn) {
                d.bank_clear_mark(high);
            }
        }
        count
    })
}

/// (kuna) Toggle the shadow-copy consolidation: `hideshadow on|off`.
pub struct OptionHideShadow;

impl OptionHideShadow {
    /// The option name.
    pub const NAME: &'static str = "hideshadow";

    /// Parse `on|off` and return the resolved flag + confirmation message.  The
    /// caller writes the flag into `Architecture::hide_shadow`.
    pub fn apply(&self, p1: &str) -> KunaResult<(bool, String)> {
        let val = on_or_off(p1)?;
        let prop = if val { "on" } else { "off" };
        Ok((val, format!("Shadow-copy consolidation turned {prop}")))
    }
}

#[cfg(test)]
#[path = "kuna_hideshadow/tests.rs"]
mod tests;
