//! Stack-pointer / spacebase recovery and symbol synchronization on `Funcdata`
//! (the back half of the stack-variable promotion chain).
//!
//! These are additional `impl Funcdata` methods porting:
//!   - `Funcdata::spacebase` (`funcdata.cc:228`) — mark the stack-pointer
//!     Varnodes as `spacebase` and type the input register as a pointer.  Driven
//!     by `ActionSpacebase::apply` (`coreaction.cc:1648`).
//!   - `Funcdata::splitUses` (`funcdata_varnode.cc`) — duplicate the def of an
//!     already-spacebase Varnode so each descendant gets its own copy.
//!   - `Funcdata::findSpacebaseInput` (`funcdata.cc:289`) — the input Varnode
//!     that holds the incoming stack pointer for a given space.
//!   - `Funcdata::syncVarnodesWithSymbols` / `syncVarnodesWithSymbol`
//!     (`funcdata_varnode.cc:959`/`889`) — paint the `mapped`/`addrtied`/
//!     `nolocalalias`/`addrforce` flags and the recovered data-type onto every
//!     Varnode in the stack space from the overlapping mapped Symbol.  Driven by
//!     `ActionRestructureVarnode`/`ActionMappedLocalSync`.
//!
//! The C++ uses raw `AddrSpace *` / `Varnode *` pointers; this port uses the
//! shared `Rc<AddrSpace>` and `VarnodeId` arena ids, and reaches the type
//! factory through `glb.types()` (W6).  `uintb` is `u64` with wrapping ops.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::types::{int4, int8, uint4, uintb};

use kuna_num::opcodes::OpCode;

use crate::dtype::TypeFactory;
use crate::funcdata::Funcdata;
use crate::op::pcodeop_flags;
use crate::context::{OpId, TypeOp, VarnodeId};
use crate::varnode::varnode_flags;

#[cfg(test)]
mod spacebase_tests;

impl Funcdata {
    /// Mark Varnode objects that hold stack-pointer values and set up special
    /// data-types (C++ `Funcdata::spacebase`, `funcdata.cc:228`).
    ///
    /// For each address space with a spacebase (stack) register, walk the
    /// Varnodes at the base register's storage: a base register that is **not**
    /// yet marked `spacebase` is marked (all of them, not just the input), and if
    /// it is the input it is typed as a pointer to the space.  A base register
    /// that is **already** marked and is defined by `INT_ADD` is split so a stale
    /// multi-descendant chain doesn't block promotion (`splitUses`).
    pub fn spacebase(&mut self) {
        let numspaces = self.get_arch().manage().num_spaces();
        for j in 0..numspaces {
            let spc = match self.get_arch().manage().get_space(j) {
                Some(s) => Rc::clone(s),
                None => continue,
            };
            let numspace = spc.num_spacebase();
            for i in 0..numspace {
                let point = match spc.get_spacebase(i) {
                    Ok(p) => p,
                    Err(_) => continue,
                };
                let point_space = match &point.space {
                    Some(s) => Rc::clone(s),
                    None => continue,
                };
                let point_size = point.size as int4;
                let point_addr = Address::new(point_space, point.offset);

                // Datatype *ct = glb->types->getTypeSpacebase(spc, getAddress());
                // Datatype *ptr = glb->types->getTypePointer(point.size, ct, wordsize);
                let ptr = {
                    let baseaddr = self.get_address().clone();
                    let types = self.get_arch().types();
                    match types {
                        Some(t) => match t.get_type_spacebase(Rc::clone(&spc), &baseaddr) {
                            Ok(ct) => {
                                t.get_type_pointer(point_size, ct, spc.get_word_size()).ok()
                            }
                            Err(_) => None,
                        },
                        None => None,
                    }
                };

                let vnlist: Vec<VarnodeId> =
                    self.vbank().iter_loc_size_addr(point_size, &point_addr).collect();
                for vn in vnlist {
                    let (is_free, is_spacebase, is_input, def) = {
                        let v = match self.vbank().get(vn) {
                            Some(v) => v,
                            None => continue,
                        };
                        (v.is_free(), v.is_spacebase(), v.is_input(), v.get_def())
                    };
                    if is_free {
                        continue;
                    }
                    if is_spacebase {
                        // Already marked spacebase: give descendants a chance to be
                        // eliminated naturally, then force a split if it still has
                        // multiple descendants of an INT_ADD def.
                        if let Some(op) = def {
                            if self.obank().get(op).map(|o| o.code()) == Some(OpCode::CPUI_INT_ADD)
                            {
                                self.split_uses(vn);
                            }
                        }
                    } else {
                        // Mark all base registers (not just input).
                        if let Some(v) = self.vbank_mut().get_mut(vn) {
                            v.set_spacebase();
                        }
                        // Only set the type on the input spacebase register.
                        if is_input {
                            if let Some(ptr) = ptr.clone() {
                                let high = self.vbank().get(vn).and_then(|v| v.get_high());
                                let changed = self
                                    .vbank_mut()
                                    .get_mut(vn)
                                    .map(|v| v.update_type_locked(ptr, true, true))
                                    .unwrap_or(false);
                                if changed {
                                    if let Some(h) = high {
                                        if let Some(hh) = self.high_bank_mut().get_mut(h) {
                                            hh.type_dirty();
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    /// C++ `TypePointer::isPtrsubMatching` (`type.cc:1260-1312`), with the
    /// `TYPE_SPACEBASE` arm resolved through this function's local symbol scope.
    ///
    /// A pure `Datatype::is_ptrsub_matching` cannot reach the symbol table that
    /// `TypeSpacebase::getSubType` needs (the C++ `TypeSpacebase` carries `glb`);
    /// for a pointer-to-spacebase we therefore resolve the sub-type here through
    /// [`ScopeLocal::spacebase_get_sub_type`] and otherwise delegate to the pure
    /// method.  Callers are `ActionSetCasts::castFixupPtrsub`
    /// (`coreaction.cc:2837`) and `RulePtrsubUndo::applyOp` (`ruleaction.cc:7169`),
    /// which must NOT undo a valid stack-frame `PTRSUB(sp, off)`.
    pub fn is_ptrsub_matching_scope(
        &self,
        ptrtype: &Rc<crate::dtype::Datatype>,
        off: int8,
        extra: int8,
        multiplier: int8,
    ) -> bool {
        use crate::dtype::type_metatype;
        // Only the TYPE_PTR -> TYPE_SPACEBASE case needs scope resolution.
        if ptrtype.get_metatype() == type_metatype::TYPE_PTR {
            if let Some(ptrto) = ptrtype.get_ptr_to() {
                if ptrto.get_metatype() == type_metatype::TYPE_SPACEBASE {
                    let word_size = ptrtype.get_word_size().unwrap_or(1);
                    let (lm, types) =
                        match (self.get_scope_local(), self.get_arch().types_rc()) {
                            (Some(lm), Some(t)) => (lm, t),
                            _ => return false,
                        };
                    let new_off =
                        kuna_base::space::AddrSpace::address_to_byte_int(off, word_size);
                    let (sub_type, new_off2) =
                        match lm.spacebase_get_sub_type(new_off, types.as_ref()) {
                            Ok(r) => r,
                            Err(_) => return false,
                        };
                    // subType is always Some here (TypeSpacebase::getSubType never
                    // returns null), so only newoff must be zero.
                    if new_off2 != 0 {
                        return false;
                    }
                    let extra_b =
                        kuna_base::space::AddrSpace::address_to_byte_int(extra, word_size);
                    if extra_b < 0 || extra_b >= sub_type.get_size() as i64 {
                        match crate::dtype::Datatype::test_for_array_slack(&sub_type, extra_b) {
                            Ok(true) => {}
                            _ => return false,
                        }
                    }
                    return true;
                }
            }
        }
        // Non-spacebase: the pure method is fully faithful.
        ptrtype.is_ptrsub_matching(off, extra, multiplier).unwrap_or(false)
    }

    /// C++ `TypeSpacebase::getSubType` (`type.cc:3411-3433`), resolved against the
    /// scope the spacebase's `localframe` selects via `getMap()` (`type.cc:3398`):
    /// the GLOBAL scope when `localframe.isInvalid()`, else the owning function's
    /// `ScopeLocal`.
    ///
    /// The pure `Datatype::get_sub_type` cannot reach the symbol table that a
    /// `TypeSpacebase` indexes (the C++ `TypeSpacebase` carries `glb`); the global
    /// arm here routes through the frozen [`GlobalQuery`](crate::context::GlobalQuery)
    /// snapshot (`glb.resolve_constant` + `glb.query_container_global`), exactly as
    /// `ActionConstantPtr::isPointer` does, and the local arm delegates to the
    /// existing [`ScopeLocal::spacebase_get_sub_type`].  Returns
    /// `(subtype, newoff)`; like the C++ it never yields a null sub-type — a miss
    /// yields `getBase(1, TYPE_UNKNOWN)` with `newoff = 0`.
    ///
    /// `sb_type` is the `TYPE_SPACEBASE` data-type; `off` is the byte offset
    /// (already address-to-byte converted by the caller, matching the `int8 off`
    /// the C++ `getSubType` receives from `TypePointer::downChain`).
    pub fn spacebase_get_sub_type(
        &self,
        sb_type: &Rc<crate::dtype::Datatype>,
        off: int8,
    ) -> Option<(Rc<crate::dtype::Datatype>, int8)> {
        use crate::dtype::type_metatype;
        let (spaceid, localframe) = sb_type.spacebase_parts()?;
        let spaceid = spaceid?;
        let types = self.get_arch().types_rc()?;
        // Local-frame spacebase: getMap() -> the owning function's ScopeLocal.
        if !localframe.is_invalid() {
            let lm = self.get_scope_local()?;
            return lm.spacebase_get_sub_type(off, types.as_ref()).ok();
        }
        // Global spacebase: getMap() -> the global scope (the GlobalQuery snapshot).
        // addrOff = byteToAddress(off, wordSize); addr = resolveConstant(spaceid,
        // addrOff, -1, nullPoint, fullEncoding).
        let word_size = spaceid.get_word_size();
        let addr_off = kuna_base::space::AddrSpace::byte_to_address(off as u64, word_size);
        let null_point = Address::new_invalid();
        let glb = Rc::clone(self.get_arch());
        let mut full_encoding: uintb = 0;
        let addr = glb
            .resolve_constant(&spaceid, addr_off, -1, &null_point, &mut full_encoding)
            .ok()?;
        // smallest = scope->queryContainer(addr, 1, nullPoint).
        match glb.query_container_global(&addr, 1, &null_point) {
            Some(entry) => {
                // newoff = (addr - smallest.addr) + smallest.offset.
                let newoff = (addr.get_offset().wrapping_sub(entry.entry_addr.get_offset())
                    as int8)
                    + entry.symbol_offset as int8;
                match entry.symbol_type {
                    Some(t) => Some((t, newoff)),
                    // A Symbol with no snapshot type degrades to UNKNOWN(1) (the C++
                    // Symbol always has a type here).
                    None => Some((types.get_base(1, type_metatype::TYPE_UNKNOWN).ok()?, 0)),
                }
            }
            // No covering Symbol: *newoff = 0; return getBase(1, TYPE_UNKNOWN).
            None => Some((types.get_base(1, type_metatype::TYPE_UNKNOWN).ok()?, 0)),
        }
    }

    /// Make a clone of the defining op for every descendant after the first, so
    /// each descendant has its own copy (C++ `Funcdata::splitUses`).
    ///
    /// Dead-code actions remove the now-unused original op.
    pub fn split_uses(&mut self, vn: VarnodeId) {
        let op = match self.vbank().get(vn).and_then(|v| v.get_def()) {
            Some(o) => o,
            None => return,
        };
        let descend: Vec<OpId> = match self.vbank().get(vn) {
            Some(v) => v.descend_iter().collect(),
            None => return,
        };
        if descend.len() < 2 {
            return; // No descendants, or only one
        }
        let numinput = match self.obank().get(op) {
            Some(o) => o.num_input(),
            None => return,
        };
        let opaddr = match self.obank().get(op) {
            Some(o) => o.get_addr().clone(),
            None => return,
        };
        let opcode = match self.obank().get(op) {
            Some(o) => o.code(),
            None => return,
        };
        let vnsize = self.vbank().get(vn).map(|v| v.get_size()).unwrap_or(0);
        let vnaddr = self.vbank().get(vn).map(|v| v.get_addr().clone());
        let vntype = self.vbank().get(vn).map(|v| v.get_type().clone());
        let (vnaddr, vntype) = match (vnaddr, vntype) {
            (Some(a), Some(t)) => (a, t),
            _ => return,
        };
        // The op's input list (snapshot before cloning).
        let inputs: Vec<Option<VarnodeId>> = (0..numinput)
            .map(|k| self.obank().get(op).and_then(|o| o.get_in(k)))
            .collect();

        for &useop in &descend {
            // slot = useop->getSlot(vn)
            let slot = match self.obank().get(useop).map(|o| o.get_slot(vn)) {
                Some(s) if s >= 0 => s,
                _ => continue,
            };
            // newop = newOp(numinput, opaddr); newvn = newVarnode(size, addr, type);
            let newop = self.new_op(numinput, opaddr.clone());
            let newvn = self.new_varnode(vnsize, &vnaddr, Some(Rc::clone(&vntype)));
            // opSetOutput(newop, newvn);
            let _ = self.op_set_output(newop, newvn);
            // opSetOpcode(newop, op->code());  -- the only opcode `spacebase`
            // splits is INT_ADD (binary | commutative; typeop.cc TypeOpIntAdd).
            self.op_set_opcode(
                newop,
                TypeOp::new(
                    opcode,
                    pcodeop_flags::binary | pcodeop_flags::commutative,
                    "+".to_string(),
                ),
            );
            // for i: opSetInput(newop, op->getIn(i), i);
            for (i, inp) in inputs.iter().enumerate() {
                if let Some(iv) = inp {
                    let _ = self.op_set_input(newop, *iv, i as int4);
                }
            }
            // opSetInput(useop, newvn, slot);
            let _ = self.op_set_input(useop, newvn, slot);
            // opInsertBefore(newop, op);
            self.op_insert_before(newop, op);
        }
    }

    /// Locate the unique input Varnode that holds the incoming value of a
    /// spacebase (stack-pointer) register (C++ `Funcdata::findSpacebaseInput`,
    /// `funcdata.cc:289`).
    pub fn find_spacebase_input(&self, id: &Rc<kuna_base::space::AddrSpace>) -> Option<VarnodeId> {
        let point = id.get_spacebase(0).ok()?;
        let point_space = point.space.as_ref()?;
        let addr = Address::new(Rc::clone(point_space), point.offset);
        self.vbank().find_input(point.size as int4, &addr)
    }

    /// C++ `localmap->queryProperties(addr,size,usepoint,...)` for the local stack
    /// scope (`database.cc:1268` via [`crate::varmap::ScopeLocal::query_properties`]):
    /// the Varnode boolean properties (`mapped | addrtied | typelock | ...`) a
    /// `map addr`-mapped stack range carries.  Returns `0` when there is no local
    /// scope or the range is not in its map.  This is the local-scope half of the
    /// `Heritage::guard` property walk; the global-scope half is
    /// [`crate::context::ArchHandle::query_global_properties`].
    pub fn query_local_properties(
        &self,
        addr: &Address,
        size: int4,
        usepoint: &Address,
    ) -> kuna_base::types::uint4 {
        match self.get_scope_local() {
            Some(lm) => lm.query_properties(addr, size, usepoint),
            None => 0,
        }
    }

    /// C++ `ActionRestrictLocal::apply` (`coreaction.cc:2003-2059`): restrict the
    /// possible range of local variables by marking two classes of stack storage as
    /// *not mapped* in the local scope.
    ///
    /// 1. The stack-passed parameter slots of every input-locked sub-function call
    ///    (`numCalls`/`getCallSpecs`).  When the call's spacebase offset is known,
    ///    each `IPTR_SPACEBASE` (or `IPTR_JOIN` of spacebase pieces) parameter
    ///    address is mapped through the call's stack frame and excised.
    /// 2. The save-register/return-address storage the function leaves *unaffected*
    ///    (iterate the prototype effect list, skip `killedbycall`).  For an input
    ///    Varnode that is `isUnaffected`, every `CPUI_COPY` descendant whose output
    ///    lands in this scope's stack space (`isUnaffectedStorage`) is the slot the
    ///    saved value is stashed in — unmap it so it is not promoted to a spurious
    ///    `undefined8 v//rsp` local.
    ///
    /// Transcribes the two C++ loops (no name/register/offset
    /// special-casing; the spacebase/join classification and the unaffected/COPY
    /// walk are purely structural).
    pub fn restrict_local(&mut self) {
        use kuna_base::space::spacetype;

        // --- Loop 1: stack-passed params of input-locked locked calls -----------
        let numcalls = self.num_calls();
        for i in 0..numcalls {
            let (input_locked, spacebase_off) = {
                let fc = self.get_call_specs(i);
                (fc.is_input_locked(), fc.get_spacebase_offset())
            };
            if !input_locked {
                continue;
            }
            if spacebase_off == crate::fspec::OFFSET_UNKNOWN {
                continue;
            }
            // Snapshot each locked param's (space, offset, size) before mutating the
            // scope (the &mut getScopeLocal borrow must not alias the &self read of
            // the call spec / arch).
            let numparam = self.get_call_specs(i).proto().num_params();
            let mut marks: Vec<(Rc<kuna_base::space::AddrSpace>, uintb, int4)> = Vec::new();
            for j in 0..numparam {
                let (addr, psize) = {
                    let fc = self.get_call_specs(i);
                    match fc.proto().get_param(j) {
                        Some(p) => (p.get_address(), p.get_size()),
                        None => continue,
                    }
                };
                let space = match addr.get_space() {
                    Some(s) => Rc::clone(s),
                    None => continue,
                };
                match space.get_type() {
                    spacetype::IPTR_SPACEBASE => {
                        // off = space->wrapOffset(spacebaseOffset + addr.getOffset());
                        let off = space
                            .wrap_offset(spacebase_off.wrapping_add(addr.get_offset()));
                        marks.push((space, off, psize));
                    }
                    spacetype::IPTR_JOIN => {
                        // joinRec = glb->findJoin(addr.getOffset());
                        let joinrec = match self.get_arch().manage().find_join(addr.get_offset()) {
                            Ok(jr) => jr,
                            Err(_) => continue,
                        };
                        for k in 0..joinrec.num_pieces() {
                            let piece = joinrec.get_piece(k);
                            let pspace = match &piece.space {
                                Some(s) => Rc::clone(s),
                                None => continue,
                            };
                            if pspace.get_type() == spacetype::IPTR_SPACEBASE {
                                let off = pspace
                                    .wrap_offset(spacebase_off.wrapping_add(piece.offset));
                                marks.push((pspace, off, piece.size as int4));
                            }
                        }
                    }
                    _ => {}
                }
            }
            if let Some(lm) = self.get_scope_local_mut() {
                for (space, off, psize) in marks {
                    // data.getScopeLocal()->markNotMapped(space,off,size,true);
                    lm.mark_not_mapped(&space, off, psize, true);
                }
            }
        }

        // --- Loop 2: saved-register / return-address storage --------------------
        // eiter = data.getFuncProto().effectBegin(); ... iterate saved registers.
        // Snapshot the (size, address) of the saved (non-killedbycall) effects.
        let saved: Vec<(int4, Address)> = self
            .get_func_proto()
            .effect_list()
            .iter()
            .filter(|er| er.get_type() != crate::fspec::effect_type::KILLEDBYCALL)
            .map(|er| (er.get_size(), er.get_address()))
            .collect();

        for (sz, addr) in saved {
            let vn = match self.find_varnode_input(sz, &addr) {
                Some(v) => v,
                None => continue,
            };
            let unaffected = self.vbank().get(vn).map(|v| v.is_unaffected()).unwrap_or(false);
            if !unaffected {
                continue;
            }
            // For each COPY descendant whose output is in this scope's space, unmap it.
            let descend: Vec<OpId> = match self.vbank().get(vn) {
                Some(v) => v.descend_iter().collect(),
                None => continue,
            };
            // Snapshot the (space, offset, size) to unmap before borrowing the scope.
            let mut outs: Vec<(Rc<kuna_base::space::AddrSpace>, uintb, int4)> = Vec::new();
            for op in descend {
                let o = match self.obank().get(op) {
                    Some(o) => o,
                    None => continue,
                };
                if o.code() != OpCode::CPUI_COPY {
                    continue;
                }
                let outvn = match o.get_out() {
                    Some(ov) => ov,
                    None => continue,
                };
                let ov = match self.vbank().get(outvn) {
                    Some(v) => v,
                    None => continue,
                };
                let ospace = Rc::clone(ov.get_space());
                let is_unaff_storage = self
                    .get_scope_local()
                    .map(|lm| lm.is_unaffected_storage(&ospace))
                    .unwrap_or(false);
                if !is_unaff_storage {
                    continue;
                }
                outs.push((ospace, ov.get_offset(), ov.get_size()));
            }
            if let Some(lm) = self.get_scope_local_mut() {
                for (ospace, ooff, osize) in outs {
                    // markNotMapped(outvn->getSpace(),getOffset(),getSize(),false);
                    lm.mark_not_mapped(&ospace, ooff, osize, false);
                }
            }
        }
    }

    /// Recover the stack-frame layout for the function (C++
    /// `ScopeLocal::restructureVarnode`, `varmap.cc:1256`).
    ///
    /// Builds a [`MapState`](crate::varmap::MapState) from the live `(stack, off)`
    /// Varnodes — the data-type hints from each Varnode (`gatherVarnodes`), the
    /// open/array references the alias checker finds (`gatherOpen`), and the
    /// already-mapped Symbols (`gatherSymbols`) — and merges them into a disjoint
    /// cover of Symbols (`restructure`).  This creates the named locals
    /// (`local_8`, `iVar1`, the `int4 i [4]` array) that
    /// `syncVarnodesWithSymbols` then paints onto the Varnodes.
    ///
    /// The C++ tail (`clearUnlockedCategory`/`fakeInputSymbols`/`sortAlias`/
    /// `markUnaliased`/`annotateRawStackPtr`) refines parameter and alias
    /// bookkeeping; the layout-creating core (gather + restructure) is realized
    /// here.
    pub fn restructure_varnode(&mut self, aliasyes: bool) {
        use crate::varmap::MapState;

        // C++ `restructureVarnode` head (varmap.cc:1259): clear out the unlocked
        // auto-recovered stack Symbols so this pass re-derives the layout from the
        // current Varnodes.  Without it a spurious open-array Symbol formed on an
        // early pass (before `RuleStoreVarnode` folded the STORE into a sized stack
        // COPY) survives and `gatherSymbols` re-injects it as a competing fixed
        // array `RangeHint`, overriding the scalar hint the converted Varnode then
        // supplies (regressing condconst.xml "Conditional Constant #10").
        if let Some(lm) = self.get_scope_local_mut() {
            let _ = lm.clear_unlocked_category_negative();
        }

        // (kuna) C++ maps the locked/recovered parameters into the local scope at
        // prototype-attach time (`ProtoStoreSymbol::setInput`, fspec.cc:3174), so
        // they are present in the scope's EntryMap during EVERY restructure/sync
        // pass.  The kuna `ProtoStoreInternal` keeps parameters off the scope, and
        // `link_proto_params` was only run at the final naming pass — so the
        // restructure/sync passes saw NO param entry and `syncVarnodesWithSymbols`
        // took the no-symbol `mapped|addrtied` fallback (keeping `addrforce` on a
        // dead width-N hole-fill that overlaps an int2 stack param).  Materialize
        // the param symbols here so `findOverlap`/sync resolve them like C++ (the
        // overlapping-but-smaller param entry clears `addrforce` once `markUnaliased`
        // paints `nolocalalias`).  Idempotent (`add_param_symbol` skips on overlap).
        // Guarded on a present proto store: a partial-clone / jump-table-recovery
        // Funcdata reaches restructure with no `ProtoStoreInternal` attached.
        if self.get_func_proto().has_store() {
            self.link_proto_params();
        }

        let (space, local_range, param_range, default_unknown, bounds) = {
            let lm = match self.get_scope_local() {
                Some(lm) => lm,
                None => return,
            };
            let space = Rc::clone(lm.get_space_id());
            // MapState clears the proto's param range out of the analysis range;
            // restructureVarnode passes (getRangeTree, proto.getParamRange()).
            let rangetree = lm.range_tree_clone();
            let param_range = self.get_func_proto().get_param_range().clone();
            let types = match self.get_arch().types() {
                Some(t) => t,
                None => return,
            };
            let default_unknown = match types.get_base(1, crate::dtype::type_metatype::TYPE_UNKNOWN)
            {
                Ok(t) => t,
                Err(_) => return,
            };
            let bounds = proto_boundaries(self.get_func_proto());
            (space, rangetree, param_range, default_unknown, bounds)
        };

        let mut state = MapState::new(Rc::clone(&space), &local_range, &param_range, default_unknown);

        // gatherVarnodes(*fd): a data-type hint per live (stack, off) Varnode.
        self.gather_varnodes(&space, &mut state);
        // gatherOpen(*fd): the open/array references the alias checker finds.
        self.gather_open(&space, &mut state, bounds.as_ref());
        // gatherSymbols(maptable[space->getIndex()]): the already-mapped Symbols.
        let hints = match self.get_scope_local() {
            Some(lm) => lm.gather_symbol_hints(),
            None => Vec::new(),
        };
        state.gather_symbols(&hints);

        // overlapProblems = restructure(state).  Clone the type factory `Rc` out
        // first so the &mut ScopeLocal borrow does not alias the &self arch read.
        let types_rc = self.get_arch().types_rc();
        if let (Some(t), Some(lm)) = (types_rc, self.get_scope_local_mut()) {
            let _ = lm.restructure(&mut state, t.as_ref());
        }

        // C++ `restructureVarnode` tail (varmap.cc:1272-1285).  The unlocked-category
        // cleanup / fake-input-symbol synthesis / `markUnaliased` are W4 ScopeLocal
        // surfaces still stubbed (`clearUnlockedCategory`/`fakeInputSymbols`/
        // `markUnaliased` need the `Symbol::category` + alias-block bookkeeping —
        // LOSS recorded).  The one tail step the typed-stack-access rendering needs
        // is the raw-stack-pointer placeholder.
        //
        // A zero-offset use of the stack pointer (e.g. `&v1` passed to a call) gets
        // a `PTRSUB(sp,#0)` placeholder so the data-type system renders `&local`.
        state.sort_alias();
        // C++ `restructureVarnode` tail (varmap.cc:1280-1285).
        // markUnaliased paints `nolocalalias` on every stack Symbol no alias
        // crosses, which is the gate `RuleIndirectCollapse` needs to drop the
        // per-call INDIRECT the heritage stack-guard layer puts on each local.
        // Without it the INDIRECTs persist and pollute the render (the RSP-keystone
        // net-negative).  Cloned out the alias list + alias_block_level so the
        // `&mut ScopeLocal` borrow does not alias the `&self` arch read.
        if aliasyes {
            let alias: Vec<uintb> = state.get_alias().to_vec();
            let abl = self.get_arch().alias_block_level;
            if let Some(lm) = self.get_scope_local_mut() {
                lm.mark_unaliased(&alias, abl);
            }
            // checkUnaliasedReturn(state.getAlias()): if the return value is passed
            // back in this scope's space and no alias reaches it, unmap it (so an
            // unmapped, unaliased return slot is not promoted to a spurious local).
            self.check_unaliased_return(&space, &alias);
        }
        let alias = state.get_alias();
        if alias.first() == Some(&0) {
            self.annotate_raw_stack_ptr(&space);
        }

        // (kuna `framelayout`) Fold this pass's layout into the running union, so a
        // slot a later pass's dataflow folds away is still reportable on the
        // `decompile-all --json` `variables` surface.  Union only -- nothing here
        // feeds back into the IR or the emitted C.
        // Recorded unconditionally (one BTreeMap insert per stack symbol per pass);
        // the `framelayout` option gates the REPORTING, in `extract_variables`,
        // which is the only reader.  The config `Architecture` carrying that option
        // is not reachable from the per-function `ArchContext` seam here.
        self.record_frame_layout_pass(&space);
    }

    /// (kuna `framelayout`) Snapshot the NO_CATEGORY stack Symbols this pass
    /// recovered into the `Funcdata`'s frame-slot union.
    fn record_frame_layout_pass(&self, space: &Rc<kuna_base::space::AddrSpace>) {
        let Some(sl) = self.get_scope_local() else { return };
        let specs = sl
            .database()
            .scope_space_local_var_specs(sl.scope_id(), space.get_index() as usize);
        let mut slots = Vec::new();
        for (name, ct, addr, category) in specs {
            if category != crate::database::symbol_category::NO_CATEGORY {
                continue;
            }
            let off = kuna_base::address::sign_extend(
                addr.get_offset() as i64,
                space.get_addr_size() as i32 * 8 - 1,
            );
            let size = ct.get_size();
            slots.push((off, crate::funcdata::FrameSlot { name, dtype: ct, size }));
        }
        self.record_frame_slots(slots);
    }

    /// C++ `ScopeLocal::checkUnaliasedReturn` (`varmap.cc:414-428`): if the return
    /// value is passed back in a location held by this scope's (stack) space, assume
    /// it is unmapped — unless a specific alias reaches into the location — and mark
    /// the range as not mapped.
    ///
    /// `alias` is the sorted list of alias offsets into the space (from the
    /// `MapState`).  Called from [`restructure_varnode`] when `aliasyes` is set.
    pub fn check_unaliased_return(
        &mut self,
        space: &Rc<kuna_base::space::AddrSpace>,
        alias: &[uintb],
    ) {
        let retop = match self.get_first_return_op() {
            Some(r) => r,
            None => return,
        };
        let invn = {
            let o = match self.obank().get(retop) {
                Some(o) => o,
                None => return,
            };
            if o.num_input() < 2 {
                return;
            }
            match o.get_in(1) {
                Some(v) => v,
                None => return,
            }
        };
        let (vn_space, vn_off, vn_size) = {
            let v = match self.vbank().get(invn) {
                Some(v) => v,
                None => return,
            };
            (Rc::clone(v.get_space()), v.get_offset(), v.get_size())
        };
        if !(Rc::ptr_eq(&vn_space, space) || vn_space.get_index() == space.get_index()) {
            return;
        }
        // Assume vn is mapped.  Cannot check vn->isMapped() as we are mid-restructure.
        if Self::alias_reaches_return_slot(alias, vn_off, vn_size) {
            return; // Alias into return storage, don't continue.
        }
        // markNotMapped(space, vn->getOffset(), vn->getSize(), false);
        if let Some(lm) = self.get_scope_local_mut() {
            lm.mark_not_mapped(space, vn_off, vn_size, false);
        }
    }

    /// The alias-overlap decision of C++ `ScopeLocal::checkUnaliasedReturn`
    /// (`varmap.cc:421-425`): does any alias offset reach into the return slot
    /// `[off, off+size)`?  `lower_bound(alias, off)` finds the first alias `>= off`;
    /// if that alias is `<= off+size-1` it lands inside the slot.  When it does, the
    /// slot stays mapped (aliased); otherwise the caller unmaps it.
    fn alias_reaches_return_slot(alias: &[uintb], off: uintb, size: int4) -> bool {
        // iter = lower_bound(alias.begin(), alias.end(), off);
        let pos = alias.partition_point(|&a| a < off);
        if pos < alias.len() {
            // Alias is greater than or equal to vn offset.
            let last = off.wrapping_add(size as uintb).wrapping_sub(1);
            return alias[pos] <= last;
        }
        false
    }

    /// C++ `ScopeLocal::applyTypeRecommendations` (`varmap.cc:1574`): for each
    /// pending `(addr, type)` recommendation, if an input Varnode exists at that
    /// address, lock the recommended data-type onto it.
    ///
    /// Driven by `ActionInferTypes::apply` (coreaction.cc:5654), at the head of
    /// each type-propagation pass.  The merged tree's only recommendation source is
    /// the `this`-pointer collection (`collectNameRecs`/`prepareThisPointer`); on a
    /// recommendation-free function this is a no-op.
    pub fn apply_type_recommendations(&mut self) {
        let recs: Vec<(int4, Address, Rc<crate::dtype::Datatype>)> = match self.get_scope_local() {
            Some(lm) => lm
                .type_recommendations()
                .iter()
                .map(|tr| (tr.data_type.get_size(), tr.addr.clone(), Rc::clone(&tr.data_type)))
                .collect(),
            None => return,
        };
        for (sz, addr, dt) in recs {
            // vn = fd->findVarnodeInput(dt->getSize(), addr);
            if let Some(vn) = self.find_varnode_input(sz, &addr) {
                // vn->updateType(dt, true, false);
                let high = self.vbank().get(vn).and_then(|v| v.get_high());
                let changed = self
                    .vbank_mut()
                    .get_mut(vn)
                    .map(|v| v.update_type_locked(Rc::clone(&dt), true, false))
                    .unwrap_or(false);
                if changed {
                    if let Some(h) = high {
                        if let Some(hh) = self.high_bank_mut().get_mut(h) {
                            hh.type_dirty();
                        }
                    }
                }
            }
        }
    }

    /// C++ `ScopeLocal::annotateRawStackPtr` (`varmap.cc:386`): for every read of
    /// the input stack pointer by a *non-additive* p-code op, splice in a
    /// `PTRSUB(sp, #0)` placeholder so the data-type system treats the raw stack
    /// pointer as a zero-offset reference into the stack frame (and renders `&local`
    /// rather than the bare stack register).
    ///
    /// Reached from [`Funcdata::restructure_varnode`] when a zero-offset use of the
    /// stack pointer exists (the first alias offset is 0).
    pub fn annotate_raw_stack_ptr(&mut self, space: &Rc<kuna_base::space::AddrSpace>) {
        if !self.has_type_recovery_started() {
            return;
        }
        let sp_vn = match self.find_spacebase_input(space) {
            Some(v) => v,
            None => return,
        };
        let sp_size = self.vbank().get(sp_vn).map(|v| v.get_size()).unwrap_or(0);

        // Collect the non-additive, non-special-non-call reader ops.
        let descend: Vec<OpId> = match self.vbank().get(sp_vn) {
            Some(v) => v.descend_iter().collect(),
            None => return,
        };
        let mut ref_ops: Vec<OpId> = Vec::new();
        for op in descend {
            let o = match self.obank().get(op) {
                Some(o) => o,
                None => continue,
            };
            if o.get_eval_type() == crate::op::pcodeop_flags::special && !o.is_call() {
                continue;
            }
            match o.code() {
                OpCode::CPUI_INT_ADD | OpCode::CPUI_PTRSUB | OpCode::CPUI_PTRADD => continue,
                _ => {}
            }
            ref_ops.push(op);
        }

        // For each, splice PTRSUB(spVn, #0) before it and rewire the slot.
        for op in ref_ops {
            // slot = op->getSlot(spVn);
            let slot = match self.obank().get(op).map(|o| o.get_slot(sp_vn)) {
                Some(s) if s >= 0 => s,
                _ => continue,
            };
            // ptrsub = fd->newOpBefore(op, PTRSUB, spVn, fd->newConstant(spVn->getSize(),0));
            let zero = self.new_constant(sp_size, 0);
            let ptrsub = self.new_op_before(op, OpCode::CPUI_PTRSUB, sp_vn, zero, None);
            // opSetInput(op, ptrsub->getOut(), slot);
            if let Some(out) = self.obank().get(ptrsub).and_then(|o| o.get_out()) {
                let _ = self.op_set_input(op, out, slot);
            }
        }
    }

    /// C++ `MapState::gatherVarnodes` (`varmap.cc:1122`): add a data-type hint for
    /// every Varnode stored in the stack space, classified by its defining op.
    fn gather_varnodes(&self, space: &Rc<kuna_base::space::AddrSpace>, state: &mut crate::varmap::MapState) {
        let types_rc = match self.get_arch().types_rc() {
            Some(t) => t,
            None => return,
        };
        let types = types_rc.as_ref();
        let vlist: Vec<VarnodeId> = self
            .vbank()
            .iter_loc()
            .filter(|vn| {
                self.vbank()
                    .get(*vn)
                    .and_then(|v| v.get_addr().get_space().map(|s| s.get_index()))
                    == Some(space.get_index())
            })
            .collect();
        for vn in vlist {
            let v = match self.vbank().get(vn) {
                Some(v) => v,
                None => continue,
            };
            if v.is_free() {
                continue;
            }
            let offset = v.get_offset();
            let vtype = Rc::clone(v.get_type());
            if !v.is_written() {
                if self.is_read_active(vn) {
                    state.add_fixed_type_pub(offset, vtype, 0, types);
                }
                continue;
            }
            let op = match v.get_def() {
                Some(o) => o,
                None => continue,
            };
            let opc = self.obank().get(op).map(|o| o.code());
            match opc {
                Some(OpCode::CPUI_INDIRECT) => {
                    let invn = self.obank().get(op).and_then(|o| o.get_in(0));
                    let vn_addr = v.get_addr().clone();
                    let invn_addr = invn.and_then(|iv| self.vbank().get(iv).map(|x| x.get_addr().clone()));
                    let diff = invn_addr.map(|a| !addr_eq(&vn_addr, &a)).unwrap_or(true);
                    if diff || self.is_read_active(vn) {
                        state.add_fixed_type_pub(offset, vtype, 0, types);
                    }
                }
                Some(OpCode::CPUI_MULTIEQUAL) => {
                    let n = self.obank().get(op).map(|o| o.num_input()).unwrap_or(0);
                    let vn_addr = v.get_addr().clone();
                    let mut all_same = true;
                    for k in 0..n {
                        let invn = self.obank().get(op).and_then(|o| o.get_in(k));
                        let ia = invn.and_then(|iv| self.vbank().get(iv).map(|x| x.get_addr().clone()));
                        if ia.map(|a| !addr_eq(&vn_addr, &a)).unwrap_or(true) {
                            all_same = false;
                            break;
                        }
                    }
                    if !all_same || self.is_read_active(vn) {
                        state.add_fixed_type_pub(offset, vtype, 0, types);
                    }
                }
                Some(OpCode::CPUI_COPY) => {
                    let in0 = self.obank().get(op).and_then(|o| o.get_in(0));
                    let is_const = in0
                        .and_then(|iv| self.vbank().get(iv).map(|x| x.is_constant()))
                        .unwrap_or(false);
                    let fl = if is_const { crate::varmap::COPY_CONSTANT } else { 0 };
                    state.add_fixed_type_pub(offset, vtype, fl, types);
                }
                Some(OpCode::CPUI_PIECE) => {
                    // C++ varmap.cc:1165 — treat PIECE as two COPYs.  Each
                    // constituent input contributes a fixed-type hint at its own
                    // sub-address, unless the input simply copies to the same
                    // storage.  Then, if `vn` itself is read-actively, add the
                    // whole-output hint too.
                    let vn_addr = v.get_addr().clone();
                    let slot: int4 = if vn_addr.is_big_endian() { 0 } else { 1 };
                    let in_first = self.obank().get(op).and_then(|o| o.get_in(slot));
                    let (first_addr, first_sz, first_ty) = match in_first
                        .and_then(|iv| self.vbank().get(iv))
                    {
                        Some(fv) => (fv.get_addr().clone(), fv.get_size(), Rc::clone(fv.get_type())),
                        None => {
                            // Degenerate: fall back to the output hint (C++ would
                            // dereference a null here; the IR never produces it).
                            state.add_fixed_type_pub(offset, vtype, 0, types);
                            continue;
                        }
                    };
                    if !addr_eq(&first_addr, &vn_addr) {
                        state.add_fixed_type_pub(vn_addr.get_offset(), first_ty, 0, types);
                    }
                    let second_addr = &vn_addr + first_sz as i64;
                    let in_second = self.obank().get(op).and_then(|o| o.get_in(1 - slot));
                    if let Some(sv) = in_second.and_then(|iv| self.vbank().get(iv)) {
                        let sv_addr = sv.get_addr().clone();
                        let sv_ty = Rc::clone(sv.get_type());
                        if !addr_eq(&sv_addr, &second_addr) {
                            state.add_fixed_type_pub(second_addr.get_offset(), sv_ty, 0, types);
                        }
                    }
                    if self.is_read_active(vn) {
                        state.add_fixed_type_pub(offset, vtype, 0, types);
                    }
                }
                Some(OpCode::CPUI_SUBPIECE) => {
                    // C++ varmap.cc:1188 — a SUBPIECE that just truncates into the
                    // same storage is not an active write.  Compute the truncated
                    // input address: addr = in0.getAddr() + trunc, where trunc is
                    // computed big/little-endian from the input/output sizes and the
                    // truncation amount in[1].
                    let (in0_addr, in0_sz) = match self
                        .obank()
                        .get(op)
                        .and_then(|o| o.get_in(0))
                        .and_then(|iv| self.vbank().get(iv))
                    {
                        Some(iv) => (iv.get_addr().clone(), iv.get_size()),
                        None => {
                            state.add_fixed_type_pub(offset, vtype, 0, types);
                            continue;
                        }
                    };
                    let vn_sz = v.get_size();
                    let trunc_off = self
                        .obank()
                        .get(op)
                        .and_then(|o| o.get_in(1))
                        .and_then(|iv| self.vbank().get(iv).map(|x| x.get_offset()))
                        .unwrap_or(0);
                    let trunc: int4 = if in0_addr.is_big_endian() {
                        in0_sz - vn_sz - trunc_off as int4
                    } else {
                        trunc_off as int4
                    };
                    let trunc_addr = &in0_addr + trunc as i64;
                    if !addr_eq(&trunc_addr, &v.get_addr().clone()) || self.is_read_active(vn) {
                        state.add_fixed_type_pub(offset, vtype, 0, types);
                    }
                }
                _ => {
                    state.add_fixed_type_pub(offset, vtype, 0, types);
                }
            }
        }
    }

    /// C++ `MapState::isReadActive` (`varmap.cc:1088`): does this Varnode have a
    /// descendant that reads it as an active (non-marker, non-same-storage) use?
    fn is_read_active(&self, vn: VarnodeId) -> bool {
        let vn_addr = match self.vbank().get(vn) {
            Some(v) => v.get_addr().clone(),
            None => return false,
        };
        let descend: Vec<OpId> = match self.vbank().get(vn) {
            Some(v) => v.descend_iter().collect(),
            None => return false,
        };
        for op in descend {
            let o = match self.obank().get(op) {
                Some(o) => o,
                None => continue,
            };
            if o.is_marker() {
                let out_addr = o.get_out().and_then(|ov| self.vbank().get(ov).map(|x| x.get_addr().clone()));
                if out_addr.map(|a| !addr_eq(&vn_addr, &a)).unwrap_or(true) {
                    return true;
                }
            } else {
                match o.code() {
                    OpCode::CPUI_PIECE => {
                        // C++ varmap.cc:1099 — a PIECE that merely copies `vn` back
                        // into the SAME storage (the slot whose sub-address equals
                        // `vn`'s) is NOT an active read: it is active iff `vn`'s
                        // address differs from the computed slot address below.
                        let out_addr = match o
                            .get_out()
                            .and_then(|ov| self.vbank().get(ov).map(|x| x.get_addr().clone()))
                        {
                            Some(a) => a,
                            None => return true,
                        };
                        let slot: int4 = if out_addr.is_big_endian() { 0 } else { 1 };
                        let slot_vn = o.get_in(slot);
                        let addr = if slot_vn != Some(vn) {
                            let in_sz = slot_vn
                                .and_then(|iv| self.vbank().get(iv).map(|x| x.get_size()))
                                .unwrap_or(0);
                            &out_addr + in_sz as i64
                        } else {
                            out_addr
                        };
                        if !addr_eq(&vn_addr, &addr) {
                            return true;
                        }
                    }
                    OpCode::CPUI_SUBPIECE => {} // type info comes from output; ignore
                    _ => return true,
                }
            }
        }
        false
    }

    /// C++ `MapState::gatherOpen` (`varmap.cc:1004`): run the alias checker over
    /// the IR and add an open/array reference for each pointer base it finds.
    fn gather_open(
        &mut self,
        space: &Rc<kuna_base::space::AddrSpace>,
        state: &mut crate::varmap::MapState,
        bounds: Option<&crate::varmap::ProtoBoundaries>,
    ) {
        // checker.gather(&fd, spaceid, false): build the alias base/offset lists.
        let mut access = FuncdataAliasAccess { fd: self, exempt_scramble: false };
        state.checker_mut().gather(Rc::clone(space), bounds, false, &mut access);

        // For each AddBase, add an open range at its alias offset.
        let addbase: Vec<(uintb, Option<VarnodeId>, VarnodeId)> = {
            let checker = state.checker_mut();
            let alias: Vec<uintb> = checker.get_alias().to_vec();
            checker
                .get_add_base()
                .iter()
                .enumerate()
                .map(|(i, ab)| (alias[i], ab.index, ab.base))
                .collect()
        };
        for (offset, index, base) in addbase {
            // ct = base->getType(); if PTR -> getPtrTo, strip arrays; else None.
            let mut ct: Option<Rc<crate::dtype::Datatype>> =
                self.vbank().get(base).map(|v| Rc::clone(v.get_type()));
            if let Some(c) = &ct {
                if c.get_metatype() == crate::dtype::type_metatype::TYPE_PTR {
                    let mut inner = c.get_ptr_to();
                    while let Some(i) = &inner {
                        if i.get_metatype() == crate::dtype::type_metatype::TYPE_ARRAY {
                            inner = i.get_array_base();
                        } else {
                            break;
                        }
                    }
                    ct = inner;
                } else {
                    ct = None; // Do unknown array
                }
            }
            // If there is an index Varnode, assume at least the 4 values [0,3].
            let min_items = if index.is_some() { 3 } else { -1 };
            state.add_range_pub(offset, ct, 0, crate::varmap::RangeType::Open, min_items);
        }
        // The LoadGuard/StoreGuard hint loops (C++ varmap.cc:1241-1248): each
        // heritage-recorded indexed LOAD/STORE guard contributes an open hint.
        // A guard only carries a bound once `option loadguardrange` has run the
        // ValueSet refinement (`addGuard` bails on `step == 0`), so with the
        // option off these loops add nothing.
        let load_guards: Vec<crate::heritage::LoadGuard> = self.get_load_guards().to_vec();
        for guard in &load_guards {
            self.add_guard(state, guard, OpCode::CPUI_LOAD);
        }
        let store_guards: Vec<crate::heritage::LoadGuard> = self.get_store_guards().to_vec();
        for guard in &store_guards {
            self.add_guard(state, guard, OpCode::CPUI_STORE);
        }
    }

    /// Convert a LoadGuard into an open RangeHint, attempting to make use of
    /// any data-type or index information (C++ `MapState::addGuard`,
    /// `varmap.cc:1003`).  `opc` is the expected op-code (CPUI_LOAD or
    /// CPUI_STORE); the guard is ignored if its op died or its range was never
    /// refined (`step == 0`).
    fn add_guard(
        &self,
        state: &mut crate::varmap::MapState,
        guard: &crate::heritage::LoadGuard,
        opc: OpCode,
    ) {
        if !guard.is_valid(self, opc) {
            return;
        }
        let mut step = guard.get_step();
        if step == 0 {
            return; // No definitive sign of array access
        }
        let op = guard.op;
        let in1 = match self.obank().get(op).and_then(|o| o.get_in(1)) {
            Some(v) => v,
            None => return,
        };
        let mut ct: Rc<crate::dtype::Datatype> = match self.vbank().get(in1) {
            Some(v) => Rc::clone(v.get_type_read_facing(op)),
            None => return,
        };
        if ct.get_metatype() == crate::dtype::type_metatype::TYPE_PTR {
            let mut inner = match ct.get_ptr_to() {
                Some(i) => i,
                None => return,
            };
            while inner.get_metatype() == crate::dtype::type_metatype::TYPE_ARRAY {
                inner = match inner.get_array_base() {
                    Some(b) => b,
                    None => break,
                };
            }
            ct = inner;
        }
        let out_size = if opc == OpCode::CPUI_STORE {
            // The Varnode being stored.
            match self
                .obank()
                .get(op)
                .and_then(|o| o.get_in(2))
                .and_then(|v| self.vbank().get(v))
                .map(|v| v.get_size())
            {
                Some(s) => s,
                None => return,
            }
        } else {
            // The Varnode being loaded.
            match self
                .obank()
                .get(op)
                .and_then(|o| o.get_out())
                .and_then(|v| self.vbank().get(v))
                .map(|v| v.get_size())
            {
                Some(s) => s,
                None => return,
            }
        };
        if out_size != step {
            // LOAD size doesn't match step: a field in an array of structures
            // or something more unusual.
            if out_size > step || (step % out_size) != 0 {
                return;
            }
            // Since the LOAD size divides the step and we want to preserve the
            // arrayness, we pretend we have an array of the LOAD's size.
            step = out_size;
        }
        if ct.get_align_size() != step {
            // Make sure the data-type matches our step size.
            if step > 8 {
                return; // Don't manufacture primitives bigger than 8-bytes
            }
            let types = match self.get_arch().types() {
                Some(t) => t,
                None => return,
            };
            ct = match types.get_base(step, crate::dtype::type_metatype::TYPE_UNKNOWN) {
                Ok(t) => t,
                Err(_) => return,
            };
        }
        if guard.is_range_locked() {
            // C++ uintb arithmetic (wraps); a locked range keeps max >= min.
            let min_items = ((guard.get_maximum().wrapping_sub(guard.get_minimum()).wrapping_add(1))
                / (step as uintb)) as int4;
            state.add_range_pub(
                guard.get_minimum(),
                Some(ct),
                0,
                crate::varmap::RangeType::Open,
                min_items - 1,
            );
        } else {
            state.add_range_pub(guard.get_minimum(), Some(ct), 0, crate::varmap::RangeType::Open, 3);
        }
    }

    /// Update Varnode flags and data-types from the local symbol map (C++
    /// `Funcdata::syncVarnodesWithSymbols`, `funcdata_varnode.cc:959`).
    ///
    /// Walks every Varnode in the local scope's space; for each, resolves the
    /// overlapping mapped Symbol and paints the `mapped`/`addrtied`/`addrforce`/
    /// `nolocalalias` flags and (when `update_datatypes`) the recovered type.
    /// Returns `true` if any Varnode changed.
    pub fn sync_varnodes_with_symbols(
        &mut self,
        update_datatypes: bool,
        unmapped_alias_check: bool,
    ) -> bool {
        let space = match (self.get_scope_local(), self.get_arch().types()) {
            (Some(lm), Some(_)) => Rc::clone(lm.get_space_id()),
            _ => return false,
        };
        let mut update_occurred = false;

        // Iterate the location set restricted to the stack space, grouped by
        // (size, addr) — each group is one syncVarnodesWithSymbol run.
        let all: Vec<VarnodeId> = self
            .vbank()
            .iter_loc()
            .filter(|vn| {
                self.vbank()
                    .get(*vn)
                    .and_then(|v| v.get_addr().get_space().map(|s| s.get_index()))
                    == Some(space.get_index())
            })
            .collect();

        // Group consecutive (size, offset) exemplars.  beginLoc/endLoc in C++
        // walks one (size,addr) block per outer step; here we resolve the symbol
        // once per exemplar group and apply to every member.
        let mut i = 0usize;
        while i < all.len() {
            let exemplar = all[i];
            let (ex_addr, ex_size) = match self.vbank().get(exemplar) {
                Some(v) => (v.get_addr().clone(), v.get_size()),
                None => {
                    i += 1;
                    continue;
                }
            };

            // Collect this (size, addr) group.
            let mut group: Vec<VarnodeId> = Vec::new();
            while i < all.len() {
                let vn = all[i];
                let same = match self.vbank().get(vn) {
                    Some(v) => {
                        v.get_size() == ex_size
                            && v.get_addr().get_offset() == ex_addr.get_offset()
                    }
                    None => false,
                };
                if !same {
                    break;
                }
                group.push(vn);
                i += 1;
            }

            // Resolve the overlapping symbol for the exemplar.
            let overlap = self.get_scope_local().and_then(|lm| {
                self.get_arch()
                    .types()
                    .and_then(|t| lm.sync_overlap(&ex_addr, ex_size, t))
            });

            let mut ct: Option<Rc<crate::dtype::Datatype>> = None;
            let fl: uint4;
            // Whether the resolved SymbolEntry is a join-piece sub-entry (precislo/
            // precishi extra-flags, NOT `mapped`) — the extra maps `Scope::addMap`
            // registers per piece of a join-address Symbol (database.cc:1161-1180).
            // The kuna addrtied pre-tie must NOT fire for such a partial piece: the
            // covered Varnode is a SUBPIECE-extracted field of the join symbol, and
            // C++'s heritage pre-tie only ties whole-symbol storage (the `mapped`
            // entry), never these piece sub-entries.  Without this gate the field
            // read gets addrtied and is forced explicit as a copymarker self-assign
            // (`d.field_b = d.field_b;`) instead of staying implied.
            let mut entry_is_join_piece = false;
            if let Some(ov) = overlap {
                let mut flags = ov.all_flags;
                entry_is_join_piece = (ov.extraflags
                    & (varnode_flags::precislo | varnode_flags::precishi))
                    != 0;
                if ov.entry_size >= ex_size {
                    if update_datatypes {
                        if let Some(t) = &ov.sized_type {
                            if t.get_metatype() != crate::dtype::type_metatype::TYPE_UNKNOWN {
                                ct = Some(Rc::clone(t));
                            }
                        }
                    }
                } else {
                    // Overlapping but not containing: drop typelock/namelock.
                    flags &= !(varnode_flags::typelock | varnode_flags::namelock);
                }
                fl = flags;
            } else {
                // No symbol found.  C++ funcdata_varnode.cc:992-1004: if the range
                // is in scope (`rangetree.inRange`) there *should* be a symbol — the
                // fallback marks `mapped | addrtied`.  (A stack-passed struct param's
                // field read used to land here because its join-piece SymbolEntry was
                // not yet registered in the stack EntryMap; the `Scope::addMap` join
                // arm now registers it, so the `Some(ov)` arm above resolves it to the
                // param Symbol instead — Stack spill #1.)
                let in_scope = self
                    .get_scope_local()
                    .map(|lm| lm.in_scope(&ex_addr, ex_size))
                    .unwrap_or(false);
                if in_scope {
                    fl = varnode_flags::mapped | varnode_flags::addrtied;
                } else if unmapped_alias_check {
                    // C++ funcdata_varnode.cc:999-1001:
                    //   fl = lm->isUnmappedUnaliased(vnexemplar) ? nolocalalias : 0;
                    // The faithful flip is [`ScopeLocal::is_unmapped_unaliased`]
                    // (varmap.rs).  Wiring it here forwards the MIPS gp spill (Gp Test
                    // #2 GAINED: the unmapped, unaliased gp-spill slot gets
                    // `nolocalalias`, RuleIndirectCollapse drops the per-call INDIRECT,
                    // and the constant reaches the GOT loads → `printf("Hello",a0)`),
                    // and — now that `protect_switch_paths` is byte-faithful (the
                    // out-of-bounds `getIn(1)` panic in `switch_is_delayed_constant` /
                    // the walk's eager input read is fixed) — ALL 6 switch datatests
                    // (switchind 16/16, switchmulti 9/9, switchloop 10/10, switchhide,
                    // switchreturn, ifswitch) HOLD.
                    //
                    // BLOCKED on ONE residual: Gp Test #1 (`populate(v1)`).  The flip
                    // is byte-faithful (C++ flips the same -0x10 gp slot), but in rust
                    // the resulting alias change drives a HighVariable OVER-MERGE: the
                    // `&v1` stack-address COPY merges into the `a0` input parameter
                    // (param typed `xunknown1 *a0` instead of C++'s `xunknown4 a0`),
                    // so `a0 = v1; populate(a0)` is emitted explicitly where C++ keeps
                    // the populate-arg and printf-arg `a0` as distinct single-use SSA
                    // versions and implies the COPY → `populate(v1)`.  This is a
                    // DOWNSTREAM merge/HighVariable divergence (the LOSS-247 stack-
                    // COPY-into-input family), not in either of these two pieces.
                    // NEXT-LOCUS: stop the speculative merge of the `&v1` address COPY
                    // into the `a0` input param (merge.rs / ActionMerge* input-merge
                    // eligibility under `nolocalalias`), THEN flip this on:
                    //   let unaliased = self.get_scope_local().and_then(|lm|
                    //       ex_addr.get_space().map(|spc|
                    //           lm.is_unmapped_unaliased(&spc, ex_addr.get_offset())))
                    //       .unwrap_or(false);
                    //   fl = if unaliased { varnode_flags::nolocalalias } else { 0 };
                    let unaliased = self.get_scope_local().and_then(|lm|
                        ex_addr.get_space().map(|spc|
                            lm.is_unmapped_unaliased(&spc, ex_addr.get_offset())))
                        .unwrap_or(false);
                    fl = if unaliased { varnode_flags::nolocalalias } else { 0 };
                } else {
                    fl = 0;
                }
            }

            if self.sync_varnodes_with_symbol(&group, fl, ct.as_ref(), entry_is_join_piece) {
                update_occurred = true;
            }
        }

        update_occurred
    }

    /// Apply resolved flags/type to one `(size, addr)` group of Varnodes (C++
    /// `Funcdata::syncVarnodesWithSymbol`, `funcdata_varnode.cc:889`).
    fn sync_varnodes_with_symbol(
        &mut self,
        group: &[VarnodeId],
        fl: uint4,
        ct: Option<&Rc<crate::dtype::Datatype>>,
        entry_is_join_piece: bool,
    ) -> bool {
        // (kuna pre-tie) The merged tree has no pass that pre-ties stack storage:
        // the C++ ties address-tied stack storage earlier, via the heritage /
        // storage-class bookkeeping and the `setSymbolEntry` attach (only the
        // return register is pre-tied here, by `mark_output_storage_addr_tied`).
        // The C++ `syncVarnodesWithSymbol` mask therefore relies on the bit being
        // ALREADY set, and the mask itself is "CLEAR-but-never-SET addrtied".  To
        // stay byte-faithful to that invariant we replicate the missing pre-tie as
        // a SEPARATE step here — when the resolved Symbol is itself address-tied,
        // we set `addrtied` on the group's Varnodes (the same end-state
        // `setSymbolEntry` reaches in C++) — and then run the verbatim C++ mask,
        // which (because the bit is now part of `fl`) correctly excludes addrtied
        // and never SETs it.  The invariant the verifier flagged is restored: the
        // mask logic can only CLEAR addrtied, exactly as `funcdata_varnode.cc:1077`.
        //
        // EXCEPTION: when the resolved entry is a *join piece* sub-entry (the extra
        // maps `Scope::addMap` registers per piece of a join-address Symbol,
        // database.cc:1161-1180), do NOT pre-tie.  Those entries exist only so a
        // piece-location `findOverlap` resolves to the unified Symbol; the covered
        // Varnode is a SUBPIECE-extracted field, not whole-symbol storage.  In C++
        // the heritage pre-tie never ties such a partial access, and the sync mask
        // (which can only CLEAR addrtied) leaves it untied — so the field read stays
        // implied (`return a + d.field_b`) rather than becoming an explicit
        // copymarker self-assign (`d.field_b = d.field_b`).
        if (fl & varnode_flags::addrtied) != 0 && !entry_is_join_piece {
            for &vn in group {
                if let Some(v) = self.vbank_mut().get_mut(vn) {
                    if !v.is_free() {
                        v.set_flags_pub(varnode_flags::addrtied);
                    }
                }
            }
        }

        // C++ `syncVarnodesWithSymbol` mask (funcdata_varnode.cc:1077): start with
        // `mapped`; `addrtied`/`addrforce` may be CLEARED but never SET (the bit is
        // pre-tied above); `nolocalalias` may be SET but not cleared.
        let mut mask = varnode_flags::mapped;
        if (fl & varnode_flags::addrtied) == 0 {
            mask |= varnode_flags::addrtied | varnode_flags::addrforce;
        }
        if (fl & varnode_flags::nolocalalias) != 0 {
            mask |= varnode_flags::nolocalalias | varnode_flags::addrforce;
        }
        let fl = fl & mask;

        let mut update_occurred = false;
        for &vn in group {
            let (is_free, vnflags, high) = match self.vbank().get(vn) {
                Some(v) => (v.is_free(), v.get_flags(), v.get_high()),
                None => continue,
            };
            if is_free {
                continue;
            }
            // (We do not model the dynamic mapentry branch — restructureVarnode's
            // symbols are all address-tied, never dynamic SymbolEntries.)
            if (vnflags & mask) != fl {
                update_occurred = true;
                if let Some(v) = self.vbank_mut().get_mut(vn) {
                    v.set_flags_pub(fl);
                    v.clear_flags_pub((!fl) & mask);
                }
            }
            if let Some(ct) = ct {
                let changed = self
                    .vbank_mut()
                    .get_mut(vn)
                    .map(|v| v.update_type(Rc::clone(ct)))
                    .unwrap_or(false);
                if changed {
                    update_occurred = true;
                    if let Some(h) = high {
                        if let Some(hh) = self.high_bank_mut().get_mut(h) {
                            hh.type_dirty();
                        }
                    }
                }
            }
        }
        update_occurred
    }
}

/// Compare two storage addresses by space index + offset (the C++
/// `vn->getAddr() != op->getOut()->getAddr()` value comparison).
fn addr_eq(a: &Address, b: &Address) -> bool {
    let sa = a.get_space().map(|s| s.get_index());
    let sb = b.get_space().map(|s| s.get_index());
    sa == sb && a.get_offset() == b.get_offset()
}

/// Extract the four `deriveBoundaries` inputs from a function prototype (C++
/// `AliasChecker::deriveBoundaries`'s reads of `proto.hasModel()` + the first
/// local / last param `Range`s).  `None` when the prototype has no model.
fn proto_boundaries(proto: &crate::fspec::FuncProto) -> Option<crate::varmap::ProtoBoundaries> {
    if !proto.has_model() {
        return None;
    }
    let localrange = proto.get_local_range();
    let paramrange = proto.get_param_range();
    let local = localrange.get_first_range();
    let param = paramrange.get_last_range();
    let has_local_first = local.is_some();
    let has_param_last = param.is_some();
    let param_last = param.map(|r| r.get_last()).unwrap_or(0);
    let param_first = paramrange.get_first_range().map(|r| r.get_first()).unwrap_or(0);
    Some(crate::varmap::ProtoBoundaries {
        has_local_first,
        has_param_last,
        param_last,
        param_first,
    })
}

/// The live-IR realization of the [`AliasGatherAccess`](crate::varmap::AliasGatherAccess):
/// `findSpacebaseInput` / `gatherAdditiveBase` / `gatherOffset` over the function's
/// def/use graph (C++ `AliasChecker`'s Varnode walks, `varmap.cc:736-858`).
pub(crate) struct FuncdataAliasAccess<'a> {
    fd: &'a Funcdata,
    /// (kuna) `cookiescramble`: exempt an `INT_XOR` of the stack pointer from
    /// the escape-site scan.  Set only for the call-site input-trial checker;
    /// the local-layout gather (`gather_open`) always answers upstream.
    exempt_scramble: bool,
}

impl crate::varmap::AliasGatherAccess for FuncdataAliasAccess<'_> {
    fn find_spacebase_input(
        &self,
        space: &Rc<kuna_base::space::AddrSpace>,
    ) -> Option<VarnodeId> {
        self.fd.find_spacebase_input(space)
    }

    fn gather_additive_base(
        &mut self,
        startvn: VarnodeId,
        addbase: &mut Vec<crate::varmap::AddBase>,
    ) {
        self.fd.gather_additive_base(startvn, addbase, self.exempt_scramble);
    }

    fn gather_offset(&mut self, vn: VarnodeId) -> uintb {
        self.fd.gather_offset(vn)
    }
}

impl Funcdata {
    /// Build the live-IR [`AliasGatherAccess`](crate::varmap::AliasGatherAccess) over
    /// this function's def/use graph, so an [`AliasChecker`](crate::varmap::AliasChecker)
    /// can lazily gather stack-pointer aliases (`findSpacebaseInput`/
    /// `gatherAdditiveBase`/`gatherOffset`).  Used by the call-site input recovery
    /// (`checkInputTrialUse`'s spacebase branch).
    pub(crate) fn alias_gather_access(&self) -> FuncdataAliasAccess<'_> {
        FuncdataAliasAccess { fd: self, exempt_scramble: self.get_arch().cookie_scramble }
    }

    /// Build the deferred local-alias checker the call-site input recovery uses
    /// (C++ `ActionActiveParam`'s `aliascheck.gather(&data, getStackSpace(), true)`).
    ///
    /// The boundaries are derived from this function's prototype; the gather is
    /// deferred (the Varnode walk runs lazily on the first `hasLocalAlias`).
    /// Returns `None` if there is no stack space (no possible local alias).
    pub(crate) fn build_alias_checker_deferred(&self) -> Option<crate::varmap::AliasChecker> {
        let stackspc = self.get_arch().manage().get_stack_space().map(Rc::clone)?;
        let bounds = proto_boundaries(self.get_func_proto());
        let mut checker = crate::varmap::AliasChecker::new();
        let mut access = self.alias_gather_access();
        checker.gather(stackspc, bounds.as_ref(), true, &mut access);
        Some(checker)
    }

    /// Mark a stack range as not mapped to a local symbol (C++
    /// `ScopeLocal::markNotMapped`, reached via `getScopeLocal()` from
    /// `buildInputFromTrials` for a stack-passed parameter).
    ///
    /// STUB(W4 ScopeLocal::markNotMapped): the `RangeHint`/`markNotMapped` range
    /// surface on `ScopeLocal` is not yet ported (LOSS recorded in the structured
    /// output).  The register-parameter recovery path the call-rendering datatests
    /// exercise never reaches this (only stack-passed params do); a stack-passed
    /// parameter simply is not marked unmapped here, which at worst leaves a stack
    /// slot eligible for a redundant local — it does not corrupt the call list.
    pub(crate) fn scope_local_mark_not_mapped(
        &mut self,
        spc: &Rc<kuna_base::space::AddrSpace>,
        first: uintb,
        sz: int4,
        param: bool,
    ) {
        // C++ `data.getScopeLocal()->markNotMapped(spc, first, sz, param)`.
        // ScopeLocal::markNotMapped IS ported (varmap.rs); delegate so a STORE the
        // internal-storage pass flagged unmapped removes its slot's Symbol and
        // range (the alias gather then can't propagate through the gap).
        if let Some(lm) = self.get_scope_local_mut() {
            lm.mark_not_mapped(spc, first, sz, param);
        }
    }

    /// C++ `AliasChecker::gatherAdditiveBase` (`varmap.cc:736`): collect the roots
    /// of every additive expression tree rooted at `startvn` (a spacebase input).
    ///
    /// Walks the descend graph; a Varnode used in a non-additive op is a base.
    /// The `setMark`/`clearMark` queue de-dup is reproduced with a visited set
    /// (the marks are a transient analysis bit; a local set is equivalent and
    /// re-entrancy-safe).
    pub fn gather_additive_base(
        &self,
        startvn: VarnodeId,
        addbase: &mut Vec<crate::varmap::AddBase>,
        exempt_scramble: bool,
    ) {
        use std::collections::BTreeSet;
        // (base, index) queue.
        let mut vnqueue: Vec<(VarnodeId, Option<VarnodeId>)> = vec![(startvn, None)];
        let mut marked: BTreeSet<VarnodeId> = BTreeSet::new();
        marked.insert(startvn);

        let mut i = 0usize;
        while i < vnqueue.len() {
            let (vn, mut indexvn) = vnqueue[i];
            i += 1;
            let mut nonadduse = false;
            let descend: Vec<OpId> = match self.vbank().get(vn) {
                Some(v) => v.descend_iter().collect(),
                None => continue,
            };
            for op in descend {
                let o = match self.obank().get(op) {
                    Some(o) => o,
                    None => continue,
                };
                let code = o.code();
                match code {
                    OpCode::CPUI_COPY => {
                        nonadduse = true; // COPY is both a non-add use and part of an ADD chain.
                        if let Some(subvn) = o.get_out() {
                            if marked.insert(subvn) {
                                vnqueue.push((subvn, indexvn));
                            }
                        }
                    }
                    OpCode::CPUI_INT_SUB => {
                        if o.get_in(1) == Some(vn) {
                            nonadduse = true; // Subtracting the pointer.
                        } else {
                            let othervn = o.get_in(1);
                            if let Some(ov) = othervn {
                                if !self.vbank().get(ov).map(|x| x.is_constant()).unwrap_or(false) {
                                    indexvn = Some(ov);
                                }
                            }
                            if let Some(subvn) = o.get_out() {
                                if marked.insert(subvn) {
                                    vnqueue.push((subvn, indexvn));
                                }
                            }
                        }
                    }
                    OpCode::CPUI_INT_ADD | OpCode::CPUI_PTRADD => {
                        // othervn = in1 (or in0 if in1==vn); non-constant => index.
                        let mut othervn = o.get_in(1);
                        if othervn == Some(vn) {
                            othervn = o.get_in(0);
                        }
                        if let Some(ov) = othervn {
                            if !self.vbank().get(ov).map(|x| x.is_constant()).unwrap_or(false) {
                                indexvn = Some(ov);
                            }
                        }
                        if let Some(subvn) = o.get_out() {
                            if marked.insert(subvn) {
                                vnqueue.push((subvn, indexvn));
                            }
                        }
                    }
                    OpCode::CPUI_PTRSUB | OpCode::CPUI_SEGMENTOP => {
                        if let Some(subvn) = o.get_out() {
                            if marked.insert(subvn) {
                                vnqueue.push((subvn, indexvn));
                            }
                        }
                    }
                    _ => {
                        // (kuna) `cookiescramble`: a stack-pointer scramble
                        // against a live value (MSVC's `/GS` cookie) is not an
                        // address, so it does not open an escape site here.
                        nonadduse |= crate::p6_variables::kuna_cookiescramble::is_escape_site(
                            exempt_scramble,
                            code,
                        );
                    }
                }
            }
            if nonadduse {
                addbase.push(crate::varmap::AddBase::new(vn, indexvn));
            }
        }
    }

    /// C++ `AliasChecker::gatherOffset` (`varmap.cc:818`): the constant portion of
    /// the additive sum the given Varnode is the result of.
    pub fn gather_offset(&self, vn: VarnodeId) -> uintb {
        let v = match self.vbank().get(vn) {
            Some(v) => v,
            None => return 0,
        };
        if v.is_constant() {
            return v.get_offset();
        }
        let size = v.get_size();
        let def = match v.get_def() {
            Some(d) => d,
            None => return 0,
        };
        let (code, in0, in1, in2) = match self.obank().get(def) {
            Some(o) => {
                let n = o.num_input();
                let g = |k: int4| if k < n { o.get_in(k) } else { None };
                (o.code(), g(0), g(1), g(2))
            }
            None => return 0,
        };
        let retval: uintb = match code {
            OpCode::CPUI_COPY => self.gather_offset_opt(in0),
            OpCode::CPUI_PTRSUB | OpCode::CPUI_INT_ADD => {
                self.gather_offset_opt(in0).wrapping_add(self.gather_offset_opt(in1))
            }
            OpCode::CPUI_INT_SUB => {
                self.gather_offset_opt(in0).wrapping_sub(self.gather_offset_opt(in1))
            }
            OpCode::CPUI_PTRADD => {
                let othervn_off = in2
                    .and_then(|iv| self.vbank().get(iv).map(|x| x.get_offset()))
                    .unwrap_or(0);
                let base = self.gather_offset_opt(in0);
                let in1_const = in1
                    .and_then(|iv| self.vbank().get(iv).map(|x| x.is_constant()))
                    .unwrap_or(false);
                if in1_const {
                    let in1_off = in1
                        .and_then(|iv| self.vbank().get(iv).map(|x| x.get_offset()))
                        .unwrap_or(0);
                    base.wrapping_add(in1_off.wrapping_mul(othervn_off))
                } else if othervn_off == 1 {
                    base.wrapping_add(self.gather_offset_opt(in1))
                } else {
                    base
                }
            }
            OpCode::CPUI_SEGMENTOP => self.gather_offset_opt(in2),
            _ => 0,
        };
        retval & kuna_base::address::calc_mask(size)
    }

    /// `gatherOffset` on an optional input (a null operand contributes 0).
    fn gather_offset_opt(&self, vn: Option<VarnodeId>) -> uintb {
        match vn {
            Some(v) => self.gather_offset(v),
            None => 0,
        }
    }
}
