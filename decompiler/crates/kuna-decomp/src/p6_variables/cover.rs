//! Port of `decompiler/cpp/cover.{hh,cc}` (W7, item `w7-s6-variable-cover`) —
//! the topological scope (`Cover`/`CoverBlock`) of a variable within a function
//! and the [`PcodeOpSet`] used for secondary intersection testing.
//!
//! ## ADR 0001 (IR arenas) realization
//!
//! The C++ `CoverBlock` stores `const PcodeOp *start`/`stop` raw pointers, with
//! three sentinel values (`(PcodeOp *)0` = beginning-of-block, `(PcodeOp *)1` =
//! end-of-block, `(PcodeOp *)2` = input).  In the arena model a boundary point is
//! a [`CoverPoint`] enum carrying the three sentinels explicitly plus an `Op`
//! variant.  The `Op` variant caches the two things the C++ reads off the stored
//! pointer — the `getUIndex` comparison value (a `uintm`) and whether the op is a
//! MULTIEQUAL — so [`CoverBlock`] comparison/merge stay pure (no op-arena
//! access).  The single place that *resolves* a `PcodeOp *` into a point is
//! [`CoverContext`], filled by `Funcdata`.
//!
//! ## ADR 0002 (ordered containers) realization
//!
//! `Cover::runs` holds the C++ `map<int4,CoverBlock>` as disjoint runs sorted
//! on block index, so `intersect`/`intersectList`/`merge`/`compareTo` visit
//! block indices in the same ascending order the C++ `std::map` iterator does.
//! A run of adjacent whole blocks (`setAll`) is one entry; any other block is an
//! entry of its own.  A [`CoverBlock`] packs each boundary into a `uindex` and a
//! one-byte kind, 12 bytes for the pair.  A long-lived value's Cover names most
//! blocks of the function, nearly all of them whole, and a function keeps one
//! Cover per Varnode and per HighVariable, so this storage is what variable
//! merging's memory is made of.  Walks that add blocks in predecessor order
//! ([`Cover::rebuild`], [`Cover::add_ref_point_for`]) fill a per-thread scratch
//! table indexed by block and store the touched blocks once, sorted, so a
//! block's insertion never shifts the rest of the Cover.  Every entry the C++
//! map would hold (including an empty block a merge creates) is kept.
//! `PcodeOpSet::opList` is a `Vec` sorted by [`PcodeOpSet::compare_by_block`]
//! (block index, then `SeqNum::order`) exactly as `finalize`'s `std::sort`.

use std::cell::RefCell;

use kuna_base::address::Address;
use kuna_base::types::{int4, uintm};

use crate::context::OpId;

/// A boundary point of a [`CoverBlock`] (the C++ `const PcodeOp *` with its three
/// sentinel encodings).
///
/// The C++ stores `(const PcodeOp *)0/1/2` as special markers and any other
/// value as a real op pointer.  `Op` caches the `getUIndex` value and whether the
/// op is a MULTIEQUAL, the only facts the geometric methods read.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoverPoint {
    /// `(const PcodeOp *)0` — the very beginning of the block.
    Begin,
    /// `(const PcodeOp *)1` — the very end of the block.
    End,
    /// `(const PcodeOp *)2` — an input (special marker), `getUIndex` == 0.
    Input,
    /// A real `PcodeOp *`.
    Op {
        /// Cached `CoverBlock::getUIndex(op)` value.
        uindex: uintm,
        /// `op->code() == CPUI_MULTIEQUAL`.
        multiequal: bool,
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
}

const K_NONE: u8 = 0;
const K_BEGIN: u8 = 1;
const K_END: u8 = 2;
const K_INPUT: u8 = 3;
const K_OP: u8 = 4;
const K_MULTI: u8 = 5;

#[inline(always)]
fn pack(p: Option<CoverPoint>) -> (uintm, u8) {
    match p {
        None => (0, K_NONE),
        Some(CoverPoint::Begin) => (0, K_BEGIN),
        Some(CoverPoint::End) => (uintm::MAX, K_END),
        Some(CoverPoint::Input) => (0, K_INPUT),
        Some(CoverPoint::Op { uindex, multiequal: false }) => (uindex, K_OP),
        Some(CoverPoint::Op { uindex, multiequal: true }) => (uindex, K_MULTI),
    }
}

#[inline(always)]
fn unpack(u: uintm, k: u8) -> Option<CoverPoint> {
    match k {
        K_NONE => None,
        K_BEGIN => Some(CoverPoint::Begin),
        K_END => Some(CoverPoint::End),
        K_INPUT => Some(CoverPoint::Input),
        K_OP => Some(CoverPoint::Op { uindex: u, multiequal: false }),
        _ => Some(CoverPoint::Op { uindex: u, multiequal: true }),
    }
}

/// The topological scope of a variable within a basic block (C++ `class
/// CoverBlock`, `cover.hh:75-96`).
///
/// Each boundary is its `getUIndex` value plus a kind byte: none (the C++
/// `(const PcodeOp *)0`), the three sentinels, an op, or a MULTIEQUAL op.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CoverBlock {
    ustart: uintm,
    ustop: uintm,
    kstart: u8,
    kstop: u8,
}

impl CoverBlock {
    /// Construct an empty/uncovered block (C++ default constructor).
    pub fn new() -> CoverBlock {
        CoverBlock::default()
    }

    /// Get the start of the range (C++ `getStart`).
    pub fn get_start(&self) -> Option<CoverPoint> {
        unpack(self.ustart, self.kstart)
    }
    /// Get the end of the range (C++ `getStop`).
    pub fn get_stop(&self) -> Option<CoverPoint> {
        unpack(self.ustop, self.kstop)
    }
    #[inline(always)]
    fn set_start(&mut self, p: Option<CoverPoint>) {
        (self.ustart, self.kstart) = pack(p);
    }
    #[inline(always)]
    fn set_stop(&mut self, p: Option<CoverPoint>) {
        (self.ustop, self.kstop) = pack(p);
    }
    #[inline(always)]
    fn stop_is_end(&self) -> bool {
        self.kstop == K_END
    }
    /// The whole block (`setAll`'s `start=0; stop=1`).
    #[inline(always)]
    fn is_full(&self) -> bool {
        self.kstart == K_NONE && self.kstop == K_END
    }
    /// Clear \b this block to empty/uncovered (C++ `clear`).
    pub fn clear(&mut self) {
        *self = CoverBlock::default();
    }
    /// Mark whole block as covered (C++ `setAll`): `start=0; stop=1`.
    pub fn set_all(&mut self) {
        self.set_start(None);
        self.set_stop(Some(CoverPoint::End));
    }
    /// Reset start of range (C++ `setBegin`): if `stop==0` set it to `1`.
    pub fn set_begin(&mut self, begin: Option<CoverPoint>) {
        self.set_start(begin);
        if self.kstop == K_NONE {
            self.set_stop(Some(CoverPoint::End));
        }
    }
    /// Reset end of range (C++ `setEnd`).
    pub fn set_end(&mut self, end: Option<CoverPoint>) {
        self.set_stop(end);
    }
    /// Return \b true if \b this is empty/uncovered (C++ `empty`):
    /// `start==0 && stop==0`.
    pub fn empty(&self) -> bool {
        self.kstart == K_NONE && self.kstop == K_NONE
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

        let ustart = self.ustart;
        let ustop = self.ustop;
        let u2start = op2.ustart;
        let u2stop = op2.ustop;
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
        let upoint = pack(point).0;
        let ustart = self.ustart;
        let ustop = self.ustop;

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
        let val = pack(point).0;
        if self.ustart == val && self.kstart != K_NONE {
            return 2;
        }
        if self.ustop == val {
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
            *self = *op2;
            return;
        }
        let ustart = self.ustart;
        let u2start = op2.ustart;
        // Is start contained in op2
        let internal4 = (ustart == 0) && op2.stop_is_end();
        let internal1 = internal4 || op2.contain(self.get_start());
        // Is op2.start contained in this
        let internal3 = (u2start == 0) && self.stop_is_end();
        let internal2 = internal3 || self.contain(op2.get_start());

        if internal1 && internal2 && ((ustart != u2start) || internal3 || internal4) {
            // Covered entire block
            self.set_all();
            return;
        }
        if internal1 {
            self.ustart = op2.ustart; // Pick non-internal start
            self.kstart = op2.kstart;
        } else if !internal2 {
            // Disjoint intervals
            if ustart < u2start {
                // Pick earliest start, then take other stop
                self.ustop = op2.ustop;
                self.kstop = op2.kstop;
            } else {
                self.ustart = op2.ustart;
                self.kstart = op2.kstart;
            }
            return;
        }
        if internal3 || op2.contain(self.get_stop()) {
            // Pick non-internal stop
            self.ustop = op2.ustop;
            self.kstop = op2.kstop;
        }
    }
}

/// One entry of a [`Cover`]: blocks `lo..=hi`, each covered by `cb`.  Only a
/// whole-block CoverBlock spans more than one block.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Run {
    lo: int4,
    hi: int4,
    cb: CoverBlock,
}

/// Append block `bl` (above every block already in `out`) covered by `cb`,
/// extending the last run when both are whole blocks and adjacent.
#[inline(always)]
fn push_block(out: &mut Vec<Run>, bl: int4, cb: CoverBlock) {
    if cb.is_full() {
        if let Some(last) = out.last_mut() {
            if last.cb.is_full() && last.hi.checked_add(1) == Some(bl) {
                last.hi = bl;
                return;
            }
        }
    }
    out.push(Run { lo: bl, hi: bl, cb });
}

/// Append blocks `lo..=hi`, each covered by `cb`.
fn push_span(out: &mut Vec<Run>, lo: int4, hi: int4, cb: CoverBlock) {
    if cb.is_full() {
        push_block(out, lo, cb);
        out.last_mut().expect("push_span: just pushed").hi = hi;
    } else {
        for bl in lo..=hi {
            out.push(Run { lo: bl, hi: bl, cb });
        }
    }
}

/// A per-thread table of CoverBlocks indexed by block, for the walks that add
/// blocks out of order.  A slot belongs to the current walk when its stamp is
/// the walk's generation; `touched` lists those slots, loaded ones first.
#[derive(Default)]
struct Scratch {
    generation: u32,
    stamp: Vec<u32>,
    slot: Vec<CoverBlock>,
    touched: Vec<int4>,
    loaded: usize,
}

impl Scratch {
    fn begin(&mut self) {
        self.generation = self.generation.wrapping_add(1);
        if self.generation == 0 {
            self.stamp.iter_mut().for_each(|s| *s = 0);
            self.generation = 1;
        }
        self.touched.clear();
        self.loaded = 0;
    }

    fn load(&mut self, runs: &[Run]) {
        for r in runs {
            for bl in r.lo..=r.hi {
                *self.entry(bl) = r.cb;
            }
        }
        self.loaded = self.touched.len();
    }

    #[inline(always)]
    fn entry(&mut self, bl: int4) -> &mut CoverBlock {
        let i = usize::try_from(bl).expect("Cover: negative block index");
        if i >= self.stamp.len() {
            let n = (i + 1).max(self.stamp.len() * 2).max(64);
            self.stamp.resize(n, 0);
            self.slot.resize(n, CoverBlock::default());
        }
        if self.stamp[i] != self.generation {
            self.stamp[i] = self.generation;
            self.slot[i] = CoverBlock::default();
            self.touched.push(bl);
        }
        &mut self.slot[i]
    }

    /// The touched blocks in ascending order, as runs.
    fn store(&mut self) -> Vec<Run> {
        let (old, new) = self.touched.split_at_mut(self.loaded);
        new.sort_unstable();
        let mut out = Vec::new();
        let (mut i, mut j) = (0, 0);
        while i < old.len() || j < new.len() {
            let bl = if j >= new.len() || (i < old.len() && old[i] < new[j]) {
                i += 1;
                old[i - 1]
            } else {
                j += 1;
                new[j - 1]
            };
            push_block(&mut out, bl, self.slot[bl as usize]);
        }
        out.shrink_to_fit();
        out
    }
}

thread_local! {
    static SCRATCH: RefCell<Scratch> = RefCell::new(Scratch::default());
}

/// Run `f` on a scratch table holding `runs` and return the table's blocks as
/// runs.  A nested call gets a table of its own.
fn with_scratch<R>(runs: &[Run], f: impl FnOnce(&mut Scratch) -> R) -> (Vec<Run>, R) {
    let run = |s: &mut Scratch| {
        s.begin();
        s.load(runs);
        let r = f(s);
        (s.store(), r)
    };
    SCRATCH.with(|cell| match cell.try_borrow_mut() {
        Ok(mut s) => run(&mut s),
        Err(_) => run(&mut Scratch::default()),
    })
}

/// The topological scope of a single variable object (C++ `class Cover`,
/// `cover.hh:108-131`).
#[derive(Debug, Clone, Default)]
pub struct Cover {
    /// The C++ `map<int4,CoverBlock> cover` as disjoint runs sorted on block
    /// index; adjacent whole blocks share one run.
    runs: Vec<Run>,
}

/// Global empty CoverBlock for blocks not covered (C++ `Cover::emptyBlock`).
const EMPTY_BLOCK: CoverBlock = CoverBlock { ustart: 0, ustop: 0, kstart: K_NONE, kstop: K_NONE };

impl Cover {
    /// Construct an empty Cover.
    pub fn new() -> Cover {
        Cover { runs: Vec::new() }
    }

    /// Clear \b this to an empty Cover (C++ `clear`).
    pub fn clear(&mut self) {
        self.runs = Vec::new();
    }

    #[inline]
    fn find(&self, bl: int4) -> Option<&CoverBlock> {
        let i = self.runs.partition_point(|r| r.hi < bl);
        match self.runs.get(i) {
            Some(r) if r.lo <= bl => Some(&r.cb),
            _ => None,
        }
    }

    /// Iterate the `(block_index, CoverBlock)` entries in ascending block order
    /// (C++ `begin`/`end`).
    pub fn iter(&self) -> impl Iterator<Item = (int4, &CoverBlock)> {
        self.runs.iter().flat_map(|r| (r.lo..=r.hi).map(move |bl| (bl, &r.cb)))
    }

    /// Give ordering of \b this and another Cover (C++ `Cover::compareTo`,
    /// `cover.cc:223-247`): compares the index of the first partly-covered block.
    pub fn compare_to(&self, op2: &Cover) -> int4 {
        let a = match self.runs.first() {
            None => 1000000,
            Some(r) => r.lo,
        };
        let b = match op2.runs.first() {
            None => 1000000,
            Some(r) => r.lo,
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
        self.find(i).unwrap_or(&EMPTY_BLOCK)
    }

    /// Visit, in ascending order, each stretch of blocks both covers hold with
    /// the two CoverBlocks covering it; stop when `f` returns true.
    fn overlaps(&self, op2: &Cover, mut f: impl FnMut(int4, int4, &CoverBlock, &CoverBlock) -> bool) {
        let (a, b) = (&self.runs, &op2.runs);
        let (mut i, mut j) = (0, 0);
        while i < a.len() && j < b.len() {
            let (ra, rb) = (&a[i], &b[j]);
            if ra.hi < rb.lo {
                i += 1;
            } else if rb.hi < ra.lo {
                j += 1;
            } else {
                if f(ra.lo.max(rb.lo), ra.hi.min(rb.hi), &ra.cb, &rb.cb) {
                    return;
                }
                if ra.hi <= rb.hi {
                    i += 1;
                }
                if rb.hi <= ra.hi {
                    j += 1;
                }
            }
        }
    }

    /// Characterize the intersection between \b this and another Cover (C++
    /// `Cover::intersect`, `cover.cc:269-297`): 0/1/2 as `CoverBlock::intersect`.
    pub fn intersect(&self, op2: &Cover) -> int4 {
        let mut res = 0;
        self.overlaps(op2, |_, _, a, b| {
            let newres = a.intersect(b);
            if newres == 1 {
                res = 1; // At least a point intersection
            }
            if newres == 2 {
                res = 2;
            }
            res == 2
        });
        res
    }

    /// Generate the list of blocks that intersect above `level` (C++
    /// `Cover::intersectList`, `cover.cc:307-334`).
    pub fn intersect_list(&self, listout: &mut Vec<int4>, op2: &Cover, level: int4) {
        listout.clear();
        self.overlaps(op2, |lo, hi, a, b| {
            if a.intersect(b) >= level {
                listout.extend(lo..=hi);
            }
            false
        });
    }

    /// Characterize the intersection on a specific block (C++
    /// `Cover::intersectByBlock`, `cover.cc:392-406`).
    pub fn intersect_by_block(&self, blk: int4, op2: &Cover) -> int4 {
        let iter = match self.find(blk) {
            None => return 0,
            Some(cb) => cb,
        };
        let iter2 = match op2.find(blk) {
            None => return 0,
            Some(cb) => cb,
        };
        iter.intersect(iter2)
    }

    /// Merge \b this with another Cover block by block (C++ `Cover::merge`,
    /// `cover.cc:465-472`).  A block only in `op2` is added the way the C++
    /// `cover[k].merge(...)` default-constructs it.
    pub fn merge(&mut self, op2: &Cover) {
        if op2.runs.is_empty() {
            return;
        }
        let (a, b) = (&self.runs, &op2.runs);
        let only_b = |cb: &CoverBlock| {
            let mut m = CoverBlock::default();
            m.merge(cb);
            m
        };
        let mut out = Vec::with_capacity(a.len() + b.len());
        let (mut i, mut j) = (0, 0);
        let mut ca = a.first().map_or(0, |r| r.lo);
        let mut cb = b[0].lo;
        loop {
            match (a.get(i), b.get(j)) {
                (None, None) => break,
                (Some(ra), None) => {
                    push_span(&mut out, ca, ra.hi, ra.cb);
                    i += 1;
                    ca = a.get(i).map_or(0, |r| r.lo);
                }
                (None, Some(rb)) => {
                    push_span(&mut out, cb, rb.hi, only_b(&rb.cb));
                    j += 1;
                    cb = b.get(j).map_or(0, |r| r.lo);
                }
                (Some(ra), Some(rb)) => {
                    if ra.hi < cb {
                        push_span(&mut out, ca, ra.hi, ra.cb);
                        i += 1;
                        ca = a.get(i).map_or(0, |r| r.lo);
                    } else if rb.hi < ca {
                        push_span(&mut out, cb, rb.hi, only_b(&rb.cb));
                        j += 1;
                        cb = b.get(j).map_or(0, |r| r.lo);
                    } else if ca < cb {
                        push_span(&mut out, ca, cb - 1, ra.cb);
                        ca = cb;
                    } else if cb < ca {
                        push_span(&mut out, cb, ca - 1, only_b(&rb.cb));
                        cb = ca;
                    } else {
                        let end = ra.hi.min(rb.hi);
                        let mut m = ra.cb;
                        m.merge(&rb.cb);
                        push_span(&mut out, ca, end, m);
                        if ra.hi == end {
                            i += 1;
                            ca = a.get(i).map_or(0, |r| r.lo);
                        } else {
                            ca = end + 1;
                        }
                        if rb.hi == end {
                            j += 1;
                            cb = b.get(j).map_or(0, |r| r.lo);
                        } else {
                            cb = end + 1;
                        }
                    }
                }
            }
        }
        out.shrink_to_fit();
        self.runs = out;
    }

    /// Reset to the single point where the given Varnode is defined (C++
    /// `Cover::addDefPoint`, `cover.cc:501-519`).
    ///
    /// `def` is the `(block_index, CoverPoint)` of the defining op (resolved by
    /// the caller from `vn->getDef()->getParent()->getIndex()`), or `None` when
    /// the Varnode is an input (the C++ `vn->isInput()` branch), in which case
    /// `is_input` distinguishes the input case from "no cover".
    pub fn add_def_point(&mut self, def: Option<(int4, CoverPoint)>, is_input: bool) {
        self.clear();
        if let Some((blk, point)) = def {
            self.runs.push(Run { lo: blk, hi: blk, cb: Self::def_block(point) });
        } else if is_input {
            // Special mark for input
            self.runs.push(Run { lo: 0, hi: 0, cb: Self::def_block(CoverPoint::Input) });
        }
    }

    fn def_block(point: CoverPoint) -> CoverBlock {
        let mut block = CoverBlock::default();
        block.set_begin(Some(point)); // Set the point topology
        block.set_end(Some(point));
        block
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
        self.add_ref_points_for(ctx, &[ref_op], vn);
    }

    /// (kuna) [`Self::add_ref_point_for`] for each of `ref_ops` in turn.
    pub fn add_ref_points_for(&mut self, ctx: &dyn CoverContext, ref_ops: &[OpId], vn: crate::context::VarnodeId) {
        if ref_ops.is_empty() {
            return;
        }
        let (runs, ()) = with_scratch(&self.runs, |s| {
            for &ref_op in ref_ops {
                let (bl, ref_point, is_multiequal, pred_blocks) = ctx.ref_point(ref_op, vn);
                Self::add_ref_point(s, ctx, bl, ref_point, is_multiequal, &pred_blocks, &mut |_, _| false);
            }
        });
        self.runs = runs;
    }

    /// (kuna) Does `hit` hold for some block of \b this?
    pub fn any_block(&self, mut hit: impl FnMut(int4, &CoverBlock) -> bool) -> bool {
        self.iter().any(|(bl, cb)| hit(bl, cb))
    }

    /// (kuna) [`Self::add_ref_point_for`] that stops at the first block of
    /// \b this, already present or just set or extended by the walk, for which
    /// `hit` holds, and says whether it stopped.  A block only grows during the
    /// walk, so for a `hit` that holds on every block containing one it holds
    /// on, this stops exactly when `hit` holds for a block of the finished
    /// Cover.
    pub fn add_ref_point_until(
        &mut self,
        ctx: &dyn CoverContext,
        ref_op: OpId,
        vn: crate::context::VarnodeId,
        mut hit: impl FnMut(int4, &CoverBlock) -> bool,
    ) -> bool {
        if self.any_block(&mut hit) {
            return true;
        }
        let (bl, ref_point, is_multiequal, pred_blocks) = ctx.ref_point(ref_op, vn);
        let (runs, stopped) = with_scratch(&self.runs, |s| {
            Self::add_ref_point(s, ctx, bl, ref_point, is_multiequal, &pred_blocks, &mut hit)
        });
        self.runs = runs;
        stopped
    }

    /// Add to the Cover recursively, starting at the bottom of the given
    /// block and filling backward until existing cover is hit (C++
    /// `Cover::addRefRecurse`, `cover.cc:524-558`).
    ///
    /// (kuna) Stops, returning true, once `hit` holds for a block it has just
    /// set or extended (see [`Self::add_ref_point_until`]).
    fn add_ref_recurse<F: FnMut(int4, &CoverBlock) -> bool>(
        s: &mut Scratch,
        ctx: &dyn CoverContext,
        bl: int4,
        hit: &mut F,
    ) -> bool {
        let block = s.entry(bl);
        if block.empty() {
            block.set_all(); // No cover encountered, fill in entire block
            if hit(bl, block) {
                return true;
            }
            let n = ctx.size_in(bl);
            for j in 0..n {
                // Recurse to predecessors
                if Self::add_ref_recurse(s, ctx, ctx.get_in(bl, j), hit) {
                    return true;
                }
            }
        } else {
            let stop_multiequal = block.kstop == K_MULTI;
            let ustart = block.ustart;
            let ustop = block.ustop;
            if (ustop != uintm::MAX) && (ustop >= ustart) {
                block.set_end(Some(CoverPoint::End)); // Fill in to the bottom
                if hit(bl, block) {
                    return true;
                }
            }

            // getStart() is unchanged by the fill above
            let start_is_zero = block.kstart == K_NONE;
            if (ustop == 0) && start_is_zero && stop_multiequal {
                // Block contains only an infinitesimal tip of cover
                // through one branch of a MULTIEQUAL; still traverse.
                let n = ctx.size_in(bl);
                for j in 0..n {
                    if Self::add_ref_recurse(s, ctx, ctx.get_in(bl, j), hit) {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// Add a variable read to the Cover (C++ `Cover::addRefPoint`,
    /// `cover.cc:565-612`).
    ///
    /// `ref_op` is `(block_index, CoverPoint, code)` for the reading PcodeOp.
    /// `ref_inputs`, when the read is a MULTIEQUAL, lists the input slots whose
    /// Varnode equals `vn` (the C++ `ref->getIn(j)==vn` loop), as predecessor
    /// block indices to recurse into.  (kuna) Stops, returning true, once `hit`
    /// holds for a block it has just set or extended.
    fn add_ref_point<F: FnMut(int4, &CoverBlock) -> bool>(
        s: &mut Scratch,
        ctx: &dyn CoverContext,
        bl: int4,
        ref_point: CoverPoint,
        is_multiequal: bool,
        multiequal_pred_blocks: &[int4],
        hit: &mut F,
    ) -> bool {
        let mut recurse_all = false;
        {
            let block = s.entry(bl);
            if block.empty() {
                block.set_end(Some(ref_point));
                if hit(bl, block) {
                    return true;
                }
            } else if block.contain(Some(ref_point)) {
                if !is_multiequal {
                    return false;
                }
                // Even if MULTIEQUAL ref is contained we may be adding new
                // cover because we are looking at a different branch.
            } else {
                let old_stop_multiequal = block.kstop == K_MULTI;
                let start_is_none = block.kstart == K_NONE;
                let ustart = block.ustart;
                block.set_end(Some(ref_point)); // Otherwise update endpoint
                if hit(bl, block) {
                    return true;
                }
                if block.ustop >= ustart {
                    if old_stop_multiequal && start_is_none {
                        recurse_all = true; // Traverse predecessors below
                    }
                    if !recurse_all {
                        return false;
                    }
                }
            }
        }

        if recurse_all {
            let n = ctx.size_in(bl);
            for j in 0..n {
                if Self::add_ref_recurse(s, ctx, ctx.get_in(bl, j), hit) {
                    return true;
                }
            }
            return false;
        }

        if is_multiequal {
            for &pred in multiequal_pred_blocks {
                if Self::add_ref_recurse(s, ctx, pred, hit) {
                    return true;
                }
            }
        } else {
            let n = ctx.size_in(bl);
            for j in 0..n {
                if Self::add_ref_recurse(s, ctx, ctx.get_in(bl, j), hit) {
                    return true;
                }
            }
        }
        false
    }

    /// Reset \b this based on def-use of a single Varnode (C++ `Cover::rebuild`,
    /// `cover.cc:477-496`).
    ///
    /// The def/use graph walk is driven by [`CoverContext`]; the implied-output
    /// chase (`outVn->isImplied()`) keeps the exact `path` BFS order of the C++.
    pub fn rebuild(&mut self, ctx: &dyn CoverContext, vn: crate::context::VarnodeId) {
        // addDefPoint(vn)
        let (def, is_input) = ctx.def_point(vn);
        self.add_def_point(def, is_input);

        let (runs, ()) = with_scratch(&self.runs, |s| {
            let mut path = vec![vn];
            let mut pos = 0usize;
            while pos < path.len() {
                let cur_vn = path[pos];
                pos += 1;
                for op in ctx.descend(cur_vn) {
                    let (bl, ref_point, is_multiequal, pred_blocks) = ctx.ref_point(op, vn);
                    let never = &mut |_, _: &CoverBlock| false;
                    Self::add_ref_point(s, ctx, bl, ref_point, is_multiequal, &pred_blocks, never);
                    if let Some(out_vn) = ctx.out_implied(op) {
                        path.push(out_vn);
                    }
                }
            }
        });
        self.runs = runs;
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
        let cb = match self.find(blk) {
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
        let cb = match self.find(op_blk) {
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
    /// (kuna) Upstream's walk keeps the index of the block it just tested, so
    /// it skips the set's next block after every block it shares with the
    /// cover. A second walk that reads each block's index tests every guarded
    /// STORE; CALLs keep upstream's walk.
    pub fn intersect_op_set(&self, op_set: &PcodeOpSet, rep_addr: &Address) -> bool {
        self.walk_op_set(op_set, rep_addr, false)
            || (op_set.op_list.iter().any(|e| e.store) && self.walk_op_set(op_set, rep_addr, true))
    }

    /// The block walk of [`Cover::intersect_op_set`]; `stores` re-reads the
    /// set's block index after a shared block and tests only STORE entries.
    fn walk_op_set(&self, op_set: &PcodeOpSet, rep_addr: &Address, stores: bool) -> bool {
        if op_set.op_list.is_empty() {
            return false;
        }
        let mut set_block = 0usize;
        let mut op_index = op_set.block_start[set_block] as usize;
        let mut set_index = op_set.op_list[op_index].block_index;
        let first_block = op_set.op_list[0].block_index;
        let from = self.runs.partition_point(|r| r.hi < first_block);
        let mut cover_iter = self.runs[from..]
            .iter()
            .flat_map(|r| (r.lo.max(first_block)..=r.hi).map(move |bl| (bl, r.cb)))
            .peekable();
        while let Some(&(cover_index, cover_block)) = cover_iter.peek() {
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
                cover_iter.next();
                let mut op_max = op_set.op_list.len();
                set_block += 1;
                if set_block < op_set.block_start.len() {
                    op_max = op_set.block_start[set_block] as usize;
                }
                loop {
                    let entry = &op_set.op_list[op_index];
                    if (!stores || entry.store) && cover_block.contain(Some(entry.point)) {
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
                if stores {
                    set_index = op_set.op_list[op_index].block_index;
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
    /// (kuna) A guarded stack STORE the second walk of
    /// [`Cover::intersect_op_set`] tests (under `stackstoreguard`).
    pub store: bool,
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
        CoverPoint::Op { uindex, multiequal: false }
    }

    fn meq_point(uindex: uintm) -> CoverPoint {
        CoverPoint::Op { uindex, multiequal: true }
    }

    fn put(c: &mut Cover, bl: int4, cb: CoverBlock) {
        let mut all: std::collections::BTreeMap<int4, CoverBlock> = c.iter().map(|(k, v)| (k, *v)).collect();
        all.insert(bl, cb);
        c.runs.clear();
        for (k, v) in all {
            push_block(&mut c.runs, k, v);
        }
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
        put(&mut a, 2, {
            let mut cb = CoverBlock::new();
            cb.set_all();
            cb
        });
        let mut b = Cover::new();
        put(&mut b, 5, {
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
        put(&mut a, 1, acb);
        let mut b = Cover::new();
        let mut bcb = CoverBlock::new();
        bcb.set_begin(Some(op_point(10)));
        bcb.set_end(Some(op_point(30)));
        put(&mut b, 1, bcb);
        assert_eq!(a.intersect(&b), 2);
        assert_eq!(a.intersect_by_block(1, &b), 2);
        // disjoint blocks -> no intersection
        let mut c = Cover::new();
        let mut ccb = CoverBlock::new();
        ccb.set_all();
        put(&mut c, 7, ccb);
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
        put(&mut a, 1, a1);
        let mut b1 = CoverBlock::new();
        b1.set_begin(Some(op_point(10)));
        b1.set_end(Some(op_point(10)));
        put(&mut b, 1, b1);
        // block 2: interval intersect (2)
        let mut a2 = CoverBlock::new();
        a2.set_begin(Some(op_point(5)));
        a2.set_end(Some(op_point(20)));
        put(&mut a, 2, a2);
        let mut b2 = CoverBlock::new();
        b2.set_begin(Some(op_point(10)));
        b2.set_end(Some(op_point(30)));
        put(&mut b, 2, b2);

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
        put(&mut a, 1, a1);

        let mut b = Cover::new();
        let mut b1 = CoverBlock::new();
        b1.set_begin(Some(op_point(20)));
        b1.set_end(Some(op_point(25)));
        put(&mut b, 1, b1);
        let mut b2 = CoverBlock::new();
        b2.set_all();
        put(&mut b, 2, b2);

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
        put(&mut c, 2, cb);
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

    /// `add_ref_point_until` stops exactly when the finished `add_ref_point_for`
    /// Cover contains a point of the set, and builds that Cover when nothing
    /// stops it.
    #[test]
    fn add_ref_point_until_agrees_with_the_full_cover() {
        let vn = crate::context::VarnodeId::from(KeyData::from_ffi(1));
        let reader = OpId::from(KeyData::from_ffi(100));
        let ctxs: [&dyn CoverContext; 2] = [&DiamondCtx, &LoopCtx];
        for ctx in ctxs {
            let start = || {
                let mut c = Cover::new();
                let (def, is_input) = ctx.def_point(vn);
                c.add_def_point(def, is_input);
                c
            };
            let mut full = start();
            full.add_ref_point_for(ctx, reader, vn);
            let mut walked = start();
            assert!(!walked.add_ref_point_until(ctx, reader, vn, |_, _| false));
            for bl in 0..4 {
                assert_eq!(walked.get_cover_block(bl), full.get_cover_block(bl));
            }
            let points: Vec<(int4, CoverPoint)> = (0..4)
                .flat_map(|bl| {
                    (0..12).map(move |k| (bl, op_point(k))).chain([(bl, meq_point(0))])
                })
                .collect();
            for point in &points {
                let want = full.contain(point.0, point.1, 1);
                let got = start().add_ref_point_until(ctx, reader, vn, |bl, cb| {
                    bl == point.0 && cb.contain(Some(point.1))
                });
                assert_eq!(got, want, "{point:?}");
            }
        }
    }

    /// A straight line of blocks 0 -> 1 -> ... -> 99.
    struct ChainCtx;
    impl CoverContext for ChainCtx {
        fn size_in(&self, bl: int4) -> int4 {
            if bl == 0 { 0 } else { 1 }
        }
        fn get_in(&self, bl: int4, _j: int4) -> int4 {
            bl - 1
        }
        fn def_point(&self, _vn: crate::context::VarnodeId) -> (Option<(int4, CoverPoint)>, bool) {
            (Some((0, op_point(5))), false)
        }
        fn descend(&self, _vn: crate::context::VarnodeId) -> Vec<OpId> {
            vec![OpId::from(KeyData::from_ffi(100))]
        }
        fn ref_point(&self, _op: OpId, _vn: crate::context::VarnodeId) -> (int4, CoverPoint, bool, Vec<int4>) {
            (99, op_point(3), false, vec![])
        }
        fn out_implied(&self, _op: OpId) -> Option<crate::context::VarnodeId> {
            None
        }
    }

    /// A value defined in block 0 and read in block 99 covers the 98 blocks
    /// between as one run, and still reads back block by block.
    #[test]
    fn whole_blocks_between_def_and_read_share_one_run() {
        let mut c = Cover::new();
        c.rebuild(&ChainCtx, crate::context::VarnodeId::from(KeyData::from_ffi(1)));
        assert_eq!(c.runs.len(), 3);
        assert_eq!(c.iter().count(), 100);
        assert_eq!(c.get_cover_block(0).get_start(), Some(op_point(5)));
        assert_eq!(c.get_cover_block(0).get_stop(), Some(CoverPoint::End));
        let mut whole = CoverBlock::new();
        whole.set_all();
        assert!((1..99).all(|bl| *c.get_cover_block(bl) == whole));
        assert_eq!(c.get_cover_block(99).get_start(), None);
        assert_eq!(c.get_cover_block(99).get_stop(), Some(op_point(3)));
        assert!(c.get_cover_block(100).empty());
    }

    struct Rng(u64);
    impl Rng {
        fn below(&mut self, n: u64) -> u64 {
            self.0 ^= self.0 << 13;
            self.0 ^= self.0 >> 7;
            self.0 ^= self.0 << 17;
            self.0 % n
        }
    }

    fn random_block(r: &mut Rng) -> CoverBlock {
        let mut cb = CoverBlock::new();
        if r.below(3) == 0 {
            cb.set_all();
            return cb;
        }
        let mut pt = |r: &mut Rng| match r.below(6) {
            0 => None,
            1 => Some(CoverPoint::Begin),
            2 => Some(CoverPoint::End),
            3 => Some(CoverPoint::Input),
            4 => Some(op_point(r.below(8) as uintm)),
            _ => Some(meq_point(0)),
        };
        let (start, stop) = (pt(r), pt(r));
        cb.set_start(start);
        cb.set_stop(stop);
        cb
    }

    fn random_cover(r: &mut Rng) -> (Cover, std::collections::BTreeMap<int4, CoverBlock>) {
        let mut model = std::collections::BTreeMap::new();
        for _ in 0..r.below(40) {
            model.insert(r.below(48) as int4, random_block(r));
        }
        if r.below(2) == 0 {
            let lo = r.below(40) as int4;
            for bl in lo..lo + r.below(12) as int4 {
                let mut cb = CoverBlock::new();
                cb.set_all();
                model.insert(bl, cb);
            }
        }
        let mut c = Cover::new();
        for (&k, &v) in &model {
            push_block(&mut c.runs, k, v);
        }
        (c, model)
    }

    /// Runs answer intersect/intersectList and merge exactly as a map of one
    /// CoverBlock per block does, and adjacent whole blocks always share a run.
    #[test]
    fn runs_merge_and_intersect_as_a_block_map_does() {
        let mut r = Rng(0x9e37_79b9_7f4a_7c15);
        for _ in 0..4000 {
            let (mut a, mut ma) = random_cover(&mut r);
            let (b, mb) = random_cover(&mut r);
            let mut want = 0;
            for (k, cb) in &ma {
                if let Some(cb2) = mb.get(k) {
                    match cb.intersect(cb2) {
                        2 => {
                            want = 2;
                            break;
                        }
                        1 => want = 1,
                        _ => {}
                    }
                }
            }
            assert_eq!(a.intersect(&b), want);
            for level in 1..=2 {
                let mut got = Vec::new();
                a.intersect_list(&mut got, &b, level);
                let want: Vec<int4> = ma
                    .iter()
                    .filter(|(k, cb)| mb.get(k).is_some_and(|cb2| cb.intersect(cb2) >= level))
                    .map(|(k, _)| *k)
                    .collect();
                assert_eq!(got, want);
            }
            for (k, cb) in &mb {
                ma.entry(*k).or_default().merge(cb);
            }
            a.merge(&b);
            let got: Vec<(int4, CoverBlock)> = a.iter().map(|(k, v)| (k, *v)).collect();
            assert_eq!(got, ma.into_iter().collect::<Vec<_>>());
            assert!(a.runs.windows(2).all(|w| w[0].hi < w[1].lo
                && !(w[0].cb.is_full() && w[1].cb.is_full() && w[0].hi + 1 == w[1].lo)));
        }
    }

    /// The CoverBlock as upstream stores it: two optional points.
    #[derive(Clone, Copy, Debug, PartialEq)]
    struct PtBlock {
        start: Option<CoverPoint>,
        stop: Option<CoverPoint>,
    }

    fn uix(p: Option<CoverPoint>) -> uintm {
        p.map_or(0, CoverPoint::get_uindex)
    }

    impl PtBlock {
        fn empty(&self) -> bool {
            self.start.is_none() && self.stop.is_none()
        }
        fn intersect(&self, o: &PtBlock) -> int4 {
            if self.empty() || o.empty() {
                return 0;
            }
            let (s, e, s2, e2) = (uix(self.start), uix(self.stop), uix(o.start), uix(o.stop));
            let touch = |hit: bool| if hit { 1 } else { 0 };
            if s <= e {
                if s2 <= e2 {
                    if e <= s2 || e2 <= s {
                        return touch(s == e2 || e == s2);
                    }
                } else if s >= e2 && e <= s2 {
                    return touch(s == e2 || e == s2);
                }
            } else if s2 <= e2 && s2 >= e && e2 <= s {
                return touch(s2 == e || e2 == s);
            }
            2
        }
        fn contain(&self, p: Option<CoverPoint>) -> bool {
            if self.empty() {
                return false;
            }
            let (u, s, e) = (uix(p), uix(self.start), uix(self.stop));
            if s <= e {
                return u >= s && u <= e;
            }
            u <= e || u >= s
        }
        fn boundary(&self, p: Option<CoverPoint>) -> int4 {
            if self.empty() {
                return 0;
            }
            if uix(self.start) == uix(p) && self.start.is_some() {
                return 2;
            }
            if uix(self.stop) == uix(p) {
                return 1;
            }
            0
        }
        fn merge(&mut self, o: &PtBlock) {
            if o.empty() {
                return;
            }
            if self.empty() {
                *self = *o;
                return;
            }
            let (s, s2) = (uix(self.start), uix(o.start));
            let i4 = s == 0 && o.stop == Some(CoverPoint::End);
            let i1 = i4 || o.contain(self.start);
            let i3 = s2 == 0 && self.stop == Some(CoverPoint::End);
            let i2 = i3 || self.contain(o.start);
            if i1 && i2 && (s != s2 || i3 || i4) {
                *self = PtBlock { start: None, stop: Some(CoverPoint::End) };
                return;
            }
            if i1 {
                self.start = o.start;
            } else if !i2 {
                if s < s2 {
                    self.stop = o.stop;
                } else {
                    self.start = o.start;
                }
                return;
            }
            if i3 || o.contain(self.stop) {
                self.stop = o.stop;
            }
        }
        fn packed(&self) -> CoverBlock {
            let mut cb = CoverBlock::new();
            cb.set_start(self.start);
            cb.set_stop(self.stop);
            cb
        }
    }

    fn point_alphabet() -> Vec<Option<CoverPoint>> {
        let mut pts = vec![None, Some(CoverPoint::Begin), Some(CoverPoint::End), Some(CoverPoint::Input)];
        for u in [0, 1, 7, uintm::MAX - 1, uintm::MAX] {
            pts.push(Some(op_point(u)));
            pts.push(Some(meq_point(u)));
        }
        pts
    }

    /// A packed CoverBlock answers every block query and merge exactly as the
    /// two-point block does, over every pair of points, and is 12 bytes.
    #[test]
    fn packed_blocks_answer_as_two_point_blocks_do() {
        assert_eq!(std::mem::size_of::<CoverBlock>(), 12);
        assert_eq!(std::mem::size_of::<Run>(), 20);
        let pts = point_alphabet();
        let blocks: Vec<PtBlock> =
            pts.iter().flat_map(|&start| pts.iter().map(move |&stop| PtBlock { start, stop })).collect();
        for a in &blocks {
            let pa = a.packed();
            assert_eq!((pa.get_start(), pa.get_stop()), (a.start, a.stop));
            assert_eq!(pa.empty(), a.empty());
            for &p in &pts {
                assert_eq!(pa.contain(p), a.contain(p), "{a:?} {p:?}");
                assert_eq!(pa.boundary(p), a.boundary(p), "{a:?} {p:?}");
            }
            for b in &blocks {
                let pb = b.packed();
                assert_eq!(pa.intersect(&pb), a.intersect(b), "{a:?} {b:?}");
                let (mut m, mut pm) = (*a, pa);
                m.merge(b);
                pm.merge(&pb);
                assert_eq!(pm, m.packed(), "{a:?} {b:?}");
            }
        }
    }

    /// Once its generation wraps, the scratch table holds none of the blocks
    /// an earlier walk left in it.
    #[test]
    fn scratch_forgets_old_blocks_when_its_generation_wraps() {
        let mut s = Scratch::default();
        s.begin();
        s.entry(3).set_all();
        s.generation = u32::MAX;
        s.begin();
        assert_eq!(s.generation, 1);
        assert!(s.entry(3).empty());
        assert_eq!(s.touched, vec![3]);
    }

    /// A walk started inside another walk gets a table of its own, and both
    /// store exactly their own blocks.
    #[test]
    fn a_walk_inside_a_walk_gets_its_own_table() {
        let mut full = CoverBlock::new();
        full.set_all();
        let mut tip = CoverBlock::new();
        tip.set_end(Some(op_point(4)));
        let (outer, inner) = with_scratch(&[Run { lo: 0, hi: 3, cb: full }], |s| {
            s.entry(5).set_all();
            with_scratch(&[Run { lo: 7, hi: 7, cb: tip }], |t| *t.entry(8) = full).0
        });
        assert_eq!(outer, vec![Run { lo: 0, hi: 3, cb: full }, Run { lo: 5, hi: 5, cb: full }]);
        assert_eq!(inner, vec![Run { lo: 7, hi: 7, cb: tip }, Run { lo: 8, hi: 8, cb: full }]);
    }

    /// The Cover as a map of one CoverBlock per block, with upstream's walks.
    mod model {
        use super::*;
        use std::collections::BTreeMap;

        pub(super) type Map = BTreeMap<int4, CoverBlock>;
        pub(super) type Hit<'a> = &'a mut dyn FnMut(int4, &CoverBlock) -> bool;

        fn stop_is_multiequal(cb: &CoverBlock) -> bool {
            matches!(cb.get_stop(), Some(CoverPoint::Op { multiequal: true, .. }))
        }

        fn recurse(m: &mut Map, ctx: &dyn CoverContext, bl: int4, hit: Hit) -> bool {
            let block = m.entry(bl).or_default();
            let mut preds = false;
            if block.empty() {
                block.set_all();
                if hit(bl, block) {
                    return true;
                }
                preds = true;
            } else {
                let meq = stop_is_multiequal(block);
                let (ustart, ustop) = (uix(block.get_start()), uix(block.get_stop()));
                if ustop != uintm::MAX && ustop >= ustart {
                    block.set_end(Some(CoverPoint::End));
                    if hit(bl, block) {
                        return true;
                    }
                }
                if ustop == 0 && block.get_start().is_none() && meq {
                    preds = true;
                }
            }
            preds && (0..ctx.size_in(bl)).any(|j| recurse(m, ctx, ctx.get_in(bl, j), hit))
        }

        pub(super) fn add_ref(
            m: &mut Map,
            ctx: &dyn CoverContext,
            op: OpId,
            vn: crate::context::VarnodeId,
            hit: Hit,
        ) -> bool {
            let (bl, point, is_meq, listed) = ctx.ref_point(op, vn);
            let mut all = false;
            {
                let block = m.entry(bl).or_default();
                if block.empty() {
                    block.set_end(Some(point));
                    if hit(bl, block) {
                        return true;
                    }
                } else if block.contain(Some(point)) {
                    if !is_meq {
                        return false;
                    }
                } else {
                    let meq = stop_is_multiequal(block);
                    let start = block.get_start();
                    block.set_end(Some(point));
                    if hit(bl, block) {
                        return true;
                    }
                    if point.get_uindex() >= uix(start) {
                        all = meq && start.is_none();
                        if !all {
                            return false;
                        }
                    }
                }
            }
            let preds: Vec<int4> =
                if all || !is_meq { (0..ctx.size_in(bl)).map(|j| ctx.get_in(bl, j)).collect() } else { listed };
            preds.into_iter().any(|p| recurse(m, ctx, p, hit))
        }

        pub(super) fn rebuild(ctx: &dyn CoverContext, vn: crate::context::VarnodeId) -> Map {
            let mut m = Map::new();
            let at = match ctx.def_point(vn) {
                (Some(def), _) => Some(def),
                (None, true) => Some((0, CoverPoint::Input)),
                (None, false) => None,
            };
            if let Some((blk, point)) = at {
                let cb = m.entry(blk).or_default();
                cb.set_begin(Some(point));
                cb.set_end(Some(point));
            }
            let mut path = vec![vn];
            let mut pos = 0;
            while pos < path.len() {
                let cur = path[pos];
                pos += 1;
                for op in ctx.descend(cur) {
                    add_ref(&mut m, ctx, op, vn, &mut |_, _| false);
                    path.extend(ctx.out_implied(op));
                }
            }
            m
        }

        pub(super) fn walk_op_set(m: &Map, set: &PcodeOpSet, addr: &Address, stores: bool) -> bool {
            if set.op_list.is_empty() {
                return false;
            }
            let mut set_block = 0usize;
            let mut op_index = set.block_start[set_block] as usize;
            let mut set_index = set.op_list[op_index].block_index;
            let mut cover_iter = m.range(set.op_list[0].block_index..).peekable();
            while let Some((&cover_index, &cover_block)) = cover_iter.peek().copied() {
                if cover_index < set_index {
                    cover_iter.next();
                } else if cover_index > set_index {
                    set_block += 1;
                    if set_block >= set.block_start.len() {
                        break;
                    }
                    op_index = set.block_start[set_block] as usize;
                    set_index = set.op_list[op_index].block_index;
                } else {
                    cover_iter.next();
                    set_block += 1;
                    let op_max = set.block_start.get(set_block).map_or(set.op_list.len(), |&s| s as usize);
                    loop {
                        let e = &set.op_list[op_index];
                        if (!stores || e.store)
                            && cover_block.contain(Some(e.point))
                            && cover_block.boundary(Some(e.point)) == 0
                            && set.affects_test(e.id, addr)
                        {
                            return true;
                        }
                        op_index += 1;
                        if op_index >= op_max {
                            break;
                        }
                    }
                    if set_block >= set.block_start.len() {
                        break;
                    }
                    if stores {
                        set_index = set.op_list[op_index].block_index;
                    }
                }
            }
            false
        }
    }

    /// A random flow graph and the reads of a Varnode `1` in it; op `1`'s
    /// output, Varnode `2`, is implied and read by the ops after `split`.
    struct RandCtx {
        preds: Vec<Vec<int4>>,
        def: (Option<(int4, CoverPoint)>, bool),
        reads: Vec<(int4, CoverPoint, bool, Vec<int4>)>,
        split: usize,
    }

    fn vid(n: u64) -> crate::context::VarnodeId {
        crate::context::VarnodeId::from(KeyData::from_ffi(n))
    }

    fn oid(n: usize) -> OpId {
        OpId::from(KeyData::from_ffi(n as u64 + 1))
    }

    impl CoverContext for RandCtx {
        fn size_in(&self, bl: int4) -> int4 {
            self.preds.get(bl as usize).map_or(0, |p| p.len() as int4)
        }
        fn get_in(&self, bl: int4, j: int4) -> int4 {
            self.preds[bl as usize][j as usize]
        }
        fn def_point(&self, _vn: crate::context::VarnodeId) -> (Option<(int4, CoverPoint)>, bool) {
            self.def
        }
        fn descend(&self, vn: crate::context::VarnodeId) -> Vec<OpId> {
            let range = if vn == vid(1) { 0..self.split } else { self.split..self.reads.len() };
            range.map(oid).collect()
        }
        fn ref_point(&self, op: OpId, _vn: crate::context::VarnodeId) -> (int4, CoverPoint, bool, Vec<int4>) {
            self.reads[(slotmap::Key::data(&op).as_ffi() & 0xffff_ffff) as usize - 1].clone()
        }
        fn out_implied(&self, op: OpId) -> Option<crate::context::VarnodeId> {
            (op == oid(0) && self.split < self.reads.len()).then(|| vid(2))
        }
    }

    fn random_ctx(r: &mut Rng) -> RandCtx {
        let n = 1 + r.below(56) as usize;
        let preds = (0..n)
            .map(|b| if b == 0 { vec![] } else { (0..r.below(4)).map(|_| r.below(n as u64) as int4).collect() })
            .collect::<Vec<Vec<int4>>>();
        let def = match r.below(4) {
            0 => (None, true),
            1 => (None, false),
            _ => (Some((r.below(n as u64) as int4, op_point(r.below(12) as uintm))), false),
        };
        let reads = (0..r.below(10))
            .map(|_| {
                let bl = r.below(n as u64) as int4;
                let ins = &preds[bl as usize];
                if !ins.is_empty() && r.below(3) == 0 {
                    let listed = (0..1 + r.below(3)).map(|_| ins[r.below(ins.len() as u64) as usize]).collect();
                    (bl, meq_point(0), true, listed)
                } else {
                    (bl, op_point(r.below(12) as uintm), false, vec![])
                }
            })
            .collect::<Vec<_>>();
        let split = r.below(reads.len() as u64 + 1) as usize;
        RandCtx { preds, def, reads, split }
    }

    fn assert_canonical(c: &Cover) {
        assert!(c.runs.iter().all(|r| r.lo <= r.hi && (r.lo == r.hi || r.cb.is_full())), "{:?}", c.runs);
        assert!(c.runs.windows(2).all(|w| w[0].hi < w[1].lo
            && !(w[0].cb.is_full() && w[1].cb.is_full() && w[0].hi + 1 == w[1].lo)), "{:?}", c.runs);
    }

    fn blocks_of(c: &Cover) -> Vec<(int4, CoverBlock)> {
        c.iter().map(|(k, v)| (k, *v)).collect()
    }

    /// `rebuild`, `add_ref_points_for` and `add_ref_point_until`, walking from
    /// covers that already hold runs and lone blocks, leave exactly the blocks
    /// upstream's walks leave in a block map, and stop where they stop.
    #[test]
    fn walks_over_runs_leave_the_blocks_a_block_map_walk_leaves() {
        let mut r = Rng(0x2545_f491_4f6c_dd1d);
        for _ in 0..3000 {
            let ctx = random_ctx(&mut r);
            let mut c = Cover::new();
            c.rebuild(&ctx, vid(1));
            assert_eq!(blocks_of(&c), model::rebuild(&ctx, vid(1)).into_iter().collect::<Vec<_>>());
            assert_canonical(&c);

            let (mut c, mut m) = random_cover(&mut r);
            let ops: Vec<OpId> = (0..ctx.reads.len()).map(oid).collect();
            c.add_ref_points_for(&ctx, &ops, vid(1));
            for &op in &ops {
                model::add_ref(&mut m, &ctx, op, vid(1), &mut |_, _| false);
            }
            assert_eq!(blocks_of(&c), m.into_iter().collect::<Vec<_>>());
            assert_canonical(&c);

            if ctx.reads.is_empty() {
                continue;
            }
            let (mut c, mut m) = random_cover(&mut r);
            let (tb, tp) = (r.below(60) as int4, op_point(r.below(14) as uintm));
            let mut hit = |bl: int4, cb: &CoverBlock| bl == tb && cb.contain(Some(tp));
            let want = m.iter().any(|(&bl, cb)| hit(bl, cb)) || model::add_ref(&mut m, &ctx, oid(0), vid(1), &mut hit);
            assert_eq!(c.add_ref_point_until(&ctx, oid(0), vid(1), hit), want);
            assert_eq!(blocks_of(&c), m.into_iter().collect::<Vec<_>>());
            assert_canonical(&c);
        }
    }

    /// Every per-block lookup and the PcodeOpSet walk read runs as they read
    /// a map of one CoverBlock per block.
    #[test]
    fn lookups_and_op_set_walks_read_runs_as_a_block_map() {
        let mut r = Rng(0xd1b5_4a32_d192_ed03);
        let addr = Address::new(test_ram_space(), 0);
        for _ in 0..3000 {
            let (a, ma) = random_cover(&mut r);
            let (b, mb) = random_cover(&mut r);
            let first = |m: &model::Map| m.keys().next().copied().unwrap_or(1000000);
            assert_eq!(a.compare_to(&b), first(&ma).cmp(&first(&mb)) as int4);
            for bl in -1..62 {
                let want = ma.get(&bl).copied().unwrap_or_default();
                assert_eq!(*a.get_cover_block(bl), want);
                let other = mb.get(&bl);
                let by_block = ma.get(&bl).zip(other).map_or(0, |(x, y)| x.intersect(y));
                assert_eq!(a.intersect_by_block(bl, &b), by_block);
                for p in point_alphabet().into_iter().flatten() {
                    let found = ma.get(&bl);
                    let contain = |max| found.is_some_and(|cb| cb.contain(Some(p)) && (max == 1 || cb.boundary(Some(p)) == 0));
                    assert_eq!(a.contain(bl, p, 1), contain(1));
                    assert_eq!(a.contain(bl, p, 2), contain(2));
                    let def = found.map_or(0, |cb| match (cb.contain(Some(p)), cb.boundary(Some(p))) {
                        (false, _) => 0,
                        (true, 0) => 1,
                        (true, 2) => 2,
                        _ => 3,
                    });
                    assert_eq!(a.contain_varnode_def(Some((bl, p))), def, "{bl} {p:?}");
                }
            }
            let input = ma.get(&0).map_or(0, |cb| match (cb.contain(Some(CoverPoint::Input)), cb.boundary(Some(CoverPoint::Input))) {
                (false, _) => 0,
                (true, 0) => 1,
                (true, 2) => 2,
                _ => 3,
            });
            assert_eq!(a.contain_varnode_def(None), input);

            let entries: Vec<PcodeOpSetEntry> = (0..r.below(14))
                .map(|i| {
                    let u = r.below(12) as uintm;
                    PcodeOpSetEntry {
                        id: oid(i as usize),
                        block_index: r.below(60) as int4,
                        point: op_point(u),
                        order: u,
                        store: r.below(3) == 0,
                    }
                })
                .collect();
            let listed = entries.clone();
            let mut set = PcodeOpSet::new(Box::new(move || listed.clone()), Box::new(|op: OpId, _: &Address| (slotmap::Key::data(&op).as_ffi() & 0xffff_ffff) % 3 != 0));
            set.populate();
            let want = model::walk_op_set(&ma, &set, &addr, false)
                || (entries.iter().any(|e| e.store) && model::walk_op_set(&ma, &set, &addr, true));
            assert_eq!(a.intersect_op_set(&set, &addr), want);
        }
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
                    store: false,
                },
                PcodeOpSetEntry {
                    id: OpId::from(KeyData::from_ffi(2)),
                    block_index: 0,
                    point: op_point(20),
                    order: 20,
                    store: false,
                },
                PcodeOpSetEntry {
                    id: OpId::from(KeyData::from_ffi(3)),
                    block_index: 0,
                    point: op_point(10),
                    order: 10,
                    store: false,
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
            store: false,
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
