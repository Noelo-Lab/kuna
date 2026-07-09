//! S8 **switch const-case return hoisting** — the direct continuation of
//! [`ActionEarlyReturn`](crate::p8_structure::kuna_earlyreturn), extended from the narrow
//! if/else-if diamond to the **multi-way switch** const-phi return (angr's SAILR
//! `ReturnDuplicatorHigh`, per-edge const narrowing; Basque et al., USENIX Security 2024).
//!
//! # The structural gap
//!
//! An enum/switch dispatch whose every case assigns a constant to a shared temp and breaks
//! to one exit return compiles to a merged phi + single return:
//!
//! ```c
//!   // source / angr                          // kuna today (the merged switch)
//!   switch (x) {                               switch (x) {
//!     case A: return K0;                         case A: v = K0; break;
//!     case B: return K1;                         case B: v = K1; break;
//!     ...                                        ...
//!   }                                          }
//!                                              return v;
//! ```
//!
//! The shared RETURN block is `v = MULTIEQUAL(#K0, #K1, …); return v` — an **all-constant**
//! (or per-case-constant) phi whose input slots align 1:1 with the switch cases.  kuna keeps
//! the merged `return v` plus one `break` per case, ~2 CFG nodes per case more than the
//! source's per-case `return K` (the GED gap on e.g. libedit `tty__getcharindex`, 17 cases).
//!
//! # Why the shipped `earlyreturn` cannot reach it
//!
//! [`ActionEarlyReturn`](crate::p8_structure::kuna_earlyreturn) already runs the exact same
//! per-edge const-peel machinery ([`Funcdata::earlyreturn_const_edges`] +
//! [`Funcdata::node_split`]) and DOES recover per-case returns for a switch whose merge block
//! has ≤ `MAX_EARLYRETURN_INEDGES` (16) predecessors (e.g. findutils `get_fts_info_name`, a
//! 14-case switch, hoists to `case 1: return "FTS_D"; …`).  But its in-degree cap leaves the
//! **wide** dispatch table untouched: a 17-case switch merges 17 in-edges (> 16), so
//! earlyreturn skips it and kuna keeps the `v = K; break; … return v;` shape.
//!
//! # What this pass does
//!
//! [`ActionSwitchReturn`] is the same peel with the in-edge cap lifted
//! ([`MAX_SWITCHRETURN_INEDGES`]) so the many-case switch-phi is reached:
//! [`Funcdata::switchreturn_apply`] splits each constant in-edge (all but the last, which
//! keeps the block alive) via [`Funcdata::node_split`], so every case ends in its own
//! `return K`.  No new tree code — it delegates to the same
//! [`Funcdata::const_return_peel`](crate::funcdata::Funcdata) core `earlyreturn` uses.
//!
//! Because it only ever splits a **constant** case arm (never a `return <variable>` share),
//! it inherits `earlyreturn`'s safety and **cannot** re-introduce the variable-return
//! over-firing that made broad `returndup` regress the aggregate GED-perfect count (~976): a
//! non-constant arm (the switch body / fall-through / default that computes a value) is left
//! merged.  It is orthogonal to `earlyreturn` (a separate option): when both are on the
//! narrower `earlyreturn` consumes the ≤16 cases first, so `switchreturn`'s incremental
//! effect is precisely the wide dispatch tables earlyreturn's cap skips.
//!
//! Bounded: the epilogue is return-only by construction (the splittable filter rejects
//! calls/stores), it never splits the last remaining edge (the block always keeps a
//! predecessor), and it is capped per function ([`MAX_SWITCHRETURN_SPLITS`]) and per block
//! in-degree ([`MAX_SWITCHRETURN_INEDGES`]).  Gated by `option switchreturn on|off`
//! (`switch_return`, default-ON per DIV-25); when off, [`ActionSwitchReturn::apply`] early-returns and
//! output is byte-identical.

use kuna_base::error::KunaResult;
use kuna_base::marshal::ElementId;
use kuna_base::types::int4;

use crate::action::{Action, ActionBase, ActionContext, ActionGroupList, ApplyResult};
use crate::funcdata::Funcdata;
use crate::options::on_or_off;

/// Marshaling element `<switchreturn>` (kuna).  ElementIds live in the 4000+ range
/// (…4110 returndup, 4111 earlyreturn taken; this is 4112).
pub const ELEM_SWITCHRETURN: ElementId = ElementId::new("switchreturn", 4112);

/// Maximum total const-case peels (`node_split`s) performed per function.
pub const MAX_SWITCHRETURN_SPLITS: int4 = 256;
/// Maximum in-degree of a shared RETURN block this pass will peel — the wide switch
/// dispatch table earlyreturn's 16-edge cap skips (a full byte-dispatch fits under 256).
pub const MAX_SWITCHRETURN_INEDGES: int4 = 256;

/// \brief Peel the constant case arms out of a wide **multi-way switch-phi** shared RETURN
/// block into per-case `return K`, so a `switch (x) { case A: v = K0; break; … } return v;`
/// dispatch recovers as `case A: return K0; …` (kuna's continuation of angr's SAILR
/// `ReturnDuplicatorHigh` past `earlyreturn`'s 16-in-edge cap).  Option-gated
/// (`switch_return`, default-off).  Reuses the existing `return_split_is_splittable` filter
/// + `earlyreturn_const_edges` + `node_split` machinery; leaves any non-const body arm merged.
pub struct ActionSwitchReturn {
    base: ActionBase,
}

impl ActionSwitchReturn {
    /// Construct in group `g`.
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionSwitchReturn { base: ActionBase::new(0, "switchreturn", g) })
    }
}

impl Action for ActionSwitchReturn {
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
        Some(Box::new(ActionSwitchReturn { base: self.base.clone() }))
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // P0 assertion `option switchreturn on|off`, carried on the seam Architecture.
        if !data.get_arch().switch_return {
            return 0;
        }
        let changes = data.switchreturn_apply(MAX_SWITCHRETURN_SPLITS, MAX_SWITCHRETURN_INEDGES);
        if changes > 0 {
            let name = data.get_name().to_string();
            data.warning_header(&format!(
                "switchreturn: hoisted {changes} const-case return(s) in {name}"
            ));
        }
        self.base_mut().count += changes;
        0
    }
}

//===========================================================================
// Option parse (`switchreturn on|off`).
//===========================================================================

/// The `switchreturn on|off` ArchOption (angr SAILR `ReturnDuplicatorHigh`, per-edge
/// const narrowing extended to the wide multi-way switch-phi that overflows
/// `earlyreturn`'s in-edge cap).
pub struct OptionSwitchReturn;

impl OptionSwitchReturn {
    /// The option name.
    pub const NAME: &'static str = "switchreturn";

    /// Parse `on`/`off`, returning the bool value + a confirmation message.
    pub fn apply(&self, p1: &str) -> KunaResult<(bool, String)> {
        let val = on_or_off(p1)?;
        let prop = if val { "on" } else { "off" };
        Ok((val, format!("Switch const-case return hoisting turned {prop}")))
    }
}
