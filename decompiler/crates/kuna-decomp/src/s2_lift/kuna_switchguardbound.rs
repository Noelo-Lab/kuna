//! (kuna) Guard-bounded jump-table index option — `option switchguardbound`
//! (angr port: `test_decompiling_missing_function_call`; S2 switch-model).
//!
//! Sibling of [`kuna_switchmodbound`](crate::s2_lift::kuna_switchmodbound)
//! (GH-9191). Both address the same recovery failure — the basic `JumpBasic`
//! model cannot bound a LOAD-table index, so `JumpTable::recoverAddresses`
//! aborts with *"Too many branches"* and `FlowInfo::truncateIndirectJump`
//! rewrites the `BRANCHIND` into a `CALLIND` (`/* WARNING: Treating indirect
//! jump as call */`), collapsing the whole `switch` (and any loop around it).
//!
//! `switchmodbound` re-bounds the table when an *in-band* `index % N` / `& mask`
//! is the only bound. This option handles the complementary case angr's
//! `test_decompiling_missing_function_call` (binary `adams`, function `main`)
//! exposes: an *out-of-band* CBRANCH **range guard** (`idx <=u K`) that
//! `JumpBasic::analyzeGuards` failed to turn into a bound because, on the early
//! partial/truncated function jump-table recovery runs over, the guard is still
//! in unsimplified x86 flag form (`sub K; ja`) — `circlerange_pull_back` cannot
//! extract `<= K` — and the switch variable is additionally spilled to the stack
//! between the guard test and the table load, so the guarded varnode never
//! `value_match`es the normalized index.
//!
//! ## What this module owns vs. SEAM(W4)
//!
//! As with `kuna_switchmodbound`, the assigned scope here is **only** the
//! `ElementId` and the `ArchOption` that flips the arch flag — *not* the
//! recovery logic. The recovery walk
//! `JumpBasic::kuna_try_guard_bound_table(Funcdata*, PcodeOp* indop, uint4
//! maxtablesize)` is a member of `JumpBasic` and lives next to the rest of the
//! model in `jumptable.rs` (it needs the protected `pathMeld`/`jrange`/
//! `selectguards`/`varnodeIndex` state). It is invoked from `recoverModel()`
//! only after the normal model — and the modulo/and-mask extension — fail to
//! bound the table. Only the gate (`Architecture::switch_guard_bound`, default
//! off) is owned here.
//!
//! The option ships **default-off** (upstream byte-identical): a CBRANCH-guard
//! bound is a control-flow heuristic that asserts the guarded value equals the
//! table index across a memory round-trip it cannot prove in dataflow, so a
//! loose match could over-bound an unrelated indirect jump. Flip it per program
//! (`option switchguardbound on`) when a switch renders as a computed call with
//! *"Could not recover jumptable … Too many branches"*.

use kuna_base::marshal::ElementId;

/// Marshaling element `<switchguardbound>` (kuna). ElementIds live in the 4000+
/// range (C++ `ELEM_SWITCHGUARDBOUND = ElementId("switchguardbound",4022)`;
/// 4021 is `stackguard`).
pub const ELEM_SWITCHGUARDBOUND: ElementId = ElementId::new("switchguardbound", 4022);

/// (kuna) Toggle bounding a LOAD-table jumptable by an out-of-band CBRANCH range
/// guard on its index (C++ `OptionSwitchGuardBound`).
///
/// SEAM(W4): the C++ `OptionSwitchGuardBound::apply` flips
/// `glb->switch_guard_bound`; here the flag is carried as a plain `bool` whose
/// [`Default`] is the *shipped* default (`option switchguardbound off`, i.e.
/// \b false — upstream byte-identical). W4's `JumpBasic::recoverModel` reads this
/// gate before calling `kuna_try_guard_bound_table`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SwitchGuardBoundOption {
    /// True if a CBRANCH range guard on a LOAD-table jumptable index bounds the
    /// table (C++ `Architecture::switch_guard_bound`).
    pub enabled: bool,
}

impl Default for SwitchGuardBoundOption {
    /// Shipped default: `option switchguardbound off` (upstream byte-identical).
    fn default() -> Self {
        SwitchGuardBoundOption { enabled: false }
    }
}

impl SwitchGuardBoundOption {
    /// (kuna) Set the gate (C++ `OptionSwitchGuardBound::apply`: `bool val =
    /// onOrOff(p1); glb->switch_guard_bound = val;`).
    pub fn apply(&mut self, val: bool) -> &'static str {
        self.enabled = val;
        if val {
            "Jump-table CBRANCH-guard index bounding turned on"
        } else {
            "Jump-table CBRANCH-guard index bounding turned off"
        }
    }

    /// Read the gate (C++ `glb->switch_guard_bound`).
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

#[cfg(test)]
#[path = "kuna_switchguardbound/tests.rs"]
mod tests;
