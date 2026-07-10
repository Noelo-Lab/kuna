//! Port of `decompiler/cpp/kuna_ovlesssimplify.{cc,hh}` — overflow-flag
//! signed-less-than simplification (kuna GH-7190, S3 simplification-quiescence).
//!
//! `RuleOvLessSimplify` collapses the OV-flag signed-less-than idiom
//!
//! ```text
//!   NE( SLESS(V+K, 0) , BOOL_AND( signTest , SLESS(-1, V+K) ) )
//! ```
//!
//! to a single `INT_SLESS(V, -K)`.  Gated default-on by the arch flag
//! `ov_less_simplify` (option `ovlesssimplify on|off`, shipped default `on`);
//! inert when off, byte-identical to upstream.
//!
//! ## Gate wiring — STUB(W4)
//!
//! As in `kuna_booleanmask`, the C++ `if (!data.getArch()->ov_less_simplify)
//! return 0;` gate is resolved at construction (the boundary `Funcdata::glb` does not
//! carry kuna analysis flags; the fixed [`Rule::apply_op`] signature cannot reach
//! the full `architecture.rs` [`Architecture`](crate::architecture::Architecture)).
//! W8 threads the live `Architecture::ov_less_simplify`; [`specs`] uses the
//! shipped default (`on`).
//!
//! ## STUB(W6) — opcode-flag resolution
//!
//! `opSetOpcode(op, CPUI_INT_SLESS)` resolves `glb->inst[CPUI_INT_SLESS]` in C++
//! (the W6 typeop `inst` table); the rewrite builds the [`TypeOp`] skeleton with
//! a zero flag word until W6 lands.  Noted in the structured losses.

use kuna_base::address::{calc_mask, sign_extend};
use kuna_base::types::{int4, intb, uintb};
use kuna_num::opcodes::OpCode;

use crate::action::{ActionGroupList, Rule, RuleSpec};
use crate::funcdata::Funcdata;
use crate::context::{OpId, TypeOp, VarnodeId};

/// (kuna GH-7190) Collapse the OV-flag signed-less-than idiom to `INT_SLESS(V,-K)`
/// (C++ `RuleOvLessSimplify`).
pub struct RuleOvLessSimplify {
    /// Resolved `glb->ov_less_simplify` gate (STUB(W4); see module docs).
    enabled: bool,
    /// Rule group (C++ `Rule::basegroup`).
    group: String,
}

impl RuleOvLessSimplify {
    /// Construct with the resolved gate.  Default group `"analysis"` (the main
    /// simplification pool, where the boolean cone is stable).
    pub fn new(enabled: bool) -> RuleOvLessSimplify {
        RuleOvLessSimplify { enabled, group: String::from("analysis") }
    }

    /// Construct with an explicit group string.
    pub fn with_group(enabled: bool, group: impl Into<String>) -> RuleOvLessSimplify {
        RuleOvLessSimplify { enabled, group: group.into() }
    }
}

impl Rule for RuleOvLessSimplify {
    /// C++ `RuleOvLessSimplify::getOpList`: `oplist.push_back(CPUI_INT_NOTEQUAL);`
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_INT_NOTEQUAL]
    }

    /// C++ `RuleOvLessSimplify::clone(grouplist)`.
    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(&self.group) {
            return None;
        }
        Some(Box::new(RuleOvLessSimplify { enabled: self.enabled, group: self.group.clone() }))
    }

    /// C++ `RuleOvLessSimplify::applyOp` (`kuna_ovlesssimplify.cc:48`) —
    /// transcribed.
    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // (kuna) gate (ArchContext-carried).
        if !self.enabled && !data.get_arch().ov_less_simplify {
            return 0;
        }

        // Match  NE( SLESS(V+K,0) , BOOL_AND( signTest , SLESS(-1, V+K) ) )
        let in0 = match op_in(data, op, 0) {
            Some(v) => v,
            None => return 0,
        };
        let in1 = match op_in(data, op, 1) {
            Some(v) => v,
            None => return 0,
        };

        // in0 = INT_SLESS(R1, 0)
        if !vn_is_written(data, in0) {
            return 0;
        }
        let sless0 = vn_def(data, in0).expect("in0 written");
        if op_code(data, sless0) != OpCode::CPUI_INT_SLESS {
            return 0;
        }
        let r1 = match op_in(data, sless0, 0) {
            Some(v) => v,
            None => return 0,
        };
        let zero = match op_in(data, sless0, 1) {
            Some(v) => v,
            None => return 0,
        };
        if !vn_is_constant(data, zero) || vn_offset(data, zero) != 0 {
            return 0;
        }

        // in1 = BOOL_AND(signTest, SLESS(-1, R2))
        if !vn_is_written(data, in1) {
            return 0;
        }
        let andop = vn_def(data, in1).expect("in1 written");
        if op_code(data, andop) != OpCode::CPUI_BOOL_AND {
            return 0;
        }

        // Locate the SLESS(-1, R2) limb of the AND; the other limb is the sign
        // test of V.
        let mut sless_hi: Option<OpId> = None;
        for i in 0..2 {
            let limb = match op_in(data, andop, i) {
                Some(v) => v,
                None => continue,
            };
            if !vn_is_written(data, limb) {
                continue;
            }
            let cand = vn_def(data, limb).expect("limb written");
            if op_code(data, cand) != OpCode::CPUI_INT_SLESS {
                continue;
            }
            let mone = match op_in(data, cand, 0) {
                Some(v) => v,
                None => continue,
            };
            if !vn_is_constant(data, mone) {
                continue;
            }
            // mone must be -1.
            if vn_offset(data, mone) != calc_mask(vn_size(data, mone)) {
                continue;
            }
            sless_hi = Some(cand);
            break;
        }
        let sless_hi = match sless_hi {
            Some(c) => c,
            None => return 0,
        };
        let r2 = match op_in(data, sless_hi, 1) {
            Some(v) => v,
            None => return 0,
        };

        // R1 and R2 must both be `V + K` with the same V and same K.
        let (base1, k1) = match match_add_const(data, r1) {
            Some(t) => t,
            None => return 0,
        };
        let (base2, k2) = match match_add_const(data, r2) {
            Some(t) => t,
            None => return 0,
        };
        if base1 != base2 {
            return 0;
        }
        if k1 != k2 {
            return 0;
        }
        if vn_size(data, r1) != vn_size(data, r2) {
            return 0;
        }

        // Collapse:  V s< -K   (negate K within V's size)
        let sz: int4 = vn_size(data, base1);
        // The ADD width must match V's width.
        if vn_size(data, r1) != sz {
            return 0;
        }
        let ksigned: intb = sign_extend(k1 as intb, sz * 8 - 1);
        let negk: uintb = (ksigned.wrapping_neg() as uintb) & calc_mask(sz);

        // STUB(W6): glb->inst[CPUI_INT_SLESS] property flags.
        data.op_set_opcode(op, TypeOp::new(OpCode::CPUI_INT_SLESS, 0, "INT_SLESS"));
        data.op_set_input(op, base1, 0)
            .expect("RuleOvLessSimplify: opSetInput base1 (internal invariant)");
        let negk_vn = data.new_constant(sz, negk);
        data.op_set_input(op, negk_vn, 1)
            .expect("RuleOvLessSimplify: opSetInput const (internal invariant)");
        1
    }
}

/// Per-file registration rows in C++ definition order (one rule).
pub fn specs() -> Vec<RuleSpec> {
    vec![RuleSpec {
        group: "analysis",
        // Shipped default: `option ovlesssimplify on` (architecture.cc resets
        // ov_less_simplify = true; kuna DIV-2 default-on, GH-7190).
        ctor: || Box::new(RuleOvLessSimplify::with_group(true, "analysis")),
    }]
}

/// Return `(base, koff)` if `vn` is `V + K` (K constant)
/// (C++ static `matchAddConst`, `kuna_ovlesssimplify.cc:18`).
fn match_add_const(data: &Funcdata, vn: VarnodeId) -> Option<(VarnodeId, uintb)> {
    if !vn_is_written(data, vn) {
        return None;
    }
    let addop = vn_def(data, vn)?;
    if op_code(data, addop) != OpCode::CPUI_INT_ADD {
        return None;
    }
    let a = op_in(data, addop, 0)?;
    let b = op_in(data, addop, 1)?;
    if vn_is_constant(data, b) {
        Some((a, vn_offset(data, b)))
    }
    else if vn_is_constant(data, a) {
        Some((b, vn_offset(data, a)))
    } else {
        None
    }
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

#[cfg(test)]
#[path = "kuna_ovlesssimplify/tests.rs"]
mod tests;
