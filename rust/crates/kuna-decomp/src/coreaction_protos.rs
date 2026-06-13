//! Port of the **S4 prototype-recovery** Action classes from
//! `decompiler/cpp/coreaction.{cc,hh}`.
//!
//! # What this is
//!
//! This file is the W6 `w6-s4-coreaction-protos` item: the *prototype/call-spec
//! analysis plane* of [`Action`](crate::action::Action) classes.  Where
//! [`coreaction_early`](crate::coreaction_early) stops at the clean class line
//! `ActionDeadCode` (the first action needing `FuncCallSpecs` /
//! `Funcdata::getCallSpecs`), this file picks up the S4 actions that set up,
//! drive, and finalize sub-function parameter/return recovery and the function's
//! own input/output prototype.
//!
//! Each is an [`Action`] trait impl per the `action.rs` registration
//! convention: it embeds an [`ActionBase`] (the engine-owned name/group/flags/
//! status/breakpoint/counter store), keeps the **exact** `name()` string and
//! group/flags the C++ constructor used, and a `clone_filtered` that mirrors the
//! C++ `clone(grouplist)` group filter.  Change signalling is via
//! `base_mut().count += 1` (the C++ `count += 1`).
//!
//! # Class list (this item), in C++ definition order (`coreaction.hh`)
//!
//! | C++ class | `name()` | flags | C++ `apply` |
//! |---|---|---|---|
//! | `ActionPrototypeTypes` | `"prototypetypes"` | `rule_onceperfunc` | `coreaction.cc:4843` |
//! | `ActionDefaultParams` | `"defaultparams"` | `rule_onceperfunc` | `coreaction.cc:2369` |
//! | `ActionExtraPopSetup` | `"extrapopsetup"` | `rule_onceperfunc` | `coreaction.cc:1452` |
//! | `ActionFuncLink` | `"funclink"` | `rule_onceperfunc` | `coreaction.cc:1619` |
//! | `ActionFuncLinkOutOnly` | `"funclink_outonly"` | `rule_onceperfunc` | `coreaction.cc:1632` |
//! | `ActionParamDouble` | `"paramdouble"` | `0` | `coreaction.cc:1641` |
//! | `ActionActiveParam` | `"activeparam"` | `0` | `coreaction.cc:1769` |
//! | `ActionActiveReturn` | `"activereturn"` | `0` | `coreaction.cc:1817` |
//! | `ActionReturnRecovery` | `"returnrecovery"` | `0` | `coreaction.cc:1954` |
//! | `ActionRestrictLocal` | `"restrictlocal"` | `0` | `coreaction.cc:2003` |
//! | `ActionInputPrototype` | `"inputprototype"` | `rule_onceperfunc` | `coreaction.cc:4941` |
//! | `ActionOutputPrototype` | `"outputprototype"` | `rule_onceperfunc` | `coreaction.cc:4999` |
//! | `ActionPrototypeWarnings` | `"prototypewarnings"` | `rule_onceperfunc` | `coreaction.cc:5140` |
//!
//! # Boundary (where this item stops)
//!
//! This item owns the S4 *prototype-recovery* leaf actions above.  The remaining
//! prototype-adjacent actions are explicitly **left for W7/W8**:
//!
//! * `ActionLikelyTrash`, `ActionRestructureVarnode`, `ActionMappedLocalSync`,
//!   `ActionMapGlobals` — local-variable / stack-frame restructuring
//!   (`coreaction.hh:848-901`), the S5 local-recovery plane.
//! * `ActionUnjustifiedParams`, `ActionInternalStorage`, the cast/typecast
//!   actions (`ActionSetCasts`, ...) — later type/prototype finalization.
//! * `ActionDeadCode`, `ActionConditionalConst`, `ActionSwitchNorm` — the
//!   dead-code / switch-normalization actions that also reach call-specs but
//!   belong to other stage items.
//!
//! W8 assembles `universalAction`; this file's leaf constructors plug into it
//! via [`ActionGroup::add_action`](crate::action::ActionGroup::add_action).
//!
//! # Seams (the `Funcdata` <-> call-spec/proto bridge is not in the merged tree)
//!
//! Every body in this file is gated on the **sub-function call-spec list**
//! (`Funcdata::qlst`, the C++ `vector<FuncCallSpecs *>`) and/or the function's
//! own recovered prototype (`Funcdata::funcp`) and output param-active
//! (`Funcdata::activeoutput`).  In the merged tree:
//!
//! * `Funcdata` has **no** `numCalls`/`getCallSpecs` accessors — the `qlst`
//!   field is seam-noted out (`funcdata.rs` struct docs: "`activeoutput`,
//!   ... `qlst` are seam-noted and omitted until their waves").
//! * `Funcdata::funcp` is the **placeholder** [`seams::FuncProto`](crate::seams)
//!   (an empty `struct FuncProto;`), *not* the real W6
//!   [`fspec::FuncProto`](crate::fspec) that the merged dependency added — the
//!   bridge that rewires `Funcdata` onto the real prototype object is a later
//!   wave and lives in `funcdata.rs`/`seams.rs`, which this item does not own.
//! * `Funcdata::getActiveOutput` / `initActiveOutput` / `clearActiveOutput`
//!   (the function-level output recovery) are likewise absent.
//!
//! The real [`FuncCallSpecs`](crate::fspec::FuncCallSpecs),
//! [`FuncProto`](crate::fspec::FuncProto), [`ProtoModel`](crate::fspec::ProtoModel),
//! [`ParamActive`](crate::fspec::ParamActive) types **do** exist in the merged
//! `fspec.rs`; what is missing is the `Funcdata` plumbing that hands them to an
//! action.  Following the established `coreaction_early` convention for an
//! action whose `apply` is a single call into an unrealized `Funcdata`
//! primitive, each body here:
//!
//! 1. transcribes the C++ `apply` structure verbatim **as commented pseudocode**
//!    (same iteration order, tie-breakers, and `count += 1` points), and
//! 2. routes the unrealized mutation through a `// SEAM(W7/W8-funcdata)` note
//!    and returns `0` changes.
//!
//! Each seam is reported in this item's `losses` so the owning wave can finish
//! the wiring by replaying the commented body against the real accessors.

use std::rc::Rc;

use kuna_num::opcodes::OpCode;

use crate::action::{ruleflags, Action, ActionBase, ActionContext, ActionGroupList, ApplyResult};
use crate::funcdata::Funcdata;

// =============================================================================
// ActionPrototypeTypes (coreaction.hh:658, coreaction.cc:4843)
// =============================================================================

/// Set up the data-types of input/output forced Varnodes (C++
/// `ActionPrototypeTypes`, `coreaction.hh:658`).
///
/// Builds forced input/output Varnodes and extends them as appropriate, sets
/// types on output forced Varnodes, and initializes the output recovery process.
pub struct ActionPrototypeTypes {
    base: ActionBase,
}

impl ActionPrototypeTypes {
    /// Construct in group `g` (C++ `ActionPrototypeTypes::ActionPrototypeTypes`).
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionPrototypeTypes {
            base: ActionBase::new(ruleflags::rule_onceperfunc, "prototypetypes", g),
        })
    }
}

impl Action for ActionPrototypeTypes {
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
        Some(Box::new(ActionPrototypeTypes { base: self.base.clone() }))
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ coreaction.cc:4843 — ActionPrototypeTypes::apply (the parts the
        // merged tree reaches: model selection, RETURN-in0 strip, output recovery
        // init).  The locked-input/output and truncated-space branches stay seamed
        // (no locked proto / no truncated stack space on the recovery path).

        // evalfp = getArch()->evalfp_current ?: getArch()->defaultfp;
        // if (!funcp.isModelLocked() && !funcp.hasMatchingModel(evalfp))
        //     funcp.setModel(evalfp);
        let evalfp = data.get_arch().eval_fp_current().cloned();
        if let Some(evalfp) = evalfp {
            if !data.get_func_proto().is_model_locked()
                && !data.get_func_proto().has_matching_model(&evalfp)
            {
                data.get_func_proto_mut().set_model(Some(evalfp));
            }
        }
        // funcp.hasThisPointer() -> prepareThisPointer(): SEAM(W4) — the default
        // models in the recovery path have no `this` pointer.

        // Strip the indirect register from all RETURN ops (so the compiler's
        // return-address mechanism does not appear in the high-level output):
        //   for op in RETURN ops: if (!getIn(0)->isConstant())
        //       opSetInput(op, newConstant(getIn(0)->getSize(), 0), 0);
        let return_ops: Vec<crate::seams::OpId> = data.obank().iter_code(OpCode::CPUI_RETURN).collect();
        for op in &return_ops {
            let in0 = match data.obank().get(*op).and_then(|o| o.get_in(0)) {
                Some(v) => v,
                None => continue,
            };
            let is_const = data.vbank().get(in0).map(|v| v.is_constant()).unwrap_or(false);
            if !is_const {
                let sz = data.vbank().get(in0).map(|v| v.get_size()).unwrap_or(1);
                let c = data.new_constant(sz, 0);
                let _ = data.op_set_input(*op, c, 0);
                self.base.count += 1;
            }
        }

        // if (funcp.isOutputLocked() && ...) { force the output varnode }
        // else data.initActiveOutput();  // begin gathering return values
        if data.get_func_proto().has_model() && data.get_func_proto().is_output_locked() {
            // SEAM(W6): the locked-output force-varnode branch (coreaction.cc:
            // 4871) needs the type system; the recovery path is the unlocked case.
        } else {
            data.init_active_output();
            self.base.count += 1;
        }

        // Truncated-space INT_ZEXT + locked-input force-varnode: SEAM(W4) — the
        // 8051/recovery path has no truncated stack space and no locked inputs.
        0
    }
}

// =============================================================================
// ActionDefaultParams (coreaction.hh:674, coreaction.cc:2369)
// =============================================================================

/// Find a prototype for each sub-function (C++ `ActionDefaultParams`,
/// `coreaction.hh:674`).
///
/// Loads prototype information for each sub-function if it exists, selects a
/// default otherwise, and injects `uponreturn` p-code where the model specifies.
pub struct ActionDefaultParams {
    base: ActionBase,
}

impl ActionDefaultParams {
    /// Construct in group `g` (C++ `ActionDefaultParams::ActionDefaultParams`).
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionDefaultParams {
            base: ActionBase::new(ruleflags::rule_onceperfunc, "defaultparams", g),
        })
    }
}

impl Action for ActionDefaultParams {
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
        Some(Box::new(ActionDefaultParams { base: self.base.clone() }))
    }
    fn apply(&mut self, _data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ coreaction.cc:2369 — ActionDefaultParams::apply
        //   evalfp = getArch()->evalfp_called ?: getArch()->defaultfp;
        //   for (i=0; i<data.numCalls(); ++i):
        //       fc = data.getCallSpecs(i);
        //       if (!fc->hasModel()):
        //           otherfunc = fc->getFuncdata();
        //           if (otherfunc != 0):
        //               fc->copy(otherfunc->getFuncProto());
        //               if (!fc->isModelLocked() && !fc->hasMatchingModel(evalfp))
        //                   fc->setModel(evalfp);
        //           else
        //               fc->setInternal(evalfp, getArch()->types->getTypeVoid());
        //       fc->insertPcode(data);   // inject any uponreturn p-code
        //   return 0;
        //
        // SEAM(W7/W8-funcdata): iterates `Funcdata::getCallSpecs(i)`
        // (the `qlst` vector) — absent from `Funcdata` in the merged tree.  The
        // `FuncCallSpecs::copy/setModel/setInternal/insertPcode` surface exists
        // in fspec.rs, but there is no per-function call-spec list to walk, so
        // this is a no-op until the bridge lands (count stays 0).
        0
    }
}

// =============================================================================
// ActionExtraPopSetup (coreaction.hh:691, coreaction.cc:1452)
// =============================================================================

/// Define the stack-pointer relationship before/after sub-function calls (C++
/// `ActionExtraPopSetup`, `coreaction.hh:691`).
///
/// Inserts a p-code relationship (`INT_ADD` if the *extrapop* is known,
/// `INDIRECT` otherwise) between the stack pointer entering and leaving each
/// sub-function call.
pub struct ActionExtraPopSetup {
    base: ActionBase,
    /// The stack space to analyze (C++ `AddrSpace *stackspace`); the space
    /// *index* in the architecture's manager, or `None` for the C++ null
    /// `(AddrSpace *)0` ("no stack to speak of").
    stackspace: Option<i32>,
}

impl ActionExtraPopSetup {
    /// Construct in group `g` with stack space `ss` (C++
    /// `ActionExtraPopSetup::ActionExtraPopSetup(g, ss)`).  `ss` is the stack
    /// space index, or `None` for the C++ null pointer.
    pub fn boxed(g: impl Into<String>, ss: Option<i32>) -> Box<dyn Action> {
        Box::new(ActionExtraPopSetup {
            base: ActionBase::new(ruleflags::rule_onceperfunc, "extrapopsetup", g),
            stackspace: ss,
        })
    }
}

impl Action for ActionExtraPopSetup {
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
        // C++ clone re-passes `stackspace` (coreaction.hh:697).
        Some(Box::new(ActionExtraPopSetup {
            base: self.base.clone(),
            stackspace: self.stackspace,
        }))
    }
    fn apply(&mut self, _data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ coreaction.cc:1452 — ActionExtraPopSetup::apply
        //   if (stackspace == (AddrSpace *)0) return 0;   // No stack to speak of
        if self.stackspace.is_none() {
            return 0;
        }
        // C++ (continued):
        //   point = stackspace->getSpacebase(0);
        //   sb_addr = Address(point.space, point.offset); sb_size = point.size;
        //   for (i=0; i<data.numCalls(); ++i):
        //       fc = data.getCallSpecs(i);
        //       if (fc->getExtraPop() == 0) continue;        // undisturbed
        //       op = data.newOp(2, fc->getOp()->getAddr());
        //       data.newVarnodeOut(sb_size, sb_addr, op);
        //       data.opSetInput(op, data.newVarnode(sb_size,sb_addr), 0);
        //       if (fc->getExtraPop() != ProtoModel::extrapop_unknown):
        //           fc->setEffectiveExtraPop(fc->getExtraPop());
        //           opSetOpcode(op, CPUI_INT_ADD);
        //           opSetInput(op, newConstant(sb_size, fc->getExtraPop()), 1);
        //           opInsertAfter(op, fc->getOp());
        //       else:
        //           opSetOpcode(op, CPUI_INDIRECT);
        //           opSetInput(op, newVarnodeIop(fc->getOp()), 1);
        //           opInsertBefore(op, fc->getOp());
        //   return 0;
        //
        // SEAM(W7/W8-funcdata): iterates `Funcdata::getCallSpecs(i)` (absent).
        // The non-null stack-space guard above is the realized portion; the
        // per-call INT_ADD/INDIRECT insertion needs the call-spec list and is
        // deferred (count stays 0).
        0
    }
}

// =============================================================================
// ActionFuncLink (coreaction.hh:707, coreaction.cc:1619)
// =============================================================================

/// Prepare for data-flow analysis of function parameters (C++
/// `ActionFuncLink`, `coreaction.hh:707`).
///
/// For each sub-function, inserts Varnodes matching known parameters (locked
/// prototypes) or prepares the parameter-recovery process (unknown prototypes),
/// and sets up output recovery.
pub struct ActionFuncLink {
    base: ActionBase,
}

impl ActionFuncLink {
    /// Construct in group `g` (C++ `ActionFuncLink::ActionFuncLink`).
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionFuncLink {
            base: ActionBase::new(ruleflags::rule_onceperfunc, "funclink", g),
        })
    }
}

impl Action for ActionFuncLink {
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
        Some(Box::new(ActionFuncLink { base: self.base.clone() }))
    }
    fn apply(&mut self, _data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ coreaction.cc:1619 — ActionFuncLink::apply
        //   size = data.numCalls();
        //   for (i=0; i<size; ++i):
        //       funcLinkInput(data.getCallSpecs(i), data);
        //       funcLinkOutput(data.getCallSpecs(i), data);
        //   return 0;
        //
        // funcLinkInput (coreaction.cc:1490): for a locked prototype, register
        //   parameter trials and insert stub Varnodes (stack-relative loads,
        //   JOIN/PIECE reassembly, or plain input Varnodes); for varargs or
        //   unlocked, initActiveInput(); create the stack placeholder if needed.
        // funcLinkOutput (coreaction.cc:1565): drop any override output Varnode
        //   (error on unique-space output), and for a locked output build the
        //   output Varnode (+ extension op) or delay it for a stack output; for
        //   an unlocked output, initActiveOutput().
        //
        // SEAM(W7/W8-funcdata): both helpers iterate `Funcdata::getCallSpecs(i)`
        // (absent) and mutate per-call `ParamActive`/output state through the
        // call-spec list.  Deferred (count stays 0).
        0
    }
}

// =============================================================================
// ActionFuncLinkOutOnly (coreaction.hh:728, coreaction.cc:1632)
// =============================================================================

/// Prepare for data-flow analysis when parameter recovery isn't required (C++
/// `ActionFuncLinkOutOnly`, `coreaction.hh:728`).
///
/// Runs only `ActionFuncLink::funcLinkOutput` per sub-function (sets up
/// potential outputs but not inputs), so local uses of output registers are not
/// mis-heritaged when the `protorecovery` group is disabled.
pub struct ActionFuncLinkOutOnly {
    base: ActionBase,
}

impl ActionFuncLinkOutOnly {
    /// Construct in group `g` (C++
    /// `ActionFuncLinkOutOnly::ActionFuncLinkOutOnly`).
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionFuncLinkOutOnly {
            base: ActionBase::new(ruleflags::rule_onceperfunc, "funclink_outonly", g),
        })
    }
}

impl Action for ActionFuncLinkOutOnly {
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
        Some(Box::new(ActionFuncLinkOutOnly { base: self.base.clone() }))
    }
    fn apply(&mut self, _data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ coreaction.cc:1632 — ActionFuncLinkOutOnly::apply
        //   size = data.numCalls();
        //   for (i=0; i<size; ++i)
        //       ActionFuncLink::funcLinkOutput(data.getCallSpecs(i), data);
        //   return 0;
        //
        // SEAM(W7/W8-funcdata): iterates `Funcdata::getCallSpecs(i)` (absent);
        // `funcLinkOutput` is the same surface seamed under `ActionFuncLink`.
        // Deferred (count stays 0).
        0
    }
}

// =============================================================================
// ActionParamDouble (coreaction.hh:745, coreaction.cc:1641)
// =============================================================================

/// Deal with situations that look like double-precision parameters (C++
/// `ActionParamDouble`, `coreaction.hh:745`).
///
/// Splits/joins `CONCAT`/`SUBPIECE` artifacts so that locked double-precision
/// parameters get their hi/lo pieces correctly labeled and grouped.
pub struct ActionParamDouble {
    base: ActionBase,
}

impl ActionParamDouble {
    /// Construct in group `g` (C++ `ActionParamDouble::ActionParamDouble`).
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionParamDouble { base: ActionBase::new(0, "paramdouble", g) })
    }
}

impl Action for ActionParamDouble {
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
        Some(Box::new(ActionParamDouble { base: self.base.clone() }))
    }
    fn apply(&mut self, _data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ coreaction.cc:1641 — ActionParamDouble::apply
        //   for (i=0; i<data.numCalls(); ++i):
        //       fc = data.getCallSpecs(i); op = fc->getOp();
        //       if (fc->isInputActive()):
        //           // walk active trials; for a checked, stack-relative,
        //           // PIECE-written trial, splitTrial + reorder op inputs by
        //           // endianness via fc->checkInputSplit; count += 1; j -= 1.
        //       else if (!fc->isInputLocked() && data.isDoublePrecisOn()):
        //           // scan adjacent op inputs for SplitVarnode hi/lo pairs;
        //           // fc->checkInputJoin -> opSetInput/opRemoveInput/
        //           // fc->doInputJoin; count += 1.
        //   // function-level: if (funcp.isInputLocked() && isDoublePrecisOn()):
        //   //   find locked primitive-whole params split into SUBPIECE hi/lo,
        //   //   mark piece Varnodes setPrecisLo/setPrecisHi; count += 1 each.
        //
        // SEAM(W7/W8-funcdata): the per-call arms iterate
        // `Funcdata::getCallSpecs(i)` (absent); the function-level arm reads
        // `Funcdata::funcp` (the empty `seams::FuncProto` placeholder, not the
        // real `fspec::FuncProto`).  Deferred (count stays 0).  `isDoublePrecisOn`
        // IS realized on `Funcdata` but is only a guard for the seamed work.
        0
    }
}

// =============================================================================
// ActionActiveParam (coreaction.hh:763, coreaction.cc:1769)
// =============================================================================

/// Determine active parameters to sub-functions (C++ `ActionActiveParam`,
/// `coreaction.hh:763`).
///
/// The final stage of parameter recovery for sub-functions without an explicit
/// prototype: decides which Heritage-collected input Varnodes are actually used
/// as parameters, then resolves the model and builds the input map.
pub struct ActionActiveParam {
    base: ActionBase,
}

impl ActionActiveParam {
    /// Construct in group `g` (C++ `ActionActiveParam::ActionActiveParam`).
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionActiveParam { base: ActionBase::new(0, "activeparam", g) })
    }
}

impl Action for ActionActiveParam {
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
        Some(Box::new(ActionActiveParam { base: self.base.clone() }))
    }
    fn apply(&mut self, _data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ coreaction.cc:1769 — ActionActiveParam::apply
        //   AliasChecker aliascheck; aliascheck.gather(&data, getStackSpace(), true);
        //   for (i=0; i<data.numCalls(); ++i):
        //       fc = data.getCallSpecs(i);
        //       try {
        //         if (fc->isInputActive()):
        //             activeinput = fc->getActiveInput();
        //             trimmable = numPasses>0 || op->code()!=CPUI_CALLIND;
        //             if (!activeinput->isFullyChecked())
        //                 fc->checkInputTrialUse(data, aliascheck);
        //             activeinput->finishPass();
        //             if (numPasses > maxPass) markFullyChecked();
        //             else count += 1;          // still work to do
        //             if (trimmable && isFullyChecked()):
        //                 if (needsFinalCheck()) fc->finalInputCheck();
        //                 fc->resolveModel(activeinput);
        //                 fc->deriveInputMap(activeinput);
        //                 fc->buildInputFromTrials(data);
        //                 fc->clearActiveInput();
        //                 count += 1;
        //       } catch (LowlevelError &err) { rethrow with call name/seqnum }
        //
        // SEAM(W7/W8-funcdata): iterates `Funcdata::getCallSpecs(i)` (absent)
        // and drives `FuncCallSpecs`-level trial/model resolution per call.
        // The `AliasChecker` gather over the stack space is part of the same
        // seamed surface.  Deferred (count stays 0).
        0
    }
}

// =============================================================================
// ActionActiveReturn (coreaction.hh:776, coreaction.cc:1817)
// =============================================================================

/// Determine which sub-functions have active output Varnodes (C++
/// `ActionActiveReturn`, `coreaction.hh:776`).
///
/// The return-value analogue of [`ActionActiveParam`]: derives the output map
/// for each sub-function with an active output and builds the output Varnodes.
pub struct ActionActiveReturn {
    base: ActionBase,
}

impl ActionActiveReturn {
    /// Construct in group `g` (C++ `ActionActiveReturn::ActionActiveReturn`).
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionActiveReturn { base: ActionBase::new(0, "activereturn", g) })
    }
}

impl Action for ActionActiveReturn {
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
        Some(Box::new(ActionActiveReturn { base: self.base.clone() }))
    }
    fn apply(&mut self, _data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ coreaction.cc:1817 — ActionActiveReturn::apply
        //   for (i=0; i<data.numCalls(); ++i):
        //       fc = data.getCallSpecs(i);
        //       if (fc->isOutputActive()):
        //           activeoutput = fc->getActiveOutput();
        //           vector<Varnode *> trialvn;
        //           fc->checkOutputTrialUse(data, trialvn);
        //           fc->deriveOutputMap(activeoutput);
        //           fc->buildOutputFromTrials(data, trialvn);
        //           fc->clearActiveOutput();
        //           count += 1;
        //   return 0;
        //
        // SEAM(W7/W8-funcdata): iterates `Funcdata::getCallSpecs(i)` (absent).
        // Deferred (count stays 0).
        0
    }
}

// =============================================================================
// ActionReturnRecovery (coreaction.hh:811, coreaction.cc:1954)
// =============================================================================

/// Determine the data-flow holding the function's return value (C++
/// `ActionReturnRecovery`, `coreaction.hh:811`).
///
/// Gathers the active output trials at each `CPUI_RETURN`, runs ancestor-realism
/// analysis, and (once fully checked) rewrites the `RETURN` ops to carry the
/// recovered return value (via `buildReturnOutput`).
pub struct ActionReturnRecovery {
    base: ActionBase,
}

impl ActionReturnRecovery {
    /// Construct in group `g` (C++
    /// `ActionReturnRecovery::ActionReturnRecovery`).
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionReturnRecovery { base: ActionBase::new(0, "returnrecovery", g) })
    }

    /// Rewrite a CPUI_RETURN op to reflect the recovered output parameter (C++
    /// `ActionReturnRecovery::buildReturnOutput`, coreaction.cc:1880).
    ///
    /// Appends the used output-trial Varnodes (in proper order) as a second (and
    /// further) input to the RETURN, concatenating multiple pieces via PIECE/JOIN
    /// when needed.  `in0` (the stripped return-indirect reference) is kept first.
    fn build_return_output(
        active: &crate::fspec::ParamActive,
        retop: crate::seams::OpId,
        data: &mut Funcdata,
        return_single: bool,
    ) {
        use kuna_num::pcoderaw::VarnodeData;
        let _ = VarnodeData::default;
        // newparam = [ retop->getIn(0) ] + used trial varnodes (in order).
        let mut newparam: Vec<crate::seams::VarnodeId> = Vec::new();
        if let Some(in0) = data.obank().get(retop).and_then(|o| o.get_in(0)) {
            newparam.push(in0);
        }
        let num_input = data.obank().get(retop).map(|o| o.num_input()).unwrap_or(0);
        for i in 0..active.get_num_trials() {
            let trial = active.get_trial(i);
            if !trial.is_used() {
                break;
            }
            if trial.get_slot() >= num_input {
                break;
            }
            if let Some(vn) = data.obank().get(retop).and_then(|o| o.get_in(trial.get_slot())) {
                newparam.push(vn);
            }
        }
        // (kuna) GH-6990: keep only the first return register (return_single).
        if crate::kuna_returnpair::keep_single_return(return_single, newparam.len()) {
            newparam.truncate(2);
        }
        // Easy zero/one return varnode case (coreaction.cc:1894).  This is the
        // register-output recovery path (a single recovered return register,
        // e.g. 8051 ACC).
        if newparam.len() <= 2 {
            let _ = data.op_set_all_input(retop, &newparam);
            return;
        }
        // Multi-piece concatenation (coreaction.cc:1896-1951): two-piece PIECE
        // via a JOIN address, or the many-piece container concat.  Both require
        // `getArch()->translate` for `constructJoinAddress` (the JOIN-space
        // register-name lookup), which the merged `ArchHandle` seam does not
        // carry (the engine `Translate` is not shared onto the IR handle).  The
        // default models on the recovery path return a single register (no
        // multi-piece output), so this branch is not reached on the live path;
        // it is left as a SEAM(W4 translate-on-handle) and the RETURN keeps the
        // first recovered piece rather than fabricating a malformed concat.
        newparam.truncate(2);
        let _ = data.op_set_all_input(retop, &newparam);
    }
}

impl Action for ActionReturnRecovery {
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
        Some(Box::new(ActionReturnRecovery { base: self.base.clone() }))
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ coreaction.cc:1954 — ActionReturnRecovery::apply.  Gather the active
        // output trials at each RETURN, run ancestor-realism, and (once fully
        // checked) rewrite the RETURN ops to carry the recovered return value.
        let mut active = match data.take_active_output() {
            Some(a) => a,
            None => return 0,
        };
        let maxancestor = data.get_arch().trim_recurse_max;
        let return_ops: Vec<crate::seams::OpId> =
            data.obank().iter_code(OpCode::CPUI_RETURN).collect();
        for &op in &return_ops {
            let o = match data.obank().get(op) {
                Some(o) => o,
                None => continue,
            };
            if o.is_dead() || o.get_halt_type() != 0 {
                continue;
            }
            for i in 0..active.get_num_trials() {
                if active.get_trial(i).is_checked() {
                    continue;
                }
                let slot = active.get_trial(i).get_slot();
                let vn = match data.obank().get(op).and_then(|o| o.get_in(slot)) {
                    Some(v) => v,
                    None => {
                        self.base.count += 1;
                        continue;
                    }
                };
                // ancestorReal.execute(op,slot,&trial,false) &&
                //   data.ancestorOpUse(maxancestor,vn,op,trial,0,0)
                let mut ancestor = crate::funcdata_varnode::AncestorRealistic::new();
                let (trial_size, trial_cond) =
                    (active.get_trial(i).get_size(), active.get_trial(i).has_cond_exe_effect());
                let (realistic, solid) =
                    ancestor.execute(data, op, slot, trial_size, trial_cond, false);
                ancestor.apply_trial(active.get_trial_mut(i), realistic, solid);
                if realistic || solid {
                    // The trial's data-flow ancestry is realistic; now test that
                    // the Varnode is only used at this op (ancestorOpUse).
                    let only = {
                        let trial = active.get_trial_mut(i);
                        data.ancestor_op_use(maxancestor, vn, op, trial, 0, 0)
                    };
                    if only {
                        active.get_trial_mut(i).mark_active();
                    }
                }
                self.base.count += 1;
            }
        }

        active.finish_pass();
        if active.get_num_passes() > active.get_max_pass() {
            active.mark_fully_checked();
        }

        if active.is_fully_checked() {
            let manager_rc = data.get_arch().manage.clone();
            let _ = data.get_func_proto().derive_output_map(&mut active, &manager_rc);
            let return_single = data.get_arch().return_single;
            for &op in &return_ops {
                let o = match data.obank().get(op) {
                    Some(o) => o,
                    None => continue,
                };
                if o.is_dead() || o.get_halt_type() != 0 {
                    continue;
                }
                Self::build_return_output(&active, op, data, return_single);
            }
            data.clear_active_output();
            self.base.count += 1;
        } else {
            data.restore_active_output(active);
        }
        0
    }
}

// =============================================================================
// ActionRestrictLocal (coreaction.hh:826, coreaction.cc:2003)
// =============================================================================

/// Restrict the possible range of local variables (C++ `ActionRestrictLocal`,
/// `coreaction.hh:826`).
///
/// Marks parameter storage of locked sub-function calls and unaffected
/// save-register storage as *not mapped*, so they cannot be treated as locals.
pub struct ActionRestrictLocal {
    base: ActionBase,
}

impl ActionRestrictLocal {
    /// Construct in group `g` (C++ `ActionRestrictLocal::ActionRestrictLocal`).
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionRestrictLocal { base: ActionBase::new(0, "restrictlocal", g) })
    }
}

impl Action for ActionRestrictLocal {
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
        Some(Box::new(ActionRestrictLocal { base: self.base.clone() }))
    }
    fn apply(&mut self, _data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ coreaction.cc:2003 — ActionRestrictLocal::apply
        //   for (i=0; i<data.numCalls(); ++i):
        //       fc = data.getCallSpecs(i); op = fc->getOp();
        //       if (!fc->isInputLocked()) continue;
        //       if (fc->getSpacebaseOffset() == offset_unknown) continue;
        //       for each locked param:
        //           if (IPTR_SPACEBASE): markNotMapped(space,
        //               wrapOffset(spacebaseOffset+addr.offset), size, true);
        //           else if (IPTR_JOIN): for each spacebase piece, markNotMapped.
        //   for eiter in funcp.effectBegin()..effectEnd():
        //       if (getType() == killedbycall) continue;       // not saved
        //       vn = data.findVarnodeInput(size, addr);
        //       if (vn && vn->isUnaffected()):
        //           for op in vn->beginDescend()..endDescend():
        //               if (op->code()!=CPUI_COPY) continue;
        //               outvn = op->getOut();
        //               if (!getScopeLocal()->isUnaffectedStorage(outvn)) continue;
        //               getScopeLocal()->markNotMapped(outvn space/off/size, false);
        //   return 0;
        //
        // SEAM(W7/W8-funcdata): the first loop iterates
        // `Funcdata::getCallSpecs(i)` (absent); the second reads
        // `Funcdata::funcp` (empty `seams::FuncProto`) effect records and the
        // local `Scope::markNotMapped` surface (the `Scope` is a placeholder in
        // the merged tree).  Deferred (count stays 0).
        0
    }
}

// =============================================================================
// ActionInputPrototype (coreaction.hh:907, coreaction.cc:4941)
// =============================================================================

/// Calculate the prototype for the function (C++ `ActionInputPrototype`,
/// `coreaction.hh:907`).
///
/// If the input prototype wasn't originally known, analyzes the discovered input
/// Varnodes against the prototype model to derive parameters and create any
/// unreferenced input Varnodes.
pub struct ActionInputPrototype {
    base: ActionBase,
}

impl ActionInputPrototype {
    /// Construct in group `g` (C++ `ActionInputPrototype::ActionInputPrototype`).
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionInputPrototype {
            base: ActionBase::new(ruleflags::rule_onceperfunc, "inputprototype", g),
        })
    }
}

impl Action for ActionInputPrototype {
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
        Some(Box::new(ActionInputPrototype { base: self.base.clone() }))
    }
    fn apply(&mut self, _data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ coreaction.cc:4941 — ActionInputPrototype::apply
        //   ParamActive active(false);
        //   data.getScopeLocal()->clearCategory(Symbol::fake_input);
        //   data.getFuncProto().clearUnlockedInput();
        //   if (!funcp.isInputLocked()):
        //       for vn in beginDef(Varnode::input)..endDef:
        //           if (funcp.possibleInputParam(vn->getAddr(),vn->getSize())):
        //               slot = active.getNumTrials();
        //               active.registerTrial(vn->getAddr(), vn->getSize());
        //               if (!vn->hasNoDescend()) active.getTrial(slot).markActive();
        //               triallist.push_back(vn);
        //       funcp.resolveModel(&active);
        //       funcp.deriveInputMap(&active);
        //       // create unreferenced input varnodes (or markNoUse on intersect)
        //       if (data.isHighOn()) funcp.updateInputTypes(data,triallist,&active);
        //       else                 funcp.updateInputNoTypes(data,triallist,&active);
        //   data.clearDeadVarnodes();
        //   return 0;
        //
        // SEAM(W7/W8-funcdata): reads/mutates `Funcdata::funcp` (the empty
        // `seams::FuncProto` placeholder, not the real `fspec::FuncProto`) and
        // the local `Scope`.  The input-Varnode def-set walk and
        // `clearDeadVarnodes` are part of the same seamed surface.  Deferred
        // (count stays 0).
        0
    }
}

// =============================================================================
// ActionOutputPrototype (coreaction.hh:918, coreaction.cc:4999)
// =============================================================================

/// Set the recovered output data-type as a formal part of the prototype (C++
/// `ActionOutputPrototype`, `coreaction.hh:918`).
pub struct ActionOutputPrototype {
    base: ActionBase,
}

impl ActionOutputPrototype {
    /// Construct in group `g` (C++
    /// `ActionOutputPrototype::ActionOutputPrototype`).
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionOutputPrototype {
            base: ActionBase::new(ruleflags::rule_onceperfunc, "outputprototype", g),
        })
    }
}

impl Action for ActionOutputPrototype {
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
        Some(Box::new(ActionOutputPrototype { base: self.base.clone() }))
    }
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ coreaction.cc:4999 — ActionOutputPrototype::apply
        //   outparam = data.getFuncProto().getOutput();
        //   if (!outparam->isTypeLocked() || outparam->isSizeTypeLocked()):
        //       op = data.getFirstReturnOp();
        //       vector<Varnode *> vnlist;
        //       if (op != 0): for (i=1; i<op->numInput(); ++i)
        //                         vnlist.push_back(op->getIn(i));
        //       if (data.isHighOn()) funcp.updateOutputTypes(vnlist);
        //       else                 funcp.updateOutputNoTypes(vnlist, getArch()->types);
        //   return 0;
        //
        // The real `fspec::FuncProto` is now on `Funcdata` (proto-recovery wave).
        // Where the W4 ScopeLocal would attach a `ProtoStoreSymbol`, the merged
        // tree attaches a stand-alone `ProtoStoreInternal` (the C++ no-scope
        // store) so the recovered output storage/type can be set.  We transcribe
        // the `updateOutputTypes` body for the single (high-on) return trial: the
        // output addr+type come from the first return value's HighVariable type.
        //
        // The HighVariable type itself is the W8 `ActionInferTypes` surface; until
        // that lands the return value's high type is the un-recovered base
        // (size-correct, metatype UNKNOWN), so the OUTPUT STORAGE recovers exactly
        // (the addr + size the merge needs for addrtied), but the TYPE NAME renders
        // the W8 default — the single documented residual to full boolless parity.
        // The `TypeFactory` (`glb->types`, `getTypeVoid`) is the W6 surface and
        // the seams `Architecture` does not expose it; the formal void type is the
        // size-0 `TYPE_VOID` base (its name renders "void", `dtype.rs:277`), which
        // is the same interned datatype `getTypeVoid` returns.
        let void_type = Rc::new(crate::dtype::Datatype::new(0, crate::dtype::type_metatype::TYPE_VOID));
        data.get_func_proto_mut().attach_internal_store(void_type);
        // C++ guard: proceed only if the output is not type-locked, or is merely
        // size-type-locked.  The freshly-attached internal store seeds an unlocked
        // void output, so this is satisfied (the locked-output arm is the W4
        // explicit-prototype path, absent here).
        {
            let outparam = data.get_func_proto().get_output();
            if outparam.is_type_locked() && !outparam.is_size_type_locked() {
                return 0;
            }
        }
        let retop = match data.get_first_return_op() {
            Some(op) => op,
            None => return 0,
        };
        // vnlist = retop inputs [1..]; the first is the trial output.
        let trial0 = {
            let o = data.obank().get(retop).expect("outputprototype: stale return op");
            if o.num_input() < 2 {
                None
            } else {
                o.get_in(1)
            }
        };
        let trial0 = match trial0 {
            Some(vn) => vn,
            None => return 0, // empty trial list: leave output void
        };
        let out_addr = data.vbank().get(trial0).expect("outputprototype: stale trial").get_addr().clone();
        // pieces.type = triallist[0]->getHigh()->getType()  (high-on path).
        let out_type = data
            .high_get_type(trial0)
            .unwrap_or_else(|| Rc::new(crate::dtype::Datatype::new(1, crate::dtype::type_metatype::TYPE_UNKNOWN)));
        let pieces = crate::fspec::ParameterPieces { addr: out_addr, type_: Some(out_type), flags: 0 };
        data.get_func_proto_mut().set_output(&pieces);
        0
    }
}

// =============================================================================
// ActionPrototypeWarnings (coreaction.hh:1060, coreaction.cc:5140)
// =============================================================================

/// Emit warnings about the function and sub-function prototypes (C++
/// `ActionPrototypeWarnings`, `coreaction.hh:1060`).
///
/// Generates override messages and headers for input/output errors, unknown
/// calling conventions, and per-call parameter/return-location problems.
pub struct ActionPrototypeWarnings {
    base: ActionBase,
}

impl ActionPrototypeWarnings {
    /// Construct in group `g` (C++
    /// `ActionPrototypeWarnings::ActionPrototypeWarnings`).
    pub fn boxed(g: impl Into<String>) -> Box<dyn Action> {
        Box::new(ActionPrototypeWarnings {
            base: ActionBase::new(ruleflags::rule_onceperfunc, "prototypewarnings", g),
        })
    }
}

impl Action for ActionPrototypeWarnings {
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
        Some(Box::new(ActionPrototypeWarnings { base: self.base.clone() }))
    }
    fn apply(&mut self, _data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ coreaction.cc:5140 — ActionPrototypeWarnings::apply
        //   data.getOverride().generateOverrideMessages(msgs, getArch());
        //   for (m in msgs) data.warningHeader(m);
        //   ourproto = data.getFuncProto();
        //   if (ourproto.hasInputErrors())
        //       data.warningHeader("Cannot assign parameter locations ...");
        //   if (ourproto.hasOutputErrors())
        //       data.warningHeader("Cannot assign location of return value ...");
        //   if (ourproto.isModelUnknown()):
        //       s = "Unknown calling convention";
        //       if (printModelInDecl()) s += ": " + getModelName();
        //       if (!hasCustomStorage() && (isInputLocked()||isOutputLocked()))
        //           s += " -- yet parameter storage is locked";
        //       data.warningHeader(s);
        //   for (i=0; i<data.numCalls(); ++i):
        //       fc = data.getCallSpecs(i); fd = fc->getFuncdata();
        //       if (fc->hasInputErrors()) data.warning("Cannot assign parameter
        //           location for function <name|<indirect>>: ...", entryAddr);
        //       if (fc->hasOutputErrors()) data.warning("Cannot assign location
        //           of return value for function <...>: ...", entryAddr);
        //
        // SEAM(W7/W8-funcdata): the override-message generation reads
        // `Funcdata::getOverride()` (the local override store is not on
        // `Funcdata` in the merged tree); the function-level headers read
        // `Funcdata::funcp` (the empty `seams::FuncProto` placeholder, not the
        // real `fspec::FuncProto` with `hasInputErrors`/`isModelUnknown`/...);
        // and the per-call loop iterates `Funcdata::getCallSpecs(i)` (absent).
        // The warning channel (`ActionContext::warnings`) IS realized, but with
        // the placeholder proto there are no errors to report.  Deferred
        // (count stays 0); no warning is emitted.
        0
    }
}

// =============================================================================
// Item action set (C++ definition order) for the W8 universalAction assembler
// =============================================================================

/// The S4 prototype-recovery leaf actions owned by this item, in C++
/// definition order, each constructed in group `g`.
///
/// `ActionExtraPopSetup` is **not** included here: its constructor takes a stack
/// `AddrSpace` (the architecture's stack space index), which the W8 assembler
/// must supply at build time — construct it directly with
/// [`ActionExtraPopSetup::boxed`].
pub fn proto_actions(g: &str) -> Vec<Box<dyn Action>> {
    vec![
        ActionPrototypeTypes::boxed(g),
        ActionDefaultParams::boxed(g),
        ActionFuncLink::boxed(g),
        ActionFuncLinkOutOnly::boxed(g),
        ActionParamDouble::boxed(g),
        ActionActiveParam::boxed(g),
        ActionActiveReturn::boxed(g),
        ActionReturnRecovery::boxed(g),
        ActionRestrictLocal::boxed(g),
        ActionInputPrototype::boxed(g),
        ActionOutputPrototype::boxed(g),
        ActionPrototypeWarnings::boxed(g),
    ]
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use kuna_base::address::Address;
    use kuna_base::space::{
        addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
    };
    use kuna_base::types::int4;

    use super::*;
    use crate::action::ruleflags;
    use crate::seams::Architecture;

    // Mirrors the coreaction_early.rs test harness (funcdata_block fixtures).
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

    /// Build a `(name, flags)` pair from a boxed action's base.
    fn name_flags(a: &dyn Action) -> (String, u32) {
        (a.get_name().to_string(), a.base().flags)
    }

    #[test]
    fn names_and_groups_match_cpp() {
        // Exact name() strings and group from the C++ constructors.
        let cases: Vec<(Box<dyn Action>, &str)> = vec![
            (ActionPrototypeTypes::boxed("g0"), "prototypetypes"),
            (ActionDefaultParams::boxed("g0"), "defaultparams"),
            (ActionFuncLink::boxed("g0"), "funclink"),
            (ActionFuncLinkOutOnly::boxed("g0"), "funclink_outonly"),
            (ActionParamDouble::boxed("g0"), "paramdouble"),
            (ActionActiveParam::boxed("g0"), "activeparam"),
            (ActionActiveReturn::boxed("g0"), "activereturn"),
            (ActionReturnRecovery::boxed("g0"), "returnrecovery"),
            (ActionRestrictLocal::boxed("g0"), "restrictlocal"),
            (ActionInputPrototype::boxed("g0"), "inputprototype"),
            (ActionOutputPrototype::boxed("g0"), "outputprototype"),
            (ActionPrototypeWarnings::boxed("g0"), "prototypewarnings"),
        ];
        for (act, expect) in &cases {
            assert_eq!(act.get_name(), *expect);
            assert_eq!(act.get_group(), "g0");
        }
        // ActionExtraPopSetup is constructed with a stack space argument.
        let ep = ActionExtraPopSetup::boxed("g0", Some(3));
        assert_eq!(ep.get_name(), "extrapopsetup");
        assert_eq!(ep.get_group(), "g0");
    }

    #[test]
    fn flags_match_cpp_constructors() {
        // rule_onceperfunc actions.
        for a in [
            ActionPrototypeTypes::boxed("g"),
            ActionDefaultParams::boxed("g"),
            ActionFuncLink::boxed("g"),
            ActionFuncLinkOutOnly::boxed("g"),
            ActionInputPrototype::boxed("g"),
            ActionOutputPrototype::boxed("g"),
            ActionPrototypeWarnings::boxed("g"),
            ActionExtraPopSetup::boxed("g", None),
        ] {
            assert_eq!(name_flags(&*a).1, ruleflags::rule_onceperfunc);
        }
        // flags == 0 actions.
        for a in [
            ActionParamDouble::boxed("g"),
            ActionActiveParam::boxed("g"),
            ActionActiveReturn::boxed("g"),
            ActionReturnRecovery::boxed("g"),
            ActionRestrictLocal::boxed("g"),
        ] {
            assert_eq!(name_flags(&*a).1, 0);
        }
    }

    #[test]
    fn clone_filtered_respects_grouplist() {
        let gl = ActionGroupList::from_names(["protorecovery"]);
        // In-group clone succeeds and preserves name/group.
        let a = ActionFuncLink::boxed("protorecovery");
        let c = a.clone_filtered(&gl).expect("in-group clone");
        assert_eq!(c.get_name(), "funclink");
        assert_eq!(c.get_group(), "protorecovery");
        // Out-of-group clone is filtered out (C++ returns null).
        let b = ActionFuncLink::boxed("notenabled");
        assert!(b.clone_filtered(&gl).is_none());
    }

    #[test]
    fn extrapop_clone_carries_stackspace() {
        let gl = ActionGroupList::from_names(["protorecovery"]);
        let a = ActionExtraPopSetup::boxed("protorecovery", Some(7));
        let c = a.clone_filtered(&gl).expect("in-group clone");
        assert_eq!(c.get_name(), "extrapopsetup");
        // The clone is still an ExtraPopSetup with a non-null stack space, so a
        // second clone must also succeed in-group (structural round-trip).
        let c2 = c.clone_filtered(&gl).expect("re-clone");
        assert_eq!(c2.get_name(), "extrapopsetup");
    }

    #[test]
    fn proto_actions_enumerates_in_cpp_order() {
        let acts = proto_actions("g");
        let names: Vec<&str> = acts.iter().map(|a| a.get_name()).collect();
        assert_eq!(
            names,
            vec![
                "prototypetypes",
                "defaultparams",
                "funclink",
                "funclink_outonly",
                "paramdouble",
                "activeparam",
                "activereturn",
                "returnrecovery",
                "restrictlocal",
                "inputprototype",
                "outputprototype",
                "prototypewarnings",
            ]
        );
    }

    #[test]
    fn extrapop_null_stackspace_applies_no_change() {
        // C++ first line: `if (stackspace == (AddrSpace *)0) return 0;`
        // This is the one realized control-path; verify it returns 0 changes.
        let mut act = ActionExtraPopSetup {
            base: ActionBase::new(ruleflags::rule_onceperfunc, "extrapopsetup", "g"),
            stackspace: None,
        };
        let mut data = build_fd();
        let mut ctx = ActionContext::new();
        let res: int4 = act.apply(&mut data, &mut ctx);
        assert_eq!(res, 0);
        assert_eq!(act.base().count, 0);
    }
}
