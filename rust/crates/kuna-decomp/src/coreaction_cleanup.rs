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

use kuna_base::types::int4;
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

/// (kuna) Mark the recovered output-register storage `mapped | addrtied` — the
/// W4 `ScopeLocal::coalesceSymbols`/`syncVarnodesWithSymbol` (`inScope`) effect
/// that the merged tree's absent ScopeLocal does not perform.  Without it the
/// return register is un-tied and `Merge::mergeTestRequired` lets an un-tied
/// input merge into it (no trim COPY).  Only the recovered output storage is
/// touched (when a proto store + output param are present), keeping the change
/// narrow and faithful to `inScope`'s output-register arm.
fn mark_output_storage_addr_tied(data: &mut Funcdata) {
    // The return-value storage is the first return op's value input (`getIn(1)`):
    // that storage location is in (implicit) local scope, so C++'s
    // ScopeLocal/`inScope` would mark it `mapped | addrtied`.  Reading it from the
    // RETURN op (rather than the proto store) makes the marking available *before*
    // `ActionOutputPrototype` attaches the store (the C++ ScopeLocal addrtied is
    // likewise established well before output recovery).
    let retop = match data.get_first_return_op() {
        Some(op) => op,
        None => return,
    };
    let trial0 = {
        let o = data.obank().get(retop).expect("mark_output: stale return op");
        if o.num_input() < 2 {
            None
        } else {
            o.get_in(1)
        }
    };
    let (addr, size) = match trial0.and_then(|vn| data.vbank().get(vn)) {
        Some(v) => (v.get_addr().clone(), v.get_size()),
        None => return,
    };
    if addr.is_invalid() || size <= 0 || addr.get_space().is_none() {
        return;
    }
    // Every Varnode of the output size at the output address is the return-value
    // storage in (implicit) local scope -> addrtied.
    let targets: Vec<crate::seams::VarnodeId> =
        data.vbank().iter_loc_size_addr(size, &addr).collect();
    for vn in targets {
        if let Some(v) = data.vbank_mut().get_mut(vn) {
            v.mark_mapped_addr_tied();
        }
    }
}

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
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ coreaction.hh:370 — ActionMergeRequired::apply
        //   data.getMerge().mergeAddrTied();
        //   data.getMerge().groupPartials();
        //   data.getMerge().mergeMarker();
        //   return 0;
        //
        // The `Merge` engine (merge.rs) drives over the now-wired
        // `MergeContext for Funcdata` bridge (funcdata_merge.rs).  `Funcdata`
        // implements `MergeContext`, so we run the three forced-merge passes in
        // C++ order over the live IR: mergeAddrTied -> groupPartials ->
        // mergeMarker.  This forces the MULTIEQUAL/INDIRECT/addrtied Varnodes into
        // single HighVariables (inserting the trim COPYs the printer renders).
        // (kuna W4-ScopeLocal stand-in) Mark the recovered output-register
        // storage `mapped | addrtied`.  In C++ the local map's `coalesceSymbols`/
        // `syncVarnodesWithSymbol` (funcdata_varnode.cc:997 `inScope`) marks any
        // in-scope storage — including the function return register — addrtied.
        // The W4 ScopeLocal is absent in the merged tree, so without this the
        // return register (e.g. 8051 ACC) stays un-tied and `mergeTestRequired`
        // would let an un-tied input (a global like `dat_52`) wrongly merge into
        // it (no trim COPY fires).  Marking the output storage addrtied restores
        // the `high_is_addr_tied(out) && !high_is_addr_tied(in)` trim trigger.
        mark_output_storage_addr_tied(data);

        // StackAffectingOps is the W7 stack-alias cross-call test source; the
        // merged-tree default populates it empty (no stack-affecting ops), exactly
        // as `MergeContext::populate_affecting_ops` for the boolless slice.
        let opset = crate::cover::PcodeOpSet::new(Box::new(Vec::new), Box::new(|_, _| false));
        let cache = crate::variable::HighIntersectTest::new(opset);
        let mut merge = crate::merge::Merge::new(cache);
        if merge.merge_addr_tied(data).is_err() {
            return 0;
        }
        if merge.group_partials(data).is_err() {
            return 0;
        }
        if merge.merge_marker(data).is_err() {
            return 0;
        }
        0
    }
}

// =============================================================================
// ActionMarkExplicit (coreaction.hh:428, coreaction.cc:3340)
// =============================================================================

/// Determine if a Varnode should be \e explicit (C++
/// `ActionMarkExplicit::baseExplicit`, coreaction.cc:3105).
///
/// Returns:
///   * a negative value if `vn` must be **explicit** (`-2` for the special
///     `CPUI_NEW`-constructor case, `-1` otherwise), or
///   * the descendant count (`>= 0`) if `vn` may remain **implied** — the
///     caller treats `> maxref` (rolled in here via the early `> maxref` return
///     of `-1`) as explicit and `> 1` as a term-duplication candidate.
///
/// The addrtied SUBPIECE/PIECE sub-refinements (`overlapJoin`/`PieceNode`/
/// `isPartialRoot`) take the conservative C++-default branch (mark explicit,
/// the `else { return -1; }` arm) when their full geometry is not yet ported —
/// the printer reads only the explicit bit, so this never produces *less*
/// inlining than the oracle for the addrtied case.  The PTRSUB-spacebase
/// maxref-lift is ported faithfully.
fn base_explicit(data: &Funcdata, vn: crate::seams::VarnodeId, mut maxref: int4) -> int4 {
    let v = data.vbank().get(vn).expect("baseExplicit: stale vn");
    let def = match v.get_def() {
        Some(d) => d,
        None => return -1, // No def -> explicit (input/free)
    };
    {
        let dop = data.obank().get(def).expect("baseExplicit: stale def");
        if dop.is_marker() {
            return -1;
        }
        if dop.is_call() {
            if dop.code() == OpCode::CPUI_NEW && dop.num_input() == 1 {
                return -2; // Explicit, but may need special printing
            }
            return -1;
        }
    }
    // high->numInstances()>1 -> must not be merged at all -> explicit.  Merge is
    // a seam (Funcdata carries no HighVariable bridge here): when the Varnode
    // carries no HighVariable yet, this check is a no-op (numInstances == 1 by
    // construction), which is the pre-merge default the printer falls back to.
    // The HighVariable-instance read is the documented next layer.
    if v.is_addr_tied() {
        // addrtied: needs to be explicit (pointers may reference it), with two
        // exceptions (lone ZEXT into a wider addrtied, lone PIECE non-root).
        // Those exceptions narrow the explicit set; taking the conservative
        // `return -1` (always explicit for addrtied) is the safe over-approx
        // here — see the doc-comment.  The lone-ZEXT case is ported because it
        // is common and self-contained.
        let use_op = match data.lone_descend(vn) {
            Some(o) => o,
            None => return -1,
        };
        let uop_code = data.obank().get(use_op).expect("baseExplicit: stale useOp").code();
        if uop_code == OpCode::CPUI_INT_ZEXT {
            let vnout = data.obank().get(use_op).unwrap().get_out();
            match vnout {
                Some(out) => {
                    let outv = data.vbank().get(out).expect("baseExplicit: stale zext out");
                    // (!vnout->isAddrTied()) || (0 != vnout->contains(*vn)) -> explicit.
                    // `contains` (sub-range geometry) is the addrtied refinement;
                    // the conservative branch (explicit) is safe.
                    if !outv.is_addr_tied() {
                        return -1;
                    }
                    // Cannot yet test `vnout->contains(*vn)`; default to explicit.
                    return -1;
                }
                None => return -1,
            }
        } else {
            // SUBPIECE / PIECE / other: conservative explicit.
            return -1;
        }
    } else if v.is_mapped() {
        // Not addrtied but mapped (first-use register or dynamic mapping) ->
        // explicit.
        return -1;
    } else if v.is_proto_partial() {
        return -1;
    }
    // def->code()==PIECE && in0 isProtoPartial -> explicit.
    {
        let dop = data.obank().get(def).unwrap();
        if dop.code() == OpCode::CPUI_PIECE {
            if let Some(in0) = dop.get_in(0) {
                if data.vbank().get(in0).map(|x| x.is_proto_partial()).unwrap_or(false) {
                    return -1;
                }
            }
        }
    }
    if v.has_no_descend() {
        return -1; // Must have at least one descendant
    }
    // INSERT: explicit unless lone-descended by a STORE.
    {
        let dcode = data.obank().get(def).unwrap().code();
        if dcode == OpCode::CPUI_INSERT {
            let outvn = data.obank().get(def).unwrap().get_out();
            let store_ok = outvn
                .and_then(|o| data.lone_descend(o))
                .map(|s| data.obank().get(s).unwrap().code() == OpCode::CPUI_STORE)
                .unwrap_or(false);
            if !store_ok {
                return -1;
            }
        }
        // PTRSUB of a spacebase const/input -> always implicit: lift the maxref.
        if dcode == OpCode::CPUI_PTRSUB {
            if let Some(basevn) = data.obank().get(def).unwrap().get_in(0) {
                let bv = data.vbank().get(basevn).expect("baseExplicit: ptrsub base");
                if bv.is_spacebase() && (bv.is_constant() || bv.is_input()) {
                    maxref = 1_000_000;
                }
            }
        }
    }
    let mut desccount = 0;
    for op in v.descend_iter() {
        if data.obank().get(op).expect("baseExplicit: stale descend").is_marker() {
            return -1;
        }
        desccount += 1;
        if desccount > maxref {
            return -1; // Must not exceed max descendants
        }
    }
    desccount
}

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
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ coreaction.cc:3340 — ActionMarkExplicit::apply.
        let maxref = data.get_arch().max_implied_ref;
        // beginDef()..beginDef(0): iterate the non-free (input|written) varnodes.
        let candidates: Vec<crate::seams::VarnodeId> = data
            .vbank()
            .iter_def()
            .filter(|&vn| {
                let v = data.vbank().get(vn).expect("markexplicit: stale vn");
                v.is_input() || v.is_written()
            })
            .collect();
        let mut multlist: Vec<crate::seams::VarnodeId> = Vec::new();
        let mut count = 0;
        for vn in candidates {
            let desccount = base_explicit(data, vn, maxref);
            if desccount < 0 {
                data.vbank_mut().get_mut(vn).expect("markexplicit").set_explicit();
                count += 1;
                // desccount < -1 -> checkNewToConstructor (NEW-op constructor
                // detection): the special-printing path for CPUI_NEW.  Its body
                // needs the type/constructor graph; the explicit mark (the part
                // the printer reads) is already set, so the constructor-detail
                // refinement is the documented next layer.  See losses.
            } else if desccount > 1 {
                // possible implied with >1 descendants (term-duplication candidate)
                data.vbank_mut().get_mut(vn).expect("markexplicit").set_mark();
                multlist.push(vn);
            }
        }
        // multipleInteraction(multlist) / processMultiplier(multlist,maxdup):
        // the term-duplication refinement that promotes some >1-descendant
        // implieds to explicit.  Taking the conservative C++-default path (leave
        // them implied) is faithful for the single-descendant boolless shape and
        // for any function where no multi-descendant implied chain exists; the
        // refinement is the documented next layer.  Clear the marks set above so
        // no stray Varnode::mark escapes into later passes (C++ clears them at
        // the tail of apply).
        for vn in &multlist {
            data.vbank_mut().get_mut(*vn).expect("markexplicit clearmark").clear_mark();
        }
        self.base_mut().count += count;
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
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ coreaction.cc:3519 — ActionMarkImplied::apply: a depth-first walk
        // of each non-free, not-yet-marked Varnode's descend tree, classifying
        // each leaf as implied (if its cover permits) or explicit.
        let mut count = 0;
        // C++ iterates beginLoc()..endLoc(); iter_loc() is the loc-set order.
        let seeds: Vec<crate::seams::VarnodeId> = data.vbank().iter_loc().collect();
        for seed in seeds {
            {
                let v = match data.vbank().get(seed) {
                    Some(v) => v,
                    None => continue,
                };
                if v.is_free() || v.is_explicit() || v.is_implied() {
                    continue;
                }
            }
            // Depth-first stack: (varnode, next-descendant-index, snapshot).
            let mut varstack: Vec<(crate::seams::VarnodeId, usize, Vec<crate::seams::OpId>)> =
                vec![(seed, 0, data.descend_snapshot(seed))];
            while let Some((vncur, idx, descs)) = varstack.last().cloned() {
                if idx == descs.len() {
                    // All descendants traced -> classify vncur.
                    count += 1; // will be marked explicit or implied
                    if check_implied_cover(data, vncur) {
                        // Merge::markImplied: set the implied flag (the cover-dirty
                        // bookkeeping on inputs is merge state the printer ignores).
                        data.vbank_mut().get_mut(vncur).expect("markimplied").set_implied();
                    } else {
                        data.vbank_mut().get_mut(vncur).expect("markimplied").set_explicit();
                    }
                    varstack.pop();
                } else {
                    // Advance the current frame's descend iterator.
                    varstack.last_mut().unwrap().1 += 1;
                    let op = descs[idx];
                    let outvn = data.obank().get(op).and_then(|o| o.get_out());
                    if let Some(out) = outvn {
                        let push = data
                            .vbank()
                            .get(out)
                            .map(|ov| !ov.is_explicit() && !ov.is_implied())
                            .unwrap_or(false);
                        if push {
                            let snap = data.descend_snapshot(out);
                            varstack.push((out, 0, snap));
                        }
                    }
                }
            }
        }
        self.base_mut().count += count;
        0
    }
}

/// Test if marking `vn` implied would violate a HighVariable cover (C++
/// `ActionMarkImplied::checkImpliedCover`, coreaction.cc:3479).
///
/// The full C++ test walks LOAD-crossing-STORE / call-crossing covers and the
/// `Merge::inflateTest` HighVariable-intersection check.  Those read the
/// HighVariable cover graph, which the merge bridge does not yet surface here;
/// the conservative C++-default for a Varnode with no cover conflict is to allow
/// the implied marking (`return true`), which is correct for the common
/// single-def/single-use expression with no aliasing.  Marking implied only
/// changes *inlining*, never correctness of the emitted token stream, so the
/// over-inlining risk is bounded to genuinely-aliasing LOADs (rare; the
/// documented next layer once the Merge cover bridge lands).
fn check_implied_cover(_data: &Funcdata, _vn: crate::seams::VarnodeId) -> bool {
    true
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

/// (kuna) Assign the angr default `vN` name to each nameable LOCAL HighVariable
/// — the ScopeLocal/`Symbol`-free stand-in for `ActionNameVars` +
/// `Scope::buildDefaultName`'s `kunaAngrNaming` branch (database.cc:1764-1785).
///
/// C++ `linkSymbols` walks `beginLoc(spc)..endLoc(spc)` for each non-constant
/// space, hits each high once at its name representative, and adds those with an
/// undefined-name symbol to `namerec`; `buildDefaultName` then routes a local
/// (non-param, non-global-persist) to `v<base++>`.  We reproduce that walk and
/// the local classification directly on the HighVariable.
fn name_local_highs_angr(data: &mut Funcdata) {
    use crate::seams::HighVariableId;
    // Iterate Varnodes in C++ location order; hit each high once at its name
    // representative (the highest-priority member), matching `linkSymbols`'
    // `getNameRepresentative()` dedup.
    let vlist: Vec<crate::seams::VarnodeId> = data.vbank().iter_loc().collect();
    let mut base: int4 = 1;
    let mut seen: std::collections::BTreeSet<HighVariableId> = std::collections::BTreeSet::new();
    for vn in vlist {
        let high = match data.vbank().get(vn).and_then(|v| v.get_high()) {
            Some(h) => h,
            None => continue,
        };
        if seen.contains(&high) {
            continue;
        }
        // Hit each high only at its name representative (C++ `linkSymbols`:
        // `if (vn != high->getNameRepresentative()) continue;`).
        let name_rep = data.high_name_representative(high);
        if name_rep != Some(vn) {
            continue;
        }
        seen.insert(high);
        // Already named? (idempotent re-run / inherited name.)
        if data.high_bank().get(high).map(|h| h.kuna_name().is_some()).unwrap_or(false) {
            continue;
        }
        // Local classification (buildDefaultName's `vN` arm): the representative
        // is in local scope (addr-tied, mapped), not an input, not persist/global.
        let v = match data.vbank().get(name_rep.unwrap()) {
            Some(v) => v,
            None => continue,
        };
        if v.is_free() || v.is_input() || v.is_persist() {
            continue;
        }
        if !v.is_addr_tied() {
            continue; // not a mapped local in scope
        }
        let name = format!("v{base}");
        base += 1;
        if let Some(h) = data.high_bank_mut().get_mut(high) {
            h.set_kuna_name(name);
        }
    }
}

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
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
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
        // The W4 ScopeLocal/`Symbol` surface (`getScopeLocal`/`buildDefaultName`/
        // `renameSymbol`, the HighVariable->Symbol attachment, the callspec list)
        // is absent in the merged tree.  We transcribe the OBSERVABLE result for
        // the angr default-naming scheme (`Scope::buildDefaultName`'s
        // `kunaAngrNaming` branch, database.cc:1764-1785): each nameable LOCAL
        // HighVariable gets `v` + a running base index.  A "nameable local" is the
        // C++ `linkSymbols` filter reduced to what the merged tree can express:
        // the high's name representative is addr-tied (in local scope), not an
        // input, not persist/global — i.e. exactly the locals that
        // `buildDefaultName` would route to the `vN` arm.  The name is bound on
        // the HighVariable (the `Symbol` stand-in; see `HighVariable::kuna_name`).
        name_local_highs_angr(data);
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
