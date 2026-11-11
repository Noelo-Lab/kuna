//! (kuna `sitetypelock`) The per-call-site output type lock — the primitive
//! that makes a client-asserted call-result type survive to the rendered
//! declaration.
//!
//! A type recovered AT a call site (a `buildLocaltypes` vote, an
//! `outputTypeLocal` arm) is temporary: `propagateOneType` folds it by
//! `typeOrder` and `ActionStructSynth` completes and locks its own record over
//! the base, so a per-site recovery visible mid-lattice is overridden by the
//! time the declaration prints.  Prototype-level forcing is no better: a
//! parked prototype locks the output for EVERY site of the callee, and a shell
//! prototype locks it `void *`.  What does survive is the Varnode type-lock
//! plane — `getLocalType` seeds a locked Varnode's own type,
//! `propagateTypeEdge` refuses to write into one, `writeBack`'s `updateType`
//! refuses to replace one, `ActionStructSynth` installs with `over=false`, and
//! `HighVariable::getTypeRepresentative` prefers the locked member — the same
//! chain a type-locked Database symbol rides to its declaration.  This module
//! carries that plane to the per-call-site case:
//!
//!   * the lock itself lives in the P0 [`Override`](crate::overrides::Override)
//!     store ([`insert_output_typelock`](crate::overrides::Override::insert_output_typelock)),
//!     keyed by the CALL op's address, so it survives a decompilation restart
//!     like every other per-site override;
//!   * [`ActionSiteTypeLock`], scheduled directly before `ActionInferTypes`,
//!     applies each pending lock to the CALL's output Varnode
//!     (`update_type_locked(ct, lock=true, over=false)`) as type recovery
//!     begins, so the first `buildLocaltypes` already seeds from it.
//!
//! Refusals — a refusal leaves the site exactly as it was, never half-typed:
//! a call with no output Varnode (no result survives to lock), an output
//! Varnode already type-locked (mapped-symbol/DWARF evidence outranks any
//! client), and a lock whose size differs from the output Varnode's.
//!
//! A per-site lock must not force its type onto a value that is only
//! sometimes that type. The client refuses a site whose result merges with
//! foreign provenance before contributing it (`Funcdata::site_value_is_pure`),
//! so a lock is applied only to a value that stays its own. As a second
//! guard against a merge not visible in the def-use graph (an address-tied
//! register piece), each applied lock tags its Varnode `site_locked`, and
//! [`crate::p6_variables::HighVariable::get_type_representative`] withholds
//! representative dominance from a `site_locked` member whose HighVariable
//! holds a foreign member — the ported selection is otherwise untouched, and a
//! real mapped/DWARF lock is unaffected.
//!
//! Clients contribute sites once per (re)flow, before the pending locks are
//! applied.  The first client is the Go `runtime.newobject` descriptor arm
//! ([`crate::kuna_gonewtype`], option `gonewtype`).

use std::rc::Rc;

use crate::action::{Action, ActionBase, ActionContext, ActionGroupList, ApplyResult};
use crate::funcdata::Funcdata;

/// Apply every pending per-site output type lock whose call still qualifies.
/// Returns whether any Varnode's type changed.
///
/// Each applied lock tags its output Varnode `site_locked`. That tag is what
/// lets [`crate::p6_variables::HighVariable::get_type_representative`] tell a
/// per-site lock from a mapped-symbol/DWARF lock: a site lock does NOT win the
/// representative of a HighVariable that also holds foreign members (a wider
/// storage piece, a value merged in from a parameter/global, or a different
/// allocation), so a merge can never export the lock's type as the whole
/// variable's. The merged high is ephemeral here — it is rebuilt after this
/// action each mainloop iteration — so the provenance test lives at the
/// representative, where the high is live, not in this apply.
pub fn apply_pending(data: &mut Funcdata) -> bool {
    if !data.get_override().has_output_typelocks() {
        return false;
    }
    let mut jobs = Vec::new();
    for i in 0..data.num_calls() {
        let op = data.get_call_specs(i).get_op();
        let Some(o) = data.obank().get(op) else { continue };
        let Some(ct) = data.get_override().find_output_typelock(o.get_addr()) else { continue };
        let Some(out) = o.get_out() else { continue };
        let Some(v) = data.vbank().get(out) else { continue };
        if v.is_type_lock() || v.get_size() != ct.get_size() {
            continue;
        }
        jobs.push((out, Rc::clone(ct)));
    }
    let mut changed = false;
    for (vn, ct) in jobs {
        if data.vn_update_type_locked(vn, ct, true, false) {
            if let Some(v) = data.vbank_mut().get_mut(vn) {
                v.set_site_locked();
            }
            changed = true;
        }
    }
    changed
}

/// (kuna) `ActionSiteTypeLock` — collect client site locks and pin them onto
/// the call output Varnodes as type recovery begins.
///
/// Both halves are idempotent, so the action simply re-runs on every pass of
/// its owning group: a client site resolved only after another pass settles
/// still contributes, and a lock whose output Varnode was replaced by a later
/// transform is re-pinned from the surviving override entry.
pub struct ActionSiteTypeLock {
    base: ActionBase,
}

impl ActionSiteTypeLock {
    /// Construct the action in the given group.
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionSiteTypeLock { base: ActionBase::new(0, "sitetypelock", g) })
    }
}

impl Action for ActionSiteTypeLock {
    fn base(&self) -> &ActionBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ActionBase {
        &mut self.base
    }
    fn clone_filtered(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Action>> {
        if !grouplist.contains(self.get_group()) {
            return None;
        }
        Some(Box::new(ActionSiteTypeLock { base: self.base.clone() }))
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        crate::kuna_gonewtype::contribute(data);
        if apply_pending(data) {
            self.base.count += 1;
        }
        0
    }
}

#[cfg(test)]
#[path = "kuna_sitetypelock/tests.rs"]
mod tests;
