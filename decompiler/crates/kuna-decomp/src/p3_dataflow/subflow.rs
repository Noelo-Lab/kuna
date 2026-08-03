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
//!     then `TransformManager::apply` (W6-stubbed in the merged transform.rs —
//!     `createReplacement` needs `glb->inst[opc]`).
//!   * `SubfloatFlow` / `RuleSubfloatConvert` (item `w8x-subflow-splits`).  The
//!     precision-tracing engine is ported verbatim, including the `FloatFormat`
//!     acquisition (`f->getArch()->translate->getFloatFormat`) and the
//!     constant-conversion (`FloatFormat::convertEncoding`); only the
//!     `preserveAddress` override remains a transform stub.
//!   * `SplitDatatype` / `RuleSplit{Copy,Load,Store}` / `RuleDumptyHumpLate`
//!     (item `w8x-subflow-splits`).  The datatype-splitting engine
//!     (`RootPointer`/`Component`, `categorizeDatatype`/`testDatatypeCompatibility`/
//!     `getComponent`/`splitCopy`/`splitLoad`/`splitStore`) is ported faithfully.
//!     The graph-mutation and type-facing calls that need the (still-stub)
//!     `Architecture`→`TypeFactory`/`Translate` bridge, `getTypeReadFacing`/
//!     `getTypeDefFacing`, `ResolveCache::addResolution`/`inheritResolution`, and
//!     `Funcdata::buildCopyTemp`/`getMerge` route through arch/W6 stub helpers (see
//!     the losses output); the size/offset categorization logic is fully
//!     executable.
//!   * `LaneDivide`: **STUB(W5-transform-lanedivide)** — not named by this item;
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
//! # Cross-wave stubs (do not invent behavior — recorded as losses)
//!
//!   * `Funcdata::opSetOutput` (the W3-funcdata `(vbank,obank)` split-borrow) and
//!     opcode→`TypeOp` resolution (`glb->inst[opc]`, W6) are both still stub-gated
//!     in the foundation.  Every graph *mutation* in [`SubvariableFlow::do_replacement`]
//!     and in the rules routes through them; the mutation **structure/order** is
//!     transcribed, but the calls return the foundation's stub error until those
//!     land.  // STUB(W3-funcdata)/STUB(W6)
//!   * `FuncCallSpecs`/`FuncProto` call-site state (W4), `JumpTable` (W5/W7), and
//!     `TypeOpFloatInt2Float::preferredZextSize` (W6): the `try*Pull`/`try*Push`
//!     helpers that need them transcribe their guard structure and return a
//!     stub-gated `false`/error.  The integer/shift/compare/copy/extension trace
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
use kuna_base::space::AddrSpace;
use kuna_base::types::{int4, int8, uint4, uintb};
use kuna_num::opcodes::OpCode;

use crate::action::{ActionGroupList, Rule, RuleSpec};
use crate::dtype::{type_metatype, Datatype, TypeFactory};
use crate::funcdata::Funcdata;
use crate::op::pcodeop_flags;
use crate::context::{ArchHandle, OpId, TypeOp, VarnodeId};
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
/// `val`/`replacement` are written by discovery and read by the stub-gated
/// [`SubvariableFlow::do_replacement`] (W3-funcdata/W6); kept here so the
/// placeholder layout is the faithful C++ one.  // STUB(W3-funcdata)/STUB(W6)
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
/// `Int2FloatPatch` is produced only by the W6-stubbed `try_int2float_pull`; the
/// full enum mirrors the C++ patchtype so [`SubvariableFlow::do_replacement`]
/// dispatches on the exact same set.  // STUB(W6)
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
/// the stub-gated [`SubvariableFlow::do_replacement`].  // STUB(W3-funcdata)
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
/// `op`/`replacement`/`opc`/`numparams` are read by the stub-gated
/// [`SubvariableFlow::do_replacement`] when it materialises the new ops.
/// // STUB(W3-funcdata)
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
/// stub-gated Funcdata mutation API).
pub struct SubvariableFlow {
    /// Size of the logical data-flow in bytes.
    flowsize: int4,
    /// Number of bits in logical variable.
    bitsize: int4,
    /// Have we tried to flow logical value across CPUI_RETURNs.  Set inside
    /// `try_return_pull` (C++ `returnsTraversed`).
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
        // Already seen before
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
        // The defining op of the *original*
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
    // try* terminal helpers (subflow.cc:208-367) — W4/W6/jumptable stubs
    // -------------------------------------------------------------------------

    /// Determine if the given subgraph variable can act as a parameter to the
    /// given CALL op (C++ `SubvariableFlow::tryCallPull`).
    ///
    /// Records a parameter patch (truncate the argument to its logical size) when
    /// the call's prototype permits it: not input-active (mid-recovery), and not a
    /// non-varargs locked prototype.
    fn try_call_pull(
        &mut self,
        data: &Funcdata,
        op: OpId,
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
        let idx = match data.get_call_specs_index(op) {
            Some(i) => i,
            None => return Ok(false), // fc == 0
        };
        let fc = data.get_call_specs(idx);
        // Don't trim while in the middle of figuring out params.
        if fc.is_input_active() {
            return Ok(false);
        }
        // Don't trim a non-varargs locked prototype.
        if fc.is_input_locked() && !fc.is_dotdotdot() {
            return Ok(false);
        }
        self.patchlist.push(PatchRecord {
            typ: PatchType::ParameterPatch,
            patch_op: op,
            in1: rvn,
            in2: None,
            slot,
        });
        self.pullcount += 1; // A true terminal modification
        Ok(true)
    }

    /// Determine if the given subgraph variable can act as return value for the
    /// given RETURN op (C++ `SubvariableFlow::tryReturnPull`, subflow.cc:238).
    ///
    /// The W4 prototype state (`FuncProto::isOutputLocked`) is now carried on the
    /// merged-tree `Funcdata`, so this is the faithful transcription: bail on the
    /// return-address container slot or a locked output, bail (non-aggressive) if
    /// anything outside the logical mask is consumed, then — once — propagate the
    /// logical size to every other RETURN so the function keeps a single return
    /// type, and record the terminal parameter patch.
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
        if data.get_func_proto().is_output_locked() {
            return Ok(false);
        }
        if !self.aggressive {
            // If there's something outside the mask being consumed, don't truncate.
            let vn = self.rv(rvn).vn.expect("try_return_pull: rvn vn");
            let rmask = self.rv(rvn).mask;
            if (data.vbank().get(vn).expect("try_return_pull: stale rvn vn").get_consume() & !rmask)
                != 0
            {
                return Ok(false);
            }
        }

        if !self.returns_traversed {
            // If we plan to truncate the size of a return variable, we need to
            // propagate the logical size to any other return variables so that
            // there can still be a single return value type for the function.
            let rmask = self.rv(rvn).mask;
            let return_ops: Vec<OpId> = data.obank().iter_code(OpCode::CPUI_RETURN).collect();
            for retop in return_ops {
                let (halt, retvn) = {
                    let o = match data.obank().get(retop) {
                        Some(o) => o,
                        None => continue,
                    };
                    (o.get_halt_type(), o.get_in(slot))
                };
                if halt != 0 {
                    continue; // Artificial halt
                }
                let retvn = match retvn {
                    Some(v) => v,
                    None => continue,
                };
                let (rep, inworklist) = self.set_replacement(data, retvn, rmask);
                let rep = match rep {
                    Some(r) => r,
                    None => return Ok(false),
                };
                if inworklist {
                    self.worklist.push(rep);
                } else if data.vbank().get(retvn).expect("try_return_pull: stale retvn").is_constant()
                    && retop != op
                {
                    // Trace won't revisit this RETURN, so generate the patch now.
                    self.patchlist.push(PatchRecord {
                        typ: PatchType::ParameterPatch,
                        patch_op: retop,
                        in1: rep,
                        in2: None,
                        slot,
                    });
                    self.pullcount += 1;
                }
            }
            self.returns_traversed = true;
        }
        self.patchlist.push(PatchRecord {
            typ: PatchType::ParameterPatch,
            patch_op: op,
            in1: rvn,
            in2: None,
            slot,
        });
        self.pullcount += 1; // A true terminal modification
        Ok(true)
    }

    /// Determine if the given subgraph variable can act as a \e created value for
    /// the given INDIRECT/CALL op (C++ `SubvariableFlow::tryCallReturnPush`).
    ///
    /// STUB(W4): `fd->getCallSpecs(op)` / `isOutputLocked` / `isOutputActive`.
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
             (isOutputLocked / isOutputActive) — STUB(W4)",
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
    /// STUB(W6): `TypeOpFloatInt2Float::preferredZextSize(flowsize)`.  The NZ/size
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
             TypeOpFloatInt2Float::preferredZextSize — STUB(W6)",
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
    // replacement geometry (subflow.cc:1258-1345) — W3-funcdata mutation stubs
    // -------------------------------------------------------------------------

    /// Decide if we use the same memory range of the original Varnode for the
    /// logical replacement (C++ `SubvariableFlow::useSameAddress`).
    ///
    /// Used by `getReplaceVarnode` inside [`SubvariableFlow::do_replacement`].
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
    /// Used by `getReplaceVarnode` inside [`SubvariableFlow::do_replacement`].
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

    /// Replace the input Varnode for the given subgraph variable, to avoid the
    /// overlap errors that arise when the original input is reused as the smaller
    /// logical input (C++ `SubvariableFlow::replaceInput`, subflow.cc:1258).
    fn replace_input(&mut self, data: &mut Funcdata, rvn: RvId) -> KunaResult<()> {
        let vn = self.rv(rvn).vn.expect("replace_input: rvn vn");
        let size = data.vbank().get(vn).expect("replace_input: stale vn").get_size();
        let newvn = data.new_unique(size, None);
        let newvn = data.set_input_varnode(newvn)?;
        data.total_replace(vn, newvn)?;
        data.delete_varnode(vn)?;
        self.rv_mut(rvn).vn = Some(newvn);
        Ok(())
    }

    /// Build (or fetch) the logical Varnode that replaces the original containing
    /// Varnode for the given subgraph variable node (C++
    /// `SubvariableFlow::getReplaceVarnode`, subflow.cc:1316).  This is the main
    /// routine for turning a logical variable into an actual Varnode object.
    fn get_replace_varnode(&mut self, data: &mut Funcdata, rvn: RvId) -> KunaResult<VarnodeId> {
        if let Some(repl) = self.rv(rvn).replacement {
            return Ok(repl);
        }
        if self.rv(rvn).vn.is_none() {
            if self.rv(rvn).def.is_none() {
                // A constant that did not come from an original Varnode
                let val = self.rv(rvn).val;
                return Ok(data.new_constant(self.flowsize, val));
            }
            let repl = data.new_unique(self.flowsize, None);
            self.rv_mut(rvn).replacement = Some(repl);
            return Ok(repl);
        }
        let vn = self.rv(rvn).vn.expect("get_replace_varnode: rvn vn");
        if data.vbank().get(vn).expect("get_replace_varnode: stale vn").is_constant() {
            let val = self.rv(rvn).val;
            let new_vn = data.new_constant(self.flowsize, val);
            // STUB(W4): EquateSymbol propagation — getSymbolEntry is null in the W4
            // symbol-scope skeleton (no equate symbols are constructed on this path),
            // so copySymbolIfValid is a no-op here; faithful (recorded as a loss).
            return Ok(new_vn);
        }

        let isinput = data.vbank().get(vn).expect("get_replace_varnode: stale vn").is_input();
        if self.use_same_address(data, rvn) {
            let addr = self.get_replacement_address(data, rvn)?;
            if isinput {
                self.replace_input(data, rvn)?; // Replace input to avoid overlap errors
            }
            let repl = data.new_varnode(self.flowsize, &addr, None);
            self.rv_mut(rvn).replacement = Some(repl);
        } else {
            let repl = data.new_unique(self.flowsize, None);
            self.rv_mut(rvn).replacement = Some(repl);
        }
        if isinput {
            let repl = self.rv(rvn).replacement.expect("get_replace_varnode: replacement set above");
            let new_repl = data.set_input_varnode(repl)?;
            self.rv_mut(rvn).replacement = Some(new_repl);
        }
        Ok(self.rv(rvn).replacement.expect("get_replace_varnode: replacement set"))
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
        // C++ builds the root link in the constructor; if a stub aborts it, the
        // C++ destructor still clears the marks `setReplacement` set.  Mirror that:
        // on an error, clear every mark recorded in `varmap` before propagating, so
        // a subsequent SubvariableFlow run does not see a stale mark (which would
        // trip the `marked vn must be in varmap` invariant).
        if let Err(e) = sf.create_link_root(data, mask, root) {
            sf.clear_marks(data);
            return Err(e);
        }
        Ok(sf)
    }

    /// Clear the `mark` bit on every Varnode this subflow recorded in `varmap`
    /// (the C++ `~SubvariableFlow` mark cleanup; also used on the early-abort
    /// path so a failed construction does not leak marks).
    fn clear_marks(&self, data: &mut Funcdata) {
        let marked: Vec<VarnodeId> = self.varmap.keys().copied().collect();
        for vn in marked {
            if let Some(v) = data.vbank_mut().get_mut(vn) {
                v.clear_mark();
            }
        }
    }

    /// Trace logical value through data-flow, constructing transform
    /// (C++ `SubvariableFlow::doTrace`).
    pub fn do_trace(&mut self, data: &mut Funcdata) -> KunaResult<bool> {
        self.pullcount = 0;
        let mut retval = false;
        let mut traced: KunaResult<()> = Ok(());
        if self.valid {
            retval = true;
            while !self.worklist.is_empty() {
                match self.process_next_work(data) {
                    Ok(true) => {}
                    Ok(false) => {
                        retval = false;
                        break;
                    }
                    Err(e) => {
                        // A stub aborted the trace; clear marks (below) before
                        // surfacing it, so the next run does not see stale marks.
                        retval = false;
                        traced = Err(e);
                        break;
                    }
                }
            }
        }

        // Clear marks (the C++ destructor's mark cleanup) — runs on every exit
        // path, including the stub-abort above.
        self.clear_marks(data);

        traced?;
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
    /// STUB(W3-funcdata)/STUB(W6): every `fd->opSet*`/`newOp`/`newVarnode*` here
    /// routes through `Funcdata::opSetOutput` (the unported `(vbank,obank)`
    /// split-borrow) and opcode→`TypeOp` resolution (`glb->inst[opc]`).  The
    /// patch *order* (call-return push patches first, then define outputs, then
    /// inputs, then the terminal patches in list order) is transcribed; the
    /// concrete mutations return the foundation's stub error until those land.
    pub fn do_replacement(&mut self, data: &mut Funcdata) -> KunaResult<()> {
        // Do up front processing of the call return patches, which will be at the
        // front of the list.  Walk while type == push_patch; the index where we
        // stop is the resume point for the pull-patch loop below.
        let mut piter = 0usize;
        while piter < self.patchlist.len() {
            if self.patchlist[piter].typ != PatchType::PushPatch {
                break;
            }
            let push_op = self.patchlist[piter].patch_op;
            let in1 = self.patchlist[piter].in1;
            let new_vn = self.get_replace_varnode(data, in1)?;
            let old_vn =
                data.obank().get(push_op).expect("do_replacement: stale push op").get_out().expect(
                    "do_replacement: push op has no output",
                );
            data.op_set_output(push_op, new_vn)?;

            // Create placeholder defining op for old Varnode, until dead code cleans it up
            let push_addr =
                data.obank().get(push_op).expect("do_replacement: stale push op").get_addr().clone();
            let new_zext = data.new_op(1, push_addr);
            data.op_set_opcode_code(new_zext, OpCode::CPUI_INT_ZEXT);
            data.op_set_input(new_zext, new_vn, 0)?;
            data.op_set_output(new_zext, old_vn)?;
            data.op_insert_after(new_zext, push_op);
            piter += 1;
        }

        // Define all the outputs first
        for idx in 0..self.oplist.len() {
            let op = self.oplist[idx].op.expect("do_replacement: ReplaceOp has no op");
            let numparams = self.oplist[idx].numparams;
            let opc = self.oplist[idx].opc;
            let addr = data.obank().get(op).expect("do_replacement: stale op").get_addr().clone();
            let newop = data.new_op(numparams, addr);
            self.oplist[idx].replacement = Some(newop);
            data.op_set_opcode_code(newop, opc);
            let rout = self.oplist[idx].output.expect("do_replacement: ReplaceOp has no output");
            let outvn = self.get_replace_varnode(data, rout)?;
            data.op_set_output(newop, outvn)?;
            data.op_insert_after(newop, op);
        }

        // Set all the inputs
        for idx in 0..self.oplist.len() {
            let newop = self.oplist[idx].replacement.expect("do_replacement: op not materialized");
            let inlen = self.oplist[idx].input.len();
            for i in 0..inlen {
                let rin = self.oplist[idx].input[i].expect("do_replacement: input slot is null");
                let invn = self.get_replace_varnode(data, rin)?;
                data.op_set_input(newop, invn, i as int4)?;
            }
        }

        // These are operations that carry flow from the small variable into an
        // existing variable of the correct size.  Resume from `piter` (past the
        // push patches handled above).
        while piter < self.patchlist.len() {
            let pullop = self.patchlist[piter].patch_op;
            let typ = self.patchlist[piter].typ;
            match typ {
                PatchType::CopyPatch => {
                    loop {
                        let n = data.obank().get(pullop).expect("do_replacement: stale pullop").num_input();
                        if n <= 1 {
                            break;
                        }
                        data.op_remove_input(pullop, n - 1);
                    }
                    let in1 = self.patchlist[piter].in1;
                    let v = self.get_replace_varnode(data, in1)?;
                    data.op_set_input(pullop, v, 0)?;
                    data.op_set_opcode_code(pullop, OpCode::CPUI_COPY);
                }
                PatchType::ComparePatch => {
                    let in1 = self.patchlist[piter].in1;
                    let in2 = self.patchlist[piter].in2.expect("compare_patch: in2 is null");
                    let v1 = self.get_replace_varnode(data, in1)?;
                    let v2 = self.get_replace_varnode(data, in2)?;
                    data.op_set_input(pullop, v1, 0)?;
                    data.op_set_input(pullop, v2, 1)?;
                }
                PatchType::ParameterPatch => {
                    let in1 = self.patchlist[piter].in1;
                    let slot = self.patchlist[piter].slot;
                    let v = self.get_replace_varnode(data, in1)?;
                    data.op_set_input(pullop, v, slot)?;
                }
                PatchType::ExtensionPatch => {
                    // operations that flow the small variable into a bigger variable
                    // where all the remaining bits are zero
                    let sa = self.patchlist[piter].slot;
                    let in1 = self.patchlist[piter].in1;
                    let in_vn = self.get_replace_varnode(data, in1)?;
                    let out_size = data
                        .vbank()
                        .get(
                            data.obank()
                                .get(pullop)
                                .expect("do_replacement: stale pullop")
                                .get_out()
                                .expect("extension_patch: pullop has no output"),
                        )
                        .expect("do_replacement: stale out vn")
                        .get_size();
                    if sa == 0 {
                        let invec: Vec<VarnodeId> = vec![in_vn];
                        let in_size =
                            data.vbank().get(in_vn).expect("do_replacement: stale in vn").get_size();
                        let opc = if in_size == out_size {
                            OpCode::CPUI_COPY
                        } else {
                            OpCode::CPUI_INT_ZEXT
                        };
                        data.op_set_opcode_code(pullop, opc);
                        data.op_set_all_input(pullop, &invec)?;
                    } else {
                        let mut invec: Vec<VarnodeId> = Vec::new();
                        let in_size =
                            data.vbank().get(in_vn).expect("do_replacement: stale in vn").get_size();
                        if in_size != out_size {
                            let pull_addr = data
                                .obank()
                                .get(pullop)
                                .expect("do_replacement: stale pullop")
                                .get_addr()
                                .clone();
                            let zextop = data.new_op(1, pull_addr);
                            data.op_set_opcode_code(zextop, OpCode::CPUI_INT_ZEXT);
                            let zextout = data.new_unique_out(out_size, zextop)?;
                            data.op_set_input(zextop, in_vn, 0)?;
                            data.op_insert_before(zextop, pullop);
                            invec.push(zextout);
                        } else {
                            invec.push(in_vn);
                        }
                        let c = data.new_constant(4, sa as uintb);
                        invec.push(c);
                        data.op_set_all_input(pullop, &invec)?;
                        data.op_set_opcode_code(pullop, OpCode::CPUI_INT_LEFT);
                    }
                }
                PatchType::PushPatch => {
                    // Shouldn't see these here, handled earlier
                }
                PatchType::Int2FloatPatch => {
                    let pull_addr = data
                        .obank()
                        .get(pullop)
                        .expect("do_replacement: stale pullop")
                        .get_addr()
                        .clone();
                    let zext_op = data.new_op(1, pull_addr);
                    data.op_set_opcode_code(zext_op, OpCode::CPUI_INT_ZEXT);
                    let in1 = self.patchlist[piter].in1;
                    let invn = self.get_replace_varnode(data, in1)?;
                    data.op_set_input(zext_op, invn, 0)?;
                    let invn_size =
                        data.vbank().get(invn).expect("do_replacement: stale invn").get_size();
                    let sizeout = preferred_zext_size(invn_size);
                    let outvn = data.new_unique_out(sizeout, zext_op)?;
                    data.op_insert_before(zext_op, pullop);
                    data.op_set_input(pullop, outvn, 0)?;
                }
            }
            piter += 1;
        }
        Ok(())
    }

    /// `vn->isZeroExtended(flowsize)` (C++ `Varnode::isZeroExtended`).
    ///
    /// STUB(W3-varnode): `Varnode::isZeroExtended` is not yet on the W3 varnode
    /// surface.  The INT_DIV/INT_REM trace paths that call it are reachable only
    /// for whole-byte divides; until the accessor lands this stub-gates them.
    fn is_zero_extended(_data: &Funcdata, _vn: VarnodeId, _flowsize: int4) -> KunaResult<bool> {
        Err(KunaError::lowlevel(
            "kuna rust port: Varnode::isZeroExtended not on the W3 varnode surface — STUB(W3-varnode)",
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
        // STUB(W4): Architecture::aggressive_ext_trim is not on the W3 ArchContext
        // (Funcdata::getArch returns the context::ArchContext skeleton).  The C++
        // default is `false`; we keep that until the W4 arch surface lands.
        self.isaggressive = 0;
    }
}

/// Preferred zero-extension size for a FLOAT_INT2FLOAT input of the given size
/// (C++ `TypeOpFloatInt2Float::preferredZextSize`).
///
/// Used by the `Int2FloatPatch` arm of [`SubvariableFlow::do_replacement`].
/// STUB(W6): the `TypeOpFloatInt2Float` precision table is not yet present; the
/// conservative default returns the input size.  That arm is only produced by the
/// W6-stubbed `try_int2float_pull` (which aborts the trace before reaching here),
/// so the default never affects a completing path (recorded as a loss).
fn preferred_zext_size(in_size: int4) -> int4 {
    in_size
}

/// Run the SubvariableFlow trace+replacement for a trigger rule (C++ idiom:
/// `SubvariableFlow subflow(...); if (!subflow.doTrace()) return 0;
/// subflow.doReplacement(); return 1;`).
///
/// Returns `1` if the transform was constructed AND applied, `0` otherwise.
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
        Err(_) => return 0, // construction reached a stub (e.g. sext constant check)
    };
    match subflow.do_trace(data) {
        Ok(true) => {}
        _ => return 0,
    }
    match subflow.do_replacement(data) {
        Ok(()) => 1,
        // C++ doReplacement() returns void; a structured error (e.g. a residual
        // symbol/iop stub) is treated as "no change" rather than aborting the pass.
        Err(_) => 0,
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

/// Advance a lane handle by `i` lanes (the C++ `rvn + i` pointer arithmetic over
/// a split `TransformVar` array, used pervasively by `LaneDivide` for the N-lane
/// case where `SplitFlow` only ever needs the 2-lane `rvn`/`rvn+1`).
fn lane_at(rvn: TVarRef, i: int4) -> TVarRef {
    match rvn {
        TVarRef::Piece { key, idx } => TVarRef::Piece { key, idx: idx + i as usize },
        // The C++ only ever does `rvn + i` on a piece-array element (a split
        // lane); a `New(_)` here would be a porting bug.
        TVarRef::New(_) => panic!("lane_at on a non-piece TransformVar"),
    }
}

/// Resolve the address space a constant space-id Varnode encodes (C++
/// `Varnode::getSpaceFromConst`).  The constant's offset is the space index into
/// the function's space manager; used by `LaneDivide::buildStore`/`buildLoad` to
/// recover the STORE/LOAD target space.
fn space_from_const(data: &Funcdata, vn: VarnodeId) -> Rc<kuna_base::space::AddrSpace> {
    let idx = data.vbank().get(vn).expect("space_from_const: stale vn").get_offset() as int4;
    Rc::clone(
        data.get_arch()
            .manage()
            .get_space(idx)
            .expect("space_from_const: getSpaceFromConst out of range"),
    )
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
    /// C++ `SplitFlow::SplitFlow` (subflow.cc:2011).
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
    /// base class).  Materializes the placeholder graph into real IR (W10).
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
            // C++ apply() returns void; a structured error here (e.g. an iop/symbol
            // stub) is treated as "no change made" rather than aborting the pass.
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
// the stub-gated split build-helpers; the rules detect the stub and return 0 (no change).

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
/// Read by the stub-gated `split{Copy,Load,Store}` build-helpers (which materialize
/// per-component ops); until the Funcdata arch bridge lands those mutation paths
/// return the stub error, so the fields are exercised only by the categorization
/// tests.  // STUB(arch/W6)
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
// `split{Copy,Load,Store}` build path.  The C++ holds `Funcdata &data`,
// `TypeFactory *types`, and a `ResolveCache resolver`; in Rust `data` is threaded
// as a `&mut Funcdata` argument, `types` is a cloned `Rc<TypeFactoryImpl>` handle
// (held across the `&mut Funcdata` borrow the build helpers need), and `resolver`
// is owned here.  Union resolution is the only piece still stubbed (W6/W7): every
// `ResolveCache` method short-circuits on `!dt->needsResolution()` (unionresolve.cc
// :1230/1269), so for the non-union corpus the resolver is a faithful no-op.
pub struct SplitDatatype {
    /// Sequence of all data-type pairs being copied (C++ `dataTypePieces`).
    data_type_pieces: Vec<Component>,
    /// The type factory (C++ `types`, a borrow of `glb->types`).  `None` only for
    /// the bare-architecture unit-test fixtures; the entry rules always have one.
    types: Option<Rc<crate::dtype::TypeFactoryImpl>>,
    /// Union-resolution cache (C++ `resolver`).
    resolver: crate::unionresolve::ResolveCache,
    /// Whether or not structures should be split (C++ `splitStructures`).
    split_structures: bool,
    /// Whether or not arrays should be split (C++ `splitArrays`).
    split_arrays: bool,
    /// True if trying to split LOAD or STORE (C++ `isLoadStore`).
    is_load_store: bool,
}

impl SplitDatatype {
    /// Construct from the split-datatype config bits, no factory handle (used by
    /// the categorization unit tests that pass the factory explicitly).
    pub fn new(split_datatype_config: uint4) -> SplitDatatype {
        SplitDatatype {
            data_type_pieces: Vec::new(),
            types: None,
            resolver: crate::unionresolve::ResolveCache::new(),
            split_structures: (split_datatype_config & option_split_datatypes::OPTION_STRUCT) != 0,
            split_arrays: (split_datatype_config & option_split_datatypes::OPTION_ARRAY) != 0,
            is_load_store: false,
        }
    }

    /// Construct from the containing function (C++
    /// `SplitDatatype::SplitDatatype(Funcdata&)`, subflow.cc:2712):
    /// `types = glb->types; splitStructures/splitArrays = glb->split_datatype_config`.
    pub fn from_funcdata(data: &Funcdata) -> SplitDatatype {
        let config = data.get_arch().split_datatype_config;
        SplitDatatype {
            data_type_pieces: Vec::new(),
            types: data.get_arch().types_rc(),
            resolver: crate::unionresolve::ResolveCache::new(),
            split_structures: (config & option_split_datatypes::OPTION_STRUCT) != 0,
            split_arrays: (config & option_split_datatypes::OPTION_ARRAY) != 0,
            is_load_store: false,
        }
    }

    /// The held type factory (C++ `types`).  Errors if absent (only the bare
    /// unit-test architecture has none; the entry rules always carry one).
    fn types(&self) -> KunaResult<&crate::dtype::TypeFactoryImpl> {
        self.types
            .as_deref()
            .ok_or_else(|| KunaError::lowlevel("SplitDatatype: no TypeFactory on Architecture"))
    }

    /// Obtain the component of the given data-type at the specified offset (C++
    /// `SplitDatatype::getComponent`, subflow.cc:2219).  Returns
    /// `(component, is_hole)`.
    #[allow(dead_code)] // consumed by the stub-gated splitCopy/Load/Store build path
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
    #[allow(dead_code)] // consumed by the stub-gated splitCopy/Load/Store build path
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
    #[allow(dead_code)] // consumed by the stub-gated splitCopy/Load/Store build path
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

    /// Test specific constraints for splitting the given COPY into pieces (C++
    /// `SplitDatatype::testCopyConstraints`, subflow.cc:2390).
    fn test_copy_constraints(&self, data: &Funcdata, copy_op: OpId) -> bool {
        // (kuna) A COPY whose output lands in a read-only address range is never a
        // store the program performs; splitting it turns one invisible whole-range
        // assignment into per-element assignments that survive
        // `Merge::mark_internal_copies` and print as stores into `.rodata`.
        let out_vn = data.obank().get(copy_op).expect("stale copy").get_out().expect("copy out");
        if data.vbank().get(out_vn).expect("stale out").is_read_only() {
            return false;
        }
        let in_vn = data.obank().get(copy_op).expect("stale copy").get_in(0).expect("copy in0");
        if data.vbank().get(in_vn).expect("stale in").is_input() {
            return false;
        }
        if data.vbank().get(in_vn).expect("stale in").is_addr_tied() {
            let out_vn = data.obank().get(copy_op).expect("stale copy").get_out().expect("copy out");
            if data.vbank().get(out_vn).expect("stale out").is_addr_tied()
                && data.vbank().get(out_vn).expect("stale out").get_addr()
                    == data.vbank().get(in_vn).expect("stale in").get_addr()
            {
                return false;
            }
        } else if data.vbank().get(in_vn).expect("stale in").is_written() {
            let def = data.vbank().get(in_vn).expect("stale in").get_def().expect("in def");
            if data.obank().get(def).expect("stale def").code() == OpCode::CPUI_LOAD
                && data.lone_descend(in_vn) == Some(copy_op)
            {
                return false; // This situation is handled by splitCopy()
            }
        }
        true
    }

    /// If the given Varnode is an extended-precision constant, create split
    /// constants (C++ `SplitDatatype::generateConstants`, subflow.cc:2413).
    fn generate_constants(
        &self,
        data: &mut Funcdata,
        vn: VarnodeId,
        in_varnodes: &mut Vec<VarnodeId>,
    ) -> KunaResult<bool> {
        if data.lone_descend(vn).is_none() {
            return Ok(false);
        }
        if !data.vbank().get(vn).expect("stale vn").is_written() {
            return Ok(false);
        }
        let op = data.vbank().get(vn).expect("stale vn").get_def().expect("vn def");
        let opc = data.obank().get(op).expect("stale def").code();
        if opc == OpCode::CPUI_INT_ZEXT {
            let i0 = data.obank().get(op).expect("stale def").get_in(0).expect("zext in0");
            if !data.vbank().get(i0).expect("stale in0").is_constant() {
                return Ok(false);
            }
        } else if opc == OpCode::CPUI_PIECE {
            let i0 = data.obank().get(op).expect("stale def").get_in(0).expect("piece in0");
            let i1 = data.obank().get(op).expect("stale def").get_in(1).expect("piece in1");
            if !data.vbank().get(i0).expect("stale in0").is_constant()
                || !data.vbank().get(i1).expect("stale in1").is_constant()
            {
                return Ok(false);
            }
        } else {
            return Ok(false);
        }
        let full_size = data.vbank().get(vn).expect("stale vn").get_size();
        let is_big_endian = data.vbank().get(vn).expect("stale vn").get_space().is_big_endian();
        let hi: uintb;
        let lo: uintb;
        let losize: int4;
        if opc == OpCode::CPUI_INT_ZEXT {
            let i0 = data.obank().get(op).expect("stale def").get_in(0).expect("zext in0");
            hi = 0;
            lo = data.vbank().get(i0).expect("stale in0").get_offset();
            losize = data.vbank().get(i0).expect("stale in0").get_size();
        } else {
            let i0 = data.obank().get(op).expect("stale def").get_in(0).expect("piece in0");
            let i1 = data.obank().get(op).expect("stale def").get_in(1).expect("piece in1");
            hi = data.vbank().get(i0).expect("stale in0").get_offset();
            lo = data.vbank().get(i1).expect("stale in1").get_offset();
            losize = data.vbank().get(i1).expect("stale in1").get_size();
        }
        for i in 0..self.data_type_pieces.len() {
            let dt = Rc::clone(&self.data_type_pieces[i].in_type);
            if dt.get_size() as usize > std::mem::size_of::<uintb>() {
                in_varnodes.clear();
                return Ok(false);
            }
            // cast: piece offset + size are bounded by full_size (a Varnode size).
            let sa: int4 = if is_big_endian {
                full_size - (self.data_type_pieces[i].offset + dt.get_size())
            } else {
                self.data_type_pieces[i].offset
            };
            let mut val: uintb = if sa >= losize {
                // cast: (sa-losize) is a non-negative bit-group shift < 64 here.
                hi >> ((sa - losize) as u32)
            } else {
                let mut v = lo >> ((sa as u32).wrapping_mul(8));
                if sa + dt.get_size() > losize {
                    v |= hi.wrapping_shl(((losize - sa) as u32).wrapping_mul(8));
                }
                v
            };
            val &= calc_mask(dt.get_size());
            let out_vn = data.new_constant(dt.get_size(), val);
            in_varnodes.push(out_vn);
            data.vbank_mut().get_mut(out_vn).expect("stale const").update_type(dt);
        }
        data.op_destroy(op);
        Ok(true)
    }

    /// Assuming the input is a constant, build split constants (C++
    /// `SplitDatatype::buildInConstants`, subflow.cc:2478).
    fn build_in_constants(
        &self,
        data: &mut Funcdata,
        root_vn: VarnodeId,
        in_varnodes: &mut Vec<VarnodeId>,
        big_endian: bool,
    ) {
        let base_val = data.vbank().get(root_vn).expect("stale root").get_offset();
        let root_size = data.vbank().get(root_vn).expect("stale root").get_size();
        for i in 0..self.data_type_pieces.len() {
            let dt = Rc::clone(&self.data_type_pieces[i].in_type);
            let off = if big_endian {
                root_size - self.data_type_pieces[i].offset - dt.get_size()
            } else {
                self.data_type_pieces[i].offset
            };
            // cast: off is a byte offset within a Varnode (< 8 here).
            let val = (base_val >> ((8 * off) as u32)) & calc_mask(dt.get_size());
            let out_vn = data.new_constant(dt.get_size(), val);
            in_varnodes.push(out_vn);
            data.vbank_mut().get_mut(out_vn).expect("stale const").update_type(dt);
        }
    }

    /// Build input Varnodes by extracting SUBPIECEs from the root (C++
    /// `SplitDatatype::buildInSubpieces`, subflow.cc:2501).
    fn build_in_subpieces(
        &self,
        data: &mut Funcdata,
        root_vn: VarnodeId,
        follow_op: OpId,
        in_varnodes: &mut Vec<VarnodeId>,
    ) -> KunaResult<()> {
        if self.generate_constants(data, root_vn, in_varnodes)? {
            return Ok(());
        }
        let base_addr = data.vbank().get(root_vn).expect("stale root").get_addr().clone();
        let root_size = data.vbank().get(root_vn).expect("stale root").get_size();
        let follow_addr = data.obank().get(follow_op).expect("stale follow").get_addr().clone();
        for i in 0..self.data_type_pieces.len() {
            let dt = Rc::clone(&self.data_type_pieces[i].in_type);
            let mut off = self.data_type_pieces[i].offset;
            let mut addr = &base_addr + off as i64;
            addr.renormalize(dt.get_size(), data.get_arch().manage())?;
            if addr.is_big_endian() {
                off = root_size - off - dt.get_size();
            }
            let subpiece = data.new_op(2, follow_addr.clone());
            data.op_set_opcode(subpiece, crate::typeop::type_op_for(OpCode::CPUI_SUBPIECE));
            data.op_set_input(subpiece, root_vn, 0)?;
            // cast: off fits int4 (bounded by Varnode size).
            let offconst = data.new_constant(4, off as uintb);
            data.op_set_input(subpiece, offconst, 1)?;
            let out_vn = data.new_varnode_out(dt.get_size(), &addr, subpiece)?;
            in_varnodes.push(out_vn);
            data.vbank_mut().get_mut(out_vn).expect("stale sub out").update_type(dt);
            data.op_insert_before(subpiece, follow_op);
        }
        Ok(())
    }

    /// Build output Varnodes with storage based on the given root (C++
    /// `SplitDatatype::buildOutVarnodes`, subflow.cc:2531).
    fn build_out_varnodes(
        &self,
        data: &mut Funcdata,
        root_vn: VarnodeId,
        out_varnodes: &mut Vec<VarnodeId>,
    ) -> KunaResult<()> {
        let base_addr = data.vbank().get(root_vn).expect("stale root").get_addr().clone();
        for i in 0..self.data_type_pieces.len() {
            let dt = Rc::clone(&self.data_type_pieces[i].out_type);
            let off = self.data_type_pieces[i].offset;
            let mut addr = &base_addr + off as i64;
            addr.renormalize(dt.get_size(), data.get_arch().manage())?;
            let out_vn = data.new_varnode(dt.get_size(), &addr, Some(dt));
            out_varnodes.push(out_vn);
        }
        Ok(())
    }

    /// Concatenate output Varnodes into the given root via PIECE ops (C++
    /// `SplitDatatype::buildOutConcats`, subflow.cc:2552).
    fn build_out_concats(
        &self,
        data: &mut Funcdata,
        root_vn: VarnodeId,
        previous_op: OpId,
        out_varnodes: &[VarnodeId],
    ) -> KunaResult<()> {
        if data.vbank().get(root_vn).expect("stale root").has_no_descend() {
            return Ok(()); // Don't need to produce concatenation if unused
        }
        let base_addr = data.vbank().get(root_vn).expect("stale root").get_addr().clone();
        let prev_addr = data.obank().get(previous_op).expect("stale prev").get_addr().clone();
        let address_tied = data.vbank().get(root_vn).expect("stale root").is_addr_tied();
        // We are creating a CONCAT stack, mark varnodes appropriately.
        if !address_tied {
            for &ov in out_varnodes {
                data.vbank_mut().get_mut(ov).expect("stale out").set_proto_partial();
            }
        }
        let mut vn: VarnodeId;
        let mut concat_op: OpId;
        let mut pre_op = previous_op;
        if base_addr.is_big_endian() {
            vn = out_varnodes[0];
            let mut i = 1usize;
            loop {
                concat_op = data.new_op(2, prev_addr.clone());
                data.op_set_opcode(concat_op, crate::typeop::type_op_for(OpCode::CPUI_PIECE));
                data.op_set_input(concat_op, vn, 0)?; // Most significant
                data.op_set_input(concat_op, out_varnodes[i], 1)?; // Least significant
                data.op_insert_after(concat_op, pre_op);
                if i + 1 >= out_varnodes.len() {
                    break;
                }
                pre_op = concat_op;
                let sz = data.vbank().get(vn).expect("stale vn").get_size()
                    + data.vbank().get(out_varnodes[i]).expect("stale out").get_size();
                let mut addr = base_addr.clone();
                addr.renormalize(sz, data.get_arch().manage())?;
                vn = data.new_varnode_out(sz, &addr, concat_op)?;
                if !address_tied {
                    data.vbank_mut().get_mut(vn).expect("stale vn").set_proto_partial();
                }
                i += 1;
            }
        } else {
            vn = out_varnodes[out_varnodes.len() - 1];
            let mut i: int4 = out_varnodes.len() as int4 - 2;
            loop {
                concat_op = data.new_op(2, prev_addr.clone());
                data.op_set_opcode(concat_op, crate::typeop::type_op_for(OpCode::CPUI_PIECE));
                data.op_set_input(concat_op, vn, 0)?; // Most significant
                data.op_set_input(concat_op, out_varnodes[i as usize], 1)?; // Least significant
                data.op_insert_after(concat_op, pre_op);
                if i <= 0 {
                    break;
                }
                pre_op = concat_op;
                let sz = data.vbank().get(vn).expect("stale vn").get_size()
                    + data.vbank().get(out_varnodes[i as usize]).expect("stale out").get_size();
                let mut addr = data.vbank().get(out_varnodes[i as usize]).expect("stale out").get_addr().clone();
                addr.renormalize(sz, data.get_arch().manage())?;
                vn = data.new_varnode_out(sz, &addr, concat_op)?;
                if !address_tied {
                    data.vbank_mut().get_mut(vn).expect("stale vn").set_proto_partial();
                }
                i -= 1;
            }
        }
        data.obank_mut().get_mut(concat_op).expect("stale concat").set_partial_root();
        data.op_set_output(concat_op, root_vn)?;
        if !address_tied {
            data.with_covermerge(|merge, data| merge.register_proto_partial_root(data, root_vn));
        }
        Ok(())
    }

    /// Build a series of PTRSUB/PTRADD ops at different offsets given a root
    /// pointer (C++ `SplitDatatype::buildPointers`, subflow.cc:2620).
    // C++ signature: buildPointers(Varnode *rootVn, TypePointer *ptrType, int4
    // baseOffset, PcodeOp *followOp, vector<Varnode*> &ptrVarnodes, bool isInput);
    // `self`+`data` replace the implicit `this`+`data` member, hence 8 params.
    #[allow(clippy::too_many_arguments)]
    fn build_pointers(
        &mut self,
        data: &mut Funcdata,
        root_vn: VarnodeId,
        ptr_type: &Rc<Datatype>,
        base_offset: int4,
        follow_op: OpId,
        ptr_varnodes: &mut Vec<VarnodeId>,
        is_input: bool,
    ) -> KunaResult<()> {
        let base_type = ptr_type.get_ptr_to().expect("buildPointers: ptrType not a pointer");
        let word_size = ptr_type.get_word_size().expect("buildPointers: ptrType wordsize");
        let ptr_size = ptr_type.get_size();
        let follow_addr = data.obank().get(follow_op).expect("stale follow").get_addr().clone();
        for i in 0..self.data_type_pieces.len() {
            let match_type = if is_input {
                Rc::clone(&self.data_type_pieces[i].in_type)
            } else {
                Rc::clone(&self.data_type_pieces[i].out_type)
            };
            let mut cur_off: int8 = base_offset as int8 + self.data_type_pieces[i].offset as int8;
            let mut tmp_type = Rc::clone(&base_type);
            let mut in_ptr = root_vn;
            loop {
                let new_off: int8;
                let new_type: Rc<Datatype>;
                if cur_off < 0 || cur_off >= tmp_type.get_size() as int8 {
                    // An offset not within the data-type indicates an array.
                    new_type = Rc::clone(&tmp_type);
                    let mut no = cur_off % tmp_type.get_size() as int8;
                    no = if no < 0 { no + tmp_type.get_size() as int8 } else { no };
                    new_off = no;
                } else {
                    let (sub, so) = tmp_type.get_sub_type(cur_off)?;
                    match sub {
                        Some(s) => {
                            new_type = s;
                            new_off = so;
                        }
                        None => {
                            // Null only for a hole in a structure; use precomputed type.
                            new_type = Rc::clone(&match_type);
                            new_off = 0;
                        }
                    }
                }
                let res_type = if new_type.needs_resolution() {
                    self.resolver.resolve(if is_input { 0 } else { 1 }, &new_type)
                } else {
                    Rc::clone(&new_type)
                };

                let new_op: OpId;
                if Rc::ptr_eq(&tmp_type, &res_type)
                    || tmp_type.get_metatype() == type_metatype::TYPE_ARRAY
                {
                    let mut final_offset = cur_off - new_off;
                    let mut sz = res_type.get_size(); // Element size in bytes
                    final_offset /= sz as int8; // Number of elements
                    // cast: byteToAddressInt(int4,..) returns int4-range here.
                    sz = AddrSpace::byte_to_address_int(sz as i64, word_size) as int4;
                    new_op = data.new_op(3, follow_addr.clone());
                    data.op_set_opcode(new_op, crate::typeop::type_op_for(OpCode::CPUI_PTRADD));
                    data.op_set_input(new_op, in_ptr, 0)?;
                    let in_ptr_size = data.vbank().get(in_ptr).expect("stale ptr").get_size();
                    // cast: finalOffset is an element index that fits the pointer width.
                    let index_vn = data.new_constant(in_ptr_size, final_offset as uintb);
                    data.op_set_input(new_op, index_vn, 1)?;
                    let szconst = data.new_constant(in_ptr_size, sz as uintb);
                    data.op_set_input(new_op, szconst, 2)?;
                    let index_size = data.vbank().get(index_vn).expect("stale idx").get_size();
                    let index_type = self.types()?.get_base(index_size, type_metatype::TYPE_INT)?;
                    data.vbank_mut().get_mut(index_vn).expect("stale idx").update_type(index_type);
                } else {
                    // cast: byteToAddressInt(int8,..) stays within int8.
                    let final_offset =
                        AddrSpace::byte_to_address_int(cur_off - new_off, word_size);
                    new_op = data.new_op(2, follow_addr.clone());
                    data.op_set_opcode(new_op, crate::typeop::type_op_for(OpCode::CPUI_PTRSUB));
                    data.op_set_input(new_op, in_ptr, 0)?;
                    let in_ptr_size = data.vbank().get(in_ptr).expect("stale ptr").get_size();
                    let offconst = data.new_constant(in_ptr_size, final_offset as uintb);
                    data.op_set_input(new_op, offconst, 1)?;
                }
                // resolver.inheritResolution — no-op for non-union types (W6/W7).
                let in_ptr_size = data.vbank().get(in_ptr).expect("stale ptr").get_size();
                in_ptr = data.new_unique_out(in_ptr_size, new_op)?;
                let tmp_ptr = self
                    .types()?
                    .get_type_pointer_strip_array(ptr_size, Rc::clone(&new_type), word_size)?;
                data.vbank_mut().get_mut(in_ptr).expect("stale ptr").update_type(tmp_ptr);
                data.op_insert_before(new_op, follow_op);
                tmp_type = res_type;
                cur_off = new_off;
                if tmp_type.get_size() <= match_type.get_size() {
                    break;
                }
            }
            ptr_varnodes.push(in_ptr);
        }
        Ok(())
    }

    /// Does the given Varnode have an arithmetic op as a descendant (C++
    /// `SplitDatatype::isArithmeticInput`, subflow.cc:2688).
    fn is_arithmetic_input(data: &Funcdata, vn: VarnodeId) -> bool {
        let descend: Vec<OpId> = data.vbank().get(vn).expect("stale vn").descend_iter().collect();
        for op in descend {
            if crate::typeop::type_op_info(data.obank().get(op).expect("stale desc").code())
                .is_arithmetic_op()
            {
                return true;
            }
        }
        false
    }

    /// Is the defining op arithmetic (C++ `SplitDatatype::isArithmeticOutput`,
    /// subflow.cc:2704).
    fn is_arithmetic_output(data: &Funcdata, vn: VarnodeId) -> bool {
        if !data.vbank().get(vn).expect("stale vn").is_written() {
            return false;
        }
        let def = data.vbank().get(vn).expect("stale vn").get_def().expect("vn def");
        crate::typeop::type_op_info(data.obank().get(def).expect("stale def").code())
            .is_arithmetic_op()
    }

    /// Split a COPY operation (C++ `SplitDatatype::splitCopy`, subflow.cc:2728).
    pub fn split_copy(
        &mut self,
        data: &mut Funcdata,
        copy_op: OpId,
        in_type: &Rc<Datatype>,
        out_type: &Rc<Datatype>,
    ) -> KunaResult<bool> {
        if !self.test_copy_constraints(data, copy_op) {
            return Ok(false);
        }
        let in_vn = data.obank().get(copy_op).expect("stale copy").get_in(0).expect("copy in0");
        let in_constant = data.vbank().get(in_vn).expect("stale in").is_constant();
        let types = self.types_clone()?;
        if !self.test_datatype_compatibility(&*types, in_type, out_type, in_constant)? {
            return Ok(false);
        }
        if Self::is_arithmetic_output(data, in_vn) {
            return Ok(false); // Sanity check on input
        }
        let out_vn = data.obank().get(copy_op).expect("stale copy").get_out().expect("copy out");
        if Self::is_arithmetic_input(data, out_vn) {
            return Ok(false); // Sanity check on output
        }
        let mut in_varnodes: Vec<VarnodeId> = Vec::new();
        let mut out_varnodes: Vec<VarnodeId> = Vec::new();
        let _unres_out_type = data.vbank().get(out_vn).expect("stale out").get_type().clone();
        // resolver.addResolution(0, unresOutType, copyOp, -1) — no-op (non-union).
        if in_constant {
            let big = data.vbank().get(out_vn).expect("stale out").get_space().is_big_endian();
            self.build_in_constants(data, in_vn, &mut in_varnodes, big);
        } else {
            self.build_in_subpieces(data, in_vn, copy_op, &mut in_varnodes)?;
        }
        self.build_out_varnodes(data, out_vn, &mut out_varnodes)?;
        self.build_out_concats(data, out_vn, copy_op, &out_varnodes)?;
        let copy_addr = data.obank().get(copy_op).expect("stale copy").get_addr().clone();
        for i in 0..in_varnodes.len() {
            let new_copy_op = data.new_op(1, copy_addr.clone());
            data.op_set_opcode(new_copy_op, crate::typeop::type_op_for(OpCode::CPUI_COPY));
            data.op_set_input(new_copy_op, in_varnodes[i], 0)?;
            data.op_set_output(new_copy_op, out_varnodes[i])?;
            data.op_insert_before(new_copy_op, copy_op);
            // resolver.inheritResolution — no-op for non-union types.
        }
        data.op_destroy(copy_op);
        Ok(true)
    }

    /// Split a LOAD operation (C++ `SplitDatatype::splitLoad`, subflow.cc:2770).
    pub fn split_load(
        &mut self,
        data: &mut Funcdata,
        load_op: OpId,
        in_type: &Rc<Datatype>,
    ) -> KunaResult<bool> {
        self.is_load_store = true;
        let mut out_vn = data.obank().get(load_op).expect("stale load").get_out().expect("load out");
        let mut copy_op: Option<OpId> = None;
        if !data.vbank().get(out_vn).expect("stale out").is_addr_tied() {
            copy_op = data.lone_descend(out_vn);
        }
        if let Some(cop) = copy_op {
            let opc = data.obank().get(cop).expect("stale copy").code();
            if opc == OpCode::CPUI_STORE {
                return Ok(false); // Handled by RuleSplitStore
            }
            if opc == OpCode::CPUI_ZPULL || opc == OpCode::CPUI_SPULL {
                return Ok(false);
            }
            if opc != OpCode::CPUI_COPY {
                copy_op = None;
            }
        }
        if let Some(cop) = copy_op {
            out_vn = data.obank().get(cop).expect("stale copy").get_out().expect("copy out");
        }
        let out_type = data.vbank().get(out_vn).expect("stale out").get_type_def_facing().clone();
        let types = self.types_clone()?;
        if !self.test_datatype_compatibility(&*types, in_type, &out_type, false)? {
            return Ok(false);
        }
        if Self::is_arithmetic_input(data, out_vn) {
            return Ok(false); // Sanity check on output
        }
        let mut root = RootPointer::default();
        if !root.find(data, &mut self.resolver, load_op, in_type)? {
            return Ok(false);
        }
        let mut ptr_varnodes: Vec<VarnodeId> = Vec::new();
        let mut out_varnodes: Vec<VarnodeId> = Vec::new();
        let insert_point = copy_op.unwrap_or(load_op);
        let root_ptr = root.pointer;
        let root_ptr_type = root.ptr_type.clone().expect("root ptrType");
        let root_base = root.base_offset;
        self.build_pointers(data, root_ptr, &root_ptr_type, root_base, load_op, &mut ptr_varnodes, true)?;
        self.build_out_varnodes(data, out_vn, &mut out_varnodes)?;
        self.build_out_concats(data, out_vn, insert_point, &out_varnodes)?;
        let load_in0 = data.obank().get(load_op).expect("stale load").get_in(0).expect("load in0");
        let spc = space_from_const(data, load_in0);
        let insert_addr = data.obank().get(insert_point).expect("stale insert").get_addr().clone();
        for i in 0..ptr_varnodes.len() {
            let new_load_op = data.new_op(2, insert_addr.clone());
            data.op_set_opcode(new_load_op, crate::typeop::type_op_for(OpCode::CPUI_LOAD));
            let spcvn = data.new_varnode_space(&spc);
            data.op_set_input(new_load_op, spcvn, 0)?;
            data.op_set_input(new_load_op, ptr_varnodes[i], 1)?;
            data.op_set_output(new_load_op, out_varnodes[i])?;
            data.op_insert_before(new_load_op, insert_point);
        }
        if let Some(cop) = copy_op {
            data.op_destroy(cop);
        }
        data.op_destroy(load_op);
        root.free_pointer_chain(data);
        Ok(true)
    }

    /// Split a STORE operation (C++ `SplitDatatype::splitStore`, subflow.cc:2823).
    pub fn split_store(
        &mut self,
        data: &mut Funcdata,
        store_op: OpId,
        out_type: &Rc<Datatype>,
    ) -> KunaResult<bool> {
        self.is_load_store = true;
        let in_vn = data.obank().get(store_op).expect("stale store").get_in(2).expect("store in2");
        let mut load_op: Option<OpId> = None;
        let mut in_type: Option<Rc<Datatype>> = None;
        if data.vbank().get(in_vn).expect("stale in").is_written() {
            let def = data.vbank().get(in_vn).expect("stale in").get_def().expect("in def");
            if data.obank().get(def).expect("stale def").code() == OpCode::CPUI_LOAD
                && data.lone_descend(in_vn) == Some(store_op)
            {
                let in_size = data.vbank().get(in_vn).expect("stale in").get_size();
                load_op = Some(def);
                in_type = Self::get_value_datatype(data, def, in_size);
                if in_type.is_none() {
                    load_op = None;
                }
            }
        }
        if in_type.is_none() {
            in_type =
                Some(data.vbank().get(in_vn).expect("stale in").get_type_read_facing(store_op).clone());
        }
        let mut in_type = in_type.expect("inType set above");
        let in_constant = data.vbank().get(in_vn).expect("stale in").is_constant();
        let types = self.types_clone()?;
        if !self.test_datatype_compatibility(&*types, &in_type, out_type, in_constant)? {
            if load_op.is_some() {
                // Not compatible considering the LOAD; retry without the LOAD.
                load_op = None;
                in_type = data.vbank().get(in_vn).expect("stale in").get_type_read_facing(store_op).clone();
                self.data_type_pieces.clear();
                if !self.test_datatype_compatibility(&*types, &in_type, out_type, in_constant)? {
                    return Ok(false);
                }
            } else {
                return Ok(false);
            }
        }
        if Self::is_arithmetic_output(data, in_vn) {
            return Ok(false); // Sanity check
        }
        let mut store_root = RootPointer::default();
        if !store_root.find(data, &mut self.resolver, store_op, out_type)? {
            return Ok(false);
        }
        let mut load_root = RootPointer::default();
        if let Some(lop) = load_op {
            if !load_root.find(data, &mut self.resolver, lop, &in_type)? {
                return Ok(false);
            }
        }
        let store_in0 = data.obank().get(store_op).expect("stale store").get_in(0).expect("store in0");
        let store_space = space_from_const(data, store_in0);
        let mut in_varnodes: Vec<VarnodeId> = Vec::new();
        if in_constant {
            self.build_in_constants(data, in_vn, &mut in_varnodes, store_space.is_big_endian());
        } else if let Some(lop) = load_op {
            let mut load_ptrs: Vec<VarnodeId> = Vec::new();
            let lr_ptr = load_root.pointer;
            let lr_type = load_root.ptr_type.clone().expect("load root ptrType");
            let lr_base = load_root.base_offset;
            self.build_pointers(data, lr_ptr, &lr_type, lr_base, lop, &mut load_ptrs, true)?;
            let load_in0 = data.obank().get(lop).expect("stale load").get_in(0).expect("load in0");
            let load_space = space_from_const(data, load_in0);
            let load_addr = data.obank().get(lop).expect("stale load").get_addr().clone();
            // Parallel loop over load_ptrs and dataTypePieces (equal length): the
            // C++ indexes both by `i`; iterate the pointer chain by value here.
            for (i, load_ptr) in load_ptrs.iter().copied().enumerate() {
                let new_load_op = data.new_op(2, load_addr.clone());
                data.op_set_opcode(new_load_op, crate::typeop::type_op_for(OpCode::CPUI_LOAD));
                let spcvn = data.new_varnode_space(&load_space);
                data.op_set_input(new_load_op, spcvn, 0)?;
                data.op_set_input(new_load_op, load_ptr, 1)?;
                let dt = Rc::clone(&self.data_type_pieces[i].in_type);
                let vn = data.new_unique_out(dt.get_size(), new_load_op)?;
                data.vbank_mut().get_mut(vn).expect("stale load out").update_type(dt);
                in_varnodes.push(vn);
                data.op_insert_before(new_load_op, lop);
            }
        } else {
            self.build_in_subpieces(data, in_vn, store_op, &mut in_varnodes)?;
        }

        let mut store_ptrs: Vec<VarnodeId> = Vec::new();
        if data.vbank().get(store_root.pointer).expect("stale root").is_addr_tied() {
            store_root.duplicate_to_temp(data, store_op)?;
        }
        let sr_ptr = store_root.pointer;
        let sr_type = store_root.ptr_type.clone().expect("store root ptrType");
        let sr_base = store_root.base_offset;
        self.build_pointers(data, sr_ptr, &sr_type, sr_base, store_op, &mut store_ptrs, false)?;
        // Preserve original STORE object (INDIRECT refs stay valid); convert it
        // into the first of the smaller STOREs.
        data.op_set_input(store_op, store_ptrs[0], 1)?;
        data.op_set_input(store_op, in_varnodes[0], 2)?;
        let mut last_store = store_op;
        let store_addr = data.obank().get(store_op).expect("stale store").get_addr().clone();
        for i in 1..store_ptrs.len() {
            let new_store_op = data.new_op(3, store_addr.clone());
            data.op_set_opcode(new_store_op, crate::typeop::type_op_for(OpCode::CPUI_STORE));
            let spcvn = data.new_varnode_space(&store_space);
            data.op_set_input(new_store_op, spcvn, 0)?;
            data.op_set_input(new_store_op, store_ptrs[i], 1)?;
            data.op_set_input(new_store_op, in_varnodes[i], 2)?;
            data.op_insert_after(new_store_op, last_store);
            last_store = new_store_op;
        }

        if let Some(lop) = load_op {
            data.op_destroy(lop);
            load_root.free_pointer_chain(data);
        }
        store_root.free_pointer_chain(data);
        Ok(true)
    }

    /// Get a data-type description of the value pointed at by a LOAD/STORE (C++
    /// `SplitDatatype::getValueDatatype`, subflow.cc:2925).
    pub fn get_value_datatype(
        data: &Funcdata,
        load_store: OpId,
        size: int4,
    ) -> Option<Rc<Datatype>> {
        let in1 = data.obank().get(load_store).expect("stale ls").get_in(1)?;
        let ptr_type = data.vbank().get(in1).expect("stale in1").get_type_read_facing(load_store).clone();
        if ptr_type.get_metatype() != type_metatype::TYPE_PTR {
            return None;
        }
        let res_type: Rc<Datatype>;
        let base_offset: int4;
        if ptr_type.is_pointer_rel() {
            res_type = ptr_type.get_rel_parent()?;
            base_offset = ptr_type.get_byte_offset()?;
        } else {
            res_type = ptr_type.get_ptr_to()?;
            base_offset = 0;
        }
        let metain = res_type.get_metatype();
        let tlst = data.get_arch().types()?;
        if res_type.get_align_size() < size {
            if metain == type_metatype::TYPE_INT
                || metain == type_metatype::TYPE_UINT
                || metain == type_metatype::TYPE_BOOL
                || metain == type_metatype::TYPE_FLOAT
                || metain == type_metatype::TYPE_PTR
            {
                if size % res_type.get_align_size() == 0 {
                    let num_el = size / res_type.get_align_size();
                    return tlst.get_type_array(num_el, res_type).ok();
                }
            }
        } else if metain == type_metatype::TYPE_STRUCT || metain == type_metatype::TYPE_ARRAY {
            return tlst.get_exact_piece(res_type, base_offset, size).ok().flatten();
        }
        None
    }

    /// Borrow the held factory as a `&dyn TypeFactory` (the explicit-factory
    /// methods need a trait object); errors if no factory is attached.
    #[allow(dead_code)]
    fn types_handle(&self) -> KunaResult<&dyn crate::dtype::TypeFactory> {
        Ok(self.types()? as &dyn crate::dtype::TypeFactory)
    }

    /// Clone the held factory `Rc` so it can be borrowed (`&*`) as the explicit
    /// `types` argument of `test_datatype_compatibility`/`get_component` while
    /// `self` is borrowed mutably (the C++ `types` is a separate member, not part
    /// of the mutable `dataTypePieces` state).
    fn types_clone(&self) -> KunaResult<Rc<crate::dtype::TypeFactoryImpl>> {
        self.types
            .clone()
            .ok_or_else(|| KunaError::lowlevel("SplitDatatype: no TypeFactory on Architecture"))
    }
}

/// A root pointer with an accumulated offset to the structure/array being split
/// (C++ `SplitDatatype::RootPointer`, subflow.hh).
#[derive(Default)]
struct RootPointer {
    /// The LOAD or STORE op (C++ `loadStore`).
    load_store: Option<OpId>,
    /// Base pointer data-type of the LOAD/STORE (C++ `ptrType`).
    ptr_type: Option<Rc<Datatype>>,
    /// First (original) pointer in the chain (C++ `firstPointer`).
    first_pointer: Option<VarnodeId>,
    /// The current root pointer Varnode (C++ `pointer`).
    pointer: VarnodeId,
    /// Offset of the structure/array relative to the root pointer (C++ `baseOffset`).
    base_offset: int4,
}

impl RootPointer {
    /// Back up the root pointer through a COPY/INT_ADD/PTRSUB/PTRADD (C++
    /// `SplitDatatype::RootPointer::backUpPointer`, subflow.cc:2098).
    fn back_up_pointer(&mut self, data: &Funcdata, implied_base: Option<&Rc<Datatype>>) -> bool {
        let pointer = self.pointer;
        if !data.vbank().get(pointer).expect("stale ptr").is_written() {
            return false;
        }
        let add_op = data.vbank().get(pointer).expect("stale ptr").get_def().expect("ptr def");
        let opc = data.obank().get(add_op).expect("stale add").code();
        let off: int4;
        if opc == OpCode::CPUI_PTRSUB || opc == OpCode::CPUI_INT_ADD || opc == OpCode::CPUI_PTRADD {
            let cvn = data.obank().get(add_op).expect("stale add").get_in(1).expect("add in1");
            if !data.vbank().get(cvn).expect("stale cvn").is_constant() {
                return false;
            }
            // cast: pointer offsets are int4-range here (matches C++ `(int4)`).
            off = data.vbank().get(cvn).expect("stale cvn").get_offset() as int4;
        } else if opc == OpCode::CPUI_COPY {
            off = 0;
        } else {
            return false;
        }
        let tmp_pointer = data.obank().get(add_op).expect("stale add").get_in(0).expect("add in0");
        let ct = data.vbank().get(tmp_pointer).expect("stale tmp").get_type_read_facing(add_op).clone();
        if ct.get_metatype() != type_metatype::TYPE_PTR {
            return false;
        }
        let parent = ct.get_ptr_to().expect("pointer has ptrTo");
        let meta = parent.get_metatype();
        if meta != type_metatype::TYPE_STRUCT && meta != type_metatype::TYPE_ARRAY {
            let parent_is_implied = match implied_base {
                Some(ib) => Rc::ptr_eq(&parent, ib),
                None => false,
            };
            if (opc != OpCode::CPUI_PTRADD && opc != OpCode::CPUI_COPY) || !parent_is_implied {
                return false;
            }
        }
        let word_size = ct.get_word_size().expect("pointer wordsize");
        self.ptr_type = Some(ct);
        let mut off = off;
        if opc == OpCode::CPUI_PTRADD {
            let i2 = data.obank().get(add_op).expect("stale add").get_in(2).expect("ptradd in2");
            // cast: PTRADD element multiplier is int4-range.
            off *= data.vbank().get(i2).expect("stale i2").get_offset() as int4;
        }
        // cast: addressToByteInt(int4,..) returns int4-range.
        off = AddrSpace::address_to_byte_int(off as i64, word_size) as int4;
        self.base_offset += off;
        self.pointer = tmp_pointer;
        true
    }

    /// Find the root pointer to the given value data-type (C++
    /// `SplitDatatype::RootPointer::find`, subflow.cc:2144).
    fn find(
        &mut self,
        data: &Funcdata,
        resolver: &mut crate::unionresolve::ResolveCache,
        op: OpId,
        value_type: &Rc<Datatype>,
    ) -> KunaResult<bool> {
        let mut implied_base: Option<Rc<Datatype>> = None;
        let mut value_type = Rc::clone(value_type);
        if value_type.get_metatype() == type_metatype::TYPE_PARTIALSTRUCT {
            value_type = value_type.get_partial_base().expect("partial parent");
        }
        if value_type.get_metatype() == type_metatype::TYPE_ARRAY {
            value_type = value_type.get_array_base().expect("array base");
            implied_base = Some(Rc::clone(&value_type)); // allow implied array (pointer to element)
        }
        let key = if data.obank().get(op).expect("stale op").code() == OpCode::CPUI_LOAD { 0 } else { 1 };
        self.load_store = Some(op);
        self.base_offset = 0;
        let in1 = data.obank().get(op).expect("stale op").get_in(1).expect("ls in1");
        self.first_pointer = Some(in1);
        self.pointer = in1;
        let ct = data.vbank().get(in1).expect("stale in1").get_type_read_facing(op).clone();
        if ct.get_metatype() != type_metatype::TYPE_PTR {
            return Ok(false);
        }
        // resolver.addResolution(key, pointer->getType(), op, 1) — no-op (non-union).
        let _ = (resolver, key);
        self.ptr_type = Some(Rc::clone(&ct));
        let ptr_to = ct.get_ptr_to().expect("pointer ptrTo");
        if !Rc::ptr_eq(&ptr_to, &value_type) {
            if implied_base.is_some() {
                return Ok(false);
            }
            if !self.back_up_pointer(data, implied_base.as_ref()) {
                return Ok(false);
            }
            let cur_ptr_to = self.ptr_type.as_ref().expect("ptrType").get_ptr_to().expect("ptrTo");
            if !Rc::ptr_eq(&cur_ptr_to, &value_type) {
                return Ok(false);
            }
        }
        // Back up to pointers to containing structures or arrays.
        for _ in 0..3 {
            if data.vbank().get(self.pointer).expect("stale ptr").is_addr_tied()
                || data.lone_descend(self.pointer).is_none()
            {
                break;
            }
            if !self.back_up_pointer(data, implied_base.as_ref()) {
                break;
            }
            // resolver.addResolution — no-op (non-union).
        }
        Ok(true)
    }

    /// COPY the root pointer into a unique temp so subsequent STOREs cannot mutate
    /// it (C++ `SplitDatatype::RootPointer::duplicateToTemp`, subflow.cc:2187).
    fn duplicate_to_temp(&mut self, data: &mut Funcdata, follow_op: OpId) -> KunaResult<()> {
        let new_root = data.build_copy_temp(self.pointer, follow_op)?;
        let ptr_type = self.ptr_type.clone().expect("ptrType");
        data.vbank_mut().get_mut(new_root).expect("stale new root").update_type(ptr_type);
        self.pointer = new_root;
        Ok(())
    }

    /// Remove the now-dead pointer chain (C++
    /// `SplitDatatype::RootPointer::freePointerChain`, subflow.cc:2199).
    fn free_pointer_chain(&mut self, data: &mut Funcdata) {
        let mut first = self.first_pointer.expect("firstPointer");
        while first != self.pointer
            && !data.vbank().get(first).expect("stale first").is_addr_tied()
            && data.vbank().get(first).expect("stale first").has_no_descend()
        {
            let tmp_op = data.vbank().get(first).expect("stale first").get_def().expect("first def");
            first = data.obank().get(tmp_op).expect("stale tmp").get_in(0).expect("tmp in0");
            data.op_destroy(tmp_op);
        }
        self.first_pointer = Some(first);
    }
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
        let in0 = data.obank().get(op).expect("stale op").get_in(0).expect("copy in0");
        let in_type = data.vbank().get(in0).expect("stale in0").get_type_read_facing(op).clone();
        let out = data.obank().get(op).expect("stale op").get_out().expect("copy out");
        let out_type = data.vbank().get(out).expect("stale out").get_type_def_facing().clone();
        let metain = in_type.get_metatype();
        let metaout = out_type.get_metatype();
        if metain != type_metatype::TYPE_PARTIALSTRUCT
            && metaout != type_metatype::TYPE_PARTIALSTRUCT
            && metain != type_metatype::TYPE_ARRAY
            && metaout != type_metatype::TYPE_ARRAY
            && metain != type_metatype::TYPE_STRUCT
            && metaout != type_metatype::TYPE_STRUCT
        {
            return 0;
        }
        let mut splitter = SplitDatatype::from_funcdata(data);
        match splitter.split_copy(data, op, &in_type, &out_type) {
            Ok(true) => 1,
            _ => 0,
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
        let mut splitter = SplitDatatype::from_funcdata(data);
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
        let mut splitter = SplitDatatype::from_funcdata(data);
        match splitter.split_store(data, op, &out_type) {
            Ok(true) => 1,
            _ => 0,
        }
    }
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
        // C++ applyOp is infallible, so we run the body and map any stub error to
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
            // fd->opSetOpcode(op, CPUI_COPY).  STUB(W6): glb->inst[CPUI_COPY] — the
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
// storage.  The trace/precision engine is ported verbatim, including the
// FloatFormat acquisition (`f->getArch()->translate->getFloatFormat`, held as
// the `arch` handle) and the `setReplacement` constant-conversion
// (`format->convertEncoding`).  Only SubfloatFlow's `preserveAddress` override
// (`return vn->isInput()`) remains a transform stub — it cannot be injected into
// the merged `TransformManager::new_piece` (transform.rs, not owned).

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
    /// True if a usable FloatFormat was found (C++ `format != 0`).
    has_format: bool,
    /// The arch handle (`Rc<Architecture>`) carrying `translate->getFloatFormat`
    /// (C++ `format = f->getArch()->translate->getFloatFormat(precision)`).  Held
    /// so [`set_replacement`]'s constant branch can `convertEncoding` between the
    /// source and `precision` float formats.
    arch: ArchHandle,
    /// Current list of placeholders still to be traced (C++ `worklist`).
    worklist: Vec<TVarRef>,
    /// Maximum precision flowing into a particular float op (C++ `maxPrecisionMap`).
    max_precision_map: BTreeMap<OpId, int4>,
}

impl SubfloatFlow {
    /// Construct the engine for a function/root/precision (C++
    /// `SubfloatFlow::SubfloatFlow`, subflow.cc:3456).
    ///
    /// `format = f->getArch()->translate->getFloatFormat(precision)`; if the
    /// arch has no format for the requested precision, the engine degrades to
    /// the C++ null-format early return (`setReplacement(root)` is skipped).
    pub fn new(data: &mut Funcdata, root: VarnodeId, prec: int4) -> SubfloatFlow {
        let arch = data.get_arch().clone();
        let has_format = arch.get_float_format(prec).is_some();
        let mut sf = SubfloatFlow {
            tm: TransformManager::new(),
            precision: prec,
            terminator_count: 0,
            has_format,
            arch,
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
            let vn_size = v.get_size();
            let off = v.get_offset();
            let form2 = match self.arch.get_float_format(vn_size) {
                Some(f) => f,
                None => return Ok(None), // Unsupported constant format
            };
            // `format` is the precision-size target format (guaranteed present:
            // `has_format` gated entry into the trace).
            let format = self
                .arch
                .get_float_format(self.precision)
                .expect("set_replacement: precision FloatFormat checked at construction");
            let converted = format.convert_encoding(off, form2);
            return Ok(Some(self.tm.new_constant(self.precision, 0, converted)));
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
    /// Materializes the placeholder graph into real IR (W10).
    pub fn apply(&mut self, data: &mut Funcdata) -> KunaResult<()> {
        self.tm.apply(data)
    }
}

// =============================================================================
// LaneDivide (subflow.hh:428, subflow.cc:3533-4143)
// =============================================================================

/// A `LaneDivide` work-list entry (C++ `LaneDivide::WorkNode`, subflow.hh:430):
/// the lane placeholders of one Varnode together with its lane-count / skip.
struct LaneWorkNode {
    /// Lane placeholders for the underlying Varnode (C++ `lanes`).
    lanes: TVarRef,
    /// Number of lanes in the particular Varnode (C++ `numLanes`).
    num_lanes: int4,
    /// Number of lanes to skip in the global description (C++ `skipLanes`).
    skip_lanes: int4,
}

/// Split a large vector register into a set of explicit logical lanes (C++
/// `LaneDivide : public TransformManager`, subflow.hh:428).
///
/// Starting from a root Varnode and a [`LaneDescription`], the engine pushes the
/// lane scheme as far through the data-flow as possible ([`do_trace`]); the base
/// [`TransformManager::apply`] then materializes each lane as an explicit
/// Varnode.  This is what removes the `SUB(XMM,0)`/`CONCAT(XMM_Qb,..)` lane
/// noise from x86 XMM/ZMM vector accesses so the float values flow cleanly.
///
/// The C++ `TransformVar *` pointer arithmetic over a split lane array is the
/// [`TVarRef::Piece`] handle plus [`lane_at`] / [`lane_above`]; the per-lane
/// subset placeholders come from [`TransformManager::get_split_subset`] /
/// [`TransformManager::new_split_subset`].
///
/// [`do_trace`]: LaneDivide::do_trace
pub struct LaneDivide {
    /// The transform-manager state (C++ base class).
    tm: TransformManager,
    /// Global description of lanes that need to be split (C++ `description`).
    description: LaneDescription,
    /// List of Varnodes still left to trace (C++ `workList`).
    work_list: Vec<LaneWorkNode>,
    /// `true` if we allow lanes to be cast (via SUBPIECE) to a smaller integer
    /// size (C++ `allowSubpieceTerminator`).
    allow_subpiece_terminator: bool,
}

impl LaneDivide {
    /// C++ `LaneDivide::LaneDivide` (subflow.cc:4117).
    ///
    /// `f is the function being transformed`, `root` is the root Varnode to start
    /// tracing lanes from, `desc` describes the root's lanes, and `allowDowncast`
    /// is `true` if SUBPIECE may be treated as terminating.
    pub fn new(
        data: &mut Funcdata,
        root: VarnodeId,
        desc: LaneDescription,
        allow_downcast: bool,
    ) -> LaneDivide {
        let num = desc.get_num_lanes();
        let mut ld = LaneDivide {
            tm: TransformManager::new(),
            description: desc,
            work_list: Vec::new(),
            allow_subpiece_terminator: allow_downcast,
        };
        ld.set_replacement(data, root, num, 0);
        ld
    }

    /// Find or build the placeholder objects for a Varnode that needs to be split
    /// into lanes (C++ `LaneDivide::setReplacement`, subflow.cc:3533).
    ///
    /// The Varnode is split based on the given subset of the lane description.
    /// Constants can be split.  Returns `None` for the C++ `(TransformVar *)0`.
    fn set_replacement(
        &mut self,
        data: &mut Funcdata,
        vn: VarnodeId,
        num_lanes: int4,
        skip_lanes: int4,
    ) -> Option<TVarRef> {
        let v = data.vbank().get(vn).expect("set_replacement: stale vn");
        if v.is_mark() {
            // Already seen before
            return Some(self.tm.get_split_subset(data, vn, &self.description, num_lanes, skip_lanes));
        }

        if v.is_constant() {
            return Some(self.tm.new_split_subset(data, vn, &self.description, num_lanes, skip_lanes));
        }

        // Allow free varnodes to be split (the C++ keeps the isFree() abort
        // commented out, so we likewise do not abort here).

        if v.is_type_lock() {
            let meta = v.get_type().get_metatype();
            if meta > type_metatype::TYPE_ARRAY {
                return None; // Don't split a primitive type
            }
            if meta == type_metatype::TYPE_STRUCT || meta == type_metatype::TYPE_UNION {
                return None;
            }
        }

        let is_free = v.is_free();
        data.vbank_mut().get_mut(vn).expect("set_replacement: stale vn").set_mark();
        let res = self.tm.new_split_subset(data, vn, &self.description, num_lanes, skip_lanes);
        if !is_free {
            self.work_list.push(LaneWorkNode { lanes: res, num_lanes, skip_lanes });
        }
        Some(res)
    }

    /// Build unary op placeholders with the same opcode across a set of lanes
    /// (C++ `LaneDivide::buildUnaryOp`, subflow.cc:3574).
    fn build_unary_op(
        &mut self,
        opc: OpCode,
        op: OpId,
        in_vars: TVarRef,
        out_vars: TVarRef,
        num_lanes: int4,
    ) {
        for i in 0..num_lanes {
            let rop = self.tm.new_op_replace(1, opc, op);
            self.tm.op_set_output(rop, lane_at(out_vars, i));
            self.tm.op_set_input(rop, lane_at(in_vars, i), 0);
        }
    }

    /// Build binary op placeholders with the same opcode across a set of lanes
    /// (C++ `LaneDivide::buildBinaryOp`, subflow.cc:3593).
    fn build_binary_op(
        &mut self,
        opc: OpCode,
        op: OpId,
        in0_vars: TVarRef,
        in1_vars: TVarRef,
        out_vars: TVarRef,
        num_lanes: int4,
    ) {
        for i in 0..num_lanes {
            let rop = self.tm.new_op_replace(2, opc, op);
            self.tm.op_set_output(rop, lane_at(out_vars, i));
            self.tm.op_set_input(rop, lane_at(in0_vars, i), 0);
            self.tm.op_set_input(rop, lane_at(in1_vars, i), 1);
        }
    }

    /// Convert a CPUI_PIECE into copies between placeholders, given the output
    /// lanes (C++ `LaneDivide::buildPiece`, subflow.cc:3614).
    fn build_piece(
        &mut self,
        data: &mut Funcdata,
        op: OpId,
        out_vars: TVarRef,
        num_lanes: int4,
        skip_lanes: int4,
    ) -> bool {
        let high_vn = data.obank().get(op).expect("build_piece: stale op").get_in(0).expect("in0");
        let low_vn = data.obank().get(op).expect("build_piece: stale op").get_in(1).expect("in1");
        let low_size = data.vbank().get(low_vn).expect("build_piece: stale lowVn").get_size();
        let high_size = data.vbank().get(high_vn).expect("build_piece: stale highVn").get_size();

        let (high_lanes, high_skip) =
            match self.description.restriction(num_lanes, skip_lanes, low_size, high_size) {
                Some(v) => v,
                None => return false,
            };
        let (low_lanes, low_skip) =
            match self.description.restriction(num_lanes, skip_lanes, 0, low_size) {
                Some(v) => v,
                None => return false,
            };
        if high_lanes == 1 {
            let high_rvn = self.tm.get_preexisting_varnode(data, high_vn);
            let rop = self.tm.new_op_replace(1, OpCode::CPUI_COPY, op);
            self.tm.op_set_input(rop, high_rvn, 0);
            self.tm.op_set_output(rop, lane_at(out_vars, num_lanes - 1));
        } else {
            // Multi-lane high
            let high_rvn = match self.set_replacement(data, high_vn, high_lanes, high_skip) {
                Some(r) => r,
                None => return false,
            };
            let out_high_start = num_lanes - high_lanes;
            for i in 0..high_lanes {
                let rop = self.tm.new_op_replace(1, OpCode::CPUI_COPY, op);
                self.tm.op_set_input(rop, lane_at(high_rvn, i), 0);
                self.tm.op_set_output(rop, lane_at(out_vars, out_high_start + i));
            }
        }
        if low_lanes == 1 {
            let low_rvn = self.tm.get_preexisting_varnode(data, low_vn);
            let rop = self.tm.new_op_replace(1, OpCode::CPUI_COPY, op);
            self.tm.op_set_input(rop, low_rvn, 0);
            self.tm.op_set_output(rop, out_vars);
        } else {
            // Multi-lane low
            let low_rvn = match self.set_replacement(data, low_vn, low_lanes, low_skip) {
                Some(r) => r,
                None => return false,
            };
            for i in 0..low_lanes {
                let rop = self.tm.new_op_replace(1, OpCode::CPUI_COPY, op);
                self.tm.op_set_input(rop, lane_at(low_rvn, i), 0);
                self.tm.op_set_output(rop, lane_at(out_vars, i));
            }
        }
        true
    }

    /// Split a CPUI_MULTIEQUAL into per-lane MULTIEQUALs (C++
    /// `LaneDivide::buildMultiequal`, subflow.cc:3669).
    fn build_multiequal(
        &mut self,
        data: &mut Funcdata,
        op: OpId,
        out_vars: TVarRef,
        num_lanes: int4,
        skip_lanes: int4,
    ) -> bool {
        let mut in_var_sets: Vec<TVarRef> = Vec::new();
        let num_input = data.obank().get(op).expect("build_multiequal: stale op").num_input();
        for i in 0..num_input {
            let ini = data.obank().get(op).expect("build_multiequal: stale op").get_in(i).expect("in");
            let in_vn = match self.set_replacement(data, ini, num_lanes, skip_lanes) {
                Some(v) => v,
                None => return false,
            };
            in_var_sets.push(in_vn);
        }
        for i in 0..num_lanes {
            let rop = self.tm.new_op_replace(num_input, OpCode::CPUI_MULTIEQUAL, op);
            self.tm.op_set_output(rop, lane_at(out_vars, i));
            for j in 0..num_input {
                self.tm.op_set_input(rop, lane_at(in_var_sets[j as usize], i), j);
            }
        }
        true
    }

    /// Split a CPUI_INDIRECT into per-lane INDIRECTs sharing the affecting iop
    /// (C++ `LaneDivide::buildIndirect`, subflow.cc:3696).
    fn build_indirect(
        &mut self,
        data: &mut Funcdata,
        op: OpId,
        out_vars: TVarRef,
        num_lanes: int4,
        skip_lanes: int4,
    ) -> bool {
        let in0 = data.obank().get(op).expect("build_indirect: stale op").get_in(0).expect("in0");
        let in_vn = match self.set_replacement(data, in0, num_lanes, skip_lanes) {
            Some(v) => v,
            None => return false,
        };
        for i in 0..num_lanes {
            let rop = self.tm.new_op_replace(2, OpCode::CPUI_INDIRECT, op);
            self.tm.op_set_output(rop, lane_at(out_vars, i));
            self.tm.op_set_input(rop, lane_at(in_vn, i), 0);
            let in1 = data.obank().get(op).expect("build_indirect: stale op").get_in(1).expect("in1");
            let iop = self.tm.new_iop(data, in1);
            self.tm.op_set_input(rop, iop, 1);
            self.tm.inherit_indirect(rop, data, op);
        }
        true
    }

    /// Split a CPUI_STORE into a sequence of per-lane STOREs (C++
    /// `LaneDivide::buildStore`, subflow.cc:3719).
    fn build_store(
        &mut self,
        data: &mut Funcdata,
        op: OpId,
        num_lanes: int4,
        skip_lanes: int4,
    ) -> bool {
        let in2 = data.obank().get(op).expect("build_store: stale op").get_in(2).expect("in2");
        let in_vars = match self.set_replacement(data, in2, num_lanes, skip_lanes) {
            Some(v) => v,
            None => return false,
        };
        let in0 = data.obank().get(op).expect("build_store: stale op").get_in(0).expect("in0");
        let space_const = data.vbank().get(in0).expect("build_store: stale in0").get_offset();
        let space_const_size = data.vbank().get(in0).expect("build_store: stale in0").get_size();
        let spc = space_from_const(data, in0); // Address space being stored to
        let orig_ptr = data.obank().get(op).expect("build_store: stale op").get_in(1).expect("in1");
        let orig_ptr_v = data.vbank().get(orig_ptr).expect("build_store: stale origPtr");
        if orig_ptr_v.is_free() && !orig_ptr_v.is_constant() {
            return false;
        }
        let base_ptr = self.tm.get_preexisting_varnode(data, orig_ptr);
        let ptr_size = data.vbank().get(orig_ptr).expect("build_store: stale origPtr").get_size();
        // Order lanes by pointer offset.  little = least to most, big = most to least.
        let mut byte_pos: int8 = 0;
        let is_big = spc.is_big_endian();
        for c in 0..num_lanes {
            let i = if is_big { num_lanes - 1 - c } else { c };
            let rop_store = self.tm.new_op_replace(3, OpCode::CPUI_STORE, op);
            // Construct the pointer
            let ptr_vn = if byte_pos == 0 {
                base_ptr
            } else {
                let ptr_vn = self.tm.new_unique(ptr_size);
                let add_op = self.tm.new_op(2, OpCode::CPUI_INT_ADD, rop_store);
                self.tm.op_set_output(add_op, ptr_vn);
                self.tm.op_set_input(add_op, base_ptr, 0);
                let cst = self.tm.new_constant(ptr_size, 0, byte_pos as uintb);
                self.tm.op_set_input(add_op, cst, 1);
                ptr_vn
            };
            let spc_cst = self.tm.new_constant(space_const_size, 0, space_const);
            self.tm.op_set_input(rop_store, spc_cst, 0);
            self.tm.op_set_input(rop_store, ptr_vn, 1);
            self.tm.op_set_input(rop_store, lane_at(in_vars, i), 2);
            byte_pos += self.description.get_size(skip_lanes + i) as int8;
        }
        true
    }

    /// Split a CPUI_LOAD into a sequence of per-lane LOADs (C++
    /// `LaneDivide::buildLoad`, subflow.cc:3768).
    fn build_load(
        &mut self,
        data: &mut Funcdata,
        op: OpId,
        out_vars: TVarRef,
        num_lanes: int4,
        skip_lanes: int4,
    ) -> bool {
        let in0 = data.obank().get(op).expect("build_load: stale op").get_in(0).expect("in0");
        let space_const = data.vbank().get(in0).expect("build_load: stale in0").get_offset();
        let space_const_size = data.vbank().get(in0).expect("build_load: stale in0").get_size();
        let spc = space_from_const(data, in0); // Address space being loaded from
        let orig_ptr = data.obank().get(op).expect("build_load: stale op").get_in(1).expect("in1");
        let orig_ptr_v = data.vbank().get(orig_ptr).expect("build_load: stale origPtr");
        if orig_ptr_v.is_free() && !orig_ptr_v.is_constant() {
            return false;
        }
        let base_ptr = self.tm.get_preexisting_varnode(data, orig_ptr);
        let ptr_size = data.vbank().get(orig_ptr).expect("build_load: stale origPtr").get_size();
        let mut byte_pos: int8 = 0;
        let is_big = spc.is_big_endian();
        for c in 0..num_lanes {
            let rop_load = self.tm.new_op_replace(2, OpCode::CPUI_LOAD, op);
            let i = if is_big { num_lanes - 1 - c } else { c };
            // Construct the pointer
            let ptr_vn = if byte_pos == 0 {
                base_ptr
            } else {
                let ptr_vn = self.tm.new_unique(ptr_size);
                let add_op = self.tm.new_op(2, OpCode::CPUI_INT_ADD, rop_load);
                self.tm.op_set_output(add_op, ptr_vn);
                self.tm.op_set_input(add_op, base_ptr, 0);
                let cst = self.tm.new_constant(ptr_size, 0, byte_pos as uintb);
                self.tm.op_set_input(add_op, cst, 1);
                ptr_vn
            };
            let spc_cst = self.tm.new_constant(space_const_size, 0, space_const);
            self.tm.op_set_input(rop_load, spc_cst, 0);
            self.tm.op_set_input(rop_load, ptr_vn, 1);
            self.tm.op_set_output(rop_load, lane_at(out_vars, i));
            byte_pos += self.description.get_size(skip_lanes + i) as int8;
        }
        true
    }

    /// Model a CPUI_INT_RIGHT that respects the lanes as COPYs (C++
    /// `LaneDivide::buildRightShift`, subflow.cc:3815).
    fn build_right_shift(
        &mut self,
        data: &mut Funcdata,
        op: OpId,
        out_vars: TVarRef,
        num_lanes: int4,
        skip_lanes: int4,
    ) -> bool {
        let in1 = data.obank().get(op).expect("build_right_shift: stale op").get_in(1).expect("in1");
        if !data.vbank().get(in1).expect("build_right_shift: stale in1").is_constant() {
            return false;
        }
        let mut shift_size = data.vbank().get(in1).expect("build_right_shift: stale in1").get_offset() as int4;
        if (shift_size & 7) != 0 {
            return false; // Not a multiple of 8
        }
        shift_size /= 8;
        let start_pos = shift_size + self.description.get_position(skip_lanes);
        let start_lane = self.description.get_boundary(start_pos);
        if start_lane < 0 {
            return false; // Shift does not end on a lane boundary
        }
        let mut src_lane = start_lane;
        let mut dest_lane = skip_lanes;
        while src_lane - skip_lanes < num_lanes {
            if self.description.get_size(src_lane) != self.description.get_size(dest_lane) {
                return false;
            }
            src_lane += 1;
            dest_lane += 1;
        }
        let in0 = data.obank().get(op).expect("build_right_shift: stale op").get_in(0).expect("in0");
        let in_vars = match self.set_replacement(data, in0, num_lanes, skip_lanes) {
            Some(v) => v,
            None => return false,
        };
        self.build_unary_op(
            OpCode::CPUI_COPY,
            op,
            lane_at(in_vars, start_lane - skip_lanes),
            out_vars,
            num_lanes - (start_lane - skip_lanes),
        );
        for zero_lane in (num_lanes - (start_lane - skip_lanes))..num_lanes {
            let rop = self.tm.new_op_replace(1, OpCode::CPUI_COPY, op);
            self.tm.op_set_output(rop, lane_at(out_vars, zero_lane));
            let cst = self.tm.new_constant(self.description.get_size(zero_lane), 0, 0);
            self.tm.op_set_input(rop, cst, 0);
        }
        true
    }

    /// Model a CPUI_INT_LEFT that respects the lanes as COPYs (C++
    /// `LaneDivide::buildLeftShift`, subflow.cc:3852).
    fn build_left_shift(
        &mut self,
        data: &mut Funcdata,
        op: OpId,
        out_vars: TVarRef,
        num_lanes: int4,
        skip_lanes: int4,
    ) -> bool {
        let in1 = data.obank().get(op).expect("build_left_shift: stale op").get_in(1).expect("in1");
        if !data.vbank().get(in1).expect("build_left_shift: stale in1").is_constant() {
            return false;
        }
        let mut shift_size = data.vbank().get(in1).expect("build_left_shift: stale in1").get_offset() as int4;
        if (shift_size & 7) != 0 {
            return false; // Not a multiple of 8
        }
        shift_size /= 8;
        let start_pos = shift_size + self.description.get_position(skip_lanes);
        let start_lane = self.description.get_boundary(start_pos);
        if start_lane < 0 {
            return false; // Shift does not end on a lane boundary
        }
        let mut dest_lane = start_lane;
        let mut src_lane = skip_lanes;
        while dest_lane - skip_lanes < num_lanes {
            if self.description.get_size(src_lane) != self.description.get_size(dest_lane) {
                return false;
            }
            src_lane += 1;
            dest_lane += 1;
        }
        let in0 = data.obank().get(op).expect("build_left_shift: stale op").get_in(0).expect("in0");
        let in_vars = match self.set_replacement(data, in0, num_lanes, skip_lanes) {
            Some(v) => v,
            None => return false,
        };
        for zero_lane in 0..(start_lane - skip_lanes) {
            let rop = self.tm.new_op_replace(1, OpCode::CPUI_COPY, op);
            self.tm.op_set_output(rop, lane_at(out_vars, zero_lane));
            let cst = self.tm.new_constant(self.description.get_size(zero_lane), 0, 0);
            self.tm.op_set_input(rop, cst, 0);
        }
        self.build_unary_op(
            OpCode::CPUI_COPY,
            op,
            in_vars,
            lane_at(out_vars, start_lane - skip_lanes),
            num_lanes - (start_lane - skip_lanes),
        );
        true
    }

    /// Split a CPUI_INT_ZEXT into COPYs of lanes plus COPYs of zero (C++
    /// `LaneDivide::buildZext`, subflow.cc:3890).
    fn build_zext(
        &mut self,
        data: &mut Funcdata,
        op: OpId,
        out_vars: TVarRef,
        num_lanes: int4,
        skip_lanes: int4,
    ) -> bool {
        let invn = data.obank().get(op).expect("build_zext: stale op").get_in(0).expect("in0");
        let in_size = data.vbank().get(invn).expect("build_zext: stale invn").get_size();
        let (in_lanes, in_skip) =
            match self.description.restriction(num_lanes, skip_lanes, 0, in_size) {
                Some(v) => v,
                None => return false,
            };
        // inSkip should always come back as equal to skipLanes
        if in_lanes == 1 {
            let rop = self.tm.new_op_replace(1, OpCode::CPUI_COPY, op);
            let in_var = self.tm.get_preexisting_varnode(data, invn);
            self.tm.op_set_input(rop, in_var, 0);
            self.tm.op_set_output(rop, out_vars);
        } else {
            let in_rvn = match self.set_replacement(data, invn, in_lanes, in_skip) {
                Some(v) => v,
                None => return false,
            };
            for i in 0..in_lanes {
                let rop = self.tm.new_op_replace(1, OpCode::CPUI_COPY, op);
                self.tm.op_set_input(rop, lane_at(in_rvn, i), 0);
                self.tm.op_set_output(rop, lane_at(out_vars, i));
            }
        }
        for i in 0..(num_lanes - in_lanes) {
            // Write 0 constants to remaining lanes
            let rop = self.tm.new_op_replace(1, OpCode::CPUI_COPY, op);
            let cst = self.tm.new_constant(self.description.get_size(skip_lanes + in_lanes + i), 0, 0);
            self.tm.op_set_input(rop, cst, 0);
            self.tm.op_set_output(rop, lane_at(out_vars, in_lanes + i));
        }
        true
    }

    /// Push the logical lanes forward through any reading op (C++
    /// `LaneDivide::traceForward`, subflow.cc:3931).
    fn trace_forward(
        &mut self,
        data: &mut Funcdata,
        rvn: TVarRef,
        num_lanes: int4,
        skip_lanes: int4,
    ) -> bool {
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
                OpCode::CPUI_SUBPIECE => {
                    let ov = outvn.expect("trace_forward: SUBPIECE out");
                    let in1 = data.obank().get(op).expect("trace_forward: stale op").get_in(1).expect("in1");
                    let byte_pos = data.vbank().get(in1).expect("trace_forward: stale in1").get_offset() as int4;
                    let out_size = data.vbank().get(ov).expect("trace_forward: stale out").get_size();
                    match self.description.restriction(num_lanes, skip_lanes, byte_pos, out_size) {
                        Some((out_lanes, out_skip)) => {
                            if out_lanes == 1 {
                                let rop = self.tm.new_preexisting_op(1, OpCode::CPUI_COPY, op);
                                self.tm.op_set_input(rop, lane_at(rvn, out_skip - skip_lanes), 0);
                            } else {
                                match self.set_replacement(data, ov, out_lanes, out_skip) {
                                    Some(_) => {}
                                    None => return false,
                                }
                                // Don't create the placeholder ops, let traceBackward make them
                            }
                        }
                        None => {
                            if self.allow_subpiece_terminator {
                                let lane_index = self.description.get_boundary(byte_pos);
                                if lane_index < 0 || lane_index >= self.description.get_num_lanes() {
                                    return false; // Does piece start on lane boundary?
                                }
                                if self.description.get_size(lane_index) <= out_size {
                                    return false; // Is the piece smaller than a lane?
                                }
                                // Treat SUBPIECE as terminating
                                let rop = self.tm.new_preexisting_op(2, OpCode::CPUI_SUBPIECE, op);
                                self.tm.op_set_input(rop, lane_at(rvn, lane_index - skip_lanes), 0);
                                let cst = self.tm.new_constant(4, 0, 0);
                                self.tm.op_set_input(rop, cst, 1);
                            } else {
                                return false;
                            }
                        }
                    }
                }
                OpCode::CPUI_PIECE => {
                    let ov = outvn.expect("trace_forward: PIECE out");
                    let in0 = data.obank().get(op).expect("trace_forward: stale op").get_in(0).expect("in0");
                    let byte_pos = if in0 == origvn {
                        let in1 = data.obank().get(op).expect("trace_forward: stale op").get_in(1).expect("in1");
                        data.vbank().get(in1).expect("trace_forward: stale in1").get_size()
                    } else {
                        0
                    };
                    let out_size = data.vbank().get(ov).expect("trace_forward: stale out").get_size();
                    match self.description.extension(num_lanes, skip_lanes, byte_pos, out_size) {
                        Some((out_lanes, out_skip)) => {
                            match self.set_replacement(data, ov, out_lanes, out_skip) {
                                Some(_) => {}
                                None => return false,
                            }
                            // Don't create the placeholder ops, let traceBackward make them
                        }
                        None => return false,
                    }
                }
                OpCode::CPUI_COPY
                | OpCode::CPUI_INT_NEGATE
                | OpCode::CPUI_INT_AND
                | OpCode::CPUI_INT_OR
                | OpCode::CPUI_INT_XOR
                | OpCode::CPUI_MULTIEQUAL
                | OpCode::CPUI_INDIRECT => {
                    let ov = outvn.expect("trace_forward: out");
                    match self.set_replacement(data, ov, num_lanes, skip_lanes) {
                        Some(_) => {}
                        None => return false,
                    }
                    // Don't create the placeholder ops, let traceBackward make them
                }
                OpCode::CPUI_INT_RIGHT => {
                    let in1 = data.obank().get(op).expect("trace_forward: stale op").get_in(1).expect("in1");
                    if !data.vbank().get(in1).expect("trace_forward: stale in1").is_constant() {
                        return false; // Trace must come through op->getIn(0)
                    }
                    let ov = outvn.expect("trace_forward: INT_RIGHT out");
                    match self.set_replacement(data, ov, num_lanes, skip_lanes) {
                        Some(_) => {}
                        None => return false,
                    }
                    // Don't create the placeholder ops, let traceBackward make them
                }
                OpCode::CPUI_STORE => {
                    let in2 = data.obank().get(op).expect("trace_forward: stale op").get_in(2).expect("in2");
                    if in2 != origvn {
                        return false; // Can only propagate through value being stored
                    }
                    if !self.build_store(data, op, num_lanes, skip_lanes) {
                        return false;
                    }
                }
                _ => return false,
            }
        }
        true
    }

    /// Pull the logical lanes back through the defining op (C++
    /// `LaneDivide::traceBackward`, subflow.cc:4027).
    fn trace_backward(
        &mut self,
        data: &mut Funcdata,
        rvn: TVarRef,
        num_lanes: int4,
        skip_lanes: int4,
    ) -> bool {
        let origvn = self.tm.var(rvn).get_original().expect("trace_backward: rvn original");
        let op = match data.vbank().get(origvn).expect("trace_backward: stale origvn").get_def() {
            Some(o) => o,
            None => return true, // If vn is input
        };
        let code = data.obank().get(op).expect("trace_backward: stale op").code();
        match code {
            OpCode::CPUI_INT_NEGATE | OpCode::CPUI_COPY => {
                let in0 = data.obank().get(op).expect("trace_backward: stale op").get_in(0).expect("in0");
                let in_vars = match self.set_replacement(data, in0, num_lanes, skip_lanes) {
                    Some(v) => v,
                    None => return false,
                };
                self.build_unary_op(code, op, in_vars, rvn, num_lanes);
            }
            OpCode::CPUI_INT_AND | OpCode::CPUI_INT_OR | OpCode::CPUI_INT_XOR => {
                let in0 = data.obank().get(op).expect("trace_backward: stale op").get_in(0).expect("in0");
                let in0_vars = match self.set_replacement(data, in0, num_lanes, skip_lanes) {
                    Some(v) => v,
                    None => return false,
                };
                let in1 = data.obank().get(op).expect("trace_backward: stale op").get_in(1).expect("in1");
                let in1_vars = match self.set_replacement(data, in1, num_lanes, skip_lanes) {
                    Some(v) => v,
                    None => return false,
                };
                self.build_binary_op(code, op, in0_vars, in1_vars, rvn, num_lanes);
            }
            OpCode::CPUI_MULTIEQUAL => {
                if !self.build_multiequal(data, op, rvn, num_lanes, skip_lanes) {
                    return false;
                }
            }
            OpCode::CPUI_INDIRECT => {
                if !self.build_indirect(data, op, rvn, num_lanes, skip_lanes) {
                    return false;
                }
            }
            OpCode::CPUI_SUBPIECE => {
                let in_vn = data.obank().get(op).expect("trace_backward: stale op").get_in(0).expect("in0");
                let in1 = data.obank().get(op).expect("trace_backward: stale op").get_in(1).expect("in1");
                let byte_pos = data.vbank().get(in1).expect("trace_backward: stale in1").get_offset() as int4;
                let in_size = data.vbank().get(in_vn).expect("trace_backward: stale inVn").get_size();
                let (in_lanes, in_skip) =
                    match self.description.extension(num_lanes, skip_lanes, byte_pos, in_size) {
                        Some(v) => v,
                        None => return false,
                    };
                let in_vars = match self.set_replacement(data, in_vn, in_lanes, in_skip) {
                    Some(v) => v,
                    None => return false,
                };
                self.build_unary_op(
                    OpCode::CPUI_COPY,
                    op,
                    lane_at(in_vars, skip_lanes - in_skip),
                    rvn,
                    num_lanes,
                );
            }
            OpCode::CPUI_PIECE => {
                if !self.build_piece(data, op, rvn, num_lanes, skip_lanes) {
                    return false;
                }
            }
            OpCode::CPUI_LOAD => {
                if !self.build_load(data, op, rvn, num_lanes, skip_lanes) {
                    return false;
                }
            }
            OpCode::CPUI_INT_RIGHT => {
                if !self.build_right_shift(data, op, rvn, num_lanes, skip_lanes) {
                    return false;
                }
            }
            OpCode::CPUI_INT_LEFT => {
                if !self.build_left_shift(data, op, rvn, num_lanes, skip_lanes) {
                    return false;
                }
            }
            OpCode::CPUI_INT_ZEXT => {
                if !self.build_zext(data, op, rvn, num_lanes, skip_lanes) {
                    return false;
                }
            }
            _ => return false,
        }
        true
    }

    /// Process the next Varnode on the work list (C++
    /// `LaneDivide::processNextWork`, subflow.cc:4100).
    fn process_next_work(&mut self, data: &mut Funcdata) -> bool {
        let node = self.work_list.pop().expect("process_next_work: empty worklist");
        let rvn = node.lanes;
        let num_lanes = node.num_lanes;
        let skip_lanes = node.skip_lanes;

        if !self.trace_backward(data, rvn, num_lanes, skip_lanes) {
            return false;
        }
        self.trace_forward(data, rvn, num_lanes, skip_lanes)
    }

    /// Trace lanes as far as possible from the root Varnode (C++
    /// `LaneDivide::doTrace`, subflow.cc:4127).
    pub fn do_trace(&mut self, data: &mut Funcdata) -> bool {
        if self.work_list.is_empty() {
            return false; // Nothing to do
        }
        let mut retval = true;
        while !self.work_list.is_empty() {
            if !self.process_next_work(data) {
                retval = false;
                break;
            }
        }
        self.tm.clear_varnode_marks(data);
        retval
    }

    /// Apply the constructed transform (C++ base `TransformManager::apply`).
    /// STUB(W6): the merged apply reaches `glb->inst[opc]`.
    pub fn apply(&mut self, data: &mut Funcdata) -> KunaResult<()> {
        self.tm.apply(data)
    }
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
                // C++ apply() returns void; a structured error -> "no change".
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
    use crate::context::{ArchContext, TypeOp};
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
        let glb = Rc::new(ArchContext::new(manage));
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
    fn split_load_store_get_value_datatype_is_stub_none() {
        // getValueDatatype needs getTypeReadFacing off the arch stub -> None, so
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
    fn rule_split_copy_is_stub_gated() {
        // RuleSplitCopy's getTypeReadFacing/getTypeDefFacing are arch-stubbed, so
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

    // ---- SplitDatatype split build path (subflow.cc:2390-2954) -------------
    // item: rport/w10-splitdatatype (round 1).  Verifier adversarial tests
    // targeting the fragile transcriptions: the testCopyConstraints boolean
    // ladder, the generateConstants extended-precision shift arithmetic, and the
    // getValueDatatype TYPE_PTR guard.

    /// `SplitDatatype::testCopyConstraints` (subflow.cc:2390): the C++ rejects
    /// COPYs whose input is a function input, and the written-LOAD-lone-descend
    /// form (handled by splitCopy itself).  Verifies both early-false arms and the
    /// true fallthrough for a plain written non-LOAD input.
    #[test]
    fn w10_splitdt_test_copy_constraints_ladder() {
        // (a) input Varnode -> false (inVn->isInput()).
        let mut fd = build_fd();
        let sd = SplitDatatype::new(3);
        let src = mk_input(&mut fd, 0x40, 8);
        let copyop = mk_op(&mut fd, 0x100, 1, OpCode::CPUI_COPY);
        let outvn = mk_reg(&mut fd, 0x80, 8);
        wire_in(&mut fd, copyop, src, 0);
        let _ = wire_out(&mut fd, copyop, outvn);
        assert!(!sd.test_copy_constraints(&fd, copyop), "input must be rejected");

        // (b) written non-input, non-LOAD def -> the LOAD-lone-descend arm is
        // skipped and the method returns true (the splittable case).
        let mut fd = build_fd();
        let sd = SplitDatatype::new(3);
        let predop = mk_op(&mut fd, 0x80, 1, OpCode::CPUI_INT_2COMP);
        let inarg = mk_input(&mut fd, 0x40, 8);
        wire_in(&mut fd, predop, inarg, 0);
        let mid = mk_reg(&mut fd, 0x48, 8);
        let mid = wire_out(&mut fd, predop, mid); // written, def is not a LOAD
        let copyop = mk_op(&mut fd, 0x100, 1, OpCode::CPUI_COPY);
        let outvn = mk_reg(&mut fd, 0x88, 8);
        wire_in(&mut fd, copyop, mid, 0);
        let _ = wire_out(&mut fd, copyop, outvn);
        assert!(sd.test_copy_constraints(&fd, copyop), "written non-LOAD input is splittable");

        // (c) written LOAD def whose lone descendant is the COPY -> false
        // (handled by splitCopy()).  Exercises inVn->getDef()->code()==LOAD and
        // loneDescend()==copyOp.
        let mut fd = build_fd();
        let sd = SplitDatatype::new(3);
        let spaceid = mk_const(&mut fd, 8, 0);
        let ptr = mk_input(&mut fd, 0x40, 8);
        let loadop = mk_op(&mut fd, 0x90, 2, OpCode::CPUI_LOAD);
        wire_in(&mut fd, loadop, spaceid, 0);
        wire_in(&mut fd, loadop, ptr, 1);
        let loaded = mk_reg(&mut fd, 0x50, 8);
        let loaded = wire_out(&mut fd, loadop, loaded);
        let copyop = mk_op(&mut fd, 0x100, 1, OpCode::CPUI_COPY);
        let outvn = mk_reg(&mut fd, 0x88, 8);
        wire_in(&mut fd, copyop, loaded, 0); // loaded's lone descend is copyOp
        let _ = wire_out(&mut fd, copyop, outvn);
        assert!(
            !sd.test_copy_constraints(&fd, copyop),
            "LOAD-into-lone-COPY must be deferred to splitCopy()"
        );
    }

    /// `SplitDatatype::generateConstants` (subflow.cc:2413): a little-endian
    /// `INT_ZEXT(c)` is split into the precomputed `dataTypePieces`.  Verifies the
    /// exact shift arithmetic (`val = lo >> sa*8` with `sa = offset`) and the
    /// `dt->getSize() > sizeof(uintb)` overflow bail that clears the accumulator.
    #[test]
    fn w10_splitdt_generate_constants_zext_and_oversize_bail() {
        let mut fd = build_fd();
        let mut sd = SplitDatatype::new(3);
        // Build a little-endian ZEXT(0x11223344 : 4) -> 8.  loneDescend must be
        // present (a sink op) and the value must be a constant input.
        let cst = mk_const(&mut fd, 4, 0x1122_3344);
        let zext = mk_op(&mut fd, 0x80, 1, OpCode::CPUI_INT_ZEXT);
        wire_in(&mut fd, zext, cst, 0);
        let zout = mk_reg(&mut fd, 0x50, 8);
        let zout = wire_out(&mut fd, zext, zout);
        // A single descendant so loneDescend() != null.
        let sink = mk_op(&mut fd, 0x100, 1, OpCode::CPUI_COPY);
        wire_in(&mut fd, sink, zout, 0);
        let sinkout = mk_reg(&mut fd, 0x88, 8);
        let _ = wire_out(&mut fd, sink, sinkout);

        // Two int2 pieces at offsets 0 and 2 (low half of the ZEXT value).
        let int2 = Rc::new(Datatype::new(2, type_metatype::TYPE_INT));
        sd.data_type_pieces.push(Component { in_type: Rc::clone(&int2), out_type: Rc::clone(&int2), offset: 0 });
        sd.data_type_pieces.push(Component { in_type: Rc::clone(&int2), out_type: Rc::clone(&int2), offset: 2 });
        let mut inv: Vec<VarnodeId> = Vec::new();
        assert!(sd.generate_constants(&mut fd, zout, &mut inv).unwrap());
        assert_eq!(inv.len(), 2);
        // ram space is little-endian here: offset 0 -> low 16 bits (0x3344),
        // offset 2 -> next 16 bits (0x1122).  losize=4, sa<losize so val=lo>>sa*8.
        assert_eq!(fd.vbank().get(inv[0]).unwrap().get_offset(), 0x3344);
        assert_eq!(fd.vbank().get(inv[1]).unwrap().get_offset(), 0x1122);

        // Oversize bail: a piece larger than sizeof(uintb) clears inVarnodes and
        // returns false (the C++ `dt->getSize() > sizeof(uintb)` guard).
        let mut fd = build_fd();
        let mut sd = SplitDatatype::new(3);
        let cst = mk_const(&mut fd, 4, 0xdead_beef);
        let zext = mk_op(&mut fd, 0x80, 1, OpCode::CPUI_INT_ZEXT);
        wire_in(&mut fd, zext, cst, 0);
        let zout = mk_reg(&mut fd, 0x50, 16);
        let zout = wire_out(&mut fd, zext, zout);
        let sink = mk_op(&mut fd, 0x100, 1, OpCode::CPUI_COPY);
        wire_in(&mut fd, sink, zout, 0);
        let sinkout = mk_reg(&mut fd, 0x90, 16);
        let _ = wire_out(&mut fd, sink, sinkout);
        let big = Rc::new(Datatype::new(16, type_metatype::TYPE_INT)); // > sizeof(uintb)
        sd.data_type_pieces.push(Component { in_type: Rc::clone(&big), out_type: Rc::clone(&big), offset: 0 });
        let mut inv: Vec<VarnodeId> = Vec::new();
        assert!(!sd.generate_constants(&mut fd, zout, &mut inv).unwrap());
        assert!(inv.is_empty(), "oversize piece must clear the accumulator");
    }

    /// `SplitDatatype::getValueDatatype` (subflow.cc:2925): a non-pointer
    /// read-facing type returns None (the TYPE_PTR guard), regardless of size.
    /// This is the guard `RuleSplitLoad`/`RuleSplitStore` lean on to early-out.
    #[test]
    fn w10_splitdt_get_value_datatype_rejects_non_pointer() {
        let mut fd = build_fd();
        let spaceid = mk_const(&mut fd, 8, 0);
        // ptr Varnode carries a plain int8 (NOT a pointer) read-facing type.
        let ptr = mk_input(&mut fd, 0x40, 8);
        fd.vbank_mut()
            .get_mut(ptr)
            .unwrap()
            .update_type(Rc::new(Datatype::new(8, type_metatype::TYPE_INT)));
        let load = mk_op(&mut fd, 0x100, 2, OpCode::CPUI_LOAD);
        wire_in(&mut fd, load, spaceid, 0);
        wire_in(&mut fd, load, ptr, 1);
        let loadout = mk_reg(&mut fd, 0x80, 8);
        let _ = wire_out(&mut fd, load, loadout);
        assert!(
            SplitDatatype::get_value_datatype(&fd, load, 8).is_none(),
            "non-pointer read-facing type must yield None"
        );
    }

    // ---- SubfloatFlow / RuleSubfloatConvert (subflow.cc:3085-3522) --------

    #[test]
    fn subfloat_flow_no_format_does_not_trace() {
        // The Funcdata arch stub exposes no Translate/FloatFormat, so SubfloatFlow
        // reports no format and doTrace returns false (the C++ null-format path).
        let mut fd = build_fd();
        let root = mk_input(&mut fd, 0x40, 8);
        let mut sf = SubfloatFlow::new(&mut fd, root, 4);
        assert!(!sf.has_format);
        assert!(!sf.do_trace(&mut fd).unwrap());
    }

    #[test]
    fn rule_subfloat_convert_returns_zero_under_stub() {
        // FLOAT_FLOAT2FLOAT widening (out>in): SubfloatFlow can't acquire a format
        // (arch stub) so doTrace is false and the rule returns 0.
        let mut fd = build_fd();
        let invn = mk_input(&mut fd, 0x40, 4);
        let f2f = mk_op(&mut fd, 0x100, 1, OpCode::CPUI_FLOAT_FLOAT2FLOAT);
        let outvn = mk_reg(&mut fd, 0x80, 8);
        wire_in(&mut fd, f2f, invn, 0);
        let _outvn = wire_out(&mut fd, f2f, outvn);
        let mut rule = RuleSubfloatConvert::new("g");
        assert_eq!(rule.apply_op(f2f, &mut fd), 0);
    }

    // ---- tryReturnPull (subflow.cc:238-284) — verifier adversarial tests ----
    // item: rport/w10-return-narrow (round 1).  These target the three fragile
    // decision branches of the now-closed stub: the slot==0 return-address bail,
    // the non-aggressive consume-outside-mask bail, and the terminal trim patch
    // (which sets returnsTraversed and emits exactly one parameter_patch).

    /// Build an engine + a single RETURN op consuming `root` (4-byte reg) at the
    /// given input slot.  Returns (sf, returnop, root_rvid).  The engine is
    /// seeded on the low-byte logical value (mask 0xff).
    fn return_setup(slot: int4, consume: uintb) -> (Funcdata, OpId, VarnodeId) {
        let mut fd = build_fd();
        let root = mk_input(&mut fd, 0x40, 4);
        fd.vbank_mut().get_mut(root).unwrap().set_consume(consume);
        // RETURN(retaddr, root) — slot 0 is the return-address container, slot 1
        // the first return value.  We wire `root` at the requested slot.
        let retop = mk_op(&mut fd, 0x200, slot + 1, OpCode::CPUI_RETURN);
        let retaddr = fd.new_constant(8, 0);
        wire_in(&mut fd, retop, retaddr, 0);
        if slot != 0 {
            wire_in(&mut fd, retop, root, slot);
        }
        (fd, retop, root)
    }

    #[test]
    fn try_return_pull_slot0_is_return_address_bail() {
        // subflow.cc:241 — slot==0 is the actual return-address container; never
        // trimmed.  Must return Ok(false) WITHOUT touching patchlist/pullcount or
        // flipping returns_traversed.
        let (mut fd, retop, root) = return_setup(0, 0xff);
        let mut sf = SubvariableFlow::new(&mut fd, root, 0xff, true, false, false).unwrap();
        let rvn = *sf.varmap.get(&root).expect("root must be linked");
        let before = sf.pullcount;
        let ok = sf.try_return_pull(&mut fd, retop, rvn, 0).unwrap();
        assert!(!ok, "slot 0 must bail (return-address container)");
        assert_eq!(sf.pullcount, before, "slot-0 bail must not bump pullcount");
        assert!(sf.patchlist.is_empty(), "slot-0 bail must not patch");
        assert!(!sf.returns_traversed, "slot-0 bail must not mark returns traversed");
    }

    #[test]
    fn try_return_pull_nonaggressive_consume_outside_mask_bails() {
        // subflow.cc:243-246 — non-aggressive: if the varnode consumes any bit
        // outside the logical mask, do not truncate.  Must bail BEFORE the
        // returns-traversed propagation.  Seed with consume==mask (so the
        // non-aggressive constructor links the root), then widen consume to
        // 0xffff to trip the gate exactly at try_return_pull.
        let (mut fd, retop, root) = return_setup(1, 0xff);
        let mut sf = SubvariableFlow::new(&mut fd, root, 0xff, false, false, false).unwrap();
        let rvn = *sf.varmap.get(&root).expect("root must be linked");
        fd.vbank_mut().get_mut(root).unwrap().set_consume(0xffff);
        let ok = sf.try_return_pull(&mut fd, retop, rvn, 1).unwrap();
        assert!(!ok, "consume outside mask (non-aggressive) must bail");
        assert!(sf.patchlist.is_empty(), "consume-bail must not patch");
        assert!(!sf.returns_traversed, "consume-bail must not mark returns traversed");
        // Sanity: the SAME setup with aggressive=true does NOT bail on consume
        // (the consume gate is skipped) and reaches the terminal trim.
        let (mut fd2, retop2, root2) = return_setup(1, 0xffff);
        let mut sf2 = SubvariableFlow::new(&mut fd2, root2, 0xff, true, false, false).unwrap();
        let rvn2 = *sf2.varmap.get(&root2).expect("root must be linked");
        let ok2 = sf2.try_return_pull(&mut fd2, retop2, rvn2, 1).unwrap();
        assert!(ok2, "aggressive engine skips the consume gate and trims");
    }

    #[test]
    fn try_return_pull_terminal_trim_emits_one_patch_and_sets_traversed() {
        // subflow.cc:248-283 — the happy path: returns_traversed starts false, the
        // single RETURN is traversed, the terminal parameter_patch on `op` is
        // pushed, pullcount bumps by exactly one, and returns_traversed flips true.
        // A SECOND call must NOT re-run the propagation loop (idempotent flag) but
        // still emits its own terminal patch (C++ pushes the terminal each call).
        let (mut fd, retop, root) = return_setup(1, 0xff);
        let mut sf = SubvariableFlow::new(&mut fd, root, 0xff, true, false, false).unwrap();
        let rvn = *sf.varmap.get(&root).expect("root must be linked");
        assert!(!sf.returns_traversed);
        let before = sf.pullcount;
        let ok = sf.try_return_pull(&mut fd, retop, rvn, 1).unwrap();
        assert!(ok, "valid trim must succeed");
        assert!(sf.returns_traversed, "first trim sets returns_traversed");
        assert_eq!(sf.pullcount, before + 1, "exactly one terminal patch bump");
        // The terminal patch is a parameter_patch on the RETURN op at slot 1.
        let term = sf.patchlist.last().expect("a terminal patch exists");
        assert_eq!(term.typ, PatchType::ParameterPatch);
        assert_eq!(term.patch_op, retop);
        assert_eq!(term.slot, 1);
        assert_eq!(term.in1, rvn);
        // Second call: returns_traversed already true, so NO new propagation, but
        // the terminal patch still fires (mirrors C++ emitting the terminal each
        // time tryReturnPull is reached for a RETURN).
        let mid = sf.pullcount;
        let patches_mid = sf.patchlist.len();
        let ok2 = sf.try_return_pull(&mut fd, retop, rvn, 1).unwrap();
        assert!(ok2);
        assert_eq!(sf.pullcount, mid + 1, "second call adds exactly one terminal");
        assert_eq!(
            sf.patchlist.len(),
            patches_mid + 1,
            "second call must not re-traverse RETURNs, only add its terminal"
        );
    }
}
