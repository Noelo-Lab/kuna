//! Port of `decompiler/cpp/kuna_stackprobeloop.{cc,hh}` — resolve a gcc
//! stack-probe loop's stack-pointer `MULTIEQUAL` to a constant offset
//! (GH-8017/6858, an S2 "stack-pointer normalization" sub-stage).
//!
//! # The shape (kuna_stackprobeloop.hh)
//!
//! gcc's stack-clash / `-fstack-check` prologue probes a large frame one page at
//! a time with a loop:
//!
//! ```text
//!     lea  limit,[sp - frame]      ; limit = SP_in + limit_const
//!   L: sub  sp, 0x1000             ; probe step = one page
//!     or   [sp], 0                 ; touch the page
//!     cmp  sp, limit
//!     jne  L
//!     sub  sp, tail
//! ```
//!
//! After heritage the loop-carried stack pointer is a spacebase `MULTIEQUAL` PHI
//! whose back-edge is `PHI - page`.  The spacebase tracker cannot break that
//! self-referential cycle (`PHI = PHI - page` has no constant solution), so the
//! stack pointer never resolves past the loop, every post-loop local renders as
//! `&pxVar3[-0x1000]` noise (GH-8017), and call-argument stores at unmatched
//! offsets are dropped so calls render with no arguments (GH-6858).
//!
//! The loop's exit comparison pins the value: it iterates until
//! `PHI - page == limit`, so the post-loop value is exactly
//! `limit - page == SP_in + (limit_const - page)`, a fixed offset from the
//! spacebase input.  When `option stackprobeloop on`, [`RuleStackProbeLoop`]
//! recognizes the exact shape and rewrites the `MULTIEQUAL` into the concrete
//! `INT_ADD(SP_in, limit_const - page)`.
//!
//! # Faithfulness
//!
//! `uintb`→`u64`, `int4`→`i32`.  The probe step is read as an unsigned constant
//! offset and the final offset is masked with [`calc_mask`] exactly as the C++
//! `(limitoff - page) & calc_mask(sz)` (wrapping subtraction in `u64`).  The
//! `data.getArch()->model_stack_probe_loop` gate is read **live** from the
//! [`Architecture`](crate::architecture::Architecture) (kuna ships it default-on,
//! `architecture.rs:423`; `option stackprobeloop off` flips it to byte-identical
//! upstream output).  `findSpacebaseInput` is inlined faithfully
//! (`vbank.findInput(point.size, Address(point.space, point.offset))` from the
//! stack space's base register, `funcdata.cc:289`) — the same surface the alias
//! checker reaches through `Funcdata::findSpacebaseInput`.
//!
//! `op_set_opcode` takes a fully-formed `TypeOp`; the `glb->inst[opc]` property
//! flags are the W6 table (filled by the architecture's `inst` lookup).  The
//! opcode *value* is the load-bearing part for the rewrite and the action
//! engine's dispatch (the `kuna_addcarrychain` / `kuna_arraystride` idiom).
//! // SEAM(W6): `glb->inst[CPUI_INT_ADD]` property flags.
//!
//! ## Gate wiring — SEAM(W4)
//!
//! The C++ `applyOp` reads `data.getArch()->model_stack_probe_loop` live.  The
//! seam [`Architecture`](crate::seams::Architecture) on `Funcdata` does **not**
//! carry the flag yet (it holds only the `AddrSpaceManager`), so — exactly as
//! `kuna_memsetsequence`'s `RuleMemsetCopy` and `kuna_loweredswitch`'s Detect
//! Action do — the resolved gate is carried in [`RuleStackProbeLoop::enabled`],
//! set at construction.  kuna ships it default-on (`architecture.rs:423`
//! `model_stack_probe_loop = true`; DIV-3 default-on, GH-8017); `apply_op`
//! declines immediately when the gate is false, which is byte-identical to
//! `option stackprobeloop off` and to upstream.  The option parse +
//! confirmation message is [`parse_stack_probe_loop_form`]
//! (`OptionStackProbeLoop::apply`); writing the flag into the live
//! `Architecture` is the W4 assembler's job.

use kuna_base::address::{calc_mask, Address};
use kuna_base::error::KunaResult;
use kuna_base::types::{int4, uintb};
use kuna_num::opcodes::OpCode;

use kuna_base::marshal::ElementId;

use crate::action::{ActionGroupList, Rule, RuleSpec};
use crate::funcdata::Funcdata;
use crate::seams::{OpId, TypeOp, VarnodeId};

/// Marshaling element `<stackprobeloop>` (kuna).  ElementIds live in the 4000+
/// range (C++ `ELEM_STACKPROBELOOP = ElementId("stackprobeloop",4012)`).
pub const ELEM_STACKPROBELOOP: ElementId = ElementId::new("stackprobeloop", 4012);

/// (kuna GH-8017/6858) Resolve a gcc stack-probe loop's stack-pointer
/// `MULTIEQUAL` to a constant offset (C++ `class RuleStackProbeLoop`).
///
/// Matches a 2-input `CPUI_MULTIEQUAL` producing the stack-pointer register
/// whose back-edge input is `PHI - page` (the probe decrement), whose other
/// input is stack-relative, and whose loop exit compares `PHI - page` against a
/// stack-relative limit.  Rewrites the `MULTIEQUAL` into
/// `INT_ADD(spacebase_input, limit_const - page)`.  Only fires when
/// `option stackprobeloop on`; the apply body returns 0 immediately when the
/// arch flag is off, preserving byte-identical upstream output by default.
pub struct RuleStackProbeLoop {
    /// Resolved `glb->model_stack_probe_loop` gate (SEAM(W4); see module docs).
    enabled: bool,
    /// Rule group (C++ `Rule::basegroup`).
    group: String,
}

impl RuleStackProbeLoop {
    /// Construct in group `g` with the resolved gate (C++
    /// `RuleStackProbeLoop(const string &g)`: `Rule(g, 0, "stackprobeloop")`).
    pub fn new(enabled: bool, g: impl Into<String>) -> RuleStackProbeLoop {
        RuleStackProbeLoop { enabled, group: g.into() }
    }
}

impl Rule for RuleStackProbeLoop {
    /// C++ `RuleStackProbeLoop::getOpList`: `oplist.push_back(CPUI_MULTIEQUAL);`
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_MULTIEQUAL]
    }

    /// C++ `RuleStackProbeLoop::clone`:
    /// `if (!grouplist.contains(getGroup())) return 0;`
    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(&self.group) {
            return None;
        }
        Some(Box::new(RuleStackProbeLoop { enabled: self.enabled, group: self.group.clone() }))
    }

    /// C++ `RuleStackProbeLoop::applyOp` (`kuna_stackprobeloop.cc:63`).
    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // if (!data.getArch()->model_stack_probe_loop) return 0;  // default-off: byte-identical
        // SEAM(W4): the seam Architecture on Funcdata has no flag; the gate is the
        // resolved `enabled` field (see module docs).
        if !self.enabled {
            return 0;
        }

        // if (op->numInput() != 2) return 0;  // one entry edge, one back edge
        if op_num_input(data, op) != 2 {
            return 0;
        }

        // The MULTIEQUAL must produce the stack-pointer register.
        // AddrSpace *stackspc = data.getArch()->getStackSpace();
        let stackspc = match data.get_arch().manage().get_stack_space() {
            Some(s) => s.clone(),
            None => return 0,
        };
        // Varnode *spIn = data.findSpacebaseInput(stackspc);
        let sp_in = match find_spacebase_input(data, &stackspc) {
            Some(v) => v,
            None => return 0,
        };
        // Varnode *phiout = op->getOut();
        let phiout = match op_out(data, op) {
            Some(v) => v,
            None => return 0,
        };
        // if (phiout->getAddr() != spIn->getAddr()) return 0;
        if vn_addr(data, phiout) != vn_addr(data, sp_in) {
            return 0;
        }
        // if (phiout->getSize() != spIn->getSize()) return 0;
        if vn_size(data, phiout) != vn_size(data, sp_in) {
            return 0;
        }

        // One input is the back-edge `PHI - page`, the other is the stack-relative
        // entry value.  Identify the probe step: an INT_ADD(phiout, page) feeding
        // (through COPYs) a MULTIEQUAL input.
        let mut page: uintb = 0;
        let mut foundback = false;
        let mut foundentry = false;
        // for(int4 i=0;i<2;++i) {
        for i in 0..2 {
            // Varnode *invn = skipCopies(op->getIn(i));
            let invn = skip_copies(data, op_in(data, op, i).expect("MULTIEQUAL input present"));
            // if (invn->isWritten()) {
            if vn_is_written(data, invn) {
                let def = vn_def(data, invn).expect("written vn has a def");
                // if (def->code() == CPUI_INT_ADD) {
                if op_code(data, def) == OpCode::CPUI_INT_ADD {
                    // Varnode *b = skipCopies(def->getIn(0));  Varnode *c = def->getIn(1);
                    let b = skip_copies(data, op_in(data, def, 0).expect("INT_ADD in0"));
                    let c = op_in(data, def, 1).expect("INT_ADD in1");
                    // if (b == phiout && c->isConstant()) { page = c->getOffset(); foundback = true; continue; }
                    if b == phiout && vn_is_constant(data, c) {
                        page = vn_offset(data, c);
                        foundback = true;
                        continue;
                    }
                }
            }
            // uintb entryoff;
            // if (stackRelative(spIn,op->getIn(i),entryoff)) foundentry = true;
            let in_i = op_in(data, op, i).expect("MULTIEQUAL input present");
            if stack_relative(data, sp_in, in_i).is_some() {
                foundentry = true;
            }
        }
        // if (!foundback || !foundentry) return 0;
        if !foundback || !foundentry {
            return 0;
        }
        // if (page == 0) return 0;
        if page == 0 {
            return 0;
        }

        // Find the loop-exit comparison: an INT_NOTEQUAL/INT_EQUAL whose one operand
        // is `PHI + page` (the probe step) and whose other operand is a stack-relative
        // limit.  Search the descendants of phiout for the probe-step INT_ADD(s) and
        // their compares.
        let mut limitoff: uintb = 0;
        let mut foundlimit = false;
        // for(iter=phiout->beginDescend();iter!=phiout->endDescend();++iter) {
        for addop in vn_descend(data, phiout) {
            // if (addop->code() != CPUI_INT_ADD) continue;
            if op_code(data, addop) != OpCode::CPUI_INT_ADD {
                continue;
            }
            // if (!addop->getIn(1)->isConstant()) continue;
            let add_in1 = match op_in(data, addop, 1) {
                Some(v) => v,
                None => continue,
            };
            if !vn_is_constant(data, add_in1) {
                continue;
            }
            // if (addop->getIn(1)->getOffset() != page) continue;  // Same page step
            if vn_offset(data, add_in1) != page {
                continue;
            }
            // Varnode *stepvn = addop->getOut();
            let stepvn = match op_out(data, addop) {
                Some(v) => v,
                None => continue,
            };
            // for(citer=stepvn->beginDescend();citer!=stepvn->endDescend();++citer) {
            for cmpop in vn_descend(data, stepvn) {
                // if (cmpop->code() != CPUI_INT_NOTEQUAL && cmpop->code() != CPUI_INT_EQUAL) continue;
                let cc = op_code(data, cmpop);
                if cc != OpCode::CPUI_INT_NOTEQUAL && cc != OpCode::CPUI_INT_EQUAL {
                    continue;
                }
                // Varnode *other = (cmpop->getIn(0) == stepvn) ? cmpop->getIn(1) : cmpop->getIn(0);
                let cmp_in0 = op_in(data, cmpop, 0).expect("compare in0");
                let other = if cmp_in0 == stepvn {
                    op_in(data, cmpop, 1).expect("compare in1")
                } else {
                    cmp_in0
                };
                // uintb loff;  if (stackRelative(spIn,other,loff)) { limitoff = loff; foundlimit = true; break; }
                if let Some(loff) = stack_relative(data, sp_in, other) {
                    limitoff = loff;
                    foundlimit = true;
                    break;
                }
            }
            // if (foundlimit) break;
            if foundlimit {
                break;
            }
        }
        // if (!foundlimit) return 0;
        if !foundlimit {
            return 0;
        }

        // At loop exit `PHI + page == limit`, so the post-loop PHI value is
        // `limit - page` relative to the spacebase input.  Rewrite the MULTIEQUAL
        // into a concrete add.
        // int4 sz = spIn->getSize();
        let sz = vn_size(data, sp_in);
        // uintb finaloff = (limitoff - page) & calc_mask(sz);
        let finaloff = limitoff.wrapping_sub(page) & calc_mask(sz);
        // vector<Varnode *> inlist;  inlist.push_back(spIn);  inlist.push_back(data.newConstant(sz,finaloff));
        let cvn = data.new_constant(sz, finaloff);
        let inlist = [sp_in, cvn];
        // data.opSetOpcode(op,CPUI_INT_ADD);
        // SEAM(W6): glb->inst[CPUI_INT_ADD] property flags; opcode value is exact.
        data.op_set_opcode(op, TypeOp::new(OpCode::CPUI_INT_ADD, 0, "INT_ADD"));
        // data.opSetAllInput(op,inlist);
        data.op_set_all_input(op, &inlist).expect("opSetAllInput on rewritten MULTIEQUAL");
        // return 1;
        1
    }
}

/// (kuna GH-8017) How stack-probe-loop SP resolution is toggled (the two values
/// of `option stackprobeloop`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StackProbeLoopForm {
    /// `on`: resolve the probe loop's SP `MULTIEQUAL` to a constant offset
    /// (`glb->model_stack_probe_loop = true`).
    On,
    /// `off`: leave the loop-carried `MULTIEQUAL` (`= false`, byte-identical
    /// upstream).
    Off,
}

impl StackProbeLoopForm {
    /// The resolved `glb->model_stack_probe_loop` flag for this form.
    pub fn model_stack_probe_loop(self) -> bool {
        matches!(self, StackProbeLoopForm::On)
    }
}

/// Parse the `option stackprobeloop on|off` argument and produce the resolved
/// form plus the confirmation message (C++ `OptionStackProbeLoop::apply`).
///
/// ```text
///   bool val = onOrOff(p1);
///   glb->model_stack_probe_loop = val;
///   string prop = val ? "on" : "off";
///   return "Stack-probe-loop stack-pointer resolution turned "+prop;
/// ```
///
/// The caller writes [`StackProbeLoopForm::model_stack_probe_loop`] into
/// `Architecture::model_stack_probe_loop` (the W4 assembler's job).
pub fn parse_stack_probe_loop_form(p1: &str) -> KunaResult<(StackProbeLoopForm, String)> {
    // bool val = onOrOff(p1);
    let val = crate::options::on_or_off(p1)?;
    // glb->model_stack_probe_loop = val;  -- left to the caller.
    let form = if val { StackProbeLoopForm::On } else { StackProbeLoopForm::Off };
    // string prop = val ? "on" : "off"; return "Stack-probe-loop ... turned "+prop;
    let prop = if val { "on" } else { "off" };
    Ok((form, format!("Stack-probe-loop stack-pointer resolution turned {prop}")))
}

/// Per-file registration row (one rule), in C++ definition order.
///
/// Shipped default: `option stackprobeloop on` (`model_stack_probe_loop = true`;
/// kuna DIV-3 default-on, GH-8017).  The gate is resolved into the rule at
/// construction (SEAM(W4); see module docs), so the [`RuleSpec`] ctor builds it
/// `enabled`; the group placeholder is the per-file `"analysis"` (the
/// `universalAction` schedule re-registers it under its slot group).
pub fn specs() -> Vec<RuleSpec> {
    vec![RuleSpec {
        group: "analysis",
        ctor: || Box::new(RuleStackProbeLoop::new(true, "analysis")),
    }]
}

// ---------------------------------------------------------------------------
// findSpacebaseInput (C++ Funcdata::findSpacebaseInput, funcdata.cc:289),
// inlined faithfully: the input Varnode for the space's base register.
// ---------------------------------------------------------------------------

/// C++ `Funcdata::findSpacebaseInput(id)`:
/// `vbank.findInput(point.size, Address(point.space, point.offset))` over the
/// base register `id->getSpacebase(0)`.
fn find_spacebase_input(
    data: &Funcdata,
    id: &std::rc::Rc<kuna_base::space::AddrSpace>,
) -> Option<VarnodeId> {
    // const VarnodeData &point(id->getSpacebase(0));
    let point = id.get_spacebase(0).ok()?;
    let space = point.space?;
    // vn = vbank.findInput(point.size, Address(point.space,point.offset));
    let addr = Address::new(space, point.offset);
    data.vbank().find_input(point.size as int4, &addr)
}

// ---------------------------------------------------------------------------
// skipCopies / stackRelative (kuna_stackprobeloop.cc static helpers).
// ---------------------------------------------------------------------------

/// C++ static `skipCopies` — trace a Varnode back through `CPUI_COPY` ops to its
/// underlying source.  Bounded at 8 to defeat a pathological COPY chain.
fn skip_copies(data: &Funcdata, mut vn: VarnodeId) -> VarnodeId {
    // for(int4 i=0;i<8;++i) {
    for _i in 0..8 {
        // if (!vn->isWritten()) break;
        if !vn_is_written(data, vn) {
            break;
        }
        // PcodeOp *op = vn->getDef();  if (op->code() != CPUI_COPY) break;
        let op = vn_def(data, vn).expect("written vn has a def");
        if op_code(data, op) != OpCode::CPUI_COPY {
            break;
        }
        // vn = op->getIn(0);
        match op_in(data, op, 0) {
            Some(v) => vn = v,
            None => break,
        }
    }
    vn
}

/// C++ static `stackRelative` — is `vn` the spacebase input itself, or
/// `spIn + constant`?  Traces through COPYs; on success returns the constant
/// offset (`Some(0)` when `vn` is the input directly).
fn stack_relative(data: &Funcdata, sp_in: VarnodeId, vn: VarnodeId) -> Option<uintb> {
    // vn = skipCopies(vn);
    let vn = skip_copies(data, vn);
    // if (vn == spIn) { off = 0; return true; }
    if vn == sp_in {
        return Some(0);
    }
    // if (!vn->isWritten()) return false;
    if !vn_is_written(data, vn) {
        return None;
    }
    // PcodeOp *op = vn->getDef();  if (op->code() != CPUI_INT_ADD) return false;
    let op = vn_def(data, vn).expect("written vn has a def");
    if op_code(data, op) != OpCode::CPUI_INT_ADD {
        return None;
    }
    // Varnode *base = skipCopies(op->getIn(0));  Varnode *cvn = op->getIn(1);
    let mut base = skip_copies(data, op_in(data, op, 0)?);
    let mut cvn = op_in(data, op, 1)?;
    // if (!cvn->isConstant()) { cvn = op->getIn(0); base = skipCopies(op->getIn(1)); }
    if !vn_is_constant(data, cvn) {
        cvn = op_in(data, op, 0)?;
        base = skip_copies(data, op_in(data, op, 1)?);
    }
    // if (!cvn->isConstant()) return false;
    if !vn_is_constant(data, cvn) {
        return None;
    }
    // if (base != spIn) return false;
    if base != sp_in {
        return None;
    }
    // off = cvn->getOffset();  return true;
    Some(vn_offset(data, cvn))
}

// ---------------------------------------------------------------------------
// Local IR read accessors (the kuna_loweredswitch / kuna_arraystride idiom).
// ---------------------------------------------------------------------------

fn op_num_input(data: &Funcdata, op: OpId) -> int4 {
    data.obank().get(op).expect("op_num_input: stale op").num_input()
}
fn op_out(data: &Funcdata, op: OpId) -> Option<VarnodeId> {
    data.obank().get(op).and_then(|o| o.get_out())
}
fn op_in(data: &Funcdata, op: OpId, slot: int4) -> Option<VarnodeId> {
    data.obank().get(op).and_then(|o| o.get_in(slot))
}
fn op_code(data: &Funcdata, op: OpId) -> OpCode {
    data.obank().get(op).expect("op_code: stale op").code()
}
fn vn_descend(data: &Funcdata, vn: VarnodeId) -> Vec<OpId> {
    data.descend_snapshot(vn)
}
fn vn_is_written(data: &Funcdata, vn: VarnodeId) -> bool {
    data.vbank().get(vn).map(|v| v.is_written()).unwrap_or(false)
}
fn vn_is_constant(data: &Funcdata, vn: VarnodeId) -> bool {
    data.vbank().get(vn).map(|v| v.is_constant()).unwrap_or(false)
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
fn vn_addr(data: &Funcdata, vn: VarnodeId) -> Address {
    data.vbank().get(vn).expect("vn_addr: stale vn").get_addr().clone()
}

#[cfg(test)]
#[path = "kuna_stackprobeloop/tests.rs"]
mod tests;
