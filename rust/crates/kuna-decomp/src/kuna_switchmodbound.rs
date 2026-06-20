//! Port of `decompiler/cpp/kuna_switchmodbound.{cc,hh}` — modulo/and-mask
//! jump-table index-bound option (kuna GH-9191, S2 switch-model).
//!
//! When a jump-table index is bounded only by an in-band `index % N` (or
//! `& mask`) — with no CBRANCH guard and no direct INT_AND on the normalized
//! switch variable — and the BRANCHIND value is a table LOAD whose pointer is
//! `index*stride + base`, the basic model fails to bound the table and recovery
//! aborts ("Too many branches").  When `option switchmodbound on`, the recovery
//! re-bounds the table to `N` (resp. `mask+1`) entries.
//!
//! ## What this module owns vs. SEAM(W4)
//!
//! The assigned C++ scope (`kuna_switchmodbound.cc/.hh`) contains **only** the
//! `ElementId` and the `ArchOption` that sets the arch flag — *not* the recovery
//! logic.  As the C++ header states, the recovery walk
//! `JumpBasic::kunaTryModuloBoundTable(Funcdata*, PcodeOp* indop, uint4
//! maxtablesize)` is a member of `JumpBasic` and lives next to the rest of the
//! model in `jumptable.cc` (it needs the protected `pathMeld`/`jrange`/
//! `varnodeIndex` state).  That member therefore ports with the **W4 jumptable**
//! wave, on the `JumpBasic` model — see the loss note below; only the gate
//! (`Architecture::switch_modulo_bound`, default off) is owned here.
//!
//! SEAM(W4): `JumpBasic::kunaTryModuloBoundTable` walks the strictly-linear
//! chain `BRANCHIND <- LOAD( index*stride + base )` with `index` bounded by
//! `% N` (`CPUI_INT_REM`/`CPUI_INT_SREM`) or `& mask` (`CPUI_INT_AND`, a clean
//! `coveringmask`), allowing only the realigning ops `INT_ADD`/`INT_MULT`/
//! `INT_LEFT`/`SUBPIECE`/`INT_ZEXT`/`INT_SEXT` with a constant other-input, and
//! — when `bound` is in `[2, maxtablesize]` and a single table LOAD was seen —
//! rebuilds the model (`pathMeld`/`varnodeIndex`/`jrange = [0, bound)`).  It is
//! invoked from `recoverModel()` only after the normal model fails to bound the
//! table.  Ported by the W4 jumptable porter on `JumpBasic`, gated on
//! [`SwitchModBoundOption`].

/// (kuna) Toggle bounding a LOAD-table jumptable by a modulo/and-mask on its
/// index (C++ `OptionSwitchModBound`, GH-9191).
///
/// SEAM(W4): the C++ `OptionSwitchModBound::apply` flips
/// `glb->switch_modulo_bound`; here the flag is carried as a plain `bool` whose
/// [`Default`] is the *shipped* default (`option switchmodbound off`, i.e.
/// \b false — upstream byte-identical; `architecture.cc:1440`).  W4's
/// `JumpBasic::recoverModel` reads this gate before calling
/// `kunaTryModuloBoundTable`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SwitchModBoundOption {
    /// True if a modulo/and-mask on a LOAD-table jumptable index bounds the
    /// table (C++ `Architecture::switch_modulo_bound`).
    pub enabled: bool,
}

impl Default for SwitchModBoundOption {
    /// Shipped default: `option switchmodbound off` (upstream byte-identical;
    /// `architecture.cc:1440` sets `switch_modulo_bound = false`).
    fn default() -> Self {
        SwitchModBoundOption { enabled: false }
    }
}

impl SwitchModBoundOption {
    /// (kuna) Set the gate (C++ `OptionSwitchModBound::apply`: `bool val =
    /// onOrOff(p1); glb->switch_modulo_bound = val;`).
    pub fn apply(&mut self, val: bool) -> &'static str {
        self.enabled = val;
        if val {
            "Jump-table modulo/and-mask index bounding turned on"
        } else {
            "Jump-table modulo/and-mask index bounding turned off"
        }
    }

    /// Read the gate (C++ `glb->switch_modulo_bound`).
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

#[cfg(test)]
#[path = "kuna_switchmodbound/tests.rs"]
mod tests;
