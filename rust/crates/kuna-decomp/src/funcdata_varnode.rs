//! Port of `decompiler/cpp/funcdata_varnode.cc` (W3, item
//! `w3-ir-funcdata-varnode`) — the `Funcdata` methods pertaining directly to
//! Varnodes: the `newVarnode*` creation family, the input/free-varnode
//! life-cycle (`setInputVarnode`/`deleteVarnode`/`destroyVarnode`), the
//! storage-range finders (`findCoveredInput`/`findVarnodeInput`/…), and the
//! def-use rewiring (`totalReplace`).
//!
//! These are additional `impl Funcdata` methods; the [`Funcdata`] struct, the
//! IR-arena ownership (ADR 0001), and the bank callbacks live in
//! [`crate::funcdata`].  This module is a **parallel** porter (it runs after the
//! serial `funcdata`/`block` chain, alongside `funcdata_op`) and therefore holds
//! **no seam-editing rights**: it may not touch `seams.rs`/`dtype.rs`, nor any
//! other module, nor the private fields of `Funcdata`.  Everything here goes
//! through the `pub`/`pub(crate)` surface `funcdata.rs` exposes.
//!
//! ## What this wave ports (self-contained on the public IR surface)
//!
//! The **free-varnode factories** create through `vbank.create`/`create_unique`
//! (the plain insertion path, no `xref` read-repointing callback): `newConstant`,
//! `newUnique`, `newVarnode` (all three overloads), `newVarnodeIop`,
//! `newVarnodeSpace`, `newVarnodeCallSpecs`.  The **create-index allocation
//! order** is preserved exactly because each goes straight to the bank's single
//! `create_index++` path, in the same statement order as the C++.
//!
//! `deleteVarnode` (`vbank.destroy`), `destroyVarnode` (the def/descend teardown,
//! sequenced through the public op/varnode accessors), the storage-range finders
//! (`findCoveredInput`/`findCoveringInput`/`hasInputIntersection`/
//! `findVarnodeInput`/`findVarnodeWritten`, each a `vbank` range query), the
//! `descend` iteration helpers, `checkForLanedRegister` (a `// SEAM(W4)` no-op),
//! and `totalReplace` (def-use rewiring, sequenced — see below) are all ported in
//! full.  `setInputVarnode`'s overlap pre-check (the pure `vbank` read half) is
//! ported as [`Funcdata::find_input_overlap`].
//!
//! ## The two-arena split borrow and the `xref` callback (missing public API)
//!
//! A handful of C++ methods route a *fresh* varnode through
//! `VarnodeBank::setInput`/`createDef`, which call `xref`, which invokes the
//! `replace_reads` callback (re-pointing every op that read an equivalent
//! pre-existing free varnode onto the kept one) **while the bank is mid-mutation**
//! — so `&mut vbank` and `&mut obank` are live *simultaneously*.  `funcdata.rs`
//! supplies the callback as the static [`Funcdata::replace_reads_thunk`]
//! (`&mut PcodeOpBank -> impl FnMut`) precisely so a caller with *direct field
//! access* can split-borrow `self.vbank` and `self.obank`.  A sibling module has
//! no such access (Rust field privacy is by module; `Funcdata::vbank`/`obank` are
//! private to `crate::funcdata`), and the only public accessors
//! (`vbank_mut()`/`obank_mut()`) each reborrow all of `self`.
//!
//! Consequently every method that drives the `xref` callback path —
//! `setInputVarnode` (its `vbank.setInput` tail), `newVarnodeOut`/`newUniqueOut`
//! (`vbank.createDef`/`createDefUnique`) — needs a `pub(crate)` split accessor on
//! `Funcdata` (e.g. `fn banks_mut(&mut self) -> (&mut VarnodeBank, &mut
//! PcodeOpBank)`) that only the `funcdata.rs` owner can add.  Exactly as the
//! `funcdata.rs` module doc anticipates ("the bodies that call `xref`
//! (`setInputVarnode`, `opSetOutput`) live in funcdata_op"), those bodies are
//! left to the seam owner; this module ports the **portable remainder**.
//!
//! `totalReplace` does **not** go through `xref`: it re-points op inputs directly
//! (the standalone `opSetInput` semantics), which decompose into *sequential*
//! single-arena borrows (`vbank.erase_descend` ; `obank.set_input` ;
//! `vbank.add_descend`), so it is ported in full here.
//!
//! ## What this wave defers to `funcdata_op` (op-creation API)
//!
//! `totalReplaceConstant`, `adjustInputVarnodes`, `combineInputVarnodes`,
//! `newExtendedConstant`, `descend2Undef`, and `splitUses` build **new ops**
//! (`newOp` + `opSetOpcode` + `opInsert{Begin,Before,After,End}` +
//! `newVarnodeOut`/`newUniqueOut`).  That op-creation/insertion API
//! (`funcdata_op.cc`) is the **funcdata_op** wave's; these methods sit at that
//! boundary and are noted in the structured `losses` rather than half-ported.
//!
//! ## Missing public surfaces on the merged W3 IR (precise notes)
//!
//! Two pre-existing public surfaces would let more of `funcdata_varnode.cc` land
//! without seam edits; both are noted as losses:
//!   - `Varnode::set_flags(uint4)` is private to `varnode.rs`, so `newCodeRef`'s
//!     `setFlags(annotation)` and `cloneVarnode`'s flag-copy cannot be expressed
//!     (the create + `assignHigh` portion is otherwise portable);
//!   - `Funcdata::banks_mut()` (the `(vbank, obank)` split accessor) for the
//!     `xref`-callback methods above.

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::space::{spacetype, AddrSpace};
use kuna_base::types::{int4, uint4, uintb, uintm, Wrap};

use kuna_num::opcodes::OpCode;

use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;
use crate::seams::{OpId, VarnodeId};
use crate::varnode::{varnode_flags, DefOpInfo};

/// Effect classes a [`crate::seams::FuncProto`] reports for a storage range
/// (C++ `EffectRecord::effecttype`, `fspec.hh:392-397`).
///
/// SEAM(W4): the prototype model subsystem (`fspec.{hh,cc}`) is W4.  The W3
/// `funcp` placeholder reports no records, so the `setInputVarnode` tail always
/// sees `UNKNOWN_EFFECT` (the "absence of an EffectRecord" value) and never marks
/// an input `unaffected`/`return_address`.  The constants are transcribed
/// verbatim so the W4 wave can wire `funcp.hasEffect` in without changing call
/// sites.
#[allow(dead_code)]
mod effect_record {
    use kuna_base::types::uint4;
    /// The sub-function does not change the value at all.
    pub const UNAFFECTED: uint4 = 1;
    /// The memory is changed and is completely unrelated to its original value.
    pub const KILLEDBYCALL: uint4 = 2;
    /// The memory is being used to store the return address.
    pub const RETURN_ADDRESS: uint4 = 3;
    /// An unknown effect (indicates the absence of an EffectRecord).
    pub const UNKNOWN_EFFECT: uint4 = 4;
}

impl Funcdata {
    // -----------------------------------------------------------------------
    // Datatype helpers (the `glb->types->getBase` calls — W6 TypeFactory seam)
    // -----------------------------------------------------------------------

    /// Stand-in for `glb->types->getBase(s, TYPE_UNKNOWN)` (C++ `TypeFactory`).
    ///
    /// SEAM(W6): the `TypeFactory` (`glb->types`, `type.{hh,cc}`) is W6; the W3
    /// data-model has no factory, so the varnode factories construct the
    /// unknown-base [`Datatype`] skeleton directly (size `s`, metatype
    /// `TYPE_UNKNOWN`), exactly as the merged `funcdata.rs`/`varnode.rs` tests do.
    /// W6 replaces this with the interned-factory lookup; call sites stay put.
    fn type_base_unknown(s: int4) -> Rc<Datatype> {
        Rc::new(Datatype::new(s, type_metatype::TYPE_UNKNOWN))
    }

    /// Materialize the function's recovered/locked input parameters as Symbols in
    /// the local scope, the symbol-creation `ProtoStoreSymbol::setInput` performs
    /// (`fspec.cc:3174`) that the kuna `ProtoStoreInternal` skips.
    ///
    /// In C++ the `FuncProto`'s scope IS the `ScopeLocal` (`funcdata.cc:69`
    /// `funcp.setScope(localmap,...)`), so each `setInput` adds a parameter Symbol
    /// at the parameter's storage address with category `function_parameter`.
    /// `ActionNameVars::linkSymbols` then binds the body Varnodes to those Symbols
    /// (`Funcdata::linkSymbol` -> `Scope::queryProperties`).  The kuna port stores
    /// parameters in a symbol-less `ProtoStoreInternal`, so those Symbols never
    /// exist and the body renders the raw registers; this method creates them from
    /// the proto store (`get_param(i)`'s name/type/address), so the existing
    /// `name_for_varnode`/`linkSymbol` resolution binds `ptr`/`a`/`b` uniformly
    /// with mapped and promoted locals.  Idempotent (see
    /// [`crate::varmap::ScopeLocal::add_param_symbol`]).
    pub fn link_proto_params(&mut self) {
        if self.get_scope_local().is_none() {
            return;
        }
        let num = self.get_func_proto().num_params();
        // Gather (slot, name, type, addr) first; `get_param` borrows the proto and
        // `add_param_symbol` borrows the scope (both on &mut self).
        let mut specs: Vec<(int4, String, Rc<Datatype>, Address)> = Vec::new();
        for i in 0..num {
            let param = match self.get_func_proto().get_param(i) {
                Some(p) => p,
                None => continue,
            };
            let ty = match param.get_type() {
                Some(t) => Rc::clone(t),
                None => continue,
            };
            let addr = param.get_address();
            if addr.is_invalid() {
                continue;
            }
            // C++ `ProtoStoreSymbol::setInput` passes the parameter's name; an
            // unnamed (recovered, unlocked) parameter is an `addSymbol("",...)` with
            // an undefined name that `ActionNameVars`/`buildDefaultName` then routes
            // to the `aN` (`function_parameter`) default.  The kuna proto store and
            // the scope symbol are separate objects, so the default is materialized
            // here (`kuna_arg_name(i)` == `buildDefaultName`'s `function_parameter`
            // arm) instead of being deferred to `assignDefaultNames`, avoiding the
            // `$$undef` placeholder that `addSymbol("",...)` would otherwise leak
            // into the body.  A locked, named proto (`parse line extern`) carries
            // the explicit `ptr`/`a`/`b`.
            let name = if param.get_name().is_empty() {
                crate::database::kuna_arg_name(i)
            } else {
                param.get_name().to_string()
            };
            specs.push((i, name, ty, addr));
        }
        for (i, name, ty, addr) in specs {
            if let Some(lm) = self.get_scope_local_mut() {
                let _ = lm.add_param_symbol(i, &name, ty, &addr);
            }
        }
    }

    /// If HighVariables are enabled, make sure the given Varnode has one assigned
    /// (C++ `Funcdata::assignHigh`, `funcdata_varnode.cc:48`).
    ///
    /// Before `setHighLevel` (`is_high_on()` false) this is a no-op, exactly as
    /// C++ (the W3 IR construction path).  Once HighVariables are on — e.g. a
    /// Varnode created by the Merge trim COPY after `ActionAssignHigh` — the new
    /// Varnode gets its singleton HighVariable here via [`assign_high_var`],
    /// matching the C++ `new HighVariable(vn)` in `assignHigh` so the merge's
    /// `getHigh()` reads never hit a null high.
    fn assign_high(&mut self, vn: VarnodeId) {
        if self.is_high_on() {
            // vn->calcCover(); ... new HighVariable(vn); vn->setHigh(...)
            self.assign_high_var(vn);
        }
    }

    /// Check if the given storage range is a potential laned register; if so,
    /// record the storage with the matching laned-register record (C++
    /// `Funcdata::checkForLanedRegister`, `funcdata_varnode.cc:300`).
    ///
    /// Faithful transcription: `glb->getLanedRegister(addr,sz)` looks up the
    /// architecture's lane table by size; on a hit, the `(space,offset,size)`
    /// storage is recorded in the laned-access map keyed by the C++ `VarnodeData`
    /// ordering.  The architecture's records are immutable, so the cloned
    /// [`LanedRegister`](crate::transform::LanedRegister) stands in for the C++
    /// `const LanedRegister *`.  The `s >= minLanedSize` guard at the call sites
    /// gates this exactly as in the C++.
    fn check_for_laned_register(&mut self, sz: int4, addr: &Address) {
        // const LanedRegister *lanedRegister = glb->getLanedRegister(addr,sz);
        // if (lanedRegister == 0) return;
        let laned_register = match self.get_arch().get_laned_register(addr, sz) {
            Some(lr) => lr.clone(),
            None => return,
        };
        // VarnodeData storage{space=addr.getSpace(), offset=addr.getOffset(), size=sz};
        // lanedMap[storage] = lanedRegister;
        let key = crate::funcdata::LanedKey::new(addr, sz);
        self.laned_map_insert(key, addr.clone(), sz, laned_register);
    }

    // -----------------------------------------------------------------------
    // Free-varnode creation factories (C++ new* — the plain `vbank.create`
    // path; no `xref` read-repointing, so fully portable here)
    // -----------------------------------------------------------------------

    /// Calculate the non-zero mask (`nzm`) property on every live Varnode (C++
    /// `Funcdata::calcNZMask`, `funcdata_varnode.cc:874`).
    ///
    /// A DFS over the def-use graph seeds each leaf Varnode's `nzm`
    /// (constant -> its value; type-locked bool -> 1; otherwise the full
    /// size-mask, with the low byte cleared for a spacebase) and computes each op
    /// output's `nzm` from [`get_nz_mask_local`](crate::op::get_nz_mask_local) in
    /// post-order.  A second fixed-point pass re-propagates through `MULTIEQUAL`
    /// loop edges (which the DFS clipped) until the masks settle.
    pub fn calc_nz_mask(&mut self) {
        use kuna_base::address::calc_mask;
        // PcodeOpNode stack: (op, next-slot).
        let alive: Vec<OpId> = self.obank().iter_alive().collect();
        let mut opstack: Vec<(OpId, int4)> = Vec::new();
        for root in &alive {
            let root = *root;
            if self.obank().get(root).map(|o| o.is_mark()).unwrap_or(true) {
                continue;
            }
            opstack.push((root, 0));
            self.obank_mut().get_mut(root).expect("calc_nz_mask: stale root").set_mark();

            while let Some(&(op, slot)) = opstack.last() {
                let num_input = self.obank().get(op).map(|o| o.num_input()).unwrap_or(0);
                if slot >= num_input {
                    // No edge left: compute the output mask in post-order.
                    if let Some(outvn) = self.obank().get(op).and_then(|o| o.get_out()) {
                        let parent = self.obank().get(op).and_then(|o| o.get_parent());
                        let nz = {
                            let bblocks = self.bblocks_ref();
                            let is_loop_in = |s: int4| -> bool {
                                parent
                                    .map(|bid| bblocks.arena[bid].is_loop_in(s))
                                    .unwrap_or(false)
                            };
                            let o = self.obank().get(op).expect("calc_nz_mask: stale op");
                            crate::op::get_nz_mask_local(o, self.vbank(), true, &is_loop_in)
                        };
                        self.vbank_mut().get_mut(outvn).expect("calc_nz_mask: stale out").set_nz_mask(nz);
                    }
                    opstack.pop();
                    continue;
                }
                // Advance to next input.
                if let Some(top) = opstack.last_mut() {
                    top.1 += 1;
                }
                let oldslot = slot;
                // MULTIEQUAL: clip looping edges.
                if self.obank().get(op).map(|o| o.code()) == Some(OpCode::CPUI_MULTIEQUAL) {
                    let parent = self.obank().get(op).and_then(|o| o.get_parent());
                    let is_loop = parent
                        .map(|bid| self.bblocks_ref().arena[bid].is_loop_in(oldslot))
                        .unwrap_or(false);
                    if is_loop {
                        continue;
                    }
                }
                let vn = match self.obank().get(op).and_then(|o| o.get_in(oldslot)) {
                    Some(v) => v,
                    None => continue,
                };
                let (is_written, is_const, off, is_type_lock, is_sb, sz, meta_bool, def) = {
                    let v = self.vbank().get(vn).expect("calc_nz_mask: stale in");
                    (
                        v.is_written(),
                        v.is_constant(),
                        v.get_offset(),
                        v.is_type_lock(),
                        v.is_spacebase(),
                        v.get_size(),
                        v.get_type().get_metatype() == type_metatype::TYPE_BOOL,
                        v.get_def(),
                    )
                };
                if !is_written {
                    let nz = if is_const {
                        off
                    } else if is_type_lock && meta_bool {
                        1
                    } else {
                        let mut m = calc_mask(sz);
                        if is_sb {
                            m &= !0xffu64; // Treat spacebase input as aligned.
                        }
                        m
                    };
                    self.vbank_mut().get_mut(vn).expect("calc_nz_mask: stale in").set_nz_mask(nz);
                } else if let Some(def) = def {
                    if !self.obank().get(def).map(|o| o.is_mark()).unwrap_or(true) {
                        opstack.push((def, 0));
                        self.obank_mut().get_mut(def).expect("calc_nz_mask: stale def").set_mark();
                    }
                }
            }
        }

        // Clear marks; collect MULTIEQUALs for the loop-edge fixpoint.
        let mut worklist: Vec<OpId> = Vec::new();
        for &op in &alive {
            self.obank_mut().get_mut(op).expect("calc_nz_mask: stale alive").clear_mark();
            if self.obank().get(op).map(|o| o.code()) == Some(OpCode::CPUI_MULTIEQUAL) {
                worklist.push(op);
            }
        }
        // Propagate changes along all edges until settled.
        while let Some(op) = worklist.pop() {
            let outvn = match self.obank().get(op).and_then(|o| o.get_out()) {
                Some(v) => v,
                None => continue,
            };
            let nzmask = {
                let o = self.obank().get(op).expect("calc_nz_mask: stale op");
                let no_loop = |_s: int4| -> bool { false };
                crate::op::get_nz_mask_local(o, self.vbank(), false, &no_loop)
            };
            let cur = self.vbank().get(outvn).map(|v| v.get_nz_mask()).unwrap_or(0);
            if nzmask != cur {
                self.vbank_mut().get_mut(outvn).expect("calc_nz_mask: stale out").set_nz_mask(nzmask);
                let descend: Vec<OpId> =
                    self.vbank().get(outvn).map(|v| v.descend_iter().collect()).unwrap_or_default();
                worklist.extend(descend);
            }
        }
    }

    /// Allocate a Varnode representing the indicated constant value
    /// (C++ `Funcdata::newConstant`, `funcdata_varnode.cc:68`).
    ///
    /// Its storage address is in the constant space (`glb->getConstant`).  There
    /// is no chance of matching `localmap`, so no property look-up is performed.
    pub fn new_constant(&mut self, s: int4, constant_val: uintb) -> VarnodeId {
        let ct = Self::type_base_unknown(s);
        // Varnode *vn = vbank.create(s,glb->getConstant(constant_val),ct);
        let caddr = self.get_arch().get_constant(constant_val);
        let vn = self.vbank_mut().create(s, caddr, ct);
        self.assign_high(vn);
        vn
    }

    /// Allocate a new \e temporary register Varnode in the unique space
    /// (C++ `Funcdata::newUnique`, `funcdata_varnode.cc:85`).
    ///
    /// `ct == None` means "use the unknown base of size `s`".  No chance of
    /// matching `localmap`.
    pub fn new_unique(&mut self, s: int4, ct: Option<Rc<Datatype>>) -> VarnodeId {
        let ct = ct.unwrap_or_else(|| Self::type_base_unknown(s));
        let vn = self.vbank_mut().create_unique(s, ct);
        self.assign_high(vn);
        // if (s >= minLanedSize) checkForLanedRegister(s, vn->getAddr());
        if s >= self.get_min_laned_size() {
            let addr = self.vbank().get(vn).expect("new_unique: stale vn").get_addr().clone();
            self.check_for_laned_register(s, &addr);
        }
        vn
    }

    /// Create a new unattached (free) Varnode object
    /// (C++ `Funcdata::newVarnode(int4,const Address&,Datatype*)`,
    /// `funcdata_varnode.cc:150`).
    ///
    /// `ct == None` means "use the unknown base of size `s`".
    ///
    /// SEAM(W4): the C++ then runs `localmap->queryProperties` to seed boolean
    /// flags/type from a symbol entry; the W3 `localmap` placeholder reports no
    /// entry, so [`Funcdata::set_varnode_properties`] (the merged `funcdata.rs`
    /// W4 no-op) is called instead, preserving the call cadence.
    pub fn new_varnode(&mut self, s: int4, m: &Address, ct: Option<Rc<Datatype>>) -> VarnodeId {
        let ct = ct.unwrap_or_else(|| Self::type_base_unknown(s));
        let vn = self.vbank_mut().create(s, m.clone(), ct);
        self.assign_high(vn);
        if s >= self.get_min_laned_size() {
            self.check_for_laned_register(s, m);
        }
        // uint4 vflags=0; entry = localmap->queryProperties(...); ...  -- SEAM(W4)
        self.set_varnode_properties(vn);
        vn
    }

    /// Create a new free Varnode from a base space and offset
    /// (C++ `Funcdata::newVarnode(int4,AddrSpace*,uintb)`,
    /// `funcdata_varnode.cc:241`).
    ///
    /// A thin wrapper over [`Funcdata::new_varnode`] that builds the Address.
    pub fn new_varnode_space_off(&mut self, s: int4, base: Rc<AddrSpace>, off: uintb) -> VarnodeId {
        let m = Address::new(base, off);
        self.new_varnode(s, &m, None)
    }

    /// Create an \e annotation Varnode holding a reference to a PcodeOp
    /// (C++ `Funcdata::newVarnodeIop`, `funcdata_varnode.cc:178`).
    ///
    /// Used so a `CPUI_INDIRECT` can refer to the op it holds an indirect effect
    /// for.  The C++ encodes the raw `PcodeOp *` pointer as the offset of an
    /// address in the \e iop space; the Rust IR has no stable pointer, so the
    /// [`OpId`]'s slotmap key bit-pattern is encoded (`op_iop_encode`) as a
    /// stable, round-trippable identity (the iop-space value is opaque to the IR
    /// and is only ever decoded back to the same op).
    pub fn new_varnode_iop(&mut self, op: OpId) -> VarnodeId {
        // Datatype *ct = glb->types->getBase(sizeof(op),TYPE_UNKNOWN);
        // sizeof(PcodeOp*) is the pointer width; the W3 model uses 8 (uintb).
        let ct = Self::type_base_unknown(8);
        let cspc = Rc::clone(
            self.get_arch().manage().get_iop_space().expect("newVarnodeIop: no iop space"),
        );
        let encoded = op_iop_encode(op);
        let vn = self.vbank_mut().create(8, Address::new(cspc, encoded), ct);
        self.assign_high(vn);
        vn
    }

    /// Create a constant Varnode encoding a reference to an address space
    /// (C++ `Funcdata::newVarnodeSpace`, `funcdata_varnode.cc:192`).
    ///
    /// Used for the space operand of LOAD/STORE ops.
    pub fn new_varnode_space(&mut self, spc: &Rc<AddrSpace>) -> VarnodeId {
        let ct = Self::type_base_unknown(8); // sizeof(AddrSpace*) → pointer width
        // Varnode *vn = vbank.create(sizeof(spc),glb->createConstFromSpace(spc),ct);
        let caddr = self.get_arch().manage().create_const_from_space(spc);
        let vn = self.vbank_mut().create(8, caddr, ct);
        self.assign_high(vn);
        vn
    }

    /// Create a call-specification \e annotation Varnode
    /// (C++ `Funcdata::newVarnodeCallSpecs`, `funcdata_varnode.cc:207`).
    ///
    /// Used as an input to `CPUI_CALL` ops to speed up access to the call
    /// specification.  The C++ encodes the raw `FuncCallSpecs *` as the offset of
    /// an address in the \e fspec space; the W3 model has no `FuncCallSpecs`
    /// (W4), so the encoded handle is supplied by the caller as a `uintb`.
    ///
    /// SEAM(W4): when the `FuncCallSpecs` type lands, the caller passes the call
    /// spec and this encodes it; the body (`vbank.create` in the fspec space) is
    /// unchanged.
    pub fn new_varnode_call_specs(&mut self, fc_encoded: uintb) -> VarnodeId {
        let ct = Self::type_base_unknown(8);
        let cspc = Rc::clone(
            self.get_arch().manage().get_fspec_space().expect("newVarnodeCallSpecs: no fspec space"),
        );
        let vn = self.vbank_mut().create(8, Address::new(cspc, fc_encoded), ct);
        self.assign_high(vn);
        vn
    }

    /// Build the `DefOpInfo` carrier (op id + its `SeqNum`) the `vbank.setDef`/
    /// `createDef` paths take.  Mirrors the funcdata_op helper of the same name
    /// (module-private there); replicated here so the codeRef-out factories below
    /// (`newVarnodeOut`/`newUniqueOut`) can build their def carrier without a seam
    /// edit (both are inherent `Funcdata` methods; the carrier is trivial).
    fn def_op_info_v(&self, op: OpId) -> DefOpInfo {
        let seqnum =
            self.obank().get(op).expect("def_op_info_v: stale op").get_seq_num().clone();
        DefOpInfo { id: op, seqnum }
    }

    /// Create a Varnode already defined as the output of a given PcodeOp
    /// (C++ `Funcdata::newVarnodeOut`, `funcdata_varnode.cc:106`).
    ///
    /// More efficient than `newVarnode`+`opSetOutput`: it avoids inserting the free
    /// form into the tree, and only queries `localmap` once.  The central
    /// `vbank.createDef(s,m,ct,op)` drives the `xref` read-repointing callback, so
    /// it split-borrows both banks ([`Funcdata::banks_mut`]) and runs
    /// [`replace_reads_thunk`](Funcdata::replace_reads_thunk) over `obank`.
    ///
    /// SEAM(W4): the `localmap->queryProperties` symbol look-up + `setSymbolProperties`/
    /// `setFlags(vflags & ~typelock)` tail is the W4 symbol scope; the W3 placeholder
    /// reports no entry, so it is the [`Funcdata::set_varnode_properties`] no-op,
    /// preserving the call cadence (and never touching the (space,offset,size) the
    /// flow gate asserts).
    pub fn new_varnode_out(&mut self, s: int4, m: &Address, op: OpId) -> KunaResult<VarnodeId> {
        // Datatype *ct = glb->types->getBase(s,TYPE_UNKNOWN);
        let ct = Self::type_base_unknown(s);
        // Varnode *vn = vbank.createDef(s,m,ct,op);  -- split-borrow scoped so the
        //   thunk (which holds &mut obank) drops before later &mut self calls.
        let def = self.def_op_info_v(op);
        let vn = {
            let (vbank, obank) = self.banks_mut();
            let mut replace = Funcdata::replace_reads_thunk(obank);
            vbank.create_def(s, m.clone(), ct, def, &mut replace)?
        };
        // op->setOutput(vn);
        self.obank_mut().get_mut(op).expect("new_varnode_out: stale op").set_output(Some(vn));
        // assignHigh(vn);
        self.assign_high(vn);
        // if (s >= minLanedSize) checkForLanedRegister(s,m);
        if s >= self.get_min_laned_size() {
            self.check_for_laned_register(s, m);
        }
        // uint4 vflags=0; entry = localmap->queryProperties(...); ...  -- SEAM(W4)
        self.set_varnode_properties(vn);
        Ok(vn)
    }

    /// Allocate a unique-space Varnode created as the output of a given PcodeOp
    /// (C++ `Funcdata::newUniqueOut`, `funcdata_varnode.cc:131`).
    ///
    /// The `vbank.createDefUnique(s,ct,op)` drives `xref`; same split-borrow as
    /// [`new_varnode_out`](Funcdata::new_varnode_out).  No `localmap` match (the
    /// unique space never carries symbols), matching the C++.
    pub fn new_unique_out(&mut self, s: int4, op: OpId) -> KunaResult<VarnodeId> {
        let ct = Self::type_base_unknown(s);
        let def = self.def_op_info_v(op);
        let vn = {
            let (vbank, obank) = self.banks_mut();
            let mut replace = Funcdata::replace_reads_thunk(obank);
            vbank.create_def_unique(s, ct, def, &mut replace)?
        };
        // op->setOutput(vn);
        self.obank_mut().get_mut(op).expect("new_unique_out: stale op").set_output(Some(vn));
        self.assign_high(vn);
        // if (s >= minLanedSize) checkForLanedRegister(s, vn->getAddr());
        if s >= self.get_min_laned_size() {
            let addr =
                self.vbank().get(vn).expect("new_unique_out: stale vn").get_addr().clone();
            self.check_for_laned_register(s, &addr);
        }
        Ok(vn)
    }

    /// Create an \e annotation Varnode holding a reference to a specific Address
    /// (C++ `Funcdata::newCodeRef`, `funcdata_varnode.cc:224`).
    ///
    /// Used by the branch p-code ops to hold a destination address: a size-1
    /// Varnode at `m` that holds no value in the data-flow.  The C++ then sets
    /// `Varnode::annotation` on it.
    ///
    /// SEAM(W6): `glb->types->getTypeCode()` (the W6 `TypeFactory`'s code type) is
    /// replaced with the unknown base (size 1), as the rest of this wave does.
    ///
    /// `vn->setFlags(Varnode::annotation)` is now expressed: LOSS-077 added a
    /// `pub(crate)` [`Varnode::set_annotation`] sliver routed through the module's
    /// private `set_flags`, so the `annotation` property bit (the previously
    /// carried LOSS-036/LOSS-037 loss) is set faithfully.
    pub fn new_code_ref(&mut self, m: &Address) -> VarnodeId {
        // ct = glb->types->getTypeCode();  -- SEAM(W6): unknown base of size 1.
        let ct = Self::type_base_unknown(1);
        // vn = vbank.create(1,m,ct);
        let vn = self.vbank_mut().create(1, m.clone(), ct);
        // vn->setFlags(Varnode::annotation);
        self.vbank_mut().get_mut(vn).expect("new_code_ref: stale vn").set_annotation();
        self.assign_high(vn);
        vn
    }

    /// Copy properties from an existing Varnode to a new (overlapping) Varnode
    /// (C++ `Funcdata::transferVarnodeProperties`, `funcdata_varnode.cc:632`).
    ///
    /// The new Varnode is assumed to overlap the storage of the existing Varnode.
    /// Boolean flags (`directwrite`/`addrforce`) and the \e consume bit-mask
    /// (shifted by `lsbOffset` bytes of significance) are copied.  Used by
    /// [`TransformVar::create_replacement`](crate::transform) when it materializes a
    /// `piece` placeholder.
    pub fn transfer_varnode_properties(
        &mut self,
        vn: VarnodeId,
        new_vn: VarnodeId,
        lsb_offset: int4,
    ) {
        use kuna_base::address::calc_mask;
        // uintb newConsume = ~((uintb)0); -- bits shifted in above precision stay set
        let mut new_consume: uintb = !0u64;
        // if (lsbOffset < sizeof(uintb)) { ... }  (sizeof(uintb) == 8)
        if (lsb_offset as usize) < std::mem::size_of::<uintb>() {
            let vn_consume = self.vbank().get(vn).expect("transfer_varnode_properties: stale vn").get_consume();
            let new_size = self.vbank().get(new_vn).expect("transfer_varnode_properties: stale newVn").get_size();
            let mut fill_bits: uintb = 0;
            if lsb_offset != 0 {
                // fillBits = newConsume << 8*(sizeof(uintb) - lsbOffset);
                fill_bits = new_consume.wshl(8u32 * (std::mem::size_of::<uintb>() as u32 - lsb_offset as u32));
            }
            // newConsume = ((vn->getConsume() >> 8*lsbOffset) | fillBits) & calc_mask(newVn->getSize());
            new_consume = (vn_consume.wshr(8u32 * lsb_offset as u32) | fill_bits) & calc_mask(new_size);
        }

        // uint4 vnFlags = vn->getFlags() & (Varnode::directwrite|Varnode::addrforce);
        let vn_flags = self.vbank().get(vn).expect("transfer_varnode_properties: stale vn").get_flags()
            & (varnode_flags::directwrite | varnode_flags::addrforce);

        // newVn->setFlags(vnFlags);  // Preserve addrforce setting
        self.vbank_mut()
            .get_mut(new_vn)
            .expect("transfer_varnode_properties: stale newVn")
            .set_flags_pub(vn_flags);
        // newVn->setConsume(newConsume);
        self.vbank_mut()
            .get_mut(new_vn)
            .expect("transfer_varnode_properties: stale newVn")
            .set_consume(new_consume);
    }

    // -----------------------------------------------------------------------
    // Input-varnode formalization (the xref-driving tail of setInputVarnode)
    // -----------------------------------------------------------------------

    /// Formally define a Varnode as a function input (C++
    /// `Funcdata::setInputVarnode`, `funcdata_varnode.cc:342`).
    ///
    /// The overlap pre-check ([`find_input_overlap`](Funcdata::find_input_overlap))
    /// is the pure `vbank` read; this drives the mutating tail:
    /// `vbank.setInput(vn)` (which runs the `xref` read-repointing callback, hence
    /// the [`banks_mut`](Funcdata::banks_mut) split-borrow), `setVarnodeProperties`,
    /// and the `funcp.hasEffect` unaffected/return-address marking.
    ///
    /// SEAM(W4): `funcp.hasEffect` (the prototype effect model) reports
    /// `UNKNOWN_EFFECT` from the W3 `FuncProto` placeholder, so the
    /// unaffected/return-address marks are never applied — faithful for the W3 IR
    /// (no prototype yet), wired transparently when W4 lands.
    pub fn set_input_varnode(&mut self, vn: VarnodeId) -> KunaResult<VarnodeId> {
        // if (vn->isInput()) return vn;  // Already an input
        if self.vbank().get(vn).expect("set_input_varnode: stale vn").is_input() {
            return Ok(vn);
        }
        // First check if it overlaps any other varnode (the pure vbank read).
        if let Some(invn) = self.find_input_overlap(vn)? {
            // Identical pre-existing input: discard the candidate, return invn.
            return Ok(invn);
        }
        // vn = vbank.setInput(vn);  -- split-borrow both banks for the xref callback.
        let vn = {
            let (vbank, obank) = self.banks_mut();
            let mut replace = Funcdata::replace_reads_thunk(obank);
            vbank.set_input(vn, &mut replace)?
        };
        // setVarnodeProperties(vn);
        self.set_varnode_properties(vn);
        // uint4 effecttype = funcp.hasEffect(vn->getAddr(),vn->getSize());
        //   -- SEAM(W4): the W3 FuncProto reports no effect record (UNKNOWN_EFFECT),
        //      so neither `setUnaffected` nor `setReturnAddress` fires.  The C++
        //      branch ladder is transcribed for the W4 wave:
        //   if (effecttype == unaffected) vn->setUnaffected();
        //   if (effecttype == return_address) { vn->setUnaffected(); vn->setReturnAddress(); }
        Ok(vn)
    }

    // -----------------------------------------------------------------------
    // Varnode deletion / teardown
    // -----------------------------------------------------------------------

    /// Delete the given Varnode (C++ `Funcdata::deleteVarnode`, inline in
    /// `funcdata.hh:302` — `vbank.destroy(vn)`).
    ///
    /// The bank refuses to destroy an integrated varnode (one with a def or
    /// descendants); that condition is the C++ `LowlevelError`.
    pub fn delete_varnode(&mut self, vn: VarnodeId) -> KunaResult<()> {
        self.vbank_mut().destroy(vn)
    }

    /// Create a new Varnode which is a \e clone of the given Varnode
    /// (C++ `Funcdata::cloneVarnode`, `funcdata_varnode.cc:254`).
    ///
    /// The new Varnode reuses the original's size/address/type, and copies only
    /// the small set of \e clonable property flags (annotation, externref,
    /// readonly, persist, addrtied, addrforce, indirect_creation, incidental_copy,
    /// volatil, mapped).  Used by [`Funcdata::clone_op`] when building the partial
    /// jump-table-recovery clone.
    pub fn clone_varnode(&mut self, vn: VarnodeId) -> VarnodeId {
        let (size, addr, ct, flags) = {
            let v = self.vbank().get(vn).expect("clone_varnode: stale vn");
            (v.get_size(), v.get_addr().clone(), Rc::clone(v.get_type()), v.get_flags())
        };
        self.clone_varnode_fields(size, addr, ct, flags)
    }

    /// Clone a Varnode from a \e different function's bank into \b this one (the
    /// cross-function variant of [`clone_varnode`] used by `truncatedFlow`).
    pub fn clone_varnode_from(&mut self, src: &Funcdata, vn: VarnodeId) -> VarnodeId {
        let (size, addr, ct, flags) = {
            let v = src.vbank().get(vn).expect("clone_varnode_from: stale src vn");
            (v.get_size(), v.get_addr().clone(), Rc::clone(v.get_type()), v.get_flags())
        };
        self.clone_varnode_fields(size, addr, ct, flags)
    }

    /// Shared body of [`clone_varnode`]/[`clone_varnode_from`]: create the new
    /// Varnode and copy the small clonable flag set (C++ `cloneVarnode`).
    fn clone_varnode_fields(
        &mut self,
        size: int4,
        addr: Address,
        ct: Rc<Datatype>,
        flags: uint4,
    ) -> VarnodeId {
        use varnode_flags as vf;
        let newvn = self.vbank_mut().create(size, addr, ct);
        // These are the flags we allow to be cloned (funcdata_varnode.cc:259-265)
        let vflags = flags
            & (vf::annotation
                | vf::externref
                | vf::readonly
                | vf::persist
                | vf::addrtied
                | vf::addrforce
                | vf::indirect_creation
                | vf::incidental_copy
                | vf::volatil
                | vf::mapped);
        self.vbank_mut()
            .get_mut(newvn)
            .expect("clone_varnode: fresh vn")
            .set_flags_pub(vflags);
        newvn
    }

    /// Remove the given Varnode, replacing references with null and freeing it
    /// (C++ `Funcdata::destroyVarnode`, `funcdata_varnode.cc:274`).
    ///
    /// Faithful transcription: for every op reading `vn`, `op->clearInput(slot)`;
    /// if `vn` is written, `vn->def->setOutput(0)` and `vn->def = 0`; then
    /// `vn->destroyDescend()` and `vbank.destroy(vn)`.  Each step is a single,
    /// sequential single-arena borrow, so no `(vbank,obank)` split is needed.
    pub fn destroy_varnode(&mut self, vn: VarnodeId) -> KunaResult<()> {
        // for(iter=vn->beginDescend(); iter!=endDescend(); ++iter)
        //   op->clearInput(op->getSlot(vn));
        let readers: Vec<OpId> = self.descend_snapshot(vn);
        for op in readers {
            let slot = self.obank().get(op).map(|o| o.get_slot(vn)).unwrap_or(-1);
            if slot >= 0 {
                if let Some(o) = self.obank_mut().get_mut(op) {
                    o.clear_input(slot);
                }
            }
        }
        // if (vn->def != 0) { vn->def->setOutput(0); vn->def = 0; }
        let def = self.vbank().get(vn).and_then(|v| v.get_def());
        if let Some(defop) = def {
            if let Some(o) = self.obank_mut().get_mut(defop) {
                o.set_output(None);
            }
            // vn->def = 0: the C++ nulls the raw `def` field (no tree reinsert,
            // since destroy() follows immediately).  `make_free` clears the
            // written class and re-keys the varnode as free so `destroy`'s
            // "integrated varnode" guard (def != 0) passes.
            self.vbank_mut().make_free(vn);
        }
        // vn->destroyDescend();  vbank.destroy(vn);
        self.vbank_mut().destroy_descend(vn);
        self.vbank_mut().destroy(vn)
    }

    /// Destroy the given Varnode and recursively any op (and its inputs) that
    /// produced it, provided nothing still reads it (C++
    /// `Funcdata::destroyVarnodeRecursive`, `funcdata_varnode.cc:545`).
    ///
    /// Faithful transcription: bail if the Varnode is auto-live or still has
    /// descendants; if it is not written, just free it; otherwise recurse into
    /// its defining op via `op_destroy_recursive`.
    pub fn destroy_varnode_recursive(&mut self, vn: VarnodeId) -> KunaResult<()> {
        let (auto_live, no_descend, written, def) = match self.vbank().get(vn) {
            Some(v) => (
                v.is_auto_live(),
                v.has_no_descend(),
                v.is_written(),
                v.get_def(),
            ),
            None => return Ok(()),
        };
        if auto_live || !no_descend {
            return Ok(());
        }
        if !written {
            return self.vbank_mut().destroy(vn);
        }
        if let Some(defop) = def {
            let mut scratch: Vec<OpId> = Vec::new();
            self.op_destroy_recursive(defop, &mut scratch);
        }
        Ok(())
    }

    /// Free / destroy Varnodes that no longer have any descendant reads (C++
    /// `Funcdata::clearDeadVarnodes`, `funcdata_varnode.cc:850`).
    ///
    /// Walks the location set; a no-descend input that is not locked is made
    /// free (and its cover cleared), and any now-free no-descend Varnode is
    /// removed from the bank.  Called at the tail of `ActionDeadCode::apply`
    /// after the op graph has been pruned.
    pub fn clear_dead_varnodes(&mut self) -> KunaResult<()> {
        // iter = vbank.beginLoc(); while(iter!=endLoc()) { vn = *iter++; ... }
        // Collect first (the loop destroys Varnodes, invalidating the BTree
        // iterator); the C++ advances the iterator before any mutation.
        let candidates: Vec<VarnodeId> = self.vbank().iter_loc().collect();
        for vn in candidates {
            let v = match self.vbank().get(vn) {
                Some(v) => v,
                None => continue, // already destroyed this pass
            };
            if !v.has_no_descend() {
                continue;
            }
            // if (vn->isInput() && !vn->isLockedInput()) { makeFree; clearCover; }
            if v.is_input() && !v.is_locked_input() {
                self.vbank_mut().make_free(vn);
                if let Some(vm) = self.vbank_mut().get_mut(vn) {
                    vm.clear_cover();
                }
            }
            // if (vn->isFree()) vbank.destroy(vn);
            if self.vbank().get(vn).map(|v| v.is_free()).unwrap_or(false) {
                self.vbank_mut().destroy(vn)?;
            }
        }
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Input-overlap pre-check (the portable half of setInputVarnode)
    // -----------------------------------------------------------------------

    /// The overlap pre-check of `setInputVarnode` (C++
    /// `funcdata_varnode.cc:347-363`): does an existing input Varnode overlap the
    /// candidate `vn`?
    ///
    /// Returns:
    ///   - `Ok(Some(invn))` if a pre-existing input Varnode has the same size and
    ///     storage address (the candidate should be discarded for `invn`);
    ///   - `Ok(None)` if there is no overlapping input (the candidate may become
    ///     the input);
    ///   - `Err(..)` if an input Varnode overlaps but is *not* identical (the C++
    ///     `LowlevelError("Overlapping input varnodes")`).
    ///
    /// This is the portion of `setInputVarnode` that needs only a `vbank` read.
    /// The subsequent `vbank.setInput(vn)` mutation drives the `xref` callback and
    /// requires the `(vbank,obank)` split accessor (see the module doc); that
    /// mutating tail (plus the `funcp.hasEffect` unaffected/return-address marking,
    /// a W4 seam) is the seam owner's / `funcdata_op`'s.
    pub fn find_input_overlap(&self, vn: VarnodeId) -> KunaResult<Option<VarnodeId>> {
        let v = self.vbank().get(vn).expect("find_input_overlap: stale vn");
        let vaddr = v.get_addr().clone();
        let vsize = v.get_size();
        // iter = vbank.beginDef(Varnode::input, vn->getAddr()+vn->getSize());
        // --iter -> previous input (comes before vn or intersects).  In def-tree
        // order inputs sort first, by Address then size; the element before the
        // lower_bound at `probe_addr` is the input with the greatest Address
        // strictly less than `probe_addr`.
        let probe_addr = &vaddr + (vsize as i64);
        let invn = match self.last_input_before(&probe_addr) {
            Some(id) => id,
            None => return Ok(None), // iter == beginDef(): nothing before
        };
        let other = self.vbank().get(invn).expect("find_input_overlap: stale invn");
        // if (invn->isInput()) { ... }  (always true here by construction)
        if !other.is_input() {
            return Ok(None);
        }
        // if ((-1 != vn->overlap(*invn)) || (-1 != invn->overlap(*vn)))
        let ov1 = self.overlap_vn(vn, invn);
        let ov2 = self.overlap_vn(invn, vn);
        if ov1 != -1 || ov2 != -1 {
            // if (size==size && addr==addr) return invn;
            if vsize == other.get_size() && &vaddr == other.get_addr() {
                return Ok(Some(invn));
            }
            return Err(KunaError::lowlevel("Overlapping input varnodes"));
        }
        Ok(None)
    }

    /// The input Varnode with the greatest Address strictly less than `addr`, in
    /// def-tree order — the `--iter` step-back of `beginDef(input, addr)`.
    fn last_input_before(&self, addr: &Address) -> Option<VarnodeId> {
        let mut last = None;
        // Inputs come out of the def tree in Address-then-size order.
        for id in self.vbank().iter_def_flag(varnode_flags::input) {
            let v = self.vbank().get(id).expect("last_input_before: stale vn");
            if v.get_addr().cmp(addr) == std::cmp::Ordering::Less {
                last = Some(id);
            } else {
                // Once we reach addresses >= addr we are past the lower_bound.
                break;
            }
        }
        last
    }

    /// `a->overlap(*b)` for two varnodes by id (C++ `Varnode::overlap`).
    fn overlap_vn(&self, a: VarnodeId, b: VarnodeId) -> int4 {
        let va = self.vbank().get(a).expect("overlap_vn: stale a");
        let vb = self.vbank().get(b).expect("overlap_vn: stale b");
        va.overlap(vb)
    }

    // -----------------------------------------------------------------------
    // Storage-range finders (C++ inline funcdata.hh delegates to vbank)
    // -----------------------------------------------------------------------

    /// Find the first input Varnode covered by the given range
    /// (C++ `Funcdata::findCoveredInput` → `vbank.findCoveredInput`,
    /// `funcdata.hh:312`).
    pub fn find_covered_input(&self, s: int4, loc: &Address) -> KunaResult<Option<VarnodeId>> {
        self.vbank().find_covered_input(s, loc)
    }

    /// Find the input Varnode that contains the given range
    /// (C++ `Funcdata::findCoveringInput` → `vbank.findCoveringInput`,
    /// `funcdata.hh:319`).
    pub fn find_covering_input(&self, s: int4, loc: &Address) -> KunaResult<Option<VarnodeId>> {
        self.vbank().find_covering_input(s, loc)
    }

    /// Check if an input Varnode exists that overlaps the given range
    /// (C++ `Funcdata::hasInputIntersection` → `vbank.hasInputIntersection`,
    /// `funcdata.hh:326`).
    pub fn has_input_intersection(&self, s: int4, loc: &Address) -> KunaResult<bool> {
        self.vbank().has_input_intersection(s, loc)
    }

    /// Find the input Varnode with the given size and storage address
    /// (C++ `Funcdata::findVarnodeInput` → `vbank.findInput`, `funcdata.hh:333`).
    pub fn find_varnode_input(&self, s: int4, loc: &Address) -> Option<VarnodeId> {
        self.vbank().find_input(s, loc)
    }

    /// Find a defined Varnode via its storage address and definition address
    /// (C++ `Funcdata::findVarnodeWritten` → `vbank.find`, `funcdata.hh:342`).
    ///
    /// `uniq == None` is the C++ `~0` "don't care" sentinel.  The def-op
    /// address/time confirmation is supplied by [`Funcdata::def_addr_time`].
    pub fn find_varnode_written(
        &self,
        s: int4,
        loc: &Address,
        pc: &Address,
        uniq: Option<uintm>,
    ) -> Option<VarnodeId> {
        self.vbank().find(s, loc, pc, uniq, &|op| self.def_addr_time(op))
    }

    // -----------------------------------------------------------------------
    // descend iteration helpers
    // -----------------------------------------------------------------------

    /// Iterate (a snapshot of) the ops reading `vn` as input, in `descend` order
    /// (C++ `vn->beginDescend()`..`endDescend()`).
    ///
    /// Returns a `Vec` because most callers mutate the graph while iterating (the
    /// C++ `op = *iter++` cursor-before-mutate idiom), and a borrowed iterator
    /// would alias the bank.  Order is the C++ `list` order exactly.
    pub fn descend_snapshot(&self, vn: VarnodeId) -> Vec<OpId> {
        self.vbank().get(vn).map(|v| v.descend_iter().collect()).unwrap_or_default()
    }

    /// Return the single op reading `vn`, or `None` if there are zero or more
    /// than one (C++ `Varnode::loneDescend`, `varnode.cc:301`).
    pub fn lone_descend(&self, vn: VarnodeId) -> Option<OpId> {
        let v = self.vbank().get(vn)?;
        let mut it = v.descend_iter();
        let first = it.next()?;
        if it.next().is_some() {
            return None; // more than one descendant
        }
        Some(first)
    }

    // -----------------------------------------------------------------------
    // linkSpacebaseSymbol / linkSymbolReference (the `&name` render payoff)
    // -----------------------------------------------------------------------

    /// C++ `Funcdata::linkSymbolReference` (`funcdata_varnode.cc:1214`), realized
    /// against the merged tree's HighVariable `kuna_*` symbol-reference fields.
    ///
    /// `offVn` is the constant **second input** to a `PTRSUB(spacebase, offset)`.
    /// The C++ recovers the Symbol the constant address refers to and stores it on
    /// the constant's HighVariable as a *reference* (`Varnode::setSymbolReference`
    /// -> `HighVariable::setSymbolReference(sym, off)` sets `symbol`/`symboloffset`,
    /// `variable.cc:283-289`).  The W4 `Symbol`/`SymbolEntry` layer is absent here,
    /// so the reference is parked as the same `kuna_name`/`symbol_offset`/
    /// `kuna_symbol_type` triple `ActionNameVars` uses for value reads; the printer
    /// reads it back in the `PrintC::opPtrsub` SPACEBASE arm (`printc.cc:1081`).
    ///
    /// Faithful to the C++ body:
    /// ```text
    /// op = vn->loneDescend();
    /// ptype = (TypePointer *)op->getIn(0)->getHigh()->getType();
    /// if (ptype->getMetatype() != TYPE_PTR) return 0;
    /// sb = (TypeSpacebase *)ptype->getPtrTo();
    /// if (sb->getMetatype() != TYPE_SPACEBASE) return 0;
    /// scope = sb->getMap();
    /// addr = sb->getAddress(vn->getOffset(), in0->getSize(), op->getAddr());
    /// entry = scope->queryContainer(addr, 1, Address());
    /// if (entry == 0) return 0;
    /// off = (int4)(addr.getOffset() - entry->getAddr().getOffset()) + entry->getOffset();
    /// vn->setSymbolReference(entry, off);
    /// ```
    ///
    /// Returns `true` when a Symbol reference was attached (the C++ non-null return,
    /// used by `linkSpacebaseSymbol` to decide whether to record the offVn for later
    /// default-naming).
    fn link_symbol_reference(&mut self, off_vn: VarnodeId) -> bool {
        // op = vn->loneDescend();  (the PTRSUB this constant feeds)
        let op = match self.lone_descend(off_vn) {
            Some(o) => o,
            None => return false,
        };
        // in0 = op->getIn(0)  (the spacebase varnode)
        let in0 = match self.obank().get(op).and_then(|o| o.get_in(0)) {
            Some(v) => v,
            None => return false,
        };
        // ptype = in0->getHigh()->getType().  The kuna render reads the varnode
        // type for the same spacebase input (`op_ptrsub_ir` line in0 = get_type),
        // which after the W10 spacebase-input typing IS the pointer-to-spacebase;
        // use the same source so the action and the render agree.
        let (ptype, in0_size) = match self.vbank().get(in0) {
            Some(v) => (Rc::clone(v.get_type()), v.get_size()),
            None => return false,
        };
        // if (ptype->getMetatype() != TYPE_PTR) return 0;
        if ptype.get_metatype() != type_metatype::TYPE_PTR {
            return false;
        }
        // sb = (TypeSpacebase *)ptype->getPtrTo();
        let sb = match ptype.get_ptr_to() {
            Some(s) => s,
            None => return false,
        };
        // if (sb->getMetatype() != TYPE_SPACEBASE) return 0;
        if sb.get_metatype() != type_metatype::TYPE_SPACEBASE {
            return false;
        }
        // off_vn->getOffset() and op->getAddr()
        let off_const = match self.vbank().get(off_vn) {
            Some(v) => v.get_offset(),
            None => return false,
        };
        let op_addr = match self.obank().get(op) {
            Some(o) => o.get_addr().clone(),
            None => return false,
        };
        // addr = sb->getAddress(vn->getOffset(), in0->getSize(), op->getAddr());
        let manager = self.get_arch().manage();
        let addr = match sb.spacebase_get_address(off_const, in0_size, &op_addr, manager) {
            Some(a) => a,
            // C++ throws "Unable to generate proper address from spacebase"; a
            // failure to resolve simply means no reference is attached here.
            None => return false,
        };
        // scope = sb->getMap(); entry = scope->queryContainer(addr,1,Address());
        // off = (addr - entry.addr) + entry.offset.  `query_container_for_link`
        // performs exactly this lookup and offset arithmetic and returns the
        // display name + in-symbol byte offset + symbol type, or None when no
        // entry contains `addr` (the C++ `entry == 0 -> return 0`).
        //
        // C++ `sb->getMap()` returns the scope tied to the spacebase's space: the
        // LOCAL scope for a stack spacebase, the GLOBAL scope for a `ram`/global
        // spacebase (the `PTRSUB(spacebaseConstant, off)` `ActionConstantPtr`
        // builds, W10 type-seed).  Query the local scope first; on a miss fall back
        // to `glb`'s frozen global-scope snapshot (`name_for_global_varnode`, the
        // same `findContainer` + `(addr-entryAddr)+entryOffset` geometry restricted
        // to the global scope) so a mapped global array (`map addr r0x601060 int4
        // myarray[3][5]`) binds its name onto the offset constant's HighVariable.
        // `(display_name, sym_off, sym_type, is_name_undefined)` from whichever
        // scope contains `addr` (local stack frame first, then the global scope).
        let resolved: Option<(String, int4, Option<Rc<Datatype>>, bool)> = match self
            .get_scope_local()
            .and_then(|lm| lm.query_container_for_link(&addr))
        {
            Some(i) => Some((i.display_name, i.sym_off, i.sym_type, i.is_name_undefined)),
            None => {
                // Global-scope fallback: glb->getGlobalScope()->queryContainer.  A
                // global `map addr` symbol always has a defined name.
                let invalid = Address::new_invalid();
                self.get_arch()
                    .name_for_global_varnode(&addr, 1, &invalid)
                    .map(|(display_name, sym_off, sym_type)| {
                        (display_name, sym_off, sym_type, false)
                    })
            }
        };
        let (info_display_name, info_sym_off, info_sym_type, info_is_name_undefined) =
            match resolved {
                Some(i) => i,
                None => return false,
            };
        // C++ `linkSpacebaseSymbol` (coreaction.cc:3015) attaches the reference even
        // for an undefined-named Symbol, then records the offVn in `namerec` so
        // `ActionNameVars::apply` renames the shared Symbol to `vN` (the render then
        // reads that final name).  The kuna namerec rename only reaches
        // whole-symbol-cover locals (`resolve_default_name` renames in the
        // database), so an undefined-named composite — an unmapped array auto-local
        // — is never given a `vN` here.  Binding its raw `$$undefNN` placeholder
        // onto the offset-constant high would leak it into the render
        // (`&$$undef00000001`); instead leave the reference unattached so the
        // printer's SPACEBASE arm falls back to the functional `PTRSUB(...)` form —
        // observably identical to the pre-render-finish state for an unmapped local.
        // A *mapped* Symbol (`map addr ... a[1]`) always has a defined name, so this
        // never suppresses the `&a` / `&myval.b` payoff.
        if info_is_name_undefined {
            return false;
        }
        // vn->setSymbolReference(entry, off):
        //   HighVariable::setSymbolReference(sym, off) — symbol = sym;
        //   symboloffset = off.  Parked on the constant's HighVariable.
        let high = match self.vbank().get(off_vn).and_then(|v| v.get_high()) {
            Some(h) => h,
            None => return false,
        };
        if let Some(h) = self.high_bank_mut().get_mut(high) {
            h.set_kuna_name(info_display_name);
            h.set_symbol_offset(info_sym_off);
            if let Some(t) = info_sym_type {
                h.set_symbol_type(t);
            }
        }
        true
    }

    /// C++ `ActionNameVars::linkSpacebaseSymbol` (`coreaction.cc:3005`): look for
    /// `PTRSUB` ops off the given spacebase Varnode that encode a `&symbol`
    /// reference, and link each one's offset constant to its Symbol.
    ///
    /// ```text
    /// if (!vn->isConstant() && !vn->isInput()) return;
    /// for (op : vn->descend()) {
    ///   if (op->code() != CPUI_PTRSUB) continue;
    ///   offVn = op->getIn(1);
    ///   sym = data.linkSymbolReference(offVn);
    ///   if (sym != 0 && sym->isNameUndefined()) namerec.push_back(offVn);
    /// }
    /// ```
    ///
    /// The kuna mapped symbols always have a defined name (the `map addr` records),
    /// so the `isNameUndefined()` namerec arm — which would route an unnamed
    /// spacebase symbol through `buildDefaultName` — is a no-op for the corpus and
    /// is intentionally not reproduced (no symbol reaching here is name-undefined).
    pub fn link_spacebase_symbol(&mut self, vn: VarnodeId) {
        // if (!vn->isConstant() && !vn->isInput()) return;
        let (is_const, is_input) = match self.vbank().get(vn) {
            Some(v) => (v.is_constant(), v.is_input()),
            None => return,
        };
        if !is_const && !is_input {
            return;
        }
        // Snapshot the descend list (the body does not mutate it, but a snapshot
        // keeps the borrow short while link_symbol_reference takes &mut self).
        for op in self.descend_snapshot(vn) {
            // if (op->code() != CPUI_PTRSUB) continue;
            if self.obank().get(op).map(|o| o.code()) != Some(OpCode::CPUI_PTRSUB) {
                continue;
            }
            // offVn = op->getIn(1);
            let off_vn = match self.obank().get(op).and_then(|o| o.get_in(1)) {
                Some(v) => v,
                None => continue,
            };
            // sym = data.linkSymbolReference(offVn);  (namerec arm omitted, above)
            let _ = self.link_symbol_reference(off_vn);
        }
    }

    // -----------------------------------------------------------------------
    // totalReplace (def-use rewiring; sequenced single-arena borrows)
    // -----------------------------------------------------------------------

    /// Replace all read references of `vn` with `newvn`
    /// (C++ `Funcdata::totalReplace`, `funcdata_varnode.cc:1495`).
    ///
    /// ```text
    /// iter = vn->beginDescend();
    /// while(iter != vn->endDescend()) {
    ///   op = *iter++;                 // Increment before removing descendant
    ///   i = op->getSlot(vn);
    ///   opSetInput(op,newvn,i);
    /// }
    /// ```
    ///
    /// `opSetInput` is replicated inline ([`Funcdata::op_set_input`]); each of its
    /// steps (`eraseDescend` ; `clearInput` ; `addDescend` ; `setInput`) is a
    /// distinct single-arena borrow, so the whole rewiring sequences cleanly
    /// without the `(vbank,obank)` split.
    pub fn total_replace(&mut self, vn: VarnodeId, newvn: VarnodeId) -> KunaResult<()> {
        // Snapshot the descend list (we mutate it as we go, mirroring `*iter++`).
        let readers = self.descend_snapshot(vn);
        for op in readers {
            // i = op->getSlot(vn);  (the first slot still reading vn — a prior
            // entry for the same op may have already been consumed, leaving -1,
            // in which case there is nothing to repoint; this matches the C++
            // where each descend entry corresponds to exactly one read.)
            let i = self.obank().get(op).map(|o| o.get_slot(vn)).unwrap_or(-1);
            if i < 0 {
                continue;
            }
            self.op_set_input_local(op, newvn, i)?;
        }
        Ok(())
    }

    /// Combine two contiguous input Varnodes into one logical input (C++
    /// `Funcdata::combineInputVarnodes`, `funcdata_varnode.cc:383`).
    ///
    /// A new Varnode covering both originals is created and marked as a function
    /// input.  Each `CPUI_PIECE` reading the two originals becomes a `CPUI_COPY`
    /// reading the new whole; any *other* reader of an original is repointed to a
    /// `SUBPIECE` of the new whole.  The originals are destroyed.  Drives
    /// `RuleDoubleOut::applyOp` (double-precision register-pair recombine).
    ///
    /// ```text
    /// if (!vnHi->isInput() || !vnLo->isInput()) throw "not inputs";
    /// addr = vnLo->getAddr();
    /// if (addr.isBigEndian()) { addr = vnHi->getAddr(); isContiguous = (vnHi->getAddr()+vnHi->getSize() == vnLo->getAddr()); }
    /// else                    { isContiguous = (vnLo->getAddr()+vnLo->getSize() == vnHi->getAddr()); }
    /// if (!isContiguous) throw "not contiguous";
    /// ... collect PIECE readers, note any other readers of hi/lo ...
    /// for(piece : pieceList) { opRemoveInput(piece,1); opUnsetInput(piece,0); }
    /// if (otherOpsHi) { subHi=newOp@bb0; SUBPIECE,const(vnLo->getSize()); newHi=out@vnHi->getAddr(); insertBegin; totalReplace(vnHi,newHi); }
    /// if (otherOpsLo) { subLo=newOp@bb0; SUBPIECE,const(0);            newLo=out@vnLo->getAddr(); insertBegin; totalReplace(vnLo,newLo); }
    /// outSize = vnHi->getSize()+vnLo->getSize();
    /// vbank.destroy(vnHi); vbank.destroy(vnLo);
    /// inVn = setInputVarnode(newVarnode(outSize, addr));
    /// for(piece : pieceList) { opSetInput(piece,inVn,0); opSetOpcode(piece, COPY); }
    /// if (otherOpsHi) opSetInput(subHi,inVn,0);
    /// if (otherOpsLo) opSetInput(subLo,inVn,0);
    /// ```
    pub fn combine_input_varnodes(
        &mut self,
        vn_hi: VarnodeId,
        vn_lo: VarnodeId,
    ) -> KunaResult<()> {
        // if (!vnHi->isInput() || !vnLo->isInput()) throw "not inputs";
        let (hi_is_input, hi_addr, hi_size) = {
            let v = self.vbank().get(vn_hi).expect("combine_input_varnodes: stale vnHi");
            (v.is_input(), v.get_addr().clone(), v.get_size())
        };
        let (lo_is_input, lo_addr, lo_size) = {
            let v = self.vbank().get(vn_lo).expect("combine_input_varnodes: stale vnLo");
            (v.is_input(), v.get_addr().clone(), v.get_size())
        };
        if !hi_is_input || !lo_is_input {
            return Err(KunaError::lowlevel("Varnodes being combined are not inputs"));
        }
        // Address addr = vnLo->getAddr(); ... compute contiguity by endianness.
        let is_contiguous: bool;
        let addr: Address;
        if lo_addr.is_big_endian() {
            // addr = vnHi->getAddr(); otheraddr = addr + vnHi->getSize(); contig = (otheraddr == vnLo->getAddr());
            addr = hi_addr.clone();
            let otheraddr = &hi_addr + hi_size as i64;
            is_contiguous = otheraddr == lo_addr;
        } else {
            // otheraddr = addr + vnLo->getSize(); contig = (otheraddr == vnHi->getAddr());
            addr = lo_addr.clone();
            let otheraddr = &lo_addr + lo_size as i64;
            is_contiguous = otheraddr == hi_addr;
        }
        if !is_contiguous {
            return Err(KunaError::lowlevel(
                "Input varnodes being combined are not contiguous",
            ));
        }

        // Partition the readers of vnHi/vnLo: PIECE(vnHi,vnLo) ops vs. all others.
        let mut piece_list: Vec<OpId> = Vec::new();
        let mut other_ops_hi = false;
        let mut other_ops_lo = false;
        // for(iter=vnHi->beginDescend(); ...) { if (op==PIECE && in0==vnHi && in1==vnLo) pieceList.push(op); else otherOpsHi=true; }
        for op in self.descend_snapshot(vn_hi) {
            let o = self.obank().get(op).expect("combine_input_varnodes: stale hi reader");
            if o.code() == OpCode::CPUI_PIECE
                && o.get_in(0) == Some(vn_hi)
                && o.get_in(1) == Some(vn_lo)
            {
                piece_list.push(op);
            } else {
                other_ops_hi = true;
            }
        }
        // for(iter=vnLo->beginDescend(); ...) { if (op!=PIECE || in0!=vnHi || in1!=vnLo) otherOpsLo=true; }
        for op in self.descend_snapshot(vn_lo) {
            let o = self.obank().get(op).expect("combine_input_varnodes: stale lo reader");
            if o.code() != OpCode::CPUI_PIECE
                || o.get_in(0) != Some(vn_hi)
                || o.get_in(1) != Some(vn_lo)
            {
                other_ops_lo = true;
            }
        }
        // for(i...) { opRemoveInput(pieceList[i], 1); opUnsetInput(pieceList[i], 0); }
        for &piece in &piece_list {
            self.op_remove_input(piece, 1);
            self.op_unset_input(piece, 0);
        }

        // If non-PIECE readers exist, build replacement SUBPIECEs of the new whole.
        let mut sub_hi: Option<OpId> = None;
        let mut sub_lo: Option<OpId> = None;
        if other_ops_hi {
            // BlockBasic *bb = bblocks.getBlock(0);
            let bb = self.bblocks_get_block(0);
            let bbstart = self.bblocks_block_start(bb);
            // subHi = newOp(2,bb->getStart()); SUBPIECE; in1 = newConstant(4, vnLo->getSize());
            let op = self.new_op(2, bbstart);
            self.op_set_opcode_code(op, OpCode::CPUI_SUBPIECE);
            let con = self.new_constant(4, lo_size as uintb);
            self.op_set_input(op, con, 1)?;
            // newHi = newVarnodeOut(vnHi->getSize(), vnHi->getAddr(), subHi);
            let new_hi = self.new_varnode_out(hi_size, &hi_addr, op)?;
            self.op_insert_begin(op, bb);
            // totalReplace(vnHi, newHi);
            self.total_replace(vn_hi, new_hi)?;
            sub_hi = Some(op);
        }
        if other_ops_lo {
            let bb = self.bblocks_get_block(0);
            let bbstart = self.bblocks_block_start(bb);
            // subLo = newOp(2,bb->getStart()); SUBPIECE; in1 = newConstant(4, 0);
            let op = self.new_op(2, bbstart);
            self.op_set_opcode_code(op, OpCode::CPUI_SUBPIECE);
            let con = self.new_constant(4, 0);
            self.op_set_input(op, con, 1)?;
            // newLo = newVarnodeOut(vnLo->getSize(), vnLo->getAddr(), subLo);
            let new_lo = self.new_varnode_out(lo_size, &lo_addr, op)?;
            self.op_insert_begin(op, bb);
            // totalReplace(vnLo, newLo);
            self.total_replace(vn_lo, new_lo)?;
            sub_lo = Some(op);
        }

        // outSize = vnHi->getSize() + vnLo->getSize();
        let out_size = hi_size + lo_size;
        // vbank.destroy(vnHi); vbank.destroy(vnLo);
        self.vbank_mut().destroy(vn_hi)?;
        self.vbank_mut().destroy(vn_lo)?;
        // inVn = newVarnode(outSize, addr); inVn = setInputVarnode(inVn);
        let in_vn = self.new_varnode(out_size, &addr, None);
        let in_vn = self.set_input_varnode(in_vn)?;
        // for(i...) { opSetInput(pieceList[i],inVn,0); opSetOpcode(pieceList[i], COPY); }
        for &piece in &piece_list {
            self.op_set_input(piece, in_vn, 0)?;
            self.op_set_opcode_code(piece, OpCode::CPUI_COPY);
        }
        // if (otherOpsHi) opSetInput(subHi,inVn,0);
        if let Some(op) = sub_hi {
            self.op_set_input(op, in_vn, 0)?;
        }
        // if (otherOpsLo) opSetInput(subLo,inVn,0);
        if let Some(op) = sub_lo {
            self.op_set_input(op, in_vn, 0)?;
        }
        Ok(())
    }

    /// Set input `slot` of `op` to read `vn` (C++ `Funcdata::opSetInput`,
    /// `funcdata_op.cc:104`), replicated for the `totalReplace` rewiring.
    ///
    /// This is a faithful local copy of the op-side `opSetInput`.  When this wave
    /// was authored the canonical public `opSetInput` did not yet exist, so this
    /// was a same-named private helper "kept private so there is no clash"; once
    /// the `funcdata_op` wave landed its public `op_set_input` (an inherent method
    /// on the same `Funcdata`), two inherent methods of identical name collided
    /// (Rust forbids this regardless of visibility).  The W3-serial-chain
    /// integration (`w3-ir-flow`) renamed this copy to `op_set_input_local` — a
    /// minimal reconciliation that preserves this wave's constant-clone behavior
    /// (which `funcdata_op`'s public copy seam-defers, so `total_replace` here
    /// must retain the cloning the `total_replace_constant_clones_per_read` test
    /// pins).  Recorded as a loss; the eventual single canonical `opSetInput`
    /// should fold the constant-clone in and this local copy should disappear.
    ///
    /// ```text
    /// if (vn == op->getIn(slot)) return;            // Already set
    /// if (vn->isConstant() && !hasNoDescend && !isSpacebase) {
    ///   cvn = newConstant(size, offset); cvn->copySymbol(vn); vn = cvn;
    /// }
    /// if (op->getIn(slot) != 0) opUnsetInput(op,slot); // erase old descend+clear
    /// vn->addDescend(op); op->setInput(vn,slot);
    /// ```
    fn op_set_input_local(&mut self, op: OpId, mut vn: VarnodeId, slot: int4) -> KunaResult<()> {
        // if (vn == op->getIn(slot)) return;
        if self.obank().get(op).and_then(|o| o.get_in(slot)) == Some(vn) {
            return Ok(());
        }
        // Constants get only one descendant (unless spacebase): clone on reuse.
        {
            let v = self.vbank().get(vn).expect("op_set_input: stale vn");
            if v.is_constant() && !v.has_no_descend() && !v.is_spacebase() {
                let sz = v.get_size();
                let off = v.get_offset();
                let cvn = self.new_constant(sz, off);
                // cvn->copySymbol(vn): copy the data-type; the symbol/typelock
                // markup is the W4 symbol seam.  -- SEAM(W4): mapentry/typelock.
                let ty = self.vbank().get(vn).expect("op_set_input: stale vn").get_type().clone();
                self.vbank_mut().get_mut(cvn).expect("op_set_input: stale cvn").update_type(ty);
                vn = cvn;
            }
        }
        // if (op->getIn(slot) != 0) opUnsetInput(op,slot);
        let old = self.obank().get(op).and_then(|o| o.get_in(slot));
        if let Some(oldvn) = old {
            // opUnsetInput: vn->eraseDescend(op); op->clearInput(slot);
            self.vbank_mut().erase_descend(oldvn, op);
            self.obank_mut().get_mut(op).expect("op_set_input: stale op").clear_input(slot);
        }
        // vn->addDescend(op); op->setInput(vn,slot);  (descend add BEFORE setInput)
        self.vbank_mut().add_descend(vn, op)?;
        self.obank_mut().get_mut(op).expect("op_set_input: stale op").set_input(Some(vn), slot);
        Ok(())
    }

    /// Treat the given Varnode as read-only, look up its value in the LoadImage
    /// and replace read references with the value as a constant Varnode (C++
    /// `Funcdata::fillinReadOnly`, `funcdata_varnode.cc:653`).
    ///
    /// Returns `true` if any change was made.  Driven by `ActionVarnodeProps` when
    /// `glb->readonlypropagate` is set (the `option readonly` switch).  The float
    /// cluster's `v1 = 1.1234567812345` initializers come from folding the
    /// IEEE-754 bytes that live in read-only RAM into a constant here.
    pub fn fillin_read_only(&mut self, vn: VarnodeId) -> KunaResult<bool> {
        // if (vn->isWritten()) { ... return false; }  -- Can't replace output
        let (is_written, defop) = {
            let v = self.vbank().get(vn).expect("fillin_read_only: stale vn");
            (v.is_written(), v.get_def())
        };
        if is_written {
            let defop = defop.expect("fillin_read_only: written vn has no def");
            // if (defop->isMarker()) defop->setAdditionalFlag(warning);
            let is_marker = self.obank().get(defop).expect("fillin_read_only: stale def").is_marker();
            if is_marker {
                self.obank_mut()
                    .get_mut(defop)
                    .expect("fillin_read_only: stale def")
                    .set_additional_flag(crate::op::pcodeop_addlflags::warning);
            } else {
                // else if (!defop->isWarning()) { ... warning(...) ... }
                let already_warned =
                    self.obank().get(defop).expect("fillin_read_only: stale def").is_warning();
                if !already_warned {
                    self.obank_mut()
                        .get_mut(defop)
                        .expect("fillin_read_only: stale def")
                        .set_additional_flag(crate::op::pcodeop_addlflags::warning);
                    // if ((!vn->isAddrForce())||(!vn->hasNoDescend())) { warning(...) }
                    let (addr_force, no_descend) = {
                        let v = self.vbank().get(vn).expect("fillin_read_only: stale vn");
                        (v.is_addr_force(), v.has_no_descend())
                    };
                    if !addr_force || !no_descend {
                        let (spcname, vaddr) = {
                            let v = self.vbank().get(vn).expect("fillin_read_only: stale vn");
                            (v.get_space().get_name().to_string(), v.get_addr().clone())
                        };
                        // s << "Read-only address (" << space << ',' << addr << ") is written"
                        let mut s = format!("Read-only address ({},", spcname);
                        let _ = vaddr.print_raw(&mut s);
                        s.push_str(") is written");
                        let defaddr = self.obank().get(defop).expect("fillin_read_only").get_addr().clone();
                        self.warning(&s, &defaddr);
                    }
                }
            }
            return Ok(false); // No change was made
        }

        // if (vn->getSize() > sizeof(uintb)) return false;  -- exceeds precision
        let vnsize = self.vbank().get(vn).expect("fillin_read_only: stale vn").get_size();
        if (vnsize as usize) > std::mem::size_of::<uintb>() {
            return Ok(false);
        }

        // try { glb->loader->loadFill(bytes,vn->getSize(),vn->getAddr()); }
        // catch(DataUnavailError) { vn->clearFlags(readonly); return true; }
        let (vaddr, big_endian) = {
            let v = self.vbank().get(vn).expect("fillin_read_only: stale vn");
            (v.get_addr().clone(), v.get_space().is_big_endian())
        };
        let glb = self.get_arch().clone();
        let mut bytes = [0u8; 32];
        if glb.loader_fill(&mut bytes[..vnsize as usize], &vaddr).is_err() {
            // Could not get value from LoadImage: treat as writeable.
            self.vbank_mut()
                .get_mut(vn)
                .expect("fillin_read_only: stale vn")
                .clear_flags_pub(varnode_flags::readonly);
            return Ok(true);
        }

        // Assemble the constant, honoring the space endianness (C++ shifts the
        // accumulator left a byte per step, most-significant byte first).
        let mut res: uintb = 0;
        if big_endian {
            // for(i=0;i<size;++i) { res<<=8; res|=bytes[i]; }
            for b in &bytes[..vnsize as usize] {
                res = res.wshl(8) | (*b as uintb);
            }
        } else {
            // for(i=size-1;i>=0;--i) { res<<=8; res|=bytes[i]; }
            for b in bytes[..vnsize as usize].iter().rev() {
                res = res.wshl(8) | (*b as uintb);
            }
        }

        // Replace all references to vn.
        // Datatype *locktype = vn->isTypeLock() ? vn->getType() : 0;
        let locktype: Option<Rc<Datatype>> = {
            let v = self.vbank().get(vn).expect("fillin_read_only: stale vn");
            if v.is_type_lock() {
                Some(Rc::clone(v.get_type()))
            } else {
                None
            }
        };

        let mut changemade = false;
        // iter = vn->beginDescend(); while(iter != endDescend()) { op = *iter++; ... }
        let readers = self.descend_snapshot(vn);
        for op in readers {
            // i = op->getSlot(vn);  (a stale -1 means this read was already retired
            // by a prior INDIRECT->COPY rewrite; the C++ descend entry is 1:1, so
            // re-resolve per entry and skip if vn is no longer read here.)
            let i = self.obank().get(op).map(|o| o.get_slot(vn)).unwrap_or(-1);
            if i < 0 {
                continue;
            }
            // if (op->isMarker()) { ... }
            let is_marker = self.obank().get(op).expect("fillin_read_only: stale op").is_marker();
            if is_marker {
                // if ((op->code()!=CPUI_INDIRECT)||(i!=0)) continue;
                let code = self.obank().get(op).expect("fillin_read_only: stale op").code();
                if code != OpCode::CPUI_INDIRECT || i != 0 {
                    continue;
                }
                // Varnode *outvn = op->getOut();
                let outvn = match self.obank().get(op).expect("fillin_read_only: stale op").get_out() {
                    Some(o) => o,
                    None => continue,
                };
                // if (outvn->getAddr() == vn->getAddr()) continue; // Ignore indirect to itself
                let same_addr = {
                    let ov = self.vbank().get(outvn).expect("fillin_read_only: stale outvn");
                    ov.get_addr() == &vaddr
                };
                if same_addr {
                    continue;
                }
                // Change the indirect to a COPY: opRemoveInput(op,1); opSetOpcode(op,COPY);
                self.op_remove_input(op, 1);
                self.op_set_opcode_code(op, OpCode::CPUI_COPY);
            }
            // Varnode *cvn = newConstant(vn->getSize(),res);
            let cvn = self.new_constant(vnsize, res);
            // if (locktype) cvn->updateType(locktype,true,true);
            if let Some(lt) = &locktype {
                self.vbank_mut()
                    .get_mut(cvn)
                    .expect("fillin_read_only: stale cvn")
                    .update_type_locked(Rc::clone(lt), true, true);
            }
            // opSetInput(op,cvn,i);
            self.op_set_input(op, cvn, i)?;
            changemade = true;
        }
        Ok(changemade)
    }

    /// Replace every read reference of the given Varnode with a constant value
    /// (C++ `Funcdata::totalReplaceConstant`, `funcdata_varnode.cc:1517`).
    ///
    /// A new constant Varnode is created for each read site.  For any marker op
    /// (MULTIEQUAL/INDIRECT) a single COPY op is inserted and the marker input is
    /// set to the COPY output (constants may not go directly into a marker).
    /// Driven by `ActionVarnodeProps` for a Varnode whose consumed bits and
    /// non-zero mask are disjoint (it can only carry the value 0).
    pub fn total_replace_constant(&mut self, vn: VarnodeId, val: uintb) -> KunaResult<()> {
        let vnsize = self.vbank().get(vn).expect("total_replace_constant: stale vn").get_size();
        // PcodeOp *copyop = 0;  (the single shared COPY for marker reads)
        let mut copyop: Option<OpId> = None;
        // iter = vn->beginDescend(); while(iter != endDescend()) { op = *iter++; ... }
        let readers = self.descend_snapshot(vn);
        for op in readers {
            let i = self.obank().get(op).map(|o| o.get_slot(vn)).unwrap_or(-1);
            if i < 0 {
                continue;
            }
            let is_marker = self.obank().get(op).expect("total_replace_constant: stale op").is_marker();
            let newrep: VarnodeId = if is_marker {
                // Do not put constant directly in marker.
                if let Some(existing) = copyop {
                    // newrep = copyop->getOut();
                    self.obank()
                        .get(existing)
                        .expect("total_replace_constant: stale copyop")
                        .get_out()
                        .expect("total_replace_constant: copyop has no out")
                } else {
                    let (is_written, def) = {
                        let v = self.vbank().get(vn).expect("total_replace_constant: stale vn");
                        (v.is_written(), v.get_def())
                    };
                    let cop = if is_written {
                        // copyop = newOp(1, vn->getDef()->getAddr());
                        let defop = def.expect("total_replace_constant: written vn has no def");
                        let defaddr =
                            self.obank().get(defop).expect("total_replace_constant: stale def").get_addr().clone();
                        let cop = self.new_op(1, defaddr);
                        self.op_set_opcode_code(cop, OpCode::CPUI_COPY);
                        let newrep = self.new_unique_out(vnsize, cop)?;
                        let con = self.new_constant(vnsize, val);
                        self.op_set_input(cop, con, 0)?;
                        self.op_insert_after(cop, defop);
                        let _ = newrep;
                        cop
                    } else {
                        // BlockBasic *bb = getBasicBlocks().getBlock(0);
                        let bb = self.bblocks_get_block(0);
                        let bbstart = self.bblocks_block_start(bb);
                        let cop = self.new_op(1, bbstart);
                        self.op_set_opcode_code(cop, OpCode::CPUI_COPY);
                        let _newrep = self.new_unique_out(vnsize, cop)?;
                        let con = self.new_constant(vnsize, val);
                        self.op_set_input(cop, con, 0)?;
                        self.op_insert_begin(cop, bb);
                        cop
                    };
                    copyop = Some(cop);
                    self.obank()
                        .get(cop)
                        .expect("total_replace_constant: stale copyop")
                        .get_out()
                        .expect("total_replace_constant: copyop has no out")
                }
            } else {
                // newrep = newConstant(vn->getSize(), val);
                self.new_constant(vnsize, val)
            };
            // opSetInput(op,newrep,i);
            self.op_set_input(op, newrep, i)?;
        }
        Ok(())
    }

    /// Model the given Varnode's volatile read/write as a special user op (C++
    /// `Funcdata::replaceVolatile`, `funcdata_varnode.cc:733`).
    ///
    /// The Varnode (assumed not fully linked) is replaced by a temporary within
    /// the data-flow, and its address becomes a CALLOTHER (volatile-read /
    /// volatile-write builtin) parameter, so the read/write side effect survives.
    /// Returns `true` if a change was made.  Driven by `ActionVarnodeProps` for a
    /// volatile Varnode.  The builtin user-op *index* is a fixed constant
    /// (`UserPcodeOp::BUILTIN_VOLATILE_{READ,WRITE}`); the C++ `registerBuiltin`
    /// side-effect (installing the op for the print pass) is the userop-print
    /// wave's, so the index is used directly here.
    pub fn replace_volatile(&mut self, vn: VarnodeId) -> KunaResult<bool> {
        use crate::userop::{BUILTIN_VOLATILE_READ, BUILTIN_VOLATILE_WRITE};
        let (is_written, vaddr, vnsize, is_type_lock) = {
            let v = self.vbank().get(vn).expect("replace_volatile: stale vn");
            (v.is_written(), v.get_addr().clone(), v.get_size(), v.is_type_lock())
        };
        let newop: OpId;
        if is_written {
            // A written value -> volatile write.
            // if (!vn->hasNoDescend()) throw "Volatile memory was propagated";
            let no_descend =
                self.vbank().get(vn).expect("replace_volatile: stale vn").has_no_descend();
            if !no_descend {
                return Err(KunaError::lowlevel("Volatile memory was propagated"));
            }
            let defop = self
                .vbank()
                .get(vn)
                .expect("replace_volatile: stale vn")
                .get_def()
                .expect("replace_volatile: written vn has no def");
            let defaddr =
                self.obank().get(defop).expect("replace_volatile: stale def").get_addr().clone();
            newop = self.new_op(3, defaddr);
            self.op_set_opcode_code(newop, OpCode::CPUI_CALLOTHER);
            // opSetInput(newop, newConstant(4, vw_op->getIndex()), 0);
            let idx_con = self.new_constant(4, BUILTIN_VOLATILE_WRITE as uintb);
            self.op_set_input(newop, idx_con, 0)?;
            // annoteVn = newCodeRef(vn->getAddr()); annoteVn->setFlags(volatil);
            let annote = self.new_code_ref(&vaddr);
            self.vbank_mut()
                .get_mut(annote)
                .expect("replace_volatile: stale annote")
                .set_flags_pub(varnode_flags::volatil);
            self.op_set_input(newop, annote, 1)?;
            // tmp = newUnique(vn->getSize()); opSetOutput(defop, tmp);
            let tmp = self.new_unique(vnsize, None);
            self.op_set_output(defop, tmp)?;
            // opSetInput(newop, tmp, 2); opInsertAfter(newop, defop);
            self.op_set_input(newop, tmp, 2)?;
            self.op_insert_after(newop, defop);
        } else {
            // A read value -> volatile read.
            // if (vn->hasNoDescend()) return false; // Dead
            let no_descend =
                self.vbank().get(vn).expect("replace_volatile: stale vn").has_no_descend();
            if no_descend {
                return Ok(false);
            }
            // readop = vn->loneDescend(); if (0) throw "used more than once";
            let readop = self
                .lone_descend(vn)
                .ok_or_else(|| KunaError::lowlevel("Volatile memory value used more than once"))?;
            let readaddr =
                self.obank().get(readop).expect("replace_volatile: stale readop").get_addr().clone();
            newop = self.new_op(2, readaddr);
            self.op_set_opcode_code(newop, OpCode::CPUI_CALLOTHER);
            // tmp = newUniqueOut(vn->getSize(), newop);
            let tmp = self.new_unique_out(vnsize, newop)?;
            // opSetInput(newop, newConstant(4, vr_op->getIndex()), 0);
            let idx_con = self.new_constant(4, BUILTIN_VOLATILE_READ as uintb);
            self.op_set_input(newop, idx_con, 0)?;
            // annoteVn = newCodeRef(vn->getAddr()); annoteVn->setFlags(volatil);
            let annote = self.new_code_ref(&vaddr);
            self.vbank_mut()
                .get_mut(annote)
                .expect("replace_volatile: stale annote")
                .set_flags_pub(varnode_flags::volatil);
            self.op_set_input(newop, annote, 1)?;
            // opSetInput(readop, tmp, readop->getSlot(vn));
            let slot =
                self.obank().get(readop).expect("replace_volatile: stale readop").get_slot(vn);
            self.op_set_input(readop, tmp, slot)?;
            // opInsertBefore(newop, readop);
            self.op_insert_before(newop, readop);
            // if (vr_op->getDisplay() != 0) newop->setHoldOutput();
            if self.get_arch().volatile_read_holds_output() {
                self.obank_mut()
                    .get_mut(newop)
                    .expect("replace_volatile: stale newop")
                    .set_hold_output();
            }
        }
        // if (vn->isTypeLock()) newop->setAdditionalFlag(special_prop);
        if is_type_lock {
            self.obank_mut()
                .get_mut(newop)
                .expect("replace_volatile: stale newop")
                .set_additional_flag(crate::op::pcodeop_addlflags::special_prop);
        }
        Ok(true)
    }

    // -----------------------------------------------------------------------
    // Parameter-trial ancestor analysis (funcdata_varnode.cc:1802-2040) — the
    // S4 return-value recovery substrate (ActionReturnRecovery / ActionActiveParam).
    // -----------------------------------------------------------------------

    /// Test if a Varnode use at a CALL/CALLIND is a legitimate double-use of a
    /// parameter (C++ `Funcdata::checkCallDoubleUse`, funcdata_varnode.cc:1802).
    ///
    /// SEAM(W4): reaches `getCallSpecs` / `FuncCallSpecs::getActiveInput`, the
    /// call-spec subsystem that is not on `Funcdata` in the merged tree
    /// (`num_calls()==0` — no CALL ops in the IR), so this is never reachable on
    /// the recovery path; it conservatively reports `false` (not a double-use).
    fn check_call_double_use(
        &self,
        _opmatch: OpId,
        _op: OpId,
        _vn: VarnodeId,
        _fl: kuna_base::types::uint4,
        _trial: &crate::fspec::ParamTrial,
    ) -> bool {
        false
    }

    /// Test if the given Varnode seems to only be used by `opmatch` as a
    /// parameter-passing location (C++ `Funcdata::onlyOpUse`,
    /// funcdata_varnode.cc:1851).
    ///
    /// Walks every descendant of `invn` forward, classifying each use; a
    /// branch/load/store/non-matching-call/persistent-output use means the
    /// Varnode is *not* exclusively a parameter (returns `false`).  Uses the
    /// Varnode mark bit to bound cycles (set on entry, cleared on exit).
    fn only_op_use(
        &mut self,
        invn: VarnodeId,
        opmatch: OpId,
        trial: &crate::fspec::ParamTrial,
        main_flags: kuna_base::types::uint4,
    ) -> bool {
        use crate::expression::{traverse_flags, TraverseNode};
        use crate::seams::OpId as OId;
        let mut res = true;
        // varlist holds (vn, flags); invn marked to prevent infinite loops.
        let mut varlist: Vec<(VarnodeId, kuna_base::types::uint4)> = Vec::with_capacity(64);
        self.vbank_mut().get_mut(invn).expect("onlyOpUse: stale invn").set_mark();
        varlist.push((invn, main_flags));
        let active_output = self.get_active_output().is_some();
        let mut i = 0;
        while i < varlist.len() {
            let (vn, base_flags) = varlist[i];
            // Snapshot the descend list (we mutate marks while iterating).
            let descend: Vec<OId> = match self.vbank().get(vn) {
                Some(v) => v.descend_iter().collect(),
                None => Vec::new(),
            };
            for op in descend {
                let o = match self.obank().get(op) {
                    Some(o) => o,
                    None => continue,
                };
                let code = o.code();
                if op == opmatch {
                    // if (op->getIn(trial.getSlot())==vn) continue;
                    if o.get_in(trial.get_slot()) == Some(vn) {
                        continue;
                    }
                }
                let mut cur_flags = base_flags;
                match code {
                    OpCode::CPUI_BRANCH
                    | OpCode::CPUI_CBRANCH
                    | OpCode::CPUI_BRANCHIND
                    | OpCode::CPUI_LOAD
                    | OpCode::CPUI_STORE => {
                        res = false;
                    }
                    OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => {
                        if self.check_call_double_use(opmatch, op, vn, cur_flags, trial) {
                            continue;
                        }
                        res = false;
                    }
                    OpCode::CPUI_INDIRECT => {
                        cur_flags |= traverse_flags::indirectalt;
                    }
                    OpCode::CPUI_COPY => {
                        let out = o.get_out();
                        let out_internal = out
                            .and_then(|ov| self.vbank().get(ov))
                            .map(|ov| ov.get_space().get_type() == spacetype::IPTR_INTERNAL)
                            .unwrap_or(false);
                        let in0 = o.get_in(0);
                        let in0_incid = in0
                            .and_then(|iv| self.vbank().get(iv))
                            .map(|iv| iv.is_incidental_copy())
                            .unwrap_or(false);
                        if !out_internal && !o.is_incidental_copy() && !in0_incid {
                            cur_flags |= traverse_flags::actionalt;
                        }
                    }
                    OpCode::CPUI_RETURN => {
                        if self.obank().get(opmatch).map(|m| m.code()) == Some(OpCode::CPUI_RETURN) {
                            // Different RETURN at the same slot is not a use.
                            if o.get_in(trial.get_slot()) == Some(vn) {
                                continue;
                            }
                        } else if active_output
                            && o.get_in(0) != Some(vn)
                            && !TraverseNode::is_alternate_path_valid(
                                vn,
                                cur_flags,
                                self.vbank(),
                                self.obank(),
                            )
                        {
                            // Middle of analyzing returns: unless we hold the
                            // actual return value (in0), validate the alt path.
                            continue;
                        }
                        res = false;
                    }
                    OpCode::CPUI_MULTIEQUAL
                    | OpCode::CPUI_INT_SEXT
                    | OpCode::CPUI_INT_ZEXT
                    | OpCode::CPUI_CAST => {}
                    OpCode::CPUI_PIECE => {
                        if o.get_in(0) == Some(vn) {
                            if (cur_flags & traverse_flags::lsb_truncated) != 0 {
                                continue;
                            }
                            cur_flags |= traverse_flags::concat_high;
                        }
                    }
                    OpCode::CPUI_SUBPIECE => {
                        let in1_off = o
                            .get_in(1)
                            .and_then(|v| self.vbank().get(v))
                            .map(|v| v.get_offset())
                            .unwrap_or(0);
                        if in1_off != 0 && (cur_flags & traverse_flags::concat_high) == 0 {
                            cur_flags |= traverse_flags::lsb_truncated;
                        }
                    }
                    _ => {
                        cur_flags |= traverse_flags::actionalt;
                    }
                }
                if !res {
                    break;
                }
                let subvn = self.obank().get(op).and_then(|o| o.get_out());
                if let Some(subvn) = subvn {
                    if self.vbank().get(subvn).map(|v| v.is_persist()).unwrap_or(false) {
                        res = false;
                        break;
                    }
                    if !self.vbank().get(subvn).map(|v| v.is_mark()).unwrap_or(true) {
                        varlist.push((subvn, cur_flags));
                        self.vbank_mut().get_mut(subvn).expect("onlyOpUse: subvn").set_mark();
                    }
                }
            }
            if !res {
                break;
            }
            i += 1;
        }
        for (vn, _) in &varlist {
            if let Some(v) = self.vbank_mut().get_mut(*vn) {
                v.clear_mark();
            }
        }
        res
    }

    /// Test whether a Varnode's data-flow ancestry makes it a realistic
    /// parameter-passing location for the matching op (C++
    /// `Funcdata::ancestorOpUse`, funcdata_varnode.cc:1963).
    ///
    /// Recurses up through INDIRECT/MULTIEQUAL/COPY/PIECE/SUBPIECE definitions
    /// (bounded by `maxlevel`), and at the top ancestor defers to
    /// [`only_op_use`](Self::only_op_use).
    pub(crate) fn ancestor_op_use(
        &mut self,
        maxlevel: int4,
        invn: VarnodeId,
        op: OpId,
        trial: &mut crate::fspec::ParamTrial,
        offset: int4,
        main_flags: kuna_base::types::uint4,
    ) -> bool {
        use crate::expression::traverse_flags;
        if maxlevel == 0 {
            return false;
        }
        let written = self.vbank().get(invn).map(|v| v.is_written()).unwrap_or(false);
        if !written {
            let is_input = self.vbank().get(invn).map(|v| v.is_input()).unwrap_or(false);
            if !is_input {
                return false;
            }
            let is_typelock =
                self.vbank().get(invn).map(|v| v.is_type_lock()).unwrap_or(false);
            if !is_typelock {
                return false;
            }
            // typelocked input is as good as written.
            return self.only_op_use(invn, op, trial, main_flags);
        }
        let def = self.vbank().get(invn).and_then(|v| v.get_def());
        let def = match def {
            Some(d) => d,
            None => return self.only_op_use(invn, op, trial, main_flags),
        };
        let code = self.obank().get(def).map(|o| o.code()).unwrap_or(OpCode::CPUI_COPY);
        match code {
            OpCode::CPUI_INDIRECT => {
                if self.obank().get(def).map(|o| o.is_indirect_creation()).unwrap_or(false) {
                    return false;
                }
                let in0 = self.obank().get(def).and_then(|o| o.get_in(0));
                if let Some(in0) = in0 {
                    return self.ancestor_op_use(
                        maxlevel - 1,
                        in0,
                        op,
                        trial,
                        offset,
                        main_flags | traverse_flags::indirect,
                    );
                }
                false
            }
            OpCode::CPUI_MULTIEQUAL => {
                // Trim loops via the op mark bit.
                if self.obank().get(def).map(|o| o.is_mark()).unwrap_or(false) {
                    return false;
                }
                self.obank_mut().get_mut(def).expect("ancestorOpUse: def").set_mark();
                let ninput = self.obank().get(def).map(|o| o.num_input()).unwrap_or(0);
                for k in 0..ninput {
                    let ink = self.obank().get(def).and_then(|o| o.get_in(k));
                    if let Some(ink) = ink {
                        if self.ancestor_op_use(maxlevel - 1, ink, op, trial, offset, main_flags) {
                            self.obank_mut().get_mut(def).expect("ancestorOpUse: def").clear_mark();
                            return true;
                        }
                    }
                }
                self.obank_mut().get_mut(def).expect("ancestorOpUse: def").clear_mark();
                false
            }
            OpCode::CPUI_COPY => {
                let space_internal = self
                    .vbank()
                    .get(invn)
                    .map(|v| v.get_space().get_type() == spacetype::IPTR_INTERNAL)
                    .unwrap_or(false);
                let incidental = self.obank().get(def).map(|o| o.is_incidental_copy()).unwrap_or(false);
                let in0 = self.obank().get(def).and_then(|o| o.get_in(0));
                let in0_incid = in0
                    .and_then(|v| self.vbank().get(v))
                    .map(|v| v.is_incidental_copy())
                    .unwrap_or(false);
                if space_internal || incidental || in0_incid {
                    if let Some(in0) = in0 {
                        return self.ancestor_op_use(maxlevel - 1, in0, op, trial, offset, main_flags);
                    }
                }
                self.only_op_use(invn, op, trial, main_flags)
            }
            OpCode::CPUI_PIECE => {
                let in1 = self.obank().get(def).and_then(|o| o.get_in(1));
                let in1_size = in1.and_then(|v| self.vbank().get(v)).map(|v| v.get_size()).unwrap_or(0);
                if offset == 0 {
                    if let Some(in1) = in1 {
                        return self.ancestor_op_use(maxlevel - 1, in1, op, trial, 0, main_flags);
                    }
                    return false;
                }
                if offset == in1_size {
                    let in0 = self.obank().get(def).and_then(|o| o.get_in(0));
                    if let Some(in0) = in0 {
                        return self.ancestor_op_use(maxlevel - 1, in0, op, trial, 0, main_flags);
                    }
                }
                false
            }
            OpCode::CPUI_SUBPIECE => {
                let in0 = self.obank().get(def).and_then(|o| o.get_in(0));
                let new_off = self
                    .obank()
                    .get(def)
                    .and_then(|o| o.get_in(1))
                    .and_then(|v| self.vbank().get(v))
                    .map(|v| v.get_offset())
                    .unwrap_or(0) as int4;
                if new_off == 0 {
                    if let Some(in0) = in0 {
                        let in0_written = self.vbank().get(in0).map(|v| v.is_written()).unwrap_or(false);
                        if in0_written {
                            let remcode = self
                                .vbank()
                                .get(in0)
                                .and_then(|v| v.get_def())
                                .and_then(|d| self.obank().get(d))
                                .map(|o| o.code());
                            if remcode == Some(OpCode::CPUI_INT_REM)
                                || remcode == Some(OpCode::CPUI_INT_SREM)
                            {
                                trial.set_rem_formed();
                            }
                        }
                    }
                }
                let space_internal = self
                    .vbank()
                    .get(invn)
                    .map(|v| v.get_space().get_type() == spacetype::IPTR_INTERNAL)
                    .unwrap_or(false);
                let incidental = self.obank().get(def).map(|o| o.is_incidental_copy()).unwrap_or(false);
                let in0_incid = in0
                    .and_then(|v| self.vbank().get(v))
                    .map(|v| v.is_incidental_copy())
                    .unwrap_or(false);
                let overlap_match = in0
                    .map(|in0v| {
                        let (inv, in0vn) = (self.vbank().get(invn), self.vbank().get(in0v));
                        match (inv, in0vn) {
                            (Some(a), Some(b)) => a.overlap(b) == new_off,
                            _ => false,
                        }
                    })
                    .unwrap_or(false);
                if space_internal || incidental || in0_incid || overlap_match {
                    if let Some(in0) = in0 {
                        return self.ancestor_op_use(
                            maxlevel - 1,
                            in0,
                            op,
                            trial,
                            offset + new_off,
                            main_flags,
                        );
                    }
                }
                self.only_op_use(invn, op, trial, main_flags)
            }
            OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => false,
            _ => self.only_op_use(invn, op, trial, main_flags),
        }
    }
}

// ===========================================================================
// AncestorRealistic (funcdata.hh:672-741, funcdata_varnode.cc:2043-2283)
// ===========================================================================

/// Per-node state in the depth-first ancestor traversal (C++
/// `AncestorRealistic::State`).
#[derive(Clone, Copy)]
struct AncestorState {
    /// Operation along the path to the Varnode (C++ `op`).
    op: OpId,
    /// `vn = op->getIn(slot)` (C++ `slot`).
    slot: int4,
    /// Boolean properties of the node (C++ `flags`).
    flags: kuna_base::types::uint4,
    /// Offset of the (eventual) trial value within a larger register (C++ `offset`).
    offset: int4,
}

mod ancestor_state_flags {
    use kuna_base::types::uint4;
    /// Solid movement into the Varnode on at least one MULTIEQUAL path (slot 0).
    pub const SEEN_SOLID0: uint4 = 1;
    /// Solid movement into anything other than slot 0.
    pub const SEEN_SOLID1: uint4 = 2;
    /// The Varnode is killed by a call on at least one MULTIEQUAL path.
    pub const SEEN_KILL: uint4 = 4;
}

impl AncestorState {
    fn new(op: OpId, slot: int4) -> AncestorState {
        AncestorState { op, slot, flags: 0, offset: 0 }
    }
    /// Pull back through a SUBPIECE, accumulating the offset (C++
    /// `State(PcodeOp *o,const State &oldState)`).  `sub_off` is the SUBPIECE's
    /// in(1) constant offset.
    fn from_subpiece(op: OpId, old: &AncestorState, sub_off: int4) -> AncestorState {
        AncestorState { op, slot: 0, flags: 0, offset: old.offset + sub_off }
    }
    fn get_solid_slot(&self) -> int4 {
        if (self.flags & ancestor_state_flags::SEEN_SOLID0) != 0 {
            0
        } else {
            1
        }
    }
    fn mark_solid(&mut self, s: int4) {
        self.flags |= if s == 0 {
            ancestor_state_flags::SEEN_SOLID0
        } else {
            ancestor_state_flags::SEEN_SOLID1
        };
    }
    fn mark_kill(&mut self) {
        self.flags |= ancestor_state_flags::SEEN_KILL;
    }
    fn seen_solid(&self) -> bool {
        (self.flags & (ancestor_state_flags::SEEN_SOLID0 | ancestor_state_flags::SEEN_SOLID1)) != 0
    }
    fn seen_kill(&self) -> bool {
        (self.flags & ancestor_state_flags::SEEN_KILL) != 0
    }
}

/// Traversal-command enum (C++ `AncestorRealistic` anonymous enum).
#[derive(Clone, Copy, PartialEq, Eq)]
enum AncestorCmd {
    EnterNode,
    PopSuccess,
    PopSolid,
    PopFail,
    PopFailKill,
}

/// Depth-first ancestor-realism analyzer for a parameter trial (C++
/// `AncestorRealistic`, funcdata.hh:672).  Decides whether a Varnode read at a
/// RETURN/CALL has realistic data-flow ancestry for a parameter-passing
/// location.  Operates on `&mut Funcdata` (it sets/clears the Varnode mark bit
/// to trim cycles, exactly as the C++ `mark`).
pub(crate) struct AncestorRealistic {
    state_stack: Vec<AncestorState>,
    marked_vn: Vec<VarnodeId>,
    multi_depth: int4,
    allow_failing_path: bool,
    /// The current trial's size (C++ `trial->getSize()`), read by the PIECE
    /// truncation logic in `enterNode`.
    trial_size: int4,
    /// Accumulated trial mutations (the C++ mutates `trial` in place during the
    /// walk; collected here and applied by the caller after `execute`).
    set_ind_create_formed: bool,
    set_cond_exe_effect: bool,
}

impl AncestorRealistic {
    pub(crate) fn new() -> AncestorRealistic {
        AncestorRealistic {
            state_stack: Vec::new(),
            marked_vn: Vec::new(),
            multi_depth: 0,
            allow_failing_path: false,
            trial_size: 0,
            set_ind_create_formed: false,
            set_cond_exe_effect: false,
        }
    }

    fn mark(&mut self, fd: &mut Funcdata, vn: VarnodeId) {
        self.marked_vn.push(vn);
        if let Some(v) = fd.vbank_mut().get_mut(vn) {
            v.set_mark();
        }
    }

    /// C++ `AncestorRealistic::checkConditionalExe` — true if two input flows,
    /// one of which is a normal solid flow.
    fn check_conditional_exe(&self, fd: &Funcdata, state: &AncestorState) -> bool {
        let bl = match fd.obank().get(state.op).and_then(|o| o.get_parent()) {
            Some(b) => b,
            None => return false,
        };
        let blref = fd.bblocks_ref().block(bl);
        if blref.size_in() != 2 {
            return false;
        }
        let solid = blref.get_in(state.get_solid_slot());
        let solidref = fd.bblocks_ref().block(solid);
        if solidref.size_out() != 1 {
            return false;
        }
        true
    }

    /// C++ `AncestorRealistic::enterNode` — traverse into a new Varnode.
    fn enter_node(&mut self, fd: &mut Funcdata) -> AncestorCmd {
        let state = *self.state_stack.last().expect("enterNode: empty stack");
        let state_vn = match fd.obank().get(state.op).and_then(|o| o.get_in(state.slot)) {
            Some(v) => v,
            None => return AncestorCmd::PopSuccess,
        };
        if fd.vbank().get(state_vn).map(|v| v.is_mark()).unwrap_or(false) {
            return AncestorCmd::PopSuccess;
        }
        let written = fd.vbank().get(state_vn).map(|v| v.is_written()).unwrap_or(false);
        if !written {
            let v = fd.vbank().get(state_vn);
            if v.map(|v| v.is_input()).unwrap_or(false) {
                if fd.vbank().get(state_vn).map(|v| v.is_unaffected()).unwrap_or(false) {
                    return AncestorCmd::PopFail;
                }
                if fd.vbank().get(state_vn).map(|v| v.is_persist()).unwrap_or(false) {
                    return AncestorCmd::PopSuccess;
                }
                if !fd.vbank().get(state_vn).map(|v| v.is_direct_write()).unwrap_or(false) {
                    return AncestorCmd::PopFail;
                }
            }
            return AncestorCmd::PopSuccess;
        }
        self.mark(fd, state_vn);
        let op = fd.vbank().get(state_vn).and_then(|v| v.get_def()).expect("enterNode: def");
        let code = fd.obank().get(op).map(|o| o.code()).unwrap_or(OpCode::CPUI_COPY);
        match code {
            OpCode::CPUI_INDIRECT => {
                if fd.obank().get(op).map(|o| o.is_indirect_creation()).unwrap_or(false) {
                    self.set_ind_create_formed = true;
                    let in0 = fd.obank().get(op).and_then(|o| o.get_in(0));
                    let in0_indzero = in0
                        .and_then(|v| fd.vbank().get(v))
                        .map(|v| v.is_indirect_zero())
                        .unwrap_or(false);
                    if in0_indzero {
                        return AncestorCmd::PopFailKill;
                    }
                    return AncestorCmd::PopSuccess;
                }
                if !fd.obank().get(op).map(|o| o.is_indirect_store()).unwrap_or(false) {
                    let out_retaddr = fd
                        .obank()
                        .get(op)
                        .and_then(|o| o.get_out())
                        .and_then(|v| fd.vbank().get(v))
                        .map(|v| v.is_return_address())
                        .unwrap_or(false);
                    if out_retaddr {
                        return AncestorCmd::PopFail;
                    }
                    // trial->isKilledByCall(): the caller passes the trial; the
                    // recovery-path register trials are not killedbycall once a
                    // model output entry matches.  Reflected via the trial flag
                    // the caller seeds; conservatively treat as not-killed here
                    // (SEAM: the killedbycall path needs the trial reference).
                }
                self.state_stack.push(AncestorState::new(op, 0));
                AncestorCmd::EnterNode
            }
            OpCode::CPUI_SUBPIECE => {
                let out_internal = fd
                    .obank()
                    .get(op)
                    .and_then(|o| o.get_out())
                    .and_then(|v| fd.vbank().get(v))
                    .map(|v| v.get_space().get_type() == spacetype::IPTR_INTERNAL)
                    .unwrap_or(false);
                let incid = fd.obank().get(op).map(|o| o.is_incidental_copy()).unwrap_or(false);
                let in0 = fd.obank().get(op).and_then(|o| o.get_in(0));
                let in0_incid =
                    in0.and_then(|v| fd.vbank().get(v)).map(|v| v.is_incidental_copy()).unwrap_or(false);
                let in1_off = fd
                    .obank()
                    .get(op)
                    .and_then(|o| o.get_in(1))
                    .and_then(|v| fd.vbank().get(v))
                    .map(|v| v.get_offset())
                    .unwrap_or(0) as int4;
                let overlap_match = {
                    let out = fd.obank().get(op).and_then(|o| o.get_out());
                    match (out.and_then(|v| fd.vbank().get(v)), in0.and_then(|v| fd.vbank().get(v))) {
                        (Some(a), Some(b)) => a.overlap(b) == in1_off,
                        _ => false,
                    }
                };
                if out_internal || incid || in0_incid || overlap_match {
                    let st = AncestorState::from_subpiece(op, &state, in1_off);
                    self.state_stack.push(st);
                    return AncestorCmd::EnterNode;
                }
                // Minimal traversal to rule out unaffected/invalid inputs.
                let mut curop = op;
                while let Some(vn) = fd.obank().get(curop).and_then(|o| o.get_in(0)) {
                    let v_marked = fd.vbank().get(vn).map(|v| v.is_mark()).unwrap_or(false);
                    let v_input = fd.vbank().get(vn).map(|v| v.is_input()).unwrap_or(false);
                    if !v_marked && v_input {
                        let unaff = fd.vbank().get(vn).map(|v| v.is_unaffected()).unwrap_or(false);
                        let dw = fd.vbank().get(vn).map(|v| v.is_direct_write()).unwrap_or(false);
                        if unaff || !dw {
                            return AncestorCmd::PopFail;
                        }
                    }
                    match fd.vbank().get(vn).and_then(|v| v.get_def()) {
                        Some(d) => {
                            let c = fd.obank().get(d).map(|o| o.code());
                            if c == Some(OpCode::CPUI_COPY) || c == Some(OpCode::CPUI_SUBPIECE) {
                                curop = d;
                            } else {
                                break;
                            }
                        }
                        None => break,
                    }
                }
                AncestorCmd::PopSolid
            }
            OpCode::CPUI_COPY => {
                let out_internal = fd
                    .obank()
                    .get(op)
                    .and_then(|o| o.get_out())
                    .and_then(|v| fd.vbank().get(v))
                    .map(|v| v.get_space().get_type() == spacetype::IPTR_INTERNAL)
                    .unwrap_or(false);
                let incid = fd.obank().get(op).map(|o| o.is_incidental_copy()).unwrap_or(false);
                let in0 = fd.obank().get(op).and_then(|o| o.get_in(0));
                let in0_incid =
                    in0.and_then(|v| fd.vbank().get(v)).map(|v| v.is_incidental_copy()).unwrap_or(false);
                let same_addr = {
                    let out = fd.obank().get(op).and_then(|o| o.get_out());
                    match (out.and_then(|v| fd.vbank().get(v)), in0.and_then(|v| fd.vbank().get(v))) {
                        (Some(a), Some(b)) => a.get_addr() == b.get_addr(),
                        _ => false,
                    }
                };
                if out_internal || incid || in0_incid || same_addr {
                    self.state_stack.push(AncestorState::new(op, 0));
                    return AncestorCmd::EnterNode;
                }
                // Minimal traversal for other COPYs.
                let mut curvn = match in0 {
                    Some(v) => v,
                    None => return AncestorCmd::PopSolid,
                };
                loop {
                    let v_marked = fd.vbank().get(curvn).map(|v| v.is_mark()).unwrap_or(false);
                    let v_input = fd.vbank().get(curvn).map(|v| v.is_input()).unwrap_or(false);
                    if !v_marked && v_input && !fd.vbank().get(curvn).map(|v| v.is_direct_write()).unwrap_or(false) {
                        return AncestorCmd::PopFail;
                    }
                    let curdef = fd.vbank().get(curvn).and_then(|v| v.get_def());
                    if let Some(d) = curdef {
                        if fd.obank().get(d).map(|o| o.is_store_unmapped()).unwrap_or(false) {
                            return AncestorCmd::PopFail;
                        }
                    }
                    let op2 = match curdef {
                        Some(d) => d,
                        None => break,
                    };
                    let opc = fd.obank().get(op2).map(|o| o.code());
                    if opc == Some(OpCode::CPUI_COPY) || opc == Some(OpCode::CPUI_SUBPIECE) {
                        curvn = fd.obank().get(op2).and_then(|o| o.get_in(0)).unwrap_or(curvn);
                    } else if opc == Some(OpCode::CPUI_PIECE) {
                        curvn = fd.obank().get(op2).and_then(|o| o.get_in(1)).unwrap_or(curvn);
                    } else {
                        break;
                    }
                }
                AncestorCmd::PopSolid
            }
            OpCode::CPUI_MULTIEQUAL => {
                self.multi_depth += 1;
                self.state_stack.push(AncestorState::new(op, 0));
                AncestorCmd::EnterNode
            }
            OpCode::CPUI_PIECE => {
                let state_vn_size = fd.vbank().get(state_vn).map(|v| v.get_size()).unwrap_or(0);
                let trial_size = self.trial_size;
                if state_vn_size > trial_size {
                    let in1_size = fd
                        .obank()
                        .get(op)
                        .and_then(|o| o.get_in(1))
                        .and_then(|v| fd.vbank().get(v))
                        .map(|v| v.get_size())
                        .unwrap_or(0);
                    let in0_size = fd
                        .obank()
                        .get(op)
                        .and_then(|o| o.get_in(0))
                        .and_then(|v| fd.vbank().get(v))
                        .map(|v| v.get_size())
                        .unwrap_or(0);
                    if state.offset == 0 && in1_size <= trial_size {
                        self.state_stack.push(AncestorState::new(op, 1));
                        return AncestorCmd::EnterNode;
                    } else if state.offset == in1_size && in0_size <= trial_size {
                        self.state_stack.push(AncestorState::new(op, 0));
                        return AncestorCmd::EnterNode;
                    }
                    let spbase = fd
                        .vbank()
                        .get(state_vn)
                        .map(|v| v.get_space().get_type() == spacetype::IPTR_SPACEBASE)
                        .unwrap_or(false);
                    if !spbase {
                        return AncestorCmd::PopFail;
                    }
                }
                AncestorCmd::PopSolid
            }
            _ => AncestorCmd::PopSolid,
        }
    }

    /// C++ `AncestorRealistic::uponPop` — backtrack into a previously visited
    /// node.
    fn upon_pop(&mut self, fd: &Funcdata, mut pop_command: AncestorCmd) -> AncestorCmd {
        let top = self.state_stack.len() - 1;
        let is_multi = fd.obank().get(self.state_stack[top].op).map(|o| o.code())
            == Some(OpCode::CPUI_MULTIEQUAL);
        if is_multi {
            let prev = top - 1;
            if pop_command == AncestorCmd::PopFail {
                self.multi_depth -= 1;
                self.state_stack.pop();
                return pop_command;
            } else if pop_command == AncestorCmd::PopSolid
                && self.multi_depth == 1
                && fd.obank().get(self.state_stack[top].op).map(|o| o.num_input()).unwrap_or(0) == 2
            {
                let slot = self.state_stack[top].slot;
                self.state_stack[prev].mark_solid(slot);
            } else if pop_command == AncestorCmd::PopFailKill {
                self.state_stack[prev].mark_kill();
            }
            self.state_stack[top].slot += 1;
            let ninput = fd.obank().get(self.state_stack[top].op).map(|o| o.num_input()).unwrap_or(0);
            if self.state_stack[top].slot == ninput {
                if self.state_stack[prev].seen_solid() {
                    pop_command = AncestorCmd::PopSuccess;
                    if self.state_stack[prev].seen_kill() {
                        if self.allow_failing_path {
                            let st = self.state_stack[top];
                            if !self.check_conditional_exe(fd, &st) {
                                pop_command = AncestorCmd::PopFail;
                            } else {
                                self.set_cond_exe_effect = true;
                            }
                        } else {
                            pop_command = AncestorCmd::PopFail;
                        }
                    }
                } else if self.state_stack[prev].seen_kill() {
                    pop_command = AncestorCmd::PopFailKill;
                } else {
                    pop_command = AncestorCmd::PopSuccess;
                }
                self.multi_depth -= 1;
                self.state_stack.pop();
                return pop_command;
            }
            AncestorCmd::EnterNode
        } else {
            self.state_stack.pop();
            pop_command
        }
    }

    /// Perform a full ancestor check on a given parameter trial (C++
    /// `AncestorRealistic::execute`, funcdata_varnode.cc:2240).
    ///
    /// `trial_size`/`trial_has_cond_exe` carry the two trial properties the walk
    /// reads; the walk may set the trial's `ind_create_formed`/`cond_exe_effect`/
    /// `ancestor_realistic`/`ancestor_solid` flags — those are applied to the
    /// caller's `trial` after `execute` returns (see [`Self::apply_trial`]).
    pub(crate) fn execute(
        &mut self,
        fd: &mut Funcdata,
        op: OpId,
        slot: int4,
        trial_size: int4,
        trial_has_cond_exe: bool,
        allow_fail: bool,
    ) -> (bool, bool) {
        self.allow_failing_path = allow_fail;
        self.trial_size = trial_size;
        self.marked_vn.clear();
        self.state_stack.clear();
        self.multi_depth = 0;
        self.set_ind_create_formed = false;
        self.set_cond_exe_effect = false;
        // If the parameter itself is an input, we don't consider this realistic
        // (unless we are re-testing a conditional-execution trial).
        let in_slot = fd.obank().get(op).and_then(|o| o.get_in(slot));
        let is_input = in_slot.and_then(|v| fd.vbank().get(v)).map(|v| v.is_input()).unwrap_or(false);
        if is_input && !trial_has_cond_exe {
            return (false, false);
        }
        let mut command = AncestorCmd::EnterNode;
        self.state_stack.push(AncestorState::new(op, slot));
        while !self.state_stack.is_empty() {
            command = match command {
                AncestorCmd::EnterNode => self.enter_node(fd),
                other => self.upon_pop(fd, other),
            };
        }
        // Clear marks left along the way.
        let marked: Vec<VarnodeId> = std::mem::take(&mut self.marked_vn);
        for vn in marked {
            if let Some(v) = fd.vbank_mut().get_mut(vn) {
                v.clear_mark();
            }
        }
        // (realistic, solid)
        match command {
            AncestorCmd::PopSuccess => (true, false),
            AncestorCmd::PopSolid => (true, true),
            _ => (false, false),
        }
    }

    /// Apply the trial mutations accumulated during [`Self::execute`] to the
    /// caller's `ParamTrial` (the C++ mutates `trial` in place; the kuna port
    /// collects the flags and applies them here so the walk can borrow `&mut
    /// Funcdata` cleanly).  `realistic`/`solid` are the `execute` result.
    pub(crate) fn apply_trial(
        &self,
        trial: &mut crate::fspec::ParamTrial,
        realistic: bool,
        solid: bool,
    ) {
        if self.set_ind_create_formed {
            trial.set_ind_create_formed();
        }
        if self.set_cond_exe_effect {
            trial.set_cond_exe_effect();
        }
        if realistic {
            trial.set_ancestor_realistic();
        }
        if solid {
            trial.set_ancestor_solid();
        }
    }
}

/// Encode an [`OpId`] as the constant offset of an \e iop-space Address
/// (C++ `(uintb)(uintp)op` in `newVarnodeIop`).
///
/// The C++ stores the raw `PcodeOp *`; the Rust IR has no stable pointer, so the
/// slotmap key's bit pattern is used.  `slotmap` keys expose their packed `u64`
/// form via `KeyData::as_ffi`; the round-trip is opaque to the IR (the iop-space
/// value is only ever decoded back to identify the same op).
fn op_iop_encode(op: OpId) -> uintb {
    use slotmap::Key;
    op.data().as_ffi()
}

/// Decode an iop-space constant offset back into the [`OpId`] it was encoded from
/// (C++ `PcodeOp::getOpFromConst(addr)` — `(PcodeOp *)(uintp)addr.getOffset()`).
///
/// Inverse of [`op_iop_encode`]: the C++ stores the raw `PcodeOp *` in the iop
/// constant and re-casts it back; the Rust IR has no stable pointer, so the
/// slotmap key bit pattern round-trips through `KeyData::from_ffi`.  This is the
/// only way the iop constant is consumed: to re-identify the *same* op.  Used by
/// [`TransformVar::create_replacement`](crate::transform) for the `constant_iop`
/// placeholder.
pub(crate) fn op_iop_decode(val: uintb) -> OpId {
    OpId::from(slotmap::KeyData::from_ffi(val))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;

    use kuna_base::address::Address;
    use kuna_base::space::{
        addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, FspecSpace,
        IopSpace, UniqueSpace,
    };
    use kuna_num::opcodes::OpCode;

    use crate::dtype::{type_metatype, Datatype};
    use crate::seams::{Architecture, TypeOp};
    use crate::varnode::{DefOpInfo, VarnodeBank};

    /// Build an AddrSpaceManager with constant/unique/iop/fspec/ram spaces.
    fn build_manager() -> AddrSpaceManager {
        let mut m = AddrSpaceManager::new();
        m.insert_space(Rc::new(ConstantSpace::new())).unwrap();
        m.insert_space(Rc::new(UniqueSpace::new(1, 0, false))).unwrap();
        m.insert_space(Rc::new(IopSpace::new(2))).unwrap();
        m.insert_space(Rc::new(FspecSpace::new(3))).unwrap();
        m.insert_space(Rc::new(AddrSpace::new(
            spacetype::IPTR_PROCESSOR,
            "ram",
            false,
            8,
            1,
            4,
            addrspace_flags::hasphysical,
            1,
            1,
        )))
        .unwrap();
        m
    }

    fn build_fd() -> Funcdata {
        let manage = build_manager();
        let glb = Rc::new(Architecture::new(manage));
        let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
        let addr = Address::new(ram, 0x1000);
        Funcdata::new("func", "func", glb, addr, 0x10000000, 0x40).unwrap()
    }

    fn ram(fd: &Funcdata) -> Rc<AddrSpace> {
        Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap())
    }

    fn unk(size: int4) -> Rc<Datatype> {
        Rc::new(Datatype::new(size, type_metatype::TYPE_UNKNOWN))
    }

    /// A no-op `replace_reads` callback (no equivalent varnode is expected in the
    /// test scaffolding, which builds varnodes at distinct addresses).
    fn no_replace() -> impl FnMut(&mut VarnodeBank, VarnodeId, VarnodeId) -> KunaResult<()> {
        |_: &mut VarnodeBank, _: VarnodeId, _: VarnodeId| -> KunaResult<()> { Ok(()) }
    }

    // --- creation factories: bank-state outcomes -------------------------

    #[test]
    fn new_constant_creates_constant_in_const_space() {
        let mut fd = build_fd();
        let vn = fd.new_constant(4, 0x1234);
        let v = fd.vbank().get(vn).unwrap();
        assert!(v.is_constant());
        assert_eq!(v.get_offset(), 0x1234);
        assert_eq!(v.get_size(), 4);
        assert_eq!(fd.num_varnodes(), 1);
    }

    #[test]
    fn new_unique_advances_uniq_offset_and_create_index() {
        let mut fd = build_fd();
        let a = fd.new_unique(4, None);
        let b = fd.new_unique(8, None);
        // create_index is a strict one-up counter shared by all create paths.
        assert_eq!(fd.vbank().get(a).unwrap().get_create_index(), 0);
        assert_eq!(fd.vbank().get(b).unwrap().get_create_index(), 1);
        // unique offsets advance by size; both are free temporaries.
        let oa = fd.vbank().get(a).unwrap().get_offset();
        let ob = fd.vbank().get(b).unwrap().get_offset();
        assert_eq!(ob, oa + 4);
        assert!(fd.vbank().get(a).unwrap().is_free());
    }

    #[test]
    fn new_varnode_creates_free_in_given_space() {
        let mut fd = build_fd();
        let r = ram(&fd);
        let m = Address::new(r, 0x80);
        let vn = fd.new_varnode(4, &m, None);
        let v = fd.vbank().get(vn).unwrap();
        assert!(v.is_free());
        assert_eq!(v.get_addr().get_offset(), 0x80);
        assert_eq!(v.get_size(), 4);
    }

    #[test]
    fn new_varnode_space_off_matches_new_varnode() {
        let mut fd = build_fd();
        let r = ram(&fd);
        let vn = fd.new_varnode_space_off(2, Rc::clone(&r), 0x200);
        let v = fd.vbank().get(vn).unwrap();
        assert_eq!(v.get_size(), 2);
        assert_eq!(v.get_offset(), 0x200);
        assert!(v.is_free());
    }

    #[test]
    fn new_varnode_iop_encodes_op_in_iop_space() {
        let mut fd = build_fd();
        let pc = Address::new(ram(&fd), 0x1000);
        let op = fd.obank_mut().create_at(1, pc);
        let vn = fd.new_varnode_iop(op);
        let v = fd.vbank().get(vn).unwrap();
        let iop = fd.get_arch().manage().get_iop_space().unwrap();
        assert!(Rc::ptr_eq(v.get_space(), iop));
        // The encoded offset round-trips to the same key bit-pattern.
        assert_eq!(v.get_offset(), op_iop_encode(op));
    }

    #[test]
    fn new_varnode_space_creates_const_from_space() {
        let mut fd = build_fd();
        let r = ram(&fd);
        let vn = fd.new_varnode_space(&r);
        let v = fd.vbank().get(vn).unwrap();
        // createConstFromSpace yields a constant-space varnode.
        assert!(v.is_constant());
    }

    #[test]
    fn new_varnode_call_specs_in_fspec_space() {
        let mut fd = build_fd();
        let vn = fd.new_varnode_call_specs(0xdead);
        let v = fd.vbank().get(vn).unwrap();
        let fspec = fd.get_arch().manage().get_fspec_space().unwrap();
        assert!(Rc::ptr_eq(v.get_space(), fspec));
        assert_eq!(v.get_offset(), 0xdead);
    }

    // --- deletion / teardown ---------------------------------------------

    #[test]
    fn delete_varnode_removes_free() {
        let mut fd = build_fd();
        let r = ram(&fd);
        let vn = fd.new_varnode(4, &Address::new(r, 0x40), None);
        assert_eq!(fd.num_varnodes(), 1);
        fd.delete_varnode(vn).unwrap();
        assert_eq!(fd.num_varnodes(), 0);
    }

    #[test]
    fn destroy_varnode_unhooks_readers() {
        let mut fd = build_fd();
        let r = ram(&fd);
        // Free varnode read by one op.
        let vn = fd.new_varnode(4, &Address::new(Rc::clone(&r), 0x40), None);
        let pc = Address::new(Rc::clone(&r), 0x1000);
        let op = fd.obank_mut().create_at(2, pc);
        fd.obank_mut().change_opcode(op, TypeOp::new(OpCode::CPUI_COPY, 0, "COPY"));
        // Wire vn as input slot 0 of op via the descend list + op input.
        fd.vbank_mut().add_descend(vn, op).unwrap();
        fd.obank_mut().get_mut(op).unwrap().set_input(Some(vn), 0);
        // destroyVarnode: op's slot reading vn becomes null, vn is freed.
        fd.destroy_varnode(vn).unwrap();
        assert_eq!(fd.obank().get(op).unwrap().get_in(0), None);
        assert!(fd.vbank().get(vn).is_none());
    }

    // --- finders: boundary cases -----------------------------------------

    /// Make an input varnode at (addr,size) by creating a free one and flipping
    /// it to input directly in the bank (bypasses the xref split-borrow, which
    /// `setInputVarnode` needs and is the seam owner's — see module doc).
    fn make_input(fd: &mut Funcdata, off: u64, size: int4) -> VarnodeId {
        let r = ram(fd);
        let vn = fd.new_varnode(size, &Address::new(r, off), None);
        fd.vbank_mut().set_input(vn, &mut no_replace()).unwrap()
    }

    #[test]
    fn find_varnode_input_exact_match() {
        let mut fd = build_fd();
        let r = ram(&fd);
        let inp = make_input(&mut fd, 0x100, 4);
        let got = fd.find_varnode_input(4, &Address::new(Rc::clone(&r), 0x100));
        assert_eq!(got, Some(inp));
        // Wrong size / wrong address miss.
        assert_eq!(fd.find_varnode_input(8, &Address::new(Rc::clone(&r), 0x100)), None);
        assert_eq!(fd.find_varnode_input(4, &Address::new(r, 0x104)), None);
    }

    #[test]
    fn find_covered_input_returns_contained() {
        let mut fd = build_fd();
        let r = ram(&fd);
        // input at 0x100 size 2, inside the range [0x100, 0x108)
        let inp = make_input(&mut fd, 0x100, 2);
        let got = fd.find_covered_input(8, &Address::new(Rc::clone(&r), 0x100)).unwrap();
        assert_eq!(got, Some(inp));
        // A range that does not cover it (starts after) finds nothing.
        let got2 = fd.find_covered_input(2, &Address::new(r, 0x200)).unwrap();
        assert_eq!(got2, None);
    }

    #[test]
    fn find_covering_input_returns_container() {
        let mut fd = build_fd();
        let r = ram(&fd);
        // input at 0x100 size 8 covers the 4-byte range at 0x102
        let inp = make_input(&mut fd, 0x100, 8);
        // The C++ `findCoveringInput` lower_bound-then-step-back idiom requires a
        // def-tree element *after* the queried range to step back from (in a real
        // function the written/free varnodes following the inputs); register a
        // higher-address input so the probe lands on a real element, exactly as
        // the canonical `varnode::tests::input_finders` does.
        let _hi = make_input(&mut fd, 0x400, 2);
        let got = fd.find_covering_input(4, &Address::new(Rc::clone(&r), 0x102)).unwrap();
        assert_eq!(got, Some(inp));
        // A range outside is not covered (the `_hi` input does not contain it).
        let got2 = fd.find_covering_input(4, &Address::new(r, 0x200)).unwrap();
        assert_eq!(got2, None);
    }

    #[test]
    fn has_input_intersection_detects_overlap() {
        let mut fd = build_fd();
        let r = ram(&fd);
        let _inp = make_input(&mut fd, 0x100, 4);
        assert!(fd.has_input_intersection(2, &Address::new(Rc::clone(&r), 0x102)).unwrap());
        assert!(!fd.has_input_intersection(2, &Address::new(r, 0x200)).unwrap());
    }

    #[test]
    fn find_input_overlap_pre_check() {
        let mut fd = build_fd();
        let r = ram(&fd);
        let inp = make_input(&mut fd, 0x100, 4);
        // A fresh free candidate at the same (addr,size) overlaps an identical
        // input -> returns that input.
        let cand = fd.new_varnode(4, &Address::new(Rc::clone(&r), 0x100), None);
        assert_eq!(fd.find_input_overlap(cand).unwrap(), Some(inp));
        // A non-overlapping candidate -> None.
        let cand2 = fd.new_varnode(4, &Address::new(Rc::clone(&r), 0x200), None);
        assert_eq!(fd.find_input_overlap(cand2).unwrap(), None);
        // An overlapping-but-not-identical candidate -> error.
        let cand3 = fd.new_varnode(2, &Address::new(r, 0x101), None);
        assert!(fd.find_input_overlap(cand3).is_err());
    }

    // --- totalReplace def-use rewiring -----------------------------------

    /// Make a written varnode (output of a fresh op) so it can have multiple
    /// descendants (free non-spacebase varnodes are limited to one).
    fn make_written(fd: &mut Funcdata, off: u64, size: int4) -> (VarnodeId, OpId) {
        let r = ram(fd);
        let pc = Address::new(Rc::clone(&r), off);
        let defop = fd.obank_mut().create_at(1, pc);
        fd.obank_mut().change_opcode(defop, TypeOp::new(OpCode::CPUI_COPY, 0, "COPY"));
        let vn = fd.new_varnode(size, &Address::new(r, off), Some(unk(size)));
        // Promote vn to written (output of defop) via the bank set_def path.
        let seq = fd.obank().get(defop).unwrap().get_seq_num().clone();
        let info = DefOpInfo { id: defop, seqnum: seq };
        let vn = fd.vbank_mut().set_def(vn, info, &mut no_replace()).unwrap();
        fd.obank_mut().get_mut(defop).unwrap().set_output(Some(vn));
        (vn, defop)
    }

    #[test]
    fn total_replace_rewires_all_reads() {
        let mut fd = build_fd();
        let r = ram(&fd);
        let (vn, _d1) = make_written(&mut fd, 0x10, 4);
        let (newvn, _d2) = make_written(&mut fd, 0x20, 4);
        // Two reader ops reading vn in slot 0.
        let mut readers = Vec::new();
        for i in 0..2 {
            let pc = Address::new(Rc::clone(&r), 0x1000 + i * 4);
            let op = fd.obank_mut().create_at(1, pc);
            fd.obank_mut().change_opcode(op, TypeOp::new(OpCode::CPUI_COPY, 0, "COPY"));
            fd.vbank_mut().add_descend(vn, op).unwrap();
            fd.obank_mut().get_mut(op).unwrap().set_input(Some(vn), 0);
            readers.push(op);
        }
        assert_eq!(fd.vbank().get(vn).unwrap().num_descend(), 2);
        assert_eq!(fd.vbank().get(newvn).unwrap().num_descend(), 0);

        fd.total_replace(vn, newvn).unwrap();

        // All readers now read newvn; vn has no descendants left.
        for &op in &readers {
            assert_eq!(fd.obank().get(op).unwrap().get_in(0), Some(newvn));
        }
        assert_eq!(fd.vbank().get(vn).unwrap().num_descend(), 0);
        assert_eq!(fd.vbank().get(newvn).unwrap().num_descend(), 2);
    }

    #[test]
    fn total_replace_constant_clones_per_read() {
        // A constant being placed by op_set_input where it already has a
        // descendant must be cloned (constants get one descendant unless
        // spacebase).
        let mut fd = build_fd();
        let r = ram(&fd);
        let (src, _d) = make_written(&mut fd, 0x10, 4);
        // Reader op reading src.
        let pc = Address::new(Rc::clone(&r), 0x1000);
        let op = fd.obank_mut().create_at(1, pc);
        fd.obank_mut().change_opcode(op, TypeOp::new(OpCode::CPUI_COPY, 0, "COPY"));
        fd.vbank_mut().add_descend(src, op).unwrap();
        fd.obank_mut().get_mut(op).unwrap().set_input(Some(src), 0);

        // A constant already read once (so it has a descendant).
        let c = fd.new_constant(4, 7);
        let pc2 = Address::new(r, 0x1004);
        let op2 = fd.obank_mut().create_at(1, pc2);
        fd.obank_mut().change_opcode(op2, TypeOp::new(OpCode::CPUI_COPY, 0, "COPY"));
        fd.vbank_mut().add_descend(c, op2).unwrap();
        fd.obank_mut().get_mut(op2).unwrap().set_input(Some(c), 0);

        // Replace all reads of src with the (already-used) constant c.  op's slot
        // must get a *clone* of c (a distinct constant varnode), not c itself.
        fd.total_replace(src, c).unwrap();
        let got = fd.obank().get(op).unwrap().get_in(0).unwrap();
        assert_ne!(got, c, "constant must be cloned for the second reader");
        assert!(fd.vbank().get(got).unwrap().is_constant());
        assert_eq!(fd.vbank().get(got).unwrap().get_offset(), 7);
    }

    #[test]
    fn lone_descend_and_snapshot() {
        let mut fd = build_fd();
        let r = ram(&fd);
        let (vn, _d) = make_written(&mut fd, 0x10, 4);
        assert_eq!(fd.lone_descend(vn), None); // zero descendants
        let pc = Address::new(r, 0x2000);
        let op = fd.obank_mut().create_at(1, pc);
        fd.obank_mut().change_opcode(op, TypeOp::new(OpCode::CPUI_COPY, 0, "COPY"));
        fd.vbank_mut().add_descend(vn, op).unwrap();
        fd.obank_mut().get_mut(op).unwrap().set_input(Some(vn), 0);
        assert_eq!(fd.lone_descend(vn), Some(op));
        assert_eq!(fd.descend_snapshot(vn), vec![op]);
    }

    /// Inputs are flagged correctly through the bank set_input path used by the
    /// finder tests (sanity check the test scaffolding itself).
    #[test]
    fn make_input_sets_input_flag() {
        let mut fd = build_fd();
        let inp = make_input(&mut fd, 0x100, 4);
        let v = fd.vbank().get(inp).unwrap();
        assert!(v.is_input());
        assert_eq!(v.get_flags() & varnode_flags::input, varnode_flags::input);
    }

    // =====================================================================
    // VERIFIER (w10-doublemove): adversarial tests for combine_input_varnodes
    // (funcdata_varnode.cc:383).  The ported body has NO porter unit test in
    // this module, so these pin the C++-faithful behavior at the spots the
    // hunt list flagged: the isInput / contiguity guards (and the partial-state
    // they leave on the throw path), the little-endian contiguity arithmetic,
    // and the PIECE-reader -> COPY collapse with the new combined input.
    // =====================================================================

    /// Build a 2-input PIECE op reading (vnhi, vnlo) and register it on both
    /// operands' descend lists (mirrors how RuleDoubleOut would have wired it).
    fn make_piece_reader(fd: &mut Funcdata, off: u64, vnhi: VarnodeId, vnlo: VarnodeId) -> OpId {
        let r = ram(fd);
        let pc = Address::new(r, off);
        let op = fd.obank_mut().create_at(2, pc);
        fd.obank_mut().change_opcode(op, TypeOp::new(OpCode::CPUI_PIECE, 0, "PIECE"));
        fd.vbank_mut().add_descend(vnhi, op).unwrap();
        fd.obank_mut().get_mut(op).unwrap().set_input(Some(vnhi), 0);
        fd.vbank_mut().add_descend(vnlo, op).unwrap();
        fd.obank_mut().get_mut(op).unwrap().set_input(Some(vnlo), 1);
        op
    }

    /// HUNT LIST (Exception -> Result partial-state parity; isInput guard).
    /// C++ `combineInputVarnodes` throws "Varnodes being combined are not inputs"
    /// (funcdata_varnode.cc:386-387) BEFORE touching the graph.  The Rust port
    /// must return Err at the same point with NO mutation (no destroyed
    /// varnodes, no new input).  A free (non-input) hi varnode triggers it.
    #[test]
    fn combine_input_varnodes_w10dm_rejects_non_input_no_mutation() {
        let mut fd = build_fd();
        // lo is a real input at 0x100 size 4; hi is a *free* varnode at 0x104.
        let lo = make_input(&mut fd, 0x100, 4);
        let r = ram(&fd);
        let hi = fd.new_varnode(4, &Address::new(r, 0x104), None);
        let before = fd.num_varnodes();
        let res = fd.combine_input_varnodes(hi, lo);
        assert!(res.is_err(), "non-input hi must be rejected (cc:386-387)");
        // Partial-state parity: the early throw leaves the graph untouched.
        assert_eq!(fd.num_varnodes(), before, "no varnode created/destroyed on the reject");
        assert!(fd.vbank().get(lo).unwrap().is_input(), "lo input survives");
        assert!(fd.vbank().get(hi).unwrap().is_free(), "hi survives unchanged");
    }

    /// HUNT LIST (contiguity arithmetic + Result partial-state; little-endian).
    /// C++ LE branch: `otheraddr = vnLo->getAddr() + vnLo->getSize();
    /// isContiguous = (otheraddr == vnHi->getAddr())` (cc:395-398).  With lo@0x100
    /// size 4 -> otheraddr 0x104; a hi placed at 0x108 (NOT 0x104) is a gap, so
    /// the C++ throws "not contiguous" (cc:399-400) before any mutation.
    #[test]
    fn combine_input_varnodes_w10dm_rejects_noncontiguous_le() {
        let mut fd = build_fd();
        let lo = make_input(&mut fd, 0x100, 4);
        let hi = make_input(&mut fd, 0x108, 4); // gap: 0x104 != 0x108
        let before = fd.num_varnodes();
        let res = fd.combine_input_varnodes(hi, lo);
        assert!(res.is_err(), "non-contiguous LE pieces must be rejected (cc:399-400)");
        assert_eq!(fd.num_varnodes(), before, "no mutation on the contiguity reject");
        // The boundary case (hi exactly at otheraddr) is the *accepted* path,
        // proven by the collapse test below — confirming the == is the real edge.
    }

    /// HUNT LIST (the happy path: PIECE-reader collapse, LE addr selection).
    /// Two contiguous LE inputs (lo@0x100 sz4, hi@0x104 sz4) read by a single
    /// PIECE(hi,lo).  Per cc:443-451 the originals are destroyed, a size-8 input
    /// is created at `addr` (= lo's addr in LE, cc:389/396), and the PIECE
    /// becomes a COPY reading that new input in slot 0.  No otherOps, so no
    /// SUBPIECE replacements are made.
    #[test]
    fn combine_input_varnodes_w10dm_le_piece_collapses_to_copy() {
        let mut fd = build_fd();
        let r = ram(&fd);
        let lo = make_input(&mut fd, 0x100, 4);
        let hi = make_input(&mut fd, 0x104, 4);
        let piece = make_piece_reader(&mut fd, 0x2000, hi, lo);

        fd.combine_input_varnodes(hi, lo).expect("contiguous LE inputs combine");

        // The PIECE became a COPY (cc:450).
        assert_eq!(fd.obank().get(piece).unwrap().code(), OpCode::CPUI_COPY);
        // Its single input is the new combined whole (slot 1 removed, cc:418).
        assert_eq!(fd.obank().get(piece).unwrap().num_input(), 1, "COPY is unary now");
        let whole = fd.obank().get(piece).unwrap().get_in(0).expect("copy in0");
        let wv = fd.vbank().get(whole).unwrap();
        assert!(wv.is_input(), "combined whole is a function input (cc:446-447)");
        assert_eq!(wv.get_size(), 8, "outSize = hi.size + lo.size (cc:443)");
        // LE: the new input sits at lo's address (addr = vnLo->getAddr()).
        assert_eq!(wv.get_addr(), &Address::new(Rc::clone(&r), 0x100));
        // Originals are gone.
        assert!(fd.vbank().get(lo).is_none(), "vnLo destroyed (cc:445)");
        assert!(fd.vbank().get(hi).is_none(), "vnHi destroyed (cc:444)");
    }
}
