//! Port of `decompiler/cpp/subflow.{cc,hh}` — reducing/splitting Varnodes that
//! carry smaller logical values (ADR 0005 rule wave, item `w5-s3-subflow`).
//!
//! # Scope
//!
//! This file ports the three engines that live in `subflow.{cc,hh}` and are in
//! scope for `w5-s3-subflow`:
//!
//!   * [`SubvariableFlow`] — the sub-variable replacement engine.  Given a root
//!     Varnode and a *mask* identifying a smaller logical value within it, it
//!     traces the logical value's flow forward/backward through the syntax tree,
//!     builds a parallel *logical subgraph* of [`ReplaceVarnode`]/[`ReplaceOp`]
//!     placeholders plus a [`PatchRecord`] list, and (via [`SubvariableFlow::do_replacement`])
//!     materialises that subgraph as real, logically-sized Varnodes/Ops.  The
//!     worklist *discovery order* (the order placeholders are pushed and the
//!     order descendants are visited) is output-affecting semantics and is
//!     transcribed verbatim.
//!   * The six trigger [`Rule`]s — `RuleSubvarAnd`, `RuleSubvarSubpiece`,
//!     `RuleSubvarCompZero`, `RuleSubvarShift`, `RuleSubvarZext`, `RuleSubvarSext`.
//!   * `SplitFlow` / `RuleSplitFlow` and `LaneDivide`: **SEAM(W5-transform)**.
//!     These derive from `TransformManager` and use `TransformVar`/`TransformOp`/
//!     `LaneDescription`, which live in `decompiler/cpp/transform.{cc,hh}` — a
//!     *separate* checklist item (`w5-s3-transform`, not yet ported, not this
//!     file).  Their structure is preserved in comments and gated behind the
//!     transform seam; see the bottom of this file and the losses output.
//!
//! `SplitDatatype` / `RuleSplit{Copy,Load,Store}` / `RuleDumptyHumpLate` /
//! `SubfloatFlow` / `RuleSubfloatConvert` are **not** in scope for this item
//! (dtype/float-heavy; they sit between `RuleSplitFlow` and `LaneDivide` in the
//! C++ and belong to later dtype/float waves).
//!
//! # Pointer identity → arena indices
//!
//! The C++ keeps `ReplaceVarnode` objects in two `list`/`map` pools and threads
//! raw `ReplaceVarnode *` / `ReplaceOp *` pointers (identity-stable) between
//! them.  Here every placeholder lives in one growable arena (`Vec`) and is
//! referenced by its index ([`RvId`]/[`RopId`]); `varmap: BTreeMap<VarnodeId,RvId>`
//! reproduces the C++ `map<Varnode*,ReplaceVarnode>` (a Varnode appears at most
//! once).  Pushing only ever appends, so indices are stable — matching the C++
//! guarantee that a `ReplaceVarnode *` stays valid for the engine's lifetime.
//!
//! # Cross-wave seams (do not invent behavior — recorded as losses)
//!
//!   * `Funcdata::opSetOutput` (the W3-funcdata `(vbank,obank)` split-borrow) and
//!     opcode→`TypeOp` resolution (`glb->inst[opc]`, W6) are both still seam-gated
//!     in the foundation.  Every graph *mutation* in [`SubvariableFlow::do_replacement`]
//!     and in the rules routes through them; the mutation **structure/order** is
//!     transcribed, but the calls return the foundation's seam error until those
//!     land.  // SEAM(W3-funcdata)/SEAM(W6)
//!   * `FuncCallSpecs`/`FuncProto` call-site state (W4), `JumpTable` (W5/W7), and
//!     `TypeOpFloatInt2Float::preferredZextSize` (W6): the `try*Pull`/`try*Push`
//!     helpers that need them transcribe their guard structure and return a
//!     seam-gated `false`/error.  The integer/shift/compare/copy/extension trace
//!     paths — the bulk of the discovery semantics — are fully executable.

// Faithful-transcription lint relaxations (each mirrors the C++ subflow.cc):
//  * collapsible_if: the C++ nests `if (typelock && !partial) { if (size!=fs) }`
//    and `if (!aggressive) { if (consume...) }` deliberately (separate comments
//    on each guard); kept nested to match.
//  * enum_variant_names: `PatchType` mirrors the C++ `patchtype` names
//    (`copy_patch`/`compare_patch`/...), all `_patch`.
//  * needless_late_init: `let cmask: uintb;` mirrors the C++ `uintb cmask;`
//    declared-then-assigned-in-branches form.
#![allow(clippy::collapsible_if)]
#![allow(clippy::enum_variant_names)]
#![allow(clippy::needless_late_init)]

use std::collections::BTreeMap;

use kuna_base::address::{
    calc_mask, leastsigbit_set, mostsigbit_set, sign_extend_sized, Address,
};
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::types::{int4, uint4, uintb};
use kuna_num::opcodes::OpCode;

use crate::action::{ActionGroupList, Rule, RuleSpec};
use crate::dtype::type_metatype;
use crate::funcdata::Funcdata;
use crate::seams::{OpId, VarnodeId};

// =============================================================================
// SubvariableFlow placeholder data structures (subflow.hh:43-95)
// =============================================================================

/// Arena index of a [`ReplaceVarnode`] (the C++ `ReplaceVarnode *`).
type RvId = usize;
/// Arena index of a [`ReplaceOp`] (the C++ `ReplaceOp *`).
type RopId = usize;

/// Placeholder node for a Varnode holding a smaller logical value
/// (C++ `SubvariableFlow::ReplaceVarnode`).
///
/// `val`/`replacement` are written by discovery and read by the seam-gated
/// [`SubvariableFlow::do_replacement`] (W3-funcdata/W6); kept here so the
/// placeholder layout is the faithful C++ one.  // SEAM(W3-funcdata)/SEAM(W6)
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct ReplaceVarnode {
    /// Varnode being shrunk (`None` for a synthesized constant, C++ `vn==NULL`).
    vn: Option<VarnodeId>,
    /// The new smaller Varnode (filled at replacement time).
    replacement: Option<VarnodeId>,
    /// Bits making up the logical sub-variable.
    mask: uintb,
    /// Value of constant (when `vn` is `None`).
    val: uintb,
    /// Defining op for new Varnode (`None` = the C++ null `def`).
    def: Option<RopId>,
}

/// The possible types of patches on ops being performed
/// (C++ `SubvariableFlow::PatchRecord::patchtype`).
///
/// `Int2FloatPatch` is produced only by the W6-seamed `try_int2float_pull`; the
/// full enum mirrors the C++ patchtype so [`SubvariableFlow::do_replacement`]
/// dispatches on the exact same set.  // SEAM(W6)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
enum PatchType {
    /// Turn op into a COPY of the logical value.
    CopyPatch,
    /// Turn compare op inputs into logical values.
    ComparePatch,
    /// Convert a CALL/CALLIND/RETURN/BRANCHIND parameter into logical value.
    ParameterPatch,
    /// Convert op into something that copies/extends logical value, adding zero bits.
    ExtensionPatch,
    /// Convert an operator output to the logical value.
    PushPatch,
    /// Zero extend logical value into FLOAT_INT2FLOAT operator.
    Int2FloatPatch,
}

/// Operation with a new logical value as (part of) input, but output Varnode is
/// unchanged (C++ `SubvariableFlow::PatchRecord`).
///
/// Fields are written by the `add_*_patch` discovery builders and read only by
/// the seam-gated [`SubvariableFlow::do_replacement`].  // SEAM(W3-funcdata)
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct PatchRecord {
    /// The type of this patch.
    typ: PatchType,
    /// Op being affected.
    patch_op: OpId,
    /// The logical variable input.
    in1: RvId,
    /// (optional second parameter).
    in2: Option<RvId>,
    /// slot being affected or other parameter.
    slot: int4,
}

/// Placeholder node for a PcodeOp operating on smaller logical values
/// (C++ `SubvariableFlow::ReplaceOp`).
///
/// `op`/`replacement`/`opc`/`numparams` are read by the seam-gated
/// [`SubvariableFlow::do_replacement`] when it materialises the new ops.
/// // SEAM(W3-funcdata)
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct ReplaceOp {
    /// op getting paralleled (`None` for an op created purely "down").
    op: Option<OpId>,
    /// The new op (filled at replacement time).
    replacement: Option<OpId>,
    /// Opcode of the new op.
    opc: OpCode,
    /// Number of parameters in (new) op.
    numparams: int4,
    /// Varnode output.
    output: Option<RvId>,
    /// Varnode inputs (`None` entries are the C++ null slots while building).
    input: Vec<Option<RvId>>,
}

// =============================================================================
// SubvariableFlow
// =============================================================================

/// Class for shrinking big Varnodes carrying smaller logical values
/// (C++ `SubvariableFlow`).
///
/// See module docs.  The engine has two halves: [`SubvariableFlow::do_trace`]
/// (the executable discovery: build the placeholder subgraph + patch list) and
/// [`SubvariableFlow::do_replacement`] (materialise it, routed through the
/// seam-gated Funcdata mutation API).
pub struct SubvariableFlow {
    /// Size of the logical data-flow in bytes.
    flowsize: int4,
    /// Number of bits in logical variable.
    bitsize: int4,
    /// Have we tried to flow logical value across CPUI_RETURNs.  Set inside the
    /// W4-seamed `try_return_pull`.  // SEAM(W4)
    #[allow(dead_code)]
    returns_traversed: bool,
    /// Do we "know" initial seed point must be a sub variable.
    aggressive: bool,
    /// Check for logical variables that are always sign extended into their container.
    sextrestrictions: bool,
    /// `false` if the constructor aborted (the C++ `fd = (Funcdata *)0` sentinel).
    valid: bool,
    /// Map from original Varnodes to the overlaying subgraph nodes (C++ `varmap`).
    varmap: BTreeMap<VarnodeId, RvId>,
    /// Storage for ALL subgraph variable nodes (C++ `varmap` values + `newvarlist`).
    /// A single arena gives the C++ pointer-stability across the two C++ pools.
    rvarena: Vec<ReplaceVarnode>,
    /// Storage for subgraph op nodes (C++ `oplist`).
    oplist: Vec<ReplaceOp>,
    /// Operations getting patched (C++ `patchlist`).  `push_patch` records are
    /// inserted at the *front* (see [`SubvariableFlow::add_push`]).
    patchlist: Vec<PatchRecord>,
    /// Subgraph variable nodes still needing to be traced (C++ `worklist`).
    worklist: Vec<RvId>,
    /// Number of instructions pulling out the logical value (C++ `pullcount`).
    pullcount: int4,
}

impl SubvariableFlow {
    // -------------------------------------------------------------------------
    // arena helpers (pointer-deref equivalents)
    // -------------------------------------------------------------------------

    fn rv(&self, id: RvId) -> &ReplaceVarnode {
        &self.rvarena[id]
    }
    fn rv_mut(&mut self, id: RvId) -> &mut ReplaceVarnode {
        &mut self.rvarena[id]
    }

    /// Append a fresh placeholder variable node and return its id.
    fn new_rv(&mut self, rv: ReplaceVarnode) -> RvId {
        self.rvarena.push(rv);
        self.rvarena.len() - 1
    }

    // -------------------------------------------------------------------------
    // static bit helpers (subflow.cc:26-53)
    // -------------------------------------------------------------------------

    /// Return \e slot of constant if INT_OR op sets all bits in mask, otherwise -1
    /// (C++ `SubvariableFlow::doesOrSet`).
    fn does_or_set(data: &Funcdata, orop: OpId, mask: uintb) -> int4 {
        let o = data.obank().get(orop).expect("does_or_set: stale op");
        let in1 = o.get_in(1).expect("does_or_set: missing in1");
        let index: int4 = if data.vbank().get(in1).expect("vn").is_constant() { 1 } else { 0 };
        let inx = o.get_in(index).expect("does_or_set: missing in[index]");
        if !data.vbank().get(inx).expect("vn").is_constant() {
            return -1;
        }
        let orval = data.vbank().get(inx).expect("vn").get_offset();
        if (mask & (!orval)) == 0u64 {
            // Are all masked bits one
            return index;
        }
        -1
    }

    /// Return \e slot of constant if INT_AND op clears all bits in mask, otherwise -1
    /// (C++ `SubvariableFlow::doesAndClear`).
    fn does_and_clear(data: &Funcdata, andop: OpId, mask: uintb) -> int4 {
        let o = data.obank().get(andop).expect("does_and_clear: stale op");
        let in1 = o.get_in(1).expect("does_and_clear: missing in1");
        let index: int4 = if data.vbank().get(in1).expect("vn").is_constant() { 1 } else { 0 };
        let inx = o.get_in(index).expect("does_and_clear: missing in[index]");
        if !data.vbank().get(inx).expect("vn").is_constant() {
            return -1;
        }
        let andval = data.vbank().get(inx).expect("vn").get_offset();
        if (mask & andval) == 0u64 {
            // Are all masked bits zero
            return index;
        }
        -1
    }

    // -------------------------------------------------------------------------
    // setReplacement (subflow.cc:66-151)
    // -------------------------------------------------------------------------

    /// Add the given Varnode as a new node in the logical subgraph
    /// (C++ `SubvariableFlow::setReplacement`).
    ///
    /// Returns `(Some(rvid), inworklist)` on success, or `(None, _)` on abort
    /// (the C++ null return).  Mirrors the C++ out-param `inworklist`.
    fn set_replacement(
        &mut self,
        data: &mut Funcdata,
        vn: VarnodeId,
        mask: uintb,
    ) -> (Option<RvId>, bool) {
        // res; if (vn->isMark()) { ... }   -- Already seen before
        if data.vbank().get(vn).expect("set_replacement: stale vn").is_mark() {
            let res = *self.varmap.get(&vn).expect("marked vn must be in varmap");
            let inworklist = false;
            if self.rv(res).mask != mask {
                return (None, inworklist);
            }
            return (Some(res), inworklist);
        }

        let v = data.vbank().get(vn).expect("set_replacement: stale vn");

        if v.is_constant() {
            let inworklist = false;
            if self.sextrestrictions {
                // Check that -vn- is a sign extension
                let cval = v.get_offset();
                let smallval = cval & mask; // From its logical size
                let sextval = sign_extend_sized(smallval, self.flowsize, v.get_size()); // to its fullsize
                if sextval != cval {
                    return (None, inworklist);
                }
            }
            let rv = self.add_constant(data, None, mask, 0, vn);
            return (Some(rv), inworklist);
        }

        if v.is_free() {
            return (None, false); // Abort
        }

        if v.is_addr_force() && (v.get_size() != self.flowsize) {
            return (None, false);
        }

        if self.sextrestrictions {
            if v.get_size() != self.flowsize {
                if (!self.aggressive) && v.is_input() {
                    return (None, false); // Cannot assume input is sign extended
                }
                if v.is_persist() {
                    return (None, false);
                }
            }
            if v.is_type_lock() && v.get_type().get_metatype() != type_metatype::TYPE_PARTIALSTRUCT
            {
                if v.get_type().get_size() != self.flowsize {
                    return (None, false);
                }
            }
        } else {
            if self.bitsize >= 8 {
                // Not a flag
                // If the logical variable is not a flag, don't consider the case where multiple variables
                // are packed into a single location, i.e. always consider it a single variable
                if (!self.aggressive) && ((v.get_consume() & !mask) != 0) {
                    // If there is any use of value outside of the logical variable
                    return (None, false); // This probably means the whole thing is a variable, i.e. quit
                }
                if v.is_type_lock()
                    && v.get_type().get_metatype() != type_metatype::TYPE_PARTIALSTRUCT
                {
                    let sz = v.get_type().get_size();
                    if sz != self.flowsize {
                        return (None, false);
                    }
                }
            }

            if v.is_input() {
                // Must be careful with inputs
                // Inputs must come in from the right register/memory
                if self.bitsize < 8 {
                    return (None, false); // Dont create input flag
                }
                if (mask & 1) == 0 {
                    return (None, false); // Dont create unique input
                }
                // Its extremely important that the code (above) which doesn't allow packed variables be applied
                // or the mechanisms we use for inputs will give us spurious temporary inputs
            }
        }

        // res = & varmap[ vn ];  -- create a fresh node, register in varmap, mark vn
        let vsize = v.get_size();
        let is_written = v.is_written();
        let res = self.new_rv(ReplaceVarnode {
            vn: Some(vn),
            replacement: None,
            mask,
            val: 0,
            def: None,
        });
        self.varmap.insert(vn, res);
        data.vbank_mut().get_mut(vn).expect("set_replacement: stale vn").set_mark();

        let mut inworklist = true;
        // Check if vn already represents the logical variable being traced
        if vsize == self.flowsize {
            if mask == calc_mask(self.flowsize) {
                inworklist = false;
                self.rv_mut(res).replacement = Some(vn);
            } else if mask == 1 {
                // if ((vn->isWritten())&&(vn->getDef()->isBoolOutput()))
                let bool_out = if is_written {
                    let defop =
                        data.vbank().get(vn).expect("vn").get_def().expect("written vn has def");
                    data.obank().get(defop).expect("stale def").is_bool_output()
                } else {
                    false
                };
                if bool_out {
                    inworklist = false;
                    self.rv_mut(res).replacement = Some(vn);
                }
            }
        }
        (Some(res), inworklist)
    }

    // -------------------------------------------------------------------------
    // createOp / createOpDown (subflow.cc:159-197)
    // -------------------------------------------------------------------------

    /// Create a logical subgraph operator node given its output variable node
    /// (C++ `SubvariableFlow::createOp`).
    fn create_op(&mut self, data: &Funcdata, opc: OpCode, numparam: int4, outrvn: RvId) -> RopId {
        if let Some(def) = self.rv(outrvn).def {
            return def;
        }
        // rop->op = outrvn->vn->getDef();  -- the defining op of the *original*
        // Varnode the placeholder shadows (callers only reach here on real vns).
        let vn = self.rv(outrvn).vn.expect("createOp: outrvn has no vn");
        let op = data.vbank().get(vn).expect("createOp: stale outrvn vn").get_def();
        self.oplist.push(ReplaceOp {
            op,
            replacement: None,
            opc,
            numparams: numparam,
            output: Some(outrvn),
            input: Vec::new(),
        });
        let rop = self.oplist.len() - 1;
        self.rv_mut(outrvn).def = Some(rop);
        rop
    }

    /// Create a logical subgraph operator node given one of its input variable nodes
    /// (C++ `SubvariableFlow::createOpDown`).
    fn create_op_down(
        &mut self,
        opc: OpCode,
        numparam: int4,
        op: OpId,
        inrvn: RvId,
        slot: int4,
    ) -> RopId {
        self.oplist.push(ReplaceOp {
            op: Some(op),
            replacement: None,
            opc,
            numparams: numparam,
            output: None,
            input: Vec::new(),
        });
        let rop = self.oplist.len() - 1;
        while (self.oplist[rop].input.len() as int4) <= slot {
            self.oplist[rop].input.push(None);
        }
        self.oplist[rop].input[slot as usize] = Some(inrvn);
        rop
    }

    // -------------------------------------------------------------------------
    // try* terminal helpers (subflow.cc:208-367) — W4/W6/jumptable seams
    // -------------------------------------------------------------------------

    /// Determine if the given subgraph variable can act as a parameter to the
    /// given CALL op (C++ `SubvariableFlow::tryCallPull`).
    ///
    /// SEAM(W4): `fd->getCallSpecs(op)` and `FuncCallSpecs::isInputActive/
    /// isInputLocked/isDotdotdot` are the W4 call-site machinery.  The guard
    /// structure is transcribed; until W4 lands this returns the seam-gated
    /// `Err`, since the early `slot==0` / consume guards alone cannot decide it.
    fn try_call_pull(
        &mut self,
        data: &Funcdata,
        _op: OpId,
        rvn: RvId,
        slot: int4,
    ) -> KunaResult<bool> {
        if slot == 0 {
            return Ok(false);
        }
        if !self.aggressive {
            let vn = self.rv(rvn).vn.expect("try_call_pull: rvn vn");
            if (data.vbank().get(vn).expect("vn").get_consume() & !self.rv(rvn).mask) != 0 {
                // If there's something outside the mask being consumed
                return Ok(false); // Don't truncate
            }
        }
        // FuncCallSpecs *fc = fd->getCallSpecs(op);
        // if (fc == 0) return false;
        // if (fc->isInputActive()) return false;
        // if (fc->isInputLocked() && !fc->isDotdotdot()) return false;
        Err(KunaError::lowlevel(
            "kuna rust port: SubvariableFlow::tryCallPull needs FuncCallSpecs \
             (fd->getCallSpecs / isInputActive / isInputLocked / isDotdotdot) — SEAM(W4)",
        ))
    }

    /// Determine if the given subgraph variable can act as return value for the
    /// given RETURN op (C++ `SubvariableFlow::tryReturnPull`).
    ///
    /// SEAM(W4): `fd->getFuncProto().isOutputLocked()` is the W4 prototype state.
    /// The cross-RETURN propagation loop and the terminal patch are transcribed.
    fn try_return_pull(
        &mut self,
        data: &mut Funcdata,
        op: OpId,
        rvn: RvId,
        slot: int4,
    ) -> KunaResult<bool> {
        if slot == 0 {
            return Ok(false); // Don't deal with actual return address container
        }
        // if (fd->getFuncProto().isOutputLocked()) return false;   -- SEAM(W4)
        // We cannot evaluate isOutputLocked() without the W4 FuncProto; the
        // structure below (the cross-RETURN propagation + terminal patch) is
        // transcribed but gated.
        let _ = (data, op, rvn);
        Err(KunaError::lowlevel(
            "kuna rust port: SubvariableFlow::tryReturnPull needs FuncProto::isOutputLocked — SEAM(W4)",
        ))
    }

    /// Determine if the given subgraph variable can act as a \e created value for
    /// the given INDIRECT/CALL op (C++ `SubvariableFlow::tryCallReturnPush`).
    ///
    /// SEAM(W4): `fd->getCallSpecs(op)` / `isOutputLocked` / `isOutputActive`.
    fn try_call_return_push(
        &mut self,
        data: &Funcdata,
        op: OpId,
        rvn: RvId,
    ) -> KunaResult<bool> {
        if !self.aggressive {
            let vn = self.rv(rvn).vn.expect("try_call_return_push: rvn vn");
            if (data.vbank().get(vn).expect("vn").get_consume() & !self.rv(rvn).mask) != 0 {
                return Ok(false);
            }
        }
        if (self.rv(rvn).mask & 1) == 0 {
            return Ok(false); // Verify the logical value is the least significant part
        }
        if self.bitsize < 8 {
            return Ok(false); // Make sure logical value is at least a byte
        }
        // FuncCallSpecs *fc = fd->getCallSpecs(op); if (0) ...; isOutputLocked/Active
        let _ = op;
        Err(KunaError::lowlevel(
            "kuna rust port: SubvariableFlow::tryCallReturnPush needs FuncCallSpecs \
             (isOutputLocked / isOutputActive) — SEAM(W4)",
        ))
    }

    /// Determine if the subgraph variable can act as a switch variable for the
    /// given BRANCHIND (C++ `SubvariableFlow::trySwitchPull`).
    ///
    /// The consume guards are executable; the actual JumpTable query the C++
    /// comment describes is implicit (the C++ body only checks the mask/consume
    /// and records a parameter patch).  Fully transcribed and executable.
    fn try_switch_pull(&mut self, data: &Funcdata, op: OpId, rvn: RvId) -> bool {
        if (self.rv(rvn).mask & 1) == 0 {
            return false; // Logical value must be justified
        }
        let vn = self.rv(rvn).vn.expect("try_switch_pull: rvn vn");
        if (data.vbank().get(vn).expect("vn").get_consume() & !self.rv(rvn).mask) != 0 {
            // If there's something outside the mask being consumed
            return false; //  we can't trim
        }
        self.patchlist.push(PatchRecord {
            typ: PatchType::ParameterPatch,
            patch_op: op,
            in1: rvn,
            in2: None,
            slot: 0,
        });
        self.pullcount += 1; // A true terminal modification
        true
    }

    /// Determine if the subgraph variable flows naturally into a terminal
    /// FLOAT_INT2FLOAT operation (C++ `SubvariableFlow::tryInt2FloatPull`).
    ///
    /// SEAM(W6): `TypeOpFloatInt2Float::preferredZextSize(flowsize)`.  The NZ/size
    /// guards are executable; the preferred-size check that decides whether the
    /// patch counts as a modification needs the W6 type-op.
    fn try_int2float_pull(&mut self, data: &Funcdata, op: OpId, rvn: RvId) -> KunaResult<bool> {
        if (self.rv(rvn).mask & 1) == 0 {
            return Ok(false); // Logical value must be justified
        }
        let vn = self.rv(rvn).vn.expect("try_int2float_pull: rvn vn");
        if (data.vbank().get(vn).expect("vn").get_nz_mask() & !self.rv(rvn).mask) != 0 {
            return Ok(false); // Everything outside the logical value must be zero
        }
        if data.vbank().get(vn).expect("vn").get_size() == self.flowsize {
            return Ok(false); // There must be some (zero) extension
        }
        // The pullModification refinement reads TypeOpFloatInt2Float::preferredZextSize.
        let _ = op;
        Err(KunaError::lowlevel(
            "kuna rust port: SubvariableFlow::tryInt2FloatPull needs \
             TypeOpFloatInt2Float::preferredZextSize — SEAM(W6)",
        ))
    }

    // -------------------------------------------------------------------------
    // traceForward (subflow.cc:373-659)
    // -------------------------------------------------------------------------

    /// Trace the logical data-flow forward for the given subgraph variable
    /// (C++ `SubvariableFlow::traceForward`).
    fn trace_forward(&mut self, data: &mut Funcdata, rvn: RvId) -> KunaResult<bool> {
        let mut dcount = 0i32;
        let mut hcount = 0i32;
        let mut callcount = 0i32;

        let rvn_vn = self.rv(rvn).vn.expect("trace_forward: rvn vn");
        // enditer = rvn->vn->endDescend(); for(iter=beginDescend ...)
        let descend: Vec<OpId> = data.descend_snapshot(rvn_vn);
        for (idx, &op) in descend.iter().enumerate() {
            let outvn = data.obank().get(op).expect("stale op").get_out();
            let is_call = data.obank().get(op).expect("stale op").is_call();
            if let Some(ov) = outvn {
                if data.vbank().get(ov).expect("vn").is_mark() && !is_call {
                    continue;
                }
            }
            dcount += 1; // Count this descendant
            let mut slot = data.obank().get(op).expect("stale op").get_slot(rvn_vn);
            let code = data.obank().get(op).expect("stale op").code();
            match code {
                OpCode::CPUI_COPY
                | OpCode::CPUI_MULTIEQUAL
                | OpCode::CPUI_INT_NEGATE
                | OpCode::CPUI_INT_XOR => {
                    let numin = data.obank().get(op).expect("stale op").num_input();
                    let rop = self.create_op_down(code, numin, op, rvn, slot);
                    let mask = self.rv(rvn).mask;
                    if !self.create_link(data, rop, mask, -1, outvn.expect("outvn"))? {
                        return Ok(false);
                    }
                    hcount += 1; // Dealt with this descendant
                }
                OpCode::CPUI_INT_OR => {
                    if Self::does_or_set(data, op, self.rv(rvn).mask) != -1 {
                        continue; // Subvar set to 1s, truncate flow
                    }
                    let rop = self.create_op_down(OpCode::CPUI_INT_OR, 2, op, rvn, slot);
                    let mask = self.rv(rvn).mask;
                    if !self.create_link(data, rop, mask, -1, outvn.expect("outvn"))? {
                        return Ok(false);
                    }
                    hcount += 1;
                }
                OpCode::CPUI_INT_AND => {
                    let in1 = data.obank().get(op).expect("stale op").get_in(1).expect("in1");
                    let in1c = data.vbank().get(in1).expect("vn").is_constant();
                    let in1off = data.vbank().get(in1).expect("vn").get_offset();
                    let mask = self.rv(rvn).mask;
                    if in1c && (in1off == mask) {
                        let ov = outvn.expect("outvn");
                        let outsize = data.vbank().get(ov).expect("vn").get_size();
                        if (outsize == self.flowsize) && ((mask & 1) != 0) {
                            self.add_terminal_patch(op, rvn);
                            hcount += 1;
                            continue;
                        }
                        // Is the small variable getting zero padded into something that is fully consumed
                        let outconsume = data.vbank().get(ov).expect("vn").get_consume();
                        if (!self.aggressive) && ((outconsume & mask) != outconsume) {
                            self.add_extension_patch(rvn, op, -1);
                            hcount += 1;
                            continue;
                        }
                    }
                    if Self::does_and_clear(data, op, mask) != -1 {
                        continue; // Subvar set to zero, truncate flow
                    }
                    let rop = self.create_op_down(OpCode::CPUI_INT_AND, 2, op, rvn, slot);
                    if !self.create_link(data, rop, mask, -1, outvn.expect("outvn"))? {
                        return Ok(false);
                    }
                    hcount += 1;
                }
                OpCode::CPUI_INT_ZEXT | OpCode::CPUI_INT_SEXT => {
                    let rop = self.create_op_down(OpCode::CPUI_COPY, 1, op, rvn, 0);
                    let mask = self.rv(rvn).mask;
                    if !self.create_link(data, rop, mask, -1, outvn.expect("outvn"))? {
                        return Ok(false);
                    }
                    hcount += 1;
                }
                OpCode::CPUI_INT_MULT => {
                    if (self.rv(rvn).mask & 1) == 0 {
                        return Ok(false); // Cannot account for carry
                    }
                    let other = data.obank().get(op).expect("stale op").get_in(1 - slot).expect("in");
                    let mut sa = leastsigbit_set(data.vbank().get(other).expect("vn").get_nz_mask());
                    sa &= !7; // Should be nearest multiple of 8
                    if self.bitsize + sa > 8 * data.vbank().get(rvn_vn).expect("vn").get_size() {
                        return Ok(false);
                    }
                    let rop = self.create_op_down(OpCode::CPUI_INT_MULT, 2, op, rvn, slot);
                    let mask = self.rv(rvn).mask << sa;
                    if !self.create_link(data, rop, mask, -1, outvn.expect("outvn"))? {
                        return Ok(false);
                    }
                    hcount += 1;
                }
                OpCode::CPUI_INT_DIV | OpCode::CPUI_INT_REM => {
                    if (self.rv(rvn).mask & 1) == 0 {
                        return Ok(false); // Logical value must be least sig bits
                    }
                    if (self.bitsize & 7) != 0 {
                        return Ok(false); // Must be a whole number of bytes
                    }
                    let i0 = data.obank().get(op).expect("stale op").get_in(0).expect("in0");
                    let i1 = data.obank().get(op).expect("stale op").get_in(1).expect("in1");
                    if !Self::is_zero_extended(data, i0, self.flowsize)? {
                        return Ok(false);
                    }
                    if !Self::is_zero_extended(data, i1, self.flowsize)? {
                        return Ok(false);
                    }
                    let rop = self.create_op_down(code, 2, op, rvn, slot);
                    let mask = self.rv(rvn).mask;
                    if !self.create_link(data, rop, mask, -1, outvn.expect("outvn"))? {
                        return Ok(false);
                    }
                    hcount += 1;
                }
                OpCode::CPUI_INT_ADD => {
                    if (self.rv(rvn).mask & 1) == 0 {
                        return Ok(false); // Cannot account for carry
                    }
                    let rop = self.create_op_down(OpCode::CPUI_INT_ADD, 2, op, rvn, slot);
                    let mask = self.rv(rvn).mask;
                    if !self.create_link(data, rop, mask, -1, outvn.expect("outvn"))? {
                        return Ok(false);
                    }
                    hcount += 1;
                }
                OpCode::CPUI_INT_LEFT => {
                    if slot == 1 {
                        // Logical flow is into shift amount
                        if (self.rv(rvn).mask & 1) == 0 {
                            return Ok(false); // Cannot account for effect of extraneous bits
                        }
                        if self.bitsize < 8 {
                            return Ok(false);
                        }
                        // Its possible that truncating to the logical value could have an effect ... (>255 undefined)
                        self.add_terminal_patch_same_op(op, rvn, slot);
                        hcount += 1;
                        continue;
                    }
                    let in1 = data.obank().get(op).expect("stale op").get_in(1).expect("in1");
                    if !data.vbank().get(in1).expect("vn").is_constant() {
                        return Ok(false); // Dynamic shift
                    }
                    let sa = data.vbank().get(in1).expect("vn").get_offset() as int4;
                    if sa as u64 >= (std::mem::size_of::<uintb>() as u64) * 8 {
                        return Ok(false); // Beyond precision of mask
                    }
                    let ov = outvn.expect("outvn");
                    let outsize = data.vbank().get(ov).expect("vn").get_size();
                    let mask = self.rv(rvn).mask;
                    let newmask = (mask << sa) & calc_mask(outsize);
                    if newmask == 0 {
                        continue; // Subvar is cleared, truncate flow
                    }
                    if mask != (newmask >> sa) {
                        return Ok(false); // subvar is clipped
                    }
                    // Is the small variable getting zero padded into something that is consumed beyond the variable
                    let outconsume = data.vbank().get(ov).expect("vn").get_consume();
                    if ((mask & 1) != 0)
                        && (sa + self.bitsize == 8 * outsize)
                        && ((outconsume & !newmask) != 0)
                    {
                        self.add_extension_patch(rvn, op, sa);
                        hcount += 1;
                        continue;
                    }
                    let rop = self.create_op_down(OpCode::CPUI_COPY, 1, op, rvn, 0);
                    if !self.create_link(data, rop, newmask, -1, ov)? {
                        return Ok(false);
                    }
                    hcount += 1;
                }
                OpCode::CPUI_INT_RIGHT | OpCode::CPUI_INT_SRIGHT => {
                    if slot == 1 {
                        // Logical flow is into shift amount
                        if (self.rv(rvn).mask & 1) == 0 {
                            return Ok(false);
                        }
                        if self.bitsize < 8 {
                            return Ok(false);
                        }
                        self.add_terminal_patch_same_op(op, rvn, slot);
                        hcount += 1;
                        continue;
                    }
                    let in1 = data.obank().get(op).expect("stale op").get_in(1).expect("in1");
                    if !data.vbank().get(in1).expect("vn").is_constant() {
                        return Ok(false);
                    }
                    let sa = data.vbank().get(in1).expect("vn").get_offset() as int4;
                    let mask = self.rv(rvn).mask;
                    let newmask = if sa as u64 >= (std::mem::size_of::<uintb>() as u64) * 8 {
                        0
                    } else {
                        mask >> sa
                    };
                    if newmask == 0 {
                        if code == OpCode::CPUI_INT_RIGHT {
                            continue; // subvar does not pass thru, truncate flow
                        }
                        return Ok(false);
                    }
                    if mask != (newmask << sa) {
                        return Ok(false);
                    }
                    let ov = outvn.expect("outvn");
                    let outsize = data.vbank().get(ov).expect("vn").get_size();
                    let i0 = data.obank().get(op).expect("stale op").get_in(0).expect("in0");
                    let i0nz = data.vbank().get(i0).expect("vn").get_nz_mask();
                    if (outsize == self.flowsize) && ((newmask & 1) == 1) && (i0nz == mask) {
                        self.add_terminal_patch(op, rvn);
                        hcount += 1;
                        continue;
                    }
                    // Is the small variable getting zero padded into something that is consumed beyond the variable
                    let outconsume = data.vbank().get(ov).expect("vn").get_consume();
                    if ((newmask & 1) == 1)
                        && (sa + self.bitsize == 8 * outsize)
                        && ((outconsume & !newmask) != 0)
                    {
                        self.add_extension_patch(rvn, op, 0);
                        hcount += 1;
                        continue;
                    }
                    let rop = self.create_op_down(OpCode::CPUI_COPY, 1, op, rvn, 0);
                    if !self.create_link(data, rop, newmask, -1, ov)? {
                        return Ok(false);
                    }
                    hcount += 1;
                }
                OpCode::CPUI_SUBPIECE => {
                    let in1 = data.obank().get(op).expect("stale op").get_in(1).expect("in1");
                    let sa = (data.vbank().get(in1).expect("vn").get_offset() as int4) * 8;
                    if sa as u64 >= (std::mem::size_of::<uintb>() as u64) * 8 {
                        continue;
                    }
                    let ov = outvn.expect("outvn");
                    let outsize = data.vbank().get(ov).expect("vn").get_size();
                    let mask = self.rv(rvn).mask;
                    let newmask = (mask >> sa) & calc_mask(outsize);
                    if newmask == 0 {
                        continue; // subvar is set to zero, truncate flow
                    }
                    if mask != (newmask << sa) {
                        // Some kind of truncation of the logical value
                        if self.flowsize > ((sa / 8) + outsize) && (mask & 1) != 0 {
                            // Only a piece of the logical value remains
                            self.add_terminal_patch_same_op(op, rvn, 0);
                            hcount += 1;
                            continue;
                        }
                        return Ok(false);
                    }
                    if ((newmask & 1) != 0) && (outsize == self.flowsize) {
                        self.add_terminal_patch(op, rvn);
                        hcount += 1;
                        continue;
                    }
                    let rop = self.create_op_down(OpCode::CPUI_COPY, 1, op, rvn, 0);
                    if !self.create_link(data, rop, newmask, -1, ov)? {
                        return Ok(false);
                    }
                    hcount += 1;
                }
                OpCode::CPUI_PIECE => {
                    let i0 = data.obank().get(op).expect("stale op").get_in(0).expect("in0");
                    let mask = self.rv(rvn).mask;
                    let newmask = if rvn_vn == i0 {
                        let i1 = data.obank().get(op).expect("stale op").get_in(1).expect("in1");
                        let i1size = data.vbank().get(i1).expect("vn").get_size();
                        mask << (8 * i1size)
                    } else {
                        mask
                    };
                    let rop = self.create_op_down(OpCode::CPUI_COPY, 1, op, rvn, 0);
                    if !self.create_link(data, rop, newmask, -1, outvn.expect("outvn"))? {
                        return Ok(false);
                    }
                    hcount += 1;
                }
                OpCode::CPUI_INT_LESS | OpCode::CPUI_INT_LESSEQUAL => {
                    let othervn =
                        data.obank().get(op).expect("stale op").get_in(1 - slot).expect("in"); // The OTHER side
                    let mask = self.rv(rvn).mask;
                    let rvn_nz = data.vbank().get(rvn_vn).expect("vn").get_nz_mask();
                    if (!self.aggressive) && ((rvn_nz | mask) != mask) {
                        return Ok(false); // Everything but logical variable must definitely be zero
                    }
                    if data.vbank().get(othervn).expect("vn").is_constant() {
                        let ooff = data.vbank().get(othervn).expect("vn").get_offset();
                        if (mask | ooff) != mask {
                            return Ok(false); // Must compare only bits of logical variable
                        }
                    } else {
                        let onz = data.vbank().get(othervn).expect("vn").get_nz_mask();
                        if (!self.aggressive) && ((mask | onz) != mask) {
                            return Ok(false); // unused bits of otherside must be zero
                        }
                    }
                    if !self.create_compare_bridge(data, op, rvn, slot, othervn)? {
                        return Ok(false);
                    }
                    hcount += 1;
                }
                OpCode::CPUI_INT_NOTEQUAL | OpCode::CPUI_INT_EQUAL => {
                    let othervn =
                        data.obank().get(op).expect("stale op").get_in(1 - slot).expect("in"); // The OTHER side
                    let mask = self.rv(rvn).mask;
                    if self.bitsize != 1 {
                        let rvn_nz = data.vbank().get(rvn_vn).expect("vn").get_nz_mask();
                        if (!self.aggressive) && ((rvn_nz | mask) != mask) {
                            return Ok(false);
                        }
                        if data.vbank().get(othervn).expect("vn").is_constant() {
                            let ooff = data.vbank().get(othervn).expect("vn").get_offset();
                            if (mask | ooff) != mask {
                                return Ok(false);
                            }
                        } else {
                            let onz = data.vbank().get(othervn).expect("vn").get_nz_mask();
                            if (!self.aggressive) && ((mask | onz) != mask) {
                                return Ok(false);
                            }
                        }
                        if !self.create_compare_bridge(data, op, rvn, slot, othervn)? {
                            return Ok(false);
                        }
                    } else {
                        // Movement of boolean variables
                        if !data.vbank().get(othervn).expect("vn").is_constant() {
                            return Ok(false);
                        }
                        let newmask = data.vbank().get(rvn_vn).expect("vn").get_nz_mask();
                        if newmask != mask {
                            return Ok(false);
                        }
                        let ooff = data.vbank().get(othervn).expect("vn").get_offset();
                        let mut booldir = if ooff == 0u64 {
                            true
                        } else if ooff == newmask {
                            false
                        } else {
                            return Ok(false);
                        };
                        if code == OpCode::CPUI_INT_EQUAL {
                            booldir = !booldir;
                        }
                        if booldir {
                            self.add_terminal_patch(op, rvn);
                        } else {
                            let rop =
                                self.create_op_down(OpCode::CPUI_BOOL_NEGATE, 1, op, rvn, 0);
                            self.create_new_out(rop, 1u64);
                            let out = self.oplist[rop].output.expect("created out");
                            self.add_terminal_patch(op, out);
                        }
                    }
                    hcount += 1;
                }
                OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => {
                    callcount += 1;
                    if callcount > 1 {
                        let o = data.obank().get(op).expect("stale op");
                        slot = o.get_repeat_slot(rvn_vn, slot, &descend[..idx], op);
                    }
                    if !self.try_call_pull(data, op, rvn, slot)? {
                        return Ok(false);
                    }
                    hcount += 1;
                }
                OpCode::CPUI_RETURN => {
                    if !self.try_return_pull(data, op, rvn, slot)? {
                        return Ok(false);
                    }
                    hcount += 1;
                }
                OpCode::CPUI_BRANCHIND => {
                    if !self.try_switch_pull(data, op, rvn) {
                        return Ok(false);
                    }
                    hcount += 1;
                }
                OpCode::CPUI_BOOL_NEGATE
                | OpCode::CPUI_BOOL_AND
                | OpCode::CPUI_BOOL_OR
                | OpCode::CPUI_BOOL_XOR => {
                    if self.bitsize != 1 {
                        return Ok(false);
                    }
                    if self.rv(rvn).mask != 1 {
                        return Ok(false);
                    }
                    self.add_boolean_patch(op, rvn, slot);
                    // C++ does NOT bump hcount here (matches the original)
                }
                OpCode::CPUI_FLOAT_INT2FLOAT => {
                    if !self.try_int2float_pull(data, op, rvn)? {
                        return Ok(false);
                    }
                    hcount += 1;
                }
                OpCode::CPUI_CBRANCH => {
                    if (self.bitsize != 1) || (slot != 1) {
                        return Ok(false);
                    }
                    if self.rv(rvn).mask != 1 {
                        return Ok(false);
                    }
                    self.add_boolean_patch(op, rvn, 1);
                    hcount += 1;
                }
                _ => {
                    return Ok(false);
                }
            }
        }
        if dcount != hcount {
            // Must account for all descendants of an input
            if data.vbank().get(rvn_vn).expect("vn").is_input() {
                return Ok(false);
            }
        }
        Ok(true)
    }

    // -------------------------------------------------------------------------
    // traceBackward (subflow.cc:665-861)
    // -------------------------------------------------------------------------

    /// Trace the logical data-flow backward for the given subgraph variable
    /// (C++ `SubvariableFlow::traceBackward`).
    fn trace_backward(&mut self, data: &mut Funcdata, rvn: RvId) -> KunaResult<bool> {
        let rvn_vn = self.rv(rvn).vn.expect("trace_backward: rvn vn");
        let op = match data.vbank().get(rvn_vn).expect("vn").get_def() {
            Some(o) => o,
            None => return Ok(true), // If vn is input
        };
        let code = data.obank().get(op).expect("stale op").code();
        let mask = self.rv(rvn).mask;

        match code {
            OpCode::CPUI_COPY
            | OpCode::CPUI_MULTIEQUAL
            | OpCode::CPUI_INT_NEGATE
            | OpCode::CPUI_INT_XOR => {
                let numin = data.obank().get(op).expect("stale op").num_input();
                let rop = self.create_op(data, code, numin, rvn);
                for i in 0..numin {
                    let ini = data.obank().get(op).expect("stale op").get_in(i).expect("in");
                    if !self.create_link(data, rop, mask, i, ini)? {
                        return Ok(false);
                    }
                }
                Ok(true)
            }
            OpCode::CPUI_INT_AND => {
                let sa = Self::does_and_clear(data, op, mask);
                if sa != -1 {
                    let rop = self.create_op(data, OpCode::CPUI_COPY, 1, rvn);
                    let insa = data.obank().get(op).expect("stale op").get_in(sa).expect("in");
                    self.add_constant(data, Some(rop), mask, 0, insa);
                } else {
                    let rop = self.create_op(data, OpCode::CPUI_INT_AND, 2, rvn);
                    let i0 = data.obank().get(op).expect("stale op").get_in(0).expect("in0");
                    let i1 = data.obank().get(op).expect("stale op").get_in(1).expect("in1");
                    if !self.create_link(data, rop, mask, 0, i0)? {
                        return Ok(false);
                    }
                    if !self.create_link(data, rop, mask, 1, i1)? {
                        return Ok(false);
                    }
                }
                Ok(true)
            }
            OpCode::CPUI_INT_OR => {
                let sa = Self::does_or_set(data, op, mask);
                if sa != -1 {
                    let rop = self.create_op(data, OpCode::CPUI_COPY, 1, rvn);
                    let insa = data.obank().get(op).expect("stale op").get_in(sa).expect("in");
                    self.add_constant(data, Some(rop), mask, 0, insa);
                } else {
                    let rop = self.create_op(data, OpCode::CPUI_INT_OR, 2, rvn);
                    let i0 = data.obank().get(op).expect("stale op").get_in(0).expect("in0");
                    let i1 = data.obank().get(op).expect("stale op").get_in(1).expect("in1");
                    if !self.create_link(data, rop, mask, 0, i0)? {
                        return Ok(false);
                    }
                    if !self.create_link(data, rop, mask, 1, i1)? {
                        return Ok(false);
                    }
                }
                Ok(true)
            }
            OpCode::CPUI_INT_ZEXT | OpCode::CPUI_INT_SEXT => {
                let i0 = data.obank().get(op).expect("stale op").get_in(0).expect("in0");
                let i0size = data.vbank().get(i0).expect("vn").get_size();
                if (mask & calc_mask(i0size)) != mask {
                    if (mask & 1) != 0 && self.flowsize > i0size {
                        self.add_push(data, op, rvn)?;
                        return Ok(true);
                    }
                    // break; -- Check if subvariable comes through extension
                    return Ok(false);
                }
                let rop = self.create_op(data, OpCode::CPUI_COPY, 1, rvn);
                if !self.create_link(data, rop, mask, 0, i0)? {
                    return Ok(false);
                }
                Ok(true)
            }
            OpCode::CPUI_INT_ADD => {
                if (mask & 1) == 0 {
                    return Ok(false); // break -- Cannot account for carry
                }
                let rop = if mask == 1u64 {
                    self.create_op(data, OpCode::CPUI_INT_XOR, 2, rvn) // Single bit add
                } else {
                    self.create_op(data, OpCode::CPUI_INT_ADD, 2, rvn)
                };
                let i0 = data.obank().get(op).expect("stale op").get_in(0).expect("in0");
                let i1 = data.obank().get(op).expect("stale op").get_in(1).expect("in1");
                if !self.create_link(data, rop, mask, 0, i0)? {
                    return Ok(false);
                }
                if !self.create_link(data, rop, mask, 1, i1)? {
                    return Ok(false);
                }
                Ok(true)
            }
            OpCode::CPUI_INT_LEFT => {
                let i1 = data.obank().get(op).expect("stale op").get_in(1).expect("in1");
                if !data.vbank().get(i1).expect("vn").is_constant() {
                    return Ok(false); // break -- Dynamic shift
                }
                let sa = data.vbank().get(i1).expect("vn").get_offset() as int4;
                let newmask = if sa as u64 >= (std::mem::size_of::<uintb>() as u64) * 8 {
                    0
                } else {
                    mask >> sa // What mask looks like before shift
                };
                if newmask == 0 {
                    // Subvariable filled with shifted zero
                    let rop = self.create_op(data, OpCode::CPUI_COPY, 1, rvn);
                    self.add_new_constant(rop, 0, 0u64);
                    return Ok(true);
                }
                if (newmask << sa) == mask {
                    let rop = self.create_op(data, OpCode::CPUI_COPY, 1, rvn);
                    let i0 = data.obank().get(op).expect("stale op").get_in(0).expect("in0");
                    if !self.create_link(data, rop, newmask, 0, i0)? {
                        return Ok(false);
                    }
                    return Ok(true);
                }
                if (mask & 1) == 0 {
                    return Ok(false); // Can't assume zeroes are shifted into least sig bits
                }
                let rop = self.create_op(data, OpCode::CPUI_INT_LEFT, 2, rvn);
                let i0 = data.obank().get(op).expect("stale op").get_in(0).expect("in0");
                if !self.create_link(data, rop, mask, 0, i0)? {
                    return Ok(false);
                }
                let i1size = data.vbank().get(i1).expect("vn").get_size();
                self.add_constant(data, Some(rop), calc_mask(i1size), 1, i1); // Preserve the shift amount
                Ok(true)
            }
            OpCode::CPUI_INT_RIGHT => {
                let i1 = data.obank().get(op).expect("stale op").get_in(1).expect("in1");
                if !data.vbank().get(i1).expect("vn").is_constant() {
                    return Ok(false); // break -- Dynamic shift
                }
                let sa = data.vbank().get(i1).expect("vn").get_offset() as int4;
                if sa as u64 >= (std::mem::size_of::<uintb>() as u64) * 8 {
                    return Ok(false); // break -- Beyond precision of mask
                }
                let i0 = data.obank().get(op).expect("stale op").get_in(0).expect("in0");
                let i0size = data.vbank().get(i0).expect("vn").get_size();
                let newmask = (mask << sa) & calc_mask(i0size);
                if newmask == 0 {
                    // Subvariable filled with shifted zero
                    let rop = self.create_op(data, OpCode::CPUI_COPY, 1, rvn);
                    self.add_new_constant(rop, 0, 0u64);
                    return Ok(true);
                }
                if (newmask >> sa) != mask {
                    return Ok(false); // break -- subvariable is truncated by shift
                }
                let rop = self.create_op(data, OpCode::CPUI_COPY, 1, rvn);
                if !self.create_link(data, rop, newmask, 0, i0)? {
                    return Ok(false);
                }
                Ok(true)
            }
            OpCode::CPUI_INT_SRIGHT => {
                let i1 = data.obank().get(op).expect("stale op").get_in(1).expect("in1");
                if !data.vbank().get(i1).expect("vn").is_constant() {
                    return Ok(false); // break -- Dynamic shift
                }
                let sa = data.vbank().get(i1).expect("vn").get_offset() as int4;
                if sa as u64 >= (std::mem::size_of::<uintb>() as u64) * 8 {
                    return Ok(false); // break -- Beyond precision of mask
                }
                let i0 = data.obank().get(op).expect("stale op").get_in(0).expect("in0");
                let i0size = data.vbank().get(i0).expect("vn").get_size();
                let newmask = (mask << sa) & calc_mask(i0size);
                if (newmask >> sa) != mask {
                    return Ok(false); // break -- subvariable is truncated by shift
                }
                let rop = self.create_op(data, OpCode::CPUI_COPY, 1, rvn);
                if !self.create_link(data, rop, newmask, 0, i0)? {
                    return Ok(false);
                }
                Ok(true)
            }
            OpCode::CPUI_INT_MULT => {
                let sa = leastsigbit_set(mask);
                if sa != 0 {
                    let i1 = data.obank().get(op).expect("stale op").get_in(1).expect("in1");
                    let sa2 = leastsigbit_set(data.vbank().get(i1).expect("vn").get_nz_mask());
                    if sa2 < sa {
                        return Ok(false); // Cannot deal with carries into logical multiply
                    }
                    let newmask = mask >> sa;
                    let rop = self.create_op(data, OpCode::CPUI_INT_MULT, 2, rvn);
                    let i0 = data.obank().get(op).expect("stale op").get_in(0).expect("in0");
                    if !self.create_link(data, rop, newmask, 0, i0)? {
                        return Ok(false);
                    }
                    if !self.create_link(data, rop, mask, 1, i1)? {
                        return Ok(false);
                    }
                } else {
                    let rop = if mask == 1u64 {
                        self.create_op(data, OpCode::CPUI_INT_AND, 2, rvn) // Single bit multiply
                    } else {
                        self.create_op(data, OpCode::CPUI_INT_MULT, 2, rvn)
                    };
                    let i0 = data.obank().get(op).expect("stale op").get_in(0).expect("in0");
                    let i1 = data.obank().get(op).expect("stale op").get_in(1).expect("in1");
                    if !self.create_link(data, rop, mask, 0, i0)? {
                        return Ok(false);
                    }
                    if !self.create_link(data, rop, mask, 1, i1)? {
                        return Ok(false);
                    }
                }
                Ok(true)
            }
            OpCode::CPUI_INT_DIV | OpCode::CPUI_INT_REM => {
                if (mask & 1) == 0 {
                    return Ok(false);
                }
                if (self.bitsize & 7) != 0 {
                    return Ok(false); // Must be a whole number of bytes
                }
                let i0 = data.obank().get(op).expect("stale op").get_in(0).expect("in0");
                let i1 = data.obank().get(op).expect("stale op").get_in(1).expect("in1");
                if !Self::is_zero_extended(data, i0, self.flowsize)? {
                    return Ok(false);
                }
                if !Self::is_zero_extended(data, i1, self.flowsize)? {
                    return Ok(false);
                }
                let rop = self.create_op(data, code, 2, rvn);
                if !self.create_link(data, rop, mask, 0, i0)? {
                    return Ok(false);
                }
                if !self.create_link(data, rop, mask, 1, i1)? {
                    return Ok(false);
                }
                Ok(true)
            }
            OpCode::CPUI_SUBPIECE => {
                let i1 = data.obank().get(op).expect("stale op").get_in(1).expect("in1");
                let sa = (data.vbank().get(i1).expect("vn").get_offset() as int4) * 8;
                let newmask = mask << sa;
                let rop = self.create_op(data, OpCode::CPUI_COPY, 1, rvn);
                let i0 = data.obank().get(op).expect("stale op").get_in(0).expect("in0");
                if !self.create_link(data, rop, newmask, 0, i0)? {
                    return Ok(false);
                }
                Ok(true)
            }
            OpCode::CPUI_PIECE => {
                let i1 = data.obank().get(op).expect("stale op").get_in(1).expect("in1");
                let i1size = data.vbank().get(i1).expect("vn").get_size();
                if (mask & calc_mask(i1size)) == mask {
                    let rop = self.create_op(data, OpCode::CPUI_COPY, 1, rvn);
                    if !self.create_link(data, rop, mask, 0, i1)? {
                        return Ok(false);
                    }
                    return Ok(true);
                }
                let sa = i1size * 8;
                let newmask = mask >> sa;
                if (newmask << sa) == mask {
                    let rop = self.create_op(data, OpCode::CPUI_COPY, 1, rvn);
                    let i0 = data.obank().get(op).expect("stale op").get_in(0).expect("in0");
                    if !self.create_link(data, rop, newmask, 0, i0)? {
                        return Ok(false);
                    }
                    return Ok(true);
                }
                Ok(false) // break
            }
            OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => {
                if self.try_call_return_push(data, op, rvn)? {
                    return Ok(true);
                }
                Ok(false) // break
            }
            OpCode::CPUI_INT_EQUAL
            | OpCode::CPUI_INT_NOTEQUAL
            | OpCode::CPUI_INT_SLESS
            | OpCode::CPUI_INT_SLESSEQUAL
            | OpCode::CPUI_INT_LESS
            | OpCode::CPUI_INT_LESSEQUAL
            | OpCode::CPUI_INT_CARRY
            | OpCode::CPUI_INT_SCARRY
            | OpCode::CPUI_INT_SBORROW
            | OpCode::CPUI_BOOL_NEGATE
            | OpCode::CPUI_BOOL_XOR
            | OpCode::CPUI_BOOL_AND
            | OpCode::CPUI_BOOL_OR
            | OpCode::CPUI_FLOAT_EQUAL
            | OpCode::CPUI_FLOAT_NOTEQUAL
            | OpCode::CPUI_FLOAT_LESSEQUAL
            | OpCode::CPUI_FLOAT_NAN => {
                // Mask won't be 1, because setReplacement takes care of it
                if (mask & 1) == 1 {
                    return Ok(false); // break -- Not normal variable flow
                }
                // Variable is filled with zero
                let rop = self.create_op(data, OpCode::CPUI_COPY, 1, rvn);
                self.add_new_constant(rop, 0, 0u64);
                Ok(true)
            }
            _ => Ok(false), // Everything else we abort
        }
    }

    // -------------------------------------------------------------------------
    // traceForwardSext (subflow.cc:867-954)
    // -------------------------------------------------------------------------

    /// Trace logical data-flow forward assuming sign-extensions
    /// (C++ `SubvariableFlow::traceForwardSext`).
    fn trace_forward_sext(&mut self, data: &mut Funcdata, rvn: RvId) -> KunaResult<bool> {
        let mut dcount = 0i32;
        let mut hcount = 0i32;
        let mut callcount = 0i32;

        let rvn_vn = self.rv(rvn).vn.expect("trace_forward_sext: rvn vn");
        let descend: Vec<OpId> = data.descend_snapshot(rvn_vn);
        for (idx, &op) in descend.iter().enumerate() {
            let outvn = data.obank().get(op).expect("stale op").get_out();
            let is_call = data.obank().get(op).expect("stale op").is_call();
            if let Some(ov) = outvn {
                if data.vbank().get(ov).expect("vn").is_mark() && !is_call {
                    continue;
                }
            }
            dcount += 1;
            let mut slot = data.obank().get(op).expect("stale op").get_slot(rvn_vn);
            let code = data.obank().get(op).expect("stale op").code();
            let mask = self.rv(rvn).mask;
            match code {
                OpCode::CPUI_COPY
                | OpCode::CPUI_MULTIEQUAL
                | OpCode::CPUI_INT_NEGATE
                | OpCode::CPUI_INT_XOR
                | OpCode::CPUI_INT_OR
                | OpCode::CPUI_INT_AND => {
                    let numin = data.obank().get(op).expect("stale op").num_input();
                    let rop = self.create_op_down(code, numin, op, rvn, slot);
                    if !self.create_link(data, rop, mask, -1, outvn.expect("outvn"))? {
                        return Ok(false);
                    }
                    hcount += 1;
                }
                OpCode::CPUI_INT_SEXT => {
                    // extended logical variable into even larger container
                    let rop = self.create_op_down(OpCode::CPUI_COPY, 1, op, rvn, 0);
                    if !self.create_link(data, rop, mask, -1, outvn.expect("outvn"))? {
                        return Ok(false);
                    }
                    hcount += 1;
                }
                OpCode::CPUI_INT_SRIGHT => {
                    let i1 = data.obank().get(op).expect("stale op").get_in(1).expect("in1");
                    if !data.vbank().get(i1).expect("vn").is_constant() {
                        return Ok(false); // Right now we only deal with constant shifts
                    }
                    let rop = self.create_op_down(OpCode::CPUI_INT_SRIGHT, 2, op, rvn, 0);
                    if !self.create_link(data, rop, mask, -1, outvn.expect("outvn"))? {
                        return Ok(false); // Keep the same mask size
                    }
                    let i1size = data.vbank().get(i1).expect("vn").get_size();
                    self.add_constant(data, Some(rop), calc_mask(i1size), 1, i1); // Preserve the shift amount
                    hcount += 1;
                }
                OpCode::CPUI_SUBPIECE => {
                    let i1 = data.obank().get(op).expect("stale op").get_in(1).expect("in1");
                    if data.vbank().get(i1).expect("vn").get_offset() != 0 {
                        return Ok(false); // Only allow proper truncation
                    }
                    let ov = outvn.expect("outvn");
                    let outsize = data.vbank().get(ov).expect("vn").get_size();
                    if outsize > self.flowsize {
                        return Ok(false);
                    }
                    if outsize == self.flowsize {
                        self.add_terminal_patch(op, rvn); // convert SUBPIECE to COPY
                    } else {
                        self.add_terminal_patch_same_op(op, rvn, 0); // SUBPIECE truncates even more
                    }
                    hcount += 1;
                }
                OpCode::CPUI_INT_LESS
                | OpCode::CPUI_INT_LESSEQUAL
                | OpCode::CPUI_INT_SLESS
                | OpCode::CPUI_INT_SLESSEQUAL
                | OpCode::CPUI_INT_EQUAL
                | OpCode::CPUI_INT_NOTEQUAL => {
                    let othervn =
                        data.obank().get(op).expect("stale op").get_in(1 - slot).expect("in"); // The OTHER side
                    if !self.create_compare_bridge(data, op, rvn, slot, othervn)? {
                        return Ok(false);
                    }
                    hcount += 1;
                }
                OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => {
                    callcount += 1;
                    if callcount > 1 {
                        let o = data.obank().get(op).expect("stale op");
                        slot = o.get_repeat_slot(rvn_vn, slot, &descend[..idx], op);
                    }
                    if !self.try_call_pull(data, op, rvn, slot)? {
                        return Ok(false);
                    }
                    hcount += 1;
                }
                OpCode::CPUI_RETURN => {
                    if !self.try_return_pull(data, op, rvn, slot)? {
                        return Ok(false);
                    }
                    hcount += 1;
                }
                OpCode::CPUI_BRANCHIND => {
                    if !self.try_switch_pull(data, op, rvn) {
                        return Ok(false);
                    }
                    hcount += 1;
                }
                _ => {
                    return Ok(false);
                }
            }
        }
        if dcount != hcount {
            if data.vbank().get(rvn_vn).expect("vn").is_input() {
                return Ok(false);
            }
        }
        Ok(true)
    }

    // -------------------------------------------------------------------------
    // traceBackwardSext (subflow.cc:960-1009)
    // -------------------------------------------------------------------------

    /// Trace logical data-flow backward assuming sign-extensions
    /// (C++ `SubvariableFlow::traceBackwardSext`).
    fn trace_backward_sext(&mut self, data: &mut Funcdata, rvn: RvId) -> KunaResult<bool> {
        let rvn_vn = self.rv(rvn).vn.expect("trace_backward_sext: rvn vn");
        let op = match data.vbank().get(rvn_vn).expect("vn").get_def() {
            Some(o) => o,
            None => return Ok(true), // If vn is input
        };
        let code = data.obank().get(op).expect("stale op").code();
        let mask = self.rv(rvn).mask;

        match code {
            OpCode::CPUI_COPY
            | OpCode::CPUI_MULTIEQUAL
            | OpCode::CPUI_INT_NEGATE
            | OpCode::CPUI_INT_XOR
            | OpCode::CPUI_INT_AND
            | OpCode::CPUI_INT_OR => {
                let numin = data.obank().get(op).expect("stale op").num_input();
                let rop = self.create_op(data, code, numin, rvn);
                for i in 0..numin {
                    let ini = data.obank().get(op).expect("stale op").get_in(i).expect("in");
                    if !self.create_link(data, rop, mask, i, ini)? {
                        return Ok(false);
                    }
                }
                Ok(true)
            }
            OpCode::CPUI_INT_ZEXT => {
                let i0 = data.obank().get(op).expect("stale op").get_in(0).expect("in0");
                if data.vbank().get(i0).expect("vn").get_size() < self.flowsize {
                    // zero extension from a smaller size still acts as a signed extension
                    self.add_push(data, op, rvn)?;
                    return Ok(true);
                }
                Ok(false) // break
            }
            OpCode::CPUI_INT_SEXT => {
                let i0 = data.obank().get(op).expect("stale op").get_in(0).expect("in0");
                if self.flowsize != data.vbank().get(i0).expect("vn").get_size() {
                    return Ok(false);
                }
                let rop = self.create_op(data, OpCode::CPUI_COPY, 1, rvn);
                if !self.create_link(data, rop, mask, 0, i0)? {
                    return Ok(false);
                }
                Ok(true)
            }
            OpCode::CPUI_INT_SRIGHT => {
                // A sign-extended logical value is arithmetically right-shifted
                let i1 = data.obank().get(op).expect("stale op").get_in(1).expect("in1");
                if !data.vbank().get(i1).expect("vn").is_constant() {
                    return Ok(false);
                }
                let rop = self.create_op(data, OpCode::CPUI_INT_SRIGHT, 2, rvn);
                let i0 = data.obank().get(op).expect("stale op").get_in(0).expect("in0");
                if !self.create_link(data, rop, mask, 0, i0)? {
                    return Ok(false); // Keep the same mask
                }
                if self.oplist[rop].input.len() == 1 {
                    let i1size = data.vbank().get(i1).expect("vn").get_size();
                    self.add_constant(data, Some(rop), calc_mask(i1size), 1, i1); // Preserve the shift amount
                }
                Ok(true)
            }
            OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => {
                if self.try_call_return_push(data, op, rvn)? {
                    return Ok(true);
                }
                Ok(false) // break
            }
            _ => Ok(false),
        }
    }

    // -------------------------------------------------------------------------
    // createLink / createCompareBridge (subflow.cc:1022-1071)
    // -------------------------------------------------------------------------

    /// Add a new variable to the logical subgraph as an input/output of the given
    /// operation (C++ `SubvariableFlow::createLink`).
    fn create_link(
        &mut self,
        data: &mut Funcdata,
        rop: RopId,
        mask: uintb,
        slot: int4,
        vn: VarnodeId,
    ) -> KunaResult<bool> {
        let (rep, inworklist) = self.set_replacement(data, vn, mask);
        let rep = match rep {
            Some(r) => r,
            None => return Ok(false),
        };

        // rop is never null in the create_link callers that pass a real op id; the
        // C++ `rop != 0` guard is always true here (the (ReplaceOp*)0 root case is
        // handled by the constructor calling create_link with a sentinel — see
        // SubvariableFlow::new, which calls create_link_root instead).
        if slot == -1 {
            self.oplist[rop].output = Some(rep);
            self.rv_mut(rep).def = Some(rop);
        } else {
            while (self.oplist[rop].input.len() as int4) <= slot {
                self.oplist[rop].input.push(None);
            }
            self.oplist[rop].input[slot as usize] = Some(rep);
        }

        if inworklist {
            self.worklist.push(rep);
        }
        Ok(true)
    }

    /// The constructor's `createLink((ReplaceOp *)0, mask, 0, root)` — the only
    /// call with a null `rop` (C++ `SubvariableFlow::createLink` with `rop==0`).
    fn create_link_root(&mut self, data: &mut Funcdata, mask: uintb, vn: VarnodeId) -> KunaResult<bool> {
        let (rep, inworklist) = self.set_replacement(data, vn, mask);
        let rep = match rep {
            Some(r) => r,
            None => return Ok(false),
        };
        // rop == 0 -> no input/output wiring
        if inworklist {
            self.worklist.push(rep);
        }
        Ok(true)
    }

    /// Extend the logical subgraph through a given comparison operator if possible
    /// (C++ `SubvariableFlow::createCompareBridge`).
    fn create_compare_bridge(
        &mut self,
        data: &mut Funcdata,
        op: OpId,
        inrvn: RvId,
        slot: int4,
        othervn: VarnodeId,
    ) -> KunaResult<bool> {
        let mask = self.rv(inrvn).mask;
        let (rep, inworklist) = self.set_replacement(data, othervn, mask);
        let rep = match rep {
            Some(r) => r,
            None => return Ok(false),
        };

        if slot == 0 {
            self.add_compare_patch(inrvn, rep, op);
        } else {
            self.add_compare_patch(rep, inrvn, op);
        }

        if inworklist {
            self.worklist.push(rep);
        }
        Ok(true)
    }

    // -------------------------------------------------------------------------
    // addConstant / addNewConstant / createNewOut (subflow.cc:1080-1143)
    // -------------------------------------------------------------------------

    /// Add a constant variable node to the logical subgraph
    /// (C++ `SubvariableFlow::addConstant`).
    fn add_constant(
        &mut self,
        data: &Funcdata,
        rop: Option<RopId>,
        mask: uintb,
        slot: uint4,
        constvn: VarnodeId,
    ) -> RvId {
        let constoff = data.vbank().get(constvn).expect("add_constant: stale const vn").get_offset();
        // Calculate the actual constant value
        let sa = leastsigbit_set(mask);
        let val = (mask & constoff) >> sa;
        let res = self.new_rv(ReplaceVarnode {
            vn: Some(constvn),
            replacement: None,
            mask,
            val,
            def: None,
        });
        if let Some(rop) = rop {
            while (self.oplist[rop].input.len() as uint4) <= slot {
                self.oplist[rop].input.push(None);
            }
            self.oplist[rop].input[slot as usize] = Some(res);
        }
        res
    }

    /// Add a new constant variable node as an input to a logical operation
    /// (C++ `SubvariableFlow::addNewConstant`).
    fn add_new_constant(&mut self, rop: RopId, slot: uint4, val: uintb) -> RvId {
        let res = self.new_rv(ReplaceVarnode {
            vn: None,
            replacement: None,
            mask: 0,
            val,
            def: None,
        });
        while (self.oplist[rop].input.len() as uint4) <= slot {
            self.oplist[rop].input.push(None);
        }
        self.oplist[rop].input[slot as usize] = Some(res);
        res
    }

    /// Create a new, non-shadowing, subgraph variable node as an operation output
    /// (C++ `SubvariableFlow::createNewOut`).
    fn create_new_out(&mut self, rop: RopId, mask: uintb) {
        let res = self.new_rv(ReplaceVarnode {
            vn: None,
            replacement: None,
            mask,
            val: 0,
            def: Some(rop),
        });
        self.oplist[rop].output = Some(res);
    }

    // -------------------------------------------------------------------------
    // patch builders (subflow.cc:1151-1250)
    // -------------------------------------------------------------------------

    /// Mark an operation where original data-flow is being pushed into a subgraph
    /// variable (C++ `SubvariableFlow::addPush`).  Pushed to the *front* of the
    /// patch list.
    fn add_push(&mut self, _data: &Funcdata, push_op: OpId, rvn: RvId) -> KunaResult<()> {
        self.patchlist.insert(
            0,
            PatchRecord {
                typ: PatchType::PushPatch,
                patch_op: push_op,
                in1: rvn,
                in2: None,
                slot: 0,
            },
        );
        Ok(())
    }

    /// Mark an operation where a subgraph variable is naturally copied into the
    /// original data-flow (C++ `SubvariableFlow::addTerminalPatch`).
    fn add_terminal_patch(&mut self, pullop: OpId, rvn: RvId) {
        self.patchlist.push(PatchRecord {
            typ: PatchType::CopyPatch, // Ultimately gets converted to a COPY
            patch_op: pullop,
            in1: rvn,
            in2: None,
            slot: 0,
        });
        self.pullcount += 1; // a true terminal modification
    }

    /// Mark an operation where a subgraph variable is naturally pulled into the
    /// original data-flow (C++ `SubvariableFlow::addTerminalPatchSameOp`).
    fn add_terminal_patch_same_op(&mut self, pullop: OpId, rvn: RvId, slot: int4) {
        self.patchlist.push(PatchRecord {
            typ: PatchType::ParameterPatch, // Keep the original op, just change input
            patch_op: pullop,
            in1: rvn,
            in2: None,
            slot,
        });
        self.pullcount += 1; // a true terminal modification
    }

    /// Mark a subgraph bit variable flowing into an operation taking a boolean
    /// input (C++ `SubvariableFlow::addBooleanPatch`).
    fn add_boolean_patch(&mut self, pullop: OpId, rvn: RvId, slot: int4) {
        self.patchlist.push(PatchRecord {
            typ: PatchType::ParameterPatch, // Make no change to the operator, just put in the new input
            patch_op: pullop,
            in1: rvn,
            in2: None,
            slot,
        });
        // this is not a true modification
    }

    /// Mark a subgraph variable flowing to an operation that extends it by padding
    /// with zero bits (C++ `SubvariableFlow::addExtensionPatch`).
    fn add_extension_patch(&mut self, rvn: RvId, pushop: OpId, sa: int4) {
        let slot = if sa == -1 { leastsigbit_set(self.rv(rvn).mask) } else { sa };
        self.patchlist.push(PatchRecord {
            typ: PatchType::ExtensionPatch,
            patch_op: pushop,
            in1: rvn,
            in2: None,
            slot,
        });
        // This is not a true modification because the output is still the expanded size
    }

    /// Mark subgraph variables flowing into a comparison operation
    /// (C++ `SubvariableFlow::addComparePatch`).
    fn add_compare_patch(&mut self, in1: RvId, in2: RvId, op: OpId) {
        self.patchlist.push(PatchRecord {
            typ: PatchType::ComparePatch,
            patch_op: op,
            in1,
            in2: Some(in2),
            slot: 0,
        });
        self.pullcount += 1;
    }

    // -------------------------------------------------------------------------
    // replacement geometry (subflow.cc:1258-1345) — W3-funcdata mutation seams
    // -------------------------------------------------------------------------

    /// Decide if we use the same memory range of the original Varnode for the
    /// logical replacement (C++ `SubvariableFlow::useSameAddress`).
    ///
    /// Used only by `getReplaceVarnode` inside the seam-gated
    /// [`SubvariableFlow::do_replacement`].  // SEAM(W3-funcdata)
    #[allow(dead_code)]
    fn use_same_address(&self, data: &Funcdata, rvn: RvId) -> bool {
        let vn = self.rv(rvn).vn.expect("use_same_address: rvn vn");
        let v = data.vbank().get(vn).expect("vn");
        if v.is_input() {
            return true;
        }
        // If we trim an addrtied varnode ... we increase chance of conflicting forms
        if v.is_addr_tied() {
            return false;
        }
        if (self.rv(rvn).mask & 1) == 0 {
            return false; // Not aligned
        }
        if self.bitsize >= 8 {
            return true;
        }
        if self.aggressive {
            return true;
        }
        let mut bitmask: uint4 = 1;
        // Try to decide if this is the ONLY subvariable passing through this container
        bitmask = (bitmask << self.bitsize) - 1;
        let mut mask = v.get_consume();
        mask |= bitmask as uintb;
        if mask == self.rv(rvn).mask {
            return true;
        }
        false // If more of the varnode is consumed than is in just this flow
    }

    /// Calculate address of replacement Varnode for given subgraph variable node
    /// (C++ `SubvariableFlow::getReplacementAddress`).
    ///
    /// Used only by `getReplaceVarnode` inside the seam-gated
    /// [`SubvariableFlow::do_replacement`].  // SEAM(W3-funcdata)
    #[allow(dead_code)]
    fn get_replacement_address(&self, data: &Funcdata, rvn: RvId) -> KunaResult<Address> {
        let vn = self.rv(rvn).vn.expect("get_replacement_address: rvn vn");
        let v = data.vbank().get(vn).expect("vn");
        let base = v.get_addr().clone();
        let vsize = v.get_size();
        let sa = leastsigbit_set(self.rv(rvn).mask) / 8; // Number of bytes value is shifted into container
        // `Add<i64>` is implemented for `&Address` (kuna-base address.rs).
        let mut addr = if base.is_big_endian() {
            &base + ((vsize - self.flowsize - sa) as i64)
        } else {
            &base + (sa as i64)
        };
        addr.renormalize(self.flowsize, data.get_arch().manage())?;
        Ok(addr)
    }

    // -------------------------------------------------------------------------
    // doTrace / processNextWork (subflow.cc:1351-1433)
    // -------------------------------------------------------------------------

    /// Extend the subgraph from the next node in the worklist
    /// (C++ `SubvariableFlow::processNextWork`).
    fn process_next_work(&mut self, data: &mut Funcdata) -> KunaResult<bool> {
        let rvn = self.worklist.pop().expect("process_next_work: empty worklist");

        if self.sextrestrictions {
            if !self.trace_backward_sext(data, rvn)? {
                return Ok(false);
            }
            return self.trace_forward_sext(data, rvn);
        }
        if !self.trace_backward(data, rvn)? {
            return Ok(false);
        }
        self.trace_forward(data, rvn)
    }

    /// Construct the engine for a function/root/mask (C++ `SubvariableFlow::SubvariableFlow`).
    ///
    /// \param root is a starting Varnode containing a smaller logical value
    /// \param mask bits indicating the logical value within \e root
    /// \param aggr aggressive (less restrictive) tests
    /// \param sext assume sign extensions from the logical value into its container
    /// \param big look for subvariable flow for \e big (8-byte) logical values
    pub fn new(
        data: &mut Funcdata,
        root: VarnodeId,
        mask: uintb,
        aggr: bool,
        sext: bool,
        big: bool,
    ) -> KunaResult<SubvariableFlow> {
        let mut sf = SubvariableFlow {
            flowsize: 0,
            bitsize: 0,
            returns_traversed: false,
            aggressive: aggr,
            sextrestrictions: sext,
            valid: false,
            varmap: BTreeMap::new(),
            rvarena: Vec::new(),
            oplist: Vec::new(),
            patchlist: Vec::new(),
            worklist: Vec::new(),
            pullcount: 0,
        };
        if mask == 0u64 {
            // fd = 0; return;  -- invalid engine
            return Ok(sf);
        }
        sf.bitsize = (mostsigbit_set(mask) - leastsigbit_set(mask)) + 1;
        if sf.bitsize <= 8 {
            sf.flowsize = 1;
        } else if sf.bitsize <= 16 {
            sf.flowsize = 2;
        } else if sf.bitsize <= 24 {
            sf.flowsize = 3;
        } else if sf.bitsize <= 32 {
            sf.flowsize = 4;
        } else if sf.bitsize <= 64 {
            if !big {
                return Ok(sf); // fd = 0; return;
            }
            sf.flowsize = 8;
        } else {
            return Ok(sf); // fd = 0; return;
        }
        sf.valid = true;
        sf.create_link_root(data, mask, root)?;
        Ok(sf)
    }

    /// Trace logical value through data-flow, constructing transform
    /// (C++ `SubvariableFlow::doTrace`).
    pub fn do_trace(&mut self, data: &mut Funcdata) -> KunaResult<bool> {
        self.pullcount = 0;
        let mut retval = false;
        if self.valid {
            retval = true;
            while !self.worklist.is_empty() {
                if !self.process_next_work(data)? {
                    retval = false;
                    break;
                }
            }
        }

        // Clear marks
        let marked: Vec<VarnodeId> = self.varmap.keys().copied().collect();
        for vn in marked {
            if let Some(v) = data.vbank_mut().get_mut(vn) {
                v.clear_mark();
            }
        }

        if !retval {
            return Ok(false);
        }
        if self.pullcount == 0 {
            return Ok(false);
        }
        Ok(true)
    }

    /// Perform the discovered transform, making logical values explicit
    /// (C++ `SubvariableFlow::doReplacement`).
    ///
    /// SEAM(W3-funcdata)/SEAM(W6): every `fd->opSet*`/`newOp`/`newVarnode*` here
    /// routes through `Funcdata::opSetOutput` (the unported `(vbank,obank)`
    /// split-borrow) and opcode→`TypeOp` resolution (`glb->inst[opc]`).  The
    /// patch *order* (call-return push patches first, then define outputs, then
    /// inputs, then the terminal patches in list order) is transcribed; the
    /// concrete mutations return the foundation's seam error until those land.
    pub fn do_replacement(&mut self, _data: &mut Funcdata) -> KunaResult<()> {
        Err(KunaError::lowlevel(
            "kuna rust port: SubvariableFlow::doReplacement needs Funcdata::opSetOutput \
             ((vbank,obank) split-borrow, W3-funcdata) and opcode->TypeOp resolution \
             (glb->inst[opc], W6); discovery (doTrace) is fully ported — SEAM(W3-funcdata)/SEAM(W6)",
        ))
    }

    /// `vn->isZeroExtended(flowsize)` (C++ `Varnode::isZeroExtended`).
    ///
    /// SEAM(W3-varnode): `Varnode::isZeroExtended` is not yet on the W3 varnode
    /// surface.  The INT_DIV/INT_REM trace paths that call it are reachable only
    /// for whole-byte divides; until the accessor lands this seam-gates them.
    fn is_zero_extended(_data: &Funcdata, _vn: VarnodeId, _flowsize: int4) -> KunaResult<bool> {
        Err(KunaError::lowlevel(
            "kuna rust port: Varnode::isZeroExtended not on the W3 varnode surface — SEAM(W3-varnode)",
        ))
    }
}

// =============================================================================
// Trigger rules (subflow.cc:1547-1746)
// =============================================================================

/// Perform SubVariableFlow analysis triggered by INT_AND (C++ `RuleSubvarAnd`).
pub struct RuleSubvarAnd {
    group: String,
}

impl RuleSubvarAnd {
    /// Construct with the rule's stage group (C++ `RuleSubvarAnd::RuleSubvarAnd`).
    pub fn new(g: impl Into<String>) -> RuleSubvarAnd {
        RuleSubvarAnd { group: g.into() }
    }
}

impl Rule for RuleSubvarAnd {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_INT_AND]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(&self.group) {
            return None;
        }
        Some(Box::new(RuleSubvarAnd::new(self.group.clone())))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        let o = data.obank().get(op).expect("stale op");
        let in1 = o.get_in(1).expect("in1");
        if !data.vbank().get(in1).expect("vn").is_constant() {
            return 0;
        }
        let vn = o.get_in(0).expect("in0");
        let outvn = o.get_out().expect("out");
        let in1off = data.vbank().get(in1).expect("vn").get_offset();
        let outconsume = data.vbank().get(outvn).expect("vn").get_consume();
        if outconsume != in1off {
            return 0;
        }
        if (outconsume & 1) == 0 {
            return 0;
        }
        let cmask: uintb;
        if outconsume == 1u64 {
            cmask = 1u64;
        } else {
            let mut m = calc_mask(data.vbank().get(vn).expect("vn").get_size());
            m >>= 8;
            loop {
                if m == 0 {
                    break;
                }
                if m == outconsume {
                    break;
                }
                m >>= 8;
            }
            cmask = m;
        }
        if cmask == 0 {
            return 0;
        }
        if data.vbank().get(outvn).expect("vn").has_no_descend() {
            return 0;
        }
        run_subflow(data, vn, cmask, false, false, false)
    }
}

/// Perform SubVariableFlow analysis triggered by SUBPIECE (C++ `RuleSubvarSubpiece`).
pub struct RuleSubvarSubpiece {
    group: String,
}

impl RuleSubvarSubpiece {
    /// Construct with the rule's stage group.
    pub fn new(g: impl Into<String>) -> RuleSubvarSubpiece {
        RuleSubvarSubpiece { group: g.into() }
    }
}

impl Rule for RuleSubvarSubpiece {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_SUBPIECE]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(&self.group) {
            return None;
        }
        Some(Box::new(RuleSubvarSubpiece::new(self.group.clone())))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        let o = data.obank().get(op).expect("stale op");
        let vn = o.get_in(0).expect("in0");
        let outvn = o.get_out().expect("out");
        let flowsize = data.vbank().get(outvn).expect("vn").get_size();
        let i1 = o.get_in(1).expect("in1");
        let sa = data.vbank().get(i1).expect("vn").get_offset() as int4;
        if (flowsize + sa) as u64 > std::mem::size_of::<uintb>() as u64 {
            // Mask must fit in precision
            return 0;
        }
        let mut mask = calc_mask(flowsize);
        mask <<= 8 * sa;
        let aggressive = data.vbank().get(outvn).expect("vn").is_ptr_flow();
        if !aggressive {
            let vnconsume = data.vbank().get(vn).expect("vn").get_consume();
            if (vnconsume & mask) != vnconsume {
                return 0;
            }
            if data.vbank().get(outvn).expect("vn").has_no_descend() {
                return 0;
            }
        }
        let mut big = false;
        if flowsize >= 8 && data.vbank().get(vn).expect("vn").is_input() {
            // Vector register inputs getting truncated to what actually gets used
            // TODO: ActionLaneDivide should be handling this
            if data.lone_descend(vn) == Some(op) {
                big = true;
            }
        }
        run_subflow(data, vn, mask, aggressive, false, big)
    }
}

/// Perform SubvariableFlow analysis triggered by testing of a single bit
/// (C++ `RuleSubvarCompZero`).
pub struct RuleSubvarCompZero {
    group: String,
}

impl RuleSubvarCompZero {
    /// Construct with the rule's stage group.
    pub fn new(g: impl Into<String>) -> RuleSubvarCompZero {
        RuleSubvarCompZero { group: g.into() }
    }
}

impl Rule for RuleSubvarCompZero {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_INT_NOTEQUAL, OpCode::CPUI_INT_EQUAL]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(&self.group) {
            return None;
        }
        Some(Box::new(RuleSubvarCompZero::new(self.group.clone())))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        let o = data.obank().get(op).expect("stale op");
        let in1 = o.get_in(1).expect("in1");
        if !data.vbank().get(in1).expect("vn").is_constant() {
            return 0;
        }
        let vn = o.get_in(0).expect("in0");
        let mask = data.vbank().get(vn).expect("vn").get_nz_mask();
        let bitnum = leastsigbit_set(mask);
        if bitnum == -1 {
            return 0;
        }
        if (mask >> bitnum) != 1 {
            return 0; // Check if only one bit active
        }

        // Check if the active bit is getting tested
        let in1off = data.vbank().get(in1).expect("vn").get_offset();
        if (in1off != mask) && (in1off != 0) {
            return 0;
        }

        let outvn = o.get_out().expect("out");
        if data.vbank().get(outvn).expect("vn").has_no_descend() {
            return 0;
        }
        // We do a basic check that the stream from which it looks like
        // the bit is getting pulled is not fully consumed
        if data.vbank().get(vn).expect("vn").is_written() {
            let andop = data.vbank().get(vn).expect("vn").get_def().expect("written vn def");
            let ao = data.obank().get(andop).expect("stale def");
            if ao.num_input() == 0 {
                return 0;
            }
            let vn0 = ao.get_in(0).expect("in0");
            match ao.code() {
                OpCode::CPUI_INT_AND | OpCode::CPUI_INT_OR | OpCode::CPUI_INT_RIGHT => {
                    if data.vbank().get(vn0).expect("vn").is_constant() {
                        return 0;
                    }
                    let mask0 = data.vbank().get(vn0).expect("vn").get_consume()
                        & data.vbank().get(vn0).expect("vn").get_nz_mask();
                    let vn0size = data.vbank().get(vn0).expect("vn").get_size();
                    let wholemask = calc_mask(vn0size) & mask0;
                    // We really need a popcnt here ...
                    if (wholemask & 0xff) == 0xff {
                        return 0;
                    }
                    if (wholemask & 0xff00) == 0xff00 {
                        return 0;
                    }
                }
                _ => {}
            }
        }

        run_subflow(data, vn, mask, false, false, false)
    }
}

/// Perform SubVariableFlow analysis triggered by INT_RIGHT (C++ `RuleSubvarShift`).
pub struct RuleSubvarShift {
    group: String,
}

impl RuleSubvarShift {
    /// Construct with the rule's stage group.
    pub fn new(g: impl Into<String>) -> RuleSubvarShift {
        RuleSubvarShift { group: g.into() }
    }
}

impl Rule for RuleSubvarShift {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_INT_RIGHT]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(&self.group) {
            return None;
        }
        Some(Box::new(RuleSubvarShift::new(self.group.clone())))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        let o = data.obank().get(op).expect("stale op");
        let vn = o.get_in(0).expect("in0");
        if data.vbank().get(vn).expect("vn").get_size() != 1 {
            return 0;
        }
        let in1 = o.get_in(1).expect("in1");
        if !data.vbank().get(in1).expect("vn").is_constant() {
            return 0;
        }
        let sa = data.vbank().get(in1).expect("vn").get_offset() as int4;
        let mut mask = data.vbank().get(vn).expect("vn").get_nz_mask();
        if (mask >> sa) != 1u64 {
            return 0; // Pulling out a single bit
        }
        mask = (mask >> sa) << sa;
        let outvn = o.get_out().expect("out");
        if data.vbank().get(outvn).expect("vn").has_no_descend() {
            return 0;
        }

        run_subflow(data, vn, mask, false, false, false)
    }
}

/// Perform SubVariableFlow analysis triggered by INT_ZEXT (C++ `RuleSubvarZext`).
pub struct RuleSubvarZext {
    group: String,
}

impl RuleSubvarZext {
    /// Construct with the rule's stage group.
    pub fn new(g: impl Into<String>) -> RuleSubvarZext {
        RuleSubvarZext { group: g.into() }
    }
}

impl Rule for RuleSubvarZext {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_INT_ZEXT]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(&self.group) {
            return None;
        }
        Some(Box::new(RuleSubvarZext::new(self.group.clone())))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        let o = data.obank().get(op).expect("stale op");
        let vn = o.get_out().expect("out");
        let invn = o.get_in(0).expect("in0");
        let invsize = data.vbank().get(invn).expect("vn").get_size();
        let mask = calc_mask(invsize);
        let ptrflow = data.vbank().get(invn).expect("vn").is_ptr_flow();

        run_subflow(data, vn, mask, ptrflow, false, false)
    }
}

/// Perform SubVariableFlow analysis triggered by INT_SEXT (C++ `RuleSubvarSext`).
pub struct RuleSubvarSext {
    group: String,
    /// Is it guaranteed the root is a sub-variable needing to be trimmed
    /// (C++ `isaggressive`).
    isaggressive: int4,
}

impl RuleSubvarSext {
    /// Construct with the rule's stage group (C++ sets `isaggressive = false`).
    pub fn new(g: impl Into<String>) -> RuleSubvarSext {
        RuleSubvarSext { group: g.into(), isaggressive: 0 }
    }
}

impl Rule for RuleSubvarSext {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_INT_SEXT]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(&self.group) {
            return None;
        }
        Some(Box::new(RuleSubvarSext::new(self.group.clone())))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        let o = data.obank().get(op).expect("stale op");
        let vn = o.get_out().expect("out");
        let invn = o.get_in(0).expect("in0");
        let invsize = data.vbank().get(invn).expect("vn").get_size();
        let mask = calc_mask(invsize);

        run_subflow(data, vn, mask, self.isaggressive != 0, true, false)
    }

    fn reset(&mut self, _data: &mut Funcdata) {
        // isaggressive = data.getArch()->aggressive_ext_trim;
        // SEAM(W4): Architecture::aggressive_ext_trim is not on the W3 arch seam
        // (Funcdata::getArch returns the seams::Architecture skeleton).  The C++
        // default is `false`; we keep that until the W4 arch surface lands.
        self.isaggressive = 0;
    }
}

/// Run the SubvariableFlow trace+replacement for a trigger rule (C++ idiom:
/// `SubvariableFlow subflow(...); if (!subflow.doTrace()) return 0;
/// subflow.doReplacement(); return 1;`).
///
/// Returns `1` if the transform was constructed AND applied, `0` otherwise.
/// While `doReplacement` is seam-gated (W3-funcdata/W6), a successful
/// `doTrace` whose replacement defers reports `0` (no change made), which is the
/// conservative, datatest-safe behavior until the mutation seam lands.
fn run_subflow(
    data: &mut Funcdata,
    root: VarnodeId,
    mask: uintb,
    aggr: bool,
    sext: bool,
    big: bool,
) -> int4 {
    let mut subflow = match SubvariableFlow::new(data, root, mask, aggr, sext, big) {
        Ok(sf) => sf,
        Err(_) => return 0, // construction reached a seam (e.g. sext constant check)
    };
    match subflow.do_trace(data) {
        Ok(true) => {}
        _ => return 0,
    }
    // subflow.doReplacement(); return 1;
    match subflow.do_replacement(data) {
        Ok(()) => 1,
        Err(_) => 0, // SEAM(W3-funcdata)/SEAM(W6): replacement deferred -> no change
    }
}

// =============================================================================
// SplitFlow / RuleSplitFlow / LaneDivide — SEAM(W5-transform)
// =============================================================================
//
// SplitFlow (subflow.cc:1754-2088) and LaneDivide (subflow.cc:3533-4143) derive
// from `TransformManager` and operate on `TransformVar`/`TransformOp`/
// `LaneDescription`.  Those types are the *separate* checklist item
// `w5-s3-transform` (decompiler/cpp/transform.{cc,hh}), still a stub
// (`transform.rs`) and not owned by this file.  Porting SplitFlow/LaneDivide
// faithfully requires that surface (newSplit/getSplit/newOpReplace/opSetInput/
// opSetOutput/newConstant/newUnique/getPreexistingVarnode/restriction/extension/
// getBoundary/getPosition, and `TransformManager::apply`).
//
// They are therefore recorded as a SEAM here rather than transcribed against a
// nonexistent API.  RuleSplitFlow's *detection* head (subflow.cc:2045-2088 — the
// SUBPIECE-of-most-significant-part-through-PIECE-via-INDIRECT/MULTIEQUAL probe)
// is likewise withheld because its terminal action `SplitFlow::doTrace/apply`
// has no engine to run on yet.  When `w5-s3-transform` lands, SplitFlow,
// RuleSplitFlow, and LaneDivide should be ported in this file on top of it.
//
// SEAM(W5-transform): SplitFlow, RuleSplitFlow, LaneDivide.

// =============================================================================
// Rule registration (C++ definition order)
// =============================================================================

/// The rules defined in this file, in C++ definition order (W8 assembles
/// `universalAction` registration from these — see `action.rs` `RuleSpec`).
///
/// Order matches `subflow.hh`/`subflow.cc`: `subvar_and`, `subvar_subpiece`,
/// `subvar_compzero`, `subvar_shift`, `subvar_zext`, `subvar_sext`.  `splitflow`
/// is omitted pending SEAM(W5-transform).
pub fn specs() -> Vec<RuleSpec> {
    vec![
        RuleSpec { group: "subvar_and_grp", ctor: || Box::new(RuleSubvarAnd::new("subvar_and_grp")) },
        // NOTE: groups are placeholders; W8 supplies the real stage-group
        // strings from coreaction.cc when it assembles universalAction.  The
        // load-bearing facts here are the rule set, their order, and that each
        // rule keeps its exact name() and getOpList().
        RuleSpec {
            group: "subvar_subpiece_grp",
            ctor: || Box::new(RuleSubvarSubpiece::new("subvar_subpiece_grp")),
        },
        RuleSpec {
            group: "subvar_compzero_grp",
            ctor: || Box::new(RuleSubvarCompZero::new("subvar_compzero_grp")),
        },
        RuleSpec {
            group: "subvar_shift_grp",
            ctor: || Box::new(RuleSubvarShift::new("subvar_shift_grp")),
        },
        RuleSpec {
            group: "subvar_zext_grp",
            ctor: || Box::new(RuleSubvarZext::new("subvar_zext_grp")),
        },
        RuleSpec {
            group: "subvar_sext_grp",
            ctor: || Box::new(RuleSubvarSext::new("subvar_sext_grp")),
        },
    ]
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;

    use kuna_base::address::Address;
    use kuna_base::space::{
        addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, FspecSpace,
        IopSpace, UniqueSpace,
    };

    use crate::dtype::{type_metatype, Datatype};
    use crate::seams::{Architecture, TypeOp};
    use crate::varnode::DefOpInfo;

    // ---- scaffolding (mirrors funcdata_varnode.rs test scaffolding) --------

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

    fn build_fd() -> Funcdata {
        let manage = build_manager();
        let glb = Rc::new(Architecture::new(manage));
        let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
        let addr = Address::new(ram, 0x1000);
        Funcdata::new("func", "func", glb, addr, 0x10000000, 0x40).unwrap()
    }

    fn ram(fd: &Funcdata) -> Rc<AddrSpace> {
        Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap())
    }

    fn unk(size: int4) -> Rc<Datatype> {
        Rc::new(Datatype::new(size, type_metatype::TYPE_UNKNOWN))
    }

    fn no_replace(
    ) -> impl FnMut(&mut crate::varnode::VarnodeBank, VarnodeId, VarnodeId) -> KunaResult<()> {
        |_: &mut crate::varnode::VarnodeBank, _: VarnodeId, _: VarnodeId| -> KunaResult<()> {
            Ok(())
        }
    }

    /// Create an op with a given opcode at a (fresh) program address.
    fn mk_op(fd: &mut Funcdata, off: u64, ninput: int4, opc: OpCode) -> OpId {
        let r = ram(fd);
        let pc = Address::new(r, off);
        let op = fd.obank_mut().create_at(ninput, pc);
        fd.obank_mut().change_opcode(op, TypeOp::new(opc, 0, format!("{opc:?}")));
        op
    }

    /// A register varnode at a ram address (free / "free"-like until wired).
    fn mk_reg(fd: &mut Funcdata, off: u64, size: int4) -> VarnodeId {
        let r = ram(fd);
        fd.new_varnode(size, &Address::new(r, off), Some(unk(size)))
    }

    /// A register varnode promoted to a function input (so it isn't \e free, the
    /// abort condition in `setReplacement`).
    fn mk_input(fd: &mut Funcdata, off: u64, size: int4) -> VarnodeId {
        let vn = mk_reg(fd, off, size);
        fd.vbank_mut().set_input(vn, &mut no_replace()).unwrap()
    }

    /// Wire `vn` as input `slot` of `op` (descend list + op input).
    fn wire_in(fd: &mut Funcdata, op: OpId, vn: VarnodeId, slot: int4) {
        fd.vbank_mut().add_descend(vn, op).unwrap();
        fd.obank_mut().get_mut(op).unwrap().set_input(Some(vn), slot);
    }

    /// Make `vn` the output of `defop` (promote to written).
    fn wire_out(fd: &mut Funcdata, defop: OpId, vn: VarnodeId) -> VarnodeId {
        let seq = fd.obank().get(defop).unwrap().get_seq_num().clone();
        let info = DefOpInfo { id: defop, seqnum: seq };
        let vn = fd.vbank_mut().set_def(vn, info, &mut no_replace()).unwrap();
        fd.obank_mut().get_mut(defop).unwrap().set_output(Some(vn));
        vn
    }

    // ---- constructor / flowsize (subflow.cc:1372-1404) ---------------------

    #[test]
    fn ctor_flowsize_from_mask_width() {
        let mut fd = build_fd();
        let root = mk_reg(&mut fd, 0x40, 8);
        // 1 bit -> 1 byte
        let sf = SubvariableFlow::new(&mut fd, root, 0x1, false, false, false).unwrap();
        assert!(sf.valid);
        assert_eq!(sf.flowsize, 1);
        assert_eq!(sf.bitsize, 1);
        // byte mask 0xff -> 1 byte, 8 bits
        let sf = SubvariableFlow::new(&mut fd, root, 0xff, false, false, false).unwrap();
        assert_eq!(sf.flowsize, 1);
        assert_eq!(sf.bitsize, 8);
        // 0xffff -> 2 bytes
        let sf = SubvariableFlow::new(&mut fd, root, 0xffff, false, false, false).unwrap();
        assert_eq!(sf.flowsize, 2);
        // 0xffffff -> 3 bytes
        let sf = SubvariableFlow::new(&mut fd, root, 0xff_ffff, false, false, false).unwrap();
        assert_eq!(sf.flowsize, 3);
        // 0xffffffff -> 4 bytes
        let sf = SubvariableFlow::new(&mut fd, root, 0xffff_ffff, false, false, false).unwrap();
        assert_eq!(sf.flowsize, 4);
    }

    #[test]
    fn ctor_invalid_masks_yield_invalid_engine() {
        let mut fd = build_fd();
        let root = mk_reg(&mut fd, 0x40, 8);
        // mask == 0 -> invalid (the C++ `fd=0` sentinel)
        let sf = SubvariableFlow::new(&mut fd, root, 0x0, false, false, false).unwrap();
        assert!(!sf.valid);
        // 8-byte logical value, big=false -> invalid
        let sf = SubvariableFlow::new(&mut fd, root, u64::MAX, false, false, false).unwrap();
        assert!(!sf.valid);
        // 8-byte logical value, big=true -> valid, flowsize 8
        let sf = SubvariableFlow::new(&mut fd, root, u64::MAX, false, false, true).unwrap();
        assert!(sf.valid);
        assert_eq!(sf.flowsize, 8);
    }

    #[test]
    fn invalid_engine_do_trace_is_false() {
        let mut fd = build_fd();
        let root = mk_reg(&mut fd, 0x40, 8);
        let mut sf = SubvariableFlow::new(&mut fd, root, 0x0, false, false, false).unwrap();
        // Invalid engine: doTrace returns false (pullcount==0 / not valid).
        assert!(!sf.do_trace(&mut fd).unwrap());
    }

    // ---- doesOrSet / doesAndClear (subflow.cc:26-53) -----------------------

    #[test]
    fn does_or_set_detects_full_cover() {
        let mut fd = build_fd();
        let a = mk_reg(&mut fd, 0x40, 4);
        let c = fd.new_constant(4, 0xff); // sets low byte
        let orop = mk_op(&mut fd, 0x100, 2, OpCode::CPUI_INT_OR);
        wire_in(&mut fd, orop, a, 0);
        wire_in(&mut fd, orop, c, 1);
        // mask 0xff fully covered by the OR constant -> returns slot 1
        assert_eq!(SubvariableFlow::does_or_set(&fd, orop, 0xff), 1);
        // mask 0x1ff not fully covered -> -1
        assert_eq!(SubvariableFlow::does_or_set(&fd, orop, 0x1ff), -1);
    }

    #[test]
    fn does_and_clear_detects_full_clear() {
        let mut fd = build_fd();
        let a = mk_reg(&mut fd, 0x40, 4);
        let c = fd.new_constant(4, 0xffff_ff00); // clears low byte
        let andop = mk_op(&mut fd, 0x100, 2, OpCode::CPUI_INT_AND);
        wire_in(&mut fd, andop, a, 0);
        wire_in(&mut fd, andop, c, 1);
        // mask 0xff fully cleared by the AND constant -> slot 1
        assert_eq!(SubvariableFlow::does_and_clear(&fd, andop, 0xff), 1);
        // mask 0x100 not cleared -> -1
        assert_eq!(SubvariableFlow::does_and_clear(&fd, andop, 0x100), -1);
    }

    // ---- forward SUBPIECE terminal: sub-register pull (subflow.cc:528-546) --

    #[test]
    fn forward_subpiece_low_byte_is_terminal_copy() {
        // root: 4-byte reg; SUBPIECE(root, 0) -> 1-byte out.  The low-byte mask
        // flows to a SUBPIECE whose out size == flowsize and newmask&1 -> a
        // copy_patch terminal (pullcount bumped, doTrace succeeds).
        let mut fd = build_fd();
        let root = mk_input(&mut fd, 0x40, 4);
        fd.vbank_mut().get_mut(root).unwrap().set_consume(0xff); // only low byte consumed
        let outvn = mk_reg(&mut fd, 0x80, 1);
        let zero = fd.new_constant(4, 0);
        let sub = mk_op(&mut fd, 0x100, 2, OpCode::CPUI_SUBPIECE);
        wire_in(&mut fd, sub, root, 0);
        wire_in(&mut fd, sub, zero, 1);
        let outvn = wire_out(&mut fd, sub, outvn);
        let _ = outvn;

        // aggressive=true so set_replacement of the root doesn't bail on consume.
        let mut sf = SubvariableFlow::new(&mut fd, root, 0xff, true, false, false).unwrap();
        assert!(sf.valid);
        assert!(sf.do_trace(&mut fd).unwrap());
        // Exactly one terminal copy patch (the SUBPIECE pull).
        assert_eq!(sf.pullcount, 1);
        assert_eq!(sf.patchlist.len(), 1);
        assert_eq!(sf.patchlist[0].typ, PatchType::CopyPatch);
        assert_eq!(sf.patchlist[0].patch_op, sub);
        // Marks cleared after doTrace.
        assert!(!fd.vbank().get(root).unwrap().is_mark());
    }

    #[test]
    fn forward_subpiece_no_terminal_fails_trace() {
        // SUBPIECE that does not terminate (out size != flowsize, mask passes
        // through as a COPY link) leaves pullcount==0 -> doTrace returns false.
        let mut fd = build_fd();
        let root = mk_input(&mut fd, 0x40, 4);
        fd.vbank_mut().get_mut(root).unwrap().set_consume(0xffff);
        let outvn = mk_reg(&mut fd, 0x80, 2); // 2-byte out, flowsize would be 1
        let zero = fd.new_constant(4, 0);
        let sub = mk_op(&mut fd, 0x100, 2, OpCode::CPUI_SUBPIECE);
        wire_in(&mut fd, sub, root, 0);
        wire_in(&mut fd, sub, zero, 1);
        let _outvn = wire_out(&mut fd, sub, outvn);

        // logical value is the low byte (mask 0xff); SUBPIECE keeps 2 bytes so it
        // does not terminate here -> no terminal patch -> doTrace false.
        let mut sf = SubvariableFlow::new(&mut fd, root, 0xff, true, false, false).unwrap();
        assert!(!sf.do_trace(&mut fd).unwrap());
        assert_eq!(sf.pullcount, 0);
    }

    // ---- forward INT_AND mask-extract terminal (subflow.cc:411-424) --------

    #[test]
    fn forward_int_and_full_mask_is_terminal() {
        // root (4-byte) & 0xff -> out (1-byte, == flowsize).  The AND constant
        // equals the mask and outvn->getSize()==flowsize with mask&1 -> the
        // INT_AND terminal branch (subflow.cc:412-417) fires a copy_patch.
        let mut fd = build_fd();
        let root = mk_input(&mut fd, 0x40, 4);
        fd.vbank_mut().get_mut(root).unwrap().set_consume(0xff);
        let cst = fd.new_constant(4, 0xff);
        let outvn = mk_reg(&mut fd, 0x80, 1);
        let andop = mk_op(&mut fd, 0x100, 2, OpCode::CPUI_INT_AND);
        wire_in(&mut fd, andop, root, 0);
        wire_in(&mut fd, andop, cst, 1);
        let _outvn = wire_out(&mut fd, andop, outvn);

        let mut sf = SubvariableFlow::new(&mut fd, root, 0xff, true, false, false).unwrap();
        assert!(sf.do_trace(&mut fd).unwrap());
        assert_eq!(sf.pullcount, 1);
        assert_eq!(sf.patchlist.len(), 1);
        assert_eq!(sf.patchlist[0].typ, PatchType::CopyPatch);
    }

    // ---- backward trace: COPY def propagates the same mask -----------------

    #[test]
    fn backward_copy_then_forward_terminal() {
        // mid = COPY(src); SUBPIECE(mid,0) -> out(1 byte).  Tracing the low byte of
        // `mid` goes backward through the COPY (creating a COPY ReplaceOp shadowing
        // src) and forward through the SUBPIECE (terminal).  doTrace succeeds.
        let mut fd = build_fd();
        let src = mk_input(&mut fd, 0x40, 4);
        fd.vbank_mut().get_mut(src).unwrap().set_consume(0xff);
        // def op: mid = COPY(src)
        let copyop = mk_op(&mut fd, 0x100, 1, OpCode::CPUI_COPY);
        let mid = mk_reg(&mut fd, 0x80, 4);
        fd.vbank_mut().get_mut(mid).unwrap().set_consume(0xff);
        wire_in(&mut fd, copyop, src, 0);
        let mid = wire_out(&mut fd, copyop, mid);
        // forward use: out = SUBPIECE(mid, 0)
        let zero = fd.new_constant(4, 0);
        let sub = mk_op(&mut fd, 0x180, 2, OpCode::CPUI_SUBPIECE);
        let outvn = mk_reg(&mut fd, 0xc0, 1);
        wire_in(&mut fd, sub, mid, 0);
        wire_in(&mut fd, sub, zero, 1);
        let _outvn = wire_out(&mut fd, sub, outvn);

        let mut sf = SubvariableFlow::new(&mut fd, mid, 0xff, true, false, false).unwrap();
        assert!(sf.do_trace(&mut fd).unwrap());
        assert_eq!(sf.pullcount, 1);
        // A backward COPY ReplaceOp was created (shadowing the def COPY).
        assert!(sf.oplist.iter().any(|r| r.opc == OpCode::CPUI_COPY && r.op == Some(copyop)));
    }

    // ---- rule heads: detection guards return 0 cleanly ---------------------

    #[test]
    fn rule_subvar_and_rejects_nonconstant() {
        // RuleSubvarAnd::applyOp on (a & b) with non-constant b returns 0.
        let mut fd = build_fd();
        let a = mk_reg(&mut fd, 0x40, 4);
        let b = mk_reg(&mut fd, 0x48, 4);
        let andop = mk_op(&mut fd, 0x100, 2, OpCode::CPUI_INT_AND);
        let outvn = mk_reg(&mut fd, 0x80, 4);
        wire_in(&mut fd, andop, a, 0);
        wire_in(&mut fd, andop, b, 1);
        let _outvn = wire_out(&mut fd, andop, outvn);
        let mut rule = RuleSubvarAnd::new("g");
        assert_eq!(rule.apply_op(andop, &mut fd), 0);
    }

    #[test]
    fn rule_subvar_shift_requires_byte_input() {
        // RuleSubvarShift::applyOp on INT_RIGHT whose in0 is not 1 byte returns 0.
        let mut fd = build_fd();
        let a = mk_reg(&mut fd, 0x40, 4); // 4-byte, not 1
        let c = fd.new_constant(4, 1);
        let shr = mk_op(&mut fd, 0x100, 2, OpCode::CPUI_INT_RIGHT);
        let outvn = mk_reg(&mut fd, 0x80, 4);
        wire_in(&mut fd, shr, a, 0);
        wire_in(&mut fd, shr, c, 1);
        let _outvn = wire_out(&mut fd, shr, outvn);
        let mut rule = RuleSubvarShift::new("g");
        assert_eq!(rule.apply_op(shr, &mut fd), 0);
    }

    #[test]
    fn rule_get_op_lists_match_cpp() {
        assert_eq!(RuleSubvarAnd::new("g").get_op_list(), vec![OpCode::CPUI_INT_AND]);
        assert_eq!(
            RuleSubvarSubpiece::new("g").get_op_list(),
            vec![OpCode::CPUI_SUBPIECE]
        );
        assert_eq!(
            RuleSubvarCompZero::new("g").get_op_list(),
            vec![OpCode::CPUI_INT_NOTEQUAL, OpCode::CPUI_INT_EQUAL]
        );
        assert_eq!(RuleSubvarShift::new("g").get_op_list(), vec![OpCode::CPUI_INT_RIGHT]);
        assert_eq!(RuleSubvarZext::new("g").get_op_list(), vec![OpCode::CPUI_INT_ZEXT]);
        assert_eq!(RuleSubvarSext::new("g").get_op_list(), vec![OpCode::CPUI_INT_SEXT]);
    }

    #[test]
    fn specs_lists_six_rules_in_definition_order() {
        let s = specs();
        assert_eq!(s.len(), 6);
        // Each ctor builds a rule whose getOpList matches the C++ definition order.
        let lists: Vec<Vec<OpCode>> =
            s.iter().map(|sp| (sp.ctor)().get_op_list()).collect();
        assert_eq!(lists[0], vec![OpCode::CPUI_INT_AND]); // subvar_and
        assert_eq!(lists[1], vec![OpCode::CPUI_SUBPIECE]); // subvar_subpiece
        assert_eq!(
            lists[2],
            vec![OpCode::CPUI_INT_NOTEQUAL, OpCode::CPUI_INT_EQUAL]
        ); // subvar_compzero
        assert_eq!(lists[3], vec![OpCode::CPUI_INT_RIGHT]); // subvar_shift
        assert_eq!(lists[4], vec![OpCode::CPUI_INT_ZEXT]); // subvar_zext
        assert_eq!(lists[5], vec![OpCode::CPUI_INT_SEXT]); // subvar_sext
    }

    #[test]
    fn clone_rule_filters_by_group() {
        let rule = RuleSubvarAnd::new("mygroup");
        let yes = ActionGroupList::from_names(["mygroup"]);
        let no = ActionGroupList::from_names(["other"]);
        assert!(rule.clone_rule(&yes).is_some());
        assert!(rule.clone_rule(&no).is_none());
    }
}
