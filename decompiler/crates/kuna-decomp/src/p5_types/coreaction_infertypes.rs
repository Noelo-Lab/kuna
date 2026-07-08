//! The `ActionInferTypes` type-propagation engine (C++ `coreaction.cc:5262-5672`).
//!
//! This is the W8 `ActionInferTypes::apply` body lifted out of the stub in
//! [`coreaction_render`](crate::coreaction_render).  It runs the bounded
//! bidirectional type lattice that the C++ deep decompiler uses to recover
//! Varnode data-types:
//!
//!   1. `buildLocaltypes` — seed every live Varnode's *temporary* data-type from
//!      local op-semantics (`getLocalType` -> `outputTypeLocal`/`inputTypeLocal`,
//!      the W6 typeop inst table).
//!   2. `propagateOneType` / `propagateTypeEdge` — push each seed across the
//!      def-use graph through the per-op-code `propagateType` transform, visiting
//!      each Varnode at most once, trimming where the pushed type is not more
//!      specific (`typeOrder`).
//!   3. `propagateAcrossReturns` — share a single return data-type across multiple
//!      `CPUI_RETURN` inputs.
//!   4. `writeBack` — copy each temporary type onto the permanent Varnode type
//!      (`updateType`), dirtying the owning HighVariable so the inferred type
//!      reaches the printer (the decl) and `ActionOutputPrototype` (the return
//!      type).
//!
//! The 7-pass settle ceiling and the `hasTypeRecoveryStarted` gate stay in the
//! [`coreaction_render::ActionInferTypes`] wrapper; this module is the engine it
//! drives.
//!
//! ## Faithfulness / seams
//!
//! The metatype lattice control flow (COPY, MULTIEQUAL, INDIRECT, every
//! comparison, INT_ADD, and the spacebase pointer construction) plus the 11
//! previously-declining `TypeOp::propagateType` overrides are transcribed:
//! LOAD/STORE (`propagateToPointer`/`propagateFromPointer` — the pointer's `*T`
//! <-> the loaded/stored `T`), PTRADD/PTRSUB (`propagateAddIn2Out` — array/struct
//! member pointer arithmetic via the looping `downChain`), INT_XOR/INT_AND
//! (enum + float sign-manipulation), INT_OR (enum only), PIECE/SUBPIECE
//! (the `resizePointer`/`getSubType`/`findTruncation` composite/truncation arms),
//! SEGMENT (`resizePointer`) and NEW (cpool result).  The remaining seams are the
//! W8 union mid-flow resolution (`needsResolution`/`resolveInFlow`), the
//! type-locked SymbolEntry `getExactPiece` seed, and the
//! `propagateSpacebaseRef`/`applyTypeRecommendations` aliasing passes; those arms
//! conservatively decline (return `None`/no-op), which is faithful — the C++ also
//! returns null along those edges when the supporting surface doesn't apply.

use std::rc::Rc;

use kuna_base::address::{calc_mask, Address};
use kuna_base::space::AddrSpace;
use kuna_base::types::{int4, int8, uintb};
use kuna_num::opcodes::OpCode;

use crate::dtype::{type_metatype, Datatype, TypeFactory};
use crate::funcdata::Funcdata;
use crate::context::{OpId, VarnodeId};
use crate::typeop::type_op_info;

/// Resolve `op->outputTypeLocal()` / `op->inputTypeLocal(slot)` faithfully via the
/// W6 typeop inst table (`getOutputLocal`/`getInputLocal`).  Falls back to a
/// size-correct `TYPE_UNKNOWN` if the factory query fails (the C++ getBase never
/// fails for a valid size).
pub(crate) fn output_type_local(data: &Funcdata, op: OpId) -> Rc<Datatype> {
    let arch = Rc::clone(data.get_arch());
    let o = data.obank().get(op).expect("output_type_local: stale op");
    let out_size = o
        .get_out()
        .and_then(|v| data.vbank().get(v))
        .map(|v| v.get_size())
        .unwrap_or(1);
    // C++ `TypeOpCallother::getOutputLocal` -> `userOp->getOutputLocal(op)`.  For
    // the internal-string builtin (`InternalStringOp::getOutputLocal`,
    // userop.cc:362) this is the op's own (locked) output type — so a
    // `BUILTIN_STRINGDATA` CALLOTHER reports its char-pointer output token and
    // `ActionSetCasts` inserts no spurious cast around the rendered string literal.
    if o.code() == OpCode::CPUI_CALLOTHER {
        let in0_off = o
            .get_in(0)
            .and_then(|v| data.vbank().get(v))
            .map(|v| v.get_offset())
            .unwrap_or(0);
        if in0_off == crate::userop::BUILTIN_STRINGDATA as u64 {
            if let Some(out) = o.get_out().and_then(|v| data.vbank().get(v)) {
                return Rc::clone(out.get_type());
            }
        }
    }
    // C++ `TypeOpCall::getOutputLocal` (typeop.cc:722) / `TypeOpCallind::getOutputLocal`
    // (typeop.cc:778): when the resolved `FuncCallSpecs` has a locked (committed)
    // output prototype, the call output Varnode's suggested type is the callee's
    // (non-VOID) return data-type.  This is what lets a deindirected CALL to a
    // known-proto function (e.g. `int4 *obtainPtr(char *)`) type its return Varnode
    // directly, so `ActionSetCasts` inserts no spurious `(int4 *)` cast.
    if o.code() == OpCode::CPUI_CALL || o.code() == OpCode::CPUI_CALLIND {
        if let Some(ct) = call_output_type_local(data, op, o.code()) {
            return ct;
        }
    }
    let info = type_op_info(o.code());
    match arch.types() {
        Some(tlst) => info
            .get_output_local(tlst, out_size)
            .unwrap_or_else(|_| Rc::new(Datatype::new(out_size, type_metatype::TYPE_UNKNOWN))),
        None => Rc::new(Datatype::new(out_size, type_metatype::TYPE_UNKNOWN)),
    }
}

pub(crate) fn input_type_local(data: &Funcdata, op: OpId, slot: int4) -> Rc<Datatype> {
    let arch = Rc::clone(data.get_arch());
    let o = data.obank().get(op).expect("input_type_local: stale op");
    let opcode = o.code();
    let in_size = o
        .get_in(slot)
        .and_then(|v| data.vbank().get(v))
        .map(|v| v.get_size())
        .unwrap_or(1);

    // C++ `TypeOpCall::getInputLocal` (typeop.cc:689) / `TypeOpCallind` (typeop.cc:763):
    // for a slot>0 whose in(0) is an fspec (CALL) / code-pointer (CALLIND) reference
    // with a recovered prototype, the suggested input type is the *callee's* locked
    // parameter type.  This is what flows a typed call argument (e.g. a `mystruct *`)
    // back onto the argument Varnode so the stack-frame member access is recovered.
    if (opcode == OpCode::CPUI_CALL || opcode == OpCode::CPUI_CALLIND) && slot > 0 {
        if let Some(ct) = call_input_type_local(data, op, slot, opcode) {
            return ct;
        }
    }

    // C++ `TypeOpCallind::getInputLocal(op, 0)` (typeop.cc:763-768): the first input
    // of a CALLIND is the *function pointer*, so its suggested type is `code *` —
    // `getTypePointer(in0Size, getTypeCode(), spc->getWordSize())`.  This is what
    // types the indirect-call target Varnode `code *v1` (and lets the printer render
    // `(*v1)(...)` rather than treating the funcptr as a plain integer).
    if opcode == OpCode::CPUI_CALLIND && slot == 0 {
        if let Some(tlst) = arch.types() {
            let code = tlst.get_type_code().unwrap_or_else(|_| {
                Rc::new(Datatype::new(0, type_metatype::TYPE_CODE))
            });
            let spc = o.get_addr().get_space().cloned();
            let wordsize = spc.as_ref().map(|s| s.get_word_size()).unwrap_or(1);
            if let Ok(ptr) = tlst.get_type_pointer(in_size, code, wordsize) {
                return ptr;
            }
        }
    }

    // C++ `TypeOpReturn::getInputLocal` (typeop.cc:903-921): a RETURN's value input
    // (slot >= 1) suggests the *function's* recovered output data-type, so a
    // struct-by-value return (e.g. `foo` in rax / edx:eax) flows onto the returned
    // Varnode and `RulePieceStructure` can split the forming PIECE tree into per-field
    // writes.  The guard mirrors the C++ exactly: non-VOID output type whose size
    // equals the input Varnode's size; otherwise the generic `getBase(size, UNKNOWN)`
    // default applies (the `bb == null` arm is implicit — a live RETURN op always has
    // a parent).  The `has_store()` precondition is the W4 no-store STUB guard: an
    // unrecovered proto has no output type to consult (typeop.cc:917 `getFuncProto`
    // is always backed by a store in C++; the merged tree may lack one).
    if opcode == OpCode::CPUI_RETURN && slot >= 1 && data.get_func_proto().has_store() {
        if let Some(ct) = data.get_func_proto().get_output_type() {
            if ct.get_metatype() != type_metatype::TYPE_VOID && ct.get_size() == in_size {
                return Rc::clone(ct);
            }
        }
    }

    let info = type_op_info(opcode);
    match arch.types() {
        Some(tlst) => info
            .get_input_local(tlst, slot, in_size)
            .unwrap_or_else(|_| Rc::new(Datatype::new(in_size, type_metatype::TYPE_UNKNOWN))),
        None => Rc::new(Datatype::new(in_size, type_metatype::TYPE_UNKNOWN)),
    }
}

/// The callee-parameter arm of `TypeOpCall::getInputLocal` / `TypeOpCallind::getInputLocal`
/// (typeop.cc:689-721 / 763-810): resolve the `FuncCallSpecs` for the CALL op and,
/// if parameter `slot-1` is type-locked (or a known `this` pointer to a struct),
/// return that parameter's data-type when it fits the argument Varnode.  `None`
/// falls back to the generic `getInputLocal`.
fn call_input_type_local(
    data: &Funcdata,
    op: OpId,
    slot: int4,
    opcode: OpCode,
) -> Option<Rc<Datatype>> {
    // C++: vn = op->getIn(0); for a CALL the prototype lives only when in(0) is an
    // fspec reference (CALLIND always consults the recovered spec).
    if opcode == OpCode::CPUI_CALL {
        let in0 = data.obank().get(op)?.get_in(0)?;
        let is_fspec = data
            .vbank()
            .get(in0)
            .and_then(|v| v.get_addr().get_space())
            .map(|s| s.get_type() == kuna_base::space::spacetype::IPTR_FSPEC)
            .unwrap_or(false);
        if !is_fspec {
            return None;
        }
    }
    // Resolve the FuncCallSpecs for this op (the C++ `getFspecFromConst` pointer
    // cast; this port keys the `qlst` by the spec's CALL op).
    let n = data.num_calls();
    let mut fc = None;
    for i in 0..n {
        if data.get_call_specs(i).get_op() == op {
            fc = Some(data.get_call_specs(i));
            break;
        }
    }
    let fc = fc?;
    // param = fc->getParam(slot - 1)
    let proto = fc.proto();
    let param = proto.get_param(slot - 1)?;
    let arg_size =
        data.obank().get(op).and_then(|o| o.get_in(slot)).and_then(|v| data.vbank().get(v)).map(|v| v.get_size())?;
    if param.is_type_locked() {
        let ct = param.get_type()?;
        // (ct->metatype != VOID) && (ct->size <= op->getIn(slot)->size)
        if ct.get_metatype() != type_metatype::TYPE_VOID && ct.get_size() <= arg_size {
            return Some(Rc::clone(ct));
        }
    } else if param.is_this_pointer() {
        // Known "this" pointer is effectively typelocked: a struct pointer flows.
        let ct = param.get_type()?;
        if ct.get_metatype() == type_metatype::TYPE_PTR
            && ct
                .get_ptr_to()
                .map(|p| p.get_metatype() == type_metatype::TYPE_STRUCT)
                .unwrap_or(false)
        {
            return Some(Rc::clone(ct));
        }
    }
    None
}

/// The locked-output arm of `TypeOpCall::getOutputLocal` (typeop.cc:722-738) /
/// `TypeOpCallind::getOutputLocal` (typeop.cc:778-791): resolve the `FuncCallSpecs`
/// for the CALL/CALLIND op and, if its output prototype is locked (committed) and
/// non-VOID, return that return data-type.  `None` falls back to the generic
/// size-only `getOutputLocal`.
fn call_output_type_local(data: &Funcdata, op: OpId, opcode: OpCode) -> Option<Rc<Datatype>> {
    // C++ `TypeOpCall`: the prototype lives only when in(0) is an fspec reference;
    // a CALL whose in(0) is not an fspec falls through to the default.
    if opcode == OpCode::CPUI_CALL {
        let in0 = data.obank().get(op)?.get_in(0)?;
        let is_fspec = data
            .vbank()
            .get(in0)
            .and_then(|v| v.get_addr().get_space())
            .map(|s| s.get_type() == kuna_base::space::spacetype::IPTR_FSPEC)
            .unwrap_or(false);
        if !is_fspec {
            return None;
        }
    }
    // Resolve the FuncCallSpecs for this op (C++ `getFspecFromConst` for CALL /
    // `getCallSpecs` for CALLIND; this port keys `qlst` by the spec's CALL op).
    let n = data.num_calls();
    let mut fc = None;
    for i in 0..n {
        if data.get_call_specs(i).get_op() == op {
            fc = Some(data.get_call_specs(i));
            break;
        }
    }
    let fc = fc?;
    let proto = fc.proto();
    // if (!fc->isOutputLocked()) return default;
    if !proto.is_output_locked() {
        return None;
    }
    // ct = fc->getOutputType(); if (ct->metatype == VOID) return default; return ct.
    let ct = proto.get_output_type()?;
    if ct.get_metatype() == type_metatype::TYPE_VOID {
        return None;
    }
    Some(Rc::clone(ct))
}

/// C++ `Varnode::getLocalType(bool &blockup)` (varnode.cc:919).  Determine an
/// initial Datatype from the defining op's output and the reading ops' input
/// requirements, picking the most-specific (`typeOrder`).  Returns the local type
/// and whether up-propagation should be blocked (`def->stopsTypePropagation()`).
fn get_local_type(data: &Funcdata, vn: VarnodeId) -> (Rc<Datatype>, bool) {
    let v = data.vbank().get(vn).expect("get_local_type: stale vn");
    // if (isTypeLock()) return type;  (not a partial lock)
    if v.is_type_lock() {
        return (Rc::clone(v.get_type()), false);
    }
    let mut ct: Option<Rc<Datatype>> = None;
    let mut blockup = false;
    // if (def != null) { ct = def->outputTypeLocal(); if (def->stopsTypePropagation()) ... }
    if let Some(def) = v.get_def() {
        ct = Some(output_type_local(data, def));
        let stops = data
            .obank()
            .get(def)
            .map(|o| o.stops_type_propagation())
            .unwrap_or(false);
        if stops {
            blockup = true;
            return (ct.unwrap(), blockup);
        }
    }
    // for each reading op: newct = op->inputTypeLocal(slot); pick the more specific.
    let descend: Vec<OpId> = v.descend_iter().collect();
    for op in descend {
        let slot = data.obank().get(op).map(|o| o.get_slot(vn)).unwrap_or(0);
        let newct = input_type_local(data, op, slot);
        ct = Some(match ct {
            None => newct,
            Some(cur) => {
                if 0 > newct.type_order(&cur).unwrap_or(0) {
                    newct
                } else {
                    cur
                }
            }
        });
    }
    // C++ throws on a null local type; a live Varnode always has a def or a
    // descendant, but guard with a size-correct UNKNOWN to avoid a panic.
    let ct = ct.unwrap_or_else(|| Rc::new(Datatype::new(v.get_size(), type_metatype::TYPE_UNKNOWN)));
    (ct, blockup)
}

/// C++ `ActionInferTypes::buildLocaltypes` (coreaction.cc:5262): seed each live
/// Varnode's temp type from local info.  (The type-locked SymbolEntry piece path
/// is a W4 surface — absent symbols fall through to the plain `getLocalType`.)
fn build_localtypes(data: &mut Funcdata) {
    let order: Vec<VarnodeId> = data.vbank().iter_loc().collect();
    for vn in order {
        let (vn_addr, vn_size, vn_type_lock);
        {
            let v = match data.vbank().get(vn) {
                Some(v) => v,
                None => continue,
            };
            if v.is_annotation() {
                continue;
            }
            if !v.is_written() && v.has_no_descend() {
                continue;
            }
            vn_addr = v.get_addr().clone();
            vn_size = v.get_size();
            vn_type_lock = v.is_type_lock();
        }
        // C++ buildLocaltypes type-locked-symbol seed (coreaction.cc:5275-5281):
        //   SymbolEntry *entry = vn->getSymbolEntry();
        //   if (entry && !vn->isTypeLock() && entry->getSymbol()->isTypeLocked()) {
        //     ct = typegrp->getExactPiece(symbolType, curOff, vn->getSize());
        //     if (ct == 0 || ct->getMetatype() == TYPE_UNKNOWN) ct = vn->getLocalType(...);
        //   } else ct = vn->getLocalType(...);
        // The seed is consulted only when the Varnode is itself not type-locked
        // (a type-locked Varnode already carries its own definitive type via the
        // getLocalType `isTypeLock` fast-path).
        let seed = if !vn_type_lock {
            data.get_scope_local().and_then(|lm| {
                data.get_arch()
                    .types()
                    .and_then(|t| lm.build_localtype_seed(&vn_addr, vn_size, t))
            })
        } else {
            None
        };
        let (ct, needs_block) = match seed {
            // getExactPiece resolved a non-UNKNOWN piece: adopt it (no up-block;
            // the C++ seed arm leaves `needsBlock` false).
            Some(ct) => (ct, false),
            // No type-locked cover, or the piece floated: fall back to getLocalType.
            None => get_local_type(data, vn),
        };
        let v = data.vbank_mut().get_mut(vn).expect("build_localtypes: stale vn");
        if needs_block {
            v.set_stop_up_propagation();
        }
        v.set_temp_type(ct);
    }
}

/// C++ `ActionInferTypes::writeBack` (coreaction.cc:5297): copy each temp type to
/// the permanent type via `updateType`, dirtying the owning high.  Returns whether
/// any Varnode's data-type changed.
fn write_back(data: &mut Funcdata) -> bool {
    let mut change = false;
    let order: Vec<VarnodeId> = data.vbank().iter_loc().collect();
    for vn in order {
        let ct = {
            let v = match data.vbank().get(vn) {
                Some(v) => v,
                None => continue,
            };
            if v.is_annotation() {
                continue;
            }
            if !v.is_written() && v.has_no_descend() {
                continue;
            }
            match v.get_temp_type() {
                Some(ct) => Rc::clone(ct),
                None => continue,
            }
        };
        let high = data.vbank().get(vn).and_then(|v| v.get_high());
        let changed = data
            .vbank_mut()
            .get_mut(vn)
            .map(|v| v.update_type(ct))
            .unwrap_or(false);
        if changed {
            // C++ `Varnode::updateType` calls `high->typeDirty()`; the merged-tree
            // setter cannot reach the high bank, so dirty it here.
            if let Some(h) = high {
                if let Some(hh) = data.high_bank_mut().get_mut(h) {
                    hh.type_dirty();
                }
            }
            change = true;
        }
    }
    change
}

/// The "incoming" Varnode of a propagation edge: `inslot==-1` => op output, else
/// the op's input at `inslot`.
fn edge_in_vn(data: &Funcdata, op: OpId, inslot: int4) -> Option<VarnodeId> {
    let o = data.obank().get(op)?;
    if inslot == -1 {
        o.get_out()
    } else {
        o.get_in(inslot)
    }
}

/// C++ `ActionInferTypes::propagateTypeEdge` (coreaction.cc:5328): try to push the
/// incoming Varnode's temp type across one PcodeOp edge onto the outgoing Varnode.
/// Returns whether the outgoing temp type was updated *and* the outgoing Varnode is
/// not yet marked (the propagateOneType DFS guard).
fn propagate_type_edge(data: &mut Funcdata, op: OpId, inslot: int4, outslot: int4) -> bool {
    if inslot == outslot {
        return false; // don't backtrack
    }
    let invn = match edge_in_vn(data, op, inslot) {
        Some(v) => v,
        None => return false,
    };
    // alttype = invn->getTempType();
    let mut alttype = match data.vbank().get(invn).and_then(|v| v.get_temp_type().cloned()) {
        Some(t) => t,
        None => return false,
    };
    // if (alttype->needsResolution()) {
    //   Datatype *resType = alttype->resolveInFlow(op, inslot);
    //   if (!op->isMarker()) alttype = resType;
    // }
    // (C++ coreaction.cc:5335-5341)  Always give the incoming union/relative-pointer
    // a chance to resolve so the field choice is cached for later facing lookups,
    // but only adopt the resolved type for the propagation when `op` is not a MULTIEQUAL
    // marker (a marker keeps the unresolved union flowing through both edges).
    if alttype.needs_resolution() {
        let res_type = match data.resolve_in_flow(&alttype, op, inslot) {
            Ok(t) => t,
            Err(_) => Rc::clone(&alttype),
        };
        let is_marker = data.obank().get(op).map(|o| o.is_marker()).unwrap_or(false);
        if !is_marker {
            alttype = res_type;
        }
    }

    // Resolve the outgoing Varnode.
    let outvn = if outslot < 0 {
        match data.obank().get(op).and_then(|o| o.get_out()) {
            Some(v) => v,
            None => return false,
        }
    } else {
        match data.obank().get(op).and_then(|o| o.get_in(outslot)) {
            Some(v) => v,
            None => return false,
        }
    };

    {
        let ov = match data.vbank().get(outvn) {
            Some(v) => v,
            None => return false,
        };
        if outslot >= 0 && ov.is_annotation() {
            return false;
        }
        if ov.is_type_lock() {
            return false; // Can't propagate through typelock
        }
        if ov.stops_up_propagation() && outslot >= 0 {
            return false; // Propagation is blocked
        }
        // if (alttype is BOOL) only propagate if output can only take boolean values.
        if alttype.get_metatype() == type_metatype::TYPE_BOOL && ov.get_nz_mask() > 1 {
            return false;
        }
    }

    let newtype = match propagate_type(data, alttype, op, invn, outvn, inslot, outslot) {
        Some(t) => t,
        None => return false,
    };

    // if (0 > newtype->typeOrder(*outvn->getTempType())) { setTempType; return !isMark }
    let cur = match data.vbank().get(outvn).and_then(|v| v.get_temp_type().cloned()) {
        Some(t) => t,
        None => return false,
    };
    if 0 > newtype.type_order(&cur).unwrap_or(0) {
        let is_mark = data.vbank().get(outvn).map(|v| v.is_mark()).unwrap_or(true);
        if let Some(v) = data.vbank_mut().get_mut(outvn) {
            v.set_temp_type(newtype);
        }
        return !is_mark;
    }
    false
}

/// Build a spacebase code-pointer type `getTypePointer(altSize, getBase(1,UNKNOWN),
/// defaultDataSpace.wordSize)` — the shared `invn->isSpacebase()` arm of COPY /
/// MULTIEQUAL / INDIRECT / compare propagation.
fn spacebase_pointer(data: &Funcdata, alt_size: int4) -> Option<Rc<Datatype>> {
    let arch = Rc::clone(data.get_arch());
    let tlst = arch.types()?;
    let spc = Rc::clone(arch.manage().get_default_data_space()?);
    let base = tlst.get_base(1, type_metatype::TYPE_UNKNOWN).ok()?;
    tlst.get_type_pointer(alt_size, base, spc.get_word_size()).ok()
}

/// C++ `getSpaceFromConst(op->getIn(0))`: the LOAD/STORE/SEGMENT space operand is
/// a constant whose offset is the address-space manager index (LOSS-015 model).
fn space_from_const(data: &Funcdata, op: OpId, slot: int4) -> Option<Rc<AddrSpace>> {
    let cvn = data.obank().get(op)?.get_in(slot)?;
    let idx = data.vbank().get(cvn)?.get_offset();
    let manage = data.get_arch().manage();
    if idx >= manage.num_spaces() as u64 {
        return None;
    }
    // cast: `idx` is a space index < num_spaces() (guarded above); `as i32` matches
    // C++ `getSpace(int4)` taking the constant Varnode offset as a small index.
    manage.get_space(idx as i32).cloned()
}

/// C++ `TypeOp::propagateToPointer` (typeop.cc:187): propagate a dereferenced
/// data-type up to its pointer data-type through a LOAD/STORE (depth at most 1).
fn propagate_to_pointer(
    tlst: &dyn TypeFactory,
    dt: Rc<Datatype>,
    sz: int4,
    wordsz: int4,
) -> Option<Rc<Datatype>> {
    let meta = dt.get_metatype();
    let dt = if meta == type_metatype::TYPE_PTR {
        // Make sure that at least we return a pointer to something the size of -pt-
        tlst.get_base(dt.get_size(), type_metatype::TYPE_UNKNOWN).ok()?
    } else if meta == type_metatype::TYPE_PARTIALSTRUCT {
        dt.get_component_for_ptr()?
    } else {
        dt
    };
    tlst.get_type_pointer(sz, dt, wordsz as u32).ok()
}

/// C++ `TypeOp::propagateFromPointer` (typeop.cc:207): propagate a pointer
/// data-type down to its element data-type through a LOAD/STORE.
fn propagate_from_pointer(
    tlst: &dyn TypeFactory,
    dt: Rc<Datatype>,
    sz: int4,
) -> Option<Rc<Datatype>> {
    if dt.get_metatype() != type_metatype::TYPE_PTR {
        return None;
    }
    let ptrto = dt.get_ptr_to()?;
    if ptrto.is_variable_length() {
        return None;
    }
    if ptrto.get_size() == sz {
        return Some(ptrto);
    }
    // Size mismatch: only propagate (partial) enumerations.
    if dt.is_pointer_rel() {
        let parent = dt.get_rel_parent()?;
        let byte_off = dt.get_byte_offset()?;
        let res = tlst.get_exact_piece(parent, byte_off, sz).ok()?;
        if let Some(res) = res {
            if res.is_enum_type() {
                return Some(res);
            }
        }
    } else if ptrto.is_enum_type() && !ptrto.has_stripped() {
        return tlst.get_type_partial_enum(ptrto, 0, sz).ok();
    }
    None
}

/// C++ `TypeOp::floatSignManipulation` (typeop.cc:154): return CPUI_FLOAT_NEG if
/// the op flips the sign bit, CPUI_FLOAT_ABS if it zeroes it out, else CPUI_MAX.
fn float_sign_manipulation(data: &Funcdata, op: OpId) -> OpCode {
    let o = match data.obank().get(op) {
        Some(o) => o,
        None => return OpCode::CPUI_MAX,
    };
    let opc = o.code();
    let cvn = match o.get_in(1) {
        Some(v) => v,
        None => return OpCode::CPUI_MAX,
    };
    let (is_const, off, size) = match data.vbank().get(cvn) {
        Some(v) => (v.is_constant(), v.get_offset(), v.get_size()),
        None => return OpCode::CPUI_MAX,
    };
    if !is_const {
        return OpCode::CPUI_MAX;
    }
    if opc == OpCode::CPUI_INT_AND {
        let val = calc_mask(size) >> 1;
        if val == off {
            return OpCode::CPUI_FLOAT_ABS;
        }
    } else if opc == OpCode::CPUI_INT_XOR {
        let mask = calc_mask(size);
        let val = mask ^ (mask >> 1);
        if val == off {
            return OpCode::CPUI_FLOAT_NEG;
        }
    }
    OpCode::CPUI_MAX
}

/// The `nearPointerSize`/`farPointerSize` the PIECE/SUBPIECE constructors compute
/// (`farPointerSize = getSizeOfAltPointer(); if (farPointerSize != 0) nearPointerSize
/// = getSizeOfPointer()`).
fn piece_pointer_sizes(data: &Funcdata) -> (int4, int4) {
    let arch = Rc::clone(data.get_arch());
    let tlst = match arch.types() {
        Some(t) => t,
        None => return (0, 0),
    };
    let far = tlst.get_size_of_alt_pointer();
    let near = if far != 0 { tlst.get_size_of_pointer() } else { 0 };
    (near, far)
}

/// Public wrapper around [`propagate_type`] (the per-op-code
/// `TypeOp::propagateType` dispatch) for in-crate callers that need to propagate a
/// type along a single edge — notably `AddTreeState::assignPropagatedType`
/// (`op->getOpcode()->propagateType(inType, op, vn, out, 0, -1)`).
// (kuna) verifier test seam: widened `pub(crate)` -> `pub` so the
// `verify_w10_charptr_signedness` adversarial integration test can drive the
// `INT_SLESS`/`INT_SLESSEQUAL` signedness gate directly.  No behaviour change.
#[allow(clippy::too_many_arguments)]
pub fn propagate_type_pub(
    data: &mut Funcdata,
    alttype: Rc<Datatype>,
    op: OpId,
    invn: VarnodeId,
    outvn: VarnodeId,
    inslot: int4,
    outslot: int4,
) -> Option<Rc<Datatype>> {
    propagate_type(data, alttype, op, invn, outvn, inslot, outslot)
}

/// C++ per-op-code `TypeOp::propagateType` dispatch (typeop.cc).  Returns the
/// outgoing data-type, or `None` (no propagation).
fn propagate_type(
    data: &mut Funcdata,
    alttype: Rc<Datatype>,
    op: OpId,
    invn: VarnodeId,
    outvn: VarnodeId,
    inslot: int4,
    outslot: int4,
) -> Option<Rc<Datatype>> {
    let code = data.obank().get(op)?.code();
    let invn_is_spacebase = data.vbank().get(invn).map(|v| v.is_spacebase()).unwrap_or(false);
    match code {
        // TypeOpCopy / TypeOpMulti / TypeOpIndirect: input <-> output, spacebase
        // produces a code pointer (typeop.cc:412, 1953, 2007).
        OpCode::CPUI_COPY | OpCode::CPUI_MULTIEQUAL => {
            if inslot != -1 && outslot != -1 {
                return None; // Must propagate input <-> output
            }
            if invn_is_spacebase {
                spacebase_pointer(data, alttype.get_size())
            } else {
                Some(alttype)
            }
        }
        OpCode::CPUI_INDIRECT => {
            let is_creation =
                data.obank().get(op).map(|o| o.is_indirect_creation()).unwrap_or(false);
            if is_creation {
                return None;
            }
            if inslot == 1 || outslot == 1 {
                return None;
            }
            if inslot != -1 && outslot != -1 {
                return None;
            }
            if invn_is_spacebase {
                spacebase_pointer(data, alttype.get_size())
            } else {
                Some(alttype)
            }
        }
        // TypeOpEqual / TypeOpNotEqual / TypeOpIntLess / TypeOpIntLessEqual all
        // dispatch to `propagateAcrossCompare` (typeop.cc:947, 1011, 1087, 1111).
        // This is the generic comparison arm: identity propagation between the two
        // inputs, with the spacebase / mid-struct-relptr special-cases.
        OpCode::CPUI_INT_EQUAL
        | OpCode::CPUI_INT_NOTEQUAL
        | OpCode::CPUI_INT_LESS
        | OpCode::CPUI_INT_LESSEQUAL => {
            propagate_across_compare(data, alttype, invn_is_spacebase, outvn, inslot, outslot)
        }
        // TypeOpIntSless / TypeOpIntSlessEqual override `propagateType` with a
        // STRICTER arm (typeop.cc:1035-1041, 1061-1067): the propagation is across
        // the two inputs (in/out both != -1), and it ONLY propagates a *signed*
        // (TYPE_INT) data-type — "Only propagate signed things".  A `char`
        // (TYPE_INT, size 1) read by a signed compare therefore flows to the other
        // operand, while a `uint1`/pointer does not.  There is NO spacebase /
        // mid-struct-relptr handling on this arm (the C++ override does not call
        // propagateAcrossCompare).
        OpCode::CPUI_INT_SLESS | OpCode::CPUI_INT_SLESSEQUAL => {
            if inslot == -1 || outslot == -1 {
                return None; // Must propagate input <-> input
            }
            if alttype.get_metatype() != type_metatype::TYPE_INT {
                return None; // Only propagate signed things
            }
            Some(alttype)
        }
        // TypeOpIntAdd: pointer add / constant-folded int (typeop.cc:1183).
        OpCode::CPUI_INT_ADD => propagate_int_add(data, alttype, op, outvn, inslot, outslot),
        // TypeOpLoad / TypeOpStore: propagate the pointed-to type through the
        // pointer's `*T` <-> the loaded/stored `T` (typeop.cc:488, 559).
        OpCode::CPUI_LOAD => {
            propagate_load_store(data, alttype, op, invn_is_spacebase, outvn, inslot, outslot, true)
        }
        OpCode::CPUI_STORE => {
            propagate_load_store(data, alttype, op, invn_is_spacebase, outvn, inslot, outslot, false)
        }
        // TypeOpIntXor / TypeOpIntAnd: propagate enums + float sign-manipulation
        // (typeop.cc:1424, 1457).
        OpCode::CPUI_INT_XOR | OpCode::CPUI_INT_AND => {
            if !alttype.is_enum_type() {
                if alttype.get_metatype() != type_metatype::TYPE_FLOAT {
                    return None;
                }
                if float_sign_manipulation(data, op) == OpCode::CPUI_MAX {
                    return None;
                }
            }
            if invn_is_spacebase {
                spacebase_pointer(data, alttype.get_size())
            } else {
                Some(alttype)
            }
        }
        // TypeOpIntOr: only propagate enums (typeop.cc:1490).
        OpCode::CPUI_INT_OR => {
            if !alttype.is_enum_type() {
                return None;
            }
            if invn_is_spacebase {
                spacebase_pointer(data, alttype.get_size())
            } else {
                Some(alttype)
            }
        }
        // TypeOpPiece / TypeOpSubpiece: composite/truncation arms (typeop.cc:2076,
        // 2163).
        OpCode::CPUI_PIECE => {
            propagate_piece(data, alttype, op, invn, outvn, inslot, outslot)
        }
        OpCode::CPUI_SUBPIECE => {
            propagate_subpiece(data, alttype, op, invn, outvn, inslot, outslot)
        }
        // TypeOpPtradd / TypeOpPtrsub: array/struct member pointer arithmetic
        // (typeop.cc:2270, 2368).
        OpCode::CPUI_PTRADD => {
            if inslot == 2 || outslot == 2 {
                return None; // Don't propagate along this edge
            }
            if inslot != -1 && outslot != -1 {
                return None; // Must propagate input <-> output
            }
            if alttype.get_metatype() != type_metatype::TYPE_PTR {
                return None;
            }
            if inslot == -1 {
                None // Don't propagate pointer types this direction
            } else {
                propagate_add_in2_out(data, alttype, op, inslot)
            }
        }
        OpCode::CPUI_PTRSUB => {
            if inslot != -1 && outslot != -1 {
                return None; // Must propagate input <-> output
            }
            if alttype.get_metatype() != type_metatype::TYPE_PTR {
                return None;
            }
            if inslot == -1 {
                None // Don't propagate pointer types this direction
            } else {
                propagate_add_in2_out(data, alttype, op, inslot)
            }
        }
        // TypeOpSegment: propagate slot2 <-> output, resized (typeop.cc:2433).
        OpCode::CPUI_SEGMENTOP => {
            if inslot == 0 || inslot == 1 || outslot == 0 || outslot == 1 {
                return None;
            }
            if invn_is_spacebase {
                return None;
            }
            if alttype.get_metatype() != type_metatype::TYPE_PTR {
                return None;
            }
            let out_size = data.vbank().get(outvn).map(|v| v.get_size())?;
            let arch = Rc::clone(data.get_arch());
            arch.types()?.resize_pointer(alttype, out_size).ok()
        }
        // TypeOpNew: propagate the cpool result as result of the new operator
        // (typeop.cc:2503).
        OpCode::CPUI_NEW => {
            if inslot != 0 || outslot != -1 {
                return None;
            }
            let vn0 = data.obank().get(op)?.get_in(0)?;
            let v0 = data.vbank().get(vn0)?;
            if !v0.is_written() {
                return None;
            }
            let def = v0.get_def()?;
            let def_code = data.obank().get(def)?.code();
            if def_code != OpCode::CPUI_CPOOLREF {
                return None;
            }
            Some(alttype)
        }
        _ => None, // default TypeOp::propagateType: don't propagate
    }
}

/// C++ `TypeOpLoad::propagateType` / `TypeOpStore::propagateType` (typeop.cc:488,
/// 559).  `is_load` selects the LOAD-specific slot conventions.
#[allow(clippy::too_many_arguments)]
fn propagate_load_store(
    data: &Funcdata,
    alttype: Rc<Datatype>,
    op: OpId,
    invn_is_spacebase: bool,
    outvn: VarnodeId,
    inslot: int4,
    outslot: int4,
    is_load: bool,
) -> Option<Rc<Datatype>> {
    if inslot == 0 || outslot == 0 {
        return None; // Don't propagate along this edge
    }
    if invn_is_spacebase {
        return None;
    }
    let out_size = data.vbank().get(outvn).map(|v| v.get_size())?;
    let arch = Rc::clone(data.get_arch());
    let tlst = arch.types()?;
    // LOAD: propagating output to input (value -> ptr) is inslot == -1.
    // STORE: propagating value (slot 2) to ptr is inslot == 2.
    let to_pointer = if is_load { inslot == -1 } else { inslot == 2 };
    if to_pointer {
        let spc = space_from_const(data, op, 0)?;
        propagate_to_pointer(tlst, alttype, out_size, spc.get_word_size() as int4)
    } else {
        propagate_from_pointer(tlst, alttype, out_size)
    }
}

/// C++ `TypeOpPiece::propagateType` (typeop.cc:2076).
fn propagate_piece(
    data: &Funcdata,
    alttype: Rc<Datatype>,
    op: OpId,
    invn: VarnodeId,
    outvn: VarnodeId,
    inslot: int4,
    outslot: int4,
) -> Option<Rc<Datatype>> {
    let (near_ptr, far_ptr) = piece_pointer_sizes(data);
    let in_size = data.vbank().get(invn).map(|v| v.get_size())?;
    let out_size = data.vbank().get(outvn).map(|v| v.get_size())?;
    if near_ptr != 0 && alttype.get_metatype() == type_metatype::TYPE_PTR {
        let arch = Rc::clone(data.get_arch());
        if inslot == 1 && outslot == -1 {
            if in_size == near_ptr && out_size == far_ptr {
                return arch.types()?.resize_pointer(alttype, far_ptr).ok();
            }
        } else if inslot == -1
            && outslot == 1
            && in_size == far_ptr
            && out_size == near_ptr
        {
            return arch.types()?.resize_pointer(alttype, near_ptr).ok();
        }
        return None;
    }
    if inslot != -1 {
        return None;
    }
    let mut byte_off = compute_byte_offset_for_composite_piece(data, op, outslot)? as int8;
    let mut cur = Some(alttype);
    while let Some(ct) = cur.clone() {
        if byte_off == 0 && ct.get_size() == out_size {
            break;
        }
        let (sub, newoff) = ct.get_sub_type(byte_off).ok()?;
        byte_off = newoff;
        cur = sub;
    }
    cur
}

/// C++ `TypeOpPiece::computeByteOffsetForComposite` (typeop.cc:2106).
fn compute_byte_offset_for_composite_piece(
    data: &Funcdata,
    op: OpId,
    slot: int4,
) -> Option<int4> {
    let o = data.obank().get(op)?;
    let in0 = o.get_in(0)?;
    let in1 = o.get_in(1)?;
    let big_endian = data.vbank().get(in0).map(|v| v.get_space().is_big_endian()).unwrap_or(false);
    let in0_size = data.vbank().get(in0).map(|v| v.get_size())?;
    let in1_size = data.vbank().get(in1).map(|v| v.get_size())?;
    let byte_off = if big_endian {
        if slot == 0 {
            0
        } else {
            in0_size
        }
    } else if slot == 0 {
        in1_size
    } else {
        0
    };
    Some(byte_off)
}

/// C++ `TypeOpSubpiece::propagateType` (typeop.cc:2163).
fn propagate_subpiece(
    data: &Funcdata,
    alttype: Rc<Datatype>,
    op: OpId,
    invn: VarnodeId,
    outvn: VarnodeId,
    inslot: int4,
    outslot: int4,
) -> Option<Rc<Datatype>> {
    let (near_ptr, far_ptr) = piece_pointer_sizes(data);
    let in_size = data.vbank().get(invn).map(|v| v.get_size())?;
    let out_size = data.vbank().get(outvn).map(|v| v.get_size())?;
    if near_ptr != 0
        && alttype.get_metatype() == type_metatype::TYPE_PTR
        && inslot == -1
        && outslot == 0
    {
        // Try to propagate UP, producing a far pointer input from a near pointer
        // output of the SUBPIECE.  The DOWN case is handled by getSubType below.
        let in1_off = data
            .obank()
            .get(op)
            .and_then(|o| o.get_in(1))
            .and_then(|v| data.vbank().get(v))
            .map(|v| v.get_offset())
            .unwrap_or(0);
        if in1_off != 0 {
            return None;
        }
        if in_size == near_ptr && out_size == far_ptr {
            let arch = Rc::clone(data.get_arch());
            return arch.types()?.resize_pointer(alttype, far_ptr).ok();
        }
        return None;
    }
    if inslot != 0 || outslot != -1 {
        return None; // Propagation must be from in0 to out
    }
    let mut byte_off = compute_byte_offset_for_composite_subpiece(data, op)? as int8;
    // UNION/PARTIALUNION resolveTruncation is the W8 union mid-flow surface; the
    // common (non-union) path falls through to the getSubType walk below.
    let mut cur = Some(alttype);
    while let Some(ct) = cur.clone() {
        if byte_off == 0 && ct.get_size() == out_size {
            break;
        }
        let (sub, newoff) = ct.get_sub_type(byte_off).ok()?;
        byte_off = newoff;
        cur = sub;
    }
    cur
}

/// C++ `TypeOpSubpiece::computeByteOffsetForComposite` (typeop.cc:2197).
fn compute_byte_offset_for_composite_subpiece(data: &Funcdata, op: OpId) -> Option<int4> {
    let o = data.obank().get(op)?;
    let out = o.get_out()?;
    let in0 = o.get_in(0)?;
    let in1 = o.get_in(1)?;
    let out_size = data.vbank().get(out).map(|v| v.get_size())?;
    let lsb = data.vbank().get(in1).map(|v| v.get_offset() as int4).unwrap_or(0);
    let in0_size = data.vbank().get(in0).map(|v| v.get_size())?;
    let big_endian = data.vbank().get(in0).map(|v| v.get_space().is_big_endian()).unwrap_or(false);
    let byte_off = if big_endian { in0_size - out_size - lsb } else { lsb };
    Some(byte_off)
}

/// C++ `TypeOpEqual::propagateAcrossCompare` (typeop.cc:965).
fn propagate_across_compare(
    data: &Funcdata,
    alttype: Rc<Datatype>,
    invn_is_spacebase: bool,
    outvn: VarnodeId,
    inslot: int4,
    outslot: int4,
) -> Option<Rc<Datatype>> {
    if inslot == -1 || outslot == -1 {
        return None;
    }
    if invn_is_spacebase {
        return spacebase_pointer(data, alttype.get_size());
    }
    // isPointerRel struct mid-pointer: if we know the pointer is in the middle of
    // a structure, don't propagate the relptr across the comparison (the two sides
    // are likely different types, and the other side can type from the structure
    // pointer); hand back a bare pointer instead.  The non-relptr / non-struct path
    // is the identity propagation.
    let outvn_const = data.vbank().get(outvn).map(|v| v.is_constant()).unwrap_or(false);
    if alttype.is_pointer_rel() && !outvn_const {
        let parent_is_struct = alttype
            .get_rel_parent()
            .map(|p| p.get_metatype() == type_metatype::TYPE_STRUCT)
            .unwrap_or(false);
        let byte_off = alttype.get_byte_offset().unwrap_or(-1);
        if parent_is_struct && byte_off >= 0 {
            let arch = Rc::clone(data.get_arch());
            let tlst = arch.types()?;
            let base = tlst.get_base(1, type_metatype::TYPE_UNKNOWN).ok()?;
            let ws = alttype.get_word_size().unwrap_or(1);
            return tlst.get_type_pointer(alttype.get_size(), base, ws).ok();
        }
    }
    Some(alttype)
}

/// C++ `TypeOpIntAdd::propagateType` + `propagateAddIn2Out` (typeop.cc:1183/1217).
/// The non-pointer int/uint constant-fold arm is transcribed; the pointer
/// `downChain` walk routes through the existing `Datatype::down_chain` machinery.
fn propagate_int_add(
    data: &Funcdata,
    alttype: Rc<Datatype>,
    op: OpId,
    outvn: VarnodeId,
    inslot: int4,
    outslot: int4,
) -> Option<Rc<Datatype>> {
    let invn_meta = alttype.get_metatype();
    if invn_meta != type_metatype::TYPE_PTR {
        if invn_meta != type_metatype::TYPE_INT && invn_meta != type_metatype::TYPE_UINT {
            return None;
        }
        let in1_const = data
            .obank()
            .get(op)
            .and_then(|o| o.get_in(1))
            .and_then(|v| data.vbank().get(v))
            .map(|v| v.is_constant())
            .unwrap_or(false);
        if outslot != 1 || !in1_const {
            return None;
        }
    } else if inslot != -1 && outslot != -1 {
        return None; // Must propagate input <-> output for pointers
    }
    let outvn_const = data.vbank().get(outvn).map(|v| v.is_constant()).unwrap_or(false);
    if outvn_const && alttype.get_metatype() != type_metatype::TYPE_PTR {
        Some(alttype)
    } else if inslot == -1 {
        None // Don't propagate pointer types output -> input
    } else {
        propagate_add_in2_out(data, alttype, op, inslot)
    }
}

/// C++ `TypeOpIntAdd::propagateAddPointer` (typeop.cc:1270): classify the
/// data-type edge as a pointer propagating through an "add a constant" op.
/// Returns `(command, off)`:
///   - 0: "add a constant" adding zero (PTRSUB or PTRADD)
///   - 1: "add a constant", `off` passed back
///   - 2: pointer does not propagate through
///   - 3: input data-type propagates through untransformed
fn propagate_add_pointer(data: &Funcdata, op: OpId, slot: int4, sz: int4) -> (int4, uintb) {
    let o = match data.obank().get(op) {
        Some(o) => o,
        None => return (2, 0),
    };
    let code = o.code();
    if code == OpCode::CPUI_PTRADD {
        if slot != 0 {
            return (2, 0);
        }
        let (const_off, const_size, is_const) = o
            .get_in(1)
            .and_then(|v| data.vbank().get(v))
            .map(|v| (v.get_offset(), v.get_size(), v.is_constant()))
            .unwrap_or((0, 0, false));
        let mult = o.get_in(2).and_then(|v| data.vbank().get(v)).map(|v| v.get_offset()).unwrap_or(0);
        if is_const {
            let off = (const_off.wrapping_mul(mult)) & calc_mask(const_size);
            return if off == 0 { (0, 0) } else { (1, off) };
        }
        // cast: C++ `(mult % sz)` promotes int4 `sz` to the uintb of `mult`; `sz as uintb`
        // is that same int4->uintb promotion (sz is the non-negative align size).
        if sz != 0 && (mult % sz as uintb) != 0 {
            return (2, 0);
        }
        return (3, 0);
    }
    if code == OpCode::CPUI_PTRSUB {
        if slot != 0 {
            return (2, 0);
        }
        let off = o.get_in(1).and_then(|v| data.vbank().get(v)).map(|v| v.get_offset()).unwrap_or(0);
        return if off == 0 { (0, 0) } else { (1, off) };
    }
    if code == OpCode::CPUI_INT_ADD {
        let othervn = match o.get_in(1 - slot) {
            Some(v) => v,
            None => return (2, 0),
        };
        let (is_const, off, ov_written, ov_temp_is_ptr) = {
            let v = match data.vbank().get(othervn) {
                Some(v) => v,
                None => return (2, 0),
            };
            let temp_is_ptr = v
                .get_temp_type()
                .map(|t| t.get_metatype() == type_metatype::TYPE_PTR)
                .unwrap_or(false);
            (v.is_constant(), v.get_offset(), v.is_written(), temp_is_ptr)
        };
        if !is_const {
            // Check if othervn is an offset (othervn = MULT(x, const)).
            if ov_written {
                let multop = data.vbank().get(othervn).and_then(|v| v.get_def());
                if let Some(multop) = multop {
                    let mcode = data.obank().get(multop).map(|o| o.code());
                    if mcode == Some(OpCode::CPUI_INT_MULT) {
                        let (mult, mult_size, mult_const) = data
                            .obank()
                            .get(multop)
                            .and_then(|o| o.get_in(1))
                            .and_then(|v| data.vbank().get(v))
                            .map(|v| (v.get_offset(), v.get_size(), v.is_constant()))
                            .unwrap_or((0, 0, false));
                        if mult_const {
                            if mult == calc_mask(mult_size) {
                                // Multiplying by -1: assume a pointer difference.
                                return (2, 0);
                            }
                            // cast: int4->uintb promotion of `sz` in C++ `(mult % sz)`.
                            if sz != 0 && (mult % sz as uintb) != 0 {
                                return (2, 0);
                            }
                        }
                        return (3, 0);
                    }
                }
            }
            if sz == 1 {
                return (3, 0);
            }
            return (2, 0);
        }
        if ov_temp_is_ptr {
            // othervn marked as ptr.
            return (2, 0);
        }
        return if off == 0 { (0, 0) } else { (1, off) };
    }
    (2, 0)
}

/// C++ `TypeOpIntAdd::propagateAddIn2Out` (typeop.cc:1217): propagate a pointer
/// data-type through an ADD/PTRADD/PTRSUB from an input to its output via the
/// `propagateAddPointer` command analysis + the looping `downChain` walk
/// (preserving any containing TYPE_STRUCT/TYPE_ARRAY as a relative pointer).
fn propagate_add_in2_out(
    data: &Funcdata,
    alttype: Rc<Datatype>,
    op: OpId,
    inslot: int4,
) -> Option<Rc<Datatype>> {
    let arch = Rc::clone(data.get_arch());
    let tlst = arch.types_impl()?;
    let ptr_to = alttype.get_ptr_to()?;
    let align = ptr_to.get_align_size();
    let (command, offset) = propagate_add_pointer(data, op, inslot, align);
    if command == 2 {
        return None; // Doesn't look like a good pointer add
    }
    let mut pointer: Option<Rc<Datatype>> = Some(Rc::clone(&alttype));
    let mut parent: Option<Rc<Datatype>> = None;
    let mut parent_off: int8 = 0;
    if command != 3 {
        let word_size = alttype.get_word_size().unwrap_or(1);
        // cast: `offset` is uintb (u64); C++ `addressToByteInt(uintb,...)` takes the
        // value reinterpreted as int8 — `offset as i64` is that same bit-reinterpret.
        let mut type_offset = AddrSpace::address_to_byte_int(offset as i64, word_size);
        let op_code = data.obank().get(op)?.code();
        let allow_wrap = op_code != OpCode::CPUI_PTRSUB;
        // C++ do { pointer = pointer->downChain(typeOffset,...); if (0) break; }
        //      while(typeOffset != 0);
        while let Some(cur) = pointer.clone() {
            // C++ `TypePointer::downChain` (type.cc:1224-1257) dispatches the
            // pointed-to `getSubType(off,&off)` to `TypeSpacebase::getSubType`,
            // which reaches the symbol scope to resolve the local/global Symbol at
            // `off` (type.cc:1248).  The generic `Datatype::down_chain` cannot reach
            // that scope, so a `PTRSUB(spacebase,-0xN)` would otherwise propagate as
            // a bare 8-byte pointer (int8) and force a spurious cast to the mapped
            // local's type.  Reproduce the spacebase arm here (scope-aware), exactly
            // as `get_output_token_ptrsub` does for the cast plane, so the PTRSUB
            // output is typed as a pointer to the mapped local during inference.
            let cur_is_spacebase = cur
                .get_ptr_to()
                .map(|p| p.get_metatype() == type_metatype::TYPE_SPACEBASE)
                .unwrap_or(false);
            if cur_is_spacebase {
                let sbptrto = cur.get_ptr_to()?;
                let cur_size = cur.get_size();
                let cur_ws = cur.get_word_size().unwrap_or(1);
                if let Some((subtype, residual)) =
                    data.spacebase_get_sub_type(&sbptrto, type_offset)
                {
                    type_offset = residual;
                    // The spacebase is never an array/struct container itself, so
                    // `par`/`parOff` stay unset; `!isArray -> getTypePointerStripArray`.
                    pointer = tlst.get_type_pointer_strip_array(cur_size, subtype, cur_ws).ok();
                } else {
                    pointer = None;
                }
                if pointer.is_none() || type_offset == 0 {
                    break;
                }
                continue;
            }
            let (next, new_off, par, par_off) =
                tlst.down_chain(&cur, type_offset, allow_wrap).ok()?;
            type_offset = new_off;
            // C++ downChain writes its `par`/`parOff` reference params ONLY on the
            // STRUCT/ARRAY arm (type.cc:1247-1250) and never resets them otherwise,
            // so across iterations the LAST struct/array container seen is retained.
            // Mirror that: only overwrite when the descent actually produced a
            // container (par.is_some()); a scalar/enum tail must NOT clear an
            // outer struct/array container set by an earlier iteration.
            if par.is_some() {
                parent = par;
                parent_off = par_off;
            }
            pointer = next;
            // C++: break out of the do-while once pointer is null or typeOffset == 0.
            if pointer.is_none() || type_offset == 0 {
                break;
            }
        }
    }
    if let Some(par) = parent {
        // Innermost containing object is a TYPE_STRUCT/TYPE_ARRAY: preserve it.
        let pt = match &pointer {
            None => tlst.get_base(1, type_metatype::TYPE_UNKNOWN).ok()?,
            Some(p) => p.get_ptr_to()?,
        };
        // cast: C++ `getTypePointerRel(parent, pt, parentOff)` takes an int4 offset;
        // `parentOff` is int8 there — this narrows int8->int4 exactly as the C++ call.
        let parent_off_i4: int4 = parent_off as int4;
        pointer = tlst.get_type_pointer_rel(par, pt, parent_off_i4).ok();
    }
    let pointer = match pointer {
        None => {
            if command == 0 {
                return Some(alttype);
            }
            return None;
        }
        Some(p) => p,
    };
    // If the input is a spacebase pointer-to-spacebase, demote to plain pointer.
    let in_is_spacebase = data
        .obank()
        .get(op)
        .and_then(|o| o.get_in(inslot))
        .and_then(|v| data.vbank().get(v))
        .map(|v| v.is_spacebase())
        .unwrap_or(false);
    if in_is_spacebase {
        let pt_meta = pointer.get_ptr_to().map(|p| p.get_metatype());
        if pt_meta == Some(type_metatype::TYPE_SPACEBASE) {
            let base = tlst.get_base(1, type_metatype::TYPE_UNKNOWN).ok()?;
            let ws = pointer.get_word_size().unwrap_or(1);
            return tlst.get_type_pointer(pointer.get_size(), base, ws).ok();
        }
    }
    Some(pointer)
}

/// C++ `ActionInferTypes::propagateOneType` (coreaction.cc:5428): DFS push a
/// Varnode's temp type as far as possible.  Mirrors the `PropagationState` stack
/// walk (descendants then the def), visiting each Varnode at most once via the
/// mark bit.
fn propagate_one_type(data: &mut Funcdata, root: VarnodeId) {
    // PropagationState: (vn, descend index, op, slot, inslot).
    struct PState {
        vn: VarnodeId,
        descend: Vec<OpId>,
        d_idx: usize,
        op: Option<OpId>,
        slot: int4,
        inslot: int4,
    }

    // C++ PropagationState::PropagationState(Varnode *v) (coreaction.cc:5371): if
    // the Varnode has descendants start at the first; else start at the def with
    // inslot=-1.
    fn make_state(data: &Funcdata, vn: VarnodeId) -> PState {
        let descend: Vec<OpId> =
            data.vbank().get(vn).map(|v| v.descend_iter().collect()).unwrap_or_default();
        let mut st = PState { vn, descend, d_idx: 0, op: None, slot: 0, inslot: -1 };
        if !st.descend.is_empty() {
            let op = st.descend[0];
            st.d_idx = 1;
            st.op = Some(op);
            let has_out = data.obank().get(op).map(|o| o.get_out().is_some()).unwrap_or(false);
            st.slot = if has_out { -1 } else { 0 };
            st.inslot = data.obank().get(op).map(|o| o.get_slot(vn)).unwrap_or(0);
        } else {
            // op = vn->getDef(); inslot=-1; slot=0.
            st.op = data.vbank().get(vn).and_then(|v| v.get_def());
            st.inslot = -1;
            st.slot = 0;
        }
        st
    }

    // PropagationState::valid() <=> op != null.
    fn valid(st: &PState) -> bool {
        st.op.is_some()
    }

    // PropagationState::step() (coreaction.cc:5395) — verbatim.
    fn step(st: &mut PState, data: &Funcdata) {
        st.slot += 1;
        let numin = st.op.and_then(|o| data.obank().get(o)).map(|o| o.num_input()).unwrap_or(0);
        if st.slot < numin {
            return;
        }
        if st.d_idx < st.descend.len() {
            let op = st.descend[st.d_idx];
            st.d_idx += 1;
            st.op = Some(op);
            let has_out = data.obank().get(op).map(|o| o.get_out().is_some()).unwrap_or(false);
            st.slot = if has_out { -1 } else { 0 };
            st.inslot = data.obank().get(op).map(|o| o.get_slot(st.vn)).unwrap_or(0);
            return;
        }
        // Descendants exhausted: if we just processed the def (inslot==-1) stop;
        // otherwise (we just processed a descendant) move to the def.
        if st.inslot == -1 {
            st.op = None;
        } else {
            st.op = data.vbank().get(st.vn).and_then(|v| v.get_def());
        }
        st.inslot = -1;
        st.slot = 0;
    }

    let mut stack: Vec<PState> = Vec::new();
    stack.push(make_state(data, root));
    if let Some(v) = data.vbank_mut().get_mut(root) {
        v.set_mark();
    }

    while let Some(top) = stack.last_mut() {
        if !valid(top) {
            let vn = top.vn;
            if let Some(v) = data.vbank_mut().get_mut(vn) {
                v.clear_mark();
            }
            stack.pop();
            continue;
        }
        let op = top.op.unwrap();
        let inslot = top.inslot;
        let slot = top.slot;
        if propagate_type_edge(data, op, inslot, slot) {
            // newvn = (slot==-1) ? op->out : op->in(slot).
            let newvn = if slot == -1 {
                data.obank().get(op).and_then(|o| o.get_out())
            } else {
                data.obank().get(op).and_then(|o| o.get_in(slot))
            };
            // step before push_back.
            step(stack.last_mut().unwrap(), data);
            if let Some(newvn) = newvn {
                stack.push(make_state(data, newvn));
                if let Some(v) = data.vbank_mut().get_mut(newvn) {
                    v.set_mark();
                }
            }
        } else {
            step(stack.last_mut().unwrap(), data);
        }
    }
}

/// C++ `ActionInferTypes::propagateRef` (coreaction.cc:5464): given a likely
/// pointer Varnode `vn` and a known alias `addr` of what it points at, try to
/// propagate `vn`'s pointee data-type onto the Varnodes that live at `addr`.
///
/// This is the pass that turns a typed stack-pointer (e.g. `mystruct *` from a
/// call argument) into a typed stack *local* (`mystruct v1`): the pointee type
/// is laid over every Varnode in the stack-frame slice the pointer addresses.
fn propagate_ref(data: &mut Funcdata, vn: VarnodeId, addr: &Address) {
    // ct = vn->getTempType(); must be a pointer to a concrete (non-spacebase,
    // non-unknown) type.
    let ct = match data.vbank().get(vn).and_then(|v| v.get_temp_type().cloned()) {
        Some(t) => t,
        None => return,
    };
    if ct.get_metatype() != type_metatype::TYPE_PTR {
        return;
    }
    let ct = match ct.get_ptr_to() {
        Some(p) => p,
        None => return,
    };
    let ctmeta = ct.get_metatype();
    if ctmeta == type_metatype::TYPE_SPACEBASE || ctmeta == type_metatype::TYPE_UNKNOWN {
        return; // Don't bother propagating this
    }
    let arch = Rc::clone(data.get_arch());
    let typegrp = match arch.types() {
        Some(t) => t,
        None => return,
    };

    let off = addr.get_offset();
    let ct_size = ct.get_size();
    // endaddr = addr + ct->getSize();  if it wrapped, run to end of the space.
    // cast: `Address + i64` advances the offset; a data-type byte size (int4) fits i64.
    let endaddr = addr + ct_size as i64;

    // Snapshot the membership window so the &mut propagate_one_type below does not
    // alias the iteration borrow (the C++ holds a live VarnodeLocSet iterator; the
    // set is not mutated structurally inside the loop — only Varnode temptypes
    // change — so the snapshot is equivalent).
    let curvns: Vec<VarnodeId> = data.vbank().iter_loc_addr_range(addr, &endaddr).collect();

    let mut lastoff: uintb = 0;
    let mut lastsize: int4 = ct_size;
    let mut lastct: Option<Rc<Datatype>> = Some(Rc::clone(&ct));
    for curvn in curvns {
        let (is_annot, written, no_descend, typelock, has_symentry, curoff_abs, cursize) =
            match data.vbank().get(curvn) {
                Some(v) => (
                    v.is_annotation(),
                    v.is_written(),
                    v.has_no_descend(),
                    v.is_type_lock(),
                    v.kuna_symbol_entry().is_some(),
                    v.get_offset(),
                    v.get_size(),
                ),
                None => continue,
            };
        if is_annot {
            continue;
        }
        if !written && no_descend {
            continue;
        }
        if typelock {
            continue;
        }
        // C++ `if (curvn->getSymbolEntry() != 0) continue;` (coreaction.cc:5490):
        // skip Varnodes already bound to a Symbol's SymbolEntry.  kuna's
        // `kuna_symbol_entry` is the faithful `mapentry` proxy (set by
        // `setSymbolEntry`/`syncVarnodesWithSymbol` when a Varnode is bound to a
        // stack Symbol) — NOT the `isMapped()` flag (a distinct bit set far earlier
        // at heritage time on every stack-frame Varnode).  Using `isMapped()` here
        // over-skipped EVERY stack Varnode, killing the spacebase type seed (the W4
        // proxy LOSS); `getSymbolEntry()` is the correct, more permissive gate.
        if has_symentry {
            continue;
        }
        let curoff = curoff_abs.wrapping_sub(off);
        if curoff.wrapping_add(cursize as uintb) > ct_size as uintb {
            continue;
        }
        if cursize != lastsize || curoff != lastoff {
            lastoff = curoff;
            lastsize = cursize;
            // getExactPiece(ct, curoff, cursize).
            // cast: `curoff` (uintb) is the byte offset within `ct`, bounded by the
            // guard above (`curoff + cursize <= ct->getSize()`), so it fits int4.
            lastct = typegrp.get_exact_piece(Rc::clone(&ct), curoff as int4, cursize).ok().flatten();
        }
        let lct = match &lastct {
            Some(c) => Rc::clone(c),
            None => continue,
        };
        // if (0 > lastct->typeOrder(*curvn->getTempType())) — the new type is more
        // specific; adopt it and re-propagate.
        let cur_temp = match data.vbank().get(curvn).and_then(|v| v.get_temp_type().cloned()) {
            Some(t) => t,
            None => continue,
        };
        if 0 > lct.type_order(&cur_temp).unwrap_or(0) {
            if let Some(v) = data.vbank_mut().get_mut(curvn) {
                v.set_temp_type(Rc::clone(&lct));
            }
            propagate_one_type(data, curvn);
        }
    }
}

/// C++ `ActionInferTypes::propagateSpacebaseRef` (coreaction.cc:5521): look for
/// ADD/PTRSUB/PTRADD/COPY off the spacebase (stack-pointer) input whose output
/// has a known pointer type, and propagate that type into the stack frame at the
/// corresponding offset (via [`propagate_ref`]).
fn propagate_spacebase_ref(data: &mut Funcdata, spcvn: VarnodeId) {
    // spctype = spcvn->getType() (the absolute type, NOT temptype); must be
    // pointer-to-spacebase.
    let spctype = match data.vbank().get(spcvn).map(|v| Rc::clone(v.get_type())) {
        Some(t) => t,
        None => return,
    };
    if spctype.get_metatype() != type_metatype::TYPE_PTR {
        return;
    }
    let sbtype = match spctype.get_ptr_to() {
        Some(p) => p,
        None => return,
    };
    if sbtype.get_metatype() != type_metatype::TYPE_SPACEBASE {
        return;
    }
    let arch = Rc::clone(data.get_arch());
    let manager = arch.manage();

    // Snapshot the descendant ops (the C++ holds a live beginDescend iterator;
    // propagate_ref does not add/remove descendants of spcvn).
    let descend: Vec<OpId> = match data.vbank().get(spcvn) {
        Some(v) => v.descend_iter().collect(),
        None => return,
    };
    for op in descend {
        let (code, op_addr) = match data.obank().get(op) {
            Some(o) => (o.code(), o.get_addr().clone()),
            None => continue,
        };
        match code {
            OpCode::CPUI_COPY => {
                // vn = op->getIn(0); addr = sbtype->getAddress(0, vn->getSize(), op->getAddr())
                let in0 = data.obank().get(op).and_then(|o| o.get_in(0));
                let sz = in0.and_then(|v| data.vbank().get(v).map(|x| x.get_size())).unwrap_or(0);
                if let (Some(addr), Some(outvn)) = (
                    sbtype.spacebase_get_address(0, sz, &op_addr, manager),
                    data.obank().get(op).and_then(|o| o.get_out()),
                ) {
                    propagate_ref(data, outvn, &addr);
                }
            }
            OpCode::CPUI_INT_ADD | OpCode::CPUI_PTRSUB => {
                // vn = op->getIn(1); if constant: addr = sbtype->getAddress(vn->offset, vn->size, op->getAddr())
                let in1 = data.obank().get(op).and_then(|o| o.get_in(1));
                let (is_const, voff, vsz) = match in1.and_then(|v| data.vbank().get(v)) {
                    Some(v) => (v.is_constant(), v.get_offset(), v.get_size()),
                    None => continue,
                };
                if is_const {
                    if let (Some(addr), Some(outvn)) = (
                        sbtype.spacebase_get_address(voff, vsz, &op_addr, manager),
                        data.obank().get(op).and_then(|o| o.get_out()),
                    ) {
                        propagate_ref(data, outvn, &addr);
                    }
                }
            }
            OpCode::CPUI_PTRADD => {
                // vn = op->getIn(1); if constant: off = vn->offset * op->getIn(2)->offset
                let in1 = data.obank().get(op).and_then(|o| o.get_in(1));
                let (is_const, voff, vsz) = match in1.and_then(|v| data.vbank().get(v)) {
                    Some(v) => (v.is_constant(), v.get_offset(), v.get_size()),
                    None => continue,
                };
                if is_const {
                    let in2off = data
                        .obank()
                        .get(op)
                        .and_then(|o| o.get_in(2))
                        .and_then(|v| data.vbank().get(v).map(|x| x.get_offset()))
                        .unwrap_or(0);
                    let off = voff.wrapping_mul(in2off);
                    if let (Some(addr), Some(outvn)) = (
                        sbtype.spacebase_get_address(off, vsz, &op_addr, manager),
                        data.obank().get(op).and_then(|o| o.get_out()),
                    ) {
                        propagate_ref(data, outvn, &addr);
                    }
                }
            }
            _ => {}
        }
    }
}

/// C++ `ActionInferTypes::canonicalReturnOp` (coreaction.cc:5567).
fn canonical_return_op(data: &Funcdata) -> Option<OpId> {
    let mut res: Option<OpId> = None;
    let mut bestdt: Option<Rc<Datatype>> = None;
    for retop in data.obank().iter_code(OpCode::CPUI_RETURN) {
        let o = data.obank().get(retop)?;
        if o.is_dead() || o.get_halt_type() != 0 {
            continue;
        }
        if o.num_input() > 1 {
            let vn = match o.get_in(1) {
                Some(v) => v,
                None => continue,
            };
            let ct = data.vbank().get(vn).and_then(|v| v.get_temp_type().cloned());
            let ct = match ct {
                Some(t) => t,
                None => continue,
            };
            match &bestdt {
                None => {
                    res = Some(retop);
                    bestdt = Some(ct);
                }
                Some(best) => {
                    if ct.type_order(best).unwrap_or(0) < 0 {
                        res = Some(retop);
                        bestdt = Some(ct);
                    }
                }
            }
        }
    }
    res
}

/// C++ `ActionInferTypes::propagateAcrossReturns` (coreaction.cc:5598): share a
/// single return data-type across multiple CPUI_RETURN inputs.
fn propagate_across_returns(data: &mut Funcdata) {
    if data.get_func_proto().is_output_locked() {
        return;
    }
    let canon = match canonical_return_op(data) {
        Some(op) => op,
        None => return,
    };
    let base_vn = match data.obank().get(canon).and_then(|o| o.get_in(1)) {
        Some(v) => v,
        None => return,
    };
    let (ct, base_size) = {
        let v = match data.vbank().get(base_vn) {
            Some(v) => v,
            None => return,
        };
        let ct = match v.get_temp_type() {
            Some(t) => Rc::clone(t),
            None => return,
        };
        (ct, v.get_size())
    };
    let is_bool = ct.get_metatype() == type_metatype::TYPE_BOOL;

    let retops: Vec<OpId> = data.obank().iter_code(OpCode::CPUI_RETURN).collect();
    for retop in retops {
        if retop == canon {
            continue;
        }
        let o = match data.obank().get(retop) {
            Some(o) => o,
            None => continue,
        };
        if o.is_dead() || o.get_halt_type() != 0 || o.num_input() <= 1 {
            continue;
        }
        let vn = match o.get_in(1) {
            Some(v) => v,
            None => continue,
        };
        {
            let v = match data.vbank().get(vn) {
                Some(v) => v,
                None => continue,
            };
            if v.get_size() != base_size {
                continue;
            }
            if is_bool && v.get_nz_mask() > 1 {
                continue;
            }
            // if (vn->getTempType() == ct) continue;  (already propagated)
            if v.get_temp_type().map(|t| Rc::ptr_eq(t, &ct)).unwrap_or(false) {
                continue;
            }
        }
        if let Some(v) = data.vbank_mut().get_mut(vn) {
            v.set_temp_type(Rc::clone(&ct));
        }
        propagate_one_type(data, vn);
    }
}

/// C++ `ActionInferTypes::apply` body (coreaction.cc:5630), minus the localcount
/// ceiling / `hasTypeRecoveryStarted` gate (kept in the wrapper).  Returns whether
/// `writeBack` reported a data-type change (used by the wrapper to bump
/// `localcount`).
pub fn run_infer_types(data: &mut Funcdata) -> bool {
    // data.getScopeLocal()->applyTypeRecommendations(): lock any pending stack
    // input-Varnode type recommendations (the `this`-pointer collection is the
    // merged tree's only source; a no-op on a recommendation-free function).
    data.apply_type_recommendations();
    build_localtypes(data);
    let order: Vec<VarnodeId> = data.vbank().iter_loc().collect();
    for vn in order {
        {
            let v = match data.vbank().get(vn) {
                Some(v) => v,
                None => continue,
            };
            if v.is_annotation() {
                continue;
            }
            if !v.is_written() && v.has_no_descend() {
                continue;
            }
        }
        propagate_one_type(data, vn);
    }
    propagate_across_returns(data);
    // C++ coreaction.cc:5663-5666 —
    //   spcid = data.getScopeLocal()->getSpaceId();
    //   spcvn = data.findSpacebaseInput(spcid);
    //   if (spcvn != 0) propagateSpacebaseRef(data, spcvn);
    // Flow the recovered pointer type (e.g. a `mystruct *` call argument) from the
    // spacebase-relative ADD/PTRSUB/PTRADD outputs into the stack-frame slice they
    // address, so a stack struct/array carries its member type.
    let spcid = data.get_scope_local().map(|sl| std::rc::Rc::clone(sl.get_space_id()));
    if let Some(spcid) = spcid {
        if let Some(spcvn) = data.find_spacebase_input(&spcid) {
            propagate_spacebase_ref(data, spcvn);
        }
    }
    write_back(data)
}

#[cfg(test)]
mod propagate_type_tests {
    //! Focused tests of the LOAD/STORE pointer-propagation helpers (C++
    //! `TypeOp::propagateToPointer`/`propagateFromPointer`) — the core of the
    //! W10 propagateType overrides.  These exercise the type transforms directly
    //! against a populated `TypeFactoryImpl` (no full `Funcdata` required), so a
    //! regression in the pointer `*T <-> T` flow is caught even while the corpus
    //! seeds (typed parameters / stack locals) remain unported (LOSS-131/132).
    use super::*;
    use crate::dtype::TypeFactoryImpl;

    fn factory() -> TypeFactoryImpl {
        let f = TypeFactoryImpl::new();
        f.set_default_alignment_map();
        f.set_max_basetype_size(8);
        f
    }

    #[test]
    fn from_pointer_dereferences_to_pointee_when_sizes_match() {
        // int4 * --LOAD(4)--> int4   (propagateFromPointer: ptrto->getSize()==sz)
        let f = factory();
        let int4t = f.get_base(4, type_metatype::TYPE_INT).unwrap();
        let ptr = f.get_type_pointer(8, Rc::clone(&int4t), 1).unwrap();
        let out = propagate_from_pointer(&f, ptr, 4).expect("should dereference");
        assert_eq!(out.get_metatype(), type_metatype::TYPE_INT);
        assert_eq!(out.get_size(), 4);
        assert!(Rc::ptr_eq(&out, &int4t), "returns the exact pointee type");
    }

    #[test]
    fn from_pointer_declines_on_size_mismatch_for_non_enum() {
        // int4 * --LOAD(2)--> (size mismatch, not an enum) -> no propagation.
        let f = factory();
        let int4t = f.get_base(4, type_metatype::TYPE_INT).unwrap();
        let ptr = f.get_type_pointer(8, int4t, 1).unwrap();
        assert!(propagate_from_pointer(&f, ptr, 2).is_none());
    }

    #[test]
    fn from_pointer_declines_for_non_pointer_input() {
        let f = factory();
        let int8t = f.get_base(8, type_metatype::TYPE_INT).unwrap();
        assert!(propagate_from_pointer(&f, int8t, 8).is_none());
    }

    #[test]
    fn to_pointer_wraps_pointee_in_a_pointer_of_the_pointer_size() {
        // int4 --STORE(value->ptr)--> int4 * (propagateToPointer of a plain value).
        let f = factory();
        let int4t = f.get_base(4, type_metatype::TYPE_INT).unwrap();
        let out = propagate_to_pointer(&f, Rc::clone(&int4t), 8, 1).expect("should wrap");
        assert_eq!(out.get_metatype(), type_metatype::TYPE_PTR);
        assert_eq!(out.get_size(), 8);
        let pointee = out.get_ptr_to().unwrap();
        assert!(Rc::ptr_eq(&pointee, &int4t), "points at the original value type");
    }

    #[test]
    fn to_pointer_of_a_pointer_demotes_pointee_to_unknown_of_same_size() {
        // A pointer value never produces a ptr->ptr deeper than depth 1: the
        // pointee is reduced to an UNKNOWN of the pointer's size.
        let f = factory();
        let int4t = f.get_base(4, type_metatype::TYPE_INT).unwrap();
        let inner = f.get_type_pointer(8, int4t, 1).unwrap();
        let out = propagate_to_pointer(&f, inner, 8, 1).expect("should wrap");
        assert_eq!(out.get_metatype(), type_metatype::TYPE_PTR);
        let pointee = out.get_ptr_to().unwrap();
        assert_eq!(pointee.get_metatype(), type_metatype::TYPE_UNKNOWN);
        assert_eq!(pointee.get_size(), 8);
    }

    #[test]
    fn from_pointer_to_struct_dereferences_whole_struct_when_size_matches() {
        // A pointer to an 8-byte struct, dereferenced at full size, yields the
        // struct — the ptr->*struct flow the array/struct overrides build on.
        let f = factory();
        let int4t = f.get_base(4, type_metatype::TYPE_INT).unwrap();
        let st = f.get_type_struct("twofield").unwrap();
        let fields = vec![
            crate::dtype::TypeField::new(0, 0, "a", Rc::clone(&int4t)),
            crate::dtype::TypeField::new(1, 4, "b", Rc::clone(&int4t)),
        ];
        let st = f.assign_raw_fields_struct(&st, fields, Vec::new()).unwrap();
        let sz = st.get_size();
        let ptr = f.get_type_pointer(8, Rc::clone(&st), 1).unwrap();
        let out = propagate_from_pointer(&f, ptr, sz).expect("deref whole struct");
        assert_eq!(out.get_metatype(), type_metatype::TYPE_STRUCT);
        assert!(Rc::ptr_eq(&out, &st));
    }

    // ---- W10 verifier (w10-type-propagation) adversarial tests ------------
    // Hunt-list targets: the enum size-mismatch arm of propagateFromPointer
    // (newly-wired getTypePartialEnum), the PARTIALSTRUCT arm of
    // propagateToPointer (getComponentForPtr), and the multi-level downChain
    // parent retention that propagateAddIn2Out relies on.

    /// An enum factory with a non-zero enum size (setup_sizes seeds enumsize).
    fn enum_factory() -> TypeFactoryImpl {
        let f = TypeFactoryImpl::new();
        f.set_default_alignment_map();
        f.set_max_basetype_size(8);
        // stack_pointer_size, default_data_addr_size, default_size(=enumsize)
        f.setup_sizes(Some(8), 8, 4);
        f
    }

    /// C++ typeop.cc:225-227 — a pointer to an enum, dereferenced at a *smaller*
    /// size, propagates a partial enumeration (not null, not the whole enum).
    /// This is the wrap-prone enum arm: getTypePartialEnum(ptrto, 0, sz).
    #[test]
    fn w10_from_pointer_enum_size_mismatch_yields_partial_enum() {
        let f = enum_factory();
        let en = f.get_type_enum("E").expect("enum");
        assert!(en.is_enum_type(), "fixture must be a real enum");
        let full = en.get_size();
        assert!(full >= 2, "enum must be wider than the truncated deref");
        let ptr = f.get_type_pointer(8, Rc::clone(&en), 1).unwrap();
        // Dereference at a strictly smaller size than the enum.
        let out = propagate_from_pointer(&f, ptr, 1).expect("partial enum, not null");
        // C++ `TypePartialEnum(par,off,sz,strip)` chains `TypeEnum(sz,
        // TYPE_PARTIALENUM)`, whose ctor body **overrides** the live metatype to
        // `(TYPE_PARTIALENUM==TYPE_ENUM_INT)?TYPE_INT:TYPE_UINT` = TYPE_UINT and
        // sets the `enumtype` flag (type.hh:548, type.cc:2683).  The
        // `TYPE_PARTIALENUM` value survives only in `submeta`
        // (`SUB_UINT_PARTIALENUM`) and as the marshalling override.  So the
        // result must read as an ENUM, not as a bare TYPE_PARTIALENUM metatype.
        assert!(
            out.is_enum_type(),
            "partial enum keeps the enumtype flag so getExactPiece/AND typing recognises it"
        );
        assert_eq!(
            out.get_metatype(),
            type_metatype::TYPE_UINT,
            "TypeEnum ctor overrides metatype to TYPE_UINT for a partial enum"
        );
        assert!(out.has_stripped(), "TypePartialEnum sets has_stripped (type.cc:2687)");
        assert_eq!(out.get_size(), 1, "partial covers the dereferenced size");
    }

    /// C++ typeop.cc:225 guard — a *plain* (non-enum) pointer dereferenced at a
    /// mismatched size must still decline (no enum, no relptr exact-piece).
    #[test]
    fn w10_from_pointer_plain_size_mismatch_still_declines() {
        let f = enum_factory();
        let int4t = f.get_base(4, type_metatype::TYPE_INT).unwrap();
        let ptr = f.get_type_pointer(8, int4t, 1).unwrap();
        assert!(
            propagate_from_pointer(&f, ptr, 1).is_none(),
            "non-enum size mismatch declines (only enums propagate here)"
        );
    }

    /// C++ typeop.cc:195-197 — propagateToPointer of a TYPE_PARTIALSTRUCT routes
    /// through getComponentForPtr: an element-aligned partial of an array yields
    /// a pointer to the *element*, not to the partial.
    #[test]
    fn w10_to_pointer_partialstruct_uses_component_for_ptr() {
        let f = factory();
        let int4t = f.get_base(4, type_metatype::TYPE_INT).unwrap();
        // array of 3 int4 (align_size of element = 4); a partial at offset 0 of
        // a whole element should resolve to the int4 element type.
        let arr = f.get_type_array(3, Rc::clone(&int4t)).unwrap();
        let partial = f.get_type_partial_struct(Rc::clone(&arr), 0, 4).unwrap();
        assert_eq!(partial.get_metatype(), type_metatype::TYPE_PARTIALSTRUCT);
        // getComponentForPtr should pick the element type (offset 0 % 4 == 0).
        let comp = partial.get_component_for_ptr().expect("component");
        assert!(
            Rc::ptr_eq(&comp, &int4t),
            "element-aligned partial of an array maps to the element type"
        );
        let out = propagate_to_pointer(&f, partial, 8, 1).expect("pointer");
        assert_eq!(out.get_metatype(), type_metatype::TYPE_PTR);
        let pointee = out.get_ptr_to().unwrap();
        assert!(
            Rc::ptr_eq(&pointee, &int4t),
            "the produced pointer points at the array element, not the partial"
        );
    }

    /// C++ TypePointer::downChain (type.cc:1247-1250) only writes `par`/`parOff`
    /// when the pointed-to type is a STRUCT/ARRAY. propagateAddIn2Out's do-while
    /// loop overwrites parent every iteration, so the *last* downChain call's
    /// parent wins.  Pin the single-level case the override depends on: a pointer
    /// into the middle of a struct hands back (parent=struct-ptr, parentOff=off).
    #[test]
    fn w10_downchain_struct_sets_parent_and_offset() {
        let f = factory();
        let int4t = f.get_base(4, type_metatype::TYPE_INT).unwrap();
        let st = f.get_type_struct("two").unwrap();
        let fields = vec![
            crate::dtype::TypeField::new(0, 0, "a", Rc::clone(&int4t)),
            crate::dtype::TypeField::new(1, 4, "b", Rc::clone(&int4t)),
        ];
        let st = f.assign_raw_fields_struct(&st, fields, Vec::new()).unwrap();
        let ptr = f.get_type_pointer(8, Rc::clone(&st), 1).unwrap();
        // downChain into offset 4 (the second field): parent must be the struct
        // pointer, parentOff the entry offset (4), result a pointer to field `b`.
        let (next, off, parent, par_off) = f.down_chain(&ptr, 4, true).expect("downchain");
        let parent = parent.expect("struct sets parent");
        assert_eq!(parent.get_metatype(), type_metatype::TYPE_PTR);
        assert!(
            Rc::ptr_eq(&parent.get_ptr_to().unwrap(), &st),
            "parent is the pointer to the containing struct"
        );
        assert_eq!(par_off, 4, "parentOff is the byte offset of entry");
        assert_eq!(off, 0, "landed exactly on field b");
        let next = next.expect("sub-pointer");
        assert!(Rc::ptr_eq(&next.get_ptr_to().unwrap(), &int4t));
    }

    /// downChain on a *scalar* pointee must NOT set a parent (C++ only the
    /// STRUCT/ARRAY arm writes `par`).  This pins the precondition under which
    /// the loop's unconditional `parent = par` overwrite is safe: a terminal
    /// scalar descent reports parent=None, matching C++ leaving the ref untouched
    /// from a prior struct hit only when the scalar is the *same* iteration.
    #[test]
    fn w10_downchain_scalar_pointee_reports_no_parent() {
        let f = factory();
        let int4t = f.get_base(4, type_metatype::TYPE_INT).unwrap();
        let ptr = f.get_type_pointer(8, int4t, 1).unwrap();
        let (_next, _off, parent, par_off) = f.down_chain(&ptr, 0, true).expect("downchain");
        assert!(parent.is_none(), "scalar pointee never sets a parent");
        assert_eq!(par_off, 0);
    }

    /// FINDING F1 fix (multi-iteration parent retention): C++ `propagateAddIn2Out`'s
    /// do-while passes `par`/`parOff` by reference, and `TypePointer::downChain`
    /// writes them ONLY on the STRUCT/ARRAY arm (type.cc:1247-1250); a later
    /// scalar iteration leaves the prior struct parent intact (the ref is never
    /// reset).  The fixed Rust loop mirrors this by overwriting `parent`/`parent_off`
    /// only when `par.is_some()`, so a scalar tail iteration (which returns None)
    /// must NOT clear a struct container set by an earlier iteration.
    ///
    /// This test replays the exact loop body of `propagate_add_in2_out` for a
    /// pointer into the *middle of a scalar field of a struct* (`&s.a + 2`,
    /// `a` an int4 at offset 0) and shows the loop now ends with the struct
    /// pointer retained, feeding `getTypePointerRel` to produce a TYPE_PTRREL.
    #[test]
    fn w10_downchain_loop_retains_struct_parent_on_scalar_tail() {
        let f = factory();
        let int4t = f.get_base(4, type_metatype::TYPE_INT).unwrap();
        let st = f.get_type_struct("s").unwrap();
        let fields =
            vec![crate::dtype::TypeField::new(0, 0, "a", Rc::clone(&int4t))];
        let st = f.assign_raw_fields_struct(&st, fields, Vec::new()).unwrap();
        let ptr = f.get_type_pointer(8, Rc::clone(&st), 1).unwrap();

        // Exact replica of the FIXED propagate_add_in2_out do-while (allow_wrap=true,
        // type_offset=2 == 2 bytes into the int4 field).
        let mut type_offset: int8 = 2;
        let mut pointer: Option<Rc<Datatype>> = Some(Rc::clone(&ptr));
        let mut parent: Option<Rc<Datatype>> = None;
        let mut parent_off: int8 = 0;
        let mut iters = 0;
        while let Some(cur) = pointer.clone() {
            let (next, new_off, par, par_off) = f.down_chain(&cur, type_offset, true).unwrap();
            type_offset = new_off;
            // The fix under test: only overwrite when a container is present, so a
            // scalar tail does not clobber the struct seen on the first iteration.
            if par.is_some() {
                parent = par;
                parent_off = par_off;
            }
            pointer = next;
            iters += 1;
            if pointer.is_none() || type_offset == 0 {
                break;
            }
        }
        assert!(iters >= 2, "must take the multi-iteration path (struct then scalar)");
        // After the fix: parent is the struct pointer from iter1, retained across
        // the scalar tail iteration (which returned par=None) — matching C++.
        let parent = parent.expect("struct container retained across scalar tail");
        assert_eq!(parent.get_metatype(), type_metatype::TYPE_PTR);
        assert!(
            Rc::ptr_eq(&parent.get_ptr_to().unwrap(), &st),
            "retained parent is the pointer to the containing struct"
        );
        assert_eq!(parent_off, 2, "parentOff is the byte offset into the struct");

        // The container, fed to getTypePointerRel, yields a struct-relative pointer
        // preserving the container — exactly the C++ wrap on this edge.  C++
        // `TypePointerRel` keeps `metatype == TYPE_PTR` internally (type.cc:3010,
        // "Don't use TYPE_PTRREL internally"); the relative identity is the
        // `is_pointer_rel()`/parent surface, not the metatype.
        let pt = match &pointer {
            None => f.get_base(1, type_metatype::TYPE_UNKNOWN).unwrap(),
            Some(p) => p.get_ptr_to().unwrap(),
        };
        // cast: int8->int4 narrow matching C++ getTypePointerRel(int4 off).
        let rel = f
            .get_type_pointer_rel(parent, pt, parent_off as int4)
            .expect("relative pointer");
        assert_eq!(
            rel.get_metatype(),
            type_metatype::TYPE_PTR,
            "a relative pointer reports TYPE_PTR internally (TYPE_PTRREL is the marshalling override)"
        );
        assert!(
            rel.is_pointer_rel(),
            "interior-of-scalar-field-of-struct yields a struct-relative pointer (is_pointer_rel)"
        );
    }

    // ====================================================================
    // Round-2 verifier adversarial tests (w10-type-propagation, F1 fix).
    // These probe the *opposite* failure mode of the F1 fix: the
    // `if par.is_some()` guard must not OVER-retain — a later STRUCT/ARRAY
    // container must replace an earlier one (C++ writes the ref every time
    // it hits the STRUCT/ARRAY arm, so the LAST container wins), and a chain
    // with NO container at all must leave parent None (no spurious PTRREL).
    // ====================================================================

    /// F1-fix boundary: STRUCT whose field is an inline ARRAY of int4.  In C++
    /// `downChain`, the struct arm sets `par=struct,parOff=off`, `getSubType`
    /// returns the ARRAY field, and because `!isArray` it goes through
    /// `getTypePointerStripArray` — which STRIPS the array to a pointer-to-int4
    /// element (type.cc:1256, 4323-4332).  So the array never surfaces as a
    /// standalone `par`; iter2 is a scalar int4 tail returning `par=None`.  The
    /// retained container is therefore the STRUCT (parOff = the byte offset),
    /// and the F1 fix must keep it across the scalar tail — NOT clear it.  This
    /// pins that the strip-array path does not accidentally bury the struct
    /// container, and that the fix retains it.
    #[test]
    fn w10_r2_downchain_struct_with_array_field_retains_struct_across_strip() {
        let f = factory();
        let int4t = f.get_base(4, type_metatype::TYPE_INT).unwrap();
        let arr = f.get_type_array(3, Rc::clone(&int4t)).unwrap(); // int4[3], size 12
        let st = f.get_type_struct("sa").unwrap();
        let fields = vec![crate::dtype::TypeField::new(0, 0, "arr", Rc::clone(&arr))];
        let st = f.assign_raw_fields_struct(&st, fields, Vec::new()).unwrap();
        let ptr = f.get_type_pointer(8, Rc::clone(&st), 1).unwrap();

        // Replay the FIXED loop body for byte offset 4 (== &s.arr[1]).
        let mut type_offset: int8 = 4;
        let mut pointer: Option<Rc<Datatype>> = Some(Rc::clone(&ptr));
        let mut parent: Option<Rc<Datatype>> = None;
        let mut parent_off: int8 = 0;
        let mut saw_struct_parent = false;
        let mut saw_scalar_tail_none = false;
        let mut iters = 0;
        while let Some(cur) = pointer.clone() {
            let (next, new_off, par, par_off) = f.down_chain(&cur, type_offset, true).unwrap();
            type_offset = new_off;
            match &par {
                Some(p) if Rc::ptr_eq(&p.get_ptr_to().unwrap(), &st) => saw_struct_parent = true,
                None if iters >= 1 => saw_scalar_tail_none = true,
                _ => {}
            }
            // The element reached after the struct must be the STRIPPED int4
            // element, never a pointer-to-array (proves getTypePointerStripArray ran).
            if iters == 0 {
                assert_eq!(
                    next.as_ref().unwrap().get_ptr_to().unwrap().get_metatype(),
                    type_metatype::TYPE_INT,
                    "struct's array field is stripped to a pointer-to-element on descent"
                );
            }
            if par.is_some() {
                parent = par;
                parent_off = par_off;
            }
            pointer = next;
            iters += 1;
            if pointer.is_none() || type_offset == 0 {
                break;
            }
        }
        assert!(iters >= 2, "struct descent then scalar-element tail");
        assert!(saw_struct_parent, "iter1 produced the struct container");
        assert!(saw_scalar_tail_none, "scalar tail iteration returned par=None");
        let parent = parent.expect("struct container retained across the scalar tail");
        // The retained container is the STRUCT (the array was stripped, so it
        // never became a `par`); the fix kept it across the par=None tail.
        assert!(
            Rc::ptr_eq(&parent.get_ptr_to().unwrap(), &st),
            "retained parent is the struct, kept across the scalar tail (F1 fix)"
        );
        assert_eq!(parent_off, 4, "parentOff is the byte offset into the struct");
    }

    /// F1-fix negative: a pointer into a pure SCALAR (no struct/array anywhere
    /// in the descent) must leave `parent == None` after the loop, so the
    /// post-loop `if let Some(par)` block is SKIPPED and no spurious TYPE_PTRREL
    /// is produced.  This guards against the fix accidentally seeding a parent.
    #[test]
    fn w10_r2_downchain_pure_scalar_leaves_no_parent() {
        let f = factory();
        // 8-byte scalar pointed at; offset 4 wraps within the scalar (allow_wrap).
        let int8t = f.get_base(8, type_metatype::TYPE_INT).unwrap();
        let ptr = f.get_type_pointer(8, int8t, 1).unwrap();

        let mut type_offset: int8 = 4;
        let mut pointer: Option<Rc<Datatype>> = Some(Rc::clone(&ptr));
        let mut parent: Option<Rc<Datatype>> = None;
        let mut iters = 0;
        while let Some(cur) = pointer.clone() {
            let (next, new_off, par, par_off) = f.down_chain(&cur, type_offset, true).unwrap();
            type_offset = new_off;
            if par.is_some() {
                parent = par;
                let _ = par_off;
            }
            pointer = next;
            iters += 1;
            if pointer.is_none() || type_offset == 0 || iters > 8 {
                break;
            }
        }
        assert!(
            parent.is_none(),
            "no struct/array anywhere -> parent stays None -> post-loop rel-wrap is skipped"
        );
    }

    /// `get_component_for_ptr` (the PARTIALSTRUCT arm of propagate_to_pointer):
    /// the array-element fast path is gated on `offset % eltype.alignSize == 0`.
    /// At a MISALIGNED offset (2, into the middle of an int4 element) the C++
    /// returns `stripped`, NOT the element type.  This pins the gate so a
    /// shortcut that always returns the element type would fail.
    #[test]
    fn w10_r2_component_for_ptr_misaligned_offset_returns_stripped_not_element() {
        let f = factory();
        let int4t = f.get_base(4, type_metatype::TYPE_INT).unwrap();
        let arr = f.get_type_array(3, Rc::clone(&int4t)).unwrap();
        // partial struct over the array, offset 2 (misaligned within int4 element).
        let partial_mis = f.get_type_partial_struct(Rc::clone(&arr), 2, 4).unwrap();
        let comp_mis = partial_mis
            .get_component_for_ptr()
            .expect("partial-struct yields a component");
        assert_ne!(
            comp_mis.get_metatype(),
            type_metatype::TYPE_INT,
            "misaligned offset must NOT take the element fast-path; returns stripped"
        );
        // Aligned offset 0 DOES take the element fast-path -> the int4 element.
        let partial_aln = f.get_type_partial_struct(Rc::clone(&arr), 0, 4).unwrap();
        let comp_aln = partial_aln.get_component_for_ptr().expect("component");
        assert!(
            Rc::ptr_eq(&comp_aln, &int4t),
            "aligned element-boundary offset returns the array element type"
        );
    }
}
