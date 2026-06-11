//! Port of `decompiler/cpp/ruleaction.cc` lines 8374-9803 — the 15 Rule classes
//! `RuleSignDiv2` .. `RuleIgnoreNan` (W5, item `w5-s3-rules-7`).
//!
//! Each class is transcribed as a unit struct implementing
//! [`Rule`](crate::action::Rule).  The `name()` strings, `getOpList` contents and
//! `applyOp` body structure are exact (rule bodies are the decompiler's output).
//! Static helpers a class owns are ported as free `fn`s next to that class; the
//! one helper shared across batches (`RuleSignMod2nOpt::check_sign_extraction`,
//! used by `RuleSignMod2Opt`) is `pub` and lives where the C++ defines it.
//!
//! # Seams (W3-output / W4 / W6)
//!
//! Several rules reach surfaces that have not yet been ported and that this item
//! has no editing rights over:
//!
//! * **Output creation** (`newUniqueOut`/`newVarnodeOut`, which route through
//!   `Funcdata::opSetOutput`).  `opSetOutput` currently returns a seam `Err`
//!   (`funcdata_op.rs`: it needs a `banks_mut()` split-borrow for
//!   `vbank.setDef`).  Rules that fabricate a *new* output Varnode
//!   (`RuleSignNearMult`, `RulePtrFlow::truncatePointer`) match fully but bail at
//!   the construction step.  // SEAM(W3-output)
//! * **`glb->inst[opc]`** — the `OpCode`→`TypeOp` resolver (W6).  The
//!   [`typeop_for`] helper below is the local stand-in; it carries the exact
//!   `opflags` from `typeop.cc` so downstream `isBoolOutput`/eval-type checks
//!   stay faithful, but the `OpBehavior`/`addlflags` (e.g.
//!   `isFloatingPointOp()`) are *not* modelled.  // SEAM(W6)
//! * **W4 subsystems** — `RuleSegment` needs `userops.getSegmentOp`,
//!   `SegmentOp::execute`, `getSpaceFromConst`, and the `contiguous_test`/
//!   `findContiguousWhole` expression helpers; `RulePtrFlow` needs the
//!   `Architecture` (`glb`), `getDefaultCodeSpace`, and `isTruncated`;
//!   `RuleIgnoreNan` needs `nan_ignore_all`, `TypeOp::isFloatingPointOp`, and
//!   `functionalEquality`; `RuleConditionalMove` needs `opBoolNegate` and the
//!   `CloneBlockOps` expression cloner.  These match as far as the available IR
//!   surface allows and bail at the unported step.  // SEAM(W4)
//!
//! Every seam is also recorded in this item's `losses` output.

use kuna_base::address::{calc_mask, mostsigbit_set, popcount, signbit_negative};
use kuna_base::types::{int4, uintb};
use kuna_num::opcodes::OpCode;

use crate::action::{ActionGroupList, Rule, RuleSpec};
use crate::funcdata::Funcdata;
use crate::op::pcodeop_flags;
use crate::seams::{BlockId, OpId, TypeOp, VarnodeId};

// =============================================================================
// SEAM(W6): glb->inst[opc] resolver
// =============================================================================

/// Resolve an [`OpCode`] to the singleton `TypeOp` the architecture caches in
/// its `inst` table (C++ `glb->inst[opc]`, consumed by `Funcdata::opSetOpcode`).
///
/// The real resolver lives in W6 (`typeop`).  Until it lands, this carries the
/// exact `opflags` from `typeop.cc` for the op-codes these rules produce, so the
/// op's cached property bits (eval-type, `booloutput`) match what the C++ would
/// install.  The `addlflags`/`OpBehavior` are *not* modelled (a rule that wrote
/// e.g. INT_SDIV does not gain its `arithmetic_op` addlflag here).  // SEAM(W6)
fn typeop_for(opc: OpCode) -> TypeOp {
    use pcodeop_flags as f;
    // opflags transcribed verbatim from typeop.cc constructors.
    let (flags, name): (u32, &str) = match opc {
        OpCode::CPUI_COPY => (f::unary | f::nocollapse, "copy"),
        OpCode::CPUI_INT_SRIGHT => (f::binary, ">>"),
        OpCode::CPUI_INT_SDIV => (f::binary, "/"),
        OpCode::CPUI_INT_REM => (f::binary, "%"),
        OpCode::CPUI_INT_SREM => (f::binary, "%"),
        OpCode::CPUI_INT_MULT => (f::binary | f::commutative, "*"),
        OpCode::CPUI_INT_ZEXT => (f::unary, "ZEXT"),
        OpCode::CPUI_BOOL_NEGATE => (f::unary | f::booloutput, "!"),
        OpCode::CPUI_BOOL_AND => (f::binary | f::commutative | f::booloutput, "&&"),
        OpCode::CPUI_BOOL_OR => (f::binary | f::commutative | f::booloutput, "||"),
        OpCode::CPUI_FLOAT_INT2FLOAT => (f::unary, "INT2FLOAT"),
        // Any op-code not enumerated above is not produced by this batch; fall
        // back to a flagless skeleton (the seam will be widened in W6).
        _ => (0, "op"),
    };
    TypeOp::new(opc, flags, name)
}

// =============================================================================
// Small read-only accessors over Funcdata (keep the rule bodies faithful)
// =============================================================================

/// `op->code()`
#[inline]
fn op_code(data: &Funcdata, op: OpId) -> OpCode {
    data.obank().get(op).expect("op_code: stale op").code()
}
/// `op->getIn(slot)`
#[inline]
fn op_in(data: &Funcdata, op: OpId, slot: int4) -> Option<VarnodeId> {
    data.obank().get(op).expect("op_in: stale op").get_in(slot)
}
/// `op->getOut()`
#[inline]
fn op_out(data: &Funcdata, op: OpId) -> Option<VarnodeId> {
    data.obank().get(op).expect("op_out: stale op").get_out()
}
/// `op->numInput()`
#[inline]
fn op_num_input(data: &Funcdata, op: OpId) -> int4 {
    data.obank().get(op).expect("op_num_input: stale op").num_input()
}
/// `op->getSlot(vn)`
#[inline]
fn op_slot(data: &Funcdata, op: OpId, vn: VarnodeId) -> int4 {
    data.obank().get(op).expect("op_slot: stale op").get_slot(vn)
}
/// `vn->isConstant()`
#[inline]
fn vn_is_constant(data: &Funcdata, vn: VarnodeId) -> bool {
    data.vbank().get(vn).expect("vn_is_constant: stale vn").is_constant()
}
/// `vn->getOffset()`
#[inline]
fn vn_offset(data: &Funcdata, vn: VarnodeId) -> uintb {
    data.vbank().get(vn).expect("vn_offset: stale vn").get_offset()
}
/// `vn->getSize()`
#[inline]
fn vn_size(data: &Funcdata, vn: VarnodeId) -> int4 {
    data.vbank().get(vn).expect("vn_size: stale vn").get_size()
}
/// `vn->isWritten()`
#[inline]
fn vn_is_written(data: &Funcdata, vn: VarnodeId) -> bool {
    data.vbank().get(vn).expect("vn_is_written: stale vn").is_written()
}
/// `vn->isFree()`
#[inline]
fn vn_is_free(data: &Funcdata, vn: VarnodeId) -> bool {
    data.vbank().get(vn).expect("vn_is_free: stale vn").is_free()
}
/// `vn->getDef()`
#[inline]
fn vn_def(data: &Funcdata, vn: VarnodeId) -> Option<OpId> {
    data.vbank().get(vn).expect("vn_def: stale vn").get_def()
}
/// `vn->beginDescend()..endDescend()` snapshotted to a Vec (rules mutate during
/// iteration; the C++ holds a live list iterator — collecting first is the
/// arena-safe equivalent that preserves descend-list order).
#[inline]
fn vn_descend(data: &Funcdata, vn: VarnodeId) -> Vec<OpId> {
    data.vbank().get(vn).expect("vn_descend: stale vn").descend_iter().collect()
}

// =============================================================================
// RuleSignDiv2 (ruleaction.cc:8374)
// =============================================================================

/// `RuleSignDiv2` — recognize the signed-divide-by-2 form and collapse it to
/// INT_SDIV by 2.
pub struct RuleSignDiv2;

impl RuleSignDiv2 {
    /// Constructor (C++ `RuleSignDiv2(g)`), name "signdiv2".
    pub fn new() -> RuleSignDiv2 {
        RuleSignDiv2
    }
}

impl Default for RuleSignDiv2 {
    fn default() -> Self {
        RuleSignDiv2::new()
    }
}

impl Rule for RuleSignDiv2 {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_INT_SRIGHT]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if grouplist.contains("analysis") {
            Some(Box::new(RuleSignDiv2))
        } else {
            None
        }
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // if (!op->getIn(1)->isConstant()) return 0;
        let in1 = op_in(data, op, 1).expect("signdiv2: missing in1");
        if !vn_is_constant(data, in1) {
            return 0;
        }
        // if (op->getIn(1)->getOffset() != 1) return 0;
        if vn_offset(data, in1) != 1 {
            return 0;
        }
        // addout = op->getIn(0);
        let addout = op_in(data, op, 0).expect("signdiv2: missing in0");
        if !vn_is_written(data, addout) {
            return 0;
        }
        let addop = vn_def(data, addout).expect("signdiv2: addout written");
        if op_code(data, addop) != OpCode::CPUI_INT_ADD {
            return 0;
        }
        let mut i: int4 = 0;
        let mut a: Option<VarnodeId> = None;
        while i < 2 {
            let multout = op_in(data, addop, i).expect("signdiv2: addop in");
            if !vn_is_written(data, multout) {
                i += 1;
                continue;
            }
            let multop = vn_def(data, multout).expect("signdiv2: multout written");
            if op_code(data, multop) != OpCode::CPUI_INT_MULT {
                i += 1;
                continue;
            }
            let m1 = op_in(data, multop, 1).expect("signdiv2: multop in1");
            if !vn_is_constant(data, m1) {
                i += 1;
                continue;
            }
            // if (multop->getIn(1)->getOffset() != calc_mask(multop->getIn(1)->getSize()))
            if vn_offset(data, m1) != calc_mask(vn_size(data, m1)) {
                i += 1;
                continue;
            }
            let shiftout = op_in(data, multop, 0).expect("signdiv2: multop in0");
            if !vn_is_written(data, shiftout) {
                i += 1;
                continue;
            }
            let shiftop = vn_def(data, shiftout).expect("signdiv2: shiftout written");
            if op_code(data, shiftop) != OpCode::CPUI_INT_SRIGHT {
                i += 1;
                continue;
            }
            let s1 = op_in(data, shiftop, 1).expect("signdiv2: shiftop in1");
            if !vn_is_constant(data, s1) {
                i += 1;
                continue;
            }
            // int4 n = shiftop->getIn(1)->getOffset();
            let n = vn_offset(data, s1) as int4;
            let cand = op_in(data, shiftop, 0).expect("signdiv2: shiftop in0");
            // if (a != addop->getIn(1-i)) continue;
            if cand != op_in(data, addop, 1 - i).expect("signdiv2: addop 1-i") {
                i += 1;
                continue;
            }
            // if (n != 8*a->getSize() - 1) continue;
            if n != 8 * vn_size(data, cand) - 1 {
                i += 1;
                continue;
            }
            if vn_is_free(data, cand) {
                i += 1;
                continue;
            }
            a = Some(cand);
            break;
        }
        if i == 2 {
            return 0;
        }
        let a = a.expect("signdiv2: a set when i<2");

        // data.opSetInput(op,a,0);
        data.op_set_input(op, a, 0).expect("signdiv2: opSetInput a");
        // data.opSetInput(op,data.newConstant(a->getSize(),2),1);
        let asize = vn_size(data, a);
        let two = data.new_constant(asize, 2);
        data.op_set_input(op, two, 1).expect("signdiv2: opSetInput 2");
        // data.opSetOpcode(op,CPUI_INT_SDIV);
        data.op_set_opcode(op, typeop_for(OpCode::CPUI_INT_SDIV));
        1
    }
}

// =============================================================================
// RuleDivChain (ruleaction.cc:8427)
// =============================================================================

/// `RuleDivChain` — collapse two consecutive divisions:
/// `(x / c1) / c2  =>  x / (c1*c2)`.
pub struct RuleDivChain;

impl RuleDivChain {
    /// Constructor (C++ `RuleDivChain(g)`), name "divchain".
    pub fn new() -> RuleDivChain {
        RuleDivChain
    }
}

impl Default for RuleDivChain {
    fn default() -> Self {
        RuleDivChain::new()
    }
}

impl Rule for RuleDivChain {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_INT_DIV, OpCode::CPUI_INT_SDIV]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if grouplist.contains("analysis") {
            Some(Box::new(RuleDivChain))
        } else {
            None
        }
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // OpCode opc2 = op->code();
        let opc2 = op_code(data, op);
        // Varnode *constVn2 = op->getIn(1);
        let const_vn2 = op_in(data, op, 1).expect("divchain: in1");
        if !vn_is_constant(data, const_vn2) {
            return 0;
        }
        // Varnode *vn = op->getIn(0);
        let vn = op_in(data, op, 0).expect("divchain: in0");
        if !vn_is_written(data, vn) {
            return 0;
        }
        // PcodeOp *divOp = vn->getDef();
        let div_op = vn_def(data, vn).expect("divchain: vn written");
        // OpCode opc1 = divOp->code();
        let opc1 = op_code(data, div_op);
        // if (opc1 != opc2 && (opc2 != CPUI_INT_DIV || opc1 != CPUI_INT_RIGHT)) return 0;
        if opc1 != opc2 && (opc2 != OpCode::CPUI_INT_DIV || opc1 != OpCode::CPUI_INT_RIGHT) {
            return 0;
        }
        // Varnode *constVn1 = divOp->getIn(1);
        let const_vn1 = op_in(data, div_op, 1).expect("divchain: divOp in1");
        if !vn_is_constant(data, const_vn1) {
            return 0;
        }
        // if (vn->loneDescend() == 0) return 0;
        if data.lone_descend(vn).is_none() {
            return 0;
        }
        // uintb val1;
        let val1: uintb = if opc1 == opc2 {
            vn_offset(data, const_vn1)
        } else {
            // Unsigned case with INT_RIGHT: int4 sa = constVn1->getOffset(); val1 = 1 << sa;
            let sa = vn_offset(data, const_vn1) as int4;
            let mut v: uintb = 1;
            v <<= sa;
            v
        };
        // Varnode *baseVn = divOp->getIn(0);
        let base_vn = op_in(data, div_op, 0).expect("divchain: divOp in0");
        if vn_is_free(data, base_vn) {
            return 0;
        }
        // int4 sz = vn->getSize();
        let sz = vn_size(data, vn);
        // uintb val2 = constVn2->getOffset();
        let val2 = vn_offset(data, const_vn2);
        // uintb resval = (val1 * val2) & calc_mask(sz);
        let resval = val1.wrapping_mul(val2) & calc_mask(sz);
        if resval == 0 {
            return 0;
        }
        // if (signbit_negative(val1, sz)) val1 = (~val1 + 1) & calc_mask(sz);
        let mut val1 = val1;
        if signbit_negative(val1, sz) {
            val1 = (!val1).wrapping_add(1) & calc_mask(sz);
        }
        // if (signbit_negative(val2, sz)) val2 = (~val2 + 1) & calc_mask(sz);
        let mut val2 = val2;
        if signbit_negative(val2, sz) {
            val2 = (!val2).wrapping_add(1) & calc_mask(sz);
        }
        // int4 bitcount = mostsigbit_set(val1) + mostsigbit_set(val2) + 2;
        let bitcount = mostsigbit_set(val1) + mostsigbit_set(val2) + 2;
        // if (opc2 == CPUI_INT_DIV && bitcount > sz * 8 ) return 0;
        if opc2 == OpCode::CPUI_INT_DIV && bitcount > sz * 8 {
            return 0;
        }
        // if (opc2 == CPUI_INT_SDIV && bitcount > sz * 8 - 2) return 0;
        if opc2 == OpCode::CPUI_INT_SDIV && bitcount > sz * 8 - 2 {
            return 0;
        }
        // data.opSetInput(op, baseVn, 0);
        data.op_set_input(op, base_vn, 0).expect("divchain: opSetInput base");
        // data.opSetInput(op,data.newConstant(sz, resval), 1);
        let c = data.new_constant(sz, resval);
        data.op_set_input(op, c, 1).expect("divchain: opSetInput resval");
        1
    }
}

// =============================================================================
// RuleSignForm (ruleaction.cc:8480)
// =============================================================================

/// `RuleSignForm` — normalize sign extraction:
/// `sub(sext(V),c)  =>  V s>> 31`.
pub struct RuleSignForm;

impl RuleSignForm {
    /// Constructor (C++ `RuleSignForm(g)`), name "signform".
    pub fn new() -> RuleSignForm {
        RuleSignForm
    }
}

impl Default for RuleSignForm {
    fn default() -> Self {
        RuleSignForm::new()
    }
}

impl Rule for RuleSignForm {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_SUBPIECE]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if grouplist.contains("analysis") {
            Some(Box::new(RuleSignForm))
        } else {
            None
        }
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // sextout = op->getIn(0);
        let sextout = op_in(data, op, 0).expect("signform: in0");
        if !vn_is_written(data, sextout) {
            return 0;
        }
        let sextop = vn_def(data, sextout).expect("signform: sextout written");
        if op_code(data, sextop) != OpCode::CPUI_INT_SEXT {
            return 0;
        }
        let a = op_in(data, sextop, 0).expect("signform: sextop in0");
        // int4 c = op->getIn(1)->getOffset();
        let c = vn_offset(data, op_in(data, op, 1).expect("signform: in1")) as int4;
        if c < vn_size(data, a) {
            return 0;
        }
        if vn_is_free(data, a) {
            return 0;
        }

        // data.opSetInput(op,a,0);
        data.op_set_input(op, a, 0).expect("signform: opSetInput a");
        // int4 n = 8*a->getSize()-1;
        let n = 8 * vn_size(data, a) - 1;
        // data.opSetInput(op,data.newConstant(4,n),1);
        let cn = data.new_constant(4, n as uintb);
        data.op_set_input(op, cn, 1).expect("signform: opSetInput n");
        // data.opSetOpcode(op,CPUI_INT_SRIGHT);
        data.op_set_opcode(op, typeop_for(OpCode::CPUI_INT_SRIGHT));
        1
    }
}

// =============================================================================
// RuleSignForm2 (ruleaction.cc:8514)
// =============================================================================

/// `RuleSignForm2` — normalize sign extraction:
/// `sub(sext(V) * small,c) s>> 31  =>  V s>> 31`.
///
/// Note (faithful): the C++ `applyOp` does its matching, calls
/// `data.opSetInput(op, a, 0)`, then **returns 0** — the input set is not
/// reported as a change.  Transcribed exactly, including the trailing `return 0`.
pub struct RuleSignForm2;

impl RuleSignForm2 {
    /// Constructor (C++ `RuleSignForm2(g)`), name "signform2".
    pub fn new() -> RuleSignForm2 {
        RuleSignForm2
    }
}

impl Default for RuleSignForm2 {
    fn default() -> Self {
        RuleSignForm2::new()
    }
}

impl Rule for RuleSignForm2 {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_INT_SRIGHT]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if grouplist.contains("analysis") {
            Some(Box::new(RuleSignForm2))
        } else {
            None
        }
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // Varnode *constVn = op->getIn(1);
        let const_vn = op_in(data, op, 1).expect("signform2: in1");
        if !vn_is_constant(data, const_vn) {
            return 0;
        }
        // Varnode *inVn = op->getIn(0);
        let in_vn = op_in(data, op, 0).expect("signform2: in0");
        // int4 sizeout = inVn->getSize();
        let sizeout = vn_size(data, in_vn);
        // if ((int4)constVn->getOffset() != sizeout*8 -1) return 0;
        if (vn_offset(data, const_vn) as int4) != sizeout * 8 - 1 {
            return 0;
        }
        if !vn_is_written(data, in_vn) {
            return 0;
        }
        // PcodeOp *subOp = inVn->getDef();
        let sub_op = vn_def(data, in_vn).expect("signform2: inVn written");
        if op_code(data, sub_op) != OpCode::CPUI_SUBPIECE {
            return 0;
        }
        // int4 c = subOp->getIn(1)->getOffset();
        let c = vn_offset(data, op_in(data, sub_op, 1).expect("signform2: subOp in1")) as int4;
        // Varnode *multOut = subOp->getIn(0);
        let mult_out = op_in(data, sub_op, 0).expect("signform2: subOp in0");
        // int4 multSize = multOut->getSize();
        let mult_size = vn_size(data, mult_out);
        // if (c + sizeout != multSize) return 0;  // Must be extracting high part
        if c + sizeout != mult_size {
            return 0;
        }
        if !vn_is_written(data, mult_out) {
            return 0;
        }
        // PcodeOp *multOp = multOut->getDef();
        let mult_op = vn_def(data, mult_out).expect("signform2: multOut written");
        if op_code(data, mult_op) != OpCode::CPUI_INT_MULT {
            return 0;
        }
        // Search for the INT_SEXT
        let mut slot: int4 = 0;
        let mut sext_op: Option<OpId> = None;
        while slot < 2 {
            let vn = op_in(data, mult_op, slot).expect("signform2: multOp in");
            if !vn_is_written(data, vn) {
                slot += 1;
                continue;
            }
            let so = vn_def(data, vn).expect("signform2: written");
            if op_code(data, so) == OpCode::CPUI_INT_SEXT {
                sext_op = Some(so);
                break;
            }
            slot += 1;
        }
        if slot > 1 {
            return 0;
        }
        let sext_op = sext_op.expect("signform2: sextOp set when slot<=1");
        // Varnode *a = sextOp->getIn(0);
        let a = op_in(data, sext_op, 0).expect("signform2: sextOp in0");
        if vn_is_free(data, a) || vn_size(data, a) != sizeout {
            return 0;
        }
        // Varnode *otherVn = multOp->getIn(1-slot);
        let other_vn = op_in(data, mult_op, 1 - slot).expect("signform2: multOp 1-slot");
        // otherVn must be a positive integer and small enough...
        if vn_is_constant(data, other_vn) {
            if vn_offset(data, other_vn) > calc_mask(sizeout) {
                return 0;
            }
            if 2 * sizeout > mult_size {
                return 0;
            }
        } else if vn_is_written(data, other_vn) {
            let zext_op = vn_def(data, other_vn).expect("signform2: otherVn written");
            if op_code(data, zext_op) != OpCode::CPUI_INT_ZEXT {
                return 0;
            }
            let zin = op_in(data, zext_op, 0).expect("signform2: zextOp in0");
            if vn_size(data, zin) + sizeout > mult_size {
                return 0;
            }
        } else {
            return 0;
        }
        // data.opSetInput(op, a, 0);
        data.op_set_input(op, a, 0).expect("signform2: opSetInput a");
        // return 0;   (faithful: the input set is NOT reported as a change)
        0
    }
}

// =============================================================================
// RuleSignNearMult (ruleaction.cc:8568)
// =============================================================================

/// `RuleSignNearMult` — simplify division form:
/// `(V + (V s>> 0x1f)>>(32-n)) & (-1<<n)  =>  (V s/ 2^n) * 2^n`.
///
/// // SEAM(W3-output): the transformation fabricates a new INT_SDIV op with a
/// fresh unique output (`newOp` + `newUniqueOut`).  `newUniqueOut` routes through
/// `Funcdata::opSetOutput`, which is the deferred split-borrow seam; the full
/// pattern match is ported, the construction bails when the seam errors.
pub struct RuleSignNearMult;

impl RuleSignNearMult {
    /// Constructor (C++ `RuleSignNearMult(g)`), name "signnearmult".
    pub fn new() -> RuleSignNearMult {
        RuleSignNearMult
    }
}

impl Default for RuleSignNearMult {
    fn default() -> Self {
        RuleSignNearMult::new()
    }
}

impl Rule for RuleSignNearMult {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_INT_AND]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if grouplist.contains("analysis") {
            Some(Box::new(RuleSignNearMult))
        } else {
            None
        }
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // if (!op->getIn(1)->isConstant()) return 0;
        let in1 = op_in(data, op, 1).expect("signnearmult: in1");
        if !vn_is_constant(data, in1) {
            return 0;
        }
        // if (!op->getIn(0)->isWritten()) return 0;
        let in0 = op_in(data, op, 0).expect("signnearmult: in0");
        if !vn_is_written(data, in0) {
            return 0;
        }
        // PcodeOp *addop = op->getIn(0)->getDef();
        let addop = vn_def(data, in0).expect("signnearmult: in0 written");
        if op_code(data, addop) != OpCode::CPUI_INT_ADD {
            return 0;
        }
        let mut i: int4 = 0;
        let mut shiftvn: Option<VarnodeId> = None;
        let mut unshiftop: Option<OpId> = None;
        while i < 2 {
            let sv = op_in(data, addop, i).expect("signnearmult: addop in");
            if !vn_is_written(data, sv) {
                i += 1;
                continue;
            }
            let uo = vn_def(data, sv).expect("signnearmult: sv written");
            if op_code(data, uo) == OpCode::CPUI_INT_RIGHT {
                if !vn_is_constant(data, op_in(data, uo, 1).expect("signnearmult: uo in1")) {
                    i += 1;
                    continue;
                }
                shiftvn = Some(sv);
                unshiftop = Some(uo);
                break;
            }
            i += 1;
        }
        if i == 2 {
            return 0;
        }
        let shiftvn = shiftvn.expect("signnearmult: shiftvn");
        let unshiftop = unshiftop.expect("signnearmult: unshiftop");
        // Varnode *x = addop->getIn(1-i);
        let x = op_in(data, addop, 1 - i).expect("signnearmult: addop 1-i");
        if vn_is_free(data, x) {
            return 0;
        }
        // int4 n = unshiftop->getIn(1)->getOffset();
        let mut n = vn_offset(data, op_in(data, unshiftop, 1).expect("signnearmult: unshiftop in1"))
            as int4;
        if n <= 0 {
            return 0;
        }
        // n = shiftvn->getSize()*8 - n;
        n = vn_size(data, shiftvn) * 8 - n;
        if n <= 0 {
            return 0;
        }
        // uintb mask = calc_mask(shiftvn->getSize());
        let mut mask = calc_mask(vn_size(data, shiftvn));
        // mask = (mask<<n)&mask;
        mask = (mask << n) & mask;
        // if (mask != op->getIn(1)->getOffset()) return 0;
        if mask != vn_offset(data, in1) {
            return 0;
        }
        // Varnode *sgnvn = unshiftop->getIn(0);
        let sgnvn = op_in(data, unshiftop, 0).expect("signnearmult: unshiftop in0");
        if !vn_is_written(data, sgnvn) {
            return 0;
        }
        let sshiftop = vn_def(data, sgnvn).expect("signnearmult: sgnvn written");
        if op_code(data, sshiftop) != OpCode::CPUI_INT_SRIGHT {
            return 0;
        }
        if !vn_is_constant(data, op_in(data, sshiftop, 1).expect("signnearmult: sshiftop in1")) {
            return 0;
        }
        // if (sshiftop->getIn(0) != x) return 0;
        if op_in(data, sshiftop, 0).expect("signnearmult: sshiftop in0") != x {
            return 0;
        }
        // int4 val = sshiftop->getIn(1)->getOffset();
        let val = vn_offset(data, op_in(data, sshiftop, 1).expect("signnearmult: sshiftop in1"))
            as int4;
        if val != 8 * vn_size(data, x) - 1 {
            return 0;
        }

        // uintb pow = 1 << n;
        let mut pow: uintb = 1;
        pow <<= n;
        // PcodeOp *newdiv = data.newOp(2,op->getAddr());
        let addr = data.obank().get(op).expect("signnearmult: op").get_addr().clone();
        let newdiv = data.new_op(2, addr);
        // data.opSetOpcode(newdiv,CPUI_INT_SDIV);
        data.op_set_opcode(newdiv, typeop_for(OpCode::CPUI_INT_SDIV));
        // Varnode *divvn = data.newUniqueOut(x->getSize(),newdiv);
        // -- SEAM(W3-output): newUniqueOut -> opSetOutput (deferred split-borrow).
        let xsize = vn_size(data, x);
        let divvn = data.new_unique(xsize, None);
        if data.op_set_output(newdiv, divvn).is_err() {
            return 0; // SEAM(W3-output): construction deferred; see module doc.
        }
        // data.opSetInput(newdiv,x,0);
        data.op_set_input(newdiv, x, 0).expect("signnearmult: newdiv in0");
        // data.opSetInput(newdiv,data.newConstant(x->getSize(),pow),1);
        let c1 = data.new_constant(xsize, pow);
        data.op_set_input(newdiv, c1, 1).expect("signnearmult: newdiv in1");
        // data.opInsertBefore(newdiv,op);
        data.op_insert_before(newdiv, op);

        // data.opSetOpcode(op,CPUI_INT_MULT);
        data.op_set_opcode(op, typeop_for(OpCode::CPUI_INT_MULT));
        // data.opSetInput(op,divvn,0);
        data.op_set_input(op, divvn, 0).expect("signnearmult: op in0");
        // data.opSetInput(op,data.newConstant(x->getSize(),pow),1);
        let c2 = data.new_constant(xsize, pow);
        data.op_set_input(op, c2, 1).expect("signnearmult: op in1");
        1
    }
}

// =============================================================================
// RuleModOpt (ruleaction.cc:8629)
// =============================================================================

/// `RuleModOpt` — simplify expressions that optimize INT_REM and INT_SREM.
pub struct RuleModOpt;

impl RuleModOpt {
    /// Constructor (C++ `RuleModOpt(g)`), name "modopt".
    pub fn new() -> RuleModOpt {
        RuleModOpt
    }
}

impl Default for RuleModOpt {
    fn default() -> Self {
        RuleModOpt::new()
    }
}

impl Rule for RuleModOpt {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_INT_DIV, OpCode::CPUI_INT_SDIV]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if grouplist.contains("analysis") {
            Some(Box::new(RuleModOpt))
        } else {
            None
        }
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // x = op->getIn(0); div = op->getIn(1); outvn = op->getOut();
        let x = op_in(data, op, 0).expect("modopt: in0");
        let div = op_in(data, op, 1).expect("modopt: in1");
        let outvn = op_out(data, op).expect("modopt: out");
        // for(iter1=outvn->beginDescend();iter1!=outvn->endDescend();++iter1)
        for multop in vn_descend(data, outvn) {
            if op_code(data, multop) != OpCode::CPUI_INT_MULT {
                continue;
            }
            // div2 = multop->getIn(1); if (div2 == outvn) div2 = multop->getIn(0);
            let mut div2 = op_in(data, multop, 1).expect("modopt: multop in1");
            if div2 == outvn {
                div2 = op_in(data, multop, 0).expect("modopt: multop in0");
            }
            // Check that div is 2's complement of div2
            if vn_is_constant(data, div2) {
                if !vn_is_constant(data, div) {
                    continue;
                }
                let mask = calc_mask(vn_size(data, div2));
                // (((div2->getOffset() ^ mask)+1)&mask) != div->getOffset()
                if (((vn_offset(data, div2) ^ mask).wrapping_add(1)) & mask) != vn_offset(data, div)
                {
                    continue;
                }
            } else {
                if !vn_is_written(data, div2) {
                    continue;
                }
                let d2def = vn_def(data, div2).expect("modopt: div2 written");
                if op_code(data, d2def) != OpCode::CPUI_INT_2COMP {
                    continue;
                }
                if op_in(data, d2def, 0).expect("modopt: 2comp in0") != div {
                    continue;
                }
            }
            // outvn2 = multop->getOut();
            let outvn2 = op_out(data, multop).expect("modopt: multop out");
            for addop in vn_descend(data, outvn2) {
                if op_code(data, addop) != OpCode::CPUI_INT_ADD {
                    continue;
                }
                // lvn = addop->getIn(0); if (lvn == outvn2) lvn = addop->getIn(1);
                let mut lvn = op_in(data, addop, 0).expect("modopt: addop in0");
                if lvn == outvn2 {
                    lvn = op_in(data, addop, 1).expect("modopt: addop in1");
                }
                if lvn != x {
                    continue;
                }
                // data.opSetInput(addop,x,0);
                data.op_set_input(addop, x, 0).expect("modopt: opSetInput x");
                if vn_is_constant(data, div) {
                    let dsize = vn_size(data, div);
                    let doff = vn_offset(data, div);
                    let c = data.new_constant(dsize, doff);
                    data.op_set_input(addop, c, 1).expect("modopt: opSetInput div const");
                } else {
                    data.op_set_input(addop, div, 1).expect("modopt: opSetInput div");
                }
                // Remainder of proper signedness
                if op_code(data, op) == OpCode::CPUI_INT_DIV {
                    data.op_set_opcode(addop, typeop_for(OpCode::CPUI_INT_REM));
                } else {
                    data.op_set_opcode(addop, typeop_for(OpCode::CPUI_INT_SREM));
                }
                return 1;
            }
        }
        0
    }
}

// =============================================================================
// RuleSignMod2nOpt (ruleaction.cc:8692)
// =============================================================================

/// `RuleSignMod2nOpt` — convert INT_SREM forms to `V s% 2^n`.
pub struct RuleSignMod2nOpt;

impl RuleSignMod2nOpt {
    /// Constructor (C++ `RuleSignMod2nOpt(g)`), name "signmod2nopt".
    pub fn new() -> RuleSignMod2nOpt {
        RuleSignMod2nOpt
    }

    /// Verify that the given Varnode is a sign extraction of the form `V s>> 63`
    /// (C++ `RuleSignMod2nOpt::checkSignExtraction`).  Returns the Varnode whose
    /// sign is extracted, or `None`.  // shared with RuleSignMod2Opt (C++ static).
    pub fn check_sign_extraction(data: &Funcdata, out_vn: VarnodeId) -> Option<VarnodeId> {
        if !vn_is_written(data, out_vn) {
            return None;
        }
        let sign_op = vn_def(data, out_vn).expect("checkSignExtraction: outVn written");
        if op_code(data, sign_op) != OpCode::CPUI_INT_SRIGHT {
            return None;
        }
        let const_vn = op_in(data, sign_op, 1).expect("checkSignExtraction: signOp in1");
        if !vn_is_constant(data, const_vn) {
            return None;
        }
        let val = vn_offset(data, const_vn) as int4;
        let res_vn = op_in(data, sign_op, 0).expect("checkSignExtraction: signOp in0");
        let insize = vn_size(data, res_vn);
        if val != insize * 8 - 1 {
            return None;
        }
        Some(res_vn)
    }
}

impl Default for RuleSignMod2nOpt {
    fn default() -> Self {
        RuleSignMod2nOpt::new()
    }
}

impl Rule for RuleSignMod2nOpt {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_INT_RIGHT]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if grouplist.contains("analysis") {
            Some(Box::new(RuleSignMod2nOpt))
        } else {
            None
        }
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // if (!op->getIn(1)->isConstant()) return 0;
        let in1 = op_in(data, op, 1).expect("signmod2nopt: in1");
        if !vn_is_constant(data, in1) {
            return 0;
        }
        // int4 shiftAmt = op->getIn(1)->getOffset();
        let shift_amt = vn_offset(data, in1) as int4;
        // Varnode *a = checkSignExtraction(op->getIn(0));
        let in0 = op_in(data, op, 0).expect("signmod2nopt: in0");
        let a = match RuleSignMod2nOpt::check_sign_extraction(data, in0) {
            Some(a) => a,
            None => return 0,
        };
        if vn_is_free(data, a) {
            return 0;
        }
        // Varnode *correctVn = op->getOut();
        let correct_vn = op_out(data, op).expect("signmod2nopt: out");
        // int4 n = a->getSize() * 8 - shiftAmt;
        let n = vn_size(data, a) * 8 - shift_amt;
        // uintb mask = (1 << n) - 1;
        let mut mask: uintb = 1;
        mask = (mask << n) - 1;
        for multop in vn_descend(data, correct_vn) {
            if op_code(data, multop) != OpCode::CPUI_INT_MULT {
                continue;
            }
            let negone = op_in(data, multop, 1).expect("signmod2nopt: multop in1");
            if !vn_is_constant(data, negone) {
                continue;
            }
            if vn_offset(data, negone) != calc_mask(vn_size(data, correct_vn)) {
                continue;
            }
            // PcodeOp *baseOp = multop->getOut()->loneDescend();
            let mult_out = op_out(data, multop).expect("signmod2nopt: multop out");
            let base_op = match data.lone_descend(mult_out) {
                Some(b) => b,
                None => continue,
            };
            if op_code(data, base_op) != OpCode::CPUI_INT_ADD {
                continue;
            }
            // int4 slot = 1 - baseOp->getSlot(multop->getOut());
            let slot = 1 - op_slot(data, base_op, mult_out);
            // Varnode *andOut = baseOp->getIn(slot);
            let mut and_out = op_in(data, base_op, slot).expect("signmod2nopt: baseOp slot");
            if !vn_is_written(data, and_out) {
                continue;
            }
            let mut and_op = vn_def(data, and_out).expect("signmod2nopt: andOut written");
            let mut trunc_size: int4 = -1;
            if op_code(data, and_op) == OpCode::CPUI_INT_ZEXT {
                // Look for intervening extension after INT_AND
                and_out = op_in(data, and_op, 0).expect("signmod2nopt: zext in0");
                if !vn_is_written(data, and_out) {
                    continue;
                }
                and_op = vn_def(data, and_out).expect("signmod2nopt: andOut2 written");
                if op_code(data, and_op) != OpCode::CPUI_INT_AND {
                    continue;
                }
                trunc_size = vn_size(data, and_out); // truncated form
            } else if op_code(data, and_op) != OpCode::CPUI_INT_AND {
                continue;
            }

            let const_vn = op_in(data, and_op, 1).expect("signmod2nopt: andOp in1");
            if !vn_is_constant(data, const_vn) {
                continue;
            }
            if vn_offset(data, const_vn) != mask {
                continue;
            }
            let add_out = op_in(data, and_op, 0).expect("signmod2nopt: andOp in0");
            if !vn_is_written(data, add_out) {
                continue;
            }
            let add_op = vn_def(data, add_out).expect("signmod2nopt: addOut written");
            if op_code(data, add_op) != OpCode::CPUI_INT_ADD {
                continue;
            }
            // Search for "a" as one of the inputs to addOp
            let mut a_slot: int4 = 0;
            while a_slot < 2 {
                let mut vn = op_in(data, add_op, a_slot).expect("signmod2nopt: addOp aSlot");
                if trunc_size >= 0 {
                    if !vn_is_written(data, vn) {
                        a_slot += 1;
                        continue;
                    }
                    let sub_op = vn_def(data, vn).expect("signmod2nopt: trunc written");
                    if op_code(data, sub_op) != OpCode::CPUI_SUBPIECE {
                        a_slot += 1;
                        continue;
                    }
                    if vn_offset(data, op_in(data, sub_op, 1).expect("signmod2nopt: subOp in1"))
                        != 0
                    {
                        a_slot += 1;
                        continue;
                    }
                    vn = op_in(data, sub_op, 0).expect("signmod2nopt: subOp in0");
                }
                if a == vn {
                    break;
                }
                a_slot += 1;
            }
            if a_slot > 1 {
                continue;
            }
            // Verify other input to addOp is INT_RIGHT by shiftAmt
            let ext_vn0 = op_in(data, add_op, 1 - a_slot).expect("signmod2nopt: addOp 1-aSlot");
            if !vn_is_written(data, ext_vn0) {
                continue;
            }
            let shift_op = vn_def(data, ext_vn0).expect("signmod2nopt: extVn written");
            if op_code(data, shift_op) != OpCode::CPUI_INT_RIGHT {
                continue;
            }
            let const_vn2 = op_in(data, shift_op, 1).expect("signmod2nopt: shiftOp in1");
            if !vn_is_constant(data, const_vn2) {
                continue;
            }
            let mut shiftval = vn_offset(data, const_vn2) as int4;
            if trunc_size >= 0 {
                shiftval += (vn_size(data, a) - trunc_size) * 8;
            }
            if shiftval != shift_amt {
                continue;
            }
            // Verify input to INT_RIGHT is a sign extraction of "a"
            let shift_in0 = op_in(data, shift_op, 0).expect("signmod2nopt: shiftOp in0");
            let mut ext_vn = match RuleSignMod2nOpt::check_sign_extraction(data, shift_in0) {
                Some(e) => e,
                None => continue,
            };
            if trunc_size >= 0 {
                if !vn_is_written(data, ext_vn) {
                    continue;
                }
                let sub_op = vn_def(data, ext_vn).expect("signmod2nopt: extVn written2");
                if op_code(data, sub_op) != OpCode::CPUI_SUBPIECE {
                    continue;
                }
                if (vn_offset(data, op_in(data, sub_op, 1).expect("signmod2nopt: subOp2 in1"))
                    as int4)
                    != trunc_size
                {
                    continue;
                }
                ext_vn = op_in(data, sub_op, 0).expect("signmod2nopt: subOp2 in0");
            }
            if a != ext_vn {
                continue;
            }

            // data.opSetOpcode(baseOp, CPUI_INT_SREM);
            data.op_set_opcode(base_op, typeop_for(OpCode::CPUI_INT_SREM));
            // data.opSetInput(baseOp, a, 0);
            data.op_set_input(base_op, a, 0).expect("signmod2nopt: baseOp in0");
            // data.opSetInput(baseOp, data.newConstant(a->getSize(), mask+1), 1);
            let asize = vn_size(data, a);
            let c = data.new_constant(asize, mask + 1);
            data.op_set_input(base_op, c, 1).expect("signmod2nopt: baseOp in1");
            return 1;
        }
        0
    }
}

// =============================================================================
// RuleSignMod2Opt (ruleaction.cc:8814)
// =============================================================================

/// `RuleSignMod2Opt` — convert INT_SREM form `(V - sign)&1 + sign  =>  V s% 2`.
/// A specialized form of `RuleSignMod2nOpt`.
pub struct RuleSignMod2Opt;

impl RuleSignMod2Opt {
    /// Constructor (C++ `RuleSignMod2Opt(g)`), name "signmod2opt".
    pub fn new() -> RuleSignMod2Opt {
        RuleSignMod2Opt
    }
}

impl Default for RuleSignMod2Opt {
    fn default() -> Self {
        RuleSignMod2Opt::new()
    }
}

impl Rule for RuleSignMod2Opt {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_INT_AND]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if grouplist.contains("analysis") {
            Some(Box::new(RuleSignMod2Opt))
        } else {
            None
        }
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // Varnode *constVn = op->getIn(1);
        let const_vn = op_in(data, op, 1).expect("signmod2opt: in1");
        if !vn_is_constant(data, const_vn) {
            return 0;
        }
        if vn_offset(data, const_vn) != 1 {
            return 0;
        }
        // Varnode *addOut = op->getIn(0);
        let add_out = op_in(data, op, 0).expect("signmod2opt: in0");
        if !vn_is_written(data, add_out) {
            return 0;
        }
        let add_op = vn_def(data, add_out).expect("signmod2opt: addOut written");
        if op_code(data, add_op) != OpCode::CPUI_INT_ADD {
            return 0;
        }
        let mut mult_slot: int4 = 0;
        let mut mult_op: Option<OpId> = None;
        let mut trunc = false;
        while mult_slot < 2 {
            let vn = op_in(data, add_op, mult_slot).expect("signmod2opt: addOp slot");
            if !vn_is_written(data, vn) {
                mult_slot += 1;
                continue;
            }
            let mo = vn_def(data, vn).expect("signmod2opt: written");
            if op_code(data, mo) != OpCode::CPUI_INT_MULT {
                mult_slot += 1;
                continue;
            }
            let c = op_in(data, mo, 1).expect("signmod2opt: multOp in1");
            if !vn_is_constant(data, c) {
                mult_slot += 1;
                continue;
            }
            // Check for INT_MULT by -1
            if vn_offset(data, c) == calc_mask(vn_size(data, c)) {
                mult_op = Some(mo);
                break;
            }
            mult_slot += 1;
        }
        if mult_slot > 1 {
            return 0;
        }
        let mult_op = mult_op.expect("signmod2opt: multOp set");
        // Varnode *base = RuleSignMod2nOpt::checkSignExtraction(multOp->getIn(0));
        let mult_in0 = op_in(data, mult_op, 0).expect("signmod2opt: multOp in0");
        let mut base = match RuleSignMod2nOpt::check_sign_extraction(data, mult_in0) {
            Some(b) => b,
            None => return 0,
        };
        // Varnode *otherBase = addOp->getIn(1-multSlot);
        let mut other_base = op_in(data, add_op, 1 - mult_slot).expect("signmod2opt: addOp other");
        if base != other_base {
            if !vn_is_written(data, base) || !vn_is_written(data, other_base) {
                return 0;
            }
            let sub_op = vn_def(data, base).expect("signmod2opt: base written");
            if op_code(data, sub_op) != OpCode::CPUI_SUBPIECE {
                return 0;
            }
            let trunc_amt =
                vn_offset(data, op_in(data, sub_op, 1).expect("signmod2opt: subOp in1")) as int4;
            // Must truncate all but high part
            let sub_in0 = op_in(data, sub_op, 0).expect("signmod2opt: subOp in0");
            if trunc_amt + vn_size(data, base) != vn_size(data, sub_in0) {
                return 0;
            }
            base = sub_in0;
            let sub_op2 = vn_def(data, other_base).expect("signmod2opt: otherBase written");
            if op_code(data, sub_op2) != OpCode::CPUI_SUBPIECE {
                return 0;
            }
            if vn_offset(data, op_in(data, sub_op2, 1).expect("signmod2opt: subOp2 in1")) != 0 {
                return 0;
            }
            other_base = op_in(data, sub_op2, 0).expect("signmod2opt: subOp2 in0");
            if other_base != base {
                return 0;
            }
            trunc = true;
        }
        if vn_is_free(data, base) {
            return 0;
        }
        // Varnode *andOut = op->getOut();
        let mut and_out = op_out(data, op).expect("signmod2opt: out");
        if trunc {
            let ext_op = match data.lone_descend(and_out) {
                Some(e) => e,
                None => return 0,
            };
            if op_code(data, ext_op) != OpCode::CPUI_INT_ZEXT {
                return 0;
            }
            and_out = op_out(data, ext_op).expect("signmod2opt: extOp out");
        }
        for root_op in vn_descend(data, and_out) {
            if op_code(data, root_op) != OpCode::CPUI_INT_ADD {
                continue;
            }
            let slot = op_slot(data, root_op, and_out);
            let root_other = op_in(data, root_op, 1 - slot).expect("signmod2opt: rootOp other");
            let other = RuleSignMod2nOpt::check_sign_extraction(data, root_other);
            if other != Some(base) {
                continue;
            }
            // data.opSetOpcode(rootOp, CPUI_INT_SREM);
            data.op_set_opcode(root_op, typeop_for(OpCode::CPUI_INT_SREM));
            // data.opSetInput(rootOp,base,0);
            data.op_set_input(root_op, base, 0).expect("signmod2opt: rootOp in0");
            // data.opSetInput(rootOp,data.newConstant(base->getSize(), 2),1);
            let bsize = vn_size(data, base);
            let c = data.new_constant(bsize, 2);
            data.op_set_input(root_op, c, 1).expect("signmod2opt: rootOp in1");
            return 1;
        }
        0
    }
}

// =============================================================================
// RuleSignMod2nOpt2 (ruleaction.cc:8886)
// =============================================================================

/// `RuleSignMod2nOpt2` — convert INT_SREM form `V - (Vadj & ~(2^n-1)) =>  V s% 2^n`.
pub struct RuleSignMod2nOpt2;

impl RuleSignMod2nOpt2 {
    /// Constructor (C++ `RuleSignMod2nOpt2(g)`), name "signmod2nopt2".
    pub fn new() -> RuleSignMod2nOpt2 {
        RuleSignMod2nOpt2
    }

    /// Verify a form of `V - (V s>> 0x3f)` (C++
    /// `RuleSignMod2nOpt2::checkSignExtForm`).  Returns V or `None`.
    fn check_sign_ext_form(data: &Funcdata, op: OpId) -> Option<VarnodeId> {
        let mut slot: int4 = 0;
        while slot < 2 {
            let minus_vn = op_in(data, op, slot).expect("checkSignExtForm: op slot");
            if !vn_is_written(data, minus_vn) {
                slot += 1;
                continue;
            }
            let mult_op = vn_def(data, minus_vn).expect("checkSignExtForm: minusVn written");
            if op_code(data, mult_op) != OpCode::CPUI_INT_MULT {
                slot += 1;
                continue;
            }
            let const_vn = op_in(data, mult_op, 1).expect("checkSignExtForm: multOp in1");
            if !vn_is_constant(data, const_vn) {
                slot += 1;
                continue;
            }
            if vn_offset(data, const_vn) != calc_mask(vn_size(data, const_vn)) {
                slot += 1;
                continue;
            }
            let base = op_in(data, op, 1 - slot).expect("checkSignExtForm: op 1-slot");
            let sign_ext = op_in(data, mult_op, 0).expect("checkSignExtForm: multOp in0");
            if !vn_is_written(data, sign_ext) {
                slot += 1;
                continue;
            }
            let shift_op = vn_def(data, sign_ext).expect("checkSignExtForm: signExt written");
            if op_code(data, shift_op) != OpCode::CPUI_INT_SRIGHT {
                slot += 1;
                continue;
            }
            if op_in(data, shift_op, 0).expect("checkSignExtForm: shiftOp in0") != base {
                slot += 1;
                continue;
            }
            let const_vn2 = op_in(data, shift_op, 1).expect("checkSignExtForm: shiftOp in1");
            if !vn_is_constant(data, const_vn2) {
                slot += 1;
                continue;
            }
            if (vn_offset(data, const_vn2) as int4) != 8 * vn_size(data, base) - 1 {
                slot += 1;
                continue;
            }
            return Some(base);
        }
        None
    }

    /// Verify an \e if block like `V = (V s< 0) ? V + 2^n-1 : V` (C++
    /// `RuleSignMod2nOpt2::checkMultiequalForm`).  Returns V or `None`.
    ///
    /// // SEAM(W7): uses block topology (`getParent`, `getIn`, `sizeOut`/`sizeIn`,
    /// `lastOp`, `getTrueOut`/`getFalseOut`).  The basic-block graph IS available
    /// (W3-block), so this is fully ported.
    fn check_multiequal_form(data: &Funcdata, op: OpId, npow: uintb) -> Option<VarnodeId> {
        // if (op->numInput() != 2) return 0;
        if op_num_input(data, op) != 2 {
            return None;
        }
        let npow = npow - 1; // 2^n - 1
        let mut slot: int4 = 0;
        let mut base: Option<VarnodeId> = None;
        while slot < op_num_input(data, op) {
            let add_out = op_in(data, op, slot).expect("checkMultiequalForm: op slot");
            if !vn_is_written(data, add_out) {
                slot += 1;
                continue;
            }
            let add_op = vn_def(data, add_out).expect("checkMultiequalForm: addOut written");
            if op_code(data, add_op) != OpCode::CPUI_INT_ADD {
                slot += 1;
                continue;
            }
            let const_vn = op_in(data, add_op, 1).expect("checkMultiequalForm: addOp in1");
            if !vn_is_constant(data, const_vn) {
                slot += 1;
                continue;
            }
            if vn_offset(data, const_vn) != npow {
                slot += 1;
                continue;
            }
            let b = op_in(data, add_op, 0).expect("checkMultiequalForm: addOp in0");
            let other_base = op_in(data, op, 1 - slot).expect("checkMultiequalForm: op 1-slot");
            if other_base == b {
                base = Some(b);
                break;
            }
            slot += 1;
        }
        if slot > 1 {
            return None;
        }
        let base = base.expect("checkMultiequalForm: base set when slot<=1");
        // BlockBasic *bl = op->getParent();
        let bl = data.obank().get(op).expect("checkMultiequalForm: op").get_parent()?;
        // int4 innerSlot = 0; inner = (BlockBasic*)bl->getIn(innerSlot);
        let mut inner_slot: int4 = 0;
        let mut inner = block_get_in(data, bl, inner_slot);
        if block_size_out(data, inner) != 1 || block_size_in(data, inner) != 1 {
            inner_slot = 1;
            inner = block_get_in(data, bl, inner_slot);
            if block_size_out(data, inner) != 1 || block_size_in(data, inner) != 1 {
                return None;
            }
        }
        // BlockBasic *decision = (BlockBasic*)inner->getIn(0);
        let decision = block_get_in(data, inner, 0);
        // if (bl->getIn(1 - innerSlot) != decision) return 0;
        if block_get_in(data, bl, 1 - inner_slot) != decision {
            return None;
        }
        // PcodeOp *cbranch = decision->lastOp();
        let cbranch = data.bb_op_tail(decision)?;
        if op_code(data, cbranch) != OpCode::CPUI_CBRANCH {
            return None;
        }
        // Varnode *boolVn = cbranch->getIn(1);
        let bool_vn = op_in(data, cbranch, 1).expect("checkMultiequalForm: cbranch in1");
        if !vn_is_written(data, bool_vn) {
            return None;
        }
        let less_op = vn_def(data, bool_vn).expect("checkMultiequalForm: boolVn written");
        if op_code(data, less_op) != OpCode::CPUI_INT_SLESS {
            return None;
        }
        let less_in1 = op_in(data, less_op, 1).expect("checkMultiequalForm: lessOp in1");
        if !vn_is_constant(data, less_in1) {
            return None;
        }
        if vn_offset(data, less_in1) != 0 {
            return None;
        }
        // FlowBlock *negBlock = cbranch->isBooleanFlip() ? decision->getFalseOut() : decision->getTrueOut();
        let cbranch_flip =
            data.obank().get(cbranch).expect("checkMultiequalForm: cbranch").is_boolean_flip();
        let neg_block = if cbranch_flip {
            block_get_false_out(data, decision)
        } else {
            block_get_true_out(data, decision)
        };
        // int4 negSlot = (negBlock == inner) ? innerSlot : (1-innerSlot);
        let neg_slot = if neg_block == inner { inner_slot } else { 1 - inner_slot };
        if neg_slot != slot {
            return None;
        }
        Some(base)
    }
}

impl Default for RuleSignMod2nOpt2 {
    fn default() -> Self {
        RuleSignMod2nOpt2::new()
    }
}

impl Rule for RuleSignMod2nOpt2 {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_INT_MULT]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if grouplist.contains("analysis") {
            Some(Box::new(RuleSignMod2nOpt2))
        } else {
            None
        }
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // Varnode *constVn = op->getIn(1);
        let const_vn = op_in(data, op, 1).expect("signmod2nopt2: in1");
        if !vn_is_constant(data, const_vn) {
            return 0;
        }
        let mask = calc_mask(vn_size(data, const_vn));
        if vn_offset(data, const_vn) != mask {
            return 0; // Must be INT_MULT by -1
        }
        // Varnode *andOut = op->getIn(0);
        let and_out = op_in(data, op, 0).expect("signmod2nopt2: in0");
        if !vn_is_written(data, and_out) {
            return 0;
        }
        let and_op = vn_def(data, and_out).expect("signmod2nopt2: andOut written");
        if op_code(data, and_op) != OpCode::CPUI_INT_AND {
            return 0;
        }
        let const_vn = op_in(data, and_op, 1).expect("signmod2nopt2: andOp in1");
        if !vn_is_constant(data, const_vn) {
            return 0;
        }
        // uintb npow = (~constVn->getOffset() + 1) & mask;
        let npow = ((!vn_offset(data, const_vn)).wrapping_add(1)) & mask;
        if popcount(npow) != 1 {
            return 0; // constVn must be of form 11111..000..
        }
        if npow == 1 {
            return 0;
        }
        // Varnode *adjVn = andOp->getIn(0);
        let adj_vn = op_in(data, and_op, 0).expect("signmod2nopt2: andOp in0");
        if !vn_is_written(data, adj_vn) {
            return 0;
        }
        let adj_op = vn_def(data, adj_vn).expect("signmod2nopt2: adjVn written");
        let base: Option<VarnodeId>;
        if op_code(data, adj_op) == OpCode::CPUI_INT_ADD {
            if npow != 2 {
                return 0; // Special mod 2 form
            }
            base = RuleSignMod2nOpt2::check_sign_ext_form(data, adj_op);
        } else if op_code(data, adj_op) == OpCode::CPUI_MULTIEQUAL {
            base = RuleSignMod2nOpt2::check_multiequal_form(data, adj_op, npow);
        } else {
            return 0;
        }
        let base = match base {
            Some(b) => b,
            None => return 0,
        };
        if vn_is_free(data, base) {
            return 0;
        }
        // Varnode *multOut = op->getOut();
        let mult_out = op_out(data, op).expect("signmod2nopt2: out");
        for root_op in vn_descend(data, mult_out) {
            if op_code(data, root_op) != OpCode::CPUI_INT_ADD {
                continue;
            }
            let slot = op_slot(data, root_op, mult_out);
            if op_in(data, root_op, 1 - slot).expect("signmod2nopt2: rootOp 1-slot") != base {
                continue;
            }
            if slot == 0 {
                data.op_set_input(root_op, base, 0).expect("signmod2nopt2: rootOp in0");
            }
            // data.opSetInput(rootOp, data.newConstant(base->getSize(),npow), 1);
            let bsize = vn_size(data, base);
            let c = data.new_constant(bsize, npow);
            data.op_set_input(root_op, c, 1).expect("signmod2nopt2: rootOp in1");
            // data.opSetOpcode(rootOp, CPUI_INT_SREM);
            data.op_set_opcode(root_op, typeop_for(OpCode::CPUI_INT_SREM));
            return 1;
        }
        0
    }
}

// =============================================================================
// RuleSegment (ruleaction.cc:9020)
// =============================================================================

/// `RuleSegment` — propagate constants through a SEGMENTOP.
///
/// // SEAM(W4): needs `getSpaceFromConst`, `userops.getSegmentOp`,
/// `SegmentOp::execute`, `hasFarPointerSupport`, and the `contiguous_test`/
/// `findContiguousWhole` expression helpers (none ported yet).  The body is a
/// no-op until W4 fills these in; recorded as a loss.
pub struct RuleSegment;

impl RuleSegment {
    /// Constructor (C++ `RuleSegment(g)`), name "segment".
    pub fn new() -> RuleSegment {
        RuleSegment
    }
}

impl Default for RuleSegment {
    fn default() -> Self {
        RuleSegment::new()
    }
}

impl Rule for RuleSegment {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_SEGMENTOP]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if grouplist.contains("segment") {
            Some(Box::new(RuleSegment))
        } else {
            None
        }
    }

    fn apply_op(&mut self, _op: OpId, _data: &mut Funcdata) -> int4 {
        // SEAM(W4): SegmentOp *segdef =
        //   data.getArch()->userops.getSegmentOp(op->getIn(0)->getSpaceFromConst()->getIndex());
        // ... segdef->execute(bindlist) / hasFarPointerSupport / contiguous_test /
        // findContiguousWhole are all unported W4 surfaces.  No change here.
        0
    }
}

// =============================================================================
// RulePtrFlow (ruleaction.cc:9071)
// =============================================================================

/// `RulePtrFlow` — mark Varnode/PcodeOp objects carrying or operating on
/// pointers (only installed when the default data space is truncated).
///
/// // SEAM(W4): the constructor reads `glb->getDefaultDataSpace()->isTruncated()`
/// to decide `hasTruncations`; `applyOp` reaches `getSpaceFromConst`,
/// `getDefaultCodeSpace`, and `truncatePointer` (which fabricates a SUBPIECE with
/// a fresh output via `newUniqueOut`/`newVarnodeOut` — the W3-output seam).  The
/// pointer-flow propagation helpers (`trialSetPtrFlow`, `propagateFlowToDef`,
/// `propagateFlowToReads`) are fully ported below; `truncatePointer` and the
/// `getSpaceFromConst`/default-space reads bail at the seam.
pub struct RulePtrFlow {
    /// `hasTruncations` (C++ field).  Without W4's `getDefaultDataSpace()`, this
    /// item cannot compute it; defaulted to `false` so the rule installs nothing
    /// (its `getOpList` early-returns) — the faithful behavior on a non-truncated
    /// architecture.  // SEAM(W4)
    has_truncations: bool,
}

impl RulePtrFlow {
    /// Constructor (C++ `RulePtrFlow(g,conf)`), name "ptrflow".
    ///
    /// C++: `glb = conf; hasTruncations = glb->getDefaultDataSpace()->isTruncated();`
    /// // SEAM(W4): `getDefaultDataSpace` unavailable; `has_truncations=false`.
    pub fn new() -> RulePtrFlow {
        RulePtrFlow { has_truncations: false }
    }

    /// Set \e ptrflow on the op only if it propagates (C++
    /// `RulePtrFlow::trialSetPtrFlow`).  Returns `true` if newly set.
    fn trial_set_ptr_flow(data: &mut Funcdata, op: OpId) -> bool {
        match op_code(data, op) {
            OpCode::CPUI_COPY
            | OpCode::CPUI_MULTIEQUAL
            | OpCode::CPUI_INT_ADD
            | OpCode::CPUI_INDIRECT
            | OpCode::CPUI_PTRSUB
            | OpCode::CPUI_PTRADD => {
                if !data.obank().get(op).expect("trialSetPtrFlow: op").is_ptr_flow() {
                    data.obank_mut().get_mut(op).expect("trialSetPtrFlow: op").set_ptr_flow();
                    return true;
                }
            }
            _ => {}
        }
        false
    }

    /// Propagate \e ptrflow to the Varnode and its defining op (C++
    /// `RulePtrFlow::propagateFlowToDef`).  Returns `true` if changed.
    fn propagate_flow_to_def(data: &mut Funcdata, vn: VarnodeId) -> bool {
        let mut made_change = false;
        if !data.vbank().get(vn).expect("propagateFlowToDef: vn").is_ptr_flow() {
            data.vbank_mut().get_mut(vn).expect("propagateFlowToDef: vn").set_ptr_flow();
            made_change = true;
        }
        if !vn_is_written(data, vn) {
            return made_change;
        }
        let op = vn_def(data, vn).expect("propagateFlowToDef: vn written");
        if RulePtrFlow::trial_set_ptr_flow(data, op) {
            made_change = true;
        }
        made_change
    }

    /// Propagate \e ptrflow to the Varnode and its descendant ops (C++
    /// `RulePtrFlow::propagateFlowToReads`).  Returns `true` if changed.
    fn propagate_flow_to_reads(data: &mut Funcdata, vn: VarnodeId) -> bool {
        let mut made_change = false;
        if !data.vbank().get(vn).expect("propagateFlowToReads: vn").is_ptr_flow() {
            data.vbank_mut().get_mut(vn).expect("propagateFlowToReads: vn").set_ptr_flow();
            made_change = true;
        }
        for op in vn_descend(data, vn) {
            if RulePtrFlow::trial_set_ptr_flow(data, op) {
                made_change = true;
            }
        }
        made_change
    }
}

impl Default for RulePtrFlow {
    fn default() -> Self {
        RulePtrFlow::new()
    }
}

impl Rule for RulePtrFlow {
    fn get_op_list(&self) -> Vec<OpCode> {
        // if (!hasTruncations) return;  -- only join the pool when aggressive
        if !self.has_truncations {
            return Vec::new();
        }
        vec![
            OpCode::CPUI_STORE,
            OpCode::CPUI_LOAD,
            OpCode::CPUI_COPY,
            OpCode::CPUI_MULTIEQUAL,
            OpCode::CPUI_INDIRECT,
            OpCode::CPUI_INT_ADD,
            OpCode::CPUI_CALLIND,
            OpCode::CPUI_BRANCHIND,
            OpCode::CPUI_PTRSUB,
            OpCode::CPUI_PTRADD,
        ]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if grouplist.contains("subvar") {
            Some(Box::new(RulePtrFlow { has_truncations: self.has_truncations }))
        } else {
            None
        }
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // Push pointer-ness.  Only the branches that do NOT need getSpaceFromConst
        // / getDefaultCodeSpace / truncatePointer are reachable here; the LOAD/
        // STORE/CALLIND/BRANCHIND arms reach W4/W3-output seams.
        let mut made_change = 0;
        match op_code(data, op) {
            OpCode::CPUI_LOAD | OpCode::CPUI_STORE => {
                // SEAM(W4): spc = op->getIn(0)->getSpaceFromConst();
                //   if (vn->getSize() > spc->getAddrSize()) truncatePointer(...);  (W3-output)
                //   then propagateFlowToDef(vn).  Deferred entirely.
                return 0;
            }
            OpCode::CPUI_CALLIND | OpCode::CPUI_BRANCHIND => {
                // SEAM(W4): spc = data.getArch()->getDefaultCodeSpace();
                //   if (vn->getSize() > spc->getAddrSize()) truncatePointer(...);  (W3-output)
                return 0;
            }
            OpCode::CPUI_NEW => {
                let vn = op_out(data, op).expect("ptrflow: NEW out");
                if RulePtrFlow::propagate_flow_to_reads(data, vn) {
                    made_change = 1;
                }
            }
            OpCode::CPUI_INDIRECT => {
                if !data.obank().get(op).expect("ptrflow: op").is_ptr_flow() {
                    return 0;
                }
                let out = op_out(data, op).expect("ptrflow: INDIRECT out");
                if RulePtrFlow::propagate_flow_to_reads(data, out) {
                    made_change = 1;
                }
                let in0 = op_in(data, op, 0).expect("ptrflow: INDIRECT in0");
                if RulePtrFlow::propagate_flow_to_def(data, in0) {
                    made_change = 1;
                }
            }
            OpCode::CPUI_COPY | OpCode::CPUI_PTRSUB | OpCode::CPUI_PTRADD => {
                if !data.obank().get(op).expect("ptrflow: op").is_ptr_flow() {
                    return 0;
                }
                let out = op_out(data, op).expect("ptrflow: copy out");
                if RulePtrFlow::propagate_flow_to_reads(data, out) {
                    made_change = 1;
                }
                let in0 = op_in(data, op, 0).expect("ptrflow: copy in0");
                if RulePtrFlow::propagate_flow_to_def(data, in0) {
                    made_change = 1;
                }
            }
            OpCode::CPUI_MULTIEQUAL | OpCode::CPUI_INT_ADD => {
                if !data.obank().get(op).expect("ptrflow: op").is_ptr_flow() {
                    return 0;
                }
                let out = op_out(data, op).expect("ptrflow: multi out");
                if RulePtrFlow::propagate_flow_to_reads(data, out) {
                    made_change = 1;
                }
                let n = op_num_input(data, op);
                for i in 0..n {
                    let vn = op_in(data, op, i).expect("ptrflow: multi in");
                    if RulePtrFlow::propagate_flow_to_def(data, vn) {
                        made_change = 1;
                    }
                }
            }
            _ => {}
        }
        made_change
    }
}

// =============================================================================
// RuleNegateNegate (ruleaction.cc:9267)
// =============================================================================

/// `RuleNegateNegate` — simplify INT_NEGATE chains:  `~~V  =>  V`.
pub struct RuleNegateNegate;

impl RuleNegateNegate {
    /// Constructor (C++ `RuleNegateNegate(g)`), name "negatenegate".
    pub fn new() -> RuleNegateNegate {
        RuleNegateNegate
    }
}

impl Default for RuleNegateNegate {
    fn default() -> Self {
        RuleNegateNegate::new()
    }
}

impl Rule for RuleNegateNegate {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_INT_NEGATE]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if grouplist.contains("analysis") {
            Some(Box::new(RuleNegateNegate))
        } else {
            None
        }
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // Varnode *vn1 = op->getIn(0);
        let vn1 = op_in(data, op, 0).expect("negatenegate: in0");
        if !vn_is_written(data, vn1) {
            return 0;
        }
        let neg2 = vn_def(data, vn1).expect("negatenegate: vn1 written");
        if op_code(data, neg2) != OpCode::CPUI_INT_NEGATE {
            return 0;
        }
        let vn2 = op_in(data, neg2, 0).expect("negatenegate: neg2 in0");
        if vn_is_free(data, vn2) {
            return 0;
        }
        // data.opSetInput(op,vn2,0);
        data.op_set_input(op, vn2, 0).expect("negatenegate: opSetInput vn2");
        // data.opSetOpcode(op,CPUI_COPY);
        data.op_set_opcode(op, typeop_for(OpCode::CPUI_COPY));
        1
    }
}

// =============================================================================
// RuleConditionalMove (ruleaction.cc:9292)
// =============================================================================

/// `RuleConditionalMove` — simplify various conditional-move situations.
///
/// // SEAM(W4): `constructBool` needs the `CloneBlockOps` expression cloner
/// (`cloneExpression`) and `gatherExpression` ordering, and several arms call
/// `data.opBoolNegate` (unported — it routes through output creation).  The
/// matching/topology (`checkBoolean`, the block-shape analysis up to the CBRANCH
/// discovery) is ported; the transformation arms that require the cloner /
/// `opBoolNegate` bail at the seam (no change).
pub struct RuleConditionalMove;

impl RuleConditionalMove {
    /// Constructor (C++ `RuleConditionalMove(g)`), name "conditionalmove".
    pub fn new() -> RuleConditionalMove {
        RuleConditionalMove
    }

    /// Check if the Varnode is a boolean value; return the root of the expression
    /// (C++ `RuleConditionalMove::checkBoolean`), else `None`.
    fn check_boolean(data: &Funcdata, vn: VarnodeId) -> Option<VarnodeId> {
        if !vn_is_written(data, vn) {
            return None;
        }
        let op = vn_def(data, vn).expect("checkBoolean: vn written");
        if data.obank().get(op).expect("checkBoolean: op").is_bool_output() {
            return Some(vn);
        }
        if op_code(data, op) == OpCode::CPUI_COPY {
            let in0 = op_in(data, op, 0).expect("checkBoolean: COPY in0");
            if vn_is_constant(data, in0) {
                let val = vn_offset(data, in0);
                if (val & !1u64) == 0 {
                    return Some(in0);
                }
            }
        }
        None
    }
}

impl Default for RuleConditionalMove {
    fn default() -> Self {
        RuleConditionalMove::new()
    }
}

impl Rule for RuleConditionalMove {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_MULTIEQUAL]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if grouplist.contains("conditionalexe") {
            Some(Box::new(RuleConditionalMove))
        } else {
            None
        }
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // if (op->numInput() != 2) return 0;
        if op_num_input(data, op) != 2 {
            return 0;
        }
        // Varnode *bool0 = checkBoolean(op->getIn(0));
        let in0 = op_in(data, op, 0).expect("condmove: in0");
        if RuleConditionalMove::check_boolean(data, in0).is_none() {
            return 0;
        }
        // Varnode *bool1 = checkBoolean(op->getIn(1));
        let in1 = op_in(data, op, 1).expect("condmove: in1");
        if RuleConditionalMove::check_boolean(data, in1).is_none() {
            return 0;
        }

        // bb = op->getParent(); inblock0 = bb->getIn(0); ...
        let bb = match data.obank().get(op).expect("condmove: op").get_parent() {
            Some(b) => b,
            None => return 0,
        };
        let inblock0 = block_get_in(data, bb, 0);
        let rootblock0 = if block_size_out(data, inblock0) == 1 {
            if block_size_in(data, inblock0) != 1 {
                return 0;
            }
            block_get_in(data, inblock0, 0)
        } else {
            inblock0
        };
        let inblock1 = block_get_in(data, bb, 1);
        let rootblock1 = if block_size_out(data, inblock1) == 1 {
            if block_size_in(data, inblock1) != 1 {
                return 0;
            }
            block_get_in(data, inblock1, 0)
        } else {
            inblock1
        };
        if rootblock0 != rootblock1 {
            return 0;
        }

        // rootblock must end in CBRANCH
        let cbranch = match data.bb_op_tail(rootblock0) {
            Some(c) => c,
            None => return 0,
        };
        if op_code(data, cbranch) != OpCode::CPUI_CBRANCH {
            return 0;
        }

        // SEAM(W4): the remainder (gatherExpression, constructBool/CloneBlockOps,
        // opBoolNegate, and the MULTIEQUAL rewrites) needs the expression cloner
        // and opBoolNegate, neither ported.  All match conditions up to the
        // CBRANCH discovery are faithful; the rewrite is deferred (no change).
        0
    }
}

// =============================================================================
// RuleFloatCast (ruleaction.cc:9568)
// =============================================================================

/// `RuleFloatCast` — replace `(casttosmall)(casttobig)V` with identity or a
/// single cast.
pub struct RuleFloatCast;

impl RuleFloatCast {
    /// Constructor (C++ `RuleFloatCast(g)`), name "floatcast".
    pub fn new() -> RuleFloatCast {
        RuleFloatCast
    }
}

impl Default for RuleFloatCast {
    fn default() -> Self {
        RuleFloatCast::new()
    }
}

impl Rule for RuleFloatCast {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_FLOAT_FLOAT2FLOAT, OpCode::CPUI_FLOAT_TRUNC]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if grouplist.contains("floatprecision") {
            Some(Box::new(RuleFloatCast))
        } else {
            None
        }
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // Varnode *vn1 = op->getIn(0);
        let vn1 = op_in(data, op, 0).expect("floatcast: in0");
        if !vn_is_written(data, vn1) {
            return 0;
        }
        let castop = vn_def(data, vn1).expect("floatcast: vn1 written");
        let opc2 = op_code(data, castop);
        if opc2 != OpCode::CPUI_FLOAT_FLOAT2FLOAT && opc2 != OpCode::CPUI_FLOAT_INT2FLOAT {
            return 0;
        }
        let opc1 = op_code(data, op);
        let vn2 = op_in(data, castop, 0).expect("floatcast: castop in0");
        let insize1 = vn_size(data, vn1);
        let insize2 = vn_size(data, vn2);
        let outsize = vn_size(data, op_out(data, op).expect("floatcast: out"));

        if vn_is_free(data, vn2) {
            return 0; // Don't propagate free
        }

        if opc2 == OpCode::CPUI_FLOAT_FLOAT2FLOAT && opc1 == OpCode::CPUI_FLOAT_FLOAT2FLOAT {
            if insize1 > outsize {
                // op is superfluous
                data.op_set_input(op, vn2, 0).expect("floatcast: superfluous in0");
                if outsize == insize2 {
                    data.op_set_opcode(op, typeop_for(OpCode::CPUI_COPY)); // identity
                }
                return 1;
            } else if insize2 < insize1 {
                // Convert two increases -> one combined increase
                data.op_set_input(op, vn2, 0).expect("floatcast: combine in0");
                return 1;
            }
        } else if opc2 == OpCode::CPUI_FLOAT_INT2FLOAT && opc1 == OpCode::CPUI_FLOAT_FLOAT2FLOAT {
            // Convert integer straight into final float size
            data.op_set_input(op, vn2, 0).expect("floatcast: int2float in0");
            data.op_set_opcode(op, typeop_for(OpCode::CPUI_FLOAT_INT2FLOAT));
            return 1;
        } else if opc2 == OpCode::CPUI_FLOAT_FLOAT2FLOAT && opc1 == OpCode::CPUI_FLOAT_TRUNC {
            // Convert float straight into final integer
            data.op_set_input(op, vn2, 0).expect("floatcast: trunc in0");
            return 1;
        }

        0
    }
}

// =============================================================================
// RuleIgnoreNan (ruleaction.cc:9624)
// =============================================================================

/// `RuleIgnoreNan` — remove certain NaN operations by assuming their result is
/// always \b false.
///
/// // SEAM(W4): the non-`nan_ignore_all` path needs `TypeOp::isFloatingPointOp`,
/// `functionalEquality`, and `nan_ignore_all` (an `Architecture` flag).  None are
/// ported.  The `nan_ignore_all` arm reads that flag (defaulted off, so this arm
/// is unreachable until W4); the comparison-based arm is deferred to W4.  The
/// `getOpList`/clone are exact.
pub struct RuleIgnoreNan;

impl RuleIgnoreNan {
    /// Constructor (C++ `RuleIgnoreNan(g)`), name "ignorenan".
    pub fn new() -> RuleIgnoreNan {
        RuleIgnoreNan
    }
}

impl Default for RuleIgnoreNan {
    fn default() -> Self {
        RuleIgnoreNan::new()
    }
}

impl Rule for RuleIgnoreNan {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_FLOAT_NAN]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if grouplist.contains("floatprecision") {
            Some(Box::new(RuleIgnoreNan))
        } else {
            None
        }
    }

    fn apply_op(&mut self, _op: OpId, _data: &mut Funcdata) -> int4 {
        // SEAM(W4): if (data.getArch()->nan_ignore_all) { opSetOpcode(op,CPUI_COPY);
        //   opSetInput(op,newConstant(1,0),0); return 1; }
        //   -- nan_ignore_all is an unported Architecture flag (defaults off).
        // The comparison-based removal (testForComparison / checkBackForCompare /
        // isAnotherNan) needs isFloatingPointOp + functionalEquality (W4/W6).
        // No change.
        0
    }
}

// =============================================================================
// Block topology accessors (FlowBlock surface; W3-block)
// =============================================================================

/// `block->getIn(i)` — the i-th predecessor block id.
#[inline]
fn block_get_in(data: &Funcdata, bl: BlockId, i: int4) -> BlockId {
    data.bblocks_ref().block(bl).get_in(i)
}
/// `block->sizeOut()`
#[inline]
fn block_size_out(data: &Funcdata, bl: BlockId) -> int4 {
    data.bblocks_ref().block(bl).size_out()
}
/// `block->sizeIn()`
#[inline]
fn block_size_in(data: &Funcdata, bl: BlockId) -> int4 {
    data.bblocks_ref().block(bl).size_in()
}
/// `block->getTrueOut()`
#[inline]
fn block_get_true_out(data: &Funcdata, bl: BlockId) -> BlockId {
    data.bblocks_ref().block(bl).get_true_out()
}
/// `block->getFalseOut()`
#[inline]
fn block_get_false_out(data: &Funcdata, bl: BlockId) -> BlockId {
    data.bblocks_ref().block(bl).get_false_out()
}

// =============================================================================
// Registration (C++ definition order — W8 assembles universalAction from these)
// =============================================================================

/// Registration rows for the rules in this file, in C++ **definition** order
/// (`RuleSignDiv2` .. `RuleIgnoreNan`, as they appear in ruleaction.cc).  W8's
/// `universal_action` builder reads these to populate the action pools.  Each
/// `group` is the group the C++ `universalAction` constructs the rule with
/// (`coreaction.cc`): the sign/div/mod family + `RuleNegateNegate` are
/// `"analysis"`; `RulePtrFlow` is `"subvar"`; `RuleConditionalMove` is
/// `"conditionalexe"`; `RuleFloatCast`/`RuleIgnoreNan` are `"floatprecision"`;
/// `RuleSegment` is `"segment"`.
pub fn specs() -> Vec<RuleSpec> {
    vec![
        RuleSpec { group: "analysis", ctor: || Box::new(RuleSignDiv2::new()) },
        RuleSpec { group: "analysis", ctor: || Box::new(RuleDivChain::new()) },
        RuleSpec { group: "analysis", ctor: || Box::new(RuleSignForm::new()) },
        RuleSpec { group: "analysis", ctor: || Box::new(RuleSignForm2::new()) },
        RuleSpec { group: "analysis", ctor: || Box::new(RuleSignNearMult::new()) },
        RuleSpec { group: "analysis", ctor: || Box::new(RuleModOpt::new()) },
        RuleSpec { group: "analysis", ctor: || Box::new(RuleSignMod2nOpt::new()) },
        RuleSpec { group: "analysis", ctor: || Box::new(RuleSignMod2Opt::new()) },
        RuleSpec { group: "analysis", ctor: || Box::new(RuleSignMod2nOpt2::new()) },
        RuleSpec { group: "segment", ctor: || Box::new(RuleSegment::new()) },
        RuleSpec { group: "subvar", ctor: || Box::new(RulePtrFlow::new()) },
        RuleSpec { group: "analysis", ctor: || Box::new(RuleNegateNegate::new()) },
        RuleSpec { group: "conditionalexe", ctor: || Box::new(RuleConditionalMove::new()) },
        RuleSpec { group: "floatprecision", ctor: || Box::new(RuleFloatCast::new()) },
        RuleSpec { group: "floatprecision", ctor: || Box::new(RuleIgnoreNan::new()) },
    ]
}

#[cfg(test)]
mod tests;
