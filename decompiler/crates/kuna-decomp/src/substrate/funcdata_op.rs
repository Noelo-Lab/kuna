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
//! ## Cross-wave boundaries (precise missing-API notes)
//!
//! This is a **parallel** porter: per the STUB rule it owns only
//! `funcdata_op.rs` and may NOT edit `context.rs`/`dtype.rs`.  The methods of
//! `funcdata_op.cc` that build *new Varnodes* (`newConstant`, `newUnique`,
//! `newUniqueOut`, `newVarnode`, `newVarnodeOut`, `newVarnodeIop`,
//! `newVarnodeSpace`, `newCodeRef`, `cloneVarnode`, `destroyVarnode`,
//! `newSpacebasePtr`, `newVarnodeCallSpecs`) are the **funcdata_varnode**
//! (`w3-ir-funcdata-varnode`) wave's factories — a sibling parallel item whose
//! module is still a stub at this item's merge base.  The op-manipulation
//! methods that depend on those factories are therefore deferred (`// STUB`):
//!
//!   - `opSetOpcode(op, OpCode)` resolves `glb->inst[opc]` (the W6 `TypeOp`
//!     table) to a `TypeOp`; that table is W6's.  The faithful bank call
//!     ([`change_opcode`](crate::op::PcodeOpBank::change_opcode)) takes the
//!     resolved `TypeOp`, so the public surface here is
//!     [`op_set_opcode`](Funcdata::op_set_opcode) which takes the resolved
//!     [`TypeOp`] (caller / W6 supplies it) — matching the op.rs test harness.
//!   - `opSetInput`'s constant-duplication guard calls `newConstant`/`copySymbol`
//!     (funcdata_varnode + W4 symbol).  Transcribed with a `// STUB`-noted
//!     fallback: a constant with an existing non-spacebase descendant is *not*
//!     re-duplicated (the W3 IR never shares constants across reads until the
//!     factory lands), and the link is made directly.  The exact note is in the
//!     method body.
//!   - `cloneOp` clones the input/output Varnodes (`cloneVarnode`); deferred.
//!   - `opDestroy`/`opDestroyRaw`/`opDestroyRecursive` destroy Varnodes
//!     (`destroyVarnode`); the op-graph half (unset inputs, dead-list move) is
//!     ported, the varnode destruction is `// STUB(W3-varnode)`-noted.
//!
//! `opInsertAfter`'s INDIRECT-marker special case (`getOpFromConst` decoding an
//! `OpId` from an iop-space address offset) needs the iop-encoding the
//! funcdata_varnode `newVarnodeIop` establishes; that branch is `// STUB`-noted.

use std::rc::Rc;

use kuna_base::error::{KunaError, KunaResult};
use kuna_base::space::AddrSpace;
use kuna_base::types::{int4, uint4, uintb};
use kuna_num::opcodes::OpCode;

use crate::funcdata::Funcdata;
use crate::op::pcodeop_flags;
use crate::context::{OpId, TypeOp, VarnodeId};
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
    /// (W6) and hands it to the bank.  That table is the W6 boundary; this surface
    /// takes the already-resolved [`TypeOp`] (the caller / W6 supplies it,
    /// exactly as the op.rs bank tests call `change_opcode` with an explicit
    /// `TypeOp`).  STUB(W6): `glb->inst[opc]`.
    pub fn op_set_opcode(&mut self, op: OpId, t_op: TypeOp) {
        self.obank_mut().change_opcode(op, t_op);
    }

    /// `data.opSetOpcode(op, opc)` taking a bare [`OpCode`] and resolving it
    /// through the W6 `inst[]` boundary (`w6_type_op`).  Convenience for in-crate
    /// callers that do not carry a resolved [`TypeOp`] (e.g. the stack-pointer
    /// flow LOAD->COPY / spurious-add rewrites in `coreaction_stackptr`).
    pub(crate) fn op_set_opcode_code(&mut self, op: OpId, opc: OpCode) {
        self.op_set_opcode(op, Self::w6_type_op(opc));
    }

    /// Mark a PcodeOp as halting type propagation through it (C++
    /// `PcodeOp::setStopTypePropagation`, set on a `PTRSUB`/`PTRADD` created by
    /// `RulePtrArith`/`RuleStructOffset0` so the inferred sub data-type is not
    /// re-propagated past the access).
    pub fn op_set_stop_type_propagation(&mut self, op: OpId) {
        self.obank_mut()
            .get_mut(op)
            .expect("op_set_stop_type_propagation: stale op")
            .set_stop_type_propagation();
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

    /// Mark a PcodeOp as a LOAD/STORE through a spacebase pointer (C++
    /// `Funcdata::opMarkSpacebasePtr`, `funcdata.hh:496` —
    /// `op->setFlag(PcodeOp::spacebase_ptr)`).
    pub fn op_mark_spacebase_ptr(&mut self, op: OpId) {
        self.obank_mut()
            .get_mut(op)
            .expect("op_mark_spacebase_ptr: stale op")
            .set_flag(pcodeop_flags::spacebase_ptr);
    }

    /// Unmark a PcodeOp as using a spacebase pointer (C++
    /// `Funcdata::opClearSpacebasePtr`, `funcdata.hh:497` —
    /// `op->clearFlag(PcodeOp::spacebase_ptr)`).
    pub fn op_clear_spacebase_ptr(&mut self, op: OpId) {
        self.obank_mut()
            .get_mut(op)
            .expect("op_clear_spacebase_ptr: stale op")
            .clear_flag(pcodeop_flags::spacebase_ptr);
    }

    /// Mark a PcodeOp as having a boolean output (C++
    /// `Funcdata::opMarkCalculatedBool`, `funcdata.hh:495` —
    /// `op->setFlag(PcodeOp::calculated_bool)`).
    pub fn op_mark_calculated_bool(&mut self, op: OpId) {
        self.obank_mut()
            .get_mut(op)
            .expect("op_mark_calculated_bool: stale op")
            .set_flag(pcodeop_flags::calculated_bool);
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

    /// Mark a `CPUI_INDIRECT` op so its output is \e indirectly \e created
    /// (C++ `Funcdata::markIndirectCreation`, `funcdata_op.cc:736`).
    ///
    /// An indirectly-created Varnode effectively has no data-flow before the
    /// INDIRECT op that defines it.  `possible_output` is `true` when the INDIRECT
    /// should be marked as a possible call output (its in0 is then NOT flagged).
    /// Used by [`TransformManager::special_handling`](crate::transform).
    pub fn mark_indirect_creation(&mut self, indop: OpId, possible_output: bool) -> KunaResult<()> {
        use crate::varnode::varnode_flags;
        let outvn = self
            .obank()
            .get(indop)
            .expect("mark_indirect_creation: stale indop")
            .get_out()
            .expect("mark_indirect_creation: INDIRECT has no output");
        let in0 = self
            .obank()
            .get(indop)
            .expect("mark_indirect_creation: stale indop")
            .get_in(0)
            .expect("mark_indirect_creation: INDIRECT missing in0");

        self.obank_mut()
            .get_mut(indop)
            .expect("mark_indirect_creation: stale indop")
            .set_flag(pcodeop_flags::indirect_creation);
        if !self.vbank().get(in0).expect("mark_indirect_creation: stale in0").is_constant() {
            return Err(KunaError::lowlevel("Indirect creation not properly formed"));
        }
        if !possible_output {
            self.vbank_mut()
                .get_mut(in0)
                .expect("mark_indirect_creation: stale in0")
                .set_flags_pub(varnode_flags::indirect_creation);
        }
        self.vbank_mut()
            .get_mut(outvn)
            .expect("mark_indirect_creation: stale outvn")
            .set_flags_pub(varnode_flags::indirect_creation);
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
        let vn = match self.obank().get(op).expect("op_unset_output: stale op").get_out() {
            Some(v) => v,
            None => return, // Nothing to do
        };
        self.obank_mut().get_mut(op).expect("op_unset_output: stale op").set_output(None);
        self.vbank_mut().make_free(vn);
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
        // Already set to this vn.
        if self.obank().get(op).expect("op_set_output: stale op").get_out() == Some(vn) {
            return Ok(());
        }
        if self.obank().get(op).expect("op_set_output: stale op").get_out().is_some() {
            self.op_unset_output(op);
        }
        if let Some(defop) = self.vbank().get(vn).expect("op_set_output: stale vn").get_def() {
            self.op_unset_output(defop);
        }
        // Split-borrow both banks so the xref
        //   read-repointing callback can reach obank mid-vbank-mutation.  Scoped so
        //   the thunk (holding &mut obank) drops before the later &mut self calls.
        let def = self.def_op_info(op);
        let vn = {
            let (vbank, obank) = self.banks_mut();
            let mut replace = Funcdata::replace_reads_thunk(obank);
            vbank.set_def(vn, def, &mut replace)?
        };
        self.set_varnode_properties(vn);
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
        let vn = self
            .obank()
            .get(op)
            .expect("op_unset_input: stale op")
            .get_in(slot)
            .expect("op_unset_input: null input (C++ UB)");
        self.vbank_mut().erase_descend(vn, op);
        self.obank_mut().get_mut(op).expect("op_unset_input: stale op").clear_input(slot);
    }

    /// C++ `Varnode::copySymbol` (varnode.cc:512): copy any bound dynamic
    /// SymbolEntry from `src` onto `dst`.  In the merged-tree stand-in the
    /// SymbolEntry binding is the `kuna_symbol_entry` Symbol id parked on the
    /// Varnode (and, for an equate, mirrored on its HighVariable so the printer's
    /// `vn_high_equate_symbol` lookup finds it).  The C++ also copies the type and
    /// the type/name locks; the equate's type is `getBase(1,TYPE_UNKNOWN)` and the
    /// forced display format is carried by the Symbol itself, so reusing the
    /// existing binding (without retyping `dst`) preserves the render facet while
    /// avoiding a spurious size-1 retype of the collapsed constant.
    pub(crate) fn copy_symbol(&mut self, dst: VarnodeId, src: VarnodeId) {
        let sym = match self.vbank().get(src).and_then(|v| v.kuna_symbol_entry()) {
            Some(s) => s,
            None => return,
        };
        // dst->setSymbolEntry-equivalent: park the binding on the Varnode (this
        // marks it `Varnode::mapped`), then mirror onto its High for the printer.
        if let Some(v) = self.vbank_mut().get_mut(dst) {
            v.set_kuna_symbol_entry(sym);
        }
        let is_equate = self
            .get_scope_local()
            .map(|l| {
                l.database().symbol(sym).get_category() == crate::database::symbol_category::EQUATE
            })
            .unwrap_or(false);
        if is_equate {
            if let Some(high) = self.vbank().get(dst).and_then(|v| v.get_high()) {
                if let Some(h) = self.high_bank_mut().get_mut(high) {
                    h.set_kuna_equate_symbol(sym);
                }
            }
        }
    }

    /// C++ `Varnode::copySymbolIfValid` (varnode.cc:528): copy symbol info from the
    /// constant `src` onto the constant `dst`, but only when `src` carries an
    /// EquateSymbol whose value is \e close (negation, complement, ±1, sign-extend
    /// fold) to `dst`'s constant value at `dst`'s size — the test that lets an
    /// equate survive the constant-folding transforms (`isValueClose`,
    /// database.cc:641).  Drives the convert `L'a'` force_char equate across the
    /// CALL-arg size-8 -> size-4 truncation.
    pub fn copy_symbol_if_valid(&mut self, dst: VarnodeId, src: VarnodeId) {
        let sym = match self.vbank().get(src).and_then(|v| v.kuna_symbol_entry()) {
            Some(s) => s,
            None => return,
        };
        let equate_value = match self.get_scope_local() {
            Some(l) => match l.database().symbol(sym).kind {
                crate::database::SymbolKind::Equate { value } => value,
                _ => return, // dynamic_cast<EquateSymbol*> == 0 -> not an equate
            },
            None => return,
        };
        let (dst_off, dst_size) = match self.vbank().get(dst) {
            Some(v) => (v.get_offset(), v.get_size()),
            None => return,
        };
        if crate::database::equate_is_value_close(equate_value, dst_off, dst_size) {
            self.copy_symbol(dst, src);
        }
    }

    /// Set the operand Varnode in the given input slot of the given op
    /// (C++ `Funcdata::opSetInput`, `funcdata_op.cc:104`).
    ///
    /// Statement order is load-bearing: `vn->addDescend(op)` MUST come before
    /// `op->setInput(vn,slot)` (the C++ comment: "op must be up to date AFTER
    /// calling descend_add").
    ///
    /// The C++ constant-sharing guard (`opSetInput`, `funcdata_op.cc:108`)
    /// re-duplicates a constant that already has a descendant (constants must have
    /// a single descendant): a fresh [`new_constant`](Funcdata::new_constant)
    /// stands in for the shared one so the new edge does not alias an existing
    /// read.  Now that the funcdata_varnode factory has landed, the
    /// re-duplication is ported faithfully.
    ///
    /// STUB(W4): `cvn->copySymbol(vn)` propagates the original constant's
    /// data-type and `SymbolEntry`/`typelock|namelock` markup onto the duplicate.
    /// The type-/symbol-propagation machinery (mapentry, `HighVariable`,
    /// `TypeFactory`) is W4/W6; until it lands the duplicate carries the unknown
    /// base of its size (exactly what `newConstant` builds — and what the original
    /// `vn` itself carries in the W3 IR, where no symbol is attached yet), so the
    /// `copySymbol` is currently a structural no-op.  Recorded as a (latent) loss.
    pub fn op_set_input(&mut self, op: OpId, vn: VarnodeId, slot: int4) -> KunaResult<()> {
        let mut vn = vn;
        // C++ `opSetInput` takes `Varnode *vn`, which may be `(Varnode *)0`
        // (e.g. `opInsertInput(op,(Varnode *)0,slot)` from `setInsertInputs`'s
        // input-growth loop).  The C++ null Varnode is `VarnodeId::default()` here.
        // C++ then hits `if (vn == op->getIn(slot)) return;` — for a null `vn`
        // against the freshly-grown (null) slot this is the null==null early
        // return, so a null insert is a structural no-op (the slot is just grown).
        // Without this short-circuit the null `vn` falls through to the
        // `vn->isConstant()` deref below and panics on a stale/non-existent
        // Varnode (the stack-struct bitfield INSERT-input growth path).
        let cur = self.obank().get(op).expect("op_set_input: stale op").get_in(slot);
        if vn == VarnodeId::default() {
            // null vn: only valid when the slot is already null (matches the C++
            // `vn == getIn(slot)` null==null early return).
            if cur.is_none() {
                return Ok(());
            }
        }
        // Already set to this vn.
        if cur == Some(vn) {
            return Ok(());
        }
        {
            let v = self.vbank().get(vn).expect("op_set_input: stale vn");
            if v.is_constant() && !v.has_no_descend() && !v.is_spacebase() {
                let (size, off) = (v.get_size(), v.get_offset());
                let cvn = self.new_constant(size, off);
                // Preserve any bound (dynamic) SymbolEntry
                // onto the duplicated constant (C++ `funcdata_op.cc:111`).  Without
                // this an equate parked on a shared constant is lost the first time
                // RulePropagateCopy duplicates it, before the size-fold collapse can
                // propagate it forward.  Same-size copy, so no value-close gate.
                self.copy_symbol(cvn, vn);
                vn = cvn;
            }
        }

        if self.obank().get(op).expect("op_set_input: stale op").get_in(slot).is_some() {
            self.op_unset_input(op, slot);
        }
        // Add this op to list of vn's descendants.
        self.vbank_mut().add_descend(vn, op)?;
        // op must be up to date AFTER calling descend_add.
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
        let n = self.obank().get(op).expect("op_set_all_input: stale op").num_input();
        for i in 0..n {
            if self.obank().get(op).expect("op_set_all_input").get_in(i).is_some() {
                self.op_unset_input(op, i);
            }
        }
        self.obank_mut()
            .get_mut(op)
            .expect("op_set_all_input")
            .set_num_inputs(vvec.len() as int4);
        let n = self.obank().get(op).expect("op_set_all_input").num_input();
        for i in 0..n {
            self.op_set_input(op, vvec[i as usize], i)?;
        }
        Ok(())
    }

    /// Remove the input Varnode in the given slot; later inputs renumber down
    /// (C++ `Funcdata::opRemoveInput`, `funcdata_op.cc:291`).
    pub fn op_remove_input(&mut self, op: OpId, slot: int4) {
        self.op_unset_input(op, slot);
        self.obank_mut().get_mut(op).expect("op_remove_input: stale op").remove_input(slot);
    }

    /// Insert space for a new input Varnode before the given slot, then set it
    /// (C++ `Funcdata::opInsertInput`, `funcdata_op.cc:308`).
    pub fn op_insert_input(&mut self, op: OpId, vn: VarnodeId, slot: int4) -> KunaResult<()> {
        self.obank_mut().get_mut(op).expect("op_insert_input: stale op").insert_input(slot);
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

    /// Build a fresh spacebase-base-register reference Varnode (C++
    /// `Funcdata::newSpacebasePtr`, `funcdata.cc:273`).
    ///
    /// `id` must have a base register (otherwise `getSpacebase(0)` errors).
    pub fn new_spacebase_ptr(&mut self, id: &Rc<AddrSpace>) -> KunaResult<VarnodeId> {
        let point = id.get_spacebase(0)?;
        let space = point
            .space
            .clone()
            .ok_or_else(|| KunaError::lowlevel("newSpacebasePtr: spacebase has no space"))?;
        Ok(self.new_varnode(
            point.size as int4,
            &kuna_base::address::Address::new(space, point.offset),
            None,
        ))
    }

    /// Build the stack-pointer-relative reference `INT_ADD(spacebase, off)` for a
    /// stack access at `off` in `spc` (C++ `Funcdata::createStackRef`,
    /// `funcdata_op.cc:459`).
    ///
    /// `stackptr` reuses an existing stack-pointer reference, or `None` to build a
    /// new `newSpacebasePtr`.  The `off` is byte-to-address scaled by the space's
    /// word size.  The SEGMENTOP container arm (`getSegmentOp`) is unreached on
    /// flat stacks (no segmented user-op on the x86 spacebase) — a faithful stub
    /// (the Rust tree has no `SegmentOp` user-op surface yet); the plain INT_ADD
    /// reference is the path the stack-relative parameter recovery exercises.
    pub fn create_stack_ref(
        &mut self,
        spc: &Rc<AddrSpace>,
        off: uintb,
        op: OpId,
        stackptr: Option<VarnodeId>,
        insertafter: bool,
    ) -> KunaResult<VarnodeId> {
        let stackptr = match stackptr {
            Some(v) => v,
            None => self.new_spacebase_ptr(spc)?,
        };
        let addrsize = self
            .vbank()
            .get(stackptr)
            .ok_or_else(|| KunaError::lowlevel("createStackRef: stale stackptr"))?
            .get_size();
        let op_addr = self
            .obank()
            .get(op)
            .ok_or_else(|| KunaError::lowlevel("createStackRef: stale op"))?
            .get_addr()
            .clone();
        let addop = self.new_op(2, op_addr);
        self.op_set_opcode_code(addop, OpCode::CPUI_INT_ADD);
        let addout = self.new_unique_out(addrsize, addop)?;
        self.op_set_input(addop, stackptr, 0)?;
        let off = AddrSpace::byte_to_address(off, spc.get_word_size());
        let coff = self.new_constant(addrsize, off);
        self.op_set_input(addop, coff, 1)?;
        if insertafter {
            self.op_insert_after(addop, op);
        } else {
            self.op_insert_before(addop, op);
        }
        // STUB(flat-stack): the SEGMENTOP container arm (glb->userops.getSegmentOp
        // on spc->getContain()) only fires for segmented spacebases; the flat x86
        // stack has no segment user-op, so addout is the reference directly.
        Ok(addout)
    }

    /// Build `LOAD` of `sz` bytes from the stack at `off` in `spc` (C++
    /// `Funcdata::opStackLoad`, `funcdata_op.cc:541`).
    ///
    /// `stackref` reuses an existing stack reference (else a new one is built via
    /// [`Self::create_stack_ref`]).  The LOAD is inserted right after the stack
    /// reference op regardless of `insertafter`.
    pub fn op_stack_load(
        &mut self,
        spc: &Rc<AddrSpace>,
        off: uintb,
        sz: uint4,
        op: OpId,
        stackref: Option<VarnodeId>,
        insertafter: bool,
    ) -> KunaResult<VarnodeId> {
        let addout = self.create_stack_ref(spc, off, op, stackref, insertafter)?;
        let op_addr = self
            .obank()
            .get(op)
            .ok_or_else(|| KunaError::lowlevel("opStackLoad: stale op"))?
            .get_addr()
            .clone();
        let loadop = self.new_op(2, op_addr);
        self.op_set_opcode_code(loadop, OpCode::CPUI_LOAD);
        let contain = spc
            .get_contain()
            .cloned()
            .ok_or_else(|| KunaError::lowlevel("opStackLoad: spacebase has no container"))?;
        let spacevn = self.new_varnode_space(&contain);
        self.op_set_input(loadop, spacevn, 0)?;
        self.op_set_input(loadop, addout, 1)?;
        let res = self.new_unique_out(sz as int4, loadop)?;
        // LOAD comes after the stack ref op.
        let addout_def = self
            .vbank()
            .get(addout)
            .and_then(|v| v.get_def())
            .ok_or_else(|| KunaError::lowlevel("opStackLoad: stack ref has no def"))?;
        self.op_insert_after(loadop, addout_def);
        Ok(res)
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

    /// Create a new PcodeOp with 2 or 3 operands and a unique-space output, inserted
    /// before `follow` (C++ `Funcdata::newOpBefore`, `funcdata_op.cc:656`).
    ///
    /// `in3 == None` builds a 2-input op; otherwise a 3-input op.  The output is a
    /// fresh \e unique Varnode sized to `in1`.
    pub fn new_op_before(
        &mut self,
        follow: OpId,
        opc: OpCode,
        in1: VarnodeId,
        in2: VarnodeId,
        in3: Option<VarnodeId>,
    ) -> OpId {
        let sz = if in3.is_none() { 2 } else { 3 };
        let addr = self.obank().get(follow).expect("new_op_before: stale follow").get_addr().clone();
        let newop = self.new_op(sz, addr);
        self.op_set_opcode_code(newop, opc);
        let in1_size = self.vbank().get(in1).expect("new_op_before: stale in1").get_size();
        let _ = self.new_unique_out(in1_size, newop);
        let _ = self.op_set_input(newop, in1, 0);
        let _ = self.op_set_input(newop, in2, 1);
        if let Some(in3) = in3 {
            let _ = self.op_set_input(newop, in3, 2);
        }
        self.op_insert_before(newop, follow);
        newop
    }

    /// Convert a PTRADD back to an INT_ADD, folding the element-size multiplier
    /// (C++ `Funcdata::opUndoPtradd`, `funcdata_op.cc:579`).
    ///
    /// `finalize` controls whether the recreated index Varnode picks up the
    /// (read-facing) data-type of the offset/multiplier — used by the late
    /// finalization pass.
    pub fn op_undo_ptradd(&mut self, op: OpId, finalize: bool) {
        let mult_vn = self.obank().get(op).expect("op_undo_ptradd: stale op").get_in(2)
            .expect("op_undo_ptradd: PTRADD missing slot 2");
        let mult_off = self.vbank().get(mult_vn).expect("op_undo_ptradd: stale multVn").get_offset();
        // cast: the PTRADD element-size constant fits int4 (a data-type byte size).
        let mult_size = mult_off as int4;
        self.op_remove_input(op, 2);
        self.op_set_opcode_code(op, OpCode::CPUI_INT_ADD);
        // No multiplier, done.
        if mult_size == 1 {
            return;
        }
        let off_vn = self.obank().get(op).expect("op_undo_ptradd: stale op").get_in(1)
            .expect("op_undo_ptradd: missing slot 1");
        let (off_is_const, off_off, off_size) = {
            let v = self.vbank().get(off_vn).expect("op_undo_ptradd: stale offVn");
            (v.is_constant(), v.get_offset(), v.get_size())
        };
        if off_is_const {
            let new_val = (mult_off.wrapping_mul(off_off)) & kuna_base::address::calc_mask(off_size);
            let new_off_vn = self.new_constant(off_size, new_val);
            if finalize {
                let rf = self.vbank().get(off_vn).expect("op_undo_ptradd: stale offVn")
                    .get_type_read_facing(op).clone();
                self.vbank_mut().get_mut(new_off_vn).expect("op_undo_ptradd: stale newOff").update_type(rf);
            }
            let _ = self.op_set_input(op, new_off_vn, 1);
            return;
        }
        let op_addr = self.obank().get(op).expect("op_undo_ptradd: stale op").get_addr().clone();
        let mult_op = self.new_op(2, op_addr);
        self.op_set_opcode_code(mult_op, OpCode::CPUI_INT_MULT);
        let add_vn = self.new_unique_out(off_size, mult_op).expect("op_undo_ptradd: unique out");
        if finalize {
            let mt = self.vbank().get(mult_vn).expect("op_undo_ptradd: stale multVn").get_type().clone();
            let av = self.vbank_mut().get_mut(add_vn).expect("op_undo_ptradd: stale addVn");
            av.update_type(mt);
            av.set_implied();
        }
        let _ = self.op_set_input(mult_op, off_vn, 0);
        let _ = self.op_set_input(mult_op, mult_vn, 1);
        let _ = self.op_set_input(op, add_vn, 1);
        self.op_insert_before(mult_op, op);
    }

    /// Collapse `z = (x * #c) * #d` to `z = x * #(c*d)` (C++
    /// `Funcdata::collapseIntMultMult`, `funcdata_op.cc:1138`).  Returns whether a
    /// collapse occurred.
    pub fn collapse_int_mult_mult(&mut self, vn: VarnodeId) -> bool {
        let op = match self.vbank().get(vn).and_then(|v| v.get_def()) {
            Some(o) => o,
            None => return false,
        };
        if self.obank().get(op).map(|o| o.code()) != Some(OpCode::CPUI_INT_MULT) {
            return false;
        }
        let const_first = self.obank().get(op).and_then(|o| o.get_in(1));
        let (cf_is_const, cf_off) = match const_first.and_then(|v| self.vbank().get(v)) {
            Some(v) => (v.is_constant(), v.get_offset()),
            None => return false,
        };
        if !cf_is_const {
            return false;
        }
        let in0 = self.obank().get(op).and_then(|o| o.get_in(0));
        let in0_written = in0.and_then(|v| self.vbank().get(v)).map(|v| v.is_written()).unwrap_or(false);
        if !in0_written {
            return false;
        }
        let other_mult = in0.and_then(|v| self.vbank().get(v)).and_then(|v| v.get_def());
        let other_mult = match other_mult {
            Some(o) => o,
            None => return false,
        };
        if self.obank().get(other_mult).map(|o| o.code()) != Some(OpCode::CPUI_INT_MULT) {
            return false;
        }
        let const_second = self.obank().get(other_mult).and_then(|o| o.get_in(1));
        let (cs_is_const, cs_off) = match const_second.and_then(|v| self.vbank().get(v)) {
            Some(v) => (v.is_constant(), v.get_offset()),
            None => return false,
        };
        if !cs_is_const {
            return false;
        }
        let invn = match self.obank().get(other_mult).and_then(|o| o.get_in(0)) {
            Some(v) => v,
            None => return false,
        };
        let (invn_free, sz) = match self.vbank().get(invn) {
            Some(v) => (v.is_free(), v.get_size()),
            None => return false,
        };
        if invn_free {
            return false;
        }
        let val = (cf_off.wrapping_mul(cs_off)) & kuna_base::address::calc_mask(sz);
        let newvn = self.new_constant(sz, val);
        let _ = self.op_set_input(op, newvn, 1);
        let _ = self.op_set_input(op, invn, 0);
        true
    }

    /// Make a clone of the given op, copying control-flow properties; the
    /// data-type is \e not cloned (C++ `Funcdata::cloneOp`, `funcdata_op.cc:616`).
    ///
    /// Now closed (rport/w10-jts-chain): `cloneVarnode` is the funcdata_varnode
    /// factory and the opcode is resolved through the W6 `inst[]` boundary via
    /// [`op_set_opcode_code`](Funcdata::op_set_opcode_code).
    pub fn clone_op(
        &mut self,
        op: OpId,
        seq: kuna_base::address::SeqNum,
    ) -> KunaResult<OpId> {
        let numinput = self.obank().get(op).expect("clone_op: stale op").num_input();
        let newop = self.new_op_seq(numinput, seq);
        let opc = self.obank().get(op).expect("clone_op").code();
        self.op_set_opcode_code(newop, opc);
        let fl = self.obank().get(op).expect("clone_op").get_flags()
            & (pcodeop_flags::startmark | pcodeop_flags::startbasic);
        self.obank_mut().get_mut(newop).expect("clone_op").set_flag(fl);
        let outvn = self.obank().get(op).expect("clone_op").get_out();
        if let Some(outvn) = outvn {
            let newout = self.clone_varnode(outvn);
            self.op_set_output(newop, newout)?;
        }
        for i in 0..numinput {
            let invn = self
                .obank()
                .get(op)
                .expect("clone_op")
                .get_in(i)
                .expect("clone_op: missing input");
            let newin = self.clone_varnode(invn);
            self.op_set_input(newop, newin, i)?;
        }
        Ok(newop)
    }

    /// Clone an op from a \e different function into \b this one (C++
    /// `Funcdata::cloneOp` where `op` belongs to the source `fd`, called by
    /// `truncatedFlow`): the op-shell/opcode/flags + varnodes are read from `src`
    /// and built into `self`.
    pub fn clone_op_from(
        &mut self,
        src: &Funcdata,
        op: OpId,
        seq: kuna_base::address::SeqNum,
    ) -> KunaResult<OpId> {
        let numinput = src.obank().get(op).expect("clone_op_from: stale src op").num_input();
        let newop = self.new_op_seq(numinput, seq);
        let opc = src.obank().get(op).expect("clone_op_from").code();
        self.op_set_opcode_code(newop, opc);
        let fl = src.obank().get(op).expect("clone_op_from").get_flags()
            & (pcodeop_flags::startmark | pcodeop_flags::startbasic);
        self.obank_mut().get_mut(newop).expect("clone_op_from").set_flag(fl);
        let outvn = src.obank().get(op).expect("clone_op_from").get_out();
        if let Some(outvn) = outvn {
            let newout = self.clone_varnode_from(src, outvn);
            self.op_set_output(newop, newout)?;
        }
        for i in 0..numinput {
            let invn = src
                .obank()
                .get(op)
                .expect("clone_op_from")
                .get_in(i)
                .expect("clone_op_from: missing input");
            let newin = self.clone_varnode_from(src, invn);
            self.op_set_input(newop, newin, i)?;
        }
        Ok(newop)
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
    /// STUB(W6): `opSetOpcode` resolves the new `INT_LESS`/`INT_SLESS` through the
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

        let v = self.vbank().get(vn).expect("replace_lessequal: stale vn");
        let val = v.get_offset();
        let size = v.get_size();
        let code = self.obank().get(op).expect("replace_lessequal").code();
        let newcode = if code == OpCode::CPUI_INT_SLESSEQUAL {
            // Check for signed overflow
            if diff == -1 && val == kuna_base::address::calc_int_min(size) {
                return Ok(false);
            }
            if diff == 1 && val == kuna_base::address::calc_int_max(size) {
                return Ok(false);
            }
            OpCode::CPUI_INT_SLESS
        } else {
            // Check for unsigned overflow
            if diff == -1 && val == 0 {
                return Ok(false);
            }
            if diff == 1 && val == kuna_base::address::calc_uint_max(size) {
                return Ok(false);
            }
            OpCode::CPUI_INT_LESS
        };
        self.op_set_opcode(op, Self::w6_type_op(newcode));

        let res = val.wrapping_add(diff as u64) & kuna_base::address::calc_mask(size);
        let newvn = self.new_constant(size, res);
        // newvn->copySymbol(vn);  -- STUB(W4): type/symbol propagation (see op_set_input).
        // opSetInput(op,newvn,i);
        self.op_set_input(op, newvn, i)?;
        // (kuna) provenance recorder.
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
        let bool_vn = match self.obank().get(cbranch).expect("op_normalize_flip: stale op").get_in(1) {
            Some(v) => v,
            None => return Ok(false),
        };
        if !self.vbank().get(bool_vn).expect("op_normalize_flip: stale boolVn").is_written() {
            return Ok(false);
        }
        if self.lone_descend(bool_vn) != Some(cbranch) {
            return Ok(false);
        }
        let cond_op = self
            .vbank()
            .get(bool_vn)
            .expect("op_normalize_flip: stale boolVn")
            .get_def()
            .expect("op_normalize_flip: written boolVn with no def (C++ UB)");
        let mut flipyes = false;
        let cond_code = self.obank().get(cond_op).expect("op_normalize_flip: stale condOp").code();
        let opc = kuna_num::opcodes::get_booleanflip(cond_code, &mut flipyes);
        if opc == OpCode::CPUI_MAX {
            return Ok(false);
        }
        self.op_set_opcode(cond_op, crate::typeop::type_op_for(opc));
        if flipyes {
            self.op_swap_input(cond_op, 0, 1);
        }
        self.obank_mut()
            .get_mut(cbranch)
            .expect("op_normalize_flip: stale cbranch")
            .flip_flag(pcodeop_flags::boolean_flip);
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
    /// STUB(W6): `opSetOpcode` for the new `INT_MULT`/`INT_ADD` ops is routed
    /// through [`w6_type_op`](Funcdata::w6_type_op) (verbatim typeop.cc opflags).
    pub fn distribute_int_mult_add(&mut self, op: OpId) -> KunaResult<bool> {
        let in0 = self.obank().get(op).expect("distribute_int_mult_add: stale op").get_in(0)
            .expect("distributeIntMultAdd: MULT null in0 (C++ UB)");
        let addop = self.vbank().get(in0).expect("distribute_int_mult_add: stale in0").get_def()
            .expect("distributeIntMultAdd: in0 has no def (C++ UB)");
        let vn0 = self.obank().get(addop).expect("distribute_int_mult_add: stale addop").get_in(0)
            .expect("distributeIntMultAdd: ADD null in0 (C++ UB)");
        let vn1 = self.obank().get(addop).expect("distribute_int_mult_add").get_in(1)
            .expect("distributeIntMultAdd: ADD null in1 (C++ UB)");
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
        let op_in1 = self.obank().get(op).expect("distribute_int_mult_add").get_in(1)
            .expect("distributeIntMultAdd: MULT null in1 (C++ UB)");
        let coeff = self.vbank().get(op_in1).expect("distribute_int_mult_add: stale in1").get_offset();
        let out = self.obank().get(op).expect("distribute_int_mult_add").get_out()
            .expect("distributeIntMultAdd: MULT has no output (C++ UB)");
        let sz = self.vbank().get(out).expect("distribute_int_mult_add: stale out").get_size();
        let mask = kuna_base::address::calc_mask(sz);
        let pc = self.obank().get(op).expect("distribute_int_mult_add").get_addr().clone();

        // Do distribution -- first input.
        let newvn0 = if self.vbank().get(vn0).expect("distribute_int_mult_add: stale vn0").is_constant() {
            let v0off = self.vbank().get(vn0).expect("distribute_int_mult_add").get_offset();
            let val = coeff.wrapping_mul(v0off) & mask;
            self.new_constant(sz, val)
        } else {
            let newop0 = self.new_op(2, pc.clone());
            self.op_set_opcode(newop0, Self::w6_type_op(OpCode::CPUI_INT_MULT));
            let r = self.new_unique_out(sz, newop0)?;
            self.op_set_input(newop0, vn0, 0)?;
            let newcvn = self.new_constant(sz, coeff);
            self.op_set_input(newop0, newcvn, 1)?;
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
    /// STUB(W6): `opSetOpcode(replace, op1->code())` for the freshly built op is
    /// routed through [`w6_type_op`](Funcdata::w6_type_op).
    pub fn cse_elimination(&mut self, op1: OpId, op2: OpId) -> KunaResult<OpId> {
        let par1 = self.obank().get(op1).expect("cse_elimination: stale op1").get_parent()
            .expect("cseElimination: op1 has no parent (C++ UB)");
        let par2 = self.obank().get(op2).expect("cse_elimination: stale op2").get_parent()
            .expect("cseElimination: op2 has no parent (C++ UB)");
        let replace: OpId = if par1 == par2 {
            let o1 = self.obank().get(op1).expect("cse_elimination").get_seq_num().get_order();
            let o2 = self.obank().get(op2).expect("cse_elimination").get_seq_num().get_order();
            if o1 < o2 { op1 } else { op2 }
        } else {
            let common = self.bblocks_mut().find_common_block(par1, par2);
            if common == Some(par1) {
                op1
            } else if common == Some(par2) {
                op2
            } else {
                // Neither op is ancestor of the other -- build a fresh op at common's stop.
                let common = common.expect("cseElimination: no common block (C++ UB)");
                let stop = crate::block::block_get_stop(&self.bblocks_ref().arena, common);
                let numin = self.obank().get(op1).expect("cse_elimination").num_input();
                let replace = self.new_op(numin, stop);
                let code1 = self.obank().get(op1).expect("cse_elimination").code();
                self.op_set_opcode(replace, Self::w6_type_op(code1));
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
                self.op_insert_end(replace, common);
                replace
            }
        };
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

    /// Return a unique-space Varnode defined by a COPY of `vn`, usable at `point`
    /// (C++ `Funcdata::buildCopyTemp`, `funcdata_op.cc:1167`).
    ///
    /// If a usable COPY-to-unique already exists it may be reused (moved earlier
    /// in control flow if needed); otherwise a fresh COPY is inserted before
    /// `point`.  Used by `SplitDatatype::RootPointer::duplicateToTemp`.
    pub fn build_copy_temp(&mut self, vn: VarnodeId, point: OpId) -> KunaResult<VarnodeId> {
        use kuna_base::space::spacetype;
        // Find a preexisting COPY of vn into the unique space (not type-locked).
        let mut other_op: Option<OpId> = None;
        let descend: Vec<OpId> =
            self.vbank().get(vn).expect("build_copy_temp: stale vn").descend_iter().collect();
        for op in descend {
            if self.obank().get(op).expect("build_copy_temp: stale op").code() != OpCode::CPUI_COPY {
                continue;
            }
            let outvn = match self.obank().get(op).expect("build_copy_temp").get_out() {
                Some(o) => o,
                None => continue,
            };
            if self.vbank().get(outvn).expect("build_copy_temp: stale out").get_space().get_type()
                == spacetype::IPTR_INTERNAL
            {
                if self.vbank().get(outvn).expect("build_copy_temp").is_type_lock() {
                    continue;
                }
                other_op = Some(op);
                break;
            }
        }
        let mut used_copy: Option<OpId> = None;
        if let Some(oop) = other_op {
            let point_par = self.obank().get(point).expect("build_copy_temp: stale point").get_parent();
            let other_par = self.obank().get(oop).expect("build_copy_temp: stale other").get_parent();
            if point_par == other_par {
                let porder =
                    self.obank().get(point).expect("build_copy_temp").get_seq_num().get_order();
                let oorder = self.obank().get(oop).expect("build_copy_temp").get_seq_num().get_order();
                used_copy = if porder < oorder { None } else { Some(oop) };
            } else {
                let pp = point_par.expect("build_copy_temp: point has no parent (C++ UB)");
                let op_par = other_par.expect("build_copy_temp: other has no parent (C++ UB)");
                let common = self.bblocks_mut().find_common_block(pp, op_par);
                if common == Some(pp) {
                    used_copy = None;
                } else if common == Some(op_par) {
                    used_copy = Some(oop);
                } else {
                    // Neither op is ancestor of the other -- build a fresh COPY at common's stop.
                    let common = common.expect("build_copy_temp: no common block (C++ UB)");
                    let stop = crate::block::block_get_stop(&self.bblocks_ref().arena, common);
                    let vnsize = self.vbank().get(vn).expect("build_copy_temp: stale vn").get_size();
                    let uc = self.new_op(1, stop);
                    self.op_set_opcode(uc, Self::w6_type_op(OpCode::CPUI_COPY));
                    self.new_unique_out(vnsize, uc)?;
                    self.op_set_input(uc, vn, 0)?;
                    self.op_insert_end(uc, common);
                    used_copy = Some(uc);
                }
            }
        }
        let used_copy = match used_copy {
            Some(uc) => uc,
            None => {
                let point_addr =
                    self.obank().get(point).expect("build_copy_temp: stale point").get_addr().clone();
                let vnsize = self.vbank().get(vn).expect("build_copy_temp: stale vn").get_size();
                let uc = self.new_op(1, point_addr);
                self.op_set_opcode(uc, Self::w6_type_op(OpCode::CPUI_COPY));
                self.new_unique_out(vnsize, uc)?;
                self.op_set_input(uc, vn, 0)?;
                self.op_insert_before(uc, point);
                uc
            }
        };
        if let Some(oop) = other_op {
            if oop != used_copy {
                let from = self.obank().get(oop).expect("build_copy_temp").get_out().expect("other out");
                let to = self.obank().get(used_copy).expect("build_copy_temp").get_out().expect("used out");
                self.total_replace(from, to)?;
                self.op_destroy(oop);
            }
        }
        Ok(self.obank().get(used_copy).expect("build_copy_temp").get_out().expect("used out"))
    }

    /// Find an existing op in block `bl` that is functionally equal to `op`,
    /// reading the given Varnode `vn`, occurring no later than `earliest`
    /// (C++ `Funcdata::cseFindInBlock`, `funcdata_op.cc:1421`).
    ///
    /// Used by [`RuleMultiCollapse`](crate::ruleaction_3::RuleMultiCollapse)'s
    /// functional-equality branch to reuse a copy already built in the common
    /// block instead of duplicating it.
    pub fn cse_find_in_block(
        &self,
        op: OpId,
        vn: VarnodeId,
        bl: crate::context::BlockId,
        earliest: Option<OpId>,
    ) -> Option<OpId> {
        let outvn1 = self.obank().get(op).expect("cse_find_in_block: stale op").get_out();
        let descend: Vec<OpId> =
            self.vbank().get(vn).expect("cse_find_in_block: stale vn").descend_iter().collect();
        let early_order = earliest.map(|e| {
            self.obank().get(e).expect("cse_find_in_block: stale earliest").get_seq_num().get_order()
        });
        for res in descend {
            if res == op {
                continue; // Must not be -op-
            }
            let res_parent =
                self.obank().get(res).expect("cse_find_in_block: stale res").get_parent();
            if res_parent != Some(bl) {
                continue; // Must be in -bl-
            }
            if let Some(early_order) = early_order {
                let res_order =
                    self.obank().get(res).expect("cse_find_in_block").get_seq_num().get_order();
                if early_order < res_order {
                    continue; // Must occur earlier than -earliest-
                }
            }
            let outvn2 = self.obank().get(res).expect("cse_find_in_block").get_out();
            let outvn2 = match outvn2 {
                Some(v) => v,
                None => continue,
            };
            let outvn1 = match outvn1 {
                Some(v) => v,
                None => continue,
            };
            let mut buf1: [VarnodeId; 2] = [outvn1, outvn1];
            let mut buf2: [VarnodeId; 2] = [outvn2, outvn2];
            if crate::expression::functional_equality_level(
                outvn1,
                outvn2,
                &mut buf1,
                &mut buf2,
                self.vbank(),
                self.obank(),
            ) == 0
            {
                return Some(res);
            }
        }
        None
    }

    /// The earliest op (by within-block order) in block `bl` that reads `vn`
    /// (C++ `BlockBasic::earliestUse`, `block.cc:2826`).  Returns `None` if no
    /// descendant of `vn` lies in `bl`.
    pub fn block_earliest_use(&self, bl: crate::context::BlockId, vn: VarnodeId) -> Option<OpId> {
        let descend: Vec<OpId> =
            self.vbank().get(vn).expect("block_earliest_use: stale vn").descend_iter().collect();
        let mut res: Option<OpId> = None;
        for op in descend {
            if self.obank().get(op).expect("block_earliest_use: stale op").get_parent() != Some(bl) {
                continue;
            }
            match res {
                None => res = Some(op),
                Some(cur) => {
                    let op_order =
                        self.obank().get(op).expect("block_earliest_use").get_seq_num().get_order();
                    let cur_order =
                        self.obank().get(cur).expect("block_earliest_use").get_seq_num().get_order();
                    if op_order < cur_order {
                        res = Some(op);
                    }
                }
            }
        }
        res
    }

    /// Test whether op `this_op` can be moved to occur immediately after op
    /// `point` (within the same basic block), keeping equivalent data-flow
    /// (C++ `PcodeOp::isMoveable`, `op.cc:201`).
    ///
    /// Ported as a `Funcdata` method because the C++ `PcodeOp` method walks both
    /// the output's descendant list and the basic-block intrusive op list, which
    /// in the rust port live on the banks owned by `Funcdata`.
    pub fn op_is_moveable(&self, this_op: OpId, point: OpId) -> bool {
        use crate::op::pcodeop_flags;
        if this_op == point {
            return true; // No movement necessary
        }
        let this = self.obank().get(this_op).expect("op_is_moveable: stale op");
        let mut moving_load = false;
        if this.get_eval_type() == pcodeop_flags::special {
            if this.code() == OpCode::CPUI_LOAD {
                moving_load = true; // Allow LOAD to be moved with extra restrictions
            } else {
                return false; // Don't move special ops
            }
        }
        let this_parent = this.get_parent();
        let point_parent =
            self.obank().get(point).expect("op_is_moveable: stale point").get_parent();
        if this_parent != point_parent {
            return false; // Not in the same block
        }
        let this_out = this.get_out();
        let point_order =
            self.obank().get(point).expect("op_is_moveable").get_seq_num().get_order();
        // Output cannot be moved past an op that reads it.
        if let Some(outvn) = this_out {
            let descend: Vec<OpId> = self
                .vbank()
                .get(outvn)
                .expect("op_is_moveable: stale out")
                .descend_iter()
                .collect();
            for read_op in descend {
                let r = self.obank().get(read_op).expect("op_is_moveable: stale read");
                if r.get_parent() != this_parent {
                    continue;
                }
                if r.get_seq_num().get_order() <= point_order {
                    return false; // Is in the block and is read before (or at) -point-
                }
            }
        }
        // Only allow a move across a CALL in very restrictive circumstances.
        let mut cross_calls = false;
        let this = self.obank().get(this_op).expect("op_is_moveable");
        let num_input = this.num_input();
        if this.get_eval_type() != pcodeop_flags::special {
            // Normal op where all inputs and output are not address tied.
            if let Some(outvn) = this_out {
                let ov = self.vbank().get(outvn).expect("op_is_moveable: out");
                if !ov.is_addr_tied() && !ov.is_persist() {
                    let mut i = 0;
                    while i < num_input {
                        let vn = self.obank().get(this_op).unwrap().get_in(i).unwrap();
                        let v = self.vbank().get(vn).expect("op_is_moveable: in");
                        if v.is_addr_tied() || v.is_persist() {
                            break;
                        }
                        i += 1;
                    }
                    if i == num_input {
                        cross_calls = true;
                    }
                }
            }
        }
        let mut tied_list: Vec<VarnodeId> = Vec::new();
        for i in 0..num_input {
            let vn = self.obank().get(this_op).unwrap().get_in(i).unwrap();
            if self.vbank().get(vn).expect("op_is_moveable: in").is_addr_tied() {
                tied_list.push(vn);
            }
        }
        // Walk the basic-block op list forward from `this_op` up to and
        // including `point` (C++ `do { ++biter; ... } while(biter != point)`).
        let mut biter = this_op;
        loop {
            biter = match self
                .obank()
                .get(biter)
                .expect("op_is_moveable: stale biter")
                .basic_neighbours()
                .1
            {
                Some(nx) => nx,
                None => return false, // ran off the end before reaching point (C++ UB; bail)
            };
            let op_ref = self.obank().get(biter).expect("op_is_moveable: stale op");
            let op_out = op_ref.get_out();
            if op_ref.get_eval_type() == pcodeop_flags::special {
                match op_ref.code() {
                    OpCode::CPUI_LOAD => {
                        if let Some(o) = this_out {
                            if self.vbank().get(o).unwrap().is_addr_tied() {
                                return false;
                            }
                        }
                    }
                    OpCode::CPUI_STORE => {
                        if moving_load {
                            return false;
                        } else {
                            if !tied_list.is_empty() {
                                return false;
                            }
                            if let Some(o) = this_out {
                                if self.vbank().get(o).unwrap().is_addr_tied() {
                                    return false;
                                }
                            }
                        }
                    }
                    OpCode::CPUI_INDIRECT | OpCode::CPUI_SEGMENTOP | OpCode::CPUI_CPOOLREF => {
                        // Let thru, deal with what's INDIRECTed around separately
                    }
                    OpCode::CPUI_CALL | OpCode::CPUI_CALLIND | OpCode::CPUI_NEW => {
                        if !cross_calls {
                            return false;
                        }
                    }
                    _ => return false,
                }
            }
            if let Some(out2) = op_out {
                if moving_load && self.vbank().get(out2).unwrap().is_addr_tied() {
                    return false;
                }
                for &tvn in &tied_list {
                    let tv = self.vbank().get(tvn).expect("op_is_moveable: tied");
                    let o2 = self.vbank().get(out2).expect("op_is_moveable: out2");
                    if tv.overlap(o2) >= 0 {
                        return false;
                    }
                    if o2.overlap(tv) >= 0 {
                        return false;
                    }
                }
            }
            if biter == point {
                break;
            }
        }
        true
    }

    /// The HighVariable of `vn` (C++ `vn->getHigh()`), via the public banks so
    /// the for-loop helpers below need no access to the merge-bank private API.
    fn vn_high_pub(&self, vn: VarnodeId) -> Option<crate::context::HighVariableId> {
        self.vbank().get(vn).and_then(|v| v.get_high())
    }
    /// `high->isMark()` via the public high bank.
    fn high_is_mark_pub(&self, high: crate::context::HighVariableId) -> bool {
        self.high_bank().get(high).map(|h| h.is_mark()).unwrap_or(false)
    }
    /// `high->setMark()` via the public high bank.
    fn bank_set_mark_pub(&mut self, high: crate::context::HighVariableId) {
        if let Some(h) = self.high_bank_mut().get_mut(high) {
            h.set_mark();
        }
    }
    /// `high->clearMark()` via the public high bank.
    fn bank_clear_mark_pub(&mut self, high: crate::context::HighVariableId) {
        if let Some(h) = self.high_bank_mut().get_mut(high) {
            h.clear_mark();
        }
    }

    /// Mark all the \e explicit HighVariables in the expression rooted at `vn`
    /// and append them to `high_list` (C++ `HighVariable::markExpression`,
    /// `variable.cc:885`).  Returns a value: `1` if a call op is in the
    /// expression, `2` if a LOAD, `3` for both, `0` otherwise.
    ///
    /// Ported here (not in the concurrent-wave-owned `variable.rs`) as a
    /// `Funcdata` helper for [`op_move_respecting_cover`]; it touches the
    /// HighVariable mark bit only through the public high bank.
    fn high_mark_expression(
        &mut self,
        vn: VarnodeId,
        high_list: &mut Vec<crate::context::HighVariableId>,
    ) -> int4 {
        let high = self.vn_high_pub(vn).expect("high_mark_expression: vn has no high");
        self.bank_set_mark_pub(high);
        high_list.push(high);
        let mut ret_val: int4 = 0;
        if !self.vbank().get(vn).expect("high_mark_expression: stale vn").is_written() {
            return ret_val;
        }
        // path of (op, slot) pairs (C++ vector<PcodeOpNode>).
        let mut path: Vec<(OpId, int4)> = Vec::new();
        let op = self.vbank().get(vn).unwrap().get_def().unwrap();
        if self.obank().get(op).expect("high_mark_expression: stale op").is_call() {
            ret_val |= 1;
        }
        if self.obank().get(op).unwrap().code() == OpCode::CPUI_LOAD {
            ret_val |= 2;
        }
        path.push((op, 0));
        while let Some(&(cur_op, slot)) = path.last() {
            let num_in =
                self.obank().get(cur_op).expect("high_mark_expression: node op").num_input();
            if num_in <= slot {
                path.pop();
                continue;
            }
            let cur_vn = self.obank().get(cur_op).unwrap().get_in(slot);
            path.last_mut().unwrap().1 += 1;
            let cur_vn = match cur_vn {
                Some(v) => v,
                None => continue,
            };
            let cv = self.vbank().get(cur_vn).expect("high_mark_expression: stale curvn");
            if cv.is_annotation() {
                continue;
            }
            if cv.is_explicit() {
                let h = self.vn_high_pub(cur_vn).expect("high_mark_expression: explicit high");
                if self.high_is_mark_pub(h) {
                    continue; // Already in the list
                }
                self.bank_set_mark_pub(h);
                high_list.push(h);
                continue; // Truncate at explicit
            }
            if !cv.is_written() {
                continue;
            }
            let def = self.vbank().get(cur_vn).unwrap().get_def().unwrap();
            if self.obank().get(def).expect("high_mark_expression: def op").is_call() {
                ret_val |= 1;
            }
            if self.obank().get(def).unwrap().code() == OpCode::CPUI_LOAD {
                ret_val |= 2;
            }
            path.push((def, 0));
        }
        ret_val
    }

    /// Determine if `op` can be moved (within its basic block) to after
    /// `last_op`, respecting HighVariable covers; perform the move if possible
    /// (C++ `Funcdata::moveRespectingCover`, `funcdata_op.cc:1498`).  Returns
    /// `true` if the move is possible (and has been performed).
    pub fn op_move_respecting_cover(&mut self, op: OpId, last_op: OpId) -> bool {
        if op == last_op {
            return true; // Nothing to move past
        }
        if self.obank().get(op).expect("op_move_respecting_cover: stale op").is_call() {
            return false;
        }
        let mut prev_op: Option<OpId> = None;
        if self.obank().get(op).unwrap().code() == OpCode::CPUI_CAST {
            let vn = self.obank().get(op).unwrap().get_in(0).expect("CAST in0");
            let cv = self.vbank().get(vn).expect("op_move_respecting_cover: cast in");
            if !cv.is_explicit() {
                // If CAST is part of expression, we need to move the previous op too.
                if !cv.is_written() {
                    return false;
                }
                let p = self.vbank().get(vn).unwrap().get_def().unwrap();
                if self.obank().get(p).expect("op_move_respecting_cover: prev").is_call() {
                    return false;
                }
                // Previous op must exist and feed into the CAST.
                if self.op_previous_op(op) != Some(p) {
                    return false;
                }
                prev_op = Some(p);
            }
        }
        let rootvn =
            self.obank().get(op).unwrap().get_out().expect("op_move_respecting_cover: out");
        let mut high_list: Vec<crate::context::HighVariableId> = Vec::new();
        let type_val = self.high_mark_expression(rootvn, &mut high_list);
        let mut cur_op = op;
        loop {
            let next_op = self.op_next_op(cur_op).expect("op_move_respecting_cover: nextOp");
            let opc = self.obank().get(next_op).expect("op_move_respecting_cover: next").code();
            if opc != OpCode::CPUI_COPY && opc != OpCode::CPUI_CAST {
                break; // Limit to crossing only COPY and CAST ops
            }
            if Some(rootvn) == self.obank().get(next_op).unwrap().get_in(0) {
                break; // Data-flow order dependence
            }
            let copy_vn = self.obank().get(next_op).unwrap().get_out().expect("copy out");
            let copy_high = self.vn_high_pub(copy_vn).expect("copy high");
            if self.high_is_mark_pub(copy_high) {
                break; // Direct interference: COPY writes what original op reads
            }
            if type_val != 0 && self.vbank().get(copy_vn).unwrap().is_addr_tied() {
                break; // Possible indirect interference
            }
            cur_op = next_op;
            if cur_op == last_op {
                break;
            }
        }
        // Clear marks on the expression.
        for h in high_list {
            self.bank_clear_mark_pub(h);
        }
        if cur_op == last_op {
            // We were able to cross everything: move -op- (and any CAST prev).
            self.op_uninsert(op);
            self.op_insert_after(op, last_op);
            if let Some(p) = prev_op {
                self.op_uninsert(p);
                self.op_insert_after(p, last_op);
            }
            return true;
        }
        false
    }

    /// Perform CSE on a list of `(hash, op)` pairs (descendants of one Varnode)
    /// (C++ `Funcdata::cseEliminateList`, `funcdata_op.cc:1459`).
    ///
    /// The list is `stable_sort`ed by hash (the verbatim `compareCseHash`
    /// comparator); equal-hash neighbours that are a true `isCseMatch` and whose
    /// outputs are both heritaged (or null) are eliminated, the surviving output
    /// pushed onto `outlist`.
    ///
    /// STUB(W7): the C++ `isHeritaged(outvn)` is `heritage.heritagePass(addr)>=0`
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
        if list.is_empty() {
            return Ok(());
        }
        // Sort by hash (.first).
        list.sort_by(|a, b| a.0.cmp(&b.0));
        let mut idx = 1;
        while idx < list.len() {
            let (h1, op1) = list[idx - 1];
            let (h2, op2) = list[idx];
            if h1 == h2 {
                let dead1 = self.obank().get(op1).expect("cse_eliminate_list: stale op1").is_dead();
                let dead2 = self.obank().get(op2).expect("cse_eliminate_list: stale op2").is_dead();
                if !dead1 && !dead2 && self.op_is_cse_match(op1, op2) {
                    let outvn1 = self.obank().get(op1).expect("cse_eliminate_list").get_out();
                    let outvn2 = self.obank().get(op2).expect("cse_eliminate_list").get_out();
                    let h1ok = match outvn1 {
                        None => true,
                        Some(v) => is_heritaged(self, v),
                    };
                    let h2ok = match outvn2 {
                        None => true,
                        Some(v) => is_heritaged(self, v),
                    };
                    if h1ok && h2ok {
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

    /// Resolve an [`OpCode`] to its [`TypeOp`] via the `glb->inst[opc]` table
    /// (C++ `Funcdata::opSetOpcode` does `obank.changeOpcode(op, glb->inst[opc])`).
    ///
    /// The handful of op-codes the funcdata helpers above install keep their
    /// original short display names (`<`, `+`, `*`, `copy`, `SUB`, `s>>`) for
    /// provenance; **every other** op-code resolves through the canonical
    /// `typeop::type_op_for` table (the verbatim `typeop.cc` `opflags`), so a
    /// cloned op — e.g. a `truncatedFlow` `INT_EQUAL`/`BOOL_OR` in the jump-table
    /// partial — reports its real `binary`/`booloutput`/`commutative` eval-type
    /// bits.  Without this, `INT_EQUAL`'s output was not flagged `booloutput`, so
    /// `RuleRangeMeld` (and any bool-output structural test) skipped the cloned
    /// guard and the switch index range never narrowed.
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
            // TypeOpPiece (typeop.cc:2039): binary; rendered "CONCAT".  Used by
            // RulePieceStructure::convertZextToPiece (INT_ZEXT -> PIECE).  Kept as
            // an explicit arm for provenance (the canonical table below would
            // classify it identically as `binary`).
            OpCode::CPUI_PIECE => (f::binary, "CONCAT"),
            // The full `glb->inst[opc]` table (verbatim typeop.cc opflags) for
            // every other op-code, so cloned/synthesised ops carry their true
            // eval-type bits (load-bearing for `booloutput` consumers).
            _ => return crate::typeop::type_op_for(opc),
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
    pub fn op_insert(&mut self, op: OpId, bl: crate::context::BlockId, before: Option<OpId>) {
        self.obank_mut().mark_alive(op);
        self.bb_insert_op(op, bl, before);
    }

    /// Remove the op from its basic block and return it to the dead list
    /// (C++ `Funcdata::opUninsert`, `funcdata_op.cc:164`).
    ///
    /// `obank.markDead(op); op->getParent()->removeOp(op);`
    pub fn op_uninsert(&mut self, op: OpId) {
        self.obank_mut().mark_dead(op);
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
        self.op_unset_output(op);
        let n = self.obank().get(op).expect("op_unlink: stale op").num_input();
        for i in 0..n {
            self.op_unset_input(op, i);
        }
        if self.obank().get(op).expect("op_unlink").get_parent().is_some() {
            self.op_uninsert(op);
        }
    }

    /// Destroy the given op: unset its output/inputs and move it permanently to
    /// the dead list (C++ `Funcdata::opDestroy`, `funcdata_op.cc:203`).
    ///
    /// Faithful: `destroyVarnode(op->getOut())` frees the output Varnode (now
    /// that `destroy_varnode` is ported), which also purges it from its
    /// HighVariable's instance list (the `~Varnode` `high->remove` step) so a
    /// later naming pass never derefs a freed member.  Then the input links are
    /// unset, the op is marked dead and removed from its block.
    pub fn op_destroy(&mut self, op: OpId) {
        if let Some(out) = self.obank().get(op).expect("op_destroy: stale op").get_out() {
            // destroyVarnode clears each reader's input, nulls out->def (and the
            // op's output back-link), then frees the varnode — purging it from
            // its HighVariable on the way (the ~Varnode high->remove step).
            self.destroy_varnode(out).expect("op_destroy: destroyVarnode(out)");
        }
        let n = self.obank().get(op).expect("op_destroy").num_input();
        for i in 0..n {
            if self.obank().get(op).expect("op_destroy").get_in(i).is_some() {
                self.op_unset_input(op, i);
            }
        }
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
                if !v.is_written() || v.is_auto_live() {
                    continue;
                }
                if self.vn_lone_descend(vn).is_none() {
                    continue;
                }
                let def_op = v.get_def().expect("op_destroy_recursive: written vn has no def");
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
    /// Ported faithfully now that
    /// [`destroy_varnode`](Funcdata::destroy_varnode) (funcdata_varnode) is ported:
    /// destroy every input Varnode, then the output if present, then the op
    /// (`obank.destroy(op)`; the op must be dead).  Statement order matches the
    /// C++ exactly (inputs first, then output, then the op).
    pub fn op_destroy_raw(&mut self, op: OpId) -> KunaResult<()> {
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
        if let Some(out) = self.obank().get(op).expect("op_destroy_raw: stale op").get_out() {
            self.destroy_varnode(out)?;
        }
        // The op must be dead; the bank panics otherwise.
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
        let parent = self
            .obank()
            .get(follow)
            .expect("op_insert_before: stale follow")
            .get_parent()
            .expect("op_insert_before: follow has no parent (C++ UB)");
        // The C++ `iter` starts at `follow`; we track the insertion point as the
        // op we will insert *before* (the dereferenced iterator), `point`.
        let mut point: Option<OpId> = Some(follow);

        // If op is not an INDIRECT, skip back over preceding INDIRECTs.
        if self.obank().get(op).expect("op_insert_before: stale op").code()
            != OpCode::CPUI_INDIRECT
        {
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
                    // Insertion point stays at `point`.
                    break;
                }
                // prev was INDIRECT: keep walking back; iter now at previousop.
                point = Some(previousop);
            }
        }
        self.op_insert(op, parent, point);
    }

    /// Insert the given op immediately after `prev`, honoring the SSA op ordering
    /// invariants (C++ `Funcdata::opInsertAfter`, `funcdata_op.cc:373`).
    ///
    /// STUB(W3-varnode): the INDIRECT-marker redirect
    /// decodes an [`OpId`] from the iop-space address offset of the INDIRECT's
    /// second input (`getOpFromConst`).  That iop encoding is established by the
    /// funcdata_varnode `newVarnodeIop` factory (a sibling parallel item) and is
    /// not available here; the redirect is skipped (treated as a no-op marker
    /// input), so when `prev` is an INDIRECT with an iop input the op is inserted
    /// after the INDIRECT itself rather than after the INDIRECT's target.
    /// Recorded as a loss; the funcdata_varnode wave supplies the decode.
    pub fn op_insert_after(&mut self, op: OpId, prev: OpId) {
        let mut prev = prev;
        // STUB(W3-varnode): getOpFromConst decode deferred (see doc).
        if self.obank().get(prev).expect("op_insert_after: stale prev").is_marker()
            && self.obank().get(prev).expect("op_insert_after").code() == OpCode::CPUI_INDIRECT
        {
            // The iop redirect is the deferred part; without it `prev` is unchanged.
            // (Faithful for every non-iop INDIRECT input, which is the common case
            // until the iop factory lands.)
        }

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

        // If op is not a MULTIEQUAL, skip forward over MULTIEQUALs.
        if self.obank().get(op).expect("op_insert_after: stale op").code()
            != OpCode::CPUI_MULTIEQUAL
        {
            // The loop ends when iter==endOp() (point==None); the explicit `break`
            // fires on the first non-MULTIEQUAL, leaving `point` at that op
            // (the C++ `--iter`).
            while let Some(nextop) = point {
                // ++iter first, then test.
                let after = self.obank().get(nextop).expect("op_insert_after").basic_neighbours().1;
                if self.obank().get(nextop).expect("op_insert_after").code()
                    != OpCode::CPUI_MULTIEQUAL
                {
                    // Insertion point is `nextop`.
                    break;
                }
                // nextop was MULTIEQUAL: advance.
                point = after;
            }
        }
        self.op_insert(op, parent, point);
    }

    /// Construct the boolean negation of a given boolean Varnode into a temporary
    /// register (C++ `Funcdata::opBoolNegate`, `funcdata_op.cc:560`).
    ///
    /// Creates a `BOOL_NEGATE` of `vn`, inserts it before/after `op`
    /// (per `insertafter`), and returns the size-1 result Varnode.  The output
    /// is a freshly created unique with no readers, so the `newUniqueOut`
    /// def-rewiring repoints nothing.
    pub fn op_bool_negate(
        &mut self,
        vn: VarnodeId,
        op: OpId,
        insertafter: bool,
    ) -> KunaResult<VarnodeId> {
        let addr = self.obank().get(op).expect("opBoolNegate: stale op").get_addr().clone();
        let negateop = self.new_op(1, addr);
        self.op_set_opcode_code(negateop, OpCode::CPUI_BOOL_NEGATE);
        let resvn = self.new_unique_out(1, negateop)?;
        self.op_set_input(negateop, vn, 0)?;
        if insertafter {
            self.op_insert_after(negateop, op);
        } else {
            self.op_insert_before(negateop, op);
        }
        Ok(resvn)
    }

    /// Insert the given op as the \e first op in the basic block, honoring the
    /// MULTIEQUAL-first invariant (C++ `Funcdata::opInsertBegin`,
    /// `funcdata_op.cc:413`).
    pub fn op_insert_begin(&mut self, op: OpId, bl: crate::context::BlockId) {
        let mut point: Option<OpId> = self.bb_op_head(bl);
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
        self.op_insert(op, bl, point);
    }

    /// Insert the given op as the \e last op in the basic block, honoring the
    /// branch-last invariant (C++ `Funcdata::opInsertEnd`, `funcdata_op.cc:435`).
    pub fn op_insert_end(&mut self, op: OpId, bl: crate::context::BlockId) {
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
                let vn = o.get_in(1).expect("opFlipInPlaceTest: CBRANCH null in1 (C++ UB)");
                if self.vn_lone_descend(vn) != Some(op) {
                    return 2;
                }
                let v = self.vbank().get(vn).expect("opFlipInPlaceTest: stale vn");
                if !v.is_written() {
                    return 2;
                }
                let def = v.get_def().expect("opFlipInPlaceTest: written vn has no def");
                let mut subtest1 = self.op_flip_in_place_test(def, fliplist, allow_op_removal);
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
                let vn = o.get_in(0).expect("opFlipInPlaceTest: LESS null in0 (C++ UB)");
                fliplist.push(op);
                if !self.vbank().get(vn).expect("opFlipInPlaceTest: stale vn").is_constant() {
                    1
                } else {
                    0
                }
            }
            OpCode::CPUI_INT_SLESSEQUAL | OpCode::CPUI_INT_LESSEQUAL => {
                let vn = o.get_in(1).expect("opFlipInPlaceTest: LESSEQUAL null in1 (C++ UB)");
                fliplist.push(op);
                if self.vbank().get(vn).expect("opFlipInPlaceTest: stale vn").is_constant() {
                    1
                } else {
                    0
                }
            }
            OpCode::CPUI_BOOL_OR | OpCode::CPUI_BOOL_AND => {
                // in0: loneDescend/isWritten checks, then recurse.
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
                // in1: same checks, then recurse.
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
                // Front of AND/OR must be normalizing.
                subtest1
            }
            _ => 2,
        }
    }

    /// Perform the op-code flips computed by [`op_flip_in_place_test`] in place
    /// (C++ `Funcdata::opFlipInPlaceExecute`, `funcdata_op.cc:1296`).
    ///
    /// For each op in `fliplist`, resolve its boolean-flipped op-code via
    /// `get_booleanflip` (the W6 op-code table, now [`crate::typeop::type_op_for`])
    /// and dispatch:
    ///   * `CPUI_COPY` — the op is a `BOOL_NEGATE` being removed: propagate its
    ///     input into the lone descendant in the negate's output slot, then destroy.
    ///   * `CPUI_MAX` — swap `BOOL_AND`↔`BOOL_OR` in place.
    ///   * otherwise — set the flipped op-code; when `flipyes`, swap operands and
    ///     (for `<=`/`s<=`) fold the constant through [`replace_lessequal`].
    pub fn op_flip_in_place_execute(&mut self, fliplist: &[OpId]) -> KunaResult<()> {
        for &op in fliplist {
            let mut flipyes = false;
            let code = self.obank().get(op).expect("op_flip_in_place_execute: stale op").code();
            let opc = kuna_num::opcodes::get_booleanflip(code, &mut flipyes);
            if opc == OpCode::CPUI_COPY {
                // We remove this (CPUI_BOOL_NEGATE) entirely.
                let vn = self
                    .obank()
                    .get(op)
                    .expect("op_flip_in_place_execute: stale op")
                    .get_in(0)
                    .expect("opFlipInPlaceExecute: BOOL_NEGATE null in0 (C++ UB)");
                let out_vn = self
                    .obank()
                    .get(op)
                    .expect("op_flip_in_place_execute: stale op")
                    .get_out()
                    .expect("opFlipInPlaceExecute: BOOL_NEGATE null out (C++ UB)");
                let otherop = self
                    .vn_lone_descend(out_vn)
                    .expect("opFlipInPlaceExecute: BOOL_NEGATE out not a lone descendant (C++ UB)");
                let slot = self.obank().get(otherop).expect("op_flip_in_place_execute: stale otherop").get_slot(out_vn);
                // Propagate -vn- into otherop.
                self.op_set_input(otherop, vn, slot)?;
                self.op_destroy(op);
            } else if opc == OpCode::CPUI_MAX {
                // Swap BOOL_AND <-> BOOL_OR.
                if code == OpCode::CPUI_BOOL_AND {
                    self.op_set_opcode_code(op, OpCode::CPUI_BOOL_OR);
                } else if code == OpCode::CPUI_BOOL_OR {
                    self.op_set_opcode_code(op, OpCode::CPUI_BOOL_AND);
                } else {
                    return Err(KunaError::lowlevel("Bad flipInPlace op"));
                }
            } else {
                self.op_set_opcode(op, crate::typeop::type_op_for(opc));
                if flipyes {
                    self.op_swap_input(op, 0, 1);
                    if opc == OpCode::CPUI_INT_LESSEQUAL || opc == OpCode::CPUI_INT_SLESSEQUAL {
                        self.replace_lessequal(op)?;
                    }
                }
            }
        }
        Ok(())
    }

    // -----------------------------------------------------------------------
    // queries / scans
    // -----------------------------------------------------------------------

    /// Return the first non-dead, non-artificial-halt CPUI_RETURN op, or `None`
    /// (C++ `Funcdata::getFirstReturnOp`, `funcdata_op.cc:632`).
    pub fn get_first_return_op(&self) -> Option<OpId> {
        for retop in self.obank().iter_code(OpCode::CPUI_RETURN) {
            let r = self.obank().get(retop).expect("get_first_return_op: stale op");
            if r.is_dead() {
                continue;
            }
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
                        // Not an internal branch.
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

    /// Override the control-flow p-code op at \b addr (C++ `Funcdata::overrideFlow`,
    /// `funcdata_op.cc:969`).  Used by `FlowInfo::process` when the per-function
    /// `Override` set a flow override at this address (`override flow ...`).
    ///
    /// `type_` is the [`flow_type`](crate::overrides::flow_type) constant
    /// (BRANCH/CALL/CALL_RETURN/RETURN).  Errors `"Could not apply flowoverride"`
    /// when no matching primary branch exists (or it is not dead), matching the C++.
    pub fn override_flow(&mut self, addr: &kuna_base::address::Address, type_: uint4) -> KunaResult<()> {
        use crate::overrides::flow_type;
        let ops: Vec<OpId> = self.obank().iter_at(addr).map(|(_, id)| id).collect();
        let op = if type_ == flow_type::BRANCH {
            self.find_primary_branch(&ops, false, true, true)
        } else if type_ == flow_type::CALL {
            self.find_primary_branch(&ops, true, false, true)
        } else if type_ == flow_type::CALL_RETURN {
            self.find_primary_branch(&ops, true, true, true)
        } else if type_ == flow_type::RETURN {
            self.find_primary_branch(&ops, true, true, false)
        } else {
            None
        };
        let op = match op {
            Some(op) if self.obank().get(op).map(|o| o.is_dead()).unwrap_or(false) => op,
            _ => return Err(KunaError::lowlevel("Could not apply flowoverride")),
        };
        let opc = self.obank().get(op).expect("override_flow: stale op").code();
        if type_ == flow_type::BRANCH {
            match opc {
                OpCode::CPUI_CALL => self.op_set_opcode_code(op, OpCode::CPUI_BRANCH),
                OpCode::CPUI_CALLIND => self.op_set_opcode_code(op, OpCode::CPUI_BRANCHIND),
                OpCode::CPUI_RETURN => self.op_set_opcode_code(op, OpCode::CPUI_BRANCHIND),
                _ => {}
            }
        } else if type_ == flow_type::CALL || type_ == flow_type::CALL_RETURN {
            match opc {
                OpCode::CPUI_BRANCH => self.op_set_opcode_code(op, OpCode::CPUI_CALL),
                OpCode::CPUI_BRANCHIND => self.op_set_opcode_code(op, OpCode::CPUI_CALLIND),
                OpCode::CPUI_CBRANCH => {
                    return Err(KunaError::lowlevel("Do not currently support CBRANCH overrides"))
                }
                OpCode::CPUI_RETURN => self.op_set_opcode_code(op, OpCode::CPUI_CALLIND),
                _ => {}
            }
            if type_ == flow_type::CALL_RETURN {
                // Insert a new return op after the call.
                let new_return = self.new_op(1, addr.clone());
                self.op_set_opcode_code(new_return, OpCode::CPUI_RETURN);
                let zero = self.new_constant(1, 0);
                self.op_set_input(new_return, zero, 0)?;
                self.op_dead_insert_after(new_return, op);
            }
        } else if type_ == flow_type::RETURN {
            match opc {
                OpCode::CPUI_BRANCH | OpCode::CPUI_CBRANCH | OpCode::CPUI_CALL => {
                    return Err(KunaError::lowlevel("Do not currently support complex overrides"))
                }
                OpCode::CPUI_BRANCHIND => self.op_set_opcode_code(op, OpCode::CPUI_RETURN),
                OpCode::CPUI_CALLIND => self.op_set_opcode_code(op, OpCode::CPUI_RETURN),
                _ => {}
            }
        }
        Ok(())
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
        let mut p = self.obank().get(op).expect("op_next_op: stale op").get_parent()?;
        // `iter++` past `op`: the successor in p's op list.
        let mut iter: Option<OpId> =
            self.obank().get(op).expect("op_next_op").basic_neighbours().1;
        // Loop while iter == endOp() (`None`).
        while iter.is_none() {
            let so = self.bblocks_ref().block(p).size_out();
            if so != 1 && so != 2 {
                return None;
            }
            p = self.bblocks_ref().block(p).get_out(0);
            iter = self.bb_op_head(p);
        }
        iter
    }

    /// Return the previous op within this op's basic block, or `None`
    /// (C++ `PcodeOp::previousOp`, `op.cc:367`).  Does not search past the block.
    pub fn op_previous_op(&self, op: OpId) -> Option<OpId> {
        self.obank().get(op).expect("op_previous_op: stale op").basic_neighbours().0
    }

    /// Return the instruction-start op for the instruction containing `op`
    /// (C++ `PcodeOp::target`, `op.cc:383`).
    ///
    /// Scans backward (over the dead-list `insertiter` if the op is dead, else
    /// the block `basiciter`) to the first op with the `startmark` flag.  Works
    /// before blocks are calculated (all ops still on the dead list).
    pub fn op_target(&self, op: OpId) -> OpId {
        let dead = self.obank().get(op).expect("op_target: stale op").is_dead();
        let mut retop = op;
        while (self.obank().get(retop).expect("op_target").get_flags() & pcodeop_flags::startmark)
            == 0
        {
            retop = if dead {
                self.obank()
                    .dead_prev(retop)
                    .expect("op_target: walked past dead-list begin (C++ UB)")
            } else {
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
