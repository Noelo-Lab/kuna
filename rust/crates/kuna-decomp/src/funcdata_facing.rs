//! The HighVariable read-/def-facing **type graph** the S6 Merge stage exposes to
//! the next (Casts) stage (W10 item `w10-merge-casts`).
//!
//! `ActionSetCasts` (the S9 cast plane) decides whether a CAST/PTRSUB op must be
//! inserted by comparing the type a PcodeOp *expects* at a slot against the type a
//! Varnode's merged HighVariable *carries* at that read/write.  That second type is
//! the **facing type**.  In C++ the four facing accessors live on `Varnode`
//! (`varnode.cc:645-691`):
//!
//! ```text
//! Varnode::getTypeDefFacing()              -> type / type->findResolve(def,-1)
//! Varnode::getTypeReadFacing(op)           -> type / type->findResolve(op,slot)
//! Varnode::getHighTypeDefFacing()          -> high->getType() / ct->findResolve(def,-1)
//! Varnode::getHighTypeReadFacing(op)       -> high->getType() / ct->findResolve(op,slot)
//! ```
//!
//! The `findResolve` overrides (`type.cc:590-2986`: `TypePointer`/`TypeArray`/
//! `TypeStruct`/`TypeUnion`/`TypePartialUnion`) all reach the per-function union
//! cache through `op->getParent()->getFuncdata()->getUnionField(this,op,slot)`.
//! A `Datatype` in the Rust port carries no `Funcdata` back-pointer (ADR-0001:
//! types are interned arena values, not graph-owned), so `findResolve` cannot be a
//! method on `Datatype` the way the C++ has it.  Instead the *whole* facing
//! computation — including the `findResolve` dispatch — lives here on `Funcdata`,
//! which already owns both the merged `high_bank` and the `union_map` cache
//! (`crate::funcdata_union`).  This is the faithful equivalent: every C++ call site
//! reaches `findResolve` only with an `op` in hand whose parent's `Funcdata` is
//! exactly the `self` here.
//!
//! ## Default (no unions) is exact
//!
//! For the overwhelming majority of the corpus the facing type does **not** need
//! resolution (`Datatype::needsResolution()` is false unless the type is a union or
//! a pointer-to-union), so every accessor returns the bare type — `vn->getType()`
//! for the def/read-facing pair and the merged `high->getType()` for the
//! High-facing pair.  That keeps boolless / readstruct / condconst byte-identical
//! (no union types flow there).  The union arms consult the landed
//! [`Funcdata::get_union_field`](crate::funcdata_union) cache exactly as the C++
//! `findResolve` overrides do, so the typed-access cluster (`union_datatype.xml`)
//! gets the resolved field type once the Casts stage drives the cache.

use std::rc::Rc;

use crate::dtype::{type_metatype, Datatype, DatatypeKind};
use crate::funcdata::Funcdata;
use crate::seams::{OpId, VarnodeId};

impl Funcdata {
    /// `Datatype::findResolve(op, slot)` dispatched against the live union cache
    /// (C++ `type.cc:590` + the `TypePointer`/`TypeArray`/`TypeStruct`/`TypeUnion`/
    /// `TypePartialUnion` overrides).  `self` stands in for
    /// `op->getParent()->getFuncdata()`.
    ///
    /// Only types for which `needsResolution()` is set ever reach the union-cache
    /// arms; the caller (the facing accessors below) already gates on
    /// `needsResolution()`, but the per-kind dispatch is transcribed in full so the
    /// fallback type (the C++ `field[0].type` / `arrayof` / `stripped`) is faithful
    /// when a needs-resolution type misses the cache.
    pub fn find_resolve_facing(
        &self,
        ct: &Rc<Datatype>,
        op: OpId,
        slot: i32,
    ) -> Rc<Datatype> {
        match &ct.kind {
            // TypePointer::findResolve (type.cc:1335-1345): only a pointer *to a
            // union* consults the cache; any other pointer returns itself.
            DatatypeKind::Pointer { ptrto, .. } => {
                if ptrto.get_metatype() == type_metatype::TYPE_UNION {
                    if let Some(res) = self.get_union_field(ct, op, slot) {
                        return Rc::clone(res.get_datatype());
                    }
                }
                Rc::clone(ct)
            }
            // TypeArray::findResolve (type.cc:1470-1478): cache, else the element.
            DatatypeKind::Array { arrayof, .. } => {
                if let Some(res) = self.get_union_field(ct, op, slot) {
                    return Rc::clone(res.get_datatype());
                }
                Rc::clone(arrayof)
            }
            // TypeStruct::findResolve (type.cc:2290-2298): cache, else field[0].
            DatatypeKind::Struct { field, .. } => {
                if let Some(res) = self.get_union_field(ct, op, slot) {
                    return Rc::clone(res.get_datatype());
                }
                match field.first() {
                    Some(f) => Rc::clone(&f.field_type),
                    // A field-less struct cannot occur for a needs-resolution
                    // struct, but keep the type rather than panic (fail-closed).
                    None => Rc::clone(ct),
                }
            }
            // TypeUnion::findResolve (type.cc:2559-2567): cache, else self.
            DatatypeKind::Union { .. } => {
                if let Some(res) = self.get_union_field(ct, op, slot) {
                    return Rc::clone(res.get_datatype());
                }
                Rc::clone(ct)
            }
            // TypePartialUnion::findResolve (type.cc:2978-2986): cache, else stripped.
            DatatypeKind::PartialUnion { stripped, .. } => {
                if let Some(res) = self.get_union_field(ct, op, slot) {
                    return Rc::clone(res.get_datatype());
                }
                Rc::clone(stripped)
            }
            // Datatype::findResolve base (type.cc:590-594): return this.
            _ => Rc::clone(ct),
        }
    }

    /// C++ `Varnode::getTypeDefFacing()` (varnode.cc:645-651): the Varnode's own
    /// data-type, resolved against the *write* edge (`slot == -1`) when it needs
    /// union resolution.  `vn` **must** be written.
    pub fn vn_type_def_facing(&self, vn: VarnodeId) -> Rc<Datatype> {
        let v = self.vbank().get(vn).expect("vn_type_def_facing: stale vn");
        let ty = Rc::clone(v.get_type());
        if !ty.needs_resolution() {
            return ty;
        }
        // return type->findResolve(def,-1);
        let def = v.get_def().expect("getTypeDefFacing: Varnode must be written");
        self.find_resolve_facing(&ty, def, -1)
    }

    /// C++ `Varnode::getTypeReadFacing(op)` (varnode.cc:658-664): the Varnode's own
    /// data-type, resolved against the *read* edge (`op->getSlot(this)`) when it
    /// needs union resolution.
    pub fn vn_type_read_facing(&self, vn: VarnodeId, op: OpId) -> Rc<Datatype> {
        let ty = Rc::clone(self.vbank().get(vn).expect("vn_type_read_facing: stale vn").get_type());
        if !ty.needs_resolution() {
            return ty;
        }
        // return type->findResolve(op, op->getSlot(this));
        let slot = self.obank().get(op).map(|o| o.get_slot(vn)).unwrap_or(-1);
        self.find_resolve_facing(&ty, op, slot)
    }

    /// C++ `Varnode::getHighTypeDefFacing()` (varnode.cc:670-677): the *merged*
    /// HighVariable's data-type, resolved against the write edge when it needs
    /// union resolution.  This is the def-facing node of the type graph the Casts
    /// stage walks.
    pub fn vn_high_type_def_facing(&mut self, vn: VarnodeId) -> Rc<Datatype> {
        // Datatype *ct = high->getType();
        let ct = self
            .high_get_type(vn)
            .unwrap_or_else(|| Rc::clone(self.vbank().get(vn).expect("stale vn").get_type()));
        if !ct.needs_resolution() {
            return ct;
        }
        // return ct->findResolve(def,-1);
        let def = self
            .vbank()
            .get(vn)
            .and_then(|v| v.get_def())
            .expect("getHighTypeDefFacing: Varnode must be written");
        self.find_resolve_facing(&ct, def, -1)
    }

    /// C++ `Varnode::getHighTypeReadFacing(op)` (varnode.cc:684-691): the *merged*
    /// HighVariable's data-type, resolved against the read edge when it needs union
    /// resolution.  This is the read-facing node of the type graph
    /// `ActionSetCasts::castInput`/`castOutput` query.
    pub fn vn_high_type_read_facing(&mut self, vn: VarnodeId, op: OpId) -> Rc<Datatype> {
        // Datatype *ct = high->getType();
        let ct = self
            .high_get_type(vn)
            .unwrap_or_else(|| Rc::clone(self.vbank().get(vn).expect("stale vn").get_type()));
        if !ct.needs_resolution() {
            return ct;
        }
        // return ct->findResolve(op, op->getSlot(this));
        let slot = self.obank().get(op).map(|o| o.get_slot(vn)).unwrap_or(-1);
        self.find_resolve_facing(&ct, op, slot)
    }
}
