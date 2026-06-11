//! Port of `decompiler/cpp/expression.{cc,hh}` — the expression-matching and
//! term-ordering machinery the simplification rules lean on (W5).
//!
//! # Scope of this item (`w5-s3-condexe-expression`)
//!
//! The task pins this module to the **equality/term-order machinery**:
//!
//!   - [`PcodeOpNode`] / [`TraverseNode`] — the lightweight data-flow edge/node
//!     structs (`expression.hh`).
//!   - [`BooleanMatch`] / [`BooleanExpressionMatch`] — the static
//!     same-vs-complementary boolean-expression evaluator and the CBRANCH
//!     correlation helper (the upstream `ConditionMarker`/`ConditionalJoin` the
//!     task names; in this Ghidra rev they were folded into
//!     `BooleanMatch`/`BooleanExpressionMatch`).  Used by
//!     [`crate::condexe::ConditionalExecution::verify_same_condition`] and
//!     [`crate::condexe::RuleOrPredicate`].
//!   - [`AdditiveEdge`] / [`TermOrder`] — the additive-term collector and the
//!     **exact** `additiveCompare` comparator used by `RuleTermOrder`.
//!   - [`functional_equality_level`] / [`functional_equality`] /
//!     [`functional_difference`] — the Varnode value-equivalence tests.
//!   - [`root_pointer`] / [`pointer_equality`] — pointer back-tracking equality.
//!
//! The bitfield-expression family (`BitFieldExpression`, `InsertExpression`,
//! `InsertStoreExpression`, `PullExpression`) and `AddExpression` from
//! `expression.{cc,hh}` are **out of scope** for this item: they are consumed by
//! later rule items, dereference the W6 `Datatype`/`TypeStruct`/`TypeBitField`
//! and W4 `Symbol` surfaces, and the task scopes this item to "equality/term-
//! order machinery".  They are NOT transcribed here; recorded as a loss so the
//! ledger is visible.  // SEAM(W6) — see losses.
//!
//! # API-surface adaptation (the "method needs the bank" pattern)
//!
//! The C++ `Varnode *`/`PcodeOp *` graph is, per ADR 0001, a `Funcdata`-owned
//! pair of slotmap arenas reached through [`VarnodeBank`]/[`PcodeOpBank`].  Every
//! read here threads the banks explicitly, exactly as `op.rs` does for
//! `getCseHash`/`isCollapsible`.  `Varnode::termOrder` (`varnode.cc:1172`) is not
//! yet ported on `varnode.rs` (W3) and this item does not own that file, so the
//! comparator's term-order primitive lives here as the free fn [`term_order`]
//! reading through the banks; it hoists to `Varnode::termOrder` when varnode.rs
//! ports it.  // SEAM(W3-varnode) — see losses.

use kuna_base::address::signbit_negative;
use kuna_base::types::{int4, uintb};
use kuna_num::opcodes::{get_booleanflip, OpCode};

use crate::op::PcodeOpBank;
use crate::seams::{OpId, VarnodeId};
use crate::varnode::VarnodeBank;

// =============================================================================
// PcodeOpNode (expression.hh:25-58)
// =============================================================================

/// An edge in a data-flow path or graph (C++ `PcodeOpNode`).
///
/// A minimal node for traversing expressions in the data-flow: the p-code
/// end-point [`op`](PcodeOpNode::op) and the [`slot`](PcodeOpNode::slot)
/// indicating the input Varnode end-point of the edge.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PcodeOpNode {
    /// The p-code end-point of the edge (C++ `PcodeOp *op`).
    pub op: OpId,
    /// Slot indicating the input Varnode end-point of the edge (C++ `int4 slot`).
    pub slot: int4,
}

impl PcodeOpNode {
    /// Constructor (C++ `PcodeOpNode(PcodeOp *o,int4 s)`).
    pub fn new(o: OpId, s: int4) -> PcodeOpNode {
        PcodeOpNode { op: o, slot: s }
    }

    /// Simple comparator for putting edges in a sorted container (C++
    /// `PcodeOpNode::operator<`, `expression.hh:40-48`).
    ///
    /// Compares ops first by their `SeqNum::getTime()`, then by slot.  The C++
    /// compares the `PcodeOp *` for inequality then keys the order on
    /// `getSeqNum().getTime()`; the Rust compares the [`OpId`]s for inequality
    /// (arena-key equality is pointer-identity) and keys order identically.
    pub fn less(&self, op2: &PcodeOpNode, obank: &PcodeOpBank) -> bool {
        if self.op != op2.op {
            let t1 = obank.get(self.op).expect("PcodeOpNode::less: stale op").get_seq_num().get_time();
            let t2 = obank.get(op2.op).expect("PcodeOpNode::less: stale op").get_seq_num().get_time();
            return t1 < t2;
        }
        if self.slot != op2.slot {
            return self.slot < op2.slot;
        }
        false
    }

    /// Compare Varnodes by their HighVariable (C++ `PcodeOpNode::compareByHigh`,
    /// `expression.hh:54-58`): `a.op->getIn(a.slot)->getHigh() <
    /// b.op->getIn(b.slot)->getHigh()`.
    ///
    /// SEAM(W7): `Varnode::getHigh` returns an `Option<HighVariableId>` that is
    /// `None` until merging builds the HighVariables (the C++ `getHigh()`
    /// likewise throws until then).  The `HighVariable *` pointer-order the C++
    /// compares is modelled by the `HighVariableId` `Ord`; until W7 every high is
    /// `None`, so the comparison degrades to `None < None == false` (no order)
    /// rather than the W7 pointer order.  Carried for layout fidelity; the only
    /// consumer (`PcodeOpNode::compareByHigh`) is a W7-era sort.
    pub fn compare_by_high(a: &PcodeOpNode, b: &PcodeOpNode, obank: &PcodeOpBank, vbank: &VarnodeBank) -> bool {
        let avn = obank.get(a.op).expect("compare_by_high: stale op").get_in(a.slot).expect("compare_by_high: null in");
        let bvn = obank.get(b.op).expect("compare_by_high: stale op").get_in(b.slot).expect("compare_by_high: null in");
        let ahigh = vbank.get(avn).expect("compare_by_high: stale vn").get_high();
        let bhigh = vbank.get(bvn).expect("compare_by_high: stale vn").get_high();
        ahigh < bhigh // SEAM(W7): HighVariable order
    }
}

// =============================================================================
// TraverseNode (expression.hh:60-73)
// =============================================================================

/// Bit flags for a [`TraverseNode`] (C++ anonymous `enum` in `TraverseNode`).
#[allow(non_upper_case_globals)]
pub mod traverse_flags {
    use kuna_base::types::uint4;
    /// Alternate path traverses a solid action or \e non-incidental COPY.
    pub const actionalt: uint4 = 1;
    /// Main path traverses an INDIRECT.
    pub const indirect: uint4 = 2;
    /// Alternate path traverses an INDIRECT.
    pub const indirectalt: uint4 = 4;
    /// Least significant byte(s) of original value have been truncated.
    pub const lsb_truncated: uint4 = 8;
    /// Original value has been concatented as \e most significant portion.
    pub const concat_high: uint4 = 0x10;
}

/// Node for a forward traversal of a Varnode expression (C++ `TraverseNode`).
#[derive(Debug, Clone, Copy)]
pub struct TraverseNode {
    /// Varnode at the point of traversal (C++ `const Varnode *vn`).
    pub vn: VarnodeId,
    /// Flags associated with the node (C++ `uint4 flags`).
    pub flags: kuna_base::types::uint4,
}

impl TraverseNode {
    /// Constructor (C++ `TraverseNode(const Varnode *v,uint4 f)`).
    pub fn new(v: VarnodeId, f: kuna_base::types::uint4) -> TraverseNode {
        TraverseNode { vn: v, flags: f }
    }

    /// Return \b true if the alternate path looks more valid than the main path
    /// (C++ `TraverseNode::isAlternatePathValid`, `expression.cc:29-49`).
    pub fn is_alternate_path_valid(vn: VarnodeId, flags: kuna_base::types::uint4, vbank: &VarnodeBank, obank: &PcodeOpBank) -> bool {
        use traverse_flags::*;
        // if ((flags & (indirect | indirectalt)) == indirect) return true;
        if (flags & (indirect | indirectalt)) == indirect {
            return true; // Main path traversed INDIRECT, alternate did not
        }
        // if ((flags & (indirect | indirectalt)) == indirectalt) return false;
        if (flags & (indirect | indirectalt)) == indirectalt {
            return false; // Alternate path traversed INDIRECT, main did not
        }
        // if ((flags & actionalt) != 0) return true;
        if (flags & actionalt) != 0 {
            return true; // Alternate path traversed a dedicated COPY
        }
        // if (vn->loneDescend() == 0) return false;
        let mut vn = vn;
        if vn_lone_descend(vn, vbank) == 0 {
            return false;
        }
        // const PcodeOp *op = vn->getDef(); if (op == 0) return true;
        let mut op = match vbank.get(vn).expect("isAlternatePathValid: stale vn").get_def() {
            Some(o) => o,
            None => return true,
        };
        // while(op->isIncidentalCopy() && op->code() == CPUI_COPY) { ... }
        while {
            let o = obank.get(op).expect("isAlternatePathValid: stale op");
            o.is_incidental_copy() && o.code() == OpCode::CPUI_COPY
        } {
            // vn = op->getIn(0);
            vn = obank.get(op).expect("isAlternatePathValid").get_in(0).expect("isAlternatePathValid: null in");
            // if (vn->loneDescend() == 0) return false;
            if vn_lone_descend(vn, vbank) == 0 {
                return false;
            }
            // op = vn->getDef(); if (op == 0) return true;
            op = match vbank.get(vn).expect("isAlternatePathValid: stale vn").get_def() {
                Some(o) => o,
                None => return true,
            };
        }
        // return !op->isMarker();
        !obank.get(op).expect("isAlternatePathValid").is_marker()
    }
}

// =============================================================================
// BooleanMatch (expression.hh:80-90, expression.cc:51-217)
// =============================================================================

/// Correlation classes returned by [`BooleanMatch::evaluate`] (C++ `BooleanMatch`
/// anonymous `enum`).
#[allow(non_upper_case_globals)]
pub mod boolean_match {
    use kuna_base::types::int4;
    /// Pair always hold the same value.
    pub const same: int4 = 1;
    /// Pair always hold complementary values.
    pub const complementary: int4 = 2;
    /// Pair values are uncorrelated.
    pub const uncorrelated: int4 = 3;
}

/// Static methods for determining if two boolean expressions are the \b same or
/// \b complementary (C++ `BooleanMatch`).
pub struct BooleanMatch;

impl BooleanMatch {
    /// Test if two operations with same opcode produce complementary boolean
    /// values (C++ `BooleanMatch::sameOpComplement`, `expression.cc:58-87`).
    ///
    /// Only tests INT_LESS / INT_SLESS with one constant input (the `x < 9`
    /// vs `8 < x` scenario).
    fn same_op_complement(bin1op: OpId, bin2op: OpId, vbank: &VarnodeBank, obank: &PcodeOpBank) -> bool {
        let b1 = obank.get(bin1op).expect("sameOpComplement: stale op");
        let opcode = b1.code();
        if opcode == OpCode::CPUI_INT_SLESS || opcode == OpCode::CPUI_INT_LESS {
            // Basically we test for the scenario like:  x < 9   8 < x
            // int4 constslot = 0; if (bin1op->getIn(1)->isConstant()) constslot = 1;
            let in1_1 = b1.get_in(1).expect("sameOpComplement: null in");
            let mut constslot: int4 = 0;
            if vbank.get(in1_1).expect("sameOpComplement: stale vn").is_constant() {
                constslot = 1;
            }
            // if (!bin1op->getIn(constslot)->isConstant()) return false;
            let b1_constslot = b1.get_in(constslot).expect("sameOpComplement: null in");
            if !vbank.get(b1_constslot).expect("sameOpComplement: stale vn").is_constant() {
                return false;
            }
            // if (!bin2op->getIn(1-constslot)->isConstant()) return false;
            let b2 = obank.get(bin2op).expect("sameOpComplement: stale op");
            let b2_1mcs = b2.get_in(1 - constslot).expect("sameOpComplement: null in");
            if !vbank.get(b2_1mcs).expect("sameOpComplement: stale vn").is_constant() {
                return false;
            }
            // if (!varnodeSame(bin1op->getIn(1-constslot),bin2op->getIn(constslot))) return false;
            let b1_1mcs = b1.get_in(1 - constslot).expect("sameOpComplement: null in");
            let b2_constslot = b2.get_in(constslot).expect("sameOpComplement: null in");
            if !Self::varnode_same(b1_1mcs, b2_constslot, vbank) {
                return false;
            }
            // uintb val1 = bin1op->getIn(constslot)->getOffset();
            // uintb val2 = bin2op->getIn(1-constslot)->getOffset();
            let mut val1: uintb = vbank.get(b1_constslot).expect("sameOpComplement: stale vn").get_offset();
            let mut val2: uintb = vbank.get(b2_1mcs).expect("sameOpComplement: stale vn").get_offset();
            // if (constslot!=0) { swap val1, val2 }
            if constslot != 0 {
                std::mem::swap(&mut val1, &mut val2);
            }
            // if (val1 + 1 != val2) return false;  (wrapping add, matching C++ uintb)
            if val1.wrapping_add(1) != val2 {
                return false;
            }
            // if ((val2 == 0)&&(opcode==CPUI_INT_LESS)) return false; // Corner case for unsigned
            if val2 == 0 && opcode == OpCode::CPUI_INT_LESS {
                return false;
            }
            if opcode == OpCode::CPUI_INT_SLESS {
                // Corner case for signed
                // int4 sz = bin1op->getIn(constslot)->getSize();
                let sz = vbank.get(b1_constslot).expect("sameOpComplement: stale vn").get_size();
                // if (signbit_negative(val2,sz) && (!signbit_negative(val1,sz))) return false;
                if signbit_negative(val2, sz) && !signbit_negative(val1, sz) {
                    return false;
                }
            }
            return true;
        }
        false
    }

    /// Do the given Varnodes hold the same value, possibly as constants (C++
    /// `BooleanMatch::varnodeSame`, `expression.cc:94-101`).
    fn varnode_same(a: VarnodeId, b: VarnodeId, vbank: &VarnodeBank) -> bool {
        if a == b {
            return true;
        }
        let va = vbank.get(a).expect("varnodeSame: stale vn");
        let vb = vbank.get(b).expect("varnodeSame: stale vn");
        if va.is_constant() && vb.is_constant() {
            return va.get_offset() == vb.get_offset();
        }
        false
    }

    /// Determine if two boolean Varnodes hold related values (same, complementary,
    /// or uncorrelated) (C++ `BooleanMatch::evaluate`, `expression.cc:112-217`).
    ///
    /// The trees constructing each Varnode are examined up to `depth`; exceeding
    /// it returns [`uncorrelated`](boolean_match::uncorrelated).
    // The three-level nested `if` (depth/opc1 → opc2 → opc1-opc2-mix) is the C++
    // structure verbatim; collapsing it would obscure the distinct guards and
    // their comments, and this is rule-output-affecting code.
    #[allow(clippy::collapsible_if)]
    pub fn evaluate(vn1: VarnodeId, vn2: VarnodeId, depth: int4, vbank: &VarnodeBank, obank: &PcodeOpBank) -> int4 {
        use boolean_match::*;
        // if (vn1 == vn2) return same;
        if vn1 == vn2 {
            return same;
        }
        let op1: Option<OpId>;
        let opc1: OpCode;
        // if (vn1->isWritten()) { op1 = vn1->getDef(); opc1 = op1->code(); ... }
        if vbank.get(vn1).expect("evaluate: stale vn").is_written() {
            let o1 = vbank.get(vn1).expect("evaluate").get_def().expect("evaluate: written but no def");
            op1 = Some(o1);
            opc1 = obank.get(o1).expect("evaluate: stale op").code();
            if opc1 == OpCode::CPUI_BOOL_NEGATE {
                // int res = evaluate(op1->getIn(0),vn2,depth);
                let in0 = obank.get(o1).expect("evaluate").get_in(0).expect("evaluate: null in");
                let mut res = Self::evaluate(in0, vn2, depth, vbank, obank);
                // Flip same <-> complementary result
                if res == same {
                    res = complementary;
                } else if res == complementary {
                    res = same;
                }
                return res;
            }
        } else {
            // Don't give up before checking if op2 is BOOL_NEGATE
            op1 = None;
            opc1 = OpCode::CPUI_MAX;
        }
        let op2: OpId;
        let opc2: OpCode;
        // if (vn2->isWritten()) { op2 = vn2->getDef(); opc2 = op2->code(); ... }
        if vbank.get(vn2).expect("evaluate: stale vn").is_written() {
            let o2 = vbank.get(vn2).expect("evaluate").get_def().expect("evaluate: written but no def");
            op2 = o2;
            opc2 = obank.get(o2).expect("evaluate: stale op").code();
            if opc2 == OpCode::CPUI_BOOL_NEGATE {
                // int4 res = evaluate(vn1,op2->getIn(0),depth);
                let in0 = obank.get(o2).expect("evaluate").get_in(0).expect("evaluate: null in");
                let mut res = Self::evaluate(vn1, in0, depth, vbank, obank);
                if res == same {
                    res = complementary;
                } else if res == complementary {
                    res = same;
                }
                return res;
            }
        } else {
            return uncorrelated;
        }
        // if (op1 == 0) return uncorrelated;
        let op1 = match op1 {
            Some(o) => o,
            None => return uncorrelated,
        };
        // if (!op1->isBoolOutput() || !op2->isBoolOutput()) return uncorrelated;
        if !obank.get(op1).expect("evaluate").is_bool_output() || !obank.get(op2).expect("evaluate").is_bool_output() {
            return uncorrelated;
        }
        if depth != 0
            && (opc1 == OpCode::CPUI_BOOL_AND || opc1 == OpCode::CPUI_BOOL_OR || opc1 == OpCode::CPUI_BOOL_XOR)
        {
            if opc2 == OpCode::CPUI_BOOL_AND || opc2 == OpCode::CPUI_BOOL_OR || opc2 == OpCode::CPUI_BOOL_XOR {
                if opc1 == opc2
                    || (opc1 == OpCode::CPUI_BOOL_AND && opc2 == OpCode::CPUI_BOOL_OR)
                    || (opc1 == OpCode::CPUI_BOOL_OR && opc2 == OpCode::CPUI_BOOL_AND)
                {
                    let o1in0 = obank.get(op1).expect("evaluate").get_in(0).expect("evaluate: null in");
                    let o1in1 = obank.get(op1).expect("evaluate").get_in(1).expect("evaluate: null in");
                    let o2in0 = obank.get(op2).expect("evaluate").get_in(0).expect("evaluate: null in");
                    let o2in1 = obank.get(op2).expect("evaluate").get_in(1).expect("evaluate: null in");
                    // int4 pair1 = evaluate(op1->getIn(0),op2->getIn(0),depth-1);
                    let mut pair1 = Self::evaluate(o1in0, o2in0, depth - 1, vbank, obank);
                    let pair2;
                    if pair1 == uncorrelated {
                        // Try other possible pairing (commutative op)
                        pair1 = Self::evaluate(o1in0, o2in1, depth - 1, vbank, obank);
                        if pair1 == uncorrelated {
                            return uncorrelated;
                        }
                        pair2 = Self::evaluate(o1in1, o2in0, depth - 1, vbank, obank);
                    } else {
                        pair2 = Self::evaluate(o1in1, o2in1, depth - 1, vbank, obank);
                    }
                    if pair2 == uncorrelated {
                        return uncorrelated;
                    }
                    if opc1 == opc2 {
                        if pair1 == same && pair2 == same {
                            return same;
                        } else if opc1 == OpCode::CPUI_BOOL_XOR {
                            if pair1 == complementary && pair2 == complementary {
                                return same;
                            }
                            return complementary;
                        }
                    } else {
                        // Must be CPUI_BOOL_AND and CPUI_BOOL_OR
                        if pair1 == complementary && pair2 == complementary {
                            return complementary; // De Morgan's Law
                        }
                    }
                }
            }
        } else {
            // Two boolean output ops, compare them directly
            if opc1 == opc2 {
                // bool sameOp = true; for (i in 0..numInputs) if !varnodeSame(...) sameOp=false,break;
                let mut same_op = true;
                let num_inputs = obank.get(op1).expect("evaluate").num_input();
                for i in 0..num_inputs {
                    let a = obank.get(op1).expect("evaluate").get_in(i).expect("evaluate: null in");
                    let b = obank.get(op2).expect("evaluate").get_in(i).expect("evaluate: null in");
                    if !Self::varnode_same(a, b, vbank) {
                        same_op = false;
                        break;
                    }
                }
                if same_op {
                    return same;
                }
                if Self::same_op_complement(op1, op2, vbank, obank) {
                    return complementary;
                }
                return uncorrelated;
            }
            // Check if the binary ops are complements of one another
            let slot1: int4 = 0;
            let mut slot2: int4 = 0;
            let mut reorder = false;
            // if (opc1 != get_booleanflip(opc2,reorder)) return uncorrelated;
            if opc1 != get_booleanflip(opc2, &mut reorder) {
                return uncorrelated;
            }
            if reorder {
                slot2 = 1;
            }
            let o1_slot1 = obank.get(op1).expect("evaluate").get_in(slot1).expect("evaluate: null in");
            let o2_slot2 = obank.get(op2).expect("evaluate").get_in(slot2).expect("evaluate: null in");
            if !Self::varnode_same(o1_slot1, o2_slot2, vbank) {
                return uncorrelated;
            }
            let o1_1mslot1 = obank.get(op1).expect("evaluate").get_in(1 - slot1).expect("evaluate: null in");
            let o2_1mslot2 = obank.get(op2).expect("evaluate").get_in(1 - slot2).expect("evaluate: null in");
            if !Self::varnode_same(o1_1mslot1, o2_1mslot2, vbank) {
                return uncorrelated;
            }
            return complementary;
        }
        uncorrelated
    }
}

// =============================================================================
// BooleanExpressionMatch (expression.hh:96-103, expression.cc:219-233)
// =============================================================================

/// Helper describing the similarity of the boolean condition between 2 CBRANCH
/// operations (C++ `BooleanExpressionMatch`).
///
/// Determines if two CBRANCHs share the same condition, whether the conditions
/// are complements, and (via [`get_multi_slot`](BooleanExpressionMatch::get_multi_slot))
/// whether they are shared along only one path.
#[derive(Debug, Clone, Copy, Default)]
pub struct BooleanExpressionMatch {
    /// True if the compared CBRANCH keys on the opposite boolean value of the
    /// root (C++ `bool matchflip`).
    matchflip: bool,
}

impl BooleanExpressionMatch {
    /// Maximum depth to trace a boolean expression (C++
    /// `BooleanExpressionMatch::maxDepth = 1`).
    pub const MAX_DEPTH: int4 = 1;

    /// Construct an unset matcher.
    pub fn new() -> BooleanExpressionMatch {
        BooleanExpressionMatch { matchflip: false }
    }

    /// Perform the correlation test on two CBRANCH operations (C++
    /// `BooleanExpressionMatch::verifyCondition`, `expression.cc:221-233`).
    pub fn verify_condition(&mut self, op: OpId, iop: OpId, vbank: &VarnodeBank, obank: &PcodeOpBank) -> bool {
        // int4 res = BooleanMatch::evaluate(op->getIn(1), iop->getIn(1), maxDepth);
        let op_in1 = obank.get(op).expect("verifyCondition: stale op").get_in(1).expect("verifyCondition: null in");
        let iop_in1 = obank.get(iop).expect("verifyCondition: stale op").get_in(1).expect("verifyCondition: null in");
        let res = BooleanMatch::evaluate(op_in1, iop_in1, Self::MAX_DEPTH, vbank, obank);
        if res == boolean_match::uncorrelated {
            return false;
        }
        // matchflip = (res == complementary);
        self.matchflip = res == boolean_match::complementary;
        // if (op->isBooleanFlip()) matchflip = !matchflip;
        if obank.get(op).expect("verifyCondition").is_boolean_flip() {
            self.matchflip = !self.matchflip;
        }
        // if (iop->isBooleanFlip()) matchflip = !matchflip;
        if obank.get(iop).expect("verifyCondition").is_boolean_flip() {
            self.matchflip = !self.matchflip;
        }
        true
    }

    /// Get the MULTIEQUAL slot in the critical path (C++
    /// `BooleanExpressionMatch::getMultiSlot`, always returns -1 in this rev).
    pub fn get_multi_slot(&self) -> int4 {
        -1
    }

    /// Return \b true if the expressions are anti-correlated (C++
    /// `BooleanExpressionMatch::getFlip`).
    pub fn get_flip(&self) -> bool {
        self.matchflip
    }
}

// =============================================================================
// AdditiveEdge / TermOrder (expression.hh:106-146, expression.cc:235-294)
// =============================================================================

/// A \e term in an additive expression (C++ `AdditiveEdge`).
#[derive(Debug, Clone, Copy)]
pub struct AdditiveEdge {
    /// Lone descendant reading the term (C++ `PcodeOp *op`).
    op: OpId,
    /// The input slot of the term (C++ `int4 slot`).
    slot: int4,
    /// The term Varnode (C++ `Varnode *vn`, `= op->getIn(slot)`).
    vn: VarnodeId,
    /// The (optional) multiplier being applied to the term (C++ `PcodeOp *mult`).
    mult: Option<OpId>,
}

impl AdditiveEdge {
    /// Constructor (C++ `AdditiveEdge(PcodeOp *o,int4 s,PcodeOp *m)`): `vn` is
    /// resolved as `op->getIn(slot)` through the op bank.
    pub fn new(o: OpId, s: int4, m: Option<OpId>, obank: &PcodeOpBank) -> AdditiveEdge {
        let vn = obank.get(o).expect("AdditiveEdge::new: stale op").get_in(s).expect("AdditiveEdge::new: null in");
        AdditiveEdge { op: o, slot: s, vn, mult: m }
    }

    /// Get the multiplier PcodeOp (C++ `getMultiplier`).
    pub fn get_multiplier(&self) -> Option<OpId> {
        self.mult
    }
    /// Get the component PcodeOp adding in the term (C++ `getOp`).
    pub fn get_op(&self) -> OpId {
        self.op
    }
    /// Get the slot reading the term (C++ `getSlot`).
    pub fn get_slot(&self) -> int4 {
        self.slot
    }
    /// Get the Varnode term (C++ `getVarnode`).
    pub fn get_varnode(&self) -> VarnodeId {
        self.vn
    }
}

/// A class for ordering Varnode terms in an additive expression (C++
/// `TermOrder`).
///
/// Given the final PcodeOp in a data-flow expression that sums 2 or more Varnode
/// \e terms, collects all the terms and sorts them to facilitate constant
/// collapse and factoring simplifications.
pub struct TermOrder {
    /// The final PcodeOp in the expression (C++ `PcodeOp *root`).
    root: OpId,
    /// Collected terms (C++ `vector<AdditiveEdge> terms`).
    terms: Vec<AdditiveEdge>,
    /// An array of indices into `terms` for quick sorting (C++ `vector<AdditiveEdge*> sorter`).
    ///
    /// The C++ holds `AdditiveEdge *` aliases into `terms`; the Rust holds the
    /// equivalent indices (arena-/vector-safe), preserving the same identity and
    /// the same sort.
    sorter: Vec<usize>,
}

impl TermOrder {
    /// Construct given root PcodeOp (C++ `TermOrder(PcodeOp *rt)`).
    pub fn new(rt: OpId) -> TermOrder {
        TermOrder { root: rt, terms: Vec::new(), sorter: Vec::new() }
    }

    /// Get the number of terms in the expression (C++ `getSize`).
    pub fn get_size(&self) -> int4 {
        self.terms.len() as int4
    }

    /// Collect all the terms in the expression (C++ `TermOrder::collect`,
    /// `expression.cc:237-284`).
    ///
    /// Depth-first traversal from `root` over `CPUI_INT_ADD` chains; a sub-tree
    /// rooted at an `INT_MULT` by a constant is descended through, carrying the
    /// multiplier.  A node with more than one use (or not written, or not an
    /// `INT_ADD`) becomes a leaf term.
    pub fn collect(&mut self, fd: &crate::funcdata::Funcdata) {
        let obank = fd.obank();
        let mut opstack: Vec<OpId> = Vec::new(); // Depth first traversal path
        let mut multstack: Vec<Option<OpId>> = Vec::new();

        opstack.push(self.root);
        multstack.push(None);

        // C++ `while(!opstack.empty())` over the two lock-step stacks.
        while let Some(curop) = opstack.pop() {
            let multop = multstack.pop().expect("collect: stacks pop in lock-step");
            let numinput = obank.get(curop).expect("collect: stale op").num_input();
            for i in 0..numinput {
                // curvn is a node of the subtree IF
                let curvn = obank.get(curop).expect("collect").get_in(i).expect("collect: null in");
                // if (!curvn->isWritten()) { terms.emplace_back(curop,i,multop); continue; }
                if !fd.vbank().get(curvn).expect("collect: stale vn").is_written() {
                    self.terms.push(AdditiveEdge::new(curop, i, multop, obank));
                    continue;
                }
                // if (curvn->loneDescend() == 0) { terms.emplace_back(curop,i,multop); continue; }
                if fd.lone_descend(curvn).is_none() {
                    self.terms.push(AdditiveEdge::new(curop, i, multop, obank));
                    continue;
                }
                // subop = curvn->getDef();
                let subop = fd.vbank().get(curvn).expect("collect").get_def().expect("collect: written but no def");
                // if (subop->code() != CPUI_INT_ADD) { ... }
                if obank.get(subop).expect("collect: stale op").code() != OpCode::CPUI_INT_ADD {
                    // if ((subop->code()==CPUI_INT_MULT)&&(subop->getIn(1)->isConstant())) { ... }
                    let subop_ref = obank.get(subop).expect("collect");
                    if subop_ref.code() == OpCode::CPUI_INT_MULT {
                        let subin1 = subop_ref.get_in(1).expect("collect: null in");
                        if fd.vbank().get(subin1).expect("collect: stale vn").is_constant() {
                            // PcodeOp *addop = subop->getIn(0)->getDef();
                            let subin0 = subop_ref.get_in(0).expect("collect: null in");
                            let addop = fd.vbank().get(subin0).expect("collect: stale vn").get_def();
                            // if ((addop!=0)&&(addop->code()==CPUI_INT_ADD)) { ... }
                            if let Some(addop) = addop {
                                if obank.get(addop).expect("collect: stale op").code() == OpCode::CPUI_INT_ADD {
                                    // if (addop->getOut()->loneDescend()!=0) { push; continue; }
                                    let addout = obank.get(addop).expect("collect").get_out().expect("collect: no out");
                                    if fd.lone_descend(addout).is_some() {
                                        opstack.push(addop);
                                        multstack.push(Some(subop));
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                    self.terms.push(AdditiveEdge::new(curop, i, multop, obank));
                    continue;
                }
                opstack.push(subop);
                multstack.push(multop);
            }
        }
    }

    /// Sort the terms using [`additive_compare`](TermOrder::additive_compare)
    /// (C++ `TermOrder::sortTerms`, `expression.cc:286-294`).
    ///
    /// The C++ `std::sort` is not guaranteed stable; the comparator below is a
    /// strict-weak-ordering wrapper around `Varnode::termOrder`.  We sort
    /// `sorter` (indices into `terms`) and use `sort_by` with a comparator that
    /// reproduces `additiveCompare` exactly.
    pub fn sort_terms(&mut self, fd: &crate::funcdata::Funcdata) {
        self.sorter.clear();
        self.sorter.reserve(self.terms.len());
        for idx in 0..self.terms.len() {
            self.sorter.push(idx);
        }
        // sort(sorter.begin(),sorter.end(),additiveCompare);
        // additiveCompare(op1,op2) == (-1 == op1->getVarnode()->termOrder(op2->getVarnode()))
        let terms = &self.terms;
        self.sorter.sort_by(|&a, &b| {
            let va = terms[a].get_varnode();
            let vb = terms[b].get_varnode();
            // The comparator is a "less than": op1 < op2 iff termOrder(va,vb) == -1.
            let lt_ab = term_order(va, vb, fd) == -1;
            if lt_ab {
                std::cmp::Ordering::Less
            } else {
                let lt_ba = term_order(vb, va, fd) == -1;
                if lt_ba {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Equal
                }
            }
        });
    }

    /// Get the sorted list of references (indices into `terms`) (C++ `getSort`).
    pub fn get_sort(&self) -> &Vec<usize> {
        &self.sorter
    }

    /// Borrow a term by sorter index (helper to read the sorted edges).
    pub fn term(&self, idx: usize) -> &AdditiveEdge {
        &self.terms[idx]
    }

    /// The comparator predicate `op1 < op2` (C++ `TermOrder::additiveCompare`,
    /// `expression.hh:144-146`): `-1 == op1->getVarnode()->termOrder(op2->getVarnode())`.
    pub fn additive_compare(op1: &AdditiveEdge, op2: &AdditiveEdge, fd: &crate::funcdata::Funcdata) -> bool {
        term_order(op1.get_varnode(), op2.get_varnode(), fd) == -1
    }
}

// =============================================================================
// Varnode::termOrder (varnode.cc:1172-1192) — local until varnode.rs ports it
// =============================================================================

/// Order two terms in a sum (C++ `Varnode::termOrder`, `varnode.cc:1172-1192`).
///
/// Returns -1, 0, or 1.  Constants sort after non-constants; non-constant terms
/// peel off a leading `INT_MULT` by a constant coefficient and then compare by
/// storage address.
///
/// SEAM(W3-varnode): the canonical home is `Varnode::termOrder` on `varnode.rs`,
/// which is not yet ported and this item does not own.  Transcribed here reading
/// through the banks (the `op.rs` "method needs the bank" pattern); hoists to
/// `Varnode::termOrder` when varnode.rs ports it.  // see losses
pub fn term_order(this_vn: VarnodeId, op_vn: VarnodeId, fd: &crate::funcdata::Funcdata) -> int4 {
    let vbank = fd.vbank();
    let obank = fd.obank();
    // if (isConstant()) { if (!op->isConstant()) return 1; }
    if vbank.get(this_vn).expect("termOrder: stale vn").is_constant() {
        if !vbank.get(op_vn).expect("termOrder: stale vn").is_constant() {
            return 1;
        }
    } else {
        // else { if (op->isConstant()) return -1; ... }
        if vbank.get(op_vn).expect("termOrder: stale vn").is_constant() {
            return -1;
        }
        // const Varnode *vn = this;
        let mut vn = this_vn;
        // if (vn->isWritten()&&(vn->getDef()->code() == CPUI_INT_MULT))
        //   if (vn->getDef()->getIn(1)->isConstant()) vn = vn->getDef()->getIn(0);
        if let Some(def) = peel_int_mult_const(vn, vbank, obank) {
            vn = def;
        }
        // (same for op)
        let mut op = op_vn;
        if let Some(def) = peel_int_mult_const(op, vbank, obank) {
            op = def;
        }
        // if (vn->getAddr() < op->getAddr()) return -1;
        // if (op->getAddr() < vn->getAddr()) return 1;
        let vn_addr = vbank.get(vn).expect("termOrder: stale vn").get_addr().clone();
        let op_addr = vbank.get(op).expect("termOrder: stale vn").get_addr().clone();
        if vn_addr < op_addr {
            return -1;
        }
        if op_addr < vn_addr {
            return 1;
        }
    }
    0
}

/// If `vn` is `INT_MULT` by a constant in slot 1, return its slot-0 input;
/// else `None` (the `vn = vn->getDef()->getIn(0)` peel in `termOrder`).
fn peel_int_mult_const(vn: VarnodeId, vbank: &VarnodeBank, obank: &PcodeOpBank) -> Option<VarnodeId> {
    let v = vbank.get(vn).expect("peel: stale vn");
    if !v.is_written() {
        return None;
    }
    let def = v.get_def().expect("peel: written but no def");
    let defop = obank.get(def).expect("peel: stale op");
    if defop.code() != OpCode::CPUI_INT_MULT {
        return None;
    }
    let in1 = defop.get_in(1).expect("peel: null in");
    if !vbank.get(in1).expect("peel: stale vn").is_constant() {
        return None;
    }
    defop.get_in(0)
}

// =============================================================================
// functionalEquality / functionalDifference (expression.cc:588-753)
// =============================================================================

/// Perform basic comparison of two given Varnodes (C++
/// `functionalEqualityLevel0`, `expression.cc:597-610`).
///
/// Returns:
///   - 0 if `vn1` and `vn2` must hold same value
///   - -1 if they definitely don't hold same value
///   - 1 if the same value depends on ops writing to `vn1` and `vn2`
fn functional_equality_level0(vn1: VarnodeId, vn2: VarnodeId, vbank: &VarnodeBank) -> int4 {
    if vn1 == vn2 {
        return 0;
    }
    let v1 = vbank.get(vn1).expect("funcEqLevel0: stale vn");
    let v2 = vbank.get(vn2).expect("funcEqLevel0: stale vn");
    if v1.get_size() != v2.get_size() {
        return -1;
    }
    if v1.is_constant() {
        if v2.is_constant() {
            return if v1.get_offset() == v2.get_offset() { 0 } else { -1 };
        }
        return -1;
    }
    if v1.is_free() || v2.is_free() {
        return -1;
    }
    1
}

/// Try to determine if `vn1` and `vn2` contain the same value (C++
/// `functionalEqualityLevel`, `expression.cc:625-705`).
///
/// Returns:
///   - `-1` if they do \b not, or if it can't be immediately verified
///   - `0` if they \b do hold the same value
///   - a positive count if the result is contingent on additional varnode pairs
///     having the same value; in that case the pairs are returned as
///     `(res1[i],res2[i])` and the return value is the number of pairs.
///
/// `res1`/`res2` are 2-element buffers, exactly as the C++ `Varnode *res1[2]`.
pub fn functional_equality_level(
    vn1: VarnodeId,
    vn2: VarnodeId,
    res1: &mut [VarnodeId; 2],
    res2: &mut [VarnodeId; 2],
    vbank: &VarnodeBank,
    obank: &PcodeOpBank,
) -> int4 {
    // int4 testval = functionalEqualityLevel0(vn1,vn2);
    let mut testval = functional_equality_level0(vn1, vn2, vbank);
    if testval != 1 {
        return testval;
    }
    // if (!vn1->isWritten() || !vn2->isWritten()) return -1;
    if !vbank.get(vn1).expect("funcEqLevel: stale vn").is_written()
        || !vbank.get(vn2).expect("funcEqLevel: stale vn").is_written()
    {
        return -1; // Did not find at least one level of match
    }
    let op1 = vbank.get(vn1).expect("funcEqLevel").get_def().expect("funcEqLevel: written no def");
    let op2 = vbank.get(vn2).expect("funcEqLevel").get_def().expect("funcEqLevel: written no def");
    let opc = obank.get(op1).expect("funcEqLevel: stale op").code();

    // if (opc != op2->code()) return -1;
    if opc != obank.get(op2).expect("funcEqLevel: stale op").code() {
        return -1;
    }

    // int4 num = op1->numInput(); if (num != op2->numInput()) return -1;
    let mut num = obank.get(op1).expect("funcEqLevel").num_input();
    if num != obank.get(op2).expect("funcEqLevel").num_input() {
        return -1;
    }
    // if (op1->isMarker()) return -1;
    if obank.get(op1).expect("funcEqLevel").is_marker() {
        return -1;
    }
    // if (op2->isCall()) return -1;
    if obank.get(op2).expect("funcEqLevel").is_call() {
        return -1;
    }
    if opc == OpCode::CPUI_LOAD {
        // We assume two loads produce the same result if the address is the same
        // and the loads occur in the same instruction
        // if (op1->getAddr() != op2->getAddr()) return -1;
        if obank.get(op1).expect("funcEqLevel").get_addr() != obank.get(op2).expect("funcEqLevel").get_addr() {
            return -1;
        }
    }
    if num >= 3 {
        // if (opc != CPUI_PTRADD) return -1;
        if opc != OpCode::CPUI_PTRADD {
            return -1;
        }
        // if (op1->getIn(2)->getOffset() != op2->getIn(2)->getOffset()) return -1;
        let o1in2 = obank.get(op1).expect("funcEqLevel").get_in(2).expect("funcEqLevel: null in");
        let o2in2 = obank.get(op2).expect("funcEqLevel").get_in(2).expect("funcEqLevel: null in");
        if vbank.get(o1in2).expect("funcEqLevel: stale vn").get_offset()
            != vbank.get(o2in2).expect("funcEqLevel: stale vn").get_offset()
        {
            return -1;
        }
        num = 2; // Otherwise treat as having 2 inputs
    }
    // for(i=0;i<num;++i){ res1[i]=op1->getIn(i); res2[i]=op2->getIn(i); }
    for i in 0..num {
        res1[i as usize] = obank.get(op1).expect("funcEqLevel").get_in(i).expect("funcEqLevel: null in");
        res2[i as usize] = obank.get(op2).expect("funcEqLevel").get_in(i).expect("funcEqLevel: null in");
    }

    // testval = functionalEqualityLevel0(res1[0],res2[0]);
    testval = functional_equality_level0(res1[0], res2[0], vbank);
    if testval == 0 {
        // A match locks in this comparison ordering
        if num == 1 {
            return 0;
        }
        testval = functional_equality_level0(res1[1], res2[1], vbank);
        if testval == 0 {
            return 0;
        }
        if testval < 0 {
            return -1;
        }
        res1[0] = res1[1]; // Match is contingent on second pair
        res2[0] = res2[1];
        return 1;
    }
    if num == 1 {
        return testval;
    }
    let testval2 = functional_equality_level0(res1[1], res2[1], vbank);
    if testval2 == 0 {
        // A match locks in this comparison ordering
        return testval;
    }
    let unmatchsize: int4 = if testval == 1 && testval2 == 1 { 2 } else { -1 };

    // if (!op1->isCommutative()) return unmatchsize;
    if !obank.get(op1).expect("funcEqLevel").is_commutative() {
        return unmatchsize;
    }
    // unmatchsize must be 2 or -1 here on a commutative operator, try flipping
    let comm1 = functional_equality_level0(res1[0], res2[1], vbank);
    let comm2 = functional_equality_level0(res1[1], res2[0], vbank);
    if comm1 == 0 && comm2 == 0 {
        return 0;
    }
    if comm1 < 0 || comm2 < 0 {
        return unmatchsize;
    }
    if comm1 == 0 {
        // AND (comm2==1)
        res1[0] = res1[1]; // Left over unmatch is res1[1] and res2[0]
        return 1;
    }
    if comm2 == 0 {
        // AND (comm1==1)
        res2[0] = res2[1]; // Left over unmatch is res1[0] and res2[1]
        return 1;
    }
    // If we reach here (comm1==1) AND (comm2==1)
    if unmatchsize == 2 {
        // If the original ordering wasn't impossible
        return 2; // Prefer the original ordering
    }
    // Otherwise swap the ordering
    // (C++: Varnode *tmpvn = res2[0]; res2[0] = res2[1]; res2[1] = tmpvn;)
    res2.swap(0, 1);
    2
}

/// Determine if two Varnodes hold the same value (C++ `functionalEquality`,
/// `expression.cc:713-719`).  Only returns \b true if it can be immediately
/// determined they are equivalent.
pub fn functional_equality(vn1: VarnodeId, vn2: VarnodeId, vbank: &VarnodeBank, obank: &PcodeOpBank) -> bool {
    let mut buf1: [VarnodeId; 2] = [vn1, vn1];
    let mut buf2: [VarnodeId; 2] = [vn2, vn2];
    functional_equality_level(vn1, vn2, &mut buf1, &mut buf2, vbank, obank) == 0
}

/// Return true if `vn1` and `vn2` are verifiably different values (C++
/// `functionalDifference`, `expression.cc:728-753`).  This is a rather
/// speculative test; `depth` bounds the recursion.
pub fn functional_difference(vn1: VarnodeId, vn2: VarnodeId, depth: int4, vbank: &VarnodeBank, obank: &PcodeOpBank) -> bool {
    // if (vn1 == vn2) return false;
    if vn1 == vn2 {
        return false;
    }
    let v1 = vbank.get(vn1).expect("funcDiff: stale vn");
    let v2 = vbank.get(vn2).expect("funcDiff: stale vn");
    if !v1.is_written() || !v2.is_written() {
        // if (vn1->isConstant() && vn2->isConstant()) return !(vn1->getAddr()==vn2->getAddr());
        if v1.is_constant() && v2.is_constant() {
            // C++: return !(vn1->getAddr()==vn2->getAddr());
            return v1.get_addr() != v2.get_addr();
        }
        // if (vn1->isInput()&&vn2->isInput()) return false; // Might be the same
        if v1.is_input() && v2.is_input() {
            return false;
        }
        // if (vn1->isFree()||vn2->isFree()) return false; // Might be the same
        if v1.is_free() || v2.is_free() {
            return false;
        }
        return true;
    }
    let op1 = v1.get_def().expect("funcDiff: written no def");
    let op2 = v2.get_def().expect("funcDiff: written no def");
    // if (op1->code() != op2->code()) return true;
    if obank.get(op1).expect("funcDiff: stale op").code() != obank.get(op2).expect("funcDiff: stale op").code() {
        return true;
    }
    // num = op1->numInput(); if (num != op2->numInput()) return true;
    let num = obank.get(op1).expect("funcDiff").num_input();
    if num != obank.get(op2).expect("funcDiff").num_input() {
        return true;
    }
    // if (depth==0) return true;
    if depth == 0 {
        return true; // Different as far as we can tell
    }
    let depth = depth - 1;
    for i in 0..num {
        let a = obank.get(op1).expect("funcDiff").get_in(i).expect("funcDiff: null in");
        let b = obank.get(op2).expect("funcDiff").get_in(i).expect("funcDiff: null in");
        if functional_difference(a, b, depth, vbank, obank) {
            return true;
        }
    }
    false
}

// =============================================================================
// rootPointer / pointerEquality (expression.cc:761-802)
// =============================================================================

/// Back-track as far as possible from a pointer Varnode thru PTRSUB, INT_ADD,
/// and COPY collecting offsets (C++ `rootPointer`, `expression.cc:761-786`).
///
/// Returns the reached Varnode and passes back the accumulated `offset`.
pub fn root_pointer(mut vn: VarnodeId, offset: &mut uintb, vbank: &VarnodeBank, obank: &PcodeOpBank) -> VarnodeId {
    *offset = 0;
    loop {
        // if (!vn->isWritten()) break;
        let v = vbank.get(vn).expect("rootPointer: stale vn");
        if !v.is_written() {
            break;
        }
        let op = v.get_def().expect("rootPointer: written no def");
        let opc = obank.get(op).expect("rootPointer: stale op").code();
        if opc == OpCode::CPUI_PTRSUB {
            // offset += op->getIn(1)->getOffset(); vn = op->getIn(0);
            let in1 = obank.get(op).expect("rootPointer").get_in(1).expect("rootPointer: null in");
            *offset = offset.wrapping_add(vbank.get(in1).expect("rootPointer: stale vn").get_offset());
            vn = obank.get(op).expect("rootPointer").get_in(0).expect("rootPointer: null in");
        } else if opc == OpCode::CPUI_INT_ADD {
            // Varnode *cvn = op->getIn(1); if (!cvn->isConstant()) break;
            let cvn = obank.get(op).expect("rootPointer").get_in(1).expect("rootPointer: null in");
            if !vbank.get(cvn).expect("rootPointer: stale vn").is_constant() {
                break;
            }
            // offset += cvn->getOffset(); vn = op->getIn(0);
            *offset = offset.wrapping_add(vbank.get(cvn).expect("rootPointer: stale vn").get_offset());
            vn = obank.get(op).expect("rootPointer").get_in(0).expect("rootPointer: null in");
        } else if opc == OpCode::CPUI_COPY {
            // vn = op->getIn(0);
            vn = obank.get(op).expect("rootPointer").get_in(0).expect("rootPointer: null in");
        } else {
            break;
        }
    }
    vn
}

/// Determine if two pointer Varnodes always hold the same value (C++
/// `pointerEquality`, `expression.cc:792-802`).
pub fn pointer_equality(vn1: VarnodeId, vn2: VarnodeId, vbank: &VarnodeBank, obank: &PcodeOpBank) -> bool {
    let mut off1: uintb = 0;
    let mut off2: uintb = 0;
    if vn1 == vn2 {
        return true;
    }
    let vn1 = root_pointer(vn1, &mut off1, vbank, obank);
    let vn2 = root_pointer(vn2, &mut off2, vbank, obank);
    if off1 != off2 {
        return false;
    }
    vn1 == vn2
}

// =============================================================================
// loneDescend helper (Varnode::loneDescend, varnode.cc:301)
// =============================================================================

/// `Varnode::loneDescend` returning whether there is exactly one descendant op:
/// the op if so, else `None`.  The C++ returns the op pointer or 0; this reads
/// through the bank.  Used by [`TraverseNode::is_alternate_path_valid`].
///
/// Returns `1` (a sentinel matching the C++ `!= 0` test) when there is exactly
/// one descendant; the callers here only test for "== 0" / "!= 0" so we return
/// the count's truthiness as an `int4` (0 or 1) — mirroring the C++ pointer
/// nullness test, not the pointer value.
fn vn_lone_descend(vn: VarnodeId, vbank: &VarnodeBank) -> int4 {
    let v = vbank.get(vn).expect("loneDescend: stale vn");
    // The C++ returns the op if exactly one descend, else 0.  Callers test only
    // == 0 / != 0; reproduce that truthiness via the descend count.
    let mut count = 0;
    let mut found = false;
    for _ in v.descend_iter() {
        count += 1;
        found = true;
        if count > 1 {
            return 0; // more than one => loneDescend() == 0
        }
    }
    if found && count == 1 {
        1
    } else {
        0
    }
}

#[cfg(test)]
mod tests;
