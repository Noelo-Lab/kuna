//! `Funcdata::resolveInFlow` — the **driver** that runs [`ScoreUnionFields`] and
//! writes the resolution into the per-function union cache (W10 item
//! `w10-union-render`).
//!
//! In C++ `resolveInFlow` is a virtual on `Datatype` (`type.cc:578-2986`); the
//! `TypePointer`/`TypeArray`/`TypeStruct`/`TypeUnion`/`TypePartialUnion` overrides
//! each reach the per-function cache through
//! `op->getParent()->getFuncdata()->getUnionField(this,op,slot)` and, on a miss,
//! build a [`ScoreUnionFields`] and call `setUnionField`.  A `Datatype` in the
//! Rust port carries no `Funcdata` back-pointer (ADR-0001: interned arena values),
//! so — exactly like [`Funcdata::find_resolve_facing`](crate::funcdata_facing) —
//! the whole `resolveInFlow` dispatch lives here on `Funcdata`, which owns both the
//! `union_map` cache ([`crate::funcdata_union`]) and the `Funcdata`/`Varnode`/
//! `PcodeOp` graph the scorer walks.  Every call site reaches `resolveInFlow` only
//! with an `op` in hand whose parent's `Funcdata` is exactly the `self` here.
//!
//! Each arm is transcribed statement-for-statement from the cited C++ override.
//! The seam-free leaf scorer is [`crate::unionresolve_run::ScoreUnionFields`].

use std::rc::Rc;

use kuna_base::error::{KunaError, KunaResult};
use kuna_base::types::{int4, int8};
use kuna_num::opcodes::OpCode;

use crate::dtype::{type_metatype, Datatype, DatatypeKind, TypeFactory};
use crate::funcdata::Funcdata;
use crate::context::OpId;
use crate::unionresolve::ResolvedUnion;
use crate::unionresolve_run::ScoreUnionFields;

impl Funcdata {
    /// `Datatype::resolveInFlow(op, slot)` dispatched against the live union
    /// scorer + cache (C++ `type.cc:578` base + the `TypePointer`/`TypeArray`/
    /// `TypeStruct`/`TypeUnion`/`TypePartialUnion` overrides).  `self` stands in
    /// for `op->getParent()->getFuncdata()`.
    ///
    /// Returns the resolved data-type (the field type, or the receiver itself when
    /// nothing better fits), and — as a side effect — populates the `union_map`
    /// cache so the print-time facing lookups
    /// ([`Funcdata::find_resolve_facing`](crate::funcdata_facing) and the printc
    /// `opPtrsub` union arm) find the resolved field.
    ///
    /// Only types for which `needsResolution()` is set ever reach the scoring arms;
    /// the cast plane (`ActionSetCasts::resolveUnion`) gates on `needsResolution()`
    /// before calling, but the base `return this` arm is kept so a non-resolution
    /// type is a no-op pass-through (the C++ `Datatype::resolveInFlow` body).
    pub fn resolve_in_flow(&mut self, ct: &Rc<Datatype>, op: OpId, slot: int4) -> KunaResult<Rc<Datatype>> {
        match &ct.kind {
            // TypePointer::resolveInFlow (type.cc:1314-1333).
            //
            // `TypePointerRel : public TypePointer` (type.hh:724) does NOT override
            // `resolveInFlow`, so a relative-pointer-to-union dispatches the
            // inherited `TypePointer::resolveInFlow`.  Match both kinds.
            DatatypeKind::Pointer { ptrto, .. } | DatatypeKind::PointerRel { ptrto, .. } => {
                if ptrto.get_metatype() == type_metatype::TYPE_UNION {
                    self.resolve_in_flow_union_like(ct, op, slot)
                } else {
                    // return this;
                    Ok(Rc::clone(ct))
                }
            }
            // TypeUnion::resolveInFlow (type.cc:2541-2556).
            DatatypeKind::Union { .. } => self.resolve_in_flow_union_like(ct, op, slot),
            // TypeArray::resolveInFlow (type.cc:1455-1468) /
            // TypeStruct::resolveInFlow (type.cc:2275-2288): score a single
            // component, then cache a field resolution.
            DatatypeKind::Array { .. } | DatatypeKind::Struct { .. } => {
                self.resolve_in_flow_single_component(ct, op, slot)
            }
            // TypePartialUnion::resolveInFlow (type.cc:2938-2976).
            DatatypeKind::PartialUnion { .. } => self.resolve_in_flow_partial(ct, op, slot),
            // Datatype::resolveInFlow base (type.cc:578-581): return this.
            _ => Ok(Rc::clone(ct)),
        }
    }

    /// The shared body of `TypePointer::resolveInFlow` (pointer-to-union) and
    /// `TypeUnion::resolveInFlow` (type.cc:1314-1333 / 2541-2556): cache, then
    /// address-based cache, then [`ScoreUnionFields::new_edge`].
    fn resolve_in_flow_union_like(
        &mut self,
        ct: &Rc<Datatype>,
        op: OpId,
        slot: int4,
    ) -> KunaResult<Rc<Datatype>> {
        // const ResolvedUnion *res = fd->getUnionField(this,op,slot);
        // if (res != 0) return res->getDatatype();
        if let Some(res) = self.get_union_field(ct, op, slot) {
            return Ok(Rc::clone(res.get_datatype()));
        }
        // res = fd->getAddressBasedUnionField(this, op->getAddr(), slot);
        let op_addr = self.obank().get(op).map(|o| o.get_addr().clone());
        if let Some(addr) = &op_addr {
            // The address-based hit stores a *new* ResolvedUnion(this, fieldNum)
            // and returns its data-type (C++ builds `resolve` then caches `*res`
            // for the union arm / `resolve` for the pointer arm — both store the
            // same field number; we rebuild a fresh resolution off `this` so the
            // cached resolve data-type matches `this`'s size).
            let field_num = self.get_address_based_union_field(ct, addr, slot).map(|r| r.get_field_num());
            if let Some(field_num) = field_num {
                let typegrp = self.get_arch().types_rc().ok_or_else(|| {
                    KunaError::lowlevel("resolveInFlow: TypeFactory unavailable")
                })?;
                let resolve = ResolvedUnion::new_field(Rc::clone(ct), field_num, typegrp.as_ref())?;
                let dt = Rc::clone(resolve.get_datatype());
                // fd->setUnionField(this, op, slot, *res / resolve);
                self.set_union_field(ct, op, slot, resolve);
                return Ok(dt);
            }
        }
        // ScoreUnionFields scoreFields(*fd,this,op,slot);
        // fd->setUnionField(this,op,slot,scoreFields.getResult());
        // return scoreFields.getResult().getDatatype();
        let typegrp = self.get_arch().types_rc().ok_or_else(|| {
            KunaError::lowlevel("resolveInFlow: TypeFactory unavailable")
        })?;
        let result = {
            let scorer = ScoreUnionFields::new_edge(self, typegrp.as_ref(), Rc::clone(ct), op, slot)?;
            scorer.into_result()
        };
        let dt = Rc::clone(result.get_datatype());
        self.set_union_field(ct, op, slot, result);
        Ok(dt)
    }

    /// `TypeArray::resolveInFlow` / `TypeStruct::resolveInFlow`
    /// (type.cc:1455-1468 / 2275-2288): cache, else score a single component and
    /// cache a field (or whole-container, `fieldNum == -1`) resolution.
    fn resolve_in_flow_single_component(
        &mut self,
        ct: &Rc<Datatype>,
        op: OpId,
        slot: int4,
    ) -> KunaResult<Rc<Datatype>> {
        if let Some(res) = self.get_union_field(ct, op, slot) {
            return Ok(Rc::clone(res.get_datatype()));
        }
        // int4 fieldNum = (TypeStruct::)scoreSingleComponent(this,op,slot);
        let field_num = self.score_single_component(ct, op, slot)?;
        // ResolvedUnion compFill(this,fieldNum,*fd->getArch()->types);
        let typegrp = self.get_arch().types_rc().ok_or_else(|| {
            KunaError::lowlevel("resolveInFlow: TypeFactory unavailable")
        })?;
        let comp_fill = ResolvedUnion::new_field(Rc::clone(ct), field_num, typegrp.as_ref())?;
        let dt = Rc::clone(comp_fill.get_datatype());
        // fd->setUnionField(this, op, slot, compFill);
        self.set_union_field(ct, op, slot, comp_fill);
        Ok(dt)
    }

    /// C++ `TypeStruct::scoreSingleComponent` (type.cc:2239-2273): decide whether a
    /// struct/array use refers to the whole container (`-1`) or a component (`0`).
    fn score_single_component(
        &self,
        parent: &Rc<Datatype>,
        op: OpId,
        slot: int4,
    ) -> KunaResult<int4> {
        let o = match self.obank().get(op) {
            Some(o) => o,
            None => return Ok(0),
        };
        let opc = o.code();
        if opc == OpCode::CPUI_COPY || opc == OpCode::CPUI_INDIRECT {
            // Varnode *vn = (slot == 0) ? op->getOut() : op->getIn(0);
            let vn = if slot == 0 { o.get_out() } else { o.get_in(0) };
            if let Some(vn) = vn {
                let v = self.vbank().get(vn);
                let is_lock = v.map(|v| v.is_type_lock()).unwrap_or(false);
                // if (vn->isTypeLock() && vn->getType() == parent) return -1;
                if is_lock && v.map(|v| Rc::ptr_eq(v.get_type(), parent)).unwrap_or(false) {
                    return Ok(-1); // COPY of the structure directly, use whole structure
                }
            }
        } else if (opc == OpCode::CPUI_LOAD && slot == -1)
            || (opc == OpCode::CPUI_STORE && slot == 2)
        {
            // Varnode *vn = op->getIn(1);
            if let Some(vn) = o.get_in(1) {
                let is_lock = self.vbank().get(vn).map(|v| v.is_type_lock()).unwrap_or(false);
                if is_lock {
                    // Datatype *ct = vn->getTypeReadFacing(op);
                    let read = self.vn_type_read_facing(vn, op);
                    // if (ct->getMetatype()==TYPE_PTR && ((TypePointer*)ct)->getPtrTo()==parent) return -1;
                    if read.get_metatype() == type_metatype::TYPE_PTR
                        && read.get_ptr_to().map(|p| Rc::ptr_eq(&p, parent)).unwrap_or(false)
                    {
                        return Ok(-1); // LOAD/STORE of the structure directly, use whole structure
                    }
                }
            }
        } else if o.is_call() {
            // FuncCallSpecs *fc = fd->getCallSpecs(op);
            if let Some(idx) = self.get_call_specs_index(op) {
                let fc = self.get_call_specs(idx);
                let proto = fc.proto();
                // ProtoParameter *param = (slot>=1 && isInputLocked) getParam(slot-1)
                //                        : (slot<0 && isOutputLocked) getOutput();
                let param_type: Option<Rc<Datatype>> = if slot >= 1 && proto.is_input_locked() {
                    proto.get_param(slot - 1).and_then(|p| p.get_type().cloned())
                } else if slot < 0 && proto.is_output_locked() {
                    proto.get_output_type().cloned()
                } else {
                    None
                };
                // if (param != 0 && param->getType() == parent) return -1;
                if let Some(pt) = param_type {
                    if Rc::ptr_eq(&pt, parent) {
                        return Ok(-1); // Function signature refers to parent directly
                    }
                }
            }
        }
        Ok(0) // In all other cases resolve to the component
    }

    /// C++ `TypePartialUnion::resolveInFlow` (type.cc:2938-2976): walk the
    /// container/offset chain, resolving truncations through the parent union, then
    /// `updateUnionField`.
    ///
    /// The truncation walk reaches `TypeUnion::resolveTruncation` (type.cc:2569),
    /// which is itself a scorer-driving surface (constructors 2/3 of
    /// [`ScoreUnionFields`]); it is wired through [`Funcdata::resolve_truncation`]
    /// below.  Each container hop pulls the resolved field's exact piece
    /// (`glb->types->getExactPiece(field->type, curOff, size)`); when the walk does
    /// not land on a field of the right size, `curType` falls to `stripped` (the
    /// C++ "Should never reach here / size mismatch" fallback, type.cc:2971-2973).
    fn resolve_in_flow_partial(
        &mut self,
        ct: &Rc<Datatype>,
        op: OpId,
        slot: int4,
    ) -> KunaResult<Rc<Datatype>> {
        // const ResolvedUnion *res = fd->getUnionField(this, op, slot);
        // if (res != 0) return res->getDatatype();
        if let Some(res) = self.get_union_field(ct, op, slot) {
            return Ok(Rc::clone(res.get_datatype()));
        }
        // Datatype *curType = container; int8 curOff = offset;
        let size = ct.get_size();
        let mut cur_type: Option<Rc<Datatype>> = ct.get_partial_base(); // container
        let mut cur_off: int8 = ct.get_partial_offset().unwrap_or(0) as int8;
        // while(curType != 0 && curType->getSize() > size)
        while let Some(cur) = cur_type.clone() {
            if cur.get_size() as int4 <= size {
                break;
            }
            let meta = cur.get_metatype();
            if meta == type_metatype::TYPE_PARTIALUNION {
                // curOff += curPartial->getOffset();
                cur_off += cur.get_partial_offset().unwrap_or(0) as int8;
                // field = curPartial->getParentUnion()->resolveTruncation(curOff,op,slot,newOff);
                let parent_union = cur.get_partial_base().ok_or_else(|| {
                    KunaError::lowlevel("resolveInFlow: partial union missing container")
                })?;
                let resolved = self.resolve_truncation(&parent_union, cur_off, op, slot)?;
                // curType = 0; if (field != 0) curType = getExactPiece(field->type, curOff, size);
                cur_type = None;
                if let Some((field_idx, _newoff)) = resolved {
                    if let Some(ftype) = parent_union.get_field(field_idx).map(|f| Rc::clone(&f.field_type)) {
                        let typegrp = self.get_arch().types_rc().ok_or_else(|| {
                            KunaError::lowlevel("resolveInFlow: TypeFactory unavailable")
                        })?;
                        cur_type = typegrp.get_exact_piece(ftype, cur_off as int4, size)?;
                    }
                }
                cur_off = 0;
            } else if meta == type_metatype::TYPE_UNION {
                // field = curType->resolveTruncation(curOff,op,slot,curOff);
                let resolved = self.resolve_truncation(&cur, cur_off, op, slot)?;
                cur_type = None;
                if let Some((field_idx, newoff)) = resolved {
                    cur_off = newoff; // resolveTruncation passes back curOff by &ref
                    if let Some(ftype) = cur.get_field(field_idx).map(|f| Rc::clone(&f.field_type)) {
                        let typegrp = self.get_arch().types_rc().ok_or_else(|| {
                            KunaError::lowlevel("resolveInFlow: TypeFactory unavailable")
                        })?;
                        cur_type = typegrp.get_exact_piece(ftype, cur_off as int4, size)?;
                    }
                }
                cur_off = 0;
            } else {
                // Should never reach here
                cur_type = None;
                break;
            }
        }
        // if (curType == 0 || curType->getSize() != size) curType = stripped;
        let cur_type = match cur_type {
            Some(c) if c.get_size() as int4 == size => c,
            _ => ct.get_stripped().ok_or_else(|| {
                KunaError::lowlevel("resolveInFlow: partial union missing stripped")
            })?,
        };
        // fd->updateUnionField(this, op, slot, curType);
        self.update_union_field(ct, op, slot, Rc::clone(&cur_type));
        Ok(cur_type)
    }

    /// `Datatype::resolveTruncation(offset, op, slot, &newoff)` dispatched against
    /// the live union scorer + cache, for the `TypeUnion` and `TypePartialUnion`
    /// overrides (C++ `TypeUnion::resolveTruncation`, type.cc:2569-2605; and
    /// `TypePartialUnion::resolveTruncation`, type.cc:2994-2998, which delegates to
    /// the container).  `self` stands in for `op->getParent()->getFuncdata()`.
    ///
    /// Returns `Some((field_index, newoff))` — the union field the truncation
    /// resolves to and the residual byte offset into it — or `None` when no field
    /// fits.  A `Datatype` that is neither union nor partial-union resolves to
    /// `None` (the base/struct overrides do not score truncations through the
    /// union cache; `Datatype::find_truncation` covers the struct field walk).
    ///
    /// This **writes** the per-function `union_map` cache (`setUnionField`), exactly
    /// as the C++ override does, so the print-time `findTruncation` read finds the
    /// resolved field.
    pub fn resolve_truncation(
        &mut self,
        ct: &Rc<Datatype>,
        offset: int8,
        op: OpId,
        slot: int4,
    ) -> KunaResult<Option<(int4, int8)>> {
        match &ct.kind {
            // TypePartialUnion::resolveTruncation (type.cc:2994-2998):
            //   return container->resolveTruncation(off + offset, op, slot, newoff);
            DatatypeKind::PartialUnion { .. } => {
                let container = ct.get_partial_base().ok_or_else(|| {
                    KunaError::lowlevel("resolveTruncation: partial union missing container")
                })?;
                let part_off = ct.get_partial_offset().unwrap_or(0) as int8;
                self.resolve_truncation(&container, offset + part_off, op, slot)
            }
            // TypeUnion::resolveTruncation (type.cc:2569-2605).
            DatatypeKind::Union { .. } => self.resolve_truncation_union(ct, offset, op, slot),
            // Datatype::resolveTruncation base / non-union: no field.
            _ => Ok(None),
        }
    }

    /// `TypeUnion::resolveTruncation` (type.cc:2569-2605): the cache lookup, the
    /// address-based fallback, and — on a miss — the SUBPIECE artificial-slot-1
    /// scorer (constructor 2, [`ScoreUnionFields::new_subpiece`]) or the
    /// implied-truncation scorer (constructor 3,
    /// [`ScoreUnionFields::new_truncation`]).
    fn resolve_truncation_union(
        &mut self,
        ct: &Rc<Datatype>,
        offset: int8,
        op: OpId,
        slot: int4,
    ) -> KunaResult<Option<(int4, int8)>> {
        // const ResolvedUnion *res = fd->getUnionResolution(this, op, slot);
        // if (res == 0) { res = fd->getAddressBasedUnionField(...); if (res != 0) setUnionField(...); }
        let mut have_cached: Option<int4> = self.get_union_resolution(ct, op, slot).map(|r| r.get_field_num());
        if have_cached.is_none() {
            let op_addr = self.obank().get(op).map(|o| o.get_addr().clone());
            if let Some(addr) = &op_addr {
                if let Some(res) = self.get_address_based_union_field(ct, addr, slot) {
                    let res = res.clone();
                    have_cached = Some(res.get_field_num());
                    // fd->setUnionField(this, op, slot, *res);
                    self.set_union_field(ct, op, slot, res);
                }
            }
        }
        // if (res != 0) { if (res->getFieldNum() >= 0) { ...; return field; } }
        if let Some(field_num) = have_cached {
            if field_num >= 0 {
                let foff = ct.get_field(field_num).map(|f| f.offset).unwrap_or(0);
                let newoff = offset - foff as int8;
                return Ok(Some((field_num, newoff)));
            }
            // res != 0 but fieldNum < 0: C++ falls through to `return 0` (the
            // else-if/else only run when res == 0).
            return Ok(None);
        }
        // else if (op->code() == CPUI_SUBPIECE && slot == 1)  // artificial slot
        let opc = self.obank().get(op).map(|o| o.code()).unwrap_or(OpCode::CPUI_COPY);
        let typegrp = self.get_arch().types_rc().ok_or_else(|| {
            KunaError::lowlevel("resolveTruncation: TypeFactory unavailable")
        })?;
        if opc == OpCode::CPUI_SUBPIECE && slot == 1 {
            // ScoreUnionFields scoreFields(*fd, this, offset, op);  // constructor 2
            let result = {
                let scorer = ScoreUnionFields::new_subpiece(
                    self,
                    typegrp.as_ref(),
                    Rc::clone(ct),
                    offset as int4,
                    op,
                )?;
                scorer.into_result()
            };
            let field_num = result.get_field_num();
            // fd->setUnionField(this, op, slot, scoreFields.getResult());
            self.set_union_field(ct, op, slot, result);
            if field_num >= 0 {
                // newoff = 0; return getField(fieldNum);
                return Ok(Some((field_num, 0)));
            }
        } else {
            // ScoreUnionFields scoreFields(*fd, this, offset, op, slot);  // constructor 3
            let result = {
                let scorer = ScoreUnionFields::new_truncation(
                    self,
                    typegrp.as_ref(),
                    Rc::clone(ct),
                    offset as int4,
                    op,
                    slot,
                )?;
                scorer.into_result()
            };
            let field_num = result.get_field_num();
            self.set_union_field(ct, op, slot, result);
            if field_num >= 0 {
                let foff = ct.get_field(field_num).map(|f| f.offset).unwrap_or(0);
                let newoff = offset - foff as int8;
                return Ok(Some((field_num, newoff)));
            }
        }
        Ok(None)
    }

    /// `Datatype::findTruncation(off, sz, op, slot, &newoff)` dispatched against the
    /// live union cache, for the `TypeUnion`/`TypePartialUnion` overrides (C++
    /// `TypeUnion::findTruncation`, type.cc:2613-2627; `TypePartialUnion::
    /// findTruncation`, type.cc:2882-2884) plus the `TypeStruct::findTruncation`
    /// field walk (type.cc:1878-1892) for a plain struct/non-union receiver.
    ///
    /// Read-only: no scoring is done, only a cached resolution is returned (the
    /// C++ findTruncation comment).  Returns `Some((field_index, newoff))` or
    /// `None`.  `self` stands in for `op->getParent()->getFuncdata()`.
    pub fn find_truncation(
        &self,
        ct: &Rc<Datatype>,
        off: int8,
        sz: int4,
        op: OpId,
        slot: int4,
    ) -> KunaResult<Option<(int4, int8)>> {
        match &ct.kind {
            // TypePartialUnion::findTruncation (type.cc:2882-2884):
            //   return container->findTruncation(off + offset, sz, op, slot, newoff);
            DatatypeKind::PartialUnion { .. } => {
                let container = ct.get_partial_base().ok_or_else(|| {
                    KunaError::lowlevel("findTruncation: partial union missing container")
                })?;
                let part_off = ct.get_partial_offset().unwrap_or(0) as int8;
                self.find_truncation(&container, off + part_off, sz, op, slot)
            }
            // TypeUnion::findTruncation (type.cc:2613-2627): cached resolution only.
            DatatypeKind::Union { .. } => {
                // const ResolvedUnion *res = fd->getUnionResolution(this, op, slot);
                // if (res != 0 && res->getFieldNum() >= 0) { ... }
                let field_num = match self.get_union_resolution(ct, op, slot) {
                    Some(r) if r.get_field_num() >= 0 => r.get_field_num(),
                    _ => return Ok(None),
                };
                let field = match ct.get_field(field_num) {
                    Some(f) => f,
                    None => return Ok(None),
                };
                // newoff = offset - field->offset;
                let newoff = off - field.offset as int8;
                // if (newoff + sz > field->type->getSize()) return 0;  // spans > one field
                if newoff + sz as int8 > field.field_type.get_size() as int8 {
                    return Ok(None);
                }
                Ok(Some((field_num, newoff)))
            }
            // TypeStruct::findTruncation (type.cc:1878-1892) / base: the struct field
            // walk needs no cache, so the `Datatype` method covers it.
            _ => ct.find_truncation(off, sz, op, slot),
        }
    }
}
