//! Port of `decompiler/cpp/varnode.{hh,cc}` (W3, item `w3-ir-varnode`) — the
//! `Varnode` and `VarnodeBank` classes: the fundamental \e variable of the
//! p-code IR and its per-function container.
//!
//! ## ADR 0001 (IR arenas) realization
//!
//! The C++ `Varnode` is a heap object cross-referenced by raw pointers:
//! `def` is a `PcodeOp *`, `descend` is a `list<PcodeOp *>` of consumers, and
//! `VarnodeBank` keeps two `std::set`s of `Varnode *` with each node caching
//! its own `set::iterator` into both trees.  Per ADR 0001 the Rust port stores
//! `Varnode`s in a `Funcdata`-owned slotmap arena keyed by [`VarnodeId`]; here
//! [`VarnodeBank`] owns that arena directly (the bank *is* the varnode store).
//! `def` becomes `Option<OpId>`, `descend` a [`SmallVec`] of [`OpId`] with the
//! exact C++ `push_back`/erase-by-value order, and the cached `set::iterator`s
//! become **stored keys** ([`LocKey`]/[`DefKey`]) so the xref operations
//! (`xref`/`makeFree`/`setInput`/`setDef`/`destroy`) do the same precise
//! `erase(iter)` + reinsert the C++ does.
//!
//! ## ADR 0002 (ordered containers) realization
//!
//! `loc_tree`/`def_tree` are `BTreeMap<LocKey,VarnodeId>` /
//! `BTreeMap<DefKey,VarnodeId>`.  [`LocKey::cmp`]/[`DefKey::cmp`] transcribe
//! `VarnodeCompareLocDef`/`VarnodeCompareDefLoc` (`varnode.cc:34-79`)
//! field-by-field, **including** the `(f1-1) < (f2-1)` wrap that orders
//! input < written < free and the `create_index` tie-break for frees.  The
//! key carries everything the comparator reads — address, size, flag class,
//! the def's `SeqNum` (written) or `create_index` (free) — so the trees never
//! dereference an op to compare, and `beginLoc`/`endLoc`/`beginDef`/`endDef`
//! become `BTreeMap::range` queries over *constructed bound keys* that mirror
//! the C++ `searchvn` template assignments exactly.

use std::collections::BTreeMap;
use std::ops::Bound;
use std::rc::Rc;

use kuna_base::address::{mach_extreme, Address, SeqNum};
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::space::{spacetype, AddrSpace, AddrSpaceManager};
use kuna_base::types::{int2, int4, uint2, uint4, uintb, uintm, Wrap};
use slotmap::SlotMap;
use smallvec::SmallVec;

use crate::cover::{Cover, CoverContext};
use crate::dtype::{type_metatype, Datatype};
use crate::seams::{HighVariableId, OpId, VarnodeId};

/// Boolean attributes of a [`Varnode`] (C++ `Varnode::varnode_flags`).
///
/// Verbatim transcription of the C++ `enum varnode_flags` (`varnode.hh:78-116`).
pub mod varnode_flags {
    #![allow(non_upper_case_globals)]
    use kuna_base::types::uint4;

    /// Prevents infinite loops
    pub const mark: uint4 = 0x01;
    /// The varnode is constant
    pub const constant: uint4 = 0x02;
    /// This varnode is an annotation and has no dataflow
    pub const annotation: uint4 = 0x04;
    /// This varnode has no ancestor
    pub const input: uint4 = 0x08;
    /// This varnode has a defining op (def is nonzero)
    pub const written: uint4 = 0x10;
    /// This varnode has been inserted in a tree (output of an op, or a
    /// constant, or an input)
    pub const insert: uint4 = 0x20;
    /// This varnode is a temporary variable
    pub const implied: uint4 = 0x40;
    /// This varnode \e CANNOT be a temporary variable
    pub const explict: uint4 = 0x80;

    /// The Dataype of the Varnode is locked
    pub const typelock: uint4 = 0x100;
    /// The Name of the Varnode is locked
    pub const namelock: uint4 = 0x200;
    /// There are no aliases pointing to this varnode
    pub const nolocalalias: uint4 = 0x400;
    /// This varnode's value is volatile
    pub const volatil: uint4 = 0x800;

    /// Varnode address is specially mapped by the loader
    pub const externref: uint4 = 0x1000;
    /// Varnode is stored at a readonly location
    pub const readonly: uint4 = 0x2000;
    /// Persists after (and before) function
    pub const persist: uint4 = 0x4000;
    /// High-level variable is tied to address
    pub const addrtied: uint4 = 0x8000;
    /// Input which is unaffected by the function
    pub const unaffected: uint4 = 0x10000;
    /// This is a base register for an address space
    pub const spacebase: uint4 = 0x20000;
    /// If all uses of illegalinput varnode are inputs to INDIRECT
    pub const indirectonly: uint4 = 0x40000;
    /// (could be) Directly affected by a valid input
    pub const directwrite: uint4 = 0x80000;
    /// Varnode is used to force variable into an address
    pub const addrforce: uint4 = 0x100000;

    /// Varnode has a database entry associated with it
    pub const mapped: uint4 = 0x200000;
    /// The value in this Varnode is created indirectly
    pub const indirect_creation: uint4 = 0x400000;
    /// Is the varnode storage for a return address
    pub const return_address: uint4 = 0x800000;
    /// Cover is not upto date
    pub const coverdirty: uint4 = 0x1000000;
    /// Is this Varnode the low part of a double precision value
    pub const precislo: uint4 = 0x2000000;
    /// Is this Varnode the high part of a double precision value
    pub const precishi: uint4 = 0x4000000;
    /// Is this Varnode storing a pointer to the actual symbol
    pub const indirectstorage: uint4 = 0x8000000;
    /// Does this varnode point to the return value storage location
    pub const hiddenretparm: uint4 = 0x10000000;
    /// Do copies of this varnode happen as a side-effect
    pub const incidental_copy: uint4 = 0x20000000;
    /// Temporarily block dead-code removal of \b this
    pub const autolive_hold: uint4 = 0x40000000;
    /// Varnode is getting PIECEd together into an (unmapped) structure
    pub const proto_partial: uint4 = 0x80000000;
}

/// Additional boolean properties of a [`Varnode`] (C++ `Varnode::addl_flags`).
///
/// Verbatim transcription of the C++ `enum addl_flags` (`varnode.hh:118-133`).
pub mod addl_flags {
    #![allow(non_upper_case_globals)]
    use kuna_base::types::uint2;

    /// The varnode is actively being heritaged
    pub const activeheritage: uint2 = 0x01;
    /// Should not be considered a write in heritage calculation
    pub const writemask: uint2 = 0x02;
    /// Vacuous consume
    pub const vacconsume: uint2 = 0x04;
    /// In consume worklist
    pub const lisconsume: uint2 = 0x08;
    /// The Varnode value is \e NOT a pointer
    pub const ptrcheck: uint2 = 0x10;
    /// If this varnode flows to or from a pointer
    pub const ptrflow: uint2 = 0x20;
    /// Constant that must be explicitly printed as an unsigned token
    pub const unsignedprint: uint2 = 0x40;
    /// Constant that must be explicitly printed as a \e long integer token
    pub const longprint: uint2 = 0x80;
    /// Created by an explicit STORE
    pub const stack_store: uint2 = 0x100;
    /// Input that exists even if its unused
    pub const locked_input: uint2 = 0x200;
    /// Inserted artificially to track a register value at a specific point
    pub const spacebase_placeholder: uint2 = 0x400;
    /// Data-types do not propagate from an output into \b this
    pub const stop_uppropagation: uint2 = 0x800;
    /// The varnode is implied but also has a data-type that needs resolution
    pub const has_implied_field: uint2 = 0x1000;
}

/// Number of inline `OpId` slots a `descend` list keeps before spilling to the
/// heap.  Most varnodes have very few reads; this just trims allocations and
/// has no semantic effect (order and contents match the C++ `list` exactly).
type DescendVec = SmallVec<[OpId; 4]>;

// ---------------------------------------------------------------------------
// Comparator keys (ADR 0002): transcribe VarnodeCompareLocDef / DefLoc exactly
// ---------------------------------------------------------------------------

/// The `(input|written)` flag class a comparator extracts, as the comparator
/// orders it: `input < written < free` via the C++ `(f1-1) < (f2-1)` wrap.
///
/// `flag_class_of` masks `flags & (input|written)` and this `Ord` reproduces
/// `((f1-1) < (f2-1))` with explicit `uint4` wrapping subtraction (free is
/// `0`, so `0u32.wrapping_sub(1) == u32::MAX` — frees sort last).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct FlagClass(uint4);

impl FlagClass {
    /// The raw `flags & (input|written)` value (one of 0, input, written).
    fn raw(self) -> uint4 {
        self.0
    }
}

impl Ord for FlagClass {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // C++: return ((f1-1) < (f2-1));  // -1 forces free varnodes to come last
        self.0.wsub(1).cmp(&other.0.wsub(1))
    }
}
impl PartialOrd for FlagClass {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// Extract the `(input|written)` class from a flag word (C++
/// `flags & (Varnode::input|Varnode::written)`).
fn flag_class_of(flags: uint4) -> FlagClass {
    FlagClass(flags & (varnode_flags::input | varnode_flags::written))
}

/// The seqnum step of the varnode comparators, faithful to C++:
///
/// ```text
/// if (a->getDef()->getSeqNum() != b->getDef()->getSeqNum())
///   return (a->getDef()->getSeqNum() < b->getDef()->getSeqNum());
/// // else fall through (contributes Equal)
/// ```
///
/// This is **not** `SeqNum::cmp`: `SeqNum::operator!=` compares only the
/// `uniq` field (`address.rs`), while `operator<` orders by `(pc, uniq)`.  So
/// when two def seqnums share a `uniq` but differ in `pc`, the C++ `!=` is
/// false and the comparator does *not* order on them (it falls through).
/// Within a real function `uniq` is unique per op so this degenerate case
/// never arises, but the transcription preserves it exactly.
fn seqnum_step(a: &SeqNum, b: &SeqNum) -> std::cmp::Ordering {
    // a != b  <=>  a.getTime() != b.getTime()  (uniq-only equality)
    if a.get_time() != b.get_time() {
        a.cmp(b) // operator< / full (pc, uniq) ordering
    } else {
        std::cmp::Ordering::Equal // fall through
    }
}

/// Sort key for the location tree (`VarnodeCompareLocDef`, `varnode.cc:34-53`).
///
/// Captures every field the comparator reads so the `BTreeMap` orders without
/// dereferencing an op.  For written varnodes `seqnum` holds the def's
/// `SeqNum`; for free varnodes `create_index` provides the final tie-break.
#[derive(Debug, Clone)]
pub struct LocKey {
    /// Storage location (C++ `getAddr()`)
    addr: Address,
    /// Size in bytes (C++ `getSize()`)
    size: int4,
    /// `(input|written)` flag class (C++ `getFlags()&(input|written)`)
    flagclass: FlagClass,
    /// The def `SeqNum`, consulted only when `flagclass == written`
    seqnum: SeqNum,
    /// Creation index, consulted only when `flagclass == 0` (free)
    create_index: uint4,
}

impl PartialEq for LocKey {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == std::cmp::Ordering::Equal
    }
}
impl Eq for LocKey {}

impl Ord for LocKey {
    /// Transcribes `VarnodeCompareLocDef::operator()` (`varnode.cc:34-53`).
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering;
        // if (a->getAddr() != b->getAddr()) return (a->getAddr() < b->getAddr());
        let ord = self.addr.cmp(&other.addr);
        if ord != Ordering::Equal {
            return ord;
        }
        // if (a->getSize() != b->getSize()) return (a->getSize() < b->getSize());
        let ord = self.size.cmp(&other.size);
        if ord != Ordering::Equal {
            return ord;
        }
        // f1 = ... ; f2 = ... ; if (f1!=f2) return ((f1-1)<(f2-1));
        let ord = self.flagclass.cmp(&other.flagclass);
        if ord != Ordering::Equal {
            return ord;
        }
        // f1 == f2 from here on
        if self.flagclass.raw() == varnode_flags::written {
            // if (a->getDef()->getSeqNum() != b->getDef()->getSeqNum())
            //   return (a->getDef()->getSeqNum() < b->getDef()->getSeqNum());
            let ord = seqnum_step(&self.seqnum, &other.seqnum);
            if ord != Ordering::Equal {
                return ord;
            }
        } else if self.flagclass.raw() == 0 {
            // both free: return (a->getCreateIndex() < b->getCreateIndex());
            return self.create_index.cmp(&other.create_index);
        }
        // return false;  (equivalent)
        Ordering::Equal
    }
}
impl PartialOrd for LocKey {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// Sort key for the definition tree (`VarnodeCompareDefLoc`, `varnode.cc:60-79`).
#[derive(Debug, Clone)]
pub struct DefKey {
    /// Storage location (C++ `getAddr()`)
    addr: Address,
    /// Size in bytes (C++ `getSize()`)
    size: int4,
    /// `(input|written)` flag class
    flagclass: FlagClass,
    /// The def `SeqNum`, consulted only when `flagclass == written`
    seqnum: SeqNum,
    /// Creation index, consulted only when `flagclass == 0` (free)
    create_index: uint4,
}

impl PartialEq for DefKey {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == std::cmp::Ordering::Equal
    }
}
impl Eq for DefKey {}

impl Ord for DefKey {
    /// Transcribes `VarnodeCompareDefLoc::operator()` (`varnode.cc:60-79`).
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering;
        // f1 = ...; f2 = ...; if (f1!=f2) return ((f1-1)<(f2-1));
        let ord = self.flagclass.cmp(&other.flagclass);
        if ord != Ordering::Equal {
            return ord;
        }
        // if (f1==Varnode::written) { compare seqnum (but DO NOT return on equal) }
        if self.flagclass.raw() == varnode_flags::written {
            let ord = seqnum_step(&self.seqnum, &other.seqnum);
            if ord != Ordering::Equal {
                return ord;
            }
        }
        // if (a->getAddr() != b->getAddr()) return (a->getAddr() < b->getAddr());
        let ord = self.addr.cmp(&other.addr);
        if ord != Ordering::Equal {
            return ord;
        }
        // if (a->getSize() != b->getSize()) return (a->getSize() < b->getSize());
        let ord = self.size.cmp(&other.size);
        if ord != Ordering::Equal {
            return ord;
        }
        // if (f1==0) return (a->getCreateIndex() < b->getCreateIndex());
        if self.flagclass.raw() == 0 {
            return self.create_index.cmp(&other.create_index);
        }
        // return false;
        Ordering::Equal
    }
}
impl PartialOrd for DefKey {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// ---------------------------------------------------------------------------
// Tree probes: faithful translation of C++ lower_bound/upper_bound iterators
// ---------------------------------------------------------------------------
//
// A C++ `set::iterator` returned by lower_bound/upper_bound (or begin()/end())
// is used by the bank either as a *start* of a half-open range or as its
// *end*.  Crucially the same iterator means different element-membership in
// each position, so a `Probe` records the C++ operation and is converted to a
// Rust `Bound` only once its position is known:
//
//   op              as a START bound        as an END bound
//   lower_bound(k)  Included(k)  (>= k)      Excluded(k)  (< k)
//   upper_bound(k)  Excluded(k)  (> k)       Included(k)  (<= k)
//   begin()         Unbounded                Excluded(min) — never used as end
//   end()           Excluded(max) — n/a      Unbounded
//
// (`begin()` is only ever a start and `end()` only ever an end here.)

/// A C++ lower/upper-bound (or begin/end) probe over the location tree.
enum LocProbe {
    /// C++ `lower_bound(k)`
    Lower(LocKey),
    /// C++ `upper_bound(k)`
    Upper(LocKey),
    /// C++ `loc_tree.end()` (used only as an end)
    End,
}

impl LocProbe {
    /// Convert to the Rust *start* bound of a half-open range.
    fn into_start(self) -> Bound<LocKey> {
        match self {
            LocProbe::Lower(k) => Bound::Included(k), // first element >= k
            LocProbe::Upper(k) => Bound::Excluded(k), // first element > k
            LocProbe::End => Bound::Unbounded,        // (never used as a start here)
        }
    }
    /// Convert to the Rust *end* bound of a half-open range.
    fn into_end(self) -> Bound<LocKey> {
        match self {
            LocProbe::Lower(k) => Bound::Excluded(k), // up to (not incl.) first >= k  => elems < k
            LocProbe::Upper(k) => Bound::Included(k), // up to (not incl.) first > k    => elems <= k
            LocProbe::End => Bound::Unbounded,
        }
    }
}

/// A C++ lower/upper-bound (or begin/end) probe over the definition tree.
#[derive(Clone)]
enum DefProbe {
    /// C++ `lower_bound(k)`
    Lower(DefKey),
    /// C++ `upper_bound(k)`
    Upper(DefKey),
    /// C++ `def_tree.begin()` (used only as a start)
    Begin,
    /// C++ `def_tree.end()` (used only as an end)
    End,
}

impl DefProbe {
    /// Convert to the Rust *start* bound of a half-open range.
    fn into_start(self) -> Bound<DefKey> {
        match self {
            DefProbe::Lower(k) => Bound::Included(k),
            DefProbe::Upper(k) => Bound::Excluded(k),
            DefProbe::Begin => Bound::Unbounded,
            DefProbe::End => Bound::Unbounded, // (never used as a start here)
        }
    }
    /// Convert to the Rust *end* bound of a half-open range.
    fn into_end(self) -> Bound<DefKey> {
        match self {
            DefProbe::Lower(k) => Bound::Excluded(k),
            DefProbe::Upper(k) => Bound::Included(k),
            DefProbe::Begin => Bound::Unbounded, // (never used as an end here)
            DefProbe::End => Bound::Unbounded,
        }
    }
}

// ---------------------------------------------------------------------------
// Varnode
// ---------------------------------------------------------------------------

/// A low-level variable or contiguous set of bytes described by an Address and
/// a size (C++ `class Varnode`).
///
/// In its raw form a Varnode is \e free and the (address,size) pair uniquely
/// identifies it; in SSA form each individual write creates a distinct, non-
/// free Varnode instance.
#[derive(Debug, Clone)]
pub struct Varnode {
    /// The collection of boolean attributes (C++ `mutable uint4 flags`)
    flags: uint4,
    /// Size of the Varnode in bytes
    size: int4,
    /// A unique one-up index assigned at creation
    create_index: uint4,
    /// Which group of forced merges this Varnode belongs to
    mergegroup: int2,
    /// Additional flags
    addlflags: uint2,
    /// Storage location (or constant value) of the Varnode
    loc: Address,

    // Heritage fields
    /// The defining operation (C++ `PcodeOp *def`; `None` == null)
    def: Option<OpId>,
    /// Cached `SeqNum` of the defining op (C++ reads it live via
    /// `def->getSeqNum()`; the SeqNum is immutable for a given def, so the
    /// comparator keys cache it here to stay self-contained without the
    /// op arena — SEAM(W3): set alongside `def`, kept in sync by the bank).
    def_seqnum: Option<SeqNum>,
    /// High-level variable of which this is an instantiation (SEAM(W7))
    high: Option<HighVariableId>,
    /// Datatype associated with this varnode (SEAM(W6): minimal skeleton)
    type_: Rc<Datatype>,
    /// Cached key into the loc tree (replaces the C++ `lociter`)
    lociter: Option<LocKey>,
    /// Cached key into the def tree (replaces the C++ `defiter`)
    defiter: Option<DefKey>,
    /// Every op reading this varnode as input (C++ `list<PcodeOp *> descend`)
    descend: DescendVec,
    /// Addresses covered by def->use of this Varnode (SEAM(W7))
    cover: Option<Cover>,
    /// What parts of this varnode are used (C++ `uintb consumed`)
    consumed: uintb,
    /// Which bits are known to be zero (C++ `uintb nzm`)
    nzm: uintb,
}

impl Varnode {
    /// Construct a \e free Varnode (C++ `Varnode(int4,const Address&,Datatype*)`).
    ///
    /// `def` is unset, flags/nzm are seeded from the space type exactly as the
    /// C++ constructor (`varnode.cc:597-626`).  Most callers go through
    /// [`VarnodeBank`]; this is the raw, unmanaged construction.
    pub fn new(s: int4, m: Address, dt: Rc<Datatype>) -> Varnode {
        let mut vn = Varnode {
            flags: 0,
            size: s,
            create_index: 0,
            mergegroup: 0,
            addlflags: 0,
            loc: m,
            def: None,
            def_seqnum: None,
            high: None,
            type_: dt,
            lociter: None,
            defiter: None,
            descend: SmallVec::new(),
            cover: None,
            consumed: u64::MAX, // ~((uintb)0)
            nzm: 0,
        };
        // if (m.getSpace() == (AddrSpace *)0) { flags = 0; return; }
        let space = match vn.loc.get_space() {
            Some(spc) => Rc::clone(spc),
            None => {
                vn.flags = 0;
                return vn;
            }
        };
        let tp = space.get_type();
        if tp == spacetype::IPTR_CONSTANT {
            vn.flags = varnode_flags::constant;
            vn.nzm = vn.loc.get_offset();
        } else if tp == spacetype::IPTR_FSPEC || tp == spacetype::IPTR_IOP {
            vn.flags = varnode_flags::annotation | varnode_flags::coverdirty;
            vn.nzm = u64::MAX;
        } else {
            vn.flags = varnode_flags::coverdirty;
            vn.nzm = u64::MAX;
        }
        vn
    }

    // --- Accessors (C++ inline getters) -----------------------------------

    /// Get the storage Address (C++ `getAddr`).
    pub fn get_addr(&self) -> &Address {
        &self.loc
    }
    /// Get the AddrSpace storing this Varnode (C++ `getSpace`).  Panics on the
    /// null/sentinel address (C++ would dereference an invalid pointer).
    pub fn get_space(&self) -> &Rc<AddrSpace> {
        self.loc.get_space().expect("Varnode::getSpace on invalid address (C++ UB)")
    }
    /// Get the offset within the space (C++ `getOffset`).
    pub fn get_offset(&self) -> uintb {
        self.loc.get_offset()
    }
    /// Get the number of bytes this Varnode stores (C++ `getSize`).
    pub fn get_size(&self) -> int4 {
        self.size
    }
    /// Get the \e forced \e merge group (C++ `getMergeGroup`).
    pub fn get_merge_group(&self) -> int2 {
        self.mergegroup
    }
    /// Get the defining op id, or `None` (C++ `getDef`).
    pub fn get_def(&self) -> Option<OpId> {
        self.def
    }
    /// Get all boolean attributes (C++ `getFlags`).
    pub fn get_flags(&self) -> uint4 {
        self.flags
    }
    /// Get the Datatype (C++ `getType`).
    pub fn get_type(&self) -> &Rc<Datatype> {
        &self.type_
    }
    /// Get the creation index (C++ `getCreateIndex`).
    pub fn get_create_index(&self) -> uint4 {
        self.create_index
    }
    /// Iterate the descendant (reading) op ids (C++ `beginDescend`/`endDescend`).
    pub fn descend_iter(&self) -> impl Iterator<Item = OpId> + '_ {
        self.descend.iter().copied()
    }
    /// Number of descendants.
    pub fn num_descend(&self) -> usize {
        self.descend.len()
    }
    /// Get the mask of consumed bits (C++ `getConsume`).
    pub fn get_consume(&self) -> uintb {
        self.consumed
    }
    /// Set the mask of consumed bits (C++ `setConsume`).
    pub fn set_consume(&mut self, val: uintb) {
        self.consumed = val;
    }
    /// Get the mask of bits known to be zero (C++ `getNZMask`).
    pub fn get_nz_mask(&self) -> uintb {
        self.nzm
    }
    /// The high-level variable id, if any (C++ `getHigh`, sans the throw).
    pub fn get_high(&self) -> Option<HighVariableId> {
        self.high
    }
    /// Borrow the Cover object, if one has been built (C++ `getCover`).  The
    /// C++ `getCover` returns the raw `cover` pointer; the lazy `updateCover`
    /// rebuild is driven separately by [`Funcdata`] (it needs the op/block graph
    /// — see [`Varnode::cover_rebuild`]).
    pub fn cover(&self) -> Option<&Cover> {
        self.cover.as_ref()
    }
    /// Delete the Cover object (C++ `Varnode::clearCover`, `varnode.cc:244-251`).
    /// Used for dead Varnodes before full deletion.
    pub fn clear_cover(&mut self) {
        // if (cover != 0) { delete cover; cover = 0; }
        self.cover = None;
    }
    /// Is the cover-dirty flag set (C++ `(flags & Varnode::coverdirty) != 0`)?
    /// Distinct from the HighVariable `isCoverDirty`; this is the Varnode bit.
    pub fn is_cover_dirty_flag(&self) -> bool {
        (self.flags & varnode_flags::coverdirty) != 0
    }
    /// Clear the cover-dirty flag (the tail of `updateCover`).  Goes through
    /// `clear_flags` so any HighVariable notification stays consistent.
    pub fn clear_cover_dirty(&mut self) {
        self.clear_flags(varnode_flags::coverdirty);
    }
    /// Install a freshly-rebuilt Cover (the `cover->rebuild(this)` write-back of
    /// the cross-arena [`Funcdata`]-driven `updateCover`).
    pub fn set_cover(&mut self, c: Cover) {
        self.cover = Some(c);
    }
    /// Initialize a new Cover and set the dirty bit so `updateCover` rebuilds it
    /// (C++ `Varnode::calcCover`, `varnode.cc:254-263`).
    pub fn calc_cover(&mut self) {
        if self.has_cover() {
            // if (cover != 0) delete cover; cover = new Cover;
            self.cover = Some(Cover::new());
            self.set_flags(varnode_flags::coverdirty);
        }
    }
    /// Rebuild the variable cover based on where \b this is defined and read
    /// (C++ `Varnode::updateCover`, `varnode.cc:233-241`).
    ///
    /// This is the cross-arena half of `getCover`: the C++ method calls
    /// `cover->rebuild(this)`, which walks the def/use graph.  The graph access
    /// is supplied by `ctx` (a [`CoverContext`], implemented by [`Funcdata`]).
    /// `self_id` is \b this Varnode's id (the `this` the rebuild traces from).
    /// This is \b only called by the Merge class which knows when to call it.
    pub fn update_cover(&mut self, ctx: &dyn CoverContext, self_id: VarnodeId) {
        if (self.flags & varnode_flags::coverdirty) != 0 {
            // if (hasCover() && (cover != 0)) cover->rebuild(this);
            if self.has_cover() {
                if let Some(cover) = self.cover.as_mut() {
                    cover.rebuild(ctx, self_id);
                }
            }
            self.clear_flags(varnode_flags::coverdirty);
        }
    }
    /// Set the HighVariable owning this Varnode (C++ `setHigh`).
    pub fn set_high(&mut self, tv: HighVariableId, mg: int2) {
        self.high = Some(tv);
        self.mergegroup = mg;
    }

    // --- Consume-list markers (dead-code algorithm) -----------------------

    /// C++ `isConsumeList`.
    pub fn is_consume_list(&self) -> bool {
        (self.addlflags & addl_flags::lisconsume) != 0
    }
    /// C++ `isConsumeVacuous`.
    pub fn is_consume_vacuous(&self) -> bool {
        (self.addlflags & addl_flags::vacconsume) != 0
    }
    /// C++ `setConsumeList`.
    pub fn set_consume_list(&mut self) {
        self.addlflags |= addl_flags::lisconsume;
    }
    /// C++ `setConsumeVacuous`.
    pub fn set_consume_vacuous(&mut self) {
        self.addlflags |= addl_flags::vacconsume;
    }
    /// C++ `clearConsumeList`.
    pub fn clear_consume_list(&mut self) {
        self.addlflags &= !addl_flags::lisconsume;
    }
    /// C++ `clearConsumeVacuous`.
    pub fn clear_consume_vacuous(&mut self) {
        self.addlflags &= !addl_flags::vacconsume;
    }

    // --- Boolean attribute predicates (C++ inline is*/has*) ---------------

    /// C++ `isAnnotation`.
    pub fn is_annotation(&self) -> bool {
        (self.flags & varnode_flags::annotation) != 0
    }
    /// C++ `isImplied`.
    pub fn is_implied(&self) -> bool {
        (self.flags & varnode_flags::implied) != 0
    }
    /// C++ `isExplicit`.
    pub fn is_explicit(&self) -> bool {
        (self.flags & varnode_flags::explict) != 0
    }
    /// C++ `isConstant`.
    pub fn is_constant(&self) -> bool {
        (self.flags & varnode_flags::constant) != 0
    }
    /// C++ `isFree`: not in SSA form (neither written nor input).
    pub fn is_free(&self) -> bool {
        (self.flags & (varnode_flags::written | varnode_flags::input)) == 0
    }
    /// C++ `isInput`.
    pub fn is_input(&self) -> bool {
        (self.flags & varnode_flags::input) != 0
    }
    /// C++ `isIllegalInput`.
    pub fn is_illegal_input(&self) -> bool {
        (self.flags & (varnode_flags::input | varnode_flags::directwrite)) == varnode_flags::input
    }
    /// C++ `isIndirectOnly`.
    pub fn is_indirect_only(&self) -> bool {
        (self.flags & varnode_flags::indirectonly) != 0
    }
    /// C++ `isExternalRef`.
    pub fn is_external_ref(&self) -> bool {
        (self.flags & varnode_flags::externref) != 0
    }
    /// C++ `hasActionProperty`.
    pub fn has_action_property(&self) -> bool {
        (self.flags & (varnode_flags::readonly | varnode_flags::volatil)) != 0
    }
    /// C++ `isReadOnly`.
    pub fn is_read_only(&self) -> bool {
        (self.flags & varnode_flags::readonly) != 0
    }
    /// C++ `isVolatile`.
    pub fn is_volatile(&self) -> bool {
        (self.flags & varnode_flags::volatil) != 0
    }
    /// C++ `isPersist`.
    pub fn is_persist(&self) -> bool {
        (self.flags & varnode_flags::persist) != 0
    }
    /// C++ `isDirectWrite`.
    pub fn is_direct_write(&self) -> bool {
        (self.flags & varnode_flags::directwrite) != 0
    }
    /// C++ `isAddrTied`.
    pub fn is_addr_tied(&self) -> bool {
        (self.flags & (varnode_flags::addrtied | varnode_flags::insert))
            == (varnode_flags::addrtied | varnode_flags::insert)
    }
    /// C++ `isAddrForce`.
    pub fn is_addr_force(&self) -> bool {
        (self.flags & varnode_flags::addrforce) != 0
    }
    /// C++ `isAutoLive`.
    pub fn is_auto_live(&self) -> bool {
        (self.flags & (varnode_flags::addrforce | varnode_flags::autolive_hold)) != 0
    }
    /// C++ `isAutoLiveHold`.
    pub fn is_auto_live_hold(&self) -> bool {
        (self.flags & varnode_flags::autolive_hold) != 0
    }
    /// C++ `isMapped`.
    pub fn is_mapped(&self) -> bool {
        (self.flags & varnode_flags::mapped) != 0
    }
    /// C++ `isUnaffected`.
    pub fn is_unaffected(&self) -> bool {
        (self.flags & varnode_flags::unaffected) != 0
    }
    /// C++ `isSpacebase`.
    pub fn is_spacebase(&self) -> bool {
        (self.flags & varnode_flags::spacebase) != 0
    }
    /// C++ `isReturnAddress`.
    pub fn is_return_address(&self) -> bool {
        (self.flags & varnode_flags::return_address) != 0
    }
    /// C++ `isProtoPartial`.
    pub fn is_proto_partial(&self) -> bool {
        (self.flags & varnode_flags::proto_partial) != 0
    }
    /// C++ `isPtrCheck`.
    pub fn is_ptr_check(&self) -> bool {
        (self.addlflags & addl_flags::ptrcheck) != 0
    }
    /// C++ `isPtrFlow`.
    pub fn is_ptr_flow(&self) -> bool {
        (self.addlflags & addl_flags::ptrflow) != 0
    }
    /// C++ `isSpacebasePlaceholder`.
    pub fn is_spacebase_placeholder(&self) -> bool {
        (self.addlflags & addl_flags::spacebase_placeholder) != 0
    }
    /// C++ `hasNoLocalAlias`.
    pub fn has_no_local_alias(&self) -> bool {
        (self.flags & varnode_flags::nolocalalias) != 0
    }
    /// C++ `isMark`.
    pub fn is_mark(&self) -> bool {
        (self.flags & varnode_flags::mark) != 0
    }
    /// C++ `isActiveHeritage`.
    pub fn is_active_heritage(&self) -> bool {
        (self.addlflags & addl_flags::activeheritage) != 0
    }
    /// C++ `isStackStore`.
    pub fn is_stack_store(&self) -> bool {
        (self.addlflags & addl_flags::stack_store) != 0
    }
    /// C++ `isLockedInput`.
    pub fn is_locked_input(&self) -> bool {
        (self.addlflags & addl_flags::locked_input) != 0
    }
    /// C++ `stopsUpPropagation`.
    pub fn stops_up_propagation(&self) -> bool {
        (self.addlflags & addl_flags::stop_uppropagation) != 0
    }
    /// C++ `hasImpliedField`.
    pub fn has_implied_field(&self) -> bool {
        (self.addlflags & addl_flags::has_implied_field) != 0
    }
    /// C++ `isIndirectZero`.
    pub fn is_indirect_zero(&self) -> bool {
        (self.flags & (varnode_flags::indirect_creation | varnode_flags::constant))
            == (varnode_flags::indirect_creation | varnode_flags::constant)
    }
    /// C++ `isExtraOut`.
    pub fn is_extra_out(&self) -> bool {
        (self.flags & (varnode_flags::indirect_creation | varnode_flags::addrtied))
            == varnode_flags::indirect_creation
    }
    /// C++ `isPrecisLo`.
    pub fn is_precis_lo(&self) -> bool {
        (self.flags & varnode_flags::precislo) != 0
    }
    /// C++ `isPrecisHi`.
    pub fn is_precis_hi(&self) -> bool {
        (self.flags & varnode_flags::precishi) != 0
    }
    /// C++ `isIncidentalCopy`.
    pub fn is_incidental_copy(&self) -> bool {
        (self.flags & varnode_flags::incidental_copy) != 0
    }
    /// C++ `isWriteMask`.
    pub fn is_write_mask(&self) -> bool {
        (self.addlflags & addl_flags::writemask) != 0
    }
    /// C++ `isUnsignedPrint`.
    pub fn is_unsigned_print(&self) -> bool {
        (self.addlflags & addl_flags::unsignedprint) != 0
    }
    /// C++ `isLongPrint`.
    pub fn is_long_print(&self) -> bool {
        (self.addlflags & addl_flags::longprint) != 0
    }
    /// C++ `isWritten`.
    pub fn is_written(&self) -> bool {
        (self.flags & varnode_flags::written) != 0
    }
    /// C++ `hasCover`.
    pub fn has_cover(&self) -> bool {
        (self.flags & (varnode_flags::constant | varnode_flags::annotation | varnode_flags::insert))
            == varnode_flags::insert
    }
    /// C++ `hasNoDescend`.
    pub fn has_no_descend(&self) -> bool {
        self.descend.is_empty()
    }
    /// C++ `constantMatch`.
    pub fn constant_match(&self, val: uintb) -> bool {
        if !self.is_constant() {
            return false;
        }
        self.loc.get_offset() == val
    }
    /// C++ `isHeritageKnown`.
    pub fn is_heritage_known(&self) -> bool {
        (self.flags
            & (varnode_flags::insert | varnode_flags::constant | varnode_flags::annotation))
            != 0
    }
    /// C++ `isTypeLock`.
    pub fn is_type_lock(&self) -> bool {
        (self.flags & varnode_flags::typelock) != 0
    }
    /// C++ `isNameLock`.
    pub fn is_name_lock(&self) -> bool {
        (self.flags & varnode_flags::namelock) != 0
    }

    // --- Public flag mutators (C++ inline set*/clear*) --------------------

    /// C++ `setActiveHeritage`.
    pub fn set_active_heritage(&mut self) {
        self.addlflags |= addl_flags::activeheritage;
    }
    /// C++ `clearActiveHeritage`.
    pub fn clear_active_heritage(&mut self) {
        self.addlflags &= !addl_flags::activeheritage;
    }
    /// C++ `setMark`.
    pub fn set_mark(&mut self) {
        self.flags |= varnode_flags::mark;
    }
    /// C++ `clearMark`.
    pub fn clear_mark(&mut self) {
        self.flags &= !varnode_flags::mark;
    }
    /// C++ `setDirectWrite`.
    pub fn set_direct_write(&mut self) {
        self.flags |= varnode_flags::directwrite;
    }
    /// C++ `clearDirectWrite`.
    pub fn clear_direct_write(&mut self) {
        self.flags &= !varnode_flags::directwrite;
    }
    /// C++ `setReturnAddress`.
    pub fn set_return_address(&mut self) {
        self.flags |= varnode_flags::return_address;
    }
    /// C++ `clearReturnAddress`.
    pub fn clear_return_address(&mut self) {
        self.flags &= !varnode_flags::return_address;
    }
    /// C++ `setPtrCheck`.
    pub fn set_ptr_check(&mut self) {
        self.addlflags |= addl_flags::ptrcheck;
    }
    /// C++ `clearPtrCheck`.
    pub fn clear_ptr_check(&mut self) {
        self.addlflags &= !addl_flags::ptrcheck;
    }
    /// C++ `setPtrFlow`.
    pub fn set_ptr_flow(&mut self) {
        self.addlflags |= addl_flags::ptrflow;
    }
    /// C++ `clearPtrFlow`.
    pub fn clear_ptr_flow(&mut self) {
        self.addlflags &= !addl_flags::ptrflow;
    }
    /// C++ `setSpacebasePlaceholder`.
    pub fn set_spacebase_placeholder(&mut self) {
        self.addlflags |= addl_flags::spacebase_placeholder;
    }
    /// C++ `clearSpacebasePlaceholder`.
    pub fn clear_spacebase_placeholder(&mut self) {
        self.addlflags &= !addl_flags::spacebase_placeholder;
    }
    /// C++ `setWriteMask`.
    pub fn set_write_mask(&mut self) {
        self.addlflags |= addl_flags::writemask;
    }
    /// C++ `clearWriteMask`.
    pub fn clear_write_mask(&mut self) {
        self.addlflags &= !addl_flags::writemask;
    }
    /// C++ `setProtoPartial`.
    pub fn set_proto_partial(&mut self) {
        self.flags |= varnode_flags::proto_partial;
    }
    /// C++ `clearProtoPartial`.
    pub fn clear_proto_partial(&mut self) {
        self.flags &= !varnode_flags::proto_partial;
    }
    /// C++ `setUnsignedPrint`.
    pub fn set_unsigned_print(&mut self) {
        self.addlflags |= addl_flags::unsignedprint;
    }
    /// C++ `setLongPrint`.
    pub fn set_long_print(&mut self) {
        self.addlflags |= addl_flags::longprint;
    }
    /// C++ `setStopUpPropagation`.
    pub fn set_stop_up_propagation(&mut self) {
        self.addlflags |= addl_flags::stop_uppropagation;
    }
    /// C++ `clearStopUpPropagation`.
    pub fn clear_stop_up_propagation(&mut self) {
        self.addlflags &= !addl_flags::stop_uppropagation;
    }
    /// C++ `setImpliedField`.
    pub fn set_implied_field(&mut self) {
        self.addlflags |= addl_flags::has_implied_field;
    }
    /// C++ `setStackStore`.
    pub fn set_stack_store(&mut self) {
        self.addlflags |= addl_flags::stack_store;
    }
    /// C++ `setLockedInput`.
    pub fn set_locked_input(&mut self) {
        self.addlflags |= addl_flags::locked_input;
    }

    // --- Flag mutators routed through setFlags/clearFlags -----------------
    //
    // The C++ setFlags/clearFlags also notify the owning HighVariable
    // (`high->flagsDirty()`, and `high->coverDirty()` when the coverdirty bit
    // moves).  In the ADR 0001 arena model the HighVariable lives in
    // `Funcdata::high_bank`, a different arena the Varnode cannot reach from a
    // `&mut self` method, so the cross-arena notification is reconciled at the
    // Funcdata level: the `HighVariable` re-derives its inherited flags lazily
    // from the live member-Varnode flags (`HighVariable::update_flags`, gated on
    // its own `flagsdirty`), and the cover rebuild is driven by
    // `Funcdata::update_varnode_cover` / `HighVariableBank::cover_dirty`.  See
    // `docs/rust-port/losses.md` (W7 setFlags→high notification).  The exact bit
    // changes and the routing through these helpers are preserved.

    /// C++ `setFlags` (`varnode.cc:371`): set the bits.  The HighVariable
    /// `flagsDirty`/`coverDirty` notification is reconciled at the Funcdata level
    /// (see the block comment above) — SEAM(W7).
    fn set_flags(&mut self, fl: uint4) {
        self.flags |= fl;
        // if (high != null) { high->flagsDirty(); if (fl&coverdirty) high->coverDirty(); }
        //   -- SEAM(W7): high lives in Funcdata::high_bank; reconciled there.
    }

    /// C++ `clearFlags` (`varnode.cc:384`): clear the bits.  HighVariable
    /// notification reconciled at the Funcdata level — SEAM(W7) (see set_flags).
    fn clear_flags(&mut self, fl: uint4) {
        self.flags &= !fl;
        // SEAM(W7): HighVariable notification reconciled in Funcdata.
    }

    /// C++ `setUnaffected`.
    pub fn set_unaffected(&mut self) {
        self.set_flags(varnode_flags::unaffected);
    }
    /// Mark this Varnode as an \e annotation (C++ has no dedicated setter; the
    /// `Funcdata::newCodeRef`/`newVarnodeIop`/`newVarnodeCallSpecs` factories do
    /// `vn->setFlags(Varnode::annotation)` directly).  `set_flags` is private to
    /// this module, so this `pub(crate)` sliver is the single primitive the
    /// `funcdata_varnode` factories use to set the bit (LOSS-077 restoration).
    pub(crate) fn set_annotation(&mut self) {
        self.set_flags(varnode_flags::annotation);
    }
    /// Test helper: mark this Varnode as \e inserted (output of an op / input /
    /// constant), so `hasCover()` is true.  The C++ sets this through the
    /// xref/heritage paths (W3-op's `setInput`/`setDef`); the W7 cover tests need
    /// the bit set directly without the full xref dance.
    #[cfg(test)]
    pub(crate) fn set_insert_for_test(&mut self) {
        self.set_flags(varnode_flags::insert);
    }
    /// Test helper: mark this Varnode as an \e input (W7 cover tests).
    #[cfg(test)]
    pub(crate) fn set_input_for_test(&mut self) {
        self.set_input_flag();
    }
    /// C++ `setInput` (the private bank helper): mark as input + coverdirty.
    fn set_input_flag(&mut self) {
        self.set_flags(varnode_flags::input | varnode_flags::coverdirty);
    }
    /// C++ `setAddrForce`.
    pub fn set_addr_force(&mut self) {
        self.set_flags(varnode_flags::addrforce);
    }
    /// C++ `clearAddrForce`.
    pub fn clear_addr_force(&mut self) {
        self.clear_flags(varnode_flags::addrforce);
    }
    /// C++ `setImplied`.
    pub fn set_implied(&mut self) {
        self.set_flags(varnode_flags::implied);
    }
    /// C++ `clearImplied`.
    pub fn clear_implied(&mut self) {
        self.clear_flags(varnode_flags::implied);
    }
    /// C++ `setExplicit`.
    pub fn set_explicit(&mut self) {
        self.set_flags(varnode_flags::explict);
    }
    /// C++ `clearExplicit`.
    pub fn clear_explicit(&mut self) {
        self.clear_flags(varnode_flags::explict);
    }
    /// C++ `setPrecisLo`.
    pub fn set_precis_lo(&mut self) {
        self.set_flags(varnode_flags::precislo);
    }
    /// C++ `clearPrecisLo`.
    pub fn clear_precis_lo(&mut self) {
        self.clear_flags(varnode_flags::precislo);
    }
    /// C++ `setPrecisHi`.
    pub fn set_precis_hi(&mut self) {
        self.set_flags(varnode_flags::precishi);
    }
    /// C++ `clearPrecisHi`.
    pub fn clear_precis_hi(&mut self) {
        self.clear_flags(varnode_flags::precishi);
    }
    /// C++ `setAutoLiveHold` (note: direct bit set, not via setFlags).
    pub fn set_auto_live_hold(&mut self) {
        self.flags |= varnode_flags::autolive_hold;
    }
    /// C++ `clearAutoLiveHold`.
    pub fn clear_auto_live_hold(&mut self) {
        self.flags &= !varnode_flags::autolive_hold;
    }

    // --- def / descend management (the private bank-facing helpers) -------

    /// Directly change the defining op and set appropriate dirty bits
    /// (C++ `Varnode::setDef`, `varnode.cc:413`).
    fn set_def(&mut self, op: Option<OpId>) {
        self.def = op;
        if op.is_none() {
            self.set_flags(varnode_flags::coverdirty);
            self.clear_flags(varnode_flags::written);
        } else {
            self.set_flags(varnode_flags::coverdirty | varnode_flags::written);
        }
    }

    /// Put a new operator in the descendant list and set the cover dirty flag
    /// (C++ `Varnode::addDescend`, `varnode.cc:330`).
    ///
    /// Returns an error if a free (non-spacebase) varnode would gain a second
    /// descendant (the C++ `LowlevelError`).
    fn add_descend(&mut self, op: OpId) -> KunaResult<()> {
        // if (isFree() && !isSpacebase()) { if (!descend.empty()) throw ... }
        if self.is_free() && !self.is_spacebase() && !self.descend.is_empty() {
            return Err(KunaError::lowlevel("Free varnode has multiple descendants"));
        }
        self.descend.push(op); // push_back
        self.set_flags(varnode_flags::coverdirty);
        Ok(())
    }

    /// Erase an operation from the descendant list and set cover dirty
    /// (C++ `Varnode::eraseDescend`, `varnode.cc:316`).
    ///
    /// Mirrors the C++ "find then erase" — the C++ loops until `*iter == op`
    /// and would walk off the end (UB) if `op` is absent; here that is a
    /// caught internal-invariant violation (panic per ADR 0004).
    fn erase_descend(&mut self, op: OpId) {
        let pos = self
            .descend
            .iter()
            .position(|&o| o == op)
            .expect("eraseDescend: op not in descend list (C++ would walk off the end)");
        self.descend.remove(pos);
        self.set_flags(varnode_flags::coverdirty);
    }

    /// Completely clear the descendant list (C++ `Varnode::destroyDescend`).
    fn destroy_descend(&mut self) {
        self.descend.clear();
    }

    // --- Geometry / comparison (no external-type dependencies) ------------

    /// Return containment info of another Varnode in \b this (C++ `contains`).
    ///
    /// Returns -1, 0, 1, 2, or 3 per the C++ contract (`varnode.cc:105`).
    pub fn contains(&self, op: &Varnode) -> int4 {
        // if (loc.getSpace() != op.loc.getSpace()) return 3;
        if !same_space(&self.loc, &op.loc) {
            return 3;
        }
        // if (loc.getSpace()->getType()==IPTR_CONSTANT) return 3;
        if self.get_space().get_type() == spacetype::IPTR_CONSTANT {
            return 3;
        }
        let a = self.loc.get_offset();
        let b = op.loc.get_offset();
        if b < a {
            return -1;
        }
        // if (b>=a+size) return 2;  (uintb arithmetic, int4 size sign-extended)
        if b >= a.wadd(self.size as i64 as u64) {
            return 2;
        }
        // if (b+op.size > a+size) return 1;
        if b.wadd(op.size as i64 as u64) > a.wadd(self.size as i64 as u64) {
            return 1;
        }
        0
    }

    /// Check whether the storage locations of two varnodes intersect
    /// (C++ `intersects(const Varnode&)`).
    pub fn intersects(&self, op: &Varnode) -> bool {
        self.intersects_range(&op.loc, op.size)
    }

    /// Check intersection against an Address range
    /// (C++ `intersects(const Address&,int4)`).
    pub fn intersects_range(&self, op2loc: &Address, op2size: int4) -> bool {
        if !same_space(&self.loc, op2loc) {
            return false;
        }
        if self.get_space().get_type() == spacetype::IPTR_CONSTANT {
            return false;
        }
        let a = self.loc.get_offset();
        let b = op2loc.get_offset();
        if b < a {
            // if (a>=b+op2size) return false;  return true;
            if a >= b.wadd(op2size as i64 as u64) {
                return false;
            }
            return true;
        }
        // if (b>=a+size) return false;  return true;
        if b >= a.wadd(self.size as i64 as u64) {
            return false;
        }
        true
    }

    /// Return 0, 1, or 2 for "no overlap", "partial overlap", "identical
    /// storage" (C++ `characterizeOverlap`).
    pub fn characterize_overlap(&self, op: &Varnode) -> int4 {
        if !same_space(&self.loc, &op.loc) {
            return 0;
        }
        let off = self.loc.get_offset();
        let opoff = op.loc.get_offset();
        if off == opoff {
            // Left sides match: either total match or partial
            if self.size == op.size {
                2
            } else {
                1
            }
        } else if off < opoff {
            // uintb thisright = loc.getOffset() + (size-1);  Test if this ends before op begins
            let thisright = off.wadd((self.size - 1) as i64 as u64);
            if thisright < opoff {
                0
            } else {
                1
            }
        } else {
            // Test if op ends before this begins
            let opright = opoff.wadd((op.size - 1) as i64 as u64);
            if opright < off {
                0
            } else {
                1
            }
        }
    }

    /// Return relative point of overlap between two Varnodes, or -1
    /// (C++ `overlap(const Varnode&)`).
    pub fn overlap(&self, op: &Varnode) -> int4 {
        if !self.loc.is_big_endian() {
            self.loc.overlap(0, &op.loc, op.size)
        } else {
            let over = self.loc.overlap(self.size - 1, &op.loc, op.size);
            if over != -1 {
                return op.size - 1 - over;
            }
            -1
        }
    }

    /// Return relative overlap, where the given Varnode may be in the \e join
    /// space (C++ `overlapJoin`).
    pub fn overlap_join(&self, op: &Varnode) -> KunaResult<int4> {
        if !self.loc.is_big_endian() {
            self.loc.overlap_join(0, &op.loc, op.size)
        } else {
            let over = self.loc.overlap_join(self.size - 1, &op.loc, op.size)?;
            if over != -1 {
                return Ok(op.size - 1 - over);
            }
            Ok(-1)
        }
    }

    /// Return relative point of overlap with an Address range, or -1
    /// (C++ `overlap(const Address&,int4)`).
    pub fn overlap_range(&self, op2loc: &Address, op2size: int4) -> int4 {
        if !self.loc.is_big_endian() {
            self.loc.overlap(0, op2loc, op2size)
        } else {
            let over = self.loc.overlap(self.size - 1, op2loc, op2size);
            if over != -1 {
                return op2size - 1 - over;
            }
            -1
        }
    }

    /// Does this Varnode hold a formal boolean value (C++ `isBooleanValue`)?
    ///
    /// Only the cases the W3 data-model can decide without an op/def graph are
    /// transcribed: the `useAnnotation` annotation case and the size-1 +
    /// `TYPE_BOOL` case.  The `def->code()` opcode test requires the PcodeOp
    /// graph (SEAM(W3): op) and is deferred to the op-aware caller.
    pub fn is_boolean_value(&self, use_annotation: bool) -> bool {
        // if (isWritten()) return (def->code() == CPUI_...);  -- SEAM(W3): op
        if self.is_written() {
            return false; // op-graph branch deferred to the op-aware caller
        }
        if use_annotation && !self.is_free() {
            // if ((flags & (input|typelock)) == (input|typelock))
            if (self.flags & (varnode_flags::input | varnode_flags::typelock))
                == (varnode_flags::input | varnode_flags::typelock)
            {
                return self.size == 1 && self.type_.get_metatype() == type_metatype::TYPE_BOOL;
            }
        }
        false
    }

    // --- Datatype updates (only the dep-free parts) -----------------------

    /// Set the Datatype if not locked (C++ `updateType(Datatype*)`).
    /// Returns true if the Datatype changed.
    pub fn update_type(&mut self, ct: Rc<Datatype>) -> bool {
        // if (type == ct || isTypeLock()) return false;  (pointer identity)
        if Rc::ptr_eq(&self.type_, &ct) || self.is_type_lock() {
            return false;
        }
        self.type_ = ct;
        // if (high != null) high->typeDirty();  -- SEAM(W7)
        true
    }

    /// (Possibly) set the Datatype given lock/override restrictions
    /// (C++ `updateType(Datatype*,bool,bool)`).
    pub fn update_type_locked(&mut self, ct: Rc<Datatype>, mut lock: bool, over: bool) -> bool {
        if ct.get_metatype() == type_metatype::TYPE_UNKNOWN {
            lock = false; // Unknown data type is ALWAYS unlocked
        }
        if self.is_type_lock() && !over {
            return false;
        }
        if Rc::ptr_eq(&self.type_, &ct) && (self.is_type_lock() == lock) {
            return false;
        }
        self.flags &= !varnode_flags::typelock;
        if lock {
            self.flags |= varnode_flags::typelock;
        }
        self.type_ = ct;
        // if (high != null) high->typeDirty();  -- SEAM(W7)
        true
    }
}

/// Compare the spaces of two addresses by raw pointer identity, as the C++
/// `loc.getSpace() != op.loc.getSpace()` does.  Two invalid (null-space)
/// addresses compare equal (both null).
fn same_space(a: &Address, b: &Address) -> bool {
    match (a.get_space(), b.get_space()) {
        (Some(x), Some(y)) => Rc::ptr_eq(x, y),
        (None, None) => true,
        _ => false,
    }
}

// ---------------------------------------------------------------------------
// VarnodeBank
// ---------------------------------------------------------------------------

/// A container for Varnode objects from a specific function (C++
/// `class VarnodeBank`).
///
/// The bank owns the [`VarnodeId`]-keyed slotmap arena (ADR 0001) and the two
/// ordering trees (ADR 0002).  The C++ `searchvn` template-varnode trick is
/// replaced by *constructed bound keys* in the range queries; no mutable
/// search node is needed.
///
/// SEAM: the def-op `SeqNum`/`getTime`/`getAddr` lookups used by `find` are
/// supplied through accessors the caller passes in (filled by `op`/`funcdata`,
/// `w3-ir-op`).  The trees store the `SeqNum` in their keys so ordering is
/// self-contained; only `find`'s exact `getTime` match needs to observe an op
/// live, which the caller already has from the defining op.
pub struct VarnodeBank {
    /// Base for unique addresses (C++ `uniqbase`)
    uniqbase: uintm,
    /// Counter for generating unique offsets (C++ `uniqid`)
    uniqid: uintm,
    /// Number of varnodes created (C++ `create_index`)
    create_index: uint4,
    /// The varnode arena (ADR 0001): the bank owns the store
    arena: SlotMap<VarnodeId, Varnode>,
    /// Varnodes sorted by location then def (C++ `loc_tree`)
    loc_tree: BTreeMap<LocKey, VarnodeId>,
    /// Varnodes sorted by def then location (C++ `def_tree`)
    def_tree: BTreeMap<DefKey, VarnodeId>,
    /// The unique space, cached at construction (C++ `uniq_space`)
    uniq_space: Rc<AddrSpace>,
}

/// A defining op's identity, as the [`VarnodeBank`] needs it to build keys for
/// written varnodes (the def's `SeqNum`).
///
/// SEAM(W3): the real `PcodeOp` (with `getSeqNum`/`getTime`/`getAddr`) is
/// `op`'s.  The bank operations that turn a varnode *written* (`set_def`,
/// `create_def`) take this small carrier so the def tree can sort on the op's
/// SeqNum without naming `PcodeOp`.
#[derive(Debug, Clone)]
pub struct DefOpInfo {
    /// The op's id (stored as the varnode's `def`).
    pub id: OpId,
    /// The op's `SeqNum` (C++ `op->getSeqNum()`), used by both comparators.
    pub seqnum: SeqNum,
}

/// Callback type for the `replace(oldvn, newvn)` op-rewiring that `xref`
/// performs when it unifies a varnode with an equivalent existing one.  The
/// real rewiring touches the op graph and is the caller's (SEAM(W3):
/// `funcdata`); the bank only sequences it.
pub type ReplaceReads<'a> =
    dyn FnMut(&mut VarnodeBank, VarnodeId, VarnodeId) -> KunaResult<()> + 'a;

impl VarnodeBank {
    /// Construct the container (C++ `VarnodeBank(AddrSpaceManager*)`).
    ///
    /// The C++ pulls `uniqbase` from the unique space's `Translate`
    /// (`getUniqueStart(Translate::ANALYSIS)`); that Translate is the
    /// sleigh-runtime's but is not wired into the W3 data-model boot yet, so
    /// the analysis unique-start is passed in by the caller (`uniq_start`).
    /// SEAM(W3): `funcdata`/`op` supply it from the program's Translate.
    pub fn new(manage: &AddrSpaceManager, uniq_start: uintm) -> KunaResult<VarnodeBank> {
        let uniq_space = manage
            .get_unique_space()
            .ok_or_else(|| KunaError::lowlevel("VarnodeBank: no unique space registered"))?;
        let uniq_space = Rc::clone(uniq_space);
        Ok(VarnodeBank {
            uniqbase: uniq_start,
            uniqid: uniq_start,
            create_index: 0,
            arena: SlotMap::with_key(),
            loc_tree: BTreeMap::new(),
            def_tree: BTreeMap::new(),
            uniq_space,
        })
    }

    /// Clear out all Varnodes and reset counters (C++ `clear`).
    pub fn clear(&mut self) {
        self.arena.clear();
        self.loc_tree.clear();
        self.def_tree.clear();
        self.uniqid = self.uniqbase; // Reset counter to base value
        self.create_index = 0; // Reset varnode creation index
    }

    /// Number of Varnodes (C++ `numVarnodes`).
    pub fn num_varnodes(&self) -> int4 {
        self.loc_tree.len() as int4
    }

    /// The next creation index to be assigned (C++ `getCreateIndex`).
    pub fn get_create_index(&self) -> uint4 {
        self.create_index
    }

    /// Borrow a Varnode by id.
    pub fn get(&self, id: VarnodeId) -> Option<&Varnode> {
        self.arena.get(id)
    }
    /// Mutably borrow a Varnode by id.
    pub fn get_mut(&mut self, id: VarnodeId) -> Option<&mut Varnode> {
        self.arena.get_mut(id)
    }

    /// Build the location-tree key for the varnode at `id`.
    fn loc_key_of(&self, id: VarnodeId) -> LocKey {
        let vn = &self.arena[id];
        LocKey {
            addr: vn.loc.clone(),
            size: vn.size,
            flagclass: flag_class_of(vn.flags),
            seqnum: vn.def_seqnum.clone().unwrap_or_default(),
            create_index: vn.create_index,
        }
    }

    /// Build the definition-tree key for the varnode at `id`.
    fn def_key_of(&self, id: VarnodeId) -> DefKey {
        let vn = &self.arena[id];
        DefKey {
            addr: vn.loc.clone(),
            size: vn.size,
            flagclass: flag_class_of(vn.flags),
            seqnum: vn.def_seqnum.clone().unwrap_or_default(),
            create_index: vn.create_index,
        }
    }

    /// Create a \e free Varnode and insert it into both trees (C++ `create`).
    pub fn create(&mut self, s: int4, m: Address, ct: Rc<Datatype>) -> VarnodeId {
        let mut vn = Varnode::new(s, m, ct);
        // vn->create_index = create_index++;
        vn.create_index = self.create_index;
        self.create_index = self.create_index.wadd(1);
        let id = self.arena.insert(vn);
        // Frees can always be inserted without duplication.
        let lk = self.loc_key_of(id);
        let dk = self.def_key_of(id);
        self.arena[id].lociter = Some(lk.clone());
        self.arena[id].defiter = Some(dk.clone());
        self.loc_tree.insert(lk, id);
        self.def_tree.insert(dk, id);
        id
    }

    /// Create a temporary varnode in the unique space (C++ `createUnique`).
    pub fn create_unique(&mut self, s: int4, ct: Rc<Datatype>) -> VarnodeId {
        // Address addr(uniq_space, uniqid);  uniqid += s;
        let addr = Address::new(Rc::clone(&self.uniq_space), self.uniqid as u64);
        self.uniqid = self.uniqid.wadd(s as uintm);
        self.create(s, addr, ct)
    }

    /// Remove a Varnode from the container and reclaim it (C++ `destroy`).
    pub fn destroy(&mut self, vn: VarnodeId) -> KunaResult<()> {
        // if ((vn->getDef() != null) || (!vn->hasNoDescend())) throw ...
        let v = &self.arena[vn];
        if v.def.is_some() || !v.has_no_descend() {
            return Err(KunaError::lowlevel("Deleting integrated varnode"));
        }
        let lk = self.arena[vn].lociter.take().expect("destroy: missing lociter");
        let dk = self.arena[vn].defiter.take().expect("destroy: missing defiter");
        self.loc_tree.remove(&lk);
        self.def_tree.remove(&dk);
        self.arena.remove(vn);
        Ok(())
    }

    /// Insert a Varnode into both sorted trees, unifying with an equivalent
    /// existing free varnode if present (C++ `xref`, `varnode.cc:1310`).
    ///
    /// If the loc tree already contains an equivalent varnode, the existing one
    /// is kept: `replace_reads` patches every op reading `vn` onto it, `vn` is
    /// removed from the arena, and the existing id is returned.  Otherwise the
    /// varnode is freshly inserted, marked `insert`, and added to the def tree.
    fn xref(&mut self, vn: VarnodeId, replace_reads: &mut ReplaceReads) -> KunaResult<VarnodeId> {
        let lk = self.loc_key_of(vn);
        // check = loc_tree.insert(vn);  if (!check.second) { ... }
        if let Some(&othervn) = self.loc_tree.get(&lk) {
            // Set already contains this varnode.
            replace_reads(self, vn, othervn)?; // Patch ops using the old varnode
            self.arena.remove(vn); // delete vn
            return Ok(othervn);
        }
        // Otherwise a new insertion.  The `insert` flag is outside the
        // (input|written) mask, so it does not change the loc/def keys.
        self.arena[vn].set_flags(varnode_flags::insert);
        self.loc_tree.insert(lk.clone(), vn);
        self.arena[vn].lociter = Some(lk);
        let dk = self.def_key_of(vn);
        self.def_tree.insert(dk.clone(), vn); // new in def_tree
        self.arena[vn].defiter = Some(dk);
        Ok(vn)
    }

    /// Define a Varnode as an input formally (C++ `setInput`).
    pub fn set_input(
        &mut self,
        vn: VarnodeId,
        replace_reads: &mut ReplaceReads,
    ) -> KunaResult<VarnodeId> {
        let v = &self.arena[vn];
        if !v.is_free() {
            return Err(KunaError::lowlevel("Making input out of varnode which is not free"));
        }
        if v.is_constant() {
            return Err(KunaError::lowlevel("Making input out of constant varnode"));
        }
        // Erase the free version of the varnode from both trees.
        let lk = self.arena[vn].lociter.take().expect("set_input: missing lociter");
        let dk = self.arena[vn].defiter.take().expect("set_input: missing defiter");
        self.loc_tree.remove(&lk);
        self.def_tree.remove(&dk);
        self.arena[vn].set_input_flag(); // Set the input flag
        self.xref(vn, replace_reads)
    }

    /// Change a Varnode to be defined by the given op (C++ `setDef`).
    ///
    /// `def` carries the op id and its `SeqNum` (the comparator's def order).
    /// The error addresses in the C++ are rendered with the op address (the
    /// op-address accessor is SEAM(W3)); the error condition is identical.
    pub fn set_def(
        &mut self,
        vn: VarnodeId,
        def: DefOpInfo,
        replace_reads: &mut ReplaceReads,
    ) -> KunaResult<VarnodeId> {
        let v = &self.arena[vn];
        if !v.is_free() {
            return Err(KunaError::lowlevel("Defining varnode which is not free"));
        }
        if v.is_constant() {
            return Err(KunaError::lowlevel("Assignment to constant"));
        }
        let lk = self.arena[vn].lociter.take().expect("set_def: missing lociter");
        let dk = self.arena[vn].defiter.take().expect("set_def: missing defiter");
        self.loc_tree.remove(&lk);
        self.def_tree.remove(&dk);
        self.arena[vn].set_def(Some(def.id)); // Change the varnode to be defined
        self.arena[vn].def_seqnum = Some(def.seqnum);
        self.xref(vn, replace_reads)
    }

    /// Convert a Varnode to be \e free (C++ `makeFree`).
    pub fn make_free(&mut self, vn: VarnodeId) {
        let lk = self.arena[vn].lociter.take().expect("make_free: missing lociter");
        let dk = self.arena[vn].defiter.take().expect("make_free: missing defiter");
        self.loc_tree.remove(&lk);
        self.def_tree.remove(&dk);

        self.arena[vn].set_def(None); // Clear things that make vn non-free
        self.arena[vn].def_seqnum = None;
        self.arena[vn].clear_flags(
            varnode_flags::insert | varnode_flags::input | varnode_flags::indirect_creation,
        );

        // Re-insert as free varnode.
        let lk = self.loc_key_of(vn);
        let dk = self.def_key_of(vn);
        self.arena[vn].lociter = Some(lk.clone());
        self.arena[vn].defiter = Some(dk.clone());
        self.loc_tree.insert(lk, vn);
        self.def_tree.insert(dk, vn);
    }

    /// Create a Varnode already in the def tree as if it were the output of the
    /// given op (C++ `createDef`).
    pub fn create_def(
        &mut self,
        s: int4,
        m: Address,
        ct: Rc<Datatype>,
        def: DefOpInfo,
        replace_reads: &mut ReplaceReads,
    ) -> KunaResult<VarnodeId> {
        let mut vn = Varnode::new(s, m, ct);
        vn.create_index = self.create_index;
        self.create_index = self.create_index.wadd(1);
        vn.set_def(Some(def.id));
        vn.def_seqnum = Some(def.seqnum);
        let id = self.arena.insert(vn);
        self.xref(id, replace_reads)
    }

    /// Create a unique Varnode as output of the given op (C++ `createDefUnique`).
    pub fn create_def_unique(
        &mut self,
        s: int4,
        ct: Rc<Datatype>,
        def: DefOpInfo,
        replace_reads: &mut ReplaceReads,
    ) -> KunaResult<VarnodeId> {
        let addr = Address::new(Rc::clone(&self.uniq_space), self.uniqid as u64);
        self.uniqid = self.uniqid.wadd(s as uintm);
        self.create_def(s, addr, ct, def, replace_reads)
    }

    // --- descend manipulation routed through the bank ---------------------

    /// Add `op` to `vn`'s descend list (C++ `vn->addDescend(op)`).
    pub fn add_descend(&mut self, vn: VarnodeId, op: OpId) -> KunaResult<()> {
        self.arena[vn].add_descend(op)
    }
    /// Erase `op` from `vn`'s descend list (C++ `vn->eraseDescend(op)`).
    pub fn erase_descend(&mut self, vn: VarnodeId, op: OpId) {
        self.arena[vn].erase_descend(op);
    }
    /// Clear `vn`'s descend list entirely (C++ `vn->destroyDescend()`).
    pub fn destroy_descend(&mut self, vn: VarnodeId) {
        self.arena[vn].destroy_descend();
    }

    // --- Iteration & lookup ----------------------------------------------

    /// Iterate all varnode ids in location order (C++ `beginLoc()`..`endLoc()`).
    pub fn iter_loc(&self) -> impl Iterator<Item = VarnodeId> + '_ {
        self.loc_tree.values().copied()
    }

    /// Iterate all varnode ids in definition order (C++ `beginDef()`..`endDef()`).
    pub fn iter_def(&self) -> impl Iterator<Item = VarnodeId> + '_ {
        self.def_tree.values().copied()
    }

    /// Iterate varnode ids whose location lies in `[start, end)` of the loc
    /// tree, where the bounds are constructed [`LocProbe`]s.
    fn iter_loc_probe(
        &self,
        start: LocProbe,
        end: LocProbe,
    ) -> impl Iterator<Item = VarnodeId> + '_ {
        self.loc_tree.range((start.into_start(), end.into_end())).map(|(_, &id)| id)
    }

    /// Find a Varnode given its (loc,size) and the address/seqnum where it is
    /// defined (C++ `find`).
    ///
    /// `uniq == None` means "don't care about uniq" (the C++ `~0` sentinel).
    /// `def_addr_time` maps an `OpId` to its `(getAddr, getTime)` (SEAM(W3):
    /// op), used to confirm the candidate's defining op matches `pc`/`uniq`.
    pub fn find(
        &self,
        s: int4,
        loc: &Address,
        pc: &Address,
        uniq: Option<uintm>,
        def_addr_time: &dyn Fn(OpId) -> (Address, uintm),
    ) -> Option<VarnodeId> {
        // iter = beginLoc(s, loc, pc, uniq);  loop while iter != loc_tree.end()
        for id in self.iter_loc_probe(self.begin_loc_pc(s, loc, pc, uniq), LocProbe::End) {
            let vn = &self.arena[id];
            if vn.size != s {
                break;
            }
            if &vn.loc != loc {
                break;
            }
            if let Some(op) = vn.def {
                let (op_addr, op_time) = def_addr_time(op);
                if &op_addr == pc {
                    match uniq {
                        None => return Some(id),
                        Some(u) if op_time == u => return Some(id),
                        _ => {}
                    }
                }
            }
        }
        None
    }

    /// Find an input Varnode of the given size and address (C++ `findInput`).
    pub fn find_input(&self, s: int4, loc: &Address) -> Option<VarnodeId> {
        // iter = beginLoc(s, loc, Varnode::input);  if (iter != end) { ... }
        let begin = self.begin_loc_flag(s, loc, varnode_flags::input);
        if let Some(id) = self.iter_loc_probe(begin, LocProbe::End).next() {
            let vn = &self.arena[id];
            if vn.is_input() && vn.size == s && &vn.loc == loc {
                return Some(id);
            }
        }
        None
    }

    // --- begin/end loc probe construction (the searchvn template family) --
    //
    // Each helper builds the exact LocKey the C++ assigns to `searchvn` and
    // names the C++ operation (lower/upper-bound) as a [`LocProbe`]; the
    // position-dependent Rust `Bound` conversion happens in `iter_loc_probe`
    // (`Probe::into_start`/`into_end`).  This keeps the start-vs-end membership
    // exactly as the C++ half-open `[iter1, iter2)` semantics demand.
    //
    // NOTE on the `input`-class bounds: the C++ `searchvn` is a *size-0 input*
    // template (`searchvn.flags = Varnode::input`).  For the input class the
    // bounds set `searchvn.size = s` but leave the flag class at `input` — so
    // the constructed bound key uses flag class `input` (raw 0x08).  A real
    // input varnode also carries the `insert` bit, but `insert` is outside the
    // `(input|written)` mask the comparator reads, so the key's flag class is
    // exactly `input`.

    /// `beginLoc(int4 s,const Address &addr,uint4 fl)` (`varnode.cc:1664`).
    fn begin_loc_flag(&self, s: int4, addr: &Address, fl: uint4) -> LocProbe {
        if fl == varnode_flags::input {
            // searchvn{size=s, loc=addr, flags=input} ; lower_bound
            return LocProbe::Lower(LocKey {
                addr: addr.clone(),
                size: s,
                flagclass: flag_class_of(varnode_flags::input),
                seqnum: SeqNum::default(),
                create_index: 0,
            });
        }
        if fl == varnode_flags::written {
            // searchvn{size=s, loc=addr, flags=written, def=&searchop(minimal seq)} ; lower_bound
            return LocProbe::Lower(LocKey {
                addr: addr.clone(),
                size: s,
                flagclass: flag_class_of(varnode_flags::written),
                seqnum: SeqNum::new_extreme(mach_extreme::m_minimal),
                create_index: 0,
            });
        }
        // fl == 0 (free): searchvn{size=s, loc=addr, flags=written, def=&searchop(maximal seq)} ; upper_bound
        LocProbe::Upper(LocKey {
            addr: addr.clone(),
            size: s,
            flagclass: flag_class_of(varnode_flags::written),
            seqnum: SeqNum::new_extreme(mach_extreme::m_maximal),
            create_index: 0,
        })
    }

    /// `endLoc(int4 s,const Address &addr,uint4 fl)` (`varnode.cc:1712`).
    fn end_loc_flag(&self, s: int4, addr: &Address, fl: uint4) -> LocProbe {
        if fl == varnode_flags::written {
            // searchvn{loc=addr, size=s, flags=written, def=&searchop(maximal seq)} ; upper_bound
            return LocProbe::Upper(LocKey {
                addr: addr.clone(),
                size: s,
                flagclass: flag_class_of(varnode_flags::written),
                seqnum: SeqNum::new_extreme(mach_extreme::m_maximal),
                create_index: 0,
            });
        }
        if fl == varnode_flags::input {
            // searchvn{loc=addr, size=s, flags=input} ; upper_bound
            return LocProbe::Upper(LocKey {
                addr: addr.clone(),
                size: s,
                flagclass: flag_class_of(varnode_flags::input),
                seqnum: SeqNum::default(),
                create_index: 0,
            });
        }
        // fl == 0 (free): searchvn{loc=addr, size=s+1, flags=input} ; lower_bound
        LocProbe::Lower(LocKey {
            addr: addr.clone(),
            size: s + 1,
            flagclass: flag_class_of(varnode_flags::input),
            seqnum: SeqNum::default(),
            create_index: 0,
        })
    }

    /// `beginLoc(int4 s,const Address &addr,const Address &pc,uintm uniq)`
    /// (`varnode.cc:1751`).
    fn begin_loc_pc(&self, s: int4, addr: &Address, pc: &Address, uniq: Option<uintm>) -> LocProbe {
        // if (uniq==~0) uniq = 0;  // find earliest
        let u = uniq.unwrap_or(0);
        // SeqNum sq(pc, u);  searchvn{size=s, loc=addr, flags=written, def=&searchop(sq)} ; lower_bound
        LocProbe::Lower(LocKey {
            addr: addr.clone(),
            size: s,
            flagclass: flag_class_of(varnode_flags::written),
            seqnum: SeqNum::new(pc.clone(), u),
            create_index: 0,
        })
    }

    /// `endLoc(int4 s,const Address &addr,const Address &pc,uintm uniq)`
    /// (`varnode.cc:1781`).
    fn end_loc_pc(&self, s: int4, addr: &Address, pc: &Address, uniq: uintm) -> LocProbe {
        // (the C++ does NOT remap ~0 here) ; upper_bound
        LocProbe::Upper(LocKey {
            addr: addr.clone(),
            size: s,
            flagclass: flag_class_of(varnode_flags::written),
            seqnum: SeqNum::new(pc.clone(), uniq),
            create_index: 0,
        })
    }

    // --- Public location range queries (the beginLoc/endLoc overloads) -----

    /// Varnode ids of a given size and starting address in location order
    /// (C++ `beginLoc(int4,const Address&)`..`endLoc(int4,const Address&)`).
    pub fn iter_loc_size_addr(
        &self,
        s: int4,
        addr: &Address,
    ) -> impl Iterator<Item = VarnodeId> + '_ {
        // beginLoc: searchvn{size=s, loc=addr} ; lower_bound  (flag = input)
        let begin = LocProbe::Lower(LocKey {
            addr: addr.clone(),
            size: s,
            flagclass: flag_class_of(varnode_flags::input),
            seqnum: SeqNum::default(),
            create_index: 0,
        });
        // endLoc: searchvn{size=s+1, loc=addr} ; lower_bound
        let end = LocProbe::Lower(LocKey {
            addr: addr.clone(),
            size: s + 1,
            flagclass: flag_class_of(varnode_flags::input),
            seqnum: SeqNum::default(),
            create_index: 0,
        });
        self.iter_loc_probe(begin, end)
    }

    /// Varnode ids of a given size, starting address, and property class in
    /// location order (C++ `beginLoc(int4,const Address&,uint4)`..`endLoc(...)`).
    pub fn iter_loc_flag(
        &self,
        s: int4,
        addr: &Address,
        fl: uint4,
    ) -> impl Iterator<Item = VarnodeId> + '_ {
        let begin = self.begin_loc_flag(s, addr, fl);
        let end = self.end_loc_flag(s, addr, fl);
        self.iter_loc_probe(begin, end)
    }

    /// Varnode ids of a given size/address defined at a specific pc/seqnum, in
    /// location order (C++ `beginLoc(int4,Address,Address,uintm)`..`endLoc(...)`).
    pub fn iter_loc_pc(
        &self,
        s: int4,
        addr: &Address,
        pc: &Address,
        uniq: Option<uintm>,
    ) -> impl Iterator<Item = VarnodeId> + '_ {
        let begin = self.begin_loc_pc(s, addr, pc, uniq);
        // endLoc uses the raw uniq (no ~0 remap)
        let end = self.end_loc_pc(s, addr, pc, uniq.unwrap_or(uintm::MAX));
        self.iter_loc_probe(begin, end)
    }

    // --- Definition-tree range queries ------------------------------------

    /// Iterate varnode ids whose definition lies in `[start, end)` of the def
    /// tree, with the bounds given as [`DefProbe`]s.
    fn iter_def_probe(
        &self,
        start: DefProbe,
        end: DefProbe,
    ) -> impl Iterator<Item = VarnodeId> + '_ {
        self.def_tree.range((start.into_start(), end.into_end())).map(|(_, &id)| id)
    }

    /// `beginDef(uint4 fl)` (`varnode.cc:1850`).
    fn begin_def_flag(&self, fl: uint4) -> DefProbe {
        if fl == varnode_flags::input {
            // return def_tree.begin();  -- inputs occur first
            return DefProbe::Begin;
        }
        if fl == varnode_flags::written {
            // searchvn{loc=minimal, flags=written, def=&searchop(minimal seq)} ; lower_bound
            return DefProbe::Lower(DefKey {
                addr: Address::new_extreme(mach_extreme::m_minimal),
                size: 0,
                flagclass: flag_class_of(varnode_flags::written),
                seqnum: SeqNum::new_extreme(mach_extreme::m_minimal),
                create_index: 0,
            });
        }
        // fl == 0 (free): searchvn{loc=maximal, flags=written, def=&searchop(maximal seq)} ; upper_bound
        DefProbe::Upper(DefKey {
            addr: Address::new_extreme(mach_extreme::m_maximal),
            size: 0,
            flagclass: flag_class_of(varnode_flags::written),
            seqnum: SeqNum::new_extreme(mach_extreme::m_maximal),
            create_index: 0,
        })
    }

    /// `endDef(uint4 fl)` (`varnode.cc:1888`).
    fn end_def_flag(&self, fl: uint4) -> DefProbe {
        if fl == varnode_flags::input {
            // searchvn{loc=minimal, flags=written, def=&searchop(minimal seq)} ; lower_bound
            return DefProbe::Lower(DefKey {
                addr: Address::new_extreme(mach_extreme::m_minimal),
                size: 0,
                flagclass: flag_class_of(varnode_flags::written),
                seqnum: SeqNum::new_extreme(mach_extreme::m_minimal),
                create_index: 0,
            });
        }
        if fl == varnode_flags::written {
            // searchvn{loc=maximal, flags=written, def=&searchop(maximal seq)} ; upper_bound
            return DefProbe::Upper(DefKey {
                addr: Address::new_extreme(mach_extreme::m_maximal),
                size: 0,
                flagclass: flag_class_of(varnode_flags::written),
                seqnum: SeqNum::new_extreme(mach_extreme::m_maximal),
                create_index: 0,
            });
        }
        // fl == 0 (free): def_tree.end()
        DefProbe::End
    }

    /// Varnode ids restricted by a definition property class, in definition
    /// order (C++ `beginDef(uint4)`..`endDef(uint4)`).
    pub fn iter_def_flag(&self, fl: uint4) -> impl Iterator<Item = VarnodeId> + '_ {
        let begin = self.begin_def_flag(fl);
        let end = self.end_def_flag(fl);
        self.iter_def_probe(begin, end)
    }

    /// `beginDef(uint4 fl,const Address &addr)` (`varnode.cc:1927`).
    /// Errors for the `written` class (C++ throws "Cannot get contiguous
    /// written AND addressed").
    fn begin_def_flag_addr(&self, fl: uint4, addr: &Address) -> KunaResult<DefProbe> {
        if fl == varnode_flags::written {
            return Err(KunaError::lowlevel("Cannot get contiguous written AND addressed"));
        }
        if fl == varnode_flags::input {
            // searchvn{loc=addr} ; lower_bound  (flags default = input)
            return Ok(DefProbe::Lower(DefKey {
                addr: addr.clone(),
                size: 0,
                flagclass: flag_class_of(varnode_flags::input),
                seqnum: SeqNum::default(),
                create_index: 0,
            }));
        }
        // fl == 0 (free): searchvn{loc=addr, flags=0(free)} ; upper_bound
        Ok(DefProbe::Upper(DefKey {
            addr: addr.clone(),
            size: 0,
            flagclass: flag_class_of(0),
            seqnum: SeqNum::default(),
            create_index: 0,
        }))
    }

    /// `endDef(uint4 fl,const Address &addr)` (`varnode.cc:1961`).
    fn end_def_flag_addr(&self, fl: uint4, addr: &Address) -> KunaResult<DefProbe> {
        if fl == varnode_flags::written {
            return Err(KunaError::lowlevel("Cannot get contiguous written AND addressed"));
        }
        if fl == varnode_flags::input {
            // searchvn{loc=addr, size=1000000} ; lower_bound
            return Ok(DefProbe::Lower(DefKey {
                addr: addr.clone(),
                size: 1000000,
                flagclass: flag_class_of(varnode_flags::input),
                seqnum: SeqNum::default(),
                create_index: 0,
            }));
        }
        // fl == 0 (free): searchvn{loc=addr, size=1000000, flags=0(free)} ; lower_bound
        Ok(DefProbe::Lower(DefKey {
            addr: addr.clone(),
            size: 1000000,
            flagclass: flag_class_of(0),
            seqnum: SeqNum::default(),
            create_index: 0,
        }))
    }

    /// Varnode ids of a given property class starting at a given address, in
    /// definition order (C++ `beginDef(uint4,Address)`..`endDef(uint4,Address)`).
    pub fn iter_def_flag_addr(&self, fl: uint4, addr: &Address) -> KunaResult<Vec<VarnodeId>> {
        let begin = self.begin_def_flag_addr(fl, addr)?;
        let end = self.end_def_flag_addr(fl, addr)?;
        Ok(self.iter_def_probe(begin, end).collect())
    }

    // --- Additional input finders (def-tree based) ------------------------

    /// Find the first input Varnode completely contained within the given range
    /// (C++ `findCoveredInput`).
    pub fn find_covered_input(&self, s: int4, loc: &Address) -> KunaResult<Option<VarnodeId>> {
        let space = loc.get_space().expect("findCoveredInput on invalid address (C++ UB)");
        let highest = space.get_highest();
        // uintb end = loc.getOffset() + s - 1;
        let end = loc.get_offset().wadd(s as i64 as u64).wsub(1);

        let begin = self.begin_def_flag_addr(varnode_flags::input, loc)?;
        // The C++ end iterator is `endDef(input, highest)` or `beginDef(input,
        // loc+s)`; either way it is a probe used in *end* position.
        let end_probe = if end == highest {
            // enditer = endDef(input, Address(space, highest));
            let tmp = Address::new(Rc::clone(space), highest);
            self.end_def_flag_addr(varnode_flags::input, &tmp)?
        } else {
            // enditer = beginDef(input, loc+s);
            let plus = loc + (s as i64);
            self.begin_def_flag_addr(varnode_flags::input, &plus)?
        };

        for id in self.iter_def_probe(begin, end_probe) {
            let vn = &self.arena[id];
            // if (vn->getOffset()+vn->getSize()-1 <= end) return vn;
            if vn.get_offset().wadd(vn.size as i64 as u64).wsub(1) <= end {
                return Ok(Some(id));
            }
        }
        Ok(None)
    }

    /// Find the input Varnode that completely contains the given range
    /// (C++ `findCoveringInput`).
    pub fn find_covering_input(&self, s: int4, loc: &Address) -> KunaResult<Option<VarnodeId>> {
        // iter = beginDef(input, loc);  then possibly step back one
        let begin = self.begin_def_flag_addr(varnode_flags::input, loc)?;
        let cand = self.iter_def_probe(begin.clone(), DefProbe::End).next();
        let vn_id = match cand {
            None => return Ok(None), // iter == def_tree.end()
            Some(id) => {
                let vn = &self.arena[id];
                // if (vn->getAddr() != loc && iter != def_tree.begin()) { --iter; vn = *iter; }
                if &vn.loc != loc {
                    // step back one: the last element strictly before the `begin`
                    // iterator position (so `begin` is used as an *end* bound)
                    match self
                        .def_tree
                        .range((Bound::Unbounded, begin.into_end()))
                        .next_back()
                        .map(|(_, &i)| i)
                    {
                        Some(prev) => prev,
                        None => id, // iter == begin(): keep candidate
                    }
                } else {
                    id
                }
            }
        };
        let vn = &self.arena[vn_id];
        if vn.is_input()
            && Rc::ptr_eq(vn.get_space(), loc.get_space().expect("covering: invalid addr"))
            && vn.get_offset() <= loc.get_offset()
            && vn.get_offset().wadd(vn.size as i64 as u64).wsub(1)
                >= loc.get_offset().wadd(s as i64 as u64).wsub(1)
        {
            return Ok(Some(vn_id));
        }
        Ok(None)
    }

    /// Check for an input Varnode overlapping the given range
    /// (C++ `hasInputIntersection`).
    pub fn has_input_intersection(&self, s: int4, loc: &Address) -> KunaResult<bool> {
        let begin = self.begin_def_flag_addr(varnode_flags::input, loc)?;
        // iter = beginDef(input, loc);  if (iter != end) { vn=*iter; ... }
        if let Some(id) = self.iter_def_probe(begin.clone(), DefProbe::End).next() {
            let vn = &self.arena[id];
            if vn.is_input() && vn.intersects_range(loc, s) {
                return Ok(true);
            }
        }
        // if (iter != def_tree.begin()) { --iter; vn=*iter; ... }
        // (the `begin` probe is used as an *end* bound to step back one)
        if let Some((_, &id)) = self.def_tree.range((Bound::Unbounded, begin.into_end())).next_back() {
            let vn = &self.arena[id];
            if vn.is_input() && vn.intersects_range(loc, s) {
                return Ok(true);
            }
        }
        Ok(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kuna_base::space::{addrspace_flags, AddrSpace, ConstantSpace, UniqueSpace};
    use std::cmp::Ordering;

    fn dt(size: int4) -> Rc<Datatype> {
        Rc::new(Datatype::new(size, type_metatype::TYPE_UNKNOWN))
    }

    /// Build a manager with const(0), unique(1), ram(2), register(3).
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
        m.insert_space(Rc::new(AddrSpace::new(
            spacetype::IPTR_PROCESSOR,
            "register",
            false,
            4,
            1,
            3,
            0,
            0,
            0,
        )))
        .unwrap();
        m.set_default_code_space(2).unwrap();
        m
    }

    fn space(m: &AddrSpaceManager, idx: i32) -> Rc<AddrSpace> {
        Rc::clone(m.get_space(idx).unwrap())
    }

    /// A def-op stand-in carrying its SeqNum (op arena is W3-op's).  Fabricates
    /// a distinct OpId through a throwaway slotmap so each call is unique.
    fn def_info(id_counter: &mut u32, pc: Address, uniq: u32) -> DefOpInfo {
        let mut sm: slotmap::SlotMap<OpId, ()> = slotmap::SlotMap::with_key();
        for _ in 0..*id_counter {
            sm.insert(());
        }
        let id = sm.insert(());
        *id_counter += 1;
        DefOpInfo { id, seqnum: SeqNum::new(pc, uniq) }
    }

    /// A no-op `replace_reads` callback (no equivalent varnode is expected).
    fn no_replace() -> impl FnMut(&mut VarnodeBank, VarnodeId, VarnodeId) -> KunaResult<()> {
        |_: &mut VarnodeBank, _: VarnodeId, _: VarnodeId| -> KunaResult<()> { Ok(()) }
    }

    // -- Reference comparators: a literal re-transcription of varnode.cc:34-79
    //    used to cross-check LocKey/DefKey::cmp exhaustively. `seq_a != seq_b`
    //    and `seq_a < seq_b` use SeqNum's own (uniq-only Eq, (pc,uniq) Ord),
    //    matching the C++ exactly. ---------------------------------------

    #[allow(clippy::too_many_arguments)]
    fn ref_locdef(
        addr_a: &Address,
        size_a: int4,
        f_a: uint4,
        seq_a: &SeqNum,
        ci_a: uint4,
        addr_b: &Address,
        size_b: int4,
        f_b: uint4,
        seq_b: &SeqNum,
        ci_b: uint4,
    ) -> bool {
        if addr_a != addr_b {
            return addr_a < addr_b;
        }
        if size_a != size_b {
            return size_a < size_b;
        }
        let f1 = f_a & (varnode_flags::input | varnode_flags::written);
        let f2 = f_b & (varnode_flags::input | varnode_flags::written);
        if f1 != f2 {
            return f1.wrapping_sub(1) < f2.wrapping_sub(1);
        }
        if f1 == varnode_flags::written {
            if seq_a != seq_b {
                return seq_a < seq_b;
            }
        } else if f1 == 0 {
            return ci_a < ci_b;
        }
        false
    }

    #[allow(clippy::too_many_arguments)]
    fn ref_defloc(
        addr_a: &Address,
        size_a: int4,
        f_a: uint4,
        seq_a: &SeqNum,
        ci_a: uint4,
        addr_b: &Address,
        size_b: int4,
        f_b: uint4,
        seq_b: &SeqNum,
        ci_b: uint4,
    ) -> bool {
        let f1 = f_a & (varnode_flags::input | varnode_flags::written);
        let f2 = f_b & (varnode_flags::input | varnode_flags::written);
        if f1 != f2 {
            return f1.wrapping_sub(1) < f2.wrapping_sub(1);
        }
        if f1 == varnode_flags::written && seq_a != seq_b {
            return seq_a < seq_b;
        }
        if addr_a != addr_b {
            return addr_a < addr_b;
        }
        if size_a != size_b {
            return size_a < size_b;
        }
        if f1 == 0 {
            return ci_a < ci_b;
        }
        false
    }

    fn make_loc_key(addr: Address, size: int4, fl: uint4, seq: SeqNum, ci: uint4) -> LocKey {
        LocKey { addr, size, flagclass: flag_class_of(fl), seqnum: seq, create_index: ci }
    }
    fn make_def_key(addr: Address, size: int4, fl: uint4, seq: SeqNum, ci: uint4) -> DefKey {
        DefKey { addr, size, flagclass: flag_class_of(fl), seqnum: seq, create_index: ci }
    }

    /// GOLDEN (varnodesort): the `tests/golden/vectors/` corpus has no varnode
    /// section (only addrsort/opbehavior/float/lift — verified against the
    /// goldengen emitter), so per the W3 item brief we pin the comparators with
    /// an exhaustive synthetic cross-product over every flag class, size,
    /// address, seqnum, and create_index, cross-checked against a literal
    /// re-transcription of `varnode.cc:34-79` (incl. the `(f1-1)<(f2-1)` wrap
    /// and the create_index tie-break).
    #[test]
    fn golden_varnodesort_comparators_exhaustive() {
        let m = build_manager();
        let addrs: Vec<Address> = vec![
            Address::new(space(&m, 0), 0), // const:0
            Address::new(space(&m, 2), 0), // ram:0
            Address::new(space(&m, 2), 0x1000),
            Address::new(space(&m, 3), 0x10), // register:0x10
        ];
        let sizes: [int4; 3] = [1, 4, 8];
        let classes: [uint4; 3] = [0, varnode_flags::input, varnode_flags::written];
        let seqs: Vec<SeqNum> = vec![
            SeqNum::new(Address::new(space(&m, 2), 0x100), 0),
            SeqNum::new(Address::new(space(&m, 2), 0x100), 5),
            SeqNum::new(Address::new(space(&m, 2), 0x200), 0),
        ];
        let cis: [uint4; 3] = [0, 1, 7];

        struct Tup {
            addr: Address,
            size: int4,
            fl: uint4,
            seq: SeqNum,
            ci: uint4,
        }
        let mut tuples: Vec<Tup> = Vec::new();
        for a in &addrs {
            for &sz in &sizes {
                for &fl in &classes {
                    for sq in &seqs {
                        for &ci in &cis {
                            tuples.push(Tup {
                                addr: a.clone(),
                                size: sz,
                                fl,
                                seq: sq.clone(),
                                ci,
                            });
                        }
                    }
                }
            }
        }

        let mut pairs = 0usize;
        for ta in &tuples {
            for tb in &tuples {
                let ka = make_loc_key(ta.addr.clone(), ta.size, ta.fl, ta.seq.clone(), ta.ci);
                let kb = make_loc_key(tb.addr.clone(), tb.size, tb.fl, tb.seq.clone(), tb.ci);
                let got_less = ka.cmp(&kb) == Ordering::Less;
                let want_less = ref_locdef(
                    &ta.addr, ta.size, ta.fl, &ta.seq, ta.ci, &tb.addr, tb.size, tb.fl, &tb.seq,
                    tb.ci,
                );
                assert_eq!(got_less, want_less, "LocKey<: a={ka:?} b={kb:?}");

                let da = make_def_key(ta.addr.clone(), ta.size, ta.fl, ta.seq.clone(), ta.ci);
                let db = make_def_key(tb.addr.clone(), tb.size, tb.fl, tb.seq.clone(), tb.ci);
                let got_dless = da.cmp(&db) == Ordering::Less;
                let want_dless = ref_defloc(
                    &ta.addr, ta.size, ta.fl, &ta.seq, ta.ci, &tb.addr, tb.size, tb.fl, &tb.seq,
                    tb.ci,
                );
                assert_eq!(got_dless, want_dless, "DefKey<: a={da:?} b={db:?}");
                pairs += 1;
            }
        }
        assert!(pairs > 100_000, "expected a large exhaustive product, got {pairs}");
    }

    /// The `(f1-1)<(f2-1)` free-class wrap: input < written < free.
    #[test]
    fn flag_class_order_input_written_free() {
        let m = build_manager();
        let addr = Address::new(space(&m, 2), 0x40);
        let seq = SeqNum::new(Address::new(space(&m, 2), 0x10), 1);
        let input = make_loc_key(addr.clone(), 4, varnode_flags::input, seq.clone(), 0);
        let written = make_loc_key(addr.clone(), 4, varnode_flags::written, seq.clone(), 0);
        let free = make_loc_key(addr.clone(), 4, 0, seq.clone(), 0);
        assert_eq!(input.cmp(&written), Ordering::Less);
        assert_eq!(written.cmp(&free), Ordering::Less);
        assert_eq!(input.cmp(&free), Ordering::Less);
        assert_eq!(
            flag_class_of(0).cmp(&flag_class_of(varnode_flags::written)),
            Ordering::Greater
        );
    }

    /// Construction seeds flags/nzm from space type (varnode.cc:597-626).
    #[test]
    fn construction_flags_by_space_type() {
        let m = build_manager();
        let c = Varnode::new(8, m.get_constant(0x1234), dt(8));
        assert!(c.is_constant());
        assert_eq!(c.get_nz_mask(), 0x1234);
        assert_eq!(c.get_flags(), varnode_flags::constant);
        let r = Varnode::new(4, Address::new(space(&m, 2), 0x80), dt(4));
        assert!(r.is_free());
        assert_eq!(r.get_flags(), varnode_flags::coverdirty);
        assert_eq!(r.get_nz_mask(), u64::MAX);
        assert_eq!(r.get_consume(), u64::MAX);
    }

    /// create_index is a strict one-up counter shared by all create paths.
    #[test]
    fn create_index_monotonic() {
        let m = build_manager();
        let mut bank = VarnodeBank::new(&m, 0x1000).unwrap();
        let a = bank.create(4, Address::new(space(&m, 2), 0x10), dt(4));
        let b = bank.create(4, Address::new(space(&m, 2), 0x20), dt(4));
        let c = bank.create_unique(8, dt(8));
        assert_eq!(bank.get(a).unwrap().get_create_index(), 0);
        assert_eq!(bank.get(b).unwrap().get_create_index(), 1);
        assert_eq!(bank.get(c).unwrap().get_create_index(), 2);
        assert_eq!(bank.get_create_index(), 3);
        assert_eq!(bank.get(c).unwrap().get_offset(), 0x1000);
        let d = bank.create_unique(4, dt(4));
        assert_eq!(bank.get(d).unwrap().get_offset(), 0x1000 + 8);
    }

    /// Two free varnodes at identical (addr,size) are distinct (multiset via
    /// create_index tie-break); iteration is create_index ascending.
    #[test]
    fn loc_tree_free_multiset_and_iteration() {
        let m = build_manager();
        let mut bank = VarnodeBank::new(&m, 0).unwrap();
        let addr = Address::new(space(&m, 2), 0x100);
        let f1 = bank.create(4, addr.clone(), dt(4));
        let f2 = bank.create(4, addr.clone(), dt(4));
        assert_eq!(bank.num_varnodes(), 2);
        let order: Vec<VarnodeId> = bank.iter_loc().collect();
        assert_eq!(order, vec![f1, f2]);
    }

    /// Iteration order across input/written/free at the same address.
    #[test]
    fn iteration_across_classes() {
        let m = build_manager();
        let mut bank = VarnodeBank::new(&m, 0).unwrap();
        let addr = Address::new(space(&m, 2), 0x200);

        let free = bank.create(4, addr.clone(), dt(4));
        let wfree = bank.create(4, addr.clone(), dt(4));
        let mut idc = 0u32;
        let dinfo = def_info(&mut idc, Address::new(space(&m, 2), 0x10), 3);
        let written = bank.set_def(wfree, dinfo, &mut no_replace()).unwrap();
        let ifree = bank.create(4, addr.clone(), dt(4));
        let input = bank.set_input(ifree, &mut no_replace()).unwrap();

        let order: Vec<VarnodeId> = bank.iter_loc().collect();
        assert_eq!(order, vec![input, written, free], "input < written < free");

        let inputs: Vec<VarnodeId> = bank.iter_loc_flag(4, &addr, varnode_flags::input).collect();
        assert_eq!(inputs, vec![input]);
        let writtens: Vec<VarnodeId> =
            bank.iter_loc_flag(4, &addr, varnode_flags::written).collect();
        assert_eq!(writtens, vec![written]);
        let frees: Vec<VarnodeId> = bank.iter_loc_flag(4, &addr, 0).collect();
        assert_eq!(frees, vec![free]);

        // size+address overload spans all three classes
        let all: Vec<VarnodeId> = bank.iter_loc_size_addr(4, &addr).collect();
        assert_eq!(all, vec![input, written, free]);
    }

    /// Definition-order iteration: inputs first, then written (by seqnum).
    #[test]
    fn def_order_inputs_then_written() {
        let m = build_manager();
        let mut bank = VarnodeBank::new(&m, 0).unwrap();
        // input at a high address
        let in_free = bank.create(4, Address::new(space(&m, 2), 0x5000), dt(4));
        let input = bank.set_input(in_free, &mut no_replace()).unwrap();
        // written at a low address
        let w_free = bank.create(4, Address::new(space(&m, 2), 0x10), dt(4));
        let mut idc = 0u32;
        let dinfo = def_info(&mut idc, Address::new(space(&m, 2), 0x8), 1);
        let written = bank.set_def(w_free, dinfo, &mut no_replace()).unwrap();

        let inputs: Vec<VarnodeId> = bank.iter_def_flag(varnode_flags::input).collect();
        assert_eq!(inputs, vec![input]);
        let writtens: Vec<VarnodeId> = bank.iter_def_flag(varnode_flags::written).collect();
        assert_eq!(writtens, vec![written]);
        let all: Vec<VarnodeId> = bank.iter_def().collect();
        assert_eq!(all, vec![input, written]);
    }

    /// find_input / find_covering_input / find_covered_input on inputs.
    ///
    /// The def-tree finders match the C++'s `lower_bound`-then-step-back idiom,
    /// which relies on there being a def-tree element *after* the queried range
    /// (in real functions the written/free varnodes that follow the inputs);
    /// the test mirrors that by registering a higher-address input so the
    /// covering/intersection probes land on a real element and step back.
    #[test]
    fn input_finders() {
        let m = build_manager();
        let mut bank = VarnodeBank::new(&m, 0).unwrap();
        let big_addr = Address::new(space(&m, 2), 0x300);
        let big_free = bank.create(8, big_addr.clone(), dt(8)); // covers [0x300,0x307]
        let big = bank.set_input(big_free, &mut no_replace()).unwrap();
        // a higher-address input so the covering/intersection lower_bounds land
        // on a real element (and step back), as they would in a real function
        let hi_addr = Address::new(space(&m, 2), 0x400);
        let hi_free = bank.create(2, hi_addr.clone(), dt(2)); // covers [0x400,0x401]
        let hi = bank.set_input(hi_free, &mut no_replace()).unwrap();

        assert_eq!(bank.find_input(8, &big_addr), Some(big));
        assert_eq!(bank.find_input(4, &big_addr), None);
        assert_eq!(bank.find_input(2, &hi_addr), Some(hi));

        // covering: [0x302,0x303] is inside big [0x300,0x307]
        let inner = Address::new(space(&m, 2), 0x302);
        assert_eq!(bank.find_covering_input(2, &inner).unwrap(), Some(big));
        // an exact-start covering query (vn->getAddr() == loc, no step back)
        assert_eq!(bank.find_covering_input(2, &hi_addr).unwrap(), Some(hi));

        // covered: a small input fully inside a larger query range
        let small_addr = Address::new(space(&m, 2), 0x401);
        let small_free = bank.create(2, small_addr.clone(), dt(2));
        let small = bank.set_input(small_free, &mut no_replace()).unwrap();
        let q = Address::new(space(&m, 2), 0x401);
        assert_eq!(bank.find_covered_input(2, &q).unwrap(), Some(small));

        // intersection
        assert!(bank.has_input_intersection(2, &inner).unwrap());
        let far = Address::new(space(&m, 2), 0x9000);
        assert!(!bank.has_input_intersection(2, &far).unwrap());
    }

    /// find() locates a written varnode by (size, loc, pc, uniq).
    #[test]
    fn find_written_by_def() {
        let m = build_manager();
        let mut bank = VarnodeBank::new(&m, 0).unwrap();
        let addr = Address::new(space(&m, 2), 0x500);
        let pc = Address::new(space(&m, 2), 0x20);
        let free = bank.create(4, addr.clone(), dt(4));
        let mut idc = 0u32;
        let dinfo = def_info(&mut idc, pc.clone(), 9);
        let op_id = dinfo.id;
        let written = bank.set_def(free, dinfo, &mut no_replace()).unwrap();

        let pc_for = pc.clone();
        let accessor = move |op: OpId| -> (Address, uintm) {
            assert_eq!(op, op_id);
            (pc_for.clone(), 9)
        };
        assert_eq!(bank.find(4, &addr, &pc, Some(9), &accessor), Some(written));
        assert_eq!(bank.find(4, &addr, &pc, Some(10), &accessor), None);
        assert_eq!(bank.find(4, &addr, &pc, None, &accessor), Some(written));
    }

    /// xref unification: re-inserting an equivalent input returns the existing
    /// varnode and runs the replace callback once with (old, new).
    #[test]
    fn xref_unifies_equivalent_input() {
        let m = build_manager();
        let mut bank = VarnodeBank::new(&m, 0).unwrap();
        let addr = Address::new(space(&m, 2), 0x600);
        let f1 = bank.create(4, addr.clone(), dt(4));
        let in1 = bank.set_input(f1, &mut no_replace()).unwrap();
        assert_eq!(in1, f1);

        let f2 = bank.create(4, addr.clone(), dt(4));
        let mut replaced: Vec<(VarnodeId, VarnodeId)> = Vec::new();
        let mut record = |_: &mut VarnodeBank, old: VarnodeId, new: VarnodeId| -> KunaResult<()> {
            replaced.push((old, new));
            Ok(())
        };
        let in2 = bank.set_input(f2, &mut record).unwrap();
        assert_eq!(in2, in1, "input unifies onto the existing input varnode");
        assert_eq!(replaced, vec![(f2, in1)]);
        assert!(bank.get(f2).is_none());
        assert_eq!(bank.num_varnodes(), 1);
    }

    /// makeFree round-trips a written varnode back to free and re-inserts it.
    #[test]
    fn make_free_roundtrip() {
        let m = build_manager();
        let mut bank = VarnodeBank::new(&m, 0).unwrap();
        let addr = Address::new(space(&m, 2), 0x700);
        let free = bank.create(4, addr.clone(), dt(4));
        let mut idc = 0u32;
        let dinfo = def_info(&mut idc, Address::new(space(&m, 2), 0x30), 1);
        let w = bank.set_def(free, dinfo, &mut no_replace()).unwrap();
        assert!(bank.get(w).unwrap().is_written());
        bank.make_free(w);
        let vn = bank.get(w).unwrap();
        assert!(vn.is_free());
        assert!(vn.get_def().is_none());
        let frees: Vec<VarnodeId> = bank.iter_loc_flag(4, &addr, 0).collect();
        assert_eq!(frees, vec![w]);
    }

    /// destroy removes a free varnode; rejects an integrated one.
    #[test]
    fn destroy_semantics() {
        let m = build_manager();
        let mut bank = VarnodeBank::new(&m, 0).unwrap();
        let addr = Address::new(space(&m, 2), 0x800);
        let free = bank.create(4, addr.clone(), dt(4));
        assert!(bank.destroy(free).is_ok());
        assert_eq!(bank.num_varnodes(), 0);

        let free2 = bank.create(4, addr.clone(), dt(4));
        let mut idc = 0u32;
        let dinfo = def_info(&mut idc, Address::new(space(&m, 2), 0x40), 2);
        let w = bank.set_def(free2, dinfo, &mut no_replace()).unwrap();
        assert!(bank.destroy(w).is_err());
    }

    /// descend list preserves push_back order and erase-by-value (C++ list).
    #[test]
    fn descend_order_and_erase() {
        let m = build_manager();
        let mut bank = VarnodeBank::new(&m, 0).unwrap();
        let v = bank.create(4, Address::new(space(&m, 2), 0x900), dt(4));
        let mut sm: slotmap::SlotMap<OpId, ()> = slotmap::SlotMap::with_key();
        let o0 = sm.insert(());
        let o1 = sm.insert(());
        let o2 = sm.insert(());
        // spacebase so add_descend allows multiple on a free varnode
        bank.get_mut(v).unwrap().flags |= varnode_flags::spacebase;
        bank.add_descend(v, o0).unwrap();
        bank.add_descend(v, o1).unwrap();
        bank.add_descend(v, o2).unwrap();
        let got: Vec<OpId> = bank.get(v).unwrap().descend_iter().collect();
        assert_eq!(got, vec![o0, o1, o2]);
        bank.erase_descend(v, o1);
        let got: Vec<OpId> = bank.get(v).unwrap().descend_iter().collect();
        assert_eq!(got, vec![o0, o2]);
    }

    /// A free (non-spacebase) varnode rejects a second descendant.
    #[test]
    fn free_varnode_single_descendant() {
        let m = build_manager();
        let mut bank = VarnodeBank::new(&m, 0).unwrap();
        let v = bank.create(4, Address::new(space(&m, 2), 0xA00), dt(4));
        let mut sm: slotmap::SlotMap<OpId, ()> = slotmap::SlotMap::with_key();
        let o0 = sm.insert(());
        let o1 = sm.insert(());
        assert!(bank.add_descend(v, o0).is_ok());
        assert!(bank.add_descend(v, o1).is_err());
    }

    /// Geometry: contains / intersects / characterizeOverlap.
    #[test]
    fn geometry_methods() {
        let m = build_manager();
        let a = Varnode::new(4, Address::new(space(&m, 2), 0x10), dt(4)); // [0x10,0x13]
        let inside = Varnode::new(2, Address::new(space(&m, 2), 0x11), dt(2)); // [0x11,0x12]
        let before = Varnode::new(2, Address::new(space(&m, 2), 0x08), dt(2));
        let after = Varnode::new(2, Address::new(space(&m, 2), 0x20), dt(2));
        assert_eq!(a.contains(&inside), 0);
        assert_eq!(a.contains(&before), -1);
        assert_eq!(a.contains(&after), 2);
        assert!(a.intersects(&inside));
        assert!(!a.intersects(&after));
        assert_eq!(a.characterize_overlap(&inside), 1);
        let same = Varnode::new(4, Address::new(space(&m, 2), 0x10), dt(4));
        assert_eq!(a.characterize_overlap(&same), 2);
        assert_eq!(a.characterize_overlap(&after), 0);
        let other = Varnode::new(4, Address::new(space(&m, 3), 0x10), dt(4));
        assert_eq!(a.contains(&other), 3);
        assert!(!a.intersects(&other));
        assert_eq!(a.characterize_overlap(&other), 0);
    }
}
