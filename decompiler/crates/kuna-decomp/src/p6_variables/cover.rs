//! Port of `decompiler/cpp/cover.{hh,cc}` (W7, item `w7-s6-variable-cover`) —
//! the topological scope (`Cover`/`CoverBlock`) of a variable within a function
//! and the [`PcodeOpSet`] used for secondary intersection testing.
//!
//! ## ADR 0001 (IR arenas) realization
//!
//! The C++ `CoverBlock` stores `const PcodeOp *start`/`stop` raw pointers, with
//! three sentinel values (`(PcodeOp *)0` = beginning-of-block, `(PcodeOp *)1` =
//! end-of-block, `(PcodeOp *)2` = input).  In the arena model a `PcodeOp *` is
//! an [`OpId`], so a boundary point becomes a [`CoverPoint`] enum carrying the
//! three sentinels explicitly plus an `Op` variant.  The `Op` variant caches the
//! two things the C++ reads off the stored pointer — the `getUIndex` comparison
//! value (a `uintm`) and the op-code — so [`CoverBlock`] comparison/merge stay
//! pure (no op-arena access), exactly mirroring the C++ where those reads are on
//! the already-held pointer.  The single place that *resolves* a `PcodeOp *`
//! into its `uindex`/code is [`CoverContext`], filled by `Funcdata` (the cross-
//! arena boundary the W7 wave wires per `varnode.rs`/`funcdata.rs`).
//!
//! ## ADR 0002 (ordered containers) realization
//!
//! `Cover::cover` is a `BTreeMap<int4, CoverBlock>` (the C++ `map<int4,CoverBlock>`),
//! so `intersect`/`intersectList`/`merge`/`compareTo`'s two-cursor block walks
//! visit block indices in the same ascending order the C++ `std::map` iterator
//! does.  `PcodeOpSet::opList` is a `Vec` sorted by [`PcodeOpSet::compare_by_block`]
//! (block index, then `SeqNum::order`) exactly as `finalize`'s `std::sort`.

use std::collections::BTreeMap;

use kuna_base::address::Address;
use kuna_base::types::{int4, uintm};
use kuna_num::opcodes::OpCode;

use crate::context::OpId;

/// A boundary point of a [`CoverBlock`] (the C++ `const PcodeOp *` with its three
/// sentinel encodings).
///
/// The C++ stores `(const PcodeOp *)0/1/2` as special markers and any other
/// value as a real op pointer.  `Op` caches the `getUIndex` value and op-code so
/// the geometric methods need no op-arena access (see module docs).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoverPoint {
    /// `(const PcodeOp *)0` — the very beginning of the block.
    Begin,
    /// `(const PcodeOp *)1` — the very end of the block.
    End,
    /// `(const PcodeOp *)2` — an input (special marker), `getUIndex` == 0.
    Input,
    /// A real `PcodeOp *`: carries its `getUIndex` value and op-code.
    Op {
        /// The op id (C++ `const PcodeOp *`).
        id: OpId,
        /// Cached `CoverBlock::getUIndex(op)` value.
        uindex: uintm,
        /// Cached `op->code()`.
        code: OpCode,
    },
}

impl CoverPoint {
    /// The comparison index for this point (C++ `CoverBlock::getUIndex`,
    /// `cover.cc:29-49`).  The sentinel cases mirror the `switch(switchval)`:
    /// `Begin`/`Input` -> 0, `End` -> ~0; the `Op` case caches the value the C++
    /// computes from `getSeqNum().getOrder()` (with the MULTIEQUAL/INDIRECT
    /// special-casing already folded in when the point was constructed).
    pub fn get_uindex(self) -> uintm {
        match self {
            CoverPoint::Begin => 0,
            CoverPoint::End => uintm::MAX, // ~((uintm)0)
            CoverPoint::Input => 0,
            CoverPoint::Op { uindex, .. } => uindex,
        }
    }

    /// The op-code of a real-op point, used by `addRefRecurse`/`addRefPoint` to
    /// test for `CPUI_MULTIEQUAL`.  Sentinels have no code.
    fn code(self) -> Option<OpCode> {
        match self {
            CoverPoint::Op { code, .. } => Some(code),
            _ => None,
        }
    }
}

/// The topological scope of a variable within a basic block (C++ `class
/// CoverBlock`, `cover.hh:75-96`).
///
/// A `None` boundary models the C++ `(const PcodeOp *)0`; the `start`/`stop`
/// pair is encoded just as the C++ does (see [`CoverPoint`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CoverBlock {
    /// Beginning of the range (C++ `start`; `None` == `(PcodeOp *)0`).
    start: Option<CoverPoint>,
    /// End of the range (C++ `stop`; `None` == `(PcodeOp *)0`).
    stop: Option<CoverPoint>,
}

impl CoverBlock {
    /// Construct an empty/uncovered block (C++ default constructor).
    pub fn new() -> CoverBlock {
        CoverBlock { start: None, stop: None }
    }

    /// `getUIndex` of a possibly-null boundary (C++ `getUIndex` with the `0`
    /// sentinel folded into `CoverPoint::Begin == 0`).
    fn uindex(p: Option<CoverPoint>) -> uintm {
        match p {
            None => 0, // (const PcodeOp *)0 -> case 0 -> return 0
            Some(cp) => cp.get_uindex(),
        }
    }

    /// Get the start of the range (C++ `getStart`).
    pub fn get_start(&self) -> Option<CoverPoint> {
        self.start
    }
    /// Get the end of the range (C++ `getStop`).
    pub fn get_stop(&self) -> Option<CoverPoint> {
        self.stop
    }
    /// Clear \b this block to empty/uncovered (C++ `clear`).
    pub fn clear(&mut self) {
        self.start = None;
        self.stop = None;
    }
    /// Mark whole block as covered (C++ `setAll`): `start=0; stop=1`.
    pub fn set_all(&mut self) {
        self.start = None;
        self.stop = Some(CoverPoint::End);
    }
    /// Reset start of range (C++ `setBegin`): if `stop==0` set it to `1`.
    pub fn set_begin(&mut self, begin: Option<CoverPoint>) {
        self.start = begin;
        if self.stop.is_none() {
            self.stop = Some(CoverPoint::End);
        }
    }
    /// Reset end of range (C++ `setEnd`).
    pub fn set_end(&mut self, end: Option<CoverPoint>) {
        self.stop = end;
    }
    /// Return \b true if \b this is empty/uncovered (C++ `empty`):
    /// `start==0 && stop==0`.
    pub fn empty(&self) -> bool {
        self.start.is_none() && self.stop.is_none()
    }

    /// Characterize the intersection of \b this range with another CoverBlock
    /// (C++ `CoverBlock::intersect`, `cover.cc:59-102`).
    ///
    /// Returns 0 (no intersection), 1 (boundary-only), or 2 (whole interval).
    pub fn intersect(&self, op2: &CoverBlock) -> int4 {
        if self.empty() {
            return 0;
        }
        if op2.empty() {
            return 0;
        }

        let ustart = CoverBlock::uindex(self.start);
        let ustop = CoverBlock::uindex(self.stop);
        let u2start = CoverBlock::uindex(op2.start);
        let u2stop = CoverBlock::uindex(op2.stop);
        if ustart <= ustop {
            if u2start <= u2stop {
                // We are both one piece
                if (ustop <= u2start) || (u2stop <= ustart) {
                    if (ustart == u2stop) || (ustop == u2start) {
                        return 1; // Boundary intersection
                    } else {
                        return 0; // No intersection
                    }
                }
            } else {
                // They are two-piece, we are one-piece
                if (ustart >= u2stop) && (ustop <= u2start) {
                    if (ustart == u2stop) || (ustop == u2start) {
                        return 1;
                    } else {
                        return 0;
                    }
                }
            }
        } else if u2start <= u2stop {
            // They are one piece, we are two-piece
            if (u2start >= ustop) && (u2stop <= ustart) {
                if (u2start == ustop) || (u2stop == ustart) {
                    return 1;
                } else {
                    return 0;
                }
            }
        }
        // If both are two-pieces, then the intersection must be an interval
        2 // Interval intersection
    }

    /// Check containment of a given point (C++ `CoverBlock::contain`,
    /// `cover.cc:107-120`).
    pub fn contain(&self, point: Option<CoverPoint>) -> bool {
        if self.empty() {
            return false;
        }
        let upoint = CoverBlock::uindex(point);
        let ustart = CoverBlock::uindex(self.start);
        let ustop = CoverBlock::uindex(self.stop);

        if ustart <= ustop {
            return (upoint >= ustart) && (upoint <= ustop);
        }
        (upoint <= ustop) || (upoint >= ustart)
    }

    /// Characterize a point as a boundary (C++ `CoverBlock::boundary`,
    /// `cover.cc:129-142`): 0 = not on boundary, 1 = on tail, 2 = on defining
    /// point.
    pub fn boundary(&self, point: Option<CoverPoint>) -> int4 {
        if self.empty() {
            return 0;
        }
        let val = CoverBlock::uindex(point);
        if CoverBlock::uindex(self.start) == val {
            if self.start.is_some() {
                return 2;
            }
        }
        if CoverBlock::uindex(self.stop) == val {
            return 1;
        }
        0
    }

    /// Merge another CoverBlock into \b this (C++ `CoverBlock::merge`,
    /// `cover.cc:147-184`).
    pub fn merge(&mut self, op2: &CoverBlock) {
        if op2.empty() {
            return; // Nothing to merge in
        }
        if self.empty() {
            self.start = op2.start;
            self.stop = op2.stop;
            return;
        }
        let ustart = CoverBlock::uindex(self.start);
        let u2start = CoverBlock::uindex(op2.start);
        // Is start contained in op2
        let internal4 = (ustart == 0) && (op2.stop == Some(CoverPoint::End));
        let internal1 = internal4 || op2.contain(self.start);
        // Is op2.start contained in this
        let internal3 = (u2start == 0) && (self.stop == Some(CoverPoint::End));
        let internal2 = internal3 || self.contain(op2.start);

        if internal1 && internal2 && ((ustart != u2start) || internal3 || internal4) {
            // Covered entire block
            self.set_all();
            return;
        }
        if internal1 {
            self.start = op2.start; // Pick non-internal start
        } else if !internal2 {
            // Disjoint intervals
            if ustart < u2start {
                // Pick earliest start, then take other stop
                self.stop = op2.stop;
            } else {
                self.start = op2.start;
            }
            return;
        }
        if internal3 || op2.contain(self.stop) {
            // Pick non-internal stop
            self.stop = op2.stop;
        }
    }
}

/// The topological scope of a single variable object (C++ `class Cover`,
/// `cover.hh:108-131`).
#[derive(Debug, Clone, Default)]
pub struct Cover {
    /// block index -> CoverBlock (C++ `map<int4,CoverBlock> cover`).
    cover: BTreeMap<int4, CoverBlock>,
}

/// Global empty CoverBlock for blocks not covered (C++ `Cover::emptyBlock`).
const EMPTY_BLOCK: CoverBlock = CoverBlock { start: None, stop: None };

impl Cover {
    /// Construct an empty Cover.
    pub fn new() -> Cover {
        Cover { cover: BTreeMap::new() }
    }

    /// Clear \b this to an empty Cover (C++ `clear`).
    pub fn clear(&mut self) {
        self.cover.clear();
    }

    /// Iterate the `(block_index, CoverBlock)` entries in ascending block order
    /// (C++ `begin`/`end`).
    pub fn iter(&self) -> impl Iterator<Item = (int4, &CoverBlock)> {
        self.cover.iter().map(|(k, v)| (*k, v))
    }

    /// Give ordering of \b this and another Cover (C++ `Cover::compareTo`,
    /// `cover.cc:223-247`): compares the index of the first partly-covered block.
    pub fn compare_to(&self, op2: &Cover) -> int4 {
        let a = match self.cover.keys().next() {
            None => 1000000,
            Some(k) => *k,
        };
        let b = match op2.cover.keys().next() {
            None => 1000000,
            Some(k) => *k,
        };
        if a < b {
            -1
        } else if a == b {
            0
        } else {
            1
        }
    }

    /// Get the CoverBlock for the i-th block (C++ `getCoverBlock`,
    /// `cover.cc:253-260`): the global empty block when not present.
    pub fn get_cover_block(&self, i: int4) -> &CoverBlock {
        match self.cover.get(&i) {
            None => &EMPTY_BLOCK,
            Some(cb) => cb,
        }
    }

    /// Characterize the intersection between \b this and another Cover (C++
    /// `Cover::intersect`, `cover.cc:269-297`): 0/1/2 as `CoverBlock::intersect`.
    pub fn intersect(&self, op2: &Cover) -> int4 {
        let mut res = 0;
        let mut iter = self.cover.iter().peekable();
        let mut iter2 = op2.cover.iter().peekable();

        loop {
            let (k1, cb1) = match iter.peek() {
                None => return res,
                Some(&(k, v)) => (*k, v),
            };
            let (k2, cb2) = match iter2.peek() {
                None => return res,
                Some(&(k, v)) => (*k, v),
            };
            if k1 < k2 {
                iter.next();
            } else if k1 > k2 {
                iter2.next();
            } else {
                let newres = cb1.intersect(cb2);
                if newres == 2 {
                    return 2;
                }
                if newres == 1 {
                    res = 1; // At least a point intersection
                }
                iter.next();
                iter2.next();
            }
        }
    }

    /// Generate the list of blocks that intersect above `level` (C++
    /// `Cover::intersectList`, `cover.cc:307-334`).
    pub fn intersect_list(&self, listout: &mut Vec<int4>, op2: &Cover, level: int4) {
        listout.clear();
        let mut iter = self.cover.iter().peekable();
        let mut iter2 = op2.cover.iter().peekable();

        loop {
            let (k1, cb1) = match iter.peek() {
                None => return,
                Some(&(k, v)) => (*k, v),
            };
            let (k2, cb2) = match iter2.peek() {
                None => return,
                Some(&(k, v)) => (*k, v),
            };
            if k1 < k2 {
                iter.next();
            } else if k1 > k2 {
                iter2.next();
            } else {
                let val = cb1.intersect(cb2);
                if val >= level {
                    listout.push(k1);
                }
                iter.next();
                iter2.next();
            }
        }
    }

    /// Characterize the intersection on a specific block (C++
    /// `Cover::intersectByBlock`, `cover.cc:392-406`).
    pub fn intersect_by_block(&self, blk: int4, op2: &Cover) -> int4 {
        let iter = match self.cover.get(&blk) {
            None => return 0,
            Some(cb) => cb,
        };
        let iter2 = match op2.cover.get(&blk) {
            None => return 0,
            Some(cb) => cb,
        };
        iter.intersect(iter2)
    }

    /// Merge \b this with another Cover block by block (C++ `Cover::merge`,
    /// `cover.cc:465-472`).
    pub fn merge(&mut self, op2: &Cover) {
        for (k, cb) in op2.cover.iter() {
            self.cover.entry(*k).or_default().merge(cb);
        }
    }

    /// Reset to the single point where the given Varnode is defined (C++
    /// `Cover::addDefPoint`, `cover.cc:501-519`).
    ///
    /// `def` is the `(block_index, CoverPoint)` of the defining op (resolved by
    /// the caller from `vn->getDef()->getParent()->getIndex()`), or `None` when
    /// the Varnode is an input (the C++ `vn->isInput()` branch), in which case
    /// `is_input` distinguishes the input case from "no cover".
    pub fn add_def_point(&mut self, def: Option<(int4, CoverPoint)>, is_input: bool) {
        self.cover.clear();

        if let Some((blk, point)) = def {
            let block = self.cover.entry(blk).or_default();
            block.set_begin(Some(point)); // Set the point topology
            block.set_end(Some(point));
        } else if is_input {
            let block = self.cover.entry(0).or_default();
            block.set_begin(Some(CoverPoint::Input)); // Special mark for input
            block.set_end(Some(CoverPoint::Input));
        }
    }

    /// Add a single read of `vn` by op `ref_op` to \b this Cover (C++
    /// `Cover::addRefPoint(const PcodeOp *ref,const Varnode *vn)`, `cover.cc:489`).
    ///
    /// Resolves the `(block, ref_point, is_multiequal, pred_blocks)` of the read
    /// via the [`CoverContext`] then folds it in.  Used by
    /// `Merge::buildDominantCopy`, where the cover of a hypothetical dominant
    /// Varnode is built from `addDefPoint(domVn)` followed by `addRefPoint(*iter,
    /// outVn)` for each read `*iter` of a *different* Varnode (`outVn`), so it is
    /// distinct from the [`Cover::rebuild`] single-varnode walk.
    pub fn add_ref_point_for(&mut self, ctx: &dyn CoverContext, ref_op: OpId, vn: crate::context::VarnodeId) {
        let (bl, ref_point, is_multiequal, pred_blocks) = ctx.ref_point(ref_op, vn);
        self.add_ref_point(ctx, bl, ref_point, is_multiequal, &pred_blocks);
    }

    /// Add to \b this Cover recursively, starting at the bottom of the given
    /// block and filling backward until existing cover is hit (C++
    /// `Cover::addRefRecurse`, `cover.cc:524-558`).
    fn add_ref_recurse(&mut self, ctx: &dyn CoverContext, bl: int4) {
        let block = self.cover.entry(bl).or_default();
        if block.empty() {
            block.set_all(); // No cover encountered, fill in entire block
            let n = ctx.size_in(bl);
            for j in 0..n {
                self.add_ref_recurse(ctx, ctx.get_in(bl, j)); // Recurse to predecessors
            }
        } else {
            let op = block.get_stop();
            let ustart = CoverBlock::uindex(block.get_start());
            let ustop = CoverBlock::uindex(op);
            if (ustop != uintm::MAX) && (ustop >= ustart) {
                block.set_end(Some(CoverPoint::End)); // Fill in to the bottom
            }

            // re-read block since set_end mutated it; getStart() unchanged
            let start_is_zero = block.get_start().is_none();
            if (ustop == 0) && start_is_zero {
                if let Some(opcode) = op.and_then(|p| p.code()) {
                    if opcode == OpCode::CPUI_MULTIEQUAL {
                        // Block contains only an infinitesimal tip of cover
                        // through one branch of a MULTIEQUAL; still traverse.
                        let n = ctx.size_in(bl);
                        for j in 0..n {
                            self.add_ref_recurse(ctx, ctx.get_in(bl, j));
                        }
                    }
                }
            }
        }
    }

    /// Add a variable read to \b this Cover (C++ `Cover::addRefPoint`,
    /// `cover.cc:565-612`).
    ///
    /// `ref_op` is `(block_index, CoverPoint, code)` for the reading PcodeOp.
    /// `ref_inputs`, when the read is a MULTIEQUAL, lists the input slots whose
    /// Varnode equals `vn` (the C++ `ref->getIn(j)==vn` loop), as predecessor
    /// block indices to recurse into.
    fn add_ref_point(
        &mut self,
        ctx: &dyn CoverContext,
        bl: int4,
        ref_point: CoverPoint,
        is_multiequal: bool,
        multiequal_pred_blocks: &[int4],
    ) {
        let mut recurse_all = false;
        {
            let block = self.cover.entry(bl).or_default();
            if block.empty() {
                block.set_end(Some(ref_point));
            } else if block.contain(Some(ref_point)) {
                if !is_multiequal {
                    return;
                }
                // Even if MULTIEQUAL ref is contained we may be adding new
                // cover because we are looking at a different branch.
            } else {
                let op = block.get_stop();
                let startop = block.get_start();
                block.set_end(Some(ref_point)); // Otherwise update endpoint
                let ustop = CoverBlock::uindex(block.get_stop());
                if ustop >= CoverBlock::uindex(startop) {
                    if let Some(opcode) = op.and_then(|p| p.code()) {
                        // op is real (Some, not Input) by virtue of code() match.
                        if op != Some(CoverPoint::Input)
                            && opcode == OpCode::CPUI_MULTIEQUAL
                            && startop.is_none()
                        {
                            recurse_all = true; // Traverse predecessors below
                        }
                    }
                    if !recurse_all {
                        return;
                    }
                }
            }
        }

        if recurse_all {
            let n = ctx.size_in(bl);
            for j in 0..n {
                self.add_ref_recurse(ctx, ctx.get_in(bl, j));
            }
            return;
        }

        if is_multiequal {
            for &pred in multiequal_pred_blocks {
                self.add_ref_recurse(ctx, pred);
            }
        } else {
            let n = ctx.size_in(bl);
            for j in 0..n {
                self.add_ref_recurse(ctx, ctx.get_in(bl, j));
            }
        }
    }

    /// Reset \b this based on def-use of a single Varnode (C++ `Cover::rebuild`,
    /// `cover.cc:477-496`).
    ///
    /// The def/use graph walk is driven by [`CoverContext`]; the implied-output
    /// chase (`outVn->isImplied()`) keeps the exact `path` BFS order of the C++.
    pub fn rebuild(&mut self, ctx: &dyn CoverContext, vn: crate::context::VarnodeId) {
        let mut path = vec![vn];
        let mut pos = 0usize;

        // addDefPoint(vn)
        let (def, is_input) = ctx.def_point(vn);
        self.add_def_point(def, is_input);

        while pos < path.len() {
            let cur_vn = path[pos];
            pos += 1;
            for op in ctx.descend(cur_vn) {
                let (bl, ref_point, is_multiequal, pred_blocks) = ctx.ref_point(op, vn);
                self.add_ref_point(ctx, bl, ref_point, is_multiequal, &pred_blocks);
                if let Some(out_vn) = ctx.out_implied(op) {
                    path.push(out_vn);
                }
            }
        }
    }

    /// Check the definition of a Varnode for containment (C++
    /// `Cover::containVarnodeDef`, `cover.cc:441-462`).
    ///
    /// `def` is `(block_index, CoverPoint)` for the defining op, or `None` for an
    /// input (the C++ `op = (const PcodeOp *)2; blk = 0;` branch).
    /// Returns 0 (not contained), 1 (interior), 2 (defining points intersect),
    /// 3 (Cover's tail is the varnode definition).
    pub fn contain_varnode_def(&self, def: Option<(int4, CoverPoint)>) -> int4 {
        let (blk, point) = match def {
            None => (0, Some(CoverPoint::Input)), // input -> (PcodeOp *)2
            Some((b, p)) => (b, Some(p)),
        };
        let cb = match self.cover.get(&blk) {
            None => return 0,
            Some(cb) => cb,
        };
        if cb.contain(point) {
            let boundtype = cb.boundary(point);
            if boundtype == 0 {
                return 1;
            }
            if boundtype == 2 {
                return 2;
            }
            return 3;
        }
        0
    }

    /// Does \b this contain the given PcodeOp (C++ `Cover::contain`,
    /// `cover.cc:413-425`): `max` 1 tests any containment, 2 forces interior.
    pub fn contain(&self, op_blk: int4, op_point: CoverPoint, max: int4) -> bool {
        let cb = match self.cover.get(&op_blk) {
            None => return false,
            Some(cb) => cb,
        };
        if cb.contain(Some(op_point)) {
            if max == 1 {
                return true;
            }
            if 0 == cb.boundary(Some(op_point)) {
                return true;
            }
        }
        false
    }

    /// Does \b this cover any PcodeOp in the given PcodeOpSet (C++
    /// `Cover::intersect(const PcodeOpSet &,Varnode *)`, `cover.cc:342-382`).
    ///
    /// `rep_addr` is the address of the representative Varnode for the secondary
    /// `affectsTest` (the C++ passes `Varnode *vn` and reads only `vn->getAddr()`
    /// in `StackAffectingOps::affectsTest`, so we thread the resolved address).
    pub fn intersect_op_set(&self, op_set: &PcodeOpSet, rep_addr: &Address) -> bool {
        if op_set.op_list.is_empty() {
            return false;
        }
        let mut set_block = 0usize;
        let mut op_index = op_set.block_start[set_block] as usize;
        let mut set_index = op_set.op_list[op_index].block_index;
        let first_block = op_set.op_list[0].block_index;
        let mut cover_iter = self.cover.range(first_block..).peekable();
        while let Some((&cover_index, cover_block)) = cover_iter.peek().copied() {
            if cover_index < set_index {
                cover_iter.next();
            } else if cover_index > set_index {
                set_block += 1;
                if set_block >= op_set.block_start.len() {
                    break;
                }
                op_index = op_set.block_start[set_block] as usize;
                set_index = op_set.op_list[op_index].block_index;
            } else {
                let cover_block = *cover_block;
                cover_iter.next();
                let mut op_max = op_set.op_list.len();
                set_block += 1;
                if set_block < op_set.block_start.len() {
                    op_max = op_set.block_start[set_block] as usize;
                }
                loop {
                    let entry = &op_set.op_list[op_index];
                    if cover_block.contain(Some(entry.point)) {
                        // Does range contain the call?
                        if cover_block.boundary(Some(entry.point)) == 0 {
                            // Is the call on the boundary
                            if op_set.affects_test(entry.id, rep_addr) {
                                return true;
                            }
                        }
                    }
                    op_index += 1;
                    if op_index >= op_max {
                        break;
                    }
                }
                if set_block >= op_set.block_start.len() {
                    break;
                }
            }
        }
        false
    }
}

/// Cross-arena graph access for the [`Cover`] def/use walk (the C++ reads off the
/// held `Varnode *`/`PcodeOp *`/`FlowBlock *` directly; here `Funcdata` supplies
/// them).  Implemented in `funcdata.rs` per the W7 boundary.
pub trait CoverContext {
    /// `bl->sizeIn()` — number of predecessor blocks of block index `bl`.
    fn size_in(&self, bl: int4) -> int4;
    /// `bl->getIn(j)->getIndex()` — index of the j-th predecessor of block `bl`.
    fn get_in(&self, bl: int4, j: int4) -> int4;
    /// `addDefPoint`'s resolution of `vn`: the `(block_index, CoverPoint)` of its
    /// defining op (`None` if it has no def), and whether `vn->isInput()`.
    fn def_point(&self, vn: crate::context::VarnodeId) -> (Option<(int4, CoverPoint)>, bool);
    /// `vn->beginDescend()..endDescend()` — the reading ops, in C++ order.
    fn descend(&self, vn: crate::context::VarnodeId) -> Vec<OpId>;
    /// `addRefPoint` inputs for reading op `op` of Varnode `vn`:
    /// `(block_index, ref_point, is_multiequal, multiequal_pred_blocks)`.
    /// `multiequal_pred_blocks` lists the predecessor block index for each input
    /// slot `j` with `ref->getIn(j)==vn` (only meaningful when `is_multiequal`).
    fn ref_point(
        &self,
        op: OpId,
        vn: crate::context::VarnodeId,
    ) -> (int4, CoverPoint, bool, Vec<int4>);
    /// `op->getOut()` when it exists and `isImplied()` (the rebuild path-chase);
    /// `None` otherwise.
    fn out_implied(&self, op: OpId) -> Option<crate::context::VarnodeId>;
}

/// One entry of a [`PcodeOpSet`] — a `PcodeOp *` resolved into the fields the set
/// reads (block index, the [`CoverPoint`] used for containment, and the op id for
/// the secondary `affectsTest`).
#[derive(Debug, Clone, Copy)]
pub struct PcodeOpSetEntry {
    /// The op id (C++ `PcodeOp *`).
    pub id: OpId,
    /// `op->getParent()->getIndex()`.
    pub block_index: int4,
    /// The `CoverBlock::contain`/`boundary` point for this op.
    pub point: CoverPoint,
    /// `op->getSeqNum().getOrder()` — the `compareByBlock` within-block key.
    pub order: uintm,
}

/// A set of PcodeOps that can be tested for Cover intersections (C++ `class
/// PcodeOpSet`, `cover.hh:35-65`).
///
/// The C++ `affectsTest` is a pure virtual the owner overrides; here it is a
/// boxed closure supplied at construction, since the only owner (the call-cross
/// intersection test in `variable.cc`) is data-driven.
pub struct PcodeOpSet {
    /// Ops in this set, sorted on block index then `SeqNum::order` (C++ `opList`).
    op_list: Vec<PcodeOpSetEntry>,
    /// Index of first op in each non-empty block (C++ `blockStart`).
    block_start: Vec<int4>,
    /// Has `populate()` been called (C++ `is_pop`).
    is_pop: bool,
    /// The `affectsTest` secondary test (C++ pure-virtual override).  The C++
    /// `affectsTest(PcodeOp *op,Varnode *vn)` reads only `vn->getAddr()`, so the
    /// closure takes the resolved `Address`.
    affects: Box<dyn Fn(OpId, &Address) -> bool>,
    /// The `populate()` call-back: returns the unsorted op entries to add.
    populate_fn: Box<dyn Fn() -> Vec<PcodeOpSetEntry>>,
}

impl PcodeOpSet {
    /// Construct an empty, unpopulated set (C++ `PcodeOpSet()`), with the
    /// `affectsTest` override and the `populate()` call-back.
    pub fn new(
        populate_fn: Box<dyn Fn() -> Vec<PcodeOpSetEntry>>,
        affects: Box<dyn Fn(OpId, &Address) -> bool>,
    ) -> PcodeOpSet {
        PcodeOpSet {
            op_list: Vec::new(),
            block_start: Vec::new(),
            is_pop: false,
            affects,
            populate_fn,
        }
    }

    /// Return \b true if \b this set is populated (C++ `isPopulated`).
    pub fn is_populated(&self) -> bool {
        self.is_pop
    }

    /// Clear all PcodeOps in \b this (C++ `clear`).
    pub fn clear(&mut self) {
        self.is_pop = false;
        self.op_list.clear();
        self.block_start.clear();
    }

    /// Secondary test that the given PcodeOp affects the Varnode (C++
    /// `affectsTest`).
    fn affects_test(&self, op: OpId, vn_addr: &Address) -> bool {
        (self.affects)(op, vn_addr)
    }

    /// Add a single resolved op entry directly (the owner-driven `populate()`
    /// path: `Funcdata::populateAffectingOps` adds each op via `addOp`).  The
    /// caller must follow with [`finalize_pub`](Self::finalize_pub).
    pub fn add_op_entry(&mut self, entry: PcodeOpSetEntry) {
        self.op_list.push(entry);
    }

    /// Install the secondary `affectsTest` (C++ pure-virtual override) — used by
    /// the owner-driven populate path to bind the freshly snapshotted
    /// store-guard data.
    pub fn set_affects(&mut self, affects: Box<dyn Fn(OpId, &Address) -> bool>) {
        self.affects = affects;
    }

    /// Sort the directly-added ops into blocks and mark the set populated (C++
    /// `finalize`).  Pairs with [`add_op_entry`](Self::add_op_entry).
    pub fn finalize_pub(&mut self) {
        self.finalize();
    }

    /// Compare PcodeOps for \b this set: block index, then `SeqNum::order` (C++
    /// `PcodeOpSet::compareByBlock`, `cover.cc:646-652`).
    fn compare_by_block(a: &PcodeOpSetEntry, b: &PcodeOpSetEntry) -> std::cmp::Ordering {
        if a.block_index != b.block_index {
            return a.block_index.cmp(&b.block_index);
        }
        a.order.cmp(&b.order)
    }

    /// Populate \b this by calling the owner's `populate()` call-back, then
    /// `finalize()` (C++ `populate` override + `finalize`, `cover.cc:627-640`).
    pub fn populate(&mut self) {
        self.op_list = (self.populate_fn)();
        self.finalize();
    }

    /// Sort ops in the set into blocks (C++ `PcodeOpSet::finalize`,
    /// `cover.cc:627-640`).
    fn finalize(&mut self) {
        self.op_list.sort_by(PcodeOpSet::compare_by_block);
        let mut block_num: int4 = -1;
        for (i, entry) in self.op_list.iter().enumerate() {
            let new_block_num = entry.block_index;
            if new_block_num > block_num {
                self.block_start.push(i as int4);
                block_num = new_block_num;
            }
        }
        self.is_pop = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use slotmap::KeyData;

    /// Build a real-op CoverPoint with a synthetic order (non-MULTIEQUAL).
    fn op_point(uindex: uintm) -> CoverPoint {
        // The arena OpId is irrelevant for the geometric tests; fabricate one.
        let id = OpId::from(KeyData::from_ffi(1));
        CoverPoint::Op { id, uindex, code: OpCode::CPUI_COPY }
    }

    fn meq_point(uindex: uintm) -> CoverPoint {
        let id = OpId::from(KeyData::from_ffi(2));
        CoverPoint::Op { id, uindex, code: OpCode::CPUI_MULTIEQUAL }
    }

    #[test]
    fn coverblock_empty_and_sentinels() {
        let cb = CoverBlock::new();
        assert!(cb.empty());
        assert_eq!(CoverPoint::Begin.get_uindex(), 0);
        assert_eq!(CoverPoint::Input.get_uindex(), 0);
        assert_eq!(CoverPoint::End.get_uindex(), uintm::MAX);
    }

    #[test]
    fn coverblock_setall_covers_whole() {
        let mut cb = CoverBlock::new();
        cb.set_all();
        assert!(!cb.empty());
        assert_eq!(cb.get_start(), None);
        assert_eq!(cb.get_stop(), Some(CoverPoint::End));
    }

    #[test]
    fn coverblock_setbegin_seeds_stop() {
        let mut cb = CoverBlock::new();
        cb.set_begin(Some(op_point(10)));
        assert_eq!(cb.get_start(), Some(op_point(10)));
        // stop was 0 -> seeded to End (1)
        assert_eq!(cb.get_stop(), Some(CoverPoint::End));
    }

    #[test]
    fn coverblock_point_intersect() {
        // A def-point block [10,10]
        let mut a = CoverBlock::new();
        a.set_begin(Some(op_point(10)));
        a.set_end(Some(op_point(10)));
        // A def-point block [10,10] again: boundary intersection at single point
        let mut b = CoverBlock::new();
        b.set_begin(Some(op_point(10)));
        b.set_end(Some(op_point(10)));
        // ustart=ustop=10, both single point -> ustop<=u2start (10<=10) true and
        // ustart==u2stop -> boundary intersection (1).
        assert_eq!(a.intersect(&b), 1);
    }

    #[test]
    fn coverblock_interval_intersect() {
        // [5,20] vs [10,30] -> interval overlap (2)
        let mut a = CoverBlock::new();
        a.set_begin(Some(op_point(5)));
        a.set_end(Some(op_point(20)));
        let mut b = CoverBlock::new();
        b.set_begin(Some(op_point(10)));
        b.set_end(Some(op_point(30)));
        assert_eq!(a.intersect(&b), 2);
    }

    #[test]
    fn coverblock_no_intersect() {
        // [5,9] vs [10,20] -> disjoint (0)
        let mut a = CoverBlock::new();
        a.set_begin(Some(op_point(5)));
        a.set_end(Some(op_point(9)));
        let mut b = CoverBlock::new();
        b.set_begin(Some(op_point(10)));
        b.set_end(Some(op_point(20)));
        assert_eq!(a.intersect(&b), 0);
    }

    #[test]
    fn coverblock_contain_and_boundary() {
        // [10,20]
        let mut a = CoverBlock::new();
        a.set_begin(Some(op_point(10)));
        a.set_end(Some(op_point(20)));
        assert!(a.contain(Some(op_point(15))));
        assert!(a.contain(Some(op_point(10))));
        assert!(!a.contain(Some(op_point(25))));
        // boundary: start point with non-zero start -> 2 (defining point)
        assert_eq!(a.boundary(Some(op_point(10))), 2);
        // tail
        assert_eq!(a.boundary(Some(op_point(20))), 1);
        // interior
        assert_eq!(a.boundary(Some(op_point(15))), 0);
    }

    #[test]
    fn coverblock_merge_disjoint_then_full() {
        // [5,10] merge [20,25]: disjoint, earliest start (5) keeps, takes stop 25
        let mut a = CoverBlock::new();
        a.set_begin(Some(op_point(5)));
        a.set_end(Some(op_point(10)));
        let mut b = CoverBlock::new();
        b.set_begin(Some(op_point(20)));
        b.set_end(Some(op_point(25)));
        a.merge(&b);
        assert_eq!(a.get_start(), Some(op_point(5)));
        assert_eq!(a.get_stop(), Some(op_point(25)));
    }

    #[test]
    fn coverblock_merge_into_empty() {
        let mut a = CoverBlock::new();
        let mut b = CoverBlock::new();
        b.set_begin(Some(op_point(5)));
        b.set_end(Some(op_point(10)));
        a.merge(&b);
        assert_eq!(a.get_start(), Some(op_point(5)));
        assert_eq!(a.get_stop(), Some(op_point(10)));
        // merging empty into non-empty is a no-op
        let mut c = CoverBlock::new();
        a.merge(&c);
        assert_eq!(a.get_start(), Some(op_point(5)));
        c.clear();
    }

    #[test]
    fn cover_compare_to_and_block_walk() {
        let mut a = Cover::new();
        a.cover.insert(2, {
            let mut cb = CoverBlock::new();
            cb.set_all();
            cb
        });
        let mut b = Cover::new();
        b.cover.insert(5, {
            let mut cb = CoverBlock::new();
            cb.set_all();
            cb
        });
        // a's first block (2) < b's first block (5) -> -1
        assert_eq!(a.compare_to(&b), -1);
        assert_eq!(b.compare_to(&a), 1);
        // empty cover compares as 1000000
        let e = Cover::new();
        assert_eq!(a.compare_to(&e), -1);
        assert_eq!(e.compare_to(&a), 1);
    }

    #[test]
    fn cover_intersect_matrix_diamond() {
        // Two covers sharing block 1 with overlapping intervals -> 2
        let mut a = Cover::new();
        let mut acb = CoverBlock::new();
        acb.set_begin(Some(op_point(5)));
        acb.set_end(Some(op_point(20)));
        a.cover.insert(1, acb);
        let mut b = Cover::new();
        let mut bcb = CoverBlock::new();
        bcb.set_begin(Some(op_point(10)));
        bcb.set_end(Some(op_point(30)));
        b.cover.insert(1, bcb);
        assert_eq!(a.intersect(&b), 2);
        assert_eq!(a.intersect_by_block(1, &b), 2);
        // disjoint blocks -> no intersection
        let mut c = Cover::new();
        let mut ccb = CoverBlock::new();
        ccb.set_all();
        c.cover.insert(7, ccb);
        assert_eq!(a.intersect(&c), 0);
    }

    #[test]
    fn cover_intersect_list_threshold() {
        let mut a = Cover::new();
        let mut b = Cover::new();
        // block 1: point intersect (1)
        let mut a1 = CoverBlock::new();
        a1.set_begin(Some(op_point(10)));
        a1.set_end(Some(op_point(10)));
        a.cover.insert(1, a1);
        let mut b1 = CoverBlock::new();
        b1.set_begin(Some(op_point(10)));
        b1.set_end(Some(op_point(10)));
        b.cover.insert(1, b1);
        // block 2: interval intersect (2)
        let mut a2 = CoverBlock::new();
        a2.set_begin(Some(op_point(5)));
        a2.set_end(Some(op_point(20)));
        a.cover.insert(2, a2);
        let mut b2 = CoverBlock::new();
        b2.set_begin(Some(op_point(10)));
        b2.set_end(Some(op_point(30)));
        b.cover.insert(2, b2);

        let mut out = Vec::new();
        a.intersect_list(&mut out, &b, 2); // only interval (level 2)
        assert_eq!(out, vec![2]);
        a.intersect_list(&mut out, &b, 1); // both
        assert_eq!(out, vec![1, 2]);
    }

    #[test]
    fn cover_merge_block_by_block() {
        let mut a = Cover::new();
        let mut a1 = CoverBlock::new();
        a1.set_begin(Some(op_point(5)));
        a1.set_end(Some(op_point(10)));
        a.cover.insert(1, a1);

        let mut b = Cover::new();
        let mut b1 = CoverBlock::new();
        b1.set_begin(Some(op_point(20)));
        b1.set_end(Some(op_point(25)));
        b.cover.insert(1, b1);
        let mut b2 = CoverBlock::new();
        b2.set_all();
        b.cover.insert(2, b2);

        a.merge(&b);
        // block 1 merged disjoint -> [5,25]
        assert_eq!(a.get_cover_block(1).get_start(), Some(op_point(5)));
        assert_eq!(a.get_cover_block(1).get_stop(), Some(op_point(25)));
        // block 2 brought over whole
        assert_eq!(a.get_cover_block(2).get_stop(), Some(CoverPoint::End));
    }

    #[test]
    fn cover_add_def_point_input_vs_def() {
        // input
        let mut c = Cover::new();
        c.add_def_point(None, true);
        let cb = c.get_cover_block(0);
        assert_eq!(cb.get_start(), Some(CoverPoint::Input));
        assert_eq!(cb.get_stop(), Some(CoverPoint::Input));
        // def at block 3
        let mut c2 = Cover::new();
        c2.add_def_point(Some((3, op_point(7))), false);
        let cb2 = c2.get_cover_block(3);
        assert_eq!(cb2.get_start(), Some(op_point(7)));
        assert_eq!(cb2.get_stop(), Some(op_point(7)));
        // free (no def, not input) -> empty cover
        let mut c3 = Cover::new();
        c3.add_def_point(None, false);
        assert_eq!(c3.iter().count(), 0);
    }

    #[test]
    fn cover_contain_varnode_def() {
        // Cover with block 2 = [10,20]; def at (2,10) is on the start boundary
        let mut c = Cover::new();
        let mut cb = CoverBlock::new();
        cb.set_begin(Some(op_point(10)));
        cb.set_end(Some(op_point(20)));
        c.cover.insert(2, cb);
        // def point (2,10) is on start boundary -> 2
        assert_eq!(c.contain_varnode_def(Some((2, op_point(10)))), 2);
        // interior (2,15) -> 1
        assert_eq!(c.contain_varnode_def(Some((2, op_point(15)))), 1);
        // tail (2,20) -> 3
        assert_eq!(c.contain_varnode_def(Some((2, op_point(20)))), 3);
        // not present block -> 0
        assert_eq!(c.contain_varnode_def(Some((9, op_point(1)))), 0);
    }

    /// A diamond CFG cover rebuild via CoverContext: def in block 0, read in
    /// block 3 (the join). Exercises addRefRecurse filling blocks 1,2 fully and
    /// block 0 to the bottom.
    struct DiamondCtx;
    impl CoverContext for DiamondCtx {
        fn size_in(&self, bl: int4) -> int4 {
            match bl {
                0 => 0,       // entry
                1 | 2 => 1,   // each branch has one pred (block 0)
                3 => 2,       // join has two preds (1,2)
                _ => 0,
            }
        }
        fn get_in(&self, bl: int4, j: int4) -> int4 {
            match (bl, j) {
                (1, 0) => 0,
                (2, 0) => 0,
                (3, 0) => 1,
                (3, 1) => 2,
                _ => panic!("bad pred"),
            }
        }
        fn def_point(&self, _vn: crate::context::VarnodeId) -> (Option<(int4, CoverPoint)>, bool) {
            // defined at block 0, order 5
            (Some((0, op_point(5))), false)
        }
        fn descend(&self, _vn: crate::context::VarnodeId) -> Vec<OpId> {
            // single reader op
            vec![OpId::from(KeyData::from_ffi(100))]
        }
        fn ref_point(
            &self,
            _op: OpId,
            _vn: crate::context::VarnodeId,
        ) -> (int4, CoverPoint, bool, Vec<int4>) {
            // read in block 3 at order 9, not a MULTIEQUAL
            (3, op_point(9), false, vec![])
        }
        fn out_implied(&self, _op: OpId) -> Option<crate::context::VarnodeId> {
            None
        }
    }

    #[test]
    fn cover_rebuild_diamond() {
        let ctx = DiamondCtx;
        let mut c = Cover::new();
        let vn = crate::context::VarnodeId::from(KeyData::from_ffi(1));
        c.rebuild(&ctx, vn);
        // block 0: def at 5, filled to bottom (End)
        let cb0 = c.get_cover_block(0);
        assert_eq!(cb0.get_start(), Some(op_point(5)));
        assert_eq!(cb0.get_stop(), Some(CoverPoint::End));
        // block 3: ref endpoint at 9, start is begin
        let cb3 = c.get_cover_block(3);
        assert_eq!(cb3.get_stop(), Some(op_point(9)));
        // blocks 1,2 filled whole (setAll: start None, stop End)
        let cb1 = c.get_cover_block(1);
        assert!(!cb1.empty());
        assert_eq!(cb1.get_stop(), Some(CoverPoint::End));
        let cb2 = c.get_cover_block(2);
        assert!(!cb2.empty());
    }

    /// A loop CFG: def in block 0, MULTIEQUAL read in block 1 (the loop head)
    /// pulling cover back through the back-edge block 2.
    struct LoopCtx;
    impl CoverContext for LoopCtx {
        fn size_in(&self, bl: int4) -> int4 {
            match bl {
                0 => 0,     // entry
                1 => 2,     // loop head: pred 0 (entry) and 2 (back edge)
                2 => 1,     // loop body: pred 1
                _ => 0,
            }
        }
        fn get_in(&self, bl: int4, j: int4) -> int4 {
            match (bl, j) {
                (1, 0) => 0,
                (1, 1) => 2,
                (2, 0) => 1,
                _ => panic!("bad pred"),
            }
        }
        fn def_point(&self, _vn: crate::context::VarnodeId) -> (Option<(int4, CoverPoint)>, bool) {
            (Some((0, op_point(3))), false)
        }
        fn descend(&self, _vn: crate::context::VarnodeId) -> Vec<OpId> {
            vec![OpId::from(KeyData::from_ffi(200))]
        }
        fn ref_point(
            &self,
            _op: OpId,
            _vn: crate::context::VarnodeId,
        ) -> (int4, CoverPoint, bool, Vec<int4>) {
            // MULTIEQUAL in block 1 reading vn from the back-edge slot (pred 2)
            (1, meq_point(0), true, vec![2])
        }
        fn out_implied(&self, _op: OpId) -> Option<crate::context::VarnodeId> {
            None
        }
    }

    #[test]
    fn cover_rebuild_loop_multiequal() {
        let ctx = LoopCtx;
        let mut c = Cover::new();
        let vn = crate::context::VarnodeId::from(KeyData::from_ffi(1));
        c.rebuild(&ctx, vn);
        // def block 0 present, filled to bottom
        let cb0 = c.get_cover_block(0);
        assert_eq!(cb0.get_start(), Some(op_point(3)));
        // back-edge block 2 filled whole (recursed via MULTIEQUAL pred)
        let cb2 = c.get_cover_block(2);
        assert!(!cb2.empty());
    }

    #[test]
    fn pcodeopset_finalize_sorts_and_blocks() {
        let populate = || {
            vec![
                PcodeOpSetEntry {
                    id: OpId::from(KeyData::from_ffi(1)),
                    block_index: 2,
                    point: op_point(50),
                    order: 50,
                },
                PcodeOpSetEntry {
                    id: OpId::from(KeyData::from_ffi(2)),
                    block_index: 0,
                    point: op_point(20),
                    order: 20,
                },
                PcodeOpSetEntry {
                    id: OpId::from(KeyData::from_ffi(3)),
                    block_index: 0,
                    point: op_point(10),
                    order: 10,
                },
            ]
        };
        let mut set = PcodeOpSet::new(Box::new(populate), Box::new(|_, _| false));
        assert!(!set.is_populated());
        set.populate();
        assert!(set.is_populated());
        // sorted: block 0 order 10, block 0 order 20, block 2 order 50
        assert_eq!(set.op_list[0].block_index, 0);
        assert_eq!(set.op_list[0].order, 10);
        assert_eq!(set.op_list[1].order, 20);
        assert_eq!(set.op_list[2].block_index, 2);
        // blockStart: index 0 (block 0), index 2 (block 2)
        assert_eq!(set.block_start, vec![0, 2]);
    }

    /// The owner-driven populate path used by `StackAffectingOps::populate`
    /// (`Funcdata::populate_affecting_ops`): direct `add_op_entry` + `finalize_pub`
    /// + `set_affects` reproduce the same sorted/blocked layout as the closure
    /// `populate()`, and the freshly-bound `affectsTest` runs against the resolved
    /// address (verify_w10_chainb_gap1).
    #[test]
    fn verify_w10_chainb_gap1_owner_driven_populate_and_affects() {
        use kuna_base::address::Address;

        let entry = |u: uintm, blk: int4| PcodeOpSetEntry {
            id: OpId::from(KeyData::from_ffi(u64::from(u) + 1)),
            block_index: blk,
            point: op_point(u),
            order: u,
        };
        let mut set = PcodeOpSet::new(Box::new(Vec::new), Box::new(|_, _: &Address| false));
        assert!(!set.is_populated());
        // add out of block/order order; finalize must sort + block-index them.
        set.add_op_entry(entry(50, 2));
        set.add_op_entry(entry(20, 0));
        set.add_op_entry(entry(10, 0));
        // bind a store-guard-style affectsTest: only op #20 (a guarded STORE whose
        // range is [0x10, 0x20]) "affects"; everything else (a CALL) always does.
        let guarded = OpId::from(KeyData::from_ffi(u64::from(20u32) + 1));
        set.set_affects(Box::new(move |op, addr: &Address| {
            if op == guarded {
                let off = addr.get_offset();
                (0x10..=0x20).contains(&off)
            } else {
                true // CALL / unguarded STORE always affects
            }
        }));
        set.finalize_pub();
        assert!(set.is_populated());
        assert_eq!(set.op_list[0].order, 10);
        assert_eq!(set.op_list[1].order, 20);
        assert_eq!(set.op_list[2].block_index, 2);
        assert_eq!(set.block_start, vec![0, 2]);

        // affectsTest on the bound closure: a CALL (op #10) always affects;
        // the guarded STORE (op #20) only affects an address inside its range.
        let call_op = OpId::from(KeyData::from_ffi(u64::from(10u32) + 1));
        let in_range = Address::new(test_ram_space(), 0x18);
        let out_of_range = Address::new(test_ram_space(), 0x40);
        assert!(set.affects_test(call_op, &out_of_range)); // CALL always affects
        assert!(set.affects_test(guarded, &in_range)); // STORE: addr in guard range
        assert!(!set.affects_test(guarded, &out_of_range)); // STORE: addr outside
    }

    /// Minimal RAM-like space for the affectsTest address fixture.
    fn test_ram_space() -> std::rc::Rc<kuna_base::space::AddrSpace> {
        use kuna_base::space::{addrspace_flags, spacetype, AddrSpace};
        std::rc::Rc::new(AddrSpace::new(
            spacetype::IPTR_PROCESSOR,
            "ram",
            false,
            8,
            1,
            2,
            addrspace_flags::hasphysical,
            1,
            1,
        ))
    }
}
