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
//! # Boundaries (the `Funcdata` <-> call-spec/proto bridge is not in the merged tree)
//!
//! Every body in this file is gated on the **sub-function call-spec list**
//! (`Funcdata::qlst`, the C++ `vector<FuncCallSpecs *>`) and/or the function's
//! own recovered prototype (`Funcdata::funcp`) and output param-active
//! (`Funcdata::activeoutput`).  In the merged tree:
//!
//! * `Funcdata` has **no** `numCalls`/`getCallSpecs` accessors — the `qlst`
//!   field is boundary-noted out (`funcdata.rs` struct docs: "`activeoutput`,
//!   ... `qlst` are boundary-noted and omitted until their waves").
//! * `Funcdata::funcp` is the **placeholder** [`context::FuncProto`](crate::context)
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
//! 2. routes the unrealized mutation through a `// STUB(W7/W8-funcdata)` note
//!    and returns `0` changes.
//!
//! Each boundary is reported in this item's `losses` so the owning wave can finish
//! the wiring by replaying the commented body against the real accessors.

use std::rc::Rc;

use kuna_base::types::{int4, uintb};

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
        // init).  The locked-input/output and truncated-space branches stay stubbed
        // (no locked proto / no truncated stack space on the recovery path).

        let evalfp = data.get_arch().eval_fp_current().cloned();
        if let Some(evalfp) = evalfp {
            if !data.get_func_proto().is_model_locked()
                && !data.get_func_proto().has_matching_model(&evalfp)
            {
                data.get_func_proto_mut().set_model(Some(evalfp));
            }
        }
        // (kuna) Establish the local-variable discovery window from the (now-set)
        // proto model's stack ranges.  C++ runs `localmap->resetLocalWindow()` at
        // `Funcdata` construction (via `funcp.setScope`, funcdata.cc:70), where the
        // default proto model is already attached; the merged tree defers the model
        // assignment to this action (the model only becomes available here), so the
        // window — `ScopeLocal`'s range tree, which `restructureVarnode`'s
        // `MapState::addRange` gates every stack hint against (varmap.cc:1409) — is
        // populated at the equivalent point in the schedule, before the first
        // `ActionRestructureVarnode`.  Without it the scope range tree is empty and
        // every gathered stack RangeHint is dropped, so no stack local is recovered.
        //
        // The sized-stack-Varnode typing boundary this used to be gated on is now
        // CLOSED: `ScopeLocal::restructureVarnode` clears the unlocked auto-recovered
        // stack Symbols at the head of every pass (`clearUnlockedCategory(-1)`,
        // funcdata_spacebase.rs / varmap.cc:1259), so the first-pass open-array hint
        // formed before `RuleStoreVarnode` folds the STORE into a sized stack COPY no
        // longer survives to compete with the scalar `Fixed int4` hint the converted
        // Varnode supplies on the next pass.  A scalar stack local now types as
        // `int4` (NOT a spurious `xunknown1 [N]` array — verified on condconst_conn)
        // and is named `vN` (`resolve_default_name`, coreaction.cc:3087).
        //
        // The downstream boundary that USED to hold this env-gated — the addr-tied
        // return-register COPY collapse — is now CLOSED too.  After typing,
        // condconst_conn is `v2(stack) = x; ...; v1(eax) = COPY(v2); return v1;`;
        // the C++ oracle emits the single `v1 = x; ... return v1;` because the eax
        // return-register COPY is IMPLIED, not merged: the eax register is written
        // by a single return-value COPY, so it is never a whole-function local and
        // C++ (`database.cc:1155` / `syncVarnodesWithSymbols`) leaves it un-tied.
        // `mark_output_storage_addr_tied` (coreaction_cleanup.rs) replicates that
        // structural rule — a single-COPY return register stays un-tied,
        // `baseExplicit` marks it IMPLIED, and the printer collapses the round-trip
        // to `return v2`.  With the collapse in place the window-reset is a strict
        // win, so it now runs UNCONDITIONALLY (matching C++ `funcp.setScope`'s
        // `resetLocalWindow` at funcdata.cc:70).
        data.reset_local_window();
        // funcp.hasThisPointer() -> prepareThisPointer(): STUB(W4) — the default
        // models in the recovery path have no `this` pointer.

        // Strip the indirect register from all RETURN ops (so the compiler's
        // return-address mechanism does not appear in the high-level output).
        let return_ops: Vec<crate::context::OpId> = data.obank().iter_code(OpCode::CPUI_RETURN).collect();
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

        if data.get_func_proto().is_output_locked() {
            let (out_size, out_addr, out_type) = {
                let outparam = data.get_func_proto().get_output();
                (
                    outparam.get_size(),
                    outparam.get_address(),
                    outparam.get_type().cloned(),
                )
            };
            let is_void = out_type
                .as_ref()
                .map(|t| t.get_metatype() == crate::dtype::type_metatype::TYPE_VOID)
                .unwrap_or(true);
            if !is_void {
                let out_type = out_type.expect("non-void output has a type");
                for op in &return_ops {
                    let halt = data
                        .obank()
                        .get(*op)
                        .map(|o| o.is_dead() || o.get_halt_type() != 0)
                        .unwrap_or(true);
                    if halt {
                        continue;
                    }
                    let numin = data.obank().get(*op).map(|o| o.num_input()).unwrap_or(0);
                    let vn = data.new_varnode(out_size, &out_addr, None);
                    let _ = data.op_insert_input(*op, vn, numin);
                    data.vbank_mut()
                        .get_mut(vn)
                        .expect("prototypetypes: stale forced output")
                        .update_type_locked(Rc::clone(&out_type), true, true);
                    self.base.count += 1;
                }
            }
        } else {
            data.init_active_output();
            self.base.count += 1;
        }

        // Truncated-space INT_ZEXT setup: STUB(W4) — the recovery path has no
        // truncated stack space (only the 8051-family default code space is).

        // Force locked inputs to exist as Varnodes.  Needed so a big locked input
        // exists even when only part is used (SUBPIECE can then be built off it).
        if data.get_func_proto().is_input_locked() {
            // ptr_size: the recovery path's default code space is never truncated,
            // so the C++ pointer-trim (spc->isTruncated()) does not fire.
            let topbl = if data.bblocks_get_size() > 0 {
                Some(data.bblocks_get_block(0))
            } else {
                None
            };
            let numparams = data.get_func_proto().num_params();
            for i in 0..numparams {
                let (psize, paddr, ptype) = {
                    let param = match data.get_func_proto().get_param(i) {
                        Some(p) => p,
                        None => continue,
                    };
                    (param.get_size(), param.get_address(), param.get_type().cloned())
                };
                // Varnode *vn = data.newVarnode(size,addr); vn = setInputVarnode(vn);
                let vn = data.new_varnode(psize, &paddr, None);
                let vn = match data.set_input_varnode(vn) {
                    Ok(v) => v,
                    Err(_) => continue,
                };
                {
                    let v = data
                        .vbank_mut()
                        .get_mut(vn)
                        .expect("prototypetypes: stale locked input");
                    v.set_locked_input();
                    // C++ binds the locked input Varnode's type via the W4 ScopeLocal
                    // parameter Symbol (read back by `ActionInferTypes::buildLocaltypes`'s
                    // type-locked SymbolEntry `getExactPiece` seed — a documented W4/W8
                    // boundary, LOSS-138).  That symbol-scope binding is not on the merged
                    // tree, so the param's declared type is set + locked on the forced
                    // input Varnode directly here: the same end state (a type-locked
                    // input Varnode carrying the parameter type), which `getLocalType`'s
                    // `isTypeLock()` arm then reads and `ActionInferTypes` propagates
                    // FROM (the type-plane seed for type-heavy functions).
                    if let Some(ptype) = ptype.as_ref() {
                        if ptype.get_metatype() != crate::dtype::type_metatype::TYPE_UNKNOWN {
                            v.update_type_locked(Rc::clone(ptype), true, true);
                        }
                    }
                }
                // extendInput(data, vn, param, topbl): build any assumed extension.
                if let (Some(topbl), Some(ptype)) = (topbl, ptype) {
                    extend_input(data, vn, &paddr, psize, &ptype, topbl);
                }
                self.base.count += 1;
            }
        }
        0
    }
}

/// Build an extension P-code op for a forced/locked input Varnode, if the
/// prototype model assumes one (C++ `ActionPrototypeTypes::extendInput`,
/// coreaction.cc:4824).
///
/// `assumedInputExtension` reports `COPY` (no extension), `PIECE` (extend per
/// the parameter type's metatype: INT → INT_SEXT, else INT_ZEXT), or a concrete
/// `INT_SEXT`/`INT_ZEXT`.  When an extension is wanted, a new op is inserted at
/// the top block writing the full-size container from `invn`.
fn extend_input(
    data: &mut Funcdata,
    invn: crate::context::VarnodeId,
    in_addr: &kuna_base::address::Address,
    in_size: int4,
    param_type: &Rc<crate::dtype::Datatype>,
    topbl: crate::context::BlockId,
) {
    use kuna_num::pcoderaw::VarnodeData;
    let mut vdata = VarnodeData::default();
    let mut res = data
        .get_func_proto()
        .assumed_input_extension(in_addr, in_size, &mut vdata);
    if res == OpCode::CPUI_COPY {
        return; // no extension
    }
    if res == OpCode::CPUI_PIECE {
        // Extend based on the parameter's metatype.
        res = if param_type.get_metatype() == crate::dtype::type_metatype::TYPE_INT {
            OpCode::CPUI_INT_SEXT
        } else {
            OpCode::CPUI_INT_ZEXT
        };
    }
    let ext_addr = vdata.get_addr();
    let ext_size = vdata.size as int4;
    let start = data.bblocks_block_start(topbl);
    let op = data.new_op(1, start);
    let _ = data.new_varnode_out(ext_size, &ext_addr, op);
    data.op_set_opcode_code(op, res);
    let _ = data.op_set_input(op, invn, 0);
    data.op_insert_begin(op, topbl);
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
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ coreaction.cc:2369 — ActionDefaultParams::apply: give every
        // sub-function call a prototype model (the callee's if known, else the
        // evaluation/default model).
        //   evalfp = getArch()->evalfp_called ?: getArch()->defaultfp;
        let evalfp = match data.get_arch().eval_fp_called().cloned() {
            Some(m) => m,
            // No default model registered (hand-built fixture): nothing to set.
            None => return 0,
        };
        let void_ty = match data.get_arch().types().map(|t| t.get_type_void()) {
            Some(Ok(t)) => t,
            _ => return 0,
        };
        // Clone the arch handle (Rc bump) so the callee-proto query/build below can
        // borrow the architecture's declared-callee map / type factory / space
        // manager while `data`'s call specs are mutated.
        let arch = data.get_arch().clone();
        let default_fp = arch.default_fp().cloned();
        let size = data.num_calls();
        for i in 0..size {
            if !data.get_call_specs(i).proto().has_model() {
                // C++ `coreaction.cc:2382-2390`.
                // The callee's locked `FuncProto` is re-built here from the source-
                // declared prototype pieces parked on its global FunctionSymbol by
                // `Architecture::set_function_prototype_pieces` (the kuna stand-in for
                // the C++ callee `Funcdata`'s lazily-built `FuncProto`).
                let has_funcdata = data.get_call_specs(i).has_funcdata();
                let (callee_pieces, callee_model) = if has_funcdata {
                    let entry = data.get_call_specs(i).get_entry_address().clone();
                    // (kuna, Phase 3) A host-declared prototype model rides the
                    // locked pieces (ghidra-mode `<prototype model=…>`); `None`
                    // on the standalone path, keeping the default-model seed.
                    (
                        arch.callee_proto_pieces(&entry),
                        arch.callee_proto_model(&entry),
                    )
                } else {
                    (None, None)
                };
                // (kuna) An *output-only* callee pieces (no declared inputs / void
                // outtype, but a custom locked output) is what the console
                // `map return <addr>` parks: in C++ `map return` locks only the
                // callee's output, leaving its inputs to be recovered by the model.
                // Detect that here so the input recovery stays default-model-driven
                // (set_internal) while the custom locked output is applied verbatim
                // — distinct from a full `parse line` prototype, which input-locks.
                let custom_output_only = match &callee_pieces {
                    Some(pieces) => {
                        let out_only = pieces.intypes.is_empty()
                            && pieces.outtype.is_none()
                            && pieces.output_storage.is_some();
                        if out_only { pieces.output_storage.clone() } else { None }
                    }
                    None => None,
                };
                let callee_proto = if custom_output_only.is_some() {
                    None
                } else {
                    match (callee_pieces, default_fp.clone(), arch.types()) {
                        (Some(pieces), Some(dfp), Some(types)) => {
                            let mut fp = crate::fspec::FuncProto::new();
                            // The host-declared model wins over defaultfp when
                            // present (Phase 3); standalone always defaultfp.
                            let seed_model = callee_model.unwrap_or(dfp);
                            match fp.seed_locked_from_pieces(
                                &pieces,
                                seed_model,
                                void_ty.clone(),
                                types,
                                arch.manage(),
                            ) {
                                Ok(()) => Some(fp),
                                // The callee storage assignment hit an un-ported boundary: fall
                                // back to the default-model recovery for this call site.
                                Err(_) => None,
                            }
                        }
                        _ => None,
                    }
                };
                match callee_proto {
                    Some(calleeproto) => {
                        data.get_call_specs_mut(i).proto_mut().copy(&calleeproto);
                        let fc = data.get_call_specs(i);
                        if !fc.proto().is_model_locked() && !fc.proto().has_matching_model(&evalfp) {
                            data.get_call_specs_mut(i).proto_mut().set_model(Some(evalfp.clone()));
                        }
                    }
                    None => {
                        // No source-declared callee prototype (a symbol with no
                        // Funcdata): set the default-model internal proto.  The
                        // register-parameter datatests resolve here.
                        data.get_call_specs_mut(i).proto_mut().set_internal(evalfp.clone(), void_ty.clone());
                        // (kuna) A console `map return <addr>`-only callee: keep the
                        // model-driven input recovery just established, but lock the
                        // custom return storage on top (the C++ `map return` locks
                        // only the output).  This sets the typed output Varnode at
                        // the (possibly stack-relative) return address and the output
                        // lock, so `ActionFuncLink::funcLinkOutput` flags
                        // `setStackOutputLock` and `Heritage::tryOutputStackGuard`
                        // materializes the caller-perspective output.
                        if let Some(out_piece) = custom_output_only.as_ref() {
                            let fc = data.get_call_specs_mut(i);
                            fc.proto_mut().set_output(out_piece);
                            fc.proto_mut().set_output_lock(true);
                        }
                    }
                }
            }
            // fc->insertPcode(data): inject any uponreturn p-code.  STUB(W4
            // pcodeinjectlib): the default models on the datatest path declare no
            // uponreturn injection, so this is a no-op here.
        }
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
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        use crate::fspec::EXTRAPOP_UNKNOWN;
        use kuna_base::address::Address;

        // C++ coreaction.cc:1452-1482 — ActionExtraPopSetup::apply, transcribed
        // verbatim.  L0 of the RSP keystone: insert the per-call spacebase
        // relationship (INT_ADD if the extrapop is known, INDIRECT otherwise) so
        // the stack-pointer flow across each sub-function call is modeled.

        let stackspace_index = match self.stackspace {
            Some(i) => i,
            None => return 0,
        };
        let stackspace = match data.get_arch().manage().get_space(stackspace_index) {
            Some(s) => Rc::clone(s),
            None => return 0,
        };
        let point = match stackspace.get_spacebase(0) {
            Ok(p) => p,
            Err(_) => return 0,
        };
        let point_space = match point.space.clone() {
            Some(s) => s,
            None => return 0,
        };
        let sb_addr = Address::new(point_space, point.offset);
        let sb_size = point.size as int4;

        for i in 0..data.num_calls() {
            let extrapop = data.get_call_specs(i).get_extra_pop();
            if extrapop == 0 {
                continue;
            }
            let fc_op = data.get_call_specs(i).get_op();
            let fc_op_addr = match data.obank().get(fc_op) {
                Some(o) => o.get_addr().clone(),
                None => continue,
            };
            let op = data.new_op(2, fc_op_addr);
            if data.new_varnode_out(sb_size, &sb_addr, op).is_err() {
                continue;
            }
            let in0 = data.new_varnode(sb_size, &sb_addr, None);
            let _ = data.op_set_input(op, in0, 0);
            if extrapop != EXTRAPOP_UNKNOWN {
                // We know exactly how stack pointer is changed.
                data.get_call_specs_mut(i).set_effective_extra_pop(extrapop);
                data.op_set_opcode_code(op, OpCode::CPUI_INT_ADD);
                // C++ widens `int4 fc->getExtraPop()` to the `uintb` parameter,
                // i.e. signed widening (sign-extend through i64); `bare as` is the
                // faithful reproduction of that conversion.
                let in1 = data.new_constant(sb_size, extrapop as i64 as uintb);
                let _ = data.op_set_input(op, in1, 1);
                //   data.opInsertAfter(op,fc->getOp());
                data.op_insert_after(op, fc_op);
            } else {
                // We don't know exactly, so we create INDIRECT.
                //   data.opSetOpcode(op,CPUI_INDIRECT);
                data.op_set_opcode_code(op, OpCode::CPUI_INDIRECT);
                //   data.opSetInput(op,data.newVarnodeIop(fc->getOp()),1);
                let in1 = data.new_varnode_iop(fc_op);
                let _ = data.op_set_input(op, in1, 1);
                //   data.opInsertBefore(op,fc->getOp());
                data.op_insert_before(op, fc_op);
            }
        }
        // return 0;
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

    /// Set up the parameter analysis for a single sub-function call (C++
    /// `ActionFuncLink::funcLinkInput`, `coreaction.cc:1490`).
    ///
    /// For an unlocked or varargs prototype, turn on active-input recovery.  For a
    /// locked prototype, register a trial per declared parameter and insert a stub
    /// input Varnode (register params).  The stack-relative (`opStackLoad`),
    /// JOIN-reassembly, and spacebase-placeholder branches are W4 (`opStackLoad` /
    /// `findJoin` are not on the W3 Funcdata) — recorded as a loss; they are not
    /// reached by the register-parameter call-rendering datatests.
    fn func_link_input(idx: int4, data: &mut Funcdata) {
        use kuna_base::address::Address;
        let inputlocked = data.get_call_specs(idx).proto().is_input_locked();
        let varargs = data.get_call_specs(idx).is_dotdotdot();
        // AddrSpace *spacebase = fc->getSpacebase(); // non-zero => stackplaceholder
        let mut spacebase = data.get_call_specs(idx).proto().get_spacebase().cloned();

        if !inputlocked || varargs {
            // NEXT-LOCUS (Local cross #2, BLOCKED): the undeclared callee `retval`
            // takes the unlocked active-input recovery path here.  rust keeps a live
            // RDI input on the `call fretval` (`call fretval(RDI)`), where C++ prunes
            // it to a zero-arg `call fretval()`.  The divergence is upstream of
            // rendering: at the prior `call fothercall` rust materialises an INDIRECT
            // `RDI = RDI [](free)` (a killedbycall indirect-creation) that keeps RDI
            // live across the call, so the active-input trial for `retval`'s RDI is
            // never killed; C++'s `killedbycall` effect drops RDI so no live trial
            // exists.  rust also recovers an 8-byte RAX return (`xunknown8 v1` +
            // `SUB84`/`(int4)` truncation) where C++ recovers the 4-byte EAX
            // (`int4 v1`).  Both stem from the call-effect / return-storage trial
            // scoring: FIX STUB is the `killedbycall` indirect-creation suppression
            // for `othercall` (so RDI is not regenerated) plus the return-trial
            // size pruning (RAX vs EAX) in the active-param recovery
            // (`ActionActiveParam`/`checkInputTrialUse` + `ParamActive` return
            // scoring) — the W4 killedbycall cluster (cf. `rport/w10-killedbycall`).
            data.get_call_specs_mut(idx).init_active_input();
        }
        // Locked-prototype branch (coreaction.cc:1500-1554): register a trial and
        // insert a stub input Varnode per declared parameter.  The stack-relative
        // (`opStackLoad`) and JOIN-reassembly arms are W4 boundaries (skipped below);
        // the plain register-parameter insertion is transcribed.
        if inputlocked {
            let op = data.get_call_specs(idx).get_op();
            let numparam = data.get_call_specs(idx).proto().num_params();
            // bool setplaceholder = varargs;
            let mut setplaceholder = varargs;
            for i in 0..numparam {
                let (paddr, psize) = {
                    let fc = data.get_call_specs(idx);
                    let p = fc.proto().get_param(i).expect("funcLinkInput: param index");
                    (p.get_address().clone(), p.get_size())
                };
                data.get_call_specs_mut(idx).get_active_input().register_trial(&paddr, psize);
                data.get_call_specs_mut(idx).get_active_input().get_trial_mut(i).mark_active();
                if varargs {
                    data.get_call_specs_mut(idx)
                        .get_active_input()
                        .get_trial_mut(i)
                        .set_fixed_position(i);
                }
                let spc = match paddr.get_space() {
                    Some(s) => s.clone(),
                    None => continue,
                };
                let off = paddr.get_offset();
                if spc.get_type() == kuna_base::space::spacetype::IPTR_SPACEBASE {
                    // Param is stack relative: build the LOAD and insert it, marking
                    // the first as the spacebase placeholder (so the explicit tail
                    // createPlaceholder is unnecessary for a locked stack param).
                    let loadval = match data.op_stack_load(
                        &spc,
                        off,
                        psize as kuna_base::types::uint4,
                        op,
                        None,
                        false,
                    ) {
                        Ok(v) => v,
                        Err(_) => continue,
                    };
                    let nin = data.obank().get(op).map(|o| o.num_input()).unwrap_or(0);
                    let _ = data.op_insert_input(op, loadval, nin);
                    if !setplaceholder {
                        setplaceholder = true;
                        if let Some(v) = data.vbank_mut().get_mut(loadval) {
                            v.set_spacebase_placeholder();
                        }
                        // With a locked stack parameter we don't need a placeholder.
                        spacebase = None;
                    }
                    continue;
                }
                if spc.get_type() == kuna_base::space::spacetype::IPTR_JOIN {
                    // JOIN-split locked parameter (coreaction.cc:1524-1550): a
                    // struct-by-value argument whose storage joins a stack piece and
                    // a register piece.  When one end of the join is a stack
                    // (IPTR_SPACEBASE) piece, materialize the stack half with a LOAD,
                    // the register remainder with a plain Varnode, concatenate them
                    // via CPUI_PIECE into a fresh UNIQUE output, and pass that as the
                    // argument.  (A join with no stack piece falls through to the
                    // plain insert below.)
                    let manager = data.get_arch().manage.clone();
                    if let Ok(join) = manager.find_join(off) {
                        let npieces = join.num_pieces();
                        // index = which end of the join is the stack piece (0 or last)
                        let mut index: i32 = -1;
                        if join.get_piece(0).space.as_ref().map(|s| s.get_type())
                            == Some(kuna_base::space::spacetype::IPTR_SPACEBASE)
                        {
                            index = 0;
                        } else if join
                            .get_piece(npieces - 1)
                            .space
                            .as_ref()
                            .map(|s| s.get_type())
                            == Some(kuna_base::space::spacetype::IPTR_SPACEBASE)
                        {
                            index = npieces - 1;
                        }
                        if index >= 0 {
                            // const VarnodeData &stack(join->getPiece(index));
                            let stack = join.get_piece(index).clone();
                            // const VarnodeData &remain(stripJoinPiece(join, index));
                            if let Ok(remain) = manager.strip_join_piece(&join, index) {
                                let stack_space = match stack.space.clone() {
                                    Some(s) => s,
                                    None => continue,
                                };
                                let remain_space = match remain.space.clone() {
                                    Some(s) => s,
                                    None => continue,
                                };
                                // loadval = opStackLoad(stack.space,stack.offset,stack.size,op,0,false)
                                let loadval = match data.op_stack_load(
                                    &stack_space,
                                    stack.offset,
                                    stack.size,
                                    op,
                                    None,
                                    false,
                                ) {
                                    Ok(v) => v,
                                    Err(_) => continue,
                                };
                                // remainval = newVarnode(remain.size, remain.space, remain.offset)
                                let remain_addr =
                                    Address::new(remain_space, remain.offset);
                                let remainval =
                                    data.new_varnode(remain.size as int4, &remain_addr, None);
                                // concatOp = newOp(2,op->getAddr()); opSetOpcode(CPUI_PIECE)
                                let op_addr = data
                                    .obank()
                                    .get(op)
                                    .expect("funcLinkInput: stale call op")
                                    .get_addr()
                                    .clone();
                                let concat_op = data.new_op(2, op_addr);
                                data.op_set_opcode_code(
                                    concat_op,
                                    kuna_num::opcodes::OpCode::CPUI_PIECE,
                                );
                                // index==0 ? (load,remain) : (remain,load)
                                if index == 0 {
                                    let _ = data.op_set_input(concat_op, loadval, 0);
                                    let _ = data.op_set_input(concat_op, remainval, 1);
                                } else {
                                    let _ = data.op_set_input(concat_op, remainval, 0);
                                    let _ = data.op_set_input(concat_op, loadval, 1);
                                }
                                // outvn = newUniqueOut(sz, concatOp)
                                let outvn = match data.new_unique_out(psize, concat_op) {
                                    Ok(v) => v,
                                    Err(_) => continue,
                                };
                                // opInsertBefore(concatOp, op)
                                data.op_insert_before(concat_op, op);
                                // opInsertInput(op, outvn, op->numInput())
                                let nin =
                                    data.obank().get(op).map(|o| o.num_input()).unwrap_or(0);
                                let _ = data.op_insert_input(op, outvn, nin);
                                continue;
                            }
                        }
                    }
                }
                // Plain register parameter: insert a fresh input Varnode at the end.
                let pvn = data.new_varnode(psize, &paddr, None);
                let nin = data.obank().get(op).map(|o| o.num_input()).unwrap_or(0);
                let _ = data.op_insert_input(op, pvn, nin);
            }
        }
        if let Some(sb) = spacebase {
            // create_placeholder needs `&mut FuncCallSpecs` + `&mut Funcdata`;
            // splice the spec out and put it back at the same index (no cross-call
            // lookup happens in funcLinkInput, so the index-stable take is safe).
            let mut fc = data.replace_call_specs(idx);
            let _ = fc.create_placeholder(data, &sb);
            data.restore_call_specs_at(idx, fc);
        }
    }

    /// Set up the return-value recovery for a single sub-function call (C++
    /// `ActionFuncLink::funcLinkOutput`, `coreaction.cc:1565`).
    ///
    /// Drop any override output Varnode; for a locked non-void output build the
    /// output (+ extension op); for an unlocked output, turn on active-output
    /// recovery.  The locked-output build (stack output lock / extension op) is
    /// the type-system path — the common datatest path is the unlocked branch.
    fn func_link_output(idx: int4, data: &mut Funcdata) {
        let callop = data.get_call_specs(idx).get_op();
        // CALL ops are expected to have no output; an override may have produced
        // one — remove it (the IPTR_INTERNAL error case is the override-unique boundary).
        if data.obank().get(callop).and_then(|o| o.get_out()).is_some() {
            data.op_unset_output(callop);
        }
        if data.get_call_specs(idx).proto().is_output_locked() {
            // C++ coreaction.cc:1582-1613 — locked-output build.  Materialize the
            // typed output Varnode at the locked output address, and (when the
            // prototype model assumes the small output is sign/zero/piece-extended
            // to a full register) insert the post-call extension op.
            use kuna_num::pcoderaw::VarnodeData;
            let outparam = data.get_call_specs(idx).proto().get_output();
            // A locked output always carries a type, but guard defensively (None is the C++ null).
            let outtype = match outparam.get_type() {
                Some(t) => Rc::clone(t),
                None => return,
            };
            let metatype = outtype.get_metatype();
            if metatype != crate::dtype::type_metatype::TYPE_VOID {
                let sz = outparam.get_size();
                let addr = outparam.get_address();
                if metatype == crate::dtype::type_metatype::TYPE_BOOL && data.is_type_recovery_on()
                {
                    data.op_mark_calculated_bool(callop);
                }
                // Stack-relative output: defer the Varnode until stack heritage.
                let is_spacebase = addr
                    .get_space()
                    .map(|s| s.get_type() == kuna_base::space::spacetype::IPTR_SPACEBASE)
                    .unwrap_or(false);
                if is_spacebase {
                    data.get_call_specs_mut(idx).set_stack_output_lock(true);
                    return;
                }
                let _ = data.new_varnode_out(sz, &addr, callop);
                let mut vdata = VarnodeData::default();
                let mut res = data
                    .get_call_specs(idx)
                    .proto()
                    .assumed_output_extension(&addr, sz, &mut vdata);
                if res == OpCode::CPUI_PIECE {
                    // Pick an extension based on type.
                    res = if metatype == crate::dtype::type_metatype::TYPE_INT {
                        OpCode::CPUI_INT_SEXT
                    } else {
                        OpCode::CPUI_INT_ZEXT
                    };
                }
                if res != OpCode::CPUI_COPY {
                    // The (small-size) output is extended to a full register;
                    // create the extension op immediately after the call.
                    let pc = data.obank().get(callop).map(|o| o.get_addr().clone()).unwrap();
                    let op = data.new_op(1, pc);
                    let ext_addr = vdata.get_addr();
                    let ext_size = vdata.size as int4;
                    let _ = data.new_varnode_out(ext_size, &ext_addr, op);
                    let invn = data.new_varnode(sz, &addr, None);
                    let _ = data.op_set_input(op, invn, 0);
                    data.op_set_opcode_code(op, res);
                    data.op_insert_after(op, callop);
                }
            }
        } else {
            // C++ `fc->initActiveOutput()` begins gathering the call's return value.
            // The killed-by-call output range is now guarded by an INDIRECT *creation*
            // marker in `Heritage::guardCalls`, whose output Varnode the downstream
            // ActionActiveReturn (collectOutputTrialVarnodes/buildOutputFromTrials)
            // promotes to the CALL's output — so the recovered return register flows
            // to `w0 = call ...` instead of leaving a free read.
            data.get_call_specs_mut(idx).init_active_output();
        }
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
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ coreaction.cc:1619 — ActionFuncLink::apply: per sub-function, set up
        // input + output recovery.
        let size = data.num_calls();
        for i in 0..size {
            ActionFuncLink::func_link_input(i, data);
            ActionFuncLink::func_link_output(i, data);
        }
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
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ coreaction.cc:1632 — ActionFuncLinkOutOnly::apply: only the output
        // recovery per sub-function (the `protorecovery` group is disabled, so
        // inputs are not gathered).
        let size = data.num_calls();
        for i in 0..size {
            ActionFuncLink::func_link_output(i, data);
        }
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
        // C++ coreaction.cc:1641 — ActionParamDouble::apply. Over each call spec:
        //   - when fc->isInputActive(): walk active trials; for a checked,
        //     stack-relative, PIECE-written trial, splitTrial + reorder op inputs
        //     by endianness via fc->checkInputSplit; count += 1; j -= 1.
        //   - else when !fc->isInputLocked() && data.isDoublePrecisOn(): scan
        //     adjacent op inputs for SplitVarnode hi/lo pairs; fc->checkInputJoin
        //     -> opSetInput/opRemoveInput/fc->doInputJoin; count += 1.
        //   Function-level: when funcp.isInputLocked() && isDoublePrecisOn(), find
        //   locked primitive-whole params split into SUBPIECE hi/lo, mark piece
        //   Varnodes setPrecisLo/setPrecisHi; count += 1 each.
        //
        // STUB(W7/W8-funcdata): the per-call arms iterate
        // `Funcdata::getCallSpecs(i)` (absent); the function-level arm reads
        // `Funcdata::funcp` (the empty `context::FuncProto` placeholder, not the
        // real `fspec::FuncProto`).  Deferred (count stays 0).  `isDoublePrecisOn`
        // IS realized on `Funcdata` but is only a guard for the stubbed work.
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
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ coreaction.cc:1769 — ActionActiveParam::apply: per sub-function, run
        // input-parameter recovery over its active trials.
        use crate::funcdata_callsite::{
            build_input_from_trials, check_input_trial_use, final_input_check,
        };
        // aliascheck.gather(&data, getStackSpace(), true): deferred local-alias
        // checker for the spacebase-parameter branch.
        let mut aliascheck = data.build_alias_checker_deferred();
        let manager_rc = data.get_arch().manage.clone();
        // (kuna) varargstackargs
        let vararg_stack_args = data.get_arch().vararg_stack_args;
        // (kuna) `calleearityfwd`: call sites that finalize with an empty argument
        // list, retried at the end of the pass against the siblings that finalize
        // after them.  See [`crate::p4_calls::kuna_calleearityfwd`].
        let mut pending_rescue = Vec::new();
        // (kuna) `calleearitylive`: call sites that finalize with a SHORT argument
        // list, retried the same way.  See
        // [`crate::p4_calls::kuna_calleearitylive`].
        let mut pending_extend = Vec::new();

        // INDEX-BASED (CORRECTION-7 #3): keep the call specs ON `data.qlst` so
        // each sub-function's input-trial ancestor walk can look up the *other*
        // calls' specs (`checkCallDoubleUse` -> `getCallSpecs`).  The C++ holds a
        // `FuncCallSpecs *` aliasing into `qlst` and mutates `data` through it; in
        // Rust we re-borrow `data.get_call_specs[_mut](idx)` between `&mut data`
        // ops.  The take/restore is used ONLY for the finalize tail
        // (`final_input_check`/`build_input_from_trials`), which performs no
        // cross-call lookup and needs a single `&mut FuncCallSpecs`.
        for idx in 0..data.num_calls() {
            if !data.get_call_specs(idx).is_input_active() {
                continue;
            }
            let op = data.get_call_specs(idx).get_op();
            // A CALL op destroyed by block/deadcode removal (or whose slot was
            // reused by a non-CALL op) leaves a dangling call spec until
            // `deleteCallSpecs` prunes it; guard against touching it.
            let op_ok = data
                .obank()
                .get(op)
                .map(|o| {
                    !o.is_dead()
                        && matches!(o.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND)
                })
                .unwrap_or(false);
            if !op_ok {
                continue;
            }
            // trimmable = numPasses>0 || op->code() != CPUI_CALLIND.
            let is_callind =
                data.obank().get(op).map(|o| o.code() == OpCode::CPUI_CALLIND).unwrap_or(false);
            let trimmable =
                data.get_call_specs_mut(idx).get_active_input().get_num_passes() > 0 || !is_callind;

            if !data.get_call_specs_mut(idx).get_active_input().is_fully_checked() {
                if let Some(ac) = aliascheck.as_mut() {
                    check_input_trial_use(idx, data, ac);
                }
            }
            data.get_call_specs_mut(idx).get_active_input().finish_pass();
            let (passes, maxpass) = {
                let ai = data.get_call_specs_mut(idx).get_active_input();
                (ai.get_num_passes(), ai.get_max_pass())
            };
            if passes > maxpass {
                data.get_call_specs_mut(idx).get_active_input().mark_fully_checked();
            } else {
                self.base.count += 1; // still have work to do
            }
            if trimmable && data.get_call_specs_mut(idx).get_active_input().is_fully_checked() {
                // Finalize: single-spec take/restore (no cross-call lookup needed).
                // Splice this spec out so `final_input_check`/`build_input_from_trials`
                // can hold `&mut FuncCallSpecs` and `&mut Funcdata` at once, then
                // put it back at the same index so the qlst stays index-stable for
                // the remaining iterations.
                let mut fc = data.replace_call_specs(idx);
                if fc.get_active_input().needs_final_check() {
                    final_input_check(&mut fc, data);
                }
                // (kuna) `varargstackargs`: tell `fillinMap` that this call's
                // variable arguments live on the stack, so the empty register
                // slots before them are the ABI's doing and not evidence that
                // the recovery has run past the argument list.
                let vararg_split = vararg_stack_args && fc.is_dotdotdot();
                fc.get_active_input().set_vararg_stack_split(vararg_split);
                // resolveModel(activeinput) + deriveInputMap(activeinput): resolve
                // the model and fill in the trial → parameter map.
                let _ = fc.resolve_and_derive_input_map(&manager_rc);
                let fixup = build_input_from_trials(&mut fc, data);
                if let Some(p) = fixup.rescue {
                    pending_rescue.push(p);
                }
                if let Some(p) = fixup.extend {
                    pending_extend.push(p);
                }
                fc.clear_active_input();
                data.restore_call_specs_at(idx, fc);
                self.base.count += 1;
            }
        }
        // (kuna) `calleearityfwd`: every spec in this pass is final now, so the
        // sites that recovered nothing get their one retry.
        crate::p4_calls::kuna_calleearityfwd::rescue_pending(data, &pending_rescue);
        // (kuna) `calleearitylive`: and the sites that recovered too FEW get theirs.
        crate::p4_calls::kuna_calleearitylive::extend_pending(data, &pending_extend);
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
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ coreaction.cc:1817 — ActionActiveReturn::apply: per sub-function with
        // an active output, recover the return value Varnode.
        use crate::funcdata_callsite::{build_output_from_trials, check_output_trial_use};
        let manager_rc = data.get_arch().manage.clone();
        let mut qlst = data.take_call_specs();
        for fc in qlst.iter_mut() {
            if !fc.is_output_active() {
                continue;
            }
            // Skip a call spec whose op was destroyed or whose slot was reused by a
            // non-CALL op (deleteCallSpecs prune analogue — see ActionActiveParam).
            let op_ok = data
                .obank()
                .get(fc.get_op())
                .map(|o| {
                    !o.is_dead()
                        && matches!(o.code(), OpCode::CPUI_CALL | OpCode::CPUI_CALLIND)
                })
                .unwrap_or(false);
            if !op_ok {
                continue;
            }
            let trialvn = check_output_trial_use(fc, data);
            let _ = fc.derive_output_map_self(&manager_rc);
            build_output_from_trials(fc, data, &trialvn);
            fc.clear_active_output();
            self.base.count += 1;
        }
        data.restore_call_specs(qlst);
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
        retop: crate::context::OpId,
        data: &mut Funcdata,
        return_single: bool,
    ) {
        use kuna_num::pcoderaw::VarnodeData;
        let _ = VarnodeData::default;
        // newparam = [ retop->getIn(0) ] + used trial varnodes (in order).
        let mut newparam: Vec<crate::context::VarnodeId> = Vec::new();
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
        if newparam.len() == 3 {
            // Two-piece concatenation (coreaction.cc:1896-1913): a return value
            // recovered as two contiguous register pieces (e.g. an xmm0 float8
            // whose low/high 4-byte lanes split during heritage refinement).
            // Build PIECE(hi,lo) at the JOIN/parent-register address so the
            // RETURN reads one whole varnode.
            let lovn = newparam[1];
            let hivn = newparam[2];
            let (lo_addr, lo_size) =
                (active.get_trial(0).get_address().clone(), active.get_trial(0).get_size());
            let (hi_addr, hi_size) =
                (active.get_trial(1).get_address().clone(), active.get_trial(1).get_size());
            let manage = data.get_arch().manage.clone();
            let join = manage.register_lookup().and_then(|rl| {
                manage
                    .construct_join_address(rl.as_ref(), &hi_addr, hi_size, &lo_addr, lo_size)
                    .ok()
            });
            match join {
                Some(joinaddr) => {
                    let retaddr = data.obank().get(retop).map(|o| o.get_addr().clone());
                    if let Some(retaddr) = retaddr {
                        let newop = data.new_op(2, retaddr);
                        data.op_set_opcode_code(newop, OpCode::CPUI_PIECE);
                        if let Ok(newwhole) =
                            data.new_varnode_out(hi_size + lo_size, &joinaddr, newop)
                        {
                            // Don't let the new whole cause additional heritage.
                            if let Some(v) = data.vbank_mut().get_mut(newwhole) {
                                v.set_write_mask();
                            }
                            data.op_insert_before(newop, retop);
                            newparam.pop();
                            let last = newparam.len() - 1;
                            newparam[last] = newwhole;
                            let _ = data.op_set_all_input(retop, &newparam);
                            let _ = data.op_set_input(newop, hivn, 0); // most-sig
                            let _ = data.op_set_input(newop, lovn, 1); // least-sig
                            return;
                        }
                    }
                }
                None => {}
            }
            // Fall back to the first piece if the join could not be constructed.
            newparam.truncate(2);
            let _ = data.op_set_all_input(retop, &newparam);
            return;
        }
        // Many-piece single-container concatenation (coreaction.cc:1915-1951):
        // walk the contiguous used trials, building a PIECE chain at the earliest
        // address.  Not reached by the default x86-64 register models (which
        // recover at most two pieces); kept faithful for completeness.
        let mut chained: Vec<crate::context::VarnodeId> = Vec::new();
        if let Some(in0) = data.obank().get(retop).and_then(|o| o.get_in(0)) {
            chained.push(in0);
        }
        let mut offmatch: int4 = 0;
        let mut preexist: Option<crate::context::VarnodeId> = None;
        let nin = data.obank().get(retop).map(|o| o.num_input()).unwrap_or(0);
        for i in 0..active.get_num_trials() {
            let (used, slot, toff, tsize) = {
                let t = active.get_trial(i);
                (t.is_used(), t.get_slot(), t.get_offset(), t.get_size())
            };
            if !used || slot >= nin {
                break;
            }
            let vn = match data.obank().get(retop).and_then(|o| o.get_in(slot)) {
                Some(v) => v,
                None => break,
            };
            match preexist {
                None => {
                    preexist = Some(vn);
                    offmatch = toff + tsize;
                }
                Some(pre) if offmatch == toff => {
                    offmatch += tsize;
                    let (pre_addr, pre_size) = {
                        let v = data.vbank().get(pre).expect("buildReturnOutput: pre");
                        (v.get_addr().clone(), v.get_size())
                    };
                    let vn_addr =
                        data.vbank().get(vn).expect("buildReturnOutput: vn").get_addr().clone();
                    let addr = if vn_addr.cmp(&pre_addr) == std::cmp::Ordering::Less {
                        vn_addr
                    } else {
                        pre_addr
                    };
                    let retaddr = data.obank().get(retop).map(|o| o.get_addr().clone());
                    if let Some(retaddr) = retaddr {
                        let newop = data.new_op(2, retaddr);
                        data.op_set_opcode_code(newop, OpCode::CPUI_PIECE);
                        if let Ok(newout) =
                            data.new_varnode_out(pre_size + tsize, &addr, newop)
                        {
                            if let Some(v) = data.vbank_mut().get_mut(newout) {
                                v.set_write_mask();
                            }
                            let _ = data.op_set_input(newop, vn, 0); // most-sig
                            let _ = data.op_set_input(newop, pre, 1);
                            data.op_insert_before(newop, retop);
                            preexist = Some(newout);
                        }
                    }
                }
                _ => break,
            }
        }
        let mut finalparam: Vec<crate::context::VarnodeId> = Vec::new();
        if let Some(in0) = data.obank().get(retop).and_then(|o| o.get_in(0)) {
            finalparam.push(in0);
        }
        if let Some(pre) = preexist {
            finalparam.push(pre);
        }
        let _ = data.op_set_all_input(retop, &finalparam);
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
        let return_ops: Vec<crate::context::OpId> =
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
                let (trial_size, trial_cond, trial_killed) = (
                    active.get_trial(i).get_size(),
                    active.get_trial(i).has_cond_exe_effect(),
                    active.get_trial(i).is_killed_by_call(),
                );
                let (realistic, solid) =
                    ancestor.execute(data, op, slot, trial_size, trial_cond, trial_killed, false);
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
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ coreaction.cc:2003 — ActionRestrictLocal::apply.
        // The two loops are realized on the Funcdata side (it owns `qlst`/`funcp`/
        // the IR banks the walk needs); `Funcdata::restrict_local` is the faithful
        // transcription.  Now that the RSP keystone made the effect list correct
        // (RSP unaffected), this has the right input.
        data.restrict_local();
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
    fn apply(&mut self, data: &mut Funcdata, _ctx: &mut ActionContext) -> ApplyResult {
        // C++ coreaction.cc:4941 — ActionInputPrototype::apply: the function's OWN
        // input-parameter recovery + typing (the type-plane SEED for an unlocked
        // prototype).
        //
        //   data.getScopeLocal()->clearCategory(Symbol::fake_input): the W4 scope
        //   fake-input category is not on the merged-tree ScopeLocal (no
        //   `fake_input` symbols are created without the W4 markup), so this is a
        //   faithful no-op here.  // STUB(W4 ScopeLocal::clearCategory)
        data.get_func_proto_mut().clear_unlocked_input();

        // The unlocked recovery reads the prototype model (`resolveModel` /
        // `deriveInputMap` / `possibleInputParam`).  In the real pipeline
        // `ActionPrototypeTypes` has already seeded the eval model + the
        // `ActionOutputPrototype` internal store; with neither (a model-less
        // fixture) there is no model to derive a map from, so the recovery cannot
        // run — leave the (empty) proto untouched, only running clearDeadVarnodes.
        let recoverable =
            data.get_func_proto().has_model() && data.get_func_proto().has_store();
        if recoverable && !data.get_func_proto().is_input_locked() {
            // Gather trials over the function's input Varnodes (registers/stack
            // the heritage collected).  `triallist[i]` is the i-th registered
            // trial's Varnode (1-based slot in ParamTrial).
            let mut active = crate::fspec::ParamActive::new(false);
            // (kuna) `inputparamgap`: these are the function's OWN input trials,
            // where an ACTIVE trial means the body reads the register before it
            // writes it.  Tell `forceInactiveChain` that a run of unused argument
            // registers is not evidence against a later one.  See
            // [`crate::p4_calls::kuna_inputparamgap`].
            active.set_own_input_gap(data.get_arch().input_param_gap);
            let mut triallist: Vec<crate::context::VarnodeId> = Vec::new();
            let input_vns: Vec<crate::context::VarnodeId> =
                data.vbank().iter_def_flag(crate::varnode::varnode_flags::input).collect();
            for vn in input_vns {
                let (addr, size, no_descend) = {
                    let v = match data.vbank().get(vn) {
                        Some(v) => v,
                        None => continue,
                    };
                    (v.get_addr().clone(), v.get_size(), v.has_no_descend())
                };
                if data.get_func_proto().possible_input_param(&addr, size) {
                    let slot = active.get_num_trials();
                    active.register_trial(&addr, size);
                    if !no_descend {
                        active.get_trial_mut(slot).mark_active(); // has descendants
                    }
                    triallist.push(vn);
                }
            }
            let manager = data.get_arch().manage.clone();
            let _ = data.get_func_proto_mut().resolve_model(&active);
            let _ = data.get_func_proto().derive_input_map(&mut active, &manager);

            // Create any unreferenced-but-used input Varnodes (or markNoUse if
            // something already occupies the slot).
            let numtrials = active.get_num_trials();
            for i in 0..numtrials {
                let (is_unref, is_used, tsize, taddr) = {
                    let t = active.get_trial(i);
                    (t.is_unref(), t.is_used(), t.get_size(), t.get_address().clone())
                };
                if is_unref && is_used {
                    let intersects = data.has_input_intersection(tsize, &taddr).unwrap_or(false);
                    if intersects {
                        active.get_trial_mut(i).mark_no_use();
                    } else {
                        let vn = data.new_varnode(tsize, &taddr, None);
                        let vn = match data.set_input_varnode(vn) {
                            Ok(v) => v,
                            Err(_) => continue,
                        };
                        let slot = triallist.len() as int4;
                        triallist.push(vn);
                        active.get_trial_mut(i).set_slot(slot + 1);
                    }
                }
            }

            // updateInputTypes / updateInputNoTypes (fspec.cc:4057/4102), inlined
            // here so the trial Varnodes (`&mut Funcdata`) can be read.  The
            // recovery path is high-on (HighVariables assigned), so the typed form
            // runs; the no-types form pins each input to `getBase(size,UNKNOWN)`.
            if data.is_high_on() {
                update_input_types(data, &triallist, &mut active);
            } else {
                update_input_no_types(data, &triallist, &mut active);
            }
        }
        let _ = data.clear_dead_varnodes();
        0
    }
}

/// Update the locked-free input parameters from the recovered trials, pulling
/// each chosen parameter's data-type from its trial Varnode's HighVariable (C++
/// `FuncProto::updateInputTypes`, fspec.cc:4057), inlined at the action so the
/// `&mut Funcdata` HighVariable reads are available.
fn update_input_types(
    data: &mut Funcdata,
    triallist: &[crate::context::VarnodeId],
    active: &mut crate::fspec::ParamActive,
) {
    if data.get_func_proto().is_input_locked() {
        return; // Input is locked, do no updating
    }
    data.get_func_proto_mut().store_clear_all_inputs();
    let mut count = 0i32;
    let numtrials = active.get_num_trials();
    for i in 0..numtrials {
        if !active.get_trial(i).is_used() {
            continue;
        }
        let slot = active.get_trial(i).get_slot();
        let vn = triallist[(slot - 1) as usize];
        if data.vbank().get(vn).map(|v| v.is_mark()).unwrap_or(true) {
            continue;
        }
        // pieces.addr = trial.getAddress(); pieces.type = vn->getHigh()->getType()
        // (the isPersist/findDisjointCover global-input branch is the W4 persist
        // surface — function-input registers/stack are never persistent here, so
        // it is a narrow STUB(W4 findDisjointCover) that does not fire).
        let is_persist = data.vbank().get(vn).map(|v| v.is_persist()).unwrap_or(false);
        let addr = active.get_trial(i).get_address().clone();
        let ty = data
            .high_get_type(vn)
            .unwrap_or_else(|| Rc::new(crate::dtype::Datatype::new(1, crate::dtype::type_metatype::TYPE_UNKNOWN)));
        let _ = is_persist;
        let pieces = crate::fspec::ParameterPieces { addr, type_: Some(ty), flags: 0 };
        data.get_func_proto_mut().store_set_input(count, "", &pieces);
        count += 1;
        data.vbank_mut().get_mut(vn).expect("update_input_types: stale trial").set_mark();
    }
    for &vn in triallist {
        if let Some(v) = data.vbank_mut().get_mut(vn) {
            v.clear_mark();
        }
    }
    data.get_func_proto_mut().update_this_pointer();
}

/// Update the locked-free input parameters from the recovered trials, using only
/// the trial Varnode's size (each parameter typed `getBase(size,UNKNOWN)`) (C++
/// `FuncProto::updateInputNoTypes`, fspec.cc:4102).
fn update_input_no_types(
    data: &mut Funcdata,
    triallist: &[crate::context::VarnodeId],
    active: &mut crate::fspec::ParamActive,
) {
    if data.get_func_proto().is_input_locked() {
        return;
    }
    data.get_func_proto_mut().store_clear_all_inputs();
    let mut count = 0i32;
    let numtrials = active.get_num_trials();
    for i in 0..numtrials {
        if !active.get_trial(i).is_used() {
            continue;
        }
        let slot = active.get_trial(i).get_slot();
        let vn = triallist[(slot - 1) as usize];
        if data.vbank().get(vn).map(|v| v.is_mark()).unwrap_or(true) {
            continue;
        }
        let addr = active.get_trial(i).get_address().clone();
        let sz = data.vbank().get(vn).map(|v| v.get_size()).unwrap_or(1);
        let ty = Rc::new(crate::dtype::Datatype::new(sz, crate::dtype::type_metatype::TYPE_UNKNOWN));
        let pieces = crate::fspec::ParameterPieces { addr, type_: Some(ty), flags: 0 };
        data.get_func_proto_mut().store_set_input(count, "", &pieces);
        count += 1;
        data.vbank_mut().get_mut(vn).expect("update_input_no_types: stale trial").set_mark();
    }
    for &vn in triallist {
        if let Some(v) = data.vbank_mut().get_mut(vn) {
            v.clear_mark();
        }
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
        // C++ coreaction.cc:4999 — ActionOutputPrototype::apply.
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
        // the boundary `Architecture` does not expose it; the formal void type is the
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
        // (kuna, ida) Repair a RETURN whose value is a bogus register PAIR before
        // the output storage and type are read off it. Return recovery registers
        // one trial per output register the model characterizes (x86-64 SysV: RAX
        // *and* RDX) and marks a trial active when its value survives ancestor
        // realism — which asks whether a value could legitimately REACH the
        // RETURN, not whether the function meant to return it. A callee-saved
        // register restore passes that test, so the spec's `join_dual_class`
        // output rule accepts the consecutive pair as one 16-byte return and the
        // function renders `undefined16 main(...)` with a phantom
        // `v[8] = <uninitialized stack slot>` — output that reads memory the
        // function never wrote. Here, unlike at recovery time, heritage has
        // finished and the leftover half is plainly an unwritten Varnode.
        crate::kuna_returnuncomputed::strip_uncomputed_return_piece(data);
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
        // C++ coreaction.cc:5140 — ActionPrototypeWarnings::apply. Generate override
        // messages via data.getOverride().generateOverrideMessages(msgs, getArch())
        // and warningHeader each. Then, for ourproto = data.getFuncProto():
        //   - when ourproto.hasInputErrors(): warningHeader("Cannot assign parameter
        //     locations ...").
        //   - when ourproto.hasOutputErrors(): warningHeader("Cannot assign location
        //     of return value ...").
        //   - when ourproto.isModelUnknown(): s = "Unknown calling convention"; append
        //     ": " + getModelName() when printModelInDecl(); append " -- yet parameter
        //     storage is locked" when !hasCustomStorage() && (isInputLocked() ||
        //     isOutputLocked()); warningHeader(s).
        // Then over each call spec (fc = data.getCallSpecs(i), fd = fc->getFuncdata()):
        //   - when fc->hasInputErrors(): warning("Cannot assign parameter location
        //     for function ...", entryAddr).
        //   - when fc->hasOutputErrors(): warning("Cannot assign location of return
        //     value for function ...", entryAddr).
        //
        // STUB(W7/W8-funcdata): the override-message generation reads
        // `Funcdata::getOverride()` (the local override store is not on
        // `Funcdata` in the merged tree); the function-level headers read
        // `Funcdata::funcp` (the empty `context::FuncProto` placeholder, not the
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
    use crate::context::ArchContext;

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
        let glb = Rc::new(ArchContext::new(manage));
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
