//! Port of the **S6/S7 cleanup + merge + marking** Action classes from
//! `decompiler/cpp/coreaction.{cc,hh}`.
//!
//! # What this is
//!
//! This file is the W7 `w7-s37-coreaction-cleanup` item: the *merge group* of
//! [`Action`](crate::action::Action) classes — the actions the C++
//! `ActionDatabase::universalAction` schedules under group `"merge"` (plus the
//! single `"casts"`-group `ActionSetCasts`).  Where
//! [`coreaction_early`](crate::coreaction_early) covers the early S2/S3 plane and
//! [`coreaction_protos`](crate::coreaction_protos) the S4 prototype plane, this
//! file picks up the S6 HighVariable-formation pass (the `data.getMerge().*`
//! delegations) and the S7-adjacent marking/cast passes that decide explicit vs.
//! implied Varnodes and place casts.
//!
//! Each is an [`Action`] trait impl per the `action.rs` registration
//! convention: it embeds an [`ActionBase`] (the engine-owned name/group/flags/
//! status/breakpoint/counter store), keeps the **exact** `name()` string and
//! group/flags the C++ constructor used, and a `clone_filtered` mirroring the
//! C++ `clone(grouplist)` group filter.  Change signalling is via
//! `base_mut().count += 1` (the C++ `count += 1`).
//!
//! # Class list (this item), in C++ schedule order (`coreaction.cc:6002-6028`)
//!
//! | C++ class | `name()` | flags | C++ `apply` body |
//! |---|---|---|---|
//! | `ActionAssignHigh` | `"assignhigh"` | `rule_onceperfunc` | `data.setHighLevel()` (**realized**) |
//! | `ActionMergeRequired` | `"mergerequired"` | `rule_onceperfunc` | `merge.mergeAddrTied/groupPartials/mergeMarker` (seam) |
//! | `ActionMarkExplicit` | `"markexplicit"` | `rule_onceperfunc` | `coreaction.cc:3340` (seam) |
//! | `ActionMarkImplied` | `"markimplied"` | `rule_onceperfunc` | `coreaction.cc:3519` (seam) |
//! | `ActionMergeMultiEntry` | `"mergemultientry"` | `rule_onceperfunc` | `merge.mergeMultiEntry()` (seam) |
//! | `ActionMergeCopy` | `"mergecopy"` | `rule_onceperfunc` | `merge.mergeOpcode(CPUI_COPY)` (seam) |
//! | `ActionDominantCopy` | `"dominantcopy"` | `rule_onceperfunc` | `merge.processCopyTrims()` (seam) |
//! | `ActionMarkIndirectOnly` | `"markindirectonly"` | `rule_onceperfunc` | `data.markIndirectOnly()` (seam) |
//! | `ActionMergeAdjacent` | `"mergeadjacent"` | `rule_onceperfunc` | `merge.mergeAdjacent()` (seam) |
//! | `ActionMergeType` | `"mergetype"` | `rule_onceperfunc` | `merge.mergeByDatatype(beginLoc,endLoc)` (seam) |
//! | `ActionHideShadow` | `"hideshadow"` | `rule_onceperfunc` | `coreaction.cc:5085` (seam) |
//! | `ActionCopyMarker` | `"copymarker"` | `rule_onceperfunc` | `merge.markInternalCopies()` (seam) |
//! | `ActionNameVars` | `"namevars"` | `rule_onceperfunc` | `coreaction.cc:3076` (seam) |
//! | `ActionSetCasts` | `"setcasts"` | `rule_onceperfunc` | `coreaction.cc:2812` (seam) |
//!
//! All fourteen are `rule_onceperfunc` in the C++ constructors.  Group strings in
//! the schedule are all `"merge"` **except** `ActionSetCasts` (`"casts"`); but the
//! group a leaf belongs to is supplied by the *caller* (the W8 assembler passes
//! the schedule's group string), so each constructor here takes `g` and stores it
//! verbatim — exactly as the C++ ctor `Action(flags,name,g)` does.
//!
//! # Where the block-structuring wrappers live (NOT here)
//!
//! The item brief names `ActionBlockStructure`/`ActionFinalStructure` wrappers and
//! the structuring engine.  Those concrete `Action` classes are defined in
//! **`blockaction.{cc,hh}`** (not `coreaction.cc`) and are **already ported** in
//! [`blockaction`](crate::blockaction) by the merged dependency (`ActionBlockStructure`,
//! `ActionFinalStructure`, `ActionStructureTransform`, `ActionNormalizeBranches`,
//! `ActionPreferComplement`, `ActionReturnSplit`, `ActionNodeJoin`).  This item
//! does **not** re-port them; it slots between them in the W8 schedule
//! (`coreaction.cc:5999-6029`).  Likewise `ActionDynamicSymbols`/`ActionMapGlobals`
//! interleave in the same schedule region but belong to the S5 dynamic/global
//! symbol plane (not the merge group) and are left for their own items.
//!
//! # Seams (the `Funcdata`<->`Merge`/HighVariable/Cast bridge is not in the tree)
//!
//! The S6 [`Merge`](crate::merge::Merge) engine **is** fully ported in the merged
//! `merge.rs`, but it is a *standalone* engine driven through a
//! [`MergeContext`](crate::merge::MergeContext) trait — and `Funcdata` does **not**
//! yet implement `MergeContext`, nor does it hold a `Merge` instance or expose a
//! `getMerge()` accessor (the C++ `Funcdata::covermerge`).  Wiring that bridge
//! (the `MergeContext for Funcdata` impl + the `Funcdata::covermerge` field) is a
//! later wave that lives in `funcdata.rs`/`merge.rs`, which this item does not own.
//! Until then, the merge-delegation bodies cannot run.
//!
//! Following the established `coreaction_early`/`coreaction_protos` convention,
//! each unrealized body:
//!
//! 1. transcribes the C++ `apply` structure verbatim **as commented pseudocode**
//!    (same iteration order, tie-breakers, and `count += 1` points), and
//! 2. routes the unrealized mutation through a `// SEAM(W7/W8-funcdata)` note and
//!    returns `0` changes (the C++ contract: changes are signalled by `count`).
//!
//! The marking/cast bodies (`ActionMarkExplicit`/`ActionMarkImplied`/
//! `ActionNameVars`/`ActionSetCasts`/`ActionHideShadow`) additionally depend on
//! Varnode loc/def-set iteration (`beginLoc`/`beginDef`), `getArch()`-borne
//! tunables (`max_implied_ref`/`max_term_duplication`), the HighVariable type
//! surface, the `CastStrategy`/`print` rendering machinery, and the
//! symbol/scope/callspec link surface — none present in the merged tree — so they
//! are seamed in full.  Each seam is reported in this item's `losses` so the
//! owning wave finishes the wiring by replaying the commented body against the
//! real accessors.
//!
//! Exactly one body is **realized**: [`ActionAssignHigh`] calls the merged
//! `Funcdata::set_high_level` and is exercised by a test.
//!
//! # Registration
//!
//! W8 assembles `universalAction`; this file's leaf constructors plug into it via
//! [`ActionGroup::add_action`](crate::action::ActionGroup::add_action).  The
//! constructor convention mirrors the C++ `Action(flags,name,group)` ctor: each
//! type exposes `boxed(group)` returning the boxed action.  [`merge_actions`]
//! enumerates the full merge-group set in C++ schedule order for the W8 assembler.

use kuna_num::opcodes::OpCode;

use crate::action::{ruleflags, Action, ActionBase, ActionContext, ActionGroupList, ApplyResult};
use crate::funcdata::Funcdata;

// =============================================================================
// ActionAssignHigh (coreaction.hh:340) — REALIZED
// =============================================================================

/// Assign initial high-level HighVariable objects to each Varnode (C++
/// `ActionAssignHigh`, `coreaction.hh:340`).
///
/// `apply` is `data.setHighLevel(); return 0;`.  The merged
/// [`Funcdata::set_high_level`](crate::funcdata::Funcdata::set_high_level) is
/// realized, so this body runs for real (one of the merge group's two
/// `rule_onceperfunc` setup passes).  It signals no change (the C++ `return 0`
/// without `count += 1` — turning on HighVariables is not a "change" the schedule
/// repeats on).
pub struct ActionAssignHigh {
    base: ActionBase,
}

impl ActionAssignHigh {
    /// Construct in group `g` (C++ `ActionAssignHigh::ActionAssignHigh`).
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionAssignHigh {
            base: ActionBase::new(ruleflags::rule_onceperfunc, "assignhigh", g),
        })
    }
}

impl Action for ActionAssignHigh {
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
        Some(Box::new(ActionAssignHigh { base: self.base.clone() }))
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ coreaction.hh:347 — ActionAssignHigh::apply
        //   data.setHighLevel(); return 0;
        data.set_high_level();
        0
    }
}

// =============================================================================
// ActionMergeRequired (coreaction.hh:363)
// =============================================================================

/// Make *required* Varnode merges as dictated by `CPUI_MULTIEQUAL`,
/// `CPUI_INDIRECT`, and the `addrtied` property (C++ `ActionMergeRequired`,
/// `coreaction.hh:363`).
pub struct ActionMergeRequired {
    base: ActionBase,
}

impl ActionMergeRequired {
    /// Construct in group `g` (C++ `ActionMergeRequired::ActionMergeRequired`).
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionMergeRequired {
            base: ActionBase::new(ruleflags::rule_onceperfunc, "mergerequired", g),
        })
    }
}

impl Action for ActionMergeRequired {
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
        Some(Box::new(ActionMergeRequired { base: self.base.clone() }))
    }
    fn apply(&mut self, _data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ coreaction.hh:370 — ActionMergeRequired::apply
        //   data.getMerge().mergeAddrTied();
        //   data.getMerge().groupPartials();
        //   data.getMerge().mergeMarker();
        //   return 0;
        //
        // SEAM(W7/W8-funcdata): the merge engine `Merge` is fully ported in
        // `merge.rs`, but `Funcdata` has no `getMerge()` accessor / `covermerge`
        // field and does not implement `MergeContext`, so the three forced-merge
        // passes (mergeAddrTied -> groupPartials -> mergeMarker, in this order)
        // cannot be driven here.  No change applied (count stays 0).
        0
    }
}

// =============================================================================
// ActionMarkExplicit (coreaction.hh:428, coreaction.cc:3340)
// =============================================================================

/// Find *explicit* Varnodes — those that have an explicit token representing them
/// in the output (C++ `ActionMarkExplicit`, `coreaction.cc:3340`).
pub struct ActionMarkExplicit {
    base: ActionBase,
}

impl ActionMarkExplicit {
    /// Construct in group `g` (C++ `ActionMarkExplicit::ActionMarkExplicit`).
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionMarkExplicit {
            base: ActionBase::new(ruleflags::rule_onceperfunc, "markexplicit", g),
        })
    }
}

impl Action for ActionMarkExplicit {
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
        Some(Box::new(ActionMarkExplicit { base: self.base.clone() }))
    }
    fn apply(&mut self, _data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ coreaction.cc:3340 — ActionMarkExplicit::apply
        //   maxref = data.getArch()->max_implied_ref;
        //   enditer = data.beginDef(0);             // cut out free varnodes
        //   for (viter = data.beginDef(); viter != enditer; ++viter):
        //       vn = *viter;
        //       desccount = baseExplicit(vn, maxref);
        //       if (desccount < 0):
        //           vn->setExplicit(); count += 1;
        //           if (desccount < -1) checkNewToConstructor(data, vn);
        //       else if (desccount > 1):           // possible implied w/ >1 desc
        //           vn->setMark(); multlist.push_back(vn);
        //   count += multipleInteraction(multlist);
        //   maxdup = data.getArch()->max_term_duplication;
        //   for (i = 0; i < multlist.size(); ++i):
        //       if (multlist[i]->isMark()) processMultiplier(multlist[i], maxdup);
        //   for (i = 0; i < multlist.size(); ++i): multlist[i]->clearMark();
        //   return 0;
        //
        // The helpers `baseExplicit`/`multipleInteraction`/`processMultiplier`/
        // `checkNewToConstructor` (coreaction.cc:3105-3339) all read the
        // HighVariable instance count, `getArch()` tunables, and the Varnode
        // descend/expression geometry, and mutate the explicit/mark flags.
        //
        // SEAM(W7/W8-funcdata): `Funcdata` exposes no `beginDef`/`beginDef(flags)`
        // loc-set iterator here and the `getArch()->max_implied_ref` /
        // `max_term_duplication` tunables are not surfaced; the HighVariable
        // numInstances read needs the merge/high bridge.  Body transcribed; no
        // change applied (count stays 0).
        0
    }
}

// =============================================================================
// ActionMarkImplied (coreaction.hh:450, coreaction.cc:3519)
// =============================================================================

/// Mark all the *implied* Varnode objects, which have no explicit token in the
/// output (C++ `ActionMarkImplied`, `coreaction.cc:3519`).
pub struct ActionMarkImplied {
    base: ActionBase,
}

impl ActionMarkImplied {
    /// Construct in group `g` (C++ `ActionMarkImplied::ActionMarkImplied`).
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionMarkImplied {
            base: ActionBase::new(ruleflags::rule_onceperfunc, "markimplied", g),
        })
    }
}

impl Action for ActionMarkImplied {
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
        Some(Box::new(ActionMarkImplied { base: self.base.clone() }))
    }
    fn apply(&mut self, _data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ coreaction.cc:3519 — ActionMarkImplied::apply
        //   for (viter = data.beginLoc(); viter != data.endLoc(); ++viter):
        //       vn = *viter;
        //       if (vn->isFree() || vn->isExplicit() || vn->isImplied()) continue;
        //       varstack.push_back(vn);              // depth-first descend walk
        //       do {
        //         vncur = varstack.back().vn;
        //         if (varstack.back().desciter == vncur->endDescend()):
        //             count += 1;                    // will be explicit or implied
        //             if (!checkImpliedCover(data, vncur)) vncur->setExplicit();
        //             else Merge::markImplied(vncur);
        //             varstack.pop_back();
        //         else:
        //             outvn = (*varstack.back().desciter++)->getOut();
        //             if (outvn && !outvn->isExplicit() && !outvn->isImplied())
        //                 varstack.push_back(outvn);
        //       } while (!varstack.empty());
        //   return 0;
        //
        // `checkImpliedCover` (coreaction.cc:3461) walks the HighVariable cover /
        // alias relations; `Merge::markImplied` is the static merged
        // `Merge::mark_implied(ctx, vn)` but takes a `MergeContext`.
        //
        // SEAM(W7/W8-funcdata): no `beginLoc` loc-set iterator on `Funcdata`, and
        // `markImplied` / `checkImpliedCover` need the merge/high cover bridge.
        // Body transcribed; no change applied (count stays 0).
        0
    }
}

// =============================================================================
// ActionMergeMultiEntry (coreaction.hh:397)
// =============================================================================

/// Try to merge Varnodes specified by Symbols with multiple `SymbolEntry`s (C++
/// `ActionMergeMultiEntry`, `coreaction.hh:397`).
pub struct ActionMergeMultiEntry {
    base: ActionBase,
}

impl ActionMergeMultiEntry {
    /// Construct in group `g` (C++ `ActionMergeMultiEntry::ActionMergeMultiEntry`).
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionMergeMultiEntry {
            base: ActionBase::new(ruleflags::rule_onceperfunc, "mergemultientry", g),
        })
    }
}

impl Action for ActionMergeMultiEntry {
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
        Some(Box::new(ActionMergeMultiEntry { base: self.base.clone() }))
    }
    fn apply(&mut self, _data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ coreaction.hh:404 — ActionMergeMultiEntry::apply
        //   data.getMerge().mergeMultiEntry(); return 0;
        //
        // SEAM(W7/W8-funcdata): `Merge::merge_multi_entry` is ported but needs the
        // `getMerge()`/`MergeContext` bridge.  No change applied (count stays 0).
        0
    }
}

// =============================================================================
// ActionMergeCopy (coreaction.hh:386)
// =============================================================================

/// Try to merge the input and output Varnodes of a `CPUI_COPY` op (C++
/// `ActionMergeCopy`, `coreaction.hh:386`).
pub struct ActionMergeCopy {
    base: ActionBase,
}

impl ActionMergeCopy {
    /// Construct in group `g` (C++ `ActionMergeCopy::ActionMergeCopy`).
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionMergeCopy {
            base: ActionBase::new(ruleflags::rule_onceperfunc, "mergecopy", g),
        })
    }
}

impl Action for ActionMergeCopy {
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
        Some(Box::new(ActionMergeCopy { base: self.base.clone() }))
    }
    fn apply(&mut self, _data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ coreaction.hh:393 — ActionMergeCopy::apply
        //   data.getMerge().mergeOpcode(CPUI_COPY); return 0;
        //
        // The opcode argument is fixed to `OpCode::CPUI_COPY`; referenced here so
        // the constant is transcribed verbatim.
        let _opc = OpCode::CPUI_COPY;
        // SEAM(W7/W8-funcdata): `Merge::merge_opcode` is ported but needs the
        // `getMerge()`/`MergeContext` bridge.  No change applied (count stays 0).
        0
    }
}

// =============================================================================
// ActionDominantCopy (coreaction.hh:1016)
// =============================================================================

/// Replace COPYs from the same source with a single dominant COPY (C++
/// `ActionDominantCopy`, `coreaction.hh:1016`).
pub struct ActionDominantCopy {
    base: ActionBase,
}

impl ActionDominantCopy {
    /// Construct in group `g` (C++ `ActionDominantCopy::ActionDominantCopy`).
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionDominantCopy {
            base: ActionBase::new(ruleflags::rule_onceperfunc, "dominantcopy", g),
        })
    }
}

impl Action for ActionDominantCopy {
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
        Some(Box::new(ActionDominantCopy { base: self.base.clone() }))
    }
    fn apply(&mut self, _data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ coreaction.hh:1023 — ActionDominantCopy::apply
        //   data.getMerge().processCopyTrims(); return 0;
        //
        // SEAM(W7/W8-funcdata): `Merge::process_copy_trims` is ported but needs
        // the `getMerge()`/`MergeContext` bridge.  No change (count stays 0).
        0
    }
}

// =============================================================================
// ActionMarkIndirectOnly (coreaction.hh:351)
// =============================================================================

/// Mark illegal Varnode inputs used only in `CPUI_INDIRECT` ops (C++
/// `ActionMarkIndirectOnly`, `coreaction.hh:351`).
pub struct ActionMarkIndirectOnly {
    base: ActionBase,
}

impl ActionMarkIndirectOnly {
    /// Construct in group `g` (C++ `ActionMarkIndirectOnly::ActionMarkIndirectOnly`).
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionMarkIndirectOnly {
            base: ActionBase::new(ruleflags::rule_onceperfunc, "markindirectonly", g),
        })
    }
}

impl Action for ActionMarkIndirectOnly {
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
        Some(Box::new(ActionMarkIndirectOnly { base: self.base.clone() }))
    }
    fn apply(&mut self, _data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ coreaction.hh:358 — ActionMarkIndirectOnly::apply
        //   data.markIndirectOnly(); return 0;
        //
        // SEAM(W7/W8-funcdata): `Funcdata::markIndirectOnly`
        // (funcdata_varnode.cc) — which sets the `indirect_creation` /
        // `indirectonly` Varnode flags by scanning the def-set — is not ported in
        // the merged tree.  No change applied (count stays 0).
        0
    }
}

// =============================================================================
// ActionMergeAdjacent (coreaction.hh:375)
// =============================================================================

/// Try to merge an op's input Varnode to its output if they are at the same
/// storage location (C++ `ActionMergeAdjacent`, `coreaction.hh:375`).
pub struct ActionMergeAdjacent {
    base: ActionBase,
}

impl ActionMergeAdjacent {
    /// Construct in group `g` (C++ `ActionMergeAdjacent::ActionMergeAdjacent`).
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionMergeAdjacent {
            base: ActionBase::new(ruleflags::rule_onceperfunc, "mergeadjacent", g),
        })
    }
}

impl Action for ActionMergeAdjacent {
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
        Some(Box::new(ActionMergeAdjacent { base: self.base.clone() }))
    }
    fn apply(&mut self, _data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ coreaction.hh:382 — ActionMergeAdjacent::apply
        //   data.getMerge().mergeAdjacent(); return 0;
        //
        // SEAM(W7/W8-funcdata): `Merge::merge_adjacent` is ported but needs the
        // `getMerge()`/`MergeContext` bridge.  No change (count stays 0).
        0
    }
}

// =============================================================================
// ActionMergeType (coreaction.hh:408)
// =============================================================================

/// Try to merge Varnodes of the same type, if they don't hold different values at
/// the same time (C++ `ActionMergeType`, `coreaction.hh:408`).
pub struct ActionMergeType {
    base: ActionBase,
}

impl ActionMergeType {
    /// Construct in group `g` (C++ `ActionMergeType::ActionMergeType`).
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionMergeType {
            base: ActionBase::new(ruleflags::rule_onceperfunc, "mergetype", g),
        })
    }
}

impl Action for ActionMergeType {
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
        Some(Box::new(ActionMergeType { base: self.base.clone() }))
    }
    fn apply(&mut self, _data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ coreaction.hh:415 — ActionMergeType::apply
        //   data.getMerge().mergeByDatatype(data.beginLoc(), data.endLoc());
        //   return 0;
        //
        // SEAM(W7/W8-funcdata): `Merge::merge_by_datatype` is ported (it takes a
        // location-ordered Varnode slice), but needs the `getMerge()`/
        // `MergeContext` bridge plus the `beginLoc`/`endLoc` loc-set iterator on
        // `Funcdata`.  No change applied (count stays 0).
        0
    }
}

// =============================================================================
// ActionHideShadow (coreaction.hh:1005, coreaction.cc:5085)
// =============================================================================

/// Locate and mark all `CONCAT` stacks that "shadow" a HighVariable (C++
/// `ActionHideShadow`, `coreaction.cc:5085`).
pub struct ActionHideShadow {
    base: ActionBase,
}

impl ActionHideShadow {
    /// Construct in group `g` (C++ `ActionHideShadow::ActionHideShadow`).
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionHideShadow {
            base: ActionBase::new(ruleflags::rule_onceperfunc, "hideshadow", g),
        })
    }
}

impl Action for ActionHideShadow {
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
        Some(Box::new(ActionHideShadow { base: self.base.clone() }))
    }
    fn apply(&mut self, _data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ coreaction.cc:5085 — ActionHideShadow::apply
        //   enditer = data.endDef(Varnode::written);
        //   for (iter = data.beginDef(); iter != enditer; ++iter):
        //       high = (*iter)->getHigh();
        //       if (high->isMark()) continue;
        //       if (data.getMerge().hideShadows(high)) count += 1;
        //       high->setMark();
        //   for (iter = data.beginDef(); iter != enditer; ++iter):
        //       (*iter)->getHigh()->clearMark();
        //   return 0;
        //
        // The walk visits the *written* def-set, dedups HighVariables via the
        // high mark flag, and calls the ported `Merge::hide_shadows(ctx, high)`.
        //
        // SEAM(W7/W8-funcdata): no `beginDef`/`endDef(flags)` def-set iterator on
        // `Funcdata`, the HighVariable mark surface needs the high bridge, and
        // `hideShadows` needs the `getMerge()`/`MergeContext` bridge.  Body
        // transcribed; no change applied (count stays 0).
        0
    }
}

// =============================================================================
// ActionCopyMarker (coreaction.hh:1027)
// =============================================================================

/// Mark COPY operations between Varnodes representing the same object as
/// *non-printing* (C++ `ActionCopyMarker`, `coreaction.hh:1027`).
pub struct ActionCopyMarker {
    base: ActionBase,
}

impl ActionCopyMarker {
    /// Construct in group `g` (C++ `ActionCopyMarker::ActionCopyMarker`).
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionCopyMarker {
            base: ActionBase::new(ruleflags::rule_onceperfunc, "copymarker", g),
        })
    }
}

impl Action for ActionCopyMarker {
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
        Some(Box::new(ActionCopyMarker { base: self.base.clone() }))
    }
    fn apply(&mut self, _data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ coreaction.hh:1034 — ActionCopyMarker::apply
        //   data.getMerge().markInternalCopies(); return 0;
        //
        // SEAM(W7/W8-funcdata): `Merge::mark_internal_copies` is ported but needs
        // the `getMerge()`/`MergeContext` bridge.  No change (count stays 0).
        0
    }
}

// =============================================================================
// ActionNameVars (coreaction.hh:471, coreaction.cc:3076)
// =============================================================================

/// Choose a *name* for all high-level variables (C++ `ActionNameVars`,
/// `coreaction.cc:3076`).
pub struct ActionNameVars {
    base: ActionBase,
}

impl ActionNameVars {
    /// Construct in group `g` (C++ `ActionNameVars::ActionNameVars`).
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionNameVars {
            base: ActionBase::new(ruleflags::rule_onceperfunc, "namevars", g),
        })
    }
}

impl Action for ActionNameVars {
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
        Some(Box::new(ActionNameVars { base: self.base.clone() }))
    }
    fn apply(&mut self, _data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ coreaction.cc:3076 — ActionNameVars::apply
        //   vector<Varnode *> namerec;
        //   linkSymbols(data, namerec);
        //   data.getScopeLocal()->recoverNameRecommendationsForSymbols();
        //   lookForBadJumpTables(data);
        //   lookForFuncParamNames(data, namerec);
        //   int4 base = 1;
        //   for (i = 0; i < namerec.size(); ++i):
        //       vn = namerec[i];
        //       sym = vn->getHigh()->getSymbol();
        //       if (sym->isNameUndefined()):
        //           scope = sym->getScope();
        //           newname = scope->buildDefaultName(sym, base, vn);
        //           scope->renameSymbol(sym, newname);
        //   data.getScopeLocal()->assignDefaultNames(base);
        //   return 0;
        //
        // The four helpers (`linkSymbols`/`lookForBadJumpTables`/
        // `lookForFuncParamNames`/`makeRec`, coreaction.cc:2877-3075) drive the
        // symbol/scope link surface (`getScopeLocal`/`buildDefaultName`/
        // `renameSymbol`), the callspec list (`numCalls`/`getCallSpecs`/
        // `isBadJumpTable`), and the HighVariable->Symbol attachment.
        //
        // SEAM(W7/W8-funcdata): none of `getScopeLocal`/`numCalls`/`getCallSpecs`/
        // the HighVariable->Symbol surface are present in the merged tree.  Body
        // transcribed; no change applied (count stays 0).
        0
    }
}

// =============================================================================
// ActionSetCasts (coreaction.hh:320, coreaction.cc:2812) — group "casts"
// =============================================================================

/// Place casts as needed by the high-level data-types of operands (C++
/// `ActionSetCasts`, `coreaction.cc:2812`).
///
/// The only merge-group action whose schedule group is `"casts"` rather than
/// `"merge"` (`coreaction.cc:6028`); the group is supplied by the caller, so the
/// constructor takes `g` verbatim like all the others.
pub struct ActionSetCasts {
    base: ActionBase,
}

impl ActionSetCasts {
    /// Construct in group `g` (C++ `ActionSetCasts::ActionSetCasts`).
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionSetCasts {
            base: ActionBase::new(ruleflags::rule_onceperfunc, "setcasts", g),
        })
    }
}

impl Action for ActionSetCasts {
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
        Some(Box::new(ActionSetCasts { base: self.base.clone() }))
    }
    fn apply(&mut self, _data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ coreaction.cc:2812 — ActionSetCasts::apply
        //   data.startCastPhase();
        //   castStrategy = data.getArch()->print->getCastStrategy();
        //   // dominance-ordered basic blocks, block-ordered ops
        //   for (j = 0; j < basicblocks.getSize(); ++j):
        //       bb = basicblocks.getBlock(j);
        //       for (op in bb->beginOp()..bb->endOp()):
        //           if (op->notPrinted()) continue;
        //           opc = op->code();
        //           if (opc == CPUI_CAST) continue;
        //           if (opc == CPUI_PTRADD):           // no-longer-fitting PTRADD
        //               ... opUndoPtradd(op, true);
        //           else if (opc == CPUI_PTRSUB):      // no-longer-fitting PTRSUB
        //               if (!isPtrsubMatching(...)):
        //                   if (off == 0): opRemoveInput(op,1); opSetOpcode(op,COPY);
        //                   else: opSetOpcode(op, CPUI_INT_ADD);
        //           for (i = 0; i < op->numInput(); ++i):
        //               count += resolveUnion(op, i, data, castStrategy);
        //           vn = op->getOut();
        //           if (vn): outHighType = vn->getHigh()->getType();
        //               if (outHighType->needsResolution())
        //                   outHighType->resolveInFlow(op, -1);
        //           for (i = 0; i < op->numInput(); ++i):
        //               count += castInput(op, i, data, castStrategy);
        //           if (opc == CPUI_LOAD): checkPointerIssues(op, op->getOut(), data);
        //           else if (opc == CPUI_STORE): checkPointerIssues(op, op->getIn(2), data);
        //           if (vn): count += castOutput(op, data, castStrategy);
        //   return 0;        // full completion
        //
        // The helpers (`castInput`/`castOutput`/`resolveUnion`/`checkPointerIssues`/
        // `insertPtrsubZero`/`testStructOffset0`/`tryResolution*`/`isOpIdentical`,
        // coreaction.cc:2400-2811) drive the `CastStrategy` + `print` rendering
        // machinery, the HighVariable read-/write-facing type surface, union
        // field resolution, and CAST/PTRSUB insertion.
        //
        // SEAM(W7/W8-render): `getArch()->print->getCastStrategy()`, the
        // HighVariable `getType`/`getHighTypeReadFacing`/`resolveInFlow` surface,
        // and `startCastPhase`/`opUndoPtradd`/`insertPtrsubZero` are the S9
        // rendering plane (W8) and are not present in the merged tree.  Body
        // transcribed; no change applied (count stays 0).
        0
    }
}

// =============================================================================
// W8 schedule assembly helper
// =============================================================================

/// The merge-group leaf actions, in the C++ `universalAction` schedule order
/// (`coreaction.cc:6002-6028`), each constructed in the group string the C++
/// schedule passes.  The interleaved structuring wrappers (`ActionBlockStructure`
/// etc., from [`blockaction`](crate::blockaction)) and the dynamic/global symbol
/// actions are *not* part of this set; the W8 assembler interleaves them at the
/// schedule positions shown in the module docs.
pub fn merge_actions() -> Vec<Box<dyn Action>> {
    vec![
        ActionAssignHigh::boxed("merge"),
        ActionMergeRequired::boxed("merge"),
        ActionMarkExplicit::boxed("merge"),
        ActionMarkImplied::boxed("merge"), // must come BEFORE general merging
        ActionMergeMultiEntry::boxed("merge"),
        ActionMergeCopy::boxed("merge"),
        ActionDominantCopy::boxed("merge"),
        ActionMarkIndirectOnly::boxed("merge"), // after required, before speculative
        ActionMergeAdjacent::boxed("merge"),
        ActionMergeType::boxed("merge"),
        ActionHideShadow::boxed("merge"),
        ActionCopyMarker::boxed("merge"),
        ActionNameVars::boxed("merge"),
        ActionSetCasts::boxed("casts"),
    ]
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use kuna_base::address::Address;
    use kuna_base::space::{
        addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
    };

    use super::*;
    use crate::seams::Architecture;

    // Mirrors the coreaction_protos.rs test harness (funcdata_block fixtures).
    fn build_manager() -> AddrSpaceManager {
        let mut m = AddrSpaceManager::new();
        m.insert_space(Rc::new(ConstantSpace::new())).unwrap();
        m.insert_space(Rc::new(UniqueSpace::new(1, 0, false))).unwrap();
        m.insert_space(Rc::new(AddrSpace::new(
            spacetype::IPTR_PROCESSOR,
            "ram",
            false,
            8,
            1,
            2,
            addrspace_flags::hasphysical,
            1,
            1,
        )))
        .unwrap();
        m
    }

    fn build_fd() -> Funcdata {
        let manage = build_manager();
        let glb = Rc::new(Architecture::new(manage));
        let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
        let addr = Address::new(ram, 0x1000);
        Funcdata::new("func", "func", glb, addr, 0x10000000, 0x40).unwrap()
    }

    /// Every merge-group leaf carries the exact C++ `name()` string and is
    /// `rule_onceperfunc`.
    #[test]
    fn names_and_flags_match_cpp() {
        let cases: Vec<(Box<dyn Action>, &str)> = vec![
            (ActionAssignHigh::boxed("g0"), "assignhigh"),
            (ActionMergeRequired::boxed("g0"), "mergerequired"),
            (ActionMarkExplicit::boxed("g0"), "markexplicit"),
            (ActionMarkImplied::boxed("g0"), "markimplied"),
            (ActionMergeMultiEntry::boxed("g0"), "mergemultientry"),
            (ActionMergeCopy::boxed("g0"), "mergecopy"),
            (ActionDominantCopy::boxed("g0"), "dominantcopy"),
            (ActionMarkIndirectOnly::boxed("g0"), "markindirectonly"),
            (ActionMergeAdjacent::boxed("g0"), "mergeadjacent"),
            (ActionMergeType::boxed("g0"), "mergetype"),
            (ActionHideShadow::boxed("g0"), "hideshadow"),
            (ActionCopyMarker::boxed("g0"), "copymarker"),
            (ActionNameVars::boxed("g0"), "namevars"),
            (ActionSetCasts::boxed("g0"), "setcasts"),
        ];
        for (act, expect) in &cases {
            assert_eq!(act.get_name(), *expect, "name mismatch");
            assert_eq!(act.get_group(), "g0", "group must be the ctor's `g`");
            // All fourteen are rule_onceperfunc in the C++ constructors.
            assert_eq!(
                act.base().flags,
                ruleflags::rule_onceperfunc,
                "{} must be rule_onceperfunc",
                expect
            );
        }
    }

    /// `clone_filtered` mirrors the C++ `clone(grouplist)` group filter: present
    /// iff the grouplist contains the action's group, else dropped.
    #[test]
    fn clone_filtered_obeys_grouplist() {
        let act = ActionMergeRequired::boxed("merge");
        // In-list: cloned, name and group preserved.
        let yes = ActionGroupList::from_names(["merge", "casts"]);
        let cl = act.clone_filtered(&yes).expect("present when grouplist contains group");
        assert_eq!(cl.get_name(), "mergerequired");
        assert_eq!(cl.get_group(), "merge");
        // Out-of-list: dropped (the C++ `return (Action *)0`).
        let no = ActionGroupList::from_names(["analysis"]);
        assert!(act.clone_filtered(&no).is_none(), "dropped when group absent");

        // ActionSetCasts lives in its own "casts" group.
        let casts = ActionSetCasts::boxed("casts");
        assert!(casts.clone_filtered(&yes).is_some());
        assert!(casts
            .clone_filtered(&ActionGroupList::from_names(["merge"]))
            .is_none());
    }

    /// `ActionAssignHigh::apply` is realized: it turns on the HighVariable layer
    /// (`Funcdata::set_high_level`) and is idempotent (the C++ `setHighLevel`
    /// early-returns if `isHighOn`).  It signals no change (C++ `return 0`).
    #[test]
    fn assign_high_turns_on_high_layer() {
        let mut fd = build_fd();
        let mut ctx = ActionContext::new();
        assert!(!fd.is_high_on(), "high layer starts off");

        let mut act = ActionAssignHigh::boxed("merge");
        let r = act.apply(&mut fd, &mut ctx);
        assert_eq!(r, 0, "apply returns 0");
        assert_eq!(act.base().count, 0, "setHighLevel signals no change");
        assert!(fd.is_high_on(), "high layer is on after assignhigh");

        // Idempotent: a second pass changes nothing.
        let r2 = act.apply(&mut fd, &mut ctx);
        assert_eq!(r2, 0);
        assert!(fd.is_high_on());
    }

    /// The merge-delegation actions are seamed (no `getMerge()` bridge yet): each
    /// `apply` runs cleanly and signals no change on a hand-built empty function.
    #[test]
    fn seamed_merge_actions_apply_as_noop() {
        let mut fd = build_fd();
        let mut ctx = ActionContext::new();
        let mut acts: Vec<Box<dyn Action>> = vec![
            ActionMergeRequired::boxed("merge"),
            ActionMergeMultiEntry::boxed("merge"),
            ActionMergeCopy::boxed("merge"),
            ActionDominantCopy::boxed("merge"),
            ActionMarkIndirectOnly::boxed("merge"),
            ActionMergeAdjacent::boxed("merge"),
            ActionMergeType::boxed("merge"),
            ActionCopyMarker::boxed("merge"),
        ];
        for act in &mut acts {
            let r = act.apply(&mut fd, &mut ctx);
            assert_eq!(r, 0, "{} seam returns 0", act.get_name());
            assert_eq!(act.base().count, 0, "{} seam makes no change", act.get_name());
        }
    }

    /// The marking/name/cast actions are seamed (HighVariable/symbol/cast bridge
    /// absent): each `apply` runs cleanly and signals no change.
    #[test]
    fn seamed_marking_actions_apply_as_noop() {
        let mut fd = build_fd();
        let mut ctx = ActionContext::new();
        let mut acts: Vec<Box<dyn Action>> = vec![
            ActionMarkExplicit::boxed("merge"),
            ActionMarkImplied::boxed("merge"),
            ActionHideShadow::boxed("merge"),
            ActionNameVars::boxed("merge"),
            ActionSetCasts::boxed("casts"),
        ];
        for act in &mut acts {
            let r = act.apply(&mut fd, &mut ctx);
            assert_eq!(r, 0, "{} seam returns 0", act.get_name());
            assert_eq!(act.base().count, 0, "{} seam makes no change", act.get_name());
        }
    }

    /// The W8 schedule helper enumerates all fourteen leaves in C++ schedule
    /// order, in the schedule's group strings (`"merge"` except `setcasts`).
    #[test]
    fn merge_actions_schedule_order_and_groups() {
        let acts = merge_actions();
        let names: Vec<&str> = acts.iter().map(|a| a.get_name()).collect();
        assert_eq!(
            names,
            vec![
                "assignhigh",
                "mergerequired",
                "markexplicit",
                "markimplied",
                "mergemultientry",
                "mergecopy",
                "dominantcopy",
                "markindirectonly",
                "mergeadjacent",
                "mergetype",
                "hideshadow",
                "copymarker",
                "namevars",
                "setcasts",
            ]
        );
        // Group strings: all "merge" except the final "casts".
        for a in &acts[..acts.len() - 1] {
            assert_eq!(a.get_group(), "merge");
        }
        assert_eq!(acts.last().unwrap().get_group(), "casts");
    }
}
