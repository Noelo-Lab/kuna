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
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::dtype::{type_metatype, Datatype, DatatypeKind};
use crate::funcdata::Funcdata;
use crate::seams::OpId;
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
            DatatypeKind::Pointer { ptrto, .. } => {
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
    /// [`ScoreUnionFields`]).  That nested resolveTruncation is **not yet wired**
    /// (`// SEAM(W10 partial-truncation)`): the partial-union arm falls through to
    /// the `stripped` type (the C++ "Should never reach here / size mismatch"
    /// fallback, type.cc:2971-2973) and caches that, which is the faithful
    /// conservative result when the truncation does not resolve to a field.
    /// `partialunion.xml`'s simple `u.b.bval1` accesses resolve through the cache
    /// the cast plane already populated for the enclosing union read.
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
        // The full container walk needs TypeUnion::resolveTruncation (the SUBPIECE
        // / implied-truncation scorer constructors).  Until that is wired, take the
        // C++ fall-through: curType = stripped; updateUnionField(this,op,slot,curType).
        let stripped = ct.get_stripped().ok_or_else(|| {
            KunaError::lowlevel("resolveInFlow: partial union missing stripped")
        })?;
        // fd->updateUnionField(this, op, slot, curType);  // only updates an existing edge
        self.update_union_field(ct, op, slot, Rc::clone(&stripped));
        Ok(stripped)
    }
}
