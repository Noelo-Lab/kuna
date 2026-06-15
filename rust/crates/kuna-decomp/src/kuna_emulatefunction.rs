//! Port of `decompiler/cpp/emulateutil.{cc,hh}` `EmulatePcodeOp` plus
//! `decompiler/cpp/jumptable.{cc,hh}` `EmulateFunction` — the lightweight
//! syntax-tree emulator the jump-table recovery uses to follow a switch
//! variable's value through a `PathMeld` to the final BRANCHIND target address.
//!
//! `EmulateFunction` (jumptable.hh:110) derives from `EmulatePcodeOp`
//! (emulateutil.hh:43), which in turn derives from the SLEIGH `Emulate` base.
//! The kuna port folds the two C++ classes into one [`EmulateFunction`] struct
//! that implements the ported [`Emulate`](kuna_sleigh::emulate::Emulate) trait
//! and supplies the `EmulatePcodeOp` method bodies directly (Rust has no
//! protected-virtual inheritance; the `EmulateFunction` overrides — `executeLoad`
//! collecting LOAD points, `executeBranch`/`executeBranchind` throwing,
//! `executeCall`/`executeCallind`/`executeCallother` ignored — replace the base
//! bodies, exactly as the C++ virtual overrides do).
//!
//! The emulator operates over the function's syntax tree: a "memory state" keyed
//! by [`VarnodeId`] ([`varnode_map`](EmulateFunction::varnode_map), C++
//! `map<Varnode *,uintb>`), constants read off the Varnode, and RAM/register
//! reads pulled from the load image (C++ `getLoadImageValue`).  Control flow is
//! driven externally by [`emulate_path`](EmulateFunction::emulate_path): there is
//! exactly one execution path even though there may be multiple data-flow paths.

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::space::AddrSpace;
use kuna_base::types::{int4, uintb};

use kuna_num::opbehavior::OpBehavior;
use kuna_num::opcodes::OpCode;

use kuna_sleigh::emulate::{Emulate, EmulateCore};

use crate::funcdata::Funcdata;
use crate::jumptable::{LoadTable, PathMeld};
use crate::seams::{OpId, VarnodeId};

/// C++ `EmulateFunction` (jumptable.hh:110), folding in the `EmulatePcodeOp`
/// base (emulateutil.hh:43).
///
/// Borrows the [`Funcdata`] immutably — emulation only *reads* the syntax tree
/// (the C++ `EmulateFunction(Funcdata *)` stores a `Funcdata *` and never mutates
/// it during a path emulation).
pub struct EmulateFunction<'a> {
    /// C++ base-class members (`emu_halted` / `currentBehave`).
    core: EmulateCore,
    /// The function being emulated (C++ `EmulateFunction::fd`).
    fd: &'a Funcdata,
    /// Light-weight memory state based on Varnodes (C++ `varnodeMap`).
    varnode_map: BTreeMap<VarnodeId, uintb>,
    /// The set of collected LOAD records, if collecting (C++ `loadpoints`).
    loadpoints: Option<Vec<LoadTable>>,
    /// C++ `EmulatePcodeOp::currentOp`: the op the next `execute_current_op`
    /// will run.
    current_op: Option<OpId>,
    /// C++ `EmulatePcodeOp::lastOp`: the previously executed op (drives
    /// MULTIEQUAL branch selection).
    last_op: Option<OpId>,
}

impl<'a> EmulateFunction<'a> {
    /// C++ `EmulateFunction::EmulateFunction(Funcdata *f)`.
    pub fn new(fd: &'a Funcdata) -> EmulateFunction<'a> {
        EmulateFunction {
            core: EmulateCore::new(),
            fd,
            varnode_map: BTreeMap::new(),
            loadpoints: None,
            current_op: None,
            last_op: None,
        }
    }

    /// C++ `EmulateFunction::setLoadCollect`: collect LOAD addresses into a
    /// fresh vector (the caller pulls them out with [`take_loadpoints`]).
    pub fn set_load_collect(&mut self, collect: bool) {
        self.loadpoints = if collect { Some(Vec::new()) } else { None };
    }

    /// Take the collected LOAD records (C++ writes them through the
    /// `loadpoints` pointer the caller owns).
    pub fn take_loadpoints(&mut self) -> Option<Vec<LoadTable>> {
        self.loadpoints.take()
    }

    /// The number of LOAD records collected so far (C++ `loadpoints->size()`,
    /// read after each `emulatePath` to populate `loadcounts`).  `None` when not
    /// collecting.
    pub fn loadpoints_len(&self) -> Option<usize> {
        self.loadpoints.as_ref().map(|v| v.len())
    }

    /// Set the executing op directly by id (the C++ `setCurrentOp` reads the
    /// op's `OpBehavior` into `currentBehave`).
    fn set_current_op(&mut self, op: OpId) {
        self.current_op = Some(op);
        let opc = self.fd.obank().get(op).map(|o| o.code()).unwrap_or(OpCode::CPUI_COPY);
        self.core.current_behave = self.fd.get_arch().op_behavior(opc).cloned();
    }

    /// C++ `EmulateFunction::getVarnodeValue(Varnode *vn)`: read a Varnode out
    /// of the syntax-tree memory state.  A constant returns its offset; a seen
    /// Varnode returns its stored value; otherwise the load image is consulted.
    fn get_varnode_value(&self, vn: VarnodeId) -> KunaResult<uintb> {
        let v = self
            .fd
            .vbank()
            .get(vn)
            .ok_or_else(|| KunaError::lowlevel("EmulateFunction: dangling varnode"))?;
        if v.is_constant() {
            return Ok(v.get_offset());
        }
        if let Some(val) = self.varnode_map.get(&vn) {
            return Ok(*val); // We have seen this varnode before
        }
        self.get_load_image_value(v.get_space(), v.get_offset(), v.get_size())
    }

    /// C++ `EmulateFunction::setVarnodeValue(Varnode *vn,uintb val)`.
    fn set_varnode_value(&mut self, vn: VarnodeId, val: uintb) {
        self.varnode_map.insert(vn, val);
    }

    /// C++ `EmulatePcodeOp::getLoadImageValue(AddrSpace*,uintb,int4)` — pull a
    /// value out of the program load image (the read-only switch table).
    fn get_load_image_value(&self, spc: &Rc<AddrSpace>, off: uintb, sz: int4) -> KunaResult<uintb> {
        let addr = Address::new(Rc::clone(spc), off);
        self.fd.get_arch().get_load_image_value(&addr, sz)
    }

    /// Resolve a LOAD/STORE `<spaceid>` constant operand to its `AddrSpace`
    /// (C++ `Varnode::getSpaceFromConst`: the constant's offset is the
    /// space-manager index).
    fn space_from_const(&self, vn: VarnodeId) -> KunaResult<Rc<AddrSpace>> {
        let idx = self
            .fd
            .vbank()
            .get(vn)
            .map(|v| v.get_offset())
            .ok_or_else(|| KunaError::lowlevel("getSpaceFromConst: dangling varnode"))?;
        let manage = self.fd.get_arch().manage();
        if idx >= manage.num_spaces() as u64 {
            return Err(KunaError::lowlevel("getSpaceFromConst: bad space index"));
        }
        manage
            .get_space(idx as i32)
            .cloned()
            .ok_or_else(|| KunaError::lowlevel("getSpaceFromConst: no space"))
    }

    /// C++ `EmulateFunction::emulatePath(uintb,const PathMeld&,PcodeOp*,Varnode*)`
    /// (jumptable.cc:217): flow the given value through every path in the
    /// container to the single output value at `pathMeld.getOp(0)`'s in0.
    pub fn emulate_path(
        &mut self,
        val: uintb,
        path_meld: &PathMeld,
        startop: OpId,
        startvn: VarnodeId,
    ) -> KunaResult<uintb> {
        let num_ops = path_meld.num_ops();
        // for(i=0;i<pathMeld.numOps();++i) if (pathMeld.getOp(i) == startop) break;
        let mut i: int4 = 0;
        while i < num_ops {
            if path_meld.get_op(i) == Some(startop) {
                break;
            }
            i += 1;
        }
        let mut startop = startop;
        let mut startvn = startvn;
        if self.fd.obank().get(startop).map(|o| o.code()) == Some(OpCode::CPUI_MULTIEQUAL) {
            // If we start on a MULTIEQUAL: is startvn one of the branches?
            let ninput = self.fd.obank().get(startop).map(|o| o.num_input()).unwrap_or(0);
            let mut j: int4 = 0;
            while j < ninput {
                if self.fd.obank().get(startop).and_then(|o| o.get_in(j)) == Some(startvn) {
                    break;
                }
                j += 1;
            }
            if j == ninput || i == 0 {
                return Err(KunaError::lowlevel(
                    "Cannot start jumptable emulation with unresolved MULTIEQUAL",
                ));
            }
            // Emulate as if we just came from that branch: the MULTIEQUAL output
            // becomes the new startvn (a COPY from the old startvn).
            startvn = self
                .fd
                .obank()
                .get(startop)
                .and_then(|o| o.get_out())
                .ok_or_else(|| KunaError::lowlevel("MULTIEQUAL without output"))?;
            i -= 1; // Move to the next instruction to be executed
            startop = path_meld
                .get_op(i)
                .ok_or_else(|| KunaError::lowlevel("Bad jumptable emulation"))?;
        }
        // Suppress unused-assignment lint on startop after the rebind.
        let _ = startop;
        if i == num_ops {
            return Err(KunaError::lowlevel("Bad jumptable emulation"));
        }
        let startvn_is_const =
            self.fd.vbank().get(startvn).map(|v| v.is_constant()).unwrap_or(false);
        if !startvn_is_const {
            self.set_varnode_value(startvn, val);
        }
        while i > 0 {
            let curop = path_meld
                .get_op(i)
                .ok_or_else(|| KunaError::lowlevel("Bad jumptable emulation"))?;
            i -= 1;
            self.set_current_op(curop);
            self.execute_current_op().map_err(|_| {
                let addr = self
                    .fd
                    .obank()
                    .get(curop)
                    .map(|o| o.get_addr().clone())
                    .unwrap_or_default();
                KunaError::lowlevel(format!("Could not emulate address calculation at {addr:?}"))
            })?;
        }
        let op0 = path_meld
            .get_op(0)
            .ok_or_else(|| KunaError::lowlevel("Bad jumptable emulation"))?;
        let invn = self
            .fd
            .obank()
            .get(op0)
            .and_then(|o| o.get_in(0))
            .ok_or_else(|| KunaError::lowlevel("Bad jumptable emulation"))?;
        self.get_varnode_value(invn)
    }
}

impl Emulate for EmulateFunction<'_> {
    fn emulate_core(&self) -> &EmulateCore {
        &self.core
    }

    fn emulate_core_mut(&mut self) -> &mut EmulateCore {
        &mut self.core
    }

    /// C++ `EmulatePcodeOp::executeUnary`.
    fn execute_unary(&mut self) -> KunaResult<()> {
        let op = self.current_op.expect("executeUnary: no current op");
        let behave = self.behave();
        let in0 = self.in_vn(op, 0)?;
        let in1val = self.get_varnode_value(in0)?;
        let outvn = self.out_vn(op)?;
        let out = behave.evaluate_unary(self.size(outvn)?, self.size(in0)?, in1val)?;
        self.set_varnode_value(outvn, out);
        Ok(())
    }

    /// C++ `EmulatePcodeOp::executeBinary`.
    fn execute_binary(&mut self) -> KunaResult<()> {
        let op = self.current_op.expect("executeBinary: no current op");
        let behave = self.behave();
        let in0 = self.in_vn(op, 0)?;
        let in1 = self.in_vn(op, 1)?;
        let in0val = self.get_varnode_value(in0)?;
        let in1val = self.get_varnode_value(in1)?;
        let outvn = self.out_vn(op)?;
        let out = behave.evaluate_binary(self.size(outvn)?, self.size(in0)?, in0val, in1val)?;
        self.set_varnode_value(outvn, out);
        Ok(())
    }

    /// C++ `EmulateFunction::executeLoad` (the override): record the LOAD point
    /// if collecting, then the base `EmulatePcodeOp::executeLoad`.
    fn execute_load(&mut self) -> KunaResult<()> {
        let op = self.current_op.expect("executeLoad: no current op");
        let in1 = self.in_vn(op, 1)?;
        let in0 = self.in_vn(op, 0)?;
        let outvn = self.out_vn(op)?;
        let spc = self.space_from_const(in0)?;
        if self.loadpoints.is_some() {
            let off = self.get_varnode_value(in1)?;
            let off = AddrSpace::address_to_byte(off, spc.get_word_size());
            let sz = self.size(outvn)?;
            if let Some(lp) = self.loadpoints.as_mut() {
                lp.push(LoadTable::single(Address::new(Rc::clone(&spc), off), sz));
            }
        }
        // EmulatePcodeOp::executeLoad
        let off = self.get_varnode_value(in1)?;
        let off = AddrSpace::address_to_byte(off, spc.get_word_size());
        let sz = self.size(outvn)?;
        let res = self.get_load_image_value(&spc, off, sz)?;
        self.set_varnode_value(outvn, res);
        Ok(())
    }

    /// C++ `EmulatePcodeOp::executeStore`: nowhere to store (null memstate).
    fn execute_store(&mut self) -> KunaResult<()> {
        Ok(())
    }

    /// C++ `EmulateFunction::executeBranch` (override): throws.
    fn execute_branch(&mut self) -> KunaResult<()> {
        Err(KunaError::lowlevel(
            "Branch encountered emulating jumptable calculation",
        ))
    }

    /// C++ `EmulatePcodeOp::executeCbranch`.
    fn execute_cbranch(&mut self) -> KunaResult<bool> {
        let op = self.current_op.expect("executeCbranch: no current op");
        let in1 = self.in_vn(op, 1)?;
        let cond = self.get_varnode_value(in1)?;
        let flip = self.fd.obank().get(op).map(|o| o.is_boolean_flip()).unwrap_or(false);
        Ok((cond != 0) != flip)
    }

    /// C++ `EmulateFunction::executeBranchind` (override): throws.
    fn execute_branchind(&mut self) -> KunaResult<()> {
        Err(KunaError::lowlevel(
            "Indirect branch encountered emulating jumptable calculation",
        ))
    }

    /// C++ `EmulateFunction::executeCall` (override): ignore calls.
    fn execute_call(&mut self) -> KunaResult<()> {
        self.fallthru_op()
    }

    /// C++ `EmulateFunction::executeCallind` (override): ignore calls.
    fn execute_callind(&mut self) -> KunaResult<()> {
        self.fallthru_op()
    }

    /// C++ `EmulateFunction::executeCallother` (override): ignore callothers.
    fn execute_callother(&mut self) -> KunaResult<()> {
        self.fallthru_op()
    }

    /// C++ `EmulatePcodeOp::executeMultiequal`.
    fn execute_multiequal(&mut self) -> KunaResult<()> {
        let op = self.current_op.expect("executeMultiequal: no current op");
        let last = self
            .last_op
            .ok_or_else(|| KunaError::lowlevel("Could not execute MULTIEQUAL"))?;
        let bl = self
            .fd
            .obank()
            .get(op)
            .and_then(|o| o.get_parent())
            .ok_or_else(|| KunaError::lowlevel("MULTIEQUAL without parent block"))?;
        let last_bl = self
            .fd
            .obank()
            .get(last)
            .and_then(|o| o.get_parent())
            .ok_or_else(|| KunaError::lowlevel("Could not execute MULTIEQUAL"))?;
        let graph = self.fd.bblocks_ref();
        let size_in = graph.block(bl).size_in();
        let mut i: int4 = 0;
        while i < size_in {
            if graph.block(bl).get_in(i) == last_bl {
                break;
            }
            i += 1;
        }
        if i == size_in {
            return Err(KunaError::lowlevel("Could not execute MULTIEQUAL"));
        }
        let invn = self.in_vn(op, i)?;
        let val = self.get_varnode_value(invn)?;
        let outvn = self.out_vn(op)?;
        self.set_varnode_value(outvn, val);
        Ok(())
    }

    /// C++ `EmulatePcodeOp::executeIndirect`: treat as a copy of in0.
    fn execute_indirect(&mut self) -> KunaResult<()> {
        let op = self.current_op.expect("executeIndirect: no current op");
        let in0 = self.in_vn(op, 0)?;
        let val = self.get_varnode_value(in0)?;
        let outvn = self.out_vn(op)?;
        self.set_varnode_value(outvn, val);
        Ok(())
    }

    /// C++ `EmulatePcodeOp::executeSegmentOp`: segment ops are unported in the
    /// jump-table emulator (the corpus switches do not use them) — a faithful
    /// "Segment operand missing definition" error on the path forces the
    /// recovery to fall back, never a wrong address.
    fn execute_segment_op(&mut self) -> KunaResult<()> {
        Err(KunaError::lowlevel("Segment operand missing definition"))
    }

    /// C++ `EmulatePcodeOp::executeCpoolRef`: ignore.
    fn execute_cpool_ref(&mut self) -> KunaResult<()> {
        Ok(())
    }

    /// C++ `EmulatePcodeOp::executeNew`: ignore.
    fn execute_new(&mut self) -> KunaResult<()> {
        Ok(())
    }

    /// C++ `EmulateFunction::fallthruOp`: keep lastOp for MULTIEQUAL, outer loop
    /// controls execution flow.
    fn fallthru_op(&mut self) -> KunaResult<()> {
        self.last_op = self.current_op;
        Ok(())
    }

    /// C++ `EmulateFunction::setExecuteAddress`.
    fn set_execute_address(&mut self, addr: &Address) -> KunaResult<()> {
        let physical = addr.get_space().map(|s| s.has_physical()).unwrap_or(false);
        if !physical {
            return Err(KunaError::lowlevel("Bad execute address"));
        }
        let op = self
            .fd
            .op_bank_target(addr)
            .ok_or_else(|| KunaError::lowlevel("Could not set execute address"))?;
        self.set_current_op(op);
        Ok(())
    }

    /// C++ `Emulate::getExecuteAddress` (the current op's address).
    fn get_execute_address(&self) -> Address {
        self.current_op
            .and_then(|op| self.fd.obank().get(op))
            .map(|o| o.get_addr().clone())
            .unwrap_or_default()
    }
}

impl EmulateFunction<'_> {
    /// The current op's behavior (`currentBehave`).
    fn behave(&self) -> Rc<dyn OpBehavior> {
        Rc::clone(
            self.core
                .current_behave
                .as_ref()
                .expect("EmulateFunction: currentBehave not set (C++ would dereference null)"),
        )
    }

    /// `currentOp->getIn(slot)`.
    fn in_vn(&self, op: OpId, slot: int4) -> KunaResult<VarnodeId> {
        self.fd
            .obank()
            .get(op)
            .and_then(|o| o.get_in(slot))
            .ok_or_else(|| KunaError::lowlevel("EmulateFunction: missing op input"))
    }

    /// `currentOp->getOut()`.
    fn out_vn(&self, op: OpId) -> KunaResult<VarnodeId> {
        self.fd
            .obank()
            .get(op)
            .and_then(|o| o.get_out())
            .ok_or_else(|| KunaError::lowlevel("EmulateFunction: op without output"))
    }

    /// `vn->getSize()`.
    fn size(&self, vn: VarnodeId) -> KunaResult<int4> {
        self.fd
            .vbank()
            .get(vn)
            .map(|v| v.get_size())
            .ok_or_else(|| KunaError::lowlevel("EmulateFunction: dangling varnode size"))
    }
}

/// A `RefCell`-wrapped temporary so a hand-built fixture can construct an
/// emulator without a shared `Funcdata` borrow conflict (test-support only).
#[allow(dead_code)]
type EmulateCell<'a> = RefCell<EmulateFunction<'a>>;
