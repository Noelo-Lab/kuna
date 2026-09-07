//! (kuna) `kuna_constselectjump` — recover an indirect branch whose destination
//! is selected between constants (S2 switch-model).
//!
//! ## The structural gap
//!
//! An obfuscated (or `-O2` predicated) dispatch computes its destination with a
//! conditional move rather than a table:
//!
//! ```text
//!   LEA  R10,[0x10003cb26]
//!   LEA  R11,[0x10003ca26]
//!   CMOVZ R11,R10
//!   JMP  R11
//! ```
//!
//! SLEIGH lifts the `CMOVcc` to a real conditional branch, so at the `BRANCHIND`
//! the destination varnode is a `MULTIEQUAL` of two *constants*.  Both models the
//! [`JumpTable`](crate::jumptable::JumpTable) tries are value-range models:
//! [`JumpBasic::isprune`](crate::jumptable::JumpBasic::isprune) prunes at any
//! marker op, so the `MULTIEQUAL` output becomes the normalized switch variable
//! and nothing bounds it.  The model is declined, `FlowInfo::truncateIndirectJump`
//! rewrites the `BRANCHIND` to a `CALLIND` with a `"Treating indirect jump as
//! call"` warning, and flow stops — the whole decoder body behind both
//! destinations is never decoded.
//!
//! ## What this module owns
//!
//! The destination set is not lost, it is simply never promoted into the CFG:
//! both targets are literal constants reaching one marker op.  When
//! `option constselectjump on`, [`kuna_collect_const_destinations`] walks the
//! `MULTIEQUAL`/`COPY` web behind the `BRANCHIND` input and, if *every* reaching
//! definition is a constant, hands the resulting set to
//! [`JumpModelConstSelect`] — a fixed-table [`JumpModel`] in the shape of
//! `JumpModelTrivial`, whose case labels are the destination addresses
//! themselves.  Recovery then proceeds exactly as for a recovered table: the
//! `BRANCHIND` keeps its opcode, both targets are queued as intraprocedural flow,
//! and the decoder body is decoded.
//!
//! The walk is deliberately narrow.  It admits only `MULTIEQUAL` and `COPY`, and
//! it requires at least one `MULTIEQUAL` and at least two distinct destinations,
//! so it can never claim a case either basic model already handles (a single
//! constant destination is a value-range of size one, which `JumpBasic` recovers).
//! Every destination must also lie within [`MAX_INTRAPROCEDURAL_SPAN`] bytes of
//! the branch, which is upstream's own intraprocedural-vs-thunk yardstick
//! (`JumpTable::sanityCheck`, `jumptable.cc:2450`): a conditional *tail call*
//! selecting between two far-away function entries stays a `CALLIND`, where
//! rendering it as a call is the honest answer.

use std::collections::HashSet;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::KunaResult;
use kuna_base::space::AddrSpace;
use kuna_base::types::{int4, uint4, uintb, Wrap};
use kuna_num::opcodes::OpCode;

use crate::context::{OpId, VarnodeId};
use crate::funcdata::Funcdata;
use crate::jumptable::{JumpModel, LoadTable};

/// Widest distance from the indirect branch at which a constant destination is
/// still taken as intraprocedural.  Upstream uses the same `0xffff` window to
/// separate a real switch target from a thunk (`JumpTable::sanityCheck`).
pub const MAX_INTRAPROCEDURAL_SPAN: uintb = 0xffff;

/// Ceiling on the varnodes the backward walk will visit before giving up.
const MAX_WALK_NODES: usize = 64;

/// (kuna) Is `dest` close enough to the branch at `branch_offset` to be taken as
/// an intraprocedural destination rather than a tail-call target?
pub fn kuna_destination_is_local(branch_offset: uintb, dest: uintb) -> bool {
    if dest == 0 {
        return false;
    }
    let span = if dest < branch_offset {
        branch_offset.wsub(dest)
    } else {
        dest.wsub(branch_offset)
    };
    span <= MAX_INTRAPROCEDURAL_SPAN
}

/// (kuna) Toggle constant-select indirect-branch recovery:
/// `constselectjump on|off`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConstSelectJumpOption {
    /// True if a `BRANCHIND` whose destination is a select over constants is
    /// recovered as a jump table (`Architecture::const_select_jump`).
    pub enabled: bool,
}

impl Default for ConstSelectJumpOption {
    /// Shipped default: `option constselectjump off` (upstream byte-identical).
    fn default() -> Self {
        ConstSelectJumpOption { enabled: false }
    }
}

impl ConstSelectJumpOption {
    /// (kuna) Set the gate.
    pub fn apply(&mut self, val: bool) -> &'static str {
        self.enabled = val;
        if val {
            "Constant-select indirect branch recovery turned on"
        } else {
            "Constant-select indirect branch recovery turned off"
        }
    }

    /// Read the gate (`glb->const_select_jump`).
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

/// (kuna) Collect the constant destination set reaching `root`, or `None` if the
/// destination is not provably a select over constants.
///
/// `branch_offset` is the address of the indirect branch and `maxtablesize` the
/// architecture's table ceiling; both are the guards described in the module
/// header.  The returned set is sorted and deduplicated.
pub fn kuna_collect_const_destinations(
    fd: &Funcdata,
    root: VarnodeId,
    branch_offset: uintb,
    maxtablesize: uint4,
) -> Option<Vec<uintb>> {
    let mut worklist: Vec<VarnodeId> = vec![root];
    let mut seen: HashSet<VarnodeId> = HashSet::new();
    let mut dests: Vec<uintb> = Vec::new();
    let mut selects = 0usize;

    while let Some(vn) = worklist.pop() {
        if !seen.insert(vn) {
            continue;
        }
        if seen.len() > MAX_WALK_NODES {
            return None;
        }
        let v = fd.vbank().get(vn)?;
        if v.is_constant() {
            let off = v.get_offset();
            if !kuna_destination_is_local(branch_offset, off) {
                return None;
            }
            if !dests.contains(&off) {
                dests.push(off);
                if dests.len() > maxtablesize as usize {
                    return None;
                }
            }
            continue;
        }
        if !v.is_written() {
            return None;
        }
        let op = fd.obank().get(v.get_def()?)?;
        match op.code() {
            OpCode::CPUI_MULTIEQUAL => {
                selects += 1;
                for i in 0..op.num_input() {
                    worklist.push(op.get_in(i)?);
                }
            }
            OpCode::CPUI_COPY => worklist.push(op.get_in(0)?),
            _ => return None,
        }
    }

    if selects == 0 || dests.len() < 2 {
        return None;
    }
    dests.sort_unstable();
    Some(dests)
}

/// (kuna) A jump-table model over a fixed set of constant destinations.
///
/// Shaped after `JumpModelTrivial`: the destinations are recovered once, the
/// address table is that set, and each case label is its own target address (the
/// `BRANCHIND` input *is* the destination, so the label is the value).  There is
/// no normalization and no guard to fold.
#[derive(Debug, Clone, Default)]
pub struct JumpModelConstSelect {
    /// The recovered destination offsets, sorted and deduplicated.
    dests: Vec<uintb>,
}

impl JumpModelConstSelect {
    /// Construct an empty model.
    pub fn new() -> JumpModelConstSelect {
        JumpModelConstSelect { dests: Vec::new() }
    }

    /// The recovered destination offsets.
    pub fn destinations(&self) -> &[uintb] {
        &self.dests
    }
}

impl JumpModel for JumpModelConstSelect {
    fn is_override(&self) -> bool {
        false
    }

    fn get_table_size(&self) -> int4 {
        self.dests.len() as int4
    }

    fn recover_model(
        &mut self,
        fd: &mut Funcdata,
        indop: OpId,
        _matchsize: uint4,
        maxtablesize: uint4,
    ) -> KunaResult<bool> {
        self.dests.clear();
        let (root, branch_offset) = match fd.obank().get(indop) {
            Some(o) => match o.get_in(0) {
                Some(vn) => (vn, o.get_addr().get_offset()),
                None => return Ok(false),
            },
            None => return Ok(false),
        };
        match kuna_collect_const_destinations(fd, root, branch_offset, maxtablesize) {
            Some(d) => {
                self.dests = d;
                Ok(true)
            }
            None => Ok(false),
        }
    }

    fn build_addresses(
        &self,
        fd: &Funcdata,
        indop: OpId,
        addresstable: &mut Vec<Address>,
        _loadpoints: Option<&mut Vec<LoadTable>>,
        _loadcounts: Option<&mut Vec<int4>>,
    ) -> KunaResult<()> {
        addresstable.clear();
        let spc = Rc::clone(fd.obank().get(indop).unwrap().get_addr().get_space().unwrap());
        let mut mask: uintb = !0u64;
        let bit = fd.get_arch().funcptr_align;
        if bit != 0 {
            mask = (mask >> bit) << bit;
        }
        for &d in self.dests.iter() {
            let off = AddrSpace::address_to_byte(d, spc.get_word_size()) & mask;
            addresstable.push(Address::new(Rc::clone(&spc), off));
        }
        Ok(())
    }

    fn find_unnormalized(
        &mut self,
        _fd: &mut Funcdata,
        _maxaddsub: uint4,
        _maxleftright: uint4,
        _maxext: uint4,
    ) -> KunaResult<()> {
        Ok(())
    }

    fn build_labels(
        &self,
        _fd: &mut Funcdata,
        addresstable: &mut [Address],
        label: &mut Vec<uintb>,
        _orig: &dyn JumpModel,
    ) -> KunaResult<()> {
        for a in addresstable.iter() {
            label.push(a.get_offset());
        }
        Ok(())
    }

    fn fold_in_normalization(
        &mut self,
        _fd: &mut Funcdata,
        _indop: OpId,
    ) -> KunaResult<Option<VarnodeId>> {
        Ok(None)
    }

    fn fold_in_guards(
        &mut self,
        _fd: &mut Funcdata,
        _jump: &mut crate::jumptable::JumpTable,
    ) -> KunaResult<bool> {
        Ok(false)
    }

    /// Every destination must have loadable bytes behind it, or the table is
    /// declined whole and the branch keeps the upstream `CALLIND` rendering:
    /// unlike a recovered LOAD table there is no partial answer worth keeping,
    /// and decoding an unmapped address is worse than reporting a call.
    fn sanity_check(
        &mut self,
        fd: &mut Funcdata,
        _indop: OpId,
        addresstable: &mut Vec<Address>,
        _loadpoints: &mut Vec<LoadTable>,
        _loadcounts: Option<&Vec<int4>>,
    ) -> KunaResult<bool> {
        if addresstable.is_empty() {
            return Ok(false);
        }
        for a in addresstable.iter() {
            if fd.get_arch().get_load_image_value(a, 4).is_err() {
                return Ok(false);
            }
        }
        Ok(true)
    }

    fn clone_box(&self) -> Box<dyn JumpModel> {
        Box::new(JumpModelConstSelect { dests: self.dests.clone() })
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[cfg(test)]
#[path = "kuna_constselectjump/tests.rs"]
mod tests;
