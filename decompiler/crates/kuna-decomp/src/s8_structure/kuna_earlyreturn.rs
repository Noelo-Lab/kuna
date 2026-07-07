//! S8 **early-return guard hoisting** (port of angr's SAILR `ReturnDuplicatorHigh`,
//! narrowed to the *per-edge* const case — the "else simplification" / early-return
//! guard shape; Basque et al., USENIX Security 2024 "SAILR").
//!
//! # The structural gap
//!
//! A function whose first action is an argument-validity check compiles to a leading
//! guard whose false arm just returns a default, merged with the body's own return:
//!
//! ```c
//!   // source / angr                       // kuna today (the merged diamond)
//!   if (!cur) return 0;                     if (cur != 0) { ...body... }
//!   ...body de-indented...                  else { v = 0; }
//!   return <var>;                           return v;
//! ```
//!
//! The shared RETURN block is `v = MULTIEQUAL(#0 /*guard-false arm*/, <var> /*body*/);
//! return v` — a **mixed** phi: one input a constant (the guard's `return 0`), the
//! other a variable (the body's computed return).  kuna keeps the merged diamond and
//! `return v` (one return, extra CFG merge nodes that inflate GED).
//!
//! # Why the existing `returndup` cannot reach it
//!
//! [`ActionReturnDup`](crate::s8_structure::kuna_returndup) (option `returndup`) gates on
//! [`Funcdata::returndup_is_const_ret`](crate::funcdata::Funcdata) — the **whole** return
//! value must be constant.  That gate exists because broad, variable-return duplication
//! diverged from the source's merged short-circuit form on the aggregate (decbench: ~976
//! GED-perfect regression across 21768 firings).  But requiring the *whole* value const
//! also rejects the mixed diamond above (its body arm is a variable), so the common
//! early-return-guard shape is never peeled.  angr does not have this problem: its
//! `ReturnDuplicatorHigh._is_simple_return_graph` peels **per predecessor** and admits a
//! `return <vvar>` body arm.
//!
//! # What this pass does
//!
//! [`ActionEarlyReturn`] runs the *same* [`Funcdata::node_split`] machinery as `returndup`,
//! but drives it **per-edge**: [`Funcdata::earlyreturn_apply`] splits only the in-edges
//! whose contribution to the return phi is a **constant** (the guard arm), leaving the
//! variable body arm merged.  On `coreutils::realpath::relpath` this peels the `return 0`
//! guard into its own block; with per-predecessor returns the guard no longer shares an
//! out-target, so the default-on `branchflip` orients it as the `if`-true and the
//! default-on `ifelseflatten` drops the else and de-indents the body — yielding angr's
//! exact `if (c) return 0; ...body...; return <var>;`.  No new tree code.
//!
//! Because it only ever splits a **constant** arm (never a `return <variable>` share), it
//! **cannot** re-introduce the variable-return over-firing that made broad `returndup`
//! regress the aggregate; it targets the mixed const/variable diamond the whole-block gate
//! structurally cannot reach.  It is orthogonal to `returndup` — a separate option.
//!
//! Bounded: the epilogue is return-only by construction (the splittable filter rejects
//! calls/stores), it never splits the last remaining edge (the body always keeps its
//! return), and it is capped per function ([`MAX_EARLYRETURN_SPLITS`]) and per block
//! in-degree ([`MAX_EARLYRETURN_INEDGES`]).  Gated by `option earlyreturn on|off`
//! (`early_return`, default-OFF); when off, [`ActionEarlyReturn::apply`] early-returns and
//! output is byte-identical.

use kuna_base::error::KunaResult;
use kuna_base::marshal::ElementId;
use kuna_base::types::int4;

use crate::action::{Action, ActionBase, ActionContext, ActionGroupList, ApplyResult};
use crate::funcdata::Funcdata;
use crate::options::on_or_off;

/// Marshaling element `<earlyreturn>` (kuna).  ElementIds live in the 4000+ range
/// (…4109 dedupitetail, 4110 returndup taken; this is 4111).
pub const ELEM_EARLYRETURN: ElementId = ElementId::new("earlyreturn", 4111);

/// Maximum total const-arm peels (`node_split`s) performed per function.
pub const MAX_EARLYRETURN_SPLITS: int4 = 32;
/// Maximum in-degree of a shared RETURN block this pass will peel — a block with more
/// predecessors than this is left merged.
pub const MAX_EARLYRETURN_INEDGES: int4 = 16;

/// \brief Peel the constant guard arms out of a shared RETURN block into per-predecessor
/// `return K`, so a leading `if (!cur) return 0;` guard recovers as an early return with a
/// de-indented body (kuna's per-edge analog of angr's SAILR `ReturnDuplicatorHigh`).
/// Option-gated (`early_return`).  Reuses the existing `return_split_is_splittable` filter
/// + `node_split` machinery; leaves the variable body return merged.
pub struct ActionEarlyReturn {
    base: ActionBase,
}

impl ActionEarlyReturn {
    /// Construct in group `g`.
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionEarlyReturn { base: ActionBase::new(0, "earlyreturn", g) })
    }
}

impl Action for ActionEarlyReturn {
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
        Some(Box::new(ActionEarlyReturn { base: self.base.clone() }))
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // P0 assertion `option earlyreturn on|off`, carried on the seam Architecture.
        if !data.get_arch().early_return {
            return 0;
        }
        let changes = data.earlyreturn_apply(MAX_EARLYRETURN_SPLITS, MAX_EARLYRETURN_INEDGES);
        if changes > 0 {
            let name = data.get_name().to_string();
            data.warning_header(&format!(
                "earlyreturn: hoisted {changes} const-guard early-return(s) in {name}"
            ));
        }
        self.base_mut().count += changes;
        0
    }
}

//===========================================================================
// Option parse (`earlyreturn on|off`).
//===========================================================================

/// The `earlyreturn on|off` ArchOption (angr SAILR `ReturnDuplicatorHigh`, per-edge
/// const-guard early-return hoisting).
pub struct OptionEarlyReturn;

impl OptionEarlyReturn {
    /// The option name.
    pub const NAME: &'static str = "earlyreturn";

    /// Parse `on`/`off`, returning the bool value + a confirmation message.
    pub fn apply(&self, p1: &str) -> KunaResult<(bool, String)> {
        let val = on_or_off(p1)?;
        let prop = if val { "on" } else { "off" };
        Ok((val, format!("Const-guard early-return hoisting turned {prop}")))
    }
}
