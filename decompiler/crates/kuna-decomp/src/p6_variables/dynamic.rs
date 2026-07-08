//! Port of `decompiler/cpp/dynamic.{cc,hh}` (item `w7-s6-dynamic-pack`) — the
//! hash-based addressing of \e dynamic Varnodes ([`DynamicHash`]) and its edge
//! helper ([`ToOpEdge`]).
//!
//! ## Why a hash at all
//!
//! Most Varnodes are pinned in the symbol table by storage address + the
//! address of their defining op.  Temporary registers (and constants) have
//! ephemeral storage, so a \e dynamic Symbol identifies them instead by a hash
//! of the *local data-flow* around the Varnode.  [`DynamicHash::calc_hash`]
//! grows a sub-graph (one of four/three "methods") rooted at the Varnode,
//! folds every edge through CRC32, and packs the 32-bit neighborhood hash
//! together with the root op-code, slot, method, and an attach/collision
//! encoding into a 64-bit value (`dynamic.cc:323` `pieceTogetherHash`).
//! [`DynamicHash::unique_hash`] cycles methods until the hash is unique among
//! same-address Varnodes.
//!
//! ## The hash computation is output-determining — transcribed exactly
//!
//! These 64-bit hashes are *persisted* in the symbol table for dynamic
//! symbols, so every bit — the [`TRANSTABLE`] op-code lumping, the CRC fold
//! order, the [`ToOpEdge`] sort (PcodeOp address, then order, then slot), the
//! field packing in [`DynamicHash::piece_together_hash`], and the
//! collision-resolution iteration in `unique_hash` — must match upstream
//! byte-for-byte.  `uintb`→`u64`, `uint4`→`u32`, `int4`→`i32`; all arithmetic
//! that the C++ leaves to natural wrap is `wrapping_*`; the slot/method/opcode
//! bit fields use exactly the C++ masks.
//!
//! ## ADR 0001 (IR arenas) realization
//!
//! The C++ `DynamicHash` keeps `vector<const PcodeOp *>` / `vector<const
//! Varnode *>`; here those become `Vec<OpId>` / `Vec<VarnodeId>` and every
//! dereference goes through the [`Funcdata`] banks.  The C++ uses the
//! `PcodeOp`/`Varnode` \e mark bit as private scratch state while building the
//! sub-graph (and clears it in `piece_together_hash`); the Rust port threads
//! `&mut Funcdata` through `calc_hash` so it can flip the same marks via
//! `vbank_mut()`/`obank_mut()`.
//!
//! ## (kuna) GH-8467 dynamichashmax
//!
//! `unique_hash` reads [`Architecture::dynamic_hash_maxdup_high`]
//! (`crate::architecture`) to raise the same-address collision budget 8→16; the
//! option that flips that flag lives in `crate::kuna_dynamichashmax`.

use kuna_base::address::Address;
use kuna_base::error::KunaResult;
use kuna_base::types::{int4, uint4, uint8, uintb, uintm, Wrap};
use kuna_base::crc32::crc_update;
use kuna_num::opcodes::OpCode;

use crate::funcdata::Funcdata;
use crate::context::{OpId, VarnodeId};

/// Table for how to hash opcodes; lumps certain operators (e.g. ADD SUB PTRADD
/// PTRSUB) into one hash.  Zero indicates the operator should be skipped
/// (C++ `DynamicHash::transtable[]`, `dynamic.cc:24`).
///
/// Indexed by the raw opcode value `op->code()` (1..=74; index 0 is the unused
/// slot, hashed as 0/"skip").  The stored values are themselves `CPUI_*`
/// opcode integers, so variants collapse: NOTEQUAL hashes as EQUAL, SUB as ADD,
/// PTRADD/PTRSUB as INT_ADD, and CAST is skipped (0).
///
/// Transcribed verbatim from the C++ array; the `OpCode as u32` casts make the
/// equivalence to the C++ `CPUI_*` enum constants explicit.
pub static TRANSTABLE: [u32; 75] = [
    0,
    OpCode::CPUI_COPY as u32,
    OpCode::CPUI_LOAD as u32,
    OpCode::CPUI_STORE as u32,
    OpCode::CPUI_BRANCH as u32,
    OpCode::CPUI_CBRANCH as u32,
    OpCode::CPUI_BRANCHIND as u32,
    OpCode::CPUI_CALL as u32,
    OpCode::CPUI_CALLIND as u32,
    OpCode::CPUI_CALLOTHER as u32,
    OpCode::CPUI_RETURN as u32,
    OpCode::CPUI_INT_EQUAL as u32,
    OpCode::CPUI_INT_EQUAL as u32, // NOT_EQUAL hashes same as EQUAL
    OpCode::CPUI_INT_SLESS as u32,
    OpCode::CPUI_INT_SLESS as u32, // SLESSEQUAL hashes same as SLESS
    OpCode::CPUI_INT_LESS as u32,
    OpCode::CPUI_INT_LESS as u32, // LESSEQUAL hashes same as LESS
    OpCode::CPUI_INT_ZEXT as u32,
    OpCode::CPUI_INT_SEXT as u32,
    OpCode::CPUI_INT_ADD as u32,
    OpCode::CPUI_INT_ADD as u32, // SUB hashes same as ADD
    OpCode::CPUI_INT_CARRY as u32,
    OpCode::CPUI_INT_SCARRY as u32,
    OpCode::CPUI_INT_SBORROW as u32,
    OpCode::CPUI_INT_2COMP as u32,
    OpCode::CPUI_INT_NEGATE as u32,
    OpCode::CPUI_INT_XOR as u32,
    OpCode::CPUI_INT_AND as u32,
    OpCode::CPUI_INT_OR as u32,
    OpCode::CPUI_INT_MULT as u32, // LEFT hashes same as MULT
    OpCode::CPUI_INT_RIGHT as u32,
    OpCode::CPUI_INT_SRIGHT as u32,
    OpCode::CPUI_INT_MULT as u32,
    OpCode::CPUI_INT_DIV as u32,
    OpCode::CPUI_INT_SDIV as u32,
    OpCode::CPUI_INT_REM as u32,
    OpCode::CPUI_INT_SREM as u32,
    OpCode::CPUI_BOOL_NEGATE as u32,
    OpCode::CPUI_BOOL_XOR as u32,
    OpCode::CPUI_BOOL_AND as u32,
    OpCode::CPUI_BOOL_OR as u32,
    OpCode::CPUI_FLOAT_EQUAL as u32,
    OpCode::CPUI_FLOAT_EQUAL as u32, // NOTEQUAL hashes same as EQUAL
    OpCode::CPUI_FLOAT_LESS as u32,
    OpCode::CPUI_FLOAT_LESS as u32, // LESSEQUAL hashes same as LESS
    0,                              // Unused slot - skip
    OpCode::CPUI_FLOAT_NAN as u32,
    OpCode::CPUI_FLOAT_ADD as u32,
    OpCode::CPUI_FLOAT_DIV as u32,
    OpCode::CPUI_FLOAT_MULT as u32,
    OpCode::CPUI_FLOAT_ADD as u32, // SUB hashes same as ADD
    OpCode::CPUI_FLOAT_NEG as u32,
    OpCode::CPUI_FLOAT_ABS as u32,
    OpCode::CPUI_FLOAT_SQRT as u32,
    OpCode::CPUI_FLOAT_INT2FLOAT as u32,
    OpCode::CPUI_FLOAT_FLOAT2FLOAT as u32,
    OpCode::CPUI_FLOAT_TRUNC as u32,
    OpCode::CPUI_FLOAT_CEIL as u32,
    OpCode::CPUI_FLOAT_FLOOR as u32,
    OpCode::CPUI_FLOAT_ROUND as u32,
    OpCode::CPUI_MULTIEQUAL as u32,
    OpCode::CPUI_INDIRECT as u32,
    OpCode::CPUI_PIECE as u32,
    OpCode::CPUI_SUBPIECE as u32,
    0, // CAST is skipped
    OpCode::CPUI_INT_ADD as u32,
    OpCode::CPUI_INT_ADD as u32, // PTRADD and PTRSUB hash same as INT_ADD
    OpCode::CPUI_SEGMENTOP as u32,
    OpCode::CPUI_CPOOLREF as u32,
    OpCode::CPUI_NEW as u32,
    OpCode::CPUI_INSERT as u32,
    OpCode::CPUI_ZPULL as u32,
    OpCode::CPUI_POPCOUNT as u32,
    OpCode::CPUI_LZCOUNT as u32,
    OpCode::CPUI_SPULL as u32,
];

/// Look up the [`TRANSTABLE`] hash value for an op-code (`transtable[op->code()]`).
///
/// Indexed by the raw opcode discriminant; `code` is the result of
/// `op->code()`, always in `1..=74` for a live op.
#[inline]
fn transcode(code: OpCode) -> u32 {
    TRANSTABLE[code as i32 as usize]
}

/// An edge between a Varnode and a PcodeOp (C++ `ToOpEdge`, `dynamic.hh:32`).
///
/// A [`DynamicHash`] is defined on a sub-graph of the data-flow; this is an edge
/// in that sub-graph.  The edge is either from an input Varnode to the PcodeOp
/// that reads it (`slot >= 0`) or from a PcodeOp to the Varnode it defines
/// (`slot == -1`).
///
/// `op` is stored as the [`OpId`]; the `operator<` comparison reads the op's
/// [`SeqNum`] through the [`Funcdata`] banks (the C++ dereferences the raw
/// `const PcodeOp *`).
#[derive(Clone, Copy)]
pub struct ToOpEdge {
    /// The PcodeOp defining the edge (C++ `const PcodeOp *op`).
    op: OpId,
    /// Slot containing the input Varnode or -1 for the p-code op output
    /// (C++ `int4 slot`).
    slot: int4,
}

impl ToOpEdge {
    /// Constructor (C++ `ToOpEdge(const PcodeOp *o,int4 s)`).
    pub fn new(op: OpId, slot: int4) -> ToOpEdge {
        ToOpEdge { op, slot }
    }

    /// Get the PcodeOp defining the edge (C++ `getOp`).
    pub fn get_op(&self) -> OpId {
        self.op
    }

    /// Get the slot of the starting Varnode (C++ `getSlot`).
    pub fn get_slot(&self) -> int4 {
        self.slot
    }

    /// Compare two edges based on PcodeOp (C++ `ToOpEdge::operator<`,
    /// `dynamic.cc:69`).
    ///
    /// Edges are sorted to provide consistency to the hash: first the PcodeOp
    /// sequence-number address, then the order, then the Varnode slot.
    /// ```text
    ///   if (addr1 != addr2) return (addr1 < addr2);
    ///   if (ord1 != ord2)   return (ord1 < ord2);
    ///   return (slot < op2.slot);
    /// ```
    /// `data` resolves both ops' [`SeqNum`]s.
    fn less_than(&self, op2: &ToOpEdge, data: &Funcdata) -> bool {
        let seq1 = data.obank().get(self.op).expect("ToOpEdge::operator<: op").get_seq_num();
        let seq2 = data.obank().get(op2.op).expect("ToOpEdge::operator<: op2").get_seq_num();
        let addr1: &Address = seq1.get_addr();
        let addr2: &Address = seq2.get_addr();
        if addr1 != addr2 {
            return addr1 < addr2;
        }
        let ord1: uintm = seq1.get_order();
        let ord2: uintm = seq2.get_order();
        if ord1 != ord2 {
            return ord1 < ord2;
        }
        self.slot < op2.slot
    }

    /// Hash \b this edge into an accumulator (C++ `ToOpEdge::hash`,
    /// `dynamic.cc:92`).
    ///
    /// Accumulates the Varnode slot, the (translated) op-code, and the address
    /// of the PcodeOp.  The op-code translation makes the hash invariant under
    /// common variants.
    /// ```text
    ///   reg = crc_update(reg,(uint4)slot);
    ///   reg = crc_update(reg,DynamicHash::transtable[op->code()]);
    ///   uintb val = op->getSeqNum().getAddr().getOffset();
    ///   int4 sz   = op->getSeqNum().getAddr().getAddrSize();
    ///   for(int4 i=0;i<sz;++i){ reg = crc_update(reg,(uint4)val); val >>= 8; }
    /// ```
    fn hash(&self, reg: uint4, data: &Funcdata) -> uint4 {
        let op = data.obank().get(self.op).expect("ToOpEdge::hash: op");
        let mut reg = crc_update(reg, self.slot as uint4);
        reg = crc_update(reg, transcode(op.code()));
        let addr = op.get_seq_num().get_addr();
        let mut val: uintb = addr.get_offset();
        let sz: int4 = addr.get_addr_size();
        for _i in 0..sz {
            reg = crc_update(reg, val as uint4); // Hash in the address
            val >>= 8;
        }
        reg
    }
}

/// A hash utility to uniquely identify a temporary Varnode in data-flow
/// (C++ `DynamicHash`, `dynamic.hh:62`).
///
/// See the module docs and the C++ class comment for the four/three hash
/// methods and the 64-bit encoding.  All working state is the C++ member set;
/// the [`Funcdata`] is supplied per-call (not stored) so the marks it flips
/// stay scoped to one hash computation.
#[derive(Default)]
pub struct DynamicHash {
    /// Number of Varnodes processed in the \b markvn list so far (C++ `vnproc`).
    vnproc: uint4,
    /// Number of PcodeOps processed in the \b markop list so far (C++ `opproc`).
    opproc: uint4,
    /// Number of edges processed in the \b opedge list (C++ `opedgeproc`).
    opedgeproc: uint4,

    /// List of PcodeOps in the sub-graph being hashed (C++ `markop`).
    markop: Vec<OpId>,
    /// List of Varnodes in the sub-graph being hashed (C++ `markvn`).
    markvn: Vec<VarnodeId>,
    /// Staging area for Varnodes before formally adding to the sub-graph
    /// (C++ `vnedge`).
    vnedge: Vec<VarnodeId>,
    /// The edges in the sub-graph (C++ `opedge`).
    opedge: Vec<ToOpEdge>,

    /// Address most closely associated with variable (C++ `addrresult`).
    addrresult: Address,
    /// The calculated hash value (C++ `hash`).
    hash: uint8,
}

impl DynamicHash {
    /// Fresh hasher (the C++ default-constructed `DynamicHash`).
    pub fn new() -> DynamicHash {
        DynamicHash::default()
    }

    /// Add in the edge between the given Varnode and its defining PcodeOp
    /// (C++ `buildVnUp`, `dynamic.cc:109`).
    ///
    /// Certain p-code ops (CAST) are effectively ignored so the hash is the
    /// same whether or not they are present.
    fn build_vn_up(&mut self, vn: VarnodeId, data: &Funcdata) {
        let mut vn = vn;
        let op: OpId;
        loop {
            let v = data.vbank().get(vn).expect("buildVnUp: vn");
            if !v.is_written() {
                return;
            }
            let def = v.get_def().expect("buildVnUp: written vn has def");
            let dop = data.obank().get(def).expect("buildVnUp: def op");
            if transcode(dop.code()) != 0 {
                op = def; // Do not ignore this operation
                break;
            }
            vn = dop.get_in(0).expect("buildVnUp: skip op getIn(0)");
        }
        self.opedge.push(ToOpEdge::new(op, -1));
    }

    /// Add in edges between the given Varnode and any PcodeOp that reads it
    /// (C++ `buildVnDown`, `dynamic.cc:125`).
    fn build_vn_down(&mut self, vn: VarnodeId, data: &Funcdata) {
        let insize = self.opedge.len();

        // for(iter=vn->beginDescend();iter!=vn->endDescend();++iter)
        for descop in data.descend_snapshot(vn) {
            let mut op: Option<OpId> = Some(descop);
            let mut tmpvn: VarnodeId = vn;
            // while(transtable[op->code()]==0) { ... }
            while {
                let cur = op.expect("buildVnDown: op present in while-cond");
                transcode(data.obank().get(cur).expect("buildVnDown: op").code()) == 0
            } {
                let cur = op.expect("buildVnDown: op in body");
                match data.obank().get(cur).expect("buildVnDown: op").get_out() {
                    None => {
                        op = None;
                        break;
                    }
                    Some(out) => {
                        tmpvn = out;
                    }
                }
                op = data.lone_descend(tmpvn);
                if op.is_none() {
                    break;
                }
            }
            let op = match op {
                None => continue,
                Some(o) => o,
            };
            let slot = data.obank().get(op).expect("buildVnDown: final op").get_slot(tmpvn);
            self.opedge.push(ToOpEdge::new(op, slot));
        }
        // if ((uint4)opedge.size()-insize > 1) sort(opedge.begin()+insize,opedge.end());
        if self.opedge.len() - insize > 1 {
            // sort_by transcribes ToOpEdge::operator< (a strict weak ordering).
            let tail = &mut self.opedge[insize..];
            // Stable sort over a strict-weak-ordering comparator reproduces the
            // C++ std::sort ordering on distinct keys; the comparator is total
            // on the (addr,order,slot) triple here.
            tail.sort_by(|a, b| {
                if a.less_than(b, data) {
                    std::cmp::Ordering::Less
                } else if b.less_than(a, data) {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Equal
                }
            });
        }
    }

    /// Move input Varnodes for the given PcodeOp into staging
    /// (C++ `buildOpUp`, `dynamic.cc:152`).
    fn build_op_up(&mut self, op: OpId, data: &Funcdata) {
        let o = data.obank().get(op).expect("buildOpUp: op");
        let n = o.num_input();
        for i in 0..n {
            let vn = o.get_in(i).expect("buildOpUp: getIn(i)");
            self.vnedge.push(vn);
        }
    }

    /// Move the output Varnode for the given PcodeOp into staging
    /// (C++ `buildOpDown`, `dynamic.cc:162`).
    fn build_op_down(&mut self, op: OpId, data: &Funcdata) {
        let vn = match data.obank().get(op).expect("buildOpDown: op").get_out() {
            None => return,
            Some(v) => v,
        };
        self.vnedge.push(vn);
    }

    /// Move staged Varnodes into the sub-graph and mark them
    /// (C++ `gatherUnmarkedVn`, `dynamic.cc:170`).
    fn gather_unmarked_vn(&mut self, data: &mut Funcdata) {
        for i in 0..self.vnedge.len() {
            let vn = self.vnedge[i];
            if data.vbank().get(vn).expect("gatherUnmarkedVn: vn").is_mark() {
                continue;
            }
            self.markvn.push(vn);
            data.vbank_mut().get_mut(vn).expect("gatherUnmarkedVn: vn mut").set_mark();
        }
        self.vnedge.clear();
    }

    /// Mark any new PcodeOps in the sub-graph (C++ `gatherUnmarkedOp`,
    /// `dynamic.cc:182`).
    fn gather_unmarked_op(&mut self, data: &mut Funcdata) {
        while (self.opedgeproc as usize) < self.opedge.len() {
            let op = self.opedge[self.opedgeproc as usize].get_op();
            self.opedgeproc = self.opedgeproc.wadd(1);
            if data.obank().get(op).expect("gatherUnmarkedOp: op").is_mark() {
                continue;
            }
            self.markop.push(op);
            data.obank_mut().get_mut(op).expect("gatherUnmarkedOp: op mut").set_mark();
        }
    }

    /// Called for each additional hash (after the first) (C++ `clear`,
    /// `dynamic.cc:193`).
    pub fn clear(&mut self) {
        self.markop.clear();
        self.markvn.clear();
        self.vnedge.clear();
        self.opedge.clear();
    }

    /// Calculate hash for given PcodeOp, slot, and method (C++ `calcHash(const
    /// PcodeOp *,int4,uint4)`, `dynamic.cc:202`).
    pub fn calc_hash_op(&mut self, op: OpId, slot: int4, method: uint4, data: &mut Funcdata) {
        let root: VarnodeId;

        // slot may be from a hash unassociated with op; check it indicates a
        // valid Varnode.
        if slot < 0 {
            match data.obank().get(op).expect("calcHash: op").get_out() {
                None => {
                    self.hash = 0;
                    self.addrresult = Address::default();
                    return; // slot does not fit op
                }
                Some(r) => root = r,
            }
        } else {
            let o = data.obank().get(op).expect("calcHash: op");
            if slot >= o.num_input() {
                self.hash = 0;
                self.addrresult = Address::default();
                return; // slot does not fit op
            }
            root = o.get_in(slot).expect("calcHash: getIn(slot)");
        }
        self.vnproc = 0;
        self.opproc = 0;
        self.opedgeproc = 0;

        self.opedge.push(ToOpEdge::new(op, slot));
        match method {
            4 => {}
            5 => {
                self.gather_unmarked_op(data);
                while (self.opproc as usize) < self.markop.len() {
                    let mop = self.markop[self.opproc as usize];
                    self.build_op_up(mop, data);
                    self.opproc = self.opproc.wadd(1);
                }
                self.gather_unmarked_vn(data);
                while (self.vnproc as usize) < self.markvn.len() {
                    let mvn = self.markvn[self.vnproc as usize];
                    self.build_vn_up(mvn, data);
                    self.vnproc = self.vnproc.wadd(1);
                }
            }
            6 => {
                self.gather_unmarked_op(data);
                while (self.opproc as usize) < self.markop.len() {
                    let mop = self.markop[self.opproc as usize];
                    self.build_op_down(mop, data);
                    self.opproc = self.opproc.wadd(1);
                }
                self.gather_unmarked_vn(data);
                while (self.vnproc as usize) < self.markvn.len() {
                    let mvn = self.markvn[self.vnproc as usize];
                    self.build_vn_down(mvn, data);
                    self.vnproc = self.vnproc.wadd(1);
                }
            }
            _ => {}
        }
        self.piece_together_hash(root, method, data);
    }

    /// Calculate the hash for given Varnode and method (C++ `calcHash(const
    /// Varnode *,uint4)`, `dynamic.cc:268`).
    ///
    /// A sub-graph is formed extending from `root`:
    ///  - Method 0: just immediate p-code ops reading or writing root
    ///  - Method 1: one more level of inputs from method 0
    ///  - Method 2: one more level of outputs from method 0
    ///  - Method 3: inputs and outputs
    pub fn calc_hash(&mut self, root: VarnodeId, method: uint4, data: &mut Funcdata) {
        self.vnproc = 0;
        self.opproc = 0;
        self.opedgeproc = 0;

        self.vnedge.push(root);
        self.gather_unmarked_vn(data);
        // for(uint4 i=vnproc;i<markvn.size();++i) buildVnUp(markvn[i]);
        let mut i = self.vnproc as usize;
        while i < self.markvn.len() {
            let mvn = self.markvn[i];
            self.build_vn_up(mvn, data);
            i += 1;
        }
        // for(;vnproc<markvn.size();++vnproc) buildVnDown(markvn[vnproc]);
        while (self.vnproc as usize) < self.markvn.len() {
            let mvn = self.markvn[self.vnproc as usize];
            self.build_vn_down(mvn, data);
            self.vnproc = self.vnproc.wadd(1);
        }

        match method {
            0 => {}
            1 => {
                self.gather_unmarked_op(data);
                while (self.opproc as usize) < self.markop.len() {
                    let mop = self.markop[self.opproc as usize];
                    self.build_op_up(mop, data);
                    self.opproc = self.opproc.wadd(1);
                }
                self.gather_unmarked_vn(data);
                while (self.vnproc as usize) < self.markvn.len() {
                    let mvn = self.markvn[self.vnproc as usize];
                    self.build_vn_up(mvn, data);
                    self.vnproc = self.vnproc.wadd(1);
                }
            }
            2 => {
                self.gather_unmarked_op(data);
                while (self.opproc as usize) < self.markop.len() {
                    let mop = self.markop[self.opproc as usize];
                    self.build_op_down(mop, data);
                    self.opproc = self.opproc.wadd(1);
                }
                self.gather_unmarked_vn(data);
                while (self.vnproc as usize) < self.markvn.len() {
                    let mvn = self.markvn[self.vnproc as usize];
                    self.build_vn_down(mvn, data);
                    self.vnproc = self.vnproc.wadd(1);
                }
            }
            3 => {
                self.gather_unmarked_op(data);
                while (self.opproc as usize) < self.markop.len() {
                    let mop = self.markop[self.opproc as usize];
                    self.build_op_up(mop, data);
                    self.opproc = self.opproc.wadd(1);
                }
                self.gather_unmarked_vn(data);
                while (self.vnproc as usize) < self.markvn.len() {
                    let mvn = self.markvn[self.vnproc as usize];
                    self.build_vn_down(mvn, data);
                    self.vnproc = self.vnproc.wadd(1);
                }
            }
            _ => {}
        }
        self.piece_together_hash(root, method, data);
    }

    /// Clean-up and piece together formal hash value (C++ `pieceTogetherHash`,
    /// `dynamic.cc:323`).
    ///
    /// Clears the scratch marks, computes the 32-bit neighborhood hash, finds
    /// the op directly attached to `root`, and packs the 64-bit hash.
    fn piece_together_hash(&mut self, root: VarnodeId, method: uint4, data: &mut Funcdata) {
        // Clear our marks
        for i in 0..self.markvn.len() {
            let vn = self.markvn[i];
            data.vbank_mut().get_mut(vn).expect("pieceTogether: clear vn").clear_mark();
        }
        for i in 0..self.markop.len() {
            let op = self.markop[i];
            data.obank_mut().get_mut(op).expect("pieceTogether: clear op").clear_mark();
        }

        if self.opedge.is_empty() {
            self.hash = 0u64;
            self.addrresult = Address::default();
            return;
        }

        let mut reg: uint4 = 0x3ba0fe06; // Calculate the 32-bit hash

        // Hash in information about the root
        let rootv = data.vbank().get(root).expect("pieceTogether: root");
        reg = crc_update(reg, rootv.get_size() as uint4);
        if rootv.is_constant() {
            let mut val: uintb = rootv.get_offset();
            let rsize = rootv.get_size();
            for _i in 0..rsize {
                reg = crc_update(reg, val as uint4);
                val >>= 8;
            }
        }

        for i in 0..self.opedge.len() {
            reg = self.opedge[i].hash(reg, data);
        }

        // Build the final 64-bit hash.  Find op directly attached to -root- (not
        // a skip op).
        let mut op: OpId = self.opedge[0].get_op();
        let mut slot: int4 = 0;
        let mut attachedop = true;
        let mut ct = 0usize;
        while ct < self.opedge.len() {
            op = self.opedge[ct].get_op();
            slot = self.opedge[ct].get_slot();
            let o = data.obank().get(op).expect("pieceTogether: ct op");
            if slot < 0 && o.get_out() == Some(root) {
                break;
            }
            if slot >= 0 && o.get_in(slot) == Some(root) {
                break;
            }
            ct += 1;
        }
        if ct == self.opedge.len() {
            // Everything attached to the root was a skip op; return op that is
            // not attached directly.
            op = self.opedge[0].get_op();
            slot = self.opedge[0].get_slot();
            attachedop = false;
        }

        // 15 bits unused
        let mut hash: uint8 = if attachedop { 0 } else { 1 };
        hash <<= 4;
        hash |= method as uint8; // 4-bits
        hash <<= 7;
        let opcode = data.obank().get(op).expect("pieceTogether: final op").code();
        hash |= transcode(opcode) as uint8; // 7-bits
        hash <<= 5;
        hash |= (slot & 0x1f) as uint8; // 5-bits

        hash <<= 32;
        hash |= reg as uint8; // 32-bits for the neighborhood hash
        self.hash = hash;
        self.addrresult =
            data.obank().get(op).expect("pieceTogether: addr op").get_seq_num().get_addr().clone();
    }

    /// Convert given PcodeOp to a non-skip op by following data-flow
    /// (C++ `moveOffSkip`, `dynamic.cc:389`).
    ///
    /// Returns the (possibly advanced) `(op, slot)` pair, or `None` for `op`
    /// when the data-flow path ends.
    fn move_off_skip(mut op: OpId, mut slot: int4, data: &Funcdata) -> (Option<OpId>, int4) {
        while transcode(data.obank().get(op).expect("moveOffSkip: op").code()) == 0 {
            if slot >= 0 {
                // C++ `const Varnode *vn = op->getOut(); op = vn->loneDescend();`
                // dereferences `getOut()` unconditionally (a skip op always has
                // an output, so this is reachable only on a well-formed op).
                // ADR 0004: an absent output is C++ UB here, so panic.
                let vn = data
                    .obank()
                    .get(op)
                    .expect("moveOffSkip: op")
                    .get_out()
                    .expect("moveOffSkip: skip op has no output (C++ UB)");
                match data.lone_descend(vn) {
                    None => return (None, slot), // Indicate the end of the data-flow path
                    Some(next) => {
                        op = next;
                        slot = data.obank().get(op).expect("moveOffSkip: next").get_slot(vn);
                    }
                }
            } else {
                let vn = data.obank().get(op).expect("moveOffSkip: op").get_in(0).expect("moveOffSkip: getIn(0)");
                let v = data.vbank().get(vn).expect("moveOffSkip: vn");
                if !v.is_written() {
                    return (Some(op), slot); // Indicate the end of the data-flow path
                }
                op = v.get_def().expect("moveOffSkip: written vn def");
            }
        }
        (Some(op), slot)
    }

    /// Select a unique hash for the given Varnode (C++ `uniqueHash(const
    /// Varnode *,Funcdata *)`, `dynamic.cc:424`).
    ///
    /// Collect the set of Varnodes at the same address as `root`; starting at
    /// method 0, increment the method and calculate hashes until `root` has a
    /// unique hash within the set.  The collision total and position are
    /// encoded when no method yields full uniqueness.
    ///
    /// (kuna GH-8467) the same-address collision budget `maxduplicates` is 8 by
    /// default and 16 when `dynamichashmax` is on.  The C++ reads the flag
    /// `fd->getArch()->dynamic_hash_maxdup_high` inline; the seam
    /// [`Funcdata::get_arch`] does not carry kuna analysis flags (STUB(W4) — the
    /// same gate-threading the sibling kuna modules use), so the resolved
    /// budget is passed in.  Use
    /// [`DynamicHashMaxOption::max_duplicates`](crate::kuna_dynamichashmax::DynamicHashMaxOption::max_duplicates)
    /// (8 off / 16 on) at the call site.
    pub fn unique_hash(&mut self, root: VarnodeId, maxduplicates: uint4, data: &mut Funcdata) {
        let mut champion: Vec<VarnodeId> = Vec::new();
        let mut tmphash: uint8 = 0;
        let mut tmpaddr: Address = Address::default();

        let mut method: uint4 = 0;
        while method < 4 {
            self.clear();
            self.calc_hash(root, method, data);
            if self.hash == 0 {
                return; // Can't get a good hash
            }
            tmphash = self.hash;
            tmpaddr = self.addrresult.clone();
            let mut vnlist: Vec<VarnodeId> = Vec::new();
            let mut vnlist2: Vec<VarnodeId> = Vec::new();
            Self::gather_first_level_vars(&mut vnlist, data, &tmpaddr, tmphash);
            for tmpvn in vnlist.iter().copied() {
                self.clear();
                self.calc_hash(tmpvn, method, data);
                if Self::get_comparable(self.hash) == Self::get_comparable(tmphash) {
                    // Hash collision
                    vnlist2.push(tmpvn);
                    if vnlist2.len() as uint4 > maxduplicates {
                        break;
                    }
                }
            }
            if vnlist2.len() as uint4 <= maxduplicates
                && (champion.is_empty() || vnlist2.len() < champion.len())
            {
                champion = vnlist2;
                if champion.len() == 1 {
                    break; // Current hash is unique
                }
            }
            method = method.wadd(1);
        }
        if champion.is_empty() {
            self.hash = 0u64;
            self.addrresult = Address::default(); // Couldn't find a unique hash
            return;
        }
        let total: uint4 = champion.len() as uint4 - 1; // in range [0,maxduplicates-1]
        let mut pos: uint4 = 0;
        while pos <= total {
            if champion[pos as usize] == root {
                break;
            }
            pos = pos.wadd(1);
        }
        if pos > total {
            self.hash = 0u64;
            self.addrresult = Address::default();
            return;
        }
        self.hash = tmphash | ((pos as uint8) << 49); // three bits for position
        self.hash |= (total as uint8) << 52; // three bits for total
        self.addrresult = tmpaddr;
    }

    /// Select unique hash for given PcodeOp and slot (C++ `uniqueHash(const
    /// PcodeOp *,int4,Funcdata *)`, `dynamic.cc:491`).
    pub fn unique_hash_op(&mut self, op: OpId, slot: int4, maxduplicates: uint4, data: &mut Funcdata) {
        let mut champion: Vec<OpId> = Vec::new();
        let mut tmphash: uint8 = 0;
        let mut tmpaddr: Address = Address::default();
        // (kuna) GH-8467: see unique_hash above -- same wider budget, passed in.

        let (op_opt, slot) = Self::move_off_skip(op, slot, data);
        let op = match op_opt {
            None => {
                self.hash = 0u64;
                self.addrresult = Address::default(); // Hash cannot be calculated
                return;
            }
            Some(o) => o,
        };
        let op_addr = data.obank().get(op).expect("uniqueHashOp: op").get_addr().clone();
        let mut oplist: Vec<OpId> = Vec::new();
        Self::gather_ops_at_address(&mut oplist, data, &op_addr);
        let mut method: uint4 = 4;
        while method < 7 {
            self.clear();
            self.calc_hash_op(op, slot, method, data);
            if self.hash == 0 {
                return; // Can't get a good hash
            }
            tmphash = self.hash;
            tmpaddr = self.addrresult.clone();
            // Faithful transcription of an upstream quirk (`dynamic.cc:519`):
            // `oplist` was filled by gatherOpsAtAddress BEFORE the method loop,
            // but the loop body re-runs `oplist.clear()` every iteration, so the
            // inner `for(i<oplist.size())` always iterates over the *cleared*
            // container -- the collision scan never executes.  Transcribed as-is
            // (C++ is spec); the net effect is `champion` stays empty and the op
            // form of uniqueHash returns hash==0.  (`oplist2` is fresh per
            // iteration here, matching the C++ `.clear()` at loop top.)
            oplist.clear();
            let mut oplist2: Vec<OpId> = Vec::new();
            for tmpop in oplist.iter().copied() {
                if slot >= data.obank().get(tmpop).expect("uniqueHashOp: tmpop").num_input() {
                    continue;
                }
                self.clear();
                self.calc_hash_op(tmpop, slot, method, data);
                if Self::get_comparable(self.hash) == Self::get_comparable(tmphash) {
                    oplist2.push(tmpop);
                    if oplist2.len() as uint4 > maxduplicates {
                        break;
                    }
                }
            }
            if oplist2.len() as uint4 <= maxduplicates
                && (champion.is_empty() || oplist2.len() < champion.len())
            {
                champion = oplist2;
                if champion.len() == 1 {
                    break; // Current hash is unique
                }
            }
            method = method.wadd(1);
        }
        if champion.is_empty() {
            self.hash = 0u64;
            self.addrresult = Address::default(); // Couldn't find a unique hash
            return;
        }
        let total: uint4 = champion.len() as uint4 - 1;
        let mut pos: uint4 = 0;
        while pos <= total {
            if champion[pos as usize] == op {
                break;
            }
            pos = pos.wadd(1);
        }
        if pos > total {
            self.hash = 0u64;
            self.addrresult = Address::default();
            return;
        }
        self.hash = tmphash | ((pos as uint8) << 49);
        self.hash |= (total as uint8) << 52;
        self.addrresult = tmpaddr;
    }

    /// Get the (current) hash (C++ `getHash`).
    pub fn get_hash(&self) -> uint8 {
        self.hash
    }

    /// Get the (current) address (C++ `getAddress`).
    pub fn get_address(&self) -> &Address {
        &self.addrresult
    }

    /// Given an address and hash, find the unique matching Varnode
    /// (C++ `findVarnode`, `dynamic.cc:571`).
    ///
    /// The method, number of collisions, and position are pulled from the hash;
    /// hashes are recomputed at Varnodes linked to `addr`, and the Varnode that
    /// matches (hash and position) is returned.  If the collision count does not
    /// match, returns `None` even if the position looks valid.
    pub fn find_varnode(&mut self, data: &mut Funcdata, addr: &Address, h: uint8) -> Option<VarnodeId> {
        let method = Self::get_method_from_hash(h);
        let total = Self::get_total_from_hash(h);
        let pos = Self::get_position_from_hash(h);
        let mut h = h;
        Self::clear_total_position(&mut h);
        let mut vnlist: Vec<VarnodeId> = Vec::new();
        let mut vnlist2: Vec<VarnodeId> = Vec::new();
        Self::gather_first_level_vars(&mut vnlist, data, addr, h);
        for tmpvn in vnlist.iter().copied() {
            self.clear();
            self.calc_hash(tmpvn, method, data);
            if Self::get_comparable(self.hash) == Self::get_comparable(h) {
                vnlist2.push(tmpvn);
            }
        }
        if total != vnlist2.len() as uint4 {
            return None;
        }
        Some(vnlist2[pos as usize])
    }

    /// Given an address and hash, find the unique matching PcodeOp
    /// (C++ `findOp`, `dynamic.cc:603`).
    pub fn find_op(&mut self, data: &mut Funcdata, addr: &Address, h: uint8) -> Option<OpId> {
        // C++ uses `int` for these four (signed); transcribed as i32.
        let method = Self::get_method_from_hash(h);
        let slot = Self::get_slot_from_hash(h);
        let total = Self::get_total_from_hash(h) as int4;
        let pos = Self::get_position_from_hash(h) as int4;
        let mut h = h;
        Self::clear_total_position(&mut h);
        let mut oplist: Vec<OpId> = Vec::new();
        let mut oplist2: Vec<OpId> = Vec::new();
        Self::gather_ops_at_address(&mut oplist, data, addr);
        for tmpop in oplist.iter().copied() {
            if slot >= data.obank().get(tmpop).expect("findOp: tmpop").num_input() {
                continue;
            }
            self.clear();
            self.calc_hash_op(tmpop, slot, method, data);
            if Self::get_comparable(self.hash) == Self::get_comparable(h) {
                oplist2.push(tmpop);
            }
        }
        if total != oplist2.len() as int4 {
            return None;
        }
        Some(oplist2[pos as usize])
    }

    /// Remove any duplicate Varnodes in given list, preserving order
    /// (C++ `dedupVarnodes`, `dynamic.cc:629`).
    fn dedup_varnodes(varlist: &mut Vec<VarnodeId>, data: &mut Funcdata) {
        if varlist.len() < 2 {
            return;
        }
        let mut res_list: Vec<VarnodeId> = Vec::new();
        for vn in varlist.iter().copied() {
            if !data.vbank().get(vn).expect("dedup: vn").is_mark() {
                data.vbank_mut().get_mut(vn).expect("dedup: vn mut").set_mark();
                res_list.push(vn);
            }
        }
        for vn in res_list.iter().copied() {
            data.vbank_mut().get_mut(vn).expect("dedup: clear").clear_mark();
        }
        std::mem::swap(varlist, &mut res_list);
    }

    /// Get the Varnodes immediately attached to PcodeOps at the given address
    /// (C++ `gatherFirstLevelVars`, `dynamic.cc:655`).
    ///
    /// Filters by the op-code, slot, and attachment boolean encoded in the hash.
    pub fn gather_first_level_vars(
        varlist: &mut Vec<VarnodeId>,
        data: &mut Funcdata,
        addr: &Address,
        h: uint8,
    ) {
        let opc_val = Self::get_op_code_from_hash(h);
        let slot = Self::get_slot_from_hash(h);
        let isnotattached = Self::get_is_not_attached(h);
        // Snapshot the optree range so the immutable borrow ends before the
        // mutable dedup pass (C++ holds a live const_iterator).
        let ops: Vec<OpId> = data.obank().iter_at(addr).map(|(_, id)| id).collect();

        for op in ops {
            if data.obank().get(op).expect("gatherFirst: op").is_dead() {
                continue;
            }
            if transcode(data.obank().get(op).expect("gatherFirst: op").code()) != opc_val {
                continue;
            }
            if slot < 0 {
                let vn = data.obank().get(op).expect("gatherFirst: op").get_out();
                if let Some(mut vn) = vn {
                    if isnotattached {
                        // If original varnode was not attached to (this) op
                        if let Some(ld) = data.lone_descend(vn) {
                            if transcode(data.obank().get(ld).expect("gatherFirst: ld").code()) == 0 {
                                // Check for skipped op
                                match data.obank().get(ld).expect("gatherFirst: ld out").get_out() {
                                    None => continue,
                                    Some(o) => vn = o,
                                }
                            }
                        }
                    }
                    varlist.push(vn);
                }
            } else if slot < data.obank().get(op).expect("gatherFirst: op").num_input() {
                let mut vn = data.obank().get(op).expect("gatherFirst: op").get_in(slot).expect("gatherFirst: getIn");
                if isnotattached {
                    if let Some(def) = data.vbank().get(vn).expect("gatherFirst: vn").get_def() {
                        if transcode(data.obank().get(def).expect("gatherFirst: def").code()) == 0 {
                            vn = data.obank().get(def).expect("gatherFirst: def in").get_in(0).expect("gatherFirst: def getIn(0)");
                        }
                    }
                }
                varlist.push(vn);
            }
        }
        Self::dedup_varnodes(varlist, data);
    }

    /// Place all PcodeOps at the given address in the provided container
    /// (C++ `gatherOpsAtAddress`, `dynamic.cc:702`).
    pub fn gather_ops_at_address(op_list: &mut Vec<OpId>, data: &Funcdata, addr: &Address) {
        for (_seq, op) in data.obank().iter_at(addr) {
            if data.obank().get(op).expect("gatherOps: op").is_dead() {
                continue;
            }
            op_list.push(op);
        }
    }

    /// Retrieve the encoded slot from a hash (C++ `getSlotFromHash`,
    /// `dynamic.cc:717`).  Returns the slot index or -1 if attached as output.
    pub fn get_slot_from_hash(h: uint8) -> int4 {
        let mut res: int4 = ((h >> 32) & 0x1f) as int4;
        if res == 31 {
            res = -1;
        }
        res
    }

    /// Retrieve the encoded method from a hash (C++ `getMethodFromHash`).
    pub fn get_method_from_hash(h: uint8) -> uint4 {
        ((h >> 44) & 0xf) as uint4
    }

    /// Retrieve the encoded op-code from a hash (C++ `getOpCodeFromHash`).
    pub fn get_op_code_from_hash(h: uint8) -> uint4 {
        ((h >> 37) & 0x7f) as uint4
    }

    /// Retrieve the encoded position from a hash (C++ `getPositionFromHash`).
    pub fn get_position_from_hash(h: uint8) -> uint4 {
        ((h >> 49) & 7) as uint4
    }

    /// Retrieve the encoded collision total from a hash (C++ `getTotalFromHash`).
    pub fn get_total_from_hash(h: uint8) -> uint4 {
        (((h >> 52) & 7) as uint4).wadd(1)
    }

    /// Retrieve the attachment boolean from a hash (C++ `getIsNotAttached`).
    pub fn get_is_not_attached(h: uint8) -> bool {
        ((h >> 48) & 1) != 0
    }

    /// Clear the collision total and position fields within a hash
    /// (C++ `clearTotalPosition`).
    pub fn clear_total_position(h: &mut uint8) {
        let mut val: uint8 = 0x3f;
        val <<= 49;
        val = !val;
        *h &= val;
    }

    /// Get only the formal hash for comparing (C++ `getComparable`).
    pub fn get_comparable(h: uint8) -> uint4 {
        h as uint4
    }
}

/// Convenience: a one-shot unique hash for a Varnode.  Returns
/// `(hash, address)` (C++ idiom: construct, `uniqueHash`, read `getHash`/`getAddress`).
///
/// `maxduplicates` is the same-address collision budget (8 upstream / 16 under
/// `dynamichashmax`); see [`DynamicHash::unique_hash`].
pub fn dynamic_unique_hash(
    root: VarnodeId,
    maxduplicates: uint4,
    data: &mut Funcdata,
) -> KunaResult<(uint8, Address)> {
    let mut dh = DynamicHash::new();
    dh.unique_hash(root, maxduplicates, data);
    Ok((dh.get_hash(), dh.get_address().clone()))
}

#[cfg(test)]
#[path = "dynamic/tests.rs"]
mod tests;
