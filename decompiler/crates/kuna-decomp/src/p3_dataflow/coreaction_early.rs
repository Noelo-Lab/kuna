//! Port of the **early S2/S3** Action classes from `decompiler/cpp/coreaction.{cc,hh}`.
//!
//! # What this is
//!
//! This file is the W5 `w5-s3-coreaction-early` item: the *early plane* of
//! [`Action`](crate::action::Action) classes that run before prototype recovery
//! begins.  These are the function-setup actions (`start`/`stop`/`startcleanup`/
//! `starttypes`), the SSA-construction wrappers (`constbase`/`spacebase`/
//! `heritage`/`nonzeromask`/`varnodeprops`), the basic-block-graph cleanup
//! actions (`unreachable`/`donothing`/`latedonothing`/`redundbranch`/
//! `determinedbranch`), and the normalization setup (`normalizesetup`).
//!
//! Each is an [`Action`] trait impl per the `action.rs` registration
//! convention: it embeds an [`ActionBase`] (the engine-owned name/group/flags/
//! status/breakpoint/counter store), keeps the **exact** `name()` string and
//! group/flags the C++ constructor used, and transcribes the `apply` body's
//! algorithm structure verbatim (same iteration order, tie-breakers, and change
//! signalling via `base_mut().count += 1`).
//!
//! # Boundary (where this item stops — drawn at a clean class line)
//!
//! Ported here, in **C++ definition order** (`coreaction.hh`):
//!
//! | C++ class | `name()` | group/flags | status |
//! |---|---|---|---|
//! | `ActionStart` | `"start"` | `0` | wrapper, STUB body |
//! | `ActionStop` | `"stop"` | `0` | wrapper, STUB body |
//! | `ActionStartCleanUp` | `"startcleanup"` | `0` | wrapper, STUB body |
//! | `ActionStartTypes` | `"starttypes"` | `0` | realized (`start_type_recovery`) |
//! | `ActionConstbase` | `"constbase"` | `0` | STUB body (W4 inject/track) |
//! | `ActionSpacebase` | `"spacebase"` | `0` | wrapper, STUB body |
//! | `ActionHeritage` | `"heritage"` | `0` | wrapper, STUB body (W4 driver) |
//! | `ActionNonzeroMask` | `"nonzeromask"` | `0` | wrapper, STUB body |
//! | `ActionVarnodeProps` | `"varnodeprops"` | `0` | STUB body (W3-vn/W4) |
//! | `ActionUnreachable` | `"unreachable"` | `0` | partial (read realized, surgery STUB) |
//! | `ActionDoNothing` | `"donothing"` | `rule_repeatapply` | partial (flags realized, surgery STUB) |
//! | `ActionLateDoNothing` | `"latedonothing"` | `0` | partial (flags realized, surgery STUB) |
//! | `ActionRedundBranch` | `"redundbranch"` | `0` | partial (splice realized, removeBranch STUB) |
//! | `ActionDeterminedBranch` | `"determinedbranch"` | `0` | realized (detect + `removeBranch`) |
//! | `ActionNormalizeSetup` | `"normalizesetup"` | `rule_onceperfunc` | realized `reset`, STUB body (W4 proto) |
//!
//! **The boundary** is the next class, [`ActionDeadCode`] (`coreaction.hh:565`,
//! `name() == "deadcode"`).  Its `apply` body needs `FuncCallSpecs` /
//! `Funcdata::getCallSpecs` / `markConsumedParameters` / `gatherConsumedReturn`
//! — the **prototype/call-spec analysis plane** (W6/W7).  `ActionDeadCode`,
//! `ActionConditionalConst`, `ActionSwitchNorm` and everything from
//! `ActionPrototypeTypes` onward are **left for the later coreaction items**
//! (W6/W7/W8), as the item brief directs.
//!
//! # Seams (do NOT invent these behaviors)
//!
//! Several early-action `apply` bodies are a single call into a `Funcdata`
//! primitive that is itself seamed to another wave and **not present in the
//! merged tree**:
//!
//! * SSA / processing primitives `startProcessing`, `stopProcessing`,
//!   `startCleanUp`, `spacebase`, `opHeritage`, `calcNZMask` (W3-op / W4 — the
//!   heritage driver itself is `// STUB(W4)` in `heritage.rs`).
//! * Block-graph surgery `removeUnreachableBlocks`, `removeBranch`,
//!   `removeDoNothingBlock`, and the predicates `BlockBasic::isDoNothing`,
//!   `unblockedMulti`, `hasNoImmediateCopy` (W3-block / `funcdata_block`).
//! * Varnode loc-set iteration `beginLoc`/`endLoc`, `getHeritagePass`, and the
//!   replace primitives `fillinReadOnly`/`replaceVolatile`/
//!   `totalReplaceConstant` (W3-vn / W4).
//! * Prototype / injection surface `getFuncProto().*`, `getArch()->context`,
//!   `pcodeinjectlib`, `doLiveInject` (W4 — `FuncProto` is an empty `Default`
//!   placeholder in `seams.rs`).
//!
//! Where a primitive is **realized** the body calls it and is exercised by a
//! test.  Where it is **seamed** the body transcribes the C++ structure and
//! routes the unrealized mutation through a `// STUB(...)`-noted private helper
//! that performs the realized read/predicate work and returns `0` changes (the
//! C++ contract: changes are signalled by incrementing `count`).  Every seam is
//! reported in this item's `losses` so the owning wave can finish the wiring.
//!
//! # Registration
//!
//! W8 assembles `universalAction` by constructing these leaf actions with
//! [`ActionGroup::add_action`](crate::action::ActionGroup::add_action).  The
//! constructor convention mirrors the C++ `Action(flags,name,group)` ctor: each
//! type exposes `new(group)` returning the boxed action.  [`early_actions`]
//! enumerates the full early set in C++ definition order for the W8 assembler.

use kuna_base::types::{int4, uintb};
use kuna_num::opcodes::OpCode;

use crate::action::{ruleflags, Action, ActionBase, ActionContext, ActionGroupList, ApplyResult};
use crate::funcdata::Funcdata;

// =============================================================================
// Setup actions (coreaction.hh:34-86)
// =============================================================================

/// Gather raw p-code for a function (C++ `ActionStart`, `coreaction.hh:34`).
///
/// `apply` is `data.startProcessing(); return 0;`.
pub struct ActionStart {
    base: ActionBase,
}

impl ActionStart {
    /// Construct in group `g` (C++ `ActionStart::ActionStart`).
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionStart { base: ActionBase::new(0, "start", g) })
    }
}

impl Action for ActionStart {
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
        Some(Box::new(ActionStart { base: self.base.clone() }))
    }
    fn apply(&mut self, _data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++: data.startProcessing(); return 0;
        // STUB(W3-op/W4): Funcdata::startProcessing (the per-function p-code
        // gather / processing-start bookkeeping) is not in the merged tree.
        0
    }
}

/// Do any post-processing after decompilation (C++ `ActionStop`,
/// `coreaction.hh:46`).
///
/// `apply` is `data.stopProcessing(); return 0;`.
pub struct ActionStop {
    base: ActionBase,
}

impl ActionStop {
    /// Construct in group `g` (C++ `ActionStop::ActionStop`).
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionStop { base: ActionBase::new(0, "stop", g) })
    }
}

impl Action for ActionStop {
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
        Some(Box::new(ActionStop { base: self.base.clone() }))
    }
    fn apply(&mut self, _data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++: data.stopProcessing(); return 0;
        // STUB(W3-op/W4): Funcdata::stopProcessing not in the merged tree.
        0
    }
}

/// Start clean up after main transform phase (C++ `ActionStartCleanUp`,
/// `coreaction.hh:58`).
///
/// `apply` is `data.startCleanUp(); return 0;`.
pub struct ActionStartCleanUp {
    base: ActionBase,
}

impl ActionStartCleanUp {
    /// Construct in group `g` (C++ `ActionStartCleanUp::ActionStartCleanUp`).
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionStartCleanUp { base: ActionBase::new(0, "startcleanup", g) })
    }
}

impl Action for ActionStartCleanUp {
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
        Some(Box::new(ActionStartCleanUp { base: self.base.clone() }))
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++: data.startCleanUp(); return 0;
        data.start_clean_up();
        0
    }
}

/// Allow type recovery to start happening (C++ `ActionStartTypes`,
/// `coreaction.hh:74`).
///
/// `reset` marks the function for data-type analysis; `apply` flags that
/// analysis has started.  Both Funcdata hooks are **realized** in the merged
/// tree (`set_type_recovery` / `start_type_recovery`).
pub struct ActionStartTypes {
    base: ActionBase,
}

impl ActionStartTypes {
    /// Construct in group `g` (C++ `ActionStartTypes::ActionStartTypes`).
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionStartTypes { base: ActionBase::new(0, "starttypes", g) })
    }
}

impl Action for ActionStartTypes {
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
        Some(Box::new(ActionStartTypes { base: self.base.clone() }))
    }
    /// C++ `ActionStartTypes::reset`: `{ data.setTypeRecovery(true); }`.
    ///
    /// Faithful: the C++ override *replaces* `Action::reset` and does **not**
    /// chain to it, so the base status/warning latch is intentionally left
    /// untouched here — only the function's type-recovery flag is set.
    fn reset(&mut self, data: &mut Funcdata) {
        data.set_type_recovery(true);
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++: if (data.startTypeRecovery()) count+=1; return 0;
        if data.start_type_recovery() {
            self.base.count += 1;
        }
        0
    }
}

// =============================================================================
// SSA-construction wrappers (coreaction.hh:259-301) + ActionVarnodeProps
// =============================================================================

/// Search for input Varnodes that have officially-provided constant values
/// (C++ `ActionConstbase`, `coreaction.hh:259`).
///
/// The C++ body injects `uponentry` p-code for the prototype model and lays
/// down COPY ops for every \e tracked register with a user-supplied constant.
pub struct ActionConstbase {
    base: ActionBase,
}

impl ActionConstbase {
    /// Construct in group `g` (C++ `ActionConstbase::ActionConstbase`).
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionConstbase { base: ActionBase::new(0, "constbase", g) })
    }
}

impl Action for ActionConstbase {
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
        Some(Box::new(ActionConstbase { base: self.base.clone() }))
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ coreaction.cc:693
        //   if (data.getBasicBlocks().getSize()==0) return 0;   // No blocks
        //   BlockBasic *bb = ...getBlock(0);                     // entry block
        //   int4 injectid = data.getFuncProto().getInjectUponEntry();
        //   if (injectid >= 0) { ...doLiveInject(payload,...); }
        //   const TrackedSet trackset(...context->getTrackedSet(...));
        //   for each tracked ctx:  newOp/newVarnodeOut/newConstant/opSet*; opInsertBegin
        if data.bblocks_get_size() == 0 {
            return 0; // No blocks
        }
        // Entry block (block 0) — constructed to have nothing falling into it.
        let bb = data.bblocks_get_block(0);
        let bb_start = data.bblocks_block_start(bb);

        // STUB(W4): getFuncProto().getInjectUponEntry + doLiveInject are the
        // upon-entry pcode-inject surface (FuncProto is an empty Default
        // placeholder in seams.rs).  None of the tracked-register datatests drive
        // it, so it stays deferred; the tracked-set COPY injection below is the
        // half `set track <reg> <val> [start end]` (ifacedecomp.cc `IfcSettrackedrange`)
        // depends on.

        //   const TrackedSet trackset(...context->getTrackedSet(data.getAddress()));
        // C++ takes a copy; the per-function `glb` skeleton holds a snapshot of the
        // engine track base (taken at `build_arch_handle`), so clone the queried set
        // out before mutating the funcdata.
        let func_addr = data.get_address().clone();
        let arch = data.get_arch().clone();
        let trackset: Vec<kuna_sleigh::globalcontext::TrackedContext> =
            arch.get_tracked_set(&func_addr).clone();
        if trackset.is_empty() {
            return 0;
        }

        // For each tracked register: emit a `COPY #val` whose output is the
        // register storage, inserted at the start of the entry block (C++
        // coreaction.cc:709-719).
        for ctx in &trackset {
            // Address addr(ctx.loc.space, ctx.loc.offset);
            let space = ctx
                .loc
                .space
                .as_ref()
                .expect("ActionConstbase: tracked register has null space")
                .clone();
            let addr = kuna_base::address::Address::new(space, ctx.loc.offset);
            let size = ctx.loc.size as int4;
            // op = newOp(1, bb->getStart());
            let op = data.new_op(1, bb_start.clone());
            // newVarnodeOut(ctx.loc.size, addr, op);
            data.new_varnode_out(size, &addr, op).expect("ActionConstbase: newVarnodeOut");
            // vnin = newConstant(ctx.loc.size, ctx.val);
            let vnin = data.new_constant(size, ctx.val);
            // opSetOpcode(op, CPUI_COPY); opSetInput(op, vnin, 0); opInsertBegin(op, bb);
            data.op_set_opcode_code(op, OpCode::CPUI_COPY);
            data.op_set_input(op, vnin, 0).expect("ActionConstbase: opSetInput");
            data.op_insert_begin(op, bb);
        }
        0
    }
}

/// Mark Varnode objects that hold stack-pointer values and set up special
/// data-types (C++ `ActionSpacebase`, `coreaction.hh:270`).
///
/// `apply` is `data.spacebase(); return 0;`.
pub struct ActionSpacebase {
    base: ActionBase,
}

impl ActionSpacebase {
    /// Construct in group `g` (C++ `ActionSpacebase::ActionSpacebase`).
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionSpacebase { base: ActionBase::new(0, "spacebase", g) })
    }
}

impl Action for ActionSpacebase {
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
        Some(Box::new(ActionSpacebase { base: self.base.clone() }))
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ coreaction.cc:1648 — ActionSpacebase::apply: data.spacebase();
        data.spacebase();
        0
    }
}

/// Build the SSA representation for the function (C++ `ActionHeritage`,
/// `coreaction.hh:282`).
///
/// `apply` is `data.opHeritage(); return 0;` — one pass of the heritage driver.
pub struct ActionHeritage {
    base: ActionBase,
}

impl ActionHeritage {
    /// Construct in group `g` (C++ `ActionHeritage::ActionHeritage`).
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionHeritage { base: ActionBase::new(0, "heritage", g) })
    }
}

impl Action for ActionHeritage {
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
        Some(Box::new(ActionHeritage { base: self.base.clone() }))
    }
    fn apply(&mut self, data: &mut Funcdata, ctx: &mut ActionContext) -> ApplyResult {
        // C++ coreaction.hh:289 — ActionHeritage::apply: data.opHeritage(); return 0;
        // `opHeritage` drives the owned Heritage engine (heritage.rs), mutating
        // the live Funcdata into SSA form (reads linked to writes, MULTIEQUAL
        // phi-nodes placed at dominance frontiers).  The action always reports 0
        // changes (the C++ does too — heritage registers no change count, which
        // is why `break action heritage` never fires; see kuna/goldens.py B3).
        // (kuna decompile-all watchdog) thread the per-function deadline into
        // the heritage per-space loop; `None` everywhere but a budgeted drive.
        data.op_heritage_with_deadline(ctx.deadline);
        // (kuna) Repair any synthetic lowered-switch BRANCHIND whose input
        // heritage normalized away (see `kuna_repair_lowered_switch_inputs`).
        // A no-op unless a lowered-switch was installed this function, so it
        // never perturbs the datatest corpus.
        data.kuna_repair_lowered_switch_inputs();
        0
    }
}

/// Calculate the non-zero mask property on all Varnodes (C++ `ActionNonzeroMask`,
/// `coreaction.hh:293`).
///
/// `apply` is `data.calcNZMask(); return 0;`.
pub struct ActionNonzeroMask {
    base: ActionBase,
}

impl ActionNonzeroMask {
    /// Construct in group `g` (C++ `ActionNonzeroMask::ActionNonzeroMask`).
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionNonzeroMask { base: ActionBase::new(0, "nonzeromask", g) })
    }
}

impl Action for ActionNonzeroMask {
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
        Some(Box::new(ActionNonzeroMask { base: self.base.clone() }))
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++: data.calcNZMask(); return 0;
        data.calc_nz_mask();
        0
    }
}

/// Apply special properties (read-only / volatile) to Varnodes built out of
/// legal parameters (C++ `ActionVarnodeProps`, `coreaction.hh:222`).
///
/// The C++ walks the loc-set, clearing `AutoLiveHold` once heritage has run,
/// replacing read-only / volatile Varnodes with their image lookup / p-code, and
/// const-folding Varnodes whose consumed-bits and non-zero mask are disjoint.
pub struct ActionVarnodeProps {
    base: ActionBase,
}

impl ActionVarnodeProps {
    /// Construct in group `g` (C++ `ActionVarnodeProps::ActionVarnodeProps`).
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionVarnodeProps { base: ActionBase::new(0, "varnodeprops", g) })
    }
}

impl Action for ActionVarnodeProps {
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
        Some(Box::new(ActionVarnodeProps { base: self.base.clone() }))
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ coreaction.cc:1298.
        //   Architecture *glb = data.getArch();
        //   bool cachereadonly = glb->readonlypropagate;
        //   int4 pass = data.getHeritagePass();
        let cachereadonly = data.get_arch().readonly_propagate();
        let pass = data.get_heritage_pass();

        // iter = data.beginLoc(); while(iter != data.endLoc()) { vn = *iter++; ... }
        // The C++ advances the iterator before vn may be deleted/rewritten; the
        // faithful Rust mirror snapshots the loc-set order once (a vn only ever
        // loses descendants here, never gains them, so a snapshot is sound).
        let locs: Vec<crate::context::VarnodeId> = data.vbank().iter_loc().collect();
        for vn in locs {
            // The varnode may have been freed by an earlier replace (the C++
            // `*iter++` already moved past it; a freed-mid-loop vn is simply
            // skipped — its descend list is now empty).
            let info = match data.vbank().get(vn) {
                Some(v) => (
                    v.is_annotation(),
                    v.get_size(),
                    v.is_auto_live_hold(),
                    v.is_written(),
                    v.has_action_property(),
                    v.is_read_only(),
                    v.is_volatile(),
                    v.get_nz_mask(),
                    v.get_consume(),
                    v.is_constant(),
                    v.has_no_descend(),
                    v.get_def(),
                ),
                None => continue,
            };
            let (
                is_annotation,
                vn_size,
                is_auto_live_hold,
                is_written,
                has_action_property,
                is_read_only,
                is_volatile,
                nz_mask,
                consume,
                is_constant,
                has_no_descend,
                def,
            ) = info;

            // if (vn->isAnnotation()) continue;
            if is_annotation {
                continue;
            }

            if is_auto_live_hold {
                // if (pass > 0) { ... clearAutoLiveHold(); count+=1; }
                if pass > 0 {
                    // The C++ skips clearing when the value is a LOAD through a
                    // constant/readonly pointer (possibly behind one COPY), so the
                    // hold survives until the LOAD has been resolved.
                    let mut skip = false;
                    if is_written {
                        let loadop = def.expect("varnodeprops: written vn has no def");
                        if data.obank().get(loadop).map(|o| o.code()) == Some(OpCode::CPUI_LOAD) {
                            // Varnode *ptr = loadOp->getIn(1);
                            let mut ptr = data.obank().get(loadop).and_then(|o| o.get_in(1));
                            if let Some(p) = ptr {
                                let (pc, pro, pw, pdef) = {
                                    let v = data.vbank().get(p).expect("varnodeprops: stale ptr");
                                    (v.is_constant(), v.is_read_only(), v.is_written(), v.get_def())
                                };
                                if pc || pro {
                                    skip = true;
                                } else if pw {
                                    // if (ptr->isWritten()) { copyOp = ptr->getDef(); ... }
                                    let copyop = pdef.expect("varnodeprops: written ptr has no def");
                                    if data.obank().get(copyop).map(|o| o.code())
                                        == Some(OpCode::CPUI_COPY)
                                    {
                                        ptr = data.obank().get(copyop).and_then(|o| o.get_in(0));
                                        if let Some(p2) = ptr {
                                            let (c2, r2) = {
                                                let v = data
                                                    .vbank()
                                                    .get(p2)
                                                    .expect("varnodeprops: stale ptr2");
                                                (v.is_constant(), v.is_read_only())
                                            };
                                            if c2 || r2 {
                                                skip = true;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    if !skip {
                        data.vbank_mut()
                            .get_mut(vn)
                            .expect("varnodeprops: stale vn")
                            .clear_auto_live_hold();
                        self.base.count += 1;
                    }
                }
            } else if has_action_property {
                if cachereadonly && is_read_only {
                    // if (data.fillinReadOnly(vn)) count += 1;
                    if data.fillin_read_only(vn).unwrap_or(false) {
                        self.base.count += 1;
                    }
                } else if is_volatile {
                    // if (data.replaceVolatile(vn)) count += 1;
                    if data.replace_volatile(vn).unwrap_or(false) {
                        self.base.count += 1;
                    }
                }
            } else if (nz_mask & consume) == 0 && (vn_size as usize) <= std::mem::size_of::<uintb>() {
                // FIXME: uintb should be arbitrary precision
                if is_constant {
                    continue; // Don't replace a constant
                }
                // Don't replace a COPY of a non-zero constant (let constant
                // propagation do COPY-0 -> 0, avoiding infinite recursion).
                if is_written {
                    let defop = def.expect("varnodeprops: written vn has no def");
                    if data.obank().get(defop).map(|o| o.code()) == Some(OpCode::CPUI_COPY) {
                        let in0 = data.obank().get(defop).and_then(|o| o.get_in(0));
                        if let Some(i0) = in0 {
                            let (i0_const, i0_off) = {
                                let v = data.vbank().get(i0).expect("varnodeprops: stale copy in");
                                (v.is_constant(), v.get_offset())
                            };
                            if i0_const && i0_off == 0 {
                                continue;
                            }
                        }
                    }
                }
                if !has_no_descend {
                    let _ = data.total_replace_constant(vn, 0);
                    self.base.count += 1;
                }
            }
        }
        0
    }
}

// =============================================================================
// Block-graph cleanup actions (coreaction.hh:492-545)
// =============================================================================

/// Remove unreachable blocks (C++ `ActionUnreachable`, `coreaction.hh:492`).
///
/// `apply` is `if (data.removeUnreachableBlocks(true,false)) count+=1; return 0;`.
pub struct ActionUnreachable {
    base: ActionBase,
}

impl ActionUnreachable {
    /// Construct in group `g` (C++ `ActionUnreachable::ActionUnreachable`).
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionUnreachable { base: ActionBase::new(0, "unreachable", g) })
    }
}

impl Action for ActionUnreachable {
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
        Some(Box::new(ActionUnreachable { base: self.base.clone() }))
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ coreaction.cc:3560
        //   if (data.removeUnreachableBlocks(true,false)) count += 1;
        //   return 0;
        // The reachability sweep + block deletion is now realized: unreachable
        // blocks are collected from the entry's reachable set, severed, and
        // removed (stranded reads rewritten to 0xBADDEF via descend2Undef).  A
        // LowlevelError on the surgery path (C++ `throw`) surfaces as a panic.
        if data
            .remove_unreachable_blocks(true, false)
            .expect("ActionUnreachable: removeUnreachableBlocks")
        {
            self.base.count += 1;
        }
        0
    }
}

/// Remove blocks that do nothing (C++ `ActionDoNothing`, `coreaction.hh:503`).
///
/// `rule_repeatapply`: the C++ removes one do-nothing block per call and re-runs
/// until none remain.
pub struct ActionDoNothing {
    base: ActionBase,
}

impl ActionDoNothing {
    /// Construct in group `g` (C++ `ActionDoNothing::ActionDoNothing`).
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionDoNothing {
            base: ActionBase::new(ruleflags::rule_repeatapply, "donothing", g),
        })
    }
}

impl Action for ActionDoNothing {
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
        Some(Box::new(ActionDoNothing { base: self.base.clone() }))
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ coreaction.cc:3569 — walk graph blocks:
        //   bb->clearDelayedDonothing();
        //   if (bb->isDoNothing()) {
        //     if (sizeOut==1 && getOut(0)==bb) {          // infinite loop
        //       if (!isDonothingLoop()) { setDonothingLoop(); data.warning(...); }
        //     } else if (bb->unblockedMulti(0)) {
        //       if (data.isNormalizationOn() || bb->hasNoImmediateCopy(0)) {
        //         data.removeDoNothingBlock(bb); count += 1; return 0;
        //       } else bb->setDelayedDonothing();
        //     }
        //   }
        //
        // The `clearDelayedDonothing()` sweep runs on every block on every call,
        // independent of removal.  Then we walk for do-nothing blocks: the first
        // removable one is removed and we return (C++ removes one per call and
        // re-runs under `rule_repeatapply`).
        let size = data.bblocks_get_size();
        for i in 0..size {
            let bb = data.bblocks_get_block(i);
            data.bblocks_mut().block_mut(bb).clear_delayed_donothing();
        }
        let size = data.bblocks_get_size();
        for i in 0..size {
            let bb = data.bblocks_get_block(i);
            if !data.bb_is_do_nothing(bb) {
                continue;
            }
            // Infinite-loop do-nothing block: warn once, mark, leave in place.
            if data.bblocks_ref().block(bb).size_out() == 1
                && data.bblocks_ref().block(bb).get_out(0) == bb
            {
                if !data.bblocks_ref().block(bb).is_donothing_loop() {
                    data.bblocks_mut().block_mut(bb).set_donothing_loop();
                    // data.warning("Do nothing block with infinite loop", ...) -- W4
                    // warning facility; the mark is the realized side effect.
                }
            } else if data.bb_unblocked_multi(bb, 0) {
                if data.is_normalization_on() || data.bb_has_no_immediate_copy(bb, 0) {
                    if data.remove_do_nothing_block(bb).is_ok() {
                        self.base.count += 1;
                        return 0;
                    }
                } else {
                    // Immediate COPYs but otherwise removable -> defer to
                    // ActionLateDoNothing (after the COPYs are trimmed).
                    data.bblocks_mut().block_mut(bb).set_delayed_donothing();
                }
            }
        }
        0
    }
}

/// Remove blocks that do nothing after variable merging (C++ `ActionLateDoNothing`,
/// `coreaction.hh:514`).
pub struct ActionLateDoNothing {
    base: ActionBase,
}

impl ActionLateDoNothing {
    /// Construct in group `g` (C++ `ActionLateDoNothing::ActionLateDoNothing`).
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionLateDoNothing { base: ActionBase::new(0, "latedonothing", g) })
    }

    /// Does removing the given block create a redundant branch point?
    /// (C++ `ActionLateDoNothing::removingCreatesRedundancy`, coreaction.cc:3604).
    ///
    /// For each in-edge of `bl`, if every *other* out-edge of that predecessor
    /// already leads to `bl`'s single successor (`outbl`, possibly via `bl`),
    /// removing `bl` would create a redundant edge.  Pure block-graph read; all
    /// accessors (`size_out`/`size_in`/`get_out`/`get_in`) are realized.
    fn removing_creates_redundancy(data: &Funcdata, bl: crate::context::BlockId) -> bool {
        let graph = data.bblocks_ref();
        let outbl = graph.block(bl).get_out(0);
        let size_in = graph.block(bl).size_in();
        for i in 0..size_in {
            let inbl = graph.block(bl).get_in(i);
            if graph.block(inbl).size_out() == 1 {
                continue;
            }
            let inbl_size_out = graph.block(inbl).size_out();
            let mut count = 0;
            while count < inbl_size_out {
                let curbl = graph.block(inbl).get_out(count);
                // Check if this edge goes to outbl (possibly via bl)
                if curbl != bl && curbl != outbl {
                    break;
                }
                count += 1;
            }
            if count == inbl_size_out {
                // All edges lead to outbl
                return true;
            }
        }
        false
    }
}

impl Action for ActionLateDoNothing {
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
        Some(Box::new(ActionLateDoNothing { base: self.base.clone() }))
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ coreaction.cc:3623 — collect then remove:
        //   for each block with isDelayedDonothing():
        //     if (!isDoNothing()) continue;
        //     if (removingCreatesRedundancy(bb)) continue;
        //     if (sizeOut==1 && getOut(0)==bb) { ...donothing loop warning... }
        //     else if (unblockedMulti(0)) removeList.push_back(bb);
        //   for each in removeList: removeDoNothingBlock(bb); count += 1;
        //
        // Collect the removable delayed-do-nothing blocks, then remove them.
        let size = data.bblocks_get_size();
        let mut remove_list: Vec<crate::context::BlockId> = Vec::new();
        for i in 0..size {
            let bb = data.bblocks_get_block(i);
            if !data.bblocks_ref().block(bb).is_delayed_donothing() {
                continue;
            }
            if !data.bb_is_do_nothing(bb) {
                continue;
            }
            if ActionLateDoNothing::removing_creates_redundancy(data, bb) {
                continue;
            }
            if data.bblocks_ref().block(bb).size_out() == 1
                && data.bblocks_ref().block(bb).get_out(0) == bb
            {
                // Infinite loop: warn once, mark, leave in place.
                if !data.bblocks_ref().block(bb).is_donothing_loop() {
                    data.bblocks_mut().block_mut(bb).set_donothing_loop();
                    // data.warning("Do nothing block with infinite loop", ...) -- W4
                }
            } else if data.bb_unblocked_multi(bb, 0) {
                remove_list.push(bb);
            }
        }
        for bb in remove_list {
            if data.remove_do_nothing_block(bb).is_ok() {
                self.base.count += 1;
            }
        }
        0
    }
}

/// Get rid of redundant branches: duplicate edges between the same input and
/// output block (C++ `ActionRedundBranch`, `coreaction.hh:526`).
pub struct ActionRedundBranch {
    base: ActionBase,
}

impl ActionRedundBranch {
    /// Construct in group `g` (C++ `ActionRedundBranch::ActionRedundBranch`).
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionRedundBranch { base: ActionBase::new(0, "redundbranch", g) })
    }
}

impl Action for ActionRedundBranch {
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
        Some(Box::new(ActionRedundBranch { base: self.base.clone() }))
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ coreaction.cc:3650 — for i over graph blocks:
        //   bb = getBlock(i);
        //   if (sizeOut==0) continue;
        //   bl = bb->getOut(0);
        //   if (sizeOut==1) {
        //     if (bl->sizeIn==1 && !bl->isEntryPoint() && !bb->isSwitchOut()) {
        //       data.spliceBlockBasic(bb); count += 1; i = -1;  // reset scan
        //     }
        //     continue;
        //   }
        //   for (j=1;j<sizeOut;++j) if (getOut(j) != bl) break;
        //   if (j != sizeOut) continue;             // not all exits to bl
        //   data.removeBranch(bb,1); count += 1;    // duplicate n-way -> remove edge
        //
        // The single-out **splice** path is realized (Funcdata::splice_block_basic
        // exists, though it itself `Err`s on a trailing branch op — STUB(W3-op));
        // the n-way `removeBranch` is a funcdata_block seam.
        let mut i: i32 = 0;
        while i < data.bblocks_get_size() {
            let bb = data.bblocks_get_block(i);
            let size_out = data.bblocks_ref().block(bb).size_out();
            if size_out == 0 {
                i += 1;
                continue;
            }
            let bl = data.bblocks_ref().block(bb).get_out(0);
            if size_out == 1 {
                let bl_size_in = data.bblocks_ref().block(bl).size_in();
                let bl_is_entry = data.bblocks_ref().block(bl).is_entry_point();
                let bb_is_switch = data.bblocks_ref().block(bb).is_switch_out();
                // Do not splice a block coming from a single-exit switch — this
                // prevents possible second-stage recovery.
                if bl_size_in == 1 && !bl_is_entry && !bb_is_switch {
                    // splice_block_basic Errs when the spliced block ends in a
                    // branch op (STUB(W3-op)); a successful splice counts as a
                    // change and resets the scan, exactly as C++ `i = -1`.
                    if data.splice_block_basic(bb).is_ok() {
                        self.base.count += 1;
                        i = -1; // This removed one block, so reset i
                    }
                }
                i += 1;
                continue;
            }
            // Are all exits to the same block (bl)?
            let mut j = 1;
            while j < size_out {
                if data.bblocks_ref().block(bb).get_out(j) != bl {
                    break;
                }
                j += 1;
            }
            if j != size_out {
                i += 1;
                continue;
            }
            // data.removeBranch(bb,1); count += 1;  — all exits go to `bl`, so the
            // branch decision is redundant: sever the duplicate edge (and drop the
            // now-decisionless CBRANCH).  C++ does NOT reset the scan here (unlike
            // the splice path), it just continues with `++i`.
            self.base.count += data.remove_branch(bb, 1).map(|_| 1).unwrap_or(0);
            i += 1;
        }
        0 // Indicate full rule was applied
    }
}

/// Remove conditional branches if the condition is constant (C++
/// `ActionDeterminedBranch`, `coreaction.hh:537`).
pub struct ActionDeterminedBranch {
    base: ActionBase,
}

impl ActionDeterminedBranch {
    /// Construct in group `g` (C++ `ActionDeterminedBranch::ActionDeterminedBranch`).
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionDeterminedBranch { base: ActionBase::new(0, "determinedbranch", g) })
    }
}

impl Action for ActionDeterminedBranch {
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
        Some(Box::new(ActionDeterminedBranch { base: self.base.clone() }))
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ coreaction.cc:3688 — for i over graph blocks:
        //   cbranch = bb->lastOp();
        //   if (cbranch==0 || cbranch->code() != CPUI_CBRANCH) continue;
        //   if (!cbranch->getIn(1)->isConstant()) continue;
        //   uintb val = cbranch->getIn(1)->getOffset();
        //   int4 num = ((val!=0)!=cbranch->isBooleanFlip()) ? 0 : 1;
        //   data.removeBranch(bb,num); count += 1;
        //
        // The last op of each block is a CBRANCH on a constant condition; the dead
        // edge `num` is severed by `removeBranch`, collapsing the conditional into
        // an unconditional branch.  `removeBranch` (funcdata_block) is now ported
        // (used by `ActionUnreachable` above), so this is a complete transcription.
        // `branchRemoveInternal` only severs an out-edge — it never removes a block
        // — so `bblocks_get_size()` is stable across the loop (matching the C++
        // `for(i=0;i<graph.getSize();++i)`).
        let size = data.bblocks_get_size();
        for i in 0..size {
            let bb = data.bblocks_get_block(i);
            let cbranch = match data.bb_op_tail(bb) {
                Some(op) => op,
                None => continue, // lastOp() == (PcodeOp *)0
            };
            let (code, nin, is_flip) = {
                let cbranch_op = data.obank().get(cbranch).expect("determinedbranch: cbranch");
                (cbranch_op.code(), cbranch_op.num_input(), cbranch_op.is_boolean_flip())
            };
            if code != OpCode::CPUI_CBRANCH {
                continue;
            }
            // A valid CBRANCH always has 2 inputs; a transient sub-2-input op (e.g.
            // mid-conversion) is not a determined branch — skip it (C++ `getIn(1)`
            // assumes the canonical form, which `bb_op_tail` may briefly violate).
            if nin < 2 {
                continue;
            }
            let cond_vn_id = data.obank().get(cbranch).expect("determinedbranch: cbranch").get_in(1);
            let cond_vn_id = match cond_vn_id {
                Some(v) => v,
                None => continue,
            };
            let val: uintb = {
                let cond_vn = data.vbank().get(cond_vn_id).expect("determinedbranch: cond vn");
                if !cond_vn.is_constant() {
                    continue;
                }
                cond_vn.get_offset()
            };
            // int4 num = ((val!=0)!=cbranch->isBooleanFlip()) ? 0 : 1;
            let num: kuna_base::types::int4 = if (val != 0) != is_flip { 0 } else { 1 };
            // data.removeBranch(bb,num); count += 1;
            self.base.count += data.remove_branch(bb, num).map(|_| 1).unwrap_or(0);
        }
        0
    }
}

// =============================================================================
// Normalize setup (coreaction.hh:642)
// =============================================================================

/// Prepare function prototypes for the "normalize" simplification style (C++
/// `ActionNormalizeSetup`, `coreaction.hh:642`).
///
/// `reset` marks the function as being normalized (realized via
/// `set_normalization`); `apply` strips input/output/model locks from the
/// prototype.
pub struct ActionNormalizeSetup {
    base: ActionBase,
}

impl ActionNormalizeSetup {
    /// Construct in group `g` (C++ `ActionNormalizeSetup::ActionNormalizeSetup`).
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionNormalizeSetup {
            base: ActionBase::new(ruleflags::rule_onceperfunc, "normalizesetup", g),
        })
    }
}

impl Action for ActionNormalizeSetup {
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
        Some(Box::new(ActionNormalizeSetup { base: self.base.clone() }))
    }
    /// C++ `ActionNormalizeSetup::reset`: `{ data.setNormalization(true); }`.
    ///
    /// Faithful: the C++ override *replaces* `Action::reset` (does not chain to
    /// it), so the base status/warning latch is intentionally left untouched
    /// here — only the function's normalization flag is set.
    fn reset(&mut self, data: &mut Funcdata) {
        data.set_normalization(true);
    }
    fn apply(&mut self, _data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ coreaction.cc:4801
        //   FuncProto &fp(data.getFuncProto());
        //   fp.clearInput();
        //   fp.setModelLock(false);   // forces model re-evaluation
        //   fp.setOutputLock(false);
        //   return 0;
        // STUB(W4): FuncProto is an empty Default placeholder in seams.rs;
        // clearInput / setModelLock / setOutputLock are W4 prototype surfaces.
        // The realized `reset` already flips the function into normalization mode;
        // the lock-stripping is deferred to the prototype wave (no change count).
        0
    }
}

// =============================================================================
// W8 registration hook
// =============================================================================

/// Construct the full early-action set in **C++ definition order**
/// (`coreaction.hh`), all bound to `group`.
///
/// W8 assembles `universalAction` by pushing these into the appropriate
/// [`ActionGroup`](crate::action::ActionGroup) via
/// [`add_action`](crate::action::ActionGroup::add_action).  The real
/// `universalAction` interleaves these with actions from other coreaction items
/// (and assigns each its own group string); this helper just enumerates the
/// classes this item owns so the assembler has a single ordered source.
pub fn early_actions(group: &str) -> Vec<Box<dyn Action>> {
    vec![
        ActionStart::boxed(group),
        ActionStop::boxed(group),
        ActionStartCleanUp::boxed(group),
        ActionStartTypes::boxed(group),
        ActionConstbase::boxed(group),
        ActionSpacebase::boxed(group),
        ActionHeritage::boxed(group),
        ActionNonzeroMask::boxed(group),
        ActionVarnodeProps::boxed(group),
        ActionUnreachable::boxed(group),
        ActionDoNothing::boxed(group),
        ActionLateDoNothing::boxed(group),
        ActionRedundBranch::boxed(group),
        ActionDeterminedBranch::boxed(group),
        ActionNormalizeSetup::boxed(group),
    ]
}

#[cfg(test)]
mod tests;
