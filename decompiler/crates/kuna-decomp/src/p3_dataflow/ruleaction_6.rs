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
//! ## Cross-wave stubs (the load-bearing missing API)
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
//!     here.  Routed through [`set_opcode_typed`].  // STUB(W6)
//!   - **`Funcdata::opSetOutput` / `newUniqueOut` / `newVarnodeOut`** — the merge
//!     base's [`Funcdata::op_set_output`](crate::funcdata::Funcdata::op_set_output)
//!     returns `Err` (it needs a `banks_mut` split-borrow accessor the funcdata
//!     owner has not yet added, for `vbank.setDef` + `replace_reads_thunk`).  So
//!     any transform that creates a new op with a fresh output Varnode cannot be
//!     committed.  Routed through [`new_unique_out_typed`].  // STUB(W3-funcdata)
//!   - **type-facing / type-factory** — `Varnode::getTypeReadFacing`,
//!     `getTypeDefFacing`, `getStructuredType`, `Varnode::isConstantExtended`,
//!     `Funcdata::newExtendedConstant`, `opUndoPtradd`, `opMarkSpecialPrint`,
//!     `inheritUnionField`, `glb->types->getBase`/`getExactPiece`,
//!     `Scope::isReadOnly`, `StringManager::isString`, `PieceNode::gatherPieces`,
//!     `Merge::registerProtoPartialRoot`, `RulePushPtr::duplicateNeed` — all W6 /
//!     W4 / sibling-W5 surfaces absent at this merge base.  Routed through the
//!     typed helpers below.  // STUB(W6)/STUB(W4)
//!
//! Every rule's `applyOp` is transcribed in full.  The early-out guards that use
//! *available* API are evaluated for real (so the negative tests below exercise
//! the genuine C++ control flow).  At the exact C++ statement where a transform
//! would commit through a missing primitive, the rule records the stub (so the
//! algorithm structure and iteration order stand in code for the next wave) and
//! returns `0` — "made no change" — preserving the engine contract.  These are
//! enumerated in the item's `losses` output.

use std::rc::Rc;

use kuna_base::address::{
    calc_mask, count_leading_zeros, popcount, sign_extend, sign_extend_sized, signbit_negative,
};
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::types::{int4, int8, uint4, uintb, Wrap};
use kuna_num::multiprecision::{
    add128, leftshift128, set_u128, subtract128, udiv128, uless128, ulessequal128,
};
use kuna_num::opcodes::OpCode;

use crate::action::{ActionGroupList, Rule, RuleSpec};
use crate::funcdata::Funcdata;
use crate::context::{OpId, TypeOp, VarnodeId};

// =============================================================================
// Cross-wave helper shims (precise missing-API surface, one place to grep)
// =============================================================================
//
// Each returns the documented `Err`/`None`; the rule bodies call them at the
// exact C++ commit point and, on `Err`, record the stub and return 0.  None of
// these invents type/op behavior (per the STUB rule); they mark the boundary.

/// `data.opSetOpcode(op, opc)` with the W6 inst-table resolution folded in
/// (the real funcdata mutator is now available end-to-end; this resolves the
/// bare [`OpCode`] to its [`TypeOp`] via [`typeop_for`] exactly as the sibling
/// `ruleaction_7`/`ruleaction_3` commit rules do).
#[inline]
fn set_opcode_typed(data: &mut Funcdata, op: OpId, opc: OpCode) -> KunaResult<()> {
    data.op_set_opcode(op, typeop_for(opc));
    Ok(())
}

/// `Funcdata::newUniqueOut(size, op)` — build a fresh unique Varnode and set it
/// as `op`'s output.  Routes to the real funcdata factory (available now that
/// the engine runs the pipeline end-to-end).
#[inline]
fn new_unique_out_typed(data: &mut Funcdata, size: int4, op: OpId) -> KunaResult<VarnodeId> {
    data.new_unique_out(size, op)
}

/// `Varnode::isConstantExtended(uint8 *val)` — if `vn` is a constant, or is
/// extended (INT_ZEXT, INT_SEXT, PIECE) from a constant, pass back the (up to
/// 128-bit) value.  Faithful transcription of `Varnode::isConstantExtended`
/// (varnode.cc:818).  Returns `Some([lo,hi])` or `None`.
fn is_constant_extended(data: &Funcdata, vn: VarnodeId) -> Option<[u64; 2]> {
    if is_const(data, vn) {
        return Some([offset(data, vn), 0]);
    }
    if !is_written(data, vn) || size(data, vn) <= 8 {
        return None;
    }
    if size(data, vn) > 16 {
        return None; // Currently only up to 128-bit values
    }
    let def = def_of(data, vn).expect("isConstantExtended: written vn has no def");
    let opc = code(data, def);
    if opc == OpCode::CPUI_INT_ZEXT {
        let vn0 = in_vn(data, def, 0);
        if is_const(data, vn0) {
            return Some([offset(data, vn0), 0]);
        }
    } else if opc == OpCode::CPUI_INT_SEXT {
        let vn0 = in_vn(data, def, 0);
        if is_const(data, vn0) {
            let mut v0 = offset(data, vn0);
            if size(data, vn0) < 8 {
                v0 = sign_extend_sized(v0, size(data, vn0), size(data, vn));
            }
            let v1 = if signbit_negative(v0, 8) { 0xffffffffffffffffu64 } else { 0 };
            return Some([v0, v1]);
        }
    } else if opc == OpCode::CPUI_PIECE {
        let vnlo = in_vn(data, def, 1);
        if is_const(data, vnlo) {
            let mut v0 = offset(data, vnlo);
            let vnhi = in_vn(data, def, 0);
            if is_const(data, vnhi) {
                let mut v1 = offset(data, vnhi);
                if size(data, vnlo) == 8 {
                    return Some([v0, v1]);
                }
                v0 |= v1.wshl((8 * size(data, vnlo)) as u32);
                v1 = v1.wshr((8 * (8 - size(data, vnlo))) as u32);
                return Some([v0, v1]);
            }
        }
    }
    None
}

/// `Funcdata::newExtendedConstant(size, val, op)` — make a (possibly >64-bit)
/// constant Varnode using INT_ZEXT or PIECE if necessary.  Faithful
/// transcription of `Funcdata::newExtendedConstant` (funcdata_varnode.cc:464).
fn new_extended_constant(
    data: &mut Funcdata,
    s: int4,
    val: &[u64; 2],
    op: OpId,
) -> KunaResult<VarnodeId> {
    if s <= 8 {
        return Ok(data.new_constant(s, val[0]));
    }
    let new_const_vn;
    let opaddr = op_addr(data, op);
    if val[1] == 0 {
        let ext_op = data.new_op(1, opaddr);
        set_opcode_typed(data, ext_op, OpCode::CPUI_INT_ZEXT)?;
        new_const_vn = new_unique_out_typed(data, s, ext_op)?;
        let c = data.new_constant(8, val[0]);
        data.op_set_input(ext_op, c, 0)?;
        data.op_insert_before(ext_op, op);
    } else {
        let piece_op = data.new_op(2, opaddr);
        set_opcode_typed(data, piece_op, OpCode::CPUI_PIECE)?;
        new_const_vn = new_unique_out_typed(data, s, piece_op)?;
        let chi = data.new_constant(8, val[1]); // Most significant piece
        let clo = data.new_constant(8, val[0]); // Least significant piece
        data.op_set_input(piece_op, chi, 0)?;
        data.op_set_input(piece_op, clo, 1)?;
        data.op_insert_before(piece_op, op);
    }
    Ok(new_const_vn)
}

/// `data.opSetOpcode(op, opc)` op-flags resolution (`glb->inst[opc]`) folded in:
/// builds the [`TypeOp`] with the exact `opflags` transcribed from `typeop.cc`
/// for every op-code these div/mod rules produce, so the op's cached property
/// bits match what the C++ would install.  The `addlflags`/`OpBehavior` are not
/// modelled here (a rule that wrote INT_DIV does not gain its arithmetic-op
/// addlflag); that wider table is the W6 stub.  // STUB(W6)
fn typeop_for(opc: OpCode) -> TypeOp {
    use crate::op::pcodeop_flags as f;
    // opflags transcribed verbatim from typeop.cc constructors.
    let (flags, name): (uint4, &str) = match opc {
        // TypeOpIntZext / TypeOpIntSext: unary
        OpCode::CPUI_INT_ZEXT => (f::unary, "ZEXT"),
        OpCode::CPUI_INT_SEXT => (f::unary, "SEXT"),
        // TypeOpIntAdd: binary | commutative
        OpCode::CPUI_INT_ADD => (f::binary | f::commutative, "+"),
        // TypeOpIntMult: binary | commutative
        OpCode::CPUI_INT_MULT => (f::binary | f::commutative, "*"),
        // TypeOpIntRight / TypeOpIntSright / TypeOpIntDiv / TypeOpIntSdiv /
        // TypeOpPiece / TypeOpSubpiece: binary
        OpCode::CPUI_INT_RIGHT => (f::binary, ">>"),
        OpCode::CPUI_INT_SRIGHT => (f::binary, "s>>"),
        OpCode::CPUI_INT_DIV => (f::binary, "/"),
        OpCode::CPUI_INT_SDIV => (f::binary, "/"),
        OpCode::CPUI_PIECE => (f::binary, "PIECE"),
        OpCode::CPUI_SUBPIECE => (f::binary, "SUB"),
        // Any other op-code these rules reach is a porting bug; fall back to a
        // bare binary skeleton (worst case: a missing special-semantics flag,
        // never an incorrect rewrite).  // STUB(W6)
        _ => (f::binary, "op"),
    };
    TypeOp::new(opc, flags, name.to_string())
}



/// `Varnode::getTypeReadFacing(op)` — the read-facing data-type resolution
/// (union/flow aware) is W6.  // STUB(W6)
#[inline]
fn type_read_facing_stub(_data: &Funcdata, _vn: VarnodeId, _op: OpId) -> KunaResult<()> {
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
/// `vn->loneDescend()`.
#[inline]
fn lone_descend(data: &Funcdata, vn: VarnodeId) -> Option<OpId> {
    data.lone_descend(vn)
}
/// `vn->isAddrTied()`.
#[inline]
fn is_addr_tied(data: &Funcdata, vn: VarnodeId) -> bool {
    data.vbank().get(vn).expect("ruleaction_6: stale vn").is_addr_tied()
}
/// `vn->isProtoPartial()`.
#[inline]
fn is_proto_partial(data: &Funcdata, vn: VarnodeId) -> bool {
    data.vbank().get(vn).expect("ruleaction_6: stale vn").is_proto_partial()
}
/// `vn->getAddr()` (cloned).
#[inline]
fn vn_addr(data: &Funcdata, vn: VarnodeId) -> kuna_base::address::Address {
    data.vbank().get(vn).expect("ruleaction_6: stale vn").get_addr().clone()
}
/// `vn->getType()` (cloned `Rc`).
#[inline]
fn vn_type(data: &Funcdata, vn: VarnodeId) -> std::rc::Rc<crate::dtype::Datatype> {
    std::rc::Rc::clone(data.vbank().get(vn).expect("ruleaction_6: stale vn").get_type())
}
/// `vn->getSpace()->isBigEndian()`.
#[inline]
fn vn_is_big_endian(data: &Funcdata, vn: VarnodeId) -> bool {
    data.vbank()
        .get(vn)
        .expect("ruleaction_6: stale vn")
        .get_addr()
        .get_space()
        .map(|s| s.is_big_endian())
        .unwrap_or(false)
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
        // Datatype *dt = basevn->getTypeReadFacing(op);  (resolve union/relptr in-flow)
        let dt = Some(data.vn_type_read_facing(basevn, op));
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

    /// `RulePtrsubUndo::removeLocalAddRecurse` (ruleaction.cc:7071) — recursively
    /// remove additive constants behind the `slot` input of `op`, converting the
    /// constant-adding INT_ADDs to COPY.  Returns the sum of removed constants.
    fn remove_local_add_recurse(
        data: &mut Funcdata,
        op: OpId,
        slot: int4,
        max_level: int4,
    ) -> int8 {
        // Varnode *vn = op->getIn(slot);
        let vn = in_vn(data, op, slot);
        if !is_written(data, vn) {
            return 0;
        }
        // if (vn->loneDescend() != op) return 0;  // not used anywhere else
        if lone_descend(data, vn) != Some(op) {
            return 0;
        }
        let max_level = max_level - 1;
        if max_level < 0 {
            return 0;
        }
        // op = vn->getDef();
        let op = def_of(data, vn).expect("removeLocalAddRecurse: written vn has no def");
        let mut retval: int8 = 0;
        if code(data, op) == OpCode::CPUI_INT_ADD {
            if is_const(data, in_vn(data, op, 1)) {
                retval = retval.wadd(offset(data, in_vn(data, op, 1)) as int8);
                data.op_remove_input(op, 1);
                data.op_set_opcode(op, typeop_for(OpCode::CPUI_COPY));
            } else {
                retval =
                    retval.wadd(Self::remove_local_add_recurse(data, op, 0, max_level));
                retval =
                    retval.wadd(Self::remove_local_add_recurse(data, op, 1, max_level));
            }
        }
        retval
    }

    /// `RulePtrsubUndo::removeLocalAdds` (ruleaction.cc:7106) — convert any
    /// additional PTRADD/PTRSUB/INT_ADD that uses `vn` and adds a constant into a
    /// COPY, plus any reachable constant-adding INT_ADD.  Returns the sum of the
    /// removed constants.
    fn remove_local_adds(data: &mut Funcdata, vn: VarnodeId) -> int8 {
        let mut extra: int8 = 0;
        // PcodeOp *op = vn->loneDescend();
        let mut op = lone_descend(data, vn);
        // Varnode *nextVn = vn;
        let mut next_vn = vn;
        while let Some(curop) = op {
            let opc = code(data, curop);
            if opc == OpCode::CPUI_INT_ADD {
                // int4 slot = op->getSlot(nextVn);
                let slot = slot_of(data, curop, next_vn);
                if slot == 0 && is_const(data, in_vn(data, curop, 1)) {
                    extra = extra.wadd(offset(data, in_vn(data, curop, 1)) as int8);
                    data.op_remove_input(curop, 1);
                    data.op_set_opcode(curop, typeop_for(OpCode::CPUI_COPY));
                } else {
                    extra = extra.wadd(Self::remove_local_add_recurse(
                        data,
                        curop,
                        1 - slot,
                        Self::DEPTH_LIMIT,
                    ));
                }
            } else if opc == OpCode::CPUI_PTRSUB {
                extra = extra.wadd(offset(data, in_vn(data, curop, 1)) as int8);
                data.obank_mut()
                    .get_mut(curop)
                    .expect("removeLocalAdds: stale ptrsub")
                    .clear_stop_type_propagation();
                data.op_remove_input(curop, 1);
                data.op_set_opcode(curop, typeop_for(OpCode::CPUI_COPY));
            } else if opc == OpCode::CPUI_PTRADD {
                // if (op->getIn(0) != nextVn) break;
                if in_vn(data, curop, 0) != next_vn {
                    break;
                }
                // int8 ptraddmult = op->getIn(2)->getOffset();
                let ptraddmult = offset(data, in_vn(data, curop, 2)) as int8;
                let invn = in_vn(data, curop, 1);
                if is_const(data, invn) {
                    extra = extra.wadd(ptraddmult.wmul(offset(data, invn) as int8));
                    data.op_remove_input(curop, 2);
                    data.op_remove_input(curop, 1);
                    data.op_set_opcode(curop, typeop_for(OpCode::CPUI_COPY));
                } else {
                    data.op_undo_ptradd(curop, false);
                    extra = extra.wadd(Self::remove_local_add_recurse(
                        data,
                        curop,
                        1,
                        Self::DEPTH_LIMIT,
                    ));
                }
            } else {
                break;
            }
            // nextVn = op->getOut();
            next_vn = out_vn(data, curop);
            // op = nextVn->loneDescend();
            op = lone_descend(data, next_vn);
        }
        // if (nextVn != vn) vn->updateType(nextVn->getType());
        if next_vn != vn {
            let nt = vn_type(data, next_vn);
            data.vbank_mut()
                .get_mut(vn)
                .expect("removeLocalAdds: stale vn")
                .update_type(nt);
        }
        extra
    }

    /// (kuna) GH-8471 guard: should `RulePtrsubUndo` preserve this PTRSUB as a
    /// mode-bit-encoded (Thumb) function pointer rather than collapse it to raw
    /// hex?  Resolves the W6 [`ThumbPtrTypeFacts`] from the read-facing pointer
    /// type and the (global-scope) `TypeSpacebase::getSubType` walk, then defers
    /// to [`kuna_preserve_thumb_funcptr`].
    ///
    /// `read_facing` is `basevn->getTypeReadFacing(op)`; `val`/`extra`/`multiplier`
    /// are the PTRSUB constant decomposition exactly as `applyOp` computes them.
    fn preserve_thumb_funcptr(
        data: &Funcdata,
        read_facing: &Rc<crate::dtype::Datatype>,
        val: int8,
        extra: int8,
        multiplier: int8,
    ) -> bool {
        use crate::dtype::type_metatype;
        use crate::kuna_thumbfuncptr::{
            kuna_preserve_thumb_funcptr, ThumbPtrTypeFacts, ThumbSubTypeFact,
        };
        let arch = data.get_arch();
        let funcptr_align = arch.funcptr_align;
        // Mirror the C++ short-circuit: `!preserve_thumb_funcptr || funcptr_align
        // == 0` -> false, before any type query.
        if !arch.preserve_thumb_funcptr || funcptr_align == 0 {
            return false;
        }
        // bt = basevn->getTypeReadFacing(op); require TYPE_PTR -> TYPE_SPACEBASE.
        let base_metatype = read_facing.get_metatype();
        if base_metatype != type_metatype::TYPE_PTR {
            return false;
        }
        let ptr_to = match read_facing.get_ptr_to() {
            Some(p) => p,
            None => return false,
        };
        let ptr_to_metatype = ptr_to.get_metatype();
        let word_size = read_facing.get_word_size().unwrap_or(1);
        // newoff = AddrSpace::addressToByteInt(val, ptype->getWordSize()).
        // The `getSubType` resolution only applies to the spacebase pointee; the
        // predicate re-checks `ptr_to_metatype == TYPE_SPACEBASE` itself, so only
        // resolve the sub-type when that holds.
        let sub = if ptr_to_metatype == type_metatype::TYPE_SPACEBASE {
            let newoff =
                kuna_base::space::AddrSpace::address_to_byte_int(val, word_size);
            match data.spacebase_get_sub_type(&ptr_to, newoff) {
                Some((sub_type, residual)) => Some(ThumbSubTypeFact {
                    metatype: sub_type.get_metatype(),
                    residual_offset: residual,
                }),
                None => None,
            }
        } else {
            None
        };
        let facts = ThumbPtrTypeFacts {
            base_metatype,
            ptr_to_metatype,
            word_size,
            sub,
        };
        kuna_preserve_thumb_funcptr(
            arch.preserve_thumb_funcptr,
            funcptr_align,
            val,
            extra,
            multiplier,
            &facts,
        )
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
        // if (!data.hasTypeRecoveryStarted()) return 0;
        if !data.has_type_recovery_started() {
            return 0;
        }
        let basevn = in_vn(data, op, 0);
        let cvn = in_vn(data, op, 1);
        // int8 val = cvn->getOffset();
        let mut val: int8 = offset(data, cvn) as int8;
        // int8 extra = getExtraOffset(op,multiplier);
        let (extra0, multiplier) = Self::get_extra_offset(data, op);
        // if (basevn->getTypeReadFacing(op)->isPtrsubMatching(val,extra,multiplier))
        //   return 0;
        // `getTypeReadFacing(op)->isPtrsubMatching` dispatches through
        // TypeSpacebase, which the kuna `is_ptrsub_matching_scope` resolves via the
        // local-variable Scope (the same call ActionSetCasts::castFixupPtrsub uses).
        let read_facing = data.vn_type_read_facing(basevn, op);
        if data.is_ptrsub_matching_scope(&read_facing, val, extra0, multiplier) {
            return 0;
        }
        // if (kunaPreserveThumbFuncPtr(basevn,op,val,extra,multiplier,data.getArch()))
        //   return 0;  -- (kuna) GH-8471
        // A pointer to a Thumb function carries the mode bit in its LSB
        // (`value = fn|1`).  When `option thumbfuncptr on` and the architecture
        // has aligned function pointers (`funcptr_align != 0`), keep the
        // `PTRSUB(fn) + 1` symbolic rather than collapsing it back to raw hex.
        // The predicate (`kuna_preserve_thumb_funcptr`) needs the resolved
        // TypeSpacebase::getSubType walk, which `is_ptrsub_matching_scope` already
        // routes through the function's scope; the global-symbol arm of
        // `spacebase_get_sub_type` resolves the constant `fn` to its TYPE_CODE
        // symbol type (the same path ActionConstantPtr::isPointer uses).
        if Self::preserve_thumb_funcptr(data, &read_facing, val, extra0, multiplier) {
            return 0;
        }

        // data.opSetOpcode(op,CPUI_INT_ADD);
        data.op_set_opcode(op, typeop_for(OpCode::CPUI_INT_ADD));
        // op->clearStopTypePropagation();
        data.obank_mut()
            .get_mut(op)
            .expect("ptrsubundo: stale op")
            .clear_stop_type_propagation();
        // extra = removeLocalAdds(op->getOut(),data);
        let outvn = out_vn(data, op);
        let extra = Self::remove_local_adds(data, outvn);
        if extra != 0 {
            // val = val + extra;
            val = val.wadd(extra);
            // data.opSetInput(op,data.newConstant(cvn->getSize(), val & calc_mask(...)),1);
            let csize = size(data, cvn);
            let nc = data.new_constant(csize, (val as uintb) & calc_mask(csize));
            data.op_set_input(op, nc, 1).expect("ptrsubundo: opSetInput");
        }
        1
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
        if set_opcode_typed(data, op, OpCode::CPUI_INT_2COMP).is_err() {
            return 0; // STUB(W6)
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
        use crate::dtype::type_metatype;
        let constvn = in_vn(data, op, 1);

        // if (!constvn->isConstant()) return 0;
        if !is_const(data, constvn) {
            return 0;
        }
        // Datatype *dt = constvn->getTypeReadFacing(op);
        // if (dt->getMetatype() != TYPE_UINT) return 0;
        // if (dt->isCharPrint()) return 0;     // Only change integer forms
        let dt = data.vn_type_read_facing(constvn, op);
        if dt.get_metatype() != type_metatype::TYPE_UINT {
            return 0;
        }
        if dt.is_char_print() {
            return 0;
        }
        // uintb val = constvn->getOffset();
        // uintb mask = calc_mask(constvn->getSize());
        // int4 sa = constvn->getSize() * 6;    // 1/4 less than full bitsize
        // uintb quarter = (mask>>sa) << sa;
        // if ((val & quarter) != quarter) return 0;  // first quarter bits all 1's
        let val = offset(data, constvn);
        let cv_size = size(data, constvn);
        let mask = calc_mask(cv_size);
        let sa = (cv_size * 6) as uint4; // 1/4 less than full bitsize
        let quarter = (mask >> sa) << sa;
        if (val & quarter) != quarter {
            return 0;
        }
        // if (constvn->getSymbolEntry() != 0) {
        //   EquateSymbol *sym = dynamic_cast<EquateSymbol*>(...->getSymbol());
        //   if (sym != 0) { if (sym->isNameLocked()) return 0; }   // named equate
        // }
        if let Some(sym) = data.vbank().get(constvn).and_then(|v| v.kuna_symbol_entry()) {
            if let Some(local) = data.get_scope_local() {
                let s = local.database().symbol(sym);
                if s.get_category() == crate::database::symbol_category::EQUATE
                    && s.is_name_locked()
                {
                    return 0;
                }
            }
        }
        // uintb negatedVal = (-val) & mask;
        let negated_val = val.wrapping_neg() & mask;
        // if (dt->isEnumType()) {
        //   TypeEnum *enumType = (TypeEnum *)dt;
        //   if (!enumType->hasNamedValue(negatedVal) && enumType->hasNamedValue((~val)&mask))
        //     return 0;
        // }
        if dt.is_enum_type()
            && !dt.has_named_value(negated_val)
            && dt.has_named_value((!val) & mask)
        {
            return 0;
        }
        // data.opSetOpcode(op,CPUI_INT_SUB);
        data.op_set_opcode_code(op, OpCode::CPUI_INT_SUB);
        // Varnode *cvn = data.newConstant(constvn->getSize(), negatedVal);
        let cvn = data.new_constant(cv_size, negated_val);
        // cvn->copySymbol(constvn);
        data.copy_symbol(cvn, constvn);
        // data.opSetInput(op,cvn,1);
        let _ = data.op_set_input(op, cvn, 1);
        1
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
        // data.opSetOpcode(addop,CPUI_INT_SUB);  -- STUB(W6)
        //   The C++ commits the opcode flip *after* rewiring inputs; here we must
        //   gate on the stub FIRST so we do not mutate the input graph and then
        //   bail with a "no change" return (which would leave a half-applied
        //   transform).  Equivalent ordering: the opcode change is the load-bearing
        //   commit — if it cannot run, the whole transform is a no-op.
        if set_opcode_typed(data, addop, OpCode::CPUI_INT_SUB).is_err() {
            return 0; // STUB(W6): no input rewiring performed -> truly no change
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
        //   data.opMarkSpecialPrint(op); return 0; }
        //
        // `Varnode::getTypeReadFacing(op)` (varnode.cc:658) returns the varnode's
        // own `type` unless it needs resolution, in which case `findResolve`.  At
        // cleanup time (where `RuleSubRight` runs) a struct/union/array input that
        // a SUBPIECE truncates a field out of carries its composite `type`, which
        // `isPieceStructured()` (metatype <= TYPE_ARRAY) reports true — so the op
        // is marked for the field-extraction render (`PrintC::opSubpiece`,
        // printc.cc:866).  The marker is a pure print flag (no IR rewrite, `return
        // 0`); the rewrite tail below is unaffected.
        let a = in_vn(data, op, 0);
        let in0_piece = data.vn_type_read_facing(a, op).is_piece_structured();
        if in0_piece {
            data.op_mark_special_print(op); // Print this as a field extraction
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
        // newconst->updateType(outtype);          -- STUB(W6) pointer datatype
        // opRemoveInput(op,2); opRemoveInput(op,1); opSetOpcode(op,CPUI_COPY);  -- STUB(W6)
        // opSetInput(op,newconst,0); return true;
        false // STUB(W6): cannot stamp pointer type / change opcode -> no push
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
        // Datatype *sbType = sb->getTypeReadFacing(op);  -- STUB(W6)
        //   The whole guard chain (TYPE_PTR -> TYPE_SPACEBASE -> getAddress / scope
        //   isReadOnly / stringManager->isString) is W6/W4.  The descendant
        //   push-const loop (pushConstFurther) and the COPY conversion are also W6.
        //   Without the read-facing type we cannot proceed.
        let _push = Self::push_const_further
            as fn(&mut Funcdata, OpId, int4, uintb) -> bool;
        if type_read_facing_stub(data, sb, op).is_err() {
            return 0; // STUB(W6)
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
        // RulePushPtr::duplicateNeed(op, data);  -- STUB(W5-sibling/W6): the
        // extension-duplication helper lives in RulePushPtr (a different batch) and
        // creates new ops/outputs (opSetOutput stub).  return 1;
        // We validated the full guard chain with real API; the commit is the stub.
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

    /// Find the base structure or array data-type that `outvn` is part of and the
    /// Varnode's starting offset within it (C++ `RulePieceStructure::determineDatatype`,
    /// ruleaction.cc:7496, including the `Varnode::getStructuredType` mapentry path,
    /// varnode.cc:1156).
    ///
    /// C++ `getStructuredType` prefers the containing Symbol's type
    /// (`mapentry->getSymbol()->getType()`) over the Varnode's own type; we re-derive
    /// the `mapentry` with the local-scope containment query (`query_container_for_link`,
    /// the same lookup `linkSymbol` uses).  When the resolved structured type is wider
    /// than the Varnode (the partial-symbol case) the C++ branch computes
    /// `baseOffset = vn->getAddr().overlap(0, entry->getAddr(), ct->getSize()) +
    /// entry->getOffset()` (== the entry query's `sym_off`) and walks the type tree to
    /// confirm the concrete sub-type is itself structured; otherwise it returns null so
    /// the CONCAT forming a non-structured sub-type is left intact.
    fn determine_datatype(
        data: &Funcdata,
        vn: VarnodeId,
    ) -> KunaResult<(Option<std::rc::Rc<crate::dtype::Datatype>>, int4)> {
        let vnsize = size(data, vn);
        // getStructuredType(): mapentry ? symbol->getType() : vn->getType(); piece-structured else null.
        // Resolve the containing local SymbolEntry (the kuna `mapentry`).
        let usepoint = data.vn_use_point(vn);
        let addr = vn_addr(data, vn);
        let container = data
            .get_scope_local()
            .and_then(|lm| lm.query_container_for_link(&addr, &usepoint));
        // ct = mapentry ? mapentry->getSymbol()->getType() : vn->getType()
        let (ct, base_offset_hint) = match container
            .as_ref()
            .and_then(|c| c.sym_type.as_ref().map(|t| (std::rc::Rc::clone(t), c.sym_off)))
        {
            Some((symty, off)) => (symty, off),
            None => (vn_type(data, vn), 0),
        };
        // if (ct->isPieceStructured()) ... else return null.
        if !ct.is_piece_structured() {
            return Ok((None, 0));
        }

        if ct.get_size() != vnsize {
            // vn is a partial.  baseOffset = overlap(0, entry, ct.size) + entry.offset
            // == the container query's sym_off (which already folds in entry->getOffset()).
            let base_offset = base_offset_hint;
            if base_offset < 0 {
                return Ok((None, 0));
            }
            // Walk to the concrete sub-type that matches the Varnode size.
            let mut sub_type = Some(std::rc::Rc::clone(&ct));
            let mut sub_offset: int8 = base_offset as int8;
            while let Some(st) = &sub_type {
                if st.get_size() <= vnsize {
                    break;
                }
                let (next, off2) = st.get_sub_type(sub_offset)?;
                sub_offset = off2;
                sub_type = next;
            }
            if let Some(st) = &sub_type {
                if st.get_size() == vnsize && sub_offset == 0 {
                    // Concrete sub-type matches the Varnode exactly.
                    if !st.is_piece_structured() {
                        // and the concrete sub-type is not structured itself:
                        // don't split out the CONCAT forming the sub-type.
                        return Ok((None, 0));
                    }
                }
            }
            return Ok((Some(ct), base_offset));
        }
        Ok((Some(ct), 0))
    }

    /// Convert an INT_ZEXT operation to a PIECE with a zero constant as the first
    /// parameter (C++ `RulePieceStructure::convertZextToPiece`).
    fn convert_zext_to_piece(
        data: &mut Funcdata,
        zext: OpId,
        ct: Option<std::rc::Rc<crate::dtype::Datatype>>,
        mut offset: int4,
    ) -> KunaResult<bool> {
        let outvn = out_vn(data, zext);
        let invn = in_vn(data, zext, 0);
        if is_const(data, invn) {
            return Ok(false);
        }
        let sz = size(data, outvn) - size(data, invn);
        // if (sz > sizeof(uintb)) return false;
        if sz as usize > std::mem::size_of::<uintb>() {
            return Ok(false);
        }
        // offset += outvn->getSpace()->isBigEndian() ? 0 : invn->getSize();
        offset += if vn_is_big_endian(data, outvn) { 0 } else { size(data, invn) };
        let mut new_off: int8 = offset as int8;
        // Walk down to the concrete sub-type matching size `sz`.
        let mut cur = ct;
        while let Some(c) = &cur {
            if c.get_size() <= sz {
                break;
            }
            let (sub, off2) = c.get_sub_type(new_off)?;
            new_off = off2;
            cur = sub;
        }
        let zerovn = data.new_constant(sz, 0);
        if let Some(c) = &cur {
            if c.get_size() == sz {
                data.vbank_mut().get_mut(zerovn).expect("convertZextToPiece: stale zerovn").update_type(std::rc::Rc::clone(c));
            }
        }
        data.op_set_opcode_code(zext, OpCode::CPUI_PIECE);
        data.op_insert_input(zext, zerovn, 0)?;
        // invn's union resolution transfer (inheritUnionField) — no unions in the
        // structured-piece corpus; the W8 union surface is a documented stub.
        // STUB(W8 union): inheritUnionField(invn->getType(), zext, 1, zext, 0).
        Ok(true)
    }

    /// Search for leaves in the CONCAT tree that are INT_ZEXT and convert them to
    /// PIECE (C++ `RulePieceStructure::findReplaceZext`).
    fn find_replace_zext(
        data: &mut Funcdata,
        stack: &[crate::op::PieceNode],
        structured_type: &std::rc::Rc<crate::dtype::Datatype>,
    ) -> KunaResult<bool> {
        let mut change = false;
        for node in stack {
            if !node.is_leaf() {
                continue;
            }
            let vn = node.get_varnode(data.obank());
            if !is_written(data, vn) {
                continue;
            }
            let op = def_of(data, vn).expect("findReplaceZext: written vn no def");
            if code(data, op) != OpCode::CPUI_INT_ZEXT {
                continue;
            }
            if !Self::spanning_range(structured_type, node.get_type_offset(), size(data, vn))? {
                continue;
            }
            if Self::convert_zext_to_piece(data, op, Some(std::rc::Rc::clone(structured_type)), node.get_type_offset())? {
                change = true;
            }
        }
        Ok(change)
    }

    /// Return `true` if the given `root` and `leaf` should be part of different
    /// symbols (C++ `RulePieceStructure::separateSymbol`).
    ///
    /// `root->getSymbolEntry() != leaf->getSymbolEntry()` (the first test) is a W4
    /// stub: the merged Varnode has no `mapentry`, so two Varnodes are taken to
    /// share a symbol entry (both null) and the test falls through to the structural
    /// arms.  This matches the concat corpus, where the root and its in-place pieces
    /// are not separately mapped. // STUB(W4 symbol-entry)
    fn separate_symbol(data: &Funcdata, root: VarnodeId, leaf: VarnodeId) -> bool {
        // if (root->getSymbolEntry() != leaf->getSymbolEntry()) return true;  // STUB(W4)
        if is_addr_tied(data, root) {
            return false;
        }
        if !is_written(data, leaf) {
            return true;
        }
        if is_proto_partial(data, leaf) {
            return true; // Already in another tree
        }
        let op = def_of(data, leaf).expect("separateSymbol: written leaf no def");
        if data.obank().get(op).expect("separateSymbol: stale op").is_marker() {
            return true; // Leaf is not defined locally
        }
        if code(data, op) != OpCode::CPUI_PIECE {
            return false;
        }
        if vn_type(data, leaf).is_piece_structured() {
            return true; // Would be a separate root
        }
        false
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
        // if (op->isPartialRoot()) return 0;  // Check if CONCAT tree already visited
        if data.obank().get(op).expect("piecestructure: stale op").is_partial_root() {
            return 0;
        }
        let outvn = out_vn(data, op);
        // Datatype *ct = determineDatatype(outvn, baseOffset);
        let (ct, base_offset) = match Self::determine_datatype(data, outvn) {
            Ok((Some(c), b)) => (c, b),
            Ok((None, _)) => return 0,
            Err(_) => return 0,
        };

        if code(data, op) == OpCode::CPUI_INT_ZEXT {
            // convertZextToPiece(op, outvn->getType(), 0, data)
            let outty = vn_type(data, outvn);
            match Self::convert_zext_to_piece(data, op, Some(outty), 0) {
                Ok(true) => return 1,
                _ => return 0,
            }
        }

        // Check if outvn is really the root of the tree.
        if let Some(zext) = lone_descend(data, outvn) {
            let zc = code(data, zext);
            if zc == OpCode::CPUI_PIECE {
                return 0; // More PIECEs below us, not a root
            }
            if zc == OpCode::CPUI_INT_ZEXT {
                // Extension of a structured data-type; convert extension to PIECE first
                let zout = out_vn(data, zext);
                let zoutty = vn_type(data, zout);
                match Self::convert_zext_to_piece(data, zext, Some(zoutty), 0) {
                    Ok(true) => return 1,
                    _ => return 0,
                }
            }
        }

        // Build the CONCAT tree, replacing INT_ZEXT leaves with PIECE as needed.
        // The Symbol-entry resolver gives `PieceNode::isLeaf` the C++
        // `getSymbolEntry()` comparison (a mapped piece sharing the root's entry is
        // not a leaf — so a CONCAT into the same stack symbol is split per element).
        // The resolver borrows `data` immutably, so the gather runs in an inner
        // scope that drops it before the `&mut data` `findReplaceZext`.
        let mut stack: Vec<crate::op::PieceNode> = Vec::new();
        loop {
            stack.clear();
            {
                let entry_of = |q: VarnodeId| data.vn_container_entry_key(q);
                crate::op::gather_pieces_inner(
                    &mut stack,
                    data.obank(),
                    data.vbank(),
                    outvn,
                    op,
                    base_offset,
                    base_offset,
                    Some(&entry_of),
                );
            }
            match Self::find_replace_zext(data, &stack, &ct) {
                Ok(true) => continue, // found some; regenerate the tree
                Ok(false) => break,
                Err(_) => break,
            }
        }

        // op->setPartialRoot();
        data.obank_mut().get_mut(op).expect("piecestructure: stale op").set_partial_root();
        let mut any_addr_tied = is_addr_tied(data, outvn);
        // Address baseAddr = outvn->getAddr() - baseOffset;
        let base_addr = &vn_addr(data, outvn) - base_offset as i64;

        for node in stack.clone() {
            let vn = node.get_varnode(data.obank());
            // Address addr = baseAddr + node.getTypeOffset();  addr.renormalize(vn->getSize());
            let mut addr = &base_addr + node.get_type_offset() as i64;
            let vnsize = size(data, vn);
            if addr.renormalize(vnsize, data.get_arch().manage()).is_err() {
                continue;
            }
            // C++ `if (vn->getAddr() == addr) { if (!isLeaf || !separateSymbol) {...} }`
            // — the two guards are independent and the inner block `continue`s, so
            // the falls-through-to-leaf-handling structure is preserved by `&&`.
            if vn_addr(data, vn) == addr
                && (!node.is_leaf() || !Self::separate_symbol(data, outvn, vn))
            {
                // Varnode already has correct address and same symbol as root.
                if !is_addr_tied(data, vn) && !is_proto_partial(data, vn) {
                    data.vbank_mut().get_mut(vn).expect("piecestructure: stale vn").set_proto_partial();
                }
                any_addr_tied = any_addr_tied || is_addr_tied(data, vn);
                continue;
            }
            if node.is_leaf() {
                // Insert a COPY into a Varnode at the correct storage.
                let node_op = node.get_op();
                let copy_op = data.new_op(1, op_addr(data, node_op));
                let new_vn = match data.new_varnode_out(vnsize, &addr, copy_op) {
                    Ok(v) => v,
                    Err(_) => continue,
                };
                any_addr_tied = any_addr_tied || is_addr_tied(data, new_vn);
                // Datatype *newType = types->getExactPiece(ct, node.getTypeOffset(), vn->getSize());
                let new_type = match data
                    .get_arch()
                    .types()
                    .and_then(|t| t.get_exact_piece(std::rc::Rc::clone(&ct), node.get_type_offset(), vnsize).ok().flatten())
                {
                    Some(t) => t,
                    None => vn_type(data, vn),
                };
                data.vbank_mut().get_mut(new_vn).expect("piecestructure: stale newVn").update_type(std::rc::Rc::clone(&new_type));
                data.op_set_opcode_code(copy_op, OpCode::CPUI_COPY);
                if data.op_set_input(copy_op, vn, 0).is_err() {
                    continue;
                }
                if data.op_set_input(node_op, new_vn, node.get_slot()).is_err() {
                    continue;
                }
                data.op_insert_before(copy_op, node_op);
                // Union resolution transfer is a W8 stub (no unions in the corpus).
                // STUB(W8 union): inheritUnionField / resolveInFlow on newType.
                if !is_addr_tied(data, new_vn) {
                    data.vbank_mut().get_mut(new_vn).expect("piecestructure: stale newVn").set_proto_partial();
                }
            } else {
                // vn is NOT addrtied and has a lone descendant; replace storage.
                let def_op = match def_of(data, vn) {
                    Some(d) => d,
                    None => continue,
                };
                let lone_op = match lone_descend(data, vn) {
                    Some(l) => l,
                    None => continue,
                };
                let slot = slot_of(data, lone_op, vn);
                let vnty = vn_type(data, vn);
                let new_vn = data.new_varnode(vnsize, &addr, Some(vnty));
                if data.op_set_output(def_op, new_vn).is_err() {
                    continue;
                }
                if data.op_set_input(lone_op, new_vn, slot).is_err() {
                    continue;
                }
                if data.delete_varnode(vn).is_err() {
                    continue;
                }
                if !is_addr_tied(data, new_vn) {
                    data.vbank_mut().get_mut(new_vn).expect("piecestructure: stale newVn").set_proto_partial();
                }
            }
        }
        if !any_addr_tied {
            data.with_covermerge(|merge, data| merge.register_proto_partial_root(data, outvn));
        }
        1
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
                if set_opcode_typed(data, newop, OpCode::CPUI_SUBPIECE).is_err() {
                    return 0;
                }
                let newout = match new_unique_out_typed(data, trunc_size, newop) {
                    Ok(v) => v,
                    Err(_) => return 0,
                };
                data.op_set_input(newop, a, 0).expect("subnormal: opSetInput");
                let cst = data.new_constant(4, c as uintb);
                data.op_set_input(newop, cst, 1).expect("subnormal: opSetInput");
                data.op_insert_before(newop, op);

                data.op_set_input(op, newout, 0).expect("subnormal: opSetInput");
                data.op_remove_input(op, 1);
                if set_opcode_typed(data, op, opc).is_err() {
                    return 0;
                }
                return 1;
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

        let newop = data.new_op(2, op_addr(data, op));
        if set_opcode_typed(data, newop, OpCode::CPUI_SUBPIECE).is_err() {
            return 0;
        }
        let newout = match new_unique_out_typed(data, outsize, newop) {
            Ok(v) => v,
            Err(_) => return 0,
        };
        data.op_set_input(newop, a, 0).expect("subnormal: opSetInput");
        let cst = data.new_constant(4, c as uintb);
        data.op_set_input(newop, cst, 1).expect("subnormal: opSetInput");
        data.op_insert_before(newop, op);

        data.op_set_input(op, newout, 0).expect("subnormal: opSetInput");
        let nc = data.new_constant(4, n as uintb);
        data.op_set_input(op, nc, 1).expect("subnormal: opSetInput");
        if set_opcode_typed(data, op, opc).is_err() {
            return 0;
        }
        1
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
        // data.opSetOpcode(op, opc); return 1;  -- STUB(W6)
        if set_opcode_typed(data, op, opc).is_err() {
            return 0; // STUB(W6)
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
        let mut mult_const = match is_constant_extended(data, in_vn(data, multop, 1)) {
            Some(v) => v,
            None => return 0, // STUB(W3-varnode): isConstantExtended unavailable
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
        let _ = subop;
        let mut shiftopc = shiftopc;
        for addop in data.descend_snapshot(opout) {
            if code(data, addop) != OpCode::CPUI_INT_ADD {
                continue;
            }
            if in_vn(data, addop, 0) != x && in_vn(data, addop, 1) != x {
                continue;
            }

            // Construct the new constant
            let new_const_vn =
                match new_extended_constant(data, size(data, extvn), &mult_const, op) {
                    Ok(v) => v,
                    Err(_) => return 0,
                };

            // Construct the new multiply
            let newmultop = data.new_op(2, op_addr(data, op));
            if set_opcode_typed(data, newmultop, OpCode::CPUI_INT_MULT).is_err() {
                return 0;
            }
            let newmultvn = match new_unique_out_typed(data, size(data, extvn), newmultop) {
                Ok(v) => v,
                Err(_) => return 0,
            };
            data.op_set_input(newmultop, extvn, 0).expect("RuleDivTermAdd: opSetInput");
            data.op_set_input(newmultop, new_const_vn, 1).expect("RuleDivTermAdd: opSetInput");
            data.op_insert_before(newmultop, op);

            let newshiftop = data.new_op(2, op_addr(data, op));
            if shiftopc == OpCode::CPUI_MAX {
                shiftopc = OpCode::CPUI_INT_RIGHT;
            }
            if set_opcode_typed(data, newshiftop, shiftopc).is_err() {
                return 0;
            }
            let newshiftvn = match new_unique_out_typed(data, size(data, extvn), newshiftop) {
                Ok(v) => v,
                Err(_) => return 0,
            };
            data.op_set_input(newshiftop, newmultvn, 0).expect("RuleDivTermAdd: opSetInput");
            let nc = data.new_constant(4, n as uintb);
            data.op_set_input(newshiftop, nc, 1).expect("RuleDivTermAdd: opSetInput");
            data.op_insert_before(newshiftop, op);

            if set_opcode_typed(data, addop, OpCode::CPUI_SUBPIECE).is_err() {
                return 0;
            }
            data.op_set_input(addop, newshiftvn, 0).expect("RuleDivTermAdd: opSetInput");
            let c0 = data.new_constant(4, 0);
            data.op_set_input(addop, c0, 1).expect("RuleDivTermAdd: opSetInput");
            return 1;
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
        let mut mult_const = match is_constant_extended(data, in_vn(data, multop, 1)) {
            Some(v) => v,
            None => return 0, // STUB(W3-varnode)
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
            // and rewrite addop to SUBPIECE(...,0).
            let mut pow: [u64; 2] = [0, 0];
            set_u128(&mut pow, 1);
            let mut shifted = [0u64; 2];
            leftshift128(&pow, &mut shifted, n); // Calculate 2^n
            let mut sum = [0u64; 2];
            add128(&mult_const, &shifted, &mut sum); // multConst = multConst + 2^n
            mult_const = sum;

            let newmultop = data.new_op(2, op_addr(data, op));
            if set_opcode_typed(data, newmultop, OpCode::CPUI_INT_MULT).is_err() {
                return 0;
            }
            let newmultvn = match new_unique_out_typed(data, size(data, zextvn), newmultop) {
                Ok(v) => v,
                Err(_) => return 0,
            };
            data.op_set_input(newmultop, zextvn, 0).expect("RuleDivTermAdd2: opSetInput");
            let new_const_vn =
                match new_extended_constant(data, size(data, zextvn), &mult_const, op) {
                    Ok(v) => v,
                    Err(_) => return 0,
                };
            data.op_set_input(newmultop, new_const_vn, 1).expect("RuleDivTermAdd2: opSetInput");
            data.op_insert_before(newmultop, op);

            let newshiftop = data.new_op(2, op_addr(data, op));
            if set_opcode_typed(data, newshiftop, OpCode::CPUI_INT_RIGHT).is_err() {
                return 0;
            }
            let newshiftvn = match new_unique_out_typed(data, size(data, zextvn), newshiftop) {
                Ok(v) => v,
                Err(_) => return 0,
            };
            data.op_set_input(newshiftop, newmultvn, 0).expect("RuleDivTermAdd2: opSetInput");
            let nc = data.new_constant(4, (n + 1) as uintb);
            data.op_set_input(newshiftop, nc, 1).expect("RuleDivTermAdd2: opSetInput");
            data.op_insert_before(newshiftop, op);

            if set_opcode_typed(data, addop, OpCode::CPUI_SUBPIECE).is_err() {
                return 0;
            }
            data.op_set_input(addop, newshiftvn, 0).expect("RuleDivTermAdd2: opSetInput");
            let c0 = data.new_constant(4, 0);
            data.op_set_input(addop, c0, 1).expect("RuleDivTermAdd2: opSetInput");
            return 1;
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
    /// Read-only except for the W3-varnode `isConstantExtended` helper (the only
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
        // C++ GUARD A (ruleaction.cc:8113): `if (!inVn->isWritten()) return 0;`
        // applies to getIn(0) BEFORE the isConstantExtended branch.  It rejects
        // both a free getIn(0) (so the unguarded `inVn->getDef()` at :8122 is
        // never reached) and a bare-constant getIn(0) (a constant is never
        // isWritten, so C++ declines rather than treating the constant as `y`).
        if !is_written(data, in_vn_cur) {
            return None;
        }
        let y: [u64; 2];
        if let Some(v) = is_constant_extended(data, in_vn_cur) {
            y = v;
            in_vn_cur = in_vn(data, cur_op, 1);
            if !is_written(data, in_vn_cur) {
                return None;
            }
        } else if let Some(v) = is_constant_extended(data, in_vn(data, cur_op, 1)) {
            y = v;
        } else {
            return None; // There MUST be a constant (STUB(W3): isConstantExtended)
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
        let (mut in_vn0, n, y, mut xsize, ext_opc) = match Self::find_form(data, op) {
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
        let mut out_size = size(data, out_vn(data, op));

        // `op` may be reassigned to a freshly-built INT_ADD in the truncation
        // branch (the original op becomes the SUBPIECE); track it locally.
        let mut op = op;

        if size(data, in_vn0) < out_size {
            // Do we need an extension to get to final size
            let in_ext = data.new_op(1, op_addr(data, op));
            if set_opcode_typed(data, in_ext, ext_opc).is_err() {
                return 0;
            }
            let ext_out = match new_unique_out_typed(data, out_size, in_ext) {
                Ok(v) => v,
                Err(_) => return 0,
            };
            data.op_set_input(in_ext, in_vn0, 0).expect("RuleDivOpt: opSetInput");
            in_vn0 = ext_out;
            data.op_insert_before(in_ext, op);
        } else if size(data, in_vn0) > out_size {
            // Do we need a truncation to get to final size.  Create a new op to
            // hold the INT_DIV or INT_SDIV:INT_ADD; the original op becomes a
            // truncation SUBPIECE.
            let newop = data.new_op(2, op_addr(data, op));
            // This gets changed immediately, but need it for opInsert
            if set_opcode_typed(data, newop, OpCode::CPUI_INT_ADD).is_err() {
                return 0;
            }
            let res_vn = match new_unique_out_typed(data, size(data, in_vn0), newop) {
                Ok(v) => v,
                Err(_) => return 0,
            };
            data.op_insert_before(newop, op);
            if set_opcode_typed(data, op, OpCode::CPUI_SUBPIECE).is_err() {
                return 0;
            }
            data.op_set_input(op, res_vn, 0).expect("RuleDivOpt: opSetInput");
            let c0 = data.new_constant(4, 0);
            data.op_set_input(op, c0, 1).expect("RuleDivOpt: opSetInput");
            op = newop; // Main transform now changes newop
            out_size = size(data, in_vn0);
        }

        if ext_opc == OpCode::CPUI_INT_ZEXT {
            // Unsigned division
            data.op_set_input(op, in_vn0, 0).expect("RuleDivOpt: opSetInput");
            let dvn = data.new_constant(out_size, divisor);
            data.op_set_input(op, dvn, 1).expect("RuleDivOpt: opSetInput");
            if set_opcode_typed(data, op, OpCode::CPUI_INT_DIV).is_err() {
                return 0;
            }
        } else {
            // Sign division
            let opout = out_vn(data, op);
            Self::move_sign_bit_extraction(data, opout, in_vn0);
            let divop = data.new_op(2, op_addr(data, op));
            if set_opcode_typed(data, divop, OpCode::CPUI_INT_SDIV).is_err() {
                return 0;
            }
            let newout = match new_unique_out_typed(data, out_size, divop) {
                Ok(v) => v,
                Err(_) => return 0,
            };
            data.op_set_input(divop, in_vn0, 0).expect("RuleDivOpt: opSetInput");
            let dvn = data.new_constant(out_size, divisor);
            data.op_set_input(divop, dvn, 1).expect("RuleDivOpt: opSetInput");
            data.op_insert_before(divop, op);
            // Build the sign value correction
            let sgnop = data.new_op(2, op_addr(data, op));
            if set_opcode_typed(data, sgnop, OpCode::CPUI_INT_SRIGHT).is_err() {
                return 0;
            }
            let sgnvn = match new_unique_out_typed(data, out_size, sgnop) {
                Ok(v) => v,
                Err(_) => return 0,
            };
            data.op_set_input(sgnop, in_vn0, 0).expect("RuleDivOpt: opSetInput");
            let shc = data.new_constant(out_size, (out_size * 8 - 1) as uintb);
            data.op_set_input(sgnop, shc, 1).expect("RuleDivOpt: opSetInput");
            data.op_insert_before(sgnop, op);
            // Add the correction into the division op
            data.op_set_input(op, newout, 0).expect("RuleDivOpt: opSetInput");
            data.op_set_input(op, sgnvn, 1).expect("RuleDivOpt: opSetInput");
            if set_opcode_typed(data, op, OpCode::CPUI_INT_ADD).is_err() {
                return 0;
            }
        }
        1
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
