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
//! The full metatype lattice (COPY, MULTIEQUAL, INDIRECT, every comparison,
//! ZEXT/SEXT, the non-pointer arithmetic, and the spacebase pointer construction)
//! is transcribed.  The composite/pointer *resize* sub-cases of PIECE/SUBPIECE
//! (`resizePointer`/`findTruncation`) and the structure `downChain` of the ADD
//! pointer walk are deep TypeFactory surfaces; where the supporting machinery is
//! not yet exposed those arms conservatively decline to propagate (return `None`),
//! which is faithful — the C++ also returns null along those edges when the
//! transform doesn't apply.

use std::rc::Rc;

use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;
use crate::seams::{OpId, VarnodeId};
use crate::typeop::type_op_info;

/// Resolve `op->outputTypeLocal()` / `op->inputTypeLocal(slot)` faithfully via the
/// W6 typeop inst table (`getOutputLocal`/`getInputLocal`).  Falls back to a
/// size-correct `TYPE_UNKNOWN` if the factory query fails (the C++ getBase never
/// fails for a valid size).
fn output_type_local(data: &Funcdata, op: OpId) -> Rc<Datatype> {
    let arch = Rc::clone(data.get_arch());
    let o = data.obank().get(op).expect("output_type_local: stale op");
    let out_size = o
        .get_out()
        .and_then(|v| data.vbank().get(v))
        .map(|v| v.get_size())
        .unwrap_or(1);
    let info = type_op_info(o.code());
    match arch.types() {
        Some(tlst) => info
            .get_output_local(tlst, out_size)
            .unwrap_or_else(|_| Rc::new(Datatype::new(out_size, type_metatype::TYPE_UNKNOWN))),
        None => Rc::new(Datatype::new(out_size, type_metatype::TYPE_UNKNOWN)),
    }
}

fn input_type_local(data: &Funcdata, op: OpId, slot: int4) -> Rc<Datatype> {
    let arch = Rc::clone(data.get_arch());
    let o = data.obank().get(op).expect("input_type_local: stale op");
    let in_size = o
        .get_in(slot)
        .and_then(|v| data.vbank().get(v))
        .map(|v| v.get_size())
        .unwrap_or(1);
    let info = type_op_info(o.code());
    match arch.types() {
        Some(tlst) => info
            .get_input_local(tlst, slot, in_size)
            .unwrap_or_else(|_| Rc::new(Datatype::new(in_size, type_metatype::TYPE_UNKNOWN))),
        None => Rc::new(Datatype::new(in_size, type_metatype::TYPE_UNKNOWN)),
    }
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
        let (ct, needs_block) = get_local_type(data, vn);
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
    // alttype = invn->getTempType().  (needsResolution()/resolveInFlow is a W8
    // union-resolution surface; the common, non-union path is alttype = temptype.)
    let alttype = match data.vbank().get(invn).and_then(|v| v.get_temp_type().cloned()) {
        Some(t) => t,
        None => return false,
    };

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
        // Comparisons: propagateAcrossCompare (typeop.cc:965).  Must be across the
        // two inputs (in/out both >= 0... in C++ both != -1).
        OpCode::CPUI_INT_EQUAL
        | OpCode::CPUI_INT_NOTEQUAL
        | OpCode::CPUI_INT_LESS
        | OpCode::CPUI_INT_LESSEQUAL
        | OpCode::CPUI_INT_SLESS
        | OpCode::CPUI_INT_SLESSEQUAL => {
            propagate_across_compare(data, alttype, invn_is_spacebase, outvn, inslot, outslot)
        }
        // TypeOpIntAdd: pointer add / constant-folded int (typeop.cc:1183).
        OpCode::CPUI_INT_ADD => propagate_int_add(data, alttype, op, outvn, inslot, outslot),
        _ => None, // default TypeOp::propagateType: don't propagate
    }
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
    // isPointerRel struct mid-pointer: don't propagate across (give the other side
    // a chance to type from the structure pointer).  The non-relptr path is the
    // identity propagation.
    if alttype.is_pointer_rel() {
        let outvn_const = data.vbank().get(outvn).map(|v| v.is_constant()).unwrap_or(false);
        if !outvn_const {
            // C++ checks parent metatype == STRUCT && byteOffset >= 0.  The
            // byte-offset accessor on a relptr is a W8 surface; conservatively
            // keep the relptr identity (the common case for non-struct relptrs).
            return Some(alttype);
        }
    }
    Some(alttype)
}

/// C++ `TypeOpIntAdd::propagateType` + `propagateAddIn2Out` (typeop.cc:1183/1217).
/// The non-pointer int/uint constant-fold arm is transcribed; the pointer
/// `downChain` walk routes through the existing `Datatype::down_chain` machinery.
fn propagate_int_add(
    data: &mut Funcdata,
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

/// C++ `TypeOpIntAdd::propagateAddIn2Out` (typeop.cc:1217), the pointer-add
/// transform, via the existing `Datatype::down_chain`.  The
/// `propagateAddPointer` command/`getExtraTypeOffset` heuristics are a deep W8
/// surface; the common single-constant-add path is handled by `down_chain` and
/// the non-applying edges decline (return `None`), matching the C++ null result.
fn propagate_add_in2_out(
    data: &mut Funcdata,
    alttype: Rc<Datatype>,
    op: OpId,
    inslot: int4,
) -> Option<Rc<Datatype>> {
    // Only the constant-offset ADD (slot 1 constant) is the recoverable common
    // case; otherwise decline (the full `propagateAddPointer` command analysis is
    // not yet ported — faithful no-propagation along those edges).
    let in1 = data.obank().get(op)?.get_in(1)?;
    let (off, in1_const) = data
        .vbank()
        .get(in1)
        .map(|v| (v.get_offset(), v.is_constant()))
        .unwrap_or((0, false));
    if !in1_const || inslot != 0 {
        return None;
    }
    let arch = Rc::clone(data.get_arch());
    let tlst = arch.types_impl()?;
    let ws = alttype.get_word_size().unwrap_or(1);
    let type_offset =
        kuna_base::space::AddrSpace::address_to_byte_int(off as i64, ws);
    if type_offset == 0 {
        return Some(alttype);
    }
    let (pointer, _off, parent, _poff) =
        tlst.down_chain(&alttype, type_offset, true).ok()?;
    // If we landed on a proper sub-pointer keep it; if down_chain bottomed out but
    // there is a containing parent, the C++ wraps it as a partial — that wrap is a
    // W8 surface, so decline rather than guess.
    if parent.is_some() {
        return None;
    }
    pointer
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
    // data.getScopeLocal()->applyTypeRecommendations() — W4 scope surface; the
    // local scope carries no type recommendations in the merged tree (no-op).
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
    // propagateSpacebaseRef: needs findSpacebaseInput(scopeLocal->getSpaceId());
    // the spacebase-input/scope-space surface is a W4/W8 path absent here, so this
    // pointer-alias propagation is skipped (faithful: no spacebase ADD aliases to
    // propagate in the merged-tree slice).
    write_back(data)
}
