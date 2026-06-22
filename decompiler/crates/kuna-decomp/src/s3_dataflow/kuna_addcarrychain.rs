//! Port of `decompiler/cpp/kuna_addcarrychain.{cc,hh}` — carry-chain wide-add
//! recovery (kuna GH-8913, S5/S3 simplification-quiescence).
//!
//! `RuleAddCarryChain` matches the reassembling `CPUI_PIECE` of an 8-bit
//! carry-chained 16-bit add (the classic two-instruction ADC idiom) and rewrites
//! it to a single wide add:
//!
//! ```text
//!   out = PIECE(hi, lo)        =>     out = INT_ADD( PIECE(hipart, b), zext(a) )
//! ```
//!
//! where `lo = INT_ADD(a,b)`, `hi = INT_ADD(hipart, carry)` and `carry` is the
//! page-cross carry of `a,b` (raw `INT_CARRY` or the const-folded
//! `INT_LESSEQUAL((-b)&mask, a)` form).  Gated default-on by the arch flag
//! `add_carry_chain` (option `addcarrychain on|off`, shipped default `on`); inert
//! when off, byte-identical to upstream.
//!
//! ## Gate wiring — SEAM(W4)
//!
//! As in the sibling kuna simplification rules, the C++
//! `if (!data.getArch()->add_carry_chain) return 0;` gate is resolved at
//! construction (the seam `Funcdata::glb` does not carry kuna analysis flags).
//! W8 threads `Architecture::add_carry_chain`; [`specs`] uses the shipped default
//! (`on`).
//!
//! ## Output-Varnode creation — `newUniqueOut`
//!
//! The C++ builds two helper ops with fresh unique outputs via
//! `data.newUniqueOut(size, op)`.  The W3 [`Funcdata::op_set_output`] is a seam
//! (it needs a `banks_mut()` split-borrow the funcdata owner has not yet exposed),
//! so this rule reproduces `newUniqueOut` directly through the public
//! [`VarnodeBank::create_def_unique`](crate::varnode::VarnodeBank::create_def_unique)
//! (C++ `createDefUnique`) + [`PcodeOp::set_output`] — exactly the path the
//! merged `funcdata_varnode` tests use for a fresh unique output.  The
//! `replace_reads` callback panics if invoked: a freshly-created unique has no
//! aliasing equivalent, so `xref` never unifies it (an invocation would be an
//! internal-invariant violation).  Noted in the structured losses as the
//! `op_set_output` seam workaround.
//!
//! ## SEAM(W6) — opcode-flag resolution
//!
//! `opSetOpcode(...)` resolves `glb->inst[opc]` (the W6 typeop `inst` table); the
//! op-shells are built with the [`TypeOp`] skeleton (zero flag word) until W6
//! lands.  Noted in the structured losses.

use std::rc::Rc;

use kuna_base::address::{calc_mask, Address};
use kuna_base::error::KunaResult;
use kuna_base::types::{int4, uintb};
use kuna_num::opcodes::OpCode;

use crate::action::{ActionGroupList, Rule, RuleSpec};
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;
use crate::seams::{OpId, VarnodeId};
use crate::varnode::{DefOpInfo, VarnodeBank};

/// (kuna GH-8913) Fuse an 8-bit carry-chained 16-bit add reassembled by CONCAT
/// (C++ `RuleAddCarryChain`).
pub struct RuleAddCarryChain {
    /// Resolved `glb->add_carry_chain` gate (SEAM(W4); see module docs).
    enabled: bool,
    /// Rule group (C++ `Rule::basegroup`).
    group: String,
}

impl RuleAddCarryChain {
    /// Construct with the resolved gate.  Default group `"analysis"`.
    pub fn new(enabled: bool) -> RuleAddCarryChain {
        RuleAddCarryChain { enabled, group: String::from("analysis") }
    }
    /// Construct with an explicit group.
    pub fn with_group(enabled: bool, group: impl Into<String>) -> RuleAddCarryChain {
        RuleAddCarryChain { enabled, group: group.into() }
    }
}

impl Rule for RuleAddCarryChain {
    /// C++ `RuleAddCarryChain::getOpList`: `oplist.push_back(CPUI_PIECE);`
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_PIECE]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(&self.group) {
            return None;
        }
        Some(Box::new(RuleAddCarryChain { enabled: self.enabled, group: self.group.clone() }))
    }

    /// C++ `RuleAddCarryChain::applyOp` (`kuna_addcarrychain.cc:77`) — transcribed.
    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // if (!data.getArch()->add_carry_chain) return 0;  // (kuna) gate (seam-carried)
        if !self.enabled && !data.get_arch().add_carry_chain {
            return 0;
        }

        // Varnode *hivn = op->getIn(0);  Varnode *lovn = op->getIn(1);
        let hivn = match op_in(data, op, 0) {
            Some(v) => v,
            None => return 0,
        };
        let lovn = match op_in(data, op, 1) {
            Some(v) => v,
            None => return 0,
        };
        // if (!hivn->isWritten() || !lovn->isWritten()) return 0;
        if !vn_is_written(data, hivn) || !vn_is_written(data, lovn) {
            return 0;
        }
        // if (hivn->getSize() != 1 || lovn->getSize() != 1) return 0;
        if vn_size(data, hivn) != 1 || vn_size(data, lovn) != 1 {
            return 0;
        }

        // PcodeOp *loadd = lovn->getDef();  if (loadd->code() != CPUI_INT_ADD) return 0;
        let loadd = vn_def(data, lovn).expect("lovn written");
        if op_code(data, loadd) != OpCode::CPUI_INT_ADD {
            return 0;
        }
        let a = match op_in(data, loadd, 0) {
            Some(v) => v,
            None => return 0,
        };
        let b = match op_in(data, loadd, 1) {
            Some(v) => v,
            None => return 0,
        };

        // PcodeOp *hiadd = hivn->getDef();  if (hiadd->code() != CPUI_INT_ADD) return 0;
        let hiadd = vn_def(data, hivn).expect("hivn written");
        if op_code(data, hiadd) != OpCode::CPUI_INT_ADD {
            return 0;
        }

        // One input of the high add is the carry; the other is the base high byte.
        let hi0 = match op_in(data, hiadd, 0) {
            Some(v) => v,
            None => return 0,
        };
        let hi1 = match op_in(data, hiadd, 1) {
            Some(v) => v,
            None => return 0,
        };
        // if (kunaIsCarryOf(hi1,a,b)) { carryvn = hi1; hipart = hi0; }
        // else if (kunaIsCarryOf(hi0,a,b)) { carryvn = hi0; hipart = hi1; }
        // else return 0;
        let hipart = if kuna_is_carry_of(data, hi1, a, b) {
            hi0
        } else if kuna_is_carry_of(data, hi0, a, b) {
            hi1
        } else {
            return 0;
        };

        // if (hipart->getSize() != 1) return 0;
        if vn_size(data, hipart) != 1 {
            return 0;
        }

        // Pick the index operand (a, zext'd) and the base-low operand (b).
        // Varnode *indexvn = a;  Varnode *baselo = b;
        let indexvn = a;
        let baselo = b;

        // int4 outsize = op->getOut()->getSize();
        let out = match data.obank().get(op).and_then(|o| o.get_out()) {
            Some(o) => o,
            None => return 0,
        };
        let outsize: int4 = vn_size(data, out);

        // The address used for the new helper ops (C++ op->getAddr()).
        let pc: Address = data.obank().get(op).expect("op live").get_addr().clone();

        // Build  PIECE(hipart, baselo)  -- the 16-bit base.
        // PcodeOp *baseop = data.newOp(2,op->getAddr());
        let baseop = data.new_op(2, pc.clone());
        // data.newUniqueOut(hipart->getSize() + baselo->getSize(),baseop);
        let base_outsize = vn_size(data, hipart) + vn_size(data, baselo);
        let _base_out = new_unique_out(data, base_outsize, baseop)
            .expect("RuleAddCarryChain: newUniqueOut(base) (internal invariant)");
        // data.opSetOpcode(baseop,CPUI_PIECE);  -- resolve glb->inst[CPUI_PIECE]
        // through the W6 inst[] table so the op carries the real `binary` eval-type
        // flag.  A zero-flag skeleton leaves PIECE(const,const) uncollapsible
        // (`dc_collapse` reads the eval-type bit), so the reassembled 16-bit base
        // would render as a literal `CONCAT11(0xNN,0xNN)` instead of folding to the
        // base constant (GH-8913 #2).
        data.op_set_opcode_code(baseop, OpCode::CPUI_PIECE);
        // data.opSetInput(baseop,hipart,0);  data.opSetInput(baseop,baselo,1);
        data.op_set_input(baseop, hipart, 0)
            .expect("RuleAddCarryChain: opSetInput baseop.0");
        data.op_set_input(baseop, baselo, 1)
            .expect("RuleAddCarryChain: opSetInput baseop.1");
        // data.opInsertBefore(baseop,op);
        data.op_insert_before(baseop, op);

        // Build  zext(index)  to the output width.
        // PcodeOp *zextop = data.newOp(1,op->getAddr());
        let zextop = data.new_op(1, pc);
        // data.newUniqueOut(outsize,zextop);
        let _zext_out = new_unique_out(data, outsize, zextop)
            .expect("RuleAddCarryChain: newUniqueOut(zext) (internal invariant)");
        // data.opSetOpcode(zextop,CPUI_INT_ZEXT);  -- resolve glb->inst[] (W6)
        data.op_set_opcode_code(zextop, OpCode::CPUI_INT_ZEXT);
        // data.opSetInput(zextop,indexvn,0);
        data.op_set_input(zextop, indexvn, 0)
            .expect("RuleAddCarryChain: opSetInput zextop.0");
        // data.opInsertBefore(zextop,op);
        data.op_insert_before(zextop, op);

        // Rewrite the PIECE op itself into the wide add:  out = base + zext(index).
        // data.opSetOpcode(op,CPUI_INT_ADD);  -- resolve glb->inst[] (W6)
        data.op_set_opcode_code(op, OpCode::CPUI_INT_ADD);
        // data.opSetInput(op,baseop->getOut(),0);
        let base_out = data.obank().get(baseop).expect("baseop live").get_out().expect("baseop out");
        data.op_set_input(op, base_out, 0)
            .expect("RuleAddCarryChain: opSetInput op.0");
        // data.opSetInput(op,zextop->getOut(),1);
        let zext_out = data.obank().get(zextop).expect("zextop live").get_out().expect("zextop out");
        data.op_set_input(op, zext_out, 1)
            .expect("RuleAddCarryChain: opSetInput op.1");
        1
    }
}

/// Per-file registration rows in C++ definition order (one rule).
pub fn specs() -> Vec<RuleSpec> {
    vec![RuleSpec {
        group: "analysis",
        // Shipped default: `option addcarrychain on` (add_carry_chain = true;
        // kuna DIV-2 default-on, GH-8913).
        ctor: || Box::new(RuleAddCarryChain::with_group(true, "analysis")),
    }]
}

// ---------------------------------------------------------------------------
// Static helpers (faithful transcriptions of the kuna_addcarrychain.cc statics)
// ---------------------------------------------------------------------------

/// Strip any CAST/COPY wrappers, returning the underlying value
/// (C++ static `kunaStripCast`).
fn kuna_strip_cast(data: &Funcdata, mut vn: VarnodeId) -> VarnodeId {
    // while (vn->isWritten()) { def = vn->getDef(); oc = def->code();
    //   if (oc != CPUI_CAST && oc != CPUI_COPY) break; vn = def->getIn(0); }
    while vn_is_written(data, vn) {
        let def = vn_def(data, vn).expect("vn written");
        let oc = op_code(data, def);
        if oc != OpCode::CPUI_CAST && oc != OpCode::CPUI_COPY {
            break;
        }
        match op_in(data, def, 0) {
            Some(v) => vn = v,
            None => break,
        }
    }
    vn
}

/// Are `a` and `b` the same value once CAST/COPY wrappers are stripped?
/// (C++ static `kunaSameValue`).
fn kuna_same_value(data: &Funcdata, a: VarnodeId, b: VarnodeId) -> bool {
    let a = kuna_strip_cast(data, a);
    let b = kuna_strip_cast(data, b);
    if a == b {
        return true;
    }
    // if (a->isConstant() && b->isConstant())
    //   return (a->getOffset() == b->getOffset() && a->getSize() == b->getSize());
    if vn_is_constant(data, a) && vn_is_constant(data, b) {
        return vn_offset(data, a) == vn_offset(data, b) && vn_size(data, a) == vn_size(data, b);
    }
    false
}

/// Given the carry varnode and the low-add operands (a,b), confirm it is the
/// page-cross carry of a+b, in either INT_CARRY or RuleCarryElim form
/// (C++ static `kunaIsCarryOf`).
fn kuna_is_carry_of(data: &Funcdata, carryvn: VarnodeId, a: VarnodeId, b: VarnodeId) -> bool {
    // if (!carryvn->isWritten()) return false;
    if !vn_is_written(data, carryvn) {
        return false;
    }
    let cop = vn_def(data, carryvn).expect("carryvn written");
    let oc = op_code(data, cop);
    if oc == OpCode::CPUI_INT_CARRY {
        let c0 = match op_in(data, cop, 0) {
            Some(v) => v,
            None => return false,
        };
        let c1 = match op_in(data, cop, 1) {
            Some(v) => v,
            None => return false,
        };
        if kuna_same_value(data, c0, a) && kuna_same_value(data, c1, b) {
            return true;
        }
        if kuna_same_value(data, c0, b) && kuna_same_value(data, c1, a) {
            return true;
        }
        return false;
    }
    if oc == OpCode::CPUI_INT_LESSEQUAL {
        // RuleCarryElim form: carry(V,c) => (-c)&mask <= V, with c the constant
        // added in the low sum.
        let cst = match op_in(data, cop, 0) {
            Some(v) => v,
            None => return false,
        };
        let val = match op_in(data, cop, 1) {
            Some(v) => v,
            None => return false,
        };
        if !vn_is_constant(data, cst) {
            return false;
        }
        // The non-constant low-add operand must equal val; the constant operand is c.
        let (c_add, v_add) = if vn_is_constant(data, a) && !vn_is_constant(data, b) {
            (a, b)
        } else if vn_is_constant(data, b) && !vn_is_constant(data, a) {
            (b, a)
        } else {
            return false;
        };
        if !kuna_same_value(data, val, v_add) {
            return false;
        }
        // uintb negc = (-cAdd->getOffset()) & calc_mask(val->getSize());
        let negc: uintb = vn_offset(data, c_add).wrapping_neg() & calc_mask(vn_size(data, val));
        // return (cst->getOffset() == negc);
        return vn_offset(data, cst) == negc;
    }
    false
}

/// Reproduce `Funcdata::newUniqueOut(size, op)` for a fresh unique output:
/// create a unique varnode already defined by `op` and link it as the op's
/// output.  See module docs (the `op_set_output` seam workaround).
fn new_unique_out(data: &mut Funcdata, size: int4, op: OpId) -> KunaResult<VarnodeId> {
    let ct: Rc<Datatype> = Rc::new(Datatype::new(size, type_metatype::TYPE_UNKNOWN));
    let seqnum = data.obank().get(op).expect("new_unique_out: stale op").get_seq_num().clone();
    let info = DefOpInfo { id: op, seqnum };
    // A freshly-created unique has no aliasing equivalent; xref never unifies it.
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
#[path = "kuna_addcarrychain/tests.rs"]
mod tests;
