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
use kuna_base::space::AddrSpace;
use kuna_base::types::{int4, uintb, uintm};

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

    /// If HighVariables are enabled, make sure the given Varnode has one assigned
    /// (C++ `Funcdata::assignHigh`, `funcdata_varnode.cc:48`).
    ///
    /// SEAM(W7): the `HighVariable` subsystem (`merge`/`HighVariable`) is W7; the
    /// W3 IR never has `highlevel_on` set during construction, so this is a no-op
    /// that the factories call unconditionally — the eventual W7 body slots in
    /// without touching the callers.  The C++ early-outs unless `(flags &
    /// highlevel_on)` is set, which `is_high_on()` reports.
    fn assign_high(&mut self, _vn: VarnodeId) {
        if self.is_high_on() {
            // vn->calcCover(); ... new HighVariable(vn);  -- SEAM(W7)
        }
    }

    /// Check if the given storage range is a potential laned register
    /// (C++ `Funcdata::checkForLanedRegister`, `funcdata_varnode.cc:300`).
    ///
    /// SEAM(W4): `glb->getLanedRegister` and the `lanedMap` cache are part of the
    /// W4 `Architecture`/lane subsystem; the W3 `Architecture` skeleton carries
    /// no laned-register table and no `lanedMap` field, so this is a no-op.  The
    /// `s >= minLanedSize` guard at the call sites is transcribed faithfully so
    /// the call cadence matches the C++ exactly.
    fn check_for_laned_register(&mut self, _sz: int4, _addr: &Address) {
        // const LanedRegister *lr = glb->getLanedRegister(addr,sz);  -- SEAM(W4)
    }

    // -----------------------------------------------------------------------
    // Free-varnode creation factories (C++ new* — the plain `vbank.create`
    // path; no `xref` read-repointing, so fully portable here)
    // -----------------------------------------------------------------------

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
    /// SEAM(W3-varnode): `vn->setFlags(Varnode::annotation)` cannot be expressed —
    /// `Varnode::set_flags` is private to `varnode.rs` (this item has no edit
    /// rights there), and no public `set_annotation` setter exists.  The (space,
    /// offset, size) triple — the only thing the flow-linkage gate asserts and the
    /// only thing the branch input needs for control-flow following — is faithful;
    /// the `annotation` property bit is the single deferred detail.  Recorded as a
    /// loss (LOSS: newCodeRef annotation flag — the `funcdata_varnode` review's
    /// LOSS-036 carryover; unblocked only by `Varnode::set_flags` visibility).
    pub fn new_code_ref(&mut self, m: &Address) -> VarnodeId {
        // ct = glb->types->getTypeCode();  -- SEAM(W6): unknown base of size 1.
        let ct = Self::type_base_unknown(1);
        // vn = vbank.create(1,m,ct);
        let vn = self.vbank_mut().create(1, m.clone(), ct);
        // vn->setFlags(Varnode::annotation);  -- SEAM(W3-varnode): set_flags private.
        self.assign_high(vn);
        vn
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
}
