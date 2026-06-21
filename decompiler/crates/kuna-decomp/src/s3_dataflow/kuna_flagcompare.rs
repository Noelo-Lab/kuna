//! Port of `decompiler/cpp/kuna_flagcompare.{cc,hh}` — flag-modelled-comparison
//! folding (kuna GH-1276 / GH-8777, S3 simplification-quiescence).
//!
//! Two rules folded under the one arch flag `fold_flag_compare` (option
//! `flagcompare on|off`, shipped default `on`):
//!
//!   * [`RuleBoolSignLess`] (GH-1276): `(b << k) s< 0` -> `b != 0`, when the
//!     boolean's NZMask is exactly the single bit landing in the sign position.
//!   * [`RuleSborrowGe`] (GH-8777): the `N == V` signed-overflow idiom
//!     (`XNOR(sign(V-K), SBORROW(V,K))`, in either AND-of-ORs or OR-of-ANDs form)
//!     -> `INT_SLESSEQUAL(K, V)` (i.e. `V s>= K`).
//!
//! Both return 0 immediately when the gate is off, byte-identical to upstream.
//!
//! ## Gate wiring — SEAM(W4)
//!
//! Identical to `kuna_booleanmask`/`kuna_ovlesssimplify`: the C++
//! `if (!data.getArch()->fold_flag_compare) return 0;` gate is resolved at
//! construction (the seam `Funcdata::glb` does not carry the kuna analysis flags).
//! W8 threads `Architecture::fold_flag_compare`; [`specs`] uses the shipped
//! default (`on`).
//!
//! ## SEAM(W6) — opcode-flag resolution
//!
//! `opSetOpcode(op, ...)` resolves `glb->inst[opc]` (the W6 typeop `inst` table);
//! the rewrites build the [`TypeOp`] skeleton with a zero flag word until W6
//! lands.  Noted in the structured losses.

use kuna_base::address::calc_mask;
use kuna_base::types::{int4, uintb};
use kuna_num::opcodes::OpCode;

use crate::action::{ActionGroupList, Rule, RuleSpec};
use crate::funcdata::Funcdata;
use crate::seams::{OpId, TypeOp, VarnodeId};

// ===========================================================================
// GH-1276: (b << k) s< 0   ->   b != 0
// ===========================================================================

/// (kuna GH-1276) Fold `(b << k) s< 0` (a boolean shifted into the sign bit) to
/// `b != 0` (C++ `RuleBoolSignLess`).
pub struct RuleBoolSignLess {
    /// Resolved `glb->fold_flag_compare` gate (SEAM(W4)).
    enabled: bool,
    /// Rule group (C++ `Rule::basegroup`).
    group: String,
}

impl RuleBoolSignLess {
    /// Construct with the resolved gate.  Default group `"analysis"`.
    pub fn new(enabled: bool) -> RuleBoolSignLess {
        RuleBoolSignLess { enabled, group: String::from("analysis") }
    }
    /// Construct with an explicit group.
    pub fn with_group(enabled: bool, group: impl Into<String>) -> RuleBoolSignLess {
        RuleBoolSignLess { enabled, group: group.into() }
    }
}

impl Rule for RuleBoolSignLess {
    /// C++ `RuleBoolSignLess::getOpList`: `oplist.push_back(CPUI_INT_SLESS);`
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_INT_SLESS]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(&self.group) {
            return None;
        }
        Some(Box::new(RuleBoolSignLess { enabled: self.enabled, group: self.group.clone() }))
    }

    /// C++ `RuleBoolSignLess::applyOp` (`kuna_flagcompare.cc:23`) — transcribed.
    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // if (!data.getArch()->fold_flag_compare) return 0;  // (kuna) gate (SEAM(W4))
        if !self.enabled {
            return 0;
        }

        // op = INT_SLESS(shiftout, 0)
        let zero = match op_in(data, op, 1) {
            Some(v) => v,
            None => return 0,
        };
        // if (!zero->isConstant() || zero->getOffset() != 0) return 0;
        if !vn_is_constant(data, zero) || vn_offset(data, zero) != 0 {
            return 0;
        }

        let shiftout = match op_in(data, op, 0) {
            Some(v) => v,
            None => return 0,
        };
        if !vn_is_written(data, shiftout) {
            return 0;
        }
        let leftshift = vn_def(data, shiftout).expect("shiftout written");
        if op_code(data, leftshift) != OpCode::CPUI_INT_LEFT {
            return 0;
        }
        let ls1 = match op_in(data, leftshift, 1) {
            Some(v) => v,
            None => return 0,
        };
        if !vn_is_constant(data, ls1) {
            return 0;
        }
        // uintb sa = leftshift->getIn(1)->getOffset();  if (sa == 0) return 0;
        let sa: uintb = vn_offset(data, ls1);
        if sa == 0 {
            return 0;
        }

        let boolvn = match op_in(data, leftshift, 0) {
            Some(v) => v,
            None => return 0,
        };
        let width: int4 = vn_size(data, boolvn);
        // if ((uintb)width * 8 <= sa) return 0;
        if (width as uintb).wrapping_mul(8) <= sa {
            return 0;
        }
        // int4 signbitpos = width * 8 - 1 - (int4)sa;
        let signbitpos: int4 = width * 8 - 1 - (sa as int4);
        // uintb relevant = (uintb)1 << signbitpos;
        let relevant: uintb = 1u64 << signbitpos;
        // uintb above = ~(relevant - 1);  // mask of bits >= signbitpos
        let above: uintb = !(relevant.wrapping_sub(1));
        // if ((boolvn->getNZMask() & above) != relevant) return 0;
        if (vn_nzmask(data, boolvn) & above) != relevant {
            return 0;
        }
        // if (boolvn->getNZMask() != relevant) return 0;
        if vn_nzmask(data, boolvn) != relevant {
            return 0;
        }

        // Rewrite `(b << sa) s< 0`  =>  `b != 0`.
        // data.opSetOpcode(op,CPUI_INT_NOTEQUAL);
        // SEAM(W6): glb->inst[CPUI_INT_NOTEQUAL] property flags.
        data.op_set_opcode(op, TypeOp::new(OpCode::CPUI_INT_NOTEQUAL, 0, "INT_NOTEQUAL"));
        // data.opSetInput(op,boolvn,0);
        data.op_set_input(op, boolvn, 0)
            .expect("RuleBoolSignLess: opSetInput boolvn (internal invariant)");
        // data.opSetInput(op,data.newConstant(boolvn->getSize(),0),1);
        let z = data.new_constant(vn_size(data, boolvn), 0);
        data.op_set_input(op, z, 1)
            .expect("RuleBoolSignLess: opSetInput const (internal invariant)");
        1
    }
}

// ===========================================================================
// GH-8777: XNOR(N,V)  ->  SLESSEQUAL(K,V)   where  V s>= K
// ===========================================================================

/// (kuna GH-8777) Fold the `N == V` signed-overflow idiom to a clean signed
/// compare `INT_SLESSEQUAL(K, V)` (C++ `RuleSborrowGe`).
pub struct RuleSborrowGe {
    /// Resolved `glb->fold_flag_compare` gate (SEAM(W4)).
    enabled: bool,
    /// Rule group (C++ `Rule::basegroup`).
    group: String,
}

impl RuleSborrowGe {
    /// Construct with the resolved gate.  Default group `"analysis"`.
    pub fn new(enabled: bool) -> RuleSborrowGe {
        RuleSborrowGe { enabled, group: String::from("analysis") }
    }
    /// Construct with an explicit group.
    pub fn with_group(enabled: bool, group: impl Into<String>) -> RuleSborrowGe {
        RuleSborrowGe { enabled, group: group.into() }
    }
}

impl Rule for RuleSborrowGe {
    /// C++ `RuleSborrowGe::getOpList`: pushes CPUI_BOOL_AND then CPUI_BOOL_OR.
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_BOOL_AND, OpCode::CPUI_BOOL_OR]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(&self.group) {
            return None;
        }
        Some(Box::new(RuleSborrowGe { enabled: self.enabled, group: self.group.clone() }))
    }

    /// C++ `RuleSborrowGe::applyOp` (`kuna_flagcompare.cc:242`) — transcribed.
    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // if (!data.getArch()->fold_flag_compare) return 0;  // (kuna) gate (SEAM(W4))
        if !self.enabled {
            return 0;
        }

        // Varnode *nflag,*vflag;
        let (nflag, vflag) = if op_code(data, op) == OpCode::CPUI_BOOL_AND {
            match extract_and_of_ors(data, op) {
                Some(p) => p,
                None => return 0,
            }
        } else {
            // CPUI_BOOL_OR
            match extract_or_of_ands(data, op) {
                Some(p) => p,
                None => return 0,
            }
        };

        // if (!matchSborrowGe(nflag,vflag,&vbase,&kval,&sz)) return 0;
        let (vbase, kval, sz) = match match_sborrow_ge(data, nflag, vflag) {
            Some(t) => t,
            None => return 0,
        };

        // Rewrite the whole boolean tree to INT_SLESSEQUAL(K, V)  (K <= V).
        // data.opSetOpcode(op,CPUI_INT_SLESSEQUAL);
        // SEAM(W6): glb->inst[CPUI_INT_SLESSEQUAL] property flags.
        data.op_set_opcode(op, TypeOp::new(OpCode::CPUI_INT_SLESSEQUAL, 0, "INT_SLESSEQUAL"));
        // data.opSetInput(op,data.newConstant(sz,kval),0);
        let kvn = data.new_constant(sz, kval);
        data.op_set_input(op, kvn, 0)
            .expect("RuleSborrowGe: opSetInput const (internal invariant)");
        // data.opSetInput(op,vbase,1);
        data.op_set_input(op, vbase, 1)
            .expect("RuleSborrowGe: opSetInput vbase (internal invariant)");
        1
    }
}

/// Per-file registration rows in C++ DEFINITION ORDER: RuleBoolSignLess then
/// RuleSborrowGe (W8 assembles `universalAction` from these).
pub fn specs() -> Vec<RuleSpec> {
    vec![
        RuleSpec {
            group: "analysis",
            // Shipped default: `option flagcompare on` (fold_flag_compare = true;
            // kuna DIV-3 default-on, GH-1276/8777).
            ctor: || Box::new(RuleBoolSignLess::with_group(true, "analysis")),
        },
        RuleSpec {
            group: "analysis",
            ctor: || Box::new(RuleSborrowGe::with_group(true, "analysis")),
        },
    ]
}

// ---------------------------------------------------------------------------
// GH-8777 helpers (faithful transcriptions of the kuna_flagcompare.cc statics)
// ---------------------------------------------------------------------------

/// If `vn` is `BOOL_NEGATE(x)`, return x; else None (C++ `stripBoolNeg`).
fn strip_bool_neg(data: &Funcdata, vn: VarnodeId) -> Option<VarnodeId> {
    if !vn_is_written(data, vn) {
        return None;
    }
    let op = vn_def(data, vn)?;
    if op_code(data, op) != OpCode::CPUI_BOOL_NEGATE {
        return None;
    }
    op_in(data, op, 0)
}

/// Recognize the `V s>= K` flag idiom rooted at `{x, y}` where one is
/// `INT_SBORROW(V,K)` and the other the sign of `(V-K)` (C++ `matchSborrowGe`).
/// Returns `(vbase, kval, sz)`.  Order-independent.
fn match_sborrow_ge(data: &Funcdata, x: VarnodeId, y: VarnodeId) -> Option<(VarnodeId, uintb, int4)> {
    // Identify which is the SBORROW.
    let mut sb: Option<VarnodeId> = None;
    let mut sn: Option<VarnodeId> = None;
    for i in 0..2 {
        let cand = if i == 0 { x } else { y };
        if !vn_is_written(data, cand) {
            continue;
        }
        let cdef = vn_def(data, cand).expect("cand written");
        if op_code(data, cdef) == OpCode::CPUI_INT_SBORROW {
            sb = Some(cand);
            sn = Some(if i == 0 { y } else { x });
            break;
        }
    }
    let sb = sb?;
    let sn = sn?;

    let sbop = vn_def(data, sb).expect("sb written");
    let base = op_in(data, sbop, 0)?; // V
    let vkonst = op_in(data, sbop, 1)?; // K
    if !vn_is_constant(data, vkonst) {
        return None;
    }
    let kv: uintb = vn_offset(data, vkonst);
    let width: int4 = vn_size(data, base);

    // sn must be the sign-of-(V-K) flag (N), in one of the equivalent forms.
    if !vn_is_written(data, sn) {
        return None;
    }
    let snop = vn_def(data, sn).expect("sn written");
    let sn_sub: VarnodeId;
    let snc = op_code(data, snop);
    if snc == OpCode::CPUI_INT_SLESSEQUAL {
        // 0 s<= (V-K)
        let z = op_in(data, snop, 0)?;
        if !vn_is_constant(data, z) || vn_offset(data, z) != 0 {
            return None;
        }
        sn_sub = op_in(data, snop, 1)?;
    } else if snc == OpCode::CPUI_INT_SLESS {
        let a0 = op_in(data, snop, 0)?;
        let a1 = op_in(data, snop, 1)?;
        if vn_is_constant(data, a0) && vn_offset(data, a0) == calc_mask(vn_size(data, a0)) {
            sn_sub = a1; // -1 s< (V-K)
        } else if vn_is_constant(data, a1) && vn_offset(data, a1) == 0 {
            sn_sub = a0; // (V-K) s< 0
        } else {
            return None;
        }
    } else {
        return None;
    }

    if !vn_is_written(data, sn_sub) {
        return None;
    }
    let subop = vn_def(data, sn_sub).expect("sn_sub written");
    if op_code(data, subop) != OpCode::CPUI_INT_ADD {
        return None; // V + (-K)
    }
    let mut sub_base = op_in(data, subop, 0)?;
    let mut sub_k = op_in(data, subop, 1)?;
    // if (!subK->isConstant()) { swap; if still not constant -> fail }
    if !vn_is_constant(data, sub_k) {
        std::mem::swap(&mut sub_base, &mut sub_k);
        if !vn_is_constant(data, sub_k) {
            return None;
        }
    }
    if sub_base != base {
        return None;
    }
    if vn_size(data, sub_base) != width {
        return None;
    }
    // (subK->getOffset()) & calc_mask(width); then (negk + kv) & mask == 0
    let negk: uintb = vn_offset(data, sub_k) & calc_mask(width);
    if (negk.wrapping_add(kv) & calc_mask(width)) != 0 {
        return None;
    }

    Some((base, kv & calc_mask(width), width))
}

/// Extract the `{N,V}` pair from an AND-of-ORs XNOR tree
/// `BOOL_AND(BOOL_OR(N,!V), BOOL_OR(!N,V))` (C++ `extractAndOfOrs`).
fn extract_and_of_ors(data: &Funcdata, andop: OpId) -> Option<(VarnodeId, VarnodeId)> {
    let ia = op_in(data, andop, 0)?;
    let ib = op_in(data, andop, 1)?;
    if !vn_is_written(data, ia) || !vn_is_written(data, ib) {
        return None;
    }
    let or_a = vn_def(data, ia).expect("ia written");
    let or_b = vn_def(data, ib).expect("ib written");
    if op_code(data, or_a) != OpCode::CPUI_BOOL_OR || op_code(data, or_b) != OpCode::CPUI_BOOL_OR {
        return None;
    }
    // Split each OR limb into (plain, negated).
    let (p_a, g_a) = split_plain_neg(data, or_a)?;
    let (p_b, g_b) = split_plain_neg(data, or_b)?;
    // XNOR shape:  plain of A == negated of B,  plain of B == negated of A.
    if p_a != g_b {
        return None;
    }
    if p_b != g_a {
        return None;
    }
    Some((p_a, p_b))
}

/// Split a BOOL_OR into `(plain, negated)` operands; the C++ inline blocks in
/// `extractAndOfOrs`.  Exactly one operand must be a `BOOL_NEGATE`.
fn split_plain_neg(data: &Funcdata, orop: OpId) -> Option<(VarnodeId, VarnodeId)> {
    let in0 = op_in(data, orop, 0)?;
    let in1 = op_in(data, orop, 1)?;
    let na = strip_bool_neg(data, in0);
    let nb = strip_bool_neg(data, in1);
    // C++: exactly one operand is negated; pick the plain one and the stripped one.
    match (na, nb) {
        (None, Some(g)) => Some((in0, g)), // in0 plain, in1 = BOOL_NEGATE(g)
        (Some(g), None) => Some((in1, g)), // in1 plain, in0 = BOOL_NEGATE(g)
        _ => None,
    }
}

/// Extract the `{N,V}` pair from an OR-of-ANDs XNOR tree
/// `BOOL_OR(BOOL_AND(N,V), BOOL_AND(!N,!V))` (C++ `extractOrOfAnds`).
fn extract_or_of_ands(data: &Funcdata, orop: OpId) -> Option<(VarnodeId, VarnodeId)> {
    let ia = op_in(data, orop, 0)?;
    let ib = op_in(data, orop, 1)?;
    if !vn_is_written(data, ia) || !vn_is_written(data, ib) {
        return None;
    }
    let and_a = vn_def(data, ia).expect("ia written");
    let and_b = vn_def(data, ib).expect("ib written");
    if op_code(data, and_a) != OpCode::CPUI_BOOL_AND || op_code(data, and_b) != OpCode::CPUI_BOOL_AND
    {
        return None;
    }
    // Identify the negated AND (both operands under BOOL_NEGATE) and the plain AND.
    let pos_and: OpId;
    let n2: VarnodeId;
    let v2: VarnodeId;
    // try andB first (C++ checks andB's operands first)
    let ba0 = op_in(data, and_b, 0)?;
    let ba1 = op_in(data, and_b, 1)?;
    let na = strip_bool_neg(data, ba0);
    let nb = strip_bool_neg(data, ba1);
    if let (Some(g0), Some(g1)) = (na, nb) {
        // andB is the negated AND(!N,!V); andA is the plain pair.
        pos_and = and_a;
        n2 = g0;
        v2 = g1;
    } else {
        let aa0 = op_in(data, and_a, 0)?;
        let aa1 = op_in(data, and_a, 1)?;
        let na2 = strip_bool_neg(data, aa0);
        let nb2 = strip_bool_neg(data, aa1);
        if let (Some(g0), Some(g1)) = (na2, nb2) {
            pos_and = and_b;
            n2 = g0;
            v2 = g1;
        } else {
            return None;
        }
    }
    let p0 = op_in(data, pos_and, 0)?;
    let p1 = op_in(data, pos_and, 1)?;
    // if (!((p0==n2 && p1==v2) || (p0==v2 && p1==n2))) return false;
    if !((p0 == n2 && p1 == v2) || (p0 == v2 && p1 == n2)) {
        return None;
    }
    Some((p0, p1))
}

// --- Local IR read helpers (see kuna_booleanmask for the rationale) ----------

fn op_in(data: &Funcdata, op: OpId, slot: int4) -> Option<VarnodeId> {
    data.obank().get(op).and_then(|o| o.get_in(slot))
}
fn op_code(data: &Funcdata, op: OpId) -> OpCode {
    data.obank().get(op).expect("op_code: stale op").code()
}
fn vn_is_constant(data: &Funcdata, vn: VarnodeId) -> bool {
    data.vbank().get(vn).map(|v| v.is_constant()).unwrap_or(false)
}
fn vn_is_written(data: &Funcdata, vn: VarnodeId) -> bool {
    data.vbank().get(vn).map(|v| v.is_written()).unwrap_or(false)
}
fn vn_def(data: &Funcdata, vn: VarnodeId) -> Option<OpId> {
    data.vbank().get(vn).and_then(|v| v.get_def())
}
fn vn_offset(data: &Funcdata, vn: VarnodeId) -> uintb {
    data.vbank().get(vn).map(|v| v.get_offset()).unwrap_or(0)
}
fn vn_size(data: &Funcdata, vn: VarnodeId) -> int4 {
    data.vbank().get(vn).map(|v| v.get_size()).unwrap_or(0)
}
fn vn_nzmask(data: &Funcdata, vn: VarnodeId) -> uintb {
    data.vbank().get(vn).map(|v| v.get_nz_mask()).unwrap_or(0)
}

#[cfg(test)]
#[path = "kuna_flagcompare/tests.rs"]
mod tests;
