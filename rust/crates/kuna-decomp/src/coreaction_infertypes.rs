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

use kuna_base::address::calc_mask;
use kuna_base::space::AddrSpace;
use kuna_base::types::{int4, int8, uintb};
use kuna_num::opcodes::OpCode;

use crate::dtype::{type_metatype, Datatype, TypeFactory};
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

/// C++ `getSpaceFromConst(op->getIn(0))`: the LOAD/STORE/SEGMENT space operand is
/// a constant whose offset is the address-space manager index (LOSS-015 model).
fn space_from_const(data: &Funcdata, op: OpId, slot: int4) -> Option<Rc<AddrSpace>> {
    let cvn = data.obank().get(op)?.get_in(slot)?;
    let idx = data.vbank().get(cvn)?.get_offset();
    let manage = data.get_arch().manage();
    if idx >= manage.num_spaces() as u64 {
        return None;
    }
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
        let mut type_offset = AddrSpace::address_to_byte_int(offset as i64, word_size);
        let op_code = data.obank().get(op)?.code();
        let allow_wrap = op_code != OpCode::CPUI_PTRSUB;
        // C++ do { pointer = pointer->downChain(typeOffset,...); if (0) break; }
        //      while(typeOffset != 0);
        while let Some(cur) = pointer.clone() {
            let (next, new_off, par, par_off) =
                tlst.down_chain(&cur, type_offset, allow_wrap).ok()?;
            type_offset = new_off;
            // downChain overwrites parent/parentOff each call (only the last is used).
            parent = par;
            parent_off = par_off;
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
}
