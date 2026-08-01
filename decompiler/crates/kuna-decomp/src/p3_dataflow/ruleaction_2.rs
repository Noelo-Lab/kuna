//! Port of `decompiler/cpp/ruleaction.cc` lines 1444-2818 — the second batch of
//! the expression-simplification rule family (C++ `Rule` subclasses
//! `RuleFloatRange` .. `RuleBooleanUndistribute`, 20 classes).
//!
//! Each rule is a faithful transcription of the upstream `applyOp`/`getOpList`
//! body: identical guard order, slot indices, constant arithmetic, and op
//! creation/insertion sequence (the rule body IS the decompiler's output, so a
//! reordered statement changes datatest text — see the porting brief).
//!
//! # Cross-wave stubs used by this batch
//!
//! * **Opcode resolution** (`glb->inst[opc]`): the C++ `Funcdata::opSetOpcode`
//!   takes an [`OpCode`] and looks up the singleton `TypeOp` in the
//!   architecture's `inst` table; the Rust [`Funcdata::op_set_opcode`] takes the
//!   already-resolved [`TypeOp`].  [`op_typeop`] resolves it through the
//!   canonical `inst[]` table ([`crate::typeop::seam_type_op_for`]), which
//!   answers for every op-code.
//! * **`newUniqueOut`/`newVarnodeOut`**: the C++ helpers create a free varnode
//!   and make it the op's output via `opSetOutput`.  The Rust
//!   [`Funcdata::op_set_output`] is blocked on a `banks_mut()` split-borrow the
//!   W3 owner has not yet added.  For a *freshly created* output varnode (no
//!   readers yet) the `replace_reads` callback never fires, so [`new_unique_out`]
//!   / [`new_varnode_out`] perform the `set_def` + `set_output` directly with a
//!   no-op replace closure — correct precisely because the varnode is brand new.
//!   // STUB(W3): proper `op_set_output` once `banks_mut()` lands.
//! * **`Varnode::copySymbol` / `copySymbolIfValid`**: copies the data-type,
//!   symbol map-entry, and type/name lock flags.  These are W4/W6 symbol+type
//!   surfaces (not ported); the structural transform is unaffected, only the
//!   annotation of the new constant.  // STUB(W6) — recorded as a loss.
//! * **`RuleBooleanUndistribute`**: depends on `BooleanMatch::evaluate`
//!   (`expression.cc`, a different wave's module — currently a stub) and
//!   `Funcdata::opBoolNegate`.  The rule structure is transcribed in full;
//!   `op_bool_negate` is implemented locally (it only needs the op factories),
//!   but `is_match` routes the non-trivial comparison through the
//!   [`boolean_match`] stub, which handles only the `vn1 == vn2` trivial case and
//!   otherwise reports "uncorrelated".  // STUB(W5-expression) — recorded as a
//!   loss.

use kuna_base::address::{calc_mask, leastsigbit_set, signbit_negative, Address};
use kuna_base::types::{int4, uintb, Wrap};
use kuna_num::opcodes::OpCode;

use crate::action::{ActionGroupList, Rule, RuleSpec};
use crate::funcdata::Funcdata;
use crate::op::is_cse_match;
use crate::context::{OpId, TypeOp, VarnodeId};
use crate::varnode::DefOpInfo;

// =============================================================================
// W6 opcode resolution stub + W3 output-creation helpers
// =============================================================================

/// Build the [`TypeOp`] singleton for an [`OpCode`] this batch emits
/// (C++ `glb->inst[opc]`).
///
/// Resolves through the canonical [`crate::typeop::seam_type_op_for`] table (the
/// verbatim `typeop.cc` `opflags`/name for *every* op-code), so a rule that
/// retypes an op to an op-code this file never names still gets the properties
/// the C++ `inst[]` would install instead of killing the function.
fn op_typeop(opc: OpCode) -> TypeOp {
    crate::typeop::seam_type_op_for(opc)
}

/// `Funcdata::opSetOpcode(op, opc)` — resolve the [`OpCode`] to its [`TypeOp`]
/// and hand it to the bank.
fn op_set_opcode(data: &mut Funcdata, op: OpId, opc: OpCode) {
    data.op_set_opcode(op, op_typeop(opc));
}

/// `Funcdata::newUniqueOut(s, op)` — create a temporary-register output for the
/// (freshly created) op `op`.
///
/// The C++ is `Varnode *tmp = newUnique(s); opSetOutput(op, tmp); return tmp;`.
/// Because the new unique varnode has no readers, the `opSetOutput`
/// def-rewiring (`vbank.setDef` + `replace_reads`) never repoints anything, so
/// it is reproduced inline with a no-op replace closure.  // STUB(W3)
fn new_unique_out(data: &mut Funcdata, s: int4, op: OpId) -> VarnodeId {
    let tmp = data.new_unique(s, None);
    set_fresh_output(data, op, tmp);
    tmp
}

/// `Funcdata::newVarnodeOut(s, m, op)` — create a storage-addressed output for
/// the (freshly created) op `op`.  Like [`new_unique_out`] the output is brand
/// new, so the def-rewiring is a no-op.  // STUB(W3)
fn new_varnode_out(data: &mut Funcdata, s: int4, m: &Address, op: OpId) -> VarnodeId {
    let tmp = data.new_varnode(s, m, None);
    set_fresh_output(data, op, tmp);
    tmp
}

/// Promote a brand-new (reader-less) free varnode to be the output of `op`.
///
/// Equivalent to the def-rewiring half of `Funcdata::opSetOutput` for the case
/// where `vn` has no descendants (so the `replace_reads` callback is a no-op)
/// and `op` currently has no output.  // STUB(W3): subsumed by a proper
/// `op_set_output` once `Funcdata::banks_mut()` exists.
fn set_fresh_output(data: &mut Funcdata, op: OpId, vn: VarnodeId) {
    debug_assert!(
        data.vbank().get(vn).expect("set_fresh_output: stale vn").num_descend() == 0,
        "set_fresh_output called on a varnode with readers (would need replace_reads)"
    );
    let seq = data.obank().get(op).expect("set_fresh_output: stale op").get_seq_num().clone();
    let info = DefOpInfo { id: op, seqnum: seq };
    // No reader can exist on a fresh varnode, so replace is never invoked.
    let mut no_replace =
        |_b: &mut crate::varnode::VarnodeBank, _a: VarnodeId, _c: VarnodeId| Ok(());
    let vn = data
        .vbank_mut()
        .set_def(vn, info, &mut no_replace)
        .expect("set_fresh_output: set_def on fresh varnode");
    data.obank_mut().get_mut(op).expect("set_fresh_output: stale op").set_output(Some(vn));
}

/// `Funcdata::opBoolNegate(vn, op, insertafter)` (`funcdata_op.cc:560`).
///
/// Creates a `BOOL_NEGATE` of `vn` and inserts it before/after `op`, returning
/// the negated output.  Implemented locally because it only needs the op
/// factories (the wider `funcdata_op` port has not lifted it).  // STUB(W3)
fn op_bool_negate(data: &mut Funcdata, vn: VarnodeId, op: OpId, insertafter: bool) -> VarnodeId {
    let addr = data.obank().get(op).expect("opBoolNegate: stale op").get_addr().clone();
    let negateop = data.new_op(1, addr);
    op_set_opcode(data, negateop, OpCode::CPUI_BOOL_NEGATE);
    let resvn = new_unique_out(data, 1, negateop);
    data.op_set_input(negateop, vn, 0).expect("opBoolNegate: set input");
    if insertafter {
        data.op_insert_after(negateop, op);
    } else {
        data.op_insert_before(negateop, op);
    }
    resvn
}

// -----------------------------------------------------------------------------
// Small read-only accessor shims (keep the rule bodies readable; each is a thin
// wrapper over the bank `get` + `PcodeOp`/`Varnode` accessor — the C++
// `op->...` / `vn->...` member calls).
// -----------------------------------------------------------------------------

#[inline]
fn op_code(data: &Funcdata, op: OpId) -> OpCode {
    data.obank().get(op).expect("stale op").code()
}
#[inline]
fn op_in(data: &Funcdata, op: OpId, slot: int4) -> VarnodeId {
    data.obank().get(op).expect("stale op").get_in(slot).expect("null input (C++ UB)")
}
#[inline]
fn op_out(data: &Funcdata, op: OpId) -> VarnodeId {
    data.obank().get(op).expect("stale op").get_out().expect("null output (C++ UB)")
}
#[inline]
fn op_num_input(data: &Funcdata, op: OpId) -> int4 {
    data.obank().get(op).expect("stale op").num_input()
}
#[inline]
fn op_addr(data: &Funcdata, op: OpId) -> Address {
    data.obank().get(op).expect("stale op").get_addr().clone()
}
#[inline]
fn vn_written(data: &Funcdata, vn: VarnodeId) -> bool {
    data.vbank().get(vn).expect("stale vn").is_written()
}
#[inline]
fn vn_constant(data: &Funcdata, vn: VarnodeId) -> bool {
    data.vbank().get(vn).expect("stale vn").is_constant()
}
#[inline]
fn vn_free(data: &Funcdata, vn: VarnodeId) -> bool {
    data.vbank().get(vn).expect("stale vn").is_free()
}
#[inline]
fn vn_heritage_known(data: &Funcdata, vn: VarnodeId) -> bool {
    data.vbank().get(vn).expect("stale vn").is_heritage_known()
}
#[inline]
fn vn_def(data: &Funcdata, vn: VarnodeId) -> Option<OpId> {
    data.vbank().get(vn).expect("stale vn").get_def()
}
#[inline]
fn vn_size(data: &Funcdata, vn: VarnodeId) -> int4 {
    data.vbank().get(vn).expect("stale vn").get_size()
}
#[inline]
fn vn_offset(data: &Funcdata, vn: VarnodeId) -> uintb {
    data.vbank().get(vn).expect("stale vn").get_offset()
}
#[inline]
fn vn_nzmask(data: &Funcdata, vn: VarnodeId) -> uintb {
    data.vbank().get(vn).expect("stale vn").get_nz_mask()
}
#[inline]
fn vn_addr(data: &Funcdata, vn: VarnodeId) -> Address {
    data.vbank().get(vn).expect("stale vn").get_addr().clone()
}

/// `*a == *b` for Varnodes (C++ `Varnode::operator==`, `varnode.cc:575`):
/// matching storage location, size, and (if both written) defining-op SeqNum.
fn varnodes_equiv(data: &Funcdata, a: VarnodeId, b: VarnodeId) -> bool {
    if a == b {
        return true;
    }
    let va = data.vbank().get(a).expect("stale vn");
    let vb = data.vbank().get(b).expect("stale vn");
    if va.get_addr() != vb.get_addr() {
        return false;
    }
    if va.get_size() != vb.get_size() {
        return false;
    }
    // f1/f2 = flags & (input|written); they must agree.
    let f1 = (va.is_input(), va.is_written());
    let f2 = (vb.is_input(), vb.is_written());
    if f1 != f2 {
        return false;
    }
    // If both written, the defining ops must share a SeqNum.
    if va.is_written() {
        let da = va.get_def().expect("written varnode has no def (C++ UB)");
        let db = vb.get_def().expect("written varnode has no def (C++ UB)");
        let sa = data.obank().get(da).expect("stale def op").get_seq_num().clone();
        let sb = data.obank().get(db).expect("stale def op").get_seq_num().clone();
        if sa != sb {
            return false;
        }
    }
    true
}

// =============================================================================
// RuleFloatRange
// =============================================================================

/// `(V f< W)||(V f== W)   =>   V f<= W` (and similar variants).  C++
/// `RuleFloatRange` (`ruleaction.cc:1440`).
pub struct RuleFloatRange;

impl Rule for RuleFloatRange {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_BOOL_OR, OpCode::CPUI_BOOL_AND]
    }
    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains("analysis") && !grouplist.contains("floatrange") {
            return None;
        }
        Some(Box::new(RuleFloatRange))
    }
    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        let vn1 = op_in(data, op, 0);
        if !vn_written(data, vn1) {
            return 0;
        }
        let vn2 = op_in(data, op, 1);
        if !vn_written(data, vn2) {
            return 0;
        }
        let mut cmp1 = vn_def(data, vn1).expect("cmp1 def");
        let mut cmp2 = vn_def(data, vn2).expect("cmp2 def");
        let mut opccmp1 = op_code(data, cmp1);
        // Set cmp1 to LESS or LESSEQUAL operator, cmp2 is the "other" operator
        if (opccmp1 != OpCode::CPUI_FLOAT_LESS) && (opccmp1 != OpCode::CPUI_FLOAT_LESSEQUAL) {
            cmp1 = cmp2;
            cmp2 = vn_def(data, vn1).expect("cmp2 def");
            opccmp1 = op_code(data, cmp1);
        }
        let mut resultopc = OpCode::CPUI_COPY;
        if opccmp1 == OpCode::CPUI_FLOAT_LESS {
            if (op_code(data, cmp2) == OpCode::CPUI_FLOAT_EQUAL)
                && (op_code(data, op) == OpCode::CPUI_BOOL_OR)
            {
                resultopc = OpCode::CPUI_FLOAT_LESSEQUAL;
            }
        } else if opccmp1 == OpCode::CPUI_FLOAT_LESSEQUAL
            && (op_code(data, cmp2) == OpCode::CPUI_FLOAT_NOTEQUAL)
            && (op_code(data, op) == OpCode::CPUI_BOOL_AND)
        {
            resultopc = OpCode::CPUI_FLOAT_LESS;
        }

        if resultopc == OpCode::CPUI_COPY {
            return 0;
        }

        // Make sure both operators are comparing the same things
        let mut slot1: int4 = 0;
        let mut nvn1 = op_in(data, cmp1, slot1); // Set nvn1 to a non-constant off of cmp1
        if vn_constant(data, nvn1) {
            slot1 = 1;
            nvn1 = op_in(data, cmp1, slot1);
            if vn_constant(data, nvn1) {
                return 0;
            }
        }
        if vn_free(data, nvn1) {
            return 0;
        }
        let cvn1 = op_in(data, cmp1, 1 - slot1); // Set cvn1 to the "other" slot off of cmp1
        let slot2: int4;
        if nvn1 != op_in(data, cmp2, 0) {
            slot2 = 1;
            if nvn1 != op_in(data, cmp2, 1) {
                return 0;
            }
        } else {
            slot2 = 0;
        }
        let matchvn = op_in(data, cmp2, 1 - slot2);
        // The two trailing branches share a body (`return 0`) but test distinct
        // conditions (`cvn1 != matchvn` vs `cvn1->isFree()`); kept separate to
        // mirror the C++ `else if` chain verbatim.
        #[allow(clippy::if_same_then_else)]
        if vn_constant(data, cvn1) {
            if !vn_constant(data, matchvn) {
                return 0;
            }
            if vn_offset(data, matchvn) != vn_offset(data, cvn1) {
                return 0;
            }
        } else if cvn1 != matchvn {
            return 0;
        } else if vn_free(data, cvn1) {
            return 0;
        }

        // Collapse the 2 comparisons into 1 comparison
        op_set_opcode(data, op, resultopc);
        data.op_set_input(op, nvn1, slot1).expect("set input");
        if vn_constant(data, cvn1) {
            let c = data.new_constant(vn_size(data, cvn1), vn_offset(data, cvn1));
            data.op_set_input(op, c, 1 - slot1).expect("set input");
        } else {
            data.op_set_input(op, cvn1, 1 - slot1).expect("set input");
        }
        1
    }
}

// =============================================================================
// RuleAndCommute
// =============================================================================

/// `(V << W) & d  =>  (V & (W >> c)) << c`.  C++ `RuleAndCommute`
/// (`ruleaction.cc:1521`).
pub struct RuleAndCommute;

impl Rule for RuleAndCommute {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_INT_AND]
    }
    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains("analysis") && !grouplist.contains("andcommute") {
            return None;
        }
        Some(Box::new(RuleAndCommute))
    }
    // The `sa`/`opc` initializers are the C++ "Unnecessary initialization"
    // placeholders; each is overwritten before any read inside the loop.
    #[allow(unused_assignments)]
    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // Unnecessary C++ initializations are reflected as Option/placeholders.
        let mut orvn: Option<VarnodeId> = None;
        let mut othervn: Option<VarnodeId> = None;
        let mut savn: Option<VarnodeId> = None;
        let mut opc = OpCode::CPUI_INT_OR; // Unnecessary initialization
        let mut sa: int4 = 0;

        let size = vn_size(data, op_out(data, op));
        if size as usize > std::mem::size_of::<uintb>() {
            return 0; // FIXME: uintb should be arbitrary precision
        }
        let fullmask = calc_mask(size);
        let mut i: int4 = 0;
        while i < 2 {
            let shiftvn = op_in(data, op, i);
            let shiftop = match vn_def(data, shiftvn) {
                Some(d) => d,
                None => {
                    i += 1;
                    continue;
                }
            };
            opc = op_code(data, shiftop);
            if (opc != OpCode::CPUI_INT_LEFT) && (opc != OpCode::CPUI_INT_RIGHT) {
                i += 1;
                continue;
            }
            let savn_l = op_in(data, shiftop, 1);
            savn = Some(savn_l);
            if !vn_constant(data, savn_l) {
                i += 1;
                continue;
            }
            sa = vn_offset(data, savn_l) as int4;

            let othervn_l = op_in(data, op, 1 - i);
            othervn = Some(othervn_l);
            if !vn_heritage_known(data, othervn_l) {
                i += 1;
                continue;
            }
            let mut othermask = vn_nzmask(data, othervn_l);
            // Check if AND is only zeroing bits which are already zeroed by the
            // shift, in which case andmask takes care of it
            // `sa` is the raw constant shift amount and can be >=64; the C++
            // mask arithmetic relies on x86 shift-count masking, so use the
            // wrapping shifts (ADR-0003) — identical to bare `>>`/`<<` for sa<64.
            if opc == OpCode::CPUI_INT_RIGHT {
                if fullmask.wshr(sa as u32) == othermask {
                    i += 1;
                    continue;
                }
                othermask = othermask.wshl(sa as u32); // Calc mask as it will be after commute
            } else {
                // NOTE: `(fullmask<<sa)&&fullmask` is the verbatim upstream
                // expression (logical-and of two non-zero words, almost always
                // true); preserved exactly so behavior matches.
                let lhs = if (fullmask.wshl(sa as u32) != 0) && (fullmask != 0) {
                    1u64
                } else {
                    0u64
                };
                if lhs == othermask {
                    i += 1;
                    continue;
                }
                othermask = othermask.wshr(sa as u32); // Calc mask as it will be after commute
            }
            if othermask == 0 {
                i += 1;
                continue; // Handled by andmask
            }
            if othermask == fullmask {
                i += 1;
                continue;
            }

            let orvn_l = op_in(data, shiftop, 0);
            orvn = Some(orvn_l);
            if (opc == OpCode::CPUI_INT_LEFT) && vn_constant(data, othervn_l) {
                // (v & #c) << #sa is preferred to (v << #sa) & #(c << sa)
                if data.lone_descend(shiftvn) == Some(op) {
                    break; // If there is no other use of shift, always do the commute
                }
            }

            if !vn_written(data, orvn_l) {
                i += 1;
                continue;
            }
            let orop = vn_def(data, orvn_l).expect("orop def");

            if op_code(data, orop) == OpCode::CPUI_INT_OR {
                let ormask1 = vn_nzmask(data, op_in(data, orop, 0));
                if (ormask1 & othermask) == 0 {
                    break;
                }
                let ormask2 = vn_nzmask(data, op_in(data, orop, 1));
                if (ormask2 & othermask) == 0 {
                    break;
                }
                if vn_constant(data, othervn_l) {
                    if (ormask1 & othermask) == ormask1 {
                        break;
                    }
                    if (ormask2 & othermask) == ormask2 {
                        break;
                    }
                }
            } else if op_code(data, orop) == OpCode::CPUI_PIECE {
                let ormask1 = vn_nzmask(data, op_in(data, orop, 1)); // Low part of piece
                if (ormask1 & othermask) == 0 {
                    break;
                }
                let mut ormask2 = vn_nzmask(data, op_in(data, orop, 0)); // High part
                // shift count = lowpart size*8, up to 64; x86-masked in C++ (ADR-0003).
                ormask2 = ormask2.wshl((vn_size(data, op_in(data, orop, 1)) * 8) as u32);
                if (ormask2 & othermask) == 0 {
                    break;
                }
            } else {
                i += 1;
                continue;
            }
            // The loop only reaches here via `break` in the C++ (matched above).
            // Falling through means none of the break conditions fired in this
            // iteration's tail; advance.
            i += 1;
        }
        if i == 2 {
            return 0;
        }
        let orvn = orvn.expect("orvn set");
        let othervn = othervn.expect("othervn set");
        let savn = savn.expect("savn set");
        // Do the commute
        let addr = op_addr(data, op);
        let newop1 = data.new_op(2, addr.clone());
        let newvn1 = new_unique_out(data, size, newop1);
        op_set_opcode(
            data,
            newop1,
            if opc == OpCode::CPUI_INT_LEFT { OpCode::CPUI_INT_RIGHT } else { OpCode::CPUI_INT_LEFT },
        );
        data.op_set_input(newop1, othervn, 0).expect("set input");
        data.op_set_input(newop1, savn, 1).expect("set input");
        data.op_insert_before(newop1, op);

        let newop2 = data.new_op(2, addr);
        let newvn2 = new_unique_out(data, size, newop2);
        op_set_opcode(data, newop2, OpCode::CPUI_INT_AND);
        data.op_set_input(newop2, orvn, 0).expect("set input");
        data.op_set_input(newop2, newvn1, 1).expect("set input");
        data.op_insert_before(newop2, op);

        data.op_set_input(op, newvn2, 0).expect("set input");
        data.op_set_input(op, savn, 1).expect("set input");
        op_set_opcode(data, op, opc);

        1
    }
}

// =============================================================================
// RuleAndPiece
// =============================================================================

/// `V & concat(W,X)  =>  zext(X)` (when the upper part is known zero), or
/// `=> V & concat(#0,X)` (when the lower part is zero).  C++ `RuleAndPiece`
/// (`ruleaction.cc:1629`).
pub struct RuleAndPiece;

impl Rule for RuleAndPiece {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_INT_AND]
    }
    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains("analysis") && !grouplist.contains("andpiece") {
            return None;
        }
        Some(Box::new(RuleAndPiece))
    }
    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        let mut highvn: Option<VarnodeId> = None;
        let mut lowvn: Option<VarnodeId> = None;
        let mut opc = OpCode::CPUI_PIECE; // Unnecessary initialization
        let size = vn_size(data, op_out(data, op));
        let mut i: int4 = 0;
        while i < 2 {
            let piecevn = op_in(data, op, i);
            if !vn_written(data, piecevn) {
                i += 1;
                continue;
            }
            let pieceop = vn_def(data, piecevn).expect("pieceop def");
            if op_code(data, pieceop) != OpCode::CPUI_PIECE {
                i += 1;
                continue;
            }
            let othervn = op_in(data, op, 1 - i);
            let othermask = vn_nzmask(data, othervn);
            if othermask == calc_mask(size) {
                i += 1;
                continue;
            }
            if othermask == 0 {
                i += 1;
                continue; // Handled by andmask
            }
            let highvn_l = op_in(data, pieceop, 0);
            highvn = Some(highvn_l);
            if !vn_heritage_known(data, highvn_l) {
                i += 1;
                continue;
            }
            let lowvn_l = op_in(data, pieceop, 1);
            lowvn = Some(lowvn_l);
            if !vn_heritage_known(data, lowvn_l) {
                i += 1;
                continue;
            }
            let maskhigh = vn_nzmask(data, highvn_l);
            let masklow = vn_nzmask(data, lowvn_l);
            // shift count = lowpart size*8, up to 64; x86-masked in C++ (ADR-0003).
            if (maskhigh & othermask.wshr((vn_size(data, lowvn_l) * 8) as u32)) == 0 {
                if (maskhigh == 0) && vn_constant(data, highvn_l) {
                    i += 1;
                    continue; // Handled by piece2zext
                }
                opc = OpCode::CPUI_INT_ZEXT;
                break;
            } else if (masklow & othermask) == 0 {
                if vn_constant(data, lowvn_l) {
                    i += 1;
                    continue; // Nothing to do
                }
                opc = OpCode::CPUI_PIECE;
                break;
            }
            i += 1;
        }
        if i == 2 {
            return 0;
        }
        let highvn = highvn.expect("highvn set");
        let lowvn = lowvn.expect("lowvn set");
        let addr = op_addr(data, op);
        let newop;
        if opc == OpCode::CPUI_INT_ZEXT {
            // Change PIECE(a,b) to ZEXT(b)
            newop = data.new_op(1, addr);
            op_set_opcode(data, newop, opc);
            data.op_set_input(newop, lowvn, 0).expect("set input");
        } else {
            // Change PIECE(a,b) to PIECE(a,#0)
            let newvn2 = data.new_constant(vn_size(data, lowvn), 0);
            newop = data.new_op(2, addr);
            op_set_opcode(data, newop, opc);
            data.op_set_input(newop, highvn, 0).expect("set input");
            data.op_set_input(newop, newvn2, 1).expect("set input");
        }
        let newvn = new_unique_out(data, size, newop);
        data.op_insert_before(newop, op);
        data.op_set_input(op, newvn, i).expect("set input");
        1
    }
}

// =============================================================================
// RuleAndZext
// =============================================================================

/// `sext(X) & 0xffff  =>  zext(X)` (and the `concat(Y,X)` variant).  C++
/// `RuleAndZext` (`ruleaction.cc:1697`).
pub struct RuleAndZext;

impl Rule for RuleAndZext {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_INT_AND]
    }
    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains("analysis") && !grouplist.contains("andzext") {
            return None;
        }
        Some(Box::new(RuleAndZext))
    }
    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        let cvn1 = op_in(data, op, 1);
        if !vn_constant(data, cvn1) {
            return 0;
        }
        let in0 = op_in(data, op, 0);
        if !vn_written(data, in0) {
            return 0;
        }
        let otherop = vn_def(data, in0).expect("otherop def");
        let opc = op_code(data, otherop);
        let rootvn = if opc == OpCode::CPUI_INT_SEXT {
            op_in(data, otherop, 0)
        } else if opc == OpCode::CPUI_PIECE {
            op_in(data, otherop, 1)
        } else {
            return 0;
        };
        let mask = calc_mask(vn_size(data, rootvn));
        if mask != vn_offset(data, cvn1) {
            return 0;
        }
        if vn_free(data, rootvn) {
            return 0;
        }
        if vn_size(data, rootvn) as usize > std::mem::size_of::<uintb>() {
            // FIXME: Should be arbitrary precision
            return 0;
        }
        op_set_opcode(data, op, OpCode::CPUI_INT_ZEXT);
        data.op_remove_input(op, 1);
        data.op_set_input(op, rootvn, 0).expect("set input");
        1
    }
}

// =============================================================================
// RuleAndCompare
// =============================================================================

/// `zext(V) & c == 0  =>  V & (c & mask) == 0` (and the `sub(V,c)` variant).
/// C++ `RuleAndCompare` (`ruleaction.cc:1735`).
pub struct RuleAndCompare;

impl Rule for RuleAndCompare {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_INT_EQUAL, OpCode::CPUI_INT_NOTEQUAL]
    }
    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains("analysis") && !grouplist.contains("andcompare") {
            return None;
        }
        Some(Box::new(RuleAndCompare))
    }
    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        if !vn_constant(data, op_in(data, op, 1)) {
            return 0;
        }
        if vn_offset(data, op_in(data, op, 1)) != 0 {
            return 0;
        }

        let andvn = op_in(data, op, 0);
        if !vn_written(data, andvn) {
            return 0;
        }
        let andop = vn_def(data, andvn).expect("andop def");
        if op_code(data, andop) != OpCode::CPUI_INT_AND {
            return 0;
        }
        if !vn_constant(data, op_in(data, andop, 1)) {
            return 0;
        }
        let subvn = op_in(data, andop, 0);
        if !vn_written(data, subvn) {
            return 0;
        }
        let subop = vn_def(data, subvn).expect("subop def");
        let basevn;
        let baseconst;
        let andconst;
        match op_code(data, subop) {
            OpCode::CPUI_SUBPIECE => {
                basevn = op_in(data, subop, 0);
                if vn_size(data, basevn) as usize > std::mem::size_of::<uintb>() {
                    return 0;
                }
                baseconst = vn_offset(data, op_in(data, andop, 1));
                // shift count = SUBPIECE offset*8, can reach 64; x86-masked in C++ (ADR-0003).
                andconst = baseconst.wshl((vn_offset(data, op_in(data, subop, 1)) * 8) as u32);
            }
            OpCode::CPUI_INT_ZEXT => {
                basevn = op_in(data, subop, 0);
                baseconst = vn_offset(data, op_in(data, andop, 1));
                andconst = baseconst & calc_mask(vn_size(data, basevn));
            }
            _ => return 0,
        }

        if baseconst == calc_mask(vn_size(data, andvn)) {
            return 0; // Degenerate AND
        }
        if vn_free(data, basevn) {
            return 0;
        }

        let constvn = data.new_constant(vn_size(data, basevn), andconst);
        if baseconst == andconst {
            // If no effective change in constant (except varnode size)
            // constvn->copySymbol(andop->getIn(1)): copy the data-type/symbol.
            // STUB(W6): copySymbol (data-type + symbol map-entry) — deferred.
        }
        // New version of and with bigger inputs
        let and_addr = op_addr(data, andop);
        let newop = data.new_op(2, and_addr);
        op_set_opcode(data, newop, OpCode::CPUI_INT_AND);
        let newout = new_unique_out(data, vn_size(data, basevn), newop);
        data.op_set_input(newop, basevn, 0).expect("set input");
        data.op_set_input(newop, constvn, 1).expect("set input");
        data.op_insert_before(newop, andop);

        data.op_set_input(op, newout, 0).expect("set input");
        let zero = data.new_constant(vn_size(data, basevn), 0);
        data.op_set_input(op, zero, 1).expect("set input");
        1
    }
}

// =============================================================================
// RuleDoubleSub
// =============================================================================

/// `sub( sub(V,c), d)  =>  sub(V, c+d)`.  C++ `RuleDoubleSub`
/// (`ruleaction.cc:1799`).
pub struct RuleDoubleSub;

impl Rule for RuleDoubleSub {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_SUBPIECE]
    }
    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains("analysis") && !grouplist.contains("doublesub") {
            return None;
        }
        Some(Box::new(RuleDoubleSub))
    }
    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        let vn = op_in(data, op, 0);
        if !vn_written(data, vn) {
            return 0;
        }
        let op2 = vn_def(data, vn).expect("op2 def");
        if op_code(data, op2) != OpCode::CPUI_SUBPIECE {
            return 0;
        }
        let in_vn = op_in(data, op2, 0);
        if vn_free(data, in_vn) {
            return 0;
        }
        let offset1 = vn_offset(data, op_in(data, op, 1)) as int4;
        let offset2 = vn_offset(data, op_in(data, op2, 1)) as int4;

        data.op_set_input(op, in_vn, 0).expect("set input"); // Skip middleman
        let c = data.new_constant(4, (offset1 + offset2) as uintb);
        data.op_set_input(op, c, 1).expect("set input");
        1
    }
}

// =============================================================================
// RuleDoubleShift
// =============================================================================

/// Combine/cancel chained INT_LEFT/INT_RIGHT shifts (INT_MULT by a power of 2
/// counts as a left shift).  C++ `RuleDoubleShift` (`ruleaction.cc:1824`).
pub struct RuleDoubleShift;

impl Rule for RuleDoubleShift {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_INT_LEFT, OpCode::CPUI_INT_RIGHT, OpCode::CPUI_INT_MULT]
    }
    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        // (kuna) C++ `new RuleDoubleShift("analysis")` — the rule's runtime group
        // is "analysis" (coreaction.cc:5807), so the live `universalAction` pool
        // clone must test "analysis"; checking only the per-op placeholder name
        // ("doubleshift") dropped this rule from every cloned pool, so `(x>>a)>>b`
        // collapses never ran — the div/mod reconstruction depends on them.  Both
        // are accepted so the placeholder-`specs()` contract still holds.
        if !grouplist.contains("analysis") && !grouplist.contains("doubleshift") {
            return None;
        }
        Some(Box::new(RuleDoubleShift))
    }
    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        let sa1: int4;
        let sa2: int4;
        let mut mask: uintb;

        if !vn_constant(data, op_in(data, op, 1)) {
            return 0;
        }
        let secvn = op_in(data, op, 0);
        if !vn_written(data, secvn) {
            return 0;
        }
        let secop = vn_def(data, secvn).expect("secop def");
        let mut opc2 = op_code(data, secop);
        if (opc2 != OpCode::CPUI_INT_LEFT)
            && (opc2 != OpCode::CPUI_INT_RIGHT)
            && (opc2 != OpCode::CPUI_INT_MULT)
        {
            return 0;
        }
        if !vn_constant(data, op_in(data, secop, 1)) {
            return 0;
        }
        let mut opc1 = op_code(data, op);
        let size = vn_size(data, secvn);
        if !vn_heritage_known(data, op_in(data, secop, 0)) {
            return 0;
        }

        if opc1 == OpCode::CPUI_INT_MULT {
            let val = vn_offset(data, op_in(data, op, 1));
            sa1 = leastsigbit_set(val);
            // sa1 == -1 when val == 0; C++ shifts by (-1 & 0x3f) == 63 via x86
            // masking — `wshr((sa1 as u32))` reproduces it (ADR-0003).
            if val.wshr(sa1 as u32) != 1u64 {
                return 0; // Not multiplying by a power of 2
            }
            opc1 = OpCode::CPUI_INT_LEFT;
        } else {
            sa1 = vn_offset(data, op_in(data, op, 1)) as int4;
        }
        if opc2 == OpCode::CPUI_INT_MULT {
            let val = vn_offset(data, op_in(data, secop, 1));
            sa2 = leastsigbit_set(val);
            // sa2 == -1 when val == 0; see sa1 note (x86-masked, ADR-0003).
            if val.wshr(sa2 as u32) != 1u64 {
                return 0; // Not multiplying by a power of 2
            }
            opc2 = OpCode::CPUI_INT_LEFT;
        } else {
            sa2 = vn_offset(data, op_in(data, secop, 1)) as int4;
        }
        let secop_in0 = op_in(data, secop, 0);
        if opc1 == opc2 {
            // Shifts in the same direction
            if sa1 + sa2 < 8 * size {
                let newvn = data.new_constant(4, (sa1 + sa2) as uintb);
                op_set_opcode(data, op, opc1);
                data.op_set_input(op, secop_in0, 0).expect("set input");
                data.op_set_input(op, newvn, 1).expect("set input");
            } else {
                let newvn = data.new_constant(size, 0);
                op_set_opcode(data, op, OpCode::CPUI_COPY);
                data.op_set_input(op, newvn, 0).expect("set input");
                data.op_remove_input(op, 1);
            }
        } else {
            // Shifts in opposite directions
            if size as usize > std::mem::size_of::<uintb>() {
                return 0; // FIXME: precision
            }
            mask = calc_mask(size);
            let mut diffsa: int4; // Bits (to the left) after cancellation
            if opc1 == OpCode::CPUI_INT_LEFT {
                // The INT_LEFT is highly likely to be a multiply
                if data.lone_descend(secvn).is_none() {
                    return 0;
                }
                // sa2 is a raw constant shift amount, can be >=64; x86-masked in C++ (ADR-0003).
                mask = mask.wshl(sa2 as u32) & mask; // Most significant bits remain after initial INT_RIGHT
                diffsa = sa1 - sa2;
                if diffsa != 0 {
                    // Don't collapse unless shift amounts are identical
                    return 0;
                }
            } else {
                // sa2 is a raw constant shift amount, can be >=64; x86-masked in C++ (ADR-0003).
                mask = mask.wshr(sa2 as u32) & mask; // Least significant bits remain after initial INT_LEFT
                diffsa = sa2 - sa1;
            }
            if diffsa == 0 {
                // Opposite shifts exactly cancel
                let newvn = data.new_constant(size, mask);
                op_set_opcode(data, op, OpCode::CPUI_INT_AND);
                data.op_set_input(op, secop_in0, 0).expect("set input");
                data.op_set_input(op, newvn, 1).expect("set input");
            } else {
                // Shifts only partly cancel
                let addr = op_addr(data, op);
                let new_and = data.new_op(2, addr);
                op_set_opcode(data, new_and, OpCode::CPUI_INT_AND);
                data.op_set_input(new_and, secop_in0, 0).expect("set input");
                let mc = data.new_constant(size, mask);
                data.op_set_input(new_and, mc, 1).expect("set input");
                let new_out = new_unique_out(data, size, new_and);
                data.op_insert_before(new_and, op);
                let mut finalopc = OpCode::CPUI_INT_LEFT;
                if diffsa < 0 {
                    finalopc = OpCode::CPUI_INT_RIGHT;
                    diffsa = -diffsa;
                }
                op_set_opcode(data, op, finalopc);
                data.op_set_input(op, new_out, 0).expect("set input");
                let dc = data.new_constant(4, diffsa as uintb);
                data.op_set_input(op, dc, 1).expect("set input");
            }
        }
        1
    }
}

// =============================================================================
// RuleDoubleArithShift
// =============================================================================

/// `(x s>> c) s>> d   =>  x s>> saturate(c + d)`.  C++ `RuleDoubleArithShift`
/// (`ruleaction.cc:1931`).
pub struct RuleDoubleArithShift;

impl Rule for RuleDoubleArithShift {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_INT_SRIGHT]
    }
    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        // (kuna) C++ `new RuleDoubleArithShift("analysis")` — the rule's runtime
        // group is "analysis" (coreaction.cc:5808), so the live `universalAction`
        // pool clone must test "analysis"; checking only the per-op placeholder
        // name ("doublearithshift") dropped this rule from every cloned pool, so
        // `(x s>> a) s>> b` collapses never ran — the signed div/mod
        // reconstruction (RuleModOpt on `x % C`) depends on them.  Both are
        // accepted so the placeholder-`specs()` contract still holds.
        if !grouplist.contains("analysis") && !grouplist.contains("doublearithshift") {
            return None;
        }
        Some(Box::new(RuleDoubleArithShift))
    }
    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        let const_d = op_in(data, op, 1);
        if !vn_constant(data, const_d) {
            return 0;
        }
        let shiftin = op_in(data, op, 0);
        if !vn_written(data, shiftin) {
            return 0;
        }
        let shift2op = vn_def(data, shiftin).expect("shift2op def");
        if op_code(data, shift2op) != OpCode::CPUI_INT_SRIGHT {
            return 0;
        }
        let const_c = op_in(data, shift2op, 1);
        if !vn_constant(data, const_c) {
            return 0;
        }
        let in_vn = op_in(data, shift2op, 0);
        if vn_free(data, in_vn) {
            return 0;
        }
        let max = vn_size(data, op_out(data, op)) * 8 - 1; // This is maximum possible shift.
        let mut sa = vn_offset(data, const_c) as int4 + vn_offset(data, const_d) as int4;
        if sa <= 0 {
            return 0; // Something is wrong
        }
        if sa > max {
            sa = max; // Shift amount has saturated
        }
        data.op_set_input(op, in_vn, 0).expect("set input");
        let c = data.new_constant(4, sa as uintb);
        data.op_set_input(op, c, 1).expect("set input");
        1
    }
}

// =============================================================================
// RuleConcatShift
// =============================================================================

/// `concat(V,W) >> c  =>  zext(V)` (signed shift gives sext).  C++
/// `RuleConcatShift` (`ruleaction.cc:1965`).
pub struct RuleConcatShift;

impl Rule for RuleConcatShift {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_INT_RIGHT, OpCode::CPUI_INT_SRIGHT]
    }
    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains("analysis") && !grouplist.contains("concatshift") {
            return None;
        }
        Some(Box::new(RuleConcatShift))
    }
    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        if !vn_constant(data, op_in(data, op, 1)) {
            return 0;
        }

        let shiftin = op_in(data, op, 0);
        if !vn_written(data, shiftin) {
            return 0;
        }
        let concat = vn_def(data, shiftin).expect("concat def");
        if op_code(data, concat) != OpCode::CPUI_PIECE {
            return 0;
        }

        let mut sa = vn_offset(data, op_in(data, op, 1)) as int4;
        let leastsize = vn_size(data, op_in(data, concat, 1)) * 8;
        if sa < leastsize {
            return 0; // Does shift throw away least sig part
        }
        let mainin = op_in(data, concat, 0);
        if vn_free(data, mainin) {
            return 0;
        }
        sa -= leastsize;
        let extcode = if op_code(data, op) == OpCode::CPUI_INT_RIGHT {
            OpCode::CPUI_INT_ZEXT
        } else {
            OpCode::CPUI_INT_SEXT
        };
        if sa == 0 {
            // Exact cancelation
            data.op_remove_input(op, 1); // Remove thrown away least
            op_set_opcode(data, op, extcode); // Change to extension
            data.op_set_input(op, mainin, 0).expect("set input");
        } else {
            // Create a new extension op
            let addr = op_addr(data, op);
            let extop = data.new_op(1, addr);
            op_set_opcode(data, extop, extcode);
            let newvn = new_unique_out(data, vn_size(data, shiftin), extop);
            data.op_set_input(extop, mainin, 0).expect("set input");

            // Adjust the shift amount
            data.op_set_input(op, newvn, 0).expect("set input");
            let c = data.new_constant(vn_size(data, op_in(data, op, 1)), sa as uintb);
            data.op_set_input(op, c, 1).expect("set input");
            data.op_insert_before(extop, op);
        }
        1
    }
}

// =============================================================================
// RuleLeftRight
// =============================================================================

/// `(V << c) s>> c  =>  sext( sub(V, #0) )` (and the unsigned variant).  C++
/// `RuleLeftRight` (`ruleaction.cc:2015`).
pub struct RuleLeftRight;

impl Rule for RuleLeftRight {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_INT_RIGHT, OpCode::CPUI_INT_SRIGHT]
    }
    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains("analysis") && !grouplist.contains("leftright") {
            return None;
        }
        Some(Box::new(RuleLeftRight))
    }
    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        if !vn_constant(data, op_in(data, op, 1)) {
            return 0;
        }

        let shiftin = op_in(data, op, 0);
        if !vn_written(data, shiftin) {
            return 0;
        }
        let leftshift = vn_def(data, shiftin).expect("leftshift def");
        if op_code(data, leftshift) != OpCode::CPUI_INT_LEFT {
            return 0;
        }
        if !vn_constant(data, op_in(data, leftshift, 1)) {
            return 0;
        }
        let sa = vn_offset(data, op_in(data, op, 1));
        if vn_offset(data, op_in(data, leftshift, 1)) != sa {
            return 0; // Left shift must be by same amount
        }

        if (sa & 7) != 0 {
            return 0; // Must be multiple of 8
        }
        let isa = (sa >> 3) as int4;
        let tsz = vn_size(data, shiftin) - isa;
        if (tsz != 1) && (tsz != 2) && (tsz != 4) && (tsz != 8) {
            return 0;
        }

        if data.lone_descend(shiftin) != Some(op) {
            return 0;
        }
        let mut addr = vn_addr(data, shiftin);
        if addr.is_big_endian() {
            addr = &addr + isa as i64;
        }
        data.op_unset_input(op, 0);
        data.op_unset_output(leftshift);
        // addr.renormalize(tsz): only acts on join addresses; the shift source
        // is a register/ram varnode, so this is a no-op here, but is invoked
        // faithfully.  The manager is read through a cloned `ArchHandle` (an Rc)
        // to avoid holding `&data` across the later `&mut data` calls.
        let arch = std::rc::Rc::clone(data.get_arch());
        addr.renormalize(tsz, arch.manage()).expect("renormalize");
        let newvn = new_varnode_out(data, tsz, &addr, leftshift);
        op_set_opcode(data, leftshift, OpCode::CPUI_SUBPIECE);
        let c = data.new_constant(vn_size(data, op_in(data, leftshift, 1)), 0);
        data.op_set_input(leftshift, c, 1).expect("set input");
        data.op_set_input(op, newvn, 0).expect("set input");
        data.op_remove_input(op, 1); // Remove the right-shift constant
        op_set_opcode(
            data,
            op,
            if op_code(data, op) == OpCode::CPUI_INT_SRIGHT {
                OpCode::CPUI_INT_SEXT
            } else {
                OpCode::CPUI_INT_ZEXT
            },
        );
        1
    }
}

// =============================================================================
// RuleShiftCompare
// =============================================================================

/// `V >> c == d  =>  V == (d << c)` (and the left-shift / mult / div variants).
/// C++ `RuleShiftCompare` (`ruleaction.cc:2063`).
pub struct RuleShiftCompare;

impl Rule for RuleShiftCompare {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_INT_EQUAL, OpCode::CPUI_INT_NOTEQUAL]
    }
    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains("analysis") && !grouplist.contains("shiftcompare") {
            return None;
        }
        Some(Box::new(RuleShiftCompare))
    }
    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        let mut sa: int4;
        let isleft: bool;

        let shiftvn = op_in(data, op, 0);
        let constvn = op_in(data, op, 1);
        if !vn_constant(data, constvn) {
            return 0;
        }
        if !vn_written(data, shiftvn) {
            return 0;
        }
        let shiftop = vn_def(data, shiftvn).expect("shiftop def");
        let opc = op_code(data, shiftop);
        let savn;
        if opc == OpCode::CPUI_INT_LEFT {
            isleft = true;
            savn = op_in(data, shiftop, 1);
            if !vn_constant(data, savn) {
                return 0;
            }
            sa = vn_offset(data, savn) as int4;
        } else if opc == OpCode::CPUI_INT_RIGHT {
            isleft = false;
            savn = op_in(data, shiftop, 1);
            if !vn_constant(data, savn) {
                return 0;
            }
            sa = vn_offset(data, savn) as int4;
            // We only apply when we know we will eliminate a variable
            if data.lone_descend(shiftvn) != Some(op) {
                return 0;
            }
        } else if opc == OpCode::CPUI_INT_MULT {
            isleft = true;
            savn = op_in(data, shiftop, 1);
            if !vn_constant(data, savn) {
                return 0;
            }
            let val = vn_offset(data, savn);
            sa = leastsigbit_set(val);
            // sa == -1 when val == 0; C++ shifts by (-1 & 0x3f) == 63 (x86, ADR-0003).
            if val.wshr(sa as u32) != 1u64 {
                return 0; // Not multiplying by a power of 2
            }
        } else if opc == OpCode::CPUI_INT_DIV {
            isleft = false;
            savn = op_in(data, shiftop, 1);
            if !vn_constant(data, savn) {
                return 0;
            }
            let val = vn_offset(data, savn);
            sa = leastsigbit_set(val);
            // sa == -1 when val == 0; C++ shifts by (-1 & 0x3f) == 63 (x86, ADR-0003).
            if val.wshr(sa as u32) != 1u64 {
                return 0; // Not dividing by a power of 2
            }
            if data.lone_descend(shiftvn) != Some(op) {
                return 0;
            }
        } else {
            return 0;
        }

        if sa == 0 {
            return 0;
        }
        let mainvn = op_in(data, shiftop, 0);
        if vn_free(data, mainvn) {
            return 0;
        }
        if vn_size(data, mainvn) as usize > std::mem::size_of::<uintb>() {
            return 0; // FIXME: uintb should be arbitrary precision
        }

        let constval = vn_offset(data, constvn);
        let nzmask = vn_nzmask(data, mainvn);
        let newconst;
        // `sa` is a raw constant shift amount and can be >=64 (LEFT/RIGHT case);
        // every value-word shift below is x86-masked in C++, so use the wrapping
        // shifts (ADR-0003) — identical to bare `>>`/`<<` for sa<64.
        if isleft {
            newconst = constval.wshr(sa as u32);
            if newconst.wshl(sa as u32) != constval {
                return 0; // Information lost in constval
            }
            let tmp = nzmask.wshl(sa as u32) & calc_mask(vn_size(data, shiftvn));
            if tmp.wshr(sa as u32) != nzmask {
                // Information is lost in main
                // We replace the LEFT with and AND mask
                // This must be the lone use of the shift
                if data.lone_descend(shiftvn) != Some(op) {
                    return 0;
                }
                sa = 8 * vn_size(data, shiftvn) - sa;
                let tmp = (1u64).wshl(sa as u32).wrapping_sub(1);
                let newmask = data.new_constant(vn_size(data, constvn), tmp);
                let addr = op_addr(data, op);
                let newop = data.new_op(2, addr);
                op_set_opcode(data, newop, OpCode::CPUI_INT_AND);
                let newtmpvn = new_unique_out(data, vn_size(data, constvn), newop);
                data.op_set_input(newop, mainvn, 0).expect("set input");
                data.op_set_input(newop, newmask, 1).expect("set input");
                data.op_insert_before(newop, shiftop);
                data.op_set_input(op, newtmpvn, 0).expect("set input");
                let nc = data.new_constant(vn_size(data, constvn), newconst);
                data.op_set_input(op, nc, 1).expect("set input");
                return 1;
            }
        } else {
            if nzmask.wshr(sa as u32).wshl(sa as u32) != nzmask {
                return 0; // Information is lost
            }
            newconst = constval.wshl(sa as u32) & calc_mask(vn_size(data, shiftvn));
            if newconst.wshr(sa as u32) != constval {
                return 0; // Information is lost in constval
            }
        }
        let newconstvn = data.new_constant(vn_size(data, constvn), newconst);
        data.op_set_input(op, mainvn, 0).expect("set input");
        data.op_set_input(op, newconstvn, 1).expect("set input");
        1
    }
}

// =============================================================================
// RuleLessEqual
// =============================================================================

/// `V < W || V == W  =>  V <= W` (and the `!=` / signed variants).  C++
/// `RuleLessEqual` (`ruleaction.cc:2249`).
pub struct RuleLessEqual;

impl Rule for RuleLessEqual {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_BOOL_OR]
    }
    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains("analysis") && !grouplist.contains("lessequal") {
            return None;
        }
        Some(Box::new(RuleLessEqual))
    }
    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        let vnout1 = op_in(data, op, 0);
        if !vn_written(data, vnout1) {
            return 0;
        }
        let vnout2 = op_in(data, op, 1);
        if !vn_written(data, vnout2) {
            return 0;
        }
        let mut op_less = vn_def(data, vnout1).expect("op_less def");
        let mut opc = op_code(data, op_less);
        let op_equal;
        if (opc != OpCode::CPUI_INT_LESS) && (opc != OpCode::CPUI_INT_SLESS) {
            op_equal = op_less;
            op_less = vn_def(data, vnout2).expect("op_less def");
            opc = op_code(data, op_less);
            if (opc != OpCode::CPUI_INT_LESS) && (opc != OpCode::CPUI_INT_SLESS) {
                return 0;
            }
        } else {
            op_equal = vn_def(data, vnout2).expect("op_equal def");
        }
        let equalopc = op_code(data, op_equal);
        if (equalopc != OpCode::CPUI_INT_EQUAL) && (equalopc != OpCode::CPUI_INT_NOTEQUAL) {
            return 0;
        }

        let compvn1 = op_in(data, op_less, 0);
        let compvn2 = op_in(data, op_less, 1);
        if !vn_heritage_known(data, compvn1) {
            return 0;
        }
        if !vn_heritage_known(data, compvn2) {
            return 0;
        }
        let eq0 = op_in(data, op_equal, 0);
        let eq1 = op_in(data, op_equal, 1);
        if (!varnodes_equiv(data, compvn1, eq0) || !varnodes_equiv(data, compvn2, eq1))
            && (!varnodes_equiv(data, compvn1, eq1) || !varnodes_equiv(data, compvn2, eq0))
        {
            return 0;
        }

        if equalopc == OpCode::CPUI_INT_NOTEQUAL {
            // op_less is redundant
            op_set_opcode(data, op, OpCode::CPUI_COPY); // Convert OR to COPY
            data.op_remove_input(op, 1);
            let eqout = op_out(data, op_equal);
            data.op_set_input(op, eqout, 0).expect("set input"); // Taking the NOTEQUAL output
        } else {
            data.op_set_input(op, compvn1, 0).expect("set input");
            data.op_set_input(op, compvn2, 1).expect("set input");
            op_set_opcode(
                data,
                op,
                if opc == OpCode::CPUI_INT_SLESS {
                    OpCode::CPUI_INT_SLESSEQUAL
                } else {
                    OpCode::CPUI_INT_LESSEQUAL
                },
            );
        }
        1
    }
}

// =============================================================================
// RuleLessNotEqual
// =============================================================================

/// `V <= W && V != W  =>  V < W` (and the signed variant).  C++
/// `RuleLessNotEqual` (`ruleaction.cc:2309`).
pub struct RuleLessNotEqual;

impl Rule for RuleLessNotEqual {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_BOOL_AND]
    }
    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains("analysis") && !grouplist.contains("lessnotequal") {
            return None;
        }
        Some(Box::new(RuleLessNotEqual))
    }
    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // Convert [(s)lessequal AND notequal] to (s)less
        let vnout1 = op_in(data, op, 0);
        if !vn_written(data, vnout1) {
            return 0;
        }
        let vnout2 = op_in(data, op, 1);
        if !vn_written(data, vnout2) {
            return 0;
        }
        let mut op_less = vn_def(data, vnout1).expect("op_less def");
        let mut opc = op_code(data, op_less);
        let op_equal;
        if (opc != OpCode::CPUI_INT_LESSEQUAL) && (opc != OpCode::CPUI_INT_SLESSEQUAL) {
            op_equal = op_less;
            op_less = vn_def(data, vnout2).expect("op_less def");
            opc = op_code(data, op_less);
            if (opc != OpCode::CPUI_INT_LESSEQUAL) && (opc != OpCode::CPUI_INT_SLESSEQUAL) {
                return 0;
            }
        } else {
            op_equal = vn_def(data, vnout2).expect("op_equal def");
        }
        if op_code(data, op_equal) != OpCode::CPUI_INT_NOTEQUAL {
            return 0;
        }

        let compvn1 = op_in(data, op_less, 0);
        let compvn2 = op_in(data, op_less, 1);
        if !vn_heritage_known(data, compvn1) {
            return 0;
        }
        if !vn_heritage_known(data, compvn2) {
            return 0;
        }
        let eq0 = op_in(data, op_equal, 0);
        let eq1 = op_in(data, op_equal, 1);
        if (!varnodes_equiv(data, compvn1, eq0) || !varnodes_equiv(data, compvn2, eq1))
            && (!varnodes_equiv(data, compvn1, eq1) || !varnodes_equiv(data, compvn2, eq0))
        {
            return 0;
        }

        data.op_set_input(op, compvn1, 0).expect("set input");
        data.op_set_input(op, compvn2, 1).expect("set input");
        op_set_opcode(
            data,
            op,
            if opc == OpCode::CPUI_INT_SLESSEQUAL {
                OpCode::CPUI_INT_SLESS
            } else {
                OpCode::CPUI_INT_LESS
            },
        );
        1
    }
}

// =============================================================================
// RuleTrivialArith
// =============================================================================

/// Simplify binary ops whose two inputs hold the same value (`V == V => true`,
/// `V & V => V`, etc.).  C++ `RuleTrivialArith` (`ruleaction.cc:2358`).
pub struct RuleTrivialArith;

impl Rule for RuleTrivialArith {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![
            OpCode::CPUI_INT_NOTEQUAL,
            OpCode::CPUI_INT_SLESS,
            OpCode::CPUI_INT_LESS,
            OpCode::CPUI_BOOL_XOR,
            OpCode::CPUI_BOOL_AND,
            OpCode::CPUI_BOOL_OR,
            OpCode::CPUI_INT_EQUAL,
            OpCode::CPUI_INT_SLESSEQUAL,
            OpCode::CPUI_INT_LESSEQUAL,
            OpCode::CPUI_INT_XOR,
            OpCode::CPUI_INT_AND,
            OpCode::CPUI_INT_OR,
            OpCode::CPUI_FLOAT_EQUAL,
            OpCode::CPUI_FLOAT_NOTEQUAL,
            OpCode::CPUI_FLOAT_LESS,
            OpCode::CPUI_FLOAT_LESSEQUAL,
        ]
    }
    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        // C++ `RuleTrivialArith::clone` (ruleaction.hh:493) gates on
        // `grouplist.contains(getGroup())`, where `getGroup()` is the group this
        // rule was constructed with: `new RuleTrivialArith("analysis")`
        // (coreaction.cc:5786) -- it is NOT the rule's `name` ("trivialarith").
        // This unit struct is registered under the "analysis" group
        // (universalaction.rs:399); the owning pool's `clone_filtered` consults
        // exactly this gate, so it must test the GROUP, or the rule is silently
        // dropped (it would not survive the "decompile"/"jumptable" grouplists,
        // which list "analysis" but never the literal "trivialarith").  The
        // `V & V => V` / `V ^ V => 0` folds it performs are what let the loaded
        // byte in a `*ptr` / signed-compare cluster type as `char` (the signed
        // compare then reads the byte directly rather than a dead self-AND
        // output) -- matching the C++ `char v1` rendering.
        if !grouplist.contains("analysis") {
            return None;
        }
        Some(Box::new(RuleTrivialArith))
    }
    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // LOCAL STUB(jumptable-recovery): C++ keeps this fold active during the
        // dedicated jump-table recovery sub-pass (`isJumptableRecoveryOn`) and
        // relies on `ActionRestructureVarnode::protectSwitchPaths`
        // (coreaction.cc:2320) to mark the switch value's INDIRECT data-flow
        // `setNoIndirectCollapse` so the self-`INT_XOR`/`INT_AND` switch-index
        // init can fold to a constant WITHOUT erasing the indirect index.  That
        // protection walk does not yet port cleanly onto the Rust recovery model
        // (it over-protects INDIRECTs the Rust table builder must collapse), so
        // until it is ported we suppress this normalization ONLY inside the
        // recovery partial.  The observable effect matches the C++ oracle: the
        // table is recovered in full, and the MAIN decompile still folds the
        // self-op (the byte-deref / signed-compare cluster types `char`).  This
        // is a pass-context gate, not function/address/type-name special-casing.
        if data.is_jumptable_recovery_on() {
            return 0;
        }
        if op_num_input(data, op) != 2 {
            return 0;
        }
        let in0 = op_in(data, op, 0);
        let in1 = op_in(data, op, 1);
        if in0 != in1 {
            // Inputs must be identical
            if !vn_written(data, in0) {
                return 0;
            }
            if !vn_written(data, in1) {
                return 0;
            }
            // or constructed identically
            let def0 = vn_def(data, in0).expect("in0 def");
            let def1 = vn_def(data, in1).expect("in1 def");
            let o0 = data.obank().get(def0).expect("stale def op");
            let o1 = data.obank().get(def1).expect("stale def op");
            if !is_cse_match(o0, o1, data.vbank()) {
                return 0;
            }
        }
        // vn: replacement input for COPY (None means "keep getIn(0)").
        let vn: Option<VarnodeId> = match op_code(data, op) {
            OpCode::CPUI_INT_NOTEQUAL // Boolean 0
            | OpCode::CPUI_INT_SLESS
            | OpCode::CPUI_INT_LESS
            | OpCode::CPUI_BOOL_XOR
            | OpCode::CPUI_FLOAT_NOTEQUAL
            | OpCode::CPUI_FLOAT_LESS => Some(data.new_constant(1, 0)),
            OpCode::CPUI_INT_EQUAL // Boolean 1
            | OpCode::CPUI_INT_SLESSEQUAL
            | OpCode::CPUI_INT_LESSEQUAL
            | OpCode::CPUI_FLOAT_EQUAL
            | OpCode::CPUI_FLOAT_LESSEQUAL => Some(data.new_constant(1, 1)),
            OpCode::CPUI_INT_XOR => {
                // Same size 0
                let s = vn_size(data, op_out(data, op));
                Some(data.new_constant(s, 0))
            }
            OpCode::CPUI_BOOL_AND // Identity
            | OpCode::CPUI_BOOL_OR
            | OpCode::CPUI_INT_AND
            | OpCode::CPUI_INT_OR => None,
            _ => return 0,
        };

        data.op_remove_input(op, 1);
        op_set_opcode(data, op, OpCode::CPUI_COPY);
        if let Some(vn) = vn {
            data.op_set_input(op, vn, 0).expect("set input");
        }
        1
    }
}

// =============================================================================
// RuleTrivialBool
// =============================================================================

/// Simplify a boolean op with one constant operand (`V && false => false`,
/// etc.).  C++ `RuleTrivialBool` (`ruleaction.cc:2434`).
pub struct RuleTrivialBool;

impl Rule for RuleTrivialBool {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_BOOL_AND, OpCode::CPUI_BOOL_OR, OpCode::CPUI_BOOL_XOR]
    }
    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains("analysis") && !grouplist.contains("trivialbool") {
            return None;
        }
        Some(Box::new(RuleTrivialBool))
    }
    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        let vnconst = op_in(data, op, 1);
        if !vn_constant(data, vnconst) {
            return 0;
        }
        let val = vn_offset(data, vnconst);

        let vn;
        let opc;
        match op_code(data, op) {
            OpCode::CPUI_BOOL_XOR => {
                vn = op_in(data, op, 0);
                opc = if val == 1 { OpCode::CPUI_BOOL_NEGATE } else { OpCode::CPUI_COPY };
            }
            OpCode::CPUI_BOOL_AND => {
                opc = OpCode::CPUI_COPY;
                vn = if val == 1 { op_in(data, op, 0) } else { data.new_constant(1, 0) }; // Copy false
            }
            OpCode::CPUI_BOOL_OR => {
                opc = OpCode::CPUI_COPY;
                vn = if val == 1 { data.new_constant(1, 1) } else { op_in(data, op, 0) };
            }
            _ => return 0,
        }

        data.op_remove_input(op, 1);
        op_set_opcode(data, op, opc);
        data.op_set_input(op, vn, 0).expect("set input");
        1
    }
}

// =============================================================================
// RuleZextEliminate
// =============================================================================

/// `zext(V) == c  =>  V == c` (and `!=`, `<`, `<=` variants).  C++
/// `RuleZextEliminate` (`ruleaction.cc:2490`).
pub struct RuleZextEliminate;

impl Rule for RuleZextEliminate {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![
            OpCode::CPUI_INT_EQUAL,
            OpCode::CPUI_INT_NOTEQUAL,
            OpCode::CPUI_INT_LESS,
            OpCode::CPUI_INT_LESSEQUAL,
        ]
    }
    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains("analysis") && !grouplist.contains("zexteliminate") {
            return None;
        }
        Some(Box::new(RuleZextEliminate))
    }
    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // vn1 equals ZEXTed input; vn2 = other input
        let mut vn1 = op_in(data, op, 0);
        let mut vn2 = op_in(data, op, 1);
        let mut zextslot: int4 = 0;
        let mut otherslot: int4 = 1;
        if vn_written(data, vn2)
            && op_code(data, vn_def(data, vn2).expect("vn2 def")) == OpCode::CPUI_INT_ZEXT
        {
            vn1 = vn2;
            vn2 = op_in(data, op, 0);
            zextslot = 1;
            otherslot = 0;
        } else if !vn_written(data, vn1)
            || op_code(data, vn_def(data, vn1).expect("vn1 def")) != OpCode::CPUI_INT_ZEXT
        {
            return 0;
        }

        if !vn_constant(data, vn2) {
            return 0;
        }
        let zext = vn_def(data, vn1).expect("zext def");
        if !vn_heritage_known(data, op_in(data, zext, 0)) {
            return 0;
        }
        if data.lone_descend(vn1) != Some(op) {
            return 0; // Make sure extension is not used for anything else
        }
        let smallsize = vn_size(data, op_in(data, zext, 0));
        let val = vn_offset(data, vn2);
        // C++ `val>>(8*smallsize)` on a `uintb`: for an 8-byte ZEXT input the
        // count is 64, and the C++ oracle relies on x86 shift-count masking
        // (count & 0x3f) to yield `val>>0 == val`.  `wshr` (ADR-0003) replicates
        // that masking; for smallsize<8 the count is <64 and `wshr` == `>>`.
        if val.wshr((8 * smallsize) as u32) == 0 {
            // Is zero extension unnecessary
            let newvn = data.new_constant(smallsize, val);
            // newvn->copySymbolIfValid(vn2): copy data-type/symbol if valid.
            // STUB(W6): copySymbolIfValid — deferred.
            let _ = newvn;
            let zin0 = op_in(data, zext, 0);
            data.op_set_input(op, zin0, zextslot).expect("set input");
            data.op_set_input(op, newvn, otherslot).expect("set input");
            return 1;
        }
        // Should have else for doing constant comparison here and now
        0
    }
}

// =============================================================================
// RuleSlessToLess
// =============================================================================

/// Convert INT_SLESS(EQUAL) to INT_LESS(EQUAL) when both operands are known
/// non-negative.  C++ `RuleSlessToLess` (`ruleaction.cc:2547`).
pub struct RuleSlessToLess;

impl Rule for RuleSlessToLess {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_INT_SLESS, OpCode::CPUI_INT_SLESSEQUAL]
    }
    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains("analysis") && !grouplist.contains("slesstoless") {
            return None;
        }
        Some(Box::new(RuleSlessToLess))
    }
    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        let vn = op_in(data, op, 0);
        let sz = vn_size(data, vn);
        if signbit_negative(vn_nzmask(data, vn), sz) {
            return 0;
        }
        if signbit_negative(vn_nzmask(data, op_in(data, op, 1)), sz) {
            return 0;
        }

        if op_code(data, op) == OpCode::CPUI_INT_SLESS {
            op_set_opcode(data, op, OpCode::CPUI_INT_LESS);
        } else {
            op_set_opcode(data, op, OpCode::CPUI_INT_LESSEQUAL);
        }
        1
    }
}

// =============================================================================
// RuleZextSless
// =============================================================================

/// `zext(V) s< c  =>  V < c`.  C++ `RuleZextSless` (`ruleaction.cc:2574`).
pub struct RuleZextSless;

impl Rule for RuleZextSless {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_INT_SLESS, OpCode::CPUI_INT_SLESSEQUAL]
    }
    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains("analysis") && !grouplist.contains("zextsless") {
            return None;
        }
        Some(Box::new(RuleZextSless))
    }
    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        let mut vn1 = op_in(data, op, 0);
        let mut vn2 = op_in(data, op, 1);
        let mut zextslot: int4 = 0;
        let mut otherslot: int4 = 1;
        if vn_written(data, vn2)
            && op_code(data, vn_def(data, vn2).expect("vn2 def")) == OpCode::CPUI_INT_ZEXT
        {
            vn1 = vn2;
            vn2 = op_in(data, op, 0);
            zextslot = 1;
            otherslot = 0;
        } else if !vn_written(data, vn1)
            || op_code(data, vn_def(data, vn1).expect("vn1 def")) != OpCode::CPUI_INT_ZEXT
        {
            return 0;
        }

        if !vn_constant(data, vn2) {
            return 0;
        }
        let zext = vn_def(data, vn1).expect("zext def");
        if !vn_heritage_known(data, op_in(data, zext, 0)) {
            return 0;
        }

        let smallsize = vn_size(data, op_in(data, zext, 0));
        let val = vn_offset(data, vn2);
        // C++ `val>>(8*smallsize-1)` on a `uintb`: for a valid comparison
        // smallsize<8 so the count is <64, but the C++ has no precision guard and
        // a wider ZEXT input would push the count past 63; `wshr` (ADR-0003)
        // replicates the x86 shift-count masking and is `>>` for count<64.
        if val.wshr((8 * smallsize - 1) as u32) != 0 {
            return 0; // Is zero extension unnecessary, sign bit must also be 0
        }

        let newvn = data.new_constant(smallsize, val);
        let zin0 = op_in(data, zext, 0);
        data.op_set_input(op, zin0, zextslot).expect("set input");
        data.op_set_input(op, newvn, otherslot).expect("set input");
        op_set_opcode(
            data,
            op,
            if op_code(data, op) == OpCode::CPUI_INT_SLESS {
                OpCode::CPUI_INT_LESS
            } else {
                OpCode::CPUI_INT_LESSEQUAL
            },
        );
        1
    }
}

// =============================================================================
// RuleBitUndistribute
// =============================================================================

/// `zext(V) & zext(W)  =>  zext( V & W )` and `(V >> X) | (W >> X)  =>  (V | W)
/// >> X`.  C++ `RuleBitUndistribute` (`ruleaction.cc:2619`).
pub struct RuleBitUndistribute;

impl Rule for RuleBitUndistribute {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_INT_AND, OpCode::CPUI_INT_OR, OpCode::CPUI_INT_XOR]
    }
    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains("analysis") && !grouplist.contains("bitundistribute") {
            return None;
        }
        Some(Box::new(RuleBitUndistribute))
    }
    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        let vn1 = op_in(data, op, 0);
        let vn2 = op_in(data, op, 1);
        let in1;
        let in2;

        if !vn_written(data, vn1) {
            return 0;
        }
        if !vn_written(data, vn2) {
            return 0;
        }

        let opc = op_code(data, vn_def(data, vn1).expect("vn1 def"));
        if op_code(data, vn_def(data, vn2).expect("vn2 def")) != opc {
            return 0;
        }
        match opc {
            OpCode::CPUI_INT_ZEXT | OpCode::CPUI_INT_SEXT => {
                // Test for full equality of extension operation
                in1 = op_in(data, vn_def(data, vn1).expect("vn1 def"), 0);
                if vn_free(data, in1) {
                    return 0;
                }
                in2 = op_in(data, vn_def(data, vn2).expect("vn2 def"), 0);
                if vn_free(data, in2) {
                    return 0;
                }
                if vn_size(data, in1) != vn_size(data, in2) {
                    return 0;
                }
                data.op_remove_input(op, 1);
            }
            OpCode::CPUI_INT_LEFT | OpCode::CPUI_INT_RIGHT | OpCode::CPUI_INT_SRIGHT => {
                // Test for full equality of shift operation
                let s1 = op_in(data, vn_def(data, vn1).expect("vn1 def"), 1);
                let s2 = op_in(data, vn_def(data, vn2).expect("vn2 def"), 1);
                let vnextra;
                if vn_constant(data, s1) && vn_constant(data, s2) {
                    if vn_offset(data, s1) != vn_offset(data, s2) {
                        return 0;
                    }
                    vnextra = data.new_constant(vn_size(data, s1), vn_offset(data, s1));
                } else if s1 != s2 {
                    return 0;
                } else {
                    if vn_free(data, s1) {
                        return 0;
                    }
                    vnextra = s1;
                }
                in1 = op_in(data, vn_def(data, vn1).expect("vn1 def"), 0);
                if vn_free(data, in1) {
                    return 0;
                }
                in2 = op_in(data, vn_def(data, vn2).expect("vn2 def"), 0);
                if vn_free(data, in2) {
                    return 0;
                }
                data.op_set_input(op, vnextra, 1).expect("set input");
            }
            _ => return 0,
        }

        let addr = op_addr(data, op);
        let newext = data.new_op(2, addr);
        let smalllogic = new_unique_out(data, vn_size(data, in1), newext);
        data.op_set_input(newext, in1, 0).expect("set input");
        data.op_set_input(newext, in2, 1).expect("set input");
        let opcode = op_code(data, op);
        op_set_opcode(data, newext, opcode);

        op_set_opcode(data, op, opc);
        data.op_set_input(op, smalllogic, 0).expect("set input");
        data.op_insert_before(newext, op);
        1
    }
}

// =============================================================================
// RuleBooleanUndistribute
// =============================================================================

/// Boolean-match result, mirroring `BooleanMatch`'s enum (`expression.hh`).
mod boolean_match {
    /// Pair always hold the same value.
    pub const SAME: i32 = 1;
    /// Pair always hold complementary values.
    pub const COMPLEMENTARY: i32 = 2;
    /// Pair values are uncorrelated.
    pub const UNCORRELATED: i32 = 3;
}

/// `BooleanMatch::evaluate(vn1, vn2, depth)` — // STUB(W5-expression).
///
/// The real algorithm lives in `expression.cc` (a different wave's module, still
/// a stub).  This stub handles only the trivial `vn1 == vn2 => same` head of the
/// C++ routine; every other pair is reported `uncorrelated`, which makes
/// [`RuleBooleanUndistribute`] conservatively decline (a faithful no-op, never a
/// wrong transform).  Recorded as a loss.
fn boolean_match_evaluate(_data: &Funcdata, vn1: VarnodeId, vn2: VarnodeId, _depth: int4) -> int4 {
    if vn1 == vn2 {
        return boolean_match::SAME;
    }
    // STUB(W5-expression): the recursive structural/complementary match is in
    // expression.cc; until ported, decline.
    let _ = boolean_match::COMPLEMENTARY;
    boolean_match::UNCORRELATED
}

/// Undo distributed BOOL_AND/BOOL_OR through INT_(NOT)EQUAL.  C++
/// `RuleBooleanUndistribute` (`ruleaction.cc:2696`).
pub struct RuleBooleanUndistribute;

impl RuleBooleanUndistribute {
    /// `RuleBooleanUndistribute::isMatch` (`ruleaction.cc:2717`): test if the two
    /// Varnodes are matching boolean expressions; `right_flip` is toggled when
    /// they are complementary.
    fn is_match(data: &Funcdata, left_vn: VarnodeId, right_vn: VarnodeId, right_flip: &mut bool) -> bool {
        let val = boolean_match_evaluate(data, left_vn, right_vn, 1);
        if val == boolean_match::SAME {
            return true;
        }
        if val == boolean_match::COMPLEMENTARY {
            *right_flip = !*right_flip;
            return true;
        }
        false
    }
}

impl Rule for RuleBooleanUndistribute {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_INT_EQUAL, OpCode::CPUI_INT_NOTEQUAL]
    }
    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains("analysis") && !grouplist.contains("booleanundistribute") {
            return None;
        }
        Some(Box::new(RuleBooleanUndistribute))
    }
    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        let vn0 = op_in(data, op, 0);
        if !vn_written(data, vn0) {
            return 0;
        }
        let vn1 = op_in(data, op, 1);
        if !vn_written(data, vn1) {
            return 0;
        }
        let op0 = vn_def(data, vn0).expect("op0 def");
        let opc0 = op_code(data, op0);
        if opc0 != OpCode::CPUI_BOOL_AND && opc0 != OpCode::CPUI_BOOL_OR {
            return 0;
        }
        let op1 = vn_def(data, vn1).expect("op1 def");
        let opc1 = op_code(data, op1);
        if opc1 != OpCode::CPUI_BOOL_AND && opc1 != OpCode::CPUI_BOOL_OR {
            return 0;
        }
        let ins: [VarnodeId; 4] =
            [op_in(data, op0, 0), op_in(data, op0, 1), op_in(data, op1, 0), op_in(data, op1, 1)];
        if vn_free(data, ins[0])
            || vn_free(data, ins[1])
            || vn_free(data, ins[2])
            || vn_free(data, ins[3])
        {
            return 0;
        }
        let mut isflipped = [false; 4];
        let mut central_equal = op_code(data, op) == OpCode::CPUI_INT_EQUAL;
        if opc0 == OpCode::CPUI_BOOL_OR {
            isflipped[0] = !isflipped[0];
            isflipped[1] = !isflipped[1];
            central_equal = !central_equal;
        }
        if opc1 == OpCode::CPUI_BOOL_OR {
            isflipped[2] = !isflipped[2];
            isflipped[3] = !isflipped[3];
            central_equal = !central_equal;
        }
        let left_slot: usize;
        let right_slot: usize;
        if Self::is_match(data, ins[0], ins[2], &mut isflipped[2]) {
            left_slot = 0;
            right_slot = 2;
        } else if Self::is_match(data, ins[0], ins[3], &mut isflipped[3]) {
            left_slot = 0;
            right_slot = 3;
        } else if Self::is_match(data, ins[1], ins[2], &mut isflipped[2]) {
            left_slot = 1;
            right_slot = 2;
        } else if Self::is_match(data, ins[1], ins[3], &mut isflipped[3]) {
            left_slot = 1;
            right_slot = 3;
        } else {
            return 0;
        }
        if isflipped[left_slot] != isflipped[right_slot] {
            return 0;
        }
        let combine_opc;
        if central_equal {
            combine_opc = OpCode::CPUI_BOOL_OR;
            isflipped[left_slot] = !isflipped[left_slot];
        } else {
            combine_opc = OpCode::CPUI_BOOL_AND;
        }
        let mut final_a = ins[left_slot];
        if isflipped[left_slot] {
            final_a = op_bool_negate(data, final_a, op, false);
        }
        if isflipped[1 - left_slot] {
            central_equal = !central_equal;
        }
        if isflipped[5 - right_slot] {
            central_equal = !central_equal;
        }
        let final_b = ins[1 - left_slot];
        let final_c = ins[5 - right_slot];
        let addr = op_addr(data, op);
        let eq_op = data.new_op(2, addr);
        op_set_opcode(
            data,
            eq_op,
            if central_equal { OpCode::CPUI_INT_EQUAL } else { OpCode::CPUI_INT_NOTEQUAL },
        );
        let tmp1 = new_unique_out(data, 1, eq_op);
        data.op_set_input(eq_op, final_b, 0).expect("set input");
        data.op_set_input(eq_op, final_c, 1).expect("set input");
        data.op_insert_before(eq_op, op);
        op_set_opcode(data, op, combine_opc);
        data.op_set_input(op, tmp1, 1).expect("set input");
        data.op_set_input(op, final_a, 0).expect("set input");
        1
    }
}

// =============================================================================
// RuleSpec registration (C++ definition order — W8 assembles universalAction
// from these)
// =============================================================================

/// The rules ported in this file, in C++ definition order (`ruleaction.cc`
/// 1444-2810).  Each `group` is the rule's stage group (its C++ constructor
/// argument), matched against [`ActionGroupList`] in `clone_rule`.
pub fn specs() -> Vec<RuleSpec> {
    vec![
        RuleSpec { group: "floatrange", ctor: || Box::new(RuleFloatRange) },
        RuleSpec { group: "andcommute", ctor: || Box::new(RuleAndCommute) },
        RuleSpec { group: "andpiece", ctor: || Box::new(RuleAndPiece) },
        RuleSpec { group: "andzext", ctor: || Box::new(RuleAndZext) },
        RuleSpec { group: "andcompare", ctor: || Box::new(RuleAndCompare) },
        RuleSpec { group: "doublesub", ctor: || Box::new(RuleDoubleSub) },
        RuleSpec { group: "doubleshift", ctor: || Box::new(RuleDoubleShift) },
        RuleSpec { group: "doublearithshift", ctor: || Box::new(RuleDoubleArithShift) },
        RuleSpec { group: "concatshift", ctor: || Box::new(RuleConcatShift) },
        RuleSpec { group: "leftright", ctor: || Box::new(RuleLeftRight) },
        RuleSpec { group: "shiftcompare", ctor: || Box::new(RuleShiftCompare) },
        RuleSpec { group: "lessequal", ctor: || Box::new(RuleLessEqual) },
        RuleSpec { group: "lessnotequal", ctor: || Box::new(RuleLessNotEqual) },
        // C++ `new RuleTrivialArith("analysis")` (coreaction.cc:5786): the group
        // is "analysis", NOT the rule name -- `clone_rule` now gates on it so the
        // rule survives the "decompile"/"jumptable" grouplists (which list
        // "analysis" but never the literal "trivialarith").
        RuleSpec { group: "analysis", ctor: || Box::new(RuleTrivialArith) },
        RuleSpec { group: "trivialbool", ctor: || Box::new(RuleTrivialBool) },
        RuleSpec { group: "zexteliminate", ctor: || Box::new(RuleZextEliminate) },
        RuleSpec { group: "slesstoless", ctor: || Box::new(RuleSlessToLess) },
        RuleSpec { group: "zextsless", ctor: || Box::new(RuleZextSless) },
        RuleSpec { group: "bitundistribute", ctor: || Box::new(RuleBitUndistribute) },
        RuleSpec { group: "booleanundistribute", ctor: || Box::new(RuleBooleanUndistribute) },
    ]
}

#[cfg(test)]
mod tests;
