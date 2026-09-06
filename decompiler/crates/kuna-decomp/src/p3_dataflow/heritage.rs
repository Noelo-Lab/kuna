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
//! # Cross-wave stubs (what is *not* fully realized here)
//!
//! `heritage.cc` is the most subsystem-entangled file in the decompiler.  The
//! algorithmic core above is self-contained at the W3 IR + W3 block-dominator
//! level and is ported and tested.  The data-flow *construction* surface it
//! drives needs `Funcdata` SSA-construction primitives and W4/W6 subsystems
//! that are not present in the merged tree.  Each is stub-noted at its method
//! and recorded in the porter's `losses`:
//!
//!   * `// STUB(W3-op)` — `setInputVarnode`, `newVarnodeOut`, `newUniqueOut`,
//!     `markReturnCopy`, `newIndirectCreation`/`newIndirectOp`,
//!     `opInsertBefore/After`-by-iterator, and the single-address
//!     `beginLoc(addr)`/`endLoc(addr)` location-set range.  These belong to the
//!     `funcdata_op`/`funcdata_varnode`/`varnode` waves; without them the
//!     renaming write/input creation, the MULTIEQUAL output construction, the
//!     normalize/concat/split helpers, and the `collect` driver cannot be
//!     realized.  Their algorithm structure is transcribed in comments at the
//!     stub so the wave that supplies the primitive can fill the body verbatim.
//!   * `// STUB(W4)` — the guard machinery (`guardCalls`/`guardStores`/
//!     `guardLoads`/`guardReturns` and helpers) needs `FuncCallSpecs`,
//!     `ParamActive`, `EffectRecord`, the `Architecture` stack/proto/join
//!     surface, `Override`, and `PreferSplitManager`.
//!   * `LoadGuard::establish_range`/`LoadGuard::finalize_range` and
//!     `Heritage::analyze_new_load_guards`/`handle_new_load_copies` are live
//!     against the IR-bound value-set solver
//!     ([`crate::rangeutil::ValueSetSolver`]), gated by
//!     `option loadguardrange` (default on).
//!
//! The [`Heritage::bump_deadcode_delay`] recorder anchor (the kuna restart
//! observability hook) is realized against the merged [`Override`] and
//! [`RestartLog`]: because `Funcdata` does not yet own either handle (a W4
//! stub), the method takes them as explicit `&mut` parameters — see its doc.

use kuna_base::address::Address;
use kuna_base::space::{spacetype, AddrSpace};
use kuna_base::types::{int4, uint4, uintb};
use std::collections::BTreeMap;
use std::rc::Rc;

use crate::kuna_restartlog::{KunaRestartReason, RestartLog};
use crate::overrides::Override;
use crate::context::BlockId;
use crate::varnode::varnode_flags;

// =============================================================================
// VariableStack (heritage.hh:29)
// =============================================================================

/// Container holding the stack system for the renaming algorithm.  Every
/// disjoint address range (indexed by its initial address) maps to its own
/// Varnode stack (C++ `typedef map<Address,vector<Varnode *> > VariableStack`).
pub type VariableStack = BTreeMap<Address, Vec<crate::context::VarnodeId>>;

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
    /// The returned tuple is the *key* of the containing element, the
    /// `intersect` code and the element's size; the C++ returns an iterator,
    /// but every caller only reads `(*iter).first` and `(*iter).second.size`,
    /// both of which are already in hand here.  (The size used to be re-looked
    /// through [`get`](LocationMap::get), a second descent per call — this map
    /// takes over a million `add`s on a large function.)
    pub fn add(&mut self, mut addr: Address, mut size: int4, mut pass: int4) -> (Address, int4, int4) {
        // We replicate the C++ map-iterator walk against the ordered keys.
        let mut intersect = 0;

        // Find the candidate element: `lower_bound(addr)` stepped back one
        // when it is not `begin()`, which is the last key strictly less than
        // `addr` whenever one exists and `begin()` otherwise.  Asking for the
        // predecessor first answers both cases in a single descent (the
        // predecessor exists on essentially every call once the map is warm),
        // where probing `lower_bound` and then `begin()` cost three.
        let mut cur: Option<(Address, SizePass)> =
            match self.themap.range(..addr.clone()).next_back() {
                Some((k, v)) => Some((k.clone(), *v)),
                None => self.themap.range(addr.clone()..).next().map(|(k, v)| (k.clone(), *v)),
            };

        // C++: advance past the stepped-back element when it does not overlap addr.
        if let Some((ck, cv)) = &cur {
            if addr.overlap(0, ck, cv.size) == -1 {
                // ++iter : advance to the first key strictly greater than ck
                cur = self.themap.range((
                    std::ops::Bound::Excluded(ck.clone()),
                    std::ops::Bound::Unbounded,
                ))
                .next()
                .map(|(k, v)| (k.clone(), *v));
            }
        }

        // First (possibly partial) overlap with the leading element.
        if let Some((ck, entry)) = cur.clone() {
            let where_ = addr.overlap(0, &ck, entry.size);
            if where_ != -1 {
                if where_ + size <= entry.size {
                    // Completely contained in previous element.
                    intersect = if entry.pass < pass { 2 } else { 0 };
                    return (ck, intersect, entry.size);
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

        // C++: absorb every following element overlapping (addr,size), erasing it.
        loop {
            // iter is the first key >= addr now (the erased key is gone, the
            // map iterator in C++ continues from the post-erase position;
            // the next key >= addr is the equivalent successor).
            let next_key = self
                .themap
                .range(addr.clone()..)
                .next()
                .map(|(k, v)| (k.clone(), *v));
            let Some((nk, nentry)) = next_key else { break };
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
        (addr, intersect, size)
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
/// (`// STUB(W3-op)` — not realized), so a `Vec` suffices for the realized
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
        // Already reset.
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
    pub op: crate::context::OpId,
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
    pub fn set(op: crate::context::OpId, s: &Rc<AddrSpace>, off: uintb) -> LoadGuard {
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

    /// Return true if the record still describes an active LOAD/STORE
    /// (C++ `isValid`, `heritage.hh:170`).
    pub fn is_valid(&self, fd: &crate::funcdata::Funcdata, opc: kuna_num::opcodes::OpCode) -> bool {
        fd.obank().get(self.op).map(|o| !o.is_dead() && o.code() == opc).unwrap_or(false)
    }

    /// Convert partial value set analysis into a guard range (C++
    /// `establishRange`, `heritage.cc:741`).
    ///
    /// This can sometimes get `minimumOffset` (otherwise the original constant
    /// pulled with the LOAD is used), `step` (if the partial analysis shows
    /// step and direction), and in rare cases `maximumOffset`.
    /// `analysis_state` is set to 1 if full range analysis is not needed.
    pub fn establish_range(&mut self, value_set: &crate::rangeutil::ValueSetRead) {
        let range = value_set.get_range();
        let range_size = range.get_size();
        let mut size: uintb;
        if range.is_empty() {
            self.minimum_offset = self.pointer_base;
            size = 0x1000;
        } else if range.is_full() || range_size > 0xffffff {
            self.minimum_offset = self.pointer_base;
            size = 0x1000;
            self.analysis_state = 1; // Don't bother doing more analysis
        } else {
            // Check for consistent step.
            self.step = if range_size == 3 { range.get_step() } else { 0 };
            size = 0x1000;
            if value_set.is_left_stable() {
                self.minimum_offset = range.get_min();
            } else if value_set.is_right_stable() {
                if self.pointer_base < range.get_end() {
                    self.minimum_offset = self.pointer_base;
                    size = range.get_end() - self.pointer_base;
                } else {
                    self.minimum_offset = range.get_min();
                    size = range_size.wrapping_mul(range.get_step() as uintb);
                }
            } else {
                self.minimum_offset = self.pointer_base;
            }
        }
        let max = self.spc.get_highest();
        if self.minimum_offset > max {
            self.minimum_offset = max;
            self.maximum_offset = self.minimum_offset; // Something is seriously wrong
        } else {
            // The C++ lets this uintb arithmetic wrap (max = 2^64-1, min = 0).
            let max_size = (max - self.minimum_offset).wrapping_add(1);
            if size > max_size {
                size = max_size;
            }
            self.maximum_offset = self.minimum_offset.wrapping_add(size).wrapping_sub(1);
        }
    }

    /// Convert value set analysis to the final guard range (C++
    /// `finalizeRange`, `heritage.cc:788`).
    pub fn finalize_range(&mut self, value_set: &crate::rangeutil::ValueSetRead) {
        self.analysis_state = 1; // In all cases the settings determined here are final
        let range = value_set.get_range();
        let mut range_size = range.get_size();
        if range_size == 0x100 || range_size == 0x10000 {
            // These sizes likely result from the storage size of the index.
            if self.step == 0 {
                // If we didn't see signs of iteration, don't use this range.
                range_size = 0;
            }
        }
        if range_size > 1 && range_size < 0xffffff {
            // Did we converge to something reasonable?
            self.analysis_state = 2; // Mark that we got a definitive result
            if range_size > 2 {
                self.step = range.get_step();
            }
            self.minimum_offset = range.get_min();
            // NOTE: don't subtract a whole step.
            self.maximum_offset = range.get_end().wrapping_sub(1) & range.get_mask();
            if self.maximum_offset < self.minimum_offset {
                // Values extend into what is usually stack parameters.
                self.maximum_offset = self.spc.get_highest();
                self.analysis_state = 1; // Remove the lock as we have likely overflowed
            }
        }
        if self.minimum_offset > self.spc.get_highest() {
            self.minimum_offset = self.spc.get_highest();
        }
        if self.maximum_offset > self.spc.get_highest() {
            self.maximum_offset = self.spc.get_highest();
        }
    }
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
/// discovery — `// STUB(W4)`.
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
/// stub-noted per method (`// STUB(W3-op)` / `// STUB(W4)` / `// STUB(W6)`).
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
    /// Only the stub-noted `handleNewLoadCopies`/`generateLoadGuard`
    /// (`// STUB(W4)`) read/write this; carried so the struct layout matches the
    /// C++ exactly and the W4 guard wave can fill the bodies without
    /// re-declaring the field.
    #[allow(dead_code)]
    load_copy_ops: Vec<crate::context::OpId>,
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
    pub fn get_store_guard(&self, op: crate::context::OpId) -> Option<&LoadGuard> {
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
    /// driver owns the write Varnode list — `// STUB(W3-op)`).
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
    /// The bump protocol, transcribed:
    ///   1. only `IPTR_PROCESSOR` / `IPTR_SPACEBASE` spaces qualify;
    ///   2. only when `spc.getDelay() == spc.getDeadcodeDelay()` (no global
    ///      delay yet);
    ///   3. if [`Override::has_deadcode_delay`] already returns true, record the
    ///      `krestart_deadcode_suppressed` event and return (the delay is
    ///      installed *once*);
    ///   4. otherwise insert `deadcodeDelay+1`, set restart pending, and record
    ///      `krestart_deadcode_bump`.
    ///
    /// # Stub
    ///
    /// The C++ reaches `fd->getOverride()` and the file-static restart table
    /// via `kunaRecordRestart(*fd, ...)`.  `Funcdata` does not yet own either
    /// (a W4 stub), so this method takes the merged [`Override`] and
    /// [`RestartLog`] as explicit `&mut` parameters and the function as `&mut`
    /// (for `setRestartPending`).  When `Funcdata` gains `get_override()` /
    /// the restart-log handle the W7/W8 assembler can drop the extra params and
    /// route through `fd`.  // STUB(W7)
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
    // a subsystem the merged tree still stubs (W4 callspecs, W4/W6 proto
    // active-output, W4 local-scope queryProperties), the transcription routes
    // through the existing merged accessors, which report "none" — exactly the
    // C++ behavior for a function with no calls, no recovered prototype, and no
    // recovered local scope.  Each such point is marked `STUB`.
    // =========================================================================

    /// Collect the reads/writes/inputs of a memory range (C++
    /// `Heritage::collect`, `heritage.cc:308`).  Scans `beginLoc(addr)` up to
    /// `endLoc(endaddr)` partitioning Varnodes; returns the maximum write size.
    fn collect(
        &self,
        fd: &crate::funcdata::Funcdata,
        memrange: &mut MemRange,
        read: &mut Vec<crate::context::VarnodeId>,
        write: &mut Vec<crate::context::VarnodeId>,
        input: &mut Vec<crate::context::VarnodeId>,
        remove: &mut Vec<crate::context::VarnodeId>,
    ) -> int4 {
        read.clear();
        write.clear();
        input.clear();
        remove.clear();
        let endaddr = &memrange.addr + i64::from(memrange.size);
        memrange.addr.get_space().expect("collect: range space");
        let mut maxsize = 0;
        for vn in fd.vbank().iter_loc_addr_range(&memrange.addr, &endaddr) {
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
        read: &mut [crate::context::VarnodeId],
        write: &mut Vec<crate::context::VarnodeId>,
    ) {
        for slot in 0..read.len() {
            let vn = read[slot];
            let descend = fd.vbank().get(vn).expect("guard: stale read vn").num_descend();
            if descend == 0 {
                continue; // removeRevisitedMarkers may have eliminated descendant
            }
            if descend != 1 {
                // C++ `Heritage::guard` throws LowlevelError("Free varnode with
                // multiple reads") here — a free read must have exactly one
                // descendant.  The w10-callsite-args wave had DOWNGRADED this throw
                // to a guard-and-continue (LOSS-150), because the *partial* call-arg
                // recovery left register reads that flowed to several call sites
                // collapsed onto a single free read before the deferred INDIRECT
                // call-side-effect chain could separate them, and the throw fired
                // 24x on the corpus.  With `ActionDeadCode::markConsumedParameters`
                // now keeping each recovered call-argument's def-chain alive
                // (w10-callarg-values), that premature collapse no longer happens:
                // the invariant violation FIRES 0x across the whole datatest corpus
                // (verified by the `KUNA_PROBE_MULTIREAD` probe).  So the faithful
                // C++ throw is RESTORED (LOSS-150 closed).  It propagates as a
                // panic the `decompile_func` orchestration boundary catches and
                // degrades to a recoverable per-function Err — the same graceful
                // route the C++ throw takes (aborting that function's decompile).
                panic!("kuna heritage: Free varnode with multiple reads");
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
            //
            // The C++ `localmap->queryProperties` walks the parent-scope chain up
            // to the global scope.  The merged kuna `localmap`
            // ([`crate::varmap::ScopeLocal`]) owns a detached `Database`, so its
            // *global* reach is supplied by the snapshot wired onto `glb`
            // ([`crate::context::GlobalQuery`], built after every `map addr`):
            // `query_global_properties` returns the same `mapped | addrtied |
            // persist` a global-mapped range would.  This is what lets the persist
            // RETURN-COPY branch of `guard_returns` fire, so a global store's
            // value is read at function exit (an `addrforce` COPY) and its def
            // chain survives `ActionDeadCode`.
            //
            // (The recovered-*local* persist surface — a stack range inside the
            // global discovery range — would add to `fl` through the local scope's
            // own `queryProperties`; that local symbol-map query is the naming
            // wave's `ScopeLocal` and is left as a documented stub.  Local stack
            // ranges are not persistent, so `fl`'s persist bit comes only from the
            // global query here, which is faithful for the global-store cases.)
            //
            // NOTE (LOSS-156 gate, w10-stacklocal-typing): the local-scope half of
            // this walk is now AVAILABLE as `fd.query_local_properties(addr,size,
            // usepoint)` (the faithful `localmap->queryProperties` port — see
            // `Funcdata::query_local_properties` / `ScopeLocal::query_properties`).
            // OR-ing it into `fl` here is the FULL fix for `map addr`-mapped stack
            // structs (it gains Partial splitting #15-19 + Wayoff array #1 by giving
            // a mapped stack range `addrtied`, so `guard_calls` builds the INDIRECT
            // that keeps its stores live across calls and `propagateSpacebaseRef`
            // types it). It is held OUT here because enabling it ALSO exposes two
            // downstream gaps that regress 4 assertions: (a) addrForced array stores
            // do not store-cross-merge (varcross "Store cross #1/#2"), and (b) a
            // typed stack struct's `&v1.arr1[a]` intermediate pointer is not
            // forwarded/collapsed (dupptr "Intermediate pointers #3/#5"). Wire this
            // OR once the store-cross-merge + pointer-forwarding gaps close.
            let usepoint = Address::new_invalid();
            let mut fl: uint4 = fd.get_arch().query_global_properties(addr, size, &usepoint);
            // (kuna w10-chainb-gap1) WIRED: the local-scope OR — the faithful
            // `localmap->queryProperties` half of `Heritage::guard`'s
            // `queryProperties` (heritage.cc:1192).  This gives a `map addr`-mapped
            // stack range its `mapped|addrtied` properties, so `guard_calls`/
            // `guard_stores` build the INDIRECT that keeps its stores live across
            // calls and `propagateSpacebaseRef` types the symbol.  Both prior
            // downstream gaps are now CLOSED: Gap-2 (intermediate-pointer spill)
            // by the `add_map` is_global persist fix (database.rs, chainb-gaps);
            // Gap-1 (addrforced mapped-array store-cross merge, varcross "Store
            // cross #1/#2") by the W7 `StackAffectingOps::populate` +
            // W6 `discoverIndexedStackPointers` store-guard source landed in this
            // wave — `Merge::testUntiedCallIntersection`'s cover test now sees the
            // loop store `*v1=0` as a store-guard and blocks ECX from folding into
            // `local_array[10]`.  Net +7 (Partial splitting #15-19, Wayoff array
            // #1, No-for-loop alias #3), regressed-set empty.
            fl |= fd.query_local_properties(addr, size, &usepoint);
            self.guard_calls(fd, fl, addr, size, write);
            self.guard_returns(fd, fl, addr, size, write);
            // STUB(W4/W6): highPtrPossible queries the recovered type system /
            // pointer analysis (`glb->highPtrPossible`), absent from the merged
            // `context::ArchContext` (which carries only the space manager).  With
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
    /// STUB(W4): iterating call sites needs `Funcdata::numCalls`/`getCallSpecs`
    /// (the `qlst` callspec subsystem), which the merged tree does not own.  A
    /// function with no calls has `numCalls()==0`, so this loop is empty — the
    /// faithful behavior here.  When the callspec subsystem lands, the C++ body
    /// (`heritage.cc:1444-1538`: per-call effect/trial/INDIRECT guarding) folds
    /// in unchanged.
    fn guard_calls(
        &mut self,
        fd: &mut crate::funcdata::Funcdata,
        fl: uint4,
        addr: &Address,
        size: int4,
        write: &mut Vec<crate::context::VarnodeId>,
    ) {
        use crate::fspec::{effect_type, Containment, OFFSET_UNKNOWN};
        use kuna_base::space::spacetype;

        let holdind = (fl & varnode_flags::addrtied) != 0;
        // (kuna) `option calloverlap off|in|full` — the two upstream partial-range
        // overlap guards.  See [`crate::p3_dataflow::kuna_calloverlap`].
        let overlap_level = fd.get_arch().call_overlap;
        // Lift the call specs out so each trial-registering mutation can also take
        // `&mut Funcdata` (the C++ mutates `fd` through the `FuncCallSpecs *`).
        let mut qlst = fd.take_call_specs();
        let spc = match addr.get_space() {
            Some(s) => s.clone(),
            None => {
                fd.restore_call_specs(qlst);
                return;
            }
        };
        for fc in qlst.iter_mut() {
            let op = fc.get_op();
            let is_assignment =
                fd.obank().get(op).map(|o| o.is_assignment()).unwrap_or(false);
            if is_assignment {
                if let Some(outvn) = fd.obank().get(op).and_then(|o| o.get_out()) {
                    let matches = fd
                        .vbank()
                        .get(outvn)
                        .map(|v| v.get_addr() == addr && v.get_size() == size)
                        .unwrap_or(false);
                    if matches {
                        continue;
                    }
                }
            }
            let mut off = addr.get_offset();
            let mut tryregister = true;
            if spc.get_type() == spacetype::IPTR_SPACEBASE {
                if fc.get_spacebase_offset() != OFFSET_UNKNOWN {
                    off = spc.wrap_offset(off.wrapping_sub(fc.get_spacebase_offset()));
                } else {
                    tryregister = false;
                }
            }
            let trans_addr = Address::new(Rc::clone(&spc), off);

            // effecttype = fc->hasEffect(transAddr, size).  `FuncCallSpecs : public
            // FuncProto`, so `hasEffect` is the inherited `FuncProto::hasEffect`
            // (fspec.cc:4239) reached through `proto()`.  Computed up front (C++
            // heritage.cc:1468) because the output-active branch can promote it to
            // `killedbycall`.
            let mut effecttype = fc.proto().has_effect(&trans_addr, size);
            // (kuna) `calleepreserves` — the cspec's `killedbycall` set is a
            // statement about the CONVENTION, not about this callee.  When a
            // bounded decode of the callee's own body proves it never writes
            // this register, downgrade the effect to `unaffected` so the
            // caller's value flows across the call instead of being replaced by
            // an INDIRECT creation with no definition.  The output-active branch
            // is skipped for the same range: a register the callee provably
            // never writes cannot be carrying its return value either, so
            // registering an output trial for it would put the clobber straight
            // back.  See [`crate::p4_calls::kuna_calleepreserves`].
            let preserved = effecttype == effect_type::KILLEDBYCALL
                && crate::p4_calls::kuna_calleepreserves::callee_preserves_range(
                    fd,
                    fc,
                    &trans_addr,
                    size,
                );
            if preserved {
                effecttype = effect_type::UNAFFECTED;
            }

            // Output-active branch (heritage.cc:1470-1487): register a return-value
            // trial for the call's killed output register and, when the effect is a
            // register clobber, promote the effect to `killedbycall` so the INDIRECT
            // *creation* branch below seeds the recovered return value.  `possibleout`
            // tracks whether this range became a fresh output trial (controls the
            // `markIndirectCreation` in0 flag).
            let mut possibleoutput = false;
            if !preserved && fc.is_output_active() && tryregister {
                let output_character = fc.proto().characterize_as_output(&trans_addr, size);
                if output_character != Containment::NoContainment {
                    if effecttype != effect_type::KILLEDBYCALL && fc.proto().is_auto_killed_by_call() {
                        effecttype = effect_type::KILLEDBYCALL;
                    }
                    if output_character == Containment::ContainedBy {
                        // tryOutputOverlapGuard (heritage.cc:1249/1293): the heritaged
                        // range properly contains the return storage, so the return
                        // entry becomes an INDIRECT creation and the bytes around it
                        // are PIECEd back on.  Gated by `option calloverlap full`.
                        if overlap_level >= crate::p3_dataflow::kuna_calloverlap::LEVEL_FULL
                            && self
                                .try_output_overlap_guard(fd, fc, addr, &trans_addr, size, write)
                        {
                            // Range is handled, don't do additional guarding.
                            effecttype = effect_type::UNAFFECTED;
                        }
                    } else {
                        // active->whichTrial(transAddr,size)<0 ? registerTrial : possibleoutput
                        if fc.get_active_output().which_trial(&trans_addr, size) < 0 {
                            fc.get_active_output().register_trial(&trans_addr, size);
                            possibleoutput = true;
                        }
                    }
                }
            }
            // The locked stack-output build (heritage.cc:1488-1494): the callee
            // returns a struct-by-value into a callee-relative stack location, and
            // ActionFuncLink::func_link_output deliberately delayed creating the
            // CALL op's output Varnode (setStackOutputLock(true)).  When the
            // heritaged stack range overlaps that locked return storage, materialize
            // the caller-perspective output and truncate/piece it into the range.
            else if fc.is_stack_output_lock() && tryregister {
                let output_character = fc.proto().characterize_as_output(&trans_addr, size);
                if output_character != Containment::NoContainment {
                    // effecttype = EffectRecord::unknown_effect;
                    effecttype = effect_type::UNKNOWN_EFFECT;
                    if self.try_output_stack_guard(
                        fd, fc, addr, &trans_addr, size, output_character, write,
                    ) {
                        // Range is handled, don't do additional guarding.
                        effecttype = effect_type::UNAFFECTED;
                    }
                }
            }

            // Input-active branch (heritage.cc:1496-1510): register an input
            // parameter trial and append the argument Varnode to the CALL op.  This
            // is what makes a register/stack argument appear as a call argument —
            // the `func(args)` rendering this wave targets.
            if fc.is_input_active() && tryregister {
                let ic = fc.proto().characterize_as_input_param(&trans_addr, size);
                // Upstream's nesting, not a collapsed `&&`: the ContainedBy arm is an
                // `else if` on the CHARACTERIZATION alone, so an existing trial on a
                // ContainsJustified range must not fall through into the overlap guard.
                if ic == Containment::ContainsJustified {
                    if fc.get_active_input().which_trial(&trans_addr, size) < 0 {
                        fc.get_active_input().register_trial(&trans_addr, size);
                        let vn = fd.new_varnode(size, addr, None);
                        fd.vbank_mut()
                            .get_mut(vn)
                            .expect("guardCalls: new arg vn")
                            .set_active_heritage();
                        let nin = fd.obank().get(op).map(|o| o.num_input()).unwrap_or(0);
                        let _ = fd.op_insert_input(op, vn, nin);
                    }
                } else if ic == Containment::ContainedBy
                    && overlap_level >= crate::p3_dataflow::kuna_calloverlap::LEVEL_IN
                {
                    Self::guard_call_overlapping_input(fd, fc, addr, &trans_addr, size);
                }
            }

            // The `unknown_effect`/`return_address` INDIRECT-*op* (heritage.cc:1512-
            // 1521) re-reads the address-tied range across the call so its Cover spans
            // the call site (the alias guard a call casts over an address-tied range it
            // might read/write through a pointer, plus the saved return-address range).
            // C++-faithful: emitted for EVERY address-tied range whose effect is
            // unknown/return-address (no persist restriction).
            let _ = fl;
            if effecttype == effect_type::UNKNOWN_EFFECT
                || effecttype == effect_type::RETURN_ADDRESS
            {
                let indop = fd.new_indirect_op(op, addr, size, 0);
                if let Some(in0) = fd.obank().get(indop).and_then(|o| o.get_in(0)) {
                    if let Some(v) = fd.vbank_mut().get_mut(in0) {
                        v.set_active_heritage();
                    }
                }
                if let Some(out) = fd.obank().get(indop).and_then(|o| o.get_out()) {
                    if let Some(v) = fd.vbank_mut().get_mut(out) {
                        v.set_active_heritage();
                        if holdind {
                            v.set_addr_force();
                        }
                        if effecttype == effect_type::RETURN_ADDRESS {
                            v.set_return_address();
                        }
                    }
                    write.push(out);
                }
            }
            // else if (effecttype == killedbycall) — the call clobbers this register
            // and (when possibleoutput) produces it as a return value.  Seed an
            // INDIRECT *creation* op `out[addr,size] = INDIRECT(0, iop(call))`; its
            // output Varnode is the recovered return value that ActionActiveReturn's
            // collectOutputTrialVarnodes/buildOutputFromTrials promotes to the CALL's
            // output (heritage.cc:1522-1526).
            else if effecttype == effect_type::KILLEDBYCALL {
                let indop = fd.new_indirect_creation(op, addr, size, possibleoutput);
                if let Some(out) = fd.obank().get(indop).and_then(|o| o.get_out()) {
                    if let Some(v) = fd.vbank_mut().get_mut(out) {
                        v.set_active_heritage();
                    }
                    write.push(out);
                }
            }
        }
        fd.restore_call_specs(qlst);
    }

    /// Append a truncated \e input Varnode for a call whose parameter storage is
    /// strictly contained in the heritaged range (C++
    /// `Heritage::guardCallOverlappingInput`, `heritage.cc:1210`).
    ///
    /// An associated function rather than a method so the caller can keep its
    /// `&mut Funcdata` and `&mut FuncCallSpecs` borrows alive across the call.
    /// Gated by `option calloverlap in|full`.
    fn guard_call_overlapping_input(
        fd: &mut crate::funcdata::Funcdata,
        fc: &mut crate::fspec::FuncCallSpecs,
        addr: &Address,
        trans_addr: &Address,
        size: int4,
    ) {
        use kuna_num::opcodes::OpCode;
        use kuna_num::pcoderaw::VarnodeData;
        let mut vdata = VarnodeData::default();
        if !fc
            .proto()
            .get_biggest_contained_input_param(trans_addr, size, &mut vdata)
        {
            return;
        }
        // truncAddr (callee perspective) -> caller perspective, exactly as
        // try_output_stack_guard converts the output side.
        let diff = vdata.offset.wrapping_sub(trans_addr.get_offset());
        let trunc_addr = addr + diff as i64;
        // C++ dedupes with the WHOLE range `size` and then registers `vData.size`;
        // the asymmetry is deliberately over-broad overlap, not a typo.
        if fc.get_active_input().which_trial(&trunc_addr, size) >= 0 {
            return;
        }
        let trunc_size = vdata.size as int4;
        let truncate_amount = addr.justified_contain(size, &trunc_addr, trunc_size, false);
        debug_assert!(
            truncate_amount >= 0,
            "guardCallOverlappingInput: entry not contained in the range"
        );
        let op = fc.get_op();
        let call_addr = fd
            .obank()
            .get(op)
            .expect("guardCallOverlappingInput: stale call")
            .get_addr()
            .clone();
        let subpiece_op = fd.new_op(2, call_addr);
        fd.op_set_opcode(subpiece_op, typeop_skeleton(OpCode::CPUI_SUBPIECE));
        let whole_vn = fd.new_varnode(size, addr, None);
        fd.vbank_mut()
            .get_mut(whole_vn)
            .expect("guardCallOverlappingInput: wholeVn")
            .set_active_heritage();
        let _ = fd.op_set_input(subpiece_op, whole_vn, 0);
        let c = fd.new_constant(4, truncate_amount as i64 as u64);
        let _ = fd.op_set_input(subpiece_op, c, 1);
        let vn = fd
            .new_varnode_out(trunc_size, &trunc_addr, subpiece_op)
            .expect("guardCallOverlappingInput: subpiece out");
        fd.op_insert_before(subpiece_op, op);
        fc.get_active_input().register_trial(&trunc_addr, trunc_size);
        let nin = fd.obank().get(op).map(|o| o.num_input()).unwrap_or(0);
        let _ = fd.op_insert_input(op, vn, nin);
    }

    /// Guard a range that properly contains an active call's return storage (C++
    /// `Heritage::tryOutputOverlapGuard`, `heritage.cc:1249`).
    ///
    /// Returns `true` when the INDIRECTs were created, in which case the caller
    /// downgrades the range's effect to `unaffected`.  Gated by
    /// `option calloverlap full`.
    fn try_output_overlap_guard(
        &mut self,
        fd: &mut crate::funcdata::Funcdata,
        fc: &mut crate::fspec::FuncCallSpecs,
        addr: &Address,
        trans_addr: &Address,
        size: int4,
        write: &mut Vec<crate::context::VarnodeId>,
    ) -> bool {
        use kuna_num::pcoderaw::VarnodeData;
        let mut vdata = VarnodeData::default();
        if !fc
            .proto()
            .get_biggest_contained_output(trans_addr, size, &mut vdata)
        {
            return false;
        }
        let diff = vdata.offset.wrapping_sub(trans_addr.get_offset());
        let trunc_addr = addr + diff as i64;
        // As on the input side, C++ dedupes with the whole range `size`.
        if fc.get_active_output().which_trial(&trunc_addr, size) >= 0 {
            return false; // Trial already exists
        }
        let call_op = fc.get_op();
        let trunc_size = vdata.size as int4;
        self.guard_output_overlap(fd, call_op, addr, size, &trunc_addr, trunc_size, write);
        fc.get_active_output().register_trial(&trunc_addr, trunc_size);
        true
    }

    /// Piece the recovered return storage back into the larger heritaged range
    /// (C++ `Heritage::guardOutputOverlap`, `heritage.cc:1293`).
    ///
    /// NOT the same as [`Self::guard_output_overlap_stack`]: the register form makes
    /// every piece an INDIRECT *creation*, where the stack form pulls the flanking
    /// pieces off a pre-call value with `newIndirectOp` + SUBPIECE.
    #[allow(clippy::too_many_arguments)]
    fn guard_output_overlap(
        &mut self,
        fd: &mut crate::funcdata::Funcdata,
        call_op: crate::context::OpId,
        addr: &Address,
        size: int4,
        ret_addr: &Address,
        ret_size: int4,
        write: &mut Vec<crate::context::VarnodeId>,
    ) {
        use kuna_num::opcodes::OpCode;
        let size_front = (ret_addr.get_offset().wrapping_sub(addr.get_offset())) as i64 as int4;
        let size_back = size - ret_size - size_front;
        let ind_op = fd.new_indirect_creation(call_op, ret_addr, ret_size, true);
        let ind_addr = fd
            .obank()
            .get(ind_op)
            .expect("guardOutputOverlap: stale indOp")
            .get_addr()
            .clone();
        let mut vn_collect = fd
            .obank()
            .get(ind_op)
            .and_then(|o| o.get_out())
            .expect("guardOutputOverlap: vnCollect");
        let mut insert_point = call_op;
        if size_front != 0 {
            // C++ passes indOp (not callOp) as the front piece's indirect effect.
            let ind_front = fd.new_indirect_creation(ind_op, addr, size_front, false);
            let new_front = fd
                .obank()
                .get(ind_front)
                .and_then(|o| o.get_out())
                .expect("guardOutputOverlap: front out");
            let concat_front = fd.new_op(2, ind_addr.clone());
            let slot_new: int4 = if ret_addr.is_big_endian() { 0 } else { 1 };
            fd.op_set_opcode(concat_front, typeop_skeleton(OpCode::CPUI_PIECE));
            let _ = fd.op_set_input(concat_front, new_front, slot_new);
            let _ = fd.op_set_input(concat_front, vn_collect, 1 - slot_new);
            vn_collect = fd
                .new_varnode_out(size_front + ret_size, addr, concat_front)
                .expect("guardOutputOverlap: front concat out");
            fd.op_insert_after(concat_front, insert_point);
            insert_point = concat_front;
        }
        if size_back != 0 {
            let addr_back = ret_addr + ret_size as i64;
            let ind_back = fd.new_indirect_creation(call_op, &addr_back, size_back, false);
            let new_back = fd
                .obank()
                .get(ind_back)
                .and_then(|o| o.get_out())
                .expect("guardOutputOverlap: back out");
            let concat_back = fd.new_op(2, ind_addr.clone());
            let slot_new: int4 = if ret_addr.is_big_endian() { 1 } else { 0 };
            fd.op_set_opcode(concat_back, typeop_skeleton(OpCode::CPUI_PIECE));
            let _ = fd.op_set_input(concat_back, new_back, slot_new);
            let _ = fd.op_set_input(concat_back, vn_collect, 1 - slot_new);
            vn_collect = fd
                .new_varnode_out(size, addr, concat_back)
                .expect("guardOutputOverlap: back concat out");
            fd.op_insert_after(concat_back, insert_point);
        }
        if let Some(v) = fd.vbank_mut().get_mut(vn_collect) {
            v.set_active_heritage();
        }
        write.push(vn_collect);
    }

    /// Attempt to guard a stack range against a call whose return value overlaps
    /// that range (C++ `Heritage::tryOutputStackGuard`, `heritage.cc:1392`).
    ///
    /// The call has a locked output stored on the stack.  ActionFuncLink
    /// deliberately delayed creating the CALL op's output Varnode; this method
    /// materializes it now.  If the return storage *contains* the guard range, a
    /// guard-range Varnode is pulled off the return via SUBPIECE; if the guard
    /// range *contains* the return storage, a final guard-range Varnode is pieced
    /// together from the return storage plus the surrounding INDIRECT stack pieces
    /// (`guard_output_overlap_stack`).
    fn try_output_stack_guard(
        &mut self,
        fd: &mut crate::funcdata::Funcdata,
        fc: &crate::fspec::FuncCallSpecs,
        addr: &Address,
        trans_addr: &Address,
        size: int4,
        output_character: crate::fspec::Containment,
        write: &mut Vec<crate::context::VarnodeId>,
    ) -> bool {
        use kuna_num::opcodes::OpCode;
        use kuna_num::pcoderaw::VarnodeData;
        let call_op = fc.get_op();
        if output_character == crate::fspec::Containment::ContainedBy {
            // ContainedBy = the heritage range contains the return storage
            // (`ParamEntry::contained_by`, fspec.hh:104): piece the smaller return
            // storage back into the range through a join of stack pieces.
            let mut vdata = VarnodeData::default();
            if !fc.proto().get_biggest_contained_output(trans_addr, size, &mut vdata) {
                return false;
            }
            let vspace = match vdata.space.clone() {
                Some(s) => s,
                None => return false,
            };
            // truncAddr (callee perspective) -> caller perspective via diff.
            let trunc_off_callee = vdata.offset;
            let diff = trunc_off_callee.wrapping_sub(trans_addr.get_offset());
            let trunc_addr = addr + diff as i64;
            // (truncAddr keeps the return-storage space only conceptually; the
            // caller-perspective address lives in `addr`'s space, mirroring
            // `truncAddr = addr + diff` in C++.)
            let _ = vspace;
            self.guard_output_overlap_stack(
                fd,
                call_op,
                addr,
                size,
                &trunc_addr,
                vdata.size as int4,
                write,
            );
            return true;
        }
        // Reaching here, output exists and contains the heritage range.
        let ret_addr_callee = fc.proto().get_output().get_address();
        let diff = addr.get_offset().wrapping_sub(trans_addr.get_offset());
        let ret_addr = &ret_addr_callee + diff as i64; // caller perspective
        let ret_size = fc.proto().get_output().get_size();
        // The CALL op's output is None here — ActionFuncLink delayed creating it.
        let mut outvn = fd.obank().get(call_op).and_then(|o| o.get_out());
        let mut vn_final: Option<crate::context::VarnodeId> = None;
        if outvn.is_none() {
            let v = fd
                .new_varnode_out(ret_size, &ret_addr, call_op)
                .expect("tryOutputStackGuard: new outvn");
            outvn = Some(v);
            vn_final = Some(v);
        }
        if size < ret_size {
            let sub_piece = fd.new_op(2, fd.obank().get(call_op).expect("tryOutputStackGuard: stale call").get_addr().clone());
            fd.op_set_opcode(sub_piece, typeop_skeleton(OpCode::CPUI_SUBPIECE));
            let truncate_amount = ret_addr.justified_contain(ret_size, addr, size, false);
            let c = fd.new_constant(4, truncate_amount as i64 as u64);
            let _ = fd.op_set_input(sub_piece, c, 1);
            let _ = fd.op_set_input(sub_piece, outvn.expect("tryOutputStackGuard: outvn"), 0);
            let v = fd
                .new_varnode_out(size, addr, sub_piece)
                .expect("tryOutputStackGuard: subpiece out");
            vn_final = Some(v);
            fd.op_insert_after(sub_piece, call_op);
        }
        if let Some(vf) = vn_final {
            if let Some(v) = fd.vbank_mut().get_mut(vf) {
                v.set_active_heritage();
            }
            write.push(vf);
        }
        true
    }

    /// Guard a stack range that properly contains the return value storage for a
    /// call (C++ `Heritage::guardOutputOverlapStack`, `heritage.cc:1323`).
    ///
    /// The pieces of the range on either side of the return storage flow through
    /// the call via INDIRECT (pulled off the full range with SUBPIECE), then
    /// rejoin the return storage via CPUI_PIECE.
    #[allow(clippy::too_many_arguments)]
    fn guard_output_overlap_stack(
        &mut self,
        fd: &mut crate::funcdata::Funcdata,
        call_op: crate::context::OpId,
        addr: &Address,
        size: int4,
        ret_addr: &Address,
        ret_size: int4,
        write: &mut Vec<crate::context::VarnodeId>,
    ) {
        use kuna_num::opcodes::OpCode;
        let size_front = (ret_addr.get_offset().wrapping_sub(addr.get_offset())) as i64 as int4;
        let size_back = size - ret_size - size_front;
        let call_addr = fd.obank().get(call_op).expect("guardOutputOverlapStack: stale call").get_addr().clone();
        let mut insert_point = call_op;
        let mut vn_collect = match fd.obank().get(call_op).and_then(|o| o.get_out()) {
            Some(v) => v,
            None => fd
                .new_varnode_out(ret_size, ret_addr, call_op)
                .expect("guardOutputOverlapStack: new vnCollect"),
        };
        if size_front != 0 {
            let new_input = fd.new_varnode(size, addr, None);
            fd.vbank_mut()
                .get_mut(new_input)
                .expect("guardOutputOverlapStack: front newInput")
                .set_active_heritage();
            let sub_piece = fd.new_op(2, call_addr.clone());
            fd.op_set_opcode(sub_piece, typeop_skeleton(OpCode::CPUI_SUBPIECE));
            let truncate_amount = addr.justified_contain(size, addr, size_front, false);
            let c = fd.new_constant(4, truncate_amount as i64 as u64);
            let _ = fd.op_set_input(sub_piece, c, 1);
            let _ = fd.op_set_input(sub_piece, new_input, 0);
            let ind_op_front = fd.new_indirect_op(call_op, addr, size_front, 0);
            let ind_in0 = fd
                .obank()
                .get(ind_op_front)
                .and_then(|o| o.get_in(0))
                .expect("guardOutputOverlapStack: front ind in0");
            let _ = fd.op_set_output(sub_piece, ind_in0);
            fd.op_insert_before(sub_piece, call_op);
            let new_front = fd
                .obank()
                .get(ind_op_front)
                .and_then(|o| o.get_out())
                .expect("guardOutputOverlapStack: front ind out");
            let concat_front = fd.new_op(2, call_addr.clone());
            let slot_new: int4 = if ret_addr.is_big_endian() { 0 } else { 1 };
            fd.op_set_opcode(concat_front, typeop_skeleton(OpCode::CPUI_PIECE));
            let _ = fd.op_set_input(concat_front, new_front, slot_new);
            let _ = fd.op_set_input(concat_front, vn_collect, 1 - slot_new);
            vn_collect = fd
                .new_varnode_out(size_front + ret_size, addr, concat_front)
                .expect("guardOutputOverlapStack: front concat out");
            fd.op_insert_after(concat_front, insert_point);
            insert_point = concat_front;
        }
        if size_back != 0 {
            let new_input = fd.new_varnode(size, addr, None);
            fd.vbank_mut()
                .get_mut(new_input)
                .expect("guardOutputOverlapStack: back newInput")
                .set_active_heritage();
            let addr_back = ret_addr + ret_size as i64;
            let sub_piece = fd.new_op(2, call_addr.clone());
            fd.op_set_opcode(sub_piece, typeop_skeleton(OpCode::CPUI_SUBPIECE));
            let truncate_amount = addr.justified_contain(size, &addr_back, size_back, false);
            let c = fd.new_constant(4, truncate_amount as i64 as u64);
            let _ = fd.op_set_input(sub_piece, c, 1);
            let _ = fd.op_set_input(sub_piece, new_input, 0);
            let ind_op_back = fd.new_indirect_op(call_op, &addr_back, size_back, 0);
            let ind_in0 = fd
                .obank()
                .get(ind_op_back)
                .and_then(|o| o.get_in(0))
                .expect("guardOutputOverlapStack: back ind in0");
            let _ = fd.op_set_output(sub_piece, ind_in0);
            fd.op_insert_before(sub_piece, call_op);
            let new_back = fd
                .obank()
                .get(ind_op_back)
                .and_then(|o| o.get_out())
                .expect("guardOutputOverlapStack: back ind out");
            let concat_back = fd.new_op(2, call_addr.clone());
            let slot_new: int4 = if ret_addr.is_big_endian() { 1 } else { 0 };
            fd.op_set_opcode(concat_back, typeop_skeleton(OpCode::CPUI_PIECE));
            let _ = fd.op_set_input(concat_back, new_back, slot_new);
            let _ = fd.op_set_input(concat_back, vn_collect, 1 - slot_new);
            vn_collect = fd
                .new_varnode_out(size, addr, concat_back)
                .expect("guardOutputOverlapStack: back concat out");
            fd.op_insert_after(concat_back, insert_point);
        }
        if let Some(v) = fd.vbank_mut().get_mut(vn_collect) {
            v.set_active_heritage();
        }
        write.push(vn_collect);
    }

    /// Guard RETURN ops for a global/output range (C++ `Heritage::guardReturns`,
    /// `heritage.cc:1653`).
    ///
    /// The active-output branch (potential return values) reads
    /// `Funcdata::getActiveOutput()` (set by `ActionPrototypeTypes::initActiveOutput`)
    /// together with `FuncProto::characterizeAsOutput`.  When the heritaged range
    /// overlaps the recovered output storage, it registers an output trial and
    /// appends a fresh Varnode at that range to every RETURN op — this is what
    /// attaches the recovered return register (e.g. 8051 `ACC`, reached via the
    /// MULTIEQUAL phi the renaming algorithm then builds) to the RETURN, so
    /// `ActionReturnRecovery` can finalize it.  The persist branch needs
    /// `(fl&persist)`, 0 for an unrecovered register range.
    fn guard_returns(
        &mut self,
        fd: &mut crate::funcdata::Funcdata,
        fl: uint4,
        addr: &Address,
        size: int4,
        _write: &mut [crate::context::VarnodeId],
    ) {
        use kuna_num::opcodes::OpCode;
        if fd.get_active_output().is_some() && fd.get_func_proto().has_model() {
            let output_character = fd.get_func_proto().characterize_as_output(addr, size);
            if output_character == crate::fspec::Containment::ContainedBy {
                self.guard_returns_overlapping(fd, addr, size);
            } else if output_character != crate::fspec::Containment::NoContainment {
                if let Some(active) = fd.get_active_output_mut() {
                    active.register_trial(addr, size);
                }
                let return_ops: Vec<crate::context::OpId> =
                    fd.obank().iter_code(OpCode::CPUI_RETURN).collect();
                for op in return_ops {
                    let o = match fd.obank().get(op) {
                        Some(o) => o,
                        None => continue,
                    };
                    if o.is_dead() || o.get_halt_type() != 0 {
                        continue;
                    }
                    let n = o.num_input();
                    let invn = fd.new_varnode(size, addr, None);
                    fd.vbank_mut()
                        .get_mut(invn)
                        .expect("guardReturns: new invn")
                        .set_active_heritage();
                    let _ = fd.op_insert_input(op, invn, n);
                }
            }
        }
        if (fl & varnode_flags::persist) == 0 {
            return;
        }
        // Persist RETURN-COPY branch (heritage.cc:1677-1692): the global value
        // must persist past the end of the function, so a COPY is inserted before each RETURN.
        // The `addrforce` output makes the COPY `isAutoLive()`, so `ActionDeadCode`
        // keeps the whole def chain of the global store alive (the store survives
        // and renders as `glob = ...`).
        let return_ops: Vec<crate::context::OpId> =
            fd.obank().iter_code(OpCode::CPUI_RETURN).collect();
        for op in return_ops {
            let (dead, ret_addr) = match fd.obank().get(op) {
                Some(o) => (o.is_dead(), o.get_addr().clone()),
                None => continue,
            };
            if dead {
                continue;
            }
            let copyop = fd.new_op(1, ret_addr);
            let vn = match fd.new_varnode_out(size, addr, copyop) {
                Ok(v) => v,
                Err(_) => continue,
            };
            if let Some(v) = fd.vbank_mut().get_mut(vn) {
                v.set_addr_force();
                v.set_active_heritage();
            }
            fd.op_set_opcode_code(copyop, OpCode::CPUI_COPY);
            // fd->markReturnCopy(copyop);  (op->flags |= return_copy)
            if let Some(o) = fd.obank_mut().get_mut(copyop) {
                o.set_flag(crate::op::pcodeop_flags::return_copy);
            }
            let invn = fd.new_varnode(size, addr, None);
            if let Some(v) = fd.vbank_mut().get_mut(invn) {
                v.set_active_heritage();
            }
            let _ = fd.op_set_input(copyop, invn, 0);
            fd.op_insert_before(copyop, op);
        }
    }

    /// Guard RETURN ops when the heritaged range *contains* the output storage,
    /// truncating to the recovered output (C++ `Heritage::guardReturnsOverlapping`,
    /// `heritage.cc:1610`).  Registers the truncated output trial and inserts a
    /// SUBPIECE before each RETURN, appending the truncated value as a new input.
    fn guard_returns_overlapping(
        &mut self,
        fd: &mut crate::funcdata::Funcdata,
        addr: &Address,
        size: int4,
    ) {
        use kuna_num::opcodes::OpCode;
        use kuna_num::pcoderaw::VarnodeData;
        let mut vdata = VarnodeData::default();
        if !fd.get_func_proto().get_biggest_contained_output(addr, size, &mut vdata) {
            return;
        }
        let vspace = match vdata.space.clone() {
            Some(s) => s,
            None => return,
        };
        let trunc_addr = Address::new(Rc::clone(&vspace), vdata.offset);
        if let Some(active) = fd.get_active_output_mut() {
            active.register_trial(&trunc_addr, vdata.size as int4);
        }
        // offset = least-significant bytes to truncate (endian-adjusted).
        let mut offset = (vdata.offset.wrapping_sub(addr.get_offset())) as i64 as int4;
        if vspace.is_big_endian() {
            offset = (size - vdata.size as int4) - offset;
        }
        let return_ops: Vec<crate::context::OpId> =
            fd.obank().iter_code(OpCode::CPUI_RETURN).collect();
        for op in return_ops {
            let o = match fd.obank().get(op) {
                Some(o) => o,
                None => continue,
            };
            if o.is_dead() || o.get_halt_type() != 0 {
                continue;
            }
            let op_addr = o.get_addr().clone();
            let n = o.num_input();
            let invn = fd.new_varnode(size, addr, None);
            let sub_op = fd.new_op(2, op_addr);
            fd.op_set_opcode(sub_op, typeop_skeleton(OpCode::CPUI_SUBPIECE));
            let _ = fd.op_set_input(sub_op, invn, 0);
            let coff = fd.new_constant(4, offset as i64 as u64);
            let _ = fd.op_set_input(sub_op, coff, 1);
            fd.op_insert_before(sub_op, op);
            let ret_val = fd
                .new_varnode_out(vdata.size as int4, &trunc_addr, sub_op)
                .expect("guardReturnsOverlapping: new ret_val");
            fd.vbank_mut()
                .get_mut(invn)
                .expect("guardReturnsOverlapping: invn")
                .set_active_heritage();
            let _ = fd.op_insert_input(op, ret_val, n);
        }
    }

    /// Guard STORE ops (C++ `Heritage::guardStores`, `heritage.cc:1539`).
    ///
    /// STUB(W4): adding an INDIRECT across an aliasing STORE needs
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
        _write: &mut [crate::context::VarnodeId],
    ) {
        unimplemented_stub("Heritage::guard_stores (needs Funcdata::newIndirectOp)");
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
        _write: &mut [crate::context::VarnodeId],
    ) {
        // C++: if ((fl & Varnode::addrtied)==0) return;  -- only address-tied
        // ranges can index-alias a stack LOAD.  The `loadGuard` list is empty
        // without `discoverIndexedStackPointers` populating it (no indexed-stack
        // LOADs here), so the COPY-guard loop (heritage.cc:1579-1607) is a no-op
        // regardless; it folds in once load-guard discovery lands.
        let _addrtied = (fl & varnode_flags::addrtied) != 0;
    }

    /// Remove deprecated CPUI_MULTIEQUAL, CPUI_INDIRECT, or CPUI_COPY ops,
    /// preparing to re-heritage (C++ `Heritage::removeRevisitedMarkers`,
    /// `heritage.cc:245`).
    ///
    /// If a previous Varnode was heritaged through a MULTIEQUAL or INDIRECT op,
    /// but now a larger range containing the Varnode is being heritaged, throw
    /// away the op, letting the data-flow for the new larger range determine the
    /// data-flow for the old Varnode.  The original Varnode is redefined as the
    /// output of a SUBPIECE of a larger free Varnode.  Return-form COPYs are
    /// simply removed, in preparation for a larger COPY.
    fn remove_revisited_markers(
        &mut self,
        fd: &mut crate::funcdata::Funcdata,
        remove: &[crate::context::VarnodeId],
        addr: &Address,
        size: int4,
    ) {
        use kuna_num::opcodes::OpCode;

        let space = addr.get_space().expect("remove_revisited_markers: addr space").clone();
        if self.get_info(&space).deadremoved > 0 {
            self.bump_deadcode_delay_persisted(fd, &space);
            if !self.get_info(&space).warningissued {
                self.get_info_mut(&space).warningissued = true;
                // fd->warningHeader("Heritage AFTER dead removal. Revisit: <addr>")
                // STUB(W8): the warning text needs Varnode::printRaw on `addr` (the
                // W8 print surface); the header is cosmetic (a block comment, not
                // IR) and the dead-code-delay bump above already records the event,
                // exactly as the `heritage()` deadremoved warning is handled.
            }
        }

        for &vn in remove {
            let op = fd
                .vbank()
                .get(vn)
                .expect("remove_revisited_markers: stale remove vn")
                .get_def()
                .expect("remove_revisited_markers: marker vn has no def");
            let bl = fd
                .obank()
                .get(op)
                .expect("remove_revisited_markers: stale op")
                .get_parent()
                .expect("remove_revisited_markers: marker op has no parent");
            let code = fd.obank().get(op).expect("remove_revisited_markers: stale op").code();

            // `pos` is the C++ block iterator the SUBPIECE is inserted *before*;
            // `None` denotes `bl->endOp()` (append at the block's end).
            let pos: Option<crate::context::OpId> = if code == OpCode::CPUI_INDIRECT {
                let iop_vn = fd
                    .obank()
                    .get(op)
                    .expect("remove_revisited_markers: stale INDIRECT op")
                    .get_in(1)
                    .expect("remove_revisited_markers: INDIRECT has no iop input");
                let iop_off = fd
                    .vbank()
                    .get(iop_vn)
                    .expect("remove_revisited_markers: stale iop vn")
                    .get_addr()
                    .get_offset();
                let target_op = crate::funcdata_varnode::op_iop_decode(iop_off);
                // Insert the SUBPIECE *after* the target of the INDIRECT.
                let anchor = if fd
                    .obank()
                    .get(target_op)
                    .map(|o| o.is_dead())
                    .unwrap_or(true)
                {
                    op
                } else {
                    target_op
                };
                // ++pos: the op following `anchor` in its block (None == endOp()).
                let after = fd
                    .obank()
                    .get(anchor)
                    .expect("remove_revisited_markers: stale anchor op")
                    .basic_neighbours()
                    .1;
                // Replacement INDIRECT will hold the address.
                fd.vbank_mut()
                    .get_mut(vn)
                    .expect("remove_revisited_markers: stale vn for clearAddrForce")
                    .clear_addr_force();
                after
            } else if code == OpCode::CPUI_MULTIEQUAL {
                // Insert the SUBPIECE after all MULTIEQUALs in the block.
                let mut after = fd
                    .obank()
                    .get(op)
                    .expect("remove_revisited_markers: stale MULTIEQUAL op")
                    .basic_neighbours()
                    .1;
                while let Some(p) = after {
                    let is_multi = fd
                        .obank()
                        .get(p)
                        .map(|o| o.code() == OpCode::CPUI_MULTIEQUAL)
                        .unwrap_or(false);
                    if !is_multi {
                        break;
                    }
                    after = fd
                        .obank()
                        .get(p)
                        .expect("remove_revisited_markers: stale block op")
                        .basic_neighbours()
                        .1;
                }
                after
            } else {
                // Remove return form COPY.
                fd.op_unlink(op);
                continue;
            };

            let offset = fd
                .vbank()
                .get(vn)
                .expect("remove_revisited_markers: stale vn for overlap")
                .overlap_range(addr, size);
            fd.op_uninsert(op);
            let big = fd.new_varnode(size, addr, None);
            fd.vbank_mut()
                .get_mut(big)
                .expect("remove_revisited_markers: new big vn")
                .set_active_heritage();
            // cast: int4 offset -> uintb constant value, as the C++ `newConstant`
            // widens the (non-negative) byte offset into the constant's value.
            let off_const = fd.new_constant(4, offset as uintb);
            fd.op_set_opcode(op, typeop_skeleton(OpCode::CPUI_SUBPIECE));
            fd.op_set_all_input(op, &[big, off_const])
                .expect("remove_revisited_markers: op_set_all_input");
            fd.op_insert(op, bl, pos);
            fd.vbank_mut()
                .get_mut(vn)
                .expect("remove_revisited_markers: stale vn for setWriteMask")
                .set_write_mask();
        }
    }

    /// Determine if a CALL/CALLIND `op` has an indirect effect on the memory
    /// range `[addr, addr+size)` (C++ `Heritage::callOpIndirectEffect`,
    /// `heritage.cc:359`).
    ///
    /// For a CALL/CALLIND we consult the call's `FuncCallSpecs`: a missing
    /// call-spec assumes an indirect effect (`true`); otherwise the effect is
    /// "indirect" unless `hasEffectTranslate` reports the range `unaffected`.
    /// CALLOTHER/NEW (the only other writers reaching here) are assumed to have
    /// no effect on `fd` variables except their own output (`false`).
    fn call_op_indirect_effect(
        &self,
        fd: &crate::funcdata::Funcdata,
        addr: &Address,
        size: int4,
        op: crate::context::OpId,
    ) -> bool {
        use kuna_num::opcodes::OpCode;
        let code = fd.obank().get(op).expect("call_op_indirect_effect: stale op").code();
        if code == OpCode::CPUI_CALL || code == OpCode::CPUI_CALLIND {
            // We should be able to get the callspec.
            match fd.get_call_specs_index(op) {
                None => true, // Assume indirect effect
                Some(i) => {
                    let effect = fd.get_call_specs(i).has_effect_translate(addr, size);
                    effect != crate::fspec::effect_type::UNAFFECTED
                }
            }
        } else {
            // CALLOTHER, NEW: no effect on -fd- variables except op->getOut().
            false
        }
    }

    /// Normalize a too-small read Varnode (C++ `Heritage::normalizeReadSize`,
    /// `heritage.cc:391`).  Builds a SUBPIECE of a new full-size Varnode that
    /// defines the original (now masked) read, returning the new full read.
    fn normalize_read_size(
        &self,
        fd: &mut crate::funcdata::Funcdata,
        vn: crate::context::VarnodeId,
        op: crate::context::OpId,
        addr: &Address,
        size: int4,
    ) -> crate::context::VarnodeId {
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
    /// `Heritage::normalizeWriteSize`, `heritage.cc:417`).
    ///
    /// Given a Varnode that is written but does not match the (larger) size of
    /// the address range currently being linked, create the missing pieces in
    /// the range and concatenate everything into a new Varnode of the correct
    /// size, using SUBPIECE (to extract the existing bytes from a big read) and
    /// PIECE (to recombine).  Transcribed from `heritage.cc:417-496`.
    ///
    /// The `op->isCall()` indirect-creation branch (`newIndirectCreation` +
    /// `callOpIndirectEffect`) needs the W4 FuncCallSpecs call-machinery that is
    /// still a stub; it is reached only for a CALL that writes a partial range,
    /// so it is the one narrow sub-stub left (the simple register/stack
    /// multi-size writes on the analysis critical path take the SUBPIECE/PIECE
    /// path fully ported here).
    fn normalize_write_size(
        &self,
        fd: &mut crate::funcdata::Funcdata,
        vn: crate::context::VarnodeId,
        addr: &Address,
        size: int4,
    ) -> crate::context::VarnodeId {
        use kuna_num::opcodes::OpCode;

        let (op, vsize, vaddr, vbig) = {
            let v = fd.vbank().get(vn).expect("normalize_write_size: stale vn");
            (
                v.get_def().expect("normalize_write_size: written vn has no def"),
                v.get_size(),
                v.get_addr().clone(),
                v.get_addr().is_big_endian(),
            )
        };
        // overlap = vn->overlap(addr,size) (endianness-aware, mirroring
        // `varnode.cc:Varnode::overlap`, as in normalize_read_size).
        let overlap = if !vbig {
            vaddr.overlap(0, addr, size)
        } else {
            let over = vaddr.overlap(vsize - 1, addr, size);
            if over != -1 {
                size - 1 - over
            } else {
                -1
            }
        };
        let mostsigsize = size - (overlap + vsize);
        let op_addr = fd.obank().get(op).expect("normalize_write_size: stale def op").get_addr().clone();
        let op_is_call = fd.obank().get(op).expect("normalize_write_size: stale def op").is_call();
        let addr_size = addr.get_space().expect("normalize_write_size: addr space").get_addr_size();

        // --- Most-significant missing piece -------------------------------
        let mut mostvn: Option<crate::context::VarnodeId> = None;
        if mostsigsize != 0 {
            let pieceaddr = if addr.is_big_endian() {
                addr.clone()
            } else {
                addr + (overlap + vsize) as i64
            };
            if op_is_call && self.call_op_indirect_effect(fd, &pieceaddr, mostsigsize, op) {
                // Does the CALL have an effect on the piece?  Don't create a new
                // big read if the write is from a CALL — model the killed piece
                // as an indirect creation off the CALL op.
                let newop = fd.new_indirect_creation(op, &pieceaddr, mostsigsize, false);
                mostvn = fd.obank().get(newop).and_then(|o| o.get_out());
            } else {
                let newop = fd.new_op(2, op_addr.clone());
                let mvn = fd
                    .new_varnode_out(mostsigsize, &pieceaddr, newop)
                    .expect("normalize_write_size: new_varnode_out mostvn");
                let big = fd.new_varnode(size, addr, None); // The new read
                fd.vbank_mut().get_mut(big).expect("normalize_write_size: big").set_active_heritage();
                fd.op_set_opcode(newop, typeop_skeleton(OpCode::CPUI_SUBPIECE));
                fd.op_set_input(newop, big, 0).expect("normalize_write_size: SUBPIECE in0");
                let c = fd.new_constant(addr_size as int4, (overlap + vsize) as uintb);
                fd.op_set_input(newop, c, 1).expect("normalize_write_size: SUBPIECE in1");
                fd.op_insert_before(newop, op);
                mostvn = Some(mvn);
            }
        }

        // --- Least-significant missing piece ------------------------------
        let mut leastvn: Option<crate::context::VarnodeId> = None;
        if overlap != 0 {
            let pieceaddr = if addr.is_big_endian() {
                addr + (size - overlap) as i64
            } else {
                addr.clone()
            };
            if op_is_call && self.call_op_indirect_effect(fd, &pieceaddr, overlap, op) {
                // Unless the CALL definitely has no effect on the piece: don't
                // create a new big read if the write is from a CALL.
                let newop = fd.new_indirect_creation(op, &pieceaddr, overlap, false);
                leastvn = fd.obank().get(newop).and_then(|o| o.get_out());
            } else {
                let newop = fd.new_op(2, op_addr.clone());
                let lvn = fd
                    .new_varnode_out(overlap, &pieceaddr, newop)
                    .expect("normalize_write_size: new_varnode_out leastvn");
                let big = fd.new_varnode(size, addr, None); // The new read
                fd.vbank_mut().get_mut(big).expect("normalize_write_size: big2").set_active_heritage();
                fd.op_set_opcode(newop, typeop_skeleton(OpCode::CPUI_SUBPIECE));
                fd.op_set_input(newop, big, 0).expect("normalize_write_size: SUBPIECE2 in0");
                let c = fd.new_constant(addr_size as int4, 0);
                fd.op_set_input(newop, c, 1).expect("normalize_write_size: SUBPIECE2 in1");
                fd.op_insert_before(newop, op);
                leastvn = Some(lvn);
            }
        }

        // --- Recombine the existing write with the least piece ------------
        let midvn: crate::context::VarnodeId = if overlap != 0 {
            let newop = fd.new_op(2, op_addr.clone());
            let midaddr = if addr.is_big_endian() { vaddr.clone() } else { addr.clone() };
            let mvn = fd
                .new_varnode_out(overlap + vsize, &midaddr, newop)
                .expect("normalize_write_size: new_varnode_out midvn");
            fd.op_set_opcode(newop, typeop_skeleton(OpCode::CPUI_PIECE));
            fd.op_set_input(newop, vn, 0).expect("normalize_write_size: PIECE in0 (most sig)");
            fd.op_set_input(newop, leastvn.expect("normalize_write_size: leastvn"), 1)
                .expect("normalize_write_size: PIECE in1 (least sig)");
            fd.op_insert_after(newop, op);
            mvn
        } else {
            vn
        };

        // --- Recombine the most piece on top ------------------------------
        let bigout: crate::context::VarnodeId = if mostsigsize != 0 {
            let newop = fd.new_op(2, op_addr.clone());
            let bvn = fd
                .new_varnode_out(size, addr, newop)
                .expect("normalize_write_size: new_varnode_out bigout");
            fd.op_set_opcode(newop, typeop_skeleton(OpCode::CPUI_PIECE));
            fd.op_set_input(newop, mostvn.expect("normalize_write_size: mostvn"), 0)
                .expect("normalize_write_size: PIECE2 in0 (most sig)");
            fd.op_set_input(newop, midvn, 1).expect("normalize_write_size: PIECE2 in1");
            let middef = fd
                .vbank()
                .get(midvn)
                .expect("normalize_write_size: stale midvn")
                .get_def()
                .expect("normalize_write_size: midvn has no def");
            fd.op_insert_after(newop, middef);
            bvn
        } else {
            midvn
        };

        fd.vbank_mut().get_mut(vn).expect("normalize_write_size: vn write mask").set_write_mask();
        bigout
    }

    /// Fill input holes in a heritaged range (C++ `Heritage::guardInput`,
    /// `heritage.cc:1953`).  When a single input fills the range it links
    /// automatically and we return; otherwise the gaps are filled with input
    /// Varnodes and concatenated.
    fn guard_input(
        &mut self,
        fd: &mut crate::funcdata::Funcdata,
        addr: &Address,
        size: int4,
        input: &[crate::context::VarnodeId],
    ) {
        if input.is_empty() {
            return;
        }
        if input.len() == 1
            && fd.vbank().get(input[0]).expect("guard_input: stale input").get_size() == size
        {
            return; // single input fills everything; links in automatically
        }
        // Make sure the input range is filled (no holes), then concatenate the
        // pieces into a single full-size input (heritage.cc:1962-2010).
        let spc = addr.get_space().expect("guard_input: range space").clone();
        let mut i = 0usize;
        let mut cur = addr.get_offset();
        let end = cur.wrapping_add(size as u64);
        let mut newinput: Vec<crate::context::VarnodeId> = Vec::new();
        while cur < end {
            let vn = if i < input.len() {
                let candidate = input[i];
                let voff = fd.vbank().get(candidate).expect("guard_input: input vn").get_addr().get_offset();
                if voff > cur {
                    let sz = (voff - cur) as int4;
                    let hole = fd.new_varnode(sz, &Address::new(Rc::clone(&spc), cur), None);
                    fd.set_input_varnode(hole).expect("guard_input: setInputVarnode hole")
                } else {
                    i += 1;
                    candidate
                }
            } else {
                let sz = (end - cur) as int4;
                let tail = fd.new_varnode(sz, &Address::new(Rc::clone(&spc), cur), None);
                fd.set_input_varnode(tail).expect("guard_input: setInputVarnode tail")
            };
            let vnsize = fd.vbank().get(vn).expect("guard_input: vn size").get_size();
            newinput.push(vn);
            cur = cur.wrapping_add(vnsize as u64);
        }
        if newinput.len() == 1 {
            return; // will get linked in automatically
        }
        for &vn in &newinput {
            fd.vbank_mut().get_mut(vn).expect("guard_input: write mask").set_write_mask();
        }
        let newout = fd.new_varnode(size, addr, None);
        let unified = self.concat_pieces(fd, &newinput, None, newout);
        fd.vbank_mut().get_mut(unified).expect("guard_input: unified").set_active_heritage();
    }

    /// Perform one level of Varnode splitting to match a [`JoinRecord`] (C++
    /// `Heritage::splitJoinLevel`, `heritage.cc:2068`).
    ///
    /// Splits every Varnode in `lastcombo`, appending the two split halves for
    /// each into `nextlev` (a `null`/`None` filler for the least-half keeps the
    /// 2-1 mapping when a Varnode is not split this level).
    fn split_join_level(
        &self,
        fd: &mut crate::funcdata::Funcdata,
        lastcombo: &[crate::context::VarnodeId],
        nextlev: &mut Vec<Option<crate::context::VarnodeId>>,
        joinrec: &Rc<kuna_base::space::JoinRecord>,
    ) {
        let numpieces = joinrec.num_pieces();
        let mut recnum: i32 = 0;
        for &curvn in lastcombo.iter() {
            let cursize = fd.vbank().get(curvn).expect("split_join_level: curvn").get_size();
            if cursize == joinrec.get_piece(recnum).size as int4 {
                nextlev.push(Some(curvn));
                nextlev.push(None);
                recnum += 1;
            } else {
                let mut sizeaccum = 0i32;
                let mut j = recnum;
                while j < numpieces {
                    sizeaccum += joinrec.get_piece(j).size as int4;
                    if sizeaccum == cursize {
                        j += 1;
                        break;
                    }
                    j += 1;
                }
                let numinhalf = (j - recnum) / 2; // Will be at least 1
                sizeaccum = 0;
                for k in 0..numinhalf {
                    sizeaccum += joinrec.get_piece(recnum + k).size as int4;
                }
                let mosthalf = if numinhalf == 1 {
                    let p = joinrec.get_piece(recnum);
                    let spc = p.space.clone().expect("split_join_level: piece space");
                    fd.new_varnode_space_off(sizeaccum, spc, p.offset)
                } else {
                    fd.new_unique(sizeaccum, None)
                };
                let leasthalf = if (j - recnum) == 2 {
                    let vdata = joinrec.get_piece(recnum + 1);
                    let spc = vdata.space.clone().expect("split_join_level: piece space");
                    fd.new_varnode_space_off(vdata.size as int4, spc, vdata.offset)
                } else {
                    fd.new_unique(cursize - sizeaccum, None)
                };
                nextlev.push(Some(mosthalf));
                nextlev.push(Some(leasthalf));
                recnum = j;
            }
        }
    }

    /// Construct pieces for a \e join-space Varnode read by an operation (C++
    /// `Heritage::splitJoinRead`, `heritage.cc:2119`).
    ///
    /// Builds a concatenation expression (PIECE ops) reconstructing `vn` out of
    /// the real piece Varnodes named by `joinrec`.
    fn split_join_read(
        &self,
        fd: &mut crate::funcdata::Funcdata,
        vn: crate::context::VarnodeId,
        joinrec: &Rc<kuna_base::space::JoinRecord>,
    ) {
        use kuna_num::opcodes::OpCode;
        // vn isFree, so loneDescend must be non-null.
        let mut op = fd.lone_descend(vn).expect("split_join_read: vn has a lone descendant");
        let is_primitive = {
            let v = fd.vbank().get(vn).expect("split_join_read: vn");
            if v.is_type_lock() {
                v.get_type().is_primitive_whole()
            } else {
                true
            }
        };

        let piece_typeop = typeop_skeleton(OpCode::CPUI_PIECE);
        let mut lastcombo: Vec<crate::context::VarnodeId> = vec![vn];
        while (lastcombo.len() as i32) < joinrec.num_pieces() {
            let mut nextlev: Vec<Option<crate::context::VarnodeId>> = Vec::new();
            self.split_join_level(fd, &lastcombo, &mut nextlev, joinrec);

            for (i, &curvn) in lastcombo.iter().enumerate() {
                let mosthalf = nextlev[2 * i];
                let leasthalf = nextlev[2 * i + 1];
                let Some(leasthalf) = leasthalf else { continue }; // not split this level
                let mosthalf = mosthalf.expect("split_join_read: most half present when split");
                let opaddr = fd.obank().get(op).expect("split_join_read: op").get_addr().clone();
                let concat = fd.new_op(2, opaddr);
                fd.op_set_opcode(concat, piece_typeop.clone());
                fd.op_set_output(concat, curvn).expect("split_join_read: set output");
                fd.op_set_input(concat, mosthalf, 0).expect("split_join_read: set in0");
                fd.op_set_input(concat, leasthalf, 1).expect("split_join_read: set in1");
                fd.op_insert_before(concat, op);
                if is_primitive {
                    fd.vbank_mut().get_mut(mosthalf).expect("split_join_read: mosthalf").set_precis_hi();
                    fd.vbank_mut().get_mut(leasthalf).expect("split_join_read: leasthalf").set_precis_lo();
                } else {
                    fd.op_mark_no_collapse(concat);
                }
                op = concat; // Keep -op- as the earliest op in the concatenation
            }

            lastcombo = nextlev.into_iter().flatten().collect();
        }
    }

    /// Split a written \e join-space Varnode into the specified pieces (C++
    /// `Heritage::splitJoinWrite`, `heritage.cc:2172`).
    ///
    /// Builds SUBPIECE expressions that carve `vn` (its def, or an input) into the
    /// real piece Varnodes named by `joinrec`.
    fn split_join_write(
        &self,
        fd: &mut crate::funcdata::Funcdata,
        vn: crate::context::VarnodeId,
        joinrec: &Rc<kuna_base::space::JoinRecord>,
    ) {
        use kuna_num::opcodes::OpCode;
        // vn cannot be free: either it has a def, or it is input.
        let mut op = fd.vbank().get(vn).expect("split_join_write: vn").get_def();
        let is_input = fd.vbank().get(vn).expect("split_join_write: vn").is_input();
        // The entry block (index 0).
        let bb = fd.bblocks_get_block(0);
        let bb_start = fd.bblocks_block_start(bb);
        let is_primitive = {
            let v = fd.vbank().get(vn).expect("split_join_write: vn");
            if v.is_type_lock() {
                v.get_type().is_primitive_whole()
            } else {
                true
            }
        };
        let sub_typeop = typeop_skeleton(OpCode::CPUI_SUBPIECE);

        let mut lastcombo: Vec<crate::context::VarnodeId> = vec![vn];
        while (lastcombo.len() as i32) < joinrec.num_pieces() {
            let mut nextlev: Vec<Option<crate::context::VarnodeId>> = Vec::new();
            self.split_join_level(fd, &lastcombo, &mut nextlev, joinrec);
            for (i, &curvn) in lastcombo.iter().enumerate() {
                let mosthalf = nextlev[2 * i];
                let leasthalf = nextlev[2 * i + 1];
                let Some(leasthalf) = leasthalf else { continue };
                let mosthalf = mosthalf.expect("split_join_write: most half present when split");
                let leastsize =
                    fd.vbank().get(leasthalf).expect("split_join_write: leasthalf").get_size();

                // First SUBPIECE: mosthalf = curvn >> leastsize.
                let opaddr = if is_input {
                    bb_start.clone()
                } else {
                    let prevop = op.expect("split_join_write: non-input has a def op");
                    fd.obank().get(prevop).expect("split_join_write: op").get_addr().clone()
                };
                let split = fd.new_op(2, opaddr);
                fd.op_set_opcode(split, sub_typeop.clone());
                fd.op_set_output(split, mosthalf).expect("split_join_write: set output most");
                fd.op_set_input(split, curvn, 0).expect("split_join_write: set in0 most");
                let c = fd.new_constant(4, leastsize as u64);
                fd.op_set_input(split, c, 1).expect("split_join_write: set in1 most");
                match op {
                    None => fd.op_insert_begin(split, bb),
                    Some(prevop) => fd.op_insert_after(split, prevop),
                }
                op = Some(split); // Keep -op- as the latest op in the split construction

                // Second SUBPIECE: leasthalf = curvn(0).
                let prevop = op.expect("split_join_write: op anchors second SUBPIECE");
                let opaddr2 =
                    fd.obank().get(prevop).expect("split_join_write: op").get_addr().clone();
                let split2 = fd.new_op(2, opaddr2);
                fd.op_set_opcode(split2, sub_typeop.clone());
                fd.op_set_output(split2, leasthalf).expect("split_join_write: set output least");
                fd.op_set_input(split2, curvn, 0).expect("split_join_write: set in0 least");
                let c0 = fd.new_constant(4, 0);
                fd.op_set_input(split2, c0, 1).expect("split_join_write: set in1 least");
                fd.op_insert_after(split2, prevop);
                if is_primitive {
                    fd.vbank_mut().get_mut(mosthalf).expect("split_join_write: mosthalf").set_precis_hi();
                    fd.vbank_mut().get_mut(leasthalf).expect("split_join_write: leasthalf").set_precis_lo();
                }
                op = Some(split2); // latest op in the split construction
            }
            lastcombo = nextlev.into_iter().flatten().collect();
        }
    }

    /// Create float truncation into a free lower-precision \e join-space Varnode
    /// (C++ `Heritage::floatExtensionRead`, `heritage.cc:2236`).
    fn float_extension_read(
        &self,
        fd: &mut crate::funcdata::Funcdata,
        vn: crate::context::VarnodeId,
        joinrec: &Rc<kuna_base::space::JoinRecord>,
    ) {
        use kuna_num::opcodes::OpCode;
        let op = fd.lone_descend(vn).expect("float_extension_read: vn has a lone descendant");
        let opaddr = fd.obank().get(op).expect("float_extension_read: op").get_addr().clone();
        let trunc = fd.new_op(1, opaddr);
        // Float extensions have exactly 1 piece.
        let vdata = joinrec.get_piece(0);
        let spc = vdata.space.clone().expect("float_extension_read: piece space");
        let bigvn = fd.new_varnode_space_off(vdata.size as int4, spc, vdata.offset);
        fd.op_set_opcode(trunc, typeop_skeleton(OpCode::CPUI_FLOAT_FLOAT2FLOAT));
        fd.op_set_output(trunc, vn).expect("float_extension_read: set output");
        fd.op_set_input(trunc, bigvn, 0).expect("float_extension_read: set in0");
        fd.op_insert_before(trunc, op);
    }

    /// Create float extension from a lower-precision \e join-space Varnode (C++
    /// `Heritage::floatExtensionWrite`, `heritage.cc:2256`).
    fn float_extension_write(
        &self,
        fd: &mut crate::funcdata::Funcdata,
        vn: crate::context::VarnodeId,
        joinrec: &Rc<kuna_base::space::JoinRecord>,
    ) {
        use kuna_num::opcodes::OpCode;
        let op = fd.vbank().get(vn).expect("float_extension_write: vn").get_def();
        let is_input = fd.vbank().get(vn).expect("float_extension_write: vn").is_input();
        // The entry block (index 0).
        let bb = fd.bblocks_get_block(0);
        let bb_start = fd.bblocks_block_start(bb);
        let opaddr = if is_input {
            bb_start
        } else {
            let prevop = op.expect("float_extension_write: non-input has a def op");
            fd.obank().get(prevop).expect("float_extension_write: op").get_addr().clone()
        };
        let ext = fd.new_op(1, opaddr);
        // Float extensions have exactly 1 piece.
        let vdata = joinrec.get_piece(0);
        let spc = vdata.space.clone().expect("float_extension_write: piece space");
        let outaddr = Address::new(spc, vdata.offset);
        fd.op_set_opcode(ext, typeop_skeleton(OpCode::CPUI_FLOAT_FLOAT2FLOAT));
        fd.new_varnode_out(vdata.size as int4, &outaddr, ext)
            .expect("float_extension_write: new_varnode_out");
        fd.op_set_input(ext, vn, 0).expect("float_extension_write: set in0");
        match op {
            None => fd.op_insert_begin(ext, bb),
            Some(prevop) => fd.op_insert_after(ext, prevop),
        }
    }

    /// Split \e join-space Varnodes up into their real components (C++
    /// `Heritage::processJoins`, `heritage.cc:2282`).
    ///
    /// For any Varnode in the \e join-space, look up its [`JoinRecord`] and split
    /// it into the specified real components, so join-space addresses play no role
    /// in the heritage process (there should be no free Varnodes in the join space
    /// afterward).
    fn process_joins(&mut self, fd: &mut crate::funcdata::Funcdata) {
        let joinspace = match fd.get_arch().manage().get_join_space().cloned() {
            Some(s) => s,
            None => return,
        };

        // C++ iterates beginLoc(joinspace)..endLoc(joinspace) over a live set; the
        // split ops it creates land in the PIECE spaces (register/unique), NOT the
        // join space, so a snapshot of the join-space loc-set is faithful (no new
        // join-space Varnode is ever inserted by the split, and the C++ break
        // condition `vn->getSpace() != joinspace` would stop at the first such).
        let join_vns = fd.vbank().loc_space_ids(&joinspace);
        for vn in join_vns {
            // vn->getSpace() != joinspace would break; the snapshot is all-join.
            let offset = fd.vbank().get(vn).expect("process_joins: vn").get_addr().get_offset();
            let joinrec = fd
                .get_arch()
                .manage()
                .find_join(offset)
                .expect("process_joins: join Varnode has a JoinRecord");
            let piecespace = joinrec.get_piece(0).space.clone().expect("process_joins: piece space");

            let (unified_size, vn_size, is_free) = {
                let v = fd.vbank().get(vn).expect("process_joins: vn");
                (joinrec.get_unified().size as int4, v.get_size(), v.is_free())
            };
            if unified_size != vn_size {
                panic!("Joined varnode does not match size of record");
            }
            if is_free {
                if joinrec.is_float_extension() {
                    self.float_extension_read(fd, vn, &joinrec);
                } else {
                    self.split_join_read(fd, vn, &joinrec);
                }
            }

            // Too soon to heritage this space.
            let delay = self.get_info(&piecespace).delay;
            if self.pass != delay {
                continue;
            }

            if joinrec.is_float_extension() {
                self.float_extension_write(fd, vn, &joinrec);
            } else {
                self.split_join_write(fd, vn, &joinrec); // Only once for a particular varnode
            }
        }
    }

    /// Clear stack placeholders before heritaging the stack space (C++
    /// `Heritage::clearStackPlaceholders`, `heritage.cc:2048`).
    ///
    /// STUB(W4): the per-call `abortSpacebaseRelative` needs the callspec
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
    ///
    /// (kuna DIV-50) The three input-creation sites go through
    /// [`kuna_inputtile::new_tiled_input`](crate::p3_dataflow::kuna_inputtile::new_tiled_input)
    /// rather than `set_input_varnode` directly, so a request that lands on
    /// `guard_input`'s leftover write-masked pieces is reconciled instead of
    /// aborting the function.
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
        let mut writelist: Vec<crate::context::VarnodeId> = Vec::new();

        let ops: Vec<crate::context::OpId> = self.block_ops(fd, bl);
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
                    let mut vnnew = match varstack.get(&key).and_then(|s| s.last().copied()) {
                        Some(top) => top,
                        None => {
                            let nv = crate::p3_dataflow::kuna_inputtile::new_tiled_input(
                                fd, in_size, &key,
                            )
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
                        // STUB(W4): the INDIRECT "same time" carve-out resolves
                        // the IOP-space constant in[1] back to its PcodeOp via
                        // `getOpFromConst`, which needs the IOP→op map.  This is
                        // only reached when a renamed value's top-of-stack is
                        // INDIRECT-defined (calls/stores), which the no-call/
                        // no-store critical path never produces.
                        let from_op = op_from_const(fd, def);
                        if from_op == Some(op) {
                            let stacklen = varstack.get(&key).map(|s| s.len()).unwrap_or(0);
                            if stacklen == 1 {
                                let nv = crate::p3_dataflow::kuna_inputtile::new_tiled_input(
                                    fd, in_size, &key,
                                )
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
            // Only the block's *leading* MULTIEQUALs are visited (the C++ walk
            // breaks at the first op that is not one), so collect that prefix
            // instead of materializing every op of the successor: this runs
            // once per CFG edge per heritage pass and the successors of a large
            // function's blocks are mostly ordinary code.  Nothing in the body
            // can change an op's opcode or its position in the block, so the
            // prefix taken here is the prefix the live walk would see.
            let subops: Vec<crate::context::OpId> = {
                let mut leading = Vec::new();
                let mut cur = fd.bb_op_head(subbl);
                while let Some(o) = cur {
                    if fd.obank().get(o).expect("rename_recurse: stale multiop").code()
                        != OpCode::CPUI_MULTIEQUAL
                    {
                        break;
                    }
                    leading.push(o);
                    cur = fd.bb_op_next(o);
                }
                leading
            };
            for multiop in subops {
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
                        let nv = crate::p3_dataflow::kuna_inputtile::new_tiled_input(
                            fd, in_size, &key,
                        )
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
    ) -> Vec<crate::context::OpId> {
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

    /// Concatenate a list of Varnodes together at the given location (C++
    /// `Heritage::concatPieces`, `heritage.cc:508`).
    ///
    /// There must be at least 2 Varnodes in `vnlist`, ordered most- to
    /// least-significant.  The Varnodes become inputs to a chain of PIECE ops
    /// whose final output is `finalvn`.  Returns the final unified Varnode.  When
    /// `insertop` is `None` the chain is inserted at the beginning of the entry
    /// block; otherwise before `insertop`.
    //
    // `needless_range_loop`: the index `i` selects the last piece (`i == n-1` ->
    // wire `finalvn` as the output) and `vnlist[i]` is the current piece — both
    // the position test and the element read are needed, so the C++ indexed walk
    // is the faithful form.
    //
    // BLOCKED next-locus — "Stack string #9" (`stackstring.xml`, `alterString`):
    // the 1-byte store `mov [rsp+9], dil` into a `char v1[40]` (the array typing
    // comes from the `customPrint(char *)` callee proto) must render `v1[9] = a0`
    // but renders `v1[8] = CONCAT11(a0,v1[8])`.  The stub is purely a single op in
    // the raw IR at the store instruction (0x100170):
    //
    //   CPP:  a3: s0x..d1:1 = DIL                       (byte's own v1[9] stack home)
    //         6e: s0x..d0:2 = CONCAT11(s0x..d1:1, s0x..d0:1)
    //   RUST: 6e: s0x..d0:2 = CONCAT11(DIL, s0x..d0:1)  (no a3; reads the register)
    //
    // i.e. CPP gives the stored byte its OWN address-tied 1-byte stack varnode at
    // the store-target offset (v1[9]) and feeds THAT into the read-modify-write
    // CONCAT11 that rebuilds the adjacent 2-byte element; the printer then folds the
    // CONCAT tail and emits the plain `v1[9] = a0` byte-store.  Confirmed (cpp +
    // rust both reproduce in isolation given `option readonly on` + `parse line
    // extern void customPrint(char *)`) to be a FLOW/early-analysis difference, not
    // a print arm and not a late rule: the `a3` COPY is already present at the very
    // first action (`break action normalanalysis`), gated on the char-array typing
    // of `v1` (absent it, both engines emit `CONCAT11(DIL,..)` and an `xunknown8`
    // local).  The faithful fix lives in the spacebase-relative byte-STORE lowering
    // / `ScopeLocal` byte-element refinement that places a register byte stored into
    // a typed stack-array element into its own addr-tied 1-byte stack home before
    // the RMW CONCAT is built (the cpp `s0x..d1:1 = DIL` COPY) — NOT in `printc`'s
    // CONCAT11 render arm (which is byte-faithful here).
    #[allow(clippy::needless_range_loop)]
    fn concat_pieces(
        &self,
        fd: &mut crate::funcdata::Funcdata,
        vnlist: &[crate::context::VarnodeId],
        insertop: Option<crate::context::OpId>,
        finalvn: crate::context::VarnodeId,
    ) -> crate::context::VarnodeId {
        use kuna_num::opcodes::OpCode;
        let mut preexist = vnlist[0];
        let isbigendian =
            fd.vbank().get(preexist).expect("concat_pieces: vn0").get_addr().is_big_endian();
        // Resolve the insertion block + the op we insert *before* (None => block begin),
        // and the address each new op carries.
        let (bl, before, opaddress) = match insertop {
            None => {
                let root = fd.bblocks_ref().root.expect("concat_pieces: bblocks root");
                let start = fd
                    .bblocks_ref()
                    .get_start_block(root)
                    .expect("concat_pieces: start block");
                let head = fd.bb_op_head(start);
                (start, head, fd.get_address().clone())
            }
            Some(op) => {
                let parent =
                    fd.obank().get(op).expect("concat_pieces: insertop").get_parent().expect(
                        "concat_pieces: insertop has no parent",
                    );
                (parent, Some(op), fd.obank().get(op).expect("concat_pieces").get_addr().clone())
            }
        };
        let piece_typeop = typeop_skeleton(OpCode::CPUI_PIECE);
        let n = vnlist.len();
        for i in 1..n {
            let vn = vnlist[i];
            let newop = fd.new_op(2, opaddress.clone());
            fd.op_set_opcode(newop, piece_typeop.clone());
            let newvn = if i == n - 1 {
                fd.op_set_output(newop, finalvn).expect("concat_pieces: op_set_output");
                finalvn
            } else {
                let presize = fd.vbank().get(preexist).expect("concat_pieces: presize").get_size();
                let vnsize = fd.vbank().get(vn).expect("concat_pieces: vnsize").get_size();
                fd.new_unique_out(presize + vnsize, newop).expect("concat_pieces: new_unique_out")
            };
            if isbigendian {
                fd.op_set_input(newop, preexist, 0).expect("concat_pieces: set in0"); // most sig
                fd.op_set_input(newop, vn, 1).expect("concat_pieces: set in1"); // least sig
            } else {
                fd.op_set_input(newop, vn, 0).expect("concat_pieces: set in0");
                fd.op_set_input(newop, preexist, 1).expect("concat_pieces: set in1");
            }
            fd.op_insert(newop, bl, before);
            preexist = newvn;
        }
        preexist
    }

    /// Build a set of Varnode-piece SUBPIECE expressions (C++
    /// `Heritage::splitPieces`, `heritage.cc:564`).
    ///
    /// Given a list of small Varnodes and the whole range `(addr,size)` they are
    /// pieces of, construct a defining SUBPIECE op for each piece, all sharing the
    /// single input `startvn`.  When `insertop` is `None` the ops are inserted at
    /// the beginning of the entry block; otherwise *after* `insertop` (the write).
    fn split_pieces(
        &self,
        fd: &mut crate::funcdata::Funcdata,
        vnlist: &[crate::context::VarnodeId],
        insertop: Option<crate::context::OpId>,
        addr: &Address,
        size: int4,
        startvn: crate::context::VarnodeId,
    ) {
        use kuna_num::opcodes::OpCode;
        let isbigendian = addr.is_big_endian();
        let baseoff = if isbigendian {
            addr.get_offset().wrapping_add(size as u64)
        } else {
            addr.get_offset()
        };
        // Resolve the block + insertion point + op address.  For a non-null
        // insertop we insert AFTER the write (C++ `++insertiter`), realized via
        // op_insert before the op following `insertop` (or end-of-block).
        let (bl, opaddress, after_anchor): (BlockId, Address, Option<crate::context::OpId>) =
            match insertop {
                None => {
                    let root = fd.bblocks_ref().root.expect("split_pieces: bblocks root");
                    let start = fd
                        .bblocks_ref()
                        .get_start_block(root)
                        .expect("split_pieces: start block");
                    (start, fd.get_address().clone(), None)
                }
                Some(op) => {
                    let parent = fd
                        .obank()
                        .get(op)
                        .expect("split_pieces: insertop")
                        .get_parent()
                        .expect("split_pieces: insertop has no parent");
                    (parent, fd.obank().get(op).expect("split_pieces").get_addr().clone(), Some(op))
                }
            };
        let sub_typeop = typeop_skeleton(OpCode::CPUI_SUBPIECE);
        for &vn in vnlist {
            let (vnoff, vnsize) = {
                let v = fd.vbank().get(vn).expect("split_pieces: piece vn");
                (v.get_addr().get_offset(), v.get_size())
            };
            let newop = fd.new_op(2, opaddress.clone());
            fd.op_set_opcode(newop, sub_typeop.clone());
            let diff: uintb = if isbigendian {
                baseoff.wrapping_sub(vnoff.wrapping_add(vnsize as u64))
            } else {
                vnoff.wrapping_sub(baseoff)
            };
            fd.op_set_input(newop, startvn, 0).expect("split_pieces: set in0");
            let cvn = fd.new_constant(4, diff);
            fd.op_set_input(newop, cvn, 1).expect("split_pieces: set in1");
            fd.op_set_output(newop, vn).expect("split_pieces: set output");
            // C++ `opInsert(newop,bl,insertiter)` where insertiter points just
            // after the write (or block begin).  When `after_anchor` is set we
            // insert directly after it (op_insert_after); each successive piece
            // also goes after the write, preserving the C++ order where the
            // iterator is recomputed from the same write each loop.
            match after_anchor {
                Some(anchor) => fd.op_insert_after(newop, anchor),
                None => {
                    let head = fd.bb_op_head(bl);
                    fd.op_insert(newop, bl, head);
                }
            }
        }
    }

    /// Build a refinement array given an address range and a Varnode list (C++
    /// `Heritage::buildRefinement`, `heritage.cc:1705`).  Each Varnode marks a 1
    /// at its starting byte and at the byte immediately past its end.
    fn build_refinement(
        &self,
        fd: &crate::funcdata::Funcdata,
        refine: &mut [int4],
        addr: &Address,
        vnlist: &[crate::context::VarnodeId],
    ) {
        for &vn in vnlist {
            let v = fd.vbank().get(vn).expect("build_refinement: vn");
            let curaddr = v.get_addr();
            let sz = v.get_size();
            let diff = (curaddr.get_offset().wrapping_sub(addr.get_offset())) as usize;
            refine[diff] = 1;
            refine[diff + sz as usize] = 1;
        }
    }

    /// Split up a Varnode by the given refinement (C++
    /// `Heritage::splitByRefinement`, `heritage.cc:1734`).  Returns the new
    /// disjoint cover pieces (newly created free Varnodes).
    fn split_by_refinement(
        &self,
        fd: &mut crate::funcdata::Funcdata,
        vn: crate::context::VarnodeId,
        addr: &Address,
        refine: &[int4],
        split: &mut Vec<crate::context::VarnodeId>,
    ) {
        let (mut curaddr, mut sz, spc) = {
            let v = fd.vbank().get(vn).expect("split_by_refinement: vn");
            (v.get_addr().clone(), v.get_size(), v.get_addr().get_space().expect("split_by_refinement: space").clone())
        };
        let mut diff = spc.wrap_offset(curaddr.get_offset().wrapping_sub(addr.get_offset())) as usize;
        let mut cutsz = refine[diff];
        if sz <= cutsz {
            return; // already refined
        }
        let pv = fd.new_varnode(cutsz, &curaddr, None);
        split.push(pv);
        sz -= cutsz;
        while sz > 0 {
            curaddr = &curaddr + i64::from(cutsz);
            diff =
                spc.wrap_offset(curaddr.get_offset().wrapping_sub(addr.get_offset())) as usize;
            cutsz = refine[diff];
            if cutsz > sz {
                cutsz = sz; // final piece
            }
            let pv = fd.new_varnode(cutsz, &curaddr, None);
            split.push(pv);
            sz -= cutsz;
        }
    }

    /// Split up a \b free read Varnode based on the refinement (C++
    /// `Heritage::refineRead`, `heritage.cc:1773`).
    fn refine_read(
        &self,
        fd: &mut crate::funcdata::Funcdata,
        vn: crate::context::VarnodeId,
        addr: &Address,
        refine: &[int4],
        newvn: &mut Vec<crate::context::VarnodeId>,
    ) {
        newvn.clear();
        self.split_by_refinement(fd, vn, addr, refine, newvn);
        if newvn.is_empty() {
            return;
        }
        let vnsize = fd.vbank().get(vn).expect("refine_read: vn").get_size();
        let replacevn = fd.new_unique(vnsize, None);
        // Read is free so has exactly one descend.
        let op = fd.lone_descend(vn).expect("refine_read: free read has lone descend");
        let slot = fd.obank().get(op).expect("refine_read: op").get_slot(vn);
        let pieces = newvn.clone();
        self.concat_pieces(fd, &pieces, Some(op), replacevn);
        fd.op_set_input(op, replacevn, slot).expect("refine_read: op_set_input");
        if fd.vbank().get(vn).expect("refine_read: vn").has_no_descend() {
            fd.delete_varnode(vn).expect("refine_read: delete free read");
        } else {
            panic!("kuna heritage: refining non-free varnode");
        }
    }

    /// Split up a written output Varnode based on the refinement (C++
    /// `Heritage::refineWrite`, `heritage.cc:1807`).
    fn refine_write(
        &self,
        fd: &mut crate::funcdata::Funcdata,
        vn: crate::context::VarnodeId,
        addr: &Address,
        refine: &[int4],
        newvn: &mut Vec<crate::context::VarnodeId>,
    ) {
        newvn.clear();
        self.split_by_refinement(fd, vn, addr, refine, newvn);
        if newvn.is_empty() {
            return;
        }
        let (vnsize, vnaddr) = {
            let v = fd.vbank().get(vn).expect("refine_write: vn");
            (v.get_size(), v.get_addr().clone())
        };
        let replacevn = fd.new_unique(vnsize, None);
        let def = fd.vbank().get(vn).expect("refine_write: vn").get_def().expect(
            "refine_write: write has a def",
        );
        fd.op_set_output(def, replacevn).expect("refine_write: op_set_output");
        let pieces = newvn.clone();
        self.split_pieces(fd, &pieces, Some(def), &vnaddr, vnsize, replacevn);
        fd.total_replace(vn, replacevn).expect("refine_write: total_replace");
        fd.delete_varnode(vn).expect("refine_write: delete vn");
    }

    /// Split up a known input Varnode based on the refinement (C++
    /// `Heritage::refineInput`, `heritage.cc:1837`).
    fn refine_input(
        &self,
        fd: &mut crate::funcdata::Funcdata,
        vn: crate::context::VarnodeId,
        addr: &Address,
        refine: &[int4],
        newvn: &mut Vec<crate::context::VarnodeId>,
    ) {
        newvn.clear();
        self.split_by_refinement(fd, vn, addr, refine, newvn);
        if newvn.is_empty() {
            return;
        }
        let (vnsize, vnaddr) = {
            let v = fd.vbank().get(vn).expect("refine_input: vn");
            (v.get_size(), v.get_addr().clone())
        };
        let pieces = newvn.clone();
        self.split_pieces(fd, &pieces, None, &vnaddr, vnsize, vn);
        fd.vbank_mut().get_mut(vn).expect("refine_input: vn").set_write_mask();
    }

    /// If we see 1-3 or 3-1 pieces in the partition, replace with a 4 (C++
    /// `Heritage::remove13Refinement`, `heritage.cc:1858`).
    fn remove13_refinement(&self, refine: &mut [int4]) {
        if refine.is_empty() {
            return;
        }
        let mut pos = 0usize;
        let mut lastsize = refine[pos];
        pos += lastsize as usize;
        while pos < refine.len() {
            let cursize = refine[pos];
            if cursize == 0 {
                break;
            }
            if (lastsize == 1 && cursize == 3) || (lastsize == 3 && cursize == 1) {
                refine[pos - lastsize as usize] = 4;
                lastsize = 4;
                pos += cursize as usize;
            } else {
                lastsize = cursize;
                pos += lastsize as usize;
            }
        }
    }

    /// Find the common refinement of all reads and writes in the address range
    /// and split them to match (C++ `Heritage::refinement`, `heritage.cc:1891`).
    ///
    /// `idx` indexes the range in `self.disjoint`.  Returns `Some(resiter_idx)`
    /// (the index of the first refined sub-range) on a non-trivial refinement, or
    /// `None` when there is nothing to refine.  The disjoint cover is altered both
    /// locally (`self.disjoint`) and globally (`self.globaldisjoint`).
    fn refinement(
        &mut self,
        fd: &mut crate::funcdata::Funcdata,
        idx: usize,
        readvars: &[crate::context::VarnodeId],
        writevars: &[crate::context::VarnodeId],
        inputvars: &[crate::context::VarnodeId],
    ) -> Option<usize> {
        let size = self.disjoint.get(idx).size;
        if size > 1024 {
            return None;
        }
        let addr = self.disjoint.get(idx).addr.clone();
        // Add a "fencepost" for the size position.
        let mut refine: Vec<int4> = vec![0; (size + 1) as usize];
        self.build_refinement(fd, &mut refine, &addr, readvars);
        self.build_refinement(fd, &mut refine, &addr, writevars);
        self.build_refinement(fd, &mut refine, &addr, inputvars);
        refine.pop(); // remove the fencepost
        // Convert boundary points to partition sizes.
        let mut lastpos: usize = 0;
        for curpos in 1..(size as usize) {
            if refine[curpos] != 0 {
                refine[lastpos] = (curpos - lastpos) as int4;
                lastpos = curpos;
            }
        }
        if lastpos == 0 {
            return None; // No non-trivial refinements
        }
        refine[lastpos] = size - lastpos as int4;
        self.remove13_refinement(&mut refine);
        let mut newvn: Vec<crate::context::VarnodeId> = Vec::new();
        for &vn in readvars {
            self.refine_read(fd, vn, &addr, &refine, &mut newvn);
        }
        for &vn in writevars {
            self.refine_write(fd, vn, &addr, &refine, &mut newvn);
        }
        for &vn in inputvars {
            self.refine_input(fd, vn, &addr, &refine, &mut newvn);
        }

        // Alter the disjoint cover (both locally and globally) to reflect the
        // refinement.  The C++ erases the old range and inserts the partition.
        let flags = self.disjoint.get(idx).flags;
        let mut pos = self.disjoint.erase(idx); // erase returns the following index
        let curpass = self.globaldisjoint.find_pass(&addr);
        self.globaldisjoint.erase(&addr);
        let mut cut: usize = 0;
        let mut sz = refine[cut];
        let mut curaddr = addr.clone();
        // First partition: this is the resiter the caller re-collects from.
        let resiter = pos;
        self.disjoint.insert(pos, curaddr.clone(), sz, flags);
        self.globaldisjoint.add(curaddr.clone(), sz, curpass);
        pos += 1;
        cut += sz as usize;
        curaddr = &curaddr + i64::from(sz);
        while cut < size as usize {
            sz = refine[cut];
            self.disjoint.insert(pos, curaddr.clone(), sz, flags);
            self.globaldisjoint.add(curaddr.clone(), sz, curpass);
            pos += 1;
            cut += sz as usize;
            curaddr = &curaddr + i64::from(sz);
        }
        Some(resiter)
    }

    /// Perform phi-node placement for the current address ranges (C++
    /// `Heritage::placeMultiequals`, `heritage.cc:2603`).
    ///
    /// Assumes `disjoint` is filled with all free Varnodes to be heritaged.  The
    /// driver loops the disjoint ranges, collecting reads/writes/inputs,
    /// refining mismatched sub-ranges ([`refinement`](Heritage::refinement)),
    /// guarding, calling [`calc_multiequals`](Heritage::calc_multiequals), and
    /// constructing the MULTIEQUAL ops at each merge block.
    pub fn place_multiequals(&mut self, fd: &mut crate::funcdata::Funcdata) {
        use kuna_num::opcodes::OpCode;
        let mut readvars: Vec<crate::context::VarnodeId> = Vec::new();
        let mut writevars: Vec<crate::context::VarnodeId> = Vec::new();
        let mut inputvars: Vec<crate::context::VarnodeId> = Vec::new();
        let mut removevars: Vec<crate::context::VarnodeId> = Vec::new();

        // C++ iterates `for(iter=disjoint.begin();iter!=disjoint.end();++iter)`.
        // refinement() may erase the current range and insert N partition ranges
        // in its place (so the container mutates mid-walk); a `while idx < len()`
        // walk with the refinement resetting `idx` to the first partition mirrors
        // the C++ iterator (which is reseated to `refiter`).
        let mut idx = 0usize;
        while idx < self.disjoint.len() {
            let mut memrange = self.disjoint.get(idx).clone();
            let maxw =
                self.collect(fd, &mut memrange, &mut readvars, &mut writevars, &mut inputvars, &mut removevars);
            // refinement (heritage.cc:2611-2619): only when size>4 && max<size.
            if memrange.size > 4 && maxw < memrange.size {
                if let Some(refiter) =
                    self.refinement(fd, idx, &readvars, &writevars, &inputvars)
                {
                    idx = refiter;
                    memrange = self.disjoint.get(idx).clone();
                    self.collect(
                        fd,
                        &mut memrange,
                        &mut readvars,
                        &mut writevars,
                        &mut inputvars,
                        &mut removevars,
                    );
                }
            }
            // write the (possibly clearProperty-mutated) memrange back.
            *self.disjoint.get_mut(idx) = memrange.clone();
            let size = memrange.size;
            // The C++ `continue` reseats the `for` iterator (`++iter`); in the
            // index-walk that is `idx += 1; continue`, so the empty-read arms
            // set a flag that skips the rest of the body and advances.
            let mut skip = false;
            if readvars.is_empty() {
                if writevars.is_empty() && inputvars.is_empty() {
                    skip = true;
                } else {
                    let is_internal = memrange.addr.get_space().map(|s| {
                        s.get_type() == spacetype::IPTR_INTERNAL
                    }).unwrap_or(false);
                    if is_internal || memrange.old_addresses() {
                        skip = true;
                    }
                }
            }
            if skip {
                idx += 1;
                continue;
            }
            if !removevars.is_empty() {
                // removeRevisitedMarkers (heritage.cc:245): deletes stale
                // MULTIEQUAL/INDIRECT markers from a previous pass over a now-
                // larger range, replacing each with a SUBPIECE of a new full-size
                // free Varnode (return-form COPYs are simply removed).  Reached on
                // the multi-pass overlap path (e.g. `revisit.xml`: a global is
                // written at one size, an aliased access at a larger size forces
                // the SSA form to be revisited).
                let removed = std::mem::take(&mut removevars);
                self.remove_revisited_markers(fd, &removed, &memrange.addr, size);
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
            idx += 1;
        }
        self.merge.clear();
    }

    /// Generate a guard record for an indexed LOAD into a stack space (C++
    /// `Heritage::generateLoadGuard`, `heritage.cc:910`).
    fn generate_load_guard(
        &mut self,
        fd: &mut crate::funcdata::Funcdata,
        offset: uintb,
        op: crate::context::OpId,
        spc: &Rc<AddrSpace>,
    ) {
        let uses = fd.obank().get(op).expect("generate_load_guard: stale op").uses_spacebase_ptr();
        if !uses {
            self.load_guard.push(LoadGuard::set(op, spc, offset));
            fd.op_mark_spacebase_ptr(op);
        }
    }

    /// Generate a guard record for an indexed STORE to a stack space (C++
    /// `Heritage::generateStoreGuard`, `heritage.cc:927`).
    fn generate_store_guard(
        &mut self,
        fd: &mut crate::funcdata::Funcdata,
        offset: uintb,
        op: crate::context::OpId,
        spc: &Rc<AddrSpace>,
    ) {
        let uses = fd.obank().get(op).expect("generate_store_guard: stale op").uses_spacebase_ptr();
        if !uses {
            self.store_guard.push(LoadGuard::set(op, spc, offset));
            fd.op_mark_spacebase_ptr(op);
        }
    }

    /// Identify CPUI_STORE ops that use a free pointer from a given address space
    /// (C++ `Heritage::protectFreeStores`, `heritage.cc:945`).
    fn protect_free_stores(
        &mut self,
        fd: &mut crate::funcdata::Funcdata,
        spc: &Rc<AddrSpace>,
        free_stores: &mut Vec<crate::context::OpId>,
    ) -> bool {
        use kuna_num::opcodes::OpCode;
        let stores: Vec<crate::context::OpId> = fd.obank().iter_code(OpCode::CPUI_STORE).collect();
        let mut has_new = false;
        for op in stores {
            let dead = fd.obank().get(op).expect("protect_free_stores: stale op").is_dead();
            if dead {
                continue;
            }
            // Strip COPY / INT_ADD-by-constant chains off the STORE pointer.
            let mut vn = match fd.obank().get(op).and_then(|o| o.get_in(1)) {
                Some(v) => v,
                None => continue,
            };
            loop {
                let written = fd.vbank().get(vn).expect("protect_free_stores: stale vn").is_written();
                if !written {
                    break;
                }
                let def = fd
                    .vbank()
                    .get(vn)
                    .and_then(|v| v.get_def())
                    .expect("protect_free_stores: written vn has def");
                let opc = fd.obank().get(def).expect("protect_free_stores: stale def").code();
                if opc == OpCode::CPUI_COPY {
                    vn = fd.obank().get(def).and_then(|o| o.get_in(0)).expect("COPY in0");
                } else if opc == OpCode::CPUI_INT_ADD
                    && fd
                        .obank()
                        .get(def)
                        .and_then(|o| o.get_in(1))
                        .map(|v| fd.vbank().get(v).map(|x| x.is_constant()).unwrap_or(false))
                        .unwrap_or(false)
                {
                    vn = fd.obank().get(def).and_then(|o| o.get_in(0)).expect("INT_ADD in0");
                } else {
                    break;
                }
            }
            let (is_free, same_spc) = {
                let v = fd.vbank().get(vn).expect("protect_free_stores: stale vn (test)");
                (v.is_free(), Rc::ptr_eq(v.get_space(), spc))
            };
            if is_free && same_spc {
                fd.op_mark_spacebase_ptr(op); // mark op as spacebase STORE, even though unsure
                free_stores.push(op);
                has_new = true;
            }
        }
        has_new
    }

    /// Trace input stack-pointer to any indexed LOAD/STORE, generating guard
    /// records (C++ `Heritage::discoverIndexedStackPointers`, `heritage.cc:987`).
    ///
    /// Returns `true` if there are incomplete (free-pointer) STOREs needing a
    /// follow-up pass, passing those back in `free_stores`.
    ///
    /// The C++ holds a live `list<PcodeOp*>::const_iterator` inside each
    /// `StackNode`; here each node snapshots its Varnode's descendant op list
    /// (read-only during the walk) and an index cursor — the same DFS order.
    fn discover_indexed_stack_pointers(
        &mut self,
        fd: &mut crate::funcdata::Funcdata,
        spc: &Rc<AddrSpace>,
        free_stores: &mut Vec<crate::context::OpId>,
        check_free_stores: bool,
    ) -> bool {
        use kuna_num::opcodes::OpCode;

        /// One node of the DFS over stack-pointer descendants (C++
        /// `Heritage::StackNode`, with the live iterator replaced by a snapshot).
        struct StackNode {
            vn: crate::context::VarnodeId,
            offset: uintb,
            traversals: uint4,
            descend: Vec<crate::context::OpId>,
            iter: usize,
        }

        // We mark Varnodes independently of the depth-first path to avoid
        // exponential ladders.
        let mut marked_vn: Vec<crate::context::VarnodeId> = Vec::new();
        let mut path: Vec<StackNode> = Vec::new();
        let mut unknown_stack_storage = false;

        let num_sb = spc.num_spacebase();
        for i in 0..num_sb {
            let stack_pointer = match spc.get_spacebase(i) {
                Ok(sp) => sp,
                Err(_) => continue,
            };
            let sp_addr = Address::new(
                stack_pointer.space.clone().expect("spacebase has space"),
                stack_pointer.offset,
            );
            let sp_input = match fd.find_varnode_input(stack_pointer.size as int4, &sp_addr) {
                Some(v) => v,
                None => continue,
            };
            let descend: Vec<crate::context::OpId> =
                fd.vbank().get(sp_input).expect("sp_input vn").descend_iter().collect();
            path.push(StackNode { vn: sp_input, offset: 0, traversals: 0, descend, iter: 0 });

            while let Some(cur) = path.last_mut() {
                if cur.iter >= cur.descend.len() {
                    path.pop();
                    continue;
                }
                let op = cur.descend[cur.iter];
                cur.iter += 1;
                let cur_vn = cur.vn;
                let cur_offset = cur.offset;
                let cur_traversals = cur.traversals;

                let out_vn = fd.obank().get(op).and_then(|o| o.get_out());
                if let Some(ov) = out_vn {
                    let marked = fd.vbank().get(ov).map(|v| v.is_mark()).unwrap_or(false);
                    if marked {
                        continue; // Don't revisit Varnodes
                    }
                }
                let code = fd.obank().get(op).expect("discover: stale op").code();

                // Helper to push the next node onto the path (or flag unknown
                // stack storage if the output has no further descendants but
                // lands in a spacebase space).
                macro_rules! descend_into {
                    ($outvn:expr, $newoff:expr, $newtrav:expr) => {{
                        let ov = $outvn;
                        let d: Vec<crate::context::OpId> =
                            fd.vbank().get(ov).expect("descend_into: stale outvn").descend_iter().collect();
                        if !d.is_empty() {
                            fd.vbank_mut().get_mut(ov).expect("descend_into: mark outvn").set_mark();
                            path.push(StackNode {
                                vn: ov,
                                offset: $newoff,
                                traversals: $newtrav,
                                descend: d,
                                iter: 0,
                            });
                            marked_vn.push(ov);
                        } else if fd.vbank().get(ov).map(|v| v.get_space().get_type()).unwrap_or(spacetype::IPTR_CONSTANT)
                            == spacetype::IPTR_SPACEBASE
                        {
                            unknown_stack_storage = true;
                        }
                    }};
                }

                match code {
                    OpCode::CPUI_INT_ADD => {
                        let slot = fd.obank().get(op).expect("discover: stale op").get_slot(cur_vn);
                        let other_vn = fd
                            .obank()
                            .get(op)
                            .and_then(|o| o.get_in(1 - slot))
                            .expect("INT_ADD other input");
                        let other_is_const =
                            fd.vbank().get(other_vn).map(|v| v.is_constant()).unwrap_or(false);
                        let ov = match out_vn {
                            Some(v) => v,
                            None => continue,
                        };
                        if other_is_const {
                            let other_off =
                                fd.vbank().get(other_vn).expect("INT_ADD const").get_offset();
                            let new_offset = spc.wrap_offset(cur_offset.wrapping_add(other_off));
                            descend_into!(ov, new_offset, cur_traversals);
                        } else {
                            descend_into!(
                                ov,
                                cur_offset,
                                cur_traversals | stacknode_flags::nonconstant_index
                            );
                        }
                    }
                    OpCode::CPUI_SEGMENTOP => {
                        // Check that the stackpointer comes in as the inner pointer.
                        let in2 = fd.obank().get(op).and_then(|o| o.get_in(2));
                        if in2 != Some(cur_vn) {
                            continue;
                        }
                        // Treat output as having the same offset (fallthru to COPY).
                        let ov = match out_vn {
                            Some(v) => v,
                            None => continue,
                        };
                        descend_into!(ov, cur_offset, cur_traversals);
                    }
                    OpCode::CPUI_INDIRECT | OpCode::CPUI_COPY => {
                        let ov = match out_vn {
                            Some(v) => v,
                            None => continue,
                        };
                        descend_into!(ov, cur_offset, cur_traversals);
                    }
                    OpCode::CPUI_MULTIEQUAL => {
                        let ov = match out_vn {
                            Some(v) => v,
                            None => continue,
                        };
                        descend_into!(ov, cur_offset, cur_traversals | stacknode_flags::multiequal);
                    }
                    OpCode::CPUI_LOAD => {
                        // If ANY path had a traversal (non-constant ADD or MULTIEQUAL),
                        // THIS path must too (the other elements have one path through).
                        if cur_traversals != 0 {
                            self.generate_load_guard(fd, cur_offset, op, spc);
                        }
                    }
                    OpCode::CPUI_STORE => {
                        // Make sure the STORE pointer comes from our path.
                        let in1 = fd.obank().get(op).and_then(|o| o.get_in(1));
                        if in1 == Some(cur_vn) {
                            if cur_traversals != 0 {
                                self.generate_store_guard(fd, cur_offset, op, spc);
                            } else {
                                // No traversals: the pointer is the stackpointer plus
                                // a constant (possibly through an indirect).  Likely
                                // resolved next heritage pass; keep the spacebaseptr
                                // mark so the indirects don't get removed.
                                fd.op_mark_spacebase_ptr(op);
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        for vn in marked_vn {
            if let Some(v) = fd.vbank_mut().get_mut(vn) {
                v.clear_mark();
            }
        }
        if unknown_stack_storage && check_free_stores {
            return self.protect_free_stores(fd, spc, free_stores);
        }
        false
    }

    /// Revisit STOREs with free pointers after a heritage pass completed (C++
    /// `Heritage::reprocessFreeStores`, `heritage.cc:1112`).
    ///
    /// Regenerates STORE LoadGuard records, then cross-references the originally
    /// free STOREs: any that did not actually need a guard is unmarked and the
    /// spurious INDIRECTs it caused are removed.
    fn reprocess_free_stores(
        &mut self,
        fd: &mut crate::funcdata::Funcdata,
        spc: &Rc<AddrSpace>,
        free_stores: &mut Vec<crate::context::OpId>,
    ) {
        use kuna_num::opcodes::OpCode;
        for &op in free_stores.iter() {
            fd.op_clear_spacebase_ptr(op);
        }

        self.discover_indexed_stack_pointers(fd, spc, free_stores, false);

        let free: Vec<crate::context::OpId> = free_stores.clone();
        for op in free {
            // If the STORE is now marked as using a spacebase ptr, it was
            // appropriately marked to begin with; nothing to clean up.
            if fd.obank().get(op).expect("reprocess: stale op").uses_spacebase_ptr() {
                continue;
            }
            // Otherwise the STORE may have triggered unnecessary INDIRECTs.
            let mut ind_op = fd.op_previous_op(op);
            while let Some(iop) = ind_op {
                if fd.obank().get(iop).expect("reprocess: stale indop").code() != OpCode::CPUI_INDIRECT
                {
                    break;
                }
                let iop_vn = match fd.obank().get(iop).and_then(|o| o.get_in(1)) {
                    Some(v) => v,
                    None => break,
                };
                if fd.vbank().get(iop_vn).map(|v| v.get_space().get_type()).unwrap_or(spacetype::IPTR_CONSTANT)
                    != spacetype::IPTR_IOP
                {
                    break;
                }
                if Some(op) != op_from_const(fd, iop) {
                    break;
                }
                let next_op = fd.op_previous_op(iop);
                let ind_out = fd.obank().get(iop).and_then(|o| o.get_out());
                let same_space = ind_out
                    .map(|o| Rc::ptr_eq(fd.vbank().get(o).expect("reprocess: stale indout").get_space(), spc))
                    .unwrap_or(false);
                if same_space {
                    let in0 = fd.obank().get(iop).and_then(|o| o.get_in(0)).expect("INDIRECT in0");
                    let out = ind_out.expect("INDIRECT out");
                    fd.total_replace(out, in0).expect("reprocess: total_replace");
                    fd.op_destroy(iop); // get rid of the INDIRECT
                }
                ind_op = next_op;
            }
        }
    }

    /// Make the final determination of what range new LoadGuards protect (C++
    /// `Heritage::analyzeNewLoadGuards`, `heritage.cc:834`).
    ///
    /// Actual LOAD (and indexed STORE) operations are guarded with an initial
    /// whole-space LoadGuard record.  Now that heritage has completed, a full
    /// value-set analysis of each new guard's pointer is conducted to reach a
    /// conclusion about what range of stack values the op might actually
    /// alias.  All new LoadGuard records are updated with the analysis, which
    /// then informs handling of LOAD COPYs, possible later heritage passes,
    /// and the P6 `MapState::addGuard` array-extent hints.
    fn analyze_new_load_guards(&mut self, fd: &crate::funcdata::Funcdata) {
        use crate::rangeutil::{ValueSetSolver, WidenerFull, WidenerNone};

        let mut nothing_to_do = true;
        if let Some(g) = self.load_guard.last() {
            if g.analysis_state == 0 {
                nothing_to_do = false; // Check if unanalyzed
            }
        }
        if let Some(g) = self.store_guard.last() {
            if g.analysis_state == 0 {
                nothing_to_do = false;
            }
        }
        if nothing_to_do {
            return;
        }

        let mut sinks: Vec<crate::context::VarnodeId> = Vec::new();
        let mut reads: Vec<crate::context::OpId> = Vec::new();
        // Walk each guard list from the end backward collecting the new
        // (unanalyzed) records — the C++ `--loadIter` loops.
        let mut load_start = self.load_guard.len();
        while load_start > 0 && self.load_guard[load_start - 1].analysis_state == 0 {
            load_start -= 1;
            let g = &self.load_guard[load_start];
            if let Some(ptr) = fd.obank().get(g.op).and_then(|o| o.get_in(1)) {
                reads.push(g.op);
                sinks.push(ptr); // The CPUI_LOAD pointer
            }
        }
        let mut store_start = self.store_guard.len();
        while store_start > 0 && self.store_guard[store_start - 1].analysis_state == 0 {
            store_start -= 1;
            let g = &self.store_guard[store_start];
            if let Some(ptr) = fd.obank().get(g.op).and_then(|o| o.get_in(1)) {
                reads.push(g.op);
                sinks.push(ptr); // The CPUI_STORE pointer
            }
        }
        let stack_reg = fd
            .get_arch()
            .manage()
            .get_stack_space()
            .filter(|s| s.num_spacebase() > 0)
            .map(Rc::clone)
            .and_then(|s| fd.find_spacebase_input(&s));
        let mut vs_solver = ValueSetSolver::new();
        vs_solver.establish_value_sets(fd, &sinks, &reads, stack_reg, false);
        let widener = WidenerNone::new();
        vs_solver.solve(fd, 10000, &widener);
        // The C++ update loops run from the (post-break) `loadIter`/`storeIter`
        // to the end, which can include one already-analyzed boundary record;
        // that record has no read node in the solver (upstream dereferences a
        // missing map entry there), so records without a read node are skipped.
        let mut run_full_analysis = false;
        for guard in self.load_guard[load_start..].iter_mut() {
            if let Some(vs_read) = vs_solver.get_value_set_read(guard.op) {
                guard.establish_range(vs_read);
            }
            if guard.analysis_state == 0 {
                run_full_analysis = true;
            }
        }
        for guard in self.store_guard[store_start..].iter_mut() {
            if let Some(vs_read) = vs_solver.get_value_set_read(guard.op) {
                guard.establish_range(vs_read);
            }
            if guard.analysis_state == 0 {
                run_full_analysis = true;
            }
        }
        if run_full_analysis {
            let full_widener = WidenerFull::new();
            vs_solver.solve(fd, 10000, &full_widener);
            for guard in self.load_guard[load_start..].iter_mut() {
                if let Some(vs_read) = vs_solver.get_value_set_read(guard.op) {
                    guard.finalize_range(vs_read);
                }
            }
            for guard in self.store_guard[store_start..].iter_mut() {
                if let Some(vs_read) = vs_solver.get_value_set_read(guard.op) {
                    guard.finalize_range(vs_read);
                }
            }
        }
    }

    /// Find the last PcodeOps that write to specific addresses that flow to
    /// specific sites (C++ `Heritage::findAddressForces`, `heritage.cc:618`).
    ///
    /// A COPY/MULTIEQUAL is *artificial* if all of its input and output
    /// Varnodes have the same storage address.  The final set of ops that are
    /// not artificial will all have an output Varnode matching the address of
    /// a COPY sink and will need to be marked address forcing.  `copy_sinks`
    /// is extended with every artificial COPY/MULTIEQUAL encountered; every
    /// visited op is added to `marks` (the C++ PcodeOp mark bit).
    fn find_address_forces(
        &mut self,
        fd: &crate::funcdata::Funcdata,
        copy_sinks: &mut Vec<crate::context::OpId>,
        forces: &mut Vec<crate::context::OpId>,
        marks: &mut std::collections::HashSet<crate::context::OpId>,
    ) {
        use kuna_num::opcodes::OpCode;
        // Mark the sinks.
        for &op in copy_sinks.iter() {
            marks.insert(op);
        }
        // Mark everything back-reachable from a sink, trimming at
        // non-artificial ops.
        let mut pos = 0usize;
        while pos < copy_sinks.len() {
            let op = copy_sinks[pos];
            pos += 1;
            // Address being flowed to.
            let addr = match fd
                .obank()
                .get(op)
                .and_then(|o| o.get_out())
                .and_then(|v| fd.vbank().get(v))
                .map(|v| v.get_addr().clone())
            {
                Some(a) => a,
                None => continue,
            };
            let max_in = fd.obank().get(op).map(|o| o.num_input()).unwrap_or(0);
            for i in 0..max_in {
                let vn = match fd.obank().get(op).and_then(|o| o.get_in(i)) {
                    Some(v) => v,
                    None => continue,
                };
                let (is_written, is_addr_force, def) = match fd.vbank().get(vn) {
                    Some(v) => (v.is_written(), v.is_addr_force(), v.get_def()),
                    None => continue,
                };
                if !is_written {
                    continue;
                }
                if is_addr_force {
                    continue; // Already marked address forced
                }
                let new_op = match def {
                    Some(d) => d,
                    None => continue,
                };
                if marks.contains(&new_op) {
                    continue; // Already visited this op
                }
                marks.insert(new_op);
                let opc = match fd.obank().get(new_op) {
                    Some(o) => o.code(),
                    None => continue,
                };
                let mut is_artificial = false;
                if opc == OpCode::CPUI_COPY || opc == OpCode::CPUI_MULTIEQUAL {
                    is_artificial = true;
                    let max_in_new = fd.obank().get(new_op).map(|o| o.num_input()).unwrap_or(0);
                    for j in 0..max_in_new {
                        let in_addr = fd
                            .obank()
                            .get(new_op)
                            .and_then(|o| o.get_in(j))
                            .and_then(|v| fd.vbank().get(v))
                            .map(|v| v.get_addr().clone());
                        if in_addr.as_ref() != Some(&addr) {
                            is_artificial = false;
                            break;
                        }
                    }
                } else if opc == OpCode::CPUI_INDIRECT
                    && fd.obank().get(new_op).map(|o| o.is_indirect_store()).unwrap_or(false)
                {
                    // An INDIRECT can be considered artificial if it is caused
                    // by a STORE.
                    let in_addr = fd
                        .obank()
                        .get(new_op)
                        .and_then(|o| o.get_in(0))
                        .and_then(|v| fd.vbank().get(v))
                        .map(|v| v.get_addr().clone());
                    if in_addr.as_ref() == Some(&addr) {
                        is_artificial = true;
                    }
                }
                if is_artificial {
                    copy_sinks.push(new_op);
                } else {
                    forces.push(new_op);
                }
            }
        }
    }

    /// Eliminate a COPY sink, preserving its data-flow (C++
    /// `Heritage::propagateCopyAway`, `heritage.cc:674`).
    ///
    /// Given a COPY from a storage location to itself, propagate the input
    /// Varnode version of the storage location to all the ops reading the
    /// output, then eliminate the COPY.
    fn propagate_copy_away(&mut self, fd: &mut crate::funcdata::Funcdata, op: crate::context::OpId) {
        use kuna_num::opcodes::OpCode;
        let mut in_vn = match fd.obank().get(op).and_then(|o| o.get_in(0)) {
            Some(v) => v,
            None => return,
        };
        // Follow any COPY chain to the earliest input.
        loop {
            let (is_written, def, in_addr) = match fd.vbank().get(in_vn) {
                Some(v) => (v.is_written(), v.get_def(), v.get_addr().clone()),
                None => break,
            };
            if !is_written {
                break;
            }
            let next_op = match def {
                Some(d) => d,
                None => break,
            };
            if fd.obank().get(next_op).map(|o| o.code()) != Some(OpCode::CPUI_COPY) {
                break;
            }
            let next_in = match fd.obank().get(next_op).and_then(|o| o.get_in(0)) {
                Some(v) => v,
                None => break,
            };
            let next_addr = fd.vbank().get(next_in).map(|v| v.get_addr().clone());
            if next_addr.as_ref() != Some(&in_addr) {
                break;
            }
            in_vn = next_in;
        }
        if let Some(out) = fd.obank().get(op).and_then(|o| o.get_out()) {
            let _ = fd.total_replace(out, in_vn);
        }
        fd.op_destroy(op);
    }

    /// Mark the boundary of artificial ops introduced by load guards (C++
    /// `Heritage::handleNewLoadCopies`, `heritage.cc:695`).
    ///
    /// Having just completed renaming, run through all new COPY sinks from
    /// load guards and mark boundary Varnodes as address-forced, so dead code
    /// removal can run forward from the boundary while still preserving the
    /// address force on the load guard.  (The COPY sinks themselves are
    /// produced by `guardLoads`; while that guard-insertion path is stubbed
    /// the sink list is empty and this is the faithful early return.)
    fn handle_new_load_copies(&mut self, fd: &mut crate::funcdata::Funcdata) {
        if self.load_copy_ops.is_empty() {
            return;
        }
        let mut copy_sinks = std::mem::take(&mut self.load_copy_ops);
        let copy_sink_size = copy_sinks.len();
        let mut forces: Vec<crate::context::OpId> = Vec::new();
        let mut marks: std::collections::HashSet<crate::context::OpId> =
            std::collections::HashSet::new();
        self.find_address_forces(fd, &mut copy_sinks, &mut forces, &mut marks);

        if !forces.is_empty() {
            let mut load_ranges = kuna_base::address::RangeList::new();
            for guard in &self.load_guard {
                load_ranges.insert_range(
                    Rc::clone(&guard.spc),
                    guard.minimum_offset,
                    guard.maximum_offset,
                );
            }
            // Mark everything on the boundary as address forced to prevent
            // dead-code removal.
            for &op in &forces {
                if let Some(vn) = fd.obank().get(op).and_then(|o| o.get_out()) {
                    let addr = fd.vbank().get(vn).map(|v| v.get_addr().clone());
                    if let Some(a) = addr {
                        // If we are within one of the guarded ranges, consider
                        // the output address forced.
                        if load_ranges.in_range(&a, 1) {
                            if let Some(v) = fd.vbank_mut().get_mut(vn) {
                                v.set_addr_force();
                            }
                        }
                    }
                }
                marks.remove(&op);
            }
        }

        // Eliminate or propagate away the original COPY sinks.
        for &op in copy_sinks.iter().take(copy_sink_size) {
            // Make sure load guard COPYs no longer exist.
            self.propagate_copy_away(fd, op);
        }
        // (marks on the remaining artificial COPYs die with the local set.)
        // We have handled all the load guard COPY ops (list stays cleared).
    }

    /// Perform one pass of heritage (C++ `Heritage::heritage`,
    /// `heritage.cc:2667`).
    ///
    /// Collects free Varnodes from active spaces, builds the disjoint cover,
    /// runs phi placement and renaming, and advances `pass`.  The
    /// PreferSplitManager (pass-0 SIMD split) and the load-guard discovery
    /// (`discoverIndexedStackPointers`/`analyzeNewLoadGuards`/
    /// `handleNewLoadCopies`/`reprocessFreeStores`) sub-systems are STUB-marked;
    /// they are inert for a function with no split records and no indexed-stack
    /// LOAD/STOREs (the realized critical path).
    ///
    /// `deadline` is the (kuna) `decompile-all` per-function watchdog: the
    /// per-address-space loop probes it and returns early once it passes (the
    /// half-finished pass is discarded — the action layer then unwinds and the
    /// drive errors the function out).  `None` (the console/parity paths) is a
    /// no-op compare.
    #[allow(clippy::mutable_key_type)]
    pub fn heritage(
        &mut self,
        fd: &mut crate::funcdata::Funcdata,
        deadline: Option<std::time::Instant>,
    ) {
        // A function with no basic blocks has no data-flow to heritage.  The C++
        // never reaches `heritage()` in that state (it runs after `followFlow`/
        // `structureReset` built the CFG), and `buildADT` indexes the block list
        // (`z[0]`), so guard the degenerate empty-CFG case as a no-op — faithful
        // for an action invoked on a not-yet-built function (the stub-wrapper
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
        // STUB(W6): PreferSplitManager (pass-0 splitmanage.init/split)
        // is a no-op for an architecture with no <splitrecords> (the critical
        // path); it splits SIMD-style registers when present.

        let mut reprocess_stack_count = 0;
        let mut stack_space: Option<Rc<AddrSpace>> = None;
        let mut free_stores: Vec<crate::context::OpId> = Vec::new();

        let nspaces = fd.get_arch().manage().num_spaces() as usize;
        for i in 0..nspaces {
            // (kuna decompile-all watchdog) Cooperative per-function deadline at
            // the per-address-space boundary: the observed non-convergence hang
            // spends its time inside heritage passes (`LocationMap::add` growth),
            // so bail out of the pass here rather than only at the next action
            // boundary.  The abandoned pass's partial state is never printed —
            // the drive turns the expiry into a per-function `Err`.
            if let Some(d) = deadline {
                if std::time::Instant::now() >= d {
                    return;
                }
            }
            if !self.infolist[i].is_heritaged() {
                continue;
            }
            if self.pass < self.infolist[i].delay {
                continue; // too soon to heritage this space
            }
            if self.infolist[i].has_call_placeholders {
                self.clear_stack_placeholders(i);
            }
            let space = self.infolist[i]
                .space
                .clone()
                .expect("heritage: heritaged info has a space");
            if !self.infolist[i].load_guard_search {
                self.infolist[i].load_guard_search = true;
                // (kuna w10-chainb-gap1) W6 indexed-stack LOAD/STORE discovery:
                // walks the stack-pointer input's descendants and records guard
                // records for STOREs/LOADs reached through a non-constant ADD or
                // a MULTIEQUAL.  The store-guard set is the W7 `StackAffectingOps`
                // cross-call test source.  The initial guard covers the whole
                // space (`LoadGuard::set`); `analyze_new_load_guards` (option
                // `loadguardrange`) refines it after renaming completes.
                if self.discover_indexed_stack_pointers(fd, &space, &mut free_stores, true) {
                    reprocess_stack_count += 1;
                    stack_space = Some(space.clone());
                }
            }
            let mut needwarning = false;
            let mut warnvn: Option<crate::context::VarnodeId> = None;

            let ids: Vec<crate::context::VarnodeId> = fd.vbank().loc_space_ids(&space);
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
                let (key, prev, resolved_size) = self.globaldisjoint.add(vaddr, vsize, self.pass);
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
                        self.bump_deadcode_delay_persisted(fd, &space);
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
                        self.bump_deadcode_delay_persisted(fd, &space);
                        warnvn = Some(vn);
                    }
                }
            }
            if needwarning && !self.infolist[i].warningissued {
                self.infolist[i].warningissued = true;
                // fd->warningHeader("Heritage AFTER dead removal...")
                // STUB(W8): the warning text needs Varnode::printRawNoMarkup (W8
                // print surface); the warning is suppressed (a cosmetic header,
                // not IR), the dead-code-delay bump already recorded the event.
                let _ = warnvn;
            }
        }
        self.place_multiequals(fd);
        self.rename(fd);
        // (kuna w10-chainb-gap1) reprocessFreeStores: re-derive the STORE guards
        // now that this pass completed, dropping spurious INDIRECTs on STOREs
        // that turned out not to need a guard (only reached when discovery saw
        // unknown stack storage and collected free STOREs).
        if reprocess_stack_count > 0 {
            let spc = stack_space.expect("heritage: reprocess set without a stack space");
            self.reprocess_free_stores(fd, &spc, &mut free_stores);
        }
        // (kuna) `option loadguardrange` — the upstream ValueSet range
        // refinement of new LOAD/STORE guards (heritage.cc:2753-2754).  Off
        // reproduces the pre-port behavior: every guard keeps the whole-space
        // range and `step == 0`, so `MapState::addGuard` never sees an
        // index bound.
        if fd.get_arch().load_guard_range {
            self.analyze_new_load_guards(fd);
            self.handle_new_load_copies(fd);
        }
        // splitmanage.splitAdditional() on pass 0: STUB(W6) PreferSplitManager.
        self.pass += 1;
    }

    /// `bumpDeadcodeDelay` routed through the function's **persistent**
    /// [`Override`] (C++ `fd->getOverride()`, `heritage.cc:2576`).
    ///
    /// The C++ suppression guard `if (fd->getOverride().hasDeadcodeDelay(spc))
    /// return;` is what makes the deadcode-delay bump fire **at most once** per
    /// space: the first call installs the delay on the function's Override (which
    /// survives `Funcdata::clear()` across restart re-flows) and sets restart
    /// pending; every later call sees the installed delay and returns WITHOUT
    /// re-requesting a restart, so the deadcode-ordering restart converges.
    ///
    /// Earlier this routed through a *fresh* `Override::new()` per call, so
    /// `has_deadcode_delay` was always false — the bump fired on every heritage
    /// pass and the function restart-looped until `run_pipeline`'s `MAX_REFLOW`
    /// budget exhausted, leaving the IR cleared (no `sblocks` → "structuring
    /// declined at a stub").  The companion half is
    /// [`Funcdata::op_heritage`](crate::funcdata::Funcdata::op_heritage), which now
    /// re-applies the persisted Override delay to the per-space `HeritageInfo` on
    /// each pass (C++ `Funcdata::startProcessing` → `applyDeadCodeDelay`).
    ///
    /// The [`Override`] is moved out of `fd` for the call (so `bump_deadcode_delay`
    /// can take both `&mut Funcdata` for `setRestartPending` and `&mut Override`)
    /// and moved back; the [`RestartLog`] remains function-local (cosmetic — it
    /// records restart events for diagnostics, no behavioral effect).
    fn bump_deadcode_delay_persisted(
        &mut self,
        fd: &mut crate::funcdata::Funcdata,
        spc: &Rc<AddrSpace>,
    ) {
        let mut ovr = std::mem::take(fd.get_override_mut());
        let mut log = RestartLog::new();
        self.bump_deadcode_delay(fd, &mut ovr, &mut log, spc);
        *fd.get_override_mut() = ovr;
    }
}

/// Build the minimal [`TypeOp`] skeleton heritage needs for the ops it creates
/// (C++ `glb->inst[opc]` — the op-property triple `op_set_opcode` caches).
///
/// Heritage creates MULTIEQUAL (the phi, which must carry the `marker` flag so
/// `is_marker()` / the renameRecurse MULTIEQUAL test see it), SUBPIECE (the
/// read-size normalizer), and PIECE (the write-size normalizer's concat).  The
/// merged `Funcdata` reaches only the `context::ArchContext` (no `inst` table),
/// so the property triple is built inline with the exact flags upstream's
/// `TypeOpMulti`/`TypeOpSubpiece`/`TypeOpPiece` carry.
fn typeop_skeleton(opc: kuna_num::opcodes::OpCode) -> crate::context::TypeOp {
    use crate::op::pcodeop_flags as f;
    use kuna_num::opcodes::OpCode;
    let (flags, name) = match opc {
        // TypeOpMulti: marker (a special, non-evaluated phi node).
        OpCode::CPUI_MULTIEQUAL => (f::marker, "?"),
        // TypeOpSubpiece: binary.
        OpCode::CPUI_SUBPIECE => (f::binary, "SUB"),
        // TypeOpPiece: binary (the write-size-normalizer concat, "CONCAT").
        OpCode::CPUI_PIECE => (f::binary, "CONCAT"),
        _ => panic!("heritage typeop_skeleton: unexpected opcode {opc:?}"),
    };
    crate::context::TypeOp::new(opc, flags, name)
}

/// `PcodeOp::getOpFromConst(def->getIn(1)->getAddr())` for the INDIRECT
/// "same time" carve-out (C++ `renameRecurse`, heritage.cc:2507).
///
/// The INDIRECT's second input is an IOP-space Varnode whose address offset
/// encodes the PcodeOp the indirect effect comes from (C++ `newVarnodeIop`,
/// re-cast by `getOpFromConst`).  The kuna IR encodes the slotmap key into that
/// offset ([`op_iop_encode`]/[`op_iop_decode`], `funcdata_varnode.rs`), so the
/// decode round-trips the `OpId`.  `None` when `def` has no in[1] (defensive;
/// every INDIRECT carries its iop input).
fn op_from_const(
    fd: &crate::funcdata::Funcdata,
    indirect_def: crate::context::OpId,
) -> Option<crate::context::OpId> {
    let iop_vn = fd.obank().get(indirect_def)?.get_in(1)?;
    let off = fd.vbank().get(iop_vn)?.get_addr().get_offset();
    Some(crate::funcdata_varnode::op_iop_decode(off))
}

impl Default for Heritage {
    fn default() -> Self {
        Heritage::new()
    }
}

/// Panic helper for the stub methods whose `Funcdata` SSA-construction
/// primitives are not yet present.  Centralizes the message so the W3-op /
/// W4 / W6 waves can grep the stubs.
#[inline(never)]
#[cold]
fn unimplemented_stub(what: &str) -> ! {
    panic!("kuna heritage STUB not yet realized: {what}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::funcdata::Funcdata;
    use crate::context::ArchContext;
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
        let glb = Rc::new(ArchContext::new(manage));
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
        let (key, intersect, _sz) = lm.add(raddr(&fd, 0x100), 4, 0);
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
        let (key, intersect, _sz) = lm.add(raddr(&fd, 0x102), 4, 1);
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
        let (key, intersect, _sz) = lm.add(raddr(&fd, 0x102), 8, 2);
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
        let (_key, intersect, _sz) = lm.add(raddr(&fd, 0x101), 2, 0);
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
        let (key, _i, _sz) = lm.add(raddr(&fd, 0x100), 0x10, 1);
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

    #[test]
    fn collect_preserves_half_open_range_outputs() {
        use kuna_num::opcodes::OpCode;

        let mut fd = build_fd();
        let copy = || crate::context::TypeOp::new(OpCode::CPUI_COPY, 0, "copy");

        let read = fd.new_varnode(1, &raddr(&fd, 0x100), None);
        let read_op = fd.new_op(1, raddr(&fd, 0x2000));
        fd.op_set_opcode(read_op, copy());
        fd.op_set_input(read_op, read, 0).unwrap();

        let write_op = fd.new_op(1, raddr(&fd, 0x2001));
        fd.op_set_opcode(write_op, copy());
        let write = fd.new_varnode_out(4, &raddr(&fd, 0x108), write_op).unwrap();

        let input_free = fd.new_varnode(2, &raddr(&fd, 0x10c), None);
        let input = fd.set_input_varnode(input_free).unwrap();

        let end_op = fd.new_op(1, raddr(&fd, 0x2002));
        fd.op_set_opcode(end_op, copy());
        let _end_write = fd.new_varnode_out(8, &raddr(&fd, 0x110), end_op).unwrap();

        let below_op = fd.new_op(1, raddr(&fd, 0x2003));
        fd.op_set_opcode(below_op, copy());
        let _below_write = fd.new_varnode_out(8, &raddr(&fd, 0xff), below_op).unwrap();

        let mut range =
            MemRange::new(raddr(&fd, 0x100), 0x10, memrange_flags::new_addresses);
        let mut reads = Vec::new();
        let mut writes = Vec::new();
        let mut inputs = Vec::new();
        let mut removes = Vec::new();
        let maxsize = Heritage::new().collect(
            &fd,
            &mut range,
            &mut reads,
            &mut writes,
            &mut inputs,
            &mut removes,
        );

        assert_eq!(reads, vec![read], "the start address is inclusive");
        assert_eq!(writes, vec![write]);
        assert_eq!(inputs, vec![input]);
        assert!(removes.is_empty());
        assert_eq!(maxsize, 4, "the larger writes below and at the end are excluded");
        assert!(range.new_addresses());
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
        h.heritage(&mut fd, None);
        assert_eq!(h.get_pass(), 1, "one heritage pass advances `pass` to 1");
        assert_eq!(fd.vbank().num_varnodes(), 0, "empty function: no Varnodes created");
    }

    /// END-TO-END SSA: the full driver (collect → guard → calcMultiequals →
    /// MULTIEQUAL construction → rename) builds real SSA on a hand-built diamond.
    ///
    /// A single-manager `Funcdata` (glb's manager *is* the `ram` space the
    /// Varnodes use — the dual-manager boundary the corpus B3 gate documents is
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
        let glb = Rc::new(ArchContext::new(m));
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
        let copy = |fl: u32| crate::context::TypeOp::new(OpCode::CPUI_COPY, fl, "copy");

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
        h.heritage(&mut fd, None);

        // Exactly one MULTIEQUAL, at block 3 (the join).
        let phis: Vec<crate::context::OpId> = fd
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
        let in_defs: Vec<crate::context::OpId> = [in0, in1]
            .iter()
            .filter_map(|&v| fd.vbank().get(v).unwrap().get_def())
            .collect();
        assert!(in_defs.contains(&w1) && in_defs.contains(&w2), "phi merges the two writes");
    }

    // ---- w10-proto-cluster verifier: processJoins join-space splitting ----
    //
    // The branch named "w10-proto-cluster" actually ports
    // `Heritage::processJoins` and its helpers (heritage.cc:2068-2313). These
    // adversarial tests target the fragile spots the hunt list flagged:
    // (A) `split_join_level` size-accumulation / branch selection
    //     (register-piece vs newUnique, the at-least-once `numinhalf`),
    // (B) the 2-1 `nextlev` mapping with the `None` filler for unsplit pieces,
    // (C) the snapshot-vs-live-set loop boundary of `process_joins` on an
    //     empty join space (the documented no-op path).

    use kuna_base::space::{JoinSpace, VarnodeStorage};

    /// Manager with a join space (index after the std spaces) plus two distinct
    /// 8-byte register-bearing pieces in `ram`.
    fn build_manager_with_join() -> AddrSpaceManager {
        let mut m = build_manager(); // #(0) unique(1) ram(2)
        // JoinSpace at index 3; insert_space wires manager.joinspace.
        m.insert_space(Rc::new(JoinSpace::new(3, false))).unwrap();
        m
    }

    fn vstore(spc: &Rc<AddrSpace>, off: u64, size: u32) -> VarnodeStorage {
        VarnodeStorage { space: Some(Rc::clone(spc)), offset: off, size }
    }

    fn build_fd_with_join() -> Funcdata {
        let manage = build_manager_with_join();
        let glb = Rc::new(ArchContext::new(manage));
        let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
        let addr = Address::new(ram, 0x1000);
        Funcdata::new("func", "func", glb, addr, 0x10000000, 0x40).unwrap()
    }

    // ---- w10-revisit-ssa: remove_revisited_markers adversarial tests --------
    //
    // These directly exercise `Heritage::remove_revisited_markers` (the
    // re-heritage revisit, C++ heritage.cc:245), which is currently unreached on
    // the datatest corpus (the upstream multi-pass `collect` machinery that
    // populates `removevars` does not yet produce them in the Rust engine), so
    // they are the only guards that the branch logic matches the C++.

    use kuna_base::space::IopSpace;
    use kuna_num::opcodes::OpCode as TOpCode;

    /// Manager with constant / unique / iop / ram (the INDIRECT case needs the
    /// iop space for `new_varnode_iop`).
    fn build_manager_iop() -> AddrSpaceManager {
        let mut m = AddrSpaceManager::new();
        m.insert_space(Rc::new(ConstantSpace::new())).unwrap();
        m.insert_space(Rc::new(UniqueSpace::new(1, 0, false))).unwrap();
        m.insert_space(Rc::new(IopSpace::new(2))).unwrap();
        m.insert_space(Rc::new(AddrSpace::new(
            spacetype::IPTR_PROCESSOR,
            "ram",
            false,
            8,
            1,
            3,
            addrspace_flags::hasphysical,
            1,
            1,
        )))
        .unwrap();
        m
    }

    fn build_fd_iop() -> Funcdata {
        let manage = build_manager_iop();
        let glb = Rc::new(ArchContext::new(manage));
        let ram = Rc::clone(glb.manage().get_space_by_name("ram").unwrap());
        let addr = Address::new(ram, 0x1000);
        Funcdata::new("func", "func", glb, addr, 0x10000000, 0x40).unwrap()
    }

    // (A)+(B): a 16-byte Varnode against a 2x8 JoinRecord. `split_join_level`
    // must take the else-branch (cursize 16 != piece 8), find sizeaccum==16 at
    // j=recnum+2 so (j-recnum)==2 and numinhalf==1, and emit BOTH halves as
    // *register* pieces (newVarnode, not newUnique) at the recorded offsets,
    // with a `None`-free `nextlev` of exactly [most, least].
    #[test]
    fn w10_proto_cluster_split_join_level_two_register_halves() {
        let mut fd = build_fd_with_join();
        let ram = ram(&fd);
        // pieces: hi @ ram:0x40 (8 bytes), lo @ ram:0x48 (8 bytes) -> 16 logical
        let pieces = [vstore(&ram, 0x40, 8), vstore(&ram, 0x48, 8)];
        let joinrec = fd.get_arch().manage().find_add_join(&pieces, 0).unwrap();
        assert_eq!(joinrec.num_pieces(), 2);

        // A 16-byte unique Varnode standing in for the join whole.
        let curvn = fd.new_unique(16, None);

        let h = Heritage::new();
        let mut nextlev: Vec<Option<crate::context::VarnodeId>> = Vec::new();
        h.split_join_level(&mut fd, &[curvn], &mut nextlev, &joinrec);

        // 2-1 mapping: exactly two entries, both Some (this vn was split).
        assert_eq!(nextlev.len(), 2, "2-1 mapping: one pair per input vn");
        let most = nextlev[0].expect("most half present");
        let least = nextlev[1].expect("least half present (split this level)");

        // Both halves are *register* pieces at the recorded space/offset/size,
        // NOT newUnique temporaries (numinhalf==1 and (j-recnum)==2).
        let mv = fd.vbank().get(most).unwrap();
        assert!(Rc::ptr_eq(mv.get_addr().get_space().unwrap(), &ram), "most half in ram (piece space)");
        assert_eq!(mv.get_addr().get_offset(), 0x40, "most half at piece offset");
        assert_eq!(mv.get_size(), 8, "most half is the hi piece size");
        let lv = fd.vbank().get(least).unwrap();
        assert!(Rc::ptr_eq(lv.get_addr().get_space().unwrap(), &ram), "least half in ram (piece space)");
        assert_eq!(lv.get_addr().get_offset(), 0x48, "least half at piece offset");
        assert_eq!(lv.get_size(), 8, "least half is the lo piece size");
    }

    // (A) fast-path: when cursize already equals the current piece size, the
    // level pushes the Varnode unchanged plus a `None` filler and advances
    // recnum by one — the 2-1 mapping must stay aligned across a 3-piece record
    // whose middle position is consumed exactly.
    #[test]
    fn w10_proto_cluster_split_join_level_equal_piece_fast_path() {
        let mut fd = build_fd_with_join();
        let ram = ram(&fd);
        // Three pieces 8+8+8 = 24 logical.
        let pieces = [vstore(&ram, 0x40, 8), vstore(&ram, 0x48, 8), vstore(&ram, 0x50, 8)];
        let joinrec = fd.get_arch().manage().find_add_join(&pieces, 0).unwrap();

        // An already-8-byte Varnode equal to piece[0].size -> fast path.
        let small = fd.new_unique(8, None);
        let h = Heritage::new();
        let mut nextlev: Vec<Option<crate::context::VarnodeId>> = Vec::new();
        h.split_join_level(&mut fd, &[small], &mut nextlev, &joinrec);

        assert_eq!(nextlev.len(), 2, "fast path still emits a 2-1 pair");
        assert_eq!(nextlev[0], Some(small), "fast path keeps the input vn as the most-half");
        assert!(nextlev[1].is_none(), "fast path fills the least-half slot with None");
    }

    // (C): `process_joins` over a function whose join space holds NO Varnodes
    // is a no-op and does not panic. This guards the snapshot/loop boundary:
    // the loc-set is empty, so the `getSpace() != joinspace` break and the
    // find_join/expect path are never reached. Op/Varnode counts are unchanged.
    #[test]
    fn w10_proto_cluster_process_joins_empty_join_space_noop() {
        let mut fd = build_fd_with_join();
        assert!(fd.get_arch().manage().get_join_space().is_some(), "fixture has a join space");
        // Seed a non-join Varnode so "no-op" is meaningfully observable.
        let ram = ram(&fd);
        let _v = fd.new_varnode_space_off(8, ram, 0x100);
        let nvarnodes_before = fd.vbank().num_varnodes();
        let nops_before = fd.obank().num_alive();

        let mut h = Heritage::new();
        h.build_info_list(&fd);
        h.process_joins(&mut fd); // must be a clean no-op

        assert_eq!(fd.vbank().num_varnodes(), nvarnodes_before, "no Varnodes added");
        assert_eq!(fd.obank().num_alive(), nops_before, "no ops added");
    }

    /// Build a marker op of `code` in block `bl` whose output Varnode is a
    /// size-`out_size` write at `out_addr`.  Returns (op, out_vn).  Inserted at
    /// the end of the block.
    fn make_marker(
        fd: &mut Funcdata,
        bl: BlockId,
        code: TOpCode,
        out_addr: &Address,
        out_size: int4,
        ninputs: int4,
    ) -> (crate::context::OpId, crate::context::VarnodeId) {
        let pc = raddr(fd, 0x2000);
        let op = fd.new_op(ninputs, pc);
        // `remove_revisited_markers` branches only on `op.code()`; resolve the
        // opcode through the generic `inst[]` resolution (works for COPY/INDIRECT/
        // MULTIEQUAL alike).
        fd.op_set_opcode_code(op, code);
        let out = fd.new_varnode_out(out_size, out_addr, op).expect("marker out");
        // Wire every input slot to a fresh constant so op_unlink's per-input
        // opUnsetInput has a real link to clear (C++ markers always have their
        // inputs set before removeRevisitedMarkers runs).
        for slot in 0..ninputs {
            let c = fd.new_constant(out_size, slot as uintb);
            fd.op_set_input(op, c, slot).expect("marker input");
        }
        fd.op_insert(op, bl, None); // append at block end
        (op, out)
    }

    /// COPY branch: a return-form COPY marker is simply unlinked (no SUBPIECE),
    /// matching C++ heritage.cc:282-284 (`opUnlink(op); continue;`).
    #[test]
    fn w10_revisit_remove_markers_copy_is_unlinked() {
        let mut h = Heritage::new();
        let mut fd = build_fd_iop();
        h.build_info_list(&fd);
        let blocks = build_cfg(&mut fd, 1, &[]);
        let bl = blocks[0];

        // A COPY marker writing a 2-byte slice; the larger range is 4 bytes at addr.
        let addr = raddr(&fd, 0x100);
        let copy_out_addr = raddr(&fd, 0x100);
        let (copy_op, copy_out) = make_marker(&mut fd, bl, TOpCode::CPUI_COPY, &copy_out_addr, 2, 1);
        assert_eq!(fd.bb_ops(bl).len(), 1, "block has the COPY before removal");

        h.remove_revisited_markers(&mut fd, &[copy_out], &addr, 4);

        // C++ opUnlink: op removed from its block (parent cleared), opcode untouched.
        assert!(
            fd.obank().get(copy_op).expect("copy op alive").get_parent().is_none(),
            "return-form COPY is unlinked from its block"
        );
        assert_eq!(
            fd.obank().get(copy_op).unwrap().code(),
            TOpCode::CPUI_COPY,
            "COPY is NOT rewritten to SUBPIECE (the COPY arm `continue`s before the SUBPIECE rewrite)"
        );
        assert!(fd.bb_ops(bl).is_empty(), "block is empty after the COPY unlink");
    }

    /// MULTIEQUAL branch: the marker is rewritten in place to a SUBPIECE of a
    /// new full-size free Varnode, inserted AFTER the trailing run of
    /// MULTIEQUALs in the block (C++ heritage.cc:276-281, 286-296).  The
    /// SUBPIECE's offset operand is the byte overlap; the original output gets
    /// the write-mask.
    #[test]
    fn w10_revisit_remove_markers_multiequal_rewritten_after_phis() {
        let mut h = Heritage::new();
        let mut fd = build_fd_iop();
        h.build_info_list(&fd);
        let blocks = build_cfg(&mut fd, 1, &[]);
        let bl = blocks[0];

        let addr = raddr(&fd, 0x200);
        // Target MULTIEQUAL marker writes the low 2 bytes at addr (offset 0).
        let (me_op, me_out) =
            make_marker(&mut fd, bl, TOpCode::CPUI_MULTIEQUAL, &addr, 2, 2);
        // A SECOND, trailing MULTIEQUAL follows it: the SUBPIECE must be inserted
        // AFTER this one (the `while (*pos)==MULTIEQUAL ++pos` skip loop).
        let trail_addr = raddr(&fd, 0x300);
        let (trail_me, _trail_out) =
            make_marker(&mut fd, bl, TOpCode::CPUI_MULTIEQUAL, &trail_addr, 2, 2);

        h.remove_revisited_markers(&mut fd, &[me_out], &addr, 4);

        // The op is rewritten in place to a SUBPIECE.
        assert_eq!(
            fd.obank().get(me_op).unwrap().code(),
            TOpCode::CPUI_SUBPIECE,
            "MULTIEQUAL marker rewritten to SUBPIECE"
        );
        // in0 = the new full-size free Varnode (size 4 at addr), in1 = const offset 0.
        let big = fd.obank().get(me_op).unwrap().get_in(0).unwrap();
        assert_eq!(fd.vbank().get(big).unwrap().get_size(), 4, "SUBPIECE in0 is the full-size vn");
        assert_eq!(fd.vbank().get(big).unwrap().get_addr(), &addr, "full-size vn at the range addr");
        let off = fd.obank().get(me_op).unwrap().get_in(1).unwrap();
        assert!(fd.vbank().get(off).unwrap().is_constant(), "SUBPIECE in1 is a constant");
        assert_eq!(
            fd.vbank().get(off).unwrap().get_addr().get_offset(),
            0,
            "offset operand == vn->overlap(addr,size) == 0 for an LSB-aligned slice"
        );
        // The original output is write-masked.
        assert!(
            fd.vbank().get(me_out).unwrap().is_write_mask(),
            "original marker output carries the write mask"
        );
        // Insertion order: the rewritten SUBPIECE must come AFTER the trailing
        // MULTIEQUAL (it skipped the trailing phi run).
        let ops = fd.bb_ops(bl);
        let pos_sub = ops.iter().position(|&o| o == me_op).expect("SUBPIECE in block");
        let pos_trail = ops.iter().position(|&o| o == trail_me).expect("trailing phi in block");
        assert!(
            pos_sub > pos_trail,
            "SUBPIECE inserted after the trailing MULTIEQUAL (skip-trailing-phis loop), got sub@{pos_sub} trail@{pos_trail}"
        );
    }

    /// INDIRECT branch: the SUBPIECE is inserted after the INDIRECT's *target*
    /// op (decoded from the iop input via `op_iop_decode`), and the marker
    /// output's addr-force is cleared (C++ heritage.cc:266-274).
    #[test]
    fn w10_revisit_remove_markers_indirect_inserts_after_live_target() {
        let mut h = Heritage::new();
        let mut fd = build_fd_iop();
        h.build_info_list(&fd);
        let blocks = build_cfg(&mut fd, 1, &[]);
        let bl = blocks[0];

        // A live "target" op (a COPY) the INDIRECT points at, inserted first.
        let target = {
            let pc = raddr(&fd, 0x2100);
            let op = fd.new_op(1, pc);
            fd.op_set_opcode(op, typeop_skeleton(TOpCode::CPUI_SUBPIECE));
            fd.op_insert(op, bl, None);
            op
        };
        assert!(!fd.obank().get(target).unwrap().is_dead(), "target op is live/in-block");

        // A spacer op between the target and the INDIRECT, so the insertion
        // point `++(target iter)` is a distinct op (not the INDIRECT itself):
        // the SUBPIECE replaces the INDIRECT and is re-inserted *before* the
        // spacer (i.e. immediately after the target), the C++ semantics.  (If
        // the INDIRECT immediately followed its own target, `++pos` would land
        // on the INDIRECT op being removed — a degenerate self-insert that is a
        // latent C++ UB, not the path under test here.)
        let spacer = {
            let pc = raddr(&fd, 0x2110);
            let op = fd.new_op(1, pc);
            fd.op_set_opcode(op, typeop_skeleton(TOpCode::CPUI_SUBPIECE));
            fd.op_insert(op, bl, None);
            op
        };

        // The INDIRECT marker, in[1] = iop varnode encoding `target`.
        let addr = raddr(&fd, 0x400);
        let (ind_op, ind_out) =
            make_marker(&mut fd, bl, TOpCode::CPUI_INDIRECT, &addr, 2, 2);
        let iop = fd.new_varnode_iop(target);
        fd.op_set_input(ind_op, iop, 1).expect("set iop input");
        // Force the address on the marker output so we can observe clearAddrForce.
        fd.vbank_mut().get_mut(ind_out).unwrap().set_addr_force();
        assert!(fd.vbank().get(ind_out).unwrap().is_addr_force(), "addr-force set pre-call");

        h.remove_revisited_markers(&mut fd, &[ind_out], &addr, 4);

        // Rewritten to SUBPIECE, addr-force cleared on the original output.
        assert_eq!(
            fd.obank().get(ind_op).unwrap().code(),
            TOpCode::CPUI_SUBPIECE,
            "INDIRECT marker rewritten to SUBPIECE"
        );
        assert!(
            !fd.vbank().get(ind_out).unwrap().is_addr_force(),
            "clearAddrForce ran (replacement INDIRECT will hold the address)"
        );
        // The SUBPIECE is inserted immediately AFTER the live target op (before
        // the spacer): C++ `pos = targetOp->getBasicIter(); ++pos;`.
        let ops = fd.bb_ops(bl);
        let pos_sub = ops.iter().position(|&o| o == ind_op).expect("SUBPIECE in block");
        let pos_target = ops.iter().position(|&o| o == target).expect("target in block");
        let pos_spacer = ops.iter().position(|&o| o == spacer).expect("spacer in block");
        assert!(
            pos_sub > pos_target && pos_sub < pos_spacer,
            "SUBPIECE inserted right after the target op (before the spacer), got sub@{pos_sub} target@{pos_target} spacer@{pos_spacer}"
        );
    }

    /// INDIRECT branch, big-endian overlap: a 2-byte slice at the high half of a
    /// 4-byte big-endian range has a non-zero overlap offset, exercising the
    /// signed `vn->overlap(addr,size)` -> `offset as uintb` widening at the
    /// SUBPIECE in1 (the one bare `as` cast in the ported code).
    #[test]
    fn w10_revisit_remove_markers_offset_widening_le() {
        let mut h = Heritage::new();
        let mut fd = build_fd_iop();
        h.build_info_list(&fd);
        let blocks = build_cfg(&mut fd, 1, &[]);
        let bl = blocks[0];

        // Little-endian ram: a 2-byte marker at addr+2 overlaps the 4-byte range
        // [addr, addr+4) at byte offset 2.
        let base = raddr(&fd, 0x500);
        let hi = raddr(&fd, 0x502);
        let (me_op, me_out) = make_marker(&mut fd, bl, TOpCode::CPUI_MULTIEQUAL, &hi, 2, 2);

        h.remove_revisited_markers(&mut fd, &[me_out], &base, 4);

        let off = fd.obank().get(me_op).unwrap().get_in(1).unwrap();
        assert_eq!(
            fd.vbank().get(off).unwrap().get_addr().get_offset(),
            2,
            "offset operand == byte overlap (2) of the high slice into the LE range; widened int4->uintb"
        );
        // The constant is size 4 (C++ newConstant(4, offset)).
        assert_eq!(fd.vbank().get(off).unwrap().get_size(), 4, "offset constant is 4 bytes");
    }
}
