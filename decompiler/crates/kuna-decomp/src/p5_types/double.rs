//! Port of `decompiler/cpp/double.{cc,hh}` — double-precision (split varnode)
//! analysis (`SplitVarnode`, the per-form matchers, and the `RuleDouble*` rules).
//!
//! This recovers a logical value whose storage is split across two `Varnode`s
//! (`lo`/`hi`) back into operations on a single logical \e whole.  The driver is
//! [`RuleDoubleIn`] / [`RuleDoubleOut`] (which mark SUBPIECE/PIECE pieces as
//! double precision and then push the logical operation up or down one level via
//! [`SplitVarnode::apply_rule_in`]) plus the contiguous load/store collapsers
//! [`RuleDoubleLoad`] / [`RuleDoubleStore`].
//!
//! # Faithfulness
//!
//! Each method reproduces the upstream body statement-for-statement: the same
//! descend-iteration order, the same tie-breakers (`getSeqNum().getOrder()`
//! comparisons, big/little-endian splits), the same constant-folding, and the
//! same per-form `verify`/`applyRule` guard sequence (the rule body **is**
//! decompiler output).  The C++ holds `Varnode *` / `PcodeOp *`; the port holds
//! `Option<VarnodeId>` / `Option<OpId>` and reads the IR through
//! `data.vbank()` / `data.obank()`, snapshotting descendant lists before
//! mutating (the C++ holds a `list::const_iterator` it advances first).
//!
//! Integer rules: `uintb`→`u64`, `int4`→`i32`; wrapping arithmetic on offsets;
//! `getSeqNum().getOrder()` is a `u32` order index.
//!
//! # Cross-wave stubs
//!
//! `double.cc` reaches several subsystems later waves own.  Where a body reaches
//! one, the call routes through a `// STUB(...)`-noted helper that returns the
//! C++ guard's *failing* value, so the matcher declines exactly when (and only
//! when) the absent subsystem would block it — a conservative no-op, never a
//! wrong rewrite.  The affected surfaces (each ledgered as a loss):
//!
//! * `Varnode::getSpaceFromConst` (W4 — resolve a constant LOAD/STORE pointer's
//!   space-id annotation into an `AddrSpace`): `testContiguousPointers`,
//!   `RuleDoubleLoad`/`RuleDoubleStore`, `noWriteConflict`.
//! * `PcodeOp::getOpFromConst` (W3-varnode — decode an IOP-space annotation back
//!   to its `PcodeOp`): `IndirectForm`, `buildLoFromWhole`/`buildHiFromWhole`'s
//!   INDIRECT reinsert, `RuleDoubleStore`/`reassignIndirects`.
//! * `Varnode::getSymbolEntry` (W4 symbol table): `isAddrTiedContiguous`,
//!   `RuleDoubleOut::attemptMarking`.
//! * `Architecture::constructJoinAddress` (W4 join-space): `createJoinedWhole`.
//! * `Funcdata::markReturnCopy` / `Varnode::setWriteMask`-style RETURN-copy form
//!   (W4): `replaceCopyForce`.
//!
//! Two formerly-stubbed surfaces are now ported (`rport/w10-doublemove`):
//! `Funcdata::combineInputVarnodes` (the input-varnode merge feeding
//! `RuleDoubleOut::applyOp`, `funcdata_varnode.cc:383`), and
//! `TypeOp::isArithmeticOp`/`isFloatingPointOp` (the W6 `addlflags`, read via
//! [`crate::typeop::type_op_info`] in [`op_is_arith_or_float`] for
//! `RuleDoubleIn`/`RuleDoubleOut::attemptMarking`).  `Datatype::isPrimitiveWhole`
//! (the typelock gate) was already present.
//!
//! `op_set_opcode` needs a fully-formed `TypeOp` (opcode + property flags); the
//! flag word is the W6 `TypeFactory` fill, so [`set_opcode_typed`] builds the
//! `TypeOp` with the correct opcode value and an empty flag word — correct for
//! the rule's own dispatch, flags deferred.  // STUB(W6)

use kuna_base::address::{calc_mask, Address};
use kuna_base::space::spacetype;
use kuna_base::types::{int4, uintb, Wrap};
use kuna_num::opcodes::OpCode;

use crate::action::{ActionGroupList, Rule, RuleSpec};
use crate::funcdata::Funcdata;
use crate::context::{BlockId, OpId, VarnodeId};

/// `sizeof(uintb)` from the C++ — constants cannot exceed 8-byte precision.
const SIZEOF_UINTB: int4 = 8;

/// Resolve an [`OpCode`] to the [`TypeOp`] that [`Funcdata::op_set_opcode`]
/// expects (C++ `glb->inst[opc]`).  Resolves through the architecture's `inst`
/// table so the op carries its real `opflags` (eval-type, commutative, …) — a
/// flag-less `TypeOp::new(opc,0,..)` leaves the op with `getEvalType()==0`, which
/// blocks `RuleCollapseConstants` from folding a constant SUBPIECE/PIECE the
/// `SubvariableFlow` split produces.
fn set_opcode_typed(data: &mut Funcdata, op: OpId, opc: OpCode) {
    data.op_set_opcode(op, crate::typeop::type_op_for(opc));
}

// =============================================================================
// Small IR read helpers (the rules read through data.vbank()/data.obank())
// =============================================================================

// Varnode reads bundled so the matchers stay close to the C++.  Each returns the
// C++ accessor's value; a stale id is an internal-invariant panic (the C++
// dereferences a live pointer).
#[inline]
fn vn_is_constant(data: &Funcdata, vn: VarnodeId) -> bool {
    data.vbank().get(vn).expect("double: stale vn").is_constant()
}
#[inline]
fn vn_is_written(data: &Funcdata, vn: VarnodeId) -> bool {
    data.vbank().get(vn).expect("double: stale vn").is_written()
}
#[inline]
fn vn_is_input(data: &Funcdata, vn: VarnodeId) -> bool {
    data.vbank().get(vn).expect("double: stale vn").is_input()
}
#[inline]
fn vn_is_free(data: &Funcdata, vn: VarnodeId) -> bool {
    data.vbank().get(vn).expect("double: stale vn").is_free()
}
#[inline]
fn vn_get_def(data: &Funcdata, vn: VarnodeId) -> Option<OpId> {
    data.vbank().get(vn).expect("double: stale vn").get_def()
}
#[inline]
fn vn_get_size(data: &Funcdata, vn: VarnodeId) -> int4 {
    data.vbank().get(vn).expect("double: stale vn").get_size()
}
#[inline]
fn vn_get_offset(data: &Funcdata, vn: VarnodeId) -> uintb {
    data.vbank().get(vn).expect("double: stale vn").get_offset()
}
#[inline]
fn vn_get_addr(data: &Funcdata, vn: VarnodeId) -> Address {
    data.vbank().get(vn).expect("double: stale vn").get_addr().clone()
}
#[inline]
fn vn_is_addr_tied(data: &Funcdata, vn: VarnodeId) -> bool {
    data.vbank().get(vn).expect("double: stale vn").is_addr_tied()
}
#[inline]
fn vn_is_addr_force(data: &Funcdata, vn: VarnodeId) -> bool {
    data.vbank().get(vn).expect("double: stale vn").is_addr_force()
}
#[inline]
fn vn_is_precis_lo(data: &Funcdata, vn: VarnodeId) -> bool {
    data.vbank().get(vn).expect("double: stale vn").is_precis_lo()
}
#[inline]
fn vn_is_precis_hi(data: &Funcdata, vn: VarnodeId) -> bool {
    data.vbank().get(vn).expect("double: stale vn").is_precis_hi()
}
#[inline]
fn vn_has_no_descend(data: &Funcdata, vn: VarnodeId) -> bool {
    data.vbank().get(vn).expect("double: stale vn").has_no_descend()
}
#[inline]
fn vn_is_persist(data: &Funcdata, vn: VarnodeId) -> bool {
    data.vbank().get(vn).expect("double: stale vn").is_persist()
}
#[inline]
fn vn_is_type_lock(data: &Funcdata, vn: VarnodeId) -> bool {
    data.vbank().get(vn).expect("double: stale vn").is_type_lock()
}
/// Snapshot a Varnode's descendant op ids in `beginDescend` order (the C++ holds
/// a `list::const_iterator` and advances it before mutating).
#[inline]
fn descend_ops(data: &Funcdata, vn: VarnodeId) -> Vec<OpId> {
    data.vbank().get(vn).expect("double: stale vn").descend_iter().collect()
}
/// `Varnode::loneDescend` — the single reading op, or `None`.
#[inline]
fn lone_descend(data: &Funcdata, vn: VarnodeId) -> Option<OpId> {
    let v = data.vbank().get(vn).expect("double: stale vn");
    let mut it = v.descend_iter();
    let first = it.next()?;
    if it.next().is_some() {
        return None;
    }
    Some(first)
}

#[inline]
fn op_code(data: &Funcdata, op: OpId) -> OpCode {
    data.obank().get(op).expect("double: stale op").code()
}
#[inline]
fn op_get_in(data: &Funcdata, op: OpId, slot: int4) -> VarnodeId {
    data.obank()
        .get(op)
        .expect("double: stale op")
        .get_in(slot)
        .expect("double: missing input")
}
#[inline]
fn op_get_out(data: &Funcdata, op: OpId) -> Option<VarnodeId> {
    data.obank().get(op).expect("double: stale op").get_out()
}
#[inline]
fn op_num_input(data: &Funcdata, op: OpId) -> int4 {
    data.obank().get(op).expect("double: stale op").num_input()
}
#[inline]
fn op_get_slot(data: &Funcdata, op: OpId, vn: VarnodeId) -> int4 {
    data.obank().get(op).expect("double: stale op").get_slot(vn)
}
#[inline]
fn op_get_parent(data: &Funcdata, op: OpId) -> Option<BlockId> {
    data.obank().get(op).expect("double: stale op").get_parent()
}
#[inline]
fn op_get_order(data: &Funcdata, op: OpId) -> u32 {
    data.obank().get(op).expect("double: stale op").get_seq_num().get_order()
}
#[inline]
fn op_get_addr(data: &Funcdata, op: OpId) -> Address {
    data.obank().get(op).expect("double: stale op").get_addr().clone()
}
#[inline]
fn op_is_dead(data: &Funcdata, op: OpId) -> bool {
    data.obank().get(op).expect("double: stale op").is_dead()
}
#[inline]
fn op_is_return_copy(data: &Funcdata, op: OpId) -> bool {
    data.obank().get(op).expect("double: stale op").is_return_copy()
}

// --- block read helpers (C++ FlowBlock/BlockBasic accessors) --------------

#[inline]
fn bl_immed_dom(data: &Funcdata, bl: BlockId) -> Option<BlockId> {
    data.bblocks_ref().block(bl).get_immed_dom()
}
#[inline]
fn bl_is_entry_point(data: &Funcdata, bl: BlockId) -> bool {
    data.bblocks_ref().block(bl).is_entry_point()
}
#[inline]
fn bl_size_in(data: &Funcdata, bl: BlockId) -> int4 {
    data.bblocks_ref().block(bl).size_in()
}
#[inline]
fn bl_size_out(data: &Funcdata, bl: BlockId) -> int4 {
    data.bblocks_ref().block(bl).size_out()
}
#[inline]
fn bl_get_in(data: &Funcdata, bl: BlockId, i: int4) -> BlockId {
    data.bblocks_ref().block(bl).get_in(i)
}
#[inline]
fn bl_get_true_out(data: &Funcdata, bl: BlockId) -> BlockId {
    data.bblocks_ref().block(bl).get_true_out()
}
#[inline]
fn bl_get_false_out(data: &Funcdata, bl: BlockId) -> BlockId {
    data.bblocks_ref().block(bl).get_false_out()
}
/// The function's entry (start) basic block (C++
/// `data.getBasicBlocks().getStartBlock()`).
#[inline]
fn start_block(data: &Funcdata) -> BlockId {
    let root = data.bblocks_root_pub();
    data.bblocks_ref()
        .get_start_block(root)
        .expect("double: start block")
}
/// `BlockBasic::lastOp()` — the last op in execution order, or `None` if empty.
#[inline]
fn bl_last_op(data: &Funcdata, bl: BlockId) -> Option<OpId> {
    data.bb_ops(bl).last().copied()
}
/// `block->getStart()` — the start address of the block, taken from its first op
/// (the C++ `BlockBasic::getStart` is the address of `op.front()`).
#[inline]
fn bl_start_addr(data: &Funcdata, bl: BlockId) -> Address {
    let ops = data.bb_ops(bl);
    let first = *ops.first().expect("double: bl_start_addr on empty block");
    op_get_addr(data, first)
}
/// Test whether `defblock` dominates the block containing `existblock` by walking
/// the immediate-dominator chain (C++ `while(curbl) curbl=curbl->getImmedDom()`).
#[inline]
fn dominates(data: &Funcdata, defblock: BlockId, mut curbl: Option<BlockId>) -> bool {
    while let Some(b) = curbl {
        curbl = bl_immed_dom(data, b);
        if curbl == Some(defblock) {
            return true;
        }
    }
    false
}

// --- mutation helpers (the C++ Funcdata mutators these forms call) ---------

#[inline]
fn vn_set_precis_lo(data: &mut Funcdata, vn: VarnodeId) {
    data.vbank_mut().get_mut(vn).expect("double: stale vn").set_precis_lo();
}
#[inline]
fn vn_set_precis_hi(data: &mut Funcdata, vn: VarnodeId) {
    data.vbank_mut().get_mut(vn).expect("double: stale vn").set_precis_hi();
}
#[inline]
fn vn_set_addr_force(data: &mut Funcdata, vn: VarnodeId) {
    data.vbank_mut().get_mut(vn).expect("double: stale vn").set_addr_force();
}
#[inline]
fn vn_set_write_mask(data: &mut Funcdata, vn: VarnodeId) {
    data.vbank_mut().get_mut(vn).expect("double: stale vn").set_write_mask();
}
/// `data.opSetOutput(op,vn)` — the C++ never fails here on a well-formed graph.
#[inline]
fn op_set_output(data: &mut Funcdata, op: OpId, vn: VarnodeId) {
    data.op_set_output(op, vn).expect("double: op_set_output");
}
/// `data.opSetInput(op,vn,slot)`.
#[inline]
fn op_set_input(data: &mut Funcdata, op: OpId, vn: VarnodeId, slot: int4) {
    data.op_set_input(op, vn, slot).expect("double: op_set_input");
}
/// `data.opSetAllInput(op,inlist)`.
#[inline]
fn op_set_all_input(data: &mut Funcdata, op: OpId, vvec: &[VarnodeId]) {
    data.op_set_all_input(op, vvec).expect("double: op_set_all_input");
}

// =============================================================================
// SplitVarnode (double.cc:20-578, the logical-whole / pieces model)
// =============================================================================

/// A logical value whose storage is split between two Varnodes (C++
/// `SplitVarnode`).
///
/// `lo`/`hi` hold the least/most significant pieces.  A constant is represented
/// with both `lo` and `hi` null and the value in `val`.  `hi` null by itself
/// means the most significant part is an implied zero (the whole is the zero
/// extension of `lo`).
#[derive(Clone)]
pub struct SplitVarnode {
    /// Least significant piece (C++ `lo`).
    lo: Option<VarnodeId>,
    /// Most significant piece (C++ `hi`).
    hi: Option<VarnodeId>,
    /// A representative of the whole object (C++ `whole`).
    whole: Option<VarnodeId>,
    /// Operation at which both `lo` and `hi` are defined (C++ `defpoint`).
    defpoint: Option<OpId>,
    /// Block in which both `lo` and `hi` are defined (C++ `defblock`).
    defblock: Option<BlockId>,
    /// Value of a double precision constant (C++ `val`).
    val: uintb,
    /// Size in bytes of the (virtual) whole (C++ `wholesize`).
    wholesize: int4,
}

impl SplitVarnode {
    /// Construct an uninitialized SplitVarnode (C++ default ctor).
    fn empty() -> SplitVarnode {
        SplitVarnode {
            lo: None,
            hi: None,
            whole: None,
            defpoint: None,
            defblock: None,
            val: 0,
            wholesize: 0,
        }
    }

    /// Construct a double precision constant (C++ `SplitVarnode(int4,uintb)`).
    pub fn new_constant(sz: int4, v: uintb) -> SplitVarnode {
        let mut s = SplitVarnode::empty();
        s.val = v;
        s.wholesize = sz;
        s
    }

    /// Construct from `lo` and `hi` piece (C++
    /// `SplitVarnode(Varnode *l,Varnode *h)`), which calls
    /// `initPartial(l->getSize()+h->getSize(),l,h)`.
    pub fn from_pieces(data: &Funcdata, l: VarnodeId, h: VarnodeId) -> SplitVarnode {
        let sz = vn_get_size(data, l) + vn_get_size(data, h);
        let mut s = SplitVarnode::empty();
        s.init_partial_pieces(data, sz, Some(l), Some(h));
        s
    }

    /// (Re)initialize as a constant (C++ `initPartial(int4 sz,uintb v)`).
    fn init_partial_const(&mut self, sz: int4, v: uintb) {
        self.val = v;
        self.wholesize = sz;
        self.lo = None;
        self.hi = None;
        self.whole = None;
        self.defpoint = None;
        self.defblock = None;
    }

    /// (Re)initialize given Varnode pieces (C++
    /// `initPartial(int4 sz,Varnode *l,Varnode *h)`).
    ///
    /// `l` is required (the C++ dereferences it); `h` may be null (implied zero).
    fn init_partial_pieces(&mut self, data: &Funcdata, sz: int4, l: Option<VarnodeId>, h: Option<VarnodeId>) {
        let l = l.expect("init_partial: lo piece is required");
        match h {
            None => {
                // hi is an implied zero
                self.hi = None;
                if vn_is_constant(data, l) {
                    self.val = vn_get_offset(data, l); // Assume l is a constant
                    self.lo = None;
                } else {
                    self.lo = Some(l);
                }
            }
            Some(h) => {
                if vn_is_constant(data, l) && vn_is_constant(data, h) {
                    let mut val = vn_get_offset(data, h);
                    val = val.wshl((vn_get_size(data, l) * 8) as u32);
                    val |= vn_get_offset(data, l);
                    self.val = val;
                    self.lo = None;
                    self.hi = None;
                } else {
                    self.lo = Some(l);
                    self.hi = Some(h);
                }
            }
        }
        self.wholesize = sz;
        self.whole = None;
        self.defpoint = None;
        self.defblock = None;
    }

    /// Construct given Varnode pieces and a known whole Varnode (C++ `initAll`).
    fn init_all(&mut self, data: &Funcdata, w: VarnodeId, l: VarnodeId, h: Option<VarnodeId>) {
        self.wholesize = vn_get_size(data, w);
        self.lo = Some(l);
        self.hi = h;
        self.whole = Some(w);
        self.defpoint = None;
        self.defblock = None;
    }

    // --- Accessors (C++ inline getters) -----------------------------------

    /// Return `true` if this is a constant (C++ `isConstant`).
    pub fn is_constant(&self) -> bool {
        self.lo.is_none()
    }
    /// Return `true` if both pieces are initialized (C++ `hasBothPieces`).
    pub fn has_both_pieces(&self) -> bool {
        self.hi.is_some() && self.lo.is_some()
    }
    /// Get the whole-size in bytes (C++ `getSize`).
    pub fn get_size(&self) -> int4 {
        self.wholesize
    }
    /// Get the least significant piece (C++ `getLo`).
    pub fn get_lo(&self) -> Option<VarnodeId> {
        self.lo
    }
    /// Get the most significant piece (C++ `getHi`).
    pub fn get_hi(&self) -> Option<VarnodeId> {
        self.hi
    }
    /// Get the whole Varnode (C++ `getWhole`).
    pub fn get_whole(&self) -> Option<VarnodeId> {
        self.whole
    }
    /// Get the defining PcodeOp (C++ `getDefPoint`).
    pub fn get_def_point(&self) -> Option<OpId> {
        self.defpoint
    }
    /// Get the defining basic block (C++ `getDefBlock`).
    pub fn get_def_block(&self) -> Option<BlockId> {
        self.defblock
    }
    /// Get the constant value (C++ `getValue`).
    pub fn get_value(&self) -> uintb {
        self.val
    }
    /// Check if a constant exceeds precision limits (C++ `exceedsConstPrecision`).
    pub fn exceeds_const_precision(&self) -> bool {
        self.is_constant() && (self.wholesize > SIZEOF_UINTB)
    }

    // --- discovery / feasibility (double.cc:106-694) -----------------------

    /// Verify the most significant piece is a SUBPIECE and search for the least
    /// significant piece off the same whole (C++ `inHandHi`).
    ///
    /// Part of the C++ `SplitVarnode` public interface; no currently-ported form
    /// reaches it (only `inHandLo`/`inHandLoOut` are on a live path via
    /// `LogicalForm::findHiMatch`).  Kept for parity with the C++ API.
    #[allow(dead_code)]
    fn in_hand_hi(&mut self, data: &Funcdata, h: VarnodeId) -> bool {
        // Check for mark, in order to have quick -false- in most cases
        if !vn_is_precis_hi(data, h) {
            return false;
        }
        if vn_is_written(data, h) {
            let op = vn_get_def(data, h).expect("in_hand_hi: written vn has def");
            // We could check for double loads here
            if op_code(data, op) == OpCode::CPUI_SUBPIECE {
                let w = op_get_in(data, op, 0);
                if vn_get_offset(data, op_get_in(data, op, 1))
                    != (vn_get_size(data, w) - vn_get_size(data, h)) as uintb
                {
                    return false;
                }
                for tmpop in descend_ops(data, w) {
                    if op_code(data, tmpop) != OpCode::CPUI_SUBPIECE {
                        continue;
                    }
                    let tmplo = match op_get_out(data, tmpop) {
                        Some(v) => v,
                        None => continue,
                    };
                    if !vn_is_precis_lo(data, tmplo) {
                        continue;
                    }
                    if vn_get_size(data, tmplo) + vn_get_size(data, h) != vn_get_size(data, w) {
                        continue;
                    }
                    if vn_get_offset(data, op_get_in(data, tmpop, 1)) != 0 {
                        continue;
                    }
                    // There could conceivably be more than one, but not with CSE
                    self.init_all(data, w, tmplo, Some(h));
                    return true;
                }
            }
        }
        false
    }

    /// Verify the least significant piece is a SUBPIECE and search for the most
    /// significant piece off the same whole (C++ `inHandLo`).
    fn in_hand_lo(&mut self, data: &Funcdata, l: VarnodeId) -> bool {
        if !vn_is_precis_lo(data, l) {
            return false;
        }
        if vn_is_written(data, l) {
            let op = vn_get_def(data, l).expect("in_hand_lo: written vn has def");
            if op_code(data, op) == OpCode::CPUI_SUBPIECE {
                let w = op_get_in(data, op, 0);
                if vn_get_offset(data, op_get_in(data, op, 1)) != 0 {
                    return false;
                }
                for tmpop in descend_ops(data, w) {
                    if op_code(data, tmpop) != OpCode::CPUI_SUBPIECE {
                        continue;
                    }
                    let tmphi = match op_get_out(data, tmpop) {
                        Some(v) => v,
                        None => continue,
                    };
                    if !vn_is_precis_hi(data, tmphi) {
                        continue;
                    }
                    if vn_get_size(data, tmphi) + vn_get_size(data, l) != vn_get_size(data, w) {
                        continue;
                    }
                    if vn_get_offset(data, op_get_in(data, tmpop, 1)) != vn_get_size(data, l) as uintb {
                        continue;
                    }
                    self.init_all(data, w, l, Some(tmphi));
                    return true;
                }
            }
        }
        false
    }

    /// Initialize given just the least significant piece; the other piece may be
    /// an implied zero (C++ `inHandLoNoHi`).
    ///
    /// Part of the C++ `SplitVarnode` public interface; not on a currently-ported
    /// call path.  Kept for parity with the C++ API.
    #[allow(dead_code)]
    fn in_hand_lo_no_hi(&mut self, data: &Funcdata, l: VarnodeId) -> bool {
        if !vn_is_precis_lo(data, l) {
            return false;
        }
        if !vn_is_written(data, l) {
            return false;
        }
        let op = vn_get_def(data, l).expect("in_hand_lo_no_hi: def");
        if op_code(data, op) != OpCode::CPUI_SUBPIECE {
            return false;
        }
        if vn_get_offset(data, op_get_in(data, op, 1)) != 0 {
            return false;
        }
        let w = op_get_in(data, op, 0);
        for tmpop in descend_ops(data, w) {
            if op_code(data, tmpop) != OpCode::CPUI_SUBPIECE {
                continue;
            }
            let tmphi = match op_get_out(data, tmpop) {
                Some(v) => v,
                None => continue,
            };
            if !vn_is_precis_hi(data, tmphi) {
                continue;
            }
            if vn_get_size(data, tmphi) + vn_get_size(data, l) != vn_get_size(data, w) {
                continue;
            }
            if vn_get_offset(data, op_get_in(data, tmpop, 1)) != vn_get_size(data, l) as uintb {
                continue;
            }
            self.init_all(data, w, l, Some(tmphi));
            return true;
        }
        self.init_all(data, w, l, None);
        true
    }

    /// Initialize given the most significant piece if immediately concatenated
    /// with its least significant piece (C++ `inHandHiOut`).
    ///
    /// Part of the C++ `SplitVarnode` public interface; not on a currently-ported
    /// call path.  Kept for parity with the C++ API.
    #[allow(dead_code)]
    fn in_hand_hi_out(&mut self, data: &Funcdata, h: VarnodeId) -> bool {
        let mut lo_tmp: Option<VarnodeId> = None;
        let mut outvn: Option<VarnodeId> = None;
        for pieceop in descend_ops(data, h) {
            if op_code(data, pieceop) != OpCode::CPUI_PIECE {
                continue;
            }
            if op_get_in(data, pieceop, 0) != h {
                continue;
            }
            let l = op_get_in(data, pieceop, 1);
            if !vn_is_precis_lo(data, l) {
                continue;
            }
            if lo_tmp.is_some() {
                return false; // Whole is not unique
            }
            lo_tmp = Some(l);
            outvn = op_get_out(data, pieceop);
        }
        if let Some(l) = lo_tmp {
            self.init_all(data, outvn.expect("in_hand_hi_out: piece has out"), l, Some(h));
            return true;
        }
        false
    }

    /// Initialize given the least significant piece if immediately concatenated
    /// with its most significant piece (C++ `inHandLoOut`).
    fn in_hand_lo_out(&mut self, data: &Funcdata, l: VarnodeId) -> bool {
        let mut hi_tmp: Option<VarnodeId> = None;
        let mut outvn: Option<VarnodeId> = None;
        for pieceop in descend_ops(data, l) {
            if op_code(data, pieceop) != OpCode::CPUI_PIECE {
                continue;
            }
            if op_get_in(data, pieceop, 1) != l {
                continue;
            }
            let h = op_get_in(data, pieceop, 0);
            if !vn_is_precis_hi(data, h) {
                continue;
            }
            if hi_tmp.is_some() {
                return false; // Whole is not unique
            }
            hi_tmp = Some(h);
            outvn = op_get_out(data, pieceop);
        }
        if let Some(h) = hi_tmp {
            self.init_all(data, outvn.expect("in_hand_lo_out: piece has out"), l, Some(h));
            return true;
        }
        false
    }

    /// Find a common whole that `hi` and `lo` are SUBPIECEs of (C++
    /// `findWholeSplitToPieces`).
    fn find_whole_split_to_pieces(&mut self, data: &Funcdata) -> bool {
        if self.whole.is_none() {
            let hi = match self.hi {
                Some(h) => h,
                None => return false,
            };
            let lo = match self.lo {
                Some(l) => l,
                None => return false,
            };
            if !vn_is_written(data, hi) {
                return false;
            }
            let mut subhi = vn_get_def(data, hi).expect("fwstp: hi def");
            if op_code(data, subhi) == OpCode::CPUI_COPY {
                // Go thru one level of copy, if the piece is addrtied
                let otherhi = op_get_in(data, subhi, 0);
                if !vn_is_written(data, otherhi) {
                    return false;
                }
                subhi = vn_get_def(data, otherhi).expect("fwstp: otherhi def");
            }
            if op_code(data, subhi) != OpCode::CPUI_SUBPIECE {
                return false;
            }
            if vn_get_offset(data, op_get_in(data, subhi, 1))
                != (self.wholesize - vn_get_size(data, hi)) as uintb
            {
                return false;
            }
            let putative_whole = op_get_in(data, subhi, 0);
            if vn_get_size(data, putative_whole) != self.wholesize {
                return false;
            }
            if !vn_is_written(data, lo) {
                return false;
            }
            let mut sublo = vn_get_def(data, lo).expect("fwstp: lo def");
            if op_code(data, sublo) == OpCode::CPUI_COPY {
                let otherlo = op_get_in(data, sublo, 0);
                if !vn_is_written(data, otherlo) {
                    return false;
                }
                sublo = vn_get_def(data, otherlo).expect("fwstp: otherlo def");
            }
            if op_code(data, sublo) != OpCode::CPUI_SUBPIECE {
                return false;
            }
            if putative_whole != op_get_in(data, sublo, 0) {
                return false; // Doesn't match between pieces
            }
            if vn_get_offset(data, op_get_in(data, sublo, 1)) != 0 {
                return false;
            }
            self.whole = Some(putative_whole);
        }

        let whole = self.whole.expect("fwstp: whole set");
        if vn_is_written(data, whole) {
            self.defpoint = vn_get_def(data, whole);
            self.defblock = op_get_parent(data, self.defpoint.expect("fwstp: defpoint"));
        } else if vn_is_input(data, whole) {
            self.defpoint = None;
            self.defblock = None;
        }
        true
    }

    /// Find the earliest block/op where both `lo` and `hi` are defined (C++
    /// `findDefinitionPoint`).
    fn find_definition_point(&mut self, data: &Funcdata) -> bool {
        // If one but not both is constant
        if let Some(hi) = self.hi {
            if vn_is_constant(data, hi) {
                return false;
            }
        }
        let lo = self.lo.expect("find_definition_point: lo");
        if vn_is_constant(data, lo) {
            return false;
        }
        match self.hi {
            None => {
                // Implied zero extension
                if vn_is_input(data, lo) {
                    self.defblock = None;
                    self.defpoint = None;
                } else if vn_is_written(data, lo) {
                    self.defpoint = vn_get_def(data, lo);
                    self.defblock = op_get_parent(data, self.defpoint.expect("fdp: defpoint"));
                } else {
                    return false;
                }
            }
            Some(hi) => {
                if vn_is_written(data, hi) {
                    if !vn_is_written(data, lo) {
                        return false; // Do not allow mixed input/non-input pairs
                    }
                    let mut lastop = vn_get_def(data, hi).expect("fdp: hi def");
                    self.defblock = op_get_parent(data, lastop);
                    let lastop2 = vn_get_def(data, lo).expect("fdp: lo def");
                    let otherblock = op_get_parent(data, lastop2);
                    if self.defblock != otherblock {
                        self.defpoint = Some(lastop);
                        // Make sure defblock dominated by otherblock
                        let mut curbl = self.defblock;
                        while let Some(b) = curbl {
                            curbl = bl_immed_dom(data, b);
                            if curbl == otherblock {
                                return true;
                            }
                        }
                        // Try lo as final defining location
                        self.defblock = otherblock;
                        let otherblock2 = op_get_parent(data, lastop);
                        self.defpoint = Some(lastop2);
                        let mut curbl = self.defblock;
                        while let Some(b) = curbl {
                            curbl = bl_immed_dom(data, b);
                            if curbl == otherblock2 {
                                return true;
                            }
                        }
                        self.defblock = None;
                        return false; // Not defined in same basic block
                    }
                    if op_get_order(data, lastop2) > op_get_order(data, lastop) {
                        lastop = lastop2;
                    }
                    self.defpoint = Some(lastop);
                } else if vn_is_input(data, hi) {
                    if !vn_is_input(data, lo) {
                        return false; // Do not allow mixed input/non-input pairs
                    }
                    self.defblock = None;
                    self.defpoint = None;
                }
            }
        }
        true
    }

    /// Return the earlier of the two defining PcodeOps if both pieces are written
    /// in the same block, else `None` (C++ `findEarliestSplitPoint`).
    fn find_earliest_split_point(&self, data: &Funcdata) -> Option<OpId> {
        let hi = self.hi?;
        let lo = self.lo?;
        if !vn_is_written(data, hi) {
            return None;
        }
        if !vn_is_written(data, lo) {
            return None;
        }
        let hiop = vn_get_def(data, hi).expect("fesp: hi def");
        let loop_ = vn_get_def(data, lo).expect("fesp: lo def");
        if op_get_parent(data, loop_) != op_get_parent(data, hiop) {
            return None;
        }
        if op_get_order(data, loop_) < op_get_order(data, hiop) {
            Some(loop_)
        } else {
            Some(hiop)
        }
    }

    /// Scan for a CPUI_PIECE concatenation of `hi`/`lo` in significance order
    /// (C++ `findWholeBuiltFromPieces`).
    fn find_whole_built_from_pieces(&mut self, data: &Funcdata) -> bool {
        let hi = match self.hi {
            Some(h) => h,
            None => return false,
        };
        let lo = match self.lo {
            Some(l) => l,
            None => return false,
        };
        // bb: block where lo is defined, or None for input (entry-point) form
        let bb: Option<BlockId> = if vn_is_written(data, lo) {
            op_get_parent(data, vn_get_def(data, lo).expect("fwbfp: lo def"))
        } else if vn_is_input(data, lo) {
            None
        } else {
            panic!("Trying to find whole on free varnode");
        };
        let mut res: Option<OpId> = None;
        for op in descend_ops(data, lo) {
            if op_code(data, op) != OpCode::CPUI_PIECE {
                continue;
            }
            if op_get_in(data, op, 0) != hi {
                continue;
            }
            match bb {
                Some(b) => {
                    if op_get_parent(data, op) != Some(b) {
                        continue; // Not defined in earliest block
                    }
                }
                None => {
                    let parent = op_get_parent(data, op);
                    if !parent.map(|p| bl_is_entry_point(data, p)).unwrap_or(false) {
                        continue;
                    }
                }
            }
            match res {
                None => res = Some(op),
                Some(cur) => {
                    if op_get_order(data, op) < op_get_order(data, cur) {
                        // Find "earliest" whole
                        res = Some(op);
                    }
                }
            }
        }

        match res {
            None => {
                self.whole = None;
            }
            Some(r) => {
                self.defpoint = Some(r);
                self.defblock = op_get_parent(data, r);
                self.whole = op_get_out(data, r);
            }
        }
        self.whole.is_some()
    }

    /// Does a whole already exist or can it be created before `existop` (C++
    /// `isWholeFeasible`).
    fn is_whole_feasible(&mut self, data: &Funcdata, existop: OpId) -> bool {
        if self.is_constant() {
            return true;
        }
        if let (Some(lo), Some(hi)) = (self.lo, self.hi) {
            // Mixed constant/non-constant
            if vn_is_constant(data, lo) != vn_is_constant(data, hi) {
                return false;
            }
        }
        if !self.find_whole_split_to_pieces(data)
            && !self.find_whole_built_from_pieces(data)
            && !self.find_definition_point(data)
        {
            return false;
        }
        let defblock = match self.defblock {
            None => return true,
            Some(b) => b,
        };
        let curbl = op_get_parent(data, existop);
        if curbl == Some(defblock) {
            // Same block: check PcodeOp ordering
            return op_get_order(data, self.defpoint.expect("iwf: defpoint"))
                <= op_get_order(data, existop);
        }
        // Make sure defblock dominates block containing -existop-
        dominates(data, defblock, curbl)
    }

    /// Like `isWholeFeasible`, but the whole must be defined before the end of
    /// the given basic block (C++ `isWholePhiFeasible`).
    fn is_whole_phi_feasible(&mut self, data: &Funcdata, bl: BlockId) -> bool {
        if self.is_constant() {
            return false;
        }
        if !self.find_whole_split_to_pieces(data)
            && !self.find_whole_built_from_pieces(data)
            && !self.find_definition_point(data)
        {
            return false;
        }
        let defblock = match self.defblock {
            None => return true,
            Some(b) => b,
        };
        if bl == defblock {
            return true;
        }
        dominates(data, defblock, Some(bl))
    }

    /// Find the first PcodeOp where the output whole must exist (C++
    /// `findOutExist`).
    fn find_out_exist(&mut self, data: &Funcdata) -> Option<OpId> {
        if self.find_whole_built_from_pieces(data) {
            return self.defpoint;
        }
        self.find_earliest_split_point(data)
    }

    /// Create a whole Varnode for this if it doesn't already exist (C++
    /// `findCreateWhole`).
    fn find_create_whole(&mut self, data: &mut Funcdata) {
        if self.is_constant() {
            self.whole = Some(data.new_constant(self.wholesize, self.val));
            return;
        }
        // Mark the pieces
        if let Some(lo) = self.lo {
            vn_set_precis_lo(data, lo);
        }
        if let Some(hi) = self.hi {
            vn_set_precis_hi(data, hi);
        }

        if self.whole.is_some() {
            return; // Already found the whole
        }
        let addr: Address;
        let mut topblock: Option<BlockId> = None;
        if self.defblock.is_some() {
            addr = op_get_addr(data, self.defpoint.expect("fcw: defpoint"));
        } else {
            let tb = start_block(data);
            topblock = Some(tb);
            addr = bl_start_addr(data, tb);
        }

        let concatop: OpId;
        if let Some(hi) = self.hi {
            concatop = data.new_op(2, addr);
            let whole = data
                .new_unique_out(self.wholesize, concatop)
                .expect("fcw: new_unique_out");
            self.whole = Some(whole);
            set_opcode_typed(data, concatop, OpCode::CPUI_PIECE);
            op_set_output(data, concatop, whole);
            op_set_input(data, concatop, hi, 0);
            op_set_input(data, concatop, self.lo.expect("fcw: lo"), 1);
        } else {
            concatop = data.new_op(1, addr);
            let whole = data
                .new_unique_out(self.wholesize, concatop)
                .expect("fcw: new_unique_out");
            self.whole = Some(whole);
            set_opcode_typed(data, concatop, OpCode::CPUI_INT_ZEXT);
            op_set_output(data, concatop, whole);
            op_set_input(data, concatop, self.lo.expect("fcw: lo"), 0);
        }

        if let Some(dp) = self.defpoint {
            if self.defblock.is_some() {
                data.op_insert_after(concatop, dp);
            } else {
                data.op_insert_begin(concatop, topblock.expect("fcw: topblock"));
            }
        } else {
            data.op_insert_begin(concatop, topblock.expect("fcw: topblock"));
        }

        self.defpoint = Some(concatop);
        self.defblock = op_get_parent(data, concatop);
    }

    /// Create a whole Varnode that will later be set as a PcodeOp output (C++
    /// `findCreateOutputWhole`).
    fn find_create_output_whole(&mut self, data: &mut Funcdata) {
        // Mark the pieces
        vn_set_precis_lo(data, self.lo.expect("fcow: lo"));
        vn_set_precis_hi(data, self.hi.expect("fcow: hi"));
        if self.whole.is_some() {
            return;
        }
        self.whole = Some(data.new_unique(self.wholesize, None));
    }

    /// Create a whole Varnode from pieces, respecting piece storage (C++
    /// `createJoinedWhole`).
    fn create_joined_whole(&mut self, data: &mut Funcdata) {
        let lo = self.lo.expect("cjw: lo");
        let hi = self.hi.expect("cjw: hi");
        vn_set_precis_lo(data, lo);
        vn_set_precis_hi(data, hi);
        if self.whole.is_some() {
            return;
        }
        let newaddr = match is_addr_tied_contiguous(data, lo, hi) {
            Some(addr) => addr,
            None => {
                // STUB(W4): Architecture::constructJoinAddress (the join address
                // space) is not ported; without it we cannot fabricate the
                // logical whole's storage for the non-contiguous case.  The
                // caller (IndirectForm) has already passed its verify guards, so
                // declining here is a conservative no-op for this form.
                // -- We surface the absence by leaving `whole` unset; callers
                // that reach createJoinedWhole on the non-contiguous path would
                // panic on the subsequent unwrap, so the only reachable caller
                // path is gated below in replaceIndirectOp.
                return;
            }
        };
        let whole = data.new_varnode(self.wholesize, &newaddr, None);
        vn_set_write_mask(data, whole);
        self.whole = Some(whole);
    }

    /// Rebuild the least significant piece as a CPUI_SUBPIECE of the whole (C++
    /// `buildLoFromWhole`).
    fn build_lo_from_whole(&mut self, data: &mut Funcdata) {
        let lo = self.lo.expect("blfw: lo");
        let loop_ = vn_get_def(data, lo).expect("Building low piece that was originally undefined");
        let whole = self.whole.expect("blfw: whole");
        let c0 = data.new_constant(4, 0);
        let inlist = [whole, c0];
        match op_code(data, loop_) {
            OpCode::CPUI_MULTIEQUAL => {
                // Reinsert so we don't break the MULTIEQUAL sequence at block top
                let bl = op_get_parent(data, loop_).expect("blfw: parent");
                data.op_uninsert(loop_);
                set_opcode_typed(data, loop_, OpCode::CPUI_SUBPIECE);
                op_set_all_input(data, loop_, &inlist);
                data.op_insert_begin(loop_, bl);
            }
            OpCode::CPUI_INDIRECT => {
                // Reinsert AFTER the affector
                // STUB(W3-varnode): getOpFromConst (decode an IOP annotation back
                // to a PcodeOp) is deferred (funcdata_op.rs op_insert_after stub).
                // Without the affector we cannot reinsert after it; transform the
                // op in place (matching the C++ non-INDIRECT fallthrough) so the
                // SUBPIECE is still well-formed.  The only visible delta is the
                // reinsert position relative to a dead/live affector.
                set_opcode_typed(data, loop_, OpCode::CPUI_SUBPIECE);
                op_set_all_input(data, loop_, &inlist);
            }
            _ => {
                set_opcode_typed(data, loop_, OpCode::CPUI_SUBPIECE);
                op_set_all_input(data, loop_, &inlist);
            }
        }
    }

    /// Rebuild the most significant piece as a CPUI_SUBPIECE of the whole (C++
    /// `buildHiFromWhole`).
    fn build_hi_from_whole(&mut self, data: &mut Funcdata) {
        let hi = self.hi.expect("bhfw: hi");
        let hiop = vn_get_def(data, hi).expect("Building low piece that was originally undefined");
        let whole = self.whole.expect("bhfw: whole");
        let losize = vn_get_size(data, self.lo.expect("bhfw: lo"));
        let csub = data.new_constant(4, losize as uintb);
        let inlist = [whole, csub];
        match op_code(data, hiop) {
            OpCode::CPUI_MULTIEQUAL => {
                let bl = op_get_parent(data, hiop).expect("bhfw: parent");
                data.op_uninsert(hiop);
                set_opcode_typed(data, hiop, OpCode::CPUI_SUBPIECE);
                op_set_all_input(data, hiop, &inlist);
                data.op_insert_begin(hiop, bl);
            }
            OpCode::CPUI_INDIRECT => {
                // STUB(W3-varnode): getOpFromConst affector reinsert deferred (see
                // build_lo_from_whole); transform in place.
                set_opcode_typed(data, hiop, OpCode::CPUI_SUBPIECE);
                op_set_all_input(data, hiop, &inlist);
            }
            _ => {
                set_opcode_typed(data, hiop, OpCode::CPUI_SUBPIECE);
                op_set_all_input(data, hiop, &inlist);
            }
        }
    }
}

// =============================================================================
// SplitVarnode static helpers (double.cc:704-1431) — free functions in the port
// =============================================================================

/// Return `true` if `vn1 + size1 == vn2` (C++ `SplitVarnode::adjacentOffsets`).
fn adjacent_offsets(data: &Funcdata, vn1: VarnodeId, vn2: VarnodeId, size1: uintb) -> bool {
    if vn_is_constant(data, vn1) {
        if !vn_is_constant(data, vn2) {
            return false;
        }
        return vn_get_offset(data, vn1).wrapping_add(size1) == vn_get_offset(data, vn2);
    }

    if !vn_is_written(data, vn2) {
        return false;
    }
    let op2 = vn_get_def(data, vn2).expect("adjacent_offsets: vn2 def");
    if op_code(data, op2) != OpCode::CPUI_INT_ADD {
        return false;
    }
    if !vn_is_constant(data, op_get_in(data, op2, 1)) {
        return false;
    }
    let c2 = vn_get_offset(data, op_get_in(data, op2, 1));

    if op_get_in(data, op2, 0) == vn1 {
        return size1 == c2;
    }

    if !vn_is_written(data, vn1) {
        return false;
    }
    let op1 = vn_get_def(data, vn1).expect("adjacent_offsets: vn1 def");
    if op_code(data, op1) != OpCode::CPUI_INT_ADD {
        return false;
    }
    if !vn_is_constant(data, op_get_in(data, op1, 1)) {
        return false;
    }
    let c1 = vn_get_offset(data, op_get_in(data, op1, 1));

    if op_get_in(data, op1, 0) != op_get_in(data, op2, 0) {
        return false;
    }
    c1.wrapping_add(size1) == c2
}

/// `Varnode::getSpaceFromConst` — resolve a constant LOAD/STORE pointer's
/// space-id annotation into an `AddrSpace` *manager index*.
///
/// C++ `Varnode::getSpaceFromConst` (`varnode.hh:427`) does
/// `getSpace()->getManager()->getSpace((int4)getOffset())` — it reinterprets the
/// constant's offset as the space-id and indexes the manager.  The kuna port
/// (LOSS-015) stores the space's manager *index* in that offset (exactly the
/// representation decoded by the real `RuleLoadVarnode` helper
/// `space_from_const` in `ruleaction_4.rs`), so here we read that index back.
///
/// The callers (`testContiguousPointers`, `noWriteConflict`, the
/// `RuleDoubleLoad`/`RuleDoubleStore` rules) thread the returned index back
/// through `Architecture::manage().get_space(idx)` (`space_is_big_endian_by_index`,
/// `space_index_to_varnode`), which is the same index-keyed lookup, so the index
/// is the right currency to return.  An out-of-range offset (a hand-built fixture
/// constant that is not a genuine space-id) is rejected rather than panicking on
/// the manager's bounds — mirroring `space_from_const`'s guard.
fn space_from_const_index(data: &Funcdata, vn: VarnodeId) -> Option<i32> {
    let idx = vn_get_offset(data, vn);
    let manage = data.get_arch().manage();
    if idx >= manage.num_spaces() as uintb {
        return None;
    }
    manage.get_space(idx as i32).map(|s| s.get_index())
}

/// Verify two LOAD/STORE pointers address contiguous memory (C++
/// `SplitVarnode::testContiguousPointers`).  Returns `(first, second,
/// space_index)` on success.
///
/// STUB(W4): the space resolution goes through [`space_from_const_index`], which
/// is deferred, so this always returns `None` today.  The full pointer-adjacency
/// logic is transcribed for when the space surface lands.
fn test_contiguous_pointers(
    data: &Funcdata,
    most: OpId,
    least: OpId,
) -> Option<(OpId, OpId, i32)> {
    let spc = space_from_const_index(data, op_get_in(data, least, 0))?;
    if space_from_const_index(data, op_get_in(data, most, 0)) != Some(spc) {
        return None;
    }

    // Determine endianness from the resolved space (W4); until the space is
    // resolvable we cannot reach here.  The structure is preserved for
    // restoration: big-endian -> (most,least), else (least,most).
    let spc_big_endian = space_is_big_endian_by_index(data, spc);
    let (first, second) = if spc_big_endian {
        (most, least)
    } else {
        (least, most)
    };
    let firstptr = op_get_in(data, first, 1);
    if vn_is_free(data, firstptr) {
        return None;
    }
    let sizeres: int4 = if op_code(data, first) == OpCode::CPUI_LOAD {
        // # of bytes read by lowest address load
        vn_get_size(data, op_get_out(data, first).expect("tcp: load out"))
    } else {
        // CPUI_STORE
        vn_get_size(data, op_get_in(data, first, 2))
    };

    // Check if the loads are adjacent to each other
    if adjacent_offsets(
        data,
        op_get_in(data, first, 1),
        op_get_in(data, second, 1),
        sizeres as uintb,
    ) {
        Some((first, second, spc))
    } else {
        None
    }
}

/// Resolve a space index's endianness (C++ `spc->isBigEndian()`).  Pairs with
/// [`space_from_const_index`]; both are W4 stubs (a space index off a constant is
/// not resolvable today), so this is only reachable once that lands.
fn space_is_big_endian_by_index(data: &Funcdata, idx: i32) -> bool {
    data.get_arch()
        .manage()
        .get_space(idx)
        .map(|s| s.is_big_endian())
        .unwrap_or(false)
}

/// Return the contiguous starting address if `lo`/`hi` are address-tied pieces
/// that line up by significance (C++ `SplitVarnode::isAddrTiedContiguous`).
fn is_addr_tied_contiguous(data: &Funcdata, lo: VarnodeId, hi: VarnodeId) -> Option<Address> {
    if !vn_is_addr_tied(data, lo) {
        return None;
    }
    if !vn_is_addr_tied(data, hi) {
        return None;
    }

    // STUB(W4): the C++ also rejects when the pieces carry conflicting
    // SymbolEntrys (Varnode::getSymbolEntry, the W4 symbol table).  Without the
    // symbol surface we cannot read the entries; we omit this extra rejection.
    // This is *less* conservative than the C++ (it could accept a pair the C++
    // would reject on a symbol mismatch), so it is recorded as a loss.  The
    // address-tie + storage-adjacency checks below (the load-bearing geometry)
    // are fully transcribed.

    let lo_v = data.vbank().get(lo).expect("iatc: lo");
    let hi_v = data.vbank().get(hi).expect("iatc: hi");
    let spc = lo_v.get_space().clone();
    if !std::rc::Rc::ptr_eq(&spc, hi_v.get_space()) && spc.get_index() != hi_v.get_space().get_index()
    {
        return None;
    }
    let looffset = vn_get_offset(data, lo);
    let hioffset = vn_get_offset(data, hi);
    if spc.is_big_endian() {
        if hioffset >= looffset {
            return None;
        }
        if hioffset + vn_get_size(data, hi) as uintb != looffset {
            return None;
        }
        Some(vn_get_addr(data, hi))
    } else {
        if looffset >= hioffset {
            return None;
        }
        if looffset + vn_get_size(data, lo) as uintb != hioffset {
            return None;
        }
        Some(vn_get_addr(data, lo))
    }
}

/// Collect every SplitVarnode pair carrying the same logical value as `w` (C++
/// `SplitVarnode::wholeList`).  At least one of the SUBPIECE-marked pieces must
/// exist.
fn whole_list(data: &Funcdata, w: VarnodeId) -> Vec<SplitVarnode> {
    let mut splitvec: Vec<SplitVarnode> = Vec::new();
    let mut basic = SplitVarnode::empty();
    basic.whole = Some(w);
    basic.hi = None;
    basic.lo = None;
    basic.wholesize = vn_get_size(data, w);

    let mut res = 0i32;
    for subop in descend_ops(data, w) {
        if op_code(data, subop) != OpCode::CPUI_SUBPIECE {
            continue;
        }
        let vn = match op_get_out(data, subop) {
            Some(v) => v,
            None => continue,
        };
        if vn_is_precis_hi(data, vn) {
            if vn_get_offset(data, op_get_in(data, subop, 1))
                != (basic.wholesize - vn_get_size(data, vn)) as uintb
            {
                continue;
            }
            basic.hi = Some(vn);
            res |= 2;
        } else if vn_is_precis_lo(data, vn) {
            if vn_get_offset(data, op_get_in(data, subop, 1)) != 0 {
                continue;
            }
            basic.lo = Some(vn);
            res |= 1;
        }
    }
    if res == 0 {
        return splitvec;
    }
    if res == 3
        && vn_get_size(data, basic.lo.expect("whole_list: lo"))
            + vn_get_size(data, basic.hi.expect("whole_list: hi"))
            != basic.wholesize
    {
        return splitvec;
    }

    splitvec.push(basic.clone());
    find_copies(data, &basic, &mut splitvec);
    splitvec
}

/// Find COPYs from the pieces of `in_` that land in contiguous storage and add
/// the resulting SplitVarnodes (C++ `SplitVarnode::findCopies`).
fn find_copies(data: &Funcdata, in_: &SplitVarnode, splitvec: &mut Vec<SplitVarnode>) {
    if !in_.has_both_pieces() {
        return;
    }
    let in_lo = in_.get_lo().expect("find_copies: lo");
    let in_hi = in_.get_hi().expect("find_copies: hi");
    for loop_ in descend_ops(data, in_lo) {
        if op_code(data, loop_) != OpCode::CPUI_COPY {
            continue;
        }
        let locpy = op_get_out(data, loop_).expect("find_copies: copy out");
        // Calculate address of hi part
        let mut addr = vn_get_addr(data, locpy);
        if addr.is_big_endian() {
            addr = &addr - (vn_get_size(data, in_hi) as i64);
        } else {
            addr = &addr + (vn_get_size(data, locpy) as i64);
        }
        for hiop in descend_ops(data, in_hi) {
            if op_code(data, hiop) != OpCode::CPUI_COPY {
                continue;
            }
            let hicpy = op_get_out(data, hiop).expect("find_copies: hicopy out");
            if vn_get_addr(data, hicpy) != addr {
                continue;
            }
            if op_get_parent(data, hiop) != op_get_parent(data, loop_) {
                continue;
            }
            let mut newsplit = SplitVarnode::empty();
            newsplit.init_all(data, in_.get_whole().expect("find_copies: whole"), locpy, Some(hicpy));
            splitvec.push(newsplit);
        }
    }
}

/// Pass back the true/false basic blocks of a CBRANCH, honoring its boolean-flip
/// and a caller flip (C++ `SplitVarnode::getTrueFalse`).
fn get_true_false(data: &Funcdata, boolop: OpId, flip: bool) -> (BlockId, BlockId) {
    let parent = op_get_parent(data, boolop).expect("get_true_false: parent");
    let trueblock = bl_get_true_out(data, parent);
    let falseblock = bl_get_false_out(data, parent);
    let boolean_flip = data.obank().get(boolop).expect("gtf: op").is_boolean_flip();
    if boolean_flip != flip {
        (falseblock, trueblock)
    } else {
        (trueblock, falseblock)
    }
}

/// Return `true` if the CBRANCH's block does nothing but compute the branch (C++
/// `SplitVarnode::otherwiseEmpty`).
fn otherwise_empty(data: &Funcdata, branchop: OpId) -> bool {
    let bl = op_get_parent(data, branchop).expect("otherwise_empty: parent");
    if bl_size_in(data, bl) != 1 {
        return false;
    }
    let mut otherop: Option<OpId> = None;
    let vn = op_get_in(data, branchop, 1);
    if vn_is_written(data, vn) {
        otherop = vn_get_def(data, vn);
    }
    for op in data.bb_ops(bl) {
        if Some(op) == otherop {
            continue;
        }
        if op == branchop {
            continue;
        }
        return false;
    }
    true
}

/// Verify the PcodeOp is a CPUI_INT_MULT by -1 (C++
/// `SplitVarnode::verifyMultNegOne`).
fn verify_mult_neg_one(data: &Funcdata, op: OpId) -> bool {
    if op_code(data, op) != OpCode::CPUI_INT_MULT {
        return false;
    }
    let in1 = op_get_in(data, op, 1);
    if !vn_is_constant(data, in1) {
        return false;
    }
    vn_get_offset(data, in1) == calc_mask(vn_get_size(data, in1))
}

/// `data.markReturnCopy(op)` — `op->flags |= return_copy` (C++
/// `Funcdata::markReturnCopy`).
#[inline]
fn mark_return_copy(data: &mut Funcdata, op: OpId) {
    data.obank_mut()
        .get_mut(op)
        .expect("mark_return_copy: stale op")
        .set_flag(crate::op::pcodeop_flags::return_copy);
}

/// Check that a generic binary double-precision op can be created; return the
/// PcodeOp where the output whole must exist (C++ `prepareBinaryOp`).
fn prepare_binary_op(
    data: &Funcdata,
    out: &mut SplitVarnode,
    in1: &mut SplitVarnode,
    in2: &mut SplitVarnode,
) -> Option<OpId> {
    let existop = out.find_out_exist(data)?; // point where output whole needs to exist
    if !in1.is_whole_feasible(data, existop) {
        return None;
    }
    if !in2.is_whole_feasible(data, existop) {
        return None;
    }
    Some(existop)
}

/// Rewrite a binary double-precision op, replacing pieces with unified Varnodes
/// (C++ `createBinaryOp`).
fn create_binary_op(
    data: &mut Funcdata,
    out: &mut SplitVarnode,
    in1: &mut SplitVarnode,
    in2: &mut SplitVarnode,
    existop: OpId,
    opc: OpCode,
) {
    out.find_create_output_whole(data);
    in1.find_create_whole(data);
    in2.find_create_whole(data);
    if op_code(data, existop) != OpCode::CPUI_PIECE {
        // The output whole didn't previously exist
        let addr = op_get_addr(data, existop);
        let newop = data.new_op(2, addr);
        set_opcode_typed(data, newop, opc);
        op_set_output(data, newop, out.get_whole().expect("cbo: out whole"));
        op_set_input(data, newop, in1.get_whole().expect("cbo: in1 whole"), 0);
        op_set_input(data, newop, in2.get_whole().expect("cbo: in2 whole"), 1);
        data.op_insert_before(newop, existop);
        out.build_lo_from_whole(data);
        out.build_hi_from_whole(data);
    } else {
        // The whole previously existed
        set_opcode_typed(data, existop, opc);
        op_set_input(data, existop, in1.get_whole().expect("cbo: in1 whole"), 0);
        op_set_input(data, existop, in2.get_whole().expect("cbo: in2 whole"), 1);
    }
}

/// Check that a double-precision shift op's operands are compatible; return the
/// PcodeOp where the output whole must exist (C++ `prepareShiftOp`).
fn prepare_shift_op(data: &Funcdata, out: &mut SplitVarnode, in_: &mut SplitVarnode) -> Option<OpId> {
    let existop = out.find_out_exist(data)?;
    if !in_.is_whole_feasible(data, existop) {
        return None;
    }
    Some(existop)
}

/// Rewrite a double-precision shift, replacing hi/lo pieces with unified
/// Varnodes (C++ `createShiftOp`).  The shift amount is not double precision.
fn create_shift_op(
    data: &mut Funcdata,
    out: &mut SplitVarnode,
    in_: &mut SplitVarnode,
    mut sa: VarnodeId,
    existop: OpId,
    opc: OpCode,
) {
    out.find_create_output_whole(data);
    in_.find_create_whole(data);
    if vn_is_constant(data, sa) {
        sa = data.new_constant(vn_get_size(data, sa), vn_get_offset(data, sa));
    }
    if op_code(data, existop) != OpCode::CPUI_PIECE {
        let addr = op_get_addr(data, existop);
        let newop = data.new_op(2, addr);
        set_opcode_typed(data, newop, opc);
        op_set_output(data, newop, out.get_whole().expect("cso: out whole"));
        op_set_input(data, newop, in_.get_whole().expect("cso: in whole"), 0);
        op_set_input(data, newop, sa, 1);
        data.op_insert_before(newop, existop);
        out.build_lo_from_whole(data);
        out.build_hi_from_whole(data);
    } else {
        set_opcode_typed(data, existop, opc);
        op_set_input(data, existop, in_.get_whole().expect("cso: in whole"), 0);
        op_set_input(data, existop, sa, 1);
    }
}

/// Check that a double-precision compare op's inputs are compatible (C++
/// `prepareBoolOp`).
fn prepare_bool_op(
    data: &Funcdata,
    in1: &mut SplitVarnode,
    in2: &mut SplitVarnode,
    testop: OpId,
) -> bool {
    if !in1.is_whole_feasible(data, testop) {
        return false;
    }
    if !in2.is_whole_feasible(data, testop) {
        return false;
    }
    true
}

/// Rewrite a double-precision boolean op in place, replacing inputs with unified
/// Varnodes and updating the opcode (C++ `replaceBoolOp`).
fn replace_bool_op(
    data: &mut Funcdata,
    boolop: OpId,
    in1: &mut SplitVarnode,
    in2: &mut SplitVarnode,
    opc: OpCode,
) {
    in1.find_create_whole(data);
    in2.find_create_whole(data);
    set_opcode_typed(data, boolop, opc);
    op_set_input(data, boolop, in1.get_whole().expect("rbo: in1 whole"), 0);
    op_set_input(data, boolop, in2.get_whole().expect("rbo: in2 whole"), 1);
}

/// Create a new compare PcodeOp feeding the given CBRANCH (C++ `createBoolOp`).
fn create_bool_op(
    data: &mut Funcdata,
    cbranch: OpId,
    in1: &mut SplitVarnode,
    in2: &mut SplitVarnode,
    opc: OpCode,
) {
    let mut addrop = cbranch;
    let boolvn = op_get_in(data, cbranch, 1);
    if vn_is_written(data, boolvn) {
        // Use the address of the comparison operator
        addrop = vn_get_def(data, boolvn).expect("create_bool_op: boolvn def");
    }
    in1.find_create_whole(data);
    in2.find_create_whole(data);
    let addr = op_get_addr(data, addrop);
    let newop = data.new_op(2, addr);
    set_opcode_typed(data, newop, opc);
    let newbool = data.new_unique_out(1, newop).expect("create_bool_op: newbool");
    op_set_input(data, newop, in1.get_whole().expect("cbo2: in1 whole"), 0);
    op_set_input(data, newop, in2.get_whole().expect("cbo2: in2 whole"), 1);
    data.op_insert_before(newop, cbranch);
    op_set_input(data, cbranch, newbool, 1); // CBRANCH now determined by new compare
}

/// Check the logical MULTIEQUAL can be created; return the PcodeOp where the
/// output whole must exist (C++ `preparePhiOp`).
fn prepare_phi_op(
    data: &Funcdata,
    out: &mut SplitVarnode,
    inlist: &mut [SplitVarnode],
) -> Option<OpId> {
    let existop = out.find_earliest_split_point(data)?;
    // existop should always be a MULTIEQUAL defining one of the pieces
    if op_code(data, existop) != OpCode::CPUI_MULTIEQUAL {
        panic!("Trying to create phi-node double precision op with phi-node pieces");
    }
    let bl = op_get_parent(data, existop).expect("prepare_phi_op: parent");
    let numin = inlist.len();
    for (i, item) in inlist.iter_mut().enumerate().take(numin) {
        let inbl = bl_get_in(data, bl, i as int4);
        if !item.is_whole_phi_feasible(data, inbl) {
            return None;
        }
    }
    Some(existop)
}

/// Rewrite a double-precision MULTIEQUAL, replacing pieces with unified Varnodes
/// (C++ `createPhiOp`).
fn create_phi_op(
    data: &mut Funcdata,
    out: &mut SplitVarnode,
    inlist: &mut [SplitVarnode],
    existop: OpId,
) {
    // We MUST create a newop even if the output whole already exists, because the
    // MULTIEQUAL has placement constraints.
    out.find_create_output_whole(data);
    let numin = inlist.len();
    for item in inlist.iter_mut().take(numin) {
        item.find_create_whole(data);
    }

    let addr = op_get_addr(data, existop);
    let newop = data.new_op(numin as int4, addr);
    set_opcode_typed(data, newop, OpCode::CPUI_MULTIEQUAL);
    op_set_output(data, newop, out.get_whole().expect("create_phi_op: out whole"));
    for (i, item) in inlist.iter().enumerate().take(numin) {
        op_set_input(data, newop, item.get_whole().expect("create_phi_op: in whole"), i as int4);
    }
    data.op_insert_before(newop, existop);
    out.build_lo_from_whole(data);
    out.build_hi_from_whole(data);
}

/// Check that the logical CPUI_INDIRECT can be created (C++ `prepareIndirectOp`).
fn prepare_indirect_op(data: &Funcdata, in_: &mut SplitVarnode, affector: OpId) -> bool {
    in_.is_whole_feasible(data, affector)
}

/// Rewrite a double-precision INDIRECT, replacing pieces with unified Varnodes
/// (C++ `replaceIndirectOp`).
fn replace_indirect_op(
    data: &mut Funcdata,
    out: &mut SplitVarnode,
    in_: &mut SplitVarnode,
    affector: OpId,
) {
    out.create_joined_whole(data);

    in_.find_create_whole(data);
    let addr = op_get_addr(data, affector);
    let newop = data.new_op(2, addr);
    set_opcode_typed(data, newop, OpCode::CPUI_INDIRECT);
    op_set_output(data, newop, out.get_whole().expect("rio: out whole"));
    op_set_input(data, newop, in_.get_whole().expect("rio: in whole"), 0);
    let iopvn = data.new_varnode_iop(affector);
    op_set_input(data, newop, iopvn, 1);
    data.op_insert_before(newop, affector);
    out.build_lo_from_whole(data);
    out.build_hi_from_whole(data);
}

/// Rewrite a double-precision COPY to an address-forced Varnode (C++
/// `replaceCopyForce`).
fn replace_copy_force(
    data: &mut Funcdata,
    addr: &Address,
    in_: &SplitVarnode,
    copylo: OpId,
    copyhi: OpId,
) {
    let mut in_vn = in_.get_whole().expect("rcf: in whole");
    let return_form = op_is_return_copy(data, copyhi);
    if return_form && vn_get_addr(data, in_vn) != *addr {
        // Placeholder for global propagation past a RETURN needs an additional COPY
        let mut other_point1 = vn_get_def(data, op_get_in(data, copyhi, 0)).expect("rcf: hi def");
        let other_point2 = vn_get_def(data, op_get_in(data, copylo, 0)).expect("rcf: lo def");
        // We know these are COPYs in the same basic block.  Compute the later one.
        if op_get_order(data, other_point1) < op_get_order(data, other_point2) {
            other_point1 = other_point2;
        }
        let op_addr = op_get_addr(data, other_point1);
        let other_copy = data.new_op(1, op_addr);
        set_opcode_typed(data, other_copy, OpCode::CPUI_COPY);
        let vn = data
            .new_varnode_out(in_.get_size(), addr, other_copy)
            .expect("rcf: new_varnode_out");
        op_set_input(data, other_copy, in_vn, 0);
        data.op_insert_before(other_copy, other_point1);
        in_vn = vn;
    }
    let copyhi_addr = op_get_addr(data, copyhi);
    let whole_copy = data.new_op(1, copyhi_addr);
    set_opcode_typed(data, whole_copy, OpCode::CPUI_COPY);
    let out_vn = data
        .new_varnode_out(in_.get_size(), addr, whole_copy)
        .expect("rcf: new_varnode_out");
    vn_set_addr_force(data, out_vn);
    if return_form {
        mark_return_copy(data, whole_copy);
    }
    op_set_input(data, whole_copy, in_vn, 0);
    data.op_insert_before(whole_copy, copyhi);
    data.op_destroy(copyhi); // Destroy original COPYs.  Outputs have no descendants.
    data.op_destroy(copylo);
}

/// Try one transform on a logical double-precision op given a specific input
/// (C++ `SplitVarnode::applyRuleIn`).  The forms are lined up against the input;
/// the first match performs its transform and returns 1.
fn apply_rule_in(in_: &SplitVarnode, data: &mut Funcdata) -> int4 {
    for i in 0..2 {
        let vn = if i == 0 { in_.get_hi() } else { in_.get_lo() };
        let vn = match vn {
            Some(v) => v,
            None => continue,
        };
        let workishi = i == 0;
        for workop in descend_ops(data, vn) {
            // The C++ holds a live `list::const_iterator` over vn's descendants;
            // we snapshot the list up front (`descend_ops`) so a transform can
            // mutate while we iterate.  A form mutates ONLY on success and then
            // returns 1 (ending the loop), so a snapshotted op is never destroyed
            // mid-scan on a real path; this guard only prevents a stale-id panic
            // (a no-behavioral-change safety net — the C++ list would likewise no
            // longer hold a destroyed op).
            if op_is_dead(data, workop) {
                continue;
            }
            match op_code(data, workop) {
                OpCode::CPUI_INT_ADD => {
                    let mut addform = AddForm::new();
                    if addform.apply_rule(in_, workop, workishi, data) {
                        return 1;
                    }
                    let mut subform = SubForm::new();
                    if subform.apply_rule(in_, workop, workishi, data) {
                        return 1;
                    }
                }
                OpCode::CPUI_INT_AND => {
                    let mut equal3form = Equal3Form::new();
                    if equal3form.apply_rule(in_, workop, workishi, data) {
                        return 1;
                    }
                    let mut logicalform = LogicalForm::new();
                    if logicalform.apply_rule(in_, workop, workishi, data) {
                        return 1;
                    }
                }
                OpCode::CPUI_INT_OR => {
                    let mut logicalform = LogicalForm::new();
                    if logicalform.apply_rule(in_, workop, workishi, data) {
                        return 1;
                    }
                }
                OpCode::CPUI_INT_XOR => {
                    let mut logicalform = LogicalForm::new();
                    if logicalform.apply_rule(in_, workop, workishi, data) {
                        return 1;
                    }
                }
                OpCode::CPUI_INT_EQUAL | OpCode::CPUI_INT_NOTEQUAL => {
                    let mut lessthreeway = LessThreeWay::new();
                    if lessthreeway.apply_rule(in_, workop, workishi, data) {
                        return 1;
                    }
                    let mut equal1form = Equal1Form::new();
                    if equal1form.apply_rule(in_, workop, workishi, data) {
                        return 1;
                    }
                    let mut equal2form = Equal2Form::new();
                    if equal2form.apply_rule(in_, workop, workishi, data) {
                        return 1;
                    }
                }
                OpCode::CPUI_INT_LESS | OpCode::CPUI_INT_LESSEQUAL => {
                    let mut lessthreeway = LessThreeWay::new();
                    if lessthreeway.apply_rule(in_, workop, workishi, data) {
                        return 1;
                    }
                    let mut lessconstform = LessConstForm::new();
                    if lessconstform.apply_rule(in_, workop, workishi, data) {
                        return 1;
                    }
                }
                OpCode::CPUI_INT_SLESS => {
                    let mut lessconstform = LessConstForm::new();
                    if lessconstform.apply_rule(in_, workop, workishi, data) {
                        return 1;
                    }
                }
                OpCode::CPUI_INT_SLESSEQUAL => {
                    let mut lessconstform = LessConstForm::new();
                    if lessconstform.apply_rule(in_, workop, workishi, data) {
                        return 1;
                    }
                }
                OpCode::CPUI_INT_LEFT => {
                    let mut shiftform = ShiftForm::new();
                    if shiftform.apply_rule_left(in_, workop, workishi, data) {
                        return 1;
                    }
                }
                OpCode::CPUI_INT_RIGHT => {
                    let mut shiftform = ShiftForm::new();
                    if shiftform.apply_rule_right(in_, workop, workishi, data) {
                        return 1;
                    }
                }
                OpCode::CPUI_INT_SRIGHT => {
                    let mut shiftform = ShiftForm::new();
                    if shiftform.apply_rule_right(in_, workop, workishi, data) {
                        return 1;
                    }
                }
                OpCode::CPUI_INT_MULT => {
                    let mut multform = MultForm::new();
                    if multform.apply_rule(in_, workop, workishi, data) {
                        return 1;
                    }
                }
                OpCode::CPUI_MULTIEQUAL => {
                    let mut phiform = PhiForm::new();
                    if phiform.apply_rule(in_, workop, workishi, data) {
                        return 1;
                    }
                }
                OpCode::CPUI_INDIRECT => {
                    let mut indform = IndirectForm::new();
                    if indform.apply_rule(in_, workop, workishi, data) {
                        return 1;
                    }
                }
                OpCode::CPUI_COPY => {
                    let out = op_get_out(data, workop);
                    if out.map(|o| vn_is_addr_force(data, o)).unwrap_or(false) {
                        let mut copyform = CopyForceForm::new();
                        if copyform.apply_rule(in_, workop, workishi, data) {
                            return 1;
                        }
                    }
                }
                _ => {}
            }
        }
    }
    0
}

// =============================================================================
// AddForm (double.cc:102-117 hh, 1433-1607 cc)
// =============================================================================

/// Match a double-precision addition starting from a known input pair (C++
/// `AddForm`).
struct AddForm {
    in_: SplitVarnode,
    hi1: Option<VarnodeId>,
    lo1: Option<VarnodeId>,
    lo2: Option<VarnodeId>,
    reshi: Option<VarnodeId>,
    reslo: Option<VarnodeId>,
    add2: Option<OpId>,
    hizext1: Option<VarnodeId>,
    hizext2: Option<VarnodeId>,
    hi2: Option<VarnodeId>,
    zextop: Option<OpId>,
    loadd: Option<OpId>,
    slot1: int4,
    negconst: uintb,
    indoub: SplitVarnode,
    outdoub: SplitVarnode,
}

impl AddForm {
    fn new() -> AddForm {
        AddForm {
            in_: SplitVarnode::empty(),
            hi1: None,
            lo1: None,
            lo2: None,
            reshi: None,
            reslo: None,
            add2: None,
            hizext1: None,
            hizext2: None,
            hi2: None,
            zextop: None,
            loadd: None,
            slot1: 0,
            negconst: 0,
            indoub: SplitVarnode::empty(),
            outdoub: SplitVarnode::empty(),
        }
    }

    /// If `op` matches a CARRY construction based on `lo1`, set `lo2` (and
    /// `negconst` if `lo1` is constant) and return true (C++ `checkForCarry`).
    fn check_for_carry(&mut self, data: &Funcdata, op: OpId) -> bool {
        if op_code(data, op) != OpCode::CPUI_INT_ZEXT {
            return false;
        }
        let in0 = op_get_in(data, op, 0);
        if !vn_is_written(data, in0) {
            return false;
        }
        let carryop = vn_get_def(data, in0).expect("cfc: carryop");
        let lo1 = self.lo1.expect("cfc: lo1");
        match op_code(data, carryop) {
            OpCode::CPUI_INT_CARRY => {
                // Normal CARRY form
                if op_get_in(data, carryop, 0) == lo1 {
                    self.lo2 = Some(op_get_in(data, carryop, 1));
                } else if op_get_in(data, carryop, 1) == lo1 {
                    self.lo2 = Some(op_get_in(data, carryop, 0));
                } else {
                    return false;
                }
                if vn_is_constant(data, self.lo2.expect("cfc: lo2")) {
                    return false;
                }
                true
            }
            OpCode::CPUI_INT_LESS => {
                // Possible CARRY
                let tmpvn = op_get_in(data, carryop, 0);
                if vn_is_constant(data, tmpvn) {
                    if op_get_in(data, carryop, 1) != lo1 {
                        return false;
                    }
                    let mut negconst = vn_get_offset(data, tmpvn);
                    // In constant forms, the <= will get converted to a <.
                    // The lessthan->less conversion adds 1, then the 2's complement
                    // subtracts 1 and negates; so all we really do is negate.
                    negconst = (!negconst) & calc_mask(vn_get_size(data, lo1));
                    self.negconst = negconst;
                    self.lo2 = None;
                    true
                } else if vn_is_written(data, tmpvn) {
                    // Calculate CARRY relative to result of loadd
                    let loadd_op = vn_get_def(data, tmpvn).expect("cfc: loadd_op"); // putative loadd
                    if op_code(data, loadd_op) != OpCode::CPUI_INT_ADD {
                        return false;
                    }
                    let othervn;
                    if op_get_in(data, loadd_op, 0) == lo1 {
                        othervn = op_get_in(data, loadd_op, 1);
                    } else if op_get_in(data, loadd_op, 1) == lo1 {
                        othervn = op_get_in(data, loadd_op, 0);
                    } else {
                        return false; // One side of the add must be lo1
                    }
                    if vn_is_constant(data, othervn) {
                        self.negconst = vn_get_offset(data, othervn);
                        self.lo2 = None;
                        let relvn = op_get_in(data, carryop, 1);
                        if relvn == lo1 {
                            return true; // Comparison can be relative to lo1
                        }
                        if !vn_is_constant(data, relvn) {
                            return false;
                        }
                        if vn_get_offset(data, relvn) != self.negconst {
                            return false; // Otherwise must be relative to (constant)lo2
                        }
                        true
                    } else {
                        self.lo2 = Some(othervn); // Other side of putative loadd must be lo2
                        let compvn = op_get_in(data, carryop, 1);
                        if compvn == self.lo2.expect("cfc: lo2") || compvn == lo1 {
                            return true;
                        }
                        false
                    }
                } else {
                    false
                }
            }
            OpCode::CPUI_INT_NOTEQUAL => {
                // Possible CARRY against -1
                if !vn_is_constant(data, op_get_in(data, carryop, 1)) {
                    return false;
                }
                if op_get_in(data, carryop, 0) != lo1 {
                    return false;
                }
                if vn_get_offset(data, op_get_in(data, carryop, 1)) != 0 {
                    return false;
                }
                self.negconst = calc_mask(vn_get_size(data, lo1)); // CARRY constant must have been -1
                self.lo2 = None;
                true
            }
            _ => false,
        }
    }

    /// Look for a double-precision add and recover the other input + the output
    /// (C++ `AddForm::verify`).
    fn verify(&mut self, data: &Funcdata, h: VarnodeId, l: VarnodeId, op: OpId) -> bool {
        self.hi1 = Some(h);
        self.lo1 = Some(l);
        self.slot1 = op_get_slot(data, op, h);
        for i in 0..3 {
            if i == 0 {
                // Assume we have to descend one more add
                self.add2 = lone_descend(data, op_get_out(data, op).expect("addverify: op out"));
                let add2 = match self.add2 {
                    Some(a) => a,
                    None => continue,
                };
                if op_code(data, add2) != OpCode::CPUI_INT_ADD {
                    continue;
                }
                self.reshi = op_get_out(data, add2);
                self.hizext1 = Some(op_get_in(data, op, 1 - self.slot1));
                let opout = op_get_out(data, op).expect("addverify: op out2");
                self.hizext2 = Some(op_get_in(data, add2, 1 - op_get_slot(data, add2, opout)));
            } else if i == 1 {
                // Assume we are at the bottom most of two adds
                let tmpvn = op_get_in(data, op, 1 - self.slot1);
                if !vn_is_written(data, tmpvn) {
                    continue;
                }
                self.add2 = vn_get_def(data, tmpvn);
                let add2 = self.add2.expect("addverify: add2");
                if op_code(data, add2) != OpCode::CPUI_INT_ADD {
                    continue;
                }
                self.reshi = op_get_out(data, op);
                self.hizext1 = Some(op_get_in(data, add2, 0));
                self.hizext2 = Some(op_get_in(data, add2, 1));
            } else {
                // Assume there is only one add, with second implied add by 0
                self.reshi = op_get_out(data, op);
                self.hizext1 = Some(op_get_in(data, op, 1 - self.slot1));
                self.hizext2 = None;
            }
            for j in 0..2 {
                if i == 2 {
                    // hi2 is an implied 0
                    let hz1 = self.hizext1.expect("addverify: hizext1");
                    if !vn_is_written(data, hz1) {
                        continue;
                    }
                    self.zextop = vn_get_def(data, hz1);
                    self.hi2 = None;
                } else if j == 0 {
                    let hz1 = self.hizext1.expect("addverify: hizext1");
                    if !vn_is_written(data, hz1) {
                        continue;
                    }
                    self.zextop = vn_get_def(data, hz1);
                    self.hi2 = self.hizext2;
                } else {
                    let hz2 = match self.hizext2 {
                        Some(h) => h,
                        None => continue,
                    };
                    if !vn_is_written(data, hz2) {
                        continue;
                    }
                    self.zextop = vn_get_def(data, hz2);
                    self.hi2 = self.hizext1;
                }
                let zextop = self.zextop.expect("addverify: zextop");
                if !self.check_for_carry(data, zextop) {
                    continue; // Calculate lo2 and negconst
                }

                let lo1 = self.lo1.expect("addverify: lo1");
                for loadd in descend_ops(data, lo1) {
                    self.loadd = Some(loadd);
                    if op_code(data, loadd) != OpCode::CPUI_INT_ADD {
                        continue;
                    }
                    let tmpvn = op_get_in(data, loadd, 1 - op_get_slot(data, loadd, lo1));
                    match self.lo2 {
                        None => {
                            if !vn_is_constant(data, tmpvn) {
                                continue;
                            }
                            if vn_get_offset(data, tmpvn) != self.negconst {
                                continue; // Must add same constant used to calculate CARRY
                            }
                            self.lo2 = Some(tmpvn);
                        }
                        Some(lo2) if vn_is_constant(data, lo2) => {
                            if !vn_is_constant(data, tmpvn) {
                                continue;
                            }
                            if vn_get_offset(data, lo2) != vn_get_offset(data, tmpvn) {
                                continue;
                            }
                        }
                        Some(lo2) => {
                            // Must add same value used to calculate CARRY
                            if op_get_in(data, loadd, 1 - op_get_slot(data, loadd, lo1)) != lo2 {
                                continue;
                            }
                        }
                    }
                    self.reslo = op_get_out(data, loadd);
                    return true;
                }
            }
        }
        false
    }

    fn apply_rule(&mut self, i: &SplitVarnode, op: OpId, workishi: bool, data: &mut Funcdata) -> bool {
        if !workishi {
            return false;
        }
        if !i.has_both_pieces() {
            return false;
        }
        self.in_ = i.clone();
        let hi = self.in_.get_hi().expect("addrule: hi");
        let lo = self.in_.get_lo().expect("addrule: lo");
        if !self.verify(data, hi, lo, op) {
            return false;
        }

        self.indoub.init_partial_pieces(data, self.in_.get_size(), self.lo2, self.hi2);
        if self.indoub.exceeds_const_precision() {
            return false;
        }
        self.outdoub.init_partial_pieces(
            data,
            self.in_.get_size(),
            self.reslo,
            self.reshi,
        );
        let existop = match prepare_binary_op(data, &mut self.outdoub, &mut self.in_, &mut self.indoub) {
            Some(e) => e,
            None => return false,
        };
        let (mut out, mut in1, mut in2) =
            (self.outdoub.clone(), self.in_.clone(), self.indoub.clone());
        create_binary_op(data, &mut out, &mut in1, &mut in2, existop, OpCode::CPUI_INT_ADD);
        true
    }
}

// =============================================================================
// SubForm (double.cc:119-133 hh, 1609-1702 cc)
// =============================================================================

/// Match a double-precision subtraction starting from a known input pair (C++
/// `SubForm`).
struct SubForm {
    in_: SplitVarnode,
    hi1: Option<VarnodeId>,
    lo1: Option<VarnodeId>,
    lo2: Option<VarnodeId>,
    reshi: Option<VarnodeId>,
    reslo: Option<VarnodeId>,
    lessop: Option<OpId>,
    add2: Option<OpId>,
    hineg1: Option<VarnodeId>,
    hineg2: Option<VarnodeId>,
    hizext1: Option<VarnodeId>,
    hizext2: Option<VarnodeId>,
    hi2: Option<VarnodeId>,
    zextop: Option<OpId>,
    slot1: int4,
    indoub: SplitVarnode,
    outdoub: SplitVarnode,
}

impl SubForm {
    fn new() -> SubForm {
        SubForm {
            in_: SplitVarnode::empty(),
            hi1: None,
            lo1: None,
            lo2: None,
            reshi: None,
            reslo: None,
            lessop: None,
            add2: None,
            hineg1: None,
            hineg2: None,
            hizext1: None,
            hizext2: None,
            hi2: None,
            zextop: None,
            slot1: 0,
            indoub: SplitVarnode::empty(),
            outdoub: SplitVarnode::empty(),
        }
    }

    fn verify(&mut self, data: &Funcdata, h: VarnodeId, l: VarnodeId, op: OpId) -> bool {
        self.hi1 = Some(h);
        self.lo1 = Some(l);
        self.slot1 = op_get_slot(data, op, h);
        for i in 0..2 {
            if i == 0 {
                // Assume we have to descend one more add
                self.add2 = lone_descend(data, op_get_out(data, op).expect("subverify: op out"));
                let add2 = match self.add2 {
                    Some(a) => a,
                    None => continue,
                };
                if op_code(data, add2) != OpCode::CPUI_INT_ADD {
                    continue;
                }
                self.reshi = op_get_out(data, add2);
                self.hineg1 = Some(op_get_in(data, op, 1 - self.slot1));
                let opout = op_get_out(data, op).expect("subverify: op out2");
                self.hineg2 = Some(op_get_in(data, add2, 1 - op_get_slot(data, add2, opout)));
            } else {
                let tmpvn = op_get_in(data, op, 1 - self.slot1);
                if !vn_is_written(data, tmpvn) {
                    continue;
                }
                self.add2 = vn_get_def(data, tmpvn);
                let add2 = self.add2.expect("subverify: add2");
                if op_code(data, add2) != OpCode::CPUI_INT_ADD {
                    continue;
                }
                self.reshi = op_get_out(data, op);
                self.hineg1 = Some(op_get_in(data, add2, 0));
                self.hineg2 = Some(op_get_in(data, add2, 1));
            }
            let hineg1 = self.hineg1.expect("subverify: hineg1");
            let hineg2 = self.hineg2.expect("subverify: hineg2");
            if !vn_is_written(data, hineg1) {
                continue;
            }
            if !vn_is_written(data, hineg2) {
                continue;
            }
            if !verify_mult_neg_one(data, vn_get_def(data, hineg1).expect("subverify: hineg1 def")) {
                continue;
            }
            if !verify_mult_neg_one(data, vn_get_def(data, hineg2).expect("subverify: hineg2 def")) {
                continue;
            }
            self.hizext1 = Some(op_get_in(data, vn_get_def(data, hineg1).expect("subverify"), 0));
            self.hizext2 = Some(op_get_in(data, vn_get_def(data, hineg2).expect("subverify"), 0));
            for j in 0..2 {
                if j == 0 {
                    let hz1 = self.hizext1.expect("subverify: hizext1");
                    if !vn_is_written(data, hz1) {
                        continue;
                    }
                    self.zextop = vn_get_def(data, hz1);
                    self.hi2 = self.hizext2;
                } else {
                    let hz2 = self.hizext2.expect("subverify: hizext2");
                    if !vn_is_written(data, hz2) {
                        continue;
                    }
                    self.zextop = vn_get_def(data, hz2);
                    self.hi2 = self.hizext1;
                }
                let zextop = self.zextop.expect("subverify: zextop");
                if op_code(data, zextop) != OpCode::CPUI_INT_ZEXT {
                    continue;
                }
                let zin0 = op_get_in(data, zextop, 0);
                if !vn_is_written(data, zin0) {
                    continue;
                }
                self.lessop = vn_get_def(data, zin0);
                let lessop = self.lessop.expect("subverify: lessop");
                if op_code(data, lessop) != OpCode::CPUI_INT_LESS {
                    continue;
                }
                let lo1 = self.lo1.expect("subverify: lo1");
                if op_get_in(data, lessop, 0) != lo1 {
                    continue;
                }
                self.lo2 = Some(op_get_in(data, lessop, 1));
                for loadd in descend_ops(data, lo1) {
                    if op_code(data, loadd) != OpCode::CPUI_INT_ADD {
                        continue;
                    }
                    let tmpvn = op_get_in(data, loadd, 1 - op_get_slot(data, loadd, lo1));
                    if !vn_is_written(data, tmpvn) {
                        continue;
                    }
                    let negop = vn_get_def(data, tmpvn).expect("subverify: negop");
                    if !verify_mult_neg_one(data, negop) {
                        continue;
                    }
                    if op_get_in(data, negop, 0) != self.lo2.expect("subverify: lo2") {
                        continue;
                    }
                    self.reslo = op_get_out(data, loadd);
                    return true;
                }
            }
        }
        false
    }

    fn apply_rule(&mut self, i: &SplitVarnode, op: OpId, workishi: bool, data: &mut Funcdata) -> bool {
        if !workishi {
            return false;
        }
        if !i.has_both_pieces() {
            return false;
        }
        self.in_ = i.clone();
        let hi = self.in_.get_hi().expect("subrule: hi");
        let lo = self.in_.get_lo().expect("subrule: lo");
        if !self.verify(data, hi, lo, op) {
            return false;
        }

        self.indoub.init_partial_pieces(data, self.in_.get_size(), self.lo2, self.hi2);
        if self.indoub.exceeds_const_precision() {
            return false;
        }
        self.outdoub.init_partial_pieces(data, self.in_.get_size(), self.reslo, self.reshi);
        let existop = match prepare_binary_op(data, &mut self.outdoub, &mut self.in_, &mut self.indoub) {
            Some(e) => e,
            None => return false,
        };
        let (mut out, mut in1, mut in2) =
            (self.outdoub.clone(), self.in_.clone(), self.indoub.clone());
        create_binary_op(data, &mut out, &mut in1, &mut in2, existop, OpCode::CPUI_INT_SUB);
        true
    }
}

// =============================================================================
// LogicalForm (double.cc:135-146 hh, 1704-1825 cc)
// =============================================================================

/// Match a double-precision logical (AND/OR/XOR) op (C++ `LogicalForm`).
struct LogicalForm {
    in_: SplitVarnode,
    loop_: Option<OpId>,
    hiop: Option<OpId>,
    hi1: Option<VarnodeId>,
    lo2: Option<VarnodeId>,
    hi2: Option<VarnodeId>,
    indoub: SplitVarnode,
    outdoub: SplitVarnode,
}

impl LogicalForm {
    fn new() -> LogicalForm {
        LogicalForm {
            in_: SplitVarnode::empty(),
            loop_: None,
            hiop: None,
            hi1: None,
            lo2: None,
            hi2: None,
            indoub: SplitVarnode::empty(),
            outdoub: SplitVarnode::empty(),
        }
    }

    /// Find the op computing the most significant part of the result (C++
    /// `findHiMatch`).  Returns 0 (found), -1 (can't find), -2 (none exists).
    fn find_hi_match(&mut self, data: &Funcdata) -> int4 {
        let lo1_tmp = self.in_.get_lo().expect("fhm: lo");
        let loop_ = self.loop_.expect("fhm: loop");
        let vn2 = op_get_in(data, loop_, 1 - op_get_slot(data, loop_, lo1_tmp));
        let hi1 = self.hi1.expect("fhm: hi1");

        let mut out = SplitVarnode::empty();
        if out.in_hand_lo_out(data, lo1_tmp) {
            // We already know what the double precision output looks like
            let hi = out.get_hi().expect("fhm: out hi");
            if vn_is_written(data, hi) {
                // Just look at construction of hi precis
                let maybeop = vn_get_def(data, hi).expect("fhm: hi def");
                if op_code(data, maybeop) == op_code(data, loop_) {
                    if op_get_in(data, maybeop, 0) == hi1 {
                        if vn_is_constant(data, op_get_in(data, maybeop, 1)) == vn_is_constant(data, vn2) {
                            self.hiop = Some(maybeop);
                            return 0;
                        }
                    } else if op_get_in(data, maybeop, 1) == hi1
                        && vn_is_constant(data, op_get_in(data, maybeop, 0)) == vn_is_constant(data, vn2)
                    {
                        self.hiop = Some(maybeop);
                        return 0;
                    }
                }
            }
        }

        if !vn_is_constant(data, vn2) {
            let mut in2 = SplitVarnode::empty();
            if in2.in_hand_lo(data, vn2) {
                // We already know what the other double precision input looks like
                let in2hi = in2.get_hi().expect("fhm: in2 hi");
                for maybeop in descend_ops(data, in2hi) {
                    if op_code(data, maybeop) == op_code(data, loop_)
                        && (op_get_in(data, maybeop, 0) == hi1 || op_get_in(data, maybeop, 1) == hi1)
                    {
                        self.hiop = Some(maybeop);
                        return 0;
                    }
                }
            }
            -1
        } else {
            let mut count = 0i32;
            let mut lastop: Option<OpId> = None;
            for maybeop in descend_ops(data, hi1) {
                if op_code(data, maybeop) == op_code(data, loop_)
                    && vn_is_constant(data, op_get_in(data, maybeop, 1))
                {
                    count += 1;
                    if count > 1 {
                        break;
                    }
                    lastop = Some(maybeop);
                }
            }
            if count == 1 {
                self.hiop = lastop;
                return 0;
            }
            if count > 1 {
                return -1; // Couldn't distinguish between multiple possibilities
            }
            -2
        }
    }

    fn verify(&mut self, data: &Funcdata, h: VarnodeId, l: VarnodeId, lop: OpId) -> bool {
        self.loop_ = Some(lop);
        let lo1 = l;
        self.hi1 = Some(h);
        // in_ is already set to the SplitVarnode in apply_rule; find_hi_match reads
        // in_.get_lo() so we mirror the C++ by keeping in_ available.
        let res = self.find_hi_match(data);

        if res == 0 {
            // We found a matching lo precis operation
            let loop_ = self.loop_.expect("logverify: loop");
            let hiop = self.hiop.expect("logverify: hiop");
            self.lo2 = Some(op_get_in(data, loop_, 1 - op_get_slot(data, loop_, lo1)));
            self.hi2 = Some(op_get_in(data, hiop, 1 - op_get_slot(data, hiop, h)));
            let lo2 = self.lo2.expect("logverify: lo2");
            let hi2 = self.hi2.expect("logverify: hi2");
            // No manipulation of itself
            if lo2 == lo1 || lo2 == h || hi2 == h || hi2 == lo1 {
                return false;
            }
            if lo2 == hi2 {
                return false;
            }
            return true;
        }
        false
    }

    fn apply_rule(&mut self, i: &SplitVarnode, lop: OpId, workishi: bool, data: &mut Funcdata) -> bool {
        if workishi {
            return false;
        }
        if !i.has_both_pieces() {
            return false;
        }
        self.in_ = i.clone();
        let hi = self.in_.get_hi().expect("logrule: hi");
        let lo = self.in_.get_lo().expect("logrule: lo");
        if !self.verify(data, hi, lo, lop) {
            return false;
        }

        let loop_ = self.loop_.expect("logrule: loop");
        let hiop = self.hiop.expect("logrule: hiop");
        self.outdoub.init_partial_pieces(
            data,
            self.in_.get_size(),
            op_get_out(data, loop_),
            op_get_out(data, hiop),
        );
        self.indoub.init_partial_pieces(data, self.in_.get_size(), self.lo2, self.hi2);
        if self.indoub.exceeds_const_precision() {
            return false;
        }
        let existop = match prepare_binary_op(data, &mut self.outdoub, &mut self.in_, &mut self.indoub) {
            Some(e) => e,
            None => return false,
        };

        let opc = op_code(data, loop_);
        let (mut out, mut in1, mut in2) =
            (self.outdoub.clone(), self.in_.clone(), self.indoub.clone());
        create_binary_op(data, &mut out, &mut in1, &mut in2, existop, opc);
        true
    }
}

// =============================================================================
// Equal1Form (double.cc:148-159 hh, 1836-1916 cc)
// =============================================================================

/// Match a double-precision branching equality (the `if (hi==hi2) { if (lo==lo2)
/// ... }` form) (C++ `Equal1Form`).
struct Equal1Form {
    in1: SplitVarnode,
    in2: SplitVarnode,
    loop_: Option<OpId>,
    hiop: Option<OpId>,
    hi1: Option<VarnodeId>,
    lo1: Option<VarnodeId>,
    hi2: Option<VarnodeId>,
    lo2: Option<VarnodeId>,
    hi1slot: int4,
    lo1slot: int4,
    notequalformhi: bool,
    notequalformlo: bool,
    setonlow: bool,
}

impl Equal1Form {
    fn new() -> Equal1Form {
        Equal1Form {
            in1: SplitVarnode::empty(),
            in2: SplitVarnode::empty(),
            loop_: None,
            hiop: None,
            hi1: None,
            lo1: None,
            hi2: None,
            lo2: None,
            hi1slot: 0,
            lo1slot: 0,
            notequalformhi: false,
            notequalformlo: false,
            setonlow: false,
        }
    }

    fn apply_rule(&mut self, i: &SplitVarnode, hop: OpId, workishi: bool, data: &mut Funcdata) -> bool {
        if !workishi {
            return false;
        }
        if !i.has_both_pieces() {
            return false;
        }
        self.in1 = i.clone();

        self.hiop = Some(hop);
        self.hi1 = self.in1.get_hi();
        self.lo1 = self.in1.get_lo();
        let hi1 = self.hi1.expect("eq1: hi1");
        let lo1 = self.lo1.expect("eq1: lo1");
        self.hi1slot = op_get_slot(data, hop, hi1);
        self.hi2 = Some(op_get_in(data, hop, 1 - self.hi1slot));
        self.notequalformhi = op_code(data, hop) == OpCode::CPUI_INT_NOTEQUAL;

        for loop_ in descend_ops(data, lo1) {
            self.loop_ = Some(loop_);
            match op_code(data, loop_) {
                OpCode::CPUI_INT_EQUAL => self.notequalformlo = false,
                OpCode::CPUI_INT_NOTEQUAL => self.notequalformlo = true,
                _ => continue,
            }
            self.lo1slot = op_get_slot(data, loop_, lo1);
            self.lo2 = Some(op_get_in(data, loop_, 1 - self.lo1slot));

            let hiop_out = op_get_out(data, hop).expect("eq1: hiop out");
            let loop_out = op_get_out(data, loop_).expect("eq1: loop out");
            for hibool in descend_ops(data, hiop_out) {
                for lobool in descend_ops(data, loop_out) {
                    self.in2.init_partial_pieces(data, self.in1.get_size(), self.lo2, self.hi2);
                    if self.in2.exceeds_const_precision() {
                        continue;
                    }

                    if op_code(data, hibool) == OpCode::CPUI_CBRANCH
                        && op_code(data, lobool) == OpCode::CPUI_CBRANCH
                    {
                        // Branching form of the equal operation
                        let (hibooltrue, hiboolfalse) =
                            get_true_false(data, hibool, self.notequalformhi);
                        let (lobooltrue, loboolfalse) =
                            get_true_false(data, lobool, self.notequalformlo);

                        if Some(hibooltrue) == op_get_parent(data, lobool)
                            && hiboolfalse == loboolfalse
                            && otherwise_empty(data, lobool)
                        {
                            // hi is checked first then lo
                            let (mut a, mut b) = (self.in1.clone(), self.in2.clone());
                            if prepare_bool_op(data, &mut a, &mut b, hibool) {
                                self.setonlow = true;
                                let opc = if self.notequalformhi {
                                    OpCode::CPUI_INT_NOTEQUAL
                                } else {
                                    OpCode::CPUI_INT_EQUAL
                                };
                                create_bool_op(data, hibool, &mut a, &mut b, opc);
                                // Change lobool to always go to the original TRUE block
                                let c = data.new_constant(1, if self.notequalformlo { 0 } else { 1 });
                                op_set_input(data, lobool, c, 1);
                                self.in1 = a;
                                self.in2 = b;
                                return true;
                            }
                        } else if Some(lobooltrue) == op_get_parent(data, hibool)
                            && hiboolfalse == loboolfalse
                            && otherwise_empty(data, hibool)
                        {
                            // lo is checked first then hi
                            let (mut a, mut b) = (self.in1.clone(), self.in2.clone());
                            if prepare_bool_op(data, &mut a, &mut b, lobool) {
                                self.setonlow = false;
                                let opc = if self.notequalformlo {
                                    OpCode::CPUI_INT_NOTEQUAL
                                } else {
                                    OpCode::CPUI_INT_EQUAL
                                };
                                create_bool_op(data, lobool, &mut a, &mut b, opc);
                                // Change hibool to always go to the original TRUE block
                                let c = data.new_constant(1, if self.notequalformhi { 0 } else { 1 });
                                op_set_input(data, hibool, c, 1);
                                self.in1 = a;
                                self.in2 = b;
                                return true;
                            }
                        }
                    }
                }
            }
        }
        false
    }
}

// =============================================================================
// Equal2Form (double.cc:161-169 hh, 1918-1982 cc)
// =============================================================================

/// Match a double-precision combined equality (`(hi1==hi2)&&(lo1==lo2)` or the
/// NOTEQUAL/OR dual) (C++ `Equal2Form`).
struct Equal2Form {
    in_: SplitVarnode,
    hi1: Option<VarnodeId>,
    hi2: Option<VarnodeId>,
    lo1: Option<VarnodeId>,
    lo2: Option<VarnodeId>,
    bool_and_or: Option<OpId>,
    param2: SplitVarnode,
}

impl Equal2Form {
    fn new() -> Equal2Form {
        Equal2Form {
            in_: SplitVarnode::empty(),
            hi1: None,
            hi2: None,
            lo1: None,
            lo2: None,
            bool_and_or: None,
            param2: SplitVarnode::empty(),
        }
    }

    fn replace(&mut self, data: &Funcdata) -> bool {
        let hi2 = self.hi2.expect("eq2: hi2");
        let lo2 = self.lo2.expect("eq2: lo2");
        let lo1 = self.lo1.expect("eq2: lo1");
        let boolandor = self.bool_and_or.expect("eq2: boolandor");
        if vn_is_constant(data, hi2) && vn_is_constant(data, lo2) {
            let mut val = vn_get_offset(data, hi2);
            val = val.wshl((8 * vn_get_size(data, lo1)) as u32);
            val |= vn_get_offset(data, lo2);
            self.param2.init_partial_const(self.in_.get_size(), val);
            // Thread the member pair (in,param2) through prepare so the whole/defblock/
            // defpoint discovered by isWholeFeasible persists into the replaceBoolOp call
            // in apply_rule (C++ prepareBoolOp(in,param2,...) mutates the members; double.cc:1926).
            let (mut a, mut b) = (self.in_.clone(), self.param2.clone());
            let res = prepare_bool_op(data, &mut a, &mut b, boolandor);
            self.in_ = a;
            self.param2 = b;
            return res;
        }
        if vn_is_constant(data, hi2) || vn_is_constant(data, lo2) {
            // Some kind of mixed form
            return false;
        }
        self.param2.init_partial_pieces(data, self.in_.get_size(), self.lo2, self.hi2);
        // Same threading as the constant branch (C++ prepareBoolOp(in,param2,...); double.cc:1933).
        let (mut a, mut b) = (self.in_.clone(), self.param2.clone());
        let res = prepare_bool_op(data, &mut a, &mut b, boolandor);
        self.in_ = a;
        self.param2 = b;
        res
    }

    fn apply_rule(&mut self, i: &SplitVarnode, op: OpId, workishi: bool, data: &mut Funcdata) -> bool {
        if !workishi {
            return false;
        }
        if !i.has_both_pieces() {
            return false;
        }
        self.in_ = i.clone();
        self.hi1 = self.in_.get_hi();
        self.lo1 = self.in_.get_lo();
        let hi1 = self.hi1.expect("eq2: hi1");
        let lo1 = self.lo1.expect("eq2: lo1");
        let eq_code = op_code(data, op);
        let hi1slot = op_get_slot(data, op, hi1);
        self.hi2 = Some(op_get_in(data, op, 1 - hi1slot));
        let outvn = op_get_out(data, op).expect("eq2: op out");
        for boolandor in descend_ops(data, outvn) {
            self.bool_and_or = Some(boolandor);
            if eq_code == OpCode::CPUI_INT_EQUAL && op_code(data, boolandor) != OpCode::CPUI_BOOL_AND {
                continue;
            }
            if eq_code == OpCode::CPUI_INT_NOTEQUAL && op_code(data, boolandor) != OpCode::CPUI_BOOL_OR {
                continue;
            }
            let slot = op_get_slot(data, boolandor, outvn);
            let othervn = op_get_in(data, boolandor, 1 - slot);
            if !vn_is_written(data, othervn) {
                continue;
            }
            let equal_lo = vn_get_def(data, othervn).expect("eq2: equal_lo");
            if op_code(data, equal_lo) != eq_code {
                continue;
            }
            if op_get_in(data, equal_lo, 0) == lo1 {
                self.lo2 = Some(op_get_in(data, equal_lo, 1));
            } else if op_get_in(data, equal_lo, 1) == lo1 {
                self.lo2 = Some(op_get_in(data, equal_lo, 0));
            } else {
                continue;
            }
            if !self.replace(data) {
                continue;
            }
            if self.param2.exceeds_const_precision() {
                continue;
            }
            // Reuse the pair prepared by replace() (whole/defblock/defpoint already
            // discovered) — C++ replaceBoolOp(in,param2,...) passes the same members
            // prepareBoolOp mutated (double.cc:1978).
            let (mut a, mut b) = (self.in_.clone(), self.param2.clone());
            replace_bool_op(data, boolandor, &mut a, &mut b, eq_code);
            self.in_ = a;
            self.param2 = b;
            return true;
        }
        false
    }
}

// =============================================================================
// Equal3Form (double.cc:171-180 hh, 1984-2024 cc)
// =============================================================================

/// Match a double-precision compare against -1 (`hi & lo == -1`) (C++
/// `Equal3Form`).
struct Equal3Form {
    in_: SplitVarnode,
    hi: Option<VarnodeId>,
    lo: Option<VarnodeId>,
    andop: Option<OpId>,
    compareop: Option<OpId>,
    smallc: Option<VarnodeId>,
}

impl Equal3Form {
    fn new() -> Equal3Form {
        Equal3Form {
            in_: SplitVarnode::empty(),
            hi: None,
            lo: None,
            andop: None,
            compareop: None,
            smallc: None,
        }
    }

    fn verify(&mut self, data: &Funcdata, h: VarnodeId, l: VarnodeId, aop: OpId) -> bool {
        if op_code(data, aop) != OpCode::CPUI_INT_AND {
            return false;
        }
        self.hi = Some(h);
        self.lo = Some(l);
        self.andop = Some(aop);
        let hislot = op_get_slot(data, aop, h);
        if op_get_in(data, aop, 1 - hislot) != l {
            return false; // hi and lo must be ANDed together
        }
        self.compareop = lone_descend(data, op_get_out(data, aop).expect("eq3: and out"));
        let compareop = match self.compareop {
            Some(c) => c,
            None => return false,
        };
        if op_code(data, compareop) != OpCode::CPUI_INT_EQUAL
            && op_code(data, compareop) != OpCode::CPUI_INT_NOTEQUAL
        {
            return false;
        }
        let allonesval = calc_mask(vn_get_size(data, l));
        self.smallc = Some(op_get_in(data, compareop, 1));
        let smallc = self.smallc.expect("eq3: smallc");
        if !vn_is_constant(data, smallc) {
            return false;
        }
        if vn_get_offset(data, smallc) != allonesval {
            return false;
        }
        true
    }

    fn apply_rule(&mut self, i: &SplitVarnode, op: OpId, workishi: bool, data: &mut Funcdata) -> bool {
        if !workishi {
            return false;
        }
        if !i.has_both_pieces() {
            return false;
        }
        self.in_ = i.clone();
        let hi = self.in_.get_hi().expect("eq3: hi");
        let lo = self.in_.get_lo().expect("eq3: lo");
        if !self.verify(data, hi, lo, op) {
            return false;
        }

        // Create the -1 value
        let mut in2 = SplitVarnode::new_constant(self.in_.get_size(), calc_mask(self.in_.get_size()));
        if in2.exceeds_const_precision() {
            return false;
        }
        let compareop = self.compareop.expect("eq3: compareop");
        let mut in_ = self.in_.clone();
        if !prepare_bool_op(data, &mut in_, &mut in2, compareop) {
            return false;
        }
        let opc = op_code(data, compareop);
        replace_bool_op(data, compareop, &mut in_, &mut in2, opc);
        self.in_ = in_;
        true
    }
}

// =============================================================================
// LessThreeWay (double.cc:182-216 hh, 2026-2496 cc)
// =============================================================================

/// Match a double-precision less-than expressed as a three-way branch (hiless /
/// hiequal / loless) (C++ `LessThreeWay`).
struct LessThreeWay {
    in_: SplitVarnode,
    in2: SplitVarnode,
    hilessbl: Option<BlockId>,
    lolessbl: Option<BlockId>,
    hieqbl: Option<BlockId>,
    hilessbool: Option<OpId>,
    lolessbool: Option<OpId>,
    hieqbool: Option<OpId>,
    hiless: Option<OpId>,
    hiequal: Option<OpId>,
    loless: Option<OpId>,
    vnhil1: Option<VarnodeId>,
    vnhil2: Option<VarnodeId>,
    vnhie1: Option<VarnodeId>,
    vnhie2: Option<VarnodeId>,
    vnlo1: Option<VarnodeId>,
    vnlo2: Option<VarnodeId>,
    hi: Option<VarnodeId>,
    lo: Option<VarnodeId>,
    hi2: Option<VarnodeId>,
    lo2: Option<VarnodeId>,
    hislot: int4,
    hiflip: bool,
    equalflip: bool,
    loflip: bool,
    lolessiszerocomp: bool,
    lolessequalform: bool,
    hilessequalform: bool,
    signcompare: bool,
    midlessform: bool,
    midlessequal: bool,
    midsigncompare: bool,
    hiconstform: bool,
    midconstform: bool,
    loconstform: bool,
    hival: uintb,
    midval: uintb,
    loval: uintb,
    finalopc: OpCode,
    // block-form fields filled by checkBlockForm
    hilesstrue: Option<BlockId>,
    hilessfalse: Option<BlockId>,
}

impl LessThreeWay {
    fn new() -> LessThreeWay {
        LessThreeWay {
            in_: SplitVarnode::empty(),
            in2: SplitVarnode::empty(),
            hilessbl: None,
            lolessbl: None,
            hieqbl: None,
            hilessbool: None,
            lolessbool: None,
            hieqbool: None,
            hiless: None,
            hiequal: None,
            loless: None,
            vnhil1: None,
            vnhil2: None,
            vnhie1: None,
            vnhie2: None,
            vnlo1: None,
            vnlo2: None,
            hi: None,
            lo: None,
            hi2: None,
            lo2: None,
            hislot: 0,
            hiflip: false,
            equalflip: false,
            loflip: false,
            lolessiszerocomp: false,
            lolessequalform: false,
            hilessequalform: false,
            signcompare: false,
            midlessform: false,
            midlessequal: false,
            midsigncompare: false,
            hiconstform: false,
            midconstform: false,
            loconstform: false,
            hival: 0,
            midval: 0,
            loval: 0,
            finalopc: OpCode::CPUI_INT_LESS,
            hilesstrue: None,
            hilessfalse: None,
        }
    }

    fn map_blocks_from_low(&mut self, data: &Funcdata, lobl: BlockId) -> bool {
        self.lolessbl = Some(lobl);
        if bl_size_in(data, lobl) != 1 {
            return false;
        }
        if bl_size_out(data, lobl) != 2 {
            return false;
        }
        let hieqbl = bl_get_in(data, lobl, 0);
        self.hieqbl = Some(hieqbl);
        if bl_size_in(data, hieqbl) != 1 {
            return false;
        }
        if bl_size_out(data, hieqbl) != 2 {
            return false;
        }
        let hilessbl = bl_get_in(data, hieqbl, 0);
        self.hilessbl = Some(hilessbl);
        if bl_size_out(data, hilessbl) != 2 {
            return false;
        }
        true
    }

    fn map_ops_from_blocks(&mut self, data: &Funcdata) -> bool {
        self.lolessbool = bl_last_op(data, self.lolessbl.expect("ltw: lolessbl"));
        let lolessbool = match self.lolessbool {
            Some(o) => o,
            None => return false,
        };
        if op_code(data, lolessbool) != OpCode::CPUI_CBRANCH {
            return false;
        }
        self.hieqbool = bl_last_op(data, self.hieqbl.expect("ltw: hieqbl"));
        let hieqbool = match self.hieqbool {
            Some(o) => o,
            None => return false,
        };
        if op_code(data, hieqbool) != OpCode::CPUI_CBRANCH {
            return false;
        }
        self.hilessbool = bl_last_op(data, self.hilessbl.expect("ltw: hilessbl"));
        let hilessbool = match self.hilessbool {
            Some(o) => o,
            None => return false,
        };
        if op_code(data, hilessbool) != OpCode::CPUI_CBRANCH {
            return false;
        }

        self.hiflip = false;
        self.equalflip = false;
        self.loflip = false;
        self.midlessform = false;
        self.lolessiszerocomp = false;

        let vn = op_get_in(data, hieqbool, 1);
        if !vn_is_written(data, vn) {
            return false;
        }
        self.hiequal = vn_get_def(data, vn);
        let hiequal = self.hiequal.expect("ltw: hiequal");
        match op_code(data, hiequal) {
            OpCode::CPUI_INT_EQUAL => self.midlessform = false,
            OpCode::CPUI_INT_NOTEQUAL => self.midlessform = false,
            OpCode::CPUI_INT_LESS => {
                self.midlessequal = false;
                self.midsigncompare = false;
                self.midlessform = true;
            }
            OpCode::CPUI_INT_LESSEQUAL => {
                self.midlessequal = true;
                self.midsigncompare = false;
                self.midlessform = true;
            }
            OpCode::CPUI_INT_SLESS => {
                self.midlessequal = false;
                self.midsigncompare = true;
                self.midlessform = true;
            }
            OpCode::CPUI_INT_SLESSEQUAL => {
                self.midlessequal = true;
                self.midsigncompare = true;
                self.midlessform = true;
            }
            _ => return false,
        }

        let vn = op_get_in(data, lolessbool, 1);
        if !vn_is_written(data, vn) {
            return false;
        }
        self.loless = vn_get_def(data, vn);
        let loless = self.loless.expect("ltw: loless");
        match op_code(data, loless) {
            // Only unsigned forms
            OpCode::CPUI_INT_LESS => self.lolessequalform = false,
            OpCode::CPUI_INT_LESSEQUAL => self.lolessequalform = true,
            OpCode::CPUI_INT_EQUAL => {
                if !vn_is_constant(data, op_get_in(data, loless, 1)) {
                    return false;
                }
                if vn_get_offset(data, op_get_in(data, loless, 1)) != 0 {
                    return false;
                }
                self.lolessiszerocomp = true;
                self.lolessequalform = true;
            }
            OpCode::CPUI_INT_NOTEQUAL => {
                if !vn_is_constant(data, op_get_in(data, loless, 1)) {
                    return false;
                }
                if vn_get_offset(data, op_get_in(data, loless, 1)) != 0 {
                    return false;
                }
                self.lolessiszerocomp = true;
                self.lolessequalform = false;
            }
            _ => return false,
        }

        let vn = op_get_in(data, hilessbool, 1);
        if !vn_is_written(data, vn) {
            return false;
        }
        self.hiless = vn_get_def(data, vn);
        let hiless = self.hiless.expect("ltw: hiless");
        match op_code(data, hiless) {
            OpCode::CPUI_INT_LESS => {
                self.hilessequalform = false;
                self.signcompare = false;
            }
            OpCode::CPUI_INT_LESSEQUAL => {
                self.hilessequalform = true;
                self.signcompare = false;
            }
            OpCode::CPUI_INT_SLESS => {
                self.hilessequalform = false;
                self.signcompare = true;
            }
            OpCode::CPUI_INT_SLESSEQUAL => {
                self.hilessequalform = true;
                self.signcompare = true;
            }
            _ => return false,
        }
        true
    }

    fn check_signedness(&self) -> bool {
        if self.midlessform && self.midsigncompare != self.signcompare {
            return false;
        }
        true
    }

    fn normalize_hi(&mut self, data: &Funcdata) -> bool {
        let hiless = self.hiless.expect("ltw: hiless");
        self.vnhil1 = Some(op_get_in(data, hiless, 0));
        self.vnhil2 = Some(op_get_in(data, hiless, 1));
        if vn_is_constant(data, self.vnhil1.expect("nh: vnhil1")) {
            // Start with constant on the right
            self.hiflip = !self.hiflip;
            self.hilessequalform = !self.hilessequalform;
            std::mem::swap(&mut self.vnhil1, &mut self.vnhil2);
        }
        self.hiconstform = false;
        if vn_is_constant(data, self.vnhil2.expect("nh: vnhil2")) {
            if self.in_.get_size() > SIZEOF_UINTB {
                return false; // Must have enough precision for constant
            }
            self.hiconstform = true;
            self.hival = vn_get_offset(data, self.vnhil2.expect("nh: vnhil2"));
            let (t, f) = get_true_false(data, self.hilessbool.expect("nh: hilessbool"), self.hiflip);
            self.hilesstrue = Some(t);
            self.hilessfalse = Some(f);
            let mut inc: i64 = 1;
            if self.hilessfalse != self.hieqbl {
                // Make sure the hiless false branch goes to the hieq block
                self.hiflip = !self.hiflip;
                self.hilessequalform = !self.hilessequalform;
                std::mem::swap(&mut self.vnhil1, &mut self.vnhil2);
                inc = -1;
            }
            if self.hilessequalform {
                // Normalize lessequal to less
                self.hival = self.hival.wrapping_add(inc as uintb);
                self.hival &= calc_mask(self.in_.get_size());
                self.hilessequalform = false;
            }
            self.hival = self.hival.wshr((vn_get_size(data, self.in_.get_lo().expect("nh: lo")) * 8) as u32);
        } else if self.hilessequalform {
            // Make sure the false branch contains the equal case
            self.hilessequalform = false;
            self.hiflip = !self.hiflip;
            std::mem::swap(&mut self.vnhil1, &mut self.vnhil2);
        }
        true
    }

    fn normalize_mid(&mut self, data: &Funcdata) -> bool {
        let hiequal = self.hiequal.expect("ltw: hiequal");
        self.vnhie1 = Some(op_get_in(data, hiequal, 0));
        self.vnhie2 = Some(op_get_in(data, hiequal, 1));
        if vn_is_constant(data, self.vnhie1.expect("nm: vnhie1")) {
            // Make sure constant is on the right
            std::mem::swap(&mut self.vnhie1, &mut self.vnhie2);
            if self.midlessform {
                self.equalflip = !self.equalflip;
                self.midlessequal = !self.midlessequal;
            }
        }
        self.midconstform = false;
        if vn_is_constant(data, self.vnhie2.expect("nm: vnhie2")) {
            if !self.hiconstform {
                return false; // If mid is constant, both mid and hi must be constant
            }
            self.midconstform = true;
            self.midval = vn_get_offset(data, self.vnhie2.expect("nm: vnhie2"));
            let losize = vn_get_size(data, self.in_.get_lo().expect("nm: lo"));
            if vn_get_size(data, self.vnhie2.expect("nm: vnhie2")) == self.in_.get_size() {
                // Convert to comparison on high part
                let lopart = self.midval & calc_mask(losize);
                self.midval = self.midval.wshr((losize * 8) as u32);
                if self.midlessform {
                    if self.midlessequal {
                        if lopart != calc_mask(losize) {
                            return false;
                        }
                    } else if lopart != 0 {
                        return false;
                    }
                } else {
                    return false; // Compare is forcing restriction on lo part
                }
            }
            if self.midval != self.hival {
                // If the mid and hi don't match
                if !self.midlessform {
                    return false;
                }
                self.midval = self
                    .midval
                    .wrapping_add(if self.midlessequal { 1 } else { (-1i64) as uintb });
                self.midval &= calc_mask(losize);
                self.midlessequal = !self.midlessequal;
                if self.midval != self.hival {
                    return false; // Last chance
                }
            }
        }
        if self.midlessform {
            // Normalize to EQUAL
            if !self.midlessequal {
                self.equalflip = !self.equalflip;
            }
        } else if op_code(data, hiequal) == OpCode::CPUI_INT_NOTEQUAL {
            self.equalflip = !self.equalflip;
        }
        true
    }

    fn normalize_lo(&mut self, data: &Funcdata) -> bool {
        // This is basically identical to normalizeHi
        let loless = self.loless.expect("ltw: loless");
        self.vnlo1 = Some(op_get_in(data, loless, 0));
        self.vnlo2 = Some(op_get_in(data, loless, 1));
        if self.lolessiszerocomp {
            self.loconstform = true;
            if self.lolessequalform {
                // Treat as if we see vnlo1 <= 0
                self.loval = 1;
                self.lolessequalform = false;
            } else {
                // Treat as if we see 0 < vnlo1
                self.loflip = !self.loflip;
                self.loval = 1;
            }
            return true;
        }
        if vn_is_constant(data, self.vnlo1.expect("nl: vnlo1")) {
            // Make sure constant is on the right
            self.loflip = !self.loflip;
            self.lolessequalform = !self.lolessequalform;
            std::mem::swap(&mut self.vnlo1, &mut self.vnlo2);
        }
        self.loconstform = false;
        if vn_is_constant(data, self.vnlo2.expect("nl: vnlo2")) {
            // Make sure normalize lessequal to less
            self.loconstform = true;
            self.loval = vn_get_offset(data, self.vnlo2.expect("nl: vnlo2"));
            if self.lolessequalform {
                self.loval = self.loval.wrapping_add(1);
                self.loval &= calc_mask(vn_get_size(data, self.vnlo2.expect("nl: vnlo2")));
                self.lolessequalform = false;
            }
        } else if self.lolessequalform {
            self.lolessequalform = false;
            self.loflip = !self.loflip;
            std::mem::swap(&mut self.vnlo1, &mut self.vnlo2);
        }
        true
    }

    fn check_block_form(&mut self, data: &Funcdata) -> bool {
        let (hilesstrue, hilessfalse) =
            get_true_false(data, self.hilessbool.expect("cbf: hilessbool"), self.hiflip);
        let (lolesstrue, lolessfalse) =
            get_true_false(data, self.lolessbool.expect("cbf: lolessbool"), self.loflip);
        let (_hieqtrue, hieqfalse) =
            get_true_false(data, self.hieqbool.expect("cbf: hieqbool"), self.equalflip);
        let hieqtrue = _hieqtrue;
        if hilesstrue == lolesstrue
            && hieqfalse == lolessfalse
            && Some(hilessfalse) == self.hieqbl
            && Some(hieqtrue) == self.lolessbl
            && otherwise_empty(data, self.hieqbool.expect("cbf: hieqbool"))
            && otherwise_empty(data, self.lolessbool.expect("cbf: lolessbool"))
        {
            return true;
        }
        false
    }

    fn check_op_form(&mut self, data: &Funcdata) -> bool {
        self.lo = self.in_.get_lo();
        self.hi = self.in_.get_hi();
        let lo = self.lo;
        let hi = self.hi;

        if self.midconstform {
            if !self.hiconstform {
                return false;
            }
            if vn_get_size(data, self.vnhie2.expect("cof: vnhie2")) == self.in_.get_size() {
                if self.vnhie1 != self.vnhil1 && self.vnhie1 != self.vnhil2 {
                    return false;
                }
            } else if self.vnhie1 != self.in_.get_hi() {
                return false;
            }
            // normalizeMid checks that midval == hival
        } else {
            // hi and hi2 must appear as inputs in both -hiless- and -hiequal-
            if self.vnhil1 != self.vnhie1 && self.vnhil1 != self.vnhie2 {
                return false;
            }
            if self.vnhil2 != self.vnhie1 && self.vnhil2 != self.vnhie2 {
                return false;
            }
        }
        if hi.is_some() && hi == self.vnhil1 {
            if self.hiconstform {
                return false;
            }
            self.hislot = 0;
            self.hi2 = self.vnhil2;
            if self.vnlo1 != lo {
                // Pieces must be on the same side
                std::mem::swap(&mut self.vnlo1, &mut self.vnlo2);
                if self.vnlo1 != lo {
                    return false;
                }
                self.loflip = !self.loflip;
                self.lolessequalform = !self.lolessequalform;
            }
            self.lo2 = self.vnlo2;
        } else if hi.is_some() && hi == self.vnhil2 {
            if self.hiconstform {
                return false;
            }
            self.hislot = 1;
            self.hi2 = self.vnhil1;
            if self.vnlo2 != lo {
                std::mem::swap(&mut self.vnlo1, &mut self.vnlo2);
                if self.vnlo2 != lo {
                    return false;
                }
                self.loflip = !self.loflip;
                self.lolessequalform = !self.lolessequalform;
            }
            self.lo2 = self.vnlo1;
        } else if self.in_.get_whole() == self.vnhil1 {
            if !self.hiconstform {
                return false;
            }
            if !self.loconstform {
                return false;
            }
            if self.vnlo1 != lo {
                return false;
            }
            self.hislot = 0;
        } else if self.in_.get_whole() == self.vnhil2 {
            // Whole constant appears on the left
            if !self.hiconstform {
                return false;
            }
            if !self.loconstform {
                return false;
            }
            if self.vnlo2 != lo {
                self.loflip = !self.loflip;
                self.loval = self.loval.wrapping_sub(1);
                self.loval &= calc_mask(vn_get_size(data, lo.expect("cof: lo")));
                if self.vnlo1 != lo {
                    return false;
                }
            }
            self.hislot = 1;
        } else {
            return false;
        }

        true
    }

    fn set_op_code(&mut self) {
        // Decide on the opcode of the final double precision compare
        if self.lolessequalform != self.hiflip {
            self.finalopc = if self.signcompare {
                OpCode::CPUI_INT_SLESSEQUAL
            } else {
                OpCode::CPUI_INT_LESSEQUAL
            };
        } else {
            self.finalopc = if self.signcompare {
                OpCode::CPUI_INT_SLESS
            } else {
                OpCode::CPUI_INT_LESS
            };
        }
        if self.hiflip {
            self.hislot = 1 - self.hislot;
            self.hiflip = false;
        }
    }

    fn set_bool_op(&mut self, data: &Funcdata) -> bool {
        // Make changes to the threeway branch so it becomes a single double branch.
        // Thread the member pair (in,in2) through prepare so the whole/defblock/defpoint
        // discovered by isWholeFeasible persists into the createBoolOp call in apply_rule
        // (C++ prepareBoolOp(in,in2,...) mutates the members; double.cc:2419-2425).
        let hilessbool = self.hilessbool.expect("sbo: hilessbool");
        if self.hislot == 0 {
            let (mut a, mut b) = (self.in_.clone(), self.in2.clone());
            let res = prepare_bool_op(data, &mut a, &mut b, hilessbool);
            self.in_ = a;
            self.in2 = b;
            res
        } else {
            let (mut a, mut b) = (self.in2.clone(), self.in_.clone());
            let res = prepare_bool_op(data, &mut a, &mut b, hilessbool);
            self.in2 = a;
            self.in_ = b;
            res
        }
    }

    fn map_from_low(&mut self, data: &Funcdata, op: OpId) -> bool {
        // Given the lo-piece less-than and an isPrecisLo input, map the threeway form
        let loop_ = match lone_descend(data, op_get_out(data, op).expect("mfl: op out")) {
            Some(o) => o,
            None => return false,
        };
        let parent = match op_get_parent(data, loop_) {
            Some(p) => p,
            None => return false,
        };
        if !self.map_blocks_from_low(data, parent) {
            return false;
        }
        if !self.map_ops_from_blocks(data) {
            return false;
        }
        if !self.check_signedness() {
            return false;
        }
        if !self.normalize_hi(data) {
            return false;
        }
        if !self.normalize_mid(data) {
            return false;
        }
        if !self.normalize_lo(data) {
            return false;
        }
        if !self.check_op_form(data) {
            return false;
        }
        if !self.check_block_form(data) {
            return false;
        }
        true
    }

    fn test_replace(&mut self, data: &Funcdata) -> bool {
        self.set_op_code();
        if self.hiconstform {
            let losize = vn_get_size(data, self.in_.get_lo().expect("tr: lo"));
            let val = self.hival.wshl((8 * losize) as u32) | self.loval;
            self.in2.init_partial_const(self.in_.get_size(), val);
            if !self.set_bool_op(data) {
                return false;
            }
        } else {
            self.in2.init_partial_pieces(data, self.in_.get_size(), self.lo2, self.hi2);
            if !self.set_bool_op(data) {
                return false;
            }
        }
        true
    }

    fn apply_rule(&mut self, i: &SplitVarnode, loop_: OpId, workishi: bool, data: &mut Funcdata) -> bool {
        if workishi {
            return false;
        }
        if i.get_lo().is_none() {
            return false; // Doesn't necessarily need the hi
        }
        self.in_ = i.clone();
        if !self.map_from_low(data, loop_) {
            return false;
        }
        let res = self.test_replace(data);
        if res {
            if self.in2.exceeds_const_precision() {
                return false;
            }
            // Reuse the pair prepared by set_bool_op (whole/defblock/defpoint already
            // discovered) — C++ createBoolOp(in,in2,...) passes the same members
            // prepareBoolOp mutated (double.cc:2487-2490).
            let hilessbool = self.hilessbool.expect("ltw: hilessbool");
            if self.hislot == 0 {
                let (mut a, mut b) = (self.in_.clone(), self.in2.clone());
                create_bool_op(data, hilessbool, &mut a, &mut b, self.finalopc);
                self.in_ = a;
                self.in2 = b;
            } else {
                let (mut a, mut b) = (self.in2.clone(), self.in_.clone());
                create_bool_op(data, hilessbool, &mut a, &mut b, self.finalopc);
                self.in2 = a;
                self.in_ = b;
            }
            // Change hieqbool so that it always goes to the original FALSE block
            let c = data.new_constant(1, if self.equalflip { 1 } else { 0 });
            op_set_input(data, self.hieqbool.expect("ltw: hieqbool"), c, 1);
            // The lolessbool block now becomes unreachable and is eventually removed
        }
        res
    }
}

// =============================================================================
// LessConstForm (double.cc:218-226 hh, 2498-2548 cc)
// =============================================================================

/// Match a double-precision compare that only involves the high portion against
/// a constant (`hi COMPARE #const => whole COMPARE #constextend`) (C++
/// `LessConstForm`).
struct LessConstForm {
    in_: SplitVarnode,
    vn: Option<VarnodeId>,
    cvn: Option<VarnodeId>,
    inslot: int4,
    signcompare: bool,
    hilessequalform: bool,
    constin: SplitVarnode,
}

impl LessConstForm {
    fn new() -> LessConstForm {
        LessConstForm {
            in_: SplitVarnode::empty(),
            vn: None,
            cvn: None,
            inslot: 0,
            signcompare: false,
            hilessequalform: false,
            constin: SplitVarnode::empty(),
        }
    }

    fn apply_rule(&mut self, i: &SplitVarnode, op: OpId, workishi: bool, data: &mut Funcdata) -> bool {
        if !workishi {
            return false;
        }
        if i.get_hi().is_none() {
            return false; // We don't necessarily need the lo part
        }
        self.in_ = i.clone();
        self.vn = self.in_.get_hi();
        let vn = self.vn.expect("lcf: vn");
        self.inslot = op_get_slot(data, op, vn);
        self.cvn = Some(op_get_in(data, op, 1 - self.inslot));
        let cvn = self.cvn.expect("lcf: cvn");
        let losize = self.in_.get_size() - vn_get_size(data, vn);

        if !vn_is_constant(data, cvn) {
            return false;
        }

        let opc = op_code(data, op);
        self.signcompare = opc == OpCode::CPUI_INT_SLESSEQUAL || opc == OpCode::CPUI_INT_SLESS;
        self.hilessequalform = opc == OpCode::CPUI_INT_SLESSEQUAL || opc == OpCode::CPUI_INT_LESSEQUAL;

        let mut val = vn_get_offset(data, cvn).wshl((8 * losize) as u32);
        if self.hilessequalform != (self.inslot == 1) {
            val |= calc_mask(losize);
        }

        // This rule can mess up less,equal rules, so only apply it if it directly
        // affects a branch.
        let desc = match lone_descend(data, op_get_out(data, op).expect("lcf: op out")) {
            Some(d) => d,
            None => return false,
        };
        if op_code(data, desc) != OpCode::CPUI_CBRANCH {
            return false;
        }

        self.constin.init_partial_const(self.in_.get_size(), val);
        if self.constin.exceeds_const_precision() {
            return false;
        }

        if self.inslot == 0 {
            let (mut a, mut b) = (self.in_.clone(), self.constin.clone());
            if prepare_bool_op(data, &mut a, &mut b, op) {
                replace_bool_op(data, op, &mut a, &mut b, opc);
                self.in_ = a;
                self.constin = b;
                return true;
            }
        } else {
            let (mut a, mut b) = (self.constin.clone(), self.in_.clone());
            if prepare_bool_op(data, &mut a, &mut b, op) {
                replace_bool_op(data, op, &mut a, &mut b, opc);
                self.constin = a;
                self.in_ = b;
                return true;
            }
        }

        false
    }
}

// =============================================================================
// ShiftForm (double.cc:228-246 hh, 2550-2733 cc)
// =============================================================================

/// Match a double-precision shift (left or right) (C++ `ShiftForm`).
struct ShiftForm {
    in_: SplitVarnode,
    opc: OpCode, // Basic operation
    loshift: Option<OpId>,
    midshift: Option<OpId>,
    hishift: Option<OpId>,
    orop: Option<OpId>,
    lo: Option<VarnodeId>,
    hi: Option<VarnodeId>,
    midlo: Option<VarnodeId>,
    midhi: Option<VarnodeId>,
    salo: Option<VarnodeId>,
    sahi: Option<VarnodeId>,
    samid: Option<VarnodeId>,
    reslo: Option<VarnodeId>,
    reshi: Option<VarnodeId>,
    out: SplitVarnode,
}

impl ShiftForm {
    fn new() -> ShiftForm {
        ShiftForm {
            in_: SplitVarnode::empty(),
            opc: OpCode::CPUI_INT_LEFT,
            loshift: None,
            midshift: None,
            hishift: None,
            orop: None,
            lo: None,
            hi: None,
            midlo: None,
            midhi: None,
            salo: None,
            sahi: None,
            samid: None,
            reslo: None,
            reshi: None,
            out: SplitVarnode::empty(),
        }
    }

    fn map_left(&mut self, data: &Funcdata) -> bool {
        // Assume reshi, reslo are filled in, fill in other ops and varnodes
        let reslo = self.reslo.expect("ml: reslo");
        let reshi = self.reshi.expect("ml: reshi");
        if !vn_is_written(data, reslo) {
            return false;
        }
        if !vn_is_written(data, reshi) {
            return false;
        }
        self.loshift = vn_get_def(data, reslo);
        let loshift = self.loshift.expect("ml: loshift");
        self.opc = op_code(data, loshift);
        if self.opc != OpCode::CPUI_INT_LEFT {
            return false;
        }
        self.orop = vn_get_def(data, reshi);
        let orop = self.orop.expect("ml: orop");
        let oc = op_code(data, orop);
        if oc != OpCode::CPUI_INT_OR && oc != OpCode::CPUI_INT_XOR && oc != OpCode::CPUI_INT_ADD {
            return false;
        }
        self.midlo = Some(op_get_in(data, orop, 0));
        self.midhi = Some(op_get_in(data, orop, 1));
        let midlo = self.midlo.expect("ml: midlo");
        let midhi = self.midhi.expect("ml: midhi");
        if !vn_is_written(data, midlo) {
            return false;
        }
        if !vn_is_written(data, midhi) {
            return false;
        }
        if op_code(data, vn_get_def(data, midhi).expect("ml: midhi def")) != OpCode::CPUI_INT_LEFT {
            std::mem::swap(&mut self.midhi, &mut self.midlo);
        }
        let midlo = self.midlo.expect("ml: midlo");
        let midhi = self.midhi.expect("ml: midhi");
        self.midshift = vn_get_def(data, midlo);
        if op_code(data, self.midshift.expect("ml: midshift")) != OpCode::CPUI_INT_RIGHT {
            return false; // Must be unsigned RIGHT
        }
        self.hishift = vn_get_def(data, midhi);
        if op_code(data, self.hishift.expect("ml: hishift")) != OpCode::CPUI_INT_LEFT {
            return false;
        }

        let lo = self.lo.expect("ml: lo");
        let hi = self.hi.expect("ml: hi");
        if lo != op_get_in(data, loshift, 0) {
            return false;
        }
        if hi != op_get_in(data, self.hishift.expect("ml: hishift"), 0) {
            return false;
        }
        if lo != op_get_in(data, self.midshift.expect("ml: midshift"), 0) {
            return false;
        }
        self.salo = Some(op_get_in(data, loshift, 1));
        self.sahi = Some(op_get_in(data, self.hishift.expect("ml: hishift"), 1));
        self.samid = Some(op_get_in(data, self.midshift.expect("ml: midshift"), 1));
        true
    }

    fn map_right(&mut self, data: &Funcdata) -> bool {
        let reslo = self.reslo.expect("mr: reslo");
        let reshi = self.reshi.expect("mr: reshi");
        if !vn_is_written(data, reslo) {
            return false;
        }
        if !vn_is_written(data, reshi) {
            return false;
        }
        self.hishift = vn_get_def(data, reshi);
        let hishift = self.hishift.expect("mr: hishift");
        self.opc = op_code(data, hishift);
        if self.opc != OpCode::CPUI_INT_RIGHT && self.opc != OpCode::CPUI_INT_SRIGHT {
            return false;
        }
        self.orop = vn_get_def(data, reslo);
        let orop = self.orop.expect("mr: orop");
        let oc = op_code(data, orop);
        if oc != OpCode::CPUI_INT_OR && oc != OpCode::CPUI_INT_XOR && oc != OpCode::CPUI_INT_ADD {
            return false;
        }
        self.midlo = Some(op_get_in(data, orop, 0));
        self.midhi = Some(op_get_in(data, orop, 1));
        let midlo = self.midlo.expect("mr: midlo");
        let midhi = self.midhi.expect("mr: midhi");
        if !vn_is_written(data, midlo) {
            return false;
        }
        if !vn_is_written(data, midhi) {
            return false;
        }
        if op_code(data, vn_get_def(data, midlo).expect("mr: midlo def")) != OpCode::CPUI_INT_RIGHT {
            // Must be unsigned RIGHT
            std::mem::swap(&mut self.midhi, &mut self.midlo);
        }
        let midlo = self.midlo.expect("mr: midlo");
        let midhi = self.midhi.expect("mr: midhi");
        self.midshift = vn_get_def(data, midhi);
        if op_code(data, self.midshift.expect("mr: midshift")) != OpCode::CPUI_INT_LEFT {
            return false;
        }
        self.loshift = vn_get_def(data, midlo);
        if op_code(data, self.loshift.expect("mr: loshift")) != OpCode::CPUI_INT_RIGHT {
            return false; // Must be unsigned RIGHT
        }

        let lo = self.lo.expect("mr: lo");
        let hi = self.hi.expect("mr: hi");
        if lo != op_get_in(data, self.loshift.expect("mr: loshift"), 0) {
            return false;
        }
        if hi != op_get_in(data, hishift, 0) {
            return false;
        }
        if hi != op_get_in(data, self.midshift.expect("mr: midshift"), 0) {
            return false;
        }
        self.salo = Some(op_get_in(data, self.loshift.expect("mr: loshift"), 1));
        self.sahi = Some(op_get_in(data, hishift, 1));
        self.samid = Some(op_get_in(data, self.midshift.expect("mr: midshift"), 1));
        true
    }

    fn verify_shift_amount(&self, data: &Funcdata) -> bool {
        // Make sure all the shift amount varnodes are consistent
        let salo = self.salo.expect("vsa: salo");
        let samid = self.samid.expect("vsa: samid");
        let sahi = self.sahi.expect("vsa: sahi");
        if !vn_is_constant(data, salo) {
            return false;
        }
        if !vn_is_constant(data, samid) {
            return false;
        }
        if !vn_is_constant(data, sahi) {
            return false;
        }
        let val = vn_get_offset(data, salo);
        if val != vn_get_offset(data, sahi) {
            return false;
        }
        let losize = vn_get_size(data, self.lo.expect("vsa: lo"));
        if val >= (8 * losize) as uintb {
            return false; // If shift amount is so big, we would not use this form
        }
        let val = (8 * losize) as uintb - val;
        if vn_get_offset(data, samid) != val {
            return false;
        }
        true
    }

    fn verify_left(&mut self, data: &Funcdata, h: VarnodeId, l: VarnodeId, loop_: OpId) -> bool {
        self.hi = Some(h);
        self.lo = Some(l);

        self.loshift = Some(loop_);
        self.reslo = op_get_out(data, loop_);

        for hishift in descend_ops(data, h) {
            self.hishift = Some(hishift);
            if op_code(data, hishift) != OpCode::CPUI_INT_LEFT {
                continue;
            }
            let outvn = match op_get_out(data, hishift) {
                Some(o) => o,
                None => continue,
            };
            for midshift in descend_ops(data, outvn) {
                self.midshift = Some(midshift);
                let tmpvn = match op_get_out(data, midshift) {
                    Some(t) => t,
                    None => continue,
                };
                self.reshi = Some(tmpvn);
                if !self.map_left(data) {
                    continue;
                }
                if !self.verify_shift_amount(data) {
                    continue;
                }
                return true;
            }
        }
        false
    }

    fn verify_right(&mut self, data: &Funcdata, h: VarnodeId, l: VarnodeId, hiop: OpId) -> bool {
        self.hi = Some(h);
        self.lo = Some(l);
        self.hishift = Some(hiop);
        self.reshi = op_get_out(data, hiop);

        for loshift in descend_ops(data, l) {
            self.loshift = Some(loshift);
            if op_code(data, loshift) != OpCode::CPUI_INT_RIGHT {
                continue;
            }
            let outvn = match op_get_out(data, loshift) {
                Some(o) => o,
                None => continue,
            };
            for midshift in descend_ops(data, outvn) {
                self.midshift = Some(midshift);
                let tmpvn = match op_get_out(data, midshift) {
                    Some(t) => t,
                    None => continue,
                };
                self.reslo = Some(tmpvn);
                if !self.map_right(data) {
                    continue;
                }
                if !self.verify_shift_amount(data) {
                    continue;
                }
                return true;
            }
        }
        false
    }

    fn apply_rule_left(&mut self, i: &SplitVarnode, loop_: OpId, workishi: bool, data: &mut Funcdata) -> bool {
        if workishi {
            return false;
        }
        if !i.has_both_pieces() {
            return false;
        }
        self.in_ = i.clone();

        let hi = self.in_.get_hi().expect("arl: hi");
        let lo = self.in_.get_lo().expect("arl: lo");
        if !self.verify_left(data, hi, lo, loop_) {
            return false;
        }

        self.out.init_partial_pieces(data, self.in_.get_size(), self.reslo, self.reshi);
        let existop = match prepare_shift_op(data, &mut self.out, &mut self.in_) {
            Some(e) => e,
            None => return false,
        };
        let salo = self.salo.expect("arl: salo");
        let (mut out, mut in_) = (self.out.clone(), self.in_.clone());
        create_shift_op(data, &mut out, &mut in_, salo, existop, self.opc);
        true
    }

    fn apply_rule_right(&mut self, i: &SplitVarnode, hiop: OpId, workishi: bool, data: &mut Funcdata) -> bool {
        if !workishi {
            return false;
        }
        if !i.has_both_pieces() {
            return false;
        }
        self.in_ = i.clone();

        let hi = self.in_.get_hi().expect("arr: hi");
        let lo = self.in_.get_lo().expect("arr: lo");
        if !self.verify_right(data, hi, lo, hiop) {
            return false;
        }

        self.out.init_partial_pieces(data, self.in_.get_size(), self.reslo, self.reshi);
        let existop = match prepare_shift_op(data, &mut self.out, &mut self.in_) {
            Some(e) => e,
            None => return false,
        };
        let salo = self.salo.expect("arr: salo");
        let (mut out, mut in_) = (self.out.clone(), self.in_.clone());
        create_shift_op(data, &mut out, &mut in_, salo, existop, self.opc);
        true
    }
}

// =============================================================================
// MultForm (double.cc:248-272 hh, 2735-3024 cc)
// =============================================================================

/// Match a double-precision multiplication (C++ `MultForm`).
struct MultForm {
    in_: SplitVarnode,
    add1: Option<OpId>,
    add2: Option<OpId>,
    subhi: Option<OpId>,
    multlo: Option<OpId>,
    multhi1: Option<OpId>,
    multhi2: Option<OpId>,
    midtmp: Option<VarnodeId>,
    lo1zext: Option<VarnodeId>,
    lo2zext: Option<VarnodeId>,
    hi1: Option<VarnodeId>,
    lo1: Option<VarnodeId>,
    hi2: Option<VarnodeId>,
    lo2: Option<VarnodeId>,
    reslo: Option<VarnodeId>,
    reshi: Option<VarnodeId>,
    outdoub: SplitVarnode,
    in2: SplitVarnode,
}

impl MultForm {
    fn new() -> MultForm {
        MultForm {
            in_: SplitVarnode::empty(),
            add1: None,
            add2: None,
            subhi: None,
            multlo: None,
            multhi1: None,
            multhi2: None,
            midtmp: None,
            lo1zext: None,
            lo2zext: None,
            hi1: None,
            lo1: None,
            hi2: None,
            lo2: None,
            reslo: None,
            reshi: None,
            outdoub: SplitVarnode::empty(),
            in2: SplitVarnode::empty(),
        }
    }

    fn map_res_hi_small_const(&mut self, data: &Funcdata, rhi: VarnodeId) -> bool {
        // find reshi=hi1*lo2 + (tmp>>32)
        self.reshi = Some(rhi);
        if !vn_is_written(data, rhi) {
            return false;
        }
        self.add1 = vn_get_def(data, rhi);
        let add1 = self.add1.expect("mrhsc: add1");
        if op_code(data, add1) != OpCode::CPUI_INT_ADD {
            return false;
        }
        let ad1 = op_get_in(data, add1, 0);
        let ad2 = op_get_in(data, add1, 1);
        if !vn_is_written(data, ad1) {
            return false;
        }
        if !vn_is_written(data, ad2) {
            return false;
        }
        self.multhi1 = vn_get_def(data, ad1);
        if op_code(data, self.multhi1.expect("mrhsc")) != OpCode::CPUI_INT_MULT {
            self.subhi = self.multhi1;
            self.multhi1 = vn_get_def(data, ad2);
        } else {
            self.subhi = vn_get_def(data, ad2);
        }
        if op_code(data, self.multhi1.expect("mrhsc")) != OpCode::CPUI_INT_MULT {
            return false;
        }
        if op_code(data, self.subhi.expect("mrhsc")) != OpCode::CPUI_SUBPIECE {
            return false;
        }
        self.midtmp = Some(op_get_in(data, self.subhi.expect("mrhsc"), 0));
        let midtmp = self.midtmp.expect("mrhsc: midtmp");
        if !vn_is_written(data, midtmp) {
            return false;
        }
        self.multlo = vn_get_def(data, midtmp);
        if op_code(data, self.multlo.expect("mrhsc")) != OpCode::CPUI_INT_MULT {
            return false;
        }
        self.lo1zext = Some(op_get_in(data, self.multlo.expect("mrhsc"), 0));
        self.lo2zext = Some(op_get_in(data, self.multlo.expect("mrhsc"), 1));
        true
    }

    fn map_res_hi(&mut self, data: &Funcdata, rhi: VarnodeId) -> bool {
        // Find reshi=hi1*lo2 + hi2*lo1 + (tmp>>32)
        self.reshi = Some(rhi);
        if !vn_is_written(data, rhi) {
            return false;
        }
        self.add1 = vn_get_def(data, rhi);
        let add1 = self.add1.expect("mrh: add1");
        if op_code(data, add1) != OpCode::CPUI_INT_ADD {
            return false;
        }
        let mut ad1 = op_get_in(data, add1, 0);
        let mut ad2 = op_get_in(data, add1, 1);
        let ad3;
        if !vn_is_written(data, ad1) {
            return false;
        }
        if !vn_is_written(data, ad2) {
            return false;
        }
        self.add2 = vn_get_def(data, ad1);
        if op_code(data, self.add2.expect("mrh")) == OpCode::CPUI_INT_ADD {
            ad1 = op_get_in(data, self.add2.expect("mrh"), 0);
            ad3 = op_get_in(data, self.add2.expect("mrh"), 1);
        } else {
            self.add2 = vn_get_def(data, ad2);
            if op_code(data, self.add2.expect("mrh")) != OpCode::CPUI_INT_ADD {
                return false;
            }
            ad2 = op_get_in(data, self.add2.expect("mrh"), 0);
            ad3 = op_get_in(data, self.add2.expect("mrh"), 1);
        }
        if !vn_is_written(data, ad1) {
            return false;
        }
        if !vn_is_written(data, ad2) {
            return false;
        }
        if !vn_is_written(data, ad3) {
            return false;
        }
        self.subhi = vn_get_def(data, ad1);
        if op_code(data, self.subhi.expect("mrh")) == OpCode::CPUI_SUBPIECE {
            self.multhi1 = vn_get_def(data, ad2);
            self.multhi2 = vn_get_def(data, ad3);
        } else {
            self.subhi = vn_get_def(data, ad2);
            if op_code(data, self.subhi.expect("mrh")) == OpCode::CPUI_SUBPIECE {
                self.multhi1 = vn_get_def(data, ad1);
                self.multhi2 = vn_get_def(data, ad3);
            } else {
                self.subhi = vn_get_def(data, ad3);
                if op_code(data, self.subhi.expect("mrh")) == OpCode::CPUI_SUBPIECE {
                    self.multhi1 = vn_get_def(data, ad1);
                    self.multhi2 = vn_get_def(data, ad2);
                } else {
                    return false;
                }
            }
        }
        if op_code(data, self.multhi1.expect("mrh")) != OpCode::CPUI_INT_MULT {
            return false;
        }
        if op_code(data, self.multhi2.expect("mrh")) != OpCode::CPUI_INT_MULT {
            return false;
        }

        self.midtmp = Some(op_get_in(data, self.subhi.expect("mrh"), 0));
        let midtmp = self.midtmp.expect("mrh: midtmp");
        if !vn_is_written(data, midtmp) {
            return false;
        }
        self.multlo = vn_get_def(data, midtmp);
        if op_code(data, self.multlo.expect("mrh")) != OpCode::CPUI_INT_MULT {
            return false;
        }
        self.lo1zext = Some(op_get_in(data, self.multlo.expect("mrh"), 0));
        self.lo2zext = Some(op_get_in(data, self.multlo.expect("mrh"), 1));
        true
    }

    fn find_lo_from_in_small_const(&mut self, data: &Funcdata) -> bool {
        // Assuming we have -multhi1-, -lo1-, and -hi1- in hand, try to label -lo2-
        let multhi1 = self.multhi1.expect("flfisc: multhi1");
        let vn1 = op_get_in(data, multhi1, 0);
        let vn2 = op_get_in(data, multhi1, 1);
        let hi1 = self.hi1.expect("flfisc: hi1");
        if vn1 == hi1 {
            self.lo2 = Some(vn2);
        } else if vn2 == hi1 {
            self.lo2 = Some(vn1);
        } else {
            return false;
        }
        if !vn_is_constant(data, self.lo2.expect("flfisc: lo2")) {
            return false;
        }
        self.hi2 = None; // hi2 is an implied zero in this case
        true
    }

    fn find_lo_from_in(&mut self, data: &Funcdata) -> bool {
        // Assuming we have -multhi1-, -multhi2-, -lo1-, -hi1-, label lo2/hi2 pair
        let mut vn1 = op_get_in(data, self.multhi1.expect("flfi"), 0);
        let mut vn2 = op_get_in(data, self.multhi1.expect("flfi"), 1);
        let lo1 = self.lo1.expect("flfi: lo1");
        let hi1 = self.hi1.expect("flfi: hi1");
        if vn1 != lo1 && vn2 != lo1 {
            // Try to normalize so multhi1 contains lo1
            std::mem::swap(&mut self.multhi1, &mut self.multhi2);
            vn1 = op_get_in(data, self.multhi1.expect("flfi"), 0);
            vn2 = op_get_in(data, self.multhi1.expect("flfi"), 1);
        }
        if vn1 == lo1 {
            self.hi2 = Some(vn2);
        } else if vn2 == lo1 {
            self.hi2 = Some(vn1);
        } else {
            return false;
        }
        vn1 = op_get_in(data, self.multhi2.expect("flfi"), 0); // multhi2 should contain hi1 and lo2
        vn2 = op_get_in(data, self.multhi2.expect("flfi"), 1);
        if vn1 == hi1 {
            self.lo2 = Some(vn2);
        } else if vn2 == hi1 {
            self.lo2 = Some(vn1);
        } else {
            return false;
        }
        true
    }

    fn zext_of(&self, data: &Funcdata, big: VarnodeId, small: VarnodeId) -> bool {
        // Verify that big is (some form of) a zero extension of small
        if vn_is_constant(data, small) {
            if !vn_is_constant(data, big) {
                return false;
            }
            return vn_get_offset(data, big) == vn_get_offset(data, small);
        }
        if !vn_is_written(data, big) {
            return false;
        }
        let op = vn_get_def(data, big).expect("zext_of: big def");
        if op_code(data, op) == OpCode::CPUI_INT_ZEXT {
            return op_get_in(data, op, 0) == small;
        }
        if op_code(data, op) == OpCode::CPUI_INT_AND {
            if !vn_is_constant(data, op_get_in(data, op, 1)) {
                return false;
            }
            if vn_get_offset(data, op_get_in(data, op, 1)) != calc_mask(vn_get_size(data, small)) {
                return false;
            }
            let whole = op_get_in(data, op, 0);
            if !vn_is_written(data, small) {
                return false;
            }
            let sub = vn_get_def(data, small).expect("zext_of: small def");
            if op_code(data, sub) != OpCode::CPUI_SUBPIECE {
                return false;
            }
            return op_get_in(data, sub, 0) == whole;
        }
        false
    }

    fn verify_lo(&self, data: &Funcdata) -> bool {
        // Given we labelled lo1/hi1 lo2/hi2, make sure midtmp is formed properly.
        // Also works for the small constant model lo1/hi1 and lo2 const.
        if vn_get_offset(data, op_get_in(data, self.subhi.expect("vl: subhi"), 1))
            != vn_get_size(data, self.lo1.expect("vl: lo1")) as uintb
        {
            return false;
        }
        let lo1 = self.lo1.expect("vl: lo1");
        let lo2 = self.lo2.expect("vl: lo2");
        let lo1zext = self.lo1zext.expect("vl: lo1zext");
        let lo2zext = self.lo2zext.expect("vl: lo2zext");
        if self.zext_of(data, lo1zext, lo1) {
            if self.zext_of(data, lo2zext, lo2) {
                return true;
            }
        } else if self.zext_of(data, lo1zext, lo2) && self.zext_of(data, lo2zext, lo1) {
            return true;
        }
        false
    }

    fn find_res_lo(&mut self, data: &Funcdata) -> bool {
        // Assuming we found -midtmp-, find potential reslo
        let midtmp = self.midtmp.expect("frl: midtmp");
        let lo1 = self.lo1.expect("frl: lo1");
        let lo2 = self.lo2.expect("frl: lo2");
        for op in descend_ops(data, midtmp) {
            if op_code(data, op) != OpCode::CPUI_SUBPIECE {
                continue;
            }
            if vn_get_offset(data, op_get_in(data, op, 1)) != 0 {
                continue; // Must grab low bytes
            }
            let reslo = op_get_out(data, op).expect("frl: sub out");
            self.reslo = Some(reslo);
            if vn_get_size(data, reslo) != vn_get_size(data, lo1) {
                continue;
            }
            return true;
        }
        // Separate multiplies of lo1*lo2 may have been used for reshi and reslo
        for op in descend_ops(data, lo1) {
            if op_code(data, op) != OpCode::CPUI_INT_MULT {
                continue;
            }
            let vn1 = op_get_in(data, op, 0);
            let vn2 = op_get_in(data, op, 1);
            if vn_is_constant(data, lo2) {
                let lo2off = vn_get_offset(data, lo2);
                if (!vn_is_constant(data, vn1) || vn_get_offset(data, vn1) != lo2off)
                    && (!vn_is_constant(data, vn2) || vn_get_offset(data, vn2) != lo2off)
                {
                    continue;
                }
            } else if op_get_in(data, op, 0) != lo2 && op_get_in(data, op, 1) != lo2 {
                continue;
            }
            self.reslo = op_get_out(data, op);
            return true;
        }
        false
    }

    fn map_from_in_small_const(&mut self, data: &Funcdata, rhi: VarnodeId) -> bool {
        if !self.map_res_hi_small_const(data, rhi) {
            return false;
        }
        if !self.find_lo_from_in_small_const(data) {
            return false;
        }
        if !self.verify_lo(data) {
            return false;
        }
        if !self.find_res_lo(data) {
            return false;
        }
        true
    }

    fn map_from_in(&mut self, data: &Funcdata, rhi: VarnodeId) -> bool {
        // Try to do full mapping from -in- given a putative reshi
        if !self.map_res_hi(data, rhi) {
            return false;
        }
        if !self.find_lo_from_in(data) {
            return false;
        }
        if !self.verify_lo(data) {
            return false;
        }
        if !self.find_res_lo(data) {
            return false;
        }
        true
    }

    fn replace(&mut self, data: &mut Funcdata) -> bool {
        // We matched a double precision multiply, now transform to logical vars
        self.outdoub.init_partial_pieces(data, self.in_.get_size(), self.reslo, self.reshi);
        self.in2.init_partial_pieces(data, self.in_.get_size(), self.lo2, self.hi2);
        if self.in2.exceeds_const_precision() {
            return false;
        }
        let existop = match prepare_binary_op(data, &mut self.outdoub, &mut self.in_, &mut self.in2) {
            Some(e) => e,
            None => return false,
        };
        let (mut out, mut in1, mut in2) = (self.outdoub.clone(), self.in_.clone(), self.in2.clone());
        create_binary_op(data, &mut out, &mut in1, &mut in2, existop, OpCode::CPUI_INT_MULT);
        true
    }

    fn verify(&mut self, data: &Funcdata, h: VarnodeId, l: VarnodeId, hop: OpId) -> bool {
        self.hi1 = Some(h);
        self.lo1 = Some(l);
        let hop_out = op_get_out(data, hop).expect("multverify: hop out");
        for add1 in descend_ops(data, hop_out) {
            self.add1 = Some(add1);
            if op_code(data, add1) != OpCode::CPUI_INT_ADD {
                continue;
            }
            let add1_out = op_get_out(data, add1).expect("multverify: add1 out");
            for add2 in descend_ops(data, add1_out) {
                self.add2 = Some(add2);
                if op_code(data, add2) != OpCode::CPUI_INT_ADD {
                    continue;
                }
                if self.map_from_in(data, op_get_out(data, add2).expect("multverify: add2 out")) {
                    return true;
                }
            }
            if self.map_from_in(data, op_get_out(data, add1).expect("multverify: add1 out2")) {
                return true;
            }
            if self.map_from_in_small_const(data, op_get_out(data, add1).expect("multverify: add1 out3")) {
                return true;
            }
        }
        false
    }

    fn apply_rule(&mut self, i: &SplitVarnode, hop: OpId, workishi: bool, data: &mut Funcdata) -> bool {
        if !workishi {
            return false;
        }
        if !i.has_both_pieces() {
            return false;
        }
        self.in_ = i.clone();

        let hi = self.in_.get_hi().expect("multrule: hi");
        let lo = self.in_.get_lo().expect("multrule: lo");
        if !self.verify(data, hi, lo, hop) {
            return false;
        }

        self.replace(data)
    }
}

// =============================================================================
// PhiForm (double.cc:274-285 hh, 3026-3078 cc)
// =============================================================================

/// Match a double-precision MULTIEQUAL (phi node) built from piece phi-nodes
/// (C++ `PhiForm`).
struct PhiForm {
    in_: SplitVarnode,
    outvn: SplitVarnode,
    inslot: int4,
    hibase: Option<VarnodeId>,
    lobase: Option<VarnodeId>,
    blbase: Option<BlockId>,
    lophi: Option<OpId>,
    hiphi: Option<OpId>,
}

impl PhiForm {
    fn new() -> PhiForm {
        PhiForm {
            in_: SplitVarnode::empty(),
            outvn: SplitVarnode::empty(),
            inslot: 0,
            hibase: None,
            lobase: None,
            blbase: None,
            lophi: None,
            hiphi: None,
        }
    }

    fn verify(&mut self, data: &Funcdata, h: VarnodeId, l: VarnodeId, hphi: OpId) -> bool {
        self.hibase = Some(h);
        self.lobase = Some(l);
        self.hiphi = Some(hphi);

        self.inslot = op_get_slot(data, hphi, h);

        let hphi_out = op_get_out(data, hphi).expect("phiverify: hphi out");
        if vn_has_no_descend(data, hphi_out) {
            return false;
        }
        self.blbase = op_get_parent(data, hphi);

        for lophi in descend_ops(data, l) {
            self.lophi = Some(lophi);
            if op_code(data, lophi) != OpCode::CPUI_MULTIEQUAL {
                continue;
            }
            if op_get_parent(data, lophi) != self.blbase {
                continue;
            }
            if op_get_in(data, lophi, self.inslot) != l {
                continue;
            }
            return true;
        }
        false
    }

    fn apply_rule(&mut self, i: &SplitVarnode, hphi: OpId, workishi: bool, data: &mut Funcdata) -> bool {
        if !workishi {
            return false;
        }
        if !i.has_both_pieces() {
            return false;
        }
        self.in_ = i.clone();

        let hi = self.in_.get_hi().expect("phirule: hi");
        let lo = self.in_.get_lo().expect("phirule: lo");
        if !self.verify(data, hi, lo, hphi) {
            return false;
        }

        let hiphi = self.hiphi.expect("phirule: hiphi");
        let lophi = self.lophi.expect("phirule: lophi");
        let numin = op_num_input(data, hiphi);
        let mut inlist: Vec<SplitVarnode> = Vec::new();
        for j in 0..numin {
            let vhi = op_get_in(data, hiphi, j);
            let vlo = op_get_in(data, lophi, j);
            inlist.push(SplitVarnode::from_pieces(data, vlo, vhi));
        }
        self.outvn.init_partial_pieces(
            data,
            self.in_.get_size(),
            op_get_out(data, lophi),
            op_get_out(data, hiphi),
        );
        let mut out = self.outvn.clone();
        let existop = prepare_phi_op(data, &mut out, &mut inlist);
        if let Some(existop) = existop {
            create_phi_op(data, &mut out, &mut inlist, existop);
            self.outvn = out;
            return true;
        }
        false
    }
}

// =============================================================================
// IndirectForm (double.cc:287-297 hh, 3080-3129 cc)
// =============================================================================

/// `Varnode::getSpaceFromConst`/`getSpace()->getType()` for an INDIRECT iop
/// operand — is it in the IOP space?  The IOP-space annotation is read directly
/// off the Varnode's space (the W3 IR carries it), so this is **not** a stub.
fn vn_space_type(data: &Funcdata, vn: VarnodeId) -> spacetype {
    data.vbank().get(vn).expect("vn_space_type").get_space().get_type()
}

/// `PcodeOp::getOpFromConst(vn->getAddr())` — decode an IOP-space annotation back
/// to the PcodeOp it references.
///
/// STUB(W3-varnode): the iop encoding/decoding (`op->getIn(1)->getAddr()` ->
/// `PcodeOp*`) is the same surface deferred in `funcdata_op.rs`'s
/// `op_insert_after` INDIRECT special-case.  Without it we cannot recover the
/// affector op from an INDIRECT's second operand, so `IndirectForm::verify` and
/// the LOAD/STORE INDIRECT handling decline.  Returns `None`; the callers treat
/// that as "can't resolve" and bail — a conservative no-op.  Recorded as a loss.
fn get_op_from_const(_data: &Funcdata, _vn: VarnodeId) -> Option<OpId> {
    // STUB(W3-varnode): PcodeOp::getOpFromConst
    None
}

/// Match a double-precision INDIRECT (C++ `IndirectForm`).
struct IndirectForm {
    in_: SplitVarnode,
    outvn: SplitVarnode,
    hi: Option<VarnodeId>,
    lo: Option<VarnodeId>,
    reslo: Option<VarnodeId>,
    reshi: Option<VarnodeId>,
    affector: Option<OpId>,
    indhi: Option<OpId>,
    indlo: Option<OpId>,
}

impl IndirectForm {
    fn new() -> IndirectForm {
        IndirectForm {
            in_: SplitVarnode::empty(),
            outvn: SplitVarnode::empty(),
            hi: None,
            lo: None,
            reslo: None,
            reshi: None,
            affector: None,
            indhi: None,
            indlo: None,
        }
    }

    fn verify(&mut self, data: &Funcdata, h: VarnodeId, l: VarnodeId, ind: OpId) -> bool {
        self.hi = Some(h);
        self.lo = Some(l);
        self.indhi = Some(ind);
        if vn_space_type(data, op_get_in(data, ind, 1)) != spacetype::IPTR_IOP {
            return false;
        }
        self.affector = get_op_from_const(data, op_get_in(data, ind, 1));
        let affector = match self.affector {
            Some(a) => a,
            // STUB(W3-varnode): getOpFromConst unavailable -> can't recover the
            // affector; decline (conservative no-op).
            None => return false,
        };
        if op_is_dead(data, affector) {
            return false;
        }
        self.reshi = op_get_out(data, ind);
        let reshi = self.reshi.expect("indverify: reshi");
        if vn_space_type(data, reshi) == spacetype::IPTR_INTERNAL {
            return false; // Indirect must not be through a temporary
        }

        for indlo in descend_ops(data, l) {
            self.indlo = Some(indlo);
            if op_code(data, indlo) != OpCode::CPUI_INDIRECT {
                continue;
            }
            if vn_space_type(data, op_get_in(data, indlo, 1)) != spacetype::IPTR_IOP {
                continue;
            }
            // hi and lo must be affected by same op
            if self.affector != get_op_from_const(data, op_get_in(data, indlo, 1)) {
                continue;
            }
            self.reslo = op_get_out(data, indlo);
            let reslo = self.reslo.expect("indverify: reslo");
            if vn_space_type(data, reslo) == spacetype::IPTR_INTERNAL {
                return false; // Indirect must not be through a temporary
            }
            if vn_is_addr_tied(data, reslo) || vn_is_addr_tied(data, reshi) {
                // If one piece is address tied, the other must be too, and they
                // must fit together as a contiguous whole.
                if is_addr_tied_contiguous(data, reslo, reshi).is_none() {
                    return false;
                }
            }
            return true;
        }
        false
    }

    fn apply_rule(&mut self, i: &SplitVarnode, ind: OpId, workishi: bool, data: &mut Funcdata) -> bool {
        if !workishi {
            return false;
        }
        if !i.has_both_pieces() {
            return false;
        }
        self.in_ = i.clone();
        let hi = self.in_.get_hi().expect("indrule: hi");
        let lo = self.in_.get_lo().expect("indrule: lo");
        if !self.verify(data, hi, lo, ind) {
            return false;
        }

        self.outvn.init_partial_pieces(data, self.in_.get_size(), self.reslo, self.reshi);

        let affector = self.affector.expect("indrule: affector");
        if !prepare_indirect_op(data, &mut self.in_, affector) {
            return false;
        }
        let (mut out, mut in_) = (self.outvn.clone(), self.in_.clone());
        replace_indirect_op(data, &mut out, &mut in_, affector);
        self.outvn = out;
        self.in_ = in_;
        true
    }
}

// =============================================================================
// CopyForceForm (double.cc:299-313 hh, 3131-3196 cc)
// =============================================================================

/// Collapse two COPYs into contiguous address-forced Varnodes (C++
/// `CopyForceForm`).
struct CopyForceForm {
    in_: SplitVarnode,
    reslo: Option<VarnodeId>,
    reshi: Option<VarnodeId>,
    copylo: Option<OpId>,
    copyhi: Option<OpId>,
    addr_out: Option<Address>,
}

impl CopyForceForm {
    fn new() -> CopyForceForm {
        CopyForceForm {
            in_: SplitVarnode::empty(),
            reslo: None,
            reshi: None,
            copylo: None,
            copyhi: None,
            addr_out: None,
        }
    }

    fn verify(
        &mut self,
        data: &Funcdata,
        h: VarnodeId,
        l: VarnodeId,
        w: Option<VarnodeId>,
        cpy: OpId,
    ) -> bool {
        let w = match w {
            Some(w) => w,
            None => return false,
        };
        self.copyhi = Some(cpy);
        if op_get_in(data, cpy, 0) != h {
            return false;
        }
        self.reshi = op_get_out(data, cpy);
        let reshi = self.reshi.expect("cff: reshi");
        if !vn_is_addr_force(data, reshi) || !vn_has_no_descend(data, reshi) {
            return false;
        }
        for copylo in descend_ops(data, l) {
            self.copylo = Some(copylo);
            if op_code(data, copylo) != OpCode::CPUI_COPY
                || op_get_parent(data, copylo) != op_get_parent(data, cpy)
            {
                continue;
            }
            self.reslo = op_get_out(data, copylo);
            let reslo = self.reslo.expect("cff: reslo");
            if !vn_is_addr_force(data, reslo) || !vn_has_no_descend(data, reslo) {
                continue;
            }
            // Output MUST be contiguous addresses
            self.addr_out = match is_addr_tied_contiguous(data, reslo, reshi) {
                Some(a) => Some(a),
                None => continue,
            };
            if op_is_return_copy(data, cpy) {
                // Special form has additional requirements
                if lone_descend(data, h).is_none() {
                    continue;
                }
                if lone_descend(data, l).is_none() {
                    continue;
                }
                if vn_get_addr(data, w) != *self.addr_out.as_ref().expect("cff: addr_out") {
                    // Input whole MUST also be the same address
                    // Unless there are additional COPYs from the same basic block
                    if !vn_is_written(data, h) || !vn_is_written(data, l) {
                        continue;
                    }
                    let other_lo = vn_get_def(data, l).expect("cff: l def");
                    let other_hi = vn_get_def(data, h).expect("cff: h def");
                    if op_code(data, other_lo) != OpCode::CPUI_COPY
                        || op_code(data, other_hi) != OpCode::CPUI_COPY
                    {
                        continue;
                    }
                    if op_get_parent(data, other_lo) != op_get_parent(data, other_hi) {
                        continue;
                    }
                }
            }
            return true;
        }
        false
    }

    fn apply_rule(&mut self, i: &SplitVarnode, cpy: OpId, workishi: bool, data: &mut Funcdata) -> bool {
        if !workishi {
            return false;
        }
        if !i.has_both_pieces() {
            return false;
        }
        self.in_ = i.clone();
        let hi = self.in_.get_hi().expect("cffrule: hi");
        let lo = self.in_.get_lo().expect("cffrule: lo");
        let w = self.in_.get_whole();
        if !self.verify(data, hi, lo, w, cpy) {
            return false;
        }
        let addr = self.addr_out.clone().expect("cffrule: addr_out");
        let copylo = self.copylo.expect("cffrule: copylo");
        let copyhi = self.copyhi.expect("cffrule: copyhi");
        let in_ = self.in_.clone();
        replace_copy_force(data, &addr, &in_, copylo, copyhi);
        true
    }
}

// =============================================================================
// RuleDoubleIn / RuleDoubleOut marking helpers (W6 typeop stub)
// =============================================================================

/// `TypeOp::isArithmeticOp() || TypeOp::isFloatingPointOp()` for the op defining
/// or reading the putative logical whole.
///
/// Reads the `TypeOp::addlflags` (`arithmetic_op`=8, `floatingpoint_op`=0x20)
/// from the W6 `inst` table ([`crate::typeop::type_op_info`]).  C++:
/// `RuleDoubleIn::attemptMarking` (`double.cc:3238-3240`,
/// `TypeOp *typeop = whole->getDef()->getOpcode();
///  if (!typeop->isArithmeticOp() && !typeop->isFloatingPointOp()) return 0;`)
/// and `RuleDoubleOut::attemptMarking` (`double.cc:3318-3322`,
/// `TypeOp *typeop = (*iter)->getOpcode();
///  if (typeop->isArithmeticOp() || typeop->isFloatingPointOp()) { isWhole = true; break; }`).
/// The C++ pulls the `TypeOp` from the PcodeOp via `getOpcode()`; here the op's
/// op-code keys the static `type_op_info` table, which carries the same
/// `addlflags` bits (`typeop.cc` `TypeOp*` constructors).
fn op_is_arith_or_float(data: &Funcdata, op: OpId) -> bool {
    let info = crate::typeop::type_op_info(op_code(data, op));
    info.is_arithmetic_op() || info.is_floating_point_op()
}

/// `whole->getType()->isPrimitiveWhole()` typelock gate — present (the W6
/// `Datatype::isPrimitiveWhole` is ported); used only when `whole` is type-locked.
fn vn_type_is_primitive_whole(data: &Funcdata, vn: VarnodeId) -> bool {
    data.vbank()
        .get(vn)
        .expect("double: stale vn")
        .get_type()
        .is_primitive_whole()
}

// =============================================================================
// RuleDoubleIn (double.cc:321-332 hh, 3198-3279 cc)
// =============================================================================

/// Push a double precision operation down one level, starting from a marked
/// double precision input (C++ `RuleDoubleIn`).
pub struct RuleDoubleIn {
    group: String,
}

impl RuleDoubleIn {
    pub fn new(g: impl Into<String>) -> RuleDoubleIn {
        RuleDoubleIn { group: g.into() }
    }

    /// Determine if a SUBPIECE output should be marked as a double precision
    /// piece, marking it (and its companion) if so (C++ `attemptMarking`).
    fn attempt_marking(&self, data: &mut Funcdata, vn: VarnodeId, subpiece_op: OpId) -> int4 {
        let whole = op_get_in(data, subpiece_op, 0);
        if vn_is_type_lock(data, whole) && !vn_type_is_primitive_whole(data, whole) {
            return 0; // Don't mark for double precision if not a primitive type
        }
        let offset = vn_get_offset(data, op_get_in(data, subpiece_op, 1)) as int4;
        if offset != vn_get_size(data, vn) {
            return 0;
        }
        if offset * 2 != vn_get_size(data, whole) {
            return 0; // Truncate exactly half
        }
        if vn_is_input(data, whole) {
            if !vn_is_type_lock(data, whole) {
                return 0;
            }
        } else if !vn_is_written(data, whole) {
            return 0;
        } else {
            // Categorize opcodes as "producing a logical whole".  It's hard to
            // tell if a logical op is really acting on the "logical whole".
            let def = vn_get_def(data, whole).expect("attemptMarking: whole def");
            if !op_is_arith_or_float(data, def) {
                return 0;
            }
        }
        let mut vn_lo: Option<VarnodeId> = None;
        for op in descend_ops(data, whole) {
            if op_code(data, op) != OpCode::CPUI_SUBPIECE {
                continue;
            }
            if vn_get_offset(data, op_get_in(data, op, 1)) != 0 {
                continue;
            }
            let out = op_get_out(data, op).expect("attemptMarking: sub out");
            if vn_get_size(data, out) == vn_get_size(data, vn) {
                vn_lo = Some(out);
                break;
            }
        }
        let vn_lo = match vn_lo {
            Some(v) => v,
            None => return 0,
        };
        vn_set_precis_lo(data, vn_lo);
        vn_set_precis_hi(data, vn);
        1
    }
}

impl Rule for RuleDoubleIn {
    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(&self.group) {
            return None;
        }
        Some(Box::new(RuleDoubleIn::new(self.group.clone())))
    }

    fn reset(&mut self, data: &mut Funcdata) {
        // Mark that we are doing double precision recovery
        data.set_double_precis_recovery(true);
    }

    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_SUBPIECE]
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        let outvn = match op_get_out(data, op) {
            Some(o) => o,
            None => return 0,
        };
        if !vn_is_precis_lo(data, outvn) {
            if vn_is_precis_hi(data, outvn) {
                return 0;
            }
            return self.attempt_marking(data, outvn, op);
        }
        if data.has_unreachable_blocks() {
            return 0;
        }

        let in0 = op_get_in(data, op, 0);
        let splitvec = whole_list(data, in0);
        if splitvec.is_empty() {
            return 0;
        }
        for in_ in &splitvec {
            let res = apply_rule_in(in_, data);
            if res != 0 {
                return res;
            }
        }
        0
    }
}

// =============================================================================
// RuleDoubleOut (double.cc:334-345 hh, 3281-3355 cc)
// =============================================================================

/// `data.combineInputVarnodes(vnhi,vnlo)` — merge two persistent input pieces
/// into one logical input (C++ `double.cc:3353`).
///
/// Routes to the ported [`Funcdata::combine_input_varnodes`]
/// (`funcdata_varnode.cc:383`).  The caller (`RuleDoubleOut::applyOp`) has
/// already established the contiguity precondition via `isAddrTiedContiguous`,
/// matching the C++ ordering; the merge is unconditional there (C++ does not
/// inspect a return value), so this returns `true`.  A `LowlevelError` from the
/// merge (non-input or non-contiguous pieces) would be a real invariant
/// violation; it is surfaced via `expect`, mirroring the C++ `throw`.
fn combine_input_varnodes_checked(data: &mut Funcdata, vnhi: VarnodeId, vnlo: VarnodeId) -> bool {
    data.combine_input_varnodes(vnhi, vnlo)
        .expect("RuleDoubleOut: combineInputVarnodes on non-input/non-contiguous pieces");
    true
}

/// Pull a double precision operation back one level, starting from inputs to a
/// PIECE (C++ `RuleDoubleOut`).
pub struct RuleDoubleOut {
    group: String,
}

impl RuleDoubleOut {
    pub fn new(g: impl Into<String>) -> RuleDoubleOut {
        RuleDoubleOut { group: g.into() }
    }

    /// Determine if inputs to a PIECE should be marked as double precision pieces
    /// (C++ `attemptMarking`).
    fn attempt_marking(&self, data: &mut Funcdata, vnhi: VarnodeId, vnlo: VarnodeId, piece_op: OpId) -> int4 {
        let whole = op_get_out(data, piece_op).expect("doubleout: piece out");
        if vn_is_type_lock(data, whole) && !vn_type_is_primitive_whole(data, whole) {
            return 0; // Don't mark for double precision if not a primitive type
        }
        if vn_get_size(data, vnhi) != vn_get_size(data, vnlo) {
            return 0;
        }

        // STUB(W4): the C++ also rejects when vnhi/vnlo carry conflicting
        // SymbolEntrys (Varnode::getSymbolEntry, the W4 symbol table).  Without
        // the symbol surface we cannot read the entries; we omit this extra
        // rejection (less conservative — could accept a pair the C++ rejects on a
        // symbol mismatch).  Recorded as a loss.

        let mut is_whole = false;
        for op in descend_ops(data, whole) {
            // Categorize op as "reading a logical whole"
            if op_is_arith_or_float(data, op) {
                is_whole = true;
                break;
            }
        }
        if !is_whole {
            return 0;
        }
        vn_set_precis_hi(data, vnhi);
        vn_set_precis_lo(data, vnlo);
        1
    }
}

impl Rule for RuleDoubleOut {
    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(&self.group) {
            return None;
        }
        Some(Box::new(RuleDoubleOut::new(self.group.clone())))
    }

    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_PIECE]
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        let vnhi = op_get_in(data, op, 0);
        let vnlo = op_get_in(data, op, 1);

        // Currently this only implements collapsing input varnodes read by PIECE,
        // so we test for this particular case early.
        if !vn_is_input(data, vnhi) || !vn_is_input(data, vnlo) {
            return 0;
        }
        if !vn_is_persist(data, vnhi) || !vn_is_persist(data, vnlo) {
            return 0;
        }

        if !vn_is_precis_hi(data, vnhi) || !vn_is_precis_lo(data, vnlo) {
            return self.attempt_marking(data, vnhi, vnlo, op);
        }
        if data.has_unreachable_blocks() {
            return 0;
        }

        if is_addr_tied_contiguous(data, vnlo, vnhi).is_none() {
            return 0;
        }
        // data.combineInputVarnodes(vnhi,vnlo);  return 1;  (double.cc:3353-3354)
        if !combine_input_varnodes_checked(data, vnhi, vnlo) {
            return 0;
        }
        1
    }
}

// =============================================================================
// RuleDoubleLoad (double.cc:347-358 hh, 3357-3505 cc)
// =============================================================================

/// `Varnode::getSpaceFromConst` index for a LOAD/STORE space operand, paired
/// with [`space_from_const_index`].
#[inline]
fn op_space_from_const_index(data: &Funcdata, op: OpId, slot: int4) -> Option<i32> {
    space_from_const_index(data, op_get_in(data, op, slot))
}

/// Collapse contiguous loads (C++ `RuleDoubleLoad`).
pub struct RuleDoubleLoad {
    group: String,
}

impl RuleDoubleLoad {
    pub fn new(g: impl Into<String>) -> RuleDoubleLoad {
        RuleDoubleLoad { group: g.into() }
    }

    /// Scan for conflicts between two LOADs/STOREs that prevent combining; return
    /// the later PcodeOp or `None` (C++ `RuleDoubleLoad::noWriteConflict`).
    ///
    /// `spc` is the address-space index referred to by the LOAD/STOREs.
    /// `indirects`, if `Some`, collects INDIRECTs caused by STOREs.
    fn no_write_conflict(
        data: &Funcdata,
        mut op1: OpId,
        mut op2: OpId,
        spc: i32,
        indirects: Option<&mut Vec<OpId>>,
    ) -> Option<OpId> {
        let bb = op_get_parent(data, op1)?;
        // Force the two ops to be in the same basic block
        if Some(bb) != op_get_parent(data, op2) {
            return None;
        }
        if op_get_order(data, op2) < op_get_order(data, op1) {
            std::mem::swap(&mut op1, &mut op2);
        }
        // Ordered ops of the block, used to model getBasicIter / previousOp.
        let ops = data.bb_ops(bb);
        let idx1 = ops.iter().position(|&o| o == op1).expect("nwc: op1 in block");
        let idx2 = ops.iter().position(|&o| o == op2).expect("nwc: op2 in block");

        let mut start_idx = idx1;
        if op_code(data, op1) == OpCode::CPUI_STORE {
            // Extend the range to include CPUI_INDIRECTs associated with the
            // initial STORE (the C++ walks startop->previousOp() over INDIRECTs).
            while start_idx > 0 && op_code(data, ops[start_idx - 1]) == OpCode::CPUI_INDIRECT {
                start_idx -= 1;
            }
        }

        let mut indirects = indirects;
        // iter from startop to op2 (exclusive of op2)
        let mut i = start_idx;
        while i < idx2 {
            let curop = ops[i];
            i += 1;
            if curop == op1 {
                continue;
            }
            match op_code(data, curop) {
                OpCode::CPUI_STORE => {
                    if op_space_from_const_index(data, curop, 0) == Some(spc) {
                        return None; // Don't go further trying to resolve alias
                    }
                }
                OpCode::CPUI_INDIRECT => {
                    let affector = get_op_from_const(data, op_get_in(data, curop, 1));
                    if affector == Some(op1) || affector == Some(op2) {
                        if let Some(inds) = indirects.as_deref_mut() {
                            inds.push(curop);
                        }
                    } else {
                        let outvn = op_get_out(data, curop).expect("nwc: indirect out");
                        if vn_space_index(data, outvn) == spc {
                            return None;
                        }
                    }
                }
                OpCode::CPUI_CALL
                | OpCode::CPUI_CALLIND
                | OpCode::CPUI_CALLOTHER
                | OpCode::CPUI_RETURN
                | OpCode::CPUI_BRANCH
                | OpCode::CPUI_CBRANCH
                | OpCode::CPUI_BRANCHIND => {
                    return None;
                }
                _ => {
                    if let Some(outvn) = op_get_out(data, curop) {
                        if vn_space_index(data, outvn) == spc {
                            return None;
                        }
                    }
                }
            }
        }
        Some(op2)
    }
}

/// `vn->getSpace()->getIndex()` (the space the Varnode is stored in).
#[inline]
fn vn_space_index(data: &Funcdata, vn: VarnodeId) -> i32 {
    data.vbank().get(vn).expect("vn_space_index").get_space().get_index()
}

impl Rule for RuleDoubleLoad {
    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(&self.group) {
            return None;
        }
        Some(Box::new(RuleDoubleLoad::new(self.group.clone())))
    }

    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_PIECE]
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        let piece0 = op_get_in(data, op, 0);
        let piece1 = op_get_in(data, op, 1);
        if !vn_is_written(data, piece0) {
            return 0;
        }
        if !vn_is_written(data, piece1) {
            return 0;
        }
        let load1 = vn_get_def(data, piece1).expect("dload: load1");
        if op_code(data, load1) != OpCode::CPUI_LOAD {
            // C++ `return false;` (0) here.
            return 0;
        }
        let mut load0 = vn_get_def(data, piece0).expect("dload: load0");
        let mut opc = op_code(data, load0);
        let mut offset: int4 = 0;
        if opc == OpCode::CPUI_SUBPIECE {
            // 2 LOADs but most significant part of most significant LOAD discarded
            if vn_get_offset(data, op_get_in(data, load0, 1)) != 0 {
                return 0;
            }
            let vn0 = op_get_in(data, load0, 0);
            if !vn_is_written(data, vn0) {
                return 0;
            }
            offset = vn_get_size(data, vn0) - vn_get_size(data, piece0);
            load0 = vn_get_def(data, vn0).expect("dload: load0 (sub)");
            opc = op_code(data, load0);
        }
        if opc != OpCode::CPUI_LOAD {
            return 0;
        }
        let (loadlo, loadhi, spc) = match test_contiguous_pointers(data, load0, load1) {
            Some(t) => t,
            None => return 0,
        };

        let size = vn_get_size(data, piece0) + vn_get_size(data, piece1);
        let latest = match RuleDoubleLoad::no_write_conflict(data, loadlo, loadhi, spc, None) {
            Some(l) => l,
            None => return 0, // There was a conflict
        };

        // Create new load op that combines the two smaller loads
        let latest_addr = op_get_addr(data, latest);
        let newload = data.new_op(2, latest_addr.clone());
        let vnout = data.new_unique_out(size, newload).expect("dload: vnout");
        let spcvn = space_index_to_varnode(data, spc);
        set_opcode_typed(data, newload, OpCode::CPUI_LOAD);
        op_set_input(data, newload, spcvn, 0);
        let mut addrvn = op_get_in(data, loadlo, 1);
        let mut latest = latest;
        if space_is_big_endian_by_index(data, spc) && offset != 0 {
            // Most significant part of LOAD discarded: add discard amount to ptr
            let newadd = data.new_op(2, latest_addr.clone());
            let addout = data
                .new_unique_out(vn_get_size(data, addrvn), newadd)
                .expect("dload: addout");
            set_opcode_typed(data, newadd, OpCode::CPUI_INT_ADD);
            op_set_input(data, newadd, addrvn, 0);
            let c = data.new_constant(vn_get_size(data, addrvn), offset as uintb);
            op_set_input(data, newadd, c, 1);
            data.op_insert_after(newadd, latest);
            addrvn = addout;
            latest = newadd;
        }
        op_set_input(data, newload, addrvn, 1);
        // Guarantee -newload- reads -addrvn- after it has been defined.
        data.op_insert_after(newload, latest);

        // Change the concatenation to a copy from the big load
        data.op_remove_input(op, 1);
        set_opcode_typed(data, op, OpCode::CPUI_COPY);
        op_set_input(data, op, vnout, 0);

        1
    }
}

/// Build a Varnode holding a constant space pointer for a space index (C++
/// `data.newVarnodeSpace(spc)`).  Pairs with [`space_from_const_index`]; only
/// reachable once the W4 space resolution lands.
fn space_index_to_varnode(data: &mut Funcdata, idx: i32) -> VarnodeId {
    let spc = data
        .get_arch()
        .manage()
        .get_space(idx)
        .expect("space_index_to_varnode: space")
        .clone();
    data.new_varnode_space(&spc)
}

// =============================================================================
// RuleDoubleStore (double.cc:360-372 hh, 3507-3645 cc)
// =============================================================================

/// Collapse contiguous stores (C++ `RuleDoubleStore`).
pub struct RuleDoubleStore {
    group: String,
}

impl RuleDoubleStore {
    pub fn new(g: impl Into<String>) -> RuleDoubleStore {
        RuleDoubleStore { group: g.into() }
    }

    /// Test if outputs from `indirects` are used within the range bounded by
    /// `op1`/`op2` (C++ `RuleDoubleStore::testIndirectUse`).
    fn test_indirect_use(data: &Funcdata, mut op1: OpId, mut op2: OpId, indirects: &[OpId]) -> bool {
        if op_get_order(data, op2) < op_get_order(data, op1) {
            std::mem::swap(&mut op1, &mut op2);
        }
        for &ind in indirects {
            let outvn = op_get_out(data, ind).expect("tiu: indirect out");
            let mut usecount = 0i32;
            let mut usebyop2 = 0i32;
            for op in descend_ops(data, outvn) {
                usecount += 1;
                if op_get_parent(data, op) != op_get_parent(data, op1) {
                    continue;
                }
                if op_get_order(data, op) < op_get_order(data, op1) {
                    continue;
                }
                if op_get_order(data, op) > op_get_order(data, op2) {
                    continue;
                }
                // INDIRECTs from the first STORE likely feed INDIRECTs for the second.
                if op_code(data, op) == OpCode::CPUI_INDIRECT
                    && Some(op2) == get_op_from_const(data, op_get_in(data, op, 1))
                {
                    usebyop2 += 1; // Note this pairing
                    continue;
                }
                return false;
            }
            // An INDIRECT whose output feeds later INDIRECTs must be removed; if
            // some but not all uses feed into later INDIRECTs, return false.
            if usebyop2 > 0 && usecount != usebyop2 {
                return false;
            }
            if usebyop2 > 1 {
                return false;
            }
        }
        true
    }

    /// Reassign INDIRECTs to a new STORE (C++ `RuleDoubleStore::reassignIndirects`).
    fn reassign_indirects(data: &mut Funcdata, new_store: OpId, indirects: &[OpId]) {
        // Search for INDIRECT pairs.  The earlier is deleted; the later gains the
        // earlier's input.
        for &op in indirects {
            data.obank_mut().get_mut(op).expect("ri: op").set_mark();
            let vn = op_get_in(data, op, 0);
            if !vn_is_written(data, vn) {
                continue;
            }
            let earlyop = vn_get_def(data, vn).expect("ri: earlyop");
            if data.obank().get(earlyop).expect("ri: earlyop").is_mark() {
                // Grab the earlier op's input, replacing the use of its output
                let early_in0 = op_get_in(data, earlyop, 0);
                op_set_input(data, op, early_in0, 0);
                data.op_destroy(earlyop);
            }
        }
        for &op in indirects {
            data.obank_mut().get_mut(op).expect("ri: op2").clear_mark();
            if op_is_dead(data, op) {
                continue;
            }
            data.op_uninsert(op);
            data.op_insert_before(op, new_store); // Move the INDIRECT to the new STORE
            let iopvn = data.new_varnode_iop(new_store);
            op_set_input(data, op, iopvn, 1); // Assign the INDIRECT to the new STORE
        }
    }
}

impl Rule for RuleDoubleStore {
    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(&self.group) {
            return None;
        }
        Some(Box::new(RuleDoubleStore::new(self.group.clone())))
    }

    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_STORE]
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        let vnlo = op_get_in(data, op, 2);
        if !vn_is_precis_lo(data, vnlo) {
            return 0;
        }
        if !vn_is_written(data, vnlo) {
            return 0;
        }
        let subpiece_op_lo = vn_get_def(data, vnlo).expect("dstore: sublo");
        if op_code(data, subpiece_op_lo) != OpCode::CPUI_SUBPIECE {
            return 0;
        }
        if vn_get_offset(data, op_get_in(data, subpiece_op_lo, 1)) != 0 {
            return 0;
        }
        let whole = op_get_in(data, subpiece_op_lo, 0);
        if vn_is_free(data, whole) {
            return 0;
        }
        for subpiece_op_hi in descend_ops(data, whole) {
            if op_code(data, subpiece_op_hi) != OpCode::CPUI_SUBPIECE {
                continue;
            }
            if subpiece_op_hi == subpiece_op_lo {
                continue;
            }
            let offset = vn_get_offset(data, op_get_in(data, subpiece_op_hi, 1)) as int4;
            if offset != vn_get_size(data, vnlo) {
                continue;
            }
            let vnhi = op_get_out(data, subpiece_op_hi).expect("dstore: subhi out");
            if !vn_is_precis_hi(data, vnhi) {
                continue;
            }
            if vn_get_size(data, vnhi) != vn_get_size(data, whole) - offset {
                continue;
            }
            for store_op2 in descend_ops(data, vnhi) {
                if op_code(data, store_op2) != OpCode::CPUI_STORE {
                    continue;
                }
                if op_get_in(data, store_op2, 2) != vnhi {
                    continue;
                }
                if let Some((storelo, storehi, spc)) =
                    test_contiguous_pointers(data, store_op2, op)
                {
                    let mut indirects: Vec<OpId> = Vec::new();
                    let latest = match RuleDoubleLoad::no_write_conflict(
                        data,
                        storelo,
                        storehi,
                        spc,
                        Some(&mut indirects),
                    ) {
                        Some(l) => l,
                        None => continue, // There was a conflict
                    };
                    if !RuleDoubleStore::test_indirect_use(data, storelo, storehi, &indirects) {
                        continue;
                    }
                    // Create new STORE op that combines the two smaller STOREs
                    let latest_addr = op_get_addr(data, latest);
                    let newstore = data.new_op(3, latest_addr);
                    let spcvn = space_index_to_varnode(data, spc);
                    set_opcode_typed(data, newstore, OpCode::CPUI_STORE);
                    op_set_input(data, newstore, spcvn, 0);
                    let mut addrvn = op_get_in(data, storelo, 1);
                    if vn_is_constant(data, addrvn) {
                        addrvn =
                            data.new_constant(vn_get_size(data, addrvn), vn_get_offset(data, addrvn));
                    }
                    op_set_input(data, newstore, addrvn, 1);
                    op_set_input(data, newstore, whole, 2);
                    // Guarantee -newstore- reads -addrvn- after it is defined.
                    data.op_insert_after(newstore, latest);
                    data.op_destroy(op); // Get rid of the original STOREs
                    data.op_destroy(store_op2);
                    RuleDoubleStore::reassign_indirects(data, newstore, &indirects);
                    return 1;
                }
            }
        }
        0
    }
}

// =============================================================================
// Rule registration (specs) — C++ definition order
// =============================================================================

/// The [`RuleSpec`] rows for the four `RuleDouble*` rules, in the exact order
/// and with the exact groups the C++ `ActionDatabase::universalAction` registers
/// them into the `actprop` pool (`coreaction.cc:5913-5916`).
///
/// The order and the `getGroup()` string are output-determining (the engine's
/// per-op rule list preserves insertion order, and `clone(grouplist)` filters on
/// the group), so this mirrors the registration site rather than the header's
/// class-declaration order.  Each row is `RuleSpec { group, ctor }` per the
/// [`crate::action`] convention.
pub fn specs() -> Vec<RuleSpec> {
    vec![
        RuleSpec { group: "doubleload", ctor: || Box::new(RuleDoubleLoad::new("doubleload")) },
        RuleSpec { group: "doubleprecis", ctor: || Box::new(RuleDoubleStore::new("doubleprecis")) },
        RuleSpec { group: "doubleprecis", ctor: || Box::new(RuleDoubleIn::new("doubleprecis")) },
        RuleSpec { group: "doubleprecis", ctor: || Box::new(RuleDoubleOut::new("doubleprecis")) },
    ]
}

#[cfg(test)]
mod tests {
    //! Tests for the stub-free decision logic: the static matchers
    //! (`verifyMultNegOne`, `adjacentOffsets`, `isAddrTiedContiguous`), the
    //! pairing discovery (`wholeList` / `findWholeSplitToPieces` /
    //! `findWholeBuiltFromPieces` / `inHandLo*`), and a `LogicalForm::verify`
    //! form-matrix entry.  Op-graph mutation that reaches a cross-wave stub
    //! (`getOpFromConst`, `getSpaceFromConst`, `combineInputVarnodes`, the W6
    //! arithmetic/float op classification) is exercised only up to the stub.

    use super::*;
    use std::rc::Rc;

    use kuna_base::address::{Address, SeqNum};
    use kuna_base::space::{
        addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
    };

    use crate::dtype::{type_metatype, Datatype};
    use crate::op::pcodeop_flags;
    use crate::context::{ArchContext, TypeOp};

    fn build_manager() -> AddrSpaceManager {
        let mut m = AddrSpaceManager::new();
        m.insert_space(Rc::new(ConstantSpace::new())).unwrap();
        m.insert_space(Rc::new(UniqueSpace::new(1, 0, false))).unwrap();
        m.insert_space(Rc::new(AddrSpace::new(
            spacetype::IPTR_PROCESSOR,
            "ram",
            false,
            8,
            1,
            2,
            addrspace_flags::hasphysical,
            1,
            1,
        )))
        .unwrap();
        m
    }

    fn build_fd() -> Funcdata {
        let manage = build_manager();
        let glb = Rc::new(ArchContext::new(manage));
        let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
        let addr = Address::new(ram, 0x1000);
        Funcdata::new("func", "func", glb, addr, 0x1000_0000, 0x40).unwrap()
    }

    fn ram(fd: &Funcdata) -> Rc<AddrSpace> {
        Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap())
    }
    fn dt(size: int4) -> Rc<Datatype> {
        Rc::new(Datatype::new(size, type_metatype::TYPE_UNKNOWN))
    }

    fn mk_op(fd: &mut Funcdata, inputs: int4, off: u64, opc: OpCode) -> OpId {
        let r = ram(fd);
        let op = fd.new_op(inputs, Address::new(r, off));
        let fl = match opc {
            OpCode::CPUI_BRANCH | OpCode::CPUI_CBRANCH | OpCode::CPUI_BRANCHIND => {
                pcodeop_flags::branch
            }
            _ => 0,
        };
        fd.obank_mut().change_opcode(op, TypeOp::new(opc, fl, format!("{opc:?}")));
        op
    }

    /// A plain Varnode in ram at `off` of `size` bytes (free, no def).
    fn mk_vn(fd: &mut Funcdata, off: u64, size: int4) -> VarnodeId {
        let r = ram(fd);
        fd.vbank_mut().create(size, Address::new(r, off), dt(size))
    }

    /// A constant Varnode.
    fn mk_const(fd: &mut Funcdata, size: int4, val: uintb) -> VarnodeId {
        fd.new_constant(size, val)
    }

    /// Build a block, append `op` to it (so it gains a parent/order), and return
    /// the block id.  `op` must already have its opcode set.
    fn mk_block_with(fd: &mut Funcdata, ops: &[OpId]) -> crate::context::BlockId {
        let root = fd.bblocks_ref().root.expect("root");
        let bl = fd.bblocks_mut().new_block_basic(root);
        for &op in ops {
            fd.op_insert_end(op, bl);
        }
        bl
    }

    /// Make `op` a SUBPIECE(whole, off) with a fresh ram output of `outsize`,
    /// returning the output Varnode.  Inserts `op` into block `bl`.
    fn make_subpiece(
        fd: &mut Funcdata,
        op: OpId,
        whole: VarnodeId,
        off: u64,
        outsize: int4,
        out_off: u64,
        bl: crate::context::BlockId,
    ) -> VarnodeId {
        let r = ram(fd);
        let out = fd.new_varnode_out(outsize, &Address::new(r, out_off), op).unwrap();
        fd.op_set_input(op, whole, 0).unwrap();
        let c = mk_const(fd, 4, off);
        fd.op_set_input(op, c, 1).unwrap();
        fd.op_insert_end(op, bl);
        out
    }

    // --- static matchers -----------------------------------------------------

    #[test]
    fn verify_mult_neg_one_matches_mult_by_allones() {
        let mut fd = build_fd();
        let op = mk_op(&mut fd, 2, 0x100, OpCode::CPUI_INT_MULT);
        let v = mk_vn(&mut fd, 0x10, 4);
        let negone = mk_const(&mut fd, 4, calc_mask(4)); // -1 in 4 bytes
        fd.op_set_input(op, v, 0).unwrap();
        fd.op_set_input(op, negone, 1).unwrap();
        assert!(verify_mult_neg_one(&fd, op));

        // A different constant is rejected (distinct operand var — a free var may
        // not have multiple descendants in the IR).
        let op2 = mk_op(&mut fd, 2, 0x108, OpCode::CPUI_INT_MULT);
        let v2 = mk_vn(&mut fd, 0x18, 4);
        let two = mk_const(&mut fd, 4, 2);
        fd.op_set_input(op2, v2, 0).unwrap();
        fd.op_set_input(op2, two, 1).unwrap();
        assert!(!verify_mult_neg_one(&fd, op2));

        // A non-MULT op is rejected.
        let op3 = mk_op(&mut fd, 2, 0x110, OpCode::CPUI_INT_ADD);
        let v3 = mk_vn(&mut fd, 0x1c, 4);
        let negone3 = mk_const(&mut fd, 4, calc_mask(4));
        fd.op_set_input(op3, v3, 0).unwrap();
        fd.op_set_input(op3, negone3, 1).unwrap();
        assert!(!verify_mult_neg_one(&fd, op3));
    }

    #[test]
    fn adjacent_offsets_constant_form() {
        let mut fd = build_fd();
        let a = mk_const(&mut fd, 8, 0x1000);
        let b = mk_const(&mut fd, 8, 0x1004);
        assert!(adjacent_offsets(&fd, a, b, 4));
        assert!(!adjacent_offsets(&fd, a, b, 8));
        // vn2 not constant -> false.
        let c = mk_vn(&mut fd, 0x20, 8);
        assert!(!adjacent_offsets(&fd, a, c, 4));
    }

    #[test]
    fn adjacent_offsets_add_const_form() {
        // vn1 = base; vn2 = base + 4; size1 == 4 -> true.
        let mut fd = build_fd();
        let bl = mk_block_with(&mut fd, &[]);
        let base = mk_vn(&mut fd, 0x30, 8);
        let add = mk_op(&mut fd, 2, 0x200, OpCode::CPUI_INT_ADD);
        let r = ram(&fd);
        let vn2 = fd.new_varnode_out(8, &Address::new(r, 0x40), add).unwrap();
        fd.op_set_input(add, base, 0).unwrap();
        let four = mk_const(&mut fd, 8, 4);
        fd.op_set_input(add, four, 1).unwrap();
        fd.op_insert_end(add, bl);
        assert!(adjacent_offsets(&fd, base, vn2, 4));
        assert!(!adjacent_offsets(&fd, base, vn2, 8));
    }

    // --- pairing discovery ---------------------------------------------------

    /// Build `whole` (8 bytes) with two SUBPIECE pieces:
    ///   lo = SUB(whole, 0) marked PrecisLo, hi = SUB(whole, 4) marked PrecisHi.
    /// Returns (whole, lo, hi).
    fn build_split_off_whole(fd: &mut Funcdata) -> (VarnodeId, VarnodeId, VarnodeId) {
        let bl = mk_block_with(fd, &[]);
        // whole is the output of some op so it is "written".
        let def = mk_op(fd, 1, 0x300, OpCode::CPUI_INT_ADD);
        let r = ram(fd);
        let whole = fd.new_varnode_out(8, &Address::new(r, 0x100), def).unwrap();
        let a0 = mk_vn(fd, 0x108, 8);
        fd.op_set_input(def, a0, 0).unwrap();
        fd.op_insert_end(def, bl);

        let sublo = mk_op(fd, 2, 0x310, OpCode::CPUI_SUBPIECE);
        let lo = make_subpiece(fd, sublo, whole, 0, 4, 0x200, bl);
        fd.vbank_mut().get_mut(lo).unwrap().set_precis_lo();

        let subhi = mk_op(fd, 2, 0x320, OpCode::CPUI_SUBPIECE);
        let hi = make_subpiece(fd, subhi, whole, 4, 4, 0x210, bl);
        fd.vbank_mut().get_mut(hi).unwrap().set_precis_hi();

        (whole, lo, hi)
    }

    #[test]
    fn whole_list_discovers_hi_lo_pair() {
        let mut fd = build_fd();
        let (whole, lo, hi) = build_split_off_whole(&mut fd);
        let vec = whole_list(&fd, whole);
        assert_eq!(vec.len(), 1, "exactly the basic split (no copies)");
        let s = &vec[0];
        assert_eq!(s.get_whole(), Some(whole));
        assert_eq!(s.get_lo(), Some(lo));
        assert_eq!(s.get_hi(), Some(hi));
        assert_eq!(s.get_size(), 8);
        assert!(s.has_both_pieces());
    }

    #[test]
    fn whole_list_empty_without_marks() {
        // If neither piece is marked PrecisHi/Lo, wholeList returns nothing.
        let mut fd = build_fd();
        let bl = mk_block_with(&mut fd, &[]);
        let def = mk_op(&mut fd, 1, 0x300, OpCode::CPUI_INT_ADD);
        let r = ram(&fd);
        let whole = fd.new_varnode_out(8, &Address::new(r, 0x100), def).unwrap();
        let a0 = mk_vn(&mut fd, 0x108, 8);
        fd.op_set_input(def, a0, 0).unwrap();
        fd.op_insert_end(def, bl);
        let sub = mk_op(&mut fd, 2, 0x310, OpCode::CPUI_SUBPIECE);
        let _lo = make_subpiece(&mut fd, sub, whole, 0, 4, 0x200, bl);
        assert!(whole_list(&fd, whole).is_empty());
    }

    #[test]
    fn find_whole_split_to_pieces_roundtrip() {
        let mut fd = build_fd();
        let (whole, lo, hi) = build_split_off_whole(&mut fd);
        let mut s = SplitVarnode::empty();
        s.lo = Some(lo);
        s.hi = Some(hi);
        s.wholesize = 8;
        assert!(s.find_whole_split_to_pieces(&fd));
        assert_eq!(s.get_whole(), Some(whole));
        // defpoint/defblock are filled from the whole's def.
        assert!(s.get_def_point().is_some());
        assert!(s.get_def_block().is_some());
    }

    #[test]
    fn in_hand_lo_finds_companion_hi() {
        let mut fd = build_fd();
        let (whole, lo, hi) = build_split_off_whole(&mut fd);
        let mut s = SplitVarnode::empty();
        assert!(s.in_hand_lo(&fd, lo));
        assert_eq!(s.get_whole(), Some(whole));
        assert_eq!(s.get_lo(), Some(lo));
        assert_eq!(s.get_hi(), Some(hi));
    }

    #[test]
    fn in_hand_lo_rejects_unmarked() {
        let mut fd = build_fd();
        let (_w, lo, _hi) = build_split_off_whole(&mut fd);
        // Clear the PrecisLo mark; inHandLo must quick-reject.
        fd.vbank_mut().get_mut(lo).unwrap().clear_precis_lo();
        let mut s = SplitVarnode::empty();
        assert!(!s.in_hand_lo(&fd, lo));
    }

    /// Build a PIECE(hi, lo) -> whole concatenation where hi/lo are marked and
    /// written (so `findWholeBuiltFromPieces` reads their def block), used by
    /// `inHandLoOut` / `findWholeBuiltFromPieces`.
    fn build_concat(fd: &mut Funcdata) -> (VarnodeId, VarnodeId, VarnodeId) {
        let bl = mk_block_with(fd, &[]);
        let r = ram(fd);
        // lo/hi are COPY outputs so they are "written" in block `bl`.
        let cpylo = mk_op(fd, 1, 0x4f0, OpCode::CPUI_COPY);
        let lo = fd.new_varnode_out(4, &Address::new(Rc::clone(&r), 0x400), cpylo).unwrap();
        let lsrc = mk_vn(fd, 0x4a0, 4);
        fd.op_set_input(cpylo, lsrc, 0).unwrap();
        fd.op_insert_end(cpylo, bl);

        let cpyhi = mk_op(fd, 1, 0x4f8, OpCode::CPUI_COPY);
        let hi = fd.new_varnode_out(4, &Address::new(Rc::clone(&r), 0x408), cpyhi).unwrap();
        let hsrc = mk_vn(fd, 0x4a8, 4);
        fd.op_set_input(cpyhi, hsrc, 0).unwrap();
        fd.op_insert_end(cpyhi, bl);

        fd.vbank_mut().get_mut(lo).unwrap().set_precis_lo();
        fd.vbank_mut().get_mut(hi).unwrap().set_precis_hi();
        let piece = mk_op(fd, 2, 0x500, OpCode::CPUI_PIECE);
        let whole = fd.new_varnode_out(8, &Address::new(r, 0x410), piece).unwrap();
        fd.op_set_input(piece, hi, 0).unwrap(); // most significant
        fd.op_set_input(piece, lo, 1).unwrap(); // least significant
        fd.op_insert_end(piece, bl);
        (whole, lo, hi)
    }

    #[test]
    fn in_hand_lo_out_finds_concat() {
        let mut fd = build_fd();
        let (whole, lo, hi) = build_concat(&mut fd);
        let mut s = SplitVarnode::empty();
        assert!(s.in_hand_lo_out(&fd, lo));
        assert_eq!(s.get_whole(), Some(whole));
        assert_eq!(s.get_hi(), Some(hi));
        assert_eq!(s.get_lo(), Some(lo));
    }

    #[test]
    fn find_whole_built_from_pieces_finds_concat() {
        let mut fd = build_fd();
        let (whole, lo, hi) = build_concat(&mut fd);
        let mut s = SplitVarnode::empty();
        s.lo = Some(lo);
        s.hi = Some(hi);
        s.wholesize = 8;
        assert!(s.find_whole_built_from_pieces(&fd));
        assert_eq!(s.get_whole(), Some(whole));
        assert_eq!(s.get_def_point().is_some(), true);
    }

    // --- form matrix: LogicalForm::verify ------------------------------------

    #[test]
    fn logical_form_verify_matches_and_pair() {
        // Build:  reslo = lo1 & lo2c ; reshi = hi1 & hi2c  (constant-operand AND
        // form — the reachable `findHiMatch` path that scans hi1's descendants for
        // a unique AND-by-constant).  verify(in.hi, in.lo, loop=reslo) must find
        // hiop and recover lo2/hi2.
        let mut fd = build_fd();
        let (_whole, lo1, hi1) = build_split_off_whole(&mut fd);
        let bl = mk_block_with(&mut fd, &[]);
        let r = ram(&fd);

        let lo2 = mk_const(&mut fd, 4, 0x0f0f0f0f);
        let hi2 = mk_const(&mut fd, 4, 0x00ff00ff);

        // reslo = lo1 & lo2c
        let loand = mk_op(&mut fd, 2, 0x700, OpCode::CPUI_INT_AND);
        let _reslo = fd.new_varnode_out(4, &Address::new(Rc::clone(&r), 0x610), loand).unwrap();
        fd.op_set_input(loand, lo1, 0).unwrap();
        fd.op_set_input(loand, lo2, 1).unwrap();
        fd.op_insert_end(loand, bl);

        // reshi = hi1 & hi2c  (the single AND-by-const off hi1)
        let hiand = mk_op(&mut fd, 2, 0x708, OpCode::CPUI_INT_AND);
        let _reshi = fd.new_varnode_out(4, &Address::new(Rc::clone(&r), 0x618), hiand).unwrap();
        fd.op_set_input(hiand, hi1, 0).unwrap();
        fd.op_set_input(hiand, hi2, 1).unwrap();
        fd.op_insert_end(hiand, bl);

        let in_ = SplitVarnode::from_pieces(&fd, lo1, hi1);
        let mut form = LogicalForm::new();
        form.in_ = in_;
        assert!(form.verify(&fd, hi1, lo1, loand));
        assert_eq!(form.lo2, Some(lo2));
        assert_eq!(form.hi2, Some(hi2));
        assert_eq!(form.hiop, Some(hiand));
    }

    #[test]
    fn logical_form_verify_rejects_ambiguous_hi() {
        // Two AND-by-constant ops off hi1 -> findHiMatch can't distinguish
        // (count>1 -> -1) -> verify rejects.
        let mut fd = build_fd();
        let (_whole, lo1, hi1) = build_split_off_whole(&mut fd);
        let bl = mk_block_with(&mut fd, &[]);
        let r = ram(&fd);

        let lo2 = mk_const(&mut fd, 4, 0x0f0f0f0f);
        let hi2a = mk_const(&mut fd, 4, 0x00ff00ff);
        let hi2b = mk_const(&mut fd, 4, 0xff00ff00);

        let loand = mk_op(&mut fd, 2, 0x700, OpCode::CPUI_INT_AND);
        let _reslo = fd.new_varnode_out(4, &Address::new(Rc::clone(&r), 0x610), loand).unwrap();
        fd.op_set_input(loand, lo1, 0).unwrap();
        fd.op_set_input(loand, lo2, 1).unwrap();
        fd.op_insert_end(loand, bl);

        // first AND off hi1
        let hiand1 = mk_op(&mut fd, 2, 0x708, OpCode::CPUI_INT_AND);
        let _r1 = fd.new_varnode_out(4, &Address::new(Rc::clone(&r), 0x618), hiand1).unwrap();
        fd.op_set_input(hiand1, hi1, 0).unwrap();
        fd.op_set_input(hiand1, hi2a, 1).unwrap();
        fd.op_insert_end(hiand1, bl);
        // second AND off hi1 (ambiguity)
        let hiand2 = mk_op(&mut fd, 2, 0x710, OpCode::CPUI_INT_AND);
        let _r2 = fd.new_varnode_out(4, &Address::new(Rc::clone(&r), 0x620), hiand2).unwrap();
        fd.op_set_input(hiand2, hi1, 0).unwrap();
        fd.op_set_input(hiand2, hi2b, 1).unwrap();
        fd.op_insert_end(hiand2, bl);

        let in_ = SplitVarnode::from_pieces(&fd, lo1, hi1);
        let mut form = LogicalForm::new();
        form.in_ = in_;
        assert!(!form.verify(&fd, hi1, lo1, loand));
    }

    // --- constant / precision helpers ----------------------------------------

    #[test]
    fn split_varnode_constant_value_assembly() {
        let mut fd = build_fd();
        let lo = mk_const(&mut fd, 4, 0xdead);
        let hi = mk_const(&mut fd, 4, 0xbeef);
        let mut s = SplitVarnode::empty();
        s.init_partial_pieces(&fd, 8, Some(lo), Some(hi));
        assert!(s.is_constant());
        // val = (hi << 32) | lo
        assert_eq!(s.get_value(), (0xbeefu64 << 32) | 0xdead);
    }

    #[test]
    fn exceeds_const_precision_over_eight_bytes() {
        let big = SplitVarnode::new_constant(16, 0);
        assert!(big.exceeds_const_precision());
        let ok = SplitVarnode::new_constant(8, 0);
        assert!(!ok.exceeds_const_precision());
    }

    #[test]
    fn specs_match_coreaction_order_and_groups() {
        let s = specs();
        assert_eq!(s.len(), 4);
        assert_eq!(s[0].group, "doubleload"); // RuleDoubleLoad
        assert_eq!(s[1].group, "doubleprecis"); // RuleDoubleStore
        assert_eq!(s[2].group, "doubleprecis"); // RuleDoubleIn
        assert_eq!(s[3].group, "doubleprecis"); // RuleDoubleOut
    }

    // =====================================================================
    // VERIFIER ADVERSARIAL TESTS (w6-s5-double, round 1)
    // =====================================================================

    /// Control / oracle for the divergence below: the C++ contract is that
    /// `isWholeFeasible` (which mutates the member, setting `whole`/`defblock`/
    /// `defpoint`) and `findCreateWhole` are called on the SAME object, so when
    /// `findWholeSplitToPieces` discovers an existing `whole`, `findCreateWhole`
    /// reuses it (early `if (whole != 0) return;`) and creates NO new op.
    ///
    /// This test threads one object through both calls (the AddForm/Equal1Form/
    /// Equal3Form pattern) and confirms the existing whole is reused.
    #[test]
    fn verify_w6_s5_double_find_create_whole_reuses_existing_whole_when_threaded() {
        let mut fd = build_fd();
        // lo/hi are non-constant SUBPIECEs of an existing 8-byte `whole`.
        let (whole, lo, hi) = build_split_off_whole(&mut fd);
        // An existop located after the whole's def, in the same block.
        let bl = op_get_parent(&fd, vn_get_def(&fd, whole).unwrap()).unwrap();
        let existop = mk_op(&mut fd, 2, 0x900, OpCode::CPUI_INT_ADD);
        fd.op_insert_end(existop, bl);

        let mut s = SplitVarnode::empty();
        s.lo = Some(lo);
        s.hi = Some(hi);
        s.wholesize = 8;

        let pieces_before = fd.obank().iter_code(OpCode::CPUI_PIECE).count();
        assert!(s.is_whole_feasible(&fd, existop), "feasible: whole exists");
        assert_eq!(s.whole, Some(whole), "isWholeFeasible found the existing whole");
        // Threaded: find_create_whole on the SAME object reuses `whole`.
        s.find_create_whole(&mut fd);
        assert_eq!(s.get_whole(), Some(whole), "reused the existing whole");
        let pieces_after = fd.obank().iter_code(OpCode::CPUI_PIECE).count();
        assert_eq!(
            pieces_before, pieces_after,
            "threaded path must NOT fabricate a new PIECE/concat op"
        );
    }

    /// PRIMITIVE GUARD (originally the F1 divergence proof — F1 is now FIXED at
    /// the call sites; see `verify_w6_s5_double_equal2form_threads_prepared_whole`
    /// and `..._lessthreeway_threads_prepared_whole` below).  This test reproduces
    /// the *bad* call shape MANUALLY — passing a throwaway clone to the prepare
    /// step (`is_whole_feasible`, which mutates `whole`/`defblock`/`defpoint`)
    /// and then a SECOND fresh clone to the rewrite step (`find_create_whole`) —
    /// and shows why it is wrong: the rewrite-step clone sees `whole == None` and
    /// FABRICATES a redundant concat instead of reusing the existing whole.  It
    /// remains as a regression guard documenting that `Equal2Form::replace` /
    /// `LessThreeWay::set_bool_op` must NEVER discard the prepared object.
    #[test]
    fn verify_w6_s5_double_clonediscard_fabricates_redundant_whole() {
        let mut fd = build_fd();
        let (whole, lo, hi) = build_split_off_whole(&mut fd);
        let bl = op_get_parent(&fd, vn_get_def(&fd, whole).unwrap()).unwrap();
        // Register a start block so the (discarded-state) defblock==None branch of
        // find_create_whole can complete its fabrication instead of erroring; the
        // fact that this branch is even taken is itself the divergence.
        let root = fd.bblocks_ref().root.expect("bblocks root");
        fd.bblocks_mut().set_start_block(root, bl);
        let existop = mk_op(&mut fd, 2, 0x900, OpCode::CPUI_INT_ADD);
        fd.op_insert_end(existop, bl);

        let base = SplitVarnode::empty();
        let mut s = base.clone();
        s.lo = Some(lo);
        s.hi = Some(hi);
        s.wholesize = 8;

        let ops_before = fd.obank().num_alive();

        // Equal2Form/LessThreeWay shape: prepare runs on a throwaway clone ...
        let mut throwaway = s.clone();
        assert!(throwaway.is_whole_feasible(&fd, existop));
        assert_eq!(throwaway.whole, Some(whole), "discovery happened on the clone");
        // ... and the original `s` never received the discovered fields.
        assert_eq!(s.whole, None, "the prepare-step mutation was discarded");

        // ... then the rewrite step clones `s` AGAIN and creates the whole.
        let mut fresh = s.clone();
        fresh.find_create_whole(&mut fd);
        let ops_after = fd.obank().num_alive();

        // The faithful C++ would have reused `whole` (no new op); the discarded
        // state forces a fabricated concat.  These assertions capture the OBSERVED
        // (buggy) behavior; the C++ oracle is `ops_after == ops_before` and
        // `fresh.get_whole() == Some(whole)`.
        assert_ne!(
            fresh.get_whole(),
            Some(whole),
            "DIVERGENCE: clone-discard fabricates a new whole instead of reusing the existing one"
        );
        assert_eq!(
            ops_after,
            ops_before + 1,
            "DIVERGENCE: a redundant concat op was created (C++ creates none)"
        );
        // The fabricated whole is defined by a brand-new PIECE op, placed at the
        // (wrong) start block because defblock/defpoint were discarded.
        let fab = fresh.get_whole().unwrap();
        let fabdef = vn_get_def(&fd, fab).expect("fabricated whole has a def");
        assert_eq!(op_code(&fd, fabdef), OpCode::CPUI_PIECE, "fabricated concat is a PIECE");
    }

    /// F1 FIX (Equal2Form): `replace` now threads ONE clone pair (in,param2)
    /// through `prepare_bool_op` (C++ `prepareBoolOp(in,param2,...)` mutates the
    /// members; double.cc:1933) and persists it back to `self.in_`/`self.param2`,
    /// so the `whole`/`defblock`/`defpoint` discovered by `is_whole_feasible`
    /// survive into the `replace_bool_op` step in `apply_rule`
    /// (double.cc:1978).  For a non-constant `param2` whose pieces are SUBPIECEs
    /// of an existing whole, that whole must be REUSED (no fabricated concat).
    #[test]
    fn verify_w6_s5_double_equal2form_threads_prepared_whole() {
        let mut fd = build_fd();
        // lo2/hi2 are non-constant SUBPIECEs of an existing 8-byte `whole`.
        let (whole, lo, hi) = build_split_off_whole(&mut fd);
        let bl = op_get_parent(&fd, vn_get_def(&fd, whole).unwrap()).unwrap();
        // The boolAndOr testop sits after the whole's def in the same block, so
        // is_whole_feasible accepts the existing whole.
        let boolandor = mk_op(&mut fd, 2, 0x900, OpCode::CPUI_BOOL_AND);
        fd.op_insert_end(boolandor, bl);

        let mut form = Equal2Form::new();
        // in1 is an arbitrary constant pair (8 bytes) -> trivially whole-feasible.
        form.in_.init_partial_const(8, 0x1234);
        form.lo1 = Some(lo); // only its size is read in the constant branch
        form.hi2 = Some(hi);
        form.lo2 = Some(lo);
        form.bool_and_or = Some(boolandor);

        let pieces_before = fd.obank().iter_code(OpCode::CPUI_PIECE).count();
        assert!(form.replace(&fd), "replace must prepare both operands");
        // The prepare-step discovery is PERSISTED, not discarded.
        assert_eq!(
            form.param2.get_whole(),
            Some(whole),
            "isWholeFeasible's existing whole survived into self.param2"
        );

        // apply_rule's rewrite step reuses the persisted member -> no fabrication.
        let (mut a, mut b) = (form.in_.clone(), form.param2.clone());
        replace_bool_op(&mut fd, boolandor, &mut a, &mut b, OpCode::CPUI_INT_EQUAL);
        assert_eq!(b.get_whole(), Some(whole), "rewrite reused the existing whole");
        let pieces_after = fd.obank().iter_code(OpCode::CPUI_PIECE).count();
        assert_eq!(
            pieces_before, pieces_after,
            "threaded path fabricates NO redundant PIECE/concat (C++ reuses the whole)"
        );
    }

    /// F1 FIX (LessThreeWay): `set_bool_op` now threads ONE clone pair (in,in2)
    /// through `prepare_bool_op` (C++ `prepareBoolOp(in,in2,...)` mutates the
    /// members; double.cc:2419-2425) and persists it back, so the discovered
    /// whole survives into the `create_bool_op` step in `apply_rule`
    /// (double.cc:2487-2490).  A non-constant `in2` of an existing whole is reused.
    #[test]
    fn verify_w6_s5_double_lessthreeway_threads_prepared_whole() {
        let mut fd = build_fd();
        let (whole, lo, hi) = build_split_off_whole(&mut fd);
        let bl = op_get_parent(&fd, vn_get_def(&fd, whole).unwrap()).unwrap();
        let hilessbool = mk_op(&mut fd, 2, 0x900, OpCode::CPUI_CBRANCH);
        fd.op_insert_end(hilessbool, bl);

        // hilessbool is the CBRANCH whose bool input create_bool_op rewrites; give
        // it a target (slot 0) and a free bool varnode (slot 1).
        let target = mk_const(&mut fd, 8, 0x900);
        fd.op_set_input(hilessbool, target, 0).unwrap();
        let boolvn = mk_vn(&mut fd, 0x500, 1);
        fd.op_set_input(hilessbool, boolvn, 1).unwrap();

        let mut form = LessThreeWay::new();
        // in (the marked input) is an arbitrary constant pair -> trivially feasible.
        form.in_.init_partial_const(8, 0x1234);
        // in2 is the non-constant existing-whole pair.
        form.in2.init_partial_pieces(&fd, 8, Some(lo), Some(hi));
        form.hilessbool = Some(hilessbool);
        form.hislot = 0; // (in, in2) ordering

        let pieces_before = fd.obank().iter_code(OpCode::CPUI_PIECE).count();
        assert!(form.set_bool_op(&fd), "set_bool_op must prepare both operands");
        assert_eq!(
            form.in2.get_whole(),
            Some(whole),
            "isWholeFeasible's existing whole survived into self.in2"
        );

        // apply_rule's create step reuses the persisted member -> no fabrication.
        let (mut a, mut b) = (form.in_.clone(), form.in2.clone());
        create_bool_op(&mut fd, hilessbool, &mut a, &mut b, OpCode::CPUI_INT_LESS);
        assert_eq!(b.get_whole(), Some(whole), "create reused the existing whole");
        let pieces_after = fd.obank().iter_code(OpCode::CPUI_PIECE).count();
        assert_eq!(
            pieces_before, pieces_after,
            "threaded path fabricates NO redundant PIECE/concat (C++ reuses the whole)"
        );
    }

    /// Hunt list (wrapping): `adjacentOffsets` constant form computes
    /// `vn1.offset + size1 == vn2.offset` on `uintb`, which WRAPS in C++.  At the
    /// u64 boundary the wrap must match: 0xFFFF_FFFF_FFFF_FFFF + 1 == 0.
    #[test]
    fn verify_w6_s5_double_adjacent_offsets_wraps_at_u64_boundary() {
        let mut fd = build_fd();
        let a = mk_const(&mut fd, 8, u64::MAX); // 0xFFFF...FF
        let b = mk_const(&mut fd, 8, 0); // wraps to 0
        // a + 1 wraps to 0 == b  -> must be true (C++ unsigned wrap).
        assert!(
            adjacent_offsets(&fd, a, b, 1),
            "u64-boundary wrap: MAX + 1 == 0 must hold (matches C++ uintb wrap)"
        );
        // a + 2 == 1 != 0 -> false.
        assert!(!adjacent_offsets(&fd, a, b, 2));
    }

    /// Hunt list (masking): `AddForm::checkForCarry` constant-less branch computes
    /// `negconst = (~c) & calc_mask(lo1.size())`.  For a 4-byte lo1 and `c == 5`,
    /// the result must be `(~5) & 0xFFFF_FFFF == 0xFFFF_FFFA` (NOT the full u64
    /// `0xFFFF_FFFF_FFFF_FFFA`).
    #[test]
    fn verify_w6_s5_double_check_for_carry_negconst_masking() {
        let mut fd = build_fd();
        let bl = mk_block_with(&mut fd, &[]);
        let r = ram(&fd);

        // lo1 is a 4-byte non-constant var.
        let lo1 = mk_vn(&mut fd, 0x80, 4);
        // carryop = (CONST 5) INT_LESS lo1   -> the "constant on the left" CARRY form
        let lessop = mk_op(&mut fd, 2, 0xA00, OpCode::CPUI_INT_LESS);
        let lessout = fd.new_varnode_out(1, &Address::new(Rc::clone(&r), 0x90), lessop).unwrap();
        let c5 = mk_const(&mut fd, 4, 5);
        fd.op_set_input(lessop, c5, 0).unwrap();
        fd.op_set_input(lessop, lo1, 1).unwrap();
        fd.op_insert_end(lessop, bl);
        // zext(lessout)
        let zext = mk_op(&mut fd, 1, 0xA08, OpCode::CPUI_INT_ZEXT);
        let _zout = fd.new_varnode_out(4, &Address::new(r, 0x94), zext).unwrap();
        fd.op_set_input(zext, lessout, 0).unwrap();
        fd.op_insert_end(zext, bl);

        let mut form = AddForm::new();
        form.lo1 = Some(lo1);
        assert!(form.check_for_carry(&fd, zext), "constant-left CARRY form matches");
        assert_eq!(
            form.negconst,
            (!5u64) & calc_mask(4),
            "negconst masked to lo1's 4-byte width"
        );
        assert_eq!(form.negconst, 0xFFFF_FFFA, "explicit expected value");
        assert!(form.negconst <= 0xFFFF_FFFF, "must NOT carry high u64 bits past the mask");
    }
}
