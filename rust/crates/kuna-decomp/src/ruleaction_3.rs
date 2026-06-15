//! Port of `decompiler/cpp/ruleaction.cc` lines 2819-4292 (W5, batch 3): the 22
//! [`Rule`] classes `RuleBooleanDedup` .. `RuleAddMultCollapse`.
//!
//! Each rule is a faithful transcription of its C++ `applyOp`/`getOpList` body:
//! the same guard order, the same op-graph edits in the same sequence, the same
//! constants and tie-breakers (rule bodies are the decompiler's *output*).  The
//! engine-owned per-rule state (name/group/flags) lives in the
//! [`RuleState`](crate::action::RuleState) the owning pool holds; the rule
//! structs here are zero-sized markers, exactly as the C++ rules carry no
//! per-instance data beyond the base.
//!
//! The C++ definition order (the order `specs()` returns, which W8 feeds to
//! `universalAction`) is:
//! `booleandedup, booleannegate, boolzext, logic2bool, indirectcollapse,
//! multicollapse, sborrow, scarry, trivialshift, signshift, testsign,
//! identityel, shift2mult, shiftpiece, collapseconstants, transformcpool,
//! propagatecopy, 2comp2mult, carryelim, sub2add, xorcollapse, addmultcollapse`.
//!
//! # Cross-wave seams (recorded as losses)
//!
//! Several rules reach subsystems that are not yet ported.  Where a rule hits
//! such a seam it transcribes every surrounding guard faithfully and returns `0`
//! ("rule did not apply", the conservative value the C++ also returns on every
//! early-out) at the precise seam point, with a `// SEAM(...)` note:
//!
//! * `SEAM(expression)` — `expression.cc` is a *separate* W5 file (still a stub):
//!   `BooleanMatch::evaluate` (RuleBooleanDedup), `AddExpression`
//!   (RuleSborrow/RuleScarry), `functionalEquality`/`cseFindInBlock` (the
//!   functional-equality branch of RuleMultiCollapse).
//! * `SEAM(W6)` — the `TypeFactory`/`OpBehavior`/cpool surfaces:
//!   `op->collapse`/`getArch()->getConstant`/`collapseConstantSymbol`
//!   (RuleCollapseConstants), the cpool record lookup (RuleTransformCpool),
//!   `Varnode::getSymbolEntry`/`copySymbolIfValid`/`updateType` (the symbol-copy
//!   tails of RuleXorCollapse / RuleAddMultCollapse).
//! * `SEAM(W4)` — `data.warning` (the partial-INDIRECT note in
//!   RuleIndirectCollapse), `getStoreGuard`/`LoadGuard` (the STORE-guard branch
//!   of RuleIndirectCollapse).
//!
//! Two infrastructure shims live in this file (not "type behavior" — the fixed
//! pcode-op property bits and the unique-output factory):
//!
//! * [`set_opcode`] resolves an [`OpCode`] to the seam [`TypeOp`] with the
//!   correct cached `opflags` (transcribed from `typeop.cc`'s `TypeOpXXX`
//!   constructors) and calls `opSetOpcode`.  This is the `glb->inst[opc]`
//!   resolution the W6 inst-table will own; the flag word is load-bearing
//!   (downstream `isMarker`/`isCommutative`/... read the op's cached flags).
//! * [`new_unique_out`] replicates `Funcdata::newUniqueOut`
//!   (`funcdata_varnode.cc:131`): `vbank.createDefUnique` + `op->setOutput`.  The
//!   `Funcdata`-level `newUniqueOut`/`opSetOutput` surface is blocked on a
//!   `banks_mut()` split-borrow accessor the serial-chain `funcdata` owner must
//!   add (documented in `funcdata_op.rs`), but the *unique* path needs no xref
//!   unification, so it is done here at the bank level (assignHigh is the W7
//!   no-op stub; checkForLanedRegister is skipped — SEAM(W7)).

use std::rc::Rc;

use kuna_base::address::calc_mask;
use kuna_base::error::KunaResult;
use kuna_base::space::spacetype;
use kuna_base::types::{int4, uintb};
use kuna_num::opcodes::OpCode;

use crate::action::{ActionGroupList, Rule, RuleSpec};
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;
use crate::op::pcodeop_flags;
use crate::seams::{OpId, TypeOp, VarnodeId};
use crate::varnode::DefOpInfo;

// =============================================================================
// W6 inst-table shim: OpCode -> cached opflags (transcribed from typeop.cc)
// =============================================================================

/// The cached `opflags` property word for an [`OpCode`], transcribed verbatim
/// from the `TypeOpXXX` constructors in `decompiler/cpp/typeop.cc`.
///
/// SEAM(W6): the real `glb->inst[opc]` (the `TypeFactory`-built `TypeOp` table)
/// owns this.  Only the opcodes these 22 rules ever *produce* via `opSetOpcode`
/// are listed; an unlisted opcode panics (it would mean a rule body produced an
/// opcode not covered here — an internal-invariant violation).
fn opflags_for(opc: OpCode) -> u32 {
    use pcodeop_flags::*;
    match opc {
        // TypeOpCopy: unary | nocollapse
        OpCode::CPUI_COPY => unary | nocollapse,
        // TypeOpBoolNegate: unary | booloutput
        OpCode::CPUI_BOOL_NEGATE => unary | booloutput,
        // TypeOpBoolAnd / BoolOr / BoolXor: binary | commutative | booloutput
        OpCode::CPUI_BOOL_AND | OpCode::CPUI_BOOL_OR | OpCode::CPUI_BOOL_XOR => {
            binary | commutative | booloutput
        }
        // TypeOpIntSless / IntSlessEqual / IntLessEqual: binary | booloutput
        OpCode::CPUI_INT_SLESS
        | OpCode::CPUI_INT_SLESSEQUAL
        | OpCode::CPUI_INT_LESSEQUAL => binary | booloutput,
        // TypeOpIntAdd / IntMult: binary | commutative
        OpCode::CPUI_INT_ADD | OpCode::CPUI_INT_MULT => binary | commutative,
        // TypeOpIntSright / IntSub / Piece / Subpiece: binary
        OpCode::CPUI_INT_SRIGHT
        | OpCode::CPUI_INT_SUB
        | OpCode::CPUI_PIECE
        | OpCode::CPUI_SUBPIECE => binary,
        // TypeOpIntZext / IntSext: unary
        OpCode::CPUI_INT_ZEXT | OpCode::CPUI_INT_SEXT => unary,
        // TypeOpIntXor / IntAnd / IntOr: binary | commutative (typeop.cc:1410/1443/1476)
        OpCode::CPUI_INT_XOR | OpCode::CPUI_INT_AND | OpCode::CPUI_INT_OR => {
            binary | commutative
        }
        // TypeOpIntNegate / Int2Comp: unary (typeop.cc:1396/1382)
        OpCode::CPUI_INT_NEGATE | OpCode::CPUI_INT_2COMP => unary,
        // TypeOpIntLeft / IntRight: binary (typeop.cc:1504/1529)
        OpCode::CPUI_INT_LEFT | OpCode::CPUI_INT_RIGHT => binary,
        // TypeOpIntLess: binary | booloutput (typeop.cc:1069)
        OpCode::CPUI_INT_LESS => binary | booloutput,
        other => panic!(
            "ruleaction_3::opflags_for: no W6 opflags entry for {other:?} \
             (a rule produced an opcode not covered by this seam shim)"
        ),
    }
}

/// `data.opSetOpcode(op, opc)` with the W6 inst-table resolution folded in.
///
/// SEAM(W6): builds the [`TypeOp`] from [`opflags_for`] and the debug name (the
/// `{opc:?}` rendering the merged production code uses), then routes through the
/// real [`Funcdata::op_set_opcode`].
fn set_opcode(data: &mut Funcdata, op: OpId, opc: OpCode) {
    data.op_set_opcode(op, TypeOp::new(opc, opflags_for(opc), format!("{opc:?}")));
}

/// `data.newUniqueOut(s, op)` (C++ `Funcdata::newUniqueOut`,
/// `funcdata_varnode.cc:131`): create a fresh unique Varnode as the output of
/// `op`.
///
/// `Datatype *ct = glb->types->getBase(s,TYPE_UNKNOWN);` (SEAM(W6), built as the
/// unknown-base skeleton exactly as the funcdata_varnode factories do);
/// `vn = vbank.createDefUnique(s,ct,op); op->setOutput(vn);`.  The fresh uniqid
/// guarantees `createDefUnique`'s xref never unifies, so the replace-reads
/// callback is the no-op (matching the C++, where a brand-new unique has no
/// pre-existing equivalent).  `assignHigh` is the W7 no-op stub and the
/// `checkForLanedRegister` laned-register probe is skipped (SEAM(W7); it affects
/// only later analysis, never the op graph this rule edits).
fn new_unique_out(data: &mut Funcdata, s: int4, op: OpId) -> VarnodeId {
    let ct: Rc<Datatype> = Rc::new(Datatype::new(s, type_metatype::TYPE_UNKNOWN));
    let seqnum = data.obank().get(op).expect("new_unique_out: stale op").get_seq_num().clone();
    let def = DefOpInfo { id: op, seqnum };
    let mut no_replace = |_: &mut crate::varnode::VarnodeBank,
                          _: VarnodeId,
                          _: VarnodeId|
     -> KunaResult<()> { Ok(()) };
    let vn = data
        .vbank_mut()
        .create_def_unique(s, ct, def, &mut no_replace)
        .expect("new_unique_out: createDefUnique on a fresh uniqid cannot collide");
    data.obank_mut().get_mut(op).expect("new_unique_out: stale op").set_output(Some(vn));
    vn
}

// =============================================================================
// Small Funcdata-mediated read helpers (mirror the C++ Varnode/PcodeOp getters)
// =============================================================================

/// `op->getIn(slot)` (panics on a stale op, as the C++ would deref a dangling
/// pointer — an internal invariant).
fn op_in(data: &Funcdata, op: OpId, slot: int4) -> Option<VarnodeId> {
    data.obank().get(op).expect("op_in: stale op").get_in(slot)
}

/// `op->getOut()`.
fn op_out(data: &Funcdata, op: OpId) -> Option<VarnodeId> {
    data.obank().get(op).expect("op_out: stale op").get_out()
}

/// `op->code()`.
fn op_code(data: &Funcdata, op: OpId) -> OpCode {
    data.obank().get(op).expect("op_code: stale op").code()
}

/// `op->numInput()`.
fn op_num_input(data: &Funcdata, op: OpId) -> int4 {
    data.obank().get(op).expect("op_num_input: stale op").num_input()
}

/// `vn->isConstant()`.
fn vn_is_constant(data: &Funcdata, vn: VarnodeId) -> bool {
    data.vbank().get(vn).expect("vn_is_constant: stale vn").is_constant()
}

/// `vn->isWritten()`.
fn vn_is_written(data: &Funcdata, vn: VarnodeId) -> bool {
    data.vbank().get(vn).expect("vn_is_written: stale vn").is_written()
}

/// `vn->isFree()`.
fn vn_is_free(data: &Funcdata, vn: VarnodeId) -> bool {
    data.vbank().get(vn).expect("vn_is_free: stale vn").is_free()
}

/// `vn->getOffset()`.
fn vn_offset(data: &Funcdata, vn: VarnodeId) -> uintb {
    data.vbank().get(vn).expect("vn_offset: stale vn").get_offset()
}

/// `vn->getSize()`.
fn vn_size(data: &Funcdata, vn: VarnodeId) -> int4 {
    data.vbank().get(vn).expect("vn_size: stale vn").get_size()
}

/// `vn->getDef()`.
fn vn_def(data: &Funcdata, vn: VarnodeId) -> Option<OpId> {
    data.vbank().get(vn).expect("vn_def: stale vn").get_def()
}

/// `vn->isBooleanValue(useAnnotation)` (C++ `Varnode::isBooleanValue`,
/// `varnode.cc`).  The written case is `def->isCalculatedBool()`; the
/// [`Varnode::is_boolean_value`](crate::varnode::Varnode::is_boolean_value) seam
/// defers that op-graph branch to "the op-aware caller" — these rules ARE that
/// caller, so the written-varnode branch is resolved here through the def op's
/// `isCalculatedBool()`.  The free/annotation cases stay in the varnode method.
fn vn_is_boolean_value(data: &Funcdata, vn: VarnodeId, use_annotation: bool) -> bool {
    if vn_is_written(data, vn) {
        // if (isWritten()) return def->isCalculatedBool();
        let def = vn_def(data, vn).expect("vn_is_boolean_value: written vn has no def");
        return data
            .obank()
            .get(def)
            .expect("vn_is_boolean_value: stale def op")
            .is_calculated_bool();
    }
    data.vbank().get(vn).expect("vn_is_boolean_value: stale vn").is_boolean_value(use_annotation)
}

/// `vn->getNZMask()`.
fn vn_nzmask(data: &Funcdata, vn: VarnodeId) -> uintb {
    data.vbank().get(vn).expect("vn_nzmask: stale vn").get_nz_mask()
}

/// `vn->isHeritageKnown()`.
fn vn_is_heritage_known(data: &Funcdata, vn: VarnodeId) -> bool {
    data.vbank().get(vn).expect("vn_is_heritage_known: stale vn").is_heritage_known()
}

/// `vn->isMark()`.
fn vn_is_mark(data: &Funcdata, vn: VarnodeId) -> bool {
    data.vbank().get(vn).expect("vn_is_mark: stale vn").is_mark()
}

/// `vn->setMark()`.
fn vn_set_mark(data: &mut Funcdata, vn: VarnodeId) {
    data.vbank_mut().get_mut(vn).expect("vn_set_mark: stale vn").set_mark();
}

/// `vn->clearMark()`.
fn vn_clear_mark(data: &mut Funcdata, vn: VarnodeId) {
    data.vbank_mut().get_mut(vn).expect("vn_clear_mark: stale vn").clear_mark();
}

// =============================================================================
// RuleBooleanDedup (ruleaction.cc:2819)
// =============================================================================

/// `RuleBooleanDedup` — deduplicate a boolean subexpression shared between the
/// two sides of a `BOOL_AND`/`BOOL_OR` (C++ `RuleBooleanDedup`).
#[derive(Clone, Default)]
pub struct RuleBooleanDedup;

impl Rule for RuleBooleanDedup {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_BOOL_AND, OpCode::CPUI_BOOL_OR]
    }

    fn clone_rule(&self, _grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        // C++ `clone`: returns null iff the rule's group is not in `grouplist`.
        // The rule's group lives in the engine-owned `RuleState` (not in this
        // zero-sized marker), and the pool only holds rules whose group W8
        // enabled, so the clone is unconditional here.  See module docs.
        Some(Box::new(RuleBooleanDedup))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        let vn0 = op_in(data, op, 0).expect("RuleBooleanDedup: null in0");
        if !vn_is_written(data, vn0) {
            return 0;
        }
        let vn1 = op_in(data, op, 1).expect("RuleBooleanDedup: null in1");
        if !vn_is_written(data, vn1) {
            return 0;
        }
        let op0 = vn_def(data, vn0).expect("RuleBooleanDedup: vn0 written but no def");
        let opc0 = op_code(data, op0);
        if opc0 != OpCode::CPUI_BOOL_AND && opc0 != OpCode::CPUI_BOOL_OR {
            return 0;
        }
        let op1 = vn_def(data, vn1).expect("RuleBooleanDedup: vn1 written but no def");
        let opc1 = op_code(data, op1);
        if opc1 != OpCode::CPUI_BOOL_AND && opc1 != OpCode::CPUI_BOOL_OR {
            return 0;
        }
        let ins = [
            op_in(data, op0, 0).expect("RuleBooleanDedup: op0 in0"),
            op_in(data, op0, 1).expect("RuleBooleanDedup: op0 in1"),
            op_in(data, op1, 0).expect("RuleBooleanDedup: op1 in0"),
            op_in(data, op1, 1).expect("RuleBooleanDedup: op1 in1"),
        ];
        if vn_is_free(data, ins[0])
            || vn_is_free(data, ins[1])
            || vn_is_free(data, ins[2])
            || vn_is_free(data, ins[3])
        {
            return 0;
        }
        // SEAM(expression): isMatch == BooleanMatch::evaluate (expression.cc),
        // which is a separate W5 file (still a stub).  Every guard above is
        // ported; the value-correlation test that selects the (leftA/rightA/
        // leftO/rightO) partition cannot run without it, so the rule
        // conservatively does not apply.  Recorded as a loss.
        let _ = (op, ins, opc0, opc1);
        0
    }
}

// =============================================================================
// RuleBooleanNegate (ruleaction.cc:2956)
// =============================================================================

/// `RuleBooleanNegate` — `V == false => !V`, `V == true => V` (C++
/// `RuleBooleanNegate`).
#[derive(Clone, Default)]
pub struct RuleBooleanNegate;

impl Rule for RuleBooleanNegate {
    fn get_op_list(&self) -> Vec<OpCode> {
        // { CPUI_INT_NOTEQUAL, CPUI_INT_EQUAL }
        vec![OpCode::CPUI_INT_NOTEQUAL, OpCode::CPUI_INT_EQUAL]
    }

    fn clone_rule(&self, _grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        // C++ `clone`: returns null iff the rule's group is not in `grouplist`.
        // The rule's group lives in the engine-owned `RuleState` (not in this
        // zero-sized marker), and the pool only holds rules whose group W8
        // enabled, so the clone is unconditional here.  See module docs.
        Some(Box::new(RuleBooleanNegate))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        let opc = op_code(data, op);
        let constvn = op_in(data, op, 1).expect("RuleBooleanNegate: null in1");
        let subbool = op_in(data, op, 0).expect("RuleBooleanNegate: null in0");
        if !vn_is_constant(data, constvn) {
            return 0;
        }
        let val = vn_offset(data, constvn);
        if (val != 0) && (val != 1) {
            return 0;
        }
        let mut negate = opc == OpCode::CPUI_INT_NOTEQUAL;
        if val == 0 {
            negate = !negate;
        }

        if !vn_is_boolean_value(data, subbool, data.is_type_recovery_on()) {
            return 0;
        }

        data.op_remove_input(op, 1); // Remove second parameter
        data.op_set_input(op, subbool, 0).expect("RuleBooleanNegate: opSetInput"); // Keep original boolean parameter
        if negate {
            set_opcode(data, op, OpCode::CPUI_BOOL_NEGATE);
        } else {
            set_opcode(data, op, OpCode::CPUI_COPY);
        }
        1
    }
}

// =============================================================================
// RuleBoolZext (ruleaction.cc:3000)
// =============================================================================

/// `RuleBoolZext` — simplify boolean expressions of the form `zext(V) * -1`
/// (C++ `RuleBoolZext`).
#[derive(Clone, Default)]
pub struct RuleBoolZext;

impl Rule for RuleBoolZext {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_INT_ZEXT]
    }

    fn clone_rule(&self, _grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        // C++ `clone`: returns null iff the rule's group is not in `grouplist`.
        // The rule's group lives in the engine-owned `RuleState` (not in this
        // zero-sized marker), and the pool only holds rules whose group W8
        // enabled, so the clone is unconditional here.  See module docs.
        Some(Box::new(RuleBoolZext))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        let bool_vn1 = op_in(data, op, 0).expect("RuleBoolZext: null in0");
        if !vn_is_boolean_value(data, bool_vn1, data.is_type_recovery_on()) {
            return 0;
        }

        let out0 = op_out(data, op).expect("RuleBoolZext: ZEXT has no output");
        let multop1 = match data.lone_descend(out0) {
            Some(o) => o,
            None => return 0,
        };
        if op_code(data, multop1) != OpCode::CPUI_INT_MULT {
            return 0;
        }
        let m1in1 = op_in(data, multop1, 1).expect("RuleBoolZext: multop1 in1");
        if !vn_is_constant(data, m1in1) {
            return 0;
        }
        let mut coeff = vn_offset(data, m1in1);
        if coeff != calc_mask(vn_size(data, m1in1)) {
            return 0;
        }
        let m1out = op_out(data, multop1).expect("RuleBoolZext: multop1 has no output");
        let size = vn_size(data, m1out);

        // If we reached here, we are Multiplying extended boolean by -1
        let actionop = match data.lone_descend(m1out) {
            Some(o) => o,
            None => return 0,
        };
        match op_code(data, actionop) {
            OpCode::CPUI_INT_ADD => {
                let aoin1 = op_in(data, actionop, 1).expect("RuleBoolZext: actionop in1");
                if !vn_is_constant(data, aoin1) {
                    return 0;
                }
                if vn_offset(data, aoin1) == 1 {
                    // newop = newOp(1,op->getAddr()); BOOL_NEGATE
                    let opaddr = data.obank().get(op).expect("RuleBoolZext: stale op").get_addr().clone();
                    let newop = data.new_op(1, opaddr);
                    set_opcode(data, newop, OpCode::CPUI_BOOL_NEGATE); // Negate the boolean
                    let vn = new_unique_out(data, 1, newop);
                    data.op_set_input(newop, bool_vn1, 0).expect("RuleBoolZext: opSetInput");
                    data.op_insert_before(newop, op);
                    data.op_set_input(op, vn, 0).expect("RuleBoolZext: opSetInput");
                    data.op_remove_input(actionop, 1); // eliminate the INT_ADD operator
                    set_opcode(data, actionop, OpCode::CPUI_COPY);
                    let opout = op_out(data, op).expect("RuleBoolZext: op out");
                    data.op_set_input(actionop, opout, 0).expect("RuleBoolZext: opSetInput"); // propagate past the INT_MULT operator
                    return 1;
                }
                0
            }
            OpCode::CPUI_INT_EQUAL | OpCode::CPUI_INT_NOTEQUAL => {
                let aoin1 = op_in(data, actionop, 1).expect("RuleBoolZext: actionop in1");
                let mut val = if vn_is_constant(data, aoin1) {
                    vn_offset(data, aoin1)
                } else {
                    return 0;
                };

                // Change comparison of extended boolean to 0 or -1
                // to comparison of unextended boolean to 0 or 1
                if val == coeff {
                    val = 1;
                } else if val != 0 {
                    return 0; // Not comparing with 0 or -1
                }

                data.op_set_input(actionop, bool_vn1, 0).expect("RuleBoolZext: opSetInput");
                let newc = data.new_constant(1, val);
                data.op_set_input(actionop, newc, 1).expect("RuleBoolZext: opSetInput");
                1
            }
            OpCode::CPUI_INT_AND | OpCode::CPUI_INT_OR | OpCode::CPUI_INT_XOR => {
                let opc = match op_code(data, actionop) {
                    OpCode::CPUI_INT_AND => OpCode::CPUI_BOOL_AND,
                    OpCode::CPUI_INT_OR => OpCode::CPUI_BOOL_OR,
                    OpCode::CPUI_INT_XOR => OpCode::CPUI_BOOL_XOR,
                    _ => unreachable!(),
                };

                // Apparently doing logical ops with extended boolean
                // Check that the other side is also an extended boolean
                let ao0 = op_in(data, actionop, 0).expect("RuleBoolZext: actionop in0");
                let ao1 = op_in(data, actionop, 1).expect("RuleBoolZext: actionop in1");
                let multop2 = if Some(multop1) == vn_def(data, ao0) {
                    vn_def(data, ao1)
                } else {
                    vn_def(data, ao0)
                };
                let multop2 = match multop2 {
                    Some(o) => o,
                    None => return 0,
                };
                if op_code(data, multop2) != OpCode::CPUI_INT_MULT {
                    return 0;
                }
                let m2in1 = op_in(data, multop2, 1).expect("RuleBoolZext: multop2 in1");
                if !vn_is_constant(data, m2in1) {
                    return 0;
                }
                coeff = vn_offset(data, m2in1);
                if coeff != calc_mask(size) {
                    return 0;
                }
                let m2in0 = op_in(data, multop2, 0).expect("RuleBoolZext: multop2 in0");
                let zextop2 = match vn_def(data, m2in0) {
                    Some(o) => o,
                    None => return 0,
                };
                if op_code(data, zextop2) != OpCode::CPUI_INT_ZEXT {
                    return 0;
                }
                let bool_vn2 = op_in(data, zextop2, 0).expect("RuleBoolZext: zextop2 in0");
                if !vn_is_boolean_value(data, bool_vn2, data.is_type_recovery_on()) {
                    return 0;
                }

                // Do the boolean calculation on unextended boolean values
                // and then extend the result
                let aoaddr =
                    data.obank().get(actionop).expect("RuleBoolZext: stale actionop").get_addr().clone();
                let newop = data.new_op(2, aoaddr.clone());
                let newres = new_unique_out(data, 1, newop);
                set_opcode(data, newop, opc);
                data.op_set_input(newop, bool_vn1, 0).expect("RuleBoolZext: opSetInput");
                data.op_set_input(newop, bool_vn2, 1).expect("RuleBoolZext: opSetInput");
                data.op_insert_before(newop, actionop);

                let newzext = data.new_op(1, aoaddr);
                let newzout = new_unique_out(data, size, newzext);
                set_opcode(data, newzext, OpCode::CPUI_INT_ZEXT);
                data.op_set_input(newzext, newres, 0).expect("RuleBoolZext: opSetInput");
                data.op_insert_before(newzext, actionop);

                set_opcode(data, actionop, OpCode::CPUI_INT_MULT);
                data.op_set_input(actionop, newzout, 0).expect("RuleBoolZext: opSetInput");
                let coeffc = data.new_constant(size, coeff);
                data.op_set_input(actionop, coeffc, 1).expect("RuleBoolZext: opSetInput");
                1
            }
            _ => 0,
        }
    }
}

// =============================================================================
// RuleLogic2Bool (ruleaction.cc:3125)
// =============================================================================

/// `RuleLogic2Bool` — `V & W => V && W`, `V | W => V || W` when both inputs are
/// booleans (C++ `RuleLogic2Bool`).
#[derive(Clone, Default)]
pub struct RuleLogic2Bool;

impl Rule for RuleLogic2Bool {
    fn get_op_list(&self) -> Vec<OpCode> {
        // { CPUI_INT_AND, CPUI_INT_OR, CPUI_INT_XOR }
        vec![OpCode::CPUI_INT_AND, OpCode::CPUI_INT_OR, OpCode::CPUI_INT_XOR]
    }

    fn clone_rule(&self, _grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        // C++ `clone`: returns null iff the rule's group is not in `grouplist`.
        // The rule's group lives in the engine-owned `RuleState` (not in this
        // zero-sized marker), and the pool only holds rules whose group W8
        // enabled, so the clone is unconditional here.  See module docs.
        Some(Box::new(RuleLogic2Bool))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        let bool_vn = op_in(data, op, 0).expect("RuleLogic2Bool: null in0");
        if !vn_is_boolean_value(data, bool_vn, data.is_type_recovery_on()) {
            return 0;
        }
        let in1 = op_in(data, op, 1).expect("RuleLogic2Bool: null in1");
        if vn_is_constant(data, in1) {
            if vn_offset(data, in1) > 1u64 {
                // If one side is a constant 0 or 1, this is boolean
                return 0;
            }
        } else if !vn_is_boolean_value(data, in1, data.is_type_recovery_on()) {
            return 0;
        }
        match op_code(data, op) {
            OpCode::CPUI_INT_AND => set_opcode(data, op, OpCode::CPUI_BOOL_AND),
            OpCode::CPUI_INT_OR => set_opcode(data, op, OpCode::CPUI_BOOL_OR),
            OpCode::CPUI_INT_XOR => set_opcode(data, op, OpCode::CPUI_BOOL_XOR),
            _ => return 0,
        }
        1
    }
}

// =============================================================================
// RuleIndirectCollapse (ruleaction.cc:3168)
// =============================================================================

/// `RuleIndirectCollapse` — remove a `CPUI_INDIRECT` if its blocking PcodeOp is
/// dead (C++ `RuleIndirectCollapse`).
#[derive(Clone, Default)]
pub struct RuleIndirectCollapse;

impl Rule for RuleIndirectCollapse {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_INDIRECT]
    }

    fn clone_rule(&self, _grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        // C++ `clone`: returns null iff the rule's group is not in `grouplist`.
        // The rule's group lives in the engine-owned `RuleState` (not in this
        // zero-sized marker), and the pool only holds rules whose group W8
        // enabled, so the clone is unconditional here.  See module docs.
        Some(Box::new(RuleIndirectCollapse))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        let in1 = op_in(data, op, 1).expect("RuleIndirectCollapse: null in1");
        // if (op->getIn(1)->getSpace()->getType()!=IPTR_IOP) return 0;
        if data.vbank().get(in1).expect("RuleIndirectCollapse: stale in1").get_space().get_type()
            != spacetype::IPTR_IOP
        {
            return 0;
        }
        // indop = PcodeOp::getOpFromConst(op->getIn(1)->getAddr());
        let indop = match op_from_iop_const(data, in1) {
            Some(o) => o,
            None => return 0,
        };

        // Is the indirect effect gone?
        let indop_dead = data.obank().get(indop).map(|o| o.is_dead()).unwrap_or(true);
        if !indop_dead {
            if op_code(data, indop) == OpCode::CPUI_COPY {
                // STORE resolved to a COPY
                let vn1 = op_out(data, indop).expect("RuleIndirectCollapse: indop COPY out");
                let vn2 = op_out(data, op).expect("RuleIndirectCollapse: op out");
                // int4 res = vn1->characterizeOverlap(*vn2);
                let res = {
                    let v1 = data.vbank().get(vn1).expect("RuleIndirectCollapse: stale vn1");
                    let v2 = data.vbank().get(vn2).expect("RuleIndirectCollapse: stale vn2");
                    v1.characterize_overlap(v2)
                };
                if res > 0 {
                    // Copy has an effect of some sort
                    if res == 2 {
                        // vn1 and vn2 are the same storage -> Convert INDIRECT to COPY
                        data.op_uninsert(op);
                        data.op_set_input(op, vn1, 0).expect("RuleIndirectCollapse: opSetInput");
                        data.op_remove_input(op, 1);
                        set_opcode(data, op, OpCode::CPUI_COPY);
                        data.op_insert_after(op, indop);
                        return 1;
                    }
                    // if (vn1->contains(*vn2) == 0) -- INDIRECT out properly contained in COPY out
                    let contains0 = {
                        let v1 = data.vbank().get(vn1).expect("RuleIndirectCollapse: stale vn1");
                        let v2 = data.vbank().get(vn2).expect("RuleIndirectCollapse: stale vn2");
                        v1.contains(v2) == 0
                    };
                    if contains0 {
                        // Convert INDIRECT to a SUBPIECE
                        let trunc = {
                            let v1 = data.vbank().get(vn1).expect("RuleIndirectCollapse: stale vn1");
                            let v2 = data.vbank().get(vn2).expect("RuleIndirectCollapse: stale vn2");
                            if v1.get_space().is_big_endian() {
                                // vn1->getOffset() + vn1->getSize() - (vn2->getOffset()+vn2->getSize())
                                (v1.get_offset().wrapping_add(v1.get_size() as uintb))
                                    .wrapping_sub(v2.get_offset().wrapping_add(v2.get_size() as uintb))
                            } else {
                                // vn2->getOffset() - vn1->getOffset()
                                v2.get_offset().wrapping_sub(v1.get_offset())
                            }
                        };
                        data.op_uninsert(op);
                        data.op_set_input(op, vn1, 0).expect("RuleIndirectCollapse: opSetInput");
                        let tc = data.new_constant(4, trunc);
                        data.op_set_input(op, tc, 1).expect("RuleIndirectCollapse: opSetInput");
                        set_opcode(data, op, OpCode::CPUI_SUBPIECE);
                        data.op_insert_after(op, indop);
                        return 1;
                    }
                    // SEAM(W4): data.warning("Ignoring partial resolution of indirect", indop->getAddr())
                    // The comment database is the W4 console seam; the message is
                    // dropped here.  The C++ then returns 0 (partial overlap).
                    return 0; // Partial overlap, not sure what to do
                }
            } else if data
                .vbank()
                .get(op_out(data, op).expect("RuleIndirectCollapse: op out"))
                .expect("RuleIndirectCollapse: stale op out")
                .has_no_local_alias()
            {
                let o = data.obank().get(op).expect("RuleIndirectCollapse: stale op");
                if o.is_indirect_creation() || o.no_indirect_collapse() {
                    return 0;
                }
            } else if data.obank().get(indop).expect("RuleIndirectCollapse: stale indop").uses_spacebase_ptr()
            {
                if op_code(data, indop) == OpCode::CPUI_STORE {
                    // SEAM(W4): const LoadGuard *guard = data.getStoreGuard(indop);
                    // getStoreGuard / LoadGuard are the heritage/W4 store-guard
                    // subsystem (not ported).  The C++ keeps the INDIRECT (returns
                    // 0) when the STORE is guarded for op's address OR when there
                    // is no guard yet; with the seam unported the conservative
                    // outcome is identical (keep the INDIRECT).  Recorded as a loss.
                    return 0;
                }
            } else {
                return 0;
            }
        }

        // data.totalReplace(op->getOut(),op->getIn(0));
        let out = op_out(data, op).expect("RuleIndirectCollapse: op out");
        let in0 = op_in(data, op, 0).expect("RuleIndirectCollapse: op in0");
        data.total_replace(out, in0).expect("RuleIndirectCollapse: totalReplace");
        data.op_destroy(op); // Get rid of the INDIRECT
        1
    }
}

/// `PcodeOp::getOpFromConst(vn->getAddr())` for an iop-space annotation Varnode:
/// decode the offset back to the [`OpId`] it encodes (the inverse of
/// `funcdata_varnode::op_iop_encode`, which packs the slotmap key's FFI form).
///
/// SEAM(W3-varnode): `funcdata_op.rs` documents the encode side as established by
/// `newVarnodeIop`; this is the matching decode.  Returns `None` if the decoded
/// key is stale (the op was destroyed).
fn op_from_iop_const(data: &Funcdata, vn: VarnodeId) -> Option<OpId> {
    use slotmap::KeyData;
    let off = vn_offset(data, vn);
    let key = OpId::from(KeyData::from_ffi(off));
    if data.obank().get(key).is_some() {
        Some(key)
    } else {
        // The C++ getOpFromConst always yields a live pointer (the INDIRECT and
        // its source share lifetimes); a stale key means the source already went
        // away, which the dead-path branch below treats as "effect is gone".
        None
    }
}

// =============================================================================
// RuleMultiCollapse (ruleaction.cc:3245)
// =============================================================================

/// `RuleMultiCollapse` — collapse a `MULTIEQUAL` whose inputs all trace to the
/// same value (C++ `RuleMultiCollapse`).
#[derive(Clone, Default)]
pub struct RuleMultiCollapse;

impl Rule for RuleMultiCollapse {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_MULTIEQUAL]
    }

    fn clone_rule(&self, _grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        // C++ `clone`: returns null iff the rule's group is not in `grouplist`.
        // The rule's group lives in the engine-owned `RuleState` (not in this
        // zero-sized marker), and the pool only holds rules whose group W8
        // enabled, so the clone is unconditional here.  See module docs.
        Some(Box::new(RuleMultiCollapse))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // Verbatim port of C++ `RuleMultiCollapse::applyOp` (ruleaction.cc:3253).
        // Collapses a MULTIEQUAL whose inputs are all absolutely or functionally
        // equal (the latter via cseFindInBlock).  This is the keystone for
        // spill/reload folding: two per-branch `COPY(reg)` inputs of a phi are
        // functionally equal and get CSE'd into one copy, emptying the COPY
        // blocks so the CFG can structure into `&&`/`||`.

        // for(i=0;i<numInput;++i) if (!getIn(i)->isHeritageKnown()) return 0;
        let numinput = op_num_input(data, op);
        for i in 0..numinput {
            let vi = op_in(data, op, i).expect("RuleMultiCollapse: null input");
            if !vn_is_heritage_known(data, vi) {
                return 0; // Everything must be heritaged before collapse
            }
        }

        let mut func_eq = false; // Start assuming absolute equality of branches
        let mut nofunc = false; // Functional equalities are initially allowed
        let mut defcopyr: Option<VarnodeId> = None;
        let mut j = 0usize;

        // for(i...) matchlist.push_back(op->getIn(i));
        let mut matchlist: Vec<VarnodeId> = Vec::with_capacity(numinput as usize);
        for i in 0..numinput {
            matchlist.push(op_in(data, op, i).expect("RuleMultiCollapse: null input"));
        }
        // Find base branch to match (first input not written by a MULTIEQUAL).
        for &copyr in matchlist.iter() {
            let is_multi = vn_is_written(data, copyr)
                && op_code(data, vn_def(data, copyr).expect("written vn has def"))
                    == OpCode::CPUI_MULTIEQUAL;
            if !is_multi {
                defcopyr = Some(copyr);
                break;
            }
        }

        let mut success = true;
        // op->getOut()->setMark(); skiplist.push_back(op->getOut());
        let outvn = op_out(data, op).expect("RuleMultiCollapse: MULTIEQUAL has no output");
        vn_set_mark(data, outvn);
        let mut skiplist: Vec<VarnodeId> = vec![outvn];

        while j < matchlist.len() {
            let copyr = matchlist[j];
            j += 1;
            if vn_is_mark(data, copyr) {
                continue; // A varnode we have seen before (loop construct) — skip
            }
            match defcopyr {
                None => {
                    // This is now the defining branch; all others must match.
                    defcopyr = Some(copyr);
                    if vn_is_written(data, copyr) {
                        if op_code(data, vn_def(data, copyr).expect("written vn has def"))
                            == OpCode::CPUI_MULTIEQUAL
                        {
                            nofunc = true; // MULTIEQUAL cannot match by functional equal
                        }
                    } else {
                        nofunc = true; // Unwritten cannot match by functional equal
                    }
                }
                Some(def) if def == copyr => {
                    continue; // A matching branch
                }
                Some(def)
                    if !nofunc
                        && crate::expression::functional_equality(
                            def,
                            copyr,
                            data.vbank(),
                            data.obank(),
                        ) =>
                {
                    func_eq = true; // Now matching by functional equality
                    continue;
                }
                _ => {
                    if vn_is_written(data, copyr)
                        && op_code(data, vn_def(data, copyr).expect("written vn has def"))
                            == OpCode::CPUI_MULTIEQUAL
                    {
                        // If the non-matching branch is a MULTIEQUAL, give it one
                        // last chance: add its inputs to the list of things to match.
                        let newop = vn_def(data, copyr).expect("written vn has def");
                        skiplist.push(copyr);
                        vn_set_mark(data, copyr);
                        let newin = op_num_input(data, newop);
                        for i in 0..newin {
                            matchlist.push(op_in(data, newop, i).expect("MULTIEQUAL null input"));
                        }
                    } else {
                        success = false; // A non-matching branch
                        break;
                    }
                }
            }
        }

        if success {
            let defcopyr = defcopyr.expect("RuleMultiCollapse: success implies a defining branch");
            // C++ iterates `skiplist` (fixed at this point — the collapse body never
            // pushes to it).  Snapshot it so the `&mut data` calls inside don't
            // alias the Vec.
            for copyr in skiplist.clone() {
                vn_clear_mark(data, copyr);
                let cur_op = vn_def(data, copyr).expect("skiplist varnode must be written");
                if func_eq {
                    // We have only functional equality.
                    let parent =
                        data.obank().get(cur_op).expect("stale op").get_parent().expect("op parent");
                    let earliest = data.block_earliest_use(parent, copyr);
                    let newop = vn_def(data, defcopyr).expect("defcopyr must be written");
                    // Has newop already been copied in this block?
                    let mut substitute: Option<OpId> = None;
                    let newin = op_num_input(data, newop);
                    for i in 0..newin {
                        let invn = op_in(data, newop, i).expect("newop null input");
                        if !vn_is_constant(data, invn) {
                            substitute = data.cse_find_in_block(newop, invn, parent, earliest);
                            break;
                        }
                    }
                    if let Some(substitute) = substitute {
                        // Already copied: use the copy's output as substitute.
                        let subout = op_out(data, substitute).expect("substitute has no output");
                        data.total_replace(copyr, subout).expect("RuleMultiCollapse: totalReplace");
                        data.op_destroy(cur_op);
                    } else {
                        // Otherwise, rewrite cur_op into a copy of newop.
                        let needsreinsert = op_code(data, cur_op) == OpCode::CPUI_MULTIEQUAL;
                        let mut parms: Vec<VarnodeId> = Vec::with_capacity(newin as usize);
                        for i in 0..newin {
                            parms.push(op_in(data, newop, i).expect("newop null input"));
                        }
                        data.op_set_all_input(cur_op, &parms)
                            .expect("RuleMultiCollapse: opSetAllInput");
                        set_opcode(data, cur_op, op_code(data, newop));
                        if needsreinsert {
                            // If the op is no longer a MULTIEQUAL, re-insert it after
                            // any other MULTIEQUAL at the block head.
                            let bl = data
                                .obank()
                                .get(cur_op)
                                .expect("stale op")
                                .get_parent()
                                .expect("op parent");
                            data.op_uninsert(cur_op);
                            data.op_insert_begin(cur_op, bl);
                        }
                    }
                } else {
                    // We have absolute equality: replace all refs to copyr with
                    // defcopyr and destroy the MULTIEQUAL.
                    data.total_replace(copyr, defcopyr).expect("RuleMultiCollapse: totalReplace");
                    data.op_destroy(cur_op);
                }
            }
            return 1;
        }

        // Failure: clear out any marks we put down.
        for &v in skiplist.iter() {
            vn_clear_mark(data, v);
        }
        0
    }
}

// =============================================================================
// RuleSborrow (ruleaction.cc:3364)
// =============================================================================

/// `RuleSborrow` — simplify signed comparisons using `INT_SBORROW` (C++
/// `RuleSborrow`).
#[derive(Clone, Default)]
pub struct RuleSborrow;

impl Rule for RuleSborrow {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_INT_SBORROW]
    }

    fn clone_rule(&self, _grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        // C++ `clone`: returns null iff the rule's group is not in `grouplist`.
        // The rule's group lives in the engine-owned `RuleState` (not in this
        // zero-sized marker), and the pool only holds rules whose group W8
        // enabled, so the clone is unconditional here.  See module docs.
        Some(Box::new(RuleSborrow))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        let bvn = op_in(data, op, 1).expect("RuleSborrow: null in1");
        // Check for trivial case: sborrow(V,0) => false
        if vn_is_constant(data, bvn) && vn_offset(data, bvn) == 0 {
            set_opcode(data, op, OpCode::CPUI_COPY);
            let z = data.new_constant(1, 0);
            data.op_set_input(op, z, 0).expect("RuleSborrow: opSetInput");
            data.op_remove_input(op, 1);
            return 1;
        }
        // SEAM(expression): the comparison-rewrite branch needs AddExpression
        // (gatherTwoTermsSubtract/gatherTwoTermsRoot/isEquivalent), defined in
        // expression.cc (a separate, unported W5 file).  The trivial case above
        // is ported; the descend-walk rewrite is deferred.  Recorded as a loss.
        0
    }
}

// =============================================================================
// RuleScarry (ruleaction.cc:3433)
// =============================================================================

/// `RuleScarry` — simplify signed comparisons using `INT_SCARRY` (C++
/// `RuleScarry`).
#[derive(Clone, Default)]
pub struct RuleScarry;

impl Rule for RuleScarry {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_INT_SCARRY]
    }

    fn clone_rule(&self, _grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        // C++ `clone`: returns null iff the rule's group is not in `grouplist`.
        // The rule's group lives in the engine-owned `RuleState` (not in this
        // zero-sized marker), and the pool only holds rules whose group W8
        // enabled, so the clone is unconditional here.  See module docs.
        Some(Box::new(RuleScarry))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        let avn = op_in(data, op, 0).expect("RuleScarry: null in0");
        let bvn = op_in(data, op, 1).expect("RuleScarry: null in1");

        // Check for trivial case: scarry(V,0) => false (either operand zero)
        if (vn_is_constant(data, bvn) && vn_offset(data, bvn) == 0)
            || (vn_is_constant(data, avn) && vn_offset(data, avn) == 0)
        {
            set_opcode(data, op, OpCode::CPUI_COPY);
            let z = data.new_constant(1, 0);
            data.op_set_input(op, z, 0).expect("RuleScarry: opSetInput");
            data.op_remove_input(op, 1);
            return 1;
        }
        // One side must be constant.  (The swap-to-canonical and the
        // integer-minimum guard are part of the AddExpression rewrite below.)
        // SEAM(expression): the rewrite needs AddExpression
        // (gatherTwoTermsAdd/gatherTwoTermsRoot/isEquivalent) from expression.cc
        // (unported W5 file).  The trivial case above is ported; the rest is
        // deferred.  Recorded as a loss.
        0
    }
}

// =============================================================================
// RuleTrivialShift (ruleaction.cc:3515)
// =============================================================================

/// `RuleTrivialShift` — `V << 0 => V`, `V << #64 => 0` (C++ `RuleTrivialShift`).
#[derive(Clone, Default)]
pub struct RuleTrivialShift;

impl Rule for RuleTrivialShift {
    fn get_op_list(&self) -> Vec<OpCode> {
        // { CPUI_INT_LEFT, CPUI_INT_RIGHT, CPUI_INT_SRIGHT }
        vec![OpCode::CPUI_INT_LEFT, OpCode::CPUI_INT_RIGHT, OpCode::CPUI_INT_SRIGHT]
    }

    fn clone_rule(&self, _grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        // C++ `clone`: returns null iff the rule's group is not in `grouplist`.
        // The rule's group lives in the engine-owned `RuleState` (not in this
        // zero-sized marker), and the pool only holds rules whose group W8
        // enabled, so the clone is unconditional here.  See module docs.
        Some(Box::new(RuleTrivialShift))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        let constvn = op_in(data, op, 1).expect("RuleTrivialShift: null in1");
        if !vn_is_constant(data, constvn) {
            return 0; // Must shift by a constant
        }
        let val = vn_offset(data, constvn);
        if val != 0 {
            let in0 = op_in(data, op, 0).expect("RuleTrivialShift: null in0");
            // if (val < 8*op->getIn(0)->getSize()) return 0;  -- Non-trivial
            if val < (8 * vn_size(data, in0)) as uintb {
                return 0;
            }
            if op_code(data, op) == OpCode::CPUI_INT_SRIGHT {
                return 0; // Cant predict signbit
            }
            let replace = data.new_constant(vn_size(data, in0), 0);
            data.op_set_input(op, replace, 0).expect("RuleTrivialShift: opSetInput");
        }
        data.op_remove_input(op, 1);
        set_opcode(data, op, OpCode::CPUI_COPY);
        1
    }
}

// =============================================================================
// RuleSignShift (ruleaction.cc:3543)
// =============================================================================

/// `RuleSignShift` — normalize sign-bit extraction `V >> 0x1f => (V s>> 0x1f) *
/// -1` when it feeds arithmetic/comparison (C++ `RuleSignShift`).
#[derive(Clone, Default)]
pub struct RuleSignShift;

impl Rule for RuleSignShift {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_INT_RIGHT]
    }

    fn clone_rule(&self, _grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        // C++ `clone`: returns null iff the rule's group is not in `grouplist`.
        // The rule's group lives in the engine-owned `RuleState` (not in this
        // zero-sized marker), and the pool only holds rules whose group W8
        // enabled, so the clone is unconditional here.  See module docs.
        Some(Box::new(RuleSignShift))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        let const_vn = op_in(data, op, 1).expect("RuleSignShift: null in1");
        if !vn_is_constant(data, const_vn) {
            return 0;
        }
        let val = vn_offset(data, const_vn);
        let in_vn = op_in(data, op, 0).expect("RuleSignShift: null in0");
        if val != (8 * vn_size(data, in_vn) - 1) as uintb {
            return 0;
        }
        if vn_is_free(data, in_vn) {
            return 0;
        }

        let mut do_conversion = false;
        let out_vn = op_out(data, op).expect("RuleSignShift: op out");
        // while iterating descendants: a re-entrant-safe snapshot (the C++ holds a
        // live const_iterator it pre-increments; we snapshot the descend list).
        for arith_op in data.descend_snapshot(out_vn) {
            match op_code(data, arith_op) {
                OpCode::CPUI_INT_EQUAL | OpCode::CPUI_INT_NOTEQUAL => {
                    let a1 = op_in(data, arith_op, 1).expect("RuleSignShift: arithOp in1");
                    if vn_is_constant(data, a1) {
                        do_conversion = true;
                    }
                }
                OpCode::CPUI_INT_ADD | OpCode::CPUI_INT_MULT => {
                    do_conversion = true;
                }
                _ => {}
            }
            if do_conversion {
                break;
            }
        }
        if !do_conversion {
            return 0;
        }
        let opaddr = data.obank().get(op).expect("RuleSignShift: stale op").get_addr().clone();
        let shift_op = data.new_op(2, opaddr);
        set_opcode(data, shift_op, OpCode::CPUI_INT_SRIGHT);
        let unique_vn = new_unique_out(data, vn_size(data, in_vn), shift_op);
        data.op_set_input(op, unique_vn, 0).expect("RuleSignShift: opSetInput");
        let mask = data.new_constant(vn_size(data, in_vn), calc_mask(vn_size(data, in_vn)));
        data.op_set_input(op, mask, 1).expect("RuleSignShift: opSetInput");
        set_opcode(data, op, OpCode::CPUI_INT_MULT);
        data.op_set_input(shift_op, in_vn, 0).expect("RuleSignShift: opSetInput");
        data.op_set_input(shift_op, const_vn, 1).expect("RuleSignShift: opSetInput");
        data.op_insert_before(shift_op, op);
        1
    }
}

// =============================================================================
// RuleTestSign (ruleaction.cc:3601)
// =============================================================================

/// `RuleTestSign` — convert a sign-bit test to a signed comparison `(V s>> 0x1f)
/// != 0 => V s< 0` (C++ `RuleTestSign`).
#[derive(Clone, Default)]
pub struct RuleTestSign;

impl RuleTestSign {
    /// `RuleTestSign::findComparisons` (ruleaction.cc:3615): trace `vn` to the
    /// `INT_EQUAL`/`INT_NOTEQUAL` ops that compare it to a constant.
    fn find_comparisons(data: &Funcdata, vn: VarnodeId, res: &mut Vec<OpId>) {
        for cmpop in data.descend_snapshot(vn) {
            let opc = op_code(data, cmpop);
            if opc == OpCode::CPUI_INT_EQUAL || opc == OpCode::CPUI_INT_NOTEQUAL {
                let c1 = op_in(data, cmpop, 1).expect("RuleTestSign: cmp in1");
                if vn_is_constant(data, c1) {
                    res.push(cmpop);
                }
            }
        }
    }
}

impl Rule for RuleTestSign {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_INT_SRIGHT]
    }

    fn clone_rule(&self, _grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        // C++ `clone`: returns null iff the rule's group is not in `grouplist`.
        // The rule's group lives in the engine-owned `RuleState` (not in this
        // zero-sized marker), and the pool only holds rules whose group W8
        // enabled, so the clone is unconditional here.  See module docs.
        Some(Box::new(RuleTestSign))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        let const_vn = op_in(data, op, 1).expect("RuleTestSign: null in1");
        if !vn_is_constant(data, const_vn) {
            return 0;
        }
        let val = vn_offset(data, const_vn);
        let in_vn = op_in(data, op, 0).expect("RuleTestSign: null in0");
        if val != (8 * vn_size(data, in_vn) - 1) as uintb {
            return 0;
        }
        let out_vn = op_out(data, op).expect("RuleTestSign: op out");

        if vn_is_free(data, in_vn) {
            return 0;
        }
        let mut compare_ops: Vec<OpId> = Vec::new();
        Self::find_comparisons(data, out_vn, &mut compare_ops);
        let mut result_code = 0;
        for compare_op in compare_ops {
            let comp_vn = op_in(data, compare_op, 0).expect("RuleTestSign: comp in0");
            let comp_size = vn_size(data, comp_vn);

            let offset = vn_offset(data, op_in(data, compare_op, 1).expect("RuleTestSign: comp in1"));
            let mut sgn: int4 = if offset == 0 {
                1
            } else if offset == calc_mask(comp_size) {
                -1
            } else {
                continue;
            };
            if op_code(data, compare_op) == OpCode::CPUI_INT_NOTEQUAL {
                sgn = -sgn; // Complement the domain
            }

            let zero_vn = data.new_constant(vn_size(data, in_vn), 0);
            if sgn == 1 {
                data.op_set_input(compare_op, in_vn, 1).expect("RuleTestSign: opSetInput");
                data.op_set_input(compare_op, zero_vn, 0).expect("RuleTestSign: opSetInput");
                set_opcode(data, compare_op, OpCode::CPUI_INT_SLESSEQUAL);
            } else {
                data.op_set_input(compare_op, in_vn, 0).expect("RuleTestSign: opSetInput");
                data.op_set_input(compare_op, zero_vn, 1).expect("RuleTestSign: opSetInput");
                set_opcode(data, compare_op, OpCode::CPUI_INT_SLESS);
            }
            result_code = 1;
        }
        result_code
    }
}

// =============================================================================
// RuleIdentityEl (ruleaction.cc:3678)
// =============================================================================

/// `RuleIdentityEl` — collapse operations using their identity element `V + 0 =>
/// V`, `V * 1 => V`, etc. (C++ `RuleIdentityEl`).
#[derive(Clone, Default)]
pub struct RuleIdentityEl;

impl Rule for RuleIdentityEl {
    fn get_op_list(&self) -> Vec<OpCode> {
        // { CPUI_INT_ADD, CPUI_INT_XOR, CPUI_INT_OR, CPUI_BOOL_XOR, CPUI_BOOL_OR, CPUI_INT_MULT }
        vec![
            OpCode::CPUI_INT_ADD,
            OpCode::CPUI_INT_XOR,
            OpCode::CPUI_INT_OR,
            OpCode::CPUI_BOOL_XOR,
            OpCode::CPUI_BOOL_OR,
            OpCode::CPUI_INT_MULT,
        ]
    }

    fn clone_rule(&self, _grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        // C++ `clone`: returns null iff the rule's group is not in `grouplist`.
        // The rule's group lives in the engine-owned `RuleState` (not in this
        // zero-sized marker), and the pool only holds rules whose group W8
        // enabled, so the clone is unconditional here.  See module docs.
        Some(Box::new(RuleIdentityEl))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        let constvn = op_in(data, op, 1).expect("RuleIdentityEl: null in1");
        if !vn_is_constant(data, constvn) {
            return 0;
        }
        let val = vn_offset(data, constvn);
        if (val == 0) && (op_code(data, op) != OpCode::CPUI_INT_MULT) {
            set_opcode(data, op, OpCode::CPUI_COPY);
            data.op_remove_input(op, 1); // Remove identity from operation
            return 1;
        }
        if op_code(data, op) != OpCode::CPUI_INT_MULT {
            return 0;
        }
        if val == 1 {
            set_opcode(data, op, OpCode::CPUI_COPY);
            data.op_remove_input(op, 1);
            return 1;
        }
        if val == 0 {
            // Multiply by zero
            set_opcode(data, op, OpCode::CPUI_COPY);
            data.op_remove_input(op, 0);
            return 1;
        }
        0
    }
}

// =============================================================================
// RuleShift2Mult (ruleaction.cc:3723)
// =============================================================================

/// `RuleShift2Mult` — `V << 2 => V * 4` when the result feeds arithmetic (C++
/// `RuleShift2Mult`).
#[derive(Clone, Default)]
pub struct RuleShift2Mult;

impl Rule for RuleShift2Mult {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_INT_LEFT]
    }

    fn clone_rule(&self, _grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        // C++ `clone`: returns null iff the rule's group is not in `grouplist`.
        // The rule's group lives in the engine-owned `RuleState` (not in this
        // zero-sized marker), and the pool only holds rules whose group W8
        // enabled, so the clone is unconditional here.  See module docs.
        Some(Box::new(RuleShift2Mult))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        let mut flag = 0;
        let vn = op_out(data, op).expect("RuleShift2Mult: op out");
        let constvn = op_in(data, op, 1).expect("RuleShift2Mult: null in1");
        if !vn_is_constant(data, constvn) {
            return 0; // Shift amount must be a constant
        }
        // int4 val = constvn->getOffset();
        let val = vn_offset(data, constvn) as int4;
        if val >= 32 {
            // FIXME: arbitrary -- this big is probably not an arithmetic multiply
            return 0;
        }
        // arithop = op->getIn(0)->getDef();  -- may be null
        let in0 = op_in(data, op, 0).expect("RuleShift2Mult: null in0");
        let mut arithop = vn_def(data, in0);
        // desc = vn->beginDescend(); the C++ loop probes arithop then steps desc.
        let descend = data.descend_snapshot(vn);
        let mut desc_idx = 0;
        loop {
            if let Some(ao) = arithop {
                let opc = op_code(data, ao);
                if opc == OpCode::CPUI_INT_ADD
                    || opc == OpCode::CPUI_INT_SUB
                    || opc == OpCode::CPUI_INT_MULT
                {
                    flag = 1;
                    break;
                }
            }
            // if (desc == vn->endDescend()) break;
            if desc_idx == descend.len() {
                break;
            }
            // arithop = *desc++;
            arithop = Some(descend[desc_idx]);
            desc_idx += 1;
        }

        if flag == 0 {
            return 0;
        }
        // constvn = data.newConstant(vn->getSize(), ((uintb)1)<<val);
        let newconst = data.new_constant(vn_size(data, vn), 1u64 << val);
        data.op_set_input(op, newconst, 1).expect("RuleShift2Mult: opSetInput");
        set_opcode(data, op, OpCode::CPUI_INT_MULT);
        1
    }
}

// =============================================================================
// RuleShiftPiece (ruleaction.cc:3772)
// =============================================================================

/// `RuleShiftPiece` — convert "shift and add" to `PIECE`:
/// `(zext(V) << 16) + zext(W) => concat(V,W)` (C++ `RuleShiftPiece`).
#[derive(Clone, Default)]
pub struct RuleShiftPiece;

impl Rule for RuleShiftPiece {
    fn get_op_list(&self) -> Vec<OpCode> {
        // push_back INT_OR, INT_XOR, INT_ADD (in this order)
        vec![OpCode::CPUI_INT_OR, OpCode::CPUI_INT_XOR, OpCode::CPUI_INT_ADD]
    }

    fn clone_rule(&self, _grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        // C++ `clone`: returns null iff the rule's group is not in `grouplist`.
        // The rule's group lives in the engine-owned `RuleState` (not in this
        // zero-sized marker), and the pool only holds rules whose group W8
        // enabled, so the clone is unconditional here.  See module docs.
        Some(Box::new(RuleShiftPiece))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        let mut vn1 = op_in(data, op, 0).expect("RuleShiftPiece: null in0");
        if !vn_is_written(data, vn1) {
            return 0;
        }
        let mut vn2 = op_in(data, op, 1).expect("RuleShiftPiece: null in1");
        if !vn_is_written(data, vn2) {
            return 0;
        }
        let mut shiftop = vn_def(data, vn1).expect("RuleShiftPiece: vn1 def");
        let mut zextloop = vn_def(data, vn2).expect("RuleShiftPiece: vn2 def");
        if op_code(data, shiftop) != OpCode::CPUI_INT_LEFT {
            if op_code(data, zextloop) != OpCode::CPUI_INT_LEFT {
                return 0;
            }
            std::mem::swap(&mut zextloop, &mut shiftop);
        }
        let shin1 = op_in(data, shiftop, 1).expect("RuleShiftPiece: shiftop in1");
        if !vn_is_constant(data, shin1) {
            return 0;
        }
        vn1 = op_in(data, shiftop, 0).expect("RuleShiftPiece: shiftop in0");
        if !vn_is_written(data, vn1) {
            return 0;
        }
        let zexthiop = vn_def(data, vn1).expect("RuleShiftPiece: vn1 def");
        if op_code(data, zexthiop) != OpCode::CPUI_INT_ZEXT
            && op_code(data, zexthiop) != OpCode::CPUI_INT_SEXT
        {
            return 0;
        }
        vn1 = op_in(data, zexthiop, 0).expect("RuleShiftPiece: zexthiop in0");
        if vn_is_constant(data, vn1) {
            // if (vn1->getSize() < sizeof(uintb)) return 0;  -- sizeof(uintb)==8
            if (vn_size(data, vn1) as usize) < std::mem::size_of::<uintb>() {
                return 0; // Normally we let ZEXT of a constant collapse naturally
            }
            // But if the ZEXTed constant is too big, this won't happen
        } else if vn_is_free(data, vn1) {
            return 0;
        }
        // int4 sa = shiftop->getIn(1)->getOffset();
        let sa = vn_offset(data, shin1) as int4;
        let concatsize = sa + 8 * vn_size(data, vn1);
        let op_out_size = vn_size(data, op_out(data, op).expect("RuleShiftPiece: op out"));
        if op_out_size * 8 < concatsize {
            return 0;
        }
        if op_code(data, zextloop) != OpCode::CPUI_INT_ZEXT {
            // Special case triggered by CDQ: IDIV (interacts with RuleSubZext)
            if !vn_is_written(data, vn1) {
                return 0;
            }
            let r_shift_op = vn_def(data, vn1).expect("RuleShiftPiece: vn1 def");
            if op_code(data, r_shift_op) != OpCode::CPUI_INT_SRIGHT {
                return 0;
            }
            let rsh1 = op_in(data, r_shift_op, 1).expect("RuleShiftPiece: rShiftOp in1");
            if !vn_is_constant(data, rsh1) {
                return 0;
            }
            vn2 = op_in(data, r_shift_op, 0).expect("RuleShiftPiece: rShiftOp in0");
            if !vn_is_written(data, vn2) {
                return 0;
            }
            let subop = vn_def(data, vn2).expect("RuleShiftPiece: vn2 def");
            if op_code(data, subop) != OpCode::CPUI_SUBPIECE {
                return 0; // SUBPIECE connects high and low parts
            }
            if vn_offset(data, op_in(data, subop, 1).expect("RuleShiftPiece: subop in1")) != 0 {
                return 0; // (must be low part)
            }
            let big_vn = op_out(data, zextloop).expect("RuleShiftPiece: zextloop out");
            if op_in(data, subop, 0) != Some(big_vn) {
                return 0; // Verify we have link thru SUBPIECE with low part
            }
            let rsa = vn_offset(data, rsh1) as int4;
            if rsa != vn_size(data, vn2) * 8 - 1 {
                return 0; // Arithmetic shift must copy sign-bit thru whole high part
            }
            if (vn_nzmask(data, big_vn) >> sa) != 0 {
                return 0; // The original most significant bytes must be zero
            }
            if sa != 8 * vn_size(data, vn2) {
                return 0;
            }
            set_opcode(data, op, OpCode::CPUI_INT_SEXT); // Original op is sign extension of low part
            data.op_set_input(op, vn2, 0).expect("RuleShiftPiece: opSetInput");
            data.op_remove_input(op, 1);
            return 1;
        }
        vn2 = op_in(data, zextloop, 0).expect("RuleShiftPiece: zextloop in0");
        if vn_is_free(data, vn2) {
            return 0;
        }
        if sa != 8 * vn_size(data, vn2) {
            return 0;
        }
        if concatsize == op_out_size * 8 {
            set_opcode(data, op, OpCode::CPUI_PIECE);
            data.op_set_input(op, vn1, 0).expect("RuleShiftPiece: opSetInput");
            data.op_set_input(op, vn2, 1).expect("RuleShiftPiece: opSetInput");
        } else {
            let opaddr = data.obank().get(op).expect("RuleShiftPiece: stale op").get_addr().clone();
            let newop = data.new_op(2, opaddr);
            new_unique_out(data, concatsize / 8, newop);
            set_opcode(data, newop, OpCode::CPUI_PIECE);
            data.op_set_input(newop, vn1, 0).expect("RuleShiftPiece: opSetInput");
            data.op_set_input(newop, vn2, 1).expect("RuleShiftPiece: opSetInput");
            data.op_insert_before(newop, op);
            let zext_code = op_code(data, zexthiop);
            set_opcode(data, op, zext_code);
            data.op_remove_input(op, 1);
            let newout = op_out(data, newop).expect("RuleShiftPiece: newop out");
            data.op_set_input(op, newout, 0).expect("RuleShiftPiece: opSetInput");
        }
        1
    }
}

// =============================================================================
// RuleCollapseConstants (ruleaction.cc:3871)
// =============================================================================

/// `RuleCollapseConstants` — collapse a constant expression to a `COPY` of the
/// computed constant (C++ `RuleCollapseConstants`).
#[derive(Clone, Default)]
pub struct RuleCollapseConstants;

impl Rule for RuleCollapseConstants {
    // applies to all opcodes (no getOpList override)

    fn clone_rule(&self, _grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        // C++ `clone`: returns null iff the rule's group is not in `grouplist`.
        // The rule's group lives in the engine-owned `RuleState` (not in this
        // zero-sized marker), and the pool only holds rules whose group W8
        // enabled, so the clone is unconditional here.  See module docs.
        Some(Box::new(RuleCollapseConstants))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // if (!op->isCollapsible()) return 0;  -- Expression must be collapsible
        let collapsible = {
            let o = data.obank().get(op).expect("RuleCollapseConstants: stale op");
            crate::op::is_collapsible(o, data.vbank())
        };
        if !collapsible {
            return 0;
        }
        // newval = data.getArch()->getConstant(op->collapse(markedInput));
        //   op->collapse() drives the OpBehavior emulation (executeSimple) on the
        //   op's constant inputs; getArch()->getConstant wraps the result in a
        //   constant-space Address.  On any evaluation error the C++ catches the
        //   LowlevelError and `opMarkNoCollapse`s the op (latching the failure).
        let (collapsed, marked_input) = match dc_collapse(op, data) {
            Some(v) => v,
            None => {
                // opMarkNoCollapse(op): don't know how / don't want to collapse.
                data.op_mark_no_collapse(op);
                return 0;
            }
        };
        let out_size =
            data.vbank().get(data.obank().get(op).unwrap().get_out().unwrap()).unwrap().get_size();
        // vn = data.newVarnode(out->getSize(), getArch()->getConstant(collapsed));
        let newval = data.get_arch().get_constant(collapsed);
        let vn = data.new_varnode(out_size, &newval, None);
        // collapseConstantSymbol(vn) — SEAM(W4): symbol propagation needs
        // `getSymbolEntry` (no symbols recovered yet); markedInput is always
        // false in the merged tree, so the symbol copy is a faithful no-op.
        let _ = marked_input;
        // for(i=numInput()-1;i>0;--i) opRemoveInput(op,i);  -- unlink old constants
        let n = data.obank().get(op).expect("RuleCollapseConstants: stale op").num_input();
        let mut i = n - 1;
        while i > 0 {
            data.op_remove_input(op, i);
            i -= 1;
        }
        // opSetInput(op, vn, 0);  opSetOpcode(op, CPUI_COPY);
        data.op_set_input(op, vn, 0).expect("RuleCollapseConstants: opSetInput");
        data.op_set_opcode(op, crate::typeop::type_op_for(OpCode::CPUI_COPY));
        1
    }
}

/// C++ `PcodeOp::collapse(bool &markedInput)` (op.cc:473): assuming every input
/// is constant, evaluate the op to its constant result via the engine's
/// [`OpBehavior`](kuna_num::opbehavior::OpBehavior) table.  Returns
/// `Some((value, markedInput))` on success, `None` if the op cannot be
/// constant-folded (the C++ throws `LowlevelError` -> caught -> opMarkNoCollapse).
///
/// `markedInput` mirrors the C++ "one of the inputs carries symbol info"
/// pass-back; with the W4 symbol seam (no `getSymbolEntry`) it is always false.
fn dc_collapse(op: OpId, data: &Funcdata) -> Option<(u64, bool)> {
    let (eval_type, opc, num_input, in0, in1, out_size) = {
        let o = data.obank().get(op).expect("collapse: stale op");
        let out = o.get_out().expect("collapse: collapsible op has no output");
        let out_size = data.vbank().get(out).expect("collapse: stale out").get_size();
        let n = o.num_input();
        // get_in panics on an out-of-range slot, so guard slot 1 by num_input
        // (the C++ `getIn(1)` is only reached in the `binary` arm).
        let in1 = if n > 1 { o.get_in(1) } else { None };
        (o.get_eval_type(), o.code(), n, o.get_in(0), in1, out_size)
    };
    let _ = num_input;
    let behave = data.get_arch().op_behavior(opc)?;
    let in0 = in0?;
    let (in0_size, in0_off) = {
        let v = data.vbank().get(in0).expect("collapse: stale in0");
        (v.get_size(), v.get_offset())
    };
    // markedInput stays false (getSymbolEntry is a W4 seam; see doc above).
    let marked_input = false;
    if (eval_type & pcodeop_flags::unary) != 0 {
        match behave.evaluate_unary(out_size, in0_size, in0_off) {
            Ok(v) => Some((v, marked_input)),
            Err(_) => None, // LowlevelError -> opMarkNoCollapse
        }
    } else if (eval_type & pcodeop_flags::binary) != 0 {
        let in1 = in1?;
        let in1_off = data.vbank().get(in1).expect("collapse: stale in1").get_offset();
        match behave.evaluate_binary(out_size, in0_size, in0_off, in1_off) {
            Ok(v) => Some((v, marked_input)),
            Err(_) => None,
        }
    } else {
        // default: throw LowlevelError("Invalid constant collapse")
        None
    }
}

// =============================================================================
// RuleTransformCpool (ruleaction.cc:3903)
// =============================================================================

/// `RuleTransformCpool` — resolve a `CPOOLREF` against the constant pool (C++
/// `RuleTransformCpool`).
#[derive(Clone, Default)]
pub struct RuleTransformCpool;

impl Rule for RuleTransformCpool {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_CPOOLREF]
    }

    fn clone_rule(&self, _grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        // C++ `clone`: returns null iff the rule's group is not in `grouplist`.
        // The rule's group lives in the engine-owned `RuleState` (not in this
        // zero-sized marker), and the pool only holds rules whose group W8
        // enabled, so the clone is unconditional here.  See module docs.
        Some(Box::new(RuleTransformCpool))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // if (op->isCpoolTransformed()) return 0;  -- Already visited
        if data.obank().get(op).expect("RuleTransformCpool: stale op").is_cpool_transformed() {
            return 0;
        }
        // SEAM(W4): data.opMarkCpoolTransformed(op) marks the visit, then the body
        // looks up `data.getArch()->cpool->getRecord(refs)` and rewrites the op
        // (COPY of the primitive, calculated-bool mark, or tag-append).  The
        // constant-pool record store (`glb->cpool`, W4) and the
        // updateType/markCalculatedBool surfaces are unported.  The
        // already-visited guard is ported; the lookup/rewrite is deferred.
        // Recorded as a loss.  (The C++ unconditionally returns 1 after marking;
        // returning 0 here avoids claiming a change we did not make.)
        0
    }
}

// =============================================================================
// RulePropagateCopy (ruleaction.cc:3943)
// =============================================================================

/// `RulePropagateCopy` — propagate a `COPY`'s input to all readers of its output
/// (C++ `RulePropagateCopy`).
#[derive(Clone, Default)]
pub struct RulePropagateCopy;

impl Rule for RulePropagateCopy {
    // applies to all opcodes (no getOpList override)

    fn clone_rule(&self, _grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        // C++ `clone`: returns null iff the rule's group is not in `grouplist`.
        // The rule's group lives in the engine-owned `RuleState` (not in this
        // zero-sized marker), and the pool only holds rules whose group W8
        // enabled, so the clone is unconditional here.  See module docs.
        Some(Box::new(RulePropagateCopy))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // if (op->isReturnCopy()) return 0;
        if data.obank().get(op).expect("RulePropagateCopy: stale op").is_return_copy() {
            return 0;
        }
        let n = op_num_input(data, op);
        for i in 0..n {
            let vn = op_in(data, op, i).expect("RulePropagateCopy: null input");
            if !vn_is_written(data, vn) {
                continue; // Varnode must be written to
            }
            let copyop = vn_def(data, vn).expect("RulePropagateCopy: written vn no def");
            if op_code(data, copyop) != OpCode::CPUI_COPY {
                continue; // not a propagating instruction
            }
            let invn = op_in(data, copyop, 0).expect("RulePropagateCopy: copyop in0");
            if !vn_is_heritage_known(data, invn) {
                continue; // Don't propagate free's away from their first use
            }
            if invn == vn {
                // throw LowlevelError("Self-defined varnode") -- internal invariant.
                panic!("RulePropagateCopy: Self-defined varnode");
            }
            // if (op->isMarker()) { ... markers special-cased ... }
            let is_marker = data.obank().get(op).expect("RulePropagateCopy: stale op").is_marker();
            if is_marker {
                if vn_is_constant(data, invn) {
                    continue; // Don't propagate constants into markers
                }
                if data.vbank().get(vn).expect("RulePropagateCopy: stale vn").is_addr_force() {
                    continue; // Don't propagate if we are keeping the COPY anyway
                }
                // SEAM(W3-varnode): the addrtied merge-safety guard reads
                // invn->isAddrTied(), op->getOut()->isAddrTied(), and the two
                // addresses.  isAddrTied is available; the address comparison is a
                // straight Address::!=.  We transcribe it faithfully.
                let invn_addrtied =
                    data.vbank().get(invn).expect("RulePropagateCopy: stale invn").is_addr_tied();
                let out = op_out(data, op).expect("RulePropagateCopy: marker out");
                let out_addrtied =
                    data.vbank().get(out).expect("RulePropagateCopy: stale out").is_addr_tied();
                if invn_addrtied && out_addrtied {
                    let out_addr =
                        data.vbank().get(out).expect("RulePropagateCopy: stale out").get_addr().clone();
                    let invn_addr =
                        data.vbank().get(invn).expect("RulePropagateCopy: stale invn").get_addr().clone();
                    if out_addr != invn_addr {
                        continue; // We must not allow merging of different addrtieds
                    }
                }
                // if (op->code()==MULTIEQUAL && copyop->getParent()==op->getParent()->getIn(i))
                //   op->setCopyImmed(i);
                // SEAM(W3-block): the MULTIEQUAL copy-immediate mark needs the
                // op's parent's i-th in-edge block (block adjacency, not yet wired
                // for this read) and setCopyImmed (an addlflags bit not exposed
                // here).  The propagation below is unaffected by skipping the mark
                // (it is a printing hint); recorded as a partial.
            }
            data.op_set_input(op, invn, i).expect("RulePropagateCopy: opSetInput"); // propagate just a single copy
            return 1;
        }
        0
    }
}

// =============================================================================
// Rule2Comp2Mult (ruleaction.cc:3981)
// =============================================================================

/// `Rule2Comp2Mult` — eliminate `INT_2COMP`: `-V => V * -1` (C++
/// `Rule2Comp2Mult`).
#[derive(Clone, Default)]
pub struct Rule2Comp2Mult;

impl Rule for Rule2Comp2Mult {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_INT_2COMP]
    }

    fn clone_rule(&self, _grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        // C++ `clone`: returns null iff the rule's group is not in `grouplist`.
        // The rule's group lives in the engine-owned `RuleState` (not in this
        // zero-sized marker), and the pool only holds rules whose group W8
        // enabled, so the clone is unconditional here.  See module docs.
        Some(Box::new(Rule2Comp2Mult))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        set_opcode(data, op, OpCode::CPUI_INT_MULT);
        let size = vn_size(data, op_in(data, op, 0).expect("Rule2Comp2Mult: null in0"));
        let negone = data.new_constant(size, calc_mask(size));
        data.op_insert_input(op, negone, 1).expect("Rule2Comp2Mult: opInsertInput");
        1
    }
}

// =============================================================================
// RuleCarryElim (ruleaction.cc:3999)
// =============================================================================

/// `RuleCarryElim` — transform `INT_CARRY` with a constant: `carry(V,c) => -c <=
/// V` (C++ `RuleCarryElim`).
#[derive(Clone, Default)]
pub struct RuleCarryElim;

impl Rule for RuleCarryElim {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_INT_CARRY]
    }

    fn clone_rule(&self, _grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        // C++ `clone`: returns null iff the rule's group is not in `grouplist`.
        // The rule's group lives in the engine-owned `RuleState` (not in this
        // zero-sized marker), and the pool only holds rules whose group W8
        // enabled, so the clone is unconditional here.  See module docs.
        Some(Box::new(RuleCarryElim))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        let vn2 = op_in(data, op, 1).expect("RuleCarryElim: null in1");
        if !vn_is_constant(data, vn2) {
            return 0;
        }
        let vn1 = op_in(data, op, 0).expect("RuleCarryElim: null in0");
        if vn_is_free(data, vn1) {
            return 0;
        }
        let mut off = vn_offset(data, vn2);
        if off == 0 {
            // Trivial case
            data.op_remove_input(op, 1); // Go down to 1 input
            let z = data.new_constant(1, 0);
            data.op_set_input(op, z, 0).expect("RuleCarryElim: opSetInput"); // boolean "false" as input to COPY
            set_opcode(data, op, OpCode::CPUI_COPY);
            return 1;
        }
        // off = (-off) & calc_mask(vn2->getSize());  -- twos-complement
        off = off.wrapping_neg() & calc_mask(vn_size(data, vn2));

        set_opcode(data, op, OpCode::CPUI_INT_LESSEQUAL);
        data.op_set_input(op, vn1, 1).expect("RuleCarryElim: opSetInput"); // Move other input to second position
        let newc = data.new_constant(vn_size(data, vn1), off);
        data.op_set_input(op, newc, 0).expect("RuleCarryElim: opSetInput"); // new constant in first position
        1
    }
}

// =============================================================================
// RuleSub2Add (ruleaction.cc:4034)
// =============================================================================

/// `RuleSub2Add` — eliminate `INT_SUB`: `V - W => V + W * -1` (C++
/// `RuleSub2Add`).
#[derive(Clone, Default)]
pub struct RuleSub2Add;

impl Rule for RuleSub2Add {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_INT_SUB]
    }

    fn clone_rule(&self, _grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        // C++ `clone`: returns null iff the rule's group is not in `grouplist`.
        // The rule's group lives in the engine-owned `RuleState` (not in this
        // zero-sized marker), and the pool only holds rules whose group W8
        // enabled, so the clone is unconditional here.  See module docs.
        Some(Box::new(RuleSub2Add))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        let vn = op_in(data, op, 1).expect("RuleSub2Add: null in1"); // Parameter being subtracted
        let opaddr = data.obank().get(op).expect("RuleSub2Add: stale op").get_addr().clone();
        let newop = data.new_op(2, opaddr);
        set_opcode(data, newop, OpCode::CPUI_INT_MULT);
        let newvn = new_unique_out(data, vn_size(data, vn), newop);
        data.op_set_input(op, newvn, 1).expect("RuleSub2Add: opSetInput"); // Replace vn's reference first
        data.op_set_input(newop, vn, 0).expect("RuleSub2Add: opSetInput");
        let negone = data.new_constant(vn_size(data, vn), calc_mask(vn_size(data, vn)));
        data.op_set_input(newop, negone, 1).expect("RuleSub2Add: opSetInput");
        set_opcode(data, op, OpCode::CPUI_INT_ADD);
        data.op_insert_before(newop, op);
        1
    }
}

// =============================================================================
// RuleXorCollapse (ruleaction.cc:4060)
// =============================================================================

/// `RuleXorCollapse` — eliminate `INT_XOR` in comparisons: `(V ^ W) == 0 => V ==
/// W`, `(V ^ c) == d => V == (c^d)` (C++ `RuleXorCollapse`).
#[derive(Clone, Default)]
pub struct RuleXorCollapse;

impl Rule for RuleXorCollapse {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_INT_EQUAL, OpCode::CPUI_INT_NOTEQUAL]
    }

    fn clone_rule(&self, _grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        // C++ `clone`: returns null iff the rule's group is not in `grouplist`.
        // The rule's group lives in the engine-owned `RuleState` (not in this
        // zero-sized marker), and the pool only holds rules whose group W8
        // enabled, so the clone is unconditional here.  See module docs.
        Some(Box::new(RuleXorCollapse))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        let in1 = op_in(data, op, 1).expect("RuleXorCollapse: null in1");
        if !vn_is_constant(data, in1) {
            return 0;
        }
        let in0 = op_in(data, op, 0).expect("RuleXorCollapse: null in0");
        let xorop = match vn_def(data, in0) {
            Some(o) => o,
            None => return 0,
        };
        if op_code(data, xorop) != OpCode::CPUI_INT_XOR {
            return 0;
        }
        if data.lone_descend(in0).is_none() {
            return 0;
        }
        let coeff1 = vn_offset(data, in1);
        let xorvn = op_in(data, xorop, 1).expect("RuleXorCollapse: xorop in1");
        let xor0 = op_in(data, xorop, 0).expect("RuleXorCollapse: xorop in0");
        if vn_is_free(data, xor0) {
            return 0; // This will be propagated
        }
        if !vn_is_constant(data, xorvn) {
            if coeff1 != 0 {
                return 0;
            }
            if vn_is_free(data, xorvn) {
                return 0;
            }
            data.op_set_input(op, xorvn, 1).expect("RuleXorCollapse: opSetInput"); // Move term to other side
            data.op_set_input(op, xor0, 0).expect("RuleXorCollapse: opSetInput");
            return 1;
        }
        let coeff2 = vn_offset(data, xorvn);
        if coeff2 == 0 {
            return 0;
        }
        let constvn = data.new_constant(vn_size(data, in1), coeff1 ^ coeff2);
        // SEAM(W6): constvn->copySymbolIfValid(xorvn) -- the symbol-entry copy is
        // the W6 TypeFactory/symbol surface; the constant value (coeff1^coeff2) is
        // ported, the symbol annotation is skipped.  Recorded as a loss.
        data.op_set_input(op, constvn, 1).expect("RuleXorCollapse: opSetInput");
        data.op_set_input(op, xor0, 0).expect("RuleXorCollapse: opSetInput");
        1
    }
}

// =============================================================================
// RuleAddMultCollapse (ruleaction.cc:4101)
// =============================================================================

/// `RuleAddMultCollapse` — collapse nested constants in an additive/
/// multiplicative expression `((V + c) + d) => V + (c+d)` (C++
/// `RuleAddMultCollapse`).
#[derive(Clone, Default)]
pub struct RuleAddMultCollapse;

impl Rule for RuleAddMultCollapse {
    fn get_op_list(&self) -> Vec<OpCode> {
        // { CPUI_INT_ADD, CPUI_INT_MULT }
        vec![OpCode::CPUI_INT_ADD, OpCode::CPUI_INT_MULT]
    }

    fn clone_rule(&self, _grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        // C++ `clone`: returns null iff the rule's group is not in `grouplist`.
        // The rule's group lives in the engine-owned `RuleState` (not in this
        // zero-sized marker), and the pool only holds rules whose group W8
        // enabled, so the clone is unconditional here.  See module docs.
        Some(Box::new(RuleAddMultCollapse))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        let opc = op_code(data, op);
        // Constant is in c[0], other is in sub
        let c0 = op_in(data, op, 1).expect("RuleAddMultCollapse: null in1");
        if !vn_is_constant(data, c0) {
            return 0; // Neither input is a constant
        }
        let sub = op_in(data, op, 0).expect("RuleAddMultCollapse: null in0");
        // Find other constant one level down
        if !vn_is_written(data, sub) {
            return 0;
        }
        let subop = vn_def(data, sub).expect("RuleAddMultCollapse: sub def");
        if op_code(data, subop) != opc {
            return 0; // Must be same exact operation
        }
        let mut c1 = op_in(data, subop, 1).expect("RuleAddMultCollapse: subop in1");
        if !vn_is_constant(data, c1) {
            // a = ((stackbase + c[1]) + othervn) + c[0] => (stackbase + c0 + c1) + othervn
            if opc != OpCode::CPUI_INT_ADD {
                return 0;
            }
            for i in 0..2 {
                let othervn = op_in(data, subop, i).expect("RuleAddMultCollapse: subop in");
                if vn_is_constant(data, othervn) {
                    continue;
                }
                if vn_is_free(data, othervn) {
                    continue;
                }
                let sub2 = op_in(data, subop, 1 - i).expect("RuleAddMultCollapse: subop in 1-i");
                if !vn_is_written(data, sub2) {
                    continue;
                }
                let baseop = vn_def(data, sub2).expect("RuleAddMultCollapse: sub2 def");
                if op_code(data, baseop) != OpCode::CPUI_INT_ADD {
                    continue;
                }
                c1 = op_in(data, baseop, 1).expect("RuleAddMultCollapse: baseop in1");
                if !vn_is_constant(data, c1) {
                    continue;
                }
                let basevn = op_in(data, baseop, 0).expect("RuleAddMultCollapse: baseop in0");
                // Only apply if we are adding to a base pointer (input spacebase)
                if !data.vbank().get(basevn).expect("RuleAddMultCollapse: stale basevn").is_spacebase() {
                    continue;
                }
                if !data.vbank().get(basevn).expect("RuleAddMultCollapse: stale basevn").is_input() {
                    continue; // because this adds a new add operation
                }

                // val = op->getOpcode()->evaluateBinary(c0sz,c0sz,c0off,c1off)
                let c0sz = vn_size(data, c0);
                let val = eval_binary(opc, c0sz, vn_offset(data, c0), vn_offset(data, c1));
                let newvn = data.new_constant(c0sz, val);
                // SEAM(W6): copySymbolIfValid(c0 or c1) -- symbol-entry copy skipped.
                let opaddr = data.obank().get(op).expect("RuleAddMultCollapse: stale op").get_addr().clone();
                let newop = data.new_op(2, opaddr);
                set_opcode(data, newop, OpCode::CPUI_INT_ADD);
                let newout = new_unique_out(data, c0sz, newop);
                data.op_set_input(newop, basevn, 0).expect("RuleAddMultCollapse: opSetInput");
                data.op_set_input(newop, newvn, 1).expect("RuleAddMultCollapse: opSetInput");
                data.op_insert_before(newop, op);
                data.op_set_input(op, newout, 0).expect("RuleAddMultCollapse: opSetInput");
                data.op_set_input(op, othervn, 1).expect("RuleAddMultCollapse: opSetInput");
                return 1;
            }
            return 0;
        }
        let sub2 = op_in(data, subop, 0).expect("RuleAddMultCollapse: subop in0");
        if vn_is_free(data, sub2) {
            return 0;
        }

        let c0sz = vn_size(data, c0);
        let val = eval_binary(opc, c0sz, vn_offset(data, c0), vn_offset(data, c1));
        let newvn = data.new_constant(c0sz, val);
        // SEAM(W6): copySymbolIfValid(c0 or c1) -- symbol-entry copy skipped.
        data.op_set_input(op, newvn, 1).expect("RuleAddMultCollapse: opSetInput"); // Replace c0 with c0(+|*)c1
        data.op_set_input(op, sub2, 0).expect("RuleAddMultCollapse: opSetInput"); // Replace sub with sub2
        1
    }
}

/// `op->getOpcode()->evaluateBinary(sizeout,sizein,in1,in2)` for the two opcodes
/// RuleAddMultCollapse folds constants for (`INT_ADD`, `INT_MULT`).
///
/// SEAM(W6): the real path dispatches through the op's `OpBehavior`.  For these
/// two pure-arithmetic opcodes the behavior is fully determined by the opcode, so
/// the kuna_num `OpBehavior` for that opcode is invoked directly (faithful, no
/// invented behavior — `OpBehaviorIntAdd`/`OpBehaviorIntMult` are the same
/// behaviors `glb->inst[opc]` would carry).
fn eval_binary(opc: OpCode, size: int4, in1: uintb, in2: uintb) -> uintb {
    use kuna_num::opbehavior::{OpBehavior, OpBehaviorIntAdd, OpBehaviorIntMult};
    match opc {
        OpCode::CPUI_INT_ADD => OpBehaviorIntAdd
            .evaluate_binary(size, size, in1, in2)
            .expect("eval_binary: INT_ADD"),
        OpCode::CPUI_INT_MULT => OpBehaviorIntMult
            .evaluate_binary(size, size, in1, in2)
            .expect("eval_binary: INT_MULT"),
        other => panic!("RuleAddMultCollapse::eval_binary: unexpected opcode {other:?}"),
    }
}

// =============================================================================
// Registration (C++ definition order — W8 assembles universalAction from this)
// =============================================================================

/// The [`RuleSpec`] rows for this batch, in C++ definition order.
///
/// `group` is left empty here: the W8 assembly assigns the real stage group when
/// it places each rule into a pool (the C++ constructs every rule as
/// `RuleXxx(group)` with the group supplied by `universalAction`).  The `ctor`
/// builds a fresh zero-sized rule marker.
pub fn specs() -> Vec<RuleSpec> {
    vec![
        RuleSpec { group: "", ctor: || Box::new(RuleBooleanDedup) },
        RuleSpec { group: "", ctor: || Box::new(RuleBooleanNegate) },
        RuleSpec { group: "", ctor: || Box::new(RuleBoolZext) },
        RuleSpec { group: "", ctor: || Box::new(RuleLogic2Bool) },
        RuleSpec { group: "", ctor: || Box::new(RuleIndirectCollapse) },
        RuleSpec { group: "", ctor: || Box::new(RuleMultiCollapse) },
        RuleSpec { group: "", ctor: || Box::new(RuleSborrow) },
        RuleSpec { group: "", ctor: || Box::new(RuleScarry) },
        RuleSpec { group: "", ctor: || Box::new(RuleTrivialShift) },
        RuleSpec { group: "", ctor: || Box::new(RuleSignShift) },
        RuleSpec { group: "", ctor: || Box::new(RuleTestSign) },
        RuleSpec { group: "", ctor: || Box::new(RuleIdentityEl) },
        RuleSpec { group: "", ctor: || Box::new(RuleShift2Mult) },
        RuleSpec { group: "", ctor: || Box::new(RuleShiftPiece) },
        RuleSpec { group: "", ctor: || Box::new(RuleCollapseConstants) },
        RuleSpec { group: "", ctor: || Box::new(RuleTransformCpool) },
        RuleSpec { group: "", ctor: || Box::new(RulePropagateCopy) },
        RuleSpec { group: "", ctor: || Box::new(Rule2Comp2Mult) },
        RuleSpec { group: "", ctor: || Box::new(RuleCarryElim) },
        RuleSpec { group: "", ctor: || Box::new(RuleSub2Add) },
        RuleSpec { group: "", ctor: || Box::new(RuleXorCollapse) },
        RuleSpec { group: "", ctor: || Box::new(RuleAddMultCollapse) },
    ]
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
    use kuna_base::types::int4;
    use kuna_num::opcodes::OpCode;

    use crate::seams::Architecture;

    // --- harness ----------------------------------------------------------

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

    /// A function with one basic block (so ops can be inserted and rules that
    /// call `opInsertBefore`/`opInsertAfter` find a parent).
    struct Fd {
        fd: Funcdata,
        block: crate::seams::BlockId,
    }

    impl std::ops::Deref for Fd {
        type Target = Funcdata;
        fn deref(&self) -> &Funcdata {
            &self.fd
        }
    }
    impl std::ops::DerefMut for Fd {
        fn deref_mut(&mut self) -> &mut Funcdata {
            &mut self.fd
        }
    }

    fn build_fd() -> Fd {
        let manage = build_manager();
        let glb = Rc::new(Architecture::new(manage));
        let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
        let addr = Address::new(ram, 0x1000);
        let mut fd = Funcdata::new("func", "func", glb, addr, 0x10000000, 0x40).unwrap();
        let root = fd.bblocks_root_pub();
        let block = fd.bblocks_mut().new_block_basic(root);
        Fd { fd, block }
    }

    fn ram(fd: &Funcdata) -> Rc<AddrSpace> {
        Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap())
    }

    fn pc(fd: &Funcdata) -> Address {
        Address::new(ram(fd), 0x1000)
    }

    /// Make a free input-register varnode at `(off,size)` in ram and promote it
    /// to a function input (so it is heritage-known and not free).
    fn make_input(fd: &mut Fd, off: u64, size: int4) -> VarnodeId {
        let r = ram(fd);
        let vn = fd.new_varnode(size, &Address::new(r, off), None);
        let mut no_replace = |_: &mut crate::varnode::VarnodeBank,
                              _: VarnodeId,
                              _: VarnodeId|
         -> KunaResult<()> { Ok(()) };
        fd.vbank_mut().set_input(vn, &mut no_replace).unwrap()
    }

    /// Cached `opflags` for the opcodes the test harness builds as *inputs*.
    /// This is the full table the real `glb->inst[opc]` would carry for these
    /// opcodes (transcribed from `typeop.cc`), covering the input shapes the rule
    /// tests construct; the production [`opflags_for`] is intentionally narrow
    /// (rule-produced opcodes only), so the harness needs its own.
    fn test_opflags(opc: OpCode) -> u32 {
        use pcodeop_flags::*;
        match opc {
            OpCode::CPUI_COPY => unary | nocollapse,
            OpCode::CPUI_BOOL_NEGATE => unary | booloutput,
            OpCode::CPUI_BOOL_AND | OpCode::CPUI_BOOL_OR | OpCode::CPUI_BOOL_XOR => {
                binary | commutative | booloutput
            }
            OpCode::CPUI_INT_EQUAL | OpCode::CPUI_INT_NOTEQUAL => {
                binary | booloutput | commutative
            }
            OpCode::CPUI_INT_SLESS | OpCode::CPUI_INT_SLESSEQUAL | OpCode::CPUI_INT_LESSEQUAL => {
                binary | booloutput
            }
            OpCode::CPUI_INT_ADD | OpCode::CPUI_INT_MULT => binary | commutative,
            OpCode::CPUI_INT_AND | OpCode::CPUI_INT_OR | OpCode::CPUI_INT_XOR => {
                binary | commutative
            }
            OpCode::CPUI_INT_SUB
            | OpCode::CPUI_INT_LEFT
            | OpCode::CPUI_INT_RIGHT
            | OpCode::CPUI_INT_SRIGHT
            | OpCode::CPUI_INT_SBORROW
            | OpCode::CPUI_INT_SCARRY
            | OpCode::CPUI_INT_CARRY
            | OpCode::CPUI_PIECE
            | OpCode::CPUI_SUBPIECE => binary,
            OpCode::CPUI_INT_2COMP | OpCode::CPUI_INT_ZEXT | OpCode::CPUI_INT_SEXT => unary,
            OpCode::CPUI_MULTIEQUAL => marker,
            other => panic!("test_opflags: add an entry for {other:?}"),
        }
    }

    /// Create an op of the given opcode at the function's pc with `ninput`
    /// inputs (no output assigned), inserted at the end of the function's single
    /// basic block (so it has a parent for `opInsertBefore`/`After`).
    fn mk_op(fd: &mut Fd, ninput: int4, opc: OpCode) -> OpId {
        let addr = pc(fd);
        let op = fd.new_op(ninput, addr);
        fd.op_set_opcode(op, TypeOp::new(opc, test_opflags(opc), format!("{opc:?}")));
        let bl = fd.block;
        fd.bb_insert_op(op, bl, None);
        op
    }

    /// Wire `vn` into `op`'s slot `slot`.
    fn wire(fd: &mut Fd, op: OpId, vn: VarnodeId, slot: int4) {
        fd.op_set_input(op, vn, slot).unwrap();
    }

    /// Build an op with a fresh unique output of `outsize` and the given inputs.
    /// Returns the op and its output varnode.
    fn def_op(fd: &mut Fd, opc: OpCode, ins: &[VarnodeId], outsize: int4) -> (OpId, VarnodeId) {
        let op = mk_op(fd, ins.len() as int4, opc);
        for (i, &vn) in ins.iter().enumerate() {
            wire(fd, op, vn, i as int4);
        }
        let out = new_unique_out(&mut fd.fd, outsize, op);
        (op, out)
    }

    // --- RuleBooleanNegate ------------------------------------------------

    #[test]
    fn boolean_negate_eq_false_becomes_negate() {
        // (bool == 0) with INT_EQUAL, val==0 => negate = !(false) = true => BOOL_NEGATE
        let mut fd = build_fd();
        // A boolean-valued subexpression: make it the output of a BOOL_AND (which
        // is a boolean value by opcode).
        let a = make_input(&mut fd, 0x100, 1);
        let b = make_input(&mut fd, 0x101, 1);
        let (_andop, subbool) = def_op(&mut fd, OpCode::CPUI_BOOL_AND, &[a, b], 1);
        let zero = fd.new_constant(1, 0);
        let op = mk_op(&mut fd, 2, OpCode::CPUI_INT_EQUAL);
        wire(&mut fd, op, subbool, 0);
        wire(&mut fd, op, zero, 1);

        assert_eq!(RuleBooleanNegate.apply_op(op, &mut fd), 1);
        assert_eq!(op_code(&fd, op), OpCode::CPUI_BOOL_NEGATE);
        assert_eq!(op_num_input(&fd, op), 1);
        assert_eq!(op_in(&fd, op, 0), Some(subbool));
    }

    #[test]
    fn boolean_negate_eq_true_becomes_copy() {
        // (bool == 1) with INT_EQUAL, val==1 => negate stays false => COPY
        let mut fd = build_fd();
        let a = make_input(&mut fd, 0x100, 1);
        let b = make_input(&mut fd, 0x101, 1);
        let (_andop, subbool) = def_op(&mut fd, OpCode::CPUI_BOOL_OR, &[a, b], 1);
        let one = fd.new_constant(1, 1);
        let op = mk_op(&mut fd, 2, OpCode::CPUI_INT_EQUAL);
        wire(&mut fd, op, subbool, 0);
        wire(&mut fd, op, one, 1);

        assert_eq!(RuleBooleanNegate.apply_op(op, &mut fd), 1);
        assert_eq!(op_code(&fd, op), OpCode::CPUI_COPY);
        assert_eq!(op_num_input(&fd, op), 1);
    }

    #[test]
    fn boolean_negate_notequal_true_becomes_negate() {
        // (bool != 1) with INT_NOTEQUAL, val==1 => negate = true => BOOL_NEGATE
        let mut fd = build_fd();
        let a = make_input(&mut fd, 0x100, 1);
        let b = make_input(&mut fd, 0x101, 1);
        let (_andop, subbool) = def_op(&mut fd, OpCode::CPUI_BOOL_AND, &[a, b], 1);
        let one = fd.new_constant(1, 1);
        let op = mk_op(&mut fd, 2, OpCode::CPUI_INT_NOTEQUAL);
        wire(&mut fd, op, subbool, 0);
        wire(&mut fd, op, one, 1);

        assert_eq!(RuleBooleanNegate.apply_op(op, &mut fd), 1);
        assert_eq!(op_code(&fd, op), OpCode::CPUI_BOOL_NEGATE);
    }

    #[test]
    fn boolean_negate_guards() {
        let mut fd = build_fd();
        // non-constant second operand -> 0
        let a = make_input(&mut fd, 0x100, 1);
        let b = make_input(&mut fd, 0x101, 1);
        let (_op0, sb) = def_op(&mut fd, OpCode::CPUI_BOOL_AND, &[a, b], 1);
        let nonconst = make_input(&mut fd, 0x200, 1);
        let op = mk_op(&mut fd, 2, OpCode::CPUI_INT_EQUAL);
        wire(&mut fd, op, sb, 0);
        wire(&mut fd, op, nonconst, 1);
        assert_eq!(RuleBooleanNegate.apply_op(op, &mut fd), 0);

        // constant != {0,1} -> 0
        let op2 = mk_op(&mut fd, 2, OpCode::CPUI_INT_EQUAL);
        let c2 = fd.new_constant(1, 2);
        wire(&mut fd, op2, sb, 0);
        wire(&mut fd, op2, c2, 1);
        assert_eq!(RuleBooleanNegate.apply_op(op2, &mut fd), 0);

        // subbool is NOT a boolean value (a plain 4-byte input) -> 0
        let notbool = make_input(&mut fd, 0x300, 4);
        let op3 = mk_op(&mut fd, 2, OpCode::CPUI_INT_EQUAL);
        let z = fd.new_constant(4, 0);
        wire(&mut fd, op3, notbool, 0);
        wire(&mut fd, op3, z, 1);
        assert_eq!(RuleBooleanNegate.apply_op(op3, &mut fd), 0);
    }

    // --- RuleLogic2Bool ---------------------------------------------------

    #[test]
    fn logic2bool_and_both_bool() {
        let mut fd = build_fd();
        // Both inputs boolean: outputs of BOOL_NEGATE (a boolean by opcode).
        let x = make_input(&mut fd, 0x100, 1);
        let y = make_input(&mut fd, 0x101, 1);
        let (_nx, bx) = def_op(&mut fd, OpCode::CPUI_BOOL_NEGATE, &[x], 1);
        let (_ny, by) = def_op(&mut fd, OpCode::CPUI_BOOL_NEGATE, &[y], 1);
        let op = mk_op(&mut fd, 2, OpCode::CPUI_INT_AND);
        wire(&mut fd, op, bx, 0);
        wire(&mut fd, op, by, 1);
        assert_eq!(RuleLogic2Bool.apply_op(op, &mut fd), 1);
        assert_eq!(op_code(&fd, op), OpCode::CPUI_BOOL_AND);
    }

    #[test]
    fn logic2bool_const_0_or_1_ok_but_2_rejected() {
        let mut fd = build_fd();
        let x = make_input(&mut fd, 0x100, 1);
        let (_nx, bx) = def_op(&mut fd, OpCode::CPUI_BOOL_NEGATE, &[x], 1);
        // second input constant 1 -> OK, INT_OR => BOOL_OR
        let one = fd.new_constant(1, 1);
        let op = mk_op(&mut fd, 2, OpCode::CPUI_INT_OR);
        wire(&mut fd, op, bx, 0);
        wire(&mut fd, op, one, 1);
        assert_eq!(RuleLogic2Bool.apply_op(op, &mut fd), 1);
        assert_eq!(op_code(&fd, op), OpCode::CPUI_BOOL_OR);

        // second input constant 2 -> rejected
        let two = fd.new_constant(1, 2);
        let op2 = mk_op(&mut fd, 2, OpCode::CPUI_INT_XOR);
        wire(&mut fd, op2, bx, 0);
        wire(&mut fd, op2, two, 1);
        assert_eq!(RuleLogic2Bool.apply_op(op2, &mut fd), 0);
        assert_eq!(op_code(&fd, op2), OpCode::CPUI_INT_XOR);
    }

    #[test]
    fn logic2bool_first_not_bool_rejected() {
        let mut fd = build_fd();
        let nb = make_input(&mut fd, 0x100, 4); // not boolean
        let other = make_input(&mut fd, 0x104, 4);
        let op = mk_op(&mut fd, 2, OpCode::CPUI_INT_AND);
        wire(&mut fd, op, nb, 0);
        wire(&mut fd, op, other, 1);
        assert_eq!(RuleLogic2Bool.apply_op(op, &mut fd), 0);
    }

    // --- RuleTrivialShift -------------------------------------------------

    #[test]
    fn trivial_shift_by_zero_is_copy() {
        let mut fd = build_fd();
        let v = make_input(&mut fd, 0x100, 4);
        let z = fd.new_constant(4, 0);
        let op = mk_op(&mut fd, 2, OpCode::CPUI_INT_LEFT);
        wire(&mut fd, op, v, 0);
        wire(&mut fd, op, z, 1);
        assert_eq!(RuleTrivialShift.apply_op(op, &mut fd), 1);
        assert_eq!(op_code(&fd, op), OpCode::CPUI_COPY);
        assert_eq!(op_num_input(&fd, op), 1);
        assert_eq!(op_in(&fd, op, 0), Some(v));
    }

    #[test]
    fn trivial_shift_oversized_left_yields_zero_copy() {
        let mut fd = build_fd();
        let v = make_input(&mut fd, 0x100, 4); // 32 bits
        let big = fd.new_constant(4, 32); // >= 8*4
        let op = mk_op(&mut fd, 2, OpCode::CPUI_INT_LEFT);
        wire(&mut fd, op, v, 0);
        wire(&mut fd, op, big, 1);
        assert_eq!(RuleTrivialShift.apply_op(op, &mut fd), 1);
        assert_eq!(op_code(&fd, op), OpCode::CPUI_COPY);
        assert_eq!(op_num_input(&fd, op), 1);
        // input replaced with a zero constant
        let in0 = op_in(&fd, op, 0).unwrap();
        assert!(vn_is_constant(&fd, in0));
        assert_eq!(vn_offset(&fd, in0), 0);
    }

    #[test]
    fn trivial_shift_guards() {
        let mut fd = build_fd();
        // non-constant shift amount -> 0
        let v = make_input(&mut fd, 0x100, 4);
        let amt = make_input(&mut fd, 0x104, 4);
        let op = mk_op(&mut fd, 2, OpCode::CPUI_INT_LEFT);
        wire(&mut fd, op, v, 0);
        wire(&mut fd, op, amt, 1);
        assert_eq!(RuleTrivialShift.apply_op(op, &mut fd), 0);

        // non-trivial (val < 32) -> 0
        let v2 = make_input(&mut fd, 0x200, 4);
        let small = fd.new_constant(4, 3);
        let op2 = mk_op(&mut fd, 2, OpCode::CPUI_INT_LEFT);
        wire(&mut fd, op2, v2, 0);
        wire(&mut fd, op2, small, 1);
        assert_eq!(RuleTrivialShift.apply_op(op2, &mut fd), 0);

        // oversized SRIGHT (cant predict signbit) -> 0
        let v3 = make_input(&mut fd, 0x300, 4);
        let big = fd.new_constant(4, 40);
        let op3 = mk_op(&mut fd, 2, OpCode::CPUI_INT_SRIGHT);
        wire(&mut fd, op3, v3, 0);
        wire(&mut fd, op3, big, 1);
        assert_eq!(RuleTrivialShift.apply_op(op3, &mut fd), 0);
    }

    // --- RuleIdentityEl ---------------------------------------------------

    #[test]
    fn identity_add_zero_is_copy() {
        let mut fd = build_fd();
        let v = make_input(&mut fd, 0x100, 4);
        let z = fd.new_constant(4, 0);
        let op = mk_op(&mut fd, 2, OpCode::CPUI_INT_ADD);
        wire(&mut fd, op, v, 0);
        wire(&mut fd, op, z, 1);
        assert_eq!(RuleIdentityEl.apply_op(op, &mut fd), 1);
        assert_eq!(op_code(&fd, op), OpCode::CPUI_COPY);
        assert_eq!(op_num_input(&fd, op), 1);
        assert_eq!(op_in(&fd, op, 0), Some(v));
    }

    #[test]
    fn identity_mult_one_is_copy_and_mult_zero_drops_slot0() {
        let mut fd = build_fd();
        // V * 1 => COPY(V) (removes slot 1)
        let v = make_input(&mut fd, 0x100, 4);
        let one = fd.new_constant(4, 1);
        let op = mk_op(&mut fd, 2, OpCode::CPUI_INT_MULT);
        wire(&mut fd, op, v, 0);
        wire(&mut fd, op, one, 1);
        assert_eq!(RuleIdentityEl.apply_op(op, &mut fd), 1);
        assert_eq!(op_code(&fd, op), OpCode::CPUI_COPY);
        assert_eq!(op_in(&fd, op, 0), Some(v));

        // V * 0 => COPY of the (zero) constant: removes slot 0, leaving the const.
        let v2 = make_input(&mut fd, 0x200, 4);
        let zero = fd.new_constant(4, 0);
        let op2 = mk_op(&mut fd, 2, OpCode::CPUI_INT_MULT);
        wire(&mut fd, op2, v2, 0);
        wire(&mut fd, op2, zero, 1);
        assert_eq!(RuleIdentityEl.apply_op(op2, &mut fd), 1);
        assert_eq!(op_code(&fd, op2), OpCode::CPUI_COPY);
        assert_eq!(op_num_input(&fd, op2), 1);
        let rem = op_in(&fd, op2, 0).unwrap();
        assert_eq!(rem, zero);
    }

    #[test]
    fn identity_mult_other_const_rejected() {
        let mut fd = build_fd();
        let v = make_input(&mut fd, 0x100, 4);
        let c = fd.new_constant(4, 7);
        let op = mk_op(&mut fd, 2, OpCode::CPUI_INT_MULT);
        wire(&mut fd, op, v, 0);
        wire(&mut fd, op, c, 1);
        assert_eq!(RuleIdentityEl.apply_op(op, &mut fd), 0);
        // non-constant second operand -> 0
        let nc = make_input(&mut fd, 0x200, 4);
        let op2 = mk_op(&mut fd, 2, OpCode::CPUI_INT_ADD);
        wire(&mut fd, op2, v, 0);
        wire(&mut fd, op2, nc, 1);
        assert_eq!(RuleIdentityEl.apply_op(op2, &mut fd), 0);
    }

    // --- RuleCarryElim ----------------------------------------------------

    #[test]
    fn carry_elim_zero_constant_is_false_copy() {
        let mut fd = build_fd();
        let v = make_input(&mut fd, 0x100, 4);
        let z = fd.new_constant(4, 0);
        let op = mk_op(&mut fd, 2, OpCode::CPUI_INT_CARRY);
        wire(&mut fd, op, v, 0);
        wire(&mut fd, op, z, 1);
        assert_eq!(RuleCarryElim.apply_op(op, &mut fd), 1);
        assert_eq!(op_code(&fd, op), OpCode::CPUI_COPY);
        assert_eq!(op_num_input(&fd, op), 1);
        let in0 = op_in(&fd, op, 0).unwrap();
        assert!(vn_is_constant(&fd, in0));
        assert_eq!(vn_offset(&fd, in0), 0);
    }

    #[test]
    fn carry_elim_nonzero_const_becomes_lessequal() {
        // carry(V, c) with c != 0 => (-c) <= V : INT_LESSEQUAL, slot0 = const(-c),
        // slot1 = V.
        let mut fd = build_fd();
        let v = make_input(&mut fd, 0x100, 4);
        let c = fd.new_constant(4, 5);
        let op = mk_op(&mut fd, 2, OpCode::CPUI_INT_CARRY);
        wire(&mut fd, op, v, 0);
        wire(&mut fd, op, c, 1);
        assert_eq!(RuleCarryElim.apply_op(op, &mut fd), 1);
        assert_eq!(op_code(&fd, op), OpCode::CPUI_INT_LESSEQUAL);
        assert_eq!(op_in(&fd, op, 1), Some(v));
        let in0 = op_in(&fd, op, 0).unwrap();
        assert!(vn_is_constant(&fd, in0));
        // (-5) & mask(4 bytes) == 0xFFFFFFFB
        assert_eq!(vn_offset(&fd, in0), (5u64.wrapping_neg()) & calc_mask(4));
    }

    #[test]
    fn carry_elim_guards() {
        let mut fd = build_fd();
        // non-constant second operand -> 0
        let v = make_input(&mut fd, 0x100, 4);
        let nc = make_input(&mut fd, 0x104, 4);
        let op = mk_op(&mut fd, 2, OpCode::CPUI_INT_CARRY);
        wire(&mut fd, op, v, 0);
        wire(&mut fd, op, nc, 1);
        assert_eq!(RuleCarryElim.apply_op(op, &mut fd), 0);
    }

    // --- Rule2Comp2Mult ---------------------------------------------------

    #[test]
    fn two_comp_to_mult() {
        let mut fd = build_fd();
        let v = make_input(&mut fd, 0x100, 4);
        let op = mk_op(&mut fd, 1, OpCode::CPUI_INT_2COMP);
        wire(&mut fd, op, v, 0);
        assert_eq!(Rule2Comp2Mult.apply_op(op, &mut fd), 1);
        assert_eq!(op_code(&fd, op), OpCode::CPUI_INT_MULT);
        assert_eq!(op_num_input(&fd, op), 2);
        assert_eq!(op_in(&fd, op, 0), Some(v));
        let neg = op_in(&fd, op, 1).unwrap();
        assert!(vn_is_constant(&fd, neg));
        assert_eq!(vn_offset(&fd, neg), calc_mask(4)); // -1 == all ones
    }

    // --- RuleSub2Add ------------------------------------------------------

    #[test]
    fn sub_to_add_inserts_mult_neg_one() {
        let mut fd = build_fd();
        let a = make_input(&mut fd, 0x100, 4);
        let b = make_input(&mut fd, 0x104, 4);
        let op = mk_op(&mut fd, 2, OpCode::CPUI_INT_SUB);
        wire(&mut fd, op, a, 0);
        wire(&mut fd, op, b, 1);
        assert_eq!(RuleSub2Add.apply_op(op, &mut fd), 1);
        // op is now INT_ADD; slot1 is the output of a new INT_MULT(b, -1).
        assert_eq!(op_code(&fd, op), OpCode::CPUI_INT_ADD);
        assert_eq!(op_in(&fd, op, 0), Some(a));
        let newvn = op_in(&fd, op, 1).unwrap();
        let multop = vn_def(&fd, newvn).unwrap();
        assert_eq!(op_code(&fd, multop), OpCode::CPUI_INT_MULT);
        assert_eq!(op_in(&fd, multop, 0), Some(b));
        let negone = op_in(&fd, multop, 1).unwrap();
        assert!(vn_is_constant(&fd, negone));
        assert_eq!(vn_offset(&fd, negone), calc_mask(4));
    }

    // --- RuleXorCollapse --------------------------------------------------

    #[test]
    fn xor_collapse_const_const_folds() {
        // (V ^ c) == d  =>  V == (c^d)
        let mut fd = build_fd();
        let v = make_input(&mut fd, 0x100, 4);
        let c = fd.new_constant(4, 0x0f);
        let (_xorop, xorout) = def_op(&mut fd, OpCode::CPUI_INT_XOR, &[v, c], 4);
        let d = fd.new_constant(4, 0x33);
        let op = mk_op(&mut fd, 2, OpCode::CPUI_INT_EQUAL);
        wire(&mut fd, op, xorout, 0);
        wire(&mut fd, op, d, 1);
        assert_eq!(RuleXorCollapse.apply_op(op, &mut fd), 1);
        // slot0 becomes V, slot1 becomes constant 0x0f ^ 0x33.
        assert_eq!(op_in(&fd, op, 0), Some(v));
        let folded = op_in(&fd, op, 1).unwrap();
        assert!(vn_is_constant(&fd, folded));
        assert_eq!(vn_offset(&fd, folded), 0x0f ^ 0x33);
    }

    #[test]
    fn xor_collapse_var_with_zero_moves_term() {
        // (V ^ W) == 0  =>  V == W
        let mut fd = build_fd();
        let v = make_input(&mut fd, 0x100, 4);
        let w = make_input(&mut fd, 0x104, 4);
        let (_xorop, xorout) = def_op(&mut fd, OpCode::CPUI_INT_XOR, &[v, w], 4);
        let zero = fd.new_constant(4, 0);
        let op = mk_op(&mut fd, 2, OpCode::CPUI_INT_EQUAL);
        wire(&mut fd, op, xorout, 0);
        wire(&mut fd, op, zero, 1);
        assert_eq!(RuleXorCollapse.apply_op(op, &mut fd), 1);
        // slot1 := W (the xor's second term), slot0 := V (the xor's first term)
        assert_eq!(op_in(&fd, op, 0), Some(v));
        assert_eq!(op_in(&fd, op, 1), Some(w));
    }

    #[test]
    fn xor_collapse_guards() {
        let mut fd = build_fd();
        // second operand not constant -> 0
        let v = make_input(&mut fd, 0x100, 4);
        let c = fd.new_constant(4, 0x0f);
        let (_xorop, xorout) = def_op(&mut fd, OpCode::CPUI_INT_XOR, &[v, c], 4);
        let dvar = make_input(&mut fd, 0x200, 4);
        let op = mk_op(&mut fd, 2, OpCode::CPUI_INT_EQUAL);
        wire(&mut fd, op, xorout, 0);
        wire(&mut fd, op, dvar, 1);
        assert_eq!(RuleXorCollapse.apply_op(op, &mut fd), 0);

        // slot0 not defined by an XOR -> 0
        let other = make_input(&mut fd, 0x300, 4);
        let d = fd.new_constant(4, 1);
        let op2 = mk_op(&mut fd, 2, OpCode::CPUI_INT_EQUAL);
        wire(&mut fd, op2, other, 0);
        wire(&mut fd, op2, d, 1);
        assert_eq!(RuleXorCollapse.apply_op(op2, &mut fd), 0);
    }

    // --- RuleAddMultCollapse ---------------------------------------------

    #[test]
    fn add_mult_collapse_nested_add_constants() {
        // ((V + c1) + c0) => V + (c0+c1)
        let mut fd = build_fd();
        let v = make_input(&mut fd, 0x100, 4);
        let c1 = fd.new_constant(4, 10);
        let (_inner, innerout) = def_op(&mut fd, OpCode::CPUI_INT_ADD, &[v, c1], 4);
        let c0 = fd.new_constant(4, 20);
        let op = mk_op(&mut fd, 2, OpCode::CPUI_INT_ADD);
        wire(&mut fd, op, innerout, 0);
        wire(&mut fd, op, c0, 1);
        assert_eq!(RuleAddMultCollapse.apply_op(op, &mut fd), 1);
        // slot0 := V (sub2), slot1 := const(30)
        assert_eq!(op_in(&fd, op, 0), Some(v));
        let folded = op_in(&fd, op, 1).unwrap();
        assert!(vn_is_constant(&fd, folded));
        assert_eq!(vn_offset(&fd, folded), 30);
    }

    #[test]
    fn add_mult_collapse_nested_mult_constants() {
        // ((V * c1) * c0) => V * (c0*c1)
        let mut fd = build_fd();
        let v = make_input(&mut fd, 0x100, 4);
        let c1 = fd.new_constant(4, 3);
        let (_inner, innerout) = def_op(&mut fd, OpCode::CPUI_INT_MULT, &[v, c1], 4);
        let c0 = fd.new_constant(4, 7);
        let op = mk_op(&mut fd, 2, OpCode::CPUI_INT_MULT);
        wire(&mut fd, op, innerout, 0);
        wire(&mut fd, op, c0, 1);
        assert_eq!(RuleAddMultCollapse.apply_op(op, &mut fd), 1);
        assert_eq!(op_in(&fd, op, 0), Some(v));
        let folded = op_in(&fd, op, 1).unwrap();
        assert_eq!(vn_offset(&fd, folded), 21);
    }

    #[test]
    fn add_mult_collapse_guards() {
        let mut fd = build_fd();
        // second operand not constant -> 0
        let v = make_input(&mut fd, 0x100, 4);
        let nc = make_input(&mut fd, 0x104, 4);
        let op = mk_op(&mut fd, 2, OpCode::CPUI_INT_ADD);
        wire(&mut fd, op, v, 0);
        wire(&mut fd, op, nc, 1);
        assert_eq!(RuleAddMultCollapse.apply_op(op, &mut fd), 0);

        // inner op is a different opcode -> 0
        let c1 = fd.new_constant(4, 10);
        let (_inner, innerout) = def_op(&mut fd, OpCode::CPUI_INT_MULT, &[v, c1], 4);
        let c0 = fd.new_constant(4, 20);
        let op2 = mk_op(&mut fd, 2, OpCode::CPUI_INT_ADD); // ADD over a MULT sub
        wire(&mut fd, op2, innerout, 0);
        wire(&mut fd, op2, c0, 1);
        assert_eq!(RuleAddMultCollapse.apply_op(op2, &mut fd), 0);
    }

    // --- RuleShift2Mult ---------------------------------------------------

    #[test]
    fn shift2mult_feeding_arithmetic() {
        // (V << 2) feeding an INT_ADD => V * 4
        let mut fd = build_fd();
        let v = make_input(&mut fd, 0x100, 4);
        let two = fd.new_constant(4, 2);
        let (shiftop, shiftout) = def_op(&mut fd, OpCode::CPUI_INT_LEFT, &[v, two], 4);
        // descendant arithmetic op that reads shiftout
        let other = make_input(&mut fd, 0x200, 4);
        let _arith = def_op(&mut fd, OpCode::CPUI_INT_ADD, &[shiftout, other], 4);
        assert_eq!(RuleShift2Mult.apply_op(shiftop, &mut fd), 1);
        assert_eq!(op_code(&fd, shiftop), OpCode::CPUI_INT_MULT);
        let c = op_in(&fd, shiftop, 1).unwrap();
        assert!(vn_is_constant(&fd, c));
        assert_eq!(vn_offset(&fd, c), 1u64 << 2); // 4
    }

    #[test]
    fn shift2mult_no_arithmetic_rejected_and_oversized_rejected() {
        let mut fd = build_fd();
        // No arithmetic descendant, def of in0 is none => flag stays 0 => reject.
        let v = make_input(&mut fd, 0x100, 4);
        let two = fd.new_constant(4, 2);
        let (shiftop, _shiftout) = def_op(&mut fd, OpCode::CPUI_INT_LEFT, &[v, two], 4);
        assert_eq!(RuleShift2Mult.apply_op(shiftop, &mut fd), 0);
        assert_eq!(op_code(&fd, shiftop), OpCode::CPUI_INT_LEFT);

        // val >= 32 -> reject regardless of arithmetic.
        let v2 = make_input(&mut fd, 0x300, 4);
        let big = fd.new_constant(4, 40);
        let (shiftop2, sout2) = def_op(&mut fd, OpCode::CPUI_INT_LEFT, &[v2, big], 4);
        let o2 = make_input(&mut fd, 0x400, 4);
        let _arith2 = def_op(&mut fd, OpCode::CPUI_INT_ADD, &[sout2, o2], 4);
        assert_eq!(RuleShift2Mult.apply_op(shiftop2, &mut fd), 0);
    }

    // --- RuleSborrow / RuleScarry trivial cases (the ported half) ---------

    #[test]
    fn sborrow_trivial_zero_is_false() {
        let mut fd = build_fd();
        let v = make_input(&mut fd, 0x100, 4);
        let z = fd.new_constant(4, 0);
        let op = mk_op(&mut fd, 2, OpCode::CPUI_INT_SBORROW);
        wire(&mut fd, op, v, 0);
        wire(&mut fd, op, z, 1);
        assert_eq!(RuleSborrow.apply_op(op, &mut fd), 1);
        assert_eq!(op_code(&fd, op), OpCode::CPUI_COPY);
        assert_eq!(op_num_input(&fd, op), 1);
        let in0 = op_in(&fd, op, 0).unwrap();
        assert!(vn_is_constant(&fd, in0) && vn_offset(&fd, in0) == 0);
        // non-trivial (no AddExpression yet) -> 0 (seamed)
        let a = make_input(&mut fd, 0x200, 4);
        let b = make_input(&mut fd, 0x204, 4);
        let op2 = mk_op(&mut fd, 2, OpCode::CPUI_INT_SBORROW);
        wire(&mut fd, op2, a, 0);
        wire(&mut fd, op2, b, 1);
        assert_eq!(RuleSborrow.apply_op(op2, &mut fd), 0);
    }

    #[test]
    fn scarry_trivial_zero_is_false() {
        let mut fd = build_fd();
        let v = make_input(&mut fd, 0x100, 4);
        let z = fd.new_constant(4, 0);
        let op = mk_op(&mut fd, 2, OpCode::CPUI_INT_SCARRY);
        wire(&mut fd, op, v, 0);
        wire(&mut fd, op, z, 1);
        assert_eq!(RuleScarry.apply_op(op, &mut fd), 1);
        assert_eq!(op_code(&fd, op), OpCode::CPUI_COPY);
        assert_eq!(op_num_input(&fd, op), 1);
        // also fires when the *first* operand is the zero constant
        let z2 = fd.new_constant(4, 0);
        let w = make_input(&mut fd, 0x200, 4);
        let op2 = mk_op(&mut fd, 2, OpCode::CPUI_INT_SCARRY);
        wire(&mut fd, op2, z2, 0);
        wire(&mut fd, op2, w, 1);
        assert_eq!(RuleScarry.apply_op(op2, &mut fd), 1);
    }

    // --- RuleTestSign -----------------------------------------------------

    #[test]
    fn test_sign_srights_to_signed_compare() {
        // (V s>> 31) != 0  =>  V s< 0  (sgn = 1 from offset 0, complemented by
        // NOTEQUAL to -1 => SLESS path).
        let mut fd = build_fd();
        let v = make_input(&mut fd, 0x100, 4);
        let shamt = fd.new_constant(4, 31);
        let (srop, srout) = def_op(&mut fd, OpCode::CPUI_INT_SRIGHT, &[v, shamt], 4);
        // a comparison (srout != 0)
        let zero = fd.new_constant(4, 0);
        let cmp = mk_op(&mut fd, 2, OpCode::CPUI_INT_NOTEQUAL);
        wire(&mut fd, cmp, srout, 0);
        wire(&mut fd, cmp, zero, 1);
        assert_eq!(RuleTestSign.apply_op(srop, &mut fd), 1);
        // offset 0 => sgn=1, NOTEQUAL complements to -1 => SLESS: in0=V, in1=0.
        assert_eq!(op_code(&fd, cmp), OpCode::CPUI_INT_SLESS);
        assert_eq!(op_in(&fd, cmp, 0), Some(v));
        let in1 = op_in(&fd, cmp, 1).unwrap();
        assert!(vn_is_constant(&fd, in1) && vn_offset(&fd, in1) == 0);
    }

    #[test]
    fn test_sign_wrong_shift_amount_rejected() {
        let mut fd = build_fd();
        let v = make_input(&mut fd, 0x100, 4);
        let shamt = fd.new_constant(4, 16); // not 8*size-1
        let (srop, srout) = def_op(&mut fd, OpCode::CPUI_INT_SRIGHT, &[v, shamt], 4);
        let zero = fd.new_constant(4, 0);
        let cmp = mk_op(&mut fd, 2, OpCode::CPUI_INT_NOTEQUAL);
        wire(&mut fd, cmp, srout, 0);
        wire(&mut fd, cmp, zero, 1);
        assert_eq!(RuleTestSign.apply_op(srop, &mut fd), 0);
    }

    // --- RuleBoolZext (the EQUAL/NOTEQUAL branch is fully portable) --------

    #[test]
    fn bool_zext_compare_extended_bool() {
        // (zext(B) * -1) == -1  =>  B == 1
        let mut fd = build_fd();
        let b_in0 = make_input(&mut fd, 0x100, 1);
        let b_in1 = make_input(&mut fd, 0x101, 1);
        // boolean value: output of a BOOL_AND
        let (_aop, boolvn) = def_op(&mut fd, OpCode::CPUI_BOOL_AND, &[b_in0, b_in1], 1);
        // zext(B) : 4 bytes
        let (_zop, zout) = def_op(&mut fd, OpCode::CPUI_INT_ZEXT, &[boolvn], 4);
        // mult by -1 (mask of 4 bytes)
        let negone = fd.new_constant(4, calc_mask(4));
        let (_mop, mout) = def_op(&mut fd, OpCode::CPUI_INT_MULT, &[zout, negone], 4);
        // compare == -1
        let minus1 = fd.new_constant(4, calc_mask(4));
        let cmp = mk_op(&mut fd, 2, OpCode::CPUI_INT_EQUAL);
        wire(&mut fd, cmp, mout, 0);
        wire(&mut fd, cmp, minus1, 1);

        // Apply on the ZEXT op.
        let zop = vn_def(&fd, zout).unwrap();
        assert_eq!(RuleBoolZext.apply_op(zop, &mut fd), 1);
        // comparison's slot0 becomes the unextended boolean, slot1 the constant 1.
        assert_eq!(op_in(&fd, cmp, 0), Some(boolvn));
        let one = op_in(&fd, cmp, 1).unwrap();
        assert!(vn_is_constant(&fd, one));
        assert_eq!(vn_offset(&fd, one), 1);
    }

    // --- RulePropagateCopy ------------------------------------------------

    #[test]
    fn propagate_copy_replaces_input() {
        // op reads X where X = COPY(Y); propagate Y into op.
        let mut fd = build_fd();
        let y = make_input(&mut fd, 0x100, 4); // heritage-known input
        let (_copyop, x) = def_op(&mut fd, OpCode::CPUI_COPY, &[y], 4);
        let other = make_input(&mut fd, 0x200, 4);
        // a non-marker consumer (INT_ADD) reading x in slot 0
        let op = mk_op(&mut fd, 2, OpCode::CPUI_INT_ADD);
        wire(&mut fd, op, x, 0);
        wire(&mut fd, op, other, 1);
        assert_eq!(RulePropagateCopy.apply_op(op, &mut fd), 1);
        assert_eq!(op_in(&fd, op, 0), Some(y));
    }

    #[test]
    fn propagate_copy_skips_non_written_and_non_copy() {
        let mut fd = build_fd();
        // input not written -> skip; defined-but-not-copy -> skip.
        let a = make_input(&mut fd, 0x100, 4);
        let b = make_input(&mut fd, 0x104, 4);
        let (_addop, addout) = def_op(&mut fd, OpCode::CPUI_INT_ADD, &[a, b], 4);
        let c = make_input(&mut fd, 0x200, 4);
        let op = mk_op(&mut fd, 2, OpCode::CPUI_INT_MULT);
        wire(&mut fd, op, addout, 0); // written by INT_ADD, not COPY
        wire(&mut fd, op, c, 1); // not written
        assert_eq!(RulePropagateCopy.apply_op(op, &mut fd), 0);
    }

    // --- registration ordering -------------------------------------------

    #[test]
    fn specs_are_in_cpp_definition_order() {
        let specs = specs();
        assert_eq!(specs.len(), 22);
        // Spot-check the first, a middle, and the last by exercising the ctor
        // and the rule's getOpList (a cheap structural identity).
        let first = (specs[0].ctor)();
        assert_eq!(first.get_op_list(), vec![OpCode::CPUI_BOOL_AND, OpCode::CPUI_BOOL_OR]);
        let last = (specs[21].ctor)();
        assert_eq!(last.get_op_list(), vec![OpCode::CPUI_INT_ADD, OpCode::CPUI_INT_MULT]);
        // RuleCollapseConstants (index 14) applies to all opcodes (default list).
        let collapse = (specs[14].ctor)();
        assert!(collapse.get_op_list().len() > 2);
    }

    // =====================================================================
    // VERIFIER adversarial tests (item w5-s3-rules-3) — target the fully
    // ported rules whose rewrites are most fragile (slot selection, sgn
    // complement, the 1-i spacebase branch, the PIECE materialization).
    // =====================================================================

    /// RuleBoolZext, the INT_AND logic branch: `(zext(B1)*-1) & (zext(B2)*-1)`
    /// must rewrite to `zext(B1 && B2) * -1`.  This is the most complex
    /// multi-op rewrite in the batch; it exercises the `multop2` slot-selection
    /// (`multop1 == in0.def ? in1 : in0`) and the coeff==calc_mask(size) reuse.
    #[test]
    fn w5_s3_rules_3_bool_zext_and_branch_rewrites_to_zext_booland() {
        let mut fd = build_fd();
        // B1 = BOOL_AND(a,b) ; B2 = BOOL_OR(c,d)  (both boolean-by-opcode)
        let a = make_input(&mut fd, 0x100, 1);
        let b = make_input(&mut fd, 0x101, 1);
        let c = make_input(&mut fd, 0x102, 1);
        let d = make_input(&mut fd, 0x103, 1);
        let (_b1op, b1) = def_op(&mut fd, OpCode::CPUI_BOOL_AND, &[a, b], 1);
        let (_b2op, b2) = def_op(&mut fd, OpCode::CPUI_BOOL_OR, &[c, d], 1);
        // zext(B1), zext(B2) : 4 bytes
        let (_z1op, z1) = def_op(&mut fd, OpCode::CPUI_INT_ZEXT, &[b1], 4);
        let (_z2op, z2) = def_op(&mut fd, OpCode::CPUI_INT_ZEXT, &[b2], 4);
        // m1 = zext(B1) * -1 ; m2 = zext(B2) * -1
        let neg1a = fd.new_constant(4, calc_mask(4));
        let neg1b = fd.new_constant(4, calc_mask(4));
        let (_m1op, m1) = def_op(&mut fd, OpCode::CPUI_INT_MULT, &[z1, neg1a], 4);
        let (_m2op, m2) = def_op(&mut fd, OpCode::CPUI_INT_MULT, &[z2, neg1b], 4);
        // actionop = m1 & m2 (multop1 = m1's def is in slot0 of the AND here)
        let (andop, _andout) = def_op(&mut fd, OpCode::CPUI_INT_AND, &[m1, m2], 4);

        // apply on the ZEXT(B1) op (the rule's entry opcode is INT_ZEXT)
        let z1op = vn_def(&fd, z1).unwrap();
        assert_eq!(RuleBoolZext.apply_op(z1op, &mut fd), 1);

        // The AND op is now INT_MULT(newzext, -1); newzext = ZEXT(BOOL_AND(B1,B2)).
        assert_eq!(op_code(&fd, andop), OpCode::CPUI_INT_MULT);
        let newzout = op_in(&fd, andop, 0).unwrap();
        let coeff = op_in(&fd, andop, 1).unwrap();
        assert!(vn_is_constant(&fd, coeff));
        assert_eq!(vn_offset(&fd, coeff), calc_mask(4));
        let newzext = vn_def(&fd, newzout).unwrap();
        assert_eq!(op_code(&fd, newzext), OpCode::CPUI_INT_ZEXT);
        let boolres = op_in(&fd, newzext, 0).unwrap();
        let booland = vn_def(&fd, boolres).unwrap();
        // INT_AND maps to BOOL_AND; operands are the *unextended* booleans B1,B2.
        assert_eq!(op_code(&fd, booland), OpCode::CPUI_BOOL_AND);
        assert_eq!(op_in(&fd, booland, 0), Some(b1));
        assert_eq!(op_in(&fd, booland, 1), Some(b2));
    }

    /// RuleTestSign, the sgn == -1 path: `(V s>> 31) == -1` (INT_EQUAL, offset
    /// == calc_mask) gives sgn=-1, not complemented (EQUAL), so the rewrite is
    /// `V s< 0` (SLESS, in0=V, in1=0).  Distinct from the in-file NOTEQUAL/0
    /// case which goes through the complement to the same SLESS — here we reach
    /// SLESS via the raw -1 offset without complement.
    #[test]
    fn w5_s3_rules_3_test_sign_minus_one_equal_to_sless() {
        let mut fd = build_fd();
        let v = make_input(&mut fd, 0x100, 4);
        let shamt = fd.new_constant(4, 31);
        let (srop, srout) = def_op(&mut fd, OpCode::CPUI_INT_SRIGHT, &[v, shamt], 4);
        // compare: srout == -1  (offset == calc_mask(4))
        let minus1 = fd.new_constant(4, calc_mask(4));
        let cmp = mk_op(&mut fd, 2, OpCode::CPUI_INT_EQUAL);
        wire(&mut fd, cmp, srout, 0);
        wire(&mut fd, cmp, minus1, 1);
        assert_eq!(RuleTestSign.apply_op(srop, &mut fd), 1);
        // sgn = -1 (offset==mask), EQUAL does NOT complement => SLESS: in0=V,in1=0.
        assert_eq!(op_code(&fd, cmp), OpCode::CPUI_INT_SLESS);
        assert_eq!(op_in(&fd, cmp, 0), Some(v));
        let in1 = op_in(&fd, cmp, 1).unwrap();
        assert!(vn_is_constant(&fd, in1) && vn_offset(&fd, in1) == 0);
    }

    /// RuleTestSign, sgn == 1 with a NON-complementing EQUAL: `(V s>> 31) == 0`
    /// gives sgn=1, EQUAL keeps it, so SLESSEQUAL with `0 <= V` (in0=0, in1=V).
    /// This is the SLESSEQUAL arm, untouched by the in-file tests.
    #[test]
    fn w5_s3_rules_3_test_sign_zero_equal_to_slessequal() {
        let mut fd = build_fd();
        let v = make_input(&mut fd, 0x100, 4);
        let shamt = fd.new_constant(4, 31);
        let (srop, srout) = def_op(&mut fd, OpCode::CPUI_INT_SRIGHT, &[v, shamt], 4);
        let zero = fd.new_constant(4, 0);
        let cmp = mk_op(&mut fd, 2, OpCode::CPUI_INT_EQUAL);
        wire(&mut fd, cmp, srout, 0);
        wire(&mut fd, cmp, zero, 1);
        assert_eq!(RuleTestSign.apply_op(srop, &mut fd), 1);
        // sgn=1, EQUAL keeps sgn => SLESSEQUAL with in0=0, in1=V (0 s<= V).
        assert_eq!(op_code(&fd, cmp), OpCode::CPUI_INT_SLESSEQUAL);
        let in0 = op_in(&fd, cmp, 0).unwrap();
        assert!(vn_is_constant(&fd, in0) && vn_offset(&fd, in0) == 0);
        assert_eq!(op_in(&fd, cmp, 1), Some(v));
    }

    /// RuleAddMultCollapse 3-term branch must NOT fire when the base is not an
    /// input spacebase (the `isSpacebase()&&isInput()` guard).  `make_input`
    /// produces an input that is NOT a spacebase, so the `(stackbase + c1) +
    /// other) + c0` shape with a plain base falls through the `for i` loop and
    /// the rule returns 0 (no rewrite).  This pins the conservative side of the
    /// spacebase guard (the fire side needs the spacebase flag, which is not
    /// settable through the public API at W5 — its fire path is covered by the
    /// nested-constant in-file tests and re-derived in the verdict).
    #[test]
    fn w5_s3_rules_3_addmult_three_term_requires_input_spacebase() {
        let mut fd = build_fd();
        let base = make_input(&mut fd, 0x200, 4); // input, but NOT spacebase
        let c1 = fd.new_constant(4, 0x10);
        let (_baseop, baseout) = def_op(&mut fd, OpCode::CPUI_INT_ADD, &[base, c1], 4);
        let other = make_input(&mut fd, 0x300, 4);
        // inner ADD: (base+c1) in slot0, other in slot1.  Non-constant c1 path
        // enters the `for i` loop; base is not a spacebase => no rewrite.
        let (_innerop, innerout) = def_op(&mut fd, OpCode::CPUI_INT_ADD, &[baseout, other], 4);
        let c0 = fd.new_constant(4, 0x07);
        let op = mk_op(&mut fd, 2, OpCode::CPUI_INT_ADD);
        wire(&mut fd, op, innerout, 0);
        wire(&mut fd, op, c0, 1);
        assert_eq!(RuleAddMultCollapse.apply_op(op, &mut fd), 0);
        // unchanged: still an INT_ADD with the inner sum in slot0 and c0 in slot1.
        assert_eq!(op_code(&fd, op), OpCode::CPUI_INT_ADD);
        assert_eq!(op_in(&fd, op, 0), Some(innerout));
    }

    /// RuleShiftPiece, the zero-extend tail (`concatsize < out*8`): build the
    /// PIECE narrower than the output, so the rule inserts a NEW PIECE op and
    /// rewrites the original op to ZEXT(newpiece).  `(zext(V) << 16) | zext(W)`
    /// with V,W 1-byte each but the OR output 4 bytes: concatsize = 16 + 8 = 24
    /// (since sa must equal 8*size(W) = 8, we use sa=8) < 32 => ZEXT tail.
    #[test]
    fn w5_s3_rules_3_shiftpiece_zext_tail() {
        let mut fd = build_fd();
        let v = make_input(&mut fd, 0x100, 1);
        let w = make_input(&mut fd, 0x102, 1);
        let (_zvop, zv) = def_op(&mut fd, OpCode::CPUI_INT_ZEXT, &[v], 4);
        let sa = fd.new_constant(4, 8); // sa == 8*size(W) == 8
        let (_shop, shout) = def_op(&mut fd, OpCode::CPUI_INT_LEFT, &[zv, sa], 4);
        let (_zwop, zw) = def_op(&mut fd, OpCode::CPUI_INT_ZEXT, &[w], 4);
        let op = mk_op(&mut fd, 2, OpCode::CPUI_INT_OR);
        wire(&mut fd, op, shout, 0);
        wire(&mut fd, op, zw, 1);
        let _out = new_unique_out(&mut fd.fd, 4, op);

        assert_eq!(RuleShiftPiece.apply_op(op, &mut fd), 1);
        // concatsize = 8 + 8 = 16 < 32 => op becomes ZEXT(newpiece), where
        // newpiece = PIECE(V, W) of size 16/8 = 2 bytes.
        assert_eq!(op_code(&fd, op), OpCode::CPUI_INT_ZEXT);
        let pieceout = op_in(&fd, op, 0).unwrap();
        assert_eq!(vn_size(&fd, pieceout), 2);
        let pieceop = vn_def(&fd, pieceout).unwrap();
        assert_eq!(op_code(&fd, pieceop), OpCode::CPUI_PIECE);
        assert_eq!(op_in(&fd, pieceop, 0), Some(v));
        assert_eq!(op_in(&fd, pieceop, 1), Some(w));
    }

    /// RuleShiftPiece, the exact-size PIECE case: `(zext(V) << 16) | zext(W)`
    /// with sizes lined up (sa == 8*size(W), concatsize == 8*out) rewrites the
    /// op in place to `PIECE(V, W)`.  Exercises the swap branch (shiftop chosen
    /// by which input is the INT_LEFT) and the concatsize == out*8 equal path.
    #[test]
    fn w5_s3_rules_3_shiftpiece_exact_piece() {
        let mut fd = build_fd();
        // V (2 bytes), W (2 bytes); out is 4 bytes.
        let v = make_input(&mut fd, 0x100, 2);
        let w = make_input(&mut fd, 0x102, 2);
        // zext(V) -> 4 bytes, then << 16
        let (_zvop, zv) = def_op(&mut fd, OpCode::CPUI_INT_ZEXT, &[v], 4);
        let sa = fd.new_constant(4, 16);
        let (_shop, shout) = def_op(&mut fd, OpCode::CPUI_INT_LEFT, &[zv, sa], 4);
        // zext(W) -> 4 bytes (the zextloop low piece)
        let (_zwop, zw) = def_op(&mut fd, OpCode::CPUI_INT_ZEXT, &[w], 4);
        // op = shout | zw  (shiftop in slot0, zextloop in slot1)
        let op = mk_op(&mut fd, 2, OpCode::CPUI_INT_OR);
        wire(&mut fd, op, shout, 0);
        wire(&mut fd, op, zw, 1);
        let out = new_unique_out(&mut fd.fd, 4, op);
        let _ = out;

        assert_eq!(RuleShiftPiece.apply_op(op, &mut fd), 1);
        // concatsize == 16 + 8*2 == 32 == out*8 => in-place PIECE(V, W).
        assert_eq!(op_code(&fd, op), OpCode::CPUI_PIECE);
        assert_eq!(op_in(&fd, op, 0), Some(v));
        assert_eq!(op_in(&fd, op, 1), Some(w));
    }

    /// RuleShiftPiece swap branch: when the INT_LEFT is in slot1 (not slot0),
    /// the rule swaps `shiftop`/`zextloop` so the shift is still found.  Mirror
    /// of the exact-PIECE test with the operands transposed; result identical.
    #[test]
    fn w5_s3_rules_3_shiftpiece_swap_when_left_in_slot1() {
        let mut fd = build_fd();
        let v = make_input(&mut fd, 0x100, 2);
        let w = make_input(&mut fd, 0x102, 2);
        let (_zvop, zv) = def_op(&mut fd, OpCode::CPUI_INT_ZEXT, &[v], 4);
        let sa = fd.new_constant(4, 16);
        let (_shop, shout) = def_op(&mut fd, OpCode::CPUI_INT_LEFT, &[zv, sa], 4);
        let (_zwop, zw) = def_op(&mut fd, OpCode::CPUI_INT_ZEXT, &[w], 4);
        // op = zw | shout  (zextloop in slot0, shiftop in slot1 -> triggers swap)
        let op = mk_op(&mut fd, 2, OpCode::CPUI_INT_OR);
        wire(&mut fd, op, zw, 0);
        wire(&mut fd, op, shout, 1);
        let _out = new_unique_out(&mut fd.fd, 4, op);

        assert_eq!(RuleShiftPiece.apply_op(op, &mut fd), 1);
        assert_eq!(op_code(&fd, op), OpCode::CPUI_PIECE);
        assert_eq!(op_in(&fd, op, 0), Some(v));
        assert_eq!(op_in(&fd, op, 1), Some(w));
    }
}
