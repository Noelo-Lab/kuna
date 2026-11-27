//! (kuna `loweredswitch`) Decide a re-rolled switch's install before restarting
//! for it.
//!
//! [`ActionLowerSwitchDetect`](crate::kuna_loweredswitch::ActionLowerSwitchDetect)
//! records a compare cascade on the simplified graph and restarts the function so
//! that [`ActionLowerSwitchInstall`](crate::kuna_loweredswitch::ActionLowerSwitchInstall)
//! can build the switch on the raw p-code, before heritage. The install declines
//! when that raw p-code does not support the switch; the restarted pass is then
//! the first pass again, and the restart only doubled the function's cost.
//!
//! Everything the install decides with reads the raw p-code at its slot, which a
//! restart lifts again unchanged. A pass that has no record for its function
//! keeps a [`RawSnapshot`] of it there, and detection runs the install's own
//! checks ([`install_declines`]) on the snapshot. A cascade they decline is
//! recorded without a restart, as the restarted pass would have left it.
//!
//! [`RawIr`] is the read-only view those checks take, implemented by
//! [`Funcdata`] and by [`RawSnapshot`].

use kuna_base::address::Address;
use kuna_base::types::{int4, uintb};
use kuna_num::opcodes::OpCode;
use slotmap::SecondaryMap;

use crate::block::{block_get_start, BlockType};
use crate::context::{BlockId, OpId, VarnodeId};
use crate::fspec::FuncProto;
use crate::funcdata::Funcdata;
use crate::op::PcodeOp;
use crate::p2_lift::kuna_loweredswitch::KunaLoweredSwitchRecord;
use crate::varnode::Varnode;

/// The parts of a p-code op the install's checks read.
pub(crate) trait RawOp {
    fn code(&self) -> OpCode;
    fn get_addr(&self) -> &Address;
    fn get_out(&self) -> Option<VarnodeId>;
    fn num_input(&self) -> int4;
    fn get_in(&self, slot: int4) -> Option<VarnodeId>;
}

/// The parts of a varnode the install's checks read.
pub(crate) trait RawVn {
    fn get_addr(&self) -> &Address;
    fn get_offset(&self) -> uintb;
    fn get_size(&self) -> int4;
    fn is_constant(&self) -> bool;
    fn is_written(&self) -> bool;
    fn get_def(&self) -> Option<OpId>;
}

/// A read-only view of a function's basic blocks, ops and varnodes.
pub(crate) trait RawIr {
    type Op: RawOp;
    type Vn: RawVn;
    fn raw_op(&self, op: OpId) -> Option<&Self::Op>;
    fn raw_vn(&self, vn: VarnodeId) -> Option<&Self::Vn>;
    fn raw_num_blocks(&self) -> int4;
    fn raw_block(&self, i: int4) -> BlockId;
    fn raw_is_basic(&self, b: BlockId) -> bool;
    fn raw_is_entry(&self, b: BlockId) -> bool;
    fn raw_start(&self, b: BlockId) -> Address;
    fn raw_size_in(&self, b: BlockId) -> int4;
    fn raw_in(&self, b: BlockId, i: int4) -> BlockId;
    fn raw_size_out(&self, b: BlockId) -> int4;
    fn raw_out(&self, b: BlockId, i: int4) -> BlockId;
    fn raw_ops(&self, b: BlockId) -> Vec<OpId>;
    fn raw_tail(&self, b: BlockId) -> Option<OpId>;
    fn raw_proto(&self) -> &FuncProto;
    /// Whether a jump table hangs off the op at `addr`.
    fn raw_table_at(&self, addr: &Address) -> bool;
}

impl RawOp for PcodeOp {
    fn code(&self) -> OpCode {
        PcodeOp::code(self)
    }
    fn get_addr(&self) -> &Address {
        PcodeOp::get_addr(self)
    }
    fn get_out(&self) -> Option<VarnodeId> {
        PcodeOp::get_out(self)
    }
    fn num_input(&self) -> int4 {
        PcodeOp::num_input(self)
    }
    fn get_in(&self, slot: int4) -> Option<VarnodeId> {
        PcodeOp::get_in(self, slot)
    }
}

impl RawVn for Varnode {
    fn get_addr(&self) -> &Address {
        Varnode::get_addr(self)
    }
    fn get_offset(&self) -> uintb {
        Varnode::get_offset(self)
    }
    fn get_size(&self) -> int4 {
        Varnode::get_size(self)
    }
    fn is_constant(&self) -> bool {
        Varnode::is_constant(self)
    }
    fn is_written(&self) -> bool {
        Varnode::is_written(self)
    }
    fn get_def(&self) -> Option<OpId> {
        Varnode::get_def(self)
    }
}

impl RawIr for Funcdata {
    type Op = PcodeOp;
    type Vn = Varnode;
    fn raw_op(&self, op: OpId) -> Option<&PcodeOp> {
        self.obank().get(op)
    }
    fn raw_vn(&self, vn: VarnodeId) -> Option<&Varnode> {
        self.vbank().get(vn)
    }
    fn raw_num_blocks(&self) -> int4 {
        self.bblocks_get_size()
    }
    fn raw_block(&self, i: int4) -> BlockId {
        self.bblocks_get_block(i)
    }
    fn raw_is_basic(&self, b: BlockId) -> bool {
        self.bblocks_ref().block(b).get_type() == BlockType::Basic
    }
    fn raw_is_entry(&self, b: BlockId) -> bool {
        self.bblocks_ref().block(b).is_entry_point()
    }
    fn raw_start(&self, b: BlockId) -> Address {
        block_get_start(&self.bblocks_ref().arena, b)
    }
    fn raw_size_in(&self, b: BlockId) -> int4 {
        self.bblocks_ref().block(b).size_in()
    }
    fn raw_in(&self, b: BlockId, i: int4) -> BlockId {
        self.bblocks_ref().block(b).get_in(i)
    }
    fn raw_size_out(&self, b: BlockId) -> int4 {
        self.bblocks_ref().block(b).size_out()
    }
    fn raw_out(&self, b: BlockId, i: int4) -> BlockId {
        self.bblocks_ref().block(b).get_out(i)
    }
    fn raw_ops(&self, b: BlockId) -> Vec<OpId> {
        self.bb_ops(b)
    }
    fn raw_tail(&self, b: BlockId) -> Option<OpId> {
        self.bb_op_tail(b)
    }
    fn raw_proto(&self) -> &FuncProto {
        self.get_func_proto()
    }
    fn raw_table_at(&self, addr: &Address) -> bool {
        (0..self.num_jump_tables()).any(|j| self.get_jump_table(j).get_op_address() == addr)
    }
}

/// An op of a [`RawSnapshot`].
pub(crate) struct SnapOp {
    code: OpCode,
    addr: Address,
    out: Option<VarnodeId>,
    num_input: int4,
    inline: [Option<VarnodeId>; 3],
    rest: Vec<Option<VarnodeId>>,
}

impl RawOp for SnapOp {
    fn code(&self) -> OpCode {
        self.code
    }
    fn get_addr(&self) -> &Address {
        &self.addr
    }
    fn get_out(&self) -> Option<VarnodeId> {
        self.out
    }
    fn num_input(&self) -> int4 {
        self.num_input
    }
    fn get_in(&self, slot: int4) -> Option<VarnodeId> {
        match usize::try_from(slot) {
            Ok(i) if i < 3 => self.inline[i],
            Ok(i) => self.rest.get(i - 3).copied().flatten(),
            Err(_) => None,
        }
    }
}

/// A varnode of a [`RawSnapshot`].
pub(crate) struct SnapVn {
    addr: Address,
    size: int4,
    constant: bool,
    written: bool,
    def: Option<OpId>,
}

impl RawVn for SnapVn {
    fn get_addr(&self) -> &Address {
        &self.addr
    }
    fn get_offset(&self) -> uintb {
        self.addr.get_offset()
    }
    fn get_size(&self) -> int4 {
        self.size
    }
    fn is_constant(&self) -> bool {
        self.constant
    }
    fn is_written(&self) -> bool {
        self.written
    }
    fn get_def(&self) -> Option<OpId> {
        self.def
    }
}

struct SnapBlock {
    basic: bool,
    entry: bool,
    start: Address,
    ins: (u32, u32),
    outs: (u32, u32),
    ops: (u32, u32),
}

/// The raw p-code of a function as [`RawIr`] reads it, copied at the install's
/// slot: its basic blocks in order, their edges and ops, every varnode an op
/// reads or writes, the prototype and the jump tables.
pub(crate) struct RawSnapshot {
    order: Vec<BlockId>,
    blocks: SecondaryMap<BlockId, SnapBlock>,
    edges: Vec<BlockId>,
    oplist: Vec<OpId>,
    ops: SecondaryMap<OpId, SnapOp>,
    vns: SecondaryMap<VarnodeId, SnapVn>,
    proto: FuncProto,
    tables: Vec<Address>,
}

impl RawSnapshot {
    /// Copy what [`RawIr`] reads of `data`.
    pub(crate) fn capture(data: &Funcdata) -> RawSnapshot {
        let n = data.bblocks_get_size();
        let mut snap = RawSnapshot {
            order: Vec::with_capacity(n.max(0) as usize),
            blocks: SecondaryMap::new(),
            edges: Vec::new(),
            oplist: Vec::new(),
            ops: SecondaryMap::new(),
            vns: SecondaryMap::new(),
            proto: FuncProto::new(),
            tables: (0..data.num_jump_tables()).map(|j| data.get_jump_table(j).get_op_address().clone()).collect(),
        };
        snap.proto.copy(data.get_func_proto());
        for i in 0..n {
            let b = data.bblocks_get_block(i);
            snap.order.push(b);
            let blk = data.bblocks_ref().block(b);
            let at = snap.edges.len() as u32;
            snap.edges.extend((0..blk.size_in()).map(|k| blk.get_in(k)));
            let ins = (at, snap.edges.len() as u32);
            let at = snap.edges.len() as u32;
            snap.edges.extend((0..blk.size_out()).map(|k| blk.get_out(k)));
            let outs = (at, snap.edges.len() as u32);
            let at = snap.oplist.len() as u32;
            let mut cur = data.bb_op_head(b);
            while let Some(op) = cur {
                snap.oplist.push(op);
                let o = data.obank().get(op).expect("bb_ops");
                snap.copy_op(data, op, o);
                cur = o.basic_neighbours().1;
            }
            let ops = (at, snap.oplist.len() as u32);
            snap.blocks.insert(
                b,
                SnapBlock {
                    basic: blk.get_type() == BlockType::Basic,
                    entry: blk.is_entry_point(),
                    start: block_get_start(&data.bblocks_ref().arena, b),
                    ins,
                    outs,
                    ops,
                },
            );
        }
        snap
    }

    fn copy_op(&mut self, data: &Funcdata, id: OpId, o: &PcodeOp) {
        let num_input = o.num_input();
        let mut inline = [None; 3];
        let mut rest = Vec::new();
        for slot in 0..num_input {
            let vn = o.get_in(slot);
            if slot < 3 {
                inline[slot as usize] = vn;
            } else {
                rest.push(vn);
            }
            if let Some(v) = vn {
                self.copy_vn(data, v);
            }
        }
        let out = o.get_out();
        if let Some(v) = out {
            self.copy_vn(data, v);
        }
        self.ops.insert(id, SnapOp { code: o.code(), addr: o.get_addr().clone(), out, num_input, inline, rest });
    }

    fn copy_vn(&mut self, data: &Funcdata, id: VarnodeId) {
        if self.vns.contains_key(id) {
            return;
        }
        if let Some(v) = data.vbank().get(id) {
            self.vns.insert(
                id,
                SnapVn {
                    addr: v.get_addr().clone(),
                    size: v.get_size(),
                    constant: v.is_constant(),
                    written: v.is_written(),
                    def: v.get_def(),
                },
            );
        }
    }

    fn block(&self, b: BlockId) -> &SnapBlock {
        self.blocks.get(b).expect("RawSnapshot: block not captured")
    }
}

impl RawIr for RawSnapshot {
    type Op = SnapOp;
    type Vn = SnapVn;
    fn raw_op(&self, op: OpId) -> Option<&SnapOp> {
        self.ops.get(op)
    }
    fn raw_vn(&self, vn: VarnodeId) -> Option<&SnapVn> {
        self.vns.get(vn)
    }
    fn raw_num_blocks(&self) -> int4 {
        self.order.len() as int4
    }
    fn raw_block(&self, i: int4) -> BlockId {
        self.order[i as usize]
    }
    fn raw_is_basic(&self, b: BlockId) -> bool {
        self.block(b).basic
    }
    fn raw_is_entry(&self, b: BlockId) -> bool {
        self.block(b).entry
    }
    fn raw_start(&self, b: BlockId) -> Address {
        self.block(b).start.clone()
    }
    fn raw_size_in(&self, b: BlockId) -> int4 {
        let (lo, hi) = self.block(b).ins;
        (hi - lo) as int4
    }
    fn raw_in(&self, b: BlockId, i: int4) -> BlockId {
        self.edges[(self.block(b).ins.0 + i as u32) as usize]
    }
    fn raw_size_out(&self, b: BlockId) -> int4 {
        let (lo, hi) = self.block(b).outs;
        (hi - lo) as int4
    }
    fn raw_out(&self, b: BlockId, i: int4) -> BlockId {
        self.edges[(self.block(b).outs.0 + i as u32) as usize]
    }
    fn raw_ops(&self, b: BlockId) -> Vec<OpId> {
        let (lo, hi) = self.block(b).ops;
        self.oplist[lo as usize..hi as usize].to_vec()
    }
    fn raw_tail(&self, b: BlockId) -> Option<OpId> {
        let (lo, hi) = self.block(b).ops;
        (hi > lo).then(|| self.oplist[hi as usize - 1])
    }
    fn raw_proto(&self) -> &FuncProto {
        &self.proto
    }
    fn raw_table_at(&self, addr: &Address) -> bool {
        self.tables.iter().any(|t| t == addr)
    }
}

/// Whether the function's raw p-code has the two conditional branches a cascade
/// needs, so a snapshot of it can matter.
pub(crate) fn worth_capturing(data: &Funcdata) -> bool {
    let mut cbranches = 0;
    for i in 0..data.bblocks_get_size() {
        let tail = data.bb_op_tail(data.bblocks_get_block(i));
        if tail.and_then(|op| data.obank().get(op)).is_some_and(|o| o.code() == OpCode::CPUI_CBRANCH) {
            cbranches += 1;
            if cbranches >= 2 {
                return true;
            }
        }
    }
    false
}

/// The basic block whose last op is at `addr`.
pub(crate) fn block_with_tail_at<R: RawIr + ?Sized>(raw: &R, addr: &Address) -> Option<BlockId> {
    for i in 0..raw.raw_num_blocks() {
        let bl = raw.raw_block(i);
        if !raw.raw_is_basic(bl) {
            continue;
        }
        if let Some(tail) = raw.raw_tail(bl) {
            if raw.raw_op(tail).map(|o| o.get_addr()) == Some(addr) {
                return Some(bl);
            }
        }
    }
    None
}

/// The basic block whose first address is `addr`.
pub(crate) fn block_at_start<R: RawIr + ?Sized>(raw: &R, addr: &Address) -> Option<BlockId> {
    for i in 0..raw.raw_num_blocks() {
        let bl = raw.raw_block(i);
        if raw.raw_is_basic(bl) && &raw.raw_start(bl) == addr {
            return Some(bl);
        }
    }
    None
}

/// True when the install would strand a recovered jump table: a `BRANCHIND`
/// with a table that no longer reaches from the entry once `head` branches
/// straight to `out_blocks`.
pub(crate) fn strands_table<R: RawIr + ?Sized>(raw: &R, head: BlockId, out_blocks: &[BlockId]) -> bool {
    let size = raw.raw_num_blocks();
    let entry = match (0..size).map(|i| raw.raw_block(i)).find(|&b| raw.raw_is_entry(b)) {
        Some(e) => e,
        None => return false,
    };
    let mut seen: std::collections::HashSet<BlockId> = std::collections::HashSet::new();
    let mut stack: Vec<BlockId> = vec![entry];
    seen.insert(entry);
    while let Some(b) = stack.pop() {
        if b == head {
            for &t in out_blocks {
                if seen.insert(t) {
                    stack.push(t);
                }
            }
            continue;
        }
        for i in 0..raw.raw_size_out(b) {
            let t = raw.raw_out(b, i);
            if seen.insert(t) {
                stack.push(t);
            }
        }
    }
    for i in 0..size {
        let blk = raw.raw_block(i);
        if seen.contains(&blk) {
            continue;
        }
        let Some(op) = raw.raw_tail(blk) else { continue };
        let Some(o) = raw.raw_op(op) else { continue };
        if o.code() == OpCode::CPUI_BRANCHIND && raw.raw_table_at(o.get_addr()) {
            return true;
        }
    }
    false
}

/// Whether [`Funcdata::kuna_install_lowered_switch`] declines `rec` on `raw`
/// before it changes anything: true only for a switch installed straight to its
/// case bodies whose checks fail on the raw p-code. The arguments are the ones
/// [`ActionLowerSwitchInstall`](crate::kuna_loweredswitch::ActionLowerSwitchInstall)
/// passes for a record that has not been installed yet.
pub(crate) fn install_declines<R: RawIr + ?Sized>(raw: &R, rec: &KunaLoweredSwitchRecord) -> bool {
    if !rec.spine.is_empty() {
        return false;
    }
    let read_home = rec.value.reads_home() && rec.load_at.is_none();
    crate::funcdata_block::plan_direct_install(
        raw,
        &rec.branch_addr,
        &rec.var_addr,
        rec.var_size,
        &rec.case_vals,
        &rec.case_targets,
        &rec.default_target,
        !read_home,
        rec.load_at.as_ref(),
        rec.linear,
    )
    .is_none()
}

#[cfg(test)]
#[path = "kuna_switchrestart/tests.rs"]
mod tests;
