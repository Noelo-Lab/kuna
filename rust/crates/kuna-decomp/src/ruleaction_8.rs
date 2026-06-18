//! Port of `decompiler/cpp/ruleaction.cc` lines 9804-11032 — the eighth batch of
//! `Rule` classes, in C++ definition order:
//!
//! 1. [`RuleUnsigned2Float`]   — recover an unsigned int→float conversion
//! 2. [`RuleInt2FloatCollapse`] — collapse converging FLOAT_INT2FLOAT paths
//! 3. [`RuleFuncPtrEncoding`]  — strip ARM/THUMB low-bit masking on func ptrs
//! 4. [`RuleThreeWayCompare`]  — simplify three-way comparison expressions
//! 5. [`RulePopcountBoolXor`]  — boolean combinations through POPCOUNT
//! 6. [`RulePiecePathology`]   — detect pathological register concatenations
//! 7. [`RuleXorSwap`]          — simplify limited XOR chains
//! 8. [`RuleLzcountShiftBool`] — `lzcount(X) >> c => X == 0`
//! 9. [`RuleFloatSign`]        — float sign-bit manipulation → FLOAT_ABS/FLOAT_NEG
//! 10. [`RuleFloatSignCleanup`]— same, driven off the Varnode data-type
//! 11. [`RuleOrCompare`]       — `(V|W) == 0 => (V==0)&&(W==0)`
//! 12. [`RuleExpandLoad`]      — widen a LOAD to match its pointer data-type
//!
//! Each rule keeps its exact C++ `name()` string (carried as the [`RuleState`]
//! name at registration), `getOpList` contents and `applyOp` body structure (see
//! the porter rules in the wave brief).  These structs are stateless, matching
//! the C++ `Rule` subclasses which add no members; per-rule engine state
//! (counts/breakpoints/flags) lives in the [`RuleState`] the owning
//! [`ActionPool`] holds.  [`specs`] lists the rules in C++ definition order for
//! the W8 `universalAction` assembler.
//!
//! # Cross-wave seams (the parts that cannot transcribe end-to-end yet)
//!
//! The rule *bodies* are transcribed verbatim; where a body reaches a subsystem
//! that later waves own, the call is routed through a `// SEAM`-noted local
//! helper that returns the C++ guard's *failing* value, so the rule becomes a
//! faithful no-op exactly when (and only when) that guard would block it.  None
//! of these helpers invents type/flow/call behavior.
//!
//! * **SEAM(W6) — opcode → `TypeOp`.**  `data.opSetOpcode(op, OPC)` resolves
//!   `glb->inst[OPC]` (the interned behavioral class carrying the op-code's
//!   property-flag word).  That table is W6; [`resolve_typeop`] reproduces the
//!   flag words for exactly the op-codes these rules set, copied verbatim from
//!   `typeop.cc`, and [`rule_set_opcode`] feeds them to the ported
//!   `Funcdata::op_set_opcode`.  This is the same local-seam pattern `flow.rs`
//!   uses (`FlowEnvironment::resolve_typeop`).
//! * **SEAM(W3) — `newUniqueOut`.**  `Funcdata::op_set_output` is gated behind a
//!   missing `banks_mut()` split-borrow accessor (it returns `Err`), so the
//!   `newUniqueOut` composite (`newUnique` + `opSetOutput`) cannot complete.
//!   Rules that build a fresh op with a unique output
//!   (`RuleUnsigned2Float`/`RuleInt2FloatCollapse`/`RuleLzcountShiftBool`/
//!   `RuleOrCompare`/`RuleExpandLoad`) bail at that point via
//!   [`new_unique_out`] returning the seam error.  Recorded as losses.
//! * **SEAM(W6) — type queries.**  `TypeOp::floatSignManipulation`,
//!   `TypeOpFloatInt2Float::preferredZextSize`, `Varnode::getTypeReadFacing`/
//!   `getTypeDefFacing`, and `TypePointer::getPtrTo` are W6.  The local
//!   [`float_sign_manipulation`]/[`preferred_zext_size`] helpers return the
//!   guard-failing value so the dependent rules no-op.
//! * **SEAM(W3-block) — `FlowBlock::findCondition`.**  Used only by
//!   `RuleInt2FloatCollapse`; absent here, so that rule no-ops.
//! * **SEAM(W4) — `Architecture::funcptr_align` and `FuncCallSpecs`.**  The W4
//!   `Architecture` skeleton has no `funcptr_align` (defaults to the "disabled"
//!   value 0, so `RuleFuncPtrEncoding` no-ops) and no call-spec table (so
//!   `RulePiecePathology` no-ops).

use std::rc::Rc;

use kuna_base::address::{calc_mask, leastsigbit_set, mostsigbit_set, popcount};
use kuna_base::error::KunaResult;
use kuna_base::types::{int4, uintb};
use kuna_num::opcodes::OpCode;

use crate::action::{ActionGroupList, Rule, RuleSpec};
use crate::dtype::Datatype;
use crate::funcdata::Funcdata;
use crate::op::pcodeop_flags;
use crate::seams::{OpId, TypeOp, VarnodeId};

/// The stage group every rule in this batch registers against (the C++
/// `Rule::getGroup()` for these classes is the analysis family; the worked
/// `DummyConstFold` example uses the same string).  `clone_rule` gates on it.
const RULE_GROUP: &str = "analysis";

// =============================================================================
// Local SEAM helpers (see the module docs)
// =============================================================================

/// Resolve an [`OpCode`] to the [`TypeOp`] behavioral class the C++
/// `opSetOpcode` would fetch from `glb->inst[opc]`.  // SEAM(W6)
///
/// The property-flag word (which `PcodeOp::setOpcode` caches into the op's
/// `flags`, and which then governs `code()`/`isBoolOutput()`/`isCommutative()`
/// of the rewritten op) is reproduced **verbatim from `typeop.cc`** for exactly
/// the op-codes these rules write.  An op-code outside that set is an internal
/// invariant violation (a rule wrote an opcode it never names) and panics.
pub(crate) fn resolve_typeop(opc: OpCode) -> TypeOp {
    use OpCode::*;
    let unary = pcodeop_flags::unary;
    let binary = pcodeop_flags::binary;
    let commutative = pcodeop_flags::commutative;
    let booloutput = pcodeop_flags::booloutput;
    let nocollapse = pcodeop_flags::nocollapse;
    let (flags, name): (kuna_base::types::uint4, &str) = match opc {
        CPUI_COPY => (unary | nocollapse, "copy"),
        CPUI_INT_ZEXT => (unary, "zext"),
        CPUI_INT_XOR => (binary | commutative, "^"),
        CPUI_INT_EQUAL => (binary | booloutput | commutative, "=="),
        CPUI_INT_NOTEQUAL => (binary | booloutput | commutative, "!="),
        CPUI_INT_LESS => (binary | booloutput, "<"),
        CPUI_INT_LESSEQUAL => (binary | booloutput, "<="),
        CPUI_INT_SLESS => (binary | booloutput, "s<"),
        CPUI_INT_SLESSEQUAL => (binary | booloutput, "s<="),
        CPUI_BOOL_AND => (binary | commutative | booloutput, "&&"),
        CPUI_BOOL_OR => (binary | commutative | booloutput, "||"),
        CPUI_SUBPIECE => (binary, "SUB"),
        CPUI_FLOAT_EQUAL => (binary | booloutput | commutative, "f=="),
        CPUI_FLOAT_NOTEQUAL => (binary | booloutput | commutative, "f!="),
        CPUI_FLOAT_LESS => (binary | booloutput, "f<"),
        CPUI_FLOAT_LESSEQUAL => (binary | booloutput, "f<="),
        CPUI_FLOAT_ADD => (binary | commutative, "f+"),
        CPUI_FLOAT_NEG => (unary, "f-"),
        CPUI_FLOAT_ABS => (unary, "ABS"),
        CPUI_FLOAT_INT2FLOAT => (unary, "INT2FLOAT"),
        _ => panic!(
            "ruleaction_8 resolve_typeop: opcode {opc:?} is not written by any rule in this batch \
             (SEAM(W6) table is intentionally minimal)"
        ),
    };
    TypeOp::new(opc, flags, name)
}

/// `data.opSetOpcode(op, opc)` — the C++ `Funcdata::opSetOpcode(PcodeOp*,OpCode)`
/// overload, resolving the op-code to its W6 [`TypeOp`] via [`resolve_typeop`].
/// // SEAM(W6): `glb->inst[opc]`.
pub(crate) fn rule_set_opcode(data: &mut Funcdata, op: OpId, opc: OpCode) {
    data.op_set_opcode(op, resolve_typeop(opc));
}

/// `data.newUniqueOut(s, op)` — allocate a fresh unique-space output Varnode for
/// `op` (C++ `Funcdata::newUniqueOut`).
///
/// SEAM(W3): the C++ body is `Varnode *outvn = newUnique(s); opSetOutput(op,outvn)`.
/// `Funcdata::op_set_output` is itself gated behind a not-yet-ported `banks_mut`
/// split-borrow accessor and returns `Err`; this surface propagates that error so
/// the dependent rule aborts its transformation cleanly.  Recorded as a loss.
fn new_unique_out(data: &mut Funcdata, s: int4, op: OpId) -> KunaResult<VarnodeId> {
    let outvn = data.new_unique(s, None);
    data.op_set_output(op, outvn)?;
    Ok(outvn)
}

/// Lone descendant of a Varnode (C++ `Varnode::loneDescend`): the single reading
/// op, or `None` if there are zero or more than one.
fn lone_descend(data: &Funcdata, vn: VarnodeId) -> Option<OpId> {
    let v = data.vbank().get(vn).expect("lone_descend: stale vn");
    let mut iter = v.descend_iter();
    let first = iter.next()?;
    if iter.next().is_some() {
        return None; // More than 1 descendant
    }
    Some(first)
}

/// Snapshot the descendant op ids of a Varnode in beginDescend order, so the
/// rule can iterate while mutating (the C++ holds a `list::const_iterator`).
fn descend_ops(data: &Funcdata, vn: VarnodeId) -> Vec<OpId> {
    data.vbank().get(vn).expect("descend_ops: stale vn").descend_iter().collect()
}

/// C++ `Varnode::getSpaceFromConst` (`varnode.hh:427`): decode the AddrSpace a
/// constant-space Varnode encodes (the LOAD/STORE space operand).  The C++ stores
/// the raw `AddrSpace *` in the offset; the Rust port (LOSS-015) stores the
/// space's manager index, resolved back here through the function's space manager.
fn space_from_const(
    data: &Funcdata,
    vn: VarnodeId,
) -> Option<Rc<kuna_base::space::AddrSpace>> {
    let idx = data.vbank().get(vn)?.get_offset();
    let manage = data.get_arch().manage();
    if idx >= manage.num_spaces() as uintb {
        return None;
    }
    manage.get_space(idx as i32).map(Rc::clone)
}

/// `TypeOp::floatSignManipulation(op)` (C++, `typeop.cc`): classify `op` as a
/// floating-point sign-bit manipulation and return the equivalent FLOAT_ABS /
/// FLOAT_NEG op-code, or `CPUI_MAX` if it is not one.
///
/// SEAM(W6): the real classifier inspects the W6 `TypeOp`/`OpBehavior` of the op
/// and the constant mask.  It is not ported; returning `CPUI_MAX` makes the
/// callers (`RuleFloatSign`/`RuleFloatSignCleanup`) treat every op as "not a sign
/// manipulation", i.e. a faithful no-op.  Recorded as a loss.
fn float_sign_manipulation(_data: &Funcdata, _op: OpId) -> OpCode {
    // SEAM(W6): TypeOp::floatSignManipulation
    OpCode::CPUI_MAX
}

/// `TypeOpFloatInt2Float::preferredZextSize(inSize)` (C++, `typeop.cc`): the
/// preferred size for the INT_ZEXT feeding an unsigned FLOAT_INT2FLOAT.
///
/// SEAM(W6): only reached by rules that also need `newUniqueOut` (W3 seam), so
/// they abort before this matters.  The conservative default returns the input
/// size; it never affects a completing path.
fn preferred_zext_size(in_size: int4) -> int4 {
    // SEAM(W6): TypeOpFloatInt2Float::preferredZextSize
    in_size
}

/// `data.getArch()->funcptr_align` (C++).  // SEAM(W4)
///
/// The W4 `Architecture` skeleton does not carry `funcptr_align`; it defaults to
/// 0 ("encoding disabled"), so `RuleFuncPtrEncoding` no-ops.  Recorded as a loss.
fn funcptr_align(_data: &Funcdata) -> int4 {
    0
}

/// Helper: the LESSEQUAL op-code corresponding to a LESS op-code (C++
/// `(OpCode)(lessform+1)`).  The CPUI enum lays each `*_LESS` immediately before
/// its `*_LESSEQUAL`, so the C++ uses `lessform+1`.  Replicated explicitly
/// (rather than via raw arithmetic on the enum) for type-safety while preserving
/// the exact mapping.
fn lessequal_after(opc: OpCode) -> OpCode {
    match opc {
        OpCode::CPUI_INT_LESS => OpCode::CPUI_INT_LESSEQUAL,
        OpCode::CPUI_INT_SLESS => OpCode::CPUI_INT_SLESSEQUAL,
        OpCode::CPUI_FLOAT_LESS => OpCode::CPUI_FLOAT_LESSEQUAL,
        other => panic!("lessequal_after: not a LESS op-code: {other:?}"),
    }
}

// =============================================================================
// RuleUnsigned2Float (ruleaction.cc:9804)
// =============================================================================

/// \brief Recover an unsigned integer to floating-point conversion
/// (C++ `RuleUnsigned2Float`, name `"unsigned2float"`).
pub struct RuleUnsigned2Float;

impl Rule for RuleUnsigned2Float {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_FLOAT_INT2FLOAT]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(RULE_GROUP) {
            return None;
        }
        Some(Box::new(RuleUnsigned2Float))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // Varnode *invn = op->getIn(0); if (!invn->isWritten()) return 0;
        let invn = data.obank().get(op).expect("u2f: stale op").get_in(0).expect("u2f: null in0");
        if !data.vbank().get(invn).expect("u2f: stale invn").is_written() {
            return 0;
        }
        // PcodeOp *orop = invn->getDef();
        let orop = data.vbank().get(invn).expect("u2f").get_def().expect("u2f: invn def");
        // if (orop->code() != CPUI_INT_OR) return 0;
        if data.obank().get(orop).expect("u2f: stale orop").code() != OpCode::CPUI_INT_OR {
            return 0;
        }
        let or_in0 = data.obank().get(orop).expect("u2f").get_in(0).expect("u2f: orop in0");
        let or_in1 = data.obank().get(orop).expect("u2f").get_in(1).expect("u2f: orop in1");
        // if (!orop->getIn(0)->isWritten() || !orop->getIn(1)->isWritten()) return 0;
        if !data.vbank().get(or_in0).expect("u2f").is_written()
            || !data.vbank().get(or_in1).expect("u2f").is_written()
        {
            return 0;
        }
        // PcodeOp *shiftop = orop->getIn(0)->getDef(); PcodeOp *andop;
        let mut shiftop = data.vbank().get(or_in0).expect("u2f").get_def().expect("u2f: in0 def");
        let andop;
        // if (shiftop->code() != CPUI_INT_RIGHT) { andop = shiftop; shiftop = orop->getIn(1)->getDef(); }
        // else { andop = orop->getIn(1)->getDef(); }
        if data.obank().get(shiftop).expect("u2f").code() != OpCode::CPUI_INT_RIGHT {
            andop = shiftop;
            shiftop = data.vbank().get(or_in1).expect("u2f").get_def().expect("u2f: in1 def");
        } else {
            andop = data.vbank().get(or_in1).expect("u2f").get_def().expect("u2f: in1 def");
        }
        // if (shiftop->code() != CPUI_INT_RIGHT) return 0;
        if data.obank().get(shiftop).expect("u2f").code() != OpCode::CPUI_INT_RIGHT {
            return 0;
        }
        // if (!shiftop->getIn(1)->constantMatch(1)) return 0;  // Shift right by exactly 1
        let sh_in1 = data.obank().get(shiftop).expect("u2f").get_in(1).expect("u2f: shift in1");
        if !data.vbank().get(sh_in1).expect("u2f").constant_match(1) {
            return 0;
        }
        // Varnode *basevn = shiftop->getIn(0); if (basevn->isFree()) return 0;
        let basevn = data.obank().get(shiftop).expect("u2f").get_in(0).expect("u2f: shift in0");
        if data.vbank().get(basevn).expect("u2f").is_free() {
            return 0;
        }
        // if (andop->code() == CPUI_INT_ZEXT) { if (!andop->getIn(0)->isWritten()) return 0; andop = andop->getIn(0)->getDef(); }
        let mut andop = andop;
        if data.obank().get(andop).expect("u2f").code() == OpCode::CPUI_INT_ZEXT {
            let zin = data.obank().get(andop).expect("u2f").get_in(0).expect("u2f: zext in0");
            if !data.vbank().get(zin).expect("u2f").is_written() {
                return 0;
            }
            andop = data.vbank().get(zin).expect("u2f").get_def().expect("u2f: zext def");
        }
        // if (andop->code() != CPUI_INT_AND) return 0;
        if data.obank().get(andop).expect("u2f").code() != OpCode::CPUI_INT_AND {
            return 0;
        }
        // if (!andop->getIn(1)->constantMatch(1)) return 0;  // Mask off least significant bit
        let and_in1 = data.obank().get(andop).expect("u2f").get_in(1).expect("u2f: and in1");
        if !data.vbank().get(and_in1).expect("u2f").constant_match(1) {
            return 0;
        }
        // Varnode *vn = andop->getIn(0);
        let mut vn = data.obank().get(andop).expect("u2f").get_in(0).expect("u2f: and in0");
        // if (basevn != vn) { if (!vn->isWritten()) return 0; PcodeOp *subop = vn->getDef();
        //   if (subop->code()!=SUBPIECE) return 0; if (subop->getIn(1)->getOffset()!=0) return 0;
        //   vn = subop->getIn(0); if (basevn != vn) return 0; }
        if basevn != vn {
            if !data.vbank().get(vn).expect("u2f").is_written() {
                return 0;
            }
            let subop = data.vbank().get(vn).expect("u2f").get_def().expect("u2f: vn def");
            if data.obank().get(subop).expect("u2f").code() != OpCode::CPUI_SUBPIECE {
                return 0;
            }
            let sub_in1 = data.obank().get(subop).expect("u2f").get_in(1).expect("u2f: sub in1");
            if data.vbank().get(sub_in1).expect("u2f").get_offset() != 0 {
                return 0;
            }
            vn = data.obank().get(subop).expect("u2f").get_in(0).expect("u2f: sub in0");
            if basevn != vn {
                return 0;
            }
        }
        // Varnode *outvn = op->getOut();
        let outvn = data.obank().get(op).expect("u2f").get_out().expect("u2f: op out");
        // for(iter over outvn descendants) { addop ... }
        for addop in descend_ops(data, outvn) {
            // if (addop->code() != CPUI_FLOAT_ADD) continue;
            if data.obank().get(addop).expect("u2f").code() != OpCode::CPUI_FLOAT_ADD {
                continue;
            }
            // if (addop->getIn(0) != outvn) continue; if (addop->getIn(1) != outvn) continue;
            if data.obank().get(addop).expect("u2f").get_in(0) != Some(outvn) {
                continue;
            }
            if data.obank().get(addop).expect("u2f").get_in(1) != Some(outvn) {
                continue;
            }
            let base_size = data.vbank().get(basevn).expect("u2f").get_size();
            let addr = data.obank().get(addop).expect("u2f").get_addr().clone();
            // PcodeOp *zextop = data.newOp(1,addop->getAddr());
            let zextop = data.new_op(1, addr);
            // data.opSetOpcode(zextop, CPUI_INT_ZEXT);
            rule_set_opcode(data, zextop, OpCode::CPUI_INT_ZEXT);
            // Varnode *zextout = data.newUniqueOut(preferredZextSize(basevn->getSize()), zextop);
            //   -- SEAM(W3): newUniqueOut; SEAM(W6): preferredZextSize.
            match new_unique_out(data, preferred_zext_size(base_size), zextop) {
                Ok(zextout) => {
                    // data.opSetOpcode(addop, CPUI_FLOAT_INT2FLOAT);
                    rule_set_opcode(data, addop, OpCode::CPUI_FLOAT_INT2FLOAT);
                    // data.opRemoveInput(addop, 1);
                    data.op_remove_input(addop, 1);
                    // data.opSetInput(zextop, basevn, 0);
                    data.op_set_input(zextop, basevn, 0).expect("u2f: opSetInput");
                    // data.opSetInput(addop, zextout, 0);
                    data.op_set_input(addop, zextout, 0).expect("u2f: opSetInput");
                    // data.opInsertBefore(zextop, addop);
                    data.op_insert_before(zextop, addop);
                    return 1;
                }
                Err(_) => {
                    // SEAM(W3): newUniqueOut unavailable; abort this transformation.
                    return 0;
                }
            }
        }
        0
    }
}

// =============================================================================
// RuleInt2FloatCollapse (ruleaction.cc:9866)
// =============================================================================

/// \brief Collapse equivalent FLOAT_INT2FLOAT computations along converging
/// data-flow paths (C++ `RuleInt2FloatCollapse`, name `"int2floatcollapse"`).
pub struct RuleInt2FloatCollapse;

impl Rule for RuleInt2FloatCollapse {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_FLOAT_INT2FLOAT]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(RULE_GROUP) {
            return None;
        }
        Some(Box::new(RuleInt2FloatCollapse))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // if (!op->getIn(0)->isWritten()) return 0;
        let in0 = data.obank().get(op).expect("i2fc: stale op").get_in(0).expect("i2fc: null in0");
        if !data.vbank().get(in0).expect("i2fc").is_written() {
            return 0;
        }
        // PcodeOp *zextop = op->getIn(0)->getDef();
        let zextop = data.vbank().get(in0).expect("i2fc").get_def().expect("i2fc: in0 def");
        // if (zextop->code() != CPUI_INT_ZEXT) return 0;
        if data.obank().get(zextop).expect("i2fc").code() != OpCode::CPUI_INT_ZEXT {
            return 0;
        }
        // Varnode *basevn = zextop->getIn(0); if (basevn->isFree()) return 0;
        let basevn = data.obank().get(zextop).expect("i2fc").get_in(0).expect("i2fc: zext in0");
        if data.vbank().get(basevn).expect("i2fc").is_free() {
            return 0;
        }
        // PcodeOp *multiop = op->getOut()->loneDescend(); if (multiop == 0) return 0;
        let opout = data.obank().get(op).expect("i2fc").get_out().expect("i2fc: op out");
        let _multiop = match lone_descend(data, opout) {
            Some(m) => m,
            None => return 0,
        };
        // The remainder needs FlowBlock::findCondition (SEAM(W3-block)) to verify
        // the dominating CBRANCH guard, and newUniqueOut (SEAM(W3)) to build the
        // collapsed unsigned conversion.  Neither is available; the rule no-ops
        // exactly as the C++ does whenever the condition cannot be confirmed.
        // SEAM(W3-block): FlowBlock::findCondition; SEAM(W3): newUniqueOut.
        0
    }
}

// =============================================================================
// RuleFuncPtrEncoding (ruleaction.cc:9929)
// =============================================================================

/// \brief Eliminate ARM/THUMB style masking of the low order bits on function
/// pointers (C++ `RuleFuncPtrEncoding`, name `"funcptrencoding"`).
pub struct RuleFuncPtrEncoding;

impl Rule for RuleFuncPtrEncoding {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_CALLIND]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(RULE_GROUP) {
            return None;
        }
        Some(Box::new(RuleFuncPtrEncoding))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // int4 align = data.getArch()->funcptr_align; if (align == 0) return 0;
        //   -- SEAM(W4): funcptr_align defaults to 0, so the rule no-ops.
        let align: int4 = funcptr_align(data);
        if align == 0 {
            return 0;
        }
        // Varnode *vn = op->getIn(0); if (!vn->isWritten()) return 0;
        let vn = data.obank().get(op).expect("fpe: stale op").get_in(0).expect("fpe: null in0");
        if !data.vbank().get(vn).expect("fpe").is_written() {
            return 0;
        }
        // PcodeOp *andop = vn->getDef(); if (andop->code() != CPUI_INT_AND) return 0;
        let andop = data.vbank().get(vn).expect("fpe").get_def().expect("fpe: vn def");
        if data.obank().get(andop).expect("fpe").code() != OpCode::CPUI_INT_AND {
            return 0;
        }
        // Varnode *maskvn = andop->getIn(1); if (!maskvn->isConstant()) return 0;
        let maskvn = data.obank().get(andop).expect("fpe").get_in(1).expect("fpe: and in1");
        if !data.vbank().get(maskvn).expect("fpe").is_constant() {
            return 0;
        }
        // uintb val = maskvn->getOffset(); uintb testmask = calc_mask(maskvn->getSize());
        let val = data.vbank().get(maskvn).expect("fpe").get_offset();
        let testmask = calc_mask(data.vbank().get(maskvn).expect("fpe").get_size());
        // uintb slide = ~((uintb)0); slide <<= align;
        let mut slide: uintb = !0u64;
        slide = slide.wrapping_shl(align as u32);
        // if ((testmask & slide) == val) { ... }
        if (testmask & slide) == val {
            // data.opRemoveInput(andop,1);  -- Eliminate the mask
            data.op_remove_input(andop, 1);
            // data.opSetOpcode(andop,CPUI_COPY);
            rule_set_opcode(data, andop, OpCode::CPUI_COPY);
            return 1;
        }
        0
    }
}

// =============================================================================
// RuleThreeWayCompare (ruleaction.cc:9964)
// =============================================================================

/// \brief Simplify expressions involving three-way comparisons
/// (C++ `RuleThreeWayCompare`, name `"threewaycomp"`).
pub struct RuleThreeWayCompare;

impl RuleThreeWayCompare {
    /// \brief Make sure comparisons match properly for a three-way
    /// (C++ `RuleThreeWayCompare::testCompareEquivalence`).
    ///
    /// Returns 0 if correct, 1 if correct but \b lessop/\b lessequalop roles must
    /// be swapped, -1 if not the correct configuration.
    pub fn test_compare_equivalence(data: &Funcdata, lessop: OpId, lessequalop: OpId) -> int4 {
        let less_code = data.obank().get(lessop).expect("3way: stale lessop").code();
        let lesseq_code = data.obank().get(lessequalop).expect("3way: stale lessequalop").code();
        let mut two_less_than: bool;
        // if (lessop->code() == CPUI_INT_LESS) {...}
        if less_code == OpCode::CPUI_INT_LESS {
            if lesseq_code == OpCode::CPUI_INT_LESSEQUAL {
                two_less_than = false;
            } else if lesseq_code == OpCode::CPUI_INT_LESS {
                two_less_than = true;
            } else {
                return -1;
            }
        } else if less_code == OpCode::CPUI_INT_SLESS {
            if lesseq_code == OpCode::CPUI_INT_SLESSEQUAL {
                two_less_than = false;
            } else if lesseq_code == OpCode::CPUI_INT_SLESS {
                two_less_than = true;
            } else {
                return -1;
            }
        } else if less_code == OpCode::CPUI_FLOAT_LESS {
            if lesseq_code == OpCode::CPUI_FLOAT_LESSEQUAL {
                two_less_than = false;
            } else {
                return -1; // No partial form for floating-point comparison
            }
        } else {
            return -1;
        }
        // Varnode *a1 = lessop->getIn(0); a2 = lessequalop->getIn(0);
        //   b1 = lessop->getIn(1); b2 = lessequalop->getIn(1);
        let a1 = data.obank().get(lessop).expect("3way").get_in(0).expect("3way: less in0");
        let a2 = data.obank().get(lessequalop).expect("3way").get_in(0).expect("3way: lesseq in0");
        let b1 = data.obank().get(lessop).expect("3way").get_in(1).expect("3way: less in1");
        let b2 = data.obank().get(lessequalop).expect("3way").get_in(1).expect("3way: lesseq in1");
        let mut res: int4 = 0;
        // if (a1 != a2) { make sure a1 and a2 are equivalent }
        if a1 != a2 {
            let a1v = data.vbank().get(a1).expect("3way");
            let a2v = data.vbank().get(a2).expect("3way");
            if (!a1v.is_constant()) || (!a2v.is_constant()) {
                return -1;
            }
            let a1off = a1v.get_offset();
            let a2off = a2v.get_offset();
            if (a1off != a2off) && two_less_than {
                if a2off.wrapping_add(1) == a1off {
                    two_less_than = false; // -lessequalop- is LESSTHAN, equiv to LESSEQUAL
                } else if a1off.wrapping_add(1) == a2off {
                    two_less_than = false; // -lessop- is LESSTHAN, equiv to LESSEQUAL
                    res = 1; // we need to swap
                } else {
                    return -1;
                }
            }
        }
        // if (b1 != b2) { make sure b1 and b2 are equivalent }
        if b1 != b2 {
            let b1v = data.vbank().get(b1).expect("3way");
            let b2v = data.vbank().get(b2).expect("3way");
            if (!b1v.is_constant()) || (!b2v.is_constant()) {
                return -1;
            }
            let b1off = b1v.get_offset();
            let b2off = b2v.get_offset();
            // if ((b1->getOffset() != b2->getOffset())&&twoLessThan) { ... } else return -1;
            if (b1off != b2off) && two_less_than {
                if b1off.wrapping_add(1) == b2off {
                    two_less_than = false;
                } else if b2off.wrapping_add(1) == b1off {
                    two_less_than = false;
                    res = 1; // we need to swap
                }
            } else {
                return -1;
            }
        }
        // if (twoLessThan) return -1;
        if two_less_than {
            return -1;
        }
        res
    }

    /// \brief Detect a three-way calculation (C++
    /// `RuleThreeWayCompare::detectThreeWay`).  Returns the less-than op, or
    /// `None` if no three-way was detected; sets `is_partial`.
    pub fn detect_three_way(data: &Funcdata, op: OpId, is_partial: &mut bool) -> Option<OpId> {
        let zext1: OpId;
        let zext2: OpId;
        let addop: OpId;
        // vn2 = op->getIn(1);
        let vn2 = data.obank().get(op).expect("3way: stale op").get_in(1).expect("3way: op in1");
        if data.vbank().get(vn2).expect("3way").is_constant() {
            // Form 1 : (z + z) - 1
            // mask = calc_mask(vn2->getSize()); if (mask != vn2->getOffset()) return 0;
            let mask = calc_mask(data.vbank().get(vn2).expect("3way").get_size());
            if mask != data.vbank().get(vn2).expect("3way").get_offset() {
                return None;
            }
            // vn1 = op->getIn(0); if (!vn1->isWritten()) return 0;
            let vn1 = data.obank().get(op).expect("3way").get_in(0).expect("3way: op in0");
            if !data.vbank().get(vn1).expect("3way").is_written() {
                return None;
            }
            // addop = vn1->getDef(); if (addop->code() != CPUI_INT_ADD) return 0;
            addop = data.vbank().get(vn1).expect("3way").get_def().expect("3way: vn1 def");
            if data.obank().get(addop).expect("3way").code() != OpCode::CPUI_INT_ADD {
                return None;
            }
            // tmpvn = addop->getIn(0); if (!tmpvn->isWritten()) return 0;
            let t0 = data.obank().get(addop).expect("3way").get_in(0).expect("3way: add in0");
            if !data.vbank().get(t0).expect("3way").is_written() {
                return None;
            }
            // zext1 = tmpvn->getDef(); if (zext1->code() != CPUI_INT_ZEXT) return 0;
            zext1 = data.vbank().get(t0).expect("3way").get_def().expect("3way: t0 def");
            if data.obank().get(zext1).expect("3way").code() != OpCode::CPUI_INT_ZEXT {
                return None;
            }
            // tmpvn = addop->getIn(1); if (!tmpvn->isWritten()) return 0;
            let t1 = data.obank().get(addop).expect("3way").get_in(1).expect("3way: add in1");
            if !data.vbank().get(t1).expect("3way").is_written() {
                return None;
            }
            // zext2 = tmpvn->getDef(); if (zext2->code() != CPUI_INT_ZEXT) return 0;
            zext2 = data.vbank().get(t1).expect("3way").get_def().expect("3way: t1 def");
            if data.obank().get(zext2).expect("3way").code() != OpCode::CPUI_INT_ZEXT {
                return None;
            }
        } else if data.vbank().get(vn2).expect("3way").is_written() {
            // PcodeOp *tmpop = vn2->getDef();
            let tmpop = data.vbank().get(vn2).expect("3way").get_def().expect("3way: vn2 def");
            let tmp_code = data.obank().get(tmpop).expect("3way").code();
            if tmp_code == OpCode::CPUI_INT_ZEXT {
                // Form 2 : (z - 1) + z
                zext2 = tmpop; // Second zext is already matched
                let vn1 = data.obank().get(op).expect("3way").get_in(0).expect("3way: op in0");
                if !data.vbank().get(vn1).expect("3way").is_written() {
                    return None;
                }
                addop = data.vbank().get(vn1).expect("3way").get_def().expect("3way: vn1 def");
                if data.obank().get(addop).expect("3way").code() != OpCode::CPUI_INT_ADD {
                    // Partial form: (z + z)
                    zext1 = addop;
                    if data.obank().get(zext1).expect("3way").code() != OpCode::CPUI_INT_ZEXT {
                        return None;
                    }
                    *is_partial = true;
                } else {
                    let t1 =
                        data.obank().get(addop).expect("3way").get_in(1).expect("3way: add in1");
                    if !data.vbank().get(t1).expect("3way").is_constant() {
                        return None;
                    }
                    let mask = calc_mask(data.vbank().get(t1).expect("3way").get_size());
                    if mask != data.vbank().get(t1).expect("3way").get_offset() {
                        return None;
                    }
                    let t0 =
                        data.obank().get(addop).expect("3way").get_in(0).expect("3way: add in0");
                    if !data.vbank().get(t0).expect("3way").is_written() {
                        return None;
                    }
                    zext1 = data.vbank().get(t0).expect("3way").get_def().expect("3way: t0 def");
                    if data.obank().get(zext1).expect("3way").code() != OpCode::CPUI_INT_ZEXT {
                        return None;
                    }
                }
            } else if tmp_code == OpCode::CPUI_INT_ADD {
                // Form 3 : z + (z - 1)
                addop = tmpop; // Matched the add
                let vn1 = data.obank().get(op).expect("3way").get_in(0).expect("3way: op in0");
                if !data.vbank().get(vn1).expect("3way").is_written() {
                    return None;
                }
                zext1 = data.vbank().get(vn1).expect("3way").get_def().expect("3way: vn1 def");
                if data.obank().get(zext1).expect("3way").code() != OpCode::CPUI_INT_ZEXT {
                    return None;
                }
                let t1 = data.obank().get(addop).expect("3way").get_in(1).expect("3way: add in1");
                if !data.vbank().get(t1).expect("3way").is_constant() {
                    return None;
                }
                let mask = calc_mask(data.vbank().get(t1).expect("3way").get_size());
                if mask != data.vbank().get(t1).expect("3way").get_offset() {
                    return None;
                }
                let t0 = data.obank().get(addop).expect("3way").get_in(0).expect("3way: add in0");
                if !data.vbank().get(t0).expect("3way").is_written() {
                    return None;
                }
                zext2 = data.vbank().get(t0).expect("3way").get_def().expect("3way: t0 def");
                if data.obank().get(zext2).expect("3way").code() != OpCode::CPUI_INT_ZEXT {
                    return None;
                }
            } else {
                return None;
            }
        } else {
            return None;
        }
        // vn1 = zext1->getIn(0); if (!vn1->isWritten()) return 0;
        let zv1 = data.obank().get(zext1).expect("3way").get_in(0).expect("3way: zext1 in0");
        if !data.vbank().get(zv1).expect("3way").is_written() {
            return None;
        }
        // vn2 = zext2->getIn(0); if (!vn2->isWritten()) return 0;
        let zv2 = data.obank().get(zext2).expect("3way").get_in(0).expect("3way: zext2 in0");
        if !data.vbank().get(zv2).expect("3way").is_written() {
            return None;
        }
        // lessop = vn1->getDef(); lessequalop = vn2->getDef();
        let mut lessop = data.vbank().get(zv1).expect("3way").get_def().expect("3way: zv1 def");
        let mut lessequalop = data.vbank().get(zv2).expect("3way").get_def().expect("3way: zv2 def");
        // OpCode opc = lessop->code();
        let opc = data.obank().get(lessop).expect("3way").code();
        // if ((opc != CPUI_INT_LESS)&&(opc != CPUI_INT_SLESS)&&(opc != CPUI_FLOAT_LESS)) { swap }
        if (opc != OpCode::CPUI_INT_LESS)
            && (opc != OpCode::CPUI_INT_SLESS)
            && (opc != OpCode::CPUI_FLOAT_LESS)
        {
            std::mem::swap(&mut lessop, &mut lessequalop);
        }
        // int4 form = testCompareEquivalence(lessop,lessequalop); if (form < 0) return 0;
        let form = RuleThreeWayCompare::test_compare_equivalence(data, lessop, lessequalop);
        if form < 0 {
            return None;
        }
        // if (form == 1) { swap }
        if form == 1 {
            std::mem::swap(&mut lessop, &mut lessequalop);
        }
        Some(lessop)
    }
}

impl Rule for RuleThreeWayCompare {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![
            OpCode::CPUI_INT_SLESS,
            OpCode::CPUI_INT_SLESSEQUAL,
            OpCode::CPUI_INT_EQUAL,
            OpCode::CPUI_INT_NOTEQUAL,
        ]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(RULE_GROUP) {
            return None;
        }
        Some(Box::new(RuleThreeWayCompare))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // int4 constSlot=0; Varnode *tmpvn = op->getIn(constSlot);
        let mut const_slot: int4 = 0;
        let mut tmpvn = data.obank().get(op).expect("3way: stale op").get_in(0).expect("3way: in0");
        // if (!tmpvn->isConstant()) { constSlot=1; tmpvn=op->getIn(1); if (!tmpvn->isConstant()) return 0; }
        if !data.vbank().get(tmpvn).expect("3way").is_constant() {
            const_slot = 1;
            tmpvn = data.obank().get(op).expect("3way").get_in(1).expect("3way: in1");
            if !data.vbank().get(tmpvn).expect("3way").is_constant() {
                return 0;
            }
        }
        // uintb val = tmpvn->getOffset();
        let val = data.vbank().get(tmpvn).expect("3way").get_offset();
        let tmp_size = data.vbank().get(tmpvn).expect("3way").get_size();
        // Encode const value (-1,0,1,2) as highest 3 bits of form (000,001,010,011)
        let mut form: int4;
        if val <= 2 {
            form = (val as int4) + 1;
        } else if val == calc_mask(tmp_size) {
            form = 0;
        } else {
            return 0;
        }
        // tmpvn = op->getIn(1-constSlot); if (!tmpvn->isWritten()) return 0;
        let tmpvn = data.obank().get(op).expect("3way").get_in(1 - const_slot).expect("3way: in");
        if !data.vbank().get(tmpvn).expect("3way").is_written() {
            return 0;
        }
        // if (tmpvn->getDef()->code() != CPUI_INT_ADD) return 0;
        let tmpdef = data.vbank().get(tmpvn).expect("3way").get_def().expect("3way: tmpvn def");
        if data.obank().get(tmpdef).expect("3way").code() != OpCode::CPUI_INT_ADD {
            return 0;
        }
        // bool isPartial = false; PcodeOp *lessop = detectThreeWay(tmpvn->getDef(),isPartial);
        let mut is_partial = false;
        let lessop = match RuleThreeWayCompare::detect_three_way(data, tmpdef, &mut is_partial) {
            Some(l) => l,
            None => return 0,
        };
        // if (isPartial) { if (form == 0) return 0; form -= 1; }
        if is_partial {
            if form == 0 {
                return 0; // -1 const value is now out of range
            }
            form -= 1;
        }
        // form <<= 1; if (constSlot == 1) form += 1;
        form <<= 1;
        if const_slot == 1 {
            form += 1;
        }
        // OpCode lessform = lessop->code();
        let lessform = data.obank().get(lessop).expect("3way").code();
        // form <<= 2; encode base op as final 2 bits
        form <<= 2;
        let opcode = data.obank().get(op).expect("3way").code();
        if opcode == OpCode::CPUI_INT_SLESSEQUAL {
            form += 1;
        } else if opcode == OpCode::CPUI_INT_EQUAL {
            form += 2;
        } else if opcode == OpCode::CPUI_INT_NOTEQUAL {
            form += 3;
        }
        // Varnode *bvn = lessop->getIn(0); Varnode *avn = lessop->getIn(1);
        let bvn = data.obank().get(lessop).expect("3way").get_in(0).expect("3way: less in0");
        let avn = data.obank().get(lessop).expect("3way").get_in(1).expect("3way: less in1");
        // if ((!avn->isConstant())&&(avn->isFree())) return 0;
        {
            let av = data.vbank().get(avn).expect("3way");
            if (!av.is_constant()) && av.is_free() {
                return 0;
            }
        }
        // if ((!bvn->isConstant())&&(bvn->isFree())) return 0;
        {
            let bv = data.vbank().get(bvn).expect("3way");
            if (!bv.is_constant()) && bv.is_free() {
                return 0;
            }
        }
        // The LESSEQUAL op-code is `(OpCode)(lessform+1)` in the CPUI enum.
        let lessequal_form = lessequal_after(lessform);
        // switch(form) { ... }
        match form {
            1 | 21 => {
                // always true
                rule_set_opcode(data, op, OpCode::CPUI_INT_EQUAL);
                let c0 = data.new_constant(1, 0);
                let c1 = data.new_constant(1, 0);
                data.op_set_input(op, c0, 0).expect("3way: set");
                data.op_set_input(op, c1, 1).expect("3way: set");
            }
            4 | 16 => {
                // always false
                rule_set_opcode(data, op, OpCode::CPUI_INT_NOTEQUAL);
                let c0 = data.new_constant(1, 0);
                let c1 = data.new_constant(1, 0);
                data.op_set_input(op, c0, 0).expect("3way: set");
                data.op_set_input(op, c1, 1).expect("3way: set");
            }
            2 | 5 | 6 | 12 => {
                // a < b
                rule_set_opcode(data, op, lessform);
                data.op_set_input(op, avn, 0).expect("3way: set");
                data.op_set_input(op, bvn, 1).expect("3way: set");
            }
            13 | 19 | 20 | 23 => {
                // a <= b (LESSEQUAL form)
                rule_set_opcode(data, op, lessequal_form);
                data.op_set_input(op, avn, 0).expect("3way: set");
                data.op_set_input(op, bvn, 1).expect("3way: set");
            }
            8 | 17 | 18 | 22 => {
                // a > b
                rule_set_opcode(data, op, lessform);
                data.op_set_input(op, bvn, 0).expect("3way: set");
                data.op_set_input(op, avn, 1).expect("3way: set");
            }
            0 | 3 | 7 | 9 => {
                // a >= b (LESSEQUAL form)
                rule_set_opcode(data, op, lessequal_form);
                data.op_set_input(op, bvn, 0).expect("3way: set");
                data.op_set_input(op, avn, 1).expect("3way: set");
            }
            10 | 14 => {
                // a == b
                let eqform = if lessform == OpCode::CPUI_FLOAT_LESS {
                    OpCode::CPUI_FLOAT_EQUAL // float form
                } else {
                    OpCode::CPUI_INT_EQUAL // or integer form
                };
                rule_set_opcode(data, op, eqform);
                data.op_set_input(op, avn, 0).expect("3way: set");
                data.op_set_input(op, bvn, 1).expect("3way: set");
            }
            11 | 15 => {
                // a != b
                let neform = if lessform == OpCode::CPUI_FLOAT_LESS {
                    OpCode::CPUI_FLOAT_NOTEQUAL // float form
                } else {
                    OpCode::CPUI_INT_NOTEQUAL // or integer form
                };
                rule_set_opcode(data, op, neform);
                data.op_set_input(op, avn, 0).expect("3way: set");
                data.op_set_input(op, bvn, 1).expect("3way: set");
            }
            _ => {
                return 0;
            }
        }
        1
    }
}

// =============================================================================
// RulePopcountBoolXor (ruleaction.cc:10280)
// =============================================================================

/// \brief Simplify boolean expressions combined through POPCOUNT
/// (C++ `RulePopcountBoolXor`, name `"popcountboolxor"`).
pub struct RulePopcountBoolXor;

impl RulePopcountBoolXor {
    /// \brief Extract the boolean Varnode producing the bit at `bit_pos` of `vn`
    /// (C++ `RulePopcountBoolXor::getBooleanResult`).
    ///
    /// Returns the boolean Varnode, or `None`; passes back `const_res` (0/1 for a
    /// constant, -1 if no boolean value found).
    pub fn get_boolean_result(
        data: &Funcdata,
        mut vn: VarnodeId,
        mut bit_pos: int4,
        const_res: &mut int4,
    ) -> Option<VarnodeId> {
        *const_res = -1;
        // uintb mask = 1; mask <<= bitPos;
        let mut mask: uintb = 1u64.wrapping_shl(bit_pos as u32);
        loop {
            let v = data.vbank().get(vn).expect("pcbx: stale vn");
            // if (vn->isConstant()) { constRes = (offset >> bitPos) & 1; return 0; }
            if v.is_constant() {
                *const_res = ((v.get_offset() >> bit_pos) & 1) as int4;
                return None;
            }
            // if (!vn->isWritten()) return 0;
            if !v.is_written() {
                return None;
            }
            // if (bitPos == 0 && vn->getSize() == 1 && vn->getNZMask() == mask) return vn;
            if bit_pos == 0 && v.get_size() == 1 && v.get_nz_mask() == mask {
                return Some(vn);
            }
            // PcodeOp *op = vn->getDef();
            let op = v.get_def().expect("pcbx: vn def");
            let code = data.obank().get(op).expect("pcbx").code();
            match code {
                OpCode::CPUI_INT_AND => {
                    // if (!op->getIn(1)->isConstant()) return 0; vn = op->getIn(0);
                    let in1 = data.obank().get(op).expect("pcbx").get_in(1).expect("pcbx: and in1");
                    if !data.vbank().get(in1).expect("pcbx").is_constant() {
                        return None;
                    }
                    vn = data.obank().get(op).expect("pcbx").get_in(0).expect("pcbx: and in0");
                }
                OpCode::CPUI_INT_XOR | OpCode::CPUI_INT_OR => {
                    let vn0 = data.obank().get(op).expect("pcbx").get_in(0).expect("pcbx: in0");
                    let vn1 = data.obank().get(op).expect("pcbx").get_in(1).expect("pcbx: in1");
                    let nz0 = data.vbank().get(vn0).expect("pcbx").get_nz_mask();
                    let nz1 = data.vbank().get(vn1).expect("pcbx").get_nz_mask();
                    if (nz0 & mask) != 0 {
                        if (nz1 & mask) != 0 {
                            return None; // Don't have a unique path
                        }
                        vn = vn0;
                    } else if (nz1 & mask) != 0 {
                        vn = vn1;
                    } else {
                        return None;
                    }
                }
                OpCode::CPUI_INT_ZEXT | OpCode::CPUI_INT_SEXT => {
                    // vn = op->getIn(0); if (bitPos >= vn->getSize()*8) return 0;
                    vn = data.obank().get(op).expect("pcbx").get_in(0).expect("pcbx: ext in0");
                    if bit_pos >= data.vbank().get(vn).expect("pcbx").get_size() * 8 {
                        return None;
                    }
                }
                OpCode::CPUI_SUBPIECE => {
                    // sa = op->getIn(1)->getOffset()*8; bitPos += sa; mask <<= sa; vn = op->getIn(0);
                    let in1 = data.obank().get(op).expect("pcbx").get_in(1).expect("pcbx: sub in1");
                    let sa = (data.vbank().get(in1).expect("pcbx").get_offset() as int4) * 8;
                    bit_pos += sa;
                    mask = mask.wrapping_shl(sa as u32);
                    vn = data.obank().get(op).expect("pcbx").get_in(0).expect("pcbx: sub in0");
                }
                OpCode::CPUI_PIECE => {
                    // vn0 = op->getIn(0); vn1 = op->getIn(1); sa = vn1->getSize()*8;
                    let vn0 = data.obank().get(op).expect("pcbx").get_in(0).expect("pcbx: pc in0");
                    let vn1 = data.obank().get(op).expect("pcbx").get_in(1).expect("pcbx: pc in1");
                    let sa = data.vbank().get(vn1).expect("pcbx").get_size() * 8;
                    if bit_pos >= sa {
                        vn = vn0;
                        bit_pos -= sa;
                        mask >>= sa as u32;
                    } else {
                        vn = vn1;
                    }
                }
                OpCode::CPUI_INT_LEFT => {
                    // vn1 = op->getIn(1); if (!const) return 0; sa = vn1->getOffset();
                    // if (sa > bitPos) return 0; bitPos -= sa; mask >>= sa; vn = op->getIn(0);
                    let vn1 = data.obank().get(op).expect("pcbx").get_in(1).expect("pcbx: shl in1");
                    if !data.vbank().get(vn1).expect("pcbx").is_constant() {
                        return None;
                    }
                    let sa = data.vbank().get(vn1).expect("pcbx").get_offset() as int4;
                    if sa > bit_pos {
                        return None;
                    }
                    bit_pos -= sa;
                    mask >>= sa as u32;
                    vn = data.obank().get(op).expect("pcbx").get_in(0).expect("pcbx: shl in0");
                }
                OpCode::CPUI_INT_RIGHT | OpCode::CPUI_INT_SRIGHT => {
                    // vn1 = op->getIn(1); if (!const) return 0; sa = vn1->getOffset();
                    // vn = op->getIn(0); bitPos += sa; if (bitPos >= vn->getSize()*8) return 0; mask <<= sa;
                    let vn1 = data.obank().get(op).expect("pcbx").get_in(1).expect("pcbx: shr in1");
                    if !data.vbank().get(vn1).expect("pcbx").is_constant() {
                        return None;
                    }
                    let sa = data.vbank().get(vn1).expect("pcbx").get_offset() as int4;
                    vn = data.obank().get(op).expect("pcbx").get_in(0).expect("pcbx: shr in0");
                    bit_pos += sa;
                    if bit_pos >= data.vbank().get(vn).expect("pcbx").get_size() * 8 {
                        return None;
                    }
                    mask = mask.wrapping_shl(sa as u32);
                }
                _ => {
                    return None;
                }
            }
        }
    }
}

impl Rule for RulePopcountBoolXor {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_POPCOUNT]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(RULE_GROUP) {
            return None;
        }
        Some(Box::new(RulePopcountBoolXor))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // Varnode *outVn = op->getOut();
        let out_vn = data.obank().get(op).expect("pcbx: stale op").get_out().expect("pcbx: out");
        for base_op in descend_ops(data, out_vn) {
            // if (baseOp->code() != CPUI_INT_AND) continue;
            if data.obank().get(base_op).expect("pcbx").code() != OpCode::CPUI_INT_AND {
                continue;
            }
            // Varnode *tmpVn = baseOp->getIn(1);
            let tmp_vn = data.obank().get(base_op).expect("pcbx").get_in(1).expect("pcbx: and in1");
            let tv = data.vbank().get(tmp_vn).expect("pcbx");
            // if (!tmpVn->isConstant()) continue;
            if !tv.is_constant() {
                continue;
            }
            // if (tmpVn->getOffset() != 1) continue;
            if tv.get_offset() != 1 {
                continue;
            }
            // if (tmpVn->getSize() != 1) continue;
            if tv.get_size() != 1 {
                continue;
            }
            // Varnode *inVn = op->getIn(0); if (!inVn->isWritten()) return 0;
            let in_vn = data.obank().get(op).expect("pcbx").get_in(0).expect("pcbx: op in0");
            if !data.vbank().get(in_vn).expect("pcbx").is_written() {
                return 0;
            }
            // int4 count = popcount(inVn->getNZMask());
            let nz = data.vbank().get(in_vn).expect("pcbx").get_nz_mask();
            let count = popcount(nz);
            if count == 1 {
                // int4 leastPos = leastsigbit_set(inVn->getNZMask());
                let least_pos = leastsigbit_set(nz);
                let mut const_res: int4 = 0;
                // Varnode *b1 = getBooleanResult(inVn, leastPos, constRes);
                let b1 =
                    RulePopcountBoolXor::get_boolean_result(data, in_vn, least_pos, &mut const_res);
                // if (b1 == 0) continue;
                let b1 = match b1 {
                    Some(b) => b,
                    None => continue,
                };
                // data.opSetOpcode(baseOp, CPUI_COPY); data.opRemoveInput(baseOp,1); data.opSetInput(baseOp,b1,0);
                rule_set_opcode(data, base_op, OpCode::CPUI_COPY);
                data.op_remove_input(base_op, 1);
                data.op_set_input(base_op, b1, 0).expect("pcbx: set");
                return 1;
            }
            if count == 2 {
                // int4 pos0 = leastsigbit_set(nz); int4 pos1 = mostsigbit_set(nz);
                let pos0 = leastsigbit_set(nz);
                let pos1 = mostsigbit_set(nz);
                let mut const_res0: int4 = 0;
                let mut const_res1: int4 = 0;
                let b1 =
                    RulePopcountBoolXor::get_boolean_result(data, in_vn, pos0, &mut const_res0);
                // if (b1 == 0 && constRes0 != 1) continue;
                if b1.is_none() && const_res0 != 1 {
                    continue;
                }
                let b2 =
                    RulePopcountBoolXor::get_boolean_result(data, in_vn, pos1, &mut const_res1);
                // if (b2 == 0 && constRes1 != 1) continue;
                if b2.is_none() && const_res1 != 1 {
                    continue;
                }
                // if (b1 == 0 && b2 == 0) continue;
                if b1.is_none() && b2.is_none() {
                    continue;
                }
                // if (b1 == 0) b1 = data.newConstant(1,1); if (b2 == 0) b2 = data.newConstant(1,1);
                let b1 = b1.unwrap_or_else(|| data.new_constant(1, 1));
                let b2 = b2.unwrap_or_else(|| data.new_constant(1, 1));
                // data.opSetOpcode(baseOp, CPUI_INT_XOR); data.opSetInput(baseOp,b1,0); data.opSetInput(baseOp,b2,1);
                rule_set_opcode(data, base_op, OpCode::CPUI_INT_XOR);
                data.op_set_input(base_op, b1, 0).expect("pcbx: set");
                data.op_set_input(base_op, b2, 1).expect("pcbx: set");
                return 1;
            }
        }
        0
    }
}

// =============================================================================
// RulePiecePathology (ruleaction.cc:10437)
// =============================================================================

/// \brief Search for concatenations with unlikely things to inform return/
/// parameter consumption calculation (C++ `RulePiecePathology`, name
/// `"piecepathology"`).
pub struct RulePiecePathology;

impl Rule for RulePiecePathology {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_PIECE]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        // C++ getGroup() == "protorecovery" (coreaction.cc), not RULE_GROUP/"analysis".
        if !grouplist.contains("protorecovery") && !grouplist.contains("piecepathology") {
            return None;
        }
        Some(Box::new(RulePiecePathology))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // Varnode *vn = op->getIn(0); if (!vn->isWritten()) return 0;
        let vn = data.obank().get(op).expect("pp: stale op").get_in(0).expect("pp: in0");
        if !data.vbank().get(vn).expect("pp").is_written() {
            return 0;
        }
        // PcodeOp *subOp = vn->getDef(); OpCode opc = subOp->code();
        let sub_op = data.vbank().get(vn).expect("pp").get_def().expect("pp: vn def");
        let opc = data.obank().get(sub_op).expect("pp").code();
        // The remaining checks (isPathology / contiguous-register INDIRECT) and
        // the forward trace all consult FuncCallSpecs / FuncProto consumption
        // bookkeeping (data.getCallSpecs / fProto->isOutputActive /
        // setInputBytesConsumed / setReturnBytesConsumed), which is the W4 call-
        // specification subsystem and is not ported.  Without it the rule cannot
        // confirm a pathology and so no-ops, exactly as the C++ does when no
        // pathology is detected.  SEAM(W4): FuncCallSpecs / FuncProto.
        if opc == OpCode::CPUI_SUBPIECE {
            // if (subOp->getIn(1)->getOffset() == 0) return 0;
            let sub_in1 = data.obank().get(sub_op).expect("pp").get_in(1).expect("pp: sub in1");
            if data.vbank().get(sub_in1).expect("pp").get_offset() == 0 {
                return 0;
            }
            // if (!isPathology(subOp->getIn(0),data)) return 0;  -- SEAM(W4)
            0
        } else if opc == OpCode::CPUI_INDIRECT {
            // if (!subOp->isIndirectCreation()) return 0;  -- SEAM(W4) for the rest
            if !data.obank().get(sub_op).expect("pp").is_indirect_creation() {
                return 0;
            }
            0
        } else {
            0
        }
    }
}

// =============================================================================
// RuleXorSwap (ruleaction.cc:10629)
// =============================================================================

/// \brief Simplify limited chains of XOR operations (C++ `RuleXorSwap`, name
/// `"xorswap"`).
pub struct RuleXorSwap;

impl Rule for RuleXorSwap {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_INT_XOR]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(RULE_GROUP) {
            return None;
        }
        Some(Box::new(RuleXorSwap))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // for(int4 i=0;i<2;++i) {
        for i in 0..2 {
            // Varnode *vn = op->getIn(i); if (!vn->isWritten()) continue;
            let vn = data.obank().get(op).expect("xs: stale op").get_in(i).expect("xs: in");
            if !data.vbank().get(vn).expect("xs").is_written() {
                continue;
            }
            // PcodeOp *op2 = vn->getDef(); if (op2->code() != CPUI_INT_XOR) continue;
            let op2 = data.vbank().get(vn).expect("xs").get_def().expect("xs: vn def");
            if data.obank().get(op2).expect("xs").code() != OpCode::CPUI_INT_XOR {
                continue;
            }
            // Varnode *othervn = op->getIn(1-i);
            let othervn = data.obank().get(op).expect("xs").get_in(1 - i).expect("xs: other");
            // Varnode *vn0 = op2->getIn(0); Varnode *vn1 = op2->getIn(1);
            let vn0 = data.obank().get(op2).expect("xs").get_in(0).expect("xs: op2 in0");
            let vn1 = data.obank().get(op2).expect("xs").get_in(1).expect("xs: op2 in1");
            // if (othervn == vn0 && !vn1->isFree()) { ... }
            if othervn == vn0 && !data.vbank().get(vn1).expect("xs").is_free() {
                data.op_remove_input(op, 1);
                rule_set_opcode(data, op, OpCode::CPUI_COPY);
                data.op_set_input(op, vn1, 0).expect("xs: set");
                return 1;
            } else if othervn == vn1 && !data.vbank().get(vn0).expect("xs").is_free() {
                data.op_remove_input(op, 1);
                rule_set_opcode(data, op, OpCode::CPUI_COPY);
                data.op_set_input(op, vn0, 0).expect("xs: set");
                return 1;
            }
        }
        0
    }
}

// =============================================================================
// RuleLzcountShiftBool (ruleaction.cc:10667)
// =============================================================================

/// \brief Simplify equality checks that use lzcount:
/// `lzcount(X) >> c => X == 0` (C++ `RuleLzcountShiftBool`, name
/// `"lzcountshiftbool"`).
pub struct RuleLzcountShiftBool;

impl Rule for RuleLzcountShiftBool {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_LZCOUNT]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(RULE_GROUP) {
            return None;
        }
        Some(Box::new(RuleLzcountShiftBool))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // Varnode *outVn = op->getOut();
        let out_vn = data.obank().get(op).expect("lsb: stale op").get_out().expect("lsb: out");
        // uintb max_return = 8 * op->getIn(0)->getSize();
        let in0 = data.obank().get(op).expect("lsb").get_in(0).expect("lsb: in0");
        let in0_size = data.vbank().get(in0).expect("lsb").get_size();
        let max_return: uintb = 8u64 * (in0_size as uintb);
        // if (popcount(max_return) != 1) return 0;
        if popcount(max_return) != 1 {
            return 0;
        }
        for base_op in descend_ops(data, out_vn) {
            // if (baseOp->code() != CPUI_INT_RIGHT && baseOp->code() != CPUI_INT_SRIGHT) continue;
            let code = data.obank().get(base_op).expect("lsb").code();
            if code != OpCode::CPUI_INT_RIGHT && code != OpCode::CPUI_INT_SRIGHT {
                continue;
            }
            // Varnode *vn1 = baseOp->getIn(1); if (!vn1->isConstant()) continue;
            let vn1 = data.obank().get(base_op).expect("lsb").get_in(1).expect("lsb: shift in1");
            if !data.vbank().get(vn1).expect("lsb").is_constant() {
                continue;
            }
            // uintb shift = vn1->getOffset();
            let shift = data.vbank().get(vn1).expect("lsb").get_offset();
            // if ((max_return >> shift) == 1) { ... }
            if (max_return >> shift) == 1 {
                let base_addr = data.obank().get(base_op).expect("lsb").get_addr().clone();
                // PcodeOp* newOp = data.newOp(2, baseOp->getAddr());
                let new_op = data.new_op(2, base_addr);
                // data.opSetOpcode(newOp, CPUI_INT_EQUAL);
                rule_set_opcode(data, new_op, OpCode::CPUI_INT_EQUAL);
                // Varnode* b = data.newConstant(op->getIn(0)->getSize(), 0);
                let b = data.new_constant(in0_size, 0);
                // data.opSetInput(newOp, op->getIn(0), 0); data.opSetInput(newOp, b, 1);
                let op_in0 = data.obank().get(op).expect("lsb").get_in(0).expect("lsb: op in0");
                data.op_set_input(new_op, op_in0, 0).expect("lsb: set");
                data.op_set_input(new_op, b, 1).expect("lsb: set");
                // Varnode* eqResVn = data.newUniqueOut(1, newOp);  -- SEAM(W3)
                let eq_res_vn = match new_unique_out(data, 1, new_op) {
                    Ok(v) => v,
                    Err(_) => return 0, // SEAM(W3): newUniqueOut unavailable
                };
                // data.opInsertBefore(newOp, baseOp);
                data.op_insert_before(new_op, base_op);
                // data.opRemoveInput(baseOp, 1);
                data.op_remove_input(base_op, 1);
                // if (baseOp->getOut()->getSize() == 1) opSetOpcode(COPY); else opSetOpcode(INT_ZEXT);
                let base_out =
                    data.obank().get(base_op).expect("lsb").get_out().expect("lsb: base out");
                if data.vbank().get(base_out).expect("lsb").get_size() == 1 {
                    rule_set_opcode(data, base_op, OpCode::CPUI_COPY);
                } else {
                    rule_set_opcode(data, base_op, OpCode::CPUI_INT_ZEXT);
                }
                // data.opSetInput(baseOp, eqResVn, 0);
                data.op_set_input(base_op, eq_res_vn, 0).expect("lsb: set");
                return 1;
            }
        }
        0
    }
}

// =============================================================================
// RuleFloatSign (ruleaction.cc:10729)
// =============================================================================

/// \brief Convert floating-point sign-bit manipulation into FLOAT_ABS or
/// FLOAT_NEG, by participation in float operations (C++ `RuleFloatSign`, name
/// `"floatsign"`).
pub struct RuleFloatSign;

impl Rule for RuleFloatSign {
    fn get_op_list(&self) -> Vec<OpCode> {
        // The exact 18-element list from the C++ getOpList, in order.
        vec![
            OpCode::CPUI_FLOAT_EQUAL,
            OpCode::CPUI_FLOAT_NOTEQUAL,
            OpCode::CPUI_FLOAT_LESS,
            OpCode::CPUI_FLOAT_LESSEQUAL,
            OpCode::CPUI_FLOAT_NAN,
            OpCode::CPUI_FLOAT_ADD,
            OpCode::CPUI_FLOAT_DIV,
            OpCode::CPUI_FLOAT_MULT,
            OpCode::CPUI_FLOAT_SUB,
            OpCode::CPUI_FLOAT_NEG,
            OpCode::CPUI_FLOAT_ABS,
            OpCode::CPUI_FLOAT_SQRT,
            OpCode::CPUI_FLOAT_FLOAT2FLOAT,
            OpCode::CPUI_FLOAT_CEIL,
            OpCode::CPUI_FLOAT_FLOOR,
            OpCode::CPUI_FLOAT_ROUND,
            OpCode::CPUI_FLOAT_INT2FLOAT,
            OpCode::CPUI_FLOAT_TRUNC,
        ]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(RULE_GROUP) {
            return None;
        }
        Some(Box::new(RuleFloatSign))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        let mut res: int4 = 0;
        // OpCode opc = op->code();
        let opc = data.obank().get(op).expect("fs: stale op").code();
        // if (opc != CPUI_FLOAT_INT2FLOAT) { ... }
        if opc != OpCode::CPUI_FLOAT_INT2FLOAT {
            // Varnode *vn = op->getIn(0); if (vn->isWritten()) { ... }
            let vn = data.obank().get(op).expect("fs").get_in(0).expect("fs: in0");
            if data.vbank().get(vn).expect("fs").is_written() {
                // PcodeOp *signOp = vn->getDef();
                let sign_op = data.vbank().get(vn).expect("fs").get_def().expect("fs: vn def");
                // OpCode resCode = TypeOp::floatSignManipulation(signOp);  -- SEAM(W6)
                let res_code = float_sign_manipulation(data, sign_op);
                if res_code != OpCode::CPUI_MAX {
                    data.op_remove_input(sign_op, 1);
                    rule_set_opcode(data, sign_op, res_code);
                    res = 1;
                }
            }
            // if (op->numInput() == 2) { vn = op->getIn(1); if (vn->isWritten()) {...} }
            if data.obank().get(op).expect("fs").num_input() == 2 {
                let vn = data.obank().get(op).expect("fs").get_in(1).expect("fs: in1");
                if data.vbank().get(vn).expect("fs").is_written() {
                    let sign_op = data.vbank().get(vn).expect("fs").get_def().expect("fs: vn def");
                    let res_code = float_sign_manipulation(data, sign_op);
                    if res_code != OpCode::CPUI_MAX {
                        data.op_remove_input(sign_op, 1);
                        rule_set_opcode(data, sign_op, res_code);
                        res = 1;
                    }
                }
            }
        }
        // if (op->isBoolOutput() || opc == CPUI_FLOAT_TRUNC) return res;
        if data.obank().get(op).expect("fs").is_bool_output() || opc == OpCode::CPUI_FLOAT_TRUNC {
            return res;
        }
        // Varnode *outvn = op->getOut();
        let outvn = data.obank().get(op).expect("fs").get_out().expect("fs: out");
        // for(iter over outvn descendants) { readOp ... }
        for read_op in descend_ops(data, outvn) {
            // OpCode resCode = TypeOp::floatSignManipulation(readOp);  -- SEAM(W6)
            let res_code = float_sign_manipulation(data, read_op);
            if res_code != OpCode::CPUI_MAX {
                data.op_remove_input(read_op, 1);
                rule_set_opcode(data, read_op, res_code);
                res = 1;
            }
        }
        res
    }
}

// =============================================================================
// RuleFloatSignCleanup (ruleaction.cc:10793)
// =============================================================================

/// \brief Convert floating-point sign-bit manipulation into FLOAT_ABS or
/// FLOAT_NEG, driven by the Varnode data-type (C++ `RuleFloatSignCleanup`, name
/// `"floatsigncleanup"`).
pub struct RuleFloatSignCleanup;

impl Rule for RuleFloatSignCleanup {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_INT_AND, OpCode::CPUI_INT_XOR]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        // C++ getGroup() == "cleanup" (coreaction.cc), not RULE_GROUP/"analysis".
        if !grouplist.contains("cleanup") && !grouplist.contains("floatsigncleanup") {
            return None;
        }
        Some(Box::new(RuleFloatSignCleanup))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        use crate::dtype::type_metatype;
        // if (op->getOut()->getType()->getMetatype() != TYPE_FLOAT) return 0;
        let outvn = data.obank().get(op).expect("fsc: stale op").get_out().expect("fsc: out");
        if data.vbank().get(outvn).expect("fsc").get_type().get_metatype()
            != type_metatype::TYPE_FLOAT
        {
            return 0;
        }
        // OpCode opc = TypeOp::floatSignManipulation(op); if (opc == CPUI_MAX) return 0;
        //   -- SEAM(W6): floatSignManipulation returns CPUI_MAX, so the rule no-ops.
        let opc = float_sign_manipulation(data, op);
        if opc == OpCode::CPUI_MAX {
            return 0;
        }
        // data.opRemoveInput(op, 1); data.opSetOpcode(op, opc);
        data.op_remove_input(op, 1);
        rule_set_opcode(data, op, opc);
        1
    }
}

// =============================================================================
// RuleOrCompare (ruleaction.cc:10818)
// =============================================================================

/// \brief Simplify INT_OR in comparisons with 0 (C++ `RuleOrCompare`, name
/// `"orcompare"`).
///
/// `(V | W) == 0` => `(V == 0) && (W == 0)`,
/// `(V | W) != 0` => `(V != 0) || (W != 0)`.
pub struct RuleOrCompare;

impl Rule for RuleOrCompare {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_INT_OR]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(RULE_GROUP) {
            return None;
        }
        Some(Box::new(RuleOrCompare))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // Varnode *outvn = op->getOut();
        let outvn = data.obank().get(op).expect("oc: stale op").get_out().expect("oc: out");
        // for(iter over outvn descendants): every reader must be (==0)/(!=0)
        let mut has_compares = false;
        for comp_op in descend_ops(data, outvn) {
            // OpCode opc = compOp->code();
            let opc = data.obank().get(comp_op).expect("oc").code();
            // if (opc != CPUI_INT_EQUAL && opc != CPUI_INT_NOTEQUAL) return 0;
            if opc != OpCode::CPUI_INT_EQUAL && opc != OpCode::CPUI_INT_NOTEQUAL {
                return 0;
            }
            // if (!compOp->getIn(1)->constantMatch(0)) return 0;
            let cin1 = data.obank().get(comp_op).expect("oc").get_in(1).expect("oc: comp in1");
            if !data.vbank().get(cin1).expect("oc").constant_match(0) {
                return 0;
            }
            has_compares = true;
        }
        // if (!hasCompares) return 0;
        if !has_compares {
            return 0;
        }
        // Varnode* V = op->getIn(0); Varnode* W = op->getIn(1);
        let v = data.obank().get(op).expect("oc").get_in(0).expect("oc: in0");
        let w = data.obank().get(op).expect("oc").get_in(1).expect("oc: in1");
        // if (V->isFree()) return 0; if (W->isFree()) return 0;
        if data.vbank().get(v).expect("oc").is_free() {
            return 0;
        }
        if data.vbank().get(w).expect("oc").is_free() {
            return 0;
        }
        let v_size = data.vbank().get(v).expect("oc").get_size();
        let w_size = data.vbank().get(w).expect("oc").get_size();
        // iterate the (snapshotted) descendants, advancing before modifying each.
        for equal_op in descend_ops(data, outvn) {
            // OpCode opc = equalOp->code();
            let opc = data.obank().get(equal_op).expect("oc").code();
            // Varnode* zero_V = data.newConstant(V->getSize(), 0);  zero_W likewise
            let zero_v = data.new_constant(v_size, 0);
            let zero_w = data.new_constant(w_size, 0);
            let addr = data.obank().get(equal_op).expect("oc").get_addr().clone();
            // PcodeOp* eq_V = data.newOp(2, equalOp->getAddr()); opSetOpcode(eq_V, opc);
            let eq_v = data.new_op(2, addr.clone());
            rule_set_opcode(data, eq_v, opc);
            data.op_set_input(eq_v, v, 0).expect("oc: set");
            data.op_set_input(eq_v, zero_v, 1).expect("oc: set");
            // PcodeOp* eq_W = data.newOp(2, equalOp->getAddr()); opSetOpcode(eq_W, opc);
            let eq_w = data.new_op(2, addr);
            rule_set_opcode(data, eq_w, opc);
            data.op_set_input(eq_w, w, 0).expect("oc: set");
            data.op_set_input(eq_w, zero_w, 1).expect("oc: set");
            // Varnode* eq_V_out = data.newUniqueOut(1, eq_V); eq_W_out likewise  -- SEAM(W3)
            let eq_v_out = match new_unique_out(data, 1, eq_v) {
                Ok(o) => o,
                Err(_) => return 0, // SEAM(W3): newUniqueOut unavailable
            };
            let eq_w_out = match new_unique_out(data, 1, eq_w) {
                Ok(o) => o,
                Err(_) => return 0,
            };
            // data.opInsertBefore(eq_V, equalOp); data.opInsertBefore(eq_W, equalOp);
            data.op_insert_before(eq_v, equal_op);
            data.op_insert_before(eq_w, equal_op);
            // change INT_EQUAL -> BOOL_AND, INT_NOTEQUAL -> BOOL_OR
            let newopc = if opc == OpCode::CPUI_INT_EQUAL {
                OpCode::CPUI_BOOL_AND
            } else {
                OpCode::CPUI_BOOL_OR
            };
            rule_set_opcode(data, equal_op, newopc);
            data.op_set_input(equal_op, eq_v_out, 0).expect("oc: set");
            data.op_set_input(equal_op, eq_w_out, 1).expect("oc: set");
        }
        1
    }
}

// =============================================================================
// RuleExpandLoad (ruleaction.cc:10889)
// =============================================================================

/// \brief Convert LOAD size to match pointer data-type (C++ `RuleExpandLoad`,
/// name `"expandload"`).
pub struct RuleExpandLoad;

impl RuleExpandLoad {
    /// \brief Check that all uses of `vn` are of the form `(V & C) == D`
    /// (C++ `RuleExpandLoad::checkAndComparison`).
    pub fn check_and_comparison(data: &Funcdata, vn: VarnodeId) -> bool {
        for op in descend_ops(data, vn) {
            // if (op->code() != CPUI_INT_AND) return false;
            if data.obank().get(op).expect("el").code() != OpCode::CPUI_INT_AND {
                return false;
            }
            // if (!op->getIn(1)->isConstant()) return false;
            let in1 = data.obank().get(op).expect("el").get_in(1).expect("el: and in1");
            if !data.vbank().get(in1).expect("el").is_constant() {
                return false;
            }
            // PcodeOp *compOp = op->getOut()->loneDescend(); if (compOp == 0) return false;
            let out = data.obank().get(op).expect("el").get_out().expect("el: and out");
            let comp_op = match lone_descend(data, out) {
                Some(c) => c,
                None => return false,
            };
            // OpCode opc = compOp->code();
            let opc = data.obank().get(comp_op).expect("el").code();
            if opc != OpCode::CPUI_INT_EQUAL && opc != OpCode::CPUI_INT_NOTEQUAL {
                return false;
            }
            // if (!compOp->getIn(1)->isConstant()) return false;
            let cin1 = data.obank().get(comp_op).expect("el").get_in(1).expect("el: comp in1");
            if !data.vbank().get(cin1).expect("el").is_constant() {
                return false;
            }
        }
        true
    }

    /// \brief Expand the constants in the previously scanned `(V & C) == D` forms
    /// (C++ `RuleExpandLoad::modifyAndComparison`).
    ///
    /// Change the size and data-type of all the AND/EQUAL constants in these
    /// expressions: replace the input `oldVn` with the bigger `newVn` and shift
    /// each constant up by `offset` *bytes*.
    fn modify_and_comparison(
        data: &mut Funcdata,
        old_vn: VarnodeId,
        new_vn: VarnodeId,
        dt: &Rc<Datatype>,
        offset: int4,
    ) {
        let shift = 8 * offset; // Convert to shift amount (bits)
        let dt_size = dt.get_size();
        // Snapshot descendants before mutating (the C++ advances the iterator
        // before modifying each op).
        for and_op in descend_ops(data, old_vn) {
            // PcodeOp *compOp = andOp->getOut()->loneDescend();
            let and_out = data.obank().get(and_op).expect("mac: stale and").get_out().expect("mac: and out");
            let comp_op = lone_descend(data, and_out).expect("mac: checkAndComparison guaranteed lone descend");
            // newOff = andOp->getIn(1)->getOffset(); newOff <<= offset;
            let and_c = data.obank().get(and_op).expect("mac: stale and").get_in(1).expect("mac: and in1");
            let and_off = data.vbank().get(and_c).expect("mac: stale and const").get_offset();
            let new_and_off = and_off << shift;
            // Varnode *vn = data.newConstant(dt->getSize(), newOff); vn->updateType(dt);
            let new_and_const = data.new_constant(dt_size, new_and_off);
            data.vbank_mut().get_mut(new_and_const).expect("mac: stale new and const").update_type(Rc::clone(dt));
            // opSetInput(andOp, newVn, 0); opSetInput(andOp, vn, 1);
            data.op_set_input(and_op, new_vn, 0).expect("mac: opSetInput and 0");
            data.op_set_input(and_op, new_and_const, 1).expect("mac: opSetInput and 1");
            // newOff = compOp->getIn(1)->getOffset(); newOff <<= offset;
            let comp_c = data.obank().get(comp_op).expect("mac: stale comp").get_in(1).expect("mac: comp in1");
            let comp_off = data.vbank().get(comp_c).expect("mac: stale comp const").get_offset();
            let new_comp_off = comp_off << shift;
            let new_comp_const = data.new_constant(dt_size, new_comp_off);
            data.vbank_mut().get_mut(new_comp_const).expect("mac: stale new comp const").update_type(Rc::clone(dt));
            // opSetInput(compOp, vn, 1);
            data.op_set_input(comp_op, new_comp_const, 1).expect("mac: opSetInput comp 1");
        }
    }
}

impl Rule for RuleExpandLoad {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_LOAD]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        // C++ getGroup() == "cleanup" (coreaction.cc), not RULE_GROUP/"analysis".
        if !grouplist.contains("cleanup") && !grouplist.contains("expandload") {
            return None;
        }
        Some(Box::new(RuleExpandLoad))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        use crate::dtype::type_metatype;

        // Varnode *outVn = op->getOut(); int4 outSize = outVn->getSize();
        let out_vn = data.obank().get(op).expect("el: stale op").get_out().expect("el: out");
        let out_size = data.vbank().get(out_vn).expect("el: stale outVn").get_size();
        // Varnode *rootPtr = op->getIn(1);
        let mut root_ptr = data.obank().get(op).expect("el: stale op").get_in(1).expect("el: load ptr");
        let mut add_op: Option<OpId> = None;
        let mut offset: int4 = 0;
        // Datatype *elType;
        let el_type: Rc<Datatype>;
        let root_v = data.vbank().get(root_ptr).expect("el: stale rootPtr");
        if root_v.is_written() {
            let def_op = root_v.get_def().expect("el: written rootPtr has def");
            let def_code = data.obank().get(def_op).expect("el: stale defOp").code();
            let def_in1 = data.obank().get(def_op).expect("el: stale defOp").get_in(1);
            let def_in1_const = def_in1
                .map(|v| data.vbank().get(v).expect("el: stale defOp in1").is_constant())
                .unwrap_or(false);
            if def_code == OpCode::CPUI_INT_ADD && def_in1_const {
                add_op = Some(def_op);
                let new_root = data.obank().get(def_op).expect("el: stale defOp").get_in(0).expect("el: defOp in0");
                // uintb off = defOp->getIn(1)->getOffset();
                let off = data.vbank().get(def_in1.unwrap()).expect("el: stale defOp in1").get_offset();
                if off > 16 {
                    return 0; // INT_ADD offset must be small
                }
                offset = off as int4;
                // if (defOp->getOut()->loneDescend() == 0) return 0;  // used only once
                let def_out = data.obank().get(def_op).expect("el: stale defOp").get_out().expect("el: defOp out");
                if lone_descend(data, def_out).is_none() {
                    return 0;
                }
                root_ptr = new_root;
                // elType = rootPtr->getTypeReadFacing(defOp);
                el_type = Rc::clone(
                    data.vbank().get(root_ptr).expect("el: stale rootPtr").get_type_read_facing(def_op),
                );
            } else {
                // elType = rootPtr->getTypeReadFacing(op);
                el_type = Rc::clone(
                    data.vbank().get(root_ptr).expect("el: stale rootPtr").get_type_read_facing(op),
                );
            }
        } else {
            el_type = Rc::clone(
                data.vbank().get(root_ptr).expect("el: stale rootPtr").get_type_read_facing(op),
            );
        }
        // if (elType->getMetatype() != TYPE_PTR) return 0;
        if el_type.get_metatype() != type_metatype::TYPE_PTR {
            return 0;
        }
        // elType = ((TypePointer *)elType)->getPtrTo();
        let el_type = match el_type.get_ptr_to() {
            Some(p) => p,
            None => return 0,
        };
        // if (elType->getSize() <= outSize) return 0;  // pointer data-type must be bigger
        if el_type.get_size() <= out_size {
            return 0;
        }
        // if (elType->getSize() < outSize + offset) return 0;
        if el_type.get_size() < out_size + offset {
            return 0;
        }

        let meta = el_type.get_metatype();
        if meta == type_metatype::TYPE_UNKNOWN
            || meta == type_metatype::TYPE_STRUCT
            || meta == type_metatype::TYPE_ARRAY
            || meta == type_metatype::TYPE_UNION
            || meta == type_metatype::TYPE_PARTIALSTRUCT
            || meta == type_metatype::TYPE_PARTIALUNION
        {
            return 0;
        }
        // bool addForm = checkAndComparison(outVn);
        let add_form = RuleExpandLoad::check_and_comparison(data, out_vn);
        // AddrSpace *spc = op->getIn(0)->getSpaceFromConst();
        let space_const = data.obank().get(op).expect("el: stale op").get_in(0).expect("el: load space");
        let spc = match space_from_const(data, space_const) {
            Some(s) => s,
            None => return 0,
        };
        let mut lsb_cut: int4 = 0;
        if add_form {
            if spc.is_big_endian() {
                lsb_cut = el_type.get_size() - out_size - offset;
            } else {
                lsb_cut = offset;
            }
        } else {
            // Check for natural integer truncation.
            if meta != type_metatype::TYPE_INT && meta != type_metatype::TYPE_UINT {
                return 0;
            }
            // type_metatype outMeta = outVn->getTypeDefFacing()->getMetatype();
            let out_meta = data
                .vbank()
                .get(out_vn)
                .expect("el: stale outVn")
                .get_type_def_facing()
                .get_metatype();
            if out_meta != type_metatype::TYPE_INT
                && out_meta != type_metatype::TYPE_UINT
                && out_meta != type_metatype::TYPE_UNKNOWN
                && out_meta != type_metatype::TYPE_BOOL
            {
                // C++ `return false;` (treated as no-op).
                return 0;
            }
            // Check that LOAD is grabbing least significant bytes.
            if spc.is_big_endian() {
                if out_size + offset != el_type.get_size() {
                    return 0;
                }
            } else if offset != 0 {
                return 0;
            }
        }

        // Modify the LOAD.
        // Varnode *newOut = data.newUnique(elType->getSize(), elType);
        let new_out = data.new_unique(el_type.get_size(), Some(Rc::clone(&el_type)));
        data.op_set_output(op, new_out).expect("el: opSetOutput newOut");
        if let Some(add) = add_op {
            // data.opSetInput(op, rootPtr, 1); data.opDestroy(addOp);
            data.op_set_input(op, root_ptr, 1).expect("el: opSetInput rootPtr");
            data.op_destroy(add);
        }
        if add_form {
            // if (meta != TYPE_INT && meta != TYPE_UINT)
            //   elType = data.getArch()->types->getBase(elType->getSize(), TYPE_UINT);
            let dt = if meta != type_metatype::TYPE_INT && meta != type_metatype::TYPE_UINT {
                data.get_arch()
                    .types()
                    .expect("el: types factory")
                    .get_base(el_type.get_size(), type_metatype::TYPE_UINT)
                    .expect("el: getBase UINT")
            } else {
                Rc::clone(&el_type)
            };
            RuleExpandLoad::modify_and_comparison(data, out_vn, new_out, &dt, lsb_cut);
        } else {
            // PcodeOp *subOp = data.newOp(2, op->getAddr());
            let opaddr = data.obank().get(op).expect("el: stale op").get_addr().clone();
            let sub_op = data.new_op(2, opaddr);
            rule_set_opcode(data, sub_op, OpCode::CPUI_SUBPIECE);
            // opSetInput(subOp, newOut, 0);
            data.op_set_input(sub_op, new_out, 0).expect("el: opSetInput sub 0");
            // opSetInput(subOp, newConstant(4, 0), 1);
            let zero = data.new_constant(4, 0);
            data.op_set_input(sub_op, zero, 1).expect("el: opSetInput sub 1");
            // opSetOutput(subOp, outVn);
            data.op_set_output(sub_op, out_vn).expect("el: opSetOutput sub");
            // opInsertAfter(subOp, op);
            data.op_insert_after(sub_op, op);
        }
        1
    }
}

// =============================================================================
// Rule registration (W8 universalAction reads these IN C++ DEFINITION ORDER)
// =============================================================================

/// The [`RuleSpec`] rows for this batch, in C++ definition order.  The `group`
/// field is the stage group the rule registers against (the analysis family, as
/// in `coreaction.cc`'s `universalAction`); W8 wraps each in a [`RuleState`]
/// carrying the rule's exact C++ `name()` string.
pub fn specs() -> Vec<RuleSpec> {
    vec![
        RuleSpec { group: RULE_GROUP, ctor: || Box::new(RuleUnsigned2Float) },
        RuleSpec { group: RULE_GROUP, ctor: || Box::new(RuleInt2FloatCollapse) },
        RuleSpec { group: RULE_GROUP, ctor: || Box::new(RuleFuncPtrEncoding) },
        RuleSpec { group: RULE_GROUP, ctor: || Box::new(RuleThreeWayCompare) },
        RuleSpec { group: RULE_GROUP, ctor: || Box::new(RulePopcountBoolXor) },
        // Registered "protorecovery" (coreaction.cc / universalaction.rs), not RULE_GROUP.
        RuleSpec { group: "protorecovery", ctor: || Box::new(RulePiecePathology) },
        RuleSpec { group: RULE_GROUP, ctor: || Box::new(RuleXorSwap) },
        RuleSpec { group: RULE_GROUP, ctor: || Box::new(RuleLzcountShiftBool) },
        RuleSpec { group: RULE_GROUP, ctor: || Box::new(RuleFloatSign) },
        // Registered "cleanup" (coreaction.cc / universalaction.rs), not RULE_GROUP.
        RuleSpec { group: "cleanup", ctor: || Box::new(RuleFloatSignCleanup) },
        RuleSpec { group: RULE_GROUP, ctor: || Box::new(RuleOrCompare) },
        // Registered "cleanup" (coreaction.cc / universalaction.rs), not RULE_GROUP.
        RuleSpec { group: "cleanup", ctor: || Box::new(RuleExpandLoad) },
    ]
}

#[cfg(test)]
mod tests {
    //! Hand-built input-op-pattern tests for the intricate rules in this batch,
    //! each asserting the exact transformation per the C++ body, plus negative
    //! tests for the early-out guards.  Coverage spans well over a third of the
    //! rules (the intricate ones): `RuleXorSwap`, `RuleFuncPtrEncoding`,
    //! `RulePopcountBoolXor` (+ `get_boolean_result`), `RuleThreeWayCompare`
    //! (+ `test_compare_equivalence`/`detect_three_way`), `RuleOrCompare`,
    //! `RuleFloatSignCleanup`, and the SEAM helpers (`resolve_typeop`,
    //! `new_unique_out`, `lone_descend`).

    use std::rc::Rc;

    use kuna_base::address::Address;
    use kuna_base::error::KunaResult;
    use kuna_base::space::{
        addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, FspecSpace,
        IopSpace, UniqueSpace,
    };
    use kuna_base::types::int4;
    use kuna_num::opcodes::OpCode;

    use crate::action::ActionGroupList;
    use crate::dtype::{type_metatype, Datatype};
    use crate::funcdata::Funcdata;
    use crate::seams::{Architecture, OpId, VarnodeId};
    use crate::varnode::{DefOpInfo, VarnodeBank};

    use super::*;

    // --- fixtures ---------------------------------------------------------

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

    fn float_t(size: int4) -> Rc<Datatype> {
        Rc::new(Datatype::new(size, type_metatype::TYPE_FLOAT))
    }

    fn no_replace() -> impl FnMut(&mut VarnodeBank, VarnodeId, VarnodeId) -> KunaResult<()> {
        |_: &mut VarnodeBank, _: VarnodeId, _: VarnodeId| -> KunaResult<()> { Ok(()) }
    }

    /// A fresh basic block under the bblocks root.
    fn mk_block(fd: &mut Funcdata) -> crate::seams::BlockId {
        let root = fd.bblocks_ref().root.expect("bblocks root");
        fd.bblocks_mut().new_block_basic(root)
    }

    /// Create an op with `inputs` slots and a given opcode at `off`, INSERTED into
    /// block `bl`, returning its id.  Uses the W6 seam resolver so the op's
    /// cached flags match the rules' expectations.
    fn mk_op(
        fd: &mut Funcdata,
        bl: crate::seams::BlockId,
        inputs: int4,
        off: u64,
        opc: OpCode,
    ) -> OpId {
        let r = ram(fd);
        let op = fd.new_op(inputs, Address::new(r, off));
        fd.op_set_opcode(op, resolve_typeop_for_test(opc));
        fd.op_insert(op, bl, None);
        op
    }

    /// Build a TypeOp for any opcode (uses the rule resolver where defined, else a
    /// flagless skeleton — fine for opcodes that only appear as *fixture* defs and
    /// are never re-written by a rule).
    fn resolve_typeop_for_test(opc: OpCode) -> crate::seams::TypeOp {
        // The batch resolver panics on opcodes it never writes; for those (e.g.
        // INT_OR, INT_RIGHT, INT_ADD, LOAD) we need a skeleton.  Match the small
        // set the rule resolver knows, else flagless.
        use OpCode::*;
        match opc {
            CPUI_COPY | CPUI_INT_ZEXT | CPUI_INT_XOR | CPUI_INT_EQUAL | CPUI_INT_NOTEQUAL
            | CPUI_INT_LESS | CPUI_INT_LESSEQUAL | CPUI_INT_SLESS | CPUI_INT_SLESSEQUAL
            | CPUI_BOOL_AND | CPUI_BOOL_OR | CPUI_SUBPIECE | CPUI_FLOAT_EQUAL
            | CPUI_FLOAT_NOTEQUAL | CPUI_FLOAT_LESS | CPUI_FLOAT_LESSEQUAL | CPUI_FLOAT_ADD
            | CPUI_FLOAT_NEG | CPUI_FLOAT_ABS | CPUI_FLOAT_INT2FLOAT => resolve_typeop(opc),
            other => crate::seams::TypeOp::new(other, 0, format!("{other:?}")),
        }
    }

    /// A free varnode at (off,size) in ram.
    fn mk_vn(fd: &mut Funcdata, off: u64, size: int4) -> VarnodeId {
        let r = ram(fd);
        fd.new_varnode(size, &Address::new(r, off), Some(unk(size)))
    }

    /// A constant varnode of the given size/value.
    fn mk_const(fd: &mut Funcdata, size: int4, val: u64) -> VarnodeId {
        fd.new_constant(size, val)
    }

    /// Make `op` define a fresh varnode at (off,size) as its output, returning the
    /// output id.  Bypasses the `op_set_output` W3 seam via the low-level
    /// `set_def`/`set_output` pair (the W3 verifier fixture pattern), with the
    /// given data-type.
    fn set_output(fd: &mut Funcdata, op: OpId, off: u64, ct: Rc<Datatype>) -> VarnodeId {
        let r = ram(fd);
        let size = ct.get_size();
        let vn = fd.new_varnode(size, &Address::new(r, off), Some(ct));
        let seq = fd.obank().get(op).unwrap().get_seq_num().clone();
        let info = DefOpInfo { id: op, seqnum: seq };
        let vn = fd.vbank_mut().set_def(vn, info, &mut no_replace()).unwrap();
        fd.obank_mut().get_mut(op).unwrap().set_output(Some(vn));
        vn
    }

    /// Wire `vn` as input `slot` of `op` (descend link + op input).
    fn wire(fd: &mut Funcdata, vn: VarnodeId, op: OpId, slot: int4) {
        fd.vbank_mut().add_descend(vn, op).unwrap();
        fd.obank_mut().get_mut(op).unwrap().set_input(Some(vn), slot);
    }

    fn groups() -> ActionGroupList {
        ActionGroupList::from_names(["analysis"])
    }

    fn code(fd: &Funcdata, op: OpId) -> OpCode {
        fd.obank().get(op).unwrap().code()
    }

    // =====================================================================
    // resolve_typeop / registration plumbing
    // =====================================================================

    #[test]
    fn resolve_typeop_carries_typeop_cc_flags() {
        // INT_EQUAL: binary | booloutput | commutative (typeop.cc:926).
        let t = resolve_typeop(OpCode::CPUI_INT_EQUAL);
        assert_eq!(
            t.get_flags(),
            pcodeop_flags::binary | pcodeop_flags::booloutput | pcodeop_flags::commutative
        );
        // COPY: unary | nocollapse (typeop.cc:391).
        let c = resolve_typeop(OpCode::CPUI_COPY);
        assert_eq!(c.get_flags(), pcodeop_flags::unary | pcodeop_flags::nocollapse);
        // INT_XOR: binary | commutative (typeop.cc:1410) — NOT booloutput.
        let x = resolve_typeop(OpCode::CPUI_INT_XOR);
        assert_eq!(x.get_flags(), pcodeop_flags::binary | pcodeop_flags::commutative);
    }

    #[test]
    fn specs_lists_twelve_rules_in_order() {
        let s = specs();
        assert_eq!(s.len(), 12);
        // Most specs register against the analysis group; three register against
        // their own pool (RulePiecePathology -> "protorecovery", RuleFloatSignCleanup
        // and RuleExpandLoad -> "cleanup"), matching coreaction.cc getGroup(). Each
        // spec's ctor yields a rule that clones in *its own* group's grouplist and is
        // filtered out of an empty grouplist.
        for spec in &s {
            let r = (spec.ctor)();
            let yes = ActionGroupList::from_names([spec.group]);
            assert!(
                r.clone_rule(&yes).is_some(),
                "{} should clone in its registration group",
                spec.group
            );
            assert!(r.clone_rule(&ActionGroupList::new()).is_none());
        }
    }

    #[test]
    fn get_op_lists_match_cpp() {
        assert_eq!(RuleUnsigned2Float.get_op_list(), vec![OpCode::CPUI_FLOAT_INT2FLOAT]);
        assert_eq!(RuleFuncPtrEncoding.get_op_list(), vec![OpCode::CPUI_CALLIND]);
        assert_eq!(RuleXorSwap.get_op_list(), vec![OpCode::CPUI_INT_XOR]);
        assert_eq!(RulePopcountBoolXor.get_op_list(), vec![OpCode::CPUI_POPCOUNT]);
        assert_eq!(RulePiecePathology.get_op_list(), vec![OpCode::CPUI_PIECE]);
        assert_eq!(RuleLzcountShiftBool.get_op_list(), vec![OpCode::CPUI_LZCOUNT]);
        assert_eq!(RuleOrCompare.get_op_list(), vec![OpCode::CPUI_INT_OR]);
        assert_eq!(RuleExpandLoad.get_op_list(), vec![OpCode::CPUI_LOAD]);
        assert_eq!(
            RuleThreeWayCompare.get_op_list(),
            vec![
                OpCode::CPUI_INT_SLESS,
                OpCode::CPUI_INT_SLESSEQUAL,
                OpCode::CPUI_INT_EQUAL,
                OpCode::CPUI_INT_NOTEQUAL,
            ]
        );
        assert_eq!(
            RuleFloatSignCleanup.get_op_list(),
            vec![OpCode::CPUI_INT_AND, OpCode::CPUI_INT_XOR]
        );
        // RuleFloatSign's exact 18-element ordered list.
        assert_eq!(RuleFloatSign.get_op_list().len(), 18);
        assert_eq!(RuleFloatSign.get_op_list()[0], OpCode::CPUI_FLOAT_EQUAL);
        assert_eq!(RuleFloatSign.get_op_list()[17], OpCode::CPUI_FLOAT_TRUNC);
    }

    // =====================================================================
    // RuleXorSwap  —  V = (a ^ b) ^ a  =>  V = b   (full transform)
    // =====================================================================

    #[test]
    fn xorswap_collapses_left_chain() {
        // op = (xorop_result) ^ a, where xorop = a ^ b.  Slot 0 of op is the inner
        // xor's output; othervn (slot 1) == a == op2->getIn(0); vn1 = b is non-free.
        // => COPY(b).
        let mut fd = build_fd();
        let bl = mk_block(&mut fd);
        // a and b are inputs (non-free).
        let a = {
            let v = mk_vn(&mut fd, 0x10, 4);
            fd.vbank_mut().set_input(v, &mut no_replace()).unwrap()
        };
        let b = {
            let v = mk_vn(&mut fd, 0x20, 4);
            fd.vbank_mut().set_input(v, &mut no_replace()).unwrap()
        };
        // inner xor:  t = a ^ b
        let op2 = mk_op(&mut fd, bl, 2, 0x100, OpCode::CPUI_INT_XOR);
        wire(&mut fd, a, op2, 0);
        wire(&mut fd, b, op2, 1);
        let t = set_output(&mut fd, op2, 0x30, unk(4));
        // outer xor:  op = t ^ a
        let op = mk_op(&mut fd, bl, 2, 0x104, OpCode::CPUI_INT_XOR);
        wire(&mut fd, t, op, 0);
        wire(&mut fd, a, op, 1);

        let r = RuleXorSwap.apply_op(op, &mut fd);
        assert_eq!(r, 1, "RuleXorSwap must apply");
        // op is now COPY(b)
        assert_eq!(code(&fd, op), OpCode::CPUI_COPY);
        assert_eq!(fd.obank().get(op).unwrap().num_input(), 1);
        assert_eq!(fd.obank().get(op).unwrap().get_in(0), Some(b));
    }

    #[test]
    fn xorswap_collapses_right_chain() {
        // op = a ^ (a ^ b): othervn(slot0)=a == op2->getIn(0); but we test the
        // other branch: othervn == vn1.  Build op = a ^ (b ^ a) so that the inner
        // xor is b^a, vn1 == a == othervn, vn0 == b non-free => COPY(b).
        let mut fd = build_fd();
        let bl = mk_block(&mut fd);
        let a = {
            let v = mk_vn(&mut fd, 0x10, 4);
            fd.vbank_mut().set_input(v, &mut no_replace()).unwrap()
        };
        let b = {
            let v = mk_vn(&mut fd, 0x20, 4);
            fd.vbank_mut().set_input(v, &mut no_replace()).unwrap()
        };
        // inner xor: t = b ^ a
        let op2 = mk_op(&mut fd, bl, 2, 0x100, OpCode::CPUI_INT_XOR);
        wire(&mut fd, b, op2, 0);
        wire(&mut fd, a, op2, 1);
        let t = set_output(&mut fd, op2, 0x30, unk(4));
        // outer xor: op = a ^ t   (i=1 finds the inner xor; othervn = a = vn1)
        let op = mk_op(&mut fd, bl, 2, 0x104, OpCode::CPUI_INT_XOR);
        wire(&mut fd, a, op, 0);
        wire(&mut fd, t, op, 1);

        let r = RuleXorSwap.apply_op(op, &mut fd);
        assert_eq!(r, 1);
        assert_eq!(code(&fd, op), OpCode::CPUI_COPY);
        assert_eq!(fd.obank().get(op).unwrap().num_input(), 1);
        assert_eq!(fd.obank().get(op).unwrap().get_in(0), Some(b));
    }

    #[test]
    fn xorswap_negative_when_no_inner_xor() {
        // op = a ^ b with both inputs plain inputs (not written by an XOR).
        let mut fd = build_fd();
        let bl = mk_block(&mut fd);
        let a = {
            let v = mk_vn(&mut fd, 0x10, 4);
            fd.vbank_mut().set_input(v, &mut no_replace()).unwrap()
        };
        let b = {
            let v = mk_vn(&mut fd, 0x20, 4);
            fd.vbank_mut().set_input(v, &mut no_replace()).unwrap()
        };
        let op = mk_op(&mut fd, bl, 2, 0x100, OpCode::CPUI_INT_XOR);
        wire(&mut fd, a, op, 0);
        wire(&mut fd, b, op, 1);
        assert_eq!(RuleXorSwap.apply_op(op, &mut fd), 0);
        assert_eq!(code(&fd, op), OpCode::CPUI_INT_XOR); // unchanged
    }

    #[test]
    fn xorswap_negative_when_remaining_input_free() {
        // inner xor t = a ^ b where b is FREE; op = t ^ a.  The remaining input
        // vn1 == b is free, so the guard `!vn1->isFree()` blocks the rewrite.
        let mut fd = build_fd();
        let bl = mk_block(&mut fd);
        let a = {
            let v = mk_vn(&mut fd, 0x10, 4);
            fd.vbank_mut().set_input(v, &mut no_replace()).unwrap()
        };
        let b = mk_vn(&mut fd, 0x20, 4); // FREE (neither input nor written)
        let op2 = mk_op(&mut fd, bl, 2, 0x100, OpCode::CPUI_INT_XOR);
        wire(&mut fd, a, op2, 0);
        wire(&mut fd, b, op2, 1);
        let t = set_output(&mut fd, op2, 0x30, unk(4));
        let op = mk_op(&mut fd, bl, 2, 0x104, OpCode::CPUI_INT_XOR);
        wire(&mut fd, t, op, 0);
        wire(&mut fd, a, op, 1);
        assert_eq!(RuleXorSwap.apply_op(op, &mut fd), 0);
        assert_eq!(code(&fd, op), OpCode::CPUI_INT_XOR);
    }

    // =====================================================================
    // RuleFuncPtrEncoding  —  guards + the W4 funcptr_align SEAM no-op
    // =====================================================================

    #[test]
    fn funcptrencoding_noops_under_w4_seam() {
        // Even with a perfect AND-mask pattern feeding a CALLIND, the rule no-ops
        // because funcptr_align() is the W4 seam (returns 0).
        let mut fd = build_fd();
        let bl = mk_block(&mut fd);
        let ptr = {
            let v = mk_vn(&mut fd, 0x10, 8);
            fd.vbank_mut().set_input(v, &mut no_replace()).unwrap()
        };
        let mask = mk_const(&mut fd, 8, !1u64); // ~1
        let andop = mk_op(&mut fd, bl, 2, 0x100, OpCode::CPUI_INT_AND);
        wire(&mut fd, ptr, andop, 0);
        wire(&mut fd, mask, andop, 1);
        let masked = set_output(&mut fd, andop, 0x20, unk(8));
        let callind = mk_op(&mut fd, bl, 1, 0x104, OpCode::CPUI_CALLIND);
        wire(&mut fd, masked, callind, 0);
        // align == 0 (W4 seam) => no-op.
        assert_eq!(RuleFuncPtrEncoding.apply_op(callind, &mut fd), 0);
        assert_eq!(code(&fd, andop), OpCode::CPUI_INT_AND); // mask not stripped
    }

    #[test]
    fn funcptrencoding_negative_when_in0_not_written() {
        // CALLIND target is a plain input (not written) — early guard.
        let mut fd = build_fd();
        let bl = mk_block(&mut fd);
        let ptr = {
            let v = mk_vn(&mut fd, 0x10, 8);
            fd.vbank_mut().set_input(v, &mut no_replace()).unwrap()
        };
        let callind = mk_op(&mut fd, bl, 1, 0x100, OpCode::CPUI_CALLIND);
        wire(&mut fd, ptr, callind, 0);
        assert_eq!(RuleFuncPtrEncoding.apply_op(callind, &mut fd), 0);
    }

    // =====================================================================
    // RulePopcountBoolXor  —  get_boolean_result + the count==1 COPY path
    // =====================================================================

    #[test]
    fn popcountboolxor_negative_when_input_nzmask_wide() {
        // popcount(inVn)&1 where inVn has the default wide nzmask (popcount > 2):
        // the count==1/count==2 branches are not taken -> no-op.  (A freshly
        // created written varnode has nzm == u64::MAX; there is no public nzmask
        // setter to model the boolean case, so the positive collapse is exercised
        // through get_boolean_result's constant path below.)
        let mut fd = build_fd();
        let bl = mk_block(&mut fd);
        let x = {
            let v = mk_vn(&mut fd, 0x10, 4);
            fd.vbank_mut().set_input(v, &mut no_replace()).unwrap()
        };
        let pop = mk_op(&mut fd, bl, 1, 0x104, OpCode::CPUI_POPCOUNT);
        wire(&mut fd, x, pop, 0);
        let popout = set_output(&mut fd, pop, 0x30, unk(4));
        let one = mk_const(&mut fd, 1, 1);
        let baseop = mk_op(&mut fd, bl, 2, 0x108, OpCode::CPUI_INT_AND);
        wire(&mut fd, popout, baseop, 0);
        wire(&mut fd, one, baseop, 1);
        let _ = set_output(&mut fd, baseop, 0x40, unk(1));
        assert_eq!(RulePopcountBoolXor.apply_op(pop, &mut fd), 0);
        assert_eq!(code(&fd, baseop), OpCode::CPUI_INT_AND);
    }

    #[test]
    fn popcountboolxor_negative_when_mask_not_one() {
        // baseOp = popout & 3  -> tmpVn->getOffset()!=1 guard fails -> no-op.
        let mut fd = build_fd();
        let bl = mk_block(&mut fd);
        let x = {
            let v = mk_vn(&mut fd, 0x10, 4);
            fd.vbank_mut().set_input(v, &mut no_replace()).unwrap()
        };
        let pop = mk_op(&mut fd, bl, 1, 0x104, OpCode::CPUI_POPCOUNT);
        wire(&mut fd, x, pop, 0);
        let popout = set_output(&mut fd, pop, 0x30, unk(4));
        let three = mk_const(&mut fd, 1, 3);
        let baseop = mk_op(&mut fd, bl, 2, 0x108, OpCode::CPUI_INT_AND);
        wire(&mut fd, popout, baseop, 0);
        wire(&mut fd, three, baseop, 1);
        let _ = set_output(&mut fd, baseop, 0x40, unk(1));
        assert_eq!(RulePopcountBoolXor.apply_op(pop, &mut fd), 0);
        assert_eq!(code(&fd, baseop), OpCode::CPUI_INT_AND);
    }

    #[test]
    fn get_boolean_result_constant_passes_back_bit() {
        // A constant varnode returns None with const_res = (offset >> bitPos) & 1.
        let mut fd = build_fd();
        let c = mk_const(&mut fd, 4, 0b100); // bit 2 set
        let mut const_res = 5;
        let got = RulePopcountBoolXor::get_boolean_result(&fd, c, 2, &mut const_res);
        assert_eq!(got, None);
        assert_eq!(const_res, 1);
        let mut const_res2 = 5;
        let got2 = RulePopcountBoolXor::get_boolean_result(&fd, c, 0, &mut const_res2);
        assert_eq!(got2, None);
        assert_eq!(const_res2, 0);
    }

    // =====================================================================
    // RuleThreeWayCompare::test_compare_equivalence  (the static helper)
    // =====================================================================

    #[test]
    fn test_compare_equivalence_matching_unsigned() {
        // lessop = a < b (INT_LESS), lessequalop = a <= b (INT_LESSEQUAL), same
        // operands.  twoLessThan=false; a1==a2, b1==b2 => res 0.
        let mut fd = build_fd();
        let bl = mk_block(&mut fd);
        let a = {
            let v = mk_vn(&mut fd, 0x10, 4);
            fd.vbank_mut().set_input(v, &mut no_replace()).unwrap()
        };
        let b = {
            let v = mk_vn(&mut fd, 0x20, 4);
            fd.vbank_mut().set_input(v, &mut no_replace()).unwrap()
        };
        let less = mk_op(&mut fd, bl, 2, 0x100, OpCode::CPUI_INT_LESS);
        wire(&mut fd, a, less, 0);
        wire(&mut fd, b, less, 1);
        let lesseq = mk_op(&mut fd, bl, 2, 0x104, OpCode::CPUI_INT_LESSEQUAL);
        wire(&mut fd, a, lesseq, 0);
        wire(&mut fd, b, lesseq, 1);
        assert_eq!(RuleThreeWayCompare::test_compare_equivalence(&fd, less, lesseq), 0);
    }

    #[test]
    fn test_compare_equivalence_mismatched_opcode_minus_one() {
        // lessop = INT_LESS but lessequalop = INT_SLESSEQUAL (signed) -> -1.
        let mut fd = build_fd();
        let bl = mk_block(&mut fd);
        let a = {
            let v = mk_vn(&mut fd, 0x10, 4);
            fd.vbank_mut().set_input(v, &mut no_replace()).unwrap()
        };
        let b = {
            let v = mk_vn(&mut fd, 0x20, 4);
            fd.vbank_mut().set_input(v, &mut no_replace()).unwrap()
        };
        let less = mk_op(&mut fd, bl, 2, 0x100, OpCode::CPUI_INT_LESS);
        wire(&mut fd, a, less, 0);
        wire(&mut fd, b, less, 1);
        let lesseq = mk_op(&mut fd, bl, 2, 0x104, OpCode::CPUI_INT_SLESSEQUAL);
        wire(&mut fd, a, lesseq, 0);
        wire(&mut fd, b, lesseq, 1);
        assert_eq!(RuleThreeWayCompare::test_compare_equivalence(&fd, less, lesseq), -1);
    }

    #[test]
    fn test_compare_equivalence_two_less_constants_swap() {
        // lessop = a < b1, lessequalop = a < b2 (both INT_LESS => twoLessThan).
        // b1,b2 constants with b2 == b1+1 => first b-branch: b1+1==b2 => false, no
        // swap (res stays 0).  Use a1==a2 (same var) so only b differs.
        let mut fd = build_fd();
        let bl = mk_block(&mut fd);
        let a = {
            let v = mk_vn(&mut fd, 0x10, 4);
            fd.vbank_mut().set_input(v, &mut no_replace()).unwrap()
        };
        let b1 = mk_const(&mut fd, 4, 5);
        let b2 = mk_const(&mut fd, 4, 6);
        let less = mk_op(&mut fd, bl, 2, 0x100, OpCode::CPUI_INT_LESS);
        wire(&mut fd, a, less, 0);
        wire(&mut fd, b1, less, 1);
        let lesseq = mk_op(&mut fd, bl, 2, 0x104, OpCode::CPUI_INT_LESS);
        wire(&mut fd, a, lesseq, 0);
        wire(&mut fd, b2, lesseq, 1);
        // a1==a2 path skipped; b1!=b2 & twoLessThan; b1+1==b2 => twoLessThan=false,
        // res unchanged (0).
        assert_eq!(RuleThreeWayCompare::test_compare_equivalence(&fd, less, lesseq), 0);
    }

    // =====================================================================
    // RuleOrCompare  —  descend guard passes, then bails on the W3 newUniqueOut
    //                   seam (returns 0 with the original op intact).
    // =====================================================================

    #[test]
    fn orcompare_rewrites_or_equal_zero_to_bool_and() {
        // (V|W) feeding a single (==0): since w4x-flow-linkage filled
        // opSetOutput/newUniqueOut, the full C++ rewrite now runs
        // (ruleaction.cc:10829): INT_EQUAL(V|W, 0) becomes
        // BOOL_AND(INT_EQUAL(V,0), INT_EQUAL(W,0)).
        let mut fd = build_fd();
        let bl = mk_block(&mut fd);
        let v = {
            let x = mk_vn(&mut fd, 0x10, 4);
            fd.vbank_mut().set_input(x, &mut no_replace()).unwrap()
        };
        let w = {
            let x = mk_vn(&mut fd, 0x20, 4);
            fd.vbank_mut().set_input(x, &mut no_replace()).unwrap()
        };
        let orop = mk_op(&mut fd, bl, 2, 0x100, OpCode::CPUI_INT_OR);
        wire(&mut fd, v, orop, 0);
        wire(&mut fd, w, orop, 1);
        let orout = set_output(&mut fd, orop, 0x30, unk(4));
        let zero = mk_const(&mut fd, 4, 0);
        let eq = mk_op(&mut fd, bl, 2, 0x104, OpCode::CPUI_INT_EQUAL);
        wire(&mut fd, orout, eq, 0);
        wire(&mut fd, zero, eq, 1);
        let _ = set_output(&mut fd, eq, 0x40, unk(1));
        assert_eq!(RuleOrCompare.apply_op(orop, &mut fd), 1);
        // The reader became BOOL_AND(eq_V_out, eq_W_out).
        let eq_rec = fd.obank().get(eq).expect("eq survives");
        assert_eq!(eq_rec.code(), OpCode::CPUI_BOOL_AND);
        for (slot, base_in) in [(0i32, v), (1i32, w)] {
            let in_vn = eq_rec.get_in(slot).expect("bool_and input");
            let in_rec = fd.vbank().get(in_vn).expect("input varnode");
            assert_eq!(in_rec.get_size(), 1, "unique bool output");
            assert_eq!(in_rec.get_space().get_name(), "unique");
            let def = in_rec.get_def().expect("defined by new INT_EQUAL");
            let def_rec = fd.obank().get(def).expect("def op");
            assert_eq!(def_rec.code(), OpCode::CPUI_INT_EQUAL);
            assert_eq!(def_rec.get_in(0), Some(base_in), "compares V/W respectively");
            let zin = def_rec.get_in(1).expect("zero constant");
            let zrec = fd.vbank().get(zin).expect("const varnode");
            assert!(zrec.is_constant() && zrec.get_offset() == 0 && zrec.get_size() == 4);
        }
    }

    #[test]
    fn orcompare_negative_when_reader_not_compare() {
        // A reader that is not (==0)/(!=0) blocks the rule immediately.
        let mut fd = build_fd();
        let bl = mk_block(&mut fd);
        let v = {
            let x = mk_vn(&mut fd, 0x10, 4);
            fd.vbank_mut().set_input(x, &mut no_replace()).unwrap()
        };
        let w = {
            let x = mk_vn(&mut fd, 0x20, 4);
            fd.vbank_mut().set_input(x, &mut no_replace()).unwrap()
        };
        let orop = mk_op(&mut fd, bl, 2, 0x100, OpCode::CPUI_INT_OR);
        wire(&mut fd, v, orop, 0);
        wire(&mut fd, w, orop, 1);
        let orout = set_output(&mut fd, orop, 0x30, unk(4));
        // reader is an INT_ADD, not a comparison.
        let add = mk_op(&mut fd, bl, 2, 0x104, OpCode::CPUI_INT_ADD);
        wire(&mut fd, orout, add, 0);
        wire(&mut fd, v, add, 1);
        assert_eq!(RuleOrCompare.apply_op(orop, &mut fd), 0);
        assert_eq!(code(&fd, orop), OpCode::CPUI_INT_OR); // unchanged
    }

    #[test]
    fn orcompare_negative_when_no_descenders() {
        // hasCompares stays false when outvn has no descendants -> return 0.
        let mut fd = build_fd();
        let bl = mk_block(&mut fd);
        let v = {
            let x = mk_vn(&mut fd, 0x10, 4);
            fd.vbank_mut().set_input(x, &mut no_replace()).unwrap()
        };
        let w = {
            let x = mk_vn(&mut fd, 0x20, 4);
            fd.vbank_mut().set_input(x, &mut no_replace()).unwrap()
        };
        let orop = mk_op(&mut fd, bl, 2, 0x100, OpCode::CPUI_INT_OR);
        wire(&mut fd, v, orop, 0);
        wire(&mut fd, w, orop, 1);
        let _ = set_output(&mut fd, orop, 0x30, unk(4));
        assert_eq!(RuleOrCompare.apply_op(orop, &mut fd), 0);
    }

    // =====================================================================
    // RuleFloatSignCleanup  —  type guard + the W6 floatSignManipulation seam
    // =====================================================================

    #[test]
    fn floatsigncleanup_negative_when_output_not_float() {
        // INT_AND with a non-float output: the TYPE_FLOAT guard fails first.
        let mut fd = build_fd();
        let bl = mk_block(&mut fd);
        let x = {
            let v = mk_vn(&mut fd, 0x10, 4);
            fd.vbank_mut().set_input(v, &mut no_replace()).unwrap()
        };
        let mask = mk_const(&mut fd, 4, 0x7fffffff);
        let andop = mk_op(&mut fd, bl, 2, 0x100, OpCode::CPUI_INT_AND);
        wire(&mut fd, x, andop, 0);
        wire(&mut fd, mask, andop, 1);
        let _ = set_output(&mut fd, andop, 0x20, unk(4)); // UNKNOWN, not FLOAT
        assert_eq!(RuleFloatSignCleanup.apply_op(andop, &mut fd), 0);
    }

    #[test]
    fn floatsigncleanup_noops_at_w6_seam_with_float_output() {
        // Float output passes the first guard, but floatSignManipulation is the W6
        // seam (returns CPUI_MAX) so the rule no-ops without altering the op.
        let mut fd = build_fd();
        let bl = mk_block(&mut fd);
        let x = {
            let v = mk_vn(&mut fd, 0x10, 4);
            fd.vbank_mut().set_input(v, &mut no_replace()).unwrap()
        };
        let mask = mk_const(&mut fd, 4, 0x7fffffff);
        let andop = mk_op(&mut fd, bl, 2, 0x100, OpCode::CPUI_INT_AND);
        wire(&mut fd, x, andop, 0);
        wire(&mut fd, mask, andop, 1);
        let _ = set_output(&mut fd, andop, 0x20, float_t(4)); // FLOAT output
        assert_eq!(RuleFloatSignCleanup.apply_op(andop, &mut fd), 0);
        assert_eq!(code(&fd, andop), OpCode::CPUI_INT_AND); // unchanged (W6 seam)
    }

    // =====================================================================
    // lone_descend helper
    // =====================================================================

    #[test]
    fn lone_descend_returns_single_reader_else_none() {
        let mut fd = build_fd();
        let bl = mk_block(&mut fd);
        let defop = mk_op(&mut fd, bl, 1, 0x100, OpCode::CPUI_COPY);
        let src = {
            let v = mk_vn(&mut fd, 0x10, 4);
            fd.vbank_mut().set_input(v, &mut no_replace()).unwrap()
        };
        wire(&mut fd, src, defop, 0);
        let out = set_output(&mut fd, defop, 0x20, unk(4));
        // zero readers -> None
        assert_eq!(lone_descend(&fd, out), None);
        // one reader -> Some
        let r1 = mk_op(&mut fd, bl, 1, 0x104, OpCode::CPUI_COPY);
        wire(&mut fd, out, r1, 0);
        assert_eq!(lone_descend(&fd, out), Some(r1));
        // two readers -> None
        let r2 = mk_op(&mut fd, bl, 1, 0x108, OpCode::CPUI_COPY);
        wire(&mut fd, out, r2, 0);
        assert_eq!(lone_descend(&fd, out), None);
    }

    // =====================================================================
    // RuleFuncPtrEncoding mask arithmetic (the (testmask & slide)==val core),
    // exercised via resolve plumbing once funcptr_align is unblocked.  We test
    // the slide computation matches the C++ (~0 << align) for align=1 here by
    // directly checking the constant the rule would compare against.
    // =====================================================================

    #[test]
    fn funcptr_slide_arithmetic_matches_cpp() {
        // For align=1 over an 8-byte mask: testmask = 0xff..ff, slide = ~0 << 1
        // = 0xff..fe, so the pattern matches a mask value of 0xff..fe (~1).
        let align: int4 = 1;
        let mut slide: u64 = !0u64;
        slide = slide.wrapping_shl(align as u32);
        let testmask = kuna_base::address::calc_mask(8);
        assert_eq!(testmask & slide, !1u64);
    }
}
