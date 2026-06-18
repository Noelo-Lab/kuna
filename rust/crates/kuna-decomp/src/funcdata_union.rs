//! Port of the `Funcdata` union-field resolution cache (`decompiler/cpp/funcdata.cc:915-1115`,
//! W8 item `w10-actionsetcasts`) — the `unionMap` accessors that the W8 cast /
//! union-resolution subsystem ([`crate::coreaction_cleanup::ActionSetCasts`])
//! and the per-op `getInputCast`/`resolveInFlow` surface key on.
//!
//! These are additional `impl Funcdata` methods; the `union_map`
//! `BTreeMap<ResolveEdge, ResolvedUnion>` field and its clear live in
//! [`crate::funcdata`].  The map is keyed by [`ResolveEdge`] whose [`Ord`] is the
//! verbatim C++ `ResolveEdge::operator<`, so the `BTreeMap` reproduces the C++
//! `std::map` `find`/`emplace` semantics (HashMap is clippy-banned and would not
//! preserve the ordered lookup the C++ relies on).
//!
//! Every method here is transcribed statement-for-statement from the cited C++
//! body.  An edge is built from either a `(PcodeOp, slot)` pair (the data-flow
//! edge form, `ResolveEdge::new_op`) or a `(Address, slot)` pair (the
//! address-based form, `ResolveEdge::new_addr`).  The `PcodeOp` is identified by
//! its [`OpId`]; the methods read the live op out of `obank()` to recover
//! `op->getTime()` / `op->code()` / `op->numInput()` / `op->getIn()` /
//! `op->isMarker()` exactly as the C++ does through the `PcodeOp *`.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::KunaResult;
use kuna_base::types::int4;
use kuna_num::opcodes::OpCode;

use crate::dtype::{type_metatype, Datatype, TypeFactory};
use crate::funcdata::Funcdata;
use crate::seams::OpId;
use crate::unionresolve::{ResolveEdge, ResolvedUnion};

impl Funcdata {
    /// `op->getTime()` for a live op (the C++ `ResolveEdge` reads `op->getTime()`).
    /// Panics on a stale id, mirroring the other `obank().get(...).expect(...)`
    /// accessors in [`crate::funcdata_op`] (a stale op here is a logic error).
    fn op_time(&self, op: OpId) -> kuna_base::types::uintm {
        self.obank().get(op).expect("union edge: stale op").get_time()
    }

    /// Build the data-flow `ResolveEdge(unresType, op, slot)`
    /// (C++ `ResolveEdge::ResolveEdge(const Datatype*,const PcodeOp*,int4)`).
    fn union_edge_op(&self, unres_type: &Datatype, op: OpId, slot: int4) -> ResolveEdge {
        ResolveEdge::new_op(unres_type, self.op_time(op), slot)
    }

    /// \brief Get the resolution (union field) associated with an unresolved
    /// data-type and the given edge (C++ `Funcdata::getUnionField`,
    /// `funcdata.cc:915-929`).
    ///
    /// The cached resolution is only returned when its data-type size matches the
    /// unresolved data-type's size (the C++ "Check that the Varnode size matches"
    /// guard).
    pub fn get_union_field(
        &self,
        unres_type: &Datatype,
        op: OpId,
        slot: int4,
    ) -> Option<&ResolvedUnion> {
        // ResolveEdge edge(unresType,op,slot);
        let edge = self.union_edge_op(unres_type, op, slot);
        // iter = unionMap.find(edge);
        if let Some(res) = self.union_map.get(&edge) {
            // Datatype *dt = res->getDatatype();
            let dt = res.get_datatype();
            // Check that the Varnode size matches the data-type
            if unres_type.get_size() == dt.get_size() {
                return Some(res);
            }
        }
        None
    }

    /// \brief Get the resolution (union field) associated with an unresolved
    /// data-type and the given edge, without the size check (C++
    /// `Funcdata::getUnionResolution`, `funcdata.cc:939-950`).
    ///
    /// The size of the resolved data-type does not have to match the given
    /// data-type.
    pub fn get_union_resolution(
        &self,
        unres_type: &Datatype,
        op: OpId,
        slot: int4,
    ) -> Option<&ResolvedUnion> {
        // ResolveEdge edge(unresType,op,slot);
        let edge = self.union_edge_op(unres_type, op, slot);
        // iter = unionMap.find(edge); if (iter != end) return &(*iter).second;
        self.union_map.get(&edge)
    }

    /// \brief Get any resolution (union field) associated with an unresolved
    /// data-type and the given address (C++
    /// `Funcdata::getAddressBasedUnionField`, `funcdata.cc:959-968`).
    pub fn get_address_based_union_field(
        &self,
        unres_type: &Datatype,
        addr: &Address,
        slot: int4,
    ) -> Option<&ResolvedUnion> {
        // ResolveEdge edge(unresType,addr,slot);
        let edge = ResolveEdge::new_addr(unres_type, addr, slot);
        self.union_map.get(&edge)
    }

    /// \brief Associate a resolution (union field) with an unresolved data-type
    /// and the given edge (C++ `Funcdata::setUnionField`, `funcdata.cc:979-1004`).
    ///
    /// If there was a previous association, it is overwritten unless it was
    /// \e locked.  Returns `true` except where a previous *locked* association
    /// exists.  For a `MULTIEQUAL`, the resolution is also copied to every other
    /// input slot holding the same Varnode (data-type propagation does not happen
    /// between `MULTIEQUAL` input slots holding the same Varnode).
    pub fn set_union_field(
        &mut self,
        unres_type: &Datatype,
        op: OpId,
        slot: int4,
        resolve: ResolvedUnion,
    ) -> bool {
        // ResolveEdge edge(unresType,op,slot);
        let edge = self.union_edge_op(unres_type, op, slot);
        // res = unionMap.emplace(edge,resolve);
        // C++ std::map::emplace inserts iff the key is absent and reports
        // `res.second` (true == inserted).  `BTreeMap::entry` reproduces this:
        // a vacant entry inserts (the "second == true" arm), an occupied entry
        // runs the C++ "!res.second" arm.
        use std::collections::btree_map::Entry;
        match self.union_map.entry(edge) {
            Entry::Occupied(mut occ) => {
                // if (!(*res.first).second.update(resolve))
                //   return !(*res.first).second.isLocked();
                if !occ.get_mut().update(&resolve) {
                    return !occ.get().is_locked();
                }
            }
            Entry::Vacant(vac) => {
                vac.insert(resolve.clone());
            }
        }
        // if (op->code() == CPUI_MULTIEQUAL && slot >= 0)
        let opref = self.obank().get(op).expect("set_union_field: stale op");
        if opref.code() == OpCode::CPUI_MULTIEQUAL && slot >= 0 {
            // const Varnode *vn = op->getIn(slot);  // The Varnode being directly set
            let vn = opref.get_in(slot);
            let numinput = opref.num_input();
            // Gather the other input slots holding the same Varnode first (the
            // `opref` borrow of obank must be released before the mutable
            // `set` re-entry below; the C++ holds a `PcodeOp *` so reads stay
            // valid throughout — here we snapshot the slot list, which is the
            // same sequence of edges in the same order).
            let mut dup_slots: Vec<int4> = Vec::new();
            for i in 0..numinput {
                // if (i == slot) continue;
                if i == slot {
                    continue;
                }
                // if (op->getIn(i) != vn) continue;  // same Varnode in another slot
                if opref.get_in(i) != vn {
                    continue;
                }
                dup_slots.push(i);
            }
            for i in dup_slots {
                // ResolveEdge dupedge(unresType,op,i);
                let dupedge = self.union_edge_op(unres_type, op, i);
                // res = unionMap.emplace(dupedge,resolve);
                // if (!res.second) (*res.first).second.update(resolve);
                match self.union_map.entry(dupedge) {
                    Entry::Occupied(mut occ) => {
                        occ.get_mut().update(&resolve);
                    }
                    Entry::Vacant(vac) => {
                        vac.insert(resolve.clone());
                    }
                }
            }
        }
        true
    }

    /// \brief Associate a resolution with an unresolved data-type and the given
    /// address (C++ `Funcdata::setAddressBasedUnionField`,
    /// `funcdata.cc:1015-1028`).
    ///
    /// Overwrites a previous association unless it was \e locked.  Returns `true`
    /// except where a previous *locked* association exists.
    pub fn set_address_based_union_field(
        &mut self,
        unres_type: &Datatype,
        addr: &Address,
        slot: int4,
        resolve: ResolvedUnion,
    ) -> bool {
        // ResolveEdge edge(unresType,addr,slot);
        let edge = ResolveEdge::new_addr(unres_type, addr, slot);
        use std::collections::btree_map::Entry;
        match self.union_map.entry(edge) {
            Entry::Occupied(mut occ) => {
                // if ((*res.first).second.isLocked()) return false;
                if occ.get().is_locked() {
                    return false;
                }
                // (*res.first).second = resolve;
                *occ.get_mut() = resolve;
            }
            Entry::Vacant(vac) => {
                vac.insert(resolve);
            }
        }
        true
    }

    /// \brief Apply a `UnionFacetSymbol` to force a union-field interpretation at
    /// the data-flow edge it is dynamic-hashed to (C++ `Funcdata::applyUnionFacet`,
    /// `funcdata_varnode.cc:1658-1696`).
    ///
    /// This is the consumer of the `map unionfacet <union> <field> <addr> <hash>`
    /// console command (`IfcMapunionfacet` -> `Scope::addUnionFacetSymbol`): the
    /// facet Symbol carries a forced field number and a [`DynamicHash`] locating the
    /// op/slot the user pinned.  We re-find that op, build a *locked* [`ResolvedUnion`]
    /// for the forced field, and install it via [`set_union_field`](Self::set_union_field)
    /// / [`set_address_based_union_field`](Self::set_address_based_union_field) so the
    /// W8 cast/render layer reads the user's field instead of the default
    /// `resolveInFlow` pick.  Returns `true` when a (new) resolution was installed.
    ///
    /// Reached from [`attempt_dynamic_mapping`](Funcdata::attempt_dynamic_mapping)
    /// and [`attempt_dynamic_mapping_late`](Funcdata::attempt_dynamic_mapping_late)
    /// for a `union_facet`-category SymbolEntry.
    pub fn apply_union_facet(&mut self, entry: &crate::database::SymbolEntry) -> KunaResult<bool> {
        let sym_id = entry.symbol;
        let (sym_type, field_number, addr_based) = {
            let localmap = match self.get_scope_local() {
                Some(l) => l,
                None => return Ok(false),
            };
            let sym = localmap.database().symbol(sym_id);
            let dt = match sym.dtype.as_ref() {
                Some(d) => Rc::clone(d),
                None => return Ok(false),
            };
            (dt, sym.get_field_number(), sym.is_addr_based())
        };
        let typegrp = match self.get_arch().types_rc() {
            Some(t) => t,
            None => return Ok(false),
        };
        // sym->isAddrBased(): build a locked resolution keyed on the first-use
        // address + the hash-encoded slot (C++ funcdata_varnode.cc:1661-1666).
        if addr_based {
            let mut resolve = ResolvedUnion::new_field(Rc::clone(&sym_type), field_number, typegrp.as_ref())?;
            resolve.set_lock(true);
            let slot = crate::dynamic::DynamicHash::get_slot_from_hash(entry.get_hash());
            let first_use = entry.get_first_use_address();
            return Ok(self.set_address_based_union_field(&sym_type, &first_use, slot, resolve));
        }
        // op = dhash.findOp(this, entry->getFirstUseAddress(), entry->getHash());
        let first_use = entry.get_first_use_address();
        let hash = entry.get_hash();
        let mut dhash = crate::dynamic::DynamicHash::new();
        let op = match dhash.find_op(self, &first_use, hash) {
            Some(o) => o,
            None => return Ok(false),
        };
        // slot = DynamicHash::getSlotFromHash(entry->getHash());
        let slot = crate::dynamic::DynamicHash::get_slot_from_hash(hash);
        // const ResolvedUnion *res = getUnionResolution(sym->getType(), op, slot);
        // if (res && res->getFieldNum() == sym->getFieldNumber()) return false;
        if let Some(res) = self.get_union_resolution(&sym_type, op, slot) {
            if res.get_field_num() == field_number {
                return Ok(false);
            }
        }
        // Varnode *vn = (slot < 0) ? op->getOut() : op->getIn(slot);
        let vn = if slot < 0 {
            self.obank().get(op).and_then(|o| o.get_out())
        } else {
            self.obank().get(op).and_then(|o| o.get_in(slot))
        };
        let vn = match vn {
            Some(v) => v,
            None => return Ok(false),
        };
        // Datatype *dt = vn->getType();  — widen unresType to the pointer / partial
        // wrapper the Varnode actually carries, when it wraps the facet's union.
        let mut unres_type = Rc::clone(&sym_type);
        if let Some(dt) = self.vbank().get(vn).map(|v| Rc::clone(v.get_type())) {
            match dt.get_metatype() {
                type_metatype::TYPE_PTR => {
                    // if (((TypePointer*)dt)->getPtrTo() == unresType) unresType = dt;
                    if dt.get_ptr_to().map(|p| Rc::ptr_eq(&p, &sym_type)).unwrap_or(false) {
                        unres_type = dt;
                    }
                }
                type_metatype::TYPE_PARTIALSTRUCT => {
                    // if (((TypePartialStruct*)dt)->getParent() == unresType) unresType = dt;
                    if dt.get_partial_base().map(|p| Rc::ptr_eq(&p, &sym_type)).unwrap_or(false) {
                        unres_type = dt;
                    }
                }
                type_metatype::TYPE_PARTIALUNION => {
                    // if (((TypePartialUnion*)dt)->getParentUnion() == unresType) unresType = dt;
                    if dt.get_partial_base().map(|p| Rc::ptr_eq(&p, &sym_type)).unwrap_or(false) {
                        unres_type = dt;
                    }
                }
                _ => {}
            }
        }
        // ResolvedUnion resolve(unresType, sym->getFieldNumber(), *glb->types);
        // resolve.setLock(true); setUnionField(unresType, op, slot, resolve);
        let mut resolve = ResolvedUnion::new_field(Rc::clone(&unres_type), field_number, typegrp.as_ref())?;
        resolve.set_lock(true);
        self.set_union_field(&unres_type, op, slot, resolve);
        Ok(true)
    }

    /// \brief Update the resolution data-type for a given edge (C++
    /// `Funcdata::updateUnionField`, `funcdata.cc:1038-1049`).
    ///
    /// If further resolution of the cached data-type has happened, update the
    /// cache with the new data-type.  Returns `true` unless there was no
    /// association for the edge.
    pub fn update_union_field(
        &mut self,
        unres_type: &Datatype,
        op: OpId,
        slot: int4,
        res_type: Rc<Datatype>,
    ) -> bool {
        let edge = self.union_edge_op(unres_type, op, slot);
        // iter = unionMap.find(edge);
        if let Some(entry) = self.union_map.get_mut(&edge) {
            // (*iter).second.setResolve(resType); return true;
            entry.set_resolve(res_type);
            return true;
        }
        false
    }

    /// \brief Force a specific resolution (union field) for an unresolved
    /// data-type and the given edge (C++ `Funcdata::forceFacingType`,
    /// `funcdata.cc:1057-1069`).
    ///
    /// A relative pointer is *not* associated with the resolution; it is first
    /// converted to a standard pointer to the same base.
    pub fn force_facing_type(
        &mut self,
        unres_type: Rc<Datatype>,
        field_num: int4,
        op: OpId,
        slot: int4,
    ) -> KunaResult<()> {
        // Datatype *baseType = unresType;
        // if (baseType->getMetatype() == TYPE_PTR) baseType = ((TypePointer*)baseType)->getPtrTo();
        let base_type = if unres_type.get_metatype() == type_metatype::TYPE_PTR {
            unres_type.get_ptr_to().ok_or_else(|| {
                kuna_base::error::KunaError::lowlevel("forceFacingType: pointer missing ptrto")
            })?
        } else {
            Rc::clone(&unres_type)
        };
        // if (unresType->isPointerRel()) { ... convert to a standard pointer ... }
        let unres_type = if unres_type.is_pointer_rel() {
            let typegrp = self.get_arch().types_rc().ok_or_else(|| {
                kuna_base::error::KunaError::lowlevel(
                    "forceFacingType: TypeFactory unavailable for relative-pointer conversion",
                )
            })?;
            let wordsize = unres_type.get_word_size().ok_or_else(|| {
                kuna_base::error::KunaError::lowlevel("forceFacingType: pointer missing wordsize")
            })?;
            // unresType = glb->types->getTypePointer(unresType->getSize(), baseType, wordSize);
            typegrp.get_type_pointer(unres_type.get_size(), base_type, wordsize)?
        } else {
            unres_type
        };
        // ResolvedUnion resolve(unresType,fieldNum,*glb->types);
        let typegrp = self.get_arch().types_rc().ok_or_else(|| {
            kuna_base::error::KunaError::lowlevel("forceFacingType: TypeFactory unavailable")
        })?;
        let resolve = ResolvedUnion::new_field(Rc::clone(&unres_type), field_num, typegrp.as_ref())?;
        // setUnionField(unresType, op, slot, resolve);
        self.set_union_field(&unres_type, op, slot, resolve);
        Ok(())
    }

    /// \brief Copy a read/write facing resolution from one PcodeOp to another
    /// (C++ `Funcdata::inheritUnionField`, `funcdata.cc:1079-1091`).
    ///
    /// Returns the field number that was inherited, or -1 if `old_op`/`old_slot`
    /// had no associated resolution.
    pub fn inherit_union_field(
        &mut self,
        unres_type: &Datatype,
        op: OpId,
        slot: int4,
        old_op: OpId,
        old_slot: int4,
    ) -> int4 {
        // if (slot < 0 && oldOp->isMarker()) slot = 0;
        let slot = if slot < 0 && self.obank().get(old_op).expect("inherit: stale op").is_marker() {
            0
        } else {
            slot
        };
        // ResolveEdge edge(unresType,oldOp,oldSlot);
        let edge = self.union_edge_op(unres_type, old_op, old_slot);
        // iter = unionMap.find(edge); if (iter == end) return -1;
        let inherited = match self.union_map.get(&edge) {
            Some(res) => res.clone(),
            None => return -1,
        };
        let field_num = inherited.get_field_num();
        // setUnionField(unresType,op,slot,(*iter).second);
        self.set_union_field(unres_type, op, slot, inherited);
        // return (*iter).second.getFieldNum();
        field_num
    }

    /// \brief Create a resolution for an unresolved pointer data-type based on an
    /// earlier non-pointer resolution (C++ `Funcdata::inheritUnionFieldPtr`,
    /// `funcdata.cc:1101-1115`).
    ///
    /// Returns the inherited field number, or -1 if there was nothing to inherit.
    pub fn inherit_union_field_ptr(
        &mut self,
        unres_ptr: Rc<Datatype>,
        op: OpId,
        slot: int4,
        old_op: OpId,
        old_slot: int4,
    ) -> KunaResult<int4> {
        // Datatype *parent = unresPtr->getDepend(0);
        let parent = unres_ptr.get_depend(0).ok_or_else(|| {
            kuna_base::error::KunaError::lowlevel("inheritUnionFieldPtr: pointer missing depend(0)")
        })?;
        // if (slot < 0 && oldOp->isMarker()) slot = 0;
        let slot = if slot < 0 && self.obank().get(old_op).expect("inherit ptr: stale op").is_marker()
        {
            0
        } else {
            slot
        };
        // ResolveEdge edge(parent,oldOp,oldSlot);
        let edge = self.union_edge_op(&parent, old_op, old_slot);
        // iter = unionMap.find(edge); if (iter == end) return -1;
        let inherited_field = match self.union_map.get(&edge) {
            Some(res) => res.get_field_num(),
            None => return Ok(-1),
        };
        // ResolvedUnion ptrres(unresPtr,(*iter).second.getFieldNum(),*glb->types);
        let typegrp = self.get_arch().types_rc().ok_or_else(|| {
            kuna_base::error::KunaError::lowlevel("inheritUnionFieldPtr: TypeFactory unavailable")
        })?;
        let ptrres = ResolvedUnion::new_field(Rc::clone(&unres_ptr), inherited_field, typegrp.as_ref())?;
        // setUnionField(unresPtr,op,slot,ptrres);
        self.set_union_field(&unres_ptr, op, slot, ptrres);
        // return (*iter).second.getFieldNum();
        Ok(inherited_field)
    }
}
