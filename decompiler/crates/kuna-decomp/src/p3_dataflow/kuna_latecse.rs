//! P3 late common-subexpression elimination (kuna `latecse`).
//!
//! Upstream merges two ops that compute the same value from the same inputs
//! only in a few places: `ActionMultiCse` for MULTIEQUALs and `RuleSelectCse`
//! for SUBPIECE / INT_SRIGHT.  An arithmetic or logical expression that the
//! source wrote once, and the compiler or the simplification rules duplicated,
//! prints twice, and the two copies become separate temporaries.  This action
//! runs `Funcdata::cse_eliminate_list` over the readers of every varnode with
//! more than one reader, for every unary or binary op `get_cse_hash` accepts
//! (no COPY, LOAD, call or other effect), after the simplification rules have
//! settled.  `cse_elimination` keeps one op, in a block that dominates both
//! uses, and reads its output at both.
//!
//! Gated by option `latecse`; in group `unreachable` (decompile, normalize,
//! paramid), not the reduced jump-table pipeline.  It runs once per function:
//! a rule that later splits a shared value again would otherwise make the
//! repeating fullloop alternate with it forever.

use crate::action::{Action, ActionBase, ActionContext, ActionGroupList, ApplyResult};
use crate::context::{OpId, VarnodeId};
use crate::funcdata::Funcdata;

/// Is `op`'s value used as a LOAD or STORE address?  Type recovery turns each
/// such use into its own field or element access, which one shared temporary
/// would hide.
fn feeds_address(data: &Funcdata, op: OpId) -> bool {
    let Some(out) = data.obank().get(op).and_then(|o| o.get_out()).and_then(|v| data.vbank().get(v)) else { return false };
    out.descend_iter().any(|d| {
        data.obank().get(d).is_some_and(|u| {
            matches!(u.code(), kuna_num::opcodes::OpCode::CPUI_LOAD | kuna_num::opcodes::OpCode::CPUI_STORE)
                && u.get_in(1) == o_out(data, op)
        })
    })
}

fn o_out(data: &Funcdata, op: OpId) -> Option<VarnodeId> {
    data.obank().get(op).and_then(|o| o.get_out())
}

/// Merge every pair of identical pure ops reading one varnode; returns how many merged.
pub(crate) fn latecse_apply(data: &mut Funcdata) -> i32 {
    let shared: Vec<VarnodeId> = data
        .vbank()
        .iter_def()
        .filter(|&vn| data.vbank().get(vn).is_some_and(|v| v.descend_iter().nth(1).is_some()))
        .collect();
    let mut count = 0;
    for vn in shared {
        let Some(v) = data.vbank().get(vn) else { continue };
        let mut list: Vec<(kuna_base::types::uintm, OpId)> = v
            .descend_iter()
            .filter_map(|op| {
                let o = data.obank().get(op)?;
                if feeds_address(data, op) {
                    return None;
                }
                let hash = crate::op::get_cse_hash(o, data.vbank());
                (hash != 0).then_some((hash, op))
            })
            .collect();
        if list.len() < 2 {
            continue;
        }
        let mut merged = Vec::new();
        let ok = data.cse_eliminate_list(&mut list, &mut merged, |fd, out| {
            let addr = fd.vbank().get(out).map(|v| v.get_addr().clone());
            addr.is_some_and(|a| fd.heritage_pass(&a) >= 0)
        });
        if ok.is_ok() {
            count += merged.len() as i32;
        }
    }
    count
}

/// How many times the merge is repeated: merging two reads of a value exposes
/// the identical ops that read their outputs (`SUBPIECE`, then the `INT_ADD`,
/// then the `INT_MULT` over it), one layer a round.
const MAX_ROUNDS: usize = 8;

/// Runs [`latecse_apply`] to a fixpoint when option `latecse` is on.
pub struct ActionLateCse {
    base: ActionBase,
}

impl ActionLateCse {
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionLateCse { base: ActionBase::new(crate::action::ruleflags::rule_onceperfunc, "latecse", g) })
    }
}

impl Action for ActionLateCse {
    fn base(&self) -> &ActionBase {
        &self.base
    }
    fn base_mut(&mut self) -> &mut ActionBase {
        &mut self.base
    }
    fn clone_filtered(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Action>> {
        grouplist
            .contains(self.get_group())
            .then(|| Box::new(ActionLateCse { base: self.base.clone() }) as Box<dyn Action>)
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        if !data.get_arch().late_cse {
            return 0;
        }
        for _ in 0..MAX_ROUNDS {
            let merged = latecse_apply(data);
            self.base.count += merged;
            if merged == 0 {
                break;
            }
        }
        0
    }
}
