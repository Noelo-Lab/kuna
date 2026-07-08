//! Port of `decompiler/cpp/paramid.{cc,hh}` — the standalone parameter-ID
//! pipeline variant (`ParamMeasure` / `ParamIDAnalysis`, item `w8-sig-pack`).
//!
//! `ParamIDAnalysis` backs the `paramid` pipeline variant: it ranks how strongly
//! each candidate input/output storage location behaves like a real parameter or
//! return by walking the data-flow graph forward (inputs) or backward (outputs)
//! from a base Varnode and folding the encountered op kinds into a [`ParamRank`].
//!
//! # Faithfulness
//!
//! The walk is transcribed op-for-op from `paramid.cc` (the
//! `BRANCH`/`CBRANCH`/`CALL`/`CALLOTHER`/`RETURN`/`INDIRECT`/`MULTIEQUAL`/default
//! dispatch and the `MAXDEPTH`/terminal-rank cutoffs).  The IR is the W3 arena
//! model (ADR 0001): the C++ `Varnode *` / `PcodeOp *` become arena keys
//! dereferenced through `&Funcdata`.  Loop-edge avoidance on `MULTIEQUAL` uses the
//! op's parent block `isLoopIn` (the same edge the C++ reads through
//! `op->getParent()->isLoopIn(slot)`).
//!
//! # Stubs
//!
//! * The `justproto` constructor path needs the recovered [`FuncProto`] from
//!   `Funcdata::getFuncProto()` (`numParams`/`getParam`/`getOutput`).  In the
//!   merged tree `Funcdata::funcp` is still the W3/W4 placeholder
//!   (`crate::context::FuncProto`), not the real `fspec::FuncProto`, so the
//!   prototype-driven measure collection cannot run yet — [`ParamIDAnalysis::new`]
//!   returns a `STUB(W4)` error for `justproto = true`.  The non-prototype path
//!   (input Varnodes outside the model, via `beginDef(input)`) is fully ported.
//! * `ParamMeasure::encode` needs `Datatype::encodeRef` and the W8 marshal of the
//!   `<rank>`/`<addr>` children; the address-space attribute encode exists, the
//!   type-ref does not, so [`ParamMeasure::encode`] stubs out.  `savePretty` is fully
//!   ported (it prints only the `VarnodeData` fields).

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::marshal::{AttributeId, ElementId};
use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;

use crate::dtype::Datatype;
use crate::funcdata::Funcdata;
use crate::context::{OpId, VarnodeId};
use crate::varnode::varnode_flags;

// ===========================================================================
// Marshaling identifiers (paramid.cc:20-22)
// ===========================================================================

/// Marshaling element `<parammeasures>` (paramid.cc:20).
pub const ELEM_PARAMMEASURES: ElementId = ElementId::new("parammeasures", 106);
/// Marshaling element `<proto>` (paramid.cc:21).
pub const ELEM_PROTO: ElementId = ElementId::new("proto", 107);
/// Marshaling element `<rank>` (paramid.cc:22).
pub const ELEM_RANK: ElementId = ElementId::new("rank", 108);

// ===========================================================================
// ParamMeasure enums (paramid.hh:27-51)
// ===========================================================================

/// Whether a measure tracks an input or an output location (C++
/// `ParamMeasure::ParamIDIO`, paramid.hh:29-32).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParamIDIO {
    /// An input location (`INPUT = 0`).
    Input,
    /// An output location (`OUTPUT = 1`).
    Output,
}

/// The rank of a parameter/return measure (C++ `ParamMeasure::ParamRank`,
/// paramid.hh:33-46).
///
/// The C++ enum gives several names the same integer value (`BESTRANK` ==
/// `DIRECTWRITEWITHOUTREAD` == 1, `DIRECTREAD` == `DIRECTWRITEWITHREAD` == 2,
/// etc.); they are kept as associated constants below so `updaterank`'s
/// `min`/`max` arithmetic and the terminal-rank comparisons transcribe exactly.
/// The measure stores the rank as the bare integer.
pub mod param_rank {
    /// Best possible rank (`BESTRANK = 1`).
    pub const BESTRANK: i32 = 1;
    /// Output: direct write without a subsequent read (`DIRECTWRITEWITHOUTREAD = 1`).
    pub const DIRECTWRITEWITHOUTREAD: i32 = 1;
    /// Input: a direct read (`DIRECTREAD = 2`).  Must equal `DIRECTWRITEWITHREAD`
    /// so a forward walk used inside a backward walk works.
    pub const DIRECTREAD: i32 = 2;
    /// Output: direct write with a read (`DIRECTWRITEWITHREAD = 2`).
    pub const DIRECTWRITEWITHREAD: i32 = 2;
    /// Output: direct write with an unknown read (`DIRECTWRITEUNKNOWNREAD = 3`).
    #[allow(dead_code)]
    pub const DIRECTWRITEUNKNOWNREAD: i32 = 3;
    /// Input: passed to a sub-function (`SUBFNPARAM = 4`).
    pub const SUBFNPARAM: i32 = 4;
    /// Output: this function's parameter (`THISFNPARAM = 4`).
    pub const THISFNPARAM: i32 = 4;
    /// Output: a sub-function's return (`SUBFNRETURN = 5`).
    pub const SUBFNRETURN: i32 = 5;
    /// Input: this function's return (`THISFNRETURN = 5`).
    pub const THISFNRETURN: i32 = 5;
    /// Input or output: through an INDIRECT (`INDIRECT = 6`).
    pub const INDIRECT: i32 = 6;
    /// Worst possible rank (`WORSTRANK = 7`).
    pub const WORSTRANK: i32 = 7;
}

/// Maximum data-flow walk depth (C++ `#define MAXDEPTH 10`, paramid.cc:36).
const MAXDEPTH: i32 = 10;

/// Per-walk transient state (C++ `ParamMeasure::WalkState`, paramid.hh:47-51).
struct WalkState {
    /// Whether we are seeking the best (min) or worst (max) rank.
    best: bool,
    /// Current recursion depth.
    depth: i32,
    /// The rank at which the walk short-circuits.
    terminalrank: i32,
}

// ===========================================================================
// ParamMeasure (paramid.hh:27-68, paramid.cc:37-184)
// ===========================================================================

/// A measure of how strongly a storage location behaves like a parameter or a
/// return value (C++ `ParamMeasure`, paramid.hh:27-68).
pub struct ParamMeasure {
    /// Storage location (space/offset/size) (C++ `vndata`).
    vndata: VarnodeData,
    /// Data-type of the location (`None` for the C++ null) (C++ `vntype`).
    vntype: Option<Rc<Datatype>>,
    /// The computed rank (C++ `rank`).
    rank: i32,
    /// Input or output (C++ `io`).
    io: ParamIDIO,
    /// Number of sub-function calls encountered (C++ `numcalls`).
    numcalls: i32,
}

impl ParamMeasure {
    /// Construct a measure for a given storage location (C++
    /// `ParamMeasure::ParamMeasure`, paramid.hh:62-63).
    pub fn new(addr: &Address, sz: i32, dt: Option<Rc<Datatype>>, io_in: ParamIDIO) -> ParamMeasure {
        ParamMeasure {
            vndata: VarnodeData {
                space: addr.get_space().cloned(),
                offset: addr.get_offset(),
                size: sz as u32,
            },
            vntype: dt,
            io: io_in,
            rank: param_rank::WORSTRANK,
            numcalls: 0,
        }
    }

    /// Get the computed measure (C++ `getMeasure`, paramid.hh:67).
    pub fn get_measure(&self) -> i32 {
        self.rank
    }

    /// Update the rank, taking the min (best) or max (worst) (C++
    /// `updaterank`, paramid.hh:60).
    fn updaterank(&mut self, rank_in: i32, best: bool) {
        self.rank = if best { self.rank.min(rank_in) } else { self.rank.max(rank_in) };
    }

    /// Calculate the rank by walking the data-flow from a base Varnode (C++
    /// `ParamMeasure::calculateRank`, paramid.cc:141-159).
    ///
    /// `best` selects min/max accumulation; `basevn` is the starting Varnode;
    /// `ignoreop` is an op to skip on the first hop (the op the caller arrived
    /// from).
    pub fn calculate_rank(
        &mut self,
        fd: &Funcdata,
        best: bool,
        basevn: VarnodeId,
        ignoreop: Option<OpId>,
    ) {
        let mut state = WalkState { best, depth: 0, terminalrank: 0 };
        if best {
            self.rank = param_rank::WORSTRANK;
            state.terminalrank = if self.io == ParamIDIO::Input {
                param_rank::DIRECTREAD
            } else {
                param_rank::DIRECTWRITEWITHOUTREAD
            };
        } else {
            self.rank = param_rank::BESTRANK;
            state.terminalrank = param_rank::INDIRECT;
        }
        self.numcalls = 0;
        if self.io == ParamIDIO::Input {
            self.walkforward(fd, &mut state, ignoreop, basevn);
        } else {
            self.walkbackward(fd, &mut state, ignoreop, basevn);
        }
    }

    /// Walk forward through the descendants of `vn`, folding op kinds into the
    /// rank (C++ `ParamMeasure::walkforward`, paramid.cc:37-88).
    fn walkforward(
        &mut self,
        fd: &Funcdata,
        state: &mut WalkState,
        ignoreop: Option<OpId>,
        vn: VarnodeId,
    ) {
        state.depth += 1;
        if state.depth >= MAXDEPTH {
            state.depth -= 1;
            return;
        }
        // Snapshot the descend list (C++ vn->beginDescend()..endDescend()).
        let descend = fd.descend_snapshot(vn);
        let mut iter = descend.iter();
        while self.rank != state.terminalrank {
            let op = match iter.next() {
                None => break,
                Some(&o) => o,
            };
            if Some(op) != ignoreop {
                let o = fd.obank().get(op).expect("walkforward: stale op");
                let oc = o.code();
                match oc {
                    OpCode::CPUI_BRANCH | OpCode::CPUI_BRANCHIND => {
                        if o.get_slot(vn) == 0 {
                            self.updaterank(param_rank::DIRECTREAD, state.best);
                        }
                    }
                    OpCode::CPUI_CBRANCH => {
                        if o.get_slot(vn) < 2 {
                            self.updaterank(param_rank::DIRECTREAD, state.best);
                        }
                    }
                    OpCode::CPUI_CALL | OpCode::CPUI_CALLIND => {
                        if o.get_slot(vn) == 0 {
                            self.updaterank(param_rank::DIRECTREAD, state.best);
                        } else {
                            self.numcalls += 1;
                            self.updaterank(param_rank::SUBFNPARAM, state.best);
                        }
                    }
                    OpCode::CPUI_CALLOTHER => {
                        self.updaterank(param_rank::DIRECTREAD, state.best);
                    }
                    OpCode::CPUI_RETURN => {
                        self.updaterank(param_rank::THISFNRETURN, state.best);
                    }
                    OpCode::CPUI_INDIRECT => {
                        self.updaterank(param_rank::INDIRECT, state.best);
                    }
                    OpCode::CPUI_MULTIEQUAL => {
                        // The only op that can form a loop in the graph.  Walk
                        // forward only if the path is not part of a loop.
                        let slot = o.get_slot(vn);
                        if !self.parent_is_loop_in(fd, op, slot) {
                            let out =
                                o.get_out().expect("walkforward: MULTIEQUAL getOut null (C++ UB)");
                            self.walkforward(fd, state, None, out);
                        }
                    }
                    _ => {
                        self.updaterank(param_rank::DIRECTREAD, state.best);
                    }
                }
            }
        }
        state.depth -= 1;
    }

    /// Walk backward through the definition of `vn`, folding op kinds into the
    /// rank (C++ `ParamMeasure::walkbackward`, paramid.cc:90-139).
    fn walkbackward(
        &mut self,
        fd: &Funcdata,
        state: &mut WalkState,
        ignoreop: Option<OpId>,
        vn: VarnodeId,
    ) {
        let v = fd.vbank().get(vn).expect("walkbackward: stale vn");
        if v.is_input() {
            self.updaterank(param_rank::THISFNPARAM, state.best);
            return;
        } else if !v.is_written() {
            self.updaterank(param_rank::THISFNPARAM, state.best); // TODO: not sure about this.
            return;
        }

        let opid = v.get_def().expect("walkbackward: written vn has a def");
        let o = fd.obank().get(opid).expect("walkbackward: stale op");
        let oc = o.code();
        match oc {
            OpCode::CPUI_BRANCH
            | OpCode::CPUI_BRANCHIND
            | OpCode::CPUI_CBRANCH
            | OpCode::CPUI_CALL
            | OpCode::CPUI_CALLIND => {}
            OpCode::CPUI_CALLOTHER => {
                if o.get_out().is_some() {
                    self.updaterank(param_rank::DIRECTREAD, state.best);
                }
            }
            OpCode::CPUI_RETURN => {
                self.updaterank(param_rank::SUBFNRETURN, state.best);
            }
            OpCode::CPUI_INDIRECT => {
                self.updaterank(param_rank::INDIRECT, state.best);
            }
            OpCode::CPUI_MULTIEQUAL => {
                // The only op that can form a loop in the graph.  Walk backward
                // only if the path is not part of a loop.
                let ninput = o.num_input();
                let mut slot = 0;
                while slot < ninput && self.rank != state.terminalrank {
                    if !self.parent_is_loop_in(fd, opid, slot) {
                        let inv = fd
                            .obank()
                            .get(opid)
                            .expect("walkbackward: stale op")
                            .get_in(slot)
                            .expect("walkbackward: MULTIEQUAL getIn null (C++ UB)");
                        self.walkbackward(fd, state, Some(opid), inv);
                    }
                    slot += 1;
                }
            }
            _ => {
                // Might be DIRECTWRITEWITHOUTREAD, but we do not know yet.  Try to
                // walk forward to see if there is at least one path forward (other
                // than the one we took here) without a direct read of this write.
                let vaddr = v.get_addr().clone();
                let vsize = v.get_size();
                let vtype = v.get_type().clone();
                let mut pmfw =
                    ParamMeasure::new(&vaddr, vsize, Some(vtype), ParamIDIO::Input);
                pmfw.calculate_rank(fd, false, vn, ignoreop);
                if pmfw.get_measure() == param_rank::DIRECTREAD {
                    self.updaterank(param_rank::DIRECTWRITEWITHREAD, state.best);
                } else {
                    self.updaterank(param_rank::DIRECTWRITEWITHOUTREAD, state.best);
                }
            }
        }
    }

    /// `op->getParent()->isLoopIn(slot)` (paramid.cc:78/124).
    ///
    /// A null parent (op not yet placed in a block) cannot have a loop edge;
    /// matches the C++ where every walked op is in the placed graph.
    fn parent_is_loop_in(&self, fd: &Funcdata, op: OpId, slot: i32) -> bool {
        match fd.obank().get(op).expect("parent_is_loop_in: stale op").get_parent() {
            None => false,
            Some(bl) => fd.bblocks_ref().block(bl).is_loop_in(slot),
        }
    }

    /// Write a plain-text description of the measure (C++
    /// `ParamMeasure::savePretty`, paramid.cc:177-184).
    pub fn save_pretty(&self, s: &mut String, _moredetail: bool) {
        let spacename = self.vndata.space.as_ref().map(|sp| sp.get_name()).unwrap_or("");
        s.push_str(&format!("  Space: {spacename}\n"));
        s.push_str(&format!("  Addr: {}\n", self.vndata.offset));
        s.push_str(&format!("  Size: {}\n", self.vndata.size));
        s.push_str(&format!("  Rank: {}\n", self.rank));
    }

    /// Encode the measure (C++ `ParamMeasure::encode`, paramid.cc:161-175).
    ///
    /// The C++ writes `<addr>` (the space attributes), then `vntype->encodeRef`,
    /// then (if `moredetail`) the `<rank>` child.  The `vntype->encodeRef` is the
    /// W8 type-marshal surface (`Datatype::encodeRef`), which is not yet ported,
    /// so the whole encode stubs out.
    pub fn encode(&self, _moredetail: bool) -> KunaResult<()> {
        // The encode genuinely consumes `vntype` (the `<addr>` type-ref child).
        let _has_type = self.vntype.is_some();
        Err(KunaError::lowlevel(
            "kuna rust port: ParamMeasure::encode needs Datatype::encodeRef (W8 type marshal)",
        ))
    }
}

// ===========================================================================
// ParamIDAnalysis (paramid.hh:70-79, paramid.cc:186-282)
// ===========================================================================

/// The standalone parameter-ID pipeline for a function (C++ `ParamIDAnalysis`,
/// paramid.hh:70-79).
///
/// Collects [`ParamMeasure`]s for the input and output locations of a function
/// and ranks each by walking the data-flow.
pub struct ParamIDAnalysis {
    /// The function's display name (cached for `savePretty`/`encode`).
    fdname: String,
    /// Measures for input locations (C++ `InputParamMeasures`).
    input_param_measures: Vec<ParamMeasure>,
    /// Measures for output locations (C++ `OutputParamMeasures`).
    output_param_measures: Vec<ParamMeasure>,
}

impl ParamIDAnalysis {
    /// Build the analysis for a function (C++ `ParamIDAnalysis::ParamIDAnalysis`,
    /// paramid.cc:186-235).
    ///
    /// `justproto = true` restricts collection to the recovered prototype; that
    /// path needs the real [`FuncProto`](crate::fspec::FuncProto) from
    /// `Funcdata::getFuncProto`, which is still the W3/W4 placeholder in the
    /// merged tree, so it stubs out.  `justproto = false` lists the input Varnodes
    /// outside the model (the `beginDef(input)` range) and is fully ported.
    pub fn new(fd: &Funcdata, justproto: bool) -> KunaResult<ParamIDAnalysis> {
        let mut analysis = ParamIDAnalysis {
            fdname: fd.get_name().to_string(),
            input_param_measures: Vec::new(),
            output_param_measures: Vec::new(),
        };
        if justproto {
            // const FuncProto &fproto( fd->getFuncProto() );
            //   numParams()/getParam(i)/getOutput() + findVarnodeInput + the
            //   RETURN-op output scan.  STUB(W4): Funcdata::funcp is still the
            //   placeholder (crate::context::FuncProto), not fspec::FuncProto, so
            //   the recovered prototype is unavailable here.
            return Err(KunaError::lowlevel(
                "kuna rust port: ParamIDAnalysis(justproto=true) needs the recovered \
                 FuncProto from Funcdata::getFuncProto (W4 — funcp is still the placeholder)",
            ));
        } else {
            // Need to list input varnodes that are outside of the model.
            // VarnodeDefSet: beginDef(Varnode::input)..endDef(Varnode::input).
            let inputs: Vec<VarnodeId> = fd.vbank().iter_def_flag(varnode_flags::input).collect();
            for invn in inputs {
                let v = fd.vbank().get(invn).expect("ParamIDAnalysis: stale input vn");
                let addr = v.get_addr().clone();
                let size = v.get_size();
                let vtype = v.get_type().clone();
                let mut pm =
                    ParamMeasure::new(&addr, size, Some(vtype), ParamIDIO::Input);
                pm.calculate_rank(fd, true, invn, None);
                analysis.input_param_measures.push(pm);
            }
        }
        Ok(analysis)
    }

    /// Number of input measures collected.
    pub fn num_input_measures(&self) -> usize {
        self.input_param_measures.len()
    }

    /// Number of output measures collected.
    pub fn num_output_measures(&self) -> usize {
        self.output_param_measures.len()
    }

    /// Borrow the i-th input measure.
    pub fn input_measure(&self, i: usize) -> &ParamMeasure {
        &self.input_param_measures[i]
    }

    /// Write a plain-text description of the analysis (C++
    /// `ParamIDAnalysis::savePretty`, paramid.cc:264-282).
    ///
    /// The C++ leading line also prints the function's address and the recovered
    /// model/extrapop; those read the W3/W4 `FuncProto` placeholder, so this port
    /// prints the function name + the measures (the prototype-derived header
    /// fields STUB with the prototype itself).
    pub fn save_pretty(&self, s: &mut String, moredetail: bool) {
        s.push_str(&format!("Param Measures\nFunction: {}\n", self.fdname));
        s.push_str(&format!("Num Params: {}\n", self.input_param_measures.len()));
        for pm in &self.input_param_measures {
            pm.save_pretty(s, moredetail);
        }
        s.push_str(&format!("Num Returns: {}\n", self.output_param_measures.len()));
        for pm in &self.output_param_measures {
            pm.save_pretty(s, moredetail);
        }
        s.push('\n');
    }

    /// Encode the analysis (C++ `ParamIDAnalysis::encode`, paramid.cc:237-262).
    ///
    /// Needs the recovered `FuncProto` (model/extrapop) and
    /// `ParamMeasure::encode` (`Datatype::encodeRef`); both STUB.
    pub fn encode(&self) -> KunaResult<()> {
        // Reference the element ids so they stay live until the marshal surface
        // (W4 FuncProto + W8 type-ref) is threaded.
        let _ = (&ELEM_PARAMMEASURES, &ELEM_PROTO, &ELEM_RANK);
        let _ = (&AttributeId::new("model", 13), &AttributeId::new("extrapop", 6));
        Err(KunaError::lowlevel(
            "kuna rust port: ParamIDAnalysis::encode needs FuncProto (model/extrapop, W4) \
             + ParamMeasure::encode (Datatype::encodeRef, W8)",
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use kuna_base::space::{
        addrspace_flags, spacetype, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
    };

    use crate::context::{ArchContext, BlockId, TypeOp};

    fn build_manager() -> AddrSpaceManager {
        let mut m = AddrSpaceManager::new();
        m.insert_space(Rc::new(ConstantSpace::new())).unwrap();
        m.insert_space(Rc::new(UniqueSpace::new(1, 0, false))).unwrap();
        m.insert_space(Rc::new(AddrSpace::new(
            spacetype::IPTR_PROCESSOR,
            "ram",
            false,
            8,
            1,
            2,
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

    fn ramspace(fd: &Funcdata) -> Rc<AddrSpace> {
        Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap())
    }

    fn add_op(fd: &mut Funcdata, bl: BlockId, opc: OpCode, ninput: i32, pc: Address) -> OpId {
        let op = fd.new_op(ninput, pc);
        fd.op_set_opcode(op, TypeOp::new(opc, 0, format!("{opc:?}")));
        fd.op_insert(op, bl, None);
        op
    }

    /// An input read directly by a non-special op ranks DIRECTREAD (the best
    /// rank for an input).
    #[test]
    fn input_directly_read_ranks_directread() {
        let mut fd = build_fd();
        let rs = ramspace(&fd);
        let root = fd.bblocks_root_pub();
        let bl = fd.bblocks_mut().new_block_basic(root);
        fd.bblocks_mut().set_start_block(root, bl);

        // input p at ram:0x100 size 4.
        let p = fd.new_varnode(4, &Address::new(Rc::clone(&rs), 0x100), None);
        let p = fd.set_input_varnode(p).unwrap();
        // out = p + p  (INT_ADD reads p directly).
        let add = add_op(&mut fd, bl, OpCode::CPUI_INT_ADD, 2, Address::new(Rc::clone(&rs), 0x1000));
        let _out = fd.new_varnode_out(4, &Address::new(Rc::clone(&rs), 0x200), add).unwrap();
        fd.op_set_input(add, p, 0).unwrap();
        fd.op_set_input(add, p, 1).unwrap();
        fd.structure_reset();

        let paddr = Address::new(Rc::clone(&rs), 0x100);
        let mut pm = ParamMeasure::new(&paddr, 4, None, ParamIDIO::Input);
        pm.calculate_rank(&fd, true, p, None);
        assert_eq!(pm.get_measure(), param_rank::DIRECTREAD);
    }

    /// An input with no descendants (dead) ranks WORSTRANK in the best (min)
    /// walk — nothing lowers it.
    #[test]
    fn dead_input_keeps_worstrank() {
        let mut fd = build_fd();
        let rs = ramspace(&fd);
        let root = fd.bblocks_root_pub();
        let bl = fd.bblocks_mut().new_block_basic(root);
        fd.bblocks_mut().set_start_block(root, bl);

        let p = fd.new_varnode(4, &Address::new(Rc::clone(&rs), 0x100), None);
        let p = fd.set_input_varnode(p).unwrap();
        fd.structure_reset();

        let paddr = Address::new(Rc::clone(&rs), 0x100);
        let mut pm = ParamMeasure::new(&paddr, 4, None, ParamIDIO::Input);
        pm.calculate_rank(&fd, true, p, None);
        assert_eq!(pm.get_measure(), param_rank::WORSTRANK);
    }

    /// An input passed to a CALL (slot != 0) ranks SUBFNPARAM and bumps numcalls.
    #[test]
    fn input_passed_to_call_ranks_subfnparam() {
        let mut fd = build_fd();
        let rs = ramspace(&fd);
        let root = fd.bblocks_root_pub();
        let bl = fd.bblocks_mut().new_block_basic(root);
        fd.bblocks_mut().set_start_block(root, bl);

        let p = fd.new_varnode(4, &Address::new(Rc::clone(&rs), 0x100), None);
        let p = fd.set_input_varnode(p).unwrap();
        // CALL target(p): input0 = call target (code ref), input1 = p.
        let call = add_op(&mut fd, bl, OpCode::CPUI_CALL, 2, Address::new(Rc::clone(&rs), 0x1000));
        let target = fd.new_code_ref(&Address::new(Rc::clone(&rs), 0x2000));
        fd.op_set_input(call, target, 0).unwrap();
        fd.op_set_input(call, p, 1).unwrap();
        fd.structure_reset();

        let paddr = Address::new(Rc::clone(&rs), 0x100);
        let mut pm = ParamMeasure::new(&paddr, 4, None, ParamIDIO::Input);
        pm.calculate_rank(&fd, true, p, None);
        assert_eq!(pm.get_measure(), param_rank::SUBFNPARAM);
    }

    /// The standalone analysis (non-justproto) lists every input Varnode and
    /// ranks each; two distinct inputs both read directly should both rank
    /// DIRECTREAD.
    #[test]
    fn analysis_lists_and_ranks_inputs() {
        let mut fd = build_fd();
        let rs = ramspace(&fd);
        let root = fd.bblocks_root_pub();
        let bl = fd.bblocks_mut().new_block_basic(root);
        fd.bblocks_mut().set_start_block(root, bl);

        let a = fd.new_varnode(4, &Address::new(Rc::clone(&rs), 0x100), None);
        let a = fd.set_input_varnode(a).unwrap();
        let b = fd.new_varnode(4, &Address::new(Rc::clone(&rs), 0x108), None);
        let b = fd.set_input_varnode(b).unwrap();
        // out = a + b reads both directly.
        let add = add_op(&mut fd, bl, OpCode::CPUI_INT_ADD, 2, Address::new(Rc::clone(&rs), 0x1000));
        let _out = fd.new_varnode_out(4, &Address::new(Rc::clone(&rs), 0x200), add).unwrap();
        fd.op_set_input(add, a, 0).unwrap();
        fd.op_set_input(add, b, 1).unwrap();
        fd.structure_reset();

        let analysis = ParamIDAnalysis::new(&fd, false).unwrap();
        assert_eq!(analysis.num_input_measures(), 2);
        assert_eq!(analysis.num_output_measures(), 0);
        for i in 0..analysis.num_input_measures() {
            assert_eq!(analysis.input_measure(i).get_measure(), param_rank::DIRECTREAD);
        }

        // savePretty renders the function name and per-measure lines.
        let mut s = String::new();
        analysis.save_pretty(&mut s, false);
        assert!(s.contains("Function: func"), "got: {s}");
        assert!(s.contains("Num Params: 2"), "got: {s}");
        assert!(s.contains("Rank: 2"), "got: {s}");
    }

    /// The justproto path stubs out (prototype unavailable in the merged tree).
    #[test]
    fn justproto_is_stub() {
        let fd = build_fd();
        assert!(ParamIDAnalysis::new(&fd, true).is_err());
    }
}
