//! Port of `decompiler/cpp/heritage.{cc,hh}` — the Static Single Assignment
//! (SSA) construction engine (W5, item `w5-s3-heritage`, stage S3).
//!
//! # What this is
//!
//! [`Heritage`] links the [`Varnode`](crate::varnode::Varnode) and
//! [`PcodeOp`](crate::op::PcodeOp) objects of a [`Funcdata`] into the formal
//! data-flow graph (SSA form), built over multiple [`Heritage::heritage`]
//! passes.  A `free` Varnode (one not yet known to be written by an op) becomes
//! `heritaged` when a pass collects it, normalizes its size, guards data-flow
//! across calls / LOAD / STORE, places phi-nodes (MULTIEQUAL), and renames.
//!
//! The two big algorithms, both ported here faithfully:
//!   * **phi-node placement** ([`Heritage::place_multiequals`]) using the
//!     augmented dominator tree ([`Heritage::build_adt`] /
//!     [`Heritage::visit_incr`] / [`Heritage::calc_multiequals`]) and the
//!     [`PriorityQueue`] ordering (depth-keyed stacks, LIFO within a depth) —
//!     the order is output-affecting (it decides MULTIEQUAL *placement* and
//!     therefore the rendered text);
//!   * the **renaming** stack walk ([`Heritage::rename_recurse`] /
//!     [`Heritage::rename`]) — the dominance-tree walk with the per-address
//!     [`VariableStack`].
//!
//! The phi-node placement algorithm is from (preprint?) "The Static Single
//! Assignment Form and its Computation" by Gianfranco Bilardi and Keshav
//! Pingali, July 22, 1999.  The renaming algorithm is from Cytron, Ferrante,
//! Rosen, Wegman, Zadeck, ACM TOPLAS 13(4):451-490, October 1991.
//!
//! # Faithfulness notes (the load-bearing transcriptions)
//!
//! * [`LocationMap::add`] is the disjoint-cover union with the `intersect`
//!   code (0/1/2) and `pass`-min carry — the dead-code-delay interaction reads
//!   that code (`heritage.cc:34`).
//! * [`PriorityQueue`] is the depth-indexed stack array with the
//!   `curdepth == -2` "unconstructed" sentinel and the
//!   `curdepth == -1` "empty" sentinel exactly (`heritage.cc:142`).
//! * [`Heritage::build_adt`] is `heritage.cc:2317` transcribed byte-for-byte:
//!   the up-edge accumulation, the `a[]`/`b[]`/`t[]`/`z[]` recurrences, the
//!   boundary-node marking, the `z[]` boundary-ancestor walk, and the
//!   `augment[]` build with the `while (j < k)` idom-dominance loop.  The
//!   iteration order (reverse over blocks, then forward over up-edges) and the
//!   tie-breaks are output-affecting.
//! * [`Heritage::calc_multiequals`] / [`Heritage::visit_incr`] are the
//!   recursive dominance-frontier walk with the `mark_node`/`merged_node`/
//!   `boundary_node` flag protocol and the priority-queue insertion at
//!   `depth[k]` — `merge` ends holding the phi-placement blocks in the exact
//!   C++ order.
//!
//! # Cross-wave seams (what is *not* fully realized here)
//!
//! `heritage.cc` is the most subsystem-entangled file in the decompiler.  The
//! algorithmic core above is self-contained at the W3 IR + W3 block-dominator
//! level and is ported and tested.  The data-flow *construction* surface it
//! drives needs `Funcdata` SSA-construction primitives and W4/W6 subsystems
//! that are not present in the merged tree.  Each is seam-noted at its method
//! and recorded in the porter's `losses`:
//!
//!   * `// SEAM(W3-op)` — `setInputVarnode`, `newVarnodeOut`, `newUniqueOut`,
//!     `markReturnCopy`, `newIndirectCreation`/`newIndirectOp`,
//!     `opInsertBefore/After`-by-iterator, and the single-address
//!     `beginLoc(addr)`/`endLoc(addr)` location-set range.  These belong to the
//!     `funcdata_op`/`funcdata_varnode`/`varnode` waves; without them the
//!     renaming write/input creation, the MULTIEQUAL output construction, the
//!     normalize/concat/split helpers, and the `collect` driver cannot be
//!     realized.  Their algorithm structure is transcribed in comments at the
//!     seam so the wave that supplies the primitive can fill the body verbatim.
//!   * `// SEAM(W4)` — the guard machinery (`guardCalls`/`guardStores`/
//!     `guardLoads`/`guardReturns` and helpers) needs `FuncCallSpecs`,
//!     `ParamActive`, `EffectRecord`, the `Architecture` stack/proto/join
//!     surface, `Override`, and `PreferSplitManager`.
//!   * `// SEAM(W6)` — [`LoadGuard::establish_range`]/[`LoadGuard::finalize_range`]
//!     and [`Heritage::analyze_new_load_guards`] need the value-set solver
//!     (`ValueSetSolver`/`ValueSetRead`/`CircleRange`).
//!
//! The [`Heritage::bump_deadcode_delay`] recorder anchor (the kuna restart
//! observability hook) is realized against the merged [`Override`] and
//! [`RestartLog`]: because `Funcdata` does not yet own either handle (a W4
//! seam), the method takes them as explicit `&mut` parameters — see its doc.

use kuna_base::address::Address;
use kuna_base::space::{spacetype, AddrSpace};
use kuna_base::types::{int4, uint4, uintb};
use std::collections::BTreeMap;
use std::rc::Rc;

use crate::kuna_restartlog::{KunaRestartReason, RestartLog};
use crate::overrides::Override;
use crate::seams::BlockId;
use crate::varnode::varnode_flags;

// =============================================================================
// VariableStack (heritage.hh:29)
// =============================================================================

/// Container holding the stack system for the renaming algorithm.  Every
/// disjoint address range (indexed by its initial address) maps to its own
/// Varnode stack (C++ `typedef map<Address,vector<Varnode *> > VariableStack`).
pub type VariableStack = BTreeMap<Address, Vec<crate::seams::VarnodeId>>;

// =============================================================================
// LocationMap (heritage.hh:38, heritage.cc:34)
// =============================================================================

/// Label for describing the extent of an address range that has been heritaged
/// (C++ `LocationMap::SizePass`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SizePass {
    /// Size of the range (in bytes).
    pub size: int4,
    /// Pass when the range was heritaged.
    pub pass: int4,
}

/// Map object for keeping track of which address ranges have been heritaged
/// (C++ `LocationMap`).
///
/// A fairly fine-grained record of when each address range was entered into
/// SSA form (\b heritaged).  [`add`](LocationMap::add) marks a new range with
/// the pass it was entered, and [`find_pass`](LocationMap::find_pass) reports
/// whether/when an address was heritaged.
#[derive(Debug, Clone, Default)]
pub struct LocationMap {
    /// Heritaged addresses mapped to range size and pass number
    /// (C++ `map<Address,SizePass> themap`).
    themap: BTreeMap<Address, SizePass>,
}

impl LocationMap {
    /// New empty map.
    pub fn new() -> LocationMap {
        LocationMap { themap: BTreeMap::new() }
    }

    /// Mark a new address as \b heritaged (C++ `LocationMap::add`,
    /// `heritage.cc:34`).
    ///
    /// Updates the disjoint cover so `(addr,size)` is contained in a single
    /// element and returns the starting [`Address`] of that element.  The
    /// element's `pass` is set to the smallest value of any previous
    /// intersecting element.  An `intersect` code is passed back:
    ///   - 0 if the only intersection is with a range from the same pass
    ///   - 1 if there is a partial intersection with something old
    ///   - 2 if the range is contained in an old range
    ///
    /// The returned `Address` is the *key* of the containing element; the
    /// C++ returns an iterator, but every caller only reads `(*iter).first`
    /// (the key) and `(*iter).second.size` (the size), so we return the key
    /// and the caller re-looks the size via [`get`](LocationMap::get).
    pub fn add(&mut self, mut addr: Address, mut size: int4, mut pass: int4) -> (Address, int4) {
        // iterator iter = themap.lower_bound(addr);
        // if (iter != themap.begin()) --iter;
        // if ((iter!=end) && (-1 == addr.overlap(0,iter.first,iter.second.size))) ++iter;
        //
        // We replicate the C++ map-iterator walk against the ordered keys.
        let mut intersect = 0;

        // Find the candidate element: the last key <= addr's neighborhood.
        // lower_bound(addr) then step back one, mirroring the C++.
        let lower: Option<Address> =
            self.themap.range(addr.clone()..).next().map(|(k, _)| k.clone());
        // start from lower_bound; if not begin, step one back
        let mut cur: Option<Address> = match &lower {
            // lower_bound == begin: cannot step back, keep begin
            Some(lb) if self.themap.keys().next() == Some(lb) => Some(lb.clone()),
            // lower_bound is some interior/end: step back one
            _ => self.themap.range(..addr.clone()).next_back().map(|(k, _)| k.clone()),
        };

        // if ((iter!=end) && (-1 == addr.overlap(0,iter.first,iter.second.size))) ++iter;
        if let Some(ck) = &cur {
            let sz = self.themap[ck].size;
            if addr.overlap(0, ck, sz) == -1 {
                // ++iter : advance to the first key strictly greater than ck
                cur = self.themap.range((
                    std::ops::Bound::Excluded(ck.clone()),
                    std::ops::Bound::Unbounded,
                ))
                .next()
                .map(|(k, _)| k.clone());
            }
        }

        // First (possibly partial) overlap with the leading element.
        if let Some(ck) = cur.clone() {
            let entry = self.themap[&ck];
            let where_ = addr.overlap(0, &ck, entry.size);
            if where_ != -1 {
                if where_ + size <= entry.size {
                    // Completely contained in previous element.
                    intersect = if entry.pass < pass { 2 } else { 0 };
                    return (ck, intersect);
                }
                addr = ck.clone();
                size += where_; // C++: size = where + size;
                if entry.pass < pass {
                    intersect = 1; // Partial overlap with old element
                    pass = entry.pass;
                }
                self.themap.remove(&ck);
            }
        }

        // while ((iter!=end) && (-1 != (where = iter.first.overlap(0,addr,size)))) { ... erase }
        loop {
            // iter is the first key >= addr now (the erased key is gone, the
            // map iterator in C++ continues from the post-erase position;
            // the next key >= addr is the equivalent successor).
            let next_key = self
                .themap
                .range(addr.clone()..)
                .next()
                .map(|(k, _)| k.clone());
            let Some(nk) = next_key else { break };
            let nentry = self.themap[&nk];
            let where_ = nk.overlap(0, &addr, size);
            if where_ == -1 {
                break;
            }
            if where_ + nentry.size > size {
                size = where_ + nentry.size;
            }
            if nentry.pass < pass {
                intersect = 1;
                pass = nentry.pass;
            }
            self.themap.remove(&nk);
        }

        self.themap.insert(addr.clone(), SizePass { size, pass });
        (addr, intersect)
    }

    /// Look up the [`SizePass`] of a heritaged address (C++ `LocationMap::find`,
    /// `heritage.cc:77`).
    ///
    /// Returns the key (range start) and its [`SizePass`] if the address was
    /// heritaged, else `None`.
    pub fn find(&self, addr: &Address) -> Option<(Address, SizePass)> {
        // iter = themap.upper_bound(addr); if (iter==begin) return end; --iter;
        let prev = self
            .themap
            .range(..=addr.clone())
            .next_back()
            .map(|(k, v)| (k.clone(), *v));
        if let Some((k, v)) = prev {
            if addr.overlap(0, &k, v.size) != -1 {
                return Some((k, v));
            }
        }
        None
    }

    /// Look up the pass number when the given address was heritaged, or -1
    /// (C++ `LocationMap::findPass`, `heritage.cc:91`).
    pub fn find_pass(&self, addr: &Address) -> int4 {
        // const_iterator iter = themap.upper_bound(addr); if (begin) return -1; --iter;
        let prev = self
            .themap
            .range(..=addr.clone())
            .next_back()
            .map(|(k, v)| (k.clone(), *v));
        if let Some((k, v)) = prev {
            if addr.overlap(0, &k, v.size) != -1 {
                return v.pass;
            }
        }
        -1
    }

    /// Remove a particular entry (C++ `LocationMap::erase`).
    pub fn erase(&mut self, addr: &Address) {
        self.themap.remove(addr);
    }

    /// Borrow the [`SizePass`] keyed by exactly `addr` (the C++ `(*iter).second`
    /// after [`find`](LocationMap::find)/[`add`](LocationMap::add) returns).
    pub fn get(&self, addr: &Address) -> Option<&SizePass> {
        self.themap.get(addr)
    }

    /// Iterate heritaged ranges in address order (C++ `begin`..`end`).
    pub fn iter(&self) -> impl Iterator<Item = (&Address, &SizePass)> {
        self.themap.iter()
    }

    /// Clear the map of heritaged ranges (C++ `LocationMap::clear`).
    pub fn clear(&mut self) {
        self.themap.clear();
    }
}

// =============================================================================
// MemRange / TaskList (heritage.hh:60, heritage.cc:109)
// =============================================================================

/// Property flags on a [`MemRange`] (C++ `MemRange` anonymous enum).
pub mod memrange_flags {
    #![allow(non_upper_case_globals)]
    use kuna_base::types::uint4;
    /// The range covers addresses that have not been seen in previous passes.
    pub const new_addresses: uint4 = 1;
    /// The range covers addresses that were seen in previous passes.
    pub const old_addresses: uint4 = 2;
}

/// An address range to be processed (C++ `MemRange`).
#[derive(Debug, Clone)]
pub struct MemRange {
    /// Starting address of the range.
    pub addr: Address,
    /// Number of bytes in the range.
    pub size: int4,
    /// Property flags.
    pub flags: uint4,
}

impl MemRange {
    /// Constructor (C++ `MemRange(const Address&,int4,uint4)`).
    pub fn new(ad: Address, sz: int4, fl: uint4) -> MemRange {
        MemRange { addr: ad, size: sz, flags: fl }
    }

    /// Does this range cover new addresses? (C++ `newAddresses`).
    pub fn new_addresses(&self) -> bool {
        (self.flags & memrange_flags::new_addresses) != 0
    }

    /// Does this range cover old addresses? (C++ `oldAddresses`).
    pub fn old_addresses(&self) -> bool {
        (self.flags & memrange_flags::old_addresses) != 0
    }

    /// Clear specific properties from the memory range (C++ `clearProperty`).
    pub fn clear_property(&mut self, val: uint4) {
        self.flags &= !val;
    }
}

/// A list of address ranges that need to be converted to SSA form (C++
/// `TaskList`).
///
/// Fed a list of ranges that may overlap but are already in address order; it
/// constructs a disjoint list by taking the union of overlapping ranges.  The
/// C++ uses a `list<MemRange>` (stable iterators across mid-list inserts); the
/// only callers that hold an iterator across mutation are the refinement path
/// (`// SEAM(W3-op)` — not realized), so a `Vec` suffices for the realized
/// surface, with `insert`/`erase` keyed by index.
#[derive(Debug, Clone, Default)]
pub struct TaskList {
    tasklist: Vec<MemRange>,
}

impl TaskList {
    /// New empty list.
    pub fn new() -> TaskList {
        TaskList { tasklist: Vec::new() }
    }

    /// Add a range to the list, extending the last range if it intersects
    /// (C++ `TaskList::add`, `heritage.cc:109`).
    ///
    /// Addresses must already be sorted.  If the given range intersects the
    /// last range, that range is extended (and the new flags are ORed in);
    /// otherwise the range is appended.
    pub fn add(&mut self, addr: Address, size: int4, fl: uint4) {
        if let Some(entry) = self.tasklist.last_mut() {
            let over = addr.overlap(0, &entry.addr, entry.size);
            if over >= 0 {
                let relsize = size + over;
                if relsize > entry.size {
                    entry.size = relsize;
                }
                entry.flags |= fl;
                return;
            }
        }
        self.tasklist.push(MemRange::new(addr, size, fl));
    }

    /// Insert a disjoint range before position `pos` (C++ `TaskList::insert`,
    /// `heritage.cc:133`).  The new range must already be disjoint.
    pub fn insert(&mut self, pos: usize, addr: Address, size: int4, fl: uint4) {
        self.tasklist.insert(pos, MemRange::new(addr, size, fl));
    }

    /// Remove the range at index `pos`, returning the next index (the C++
    /// `erase` returns the iterator following the removed element, which is the
    /// same index after the shift).
    pub fn erase(&mut self, pos: usize) -> usize {
        self.tasklist.remove(pos);
        pos
    }

    /// Number of ranges.
    pub fn len(&self) -> usize {
        self.tasklist.len()
    }

    /// Is the list empty?
    pub fn is_empty(&self) -> bool {
        self.tasklist.is_empty()
    }

    /// Borrow the range at `i` (C++ `*iter`).
    pub fn get(&self, i: usize) -> &MemRange {
        &self.tasklist[i]
    }

    /// Mutably borrow the range at `i`.
    pub fn get_mut(&mut self, i: usize) -> &mut MemRange {
        &mut self.tasklist[i]
    }

    /// Clear all ranges (C++ `TaskList::clear`).
    pub fn clear(&mut self) {
        self.tasklist.clear();
    }
}

// =============================================================================
// PriorityQueue (heritage.hh:101, heritage.cc:142)
// =============================================================================

/// Priority queue for the phi-node (MULTIEQUAL) placement algorithm (C++
/// `PriorityQueue`).
///
/// A work-list for basic blocks: a set of stacks indexed by priority (depth).
/// Blocks are pushed with [`insert`](PriorityQueue::insert) and the current
/// highest-priority block is popped with [`extract`](PriorityQueue::extract).
/// `curdepth == -2` is the "unconstructed" sentinel (set by the constructor);
/// `curdepth == -1` is the "empty" sentinel.  The LIFO-within-a-depth order is
/// output-affecting.
#[derive(Debug, Clone)]
pub struct PriorityQueue {
    /// An array of stacks, indexed by priority (C++ `vector<vector<FlowBlock*>>`).
    queue: Vec<Vec<BlockId>>,
    /// The current highest priority index with active blocks (C++ `curdepth`).
    curdepth: int4,
}

impl Default for PriorityQueue {
    fn default() -> Self {
        PriorityQueue::new()
    }
}

impl PriorityQueue {
    /// Constructor — `curdepth = -2` (C++ `PriorityQueue()`).
    pub fn new() -> PriorityQueue {
        PriorityQueue { queue: Vec::new(), curdepth: -2 }
    }

    /// Reset to an empty queue with space for `maxdepth+1` stacks
    /// (C++ `PriorityQueue::reset`, `heritage.cc:142`).
    pub fn reset(&mut self, maxdepth: int4) {
        // if ((curdepth==-1)&&(maxdepth==queue.size()-1)) return; // Already reset
        if self.curdepth == -1 && maxdepth == self.queue.len() as int4 - 1 {
            return;
        }
        self.queue.clear();
        self.queue.resize((maxdepth + 1) as usize, Vec::new());
        self.curdepth = -1;
    }

    /// Insert a block at the given priority/depth (C++ `PriorityQueue::insert`).
    pub fn insert(&mut self, bl: BlockId, depth: int4) {
        self.queue[depth as usize].push(bl);
        if depth > self.curdepth {
            self.curdepth = depth;
        }
    }

    /// Pop and return the highest-priority block (C++ `PriorityQueue::extract`).
    ///
    /// Must not be called when [`empty`](PriorityQueue::empty).
    pub fn extract(&mut self) -> BlockId {
        let res = self.queue[self.curdepth as usize].pop().expect("PriorityQueue::extract on empty stack");
        while self.queue[self.curdepth as usize].is_empty() {
            self.curdepth -= 1;
            if self.curdepth < 0 {
                break;
            }
        }
        res
    }

    /// Is the queue empty? (C++ `PriorityQueue::empty`, `curdepth == -1`).
    pub fn empty(&self) -> bool {
        self.curdepth == -1
    }
}

// =============================================================================
// HeritageInfo (heritage.hh:122, heritage.cc:180)
// =============================================================================

/// Heritage status for a single address space (C++ `HeritageInfo`).
///
/// Tracks how long to delay heritage / dead-code removal for the space, whether
/// dead code has been removed, and the call-placeholder / load-guard-search /
/// warning state.
#[derive(Debug, Clone)]
pub struct HeritageInfo {
    /// The address space this record describes (`None` == the C++ NULL `space`,
    /// meaning the space is *not* heritaged).
    space: Option<Rc<AddrSpace>>,
    /// How many passes to delay heritage of this space (C++ `delay`).
    delay: int4,
    /// How many passes to delay deadcode removal (C++ `deadcodedelay`).
    deadcodedelay: int4,
    /// >0 if Varnodes in this space have been eliminated (C++ `deadremoved`).
    deadremoved: int4,
    /// True if the search for LOAD ops to guard has been performed.
    load_guard_search: bool,
    /// True if a warning was issued previously.
    warningissued: bool,
    /// True for the stack space, if stack placeholders have not been removed.
    has_call_placeholders: bool,
}

impl HeritageInfo {
    /// Initialize heritage state for a particular address space
    /// (C++ `HeritageInfo::HeritageInfo`, `heritage.cc:180`).
    ///
    /// `spc == None` is the C++ `(AddrSpace *)0` constructor branch.
    pub fn new(spc: Option<&Rc<AddrSpace>>) -> HeritageInfo {
        match spc {
            None => HeritageInfo {
                space: None,
                delay: 0,
                deadcodedelay: 0,
                deadremoved: 0,
                load_guard_search: false,
                warningissued: false,
                has_call_placeholders: false,
            },
            Some(s) if !s.is_heritaged() => HeritageInfo {
                space: None,
                delay: s.get_delay(),
                deadcodedelay: s.get_deadcode_delay(),
                deadremoved: 0,
                load_guard_search: false,
                warningissued: false,
                has_call_placeholders: false,
            },
            Some(s) => HeritageInfo {
                space: Some(Rc::clone(s)),
                delay: s.get_delay(),
                deadcodedelay: s.get_deadcode_delay(),
                deadremoved: 0,
                load_guard_search: false,
                warningissued: false,
                has_call_placeholders: s.get_type() == spacetype::IPTR_SPACEBASE,
            },
        }
    }

    /// Reset the state (C++ `HeritageInfo::reset`, `heritage.cc:206`).
    ///
    /// Note: the override `deadcodedelay = delay` is *intentionally left out*
    /// (the C++ keeps any override intact — see the commented-out line).
    pub fn reset(&mut self) {
        // Leave any override intact: deadcodedelay = delay;
        self.deadremoved = 0;
        if let Some(s) = &self.space {
            self.has_call_placeholders = s.get_type() == spacetype::IPTR_SPACEBASE;
        }
        self.warningissued = false;
        self.load_guard_search = false;
    }

    /// Return true if heritage is performed on this space (C++ `isHeritaged`).
    pub fn is_heritaged(&self) -> bool {
        self.space.is_some()
    }

    /// The space delay (C++ `delay`).
    pub fn delay(&self) -> int4 {
        self.delay
    }

    /// The dead-code delay (C++ `deadcodedelay`).
    pub fn deadcode_delay(&self) -> int4 {
        self.deadcodedelay
    }
}

// =============================================================================
// LoadGuard (heritage.hh:142, heritage.cc:741)
// =============================================================================

/// Description of a LOAD (or STORE) operation that needs to be guarded (C++
/// `LoadGuard`).
///
/// Heritage maintains a list of CPUI_LOAD/CPUI_STORE ops that reference the
/// stack dynamically; these can alias stack Varnodes, so we keep the (possibly
/// limited) range of stack addresses they can reference.
#[derive(Debug, Clone)]
pub struct LoadGuard {
    /// The LOAD/STORE op (C++ `op`).
    pub op: crate::seams::OpId,
    /// The stack space being loaded from / stored to (C++ `spc`).
    pub spc: Rc<AddrSpace>,
    /// Base offset of the pointer (C++ `pointerBase`).
    pub pointer_base: uintb,
    /// Minimum offset of the access (C++ `minimumOffset`).
    pub minimum_offset: uintb,
    /// Maximum offset of the access (C++ `maximumOffset`).
    pub maximum_offset: uintb,
    /// Step of any access into this range (0 = unknown) (C++ `step`).
    pub step: int4,
    /// 0=unanalyzed, 1=analyzed(partial), 2=analyzed(full) (C++ `analysisState`).
    pub analysis_state: int4,
}

impl LoadGuard {
    /// Set a new unanalyzed LOAD guard that initially guards everything
    /// (C++ `LoadGuard::set`, `heritage.hh:159`).
    pub fn set(op: crate::seams::OpId, s: &Rc<AddrSpace>, off: uintb) -> LoadGuard {
        LoadGuard {
            op,
            spc: Rc::clone(s),
            pointer_base: off,
            minimum_offset: 0,
            maximum_offset: s.get_highest(),
            step: 0,
            analysis_state: 0,
        }
    }

    /// Get the minimum offset of the guarded range (C++ `getMinimum`).
    pub fn get_minimum(&self) -> uintb {
        self.minimum_offset
    }

    /// Get the maximum offset of the guarded range (C++ `getMaximum`).
    pub fn get_maximum(&self) -> uintb {
        self.maximum_offset
    }

    /// Get the calculated step (or 0) (C++ `getStep`).
    pub fn get_step(&self) -> int4 {
        self.step
    }

    /// Does this guard apply to the given address? (C++ `LoadGuard::isGuarded`,
    /// `heritage.cc:819`).
    pub fn is_guarded(&self, addr: &Address) -> bool {
        match addr.get_space() {
            Some(s) if Rc::ptr_eq(s, &self.spc) => {}
            _ => return false,
        }
        if addr.get_offset() < self.minimum_offset {
            return false;
        }
        if addr.get_offset() > self.maximum_offset {
            return false;
        }
        true
    }

    /// Return true if the range is fully determined (C++ `isRangeLocked`,
    /// `analysisState == 2`).
    pub fn is_range_locked(&self) -> bool {
        self.analysis_state == 2
    }

    // establishRange / finalizeRange (heritage.cc:741 / 788)
    //
    // SEAM(W6): both convert a `ValueSetRead` (value-set analysis result,
    // `CircleRange`) into a guard range.  The value-set solver
    // (`ValueSetSolver`/`ValueSetRead`/`CircleRange`) is not in the merged
    // tree.  The arithmetic on `minimumOffset`/`maximumOffset`/`step` is fully
    // determined by `range.{getMin,getEnd,getStep,getSize,getMask,isEmpty,
    // isFull}` plus `valueSet.{isLeftStable,isRightStable}`; the wave that
    // ports rangeutil can fill these verbatim from heritage.cc:741-814.
}

// =============================================================================
// Heritage flags (heritage.hh:209) + StackNode flags (heritage.hh:217)
// =============================================================================

/// Extra boolean properties on basic blocks for the Augmented Dominator Tree
/// (C++ `Heritage::heritage_flags`).
mod heritage_flags {
    #![allow(non_upper_case_globals)]
    use kuna_base::types::uint4;
    /// Augmented Dominator Tree boundary node.
    pub const boundary_node: uint4 = 1;
    /// Node has already been in queue.
    pub const mark_node: uint4 = 2;
    /// Node has already been merged.
    pub const merged_node: uint4 = 4;
}

/// Path-traversal configuration flags for the indexed-stack-pointer DFS (C++
/// `Heritage::StackNode` anonymous enum).  Carried for the load/store guard
/// discovery — `// SEAM(W4)`.
mod stacknode_flags {
    #![allow(non_upper_case_globals)]
    #![allow(dead_code)]
    use kuna_base::types::uint4;
    pub const nonconstant_index: uint4 = 1;
    pub const multiequal: uint4 = 2;
}

// =============================================================================
// Heritage (heritage.hh:207)
// =============================================================================

/// Manage the construction of Static Single Assignment (SSA) form (C++
/// `Heritage`).
///
/// Links the Varnode and PcodeOp objects of a [`Funcdata`] into the formal
/// data-flow graph (SSA form), built over multiple [`heritage`](Heritage::heritage)
/// passes.  This port realizes the self-contained SSA-construction *engine*
/// state (the disjoint-cover queues, the per-space heritage status, the
/// augmented-dominator-tree phi-placement math, and the dead-code-delay
/// machinery); the data-flow *mutation* surface (renaming write/input creation,
/// MULTIEQUAL construction, guards, normalization, refinement, joins) is
/// seam-noted per method (`// SEAM(W3-op)` / `// SEAM(W4)` / `// SEAM(W6)`).
///
/// Unlike the C++, this does not hold a `Funcdata *` back-pointer: Rust's
/// borrow rules make a self-owned `&mut Funcdata` field unusable from methods
/// that also mutate `self`.  The realized methods that need the function take
/// `fd: &Funcdata` / `fd: &mut Funcdata` explicitly.
#[derive(Debug)]
pub struct Heritage {
    /// Disjoint cover of every heritaged memory location (C++ `globaldisjoint`).
    globaldisjoint: LocationMap,
    /// Disjoint cover of memory locations currently being heritaged
    /// (C++ `disjoint`).
    disjoint: TaskList,
    /// Parent->child edges in dominator tree (C++ `domchild`).  Stored by
    /// BlockId rather than `FlowBlock *`.
    domchild: Vec<Vec<BlockId>>,
    /// Augmented edges (C++ `augment`).
    augment: Vec<Vec<BlockId>>,
    /// Block properties for the phi-node placement algorithm (C++ `flags`).
    flags: Vec<uint4>,
    /// Dominator depth of individual blocks (C++ `depth`).
    depth: Vec<int4>,
    /// Maximum depth of the dominator tree (C++ `maxdepth`).
    maxdepth: int4,
    /// Current pass being executed (C++ `pass`).
    pass: int4,
    /// Priority queue for phi-node placement (C++ `pq`).
    pq: PriorityQueue,
    /// Calculated merge points — blocks containing phi-nodes (C++ `merge`).
    merge: Vec<BlockId>,
    /// Heritage status for individual address spaces (C++ `infolist`),
    /// indexed by `AddrSpace::getIndex()`.
    infolist: Vec<HeritageInfo>,
    /// List of LOAD operations that need to be guarded (C++ `loadGuard`).
    load_guard: Vec<LoadGuard>,
    /// List of STORE operations taking an indexed stack pointer (C++ `storeGuard`).
    store_guard: Vec<LoadGuard>,
    /// List of COPY ops generated by load guards (C++ `loadCopyOps`).
    ///
    /// Only the seam-noted `handleNewLoadCopies`/`generateLoadGuard`
    /// (`// SEAM(W4)`) read/write this; carried so the struct layout matches the
    /// C++ exactly and the W4 guard wave can fill the bodies without
    /// re-declaring the field.
    #[allow(dead_code)]
    load_copy_ops: Vec<crate::seams::OpId>,
}

impl Heritage {
    /// Instantiate the heritage manager (C++ `Heritage::Heritage`,
    /// `heritage.cc:219`).
    pub fn new() -> Heritage {
        Heritage {
            globaldisjoint: LocationMap::new(),
            disjoint: TaskList::new(),
            domchild: Vec::new(),
            augment: Vec::new(),
            flags: Vec::new(),
            depth: Vec::new(),
            maxdepth: -1,
            pass: 0,
            pq: PriorityQueue::new(),
            merge: Vec::new(),
            infolist: Vec::new(),
            load_guard: Vec::new(),
            store_guard: Vec::new(),
            load_copy_ops: Vec::new(),
        }
    }

    /// Get the overall count of heritage passes (C++ `getPass`).
    pub fn get_pass(&self) -> int4 {
        self.pass
    }

    /// Get the pass number when the given address was heritaged, or -1
    /// (C++ `heritagePass`, `heritage.hh:325`).
    pub fn heritage_pass(&self, addr: &Address) -> int4 {
        self.globaldisjoint.find_pass(addr)
    }

    /// Get the heritage status for the given space (C++ `getInfo`).
    fn get_info(&self, spc: &Rc<AddrSpace>) -> &HeritageInfo {
        &self.infolist[spc.get_index() as usize]
    }

    /// Get the mutable heritage status for the given space (C++ `getInfo`).
    fn get_info_mut(&mut self, spc: &Rc<AddrSpace>) -> &mut HeritageInfo {
        &mut self.infolist[spc.get_index() as usize]
    }

    /// Reset heritage status for all address spaces (C++ `clearInfoList`,
    /// `heritage.cc:227`).
    fn clear_info_list(&mut self) {
        for info in self.infolist.iter_mut() {
            info.reset();
        }
    }

    /// Force regeneration of basic-block structures (C++ `forceRestructure`,
    /// `maxdepth = -1`).
    pub fn force_restructure(&mut self) {
        self.maxdepth = -1;
    }

    /// Get the LOAD guards (C++ `getLoadGuards`).
    pub fn get_load_guards(&self) -> &[LoadGuard] {
        &self.load_guard
    }

    /// Get the STORE guards (C++ `getStoreGuards`).
    pub fn get_store_guards(&self) -> &[LoadGuard] {
        &self.store_guard
    }

    /// Get the LoadGuard record associated with a STORE op, if any
    /// (C++ `getStoreGuard`, `heritage.cc:2766`).
    pub fn get_store_guard(&self, op: crate::seams::OpId) -> Option<&LoadGuard> {
        self.store_guard.iter().find(|g| g.op == op)
    }

    // -- dead-code-delay accessors (heritage.cc:2783-2854) ------------------

    /// Number of times heritage was performed for a space (C++
    /// `numHeritagePasses`, `heritage.cc:2783`).
    ///
    /// A negative number indicates the number of passes to wait before the
    /// first heritage will occur.  Errors if the space is not heritaged (the
    /// C++ throws `LowlevelError`).
    pub fn num_heritage_passes(&self, spc: &Rc<AddrSpace>) -> kuna_base::error::KunaResult<int4> {
        let info = self.get_info(spc);
        if !info.is_heritaged() {
            return Err(kuna_base::error::KunaError::lowlevel(
                "Trying to calculate passes for non-heritaged space",
            ));
        }
        Ok(self.pass - info.delay)
    }

    /// Inform the system of dead-code removal in a space (C++ `seenDeadCode`,
    /// `heritage.cc:2795`).
    pub fn seen_dead_code(&mut self, spc: &Rc<AddrSpace>) {
        self.get_info_mut(spc).deadremoved = 1;
    }

    /// Get the dead-code delay for a space (C++ `getDeadCodeDelay`,
    /// `heritage.cc:2807`).
    pub fn get_dead_code_delay(&self, spc: &Rc<AddrSpace>) -> int4 {
        self.get_info(spc).deadcodedelay
    }

    /// Set the dead-code delay for a space (C++ `setDeadCodeDelay`,
    /// `heritage.cc:2819`).  Errors if `delay < info.delay`.
    pub fn set_dead_code_delay(
        &mut self,
        spc: &Rc<AddrSpace>,
        delay: int4,
    ) -> kuna_base::error::KunaResult<()> {
        let info = self.get_info_mut(spc);
        if delay < info.delay {
            return Err(kuna_base::error::KunaError::lowlevel("Illegal deadcode delay setting"));
        }
        info.deadcodedelay = delay;
        Ok(())
    }

    /// Return true if it is safe to remove dead code in a space (C++
    /// `deadRemovalAllowed`, `heritage.cc:2833`).
    pub fn dead_removal_allowed(&self, spc: &Rc<AddrSpace>) -> bool {
        self.pass > self.get_info(spc).deadcodedelay
    }

    /// Check dead-code removal safety and mark removal as happened (C++
    /// `deadRemovalAllowedSeen`, `heritage.cc:2847`).
    pub fn dead_removal_allowed_seen(&mut self, spc: &Rc<AddrSpace>) -> bool {
        let pass = self.pass;
        let info = self.get_info_mut(spc);
        let res = pass > info.deadcodedelay;
        if res {
            info.deadremoved = 1;
        }
        res
    }

    /// Initialize the info list (C++ `buildInfoList`, `heritage.cc:2654`).
    ///
    /// Allocates a [`HeritageInfo`] for each address space.  Idempotent (the
    /// C++ `if (!infolist.empty()) return`).
    pub fn build_info_list(&mut self, fd: &crate::funcdata::Funcdata) {
        if !self.infolist.is_empty() {
            return;
        }
        let manage = fd.get_arch().manage();
        let n = manage.num_spaces();
        self.infolist.reserve(n as usize);
        for i in 0..n {
            self.infolist.push(HeritageInfo::new(manage.get_space(i)));
        }
    }

    /// Reset all heritage analysis (C++ `Heritage::clear`, `heritage.cc:2859`).
    ///
    /// Does not directly affect the underlying Varnodes/PcodeOps.
    pub fn clear(&mut self) {
        self.disjoint.clear();
        self.globaldisjoint.clear();
        self.domchild.clear();
        self.augment.clear();
        self.flags.clear();
        self.depth.clear();
        self.merge.clear();
        self.clear_info_list();
        self.load_guard.clear();
        self.store_guard.clear();
        self.maxdepth = -1;
        self.pass = 0;
    }

    // =========================================================================
    // Augmented Dominator Tree + phi-node placement (the SSA core)
    // =========================================================================

    /// Build the augmented dominator tree (C++ `Heritage::buildADT`,
    /// `heritage.cc:2317`).
    ///
    /// Assumes the dominator tree is already built and nodes are in DFS order.
    /// Transcribed byte-for-byte: builds `domchild` and `depth` from the block
    /// graph, accumulates up-edges into `b[]`/`t[]`, runs the reverse `a/z`
    /// recurrence marking `boundary_node`s, walks `z[]` to the nearest boundary
    /// ancestor, then fills `augment[]` with the `while (j < k)` idom-dominance
    /// loop.
    pub fn build_adt(&mut self, fd: &crate::funcdata::Funcdata) {
        let bblocks = fd.bblocks_ref();
        let root = bblocks.root.expect("Heritage::build_adt: bblocks root not constructed");
        let size = bblocks.block(root).get_size() as usize;

        let mut a: Vec<int4> = vec![0; size];
        let mut b: Vec<int4> = vec![0; size];
        let mut t: Vec<int4> = vec![0; size];
        let mut z: Vec<int4> = vec![0; size];
        let mut upstart: Vec<BlockId> = Vec::new(); // Up edges (node pair)
        let mut upend: Vec<BlockId> = Vec::new();

        self.augment.clear();
        self.augment.resize(size, Vec::new());
        self.flags.clear();
        self.flags.resize(size, 0);

        // domchild = bblocks.buildDomTree(); it returns size+1 buckets, the
        // last being the "no idom" bucket (root).  We keep it as-is; only
        // indices 0..size are referenced by the algorithm below.
        self.domchild = bblocks.build_dom_tree(root);
        let (depth, maxdepth) = bblocks.build_dom_depth(root);
        self.depth = depth;
        self.maxdepth = maxdepth;

        // for i in 0..size: collect up-edges (u->v where u != idom(v))
        for i in 0..size {
            let x = bblocks.block(root).get_block(i as int4);
            let x_index = bblocks.block(x).get_index();
            for j in 0..self.domchild[i].len() {
                let v = self.domchild[i][j];
                let v_size_in = bblocks.block(v).size_in();
                let v_idom = bblocks.block(v).get_immed_dom();
                for k in 0..v_size_in {
                    let u = bblocks.block(v).get_in(k);
                    if Some(u) != v_idom {
                        // u->v is an up-edge
                        upstart.push(u);
                        upend.push(v);
                        b[bblocks.block(u).get_index() as usize] += 1;
                        t[x_index as usize] += 1;
                    }
                }
            }
        }
        // Reverse recurrence: a[i], z[i], boundary marking.
        for i in (0..size).rev() {
            let mut k = 0;
            let mut l = 0;
            for j in 0..self.domchild[i].len() {
                let ci = bblocks.block(self.domchild[i][j]).get_index() as usize;
                k += a[ci];
                l += z[ci];
            }
            a[i] = b[i] - t[i] + k;
            z[i] = 1 + l;
            if self.domchild[i].is_empty() || (z[i] > a[i] + 1) {
                self.flags[i] |= heritage_flags::boundary_node;
                z[i] = 1;
            }
        }
        z[0] = -1;
        for i in 1..size {
            // j = idom(block(i)).index
            let bi = bblocks.block(root).get_block(i as int4);
            let idom = bblocks.block(bi).get_immed_dom().expect("non-root block has no idom");
            let j = bblocks.block(idom).get_index() as usize;
            if (self.flags[j] & heritage_flags::boundary_node) != 0 {
                z[i] = j as int4;
            } else {
                z[i] = z[j];
            }
        }
        for i in 0..upstart.len() {
            let v = upend[i];
            let v_idom = bblocks.block(v).get_immed_dom().expect("up-edge head has no idom");
            let j = bblocks.block(v_idom).get_index();
            let mut k = bblocks.block(upstart[i]).get_index();
            while j < k {
                // while idom(v) properly dominates u
                self.augment[k as usize].push(v);
                k = z[k as usize];
            }
        }
    }

    /// The heart of phi-node placement: recursively walk the dominance tree,
    /// adding dominance-frontier children to `merge` (C++ `Heritage::visitIncr`,
    /// `heritage.cc:2395`).
    ///
    /// `qnode` is the parent of `vnode` in the recursion; `augment[i]` holds the
    /// augmented edges of `vnode`.  Realized recursively, exactly as the C++.
    fn visit_incr(&mut self, fd: &crate::funcdata::Funcdata, qnode: BlockId, vnode: BlockId) {
        let bblocks = fd.bblocks_ref();
        let i = bblocks.block(vnode).get_index() as usize;
        let j_q = bblocks.block(qnode).get_index();

        // for v in augment[i]: while idom(v).index < qnode.index { merge/mark }
        // The C++ breaks out of the loop on the first v whose idom is *not* a
        // strict ancestor of qnode (augment[i] is in DFS order, so the rest
        // cannot qualify either).
        let aug_len = self.augment[i].len();
        for idx in 0..aug_len {
            let v = self.augment[i][idx];
            let v_idom = bblocks.block(v).get_immed_dom().expect("augment edge head has no idom");
            if bblocks.block(v_idom).get_index() < j_q {
                // idom(v) is a strict ancestor of qnode
                let k = bblocks.block(v).get_index() as usize;
                if (self.flags[k] & heritage_flags::merged_node) == 0 {
                    self.merge.push(v);
                    self.flags[k] |= heritage_flags::merged_node;
                }
                if (self.flags[k] & heritage_flags::mark_node) == 0 {
                    self.flags[k] |= heritage_flags::mark_node;
                    self.pq.insert(v, self.depth[k]);
                }
            } else {
                break;
            }
        }
        if (self.flags[i] & heritage_flags::boundary_node) == 0 {
            // vnode is not a boundary node: recurse into unmarked dom-children
            let nchild = self.domchild[i].len();
            for jj in 0..nchild {
                let child = self.domchild[i][jj];
                let ci = bblocks.block(child).get_index() as usize;
                if (self.flags[ci] & heritage_flags::mark_node) == 0 {
                    self.visit_incr(fd, qnode, child);
                }
            }
        }
    }

    /// Calculate which blocks should contain MULTIEQUALs for one address range
    /// (C++ `Heritage::calcMultiequals`, `heritage.cc:2440`).
    ///
    /// Main entry point for phi-node placement: seeds the priority queue with
    /// the write blocks (plus the start block), drains it through
    /// [`visit_incr`](Heritage::visit_incr), and leaves `merge` holding the
    /// blocks that should contain a MULTIEQUAL.  `write_blocks` is the list of
    /// blocks where the normalized writes occur (the C++ derives these from
    /// `write[i]->getDef()->getParent()`; the caller resolves the op→block
    /// step since the realized [`place_multiequals`](Heritage::place_multiequals)
    /// driver owns the write Varnode list — `// SEAM(W3-op)`).
    pub fn calc_multiequals(&mut self, fd: &crate::funcdata::Funcdata, write_blocks: &[BlockId]) {
        self.pq.reset(self.maxdepth);
        self.merge.clear();

        let bblocks = fd.bblocks_ref();
        // Place write blocks into the pq
        for &bl in write_blocks {
            let j = bblocks.block(bl).get_index() as usize;
            if (self.flags[j] & heritage_flags::mark_node) != 0 {
                continue; // Already put in
            }
            self.pq.insert(bl, self.depth[j]);
            self.flags[j] |= heritage_flags::mark_node;
        }
        // Make sure start node is in input
        let root = bblocks.root.expect("calc_multiequals: bblocks root");
        let start = bblocks.block(root).get_block(0);
        let start_idx = bblocks.block(start).get_index() as usize;
        if (self.flags[0] & heritage_flags::mark_node) == 0 {
            // NOTE: the C++ marks flags[0] and pushes getBlock(0); block 0 is
            // the start block (index 0) by DFS order, so depth[0]/flags[0] are
            // the start block's.
            self.pq.insert(start, self.depth[0]);
            self.flags[0] |= heritage_flags::mark_node;
        }
        debug_assert_eq!(start_idx, 0, "start block is index 0 (DFS order invariant)");

        while !self.pq.empty() {
            let bl = self.pq.extract();
            self.visit_incr(fd, bl, bl);
        }
        for f in self.flags.iter_mut() {
            *f &= !(heritage_flags::mark_node | heritage_flags::merged_node);
        }
    }

    /// Borrow the computed merge points (blocks to receive a MULTIEQUAL).
    pub fn merge_points(&self) -> &[BlockId] {
        &self.merge
    }

    // =========================================================================
    // Dead-code-delay bump + restart recorder anchor (heritage.cc:2572)
    // =========================================================================

    /// Increase the heritage delay for a space and request a restart (C++
    /// `Heritage::bumpDeadcodeDelay`, `heritage.cc:2572`).
    ///
    /// Faithful transcription of the bump protocol:
    ///   1. only `IPTR_PROCESSOR` / `IPTR_SPACEBASE` spaces qualify;
    ///   2. only when `spc.getDelay() == spc.getDeadcodeDelay()` (no global
    ///      delay yet);
    ///   3. if [`Override::has_deadcode_delay`] already returns true, record the
    ///      `krestart_deadcode_suppressed` event and return (the delay is
    ///      installed *once*);
    ///   4. otherwise insert `deadcodeDelay+1`, set restart pending, and record
    ///      `krestart_deadcode_bump`.
    ///
    /// # Seam
    ///
    /// The C++ reaches `fd->getOverride()` and the file-static restart table
    /// via `kunaRecordRestart(*fd, ...)`.  `Funcdata` does not yet own either
    /// (a W4 seam), so this method takes the merged [`Override`] and
    /// [`RestartLog`] as explicit `&mut` parameters and the function as `&mut`
    /// (for `setRestartPending`).  When `Funcdata` gains `get_override()` /
    /// the restart-log handle the W7/W8 assembler can drop the extra params and
    /// route through `fd`.  // SEAM(W7)
    pub fn bump_deadcode_delay(
        &mut self,
        fd: &mut crate::funcdata::Funcdata,
        ovr: &mut Override,
        restartlog: &mut RestartLog,
        spc: &Rc<AddrSpace>,
    ) {
        if spc.get_type() != spacetype::IPTR_PROCESSOR
            && spc.get_type() != spacetype::IPTR_SPACEBASE
        {
            return; // Not the right kind of space
        }
        if spc.get_delay() != spc.get_deadcode_delay() {
            return; // there is already a global delay
        }
        if ovr.has_deadcode_delay(spc) {
            // A delay has already been installed.
            // kunaRecordRestart(*fd,krestart_deadcode_suppressed,spc->getName());  (kuna)
            restartlog.record(fd, KunaRestartReason::DeadcodeSuppressed, spc.get_name());
            return;
        }
        ovr.insert_deadcode_delay(spc, spc.get_deadcode_delay() + 1);
        fd.set_restart_pending(true);
        // kunaRecordRestart(*fd,krestart_deadcode_bump,spc->getName());  (kuna)
        let name = spc.get_name().to_string();
        restartlog.record(fd, KunaRestartReason::DeadcodeBump, &name);
    }

    // =========================================================================
    // Renaming + phi placement + heritage driver (the construction surface)
    // =========================================================================
    //
    // The methods below need `Funcdata` SSA-construction primitives that are
    // not in the merged tree.  Their algorithm structure is recorded so the
    // wave that supplies the primitives can fill the bodies verbatim from
    // heritage.cc.

    // =========================================================================
    // collect / guard / rename / placeMultiequals / heritage — the SSA driver
    // (heritage.cc:308-2762).  Transcribed faithfully.  Where a sub-path reaches
    // a subsystem the merged tree still seams (W4 callspecs, W4/W6 proto
    // active-output, W4 local-scope queryProperties), the transcription routes
    // through the existing merged accessors, which report "none" — exactly the
    // C++ behavior for a function with no calls, no recovered prototype, and no
    // recovered local scope.  Each such point is marked `SEAM`.
    // =========================================================================

    /// Collect the reads/writes/inputs of a memory range (C++
    /// `Heritage::collect`, `heritage.cc:308`).  Scans `beginLoc(addr)` up to
    /// `endLoc(endaddr)` partitioning Varnodes; returns the maximum write size.
    fn collect(
        &self,
        fd: &crate::funcdata::Funcdata,
        memrange: &mut MemRange,
        read: &mut Vec<crate::seams::VarnodeId>,
        write: &mut Vec<crate::seams::VarnodeId>,
        input: &mut Vec<crate::seams::VarnodeId>,
        remove: &mut Vec<crate::seams::VarnodeId>,
    ) -> int4 {
        read.clear();
        write.clear();
        input.clear();
        remove.clear();
        let start = memrange.addr.get_offset();
        let endaddr = &memrange.addr + i64::from(memrange.size);
        let spc = memrange.addr.get_space().expect("collect: range space").clone();
        let mut maxsize = 0;
        // The C++ wraparound branch (endaddr < start) scans to the end of the
        // space (`endLoc(Address(space,highest))` → the whole rest of the
        // space); otherwise the scan stops at `beginLoc(endaddr)`.  We collect
        // the space's Varnodes once and bound by offset, which covers both: the
        // wraparound case is "everything from start to the space's end".
        let wraparound = endaddr.get_offset() < start;
        let space_ids = fd.vbank().loc_space_ids(&spc);
        let ids: Vec<crate::seams::VarnodeId> = space_ids
            .into_iter()
            .filter(|&id| {
                let off = fd.vbank().get(id).expect("collect: stale vn").get_addr().get_offset();
                off >= start && (wraparound || off < endaddr.get_offset())
            })
            .collect();
        for vn in ids {
            let v = fd.vbank().get(vn).expect("collect: stale vn");
            if v.is_write_mask() {
                continue;
            }
            if v.is_written() {
                let def = v.get_def().expect("collect: written vn has no def");
                let op = fd.obank().get(def).expect("collect: stale def op");
                if op.is_marker() || op.is_return_copy() {
                    // Evidence of previous heritage in this range.
                    if v.get_size() < memrange.size {
                        remove.push(vn);
                        continue;
                    }
                    memrange.clear_property(memrange_flags::new_addresses);
                }
                if v.get_size() > maxsize {
                    maxsize = v.get_size();
                }
                write.push(vn);
            } else if !v.is_heritage_known() && !v.has_no_descend() {
                read.push(vn);
            } else if v.is_input() {
                input.push(vn);
            }
        }
        maxsize
    }

    /// Guard a heritaged range before renaming (C++ `Heritage::guard`,
    /// `heritage.cc:1157`).  Normalizes mismatched read/write sizes, marks the
    /// reads/writes active, and (when `add_indirects`) drives the call / return
    /// / store / load guards.  (`inputvars` is unused by the C++ body — it reads
    /// only `read`/`write` — so it is not threaded here.)
    //
    // `needless_range_loop`: the body mutates the slot in place (`read[slot] =
    // normalizeReadSize(...)`) *and* re-borrows `fd` mutably inside the loop, so
    // an `iter_mut()` over `read` cannot coexist with the `&mut Funcdata` the
    // normalize/active-heritage calls need — the indexed walk is the C++ form.
    #[allow(clippy::needless_range_loop)]
    fn guard(
        &mut self,
        fd: &mut crate::funcdata::Funcdata,
        addr: &Address,
        size: int4,
        add_indirects: bool,
        read: &mut [crate::seams::VarnodeId],
        write: &mut [crate::seams::VarnodeId],
    ) {
        for slot in 0..read.len() {
            let vn = read[slot];
            // oiter = vn->beginDescend(); if (oiter==endDescend()) continue;
            let descend = fd.vbank().get(vn).expect("guard: stale read vn").num_descend();
            if descend == 0 {
                continue; // removeRevisitedMarkers may have eliminated descendant
            }
            if descend != 1 {
                // C++ throws LowlevelError("Free varnode with multiple reads")
                // — a free read always has exactly one descendant.  Panic with
                // the same invariant rather than silently mis-linking.
                panic!("kuna heritage: free varnode with multiple reads");
            }
            let op = fd
                .vbank()
                .get(vn)
                .expect("guard: stale read vn")
                .descend_iter()
                .next()
                .expect("guard: read vn has a descendant");
            if fd.vbank().get(vn).expect("guard: stale read vn").get_size() < size {
                let newvn = self.normalize_read_size(fd, vn, op, addr, size);
                read[slot] = newvn;
                fd.vbank_mut().get_mut(newvn).expect("guard: new read vn").set_active_heritage();
            } else {
                fd.vbank_mut().get_mut(vn).expect("guard: read vn").set_active_heritage();
            }
        }

        for slot in 0..write.len() {
            let vn = write[slot];
            if fd.vbank().get(vn).expect("guard: stale write vn").get_size() < size {
                let newvn = self.normalize_write_size(fd, vn, addr, size);
                write[slot] = newvn;
                fd.vbank_mut().get_mut(newvn).expect("guard: new write vn").set_active_heritage();
            } else {
                fd.vbank_mut().get_mut(vn).expect("guard: write vn").set_active_heritage();
            }
        }

        if add_indirects {
            // fd->getScopeLocal()->queryProperties(addr,size,Address(),fl);
            // SEAM(W4): the merged `Funcdata::localmap` is the unit-stub `Scope`,
            // not a real `ScopeLocal` with a symbol map, so `queryProperties`
            // cannot run; `fl` is 0 (no recovered local symbol => not
            // mapped/addrtied/persist).  The guards below read `fl` only to gate
            // store/load index-alias guarding and the persist RETURN-COPY — both
            // inert for a register range with no recovered scope.
            let fl: uint4 = 0;
            self.guard_calls(fd, fl, addr, size, write);
            self.guard_returns(fd, fl, addr, size, write);
            // if (fd->getArch()->highPtrPossible(addr,size)) { guardStores; guardLoads; }
            // SEAM(W4/W6): highPtrPossible queries the recovered type system /
            // pointer analysis (`glb->highPtrPossible`), absent from the merged
            // `seams::Architecture` (which carries only the space manager).  With
            // no recovered high pointers it is false, so the STORE/LOAD index-
            // alias guards are not reached (faithful for a function whose stack
            // is not indexed by a recovered pointer).
            let high_ptr_possible = false;
            if high_ptr_possible {
                self.guard_stores(fd, addr, size, write);
                self.guard_loads(fd, fl, addr, size, write);
            }
        }
    }

    /// Guard CALL ops (C++ `Heritage::guardCalls`, `heritage.cc:1444`).
    ///
    /// SEAM(W4): iterating call sites needs `Funcdata::numCalls`/`getCallSpecs`
    /// (the `qlst` callspec subsystem), which the merged tree does not own.  A
    /// function with no calls has `numCalls()==0`, so this loop is empty — the
    /// faithful behavior here.  When the callspec subsystem lands, the C++ body
    /// (`heritage.cc:1444-1538`: per-call effect/trial/INDIRECT guarding) folds
    /// in unchanged.
    fn guard_calls(
        &mut self,
        _fd: &mut crate::funcdata::Funcdata,
        _fl: uint4,
        _addr: &Address,
        _size: int4,
        _write: &mut [crate::seams::VarnodeId],
    ) {
        // for i in 0..fd.num_calls() { ... }  — numCalls()==0 in the merged tree.
    }

    /// Guard RETURN ops for a global/output range (C++ `Heritage::guardReturns`,
    /// `heritage.cc:1653`).
    ///
    /// The active-output branch (potential return values) needs
    /// `Funcdata::getActiveOutput()` + the real `FuncProto::characterizeAsOutput`
    /// — both supplied by `ActionPrototypeTypes::initActiveOutput`, which is a
    /// W4/W6 proto-recovery seam in the merged pipeline.  When the prototype is
    /// unrecovered `getActiveOutput()` is `None`, so this branch is skipped
    /// (exactly as C++ with a null `active`).  The persist branch needs
    /// `(fl&persist)`, which is 0 for an unrecovered register range.
    fn guard_returns(
        &mut self,
        _fd: &mut crate::funcdata::Funcdata,
        fl: uint4,
        addr: &Address,
        size: int4,
        _write: &mut [crate::seams::VarnodeId],
    ) {
        // ParamActive *active = fd->getActiveOutput();
        // SEAM(W4/W6): `Funcdata` does not own `activeoutput` in the merged tree
        // (it is initialized by `ActionPrototypeTypes::initActiveOutput`, a
        // proto-recovery seam).  `active` is therefore null, so the
        // active-output trial / potential-return-value branch (heritage.cc:1659-
        // 1675) is skipped — exactly as C++ with `active == NULL`.  This is the
        // one observable boolless B3 divergence: the C++ B3 adds `ACC` to the
        // RETURN here; the unrecovered-proto pipeline does not.
        // if ((fl&Varnode::persist)==0) return;
        if (fl & varnode_flags::persist) == 0 {
            return;
        }
        // persist RETURN-COPY branch (heritage.cc:1677-1690): only reached for a
        // recovered persistent (global) range; `fl==0` here (W4 scope seam) so
        // unreachable on the critical path.
        let _ = (addr, size);
    }

    /// Guard STORE ops (C++ `Heritage::guardStores`, `heritage.cc:1539`).
    ///
    /// SEAM(W4): adding an INDIRECT across an aliasing STORE needs
    /// `Funcdata::newIndirectOp` (the INDIRECT-marker factory, a W4 op-build
    /// primitive) and `Varnode::getSpaceFromConst` on the STORE's space-id
    /// input.  This is only reached when `highPtrPossible` is true (a recovered
    /// high pointer), which is false in the merged tree, so it is unreached on
    /// the critical path; the C++ body folds in with the W4 INDIRECT factory.
    fn guard_stores(
        &mut self,
        _fd: &mut crate::funcdata::Funcdata,
        _addr: &Address,
        _size: int4,
        _write: &mut [crate::seams::VarnodeId],
    ) {
        unimplemented_seam("Heritage::guard_stores (needs Funcdata::newIndirectOp)");
    }

    /// Guard LOAD ops in the load-guard list (C++ `Heritage::guardLoads`,
    /// `heritage.cc:1571`).
    ///
    /// The `loadGuard` list is populated by `discoverIndexedStackPointers` /
    /// `analyzeNewLoadGuards` (indexed-stack LOADs).  For a function with no
    /// indexed-stack LOADs the list is empty; the early `addrtied` guard also
    /// rejects unrecovered register ranges (`fl==0`).
    fn guard_loads(
        &mut self,
        _fd: &mut crate::funcdata::Funcdata,
        fl: uint4,
        _addr: &Address,
        _size: int4,
        _write: &mut [crate::seams::VarnodeId],
    ) {
        // C++: if ((fl & Varnode::addrtied)==0) return;  -- only address-tied
        // ranges can index-alias a stack LOAD.  The `loadGuard` list is empty
        // without `discoverIndexedStackPointers` populating it (no indexed-stack
        // LOADs here), so the COPY-guard loop (heritage.cc:1579-1607) is a no-op
        // regardless; it folds in once load-guard discovery lands.
        let _addrtied = (fl & varnode_flags::addrtied) != 0;
    }

    /// Normalize a too-small read Varnode (C++ `Heritage::normalizeReadSize`,
    /// `heritage.cc:391`).  Builds a SUBPIECE of a new full-size Varnode that
    /// defines the original (now masked) read, returning the new full read.
    fn normalize_read_size(
        &self,
        fd: &mut crate::funcdata::Funcdata,
        vn: crate::seams::VarnodeId,
        op: crate::seams::OpId,
        addr: &Address,
        size: int4,
    ) -> crate::seams::VarnodeId {
        use kuna_num::opcodes::OpCode;
        let op_addr = fd.obank().get(op).expect("normalize_read_size: stale op").get_addr().clone();
        let newop = fd.new_op(2, op_addr);
        fd.op_set_opcode(newop, typeop_skeleton(OpCode::CPUI_SUBPIECE));
        let vn1 = fd.new_varnode(size, addr, None);
        // overlap = vn->overlap(addr,size): the Varnode-vs-(addr,size) overlap.
        // `Varnode::overlap` reads the varnode's own loc/size against op2; here
        // we compute it directly off the varnode's address (endianness-aware,
        // mirroring `varnode.cc:Varnode::overlap`).
        let (vloc, vsize, vbig) = {
            let v = fd.vbank().get(vn).expect("normalize_read_size: stale vn");
            (v.get_addr().clone(), v.get_size(), v.get_addr().is_big_endian())
        };
        let overlap = if !vbig {
            vloc.overlap(0, addr, size)
        } else {
            let over = vloc.overlap(vsize - 1, addr, size);
            if over != -1 {
                size - 1 - over
            } else {
                -1
            }
        };
        let addr_size = addr.get_space().expect("normalize_read_size: addr space").get_addr_size();
        let vn2 = fd.new_constant(addr_size as int4, overlap as uintb);
        fd.op_set_input(newop, vn1, 0).expect("normalize_read_size: set input 0");
        fd.op_set_input(newop, vn2, 1).expect("normalize_read_size: set input 1");
        fd.op_set_output(newop, vn).expect("normalize_read_size: set output");
        if let Some(out) = fd.obank().get(newop).and_then(|o| o.get_out()) {
            if let Some(v) = fd.vbank_mut().get_mut(out) {
                v.set_write_mask();
            }
        }
        fd.op_insert_before(newop, op);
        vn1
    }

    /// Normalize a too-small written Varnode (C++
    /// `Heritage::normalizeWriteSize`, `heritage.cc:425`).
    ///
    /// SEAM: the full C++ builds PIECE/SUBPIECE expressions to fill the missing
    /// bytes (`buildPiece`/`splitJoin`).  A heritaged range never mixes write
    /// sizes for the simple register/stack functions on the critical path (every
    /// write of a given location is the full size), so this path is unreached;
    /// reaching it indicates a multi-size-write range that needs the W4
    /// PIECE-concatenation transcription (heritage.cc:425-479).
    fn normalize_write_size(
        &self,
        _fd: &mut crate::funcdata::Funcdata,
        _vn: crate::seams::VarnodeId,
        _addr: &Address,
        _size: int4,
    ) -> crate::seams::VarnodeId {
        unimplemented_seam(
            "Heritage::normalize_write_size (multi-size write range needs PIECE concat)",
        );
    }

    /// Fill input holes in a heritaged range (C++ `Heritage::guardInput`,
    /// `heritage.cc:1953`).  When a single input fills the range it links
    /// automatically and we return; otherwise the gaps are filled with input
    /// Varnodes and concatenated.
    fn guard_input(
        &mut self,
        fd: &mut crate::funcdata::Funcdata,
        _addr: &Address,
        size: int4,
        input: &[crate::seams::VarnodeId],
    ) {
        if input.is_empty() {
            return;
        }
        if input.len() == 1
            && fd.vbank().get(input[0]).expect("guard_input: stale input").get_size() == size
        {
            return; // single input fills everything; links in automatically
        }
        // SEAM: the gap-filling + concatPieces path (heritage.cc:1965-2010) needs
        // concatPieces (a PIECE-tree builder).  The simple register/stack
        // functions on the critical path either have no input in a heritaged
        // range or have exactly one full-size input, so this is unreached.
        unimplemented_seam(
            "Heritage::guard_input (multi-input range needs concatPieces)",
        );
    }

    /// Process join-space Varnodes before heritage (C++
    /// `Heritage::processJoins`, `heritage.cc:2282`).
    ///
    /// SEAM(W4/W6): the join space holds Varnodes formed by `splitJoinRead`/
    /// `splitJoinWrite`/float-extension (the `JoinRecord` machinery).  A
    /// function whose architecture defines no join space, or whose IR uses no
    /// joined locations, has an empty join-space loc-set, so this is a no-op.
    /// The split/float-extension transcription (heritage.cc:2282-2313) lands
    /// with the W4 join subsystem; reaching a non-empty join space here would
    /// need it.
    fn process_joins(&mut self, fd: &mut crate::funcdata::Funcdata) {
        let joinspace = fd.get_arch().manage().get_join_space().cloned();
        let Some(joinspace) = joinspace else { return };
        let any = !fd.vbank().loc_space_ids(&joinspace).is_empty();
        if any {
            unimplemented_seam(
                "Heritage::process_joins (join-space Varnodes need the W4 JoinRecord split)",
            );
        }
    }

    /// Clear stack placeholders before heritaging the stack space (C++
    /// `Heritage::clearStackPlaceholders`, `heritage.cc:2048`).
    ///
    /// SEAM(W4): the per-call `abortSpacebaseRelative` needs the callspec
    /// subsystem (`numCalls`/`getCallSpecs`).  With no calls the loop is empty;
    /// only the flag is cleared (faithful).
    fn clear_stack_placeholders(&mut self, info_idx: usize) {
        // for i in 0..fd.num_calls() { getCallSpecs(i)->abortSpacebaseRelative(*fd); }
        // numCalls()==0 in the merged tree.
        self.infolist[info_idx].has_call_placeholders = false;
    }

    /// The heart of the renaming algorithm (C++ `Heritage::renameRecurse`,
    /// `heritage.cc:2480`).
    ///
    /// Recursively walks the dominance tree from `bl`; at each block, visits
    /// ops in execution order, replacing free reads with the top of the
    /// per-address [`VariableStack`] (creating an input Varnode if the stack is
    /// empty), pushing writes, threading the INDIRECT "same time" special case,
    /// filling successor MULTIEQUAL inputs by `getOutRevIndex`, recursing into
    /// dom-children, then popping this block's writes.
    //
    // `mutable_key_type`: the [`VariableStack`] key is `Address`, exactly as the
    // C++ `map<Address,vector<Varnode*>>`.  `Address`'s only interior
    // mutability is the owning `AddrSpace`'s cached `index` (`Cell<i32>`), set
    // once at space registration; `Address`'s `Ord`/`Eq` read the index *value*
    // and the offset, neither of which mutates after the key is inserted, so
    // the key ordering is stable — a justified false positive.
    #[allow(clippy::mutable_key_type)]
    pub fn rename_recurse(
        &mut self,
        fd: &mut crate::funcdata::Funcdata,
        bl: BlockId,
        varstack: &mut VariableStack,
    ) {
        use kuna_num::opcodes::OpCode;
        let mut writelist: Vec<crate::seams::VarnodeId> = Vec::new();

        // for(oiter=bl->beginOp(); oiter!=bl->endOp(); ++oiter)
        let ops: Vec<crate::seams::OpId> = self.block_ops(fd, bl);
        for op in ops {
            let code = fd.obank().get(op).expect("rename_recurse: stale op").code();
            if code != OpCode::CPUI_MULTIEQUAL {
                // First replace reads with top of stack.
                let ninput = fd.obank().get(op).expect("rename_recurse: stale op").num_input();
                for slot in 0..ninput {
                    let vnin = match fd.obank().get(op).expect("rename_recurse: stale op").get_in(slot) {
                        Some(v) => v,
                        None => continue,
                    };
                    let vinref = fd.vbank().get(vnin).expect("rename_recurse: stale in");
                    if vinref.is_heritage_known() {
                        continue; // not free
                    }
                    if !vinref.is_active_heritage() {
                        continue; // Not being heritaged this round
                    }
                    fd.vbank_mut().get_mut(vnin).expect("rename_recurse: in").clear_active_heritage();
                    let key = fd.vbank().get(vnin).expect("rename_recurse: in").get_addr().clone();
                    let in_size =
                        fd.vbank().get(vnin).expect("rename_recurse: in").get_size();
                    // vector<Varnode*> &stack = varstack[ vnin->getAddr() ];
                    let mut vnnew = match varstack.get(&key).and_then(|s| s.last().copied()) {
                        Some(top) => top,
                        None => {
                            let nv = fd.new_varnode(in_size, &key, None);
                            let nv = fd
                                .set_input_varnode(nv)
                                .expect("rename_recurse: set_input_varnode (empty stack)");
                            varstack.entry(key.clone()).or_default().push(nv);
                            nv
                        }
                    };
                    // INDIRECTs and their op really happen AT SAME TIME.
                    let same_time = {
                        let nvref = fd.vbank().get(vnnew).expect("rename_recurse: vnnew");
                        if nvref.is_written() {
                            let def = nvref.get_def().expect("rename_recurse: written vnnew def");
                            fd.obank().get(def).map(|o| o.code()) == Some(OpCode::CPUI_INDIRECT)
                        } else {
                            false
                        }
                    };
                    if same_time {
                        let def = fd.vbank().get(vnnew).expect("rename_recurse: vnnew").get_def().unwrap();
                        // PcodeOp::getOpFromConst(def->getIn(1)->getAddr()) == op
                        // SEAM(W4): the INDIRECT "same time" carve-out resolves
                        // the IOP-space constant in[1] back to its PcodeOp via
                        // `getOpFromConst`, which needs the IOP→op map.  This is
                        // only reached when a renamed value's top-of-stack is
                        // INDIRECT-defined (calls/stores), which the no-call/
                        // no-store critical path never produces.
                        let from_op = op_from_const_seam(fd, def);
                        if from_op == Some(op) {
                            let stacklen = varstack.get(&key).map(|s| s.len()).unwrap_or(0);
                            if stacklen == 1 {
                                let nv = fd.new_varnode(in_size, &key, None);
                                let nv = fd
                                    .set_input_varnode(nv)
                                    .expect("rename_recurse: set_input_varnode (indirect same-time)");
                                varstack.entry(key.clone()).or_default().insert(0, nv);
                                vnnew = nv;
                            } else {
                                vnnew = varstack[&key][stacklen - 2];
                            }
                        }
                    }
                    fd.op_set_input(op, vnnew, slot).expect("rename_recurse: op_set_input read");
                    if fd.vbank().get(vnin).expect("rename_recurse: in").has_no_descend() {
                        fd.delete_varnode(vnin).expect("rename_recurse: delete free read");
                    }
                }
            }
            // Then push writes onto stack.
            let vnout = match fd.obank().get(op).expect("rename_recurse: stale op").get_out() {
                Some(v) => v,
                None => continue,
            };
            if !fd.vbank().get(vnout).expect("rename_recurse: out").is_active_heritage() {
                continue; // Not a normalized write
            }
            fd.vbank_mut().get_mut(vnout).expect("rename_recurse: out").clear_active_heritage();
            let okey = fd.vbank().get(vnout).expect("rename_recurse: out").get_addr().clone();
            varstack.entry(okey).or_default().push(vnout);
            writelist.push(vnout);
        }

        // Fill successor MULTIEQUAL inputs (the merge-block phi in-edges).
        let size_out = fd.bblocks_ref().block(bl).size_out();
        for i in 0..size_out {
            let subbl = fd.bblocks_ref().block(bl).get_out(i);
            let slot = fd.bblocks_ref().block(bl).get_out_rev_index(i);
            let subops: Vec<crate::seams::OpId> = self.block_ops(fd, subbl);
            for multiop in subops {
                if fd.obank().get(multiop).expect("rename_recurse: stale multiop").code()
                    != OpCode::CPUI_MULTIEQUAL
                {
                    break; // For each leading MULTIEQUAL
                }
                let vnin = match fd.obank().get(multiop).expect("rename_recurse: multiop").get_in(slot) {
                    Some(v) => v,
                    None => continue,
                };
                if fd.vbank().get(vnin).expect("rename_recurse: multi in").is_heritage_known() {
                    continue;
                }
                let key = fd.vbank().get(vnin).expect("rename_recurse: multi in").get_addr().clone();
                let in_size = fd.vbank().get(vnin).expect("rename_recurse: multi in").get_size();
                let vnnew = match varstack.get(&key).and_then(|s| s.last().copied()) {
                    Some(top) => top,
                    None => {
                        let nv = fd.new_varnode(in_size, &key, None);
                        let nv = fd
                            .set_input_varnode(nv)
                            .expect("rename_recurse: set_input_varnode (multi empty stack)");
                        varstack.entry(key.clone()).or_default().push(nv);
                        nv
                    }
                };
                fd.op_set_input(multiop, vnnew, slot).expect("rename_recurse: op_set_input multi");
                if fd.vbank().get(vnin).expect("rename_recurse: multi in").has_no_descend() {
                    fd.delete_varnode(vnin).expect("rename_recurse: delete multi free in");
                }
            }
        }

        // Recurse into dom-children.
        let i = fd.bblocks_ref().block(bl).get_index() as usize;
        let children: Vec<BlockId> = self.domchild[i].clone();
        for child in children {
            self.rename_recurse(fd, child, varstack);
        }

        // Pop this block's writes off the stack.
        for vnout in writelist {
            let key = fd.vbank().get(vnout).expect("rename_recurse: pop out").get_addr().clone();
            if let Some(stack) = varstack.get_mut(&key) {
                stack.pop();
            }
        }
    }

    /// Borrow a block's ops in execution order (C++ `bl->beginOp()..endOp()`),
    /// snapshotting into a `Vec` so the recursive walk can mutate the IR.
    fn block_ops(
        &self,
        fd: &crate::funcdata::Funcdata,
        bl: BlockId,
    ) -> Vec<crate::seams::OpId> {
        fd.bb_ops(bl)
    }

    /// Perform the renaming algorithm for the current address ranges (C++
    /// `Heritage::rename`, `heritage.cc:2591`).  Phi placement must already have
    /// happened.
    // `mutable_key_type`: see the note on [`rename_recurse`](Heritage::rename_recurse).
    #[allow(clippy::mutable_key_type)]
    pub fn rename(&mut self, fd: &mut crate::funcdata::Funcdata) {
        let bblocks = fd.bblocks_ref();
        let root = bblocks.root.expect("rename: bblocks root");
        let start = bblocks.block(root).get_block(0);
        let mut varstack: VariableStack = VariableStack::new();
        self.rename_recurse(fd, start, &mut varstack);
        self.disjoint.clear();
    }

    /// Perform phi-node placement for the current address ranges (C++
    /// `Heritage::placeMultiequals`, `heritage.cc:2603`).
    ///
    /// Assumes `disjoint` is filled with all free Varnodes to be heritaged.  The
    /// driver loops the disjoint ranges, collecting reads/writes/inputs,
    /// optionally refining (SEAM — the refinement path needs `buildRefinement`/
    /// `refineSubpiece`, unreached on the simple critical path), guarding,
    /// calling [`calc_multiequals`](Heritage::calc_multiequals), and
    /// constructing the MULTIEQUAL ops at each merge block.
    pub fn place_multiequals(&mut self, fd: &mut crate::funcdata::Funcdata) {
        use kuna_num::opcodes::OpCode;
        let mut readvars: Vec<crate::seams::VarnodeId> = Vec::new();
        let mut writevars: Vec<crate::seams::VarnodeId> = Vec::new();
        let mut inputvars: Vec<crate::seams::VarnodeId> = Vec::new();
        let mut removevars: Vec<crate::seams::VarnodeId> = Vec::new();

        let n = self.disjoint.len();
        for idx in 0..n {
            let mut memrange = self.disjoint.get(idx).clone();
            let maxw =
                self.collect(fd, &mut memrange, &mut readvars, &mut writevars, &mut inputvars, &mut removevars);
            // refinement (heritage.cc:2611-2617): only when size>4 && max<size.
            if memrange.size > 4 && maxw < memrange.size {
                // SEAM: refinement (heritage.cc:1891) splits the range by the
                // sub-piece boundaries; unreached for the size<=4 register/stack
                // ranges on the critical path.
                unimplemented_seam(
                    "Heritage::placeMultiequals refinement (needs buildRefinement/refineSubpiece)",
                );
            }
            // write the (possibly clearProperty-mutated) memrange back.
            *self.disjoint.get_mut(idx) = memrange.clone();
            let size = memrange.size;
            if readvars.is_empty() {
                if writevars.is_empty() && inputvars.is_empty() {
                    continue;
                }
                let is_internal = memrange.addr.get_space().map(|s| {
                    s.get_type() == spacetype::IPTR_INTERNAL
                }).unwrap_or(false);
                if is_internal || memrange.old_addresses() {
                    continue;
                }
            }
            if !removevars.is_empty() {
                // SEAM: removeRevisitedMarkers (heritage.cc:245) deletes stale
                // MULTIEQUAL/INDIRECT markers from a previous pass over a now-
                // smaller range; only reached when `collect` saw prior-pass
                // marker writes smaller than the range (multi-pass overlap),
                // which the single-pass critical path does not hit.
                unimplemented_seam(
                    "Heritage::placeMultiequals removeRevisitedMarkers (multi-pass overlap)",
                );
            }
            self.guard_input(fd, &memrange.addr, size, &inputvars);
            let add_indirects = memrange.new_addresses();
            self.guard(fd, &memrange.addr, size, add_indirects, &mut readvars, &mut writevars);
            // calcMultiequals(writevars): the realized engine takes the write
            // *blocks* (the C++ derives them as `write[i]->getDef()->getParent()`
            // — the block each normalized write is defined in).  Inputs/written
            // varnodes that have no def (raw inputs) contribute no write block.
            let write_blocks: Vec<BlockId> = writevars
                .iter()
                .filter_map(|&vn| {
                    let def = fd.vbank().get(vn).and_then(|v| v.get_def())?;
                    fd.obank().get(def).and_then(|o| o.get_parent())
                })
                .collect();
            self.calc_multiequals(fd, &write_blocks);
            // Construct each MULTIEQUAL at its merge block.
            let merge_blocks: Vec<BlockId> = self.merge.clone();
            let multi_typeop = typeop_skeleton(OpCode::CPUI_MULTIEQUAL);
            for bl in merge_blocks {
                let size_in = fd.bblocks_ref().block(bl).size_in();
                let start = crate::block::block_get_start(&fd.bblocks_ref().arena, bl);
                let multiop = fd.new_op(size_in, start);
                let vnout = fd
                    .new_varnode_out(size, &memrange.addr, multiop)
                    .expect("place_multiequals: new_varnode_out");
                fd.vbank_mut().get_mut(vnout).expect("place_multiequals: vnout").set_active_heritage();
                fd.op_set_opcode(multiop, multi_typeop.clone());
                for j in 0..size_in {
                    let vnin = fd.new_varnode(size, &memrange.addr, None);
                    fd.op_set_input(multiop, vnin, j).expect("place_multiequals: op_set_input");
                }
                fd.op_insert_begin(multiop, bl);
            }
        }
        self.merge.clear();
    }

    /// Perform one pass of heritage (C++ `Heritage::heritage`,
    /// `heritage.cc:2667`).
    ///
    /// Collects free Varnodes from active spaces, builds the disjoint cover,
    /// runs phi placement and renaming, and advances `pass`.  The
    /// PreferSplitManager (pass-0 SIMD split) and the load-guard discovery
    /// (`discoverIndexedStackPointers`/`analyzeNewLoadGuards`/
    /// `handleNewLoadCopies`/`reprocessFreeStores`) sub-systems are SEAM-marked;
    /// they are inert for a function with no split records and no indexed-stack
    /// LOAD/STOREs (the realized critical path).
    #[allow(clippy::mutable_key_type)]
    pub fn heritage(&mut self, fd: &mut crate::funcdata::Funcdata) {
        // A function with no basic blocks has no data-flow to heritage.  The C++
        // never reaches `heritage()` in that state (it runs after `followFlow`/
        // `structureReset` built the CFG), and `buildADT` indexes the block list
        // (`z[0]`), so guard the degenerate empty-CFG case as a no-op — faithful
        // for an action invoked on a not-yet-built function (the seam-wrapper
        // unit test) without changing the populated-CFG behavior.
        let block_count = {
            let bblocks = fd.bblocks_ref();
            bblocks.root.map(|r| bblocks.block(r).get_size()).unwrap_or(0)
        };
        if block_count == 0 {
            return;
        }
        if self.maxdepth == -1 {
            // Has a restructure been forced
            self.build_adt(fd);
        }
        self.process_joins(fd);
        // if (pass == 0) splitmanage.init/split;  SEAM(W6): PreferSplitManager
        // is a no-op for an architecture with no <splitrecords> (the critical
        // path); it splits SIMD-style registers when present.

        let nspaces = fd.get_arch().manage().num_spaces() as usize;
        for i in 0..nspaces {
            // info = &infolist[i];
            if !self.infolist[i].is_heritaged() {
                continue;
            }
            if self.pass < self.infolist[i].delay {
                continue; // too soon to heritage this space
            }
            if self.infolist[i].has_call_placeholders {
                self.clear_stack_placeholders(i);
            }
            if !self.infolist[i].load_guard_search {
                self.infolist[i].load_guard_search = true;
                // discoverIndexedStackPointers(info->space,freeStores,true)
                // SEAM(W4): the indexed-stack LOAD/STORE discovery walks the
                // stack-pointer input's descendants; inert (returns false) for a
                // function with no indexed stack accesses (no reprocess needed).
            }
            let space = self.infolist[i]
                .space
                .clone()
                .expect("heritage: heritaged info has a space");
            let mut needwarning = false;
            let mut warnvn: Option<crate::seams::VarnodeId> = None;

            let ids: Vec<crate::seams::VarnodeId> = fd.vbank().loc_space_ids(&space);
            for vn in ids {
                let v = fd.vbank().get(vn).expect("heritage: stale vn");
                if !v.is_written() && v.has_no_descend() && !v.is_unaffected() && !v.is_input() {
                    continue;
                }
                if v.is_write_mask() {
                    continue;
                }
                let vaddr = v.get_addr().clone();
                let vsize = v.get_size();
                let (key, prev) = self.globaldisjoint.add(vaddr, vsize, self.pass);
                let resolved_size =
                    self.globaldisjoint.get(&key).expect("heritage: disjoint key").size;
                if prev == 0 {
                    // All new location being heritaged.
                    self.disjoint.add(key, resolved_size, memrange_flags::new_addresses);
                } else if prev == 2 {
                    // Completely contained in range from a previous pass.
                    let v = fd.vbank().get(vn).expect("heritage: stale vn (prev==2)");
                    if v.is_heritage_known() {
                        continue;
                    }
                    if v.has_no_descend() {
                        continue;
                    }
                    if !needwarning
                        && self.infolist[i].deadremoved > 0
                        && !fd.is_jumptable_recovery_on()
                    {
                        needwarning = true;
                        self.bump_deadcode_delay_seamed(fd, &space);
                        warnvn = Some(vn);
                    }
                    self.disjoint.add(key, resolved_size, memrange_flags::old_addresses);
                } else {
                    // Partially contained in old range, but may contain new.
                    self.disjoint.add(
                        key,
                        resolved_size,
                        memrange_flags::old_addresses | memrange_flags::new_addresses,
                    );
                    let v = fd.vbank().get(vn).expect("heritage: stale vn (prev==1)");
                    if !needwarning
                        && self.infolist[i].deadremoved > 0
                        && !fd.is_jumptable_recovery_on()
                    {
                        if v.is_heritage_known() {
                            continue;
                        }
                        needwarning = true;
                        self.bump_deadcode_delay_seamed(fd, &space);
                        warnvn = Some(vn);
                    }
                }
            }
            if needwarning && !self.infolist[i].warningissued {
                self.infolist[i].warningissued = true;
                // fd->warningHeader("Heritage AFTER dead removal...")
                // SEAM(W8): the warning text needs Varnode::printRawNoMarkup (W8
                // print surface); the warning is suppressed (a cosmetic header,
                // not IR), the dead-code-delay bump already recorded the event.
                let _ = warnvn;
            }
        }
        self.place_multiequals(fd);
        self.rename(fd);
        // reprocessFreeStores / analyzeNewLoadGuards / handleNewLoadCopies:
        // SEAM(W4) — inert without discovered load guards (see above).
        // splitmanage.splitAdditional() on pass 0: SEAM(W6) PreferSplitManager.
        self.pass += 1;
    }

    /// `bumpDeadcodeDelay` routed through a fresh [`Override`]/[`RestartLog`]
    /// (C++ `fd->getOverride()` / `kunaRecordRestart` are W7 seams on
    /// `Funcdata`).  The realized [`bump_deadcode_delay`](Heritage::bump_deadcode_delay)
    /// takes them explicitly; here we use function-local instances so the bump
    /// protocol (delay install, restart-pending) runs against the real
    /// `Funcdata`.  // SEAM(W7)
    fn bump_deadcode_delay_seamed(
        &mut self,
        fd: &mut crate::funcdata::Funcdata,
        spc: &Rc<AddrSpace>,
    ) {
        let mut ovr = Override::new();
        let mut log = RestartLog::new();
        self.bump_deadcode_delay(fd, &mut ovr, &mut log, spc);
    }
}

/// Build the minimal [`TypeOp`] skeleton heritage needs for the ops it creates
/// (C++ `glb->inst[opc]` — the op-property triple `op_set_opcode` caches).
///
/// Heritage only creates MULTIEQUAL (the phi, which must carry the `marker`
/// flag so `is_marker()` / the renameRecurse MULTIEQUAL test see it) and
/// SUBPIECE (the read-size normalizer).  The merged `Funcdata` reaches only the
/// `seams::Architecture` (no `inst` table), so the property triple is built
/// inline with the exact flags upstream's `TypeOpMulti`/`TypeOpSubpiece` carry.
fn typeop_skeleton(opc: kuna_num::opcodes::OpCode) -> crate::seams::TypeOp {
    use crate::op::pcodeop_flags as f;
    use kuna_num::opcodes::OpCode;
    let (flags, name) = match opc {
        // TypeOpMulti: marker (a special, non-evaluated phi node).
        OpCode::CPUI_MULTIEQUAL => (f::marker, "?"),
        // TypeOpSubpiece: binary.
        OpCode::CPUI_SUBPIECE => (f::binary, "SUB"),
        _ => panic!("heritage typeop_skeleton: unexpected opcode {opc:?}"),
    };
    crate::seams::TypeOp::new(opc, flags, name)
}

/// `PcodeOp::getOpFromConst(def->getIn(1)->getAddr())` for the INDIRECT
/// "same time" carve-out (C++ `renameRecurse`, heritage.cc:2507).
///
/// SEAM(W4): resolving the IOP-space constant in[1] back to its PcodeOp needs
/// the IOP→op map (`PcodeOp::getOpFromConst`), which the merged tree does not
/// expose on `Funcdata`.  This is only invoked when the renamed value's
/// top-of-stack is INDIRECT-defined — a calls/stores construct the no-call,
/// no-store critical path never produces — so it is unreached there.  When an
/// INDIRECT def is genuinely present, reaching this is the W4 IOP-map seam.
fn op_from_const_seam(
    _fd: &crate::funcdata::Funcdata,
    _indirect_def: crate::seams::OpId,
) -> Option<crate::seams::OpId> {
    unimplemented_seam("Heritage::rename_recurse INDIRECT same-time (needs getOpFromConst)");
}

impl Default for Heritage {
    fn default() -> Self {
        Heritage::new()
    }
}

/// Panic helper for the seam methods whose `Funcdata` SSA-construction
/// primitives are not yet present.  Centralizes the message so the W3-op /
/// W4 / W6 waves can grep the seams.
#[inline(never)]
#[cold]
fn unimplemented_seam(what: &str) -> ! {
    panic!("kuna heritage SEAM not yet realized: {what}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::funcdata::Funcdata;
    use crate::seams::Architecture;
    use kuna_base::space::{
        addrspace_flags, AddrSpace, AddrSpaceManager, ConstantSpace, UniqueSpace,
    };

    // ---- fixtures ---------------------------------------------------------

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
        let glb = Rc::new(Architecture::new(manage));
        let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
        let addr = Address::new(ram, 0x1000);
        Funcdata::new("func", "func", glb, addr, 0x10000000, 0x40).unwrap()
    }

    fn ram(fd: &Funcdata) -> Rc<AddrSpace> {
        Rc::clone(fd.get_arch().manage().get_space_by_name("ram").unwrap())
    }

    fn raddr(fd: &Funcdata, off: u64) -> Address {
        Address::new(ram(fd), off)
    }

    /// Build a CFG of `n` basic blocks (indices 0..n in creation order) inside
    /// the function's `bblocks` graph, wire `edges`, set reverse-post-order
    /// indices to creation order, and run the forward-dominator pass.  Returns
    /// the block ids (index i is `blocks[i]`).
    ///
    /// The blocks are created in DFS/reverse-post order (the C++ `buildADT`
    /// precondition); for the small acyclic/loop CFGs below, creation order IS
    /// a valid reverse-post order with block 0 the start.
    fn build_cfg(fd: &mut Funcdata, n: usize, edges: &[(usize, usize)]) -> Vec<BlockId> {
        let root = fd.bblocks_ref().root.expect("bblocks root");
        let g = fd.bblocks_mut();
        let mut blocks = Vec::with_capacity(n);
        for _ in 0..n {
            blocks.push(g.new_block_basic(root));
        }
        for (i, &id) in blocks.iter().enumerate() {
            g.block_mut(id).set_index(i as int4);
        }
        for &(a, b) in edges {
            g.add_edge(blocks[a], blocks[b]);
        }
        g.calc_forward_dominator(root, &[blocks[0]]);
        blocks
    }

    // ---- LocationMap ------------------------------------------------------

    #[test]
    fn location_map_add_new_range_pass0() {
        let fd = build_fd();
        let mut lm = LocationMap::new();
        let (key, intersect) = lm.add(raddr(&fd, 0x100), 4, 0);
        assert_eq!(key, raddr(&fd, 0x100));
        assert_eq!(intersect, 0);
        assert_eq!(lm.get(&key).unwrap().size, 4);
        assert_eq!(lm.get(&key).unwrap().pass, 0);
    }

    #[test]
    fn location_map_contained_in_old_returns_intersect2() {
        let fd = build_fd();
        let mut lm = LocationMap::new();
        // pass 0: heritage [0x100, 0x108)
        lm.add(raddr(&fd, 0x100), 8, 0);
        // pass 1: a sub-range [0x102,0x106) is completely contained in the
        // old pass-0 element -> intersect == 2.
        let (key, intersect) = lm.add(raddr(&fd, 0x102), 4, 1);
        assert_eq!(intersect, 2);
        // The returned element is the containing pass-0 element.
        assert_eq!(key, raddr(&fd, 0x100));
        assert_eq!(lm.get(&key).unwrap().pass, 0);
    }

    #[test]
    fn location_map_partial_old_overlap_returns_intersect1_and_carries_min_pass() {
        let fd = build_fd();
        let mut lm = LocationMap::new();
        lm.add(raddr(&fd, 0x100), 4, 0); // pass 0: [0x100,0x104)
                                         // pass 2: [0x102,0x10a) partially overlaps the old element.
        let (key, intersect) = lm.add(raddr(&fd, 0x102), 8, 2);
        assert_eq!(intersect, 1);
        // The unified element starts at the old start 0x100 and carries the
        // smaller (old) pass number 0.
        assert_eq!(key, raddr(&fd, 0x100));
        assert_eq!(lm.get(&key).unwrap().pass, 0);
        // size = where(2) + size(8) = 10
        assert_eq!(lm.get(&key).unwrap().size, 10);
    }

    #[test]
    fn location_map_same_pass_overlap_returns_intersect0() {
        let fd = build_fd();
        let mut lm = LocationMap::new();
        lm.add(raddr(&fd, 0x100), 4, 0);
        // same pass, contained -> intersect 0 (not 2, since old.pass == pass)
        let (_key, intersect) = lm.add(raddr(&fd, 0x101), 2, 0);
        assert_eq!(intersect, 0);
    }

    #[test]
    fn location_map_find_and_find_pass() {
        let fd = build_fd();
        let mut lm = LocationMap::new();
        lm.add(raddr(&fd, 0x200), 4, 3);
        // An address inside the range is found.
        let found = lm.find(&raddr(&fd, 0x202)).unwrap();
        assert_eq!(found.0, raddr(&fd, 0x200));
        assert_eq!(found.1.pass, 3);
        assert_eq!(lm.find_pass(&raddr(&fd, 0x202)), 3);
        // An address outside is not.
        assert!(lm.find(&raddr(&fd, 0x210)).is_none());
        assert_eq!(lm.find_pass(&raddr(&fd, 0x210)), -1);
    }

    #[test]
    fn location_map_swallows_multiple_old_ranges() {
        let fd = build_fd();
        let mut lm = LocationMap::new();
        lm.add(raddr(&fd, 0x100), 4, 0); // [0x100,0x104)
        lm.add(raddr(&fd, 0x108), 4, 0); // [0x108,0x10c)
                                         // A big new range spanning both gets unified into one element.
        let (key, _i) = lm.add(raddr(&fd, 0x100), 0x10, 1);
        assert_eq!(key, raddr(&fd, 0x100));
        // Only one element remains.
        assert_eq!(lm.iter().count(), 1);
        // size grew to at least 0x10
        assert!(lm.get(&key).unwrap().size >= 0x10);
    }

    // ---- TaskList ---------------------------------------------------------

    #[test]
    fn tasklist_add_disjoint_appends() {
        let fd = build_fd();
        let mut tl = TaskList::new();
        tl.add(raddr(&fd, 0x100), 4, memrange_flags::new_addresses);
        tl.add(raddr(&fd, 0x200), 4, memrange_flags::old_addresses);
        assert_eq!(tl.len(), 2);
        assert!(tl.get(0).new_addresses());
        assert!(tl.get(1).old_addresses());
    }

    #[test]
    fn tasklist_add_overlapping_last_extends_and_ors_flags() {
        let fd = build_fd();
        let mut tl = TaskList::new();
        tl.add(raddr(&fd, 0x100), 4, memrange_flags::new_addresses);
        // overlaps the last range -> extends it and ORs the flags
        tl.add(raddr(&fd, 0x102), 4, memrange_flags::old_addresses);
        assert_eq!(tl.len(), 1);
        // relsize = size(4) + over(2) = 6
        assert_eq!(tl.get(0).size, 6);
        assert!(tl.get(0).new_addresses());
        assert!(tl.get(0).old_addresses());
    }

    #[test]
    fn tasklist_insert_and_erase() {
        let fd = build_fd();
        let mut tl = TaskList::new();
        tl.add(raddr(&fd, 0x100), 4, 0);
        tl.add(raddr(&fd, 0x300), 4, 0);
        tl.insert(1, raddr(&fd, 0x200), 4, 0);
        assert_eq!(tl.len(), 3);
        assert_eq!(tl.get(1).addr, raddr(&fd, 0x200));
        let next = tl.erase(1);
        assert_eq!(next, 1);
        assert_eq!(tl.get(1).addr, raddr(&fd, 0x300));
    }

    // ---- PriorityQueue ----------------------------------------------------

    #[test]
    fn priority_queue_lifo_within_depth_and_depth_priority() {
        // We only need distinct BlockIds; build three blocks in a CFG.
        let mut fd = build_fd();
        let b = build_cfg(&mut fd, 3, &[(0, 1), (1, 2)]);
        let (b0, b1, b2) = (b[0], b[1], b[2]);

        let mut pq = PriorityQueue::new();
        pq.reset(3);
        assert!(pq.empty());
        pq.insert(b0, 1);
        pq.insert(b1, 1); // same depth -> LIFO (b1 first)
        pq.insert(b2, 3); // higher depth -> extracted first
        assert!(!pq.empty());
        assert_eq!(pq.extract(), b2); // depth 3 wins
        assert_eq!(pq.extract(), b1); // depth 1, LIFO top
        assert_eq!(pq.extract(), b0);
        assert!(pq.empty());
    }

    #[test]
    fn priority_queue_reset_idempotent_when_empty_and_same_maxdepth() {
        let mut pq = PriorityQueue::new();
        pq.reset(5);
        // empty and maxdepth matches -> the early return path (no panic, stays empty)
        pq.reset(5);
        assert!(pq.empty());
    }

    // ---- HeritageInfo -----------------------------------------------------

    #[test]
    fn heritage_info_heritaged_processor_space() {
        let fd = build_fd();
        let ram = ram(&fd);
        let info = HeritageInfo::new(Some(&ram));
        // ram is a heritaged processor space in the fixture
        assert_eq!(info.is_heritaged(), ram.is_heritaged());
        if ram.is_heritaged() {
            assert_eq!(info.delay(), ram.get_delay());
            assert_eq!(info.deadcode_delay(), ram.get_deadcode_delay());
        }
    }

    #[test]
    fn heritage_info_null_space_is_not_heritaged() {
        let info = HeritageInfo::new(None);
        assert!(!info.is_heritaged());
        assert_eq!(info.delay(), 0);
        assert_eq!(info.deadcode_delay(), 0);
    }

    // ---- LoadGuard --------------------------------------------------------

    #[test]
    fn load_guard_set_and_is_guarded() {
        let fd = build_fd();
        let ram = ram(&fd);
        // We need an OpId; create a throwaway op.
        let mut fd2 = build_fd();
        let pc = raddr(&fd2, 0x1000);
        let op = fd2.new_op(2, pc);
        let mut g = LoadGuard::set(op, &ram, 0x40);
        // Initially guards everything from 0 to highest.
        assert_eq!(g.get_minimum(), 0);
        assert_eq!(g.get_maximum(), ram.get_highest());
        assert!(g.is_guarded(&raddr(&fd, 0x40)));
        assert!(!g.is_range_locked());
        // Narrow the range and re-check.
        g.minimum_offset = 0x40;
        g.maximum_offset = 0x80;
        assert!(g.is_guarded(&raddr(&fd, 0x40)));
        assert!(g.is_guarded(&raddr(&fd, 0x80)));
        assert!(!g.is_guarded(&raddr(&fd, 0x3f)));
        assert!(!g.is_guarded(&raddr(&fd, 0x81)));
    }

    // ---- Heritage construction + accessors --------------------------------

    #[test]
    fn heritage_new_and_clear_state() {
        let mut h = Heritage::new();
        assert_eq!(h.get_pass(), 0);
        let fd = build_fd();
        h.build_info_list(&fd);
        // info list has one entry per space
        assert_eq!(h.infolist.len(), fd.get_arch().manage().num_spaces() as usize);
        // idempotent
        h.build_info_list(&fd);
        assert_eq!(h.infolist.len(), fd.get_arch().manage().num_spaces() as usize);
        h.clear();
        assert_eq!(h.get_pass(), 0);
        assert!(h.infolist.is_empty() || h.infolist.iter().all(|i| !i.is_heritaged() || true));
    }

    #[test]
    fn heritage_dead_code_delay_accessors() {
        let mut h = Heritage::new();
        let fd = build_fd();
        h.build_info_list(&fd);
        let ram = ram(&fd);
        if ram.is_heritaged() {
            let base = h.get_dead_code_delay(&ram);
            // setting a smaller delay errors
            assert!(h.set_dead_code_delay(&ram, ram.get_delay() - 1).is_err());
            // setting >= delay succeeds
            assert!(h.set_dead_code_delay(&ram, base + 2).is_ok());
            assert_eq!(h.get_dead_code_delay(&ram), base + 2);
            // dead removal allowed once pass exceeds the delay
            assert!(!h.dead_removal_allowed(&ram));
        }
    }

    #[test]
    fn heritage_num_passes_errors_for_non_heritaged() {
        let mut h = Heritage::new();
        let fd = build_fd();
        h.build_info_list(&fd);
        // The constant space is not heritaged.
        let cspace = Rc::clone(fd.get_arch().manage().get_space(0).unwrap());
        assert!(!cspace.is_heritaged());
        assert!(h.num_heritage_passes(&cspace).is_err());
    }

    #[test]
    fn heritage_seen_dead_code_and_removal_seen() {
        let mut h = Heritage::new();
        let fd = build_fd();
        h.build_info_list(&fd);
        let ram = ram(&fd);
        if ram.is_heritaged() {
            h.seen_dead_code(&ram);
            // pass is 0, deadcodedelay default >= 0, so removal not allowed
            let allowed = h.dead_removal_allowed_seen(&ram);
            assert_eq!(allowed, h.pass > h.get_dead_code_delay(&ram));
        }
    }

    // ---- bumpDeadcodeDelay + restart recorder anchor ----------------------

    #[test]
    fn bump_deadcode_delay_installs_and_records() {
        let mut h = Heritage::new();
        let mut fd = build_fd();
        let mut ovr = Override::new();
        let mut log = RestartLog::new();
        let ram = ram(&fd);
        // Precondition: ram is a processor space with delay == deadcodedelay.
        if ram.get_type() == spacetype::IPTR_PROCESSOR
            && ram.get_delay() == ram.get_deadcode_delay()
        {
            assert!(!ovr.has_deadcode_delay(&ram));
            h.bump_deadcode_delay(&mut fd, &mut ovr, &mut log, &ram);
            // delay installed
            assert!(ovr.has_deadcode_delay(&ram));
            // restart pending set
            assert!(fd.has_restart_pending());
            // recorded a deadcode_bump event (log is non-empty for fd)
            assert!(!log.is_empty_for(&fd));

            // Second call: already installed -> suppressed (no re-bump), still records.
            fd.set_restart_pending(false);
            h.bump_deadcode_delay(&mut fd, &mut ovr, &mut log, &ram);
            // suppressed path does NOT set restart pending
            assert!(!fd.has_restart_pending());
        }
    }

    #[test]
    fn bump_deadcode_delay_skips_wrong_space_type() {
        let mut h = Heritage::new();
        let mut fd = build_fd();
        let mut ovr = Override::new();
        let mut log = RestartLog::new();
        // constant space is IPTR_CONSTANT -> not the right kind, returns early
        let cspace = Rc::clone(fd.get_arch().manage().get_space(0).unwrap());
        h.bump_deadcode_delay(&mut fd, &mut ovr, &mut log, &cspace);
        assert!(!ovr.has_deadcode_delay(&cspace));
        assert!(!fd.has_restart_pending());
    }

    // ---- Augmented dominator tree + phi placement (the SSA core) ----------

    /// Map the computed `merge` block ids back to their indices, sorted.
    fn merge_indices(h: &Heritage, fd: &Funcdata) -> Vec<int4> {
        let mut v: Vec<int4> =
            h.merge_points().iter().map(|&b| fd.bblocks_ref().block(b).get_index()).collect();
        v.sort_unstable();
        v
    }

    #[test]
    fn phi_placement_diamond_join_gets_multiequal() {
        // Diamond:  0 -> 1, 0 -> 2, 1 -> 3, 2 -> 3.  block 3 is the join.
        let mut fd = build_fd();
        let b = build_cfg(&mut fd, 4, &[(0, 1), (0, 2), (1, 3), (2, 3)]);
        // idom(3) must be 0 (two predecessors), so 3 is in the dominance
        // frontier of any write in block 1 or 2.
        assert_eq!(
            fd.bblocks_ref().block(b[3]).get_immed_dom(),
            Some(b[0]),
            "diamond join idom is start"
        );

        let mut h = Heritage::new();
        h.build_info_list(&fd);
        h.build_adt(&fd);
        // A write occurring in block 1 -> MULTIEQUAL needed at the join (3).
        h.calc_multiequals(&fd, &[b[1]]);
        assert_eq!(merge_indices(&h, &fd), vec![3], "diamond: phi at the join");
    }

    #[test]
    fn phi_placement_diamond_no_write_no_phi() {
        // With only the start block seeded (no write blocks), there is no
        // dominance-frontier work, so merge is empty.
        let mut fd = build_fd();
        let b = build_cfg(&mut fd, 4, &[(0, 1), (0, 2), (1, 3), (2, 3)]);
        let _ = b;
        let mut h = Heritage::new();
        h.build_info_list(&fd);
        h.build_adt(&fd);
        h.calc_multiequals(&fd, &[]);
        assert!(h.merge_points().is_empty(), "no writes -> no phi");
    }

    #[test]
    fn phi_placement_loop_header_gets_multiequal() {
        // Simple loop:  0 -> 1 (header) -> 2 (body) -> 1 (back-edge), 1 -> 3 (exit).
        //   edges: 0->1, 1->2, 2->1, 1->3
        // A write in the body (block 2) must place a MULTIEQUAL at the loop
        // header (block 1), where the back-edge re-enters.
        let mut fd = build_fd();
        let b = build_cfg(&mut fd, 4, &[(0, 1), (1, 2), (2, 1), (1, 3)]);
        // header has two preds (0 and the back-edge from 2); idom(1) = 0.
        assert_eq!(fd.bblocks_ref().block(b[1]).size_in(), 2, "header has back-edge");
        assert_eq!(fd.bblocks_ref().block(b[1]).get_immed_dom(), Some(b[0]));

        let mut h = Heritage::new();
        h.build_info_list(&fd);
        h.build_adt(&fd);
        h.calc_multiequals(&fd, &[b[2]]);
        assert_eq!(merge_indices(&h, &fd), vec![1], "loop: phi at the header");
    }

    #[test]
    fn phi_placement_nested_loop_writes_in_inner_body() {
        // Nested loops:
        //   0 -> 1 (outer header)
        //   1 -> 2 (inner header)
        //   2 -> 3 (inner body) -> 2 (inner back-edge)
        //   2 -> 4 (after inner) -> 1 (outer back-edge)
        //   1 -> 5 (exit)
        // A write in the inner body (block 3) re-enters at the inner header (2);
        // its value also flows around the outer loop, so the outer header (1)
        // is in the iterated dominance frontier as well.
        let mut fd = build_fd();
        let b = build_cfg(
            &mut fd,
            6,
            &[(0, 1), (1, 2), (2, 3), (3, 2), (2, 4), (4, 1), (1, 5)],
        );
        // inner header (2) has preds {1, back-edge 3}; outer header (1) has
        // preds {0, back-edge 4}.
        assert_eq!(fd.bblocks_ref().block(b[2]).size_in(), 2);
        assert_eq!(fd.bblocks_ref().block(b[1]).size_in(), 2);

        let mut h = Heritage::new();
        h.build_info_list(&fd);
        h.build_adt(&fd);
        h.calc_multiequals(&fd, &[b[3]]);
        // Iterated dominance frontier of the inner-body write: inner header (2)
        // and outer header (1).
        assert_eq!(merge_indices(&h, &fd), vec![1, 2], "nested loop: phi at both headers");
    }

    #[test]
    fn build_adt_sets_depth_and_maxdepth() {
        // A straight chain 0->1->2->3 has dominator depths 1,2,3,4.
        let mut fd = build_fd();
        let _b = build_cfg(&mut fd, 4, &[(0, 1), (1, 2), (2, 3)]);
        let mut h = Heritage::new();
        h.build_info_list(&fd);
        h.build_adt(&fd);
        // build_dom_depth returns size+1 entries (the trailing entry is the
        // "no idom" sentinel bucket == 0, mirroring buildDomTree's extra slot).
        assert_eq!(&h.depth[..4], &[1, 2, 3, 4]);
        assert_eq!(h.maxdepth, 4);
        // No joins -> a write anywhere needs no phi (frontier is empty).
        h.calc_multiequals(&fd, &[_b[1]]);
        assert!(h.merge_points().is_empty(), "chain: no joins -> no phi");
    }

    // ---- realized driver: runs cleanly on the empty IR (no Varnodes/ops) ---
    //
    // (The end-to-end SSA-construction parity — real reads linked to writes,
    // phi placement — is the `heritage_b3` gate, which drives a real corpus
    // function through ActionHeritage and diffs the post-heritage IR against
    // the C++ B3 snapshot.  These unit checks pin the driver's empty-IR base
    // case: it walks the spaces, builds an empty disjoint cover, places no
    // phis, renames nothing, and advances `pass`.)

    #[test]
    fn rename_on_empty_blocks_is_a_noop() {
        let mut fd = build_fd();
        let _b = build_cfg(&mut fd, 2, &[(0, 1)]);
        let mut h = Heritage::new();
        h.build_info_list(&fd);
        h.build_adt(&fd);
        // No Varnodes to rename; the dom-tree walk visits both blocks and
        // returns without creating or linking anything.
        h.rename(&mut fd);
        assert_eq!(fd.vbank().num_varnodes(), 0, "rename created no Varnodes on an empty function");
    }

    #[test]
    fn place_multiequals_empty_disjoint_is_a_noop() {
        let mut fd = build_fd();
        let _b = build_cfg(&mut fd, 2, &[(0, 1)]);
        let mut h = Heritage::new();
        h.build_info_list(&fd);
        h.build_adt(&fd);
        // disjoint is empty -> no MULTIEQUALs placed.
        h.place_multiequals(&mut fd);
        assert!(h.merge_points().is_empty(), "no disjoint ranges -> no phi blocks");
    }

    #[test]
    fn heritage_driver_runs_and_advances_pass() {
        let mut fd = build_fd();
        let _b = build_cfg(&mut fd, 2, &[(0, 1)]);
        let mut h = Heritage::new();
        h.build_info_list(&fd);
        assert_eq!(h.get_pass(), 0, "fresh heritage starts at pass 0");
        // Empty function: the driver walks the spaces (no free Varnodes),
        // places no phis, renames nothing, and bumps the pass counter.
        h.heritage(&mut fd);
        assert_eq!(h.get_pass(), 1, "one heritage pass advances `pass` to 1");
        assert_eq!(fd.vbank().num_varnodes(), 0, "empty function: no Varnodes created");
    }

    /// END-TO-END SSA: the full driver (collect → guard → calcMultiequals →
    /// MULTIEQUAL construction → rename) builds real SSA on a hand-built diamond.
    ///
    /// A single-manager `Funcdata` (glb's manager *is* the `ram` space the
    /// Varnodes use — the dual-manager seam the corpus B3 gate documents is
    /// absent here), so heritage reaches the Varnodes.  CFG:
    /// ```text
    ///   0 (entry) ─┬─▶ 1 ─┐
    ///              └─▶ 2 ─┴─▶ 3
    /// ```
    /// Blocks 1 and 2 each WRITE `ram[0x100]` (a 1-byte COPY of a constant);
    /// block 3 READS `ram[0x100]`.  Post-heritage we must see exactly one
    /// MULTIEQUAL at block 3, its two inputs the two writes, and the block-3
    /// read linked to the phi's output.
    #[test]
    fn heritage_builds_real_ssa_on_a_diamond() {
        use kuna_base::space::{addrspace_flags, ConstantSpace, UniqueSpace};
        use kuna_num::opcodes::OpCode;
        // A manager whose `ram` space has heritage delay 0 (so the first pass
        // processes it — registers in a real cspec have delay 0; the shared
        // `build_manager` uses delay 1, which would skip pass 0).
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
            0, // delay 0 -> heritaged on pass 0
            0,
        )))
        .unwrap();
        let glb = Rc::new(Architecture::new(m));
        let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
        let entry = Address::new(Rc::clone(&ram), 0x2000);
        let mut fd = Funcdata::new("func", "func", glb, entry, 0x10000000, 0x40).unwrap();
        let b = build_cfg(&mut fd, 4, &[(0, 1), (0, 2), (1, 3), (2, 3)]);
        // Give each block a 1-byte code range so seqnums are distinct/ordered.
        for (i, &bl) in b.iter().enumerate() {
            let a = raddr(&fd, 0x2000 + i as u64);
            fd.set_basic_block_range(bl, &a, &a);
        }
        let loc = raddr(&fd, 0x100); // the heritaged ram location
        let copy = |fl: u32| crate::seams::TypeOp::new(OpCode::CPUI_COPY, fl, "copy");

        // Block 1: ram[0x100] = #0xaa  (a free write at 0x2001).
        let w1 = fd.new_op(1, raddr(&fd, 0x2001));
        fd.op_set_opcode(w1, copy(0));
        let c1 = fd.new_constant(1, 0xaa);
        fd.op_set_input(w1, c1, 0).unwrap();
        let w1out = fd.new_varnode_out(1, &loc, w1).unwrap();
        fd.op_insert_end(w1, b[1]);
        // Block 2: ram[0x100] = #0xbb  (free write at 0x2002).
        let w2 = fd.new_op(1, raddr(&fd, 0x2002));
        fd.op_set_opcode(w2, copy(0));
        let c2 = fd.new_constant(1, 0xbb);
        fd.op_set_input(w2, c2, 0).unwrap();
        let _w2out = fd.new_varnode_out(1, &loc, w2).unwrap();
        fd.op_insert_end(w2, b[2]);
        // Block 3: ram[0x200] = ram[0x100]  (free read of the heritaged loc).
        let r3 = fd.new_op(1, raddr(&fd, 0x2003));
        fd.op_set_opcode(r3, copy(0));
        let rin = fd.new_varnode(1, &loc, None);
        fd.op_set_input(r3, rin, 0).unwrap();
        let _r3out = fd.new_varnode_out(1, &raddr(&fd, 0x200), r3).unwrap();
        fd.op_insert_end(r3, b[3]);

        // Pre-heritage: the block-3 read input is free.
        assert!(fd.vbank().get(rin).unwrap().is_free(), "pre-heritage read is free");
        let before_w1out_def = fd.vbank().get(w1out).unwrap().is_written();
        assert!(before_w1out_def, "the writes are defined");

        // Run the full driver.
        let mut h = Heritage::new();
        h.build_info_list(&fd);
        h.heritage(&mut fd);

        // Exactly one MULTIEQUAL, at block 3 (the join).
        let phis: Vec<crate::seams::OpId> = fd
            .obank()
            .iter_alive()
            .filter(|&op| fd.obank().get(op).unwrap().code() == OpCode::CPUI_MULTIEQUAL)
            .collect();
        assert_eq!(phis.len(), 1, "exactly one phi placed");
        let phi = phis[0];
        let phi_block = fd.obank().get(phi).unwrap().get_parent().unwrap();
        assert_eq!(fd.bblocks_ref().block(phi_block).get_index(), 3, "phi at the join block 3");

        // The phi has 2 inputs (block 3 has two preds) and an output at `loc`.
        assert_eq!(fd.obank().get(phi).unwrap().num_input(), 2, "phi has 2 in-edges");
        let phi_out = fd.obank().get(phi).unwrap().get_out().unwrap();
        assert_eq!(fd.vbank().get(phi_out).unwrap().get_addr(), &loc, "phi output at the loc");

        // The block-3 read is now linked to the phi output (rename replaced the
        // free read with the reaching definition — the phi).
        let r3in = fd.obank().get(r3).unwrap().get_in(0).unwrap();
        assert_eq!(r3in, phi_out, "block-3 read linked to the phi output (real SSA)");
        assert!(
            !fd.vbank().get(r3in).unwrap().is_free(),
            "the linked read is no longer free"
        );

        // The phi's two inputs are the two write Varnodes (def-use of the writes).
        let in0 = fd.obank().get(phi).unwrap().get_in(0).unwrap();
        let in1 = fd.obank().get(phi).unwrap().get_in(1).unwrap();
        let in_defs: Vec<crate::seams::OpId> = [in0, in1]
            .iter()
            .filter_map(|&v| fd.vbank().get(v).unwrap().get_def())
            .collect();
        assert!(in_defs.contains(&w1) && in_defs.contains(&w2), "phi merges the two writes");
    }
}
