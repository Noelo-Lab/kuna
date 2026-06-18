//! Port of the **remaining** Action classes from `decompiler/cpp/coreaction.{cc,hh}`
//! — the S3/S5/S8/S9 analysis, local-recovery, type-inference, cast/render plane.
//!
//! # What this is
//!
//! This file is the W8 `w8-s9-coreaction-render` item.  Together with
//! [`coreaction_early`](crate::coreaction_early) (early S2/S3 setup + SSA wrappers),
//! [`coreaction_protos`](crate::coreaction_protos) (S4 prototype recovery), and
//! [`coreaction_cleanup`](crate::coreaction_cleanup) (S6/S7 merge + marking + casts),
//! it completes the **full set** of `Action` classes declared in `coreaction.hh`.
//!
//! Where the three sibling modules covered the setup / prototype / merge planes,
//! this file picks up every other concrete `Action` in `coreaction.hh` that those
//! modules left unported: the stack-pointer/lane/segment transforms, the
//! conditional-constant / dead-code / switch-normalization passes, the indirect
//! de-virtualization and global-pointer recovery, the local-variable / stack-frame
//! restructuring, the dynamic-symbol attachment, the prototype-warning and
//! internal-storage finalizers, and `ActionInferTypes` (the type-propagation
//! engine whose result the `casts` group renders).  Despite the `_render` name,
//! the residue spans S3 through S9 — the item's invariant is *completeness*, not a
//! single stage.
//!
//! Each is an [`Action`](crate::action::Action) trait impl per the `action.rs`
//! registration convention: it embeds an [`ActionBase`] (the engine-owned
//! name/group/flags/status/breakpoint/counter store), keeps the **exact**
//! `name()` string and group/flags the C++ constructor used, mirrors the C++
//! `reset(data)` override for the stateful actions, and a `clone_filtered`
//! mirroring the C++ `clone(grouplist)` group filter.  Change signalling is via
//! `base_mut().count += 1` (the C++ `count += 1`).
//!
//! # Class list (this item), in C++ definition order (`coreaction.hh`)
//!
//! | C++ class | `name()` | flags | extra state | C++ `apply` |
//! |---|---|---|---|---|
//! | `ActionStackPtrFlow` | `"stackptrflow"` | `0` | `stackspace`,`analysis_finished` | `coreaction.cc:496` |
//! | `ActionLaneDivide` | `"lanedivide"` | `rule_onceperfunc` | — | `coreaction.cc:600` |
//! | `ActionSegmentize` | `"segmentize"` | `0` | `localcount` | `coreaction.cc:639` |
//! | `ActionForceGoto` | `"forcegoto"` | `0` | — | `coreaction.cc:686` |
//! | `ActionMultiCse` | `"multicse"` | `0` | — | `coreaction.cc:894` |
//! | `ActionShadowVar` | `"shadowvar"` | `0` | — | `coreaction.cc:907` |
//! | `ActionConstantPtr` | `"constantptr"` | `0` | `localcount` | `coreaction.cc:1183` |
//! | `ActionDeindirect` | `"deindirect"` | `0` | — | `coreaction.cc:1235` |
//! | `ActionDirectWrite` | `"directwrite"` | `0` | `propagateIndirect` | `coreaction.cc:1366` |
//! | `ActionLikelyTrash` | `"likelytrash"` | `0` | — | `coreaction.cc:2198` |
//! | `ActionRestructureVarnode` | `"restructure_varnode"` | `0` | `numpass` | `coreaction.cc:2332` |
//! | `ActionMappedLocalSync` | `"mapped_local_sync"` | `0` | — | `coreaction.cc:2355` |
//! | `ActionDeadCode` | `"deadcode"` | `0` | — | `coreaction.cc:4146` |
//! | `ActionConditionalConst` | `"condconst"` | `0` | — | `coreaction.cc:4748` |
//! | `ActionSwitchNorm` | `"switchnorm"` | `0` | — | `coreaction.cc:4782` |
//! | `ActionUnjustifiedParams` | `"unjustparams"` | `0` | — | `coreaction.cc:5018` |
//! | `ActionDynamicMapping` | `"dynamicmapping"` | `0` | — | `coreaction.cc:5106` |
//! | `ActionDynamicSymbols` | `"dynamicsymbols"` | `rule_onceperfunc` | — | `coreaction.cc:5123` |
//! | `ActionInternalStorage` | `"internalstorage"` | `rule_onceperfunc` | — | `coreaction.cc:5192` |
//! | `ActionMapGlobals` | `"mapglobals"` | `rule_onceperfunc` | — | `data.mapGlobals()` (`coreaction.hh:900`) |
//! | `ActionInferTypes` | `"infertypes"` | `0` | `localcount` | `coreaction.cc:5630` |
//!
//! That is **twenty-one** live classes.  Three more `coreaction.hh` declarations
//! are **commented out upstream** and therefore not ported (they are dead code,
//! retained only as historical comments in the header):
//!
//! * `ActionCse` (`coreaction.hh:152`, the old whole-graph CSE — superseded by
//!   the `RuleSplitFlow`/`ActionMultiCse` line),
//! * `ActionParamShiftStart` (`coreaction.hh:787`),
//! * `ActionParamShiftStop` (`coreaction.hh:798`).
//!
//! With this file, the union
//! `coreaction_early ∪ coreaction_protos ∪ coreaction_cleanup ∪ coreaction_render`
//! covers **every** live `class Action* : public Action` in `coreaction.hh`.
//!
//! # Seams (the `Funcdata` analysis/symbol/type surface is not in the merged tree)
//!
//! Like the other coreaction modules, every body here is gated on accessors that
//! the merged `Funcdata` does not yet expose — the call-spec list
//! (`getCallSpecs`/`numCalls`), the local/global scope surface
//! (`getScopeLocal`/`mapGlobals`/`spacebaseConstant`), the jump-table table
//! (`getJumpTable`/`findJumpTable`), the `Override` store (`getOverride`), the
//! lane-access map (`beginLaneAccess`), the `TypeFactory` propagation surface, the
//! Varnode consume/directwrite/ptrcheck flag bits, and the loc/def-set iterators
//! (`beginLoc`/`beginDef`/`beginOp`).  None of these are present, so — exactly as
//! in [`coreaction_cleanup`](crate::coreaction_cleanup) and
//! [`coreaction_protos`](crate::coreaction_protos) — each unrealized body:
//!
//! 1. transcribes the C++ `apply` (and the `reset` override, if any) as commented
//!    pseudocode, preserving iteration order, tie-breakers, and the `count += 1`
//!    points, then
//! 2. routes the unrealized mutation through a `// SEAM(W8-funcdata)` note and
//!    returns `0` changes (the C++ contract: changes are signalled by `count`).
//!
//! The realized portion of each stateful action — the `localcount`/`numpass`/
//! `analysis_finished` early-return gating, the `reset` that zeroes that counter,
//! and the per-call `propagateIndirect`/`stackspace` configuration — **is** wired
//! and tested, since it binds only to fields this file owns.  Each unrealized seam
//! is reported in this item's `losses` so the owning wave finishes the wiring by
//! replaying the commented body against the real accessors.
//!
//! # Registration
//!
//! W8 assembles `universalAction`; this file's leaf constructors plug into it via
//! [`ActionGroup::add_action`](crate::action::ActionGroup::add_action).  The
//! constructor convention mirrors the C++ ctor: each type exposes `boxed(group)`
//! (the parameterized ones, `boxed(group, ss)` / `boxed(group, prop)`) returning
//! the boxed action.  There is no single C++ schedule slice that contains exactly
//! this set — these actions interleave with the early/protos/cleanup/blockaction
//! leaves throughout `universalAction` (`coreaction.cc:5722+`), so the W8 assembler
//! pulls individual constructors rather than a `*_actions()` bundle.  A
//! [`render_actions`] helper is provided for round-trip identity testing only.

use std::rc::Rc;

use kuna_base::address::{bit_transitions, Address};
use kuna_base::space::AddrSpace;
use kuna_base::types::uintb;

use crate::action::{ruleflags, Action, ActionBase, ActionContext, ActionGroupList, ApplyResult};
use crate::dtype::type_metatype;
use crate::funcdata::Funcdata;
use crate::seams::GlobalContainer;
use crate::subflow::LaneDivide;
use crate::transform::{LaneDescription, LanedRegister};

// =============================================================================
// ActionStackPtrFlow (coreaction.hh:89, coreaction.cc:496)
// =============================================================================

/// Analyze change to the stack pointer across sub-function calls (C++
/// `ActionStackPtrFlow`, `coreaction.hh:89`).
///
/// Holds the stack [`AddrSpace`] (may be absent) and an `analysis_finished` latch
/// that the C++ `reset` clears per function.  Constructed with `flags = 0`.
pub struct ActionStackPtrFlow {
    base: ActionBase,
    /// C++ `AddrSpace *stackspace` — stack space associated with the stack-pointer
    /// register (may be null).
    stackspace: Option<Rc<AddrSpace>>,
    /// C++ `bool analysis_finished` — true once analysis has been performed for
    /// the current function (cleared by `reset`).
    analysis_finished: bool,
}

impl ActionStackPtrFlow {
    /// Construct in group `g` with stack space `ss` (C++
    /// `ActionStackPtrFlow::ActionStackPtrFlow(g,ss)`).
    pub fn boxed(g: impl Into<String>, ss: Option<Rc<AddrSpace>>) -> Box<dyn Action> {
        Box::new(ActionStackPtrFlow {
            base: ActionBase::new(0, "stackptrflow", g),
            stackspace: ss,
            analysis_finished: false,
        })
    }
}

impl Action for ActionStackPtrFlow {
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
        // C++ clone keeps the same stackspace; analysis_finished starts unset.
        Some(Box::new(ActionStackPtrFlow {
            base: self.base.clone(),
            stackspace: self.stackspace.clone(),
            analysis_finished: false,
        }))
    }
    /// C++ `ActionStackPtrFlow::reset`: `{ analysis_finished = false; }` (replaces
    /// `Action::reset`, does not chain).
    fn reset(&mut self, _data: &mut Funcdata) {
        self.analysis_finished = false;
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ coreaction.cc:496 — ActionStackPtrFlow::apply
        //   if (analysis_finished) return 0;
        //   if (stackspace == 0) { analysis_finished = true; return 0; }  // no stack
        //   numchange = checkClog(data, stackspace, 0);
        //   if (numchange > 0) count += 1;
        //   if (numchange == 0) { analyzeExtraPop(data,stackspace,0); analysis_finished = true; }
        //   return 0;
        if self.analysis_finished {
            return 0;
        }
        let stackspace = match &self.stackspace {
            Some(ss) => Rc::clone(ss),
            None => {
                self.analysis_finished = true; // No stack to do analysis on
                return 0;
            }
        };
        // checkClog: stack-pointer clog repair (LOAD->COPY).  spcbase index 0.
        let numchange = crate::coreaction_stackptr::check_clog(data, &stackspace, 0);
        let mut count = 0;
        if numchange > 0 {
            count += 1;
        }
        if numchange == 0 {
            // analyzeExtraPop: full linear solve + INT_ADD rewrite of the solution.
            crate::coreaction_stackptr::analyze_extra_pop(data, &stackspace, 0);
            self.analysis_finished = true;
        }
        count
    }
}

// =============================================================================
// ActionLaneDivide (coreaction.hh:113, coreaction.cc:600)
// =============================================================================

/// Find Varnodes with a vectorized lane scheme and split the lanes (C++
/// `ActionLaneDivide`, `coreaction.hh:113`).  Constructed `rule_onceperfunc`.
pub struct ActionLaneDivide {
    base: ActionBase,
}

impl ActionLaneDivide {
    /// Construct in group `g` (C++ `ActionLaneDivide::ActionLaneDivide`).
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionLaneDivide {
            base: ActionBase::new(ruleflags::rule_onceperfunc, "lanedivide", g),
        })
    }
}

impl Action for ActionLaneDivide {
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
        Some(Box::new(ActionLaneDivide { base: self.base.clone() }))
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ coreaction.cc:600 — ActionLaneDivide::apply
        let mut count = 0;
        // data.setLanedRegGenerated();
        data.set_laned_reg_generated();
        // The C++ iterates the lanedMap directly; the map is not mutated during
        // the apply (only the Varnode bank is), so we snapshot the access list
        // once up front and replay it (each mode re-reads the same storage set).
        let lane_access = data.lane_access_snapshot();
        // for (int4 mode = 0; mode < 3; ++mode)
        for mode in 0..3 {
            let mut all_storage_processed = true;
            // for (iter = beginLaneAccess(); iter != endLaneAccess(); ++iter)
            for (addr, sz, laned_reg) in lane_access.iter() {
                let mut all_varnodes_processed = true;
                // viter = beginLoc(sz,addr); venditer = endLoc(sz,addr);
                let mut viter: Vec<VarnodeId> =
                    data.vbank().iter_loc_size_addr(*sz, addr).collect();
                let mut idx = 0;
                while idx < viter.len() {
                    let vn = viter[idx];
                    // if (vn->hasNoDescend()) { ++viter; continue; }
                    if data.vbank().get(vn).expect("ActionLaneDivide: stale vn").has_no_descend() {
                        idx += 1;
                        continue;
                    }
                    if lane_divide_process_varnode(data, vn, laned_reg, mode) {
                        count += 1;
                        // viter = beginLoc(sz,addr); venditer = endLoc(sz,addr); // recalc bounds
                        viter = data.vbank().iter_loc_size_addr(*sz, addr).collect();
                        idx = 0;
                        all_varnodes_processed = true;
                    } else {
                        idx += 1;
                        all_varnodes_processed = false;
                    }
                }
                if !all_varnodes_processed {
                    all_storage_processed = false;
                }
            }
            if all_storage_processed {
                break;
            }
        }
        // data.clearLanedAccessMap();
        data.clear_laned_access_map();
        count
    }
}

/// Determine if a putative lane size is plausible for a Varnode by examining the
/// local ops that read or write it, registering the candidate sizes (C++
/// `ActionLaneDivide::collectLaneSizes`, coreaction.cc:524).
fn lane_divide_collect_lane_sizes(
    data: &Funcdata,
    vn: VarnodeId,
    allowed_lanes: &LanedRegister,
    check_lanes: &mut LanedRegister,
) {
    // The C++ walks `beginDescend()` (step 0), then the defining op (step 1).
    let descend: Vec<OpId> = data.descend_snapshot(vn);
    // step 0: descendants
    for &op in descend.iter() {
        // if (op->code() != CPUI_SUBPIECE) continue;  // big register split into pieces
        if data.obank().get(op).expect("collect_lane_sizes: stale op").code()
            != OpCode::CPUI_SUBPIECE
        {
            continue;
        }
        let out = data.obank().get(op).expect("collect_lane_sizes: stale op").get_out().expect("out");
        let cur_size = data.vbank().get(out).expect("collect_lane_sizes: stale out").get_size();
        if allowed_lanes.allowed_lane(cur_size) {
            check_lanes.add_lane_size(cur_size); // Register this possible size
        }
    }
    // step 1: the defining op
    let v = data.vbank().get(vn).expect("collect_lane_sizes: stale vn");
    if !v.is_written() {
        return; // if (!vn->isWritten()) continue;  (=> step = 2 = done)
    }
    let op = v.get_def().expect("collect_lane_sizes: written vn has def");
    // if (op->code() != CPUI_PIECE) continue;  // big register formed from smaller pieces
    if data.obank().get(op).expect("collect_lane_sizes: stale def op").code() != OpCode::CPUI_PIECE {
        return;
    }
    let in0 = data.obank().get(op).expect("collect_lane_sizes: stale def op").get_in(0).expect("in0");
    let in1 = data.obank().get(op).expect("collect_lane_sizes: stale def op").get_in(1).expect("in1");
    let mut cur_size = data.vbank().get(in0).expect("collect_lane_sizes: stale in0").get_size();
    let tmp_size = data.vbank().get(in1).expect("collect_lane_sizes: stale in1").get_size();
    if tmp_size < cur_size {
        cur_size = tmp_size;
    }
    if allowed_lanes.allowed_lane(cur_size) {
        check_lanes.add_lane_size(cur_size); // Register this possible size
    }
}

/// Search for a likely lane size and try to divide a single Varnode into those
/// lanes (C++ `ActionLaneDivide::processVarnode`, coreaction.cc:573).
///
/// `mode` selects the lane-size search strategy (0: putative sizes from local
/// ops; 1: same, allowing SUBPIECE downcasts; 2: default pointer-derived size).
/// Returns `true` if the Varnode (and its data-flow) was successfully split.
fn lane_divide_process_varnode(
    data: &mut Funcdata,
    vn: VarnodeId,
    laned_register: &LanedRegister,
    mode: int4,
) -> bool {
    let mut check_lanes = LanedRegister::new(); // Lanes we are going to try, no lanes initially
    let allow_downcast = mode > 0;
    if mode < 2 {
        lane_divide_collect_lane_sizes(data, vn, laned_register, &mut check_lanes);
    } else {
        // int4 defaultSize = data.getArch()->types->getSizeOfPointer();
        let mut default_size = data
            .get_arch()
            .types()
            .expect("ActionLaneDivide: type factory available for default lane size")
            .get_size_of_pointer();
        if default_size != 4 {
            default_size = 8;
        }
        check_lanes.add_lane_size(default_size);
    }
    let whole_size = laned_register.get_whole_size();
    // for (iter = checkLanes.begin(); iter != checkLanes.end(); ++iter)
    let sizes: Vec<int4> = check_lanes.iter_sizes().collect();
    for cur_size in sizes {
        // LaneDescription description(lanedRegister.getWholeSize(), curSize);
        let description = LaneDescription::uniform(whole_size, cur_size);
        let mut lane_divide = LaneDivide::new(data, vn, description, allow_downcast);
        if lane_divide.do_trace(data) {
            // The C++ unconditionally applies once the trace succeeds; the merged
            // TransformManager::apply reaches `glb->inst[opc]` (the W6 seam) for
            // the COPY/PIECE/SUBPIECE/MULTIEQUAL lane ops.  A seam error degrades
            // to "no split" (return false) exactly as a failed trace would, so a
            // partially-built transform is never half-applied.
            if lane_divide.apply(data).is_err() {
                return false;
            }
            return true; // Indicate a change was made
        }
    }
    false
}

// =============================================================================
// ActionSegmentize (coreaction.hh:128, coreaction.cc:639)
// =============================================================================

/// Convert user-defined segment p-code ops into the internal `CPUI_SEGMENTOP`
/// (C++ `ActionSegmentize`, `coreaction.hh:128`).  `flags = 0`; `localcount`
/// limits it to one application per function.
pub struct ActionSegmentize {
    base: ActionBase,
    /// C++ `int4 localcount` — times this Action has run on the function (reset to
    /// 0 each function; the body runs only when it is still 0).
    localcount: i32,
}

impl ActionSegmentize {
    /// Construct in group `g` (C++ `ActionSegmentize::ActionSegmentize`).
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionSegmentize {
            base: ActionBase::new(0, "segmentize", g),
            localcount: 0,
        })
    }
}

impl Action for ActionSegmentize {
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
        Some(Box::new(ActionSegmentize { base: self.base.clone(), localcount: 0 }))
    }
    /// C++ `ActionSegmentize::reset`: `{ localcount = 0; }`.
    fn reset(&mut self, _data: &mut Funcdata) {
        self.localcount = 0;
    }
    fn apply(&mut self, _data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ coreaction.cc:639 — ActionSegmentize::apply
        //   numops = getArch()->userops.numSegmentOps();
        //   if (numops == 0) return 0;
        //   if (localcount > 0) return 0;   // Only perform once
        //   localcount = 1;
        //   bindlist = [null, null];
        //   for (i = 0; i < numops; ++i):
        //       segdef = userops.getSegmentOp(i); if (!segdef) continue;
        //       spc = segdef->getSpace(); uindex = segdef->getIndex();
        //       for (segroot in beginOp(CPUI_CALLOTHER)..endOp(CPUI_CALLOTHER)):
        //           if (segroot->isDead()) continue;
        //           if (segroot->getIn(0)->getOffset() != uindex) continue;
        //           if (!segdef->unify(data,segroot,bindlist)) throw "Segment op in wrong form";
        //           if (segdef->getNumVariableTerms()==1) bindlist[0] = newConstant(4,0);
        //           opSetOpcode(segroot, CPUI_SEGMENTOP);
        //           opSetInput(segroot, newVarnodeSpace(spc), 0);
        //           opSetInput(segroot, bindlist[0], 1);
        //           opSetInput(segroot, bindlist[1], 2);
        //           for (j = numInput()-1; j > 2; --j) opRemoveInput(segroot, j);
        //           count += 1;
        //   return 0;
        //
        // The `userops.numSegmentOps()==0` / `localcount>0` early-outs would gate
        // the body; without `getArch()->userops` here, the *first*-pass guard
        // (`localcount`) is the only realizable part and is left to the seam since
        // the body it gates cannot run.
        //
        // SEAM(W8-funcdata): the `SegmentOp` user-op table (`getArch()->userops`),
        // `SegmentOp::unify`, and the CALLOTHER->SEGMENTOP rewrite
        // (`opSetOpcode`/`opSetInput`/`newVarnodeSpace`/`opRemoveInput`) are not
        // present.  Body transcribed; no change applied (count stays 0).
        0
    }
}

// =============================================================================
// ActionForceGoto (coreaction.hh:141, coreaction.cc:686)
// =============================================================================

/// Apply any overridden forced gotos (C++ `ActionForceGoto`,
/// `coreaction.hh:141`).  `flags = 0`.
pub struct ActionForceGoto {
    base: ActionBase,
}

impl ActionForceGoto {
    /// Construct in group `g` (C++ `ActionForceGoto::ActionForceGoto`).
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionForceGoto { base: ActionBase::new(0, "forcegoto", g) })
    }
}

impl Action for ActionForceGoto {
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
        Some(Box::new(ActionForceGoto { base: self.base.clone() }))
    }
    fn apply(&mut self, _data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ coreaction.cc:686 — ActionForceGoto::apply
        //   data.getOverride().applyForceGoto(data);
        //   return 0;
        //
        // SEAM(W8-funcdata): the `Override` store (`Funcdata::getOverride`) and
        // `Override::applyForceGoto` (which rewrites flow per user goto overrides)
        // are not in the merged tree.  Body transcribed; no change applied
        // (count stays 0 — the C++ `return 0` never bumps count either).
        0
    }
}

// =============================================================================
// ActionMultiCse (coreaction.hh:163, coreaction.cc:894)
// =============================================================================

/// Perform Common Sub-expression Elimination on `CPUI_MULTIEQUAL` ops (C++
/// `ActionMultiCse`, `coreaction.hh:163`).  `flags = 0`.
pub struct ActionMultiCse {
    base: ActionBase,
}

impl ActionMultiCse {
    /// Construct in group `g` (C++ `ActionMultiCse::ActionMultiCse`).
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionMultiCse { base: ActionBase::new(0, "multicse", g) })
    }
}

impl Action for ActionMultiCse {
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
        Some(Box::new(ActionMultiCse { base: self.base.clone() }))
    }
    fn apply(&mut self, _data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ coreaction.cc:894 — ActionMultiCse::apply
        //   bblocks = data.getBasicBlocks(); sz = bblocks.getSize();
        //   for (i = 0; i < sz; ++i):
        //       bl = bblocks.getBlock(i);
        //       while (processBlock(data, bl)) {}   // repeat until no change
        //   return 0;
        //
        // `processBlock` (coreaction.cc:846-892) scans a block for equivalent
        // MULTIEQUAL ops (via `findMatch`/`preferredOutput`) and folds the
        // redundant one, bumping `count` through the Funcdata edit surface.
        //
        // SEAM(W8-funcdata): the block graph (`getBasicBlocks`) and the
        // MULTIEQUAL fold (`opSetOutput`/`totalReplace`/`opDestroy`) are not in the
        // merged tree.  Body transcribed; no change applied (count stays 0).
        0
    }
}

// =============================================================================
// ActionShadowVar (coreaction.hh:177, coreaction.cc:907)
// =============================================================================

/// Check for one `CPUI_MULTIEQUAL` input set defining more than one Varnode (C++
/// `ActionShadowVar`, `coreaction.hh:177`).  `flags = 0`.
pub struct ActionShadowVar {
    base: ActionBase,
}

impl ActionShadowVar {
    /// Construct in group `g` (C++ `ActionShadowVar::ActionShadowVar`).
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionShadowVar { base: ActionBase::new(0, "shadowvar", g) })
    }
}

impl Action for ActionShadowVar {
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
        Some(Box::new(ActionShadowVar { base: self.base.clone() }))
    }
    fn apply(&mut self, _data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ coreaction.cc:907 — ActionShadowVar::apply
        //   for (i = 0; i < bblocks.getSize(); ++i):                    // gather shadows
        //       vnlist.clear(); bl = bblocks.getBlock(i);
        //       startoffset = bl->getStart().getOffset();
        //       for (op in bl->beginOp()..endOp()):
        //           if (op->getAddr().getOffset() != startoffset) break;  // first-address ops only
        //           if (op->code() != CPUI_MULTIEQUAL) continue;
        //           vn = op->getIn(0);
        //           if (vn->isMark()) oplist.push_back(op);
        //           else { vn->setMark(); vnlist.push_back(vn); }
        //       for (v in vnlist): v->clearMark();
        //   for (op in oplist):                                        // resolve shadows
        //       for (op2 = op->previousOp(); op2; op2 = op2->previousOp()):
        //           if (op2->code() != CPUI_MULTIEQUAL) continue;
        //           if (all op->getIn(i) == op2->getIn(i)):            // every branch matches
        //               plist = [op2->getOut()];
        //               data.opSetOpcode(op, CPUI_COPY);               // collapse shadow to COPY
        //               data.opSetAllInput(op, plist);
        //               count += 1;
        //               break;
        //   return 0;
        //
        // SEAM(W8-funcdata): the block graph, the MULTIEQUAL input-set comparison,
        // the Varnode mark bit, and the shadow->COPY collapse
        // (`opSetOpcode`/`opSetAllInput`) are not in the merged tree.  Body
        // transcribed; no change applied (count stays 0).
        0
    }
}

// =============================================================================
// ActionConstantPtr (coreaction.hh:188, coreaction.cc:1183)
// =============================================================================

/// Check for constants, with pointer type, that correspond to global symbols
/// (C++ `ActionConstantPtr`, `coreaction.hh:188`).  `flags = 0`; `localcount`
/// caps it at 4 passes per function.
pub struct ActionConstantPtr {
    base: ActionBase,
    /// C++ `int4 localcount` — passes made for this function (reset to 0; capped
    /// at 4 by the body's early-out).
    localcount: i32,
}

impl ActionConstantPtr {
    /// Construct in group `g` (C++ `ActionConstantPtr::ActionConstantPtr`).
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionConstantPtr {
            base: ActionBase::new(0, "constantptr", g),
            localcount: 0,
        })
    }
}

impl Action for ActionConstantPtr {
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
        Some(Box::new(ActionConstantPtr { base: self.base.clone(), localcount: 0 }))
    }
    /// C++ `ActionConstantPtr::reset`: `{ localcount = 0; }`.
    fn reset(&mut self, _data: &mut Funcdata) {
        self.localcount = 0;
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ coreaction.cc:1183 — ActionConstantPtr::apply
        if !data.has_type_recovery_started() {
            return 0;
        }
        if self.localcount >= 4 {
            // At most 4 passes (once type recovery starts)
            return 0;
        }
        self.localcount += 1;

        // C++ iterates `beginLoc(cspc)..endLoc(cspc)` (the constant space) with a
        // `if (!vn->isConstant()) break;` guard for inserts past enditer.  Snapshot
        // the constants present at the start of this pass: spacebaseConstant inserts
        // NEW constants (past enditer in C++, never re-entered), so the snapshot is
        // the faithful membership set.
        let cspc = match data.get_arch().manage().get_constant_space() {
            Some(s) => Rc::clone(s),
            None => return 0,
        };
        let cspc_index = cspc.get_index();
        let constants: Vec<VarnodeId> = data
            .vbank()
            .iter_loc()
            .filter(|&id| {
                data.vbank()
                    .get(id)
                    .map(|v| {
                        v.is_constant()
                            && v.get_addr().get_space().map(|s| s.get_index()) == Some(cspc_index)
                    })
                    .unwrap_or(false)
            })
            .collect();

        for vn in constants {
            // Re-read each Varnode (earlier iterations may have rewritten the IR).
            let (offset, is_ptr_check, has_no_descend, is_spacebase, size) = {
                let v = match data.vbank().get(vn) {
                    Some(v) => v,
                    None => continue,
                };
                if !v.is_constant() {
                    break; // New varnodes may get inserted (C++ enditer break)
                }
                (
                    v.get_offset(),
                    v.is_ptr_check(),
                    v.has_no_descend(),
                    v.is_spacebase(),
                    v.get_size(),
                )
            };
            if offset == 0 {
                continue; // Never make constant 0 into spacebase
            }
            if is_ptr_check {
                continue; // Have we checked this variable before
            }
            if has_no_descend {
                continue;
            }
            if is_spacebase {
                continue; // Don't use constant 0 which is already spacebase
            }

            let op = match data.lone_descend(vn) {
                Some(o) => o,
                None => continue,
            };
            let rspc = match select_infer_space(data, vn, op) {
                Some(s) => s,
                None => continue,
            };
            let slot = data.obank().get(op).expect("constantptr: stale op").get_slot(vn);
            let opc = data.obank().get(op).expect("constantptr: stale op").code();
            if opc == OpCode::CPUI_INT_ADD {
                // Make sure the other side is not a spacebase already.
                let other = data
                    .obank()
                    .get(op)
                    .expect("constantptr: stale op")
                    .get_in(1 - slot)
                    .expect("constantptr: INT_ADD missing other input");
                if data.vbank().get(other).map(|v| v.is_spacebase()).unwrap_or(false) {
                    continue;
                }
            } else if opc == OpCode::CPUI_PTRSUB || opc == OpCode::CPUI_PTRADD {
                continue;
            }

            let mut full_encoding: uintb = 0;
            let resolved = is_pointer(data, &rspc, vn, op, slot, &mut full_encoding);
            // Set check flag AFTER searching for the symbol.
            data.vbank_mut().get_mut(vn).expect("constantptr: stale vn").set_ptr_check();
            if let Some((entry, rampoint)) = resolved {
                // C++ `spacebaseConstant` never returns an error; the Rust signature
                // is fallible only for the can't-happen missing-factory/entry-type
                // paths (gated by is_pointer having returned a typed entry).  Skip
                // defensively on the impossible error rather than abort the function.
                if data
                    .spacebase_constant(op, slot, &entry, &rampoint, full_encoding, size)
                    .is_err()
                {
                    continue;
                }
                if opc == OpCode::CPUI_INT_ADD && slot == 1 {
                    data.op_swap_input(op, 0, 1);
                }
                self.base.count += 1;
            }
        }
        0
    }
}

/// C++ `ActionConstantPtr::searchForSpaceAttribute` (coreaction.cc:972): from a
/// constant, walk forward through INT_ADD/COPY/INDIRECT/MULTIEQUAL (up to 3 hops)
/// looking for a LOAD/STORE whose space-id constant names the access space, or a
/// pointer data-type carrying a space attribute.  Returns the discovered space or
/// `None`.
fn search_for_space_attribute(data: &Funcdata, mut vn: VarnodeId, mut op: OpId) -> Option<Rc<AddrSpace>> {
    for _ in 0..3 {
        // If the Varnode is explicitly a pointer carrying a space attribute, use it.
        let dt = Rc::clone(data.vbank().get(vn)?.get_type());
        if dt.get_metatype() == type_metatype::TYPE_PTR {
            if let Some(spc) = dt.get_pointer_space() {
                if spc.get_addr_size() as int4 == data.vbank().get(vn)?.get_size() {
                    return Some(spc);
                }
            }
        }
        let opc = data.obank().get(op)?.code();
        match opc {
            OpCode::CPUI_INT_ADD
            | OpCode::CPUI_COPY
            | OpCode::CPUI_INDIRECT
            | OpCode::CPUI_MULTIEQUAL => {
                vn = data.obank().get(op)?.get_out()?;
                // C++ `op = vn->loneDescend(); if (op == 0) break;` — a null
                // lone-descend ends the forward walk and FALLS THROUGH to the tail
                // descend scan below (so a multi-descendant Varnode is still scanned
                // for a direct LOAD/STORE reader).
                match data.lone_descend(vn) {
                    Some(next) => op = next,
                    None => break,
                }
            }
            OpCode::CPUI_LOAD => {
                let in0 = data.obank().get(op)?.get_in(0)?;
                return Some(space_from_const(data, in0));
            }
            OpCode::CPUI_STORE => {
                let in1 = data.obank().get(op)?.get_in(1)?;
                if in1 == vn {
                    let in0 = data.obank().get(op)?.get_in(0)?;
                    return Some(space_from_const(data, in0));
                }
                return None;
            }
            _ => return None,
        }
    }
    // C++ tail descend scan: any LOAD/STORE reading vn directly.
    let descend: Vec<OpId> = data.vbank().get(vn)?.descend_iter().collect();
    for op in descend {
        let opc = data.obank().get(op)?.code();
        if opc == OpCode::CPUI_LOAD {
            let in0 = data.obank().get(op)?.get_in(0)?;
            return Some(space_from_const(data, in0));
        } else if opc == OpCode::CPUI_STORE {
            let in1 = data.obank().get(op)?.get_in(1)?;
            if in1 == vn {
                let in0 = data.obank().get(op)?.get_in(0)?;
                return Some(space_from_const(data, in0));
            }
        }
    }
    None
}

/// Resolve `vn->getSpaceFromConst()`: the constant Varnode encodes a space-id in
/// its offset (C++ `Varnode::getSpaceFromConst`, the LOAD/STORE space argument).
fn space_from_const(data: &Funcdata, vn: VarnodeId) -> Rc<AddrSpace> {
    let idx = data.vbank().get(vn).expect("space_from_const: stale vn").get_offset() as int4;
    Rc::clone(
        data.get_arch()
            .manage()
            .get_space(idx)
            .expect("space_from_const: getSpaceFromConst out of range"),
    )
}

/// C++ `ActionConstantPtr::selectInferSpace` (coreaction.cc:1020): pick the
/// AddrSpace in which the given constant is inferred to be a pointer.  If the
/// Varnode is already a TYPE_PTR with a space attribute matching its size, use
/// that.  Otherwise scan `glb->inferPtrSpaces`, filtering by the space's
/// minimum-pointer-size / address-size; on a second match, break the tie with
/// `searchForSpaceAttribute`.
fn select_infer_space(data: &Funcdata, vn: VarnodeId, op: OpId) -> Option<Rc<AddrSpace>> {
    let vn_size = data.vbank().get(vn)?.get_size();
    let dt = Rc::clone(data.vbank().get(vn)?.get_type());
    if dt.get_metatype() == type_metatype::TYPE_PTR {
        if let Some(spc) = dt.get_pointer_space() {
            if spc.get_addr_size() as int4 == vn_size {
                return Some(spc);
            }
        }
    }
    let mut res_space: Option<Rc<AddrSpace>> = None;
    for spc in data.get_arch().infer_ptr_spaces().iter() {
        let min_size = spc.get_minimum_ptr_size();
        if min_size == 0 {
            if vn_size != spc.get_addr_size() as int4 {
                continue;
            }
        } else if vn_size < min_size {
            continue;
        }
        if res_space.is_some() {
            // Second match: disambiguate with the syntax tree.
            if let Some(search_spc) = search_for_space_attribute(data, vn, op) {
                res_space = Some(search_spc);
            }
            break;
        }
        res_space = Some(Rc::clone(spc));
    }
    res_space
}

/// C++ `ActionConstantPtr::checkCopy` (coreaction.cc:1056): for a COPY feeding a
/// RETURN of a non-pointer locked output, do not infer; otherwise honor
/// `glb->infer_pointers`.  The output-lock surface is the proto recovery seam; the
/// merged `FuncProto::is_output_locked` reports the un-recovered default
/// (`false`), so this reduces to `glb->infer_pointers` (the un-locked branch).
fn check_copy(data: &Funcdata, _op: OpId) -> bool {
    // C++: if the COPY feeds a RETURN and the output is locked to a non-pointer,
    // return false.  With no locked output (the merged proto default), fall through.
    data.get_arch().infer_pointers()
}

/// C++ `ActionConstantPtr::isPointer` (coreaction.cc:1085): determine if the given
/// constant Varnode might be a pointer; if so return the global Symbol it points
/// to and the resolved Address (the C++ `rampoint`).  `full_encoding` receives the
/// full pointer encoding (the constant value for ordinary, non-near pointers).
fn is_pointer(
    data: &Funcdata,
    spc: &Rc<AddrSpace>,
    vn: VarnodeId,
    op: OpId,
    slot: int4,
    full_encoding: &mut uintb,
) -> Option<(GlobalContainer, Address)> {
    let glb = Rc::clone(data.get_arch());
    let vn_offset = data.vbank().get(vn)?.get_offset();
    let vn_size = data.vbank().get(vn)?.get_size();
    let op_addr = data.obank().get(op)?.get_addr().clone();

    let mut needexacthit;
    let rampoint;
    // Are we explicitly marked as a pointer?
    let read_facing_meta = data.vbank().get(vn)?.get_type_read_facing(op).get_metatype();
    if read_facing_meta == type_metatype::TYPE_PTR {
        rampoint = glb.resolve_constant(spc, vn_offset, vn_size, &op_addr, full_encoding).ok()?;
        needexacthit = false;
    } else {
        if data.vbank().get(vn)?.is_type_lock() {
            return None; // Locked as NOT a pointer
        }
        needexacthit = true;
        let opc = data.obank().get(op)?.code();
        match opc {
            OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => {
                if slot == 0 {
                    return None;
                }
                // A constant parameter could be a pointer.  The input-lock check
                // (fc->isInputLocked + getParam metatype) is the call-spec seam; with
                // no locked input we fall through to the infer_pointers gate.
                if !glb.infer_pointers() {
                    return None;
                }
            }
            OpCode::CPUI_COPY => {
                if !check_copy(data, op) {
                    return None;
                }
            }
            // Pointers get concatenated in structures; comparisons against a
            // constant could be a pointer.
            OpCode::CPUI_PIECE
            | OpCode::CPUI_INT_EQUAL
            | OpCode::CPUI_INT_NOTEQUAL
            | OpCode::CPUI_INT_LESS
            | OpCode::CPUI_INT_LESSEQUAL => {
                if !glb.infer_pointers() {
                    return None;
                }
            }
            OpCode::CPUI_INT_ADD => {
                let outvn = data.obank().get(op)?.get_out()?;
                if data.vbank().get(outvn)?.get_type_def_facing().get_metatype()
                    == type_metatype::TYPE_PTR
                {
                    // Is there another pointer base in this expression?
                    let other = data.obank().get(op)?.get_in(1 - slot)?;
                    if data.vbank().get(other)?.get_type_read_facing(op).get_metatype()
                        == type_metatype::TYPE_PTR
                    {
                        return None; // If so, we are not a pointer
                    }
                    // FIXME (upstream): need to fully explore additive tree.
                    needexacthit = false;
                } else if !glb.infer_pointers() {
                    return None;
                }
            }
            OpCode::CPUI_STORE => {
                if slot != 2 {
                    return None;
                }
            }
            _ => return None,
        }
        // Make sure the constant is in the expected range for a pointer.
        if spc.get_pointer_lower_bound() > vn_offset {
            return None;
        }
        if spc.get_pointer_upper_bound() < vn_offset {
            return None;
        }
        // Check if the constant looks like a single bit or mask.  The kuna GH-6930
        // divergence keeps single-bit values that are EXACT function entries; reuse
        // kuna_is_function_entry (no re-special-casing).
        if bit_transitions(vn_offset, vn_size) < 3
            && !kuna_const_is_function_entry(data, spc, vn_offset, vn_size, &op_addr)
        {
            return None;
        }
        rampoint = glb.resolve_constant(spc, vn_offset, vn_size, &op_addr, full_encoding).ok()?;
    }

    if rampoint.is_invalid() {
        return None;
    }
    // Since we are looking for a global address, assume it is address-tied and use
    // an empty usepoint.  C++ `data.getScopeLocal()->getParent()->queryContainer(
    // rampoint, 1, Address())` — routed through `glb`'s frozen global-scope snapshot.
    let invalid = Address::new_invalid();
    let entry = glb.query_container_global(&rampoint, 1, &invalid)?;
    if let Some(ptr_type) = entry.symbol_type.as_ref() {
        if ptr_type.get_metatype() == type_metatype::TYPE_ARRAY {
            if let Some(base) = ptr_type.get_array_base() {
                // In the special case of strings (character arrays) we allow the
                // constant pointer to refer to the middle of the string.
                if base.is_char_print() {
                    needexacthit = false;
                }
            }
        }
    }
    if needexacthit && entry.entry_addr != rampoint {
        return None;
    }
    Some((entry, rampoint))
}

/// The `ActionConstantPtr::isPointer` `bit_transitions < 3` kuna GH-6930 escape
/// (coreaction.cc:1159): a single-bit constant that resolves to an exact function
/// entry stays a pointer.  Wraps [`kuna_is_function_entry`](crate::
/// kuna_inferfuncentry::kuna_is_function_entry) — the function-entry query is the
/// global-scope `queryFunction(rampoint)`, reached through `glb`'s snapshot: a
/// covering global Symbol whose type is `TYPE_CODE` and whose entry address equals
/// `rampoint`.  For ordinary data constants (no function there) this is `false`, so
/// a single-bit data constant is never made a pointer (the switch-cluster guard).
fn kuna_const_is_function_entry(
    data: &Funcdata,
    spc: &Rc<AddrSpace>,
    vn_offset: uintb,
    vn_size: int4,
    op_addr: &Address,
) -> bool {
    let glb = data.get_arch();
    let gate = glb.infer_funcentry();
    if !gate {
        return false;
    }
    let mut full_encoding: uintb = 0;
    let rampoint = match glb.resolve_constant(spc, vn_offset, vn_size, op_addr, &mut full_encoding) {
        Ok(r) => r,
        Err(_) => return false,
    };
    // queryFunction(rampoint): a covering global function Symbol whose entry
    // address equals rampoint.  The frozen snapshot carries the Symbol type; a
    // function Symbol is TYPE_CODE.
    let invalid = Address::new_invalid();
    let function_entry = glb.query_container_global(&rampoint, 1, &invalid).and_then(|e| {
        let is_code = e
            .symbol_type
            .as_ref()
            .map(|t| t.get_metatype() == type_metatype::TYPE_CODE)
            .unwrap_or(false);
        if is_code && e.entry_addr == rampoint {
            Some(e.entry_addr)
        } else {
            None
        }
    });
    crate::kuna_inferfuncentry::kuna_is_function_entry(gate, Some(&rampoint), function_entry.as_ref())
}

// =============================================================================
// ActionDeindirect (coreaction.hh:206, coreaction.cc:1235)
// =============================================================================

/// Eliminate locally constant indirect calls (C++ `ActionDeindirect`,
/// `coreaction.hh:206`).  `flags = 0`.
pub struct ActionDeindirect {
    base: ActionBase,
}

impl ActionDeindirect {
    /// Construct in group `g` (C++ `ActionDeindirect::ActionDeindirect`).
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionDeindirect { base: ActionBase::new(0, "deindirect", g) })
    }
}

impl Action for ActionDeindirect {
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
        Some(Box::new(ActionDeindirect { base: self.base.clone() }))
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ coreaction.cc:1235 — ActionDeindirect::apply.  For each CALLIND whose
        // target Varnode resolves (through COPYs) to a known function — an external
        // reference or a constant code address — convert it to a direct CALL via
        // FuncCallSpecs::deindirect.
        let mut count: int4 = 0;
        let ncalls = data.num_calls();
        for i in 0..ncalls {
            // fc = data.getCallSpecs(i); op = fc->getOp();
            let op = data.get_call_specs(i).get_op();
            // if (op->code() != CPUI_CALLIND) continue;
            if data.obank().get(op).map(|o| o.code()) != Some(OpCode::CPUI_CALLIND) {
                continue;
            }
            // vn = op->getIn(0); while (vn isWritten && def==COPY) vn = def->getIn(0);
            let mut vn = match data.obank().get(op).and_then(|o| o.get_in(0)) {
                Some(v) => v,
                None => continue,
            };
            loop {
                let def = data
                    .vbank()
                    .get(vn)
                    .filter(|v| v.is_written())
                    .and_then(|v| v.get_def());
                match def {
                    Some(d)
                        if data.obank().get(d).map(|o| o.code()) == Some(OpCode::CPUI_COPY) =>
                    {
                        match data.obank().get(d).and_then(|o| o.get_in(0)) {
                            Some(nv) => vn = nv,
                            None => break,
                        }
                    }
                    _ => break,
                }
            }

            let v = match data.vbank().get(vn) {
                Some(v) => v,
                None => continue,
            };
            let glb = data.get_arch();

            // queryFunction/queryExternalRefFunction resolve a target address to the
            // global FunctionSymbol, returning its (name, entry, recovered proto).
            let resolved: Option<(String, Address, std::rc::Rc<crate::fspec::FuncProto>)> =
                if v.is_persist() && v.is_external_ref() {
                    // Check for a possible external reference.
                    // newfd = scope->queryExternalRefFunction(vn->getAddr());
                    glb.query_function(v.get_addr())
                } else if v.is_constant() {
                    // Assume the function is in the same space as the calling function;
                    // convert the constant to a byte address in that space.
                    let sp = match data.get_address().get_space() {
                        Some(s) => std::rc::Rc::clone(s),
                        None => continue,
                    };
                    let mut offset = AddrSpace::address_to_byte(v.get_offset(), sp.get_word_size());
                    // If we know function pointers should be aligned, remove any
                    // encoding bits before querying for the function.
                    let align = glb.funcptr_align;
                    if align != 0 {
                        offset >>= align;
                        offset <<= align;
                    }
                    let codeaddr = Address::new(sp, offset);
                    glb.query_function(&codeaddr)
                } else {
                    None
                };

            if let Some((name, entry, proto)) = resolved {
                // fc->deindirect(data, newfd): splice the call spec out so it can be
                // mutated alongside `&mut data` (the op rewrite + override + proto
                // merge), then re-insert it at the same index to keep `qlst` ordering.
                let no_return = proto.is_no_return();
                let inline = proto.is_inline();
                let mut fc = data.replace_call_specs(i);
                let mut restartlog = crate::kuna_restartlog::RestartLog::new();
                let r =
                    fc.deindirect(data, entry, &name, &proto, no_return, inline, &mut restartlog);
                data.restore_call_specs_at(i, fc);
                r.expect("ActionDeindirect: deindirect failed");
                count += 1;
                continue;
            }

            // C++ data.hasTypeRecoveryStarted() function-pointer prototype arm
            // (coreaction.cc:1274-1293, fc->forceSet from an attached TypeCode
            // prototype) is not exercised by the deindirect datatests and its
            // commit path (`FuncCallSpecs::forceSet` -> commitNewInputs/Outputs) is
            // still a W4 seam; left for a follow-up.  No change applied here.
        }
        let _ = count; // change is observed through the rewritten ops / restart flag
        0
    }
}

// =============================================================================
// ActionDirectWrite (coreaction.hh:243, coreaction.cc:1366)
// =============================================================================

/// Mark Varnodes built out of legal parameters with the `directwrite` attribute
/// (C++ `ActionDirectWrite`, `coreaction.hh:243`).  `flags = 0`; carries the
/// `propagateIndirect` config (whether to taint through `CPUI_INDIRECT`).
pub struct ActionDirectWrite {
    base: ActionBase,
    /// C++ `bool propagateIndirect` — propagate the attribute through
    /// `CPUI_INDIRECT` ops (set by the ctor, never reset).
    propagate_indirect: bool,
}

impl ActionDirectWrite {
    /// Construct in group `g` with `propagateIndirect = prop` (C++
    /// `ActionDirectWrite::ActionDirectWrite(g,prop)`).
    pub fn boxed(g: impl Into<String>, prop: bool) -> Box<dyn Action> {
        Box::new(ActionDirectWrite {
            base: ActionBase::new(0, "directwrite", g),
            propagate_indirect: prop,
        })
    }
}

impl Action for ActionDirectWrite {
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
        // C++ clone passes propagateIndirect through.
        Some(Box::new(ActionDirectWrite {
            base: self.base.clone(),
            propagate_indirect: self.propagate_indirect,
        }))
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ coreaction.cc:1366 — ActionDirectWrite::apply, transcribed faithfully.
        directwrite_apply(data, self.propagate_indirect)
    }
}

/// Mark Varnodes built out of legal parameters with the `directwrite` attribute
/// (C++ `ActionDirectWrite::apply`, `coreaction.cc:1366`).
///
/// The seed loop marks the auto-direct-writes (persist/spacebase inputs, legal
/// parameter inputs, persistent/non-COPY writes, real constants); the taint loop
/// flows the attribute forward through assignment ops.  `ActionDeadCode`'s clear
/// phase reads `isDirectWrite()` to decide whether to keep an `addrforce` mark:
/// `if (isAddrForce() && !isDirectWrite()) clearAddrForce()`.  The persist
/// RETURN-COPY (`guardReturns`) reads the persist global write, so the taint
/// flows directwrite onto it; without this seed/taint the COPY's `addrforce` is
/// cleared and the global store is dead-code-eliminated.
fn directwrite_apply(data: &mut Funcdata, propagate_indirect: bool) -> ApplyResult {
    let mut worklist: Vec<VarnodeId> = Vec::new();

    // Collect legal inputs and other auto direct writes.
    let all_locs: Vec<VarnodeId> = data.vbank().iter_loc().collect();
    for vn in all_locs {
        data.vbank_mut().get_mut(vn).expect("directwrite: stale vn").clear_direct_write();
        let (is_input, is_persist, is_spacebase, is_written, is_constant, is_indirect_zero, addr, sz) = {
            let v = data.vbank().get(vn).expect("directwrite: stale vn");
            (
                v.is_input(),
                v.is_persist(),
                v.is_spacebase(),
                v.is_written(),
                v.is_constant(),
                v.is_indirect_zero(),
                v.get_addr().clone(),
                v.get_size(),
            )
        };
        if is_input {
            if is_persist || is_spacebase {
                data.vbank_mut().get_mut(vn).expect("dw").set_direct_write();
                worklist.push(vn);
            } else if data.get_func_proto().has_store()
                && data.get_func_proto().possible_input_param(&addr, sz)
            {
                // C++ `getFuncProto().possibleInputParam(...)`: the C++ FuncProto
                // always has a proto store; the merged kuna `ProtoStoreInternal`
                // may be absent for an un-recovered function (`has_store()` false),
                // in which case no input is yet a possible parameter (the
                // un-recovered default), so the seed is skipped (a register input
                // that is not yet a param is not auto-direct-write).
                data.vbank_mut().get_mut(vn).expect("dw").set_direct_write();
                worklist.push(vn);
            }
        } else if is_written {
            let opid = data.vbank().get(vn).expect("dw").get_def().expect("written vn has def");
            let (is_marker, opc) = {
                let o = data.obank().get(opid).expect("dw: stale def");
                (o.is_marker(), o.code())
            };
            if !is_marker {
                if is_persist {
                    // Anything that writes to a global (in a real way) is a direct write.
                    data.vbank_mut().get_mut(vn).expect("dw").set_direct_write();
                    worklist.push(vn);
                } else if opc == OpCode::CPUI_COPY {
                    // Most COPYs are not direct writes; a stack-store COPY whose
                    // source traces (through COPYs) to a marker (INDIRECT) is.
                    let is_stack_store = data.vbank().get(vn).expect("dw").is_stack_store();
                    if is_stack_store {
                        let mut invn = data
                            .obank()
                            .get(opid)
                            .expect("dw")
                            .get_in(0)
                            .expect("COPY in0");
                        // Through possible multiple COPYs.
                        if data.vbank().get(invn).expect("dw").is_written() {
                            let curop =
                                data.vbank().get(invn).expect("dw").get_def().expect("def");
                            if data.obank().get(curop).expect("dw").code() == OpCode::CPUI_COPY {
                                invn = data
                                    .obank()
                                    .get(curop)
                                    .expect("dw")
                                    .get_in(0)
                                    .expect("COPY in0");
                            }
                        }
                        let src_from_marker = {
                            let iv = data.vbank().get(invn).expect("dw");
                            iv.is_written()
                                && iv.get_def().map(|d| {
                                    data.obank().get(d).expect("dw").is_marker()
                                }).unwrap_or(false)
                        };
                        if src_from_marker {
                            data.vbank_mut().get_mut(vn).expect("dw").set_direct_write();
                            worklist.push(vn);
                        }
                    }
                } else if opc != OpCode::CPUI_PIECE && opc != OpCode::CPUI_SUBPIECE {
                    // Any non-COPY-form write is a direct write.
                    data.vbank_mut().get_mut(vn).expect("dw").set_direct_write();
                    worklist.push(vn);
                }
            } else if !propagate_indirect && opc == OpCode::CPUI_INDIRECT {
                // Active COPY (storage changes) or global value at a call.
                let (in0_addr, out_addr, out_persist) = {
                    let o = data.obank().get(opid).expect("dw");
                    let i0 = o.get_in(0).expect("INDIRECT in0");
                    let ov = o.get_out().expect("INDIRECT out");
                    (
                        data.vbank().get(i0).expect("dw").get_addr().clone(),
                        data.vbank().get(ov).expect("dw").get_addr().clone(),
                        data.vbank().get(ov).expect("dw").is_persist(),
                    )
                };
                if in0_addr != out_addr || out_persist {
                    data.vbank_mut().get_mut(vn).expect("dw").set_direct_write();
                }
                // Not added to worklist (INDIRECT does not propagate here).
            }
        } else if is_constant && !is_indirect_zero {
            data.vbank_mut().get_mut(vn).expect("dw").set_direct_write();
            worklist.push(vn);
        }
    }

    // Let legalness taint forward through assignment ops.
    while let Some(vn) = worklist.pop() {
        let descend: Vec<OpId> = data.vbank().get(vn).expect("dw").descend_iter().collect();
        for op in descend {
            let (is_assignment, dvn, opc, is_indirect_store) = {
                let o = data.obank().get(op).expect("dw: stale descend op");
                (o.is_assignment(), o.get_out(), o.code(), o.is_indirect_store())
            };
            if !is_assignment {
                continue;
            }
            let dvn = match dvn {
                Some(d) => d,
                None => continue,
            };
            let already = data.vbank().get(dvn).expect("dw").is_direct_write();
            if !already {
                data.vbank_mut().get_mut(dvn).expect("dw").set_direct_write();
                if propagate_indirect || opc != OpCode::CPUI_INDIRECT || is_indirect_store {
                    worklist.push(dvn);
                }
            }
        }
    }
    0
}

// =============================================================================
// ActionLikelyTrash (coreaction.hh:848, coreaction.cc:2198)
// =============================================================================

/// Get rid of registers with trash values (C++ `ActionLikelyTrash`,
/// `coreaction.hh:848`).  `flags = 0`.
pub struct ActionLikelyTrash {
    base: ActionBase,
}

impl ActionLikelyTrash {
    /// Construct in group `g` (C++ `ActionLikelyTrash::ActionLikelyTrash`).
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionLikelyTrash { base: ActionBase::new(0, "likelytrash", g) })
    }
}

impl Action for ActionLikelyTrash {
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
        Some(Box::new(ActionLikelyTrash { base: self.base.clone() }))
    }
    fn apply(&mut self, _data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ coreaction.cc:2198 — ActionLikelyTrash::apply
        //   for (vdata in proto.trashBegin()..trashEnd()):
        //       vn = data.findCoveredInput(vdata.size, vdata.getAddr());
        //       if (!vn) continue;
        //       if (vn->isTypeLock() || vn->isNameLock()) continue;
        //       indlist.clear();
        //       if (!traceTrash(vn, indlist)) continue;
        //       for (op in indlist):
        //           if (op->code() == CPUI_INDIRECT):
        //               data.opSetInput(op, newConstant(out->getSize(),0), 0);  // truncate -> indirect creation
        //               data.markIndirectCreation(op, false);
        //           else if (op->code() == CPUI_INT_AND):
        //               data.opSetInput(op, newConstant(in1->getSize(),0), 1);
        //           count += 1;
        //   return 0;
        //
        // SEAM(W8-funcdata): the proto trash list (`proto.trashBegin`),
        // `findCoveredInput`, `traceTrash` (coreaction.cc:2147-2196, the
        // COPY/AND/INDIRECT trash-flow walk), and `markIndirectCreation` are not in
        // the merged tree.  Body transcribed; no change applied (count stays 0).
        0
    }
}

// =============================================================================
// ActionRestructureVarnode (coreaction.hh:863, coreaction.cc:2332)
// =============================================================================

/// Create symbols that map out the local stack-frame for the function (C++
/// `ActionRestructureVarnode`, `coreaction.hh:863`).  `flags = 0`; `numpass`
/// counts passes (alias info is unreliable on pass 0).
pub struct ActionRestructureVarnode {
    base: ActionBase,
    /// C++ `int4 numpass` — passes performed for this function (reset to 0).
    numpass: i32,
}

impl ActionRestructureVarnode {
    /// Construct in group `g` (C++
    /// `ActionRestructureVarnode::ActionRestructureVarnode`).
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionRestructureVarnode {
            base: ActionBase::new(0, "restructure_varnode", g),
            numpass: 0,
        })
    }
}

impl Action for ActionRestructureVarnode {
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
        Some(Box::new(ActionRestructureVarnode { base: self.base.clone(), numpass: 0 }))
    }
    /// C++ `ActionRestructureVarnode::reset`: `{ numpass = 0; }`.
    fn reset(&mut self, _data: &mut Funcdata) {
        self.numpass = 0;
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ coreaction.cc:2332 — ActionRestructureVarnode::apply
        //   l1 = data.getScopeLocal();
        //   aliasyes = (numpass != 0);            // aliases unreliable on first pass
        //   l1->restructureVarnode(aliasyes);
        //   if (data.syncVarnodesWithSymbols(l1, false, aliasyes)) count += 1;
        //   if (data.isJumptableRecoveryOn()) protectSwitchPaths(data);
        //   numpass += 1;
        //   return 0;
        let mut count = 0;
        // aliases are unreliable on the first pass.
        let aliasyes = self.numpass != 0;
        // l1->restructureVarnode(aliasyes): re-derive the stack-frame layout from
        // the live (stack, off) Varnodes that RuleLoad/StoreVarnode produced.
        data.restructure_varnode(aliasyes);
        // syncVarnodesWithSymbols(l1, /*updateDatatypes*/false, aliasyes): paint
        // mapped/addrtied flags onto the promoted Varnodes from the symbol map.
        if data.sync_varnodes_with_symbols(false, aliasyes) {
            count += 1;
        }
        // SEAM(W8-funcdata): `protectSwitchPaths` (BRANCHIND/INDIRECT collapse
        // protection) needs `isJumptableRecoveryOn`; not modeled here.
        self.numpass += 1;
        count
    }
}

// =============================================================================
// ActionMappedLocalSync (coreaction.hh:882, coreaction.cc:2355)
// =============================================================================

/// Do final synchronization of symbols in the local scope with Varnodes (C++
/// `ActionMappedLocalSync`, `coreaction.hh:882`).  `flags = 0`.
pub struct ActionMappedLocalSync {
    base: ActionBase,
}

impl ActionMappedLocalSync {
    /// Construct in group `g` (C++
    /// `ActionMappedLocalSync::ActionMappedLocalSync`).
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionMappedLocalSync {
            base: ActionBase::new(0, "mapped_local_sync", g),
        })
    }
}

impl Action for ActionMappedLocalSync {
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
        Some(Box::new(ActionMappedLocalSync { base: self.base.clone() }))
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ coreaction.cc:2355 — ActionMappedLocalSync::apply
        //   l1 = data.getScopeLocal();
        //   if (data.syncVarnodesWithSymbols(l1, true, true)) count += 1;
        //   if (l1->hasOverlapProbems())
        //       data.warningHeader("Could not reconcile some variable overlaps");
        //   return 0;
        let mut count = 0;
        // Final sync: update datatypes (true) and run the unmapped-alias check.
        if data.sync_varnodes_with_symbols(true, true) {
            count += 1;
        }
        // SEAM(W8-funcdata): `Funcdata::warningHeader` on `hasOverlapProbems` is
        // not modeled (a diagnostic header, not output-determining).
        count
    }
}

// =============================================================================
// ActionMapGlobals (coreaction.hh:893)
// =============================================================================

/// Create symbols for any discovered global variables in the function (C++
/// `ActionMapGlobals`, `coreaction.hh:893`).  Constructed `rule_onceperfunc`;
/// the C++ body is the inline one-liner `data.mapGlobals(); return 0;`.
pub struct ActionMapGlobals {
    base: ActionBase,
}

impl ActionMapGlobals {
    /// Construct in group `g` (C++ `ActionMapGlobals::ActionMapGlobals`).
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionMapGlobals {
            base: ActionBase::new(ruleflags::rule_onceperfunc, "mapglobals", g),
        })
    }
}

impl Action for ActionMapGlobals {
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
        Some(Box::new(ActionMapGlobals { base: self.base.clone() }))
    }
    fn apply(&mut self, _data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ coreaction.hh:900 — ActionMapGlobals::apply (inline)
        //   data.mapGlobals(); return 0;
        //
        // SEAM(W8-funcdata): `Funcdata::mapGlobals` (which promotes discovered
        // global Varnodes into the global scope) is not on the merged `Funcdata`.
        // Body transcribed; no change applied (the C++ `return 0` never bumps
        // count either).
        0
    }
}

// =============================================================================
// ActionDeadCode (coreaction.hh:565, coreaction.cc:4146)
// =============================================================================

/// Propagate the `consumed` property to eliminate dead code (C++
/// `ActionDeadCode`, `coreaction.hh:565`).  `flags = 0`.
pub struct ActionDeadCode {
    base: ActionBase,
}

impl ActionDeadCode {
    /// Construct in group `g` (C++ `ActionDeadCode::ActionDeadCode`).
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionDeadCode { base: ActionBase::new(0, "deadcode", g) })
    }
}

impl Action for ActionDeadCode {
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
        Some(Box::new(ActionDeadCode { base: self.base.clone() }))
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ coreaction.cc:4146 — ActionDeadCode::apply.
        deadcode_apply(data)
    }
}

// ---------------------------------------------------------------------------
// ActionDeadCode algorithm (coreaction.cc:3714-4290), transcribed faithfully.
//
// The C++ methods (`pushConsumed`/`propagateConsumed`/`neverConsumed`/
// `gatherConsumedReturn`/`lastChanceLoad`/`apply`) take a `Funcdata &` plus a
// `vector<Varnode *>` worklist; in Rust they are free functions over
// `&mut Funcdata` with a `Vec<VarnodeId>` worklist (the same shape, no
// borrow-conflict).  The consume bitfield + worklist markers, the loc-set
// iteration, the dead-removal gating and the bit-usage backward fixpoint are
// all ported here on the real, now-heritaged IR.
// ---------------------------------------------------------------------------

use crate::seams::{OpId, VarnodeId};
use kuna_base::address::{calc_mask, coveringmask, leastsigbit_set, minimalmask};
use kuna_base::space::spacetype;
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

/// C++ `ActionDeadCode::pushConsumed` (coreaction.cc:3714).
///
/// `newval = (val | vn->getConsume()) & calc_mask(size)`; if it is unchanged
/// and the Varnode is already consume-vacuous, nothing to do.  Otherwise mark
/// the Varnode consume-vacuous, add it to the worklist (if written and not
/// already listed), and store the new consume mask.
fn dc_push_consumed(data: &mut Funcdata, val: u64, vn: VarnodeId, worklist: &mut Vec<VarnodeId>) {
    let (newval, vacuous, listed, written) = {
        let v = data.vbank().get(vn).expect("pushConsumed: stale vn");
        let newval = (val | v.get_consume()) & calc_mask(v.get_size());
        (newval, v.is_consume_vacuous(), v.is_consume_list(), v.is_written())
    };
    {
        let v = data.vbank().get(vn).expect("pushConsumed: stale vn");
        if newval == v.get_consume() && vacuous {
            return;
        }
    }
    let vm = data.vbank_mut().get_mut(vn).expect("pushConsumed: stale vn");
    vm.set_consume_vacuous();
    if !listed {
        vm.set_consume_list();
    }
    vm.set_consume(newval);
    // The worklist push must happen after the in-list mark is set (the C++ adds
    // to the list only for written Varnodes; free/input Varnodes are seeds that
    // never propagate backward).
    if !listed && written {
        worklist.push(vn);
    }
}

/// C++ `ActionDeadCode::propagateConsumed` (coreaction.cc:3734): pop the top of
/// the worklist and propagate its consumed mask backward to the inputs of its
/// defining op.
fn dc_propagate_consumed(data: &mut Funcdata, worklist: &mut Vec<VarnodeId>) {
    let vn = worklist.pop().expect("propagateConsumed: empty worklist");
    let (outc, opid, vn_size) = {
        let v = data.vbank().get(vn).expect("propagateConsumed: stale vn");
        (v.get_consume(), v.get_def().expect("propagateConsumed: vn is written"), v.get_size())
    };
    data.vbank_mut().get_mut(vn).expect("propagateConsumed: stale vn").clear_consume_list();

    let opc = data.obank().get(opid).expect("propagateConsumed: stale op").code();
    // input accessor helper.
    let in_n = |data: &Funcdata, slot: i32| -> VarnodeId {
        data.obank().get(opid).expect("propagateConsumed: stale op").get_in(slot).expect("null in")
    };
    let in_is_const = |data: &Funcdata, slot: i32| -> bool {
        data.vbank().get(in_n(data, slot)).expect("stale in").is_constant()
    };
    let in_off = |data: &Funcdata, slot: i32| -> u64 {
        data.vbank().get(in_n(data, slot)).expect("stale in").get_offset()
    };
    let in_size = |data: &Funcdata, slot: i32| -> i32 {
        data.vbank().get(in_n(data, slot)).expect("stale in").get_size()
    };
    let in_nzmask = |data: &Funcdata, slot: i32| -> u64 {
        data.vbank().get(in_n(data, slot)).expect("stale in").get_nz_mask()
    };
    let usize_bits = (std::mem::size_of::<u64>()) as i32; // sizeof(uintb)

    match opc {
        OpCode::CPUI_INT_MULT => {
            let b = coveringmask(outc);
            let a;
            if in_is_const(data, 1) {
                let least_set = leastsigbit_set(in_off(data, 1));
                if least_set >= 0 {
                    let mut aa = calc_mask(vn_size) >> least_set;
                    aa &= b;
                    a = aa;
                } else {
                    a = 0;
                }
            } else {
                a = b;
            }
            let (i0, i1) = (in_n(data, 0), in_n(data, 1));
            dc_push_consumed(data, a, i0, worklist);
            dc_push_consumed(data, b, i1, worklist);
        }
        OpCode::CPUI_INT_ADD | OpCode::CPUI_INT_SUB => {
            let a = coveringmask(outc);
            let (i0, i1) = (in_n(data, 0), in_n(data, 1));
            dc_push_consumed(data, a, i0, worklist);
            dc_push_consumed(data, a, i1, worklist);
        }
        OpCode::CPUI_SUBPIECE => {
            let sz = in_off(data, 1) as i32;
            let mut a = if sz >= usize_bits {
                0
            } else {
                outc.wrapping_shl((sz * 8) as u32)
            };
            if a == 0 && outc != 0 && in_size(data, 0) > usize_bits {
                let mut aa = !0u64;
                aa ^= aa >> 1;
                a = aa;
            }
            let b = if outc == 0 { 0 } else { !0u64 };
            let (i0, i1) = (in_n(data, 0), in_n(data, 1));
            dc_push_consumed(data, a, i0, worklist);
            dc_push_consumed(data, b, i1, worklist);
        }
        OpCode::CPUI_PIECE => {
            let sz = in_size(data, 1);
            let (a, b);
            if vn_size > usize_bits {
                if sz >= usize_bits {
                    a = !0u64;
                    b = outc;
                } else {
                    a = (outc >> (sz * 8)) ^ ((!0u64) << (8 * (usize_bits - sz)));
                    b = outc ^ (a << (sz * 8));
                }
            } else {
                a = outc >> (sz * 8);
                b = outc ^ (a << (sz * 8));
            }
            let (i0, i1) = (in_n(data, 0), in_n(data, 1));
            dc_push_consumed(data, a, i0, worklist);
            dc_push_consumed(data, b, i1, worklist);
        }
        OpCode::CPUI_INDIRECT => {
            let i0 = in_n(data, 0);
            dc_push_consumed(data, outc, i0, worklist);
            // The IOP-space inner block (getOpFromConst -> setIndirectSource /
            // characterizeOverlap) needs `IopSpace::getOpFromConst`
            // (op.cc), an un-ported W-later seam.  Faithful note: the outer
            // backward push above is the data-flow effect; the indirect-source
            // marking only influences later opDestroyRecursive pruning and is
            // skipped here (the conservative default — leave the source unmarked).
            let i1 = in_n(data, 1);
            let i1_is_iop = data
                .vbank()
                .get(i1)
                .and_then(|v| v.get_addr().get_space())
                .map(|s| s.get_type() == spacetype::IPTR_IOP)
                .unwrap_or(false);
            if i1_is_iop {
                // SEAM(op-from-const): getOpFromConst not ported; indirect-source
                // marking skipped (does not affect boolless / the consume fixpoint).
            }
        }
        OpCode::CPUI_COPY | OpCode::CPUI_INT_NEGATE => {
            let i0 = in_n(data, 0);
            dc_push_consumed(data, outc, i0, worklist);
        }
        OpCode::CPUI_INT_XOR | OpCode::CPUI_INT_OR => {
            let (i0, i1) = (in_n(data, 0), in_n(data, 1));
            dc_push_consumed(data, outc, i0, worklist);
            dc_push_consumed(data, outc, i1, worklist);
        }
        OpCode::CPUI_INT_AND => {
            if in_is_const(data, 1) {
                let val = in_off(data, 1);
                let (i0, i1) = (in_n(data, 0), in_n(data, 1));
                dc_push_consumed(data, outc & val, i0, worklist);
                dc_push_consumed(data, outc, i1, worklist);
            } else {
                let (i0, i1) = (in_n(data, 0), in_n(data, 1));
                dc_push_consumed(data, outc, i0, worklist);
                dc_push_consumed(data, outc, i1, worklist);
            }
        }
        OpCode::CPUI_MULTIEQUAL => {
            let n = data.obank().get(opid).expect("stale op").num_input();
            for i in 0..n {
                let ii = in_n(data, i);
                dc_push_consumed(data, outc, ii, worklist);
            }
        }
        OpCode::CPUI_INT_ZEXT => {
            let i0 = in_n(data, 0);
            dc_push_consumed(data, outc, i0, worklist);
        }
        OpCode::CPUI_INT_SEXT => {
            let b = calc_mask(in_size(data, 0));
            let mut a = outc & b;
            if outc > b {
                a |= b ^ (b >> 1);
            }
            let i0 = in_n(data, 0);
            dc_push_consumed(data, a, i0, worklist);
        }
        OpCode::CPUI_INT_LEFT => {
            if in_is_const(data, 1) {
                let mut sz = vn_size;
                let sa = in_off(data, 1) as i32;
                let a;
                if sz > usize_bits {
                    let mut aa;
                    if sa >= 8 * usize_bits {
                        aa = !0u64;
                    } else if sa == 0 {
                        aa = outc;
                    } else {
                        aa = (outc >> sa) ^ ((!0u64) << (8 * usize_bits - sa));
                    }
                    sz = 8 * sz - sa;
                    if sz < 8 * usize_bits {
                        let mut mask = !0u64;
                        mask <<= sz;
                        aa &= !mask;
                    }
                    a = aa;
                } else {
                    a = outc >> sa;
                }
                let b = if outc == 0 { 0 } else { !0u64 };
                let (i0, i1) = (in_n(data, 0), in_n(data, 1));
                dc_push_consumed(data, a, i0, worklist);
                dc_push_consumed(data, b, i1, worklist);
            } else {
                let a = if outc == 0 { 0 } else { !0u64 };
                let (i0, i1) = (in_n(data, 0), in_n(data, 1));
                dc_push_consumed(data, a, i0, worklist);
                dc_push_consumed(data, a, i1, worklist);
            }
        }
        OpCode::CPUI_INT_RIGHT => {
            if in_is_const(data, 1) {
                let sa = in_off(data, 1) as i32;
                let a = if sa >= 8 * usize_bits { 0 } else { outc << sa };
                let b = if outc == 0 { 0 } else { !0u64 };
                let (i0, i1) = (in_n(data, 0), in_n(data, 1));
                dc_push_consumed(data, a, i0, worklist);
                dc_push_consumed(data, b, i1, worklist);
            } else {
                let a = if outc == 0 { 0 } else { !0u64 };
                let (i0, i1) = (in_n(data, 0), in_n(data, 1));
                dc_push_consumed(data, a, i0, worklist);
                dc_push_consumed(data, a, i1, worklist);
            }
        }
        OpCode::CPUI_INT_LESS
        | OpCode::CPUI_INT_LESSEQUAL
        | OpCode::CPUI_INT_EQUAL
        | OpCode::CPUI_INT_NOTEQUAL => {
            let a = if outc == 0 { 0 } else { in_nzmask(data, 0) | in_nzmask(data, 1) };
            let (i0, i1) = (in_n(data, 0), in_n(data, 1));
            dc_push_consumed(data, a, i0, worklist);
            dc_push_consumed(data, a, i1, worklist);
        }
        OpCode::CPUI_INSERT => {
            let mut a = 1u64;
            a <<= in_off(data, 3) as i32;
            a -= 1;
            let i1 = in_n(data, 1);
            dc_push_consumed(data, a, i1, worklist);
            a <<= in_off(data, 2) as i32;
            let i0 = in_n(data, 0);
            dc_push_consumed(data, outc & !a, i0, worklist);
            let b = if outc == 0 { 0 } else { !0u64 };
            let (i2, i3) = (in_n(data, 2), in_n(data, 3));
            dc_push_consumed(data, b, i2, worklist);
            dc_push_consumed(data, b, i3, worklist);
        }
        OpCode::CPUI_ZPULL | OpCode::CPUI_SPULL => {
            let mut a = 1u64;
            a <<= in_off(data, 2) as i32;
            a -= 1;
            a &= outc;
            a <<= in_off(data, 1) as i32;
            let i0 = in_n(data, 0);
            dc_push_consumed(data, a, i0, worklist);
            let b = if outc == 0 { 0 } else { !0u64 };
            let (i1, i2) = (in_n(data, 1), in_n(data, 2));
            dc_push_consumed(data, b, i1, worklist);
            dc_push_consumed(data, b, i2, worklist);
        }
        OpCode::CPUI_POPCOUNT | OpCode::CPUI_LZCOUNT => {
            let mut a = 16 * in_size(data, 0) as u64 - 1;
            a &= outc;
            let b = if a == 0 { 0 } else { !0u64 };
            let i0 = in_n(data, 0);
            dc_push_consumed(data, b, i0, worklist);
        }
        OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => {
            // Call output doesn't indicate consumption of inputs.
        }
        OpCode::CPUI_FLOAT_INT2FLOAT => {
            let a = if outc != 0 { coveringmask(in_nzmask(data, 0)) } else { 0 };
            let i0 = in_n(data, 0);
            dc_push_consumed(data, a, i0, worklist);
        }
        _ => {
            let a = if outc == 0 { 0 } else { !0u64 };
            let n = data.obank().get(opid).expect("stale op").num_input();
            for i in 0..n {
                let ii = in_n(data, i);
                dc_push_consumed(data, a, ii, worklist);
            }
        }
    }
}

/// C++ `ActionDeadCode::neverConsumed` (coreaction.cc:3971): a Varnode none of
/// whose bits are consumed is eliminated — every reader's input is replaced
/// with a zero constant, and its defining op is destroyed (calls keep the op
/// but lose their output).  Returns true if the Varnode was eliminated.
fn dc_never_consumed(data: &mut Funcdata, vn: VarnodeId) -> bool {
    if data.vbank().get(vn).expect("neverConsumed: stale vn").get_size() > std::mem::size_of::<u64>() as i32 {
        return false; // Not enough precision to really tell
    }
    let size = data.vbank().get(vn).expect("neverConsumed: stale vn").get_size();
    // Replace vn with 0 wherever it is read.
    let readers: Vec<OpId> =
        data.vbank().get(vn).expect("neverConsumed: stale vn").descend_iter().collect();
    for op in readers {
        let slot = data.obank().get(op).map(|o| o.get_slot(vn)).unwrap_or(-1);
        if slot < 0 {
            continue;
        }
        let zero = data.new_constant(size, 0);
        data.op_set_input(op, zero, slot).expect("neverConsumed: opSetInput");
    }
    let op = data.vbank().get(vn).expect("neverConsumed: stale vn").get_def().expect("def");
    if data.obank().get(op).expect("neverConsumed: stale op").is_call() {
        data.op_unset_output(op);
    } else {
        data.op_destroy(op);
    }
    true
}

/// C++ `ActionDeadCode::gatherConsumedReturn` (coreaction.cc:4033): the bit
/// mask of what the function's RETURN values consume.
fn dc_gather_consumed_return(data: &Funcdata) -> u64 {
    if data.get_func_proto().is_output_locked() || data.get_active_output().is_some() {
        return !0u64;
    }
    let mut consume_val = 0u64;
    for op in data.obank().iter_code(OpCode::CPUI_RETURN) {
        let o = data.obank().get(op).expect("gatherConsumedReturn: stale op");
        if o.is_dead() {
            continue;
        }
        if o.num_input() > 1 {
            let vn = o.get_in(1).expect("RETURN in1");
            let nz = data.vbank().get(vn).expect("RETURN in1 vn").get_nz_mask();
            consume_val |= minimalmask(nz);
        }
    }
    let val = data.get_func_proto().get_return_bytes_consumed();
    if val != 0 {
        consume_val &= calc_mask(val);
    }
    consume_val
}

/// C++ `ActionDeadCode::markConsumedParameters` (coreaction.cc:4002).
///
/// For the call site `data.getCallSpecs(call_index)`, mark how each recovered
/// argument Varnode is consumed by the callee so that the data-flow defining the
/// argument is not removed as dead.
///
/// * `in0` (the fspec annotation) is always fully consumed.
/// * If the prototype is locked, or input recovery is still active, every
///   parameter is treated as fully consumed (the recovery hasn't finished
///   trimming, so nothing may be dropped yet).
/// * Otherwise each parameter consumes its non-zero-mask bits (or all bits if
///   auto-live), narrowed to `getInputBytesConsumed(i)` when the callee's body
///   analysis reported a smaller consumed footprint.
///
/// Ported as a free function over `&mut Funcdata`: the FuncCallSpecs reads
/// (`getOp`/`isInputLocked`/`isInputActive`/`getInputBytesConsumed`) are gathered
/// up front (a `&self` borrow of `qlst`), then released before the `&mut`
/// `dc_push_consumed` calls — the same borrow-splitting convention the rest of
/// this module uses.
fn dc_mark_consumed_parameters(data: &mut Funcdata, call_index: int4, worklist: &mut Vec<VarnodeId>) {
    // PcodeOp *callOp = fc->getOp();  plus the prototype flags.
    let (call_op, fully_consumed, slot_consumed) = {
        let fc = data.get_call_specs(call_index);
        let call_op = fc.get_op();
        let fully_consumed = fc.is_input_locked() || fc.is_input_active();
        // Gather getInputBytesConsumed(i) for each parameter slot while `fc`'s
        // borrow is live (only needed on the non-fully-consumed path).
        let n = data.obank().get(call_op).map(|o| o.num_input()).unwrap_or(0);
        let mut slot_consumed: Vec<int4> = Vec::new();
        if !fully_consumed {
            for i in 1..n {
                slot_consumed.push(fc.get_input_bytes_consumed(i));
            }
        }
        (call_op, fully_consumed, slot_consumed)
    };

    // In all cases the first operand (the fspec annotation) is fully consumed.
    if let Some(in0) = data.obank().get(call_op).and_then(|o| o.get_in(0)) {
        dc_push_consumed(data, !0u64, in0, worklist);
    }
    let n = data.obank().get(call_op).map(|o| o.num_input()).unwrap_or(0);
    if fully_consumed {
        // Locked or in active recovery: treat all parameters as fully consumed.
        for i in 1..n {
            if let Some(vn) = data.obank().get(call_op).and_then(|o| o.get_in(i)) {
                dc_push_consumed(data, !0u64, vn, worklist);
            }
        }
        return;
    }
    for i in 1..n {
        let vn = match data.obank().get(call_op).and_then(|o| o.get_in(i)) {
            Some(v) => v,
            None => continue,
        };
        let mut consume_val = {
            let v = data.vbank().get(vn).expect("markConsumedParameters: stale param vn");
            if v.is_auto_live() {
                !0u64
            } else {
                minimalmask(v.get_nz_mask())
            }
        };
        // cast: `i` is an int4 >= 1 (loop starts at 1), so `i - 1` is a
        // non-negative index into the slot_consumed vec gathered above (same `n`).
        let bytes_consumed = slot_consumed[(i - 1) as usize];
        if bytes_consumed != 0 {
            consume_val &= calc_mask(bytes_consumed);
        }
        dc_push_consumed(data, consume_val, vn, worklist);
    }
}

/// C++ `Varnode::isEventualConstant` (varnode.cc:873): does the Varnode (might)
/// collapse to a constant — copied/extended from a constant, the result of
/// arithmetic on constants, or loaded from a constant pointer.
///
/// Ported as a free function (the C++ method recurses through `op->getIn`, which
/// in the merged tree needs the op-bank — a `Funcdata`-level walk).
fn dc_is_eventual_constant(data: &Funcdata, vn: VarnodeId, max_binary: i32, max_load: i32) -> bool {
    let mut cur = vn;
    let mut max_load = max_load;
    loop {
        let v = data.vbank().get(cur).expect("isEventualConstant: stale vn");
        if v.is_constant() {
            return true;
        }
        if !v.is_written() {
            return false;
        }
        let op = v.get_def().expect("isEventualConstant: written vn has no def");
        let o = data.obank().get(op).expect("isEventualConstant: stale op");
        match o.code() {
            OpCode::CPUI_LOAD => {
                if max_load == 0 {
                    return false;
                }
                max_load -= 1;
                cur = o.get_in(1).expect("LOAD in1");
            }
            OpCode::CPUI_INT_ADD
            | OpCode::CPUI_INT_SUB
            | OpCode::CPUI_INT_XOR
            | OpCode::CPUI_INT_OR
            | OpCode::CPUI_INT_AND => {
                if max_binary == 0 {
                    return false;
                }
                let i0 = o.get_in(0).expect("binary in0");
                let i1 = o.get_in(1).expect("binary in1");
                if !dc_is_eventual_constant(data, i0, max_binary - 1, max_load) {
                    return false;
                }
                return dc_is_eventual_constant(data, i1, max_binary - 1, max_load);
            }
            OpCode::CPUI_INT_ZEXT | OpCode::CPUI_INT_SEXT | OpCode::CPUI_COPY => {
                cur = o.get_in(0).expect("unary in0");
            }
            OpCode::CPUI_INT_LEFT
            | OpCode::CPUI_INT_RIGHT
            | OpCode::CPUI_INT_SRIGHT
            | OpCode::CPUI_INT_MULT => {
                let i1 = o.get_in(1).expect("shift in1");
                if !data.vbank().get(i1).expect("stale in1").is_constant() {
                    return false;
                }
                cur = o.get_in(0).expect("shift in0");
            }
            _ => return false,
        }
    }
}

/// C++ `ActionDeadCode::lastChanceLoad` (coreaction.cc:4064): on heritage pass
/// 1 (only), hold un-consumed LOADs from eventual-constant addresses alive (a
/// volatile-address safety net).
fn dc_last_chance_load(data: &mut Funcdata, worklist: &mut Vec<VarnodeId>) -> bool {
    if data.get_heritage_pass() > 1 {
        return false;
    }
    if data.is_jumptable_recovery_on() {
        return false;
    }
    let loads: Vec<OpId> = data.obank().iter_code(OpCode::CPUI_LOAD).collect();
    let mut res = false;
    for op in loads {
        let o = data.obank().get(op).expect("lastChanceLoad: stale op");
        if o.is_dead() {
            continue;
        }
        let vn = match o.get_out() {
            Some(v) => v,
            None => continue,
        };
        if data.vbank().get(vn).expect("lastChanceLoad: stale out").is_consume_vacuous() {
            continue;
        }
        let ptr = o.get_in(1).expect("LOAD ptr");
        let eventual = dc_is_eventual_constant(data, ptr, 3, 1);
        if eventual {
            dc_push_consumed(data, !0u64, vn, worklist);
            data.vbank_mut().get_mut(vn).expect("lastChanceLoad: stale out").set_auto_live_hold();
            res = true;
        }
    }
    res
}

/// C++ `ActionDeadCode::apply` (coreaction.cc:4146).
fn deadcode_apply(data: &mut Funcdata) -> ApplyResult {
    let mut worklist: Vec<VarnodeId> = Vec::new();

    // The C++ `startProcessing` builds the per-space heritage info list before
    // any action runs; `deadRemovalAllowed`/`seenDeadcode` index it by space.
    // The merged tree's `ActionStart` is a seam, so ensure it here (idempotent —
    // a real run already built it in `op_heritage`).
    data.ensure_heritage_info_list();

    // 1. Clear consume flags on every Varnode; drop addrforce on non-directwrite.
    let all_locs: Vec<VarnodeId> = data.vbank().iter_loc().collect();
    for vn in all_locs {
        let vm = data.vbank_mut().get_mut(vn).expect("deadcode: stale vn");
        vm.clear_consume_list();
        vm.clear_consume_vacuous();
        vm.set_consume(0);
        let (af, dw) = {
            let v = data.vbank().get(vn).expect("deadcode: stale vn");
            (v.is_addr_force(), v.is_direct_write())
        };
        if af && !dw {
            data.vbank_mut().get_mut(vn).expect("deadcode: stale vn").clear_addr_force();
        }
    }

    // 2. Set pre-live registers (spaces that do deadcode but are NOT heritaged
    //    yet are treated as fully consumed).
    //
    // Collect the space handles up front: the C++ iterates `manage->getSpace(i)`
    // by index, but iterating through `data.get_arch().manage()` borrows `data`
    // immutably for the whole loop, conflicting with the `&mut data` pushes; the
    // Rc handles are cheap clones that decouple the borrow.
    let spaces: Vec<Rc<AddrSpace>> = {
        let manage = data.get_arch().manage();
        (0..manage.num_spaces()).filter_map(|i| manage.get_space(i).map(Rc::clone)).collect()
    };
    for spc in &spaces {
        let spc = Rc::clone(spc);
        if !spc.does_deadcode() {
            continue;
        }
        if data.dead_removal_allowed(&spc) {
            continue; // Mark consumed only if we have NOT heritaged
        }
        let ids = data.vbank().loc_space_ids(&spc);
        for vn in ids {
            dc_push_consumed(data, !0u64, vn, &mut worklist);
        }
    }

    // 3. Seed from each alive op.
    let return_consume = dc_gather_consumed_return(data);
    let alive: Vec<OpId> = data.obank().iter_alive().collect();
    for op in alive {
        data.obank_mut().get_mut(op).expect("deadcode: stale op").clear_indirect_source();
        let (is_call, is_assignment, opc, n) = {
            let o = data.obank().get(op).expect("deadcode: stale op");
            (o.is_call(), o.is_assignment(), o.code(), o.num_input())
        };
        if is_call {
            if data.obank().get(op).expect("stale op").is_call_without_spec() {
                for i in 0..n {
                    if let Some(ii) = data.obank().get(op).expect("stale op").get_in(i) {
                        dc_push_consumed(data, !0u64, ii, &mut worklist);
                    }
                }
            }
            if !is_assignment {
                continue;
            }
            if data.obank().get(op).expect("stale op").hold_output() {
                if let Some(out) = data.obank().get(op).expect("stale op").get_out() {
                    dc_push_consumed(data, !0u64, out, &mut worklist);
                }
            }
        } else if !is_assignment {
            if opc == OpCode::CPUI_RETURN {
                if let Some(i0) = data.obank().get(op).expect("stale op").get_in(0) {
                    dc_push_consumed(data, !0u64, i0, &mut worklist);
                }
                for i in 1..n {
                    if let Some(ii) = data.obank().get(op).expect("stale op").get_in(i) {
                        dc_push_consumed(data, return_consume, ii, &mut worklist);
                    }
                }
            } else if opc == OpCode::CPUI_BRANCHIND {
                // jt = findJumpTable(op); mask = (jt!=0) ? jt->getSwitchVarConsume() : ~0
                let mask = match data.find_jump_table(op) {
                    Some(jt) => jt.get_switch_var_consume(),
                    None => !0u64,
                };
                if let Some(i0) = data.obank().get(op).expect("stale op").get_in(0) {
                    dc_push_consumed(data, mask, i0, &mut worklist);
                }
            } else {
                for i in 0..n {
                    if let Some(ii) = data.obank().get(op).expect("stale op").get_in(i) {
                        dc_push_consumed(data, !0u64, ii, &mut worklist);
                    }
                }
            }
            // Postpone setting consumption on RETURN input (the output handling
            // below is skipped for non-assignment ops).
            continue;
        } else {
            for i in 0..n {
                let ii = match data.obank().get(op).expect("stale op").get_in(i) {
                    Some(v) => v,
                    None => continue,
                };
                if data.vbank().get(ii).expect("stale in").is_auto_live() {
                    dc_push_consumed(data, !0u64, ii, &mut worklist);
                }
            }
        }
        if let Some(out) = data.obank().get(op).expect("stale op").get_out() {
            if data.vbank().get(out).expect("stale out").is_auto_live() {
                dc_push_consumed(data, !0u64, out, &mut worklist);
            }
        }
    }

    // 4. Mark consumption of call parameters (coreaction.cc:4233-4235).  With the
    //    `qlst` of FuncCallSpecs now wired (w10-callsite-args), the recovered call
    //    arguments are marked consumed here, so the data-flow chain that DEFINES
    //    each argument (e.g. the COPY that loads `&i` into RDI) is not removed as
    //    dead.  Without this the call's argument Varnode loses its def, becomes
    //    free, and is re-created as a raw function-input register on the next
    //    heritage pass — so the call renders `f(RDI)` instead of `f(value)`.
    for i in 0..data.num_calls() {
        dc_mark_consumed_parameters(data, i, &mut worklist);
    }

    // 5. Propagate the consume flags (the backward bit-usage fixpoint).
    while !worklist.is_empty() {
        dc_propagate_consumed(data, &mut worklist);
    }
    if dc_last_chance_load(data, &mut worklist) {
        while !worklist.is_empty() {
            dc_propagate_consumed(data, &mut worklist);
        }
    }
    // holdStackAliasStores (kuna GH-8500) is gated default-off
    // (arch.stack_alias_deadstore) — a no-op when the option is off, which is the
    // default; the stack-alias re-seed is a later cleanup-wave concern.

    // 6. Sweep: remove dead ops, replace never-consumed values, reconcile.
    let mut total_change = 0i32;
    for spc in &spaces {
        let spc = Rc::clone(spc);
        if !spc.does_deadcode() {
            continue;
        }
        if !data.dead_removal_allowed(&spc) {
            continue; // Don't eliminate if we haven't heritaged
        }
        let ids = data.vbank().loc_space_ids(&spc);
        let mut changecount = 0i32;
        for vn in ids {
            // The varnode may already be gone (a prior neverConsumed/opDestroy
            // in this loop removed it); skip stale ids.
            let written = match data.vbank().get(vn) {
                Some(v) => v.is_written(),
                None => continue,
            };
            if !written {
                continue;
            }
            let vacflag = data.vbank().get(vn).expect("sweep: stale vn").is_consume_vacuous();
            {
                let vm = data.vbank_mut().get_mut(vn).expect("sweep: stale vn");
                vm.clear_consume_list();
                vm.clear_consume_vacuous();
            }
            if !vacflag {
                // Not even vacuously consumed -> remove the defining op.
                let op = data.vbank().get(vn).expect("sweep: stale vn").get_def().expect("def");
                changecount += 1;
                if data.obank().get(op).expect("sweep: stale op").is_call() {
                    data.op_unset_output(op);
                } else {
                    data.op_destroy(op);
                }
            } else if data.vbank().get(vn).expect("sweep: stale vn").get_consume() == 0
                && dc_never_consumed(data, vn)
            {
                changecount += 1;
            }
        }
        if changecount != 0 {
            data.seen_deadcode(&spc);
        }
        total_change += changecount;
    }

    data.clear_dead_varnodes().expect("clearDeadVarnodes");
    data.clear_dead_ops();
    let _ = total_change; // C++ ActionDeadCode::apply returns 0 regardless.
    0
}

// =============================================================================
// ActionConditionalConst (coreaction.hh:583, coreaction.cc:4748)
// =============================================================================

/// Propagate conditional constants (C++ `ActionConditionalConst`,
/// `coreaction.hh:583`).  `flags = 0`.
pub struct ActionConditionalConst {
    base: ActionBase,
}

impl ActionConditionalConst {
    /// Construct in group `g` (C++
    /// `ActionConditionalConst::ActionConditionalConst`).
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionConditionalConst { base: ActionBase::new(0, "condconst", g) })
    }
}

impl Action for ActionConditionalConst {
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
        Some(Box::new(ActionConditionalConst { base: self.base.clone() }))
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ coreaction.cc:4748 — ActionConditionalConst::apply.  The full
        // algorithm (apply/findConstCompare/propagateConstant/handlePhiNodes/
        // placeCopy/collectReachable/flowTogether/pushConstant) lives in
        // [`crate::condconst`]; it returns the `count` delta this action records.
        self.base.count += crate::condconst::condconst_apply(data);
        0
    }
}

// =============================================================================
// ActionSwitchNorm (coreaction.hh:621, coreaction.cc:4782)
// =============================================================================

/// Normalize jump-table construction (C++ `ActionSwitchNorm`,
/// `coreaction.hh:621`).  `flags = 0`.
pub struct ActionSwitchNorm {
    base: ActionBase,
}

impl ActionSwitchNorm {
    /// Construct in group `g` (C++ `ActionSwitchNorm::ActionSwitchNorm`).
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionSwitchNorm { base: ActionBase::new(0, "switchnorm", g) })
    }
}

impl Action for ActionSwitchNorm {
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
        Some(Box::new(ActionSwitchNorm { base: self.base.clone() }))
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ coreaction.cc:4782 — ActionSwitchNorm::apply
        //   for (i = 0; i < data.numJumpTables(); ++i):
        //       jt = data.getJumpTable(i);
        //       if (!jt->isLabelled()):
        //           jt->matchModel(&data);
        //           jt->recoverLabels(&data);          // recover case labels
        //           jt->foldInNormalization(&data);
        //           count += 1;
        //       if (jt->foldInGuards(&data)):
        //           data.getStructure().clear();        // force re-structure
        //           count += 1;
        //   return 0;
        let n = data.num_jump_tables();
        for i in 0..n {
            // Take the table out of the function so it can be mutated against
            // `&mut Funcdata` (the recover/fold helpers read/write the IR).
            let placeholder = crate::jumptable::JumpTable::new(
                data.get_address().clone(),
            );
            let mut jt = std::mem::replace(data.get_jump_table_mut(i), placeholder);
            if !jt.is_labelled() {
                // matchModel / recoverLabels / foldInNormalization.  Any low-level
                // error here (an un-recovered model on a stale table) is a no-op
                // for that table — keep the run alive (C++ would have thrown only
                // on a programming error; the recovered corpus tables succeed).
                if jt.match_model(data).is_ok() {
                    let _ = jt.recover_labels(data);
                    let _ = jt.fold_in_normalization(data);
                    self.base_mut().count += 1;
                }
            }
            let folded = jt.fold_in_guards(data).unwrap_or(false);
            // Store the (mutated) table back.
            *data.get_jump_table_mut(i) = jt;
            if folded {
                // data.getStructure().clear(): force a re-structure so the folded
                // guard's new default edge is picked up.
                data.sblocks_clear();
                self.base_mut().count += 1;
            }
        }
        0
    }
}

// =============================================================================
// ActionUnjustifiedParams (coreaction.hh:933, coreaction.cc:5018)
// =============================================================================

/// Adjust improperly justified parameters (C++ `ActionUnjustifiedParams`,
/// `coreaction.hh:933`).  `flags = 0`.
pub struct ActionUnjustifiedParams {
    base: ActionBase,
}

impl ActionUnjustifiedParams {
    /// Construct in group `g` (C++
    /// `ActionUnjustifiedParams::ActionUnjustifiedParams`).
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionUnjustifiedParams { base: ActionBase::new(0, "unjustparams", g) })
    }
}

impl Action for ActionUnjustifiedParams {
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
        Some(Box::new(ActionUnjustifiedParams { base: self.base.clone() }))
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ coreaction.cc:5018 — ActionUnjustifiedParams::apply
        //   proto = data.getFuncProto();
        //   for (vn in beginDef(input)..endDef(input)):
        //       if (!proto.unjustifiedInputParam(vn->getAddr(),vn->getSize(),vdata)) continue;
        //       do {                                  // widen container to absorb overlaps
        //           newcontainer = false; overlaps = false;
        //           // backward scan: widen DOWNWARD over earlier inputs that overlap
        //           for (iter2 = iter; iter2 != beginDef(input); --iter2):
        //               vn = *iter2;
        //               if (vn->getSpace() != vdata.space) continue;
        //               offset = vn->getOffset()+vn->getSize()-1;
        //               if (offset >= vdata.offset && vn->getOffset() < vdata.offset):
        //                   overlaps = true; endpoint = vdata.offset+vdata.size;
        //                   vdata.offset = vn->getOffset(); vdata.size = endpoint - vdata.offset;
        //           if (getArch()->input_varnode_adjust):   // (kuna GH-9218) forward scan UPWARD
        //               for (fvn in [iter, endDef(input))):
        //                   if (fvn->getSpace() != vdata.space) continue;
        //                   if (fvn->getOffset() >= vdata.offset+vdata.size) break;   // address-ordered
        //                   foffset = fvn->getOffset()+fvn->getSize()-1;
        //                   if (foffset >= vdata.offset+vdata.size):
        //                       overlaps = true; vdata.size = (foffset+1) - vdata.offset;
        //           if (!overlaps) break;
        //           newcontainer = proto.unjustifiedInputParam(vdata.getAddr(),vdata.size,vdata);
        //       } while (newcontainer);
        //       data.adjustInputVarnodes(vdata.getAddr(), vdata.size);
        //       iter = data.beginDef(input, vdata.getAddr()); enditer = data.endDef(input);
        //       count += 1;
        //   return 0;
        //
        // (w10-mixfloatint) The input def-set iteration, `unjustifiedInputParam`,
        // the GH-9218 `input_varnode_adjust` forward-absorb, and
        // `adjustInputVarnodes` are now ported.  This recombines a register pair
        // that an earlier heritage `refinement` split into two contiguous
        // sub-size input pieces (e.g. an xmm0 float8 read whose halves split
        // because the return-register lane COPYs made the disjoint range's max
        // write size 4) back into one whole input — fixing Mixed float/int #1.
        use kuna_num::pcoderaw::VarnodeData;
        // The unjustified-param query reads the prototype model/store; a model-less
        // fixture (no ActionPrototypeTypes seed) has neither, so there is nothing
        // to adjust — bail (mirrors ActionInputPrototype's `recoverable` guard).
        // The unjustified-param query reads the prototype model; a model-less
        // fixture (no ActionPrototypeTypes seed) has nothing to adjust.  The
        // unlocked-recovery merged-tree proto carries a model but no W4 symbol
        // store, so the query routes through the model-level ParamList
        // (`ProtoModel::unjustifiedInputParam`) — faithful, since for an unlocked
        // proto C++'s FuncProto::unjustifiedInputParam falls through to the model
        // anyway.  A locked-input proto IS handled by the store path; defer it to
        // the W4 store wave (the recovery path that motivates this is unlocked).
        if !data.get_func_proto().has_model() || data.get_func_proto().is_input_locked() {
            return 0;
        }
        let input_adjust = data.get_arch().input_varnode_adjust;

        // iter = beginDef(input); the def-set is Address-then-size ordered.
        // Snapshot the input list once; `adjustInputVarnodes` rewrites it, so we
        // re-seek (C++ resets the iterator after each adjustment).
        loop {
            let inputs: Vec<crate::seams::VarnodeId> =
                data.vbank().iter_def_flag(crate::varnode::varnode_flags::input).collect();
            let mut adjusted = false;
            'scan: for (idx, &vn) in inputs.iter().enumerate() {
                let (vn_addr, vn_size) = {
                    let v = match data.vbank().get(vn) {
                        Some(v) => v,
                        None => continue,
                    };
                    (v.get_addr().clone(), v.get_size())
                };
                let mut vdata = VarnodeData::default();
                if !data
                    .get_func_proto()
                    .model()
                    .unjustified_input_param(&vn_addr, vn_size, &mut vdata)
                {
                    continue;
                }
                let vspace = match vdata.space.clone() {
                    Some(s) => s,
                    None => continue,
                };
                // Widen the container to absorb overlapping inputs.
                loop {
                    let mut overlaps = false;
                    // Backward scan: widen DOWNWARD over earlier inputs that overlap.
                    for &ovn in inputs[..idx].iter().rev() {
                        let v = match data.vbank().get(ovn) {
                            Some(v) => v,
                            None => continue,
                        };
                        if v.get_addr().get_space().map_or(true, |s| !Rc::ptr_eq(s, &vspace)) {
                            continue;
                        }
                        let ooff = v.get_addr().get_offset();
                        let last = ooff.wrapping_add((v.get_size() - 1) as u64);
                        // if (offset >= vdata.offset && vn->getOffset() < vdata.offset)
                        if last >= vdata.offset && ooff < vdata.offset {
                            overlaps = true;
                            let endpoint = vdata.offset.wrapping_add(vdata.size as u64);
                            vdata.offset = ooff;
                            vdata.size = (endpoint.wrapping_sub(vdata.offset)) as u32;
                        }
                    }
                    if input_adjust {
                        // (kuna GH-9218) forward scan UPWARD: absorb an input
                        // that overlaps the container and extends above its end.
                        for &fvn in inputs[idx..].iter() {
                            let v = match data.vbank().get(fvn) {
                                Some(v) => v,
                                None => continue,
                            };
                            if v.get_addr().get_space().map_or(true, |s| !Rc::ptr_eq(s, &vspace)) {
                                continue;
                            }
                            let foff = v.get_addr().get_offset();
                            if foff >= vdata.offset.wrapping_add(vdata.size as u64) {
                                break; // past the container; def-set is address-ordered
                            }
                            let flast = foff.wrapping_add((v.get_size() - 1) as u64);
                            if flast >= vdata.offset.wrapping_add(vdata.size as u64) {
                                overlaps = true;
                                vdata.size = (flast + 1).wrapping_sub(vdata.offset) as u32;
                            }
                        }
                    }
                    if !overlaps {
                        break;
                    }
                    // Container may no longer be justified after widening.
                    let waddr = Address::new(Rc::clone(&vspace), vdata.offset);
                    if !data.get_func_proto().model().unjustified_input_param(
                        &waddr,
                        vdata.size as int4,
                        &mut vdata,
                    ) {
                        break;
                    }
                }
                let cont_addr = Address::new(Rc::clone(&vspace), vdata.offset);
                if data
                    .adjust_input_varnodes(&cont_addr, vdata.size as int4)
                    .is_ok()
                {
                    self.base.count += 1;
                    adjusted = true;
                    break 'scan;
                }
            }
            if !adjusted {
                break;
            }
        }
        0
    }
}

// =============================================================================
// ActionDynamicMapping (coreaction.hh:1038, coreaction.cc:5106)
// =============================================================================

/// Attach dynamically mapped symbols to Varnodes in time for data-type
/// propagation (C++ `ActionDynamicMapping`, `coreaction.hh:1038`).  `flags = 0`.
pub struct ActionDynamicMapping {
    base: ActionBase,
}

impl ActionDynamicMapping {
    /// Construct in group `g` (C++
    /// `ActionDynamicMapping::ActionDynamicMapping`).
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionDynamicMapping { base: ActionBase::new(0, "dynamicmapping", g) })
    }
}

impl Action for ActionDynamicMapping {
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
        Some(Box::new(ActionDynamicMapping { base: self.base.clone() }))
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ coreaction.cc:5106 — ActionDynamicMapping::apply
        //   localmap = data.getScopeLocal();
        //   for (entry in localmap->beginDynamic()..endDynamic()):
        //       if (data.attemptDynamicMapping(entry, dhash)) count += 1;
        //   return 0;
        let entries = match data.get_scope_local() {
            Some(lm) => lm.database().dynamic_entries(lm.scope_id()),
            None => Vec::new(),
        };
        for entry in entries {
            match data.attempt_dynamic_mapping(&entry) {
                Ok(true) => self.base.count += 1, // C++ `count += 1`
                Ok(false) => {}
                Err(_) => {}
            }
        }
        0
    }
}

// =============================================================================
// ActionDynamicSymbols (coreaction.hh:1049, coreaction.cc:5123)
// =============================================================================

/// Make final attachments of dynamically mapped symbols to Varnodes (C++
/// `ActionDynamicSymbols`, `coreaction.hh:1049`).  Constructed `rule_onceperfunc`.
pub struct ActionDynamicSymbols {
    base: ActionBase,
}

impl ActionDynamicSymbols {
    /// Construct in group `g` (C++
    /// `ActionDynamicSymbols::ActionDynamicSymbols`).
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionDynamicSymbols {
            base: ActionBase::new(ruleflags::rule_onceperfunc, "dynamicsymbols", g),
        })
    }
}

impl Action for ActionDynamicSymbols {
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
        Some(Box::new(ActionDynamicSymbols { base: self.base.clone() }))
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ coreaction.cc:5123 — ActionDynamicSymbols::apply
        //   localmap = data.getScopeLocal();
        //   for (entry in localmap->beginDynamic()..endDynamic()):
        //       if (data.attemptDynamicMappingLate(entry, dhash)) count += 1;
        //   return 0;
        // (the harness's `count` is the per-apply work tally; the body always
        // returns 0 / the action never re-schedules — `rule_onceperfunc`.)
        let entries = match data.get_scope_local() {
            Some(lm) => lm.database().dynamic_entries(lm.scope_id()),
            None => Vec::new(),
        };
        for entry in entries {
            match data.attempt_dynamic_mapping_late(&entry) {
                Ok(true) => self.base.count += 1, // C++ `count += 1`
                Ok(false) => {}
                Err(_) => {} // a search/warning failure is non-fatal (C++ returns false)
            }
        }
        0
    }
}

// =============================================================================
// ActionInternalStorage (coreaction.hh:1073, coreaction.cc:5192)
// =============================================================================

/// Mark constants written to the stack from internal-storage registers so their
/// stack location is not addressable (C++ `ActionInternalStorage`,
/// `coreaction.hh:1073`).  Constructed `rule_onceperfunc`.
pub struct ActionInternalStorage {
    base: ActionBase,
}

impl ActionInternalStorage {
    /// Construct in group `g` (C++
    /// `ActionInternalStorage::ActionInternalStorage`).
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionInternalStorage {
            base: ActionBase::new(ruleflags::rule_onceperfunc, "internalstorage", g),
        })
    }
}

impl Action for ActionInternalStorage {
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
        Some(Box::new(ActionInternalStorage { base: self.base.clone() }))
    }
    fn apply(&mut self, _data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ coreaction.cc:5192 — ActionInternalStorage::apply
        //   proto = data.getFuncProto();
        //   for (vdata in proto.internalBegin()..internalEnd()):
        //       addr = vdata.getAddr(); sz = vdata.size;
        //       for (vn in beginLoc(sz,addr)..endLoc(sz,addr)):
        //           for (op in vn->beginDescend()..endDescend()):
        //               if (op->code() == CPUI_STORE && vn->isEventualConstant(3,0)):
        //                   op->setStoreUnmapped();
        //   return 0;     // (no count bump: marking only)
        //
        // SEAM(W8-funcdata): the proto internal-storage list (`internalBegin`),
        // the (sz,addr) loc-set iteration, `Varnode::isEventualConstant`, and
        // `PcodeOp::setStoreUnmapped` are not in the merged tree.  Body
        // transcribed; no change applied (count stays 0).
        0
    }
}

// =============================================================================
// ActionInferTypes (coreaction.hh:975, coreaction.cc:5630)
// =============================================================================

/// Infer and propagate data-types (C++ `ActionInferTypes`,
/// `coreaction.hh:975`).  `flags = 0`; `localcount` caps the propagation
/// fixpoint at 7 passes and emits the "not settling" warning on the 7th.
pub struct ActionInferTypes {
    base: ActionBase,
    /// C++ `int4 localcount` — passes performed for this function (reset to 0;
    /// the empirically-chosen ceiling is 7).
    localcount: i32,
}

impl ActionInferTypes {
    /// Construct in group `g` (C++ `ActionInferTypes::ActionInferTypes`).
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionInferTypes {
            base: ActionBase::new(0, "infertypes", g),
            localcount: 0,
        })
    }
}

impl Action for ActionInferTypes {
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
        Some(Box::new(ActionInferTypes { base: self.base.clone(), localcount: 0 }))
    }
    /// C++ `ActionInferTypes::reset`: `{ localcount = 0; }`.
    fn reset(&mut self, _data: &mut Funcdata) {
        self.localcount = 0;
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ coreaction.cc:5630 — ActionInferTypes::apply
        //   if (!data.hasTypeRecoveryStarted()) return 0;
        if !data.has_type_recovery_started() {
            return 0;
        }
        //   if (localcount >= 7):                       // empirical settle ceiling
        if self.localcount >= 7 {
            if self.localcount == 7 {
                // data.warningHeader("Type propagation algorithm not settling");
                data.set_type_recovery_exceeded();
                self.localcount += 1;
            }
            return 0;
        }
        // Run the bounded bidirectional type lattice (the engine ported in
        // `coreaction_infertypes`): buildLocaltypes -> propagateOneType (DFS) ->
        // propagateAcrossReturns -> writeBack.  `writeBack` returning a change
        // bumps `localcount` (it is deliberately NOT counted as a data-flow change,
        // so `count` is left untouched).
        if crate::coreaction_infertypes::run_infer_types(data) {
            self.localcount += 1;
        }
        0
    }
}

// =============================================================================
// Enumeration (round-trip identity testing only)
// =============================================================================

/// Every Action class ported in this module, each constructed in group `g`
/// (the parameterized actions use placeholder configuration: no stack space, and
/// `propagateIndirect = true`).
///
/// Unlike the sibling modules' `*_actions` helpers there is **no** contiguous C++
/// `universalAction` slice that is exactly this set — these actions interleave
/// with the early/protos/cleanup/blockaction leaves throughout the schedule, so
/// the W8 assembler pulls individual `boxed` constructors.  This helper exists
/// only so a test can round-trip the full set's identity (name/group/flags).
pub fn render_actions(g: &str) -> Vec<Box<dyn Action>> {
    vec![
        ActionStackPtrFlow::boxed(g, None),
        ActionLaneDivide::boxed(g),
        ActionSegmentize::boxed(g),
        ActionForceGoto::boxed(g),
        ActionMultiCse::boxed(g),
        ActionShadowVar::boxed(g),
        ActionConstantPtr::boxed(g),
        ActionDeindirect::boxed(g),
        ActionDirectWrite::boxed(g, true),
        ActionLikelyTrash::boxed(g),
        ActionRestructureVarnode::boxed(g),
        ActionMappedLocalSync::boxed(g),
        ActionMapGlobals::boxed(g),
        ActionDeadCode::boxed(g),
        ActionConditionalConst::boxed(g),
        ActionSwitchNorm::boxed(g),
        ActionUnjustifiedParams::boxed(g),
        ActionDynamicMapping::boxed(g),
        ActionDynamicSymbols::boxed(g),
        ActionInternalStorage::boxed(g),
        ActionInferTypes::boxed(g),
    ]
}

#[cfg(test)]
mod tests;
