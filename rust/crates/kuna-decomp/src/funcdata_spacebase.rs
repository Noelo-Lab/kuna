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
use kuna_base::types::{int4, uint4, uintb};

use kuna_num::opcodes::OpCode;

use crate::funcdata::Funcdata;
use crate::op::pcodeop_flags;
use crate::seams::{OpId, TypeOp, VarnodeId};
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
    pub fn restructure_varnode(&mut self, _aliasyes: bool) {
        use crate::varmap::MapState;

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
                        // `vn`'s) is NOT an active read.  Compute the slot's address:
                        //   addr = out.getAddr(); slot = bigEndian ? 0 : 1;
                        //   if (in[slot] != vn) addr = addr + in[slot].getSize();
                        //   active iff vn.getAddr() != addr
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
        let mut seam = FuncdataAliasSeam { fd: self };
        state.checker_mut().gather(Rc::clone(space), bounds, false, &mut seam);

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
        // LoadGuard/StoreGuard handling (CPUI_LOAD/STORE array steps) is a
        // heritage-driven refinement; the merged tree carries no load guards yet,
        // so the guard loops add nothing (faithful empty-list behavior).
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
            if let Some(ov) = overlap {
                let mut flags = ov.all_flags;
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
                // No symbol found.
                let in_scope = self
                    .get_scope_local()
                    .map(|lm| lm.in_scope(&ex_addr, ex_size))
                    .unwrap_or(false);
                if in_scope {
                    fl = varnode_flags::mapped | varnode_flags::addrtied;
                } else if unmapped_alias_check {
                    // isUnmappedUnaliased -> nolocalalias (conservatively 0 here:
                    // the alias map is built by restructureVarnode; if absent we
                    // do not assert unaliased).
                    fl = 0;
                } else {
                    fl = 0;
                }
            }

            if self.sync_varnodes_with_symbol(&group, fl, ct.as_ref()) {
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
        if (fl & varnode_flags::addrtied) != 0 {
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

/// The live-IR realization of the [`AliasGatherSeam`](crate::varmap::AliasGatherSeam):
/// `findSpacebaseInput` / `gatherAdditiveBase` / `gatherOffset` over the function's
/// def/use graph (C++ `AliasChecker`'s Varnode walks, `varmap.cc:736-858`).
struct FuncdataAliasSeam<'a> {
    fd: &'a Funcdata,
}

impl crate::varmap::AliasGatherSeam for FuncdataAliasSeam<'_> {
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
        self.fd.gather_additive_base(startvn, addbase);
    }

    fn gather_offset(&mut self, vn: VarnodeId) -> uintb {
        self.fd.gather_offset(vn)
    }
}

impl Funcdata {
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
                        nonadduse = true; // Used in a non-additive expression.
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
