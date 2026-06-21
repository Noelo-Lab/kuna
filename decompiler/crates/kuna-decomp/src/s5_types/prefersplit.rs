//! Port of `decompiler/cpp/prefersplit.{cc,hh}` — the `PreferSplitManager`.
//!
//! Some processors hold a logical value in a single physical register but the
//! decompiler renders it more clearly as two independent pieces (a "preferred
//! split").  Given a table of [`PreferSplitRecord`]s (storage + a byte offset
//! where the high piece begins), this pass walks the function and, wherever a
//! whole-register Varnode appears as the *single* product/consumer of a COPY /
//! PIECE / SUBPIECE / LOAD / STORE / INT_ZEXT, rewrites that op into two
//! independent ops acting on the two pieces.  A second sweep
//! ([`PreferSplitManager::split_additional`]) cleans up temporaries that the
//! first sweep produced but could not fully split.
//!
//! # Faithfulness
//!
//! The bodies are transcribed statement-for-statement from the C++ oracle: the
//! same big/little-endian piece layout in [`PreferSplitManager::fillin_instance`],
//! the same per-opcode `test*`/`split*` guard pairs, the same op-creation /
//! `opInsertAfter` ordering (the order matters — the C++ inserts `loop` then
//! `hiop` then `addop` so they end up `addop, hiop, loop` after `op`), and the
//! same total-order [`PreferSplitRecord`] comparator (space index, then *bigger*
//! size first, then offset) which is output-determining for
//! [`PreferSplitManager::find_record`]'s `lower_bound`.
//!
//! `int4`→`i32`, `uintb`→`u64`; constant splitting uses wrapping shifts/masks
//! (`calc_mask`).  The C++ holds raw `Varnode *` / `PcodeOp *`; the port holds
//! `VarnodeId` / `OpId` and reads the IR through `data.vbank()` / `data.obank()`,
//! snapshotting descendant lists before mutating (the C++ holds a
//! `list::const_iterator` it advances first).
//!
//! `getSpaceFromConst` (the constant LOAD/STORE space-id argument) is resolved
//! here directly through the function's [`AddrSpaceManager`]
//! (`getSpace()->getManager()->getSpace((int4)getOffset())`) — unlike the
//! `double.cc` seam, the value is a genuine constant space-id and the manager is
//! reachable from `Funcdata`.
//!
//! `findRecord` uses a `BTreeMap`-free [`slice::binary_search_by`] over the
//! sorted record vector to reproduce the C++ `lower_bound` + equality probe
//! (ADR: BTree-only for owned maps; this is a sorted-`Vec` binary search, the
//! exact analogue of the C++ `vector` + `lower_bound`).

use kuna_base::address::{calc_mask, Address};
use kuna_base::marshal::ElementId;
use kuna_base::space::{spacetype, AddrSpace};
use kuna_base::types::{int4, uintb};
use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;
use std::cmp::Ordering;
use std::rc::Rc;

use crate::funcdata::Funcdata;
use crate::seams::{OpId, VarnodeId};

/// Marshaling element `<prefersplit>` (C++ `ELEM_PREFERSPLIT = ElementId("prefersplit",225)`).
pub const ELEM_PREFERSPLIT: ElementId = ElementId::new("prefersplit", 225);

/// A record describing storage that should be rendered as two split pieces.
///
/// C++ `struct PreferSplitRecord` (`prefersplit.hh`).
#[derive(Clone, Debug)]
pub struct PreferSplitRecord {
    /// The full storage location of the value to split.
    pub storage: VarnodeData,
    /// Number of initial bytes (in address order) to split into the first piece.
    pub splitoffset: int4,
}

impl PreferSplitRecord {
    /// Total order over records (C++ `PreferSplitRecord::operator<`).
    ///
    /// Ordering: space index, then **bigger sizes first**, then offset.  This is
    /// the comparator `initialize`'s `sort` and `find_record`'s `lower_bound`
    /// both rely on, so it is output-determining and transcribed field-by-field.
    fn cmp_key(&self) -> (i32, std::cmp::Reverse<u32>, u64) {
        let space_index = self
            .storage
            .space
            .as_ref()
            .map(|s| s.get_index())
            .unwrap_or(-1);
        // size: bigger sizes come first -> Reverse on the size
        (space_index, std::cmp::Reverse(self.storage.size), self.storage.offset)
    }
}

impl PartialEq for PreferSplitRecord {
    fn eq(&self, other: &Self) -> bool {
        self.cmp_key() == other.cmp_key()
    }
}
impl Eq for PreferSplitRecord {}
impl PartialOrd for PreferSplitRecord {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for PreferSplitRecord {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cmp_key().cmp(&other.cmp_key())
    }
}

// =============================================================================
// Small IR read helpers (mirrors double.rs' idiom: read through the banks)
// =============================================================================

#[inline]
fn vn_is_constant(data: &Funcdata, vn: VarnodeId) -> bool {
    data.vbank().get(vn).expect("prefersplit: stale vn").is_constant()
}
#[inline]
fn vn_is_written(data: &Funcdata, vn: VarnodeId) -> bool {
    data.vbank().get(vn).expect("prefersplit: stale vn").is_written()
}
#[inline]
fn vn_is_free(data: &Funcdata, vn: VarnodeId) -> bool {
    data.vbank().get(vn).expect("prefersplit: stale vn").is_free()
}
#[inline]
fn vn_has_no_descend(data: &Funcdata, vn: VarnodeId) -> bool {
    data.vbank().get(vn).expect("prefersplit: stale vn").has_no_descend()
}
#[inline]
fn vn_get_def(data: &Funcdata, vn: VarnodeId) -> Option<OpId> {
    data.vbank().get(vn).expect("prefersplit: stale vn").get_def()
}
#[inline]
fn vn_get_size(data: &Funcdata, vn: VarnodeId) -> int4 {
    data.vbank().get(vn).expect("prefersplit: stale vn").get_size()
}
#[inline]
fn vn_get_offset(data: &Funcdata, vn: VarnodeId) -> uintb {
    data.vbank().get(vn).expect("prefersplit: stale vn").get_offset()
}
#[inline]
fn vn_get_addr(data: &Funcdata, vn: VarnodeId) -> Address {
    data.vbank().get(vn).expect("prefersplit: stale vn").get_addr().clone()
}
#[inline]
fn vn_get_space(data: &Funcdata, vn: VarnodeId) -> Rc<AddrSpace> {
    Rc::clone(data.vbank().get(vn).expect("prefersplit: stale vn").get_space())
}
#[inline]
fn vn_space_is_big_endian(data: &Funcdata, vn: VarnodeId) -> bool {
    data.vbank().get(vn).expect("prefersplit: stale vn").get_space().is_big_endian()
}
#[inline]
fn vn_space_type(data: &Funcdata, vn: VarnodeId) -> spacetype {
    data.vbank().get(vn).expect("prefersplit: stale vn").get_space().get_type()
}
/// Snapshot a Varnode's descendant op ids in `beginDescend` order (the C++ holds
/// a `list::const_iterator` and advances it before mutating).
#[inline]
fn descend_ops(data: &Funcdata, vn: VarnodeId) -> Vec<OpId> {
    data.vbank().get(vn).expect("prefersplit: stale vn").descend_iter().collect()
}
/// `Varnode::loneDescend` — the single reading op, or `None`.
#[inline]
fn lone_descend(data: &Funcdata, vn: VarnodeId) -> Option<OpId> {
    let v = data.vbank().get(vn).expect("prefersplit: stale vn");
    let mut it = v.descend_iter();
    let first = it.next()?;
    if it.next().is_some() {
        return None;
    }
    Some(first)
}

#[inline]
fn op_code(data: &Funcdata, op: OpId) -> OpCode {
    data.obank().get(op).expect("prefersplit: stale op").code()
}
#[inline]
fn op_get_in(data: &Funcdata, op: OpId, slot: int4) -> VarnodeId {
    data.obank()
        .get(op)
        .expect("prefersplit: stale op")
        .get_in(slot)
        .expect("prefersplit: missing input")
}
#[inline]
fn op_get_out(data: &Funcdata, op: OpId) -> Option<VarnodeId> {
    data.obank().get(op).expect("prefersplit: stale op").get_out()
}
#[inline]
fn op_get_addr(data: &Funcdata, op: OpId) -> Address {
    data.obank().get(op).expect("prefersplit: stale op").get_addr().clone()
}
#[inline]
fn op_is_dead(data: &Funcdata, op: OpId) -> bool {
    data.obank().get(op).expect("prefersplit: stale op").is_dead()
}

/// Resolve `vn->getSpaceFromConst()`: `getSpace()->getManager()->getSpace((int4)getOffset())`.
///
/// The constant Varnode encodes a space-id in its offset; index the function's
/// space manager.  Unlike the `double.cc` seam this is reachable here.
fn space_from_const(data: &Funcdata, vn: VarnodeId) -> Rc<AddrSpace> {
    let idx = vn_get_offset(data, vn) as int4;
    Rc::clone(
        data.get_arch()
            .manage()
            .get_space(idx)
            .expect("prefersplit: getSpaceFromConst out of range"),
    )
}

/// Resolve an [`OpCode`] to the [`TypeOp`] that [`Funcdata::op_set_opcode`]
/// expects.  Resolves through the canonical `type_op_info` table so the op
/// carries its real `opflags` (eval-type bit included) — a flag-less
/// `TypeOp::new(opc,0,..)` leaves `getEvalType()==0`, blocking constant folding.
fn set_opcode(data: &mut Funcdata, op: OpId, opc: OpCode) {
    data.op_set_opcode(op, crate::typeop::type_op_for(opc));
}

// =============================================================================
// SplitInstance: the per-Varnode working state of a single split
// =============================================================================

/// C++ `PreferSplitManager::SplitInstance` — the Varnode being split plus its
/// two pieces (`hi`/`lo`) as they get materialized.
struct SplitInstance {
    splitoffset: int4,
    vn: Option<VarnodeId>,
    /// Most significant piece.
    hi: Option<VarnodeId>,
    /// Least significant piece.
    lo: Option<VarnodeId>,
}

impl SplitInstance {
    fn new(v: Option<VarnodeId>, off: int4) -> Self {
        SplitInstance { vn: v, splitoffset: off, hi: None, lo: None }
    }
}

/// C++ `class PreferSplitManager`.
#[derive(Default)]
pub struct PreferSplitManager {
    records: Vec<PreferSplitRecord>,
    /// Copies of temporaries that need additional splitting.
    tempsplits: Vec<OpId>,
}

impl PreferSplitManager {
    /// Initialize the manager against a (already-sorted) record table
    /// (C++ `PreferSplitManager::init`).
    ///
    /// The C++ holds a `const vector<PreferSplitRecord> *`; the Rust port owns a
    /// clone so the manager is self-contained (the records are tiny VarnodeData
    /// rows).  `data` is passed to each driving method instead of being stashed.
    pub fn init(&mut self, rec: &[PreferSplitRecord]) {
        self.records = rec.to_vec();
        self.tempsplits.clear();
    }

    /// Define the Varnode pieces of `inst` (C++ `fillinInstance`).
    fn fillin_instance(
        data: &mut Funcdata,
        inst: &mut SplitInstance,
        bigendian: bool,
        sethi: bool,
        setlo: bool,
    ) {
        let vn = inst.vn.expect("fillinInstance: null vn");
        let vn_size = vn_get_size(data, vn);
        let losize = if bigendian { vn_size - inst.splitoffset } else { inst.splitoffset };
        let hisize = vn_size - losize;
        if vn_is_constant(data, vn) {
            let origval = vn_get_offset(data, vn);
            // Split the constant into two pieces
            let loval = origval & calc_mask(losize);
            let hival = (origval >> (8 * losize)) & calc_mask(hisize);
            if setlo && inst.lo.is_none() {
                inst.lo = Some(data.new_constant(losize, loval));
            }
            if sethi && inst.hi.is_none() {
                inst.hi = Some(data.new_constant(hisize, hival));
            }
        } else {
            let vn_addr = vn_get_addr(data, vn);
            if bigendian {
                if setlo && inst.lo.is_none() {
                    let a = &vn_addr + inst.splitoffset as i64;
                    inst.lo = Some(data.new_varnode(losize, &a, None));
                }
                if sethi && inst.hi.is_none() {
                    inst.hi = Some(data.new_varnode(hisize, &vn_addr, None));
                }
            } else {
                if setlo && inst.lo.is_none() {
                    inst.lo = Some(data.new_varnode(losize, &vn_addr, None));
                }
                if sethi && inst.hi.is_none() {
                    let a = &vn_addr + inst.splitoffset as i64;
                    inst.hi = Some(data.new_varnode(hisize, &a, None));
                }
            }
        }
    }

    /// Create COPY ops based on input `ininst` and output `outinst` to replace
    /// `op` (C++ `createCopyOps`).
    fn create_copy_ops(
        &mut self,
        data: &mut Funcdata,
        ininst: &SplitInstance,
        outinst: &SplitInstance,
        op: OpId,
        _istemp: bool,
    ) {
        let addr = op_get_addr(data, op);
        let hiop = data.new_op(1, addr.clone()); // Create two new COPYs
        let loop_ = data.new_op(1, addr);
        set_opcode(data, hiop, OpCode::CPUI_COPY);
        set_opcode(data, loop_, OpCode::CPUI_COPY);

        data.op_insert_after(loop_, op); // Insert new COPYs at same position as original operation
        data.op_insert_after(hiop, op);
        data.op_unset_input(op, 0); // Unset input so we can reassign free inputs to new ops

        data.op_set_output(hiop, outinst.hi.expect("createCopyOps: out hi")).ok(); // Outputs
        data.op_set_output(loop_, outinst.lo.expect("createCopyOps: out lo")).ok();
        data.op_set_input(hiop, ininst.hi.expect("createCopyOps: in hi"), 0).ok();
        data.op_set_input(loop_, ininst.lo.expect("createCopyOps: in lo"), 0).ok();
        self.tempsplits.push(hiop);
        self.tempsplits.push(loop_);
    }

    /// Check that `inst`, defined by `def` (a COPY), is really splittable
    /// (C++ `testDefiningCopy`).  Returns `(ok, istemp)`.
    fn test_defining_copy(&self, data: &Funcdata, inst: &SplitInstance, def: OpId) -> (bool, bool) {
        let invn = op_get_in(data, def, 0);
        let mut istemp = false;
        if !vn_is_constant(data, invn) {
            if vn_space_type(data, invn) != spacetype::IPTR_INTERNAL {
                let inrec = self.find_record(data, invn);
                let inrec = match inrec {
                    None => return (false, istemp),
                    Some(r) => r,
                };
                if inrec.splitoffset != inst.splitoffset {
                    return (false, istemp);
                }
                if !vn_is_free(data, invn) {
                    return (false, istemp);
                }
            } else {
                istemp = true;
            }
        }
        (true, istemp)
    }

    /// Do split of preferred-split Varnode that is defined by a COPY
    /// (C++ `splitDefiningCopy`).
    fn split_defining_copy(
        &mut self,
        data: &mut Funcdata,
        inst: &mut SplitInstance,
        def: OpId,
        istemp: bool,
    ) {
        let invn = op_get_in(data, def, 0);
        let mut ininst = SplitInstance::new(Some(invn), inst.splitoffset);
        let bigendian = vn_space_is_big_endian(data, inst.vn.expect("splitDefiningCopy"));
        Self::fillin_instance(data, inst, bigendian, true, true);
        Self::fillin_instance(data, &mut ininst, bigendian, true, true);
        self.create_copy_ops(data, &ininst, inst, def, istemp);
    }

    /// Check that `inst`, read by `readop` (a COPY), is really splittable
    /// (C++ `testReadingCopy`).  Returns `(ok, istemp)`.
    fn test_reading_copy(&self, data: &Funcdata, inst: &SplitInstance, readop: OpId) -> (bool, bool) {
        let outvn = op_get_out(data, readop).expect("testReadingCopy: no out");
        let mut istemp = false;
        if vn_space_type(data, outvn) != spacetype::IPTR_INTERNAL {
            let outrec = self.find_record(data, outvn);
            let outrec = match outrec {
                None => return (false, istemp),
                Some(r) => r,
            };
            if outrec.splitoffset != inst.splitoffset {
                return (false, istemp);
            }
        } else {
            istemp = true;
        }
        (true, istemp)
    }

    /// Do split of Varnode that is read by a COPY (C++ `splitReadingCopy`).
    fn split_reading_copy(
        &mut self,
        data: &mut Funcdata,
        inst: &mut SplitInstance,
        readop: OpId,
        istemp: bool,
    ) {
        let outvn = op_get_out(data, readop).expect("splitReadingCopy: no out");
        let mut outinst = SplitInstance::new(Some(outvn), inst.splitoffset);
        let bigendian = vn_space_is_big_endian(data, inst.vn.expect("splitReadingCopy"));
        Self::fillin_instance(data, inst, bigendian, true, true);
        Self::fillin_instance(data, &mut outinst, bigendian, true, true);
        // createCopyOps(inst,&outinst,readop,istemp): input is inst, output is outinst
        let in_copy = SplitInstance {
            splitoffset: inst.splitoffset,
            vn: inst.vn,
            hi: inst.hi,
            lo: inst.lo,
        };
        self.create_copy_ops(data, &in_copy, &outinst, readop, istemp);
    }

    /// Check that `inst`, defined by ZEXT, is really splittable (C++ `testZext`).
    fn test_zext(data: &Funcdata, inst: &SplitInstance, op: OpId) -> bool {
        let invn = op_get_in(data, op, 0);
        if vn_is_constant(data, invn) {
            return true;
        }
        let vn = inst.vn.expect("testZext");
        let bigendian = vn_space_is_big_endian(data, vn);
        let losize = if bigendian {
            vn_get_size(data, vn) - inst.splitoffset
        } else {
            inst.splitoffset
        };
        if vn_get_size(data, invn) != losize {
            return false;
        }
        true
    }

    /// C++ `splitZext`.
    fn split_zext(&mut self, data: &mut Funcdata, inst: &mut SplitInstance, op: OpId) {
        let invn0 = op_get_in(data, op, 0);
        let mut ininst = SplitInstance::new(Some(invn0), inst.splitoffset);
        let vn = inst.vn.expect("splitZext");
        let bigendian = vn_space_is_big_endian(data, vn);
        // C++ bigendian: hisize = splitoffset; losize = size - splitoffset
        //     little:    losize = splitoffset; hisize = size - splitoffset
        let (losize, hisize) = if bigendian {
            (vn_get_size(data, vn) - inst.splitoffset, inst.splitoffset)
        } else {
            (inst.splitoffset, vn_get_size(data, vn) - inst.splitoffset)
        };
        let ininst_vn = ininst.vn.expect("splitZext: ininst vn");
        if vn_is_constant(data, ininst_vn) {
            let origval = vn_get_offset(data, ininst_vn);
            let loval = origval & calc_mask(losize); // Split the constant into two pieces
            let hival = (origval >> (8 * losize)) & calc_mask(hisize);
            ininst.lo = Some(data.new_constant(losize, loval));
            ininst.hi = Some(data.new_constant(hisize, hival));
        } else {
            ininst.lo = ininst.vn;
            ininst.hi = Some(data.new_constant(hisize, 0));
        }

        Self::fillin_instance(data, inst, bigendian, true, true);
        self.create_copy_ops(data, &ininst, inst, op, false);
    }

    /// Check that `inst`, defined by PIECE, is really splittable (C++ `testPiece`).
    fn test_piece(data: &Funcdata, inst: &SplitInstance, op: OpId) -> bool {
        if vn_space_is_big_endian(data, inst.vn.expect("testPiece")) {
            if vn_get_size(data, op_get_in(data, op, 0)) != inst.splitoffset {
                return false;
            }
        } else if vn_get_size(data, op_get_in(data, op, 1)) != inst.splitoffset {
            return false;
        }
        true
    }

    /// C++ `splitPiece`.
    fn split_piece(data: &mut Funcdata, inst: &mut SplitInstance, op: OpId) {
        let mut loin = op_get_in(data, op, 1);
        let mut hiin = op_get_in(data, op, 0);
        let bigendian = vn_space_is_big_endian(data, inst.vn.expect("splitPiece"));
        Self::fillin_instance(data, inst, bigendian, true, true);
        let addr = op_get_addr(data, op);
        let hiop = data.new_op(1, addr.clone());
        let loop_ = data.new_op(1, addr);
        set_opcode(data, hiop, OpCode::CPUI_COPY);
        set_opcode(data, loop_, OpCode::CPUI_COPY);
        data.op_set_output(hiop, inst.hi.expect("splitPiece: hi")).ok(); // Outputs are the pieces of the original
        data.op_set_output(loop_, inst.lo.expect("splitPiece: lo")).ok();

        data.op_insert_after(loop_, op);
        data.op_insert_after(hiop, op);
        data.op_unset_input(op, 0);
        data.op_unset_input(op, 1);

        if vn_is_constant(data, hiin) {
            hiin = data.new_constant(vn_get_size(data, hiin), vn_get_offset(data, hiin));
        }
        // Input for the COPY of the most significant part comes from high part of PIECE
        data.op_set_input(hiop, hiin, 0).ok();
        if vn_is_constant(data, loin) {
            loin = data.new_constant(vn_get_size(data, loin), vn_get_offset(data, loin));
        }
        // Input for the COPY of the least significant part comes from low part of PIECE
        data.op_set_input(loop_, loin, 0).ok();
    }

    /// Check that `inst`, read by SUBPIECE, is really splittable
    /// (C++ `testSubpiece`).
    fn test_subpiece(data: &Funcdata, inst: &SplitInstance, op: OpId) -> bool {
        let vn = inst.vn.expect("testSubpiece");
        let outvn = op_get_out(data, op).expect("testSubpiece: no out");
        let suboff = vn_get_offset(data, op_get_in(data, op, 1)) as int4;
        if suboff == 0 {
            if vn_get_size(data, vn) - inst.splitoffset != vn_get_size(data, outvn) {
                return false;
            }
        } else {
            if vn_get_size(data, vn) - suboff != inst.splitoffset {
                return false;
            }
            if vn_get_size(data, outvn) != inst.splitoffset {
                return false;
            }
        }
        true
    }

    /// Knowing `op` is a SUBPIECE that extracts a logical piece from `inst`,
    /// rewrite it to a copy (C++ `splitSubpiece`).
    fn split_subpiece(data: &mut Funcdata, inst: &mut SplitInstance, op: OpId) {
        let vn = inst.vn.expect("splitSubpiece");
        let suboff = vn_get_offset(data, op_get_in(data, op, 1)) as int4;
        let grabbinglo = suboff == 0;

        let bigendian = vn_space_is_big_endian(data, vn);
        Self::fillin_instance(data, inst, bigendian, !grabbinglo, grabbinglo);
        set_opcode(data, op, OpCode::CPUI_COPY); // Change SUBPIECE to a copy
        data.op_remove_input(op, 1);

        // Input is most/least significant piece, depending on which the SUBPIECE extracts
        let invn = if grabbinglo {
            inst.lo.expect("splitSubpiece: lo")
        } else {
            inst.hi.expect("splitSubpiece: hi")
        };
        data.op_set_input(op, invn, 0).ok();
    }

    /// C++ `testLoad` (always true).
    fn test_load(_data: &Funcdata, _inst: &SplitInstance, _op: OpId) -> bool {
        true
    }

    /// Knowing `op` is a LOAD that defines the `inst` Varnode, split it into two
    /// pieces (C++ `splitLoad`).
    fn split_load(data: &mut Funcdata, inst: &mut SplitInstance, op: OpId) {
        let bigendian = vn_space_is_big_endian(data, inst.vn.expect("splitLoad"));
        Self::fillin_instance(data, inst, bigendian, true, true);
        let addr = op_get_addr(data, op);
        let hiop = data.new_op(2, addr.clone()); // Create two new LOAD ops
        let loop_ = data.new_op(2, addr.clone());
        let addop = data.new_op(2, addr);
        let ptrvn = op_get_in(data, op, 1);

        set_opcode(data, hiop, OpCode::CPUI_LOAD);
        set_opcode(data, loop_, OpCode::CPUI_LOAD);

        set_opcode(data, addop, OpCode::CPUI_INT_ADD); // Create a new ADD op to calculate and hold the second pointer

        data.op_insert_after(loop_, op);
        data.op_insert_after(hiop, op);
        data.op_insert_after(addop, op);
        data.op_unset_input(op, 1); // Free up ptrvn

        let ptrvn_size = vn_get_size(data, ptrvn);
        let addvn = data.new_unique_out(ptrvn_size, addop).expect("splitLoad: addvn");
        data.op_set_input(addop, ptrvn, 0).ok();
        let c = data.new_constant(ptrvn_size, inst.splitoffset as uintb);
        data.op_set_input(addop, c, 1).ok();

        data.op_set_output(hiop, inst.hi.expect("splitLoad: hi")).ok(); // Outputs are the pieces of the original
        data.op_set_output(loop_, inst.lo.expect("splitLoad: lo")).ok();
        let spaceid0 = op_get_in(data, op, 0);
        let spc = space_from_const(data, spaceid0);
        // Duplicate original spaceid into new LOADs
        let sp_size = vn_get_size(data, spaceid0);
        let sp_off = vn_get_offset(data, spaceid0);
        let spaceid_hi = data.new_constant(sp_size, sp_off);
        data.op_set_input(hiop, spaceid_hi, 0).ok();
        let spaceid_lo = data.new_constant(sp_size, sp_off);
        data.op_set_input(loop_, spaceid_lo, 0).ok();

        let mut ptrvn = ptrvn;
        if vn_is_free(data, ptrvn) {
            // Don't read a free varnode twice
            let space = vn_get_space(data, ptrvn);
            let off = vn_get_offset(data, ptrvn);
            ptrvn = data.new_varnode_space_off(ptrvn_size, space, off);
        }

        if spc.is_big_endian() {
            data.op_set_input(hiop, ptrvn, 1).ok();
            data.op_set_input(loop_, addvn, 1).ok();
        } else {
            data.op_set_input(hiop, addvn, 1).ok();
            data.op_set_input(loop_, ptrvn, 1).ok();
        }
    }

    /// C++ `testStore` (always true).
    fn test_store(_data: &Funcdata, _inst: &SplitInstance, _op: OpId) -> bool {
        true
    }

    /// Knowing `op` stores the value `inst`, split it in two (C++ `splitStore`).
    fn split_store(data: &mut Funcdata, inst: &mut SplitInstance, op: OpId) {
        let bigendian = vn_space_is_big_endian(data, inst.vn.expect("splitStore"));
        Self::fillin_instance(data, inst, bigendian, true, true);
        let addr = op_get_addr(data, op);
        let hiop = data.new_op(3, addr.clone()); // Create 2 new STOREs
        let loop_ = data.new_op(3, addr.clone());
        let addop = data.new_op(2, addr);
        let ptrvn = op_get_in(data, op, 1);

        set_opcode(data, hiop, OpCode::CPUI_STORE);
        set_opcode(data, loop_, OpCode::CPUI_STORE);

        set_opcode(data, addop, OpCode::CPUI_INT_ADD); // Create a new ADD op to calculate and hold the second pointer

        data.op_insert_after(loop_, op);
        data.op_insert_after(hiop, op);
        data.op_insert_after(addop, op);
        data.op_unset_input(op, 1); // Free up ptrvn
        data.op_unset_input(op, 2); // Free up inst

        let ptrvn_size = vn_get_size(data, ptrvn);
        let addvn = data.new_unique_out(ptrvn_size, addop).expect("splitStore: addvn");
        data.op_set_input(addop, ptrvn, 0).ok();
        let c = data.new_constant(ptrvn_size, inst.splitoffset as uintb);
        data.op_set_input(addop, c, 1).ok();

        data.op_set_input(hiop, inst.hi.expect("splitStore: hi"), 2).ok(); // Varnodes "being stored" are the pieces of the original
        data.op_set_input(loop_, inst.lo.expect("splitStore: lo"), 2).ok();
        let spaceid0 = op_get_in(data, op, 0);
        let spc = space_from_const(data, spaceid0);
        // Duplicate original spaceid into new STOREs
        let sp_size = vn_get_size(data, spaceid0);
        let sp_off = vn_get_offset(data, spaceid0);
        let spaceid_hi = data.new_constant(sp_size, sp_off);
        data.op_set_input(hiop, spaceid_hi, 0).ok();
        let spaceid_lo = data.new_constant(sp_size, sp_off);
        data.op_set_input(loop_, spaceid_lo, 0).ok();

        let mut ptrvn = ptrvn;
        if vn_is_free(data, ptrvn) {
            // Don't read a free varnode twice
            let space = vn_get_space(data, ptrvn);
            let off = vn_get_offset(data, ptrvn);
            ptrvn = data.new_varnode_space_off(ptrvn_size, space, off);
        }
        if spc.is_big_endian() {
            data.op_set_input(hiop, ptrvn, 1).ok();
            data.op_set_input(loop_, addvn, 1).ok();
        } else {
            data.op_set_input(hiop, addvn, 1).ok();
            data.op_set_input(loop_, ptrvn, 1).ok();
        }
    }

    /// Test if the instance Varnode can be readily split; if so, do the split
    /// (C++ `splitVarnode`).
    fn split_varnode(&mut self, data: &mut Funcdata, inst: &mut SplitInstance) -> bool {
        let vn = inst.vn.expect("splitVarnode: null vn");
        if vn_is_written(data, vn) {
            if !vn_has_no_descend(data, vn) {
                return false; // Already linked in
            }
            let op = vn_get_def(data, vn).expect("splitVarnode: written but no def");
            match op_code(data, op) {
                OpCode::CPUI_COPY => {
                    let (ok, istemp) = self.test_defining_copy(data, inst, op);
                    if !ok {
                        return false;
                    }
                    self.split_defining_copy(data, inst, op, istemp);
                }
                OpCode::CPUI_PIECE => {
                    if !Self::test_piece(data, inst, op) {
                        return false;
                    }
                    Self::split_piece(data, inst, op);
                }
                OpCode::CPUI_LOAD => {
                    if !Self::test_load(data, inst, op) {
                        return false;
                    }
                    Self::split_load(data, inst, op);
                }
                OpCode::CPUI_INT_ZEXT => {
                    if !Self::test_zext(data, inst, op) {
                        return false;
                    }
                    self.split_zext(data, inst, op);
                }
                _ => return false,
            }
            data.op_destroy(op);
        } else {
            if !vn_is_free(data, vn) {
                return false; // Make sure vn is not already a marked input
            }
            let op = match lone_descend(data, vn) {
                None => return false, // vn must be read exactly once
                Some(o) => o,
            };
            match op_code(data, op) {
                OpCode::CPUI_COPY => {
                    let (ok, istemp) = self.test_reading_copy(data, inst, op);
                    if !ok {
                        return false;
                    }
                    self.split_reading_copy(data, inst, op, istemp);
                }
                OpCode::CPUI_SUBPIECE => {
                    if !Self::test_subpiece(data, inst, op) {
                        return false;
                    }
                    Self::split_subpiece(data, inst, op);
                    return true; // Do not destroy op, it has been transformed
                }
                OpCode::CPUI_STORE => {
                    if !Self::test_store(data, inst, op) {
                        return false;
                    }
                    Self::split_store(data, inst, op);
                }
                _ => return false,
            }
            data.op_destroy(op); // Original op is now dead
        }
        true
    }

    /// Split every applicable Varnode matching a given record (C++ `splitRecord`).
    fn split_record(&mut self, data: &mut Funcdata, rec: &PreferSplitRecord) {
        let addr = rec.storage.get_addr();
        let size = rec.storage.size as int4;

        let mut inst = SplitInstance::new(None, rec.splitoffset);
        // iter = data->beginLoc(rec.storage.size,addr) .. endLoc(...)
        let mut worklist: Vec<VarnodeId> =
            data.vbank().iter_loc_size_addr(size, &addr).collect();
        let mut pos = 0;
        while pos < worklist.len() {
            inst.vn = Some(worklist[pos]);
            pos += 1;
            inst.lo = None;
            inst.hi = None;
            if self.split_varnode(data, &mut inst) {
                // If we found something, regenerate iterators, as they may be stale
                worklist = data.vbank().iter_loc_size_addr(size, &addr).collect();
                pos = 0;
            }
        }
    }

    /// Check that the temporary `inst` (and all of its readers) is splittable
    /// (C++ `testTemporary`).
    fn test_temporary(&self, data: &Funcdata, inst: &SplitInstance) -> bool {
        let vn = inst.vn.expect("testTemporary: null vn");
        let op = vn_get_def(data, vn).expect("testTemporary: no def");
        match op_code(data, op) {
            OpCode::CPUI_PIECE => {
                if !Self::test_piece(data, inst, op) {
                    return false;
                }
            }
            OpCode::CPUI_LOAD => {
                if !Self::test_load(data, inst, op) {
                    return false;
                }
            }
            OpCode::CPUI_INT_ZEXT => {
                if !Self::test_zext(data, inst, op) {
                    return false;
                }
            }
            _ => return false,
        }
        for readop in descend_ops(data, vn) {
            match op_code(data, readop) {
                OpCode::CPUI_SUBPIECE => {
                    if !Self::test_subpiece(data, inst, readop) {
                        return false;
                    }
                }
                OpCode::CPUI_STORE => {
                    if !Self::test_store(data, inst, readop) {
                        return false;
                    }
                }
                _ => return false,
            }
        }
        true
    }

    /// Split the temporary `inst` (C++ `splitTemporary`).
    fn split_temporary(&mut self, data: &mut Funcdata, inst: &mut SplitInstance) {
        let vn = inst.vn.expect("splitTemporary: null vn");
        let op = vn_get_def(data, vn).expect("splitTemporary: no def");
        match op_code(data, op) {
            OpCode::CPUI_PIECE => Self::split_piece(data, inst, op),
            OpCode::CPUI_LOAD => Self::split_load(data, inst, op),
            OpCode::CPUI_INT_ZEXT => self.split_zext(data, inst, op),
            _ => {}
        }

        // while(vn->beginDescend() != vn->endDescend()) { readop = *vn->beginDescend(); ... }
        loop {
            let readop = {
                let v = data.vbank().get(vn).expect("splitTemporary: stale vn");
                match v.descend_iter().next() {
                    None => break,
                    Some(o) => o,
                }
            };
            match op_code(data, readop) {
                OpCode::CPUI_SUBPIECE => {
                    Self::split_subpiece(data, inst, readop);
                }
                OpCode::CPUI_STORE => {
                    Self::split_store(data, inst, readop);
                    data.op_destroy(readop);
                }
                _ => {
                    // C++ default: break out of switch but loop continues; without
                    // a removal this would spin, but testTemporary guarantees only
                    // SUBPIECE/STORE readers reach here.  Match C++: no-op branch.
                    break;
                }
            }
        }
        data.op_destroy(op);
    }

    /// Find the split record that applies to `vn`, otherwise return `None`
    /// (C++ `findRecord`).
    ///
    /// The C++ does `lower_bound(records, templ)` then an equality probe
    /// (`if (templ < *iter) return 0`).  We replicate with `binary_search_by`
    /// over the sorted record vector against the synthesized template key.
    pub fn find_record(&self, data: &Funcdata, vn: VarnodeId) -> Option<&PreferSplitRecord> {
        let templ = PreferSplitRecord {
            storage: VarnodeData {
                space: Some(vn_get_space(data, vn)),
                size: vn_get_size(data, vn) as u32,
                offset: vn_get_offset(data, vn),
            },
            splitoffset: 0,
        };
        let tkey = templ.cmp_key();
        // lower_bound: first record whose key is >= tkey.
        let idx = self.records.partition_point(|r| r.cmp_key() < tkey);
        // if (iter == records->end()) return 0;
        let rec = self.records.get(idx)?;
        // if (templ < *iter) return 0;  (templ.key < rec.key)
        if tkey < rec.cmp_key() {
            return None;
        }
        Some(rec)
    }

    /// Sort a freshly built record table into the order `find_record` requires
    /// (C++ static `PreferSplitManager::initialize`).
    pub fn initialize(records: &mut [PreferSplitRecord]) {
        records.sort();
    }

    /// Run the split over all records (C++ `split`).
    pub fn split(&mut self, data: &mut Funcdata) {
        // for(i=0;i<records->size();++i) splitRecord((*records)[i]);
        let recs = std::mem::take(&mut self.records);
        for rec in &recs {
            self.split_record(data, rec);
        }
        self.records = recs;
    }

    /// Clean up temporaries produced by [`Self::split`] that need a further
    /// split (C++ `splitAdditional`).
    pub fn split_additional(&mut self, data: &mut Funcdata) {
        let mut defops: Vec<OpId> = Vec::new();
        let tempsplits = std::mem::take(&mut self.tempsplits);
        for &op in &tempsplits {
            // Look at everything connected to COPYs in -tempsplits-
            if op_is_dead(data, op) {
                continue;
            }
            let vn = op_get_in(data, op, 0);
            if vn_is_written(data, vn) {
                let defop = vn_get_def(data, vn).expect("splitAdditional: written but no def");
                if op_code(data, defop) == OpCode::CPUI_SUBPIECE {
                    // SUBPIECEs flowing into the COPY
                    let invn = op_get_in(data, defop, 0);
                    if vn_space_type(data, invn) == spacetype::IPTR_INTERNAL {
                        // Might be from a temporary that needs further splitting
                        defops.push(defop);
                    }
                }
            }
            let outvn = op_get_out(data, op).expect("splitAdditional: COPY has no out");
            for defop in descend_ops(data, outvn) {
                if op_code(data, defop) == OpCode::CPUI_PIECE {
                    // COPY flowing into PIECEs
                    let out2 = op_get_out(data, defop).expect("splitAdditional: PIECE no out");
                    if vn_space_type(data, out2) == spacetype::IPTR_INTERNAL {
                        // Might be to a temporary that needs further splitting
                        defops.push(defop);
                    }
                }
            }
        }
        self.tempsplits = tempsplits;
        for &op in &defops {
            if op_is_dead(data, op) {
                continue;
            }
            if op_code(data, op) == OpCode::CPUI_PIECE {
                let vn = op_get_out(data, op).expect("splitAdditional: PIECE no out");
                let splitoff = if vn_space_is_big_endian(data, vn) {
                    vn_get_size(data, op_get_in(data, op, 0))
                } else {
                    vn_get_size(data, op_get_in(data, op, 1))
                };
                let mut inst = SplitInstance::new(Some(vn), splitoff);
                if self.test_temporary(data, &inst) {
                    self.split_temporary(data, &mut inst);
                }
            } else if op_code(data, op) == OpCode::CPUI_SUBPIECE {
                let vn = op_get_in(data, op, 0);
                let suboff = vn_get_offset(data, op_get_in(data, op, 1));
                let splitoff = if vn_space_is_big_endian(data, vn) {
                    if suboff == 0 {
                        vn_get_size(data, vn)
                            - vn_get_size(data, op_get_out(data, op).expect("splitAdditional: SUBPIECE no out"))
                    } else {
                        vn_get_size(data, vn) - suboff as int4
                    }
                } else if suboff == 0 {
                    vn_get_size(data, op_get_out(data, op).expect("splitAdditional: SUBPIECE no out"))
                } else {
                    suboff as int4
                };
                let mut inst = SplitInstance::new(Some(vn), splitoff);
                if self.test_temporary(data, &inst) {
                    self.split_temporary(data, &mut inst);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests;
