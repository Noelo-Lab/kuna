//! Port of `decompiler/cpp/ruleaction.cc` lines 4293-5526 (W5, item
//! `w5-s3-rules-4`): the 19 simplification [`Rule`]s `RuleLoadVarnode` ..
//! `RuleCondNegate`.
//!
//! Each rule is transcribed statement-for-statement from the upstream
//! `applyOp`/`getOpList` bodies — the rule bodies *are* the decompiler's output,
//! so iteration order, tie-breakers, and the exact sequence of graph mutations
//! are load-bearing (a transposed condition changes datatest text).  The C++
//! semantic comments are carried verbatim.
//!
//! ## Registration (W8 reads [`specs`])
//!
//! [`specs`] lists every rule in **C++ definition order** (the order the classes
//! appear in `ruleaction.cc`), so the W8 `universalAction` builder can splice
//! these into the right [`ActionPool`]s.  Each [`RuleSpec`]'s `group` is the
//! stage group the rule belongs to; see `docs/stage-mapping.md`.  17 of these
//! rules live in the `"analysis"` group; `RuleLoadVarnode`/`RuleStoreVarnode`
//! are the exceptions — the C++ registers them in `actprop2` under the
//! `"stackvars"` group (`coreaction.cc:5939-5940`).
//!
//! ## Cross-wave seams
//!
//! The W3 IR data-model that these rules drive is itself mid-port; several
//! `Funcdata` methods the upstream bodies call are not yet available to this
//! parallel item (it owns only `ruleaction_4.rs`).  Where a method is missing the
//! body is still transcribed and the missing call is routed through a local
//! `// STUB`-noted shim; the affected rules are listed in this item's losses.
//! The notable seams:
//!
//!   - **`opSetOpcode(op, OpCode)`** resolves `glb->inst[opc]` (the W6 `TypeOp`
//!     table) to cache the op's property flags.  That table is W6's; [`set_opcode`]
//!     builds a minimal [`TypeOp`] with the branch/return flag bits that matter
//!     to later passes (the only flags any of these rules' new opcodes carry).
//!     STUB(W6).
//!   - **`newUniqueOut`/`newVarnodeOut`** (the output-Varnode factories) are the
//!     funcdata_varnode wave's; `Funcdata::opSetOutput` itself is deferred on a
//!     `banks_mut()` split-borrow accessor.  [`new_unique_out`]/[`new_varnode_out`]
//!     compose the public `VarnodeBank::create_def*` primitives directly — exact
//!     for a fresh unique output (which never unifies), with a `// STUB(W3)` note
//!     for the register-address unification corner of `new_varnode_out`.
//!   - **W4 `Architecture`/`Scope` surfaces** — `getSpaceBySpacebase`,
//!     `Varnode::getSpaceFromConst`, `getCallSpecs`, `getScopeLocal`,
//!     `findJumpTable`, `opNormalizeFlip`/`opFlipCondition` — are not ported.
//!     The rules that depend on them (`RuleLoadVarnode`, `RuleStoreVarnode`,
//!     `RuleSwitchSingle`, `RuleCondNegate`) transcribe the body but short-circuit
//!     at the missing call (returning the C++ early-out) with a `// STUB` note.

use kuna_base::address::{calc_mask, leastsigbit_set, sign_extend_sized, Address, SeqNum};
use kuna_base::types::{int4, uintb, Wrap};
use kuna_num::opcodes::OpCode;
use std::rc::Rc;

use crate::action::{ActionGroupList, Rule, RuleSpec};
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;
use crate::context::{OpId, VarnodeId};
use crate::varnode::DefOpInfo;

// =============================================================================
// Shared local shims (W3/W6 seams — see module docs)
// =============================================================================

/// `data.opSetOpcode(op, opc)` — resolves the [`OpCode`] to a [`TypeOp`] and
/// hands it to the bank (C++ `obank.changeOpcode(op, glb->inst[opc])`).
///
/// Resolves through the canonical `type_op_info` table so the op carries its real
/// `opflags` (the eval-type bit in particular).  An op set with a flag-less
/// `TypeOp` reports `getEvalType()==0`, so `PcodeOp::collapse` /
/// `RuleCollapseConstants` cannot fold it: a SUBPIECE this rule (`RuleSubCommute`)
/// pushes earlier then stays as an unfolded `SUB(const,0)`, surfacing as a
/// redundant `value & SUB(0xffffffff,0)` mask in the rendered C.
fn set_opcode(data: &mut Funcdata, op: OpId, opc: OpCode) {
    data.op_set_opcode(op, crate::typeop::type_op_for(opc));
}

/// The unknown-base [`Datatype`] of size `s` (C++ `glb->types->getBase(s,
/// TYPE_UNKNOWN)`).  STUB(W6): the `TypeFactory` is W6; the skeleton is built
/// directly, exactly as the funcdata_varnode factories do.
fn type_base_unknown(s: int4) -> Rc<Datatype> {
    Rc::new(Datatype::new(s, type_metatype::TYPE_UNKNOWN))
}

/// `data.newUniqueOut(s, op)` (C++ `funcdata_varnode.cc:131`): allocate a fresh
/// \e unique-space Varnode as the output of `op`.
///
/// Composed from the public `VarnodeBank::create_def_unique` primitive — a fresh
/// unique Varnode is never unified by `xref`, so the `replace_reads` callback is
/// a no-op (the genuine `Funcdata::opSetOutput` defers only because *register*
/// outputs may unify).  The C++ `assignHigh`/`checkForLanedRegister` tail is a
/// W7/W4 no-op at this wave.
fn new_unique_out(data: &mut Funcdata, s: int4, op: OpId) -> VarnodeId {
    let seqnum = data.obank().get(op).expect("new_unique_out: stale op").get_seq_num().clone();
    let def = DefOpInfo { id: op, seqnum };
    let ct = type_base_unknown(s);
    // Varnode *vn = vbank.createDefUnique(s,ct,op);
    let vn = data
        .vbank_mut()
        .create_def_unique(s, ct, def, &mut |_, _, _| Ok(()))
        .expect("new_unique_out: createDefUnique");
    // op->setOutput(vn);
    data.obank_mut().get_mut(op).expect("new_unique_out: stale op").set_output(Some(vn));
    vn
}

/// `data.newVarnodeOut(s, m, op)` (C++ `funcdata_varnode.cc:106`): allocate a
/// Varnode at storage address `m` as the output of `op`.
///
/// Composed from `VarnodeBank::create_def`.  STUB(W3): a register-address output
/// can in principle unify with an existing equivalent free Varnode, in which case
/// the genuine `opSetOutput` would run the `replace_reads` op-rewiring; the
/// `banks_mut()` split-borrow that needs is the funcdata serial chain's, so the
/// callback here is a no-op (correct whenever no equivalent pre-exists, the case
/// the calling rules construct).
///
/// The C++ `Funcdata::newVarnodeOut` (`funcdata_varnode.cc:106`) tail then runs
/// `setVarnodeProperties(vn)` (the `localmap->queryProperties` symbol/flag seed).
/// `RuleStoreVarnode` builds the output at the *global* storage address of a
/// `STORE ram,#const,val`, so in C++ that seed paints `persist`/`addrtied` on the
/// global write.  Here [`Funcdata::set_varnode_properties`](crate::funcdata::Funcdata::set_varnode_properties)
/// is the faithful call site, but its persist/addrtied marking is currently
/// DEFERRED (see its doc): the global-store survival is instead delivered by the
/// heritage path (`Heritage::guard` + `guard_returns` RETURN-COPY), which is
/// sufficient for every global-store datatest and does not regress the
/// HighVariable-naming-dependent cases.  The call is retained so the marking
/// re-lands here unchanged when the naming seam arrives.
fn new_varnode_out(data: &mut Funcdata, s: int4, m: Address, op: OpId) -> VarnodeId {
    let seqnum = data.obank().get(op).expect("new_varnode_out: stale op").get_seq_num().clone();
    let def = DefOpInfo { id: op, seqnum };
    let ct = type_base_unknown(s);
    let vn = data
        .vbank_mut()
        .create_def(s, m, ct, def, &mut |_, _, _| Ok(()))
        .expect("new_varnode_out: createDef");
    data.obank_mut().get_mut(op).expect("new_varnode_out: stale op").set_output(Some(vn));
    // setVarnodeProperties(vn): the C++ tail seed (persist/addrtied marking
    // currently deferred in the callee — see its doc).
    data.set_varnode_properties(vn);
    vn
}

/// `data.opSetOutput(op, vn)` for the case where `vn` is a fresh (unique)
/// Varnode being moved onto a different op (C++ `Funcdata::opSetOutput`,
/// `funcdata_op.cc:70`).
///
/// The full `Funcdata::opSetOutput` is deferred on the `banks_mut()` accessor;
/// here `vn` is always a just-created unique with no pre-existing equivalent, so
/// the unify branch of `set_def` is dead and the no-op `replace_reads` is exact.
/// The C++ prologue (unset `op`'s old output, steal `vn` from any prior def) is
/// transcribed.  STUB(W3).
fn op_set_output(data: &mut Funcdata, op: OpId, vn: VarnodeId) {
    // if (vn == op->getOut()) return;
    if data.obank().get(op).expect("op_set_output: stale op").get_out() == Some(vn) {
        return;
    }
    // if (op->getOut() != 0) opUnsetOutput(op);
    if data.obank().get(op).expect("op_set_output: stale op").get_out().is_some() {
        data.op_unset_output(op);
    }
    // if (vn->getDef() != 0) opUnsetOutput(vn->getDef());
    if let Some(defop) = data.vbank().get(vn).expect("op_set_output: stale vn").get_def() {
        data.op_unset_output(defop);
    }
    // vn = vbank.setDef(vn, op);  (no unify for a fresh unique — no-op replace)
    let seqnum = data.obank().get(op).expect("op_set_output: stale op").get_seq_num().clone();
    let def = DefOpInfo { id: op, seqnum };
    let vn = data
        .vbank_mut()
        .set_def(vn, def, &mut |_, _, _| Ok(()))
        .expect("op_set_output: setDef");
    // op->setOutput(vn);
    data.obank_mut().get_mut(op).expect("op_set_output: stale op").set_output(Some(vn));
}

// --- terse Varnode/op read helpers (the C++ `op->getIn(i)->...` chains) -------

/// `op->getIn(slot)` (panics on a null operand, as the C++ deref would be UB).
fn in_vn(data: &Funcdata, op: OpId, slot: int4) -> VarnodeId {
    data.obank().get(op).expect("in_vn: stale op").get_in(slot).expect("in_vn: null operand")
}

/// `op->getOut()` (panics if `op` has no output).
fn out_vn(data: &Funcdata, op: OpId) -> VarnodeId {
    data.obank().get(op).expect("out_vn: stale op").get_out().expect("out_vn: no output")
}

/// `op->code()`.
fn code(data: &Funcdata, op: OpId) -> OpCode {
    data.obank().get(op).expect("code: stale op").code()
}

/// `vn->isWritten()`.
fn is_written(data: &Funcdata, vn: VarnodeId) -> bool {
    data.vbank().get(vn).expect("is_written: stale vn").is_written()
}

/// `vn->getDef()` (the C++ `getDef()` is only ever called on a written Varnode).
fn def_of(data: &Funcdata, vn: VarnodeId) -> OpId {
    data.vbank().get(vn).expect("def_of: stale vn").get_def().expect("def_of: not written")
}

/// `vn->isFree()`.
fn is_free(data: &Funcdata, vn: VarnodeId) -> bool {
    data.vbank().get(vn).expect("is_free: stale vn").is_free()
}

/// `vn->isConstant()`.
fn is_constant(data: &Funcdata, vn: VarnodeId) -> bool {
    data.vbank().get(vn).expect("is_constant: stale vn").is_constant()
}

/// `vn->getOffset()`.
fn offset_of(data: &Funcdata, vn: VarnodeId) -> uintb {
    data.vbank().get(vn).expect("offset_of: stale vn").get_offset()
}

/// `vn->getSize()`.
fn size_of(data: &Funcdata, vn: VarnodeId) -> int4 {
    data.vbank().get(vn).expect("size_of: stale vn").get_size()
}

/// `vn->getNZMask()`.
fn nzmask_of(data: &Funcdata, vn: VarnodeId) -> uintb {
    data.vbank().get(vn).expect("nzmask_of: stale vn").get_nz_mask()
}

/// `op->getAddr()`.
fn addr_of(data: &Funcdata, op: OpId) -> Address {
    data.obank().get(op).expect("addr_of: stale op").get_addr().clone()
}

/// `op->getSeqNum()`.
fn seqnum_of(data: &Funcdata, op: OpId) -> SeqNum {
    data.obank().get(op).expect("seqnum_of: stale op").get_seq_num().clone()
}

// =============================================================================
// RuleLoadVarnode  (ruleaction.cc:4293)
// =============================================================================

/// \brief Convert LOAD operations using a constant offset to COPY (C++
/// `RuleLoadVarnode`).
///
/// STUB(W4): the C++ classifies the pointer with `checkSpacebase`, which calls
/// `Varnode::getSpaceFromConst` and `Architecture::getSpaceBySpacebase` — neither
/// is on the W4 `Architecture`/`Varnode` skeleton.  The conversion body is fully
/// transcribed below the classification, but the classifier short-circuits to
/// "not a spacebase load" (the C++ `baseoff == 0` early-out), so the rule is a
/// no-op until W4 supplies the space machinery.  Recorded as a loss.
pub struct RuleLoadVarnode;

impl RuleLoadVarnode {
    /// Constructor (C++ `RuleLoadVarnode(const string &g)`).
    pub fn new() -> RuleLoadVarnode {
        RuleLoadVarnode
    }

    /// C++ `RuleLoadVarnode::correctSpacebase` (`ruleaction.cc:4195`).
    ///
    /// Return the associated space if `vn` is an \e active spacebase that loads
    /// from the correct space `spc`.  The Varnode is either a spacebase-register
    /// input (the global is then `getSpaceBySpacebase`), or a \e constant pseudo
    /// spacebase (then `spc` itself).
    fn correct_spacebase(
        data: &Funcdata,
        vn: VarnodeId,
        spc: &Rc<kuna_base::space::AddrSpace>,
    ) -> Option<Rc<kuna_base::space::AddrSpace>> {
        let v = data.vbank().get(vn)?;
        if !v.is_spacebase() {
            return None;
        }
        if v.is_constant() {
            // We have a global pseudo spacebase: associate with load/stored space.
            return Some(Rc::clone(spc));
        }
        if !v.is_input() {
            return None;
        }
        let assoc = data
            .get_arch()
            .manage()
            .get_space_by_spacebase(v.get_addr(), v.get_size())?;
        // Loading off the right space?
        match assoc.get_contain() {
            Some(c) if c.get_index() == spc.get_index() => Some(assoc),
            _ => None,
        }
    }

    /// C++ `RuleLoadVarnode::vnSpacebase` (`ruleaction.cc:4216`).
    ///
    /// Check if `vn` is `spacebase + constant`; if so return the associated space
    /// and pass back the constant in the returned tuple's second slot.
    fn vn_spacebase(
        data: &Funcdata,
        vn: VarnodeId,
        spc: &Rc<kuna_base::space::AddrSpace>,
    ) -> Option<(Rc<kuna_base::space::AddrSpace>, uintb)> {
        if let Some(retspace) = RuleLoadVarnode::correct_spacebase(data, vn, spc) {
            return Some((retspace, 0));
        }
        let v = data.vbank().get(vn)?;
        if !v.is_written() {
            return None;
        }
        let op = v.get_def()?;
        if code(data, op) != OpCode::CPUI_INT_ADD {
            return None;
        }
        let vn1 = in_vn(data, op, 0);
        let vn2 = in_vn(data, op, 1);
        if let Some(retspace) = RuleLoadVarnode::correct_spacebase(data, vn1, spc) {
            if is_constant(data, vn2) {
                return Some((retspace, offset_of(data, vn2)));
            }
            return None;
        }
        if let Some(retspace) = RuleLoadVarnode::correct_spacebase(data, vn2, spc) {
            if is_constant(data, vn1) {
                return Some((retspace, offset_of(data, vn1)));
            }
        }
        None
    }

    /// C++ `RuleLoadVarnode::checkSpacebase` (`ruleaction.cc:4258`).
    ///
    /// Returns `Some((space, offoff))` if the LOAD/STORE `op` reads off a
    /// spacebase+constant (or a literal constant offset), else `None`.
    pub fn check_spacebase(data: &Funcdata, op: OpId) -> Option<(Rc<kuna_base::space::AddrSpace>, uintb)> {
        // offvn = op->getIn(1); loadspace = op->getIn(0)->getSpaceFromConst();
        let offvn = in_vn(data, op, 1);
        let space_const = in_vn(data, op, 0);
        let loadspace = space_from_const(data, space_const)?;
        // Treat segmentop as part of load/store.
        let mut offvn = offvn;
        if is_written(data, offvn) && code(data, def_of(data, offvn)) == OpCode::CPUI_SEGMENTOP {
            let segdef = def_of(data, offvn);
            offvn = in_vn(data, segdef, 2);
            // If the segmentop inner is constant we are NOT looking for a
            // spacebase; defer to RuleSegmentOp / a fixed address (return None).
            if is_constant(data, offvn) {
                return None;
            }
        } else if is_constant(data, offvn) {
            // Check for a literal constant offset into `loadspace`.
            return Some((loadspace, offset_of(data, offvn)));
        }
        RuleLoadVarnode::vn_spacebase(data, offvn, &loadspace)
    }
}

/// C++ `Varnode::getSpaceFromConst` (`varnode.hh:427`): decode the AddrSpace a
/// constant-space Varnode encodes.  The C++ stores the raw `AddrSpace *` pointer
/// in the offset; the Rust port (LOSS-015) stores the space's manager *index*,
/// resolved back here through the function's space manager.
fn space_from_const(data: &Funcdata, vn: VarnodeId) -> Option<Rc<kuna_base::space::AddrSpace>> {
    let idx = offset_of(data, vn);
    let manage = data.get_arch().manage();
    // The C++ `getSpaceFromConst` reinterprets the offset as a raw `AddrSpace *`;
    // here the offset is the manager index (LOSS-015).  A non-index value (e.g. a
    // hand-built fixture's plain constant) is out of range — reject it rather than
    // panic on the `baselist` bounds (the C++ would deref a bad pointer; the only
    // real caller decodes a genuine spaceid constant, always in range).
    if idx >= manage.num_spaces() as u64 {
        return None;
    }
    manage.get_space(idx as i32).map(Rc::clone)
}

impl Default for RuleLoadVarnode {
    fn default() -> Self {
        RuleLoadVarnode::new()
    }
}

impl Rule for RuleLoadVarnode {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_LOAD]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        // C++ `clone` gates on `grouplist.contains(getGroup())`; RuleLoadVarnode
        // is registered with group "stackvars" (coreaction.cc:5939), not
        // "analysis".
        if !grouplist.contains("stackvars") {
            return None;
        }
        Some(Box::new(RuleLoadVarnode::new()))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // baseoff = checkSpacebase(data.getArch(),op,offoff);
        // if (baseoff == 0) return 0;
        let (baseoff, offoff) = match RuleLoadVarnode::check_spacebase(data, op) {
            Some(v) => v,
            None => return 0,
        };
        // size = op->getOut()->getSize();
        let size = size_of(data, out_vn(data, op));
        // offoff = AddressToByte(offoff, baseoff->getWordSize());
        let offoff = kuna_base::space::AddrSpace::address_to_byte(offoff, baseoff.get_word_size());
        // newvn = data.newVarnode(size, baseoff, offoff);
        let newvn = data.new_varnode_space_off(size, Rc::clone(&baseoff), offoff);
        data.op_set_input(op, newvn, 0).expect("RuleLoadVarnode: opSetInput");
        data.op_remove_input(op, 1);
        set_opcode(data, op, OpCode::CPUI_COPY);
        // C++ ruleaction.cc:4316-4324 — the spacebase-placeholder tail.  When the
        // converted COPY output is the stack-pointer placeholder the call set up,
        // clear the trigger and resolve the call's relative stack offset (the RSP
        // keystone's resolveSpacebaseRelative wire: required so `tryreg=true` yields
        // the proper stack offset and `&val` arg).
        // Varnode *refvn = op->getOut();
        let refvn = out_vn(data, op);
        // if (refvn->isSpacebasePlaceholder()) {
        let is_ph = data
            .vbank()
            .get(refvn)
            .map(|v| v.is_spacebase_placeholder())
            .unwrap_or(false);
        if is_ph {
            // refvn->clearSpacebasePlaceholder(); // Clear the trigger
            if let Some(v) = data.vbank_mut().get_mut(refvn) {
                v.clear_spacebase_placeholder();
            }
            // PcodeOp *placeOp = refvn->loneDescend();
            if let Some(place_op) = data.lone_descend(refvn) {
                // FuncCallSpecs *fc = data.getCallSpecs(placeOp);
                if let Some(fc_idx) = data.get_call_specs_index(place_op) {
                    // if (fc != 0) fc->resolveSpacebaseRelative(data, refvn);
                    // The C++ mutates `fc` (a `FuncCallSpecs *`) while passing
                    // `&data`; lift the call-spec out with take/restore so the
                    // `&mut FuncCallSpecs` and `&mut Funcdata` borrows don't alias.
                    let mut qlst = data.take_call_specs();
                    let _ = qlst[fc_idx as usize].resolve_spacebase_relative(data, refvn);
                    data.restore_call_specs(qlst);
                }
            }
        }
        1
    }
}

// =============================================================================
// RuleStoreVarnode  (ruleaction.cc:4335)
// =============================================================================

/// \brief Convert STORE operations using a constant offset to COPY (C++
/// `RuleStoreVarnode`).  STUB(W4): shares `RuleLoadVarnode::checkSpacebase` and
/// `getScopeLocal()->markNotMapped`; see [`RuleLoadVarnode`]'s seam note.
pub struct RuleStoreVarnode;

impl RuleStoreVarnode {
    /// Constructor (C++ `RuleStoreVarnode(const string &g)`).
    pub fn new() -> RuleStoreVarnode {
        RuleStoreVarnode
    }
}

impl Default for RuleStoreVarnode {
    fn default() -> Self {
        RuleStoreVarnode::new()
    }
}

impl Rule for RuleStoreVarnode {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_STORE]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        // C++ `clone` gates on `grouplist.contains(getGroup())`; RuleStoreVarnode
        // is registered with group "stackvars" (coreaction.cc:5940), not
        // "analysis".
        if !grouplist.contains("stackvars") {
            return None;
        }
        Some(Box::new(RuleStoreVarnode::new()))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // baseoff = RuleLoadVarnode::checkSpacebase(data.getArch(),op,offoff);
        // if (baseoff == 0) return 0;
        let (baseoff, offoff) = match RuleLoadVarnode::check_spacebase(data, op) {
            Some(v) => v,
            None => return 0,
        };
        // size = op->getIn(2)->getSize();
        let size = size_of(data, in_vn(data, op, 2));
        // offoff = AddressToByte(offoff, baseoff->getWordSize());
        let offoff = kuna_base::space::AddrSpace::address_to_byte(offoff, baseoff.get_word_size());
        // Address addr(baseoff, offoff);  data.newVarnodeOut(size, addr, op);
        let addr = Address::new(Rc::clone(&baseoff), offoff);
        let outvn = new_varnode_out(data, size, addr, op);
        // op->getOut()->setStackStore();
        data.vbank_mut().get_mut(outvn).expect("RuleStoreVarnode: stale out").set_stack_store();
        data.op_remove_input(op, 1);
        data.op_remove_input(op, 0);
        set_opcode(data, op, OpCode::CPUI_COPY);
        // if (op->isStoreUnmapped()) data.getScopeLocal()->markNotMapped(baseoff,offoff,size,false);
        // ActionInternalStorage flags an eventual-constant STORE of an
        // <internal_storage> register (e.g. MIPS gp) as unmapped; converting it to a
        // COPY here unmaps the destination stack slot so it does not propagate as a
        // local alias across calls.
        let store_unmapped = data.obank().get(op).map(|o| o.is_store_unmapped()).unwrap_or(false);
        if store_unmapped {
            data.scope_local_mark_not_mapped(&baseoff, offoff, size, false);
        }
        1
    }
}

// =============================================================================
// RuleSubExtComm  (ruleaction.cc:4407)
// =============================================================================

/// \brief Commute SUBPIECE and INT_ZEXT: `sub(zext(V),c) => zext(sub(V,c))`
/// (C++ `RuleSubExtComm`).  Also works with INT_SEXT.  Corner cases:
///   - `sub(zext(V),c) => sub(V,C)`
///   - `sub(zext(V),0) => zext(V)`
pub struct RuleSubExtComm;

impl RuleSubExtComm {
    /// Constructor (C++ `RuleSubExtComm(const string &g)`).
    pub fn new() -> RuleSubExtComm {
        RuleSubExtComm
    }
}

impl Default for RuleSubExtComm {
    fn default() -> Self {
        RuleSubExtComm::new()
    }
}

impl Rule for RuleSubExtComm {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_SUBPIECE]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains("analysis") {
            return None;
        }
        Some(Box::new(RuleSubExtComm::new()))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // Varnode *base = op->getIn(0); if (!base->isWritten()) return 0;
        let base = in_vn(data, op, 0);
        if !is_written(data, base) {
            return 0;
        }
        // PcodeOp *extop = base->getDef();
        let extop = def_of(data, base);
        // if (code != INT_ZEXT && code != INT_SEXT) return 0;
        let extcode = code(data, extop);
        if extcode != OpCode::CPUI_INT_ZEXT && extcode != OpCode::CPUI_INT_SEXT {
            return 0;
        }
        // Varnode *invn = extop->getIn(0); if (invn->isFree()) return 0;
        let invn = in_vn(data, extop, 0);
        if is_free(data, invn) {
            return 0;
        }
        // int4 subcut = (int4)op->getIn(1)->getOffset();
        let subcut = offset_of(data, in_vn(data, op, 1)) as int4;
        let outvn = out_vn(data, op);
        let outsize = size_of(data, outvn);
        let invn_size = size_of(data, invn);
        // if (out->getSize() + subcut <= invn->getSize())  -- SUBPIECE misses ext bits
        if outsize + subcut <= invn_size {
            data.op_set_input(op, invn, 0).expect("RuleSubExtComm: opSetInput");
            // if (invn->getSize() == out->getSize()) { opRemoveInput(op,1); opSetOpcode(op,COPY); }
            if invn_size == outsize {
                data.op_remove_input(op, 1);
                set_opcode(data, op, OpCode::CPUI_COPY);
            }
            return 1;
        }

        // if (subcut >= invn->getSize()) return 0;
        if subcut >= invn_size {
            return 0;
        }

        let newvn;
        if subcut != 0 {
            // PcodeOp *newop = data.newOp(2,op->getAddr());
            let opaddr = addr_of(data, op);
            let newop = data.new_op(2, opaddr);
            set_opcode(data, newop, OpCode::CPUI_SUBPIECE);
            // newvn = data.newUniqueOut(invn->getSize()-subcut,newop);
            newvn = new_unique_out(data, invn_size - subcut, newop);
            // opSetInput(newop, newConstant(op->getIn(1)->getSize(), (uintb)subcut), 1);
            let csize = size_of(data, in_vn(data, op, 1));
            let cvn = data.new_constant(csize, subcut as uintb);
            data.op_set_input(newop, cvn, 1).expect("RuleSubExtComm: opSetInput");
            data.op_set_input(newop, invn, 0).expect("RuleSubExtComm: opSetInput");
            data.op_insert_before(newop, op);
        } else {
            newvn = invn;
        }

        data.op_remove_input(op, 1);
        set_opcode(data, op, extcode);
        data.op_set_input(op, newvn, 0).expect("RuleSubExtComm: opSetInput");
        1
    }
}

// =============================================================================
// RuleSubCommute  (ruleaction.cc:4465)
// =============================================================================

/// \brief Commute SUBPIECE operations with earlier operations where possible
/// (C++ `RuleSubCommute`).  Pushes SUBPIECE earlier (preferring short forms) in
/// the hope it runs into a constant / INT_SEXT / INT_ZEXT and cancels.
pub struct RuleSubCommute;

impl RuleSubCommute {
    /// Constructor (C++ `RuleSubCommute(const string &g)`).
    pub fn new() -> RuleSubCommute {
        RuleSubCommute
    }

    /// \brief Shrink the output of an extension to the given size (C++
    /// `RuleSubCommute::shortenExtension`, `ruleaction.cc:4485`).
    ///
    /// The output of an INT_ZEXT/INT_SEXT is replaced with a smaller/truncated
    /// Varnode; returns the new smaller Varnode.
    pub fn shorten_extension(data: &mut Funcdata, ext_op: OpId, max_size: int4) -> VarnodeId {
        // Varnode *origOut = extOp->getOut(); Address addr = origOut->getAddr();
        let orig_out = out_vn(data, ext_op);
        let orig_size = size_of(data, orig_out);
        let mut addr = data.vbank().get(orig_out).expect("shorten_extension: stale out").get_addr().clone();
        // if (addr.isBigEndian()) addr = addr + (origOut->getSize() - maxSize);
        if addr.is_big_endian() {
            addr = &addr + ((orig_size - max_size) as i64);
        }
        // data.opUnsetOutput(extOp);
        data.op_unset_output(ext_op);
        // return data.newVarnodeOut(maxSize, addr, extOp);
        new_varnode_out(data, max_size, addr, ext_op)
    }

    /// \brief Eliminate input extensions on the given binary PcodeOp (C++
    /// `RuleSubCommute::cancelExtensions`, `ruleaction.cc:4505`).
    ///
    /// Returns true if `longform` was successfully modified.
    pub fn cancel_extensions(
        data: &mut Funcdata,
        longform: OpId,
        sub_op: OpId,
        mut ext0_in: VarnodeId,
        mut ext1_in: VarnodeId,
    ) -> bool {
        let max_size;
        // Varnode *outvn = longform->getOut();
        let outvn = out_vn(data, longform);
        // if (outvn->loneDescend() != subOp) return false;
        if data.lone_descend(outvn) != Some(sub_op) {
            return false;
        }
        let e0sz = size_of(data, ext0_in);
        let e1sz = size_of(data, ext1_in);
        if e0sz == e1sz {
            max_size = e0sz;
            if is_free(data, ext0_in) {
                return false;
            }
            if is_free(data, ext1_in) {
                return false;
            }
        } else if e0sz < e1sz {
            max_size = e1sz;
            if is_free(data, ext1_in) {
                return false;
            }
            // if (longform->getIn(0)->loneDescend() != longform) return false;
            let lin0 = in_vn(data, longform, 0);
            if data.lone_descend(lin0) != Some(longform) {
                return false;
            }
            // ext0In = shortenExtension(longform->getIn(0)->getDef(), maxSize, data);
            let ext0_def = def_of(data, lin0);
            ext0_in = RuleSubCommute::shorten_extension(data, ext0_def, max_size);
        } else {
            max_size = e0sz;
            if is_free(data, ext0_in) {
                return false;
            }
            let lin1 = in_vn(data, longform, 1);
            if data.lone_descend(lin1) != Some(longform) {
                return false;
            }
            let ext1_def = def_of(data, lin1);
            ext1_in = RuleSubCommute::shorten_extension(data, ext1_def, max_size);
        }
        // data.opUnsetOutput(longform);
        data.op_unset_output(longform);
        // outvn = data.newUniqueOut(maxSize,longform);  (truncated longform output)
        let outvn = new_unique_out(data, max_size, longform);
        data.op_set_input(longform, ext0_in, 0).expect("cancel_extensions: opSetInput");
        data.op_set_input(longform, ext1_in, 1).expect("cancel_extensions: opSetInput");
        data.op_set_input(sub_op, outvn, 0).expect("cancel_extensions: opSetInput");
        true
    }
}

impl Default for RuleSubCommute {
    fn default() -> Self {
        RuleSubCommute::new()
    }
}

impl Rule for RuleSubCommute {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_SUBPIECE]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains("analysis") {
            return None;
        }
        Some(Box::new(RuleSubCommute::new()))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // Varnode *base = op->getIn(0); if (!base->isWritten()) return 0;
        let base = in_vn(data, op, 0);
        if !is_written(data, base) {
            return 0;
        }
        // int4 offset = op->getIn(1)->getOffset();
        let offset = offset_of(data, in_vn(data, op, 1)) as int4;
        let outvn = out_vn(data, op);
        // if (outvn->isPrecisLo()||outvn->isPrecisHi()) return 0;
        {
            let ov = data.vbank().get(outvn).expect("RuleSubCommute: stale out");
            if ov.is_precis_lo() || ov.is_precis_hi() {
                return 0;
            }
        }
        let insize = size_of(data, base);
        let outsize = size_of(data, outvn);
        // PcodeOp *longform = base->getDef();
        let longform = def_of(data, base);
        let mut j: int4 = -1;
        match code(data, longform) {
            OpCode::CPUI_INT_LEFT => {
                j = 1; // Special processing for shift amount param
                if offset != 0 {
                    return 0;
                }
                // if (longform->getIn(0)->isWritten()) { ... } else return 0;
                let lin0 = in_vn(data, longform, 0);
                if is_written(data, lin0) {
                    let opc = code(data, def_of(data, lin0));
                    if opc != OpCode::CPUI_INT_ZEXT && opc != OpCode::CPUI_PIECE {
                        return 0;
                    }
                } else {
                    return 0;
                }
            }
            OpCode::CPUI_INT_REM | OpCode::CPUI_INT_DIV => {
                // Only commutes if inputs are zero extended
                if offset != 0 {
                    return 0;
                }
                let lin0 = in_vn(data, longform, 0);
                if !is_written(data, lin0) {
                    return 0;
                }
                let zext0 = def_of(data, lin0);
                if code(data, zext0) != OpCode::CPUI_INT_ZEXT {
                    return 0;
                }
                let zext0_in = in_vn(data, zext0, 0);
                let lin1 = in_vn(data, longform, 1);
                if is_written(data, lin1) {
                    let zext1 = def_of(data, lin1);
                    if code(data, zext1) != OpCode::CPUI_INT_ZEXT {
                        return 0;
                    }
                    let zext1_in = in_vn(data, zext1, 0);
                    if size_of(data, zext1_in) > outsize || size_of(data, zext0_in) > outsize {
                        // Special case: PARTIAL commute of the SUBPIECE
                        if RuleSubCommute::cancel_extensions(data, longform, op, zext0_in, zext1_in) {
                            return 1; // Leave SUBPIECE intact
                        }
                        return 0;
                    }
                    // ZEXT sizes both not bigger -> commute SUBPIECE (fallthru)
                } else if is_constant(data, lin1) && (size_of(data, zext0_in) <= outsize) {
                    let val = offset_of(data, lin1);
                    let smallval = val & calc_mask(outsize);
                    if val != smallval {
                        return 0;
                    }
                } else {
                    return 0;
                }
            }
            OpCode::CPUI_INT_SREM | OpCode::CPUI_INT_SDIV => {
                // Only commutes if inputs are sign extended
                if offset != 0 {
                    return 0;
                }
                let lin0 = in_vn(data, longform, 0);
                if !is_written(data, lin0) {
                    return 0;
                }
                let sext0 = def_of(data, lin0);
                if code(data, sext0) != OpCode::CPUI_INT_SEXT {
                    return 0;
                }
                let sext0_in = in_vn(data, sext0, 0);
                let lin1 = in_vn(data, longform, 1);
                if is_written(data, lin1) {
                    let sext1 = def_of(data, lin1);
                    if code(data, sext1) != OpCode::CPUI_INT_SEXT {
                        return 0;
                    }
                    let sext1_in = in_vn(data, sext1, 0);
                    if size_of(data, sext1_in) > outsize || size_of(data, sext0_in) > outsize {
                        // Special case: PARTIAL commute of the SUBPIECE
                        if RuleSubCommute::cancel_extensions(data, longform, op, sext0_in, sext1_in) {
                            return 1; // Leave SUBPIECE intact
                        }
                        return 0;
                    }
                    // SEXT sizes both not bigger -> commute SUBPIECE (fallthru)
                } else if is_constant(data, lin1) && (size_of(data, sext0_in) <= outsize) {
                    let val = offset_of(data, lin1);
                    let mut smallval = val & calc_mask(outsize);
                    // smallval = sign_extend(smallval,outvn->getSize(),insize);
                    smallval = sign_extend_sized(smallval, outsize, insize);
                    if val != smallval {
                        return 0;
                    }
                } else {
                    return 0;
                }
            }
            OpCode::CPUI_INT_ADD => {
                if offset != 0 {
                    return 0; // Only commutes with least significant SUBPIECE
                }
                // if (longform->getIn(0)->isSpacebase()) return 0; // Deconflict RulePtrArith
                let lin0 = in_vn(data, longform, 0);
                if data.vbank().get(lin0).expect("RuleSubCommute: stale lin0").is_spacebase() {
                    return 0;
                }
            }
            OpCode::CPUI_INT_MULT => {
                if offset != 0 {
                    return 0; // Only commutes with least significant SUBPIECE
                }
            }
            // Bitwise ops, type of subpiece doesn't matter
            OpCode::CPUI_INT_NEGATE
            | OpCode::CPUI_INT_XOR
            | OpCode::CPUI_INT_AND
            | OpCode::CPUI_INT_OR => {}
            _ => {
                // Most ops don't commute
                return 0;
            }
        }

        // Make sure no other piece of base is getting used
        if data.lone_descend(base) != Some(op) {
            return 0;
        }

        if offset == 0 {
            // Look for overlap with RuleSubZext
            let nextop = data.lone_descend(outvn);
            if let Some(nextop) = nextop {
                if code(data, nextop) == OpCode::CPUI_INT_ZEXT
                    && size_of(data, out_vn(data, nextop)) == insize
                {
                    return 0;
                }
            }
        }

        let numinput = data.obank().get(longform).expect("RuleSubCommute: stale longform").num_input();
        let mut last_in: Option<VarnodeId> = None;
        let mut new_vn: Option<VarnodeId> = None;
        for i in 0..numinput {
            let vn = in_vn(data, longform, i);
            if i != j {
                // if (lastIn != vn || newVn == 0)  -- don't duplicate the SUBPIECE
                //   if inputs are the same.  The else-branch only runs when
                //   `last_in == Some(vn)` *and* `new_vn` is already Some.
                match new_vn.filter(|_| last_in == Some(vn)) {
                    Some(existing) => {
                        data.op_set_input(longform, existing, i)
                            .expect("RuleSubCommute: opSetInput");
                    }
                    None => {
                        let opaddr = addr_of(data, op);
                        let newsub = data.new_op(2, opaddr); // Commuted SUBPIECE op
                        set_opcode(data, newsub, OpCode::CPUI_SUBPIECE);
                        // newVn = data.newUniqueOut(outvn->getSize(),newsub);
                        let nv = new_unique_out(data, outsize, newsub);
                        new_vn = Some(nv);
                        data.op_set_input(longform, nv, i).expect("RuleSubCommute: opSetInput");
                        // vn may be free, set as input after setting newVn
                        data.op_set_input(newsub, vn, 0).expect("RuleSubCommute: opSetInput");
                        let cvn = data.new_constant(4, offset as uintb);
                        data.op_set_input(newsub, cvn, 1).expect("RuleSubCommute: opSetInput");
                        data.op_insert_before(newsub, longform);
                    }
                }
            }
            last_in = Some(vn);
        }
        // data.opSetOutput(longform,outvn);
        op_set_output(data, longform, outvn);
        // data.opDestroy(op);  -- Get rid of old SUBPIECE
        data.op_destroy(op);
        1
    }
}

// =============================================================================
// RuleConcatCommute  (ruleaction.cc:4677)
// =============================================================================

/// \brief Commute PIECE with INT_AND, INT_OR, and INT_XOR (C++
/// `RuleConcatCommute`):
///   - `concat( V & c, W)  =>  concat(V,W) & (c<<16 | 0xffff)`
///   - `concat( V, W | c)  =>  concat(V,W) | c`
pub struct RuleConcatCommute;

impl RuleConcatCommute {
    /// Constructor (C++ `RuleConcatCommute(const string &g)`).
    pub fn new() -> RuleConcatCommute {
        RuleConcatCommute
    }
}

impl Default for RuleConcatCommute {
    fn default() -> Self {
        RuleConcatCommute::new()
    }
}

impl Rule for RuleConcatCommute {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_PIECE]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains("analysis") {
            return None;
        }
        Some(Box::new(RuleConcatCommute::new()))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // int4 outsz = op->getOut()->getSize();
        let outsz = size_of(data, out_vn(data, op));
        // if (outsz > sizeof(uintb)) return 0;  // FIXME precision for constants
        if outsz as usize > std::mem::size_of::<uintb>() {
            return 0;
        }
        for i in 0..2i32 {
            // vn = op->getIn(i); if (!vn->isWritten()) continue;
            let vn = in_vn(data, op, i);
            if !is_written(data, vn) {
                continue;
            }
            let logicop = def_of(data, vn);
            let opc = code(data, logicop);
            let hi;
            let lo;
            let mut val: uintb;
            if opc == OpCode::CPUI_INT_OR || opc == OpCode::CPUI_INT_XOR {
                let lc1 = in_vn(data, logicop, 1);
                if !is_constant(data, lc1) {
                    continue;
                }
                val = offset_of(data, lc1);
                if i == 0 {
                    hi = in_vn(data, logicop, 0);
                    lo = in_vn(data, op, 1);
                    val <<= 8 * (size_of(data, lo) as u32);
                } else {
                    hi = in_vn(data, op, 0);
                    lo = in_vn(data, logicop, 0);
                }
            } else if opc == OpCode::CPUI_INT_AND {
                let lc1 = in_vn(data, logicop, 1);
                if !is_constant(data, lc1) {
                    continue;
                }
                val = offset_of(data, lc1);
                if i == 0 {
                    hi = in_vn(data, logicop, 0);
                    lo = in_vn(data, op, 1);
                    val <<= 8 * (size_of(data, lo) as u32);
                    val |= calc_mask(size_of(data, lo));
                } else {
                    hi = in_vn(data, op, 0);
                    lo = in_vn(data, logicop, 0);
                    val |= calc_mask(size_of(data, hi)) << (8 * (size_of(data, lo) as u32));
                }
            } else {
                continue;
            }
            if is_free(data, hi) {
                continue;
            }
            if is_free(data, lo) {
                continue;
            }
            let opaddr = addr_of(data, op);
            let newconcat = data.new_op(2, opaddr);
            set_opcode(data, newconcat, OpCode::CPUI_PIECE);
            let newvn = new_unique_out(data, outsz, newconcat);
            data.op_set_input(newconcat, hi, 0).expect("RuleConcatCommute: opSetInput");
            data.op_set_input(newconcat, lo, 1).expect("RuleConcatCommute: opSetInput");
            data.op_insert_before(newconcat, op);
            set_opcode(data, op, opc);
            data.op_set_input(op, newvn, 0).expect("RuleConcatCommute: opSetInput");
            let newvn_size = size_of(data, newvn);
            let cvn = data.new_constant(newvn_size, val);
            data.op_set_input(op, cvn, 1).expect("RuleConcatCommute: opSetInput");
            return 1;
        }
        0
    }
}

// =============================================================================
// RuleConcatZext  (ruleaction.cc:4808)
// =============================================================================

/// \brief Commute PIECE with INT_ZEXT: `concat(zext(V),W) => zext(concat(V,W))`
/// (C++ `RuleConcatZext`).
pub struct RuleConcatZext;

impl RuleConcatZext {
    /// Constructor (C++ `RuleConcatZext(const string &g)`).
    pub fn new() -> RuleConcatZext {
        RuleConcatZext
    }
}

impl Default for RuleConcatZext {
    fn default() -> Self {
        RuleConcatZext::new()
    }
}

impl Rule for RuleConcatZext {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_PIECE]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains("analysis") {
            return None;
        }
        Some(Box::new(RuleConcatZext::new()))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // hi = op->getIn(0); if (!hi->isWritten()) return 0;
        let mut hi = in_vn(data, op, 0);
        if !is_written(data, hi) {
            return 0;
        }
        let zextop = def_of(data, hi);
        if code(data, zextop) != OpCode::CPUI_INT_ZEXT {
            return 0;
        }
        hi = in_vn(data, zextop, 0);
        let lo = in_vn(data, op, 1);
        if is_free(data, hi) {
            return 0;
        }
        if is_free(data, lo) {
            return 0;
        }

        // Create new (earlier) concat out of hi and lo
        let opaddr = addr_of(data, op);
        let newconcat = data.new_op(2, opaddr);
        set_opcode(data, newconcat, OpCode::CPUI_PIECE);
        let newvn = new_unique_out(data, size_of(data, hi) + size_of(data, lo), newconcat);
        data.op_set_input(newconcat, hi, 0).expect("RuleConcatZext: opSetInput");
        data.op_set_input(newconcat, lo, 1).expect("RuleConcatZext: opSetInput");
        data.op_insert_before(newconcat, op);

        // Change original op into a ZEXT
        data.op_remove_input(op, 1);
        data.op_set_input(op, newvn, 0).expect("RuleConcatZext: opSetInput");
        set_opcode(data, op, OpCode::CPUI_INT_ZEXT);
        1
    }
}

// =============================================================================
// RuleZextCommute  (ruleaction.cc:4846)
// =============================================================================

/// \brief Commute INT_ZEXT with INT_RIGHT: `zext(V) >> W => zext(V >> W)`
/// (C++ `RuleZextCommute`).
pub struct RuleZextCommute;

impl RuleZextCommute {
    /// Constructor (C++ `RuleZextCommute(const string &g)`).
    pub fn new() -> RuleZextCommute {
        RuleZextCommute
    }
}

impl Default for RuleZextCommute {
    fn default() -> Self {
        RuleZextCommute::new()
    }
}

impl Rule for RuleZextCommute {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_INT_RIGHT]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains("analysis") {
            return None;
        }
        Some(Box::new(RuleZextCommute::new()))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // Varnode *zextvn = op->getIn(0); if (!zextvn->isWritten()) return 0;
        let zextvn = in_vn(data, op, 0);
        if !is_written(data, zextvn) {
            return 0;
        }
        let zextop = def_of(data, zextvn);
        if code(data, zextop) != OpCode::CPUI_INT_ZEXT {
            return 0;
        }
        let zextin = in_vn(data, zextop, 0);
        if is_free(data, zextin) {
            return 0;
        }
        // Varnode *savn = op->getIn(1);
        let savn = in_vn(data, op, 1);
        // if ((!savn->isConstant())&&(savn->isFree())) return 0;
        if !is_constant(data, savn) && is_free(data, savn) {
            return 0;
        }

        let opaddr = addr_of(data, op);
        let newop = data.new_op(2, opaddr);
        set_opcode(data, newop, OpCode::CPUI_INT_RIGHT);
        let newout = new_unique_out(data, size_of(data, zextin), newop);
        data.op_remove_input(op, 1);
        data.op_set_input(op, newout, 0).expect("RuleZextCommute: opSetInput");
        set_opcode(data, op, OpCode::CPUI_INT_ZEXT);
        data.op_set_input(newop, zextin, 0).expect("RuleZextCommute: opSetInput");
        data.op_set_input(newop, savn, 1).expect("RuleZextCommute: opSetInput");
        data.op_insert_before(newop, op);
        1
    }
}

// =============================================================================
// RuleZextShiftZext  (ruleaction.cc:4879)
// =============================================================================

/// \brief Simplify multiple INT_ZEXT operations:
/// `zext( zext(V) << c ) => zext(V) << c` (C++ `RuleZextShiftZext`).
pub struct RuleZextShiftZext;

impl RuleZextShiftZext {
    /// Constructor (C++ `RuleZextShiftZext(const string &g)`).
    pub fn new() -> RuleZextShiftZext {
        RuleZextShiftZext
    }
}

impl Default for RuleZextShiftZext {
    fn default() -> Self {
        RuleZextShiftZext::new()
    }
}

impl Rule for RuleZextShiftZext {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_INT_ZEXT]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains("analysis") {
            return None;
        }
        Some(Box::new(RuleZextShiftZext::new()))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // Varnode *invn = op->getIn(0); if (!invn->isWritten()) return 0;
        let invn = in_vn(data, op, 0);
        if !is_written(data, invn) {
            return 0;
        }
        let shiftop = def_of(data, invn);
        if code(data, shiftop) == OpCode::CPUI_INT_ZEXT {
            // Check for ZEXT( ZEXT( a ) )
            let vn = in_vn(data, shiftop, 0);
            if is_free(data, vn) {
                return 0;
            }
            // Only propagate if -op- is only use of -invn-
            if data.lone_descend(invn) != Some(op) {
                return 0;
            }
            data.op_set_input(op, vn, 0).expect("RuleZextShiftZext: opSetInput");
            return 1;
        }
        if code(data, shiftop) != OpCode::CPUI_INT_LEFT {
            return 0;
        }
        if !is_constant(data, in_vn(data, shiftop, 1)) {
            return 0;
        }
        let sin0 = in_vn(data, shiftop, 0);
        if !is_written(data, sin0) {
            return 0;
        }
        let zext2op = def_of(data, sin0);
        if code(data, zext2op) != OpCode::CPUI_INT_ZEXT {
            return 0;
        }
        let rootvn = in_vn(data, zext2op, 0);
        if is_free(data, rootvn) {
            return 0;
        }

        // uintb sa = shiftop->getIn(1)->getOffset();
        let sa = offset_of(data, in_vn(data, shiftop, 1));
        // if (sa > 8*(uintb)(zext2op->getOut()->getSize() - rootvn->getSize())) return 0;
        let zext2_out_size = size_of(data, out_vn(data, zext2op));
        if sa > 8 * ((zext2_out_size - size_of(data, rootvn)) as uintb) {
            return 0; // Shift might lose bits off the top
        }
        let opaddr = addr_of(data, op);
        let newop = data.new_op(1, opaddr);
        set_opcode(data, newop, OpCode::CPUI_INT_ZEXT);
        let outvn = new_unique_out(data, size_of(data, out_vn(data, op)), newop);
        data.op_set_input(newop, rootvn, 0).expect("RuleZextShiftZext: opSetInput");
        set_opcode(data, op, OpCode::CPUI_INT_LEFT);
        data.op_set_input(op, outvn, 0).expect("RuleZextShiftZext: opSetInput");
        let cvn = data.new_constant(4, sa);
        data.op_insert_input(op, cvn, 1).expect("RuleZextShiftZext: opInsertInput");
        data.op_insert_before(newop, op);
        1
    }
}

// =============================================================================
// RuleShiftAnd  (ruleaction.cc:4923)
// =============================================================================

/// \brief Eliminate any INT_AND when the bits it zeroes out are discarded by a
/// shift (C++ `RuleShiftAnd`).  Also handles bits that are already zero.
pub struct RuleShiftAnd;

impl RuleShiftAnd {
    /// Constructor (C++ `RuleShiftAnd(const string &g)`).
    pub fn new() -> RuleShiftAnd {
        RuleShiftAnd
    }
}

impl Default for RuleShiftAnd {
    fn default() -> Self {
        RuleShiftAnd::new()
    }
}

impl Rule for RuleShiftAnd {
    fn get_op_list(&self) -> Vec<OpCode> {
        // C++ pushes INT_RIGHT, INT_LEFT, INT_MULT in this order.
        vec![OpCode::CPUI_INT_RIGHT, OpCode::CPUI_INT_LEFT, OpCode::CPUI_INT_MULT]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains("analysis") {
            return None;
        }
        Some(Box::new(RuleShiftAnd::new()))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // Varnode *cvn = op->getIn(1); if (!cvn->isConstant()) return 0;
        let cvn = in_vn(data, op, 1);
        if !is_constant(data, cvn) {
            return 0;
        }
        let shiftin = in_vn(data, op, 0);
        if !is_written(data, shiftin) {
            return 0;
        }
        let andop = def_of(data, shiftin);
        if code(data, andop) != OpCode::CPUI_INT_AND {
            return 0;
        }
        let maskvn = in_vn(data, andop, 1);
        if !is_constant(data, maskvn) {
            return 0;
        }
        let mut mask = offset_of(data, maskvn);
        let invn = in_vn(data, andop, 0);
        if is_free(data, invn) {
            return 0;
        }

        let mut opc = code(data, op);
        let sa: int4;
        if opc == OpCode::CPUI_INT_RIGHT || opc == OpCode::CPUI_INT_LEFT {
            sa = offset_of(data, cvn) as int4;
        } else {
            // Make sure the multiply is really a shift
            sa = leastsigbit_set(offset_of(data, cvn));
            if sa <= 0 {
                return 0;
            }
            let mut testval: uintb = 1;
            testval <<= sa as u32;
            if testval != offset_of(data, cvn) {
                return 0;
            }
            opc = OpCode::CPUI_INT_LEFT; // Treat CPUI_INT_MULT as CPUI_INT_LEFT
        }
        let mut nzm = nzmask_of(data, invn);
        let fullmask = calc_mask(size_of(data, invn));
        if opc == OpCode::CPUI_INT_RIGHT {
            // C++ `nzm >>= sa; mask >>= sa;` — `sa` is an int4 from a constant
            // operand and may be >= 64 (legal p-code) or have a negative (int4)
            // cast; on the x86 target the shift count is masked to `& 63`, so
            // use wshr (ADR 0003: wrapping shift mandatory for data-derived
            // counts). cast: matches C++ `(int4) >> ` count, x86 `& 63`.
            nzm = nzm.wshr(sa as u32);
            mask = mask.wshr(sa as u32);
        } else {
            nzm = nzm.wshl(sa as u32);
            mask = mask.wshl(sa as u32);
            nzm &= fullmask;
            mask &= fullmask;
        }
        if (mask & nzm) != nzm {
            return 0;
        }
        // data.opSetInput(op, invn, 0);  // Bypass the INT_AND
        data.op_set_input(op, invn, 0).expect("RuleShiftAnd: opSetInput");
        1
    }
}

// =============================================================================
// RuleConcatZero  (ruleaction.cc:4979)
// =============================================================================

/// \brief Simplify concatenation with zero: `concat(V,0) => zext(V) << c`
/// (C++ `RuleConcatZero`).
pub struct RuleConcatZero;

impl RuleConcatZero {
    /// Constructor (C++ `RuleConcatZero(const string &g)`).
    pub fn new() -> RuleConcatZero {
        RuleConcatZero
    }
}

impl Default for RuleConcatZero {
    fn default() -> Self {
        RuleConcatZero::new()
    }
}

impl Rule for RuleConcatZero {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_PIECE]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains("analysis") {
            return None;
        }
        Some(Box::new(RuleConcatZero::new()))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // if (!op->getIn(1)->isConstant()) return 0;
        let in1 = in_vn(data, op, 1);
        if !is_constant(data, in1) {
            return 0;
        }
        // if (op->getIn(1)->getOffset() != 0) return 0;
        if offset_of(data, in1) != 0 {
            return 0;
        }

        // int4 sa = 8*op->getIn(1)->getSize();
        let sa = 8 * size_of(data, in1);
        let highvn = in_vn(data, op, 0);
        let opaddr = addr_of(data, op);
        let newop = data.new_op(1, opaddr);
        let outvn = new_unique_out(data, size_of(data, out_vn(data, op)), newop);
        set_opcode(data, newop, OpCode::CPUI_INT_ZEXT);
        set_opcode(data, op, OpCode::CPUI_INT_LEFT);
        data.op_set_input(op, outvn, 0).expect("RuleConcatZero: opSetInput");
        let cvn = data.new_constant(4, sa as uintb);
        data.op_set_input(op, cvn, 1).expect("RuleConcatZero: opSetInput");
        data.op_set_input(newop, highvn, 0).expect("RuleConcatZero: opSetInput");
        data.op_insert_before(newop, op);
        1
    }
}

// =============================================================================
// RuleConcatLeftShift  (ruleaction.cc:5006)
// =============================================================================

/// \brief Simplify concatenation of extended value:
/// `concat(V, zext(W) << c) => concat( concat(V,W), 0)` (C++
/// `RuleConcatLeftShift`).
pub struct RuleConcatLeftShift;

impl RuleConcatLeftShift {
    /// Constructor (C++ `RuleConcatLeftShift(const string &g)`).
    pub fn new() -> RuleConcatLeftShift {
        RuleConcatLeftShift
    }
}

impl Default for RuleConcatLeftShift {
    fn default() -> Self {
        RuleConcatLeftShift::new()
    }
}

impl Rule for RuleConcatLeftShift {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_PIECE]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains("analysis") {
            return None;
        }
        Some(Box::new(RuleConcatLeftShift::new()))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // Varnode *vn2 = op->getIn(1); if (!vn2->isWritten()) return 0;
        let vn2 = in_vn(data, op, 1);
        if !is_written(data, vn2) {
            return 0;
        }
        let shiftop = def_of(data, vn2);
        if code(data, shiftop) != OpCode::CPUI_INT_LEFT {
            return 0;
        }
        let shift_sa = in_vn(data, shiftop, 1);
        if !is_constant(data, shift_sa) {
            return 0; // Must be a constant shift
        }
        let mut sa = offset_of(data, shift_sa) as int4;
        if (sa & 7) != 0 {
            return 0; // Not a multiple of 8
        }
        let tmpvn = in_vn(data, shiftop, 0);
        if !is_written(data, tmpvn) {
            return 0;
        }
        let zextop = def_of(data, tmpvn);
        if code(data, zextop) != OpCode::CPUI_INT_ZEXT {
            return 0;
        }
        let b = in_vn(data, zextop, 0);
        if is_free(data, b) {
            return 0;
        }
        let vn1 = in_vn(data, op, 0);
        if is_free(data, vn1) {
            return 0;
        }
        sa /= 8; // bits to bytes
        // if (sa + b->getSize() != tmpvn->getSize()) return 0;  // Must shift to most sig boundary
        if sa + size_of(data, b) != size_of(data, tmpvn) {
            return 0;
        }

        let opaddr = addr_of(data, op);
        let newop = data.new_op(2, opaddr);
        set_opcode(data, newop, OpCode::CPUI_PIECE);
        let newout = new_unique_out(data, size_of(data, vn1) + size_of(data, b), newop);
        data.op_set_input(newop, vn1, 0).expect("RuleConcatLeftShift: opSetInput");
        data.op_set_input(newop, b, 1).expect("RuleConcatLeftShift: opSetInput");
        data.op_insert_before(newop, op);
        data.op_set_input(op, newout, 0).expect("RuleConcatLeftShift: opSetInput");
        // opSetInput(op, newConstant(op->getOut()->getSize()-newout->getSize(), 0), 1);
        let csize = size_of(data, out_vn(data, op)) - size_of(data, newout);
        let cvn = data.new_constant(csize, 0);
        data.op_set_input(op, cvn, 1).expect("RuleConcatLeftShift: opSetInput");
        1
    }
}

// =============================================================================
// RuleSubZext  (ruleaction.cc:5046)
// =============================================================================

/// \brief Simplify INT_ZEXT applied to SUBPIECE expressions (C++ `RuleSubZext`):
///   - `zext( sub( V, 0) )        =>    V & mask`
///   - `zext( sub( V, c) )        =>    (V >> c*8) & mask`
///   - `zext( sub( V, c) >> d )   =>    (V >> (c*8+d)) & mask`
pub struct RuleSubZext;

impl RuleSubZext {
    /// Constructor (C++ `RuleSubZext(const string &g)`).
    pub fn new() -> RuleSubZext {
        RuleSubZext
    }
}

impl Default for RuleSubZext {
    fn default() -> Self {
        RuleSubZext::new()
    }
}

impl Rule for RuleSubZext {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_INT_ZEXT]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains("analysis") {
            return None;
        }
        Some(Box::new(RuleSubZext::new()))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // subvn = op->getIn(0); if (!subvn->isWritten()) return 0;
        let subvn = in_vn(data, op, 0);
        if !is_written(data, subvn) {
            return 0;
        }
        let mut subop = def_of(data, subvn);
        let outvn = out_vn(data, op);
        let out_size = size_of(data, outvn);
        if code(data, subop) == OpCode::CPUI_SUBPIECE {
            let basevn = in_vn(data, subop, 0);
            if is_free(data, basevn) {
                return 0;
            }
            // Truncating then extending to same size
            if size_of(data, basevn) != out_size {
                return 0;
            }
            if size_of(data, basevn) as usize > std::mem::size_of::<uintb>() {
                return 0;
            }
            let sub_in1 = in_vn(data, subop, 1);
            if offset_of(data, sub_in1) != 0 {
                // If truncating from middle: there must be no other use of subvn
                if data.lone_descend(subvn) != Some(op) {
                    return 0;
                }
                let basesize = size_of(data, basevn);
                let newvn = data.new_unique(basesize, None);
                let constvn = in_vn(data, subop, 1);
                let csize = size_of(data, constvn);
                let right_val = offset_of(data, constvn) * 8;
                data.op_set_input(op, newvn, 0).expect("RuleSubZext: opSetInput");
                set_opcode(data, subop, OpCode::CPUI_INT_RIGHT); // truncation -> shift
                let rc = data.new_constant(csize, right_val);
                data.op_set_input(subop, rc, 1).expect("RuleSubZext: opSetInput");
                op_set_output(data, subop, newvn);
            } else {
                data.op_set_input(op, basevn, 0).expect("RuleSubZext: opSetInput");
            }
            let val = calc_mask(size_of(data, subvn));
            let basesize = size_of(data, basevn);
            let constvn = data.new_constant(basesize, val);
            set_opcode(data, op, OpCode::CPUI_INT_AND);
            data.op_insert_input(op, constvn, 1).expect("RuleSubZext: opInsertInput");
            return 1;
        } else if code(data, subop) == OpCode::CPUI_INT_RIGHT {
            let shiftop = subop;
            let shift_sa = in_vn(data, shiftop, 1);
            if !is_constant(data, shift_sa) {
                return 0;
            }
            let midvn = in_vn(data, shiftop, 0);
            if !is_written(data, midvn) {
                return 0;
            }
            subop = def_of(data, midvn);
            if code(data, subop) != OpCode::CPUI_SUBPIECE {
                return 0;
            }
            let basevn = in_vn(data, subop, 0);
            if is_free(data, basevn) {
                return 0;
            }
            // Truncating then extending to same size
            if size_of(data, basevn) != out_size {
                return 0;
            }
            if data.lone_descend(midvn) != Some(shiftop) {
                return 0;
            }
            if data.lone_descend(subvn) != Some(op) {
                return 0;
            }
            let val = calc_mask(size_of(data, midvn)); // Mask based on truncated size
            let sa = offset_of(data, in_vn(data, shiftop, 1)); // shift shrinks the mask
            // C++ `val >>= sa;` — `sa` is a uintb from a constant operand and may
            // be >= 64 (legal p-code); the x86 target masks the count to `& 63`,
            // so use wshr (ADR 0003). cast: low 6 bits of `sa` are the x86 mask.
            let val = val.wshr(sa as u32);
            // sa += subop->getIn(1)->getOffset() * 8;  // total shift = truncation + small shift
            let sa = sa + offset_of(data, in_vn(data, subop, 1)) * 8;
            let basesize = size_of(data, basevn);
            let newvn = data.new_unique(basesize, None);
            data.op_set_input(op, newvn, 0).expect("RuleSubZext: opSetInput");
            data.op_set_input(shiftop, basevn, 0).expect("RuleSubZext: opSetInput"); // full value
            let shift_in1_size = size_of(data, in_vn(data, shiftop, 1));
            let sc = data.new_constant(shift_in1_size, sa); // by the combined amount
            data.op_set_input(shiftop, sc, 1).expect("RuleSubZext: opSetInput");
            op_set_output(data, shiftop, newvn);
            let constvn = data.new_constant(basesize, val);
            set_opcode(data, op, OpCode::CPUI_INT_AND); // Turn the ZEXT into an AND
            data.op_insert_input(op, constvn, 1).expect("RuleSubZext: opInsertInput");
            return 1;
        }
        0
    }
}

// =============================================================================
// RuleSubCancel  (ruleaction.cc:5122)
// =============================================================================

/// \brief Simplify composition of SUBPIECE with INT_ZEXT, INT_SEXT, and INT_AND
/// (C++ `RuleSubCancel`).  The SUBPIECE may partially or wholly cancel:
///   - `sub(zext(V),0)  =>  zext(V)` / `V` / `sub(V)`
///   - `sub(V & 0xffff, 0)  =>  sub(V)`
///   - corner: `sub(zext(V),c)  =>  0  when c is big enough`
pub struct RuleSubCancel;

impl RuleSubCancel {
    /// Constructor (C++ `RuleSubCancel(const string &g)`).
    pub fn new() -> RuleSubCancel {
        RuleSubCancel
    }
}

impl Default for RuleSubCancel {
    fn default() -> Self {
        RuleSubCancel::new()
    }
}

impl Rule for RuleSubCancel {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_SUBPIECE]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains("analysis") {
            return None;
        }
        Some(Box::new(RuleSubCancel::new()))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // base = op->getIn(0); if (!base->isWritten()) return 0;
        let base = in_vn(data, op, 0);
        if !is_written(data, base) {
            return 0;
        }
        let extop = def_of(data, base);
        let mut opc = code(data, extop);
        if opc != OpCode::CPUI_INT_ZEXT
            && opc != OpCode::CPUI_INT_SEXT
            && opc != OpCode::CPUI_INT_AND
        {
            return 0;
        }
        let offset = offset_of(data, in_vn(data, op, 1)) as int4;
        let outsize = size_of(data, out_vn(data, op));

        if opc == OpCode::CPUI_INT_AND {
            let cvn = in_vn(data, extop, 1);
            // if (offset==0 && cvn->isConstant() && cvn->getOffset()==calc_mask(outsize))
            if offset == 0 && is_constant(data, cvn) && offset_of(data, cvn) == calc_mask(outsize) {
                let thruvn = in_vn(data, extop, 0);
                if !is_free(data, thruvn) {
                    data.op_set_input(op, thruvn, 0).expect("RuleSubCancel: opSetInput");
                    return 1;
                }
            }
            return 0;
        }
        let insize = size_of(data, base);
        let farinsize = size_of(data, in_vn(data, extop, 0));

        let mut thruvn: VarnodeId;
        if offset == 0 {
            // SUBPIECE is of least sig part
            thruvn = in_vn(data, extop, 0); // Something still comes through
            if is_free(data, thruvn) {
                if is_constant(data, thruvn)
                    && (insize as usize > std::mem::size_of::<uintb>())
                    && (outsize == farinsize)
                {
                    // Constant too big to represent, elimination is total
                    opc = OpCode::CPUI_COPY;
                    let tsize = size_of(data, thruvn);
                    let toff = offset_of(data, thruvn);
                    thruvn = data.new_constant(tsize, toff); // with new constant varnode
                } else {
                    return 0; // original is constant or undefined: don't proceed
                }
            } else if outsize == farinsize {
                opc = OpCode::CPUI_COPY; // Total elimination of extension
            } else if outsize < farinsize {
                opc = OpCode::CPUI_SUBPIECE;
            }
        } else if (opc == OpCode::CPUI_INT_ZEXT) && (farinsize <= offset) {
            // output contains nothing of the original input
            opc = OpCode::CPUI_COPY; // Nothing but zero coming through
            thruvn = data.new_constant(outsize, 0);
        } else {
            // Missing one case here
            return 0;
        }

        set_opcode(data, op, opc); // SUBPIECE <- EXT replaced with one op
        data.op_set_input(op, thruvn, 0).expect("RuleSubCancel: opSetInput");

        if opc != OpCode::CPUI_SUBPIECE {
            data.op_remove_input(op, 1); // ZEXT, SEXT, or COPY has only 1 input
        }
        1
    }
}

// =============================================================================
// RuleShiftSub  (ruleaction.cc:5203)
// =============================================================================

/// \brief Simplify SUBPIECE applied to INT_LEFT:
/// `sub( V << 8*k, c) => sub(V,c-k)` (C++ `RuleShiftSub`).
pub struct RuleShiftSub;

impl RuleShiftSub {
    /// Constructor (C++ `RuleShiftSub(const string &g)`).
    pub fn new() -> RuleShiftSub {
        RuleShiftSub
    }
}

impl Default for RuleShiftSub {
    fn default() -> Self {
        RuleShiftSub::new()
    }
}

impl Rule for RuleShiftSub {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_SUBPIECE]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains("analysis") {
            return None;
        }
        Some(Box::new(RuleShiftSub::new()))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // if (!op->getIn(0)->isWritten()) return 0;
        let in0 = in_vn(data, op, 0);
        if !is_written(data, in0) {
            return 0;
        }
        let shiftop = def_of(data, in0);
        if code(data, shiftop) != OpCode::CPUI_INT_LEFT {
            return 0;
        }
        let sa = in_vn(data, shiftop, 1);
        if !is_constant(data, sa) {
            return 0;
        }
        let n = offset_of(data, sa) as int4;
        if (n & 7) != 0 {
            return 0; // Must shift by a multiple of 8 bits
        }
        let mut c = offset_of(data, in_vn(data, op, 1)) as int4;
        let vn = in_vn(data, shiftop, 0);
        if is_free(data, vn) {
            return 0;
        }
        let insize = size_of(data, vn);
        let outsize = size_of(data, out_vn(data, op));
        c -= n / 8;
        // if (c < 0 || c + outsize > insize) return 0;  // natural truncation?
        if c < 0 || c + outsize > insize {
            return 0;
        }
        data.op_set_input(op, vn, 0).expect("RuleShiftSub: opSetInput");
        let csize = size_of(data, in_vn(data, op, 1));
        let cvn = data.new_constant(csize, c as uintb);
        data.op_set_input(op, cvn, 1).expect("RuleShiftSub: opSetInput");
        1
    }
}

// =============================================================================
// RuleHumptyDumpty  (ruleaction.cc:5234)
// =============================================================================

/// \brief Simplify break and rejoin: `concat( sub(V,c), sub(V,0) ) => V`
/// (C++ `RuleHumptyDumpty`).  Variation: `concat( sub(V,c), sub(V,d) ) => sub(V,d)`.
pub struct RuleHumptyDumpty;

impl RuleHumptyDumpty {
    /// Constructor (C++ `RuleHumptyDumpty(const string &g)`).
    pub fn new() -> RuleHumptyDumpty {
        RuleHumptyDumpty
    }
}

impl Default for RuleHumptyDumpty {
    fn default() -> Self {
        RuleHumptyDumpty::new()
    }
}

impl Rule for RuleHumptyDumpty {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_PIECE]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains("analysis") {
            return None;
        }
        Some(Box::new(RuleHumptyDumpty::new()))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // vn1 = op->getIn(0); if (!vn1->isWritten()) return 0;
        let vn1 = in_vn(data, op, 0);
        if !is_written(data, vn1) {
            return 0;
        }
        let sub1 = def_of(data, vn1);
        if code(data, sub1) != OpCode::CPUI_SUBPIECE {
            return 0; // from piece1
        }
        let vn2 = in_vn(data, op, 1);
        if !is_written(data, vn2) {
            return 0;
        }
        let sub2 = def_of(data, vn2);
        if code(data, sub2) != OpCode::CPUI_SUBPIECE {
            return 0; // from piece2
        }

        // root = sub1->getIn(0); if (root != sub2->getIn(0)) return 0;
        let root = in_vn(data, sub1, 0);
        if root != in_vn(data, sub2, 0) {
            return 0; // pieces of the same whole
        }

        let pos1 = offset_of(data, in_vn(data, sub1, 1));
        let pos2 = offset_of(data, in_vn(data, sub2, 1));
        let size1 = size_of(data, vn1);
        let size2 = size_of(data, vn2);

        // if (pos1 != pos2 + size2) return 0;  // Pieces do not match up
        if pos1 != pos2 + (size2 as uintb) {
            return 0;
        }

        // if ((pos2==0)&&(size1+size2==root->getSize()))  // Pieced together whole thing
        if pos2 == 0 && (size1 + size2 == size_of(data, root)) {
            data.op_remove_input(op, 1);
            data.op_set_input(op, root, 0).expect("RuleHumptyDumpty: opSetInput");
            set_opcode(data, op, OpCode::CPUI_COPY);
        } else {
            // Pieced together a larger part of the whole
            data.op_set_input(op, root, 0).expect("RuleHumptyDumpty: opSetInput");
            let csize = size_of(data, in_vn(data, sub2, 1));
            let cvn = data.new_constant(csize, pos2);
            data.op_set_input(op, cvn, 1).expect("RuleHumptyDumpty: opSetInput");
            set_opcode(data, op, OpCode::CPUI_SUBPIECE);
        }
        1
    }
}

// =============================================================================
// RuleDumptyHump  (ruleaction.cc:5285)
// =============================================================================

/// \brief Simplify join and break apart: `sub( concat(V,W), c) => sub(W,c)`
/// (C++ `RuleDumptyHump`).  Variants:
///   - `sub( concat(V,W), 0)  =>  W`
///   - `sub( concat(V,W), c)  =>  V`
///   - `sub( concat(V,W), c)  =>  sub(V,c)`
pub struct RuleDumptyHump;

impl RuleDumptyHump {
    /// Constructor (C++ `RuleDumptyHump(const string &g)`).
    pub fn new() -> RuleDumptyHump {
        RuleDumptyHump
    }
}

impl Default for RuleDumptyHump {
    fn default() -> Self {
        RuleDumptyHump::new()
    }
}

impl Rule for RuleDumptyHump {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_SUBPIECE]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains("analysis") {
            return None;
        }
        Some(Box::new(RuleDumptyHump::new()))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // base = op->getIn(0); if (!base->isWritten()) return 0;
        let base = in_vn(data, op, 0);
        if !is_written(data, base) {
            return 0;
        }
        let pieceop = def_of(data, base);
        if code(data, pieceop) != OpCode::CPUI_PIECE {
            return 0;
        }
        let mut offset = offset_of(data, in_vn(data, op, 1)) as int4;
        let outsize = size_of(data, out_vn(data, op));

        let vn1 = in_vn(data, pieceop, 0);
        let vn2 = in_vn(data, pieceop, 1);

        let vn;
        if offset < size_of(data, vn2) {
            // Sub draws from vn2
            if offset + outsize > size_of(data, vn2) {
                return 0; // Also from vn1
            }
            vn = vn2;
        } else {
            // Sub draws from vn1
            vn = vn1;
            offset -= size_of(data, vn2); // offset relative to vn1
        }

        // if (vn->isFree() && (!vn->isConstant())) return 0;
        if is_free(data, vn) && !is_constant(data, vn) {
            return 0;
        }
        if (offset == 0) && (outsize == size_of(data, vn)) {
            // Eliminate SUB and CONCAT altogether
            set_opcode(data, op, OpCode::CPUI_COPY);
            data.op_remove_input(op, 1);
            data.op_set_input(op, vn, 0).expect("RuleDumptyHump: opSetInput"); // Skip over CONCAT
        } else {
            // Eliminate CONCAT and adjust SUB
            data.op_set_input(op, vn, 0).expect("RuleDumptyHump: opSetInput"); // Skip over CONCAT
            let cvn = data.new_constant(4, offset as uintb);
            data.op_set_input(op, cvn, 1).expect("RuleDumptyHump: opSetInput");
        }
        1
    }
}

// =============================================================================
// RuleHumptyOr  (ruleaction.cc:5341)
// =============================================================================

/// \brief Simplify masked pieces INT_ORed together:
/// `(V & ff00) | (V & 00ff) => V` (C++ `RuleHumptyOr`).  General form:
/// `(V & W) | (V & X) => V & (W|X)`.
pub struct RuleHumptyOr;

impl RuleHumptyOr {
    /// Constructor (C++ `RuleHumptyOr(const string &g)`).
    pub fn new() -> RuleHumptyOr {
        RuleHumptyOr
    }
}

impl Default for RuleHumptyOr {
    fn default() -> Self {
        RuleHumptyOr::new()
    }
}

impl Rule for RuleHumptyOr {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_INT_OR]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains("analysis") {
            return None;
        }
        Some(Box::new(RuleHumptyOr::new()))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // vn1 = op->getIn(0); if (!vn1->isWritten()) return 0;
        let vn1 = in_vn(data, op, 0);
        if !is_written(data, vn1) {
            return 0;
        }
        let vn2 = in_vn(data, op, 1);
        if !is_written(data, vn2) {
            return 0;
        }
        let and1 = def_of(data, vn1);
        if code(data, and1) != OpCode::CPUI_INT_AND {
            return 0;
        }
        let and2 = def_of(data, vn2);
        if code(data, and2) != OpCode::CPUI_INT_AND {
            return 0;
        }
        let mut a = in_vn(data, and1, 0);
        let mut b = in_vn(data, and1, 1);
        let mut c = in_vn(data, and2, 0);
        let d = in_vn(data, and2, 1);
        if a == c {
            c = d; // non-matching are b and d
        } else if a == d {
            // non-matching are b and c
        } else if b == c {
            // non-matching are a and d
            b = a;
            a = c;
            c = d;
        } else if b == d {
            // non-matching are a and c
            b = a;
            a = d;
        } else {
            return 0;
        }
        // Reaching here: a matches across both ANDs; b and c are the other params.
        // a is not free (>= two references).
        if is_constant(data, b) && is_constant(data, c) {
            let totalbits = offset_of(data, b) | offset_of(data, c);
            if totalbits == calc_mask(size_of(data, a)) {
                // All bits of a between the two sides. Convert to COPY
                set_opcode(data, op, OpCode::CPUI_COPY);
                data.op_remove_input(op, 1);
                data.op_set_input(op, a, 0).expect("RuleHumptyOr: opSetInput");
            } else {
                // Some bits, not all. Convert to an AND
                set_opcode(data, op, OpCode::CPUI_INT_AND);
                data.op_set_input(op, a, 0).expect("RuleHumptyOr: opSetInput");
                let asize = size_of(data, a);
                let newconst = data.new_constant(asize, totalbits);
                data.op_set_input(op, newconst, 1).expect("RuleHumptyOr: opSetInput");
            }
        } else {
            // if (!b->isHeritageKnown()) return 0;
            if !data.vbank().get(b).expect("RuleHumptyOr: stale b").is_heritage_known() {
                return 0;
            }
            if !data.vbank().get(c).expect("RuleHumptyOr: stale c").is_heritage_known() {
                return 0;
            }
            let a_mask = nzmask_of(data, a);
            // if ((b->getNZMask() & aMask)==0) return 0;  // RuleAndDistribute would reverse us
            if (nzmask_of(data, b) & a_mask) == 0 {
                return 0;
            }
            if (nzmask_of(data, c) & a_mask) == 0 {
                return 0;
            }
            let opaddr = addr_of(data, op);
            let new_or_op = data.new_op(2, opaddr);
            set_opcode(data, new_or_op, OpCode::CPUI_INT_OR);
            let or_vn = new_unique_out(data, size_of(data, a), new_or_op);
            data.op_set_input(new_or_op, b, 0).expect("RuleHumptyOr: opSetInput");
            data.op_set_input(new_or_op, c, 1).expect("RuleHumptyOr: opSetInput");
            data.op_insert_before(new_or_op, op);
            data.op_set_input(op, a, 0).expect("RuleHumptyOr: opSetInput");
            data.op_set_input(op, or_vn, 1).expect("RuleHumptyOr: opSetInput");
            set_opcode(data, op, OpCode::CPUI_INT_AND);
        }
        1
    }
}

// =============================================================================
// RuleSwitchSingle  (ruleaction.cc:5424)
// =============================================================================

/// \brief Convert BRANCHIND with only one computed destination to a BRANCH
/// (C++ `RuleSwitchSingle`).
///
/// STUB(W?-jumptable): the body needs `Funcdata::findJumpTable`,
/// `removeJumpTable`, `getStructure`, `newCodeRef`, and `warningHeader` plus the
/// `JumpTable` accessors — none ported to this wave's `Funcdata`.  The guard
/// `bb->sizeOut() != 1` and the no-jumptable early-out are transcribed; with
/// `findJumpTable` unported the rule short-circuits to "no jumptable" (the C++
/// `jt == 0` early-out).  Recorded as a loss.
pub struct RuleSwitchSingle;

impl RuleSwitchSingle {
    /// Constructor (C++ `RuleSwitchSingle(const string &g)`).
    pub fn new() -> RuleSwitchSingle {
        RuleSwitchSingle
    }
}

impl Default for RuleSwitchSingle {
    fn default() -> Self {
        RuleSwitchSingle::new()
    }
}

impl Rule for RuleSwitchSingle {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_BRANCHIND]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains("analysis") {
            return None;
        }
        Some(Box::new(RuleSwitchSingle::new()))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // BlockBasic *bb = op->getParent(); if (bb->sizeOut() != 1) return 0;
        let bb = match data.obank().get(op).expect("RuleSwitchSingle: stale op").get_parent() {
            Some(b) => b,
            None => return 0,
        };
        if data.bblocks_ref().block(bb).size_out() != 1 {
            return 0;
        }
        // JumpTable *jt = data.findJumpTable(op); if (jt == 0) return 0;
        //   -- STUB(W?-jumptable): findJumpTable + JumpTable accessors + newCodeRef
        //   + removeJumpTable + getStructure().clear() + warningHeader unported.
        //   The remaining body (label scan, BRANCHIND->BRANCH conversion) is
        //   transcribed in the C++ but unreachable until the jumptable surface lands.
        0
    }
}

// =============================================================================
// RuleCondNegate  (ruleaction.cc:5481)
// =============================================================================

/// \brief Flip conditions to match structuring cues (C++ `RuleCondNegate`).
///
/// Structuring assigns a preferred true/false meaning to branch directions; this
/// rule inserts a BOOL_NEGATE so the boolean calculation feeding a CBRANCH aligns.
///
/// STUB(W3-block): the body needs `Funcdata::opNormalizeFlip` and
/// `opFlipCondition` (the CBRANCH condition-flip block primitives) — not ported
/// to this wave's `Funcdata`.  The `isBooleanFlip` guard and the new-op shell are
/// transcribed; the two flip calls are routed to seam shims, so the BOOL_NEGATE
/// insertion path is reached but `opNormalizeFlip` always reports "no normalize"
/// and `opFlipCondition` is a no-op.  Recorded as a loss.
pub struct RuleCondNegate;

impl RuleCondNegate {
    /// Constructor (C++ `RuleCondNegate(const string &g)`).
    pub fn new() -> RuleCondNegate {
        RuleCondNegate
    }
}

impl Default for RuleCondNegate {
    fn default() -> Self {
        RuleCondNegate::new()
    }
}

impl Rule for RuleCondNegate {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_CBRANCH]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains("analysis") {
            return None;
        }
        Some(Box::new(RuleCondNegate::new()))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // if (!op->isBooleanFlip()) return 0;
        if !data.obank().get(op).expect("RuleCondNegate: stale op").is_boolean_flip() {
            return 0;
        }
        // if (data.opNormalizeFlip(op)) return 1;
        if data.op_normalize_flip(op).expect("RuleCondNegate: opNormalizeFlip") {
            return 1;
        }

        // vn = op->getIn(1);
        let vn = in_vn(data, op, 1);
        let opaddr = addr_of(data, op);
        let newop = data.new_op(1, opaddr);
        set_opcode(data, newop, OpCode::CPUI_BOOL_NEGATE);
        let outvn = new_unique_out(data, 1, newop); // Flipped version of varnode
        data.op_set_input(newop, vn, 0).expect("RuleCondNegate: opSetInput");
        data.op_set_input(op, outvn, 1).expect("RuleCondNegate: opSetInput");
        data.op_insert_before(newop, op);
        // data.opFlipCondition(op);  // Flip meaning of condition
        //   NOTE: fallthru block is still same status (the flag toggle clears the
        //   boolean_flip so the rule does not re-fire — the oscillation fix).
        data.op_flip_condition(op);
        let _ = seqnum_of(data, op); // keep the op-read helper in scope (no-op)
        1
    }
}

// =============================================================================
// Registration (W8 reads this — C++ DEFINITION ORDER)
// =============================================================================

/// The [`RuleSpec`]s for every rule in this file, in C++ definition order
/// (`ruleaction.cc:4293`..`5526`).  W8 splices these into the matching-group
/// [`ActionPool`]s of `universalAction` (most into `"analysis"`;
/// `RuleLoadVarnode`/`RuleStoreVarnode` into `"stackvars"`).
pub fn specs() -> Vec<RuleSpec> {
    vec![
        // RuleLoadVarnode / RuleStoreVarnode register under "stackvars"
        // (coreaction.cc:5939-5940, actprop2), not "analysis".
        RuleSpec { group: "stackvars", ctor: || Box::new(RuleLoadVarnode::new()) },
        RuleSpec { group: "stackvars", ctor: || Box::new(RuleStoreVarnode::new()) },
        RuleSpec { group: "analysis", ctor: || Box::new(RuleSubExtComm::new()) },
        RuleSpec { group: "analysis", ctor: || Box::new(RuleSubCommute::new()) },
        RuleSpec { group: "analysis", ctor: || Box::new(RuleConcatCommute::new()) },
        RuleSpec { group: "analysis", ctor: || Box::new(RuleConcatZext::new()) },
        RuleSpec { group: "analysis", ctor: || Box::new(RuleZextCommute::new()) },
        RuleSpec { group: "analysis", ctor: || Box::new(RuleZextShiftZext::new()) },
        RuleSpec { group: "analysis", ctor: || Box::new(RuleShiftAnd::new()) },
        RuleSpec { group: "analysis", ctor: || Box::new(RuleConcatZero::new()) },
        RuleSpec { group: "analysis", ctor: || Box::new(RuleConcatLeftShift::new()) },
        RuleSpec { group: "analysis", ctor: || Box::new(RuleSubZext::new()) },
        RuleSpec { group: "analysis", ctor: || Box::new(RuleSubCancel::new()) },
        RuleSpec { group: "analysis", ctor: || Box::new(RuleShiftSub::new()) },
        RuleSpec { group: "analysis", ctor: || Box::new(RuleHumptyDumpty::new()) },
        RuleSpec { group: "analysis", ctor: || Box::new(RuleDumptyHump::new()) },
        RuleSpec { group: "analysis", ctor: || Box::new(RuleHumptyOr::new()) },
        RuleSpec { group: "analysis", ctor: || Box::new(RuleSwitchSingle::new()) },
        RuleSpec { group: "analysis", ctor: || Box::new(RuleCondNegate::new()) },
    ]
}

#[cfg(test)]
mod tests;
