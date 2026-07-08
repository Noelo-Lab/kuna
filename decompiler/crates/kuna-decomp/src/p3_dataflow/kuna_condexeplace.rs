//! Port of `decompiler/cpp/kuna_condexeplace.{cc,hh}` — conditional-constant COPY
//! block-placement (kuna GH-9203, S3 simplification-quiescence).
//!
//! `ActionConditionalConst::handlePhiNodes` materializes a known-constant value
//! reaching a MULTIEQUAL on a disconnected edge as a COPY placed at the bottom of
//! the edge's predecessor block.  If that predecessor is a LOOP block the COPY
//! lands inside the loop body and re-executes every iteration (GH-9203).  When
//! `option condexeplace on`, `handlePhiNodes` declines to materialize the COPY
//! into a loop predecessor block, leaving the MULTIEQUAL input edge unchanged.
//!
//! ## What this module owns vs. ANCHOR-EDIT(coreaction.cc)
//!
//! The assigned C++ scope (`kuna_condexeplace.cc/.hh`) contains **only** the
//! `ElementId` and the `ArchOption` that sets the arch flag
//! `condexe_block_placement` — *not* the behavioral fix.  As the C++ header
//! states, "the actual gate is a minimal guarded block at the decision point in
//! `ActionConditionalConst::handlePhiNodes` (coreaction.cc); this file only owns
//! the `OptionCondExePlace` surface and its `ElementId`."  Only the gate
//! ([`Architecture::condexe_block_placement`](crate::architecture::Architecture),
//! default on) is owned here.
//!
//! ### ANCHOR-EDIT for W8 / the W-coreaction porter
//!
//! `ActionConditionalConst::handlePhiNodes` (an upstream `coreaction.cc` method)
//! gets a kuna anchor edit: when `glb->condexe_block_placement` is set, the
//! method declines to materialize the constant COPY into a loop predecessor
//! block.  That edit lands with the `ActionConditionalConst` port
//! (`coreaction.cc`, the W-coreaction wave), gated on [`CondExePlaceOption`].
//! See `UPSTREAM.md` *Divergence*; this module supplies only the toggle so the
//! option surface and its ElementId exist.

/// (kuna GH-9203) Toggle whether `ActionConditionalConst` may place a const COPY
/// in a loop block (C++ `OptionCondExePlace`).
///
/// ANCHOR-EDIT(coreaction.cc): the C++ `OptionCondExePlace::apply` flips
/// `glb->condexe_block_placement`; here the flag is carried as a plain `bool`
/// whose [`Default`] is the *shipped* default (`option condexeplace on`, i.e.
/// \b true — kuna DIV-3 default-on; `architecture.cc` sets
/// `condexe_block_placement = true`).  `ActionConditionalConst::handlePhiNodes`
/// reads this gate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CondExePlaceOption {
    /// True if `ActionConditionalConst` declines to place a const COPY in a loop
    /// block (C++ `Architecture::condexe_block_placement`).
    pub enabled: bool,
}

impl Default for CondExePlaceOption {
    /// Shipped default: `option condexeplace on` (kuna DIV-3 default-on, GH-9203;
    /// `architecture.cc` sets `condexe_block_placement = true`).
    fn default() -> Self {
        CondExePlaceOption { enabled: true }
    }
}

impl CondExePlaceOption {
    /// (kuna) Set the gate (C++ `OptionCondExePlace::apply`: `bool val =
    /// onOrOff(p1); glb->condexe_block_placement = val;`).
    pub fn apply(&mut self, val: bool) -> &'static str {
        self.enabled = val;
        if val {
            "Conditional-constant loop-block placement guard turned on"
        } else {
            "Conditional-constant loop-block placement guard turned off"
        }
    }

    /// Read the gate (C++ `glb->condexe_block_placement`).
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

#[cfg(test)]
#[path = "kuna_condexeplace/tests.rs"]
mod tests;
