//! (kuna `paramcopyhoist`) Re-anchor the copy-shadow of an unmodified incoming
//! parameter in the function's entry block (P6, after HighVariable formation).
//!
//! ## The construct
//!
//! `gcc -O0` gives every parameter a stack home and, when the source guards the
//! parameter (`if (p && !*p) p = NULL;`), the guard's join carries a MULTIEQUAL
//! whose incoming slots are the raw input register.  `Merge::mergeOp` cannot merge
//! an input-register HighVariable with the address-tied stack-slot HighVariable, so
//! `Merge::trimOpInput` snips each offending slot into a COPY and places it at the
//! *stop address of that slot's predecessor block*
//! (`opInsertEnd(copyop, op->getParent()->getIn(slot))`).  The COPY's output later
//! merges into the stack-slot high, so the COPY is what prints as `vN = aM;`.
//!
//! When the parameter's guard is not the first one in the cascade, that predecessor
//! is not the entry block and the copy-shadow renders *below* an earlier guard:
//!
//! ```text
//!   v7 = a0;                      // entry block (parameter 1: trim sites {BB0,BB1}, common dominator = entry)
//!   if ((a0) && (!*a0)) v7 = NULL;
//!   v6 = a1;                      // NOT the entry block -> +1 CFG node
//!   if ((a1) && (!*a1)) v6 = NULL;
//!   v5 = a2;                      // NOT the entry block -> +1 CFG node
//! ```
//!
//! kuna is internally inconsistent about the identical construct: the discriminator
//! is only whether the guard's join happens to be the entry block.  The source's
//! entry block holds all the spills, so each sunk copy splits one source block in two.
//!
//! Two producing sub-cases both end here.  With **two or more** trimmed slots each
//! gets its own COPY and `Merge::processCopyTrims` -> `Merge::buildDominantCopy`
//! collapses them into one COPY at `FlowBlock::findCommonBlock` of the trim sites (a
//! pure `immed_dom`-chain intersection).  With **exactly one** trimmed slot no
//! dominant-copy pass runs and the single `opInsertEnd` is the final placement.
//!
//! ## Why this runs after the merge, not inside `trimOpInput`
//!
//! Moving the placement decision into `Merge::trimOpInput` itself is not sound, and
//! measurably breaks code.  `trimOpInput` exists precisely to give the snipped slot a
//! *tiny* Cover so the following `Merge::mergeOp` cover test passes; widening it there
//! makes that test fail, and `mergeOp` then falls through to `trimOpOutput` ("one last
//! trim"), restructuring the MULTIEQUAL into `u = MULTIEQUAL(...); dest = u;` whose two
//! COPYs `markInternalCopies` then hides — so the assignment disappears from the
//! emitted C entirely (measured on e2fsprogs `e2fsck` `ext2fs_read_inode2`, where
//! `v10 = a3;` vanished and left an empty `if` body).  The legality question is also
//! unanswerable at that point: `mergeAddrTied` has run but `ActionMergeCopy` has not,
//! so the stack slot's other definitions are still in a *different* HighVariable and an
//! entry-block clobber is invisible.
//!
//! [`ActionParamCopyHoist`] therefore runs after `ActionCopyMarker`, when every
//! HighVariable is final: no merge decision is perturbed, and the Cover test sees the
//! variable's complete live range.
//!
//! ## Legality
//!
//! Relocating the COPY widens its output's live range from "tail of one predecessor"
//! to "entry block onward".  That output is an instance of the target HighVariable, so
//! the widened range must not overlap a range the variable already holds — otherwise
//! the hoisted assignment clobbers a live value.
//!
//! The test is the one `Merge::buildDominantCopy` applies to its own redirects, run
//! against the hypothetical hoisted placement: `b_cover` is the union of the target
//! high's instance Covers (skipping the moving Varnode itself and any COPY instance
//! that copy-shadows the same root), `a_cover` is the Cover the COPY would have if
//! defined in the entry block and read where its output is read today, and an overlap
//! of more than a def/use boundary (`Cover::intersect > 1`) rejects the hoist.
//! `a_cover`'s def point is taken at the *start* of the entry block rather than at the
//! real insertion point (just above the terminating branch), which over-approximates
//! the widened range, so the test errs toward rejecting.
//!
//! Only a `Varnode::isInput` root is hoisted -- a function input is defined at entry
//! and live from there, so availability is never in question -- and only when every
//! read of the COPY's output is a MULTIEQUAL, which is exactly the `trimOpInput`
//! MULTIEQUAL arm.  The INDIRECT arm (`Merge::mergeIndirect`'s call-adjacent copy) is
//! a different Cover shape and is deliberately left alone.

use kuna_base::error::KunaResult;
use kuna_base::marshal::ElementId;
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::action::{Action, ActionBase, ActionContext, ActionGroupList, ApplyResult};
use crate::context::{BlockId, HighVariableId, OpId, VarnodeId};
use crate::cover::{Cover, CoverPoint};
use crate::funcdata::Funcdata;
use crate::options::on_or_off;

/// Marshaling element `<paramcopyhoist>` (kuna 4000+ range; 4119 = iteboolean).
pub const ELEM_PARAMCOPYHOIST: ElementId = ElementId::new("paramcopyhoist", 4120);

/// (kuna) The `paramcopyhoist on|off` ArchOption.
pub struct OptionParamCopyHoist;

impl OptionParamCopyHoist {
    /// The option name.
    pub const NAME: &'static str = "paramcopyhoist";

    /// Parse `on`/`off`, returning the bool value + a confirmation message.
    pub fn apply(&self, p1: &str) -> KunaResult<(bool, String)> {
        let val = on_or_off(p1)?;
        let prop = if val { "on" } else { "off" };
        Ok((val, format!("Parameter copy-shadow entry-block hoist turned {prop}")))
    }
}

/// \brief Re-anchor an unmodified incoming parameter's trim COPY in the entry block
/// so its copy-shadow prints with the other parameter spills instead of below an
/// earlier guard.  Option-gated (`paramcopyhoist`); with the option off
/// [`ActionParamCopyHoist::apply`] early-returns and output is byte-identical.
pub struct ActionParamCopyHoist {
    base: ActionBase,
}

impl ActionParamCopyHoist {
    /// Construct in group `g`.
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionParamCopyHoist { base: ActionBase::new(0, "paramcopyhoist", g) })
    }
}

impl Action for ActionParamCopyHoist {
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
        Some(Box::new(ActionParamCopyHoist { base: self.base.clone() }))
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // P0 assertion `option paramcopyhoist on|off`, carried on the Architecture
        // boundary.  Off => nothing moves => byte-identical output.
        if !data.get_arch().param_copy_hoist {
            return 0;
        }
        if data.bblocks_get_size() < 2 {
            return 0;
        }
        let entry = data.bblocks_get_block(0);
        let candidates: Vec<OpId> =
            data.obank().iter_alive().filter(|&op| is_hoistable(data, op, entry)).collect();
        let candidates = one_per_high(data, candidates);
        if candidates.is_empty() {
            return 0;
        }
        let count = candidates.len() as int4;
        for op in candidates {
            data.op_uninsert(op);
            data.op_insert_end(op, entry);
        }
        let name = data.get_name().to_string();
        data.warning_header(&format!(
            "paramcopyhoist: anchored {count} parameter copy-shadow(s) in the entry block of {name}"
        ));
        self.base_mut().count += count;
        0
    }
}

/// Drop every candidate belonging to a HighVariable that has more than one.
///
/// The Cover test judges each move against the placements the *other* definitions
/// have today, so it cannot see two definitions of one variable converging on the
/// entry block: each is dead where the other is live, they never intersect, and yet
/// after both moves the second write kills the first on every path.  Measured on
/// e2fsck `sub_7431a`, where `v1 = a1;` and `v1 = a2;` both hoisted and the function
/// started returning `a2` unconditionally.  A parameter copy-shadow is one COPY into
/// one variable, so requiring exactly one candidate per high costs nothing on the
/// construct and closes the hole.
fn one_per_high(fd: &Funcdata, candidates: Vec<OpId>) -> Vec<OpId> {
    let high_of = |op: OpId| {
        fd.obank().get(op).and_then(|o| o.get_out()).and_then(|v| fd.vbank().get(v)?.get_high())
    };
    candidates
        .iter()
        .copied()
        .filter(|&op| {
            let h = high_of(op);
            h.is_some() && candidates.iter().filter(|&&o| high_of(o) == h).count() == 1
        })
        .collect()
}

/// Is `op` a printing COPY of a function input whose output can legally be defined in
/// the entry block instead of where it sits now?
fn is_hoistable(fd: &Funcdata, op: OpId, entry: BlockId) -> bool {
    let Some(o) = fd.obank().get(op) else { return false };
    if o.code() != OpCode::CPUI_COPY || o.not_printed() {
        return false;
    }
    let (Some(root_vn), Some(out_vn), Some(parent)) = (o.get_in(0), o.get_out(), o.get_parent())
    else {
        return false;
    };
    if parent == entry {
        return false;
    }
    if !is_incoming_parameter(fd, root_vn) {
        return false;
    }
    if !fd.bblocks_ref().dominates(entry, Some(parent)) {
        return false;
    }
    // The MULTIEQUAL arm of `Merge::trimOpInput` only: the copy must feed a
    // guard-join phi, and may otherwise only be consumed by the INDIRECTs heritage
    // threaded through the same storage.  A copy read by no MULTIEQUAL is
    // `Merge::mergeIndirect`'s call-adjacent snip, a different Cover shape that is
    // deliberately left alone; a copy with an ordinary value read is a computation,
    // not a copy-shadow.
    let reads: Vec<OpId> =
        fd.vbank().get(out_vn).map(|v| v.descend_iter().collect()).unwrap_or_default();
    let code_of = |r: OpId| fd.obank().get(r).map(|x| x.code());
    if !reads.iter().any(|&r| code_of(r) == Some(OpCode::CPUI_MULTIEQUAL))
        || !reads.iter().all(|&r| {
            matches!(code_of(r), Some(OpCode::CPUI_MULTIEQUAL) | Some(OpCode::CPUI_INDIRECT))
        })
    {
        return false;
    }
    let Some(high) = fd.vbank().get(out_vn).and_then(|v| v.get_high()) else { return false };
    cover_admits_hoist(fd, high, out_vn, root_vn, &reads, entry)
}

/// Is `vn` a function input Varnode occupying a **formal parameter's** storage?
///
/// `Varnode::isInput` alone is too broad — a global or a stack location read before
/// it is written is also an SSA input, and hoisting those is outside this feature's
/// measured scope (and outside what the option's name promises).
fn is_incoming_parameter(fd: &Funcdata, vn: VarnodeId) -> bool {
    let Some(v) = fd.vbank().get(vn) else { return false };
    if !v.is_input() {
        return false;
    }
    let (addr, size) = (v.get_addr().clone(), v.get_size());
    let proto = fd.get_func_proto();
    (0..proto.num_params()).any(|i| {
        proto
            .get_param(i)
            .map(|p| p.get_address() == addr && p.get_size() == size)
            .unwrap_or(false)
    })
}

/// The `buildDominantCopy` Cover test applied to the hypothetical entry-block
/// placement: `false` when the widened range would overlap the target high.
fn cover_admits_hoist(
    fd: &Funcdata,
    high: HighVariableId,
    out_vn: VarnodeId,
    root_vn: VarnodeId,
    reads: &[OpId],
    entry: BlockId,
) -> bool {
    let ctx = crate::funcdata::FuncdataCoverCtx { fd };

    let mut b_cover = Cover::new();
    let n = fd.high_bank().get(high).map(|h| h.num_instances()).unwrap_or(0);
    for i in 0..n {
        let Some(vn) = fd.high_bank().get(high).map(|h| h.get_instance(i)) else { return false };
        if vn == out_vn || instance_shadows_root(fd, vn, root_vn) {
            continue;
        }
        let mut vc = Cover::new();
        vc.rebuild(&ctx, vn);
        b_cover.merge(&vc);
    }

    let mut a_cover = Cover::new();
    let entry_index = fd.bblocks_ref().block(entry).get_index();
    a_cover.add_def_point(Some((entry_index, CoverPoint::Begin)), false);
    for &refop in reads {
        a_cover.add_ref_point_for(&ctx, refop, out_vn);
    }

    b_cover.intersect(&a_cover) <= 1
}

/// `buildDominantCopy`'s instance filter: a COPY whose input copy-shadows the trim
/// root carries the same value, so it contributes no independent range.
fn instance_shadows_root(fd: &Funcdata, vn: VarnodeId, root_vn: VarnodeId) -> bool {
    let Some(v) = fd.vbank().get(vn) else { return false };
    if !v.is_written() {
        return false;
    }
    let Some(def) = v.get_def() else { return false };
    if fd.obank().get(def).map(|o| o.code()) != Some(OpCode::CPUI_COPY) {
        return false;
    }
    match fd.obank().get(def).and_then(|o| o.get_in(0)) {
        Some(in0) => fd.varnode_copy_shadow(in0, root_vn),
        None => false,
    }
}
