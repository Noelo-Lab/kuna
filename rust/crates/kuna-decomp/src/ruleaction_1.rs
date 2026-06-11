//! Port of `decompiler/cpp/ruleaction.cc` lines 1-1443 — the first batch of the
//! expression-simplification [`Rule`](crate::action::Rule) set
//! (`RuleEarlyRemoval` .. `RuleRangeMeld`, 25 classes, in C++ definition order).
//!
//! # Faithfulness
//!
//! Each rule reproduces the upstream `applyOp` body statement-for-statement and
//! `getOpList` opcode-for-opcode (the rule body **is** decompiler output — a
//! reordered guard changes datatest text).  The C++ semantic comments are carried
//! through.  Mixed-sign comparisons and wrapping arithmetic are made explicit per
//! the porting rules.
//!
//! # Cross-wave seams
//!
//! Several upstream `Funcdata`/global helpers are not yet ported; the rule that
//! needs one transcribes every early-out guard *up to* the call and then returns
//! `0` (no change) at the seam, with a `// SEAM(...)` note and a recorded loss.
//! The affected calls:
//!
//! * `op_set_output` / `new_varnode_out` / `new_unique_out` (W3 IR seam — the
//!   `Funcdata::banks_mut()` split-borrow accessor is missing, so any rule that
//!   *creates a new op with an output* defers at the output-creation step):
//!   `RuleCollectTerms`, `RuleNotDistribute`, `RuleAndDistribute`,
//!   `RulePullsubMulti`, `RulePullsubIndirect`, `RulePushMulti`.
//! * `deadRemovalAllowedSeen` (heritage seam): `RuleEarlyRemoval`.
//! * `cseEliminateList` (funcdata CSE seam): `RuleSelectCse`.
//! * `replaceLessequal` (funcdata compare-form seam): `RuleIntLessEqual`.
//! * `distributeIntMultAdd` + `TermOrder` (expression seam): `RuleCollectTerms`.
//! * `functionalEquality` / `functionalEqualityLevel` (expression seam):
//!   `RuleEquality`, `RulePushMulti`, `RuleRangeMeld`.
//! * `getOpFromConst` / `newIndirectCreation` (W3/W4 seam): `RulePullsubIndirect`.
//! * `findJoin` (W5 join-record seam): `RulePullsubMulti::build_subpiece`.
//! * `cseFindInBlock` / `earliestUse` (block/CSE seam): `RulePushMulti`.
//! * `CircleRange::pullBack`/`circleUnion`/`translate2Op`/`intersect`
//!   (rangeutil seam): `RuleRangeMeld`.
//! * `op_set_opcode` requires a fully-formed `TypeOp` (opcode + property flags +
//!   name); the `glb->inst[opc]` table that supplies the **property flags** is
//!   the W6 `TypeFactory`.  [`set_opcode_seam`] builds the `TypeOp` with the
//!   correct opcode value and an empty flag word (`0`).  The opcode *value* — the
//!   load-bearing part for these rules and the action engine's per-op dispatch —
//!   is correct; the cached flag bits are filled when W6 lands.  // SEAM(W6)

use kuna_base::address::{calc_mask, leastsigbit_set, pcode_left, pcode_right};
use kuna_base::types::{int4, uintb, Wrap};
use kuna_num::opcodes::OpCode;

use crate::action::{ActionGroupList, Rule, RuleSpec};
use crate::funcdata::Funcdata;
use crate::seams::{OpId, TypeOp, VarnodeId};

/// `sizeof(uintb)` from the C++ — the rules cap at 8-byte precision
/// (`if (size > sizeof(uintb)) return 0`).
const SIZEOF_UINTB: int4 = 8;

/// Resolve an [`OpCode`] to the [`TypeOp`] that [`Funcdata::op_set_opcode`]
/// expects (C++ `glb->inst[opc]`).  // SEAM(W6)
///
/// The C++ `Funcdata::opSetOpcode` does `obank.changeOpcode(op, glb->inst[opc])`,
/// resolving the singleton `TypeOp` (opcode value + cached property flags +
/// display name) from the architecture's `inst` table.  That table is the W6
/// `TypeFactory`; until it lands we construct the `TypeOp` with the right opcode
/// value, an empty property-flag word, and the debug name.  The opcode value is
/// what the rules and the action engine's per-op dispatch read; the flags are a
/// W6 fill.
fn set_opcode_seam(data: &mut Funcdata, op: OpId, opc: OpCode) {
    // obank.changeOpcode(op, glb->inst[opc]);  -- flags from glb->inst is W6.
    data.op_set_opcode(op, TypeOp::new(opc, 0, format!("{opc:?}")));
}

// =============================================================================
// RuleEarlyRemoval (ruleaction.cc:24-45)
// =============================================================================

/// Get rid of unused PcodeOp objects where we can guarantee the output is unused
/// (C++ `RuleEarlyRemoval`).
pub struct RuleEarlyRemoval {
    group: String,
}

impl RuleEarlyRemoval {
    /// Constructor (C++ `RuleEarlyRemoval(const string &g)`, name `"earlyremoval"`).
    pub fn new(g: impl Into<String>) -> RuleEarlyRemoval {
        RuleEarlyRemoval { group: g.into() }
    }
}

impl Rule for RuleEarlyRemoval {
    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(&self.group) {
            return None;
        }
        Some(Box::new(RuleEarlyRemoval::new(self.group.clone())))
    }

    // This rule applies to all ops (default get_op_list).

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // if (op->isCall()) return 0;	// Functions automatically consumed
        if data.obank().get(op).expect("RuleEarlyRemoval: stale op").is_call() {
            return 0;
        }
        // if (op->isIndirectSource()) return 0;
        if data.obank().get(op).expect("RuleEarlyRemoval: stale op").is_indirect_source() {
            return 0;
        }
        // vn = op->getOut();
        let vn = match data.obank().get(op).expect("RuleEarlyRemoval: stale op").get_out() {
            // if (vn == (Varnode *)0) return 0;
            None => return 0,
            Some(v) => v,
        };
        //  if (vn->isPersist()) return 0;
        // if (!vn->hasNoDescend()) return 0;
        if !data.vbank().get(vn).expect("RuleEarlyRemoval: stale vn").has_no_descend() {
            return 0;
        }
        // if (vn->isAutoLive()) return 0;
        if data.vbank().get(vn).expect("RuleEarlyRemoval: stale vn").is_auto_live() {
            return 0;
        }
        // AddrSpace *spc = vn->getSpace();
        let spc = data.vbank().get(vn).expect("RuleEarlyRemoval: stale vn").get_space().clone();
        // if (spc->doesDeadcode())
        if spc.does_deadcode() {
            //   if (!data.deadRemovalAllowedSeen(spc))
            //     return 0;
            // SEAM(heritage): Funcdata::deadRemovalAllowedSeen delegates to the
            // Heritage subsystem (heritage.rs is a stub).  Without it we cannot
            // confirm dead removal is allowed for this space, so we
            // conservatively decline (the C++ also returns 0 in that case).
            return 0;
        }

        // data.opDestroy(op);		// Get rid of unused op
        data.op_destroy(op);
        1
    }
}

// =============================================================================
// RuleCollectTerms (ruleaction.cc:76-177)
// =============================================================================

/// Collect terms in a sum: `V * c + V * d => V * (c + d)` (C++ `RuleCollectTerms`).
pub struct RuleCollectTerms {
    group: String,
}

impl RuleCollectTerms {
    /// Constructor (C++ `RuleCollectTerms`, name `"collect_terms"`).
    pub fn new(g: impl Into<String>) -> RuleCollectTerms {
        RuleCollectTerms { group: g.into() }
    }

    /// Get the multiplicative coefficient (C++ `RuleCollectTerms::getMultCoeff`).
    ///
    /// Given a Varnode term, if its defining op is `V * c` with `c` constant,
    /// pass back `c` and return `V`; otherwise pass back `1` and return the
    /// original Varnode.  This static is shared with the `applyOp` body below; it
    /// reads only the IR so it is portable.
    pub fn get_mult_coeff(data: &Funcdata, vn: VarnodeId, coef: &mut uintb) -> VarnodeId {
        // if (!vn->isWritten()) { coef = 1; return vn; }
        let v = data.vbank().get(vn).expect("getMultCoeff: stale vn");
        if !v.is_written() {
            *coef = 1;
            return vn;
        }
        // testop = vn->getDef();
        let testop = v.get_def().expect("getMultCoeff: written vn has no def");
        let to = data.obank().get(testop).expect("getMultCoeff: stale def op");
        // if ((testop->code() != CPUI_INT_MULT)||(!testop->getIn(1)->isConstant())) {
        //   coef = 1; return vn; }
        let in1 = to.get_in(1);
        let in1_const = in1
            .map(|i| data.vbank().get(i).expect("getMultCoeff: stale in1").is_constant())
            .unwrap_or(false);
        if to.code() != OpCode::CPUI_INT_MULT || !in1_const {
            *coef = 1;
            return vn;
        }
        // coef = testop->getIn(1)->getOffset();
        *coef = data
            .vbank()
            .get(in1.expect("getMultCoeff: in1"))
            .expect("getMultCoeff: stale in1")
            .get_offset();
        // return testop->getIn(0);
        data.obank().get(testop).expect("getMultCoeff: stale def op").get_in(0).expect(
            "getMultCoeff: INT_MULT has no in0",
        )
    }
}

impl Rule for RuleCollectTerms {
    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(&self.group) {
            return None;
        }
        Some(Box::new(RuleCollectTerms::new(self.group.clone())))
    }

    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_INT_ADD]
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // PcodeOp *nextop = op->getOut()->loneDescend();
        let outvn = data.obank().get(op).expect("RuleCollectTerms: stale op").get_out().expect(
            "RuleCollectTerms: INT_ADD has no output",
        );
        let nextop = data.lone_descend(outvn);
        // Do we have the root of an ADD tree
        // if ((nextop!=0)&&(nextop->code()==CPUI_INT_ADD)) return 0;
        if let Some(no) = nextop {
            if data.obank().get(no).expect("RuleCollectTerms: stale nextop").code()
                == OpCode::CPUI_INT_ADD
            {
                return 0;
            }
        }

        // TermOrder termorder(op); termorder.collect(); termorder.sortTerms();
        //   ... iterate the sorted additive edges, combine like terms ...
        // SEAM(expression): `TermOrder`/`AdditiveEdge` live in expression.cc
        // (not yet ported) and the combine branch also calls
        // `data.distributeIntMultAdd(...)` (funcdata seam) and creates a new
        // INT_MULT op with `newUniqueOut` (W3 output-creation seam).  The root-of-
        // ADD-tree early-out above is ported faithfully; the term-collection body
        // defers here.  No change.
        0
    }
}

// =============================================================================
// RuleSelectCse (ruleaction.cc:179-210)
// =============================================================================

/// Look for common sub-expressions built from a restricted set of ops
/// (C++ `RuleSelectCse`).
pub struct RuleSelectCse {
    group: String,
}

impl RuleSelectCse {
    /// Constructor (C++ `RuleSelectCse`, name `"selectcse"`).
    pub fn new(g: impl Into<String>) -> RuleSelectCse {
        RuleSelectCse { group: g.into() }
    }
}

impl Rule for RuleSelectCse {
    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(&self.group) {
            return None;
        }
        Some(Box::new(RuleSelectCse::new(self.group.clone())))
    }

    fn get_op_list(&self) -> Vec<OpCode> {
        // oplist.push_back(CPUI_SUBPIECE);
        // oplist.push_back(CPUI_INT_SRIGHT); // For division optimization corrections
        vec![OpCode::CPUI_SUBPIECE, OpCode::CPUI_INT_SRIGHT]
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // Varnode *vn = op->getIn(0);  OpCode opc = op->code();
        let vn = data.obank().get(op).expect("RuleSelectCse: stale op").get_in(0).expect(
            "RuleSelectCse: op has no in0",
        );
        let opc = data.obank().get(op).expect("RuleSelectCse: stale op").code();

        // vector< pair<uintm,PcodeOp *> > list;
        // for(iter over vn->descendants) { otherop=*iter;
        //   if (otherop->code() != opc) continue;
        //   hash = otherop->getCseHash(); if (hash==0) continue;
        //   list.push_back(pair(hash,otherop)); }
        // if (list.size()<=1) return 0;
        // SEAM(funcdata CSE): `PcodeOp::getCseHash` + `data.cseEliminateList` are
        // the funcdata CSE machinery (not yet ported).  We can count candidate
        // descendants sharing the opcode, but `getCseHash` (which may return 0 to
        // skip an op) and the elimination step are unavailable, so the rule
        // declines.  No change.
        let _ = (vn, opc);
        0
    }
}

// =============================================================================
// RulePiece2Zext (ruleaction.cc:212-231)
// =============================================================================

/// Concatenation with 0 becomes an extension: `V = concat(#0,W) => V = zext(W)`
/// (C++ `RulePiece2Zext`).
pub struct RulePiece2Zext {
    group: String,
}

impl RulePiece2Zext {
    /// Constructor (C++ `RulePiece2Zext`, name `"piece2zext"`).
    pub fn new(g: impl Into<String>) -> RulePiece2Zext {
        RulePiece2Zext { group: g.into() }
    }
}

impl Rule for RulePiece2Zext {
    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(&self.group) {
            return None;
        }
        Some(Box::new(RulePiece2Zext::new(self.group.clone())))
    }

    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_PIECE]
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // constvn = op->getIn(0);	// Constant must be most significant bits
        let constvn = data.obank().get(op).expect("RulePiece2Zext: stale op").get_in(0).expect(
            "RulePiece2Zext: PIECE has no in0",
        );
        let cv = data.vbank().get(constvn).expect("RulePiece2Zext: stale constvn");
        // if (!constvn->isConstant()) return 0;	// Must append with constant
        if !cv.is_constant() {
            return 0;
        }
        // if (constvn->getOffset() != 0) return 0; // of value 0
        if cv.get_offset() != 0 {
            return 0;
        }
        // data.opRemoveInput(op,0);	// Remove the constant
        data.op_remove_input(op, 0);
        // data.opSetOpcode(op,CPUI_INT_ZEXT);
        set_opcode_seam(data, op, OpCode::CPUI_INT_ZEXT);
        1
    }
}

// =============================================================================
// RulePiece2Sext (ruleaction.cc:233-260)
// =============================================================================

/// Concatenation with sign bits becomes an extension:
/// `concat( V s>> #0x1f , V) => sext(V)` (C++ `RulePiece2Sext`).
pub struct RulePiece2Sext {
    group: String,
}

impl RulePiece2Sext {
    /// Constructor (C++ `RulePiece2Sext`, name `"piece2sext"`).
    pub fn new(g: impl Into<String>) -> RulePiece2Sext {
        RulePiece2Sext { group: g.into() }
    }
}

impl Rule for RulePiece2Sext {
    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(&self.group) {
            return None;
        }
        Some(Box::new(RulePiece2Sext::new(self.group.clone())))
    }

    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_PIECE]
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // shiftout = op->getIn(0);
        let shiftout = data.obank().get(op).expect("RulePiece2Sext: stale op").get_in(0).expect(
            "RulePiece2Sext: PIECE has no in0",
        );
        // if (!shiftout->isWritten()) return 0;
        if !data.vbank().get(shiftout).expect("RulePiece2Sext: stale shiftout").is_written() {
            return 0;
        }
        // shiftop = shiftout->getDef();
        let shiftop = data
            .vbank()
            .get(shiftout)
            .expect("RulePiece2Sext: stale shiftout")
            .get_def()
            .expect("RulePiece2Sext: written shiftout has no def");
        // if (shiftop->code() != CPUI_INT_SRIGHT) return 0;
        if data.obank().get(shiftop).expect("RulePiece2Sext: stale shiftop").code()
            != OpCode::CPUI_INT_SRIGHT
        {
            return 0;
        }
        // if (!shiftop->getIn(1)->isConstant()) return 0;
        let sh_in1 = data.obank().get(shiftop).expect("RulePiece2Sext: stale shiftop").get_in(1)
            .expect("RulePiece2Sext: SRIGHT has no in1");
        if !data.vbank().get(sh_in1).expect("RulePiece2Sext: stale sh_in1").is_constant() {
            return 0;
        }
        // int4 n = shiftop->getIn(1)->getOffset();
        let n = data.vbank().get(sh_in1).expect("RulePiece2Sext: stale sh_in1").get_offset() as int4;
        // x = shiftop->getIn(0);
        let x = data.obank().get(shiftop).expect("RulePiece2Sext: stale shiftop").get_in(0).expect(
            "RulePiece2Sext: SRIGHT has no in0",
        );
        // if (x != op->getIn(1)) return 0;
        let op_in1 = data.obank().get(op).expect("RulePiece2Sext: stale op").get_in(1).expect(
            "RulePiece2Sext: PIECE has no in1",
        );
        if x != op_in1 {
            return 0;
        }
        // if (n != 8*x->getSize() -1) return 0;
        let xsize = data.vbank().get(x).expect("RulePiece2Sext: stale x").get_size();
        if n != 8 * xsize - 1 {
            return 0;
        }

        // data.opRemoveInput(op,0);
        data.op_remove_input(op, 0);
        // data.opSetOpcode(op,CPUI_INT_SEXT);
        set_opcode_seam(data, op, OpCode::CPUI_INT_SEXT);
        1
    }
}

// =============================================================================
// RuleBxor2NotEqual (ruleaction.cc:262-275)
// =============================================================================

/// Eliminate BOOL_XOR: `V ^^ W => V != W` (C++ `RuleBxor2NotEqual`).
pub struct RuleBxor2NotEqual {
    group: String,
}

impl RuleBxor2NotEqual {
    /// Constructor (C++ `RuleBxor2NotEqual`, name `"bxor2notequal"`).
    pub fn new(g: impl Into<String>) -> RuleBxor2NotEqual {
        RuleBxor2NotEqual { group: g.into() }
    }
}

impl Rule for RuleBxor2NotEqual {
    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(&self.group) {
            return None;
        }
        Some(Box::new(RuleBxor2NotEqual::new(self.group.clone())))
    }

    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_BOOL_XOR]
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // data.opSetOpcode(op,CPUI_INT_NOTEQUAL);
        set_opcode_seam(data, op, OpCode::CPUI_INT_NOTEQUAL);
        1
    }
}

// =============================================================================
// RuleOrMask (ruleaction.cc:277-301)
// =============================================================================

/// Simplify INT_OR with full mask: `V = W | 0xffff => V = W` (C++ `RuleOrMask`).
pub struct RuleOrMask {
    group: String,
}

impl RuleOrMask {
    /// Constructor (C++ `RuleOrMask`, name `"ormask"`).
    pub fn new(g: impl Into<String>) -> RuleOrMask {
        RuleOrMask { group: g.into() }
    }
}

impl Rule for RuleOrMask {
    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(&self.group) {
            return None;
        }
        Some(Box::new(RuleOrMask::new(self.group.clone())))
    }

    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_INT_OR]
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // int4 size = op->getOut()->getSize();
        let outvn = data.obank().get(op).expect("RuleOrMask: stale op").get_out().expect(
            "RuleOrMask: INT_OR has no output",
        );
        let size = data.vbank().get(outvn).expect("RuleOrMask: stale outvn").get_size();
        // if (size > sizeof(uintb)) return 0; // FIXME: uintb should be arbitrary precision
        if size > SIZEOF_UINTB {
            return 0;
        }
        // constvn = op->getIn(1);
        let constvn = data.obank().get(op).expect("RuleOrMask: stale op").get_in(1).expect(
            "RuleOrMask: INT_OR has no in1",
        );
        // if (!constvn->isConstant()) return 0;
        if !data.vbank().get(constvn).expect("RuleOrMask: stale constvn").is_constant() {
            return 0;
        }
        // uintb val = constvn->getOffset();
        let val = data.vbank().get(constvn).expect("RuleOrMask: stale constvn").get_offset();
        // uintb mask = calc_mask(size);
        let mask = calc_mask(size);
        // if ((val&mask) != mask) return 0;
        if (val & mask) != mask {
            return 0;
        }
        // data.opSetOpcode(op,CPUI_COPY);
        set_opcode_seam(data, op, OpCode::CPUI_COPY);
        // data.opSetInput(op,constvn,0);
        data.op_set_input(op, constvn, 0).expect("RuleOrMask: op_set_input");
        // data.opRemoveInput(op,1);
        data.op_remove_input(op, 1);
        1
    }
}

// =============================================================================
// RuleAndMask (ruleaction.cc:303-343)
// =============================================================================

/// Collapse unnecessary INT_AND (C++ `RuleAndMask`).
pub struct RuleAndMask {
    group: String,
}

impl RuleAndMask {
    /// Constructor (C++ `RuleAndMask`, name `"andmask"`).
    pub fn new(g: impl Into<String>) -> RuleAndMask {
        RuleAndMask { group: g.into() }
    }
}

impl Rule for RuleAndMask {
    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(&self.group) {
            return None;
        }
        Some(Box::new(RuleAndMask::new(self.group.clone())))
    }

    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_INT_AND]
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // int4 size = op->getOut()->getSize();
        let outvn = data.obank().get(op).expect("RuleAndMask: stale op").get_out().expect(
            "RuleAndMask: INT_AND has no output",
        );
        let size = data.vbank().get(outvn).expect("RuleAndMask: stale outvn").get_size();

        // if (size > sizeof(uintb)) return 0; // FIXME: uintb should be arbitrary precision
        if size > SIZEOF_UINTB {
            return 0;
        }
        // mask1 = op->getIn(0)->getNZMask();
        let in0 = data.obank().get(op).expect("RuleAndMask: stale op").get_in(0).expect(
            "RuleAndMask: INT_AND has no in0",
        );
        let mask1 = data.vbank().get(in0).expect("RuleAndMask: stale in0").get_nz_mask();
        // uintb andmask;  if (mask1 == 0) andmask = 0; else { mask2 = in(1)->NZMask; andmask = mask1 & mask2; }
        let andmask: uintb = if mask1 == 0 {
            0
        } else {
            // mask2 = op->getIn(1)->getNZMask();
            let in1 = data.obank().get(op).expect("RuleAndMask: stale op").get_in(1).expect(
                "RuleAndMask: INT_AND has no in1",
            );
            let mask2 = data.vbank().get(in1).expect("RuleAndMask: stale in1").get_nz_mask();
            // andmask = mask1 & mask2;
            mask1 & mask2
        };

        // C++ evaluates `op->getOut()->getConsume()` only in the second branch; it
        // has no side effects so hoisting it does not change behavior.
        let consume = data.vbank().get(outvn).expect("RuleAndMask: stale outvn").get_consume();
        let vn: VarnodeId;
        // if (andmask==0)  // Result of AND is always zero
        // else if ((andmask & op->getOut()->getConsume())==0)
        //   -- both produce vn = newConstant(size,0)
        if andmask == 0 || (andmask & consume) == 0 {
            // vn = data.newConstant( size, 0);
            vn = data.new_constant(size, 0);
        } else if andmask == mask1 {
            // else if (andmask == mask1) {
            //   if (!op->getIn(1)->isConstant()) return 0;
            let in1 = data.obank().get(op).expect("RuleAndMask: stale op").get_in(1).expect(
                "RuleAndMask: INT_AND has no in1",
            );
            if !data.vbank().get(in1).expect("RuleAndMask: stale in1").is_constant() {
                return 0;
            }
            // vn = op->getIn(0);		// Result of AND is equal to input(0)
            vn = data.obank().get(op).expect("RuleAndMask: stale op").get_in(0).expect(
                "RuleAndMask: INT_AND has no in0",
            );
        } else {
            // else return 0;
            return 0;
        }
        // if (!vn->isHeritageKnown()) return 0;
        if !data.vbank().get(vn).expect("RuleAndMask: stale vn").is_heritage_known() {
            return 0;
        }

        // data.opSetOpcode(op,CPUI_COPY);
        set_opcode_seam(data, op, OpCode::CPUI_COPY);
        // data.opRemoveInput(op,1);
        data.op_remove_input(op, 1);
        // data.opSetInput(op,vn,0);
        data.op_set_input(op, vn, 0).expect("RuleAndMask: op_set_input");
        1
    }
}

// =============================================================================
// RuleOrConsume (ruleaction.cc:345-372)
// =============================================================================

/// Simplify OR/XOR with unconsumed input:
/// `V = A | B => V = B  if nzm(A) & consume(V) == 0` (C++ `RuleOrConsume`).
pub struct RuleOrConsume {
    group: String,
}

impl RuleOrConsume {
    /// Constructor (C++ `RuleOrConsume`, name `"orconsume"`).
    pub fn new(g: impl Into<String>) -> RuleOrConsume {
        RuleOrConsume { group: g.into() }
    }
}

impl Rule for RuleOrConsume {
    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(&self.group) {
            return None;
        }
        Some(Box::new(RuleOrConsume::new(self.group.clone())))
    }

    fn get_op_list(&self) -> Vec<OpCode> {
        // oplist.push_back(CPUI_INT_OR);
        // oplist.push_back(CPUI_INT_XOR);
        vec![OpCode::CPUI_INT_OR, OpCode::CPUI_INT_XOR]
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // Varnode *outvn = op->getOut();  int4 size = outvn->getSize();
        let outvn = data.obank().get(op).expect("RuleOrConsume: stale op").get_out().expect(
            "RuleOrConsume: op has no output",
        );
        let size = data.vbank().get(outvn).expect("RuleOrConsume: stale outvn").get_size();
        // if (size > sizeof(uintb)) return 0; // FIXME: uintb should be arbitrary precision
        if size > SIZEOF_UINTB {
            return 0;
        }
        // uintb consume = outvn->getConsume();
        let consume = data.vbank().get(outvn).expect("RuleOrConsume: stale outvn").get_consume();
        // if ((consume & op->getIn(0)->getNZMask()) == 0) {
        let in0 = data.obank().get(op).expect("RuleOrConsume: stale op").get_in(0).expect(
            "RuleOrConsume: op has no in0",
        );
        let nzm0 = data.vbank().get(in0).expect("RuleOrConsume: stale in0").get_nz_mask();
        if (consume & nzm0) == 0 {
            //   data.opRemoveInput(op,0);
            data.op_remove_input(op, 0);
            //   data.opSetOpcode(op, CPUI_COPY);
            set_opcode_seam(data, op, OpCode::CPUI_COPY);
            return 1;
        }
        // else if ((consume & op->getIn(1)->getNZMask()) == 0) {
        let in1 = data.obank().get(op).expect("RuleOrConsume: stale op").get_in(1).expect(
            "RuleOrConsume: op has no in1",
        );
        let nzm1 = data.vbank().get(in1).expect("RuleOrConsume: stale in1").get_nz_mask();
        if (consume & nzm1) == 0 {
            //   data.opRemoveInput(op,1);
            data.op_remove_input(op, 1);
            //   data.opSetOpcode(op, CPUI_COPY);
            set_opcode_seam(data, op, OpCode::CPUI_COPY);
            return 1;
        }
        0
    }
}

// =============================================================================
// RuleOrCollapse (ruleaction.cc:374-402)
// =============================================================================

/// Collapse unnecessary INT_OR: replace `V | c` with `c` if every bit not set in
/// `c` is also not set in `V` (C++ `RuleOrCollapse`).
pub struct RuleOrCollapse {
    group: String,
}

impl RuleOrCollapse {
    /// Constructor (C++ `RuleOrCollapse`, name `"orcollapse"`).
    pub fn new(g: impl Into<String>) -> RuleOrCollapse {
        RuleOrCollapse { group: g.into() }
    }
}

impl Rule for RuleOrCollapse {
    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(&self.group) {
            return None;
        }
        Some(Box::new(RuleOrCollapse::new(self.group.clone())))
    }

    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_INT_OR]
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // int4 size = op->getOut()->getSize();
        let outvn = data.obank().get(op).expect("RuleOrCollapse: stale op").get_out().expect(
            "RuleOrCollapse: INT_OR has no output",
        );
        let size = data.vbank().get(outvn).expect("RuleOrCollapse: stale outvn").get_size();

        // vn = op->getIn(1);
        let vn = data.obank().get(op).expect("RuleOrCollapse: stale op").get_in(1).expect(
            "RuleOrCollapse: INT_OR has no in1",
        );
        // if (!vn->isConstant()) return 0;
        if !data.vbank().get(vn).expect("RuleOrCollapse: stale vn").is_constant() {
            return 0;
        }
        // if (size > sizeof(uintb)) return 0; // FIXME: uintb should be arbitrary precision
        if size > SIZEOF_UINTB {
            return 0;
        }
        // mask = op->getIn(0)->getNZMask();
        let in0 = data.obank().get(op).expect("RuleOrCollapse: stale op").get_in(0).expect(
            "RuleOrCollapse: INT_OR has no in0",
        );
        let mask = data.vbank().get(in0).expect("RuleOrCollapse: stale in0").get_nz_mask();
        // val = vn->getOffset();
        let val = data.vbank().get(vn).expect("RuleOrCollapse: stale vn").get_offset();
        // if ((mask | val)!=val) return 0; // first param may turn on other bits
        if (mask | val) != val {
            return 0;
        }

        // data.opSetOpcode(op,CPUI_COPY);
        set_opcode_seam(data, op, OpCode::CPUI_COPY);
        // data.opRemoveInput(op,0);
        data.op_remove_input(op, 0);
        1
    }
}

// =============================================================================
// RuleAndOrLump (ruleaction.cc:404-443)
// =============================================================================

/// Collapse constants in logical expressions: `(V & c) & d => V & (c & d)`
/// (C++ `RuleAndOrLump`).
pub struct RuleAndOrLump {
    group: String,
}

impl RuleAndOrLump {
    /// Constructor (C++ `RuleAndOrLump`, name `"andorlump"`).
    pub fn new(g: impl Into<String>) -> RuleAndOrLump {
        RuleAndOrLump { group: g.into() }
    }
}

impl Rule for RuleAndOrLump {
    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(&self.group) {
            return None;
        }
        Some(Box::new(RuleAndOrLump::new(self.group.clone())))
    }

    fn get_op_list(&self) -> Vec<OpCode> {
        // oplist.push_back(CPUI_INT_AND);
        // oplist.push_back(CPUI_INT_OR);
        // oplist.push_back(CPUI_INT_XOR);
        vec![OpCode::CPUI_INT_AND, OpCode::CPUI_INT_OR, OpCode::CPUI_INT_XOR]
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // opc = op->code();
        let opc = data.obank().get(op).expect("RuleAndOrLump: stale op").code();
        // if (!op->getIn(1)->isConstant()) return 0;
        let op_in1 = data.obank().get(op).expect("RuleAndOrLump: stale op").get_in(1).expect(
            "RuleAndOrLump: op has no in1",
        );
        if !data.vbank().get(op_in1).expect("RuleAndOrLump: stale op_in1").is_constant() {
            return 0;
        }
        // vn1 = op->getIn(0);
        let vn1 = data.obank().get(op).expect("RuleAndOrLump: stale op").get_in(0).expect(
            "RuleAndOrLump: op has no in0",
        );
        // if (!vn1->isWritten()) return 0;
        if !data.vbank().get(vn1).expect("RuleAndOrLump: stale vn1").is_written() {
            return 0;
        }
        // op2 = vn1->getDef();
        let op2 = data.vbank().get(vn1).expect("RuleAndOrLump: stale vn1").get_def().expect(
            "RuleAndOrLump: written vn1 has no def",
        );
        // if (op2->code() != opc) return 0; // Must be same op
        if data.obank().get(op2).expect("RuleAndOrLump: stale op2").code() != opc {
            return 0;
        }
        // if (!op2->getIn(1)->isConstant()) return 0;
        let op2_in1 = data.obank().get(op2).expect("RuleAndOrLump: stale op2").get_in(1).expect(
            "RuleAndOrLump: op2 has no in1",
        );
        if !data.vbank().get(op2_in1).expect("RuleAndOrLump: stale op2_in1").is_constant() {
            return 0;
        }
        // basevn = op2->getIn(0);
        let basevn = data.obank().get(op2).expect("RuleAndOrLump: stale op2").get_in(0).expect(
            "RuleAndOrLump: op2 has no in0",
        );
        // if (basevn->isFree()) return 0;
        if data.vbank().get(basevn).expect("RuleAndOrLump: stale basevn").is_free() {
            return 0;
        }

        // uintb val = op->getIn(1)->getOffset();
        let mut val = data.vbank().get(op_in1).expect("RuleAndOrLump: stale op_in1").get_offset();
        // uintb val2 = op2->getIn(1)->getOffset();
        let val2 = data.vbank().get(op2_in1).expect("RuleAndOrLump: stale op2_in1").get_offset();
        // if (opc == CPUI_INT_AND) val &= val2;
        // else if (opc == CPUI_INT_OR) val |= val2;
        // else if (opc == CPUI_INT_XOR) val ^= val2;
        if opc == OpCode::CPUI_INT_AND {
            val &= val2;
        } else if opc == OpCode::CPUI_INT_OR {
            val |= val2;
        } else if opc == OpCode::CPUI_INT_XOR {
            val ^= val2;
        }

        // data.opSetInput(op,basevn,0);
        data.op_set_input(op, basevn, 0).expect("RuleAndOrLump: op_set_input basevn");
        // data.opSetInput(op,data.newConstant(basevn->getSize(),val),1);
        let basevn_size = data.vbank().get(basevn).expect("RuleAndOrLump: stale basevn").get_size();
        let newc = data.new_constant(basevn_size, val);
        data.op_set_input(op, newc, 1).expect("RuleAndOrLump: op_set_input newc");
        1
    }
}

// =============================================================================
// RuleNegateIdentity (ruleaction.cc:445-475)
// =============================================================================

/// Apply INT_NEGATE identities: `V & ~V => #0, V | ~V -> #-1` (C++
/// `RuleNegateIdentity`).
pub struct RuleNegateIdentity {
    group: String,
}

impl RuleNegateIdentity {
    /// Constructor (C++ `RuleNegateIdentity`, name `"negateidentity"`).
    pub fn new(g: impl Into<String>) -> RuleNegateIdentity {
        RuleNegateIdentity { group: g.into() }
    }
}

impl Rule for RuleNegateIdentity {
    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(&self.group) {
            return None;
        }
        Some(Box::new(RuleNegateIdentity::new(self.group.clone())))
    }

    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_INT_NEGATE]
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // Varnode *vn = op->getIn(0);
        let vn = data.obank().get(op).expect("RuleNegateIdentity: stale op").get_in(0).expect(
            "RuleNegateIdentity: NEGATE has no in0",
        );
        // Varnode *outVn = op->getOut();
        let out_vn = data.obank().get(op).expect("RuleNegateIdentity: stale op").get_out().expect(
            "RuleNegateIdentity: NEGATE has no output",
        );
        // for(iter=outVn->beginDescend();iter!=outVn->endDescend();++iter) {
        let descs: Vec<OpId> =
            data.vbank().get(out_vn).expect("RuleNegateIdentity: stale outVn").descend_iter().collect();
        for logic_op in descs {
            // OpCode opc = logicOp->code();
            let opc = data.obank().get(logic_op).expect("RuleNegateIdentity: stale logicOp").code();
            // if (opc != CPUI_INT_AND && opc != CPUI_INT_OR && opc != CPUI_INT_XOR) continue;
            if opc != OpCode::CPUI_INT_AND
                && opc != OpCode::CPUI_INT_OR
                && opc != OpCode::CPUI_INT_XOR
            {
                continue;
            }
            // int4 slot = logicOp->getSlot(outVn);
            let slot = data.obank().get(logic_op).expect("RuleNegateIdentity: stale logicOp").get_slot(out_vn);
            // if (logicOp->getIn(1-slot) != vn) continue;
            let other = data.obank().get(logic_op).expect("RuleNegateIdentity: stale logicOp").get_in(1 - slot);
            if other != Some(vn) {
                continue;
            }
            // uintb value = 0; if (opc != CPUI_INT_AND) value = calc_mask(vn->getSize());
            let vn_size = data.vbank().get(vn).expect("RuleNegateIdentity: stale vn").get_size();
            let value = if opc != OpCode::CPUI_INT_AND { calc_mask(vn_size) } else { 0 };
            // data.opSetInput(logicOp,data.newConstant(vn->getSize(),value),0);
            let newc = data.new_constant(vn_size, value);
            data.op_set_input(logic_op, newc, 0).expect("RuleNegateIdentity: op_set_input");
            // data.opRemoveInput(logicOp,1);
            data.op_remove_input(logic_op, 1);
            // data.opSetOpcode(logicOp,CPUI_COPY);
            set_opcode_seam(data, logic_op, OpCode::CPUI_COPY);
            return 1;
        }
        0
    }
}

// =============================================================================
// RuleShiftBitops (ruleaction.cc:477-567)
// =============================================================================

/// Shifting away all non-zero bits of one side of a logical/arithmetic op:
/// `( V & 0xf000 ) << 4 => #0 << 4`, `( V + 0xf000 ) << 4 => V << 4`
/// (C++ `RuleShiftBitops`).
pub struct RuleShiftBitops {
    group: String,
}

impl RuleShiftBitops {
    /// Constructor (C++ `RuleShiftBitops`, name `"shiftbitops"`).
    pub fn new(g: impl Into<String>) -> RuleShiftBitops {
        RuleShiftBitops { group: g.into() }
    }
}

impl Rule for RuleShiftBitops {
    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(&self.group) {
            return None;
        }
        Some(Box::new(RuleShiftBitops::new(self.group.clone())))
    }

    fn get_op_list(&self) -> Vec<OpCode> {
        // oplist.push_back(CPUI_INT_LEFT); CPUI_INT_RIGHT; CPUI_SUBPIECE; CPUI_INT_MULT;
        vec![
            OpCode::CPUI_INT_LEFT,
            OpCode::CPUI_INT_RIGHT,
            OpCode::CPUI_SUBPIECE,
            OpCode::CPUI_INT_MULT,
        ]
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // Varnode *constvn = op->getIn(1);
        let constvn = data.obank().get(op).expect("RuleShiftBitops: stale op").get_in(1).expect(
            "RuleShiftBitops: op has no in1",
        );
        // if (!constvn->isConstant()) return 0;	// Must be a constant shift
        if !data.vbank().get(constvn).expect("RuleShiftBitops: stale constvn").is_constant() {
            return 0;
        }
        // Varnode *vn = op->getIn(0);
        let mut vn = data.obank().get(op).expect("RuleShiftBitops: stale op").get_in(0).expect(
            "RuleShiftBitops: op has no in0",
        );
        // if (!vn->isWritten()) return 0;
        if !data.vbank().get(vn).expect("RuleShiftBitops: stale vn").is_written() {
            return 0;
        }
        // if (vn->getSize() > sizeof(uintb)) return 0;	// FIXME: Can't exceed uintb precision
        let vn_size = data.vbank().get(vn).expect("RuleShiftBitops: stale vn").get_size();
        if vn_size > SIZEOF_UINTB {
            return 0;
        }
        let sa: int4;
        let leftshift: bool;
        let op_code = data.obank().get(op).expect("RuleShiftBitops: stale op").code();
        let const_off =
            data.vbank().get(constvn).expect("RuleShiftBitops: stale constvn").get_offset();
        // switch(op->code()) {
        match op_code {
            // case CPUI_INT_LEFT: sa = (int4) constvn->getOffset(); leftshift = true; break;
            OpCode::CPUI_INT_LEFT => {
                sa = const_off as int4;
                leftshift = true;
            }
            // case CPUI_INT_RIGHT: sa = (int4) constvn->getOffset(); leftshift = false; break;
            OpCode::CPUI_INT_RIGHT => {
                sa = const_off as int4;
                leftshift = false;
            }
            // case CPUI_SUBPIECE: sa = (int4) constvn->getOffset(); sa = sa * 8; leftshift = false; break;
            OpCode::CPUI_SUBPIECE => {
                sa = (const_off as int4) * 8;
                leftshift = false;
            }
            // case CPUI_INT_MULT: sa = leastsigbit_set(constvn->getOffset());
            //   if (sa == -1) return 0; leftshift = true; break;
            OpCode::CPUI_INT_MULT => {
                sa = leastsigbit_set(const_off);
                if sa == -1 {
                    return 0;
                }
                leftshift = true;
            }
            // default: return 0;			// Never reaches here
            _ => return 0,
        }

        // PcodeOp *bitop = vn->getDef();
        let bitop = data.vbank().get(vn).expect("RuleShiftBitops: stale vn").get_def().expect(
            "RuleShiftBitops: written vn has no def",
        );
        let bitop_code = data.obank().get(bitop).expect("RuleShiftBitops: stale bitop").code();
        // switch(bitop->code()) {
        match bitop_code {
            // case CPUI_INT_AND: case CPUI_INT_OR: case CPUI_INT_XOR: break;
            OpCode::CPUI_INT_AND | OpCode::CPUI_INT_OR | OpCode::CPUI_INT_XOR => {}
            // case CPUI_INT_MULT: case CPUI_INT_ADD: if (!leftshift) return 0; break;
            OpCode::CPUI_INT_MULT | OpCode::CPUI_INT_ADD => {
                if !leftshift {
                    return 0;
                }
            }
            // default: return 0;
            _ => return 0,
        }

        // int4 i; for(i=0;i<bitop->numInput();++i) {
        let outvn = data.obank().get(op).expect("RuleShiftBitops: stale op").get_out().expect(
            "RuleShiftBitops: op has no output",
        );
        let numinput = data.obank().get(bitop).expect("RuleShiftBitops: stale bitop").num_input();
        let mut i = 0;
        while i < numinput {
            // uintb nzm = bitop->getIn(i)->getNZMask();
            let bin = data.obank().get(bitop).expect("RuleShiftBitops: stale bitop").get_in(i)
                .expect("RuleShiftBitops: bitop input null");
            let mut nzm = data.vbank().get(bin).expect("RuleShiftBitops: stale bitop in").get_nz_mask();
            // uintb mask = calc_mask(op->getOut()->getSize());
            let out_size = data.vbank().get(outvn).expect("RuleShiftBitops: stale outvn").get_size();
            let mask = calc_mask(out_size);
            // if (leftshift) nzm = pcode_left(nzm,sa); else nzm = pcode_right(nzm,sa);
            if leftshift {
                nzm = pcode_left(nzm, sa);
            } else {
                nzm = pcode_right(nzm, sa);
            }
            // if ((nzm&mask)==(uintb)0) break;
            if (nzm & mask) == 0 {
                break;
            }
            i += 1;
        }
        // if (i==bitop->numInput()) return 0;
        if i == numinput {
            return 0;
        }
        // switch(bitop->code()) {
        match bitop_code {
            // case CPUI_INT_MULT: case CPUI_INT_AND:
            //   vn = data.newConstant(vn->getSize(),0); data.opSetInput(op,vn,0); break;
            OpCode::CPUI_INT_MULT | OpCode::CPUI_INT_AND => {
                let vn_size = data.vbank().get(vn).expect("RuleShiftBitops: stale vn").get_size();
                vn = data.new_constant(vn_size, 0);
                data.op_set_input(op, vn, 0).expect("RuleShiftBitops: op_set_input const");
            }
            // case CPUI_INT_ADD: case CPUI_INT_XOR: case CPUI_INT_OR:
            //   vn = bitop->getIn(1-i); if (!vn->isHeritageKnown()) return 0;
            //   data.opSetInput(op,vn,0); break;
            OpCode::CPUI_INT_ADD | OpCode::CPUI_INT_XOR | OpCode::CPUI_INT_OR => {
                vn = data.obank().get(bitop).expect("RuleShiftBitops: stale bitop").get_in(1 - i)
                    .expect("RuleShiftBitops: bitop in (1-i) null");
                if !data.vbank().get(vn).expect("RuleShiftBitops: stale vn").is_heritage_known() {
                    return 0;
                }
                data.op_set_input(op, vn, 0).expect("RuleShiftBitops: op_set_input vn");
            }
            // default: break;
            _ => {}
        }
        1
    }
}

// =============================================================================
// RuleRightShiftAnd (ruleaction.cc:569-601)
// =============================================================================

/// Simplify INT_RIGHT and INT_SRIGHT where an INT_AND mask becomes unnecessary:
/// `( V & 0xf000 ) >> 24 => V >> 24` (C++ `RuleRightShiftAnd`).
pub struct RuleRightShiftAnd {
    group: String,
}

impl RuleRightShiftAnd {
    /// Constructor (C++ `RuleRightShiftAnd`, name `"rightshiftand"`).
    pub fn new(g: impl Into<String>) -> RuleRightShiftAnd {
        RuleRightShiftAnd { group: g.into() }
    }
}

impl Rule for RuleRightShiftAnd {
    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(&self.group) {
            return None;
        }
        Some(Box::new(RuleRightShiftAnd::new(self.group.clone())))
    }

    fn get_op_list(&self) -> Vec<OpCode> {
        // oplist.push_back(CPUI_INT_RIGHT); oplist.push_back(CPUI_INT_SRIGHT);
        vec![OpCode::CPUI_INT_RIGHT, OpCode::CPUI_INT_SRIGHT]
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // Varnode *constVn = op->getIn(1);
        let const_vn = data.obank().get(op).expect("RuleRightShiftAnd: stale op").get_in(1).expect(
            "RuleRightShiftAnd: op has no in1",
        );
        // if (!constVn->isConstant()) return 0;
        if !data.vbank().get(const_vn).expect("RuleRightShiftAnd: stale constVn").is_constant() {
            return 0;
        }
        // Varnode *inVn = op->getIn(0);
        let in_vn = data.obank().get(op).expect("RuleRightShiftAnd: stale op").get_in(0).expect(
            "RuleRightShiftAnd: op has no in0",
        );
        // if (!inVn->isWritten()) return 0;
        if !data.vbank().get(in_vn).expect("RuleRightShiftAnd: stale inVn").is_written() {
            return 0;
        }
        // PcodeOp *andOp = inVn->getDef();
        let and_op = data.vbank().get(in_vn).expect("RuleRightShiftAnd: stale inVn").get_def().expect(
            "RuleRightShiftAnd: written inVn has no def",
        );
        // if (andOp->code() != CPUI_INT_AND) return 0;
        if data.obank().get(and_op).expect("RuleRightShiftAnd: stale andOp").code()
            != OpCode::CPUI_INT_AND
        {
            return 0;
        }
        // Varnode *maskVn = andOp->getIn(1);
        let mask_vn = data.obank().get(and_op).expect("RuleRightShiftAnd: stale andOp").get_in(1).expect(
            "RuleRightShiftAnd: andOp has no in1",
        );
        // if (!maskVn->isConstant()) return 0;
        if !data.vbank().get(mask_vn).expect("RuleRightShiftAnd: stale maskVn").is_constant() {
            return 0;
        }

        // int4 sa = (int4)constVn->getOffset();
        let sa = data.vbank().get(const_vn).expect("RuleRightShiftAnd: stale constVn").get_offset() as int4;
        // uintb mask = maskVn->getOffset() >> sa;
        // sa is a data-derived p-code shift count that can be >= 64 (or negative
        // after the (int4) truncation); the C++ raw `>>` on the x86-64 reference
        // build masks the count to 6 bits (`>> 64` behaves like `>> 0`), so use
        // wshr (wrapping_shr, count taken mod 64) to replicate that exactly.
        let mask = data.vbank().get(mask_vn).expect("RuleRightShiftAnd: stale maskVn").get_offset().wshr(sa as u32);
        // Varnode *rootVn = andOp->getIn(0);
        let root_vn = data.obank().get(and_op).expect("RuleRightShiftAnd: stale andOp").get_in(0).expect(
            "RuleRightShiftAnd: andOp has no in0",
        );
        // uintb full = calc_mask(rootVn->getSize()) >> sa;
        let root_size = data.vbank().get(root_vn).expect("RuleRightShiftAnd: stale rootVn").get_size();
        let full = calc_mask(root_size).wshr(sa as u32);
        // if (full != mask) return 0;
        if full != mask {
            return 0;
        }
        // if (rootVn->isFree()) return 0;
        if data.vbank().get(root_vn).expect("RuleRightShiftAnd: stale rootVn").is_free() {
            return 0;
        }
        // data.opSetInput(op, rootVn, 0);	// Bypass the INT_AND
        data.op_set_input(op, root_vn, 0).expect("RuleRightShiftAnd: op_set_input");
        1
    }
}

// =============================================================================
// RuleIntLessEqual (ruleaction.cc:603-618)
// =============================================================================

/// Convert LESSEQUAL to LESS: `V <= c => V < (c+1)` (C++ `RuleIntLessEqual`).
pub struct RuleIntLessEqual {
    group: String,
}

impl RuleIntLessEqual {
    /// Constructor (C++ `RuleIntLessEqual`, name `"intlessequal"`).
    pub fn new(g: impl Into<String>) -> RuleIntLessEqual {
        RuleIntLessEqual { group: g.into() }
    }
}

impl Rule for RuleIntLessEqual {
    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(&self.group) {
            return None;
        }
        Some(Box::new(RuleIntLessEqual::new(self.group.clone())))
    }

    fn get_op_list(&self) -> Vec<OpCode> {
        // oplist.push_back(CPUI_INT_LESSEQUAL); oplist.push_back(CPUI_INT_SLESSEQUAL);
        vec![OpCode::CPUI_INT_LESSEQUAL, OpCode::CPUI_INT_SLESSEQUAL]
    }

    fn apply_op(&mut self, op: OpId, _data: &mut Funcdata) -> int4 {
        // if (data.replaceLessequal(op)) return 1; return 0;
        // SEAM(funcdata compare-form): `Funcdata::replaceLessequal` (funcdata.cc /
        // kuna_compareform) is not yet ported.  The whole body is that one call,
        // so the rule defers entirely.  No change.
        let _ = op;
        0
    }
}

// =============================================================================
// RuleEquality (ruleaction.cc:620-644)
// =============================================================================

/// Collapse INT_EQUAL and INT_NOTEQUAL: `f(V,W) == f(V,W) => true` (C++
/// `RuleEquality`).
pub struct RuleEquality {
    group: String,
}

impl RuleEquality {
    /// Constructor (C++ `RuleEquality`, name `"equality"`).
    pub fn new(g: impl Into<String>) -> RuleEquality {
        RuleEquality { group: g.into() }
    }
}

impl Rule for RuleEquality {
    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(&self.group) {
            return None;
        }
        Some(Box::new(RuleEquality::new(self.group.clone())))
    }

    fn get_op_list(&self) -> Vec<OpCode> {
        // oplist.push_back(CPUI_INT_EQUAL); oplist.push_back(CPUI_INT_NOTEQUAL);
        vec![OpCode::CPUI_INT_EQUAL, OpCode::CPUI_INT_NOTEQUAL]
    }

    fn apply_op(&mut self, op: OpId, _data: &mut Funcdata) -> int4 {
        // if (!functionalEquality(op->getIn(0),op->getIn(1))) return 0;
        //   ... opSetOpcode(COPY); opRemoveInput(1); newConstant; opSetInput ...
        // SEAM(expression): `functionalEquality` lives in expression.cc (not yet
        // ported), so the gating predicate is unavailable and the rule defers
        // before any mutation.  No change.
        let _ = op;
        0
    }
}

// =============================================================================
// RuleTermOrder (ruleaction.cc:646-675) — Reversible rules
// =============================================================================

/// Order the inputs to commutative operations (constants come last)
/// (C++ `RuleTermOrder`).
pub struct RuleTermOrder {
    group: String,
}

impl RuleTermOrder {
    /// Constructor (C++ `RuleTermOrder`, name `"termorder"`).
    pub fn new(g: impl Into<String>) -> RuleTermOrder {
        RuleTermOrder { group: g.into() }
    }
}

impl Rule for RuleTermOrder {
    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(&self.group) {
            return None;
        }
        Some(Box::new(RuleTermOrder::new(self.group.clone())))
    }

    fn get_op_list(&self) -> Vec<OpCode> {
        // FIXME:  All the commutative ops -- Use the TypeOp::commutative function
        // uint4 list[]={ CPUI_INT_EQUAL, CPUI_INT_NOTEQUAL, CPUI_INT_ADD, CPUI_INT_CARRY,
        //   CPUI_INT_SCARRY, CPUI_INT_XOR, CPUI_INT_AND, CPUI_INT_OR, CPUI_INT_MULT,
        //   CPUI_BOOL_XOR, CPUI_BOOL_AND, CPUI_BOOL_OR, CPUI_FLOAT_EQUAL,
        //   CPUI_FLOAT_NOTEQUAL, CPUI_FLOAT_ADD, CPUI_FLOAT_MULT };
        vec![
            OpCode::CPUI_INT_EQUAL,
            OpCode::CPUI_INT_NOTEQUAL,
            OpCode::CPUI_INT_ADD,
            OpCode::CPUI_INT_CARRY,
            OpCode::CPUI_INT_SCARRY,
            OpCode::CPUI_INT_XOR,
            OpCode::CPUI_INT_AND,
            OpCode::CPUI_INT_OR,
            OpCode::CPUI_INT_MULT,
            OpCode::CPUI_BOOL_XOR,
            OpCode::CPUI_BOOL_AND,
            OpCode::CPUI_BOOL_OR,
            OpCode::CPUI_FLOAT_EQUAL,
            OpCode::CPUI_FLOAT_NOTEQUAL,
            OpCode::CPUI_FLOAT_ADD,
            OpCode::CPUI_FLOAT_MULT,
        ]
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // Varnode *vn1 = op->getIn(0);  Varnode *vn2 = op->getIn(1);
        let vn1 = data.obank().get(op).expect("RuleTermOrder: stale op").get_in(0).expect(
            "RuleTermOrder: op has no in0",
        );
        let vn2 = data.obank().get(op).expect("RuleTermOrder: stale op").get_in(1).expect(
            "RuleTermOrder: op has no in1",
        );
        // if (vn1->isConstant() && (!vn2->isConstant())) {
        let vn1_const = data.vbank().get(vn1).expect("RuleTermOrder: stale vn1").is_constant();
        let vn2_const = data.vbank().get(vn2).expect("RuleTermOrder: stale vn2").is_constant();
        if vn1_const && !vn2_const {
            //   data.opSwapInput(op,0,1);	// Reverse the order of the terms
            data.op_swap_input(op, 0, 1);
            return 1;
        }
        0
    }
}

// =============================================================================
// RulePullsubMulti (ruleaction.cc:677-953) — and its shared statics
// =============================================================================

/// Pull SUBPIECE back through MULTIEQUAL (C++ `RulePullsubMulti`).
///
/// The static helpers `min_max_use`, `replace_descendants`, `accept_size`,
/// `build_subpiece`, `find_subpiece` are shared with [`RulePullsubIndirect`]
/// (which lives in this same batch and calls them); the C++ defines them on
/// `RulePullsubMulti` so they stay here as associated functions.
pub struct RulePullsubMulti {
    group: String,
}

impl RulePullsubMulti {
    /// Constructor (C++ `RulePullsubMulti`, name `"pullsub_multi"`).
    pub fn new(g: impl Into<String>) -> RulePullsubMulti {
        RulePullsubMulti { group: g.into() }
    }

    /// Compute minimum and maximum bytes being used (C++
    /// `RulePullsubMulti::minMaxUse`).  Reads only the IR, so it is portable.
    pub fn min_max_use(data: &Funcdata, vn: VarnodeId, max_byte: &mut int4, min_byte: &mut int4) {
        // int4 inSize = vn->getSize();
        let in_size = data.vbank().get(vn).expect("minMaxUse: stale vn").get_size();
        // maxByte = -1; minByte = inSize;
        *max_byte = -1;
        *min_byte = in_size;
        // for(iter=vn->beginDescend();iter!=enditer;++iter) {
        let descs: Vec<OpId> = data.vbank().get(vn).expect("minMaxUse: stale vn").descend_iter().collect();
        for op in descs {
            // OpCode opc = op->code();
            let opc = data.obank().get(op).expect("minMaxUse: stale op").code();
            // if (opc == CPUI_SUBPIECE) {
            if opc == OpCode::CPUI_SUBPIECE {
                // int4 min = (int4)op->getIn(1)->getOffset();
                let in1 = data.obank().get(op).expect("minMaxUse: stale op").get_in(1).expect(
                    "minMaxUse: SUBPIECE has no in1",
                );
                let min = data.vbank().get(in1).expect("minMaxUse: stale in1").get_offset() as int4;
                // int4 max = min + op->getOut()->getSize() - 1;
                let outvn = data.obank().get(op).expect("minMaxUse: stale op").get_out().expect(
                    "minMaxUse: SUBPIECE has no output",
                );
                let outsize = data.vbank().get(outvn).expect("minMaxUse: stale outvn").get_size();
                let max = min + outsize - 1;
                // if (min < minByte) minByte = min;
                if min < *min_byte {
                    *min_byte = min;
                }
                // if (max > maxByte) maxByte = max;
                if max > *max_byte {
                    *max_byte = max;
                }
            } else {
                // else { // By default assume all bytes are used
                //   maxByte = inSize - 1; minByte = 0; return; }
                *max_byte = in_size - 1;
                *min_byte = 0;
                return;
            }
        }
    }

    /// Return \b true if given size is a suitable truncated size (C++
    /// `RulePullsubMulti::acceptableSize`).
    pub fn acceptable_size(size: int4) -> bool {
        // if (size == 0) return false;
        if size == 0 {
            return false;
        }
        // if (size >= 8) return true;
        if size >= 8 {
            return true;
        }
        // if (size == 1 || size == 2 || size == 4 || size == 8) return true;
        if size == 1 || size == 2 || size == 4 || size == 8 {
            return true;
        }
        // return false;
        false
    }
}

impl Rule for RulePullsubMulti {
    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(&self.group) {
            return None;
        }
        Some(Box::new(RulePullsubMulti::new(self.group.clone())))
    }

    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_SUBPIECE]
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        let mut max_byte: int4 = 0;
        let mut min_byte: int4 = 0;

        // Varnode *vn = op->getIn(0);
        let vn = data.obank().get(op).expect("RulePullsubMulti: stale op").get_in(0).expect(
            "RulePullsubMulti: SUBPIECE has no in0",
        );
        // if (!vn->isWritten()) return 0;
        if !data.vbank().get(vn).expect("RulePullsubMulti: stale vn").is_written() {
            return 0;
        }
        // PcodeOp *mult = vn->getDef();
        let mult = data.vbank().get(vn).expect("RulePullsubMulti: stale vn").get_def().expect(
            "RulePullsubMulti: written vn has no def",
        );
        // if (mult->code()!=CPUI_MULTIEQUAL) return 0;
        if data.obank().get(mult).expect("RulePullsubMulti: stale mult").code()
            != OpCode::CPUI_MULTIEQUAL
        {
            return 0;
        }
        // We only pull up, do not pull "down" to bottom of loop
        // if (mult->getParent()->hasLoopIn()) return 0;
        let parent = data.obank().get(mult).expect("RulePullsubMulti: stale mult").get_parent().expect(
            "RulePullsubMulti: mult has no parent",
        );
        if data.bblocks_ref().block(parent).has_loop_in() {
            return 0;
        }
        // minMaxUse(vn, maxByte, minByte);		// Figure out what part of -vn- is used
        RulePullsubMulti::min_max_use(data, vn, &mut max_byte, &mut min_byte);
        // newSize = maxByte - minByte + 1;
        let new_size = max_byte - min_byte + 1;
        // if (maxByte < minByte || (newSize >= vn->getSize())) return 0;
        let vn_size = data.vbank().get(vn).expect("RulePullsubMulti: stale vn").get_size();
        if max_byte < min_byte || new_size >= vn_size {
            return 0;
        }
        // if (!acceptableSize(newSize)) return 0;
        if !RulePullsubMulti::acceptable_size(new_size) {
            return 0;
        }
        // Varnode *outvn = op->getOut();
        let outvn = data.obank().get(op).expect("RulePullsubMulti: stale op").get_out().expect(
            "RulePullsubMulti: SUBPIECE has no output",
        );
        // if (outvn->isPrecisLo()||outvn->isPrecisHi()) return 0; // Don't pull apart double precision
        let ov = data.vbank().get(outvn).expect("RulePullsubMulti: stale outvn");
        if ov.is_precis_lo() || ov.is_precis_hi() {
            return 0;
        }

        // Make sure we don't add new SUBPIECE ops that aren't going to cancel in some way
        // if (minByte > sizeof(uintb)) return 0;
        if min_byte > SIZEOF_UINTB {
            return 0;
        }
        // uintb consume; if (minByte < sizeof(uintb)) consume = calc_mask(newSize) << 8*minByte; else consume = 0;
        let mut consume: uintb = if min_byte < SIZEOF_UINTB {
            calc_mask(new_size) << (8 * min_byte)
        } else {
            0
        };
        // consume = ~consume;			// Check for use of bits outside of what gets truncated later
        consume = !consume;
        // int4 branches = mult->numInput();
        let branches = data.obank().get(mult).expect("RulePullsubMulti: stale mult").num_input();
        // for(int4 i=0;i<branches;++i) {
        for i in 0..branches {
            // Varnode *inVn = mult->getIn(i);
            let in_vn = data.obank().get(mult).expect("RulePullsubMulti: stale mult").get_in(i).expect(
                "RulePullsubMulti: mult input null",
            );
            // if ((consume & inVn->getConsume()) != 0) {	// Check if bits not truncated are still used
            let in_consume = data.vbank().get(in_vn).expect("RulePullsubMulti: stale inVn").get_consume();
            if (consume & in_consume) != 0 {
                // Check if there's an extension that matches the truncation
                // if (minByte == 0 && inVn->isWritten()) {
                let in_written = data.vbank().get(in_vn).expect("RulePullsubMulti: stale inVn").is_written();
                if min_byte == 0 && in_written {
                    // PcodeOp *defOp = inVn->getDef(); OpCode opc = defOp->code();
                    let def_op = data.vbank().get(in_vn).expect("RulePullsubMulti: stale inVn").get_def()
                        .expect("RulePullsubMulti: written inVn has no def");
                    let opc = data.obank().get(def_op).expect("RulePullsubMulti: stale defOp").code();
                    // if (opc == CPUI_INT_ZEXT || opc == CPUI_INT_SEXT) {
                    if opc == OpCode::CPUI_INT_ZEXT || opc == OpCode::CPUI_INT_SEXT {
                        // if (newSize == defOp->getIn(0)->getSize())
                        let def_in0 = data.obank().get(def_op).expect("RulePullsubMulti: stale defOp").get_in(0)
                            .expect("RulePullsubMulti: ext has no in0");
                        let def_in0_size = data.vbank().get(def_in0).expect("RulePullsubMulti: stale def_in0").get_size();
                        if new_size == def_in0_size {
                            // continue; // We have matching extension, so new SUBPIECE will cancel anyway
                            continue;
                        }
                    }
                }
                // return 0;
                return 0;
            }
        }

        // ... smalladdr2; build new SUBPIECE per branch (findSubpiece/buildSubpiece);
        //     newOp + newVarnodeOut MULTIEQUAL; opSetAllInput; opInsertBegin;
        //     replaceDescendants(...);
        // SEAM(W3/W5): `buildSubpiece`/`findSubpiece`/`replaceDescendants` create
        // new SUBPIECE/MULTIEQUAL ops with outputs (`newVarnodeOut`/`newUniqueOut`
        // — the W3 output-creation seam) and `buildSubpiece` also calls
        // `glb->findJoin` (the W5 join-record table).  Every early-out guard above
        // is ported faithfully; the construction body defers here.  No change.
        0
    }
}

// =============================================================================
// RulePullsubIndirect (ruleaction.cc:955-1020)
// =============================================================================

/// Pull-back SUBPIECE through INDIRECT (C++ `RulePullsubIndirect`).
pub struct RulePullsubIndirect {
    group: String,
}

impl RulePullsubIndirect {
    /// Constructor (C++ `RulePullsubIndirect`, name `"pullsub_indirect"`).
    pub fn new(g: impl Into<String>) -> RulePullsubIndirect {
        RulePullsubIndirect { group: g.into() }
    }
}

impl Rule for RulePullsubIndirect {
    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(&self.group) {
            return None;
        }
        Some(Box::new(RulePullsubIndirect::new(self.group.clone())))
    }

    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_SUBPIECE]
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // Varnode *vn = op->getIn(0);
        let vn = data.obank().get(op).expect("RulePullsubIndirect: stale op").get_in(0).expect(
            "RulePullsubIndirect: SUBPIECE has no in0",
        );
        // if (!vn->isWritten()) return 0;
        if !data.vbank().get(vn).expect("RulePullsubIndirect: stale vn").is_written() {
            return 0;
        }
        // if (vn->getSize() > sizeof(uintb)) return 0;
        let vn_size = data.vbank().get(vn).expect("RulePullsubIndirect: stale vn").get_size();
        if vn_size > SIZEOF_UINTB {
            return 0;
        }
        // PcodeOp *indir = vn->getDef();
        let indir = data.vbank().get(vn).expect("RulePullsubIndirect: stale vn").get_def().expect(
            "RulePullsubIndirect: written vn has no def",
        );
        // if (indir->code()!=CPUI_INDIRECT) return 0;
        if data.obank().get(indir).expect("RulePullsubIndirect: stale indir").code()
            != OpCode::CPUI_INDIRECT
        {
            return 0;
        }
        // if (indir->getIn(1)->getSpace()->getType()!=IPTR_IOP) return 0;
        let indir_in1 = data.obank().get(indir).expect("RulePullsubIndirect: stale indir").get_in(1).expect(
            "RulePullsubIndirect: INDIRECT has no in1",
        );
        let in1_space_type = data
            .vbank()
            .get(indir_in1)
            .expect("RulePullsubIndirect: stale indir_in1")
            .get_space()
            .get_type();
        if in1_space_type != kuna_base::space::spacetype::IPTR_IOP {
            return 0;
        }

        // PcodeOp *targ_op = PcodeOp::getOpFromConst(indir->getIn(1)->getAddr());
        // if (targ_op->isDead()) return 0;
        //   ... newSize/minMaxUse/acceptableSize/consume guards; then
        //   newIndirectCreation or newOp(INDIRECT)+newVarnodeOut; replaceDescendants ...
        // SEAM(W3/W4): `PcodeOp::getOpFromConst` (iop-space decode — the W3-varnode
        // seam funcdata_op also notes) and `newIndirectCreation`/`newVarnodeOut`
        // (W3 output-creation seam) are unavailable.  The INDIRECT/iop structural
        // guards above are ported; the body defers here.  No change.
        0
    }
}

// =============================================================================
// RulePushMulti (ruleaction.cc:1022-1138) — and its shared static findSubstitute
// =============================================================================

/// Simplify MULTIEQUAL where both branches hold the same value (C++
/// `RulePushMulti`).
pub struct RulePushMulti {
    group: String,
}

impl RulePushMulti {
    /// Constructor (C++ `RulePushMulti`, name `"push_multi"`).
    pub fn new(g: impl Into<String>) -> RulePushMulti {
        RulePushMulti { group: g.into() }
    }
}

impl Rule for RulePushMulti {
    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(&self.group) {
            return None;
        }
        Some(Box::new(RulePushMulti::new(self.group.clone())))
    }

    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_MULTIEQUAL]
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // if (op->numInput() != 2) return 0;
        if data.obank().get(op).expect("RulePushMulti: stale op").num_input() != 2 {
            return 0;
        }
        // Varnode *in1 = op->getIn(0);  Varnode *in2 = op->getIn(1);
        let in1 = data.obank().get(op).expect("RulePushMulti: stale op").get_in(0).expect(
            "RulePushMulti: MULTIEQUAL has no in0",
        );
        let in2 = data.obank().get(op).expect("RulePushMulti: stale op").get_in(1).expect(
            "RulePushMulti: MULTIEQUAL has no in1",
        );
        // if (!in1->isWritten()) return 0;
        if !data.vbank().get(in1).expect("RulePushMulti: stale in1").is_written() {
            return 0;
        }
        // if (!in2->isWritten()) return 0;
        if !data.vbank().get(in2).expect("RulePushMulti: stale in2").is_written() {
            return 0;
        }
        // if (in1->isSpacebase()) return 0;
        if data.vbank().get(in1).expect("RulePushMulti: stale in1").is_spacebase() {
            return 0;
        }
        // if (in2->isSpacebase()) return 0;
        if data.vbank().get(in2).expect("RulePushMulti: stale in2").is_spacebase() {
            return 0;
        }

        // int4 res = functionalEqualityLevel(in1,in2,buf1,buf2);
        //   ... the rest builds/relocates ops (opSetOutput, opUninsert, newOp,
        //   newVarnodeOut/newUniqueOut, findSubstitute -> cseFindInBlock,
        //   earliestUse, opDestroy) ...
        // SEAM(expression/W3/block): `functionalEqualityLevel` (expression.cc),
        // `findSubstitute` -> `cseFindInBlock`/`earliestUse`, and the op
        // relocation (`opSetOutput` + `newVarnodeOut`/`newUniqueOut`, the W3
        // output-creation seam) are all unavailable.  The two-branch / written /
        // spacebase guards above are ported; the body defers here.  No change.
        0
    }
}

// =============================================================================
// RuleNotDistribute (ruleaction.cc:1140-1184)
// =============================================================================

/// Distribute BOOL_NEGATE: `!(V && W) => !V || !W` (C++ `RuleNotDistribute`).
pub struct RuleNotDistribute {
    group: String,
}

impl RuleNotDistribute {
    /// Constructor (C++ `RuleNotDistribute`, name `"notdistribute"`).
    pub fn new(g: impl Into<String>) -> RuleNotDistribute {
        RuleNotDistribute { group: g.into() }
    }
}

impl Rule for RuleNotDistribute {
    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(&self.group) {
            return None;
        }
        Some(Box::new(RuleNotDistribute::new(self.group.clone())))
    }

    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_BOOL_NEGATE]
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // PcodeOp *compop = op->getIn(0)->getDef();
        let in0 = data.obank().get(op).expect("RuleNotDistribute: stale op").get_in(0).expect(
            "RuleNotDistribute: NEGATE has no in0",
        );
        let compop = data.vbank().get(in0).expect("RuleNotDistribute: stale in0").get_def();
        // if (compop == (PcodeOp *)0) return 0;
        let compop = match compop {
            None => return 0,
            Some(c) => c,
        };
        // switch(compop->code()) {
        //   case CPUI_BOOL_AND: opc = CPUI_BOOL_OR; break;
        //   case CPUI_BOOL_OR: opc = CPUI_BOOL_AND; break;
        //   default: return 0; }
        let comp_code = data.obank().get(compop).expect("RuleNotDistribute: stale compop").code();
        let _opc = match comp_code {
            OpCode::CPUI_BOOL_AND => OpCode::CPUI_BOOL_OR,
            OpCode::CPUI_BOOL_OR => OpCode::CPUI_BOOL_AND,
            _ => return 0,
        };

        // newneg1 = data.newOp(1,op->getAddr()); newout1 = data.newUniqueOut(1,newneg1);
        //   ... build two BOOL_NEGATE ops with unique outputs, retarget op ...
        // SEAM(W3): the two new BOOL_NEGATE ops need `newUniqueOut` (the W3
        // output-creation seam).  The BOOL_AND/BOOL_OR structural match above is
        // ported; the construction body defers here.  No change.
        0
    }
}

// =============================================================================
// RuleHighOrderAnd (ruleaction.cc:1186-1251)
// =============================================================================

/// Simplify INT_AND applied to aligned INT_ADD:
/// `(V + c) & 0xfff0 => V + (c & 0xfff0)` (C++ `RuleHighOrderAnd`).
pub struct RuleHighOrderAnd {
    group: String,
}

impl RuleHighOrderAnd {
    /// Constructor (C++ `RuleHighOrderAnd`, name `"highorderand"`).
    pub fn new(g: impl Into<String>) -> RuleHighOrderAnd {
        RuleHighOrderAnd { group: g.into() }
    }
}

impl Rule for RuleHighOrderAnd {
    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(&self.group) {
            return None;
        }
        Some(Box::new(RuleHighOrderAnd::new(self.group.clone())))
    }

    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_INT_AND]
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // Varnode *cvn1 = op->getIn(1);
        let cvn1 = data.obank().get(op).expect("RuleHighOrderAnd: stale op").get_in(1).expect(
            "RuleHighOrderAnd: INT_AND has no in1",
        );
        // if (!cvn1->isConstant()) return 0;
        if !data.vbank().get(cvn1).expect("RuleHighOrderAnd: stale cvn1").is_constant() {
            return 0;
        }
        // if (!op->getIn(0)->isWritten()) return 0;
        let op_in0 = data.obank().get(op).expect("RuleHighOrderAnd: stale op").get_in(0).expect(
            "RuleHighOrderAnd: INT_AND has no in0",
        );
        if !data.vbank().get(op_in0).expect("RuleHighOrderAnd: stale op_in0").is_written() {
            return 0;
        }
        // PcodeOp *addop = op->getIn(0)->getDef();
        let addop = data.vbank().get(op_in0).expect("RuleHighOrderAnd: stale op_in0").get_def().expect(
            "RuleHighOrderAnd: written in0 has no def",
        );
        // if (addop->code() != CPUI_INT_ADD) return 0;
        if data.obank().get(addop).expect("RuleHighOrderAnd: stale addop").code()
            != OpCode::CPUI_INT_ADD
        {
            return 0;
        }

        // uintb val = cvn1->getOffset();  int4 size = cvn1->getSize();
        let mut val = data.vbank().get(cvn1).expect("RuleHighOrderAnd: stale cvn1").get_offset();
        let size = data.vbank().get(cvn1).expect("RuleHighOrderAnd: stale cvn1").get_size();
        // Check that cvn1 is of the form    11110000
        // if (((val-1)|val) != calc_mask(size)) return 0;
        if (val.wsub(1) | val) != calc_mask(size) {
            return 0;
        }

        // Varnode *cvn2 = addop->getIn(1);
        let cvn2 = data.obank().get(addop).expect("RuleHighOrderAnd: stale addop").get_in(1).expect(
            "RuleHighOrderAnd: INT_ADD has no in1",
        );
        // if (cvn2->isConstant()) {
        if data.vbank().get(cvn2).expect("RuleHighOrderAnd: stale cvn2").is_constant() {
            // xalign = addop->getIn(0);
            let xalign = data.obank().get(addop).expect("RuleHighOrderAnd: stale addop").get_in(0).expect(
                "RuleHighOrderAnd: INT_ADD has no in0",
            );
            // if (xalign->isFree()) return 0;
            if data.vbank().get(xalign).expect("RuleHighOrderAnd: stale xalign").is_free() {
                return 0;
            }
            // uintb mask1 = xalign->getNZMask();
            let mask1 = data.vbank().get(xalign).expect("RuleHighOrderAnd: stale xalign").get_nz_mask();
            // addop->Input(0) must be unaffected by the AND
            // if ((mask1 & val)!=mask1) return 0;
            if (mask1 & val) != mask1 {
                return 0;
            }

            // data.opSetOpcode(op,CPUI_INT_ADD);
            set_opcode_seam(data, op, OpCode::CPUI_INT_ADD);
            // data.opSetInput(op,xalign,0);
            data.op_set_input(op, xalign, 0).expect("RuleHighOrderAnd: op_set_input xalign");
            // val = val & cvn2->getOffset();
            let cvn2_off = data.vbank().get(cvn2).expect("RuleHighOrderAnd: stale cvn2").get_offset();
            val &= cvn2_off;
            // data.opSetInput(op,data.newConstant(size,val),1);
            let newc = data.new_constant(size, val);
            data.op_set_input(op, newc, 1).expect("RuleHighOrderAnd: op_set_input newc");
            1
        } else {
            // else {
            //   if (addop->getOut()->loneDescend() != op) return 0;
            let addop_out = data.obank().get(addop).expect("RuleHighOrderAnd: stale addop").get_out().expect(
                "RuleHighOrderAnd: INT_ADD has no output",
            );
            if data.lone_descend(addop_out) != Some(op) {
                return 0;
            }
            // for(int4 i=0;i<2;++i) {
            for i in 0..2 {
                // Varnode *zerovn = addop->getIn(i);
                let zerovn = data.obank().get(addop).expect("RuleHighOrderAnd: stale addop").get_in(i).expect(
                    "RuleHighOrderAnd: INT_ADD input null",
                );
                // uintb mask2 = zerovn->getNZMask();
                let mask2_z = data.vbank().get(zerovn).expect("RuleHighOrderAnd: stale zerovn").get_nz_mask();
                // if ((mask2 & val)!=mask2) continue; // zerovn must be unaffected by the AND operation
                if (mask2_z & val) != mask2_z {
                    continue;
                }
                // Varnode *nonzerovn = addop->getIn(1-i);
                let nonzerovn = data.obank().get(addop).expect("RuleHighOrderAnd: stale addop").get_in(1 - i).expect(
                    "RuleHighOrderAnd: INT_ADD input null",
                );
                // if (!nonzerovn->isWritten()) continue;
                if !data.vbank().get(nonzerovn).expect("RuleHighOrderAnd: stale nonzerovn").is_written() {
                    continue;
                }
                // PcodeOp *addop2 = nonzerovn->getDef();
                let addop2 = data.vbank().get(nonzerovn).expect("RuleHighOrderAnd: stale nonzerovn").get_def()
                    .expect("RuleHighOrderAnd: written nonzerovn has no def");
                // if (addop2->code() != CPUI_INT_ADD) continue;
                if data.obank().get(addop2).expect("RuleHighOrderAnd: stale addop2").code()
                    != OpCode::CPUI_INT_ADD
                {
                    continue;
                }
                // if (nonzerovn->loneDescend() != addop) continue;
                if data.lone_descend(nonzerovn) != Some(addop) {
                    continue;
                }
                // cvn2 = addop2->getIn(1);
                let cvn2_2 = data.obank().get(addop2).expect("RuleHighOrderAnd: stale addop2").get_in(1).expect(
                    "RuleHighOrderAnd: addop2 has no in1",
                );
                // if (!cvn2->isConstant()) continue;
                if !data.vbank().get(cvn2_2).expect("RuleHighOrderAnd: stale cvn2_2").is_constant() {
                    continue;
                }
                // xalign = addop2->getIn(0);
                let xalign = data.obank().get(addop2).expect("RuleHighOrderAnd: stale addop2").get_in(0).expect(
                    "RuleHighOrderAnd: addop2 has no in0",
                );
                // mask2 = xalign->getNZMask();
                let mask2_x = data.vbank().get(xalign).expect("RuleHighOrderAnd: stale xalign").get_nz_mask();
                // if ((mask2 & val)!=mask2) continue;
                if (mask2_x & val) != mask2_x {
                    continue;
                }
                // val = val & cvn2->getOffset();
                let cvn2_2_off = data.vbank().get(cvn2_2).expect("RuleHighOrderAnd: stale cvn2_2").get_offset();
                val &= cvn2_2_off;
                // data.opSetInput(addop2,data.newConstant(size,val),1);
                let newc = data.new_constant(size, val);
                data.op_set_input(addop2, newc, 1).expect("RuleHighOrderAnd: op_set_input addop2");
                // Convert the AND to a COPY
                // data.opRemoveInput(op,1);
                data.op_remove_input(op, 1);
                // data.opSetOpcode(op,CPUI_COPY);
                set_opcode_seam(data, op, OpCode::CPUI_COPY);
                return 1;
            }
            0
        }
    }
}

// =============================================================================
// RuleAndDistribute (ruleaction.cc:1253-1315)
// =============================================================================

/// Distribute INT_AND through INT_OR if result is simpler (C++
/// `RuleAndDistribute`).
pub struct RuleAndDistribute {
    group: String,
}

impl RuleAndDistribute {
    /// Constructor (C++ `RuleAndDistribute`, name `"anddistribute"`).
    pub fn new(g: impl Into<String>) -> RuleAndDistribute {
        RuleAndDistribute { group: g.into() }
    }
}

impl Rule for RuleAndDistribute {
    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(&self.group) {
            return None;
        }
        Some(Box::new(RuleAndDistribute::new(self.group.clone())))
    }

    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_INT_AND]
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // int4 size = op->getOut()->getSize();
        let outvn = data.obank().get(op).expect("RuleAndDistribute: stale op").get_out().expect(
            "RuleAndDistribute: INT_AND has no output",
        );
        let size = data.vbank().get(outvn).expect("RuleAndDistribute: stale outvn").get_size();
        // if (size > sizeof(uintb)) return 0; // FIXME: uintb should be arbitrary precision
        if size > SIZEOF_UINTB {
            return 0;
        }
        // fullmask = calc_mask(size);
        let fullmask = calc_mask(size);
        // for(i=0;i<2;++i) {
        let mut i = 0;
        while i < 2 {
            // othervn = op->getIn(1-i);
            let othervn = data.obank().get(op).expect("RuleAndDistribute: stale op").get_in(1 - i).expect(
                "RuleAndDistribute: INT_AND input null",
            );
            // if (!othervn->isHeritageKnown()) continue;
            if !data.vbank().get(othervn).expect("RuleAndDistribute: stale othervn").is_heritage_known() {
                i += 1;
                continue;
            }
            // orvn = op->getIn(i);
            let orvn = data.obank().get(op).expect("RuleAndDistribute: stale op").get_in(i).expect(
                "RuleAndDistribute: INT_AND input null",
            );
            // orop = orvn->getDef();
            let orop = data.vbank().get(orvn).expect("RuleAndDistribute: stale orvn").get_def();
            // if (orop == (PcodeOp *)0) continue;
            let orop = match orop {
                None => {
                    i += 1;
                    continue;
                }
                Some(o) => o,
            };
            // if (orop->code() != CPUI_INT_OR) continue;
            if data.obank().get(orop).expect("RuleAndDistribute: stale orop").code()
                != OpCode::CPUI_INT_OR
            {
                i += 1;
                continue;
            }
            // if (!orop->getIn(0)->isHeritageKnown()) continue;
            let orop_in0 = data.obank().get(orop).expect("RuleAndDistribute: stale orop").get_in(0).expect(
                "RuleAndDistribute: INT_OR has no in0",
            );
            if !data.vbank().get(orop_in0).expect("RuleAndDistribute: stale orop_in0").is_heritage_known() {
                i += 1;
                continue;
            }
            // if (!orop->getIn(1)->isHeritageKnown()) continue;
            let orop_in1 = data.obank().get(orop).expect("RuleAndDistribute: stale orop").get_in(1).expect(
                "RuleAndDistribute: INT_OR has no in1",
            );
            if !data.vbank().get(orop_in1).expect("RuleAndDistribute: stale orop_in1").is_heritage_known() {
                i += 1;
                continue;
            }
            // othermask = othervn->getNZMask();
            let othermask = data.vbank().get(othervn).expect("RuleAndDistribute: stale othervn").get_nz_mask();
            // if (othermask == 0) continue; // This case picked up by andmask
            if othermask == 0 {
                i += 1;
                continue;
            }
            // if (othermask == fullmask) continue; // Nothing useful from distributing
            if othermask == fullmask {
                i += 1;
                continue;
            }
            // ormask1 = orop->getIn(0)->getNZMask();
            let ormask1 = data.vbank().get(orop_in0).expect("RuleAndDistribute: stale orop_in0").get_nz_mask();
            // if ((ormask1 & othermask)==0) break; // AND would cancel if distributed
            if (ormask1 & othermask) == 0 {
                break;
            }
            // ormask2 = orop->getIn(1)->getNZMask();
            let ormask2 = data.vbank().get(orop_in1).expect("RuleAndDistribute: stale orop_in1").get_nz_mask();
            // if ((ormask2 & othermask)==0) break; // AND would cancel if distributed
            if (ormask2 & othermask) == 0 {
                break;
            }
            // if (othervn->isConstant()) {
            if data.vbank().get(othervn).expect("RuleAndDistribute: stale othervn").is_constant() {
                // if ((ormask1 & othermask) == ormask1) break; // AND is trivial if distributed
                if (ormask1 & othermask) == ormask1 {
                    break;
                }
                // if ((ormask2 & othermask) == ormask2) break;
                if (ormask2 & othermask) == ormask2 {
                    break;
                }
            }
            i += 1;
        }
        // if (i==2) return 0;
        if i == 2 {
            return 0;
        }
        // Do distribution -- builds two new INT_AND ops with newUniqueOut, then
        // rewires op into an INT_OR of their outputs.
        // SEAM(W3): the two distributed INT_AND ops need `newUniqueOut` (the W3
        // output-creation seam).  Every match/early-out guard above is ported; the
        // distribution body defers here.  No change.
        0
    }
}

// =============================================================================
// RuleLessOne (ruleaction.cc:1317-1340)
// =============================================================================

/// Transform INT_LESS of 0 or 1: `V < 1 => V == 0, V <= 0 => V == 0`
/// (C++ `RuleLessOne`).
pub struct RuleLessOne {
    group: String,
}

impl RuleLessOne {
    /// Constructor (C++ `RuleLessOne`, name `"lessone"`).
    pub fn new(g: impl Into<String>) -> RuleLessOne {
        RuleLessOne { group: g.into() }
    }
}

impl Rule for RuleLessOne {
    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(&self.group) {
            return None;
        }
        Some(Box::new(RuleLessOne::new(self.group.clone())))
    }

    fn get_op_list(&self) -> Vec<OpCode> {
        // oplist.push_back(CPUI_INT_LESS); oplist.push_back(CPUI_INT_LESSEQUAL);
        vec![OpCode::CPUI_INT_LESS, OpCode::CPUI_INT_LESSEQUAL]
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // Varnode *constvn = op->getIn(1);
        let constvn = data.obank().get(op).expect("RuleLessOne: stale op").get_in(1).expect(
            "RuleLessOne: op has no in1",
        );
        // if (!constvn->isConstant()) return 0;
        if !data.vbank().get(constvn).expect("RuleLessOne: stale constvn").is_constant() {
            return 0;
        }
        // uintb val = constvn->getOffset();
        let val = data.vbank().get(constvn).expect("RuleLessOne: stale constvn").get_offset();
        let op_code = data.obank().get(op).expect("RuleLessOne: stale op").code();
        // if ((op->code()==CPUI_INT_LESS)&&(val != 1)) return 0;
        if op_code == OpCode::CPUI_INT_LESS && val != 1 {
            return 0;
        }
        // if ((op->code()==CPUI_INT_LESSEQUAL)&&(val != 0)) return 0;
        if op_code == OpCode::CPUI_INT_LESSEQUAL && val != 0 {
            return 0;
        }

        // data.opSetOpcode(op,CPUI_INT_EQUAL);
        set_opcode_seam(data, op, OpCode::CPUI_INT_EQUAL);
        // if (val != 0) data.opSetInput(op,data.newConstant(constvn->getSize(),0),1);
        if val != 0 {
            let constvn_size = data.vbank().get(constvn).expect("RuleLessOne: stale constvn").get_size();
            let newc = data.new_constant(constvn_size, 0);
            data.op_set_input(op, newc, 1).expect("RuleLessOne: op_set_input");
        }
        1
    }
}

// =============================================================================
// RuleRangeMeld (ruleaction.cc:1342-1438)
// =============================================================================

/// Merge range conditions of the form `V s< c, c s< V, V == c, V != c` via
/// BOOL_AND / BOOL_OR (C++ `RuleRangeMeld`).
pub struct RuleRangeMeld {
    group: String,
}

impl RuleRangeMeld {
    /// Constructor (C++ `RuleRangeMeld`, name `"rangemeld"`).
    pub fn new(g: impl Into<String>) -> RuleRangeMeld {
        RuleRangeMeld { group: g.into() }
    }
}

impl Rule for RuleRangeMeld {
    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(&self.group) {
            return None;
        }
        Some(Box::new(RuleRangeMeld::new(self.group.clone())))
    }

    fn get_op_list(&self) -> Vec<OpCode> {
        // oplist.push_back(CPUI_BOOL_OR); oplist.push_back(CPUI_BOOL_AND);
        vec![OpCode::CPUI_BOOL_OR, OpCode::CPUI_BOOL_AND]
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // vn1 = op->getIn(0);
        let vn1 = data.obank().get(op).expect("RuleRangeMeld: stale op").get_in(0).expect(
            "RuleRangeMeld: op has no in0",
        );
        // if (!vn1->isWritten()) return 0;
        if !data.vbank().get(vn1).expect("RuleRangeMeld: stale vn1").is_written() {
            return 0;
        }
        // vn2 = op->getIn(1);
        let vn2 = data.obank().get(op).expect("RuleRangeMeld: stale op").get_in(1).expect(
            "RuleRangeMeld: op has no in1",
        );
        // if (!vn2->isWritten()) return 0;
        if !data.vbank().get(vn2).expect("RuleRangeMeld: stale vn2").is_written() {
            return 0;
        }
        // sub1 = vn1->getDef();
        let sub1 = data.vbank().get(vn1).expect("RuleRangeMeld: stale vn1").get_def().expect(
            "RuleRangeMeld: written vn1 has no def",
        );
        // if (!sub1->isBoolOutput()) return 0;
        if !data.obank().get(sub1).expect("RuleRangeMeld: stale sub1").is_bool_output() {
            return 0;
        }
        // sub2 = vn2->getDef();
        let sub2 = data.vbank().get(vn2).expect("RuleRangeMeld: stale vn2").get_def().expect(
            "RuleRangeMeld: written vn2 has no def",
        );
        // if (!sub2->isBoolOutput()) return 0;
        if !data.obank().get(sub2).expect("RuleRangeMeld: stale sub2").is_bool_output() {
            return 0;
        }

        // CircleRange range1(true); ... A1 = range1.pullBack(sub1,...); ...
        //   range1.intersect/circleUnion(range2); translate2Op(...); ...
        // SEAM(rangeutil): `CircleRange::pullBack`/`circleUnion`/`translate2Op`/
        // `intersect` are rangeutil value-set operators still deferred (the
        // `CircleRange` in jumptable.rs is a stub), and the equality fixup uses
        // `functionalEquality` (expression seam).  The BOOL_AND/BOOL_OR
        // bool-output structural guards above are ported; the value-set body
        // defers here.  No change.
        0
    }
}

// =============================================================================
// Registration (W8 universalAction assembly reads this)
// =============================================================================

/// The [`RuleSpec`] rows for every rule in this batch, in **C++ definition
/// order** (`RuleEarlyRemoval` .. `RuleRangeMeld`).  The W8 universal-action
/// builder pushes these into the appropriate pools.  Each ctor builds the rule
/// with the `"analysis"` group — the group most of these expression rules belong
/// to in `coreaction.cc`'s `universalAction`; W8 supplies the real group when it
/// assembles the registration, exactly like the worked example in the action.rs
/// module docs (`RuleSpec { group, ctor }`).
pub fn specs() -> Vec<RuleSpec> {
    vec![
        RuleSpec { group: "analysis", ctor: || Box::new(RuleEarlyRemoval::new("analysis")) },
        RuleSpec { group: "analysis", ctor: || Box::new(RuleCollectTerms::new("analysis")) },
        RuleSpec { group: "analysis", ctor: || Box::new(RuleSelectCse::new("analysis")) },
        RuleSpec { group: "analysis", ctor: || Box::new(RulePiece2Zext::new("analysis")) },
        RuleSpec { group: "analysis", ctor: || Box::new(RulePiece2Sext::new("analysis")) },
        RuleSpec { group: "analysis", ctor: || Box::new(RuleBxor2NotEqual::new("analysis")) },
        RuleSpec { group: "analysis", ctor: || Box::new(RuleOrMask::new("analysis")) },
        RuleSpec { group: "analysis", ctor: || Box::new(RuleAndMask::new("analysis")) },
        RuleSpec { group: "analysis", ctor: || Box::new(RuleOrConsume::new("analysis")) },
        RuleSpec { group: "analysis", ctor: || Box::new(RuleOrCollapse::new("analysis")) },
        RuleSpec { group: "analysis", ctor: || Box::new(RuleAndOrLump::new("analysis")) },
        RuleSpec { group: "analysis", ctor: || Box::new(RuleNegateIdentity::new("analysis")) },
        RuleSpec { group: "analysis", ctor: || Box::new(RuleShiftBitops::new("analysis")) },
        RuleSpec { group: "analysis", ctor: || Box::new(RuleRightShiftAnd::new("analysis")) },
        RuleSpec { group: "analysis", ctor: || Box::new(RuleIntLessEqual::new("analysis")) },
        RuleSpec { group: "analysis", ctor: || Box::new(RuleEquality::new("analysis")) },
        RuleSpec { group: "analysis", ctor: || Box::new(RuleTermOrder::new("analysis")) },
        RuleSpec { group: "analysis", ctor: || Box::new(RulePullsubMulti::new("analysis")) },
        RuleSpec { group: "analysis", ctor: || Box::new(RulePullsubIndirect::new("analysis")) },
        RuleSpec { group: "analysis", ctor: || Box::new(RulePushMulti::new("analysis")) },
        RuleSpec { group: "analysis", ctor: || Box::new(RuleNotDistribute::new("analysis")) },
        RuleSpec { group: "analysis", ctor: || Box::new(RuleHighOrderAnd::new("analysis")) },
        RuleSpec { group: "analysis", ctor: || Box::new(RuleAndDistribute::new("analysis")) },
        RuleSpec { group: "analysis", ctor: || Box::new(RuleLessOne::new("analysis")) },
        RuleSpec { group: "analysis", ctor: || Box::new(RuleRangeMeld::new("analysis")) },
    ]
}

#[cfg(test)]
mod tests;
