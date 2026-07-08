//! Port of `decompiler/cpp/kuna_arraystride.{cc,hh}` — strided-induction
//! (array-stride) recovery (kuna GH-8724, S3 simplification-quiescence).
//!
//! `RuleArrayStride` recognizes a loop-header stride accumulator phi
//! `acc = MULTIEQUAL(#0, acc + STRIDE)` (STRIDE a nonzero, non-one constant),
//! finds its sibling unit-step counter phi `cnt = MULTIEQUAL(#0, cnt + 1)` in the
//! SAME block (lining up edge-for-edge), and replaces every use of `acc` with
//! `INT_MULT(cnt, STRIDE)`, re-exposing the array index.  Gated default-on by the
//! arch flag `recover_array_stride` (option `arraystride on|off`, shipped default
//! `on`); inert when off, byte-identical to upstream.
//!
//! ## Gate wiring — SEAM(W4)
//!
//! As in the sibling kuna simplification rules, the C++
//! `if (!data.getArch()->recover_array_stride) return 0;` gate is resolved at
//! construction (the seam `Funcdata::glb` does not carry kuna analysis flags).
//! W8 threads `Architecture::recover_array_stride`; [`specs`] uses the shipped
//! default (`on`).
//!
//! ## Output-Varnode creation — `newUniqueOut`
//!
//! `data.newUniqueOut(size, op)` is reproduced through the public
//! [`VarnodeBank::create_def_unique`](crate::varnode::VarnodeBank::create_def_unique)
//! together with [`PcodeOp::set_output`] (the [`Funcdata::op_set_output`] seam
//! needs a `banks_mut()` split-borrow the funcdata owner has not exposed) — see
//! the `kuna_addcarrychain` module docs.  Noted in the structured losses.
//!
//! ## Block-op walk
//!
//! The C++ `for(iter=bl->beginOp(); iter!=bl->endOp(); ++iter)` sibling-phi scan
//! uses [`Funcdata::bb_ops`] (the head..tail intrusive-list order, where the
//! MULTIEQUALs are all first), preserving the `cand->code() != CPUI_MULTIEQUAL ->
//! break` early-out exactly.
//!
//! ## SEAM(W6) — opcode-flag resolution
//!
//! `opSetOpcode(multop, CPUI_INT_MULT)` resolves `glb->inst[CPUI_INT_MULT]` (the
//! W6 typeop `inst` table); the op-shell is built with the [`TypeOp`] skeleton
//! (zero flag word) until W6 lands.  Noted in the structured losses.

use std::rc::Rc;

use kuna_base::error::KunaResult;
use kuna_base::types::{int4, uintb};
use kuna_num::opcodes::OpCode;

use crate::action::{ActionGroupList, Rule, RuleSpec};
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;
use crate::seams::{BlockId, OpId, TypeOp, VarnodeId};
use crate::varnode::{DefOpInfo, VarnodeBank};

/// (kuna GH-8724) Re-expose a strided-induction index: rewrite a stride
/// accumulator phi as `counter * stride` (C++ `RuleArrayStride`).
pub struct RuleArrayStride {
    /// Resolved `glb->recover_array_stride` gate (SEAM(W4); see module docs).
    enabled: bool,
    /// Rule group (C++ `Rule::basegroup`).
    group: String,
}

impl RuleArrayStride {
    /// Construct with the resolved gate.  Default group `"analysis"`.
    pub fn new(enabled: bool) -> RuleArrayStride {
        RuleArrayStride { enabled, group: String::from("analysis") }
    }
    /// Construct with an explicit group.
    pub fn with_group(enabled: bool, group: impl Into<String>) -> RuleArrayStride {
        RuleArrayStride { enabled, group: group.into() }
    }
}

impl Rule for RuleArrayStride {
    /// C++ `RuleArrayStride::getOpList`: `oplist.push_back(CPUI_MULTIEQUAL);`
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_MULTIEQUAL]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(&self.group) {
            return None;
        }
        Some(Box::new(RuleArrayStride { enabled: self.enabled, group: self.group.clone() }))
    }

    /// C++ `RuleArrayStride::applyOp` (`kuna_arraystride.cc:88`) — transcribed.
    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // if (!data.getArch()->recover_array_stride) return 0;  // (kuna) gate (seam-carried)
        if !self.enabled && !data.get_arch().recover_array_stride {
            return 0;
        }

        // `op` must be the strided OFFSET accumulator phi: MULTIEQUAL(#0, acc + STRIDE).
        // if (!kunaIsInductionPhi(op,initIdx,backIdx,stride,accadd,true)) return 0;
        let (init_idx, back_idx, stride, accadd) = match kuna_is_induction_phi(data, op, true) {
            Some(t) => t,
            None => return 0,
        };
        // if (stride == 1) return 0;  // a unit step is the counter itself
        if stride == 1 {
            return 0;
        }

        // Varnode *accout = op->getOut();
        let accout = match data.obank().get(op).and_then(|o| o.get_out()) {
            Some(o) => o,
            None => return 0,
        };
        // BlockBasic *bl = op->getParent();  if (bl == 0) return 0;
        let bl: BlockId = match data.obank().get(op).and_then(|o| o.get_parent()) {
            Some(b) => b,
            None => return 0,
        };

        // Find a sibling unit-step counter phi `cnt = MULTIEQUAL(#0, cnt + 1)` in
        // the SAME block.  The MULTIEQUALs are all first in the block, so we walk
        // `beginOp()..endOp()` and `break` at the first non-MULTIEQUAL.
        let mut cntout: Option<VarnodeId> = None;
        for cand in data.bb_ops(bl) {
            // if (cand->code() != CPUI_MULTIEQUAL) break;
            if op_code(data, cand) != OpCode::CPUI_MULTIEQUAL {
                break;
            }
            // if (cand == op) continue;
            if cand == op {
                continue;
            }
            // if (!kunaIsInductionPhi(cand,ci,cb,cstep,cadd,false)) continue;
            let (ci, cb, cstep, _cadd) = match kuna_is_induction_phi(data, cand, false) {
                Some(t) => t,
                None => continue,
            };
            // if (cstep != 1) continue;  // counter must step by exactly 1
            if cstep != 1 {
                continue;
            }
            // if (ci != initIdx || cb != backIdx) continue;  // match edge-for-edge
            if ci != init_idx || cb != back_idx {
                continue;
            }
            // cntout = cand->getOut();  break;
            cntout = data.obank().get(cand).and_then(|o| o.get_out());
            break;
        }
        let cntout = match cntout {
            Some(c) => c,
            None => return 0,
        };
        // if (cntout->getSize() != accout->getSize()) return 0;
        if vn_size(data, cntout) != vn_size(data, accout) {
            return 0;
        }

        // Build the multiply  cnt * stride  and redirect all descendants of acc.
        // PcodeOp *multop = data.newOp(2,op->getAddr());
        let pc = data.obank().get(op).expect("op live").get_addr().clone();
        let multop = data.new_op(2, pc);
        // data.opSetOpcode(multop,CPUI_INT_MULT);  // SEAM(W6): glb->inst[CPUI_INT_MULT]
        data.op_set_opcode(multop, TypeOp::new(OpCode::CPUI_INT_MULT, 0, "INT_MULT"));
        // Varnode *multout = data.newUniqueOut(accout->getSize(),multop);
        let multout = new_unique_out(data, vn_size(data, accout), multop)
            .expect("RuleArrayStride: newUniqueOut(mult) (internal invariant)");
        // data.opSetInput(multop,cntout,0);
        data.op_set_input(multop, cntout, 0)
            .expect("RuleArrayStride: opSetInput multop.0");
        // data.opSetInput(multop,data.newConstant(accout->getSize(),stride),1);
        let stride_c = data.new_constant(vn_size(data, accout), stride);
        data.op_set_input(multop, stride_c, 1)
            .expect("RuleArrayStride: opSetInput multop.1");
        // data.opInsertBegin(multop,bl);
        data.op_insert_begin(multop, bl);

        // data.totalReplace(accout,multout);  // every use of acc now reads cnt*stride
        data.total_replace(accout, multout)
            .expect("RuleArrayStride: totalReplace (internal invariant)");
        // data.opDestroy(op);     // the dead accumulator phi ...
        data.op_destroy(op);
        // data.opDestroy(accadd); // ... and its now-dead back-edge add
        data.op_destroy(accadd);
        1
    }
}

/// Per-file registration rows in C++ definition order (one rule).
pub fn specs() -> Vec<RuleSpec> {
    vec![RuleSpec {
        group: "analysis",
        // Shipped default: `option arraystride on` (recover_array_stride = true;
        // kuna DIV-3 default-on, GH-8724).
        ctor: || Box::new(RuleArrayStride::with_group(true, "analysis")),
    }]
}

// ---------------------------------------------------------------------------
// Static helpers (faithful transcriptions of the kuna_arraystride.cc statics)
// ---------------------------------------------------------------------------

/// Does `vn` carry the constant value 0 (directly, or via a single COPY)?
/// (C++ static `kunaIsZeroInit`).
fn kuna_is_zero_init(data: &Funcdata, vn: VarnodeId) -> bool {
    // if (vn->isConstant()) return vn->getOffset() == 0;
    if vn_is_constant(data, vn) {
        return vn_offset(data, vn) == 0;
    }
    // if (vn->isWritten()) { def = vn->getDef(); if (def->code() == CPUI_COPY) {
    //   src = def->getIn(0); if (src->isConstant()) return src->getOffset() == 0; } }
    if vn_is_written(data, vn) {
        let def = vn_def(data, vn).expect("vn written");
        if op_code(data, def) == OpCode::CPUI_COPY {
            if let Some(src) = op_in(data, def, 0) {
                if vn_is_constant(data, src) {
                    return vn_offset(data, src) == 0;
                }
            }
        }
    }
    false
}

/// Is `phi` a 2-input loop-header phi `MULTIEQUAL(#0, self + C)`?
/// (C++ static `kunaIsInductionPhi`).  Returns `(initIdx, backIdx, step, addop)`.
///
/// `require_lone_add`: when true the back-edge add must feed ONLY this phi (so
/// it can be destroyed safely).
fn kuna_is_induction_phi(
    data: &Funcdata,
    phi: OpId,
    require_lone_add: bool,
) -> Option<(int4, int4, uintb, OpId)> {
    // if (phi->numInput() != 2) return false;
    if data.obank().get(phi).map(|o| o.num_input()).unwrap_or(0) != 2 {
        return None;
    }
    // Varnode *out = phi->getOut();
    let out = data.obank().get(phi).and_then(|o| o.get_out())?;
    let mut zero_idx: int4 = -1;
    let mut add_idx: int4 = -1;
    let mut foundadd: Option<OpId> = None;
    let mut foundstep: uintb = 0;
    for i in 0..2i32 {
        // Varnode *vn = phi->getIn(i);
        let vn = match op_in(data, phi, i) {
            Some(v) => v,
            None => continue,
        };
        if kuna_is_zero_init(data, vn) {
            // init edge must carry the constant 0
            zero_idx = i;
        } else if vn_is_written(data, vn) {
            let def = vn_def(data, vn).expect("vn written");
            // if (def->code() == CPUI_INT_ADD && (!requireLoneAdd || vn->loneDescend() == phi))
            if op_code(data, def) == OpCode::CPUI_INT_ADD
                && (!require_lone_add || data.lone_descend(vn) == Some(phi))
            {
                // One operand of the add must be the phi's own output (the
                // loop-back), the other a nonzero constant step.
                let a = op_in(data, def, 0)?;
                let b = op_in(data, def, 1)?;
                let mut self_vn: Option<VarnodeId> = None;
                let mut cst: Option<VarnodeId> = None;
                if a == out {
                    self_vn = Some(a);
                    cst = Some(b);
                } else if b == out {
                    self_vn = Some(b);
                    cst = Some(a);
                }
                if let (Some(_self), Some(cst)) = (self_vn, cst) {
                    if vn_is_constant(data, cst) && vn_offset(data, cst) != 0 {
                        add_idx = i;
                        foundadd = Some(def);
                        foundstep = vn_offset(data, cst);
                    }
                }
            }
        }
    }
    // if (zeroIdx < 0 || addIdx < 0 || zeroIdx == addIdx) return false;
    if zero_idx < 0 || add_idx < 0 || zero_idx == add_idx {
        return None;
    }
    Some((zero_idx, add_idx, foundstep, foundadd.expect("addIdx set => foundadd")))
}

/// Reproduce `Funcdata::newUniqueOut(size, op)` for a fresh unique output (see
/// `kuna_addcarrychain` module docs for the `op_set_output` seam rationale).
fn new_unique_out(data: &mut Funcdata, size: int4, op: OpId) -> KunaResult<VarnodeId> {
    let ct: Rc<Datatype> = Rc::new(Datatype::new(size, type_metatype::TYPE_UNKNOWN));
    let seqnum = data.obank().get(op).expect("new_unique_out: stale op").get_seq_num().clone();
    let info = DefOpInfo { id: op, seqnum };
    let mut never = |_: &mut VarnodeBank, _: VarnodeId, _: VarnodeId| -> KunaResult<()> {
        panic!("new_unique_out: xref unified a fresh unique output (internal invariant)")
    };
    let vn = data.vbank_mut().create_def_unique(size, ct, info, &mut never)?;
    data.obank_mut().get_mut(op).expect("new_unique_out: stale op").set_output(Some(vn));
    Ok(vn)
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
#[path = "kuna_arraystride/tests.rs"]
mod tests;
