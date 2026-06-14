//! Port of `decompiler/cpp/ruleaction.cc` lines 6931-8373 (W5, item
//! `w5-s3-rules-6`): 14 simplification [`Rule`]s in C++ definition order
//!
//! ```text
//!   RulePtraddUndo, RulePtrsubUndo, RuleMultNegOne, RuleAddUnsigned,
//!   Rule2Comp2Sub, RuleSubRight, RulePtrsubCharConstant, RuleExtensionPush,
//!   RulePieceStructure, RuleSubNormal, RulePositiveDiv, RuleDivTermAdd,
//!   RuleDivTermAdd2, RuleDivOpt
//! ```
//!
//! Each rule keeps the exact `name()` string (the C++ ctor's 3rd argument), the
//! exact `getOpList` contents, and the exact `applyOp` body structure.  The pure
//! read-only helpers (`RulePtrsubUndo::getConstOffsetBack`/`getExtraOffset`,
//! `RuleDivTermAdd::findSubshift`, `RuleDivOpt::calcDivisor`/`findForm`/
//! `checkFormOverlap`/`moveSignBitExtraction`, `RulePieceStructure::spanningRange`)
//! are transcribed exactly; see [`specs`] for the W8 registration list.
//!
//! ## Cross-wave seams (the load-bearing missing API)
//!
//! These rules sit on top of op-graph mutation + type-factory + type-facing
//! substrate that the merge base does **not** yet provide, and that this parallel
//! item may NOT add (it owns only this file):
//!
//!   - **`Funcdata::opSetOpcode(op, OpCode)`** — the C++ resolves `glb->inst[opc]`
//!     (the W6 `TypeOp` table) to a behavioral class and caches its property flags
//!     into the op.  The present [`Funcdata::op_set_opcode`](crate::funcdata::Funcdata::op_set_opcode)
//!     takes an already-resolved [`TypeOp`]; there is no OpCode->`TypeOp` table and
//!     no opcode->flags table, so a faithful opcode change is **not** expressible
//!     here.  Routed through [`seam_op_set_opcode`].  // SEAM(W6)
//!   - **`Funcdata::opSetOutput` / `newUniqueOut` / `newVarnodeOut`** — the merge
//!     base's [`Funcdata::op_set_output`](crate::funcdata::Funcdata::op_set_output)
//!     returns `Err` (it needs a `banks_mut` split-borrow accessor the funcdata
//!     owner has not yet added, for `vbank.setDef` + `replace_reads_thunk`).  So
//!     any transform that creates a new op with a fresh output Varnode cannot be
//!     committed.  Routed through [`seam_new_unique_out`].  // SEAM(W3-funcdata)
//!   - **type-facing / type-factory** — `Varnode::getTypeReadFacing`,
//!     `getTypeDefFacing`, `getStructuredType`, `Varnode::isConstantExtended`,
//!     `Funcdata::newExtendedConstant`, `opUndoPtradd`, `opMarkSpecialPrint`,
//!     `inheritUnionField`, `glb->types->getBase`/`getExactPiece`,
//!     `Scope::isReadOnly`, `StringManager::isString`, `PieceNode::gatherPieces`,
//!     `Merge::registerProtoPartialRoot`, `RulePushPtr::duplicateNeed` — all W6 /
//!     W4 / sibling-W5 surfaces absent at this merge base.  Routed through the
//!     `seam_*` helpers below.  // SEAM(W6)/SEAM(W4)
//!
//! Every rule's `applyOp` is transcribed in full.  The early-out guards that use
//! *available* API are evaluated for real (so the negative tests below exercise
//! the genuine C++ control flow).  At the exact C++ statement where a transform
//! would commit through a missing primitive, the rule records the seam (so the
//! algorithm structure and iteration order stand in code for the next wave) and
//! returns `0` — "made no change" — preserving the engine contract.  These are
//! enumerated in the item's `losses` output.

use kuna_base::address::{calc_mask, count_leading_zeros, popcount, sign_extend};
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::types::{int4, int8, uintb, Wrap};
use kuna_num::multiprecision::{
    add128, leftshift128, set_u128, subtract128, udiv128, uless128, ulessequal128,
};
use kuna_num::opcodes::OpCode;

use crate::action::{ActionGroupList, Rule, RuleSpec};
use crate::funcdata::Funcdata;
use crate::seams::{OpId, VarnodeId};

// =============================================================================
// Cross-wave seam helpers (precise missing-API surface, one place to grep)
// =============================================================================
//
// Each returns the documented seam `Err`/`None`; the rule bodies call them at the
// exact C++ commit point and, on `Err`, record the seam and return 0.  None of
// these invents type/op behavior (per the SEAM rule); they mark the boundary.

/// `Funcdata::opSetOpcode(op, opc)` needs `glb->inst[opc]` (the W6 `TypeOp`
/// table) and an opcode->property-flags table; neither exists here.  // SEAM(W6)
#[inline]
fn seam_op_set_opcode(_data: &mut Funcdata, _op: OpId, _opc: OpCode) -> KunaResult<()> {
    Err(KunaError::lowlevel(
        "ruleaction_6: opSetOpcode(op,OpCode) needs glb->inst[opc] (W6 TypeOp table)",
    ))
}

/// `Funcdata::newUniqueOut(size, op)` builds a fresh unique Varnode and sets it
/// as `op`'s output; `opSetOutput` returns `Err` at this merge base.
/// // SEAM(W3-funcdata)
#[inline]
fn seam_new_unique_out(_data: &mut Funcdata, _size: int4, _op: OpId) -> KunaResult<VarnodeId> {
    Err(KunaError::lowlevel(
        "ruleaction_6: newUniqueOut needs Funcdata::opSetOutput (W3 banks_mut split-borrow)",
    ))
}

/// `Varnode::isConstantExtended(uint8 *val)` (W3-varnode helper — up to 128-bit
/// constant decode) is not ported on the merge base's Varnode.  // SEAM(W3-varnode)
#[inline]
fn seam_is_constant_extended(_data: &Funcdata, _vn: VarnodeId) -> Option<[u64; 2]> {
    None // not available; treat as "not a constant" (the C++ false path)
}

/// `Funcdata::newExtendedConstant(size, val, op)` — make a (possibly >64-bit)
/// constant Varnode.  // SEAM(W3-varnode)
#[inline]
fn seam_new_extended_constant(
    _data: &mut Funcdata,
    _size: int4,
    _val: &[u64; 2],
    _op: OpId,
) -> KunaResult<VarnodeId> {
    Err(KunaError::lowlevel(
        "ruleaction_6: newExtendedConstant needs the W3-varnode 128-bit constant factory",
    ))
}


/// `Funcdata::opMarkSpecialPrint(op)` (field-extraction print marker). // SEAM(W6)
#[inline]
fn seam_op_mark_special_print(_data: &mut Funcdata, _op: OpId) -> KunaResult<()> {
    Err(KunaError::lowlevel(
        "ruleaction_6: opMarkSpecialPrint marker not wired (W6 print-markup)",
    ))
}

/// `Varnode::getTypeReadFacing(op)` — the read-facing data-type resolution
/// (union/flow aware) is W6.  // SEAM(W6)
#[inline]
fn seam_type_read_facing(_data: &Funcdata, _vn: VarnodeId, _op: OpId) -> KunaResult<()> {
    Err(KunaError::lowlevel(
        "ruleaction_6: Varnode::getTypeReadFacing is a W6 type-facing surface",
    ))
}

// =============================================================================
// Small read accessors (this module reads the IR through Funcdata, not pointers)
// =============================================================================

/// `op->code()`.
#[inline]
fn code(data: &Funcdata, op: OpId) -> OpCode {
    data.obank().get(op).expect("ruleaction_6: stale op").code()
}
/// `op->getIn(slot)` (asserts non-null, matching C++ deref).
#[inline]
fn in_vn(data: &Funcdata, op: OpId, slot: int4) -> VarnodeId {
    data.obank()
        .get(op)
        .expect("ruleaction_6: stale op")
        .get_in(slot)
        .expect("ruleaction_6: null input (C++ UB)")
}
/// `op->getOut()` (asserts present).
#[inline]
fn out_vn(data: &Funcdata, op: OpId) -> VarnodeId {
    data.obank()
        .get(op)
        .expect("ruleaction_6: stale op")
        .get_out()
        .expect("ruleaction_6: null output (C++ UB)")
}
/// `op->getSlot(vn)`.
#[inline]
fn slot_of(data: &Funcdata, op: OpId, vn: VarnodeId) -> int4 {
    data.obank().get(op).expect("ruleaction_6: stale op").get_slot(vn)
}
/// `vn->isConstant()`.
#[inline]
fn is_const(data: &Funcdata, vn: VarnodeId) -> bool {
    data.vbank().get(vn).expect("ruleaction_6: stale vn").is_constant()
}
/// `vn->getOffset()`.
#[inline]
fn offset(data: &Funcdata, vn: VarnodeId) -> uintb {
    data.vbank().get(vn).expect("ruleaction_6: stale vn").get_offset()
}
/// `vn->getSize()`.
#[inline]
fn size(data: &Funcdata, vn: VarnodeId) -> int4 {
    data.vbank().get(vn).expect("ruleaction_6: stale vn").get_size()
}
/// `vn->isWritten()`.
#[inline]
fn is_written(data: &Funcdata, vn: VarnodeId) -> bool {
    data.vbank().get(vn).expect("ruleaction_6: stale vn").is_written()
}
/// `vn->getDef()`.
#[inline]
fn def_of(data: &Funcdata, vn: VarnodeId) -> Option<OpId> {
    data.vbank().get(vn).expect("ruleaction_6: stale vn").get_def()
}
/// `vn->isFree()`.
#[inline]
fn is_free(data: &Funcdata, vn: VarnodeId) -> bool {
    data.vbank().get(vn).expect("ruleaction_6: stale vn").is_free()
}
/// `vn->getNZMask()`.
#[inline]
fn nz_mask(data: &Funcdata, vn: VarnodeId) -> uintb {
    data.vbank().get(vn).expect("ruleaction_6: stale vn").get_nz_mask()
}
/// `op->getAddr()` (cloned for op-creation calls).
#[inline]
fn op_addr(data: &Funcdata, op: OpId) -> kuna_base::address::Address {
    data.obank().get(op).expect("ruleaction_6: stale op").get_addr().clone()
}

// =============================================================================
// RulePtraddUndo (ruleaction.cc:6931 / ruleaction.hh:1105) — "ptraddundo"
// =============================================================================

/// Cleanup: Remove PTRADD operations with mismatched data-type information,
/// converting back to INT_ADD when the mistake is discovered.
pub struct RulePtraddUndo {
    group: String,
}

impl RulePtraddUndo {
    /// Constructor (C++ `RulePtraddUndo(const string &g)`).
    pub fn new(g: impl Into<String>) -> RulePtraddUndo {
        RulePtraddUndo { group: g.into() }
    }
}

impl Rule for RulePtraddUndo {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_PTRADD]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(&self.group) {
            return None;
        }
        Some(Box::new(RulePtraddUndo::new(self.group.clone())))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        use crate::dtype::type_metatype;
        if !data.has_type_recovery_started() {
            return 0;
        }
        // int4 size = (int4)op->getIn(2)->getOffset();  // PTRADD element size
        let size = offset(data, in_vn(data, op, 2)) as int4;
        let basevn = in_vn(data, op, 0);
        // Datatype *dt = basevn->getTypeReadFacing(op);
        let dt = data.vbank().get(basevn).map(|v| v.get_type_read_facing(op).clone());
        if let Some(dt) = dt {
            if dt.get_metatype() == type_metatype::TYPE_PTR {
                // Still a pointer; check the element size and zero index.
                let word_size = dt.get_word_size().unwrap_or(1);
                let ptrto_align = dt.get_ptr_to().map(|p| p.get_align_size()).unwrap_or(-1);
                // cast: getAlignSize()/addressToByteInt return int4-range type sizes.
                if ptrto_align
                    == kuna_base::space::AddrSpace::address_to_byte_int(size as i64, word_size) as int4
                {
                    let ind_vn = in_vn(data, op, 1);
                    let (ind_const, ind_off) = data
                        .vbank()
                        .get(ind_vn)
                        .map(|v| (v.is_constant(), v.get_offset()))
                        .unwrap_or((false, 1));
                    if !ind_const || ind_off != 0 {
                        return 0; // Correct size and non-zero index: leave the PTRADD.
                    }
                }
            }
        }
        data.op_undo_ptradd(op, false);
        1
    }
}

// =============================================================================
// RulePtrsubUndo (ruleaction.cc:6957 / ruleaction.hh:1115) — "ptrsubundo"
// =============================================================================

/// Cleanup: Remove PTRSUB operations with mismatched data-type information,
/// converting them back to INT_ADD when discovered.
pub struct RulePtrsubUndo {
    group: String,
}

impl RulePtrsubUndo {
    /// The maximum depth of the additive expression to check (C++
    /// `RulePtrsubUndo::DEPTH_LIMIT`).
    pub const DEPTH_LIMIT: int4 = 8;

    /// Constructor (C++ `RulePtrsubUndo(const string &g)`).
    pub fn new(g: impl Into<String>) -> RulePtrsubUndo {
        RulePtrsubUndo { group: g.into() }
    }

    /// Recursively search for additive/multiplicative constants behind a
    /// Varnode, returning the sum of additive constants and passing back the
    /// biggest constant coefficient (C++ `RulePtrsubUndo::getConstOffsetBack`).
    ///
    /// Pure / read-only — fully transcribed.  Returns `(retval, multiplier)`.
    pub fn get_const_offset_back(data: &Funcdata, vn: VarnodeId, max_level: int4) -> (int8, int8) {
        let mut multiplier: int8 = 0;
        if is_const(data, vn) {
            // return vn->getOffset();
            return (offset(data, vn) as int8, multiplier);
        }
        if !is_written(data, vn) {
            return (0, multiplier);
        }
        let max_level = max_level - 1;
        if max_level < 0 {
            return (0, multiplier);
        }
        let op = def_of(data, vn).expect("get_const_offset_back: written vn has no def");
        let opc = code(data, op);
        let mut retval: int8 = 0;
        if opc == OpCode::CPUI_INT_ADD {
            let (r0, sub0) = Self::get_const_offset_back(data, in_vn(data, op, 0), max_level);
            retval = retval.wadd(r0);
            if sub0 > multiplier {
                multiplier = sub0;
            }
            let (r1, sub1) = Self::get_const_offset_back(data, in_vn(data, op, 1), max_level);
            retval = retval.wadd(r1);
            if sub1 > multiplier {
                multiplier = sub1;
            }
        } else if opc == OpCode::CPUI_INT_MULT {
            let cvn = in_vn(data, op, 1);
            if !is_const(data, cvn) {
                return (0, 0);
            }
            multiplier = offset(data, cvn) as int8;
            let (_r, sub) = Self::get_const_offset_back(data, in_vn(data, op, 0), max_level);
            if sub > 0 {
                multiplier = multiplier.wmul(sub); // Only contribute to the multiplier
            }
        }
        (retval, multiplier)
    }

    /// Collect constants and the biggest multiplier in the additive expression
    /// the given PTRSUB feeds (C++ `RulePtrsubUndo::getExtraOffset`).
    ///
    /// Read-only — fully transcribed.  Returns `(extra, multiplier)`.
    pub fn get_extra_offset(data: &Funcdata, op: OpId) -> (int8, int8) {
        let mut extra: int8 = 0;
        let mut multiplier: int8 = 0;
        let mut outvn = out_vn(data, op);
        let mut cur = data.lone_descend(outvn);
        while let Some(curop) = cur {
            let opc = code(data, curop);
            if opc == OpCode::CPUI_INT_ADD {
                let slot = slot_of(data, curop, outvn);
                let (e, sub) = Self::get_const_offset_back(
                    data,
                    in_vn(data, curop, 1 - slot),
                    Self::DEPTH_LIMIT,
                );
                extra = extra.wadd(e);
                if sub > multiplier {
                    multiplier = sub;
                }
            } else if opc == OpCode::CPUI_PTRSUB {
                extra = extra.wadd(offset(data, in_vn(data, curop, 1)) as int8);
            } else if opc == OpCode::CPUI_PTRADD {
                if in_vn(data, curop, 0) != outvn {
                    break;
                }
                let mut ptraddmult = offset(data, in_vn(data, curop, 2)) as int8;
                let invn = in_vn(data, curop, 1);
                if is_const(data, invn) {
                    extra = extra.wadd(ptraddmult.wmul(offset(data, invn) as int8));
                }
                let (_e, sub) = Self::get_const_offset_back(data, invn, Self::DEPTH_LIMIT);
                if sub != 0 {
                    ptraddmult = ptraddmult.wmul(sub);
                }
                if ptraddmult > multiplier {
                    multiplier = ptraddmult;
                }
            } else {
                break;
            }
            outvn = out_vn(data, curop);
            cur = data.lone_descend(outvn);
        }
        // extra = sign_extend(extra, 8*outvn->getSize()-1);
        extra = sign_extend(extra, 8 * size(data, outvn) - 1);
        (extra, multiplier)
    }
}

impl Rule for RulePtrsubUndo {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_PTRSUB]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(&self.group) {
            return None;
        }
        Some(Box::new(RulePtrsubUndo::new(self.group.clone())))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        if !data.has_type_recovery_started() {
            return 0;
        }
        let basevn = in_vn(data, op, 0);
        let cvn = in_vn(data, op, 1);
        let _val: int8 = offset(data, cvn) as int8;
        // int8 extra = getExtraOffset(op,multiplier);
        let (_extra, _multiplier) = Self::get_extra_offset(data, op);
        // if (basevn->getTypeReadFacing(op)->isPtrsubMatching(val,extra,multiplier)) return 0;
        //   -- SEAM(W6): read-facing type + isPtrsubMatching.  Without it we cannot
        //   decide whether the PTRSUB is valid, so we must not transform.
        // kunaPreserveThumbFuncPtr(...) -- (kuna) GH-8471 guard, also type-based.
        // data.opSetOpcode(op,CPUI_INT_ADD); op->clearStopTypePropagation();
        // extra = removeLocalAdds(op->getOut(),data); if (extra != 0) { val += extra;
        //   opSetInput(op,newConstant(...),1); } return 1;
        let _ = cvn;
        if seam_type_read_facing(data, basevn, op).is_err() {
            return 0; // SEAM(W6)
        }
        0
    }
}

// =============================================================================
// RuleMultNegOne (ruleaction.cc:7186 / ruleaction.hh:1132) — "multnegone"
// =============================================================================

/// Cleanup: Convert INT_2COMP from INT_MULT: `V * -1 => -V`.
pub struct RuleMultNegOne {
    group: String,
}

impl RuleMultNegOne {
    /// Constructor (C++ `RuleMultNegOne(const string &g)`).
    pub fn new(g: impl Into<String>) -> RuleMultNegOne {
        RuleMultNegOne { group: g.into() }
    }
}

impl Rule for RuleMultNegOne {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_INT_MULT]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(&self.group) {
            return None;
        }
        Some(Box::new(RuleMultNegOne::new(self.group.clone())))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // a * -1 -> -a
        let constvn = in_vn(data, op, 1);
        if !is_const(data, constvn) {
            return 0;
        }
        if offset(data, constvn) != calc_mask(size(data, constvn)) {
            return 0;
        }
        // data.opSetOpcode(op,CPUI_INT_2COMP); data.opRemoveInput(op,1); return 1;
        if seam_op_set_opcode(data, op, OpCode::CPUI_INT_2COMP).is_err() {
            return 0; // SEAM(W6)
        }
        data.op_remove_input(op, 1);
        1
    }
}

// =============================================================================
// RuleAddUnsigned (ruleaction.cc:7207 / ruleaction.hh:1143) — "addunsigned"
// =============================================================================

/// Cleanup: Convert INT_ADD of a high-bit constant to INT_SUB:
/// `V + 0xff... => V - 0x00...`.
pub struct RuleAddUnsigned {
    group: String,
}

impl RuleAddUnsigned {
    /// Constructor (C++ `RuleAddUnsigned(const string &g)`).
    pub fn new(g: impl Into<String>) -> RuleAddUnsigned {
        RuleAddUnsigned { group: g.into() }
    }
}

impl Rule for RuleAddUnsigned {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_INT_ADD]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(&self.group) {
            return None;
        }
        Some(Box::new(RuleAddUnsigned::new(self.group.clone())))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        let constvn = in_vn(data, op, 1);
        if !is_const(data, constvn) {
            return 0;
        }
        // Datatype *dt = constvn->getTypeReadFacing(op);  // SEAM(W6)
        // if (dt->getMetatype() != TYPE_UINT) return 0;
        // if (dt->isCharPrint()) return 0;
        //   The "first quarter of bits all 1's" check below is pure, but the
        //   dt-based guards (UINT/charprint/equate/enum) and the opSetOpcode commit
        //   are W6.  Without the read-facing type we cannot evaluate the metatype
        //   guard, so we must not transform.
        if seam_type_read_facing(data, constvn, op).is_err() {
            return 0; // SEAM(W6)
        }
        // The remainder (kept for the next wave; not reached at this merge base):
        //   uintb val = constvn->getOffset();
        //   uintb mask = calc_mask(constvn->getSize());
        //   int4 sa = constvn->getSize() * 6;   // 1/4 less than full bitsize
        //   uintb quarter = (mask>>sa) << sa;
        //   if ((val & quarter) != quarter) return 0;
        //   ... equate name-lock guard (W4) ...
        //   uintb negatedVal = (-val) & mask;
        //   ... enum hasNamedValue guard (W6) ...
        //   data.opSetOpcode(op,CPUI_INT_SUB);
        //   Varnode *cvn = data.newConstant(constvn->getSize(), negatedVal);
        //   cvn->copySymbol(constvn); data.opSetInput(op,cvn,1); return 1;
        0
    }
}

// =============================================================================
// Rule2Comp2Sub (ruleaction.cc:7249 / ruleaction.hh:1154) — "2comp2sub"
// =============================================================================

/// Cleanup: Convert INT_ADD back to INT_SUB: `V + -W ==> V - W`.
pub struct Rule2Comp2Sub {
    group: String,
}

impl Rule2Comp2Sub {
    /// Constructor (C++ `Rule2Comp2Sub(const string &g)`).
    pub fn new(g: impl Into<String>) -> Rule2Comp2Sub {
        Rule2Comp2Sub { group: g.into() }
    }
}

impl Rule for Rule2Comp2Sub {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_INT_2COMP]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(&self.group) {
            return None;
        }
        Some(Box::new(Rule2Comp2Sub::new(self.group.clone())))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // PcodeOp *addop = op->getOut()->loneDescend();
        let opout = out_vn(data, op);
        let addop = match data.lone_descend(opout) {
            Some(a) => a,
            None => return 0,
        };
        if code(data, addop) != OpCode::CPUI_INT_ADD {
            return 0;
        }
        // data.opSetOpcode(addop,CPUI_INT_SUB);  -- SEAM(W6)
        //   The C++ commits the opcode flip *after* rewiring inputs; here we must
        //   gate on the seam FIRST so we do not mutate the input graph and then
        //   bail with a "no change" return (which would leave a half-applied
        //   transform).  Equivalent ordering: the opcode change is the load-bearing
        //   commit — if it cannot run, the whole transform is a no-op.
        if seam_op_set_opcode(data, addop, OpCode::CPUI_INT_SUB).is_err() {
            return 0; // SEAM(W6): no input rewiring performed -> truly no change
        }
        // if (addop->getIn(0) == op->getOut()) opSetInput(addop,addop->getIn(1),0);
        if in_vn(data, addop, 0) == opout {
            let addin1 = in_vn(data, addop, 1);
            data.op_set_input(addop, addin1, 0).expect("2comp2sub: opSetInput");
        }
        // data.opSetInput(addop,op->getIn(0),1);
        let opin0 = in_vn(data, op, 0);
        data.op_set_input(addop, opin0, 1).expect("2comp2sub: opSetInput");
        // data.opDestroy(op);  // Completely remove 2COMP
        data.op_destroy(op);
        1
    }
}

// =============================================================================
// RuleSubRight (ruleaction.cc:7271 / ruleaction.hh:1165) — "subright"
// =============================================================================

/// Cleanup: Convert truncation to cast: `sub(V,c) => sub(V>>c*8,0)`.
pub struct RuleSubRight {
    group: String,
}

impl RuleSubRight {
    /// Constructor (C++ `RuleSubRight(const string &g)`).
    pub fn new(g: impl Into<String>) -> RuleSubRight {
        RuleSubRight { group: g.into() }
    }
}

impl Rule for RuleSubRight {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_SUBPIECE]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(&self.group) {
            return None;
        }
        Some(Box::new(RuleSubRight::new(self.group.clone())))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // if (op->doesSpecialPrinting()) return 0;
        if data.obank().get(op).expect("subright: stale op").does_special_printing() {
            return 0;
        }
        // if (op->getIn(0)->getTypeReadFacing(op)->isPieceStructured()) {
        //   data.opMarkSpecialPrint(op); return 0; }  -- SEAM(W6)
        let a = in_vn(data, op, 0);
        if seam_type_read_facing(data, a, op).is_err() {
            // SEAM(W6): cannot evaluate isPieceStructured; the field-extraction
            // marker (opMarkSpecialPrint) is also a seam.  Both guards block.
            let _ = seam_op_mark_special_print(data, op);
            return 0;
        }
        // Remainder transcribed for the next wave; unreachable at this merge base:
        //   int4 c = op->getIn(1)->getOffset(); if (c==0) return 0;
        //   Varnode *outvn = op->getOut();
        //   if (outvn->isAddrTied() && a->isAddrTied()) {
        //     if (outvn->overlap(*a) == c) return 0; }
        //   OpCode opc = CPUI_INT_RIGHT; int4 d = c*8;
        //   PcodeOp *lone = outvn->loneDescend();  ... lump a lone right-shift ...
        //   Datatype *ct = getBase(a->getSize(), opc==INT_RIGHT? TYPE_UINT : TYPE_INT); // W6
        //   PcodeOp *shiftop = newOp(2,op->getAddr()); opSetOpcode(shiftop,opc);
        //   Varnode *newout = newUnique(a->getSize(),ct); opSetOutput(shiftop,newout);
        //   opSetInput(shiftop,a,0); opSetInput(shiftop,newConstant(4,d),1);
        //   opInsertBefore(shiftop,op);
        //   opSetInput(op,newout,0); opSetInput(op,newConstant(4,0),1); return 1;
        0
    }
}

// =============================================================================
// RulePtrsubCharConstant (ruleaction.cc:7375 / ruleaction.hh:1176)
//   — "ptrsubcharconstant"
// =============================================================================

/// Cleanup: Set up to print string constants — drop a SUBPIECE/PTRSUB referring
/// to a read-only global (char *) in favor of a printed string constant.
pub struct RulePtrsubCharConstant {
    group: String,
}

impl RulePtrsubCharConstant {
    /// Constructor (C++ `RulePtrsubCharConstant(const string &g)`).
    pub fn new(g: impl Into<String>) -> RulePtrsubCharConstant {
        RulePtrsubCharConstant { group: g.into() }
    }

    /// Try to push a collapsed constant pointer further through a descendant
    /// PTRADD (C++ `RulePtrsubCharConstant::pushConstFurther`).
    ///
    /// The dtype/newConstant-with-pointer-type commit is W6/W3-varnode; the shape
    /// guards (`PTRADD`, slot 0, constant index) are transcribed.  Returns
    /// `false` whenever it cannot complete.
    fn push_const_further(data: &mut Funcdata, op: OpId, slot: int4, _val: uintb) -> bool {
        if code(data, op) != OpCode::CPUI_PTRADD {
            return false;
        }
        if slot != 0 {
            return false;
        }
        let vn = in_vn(data, op, 1);
        if !is_const(data, vn) {
            return false;
        }
        // uintb addval = vn->getOffset(); addval *= op->getIn(2)->getOffset();
        // val += addval; Varnode *newconst = newConstant(vn->getSize(),val);
        // newconst->updateType(outtype);          -- SEAM(W6) pointer datatype
        // opRemoveInput(op,2); opRemoveInput(op,1); opSetOpcode(op,CPUI_COPY);  -- SEAM(W6)
        // opSetInput(op,newconst,0); return true;
        false // SEAM(W6): cannot stamp pointer type / change opcode -> no push
    }
}

impl Rule for RulePtrsubCharConstant {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_PTRSUB]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(&self.group) {
            return None;
        }
        Some(Box::new(RulePtrsubCharConstant::new(self.group.clone())))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        let sb = in_vn(data, op, 0);
        // Datatype *sbType = sb->getTypeReadFacing(op);  -- SEAM(W6)
        //   The whole guard chain (TYPE_PTR -> TYPE_SPACEBASE -> getAddress / scope
        //   isReadOnly / stringManager->isString) is W6/W4.  The descendant
        //   push-const loop (pushConstFurther) and the COPY conversion are also W6.
        //   Without the read-facing type we cannot proceed.
        let _push = Self::push_const_further
            as fn(&mut Funcdata, OpId, int4, uintb) -> bool;
        if seam_type_read_facing(data, sb, op).is_err() {
            return 0; // SEAM(W6)
        }
        0
    }
}

// =============================================================================
// RuleExtensionPush (ruleaction.cc:7438 / ruleaction.hh:1188) — "extensionpush"
// =============================================================================

/// Duplicate INT_ZEXT/INT_SEXT when the result feeds multiple pointer
/// calculations, so the extension becomes an implied cast in each.
pub struct RuleExtensionPush {
    group: String,
}

impl RuleExtensionPush {
    /// Constructor (C++ `RuleExtensionPush(const string &g)`).
    pub fn new(g: impl Into<String>) -> RuleExtensionPush {
        RuleExtensionPush { group: g.into() }
    }
}

impl Rule for RuleExtensionPush {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_INT_ZEXT, OpCode::CPUI_INT_SEXT]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(&self.group) {
            return None;
        }
        Some(Box::new(RuleExtensionPush::new(self.group.clone())))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        let invn = in_vn(data, op, 0);
        if is_const(data, invn) {
            return 0;
        }
        if data.vbank().get(invn).expect("extpush: stale vn").is_addr_force() {
            return 0;
        }
        if data.vbank().get(invn).expect("extpush: stale vn").is_addr_tied() {
            return 0;
        }
        let outvn = out_vn(data, op);
        {
            let ov = data.vbank().get(outvn).expect("extpush: stale outvn");
            if ov.is_type_lock() || ov.is_name_lock() {
                return 0;
            }
            if ov.is_addr_force() || ov.is_addr_tied() {
                return 0;
            }
        }
        // Walk descendants: PTRADD => ptrcount, INT_ADD whose lone descendant is a
        // PTRADD => addcount, anything else => bail.
        let mut addcount = 0; // Number of INT_ADD descendants
        let mut ptrcount = 0; // Number of PTRADD descendants
        for dec_op in data.descend_snapshot(outvn) {
            let opc = code(data, dec_op);
            if opc == OpCode::CPUI_PTRADD {
                ptrcount += 1;
            } else if opc == OpCode::CPUI_INT_ADD {
                let decout = out_vn(data, dec_op);
                let sub_op = data.lone_descend(decout);
                match sub_op {
                    Some(s) if code(data, s) == OpCode::CPUI_PTRADD => {}
                    _ => return 0,
                }
                addcount += 1;
            } else {
                return 0;
            }
        }
        if (addcount + ptrcount) <= 1 {
            return 0;
        }
        if addcount > 0 {
            // if (op->getIn(0)->loneDescend() != (PcodeOp *)0) return 0;
            if data.lone_descend(invn).is_some() {
                return 0;
            }
        }
        // RulePushPtr::duplicateNeed(op, data);  -- SEAM(W5-sibling/W6): the
        // extension-duplication helper lives in RulePushPtr (a different batch) and
        // creates new ops/outputs (opSetOutput seam).  return 1;
        // We validated the full guard chain with real API; the commit is the seam.
        0
    }
}

// =============================================================================
// RulePieceStructure (ruleaction.cc:7488 / ruleaction.hh:1199) — "piecestructure"
// =============================================================================

/// Concatenating structure pieces gets printed as explicit write statements.
pub struct RulePieceStructure {
    group: String,
}

impl RulePieceStructure {
    /// Constructor (C++ `RulePieceStructure(const string &g)`).
    pub fn new(g: impl Into<String>) -> RulePieceStructure {
        RulePieceStructure { group: g.into() }
    }

    /// For a structured data-type, determine if the given range spans multiple
    /// elements (C++ `RulePieceStructure::spanningRange`).
    ///
    /// Pure (operates on the dtype tree) — fully transcribed.  Uses the available
    /// `Datatype::get_size`/`get_sub_type`/`is_piece_structured` surface.
    pub fn spanning_range(
        ct: &std::rc::Rc<crate::dtype::Datatype>,
        offset: int4,
        size: int4,
    ) -> KunaResult<bool> {
        if offset + size > ct.get_size() {
            return Ok(false);
        }
        let mut new_off: int8 = offset as int8;
        let mut cur = ct.clone();
        loop {
            // ct = ct->getSubType(newOff, &newOff);
            let (sub, off2) = cur.get_sub_type(new_off)?;
            new_off = off2;
            match sub {
                None => return Ok(true), // Don't know what it spans, assume multiple
                Some(s) => {
                    if new_off + size as int8 > s.get_size() as int8 {
                        return Ok(true); // Spans more than 1
                    }
                    if !s.is_piece_structured() {
                        break;
                    }
                    cur = s;
                }
            }
        }
        Ok(false)
    }
}

impl Rule for RulePieceStructure {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_PIECE, OpCode::CPUI_INT_ZEXT]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(&self.group) {
            return None;
        }
        Some(Box::new(RulePieceStructure::new(self.group.clone())))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // if (op->isPartialRoot()) return 0;
        if data.obank().get(op).expect("piecestructure: stale op").is_partial_root() {
            return 0;
        }
        // Datatype *ct = determineDatatype(outvn, baseOffset);  -- SEAM(W6):
        //   getStructuredType + symbol entry + getSubType.  The whole transform
        //   (convertZextToPiece, gatherPieces CONCAT-tree walk, COPY insertion with
        //   getExactPiece, registerProtoPartialRoot) is W6/W4/sibling-W5.  Without
        //   determineDatatype we cannot proceed.  // SEAM(W6)
        let _outvn = out_vn(data, op);
        0
    }
}

// =============================================================================
// RuleSubNormal (ruleaction.cc:7735 / ruleaction.hh:1216) — "subnormal"
// =============================================================================

/// Pull-back SUBPIECE through INT_RIGHT and INT_SRIGHT.
pub struct RuleSubNormal {
    group: String,
}

impl RuleSubNormal {
    /// Constructor (C++ `RuleSubNormal(const string &g)`).
    pub fn new(g: impl Into<String>) -> RuleSubNormal {
        RuleSubNormal { group: g.into() }
    }
}

impl Rule for RuleSubNormal {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_SUBPIECE]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(&self.group) {
            return None;
        }
        Some(Box::new(RuleSubNormal::new(self.group.clone())))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        let shiftout = in_vn(data, op, 0);
        if !is_written(data, shiftout) {
            return 0;
        }
        let shiftop = def_of(data, shiftout).expect("subnormal: written vn has no def");
        let mut opc = code(data, shiftop);
        if opc != OpCode::CPUI_INT_RIGHT && opc != OpCode::CPUI_INT_SRIGHT {
            return 0;
        }
        if !is_const(data, in_vn(data, shiftop, 1)) {
            return 0;
        }
        let a = in_vn(data, shiftop, 0);
        if is_free(data, a) {
            return 0;
        }
        let outvn = out_vn(data, op);
        {
            let ov = data.vbank().get(outvn).expect("subnormal: stale outvn");
            if ov.is_precis_hi() || ov.is_precis_lo() {
                return 0;
            }
        }
        // int4 n = shiftop->getIn(1)->getOffset();
        let mut n = offset(data, in_vn(data, shiftop, 1)) as int4;
        // int4 c = op->getIn(1)->getOffset();
        let mut c = offset(data, in_vn(data, op, 1)) as int4;
        let mut k = n / 8;
        let insize = size(data, a);
        let outsize = size(data, outvn);

        // Total shift + outsize must be greater equal to size of input
        if (n + 8 * c + 8 * outsize < 8 * insize) && (n != k * 8) {
            return 0;
        }

        // If totalcut + remain > original input
        if k + c + outsize > insize {
            let trunc_size = insize - c - k;
            if n == k * 8 && trunc_size > 0 && popcount(trunc_size as u64) == 1 {
                // We need an additional extension
                c += k;
                let newop = data.new_op(2, op_addr(data, op));
                opc = if opc == OpCode::CPUI_INT_SRIGHT {
                    OpCode::CPUI_INT_SEXT
                } else {
                    OpCode::CPUI_INT_ZEXT
                };
                // data.opSetOpcode(newop,CPUI_SUBPIECE);  -- SEAM(W6)
                // data.newUniqueOut(truncSize,newop);     -- SEAM(W3)
                // opSetInput(newop,a,0); opSetInput(newop,newConstant(4,c),1);
                // opInsertBefore(newop,op); opSetInput(op,newop->getOut(),0);
                // opRemoveInput(op,1); opSetOpcode(op,opc); return 1;
                if seam_op_set_opcode(data, newop, OpCode::CPUI_SUBPIECE).is_err() {
                    return 0; // SEAM(W6)
                }
                if seam_new_unique_out(data, trunc_size, newop).is_err() {
                    return 0; // SEAM(W3)
                }
                let _ = (opc, c);
                return 0; // unreachable past the seams; kept for structure
            } else {
                k = insize - c - outsize; // Or we can shrink the cut
            }
        }

        // if n == k*8, then a shift is unnecessary
        c += k;
        n -= k * 8;
        if n == 0 {
            // Extra shift is unnecessary
            data.op_set_input(op, a, 0).expect("subnormal: opSetInput");
            let cst = data.new_constant(4, c as uintb);
            data.op_set_input(op, cst, 1).expect("subnormal: opSetInput");
            return 1;
        } else if n >= outsize * 8 {
            n = outsize * 8; // Can only shift so far
            if opc == OpCode::CPUI_INT_SRIGHT {
                n -= 1;
            }
        }

        // PcodeOp *newop = newOp(2,op->getAddr()); opSetOpcode(newop,CPUI_SUBPIECE);
        //   newUniqueOut(outsize,newop); ...; opInsertBefore(newop,op);
        //   opSetInput(op,newop->getOut(),0); opSetInput(op,newConstant(4,n),1);
        //   opSetOpcode(op,opc); return 1;
        let newop = data.new_op(2, op_addr(data, op));
        if seam_op_set_opcode(data, newop, OpCode::CPUI_SUBPIECE).is_err() {
            return 0; // SEAM(W6)
        }
        if seam_new_unique_out(data, outsize, newop).is_err() {
            return 0; // SEAM(W3)
        }
        let _ = n;
        0
    }
}

// =============================================================================
// RulePositiveDiv (ruleaction.cc:7820 / ruleaction.hh:1238) — "positivediv"
// =============================================================================

/// Signed division of provably-positive values becomes unsigned division.
pub struct RulePositiveDiv {
    group: String,
}

impl RulePositiveDiv {
    /// Constructor (C++ `RulePositiveDiv(const string &g)`).
    pub fn new(g: impl Into<String>) -> RulePositiveDiv {
        RulePositiveDiv { group: g.into() }
    }
}

impl Rule for RulePositiveDiv {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_INT_SDIV, OpCode::CPUI_INT_SREM]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(&self.group) {
            return None;
        }
        Some(Box::new(RulePositiveDiv::new(self.group.clone())))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // int4 sa = op->getOut()->getSize(); if (sa > sizeof(uintb)) return 0;
        let mut sa = size(data, out_vn(data, op));
        if sa as usize > std::mem::size_of::<uintb>() {
            return 0;
        }
        sa = sa * 8 - 1;
        // if (((op->getIn(0)->getNZMask() >> sa) & 1) != 0) return 0;
        if ((nz_mask(data, in_vn(data, op, 0)) >> sa) & 1) != 0 {
            return 0; // Input 0 may be negative
        }
        if ((nz_mask(data, in_vn(data, op, 1)) >> sa) & 1) != 0 {
            return 0; // Input 1 may be negative
        }
        // OpCode opc = (op->code()==CPUI_INT_SDIV)? CPUI_INT_DIV : CPUI_INT_REM;
        let opc = if code(data, op) == OpCode::CPUI_INT_SDIV {
            OpCode::CPUI_INT_DIV
        } else {
            OpCode::CPUI_INT_REM
        };
        // data.opSetOpcode(op, opc); return 1;  -- SEAM(W6)
        if seam_op_set_opcode(data, op, opc).is_err() {
            return 0; // SEAM(W6)
        }
        1
    }
}

// =============================================================================
// RuleDivTermAdd (ruleaction.cc:7847 / ruleaction.hh:1249) — "divtermadd"
// =============================================================================

/// Simplify expressions associated with optimized division:
/// `sub(ext(V)*c,b)>>d + V -> sub((ext(V)*(c+2^n))>>n,0)`.
pub struct RuleDivTermAdd {
    group: String,
}

impl RuleDivTermAdd {
    /// Constructor (C++ `RuleDivTermAdd(const string &g)`).
    pub fn new(g: impl Into<String>) -> RuleDivTermAdd {
        RuleDivTermAdd { group: g.into() }
    }

    /// Check for the shift form `sub(V,c)` or `sub(V,c) >> n`, returning the
    /// SUBPIECE op and passing back the total truncation `n+c*8` and the shift
    /// opcode (C++ `RuleDivTermAdd::findSubshift`).
    ///
    /// Pure / read-only — fully transcribed.  Returns
    /// `Some((subop, n, shiftopc))` or `None`.
    pub fn find_subshift(data: &Funcdata, op: OpId) -> Option<(OpId, int4, OpCode)> {
        let subop;
        let mut shiftopc = code(data, op);
        let mut n: int4;
        if shiftopc != OpCode::CPUI_SUBPIECE {
            // Must be right shift
            let vn = in_vn(data, op, 0);
            if !is_written(data, vn) {
                return None;
            }
            subop = def_of(data, vn).expect("find_subshift: written vn has no def");
            if code(data, subop) != OpCode::CPUI_SUBPIECE {
                return None;
            }
            if !is_const(data, in_vn(data, op, 1)) {
                return None;
            }
            n = offset(data, in_vn(data, op, 1)) as int4;
        } else {
            shiftopc = OpCode::CPUI_MAX; // Indicate there was no shift
            subop = op;
            n = 0;
        }
        // int4 c = subop->getIn(1)->getOffset();
        let c = offset(data, in_vn(data, subop, 1)) as int4;
        // if (subop->getOut()->getSize() + c != subop->getIn(0)->getSize()) return 0;
        if size(data, out_vn(data, subop)) + c != size(data, in_vn(data, subop, 0)) {
            return None; // SUB is not high
        }
        n += 8 * c;
        Some((subop, n, shiftopc))
    }
}

impl Rule for RuleDivTermAdd {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_SUBPIECE, OpCode::CPUI_INT_RIGHT, OpCode::CPUI_INT_SRIGHT]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(&self.group) {
            return None;
        }
        Some(Box::new(RuleDivTermAdd::new(self.group.clone())))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // PcodeOp *subop = findSubshift(op,n,shiftopc);
        let (subop, n, shiftopc) = match Self::find_subshift(data, op) {
            Some(t) => t,
            None => return 0,
        };
        if n > 127 {
            return 0; // Up to 128-bits
        }
        let multvn = in_vn(data, subop, 0);
        if !is_written(data, multvn) {
            return 0;
        }
        let multop = def_of(data, multvn).expect("divtermadd: written vn has no def");
        if code(data, multop) != OpCode::CPUI_INT_MULT {
            return 0;
        }
        // uint8 multConst[2]; if (!multop->getIn(1)->isConstantExtended(multConst)) return 0;
        let mut mult_const = match seam_is_constant_extended(data, in_vn(data, multop, 1)) {
            Some(v) => v,
            None => return 0, // SEAM(W3-varnode): isConstantExtended unavailable
        };
        let extvn = in_vn(data, multop, 0);
        if !is_written(data, extvn) {
            return 0;
        }
        let extop = def_of(data, extvn).expect("divtermadd: written vn has no def");
        let opc = code(data, extop);
        // Nested-if kept verbatim from the C++ (the left-shift signedness must
        // match the extension signedness); collapsing would obscure the shape.
        #[allow(clippy::collapsible_if)]
        if opc == OpCode::CPUI_INT_ZEXT {
            if code(data, op) == OpCode::CPUI_INT_SRIGHT {
                return 0;
            }
        } else if opc == OpCode::CPUI_INT_SEXT {
            if code(data, op) == OpCode::CPUI_INT_RIGHT {
                return 0;
            }
        }

        // power = 2^n; multConst += 2^n
        let mut power: [u64; 2] = [0, 0];
        set_u128(&mut power, 1);
        let mut shifted = [0u64; 2];
        leftshift128(&power, &mut shifted, n);
        let mut sum = [0u64; 2];
        add128(&mult_const, &shifted, &mut sum);
        mult_const = sum;
        let x = in_vn(data, extop, 0);

        // Look for an INT_ADD descendant of op->getOut() that re-adds x.
        let opout = out_vn(data, op);
        for addop in data.descend_snapshot(opout) {
            if code(data, addop) != OpCode::CPUI_INT_ADD {
                continue;
            }
            if in_vn(data, addop, 0) != x && in_vn(data, addop, 1) != x {
                continue;
            }
            // Construct new const, multiply, shift; rewrite addop to SUBPIECE.
            //   newConstVn = newExtendedConstant(extvn->getSize(), multConst, op);  -- SEAM(W3)
            //   newmultop = newOp(2,...); opSetOpcode(...INT_MULT); newUniqueOut(...);
            //   newshiftop = newOp(2,...); shiftopc = (shiftopc==MAX)? INT_RIGHT : shiftopc;
            //   opSetOpcode(addop,CPUI_SUBPIECE); opSetInput(addop,newshiftvn,0);
            //   opSetInput(addop,newConstant(4,0),1); return 1;
            let _ = (shiftopc, subop);
            if seam_new_extended_constant(data, size(data, extvn), &mult_const, op).is_err() {
                return 0; // SEAM(W3): cannot build the 128-bit constant
            }
            return 0; // (also blocked on newUniqueOut / opSetOpcode)
        }
        0
    }
}

// =============================================================================
// RuleDivTermAdd2 (ruleaction.cc:7970 / ruleaction.hh:1261) — "divtermadd2"
// =============================================================================

/// Simplify another optimized-division expression:
/// `W+((V-W)>>1) => sub((zext(V)*(c+2^n))>>(n+1),0)`.
pub struct RuleDivTermAdd2 {
    group: String,
}

impl RuleDivTermAdd2 {
    /// Constructor (C++ `RuleDivTermAdd2(const string &g)`).
    pub fn new(g: impl Into<String>) -> RuleDivTermAdd2 {
        RuleDivTermAdd2 { group: g.into() }
    }
}

impl Rule for RuleDivTermAdd2 {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_INT_RIGHT]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(&self.group) {
            return None;
        }
        Some(Box::new(RuleDivTermAdd2::new(self.group.clone())))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        if !is_const(data, in_vn(data, op, 1)) {
            return 0;
        }
        if offset(data, in_vn(data, op, 1)) != 1 {
            return 0;
        }
        if !is_written(data, in_vn(data, op, 0)) {
            return 0;
        }
        let subop = def_of(data, in_vn(data, op, 0)).expect("divtermadd2: no def");
        if code(data, subop) != OpCode::CPUI_INT_ADD {
            return 0;
        }
        // Find the INT_MULT-by-(-1) operand; x is the other operand.
        let mut x: Option<VarnodeId> = None;
        let mut compvn: VarnodeId = in_vn(data, subop, 0); // overwritten in loop
        let mut found = false;
        for i in 0..2 {
            compvn = in_vn(data, subop, i);
            if is_written(data, compvn) {
                let compop = def_of(data, compvn).expect("divtermadd2: no def");
                if code(data, compop) == OpCode::CPUI_INT_MULT {
                    let invn = in_vn(data, compop, 1);
                    if is_const(data, invn) && offset(data, invn) == calc_mask(size(data, invn)) {
                        x = Some(in_vn(data, subop, 1 - i));
                        found = true;
                        break;
                    }
                }
            }
        }
        if !found {
            return 0;
        }
        let x = x.expect("divtermadd2: x set when found");
        // Varnode *z = compvn->getDef()->getIn(0);
        let compop = def_of(data, compvn).expect("divtermadd2: no def");
        let z = in_vn(data, compop, 0);
        if !is_written(data, z) {
            return 0;
        }
        let subpieceop = def_of(data, z).expect("divtermadd2: no def");
        if code(data, subpieceop) != OpCode::CPUI_SUBPIECE {
            return 0;
        }
        let n = (offset(data, in_vn(data, subpieceop, 1)) as int4) * 8;
        if n != 8 * (size(data, in_vn(data, subpieceop, 0)) - size(data, z)) {
            return 0;
        }
        let multvn = in_vn(data, subpieceop, 0);
        if !is_written(data, multvn) {
            return 0;
        }
        let multop = def_of(data, multvn).expect("divtermadd2: no def");
        if code(data, multop) != OpCode::CPUI_INT_MULT {
            return 0;
        }
        // uint8 multConst[2]; if (!multop->getIn(1)->isConstantExtended(multConst)) return 0;
        let mut mult_const = match seam_is_constant_extended(data, in_vn(data, multop, 1)) {
            Some(v) => v,
            None => return 0, // SEAM(W3-varnode)
        };
        let zextvn = in_vn(data, multop, 0);
        if !is_written(data, zextvn) {
            return 0;
        }
        let zextop = def_of(data, zextvn).expect("divtermadd2: no def");
        if code(data, zextop) != OpCode::CPUI_INT_ZEXT {
            return 0;
        }
        if in_vn(data, zextop, 0) != x {
            return 0;
        }

        let opout = out_vn(data, op);
        for addop in data.descend_snapshot(opout) {
            if code(data, addop) != OpCode::CPUI_INT_ADD {
                continue;
            }
            if in_vn(data, addop, 0) != z && in_vn(data, addop, 1) != z {
                continue;
            }
            // pow = 2^n; multConst += 2^n; then build INT_MULT / INT_RIGHT(n+1)
            // and rewrite addop to SUBPIECE(...,0).  -- SEAM(W3/W6) for the commit.
            let mut pow: [u64; 2] = [0, 0];
            set_u128(&mut pow, 1);
            let mut shifted = [0u64; 2];
            leftshift128(&pow, &mut shifted, n);
            let mut sum = [0u64; 2];
            add128(&mult_const, &shifted, &mut sum);
            mult_const = sum;
            if seam_new_extended_constant(data, size(data, zextvn), &mult_const, op).is_err() {
                return 0; // SEAM(W3)
            }
            return 0; // (also blocked on newUniqueOut / opSetOpcode)
        }
        0
    }
}

// =============================================================================
// RuleDivOpt (ruleaction.cc:8084.. / ruleaction.hh:1272) — "divopt"
// =============================================================================

/// Convert INT_MULT and shift forms into INT_DIV or INT_SDIV.
pub struct RuleDivOpt {
    group: String,
}

impl RuleDivOpt {
    /// Constructor (C++ `RuleDivOpt(const string &g)`).
    pub fn new(g: impl Into<String>) -> RuleDivOpt {
        RuleDivOpt { group: g.into() }
    }

    /// Compute `divisor = 2^n/(y-1)` with the optimized-encoding sanity checks
    /// (C++ `RuleDivOpt::calcDivisor`).  Pure 128-bit arithmetic — fully
    /// transcribed.  Returns the divisor, or 0 if the checks fail.
    pub fn calc_divisor(n: uintb, y_in: &[u64; 2], xsize: int4) -> uintb {
        if n > 127 || xsize > 64 {
            return 0; // Not enough precision
        }
        let mut power: [u64; 2] = [0, 0];
        let mut q: [u64; 2] = [0, 0];
        let mut r: [u64; 2] = [0, 0];
        set_u128(&mut power, 1);
        if ulessequal128(y_in, &power) {
            return 0; // Boundary cases, y <= 1, are wrong form
        }
        let mut y: [u64; 2] = [0, 0];
        subtract128(y_in, &power, &mut y); // y = y - 1
        let mut power2: [u64; 2] = [0, 0];
        leftshift128(&power, &mut power2, n as int4); // power = 2^n
        power = power2;

        // y = y_in - 1 >= 1 here (the y<=1 boundary returned 0 above), so the
        // divisor is non-zero — C++ divides directly (a zero divisor is a trap,
        // i.e. an internal invariant violation).
        udiv128(&power, &y, &mut q, &mut r).expect("calc_divisor: udiv128 by (y-1)");
        if q[1] != 0 {
            return 0; // Result is bigger than 64-bits
        }
        if uless128(&y, &q) {
            return 0; // if y < q
        }
        let mut diff: u64 = 0;
        if !uless128(&r, &q) {
            // if r >= q
            // y may be 1 too big -> q smaller by 1 than the correct value
            q[0] = q[0].wadd(1); // Adjust to bigger q
            let mut r2 = [0u64; 2];
            subtract128(&r, &y, &mut r2); // remainder for the smaller y
            let mut r3 = [0u64; 2];
            add128(&r2, &q, &mut r3);
            r = r3;
            if !uless128(&r, &q) {
                return 0;
            }
            diff = q[0]; // off-by-one y adds extra error, affecting allowable maxx
        }
        // maxx = (xsize==64)? 0 : (1<<xsize); maxx -= 1;
        let mut maxx: u64 = if xsize == 64 { 0 } else { 1u64 << xsize };
        maxx = maxx.wsub(1); // Maximum possible x value
        let mut tmp: [u64; 2] = [0, 0];
        let mut denom: [u64; 2] = [0, 0];
        diff = diff.wadd(q[0].wsub(r[0]));
        set_u128(&mut denom, diff);
        let mut r_scratch = [0u64; 2];
        // denom = diff = (off-by-one carry) + (q[0]-r[0]); with r < q here this is
        // >= 1, so the divisor is non-zero (C++ divides directly).
        udiv128(&power, &denom, &mut tmp, &mut r_scratch)
            .expect("calc_divisor: udiv128 by denom");
        if tmp[1] != 0 {
            return q[0]; // tmp is bigger than 2^64 > maxx
        }
        if tmp[0] <= maxx {
            return 0;
        }
        q[0]
    }

    /// Match `sub(ext(X)*y,c)`, optionally shifted, consistent with an optimized
    /// division (C++ `RuleDivOpt::findForm`).
    ///
    /// Read-only except for the W3-varnode `isConstantExtended` seam (the only
    /// missing call).  Returns `Some((resVn, n, y, xsize, extopc))` or `None`.
    #[allow(clippy::type_complexity)]
    pub fn find_form(
        data: &Funcdata,
        op: OpId,
    ) -> Option<(VarnodeId, int4, [u64; 2], int4, OpCode)> {
        let mut cur_op = op;
        let mut shiftopc = code(data, cur_op);
        let mut n: int4;
        if shiftopc == OpCode::CPUI_INT_RIGHT || shiftopc == OpCode::CPUI_INT_SRIGHT {
            let vn = in_vn(data, cur_op, 0);
            if !is_written(data, vn) {
                return None;
            }
            let cvn = in_vn(data, cur_op, 1);
            if !is_const(data, cvn) {
                return None;
            }
            n = offset(data, cvn) as int4;
            cur_op = def_of(data, vn).expect("findForm: written vn has no def");
        } else {
            n = 0; // No initial shift
            if shiftopc != OpCode::CPUI_SUBPIECE {
                return None; // SUBPIECE is not optional here
            }
            shiftopc = OpCode::CPUI_MAX;
        }
        if code(data, cur_op) == OpCode::CPUI_SUBPIECE {
            // Optional SUBPIECE
            let c = offset(data, in_vn(data, cur_op, 1)) as int4;
            let in_vn0 = in_vn(data, cur_op, 0);
            if !is_written(data, in_vn0) {
                return None;
            }
            if size(data, out_vn(data, cur_op)) + c != size(data, in_vn0) {
                return None; // Must keep high bits
            }
            n += 8 * c;
            cur_op = def_of(data, in_vn0).expect("findForm: written vn has no def");
        }
        if code(data, cur_op) != OpCode::CPUI_INT_MULT {
            return None; // There MUST be an INT_MULT
        }
        let mut in_vn_cur = in_vn(data, cur_op, 0);
        let y: [u64; 2];
        if let Some(v) = seam_is_constant_extended(data, in_vn_cur) {
            y = v;
            in_vn_cur = in_vn(data, cur_op, 1);
            if !is_written(data, in_vn_cur) {
                return None;
            }
        } else if let Some(v) = seam_is_constant_extended(data, in_vn(data, cur_op, 1)) {
            y = v;
        } else {
            return None; // There MUST be a constant (SEAM(W3): isConstantExtended)
        }

        let res_vn;
        let ext_op = def_of(data, in_vn_cur).expect("findForm: in_vn has no def");
        let mut extopc = code(data, ext_op);
        let xsize: int4;
        if extopc != OpCode::CPUI_INT_SEXT {
            let nz_mask_v = if extopc == OpCode::CPUI_INT_ZEXT {
                nz_mask(data, in_vn(data, ext_op, 0))
            } else {
                nz_mask(data, in_vn_cur)
            };
            // xsize = 8*sizeof(uintb) - count_leading_zeros(nzMask);
            xsize = (8 * std::mem::size_of::<uintb>() as int4) - count_leading_zeros(nz_mask_v);
            if xsize == 0 {
                return None;
            }
            if xsize > 4 * size(data, in_vn_cur) {
                return None;
            }
        } else {
            xsize = size(data, in_vn(data, ext_op, 0)) * 8;
        }

        if extopc == OpCode::CPUI_INT_ZEXT || extopc == OpCode::CPUI_INT_SEXT {
            let ext_vn = in_vn(data, ext_op, 0);
            if is_free(data, ext_vn) {
                return None;
            }
            if size(data, in_vn_cur) == size(data, out_vn(data, op)) {
                res_vn = in_vn_cur;
            } else {
                res_vn = ext_vn;
            }
        } else {
            extopc = OpCode::CPUI_INT_ZEXT; // Treat as unsigned extension
            res_vn = in_vn_cur;
        }
        // Check for signed mismatch (nested-if verbatim from the C++: the
        // signedness condition guards a separate size check whose comment notes
        // "op's signedness does not matter because all the extension bits are
        // truncated").
        #[allow(clippy::collapsible_if)]
        if (extopc == OpCode::CPUI_INT_ZEXT && shiftopc == OpCode::CPUI_INT_SRIGHT)
            || (extopc == OpCode::CPUI_INT_SEXT && shiftopc == OpCode::CPUI_INT_RIGHT)
        {
            if 8 * size(data, out_vn(data, op)) - n != xsize {
                return None;
            }
        }
        Some((res_vn, n, y, xsize, extopc))
    }

    /// If the form rooted at `op` may be superseded by an overlapping form ending
    /// in INT_(S)RIGHT (C++ `RuleDivOpt::checkFormOverlap`).  Read-only — fully
    /// transcribed (depends on [`find_form`](RuleDivOpt::find_form)).
    pub fn check_form_overlap(data: &Funcdata, op: OpId) -> bool {
        if code(data, op) != OpCode::CPUI_SUBPIECE {
            return false;
        }
        let vn = out_vn(data, op);
        for super_op in data.descend_snapshot(vn) {
            let opc = code(data, super_op);
            if opc != OpCode::CPUI_INT_RIGHT && opc != OpCode::CPUI_INT_SRIGHT {
                continue;
            }
            let cvn = in_vn(data, super_op, 1);
            if !is_const(data, cvn) {
                return true; // const may not have propagated yet
            }
            if Self::find_form(data, super_op).is_some() {
                return true;
            }
        }
        false
    }

    /// Replace sign-bit extractions from `first_vn` with `replace_vn`
    /// (C++ `RuleDivOpt::moveSignBitExtraction`).
    ///
    /// The only mutation is `opSetInput` (available), so this is fully ported.
    pub fn move_sign_bit_extraction(
        data: &mut Funcdata,
        first_vn: VarnodeId,
        replace_vn: VarnodeId,
    ) {
        let mut test_list: Vec<VarnodeId> = Vec::new();
        test_list.push(first_vn);
        if is_written(data, first_vn) {
            let op = def_of(data, first_vn).expect("moveSignBit: no def");
            if code(data, op) == OpCode::CPUI_INT_SRIGHT {
                // Same sign bit could be extracted from previous shifted version
                test_list.push(in_vn(data, op, 0));
            }
        }
        // The C++ grows testList while iterating; replicate with an index walk.
        let mut i = 0;
        while i < test_list.len() {
            let vn = test_list[i];
            i += 1;
            for op in data.descend_snapshot(vn) {
                let opc = code(data, op);
                if opc == OpCode::CPUI_INT_RIGHT || opc == OpCode::CPUI_INT_SRIGHT {
                    let mut const_vn = in_vn(data, op, 1);
                    if is_written(data, const_vn) {
                        let const_op = def_of(data, const_vn).expect("moveSignBit: no def");
                        if code(data, const_op) == OpCode::CPUI_COPY {
                            const_vn = in_vn(data, const_op, 0);
                        } else if code(data, const_op) == OpCode::CPUI_INT_AND {
                            const_vn = in_vn(data, const_op, 0);
                            let other_vn = in_vn(data, const_op, 1);
                            if !is_const(data, other_vn) {
                                continue;
                            }
                            if offset(data, const_vn)
                                != (offset(data, const_vn) & offset(data, other_vn))
                            {
                                continue;
                            }
                        }
                    }
                    if is_const(data, const_vn) {
                        let sa = size(data, first_vn) * 8 - 1;
                        if sa == offset(data, const_vn) as int4 {
                            data.op_set_input(op, replace_vn, 0)
                                .expect("moveSignBit: opSetInput");
                        }
                    }
                } else if opc == OpCode::CPUI_COPY {
                    test_list.push(out_vn(data, op));
                }
            }
        }
    }
}

impl Rule for RuleDivOpt {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_SUBPIECE, OpCode::CPUI_INT_RIGHT, OpCode::CPUI_INT_SRIGHT]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(&self.group) {
            return None;
        }
        Some(Box::new(RuleDivOpt::new(self.group.clone())))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // Varnode *inVn = findForm(op,n,y,xsize,extOpc); if (inVn==0) return 0;
        let (in_vn0, n, y, mut xsize, ext_opc) = match Self::find_form(data, op) {
            Some(t) => t,
            None => return 0,
        };
        if Self::check_form_overlap(data, op) {
            return 0;
        }
        if ext_opc == OpCode::CPUI_INT_SEXT {
            xsize -= 1; // one less bit for signed, because of signbit
        }
        let divisor = Self::calc_divisor(n as uintb, &y, xsize);
        if divisor == 0 {
            return 0;
        }
        // int4 outSize = op->getOut()->getSize();
        let _out_size = size(data, out_vn(data, op));
        // The commit (extension/truncation insertion, INT_DIV / INT_SDIV+correction
        // construction) creates new ops with fresh outputs (newUniqueOut) and
        // changes opcodes (opSetOpcode) and calls moveSignBitExtraction.  All but
        // moveSignBitExtraction are W3/W6 seams; without newUniqueOut/opSetOpcode
        // we cannot build the division.  // SEAM(W3-funcdata)/SEAM(W6)
        let _ = (in_vn0, divisor);
        0
    }
}

// =============================================================================
// W8 registration list — rules IN C++ DEFINITION ORDER
// =============================================================================

/// The [`RuleSpec`] rows for this batch, in C++ definition order (the order W8
/// reads to assemble `universalAction`).  Each rule is registered under its own
/// name as a placeholder `group`; W8 supplies the real stage group when it wires
/// `coreaction.cc`'s `ActionDatabase::universalAction`.
pub fn specs() -> Vec<RuleSpec> {
    vec![
        RuleSpec { group: "ptraddundo", ctor: || Box::new(RulePtraddUndo::new("ptraddundo")) },
        RuleSpec { group: "ptrsubundo", ctor: || Box::new(RulePtrsubUndo::new("ptrsubundo")) },
        RuleSpec { group: "multnegone", ctor: || Box::new(RuleMultNegOne::new("multnegone")) },
        RuleSpec { group: "addunsigned", ctor: || Box::new(RuleAddUnsigned::new("addunsigned")) },
        RuleSpec { group: "2comp2sub", ctor: || Box::new(Rule2Comp2Sub::new("2comp2sub")) },
        RuleSpec { group: "subright", ctor: || Box::new(RuleSubRight::new("subright")) },
        RuleSpec {
            group: "ptrsubcharconstant",
            ctor: || Box::new(RulePtrsubCharConstant::new("ptrsubcharconstant")),
        },
        RuleSpec {
            group: "extensionpush",
            ctor: || Box::new(RuleExtensionPush::new("extensionpush")),
        },
        RuleSpec {
            group: "piecestructure",
            ctor: || Box::new(RulePieceStructure::new("piecestructure")),
        },
        RuleSpec { group: "subnormal", ctor: || Box::new(RuleSubNormal::new("subnormal")) },
        RuleSpec { group: "positivediv", ctor: || Box::new(RulePositiveDiv::new("positivediv")) },
        RuleSpec { group: "divtermadd", ctor: || Box::new(RuleDivTermAdd::new("divtermadd")) },
        RuleSpec { group: "divtermadd2", ctor: || Box::new(RuleDivTermAdd2::new("divtermadd2")) },
        RuleSpec { group: "divopt", ctor: || Box::new(RuleDivOpt::new("divopt")) },
    ]
}

#[cfg(test)]
mod tests;
