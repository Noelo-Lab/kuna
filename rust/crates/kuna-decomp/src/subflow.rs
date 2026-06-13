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
//!   * `SplitFlow` / `RuleSplitFlow` (item `w8x-subflow-splits`).  These derive
//!     from `TransformManager` (now merged via W6) and use
//!     `TransformVar`/`TransformOp`/`LaneDescription` from `transform.rs`.  The
//!     full split engine (`setReplacement`/`addOp`/`traceForward`/`traceBackward`/
//!     `processNextWork`/`doTrace`) and the `RuleSplitFlow` detection head are
//!     ported here.  `RuleSplitFlow::applyOp` runs `doTrace` (fully executable)
//!     then `TransformManager::apply` (W6-seamed in the merged transform.rs —
//!     `createReplacement` needs `glb->inst[opc]`).
//!   * `SubfloatFlow` / `RuleSubfloatConvert` (item `w8x-subflow-splits`).  The
//!     precision-tracing engine is ported verbatim; the `FloatFormat` acquisition
//!     (`f->getArch()->translate->getFloatFormat`) and the `preserveAddress`
//!     override are arch/transform seams (see [`subfloat_float_format`] and the
//!     losses output).
//!   * `SplitDatatype` / `RuleSplit{Copy,Load,Store}` / `RuleDumptyHumpLate`
//!     (item `w8x-subflow-splits`).  The datatype-splitting engine
//!     (`RootPointer`/`Component`, `categorizeDatatype`/`testDatatypeCompatibility`/
//!     `getComponent`/`splitCopy`/`splitLoad`/`splitStore`) is ported faithfully.
//!     The graph-mutation and type-facing calls that need the (still-stub)
//!     `Architecture`→`TypeFactory`/`Translate` bridge, `getTypeReadFacing`/
//!     `getTypeDefFacing`, `ResolveCache::addResolution`/`inheritResolution`, and
//!     `Funcdata::buildCopyTemp`/`getMerge` route through arch/W6 seam helpers (see
//!     the losses output); the size/offset categorization logic is fully
//!     executable.
//!   * `LaneDivide`: **SEAM(W5-transform-lanedivide)** — not named by this item;
//!     it is the last `TransformManager` subclass in `subflow.cc` and is left for
//!     a dedicated lane-divide item.  Recorded as a loss.
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
use kuna_base::types::{int4, int8, uint4, uintb};
use kuna_num::opcodes::OpCode;

use crate::action::{ActionGroupList, Rule, RuleSpec};
use crate::dtype::{type_metatype, Datatype};
use crate::funcdata::Funcdata;
use crate::op::pcodeop_flags;
use crate::seams::{OpId, TypeOp, VarnodeId};
use crate::transform::{LaneDescription, TVarRef, TransformManager};
use std::rc::Rc;

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
// SplitFlow / RuleSplitFlow (subflow.cc:1754-2088)
// =============================================================================
//
// SplitFlow derives from TransformManager and operates on
// TransformVar/TransformOp/LaneDescription (transform.rs, merged via W6).  The
// C++ `TransformVar *rvn` / `rvn+1` pointer arithmetic over the lane pair maps to
// the [`TVarRef`] handle and [`lane_above`] (advance the `Piece { key, idx }`
// index by one).

/// Advance a lane handle to the next-most-significant lane (the C++ `rvn+1`
/// pointer arithmetic over a `TransformVar` pair/array).
fn lane_above(rvn: TVarRef) -> TVarRef {
    match rvn {
        TVarRef::Piece { key, idx } => TVarRef::Piece { key, idx: idx + 1 },
        // The C++ only ever does `rvn+1` on a piece-array element (a split lane);
        // a `New(_)` here would be a porting bug.
        TVarRef::New(_) => panic!("lane_above on a non-piece TransformVar"),
    }
}

/// Class for splitting larger registers holding smaller logical values
/// (C++ `SplitFlow : public TransformManager`).
pub struct SplitFlow {
    /// The transform-manager state (C++ base class).
    tm: TransformManager,
    /// Description of how to split Varnodes (C++ `laneDescription`).
    lane_description: LaneDescription,
    /// Pending work list of Varnodes to push the split through (C++ `worklist`).
    worklist: Vec<TVarRef>,
}

impl SplitFlow {
    /// Constructor (C++ `SplitFlow::SplitFlow(Funcdata*,Varnode*,int4)`,
    /// subflow.cc:2011).
    pub fn new(data: &mut Funcdata, root: VarnodeId, low_size: int4) -> SplitFlow {
        let root_size = data.vbank().get(root).expect("SplitFlow: stale root").get_size();
        let mut sf = SplitFlow {
            tm: TransformManager::new(),
            // laneDescription(root->getSize(), lowSize, root->getSize()-lowSize)
            lane_description: LaneDescription::two_lane(root_size, low_size, root_size - low_size),
            worklist: Vec::new(),
        };
        sf.set_replacement(data, root);
        sf
    }

    /// Find or build the placeholder pair for a Varnode being split (C++
    /// `SplitFlow::setReplacement`, subflow.cc:1754).  Returns the lane-0 handle,
    /// or `None` for the C++ `(TransformVar *)0`.
    fn set_replacement(&mut self, data: &mut Funcdata, vn: VarnodeId) -> Option<TVarRef> {
        let v = data.vbank().get(vn).expect("set_replacement: stale vn");
        if v.is_mark() {
            // Already seen before
            return Some(self.tm.get_split(data, vn, &self.lane_description));
        }

        if v.is_type_lock() && v.get_type().get_metatype() != type_metatype::TYPE_PARTIALSTRUCT {
            return None;
        }
        if v.is_input() {
            return None; // Right now we can't split inputs
        }
        if v.is_free() && !v.is_constant() {
            return None; // Abort
        }

        let is_const = v.is_constant();
        // newSplit: Create new ReplaceVarnode and put it in map
        let res = self.tm.new_split(data, vn, &self.lane_description);
        data.vbank_mut().get_mut(vn).expect("set_replacement: stale vn").set_mark();
        if !is_const {
            self.worklist.push(res);
        }

        Some(res)
    }

    /// Split given op into its lanes (C++ `SplitFlow::addOp`, subflow.cc:1787).
    fn add_op(&mut self, data: &mut Funcdata, op: OpId, rvn: TVarRef, slot: int4) -> bool {
        let outvn = if slot == -1 {
            rvn
        } else {
            let out = data.obank().get(op).expect("add_op: stale op").get_out().expect("add_op: out");
            match self.set_replacement(data, out) {
                Some(o) => o,
                None => return false,
            }
        };

        if self.tm.var(outvn).get_def().is_some() {
            return true; // Already traversed
        }

        let code = data.obank().get(op).expect("add_op: stale op").code();
        let numinput = data.obank().get(op).expect("add_op: stale op").num_input();
        let lo_op = self.tm.new_op_replace(numinput, code, op);
        let hi_op = self.tm.new_op_replace(numinput, code, op);
        let mut num_param = numinput;
        if code == OpCode::CPUI_INDIRECT {
            let in1 = data.obank().get(op).expect("add_op: stale op").get_in(1).expect("add_op: in1");
            let iop_lo = self.tm.new_iop(data, in1);
            self.tm.op_set_input(lo_op, iop_lo, 1);
            let iop_hi = self.tm.new_iop(data, in1);
            self.tm.op_set_input(hi_op, iop_hi, 1);
            self.tm.inherit_indirect(lo_op, data, op);
            self.tm.inherit_indirect(hi_op, data, op);
            num_param = 1;
        }
        for i in 0..num_param {
            let invn = if i == slot {
                rvn
            } else {
                let ini = data.obank().get(op).expect("add_op: stale op").get_in(i).expect("add_op: in");
                match self.set_replacement(data, ini) {
                    Some(iv) => iv,
                    None => return false,
                }
            };
            self.tm.op_set_input(lo_op, invn, i); // Low piece with low op
            self.tm.op_set_input(hi_op, lane_above(invn), i); // High piece with high op
        }
        self.tm.op_set_output(lo_op, outvn);
        self.tm.op_set_output(hi_op, lane_above(outvn));
        true
    }

    /// Try to trace the pair of logical values forward through reading ops (C++
    /// `SplitFlow::traceForward`, subflow.cc:1834).
    fn trace_forward(&mut self, data: &mut Funcdata, rvn: TVarRef) -> bool {
        let origvn = self.tm.var(rvn).get_original().expect("trace_forward: rvn original");
        let descend: Vec<OpId> = data.descend_snapshot(origvn);
        for &op in descend.iter() {
            let outvn = data.obank().get(op).expect("trace_forward: stale op").get_out();
            if let Some(ov) = outvn {
                if data.vbank().get(ov).expect("trace_forward: stale outvn").is_mark() {
                    continue;
                }
            }
            let code = data.obank().get(op).expect("trace_forward: stale op").code();
            match code {
                OpCode::CPUI_COPY
                | OpCode::CPUI_MULTIEQUAL
                | OpCode::CPUI_INDIRECT
                | OpCode::CPUI_INT_AND
                | OpCode::CPUI_INT_OR
                | OpCode::CPUI_INT_XOR => {
                    let slot = data.obank().get(op).expect("trace_forward: stale op").get_slot(origvn);
                    if !self.add_op(data, op, rvn, slot) {
                        return false;
                    }
                }
                OpCode::CPUI_SUBPIECE => {
                    let ov = outvn.expect("trace_forward: SUBPIECE out");
                    let ovv = data.vbank().get(ov).expect("trace_forward: stale subpiece out");
                    if ovv.is_precis_lo() || ovv.is_precis_hi() {
                        return false; // Do not split if we know value comes from double precision pieces
                    }
                    let in1 = data.obank().get(op).expect("trace_forward: stale op").get_in(1).expect("in1");
                    let val = data.vbank().get(in1).expect("trace_forward: stale in1").get_offset();
                    let out_size = data.vbank().get(ov).expect("trace_forward: stale out").get_size();
                    if val == 0 && out_size == self.lane_description.get_size(0) {
                        let rop = self.tm.new_preexisting_op(1, OpCode::CPUI_COPY, op); // Grabs the low piece
                        self.tm.op_set_input(rop, rvn, 0);
                    } else if val == self.lane_description.get_size(0) as uintb
                        && out_size == self.lane_description.get_size(1)
                    {
                        let rop = self.tm.new_preexisting_op(1, OpCode::CPUI_COPY, op); // Grabs the high piece
                        self.tm.op_set_input(rop, lane_above(rvn), 0);
                    } else {
                        return false;
                    }
                }
                OpCode::CPUI_INT_LEFT => {
                    let tmpvn = data.obank().get(op).expect("trace_forward: stale op").get_in(1).expect("in1");
                    let tv = data.vbank().get(tmpvn).expect("trace_forward: stale shift amt");
                    if !tv.is_constant() {
                        return false;
                    }
                    let val = tv.get_offset();
                    if val < (self.lane_description.get_size(1) * 8) as uintb {
                        return false; // Must obliterate all high bits
                    }
                    let rop = self.tm.new_preexisting_op(2, OpCode::CPUI_INT_LEFT, op); // Keep original shift
                    let zextrop = self.tm.new_op(1, OpCode::CPUI_INT_ZEXT, rop);
                    self.tm.op_set_input(zextrop, rvn, 0); // Input is just the low piece
                    let uout = self.tm.new_unique(self.lane_description.get_whole_size());
                    self.tm.op_set_output(zextrop, uout);
                    let zout = self.tm.op(zextrop).get_out().expect("trace_forward: zext out");
                    self.tm.op_set_input(rop, zout, 0);
                    let in1size = data.vbank().get(tmpvn).expect("trace_forward: stale in1").get_size();
                    let in1off = data.vbank().get(tmpvn).expect("trace_forward: stale in1").get_offset();
                    let cst = self.tm.new_constant(in1size, 0, in1off); // Original shift amount
                    self.tm.op_set_input(rop, cst, 1);
                }
                OpCode::CPUI_INT_SRIGHT | OpCode::CPUI_INT_RIGHT => {
                    let tmpvn = data.obank().get(op).expect("trace_forward: stale op").get_in(1).expect("in1");
                    let tv = data.vbank().get(tmpvn).expect("trace_forward: stale shift amt");
                    if !tv.is_constant() {
                        return false;
                    }
                    let val = tv.get_offset();
                    if val < (self.lane_description.get_size(0) * 8) as uintb {
                        return false;
                    }
                    let ext_op_code = if code == OpCode::CPUI_INT_RIGHT {
                        OpCode::CPUI_INT_ZEXT
                    } else {
                        OpCode::CPUI_INT_SEXT
                    };
                    if val == (self.lane_description.get_size(0) * 8) as uintb {
                        // Shift of exactly loSize bytes
                        let rop = self.tm.new_preexisting_op(1, ext_op_code, op);
                        self.tm.op_set_input(rop, lane_above(rvn), 0); // Input is the high piece
                    } else {
                        let remain_shift = val - (self.lane_description.get_size(0) * 8) as uintb;
                        let rop = self.tm.new_preexisting_op(2, code, op);
                        let extrop = self.tm.new_op(1, ext_op_code, rop);
                        self.tm.op_set_input(extrop, lane_above(rvn), 0); // Input is the high piece
                        let uout = self.tm.new_unique(self.lane_description.get_whole_size());
                        self.tm.op_set_output(extrop, uout);
                        let eout = self.tm.op(extrop).get_out().expect("trace_forward: ext out");
                        self.tm.op_set_input(rop, eout, 0);
                        let in1size = data.vbank().get(tmpvn).expect("trace_forward: stale in1").get_size();
                        let cst = self.tm.new_constant(in1size, 0, remain_shift); // Shift any remaining bits
                        self.tm.op_set_input(rop, cst, 1);
                    }
                }
                _ => return false,
            }
        }
        true
    }

    /// Try to trace the pair of logical values backward through the defining op
    /// (C++ `SplitFlow::traceBackward`, subflow.cc:1927).
    fn trace_backward(&mut self, data: &mut Funcdata, rvn: TVarRef) -> bool {
        let origvn = self.tm.var(rvn).get_original().expect("trace_backward: rvn original");
        let op = match data.vbank().get(origvn).expect("trace_backward: stale origvn").get_def() {
            Some(o) => o,
            None => return true, // If vn is input
        };

        let code = data.obank().get(op).expect("trace_backward: stale op").code();
        match code {
            OpCode::CPUI_COPY
            | OpCode::CPUI_MULTIEQUAL
            | OpCode::CPUI_INT_AND
            | OpCode::CPUI_INT_OR
            | OpCode::CPUI_INT_XOR
            | OpCode::CPUI_INDIRECT => {
                if !self.add_op(data, op, rvn, -1) {
                    return false;
                }
            }
            OpCode::CPUI_PIECE => {
                let in0 = data.obank().get(op).expect("stale op").get_in(0).expect("in0");
                let in1 = data.obank().get(op).expect("stale op").get_in(1).expect("in1");
                if data.vbank().get(in0).expect("stale in0").get_size() != self.lane_description.get_size(1) {
                    return false;
                }
                if data.vbank().get(in1).expect("stale in1").get_size() != self.lane_description.get_size(0) {
                    return false;
                }
                let lo_op = self.tm.new_op_replace(1, OpCode::CPUI_COPY, op);
                let hi_op = self.tm.new_op_replace(1, OpCode::CPUI_COPY, op);
                let pre_lo = self.tm.get_preexisting_varnode(data, in1);
                self.tm.op_set_input(lo_op, pre_lo, 0);
                self.tm.op_set_output(lo_op, rvn); // Least sig -> low
                let pre_hi = self.tm.get_preexisting_varnode(data, in0);
                self.tm.op_set_input(hi_op, pre_hi, 0);
                self.tm.op_set_output(hi_op, lane_above(rvn)); // Most sig -> high
            }
            OpCode::CPUI_INT_ZEXT => {
                let in0 = data.obank().get(op).expect("stale op").get_in(0).expect("in0");
                let out = data.obank().get(op).expect("stale op").get_out().expect("out");
                if data.vbank().get(in0).expect("stale in0").get_size() != self.lane_description.get_size(0) {
                    return false;
                }
                if data.vbank().get(out).expect("stale out").get_size() != self.lane_description.get_whole_size() {
                    return false;
                }
                let lo_op = self.tm.new_op_replace(1, OpCode::CPUI_COPY, op);
                let hi_op = self.tm.new_op_replace(1, OpCode::CPUI_COPY, op);
                let pre0 = self.tm.get_preexisting_varnode(data, in0);
                self.tm.op_set_input(lo_op, pre0, 0);
                self.tm.op_set_output(lo_op, rvn); // ZEXT input -> low
                let zero = self.tm.new_constant(self.lane_description.get_size(1), 0, 0);
                self.tm.op_set_input(hi_op, zero, 0);
                self.tm.op_set_output(hi_op, lane_above(rvn)); // zero -> high
            }
            OpCode::CPUI_INT_LEFT => {
                let cvn = data.obank().get(op).expect("stale op").get_in(1).expect("in1");
                let cv = data.vbank().get(cvn).expect("stale cvn");
                if !cv.is_constant() {
                    return false;
                }
                if cv.get_offset() != (self.lane_description.get_size(0) * 8) as uintb {
                    return false;
                }
                let mut invn = data.obank().get(op).expect("stale op").get_in(0).expect("in0");
                if !data.vbank().get(invn).expect("stale invn").is_written() {
                    return false;
                }
                let zext_op = data.vbank().get(invn).expect("stale invn").get_def().expect("zext def");
                if data.obank().get(zext_op).expect("stale zext").code() != OpCode::CPUI_INT_ZEXT {
                    return false;
                }
                invn = data.obank().get(zext_op).expect("stale zext").get_in(0).expect("zext in0");
                if data.vbank().get(invn).expect("stale invn").get_size() != self.lane_description.get_size(1) {
                    return false;
                }
                if data.vbank().get(invn).expect("stale invn").is_free() {
                    return false;
                }
                let lo_op = self.tm.new_op_replace(1, OpCode::CPUI_COPY, op);
                let hi_op = self.tm.new_op_replace(1, OpCode::CPUI_COPY, op);
                let zero = self.tm.new_constant(self.lane_description.get_size(0), 0, 0);
                self.tm.op_set_input(lo_op, zero, 0);
                self.tm.op_set_output(lo_op, rvn); // zero -> low
                let pre = self.tm.get_preexisting_varnode(data, invn);
                self.tm.op_set_input(hi_op, pre, 0);
                self.tm.op_set_output(hi_op, lane_above(rvn)); // invn -> high
            }
            _ => return false,
        }
        true
    }

    /// Process the next logical value on the worklist (C++
    /// `SplitFlow::processNextWork`, subflow.cc:2000).
    fn process_next_work(&mut self, data: &mut Funcdata) -> bool {
        let rvn = self.worklist.pop().expect("process_next_work: empty worklist");
        if !self.trace_backward(data, rvn) {
            return false;
        }
        self.trace_forward(data, rvn)
    }

    /// Trace split through data-flow, constructing the transform (C++
    /// `SplitFlow::doTrace`, subflow.cc:2021).
    pub fn do_trace(&mut self, data: &mut Funcdata) -> bool {
        if self.worklist.is_empty() {
            return false; // Nothing to do
        }
        let mut retval = true;
        while !self.worklist.is_empty() {
            if !self.process_next_work(data) {
                retval = false;
                break;
            }
        }
        self.tm.clear_varnode_marks(data);
        retval
    }

    /// Apply the constructed transform (C++ `TransformManager::apply` via the
    /// base class).  SEAM(W6): the merged `TransformManager::apply` reaches
    /// `createReplacement` → `glb->inst[opc]` and returns its W6 seam error.
    pub fn apply(&mut self, data: &mut Funcdata) -> KunaResult<()> {
        self.tm.apply(data)
    }
}

/// Try to detect and split artificially joined Varnodes (C++ `RuleSplitFlow`).
pub struct RuleSplitFlow {
    group: String,
}

impl RuleSplitFlow {
    /// Construct with the rule's stage group (C++ `RuleSplitFlow::RuleSplitFlow`).
    pub fn new(g: impl Into<String>) -> RuleSplitFlow {
        RuleSplitFlow { group: g.into() }
    }
}

impl Rule for RuleSplitFlow {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_SUBPIECE]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(&self.group) {
            return None;
        }
        Some(Box::new(RuleSplitFlow::new(self.group.clone())))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        let in1 = data.obank().get(op).expect("stale op").get_in(1).expect("in1");
        let lo_size = data.vbank().get(in1).expect("stale in1").get_offset() as int4;
        if lo_size == 0 {
            return 0; // Make sure SUBPIECE doesn't take least significant part
        }
        let vn = data.obank().get(op).expect("stale op").get_in(0).expect("in0");
        let vv = data.vbank().get(vn).expect("stale vn");
        if !vv.is_written() {
            return 0;
        }
        if vv.is_precis_lo() || vv.is_precis_hi() {
            return 0;
        }
        let out = data.obank().get(op).expect("stale op").get_out().expect("out");
        let out_size = data.vbank().get(out).expect("stale out").get_size();
        let vn_size = data.vbank().get(vn).expect("stale vn").get_size();
        if out_size + lo_size != vn_size {
            return 0; // Make sure SUBPIECE is taking most significant part
        }
        let mut concat_op: Option<OpId> = None;
        let mut multi_op = data.vbank().get(vn).expect("stale vn").get_def().expect("vn def");
        while data.obank().get(multi_op).expect("stale multi").code() == OpCode::CPUI_INDIRECT {
            // PIECE may come through INDIRECT
            let tmpvn = data.obank().get(multi_op).expect("stale multi").get_in(0).expect("indirect in0");
            if !data.vbank().get(tmpvn).expect("stale tmpvn").is_written() {
                return 0;
            }
            multi_op = data.vbank().get(tmpvn).expect("stale tmpvn").get_def().expect("tmpvn def");
        }
        let multi_code = data.obank().get(multi_op).expect("stale multi").code();
        if multi_code == OpCode::CPUI_PIECE {
            let vn_def = data.vbank().get(vn).expect("stale vn").get_def();
            if vn_def != Some(multi_op) {
                concat_op = Some(multi_op);
            }
        } else if multi_code == OpCode::CPUI_MULTIEQUAL {
            // Otherwise PIECE comes through MULTIEQUAL
            let ninput = data.obank().get(multi_op).expect("stale multi").num_input();
            for i in 0..ninput {
                let invn = data.obank().get(multi_op).expect("stale multi").get_in(i).expect("multi in");
                if !data.vbank().get(invn).expect("stale invn").is_written() {
                    continue;
                }
                let tmp_op = data.vbank().get(invn).expect("stale invn").get_def().expect("invn def");
                if data.obank().get(tmp_op).expect("stale tmp").code() == OpCode::CPUI_PIECE {
                    concat_op = Some(tmp_op);
                    break;
                }
            }
        }
        let concat_op = match concat_op {
            Some(c) => c,
            None => return 0, // Didn't find the concatenate
        };
        let cin1 = data.obank().get(concat_op).expect("stale concat").get_in(1).expect("concat in1");
        if data.vbank().get(cin1).expect("stale cin1").get_size() != lo_size {
            return 0;
        }
        let mut split_flow = SplitFlow::new(data, vn, lo_size);
        if !split_flow.do_trace(data) {
            return 0;
        }
        match split_flow.apply(data) {
            Ok(()) => 1,
            // SEAM(W6): TransformManager::apply → createReplacement needs
            // glb->inst[opc]; discovery (doTrace) ran fully -> no change applied.
            Err(_) => 0,
        }
    }
}

// =============================================================================
// SplitDatatype / RuleSplit{Copy,Load,Store} (subflow.cc:2090-3019)
// =============================================================================
//
// SplitDatatype breaks a COPY/LOAD/STORE on a partial structure or array into
// per-component ops.  The *categorization* logic (categorizeDatatype,
// getComponent, testDatatypeCompatibility) needs only the dtype.rs surface and
// is fully executable.  The graph mutation (splitCopy/splitLoad/splitStore and
// the build* helpers + RootPointer) needs the (still-stub) Architecture →
// TypeFactory/Translate bridge through Funcdata::getArch(), getTypeReadFacing/
// getTypeDefFacing (W6), ResolveCache::addResolution/inheritResolution (W6),
// and Funcdata::buildCopyTemp/getMerge — none of which are on the surface the
// merged W6 wave wired into Funcdata.  Those call sites route through
// [`split_datatype_seam`]; the rules detect the seam and return 0 (no change).

/// `OptionSplitDatatypes` config bits (C++ `OptionSplitDatatypes`,
/// `options.hh`).  `split_datatype_config` is a bit-mask of these.
mod option_split_datatypes {
    use kuna_base::types::uint4;
    /// Split structures (C++ `OptionSplitDatatypes::option_struct`).
    pub const OPTION_STRUCT: uint4 = 1;
    /// Split arrays (C++ `OptionSplitDatatypes::option_array`).
    pub const OPTION_ARRAY: uint4 = 2;
}

/// A pair of matching data-types for one logical piece of the split (C++
/// `SplitDatatype::Component`).
///
/// Read by the seam-gated `split{Copy,Load,Store}` build-helpers (which materialize
/// per-component ops); until the Funcdata arch bridge lands those mutation paths
/// return the seam error, so the fields are exercised only by the categorization
/// tests.  // SEAM(arch/W6)
#[derive(Debug, Clone)]
#[allow(dead_code)]
struct Component {
    /// Data-type coming into the logical COPY operation (C++ `inType`).
    in_type: Rc<Datatype>,
    /// Data-type coming out of the logical COPY operation (C++ `outType`).
    out_type: Rc<Datatype>,
    /// Offset of this logical piece within the whole (C++ `offset`).
    offset: int4,
}

/// Split a p-code COPY, LOAD, or STORE based on the underlying composite
/// data-type (C++ `SplitDatatype`).
///
/// The C++ holds a `Funcdata &data` and `TypeFactory *types`; here both come
/// through the threaded `&mut Funcdata` argument plus the supplied
/// `&dyn TypeFactory` (the pure-logic methods take the factory explicitly so
/// they remain testable, mirroring how the C++ `types` member is a borrow of
/// `glb->types`).
//
// `data_type_pieces`/`split_structures`/`split_arrays` and the logic methods
// (`get_component`/`categorize_datatype`/`test_datatype_compatibility`) feed the
// seam-gated `split{Copy,Load,Store}` build path; they run today only from the
// categorization tests until the Funcdata arch bridge lands.  // SEAM(arch/W6)
#[allow(dead_code)]
pub struct SplitDatatype {
    /// Sequence of all data-type pairs being copied (C++ `dataTypePieces`).
    data_type_pieces: Vec<Component>,
    /// Whether or not structures should be split (C++ `splitStructures`).
    split_structures: bool,
    /// Whether or not arrays should be split (C++ `splitArrays`).
    split_arrays: bool,
    /// True if trying to split LOAD or STORE (C++ `isLoadStore`).
    is_load_store: bool,
}

impl SplitDatatype {
    /// Construct from the split-datatype config bits (C++
    /// `SplitDatatype::SplitDatatype(Funcdata&)`, subflow.cc:2712).
    ///
    /// The C++ reads `glb->split_datatype_config`; the Funcdata-side
    /// `Architecture` seam does not yet expose it, so the caller supplies the
    /// config mask (the entry rules pass 0 until the arch bridge lands).
    pub fn new(split_datatype_config: uint4) -> SplitDatatype {
        SplitDatatype {
            data_type_pieces: Vec::new(),
            split_structures: (split_datatype_config & option_split_datatypes::OPTION_STRUCT) != 0,
            split_arrays: (split_datatype_config & option_split_datatypes::OPTION_ARRAY) != 0,
            is_load_store: false,
        }
    }

    /// Obtain the component of the given data-type at the specified offset (C++
    /// `SplitDatatype::getComponent`, subflow.cc:2219).  Returns
    /// `(component, is_hole)`.
    #[allow(dead_code)] // consumed by the seam-gated splitCopy/Load/Store build path
    fn get_component(
        &self,
        types: &dyn crate::dtype::TypeFactory,
        ct: &Rc<Datatype>,
        offset: int4,
    ) -> KunaResult<(Option<Rc<Datatype>>, bool)> {
        let mut cur_type = Rc::clone(ct);
        let mut cur_off: int8 = offset as int8;
        loop {
            let (sub, newoff) = cur_type.get_sub_type(cur_off)?;
            match sub {
                None => {
                    let hole = ct.get_hole_size(offset)?;
                    if hole > 0 {
                        let hole = if hole > 8 { 8 } else { hole };
                        return Ok((Some(types.get_base(hole, type_metatype::TYPE_UNKNOWN)?), true));
                    }
                    return Ok((None, false));
                }
                Some(s) => {
                    cur_type = s;
                    cur_off = newoff;
                }
            }
            if cur_off == 0 && cur_type.get_metatype() != type_metatype::TYPE_ARRAY {
                break;
            }
        }
        Ok((Some(cur_type), false))
    }

    /// Categorize if and how a data-type should be split (C++
    /// `SplitDatatype::categorizeDatatype`, subflow.cc:2248).
    ///
    /// Returns -1 (not splittable), 0 (struct-based), 1 (array-based), 2
    /// (primitive splittable multiple ways).
    #[allow(dead_code)] // consumed by the seam-gated splitCopy/Load/Store build path
    fn categorize_datatype(&self, ct: &Rc<Datatype>) -> int4 {
        match ct.get_metatype() {
            type_metatype::TYPE_ARRAY => {
                if !self.split_arrays {
                    return -1;
                }
                let sub = ct.get_array_base().expect("TYPE_ARRAY has base");
                if sub.get_metatype() != type_metatype::TYPE_UNKNOWN || sub.get_size() != 1 {
                    1
                } else {
                    2 // unknown1 array acts as a (large) primitive
                }
            }
            type_metatype::TYPE_PARTIALSTRUCT => {
                let sub = ct.get_partial_base().expect("TYPE_PARTIALSTRUCT has parent");
                if sub.get_metatype() == type_metatype::TYPE_ARRAY {
                    if !self.split_arrays {
                        return -1;
                    }
                    let subsub = sub.get_array_base().expect("array base");
                    if subsub.get_metatype() != type_metatype::TYPE_UNKNOWN || subsub.get_size() != 1 {
                        1
                    } else {
                        2
                    }
                } else if sub.get_metatype() == type_metatype::TYPE_STRUCT {
                    if !self.split_structures {
                        return -1;
                    }
                    0
                } else {
                    -1
                }
            }
            type_metatype::TYPE_STRUCT => {
                if !self.split_structures {
                    return -1;
                }
                if ct.num_depend() > 1 {
                    return 0;
                }
                -1
            }
            type_metatype::TYPE_INT | type_metatype::TYPE_UINT | type_metatype::TYPE_UNKNOWN => 2,
            _ => -1,
        }
    }

    /// Can the two given data-types be mutually split into matching logical
    /// components (C++ `SplitDatatype::testDatatypeCompatibility`,
    /// subflow.cc:2300).  Populates `data_type_pieces` and returns `true` on a
    /// compatible split.
    #[allow(dead_code)] // consumed by the seam-gated splitCopy/Load/Store build path
    fn test_datatype_compatibility(
        &mut self,
        types: &dyn crate::dtype::TypeFactory,
        in_base: &Rc<Datatype>,
        out_base: &Rc<Datatype>,
        in_constant: bool,
    ) -> KunaResult<bool> {
        let in_category = self.categorize_datatype(in_base);
        if in_category < 0 {
            return Ok(false);
        }
        let out_category = self.categorize_datatype(out_base);
        if out_category < 0 {
            return Ok(false);
        }
        if out_category == 2 && in_category == 2 {
            return Ok(false);
        }
        if !in_constant
            && Rc::ptr_eq(in_base, out_base)
            && in_base.get_metatype() == type_metatype::TYPE_STRUCT
        {
            return Ok(false); // Don't split a whole structure unless initialized from a constant
        }
        if self.is_load_store && out_category == 2 && in_category == 1 {
            return Ok(false); // Don't split array pointer writing into primitive
        }
        if self.is_load_store && in_category == 2 && !in_constant && out_category == 1 {
            return Ok(false); // Don't split primitive into an array pointer
        }
        if self.is_load_store && in_category == 1 && out_category == 1 && !in_constant {
            return Ok(false); // Don't split copies between arrays
        }
        let mut cur_off = 0i32;
        let mut size_left = in_base.get_size();
        if in_category == 2 {
            // Input is primitive
            while size_left > 0 {
                let (cur_out, out_hole) = self.get_component(types, out_base, cur_off)?;
                let cur_out = match cur_out {
                    Some(c) => c,
                    None => return Ok(false),
                };
                // Throw away primitive data-type if it is a constant
                let cur_in = if in_constant {
                    Rc::clone(&cur_out)
                } else {
                    types.get_base(cur_out.get_size(), type_metatype::TYPE_UNKNOWN)?
                };
                let osize = cur_out.get_size();
                self.data_type_pieces.push(Component { in_type: cur_in, out_type: cur_out, offset: cur_off });
                size_left -= osize;
                cur_off += osize;
                if out_hole {
                    if self.data_type_pieces.len() == 1 {
                        return Ok(false); // Initial offset into structure is at a hole
                    }
                    if size_left == 0 && self.data_type_pieces.len() == 2 {
                        return Ok(false); // Two pieces, one a hole.  Likely padding.
                    }
                }
            }
        } else if out_category == 2 {
            // Output is primitive
            while size_left > 0 {
                let (cur_in, in_hole) = self.get_component(types, in_base, cur_off)?;
                let cur_in = match cur_in {
                    Some(c) => c,
                    None => return Ok(false),
                };
                let cur_out = types.get_base(cur_in.get_size(), type_metatype::TYPE_UNKNOWN)?;
                let isize = cur_in.get_size();
                self.data_type_pieces.push(Component { in_type: cur_in, out_type: cur_out, offset: cur_off });
                size_left -= isize;
                cur_off += isize;
                if in_hole {
                    if self.data_type_pieces.len() == 1 {
                        return Ok(false);
                    }
                    if size_left == 0 && self.data_type_pieces.len() == 2 {
                        return Ok(false);
                    }
                }
            }
        } else {
            // Both in and out data-types have components
            while size_left > 0 {
                let (cur_in0, mut in_hole) = self.get_component(types, in_base, cur_off)?;
                let mut cur_in = match cur_in0 {
                    Some(c) => c,
                    None => return Ok(false),
                };
                let (cur_out0, mut out_hole) = self.get_component(types, out_base, cur_off)?;
                let mut cur_out = match cur_out0 {
                    Some(c) => c,
                    None => return Ok(false),
                };
                while cur_in.get_size() != cur_out.get_size() {
                    if cur_in.get_size() > cur_out.get_size() {
                        if in_hole {
                            cur_in = types.get_base(cur_out.get_size(), type_metatype::TYPE_UNKNOWN)?;
                        } else {
                            let (ci, ih) = self.get_component(types, &cur_in, 0)?;
                            in_hole = ih;
                            cur_in = match ci {
                                Some(c) => c,
                                None => return Ok(false),
                            };
                        }
                    } else if out_hole {
                        cur_out = types.get_base(cur_in.get_size(), type_metatype::TYPE_UNKNOWN)?;
                    } else {
                        let (co, oh) = self.get_component(types, &cur_out, 0)?;
                        out_hole = oh;
                        cur_out = match co {
                            Some(c) => c,
                            None => return Ok(false),
                        };
                    }
                }
                let isize = cur_in.get_size();
                self.data_type_pieces.push(Component { in_type: cur_in, out_type: cur_out, offset: cur_off });
                size_left -= isize;
                cur_off += isize;
            }
        }
        Ok(self.data_type_pieces.len() > 1)
    }

    /// Split a COPY operation (C++ `SplitDatatype::splitCopy`, subflow.cc:2728).
    ///
    /// SEAM(arch/W6): `testCopyConstraints`, `getTypeReadFacing`/`getTypeDefFacing`,
    /// `ResolveCache::addResolution/inheritResolution`, the `build*` helpers, and
    /// `RootPointer` all route through the unported Funcdata→Architecture→
    /// TypeFactory/Translate bridge.  The detection in `RuleSplitCopy` already
    /// returns 0 before reaching here on that seam; this body is the faithful
    /// transcription guarded by the seam error.
    pub fn split_copy(
        &mut self,
        _data: &mut Funcdata,
        _copy_op: OpId,
        _in_type: &Rc<Datatype>,
        _out_type: &Rc<Datatype>,
    ) -> KunaResult<bool> {
        split_datatype_seam("SplitDatatype::splitCopy")
    }

    /// Split a LOAD operation (C++ `SplitDatatype::splitLoad`, subflow.cc:2770).
    pub fn split_load(
        &mut self,
        _data: &mut Funcdata,
        _load_op: OpId,
        _in_type: &Rc<Datatype>,
    ) -> KunaResult<bool> {
        self.is_load_store = true;
        split_datatype_seam("SplitDatatype::splitLoad")
    }

    /// Split a STORE operation (C++ `SplitDatatype::splitStore`, subflow.cc:2823).
    pub fn split_store(
        &mut self,
        _data: &mut Funcdata,
        _store_op: OpId,
        _out_type: &Rc<Datatype>,
    ) -> KunaResult<bool> {
        self.is_load_store = true;
        split_datatype_seam("SplitDatatype::splitStore")
    }

    /// Get a data-type description of the value pointed at by a LOAD/STORE (C++
    /// `SplitDatatype::getValueDatatype`, subflow.cc:2925).
    ///
    /// SEAM(W6): needs `loadStore->getIn(1)->getTypeReadFacing(loadStore)` and the
    /// `TypeFactory` (`getTypeArray`/`getExactPiece`) — both off the Funcdata arch
    /// seam.  Returns `None` (the C++ null) until that bridge lands.
    pub fn get_value_datatype(
        _data: &Funcdata,
        _load_store: OpId,
        _size: int4,
    ) -> Option<Rc<Datatype>> {
        // C++: ptrType = loadStore->getIn(1)->getTypeReadFacing(loadStore); ...
        None
    }
}

/// The shared SEAM error for the SplitDatatype graph-mutation path
/// (Funcdata→Architecture→TypeFactory/Translate bridge + getTypeReadFacing/
/// getTypeDefFacing + ResolveCache::addResolution + buildCopyTemp/getMerge).
fn split_datatype_seam(what: &str) -> KunaResult<bool> {
    Err(KunaError::lowlevel(format!(
        "kuna rust port: {what} needs the Funcdata→Architecture→TypeFactory/Translate \
         bridge, getTypeReadFacing/getTypeDefFacing (W6), ResolveCache::addResolution/\
         inheritResolution (W6), and Funcdata::buildCopyTemp/getMerge — SEAM(arch/W6)"
    )))
}

/// Split COPY ops based on TypePartialStruct (C++ `RuleSplitCopy`).
pub struct RuleSplitCopy {
    group: String,
}

impl RuleSplitCopy {
    /// Construct with the rule's stage group (C++ `RuleSplitCopy::RuleSplitCopy`).
    pub fn new(g: impl Into<String>) -> RuleSplitCopy {
        RuleSplitCopy { group: g.into() }
    }
}

impl Rule for RuleSplitCopy {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_COPY]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(&self.group) {
            return None;
        }
        Some(Box::new(RuleSplitCopy::new(self.group.clone())))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // Datatype *inType = op->getIn(0)->getTypeReadFacing(op);   -- SEAM(W6)
        // Datatype *outType = op->getOut()->getTypeDefFacing();     -- SEAM(W6)
        // The type-facing accessors are off the Funcdata arch seam; the
        // categorization on metatype below cannot run without them, so the rule
        // returns 0 (no change) until the bridge lands.  // SEAM(arch/W6)
        match split_copy_facing_seam(data, op) {
            Ok(()) => 0,
            Err(_) => 0,
        }
    }
}

/// Split LOAD ops based on TypePartialStruct (C++ `RuleSplitLoad`).
pub struct RuleSplitLoad {
    group: String,
}

impl RuleSplitLoad {
    /// Construct with the rule's stage group (C++ `RuleSplitLoad::RuleSplitLoad`).
    pub fn new(g: impl Into<String>) -> RuleSplitLoad {
        RuleSplitLoad { group: g.into() }
    }
}

impl Rule for RuleSplitLoad {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_LOAD]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(&self.group) {
            return None;
        }
        Some(Box::new(RuleSplitLoad::new(self.group.clone())))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // Datatype *inType = SplitDatatype::getValueDatatype(op, op->getOut()->getSize(), types);
        let out = data.obank().get(op).expect("stale op").get_out().expect("out");
        let out_size = data.vbank().get(out).expect("stale out").get_size();
        let in_type = match SplitDatatype::get_value_datatype(data, op, out_size) {
            Some(t) => t,
            None => return 0,
        };
        let metain = in_type.get_metatype();
        if metain != type_metatype::TYPE_STRUCT
            && metain != type_metatype::TYPE_ARRAY
            && metain != type_metatype::TYPE_PARTIALSTRUCT
        {
            return 0;
        }
        let mut splitter = SplitDatatype::new(0);
        match splitter.split_load(data, op, &in_type) {
            Ok(true) => 1,
            _ => 0,
        }
    }
}

/// Split STORE ops based on TypePartialStruct (C++ `RuleSplitStore`).
pub struct RuleSplitStore {
    group: String,
}

impl RuleSplitStore {
    /// Construct with the rule's stage group (C++ `RuleSplitStore::RuleSplitStore`).
    pub fn new(g: impl Into<String>) -> RuleSplitStore {
        RuleSplitStore { group: g.into() }
    }
}

impl Rule for RuleSplitStore {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_STORE]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(&self.group) {
            return None;
        }
        Some(Box::new(RuleSplitStore::new(self.group.clone())))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // Datatype *outType = SplitDatatype::getValueDatatype(op, op->getIn(2)->getSize(), types);
        let in2 = data.obank().get(op).expect("stale op").get_in(2).expect("store in2");
        let in2_size = data.vbank().get(in2).expect("stale in2").get_size();
        let out_type = match SplitDatatype::get_value_datatype(data, op, in2_size) {
            Some(t) => t,
            None => return 0,
        };
        let metain = out_type.get_metatype();
        if metain != type_metatype::TYPE_STRUCT
            && metain != type_metatype::TYPE_ARRAY
            && metain != type_metatype::TYPE_PARTIALSTRUCT
        {
            return 0;
        }
        let mut splitter = SplitDatatype::new(0);
        match splitter.split_store(data, op, &out_type) {
            Ok(true) => 1,
            _ => 0,
        }
    }
}

/// The COPY-split type-facing seam (C++ `op->getIn(0)->getTypeReadFacing(op)`
/// / `op->getOut()->getTypeDefFacing()`).  SEAM(arch/W6).
fn split_copy_facing_seam(_data: &Funcdata, _op: OpId) -> KunaResult<()> {
    Err(KunaError::lowlevel(
        "kuna rust port: RuleSplitCopy needs getTypeReadFacing/getTypeDefFacing off the \
         Funcdata arch seam — SEAM(arch/W6)",
    ))
}

// =============================================================================
// RuleDumptyHumpLate (subflow.cc:3021-3083)
// =============================================================================

/// Simplify join and break apart based on data-types (C++ `RuleDumptyHumpLate`).
pub struct RuleDumptyHumpLate {
    group: String,
}

impl RuleDumptyHumpLate {
    /// Construct with the rule's stage group (C++
    /// `RuleDumptyHumpLate::RuleDumptyHumpLate`).
    pub fn new(g: impl Into<String>) -> RuleDumptyHumpLate {
        RuleDumptyHumpLate { group: g.into() }
    }
}

impl Rule for RuleDumptyHumpLate {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_SUBPIECE]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(&self.group) {
            return None;
        }
        Some(Box::new(RuleDumptyHumpLate::new(self.group.clone())))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        // The funcdata mutations (opSetInput/totalReplace) return KunaResult; the
        // C++ applyOp is infallible, so we run the body and map any seam error to
        // 0 (no change) at this boundary.
        Self::apply_op_inner(op, data).unwrap_or(0)
    }
}

impl RuleDumptyHumpLate {
    /// Body of `applyOp` (C++ `RuleDumptyHumpLate::applyOp`, subflow.cc:3027),
    /// returning `KunaResult` so the funcdata mutations can use `?`.
    fn apply_op_inner(op: OpId, data: &mut Funcdata) -> KunaResult<int4> {
        let in0 = data.obank().get(op).expect("stale op").get_in(0).expect("in0");
        if !data.vbank().get(in0).expect("stale in0").is_written() {
            return Ok(0);
        }
        let mut piece_op = data.vbank().get(in0).expect("stale in0").get_def().expect("in0 def");
        if data.obank().get(piece_op).expect("stale piece").code() != OpCode::CPUI_PIECE {
            return Ok(0);
        }
        let out = data.obank().get(op).expect("stale op").get_out().expect("out");
        let out_size = data.vbank().get(out).expect("stale out").get_size();
        let in1 = data.obank().get(op).expect("stale op").get_in(1).expect("in1");
        let mut trunc = data.vbank().get(in1).expect("stale in1").get_offset() as int4;
        let mut vn = in0;
        loop {
            // Try to backtrack thru PIECE to the component vn is truncated from
            let p_in1 = data.obank().get(piece_op).expect("stale piece").get_in(1).expect("piece in1");
            let mut trial_vn = p_in1; // Assume the least significant component
            let mut trial_trunc = trunc;
            let trial_vn_size = data.vbank().get(trial_vn).expect("stale trial").get_size();
            if trunc >= trial_vn_size {
                // Test for truncation from the most significant part
                trial_trunc -= trial_vn_size; // How much is truncated
                trial_vn = data.obank().get(piece_op).expect("stale piece").get_in(0).expect("piece in0");
            }
            let trial_vn_size = data.vbank().get(trial_vn).expect("stale trial").get_size();
            if out_size + trial_trunc > trial_vn_size {
                break; // vn crosses both components
            }
            vn = trial_vn; // Commit to this component
            trunc = trial_trunc;
            if data.vbank().get(vn).expect("stale vn").get_size() == out_size {
                break; // Found matching component
            }
            if !data.vbank().get(vn).expect("stale vn").is_written() {
                break;
            }
            piece_op = data.vbank().get(vn).expect("stale vn").get_def().expect("vn def");
            if data.obank().get(piece_op).expect("stale piece").code() != OpCode::CPUI_PIECE {
                break;
            }
        }
        if vn == in0 {
            return Ok(0); // Didn't backtrack thru any PIECE
        }
        if data.vbank().get(vn).expect("stale vn").is_written() {
            let vndef = data.vbank().get(vn).expect("stale vn").get_def().expect("vn def");
            if data.obank().get(vndef).expect("stale vndef").code() == OpCode::CPUI_COPY {
                vn = data.obank().get(vndef).expect("stale vndef").get_in(0).expect("copy in0");
            }
        }
        let vn_size = data.vbank().get(vn).expect("stale vn").get_size();
        let remove_op: OpId;
        if out_size != vn_size {
            // Component does not match size exactly.  Preserve SUBPIECE.
            remove_op = data.vbank().get(in0).expect("stale in0").get_def().expect("in0 def");
            let in1off = data.vbank().get(in1).expect("stale in1").get_offset();
            if in1off != trunc as uintb {
                let cst = data.new_constant(4, trunc as uintb);
                data.op_set_input(op, cst, 1)?;
            }
            data.op_set_input(op, vn, 0)?;
        } else if data.vbank().get(out).expect("stale out").is_auto_live() {
            // Exact match but output address fixed.  Change SUBPIECE to COPY.
            remove_op = data.vbank().get(in0).expect("stale in0").get_def().expect("in0 def");
            data.op_remove_input(op, 1);
            // fd->opSetOpcode(op, CPUI_COPY).  SEAM(W6): glb->inst[CPUI_COPY] — the
            // TypeOp flag word is transcribed inline from typeop.cc
            // (TypeOpCopy: unary | nocollapse), mirroring Funcdata::w6_type_op.
            data.op_set_opcode(
                op,
                TypeOp::new(
                    OpCode::CPUI_COPY,
                    pcodeop_flags::unary | pcodeop_flags::nocollapse,
                    "copy".to_string(),
                ),
            );
            data.op_set_input(op, vn, 0)?;
        } else {
            // Exact match.  Completely replace output with component.
            remove_op = op;
            data.total_replace(out, vn)?;
        }
        let remove_out = data.obank().get(remove_op).expect("stale remove").get_out().expect("remove out");
        let rov = data.vbank().get(remove_out).expect("stale remove out");
        if rov.has_no_descend() && !rov.is_auto_live() {
            let mut scratch: Vec<OpId> = Vec::new();
            data.op_destroy_recursive(remove_op, &mut scratch);
        }
        Ok(1)
    }
}

// =============================================================================
// SubfloatFlow / RuleSubfloatConvert (subflow.cc:3085-3522)
// =============================================================================
//
// SubfloatFlow traces a lower-precision float value through higher-precision
// storage.  The trace/precision engine is ported verbatim; the FloatFormat
// acquisition (`f->getArch()->translate->getFloatFormat`) is an arch seam (see
// [`subfloat_float_format`]) and SubfloatFlow's `preserveAddress` override
// (`return vn->isInput()`) cannot be injected into the merged
// `TransformManager::new_piece` (transform.rs, not owned) — both recorded as
// losses.  The constant-conversion in `setReplacement` (`format->convertEncoding`)
// is reachable only with a real FloatFormat and so is seam-gated.

/// Internal state for walking floating-point data-flow (C++
/// `SubfloatFlow::State`).
#[derive(Debug, Clone, Copy)]
struct SubfloatState {
    /// Operation being traversed (C++ `op`).
    op: OpId,
    /// Input edge being traversed (C++ `slot`).
    slot: int4,
    /// Maximum precision traversed through inputs so far (C++ `maxPrecision`).
    max_precision: int4,
}

impl SubfloatState {
    fn new(op: OpId) -> SubfloatState {
        SubfloatState { op, slot: 0, max_precision: 0 }
    }
    /// Accumulate precision from an input (C++ `State::incorporateInputSize`).
    fn incorporate_input_size(&mut self, sz: int4) {
        self.max_precision = if self.max_precision < sz { sz } else { self.max_precision };
    }
}

/// Trace changes of precision in floating point variables (C++ `SubfloatFlow`).
pub struct SubfloatFlow {
    /// The transform-manager state (C++ base class).
    tm: TransformManager,
    /// Number of bytes of precision in the logical flow (C++ `precision`).
    precision: int4,
    /// Number of terminating nodes reachable via the root (C++ `terminatorCount`).
    terminator_count: int4,
    /// True if a usable FloatFormat was found (C++ `format != 0`).  The actual
    /// FloatFormat lives behind the arch seam (see [`subfloat_float_format`]).
    has_format: bool,
    /// Current list of placeholders still to be traced (C++ `worklist`).
    worklist: Vec<TVarRef>,
    /// Maximum precision flowing into a particular float op (C++ `maxPrecisionMap`).
    max_precision_map: BTreeMap<OpId, int4>,
}

impl SubfloatFlow {
    /// Construct the engine for a function/root/precision (C++
    /// `SubfloatFlow::SubfloatFlow`, subflow.cc:3456).
    ///
    /// SEAM(arch): `format = f->getArch()->translate->getFloatFormat(precision)`
    /// is unavailable off the Funcdata arch seam, so `has_format` is whatever
    /// [`subfloat_float_format`] reports and `setReplacement(root)` is only run
    /// when a format is present (matching the C++ early return on null format).
    pub fn new(data: &mut Funcdata, root: VarnodeId, prec: int4) -> SubfloatFlow {
        let mut sf = SubfloatFlow {
            tm: TransformManager::new(),
            precision: prec,
            terminator_count: 0,
            has_format: subfloat_float_format(data, prec),
            worklist: Vec::new(),
            max_precision_map: BTreeMap::new(),
        };
        if !sf.has_format {
            return sf; // C++: if (format == 0) return;
        }
        let _ = sf.set_replacement(data, root);
        sf
    }

    /// Maximum floating-point precision reaching a Varnode (C++
    /// `SubfloatFlow::maxPrecision`, subflow.cc:3094).
    fn max_precision(&mut self, data: &mut Funcdata, vn: VarnodeId) -> int4 {
        if !data.vbank().get(vn).expect("max_precision: stale vn").is_written() {
            return data.vbank().get(vn).expect("stale vn").get_size();
        }
        let op = data.vbank().get(vn).expect("stale vn").get_def().expect("vn def");
        let vn_size = data.vbank().get(vn).expect("stale vn").get_size();
        match data.obank().get(op).expect("stale op").code() {
            OpCode::CPUI_MULTIEQUAL
            | OpCode::CPUI_FLOAT_NEG
            | OpCode::CPUI_FLOAT_ABS
            | OpCode::CPUI_FLOAT_SQRT
            | OpCode::CPUI_FLOAT_CEIL
            | OpCode::CPUI_FLOAT_FLOOR
            | OpCode::CPUI_FLOAT_ROUND
            | OpCode::CPUI_COPY => {}
            OpCode::CPUI_FLOAT_ADD
            | OpCode::CPUI_FLOAT_SUB
            | OpCode::CPUI_FLOAT_MULT
            | OpCode::CPUI_FLOAT_DIV => return 0, // Delay checking other binary ops
            OpCode::CPUI_FLOAT_FLOAT2FLOAT | OpCode::CPUI_FLOAT_INT2FLOAT => {
                let in0 = data.obank().get(op).expect("stale op").get_in(0).expect("in0");
                let in0_size = data.vbank().get(in0).expect("stale in0").get_size();
                if in0_size > vn_size {
                    return vn_size;
                }
                return in0_size;
            }
            _ => return vn_size,
        }

        if let Some(v) = self.max_precision_map.get(&op) {
            return *v;
        }
        let mut op_stack: Vec<SubfloatState> = Vec::new();
        op_stack.push(SubfloatState::new(op));
        data.obank_mut().get_mut(op).expect("stale op").set_mark();
        let mut max = 0i32;
        while !op_stack.is_empty() {
            let st_idx = op_stack.len() - 1;
            let state_op = op_stack[st_idx].op;
            let state_slot = op_stack[st_idx].slot;
            let numin = data.obank().get(state_op).expect("stale state op").num_input();
            if state_slot >= numin {
                max = op_stack[st_idx].max_precision;
                data.obank_mut().get_mut(state_op).expect("stale state op").clear_mark();
                self.max_precision_map.insert(state_op, op_stack[st_idx].max_precision);
                op_stack.pop();
                if let Some(last) = op_stack.last_mut() {
                    last.incorporate_input_size(max);
                }
                continue;
            }
            let next_vn = data.obank().get(state_op).expect("stale state op").get_in(state_slot).expect("in");
            op_stack[st_idx].slot += 1;
            if !data.vbank().get(next_vn).expect("stale next_vn").is_written() {
                let sz = data.vbank().get(next_vn).expect("stale next_vn").get_size();
                op_stack[st_idx].incorporate_input_size(sz);
                continue;
            }
            let next_op = data.vbank().get(next_vn).expect("stale next_vn").get_def().expect("next def");
            if data.obank().get(next_op).expect("stale next_op").is_mark() {
                continue; // Truncate the cycle edge
            }
            let next_vn_size = data.vbank().get(next_vn).expect("stale next_vn").get_size();
            match data.obank().get(next_op).expect("stale next_op").code() {
                OpCode::CPUI_MULTIEQUAL
                | OpCode::CPUI_FLOAT_NEG
                | OpCode::CPUI_FLOAT_ABS
                | OpCode::CPUI_FLOAT_SQRT
                | OpCode::CPUI_FLOAT_CEIL
                | OpCode::CPUI_FLOAT_FLOOR
                | OpCode::CPUI_FLOAT_ROUND
                | OpCode::CPUI_COPY => {
                    if let Some(v) = self.max_precision_map.get(&next_op) {
                        // Seen the op before, incorporate cached precision
                        op_stack[st_idx].incorporate_input_size(*v);
                    } else {
                        data.obank_mut().get_mut(next_op).expect("stale next_op").set_mark();
                        op_stack.push(SubfloatState::new(next_op)); // Recursively push
                    }
                }
                OpCode::CPUI_FLOAT_ADD
                | OpCode::CPUI_FLOAT_SUB
                | OpCode::CPUI_FLOAT_MULT
                | OpCode::CPUI_FLOAT_DIV => {}
                OpCode::CPUI_FLOAT_FLOAT2FLOAT | OpCode::CPUI_FLOAT_INT2FLOAT => {
                    let nin0 = data.obank().get(next_op).expect("stale next_op").get_in(0).expect("in0");
                    let nin0_size = data.vbank().get(nin0).expect("stale nin0").get_size();
                    if nin0_size > next_vn_size {
                        op_stack[st_idx].incorporate_input_size(next_vn_size);
                    } else {
                        op_stack[st_idx].incorporate_input_size(nin0_size);
                    }
                }
                _ => op_stack[st_idx].incorporate_input_size(next_vn_size),
            }
        }
        max
    }

    /// Determine if the given binary float op exceeds our precision (C++
    /// `SubfloatFlow::exceedsPrecision`, subflow.cc:3201).
    fn exceeds_precision(&mut self, data: &mut Funcdata, op: OpId) -> bool {
        let in0 = data.obank().get(op).expect("stale op").get_in(0).expect("in0");
        let in1 = data.obank().get(op).expect("stale op").get_in(1).expect("in1");
        let val1 = self.max_precision(data, in0);
        let val2 = self.max_precision(data, in1);
        let min = if val1 < val2 { val1 } else { val2 };
        min > self.precision
    }

    /// Create/return a placeholder for a Varnode (C++
    /// `SubfloatFlow::setReplacement`, subflow.cc:3215).  `None` is the C++
    /// `(TransformVar *)0`.
    fn set_replacement(&mut self, data: &mut Funcdata, vn: VarnodeId) -> KunaResult<Option<TVarRef>> {
        let v = data.vbank().get(vn).expect("set_replacement: stale vn");
        if v.is_mark() {
            // Already seen before
            return Ok(Some(self.tm.get_piece(data, vn, self.precision * 8, 0)?));
        }

        if v.is_constant() {
            // const FloatFormat *form2 = translate->getFloatFormat(vn->getSize());
            // return newConstant(precision, 0, format->convertEncoding(vn->getOffset(), form2));
            // SEAM(arch): both float formats come off the unported arch seam.
            return subfloat_convert_constant_seam();
        }

        if v.is_free() {
            return Ok(None); // Abort
        }
        if v.is_addr_force() && v.get_size() != self.precision {
            return Ok(None);
        }
        if v.is_type_lock() && v.get_type().get_metatype() != type_metatype::TYPE_PARTIALSTRUCT {
            let sz = v.get_type().get_size();
            if sz != self.precision {
                return Ok(None);
            }
        }
        if v.is_input() && v.get_size() != self.precision {
            return Ok(None); // Must be careful with inputs
        }

        let vn_size = v.get_size();
        data.vbank_mut().get_mut(vn).expect("stale vn").set_mark();
        let res = if vn_size == self.precision {
            // Already represents the logical variable being traced
            self.tm.new_preexisting_varnode(data, vn)
        } else {
            let r = self.tm.new_piece(data, vn, self.precision * 8, 0);
            self.worklist.push(r);
            r
        };
        Ok(Some(res))
    }

    /// Trace logical value forward through descendant Varnodes (C++
    /// `SubfloatFlow::traceForward`, subflow.cc:3264).
    fn trace_forward(&mut self, data: &mut Funcdata, rvn: TVarRef) -> KunaResult<bool> {
        let vn = self.tm.var(rvn).get_original().expect("trace_forward: rvn original");
        let descend: Vec<OpId> = data.descend_snapshot(vn);
        for (idx, &op) in descend.iter().enumerate() {
            let outvn = data.obank().get(op).expect("stale op").get_out();
            if let Some(ov) = outvn {
                if data.vbank().get(ov).expect("stale outvn").is_mark() {
                    continue;
                }
            }
            let code = data.obank().get(op).expect("stale op").code();
            match code {
                OpCode::CPUI_FLOAT_ADD
                | OpCode::CPUI_FLOAT_SUB
                | OpCode::CPUI_FLOAT_MULT
                | OpCode::CPUI_FLOAT_DIV
                | OpCode::CPUI_MULTIEQUAL
                | OpCode::CPUI_COPY
                | OpCode::CPUI_FLOAT_CEIL
                | OpCode::CPUI_FLOAT_FLOOR
                | OpCode::CPUI_FLOAT_ROUND
                | OpCode::CPUI_FLOAT_NEG
                | OpCode::CPUI_FLOAT_ABS
                | OpCode::CPUI_FLOAT_SQRT => {
                    if matches!(
                        code,
                        OpCode::CPUI_FLOAT_ADD
                            | OpCode::CPUI_FLOAT_SUB
                            | OpCode::CPUI_FLOAT_MULT
                            | OpCode::CPUI_FLOAT_DIV
                    ) && self.exceeds_precision(data, op)
                    {
                        return Ok(false);
                    }
                    let numin = data.obank().get(op).expect("stale op").num_input();
                    let rop = self.tm.new_op_replace(numin, code, op);
                    let ov = outvn.expect("trace_forward: out");
                    let outrvn = match self.set_replacement(data, ov)? {
                        Some(o) => o,
                        None => return Ok(false),
                    };
                    let slot = data.obank().get(op).expect("stale op").get_slot(vn);
                    self.tm.op_set_input(rop, rvn, slot);
                    self.tm.op_set_output(rop, outrvn);
                }
                OpCode::CPUI_FLOAT_FLOAT2FLOAT => {
                    let ov = outvn.expect("trace_forward: out");
                    let out_size = data.vbank().get(ov).expect("stale out").get_size();
                    if out_size < self.precision {
                        return Ok(false);
                    }
                    let new_code = if out_size == self.precision {
                        OpCode::CPUI_COPY
                    } else {
                        OpCode::CPUI_FLOAT_FLOAT2FLOAT
                    };
                    let rop = self.tm.new_preexisting_op(1, new_code, op);
                    self.tm.op_set_input(rop, rvn, 0);
                    self.terminator_count += 1;
                }
                OpCode::CPUI_FLOAT_EQUAL
                | OpCode::CPUI_FLOAT_NOTEQUAL
                | OpCode::CPUI_FLOAT_LESS
                | OpCode::CPUI_FLOAT_LESSEQUAL => {
                    if self.exceeds_precision(data, op) {
                        return Ok(false);
                    }
                    let mut slot = data.obank().get(op).expect("stale op").get_slot(vn);
                    let other = data.obank().get(op).expect("stale op").get_in(1 - slot).expect("other in");
                    let rvn2 = match self.set_replacement(data, other)? {
                        Some(r) => r,
                        None => return Ok(false),
                    };
                    if rvn == rvn2 {
                        // Back up one to our original iterator (descend[..idx]).
                        slot = data
                            .obank()
                            .get(op)
                            .expect("stale op")
                            .get_repeat_slot(vn, slot, &descend[..idx], op);
                    }
                    if self.tm.preexisting_guard(slot, rvn2) {
                        let rop = self.tm.new_preexisting_op(2, code, op);
                        self.tm.op_set_input(rop, rvn, slot);
                        self.tm.op_set_input(rop, rvn2, 1 - slot);
                        self.terminator_count += 1;
                    }
                }
                OpCode::CPUI_FLOAT_TRUNC | OpCode::CPUI_FLOAT_NAN => {
                    let rop = self.tm.new_preexisting_op(1, code, op);
                    self.tm.op_set_input(rop, rvn, 0);
                    self.terminator_count += 1;
                }
                _ => return Ok(false),
            }
        }
        Ok(true)
    }

    /// Trace a logical value backward through its defining op (C++
    /// `SubfloatFlow::traceBackward`, subflow.cc:3354).
    fn trace_backward(&mut self, data: &mut Funcdata, rvn: TVarRef) -> KunaResult<bool> {
        let origvn = self.tm.var(rvn).get_original().expect("trace_backward: rvn original");
        let op = match data.vbank().get(origvn).expect("stale origvn").get_def() {
            Some(o) => o,
            None => return Ok(true), // If vn is input
        };
        let code = data.obank().get(op).expect("stale op").code();
        match code {
            OpCode::CPUI_FLOAT_ADD
            | OpCode::CPUI_FLOAT_SUB
            | OpCode::CPUI_FLOAT_MULT
            | OpCode::CPUI_FLOAT_DIV
            | OpCode::CPUI_COPY
            | OpCode::CPUI_FLOAT_CEIL
            | OpCode::CPUI_FLOAT_FLOOR
            | OpCode::CPUI_FLOAT_ROUND
            | OpCode::CPUI_FLOAT_NEG
            | OpCode::CPUI_FLOAT_ABS
            | OpCode::CPUI_FLOAT_SQRT
            | OpCode::CPUI_MULTIEQUAL => {
                if matches!(
                    code,
                    OpCode::CPUI_FLOAT_ADD
                        | OpCode::CPUI_FLOAT_SUB
                        | OpCode::CPUI_FLOAT_MULT
                        | OpCode::CPUI_FLOAT_DIV
                ) && self.exceeds_precision(data, op)
                {
                    return Ok(false);
                }
                let mut rop = self.tm.var(rvn).get_def();
                if rop.is_none() {
                    let numin = data.obank().get(op).expect("stale op").num_input();
                    let r = self.tm.new_op_replace(numin, code, op);
                    self.tm.op_set_output(r, rvn);
                    rop = Some(r);
                }
                let rop = rop.expect("trace_backward: rop");
                let numin = data.obank().get(op).expect("stale op").num_input();
                for i in 0..numin {
                    if self.tm.op(rop).get_in(i).is_none() {
                        let ini = data.obank().get(op).expect("stale op").get_in(i).expect("in");
                        let newvar = match self.set_replacement(data, ini)? {
                            Some(v) => v,
                            None => return Ok(false),
                        };
                        self.tm.op_set_input(rop, newvar, i);
                    }
                }
                Ok(true)
            }
            OpCode::CPUI_FLOAT_INT2FLOAT => {
                let invn = data.obank().get(op).expect("stale op").get_in(0).expect("in0");
                let iv = data.vbank().get(invn).expect("stale invn");
                if !iv.is_constant() && iv.is_free() {
                    return Ok(false);
                }
                let rop = self.tm.new_op_replace(1, OpCode::CPUI_FLOAT_INT2FLOAT, op);
                self.tm.op_set_output(rop, rvn);
                let newvar = self.tm.get_preexisting_varnode(data, invn);
                self.tm.op_set_input(rop, newvar, 0);
                Ok(true)
            }
            OpCode::CPUI_FLOAT_FLOAT2FLOAT => {
                let invn = data.obank().get(op).expect("stale op").get_in(0).expect("in0");
                let iv = data.vbank().get(invn).expect("stale invn");
                let newvar: TVarRef;
                let opc: OpCode;
                if iv.is_constant() {
                    opc = OpCode::CPUI_COPY;
                    if iv.get_size() == self.precision {
                        let off = iv.get_offset();
                        newvar = self.tm.new_constant(self.precision, 0, off);
                    } else {
                        newvar = match self.set_replacement(data, invn)? {
                            Some(v) => v,
                            None => return Ok(false), // Unsupported float format
                        };
                    }
                } else {
                    if iv.is_free() {
                        return Ok(false);
                    }
                    opc = if iv.get_size() == self.precision {
                        OpCode::CPUI_COPY
                    } else {
                        OpCode::CPUI_FLOAT_FLOAT2FLOAT
                    };
                    newvar = self.tm.get_preexisting_varnode(data, invn);
                }
                let rop = self.tm.new_op_replace(1, opc, op);
                self.tm.op_set_output(rop, rvn);
                self.tm.op_set_input(rop, newvar, 0);
                Ok(true)
            }
            _ => Ok(false), // Everything else we abort
        }
    }

    /// Push the trace one hop from the worklist top (C++
    /// `SubfloatFlow::processNextWork`, subflow.cc:3442).
    fn process_next_work(&mut self, data: &mut Funcdata) -> KunaResult<bool> {
        let rvn = self.worklist.pop().expect("process_next_work: empty worklist");
        if !self.trace_backward(data, rvn)? {
            return Ok(false);
        }
        self.trace_forward(data, rvn)
    }

    /// Trace logical value as far as possible (C++ `SubfloatFlow::doTrace`,
    /// subflow.cc:3477).
    pub fn do_trace(&mut self, data: &mut Funcdata) -> KunaResult<bool> {
        if !self.has_format {
            return Ok(false);
        }
        self.terminator_count = 0; // Have seen no terminators
        let mut retval = true;
        while !self.worklist.is_empty() {
            if !self.process_next_work(data)? {
                retval = false;
                break;
            }
        }
        self.tm.clear_varnode_marks(data);
        if !retval {
            return Ok(false);
        }
        if self.terminator_count == 0 {
            return Ok(false); // Must see at least 1 terminator
        }
        Ok(true)
    }

    /// Apply the constructed transform (C++ base `TransformManager::apply`).
    /// SEAM(W6): the merged apply reaches `glb->inst[opc]`.
    pub fn apply(&mut self, data: &mut Funcdata) -> KunaResult<()> {
        self.tm.apply(data)
    }
}

/// SEAM(arch): `f->getArch()->translate->getFloatFormat(precision)` is not on the
/// Funcdata arch seam (the stub `Architecture` carries no `Translate`).  Report
/// "no format" so SubfloatFlow degrades to the C++ null-format early return.
fn subfloat_float_format(_data: &Funcdata, _precision: int4) -> bool {
    false
}

/// SEAM(arch): the constant-conversion branch of `SubfloatFlow::setReplacement`
/// needs `translate->getFloatFormat` + `format->convertEncoding`, both off the
/// unported arch seam.  Unreachable while [`subfloat_float_format`] reports no
/// format (the engine never traces); kept for faithful dispatch.
fn subfloat_convert_constant_seam() -> KunaResult<Option<TVarRef>> {
    Err(KunaError::lowlevel(
        "kuna rust port: SubfloatFlow::setReplacement constant conversion needs \
         translate->getFloatFormat + FloatFormat::convertEncoding — SEAM(arch)",
    ))
}

/// Perform SubfloatFlow analysis triggered by FLOAT_FLOAT2FLOAT (C++
/// `RuleSubfloatConvert`).
pub struct RuleSubfloatConvert {
    group: String,
}

impl RuleSubfloatConvert {
    /// Construct with the rule's stage group (C++
    /// `RuleSubfloatConvert::RuleSubfloatConvert`).
    pub fn new(g: impl Into<String>) -> RuleSubfloatConvert {
        RuleSubfloatConvert { group: g.into() }
    }
}

impl Rule for RuleSubfloatConvert {
    fn get_op_list(&self) -> Vec<OpCode> {
        vec![OpCode::CPUI_FLOAT_FLOAT2FLOAT]
    }

    fn clone_rule(&self, grouplist: &ActionGroupList) -> Option<Box<dyn Rule>> {
        if !grouplist.contains(&self.group) {
            return None;
        }
        Some(Box::new(RuleSubfloatConvert::new(self.group.clone())))
    }

    fn apply_op(&mut self, op: OpId, data: &mut Funcdata) -> int4 {
        let invn = data.obank().get(op).expect("stale op").get_in(0).expect("in0");
        let outvn = data.obank().get(op).expect("stale op").get_out().expect("out");
        let insize = data.vbank().get(invn).expect("stale invn").get_size();
        let outsize = data.vbank().get(outvn).expect("stale outvn").get_size();
        if outsize > insize {
            let mut subflow = SubfloatFlow::new(data, outvn, insize);
            match subflow.do_trace(data) {
                Ok(true) => {}
                _ => return 0,
            }
            match subflow.apply(data) {
                Ok(()) => {}
                // SEAM(W6): TransformManager::apply → glb->inst[opc].
                Err(_) => return 0,
            }
        } else {
            let mut subflow = SubfloatFlow::new(data, invn, outsize);
            match subflow.do_trace(data) {
                Ok(true) => {}
                _ => return 0,
            }
            match subflow.apply(data) {
                Ok(()) => {}
                Err(_) => return 0,
            }
        }
        1
    }
}

// =============================================================================
// Rule registration (C++ definition order)
// =============================================================================

/// The rules defined in this file, in C++ definition order (W8 assembles
/// `universalAction` registration from these — see `action.rs` `RuleSpec`).
///
/// Order matches `subflow.hh`/`subflow.cc` Rule declaration order:
/// `subvar_and`, `subvar_subpiece`, `subvar_compzero`, `subvar_shift`,
/// `subvar_zext`, `subvar_sext`, then (item `w8x-subflow-splits`) `splitflow`,
/// `splitcopy`, `splitload`, `splitstore`, `dumptyhumplate`, `subfloat_convert`.
/// `LaneDivide` has no trigger Rule and is not listed.  Each entry keeps the
/// exact C++ `name()` / `getOpList()`; the registering pool (universalAction)
/// supplies the real stage-group string (the per-spec `group` here is a
/// placeholder, as the W5 subvar entries already document).
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
        RuleSpec { group: "splitflow_grp", ctor: || Box::new(RuleSplitFlow::new("splitflow_grp")) },
        RuleSpec { group: "splitcopy_grp", ctor: || Box::new(RuleSplitCopy::new("splitcopy_grp")) },
        RuleSpec { group: "splitload_grp", ctor: || Box::new(RuleSplitLoad::new("splitload_grp")) },
        RuleSpec { group: "splitstore_grp", ctor: || Box::new(RuleSplitStore::new("splitstore_grp")) },
        RuleSpec {
            group: "dumptyhumplate_grp",
            ctor: || Box::new(RuleDumptyHumpLate::new("dumptyhumplate_grp")),
        },
        RuleSpec {
            group: "subfloat_convert_grp",
            ctor: || Box::new(RuleSubfloatConvert::new("subfloat_convert_grp")),
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

    use crate::dtype::{type_metatype, Datatype, TypeFactory};
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
        // w8x-subflow-splits rules.
        assert_eq!(RuleSplitFlow::new("g").get_op_list(), vec![OpCode::CPUI_SUBPIECE]);
        assert_eq!(RuleSplitCopy::new("g").get_op_list(), vec![OpCode::CPUI_COPY]);
        assert_eq!(RuleSplitLoad::new("g").get_op_list(), vec![OpCode::CPUI_LOAD]);
        assert_eq!(RuleSplitStore::new("g").get_op_list(), vec![OpCode::CPUI_STORE]);
        assert_eq!(RuleDumptyHumpLate::new("g").get_op_list(), vec![OpCode::CPUI_SUBPIECE]);
        assert_eq!(
            RuleSubfloatConvert::new("g").get_op_list(),
            vec![OpCode::CPUI_FLOAT_FLOAT2FLOAT]
        );
    }

    #[test]
    fn specs_lists_rules_in_definition_order() {
        let s = specs();
        assert_eq!(s.len(), 12);
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
        assert_eq!(lists[6], vec![OpCode::CPUI_SUBPIECE]); // splitflow
        assert_eq!(lists[7], vec![OpCode::CPUI_COPY]); // splitcopy
        assert_eq!(lists[8], vec![OpCode::CPUI_LOAD]); // splitload
        assert_eq!(lists[9], vec![OpCode::CPUI_STORE]); // splitstore
        assert_eq!(lists[10], vec![OpCode::CPUI_SUBPIECE]); // dumptyhumplate
        assert_eq!(lists[11], vec![OpCode::CPUI_FLOAT_FLOAT2FLOAT]); // subfloat_convert
    }

    #[test]
    fn clone_rule_filters_by_group() {
        let rule = RuleSubvarAnd::new("mygroup");
        let yes = ActionGroupList::from_names(["mygroup"]);
        let no = ActionGroupList::from_names(["other"]);
        assert!(rule.clone_rule(&yes).is_some());
        assert!(rule.clone_rule(&no).is_none());
    }

    // ===== w8x-subflow-splits =================================================

    fn mk_const(fd: &mut Funcdata, size: int4, val: u64) -> VarnodeId {
        fd.new_constant(size, val)
    }

    // ---- SplitFlow / RuleSplitFlow (subflow.cc:1754-2088) ------------------

    #[test]
    fn split_flow_lane_above_advances_piece() {
        // The C++ `rvn+1` pointer arithmetic maps to advancing the piece index.
        let r = lane_above(TVarRef::Piece { key: 7, idx: 0 });
        assert_eq!(r, TVarRef::Piece { key: 7, idx: 1 });
    }

    #[test]
    fn split_flow_traces_subpiece_of_piece() {
        // whole = PIECE(hi, lo);  out = SUBPIECE(whole, 4)  -> grabs the high lane.
        // SplitFlow from `whole` with lowSize=4 traces backward through the PIECE
        // (low/high COPY lanes) and forward through the SUBPIECE (high-lane COPY).
        // doTrace succeeds (the discovery half is fully ported).
        let mut fd = build_fd();
        let lo = mk_input(&mut fd, 0x40, 4);
        let hi = mk_input(&mut fd, 0x48, 4);
        // whole = PIECE(hi, lo)
        let pieceop = mk_op(&mut fd, 0x100, 2, OpCode::CPUI_PIECE);
        let whole = mk_reg(&mut fd, 0x80, 8);
        wire_in(&mut fd, pieceop, hi, 0); // most significant
        wire_in(&mut fd, pieceop, lo, 1); // least significant
        let whole = wire_out(&mut fd, pieceop, whole);
        // out = SUBPIECE(whole, 4)  (most significant 4 bytes)
        let four = mk_const(&mut fd, 4, 4);
        let sub = mk_op(&mut fd, 0x180, 2, OpCode::CPUI_SUBPIECE);
        let outvn = mk_reg(&mut fd, 0xc0, 4);
        wire_in(&mut fd, sub, whole, 0);
        wire_in(&mut fd, sub, four, 1);
        let _outvn = wire_out(&mut fd, sub, outvn);

        let mut sf = SplitFlow::new(&mut fd, whole, 4);
        assert!(sf.do_trace(&mut fd));
        // Marks cleared after doTrace.
        assert!(!fd.vbank().get(whole).unwrap().is_mark());
    }

    #[test]
    fn split_flow_aborts_on_unsplittable_use() {
        // whole = PIECE(hi, lo); used by an op SplitFlow can't push through
        // (FLOAT_ADD) -> traceForward returns false -> doTrace false.
        let mut fd = build_fd();
        let lo = mk_input(&mut fd, 0x40, 4);
        let hi = mk_input(&mut fd, 0x48, 4);
        let pieceop = mk_op(&mut fd, 0x100, 2, OpCode::CPUI_PIECE);
        let whole = mk_reg(&mut fd, 0x80, 8);
        wire_in(&mut fd, pieceop, hi, 0);
        wire_in(&mut fd, pieceop, lo, 1);
        let whole = wire_out(&mut fd, pieceop, whole);
        let fadd = mk_op(&mut fd, 0x180, 2, OpCode::CPUI_FLOAT_ADD);
        let other = mk_input(&mut fd, 0x90, 8);
        let outvn = mk_reg(&mut fd, 0xc0, 8);
        wire_in(&mut fd, fadd, whole, 0);
        wire_in(&mut fd, fadd, other, 1);
        let _outvn = wire_out(&mut fd, fadd, outvn);

        let mut sf = SplitFlow::new(&mut fd, whole, 4);
        assert!(!sf.do_trace(&mut fd));
    }

    #[test]
    fn rule_split_flow_rejects_low_subpiece() {
        // SUBPIECE taking the least significant part (offset 0) -> rule returns 0.
        let mut fd = build_fd();
        let vn = mk_input(&mut fd, 0x40, 8);
        let zero = mk_const(&mut fd, 4, 0);
        let sub = mk_op(&mut fd, 0x100, 2, OpCode::CPUI_SUBPIECE);
        let outvn = mk_reg(&mut fd, 0x80, 4);
        wire_in(&mut fd, sub, vn, 0);
        wire_in(&mut fd, sub, zero, 1);
        let _outvn = wire_out(&mut fd, sub, outvn);
        let mut rule = RuleSplitFlow::new("g");
        assert_eq!(rule.apply_op(sub, &mut fd), 0);
    }

    #[test]
    fn rule_split_flow_rejects_without_concat() {
        // SUBPIECE of the most significant part but the source is not written by a
        // PIECE (it's a plain input) -> no concat found -> rule returns 0.
        let mut fd = build_fd();
        let copyop = mk_op(&mut fd, 0x100, 1, OpCode::CPUI_COPY);
        let src = mk_input(&mut fd, 0x40, 8);
        let whole = mk_reg(&mut fd, 0x80, 8);
        wire_in(&mut fd, copyop, src, 0);
        let whole = wire_out(&mut fd, copyop, whole);
        let four = mk_const(&mut fd, 4, 4);
        let sub = mk_op(&mut fd, 0x180, 2, OpCode::CPUI_SUBPIECE);
        let outvn = mk_reg(&mut fd, 0xc0, 4);
        wire_in(&mut fd, sub, whole, 0);
        wire_in(&mut fd, sub, four, 1);
        let _outvn = wire_out(&mut fd, sub, outvn);
        let mut rule = RuleSplitFlow::new("g");
        assert_eq!(rule.apply_op(sub, &mut fd), 0);
    }

    // ---- RuleDumptyHumpLate (subflow.cc:3021-3083) ------------------------

    #[test]
    fn dumpty_hump_late_sub_concat_low_replaces_with_component() {
        // whole = PIECE(V, W);  out = SUBPIECE(whole, 0)  with out size == W size.
        // RuleDumptyHumpLate backtracks thru the PIECE to W (exact match), and,
        // since the SUBPIECE output is not addr-tied, totalReplaces out with W.
        let mut fd = build_fd();
        let v = mk_input(&mut fd, 0x40, 4); // most significant component
        let w = mk_input(&mut fd, 0x48, 4); // least significant component
        let pieceop = mk_op(&mut fd, 0x100, 2, OpCode::CPUI_PIECE);
        let whole = mk_reg(&mut fd, 0x80, 8);
        wire_in(&mut fd, pieceop, v, 0); // most significant
        wire_in(&mut fd, pieceop, w, 1); // least significant
        let whole = wire_out(&mut fd, pieceop, whole);
        // out = SUBPIECE(whole, 0)  (least significant 4 bytes == W)
        let zero = mk_const(&mut fd, 4, 0);
        let sub = mk_op(&mut fd, 0x180, 2, OpCode::CPUI_SUBPIECE);
        let outvn = mk_reg(&mut fd, 0xc0, 4);
        wire_in(&mut fd, sub, whole, 0);
        wire_in(&mut fd, sub, zero, 1);
        let outvn = wire_out(&mut fd, sub, outvn);
        // Give the output a descendant so the SUBPIECE op survives the destroy.
        let user = mk_op(&mut fd, 0x200, 1, OpCode::CPUI_COPY);
        wire_in(&mut fd, user, outvn, 0);

        let mut rule = RuleDumptyHumpLate::new("g");
        assert_eq!(rule.apply_op(sub, &mut fd), 1);
        // The user now reads W directly (totalReplace).
        assert_eq!(fd.obank().get(user).unwrap().get_in(0), Some(w));
    }

    #[test]
    fn dumpty_hump_late_sub_concat_high_preserves_subpiece() {
        // whole = PIECE(V, W);  out = SUBPIECE(whole, 6) with out size 2.  The
        // truncation lands inside V (the most significant component), so the rule
        // backtracks to V and rewrites the SUBPIECE to read V with the adjusted
        // truncation (6 - W.size = 2) — SUBPIECE preserved, in0 now V.
        let mut fd = build_fd();
        let v = mk_input(&mut fd, 0x40, 4);
        let w = mk_input(&mut fd, 0x48, 4);
        let pieceop = mk_op(&mut fd, 0x100, 2, OpCode::CPUI_PIECE);
        let whole = mk_reg(&mut fd, 0x80, 8);
        wire_in(&mut fd, pieceop, v, 0);
        wire_in(&mut fd, pieceop, w, 1);
        let whole = wire_out(&mut fd, pieceop, whole);
        let six = mk_const(&mut fd, 4, 6);
        let sub = mk_op(&mut fd, 0x180, 2, OpCode::CPUI_SUBPIECE);
        let outvn = mk_reg(&mut fd, 0xc0, 2);
        wire_in(&mut fd, sub, whole, 0);
        wire_in(&mut fd, sub, six, 1);
        let outvn = wire_out(&mut fd, sub, outvn);
        let user = mk_op(&mut fd, 0x200, 1, OpCode::CPUI_COPY);
        wire_in(&mut fd, user, outvn, 0);

        let mut rule = RuleDumptyHumpLate::new("g");
        assert_eq!(rule.apply_op(sub, &mut fd), 1);
        // SUBPIECE now reads V (the most significant component) at truncation 2.
        assert_eq!(fd.obank().get(sub).unwrap().get_in(0), Some(v));
        assert_eq!(fd.obank().get(sub).unwrap().code(), OpCode::CPUI_SUBPIECE);
        let nin1 = fd.obank().get(sub).unwrap().get_in(1).unwrap();
        assert_eq!(fd.vbank().get(nin1).unwrap().get_offset(), 2);
    }

    #[test]
    fn dumpty_hump_late_rejects_non_piece_input() {
        // SUBPIECE whose input is not written by a PIECE -> rule returns 0.
        let mut fd = build_fd();
        let src = mk_input(&mut fd, 0x40, 8);
        let copyop = mk_op(&mut fd, 0x100, 1, OpCode::CPUI_COPY);
        let mid = mk_reg(&mut fd, 0x80, 8);
        wire_in(&mut fd, copyop, src, 0);
        let mid = wire_out(&mut fd, copyop, mid);
        let zero = mk_const(&mut fd, 4, 0);
        let sub = mk_op(&mut fd, 0x180, 2, OpCode::CPUI_SUBPIECE);
        let outvn = mk_reg(&mut fd, 0xc0, 4);
        wire_in(&mut fd, sub, mid, 0);
        wire_in(&mut fd, sub, zero, 1);
        let _outvn = wire_out(&mut fd, sub, outvn);
        let mut rule = RuleDumptyHumpLate::new("g");
        assert_eq!(rule.apply_op(sub, &mut fd), 0);
    }

    // ---- SplitDatatype categorization (subflow.cc:2219-2384) --------------

    #[test]
    fn split_datatype_categorizes_primitive_and_struct() {
        use crate::dtype::TypeFactoryImpl;
        let factory = TypeFactoryImpl::new();
        factory.set_default_alignment_map();
        factory.set_max_basetype_size(8);
        // Structures off / arrays off -> nothing is splittable except primitives.
        let sd = SplitDatatype::new(0);
        let int4t = factory.get_base(4, type_metatype::TYPE_INT).unwrap();
        assert_eq!(sd.categorize_datatype(&int4t), 2); // TYPE_INT -> primitive
        let unk2 = unk(2);
        assert_eq!(sd.categorize_datatype(&unk2), 2); // TYPE_UNKNOWN -> primitive
    }

    #[test]
    fn split_datatype_array_compatibility_splits_primitive_into_elements() {
        use crate::dtype::TypeFactoryImpl;
        let factory = TypeFactoryImpl::new();
        factory.set_default_alignment_map();
        factory.set_max_basetype_size(8);
        // out = int4[2] (size 8); in = unknown8 (primitive).  With arrays enabled,
        // testDatatypeCompatibility splits the primitive into the two int4 lanes.
        let int4t = factory.get_base(4, type_metatype::TYPE_INT).unwrap();
        let arr = factory.get_type_array(2, Rc::clone(&int4t)).unwrap();
        let unk8 = factory.get_base(8, type_metatype::TYPE_UNKNOWN).unwrap();
        // option_array bit set (config == 2).
        let mut sd = SplitDatatype::new(2);
        assert_eq!(sd.categorize_datatype(&arr), 1); // array
        assert_eq!(sd.categorize_datatype(&unk8), 2); // primitive
        let ok = sd
            .test_datatype_compatibility(&factory, &unk8, &arr, false)
            .unwrap();
        assert!(ok);
        assert_eq!(sd.data_type_pieces.len(), 2);
        assert_eq!(sd.data_type_pieces[0].offset, 0);
        assert_eq!(sd.data_type_pieces[1].offset, 4);
        assert_eq!(sd.data_type_pieces[0].out_type.get_size(), 4);
        // getComponent of the array at offset 4 yields the second element.
        let (comp, hole) = sd.get_component(&factory, &arr, 4).unwrap();
        assert!(!hole);
        assert_eq!(comp.unwrap().get_size(), 4);
    }

    #[test]
    fn split_datatype_struct_disabled_when_config_zero() {
        // With split_datatype_config == 0, splitStructures/splitArrays are false,
        // so a struct categorizes as not-splittable (-1).
        let sd = SplitDatatype::new(0);
        // Build a 2-field struct (size 8) via the factory not required: a bare
        // TYPE_STRUCT with >1 depend is needed; use a partial-struct's parent path
        // instead exercising the metatype switch's struct arm.
        let struct_ty = Rc::new(Datatype::new(8, type_metatype::TYPE_STRUCT));
        assert_eq!(sd.categorize_datatype(&struct_ty), -1);
    }

    #[test]
    fn split_load_store_get_value_datatype_is_seam_none() {
        // getValueDatatype needs getTypeReadFacing off the arch seam -> None, so
        // RuleSplitLoad / RuleSplitStore return 0 (no change).
        let mut fd = build_fd();
        let spaceid = mk_const(&mut fd, 8, 0);
        let ptr = mk_input(&mut fd, 0x40, 8);
        let load = mk_op(&mut fd, 0x100, 2, OpCode::CPUI_LOAD);
        let outvn = mk_reg(&mut fd, 0x80, 8);
        wire_in(&mut fd, load, spaceid, 0);
        wire_in(&mut fd, load, ptr, 1);
        let _outvn = wire_out(&mut fd, load, outvn);
        assert!(SplitDatatype::get_value_datatype(&fd, load, 8).is_none());
        let mut rule = RuleSplitLoad::new("g");
        assert_eq!(rule.apply_op(load, &mut fd), 0);
    }

    #[test]
    fn rule_split_copy_is_seam_gated() {
        // RuleSplitCopy's getTypeReadFacing/getTypeDefFacing are arch-seamed, so
        // the rule returns 0 cleanly.
        let mut fd = build_fd();
        let src = mk_input(&mut fd, 0x40, 8);
        let copyop = mk_op(&mut fd, 0x100, 1, OpCode::CPUI_COPY);
        let outvn = mk_reg(&mut fd, 0x80, 8);
        wire_in(&mut fd, copyop, src, 0);
        let _outvn = wire_out(&mut fd, copyop, outvn);
        let mut rule = RuleSplitCopy::new("g");
        assert_eq!(rule.apply_op(copyop, &mut fd), 0);
    }

    // ---- SubfloatFlow / RuleSubfloatConvert (subflow.cc:3085-3522) --------

    #[test]
    fn subfloat_flow_no_format_does_not_trace() {
        // The Funcdata arch seam exposes no Translate/FloatFormat, so SubfloatFlow
        // reports no format and doTrace returns false (the C++ null-format path).
        let mut fd = build_fd();
        let root = mk_input(&mut fd, 0x40, 8);
        let mut sf = SubfloatFlow::new(&mut fd, root, 4);
        assert!(!sf.has_format);
        assert!(!sf.do_trace(&mut fd).unwrap());
    }

    #[test]
    fn rule_subfloat_convert_returns_zero_under_seam() {
        // FLOAT_FLOAT2FLOAT widening (out>in): SubfloatFlow can't acquire a format
        // (arch seam) so doTrace is false and the rule returns 0.
        let mut fd = build_fd();
        let invn = mk_input(&mut fd, 0x40, 4);
        let f2f = mk_op(&mut fd, 0x100, 1, OpCode::CPUI_FLOAT_FLOAT2FLOAT);
        let outvn = mk_reg(&mut fd, 0x80, 8);
        wire_in(&mut fd, f2f, invn, 0);
        let _outvn = wire_out(&mut fd, f2f, outvn);
        let mut rule = RuleSubfloatConvert::new("g");
        assert_eq!(rule.apply_op(f2f, &mut fd), 0);
    }
}
