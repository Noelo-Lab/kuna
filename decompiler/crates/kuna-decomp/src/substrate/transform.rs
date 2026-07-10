//! Port of `decompiler/cpp/transform.{cc,hh}` (W5, item `w5-s3-transform`) —
//! the **lazy placeholder graph** that [`crate::subflow`]'s `LaneDivide` and
//! `SplitFlow` build up before committing a large-scale data-flow transform to a
//! [`Funcdata`].
//!
//! # What this is
//!
//! A transform (splitting a wide Varnode into logical lanes, or splitting a
//! double-precision value into halves) is expressed as a graph of
//! [`TransformVar`] (placeholder Varnodes) and [`TransformOp`] (placeholder
//! PcodeOps) before any real IR is touched.  [`TransformManager`] owns that graph
//! and, on [`TransformManager::apply`], materializes the placeholders into real
//! [`crate::varnode::Varnode`]s and [`crate::op::PcodeOp`]s **in a fixed order**.
//!
//! # Faithfulness: allocation order is load-bearing
//!
//! The C++ `apply()` calls `createOps()` then `createVarnodes()`; each iterates
//! its placeholder storage (`newOps` is a `list`, `pieceMap` is a
//! `map<int4,...>` keyed by the big-Varnode create-index, `newVarnodes` is a
//! `list`) and calls `fd->newOp`/`fd->newConstant`/`fd->newUnique`/… in exactly
//! that iteration order.  Each `newOp`/`newVarnode*` bumps the function's
//! create-index counter, so the **order placeholders are visited determines the
//! create-indices of the resulting real IR**, which is observable downstream
//! (varnode ordering, `print C` text).  This port preserves that order exactly:
//!
//!   - `newOps` / `newVarnodes` are `Vec`s appended to in factory-call order (the
//!     C++ `list::emplace_back`);
//!   - `pieceMap` is a [`BTreeMap`] keyed by the big-Varnode create-index (the
//!     C++ `map<int4,TransformVar*>`), so `createVarnodes` walks pieces in
//!     ascending create-index order;
//!   - within a `pieceMap` entry the pieces are a `Vec` walked least-significant
//!     first up to the [`tvar_flags::split_terminator`] element, exactly as the
//!     C++ walks `vArray + i` until the terminator flag.
//!
//! # Pointer model
//!
//! The C++ uses raw `TransformVar *` / `TransformOp *` pointers with `new[]`
//! arrays and intrusive cross-links (`def`, `output`, `input[]`, `follow`).  Rust
//! cannot hold those aliased pointers, so each placeholder is addressed by a
//! stable [`TVarRef`] / [`TOpRef`] index into the manager's arenas.  The arenas
//! never remove or reorder elements during graph construction, so an index is as
//! stable as the C++ pointer it replaces.
//!
//! # Materialization (W10)
//!
//! [`TransformManager::apply`] materializes through the `Funcdata` factory
//! surface, all of which is now present: `newVarnodeOut`/`newUniqueOut` (the
//! `xref` split-borrow), `setInputVarnode`, `transferVarnodeProperties`,
//! `markIndirectCreation`, and `opSetOpcode(OpCode)` (resolved through the W6
//! `TypeOp` table via [`Funcdata::op_set_opcode_code`]).  The `constant_iop`
//! placeholder round-trips through [`crate::funcdata_varnode::op_iop_decode`].
//! The placeholder-graph construction and the visit-order logic — the parts that
//! determine downstream allocation order — drive the live mutators in exactly the
//! C++ order, so the materialized IR's create-indices match the oracle.

use std::collections::BTreeMap;

use kuna_base::address::{calc_mask, Address};
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::space::spacetype;
use kuna_base::types::{int4, uint4, uintb, Wrap};
use kuna_num::opcodes::OpCode;

use crate::funcdata::Funcdata;
use crate::context::{OpId, VarnodeId};

// =============================================================================
// TransformVar / TransformOp flag + type enums (transform.hh:31-91)
// =============================================================================

/// Types of replacement Varnodes (C++ anonymous `enum` in `class TransformVar`).
pub mod tvar_type {
    #![allow(non_upper_case_globals)]
    use kuna_base::types::uint4;
    /// New Varnode is a piece of an original Varnode.
    pub const piece: uint4 = 1;
    /// Varnode preexisted in the original data-flow.
    pub const preexisting: uint4 = 2;
    /// A new temporary (unique space) Varnode.
    pub const normal_temp: uint4 = 3;
    /// A temporary representing a piece of an original Varnode.
    pub const piece_temp: uint4 = 4;
    /// A new constant Varnode.
    pub const constant: uint4 = 5;
    /// Special iop constant encoding a PcodeOp reference.
    pub const constant_iop: uint4 = 6;
}

/// Boolean properties of a [`TransformVar`] placeholder (C++ anonymous `enum`).
pub mod tvar_flags {
    #![allow(non_upper_case_globals)]
    use kuna_base::types::uint4;
    /// The last (most significant piece) of a split array.
    pub const split_terminator: uint4 = 1;
    /// This is a piece of an input that has already been visited.
    pub const input_duplicate: uint4 = 2;
}

/// Special annotations on new pcode ops (C++ anonymous `enum` in
/// `class TransformOp`).
pub mod top_special {
    #![allow(non_upper_case_globals)]
    use kuna_base::types::uint4;
    /// Op replaces an existing op.
    pub const op_replacement: uint4 = 1;
    /// Op already exists (but will be transformed).
    pub const op_preexisting: uint4 = 2;
    /// Mark op as indirect creation.
    pub const indirect_creation: uint4 = 4;
    /// Mark op as indirect creation and possible call output.
    pub const indirect_creation_possible_out: uint4 = 8;
}

// =============================================================================
// Placeholder references (the pointer replacements)
// =============================================================================

/// Stable handle to a [`TransformVar`] in a [`TransformManager`]'s arenas
/// (replaces the C++ `TransformVar *`).
///
/// A var either lives in a `pieceMap` array (`Piece { key, idx }`, keyed by the
/// big-Varnode create-index and the lane index within that array) or in the
/// `newVarnodes` list (`New(idx)`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TVarRef {
    /// Lane `idx` of the piece array for the big Varnode with create-index `key`.
    Piece {
        /// Big-Varnode create-index this piece array is keyed by.
        key: int4,
        /// Lane index within the piece array (0 = least significant).
        idx: usize,
    },
    /// Element `idx` of the `newVarnodes` list.
    New(usize),
}

/// Stable handle to a [`TransformOp`] in a [`TransformManager`]'s `newOps` list
/// (replaces the C++ `TransformOp *`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TOpRef(pub usize);

// =============================================================================
// TransformVar (transform.hh:31-63)
// =============================================================================

/// Placeholder node for a Varnode that will exist after a transform is applied
/// (C++ `TransformVar`).
#[derive(Debug, Clone)]
pub struct TransformVar {
    /// Original \b big Varnode of which \b this is a component (`TransformVar::vn`).
    vn: Option<VarnodeId>,
    /// The new explicit lane Varnode (`TransformVar::replacement`).
    replacement: Option<VarnodeId>,
    /// Type of new Varnode (`TransformVar::type`).
    ty: uint4,
    /// Boolean properties of the placeholder (`TransformVar::flags`).
    flags: uint4,
    /// Size of the lane Varnode in bytes (`TransformVar::byteSize`).
    byte_size: int4,
    /// Size of the logical value in bits (`TransformVar::bitSize`).
    bit_size: int4,
    /// Value of constant or (bit) position within the original big Varnode
    /// (`TransformVar::val`).
    val: uintb,
    /// Defining op for new Varnode (`TransformVar::def`).
    def: Option<TOpRef>,
}

impl TransformVar {
    /// Initialize \b this variable from raw data (C++ `TransformVar::initialize`,
    /// transform.hh:203).
    fn initialize(&mut self, tp: uint4, v: Option<VarnodeId>, bits: int4, bytes: int4, value: uintb) {
        self.ty = tp;
        self.vn = v;
        self.val = value;
        self.bit_size = bits;
        self.byte_size = bytes;
        self.flags = 0;
        self.def = None;
        self.replacement = None;
    }

    /// A default-constructed placeholder (the C++ `TransformVar` after `new[]`,
    /// before `initialize`).
    fn blank() -> TransformVar {
        TransformVar {
            vn: None,
            replacement: None,
            ty: 0,
            flags: 0,
            byte_size: 0,
            bit_size: 0,
            val: 0,
            def: None,
        }
    }

    /// Get the original Varnode \b this placeholder models (C++ `getOriginal`).
    pub fn get_original(&self) -> Option<VarnodeId> {
        self.vn
    }

    /// Get the operator that defines this placeholder variable (C++ `getDef`).
    pub fn get_def(&self) -> Option<TOpRef> {
        self.def
    }

    /// Get the real Varnode this placeholder was materialized into, if any.
    pub fn get_replacement(&self) -> Option<VarnodeId> {
        self.replacement
    }

    /// Type of new Varnode (`TransformVar::type`).
    pub fn get_type(&self) -> uint4 {
        self.ty
    }

    /// Boolean properties of the placeholder (`TransformVar::flags`).
    pub fn get_flags(&self) -> uint4 {
        self.flags
    }

    /// Size of the lane Varnode in bytes (`TransformVar::byteSize`).
    pub fn get_byte_size(&self) -> int4 {
        self.byte_size
    }

    /// Size of the logical value in bits (`TransformVar::bitSize`).
    pub fn get_bit_size(&self) -> int4 {
        self.bit_size
    }

    /// Value of constant or (bit) position within the original big Varnode
    /// (`TransformVar::val`).
    pub fn get_val(&self) -> uintb {
        self.val
    }
}

// =============================================================================
// TransformOp (transform.hh:65-91)
// =============================================================================

/// Placeholder node for a PcodeOp that will exist after a transform is applied
/// (C++ `TransformOp`).
#[derive(Debug, Clone)]
pub struct TransformOp {
    /// Original op which \b this is splitting (or null) (`TransformOp::op`).
    op: Option<OpId>,
    /// The new replacement op (`TransformOp::replacement`).
    replacement: Option<OpId>,
    /// Opcode of the new op (`TransformOp::opc`).
    opc: OpCode,
    /// Special handling code when creating (`TransformOp::special`).
    special: uint4,
    /// Varnode output (`TransformOp::output`).
    output: Option<TVarRef>,
    /// Varnode inputs (`TransformOp::input`).
    input: Vec<Option<TVarRef>>,
    /// The following op after \b this, if not null (`TransformOp::follow`).
    follow: Option<TOpRef>,
}

impl TransformOp {
    /// Get the output placeholder variable for \b this operator (C++ `getOut`).
    pub fn get_out(&self) -> Option<TVarRef> {
        self.output
    }

    /// Get the i-th input placeholder variable for \b this (C++ `getIn`).
    pub fn get_in(&self, i: int4) -> Option<TVarRef> {
        self.input[i as usize]
    }

    /// Opcode of the new op (`TransformOp::opc`).
    pub fn get_opc(&self) -> OpCode {
        self.opc
    }

    /// Special handling code (`TransformOp::special`).
    pub fn get_special(&self) -> uint4 {
        self.special
    }

    /// The real PcodeOp this placeholder was materialized into, if any.
    pub fn get_replacement(&self) -> Option<OpId> {
        self.replacement
    }
}

// =============================================================================
// LanedRegister (transform.hh:93-125)
// =============================================================================

/// A (register) storage location and the ways it might be split into lanes
/// (C++ `LanedRegister`).
///
/// The derived `Default` (`whole_size = 0; size_bit_mask = 0`) reproduces the C++
/// default constructor `LanedRegister() { wholeSize = 0; sizeBitMask = 0; }`.
#[derive(Debug, Clone, Default)]
pub struct LanedRegister {
    /// Size of the whole register (`LanedRegister::wholeSize`).
    whole_size: int4,
    /// A 1-bit for every permissible lane size (`LanedRegister::sizeBitMask`).
    size_bit_mask: uint4,
}

impl LanedRegister {
    /// Construct for use with decode (C++ `LanedRegister()`).
    pub fn new() -> LanedRegister {
        LanedRegister::default()
    }

    /// Construct from a whole size and a lane-size bit mask
    /// (C++ `LanedRegister(int4,uint4)`).
    pub fn with_mask(sz: int4, mask: uint4) -> LanedRegister {
        LanedRegister { whole_size: sz, size_bit_mask: mask }
    }

    /// Parse a \e vector_lane_sizes attribute (C++ `LanedRegister::parseSizes`,
    /// transform.cc:300).
    ///
    /// `laneSizes` is a comma-separated list of sizes.
    pub fn parse_sizes(&mut self, register_size: int4, lane_sizes: &str) -> KunaResult<()> {
        self.whole_size = register_size;
        self.size_bit_mask = 0;
        // The C++ walks comma-separated tokens; the split iterator reproduces the
        // same tokenization (including a trailing empty token for a trailing comma,
        // which the C++ `istringstream` would read as a failed parse → sz stays -1
        // → throw on the bad-size check).
        for value in lane_sizes.split(',') {
            // The base flags are reset so a "0x.." token parses as hex; the values
            // used here are plain decimal lane byte-sizes.
            let sz: int4 = parse_int_autobase(value).unwrap_or(-1);
            if !(0..=16).contains(&sz) {
                return Err(KunaError::lowlevel(format!("Bad lane size: {value}")));
            }
            self.add_lane_size(sz);
        }
        Ok(())
    }

    /// Get the size in bytes of the whole laned register (C++ `getWholeSize`).
    pub fn get_whole_size(&self) -> int4 {
        self.whole_size
    }

    /// Get the bit mask of possible lane sizes (C++ `getSizeBitMask`).
    pub fn get_size_bit_mask(&self) -> uint4 {
        self.size_bit_mask
    }

    /// Add a new \e size to the allowed list (C++ `addLaneSize`).
    pub fn add_lane_size(&mut self, size: int4) {
        self.size_bit_mask |= 1u32.wshl(size as u32);
    }

    /// Is \e size among the allowed lane sizes (C++ `allowedLane`).
    pub fn allowed_lane(&self, size: int4) -> bool {
        (self.size_bit_mask.wshr(size as u32) & 1) != 0
    }

    /// Iterate over the permissible lane sizes, least to greatest (C++
    /// `LanedRegister::const_iterator` / `LanedIterator`).
    pub fn iter_sizes(&self) -> LanedIterator {
        LanedIterator::new(self)
    }
}

/// Iterator over possible lane sizes for a [`LanedRegister`] (C++
/// `LanedRegister::LanedIterator`).
#[derive(Debug, Clone)]
pub struct LanedIterator {
    /// Current lane size (`LanedIterator::size`); `-1` means ended.
    size: int4,
    /// Collection being iterated over (`LanedIterator::mask`).
    mask: uint4,
}

impl LanedIterator {
    fn new(laned_r: &LanedRegister) -> LanedIterator {
        let mut it = LanedIterator { size: 0, mask: laned_r.size_bit_mask };
        it.normalize();
        it
    }

    /// Normalize the iterator, after increment or initialization (C++
    /// `LanedIterator::normalize`, transform.cc:284).
    fn normalize(&mut self) {
        let mut flag: uint4 = 1;
        flag = flag.wshl(self.size as u32);
        while flag <= self.mask {
            if (flag & self.mask) != 0 {
                return; // Found a valid lane size
            }
            self.size += 1;
            flag = flag.wshl(1);
        }
        self.size = -1; // Indicate ending iterator
    }
}

impl Iterator for LanedIterator {
    type Item = int4;

    fn next(&mut self) -> Option<int4> {
        // *iter dereferences the *current* size; ++iter advances then normalizes.
        if self.size < 0 {
            return None;
        }
        let cur = self.size;
        self.size += 1;
        self.normalize();
        Some(cur)
    }
}

// =============================================================================
// LaneDescription (transform.hh:127-148)
// =============================================================================

/// Description of logical lanes within a \b big Varnode (C++ `LaneDescription`).
///
/// A \b lane is a byte offset and size within a Varnode. Lanes within a Varnode
/// are disjoint.
#[derive(Debug, Clone)]
pub struct LaneDescription {
    /// Size of the region being split in bytes (`LaneDescription::wholeSize`).
    whole_size: int4,
    /// Size of lanes in bytes (`LaneDescription::laneSize`).
    lane_size: Vec<int4>,
    /// Significance positions of lanes in bytes (`LaneDescription::lanePosition`).
    lane_position: Vec<int4>,
}

impl LaneDescription {
    /// Construct uniform lanes (C++ `LaneDescription(int4 origSize,int4 sz)`,
    /// transform.cc:35).
    pub fn uniform(orig_size: int4, sz: int4) -> LaneDescription {
        let whole_size = orig_size;
        let num_lanes = orig_size / sz;
        let mut lane_size = vec![0i32; num_lanes as usize];
        let mut lane_position = vec![0i32; num_lanes as usize];
        let mut pos = 0;
        for i in 0..num_lanes {
            lane_size[i as usize] = sz;
            lane_position[i as usize] = pos;
            pos += sz;
        }
        LaneDescription { whole_size, lane_size, lane_position }
    }

    /// Construct two lanes of arbitrary size (C++
    /// `LaneDescription(int4 origSize,int4 lo,int4 hi)`, transform.cc:53).
    pub fn two_lane(orig_size: int4, lo: int4, hi: int4) -> LaneDescription {
        LaneDescription {
            whole_size: orig_size,
            lane_size: vec![lo, hi],
            lane_position: vec![0, lo],
        }
    }

    /// Trim \b this to a subset of the original lanes (C++ `LaneDescription::subset`,
    /// transform.cc:72).
    ///
    /// Given a subrange (offset into the whole + size), throw out any lanes not in
    /// the subrange.  Returns `false` if the subrange partially intersects a lane.
    pub fn subset(&mut self, lsb_offset: int4, size: int4) -> bool {
        if lsb_offset == 0 && size == self.whole_size {
            return true; // subrange is the whole range
        }
        let first_lane = self.get_boundary(lsb_offset);
        if first_lane < 0 {
            return false;
        }
        let last_lane = self.get_boundary(lsb_offset + size);
        if last_lane < 0 {
            return false;
        }
        let mut new_lane_size: Vec<int4> = Vec::new();
        self.lane_position.clear();
        let mut new_position = 0;
        for i in first_lane..last_lane {
            let sz = self.lane_size[i as usize];
            self.lane_position.push(new_position);
            new_lane_size.push(sz);
            new_position += sz;
        }
        self.whole_size = size;
        self.lane_size = new_lane_size;
        true
    }

    /// Get the total number of lanes (C++ `getNumLanes`).
    pub fn get_num_lanes(&self) -> int4 {
        self.lane_size.len() as int4
    }

    /// Get the size of the region being split (C++ `getWholeSize`).
    pub fn get_whole_size(&self) -> int4 {
        self.whole_size
    }

    /// Get the size of the i-th lane (C++ `getSize`).
    pub fn get_size(&self, i: int4) -> int4 {
        self.lane_size[i as usize]
    }

    /// Get the significance offset of the i-th lane (C++ `getPosition`).
    pub fn get_position(&self, i: int4) -> int4 {
        self.lane_position[i as usize]
    }

    /// Get index of the lane that starts at the given byte position (C++
    /// `LaneDescription::getBoundary`, transform.cc:100).
    ///
    /// Position 0 maps to index 0; a position equal to whole size maps to the
    /// number of lanes.  Out-of-bounds / non-boundary positions return -1.
    pub fn get_boundary(&self, byte_pos: int4) -> int4 {
        if byte_pos < 0 || byte_pos > self.whole_size {
            return -1;
        }
        if byte_pos == self.whole_size {
            return self.lane_position.len() as int4;
        }
        let mut min: int4 = 0;
        let mut max: int4 = self.lane_position.len() as int4 - 1;
        while min <= max {
            let index = (min + max) / 2;
            let pos = self.lane_position[index as usize];
            if pos == byte_pos {
                return index;
            }
            if pos < byte_pos {
                min = index + 1;
            } else {
                max = index - 1;
            }
        }
        -1
    }

    /// Decide if a given truncation is natural for \b this description (C++
    /// `LaneDescription::restriction`, transform.cc:133).
    ///
    /// On success passes back `(resNumLanes, resSkipLanes)` and returns `Some`;
    /// `None` plays the role of the C++ `false` return (the out-params are then
    /// undefined).
    pub fn restriction(
        &self,
        _num_lanes: int4,
        skip_lanes: int4,
        byte_pos: int4,
        size: int4,
    ) -> Option<(int4, int4)> {
        let res_skip_lanes = self.get_boundary(self.lane_position[skip_lanes as usize] + byte_pos);
        if res_skip_lanes < 0 {
            return None;
        }
        let final_index =
            self.get_boundary(self.lane_position[skip_lanes as usize] + byte_pos + size);
        if final_index < 0 {
            return None;
        }
        let res_num_lanes = final_index - res_skip_lanes;
        if res_num_lanes != 0 {
            Some((res_num_lanes, res_skip_lanes))
        } else {
            None
        }
    }

    /// Decide if a given subset of lanes can be extended naturally for \b this
    /// description (C++ `LaneDescription::extension`, transform.cc:158).
    ///
    /// On success passes back `(resNumLanes, resSkipLanes)` and returns `Some`.
    pub fn extension(
        &self,
        _num_lanes: int4,
        skip_lanes: int4,
        byte_pos: int4,
        size: int4,
    ) -> Option<(int4, int4)> {
        let res_skip_lanes = self.get_boundary(self.lane_position[skip_lanes as usize] - byte_pos);
        if res_skip_lanes < 0 {
            return None;
        }
        let final_index =
            self.get_boundary(self.lane_position[skip_lanes as usize] - byte_pos + size);
        if final_index < 0 {
            return None;
        }
        let res_num_lanes = final_index - res_skip_lanes;
        if res_num_lanes != 0 {
            Some((res_num_lanes, res_skip_lanes))
        } else {
            None
        }
    }
}

// =============================================================================
// TransformManager (transform.hh:150-194)
// =============================================================================

/// Class for splitting larger registers holding smaller logical lanes (C++
/// `TransformManager`).
///
/// Given a starting Varnode, look for evidence of the Varnode being interpreted
/// as disjoint logical values concatenated together (lanes); if the
/// interpretation is consistent, split the Varnode and its data-flow into
/// explicit operations on the lanes.
///
/// The `fd` argument the C++ holds is supplied **per call** here (most methods
/// take `&mut Funcdata` / `&Funcdata`) rather than stored, so the manager does
/// not alias the function it transforms — exactly the same operations in exactly
/// the same order.
pub struct TransformManager {
    /// Map from large-Varnode create-index to their new pieces
    /// (`TransformManager::pieceMap`).
    piece_map: BTreeMap<int4, Vec<TransformVar>>,
    /// Storage for Varnode placeholder nodes (`TransformManager::newVarnodes`).
    new_varnodes: Vec<TransformVar>,
    /// Storage for PcodeOp placeholder nodes (`TransformManager::newOps`).
    new_ops: Vec<TransformOp>,
}

impl Default for TransformManager {
    fn default() -> Self {
        TransformManager::new()
    }
}

impl TransformManager {
    pub fn new() -> TransformManager {
        TransformManager {
            piece_map: BTreeMap::new(),
            new_varnodes: Vec::new(),
            new_ops: Vec::new(),
        }
    }

    // -- placeholder accessors (the index-based pointer dereference) ----------

    /// Borrow the placeholder var addressed by `r`.
    pub fn var(&self, r: TVarRef) -> &TransformVar {
        match r {
            TVarRef::Piece { key, idx } => &self.piece_map[&key][idx],
            TVarRef::New(idx) => &self.new_varnodes[idx],
        }
    }

    fn var_mut(&mut self, r: TVarRef) -> &mut TransformVar {
        match r {
            TVarRef::Piece { key, idx } => {
                &mut self.piece_map.get_mut(&key).expect("stale piece key")[idx]
            }
            TVarRef::New(idx) => &mut self.new_varnodes[idx],
        }
    }

    /// Borrow the placeholder op addressed by `r`.
    pub fn op(&self, r: TOpRef) -> &TransformOp {
        &self.new_ops[r.0]
    }

    fn op_mut(&mut self, r: TOpRef) -> &mut TransformOp {
        &mut self.new_ops[r.0]
    }

    // -- preserveAddress (transform.cc:348) -----------------------------------

    /// Should the address of the given Varnode be preserved when constructing a
    /// piece (C++ `TransformManager::preserveAddress`, transform.cc:348).
    ///
    /// Returns `true` if overlapping storage should be used, `false` if the new
    /// Varnode should be a unique temporary.
    pub fn preserve_address(
        &self,
        fd: &Funcdata,
        vn: VarnodeId,
        _bit_size: int4,
        lsb_offset: int4,
    ) -> bool {
        if (lsb_offset & 7) != 0 {
            return false; // Logical value not aligned
        }
        let v = fd.vbank().get(vn).expect("preserve_address: stale vn");
        if v.get_space().get_type() == spacetype::IPTR_INTERNAL {
            return false;
        }
        true
    }

    // -- clearVarnodeMarks (transform.cc:356) ---------------------------------

    /// Clear mark for all Varnodes in the map (C++
    /// `TransformManager::clearVarnodeMarks`, transform.cc:356).
    pub fn clear_varnode_marks(&self, fd: &mut Funcdata) {
        for vec in self.piece_map.values() {
            // C++ reads only the FIRST array element's vn (`(*iter).second->vn`).
            let vn = match vec.first().and_then(|tv| tv.vn) {
                Some(vn) => vn,
                None => continue,
            };
            fd.vbank_mut().get_mut(vn).expect("clear_varnode_marks: stale vn").clear_mark();
        }
    }

    // -- newPreexistingVarnode (transform.cc:370) -----------------------------

    /// Make placeholder for preexisting Varnode (C++
    /// `TransformManager::newPreexistingVarnode`, transform.cc:370).
    pub fn new_preexisting_varnode(&mut self, fd: &Funcdata, vn: VarnodeId) -> TVarRef {
        let v = fd.vbank().get(vn).expect("new_preexisting_varnode: stale vn");
        let create_index = v.get_create_index() as int4;
        let size = v.get_size();
        let mut res = TransformVar::blank();
        // value of 0 treats this as "piece" of itself at offset 0, allows getPiece() to find it
        res.initialize(tvar_type::preexisting, Some(vn), size * 8, size, 0);
        res.flags = tvar_flags::split_terminator;
        self.piece_map.insert(create_index, vec![res]);
        TVarRef::Piece { key: create_index, idx: 0 }
    }

    // -- newUnique (transform.cc:384) -----------------------------------------

    /// Make placeholder for new unique space Varnode (C++
    /// `TransformManager::newUnique`, transform.cc:384).
    pub fn new_unique(&mut self, size: int4) -> TVarRef {
        let mut res = TransformVar::blank();
        res.initialize(tvar_type::normal_temp, None, size * 8, size, 0);
        self.new_varnodes.push(res);
        TVarRef::New(self.new_varnodes.len() - 1)
    }

    // -- newConstant (transform.cc:399) ---------------------------------------

    /// Make placeholder for constant Varnode (C++
    /// `TransformManager::newConstant`, transform.cc:399).
    ///
    /// A piece of an existing constant can be created by giving the existing
    /// value and the least-significant offset.
    pub fn new_constant(&mut self, size: int4, lsb_offset: int4, val: uintb) -> TVarRef {
        let mut res = TransformVar::blank();
        let v = val.wshr(lsb_offset as u32) & calc_mask(size);
        res.initialize(tvar_type::constant, None, size * 8, size, v);
        self.new_varnodes.push(res);
        TVarRef::New(self.new_varnodes.len() - 1)
    }

    // -- newIop (transform.cc:411) --------------------------------------------

    /// Make placeholder for a special iop constant (C++
    /// `TransformManager::newIop`, transform.cc:411).
    ///
    /// Used for creating INDIRECT placeholders.
    pub fn new_iop(&mut self, fd: &Funcdata, vn: VarnodeId) -> TVarRef {
        let v = fd.vbank().get(vn).expect("new_iop: stale vn");
        let size = v.get_size();
        let offset = v.get_offset();
        let mut res = TransformVar::blank();
        res.initialize(tvar_type::constant_iop, None, size * 8, size, offset);
        self.new_varnodes.push(res);
        TVarRef::New(self.new_varnodes.len() - 1)
    }

    // -- newPiece (transform.cc:426) ------------------------------------------

    /// Make placeholder for a piece of a Varnode (C++
    /// `TransformManager::newPiece`, transform.cc:426).
    pub fn new_piece(
        &mut self,
        fd: &Funcdata,
        vn: VarnodeId,
        bit_size: int4,
        lsb_offset: int4,
    ) -> TVarRef {
        let create_index =
            fd.vbank().get(vn).expect("new_piece: stale vn").get_create_index() as int4;
        let byte_size = (bit_size + 7) / 8;
        let ty = if self.preserve_address(fd, vn, bit_size, lsb_offset) {
            tvar_type::piece
        } else {
            tvar_type::piece_temp
        };
        let mut res = TransformVar::blank();
        res.initialize(ty, Some(vn), bit_size, byte_size, lsb_offset as uintb);
        res.flags = tvar_flags::split_terminator;
        self.piece_map.insert(create_index, vec![res]);
        TVarRef::Piece { key: create_index, idx: 0 }
    }

    // -- newSplit (transform.cc:445) ------------------------------------------

    /// Create placeholder nodes splitting a Varnode into all its lanes (C++
    /// `TransformManager::newSplit(Varnode*,const LaneDescription&)`,
    /// transform.cc:445).
    ///
    /// Returns the [`TVarRef`] of lane 0; the lanes are `Piece { key, idx }` for
    /// `idx` 0..numLanes, least to most significant.
    pub fn new_split(
        &mut self,
        fd: &Funcdata,
        vn: VarnodeId,
        description: &LaneDescription,
    ) -> TVarRef {
        let v = fd.vbank().get(vn).expect("new_split: stale vn");
        let create_index = v.get_create_index() as int4;
        let is_const = v.is_constant();
        let offset = v.get_offset();
        let num = description.get_num_lanes();
        let mut res: Vec<TransformVar> = Vec::with_capacity(num as usize);
        for i in 0..num {
            let bitpos = description.get_position(i) * 8;
            let byte_size = description.get_size(i);
            let mut new_var = TransformVar::blank();
            if is_const {
                let val: uintb = if (bitpos as usize) < std::mem::size_of::<uintb>() * 8 {
                    // (vn->getOffset() >> bitpos) & calc_mask(byteSize)
                    offset.wshr(bitpos as u32) & calc_mask(byte_size)
                } else {
                    0 // Assume bits beyond precision are 0
                };
                new_var.initialize(tvar_type::constant, Some(vn), byte_size * 8, byte_size, val);
            } else {
                let ty = if self.preserve_address(fd, vn, byte_size * 8, bitpos) {
                    tvar_type::piece
                } else {
                    tvar_type::piece_temp
                };
                new_var.initialize(ty, Some(vn), byte_size * 8, byte_size, bitpos as uintb);
            }
            res.push(new_var);
        }
        res[(num - 1) as usize].flags = tvar_flags::split_terminator;
        self.piece_map.insert(create_index, res);
        TVarRef::Piece { key: create_index, idx: 0 }
    }

    // -- newSplit (subset) (transform.cc:481) ---------------------------------

    /// Create placeholder nodes splitting a Varnode into a subset of lanes (C++
    /// `TransformManager::newSplit(Varnode*,const LaneDescription&,int4,int4)`,
    /// transform.cc:481).
    pub fn new_split_subset(
        &mut self,
        fd: &Funcdata,
        vn: VarnodeId,
        description: &LaneDescription,
        num_lanes: int4,
        start_lane: int4,
    ) -> TVarRef {
        let v = fd.vbank().get(vn).expect("new_split_subset: stale vn");
        let create_index = v.get_create_index() as int4;
        let is_const = v.is_constant();
        let offset = v.get_offset();
        let base_bit_pos = description.get_position(start_lane) * 8;
        let mut res: Vec<TransformVar> = Vec::with_capacity(num_lanes as usize);
        for i in 0..num_lanes {
            let bitpos = description.get_position(start_lane + i) * 8 - base_bit_pos;
            let byte_size = description.get_size(start_lane + i);
            let mut new_var = TransformVar::blank();
            if is_const {
                let val: uintb = if (bitpos as usize) < std::mem::size_of::<uintb>() * 8 {
                    offset.wshr(bitpos as u32) & calc_mask(byte_size)
                } else {
                    0
                };
                new_var.initialize(tvar_type::constant, Some(vn), byte_size * 8, byte_size, val);
            } else {
                let ty = if self.preserve_address(fd, vn, byte_size * 8, bitpos) {
                    tvar_type::piece
                } else {
                    tvar_type::piece_temp
                };
                new_var.initialize(ty, Some(vn), byte_size * 8, byte_size, bitpos as uintb);
            }
            res.push(new_var);
        }
        res[(num_lanes - 1) as usize].flags = tvar_flags::split_terminator;
        self.piece_map.insert(create_index, res);
        TVarRef::Piece { key: create_index, idx: 0 }
    }

    // -- newOpReplace (transform.cc:515) --------------------------------------

    /// Create a new placeholder op intended to replace an existing op (C++
    /// `TransformManager::newOpReplace`, transform.cc:515).
    pub fn new_op_replace(&mut self, num_params: int4, opc: OpCode, replace: OpId) -> TOpRef {
        let rop = TransformOp {
            op: Some(replace),
            replacement: None,
            opc,
            special: top_special::op_replacement,
            output: None,
            follow: None,
            input: vec![None; num_params as usize],
        };
        self.new_ops.push(rop);
        TOpRef(self.new_ops.len() - 1)
    }

    // -- newOp (transform.cc:538) ---------------------------------------------

    /// Create a new placeholder op that will not replace an existing op (C++
    /// `TransformManager::newOp`, transform.cc:538).
    ///
    /// The op that follows it must be given.
    pub fn new_op(&mut self, num_params: int4, opc: OpCode, follow: TOpRef) -> TOpRef {
        let follow_op = self.new_ops[follow.0].op;
        let rop = TransformOp {
            op: follow_op,
            replacement: None,
            opc,
            special: 0,
            output: None,
            follow: Some(follow),
            input: vec![None; num_params as usize],
        };
        self.new_ops.push(rop);
        TOpRef(self.new_ops.len() - 1)
    }

    // -- newPreexistingOp (transform.cc:562) ----------------------------------

    /// Create a new placeholder op for an existing PcodeOp (C++
    /// `TransformManager::newPreexistingOp`, transform.cc:562).
    pub fn new_preexisting_op(&mut self, num_params: int4, opc: OpCode, original_op: OpId) -> TOpRef {
        let rop = TransformOp {
            op: Some(original_op),
            replacement: None,
            opc,
            special: top_special::op_preexisting,
            output: None,
            follow: None,
            input: vec![None; num_params as usize],
        };
        self.new_ops.push(rop);
        TOpRef(self.new_ops.len() - 1)
    }

    // -- getPreexistingVarnode (transform.cc:581) -----------------------------

    /// Get (or create) placeholder for a preexisting Varnode (C++
    /// `TransformManager::getPreexistingVarnode`, transform.cc:581).
    pub fn get_preexisting_varnode(&mut self, fd: &Funcdata, vn: VarnodeId) -> TVarRef {
        let v = fd.vbank().get(vn).expect("get_preexisting_varnode: stale vn");
        if v.is_constant() {
            let size = v.get_size();
            let offset = v.get_offset();
            return self.new_constant(size, 0, offset);
        }
        let create_index = v.get_create_index() as int4;
        if self.piece_map.contains_key(&create_index) {
            return TVarRef::Piece { key: create_index, idx: 0 };
        }
        self.new_preexisting_varnode(fd, vn)
    }

    // -- getPiece (transform.cc:599) ------------------------------------------

    /// Get (or create) placeholder piece (C++ `TransformManager::getPiece`,
    /// transform.cc:599).
    pub fn get_piece(
        &mut self,
        fd: &Funcdata,
        vn: VarnodeId,
        bit_size: int4,
        lsb_offset: int4,
    ) -> KunaResult<TVarRef> {
        let create_index =
            fd.vbank().get(vn).expect("get_piece: stale vn").get_create_index() as int4;
        if let Some(vec) = self.piece_map.get(&create_index) {
            let res = &vec[0];
            if res.bit_size != bit_size || res.val != lsb_offset as uintb {
                return Err(KunaError::lowlevel(
                    "Cannot create multiple pieces for one Varnode through getPiece",
                ));
            }
            return Ok(TVarRef::Piece { key: create_index, idx: 0 });
        }
        Ok(self.new_piece(fd, vn, bit_size, lsb_offset))
    }

    // -- getSplit (transform.cc:620) ------------------------------------------

    /// Get (or create) placeholder nodes splitting a Varnode into its lanes (C++
    /// `TransformManager::getSplit(Varnode*,const LaneDescription&)`,
    /// transform.cc:620).
    pub fn get_split(
        &mut self,
        fd: &Funcdata,
        vn: VarnodeId,
        description: &LaneDescription,
    ) -> TVarRef {
        let create_index =
            fd.vbank().get(vn).expect("get_split: stale vn").get_create_index() as int4;
        if self.piece_map.contains_key(&create_index) {
            return TVarRef::Piece { key: create_index, idx: 0 };
        }
        self.new_split(fd, vn, description)
    }

    // -- getSplit (subset) (transform.cc:640) ---------------------------------

    /// Get (or create) placeholder nodes splitting a Varnode into a subset of
    /// lanes (C++ `TransformManager::getSplit(Varnode*,const
    /// LaneDescription&,int4,int4)`, transform.cc:640).
    pub fn get_split_subset(
        &mut self,
        fd: &Funcdata,
        vn: VarnodeId,
        description: &LaneDescription,
        num_lanes: int4,
        start_lane: int4,
    ) -> TVarRef {
        let create_index =
            fd.vbank().get(vn).expect("get_split_subset: stale vn").get_create_index() as int4;
        if self.piece_map.contains_key(&create_index) {
            return TVarRef::Piece { key: create_index, idx: 0 };
        }
        self.new_split_subset(fd, vn, description, num_lanes, start_lane)
    }

    // -- opSetInput / opSetOutput (transform.hh:219-234) ----------------------

    /// Mark given variable as input to given op (C++
    /// `TransformManager::opSetInput`, transform.hh:219).
    pub fn op_set_input(&mut self, rop: TOpRef, rvn: TVarRef, slot: int4) {
        self.new_ops[rop.0].input[slot as usize] = Some(rvn);
    }

    /// Mark given variable as output of given op (C++
    /// `TransformManager::opSetOutput`, transform.hh:229).
    ///
    /// Marks both the op's `output` field and the var's `def` field.
    pub fn op_set_output(&mut self, rop: TOpRef, rvn: TVarRef) {
        self.new_ops[rop.0].output = Some(rvn);
        self.var_mut(rvn).def = Some(rop);
    }

    // -- preexistingGuard (transform.hh:246) ----------------------------------

    /// Should `newPreexistingOp` be called (C++
    /// `TransformManager::preexistingGuard`, transform.hh:246).
    pub fn preexisting_guard(&self, slot: int4, rvn: TVarRef) -> bool {
        if slot == 0 {
            return true; // If we came in on the first slot, build the TransformOp
        }
        let ty = self.var(rvn).ty;
        if ty == tvar_type::piece || ty == tvar_type::piece_temp {
            return false; // The op was/will be visited on slot 0, don't create now
        }
        true // The op was not (will not be) visited on slot 0, build now
    }

    // -- inheritIndirect (transform.cc:273) -----------------------------------

    /// Set \e indirect \e creation flags for `rop` based on a given INDIRECT (C++
    /// `TransformOp::inheritIndirect`, transform.cc:273).
    pub fn inherit_indirect(&mut self, rop: TOpRef, fd: &Funcdata, ind_op: OpId) {
        let indop = fd.obank().get(ind_op).expect("inherit_indirect: stale ind op");
        if indop.is_indirect_creation() {
            let in0 = indop.get_in(0).expect("inherit_indirect: INDIRECT missing in0");
            let in0_indirect_zero =
                fd.vbank().get(in0).expect("inherit_indirect: stale in0").is_indirect_zero();
            if in0_indirect_zero {
                self.op_mut(rop).special |= top_special::indirect_creation;
            } else {
                self.op_mut(rop).special |= top_special::indirect_creation_possible_out;
            }
        }
    }

    // =====================================================================
    // apply() and its private materialization sub-phases (transform.cc:654-765)
    // =====================================================================

    /// Apply the full transform to the function (C++ `TransformManager::apply`,
    /// transform.cc:756).
    ///
    /// W10: the materialization sub-phases are now live.  They resolve opcodes via
    /// [`Funcdata::op_set_opcode_code`] (`glb->inst[opc]` through the W6 `TypeOp`
    /// table), output Varnodes via `newVarnodeOut`/`newUniqueOut` (the
    /// `funcdata_varnode` xref split-borrow), inputs via `setInputVarnode`,
    /// `piece` properties via `transferVarnodeProperties`, the `constant_iop`
    /// placeholder via [`crate::funcdata_varnode::op_iop_decode`], and indirect
    /// creation via `markIndirectCreation`.
    pub fn apply(&mut self, fd: &mut Funcdata) -> KunaResult<()> {
        let mut input_list: Vec<TVarRef> = Vec::new();
        self.create_ops(fd)?;
        self.create_varnodes(fd, &mut input_list)?;
        self.remove_old(fd);
        self.transform_input_varnodes(fd, &input_list)?;
        self.place_inputs(fd)?;
        Ok(())
    }

    /// Handle some special PcodeOp marking (C++
    /// `TransformManager::specialHandling`, transform.cc:654).
    fn special_handling(&self, fd: &mut Funcdata, rop: TOpRef) -> KunaResult<()> {
        let special = self.new_ops[rop.0].special;
        let replacement =
            self.new_ops[rop.0].replacement.expect("special_handling: op not materialized");
        if (special & top_special::indirect_creation) != 0 {
            fd.mark_indirect_creation(replacement, false)?;
        } else if (special & top_special::indirect_creation_possible_out) != 0 {
            fd.mark_indirect_creation(replacement, true)?;
        }
        Ok(())
    }

    /// Create a new op for each placeholder, then insert them all into control
    /// flow (C++ `TransformManager::createOps`, transform.cc:665).
    fn create_ops(&mut self, fd: &mut Funcdata) -> KunaResult<()> {
        for i in 0..self.new_ops.len() {
            self.create_op_replacement(fd, TOpRef(i))?;
        }

        loop {
            let mut follow_count = 0;
            for i in 0..self.new_ops.len() {
                if !self.attempt_insertion(fd, TOpRef(i)) {
                    follow_count += 1;
                }
            }
            if follow_count == 0 {
                break;
            }
        }
        Ok(())
    }

    /// Create the new/modified op a placeholder represents (C++
    /// `TransformOp::createReplacement`, transform.cc:225).
    fn create_op_replacement(&mut self, fd: &mut Funcdata, rop: TOpRef) -> KunaResult<()> {
        let special = self.new_ops[rop.0].special;
        let opc = self.new_ops[rop.0].opc;
        if (special & top_special::op_preexisting) != 0 {
            let op = self.new_ops[rop.0].op.expect("create_op_replacement: preexisting op is null");
            self.new_ops[rop.0].replacement = Some(op);
            fd.op_set_opcode_code(op, opc);
            let input_size = self.new_ops[rop.0].input.len() as int4;
            loop {
                let num_input = fd.obank().get(op).expect("create_op_replacement: stale op").num_input();
                if input_size >= num_input {
                    break;
                }
                fd.op_remove_input(op, num_input - 1);
            }
            // Clear any remaining inputs.
            let num_input = fd.obank().get(op).expect("create_op_replacement: stale op").num_input();
            for i in 0..num_input {
                fd.op_unset_input(op, i);
            }
            // The inserted slot is null; opSetInput(op,null,slot) is the C++ no-op
            // form (the slot is filled in placeInputs), so insert the null slot
            // directly on the bank rather than through opSetInput's non-null surface.
            loop {
                let num_input = fd.obank().get(op).expect("create_op_replacement: stale op").num_input();
                if num_input >= input_size {
                    break;
                }
                fd.obank_mut()
                    .get_mut(op)
                    .expect("create_op_replacement: stale op")
                    .insert_input(num_input - 1);
            }
            Ok(())
        } else {
            let op = self.new_ops[rop.0].op.expect("create_op_replacement: op is null");
            let n = self.new_ops[rop.0].input.len() as int4;
            let addr: Address =
                fd.obank().get(op).expect("create_op_replacement: stale op").get_addr().clone();
            let replacement = fd.new_op(n, addr);
            self.new_ops[rop.0].replacement = Some(replacement);
            fd.op_set_opcode_code(replacement, opc);
            let output = self.new_ops[rop.0].output;
            if let Some(rout) = output {
                self.create_var_replacement(fd, rout)?;
            }
            let follow = self.new_ops[rop.0].follow;
            if follow.is_none() {
                // Can be inserted immediately
                if opc == OpCode::CPUI_MULTIEQUAL {
                    let parent = fd
                        .obank()
                        .get(op)
                        .expect("create_op_replacement: stale op")
                        .get_parent()
                        .expect("create_op_replacement: op has no parent");
                    fd.op_insert_begin(replacement, parent);
                } else {
                    fd.op_insert_before(replacement, op);
                }
            }
            Ok(())
        }
    }

    /// Try to put the new PcodeOp into its basic block (C++
    /// `TransformOp::attemptInsertion`, transform.cc:254).
    ///
    /// Returns `true` if the op is successfully inserted or already inserted.
    fn attempt_insertion(&mut self, fd: &mut Funcdata, rop: TOpRef) -> bool {
        let follow = self.new_ops[rop.0].follow;
        if let Some(follow) = follow {
            if self.new_ops[follow.0].follow.is_none() {
                let opc = self.new_ops[rop.0].opc;
                let replacement =
                    self.new_ops[rop.0].replacement.expect("attempt_insertion: no replacement");
                let follow_repl = self.new_ops[follow.0]
                    .replacement
                    .expect("attempt_insertion: follow not materialized");
                if opc == OpCode::CPUI_MULTIEQUAL {
                    let parent = fd
                        .obank()
                        .get(follow_repl)
                        .expect("attempt_insertion: stale follow replacement")
                        .get_parent()
                        .expect("attempt_insertion: follow replacement has no parent");
                    fd.op_insert_begin(replacement, parent);
                } else {
                    fd.op_insert_before(replacement, follow_repl);
                }
                // Mark that this has been inserted.
                self.new_ops[rop.0].follow = None;
                return true;
            }
            return false;
        }
        true // Already inserted
    }

    /// Create a Varnode for each placeholder, recording inputs (C++
    /// `TransformManager::createVarnodes`, transform.cc:684).
    fn create_varnodes(
        &mut self,
        fd: &mut Funcdata,
        input_list: &mut Vec<TVarRef>,
    ) -> KunaResult<()> {
        // BTreeMap iteration is ascending create-index order, matching map<int4,...>.
        let keys: Vec<int4> = self.piece_map.keys().copied().collect();
        for key in keys {
            let len = self.piece_map[&key].len();
            for idx in 0..len {
                let r = TVarRef::Piece { key, idx };
                let ty = self.var(r).ty;
                if ty == tvar_type::piece {
                    let vn = self.var(r).vn.expect("create_varnodes: piece has no vn");
                    let is_input = fd.vbank().get(vn).expect("create_varnodes: stale vn").is_input();
                    if is_input {
                        input_list.push(r);
                        let is_mark = fd.vbank().get(vn).expect("create_varnodes").is_mark();
                        if is_mark {
                            self.var_mut(r).flags |= tvar_flags::input_duplicate;
                        } else {
                            fd.vbank_mut().get_mut(vn).expect("create_varnodes").set_mark();
                        }
                    }
                }
                self.create_var_replacement(fd, r)?;
                if (self.var(r).flags & tvar_flags::split_terminator) != 0 {
                    break;
                }
            }
        }
        for idx in 0..self.new_varnodes.len() {
            self.create_var_replacement(fd, TVarRef::New(idx))?;
        }
        Ok(())
    }

    /// Create the new/modified variable a placeholder represents (C++
    /// `TransformVar::createReplacement`, transform.cc:175).
    fn create_var_replacement(&mut self, fd: &mut Funcdata, rvn: TVarRef) -> KunaResult<()> {
        if self.var(rvn).replacement.is_some() {
            return Ok(()); // Replacement already created
        }
        let ty = self.var(rvn).ty;
        match ty {
            tvar_type::preexisting => {
                let vn = self.var(rvn).vn.expect("create_var_replacement: preexisting has no vn");
                self.var_mut(rvn).replacement = Some(vn);
                Ok(())
            }
            tvar_type::constant => {
                let byte_size = self.var(rvn).byte_size;
                let val = self.var(rvn).val;
                let new_vn = fd.new_constant(byte_size, val);
                self.var_mut(rvn).replacement = Some(new_vn);
                Ok(())
            }
            tvar_type::normal_temp | tvar_type::piece_temp => {
                let byte_size = self.var(rvn).byte_size;
                let def = self.var(rvn).def;
                match def {
                    None => {
                        let new_vn = fd.new_unique(byte_size, None);
                        self.var_mut(rvn).replacement = Some(new_vn);
                    }
                    Some(def_rop) => {
                        let def_repl = self.new_ops[def_rop.0]
                            .replacement
                            .expect("create_var_replacement: temp def not materialized");
                        let new_vn = fd.new_unique_out(byte_size, def_repl)?;
                        self.var_mut(rvn).replacement = Some(new_vn);
                    }
                }
                Ok(())
            }
            tvar_type::piece => {
                let mut byte_pos = self.var(rvn).val as int4;
                if (byte_pos & 7) != 0 {
                    return Err(KunaError::lowlevel("Varnode piece is not byte aligned"));
                }
                byte_pos >>= 3;
                let vn = self.var(rvn).vn.expect("create_var_replacement: piece has no vn");
                let byte_size = self.var(rvn).byte_size;
                let v = fd.vbank().get(vn).expect("create_var_replacement: stale piece vn");
                if v.get_space().is_big_endian() {
                    byte_pos = v.get_size() - byte_pos - byte_size;
                }
                // `Add<i64>` is impl'd on `&Address`; build the offset address from the
                // borrowed `vn->getAddr()` directly (it returns an owned Address).
                let mut addr: Address = v.get_addr() + (byte_pos as i64);
                addr.renormalize(byte_size, fd.get_arch().manage())?;
                let def = self.var(rvn).def;
                let replacement = match def {
                    None => {
                        fd.new_varnode(byte_size, &addr, None)
                    }
                    Some(def_rop) => {
                        let def_repl = self.new_ops[def_rop.0]
                            .replacement
                            .expect("create_var_replacement: piece def not materialized");
                        fd.new_varnode_out(byte_size, &addr, def_repl)?
                    }
                };
                self.var_mut(rvn).replacement = Some(replacement);
                fd.transfer_varnode_properties(vn, replacement, byte_pos);
                Ok(())
            }
            tvar_type::constant_iop => {
                let val = self.var(rvn).val;
                let indeffect = crate::funcdata_varnode::op_iop_decode(val);
                let new_vn = fd.new_varnode_iop(indeffect);
                self.var_mut(rvn).replacement = Some(new_vn);
                Ok(())
            }
            _ => Err(KunaError::lowlevel("Bad TransformVar type")),
        }
    }

    /// Remove old preexisting PcodeOps that are now obsolete (C++
    /// `TransformManager::removeOld`, transform.cc:713).
    fn remove_old(&mut self, fd: &mut Funcdata) {
        for i in 0..self.new_ops.len() {
            let special = self.new_ops[i].special;
            if (special & top_special::op_replacement) != 0 {
                let op = self.new_ops[i].op.expect("remove_old: op_replacement op is null");
                if !fd.obank().get(op).expect("remove_old: stale op").is_dead() {
                    fd.op_destroy(op);
                }
            }
        }
    }

    /// Remove old input Varnodes, mark new input Varnodes (C++
    /// `TransformManager::transformInputVarnodes`, transform.cc:729).
    fn transform_input_varnodes(
        &mut self,
        fd: &mut Funcdata,
        input_list: &[TVarRef],
    ) -> KunaResult<()> {
        for &rvn in input_list {
            let flags = self.var(rvn).flags;
            if (flags & tvar_flags::input_duplicate) == 0 {
                let vn = self.var(rvn).vn.expect("transform_input_varnodes: no vn");
                fd.delete_varnode(vn)?;
            }
            let repl =
                self.var(rvn).replacement.expect("transform_input_varnodes: input not materialized");
            let new_repl = fd.set_input_varnode(repl)?;
            self.var_mut(rvn).replacement = Some(new_repl);
        }
        Ok(())
    }

    /// Set input Varnodes for all new ops (C++ `TransformManager::placeInputs`,
    /// transform.cc:740).
    fn place_inputs(&mut self, fd: &mut Funcdata) -> KunaResult<()> {
        for i in 0..self.new_ops.len() {
            let rop = TOpRef(i);
            let op = self.new_ops[i].replacement.expect("place_inputs: op not materialized");
            let n = self.new_ops[i].input.len();
            for slot in 0..n {
                let rvn = self.new_ops[i].input[slot].expect("place_inputs: input slot is null");
                let vn = self.var(rvn).replacement.expect("place_inputs: input var not materialized");
                fd.op_set_input(op, vn, slot as int4)?;
            }
            self.special_handling(fd, rop)?;
        }
        Ok(())
    }
}

/// Parse an integer the way the C++ `istringstream` with `unsetf(dec|hex|oct)`
/// does: honor a `0x`/`0X` prefix (hex), a leading `0` (octal), else decimal.
/// Returns `None` on any parse failure (matching the C++ leaving `sz == -1`).
fn parse_int_autobase(s: &str) -> Option<int4> {
    let t = s.trim();
    if let Some(hex) = t.strip_prefix("0x").or_else(|| t.strip_prefix("0X")) {
        i32::from_str_radix(hex, 16).ok()
    } else if t.len() > 1 && t.starts_with('0') {
        i32::from_str_radix(&t[1..], 8).ok()
    } else {
        t.parse::<i32>().ok()
    }
}

#[cfg(test)]
mod tests;
