//! Port of `decompiler/cpp/jumptable.{cc,hh}` (W3, item `w3-ir-jumptable`):
//! jump-table (switch) recovery.
//!
//! This is a faithful transcription of upstream's switch-recovery machinery
//! against the W3 IR surface (slotmap arenas mediated by [`Funcdata`], ADR
//! 0001).  The structurally-complete parts are ported and tested directly:
//!
//!   - [`LoadTable`] (incl. `collapseTable` and `encode`/`decode`),
//!   - [`PcodeOpNode`] (the data-flow edge from `expression.hh`),
//!   - [`PathMeld`] (the path-intersection container: `set`/`append`/`meld`/
//!     `meldOps`/`internalIntersect`/`truncatePaths`/`markPaths`/`getEarliestOp`/
//!     `isLoadInPath`),
//!   - [`GuardRecord`] static logic (`oneOffMatch`, `quasiCopy`, `valueMatch`),
//!   - the [`JumpValues`] iterator family ([`JumpValuesRange`],
//!     [`JumpValuesRangeDefault`]),
//!   - [`JumpModelTrivial`] and the [`JumpModel`] trait surface,
//!   - the [`JumpBasic`] pure static helpers (`isprune`, `ispoint`, `getStride`,
//!     `getMaxValue`, `duplicateVarnodes`, `findDeterminingVarnodes`),
//!   - [`JumpTable`] bookkeeping (`IndexPair`, `switchOver`, `trivialSwitchOver`,
//!     `block2Position`, `isReachable`, the `sanityCheck` control logic, the
//!     two-phase `recoverAddresses`/`matchModel`/`recoverLabels` driver,
//!     `encode`/`decode`).
//!
//! ## Genuinely-absent dependencies (seam-noted; see structured losses)
//!
//! Several pieces depend on subsystems that are not yet ported.  Per the W3
//! task rule (*"Where SSA/heritage context (W5) is genuinely required: faithful
//! algorithm port against the existing IR surface, explicit Err seam notes +
//! losses for the unreachable-now paths"*) these surfaces are present as method
//! shells that return a precise `Err`, and are recorded as losses:
//!
//!   - **`CircleRange`** — extracted to `kuna-num` per its lib doc but *not yet
//!     implemented anywhere*.  [`GuardRecord`] needs it as its `range` member,
//!     and `calcRange`/`analyzeGuards`/`pullBack` are built on its
//!     `pullBack`/`intersect`/`contains`/`getNext` API.  Modeled here as a
//!     local placeholder [`CircleRange`] carrying only the fields the *ported*
//!     code constructs; the value-set operators are `// SEAM(W5)` shells.
//!   - **`EmulateFunction`/`EmulatePcodeOp`** — emulation over the syntax tree
//!     (LOSS-023: `EmulatePcodeOp` deferred to this wave but needs `glb->userops`
//!     and `glb->loader`, which are W4 [`Architecture`] slices not in the seam);
//!     `buildAddresses`/`emulatePath`/`trialNorm` are `// SEAM(W4)` shells.
//!   - **`MemoryImage`/loader reads** — `backup2Switch`/`findNormalized`/
//!     `sanityCheck` read the LoadImage (`glb->loader`), W4.
//!   - **`TypeOp::recoverInputBinary`/`recoverInputUnary`** — `backup2Switch`
//!     reverse emulation, W6 (`typeop`).
//!   - **block-graph SSA helpers** — `BlockBasic::findMultiequal`,
//!     `liftVerifyUnroll`, `noInterveningStatement`, `FlowBlock::getImmedDom`,
//!     `Funcdata::pushBranch`, the `Override`/restart machinery, and
//!     `DynamicHash` are W5/structuring/W4.
//!   - **`JumpAssisted`** — the `jumpassist` model is entirely W4
//!     (`pcodeinjectlib`/`userops`/`ExecutablePcode`); ported as a shell.
//!   - **arch config** — `switch_modulo_bound`, `funcptr_align`,
//!     `max_jumptable_size` are W4 [`Architecture`] fields not in the seam.
//!
//! Integer semantics follow ADR 0003 (`uintb -> u64`, `intb -> i64`, explicit
//! wrapping); containers follow ADR 0002 (`BTreeSet` for the override address
//! set, transcribing the `set<Address>` order).

use std::rc::Rc;

use kuna_base::address::{coveringmask, count_leading_zeros, mostsigbit_set, Address};
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::marshal::{
    AttributeId, Decoder, ElementId, Encoder, ATTRIB_FORMAT, ATTRIB_SIZE,
};
use kuna_base::space::AddrSpace;
use kuna_base::types::{int4, uint4, uint8, uintb, Wrap};
use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;

use crate::block::block_get_start;
use crate::funcdata::Funcdata;
use crate::kuna_emulatefunction::EmulateFunction;
use crate::seams::{BlockId, OpId, VarnodeId};

// ---------------------------------------------------------------------------
// Marshaling identifiers (jumptable.cc:24-33)
// ---------------------------------------------------------------------------

/// Marshaling attribute "label" (C++ `ATTRIB_LABEL`, `jumptable.cc:24`).
pub const ATTRIB_LABEL: AttributeId = AttributeId::new("label", 131);
/// Marshaling attribute "num" (C++ `ATTRIB_NUM`, `jumptable.cc:25`).
pub const ATTRIB_NUM: AttributeId = AttributeId::new("num", 132);

/// Marshaling element \<basicoverride> (C++ `ELEM_BASICOVERRIDE`).
pub const ELEM_BASICOVERRIDE: ElementId = ElementId::new("basicoverride", 211);
/// Marshaling element \<dest> (C++ `ELEM_DEST`).
pub const ELEM_DEST: ElementId = ElementId::new("dest", 212);
/// Marshaling element \<jumptable> (C++ `ELEM_JUMPTABLE`).
pub const ELEM_JUMPTABLE: ElementId = ElementId::new("jumptable", 213);
/// Marshaling element \<loadtable> (C++ `ELEM_LOADTABLE`).
pub const ELEM_LOADTABLE: ElementId = ElementId::new("loadtable", 214);
/// Marshaling element \<normaddr> (C++ `ELEM_NORMADDR`).
pub const ELEM_NORMADDR: ElementId = ElementId::new("normaddr", 215);
/// Marshaling element \<normhash> (C++ `ELEM_NORMHASH`).
pub const ELEM_NORMHASH: ElementId = ElementId::new("normhash", 216);
/// Marshaling element \<startval> (C++ `ELEM_STARTVAL`).
pub const ELEM_STARTVAL: ElementId = ElementId::new("startval", 217);

/// Jump-table label reserved to indicate \e no \e label
/// (C++ `JumpValues::NO_LABEL`, `jumptable.cc:35`).
pub const NO_LABEL: uint8 = 0xBAD1ABE1BAD1ABE1;

// ---------------------------------------------------------------------------
// CircleRange (now the real port — rangeutil.rs, item w6-s5-rangeutil)
// ---------------------------------------------------------------------------

// `CircleRange` is the circular value-set domain that `GuardRecord`/the
// jump-table normalization analysis are built on.  It used to be a local
// placeholder here (only the constructor/accessor surface, value-set operators
// were `Err` shells); w6-s5-rangeutil ports it completely, so it is now
// re-exported from [`crate::rangeutil`].  The faithful API names differ
// slightly from the old shims: `new_value`/`new`/`new_bool` (vs the old
// `from_value`/`from_range`/`from_bool`) and `contains_val`/`get_next`/
// `intersect` return plain values rather than the old `KunaResult` shells.
pub use crate::rangeutil::CircleRange;

// ---------------------------------------------------------------------------
// PcodeOpNode (expression.hh:28 -- a data-flow edge)
// ---------------------------------------------------------------------------

/// An edge in a data-flow path or graph (C++ `PcodeOpNode`, `expression.hh:28`).
///
/// A minimal node for traversing expressions in the data-flow.  `expression.hh`
/// is W5; only this 2-field edge is needed by jumptable's path enumeration, so
/// it is transcribed locally with the IR keys.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PcodeOpNode {
    /// The p-code end-point of the edge (C++ `PcodeOpNode::op`).
    pub op: OpId,
    /// Slot indicating the input Varnode end-point of the edge
    /// (C++ `PcodeOpNode::slot`).
    pub slot: int4,
}

impl PcodeOpNode {
    /// Construct an edge (C++ `PcodeOpNode(PcodeOp *o,int4 s)`).
    pub fn new(op: OpId, slot: int4) -> PcodeOpNode {
        PcodeOpNode { op, slot }
    }
}

// ---------------------------------------------------------------------------
// LoadTable (jumptable.hh:50, jumptable.cc:38-112)
// ---------------------------------------------------------------------------

/// A description where and how data was loaded from memory
/// (C++ `LoadTable`, `jumptable.hh:50`).
///
/// A generic table description: starting address of the table, the size of an
/// entry, and number of entries.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadTable {
    /// Starting address of table (C++ `LoadTable::addr`).
    pub addr: Address,
    /// Size of table entry (C++ `LoadTable::size`).
    pub size: int4,
    /// Number of entries in table (C++ `LoadTable::num`).
    pub num: int4,
}

impl LoadTable {
    /// Construct a single-entry table (C++ `LoadTable(const Address &ad,int4 sz)`).
    pub fn single(addr: Address, size: int4) -> LoadTable {
        LoadTable { addr, size, num: 1 }
    }

    /// Construct a full table (C++ `LoadTable(const Address &ad,int4 sz,int4 nm)`).
    pub fn full(addr: Address, size: int4, num: int4) -> LoadTable {
        LoadTable { addr, size, num }
    }

    /// Encode a description of \b this as a \<loadtable> element
    /// (C++ `LoadTable::encode`, `jumptable.cc:38`).
    pub fn encode(&self, encoder: &mut dyn Encoder) -> KunaResult<()> {
        encoder.open_element(&ELEM_LOADTABLE);
        encoder.write_signed_integer(&ATTRIB_SIZE, self.size as i64);
        encoder.write_signed_integer(&ATTRIB_NUM, self.num as i64);
        self.addr.encode(encoder)?;
        encoder.close_element(&ELEM_LOADTABLE);
        Ok(())
    }

    /// Decode \b this table from a \<loadtable> element
    /// (C++ `LoadTable::decode`, `jumptable.cc:49`).
    pub fn decode(&mut self, decoder: &mut dyn Decoder) -> KunaResult<()> {
        let elem_id = decoder.open_element_id(&ELEM_LOADTABLE)?;
        self.size = decoder.read_signed_integer_id(&ATTRIB_SIZE)? as int4;
        self.num = decoder.read_signed_integer_id(&ATTRIB_NUM)? as int4;
        self.addr = Address::decode(decoder)?;
        decoder.close_element(elem_id)?;
        Ok(())
    }

    /// Sort the entries and collapse any contiguous sequences into a single
    /// `LoadTable` entry (C++ `LoadTable::collapseTable`, `jumptable.cc:61`).
    pub fn collapse_table(table: &mut Vec<LoadTable>) {
        if table.is_empty() {
            return;
        }

        // Test if the table is already sorted and contiguous entries
        let mut issorted = true;
        let mut num = table[0].num;
        let size = table[0].size;
        // nextaddr = table[0].addr + size
        let mut nextaddr = &table[0].addr + (size as i64);

        for entry in table.iter().skip(1) {
            if entry.addr == nextaddr && entry.size == size {
                num += entry.num;
                nextaddr = &entry.addr + (entry.size as i64);
            } else {
                issorted = false;
                break;
            }
        }
        if issorted {
            // Table is sorted and contiguous.
            table.truncate(1); // Truncate everything but the first entry
            table[0].num = num;
            return;
        }

        // sort(table.begin(),table.end()) -- LoadTable::operator< compares by addr
        table.sort_by(|a, b| a.addr.cmp(&b.addr));

        // The C++ uses a two-iterator compaction (lastiter / iter). We mirror it
        // with explicit indices over the now-sorted vector, writing into the same
        // vector in place exactly as upstream does.
        let mut count = 1usize;
        let mut last = 0usize;
        // nextaddr = table[0].addr + table[0].size * table[0].num
        let mut nextaddr =
            &table[0].addr + ((table[0].size as i64).wrapping_mul(table[0].num as i64));
        let mut i = 1usize;
        while i < table.len() {
            if table[i].addr == nextaddr && table[i].size == table[last].size {
                table[last].num += table[i].num;
                nextaddr =
                    &table[i].addr + ((table[i].size as i64).wrapping_mul(table[i].num as i64));
            } else if nextaddr < table[i].addr || table[i].size != table[last].size {
                // Starting a new table
                last += 1;
                table[last] = table[i].clone();
                nextaddr =
                    &table[i].addr + ((table[i].size as i64).wrapping_mul(table[i].num as i64));
                count += 1;
            }
            i += 1;
        }
        // table.resize(count, LoadTable(nextaddr,0)) -- the fill element is only
        // used if count grows, which it cannot here (count <= original len), so
        // truncation is the faithful net effect.
        table.truncate(count);
    }
}

// ---------------------------------------------------------------------------
// PathMeld (jumptable.hh:72, jumptable.cc:795-1047)
// ---------------------------------------------------------------------------

/// A PcodeOp in the path set associated with the last Varnode in the
/// intersection (C++ `PathMeld::RootedOp`, `jumptable.hh:77`).
///
/// Links a PcodeOp to the point where the flow path to it split from the common
/// path.  `op == None` models the C++ `(PcodeOp *)0` "op split but did not
/// rejoin" sentinel.
#[derive(Debug, Clone, Copy)]
struct RootedOp {
    /// An op in the container (C++ `RootedOp::op`; `None` == split-and-dropped).
    op: Option<OpId>,
    /// The index, within `commonVn`, of the Varnode at the split point
    /// (C++ `RootedOp::rootVn`).
    root_vn: int4,
}

/// All paths from a (putative) switch variable to the `CPUI_BRANCHIND`
/// (C++ `PathMeld`, `jumptable.hh:72`).
///
/// Container for intersecting paths during the construction of a `JumpModel`.
/// Holds every PcodeOp from some starting Varnode through all paths to a
/// specific BRANCHIND.  Paths can split and rejoin.  Also tracks Varnodes
/// present on \e all paths -- the potential switch variables.
#[derive(Debug, Clone, Default)]
pub struct PathMeld {
    /// Varnodes in common with all paths (C++ `PathMeld::commonVn`).
    common_vn: Vec<VarnodeId>,
    /// All the ops for the melded paths (C++ `PathMeld::opMeld`).
    op_meld: Vec<RootedOp>,
}

impl PathMeld {
    /// Construct an empty container.
    pub fn new() -> PathMeld {
        PathMeld { common_vn: Vec::new(), op_meld: Vec::new() }
    }

    /// Calculate intersection of a new (marked) Varnode path with the old path
    /// (C++ `PathMeld::internalIntersect`, `jumptable.cc:795`).
    ///
    /// The new path of Varnodes must all be \e marked.  `commonVn` is replaced
    /// with the intersection, and `parentMap` is filled mapping each old index
    /// to its new index (-1 if cut out).
    fn internal_intersect(&mut self, fd: &mut Funcdata, parent_map: &mut Vec<int4>) {
        let mut new_vn: Vec<VarnodeId> = Vec::new();
        let mut last_intersect: int4;
        for &vn in self.common_vn.iter() {
            // Look for previously marked varnode, so we know it is in both lists
            if fd.vbank().get(vn).map(|v| v.is_mark()).unwrap_or(false) {
                last_intersect = new_vn.len() as int4;
                parent_map.push(last_intersect);
                new_vn.push(vn);
                fd.vbank_mut().get_mut(vn).unwrap().clear_mark();
            } else {
                parent_map.push(-1);
            }
        }
        self.common_vn = new_vn;
        last_intersect = -1;
        for i in (0..parent_map.len()).rev() {
            let val = parent_map[i];
            if val == -1 {
                // Fill in varnodes that are cut out of intersection with next
                // earliest varnode that is in intersection
                parent_map[i] = last_intersect;
            } else {
                last_intersect = val;
            }
        }
    }

    /// Meld in PcodeOps from a new path into \b this container
    /// (C++ `PathMeld::meldOps`, `jumptable.cc:833`).
    ///
    /// Execution order of the PcodeOps is maintained.  Returns the index of the
    /// last (earliest) Varnode in the common path or -1.
    fn meld_ops(
        &mut self,
        fd: &Funcdata,
        path: &[PcodeOpNode],
        cut_off: int4,
        parent_map: &[int4],
    ) -> int4 {
        // First update opMeld.rootVn with new intersection information
        for rooted in self.op_meld.iter_mut() {
            let pos = parent_map[rooted.root_vn as usize];
            if pos == -1 {
                rooted.op = None; // Op split but did not rejoin
            } else {
                rooted.root_vn = pos; // New index
            }
        }

        // Do a merge sort, keeping ops in execution order
        let mut new_meld: Vec<RootedOp> = Vec::new();
        let mut cur_root: int4 = -1;
        let mut meld_pos: usize = 0; // Ops moved from old opMeld into newMeld
        let mut last_block: Option<BlockId> = None;
        for node in path.iter().take(cut_off as usize) {
            let op = node.op; // Current op in the new path
            let mut cur_op: Option<OpId> = None;
            while meld_pos < self.op_meld.len() {
                let trial_op = match self.op_meld[meld_pos].op {
                    None => {
                        meld_pos += 1;
                        continue;
                    }
                    Some(t) => t,
                };
                let trial_parent = fd.obank().get(trial_op).unwrap().get_parent();
                let op_parent = fd.obank().get(op).unwrap().get_parent();
                if trial_parent != op_parent {
                    if op_parent == last_block {
                        cur_op = None; // op comes AFTER trialOp
                        break;
                    } else if trial_parent != last_block {
                        // Both trialOp and op come from different blocks that are
                        // not the lastBlock
                        let res = self.op_meld[meld_pos].root_vn; // Force truncatePath at (and above) this op

                        // Found a new cut point
                        self.op_meld = new_meld; // Take what we've melded so far
                        return res; // return the new cutpoint
                    }
                } else {
                    let trial_order =
                        fd.obank().get(trial_op).unwrap().get_seq_num().get_order();
                    let op_order = fd.obank().get(op).unwrap().get_seq_num().get_order();
                    if trial_order <= op_order {
                        cur_op = Some(trial_op); // op is equal to or comes later than trialOp
                        break;
                    }
                }
                last_block = trial_parent;
                new_meld.push(self.op_meld[meld_pos]); // Current old op moved into newMeld
                cur_root = self.op_meld[meld_pos].root_vn;
                meld_pos += 1;
            }
            if cur_op == Some(op) {
                new_meld.push(self.op_meld[meld_pos]);
                cur_root = self.op_meld[meld_pos].root_vn;
                meld_pos += 1;
            } else {
                new_meld.push(RootedOp { op: Some(op), root_vn: cur_root });
            }
            last_block = fd.obank().get(op).unwrap().get_parent();
        }
        self.op_meld = new_meld;
        -1
    }

    /// Truncate all paths at the given new Varnode index
    /// (C++ `PathMeld::truncatePaths`, `jumptable.cc:902`).
    fn truncate_paths(&mut self, cut_point: int4) {
        while self.op_meld.len() > 1 {
            if self.op_meld.last().unwrap().root_vn < cut_point {
                // If we see op using varnode earlier than cut point, keep that
                // and all subsequent ops
                break;
            }
            self.op_meld.pop(); // Otherwise cut the op
        }
        self.common_vn.truncate(cut_point as usize);
    }

    /// Copy paths from another container (C++ `PathMeld::set(const PathMeld&)`).
    pub fn set_from(&mut self, op2: &PathMeld) {
        self.common_vn = op2.common_vn.clone();
        self.op_meld = op2.op_meld.clone();
    }

    /// Initialize \b this to be a single path
    /// (C++ `PathMeld::set(const vector<PcodeOpNode>&)`, `jumptable.cc:923`).
    pub fn set_path(&mut self, fd: &Funcdata, path: &[PcodeOpNode]) {
        for (i, node) in path.iter().enumerate() {
            let vn = fd.obank().get(node.op).unwrap().get_in(node.slot).unwrap();
            self.op_meld.push(RootedOp { op: Some(node.op), root_vn: i as int4 });
            self.common_vn.push(vn);
        }
    }

    /// Initialize \b this container to a single node "path"
    /// (C++ `PathMeld::set(PcodeOp*,Varnode*)`, `jumptable.cc:936`).
    pub fn set_single(&mut self, op: OpId, vn: VarnodeId) {
        self.common_vn.push(vn);
        self.op_meld.push(RootedOp { op: Some(op), root_vn: 0 });
    }

    /// Append a new set of paths to \b this set of paths
    /// (C++ `PathMeld::append`, `jumptable.cc:948`).
    pub fn append(&mut self, op2: &PathMeld) {
        // commonVn.insert(begin, op2.commonVn...)
        let prefix_vn = op2.common_vn.clone();
        let n_vn = prefix_vn.len();
        let mut new_common = prefix_vn;
        new_common.extend(self.common_vn.iter().copied());
        self.common_vn = new_common;

        let prefix_ops = op2.op_meld.clone();
        let n_ops = prefix_ops.len();
        let mut new_meld = prefix_ops;
        new_meld.extend(self.op_meld.iter().copied());
        self.op_meld = new_meld;

        // Renumber all the rootVn refs to varnodes we have moved
        for rooted in self.op_meld.iter_mut().skip(n_ops) {
            rooted.root_vn += n_vn as int4;
        }
    }

    /// Clear \b this to be an empty container (C++ `PathMeld::clear`).
    pub fn clear(&mut self) {
        self.common_vn.clear();
        self.op_meld.clear();
    }

    /// Meld a new path into \b this container (C++ `PathMeld::meld`,
    /// `jumptable.cc:969`).
    pub fn meld(&mut self, fd: &mut Funcdata, path: &mut Vec<PcodeOpNode>) {
        let mut parent_map: Vec<int4> = Vec::new();

        for node in path.iter() {
            // Mark varnodes in the new path, so its easy to see intersection
            let vn = fd.obank().get(node.op).unwrap().get_in(node.slot).unwrap();
            fd.vbank_mut().get_mut(vn).unwrap().set_mark();
        }
        self.internal_intersect(fd, &mut parent_map); // Calculate varnode intersection, and map
        let mut cut_off: int4 = -1;

        // Calculate where the cutoff point is in the new path
        for (i, node) in path.iter().enumerate() {
            let vn = fd.obank().get(node.op).unwrap().get_in(node.slot).unwrap();
            if !fd.vbank().get(vn).unwrap().is_mark() {
                // If mark already cleared, we know it is in intersection
                cut_off = i as int4 + 1; // Cut-off must at least be past this -vn-
            } else {
                fd.vbank_mut().get_mut(vn).unwrap().clear_mark();
            }
        }
        let new_cutoff = self.meld_ops(fd, path, cut_off, &parent_map); // meld in new ops
        if new_cutoff >= 0 {
            // If not all ops could be ordered
            self.truncate_paths(new_cutoff); // Cut off at the point where we couldn't order
        }
        path.truncate(cut_off as usize);
    }

    /// Mark (or unmark) PcodeOps along the paths from the given start
    /// (C++ `PathMeld::markPaths`, `jumptable.cc:1001`).
    pub fn mark_paths(&self, fd: &mut Funcdata, val: bool, start_varnode: int4) {
        let mut start_op: int4 = -1;
        for i in (0..self.op_meld.len()).rev() {
            if self.op_meld[i].root_vn == start_varnode {
                start_op = i as int4;
                break;
            }
        }
        if start_op < 0 {
            return;
        }
        if val {
            for i in 0..=(start_op as usize) {
                if let Some(op) = self.op_meld[i].op {
                    fd.obank_mut().get_mut(op).unwrap().set_mark();
                }
            }
        } else {
            for i in 0..=(start_op as usize) {
                if let Some(op) = self.op_meld[i].op {
                    fd.obank_mut().get_mut(op).unwrap().clear_mark();
                }
            }
        }
    }

    /// Return the number of Varnodes common to all paths
    /// (C++ `PathMeld::numCommonVarnode`).
    pub fn num_common_varnode(&self) -> int4 {
        self.common_vn.len() as int4
    }

    /// Return the number of PcodeOps across all paths (C++ `PathMeld::numOps`).
    pub fn num_ops(&self) -> int4 {
        self.op_meld.len() as int4
    }

    /// Get the i-th common Varnode (C++ `PathMeld::getVarnode`).
    pub fn get_varnode(&self, i: int4) -> VarnodeId {
        self.common_vn[i as usize]
    }

    /// Get the split-point for the i-th PcodeOp (C++ `PathMeld::getOpParent`).
    pub fn get_op_parent(&self, i: int4) -> VarnodeId {
        self.common_vn[self.op_meld[i as usize].root_vn as usize]
    }

    /// Get the i-th PcodeOp (C++ `PathMeld::getOp`).  `None` models the dropped
    /// split-op sentinel.
    pub fn get_op(&self, i: int4) -> Option<OpId> {
        self.op_meld[i as usize].op
    }

    /// Find \e earliest PcodeOp that has a specific common Varnode as input
    /// (C++ `PathMeld::getEarliestOp`, `jumptable.cc:1024`).
    pub fn get_earliest_op(&self, pos: int4) -> Option<OpId> {
        for i in (0..self.op_meld.len()).rev() {
            if self.op_meld[i].root_vn == pos {
                return self.op_meld[i].op;
            }
        }
        None
    }

    /// Return \b true if a LOAD exists in the common path
    /// (C++ `PathMeld::isLoadInPath`, `jumptable.cc:1037`).
    pub fn is_load_in_path(&self, fd: &Funcdata, mut i: int4) -> bool {
        while i > 0 {
            i -= 1;
            let vn = self.common_vn[i as usize];
            let v = fd.vbank().get(vn).unwrap();
            if !v.is_written() {
                continue;
            }
            let def = v.get_def().unwrap();
            if fd.obank().get(def).unwrap().code() == OpCode::CPUI_LOAD {
                return true;
            }
        }
        false
    }

    /// Return \b true if \b this container holds no paths (C++ `PathMeld::empty`).
    pub fn empty(&self) -> bool {
        self.common_vn.is_empty()
    }
}

// ---------------------------------------------------------------------------
// GuardRecord (jumptable.hh:138, jumptable.cc:599-786)
// ---------------------------------------------------------------------------

/// A (putative) switch variable Varnode and a constraint imposed by a CBRANCH
/// (C++ `GuardRecord`, `jumptable.hh:138`).
///
/// Constrains a specific Varnode.  If the associated CBRANCH is followed along
/// the path that reaches the switch's BRANCHIND, then we have an explicit
/// description of the possible values the Varnode can hold.
#[derive(Debug, Clone)]
pub struct GuardRecord {
    /// PcodeOp CBRANCH that branches around the switch (`None` == cleared/unused).
    cbranch: Option<OpId>,
    /// The immediate PcodeOp causing the restriction (C++ `GuardRecord::readOp`).
    read_op: Option<OpId>,
    /// The Varnode being restricted (C++ `GuardRecord::vn`).
    vn: VarnodeId,
    /// Value being (quasi)copied to the Varnode (C++ `GuardRecord::baseVn`).
    base_vn: VarnodeId,
    /// Specific CBRANCH path going to the switch (C++ `GuardRecord::indpath`).
    indpath: int4,
    /// Number of bits copied (all other bits are zero)
    /// (C++ `GuardRecord::bitsPreserved`).
    bits_preserved: int4,
    /// Range of values causing the CBRANCH to take the path to the switch.
    range: CircleRange,
    /// \b true if guarding CBRANCH is duplicated across multiple blocks.
    unrolled: bool,
}

impl GuardRecord {
    /// Construct a guard record (C++ `GuardRecord::GuardRecord`, `jumptable.cc:614`).
    pub fn new(
        fd: &Funcdata,
        b_op: OpId,
        r_op: OpId,
        path: int4,
        rng: CircleRange,
        v: VarnodeId,
        unr: bool,
    ) -> GuardRecord {
        let mut bits_preserved = 0;
        let base_vn = Self::quasi_copy(fd, v, &mut bits_preserved); // Look for varnode whose bits are copied
        GuardRecord {
            cbranch: Some(b_op),
            read_op: Some(r_op),
            vn: v,
            base_vn,
            indpath: path,
            bits_preserved,
            range: rng,
            unrolled: unr,
        }
    }

    /// Is \b this guard duplicated across multiple blocks (C++ `isUnrolled`).
    pub fn is_unrolled(&self) -> bool {
        self.unrolled
    }

    /// Get the CBRANCH associated with \b this guard (C++ `getBranch`).
    pub fn get_branch(&self) -> Option<OpId> {
        self.cbranch
    }

    /// Get the PcodeOp immediately causing the restriction (C++ `getReadOp`).
    pub fn get_read_op(&self) -> Option<OpId> {
        self.read_op
    }

    /// Get the specific path index going towards the switch (C++ `getPath`).
    pub fn get_path(&self) -> int4 {
        self.indpath
    }

    /// Get the range of values causing the switch path to be taken (C++ `getRange`).
    pub fn get_range(&self) -> &CircleRange {
        &self.range
    }

    /// Mark \b this guard as unused (C++ `GuardRecord::clear`).
    pub fn clear(&mut self) {
        self.cbranch = None;
    }

    /// Determine if \b this guard applies to the given Varnode
    /// (C++ `GuardRecord::valueMatch`, `jumptable.cc:638`).
    ///
    /// Returns 0 (clearly different), 1 (clearly the same value), or 2 (same
    /// value pending no writes between their defining ops).
    pub fn value_match(
        &self,
        fd: &Funcdata,
        vn2: VarnodeId,
        base_vn2: VarnodeId,
        bits_preserved2: int4,
    ) -> int4 {
        if self.vn == vn2 {
            return 1; // Same varnode, same value
        }
        let load_op: Option<OpId>;
        let load_op2: Option<OpId>;
        if self.bits_preserved == bits_preserved2 {
            // Are the same number of bits being copied
            if self.base_vn == base_vn2 {
                // Are bits being copied from same varnode
                return 1; // If so, values are the same
            }
            load_op = fd.vbank().get(self.base_vn).unwrap().get_def();
            load_op2 = fd.vbank().get(base_vn2).unwrap().get_def();
        } else {
            load_op = fd.vbank().get(self.vn).unwrap().get_def();
            load_op2 = fd.vbank().get(vn2).unwrap().get_def();
        }
        let load_op = match load_op {
            None => return 0,
            Some(o) => o,
        };
        let load_op2 = match load_op2 {
            None => return 0,
            Some(o) => o,
        };
        if Self::one_off_match(fd, load_op, load_op2) == 1 {
            // Check for simple duplicate calculations
            return 1;
        }
        if fd.obank().get(load_op).unwrap().code() != OpCode::CPUI_LOAD {
            return 0;
        }
        if fd.obank().get(load_op2).unwrap().code() != OpCode::CPUI_LOAD {
            return 0;
        }
        let l0 = fd.obank().get(load_op).unwrap().get_in(0).unwrap();
        let l20 = fd.obank().get(load_op2).unwrap().get_in(0).unwrap();
        if fd.vbank().get(l0).unwrap().get_offset()
            != fd.vbank().get(l20).unwrap().get_offset()
        {
            return 0;
        }
        let ptr = fd.obank().get(load_op).unwrap().get_in(1).unwrap();
        let ptr2 = fd.obank().get(load_op2).unwrap().get_in(1).unwrap();
        if ptr == ptr2 {
            return 2;
        }
        if !fd.vbank().get(ptr).unwrap().is_written() {
            return 0;
        }
        if !fd.vbank().get(ptr2).unwrap().is_written() {
            return 0;
        }
        let addop = fd.vbank().get(ptr).unwrap().get_def().unwrap();
        if fd.obank().get(addop).unwrap().code() != OpCode::CPUI_INT_ADD {
            return 0;
        }
        let constvn = fd.obank().get(addop).unwrap().get_in(1).unwrap();
        if !fd.vbank().get(constvn).unwrap().is_constant() {
            return 0;
        }
        let addop2 = fd.vbank().get(ptr2).unwrap().get_def().unwrap();
        if fd.obank().get(addop2).unwrap().code() != OpCode::CPUI_INT_ADD {
            return 0;
        }
        let constvn2 = fd.obank().get(addop2).unwrap().get_in(1).unwrap();
        if !fd.vbank().get(constvn2).unwrap().is_constant() {
            return 0;
        }
        if fd.obank().get(addop).unwrap().get_in(0) != fd.obank().get(addop2).unwrap().get_in(0) {
            return 0;
        }
        if fd.vbank().get(constvn).unwrap().get_offset()
            != fd.vbank().get(constvn2).unwrap().get_offset()
        {
            return 0;
        }
        2
    }

    /// Return 1 if the two given PcodeOps produce exactly the same value, 0
    /// otherwise (C++ `GuardRecord::oneOffMatch`, `jumptable.cc:685`).
    pub fn one_off_match(fd: &Funcdata, op1: OpId, op2: OpId) -> int4 {
        let c1 = fd.obank().get(op1).unwrap().code();
        if c1 != fd.obank().get(op2).unwrap().code() {
            return 0;
        }
        match c1 {
            OpCode::CPUI_INT_AND
            | OpCode::CPUI_INT_ADD
            | OpCode::CPUI_INT_XOR
            | OpCode::CPUI_INT_OR
            | OpCode::CPUI_INT_LEFT
            | OpCode::CPUI_INT_RIGHT
            | OpCode::CPUI_INT_SRIGHT
            | OpCode::CPUI_INT_MULT
            | OpCode::CPUI_SUBPIECE => {
                if fd.obank().get(op2).unwrap().get_in(0) != fd.obank().get(op1).unwrap().get_in(0)
                {
                    return 0;
                }
                let a = fd.obank().get(op2).unwrap().get_in(1).unwrap();
                let b = fd.obank().get(op1).unwrap().get_in(1).unwrap();
                if matching_constants(fd, a, b) {
                    return 1;
                }
            }
            _ => {}
        }
        0
    }

    /// Compute the source of a quasi-COPY chain for the given Varnode
    /// (C++ `GuardRecord::quasiCopy`, `jumptable.cc:720`).
    ///
    /// A value is a \b quasi-copy if a sequence of PcodeOps producing it always
    /// hold the value as the least significant bits of their output, possibly
    /// with other non-zero values in the upper bits.  Computes the earliest
    /// ancestor Varnode for which `vn` is a quasi-copy, and the number of
    /// preserved low bits.
    pub fn quasi_copy(fd: &Funcdata, mut vn: VarnodeId, bits_preserved: &mut int4) -> VarnodeId {
        *bits_preserved = mostsigbit_set(fd.vbank().get(vn).unwrap().get_nz_mask()) + 1;
        if *bits_preserved == 0 {
            return vn;
        }
        // mask = ((1<<1) << (bitsPreserved-1)) - 1
        let mut mask: uintb = 1u64 << 1;
        mask <<= *bits_preserved - 1;
        mask = mask.wsub(1);
        let mut op = fd.vbank().get(vn).unwrap().get_def();
        while let Some(o) = op {
            let oc = fd.obank().get(o).unwrap().code();
            match oc {
                OpCode::CPUI_COPY => {
                    vn = fd.obank().get(o).unwrap().get_in(0).unwrap();
                    op = fd.vbank().get(vn).unwrap().get_def();
                }
                OpCode::CPUI_INT_AND => {
                    let const_vn = fd.obank().get(o).unwrap().get_in(1).unwrap();
                    let cv = fd.vbank().get(const_vn).unwrap();
                    if cv.is_constant() && cv.get_offset() == mask {
                        vn = fd.obank().get(o).unwrap().get_in(0).unwrap();
                        op = fd.vbank().get(vn).unwrap().get_def();
                    } else {
                        op = None;
                    }
                }
                OpCode::CPUI_INT_OR => {
                    let const_vn = fd.obank().get(o).unwrap().get_in(1).unwrap();
                    let cv = fd.vbank().get(const_vn).unwrap();
                    if cv.is_constant() && ((cv.get_offset() | mask) == (cv.get_offset() ^ mask)) {
                        vn = fd.obank().get(o).unwrap().get_in(0).unwrap();
                        op = fd.vbank().get(vn).unwrap().get_def();
                    } else {
                        op = None;
                    }
                }
                OpCode::CPUI_INT_SEXT | OpCode::CPUI_INT_ZEXT => {
                    let in0 = fd.obank().get(o).unwrap().get_in(0).unwrap();
                    if fd.vbank().get(in0).unwrap().get_size() * 8 >= *bits_preserved {
                        vn = in0;
                        op = fd.vbank().get(vn).unwrap().get_def();
                    } else {
                        op = None;
                    }
                }
                OpCode::CPUI_PIECE => {
                    let in1 = fd.obank().get(o).unwrap().get_in(1).unwrap();
                    if fd.vbank().get(in1).unwrap().get_size() * 8 >= *bits_preserved {
                        vn = in1;
                        op = fd.vbank().get(vn).unwrap().get_def();
                    } else {
                        op = None;
                    }
                }
                OpCode::CPUI_SUBPIECE => {
                    let const_vn = fd.obank().get(o).unwrap().get_in(1).unwrap();
                    let cv = fd.vbank().get(const_vn).unwrap();
                    if cv.is_constant() && cv.get_offset() == 0 {
                        vn = fd.obank().get(o).unwrap().get_in(0).unwrap();
                        op = fd.vbank().get(vn).unwrap().get_def();
                    } else {
                        op = None;
                    }
                }
                _ => {
                    op = None;
                }
            }
        }
        vn
    }
}

/// Check if the two given Varnodes are matching constants
/// (C++ `matching_constants`, `jumptable.cc:599`).
fn matching_constants(fd: &Funcdata, vn1: VarnodeId, vn2: VarnodeId) -> bool {
    let v1 = fd.vbank().get(vn1).unwrap();
    if !v1.is_constant() {
        return false;
    }
    let v2 = fd.vbank().get(vn2).unwrap();
    if !v2.is_constant() {
        return false;
    }
    if v1.get_offset() != v2.get_offset() {
        return false;
    }
    true
}

// ---------------------------------------------------------------------------
// JumpValues iterator family (jumptable.hh:166-231, jumptable.cc:261-388)
// ---------------------------------------------------------------------------

/// An iterator over values a switch variable can take (C++ `JumpValues`,
/// `jumptable.hh:166`).
///
/// Provides start values for emulation of a jump-table model.  Each value can
/// be associated with a starting Varnode and PcodeOp via [`JumpValues::get_start_varnode`]
/// and [`JumpValues::get_start_op`].
pub trait JumpValues {
    /// Truncate the number of values to the given number (C++ `truncate`).
    fn truncate(&mut self, nm: int4);
    /// Return the number of values the variables can take (C++ `getSize`).
    fn get_size(&self) -> uintb;
    /// Return \b true if the given value is in the set of possible values.
    fn contains(&self, val: uintb) -> KunaResult<bool>;
    /// Initialize \b this for iterating over the set of possible values; returns
    /// \b true if there are any values to iterate over.
    fn initialize_for_reading(&mut self) -> bool;
    /// Advance the iterator, return \b true if there is another value.
    fn next(&mut self) -> KunaResult<bool>;
    /// Get the current value (C++ `getValue`).
    fn get_value(&self) -> uintb;
    /// Get the Varnode associated with the current value (C++ `getStartVarnode`).
    fn get_start_varnode(&self) -> Option<VarnodeId>;
    /// Get the PcodeOp associated with the current value (C++ `getStartOp`).
    fn get_start_op(&self) -> Option<OpId>;
    /// Return \b true if the current value can be reversed to get a label.
    fn is_reversible(&self) -> bool;
    /// Clone \b this iterator (C++ `clone`).
    fn clone_box(&self) -> Box<dyn JumpValues>;
    /// Mutable access to the range/start setters common to both iterator
    /// variants (the `setRange`/`setStartVn`/`setStartOp`/`truncate` surface the
    /// model writes during recovery).
    fn as_setters_mut(&mut self) -> &mut dyn JumpValuesSetters;
}

/// The mutable setter surface common to [`JumpValuesRange`] and
/// [`JumpValuesRangeDefault`] (both write the same base range/start fields during
/// model recovery: C++ `jrange->setRange(...)`, `setStartVn`, `setStartOp`).
pub trait JumpValuesSetters {
    /// Set the value range (C++ `setRange`).
    fn set_range(&mut self, rng: CircleRange);
    /// Set the normalized switch Varnode (C++ `setStartVn`).
    fn set_start_vn(&mut self, vn: VarnodeId);
    /// Set the starting PcodeOp (C++ `setStartOp`).
    fn set_start_op(&mut self, op: OpId);
    /// Truncate the iterated value count (C++ `truncate`).
    fn truncate(&mut self, nm: int4);
}

impl JumpValuesSetters for JumpValuesRange {
    fn set_range(&mut self, rng: CircleRange) {
        JumpValuesRange::set_range(self, rng)
    }
    fn set_start_vn(&mut self, vn: VarnodeId) {
        JumpValuesRange::set_start_vn(self, vn)
    }
    fn set_start_op(&mut self, op: OpId) {
        JumpValuesRange::set_start_op(self, op)
    }
    fn truncate(&mut self, nm: int4) {
        <Self as JumpValues>::truncate(self, nm)
    }
}

impl JumpValuesSetters for JumpValuesRangeDefault {
    fn set_range(&mut self, rng: CircleRange) {
        JumpValuesRangeDefault::set_range(self, rng)
    }
    fn set_start_vn(&mut self, vn: VarnodeId) {
        JumpValuesRangeDefault::set_start_vn(self, vn)
    }
    fn set_start_op(&mut self, op: OpId) {
        JumpValuesRangeDefault::set_start_op(self, op)
    }
    fn truncate(&mut self, nm: int4) {
        <Self as JumpValues>::truncate(self, nm)
    }
}

/// Single entry switch variable that can take a range of values
/// (C++ `JumpValuesRange`, `jumptable.hh:188`).
#[derive(Debug, Clone, Default)]
pub struct JumpValuesRange {
    /// Acceptable range of values for the normalized switch variable.
    range: CircleRange,
    /// Varnode representing the normalized switch variable
    /// (C++ `JumpValuesRange::normqvn`).
    normqvn: Option<VarnodeId>,
    /// First PcodeOp in the jump-table calculation (C++ `JumpValuesRange::startop`).
    startop: Option<OpId>,
    /// The current value pointed to by the iterator (C++ `JumpValuesRange::curval`).
    curval: uintb,
}

impl JumpValuesRange {
    /// Construct an empty range iterator.
    pub fn new() -> JumpValuesRange {
        JumpValuesRange::default()
    }

    /// Set the range of values explicitly (C++ `setRange`).
    pub fn set_range(&mut self, rng: CircleRange) {
        self.range = rng;
    }

    /// Set the normalized switch Varnode explicitly (C++ `setStartVn`).
    pub fn set_start_vn(&mut self, vn: VarnodeId) {
        self.normqvn = Some(vn);
    }

    /// Set the starting PcodeOp explicitly (C++ `setStartOp`).
    pub fn set_start_op(&mut self, op: OpId) {
        self.startop = Some(op);
    }
}

impl JumpValues for JumpValuesRange {
    fn truncate(&mut self, nm: int4) {
        // jumptable.cc:261
        let mut range_size: int4 = 8 * 8 - count_leading_zeros(self.range.get_mask());
        range_size >>= 3;
        let left = self.range.get_min();
        let step = self.range.get_step();
        // right = (left + step*nm) & mask
        let right = left.wadd((step as uintb).wmul(nm as uintb)) & self.range.get_mask();
        self.range.set_range(left, right, range_size, step);
    }

    fn get_size(&self) -> uintb {
        self.range.get_size()
    }

    fn contains(&self, val: uintb) -> KunaResult<bool> {
        Ok(self.range.contains_val(val))
    }

    fn initialize_for_reading(&mut self) -> bool {
        if self.range.get_size() == 0 {
            return false;
        }
        self.curval = self.range.get_min();
        true
    }

    fn next(&mut self) -> KunaResult<bool> {
        let mut cv = self.curval;
        let r = self.range.get_next(&mut cv);
        self.curval = cv;
        Ok(r)
    }

    fn get_value(&self) -> uintb {
        self.curval
    }

    fn get_start_varnode(&self) -> Option<VarnodeId> {
        self.normqvn
    }

    fn get_start_op(&self) -> Option<OpId> {
        self.startop
    }

    fn is_reversible(&self) -> bool {
        true
    }

    fn clone_box(&self) -> Box<dyn JumpValues> {
        Box::new(JumpValuesRange {
            range: self.range.clone(),
            normqvn: self.normqvn,
            startop: self.startop,
            curval: 0,
        })
    }

    fn as_setters_mut(&mut self) -> &mut dyn JumpValuesSetters {
        self
    }
}

/// A jump-table starting range with two possible execution paths
/// (C++ `JumpValuesRangeDefault`, `jumptable.hh:214`).
///
/// Extends [`JumpValuesRange`] with a second entry point that takes only a
/// single value, which comes last in the iteration.
#[derive(Debug, Clone, Default)]
pub struct JumpValuesRangeDefault {
    /// The base single-entry range iterator.
    base: JumpValuesRange,
    /// The extra value (C++ `JumpValuesRangeDefault::extravalue`).
    extravalue: uintb,
    /// The starting Varnode associated with the extra value
    /// (C++ `JumpValuesRangeDefault::extravn`).
    extravn: Option<VarnodeId>,
    /// The starting PcodeOp associated with the extra value
    /// (C++ `JumpValuesRangeDefault::extraop`).
    extraop: Option<OpId>,
    /// \b true if the extra value has been visited by the iterator.
    lastvalue: bool,
}

impl JumpValuesRangeDefault {
    /// Construct an empty default-range iterator.
    pub fn new() -> JumpValuesRangeDefault {
        JumpValuesRangeDefault::default()
    }

    /// Set the range of values explicitly (delegates to base; C++ `setRange`).
    pub fn set_range(&mut self, rng: CircleRange) {
        self.base.set_range(rng);
    }

    /// Set the normalized switch Varnode (delegates to base; C++ `setStartVn`).
    pub fn set_start_vn(&mut self, vn: VarnodeId) {
        self.base.set_start_vn(vn);
    }

    /// Set the starting PcodeOp (delegates to base; C++ `setStartOp`).
    pub fn set_start_op(&mut self, op: OpId) {
        self.base.set_start_op(op);
    }

    /// Set the extra value explicitly (C++ `setExtraValue`).
    pub fn set_extra_value(&mut self, val: uintb) {
        self.extravalue = val;
    }

    /// Set the associated start Varnode (C++ `setDefaultVn`).
    pub fn set_default_vn(&mut self, vn: VarnodeId) {
        self.extravn = Some(vn);
    }

    /// Set the associated start PcodeOp (C++ `setDefaultOp`).
    pub fn set_default_op(&mut self, op: OpId) {
        self.extraop = Some(op);
    }
}

impl JumpValues for JumpValuesRangeDefault {
    fn truncate(&mut self, nm: int4) {
        <JumpValuesRange as JumpValues>::truncate(&mut self.base, nm);
    }

    fn get_size(&self) -> uintb {
        self.base.range.get_size() + 1
    }

    fn contains(&self, val: uintb) -> KunaResult<bool> {
        if self.extravalue == val {
            return Ok(true);
        }
        Ok(self.base.range.contains_val(val))
    }

    fn initialize_for_reading(&mut self) -> bool {
        if self.base.range.get_size() == 0 {
            self.base.curval = self.extravalue;
            self.lastvalue = true;
        } else {
            self.base.curval = self.base.range.get_min();
            self.lastvalue = false;
        }
        true
    }

    fn next(&mut self) -> KunaResult<bool> {
        if self.lastvalue {
            return Ok(false);
        }
        let mut cv = self.base.curval;
        if self.base.range.get_next(&mut cv) {
            self.base.curval = cv;
            return Ok(true);
        }
        self.lastvalue = true;
        self.base.curval = self.extravalue;
        Ok(true)
    }

    fn get_value(&self) -> uintb {
        self.base.curval
    }

    fn get_start_varnode(&self) -> Option<VarnodeId> {
        if self.lastvalue {
            self.extravn
        } else {
            self.base.normqvn
        }
    }

    fn get_start_op(&self) -> Option<OpId> {
        if self.lastvalue {
            self.extraop
        } else {
            self.base.startop
        }
    }

    fn is_reversible(&self) -> bool {
        !self.lastvalue // The -extravalue- is not reversible
    }

    fn clone_box(&self) -> Box<dyn JumpValues> {
        Box::new(JumpValuesRangeDefault {
            base: JumpValuesRange {
                range: self.base.range.clone(),
                normqvn: self.base.normqvn,
                startop: self.base.startop,
                curval: 0,
            },
            extravalue: self.extravalue,
            extravn: self.extravn,
            extraop: self.extraop,
            lastvalue: false,
        })
    }

    fn as_setters_mut(&mut self) -> &mut dyn JumpValuesSetters {
        self
    }
}

// ---------------------------------------------------------------------------
// JumpModel trait + JumpModelTrivial (jumptable.hh:243-366)
// ---------------------------------------------------------------------------

/// A jump-table execution model (C++ `JumpModel`, `jumptable.hh:243`).
///
/// Holds model details and recovers them in stages.  The model concepts:
/// the Address Table, the Normalized/Unnormalized Switch Variables, the case
/// labels, and the guards.
///
/// SEAM note: many model methods require absent subsystems (emulation, the
/// loader, TypeOp reverse-eval, structuring).  Those are `// SEAM(...)` `Err`
/// shells.  The trait surface itself is the faithful C++ virtual interface.
pub trait JumpModel {
    /// Return \b true if \b this model was manually overridden (C++ `isOverride`).
    fn is_override(&self) -> bool;
    /// Return the number of entries in the address table (C++ `getTableSize`).
    fn get_table_size(&self) -> int4;
    /// Attempt to recover details of the model, given a specific BRANCHIND
    /// (C++ `recoverModel`).
    fn recover_model(
        &mut self,
        fd: &mut Funcdata,
        indop: OpId,
        matchsize: uint4,
        maxtablesize: uint4,
    ) -> KunaResult<bool>;
    /// Construct the explicit list of target addresses (C++ `buildAddresses`).
    fn build_addresses(
        &self,
        fd: &Funcdata,
        indop: OpId,
        addresstable: &mut Vec<Address>,
        loadpoints: Option<&mut Vec<LoadTable>>,
        loadcounts: Option<&mut Vec<int4>>,
    ) -> KunaResult<()>;
    /// Recover the unnormalized switch variable (C++ `findUnnormalized`).
    fn find_unnormalized(
        &mut self,
        fd: &mut Funcdata,
        maxaddsub: uint4,
        maxleftright: uint4,
        maxext: uint4,
    ) -> KunaResult<()>;
    /// Recover \e case labels associated with the Address table (C++ `buildLabels`).
    fn build_labels(
        &self,
        fd: &mut Funcdata,
        addresstable: &mut [Address],
        label: &mut Vec<uintb>,
        orig: &dyn JumpModel,
    ) -> KunaResult<()>;
    /// Do normalization of the given switch specific to \b this model
    /// (C++ `foldInNormalization`).  Returns the final unnormalized switch
    /// Varnode (or `None`).
    fn fold_in_normalization(&mut self, fd: &mut Funcdata, indop: OpId)
        -> KunaResult<Option<VarnodeId>>;
    /// Eliminate any \e guard code involved in computing the switch destination
    /// (C++ `foldInGuards`).
    fn fold_in_guards(&mut self, fd: &mut Funcdata, jump: &mut JumpTable) -> KunaResult<bool>;
    /// Perform a sanity check on recovered addresses (C++ `sanityCheck`).
    fn sanity_check(
        &mut self,
        fd: &mut Funcdata,
        indop: OpId,
        addresstable: &mut Vec<Address>,
        loadpoints: &mut Vec<LoadTable>,
        loadcounts: Option<&Vec<int4>>,
    ) -> KunaResult<bool>;
    /// Clone \b this model (C++ `clone`).
    fn clone_box(&self) -> Box<dyn JumpModel>;
    /// Downcast support: `buildLabels` receives the \e orig model as a
    /// `&dyn JumpModel` and (in C++) `static_cast`s it to `const JumpBasic *`.
    fn as_any(&self) -> &dyn std::any::Any;
    /// Clear any non-permanent aspects of the model (C++ `clear`).
    fn clear(&mut self) {}
    /// Encode \b this model to a stream (C++ `encode`).
    fn encode(&self, _encoder: &mut dyn Encoder) -> KunaResult<()> {
        Ok(())
    }
    /// Decode \b this model from a stream (C++ `decode`).
    fn decode(&mut self, _decoder: &mut dyn Decoder) -> KunaResult<()> {
        Ok(())
    }
}

/// A trivial jump-table model, where the BRANCHIND input Varnode is the switch
/// variable (C++ `JumpModelTrivial`, `jumptable.hh:350`).
///
/// Treats the input Varnode to the BRANCHIND as the switch variable, recovering
/// its possible values from the existing block structure.  Used when flow
/// following recovers destinations but switch normalization cannot recover the
/// model.
#[derive(Debug, Clone, Default)]
pub struct JumpModelTrivial {
    /// Number of addresses in the table as reported by the JumpTable.
    size: uint4,
}

impl JumpModelTrivial {
    /// Construct a trivial model (C++ `JumpModelTrivial(JumpTable*)`).
    pub fn new() -> JumpModelTrivial {
        JumpModelTrivial { size: 0 }
    }
}

impl JumpModel for JumpModelTrivial {
    fn is_override(&self) -> bool {
        false
    }

    fn get_table_size(&self) -> int4 {
        self.size as int4
    }

    fn recover_model(
        &mut self,
        fd: &mut Funcdata,
        indop: OpId,
        matchsize: uint4,
        _maxtablesize: uint4,
    ) -> KunaResult<bool> {
        // size = indop->getParent()->sizeOut()  (jumptable.cc:390)
        let parent = fd
            .obank()
            .get(indop)
            .unwrap()
            .get_parent()
            .ok_or_else(|| KunaError::lowlevel("JumpModelTrivial: BRANCHIND has no parent block"))?;
        self.size = fd.bblocks_ref().block(parent).size_out() as uint4;
        Ok((self.size != 0) && (self.size <= matchsize))
    }

    fn build_addresses(
        &self,
        fd: &Funcdata,
        indop: OpId,
        addresstable: &mut Vec<Address>,
        _loadpoints: Option<&mut Vec<LoadTable>>,
        _loadcounts: Option<&mut Vec<int4>>,
    ) -> KunaResult<()> {
        // jumptable.cc:397
        addresstable.clear();
        let bl = fd
            .obank()
            .get(indop)
            .unwrap()
            .get_parent()
            .ok_or_else(|| KunaError::lowlevel("JumpModelTrivial: BRANCHIND has no parent block"))?;
        let nout = fd.bblocks_ref().block(bl).size_out();
        for i in 0..nout {
            let outbl = fd.bblocks_ref().block(bl).get_out(i);
            addresstable.push(block_get_start(&fd.bblocks_ref().arena, outbl));
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
        // jumptable.cc:408 -- Address itself is the label
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

    fn fold_in_guards(&mut self, _fd: &mut Funcdata, _jump: &mut JumpTable) -> KunaResult<bool> {
        Ok(false)
    }

    fn sanity_check(
        &mut self,
        _fd: &mut Funcdata,
        _indop: OpId,
        _addresstable: &mut Vec<Address>,
        _loadpoints: &mut Vec<LoadTable>,
        _loadcounts: Option<&Vec<int4>>,
    ) -> KunaResult<bool> {
        Ok(true)
    }

    fn clone_box(&self) -> Box<dyn JumpModel> {
        Box::new(JumpModelTrivial { size: self.size })
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

// ---------------------------------------------------------------------------
// JumpBasic static helpers (jumptable.hh:374, jumptable.cc:425-547)
// ---------------------------------------------------------------------------

/// Pull a [`CircleRange`] back through a given PcodeOp (C++
/// `CircleRange::pullBack(PcodeOp*,Varnode**,bool)`, `rangeutil.cc:1022`).
///
/// The op's single unknown input is found (the other input, if any, must be a
/// constant); the range is pulled back through the op's primitive (via
/// [`CircleRange::pull_back_unary`]/[`CircleRange::pull_back_binary`]).  Returns
/// the input Varnode whose value range is now described by `rng`, or `None` if
/// the pull-back is not possible.  `usenzmask` intersects the input's NZMASK.
///
/// The op-coupled wrapper lives in `jumptable.rs` (not `rangeutil.rs`) because it
/// reads the function IR (`Funcdata`), which is above the value-set layer.
pub(crate) fn circlerange_pull_back(
    fd: &Funcdata,
    rng: &mut CircleRange,
    op: OpId,
    usenzmask: bool,
) -> Option<VarnodeId> {
    let opc = fd.obank().get(op).unwrap().code();
    let outsize = fd.obank().get(op).unwrap().get_out().map(|o| fd.vbank().get(o).unwrap().get_size()).unwrap_or(0);
    let numinput = fd.obank().get(op).unwrap().num_input();
    let res: VarnodeId;
    if numinput == 1 {
        res = fd.obank().get(op).unwrap().get_in(0).unwrap();
        if fd.vbank().get(res).unwrap().is_constant() {
            return None;
        }
        let insize = fd.vbank().get(res).unwrap().get_size();
        if !rng.pull_back_unary(opc, insize, outsize) {
            return None;
        }
    } else if numinput == 2 {
        // Find the non-constant varnode input, and slot. Make sure the second
        // input is constant.
        let mut slot = 0;
        let mut res_vn = fd.obank().get(op).unwrap().get_in(slot).unwrap();
        let mut constvn = fd.obank().get(op).unwrap().get_in(1 - slot).unwrap();
        if fd.vbank().get(res_vn).unwrap().is_constant() {
            slot = 1;
            constvn = res_vn;
            res_vn = fd.obank().get(op).unwrap().get_in(slot).unwrap();
            if fd.vbank().get(res_vn).unwrap().is_constant() {
                return None;
            }
        } else if !fd.vbank().get(constvn).unwrap().is_constant() {
            return None;
        }
        res = res_vn;
        let val = fd.vbank().get(constvn).unwrap().get_offset();
        let insize = fd.vbank().get(res).unwrap().get_size();
        if !rng.pull_back_binary(opc, val, slot, insize, outsize) {
            if usenzmask && opc == OpCode::CPUI_SUBPIECE && val == 0 {
                // If everything we are truncating is known to be zero, we may
                // still have a range.
                let mut msbset = mostsigbit_set(fd.vbank().get(res).unwrap().get_nz_mask());
                msbset = (msbset + 8) / 8;
                if outsize < msbset {
                    return None;
                } else {
                    // Keep the range but make the mask bigger.
                    rng.set_mask(kuna_base::address::calc_mask(insize));
                }
            } else {
                return None;
            }
        }
        // constMarkup: a symbol entry on the const is W6/symbol markup; the
        // recovery only needs the returned Varnode, so the markup is dropped.
    } else {
        return None;
    }

    if usenzmask {
        let mut nzrange = CircleRange::new_empty();
        let nz = fd.vbank().get(res).unwrap().get_nz_mask();
        let sz = fd.vbank().get(res).unwrap().get_size();
        if !nzrange.set_nz_mask(nz, sz) {
            return Some(res);
        }
        rng.intersect(&nzrange);
        // If the intersect produces 2 pieces, the original range is preserved
        // and the pull-back still counts as successful.
    }
    Some(res)
}

/// Static helpers of the basic switch model (C++ `JumpBasic`, `jumptable.hh:374`).
///
/// The pure (graph-reading) static helpers of `JumpBasic` are ported and
/// tested here.  The full `JumpBasic` instance methods (`analyzeGuards`,
/// `findNormalized`, `recoverModel`, `buildAddresses`, ...) are ported as the
/// instance struct [`JumpBasicModel`] below.
pub struct JumpBasic;

impl JumpBasic {
    /// Do we prune in our depth-first search for the normalized switch variable
    /// (C++ `JumpBasic::isprune`, `jumptable.cc:425`).
    pub fn isprune(fd: &Funcdata, vn: VarnodeId) -> bool {
        let v = fd.vbank().get(vn).unwrap();
        if !v.is_written() {
            return true;
        }
        let op = v.get_def().unwrap();
        let o = fd.obank().get(op).unwrap();
        if o.is_call() || o.is_marker() {
            return true;
        }
        if o.num_input() == 0 {
            return true;
        }
        false
    }

    /// Is it possible for the given Varnode to be a switch variable?
    /// (C++ `JumpBasic::ispoint`, `jumptable.cc:437`).
    pub fn ispoint(fd: &Funcdata, vn: VarnodeId) -> bool {
        let v = fd.vbank().get(vn).unwrap();
        if v.is_constant() {
            return false;
        }
        if v.is_annotation() {
            return false;
        }
        if v.is_read_only() {
            return false;
        }
        true
    }

    /// Get the step/stride associated with the Varnode
    /// (C++ `JumpBasic::getStride`, `jumptable.cc:450`).
    pub fn get_stride(fd: &Funcdata, vn: VarnodeId) -> int4 {
        let mut mask = fd.vbank().get(vn).unwrap().get_nz_mask();
        if (mask & 0x3f) == 0 {
            // Limit the maximum stride we can return
            return 32;
        }
        let mut stride = 1;
        while (mask & 1) == 0 {
            mask >>= 1;
            stride <<= 1;
        }
        stride
    }

    /// Get maximum value associated with the given Varnode
    /// (C++ `JumpBasic::getMaxValue`, `jumptable.cc:513`).
    ///
    /// If the Varnode has a restricted range due to masking via INT_AND, returns
    /// that range's maximum; otherwise 0 (all possible values).
    pub fn get_max_value(fd: &Funcdata, vn: VarnodeId) -> uintb {
        let mut max_value: uintb = 0; // 0 indicates maximum possible value
        let v = fd.vbank().get(vn).unwrap();
        if !v.is_written() {
            return max_value;
        }
        let op = v.get_def().unwrap();
        let oc = fd.obank().get(op).unwrap().code();
        if oc == OpCode::CPUI_INT_AND {
            let constvn = fd.obank().get(op).unwrap().get_in(1).unwrap();
            let cv = fd.vbank().get(constvn).unwrap();
            if cv.is_constant() {
                max_value = coveringmask(cv.get_offset());
                max_value = max_value.wadd(1)
                    & kuna_base::address::calc_mask(fd.vbank().get(vn).unwrap().get_size());
            }
        } else if oc == OpCode::CPUI_MULTIEQUAL {
            // Its possible the AND is duplicated across multiple blocks
            let ninput = fd.obank().get(op).unwrap().num_input();
            let mut i = 0;
            while i < ninput {
                let subvn = fd.obank().get(op).unwrap().get_in(i).unwrap();
                if !fd.vbank().get(subvn).unwrap().is_written() {
                    break;
                }
                let and_op = fd.vbank().get(subvn).unwrap().get_def().unwrap();
                if fd.obank().get(and_op).unwrap().code() != OpCode::CPUI_INT_AND {
                    break;
                }
                let constvn = fd.obank().get(and_op).unwrap().get_in(1).unwrap();
                let cv = fd.vbank().get(constvn).unwrap();
                if !cv.is_constant() {
                    break;
                }
                if max_value < cv.get_offset() {
                    max_value = cv.get_offset();
                }
                i += 1;
            }
            if i == ninput {
                max_value = coveringmask(max_value);
                max_value = max_value.wadd(1)
                    & kuna_base::address::calc_mask(fd.vbank().get(vn).unwrap().get_size());
            } else {
                max_value = 0;
            }
        }
        max_value
    }

    /// Return \b true if all array elements are the same Varnode
    /// (C++ `JumpBasic::duplicateVarnodes`, `jumptable.cc:1307`).
    pub fn duplicate_varnodes(arr: &[VarnodeId]) -> bool {
        let vn = arr[0];
        for &x in arr.iter().skip(1) {
            if x != vn {
                return false;
            }
        }
        true
    }

    /// Calculate the initial set of Varnodes that might be switch variables
    /// (C++ `JumpBasic::findDeterminingVarnodes`, `jumptable.cc:555`).
    ///
    /// Paths terminating at `op` (at input `slot`) are enumerated into the given
    /// [`PathMeld`], computing the Varnodes common to all paths.
    pub fn find_determining_varnodes(
        fd: &mut Funcdata,
        path_meld: &mut PathMeld,
        op: OpId,
        slot: int4,
    ) {
        let mut path: Vec<PcodeOpNode> = Vec::new();
        let mut firstpoint = false; // Have not seen likely switch variable yet

        path.push(PcodeOpNode::new(op, slot));

        // Traverse through tree of inputs to final address.
        // C++ is `do { ... } while(path.size() > 1)`: the body runs once with
        // size 1, then continues only while size > 1. Modeled as a `loop` with a
        // bottom break when `path.len() <= 1`.
        loop {
            let node = *path.last().unwrap();
            let curvn = fd.obank().get(node.op).unwrap().get_in(node.slot).unwrap();
            if JumpBasic::isprune(fd, curvn) {
                // Here is a node of the tree
                if JumpBasic::ispoint(fd, curvn) {
                    // Is it a possible switch variable
                    if !firstpoint {
                        // If it is the first possible
                        path_meld.set_path(fd, &path); // Take the current path as the result
                        firstpoint = true;
                    } else {
                        // If we have already seen at least one possible
                        path_meld.meld(fd, &mut path);
                    }
                }

                path.last_mut().unwrap().slot += 1;
                while path.last().unwrap().slot
                    >= fd.obank().get(path.last().unwrap().op).unwrap().num_input()
                {
                    path.pop();
                    if path.is_empty() {
                        break;
                    }
                    path.last_mut().unwrap().slot += 1;
                }
            } else {
                // This varnode is not pruned
                let def = fd.vbank().get(curvn).unwrap().get_def().unwrap();
                path.push(PcodeOpNode::new(def, 0));
            }
            if path.len() <= 1 {
                break;
            }
        }
        if path_meld.empty() {
            // Never found a likely point, which means that it looks like the
            // address is uniquely determined but the constants/readonlys haven't
            // been collapsed
            let invn = fd.obank().get(op).unwrap().get_in(slot).unwrap();
            path_meld.set_single(op, invn);
        }
    }
}

// ---------------------------------------------------------------------------
// JumpBasicModel (the instance JumpBasic, jumptable.cc:1062-1786)
// ---------------------------------------------------------------------------

/// The basic jump-table model (C++ `JumpBasic`, the instance methods).
///
/// Recovers a straight-line index-range model for the switch: the normalized
/// switch variable (smallest reaching value range), the guard constraints, and
/// the emulation-driven address table.  Holds the recovered model state
/// (`jrange`/`pathMeld`/`selectguards`/`normalvn`/`switchvn`).
///
/// The structuring-only pieces (`foldInGuards`/`foldInOneGuard`/
/// `checkUnrolledGuard`, which need `BlockBasic::findMultiequal`/
/// `liftVerifyUnroll`/`noInterveningStatement`/`Funcdata::pushBranch`) and the
/// reverse-emulation label recovery (`backup2Switch`, TypeOp reverse eval) are
/// `// SEAM(structuring/W6)` — recorded as losses; they are reached only at
/// label/guard-fold time, after the BRANCHIND addresses are recovered.
pub struct JumpBasicModel {
    /// Range of values for the (normalized) switch variable (C++ `jrange`).
    /// Boxed as a trait object so [`JumpValuesRangeDefault`] (model 2) fits.
    jrange: Option<Box<dyn JumpValues>>,
    /// Set of PcodeOps and Varnodes producing the final target addresses.
    path_meld: PathMeld,
    /// Any guards associated with the model (C++ `selectguards`).
    selectguards: Vec<GuardRecord>,
    /// Position of the normalized switch Varnode within `pathMeld`.
    varnode_index: int4,
    /// Normalized switch Varnode (C++ `normalvn`).
    normalvn: Option<VarnodeId>,
    /// Unnormalized switch Varnode (C++ `switchvn`).
    switchvn: Option<VarnodeId>,
    /// `true` if \b this is model 2 (the [`JumpBasic2`] default-path extension).
    is_model2: bool,
    /// (model 2) The extra Varnode holding the default value (C++ `extravn`).
    extravn: Option<VarnodeId>,
    /// (model 2) The set of paths that produce non-default addresses.
    orig_path_meld: PathMeld,
    /// `true` if the owning JumpTable is marked partial (drives `usenzmask`).
    is_partial: bool,
}

impl JumpBasicModel {
    /// Construct an empty basic model (C++ `JumpBasic(JumpTable*)`).
    pub fn new() -> JumpBasicModel {
        JumpBasicModel {
            jrange: None,
            path_meld: PathMeld::new(),
            selectguards: Vec::new(),
            varnode_index: 0,
            normalvn: None,
            switchvn: None,
            is_model2: false,
            extravn: None,
            orig_path_meld: PathMeld::new(),
            is_partial: false,
        }
    }

    /// Get the possible paths to the switch (C++ `getPathMeld`).
    pub fn get_path_meld(&self) -> &PathMeld {
        &self.path_meld
    }

    /// The normalized value iterator (C++ `getValueRange`).
    fn jrange(&self) -> &dyn JumpValues {
        &**self.jrange.as_ref().expect("JumpBasic: jrange not set")
    }

    /// Calculate the range of values in `vn` that direct control-flow to the
    /// switch (C++ `JumpBasic::calcRange`, `jumptable.cc:1136`).
    fn calc_range(&self, fd: &Funcdata, vn: VarnodeId, rng: &mut CircleRange) {
        // Get an initial range, based on the size/type of -vn-.
        let mut stride = 1;
        let v = fd.vbank().get(vn).unwrap();
        if v.is_constant() {
            *rng = CircleRange::new_value(v.get_offset(), v.get_size());
        } else if v.is_written()
            && fd.obank().get(v.get_def().unwrap()).unwrap().is_bool_output()
        {
            *rng = CircleRange::new(0, 2, 1, 1); // Only 0 or 1 possible
        } else {
            let max_value = JumpBasic::get_max_value(fd, vn);
            stride = JumpBasic::get_stride(fd, vn);
            *rng = CircleRange::new(0, max_value, v.get_size(), stride);
        }

        // Intersect any guard ranges which apply to -vn-.
        let mut bits_preserved = 0;
        let base_vn = GuardRecord::quasi_copy(fd, vn, &mut bits_preserved);
        for guard in self.selectguards.iter() {
            let matchval = guard.value_match(fd, vn, base_vn, bits_preserved);
            // if (matchval == 2) TODO: check aliases (upstream comment)
            if matchval == 0 {
                continue;
            }
            if rng.intersect(guard.get_range()) != 0 {
                continue;
            }
        }

        // It may be assumed the switch value is positive; if the size is too big,
        // try only positive values.
        if rng.get_size() > 0x10000 {
            let sz = fd.vbank().get(vn).unwrap().get_size();
            let mut positive = CircleRange::new(0, (rng.get_mask() >> 1).wadd(1), sz, stride);
            positive.intersect(rng);
            if !positive.is_empty() {
                *rng = positive;
            }
        }
    }

    /// Find the putative switch variable with the smallest reaching range (C++
    /// `JumpBasic::findSmallestNormal`, `jumptable.cc:1181`).
    fn find_smallest_normal(&mut self, fd: &Funcdata, matchsize: uint4) {
        let mut rng = CircleRange::new_empty();
        self.varnode_index = 0;
        let vn0 = self.path_meld.get_varnode(0);
        self.calc_range(fd, vn0, &mut rng);
        let mut maxsize = rng.get_size();
        let op0 = self.path_meld.get_op(0);
        {
            let jr = self.jrange_range_mut();
            jr.set_range(rng.clone());
            jr.set_start_vn(vn0);
            if let Some(op0) = op0 {
                jr.set_start_op(op0);
            }
        }
        let mut i: uint4 = 1;
        while (i as int4) < self.path_meld.num_common_varnode() {
            if maxsize == matchsize as uintb {
                return;
            }
            let vni = self.path_meld.get_varnode(i as int4);
            self.calc_range(fd, vni, &mut rng);
            let sz = rng.get_size();
            if sz < maxsize {
                // Don't accept a 1-byte switch var unless there is an explicit
                // guard or table lookup between the byte and the indirect jump.
                let vsize = fd.vbank().get(vni).unwrap().get_size();
                if sz != 256 || vsize != 1 || self.path_meld.is_load_in_path(fd, i as int4) {
                    self.varnode_index = i as int4;
                    maxsize = sz;
                    let earliest = self.path_meld.get_earliest_op(i as int4);
                    let jr = self.jrange_range_mut();
                    jr.set_range(rng.clone());
                    jr.set_start_vn(vni);
                    if let Some(op) = earliest {
                        jr.set_start_op(op);
                    }
                }
            }
            i += 1;
        }
    }

    /// Mutable access to the base [`JumpValuesRange`] of `jrange` (both model 1
    /// `JumpValuesRange` and model 2 `JumpValuesRangeDefault` set the same base
    /// fields via `set_range`/`set_start_vn`/`set_start_op`).
    fn jrange_range_mut(&mut self) -> &mut dyn JumpValuesSetters {
        self.jrange
            .as_mut()
            .expect("JumpBasic: jrange not set")
            .as_setters_mut()
    }

    /// Analyze the guards (CBRANCHs) restricting the switch variable (C++
    /// `JumpBasic::analyzeGuards`, `jumptable.cc:1062`).
    fn analyze_guards(
        &mut self,
        fd: &mut Funcdata,
        rootbl: BlockId,
        pathout_in: int4,
        indirect: OpId,
    ) {
        let maxbranch = 2; // Maximum number of CBRANCHs to consider
        let maxpullback = 2;
        let usenzmask = !self.is_partial;
        self.selectguards.clear();

        let mut bl = rootbl;
        let mut pathout = pathout_in;
        for i in 0..maxbranch {
            let prevbl;
            let indpath;
            if pathout >= 0 && fd.bblocks_ref().block(bl).size_out() == 2 {
                prevbl = bl;
                // C++ `bl = prevbl->getOut(pathout)` is immediately reset to
                // `prevbl` by the `bl = prevbl` below; the intermediate value is
                // never read (dead in upstream too) — kept for provenance.
                indpath = pathout;
                pathout = -1;
            } else {
                pathout = -1; // Make sure not to use pathout next time around
                loop {
                    if fd.bblocks_ref().block(bl).size_in() != 1 {
                        if fd.bblocks_ref().block(bl).size_in() > 1 {
                            self.check_unrolled_guard(fd, bl, maxpullback, usenzmask);
                        }
                        return;
                    }
                    let pb = fd.bblocks_ref().block(bl).get_in(0);
                    if fd.bblocks_ref().block(pb).size_out() != 1 {
                        // Possible to deviate from switch path in this block
                        break;
                    }
                    bl = pb; // back up to next block
                }
                prevbl = fd.bblocks_ref().block(bl).get_in(0);
                indpath = fd.bblocks_ref().block(bl).get_in_rev_index(0);
            }
            let cbranch = match fd.bb_op_tail(prevbl) {
                None => break,
                Some(c) => c,
            };
            if fd.obank().get(cbranch).unwrap().code() != OpCode::CPUI_CBRANCH {
                break;
            }
            if i != 0 {
                // Check that this CBRANCH isn't protecting some other switch.
                let otherbl = fd.bblocks_ref().block(prevbl).get_out(1 - indpath);
                if let Some(otherop) = fd.bb_op_tail(otherbl) {
                    if fd.obank().get(otherop).unwrap().code() == OpCode::CPUI_BRANCHIND
                        && otherop != indirect
                    {
                        break;
                    }
                }
            }
            let mut toswitchval = indpath == 1;
            if fd.obank().get(cbranch).unwrap().is_boolean_flip() {
                toswitchval = !toswitchval;
            }
            bl = prevbl;
            let mut vn = fd.obank().get(cbranch).unwrap().get_in(1).unwrap();
            let mut rng = CircleRange::new_bool(toswitchval);

            let indpathstore = if fd.bblocks_ref().block(prevbl).get_flip_path() {
                1 - indpath
            } else {
                indpath
            };
            self.selectguards.push(GuardRecord::new(
                fd, cbranch, cbranch, indpathstore, rng.clone(), vn, false,
            ));
            for _j in 0..maxpullback {
                if !fd.vbank().get(vn).unwrap().is_written() {
                    break;
                }
                let read_op = fd.vbank().get(vn).unwrap().get_def().unwrap();
                match circlerange_pull_back(fd, &mut rng, read_op, usenzmask) {
                    None => break,
                    Some(newvn) => vn = newvn,
                }
                if rng.is_empty() {
                    break;
                }
                self.selectguards.push(GuardRecord::new(
                    fd, cbranch, read_op, indpathstore, rng.clone(), vn, false,
                ));
            }
        }
    }

    /// Recover the normalized switch variable (C++ `JumpBasic::findNormalized`,
    /// `jumptable.cc:1222`).
    fn find_normalized(
        &mut self,
        fd: &mut Funcdata,
        rootbl: BlockId,
        pathout: int4,
        matchsize: uint4,
        maxtablesize: uint4,
        indirect: OpId,
    ) -> KunaResult<()> {
        self.analyze_guards(fd, rootbl, pathout, indirect);
        self.find_smallest_normal(fd, matchsize);
        let sz = self.jrange().get_size();
        if sz > maxtablesize as uintb && self.path_meld.num_common_varnode() == 1 {
            // Check for jump through readonly variable.
            let vn = self.path_meld.get_varnode(0);
            if fd.vbank().get(vn).unwrap().is_read_only() {
                let spc = Rc::clone(fd.vbank().get(vn).unwrap().get_space());
                let off = fd.vbank().get(vn).unwrap().get_offset();
                let vsize = fd.vbank().get(vn).unwrap().get_size();
                let addr = Address::new(spc, off);
                let val = fd.get_arch().get_load_image_value(&addr, vsize)?;
                self.varnode_index = 0;
                let op0 = self.path_meld.get_op(0);
                let jr = self.jrange_range_mut();
                jr.set_range(CircleRange::new_value(val, vsize));
                jr.set_start_vn(vn);
                if let Some(op0) = op0 {
                    jr.set_start_op(op0);
                }
            }
        }
        Ok(())
    }

    /// Mark/unmark the model PcodeOps (C++ `JumpBasic::markModel`,
    /// `jumptable.cc:1272`).
    fn mark_model(&self, fd: &mut Funcdata, val: bool) {
        self.path_meld.mark_paths(fd, val, self.varnode_index);
        for guard in self.selectguards.iter() {
            if guard.get_branch().is_none() {
                continue;
            }
            if let Some(read_op) = guard.get_read_op() {
                if val {
                    fd.obank_mut().get_mut(read_op).unwrap().set_mark();
                } else {
                    fd.obank_mut().get_mut(read_op).unwrap().clear_mark();
                }
            }
        }
    }

    /// Mark the guard CBRANCHs that are truly part of the model (C++
    /// `JumpBasic::markFoldableGuards`, `jumptable.cc:1257`).
    fn mark_foldable_guards(&mut self, fd: &Funcdata) {
        let vn = self.path_meld.get_varnode(self.varnode_index);
        let mut bits_preserved = 0;
        let base_vn = GuardRecord::quasi_copy(fd, vn, &mut bits_preserved);
        for guard in self.selectguards.iter_mut() {
            if guard.value_match(fd, vn, base_vn, bits_preserved) == 0 || guard.is_unrolled() {
                guard.clear(); // Indicate this guard was not used / not folded
            }
        }
    }

    /// Check if the given Varnode flows only into \b this model (C++
    /// `JumpBasic::flowsOnlyToModel`, `jumptable.cc:1292`).
    fn flows_only_to_model(&self, fd: &Funcdata, vn: VarnodeId, trail_op: Option<OpId>) -> bool {
        for op in fd.descend_snapshot(vn) {
            if Some(op) == trail_op {
                continue;
            }
            if !fd.obank().get(op).unwrap().is_mark() {
                return false;
            }
        }
        true
    }

    /// SEAM(structuring): unrolled-guard detection needs `BlockBasic::
    /// findMultiequal`/`liftVerifyUnroll` (structuring helpers not yet ported).
    /// A guard duplicated across multiple incoming blocks is left undetected; the
    /// switch still recovers via the straight-line path (the unrolled guard only
    /// adds an extra range constraint).  Recorded as a loss.
    fn check_unrolled_guard(
        &mut self,
        _fd: &Funcdata,
        _bl: BlockId,
        _maxpullback: int4,
        _usenzmask: bool,
    ) {
        // SEAM(structuring): checkCommonCbranch + findMultiequal + liftVerifyUnroll
    }

    /// Recover details of the model from the BRANCHIND (C++
    /// `JumpBasic::recoverModel`, `jumptable.cc:1565`).
    fn recover_model_basic(
        &mut self,
        fd: &mut Funcdata,
        indop: OpId,
        matchsize: uint4,
        maxtablesize: uint4,
    ) -> KunaResult<bool> {
        self.jrange = Some(Box::new(JumpValuesRange::new()));
        JumpBasic::find_determining_varnodes(fd, &mut self.path_meld, indop, 0);
        let parent = fd.obank().get(indop).unwrap().get_parent().unwrap();
        self.find_normalized(fd, parent, -1, matchsize, maxtablesize, indop)?;
        if self.jrange().get_size() > maxtablesize as uintb {
            // (kuna) GH-9191: the basic model could not bound the table.  When
            // `option switchmodbound on`, look for a modulo/and-mask bound on the
            // LOAD-table index and re-bound the table to [0, N).
            if fd.get_arch().switch_modulo_bound
                && self.kuna_try_modulo_bound_table(fd, maxtablesize)
            {
                self.mark_foldable_guards(fd);
                return Ok(true);
            }
            return Ok(false);
        }
        self.mark_foldable_guards(fd);
        Ok(true)
    }

    /// (kuna) GH-9191: bound a LOAD-table jump-table by a modulo/and-mask on its
    /// index when the basic model could not bound it (C++ header:
    /// `JumpBasic::kunaTryModuloBoundTable`).  Gated by `option switchmodbound`.
    ///
    /// Walks the melded path from the BRANCHIND toward the switch variable,
    /// allowing only the realigning ops `INT_ADD`/`INT_MULT`/`INT_LEFT`/
    /// `SUBPIECE`/`INT_ZEXT`/`INT_SEXT`/`INT_OR`/`COPY` (each with a constant
    /// other-input where binary), and a single table `LOAD`.  When it finds an
    /// `INT_REM`/`INT_SREM` by a constant `N` (resp. an `INT_AND` with a clean
    /// covering mask `mask`, bound `mask+1`) with `bound` in `[2, maxtablesize]`,
    /// it rebuilds the model so the index iterates over `[0, bound)`: the jrange
    /// start becomes the modulo/mask **result** varnode (the value is already in
    /// `[0, bound)` post-bound, so the bound op itself is skipped during
    /// emulation) and `varnodeIndex` points at it.
    ///
    /// Returns `true` if the table was re-bounded, `false` otherwise (the caller
    /// then declines the model exactly as before — upstream byte-identical when
    /// the gate is off, since this is only reached with the gate on).
    fn kuna_try_modulo_bound_table(&mut self, fd: &mut Funcdata, maxtablesize: uint4) -> bool {
        // Resolve a varnode to a constant value, tracing through a single
        // zero/sign-extension or copy (the SLEIGH lifter widens a small constant
        // divisor to the division width via INT_ZEXT/INT_SEXT).
        fn resolve_const(fd: &Funcdata, vn: VarnodeId) -> Option<uintb> {
            let v = fd.vbank().get(vn)?;
            if v.is_constant() {
                return Some(v.get_offset());
            }
            let def = v.get_def()?;
            let dop = fd.obank().get(def)?;
            match dop.code() {
                OpCode::CPUI_INT_ZEXT | OpCode::CPUI_INT_SEXT | OpCode::CPUI_COPY => {
                    let inv = dop.get_in(0)?;
                    let ivr = fd.vbank().get(inv)?;
                    if ivr.is_constant() {
                        Some(ivr.get_offset())
                    } else {
                        None
                    }
                }
                _ => None,
            }
        }

        // Scan the melded ops for the bounding op (INT_REM/INT_SREM/INT_AND).
        // The op_meld is in execution order from the BRANCHIND back; only the ops
        // strictly between the BRANCHIND and the bound participate, and they must
        // all be realigning ops or the single table LOAD.
        let num_ops = self.path_meld.num_ops();
        let mut load_seen = 0;
        let mut bound: Option<uintb> = None;
        let mut bound_out: Option<VarnodeId> = None;
        for i in 0..num_ops {
            let op = match self.path_meld.get_op(i) {
                Some(o) => o,
                None => continue,
            };
            let code = match fd.obank().get(op) {
                Some(o) => o.code(),
                None => return false,
            };
            match code {
                OpCode::CPUI_INT_REM | OpCode::CPUI_INT_SREM => {
                    let divisor = match fd.obank().get(op).and_then(|o| o.get_in(1)) {
                        Some(d) => d,
                        None => return false,
                    };
                    let n = match resolve_const(fd, divisor) {
                        Some(n) => n,
                        None => return false, // non-constant modulus: cannot bound
                    };
                    bound = Some(n);
                    bound_out = fd.obank().get(op).and_then(|o| o.get_out());
                    break;
                }
                OpCode::CPUI_INT_AND => {
                    // A clean covering mask 0b0..01..1 (mask = bound-1) bounds the
                    // index to [0, mask+1).
                    let other = match fd.obank().get(op).and_then(|o| o.get_in(1)) {
                        Some(d) => d,
                        None => return false,
                    };
                    let m = match resolve_const(fd, other) {
                        Some(m) => m,
                        None => return false,
                    };
                    // Require a covering mask (all set bits contiguous from bit 0).
                    if m == 0 || (m & m.wadd(1)) != 0 {
                        return false;
                    }
                    bound = Some(m.wadd(1));
                    bound_out = fd.obank().get(op).and_then(|o| o.get_out());
                    break;
                }
                // Realigning ops the index calculation may pass through.
                OpCode::CPUI_INT_ADD
                | OpCode::CPUI_INT_MULT
                | OpCode::CPUI_INT_LEFT
                | OpCode::CPUI_INT_OR
                | OpCode::CPUI_SUBPIECE
                | OpCode::CPUI_INT_ZEXT
                | OpCode::CPUI_INT_SEXT
                | OpCode::CPUI_COPY
                | OpCode::CPUI_BRANCHIND => {}
                OpCode::CPUI_LOAD => {
                    load_seen += 1;
                    if load_seen > 1 {
                        return false; // only a single table LOAD is supported
                    }
                }
                // Any other op in the path means this is not a clean modulo/mask
                // LOAD-table dispatch — decline.
                _ => return false,
            }
        }

        let bound = match bound {
            Some(b) => b,
            None => return false,
        };
        // A single table LOAD must be in the path (this is a LOAD-table dispatch,
        // not a direct computed jump).
        if load_seen != 1 {
            return false;
        }
        // bound must be in [2, maxtablesize].
        if bound < 2 || bound > maxtablesize as uintb {
            return false;
        }
        let bound_out = match bound_out {
            Some(v) => v,
            None => return false,
        };
        // Locate the bound-result varnode in the common path: emulation will start
        // there with the index already reduced to [0, bound).
        let mut pos: int4 = -1;
        for i in 0..self.path_meld.num_common_varnode() {
            if self.path_meld.get_varnode(i) == bound_out {
                pos = i;
                break;
            }
        }
        if pos < 0 {
            return false; // bound result is not a common path varnode
        }
        // The op that consumes the bound result (the next op toward the BRANCHIND)
        // is the emulation start op.
        let startop = match self.path_meld.get_earliest_op(pos) {
            Some(o) => o,
            None => return false,
        };
        let vsize = match fd.vbank().get(bound_out) {
            Some(v) => v.get_size(),
            None => return false,
        };
        self.varnode_index = pos;
        let jr = self.jrange_range_mut();
        // [0, bound) over the bound-result width, step 1.
        jr.set_range(CircleRange::new(0, bound, vsize, 1));
        jr.set_start_vn(bound_out);
        jr.set_start_op(startop);
        true
    }

    /// Build the explicit address table by emulating the switch calculation for
    /// each value in `jrange` (C++ `JumpBasic::buildAddresses`,
    /// `jumptable.cc:1588`).
    fn build_addresses_basic(
        &self,
        fd: &Funcdata,
        indop: OpId,
        addresstable: &mut Vec<Address>,
        mut loadpoints: Option<&mut Vec<LoadTable>>,
        mut loadcounts: Option<&mut Vec<int4>>,
    ) -> KunaResult<()> {
        addresstable.clear();
        let mut emul = EmulateFunction::new(fd);
        emul.set_load_collect(loadpoints.is_some());

        let mut mask: uintb = !0u64;
        let bit = fd.get_arch().funcptr_align;
        if bit != 0 {
            mask = (mask >> bit) << bit;
        }
        let spc = Rc::clone(fd.obank().get(indop).unwrap().get_addr().get_space().unwrap());

        let mut jr = self.jrange().clone_box();
        let mut notdone = jr.initialize_for_reading();
        while notdone {
            let val = jr.get_value();
            let startop = jr
                .get_start_op()
                .ok_or_else(|| KunaError::lowlevel("buildAddresses: no start op"))?;
            let startvn = jr
                .get_start_varnode()
                .ok_or_else(|| KunaError::lowlevel("buildAddresses: no start vn"))?;
            let mut addr = emul.emulate_path(val, &self.path_meld, startop, startvn)?;
            addr = AddrSpace::address_to_byte(addr, spc.get_word_size());
            addr &= mask;
            addresstable.push(Address::new(Rc::clone(&spc), addr));
            if let Some(lc) = loadcounts.as_mut() {
                let count = emul
                    .loadpoints_len()
                    .ok_or_else(|| KunaError::lowlevel("buildAddresses: loadcounts without loadpoints"))?;
                lc.push(count as int4);
            }
            notdone = jr.next()?;
        }
        // Hand back any collected LOAD records.
        if let Some(lp) = loadpoints.as_mut() {
            if let Some(collected) = emul.take_loadpoints() {
                lp.extend(collected);
            }
        }
        Ok(())
    }

    /// Recover the unnormalized switch variable (C++
    /// `JumpBasic::findUnnormalized`, `jumptable.cc:1616`).
    fn find_unnormalized_basic(
        &mut self,
        fd: &mut Funcdata,
        maxaddsub: uint4,
        _maxleftright: uint4,
        maxext: uint4,
    ) {
        let mut i = self.varnode_index;
        self.normalvn = Some(self.path_meld.get_varnode(i));
        i += 1;
        self.switchvn = self.normalvn;
        self.mark_model(fd, true);

        let mut countaddsub = 0;
        let mut countext = 0;
        let mut normop: Option<OpId> = None;
        while i < self.path_meld.num_common_varnode() {
            let switchvn = self.switchvn.unwrap();
            if !self.flows_only_to_model(fd, switchvn, normop) {
                break;
            }
            let testvn = self.path_meld.get_varnode(i);
            if !fd.vbank().get(switchvn).unwrap().is_written() {
                break;
            }
            let nop = fd.vbank().get(switchvn).unwrap().get_def().unwrap();
            normop = Some(nop);
            let ninput = fd.obank().get(nop).unwrap().num_input();
            let mut j = 0;
            while j < ninput {
                if fd.obank().get(nop).unwrap().get_in(j) == Some(testvn) {
                    break;
                }
                j += 1;
            }
            if j == ninput {
                break;
            }
            match fd.obank().get(nop).unwrap().code() {
                OpCode::CPUI_INT_ADD | OpCode::CPUI_INT_SUB => {
                    countaddsub += 1;
                    if countaddsub > maxaddsub {
                        break;
                    }
                    let otherin = fd.obank().get(nop).unwrap().get_in(1 - j).unwrap();
                    if !fd.vbank().get(otherin).unwrap().is_constant() {
                        break;
                    }
                    self.switchvn = Some(testvn);
                }
                OpCode::CPUI_INT_ZEXT | OpCode::CPUI_INT_SEXT => {
                    countext += 1;
                    if countext > maxext {
                        break;
                    }
                    self.switchvn = Some(testvn);
                }
                _ => {}
            }
            if self.switchvn != Some(testvn) {
                break;
            }
            i += 1;
        }
        self.mark_model(fd, false);
    }

    /// Build the case labels via reverse emulation (C++
    /// `JumpBasic::buildLabels`, `jumptable.cc:1660`).
    ///
    /// For each value in the original index range, [`backup2_switch`](Self::backup2_switch)
    /// reverse-emulates the normalization chain (`normalvn` → `switchvn`) to
    /// recover the original switch-statement label.  A non-reversible value (the
    /// default/exit path) labels `NO_LABEL`.
    fn build_labels_basic(
        &self,
        fd: &mut Funcdata,
        addresstable: &mut [Address],
        label: &mut Vec<uintb>,
        orig: &JumpBasicModel,
    ) -> KunaResult<()> {
        let mut origrange = orig.jrange().clone_box();
        let mut notdone = origrange.initialize_for_reading();
        while notdone {
            let val = origrange.get_value();
            let switchval = if origrange.is_reversible() {
                // Reverse-emulate the normalization to get the original switch
                // value.  A failed reversal labels NO_LABEL (the default/exit
                // path or an un-invertible op; C++ warns + NO_LABEL).
                match self.backup2_switch(fd, val) {
                    Ok(v) => v,
                    Err(_) => NO_LABEL,
                }
            } else {
                NO_LABEL // If can't reverse, hopefully default/exit
            };
            label.push(switchval);
            // The address table may have been truncated by the sanity check.
            if label.len() >= addresstable.len() {
                break;
            }
            notdone = origrange.next()?;
        }
        while label.len() < addresstable.len() {
            label.push(NO_LABEL);
        }
        Ok(())
    }

    /// Reverse-emulate the switch normalization to recover the original switch
    /// value from a normalized index value (C++ `JumpBasic::backup2Switch`,
    /// `jumptable.cc:1639`).
    ///
    /// Walks from `normalvn` back to `switchvn`, inverting each normalization op
    /// via the architecture's [`OpBehavior`](kuna_num::opbehavior::OpBehavior)
    /// `recover_input_unary`/`recover_input_binary`.  The normalization is
    /// add/sub/zext/sext/... over a constant, so `recoverInput*` is exact.
    fn backup2_switch(&self, fd: &Funcdata, output_in: uintb) -> KunaResult<uintb> {
        let invn = self
            .switchvn
            .ok_or_else(|| KunaError::lowlevel("backup2Switch: no switchvn"))?;
        let mut curvn = self
            .normalvn
            .ok_or_else(|| KunaError::lowlevel("backup2Switch: no normalvn"))?;
        let mut output = output_in;
        while curvn != invn {
            let op = fd
                .vbank()
                .get(curvn)
                .and_then(|v| v.get_def())
                .ok_or_else(|| KunaError::lowlevel("backup2Switch: normalvn not written"))?;
            let opc = fd.obank().get(op).unwrap().code();
            let numin = fd.obank().get(op).unwrap().num_input();
            // Find first non-constant input slot.
            let mut slot = 0;
            while slot < numin {
                let invn_s = fd.obank().get(op).unwrap().get_in(slot).unwrap();
                if !fd.vbank().get(invn_s).unwrap().is_constant() {
                    break;
                }
                slot += 1;
            }
            let behave = fd
                .get_arch()
                .op_behavior(opc)
                .ok_or_else(|| KunaError::lowlevel("backup2Switch: no op behavior"))?;
            let eval = fd.obank().get(op).unwrap().get_eval_type();
            use crate::op::pcodeop_flags as pf;
            if eval & pf::binary != 0 {
                let otherslot = 1 - slot;
                let othervn = fd.obank().get(op).unwrap().get_in(otherslot).unwrap();
                let otheraddr = fd.vbank().get(othervn).unwrap().get_addr().clone();
                let othersize = fd.vbank().get(othervn).unwrap().get_size();
                let otherval = if otheraddr.is_constant() {
                    otheraddr.get_offset()
                } else {
                    // C++ reads a MemoryImage(addr.getSpace(),4,1024,loader); the
                    // other operand is a read-only/global constant load.
                    fd.get_arch().get_load_image_value(&otheraddr, othersize)?
                };
                let sizeout = fd
                    .obank()
                    .get(op)
                    .unwrap()
                    .get_out()
                    .map(|o| fd.vbank().get(o).unwrap().get_size())
                    .unwrap_or(0);
                let slotvn = fd.obank().get(op).unwrap().get_in(slot).unwrap();
                let sizein = fd.vbank().get(slotvn).unwrap().get_size();
                output = behave.recover_input_binary(slot, sizeout, output, sizein, otherval)?;
                curvn = slotvn;
            } else if eval & pf::unary != 0 {
                let sizeout = fd
                    .obank()
                    .get(op)
                    .unwrap()
                    .get_out()
                    .map(|o| fd.vbank().get(o).unwrap().get_size())
                    .unwrap_or(0);
                let slotvn = fd.obank().get(op).unwrap().get_in(slot).unwrap();
                let sizein = fd.vbank().get(slotvn).unwrap().get_size();
                output = behave.recover_input_unary(sizeout, output, sizein)?;
                curvn = slotvn;
            } else {
                return Err(KunaError::lowlevel("Bad switch normalization op"));
            }
        }
        Ok(output)
    }
}

impl Default for JumpBasicModel {
    fn default() -> Self {
        Self::new()
    }
}

impl JumpModel for JumpBasicModel {
    fn is_override(&self) -> bool {
        false
    }

    fn get_table_size(&self) -> int4 {
        self.jrange().get_size() as int4
    }

    fn recover_model(
        &mut self,
        fd: &mut Funcdata,
        indop: OpId,
        matchsize: uint4,
        maxtablesize: uint4,
    ) -> KunaResult<bool> {
        if self.is_model2 {
            self.recover_model2(fd, indop, matchsize, maxtablesize)
        } else {
            self.recover_model_basic(fd, indop, matchsize, maxtablesize)
        }
    }

    fn build_addresses(
        &self,
        fd: &Funcdata,
        indop: OpId,
        addresstable: &mut Vec<Address>,
        loadpoints: Option<&mut Vec<LoadTable>>,
        loadcounts: Option<&mut Vec<int4>>,
    ) -> KunaResult<()> {
        self.build_addresses_basic(fd, indop, addresstable, loadpoints, loadcounts)
    }

    fn find_unnormalized(
        &mut self,
        fd: &mut Funcdata,
        maxaddsub: uint4,
        maxleftright: uint4,
        maxext: uint4,
    ) -> KunaResult<()> {
        if self.is_model2 {
            self.find_unnormalized2(fd, maxaddsub, maxleftright, maxext)
        } else {
            self.find_unnormalized_basic(fd, maxaddsub, maxleftright, maxext);
            Ok(())
        }
    }

    fn build_labels(
        &self,
        fd: &mut Funcdata,
        addresstable: &mut [Address],
        label: &mut Vec<uintb>,
        orig: &dyn JumpModel,
    ) -> KunaResult<()> {
        // orig is always a JumpBasicModel (the C++ casts to const JumpBasic*).
        let orig = orig
            .as_any()
            .downcast_ref::<JumpBasicModel>()
            .ok_or_else(|| KunaError::lowlevel("buildLabels: orig is not a JumpBasic model"))?;
        self.build_labels_basic(fd, addresstable, label, orig)
    }

    fn fold_in_normalization(
        &mut self,
        fd: &mut Funcdata,
        indop: OpId,
    ) -> KunaResult<Option<VarnodeId>> {
        // Set the BRANCHIND input to the unnormalized switch variable, so the
        // intervening address calculation becomes dead.
        let switchvn = self
            .switchvn
            .ok_or_else(|| KunaError::lowlevel("foldInNormalization: switchvn not recovered"))?;
        fd.op_set_input(indop, switchvn, 0)?;
        Ok(Some(switchvn))
    }

    fn fold_in_guards(&mut self, fd: &mut Funcdata, jump: &mut JumpTable) -> KunaResult<bool> {
        // C++ JumpBasic::foldInGuards (jumptable.cc:1709).
        //   bool change = false;
        //   for(i=0;i<selectguards.size();++i) {
        //     cbranch = selectguards[i].getBranch();
        //     if (cbranch == 0) continue;        // already normalized
        //     if (cbranch->isDead()) { selectguards[i].clear(); continue; }
        //     if (foldInOneGuard(fd,selectguards[i],jump)) change = true;
        //   }
        //   return change;
        //
        // The `pos == nout` arm of foldInOneGuard converts the guard CBRANCH into
        // an unconditional BRANCH via pushBranch (no residue).  The `pos != nout`
        // arm sets the guard CBRANCH predicate to a constant (an `if (1)`/`if (0)`
        // residue); C++ collapses that with a SUBSEQUENT pass — ActionSwitchNorm
        // (coreaction.cc:4793) returns a positive count when a guard folds AND does
        // `getStructure().clear()`, which re-runs `actfullloop` (rule_repeatapply),
        // so `ActionDeterminedBranch` (coreaction.cc:3688, in actmainloop) then
        // severs the now-constant CBRANCH's dead edge.  The Rust port re-runs the
        // loop identically; the only missing piece was that `removeBranch` was a
        // seam in `ActionDeterminedBranch::apply` — now wired (coreaction_early.rs),
        // so the `if (1)` collapses on the re-run exactly as upstream.
        let mut change = false;
        let nguards = self.selectguards.len();
        for i in 0..nguards {
            let cbranch = match self.selectguards[i].get_branch() {
                Some(c) => c,
                None => continue, // Already normalized
            };
            let is_dead = fd.obank().get(cbranch).map(|o| o.is_dead()).unwrap_or(true);
            if is_dead {
                self.selectguards[i].clear();
                continue;
            }
            // foldInOneGuard mutates the guard record in place (`fold_in_one_guard`
            // is an associated fn — it borrows the single guard, `fd`, and `jump`,
            // all disjoint from the rest of `self`).
            if Self::fold_in_one_guard(fd, &mut self.selectguards[i], jump)? {
                change = true;
            }
        }
        Ok(change)
    }

    fn sanity_check(
        &mut self,
        fd: &mut Funcdata,
        _indop: OpId,
        addresstable: &mut Vec<Address>,
        loadpoints: &mut Vec<LoadTable>,
        loadcounts: Option<&Vec<int4>>,
    ) -> KunaResult<bool> {
        // C++ JumpBasic::sanityCheck (jumptable.cc:1726).
        if addresstable.is_empty() {
            return Ok(true);
        }
        let addr0 = addresstable[0].clone();
        let mut i = 0usize;
        if addr0.get_offset() != 0 {
            i = 1;
            while i < addresstable.len() {
                if addresstable[i].get_offset() == 0 {
                    break;
                }
                let oi = addresstable[i].get_offset();
                let diff = if addr0.get_offset() < oi {
                    oi.wsub(addr0.get_offset())
                } else {
                    addr0.get_offset().wsub(oi)
                };
                if diff > 0xffff {
                    // Far address: require the load image to have data there.
                    let dataavail = fd.get_arch().get_load_image_value(&addresstable[i], 4).is_ok();
                    if !dataavail {
                        break;
                    }
                }
                i += 1;
            }
        }
        if i == 0 {
            return Ok(false);
        }
        if i != addresstable.len() {
            addresstable.truncate(i);
            self.jrange_range_mut().truncate(i as int4);
            if let Some(lc) = loadcounts {
                if i >= 1 && (i - 1) < lc.len() {
                    loadpoints.truncate(lc[i - 1] as usize);
                }
            }
        }
        Ok(true)
    }

    fn clone_box(&self) -> Box<dyn JumpModel> {
        // C++ JumpBasic::clone only clones the JumpValues iterator.
        let mut res = JumpBasicModel::new();
        res.is_model2 = self.is_model2;
        if let Some(jr) = &self.jrange {
            res.jrange = Some(jr.clone_box());
        }
        Box::new(res)
    }

    fn clear(&mut self) {
        self.jrange = None;
        self.path_meld.clear();
        self.selectguards.clear();
        self.normalvn = None;
        self.switchvn = None;
        self.extravn = None;
        self.orig_path_meld.clear();
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl JumpBasicModel {
    /// Fold a single guard CBRANCH into the switch's default destination (C++
    /// `JumpBasic::foldInOneGuard`, `jumptable.cc`).
    ///
    /// If the guard branch goes directly into the switch block, route its other
    /// (out-of-range) target through the switch as the folded default — either by
    /// adding it as a new no-label destination (and converting the CBRANCH to an
    /// unconditional BRANCH into the switch via `pushBranch`) or, when the target
    /// is already a switch destination, by making the CBRANCH unconditional toward
    /// the switch and recording that destination as the default block.
    ///
    /// Reached from `fold_in_guards` (the constant-predicate `if (1)` residue the
    /// `pos != nout` arm leaves is collapsed in place here, see that arm).
    fn fold_in_one_guard(
        fd: &mut Funcdata,
        guard: &mut GuardRecord,
        jump: &mut JumpTable,
    ) -> KunaResult<bool> {
        let cbranch = match guard.get_branch() {
            Some(c) => c,
            None => return Ok(false),
        };
        let cbranchblock = match fd.obank().get(cbranch).and_then(|o| o.get_parent()) {
            Some(b) => b,
            None => return Ok(false),
        };
        // The guard branch may have been converted between recovery and now.
        if fd.bblocks_ref().block(cbranchblock).size_out() != 2 {
            return Ok(false);
        }
        let mut indpath = guard.get_path();
        if fd.bblocks_ref().block(cbranchblock).get_flip_path() {
            indpath = 1 - indpath;
        }
        let switchbl = match jump.get_indirect_op().and_then(|io| fd.obank().get(io)).and_then(|o| o.get_parent()) {
            Some(b) => b,
            None => return Ok(false),
        };
        // Guard must go directly into the switch block.
        if fd.bblocks_ref().block(cbranchblock).get_out(indpath) != switchbl {
            return Ok(false);
        }
        let guardtarget = fd.bblocks_ref().block(cbranchblock).get_out(1 - indpath);

        let nout = fd.bblocks_ref().block(switchbl).size_out();
        let mut pos = 0;
        while pos < nout {
            if fd.bblocks_ref().block(switchbl).get_out(pos) == guardtarget {
                break;
            }
            pos += 1;
        }
        // There can be only one folded target.
        if jump.has_folded_default() && jump.get_default_block() != pos {
            return Ok(false);
        }
        if !fd.block_no_intervening_statement(switchbl) {
            return Ok(false);
        }
        if pos == nout {
            // Add the new destination to the table without a label and treat it as
            // the default/exit case; route the guard branch into the switch.
            jump.add_block_to_switch(fd, guardtarget, NO_LABEL)?;
            jump.set_last_as_default();
            fd.push_branch(cbranchblock, 1 - indpath, switchbl)?;
        } else {
            // The guard's out-of-range target is already a switch destination:
            // make the CBRANCH unconditional toward the switch (set the predicate
            // constant) and record that block as the default.
            let is_flip = fd.obank().get(cbranch).map(|o| o.is_boolean_flip()).unwrap_or(false);
            let val: uintb = if (indpath == 0) != is_flip { 0 } else { 1 };
            let in0_size = fd
                .obank()
                .get(cbranch)
                .and_then(|o| o.get_in(0))
                .and_then(|v| fd.vbank().get(v))
                .map(|v| v.get_size())
                .unwrap_or(1);
            let c = fd.new_constant(in0_size, val);
            fd.op_set_input(cbranch, c, 1)?;
            jump.set_default_block(pos);
        }
        jump.set_folded_default();
        guard.clear();
        Ok(true)
    }

    /// Construct an empty model-2 (C++ `JumpBasic2(JumpTable*)`).
    pub fn new_model2() -> JumpBasicModel {
        let mut m = JumpBasicModel::new();
        m.is_model2 = true;
        m
    }

    /// Pass in the prior PathMeld calculation (C++ `JumpBasic2::initializeStart`,
    /// `jumptable.cc:1805`).
    pub fn initialize_start(&mut self, pmeld: &PathMeld) {
        if pmeld.empty() {
            self.extravn = None;
            return;
        }
        self.extravn = Some(pmeld.get_varnode(pmeld.num_common_varnode() - 1));
        self.orig_path_meld.set_from(pmeld);
    }

    /// Recover the model-2 (default-path) switch (C++ `JumpBasic2::recoverModel`,
    /// `jumptable.cc:1817`).
    fn recover_model2(
        &mut self,
        fd: &mut Funcdata,
        _indop: OpId,
        matchsize: uint4,
        maxtablesize: uint4,
    ) -> KunaResult<bool> {
        let joinvn = match self.extravn {
            None => return Ok(false),
            Some(v) => v,
        };
        if !fd.vbank().get(joinvn).unwrap().is_written() {
            return Ok(false);
        }
        let multiop = fd.vbank().get(joinvn).unwrap().get_def().unwrap();
        if fd.obank().get(multiop).unwrap().code() != OpCode::CPUI_MULTIEQUAL {
            return Ok(false);
        }
        if fd.obank().get(multiop).unwrap().num_input() != 2 {
            return Ok(false);
        }
        // Search for a constant along one of the paths.
        let mut extravalue = 0;
        let mut path = 2;
        for p in 0..2 {
            let vn = fd.obank().get(multiop).unwrap().get_in(p).unwrap();
            if !fd.vbank().get(vn).unwrap().is_written() {
                continue;
            }
            let copyop = fd.vbank().get(vn).unwrap().get_def().unwrap();
            if fd.obank().get(copyop).unwrap().code() != OpCode::CPUI_COPY {
                continue;
            }
            let othervn = fd.obank().get(copyop).unwrap().get_in(0).unwrap();
            if fd.vbank().get(othervn).unwrap().is_constant() {
                extravalue = fd.vbank().get(othervn).unwrap().get_offset();
                path = p;
                break;
            }
        }
        if path == 2 {
            return Ok(false);
        }
        let mparent = fd.obank().get(multiop).unwrap().get_parent().unwrap();
        let rootbl = fd.bblocks_ref().block(mparent).get_in(1 - path);
        let pathout = fd.bblocks_ref().block(mparent).get_in_rev_index(1 - path);

        let mut jdef = JumpValuesRangeDefault::new();
        jdef.set_extra_value(extravalue);
        jdef.set_default_vn(joinvn);
        let defop = self
            .orig_path_meld
            .get_op(self.orig_path_meld.num_ops() - 1)
            .ok_or_else(|| KunaError::lowlevel("JumpBasic2: orig path empty"))?;
        jdef.set_default_op(defop);
        self.jrange = Some(Box::new(jdef));

        JumpBasic::find_determining_varnodes(fd, &mut self.path_meld, multiop, 1 - path);
        self.find_normalized(fd, rootbl, pathout, matchsize, maxtablesize, _indop)?;
        if self.jrange().get_size() > maxtablesize as uintb {
            return Ok(false);
        }
        // Insert the final sequence after the MULTIEQUAL.
        let orig = self.orig_path_meld.clone();
        self.path_meld.append(&orig);
        self.varnode_index += self.orig_path_meld.num_common_varnode();
        Ok(true)
    }

    /// Model-2 unnormalized switch recovery (C++ `JumpBasic2::findUnnormalized`,
    /// `jumptable.cc:1887`).
    fn find_unnormalized2(
        &mut self,
        fd: &mut Funcdata,
        maxaddsub: uint4,
        maxleftright: uint4,
        maxext: uint4,
    ) -> KunaResult<()> {
        self.normalvn = Some(self.path_meld.get_varnode(self.varnode_index));
        if self.check_normal_dominance(fd) {
            self.find_unnormalized_basic(fd, maxaddsub, maxleftright, maxext);
            return Ok(());
        }
        // Go backward from the unnormalized variable to the normalized variable.
        let extravn = self.extravn.unwrap();
        self.switchvn = Some(extravn);
        let multiop = fd.vbank().get(extravn).unwrap().get_def().unwrap();
        let in0 = fd.obank().get(multiop).unwrap().get_in(0);
        let in1 = fd.obank().get(multiop).unwrap().get_in(1);
        if in0 == self.normalvn || in1 == self.normalvn {
            self.normalvn = self.switchvn;
            Ok(())
        } else {
            Err(KunaError::lowlevel("Backward normalization not implemented"))
        }
    }

    /// Check if the block defining the normalized var dominates the switch (C++
    /// `JumpBasic2::checkNormalDominance`, `jumptable.cc:1872`).
    fn check_normal_dominance(&self, fd: &Funcdata) -> bool {
        let normalvn = self.normalvn.unwrap();
        if fd.vbank().get(normalvn).unwrap().is_input() {
            return true;
        }
        let defblock = fd
            .obank()
            .get(fd.vbank().get(normalvn).unwrap().get_def().unwrap())
            .unwrap()
            .get_parent();
        let defblock = match defblock {
            None => return false,
            Some(b) => b,
        };
        let mut switchblock = self.path_meld.get_op(0).and_then(|o| fd.obank().get(o).unwrap().get_parent());
        while let Some(sb) = switchblock {
            if sb == defblock {
                return true;
            }
            switchblock = fd.bblocks_ref().block(sb).get_immed_dom();
        }
        false
    }
}

// ---------------------------------------------------------------------------
// JumpTable (jumptable.hh:542, jumptable.cc:2379-3016)
// ---------------------------------------------------------------------------

/// Recovery status for a specific JumpTable (C++ `JumpTable::RecoveryMode`,
/// `jumptable.hh:545`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecoveryMode {
    /// JumpTable is fully recovered.
    Success,
    /// Normal failure to recover.
    FailNormal,
    /// Likely \b thunk.
    FailThunk,
    /// Likely \b return operation.
    FailReturn,
    /// Address formed by CALLOTHER.
    FailCallother,
}

/// An address table index and its corresponding out-edge (C++
/// `JumpTable::IndexPair`, `jumptable.hh:557`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IndexPair {
    /// Out-edge index for the basic-block (C++ `IndexPair::blockPosition`).
    pub block_position: int4,
    /// Index of address targeting the basic-block (C++ `IndexPair::addressIndex`).
    pub address_index: int4,
}

impl IndexPair {
    /// Construct a pair (C++ `IndexPair(int4 pos,int4 index)`).
    pub fn new(block_position: int4, address_index: int4) -> IndexPair {
        IndexPair { block_position, address_index }
    }
}

impl PartialOrd for IndexPair {
    fn partial_cmp(&self, op2: &IndexPair) -> Option<std::cmp::Ordering> {
        Some(self.cmp(op2))
    }
}

impl Ord for IndexPair {
    /// Compare by position then by index (C++ `IndexPair::operator<`,
    /// `jumptable.hh:634`).
    fn cmp(&self, op2: &IndexPair) -> std::cmp::Ordering {
        if self.block_position != op2.block_position {
            return self.block_position.cmp(&op2.block_position);
        }
        self.address_index.cmp(&op2.address_index)
    }
}

/// A map from values to control-flow targets within a function (C++ `JumpTable`,
/// `jumptable.hh:542`).
///
/// Attached to a specific `CPUI_BRANCHIND`; encapsulates the information needed
/// to model the indirect jump as a \e switch.
pub struct JumpTable {
    /// Maximum ADDs or SUBs to normalize (C++ `JumpTable::maxaddsub`).
    pub(crate) maxaddsub: uint4,
    /// Maximum shifts to normalize (C++ `JumpTable::maxleftright`).
    pub(crate) maxleftright: uint4,
    /// Maximum extensions to normalize (C++ `JumpTable::maxext`).
    pub(crate) maxext: uint4,
    /// Current model of how the jump table is implemented in code.
    jmodel: Option<Box<dyn JumpModel>>,
    /// Initial jump table model, which may be incomplete (C++ `origmodel`).
    origmodel: Option<Box<dyn JumpModel>>,
    /// Raw addresses in the jump-table (C++ `addresstable`).
    addresstable: Vec<Address>,
    /// Map from basic-blocks to address table index (C++ `block2addr`).
    block2addr: Vec<IndexPair>,
    /// The case label for each explicit target (C++ `label`).
    label: Vec<uintb>,
    /// Any recovered in-memory data for the jump-table (C++ `loadpoints`).
    loadpoints: Vec<LoadTable>,
    /// Absolute address of the BRANCHIND jump (C++ `opaddress`).
    opaddress: Address,
    /// CPUI_BRANCHIND linked to \b this jump-table (C++ `indirect`).
    indirect: Option<OpId>,
    /// Bits of the switch variable being consumed (C++ `switchVarConsume`).
    switch_var_consume: uintb,
    /// The out-edge corresponding to the \e default switch destination (-1 =
    /// undefined) (C++ `defaultBlock`).
    default_block: int4,
    /// Block out-edge corresponding to last entry in the address table.
    last_block: int4,
    /// Number of times recovery attempted on \b this table (C++ `recoverCount`).
    recover_count: int4,
    /// Display format for integer \e case values (C++ `displayFormat`).
    display_format: uint4,
    /// Set to \b true if \b this table is incomplete (C++ `partialTable`).
    partial_table: bool,
    /// Set to \b true if in-memory model data should be collected
    /// (C++ `collectloads`).
    collectloads: bool,
    /// The \e default block is the target of a folded CBRANCH (cannot have a
    /// label) (C++ `defaultIsFolded`).
    default_is_folded: bool,
}

impl JumpTable {
    /// Construct with the address of the BRANCHIND being modeled
    /// (C++ `JumpTable::JumpTable(Address ad)`, `jumptable.cc:2534`).
    pub fn new(ad: Address) -> JumpTable {
        JumpTable {
            maxaddsub: 1,
            maxleftright: 1,
            maxext: 1,
            jmodel: None,
            origmodel: None,
            addresstable: Vec::new(),
            block2addr: Vec::new(),
            label: Vec::new(),
            loadpoints: Vec::new(),
            opaddress: ad,
            indirect: None,
            switch_var_consume: !0u64,
            default_block: -1,
            last_block: -1,
            recover_count: 0,
            display_format: 0,
            partial_table: false,
            collectloads: false,
            default_is_folded: false,
        }
    }

    /// Clone from another table (C++ `JumpTable::JumpTable(const JumpTable*)`,
    /// `jumptable.cc:2541`): copy the recovered addresses/loadpoints and the
    /// permanent fields, clone the model's value iterator, but reset the per-
    /// instance state (`indirect`/`origmodel`/`block2addr`/`default_block`).
    pub fn new_clone(op2: &JumpTable) -> JumpTable {
        JumpTable {
            maxaddsub: op2.maxaddsub,
            maxleftright: op2.maxleftright,
            maxext: op2.maxext,
            jmodel: op2.jmodel.as_ref().map(|m| m.clone_box()),
            origmodel: None,
            addresstable: op2.addresstable.clone(),
            block2addr: Vec::new(),
            label: Vec::new(),
            loadpoints: op2.loadpoints.clone(),
            opaddress: op2.opaddress.clone(),
            indirect: None,
            switch_var_consume: !0u64,
            default_block: -1,
            last_block: op2.last_block,
            recover_count: op2.recover_count,
            display_format: op2.display_format,
            partial_table: op2.partial_table,
            collectloads: op2.collectloads,
            default_is_folded: false,
        }
    }

    /// Return \b true if a model has been recovered (C++ `isRecovered`).
    pub fn is_recovered(&self) -> bool {
        !self.addresstable.is_empty()
    }

    /// Return \b true if \e case labels are computed (C++ `isLabelled`).
    pub fn is_labelled(&self) -> bool {
        !self.label.is_empty()
    }

    /// Return \b true if \b this table was manually overridden (C++ `isOverride`,
    /// `jumptable.cc:2597`).
    pub fn is_override(&self) -> bool {
        match &self.jmodel {
            None => false,
            Some(m) => m.is_override(),
        }
    }

    /// Return \b true if \b this is a partial table needing more recovery
    /// (C++ `isPartial`).
    pub fn is_partial(&self) -> bool {
        self.partial_table
    }

    /// Mark whatever is recovered so far as the complete table (C++ `markComplete`).
    pub fn mark_complete(&mut self) {
        self.partial_table = false;
    }

    /// Return the size of the address table for \b this jump-table
    /// (C++ `numEntries`).
    pub fn num_entries(&self) -> int4 {
        self.addresstable.len() as int4
    }

    /// Get bits of switch variable consumed by \b this table
    /// (C++ `getSwitchVarConsume`).
    pub fn get_switch_var_consume(&self) -> uintb {
        self.switch_var_consume
    }

    /// Get the out-edge corresponding to the \e default switch destination
    /// (C++ `getDefaultBlock`).
    pub fn get_default_block(&self) -> int4 {
        self.default_block
    }

    /// Get the address of the BRANCHIND for the switch (C++ `getOpAddress`).
    pub fn get_op_address(&self) -> &Address {
        &self.opaddress
    }

    /// Get the BRANCHIND PcodeOp (C++ `getIndirectOp`).
    pub fn get_indirect_op(&self) -> Option<OpId> {
        self.indirect
    }

    /// Set the BRANCHIND PcodeOp (C++ `setIndirectOp`).
    pub fn set_indirect_op(&mut self, fd: &Funcdata, ind: OpId) {
        self.opaddress = fd.obank().get(ind).unwrap().get_addr().clone();
        self.indirect = Some(ind);
    }

    /// Set the BRANCHIND PcodeOp with its address supplied directly (C++
    /// `setIndirectOp`, for callers already holding the op's address while
    /// `&mut self` aliases the function — the recovery chain's `linkJumpTable`/
    /// `truncatedFlow`).
    pub fn set_indirect_op_addr(&mut self, ind: OpId, addr: Address) {
        self.opaddress = addr;
        self.indirect = Some(ind);
    }

    /// Get the display format for integer cases (C++ `getDisplayFormat`).
    pub fn get_display_format(&self) -> uint4 {
        self.display_format
    }

    /// Set the display format to use for integer case values (C++ `setDisplayFormat`).
    pub fn set_display_format(&mut self, format: uint4) {
        self.display_format = format;
    }

    /// Get the i-th address table entry (C++ `getAddressByIndex`).
    pub fn get_address_by_index(&self, i: int4) -> Address {
        self.addresstable[i as usize].clone()
    }

    /// Get number of times a recovery has been attempted (C++ `getRecoverCount`).
    pub fn get_recover_count(&self) -> int4 {
        self.recover_count
    }

    /// Record that another round of recovery is being attempted
    /// (C++ `incrementRecoveryCount`).
    pub fn increment_recovery_count(&mut self) {
        self.recover_count += 1;
    }

    /// Set the \e default jump-table target to be the last address in the table
    /// (C++ `setLastAsDefault`, `jumptable.cc:2652`).
    pub fn set_last_as_default(&mut self) {
        self.default_block = self.last_block;
    }

    /// Set out-edge of the switch destination considered to be \e default
    /// (C++ `setDefaultBlock`).
    pub fn set_default_block(&mut self, bl: int4) {
        self.default_block = bl;
    }

    /// Set whether LOAD records should be collected (C++ `setLoadCollect`).
    pub fn set_load_collect(&mut self, val: bool) {
        self.collectloads = val;
    }

    /// Mark that the \e default block is a folded CBRANCH target (C++ `setFoldedDefault`).
    pub fn set_folded_default(&mut self) {
        self.default_is_folded = true;
    }

    /// Return \b true if the \e default block is a folded CBRANCH target
    /// (C++ `hasFoldedDefault`).
    pub fn has_folded_default(&self) -> bool {
        self.default_is_folded
    }

    /// Given a \e case index, get its label (C++ `getLabelByIndex`).
    pub fn get_label_by_index(&self, index: int4) -> uintb {
        self.label[index as usize]
    }

    /// Force a given basic-block to be a switch destination
    /// (C++ `JumpTable::addBlockToSwitch`, `jumptable.cc:2663`).
    pub fn add_block_to_switch(&mut self, fd: &Funcdata, bl: BlockId, lab: uintb) -> KunaResult<()> {
        self.addresstable.push(block_get_start(&fd.bblocks_ref().arena, bl));
        let parent = fd
            .obank()
            .get(self.indirect.unwrap())
            .unwrap()
            .get_parent()
            .ok_or_else(|| KunaError::lowlevel("addBlockToSwitch: BRANCHIND has no parent"))?;
        // The block WILL be added to the end of the out-edges
        self.last_block = fd.bblocks_ref().block(parent).size_out();
        self.block2addr
            .push(IndexPair::new(self.last_block, self.addresstable.len() as int4 - 1));
        self.label.push(lab);
        Ok(())
    }

    /// Return the number of address table entries that target the given block
    /// (C++ `JumpTable::numIndicesByBlock`, `jumptable.cc:2588`).
    pub fn num_indices_by_block(&self, fd: &Funcdata, bl: BlockId) -> KunaResult<int4> {
        let pos = self.block2_position(fd, bl)?;
        // equal_range over block2addr by compareByPosition
        let mut count = 0;
        for ip in self.block2addr.iter() {
            if ip.block_position == pos {
                count += 1;
            }
        }
        Ok(count)
    }

    /// Get the index of the i-th address table entry corresponding to the given
    /// block (C++ `JumpTable::getIndexByBlock`, `jumptable.cc:2635`).
    pub fn get_index_by_block(&self, fd: &Funcdata, bl: BlockId, i: int4) -> KunaResult<int4> {
        let block_pos = self.block2_position(fd, bl)?;
        let mut count = 0;
        // block2addr is sorted by IndexPair order; iterate in order.
        for ip in self.block2addr.iter() {
            if ip.block_position == block_pos {
                if count == i {
                    return Ok(ip.address_index);
                }
                count += 1;
            }
        }
        Err(KunaError::lowlevel("Could not get jumptable index for block"))
    }

    /// Convert a basic-block to an out-edge index from the switch
    /// (C++ `JumpTable::block2Position`, `jumptable.cc:2492`).
    fn block2_position(&self, fd: &Funcdata, bl: BlockId) -> KunaResult<int4> {
        let parent = fd
            .obank()
            .get(self.indirect.unwrap())
            .unwrap()
            .get_parent()
            .ok_or_else(|| KunaError::lowlevel("block2Position: BRANCHIND has no parent"))?;
        let size_in = fd.bblocks_ref().block(bl).size_in();
        let mut position = 0;
        while position < size_in {
            if fd.bblocks_ref().block(bl).get_in(position) == parent {
                break;
            }
            position += 1;
        }
        if position == size_in {
            return Err(KunaError::lowlevel("Requested block, not in jumptable"));
        }
        Ok(fd.bblocks_ref().block(bl).get_in_rev_index(position))
    }

    /// Set out-edge of the last entry to the default; trivial-switch mapping
    /// (C++ `JumpTable::trivialSwitchOver`, `jumptable.cc:2744`).
    pub fn trivial_switch_over(&mut self, fd: &Funcdata) -> KunaResult<()> {
        self.block2addr.clear();
        self.block2addr.reserve(self.addresstable.len());
        let parent = fd
            .obank()
            .get(self.indirect.unwrap())
            .unwrap()
            .get_parent()
            .ok_or_else(|| KunaError::lowlevel("trivialSwitchOver: BRANCHIND has no parent"))?;
        let nout = fd.bblocks_ref().block(parent).size_out();
        if nout as usize != self.addresstable.len() {
            return Err(KunaError::lowlevel(
                "Trivial addresstable and switch block size do not match",
            ));
        }
        for i in 0..nout {
            // Addresses corresponds exactly to out-edges of switch block
            self.block2addr.push(IndexPair::new(i, i));
        }
        self.last_block = nout - 1;
        self.default_block = -1; // Trivial case does not have default case
        Ok(())
    }

    /// Set the BRANCHIND input to the unnormalized switch variable, eliding the
    /// address computation (C++ `JumpTable::foldInNormalization`,
    /// `jumptable.cc:2724`).
    ///
    /// SEAM(W5): `minimalmask`/nzmask exist, but this calls
    /// `jmodel->foldInNormalization`, whose `JumpBasic` body needs the model
    /// state (`switchvn`) recovered by the W5-dependent `recoverModel`.  The
    /// driver is faithful; the model body returns the seam `Err`.
    pub fn fold_in_normalization(&mut self, fd: &mut Funcdata) -> KunaResult<()> {
        let indirect = self.indirect.unwrap();
        let model = self
            .jmodel
            .as_mut()
            .ok_or_else(|| KunaError::lowlevel("foldInNormalization: no model"))?;
        let switchvn = model.fold_in_normalization(fd, indirect)?;
        if let Some(switchvn) = switchvn {
            // If possible, mark up the switch variable as not fully consumed so
            // that subvariable flow can truncate it.
            let nz = fd.vbank().get(switchvn).unwrap().get_nz_mask();
            self.switch_var_consume = kuna_base::address::minimalmask(nz);
            let sz = fd.vbank().get(switchvn).unwrap().get_size();
            if self.switch_var_consume >= kuna_base::address::calc_mask(sz) {
                // If mask covers everything
                if fd.vbank().get(switchvn).unwrap().is_written() {
                    let op = fd.vbank().get(switchvn).unwrap().get_def().unwrap();
                    if fd.obank().get(op).unwrap().code() == OpCode::CPUI_INT_SEXT {
                        // Check for a signed extension; assume the extension is
                        // not consumed
                        let in0 = fd.obank().get(op).unwrap().get_in(0).unwrap();
                        let isz = fd.vbank().get(in0).unwrap().get_size();
                        self.switch_var_consume = kuna_base::address::calc_mask(isz);
                    }
                }
            }
        }
        Ok(())
    }

    /// Hide any guard code for \b this switch (C++ `foldInGuards`).
    pub fn fold_in_guards(&mut self, fd: &mut Funcdata) -> KunaResult<bool> {
        // jmodel->foldInGuards(fd,this)
        let mut model = self
            .jmodel
            .take()
            .ok_or_else(|| KunaError::lowlevel("foldInGuards: no model"))?;
        let res = model.fold_in_guards(fd, self);
        self.jmodel = Some(model);
        res
    }

    /// Check if the given PcodeOp still seems reachable in its function
    /// (C++ `JumpTable::isReachable`, `jumptable.cc:2509`).
    pub fn is_reachable(fd: &Funcdata, op: OpId) -> bool {
        let mut parent = match fd.obank().get(op).unwrap().get_parent() {
            None => return true,
            Some(p) => p,
        };
        for _ in 0..2 {
            // Only check two levels
            if fd.bblocks_ref().block(parent).size_in() != 1 {
                return true;
            }
            let bl = fd.bblocks_ref().block(parent).get_in(0);
            if fd.bblocks_ref().block(bl).size_out() != 2 {
                continue; // Check if -bl- looks like it contains a guard
            }
            let cbranch = match fd.bb_op_tail(bl) {
                None => continue,
                Some(c) => c,
            };
            if fd.obank().get(cbranch).unwrap().code() != OpCode::CPUI_CBRANCH {
                continue;
            }
            let vn = fd.obank().get(cbranch).unwrap().get_in(1).unwrap(); // Get the boolean variable
            if !fd.vbank().get(vn).unwrap().is_constant() {
                continue; // Has the guard collapsed
            }
            let mut trueslot =
                if fd.obank().get(cbranch).unwrap().is_boolean_flip() { 0 } else { 1 };
            if fd.vbank().get(vn).unwrap().get_offset() == 0 {
                trueslot = 1 - trueslot;
            }
            if fd.bblocks_ref().block(bl).get_out(trueslot) != parent {
                // If the remaining path does not lead to -op-
                return false; // op is not reachable
            }
            parent = bl;
        }
        true
    }

    /// Borrow the recovered address table.
    pub fn address_table(&self) -> &[Address] {
        &self.addresstable
    }

    /// Borrow the recovered load points.
    pub fn load_points(&self) -> &[LoadTable] {
        &self.loadpoints
    }

    /// Clear instance-specific data for \b this jump-table
    /// (C++ `JumpTable::clear`, `jumptable.cc:2890`).
    pub fn clear(&mut self) {
        self.clear_saved_model();
        let is_override = self.jmodel.as_ref().map(|m| m.is_override()).unwrap_or(false);
        if is_override {
            if let Some(m) = self.jmodel.as_mut() {
                m.clear();
            }
        } else {
            self.jmodel = None;
        }
        self.addresstable.clear();
        self.block2addr.clear();
        self.last_block = -1;
        self.label.clear();
        self.loadpoints.clear();
        self.indirect = None;
        self.switch_var_consume = !0u64;
        self.default_block = -1;
        self.recover_count = 0;
        self.partial_table = false;
        // -opaddress- -maxtablesize- -collectloads- are permanent
    }

    /// Save off current model and prepare for instantiating a new model
    /// (C++ `JumpTable::saveModel`, `jumptable.cc:2379`).
    fn save_model(&mut self) {
        // origmodel = jmodel; jmodel = 0  (delete prior origmodel)
        self.origmodel = self.jmodel.take();
    }

    /// Restore any saved model as the current model
    /// (C++ `JumpTable::restoreSavedModel`, `jumptable.cc:2388`).
    fn restore_saved_model(&mut self) {
        self.jmodel = self.origmodel.take();
    }

    /// Clear any saved model (C++ `JumpTable::clearSavedModel`, `jumptable.cc:2397`).
    fn clear_saved_model(&mut self) {
        self.origmodel = None;
    }

    /// Convert addresses in \b this table to actual targeted basic-blocks
    /// (C++ `JumpTable::switchOver`, `jumptable.cc:2678`).
    ///
    /// `target` resolves an [`Address`] to the [`OpId`] starting its block (the
    /// `FlowInfo::target` surface, provided by the caller).
    pub fn switch_over<F>(&mut self, fd: &Funcdata, target: F) -> KunaResult<()>
    where
        F: Fn(&Address) -> KunaResult<OpId>,
    {
        self.block2addr.clear();
        self.block2addr.reserve(self.addresstable.len());
        let parent = fd
            .obank()
            .get(self.indirect.unwrap())
            .unwrap()
            .get_parent()
            .ok_or_else(|| KunaError::lowlevel("switchOver: BRANCHIND has no parent"))?;

        for i in 0..self.addresstable.len() {
            let addr = self.addresstable[i].clone();
            let op = target(&addr)?;
            let tmpbl = fd
                .obank()
                .get(op)
                .unwrap()
                .get_parent()
                .ok_or_else(|| KunaError::lowlevel("switchOver: target op has no parent"))?;
            let nout = fd.bblocks_ref().block(parent).size_out();
            let mut pos = 0;
            while pos < nout {
                if fd.bblocks_ref().block(parent).get_out(pos) == tmpbl {
                    break;
                }
                pos += 1;
            }
            if pos == nout {
                return Err(KunaError::lowlevel("Jumptable destination not linked"));
            }
            self.block2addr.push(IndexPair::new(pos, i as int4));
        }
        self.last_block = self.block2addr.last().unwrap().block_position; // Out-edge of last address
        self.block2addr.sort();

        self.default_block = -1; // There is no default case initially
        let mut maxcount = 1; // If the maxcount is less than 2
        let mut idx = 0;
        while idx < self.block2addr.len() {
            let cur_pos = self.block2addr[idx].block_position;
            let mut count = 0;
            while idx < self.block2addr.len() && self.block2addr[idx].block_position == cur_pos {
                count += 1;
                idx += 1;
            }
            if count > maxcount {
                maxcount = count;
                self.default_block = cur_pos;
            }
        }
        Ok(())
    }

    /// Try to recover each model in turn until one matches the BRANCHIND
    /// (C++ `JumpTable::recoverModel`, `jumptable.cc:2408`).
    ///
    /// Walks `JumpBasic` then `JumpBasic2` (the `JumpAssisted`/CALLOTHER model is
    /// the `jumpassist` userop family — `// SEAM(W4)`, recorded as a loss).  Each
    /// model's `recoverModel` emulation-drives the index range over the landed
    /// [`EmulateFunction`].
    fn recover_model(&mut self, fd: &mut Funcdata) -> KunaResult<()> {
        let max_table_size = fd.get_arch().max_jumptable_size;
        let indirect = self.indirect.unwrap();
        if let Some(model) = self.jmodel.as_mut() {
            if model.is_override() {
                // Preexisting override model: re-run its recoverModel.
                let mut m = self.jmodel.take().unwrap();
                let r = m.recover_model(fd, indirect, 0, max_table_size);
                self.jmodel = Some(m);
                r?;
                return Ok(());
            }
            // Otherwise this is an old attempt we should remove.
            self.jmodel = None;
        }
        // SEAM(W4): the CALLOTHER `JumpAssisted` (jumpassist userop) model — not
        // yet ported (needs pcodeinjectlib/ExecutablePcode).  Recorded as a loss;
        // the basic models cover the corpus switches.

        // Try the basic model.
        let mut jbasic = JumpBasicModel::new();
        jbasic.is_partial = self.partial_table;
        let basic_ok = jbasic.recover_model(fd, indirect, self.addresstable.len() as uint4, max_table_size)?;
        // Stash the basic model's path-meld for model 2's piggyback.
        let basic_path = jbasic.get_path_meld().clone();
        if basic_ok {
            self.jmodel = Some(Box::new(jbasic));
            return Ok(());
        }
        // Try model 2 (default-path).
        let mut jbasic2 = JumpBasicModel::new_model2();
        jbasic2.is_partial = self.partial_table;
        jbasic2.initialize_start(&basic_path);
        let m2_ok = jbasic2.recover_model(fd, indirect, self.addresstable.len() as uint4, max_table_size)?;
        if m2_ok {
            self.jmodel = Some(Box::new(jbasic2));
            return Ok(());
        }
        // No model matched.
        self.jmodel = None;
        Ok(())
    }

    /// Recover the raw jump-table addresses (C++ `JumpTable::recoverAddresses`,
    /// `jumptable.cc:2773`).
    pub fn recover_addresses(&mut self, fd: &mut Funcdata) -> KunaResult<()> {
        self.recover_model(fd)?;
        if self.jmodel.is_none() {
            return Err(KunaError::lowlevel(format!(
                "Could not recover jumptable at {:?}. Too many branches",
                self.opaddress
            )));
        }
        if self.jmodel.as_ref().unwrap().get_table_size() == 0 {
            return Err(KunaError::lowlevel(format!(
                "Jumptable with 0 entries at {:?}",
                self.opaddress
            )));
        }
        let indirect = self.indirect.unwrap();
        if self.collectloads {
            let mut loadcounts: Vec<int4> = Vec::new();
            let mut addresstable = std::mem::take(&mut self.addresstable);
            let mut loadpoints = std::mem::take(&mut self.loadpoints);
            let model = self.jmodel.as_ref().unwrap();
            let r = model.build_addresses(
                fd,
                indirect,
                &mut addresstable,
                Some(&mut loadpoints),
                Some(&mut loadcounts),
            );
            self.addresstable = addresstable;
            self.loadpoints = loadpoints;
            r?;
            self.sanity_check(fd, Some(&loadcounts))?;
            LoadTable::collapse_table(&mut self.loadpoints);
        } else {
            let mut addresstable = std::mem::take(&mut self.addresstable);
            let model = self.jmodel.as_ref().unwrap();
            let r = model.build_addresses(fd, indirect, &mut addresstable, None, None);
            self.addresstable = addresstable;
            r?;
            self.sanity_check(fd, None)?;
        }
        Ok(())
    }

    /// Recover jump-table addresses keeping track of a possible previous stage
    /// (C++ `JumpTable::recoverMultistage`, `jumptable.cc:2803`).
    ///
    /// SEAM(W4/W5): the save/restore-on-failure control flow is faithful; the
    /// inner `recoverAddresses` needs the absent subsystems.
    pub fn recover_multistage(&mut self, fd: &mut Funcdata) -> KunaResult<()> {
        self.save_model();
        let oldaddresstable = std::mem::take(&mut self.addresstable);
        self.loadpoints.clear();
        match self.recover_addresses(fd) {
            Ok(()) => {}
            Err(_e) => {
                // C++ catches JumptableThunkError and LowlevelError identically:
                // restore the saved model and warn. (We do not yet distinguish
                // the thunk subclass; both downgrade to the same restore path.)
                self.restore_saved_model();
                self.addresstable = oldaddresstable;
                // fd->warning("Second-stage recovery error", indirect->getAddr())
                // SEAM(W4): warning sink lives on Funcdata; recorded as a loss.
            }
        }
        self.partial_table = false;
        self.clear_saved_model(); // Keep the new model if it was created successfully
        Ok(())
    }

    /// Set manual override information on \b this jump-table
    /// (C++ `JumpTable::setOverride`, `jumptable.cc:2616`).
    ///
    /// SEAM(W4): builds a `JumpBasicOverride`, which is a seam shell. Records the
    /// override intent on the table but cannot wire the model body yet.
    pub fn set_override(
        &mut self,
        _addrtable: &[Address],
        _naddr: &Address,
        _h: uintb,
        _sv: uintb,
    ) -> KunaResult<()> {
        Err(KunaError::lowlevel(
            "JumpTable::setOverride: JumpBasicOverride model needs DynamicHash + \
             EmulateFunction (W4/W5)",
        ))
    }

    /// Try to match the JumpTable model to the existing function
    /// (C++ `JumpTable::matchModel`, `jumptable.cc:2833`).
    ///
    /// SEAM(W4): the multistage-restart accounting on a table-size mismatch
    /// (`Override::insertMultistageJump` + `setRestartPending`) is the W4 override
    /// table; here a mismatch is recorded as a loss and the table keeps its (flow-
    /// recovered) addresses.  The model recovery itself is real.
    pub fn match_model(&mut self, fd: &mut Funcdata) -> KunaResult<()> {
        if !self.is_recovered() {
            return Err(KunaError::lowlevel(
                "Trying to recover jumptable labels without addresses",
            ));
        }
        // Unless the model is an override, move model so we can create a current
        // instance.
        if self.jmodel.is_some() {
            if !self.jmodel.as_ref().unwrap().is_override() {
                self.save_model();
            } else {
                self.clear_saved_model();
                // fd->warning("Switch is manually overridden", opaddress) SEAM(W4)
            }
        }
        self.recover_model(fd)?; // Create a current instance of the model
        if let Some(model) = self.jmodel.as_ref() {
            if model.get_table_size() != self.addresstable.len() as int4 {
                // SEAM(W4): the multistage-restart path
                // (Override::insertMultistageJump / setRestartPending) is the W4
                // override table; a (1 -> >1) mismatch would request a restart.
                // Recorded as a loss: the flow-recovered address table is kept.
            }
        }
        Ok(())
    }

    /// Recover the case labels for \b this jump-table
    /// (C++ `JumpTable::recoverLabels`, `jumptable.cc:2865`).
    ///
    /// SEAM(W4/W5/W6): findUnnormalized/buildLabels on JumpBasic need
    /// `backup2Switch` (TypeOp reverse eval, W6) and the model state; the trivial
    /// fallback needs `recoverModel`/`buildAddresses` (W4). The two-phase
    /// structure is faithful.
    pub fn recover_labels(&mut self, fd: &mut Funcdata) -> KunaResult<()> {
        if let Some(mut model) = self.jmodel.take() {
            let orig_size = self.origmodel.as_ref().map(|m| m.get_table_size()).unwrap_or(0);
            model.find_unnormalized(fd, self.maxaddsub, self.maxleftright, self.maxext)?;
            let res = if self.origmodel.is_none() || orig_size == 0 {
                // orig is the model itself (C++ buildLabels(...,jmodel)).
                let mut addresstable = std::mem::take(&mut self.addresstable);
                let mut label = std::mem::take(&mut self.label);
                let r = model.build_labels(fd, &mut addresstable, &mut label, &*model);
                self.addresstable = addresstable;
                self.label = label;
                r
            } else {
                let orig = self.origmodel.take().unwrap();
                let mut addresstable = std::mem::take(&mut self.addresstable);
                let mut label = std::mem::take(&mut self.label);
                let r = model.build_labels(fd, &mut addresstable, &mut label, &*orig);
                self.addresstable = addresstable;
                self.label = label;
                self.origmodel = Some(orig);
                r
            };
            self.jmodel = Some(model);
            res?;
        } else {
            // Trivial fallback (jumptable.cc:2878): the model could not be
            // recovered, but the addresses came from flow.
            let mut tm = JumpModelTrivial::new();
            let indirect = self.indirect.unwrap();
            let max = fd.get_arch().max_jumptable_size;
            tm.recover_model(fd, indirect, self.addresstable.len() as uint4, max)?;
            let mut addresstable = std::mem::take(&mut self.addresstable);
            tm.build_addresses(fd, indirect, &mut addresstable, None, None)?;
            self.addresstable = addresstable;
            self.trivial_switch_over(fd)?;
            let mut addresstable = std::mem::take(&mut self.addresstable);
            let mut label = std::mem::take(&mut self.label);
            // origmodel is None in the trivial path.
            let r = tm.build_labels(fd, &mut addresstable, &mut label, &tm);
            self.addresstable = addresstable;
            self.label = label;
            self.jmodel = Some(Box::new(tm));
            r?;
        }
        self.clear_saved_model();
        Ok(())
    }

    /// Check if this jump-table requires an additional recovery stage
    /// (C++ `JumpTable::checkForMultistage`, `jumptable.cc:3003`).
    ///
    /// SEAM(W4): the override-table query (`fd->getOverride().queryMultistageJumptable`)
    /// is W4; the structural guards are faithful and short-circuit before the
    /// query whenever possible.
    pub fn check_for_multistage(&mut self, _fd: &Funcdata) -> KunaResult<bool> {
        if self.addresstable.len() != 1 {
            return Ok(false);
        }
        if self.partial_table {
            return Ok(false);
        }
        if self.indirect.is_none() {
            return Ok(false);
        }
        if self.recover_count > 1 {
            return Ok(false);
        }
        // fd->getOverride().queryMultistageJumptable(indirect->getAddr()) SEAM(W4)
        Err(KunaError::lowlevel(
            "JumpTable::checkForMultistage: Override::queryMultistageJumptable is W4",
        ))
    }

    /// Perform a sanity check on recovered address targets
    /// (C++ `JumpTable::sanityCheck`, `jumptable.cc:2450`).
    ///
    /// The reachability/thunk control logic is ported faithfully; the inner
    /// `jmodel->sanityCheck` may need the loader (W4) for some models.
    pub fn sanity_check(
        &mut self,
        fd: &mut Funcdata,
        loadcounts: Option<&Vec<int4>>,
    ) -> KunaResult<()> {
        let indirect = self.indirect.unwrap();
        if self.jmodel.as_ref().unwrap().is_override() {
            return Ok(()); // Don't perform sanity check on an override
        }
        let sz = self.addresstable.len();

        if !JumpTable::is_reachable(fd, indirect) {
            self.partial_table = true; // If not reachable, mark as incomplete
        }
        if self.addresstable.len() == 1 {
            // One entry is likely some kind of thunk
            let mut isthunk = false;
            let addr = self.addresstable[0].clone();
            if addr.get_offset() == 0 {
                isthunk = true;
            } else {
                let addr2 = fd.obank().get(indirect).unwrap().get_addr().clone();
                let diff = if addr.get_offset() < addr2.get_offset() {
                    addr2.get_offset().wsub(addr.get_offset())
                } else {
                    addr.get_offset().wsub(addr2.get_offset())
                };
                if diff > 0xffff {
                    isthunk = true;
                }
            }
            if isthunk {
                return Err(KunaError::lowlevel("Likely thunk"));
            }
        }
        let mut model = self.jmodel.take().unwrap();
        let mut addresstable = std::mem::take(&mut self.addresstable);
        let mut loadpoints = std::mem::take(&mut self.loadpoints);
        let pass =
            model.sanity_check(fd, indirect, &mut addresstable, &mut loadpoints, loadcounts);
        self.addresstable = addresstable;
        self.loadpoints = loadpoints;
        self.jmodel = Some(model);
        if !pass? {
            return Err(KunaError::lowlevel("Jumptable did not pass sanity check"));
        }
        if sz != self.addresstable.len() {
            // If address table was resized
            // fd->warning("Sanity check requires truncation of jumptable", opaddress) SEAM(W4)
        }
        Ok(())
    }

    /// Encode \b this jump-table as a \<jumptable> element
    /// (C++ `JumpTable::encode`, `jumptable.cc:2916`).
    pub fn encode(&self, encoder: &mut dyn Encoder) -> KunaResult<()> {
        if !self.is_recovered() {
            return Err(KunaError::lowlevel("Trying to save unrecovered jumptable"));
        }

        encoder.open_element(&ELEM_JUMPTABLE);
        if self.display_format != 0 {
            encoder.write_unsigned_integer(&ATTRIB_FORMAT, self.display_format as u64);
        }
        self.opaddress.encode(encoder)?;
        for i in 0..self.addresstable.len() {
            encoder.open_element(&ELEM_DEST);
            let spc = self.addresstable[i].get_space();
            let off = self.addresstable[i].get_offset();
            // C++: if (spc != (AddrSpace *)0) spc->encodeAttributes(...)
            if let Some(spc) = spc {
                spc.encode_attributes(encoder, off)?;
            }
            if i < self.label.len() && self.label[i] != NO_LABEL {
                encoder.write_unsigned_integer(&ATTRIB_LABEL, self.label[i]);
            }
            encoder.close_element(&ELEM_DEST);
        }
        if !self.loadpoints.is_empty() {
            for lp in self.loadpoints.iter() {
                lp.encode(encoder)?;
            }
        }
        if let Some(m) = &self.jmodel {
            if m.is_override() {
                m.encode(encoder)?;
            }
        }
        encoder.close_element(&ELEM_JUMPTABLE);
        Ok(())
    }

    /// Decode \b this jump-table from a \<jumptable> element
    /// (C++ `JumpTable::decode`, `jumptable.cc:2950`).
    ///
    /// SEAM(W4): a `<basicoverride>` child instantiates `JumpBasicOverride`,
    /// which is a seam shell; that branch returns the precise `Err`. The address/
    /// label/loadtable parsing is fully ported.
    pub fn decode(&mut self, decoder: &mut dyn Decoder) -> KunaResult<()> {
        let elem_id = decoder.open_element_id(&ELEM_JUMPTABLE)?;
        if decoder.get_next_attribute_id()? == ATTRIB_FORMAT.get_id() {
            self.display_format = decoder.read_unsigned_integer()? as uint4;
        }
        self.opaddress = Address::decode(decoder)?;
        let mut missedlabel = false;
        loop {
            let sub_id = decoder.peek_element()?;
            if sub_id == 0 {
                break;
            }
            if sub_id == ELEM_DEST.get_id() {
                // C++: openElement(); scan attributes for ATTRIB_LABEL; then
                // Address::decode reads the (inline) space/offset attributes of
                // the same <dest>.  The C++ Address::decode goes through
                // VarnodeData::decodeFromAttributes; in the Rust element-based
                // address model we read the inline attributes directly and close
                // <dest> ourselves (after rewinding the attribute cursor that the
                // label scan advanced).
                let dest_id = decoder.open_element()?;
                let mut foundlabel = false;
                loop {
                    let attrib_id = decoder.get_next_attribute_id()?;
                    if attrib_id == 0 {
                        break;
                    }
                    if attrib_id == ATTRIB_LABEL.get_id() {
                        if missedlabel {
                            return Err(KunaError::lowlevel(
                                "Jumptable entries are missing labels",
                            ));
                        }
                        let lab = decoder.read_unsigned_integer()?;
                        self.label.push(lab);
                        foundlabel = true;
                        break;
                    }
                }
                if !foundlabel {
                    // No label attribute: no following entries may have a label
                    missedlabel = true;
                }
                decoder.rewind_attributes();
                let mut vdata = VarnodeData::default();
                vdata.decode_from_attributes(decoder)?;
                self.addresstable.push(vdata.get_addr());
                decoder.close_element(dest_id)?;
            } else if sub_id == ELEM_LOADTABLE.get_id() {
                let mut lt = LoadTable::single(Address::default(), 0);
                lt.decode(decoder)?;
                self.loadpoints.push(lt);
            } else if sub_id == ELEM_BASICOVERRIDE.get_id() {
                if self.jmodel.is_some() {
                    return Err(KunaError::lowlevel("Duplicate jumptable override specs"));
                }
                // jmodel = new JumpBasicOverride(this); jmodel->decode(decoder)
                return Err(KunaError::lowlevel(
                    "JumpTable::decode: <basicoverride> needs JumpBasicOverride (W4/W5)",
                ));
            } else {
                // Unknown child: the C++ has no else (the for-loop would spin),
                // but openElement consumed nothing; to avoid an infinite loop we
                // skip the element faithfully.
                let id = decoder.open_element()?;
                decoder.close_element_skipping(id)?;
            }
        }
        decoder.close_element(elem_id)?;

        if !self.label.is_empty() {
            while self.label.len() < self.addresstable.len() {
                self.label.push(NO_LABEL);
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests;
