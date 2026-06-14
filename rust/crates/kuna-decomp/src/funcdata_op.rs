//! Port of `decompiler/cpp/funcdata_op.cc` (W3, item `w3-ir-funcdata-op`) — the
//! `Funcdata` methods pertaining directly to PcodeOps: the op-graph mutation
//! primitives (`opSetInput`/`opSetOutput`/`opInsert*`/`opDestroy`/…), the
//! dead-list discipline, and op-creation SeqNum allocation.
//!
//! These are additional `impl Funcdata` methods (the struct, IR ownership, the
//! [`PcodeOpBank`](crate::op::PcodeOpBank)/[`VarnodeBank`](crate::varnode::VarnodeBank)
//! containers, and the cross-arena basic-block op-list primitives
//! `bb_insert_op`/`bb_remove_op`/`bb_set_order` live in [`crate::funcdata`]).
//!
//! ## Scope realized here (the item's core)
//!
//! Faithfully ported, statement-for-statement (descend-list bookkeeping order is
//! load-bearing — see the C++ comments `// op must be up to date AFTER calling
//! descend_add`):
//!
//!   - **op creation / SeqNum allocation**: `newOp` (both overloads), `cloneOp`.
//!     The `uniqid`→`SeqNum::uniq` allocation path is the bank's
//!     [`create_at`](crate::op::PcodeOpBank::create_at)/[`create_seq`](crate::op::PcodeOpBank::create_seq);
//!     these wrappers add nothing, exactly as the C++ `newOp` is a one-line
//!     `obank.create(...)`.
//!   - **input/output linkage**: `opSetInput`, `opUnsetInput`, `opSwapInput`,
//!     `opSetOutput`, `opUnsetOutput`, `opSetAllInput`, `opRemoveInput`,
//!     `opInsertInput`.
//!   - **block insertion / dead-list discipline**: `opInsert`, `opUninsert`,
//!     `opUnlink`, `opDestroy`, `opDestroyRaw`, `opDestroyRecursive`,
//!     `opInsertBefore`, `opInsertAfter`, `opInsertBegin`, `opInsertEnd`,
//!     `opDeadInsertAfter`.
//!   - **op-code / flag marks**: `opSetOpcode`, `opMarkHalt`, `opMark*`/`opUndo`
//!     via the per-op flag mutators, `opFlipInPlaceTest`, `opFlipInPlaceExecute`.
//!   - **queries / scans**: `getFirstReturnOp`, `findPrimaryBranch`.
//!   - **`PcodeOp::nextOp`/`previousOp`/`target`**: block.rs/op.rs deferred these
//!     (they need *both* the op intrusive lists and the block graph, i.e.
//!     `Funcdata`); ported here as `op_next_op`/`op_previous_op`/`op_target`.
//!
//! ## Cross-wave seams (precise missing-API notes)
//!
//! This is a **parallel** porter: per the SEAM rule it owns only
//! `funcdata_op.rs` and may NOT edit `seams.rs`/`dtype.rs`.  The methods of
//! `funcdata_op.cc` that build *new Varnodes* (`newConstant`, `newUnique`,
//! `newUniqueOut`, `newVarnode`, `newVarnodeOut`, `newVarnodeIop`,
//! `newVarnodeSpace`, `newCodeRef`, `cloneVarnode`, `destroyVarnode`,
//! `newSpacebasePtr`, `newVarnodeCallSpecs`) are the **funcdata_varnode**
//! (`w3-ir-funcdata-varnode`) wave's factories — a sibling parallel item whose
//! module is still a stub at this item's merge base.  The op-manipulation
//! methods that depend on those factories are therefore deferred (`// SEAM`):
//!
//!   - `opSetOpcode(op, OpCode)` resolves `glb->inst[opc]` (the W6 `TypeOp`
//!     table) to a `TypeOp`; that table is W6's.  The faithful bank call
//!     ([`change_opcode`](crate::op::PcodeOpBank::change_opcode)) takes the
//!     resolved `TypeOp`, so the public surface here is
//!     [`op_set_opcode`](Funcdata::op_set_opcode) which takes the resolved
//!     [`TypeOp`] (caller / W6 supplies it) — matching the op.rs test harness.
//!   - `opSetInput`'s constant-duplication guard calls `newConstant`/`copySymbol`
//!     (funcdata_varnode + W4 symbol).  Transcribed with a `// SEAM`-noted
//!     fallback: a constant with an existing non-spacebase descendant is *not*
//!     re-duplicated (the W3 IR never shares constants across reads until the
//!     factory lands), and the link is made directly.  The exact note is in the
//!     method body.
//!   - `cloneOp` clones the input/output Varnodes (`cloneVarnode`); deferred.
//!   - `opDestroy`/`opDestroyRaw`/`opDestroyRecursive` destroy Varnodes
//!     (`destroyVarnode`); the op-graph half (unset inputs, dead-list move) is
//!     ported, the varnode destruction is `// SEAM(W3-varnode)`-noted.
//!
//! `opInsertAfter`'s INDIRECT-marker special case (`getOpFromConst` decoding an
//! `OpId` from an iop-space address offset) needs the iop-encoding the
//! funcdata_varnode `newVarnodeIop` establishes; that branch is `// SEAM`-noted.

use kuna_base::error::{KunaError, KunaResult};
use kuna_base::types::{int4, uint4};
use kuna_num::opcodes::OpCode;

use crate::funcdata::Funcdata;
use crate::op::pcodeop_flags;
use crate::seams::{OpId, TypeOp, VarnodeId};
use crate::varnode::DefOpInfo;

impl Funcdata {
    // -----------------------------------------------------------------------
    // op-code / halt marks
    // -----------------------------------------------------------------------

    /// Set the op-code for the given op (C++ `Funcdata::opSetOpcode`,
    /// `funcdata_op.cc:25`).
    ///
    /// The C++ body is `obank.changeOpcode(op, glb->inst[opc])`: it resolves the
    /// [`OpCode`] to the singleton `TypeOp` in the architecture's `inst` table
    /// (W6) and hands it to the bank.  That table is the W6 seam; this surface
    /// takes the already-resolved [`TypeOp`] (the caller / W6 supplies it,
    /// exactly as the op.rs bank tests call `change_opcode` with an explicit
    /// `TypeOp`).  SEAM(W6): `glb->inst[opc]`.
    pub fn op_set_opcode(&mut self, op: OpId, t_op: TypeOp) {
        // obank.changeOpcode(op, glb->inst[opc]);
        self.obank_mut().change_opcode(op, t_op);
    }

    /// `data.opSetOpcode(op, opc)` taking a bare [`OpCode`] and resolving it
    /// through the W6 `inst[]` seam (`w6_type_op`).  Convenience for in-crate
    /// callers that do not carry a resolved [`TypeOp`] (e.g. the stack-pointer
    /// flow LOAD->COPY / spurious-add rewrites in `coreaction_stackptr`).
    pub(crate) fn op_set_opcode_code(&mut self, op: OpId, opc: OpCode) {
        self.op_set_opcode(op, Self::w6_type_op(opc));
    }

    /// Mark a PcodeOp as not collapsible (C++ `Funcdata::opMarkNoCollapse`,
    /// `funcdata.hh:493` — `op->setFlag(PcodeOp::nocollapse)`).
    ///
    /// Latches a failed constant-collapse so `RuleCollapseConstants` does not
    /// retry the op (the `isCollapsible` guard reads `nocollapse`).
    pub fn op_mark_no_collapse(&mut self, op: OpId) {
        self.obank_mut()
            .get_mut(op)
            .expect("op_mark_no_collapse: stale op")
            .set_flag(crate::op::pcodeop_flags::nocollapse);
    }

    /// Mark up the given CPUI_RETURN op with a \e halt type
    /// (C++ `Funcdata::opMarkHalt`, `funcdata_op.cc:37`).
    ///
    /// Returns an error if `op` is not a RETURN or the masked flag is zero
    /// (the C++ `throw LowlevelError`).
    pub fn op_mark_halt(&mut self, op: OpId, mut flag: uint4) -> KunaResult<()> {
        let o = self.obank().get(op).expect("op_mark_halt: stale op");
        if o.code() != OpCode::CPUI_RETURN {
            return Err(KunaError::lowlevel("Only RETURN pcode ops can be marked as halt"));
        }
        // flag &= (halt|badinstruction|unimplemented|noreturn|missing);
        flag &= pcodeop_flags::halt
            | pcodeop_flags::badinstruction
            | pcodeop_flags::unimplemented
            | pcodeop_flags::noreturn
            | pcodeop_flags::missing;
        if flag == 0 {
            return Err(KunaError::lowlevel("Bad halt flag"));
        }
        self.obank_mut().get_mut(op).expect("op_mark_halt: stale op").set_flag(flag);
        Ok(())
    }

    // -----------------------------------------------------------------------
    // output linkage
    // -----------------------------------------------------------------------

    /// Unset the output Varnode of the given op (C++ `Funcdata::opUnsetOutput`,
    /// `funcdata_op.cc:52`).  The output Varnode becomes \e free but is not
    /// deleted.
    ///
    /// Statement order is load-bearing: `op->setOutput(0)` MUST come before
    /// `vbank.makeFree(vn)`.
    pub fn op_unset_output(&mut self, op: OpId) {
        // vn = op->getOut(); if (vn == 0) return;
        let vn = match self.obank().get(op).expect("op_unset_output: stale op").get_out() {
            Some(v) => v,
            None => return, // Nothing to do
        };
        // op->setOutput(0);  -- This must come before make_free
        self.obank_mut().get_mut(op).expect("op_unset_output: stale op").set_output(None);
        // vbank.makeFree(vn);
        self.vbank_mut().make_free(vn);
        // vn->clearCover();
        self.vbank_mut().get_mut(vn).expect("op_unset_output: stale vn").clear_cover();
    }

    /// Set the output Varnode of the given op (C++ `Funcdata::opSetOutput`,
    /// `funcdata_op.cc:70`).
    ///
    /// Statement order is load-bearing: unset any prior output, steal `vn` from
    /// any op currently defining it, then `vbank.setDef`/`setVarnodeProperties`/
    /// `op->setOutput` in that exact order.
    ///
    /// The central `vn = vbank.setDef(vn,op)` step needs the
    /// [`VarnodeBank`](crate::varnode::VarnodeBank) AND the
    /// [`PcodeOpBank`](crate::op::PcodeOpBank) borrowed **simultaneously** — the
    /// bank's `xref` may unify the new def varnode with an existing equivalent and
    /// must then run [`replace_reads_thunk`](Funcdata::replace_reads_thunk) over
    /// `obank` to repoint readers.  That is exactly what
    /// [`banks_mut`](Funcdata::banks_mut) supplies; `set_def` returns the kept
    /// varnode id (`vn` after a possible xref-unification), which the C++ assigns
    /// back to `vn` before `op->setOutput(vn)`.
    pub fn op_set_output(&mut self, op: OpId, vn: VarnodeId) -> KunaResult<()> {
        // if (vn == op->getOut()) return; // Already set to this vn
        if self.obank().get(op).expect("op_set_output: stale op").get_out() == Some(vn) {
            return Ok(());
        }
        // if (op->getOut() != 0) opUnsetOutput(op);
        if self.obank().get(op).expect("op_set_output: stale op").get_out().is_some() {
            self.op_unset_output(op);
        }
        // if (vn->getDef() != 0) opUnsetOutput(vn->getDef());
        if let Some(defop) = self.vbank().get(vn).expect("op_set_output: stale vn").get_def() {
            self.op_unset_output(defop);
        }
        // vn = vbank.setDef(vn, op);  -- split-borrow both banks so the xref
        //   read-repointing callback can reach obank mid-vbank-mutation.  Scoped so
        //   the thunk (holding &mut obank) drops before the later &mut self calls.
        let def = self.def_op_info(op);
        let vn = {
            let (vbank, obank) = self.banks_mut();
            let mut replace = Funcdata::replace_reads_thunk(obank);
            vbank.set_def(vn, def, &mut replace)?
        };
        // setVarnodeProperties(vn);
        self.set_varnode_properties(vn);
        // op->setOutput(vn);
        self.obank_mut().get_mut(op).expect("op_set_output: stale op").set_output(Some(vn));
        Ok(())
    }

    // -----------------------------------------------------------------------
    // input linkage
    // -----------------------------------------------------------------------

    /// Clear (unlink) the input Varnode in the given slot of the given op
    /// (C++ `Funcdata::opUnsetInput`, `funcdata_op.cc:92`).
    ///
    /// Statement order is load-bearing: `vn->eraseDescend(op)` MUST come before
    /// `op->clearInput(slot)` (the C++ comment: "Must be called AFTER
    /// descend_erase").
    pub fn op_unset_input(&mut self, op: OpId, slot: int4) {
        // Varnode *vn = op->getIn(slot);
        let vn = self
            .obank()
            .get(op)
            .expect("op_unset_input: stale op")
            .get_in(slot)
            .expect("op_unset_input: null input (C++ UB)");
        // vn->eraseDescend(op);
        self.vbank_mut().erase_descend(vn, op);
        // op->clearInput(slot);  -- Must be called AFTER descend_erase
        self.obank_mut().get_mut(op).expect("op_unset_input: stale op").clear_input(slot);
    }

    /// Set the operand Varnode in the given input slot of the given op
    /// (C++ `Funcdata::opSetInput`, `funcdata_op.cc:104`).
    ///
    /// Statement order is load-bearing: `vn->addDescend(op)` MUST come before
    /// `op->setInput(vn,slot)` (the C++ comment: "op must be up to date AFTER
    /// calling descend_add").
    ///
    /// The C++ constant-sharing guard (`opSetInput`, `funcdata_op.cc:108`)
    ///
    /// ```text
    ///   if (vn->isConstant()) {            // Constants should have only one descendant
    ///     if (!vn->hasNoDescend())
    ///       if (!vn->isSpacebase()) {      // Unless they are a spacebase
    ///         Varnode *cvn = newConstant(vn->getSize(), vn->getOffset());
    ///         cvn->copySymbol(vn); vn = cvn;
    ///       }
    ///   }
    /// ```
    ///
    /// re-duplicates a constant that already has a descendant (constants must have
    /// a single descendant): a fresh [`new_constant`](Funcdata::new_constant)
    /// stands in for the shared one so the new edge does not alias an existing
    /// read.  Now that the funcdata_varnode factory has landed, the
    /// re-duplication is ported faithfully.
    ///
    /// SEAM(W4): `cvn->copySymbol(vn)` propagates the original constant's
    /// data-type and `SymbolEntry`/`typelock|namelock` markup onto the duplicate.
    /// The type-/symbol-propagation machinery (mapentry, `HighVariable`,
    /// `TypeFactory`) is W4/W6; until it lands the duplicate carries the unknown
    /// base of its size (exactly what `newConstant` builds — and what the original
    /// `vn` itself carries in the W3 IR, where no symbol is attached yet), so the
    /// `copySymbol` is currently a structural no-op.  Recorded as a (latent) loss.
    pub fn op_set_input(&mut self, op: OpId, vn: VarnodeId, slot: int4) -> KunaResult<()> {
        let mut vn = vn;
        // if (vn == op->getIn(slot)) return; // Already set to this vn
        if self.obank().get(op).expect("op_set_input: stale op").get_in(slot) == Some(vn) {
            return Ok(());
        }
        // if (vn->isConstant()) { if (!vn->hasNoDescend()) if (!vn->isSpacebase()) {...} }
        {
            let v = self.vbank().get(vn).expect("op_set_input: stale vn");
            if v.is_constant() && !v.has_no_descend() && !v.is_spacebase() {
                // Varnode *cvn = newConstant(vn->getSize(), vn->getOffset());
                let (size, off) = (v.get_size(), v.get_offset());
                let cvn = self.new_constant(size, off);
                // cvn->copySymbol(vn);  -- SEAM(W4): type/symbol propagation (see doc).
                // vn = cvn;
                vn = cvn;
            }
        }

        // if (op->getIn(slot) != 0) opUnsetInput(op,slot);
        if self.obank().get(op).expect("op_set_input: stale op").get_in(slot).is_some() {
            self.op_unset_input(op, slot);
        }
        // vn->addDescend(op);  -- Add this op to list of vn's descendants
        self.vbank_mut().add_descend(vn, op)?;
        // op->setInput(vn,slot);  -- op must be up to date AFTER calling descend_add
        self.obank_mut().get_mut(op).expect("op_set_input: stale op").set_input(Some(vn), slot);
        Ok(())
    }

    /// Swap two input slots of the given op (C++ `Funcdata::opSwapInput`,
    /// `funcdata_op.cc:131`).  More efficient than two `opSetInput` calls and
    /// does not disturb the descend lists (the C++ uses raw `setInput`).
    pub fn op_swap_input(&mut self, op: OpId, slot1: int4, slot2: int4) {
        let o = self.obank_mut().get_mut(op).expect("op_swap_input: stale op");
        // Varnode *tmp = op->getIn(slot1);
        let tmp = o.get_in(slot1);
        // op->setInput(op->getIn(slot2),slot1);
        let in2 = o.get_in(slot2);
        o.set_input(in2, slot1);
        // op->setInput(tmp,slot2);
        o.set_input(tmp, slot2);
    }

    /// Replace all input Varnodes from the given array
    /// (C++ `Funcdata::opSetAllInput`, `funcdata_op.cc:267`).
    pub fn op_set_all_input(&mut self, op: OpId, vvec: &[VarnodeId]) -> KunaResult<()> {
        // for(i=0;i<numInput();++i) if (getIn(i) != 0) opUnsetInput(op,i);
        let n = self.obank().get(op).expect("op_set_all_input: stale op").num_input();
        for i in 0..n {
            if self.obank().get(op).expect("op_set_all_input").get_in(i).is_some() {
                self.op_unset_input(op, i);
            }
        }
        // op->setNumInputs(vvec.size());
        self.obank_mut()
            .get_mut(op)
            .expect("op_set_all_input")
            .set_num_inputs(vvec.len() as int4);
        // for(i=0;i<numInput();++i) opSetInput(op,vvec[i],i);
        let n = self.obank().get(op).expect("op_set_all_input").num_input();
        for i in 0..n {
            self.op_set_input(op, vvec[i as usize], i)?;
        }
        Ok(())
    }

    /// Remove the input Varnode in the given slot; later inputs renumber down
    /// (C++ `Funcdata::opRemoveInput`, `funcdata_op.cc:291`).
    pub fn op_remove_input(&mut self, op: OpId, slot: int4) {
        // opUnsetInput(op,slot);
        self.op_unset_input(op, slot);
        // op->removeInput(slot);
        self.obank_mut().get_mut(op).expect("op_remove_input: stale op").remove_input(slot);
    }

    /// Insert space for a new input Varnode before the given slot, then set it
    /// (C++ `Funcdata::opInsertInput`, `funcdata_op.cc:308`).
    pub fn op_insert_input(&mut self, op: OpId, vn: VarnodeId, slot: int4) -> KunaResult<()> {
        // op->insertInput(slot);
        self.obank_mut().get_mut(op).expect("op_insert_input: stale op").insert_input(slot);
        // opSetInput(op,vn,slot);
        self.op_set_input(op, vn, slot)
    }

    // -----------------------------------------------------------------------
    // op creation (SeqNum allocation — uniq path is the bank's)
    // -----------------------------------------------------------------------

    /// Create a new PcodeOp with `inputs` operands at the given Address
    /// (C++ `Funcdata::newOp(int4,const Address&)`, `funcdata_op.cc:322`).
    ///
    /// `return obank.create(inputs,pc);` — the `uniqid`→`SeqNum::uniq`
    /// allocation lives in [`create_at`](crate::op::PcodeOpBank::create_at).
    pub fn new_op(&mut self, inputs: int4, pc: kuna_base::address::Address) -> OpId {
        self.obank_mut().create_at(inputs, pc)
    }

    /// Create a new PcodeOp with `inputs` operands and a specific sequence number
    /// (C++ `Funcdata::newOp(int4,const SeqNum&)`, `funcdata_op.cc:332`).
    ///
    /// Used for cloning; `return obank.create(inputs,sq);` — the `uniqid` bump
    /// to `sq.getTime()+1` lives in [`create_seq`](crate::op::PcodeOpBank::create_seq).
    pub fn new_op_seq(&mut self, inputs: int4, sq: kuna_base::address::SeqNum) -> OpId {
        self.obank_mut().create_seq(inputs, sq)
    }

    /// Build a `CPUI_INDIRECT` op modeling the indirect effect of `indeffect` (a
    /// CALL/STORE) on a memory range (C++ `Funcdata::newIndirectOp`,
    /// `funcdata_op.cc:...`).
    ///
    /// Inserts `out[addr,sz] = INDIRECT(in[addr,sz], iop(indeffect))` immediately
    /// before `indeffect`.  Used by `Heritage::guardCalls` to prepopulate
    /// data-flow across the call's clobber of registers.
    pub fn new_indirect_op(
        &mut self,
        indeffect: OpId,
        addr: &kuna_base::address::Address,
        sz: int4,
        extra_flags: uint4,
    ) -> OpId {
        let indaddr = self.obank().get(indeffect).expect("newIndirectOp: stale op").get_addr().clone();
        let newin = self.new_varnode(sz, addr, None);
        let newop = self.new_op(2, indaddr);
        self.obank_mut().get_mut(newop).expect("newIndirectOp").set_flag(extra_flags);
        let _ = self.new_varnode_out(sz, addr, newop);
        self.op_set_opcode_code(newop, OpCode::CPUI_INDIRECT);
        let _ = self.op_set_input(newop, newin, 0);
        let iop = self.new_varnode_iop(indeffect);
        let _ = self.op_set_input(newop, iop, 1);
        self.op_insert_before(newop, indeffect);
        newop
    }

    /// Build a `CPUI_INDIRECT` *creation* op modeling a register killed (and
    /// possibly produced) by `indeffect` (C++ `Funcdata::newIndirectCreation`,
    /// `funcdata_op.cc:...`).
    ///
    /// Inserts `out[addr,sz] = INDIRECT(0, iop(indeffect))` with the
    /// `indirect_creation` marker, before `indeffect`.  Used by
    /// `Heritage::guardCalls` for a killed-by-call output range (the call's return
    /// register), and by `FuncCallSpecs::collectOutputTrialVarnodes` to find the
    /// recovered return value.
    pub fn new_indirect_creation(
        &mut self,
        indeffect: OpId,
        addr: &kuna_base::address::Address,
        sz: int4,
        possibleout: bool,
    ) -> OpId {
        use crate::op::pcodeop_flags;
        use crate::varnode::varnode_flags;
        let indaddr = self.obank().get(indeffect).expect("newIndirectCreation: stale op").get_addr().clone();
        let newin = self.new_constant(sz, 0);
        let newop = self.new_op(2, indaddr);
        self.obank_mut()
            .get_mut(newop)
            .expect("newIndirectCreation")
            .set_flag(pcodeop_flags::indirect_creation);
        let newout = self.new_varnode_out(sz, addr, newop).expect("newIndirectCreation out");
        if !possibleout {
            self.vbank_mut()
                .get_mut(newin)
                .expect("newIndirectCreation in")
                .set_flags_pub(varnode_flags::indirect_creation);
        }
        self.vbank_mut()
            .get_mut(newout)
            .expect("newIndirectCreation out")
            .set_flags_pub(varnode_flags::indirect_creation);
        self.op_set_opcode_code(newop, OpCode::CPUI_INDIRECT);
        let _ = self.op_set_input(newop, newin, 0);
        let iop = self.new_varnode_iop(indeffect);
        let _ = self.op_set_input(newop, iop, 1);
        self.op_insert_before(newop, indeffect);
        newop
    }

    /// Make a clone of the given op, copying control-flow properties; the
    /// data-type is \e not cloned (C++ `Funcdata::cloneOp`, `funcdata_op.cc:616`).
    ///
    /// SEAM(W3-varnode): the output/input Varnode clones go through
    /// `cloneVarnode`, the funcdata_varnode factory (still a stub at this merge
    /// base).  The op-shell clone (new op, opcode, the `startmark|startbasic`
    /// flag copy) is ported; the varnode-clone-and-link half returns the seam
    /// error.  Recorded as a loss; the funcdata_varnode wave fills it in.
    pub fn clone_op(
        &mut self,
        op: OpId,
        seq: kuna_base::address::SeqNum,
    ) -> KunaResult<OpId> {
        // PcodeOp *newop = newOp(op->numInput(),seq);
        let numinput = self.obank().get(op).expect("clone_op: stale op").num_input();
        let newop = self.new_op_seq(numinput, seq);
        // opSetOpcode(newop,op->code());  -- needs glb->inst[code] (W6).
        // uint4 fl = op->flags & (startmark | startbasic); newop->setFlag(fl);
        let fl = self.obank().get(op).expect("clone_op").get_flags()
            & (pcodeop_flags::startmark | pcodeop_flags::startbasic);
        self.obank_mut().get_mut(newop).expect("clone_op").set_flag(fl);
        // if (op->getOut() != 0) opSetOutput(newop,cloneVarnode(op->getOut()));
        // for(i=0;i<numInput;++i) opSetInput(newop,cloneVarnode(op->getIn(i)),i);
        //   -- SEAM(W3-varnode): cloneVarnode + opSetOpcode(W6).
        let _ = newop;
        Err(KunaError::lowlevel(
            "kuna rust port: Funcdata::cloneOp needs cloneVarnode (funcdata_varnode) \
             and glb->inst[opc] (W6); op-shell created, varnode clone deferred",
        ))
    }

    // -----------------------------------------------------------------------
    // comparison canonicalization / multiply distribution / CSE
    // -----------------------------------------------------------------------

    /// Do in-place replacement of `c <= x` with `c-1 < x`, OR `x <= c` with
    /// `x < c+1` (C++ `Funcdata::replaceLessequal`, `funcdata_op.cc:1029`).
    ///
    /// `op` is a comparison op (`INT_LESSEQUAL`/`INT_SLESSEQUAL`).  Returns `true`
    /// if a valid replacement was performed (the constant operand becomes `±1`
    /// adjusted and the op-code drops the `EQUAL`).
    ///
    /// SEAM(W6): `opSetOpcode` resolves the new `INT_LESS`/`INT_SLESS` through the
    /// `glb->inst[opc]` table — routed through [`w6_type_op`](Funcdata::w6_type_op)
    /// with the verbatim typeop.cc opflags.
    ///
    /// (kuna) provenance: the rewritten op records `setCanonicalLessequal()` — the
    /// single primitive of the comparison-canonicalization sub-stage (UPSTREAM.md
    /// *Divergence*: "`replaceLessequal` records provenance on the rewritten op").
    /// `ActionPresentCompareForm` can invert it for presentation (GH-558).  The
    /// restart-log side table (`kuna_restartlog`) records *restart* reasons, not
    /// this op flag, so no `kuna_restartlog` event is emitted here.
    pub fn replace_lessequal(&mut self, op: OpId) -> KunaResult<bool> {
        // if ((vn=op->getIn(0))->isConstant()) { diff=-1; i=0; }
        // else if ((vn=op->getIn(1))->isConstant()) { diff=1; i=1; }
        // else return false;
        let in0 = self.obank().get(op).expect("replace_lessequal: stale op").get_in(0)
            .expect("replaceLessequal: comparison null in0 (C++ UB)");
        let in1 = self.obank().get(op).expect("replace_lessequal").get_in(1)
            .expect("replaceLessequal: comparison null in1 (C++ UB)");
        let (vn, diff, i): (VarnodeId, i64, int4) =
            if self.vbank().get(in0).expect("replace_lessequal: stale in0").is_constant() {
                (in0, -1, 0)
            } else if self.vbank().get(in1).expect("replace_lessequal: stale in1").is_constant() {
                (in1, 1, 1)
            } else {
                return Ok(false);
            };

        // val = vn->getOffset();
        let v = self.vbank().get(vn).expect("replace_lessequal: stale vn");
        let val = v.get_offset();
        let size = v.get_size();
        let code = self.obank().get(op).expect("replace_lessequal").code();
        let newcode = if code == OpCode::CPUI_INT_SLESSEQUAL {
            // Check for signed overflow
            // if ((diff==-1)&&(val==calc_int_min(size))) return false;
            if diff == -1 && val == kuna_base::address::calc_int_min(size) {
                return Ok(false);
            }
            // if ((diff==1)&&(val==calc_int_max(size))) return false;
            if diff == 1 && val == kuna_base::address::calc_int_max(size) {
                return Ok(false);
            }
            // opSetOpcode(op,CPUI_INT_SLESS);
            OpCode::CPUI_INT_SLESS
        } else {
            // Check for unsigned overflow
            // if ((diff==-1)&&(val==0)) return false;
            if diff == -1 && val == 0 {
                return Ok(false);
            }
            // if ((diff==1)&&(val==calc_uint_max(size))) return false;
            if diff == 1 && val == kuna_base::address::calc_uint_max(size) {
                return Ok(false);
            }
            // opSetOpcode(op,CPUI_INT_LESS);
            OpCode::CPUI_INT_LESS
        };
        self.op_set_opcode(op, Self::w6_type_op(newcode));

        // uintb res = (val+diff) & calc_mask(size);
        let res = val.wrapping_add(diff as u64) & kuna_base::address::calc_mask(size);
        // Varnode *newvn = newConstant(size,res);
        let newvn = self.new_constant(size, res);
        // newvn->copySymbol(vn);  -- SEAM(W4): type/symbol propagation (see op_set_input).
        // opSetInput(op,newvn,i);
        self.op_set_input(op, newvn, i)?;
        // op->setCanonicalLessequal();  -- (kuna) provenance recorder.
        self.obank_mut().get_mut(op).expect("replace_lessequal: stale op").set_canonical_lessequal();
        Ok(true)
    }

    /// Flip the output condition of a CBRANCH (C++ `Funcdata::opFlipCondition`,
    /// `funcdata.hh:498` — `op->flipFlag(PcodeOp::boolean_flip)`).
    pub fn op_flip_condition(&mut self, op: OpId) {
        self.obank_mut()
            .get_mut(op)
            .expect("op_flip_condition: stale op")
            .flip_flag(pcodeop_flags::boolean_flip);
    }

    /// Remove the `boolean_flip` flag on a CBRANCH op, without changing behavior
    /// (C++ `Funcdata::opNormalizeFlip`, `funcdata_op.cc:1337`).
    ///
    /// Flips the true/false meaning of the CBRANCH and negates the comparison op
    /// feeding it (swapping `INT_LESS`↔`INT_LESSEQUAL` etc. via `get_booleanflip`,
    /// reversing operands when required, and folding the resulting `<=` constant
    /// through [`replace_lessequal`](Funcdata::replace_lessequal)).  Returns `true`
    /// if the ops were rewritten.
    pub fn op_normalize_flip(&mut self, cbranch: OpId) -> KunaResult<bool> {
        // Varnode *boolVn = cbranch->getIn(1);
        let bool_vn = match self.obank().get(cbranch).expect("op_normalize_flip: stale op").get_in(1) {
            Some(v) => v,
            None => return Ok(false),
        };
        // if (!boolVn->isWritten()) return false;
        if !self.vbank().get(bool_vn).expect("op_normalize_flip: stale boolVn").is_written() {
            return Ok(false);
        }
        // if (boolVn->loneDescend() != cbranch) return false;
        if self.lone_descend(bool_vn) != Some(cbranch) {
            return Ok(false);
        }
        // PcodeOp *condOp = boolVn->getDef();
        let cond_op = self
            .vbank()
            .get(bool_vn)
            .expect("op_normalize_flip: stale boolVn")
            .get_def()
            .expect("op_normalize_flip: written boolVn with no def (C++ UB)");
        // OpCode opc = get_booleanflip(condOp->code(), flipyes);
        let mut flipyes = false;
        let cond_code = self.obank().get(cond_op).expect("op_normalize_flip: stale condOp").code();
        let opc = kuna_num::opcodes::get_booleanflip(cond_code, &mut flipyes);
        // if (opc == CPUI_MAX) return false;
        if opc == OpCode::CPUI_MAX {
            return Ok(false);
        }
        // opSetOpcode(condOp,opc);  -- glb->inst[opc] (the canonical op-info table).
        self.op_set_opcode(cond_op, crate::typeop::type_op_for(opc));
        // if (flipyes) opSwapInput(condOp,0,1);
        if flipyes {
            self.op_swap_input(cond_op, 0, 1);
        }
        // cbranch->flipFlag(PcodeOp::boolean_flip);
        self.obank_mut()
            .get_mut(cbranch)
            .expect("op_normalize_flip: stale cbranch")
            .flip_flag(pcodeop_flags::boolean_flip);
        // if (opc == INT_LESSEQUAL || opc == INT_SLESSEQUAL) replaceLessequal(condOp);
        if opc == OpCode::CPUI_INT_LESSEQUAL || opc == OpCode::CPUI_INT_SLESSEQUAL {
            self.replace_lessequal(cond_op)?;
        }
        Ok(true)
    }

    /// Distribute a multiplicative coefficient over an additive sub-term
    /// (C++ `Funcdata::distributeIntMultAdd`, `funcdata_op.cc:1079`).
    ///
    /// `op` is an `INT_MULT` whose first input is defined by an `INT_ADD` and whose
    /// second input is a constant.  Distributes the coefficient to the `INT_ADD`
    /// inputs, turning `op` itself into the resulting `INT_ADD`.  Returns `true`.
    ///
    /// SEAM(W6): `opSetOpcode` for the new `INT_MULT`/`INT_ADD` ops is routed
    /// through [`w6_type_op`](Funcdata::w6_type_op) (verbatim typeop.cc opflags).
    pub fn distribute_int_mult_add(&mut self, op: OpId) -> KunaResult<bool> {
        // PcodeOp *addop = op->getIn(0)->getDef();
        let in0 = self.obank().get(op).expect("distribute_int_mult_add: stale op").get_in(0)
            .expect("distributeIntMultAdd: MULT null in0 (C++ UB)");
        let addop = self.vbank().get(in0).expect("distribute_int_mult_add: stale in0").get_def()
            .expect("distributeIntMultAdd: in0 has no def (C++ UB)");
        // Varnode *vn0 = addop->getIn(0); Varnode *vn1 = addop->getIn(1);
        let vn0 = self.obank().get(addop).expect("distribute_int_mult_add: stale addop").get_in(0)
            .expect("distributeIntMultAdd: ADD null in0 (C++ UB)");
        let vn1 = self.obank().get(addop).expect("distribute_int_mult_add").get_in(1)
            .expect("distributeIntMultAdd: ADD null in1 (C++ UB)");
        // if ((vn0->isFree())&&(!vn0->isConstant())) return false;
        {
            let v0 = self.vbank().get(vn0).expect("distribute_int_mult_add: stale vn0");
            if v0.is_free() && !v0.is_constant() {
                return Ok(false);
            }
            let v1 = self.vbank().get(vn1).expect("distribute_int_mult_add: stale vn1");
            if v1.is_free() && !v1.is_constant() {
                return Ok(false);
            }
        }
        // uintb coeff = op->getIn(1)->getOffset();
        let op_in1 = self.obank().get(op).expect("distribute_int_mult_add").get_in(1)
            .expect("distributeIntMultAdd: MULT null in1 (C++ UB)");
        let coeff = self.vbank().get(op_in1).expect("distribute_int_mult_add: stale in1").get_offset();
        // int4 sz = op->getOut()->getSize();
        let out = self.obank().get(op).expect("distribute_int_mult_add").get_out()
            .expect("distributeIntMultAdd: MULT has no output (C++ UB)");
        let sz = self.vbank().get(out).expect("distribute_int_mult_add: stale out").get_size();
        let mask = kuna_base::address::calc_mask(sz);
        let pc = self.obank().get(op).expect("distribute_int_mult_add").get_addr().clone();

        // Do distribution -- first input.
        let newvn0 = if self.vbank().get(vn0).expect("distribute_int_mult_add: stale vn0").is_constant() {
            // uintb val = coeff * vn0->getOffset(); val &= calc_mask(sz);
            let v0off = self.vbank().get(vn0).expect("distribute_int_mult_add").get_offset();
            let val = coeff.wrapping_mul(v0off) & mask;
            self.new_constant(sz, val)
        } else {
            // PcodeOp *newop0 = newOp(2,op->getAddr()); opSetOpcode(newop0,CPUI_INT_MULT);
            let newop0 = self.new_op(2, pc.clone());
            self.op_set_opcode(newop0, Self::w6_type_op(OpCode::CPUI_INT_MULT));
            // newvn0 = newUniqueOut(sz,newop0);
            let r = self.new_unique_out(sz, newop0)?;
            // opSetInput(newop0, vn0, 0);
            self.op_set_input(newop0, vn0, 0)?;
            // Varnode *newcvn = newConstant(sz,coeff); opSetInput(newop0, newcvn, 1);
            let newcvn = self.new_constant(sz, coeff);
            self.op_set_input(newop0, newcvn, 1)?;
            // opInsertBefore(newop0, op);
            self.op_insert_before(newop0, op);
            r
        };

        // Second input.
        let newvn1 = if self.vbank().get(vn1).expect("distribute_int_mult_add: stale vn1").is_constant() {
            let v1off = self.vbank().get(vn1).expect("distribute_int_mult_add").get_offset();
            let val = coeff.wrapping_mul(v1off) & mask;
            self.new_constant(sz, val)
        } else {
            let newop1 = self.new_op(2, pc);
            self.op_set_opcode(newop1, Self::w6_type_op(OpCode::CPUI_INT_MULT));
            let r = self.new_unique_out(sz, newop1)?;
            self.op_set_input(newop1, vn1, 0)?;
            let newcvn = self.new_constant(sz, coeff);
            self.op_set_input(newop1, newcvn, 1)?;
            self.op_insert_before(newop1, op);
            r
        };

        // opSetInput(op,newvn0,0); opSetInput(op,newvn1,1); opSetOpcode(op,CPUI_INT_ADD);
        self.op_set_input(op, newvn0, 0)?;
        self.op_set_input(op, newvn1, 1)?;
        self.op_set_opcode(op, Self::w6_type_op(OpCode::CPUI_INT_ADD));
        Ok(true)
    }

    /// Eliminate a depth-1 common-subexpression redundancy between two ops
    /// (C++ `Funcdata::cseElimination`, `funcdata_op.cc:1397`).
    ///
    /// The two ops perform the identical operation on identical operands; return
    /// the remaining (dominating) op.  If neither dominates, both are eliminated
    /// and a new op is built at a commonly accessible point (`findCommonBlock`).
    ///
    /// SEAM(W6): `opSetOpcode(replace, op1->code())` for the freshly built op is
    /// routed through [`w6_type_op`](Funcdata::w6_type_op).
    pub fn cse_elimination(&mut self, op1: OpId, op2: OpId) -> KunaResult<OpId> {
        let par1 = self.obank().get(op1).expect("cse_elimination: stale op1").get_parent()
            .expect("cseElimination: op1 has no parent (C++ UB)");
        let par2 = self.obank().get(op2).expect("cse_elimination: stale op2").get_parent()
            .expect("cseElimination: op2 has no parent (C++ UB)");
        let replace: OpId = if par1 == par2 {
            // if (op1->getSeqNum().getOrder() < op2->getSeqNum().getOrder()) replace=op1; else op2;
            let o1 = self.obank().get(op1).expect("cse_elimination").get_seq_num().get_order();
            let o2 = self.obank().get(op2).expect("cse_elimination").get_seq_num().get_order();
            if o1 < o2 { op1 } else { op2 }
        } else {
            // common = findCommonBlock(op1->getParent(), op2->getParent());
            let common = self.bblocks_mut().find_common_block(par1, par2);
            if common == Some(par1) {
                op1
            } else if common == Some(par2) {
                op2
            } else {
                // Neither op is ancestor of the other -- build a fresh op at common's stop.
                let common = common.expect("cseElimination: no common block (C++ UB)");
                let stop = crate::block::block_get_stop(&self.bblocks_ref().arena, common);
                // replace = newOp(op1->numInput(),common->getStop());
                let numin = self.obank().get(op1).expect("cse_elimination").num_input();
                let replace = self.new_op(numin, stop);
                // opSetOpcode(replace,op1->code());
                let code1 = self.obank().get(op1).expect("cse_elimination").code();
                self.op_set_opcode(replace, Self::w6_type_op(code1));
                // newVarnodeOut(op1->getOut()->getSize(),op1->getOut()->getAddr(),replace);
                let out1 = self.obank().get(op1).expect("cse_elimination").get_out()
                    .expect("cseElimination: op1 has no output (C++ UB)");
                let (osz, oaddr) = {
                    let v = self.vbank().get(out1).expect("cse_elimination: stale out1");
                    (v.get_size(), v.get_addr().clone())
                };
                self.new_varnode_out(osz, &oaddr, replace)?;
                // for each input: constants are re-duplicated, else passed through.
                for ix in 0..numin {
                    let inx = self.obank().get(op1).expect("cse_elimination").get_in(ix)
                        .expect("cseElimination: op1 null input (C++ UB)");
                    if self.vbank().get(inx).expect("cse_elimination: stale input").is_constant() {
                        let (isz, ioff) = {
                            let v = self.vbank().get(inx).expect("cse_elimination");
                            (v.get_size(), v.get_offset())
                        };
                        let c = self.new_constant(isz, ioff);
                        self.op_set_input(replace, c, ix)?;
                    } else {
                        self.op_set_input(replace, inx, ix)?;
                    }
                }
                // opInsertEnd(replace,common);
                self.op_insert_end(replace, common);
                replace
            }
        };
        // if (replace != op1) { totalReplace(op1->getOut(),replace->getOut()); opDestroy(op1); }
        if replace != op1 {
            let from = self.obank().get(op1).expect("cse_elimination").get_out()
                .expect("cseElimination: op1 has no output (C++ UB)");
            let to = self.obank().get(replace).expect("cse_elimination").get_out()
                .expect("cseElimination: replace has no output (C++ UB)");
            self.total_replace(from, to)?;
            self.op_destroy(op1);
        }
        if replace != op2 {
            let from = self.obank().get(op2).expect("cse_elimination").get_out()
                .expect("cseElimination: op2 has no output (C++ UB)");
            let to = self.obank().get(replace).expect("cse_elimination").get_out()
                .expect("cseElimination: replace has no output (C++ UB)");
            self.total_replace(from, to)?;
            self.op_destroy(op2);
        }
        Ok(replace)
    }

    /// Perform CSE on a list of `(hash, op)` pairs (descendants of one Varnode)
    /// (C++ `Funcdata::cseEliminateList`, `funcdata_op.cc:1459`).
    ///
    /// The list is `stable_sort`ed by hash (the verbatim `compareCseHash`
    /// comparator); equal-hash neighbours that are a true `isCseMatch` and whose
    /// outputs are both heritaged (or null) are eliminated, the surviving output
    /// pushed onto `outlist`.
    ///
    /// SEAM(W7): the C++ `isHeritaged(outvn)` is `heritage.heritagePass(addr)>=0`
    /// on the `Heritage` instance the C++ `Funcdata` owns.  This Rust `Funcdata`
    /// does not own a `Heritage` yet (W7 heritage-deps), so the heritaged test is
    /// passed in as the `is_heritaged` predicate — the W7 caller supplies
    /// `|vn| heritage.heritage_pass(&addr_of(vn)) >= 0`.  Every other statement
    /// (the sort, the dead/CSE-match gate, the elimination + output collection) is
    /// fully ported.
    pub fn cse_eliminate_list(
        &mut self,
        list: &mut [(kuna_base::types::uintm, OpId)],
        outlist: &mut Vec<VarnodeId>,
        mut is_heritaged: impl FnMut(&Funcdata, VarnodeId) -> bool,
    ) -> KunaResult<()> {
        // if (list.empty()) return;
        if list.is_empty() {
            return Ok(());
        }
        // stable_sort(list.begin(),list.end(),compareCseHash);  -- compare by .first.
        list.sort_by(|a, b| a.0.cmp(&b.0));
        // liter1 = begin; liter2 = begin+1; while(liter2 != end) {...}
        let mut idx = 1;
        while idx < list.len() {
            let (h1, op1) = list[idx - 1];
            let (h2, op2) = list[idx];
            if h1 == h2 {
                // if ((!op1->isDead())&&(!op2->isDead())&&op1->isCseMatch(op2)) {...}
                let dead1 = self.obank().get(op1).expect("cse_eliminate_list: stale op1").is_dead();
                let dead2 = self.obank().get(op2).expect("cse_eliminate_list: stale op2").is_dead();
                if !dead1 && !dead2 && self.op_is_cse_match(op1, op2) {
                    // Varnode *outvn1 = op1->getOut(); Varnode *outvn2 = op2->getOut();
                    let outvn1 = self.obank().get(op1).expect("cse_eliminate_list").get_out();
                    let outvn2 = self.obank().get(op2).expect("cse_eliminate_list").get_out();
                    // if ((outvn1==0)||isHeritaged(outvn1))
                    //   if ((outvn2==0)||isHeritaged(outvn2)) {...}
                    let h1ok = match outvn1 {
                        None => true,
                        Some(v) => is_heritaged(self, v),
                    };
                    let h2ok = match outvn2 {
                        None => true,
                        Some(v) => is_heritaged(self, v),
                    };
                    if h1ok && h2ok {
                        // resop = cseElimination(op1,op2); outlist.push_back(resop->getOut());
                        let resop = self.cse_elimination(op1, op2)?;
                        let resout = self.obank().get(resop).expect("cse_eliminate_list: stale resop")
                            .get_out().expect("cseEliminateList: resop has no output (C++ UB)");
                        outlist.push(resout);
                    }
                }
            }
            idx += 1;
        }
        Ok(())
    }

    /// `op1->isCseMatch(op2)` routed through the [`is_cse_match`](crate::op::is_cse_match)
    /// free function (it needs the [`VarnodeBank`](crate::varnode::VarnodeBank) and
    /// both ops; the obank is borrowed twice immutably here).
    fn op_is_cse_match(&self, op1: OpId, op2: OpId) -> bool {
        let o1 = self.obank().get(op1).expect("op_is_cse_match: stale op1");
        let o2 = self.obank().get(op2).expect("op_is_cse_match: stale op2");
        crate::op::is_cse_match(o1, o2, self.vbank())
    }

    /// SEAM(W6): resolve an [`OpCode`] to its [`TypeOp`] via the `glb->inst[opc]`
    /// table, supplying the verbatim `typeop.cc` `opflags` for the handful of
    /// op-codes the funcdata helpers above install (so the resulting op reports the
    /// right `binary`/`booloutput`/`commutative` eval-type bits).  The W6 wave
    /// replaces this with the real `Architecture::inst` table; until then the flag
    /// word is transcribed inline (mirrors `ruleaction_5::type_op_seam`).
    fn w6_type_op(opc: OpCode) -> TypeOp {
        use pcodeop_flags as f;
        // opflags transcribed verbatim from decompiler/cpp/typeop.cc.
        let (flags, name): (uint4, &str) = match opc {
            // TypeOpIntSless / TypeOpIntSlessEqual: binary | booloutput
            OpCode::CPUI_INT_SLESS => (f::binary | f::booloutput, "<"),
            // TypeOpIntLess / TypeOpIntLessEqual: binary | booloutput
            OpCode::CPUI_INT_LESS => (f::binary | f::booloutput, "<"),
            // TypeOpIntAdd: binary | commutative
            OpCode::CPUI_INT_ADD => (f::binary | f::commutative, "+"),
            // TypeOpIntMult: binary | commutative
            OpCode::CPUI_INT_MULT => (f::binary | f::commutative, "*"),
            // cseElimination rebuilds `op1->code()`; the common unary/binary ops it
            // sees carry just the eval-type bit (no special semantics needed here).
            OpCode::CPUI_COPY => (f::unary | f::nocollapse, "copy"),
            OpCode::CPUI_SUBPIECE => (f::binary, "SUB"),
            OpCode::CPUI_INT_SRIGHT => (f::binary, "s>>"),
            // Any other op-code the helpers reach is a porting bug; fall back to a
            // bare binary op so eval-type still classifies it (the worst case is a
            // missing special-semantics flag, never an incorrect rewrite).
            _ => (f::binary, "?"),
        };
        TypeOp::new(opc, flags, name.to_string())
    }

    // -----------------------------------------------------------------------
    // block insertion / dead-list discipline
    // -----------------------------------------------------------------------

    /// Insert the given op into a basic block immediately before `before` (or at
    /// the end of `bl` if `before` is `None`)
    /// (C++ `Funcdata::opInsert`, `funcdata_op.cc:150`).
    ///
    /// `obank.markAlive(op); bl->insert(iter,op);` — the block insert is the
    /// cross-arena [`bb_insert_op`](Funcdata::bb_insert_op).
    pub fn op_insert(&mut self, op: OpId, bl: crate::seams::BlockId, before: Option<OpId>) {
        // obank.markAlive(op);
        self.obank_mut().mark_alive(op);
        // bl->insert(iter,op);
        self.bb_insert_op(op, bl, before);
    }

    /// Remove the op from its basic block and return it to the dead list
    /// (C++ `Funcdata::opUninsert`, `funcdata_op.cc:164`).
    ///
    /// `obank.markDead(op); op->getParent()->removeOp(op);`
    pub fn op_uninsert(&mut self, op: OpId) {
        // obank.markDead(op);
        self.obank_mut().mark_dead(op);
        // op->getParent()->removeOp(op);
        let parent = self
            .obank()
            .get(op)
            .expect("op_uninsert: stale op")
            .get_parent()
            .expect("op_uninsert: op has no parent (C++ UB)");
        self.bb_remove_op(parent, op);
    }

    /// Extricate the op from all its Varnode connections and remove it from its
    /// basic block; the op stays in the dead list, block connections unchanged
    /// (C++ `Funcdata::opUnlink`, `funcdata_op.cc:179`).
    pub fn op_unlink(&mut self, op: OpId) {
        // opUnsetOutput(op);
        self.op_unset_output(op);
        // for(i=0;i<numInput();++i) opUnsetInput(op,i);
        let n = self.obank().get(op).expect("op_unlink: stale op").num_input();
        for i in 0..n {
            self.op_unset_input(op, i);
        }
        // if (op->getParent() != 0) opUninsert(op);
        if self.obank().get(op).expect("op_unlink").get_parent().is_some() {
            self.op_uninsert(op);
        }
    }

    /// Destroy the given op: unset its output/inputs and move it permanently to
    /// the dead list (C++ `Funcdata::opDestroy`, `funcdata_op.cc:203`).
    ///
    /// SEAM(W3-varnode): the C++ `destroyVarnode(op->getOut())` frees the output
    /// Varnode's object resources (the funcdata_varnode factory).  The op-graph
    /// half is ported faithfully — the output is *unset* (made free) here in
    /// place of destroyed; the input links are unset; the op is marked dead and
    /// removed from its block.  This keeps the IR consistent (no dangling
    /// def/use) at the cost of leaving a now-orphan free output Varnode in the
    /// bank until the funcdata_varnode wave wires `destroy_varnode`.  Recorded as
    /// a loss.
    pub fn op_destroy(&mut self, op: OpId) {
        // if (op->getOut() != 0) destroyVarnode(op->getOut());
        //   -- SEAM(W3-varnode): destroyVarnode not yet ported; unset instead so
        //      the def/use graph stays consistent.
        if self.obank().get(op).expect("op_destroy: stale op").get_out().is_some() {
            self.op_unset_output(op);
        }
        // for(i=0;i<numInput();++i) { vn=op->getIn(i); if (vn!=0) opUnsetInput(op,i); }
        let n = self.obank().get(op).expect("op_destroy").num_input();
        for i in 0..n {
            if self.obank().get(op).expect("op_destroy").get_in(i).is_some() {
                self.op_unset_input(op, i);
            }
        }
        // if (op->getParent() != 0) { obank.markDead(op); op->getParent()->removeOp(op); }
        if let Some(parent) = self.obank().get(op).expect("op_destroy").get_parent() {
            self.obank_mut().mark_dead(op);
            self.bb_remove_op(parent, op);
        }
    }

    /// Recursively remove the op and any single-use, non-special ops feeding only
    /// it (C++ `Funcdata::opDestroyRecursive`, `funcdata_op.cc:228`).
    ///
    /// `scratch` is reused exactly as the C++ scratch vector (worklist order is
    /// observable).
    pub fn op_destroy_recursive(&mut self, op: OpId, scratch: &mut Vec<OpId>) {
        scratch.clear();
        scratch.push(op);
        let mut pos = 0;
        while pos < scratch.len() {
            let op = scratch[pos];
            pos += 1;
            let n = self.obank().get(op).expect("op_destroy_recursive: stale op").num_input();
            for i in 0..n {
                let vn = self
                    .obank()
                    .get(op)
                    .expect("op_destroy_recursive")
                    .get_in(i)
                    .expect("op_destroy_recursive: null input (C++ UB)");
                let v = self.vbank().get(vn).expect("op_destroy_recursive: stale vn");
                // if (!vn->isWritten() || vn->isAutoLive()) continue;
                if !v.is_written() || v.is_auto_live() {
                    continue;
                }
                // if (vn->loneDescend() == 0) continue;
                if self.vn_lone_descend(vn).is_none() {
                    continue;
                }
                // PcodeOp *defOp = vn->getDef();
                let def_op = v.get_def().expect("op_destroy_recursive: written vn has no def");
                // if (defOp->isCall() || defOp->isIndirectSource()) continue;
                let d = self.obank().get(def_op).expect("op_destroy_recursive: stale defOp");
                if d.is_call() || d.is_indirect_source() {
                    continue;
                }
                scratch.push(def_op);
            }
            self.op_destroy(op);
        }
    }

    /// Destroy a \e dead op replaced during flow generation, with all of its
    /// input/output Varnodes (C++ `Funcdata::opDestroyRaw`, `funcdata_op.cc:253`).
    ///
    /// Faithful transcription now that
    /// [`destroy_varnode`](Funcdata::destroy_varnode) (funcdata_varnode) is ported:
    /// destroy every input Varnode, then the output if present, then the op
    /// (`obank.destroy(op)`; the op must be dead).  Statement order matches the
    /// C++ exactly (inputs first, then output, then the op).
    pub fn op_destroy_raw(&mut self, op: OpId) -> KunaResult<()> {
        // for(int4 i=0;i<op->numInput();++i) destroyVarnode(op->getIn(i));
        let n = self.obank().get(op).expect("op_destroy_raw: stale op").num_input();
        for i in 0..n {
            let vn = self
                .obank()
                .get(op)
                .expect("op_destroy_raw: stale op")
                .get_in(i)
                .expect("op_destroy_raw: null input (C++ destroyVarnode UB)");
            self.destroy_varnode(vn)?;
        }
        // if (op->getOut() != 0) destroyVarnode(op->getOut());
        if let Some(out) = self.obank().get(op).expect("op_destroy_raw: stale op").get_out() {
            self.destroy_varnode(out)?;
        }
        // obank.destroy(op);  -- op must be dead; the bank panics otherwise.
        self.obank_mut().destroy(op);
        Ok(())
    }

    /// Insert the op into the dead list immediately after `prev`
    /// (C++ `Funcdata::opDeadInsertAfter`, `funcdata.hh` inline → `op.cc`
    /// `insertAfterDead`).  Both must be dead.
    pub fn op_dead_insert_after(&mut self, op: OpId, prev: OpId) {
        self.obank_mut().insert_after_dead(op, prev);
    }

    // -----------------------------------------------------------------------
    // block-relative insertion (the MULTIEQUAL/INDIRECT/branch invariants)
    // -----------------------------------------------------------------------

    /// Insert the given op immediately before `follow`, honoring the SSA op
    /// ordering invariants (C++ `Funcdata::opInsertBefore`, `funcdata_op.cc:345`).
    ///
    ///   - MULTIEQUALS in a basic block all occur first
    ///   - INDIRECTs occur immediately before their op
    ///   - a branch op must be the very last op in a basic block
    pub fn op_insert_before(&mut self, op: OpId, follow: OpId) {
        // iter = follow->getBasicIter(); parent = follow->getParent();
        let parent = self
            .obank()
            .get(follow)
            .expect("op_insert_before: stale follow")
            .get_parent()
            .expect("op_insert_before: follow has no parent (C++ UB)");
        // The C++ `iter` starts at `follow`; we track the insertion point as the
        // op we will insert *before* (the dereferenced iterator), `point`.
        let mut point: Option<OpId> = Some(follow);

        // if (op->code() != CPUI_INDIRECT) { skip back over preceding INDIRECTs }
        if self.obank().get(op).expect("op_insert_before: stale op").code()
            != OpCode::CPUI_INDIRECT
        {
            // while(iter != parent->beginOp()) { --iter; prev=*iter;
            //   if (prev->code()!=INDIRECT) { ++iter; break; } }
            let head = self.bb_op_head(parent);
            loop {
                if point == head {
                    break; // iter == beginOp(): stop
                }
                // --iter: move to the predecessor of `point`.
                let prev = match point {
                    Some(p) => self.obank().get(p).expect("op_insert_before").basic_neighbours().0,
                    None => self.bb_op_tail(parent),
                };
                let previousop = prev.expect("op_insert_before: walked past begin (C++ UB)");
                if self.obank().get(previousop).expect("op_insert_before").code()
                    != OpCode::CPUI_INDIRECT
                {
                    // ++iter; break;  -- insertion point stays at `point`.
                    break;
                }
                // prev was INDIRECT: keep walking back; iter now at previousop.
                point = Some(previousop);
            }
        }
        // opInsert(op,parent,iter);
        self.op_insert(op, parent, point);
    }

    /// Insert the given op immediately after `prev`, honoring the SSA op ordering
    /// invariants (C++ `Funcdata::opInsertAfter`, `funcdata_op.cc:373`).
    ///
    /// SEAM(W3-varnode): the INDIRECT-marker redirect
    ///
    /// ```text
    ///   if (prev->isMarker()) {
    ///     if (prev->code() == CPUI_INDIRECT) {
    ///       Varnode *invn = prev->getIn(1);
    ///       if (invn->getSpace()->getType()==IPTR_IOP) {
    ///         PcodeOp *targOp = PcodeOp::getOpFromConst(invn->getAddr());
    ///         if (!targOp->isDead()) prev = targOp;
    ///       } } }
    /// ```
    ///
    /// decodes an [`OpId`] from the iop-space address offset of the INDIRECT's
    /// second input (`getOpFromConst`).  That iop encoding is established by the
    /// funcdata_varnode `newVarnodeIop` factory (a sibling parallel item) and is
    /// not available here; the redirect is skipped (treated as a no-op marker
    /// input), so when `prev` is an INDIRECT with an iop input the op is inserted
    /// after the INDIRECT itself rather than after the INDIRECT's target.
    /// Recorded as a loss; the funcdata_varnode wave supplies the decode.
    pub fn op_insert_after(&mut self, op: OpId, prev: OpId) {
        let mut prev = prev;
        // if (prev->isMarker()) { if INDIRECT { ... getOpFromConst ... } }
        //   -- SEAM(W3-varnode): getOpFromConst decode deferred (see doc).
        if self.obank().get(prev).expect("op_insert_after: stale prev").is_marker()
            && self.obank().get(prev).expect("op_insert_after").code() == OpCode::CPUI_INDIRECT
        {
            // The iop redirect is the deferred part; without it `prev` is unchanged.
            // (Faithful for every non-iop INDIRECT input, which is the common case
            // until the iop factory lands.)
        }

        // iter = prev->getBasicIter(); parent = prev->getParent(); iter++;
        let parent = self
            .obank()
            .get(prev)
            .expect("op_insert_after: stale prev")
            .get_parent()
            .expect("op_insert_after: prev has no parent (C++ UB)");
        // After `iter++`, the insertion point (op we insert *before*) is prev's
        // successor; `None` means end-of-block.
        let mut point: Option<OpId> =
            self.obank().get(prev).expect("op_insert_after").basic_neighbours().1;
        let _ = &mut prev;

        // if (op->code() != CPUI_MULTIEQUAL) { skip forward over MULTIEQUALs }
        if self.obank().get(op).expect("op_insert_after: stale op").code()
            != OpCode::CPUI_MULTIEQUAL
        {
            // while(iter != parent->endOp()) { nextop=*iter; ++iter;
            //   if (nextop->code()!=MULTIEQUAL) { --iter; break; } }
            // The loop ends when iter==endOp() (point==None); the explicit `break`
            // fires on the first non-MULTIEQUAL, leaving `point` at that op
            // (the C++ `--iter`).
            while let Some(nextop) = point {
                // ++iter first, then test.
                let after = self.obank().get(nextop).expect("op_insert_after").basic_neighbours().1;
                if self.obank().get(nextop).expect("op_insert_after").code()
                    != OpCode::CPUI_MULTIEQUAL
                {
                    // --iter; break;  -- insertion point is `nextop`.
                    break;
                }
                // nextop was MULTIEQUAL: advance.
                point = after;
            }
        }
        // opInsert(op,prev->getParent(),iter);
        self.op_insert(op, parent, point);
    }

    /// Insert the given op as the \e first op in the basic block, honoring the
    /// MULTIEQUAL-first invariant (C++ `Funcdata::opInsertBegin`,
    /// `funcdata_op.cc:413`).
    pub fn op_insert_begin(&mut self, op: OpId, bl: crate::seams::BlockId) {
        // iter = bl->beginOp();
        let mut point: Option<OpId> = self.bb_op_head(bl);
        // if (op->code()!=MULTIEQUAL) { while(iter!=endOp()) { if (*iter != MULTIEQUAL) break; ++iter; } }
        if self.obank().get(op).expect("op_insert_begin: stale op").code()
            != OpCode::CPUI_MULTIEQUAL
        {
            while let Some(cur) = point {
                if self.obank().get(cur).expect("op_insert_begin").code()
                    != OpCode::CPUI_MULTIEQUAL
                {
                    break;
                }
                point = self.obank().get(cur).expect("op_insert_begin").basic_neighbours().1;
            }
        }
        // opInsert(op,bl,iter);
        self.op_insert(op, bl, point);
    }

    /// Insert the given op as the \e last op in the basic block, honoring the
    /// branch-last invariant (C++ `Funcdata::opInsertEnd`, `funcdata_op.cc:435`).
    pub fn op_insert_end(&mut self, op: OpId, bl: crate::seams::BlockId) {
        // iter = bl->endOp();
        // if (iter != bl->beginOp()) { --iter; if (!(*iter)->isFlowBreak()) ++iter; }
        let head = self.bb_op_head(bl);
        let tail = self.bb_op_tail(bl);
        // Insertion point (op we insert before); `None` == end-of-block.
        let point: Option<OpId> = if head.is_none() {
            // empty block: iter == beginOp() == endOp(); no decrement.
            None
        } else {
            // --iter -> last op.  If it is NOT a flow-break, ++iter (insert at end);
            // otherwise insert before the flow-break (the branch stays last).
            let last = tail.expect("op_insert_end: non-empty block has no tail");
            if self.obank().get(last).expect("op_insert_end").is_flow_break() {
                Some(last)
            } else {
                None
            }
        };
        // opInsert(op,bl,iter);
        self.op_insert(op, bl, point);
    }

    // -----------------------------------------------------------------------
    // boolean flip machinery
    // -----------------------------------------------------------------------

    /// Trace a boolean value to the set of ops whose op-codes can be changed to
    /// flip it (C++ `Funcdata::opFlipInPlaceTest`, `funcdata_op.cc:1230`).
    ///
    /// Returns 0 if the change normalizes, 1 if it denormalizes/is ambivalent,
    /// 2 if flip-in-place is not possible.  `fliplist` is appended in the exact
    /// C++ order (the order is consumed by `opFlipInPlaceExecute`).
    pub fn op_flip_in_place_test(
        &self,
        op: OpId,
        fliplist: &mut Vec<OpId>,
        allow_op_removal: bool,
    ) -> int4 {
        let o = self.obank().get(op).expect("op_flip_in_place_test: stale op");
        match o.code() {
            OpCode::CPUI_CBRANCH => {
                // vn = op->getIn(1);
                let vn = o.get_in(1).expect("opFlipInPlaceTest: CBRANCH null in1 (C++ UB)");
                // if (vn->loneDescend() != op) return 2;
                if self.vn_lone_descend(vn) != Some(op) {
                    return 2;
                }
                // if (!vn->isWritten()) return 2;
                let v = self.vbank().get(vn).expect("opFlipInPlaceTest: stale vn");
                if !v.is_written() {
                    return 2;
                }
                let def = v.get_def().expect("opFlipInPlaceTest: written vn has no def");
                let mut subtest1 = self.op_flip_in_place_test(def, fliplist, allow_op_removal);
                // if (subtest1 != 2 && op->isBooleanFlip()) subtest1 = 1-subtest1;
                if subtest1 != 2 && o.is_boolean_flip() {
                    subtest1 = 1 - subtest1;
                }
                subtest1
            }
            OpCode::CPUI_INT_EQUAL | OpCode::CPUI_FLOAT_EQUAL => {
                fliplist.push(op);
                1
            }
            OpCode::CPUI_BOOL_NEGATE => {
                if !allow_op_removal {
                    return 2;
                }
                fliplist.push(op);
                0
            }
            OpCode::CPUI_INT_NOTEQUAL | OpCode::CPUI_FLOAT_NOTEQUAL => {
                fliplist.push(op);
                0
            }
            OpCode::CPUI_INT_SLESS | OpCode::CPUI_INT_LESS => {
                // vn = op->getIn(0);
                let vn = o.get_in(0).expect("opFlipInPlaceTest: LESS null in0 (C++ UB)");
                fliplist.push(op);
                // if (!vn->isConstant()) return 1; return 0;
                if !self.vbank().get(vn).expect("opFlipInPlaceTest: stale vn").is_constant() {
                    1
                } else {
                    0
                }
            }
            OpCode::CPUI_INT_SLESSEQUAL | OpCode::CPUI_INT_LESSEQUAL => {
                // vn = op->getIn(1);
                let vn = o.get_in(1).expect("opFlipInPlaceTest: LESSEQUAL null in1 (C++ UB)");
                fliplist.push(op);
                // if (vn->isConstant()) return 1; return 0;
                if self.vbank().get(vn).expect("opFlipInPlaceTest: stale vn").is_constant() {
                    1
                } else {
                    0
                }
            }
            OpCode::CPUI_BOOL_OR | OpCode::CPUI_BOOL_AND => {
                // vn = op->getIn(0); loneDescend/isWritten checks; recurse.
                let vn0 = o.get_in(0).expect("opFlipInPlaceTest: BOOL null in0 (C++ UB)");
                if self.vn_lone_descend(vn0) != Some(op) {
                    return 2;
                }
                let v0 = self.vbank().get(vn0).expect("opFlipInPlaceTest: stale vn");
                if !v0.is_written() {
                    return 2;
                }
                let def0 = v0.get_def().expect("opFlipInPlaceTest: written vn0 has no def");
                let subtest1 = self.op_flip_in_place_test(def0, fliplist, allow_op_removal);
                if subtest1 == 2 {
                    return 2;
                }
                // vn = op->getIn(1); same checks; recurse.
                let vn1 = o.get_in(1).expect("opFlipInPlaceTest: BOOL null in1 (C++ UB)");
                if self.vn_lone_descend(vn1) != Some(op) {
                    return 2;
                }
                let v1 = self.vbank().get(vn1).expect("opFlipInPlaceTest: stale vn");
                if !v1.is_written() {
                    return 2;
                }
                let def1 = v1.get_def().expect("opFlipInPlaceTest: written vn1 has no def");
                let subtest2 = self.op_flip_in_place_test(def1, fliplist, allow_op_removal);
                if subtest2 == 2 {
                    return 2;
                }
                fliplist.push(op);
                // return subtest1;  -- Front of AND/OR must be normalizing
                subtest1
            }
            _ => 2,
        }
    }

    /// Perform the op-code flips computed by [`op_flip_in_place_test`] in place
    /// (C++ `Funcdata::opFlipInPlaceExecute`, `funcdata_op.cc:1296`).
    ///
    /// SEAM(W6)+SEAM(W3-varnode): the per-op flip resolves a *new* op-code via
    /// `get_booleanflip` (the W6 op-code table) and, for the LESSEQUAL→LESS
    /// canonicalization, calls `replaceLessequal` (which builds a `newConstant`,
    /// funcdata_varnode).  Both are out of this parallel item; the method returns
    /// the seam error.  The flip *list* (`opFlipInPlaceTest`) is the W3-portable
    /// half and is fully ported.  Recorded as a loss.
    pub fn op_flip_in_place_execute(&mut self, _fliplist: &[OpId]) -> KunaResult<()> {
        // for each op: opc=get_booleanflip(code(),flipyes); dispatch:
        //   COPY  -> remove BOOL_NEGATE (opSetInput/opDestroy)
        //   MAX   -> swap BOOL_AND<->BOOL_OR (opSetOpcode -- W6 inst table)
        //   else  -> opSetOpcode(opc); if (flipyes) { opSwapInput; replaceLessequal }
        //   -- SEAM(W6): get_booleanflip + glb->inst[opc]; SEAM(W3-varnode): replaceLessequal.
        Err(KunaError::lowlevel(
            "kuna rust port: Funcdata::opFlipInPlaceExecute needs get_booleanflip + glb->inst (W6) \
             and replaceLessequal/newConstant (funcdata_varnode); flip-list (opFlipInPlaceTest) ported",
        ))
    }

    // -----------------------------------------------------------------------
    // queries / scans
    // -----------------------------------------------------------------------

    /// Return the first non-dead, non-artificial-halt CPUI_RETURN op, or `None`
    /// (C++ `Funcdata::getFirstReturnOp`, `funcdata_op.cc:632`).
    pub fn get_first_return_op(&self) -> Option<OpId> {
        // for(iter=beginOp(RETURN); iter!=endOp(RETURN); ++iter)
        for retop in self.obank().iter_code(OpCode::CPUI_RETURN) {
            let r = self.obank().get(retop).expect("get_first_return_op: stale op");
            // if (retop->isDead()) continue;
            if r.is_dead() {
                continue;
            }
            // if (retop->getHaltType()!=0) continue;
            if r.get_halt_type() != 0 {
                continue;
            }
            return Some(retop);
        }
        None
    }

    /// Find the \e primary branch op among the ops of one instruction
    /// (C++ `Funcdata::findPrimaryBranch`, `funcdata_op.cc:929`).
    ///
    /// `ops` is the instruction's ops in `optree` order (the C++
    /// `PcodeOpTree::const_iterator` range; the caller resolves it from
    /// [`iter_at`](crate::op::PcodeOpBank::iter_at)).
    pub fn find_primary_branch(
        &self,
        ops: &[OpId],
        findbranch: bool,
        findcall: bool,
        findreturn: bool,
    ) -> Option<OpId> {
        for &op in ops {
            let o = self.obank().get(op).expect("find_primary_branch: stale op");
            match o.code() {
                OpCode::CPUI_BRANCH | OpCode::CPUI_CBRANCH => {
                    if findbranch {
                        // if (!op->getIn(0)->isConstant()) return op; // not internal branch
                        let in0 =
                            o.get_in(0).expect("findPrimaryBranch: BRANCH null in0 (C++ UB)");
                        if !self.vbank().get(in0).expect("findPrimaryBranch: stale vn").is_constant()
                        {
                            return Some(op);
                        }
                    }
                }
                OpCode::CPUI_BRANCHIND => {
                    if findbranch {
                        return Some(op);
                    }
                }
                OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => {
                    if findcall {
                        return Some(op);
                    }
                }
                OpCode::CPUI_RETURN => {
                    if findreturn {
                        return Some(op);
                    }
                }
                _ => {}
            }
        }
        None
    }

    // -----------------------------------------------------------------------
    // PcodeOp::nextOp / previousOp / target (block.rs/op.rs deferred to here)
    // -----------------------------------------------------------------------

    /// Return the unique next op in control flow, or `None`
    /// (C++ `PcodeOp::nextOp`, `op.cc:346`).
    ///
    /// Follows flow into successive blocks while there is exactly one path
    /// (`sizeOut()==1 || ==2`, taking `getOut(0)`).  Needs both the op intrusive
    /// basic-block list and the block graph, so it lives on `Funcdata`.
    pub fn op_next_op(&self, op: OpId) -> Option<OpId> {
        // p = parent; iter = basiciter; iter++;
        let mut p = self.obank().get(op).expect("op_next_op: stale op").get_parent()?;
        // `iter++` past `op`: the successor in p's op list.
        let mut iter: Option<OpId> =
            self.obank().get(op).expect("op_next_op").basic_neighbours().1;
        // while(iter == p->endOp()) { ... }  -- iter==endOp() is `None`.
        while iter.is_none() {
            let so = self.bblocks_ref().block(p).size_out();
            if so != 1 && so != 2 {
                return None;
            }
            // p = p->getOut(0); iter = p->beginOp();
            p = self.bblocks_ref().block(p).get_out(0);
            iter = self.bb_op_head(p);
        }
        iter
    }

    /// Return the previous op within this op's basic block, or `None`
    /// (C++ `PcodeOp::previousOp`, `op.cc:367`).  Does not search past the block.
    pub fn op_previous_op(&self, op: OpId) -> Option<OpId> {
        // if (basiciter == parent->beginOp()) return 0; iter = basiciter; iter--;
        self.obank().get(op).expect("op_previous_op: stale op").basic_neighbours().0
    }

    /// Return the instruction-start op for the instruction containing `op`
    /// (C++ `PcodeOp::target`, `op.cc:383`).
    ///
    /// Scans backward (over the dead-list `insertiter` if the op is dead, else
    /// the block `basiciter`) to the first op with the `startmark` flag.  Works
    /// before blocks are calculated (all ops still on the dead list).
    pub fn op_target(&self, op: OpId) -> OpId {
        // iter = isDead() ? insertiter : basiciter; retop = *iter;
        let dead = self.obank().get(op).expect("op_target: stale op").is_dead();
        // For the dead case the C++ walks the global dead list backward via
        // `insertiter`.  op.rs owns the dead-list intrusive links privately and
        // exposes only forward iteration ([`iter_dead`]); reconstruct the
        // predecessor map once (semantically identical — the dead list is a plain
        // doubly-linked list, so `--insertiter` is the previous element).
        let dead_prev: std::collections::BTreeMap<OpId, OpId> = if dead {
            let order: Vec<OpId> = self.obank().iter_dead().collect();
            order
                .windows(2)
                .map(|w| (w[1], w[0])) // prev-of(w[1]) == w[0]
                .collect()
        } else {
            std::collections::BTreeMap::new()
        };
        let mut retop = op;
        // while((retop->flags & startmark)==0) { --iter; retop = *iter; }
        while (self.obank().get(retop).expect("op_target").get_flags() & pcodeop_flags::startmark)
            == 0
        {
            retop = if dead {
                // --insertiter
                *dead_prev
                    .get(&retop)
                    .expect("op_target: walked past dead-list begin (C++ UB)")
            } else {
                // --basiciter
                self.obank()
                    .get(retop)
                    .expect("op_target")
                    .basic_neighbours()
                    .0
                    .expect("op_target: walked past block begin (C++ UB)")
            };
        }
        retop
    }

    /// Find the first executing PcodeOp at-or-after a target address
    /// (C++ `PcodeOpBank::target`, `op.cc:1119`).
    ///
    /// The C++ takes `lower_bound(SeqNum(addr,0))` then runs `op->target()` on
    /// it; here [`target_lower_bound`](crate::op::PcodeOpBank::target_lower_bound)
    /// supplies the lower-bound op and [`op_target`](Funcdata::op_target) does the
    /// backward instruction-start scan.  Returns `None` when no op is at-or-after.
    pub fn op_bank_target(&self, addr: &kuna_base::address::Address) -> Option<OpId> {
        let op = self.obank().target_lower_bound(addr)?;
        Some(self.op_target(op))
    }

    // -----------------------------------------------------------------------
    // local helpers (Varnode reads routed through the bank, ADR 0001)
    // -----------------------------------------------------------------------

    /// `vn->loneDescend()` (C++ `Varnode::loneDescend`, `varnode.cc`): the unique
    /// descendant op, or `None` if there are zero or more than one.  Read-only,
    /// so it lives here rather than on the (parallel) varnode wave.
    fn vn_lone_descend(&self, vn: VarnodeId) -> Option<OpId> {
        let v = self.vbank().get(vn).expect("vn_lone_descend: stale vn");
        let mut it = v.descend_iter();
        let first = it.next()?;
        if it.next().is_some() {
            return None; // More than 1 descendant
        }
        Some(first)
    }

    /// Build the `DefOpInfo` carrier for an op (its id + SeqNum), the
    /// `VarnodeBank::set_def`/`create_def` paths take.
    fn def_op_info(&self, op: OpId) -> DefOpInfo {
        let seqnum = self.obank().get(op).expect("def_op_info: stale op").get_seq_num().clone();
        DefOpInfo { id: op, seqnum }
    }
}

#[cfg(test)]
mod tests {
    include!("funcdata_op/tests.rs");
}
