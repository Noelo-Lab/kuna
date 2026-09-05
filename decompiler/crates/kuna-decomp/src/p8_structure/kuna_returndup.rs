//! S8 **shared bare-epilogue return duplication** (port of angr's SAILR
//! `ReturnDuplicatorHigh` — the *gotoless* return duplicator; Basque et al.,
//! USENIX Security 2024 "SAILR").
//!
//! # The structural gap
//!
//! At `-O0` (and elsewhere) the classic guard shape —
//!
//! ```c
//!   if (cond) { body; return X; }
//!   return Y;
//! ```
//!
//! — compiles to a single **shared** RETURN block reached from several
//! predecessors (each `return` becomes `mov eax,..; jmp .Lepilogue`).  kuna, like
//! upstream Ghidra, keeps the merged form:
//! [`CollapseStructure::rule_block_or`](crate::blockaction) merges the guards into
//! one condition with comma-expression side effects
//! (`if ((A||B) && (v3=a0, v2=f(..), C||D)) { rest }`), because the two conditionals
//! *share an out-target* (the epilogue).  On `coreutils::factor` this collapses two
//! source early-return guards **and the intervening call** into one giant
//! short-circuit condition with a single trailing return — kuna 3 ifs / 0 early
//! returns vs the source's 4 ifs / early returns (GED 12).
//!
//! The existing [`ActionReturnSplit`](crate::blockaction::ActionReturnSplit)
//! (option-less, the port of upstream Ghidra's epilogue splitter) only splits a
//! shared RETURN block when [`Funcdata::gather_return_gotos`] finds **goto**-in
//! edges — i.e. it is the analog of angr's *goto-driven* `ReturnDuplicatorLow`.  On
//! the factor shape structuring is goto-free (the guards collapse cleanly into the
//! comma condition), so `gotoblocks.is_empty()` and the split never fires.  kuna has
//! had no analog of the **gotoless** `ReturnDuplicatorHigh`.
//!
//! # What this pass does
//!
//! Running once **before** the final [`ActionBlockStructure`](crate::blockaction)
//! (registered in the `returnsplit` group right after `ActionReturnSplit`, so its
//! duplicated returns flow through the same merge/naming passes and are re-structured
//! by the next `fullloop` iteration), [`ActionReturnDup`] duplicates every shared
//! **bare-epilogue** RETURN block into each of its predecessors but one.  It reuses,
//! verbatim, the machinery `ActionReturnSplit` already drives
//! ([`Funcdata::returndup_apply`] → the *existing*
//! [`return_split_is_splittable`](crate::funcdata::Funcdata) filter +
//! [`Funcdata::node_split`]):
//!
//! * for every RETURN block with `size_in > 1` that passes the **existing**
//!   `return_split_is_splittable` filter (only MULTIEQUAL/COPY/RETURN over
//!   constant/annotation/non-free inputs — a *bare epilogue*, exactly angr's
//!   "simple return graph"),
//! * split every in-edge but the first via [`Funcdata::node_split`] (the p-code
//!   clone + control-flow duplication `ActionReturnSplit` uses), giving each
//!   predecessor its own private return.
//!
//! With per-predecessor returns the two guards no longer share an out-target, so
//! `rule_block_or` cannot form the comma-merge and the structurer yields the source's
//! early-return `if`s naturally (and the default-on `ifelseflatten` flattens any
//! residual else-chains into guard-clause form).  The duplication is bounded — the
//! epilogue is return-only by construction (no calls/stores; the splittable filter
//! rejects anything else), growth is ~1 return per predecessor, capped at
//! [`MAX_RETURNDUP_SPLITS`] splits per function and skipping any block with more than
//! [`MAX_RETURNDUP_INEDGES`] predecessors (guard runaway).  Duplicating a bare
//! epilogue is semantics-preserving: no side-effecting op is cloned, and each
//! predecessor reaches a return with the same MULTIEQUAL-selected value it did before.
//!
//! It is gated by `option returndup on|off` (`duplicate_shared_returns`, DIV-54
//! default-ON, superseding the DIV-18 revert — the const-return gate is what turned
//! the aggregate decbench measurement positive) — when off, [`ActionReturnDup::apply`]
//! early-returns and output is byte-identical to upstream.  It is broad: any function
//! with a shared const bare epilogue gains early returns, so nine datatests pin the
//! merged upstream rendering with an explicit `option returndup off`.

use kuna_base::error::KunaResult;
use kuna_base::marshal::ElementId;
use kuna_base::types::int4;

use crate::action::{Action, ActionBase, ActionContext, ActionGroupList, ApplyResult};
use crate::funcdata::Funcdata;
use crate::options::on_or_off;

/// Marshaling element `<returndup>` (kuna).  ElementIds live in the 4000+ range
/// (4100 gotoreduce, 4101 tailcalljump, 4102 regionstructure, 4103 noreturn_extern,
/// 4104 noreturn_externmatch, 4105 crossjumprevert, 4106 switchsharedcase,
/// 4107 ifelseflatten, 4108 taildup, 4109 dedupitetail are taken; this is 4110).
pub const ELEM_RETURNDUP: ElementId = ElementId::new("returndup", 4135);

/// Maximum total return-block splits (`node_split`s) performed per function — the
/// per-function duplication cap that guards runaway on a function with many shared
/// bare epilogues.
pub const MAX_RETURNDUP_SPLITS: int4 = 64;
/// Maximum in-degree of a shared RETURN block this pass will duplicate — a block with
/// more predecessors than this is left merged (splitting it would clone the epilogue
/// once per predecessor, an unprofitable blow-up angr's duplicator also avoids).
pub const MAX_RETURNDUP_INEDGES: int4 = 16;

/// \brief Duplicate a shared **bare-epilogue** RETURN block into each predecessor but
/// one, so the classic `if (c) { body; return X; } return Y;` guard shape structures
/// as per-predecessor early returns instead of one merged, comma-folded exit (kuna's
/// analog of angr's SAILR gotoless `ReturnDuplicatorHigh`).  Option-gated
/// (`duplicate_shared_returns`).  Reuses the existing `return_split_is_splittable`
/// filter + `node_split` machinery that `ActionReturnSplit` drives.
pub struct ActionReturnDup {
    base: ActionBase,
}

impl ActionReturnDup {
    /// Construct in group `g`.
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionReturnDup { base: ActionBase::new(0, "returndup", g) })
    }
}

impl Action for ActionReturnDup {
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
        Some(Box::new(ActionReturnDup { base: self.base.clone() }))
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // P0 assertion `option returndup on|off`, carried on the Architecture boundary.
        if !data.get_arch().duplicate_shared_returns {
            return 0;
        }
        // The whole split loop (reuse of the existing `return_split_is_splittable`
        // filter + `node_split`) lives on Funcdata, next to `return_split_apply`,
        // where it has op + block + cloner access.
        let changes = data.returndup_apply(MAX_RETURNDUP_SPLITS, MAX_RETURNDUP_INEDGES);
        if changes > 0 {
            let name = data.get_name().to_string();
            data.warning_header(&format!(
                "returndup: duplicated {changes} shared bare-epilogue return(s) for early-return recovery in {name}"
            ));
        }
        self.base_mut().count += changes;
        0
    }
}

//===========================================================================
// Option parse (`returndup on|off`).
//===========================================================================

/// The `returndup on|off` ArchOption (angr SAILR gotoless `ReturnDuplicatorHigh`
/// shared-bare-epilogue return duplication).
pub struct OptionReturnDup;

impl OptionReturnDup {
    /// The option name.
    pub const NAME: &'static str = "returndup";

    /// Parse `on`/`off`, returning the bool value + a confirmation message.
    pub fn apply(&self, p1: &str) -> KunaResult<(bool, String)> {
        let val = on_or_off(p1)?;
        let prop = if val { "on" } else { "off" };
        Ok((val, format!("Shared bare-epilogue return duplication turned {prop}")))
    }
}
