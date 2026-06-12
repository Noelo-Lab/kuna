//! Port of `decompiler/cpp/ruleaction.cc` lines 5527-6930 (W5 item
//! `w5-s3-rules-5`): nine `Rule` classes, in C++ definition order
//!
//!   1. [`RuleBoolNegate`]      — collapse a BOOL_NEGATE chain into its source op
//!   2. [`RuleLess2Zero`]       — INT_LESS against extremal constants
//!   3. [`RuleLessEqual2Zero`]  — INT_LESSEQUAL against extremal constants
//!   4. [`RuleSLess2Zero`]      — INT_SLESS against 0 / -1
//!   5. [`RuleEqual2Zero`]      — `0 == V + W*-1  =>  V == W`
//!   6. [`RuleEqual2Constant`]  — `V op c == d  =>  V == d'`
//!   7. [`RulePtrArith`]        — INT_ADD tree -> PTRADD/PTRSUB  *(SEAM-stubbed)*
//!   8. [`RuleStructOffset0`]   — LOAD/STORE of first struct field -> PTRSUB *(SEAM-stubbed)*
//!   9. [`RulePushPtr`]         — push a pointer to the bottom of its add tree *(SEAM-stubbed)*
//!
//! Each rule keeps its exact upstream `name()` string (`Rule(g,0,"<name>")`),
//! the exact `getOpList` contents, and an `applyOp` body transcribed
//! statement-for-statement.  Each rule carries its *group* string (the C++
//! `getGroup()`), so `clone_rule` filters with `grouplist.contains(group)`
//! exactly as the C++ `clone` does — the first six register in `"analysis"`,
//! the last three in `"typerecovery"` (see `coreaction.cc::universalAction`).
//! [`specs`] lists the rules in C++ definition order with their groups for the
//! W8 `universalAction` assembly.
//!
//! ## SEAM notes (do NOT invent type behavior)
//!
//! `Funcdata::opSetOpcode(op, OpCode)` resolves `glb->inst[opc]` — the W6
//! `TypeOp` table — to a [`TypeOp`] before handing it to the op bank.  That
//! table is W6's.  The merged W3/W5 IR exposes only
//! [`Funcdata::op_set_opcode`](crate::funcdata::Funcdata) taking an
//! already-resolved [`TypeOp`].  The small [`type_op_seam`] helper builds the
//! `TypeOp` for the handful of op-codes these rules set, transcribing the exact
//! `opflags` from `typeop.cc` (the same values `glb->inst[opc]` would cache).
//! `op.cc`'s `set_opcode` ORs those flags into the op's flag word, so
//! `isBoolOutput()` etc. stay correct.  SEAM(W6): `glb->inst[opc]`.
//!
//! `RulePtrArith`/`RuleStructOffset0`/`RulePushPtr` (and the `AddTreeState`
//! machinery and the static helpers they own) require *unported* surfaces:
//!   - W6 type-facing/relative-pointer reads
//!     (`Varnode::getTypeReadFacing(op)`, `TypePointer::getPtrTo`,
//!     `TypePointerRel::evaluateThruParent`/`getByteOffset`, …) — not yet on
//!     the Rust `Varnode`/`Datatype`;
//!   - W3-varnode op-creation factories with outputs
//!     (`newOpBefore`, `newUniqueOut`, `newVarnodeOut`), plus
//!     `opSetOutput`/`opUndoPtradd`/`distributeIntMultAdd`/`collapseIntMultMult`/
//!     `inheritUnionField`/`forceFacingType`/`warning` — deferred in the merged
//!     IR (`Funcdata::op_set_output` itself is an `Err` stub there).
//!
//! These three are ported as structurally-present rules (name/getOpList/specs)
//! whose `applyOp` is a SEAM stub returning `0` (no transform).  Each is in the
//! structured `losses` output.

use std::rc::Rc;

use kuna_base::address::{calc_mask, uintb_negate, Address};
use kuna_base::types::{int4, uint4};
use kuna_num::opcodes::{get_booleanflip, OpCode};

use crate::action::{ActionGroupList, Rule, RuleSpec};
use crate::funcdata::Funcdata;
use crate::op::pcodeop_flags;
use crate::seams::{OpId, TypeOp};

// =============================================================================
// SEAM(W6): opcode -> TypeOp resolution (glb->inst[opc])
// =============================================================================

/// Build the [`TypeOp`] for `opc` the way `glb->inst[opc]` would (SEAM(W6)).
///
/// `Funcdata::opSetOpcode(op, OpCode)` in C++ is `obank.changeOpcode(op,
/// glb->inst[opc])`: it looks up the singleton [`TypeOp`] in the architecture's
/// instruction table.  That table is the W6 `TypeFactory`-backed structure and
/// is not ported yet, so this helper resolves exactly the op-codes the rules in
/// this file ever *set*, transcribing the `opflags` from `typeop.cc` verbatim
/// (those are the bits `set_opcode` ORs into the op's flag word, on which
/// `isBoolOutput`/`isCommutative`/etc. depend).  The `name` is the upstream
/// display symbol.
///
/// The set of op-codes the rules in this file can set is: `CPUI_COPY` and
/// `CPUI_INT_EQUAL`/`CPUI_INT_NOTEQUAL` (RuleEqual2Zero/RuleEqual2Constant), plus
/// every op-code `get_booleanflip` can return for a boolean-output input op
/// (RuleBoolNegate sets `flip_op`'s code to `get_booleanflip(flip_op->code())`).
/// `get_booleanflip` (opcodes.cc:94-135) returns one of: `CPUI_COPY`,
/// `CPUI_INT_EQUAL`, `CPUI_INT_NOTEQUAL`, `CPUI_INT_SLESS`, `CPUI_INT_SLESSEQUAL`,
/// `CPUI_INT_LESS`, `CPUI_INT_LESSEQUAL`, `CPUI_FLOAT_EQUAL`,
/// `CPUI_FLOAT_NOTEQUAL`, `CPUI_FLOAT_LESS`, `CPUI_FLOAT_LESSEQUAL`.  All of those
/// are covered below with their `glb->inst[opc]` opflags so the flipped op keeps
/// the right `binary`/`booloutput`/`commutative` bits.  Any other op-code is a
/// porting bug (the rules below set only these), so the fallback uses the debug
/// symbol with no flags.
fn type_op_seam(opc: OpCode) -> TypeOp {
    // opflags transcribed verbatim from decompiler/cpp/typeop.cc.  SEAM(W6).
    let (flags, name): (uint4, &str) = match opc {
        // TypeOpCopy: PcodeOp::unary | PcodeOp::nocollapse
        OpCode::CPUI_COPY => (pcodeop_flags::unary | pcodeop_flags::nocollapse, "copy"),
        // TypeOpEqual: PcodeOp::binary | PcodeOp::booloutput | PcodeOp::commutative
        OpCode::CPUI_INT_EQUAL => (
            pcodeop_flags::binary | pcodeop_flags::booloutput | pcodeop_flags::commutative,
            "==",
        ),
        // TypeOpNotEqual: PcodeOp::binary | PcodeOp::booloutput | PcodeOp::commutative
        OpCode::CPUI_INT_NOTEQUAL => (
            pcodeop_flags::binary | pcodeop_flags::booloutput | pcodeop_flags::commutative,
            "!=",
        ),
        // TypeOpIntSless: PcodeOp::binary | PcodeOp::booloutput
        OpCode::CPUI_INT_SLESS => (pcodeop_flags::binary | pcodeop_flags::booloutput, "<"),
        // TypeOpIntSlessEqual: PcodeOp::binary | PcodeOp::booloutput
        OpCode::CPUI_INT_SLESSEQUAL => (pcodeop_flags::binary | pcodeop_flags::booloutput, "<="),
        // TypeOpIntLess: PcodeOp::binary | PcodeOp::booloutput
        OpCode::CPUI_INT_LESS => (pcodeop_flags::binary | pcodeop_flags::booloutput, "<"),
        // TypeOpIntLessEqual: PcodeOp::binary | PcodeOp::booloutput
        OpCode::CPUI_INT_LESSEQUAL => (pcodeop_flags::binary | pcodeop_flags::booloutput, "<="),
        // TypeOpFloatEqual: PcodeOp::binary | PcodeOp::booloutput | PcodeOp::commutative
        OpCode::CPUI_FLOAT_EQUAL => (
            pcodeop_flags::binary | pcodeop_flags::booloutput | pcodeop_flags::commutative,
            "==",
        ),
        // TypeOpFloatNotEqual: PcodeOp::binary | PcodeOp::booloutput | PcodeOp::commutative
        OpCode::CPUI_FLOAT_NOTEQUAL => (
            pcodeop_flags::binary | pcodeop_flags::booloutput | pcodeop_flags::commutative,
            "!=",
        ),
        // TypeOpFloatLess: PcodeOp::binary | PcodeOp::booloutput
        OpCode::CPUI_FLOAT_LESS => (pcodeop_flags::binary | pcodeop_flags::booloutput, "<"),
        // TypeOpFloatLessEqual: PcodeOp::binary | PcodeOp::booloutput
        OpCode::CPUI_FLOAT_LESSEQUAL => (pcodeop_flags::binary | pcodeop_flags::booloutput, "<="),
        // TypeOpBoolNegate: PcodeOp::unary | PcodeOp::booloutput
        OpCode::CPUI_BOOL_NEGATE => (pcodeop_flags::unary | pcodeop_flags::booloutput, "!"),
        // Fallback: should not be reached by these rules.  No flags invented.
        other => return TypeOp::new(other, 0, format!("{other:?}")),
    };
    TypeOp::new(opc, flags, name)
}

// =============================================================================
// RuleBoolNegate (ruleaction.cc:5527)
// =============================================================================

/// \brief Collapse a chain of BOOL_NEGATEs over a comparison into the negated
/// comparison (C++ `RuleBoolNegate`).
///
/// If a Varnode `vn` is produced by a boolean op `flip_op` and *every*
/// descendant of `vn` is a BOOL_NEGATE, flip `flip_op`'s op-code (via
/// [`get_booleanflip`]) and turn all the negates into COPYs.
pub struct RuleBoolNegate {
    group: &'static str,
}

impl RuleBoolNegate {
    /// Constructor (C++ `RuleBoolNegate(g) : Rule(g,0,"boolnegate")`).
    pub fn new(g: &'static str) -> RuleBoolNegate {
        RuleBoolNegate { group: g }
    }
}

impl Rule for RuleBoolNegate {
    fn get_op_list(&self) -> Vec<OpCode> {
        // oplist.push_back(CPUI_BOOL_NEGATE);
        vec![OpCode::CPUI_BOOL_NEGATE]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        // if (!grouplist.contains(getGroup())) return 0; return new RuleBoolNegate(getGroup());
        if !grouplist.contains(self.group) {
            return None;
        }
        Some(Box::new(RuleBoolNegate { group: self.group }))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // vn = op->getIn(0);
        let vn = data.obank().get(op).expect("RuleBoolNegate: stale op").get_in(0).unwrap();
        // if (!vn->isWritten()) return 0;
        let v = data.vbank().get(vn).expect("RuleBoolNegate: stale vn");
        if !v.is_written() {
            return 0;
        }
        // flip_op = vn->getDef();
        let flip_op = v.get_def().expect("RuleBoolNegate: written vn has no def");

        // ALL descendants must be negates
        // for(iter=vn->beginDescend();...) if ((*iter)->code() != CPUI_BOOL_NEGATE) return 0;
        let descend = data.descend_snapshot(vn);
        for &decop in &descend {
            if data.obank().get(decop).expect("RuleBoolNegate: stale descend op").code()
                != OpCode::CPUI_BOOL_NEGATE
            {
                return 0;
            }
        }

        // opc = get_booleanflip(flip_op->code(),flipyes);
        let mut flipyes = false;
        let flip_code = data.obank().get(flip_op).expect("RuleBoolNegate: stale flip_op").code();
        let opc = get_booleanflip(flip_code, &mut flipyes);
        // if (opc == CPUI_MAX) return 0;
        if opc == OpCode::CPUI_MAX {
            return 0;
        }
        // data.opSetOpcode(flip_op,opc); // Set the negated opcode
        data.op_set_opcode(flip_op, type_op_seam(opc));
        // if (flipyes) data.opSwapInput(flip_op,0,1);
        if flipyes {
            data.op_swap_input(flip_op, 0, 1);
        }
        // for(...) data.opSetOpcode(*iter,CPUI_COPY); // Remove all the negates
        for &decop in &descend {
            data.op_set_opcode(decop, type_op_seam(OpCode::CPUI_COPY));
        }
        1
    }
}

// =============================================================================
// RuleLess2Zero (ruleaction.cc:5561)
// =============================================================================

/// \brief Simplify INT_LESS applied to extremal constants (C++ `RuleLess2Zero`).
///
/// Forms include:
///  - `0 < V  =>  0 != V`
///  - `V < 0  =>  false`
///  - `ffff < V  =>  false`
///  - `V < ffff` =>  V != ffff`
pub struct RuleLess2Zero {
    group: &'static str,
}

impl RuleLess2Zero {
    /// Constructor (C++ `RuleLess2Zero(g) : Rule(g,0,"less2zero")`).
    pub fn new(g: &'static str) -> RuleLess2Zero {
        RuleLess2Zero { group: g }
    }
}

impl Rule for RuleLess2Zero {
    fn get_op_list(&self) -> Vec<OpCode> {
        // oplist.push_back(CPUI_INT_LESS);
        vec![OpCode::CPUI_INT_LESS]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(self.group) {
            return None;
        }
        Some(Box::new(RuleLess2Zero { group: self.group }))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // lvn = op->getIn(0); rvn = op->getIn(1);
        let o = data.obank().get(op).expect("RuleLess2Zero: stale op");
        let lvn = o.get_in(0).unwrap();
        let rvn = o.get_in(1).unwrap();
        let l = data.vbank().get(lvn).expect("RuleLess2Zero: stale lvn");
        let r = data.vbank().get(rvn).expect("RuleLess2Zero: stale rvn");

        if l.is_constant() {
            // if (lvn->getOffset() == 0)
            if l.get_offset() == 0 {
                // data.opSetOpcode(op,CPUI_INT_NOTEQUAL);  // ->  NOT_EQUAL
                data.op_set_opcode(op, type_op_seam(OpCode::CPUI_INT_NOTEQUAL));
                return 1;
            }
            // else if (lvn->getOffset() == calc_mask(lvn->getSize()))
            else if l.get_offset() == calc_mask(l.get_size()) {
                // data.opSetOpcode(op,CPUI_COPY); // Always false
                data.op_set_opcode(op, type_op_seam(OpCode::CPUI_COPY));
                // data.opRemoveInput(op,1);
                data.op_remove_input(op, 1);
                // data.opSetInput(op,data.newConstant(1,0),0);
                let c = data.new_constant(1, 0);
                data.op_set_input(op, c, 0).expect("RuleLess2Zero: opSetInput");
                return 1;
            }
        } else if r.is_constant() {
            // if (rvn->getOffset() == 0)
            if r.get_offset() == 0 {
                // data.opSetOpcode(op,CPUI_COPY); // Always false
                data.op_set_opcode(op, type_op_seam(OpCode::CPUI_COPY));
                data.op_remove_input(op, 1);
                let c = data.new_constant(1, 0);
                data.op_set_input(op, c, 0).expect("RuleLess2Zero: opSetInput");
                return 1;
            }
            // else if (rvn->getOffset() == calc_mask(rvn->getSize()))  // -> NOT_EQUAL
            else if r.get_offset() == calc_mask(r.get_size()) {
                // data.opSetOpcode(op,CPUI_INT_NOTEQUAL);
                data.op_set_opcode(op, type_op_seam(OpCode::CPUI_INT_NOTEQUAL));
                return 1;
            }
        }
        0
    }
}

// =============================================================================
// RuleLessEqual2Zero (ruleaction.cc:5609)
// =============================================================================

/// \brief Simplify INT_LESSEQUAL applied to extremal constants
/// (C++ `RuleLessEqual2Zero`).
///
/// Forms include:
///  - `0 <= V  =>  true`
///  - `V <= 0  =>  V == 0`
///  - `ffff <= V  =>  ffff == V`
///  - `V <= ffff` =>  true`
pub struct RuleLessEqual2Zero {
    group: &'static str,
}

impl RuleLessEqual2Zero {
    /// Constructor (C++ `RuleLessEqual2Zero(g) : Rule(g,0,"lessequal2zero")`).
    pub fn new(g: &'static str) -> RuleLessEqual2Zero {
        RuleLessEqual2Zero { group: g }
    }
}

impl Rule for RuleLessEqual2Zero {
    fn get_op_list(&self) -> Vec<OpCode> {
        // oplist.push_back(CPUI_INT_LESSEQUAL);
        vec![OpCode::CPUI_INT_LESSEQUAL]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(self.group) {
            return None;
        }
        Some(Box::new(RuleLessEqual2Zero { group: self.group }))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        let o = data.obank().get(op).expect("RuleLessEqual2Zero: stale op");
        let lvn = o.get_in(0).unwrap();
        let rvn = o.get_in(1).unwrap();
        let l = data.vbank().get(lvn).expect("RuleLessEqual2Zero: stale lvn");
        let r = data.vbank().get(rvn).expect("RuleLessEqual2Zero: stale rvn");

        if l.is_constant() {
            if l.get_offset() == 0 {
                // data.opSetOpcode(op,CPUI_COPY); // All values => true
                data.op_set_opcode(op, type_op_seam(OpCode::CPUI_COPY));
                data.op_remove_input(op, 1);
                // data.opSetInput(op,data.newConstant(1,1),0);
                let c = data.new_constant(1, 1);
                data.op_set_input(op, c, 0).expect("RuleLessEqual2Zero: opSetInput");
                return 1;
            } else if l.get_offset() == calc_mask(l.get_size()) {
                // data.opSetOpcode(op,CPUI_INT_EQUAL); // No value is true except -1
                data.op_set_opcode(op, type_op_seam(OpCode::CPUI_INT_EQUAL));
                return 1;
            }
        } else if r.is_constant() {
            if r.get_offset() == 0 {
                // data.opSetOpcode(op,CPUI_INT_EQUAL); // No value is true except 0
                data.op_set_opcode(op, type_op_seam(OpCode::CPUI_INT_EQUAL));
                return 1;
            } else if r.get_offset() == calc_mask(r.get_size()) {
                // data.opSetOpcode(op,CPUI_COPY); // All values => true
                data.op_set_opcode(op, type_op_seam(OpCode::CPUI_COPY));
                data.op_remove_input(op, 1);
                let c = data.new_constant(1, 1);
                data.op_set_input(op, c, 0).expect("RuleLessEqual2Zero: opSetInput");
                return 1;
            }
        }
        0
    }
}

// =============================================================================
// RuleSLess2Zero (ruleaction.cc:5657)
// =============================================================================

/// \brief Simplify INT_SLESS applied to 0 or -1 (C++ `RuleSLess2Zero`).
///
/// See the class doc-comment in `ruleaction.cc` for the full list of forms.
pub struct RuleSLess2Zero {
    group: &'static str,
}

impl RuleSLess2Zero {
    /// Constructor (C++ `RuleSLess2Zero(g) : Rule(g,0,"sless2zero")`).
    pub fn new(g: &'static str) -> RuleSLess2Zero {
        RuleSLess2Zero { group: g }
    }

    /// \brief Get the piece containing the sign-bit (C++
    /// `RuleSLess2Zero::getHiBit`).
    ///
    /// If `op` pieces together 2 Varnodes only one of which determines the high
    /// bit, return that Varnode (else `None`).
    fn get_hi_bit(op: OpId, data: &Funcdata) -> Option<crate::seams::VarnodeId> {
        // OpCode opc = op->code();
        let o = data.obank().get(op).expect("getHiBit: stale op");
        let opc = o.code();
        // if ((opc != CPUI_INT_ADD)&&(opc != CPUI_INT_OR)&&(opc != CPUI_INT_XOR)) return 0;
        if opc != OpCode::CPUI_INT_ADD
            && opc != OpCode::CPUI_INT_OR
            && opc != OpCode::CPUI_INT_XOR
        {
            return None;
        }
        // Varnode *vn1 = op->getIn(0); Varnode *vn2 = op->getIn(1);
        let vn1 = o.get_in(0).unwrap();
        let vn2 = o.get_in(1).unwrap();
        let v1 = data.vbank().get(vn1).expect("getHiBit: stale vn1");
        let v2 = data.vbank().get(vn2).expect("getHiBit: stale vn2");
        // uintb mask = calc_mask(vn1->getSize()); mask = (mask ^ (mask>>1)); // high-bit only
        let mask = calc_mask(v1.get_size());
        let mask = mask ^ (mask >> 1);
        // uintb nzmask1 = vn1->getNZMask();
        let nzmask1 = v1.get_nz_mask();
        // if ((nzmask1!=mask)&&((nzmask1 & mask)!=0)) return 0;  // high-bit set AND some other bit
        if nzmask1 != mask && (nzmask1 & mask) != 0 {
            return None;
        }
        // uintb nzmask2 = vn2->getNZMask();
        let nzmask2 = v2.get_nz_mask();
        if nzmask2 != mask && (nzmask2 & mask) != 0 {
            return None;
        }
        // if (nzmask1 == mask) return vn1;
        if nzmask1 == mask {
            return Some(vn1);
        }
        // if (nzmask2 == mask) return vn2;
        if nzmask2 == mask {
            return Some(vn2);
        }
        None
    }
}

impl Rule for RuleSLess2Zero {
    fn get_op_list(&self) -> Vec<OpCode> {
        // oplist.push_back(CPUI_INT_SLESS);
        vec![OpCode::CPUI_INT_SLESS]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(self.group) {
            return None;
        }
        Some(Box::new(RuleSLess2Zero { group: self.group }))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // lvn = op->getIn(0); rvn = op->getIn(1);
        let o = data.obank().get(op).expect("RuleSLess2Zero: stale op");
        let lvn = o.get_in(0).unwrap();
        let rvn = o.get_in(1).unwrap();
        let l = data.vbank().get(lvn).expect("RuleSLess2Zero: stale lvn");
        let lvn_is_constant = l.is_constant();
        let lvn_off = l.get_offset();
        let lvn_size = l.get_size();
        let r = data.vbank().get(rvn).expect("RuleSLess2Zero: stale rvn");
        let rvn_is_constant = r.is_constant();
        let rvn_off = r.get_offset();
        let rvn_size = r.get_size();

        if lvn_is_constant {
            // if (!rvn->isWritten()) return 0;
            if !data.vbank().get(rvn).unwrap().is_written() {
                return 0;
            }
            // if (lvn->getOffset() == calc_mask(lvn->getSize()))  // -1 s< rvn
            if lvn_off == calc_mask(lvn_size) {
                // feedOp = rvn->getDef(); feedOpCode = feedOp->code();
                let feed_op = data.vbank().get(rvn).unwrap().get_def().unwrap();
                let feed_opcode = data.obank().get(feed_op).unwrap().code();
                // Varnode *hibit = getHiBit(feedOp);
                let hibit = RuleSLess2Zero::get_hi_bit(feed_op, data);
                if let Some(hibit) = hibit {
                    // Test for -1 s< (hi ^ lo)
                    let hv = data.vbank().get(hibit).unwrap();
                    if hv.is_constant() {
                        // data.opSetInput(op, data.newConstant(hibit->getSize(), hibit->getOffset()), 1);
                        let sz = hv.get_size();
                        let off = hv.get_offset();
                        let c = data.new_constant(sz, off);
                        data.op_set_input(op, c, 1).expect("sless2zero: setInput");
                    } else {
                        // data.opSetInput(op, hibit, 1);
                        data.op_set_input(op, hibit, 1).expect("sless2zero: setInput");
                    }
                    // data.opSetOpcode(op, CPUI_INT_EQUAL);
                    data.op_set_opcode(op, type_op_seam(OpCode::CPUI_INT_EQUAL));
                    // data.opSetInput(op, data.newConstant(hibit->getSize(), 0), 0);
                    let sz = data.vbank().get(hibit).unwrap().get_size();
                    let c = data.new_constant(sz, 0);
                    data.op_set_input(op, c, 0).expect("sless2zero: setInput");
                    return 1;
                } else if feed_opcode == OpCode::CPUI_SUBPIECE {
                    // avn = feedOp->getIn(0);
                    let avn = data.obank().get(feed_op).unwrap().get_in(0).unwrap();
                    let av = data.vbank().get(avn).unwrap();
                    // if (avn->isFree() || avn->getSize() > 8) return 0;
                    if av.is_free() || av.get_size() > 8 {
                        return 0;
                    }
                    let avn_size = av.get_size();
                    // if (rvn->getSize() + (int4)feedOp->getIn(1)->getOffset() == avn->getSize())
                    let in1 = data.obank().get(feed_op).unwrap().get_in(1).unwrap();
                    let in1_off = data.vbank().get(in1).unwrap().get_offset();
                    if rvn_size + (in1_off as int4) == avn_size {
                        // We have -1 s< SUB( avn, #hi )
                        // data.opSetInput(op, avn, 1);
                        data.op_set_input(op, avn, 1).expect("sless2zero: setInput");
                        // data.opSetInput(op, data.newConstant(avn->getSize(), calc_mask(avn->getSize())), 0);
                        let c = data.new_constant(avn_size, calc_mask(avn_size));
                        data.op_set_input(op, c, 0).expect("sless2zero: setInput");
                        return 1;
                    }
                } else if feed_opcode == OpCode::CPUI_INT_NEGATE {
                    // We have -1 s< ~avn
                    let avn = data.obank().get(feed_op).unwrap().get_in(0).unwrap();
                    let av = data.vbank().get(avn).unwrap();
                    // if (avn->isFree()) return 0;
                    if av.is_free() {
                        return 0;
                    }
                    let avn_size = av.get_size();
                    // data.opSetInput(op, avn, 0);
                    data.op_set_input(op, avn, 0).expect("sless2zero: setInput");
                    // data.opSetInput(op, data.newConstant(avn->getSize(), 0), 1);
                    let c = data.new_constant(avn_size, 0);
                    data.op_set_input(op, c, 1).expect("sless2zero: setInput");
                    return 1;
                } else if feed_opcode == OpCode::CPUI_INT_AND {
                    let avn = data.obank().get(feed_op).unwrap().get_in(0).unwrap();
                    // if (avn->isFree() || rvn->loneDescend() == 0) return 0;
                    if data.vbank().get(avn).unwrap().is_free()
                        || data.lone_descend(rvn).is_none()
                    {
                        return 0;
                    }
                    let avn_size = data.vbank().get(avn).unwrap().get_size();
                    // Varnode *maskVn = feedOp->getIn(1);
                    let mask_vn = data.obank().get(feed_op).unwrap().get_in(1).unwrap();
                    let mv = data.vbank().get(mask_vn).unwrap();
                    if mv.is_constant() {
                        // uintb mask = maskVn->getOffset(); mask >>= (8 * avn->getSize() - 1);
                        let mask = mv.get_offset() >> (8 * avn_size - 1);
                        // if ((mask & 1) != 0)  // -1 s< avn & 0x8...
                        if (mask & 1) != 0 {
                            // data.opSetInput(op, avn, 1);
                            data.op_set_input(op, avn, 1).expect("sless2zero: setInput");
                            return 1;
                        }
                    }
                } else if feed_opcode == OpCode::CPUI_PIECE {
                    // We have -1 s< CONCAT(V,W)
                    let avn = data.obank().get(feed_op).unwrap().get_in(0).unwrap(); // most significant
                    let av = data.vbank().get(avn).unwrap();
                    if av.is_free() {
                        return 0;
                    }
                    let avn_size = av.get_size();
                    // data.opSetInput(op, avn, 1);
                    data.op_set_input(op, avn, 1).expect("sless2zero: setInput");
                    // data.opSetInput(op, data.newConstant(avn->getSize(),calc_mask(avn->getSize())), 0);
                    let c = data.new_constant(avn_size, calc_mask(avn_size));
                    data.op_set_input(op, c, 0).expect("sless2zero: setInput");
                    return 1;
                } else if feed_opcode == OpCode::CPUI_INT_LEFT {
                    // coeff = feedOp->getIn(1);
                    let coeff = data.obank().get(feed_op).unwrap().get_in(1).unwrap();
                    let cf = data.vbank().get(coeff).unwrap();
                    // if (!coeff->isConstant() || coeff->getOffset() != lvn->getSize() * 8 - 1) return 0;
                    if !cf.is_constant() || cf.get_offset() != (lvn_size as u64) * 8 - 1 {
                        return 0;
                    }
                    // avn = feedOp->getIn(0);
                    let avn = data.obank().get(feed_op).unwrap().get_in(0).unwrap();
                    let av = data.vbank().get(avn).unwrap();
                    // if (!avn->isWritten() || !avn->getDef()->isBoolOutput()) return 0;
                    if !av.is_written() {
                        return 0;
                    }
                    let avn_def = av.get_def().unwrap();
                    if !data.obank().get(avn_def).unwrap().is_bool_output() {
                        return 0;
                    }
                    // We have -1 s< (bool << #8*sz-1)
                    // data.opSetOpcode(op, CPUI_BOOL_NEGATE);
                    data.op_set_opcode(op, type_op_seam(OpCode::CPUI_BOOL_NEGATE));
                    // data.opRemoveInput(op, 1);
                    data.op_remove_input(op, 1);
                    // data.opSetInput(op, avn, 0);
                    data.op_set_input(op, avn, 0).expect("sless2zero: setInput");
                    return 1;
                }
            }
        } else if rvn_is_constant {
            // if (!lvn->isWritten()) return 0;
            if !data.vbank().get(lvn).unwrap().is_written() {
                return 0;
            }
            // if (rvn->getOffset() == 0)  // ... s< 0
            if rvn_off == 0 {
                // feedOp = lvn->getDef(); feedOpCode = feedOp->code();
                let feed_op = data.vbank().get(lvn).unwrap().get_def().unwrap();
                let feed_opcode = data.obank().get(feed_op).unwrap().code();
                // Varnode *hibit = getHiBit(feedOp);
                let hibit = RuleSLess2Zero::get_hi_bit(feed_op, data);
                if let Some(hibit) = hibit {
                    // Test for (hi ^ lo) s< 0
                    let hv = data.vbank().get(hibit).unwrap();
                    if hv.is_constant() {
                        let sz = hv.get_size();
                        let off = hv.get_offset();
                        let c = data.new_constant(sz, off);
                        data.op_set_input(op, c, 0).expect("sless2zero: setInput");
                    } else {
                        data.op_set_input(op, hibit, 0).expect("sless2zero: setInput");
                    }
                    // data.opSetOpcode(op,CPUI_INT_NOTEQUAL);
                    data.op_set_opcode(op, type_op_seam(OpCode::CPUI_INT_NOTEQUAL));
                    return 1;
                } else if feed_opcode == OpCode::CPUI_SUBPIECE {
                    let avn = data.obank().get(feed_op).unwrap().get_in(0).unwrap();
                    let av = data.vbank().get(avn).unwrap();
                    // if (avn->isFree() || avn->getSize() > 8) return 0;
                    if av.is_free() || av.get_size() > 8 {
                        return 0;
                    }
                    let avn_size = av.get_size();
                    let in1 = data.obank().get(feed_op).unwrap().get_in(1).unwrap();
                    let in1_off = data.vbank().get(in1).unwrap().get_offset();
                    // if (lvn->getSize() + (int4)feedOp->getIn(1)->getOffset() == avn->getSize())
                    if lvn_size + (in1_off as int4) == avn_size {
                        // We have SUB( avn, #hi ) s< 0
                        // data.opSetInput(op,avn,0);
                        data.op_set_input(op, avn, 0).expect("sless2zero: setInput");
                        // data.opSetInput(op,data.newConstant(avn->getSize(),0),1);
                        let c = data.new_constant(avn_size, 0);
                        data.op_set_input(op, c, 1).expect("sless2zero: setInput");
                        return 1;
                    }
                } else if feed_opcode == OpCode::CPUI_INT_NEGATE {
                    // We have ~avn s< 0
                    let avn = data.obank().get(feed_op).unwrap().get_in(0).unwrap();
                    let av = data.vbank().get(avn).unwrap();
                    if av.is_free() {
                        return 0;
                    }
                    let avn_size = av.get_size();
                    // data.opSetInput(op,avn,1);
                    data.op_set_input(op, avn, 1).expect("sless2zero: setInput");
                    // data.opSetInput(op,data.newConstant(avn->getSize(),calc_mask(avn->getSize())),0);
                    let c = data.new_constant(avn_size, calc_mask(avn_size));
                    data.op_set_input(op, c, 0).expect("sless2zero: setInput");
                    return 1;
                } else if feed_opcode == OpCode::CPUI_INT_AND {
                    let avn = data.obank().get(feed_op).unwrap().get_in(0).unwrap();
                    // if (avn->isFree() || lvn->loneDescend() == 0) return 0;
                    if data.vbank().get(avn).unwrap().is_free()
                        || data.lone_descend(lvn).is_none()
                    {
                        return 0;
                    }
                    let avn_size = data.vbank().get(avn).unwrap().get_size();
                    let mask_vn = data.obank().get(feed_op).unwrap().get_in(1).unwrap();
                    let mv = data.vbank().get(mask_vn).unwrap();
                    if mv.is_constant() {
                        let mask = mv.get_offset() >> (8 * avn_size - 1);
                        // if ((mask & 1) != 0)  // avn & 0x8... s< 0
                        if (mask & 1) != 0 {
                            // data.opSetInput(op, avn, 0);
                            data.op_set_input(op, avn, 0).expect("sless2zero: setInput");
                            return 1;
                        }
                    }
                } else if feed_opcode == OpCode::CPUI_PIECE {
                    // We have CONCAT(V,W) s< 0
                    let avn = data.obank().get(feed_op).unwrap().get_in(0).unwrap(); // most significant
                    let av = data.vbank().get(avn).unwrap();
                    if av.is_free() {
                        return 0;
                    }
                    let avn_size = av.get_size();
                    // data.opSetInput(op, avn, 0);
                    data.op_set_input(op, avn, 0).expect("sless2zero: setInput");
                    // data.opSetInput(op, data.newConstant(avn->getSize(), 0), 1);
                    let c = data.new_constant(avn_size, 0);
                    data.op_set_input(op, c, 1).expect("sless2zero: setInput");
                    return 1;
                }
            }
        }
        0
    }
}

// =============================================================================
// RuleEqual2Zero (ruleaction.cc:5861)
// =============================================================================

/// \brief Simplify INT_EQUAL applied to 0 (C++ `RuleEqual2Zero`).
///
/// `0 == V + W * -1  =>  V == W` or `0 == V + c  =>  V == -c`.  Also applies to
/// INT_NOTEQUAL.
pub struct RuleEqual2Zero {
    group: &'static str,
}

impl RuleEqual2Zero {
    /// Constructor (C++ `RuleEqual2Zero(g) : Rule(g,0,"equal2zero")`).
    pub fn new(g: &'static str) -> RuleEqual2Zero {
        RuleEqual2Zero { group: g }
    }
}

impl Rule for RuleEqual2Zero {
    fn get_op_list(&self) -> Vec<OpCode> {
        // uint4 list[] = { CPUI_INT_EQUAL, CPUI_INT_NOTEQUAL };
        vec![OpCode::CPUI_INT_EQUAL, OpCode::CPUI_INT_NOTEQUAL]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(self.group) {
            return None;
        }
        Some(Box::new(RuleEqual2Zero { group: self.group }))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // vn = op->getIn(0);
        let o = data.obank().get(op).expect("RuleEqual2Zero: stale op");
        let in0 = o.get_in(0).unwrap();
        let in1 = o.get_in(1).unwrap();
        let v0 = data.vbank().get(in0).unwrap();
        // if ((vn->isConstant())&&(vn->getOffset() == 0)) addvn = op->getIn(1);
        let addvn = if v0.is_constant() && v0.get_offset() == 0 {
            in1
        } else {
            // else { addvn = vn; vn = op->getIn(1);
            //        if ((!vn->isConstant())||(vn->getOffset() != 0)) return 0; }
            let v1 = data.vbank().get(in1).unwrap();
            if !v1.is_constant() || v1.get_offset() != 0 {
                return 0;
            }
            in0
        };

        // make sure the sum is only used in comparisons
        // for(...addvn->beginDescend()...) if (!boolop->isBoolOutput()) return 0;
        let descend = data.descend_snapshot(addvn);
        for &boolop in &descend {
            if !data
                .obank()
                .get(boolop)
                .expect("RuleEqual2Zero: stale descend op")
                .is_bool_output()
            {
                return 0;
            }
        }
        // addop = addvn->getDef();
        let av = data.vbank().get(addvn).unwrap();
        let addop = match av.get_def() {
            Some(o) => o,
            // if (addop==0) return 0;
            None => return 0,
        };
        // if (addop->code() != CPUI_INT_ADD) return 0;
        if data.obank().get(addop).unwrap().code() != OpCode::CPUI_INT_ADD {
            return 0;
        }
        // vn = addop->getIn(0); vn2 = addop->getIn(1);
        let addop_o = data.obank().get(addop).unwrap();
        let vn = addop_o.get_in(0).unwrap();
        let vn2 = addop_o.get_in(1).unwrap();

        let v2 = data.vbank().get(vn2).unwrap();
        let posvn;
        let unnegvn;
        if v2.is_constant() {
            // Address val(vn2->getSpace(),uintb_negate(vn2->getOffset()-1,vn2->getSize()));
            let vn2_size = v2.get_size();
            let vn2_off = v2.get_offset();
            let space = Rc::clone(v2.get_space());
            let val = Address::new(space, uintb_negate(vn2_off.wrapping_sub(1), vn2_size));
            // unnegvn = data.newVarnode(vn2->getSize(),val);
            let uvn = data.new_varnode(vn2_size, &val, None);
            // unnegvn->copySymbolIfValid(vn2);  // SEAM(W4): equate-symbol markup
            //   propagation (Varnode::copySymbolIfValid). The W3 IR carries no
            //   symbol entries (mapentry always null), so this is a no-op now.
            unnegvn = uvn;
            // posvn = vn;
            posvn = vn;
        } else {
            // if ((vn->isWritten())&&(vn->getDef()->code()==CPUI_INT_MULT)) { negvn=vn; posvn=vn2; }
            let vv = data.vbank().get(vn).unwrap();
            let vv_written = vv.is_written();
            let vv_def_is_mult = vv_written
                && data.obank().get(vv.get_def().unwrap()).unwrap().code() == OpCode::CPUI_INT_MULT;
            let negvn;
            if vv_def_is_mult {
                negvn = vn;
                posvn = vn2;
            } else {
                // else if ((vn2->isWritten())&&(vn2->getDef()->code()==CPUI_INT_MULT)) { negvn=vn2; posvn=vn; }
                let v2v = data.vbank().get(vn2).unwrap();
                let v2_written = v2v.is_written();
                let v2_def_is_mult = v2_written
                    && data.obank().get(v2v.get_def().unwrap()).unwrap().code()
                        == OpCode::CPUI_INT_MULT;
                if v2_def_is_mult {
                    negvn = vn2;
                    posvn = vn;
                } else {
                    // else return 0;
                    return 0;
                }
            }
            // if (!negvn->getDef()->getIn(1)->isConstant()) return 0;
            let negdef = data.vbank().get(negvn).unwrap().get_def().unwrap();
            let neg_in1 = data.obank().get(negdef).unwrap().get_in(1).unwrap();
            let neg_in1_v = data.vbank().get(neg_in1).unwrap();
            if !neg_in1_v.is_constant() {
                return 0;
            }
            // unnegvn = negvn->getDef()->getIn(0);
            let neg_in0 = data.obank().get(negdef).unwrap().get_in(0).unwrap();
            // multiplier = negvn->getDef()->getIn(1)->getOffset();
            let multiplier = data.vbank().get(neg_in1).unwrap().get_offset();
            // if (multiplier != calc_mask(unnegvn->getSize())) return 0;
            let unneg_size = data.vbank().get(neg_in0).unwrap().get_size();
            if multiplier != calc_mask(unneg_size) {
                return 0;
            }
            unnegvn = neg_in0;
        }
        // if (!posvn->isHeritageKnown()) return 0;
        if !data.vbank().get(posvn).unwrap().is_heritage_known() {
            return 0;
        }
        // if (!unnegvn->isHeritageKnown()) return 0;
        if !data.vbank().get(unnegvn).unwrap().is_heritage_known() {
            return 0;
        }
        // data.opSetInput(op,posvn,0);
        data.op_set_input(op, posvn, 0).expect("RuleEqual2Zero: opSetInput");
        // data.opSetInput(op,unnegvn,1);
        data.op_set_input(op, unnegvn, 1).expect("RuleEqual2Zero: opSetInput");
        1
    }
}

// =============================================================================
// RuleEqual2Constant (ruleaction.cc:5930)
// =============================================================================

/// \brief Simplify INT_EQUAL applied to arithmetic expressions
/// (C++ `RuleEqual2Constant`).
///
/// Forms include:
///  - `V * -1 == c  =>  V == -c`
///  - `V + c == d  =>  V == (d-c)`
///  - `~V == c     =>  V == ~c`
pub struct RuleEqual2Constant {
    group: &'static str,
}

impl RuleEqual2Constant {
    /// Constructor (C++ `RuleEqual2Constant(g) : Rule(g,0,"equal2constant")`).
    pub fn new(g: &'static str) -> RuleEqual2Constant {
        RuleEqual2Constant { group: g }
    }
}

impl Rule for RuleEqual2Constant {
    fn get_op_list(&self) -> Vec<OpCode> {
        // uint4 list[] = { CPUI_INT_EQUAL, CPUI_INT_NOTEQUAL };
        vec![OpCode::CPUI_INT_EQUAL, OpCode::CPUI_INT_NOTEQUAL]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(self.group) {
            return None;
        }
        Some(Box::new(RuleEqual2Constant { group: self.group }))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // Varnode *cvn = op->getIn(1);
        let o = data.obank().get(op).expect("RuleEqual2Constant: stale op");
        let cvn = o.get_in(1).unwrap();
        let lhs = o.get_in(0).unwrap();
        let cv = data.vbank().get(cvn).unwrap();
        // if (!cvn->isConstant()) return 0;
        if !cv.is_constant() {
            return 0;
        }
        let cvn_off = cv.get_offset();
        let cvn_size = cv.get_size();
        // Varnode *lhs = op->getIn(0); if (!lhs->isWritten()) return 0;
        let lv = data.vbank().get(lhs).unwrap();
        if !lv.is_written() {
            return 0;
        }
        let lhs_size = lv.get_size();
        // PcodeOp *leftop = lhs->getDef();
        let leftop = lv.get_def().unwrap();
        // OpCode opc = leftop->code();
        let opc = data.obank().get(leftop).unwrap().code();

        let newconst: u64;
        if opc == OpCode::CPUI_INT_ADD {
            // Varnode *otherconst = leftop->getIn(1);
            let otherconst = data.obank().get(leftop).unwrap().get_in(1).unwrap();
            let oc = data.vbank().get(otherconst).unwrap();
            // if (!otherconst->isConstant()) return 0;
            if !oc.is_constant() {
                return 0;
            }
            // newconst = cvn->getOffset() - otherconst->getOffset(); newconst &= calc_mask(cvn->getSize());
            newconst = cvn_off.wrapping_sub(oc.get_offset()) & calc_mask(cvn_size);
        } else if opc == OpCode::CPUI_INT_MULT {
            let otherconst = data.obank().get(leftop).unwrap().get_in(1).unwrap();
            let oc = data.vbank().get(otherconst).unwrap();
            if !oc.is_constant() {
                return 0;
            }
            let oc_size = oc.get_size();
            // The only multiply we transform is multiply by -1
            // if (otherconst->getOffset() != calc_mask(otherconst->getSize())) return 0;
            if oc.get_offset() != calc_mask(oc_size) {
                return 0;
            }
            // newconst = cvn->getOffset(); newconst = (-newconst) & calc_mask(otherconst->getSize());
            newconst = cvn_off.wrapping_neg() & calc_mask(oc_size);
        } else if opc == OpCode::CPUI_INT_NEGATE {
            // newconst = cvn->getOffset(); newconst = (~newconst) & calc_mask(lhs->getSize());
            newconst = (!cvn_off) & calc_mask(lhs_size);
        } else {
            // else return 0;
            return 0;
        }

        // a = leftop->getIn(0); if (a->isFree()) return 0;
        let a = data.obank().get(leftop).unwrap().get_in(0).unwrap();
        let a_size = data.vbank().get(a).unwrap().get_size();
        if data.vbank().get(a).unwrap().is_free() {
            return 0;
        }

        // Make sure the transformed form of a is only used in comparisons of similar form
        // for(iter=lhs->beginDescend();...)
        let descend = data.descend_snapshot(lhs);
        for &dop in &descend {
            // if (dop == op) continue;
            if dop == op {
                continue;
            }
            let dcode = data.obank().get(dop).unwrap().code();
            // if ((dop->code()!=CPUI_INT_EQUAL)&&(dop->code()!=CPUI_INT_NOTEQUAL)) return 0;
            if dcode != OpCode::CPUI_INT_EQUAL && dcode != OpCode::CPUI_INT_NOTEQUAL {
                return 0;
            }
            // if (!dop->getIn(1)->isConstant()) return 0;
            let dop_in1 = data.obank().get(dop).unwrap().get_in(1).unwrap();
            if !data.vbank().get(dop_in1).unwrap().is_constant() {
                return 0;
            }
        }

        // data.opSetInput(op,a,0);
        data.op_set_input(op, a, 0).expect("RuleEqual2Constant: opSetInput");
        // data.opSetInput(op,data.newConstant(a->getSize(),newconst),1);
        let c = data.new_constant(a_size, newconst);
        data.op_set_input(op, c, 1).expect("RuleEqual2Constant: opSetInput");
        1
    }
}

// =============================================================================
// RulePtrArith (ruleaction.cc:6639)   --- SEAM-stubbed (W6 + W3-varnode)
// =============================================================================

/// \brief Transform pointer arithmetic: convert a string of INT_ADDs into
/// PTRADDs and PTRSUBs (C++ `RulePtrArith`).
///
/// SEAM(W6)/SEAM(W3-varnode): the `applyOp` body drives `AddTreeState`, which
/// reads `Varnode::getTypeReadFacing(op)` and `TypePointer`/`TypePointerRel`
/// internals (W6, unported) and rewrites the tree with `newOpBefore`/
/// `newUniqueOut`/`newVarnodeOut`/`opSetOutput` and
/// `distributeIntMultAdd`/`collapseIntMultMult`/`inheritUnionField`/
/// `forceFacingType` (W3-varnode op-creation surfaces that are themselves
/// deferred — `Funcdata::op_set_output` is an `Err` stub in the merged IR).
/// Ported structurally (name/getOpList/specs); the transform is a no-op stub.
pub struct RulePtrArith {
    group: &'static str,
}

impl RulePtrArith {
    /// Constructor (C++ `RulePtrArith(g) : Rule(g,0,"ptrarith")`).
    pub fn new(g: &'static str) -> RulePtrArith {
        RulePtrArith { group: g }
    }
}

impl Rule for RulePtrArith {
    fn get_op_list(&self) -> Vec<OpCode> {
        // oplist.push_back(CPUI_INT_ADD);
        vec![OpCode::CPUI_INT_ADD]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(self.group) {
            return None;
        }
        Some(Box::new(RulePtrArith { group: self.group }))
    }

    fn apply_op(&mut self, _op: OpId, _data: &mut Funcdata) -> int4 {
        // SEAM(W6)+SEAM(W3-varnode): AddTreeState + getTypeReadFacing + newOpBefore.
        // The full C++ body:
        //   if (!data.hasTypeRecoveryStarted()) return 0;
        //   for(slot=0;...) ct = op->getIn(slot)->getTypeReadFacing(op);
        //                   if (ct->getMetatype() == TYPE_PTR) break;
        //   if (slot == op->numInput()) return 0;
        //   if (evaluatePointerExpression(op, slot) != 2) return 0;
        //   if (!verifyPreferredPointer(op, slot)) return 0;
        //   AddTreeState state(data,op,slot);
        //   if (state.apply()) return 1;
        //   if (state.initAlternateForm()) { if (state.apply()) return 1; }
        //   return 0;
        0
    }
}

// =============================================================================
// RuleStructOffset0 (ruleaction.cc:6688)   --- SEAM-stubbed (W6 + W3-varnode)
// =============================================================================

/// \brief Convert a LOAD or STORE to the first element of a structure to a
/// PTRSUB (C++ `RuleStructOffset0`).
///
/// SEAM(W6)/SEAM(W3-varnode): the body reads `Varnode::getTypeReadFacing(op)`,
/// `TypePointer::getPtrTo`, `TypePointerRel::evaluateThruParent`/`getByteOffset`,
/// `Datatype::getSubType`, `TypeArray::numElements` (W6, unported) and builds a
/// PTRSUB with `newOpBefore` + `inheritUnionField` + `setStopTypePropagation`
/// (W3-varnode op-creation, deferred).  Ported structurally; transform is a
/// no-op stub.
pub struct RuleStructOffset0 {
    group: &'static str,
}

impl RuleStructOffset0 {
    /// Constructor (C++ `RuleStructOffset0(g) : Rule(g,0,"structoffset0")`).
    pub fn new(g: &'static str) -> RuleStructOffset0 {
        RuleStructOffset0 { group: g }
    }
}

impl Rule for RuleStructOffset0 {
    fn get_op_list(&self) -> Vec<OpCode> {
        // uint4 list[]={ CPUI_LOAD, CPUI_STORE };
        vec![OpCode::CPUI_LOAD, OpCode::CPUI_STORE]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(self.group) {
            return None;
        }
        Some(Box::new(RuleStructOffset0 { group: self.group }))
    }

    fn apply_op(&mut self, _op: OpId, _data: &mut Funcdata) -> int4 {
        // SEAM(W6)+SEAM(W3-varnode): getTypeReadFacing + TypePointer(Rel) + newOpBefore.
        0
    }
}

// =============================================================================
// RulePushPtr (ruleaction.cc:6786)   --- SEAM-stubbed (W6 + W3-varnode)
// =============================================================================

/// \brief Push a Varnode with known pointer data-type to the bottom of its
/// additive expression (C++ `RulePushPtr`).
///
/// SEAM(W6)/SEAM(W3-varnode): `applyOp` reads `Varnode::getTypeReadFacing(op)`
/// (W6, unported) and rewrites the tree with `newOp`/`newUniqueOut`/
/// `newVarnodeOut`/`opInsertBefore`/`opDestroy` plus the static helpers
/// `buildVarnodeOut`/`collectDuplicateNeeds`/`duplicateNeed` (W3-varnode
/// op-creation with outputs, deferred).  It also calls the in-range static
/// `RulePtrArith::evaluatePointerExpression`, which is itself a W6 seam.  Ported
/// structurally; transform is a no-op stub.
pub struct RulePushPtr {
    group: &'static str,
}

impl RulePushPtr {
    /// Constructor (C++ `RulePushPtr(g) : Rule(g,0,"pushptr")`).
    pub fn new(g: &'static str) -> RulePushPtr {
        RulePushPtr { group: g }
    }
}

impl Rule for RulePushPtr {
    fn get_op_list(&self) -> Vec<OpCode> {
        // oplist.push_back(CPUI_INT_ADD);
        vec![OpCode::CPUI_INT_ADD]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(self.group) {
            return None;
        }
        Some(Box::new(RulePushPtr { group: self.group }))
    }

    fn apply_op(&mut self, _op: OpId, _data: &mut Funcdata) -> int4 {
        // SEAM(W6)+SEAM(W3-varnode): getTypeReadFacing + evaluatePointerExpression
        // + newOp/newUniqueOut/buildVarnodeOut/duplicateNeed.
        0
    }
}

// =============================================================================
// Registration (W8 universalAction assembly)
// =============================================================================

/// Per-file [`RuleSpec`] rows for the nine rules in this batch, in C++
/// definition order.
///
/// The `group` strings are the ones the rules are constructed with in
/// `coreaction.cc::universalAction`: the first six are `"analysis"`, the last
/// three (the pointer-arithmetic rules) are `"typerecovery"`.  Each spec's
/// `ctor` builds the rule with that same group so its `clone_rule` filters
/// correctly.
///
/// NOTE: definition order (this file's order) is preserved here; the *exact*
/// registration order within `universalAction` (where `RuleBoolNegate`
/// registers after the comparison rules, and the pointer rules register in a
/// different pool) is the W8 builder's concern — W8 reorders/reassembles using
/// these named, grouped specs.
pub fn specs() -> Vec<RuleSpec> {
    vec![
        RuleSpec { group: "analysis", ctor: || Box::new(RuleBoolNegate::new("analysis")) },
        RuleSpec { group: "analysis", ctor: || Box::new(RuleLess2Zero::new("analysis")) },
        RuleSpec { group: "analysis", ctor: || Box::new(RuleLessEqual2Zero::new("analysis")) },
        RuleSpec { group: "analysis", ctor: || Box::new(RuleSLess2Zero::new("analysis")) },
        RuleSpec { group: "analysis", ctor: || Box::new(RuleEqual2Zero::new("analysis")) },
        RuleSpec { group: "analysis", ctor: || Box::new(RuleEqual2Constant::new("analysis")) },
        RuleSpec { group: "typerecovery", ctor: || Box::new(RulePtrArith::new("typerecovery")) },
        RuleSpec {
            group: "typerecovery",
            ctor: || Box::new(RuleStructOffset0::new("typerecovery")),
        },
        RuleSpec { group: "typerecovery", ctor: || Box::new(RulePushPtr::new("typerecovery")) },
    ]
}

#[cfg(test)]
mod tests;
