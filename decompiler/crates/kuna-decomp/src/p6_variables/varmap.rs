//! Stack-frame variable mapping and scope-layout recovery (C++ `varmap.{cc,hh}`).
//!
//! This module ports the **stack-layout recovery** machinery: the data-type
//! hints (`RangeHint`), the light-weight pointer/alias analyzer
//! (`AliasChecker`), the hint collector/sorter (`MapState`), and the
//! function-local symbol scope (`ScopeLocal`).  Its job is to take the raw
//! Varnodes that live in a function's stack address space and reconstruct a
//! disjoint cover of named local variables and parameters.
//!
//! ## What is order-determining (transcribed exactly)
//!
//! The recovered layout is a function of two orderings the C++ pins precisely,
//! and they are transcribed field-for-field here:
//!
//!   - **`RangeHint::compare`** (`varmap.cc:321-335`) — the `stable_sort`
//!     comparator (`MapState::initialize`) that fixes the order in which hints
//!     are merged.  It compares signed start, then size, then range-type, then
//!     flags, then high-index — *not* the data-type.  [`RangeHint::compare`] /
//!     [`RangeHint::compare_ranges`].
//!   - **`MapState::reconcileDatatypes`** (`varmap.cc:960-996`) — the
//!     same-`(start,size,flags)` run collapse that selects the most specific
//!     data-type (`typeOrder`) and de-duplicates, walking the sorted list.
//!
//! The merge core (`merge`/`reconcile`/`contain`/`preferred`/`absorb`/
//! `attemptJoin`/`isConstAbsorbable`) is the rest of the output-determining
//! logic and is transcribed in full, including the mixed-sign `int8` modulus
//! in `reconcile` and the `resType` 0/1/2 dispatch in `merge`.
//!
//! ## Cross-wave boundaries
//!
//! `ScopeLocal` is a `ScopeInternal` subclass in C++ (`varmap.hh:212`) and is
//! wired tightly to `Funcdata` (the Varnode location/def sets, the spacebase
//! input, the load guards, warning emission, varnode remapping) and to the
//! symbol `Database` (category clearing, `addSymbol`, `findOverlap`,
//! `makeNameUnique`, attribute setting).  At this wave boundary the
//! `Funcdata`/`Database` surfaces those methods need are largely still stubs
//! (the descend list, `findSpacebaseInput`, `beginLoc`/`beginDef`,
//! `getLoadGuards`, `warningHeader`, `remapVarnode`, `clearUnlockedCategory`,
//! `setCategory`, the per-scope `EntryMap` iteration).  The self-contained core
//! is ported and unit-tested here; the `Funcdata`/`Database`-bound methods are
//! transcribed against documented [`ScopeLocalAccess`] hooks and otherwise
//! recorded as a loss (see `docs/rust-port/losses.md`, w7-s6-varmap).
//!
//! Per ADR 0002 all collections are ordered; per ADR 0003 every legitimate
//! wrap is an explicit `w*` call (`intb`/`uintb` arithmetic in the offset and
//! merge math).  Per ADR 0004 the one error the C++ throws here
//! (`"Overlapping forced variable types"`) becomes a `KunaError`.

use std::rc::Rc;

use kuna_base::address::{sign_extend, Address};
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::space::AddrSpace;
use kuna_base::types::{int4, int8, intb, uint4, uint8, uintb, Wrap};

use crate::dtype::{type_metatype, Datatype, TypeFactory};

// ===========================================================================
// RangeType + RangeHint flag constants (varmap.hh:95-104)
// ===========================================================================

/// The basic categorization of a [`RangeHint`] (C++ `RangeHint::RangeType`,
/// `varmap.hh:95-99`).
///
/// The integer discriminants are load-bearing: `RangeHint::compare`
/// (`varmap.cc:328-329`) orders by `(rangeType < op2.rangeType)`, so the
/// `fixed=0 < open=1 < endpoint=2` ordering is part of the output-determining
/// sort.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RangeType {
    /// A data-type with a fixed size (C++ `fixed = 0`).
    Fixed = 0,
    /// An array with a (possibly unknown) number of elements (C++ `open = 1`).
    Open = 1,
    /// An (artificial) boundary to the range of bytes getting analyzed
    /// (C++ `endpoint = 2`).
    Endpoint = 2,
}

/// Boolean property: the data-type for the range is locked
/// (C++ `RangeHint::typelock = 1`, `varmap.hh:102`).
pub const TYPELOCK: uint4 = 1;
/// Boolean property: only a constant is COPYed into the range
/// (C++ `RangeHint::copy_constant = 2`, `varmap.hh:103`).
pub const COPY_CONSTANT: uint4 = 2;

// ===========================================================================
// RangeHint (varmap.hh:90-127, varmap.cc:30-335)
// ===========================================================================

/// Partial data-type information mapped to a specific range of bytes
/// (C++ `RangeHint`, `varmap.hh:90-127`).
///
/// Gives a hint about the data-type for a sequence of bytes starting at a
/// specific address offset (typically on the stack): where the data-type
/// starts, what data-type it might be, and how far it extends from the start
/// (possibly as an array).
#[derive(Clone)]
pub struct RangeHint {
    /// Starting offset of this range of bytes (C++ `start`).
    pub start: uintb,
    /// Number of bytes in a single element of this range (C++ `size`).
    pub size: int4,
    /// A signed version of the starting offset (C++ `sstart`).
    pub sstart: intb,
    /// Putative data-type for a single element of this range (C++ `type`).
    pub type_: Rc<Datatype>,
    /// Additional boolean properties of this range (C++ `flags`).
    pub flags: uint4,
    /// The type of range (C++ `rangeType`).
    pub range_type: RangeType,
    /// Minimum upper bound on the array index, if `open` (C++ `highind`).
    pub highind: int4,
}

impl RangeHint {
    /// C++ initialized constructor `RangeHint(st,sz,sst,ct,fl,rt,hi)`
    /// (`varmap.hh:115-116`).
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        st: uintb,
        sz: int4,
        sst: intb,
        ct: Rc<Datatype>,
        fl: uint4,
        rt: RangeType,
        hi: int4,
    ) -> RangeHint {
        RangeHint { start: st, size: sz, sstart: sst, type_: ct, flags: fl, range_type: rt, highind: hi }
    }

    /// Is the data-type for this range locked (C++ `isTypeLock`,
    /// `varmap.hh:117`).
    pub fn is_type_lock(&self) -> bool {
        (self.flags & TYPELOCK) != 0
    }

    /// Can another range be absorbed into this as a constant (C++
    /// `isConstAbsorbable`, `varmap.cc:30-53`).
    ///
    /// `this` is assumed to be \e open.  If `this` is a primitive integer or
    /// float and `b` is just a constant being COPYed, return `true`, even if
    /// the constant is bigger.
    pub fn is_const_absorbable(&self, b: &RangeHint) -> bool {
        if (b.flags & COPY_CONSTANT) == 0 {
            return false;
        }
        if b.is_type_lock() {
            return false;
        }
        if b.size < self.size {
            return false;
        }
        let meta = self.type_.get_metatype();
        if meta != type_metatype::TYPE_INT
            && meta != type_metatype::TYPE_UINT
            && meta != type_metatype::TYPE_BOOL
            && meta != type_metatype::TYPE_FLOAT
        {
            return false;
        }
        let b_meta = b.type_.get_metatype();
        if b_meta != type_metatype::TYPE_UNKNOWN
            && b_meta != type_metatype::TYPE_INT
            && b_meta != type_metatype::TYPE_UINT
        {
            return false;
        }
        let mut end: intb = self.sstart;
        if self.highind > 0 {
            // C++: end += highind * type->getAlignSize();
            end = end.wadd((self.highind as intb).wmul(self.type_.get_align_size() as intb));
        } else {
            end = end.wadd(self.size as intb);
        }
        if b.sstart > end {
            return false;
        }
        true
    }

    /// Can the given intersecting RangeHint coexist with `this` at their given
    /// offsets (C++ `reconcile`, `varmap.cc:62-101`).
    ///
    /// Determine if the data-type information in the two ranges \e line \e up
    /// properly, in which case the union of the two ranges can exist without
    /// destroying data-type information.
    pub fn reconcile(&self, b: &RangeHint) -> KunaResult<bool> {
        // C++ swaps `a`/`b` so that `b` is the smaller-aligned of the two.
        let (a, b): (&RangeHint, &RangeHint) =
            if self.type_.get_align_size() < b.type_.get_align_size() { (b, self) } else { (self, b) };

        // int8 mod = (b->sstart - a->sstart) % a->type->getAlignSize();
        let align_a = a.type_.get_align_size() as int8;
        let mut mod_: int8 = (b.sstart.wsub(a.sstart)) % align_a;
        if mod_ < 0 {
            mod_ = mod_.wadd(align_a);
        }

        // Walk down `a`'s subtypes until the align-size no longer exceeds `b`'s.
        let mut sub: Option<Rc<Datatype>> = Some(Rc::clone(&a.type_));
        let b_align = b.type_.get_align_size();
        while let Some(cur) = sub.clone() {
            if cur.get_align_size() <= b_align {
                break;
            }
            let (next, newmod) = cur.get_sub_type(mod_)?;
            sub = next;
            mod_ = newmod;
        }

        if let Some(sub) = sub.as_ref() {
            if sub.get_align_size() == b.type_.get_align_size() {
                return Ok(true);
            }
            // If we reach here, b overlaps multiple components of a
        }

        // Component sizes do not match. Check for data-types we want to protect more.
        if b.range_type == RangeType::Open && b.is_const_absorbable(a) {
            return Ok(true);
        }
        if b.is_type_lock() {
            return Ok(false);
        }
        let meta = a.type_.get_metatype();
        if meta != type_metatype::TYPE_STRUCT && meta != type_metatype::TYPE_UNION {
            let array_base_unknown = meta == type_metatype::TYPE_ARRAY
                && a
                    .type_
                    .get_array_base()
                    .map(|bt| bt.get_metatype() == type_metatype::TYPE_UNKNOWN)
                    .unwrap_or(false);
            if !array_base_unknown {
                return Ok(false);
            }
        }
        // For structures, unions, and arrays, test if b looks like a partial/combined data-type
        let meta = b.type_.get_metatype();
        if meta == type_metatype::TYPE_UNKNOWN
            || meta == type_metatype::TYPE_INT
            || meta == type_metatype::TYPE_UINT
        {
            return Ok(true);
        }
        Ok(false)
    }

    /// Return `true` if `this` or the given range contains the other (C++
    /// `contain`, `varmap.cc:109-117`).
    ///
    /// Assumes `this` range starts at least as early as the given range and
    /// that the two ranges intersect.
    pub fn contain(&self, b: &RangeHint) -> bool {
        if self.sstart == b.sstart {
            return true;
        }
        let b_end = b.sstart.wadd(b.size as intb).wsub(1);
        let a_end = self.sstart.wadd(self.size as intb).wsub(1);
        if b_end <= a_end {
            return true;
        }
        false
    }

    /// Return `true` if `this` range's data-type is preferred over the other
    /// given range (C++ `preferred`, `varmap.cc:126-157`).
    ///
    /// A locked data-type is preferred over unlocked. A \e fixed size over
    /// \e open size. Otherwise data-type ordering is used.
    pub fn preferred(&self, b: &RangeHint, reconcile: bool) -> KunaResult<bool> {
        if self.start != b.start {
            return Ok(true); // Something must occupy a->start to b->start
        }
        // Prefer the locked type
        if b.is_type_lock() {
            if !self.is_type_lock() {
                return Ok(false);
            }
        } else if self.is_type_lock() {
            return Ok(true);
        }

        if self.range_type == RangeType::Open && b.range_type != RangeType::Open {
            if !reconcile {
                return Ok(false); // Throw out open range
            }
            if self.is_const_absorbable(b) {
                return Ok(true);
            }
        } else if b.range_type == RangeType::Open && self.range_type != RangeType::Open {
            if !reconcile {
                return Ok(true); // Throw out open range
            }
            if b.is_const_absorbable(self) {
                return Ok(false);
            }
        } else if self.range_type == RangeType::Fixed
            && b.range_type == RangeType::Fixed
            && self.size != b.size
            && !reconcile
        {
            // C++ nested `if (size != b->size && !reconcile)` inside the
            // both-fixed branch; collapsed (the branch has no other body).
            return Ok(self.size > b.size);
        }

        // Prefer the more specific.
        Ok(0 > self.type_.type_order(&b.type_)?)
    }

    /// Try to concatenate another RangeHint onto `this` (C++ `attemptJoin`,
    /// `varmap.cc:170-211`).
    ///
    /// If `this` is an array and the following RangeHint lines up, adjust
    /// `this` so it \e absorbs the other and return `true`.
    pub fn attempt_join(&mut self, b: &RangeHint) -> bool {
        if self.range_type != RangeType::Open {
            return false;
        }
        if b.range_type == RangeType::Endpoint {
            return false; // Don't merge with bounding range
        }
        if self.is_const_absorbable(b) {
            self.absorb(b);
            return true;
        }
        if self.highind < 0 {
            return false;
        }
        let mut settype = Rc::clone(&self.type_); // Assume we will keep this data-type
        if settype.get_align_size() != b.type_.get_align_size() {
            return false;
        }
        if !Rc::ptr_eq(&settype, &b.type_) {
            let mut a_test_type = Rc::clone(&self.type_);
            let mut b_test_type = Rc::clone(&b.type_);
            while a_test_type.get_metatype() == type_metatype::TYPE_PTR {
                if b_test_type.get_metatype() != type_metatype::TYPE_PTR {
                    break;
                }
                let an = match a_test_type.get_ptr_to() {
                    Some(t) => t,
                    None => break,
                };
                let bn = match b_test_type.get_ptr_to() {
                    Some(t) => t,
                    None => break,
                };
                a_test_type = an;
                b_test_type = bn;
            }
            let am = a_test_type.get_metatype();
            let bm = b_test_type.get_metatype();
            // C++ branch ladder (varmap.cc:191-200):
            //   if aTestType==UNKNOWN          settype = b->type;
            //   else if bTestType==UNKNOWN     { }   // keep settype
            //   else if INT/UINT or UINT/INT   { }   // keep settype
            //   else if aTestType != bTestType return false;
            // The middle three branches are all "keep settype"; collapsed here
            // (the empty C++ blocks would trip clippy's `if_same_then_else`).
            if am == type_metatype::TYPE_UNKNOWN {
                settype = Rc::clone(&b.type_);
            } else {
                let keep = bm == type_metatype::TYPE_UNKNOWN
                    || (am == type_metatype::TYPE_INT && bm == type_metatype::TYPE_UINT)
                    || (am == type_metatype::TYPE_UINT && bm == type_metatype::TYPE_INT);
                if !keep && !Rc::ptr_eq(&a_test_type, &b_test_type) {
                    // If they are both not unknown, they must be the same
                    return false;
                }
            }
        }
        if self.is_type_lock() {
            return false;
        }
        if b.is_type_lock() {
            return false;
        }
        let mut diffsz: intb = b.sstart.wsub(self.sstart);
        if (diffsz % (settype.get_align_size() as intb)) != 0 {
            return false;
        }
        diffsz /= settype.get_align_size() as intb;
        if diffsz > self.highind as intb {
            return false;
        }
        self.type_ = settype;
        self.absorb(b);
        true
    }

    /// Absorb details of the other RangeHint into `this`, except for the
    /// data-type (C++ `absorb`, `varmap.cc:217-248`).
    ///
    /// Inherit an \e open range type and any indexing information. The
    /// data-type for `this` is assumed compatible and preferred and is not
    /// changed.
    pub fn absorb(&mut self, b: &RangeHint) {
        if b.range_type == RangeType::Open {
            if self.type_.get_align_size() == b.type_.get_align_size() {
                // Compatible element data-type
                self.range_type = RangeType::Open;
                if 0 <= b.highind {
                    // If b has array indexing
                    let mut diffsz: intb = b.sstart.wsub(self.sstart);
                    diffsz /= self.type_.get_align_size() as intb;
                    let trialhi: int4 = (b.highind as intb).wadd(diffsz) as int4;
                    if self.highind < trialhi {
                        self.highind = trialhi;
                    }
                }
            } else if self.start == b.start {
                let meta = self.type_.get_metatype();
                if meta != type_metatype::TYPE_STRUCT && meta != type_metatype::TYPE_UNION {
                    self.range_type = RangeType::Open;
                }
            }
        } else if (b.flags & COPY_CONSTANT) != 0 && self.range_type == RangeType::Open {
            let diffsz: intb = b.sstart.wsub(self.sstart).wadd(b.size as intb);
            if diffsz > self.size as intb {
                let trialhi: int4 = (diffsz / (self.type_.get_align_size() as intb)) as int4;
                if self.highind < trialhi {
                    self.highind = trialhi;
                }
            }
        }
        if (self.flags & COPY_CONSTANT) != 0 && (b.flags & COPY_CONSTANT) == 0 {
            self.flags ^= COPY_CONSTANT;
        }
    }

    /// Try to form the union of `this` with another RangeHint (C++ `merge`,
    /// `varmap.cc:259-315`).
    ///
    /// Given that `this` and `b` intersect, redefine `this` so it becomes the
    /// union of the two original ranges. Preserves data-type info of both
    /// where possible.  Returns `Err` if the data-types are locked and cannot
    /// be reconciled (C++ throws `LowlevelError`).  The boolean return mirrors
    /// the C++ `bool` (which is `false` on every non-throw path — `merge`
    /// returns the "overlap that could be reconciled" flag, and the C++ body
    /// only ever returns `false`).
    pub fn merge(
        &mut self,
        b: &RangeHint,
        _space: &Rc<AddrSpace>,
        type_factory: &dyn TypeFactory,
    ) -> KunaResult<bool> {
        let did_reconcile: bool;
        let res_type: i32; // 0=this, 1=b, 2=confuse

        if self.contain(b) {
            did_reconcile = self.reconcile(b)?;
            if !did_reconcile && self.start != b.start {
                res_type = 2;
            } else {
                res_type = if self.preferred(b, did_reconcile)? { 0 } else { 1 };
            }
        } else {
            did_reconcile = false;
            res_type = if self.is_type_lock() { 0 } else { 2 };
        }

        // Check for really problematic cases
        if !did_reconcile && self.is_type_lock() {
            if b.is_type_lock() {
                return Err(KunaError::lowlevel(format!(
                    "Overlapping forced variable types : {}   {}",
                    self.type_.get_name(),
                    b.type_.get_name()
                )));
            }
            if self.start != b.start {
                return Ok(false); // Discard b entirely
            }
        }

        if res_type == 0 {
            self.absorb(b);
        } else if res_type == 1 {
            let copy_range = self.clone();
            self.type_ = Rc::clone(&b.type_);
            self.flags = b.flags;
            self.range_type = b.range_type;
            self.highind = b.highind;
            self.size = b.size;
            self.absorb(&copy_range);
        } else if res_type == 2 {
            // Concede confusion about types, set unknown type rather than this or b's type
            self.flags = 0;
            self.range_type = RangeType::Fixed;
            let diff: int4 = (b.sstart.wsub(self.sstart)) as int4;
            if diff.wadd(b.size) > self.size {
                self.size = diff.wadd(b.size);
            }
            if self.size != 1 && self.size != 2 && self.size != 4 && self.size != 8 {
                self.size = 1;
                self.range_type = RangeType::Open;
            }
            self.type_ = type_factory.get_base(self.size, type_metatype::TYPE_UNKNOWN)?;
            self.flags = 0;
            self.highind = -1;
            return Ok(false);
        }
        Ok(false)
    }

    /// Order `this` with another RangeHint (C++ `compare`, `varmap.cc:321-335`).
    ///
    /// Compare (signed) offset, size, RangeType, flags, and high index, in that
    /// order. Datatype is \e not compared.  Returns -1, 0, or 1.
    pub fn compare(&self, op2: &RangeHint) -> int4 {
        if self.sstart != op2.sstart {
            return if self.sstart < op2.sstart { -1 } else { 1 };
        }
        if self.size != op2.size {
            return if self.size < op2.size { -1 } else { 1 }; // Small sizes come first
        }
        if self.range_type != op2.range_type {
            return if self.range_type < op2.range_type { -1 } else { 1 };
        }
        if self.flags != op2.flags {
            return if self.flags < op2.flags { -1 } else { 1 };
        }
        if self.highind != op2.highind {
            return if self.highind < op2.highind { -1 } else { 1 };
        }
        0
    }

    /// Compare two RangeHints for the `stable_sort` in `MapState::initialize`
    /// (C++ `compareRanges`, `varmap.hh:126`): `a->compare(*b) < 0`.
    pub fn compare_ranges(a: &RangeHint, b: &RangeHint) -> std::cmp::Ordering {
        a.compare(b).cmp(&0)
    }
}

// ===========================================================================
// AddBase + AliasChecker (varmap.hh:137-166, varmap.cc:633-858)
// ===========================================================================

/// A Varnode pointer reference and a possible index added to it (C++
/// `AliasChecker::AddBase`, `varmap.hh:140-144`).
///
/// The C++ holds `Varnode *base` / `Varnode *index`; in the arena IR model
/// (ADR 0001) these are [`crate::context::VarnodeId`]s.  `index` is `None` for
/// the C++ `(Varnode *)0`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AddBase {
    /// The Varnode holding the base pointer (C++ `base`).
    pub base: crate::context::VarnodeId,
    /// The index value or NULL (C++ `index`).
    pub index: Option<crate::context::VarnodeId>,
}

impl AddBase {
    /// C++ constructor `AddBase(Varnode *b,Varnode *i)` (`varmap.hh:143`).
    pub fn new(base: crate::context::VarnodeId, index: Option<crate::context::VarnodeId>) -> AddBase {
        AddBase { base, index }
    }
}

/// A light-weight analyzer for pointers and aliasing on the stack (C++
/// `AliasChecker`, `varmap.hh:137-166`).
///
/// `gather` looks for pointer references into a specific AddressSpace (usually
/// the stack); `hasLocalAlias` then checks if a specific Varnode within that
/// space is (possibly) aliased by one of the gathered pointer references.
///
/// STUB: the C++ `gatherInternal`/`gatherAdditiveBase` walk the Varnode
/// descend graph and call `Funcdata::findSpacebaseInput`, neither of which is
/// available at this wave boundary.  The boundary-derivation, offset-gathering
/// (`gatherOffset`), alias sort, and the `hasLocalAlias` decision are ported in
/// full; the graph walk routes through [`AliasGatherAccess`].
pub struct AliasChecker {
    /// AddressSpace in which to search (C++ `space`).
    space: Option<Rc<AddrSpace>>,
    /// Collection of pointers into the AddressSpace (C++ `addBase`).
    add_base: Vec<AddBase>,
    /// List of aliased addresses, as offsets (C++ `alias`).
    alias: Vec<uintb>,
    /// Have aliases been calculated (C++ `calculated`).
    calculated: bool,
    /// Largest possible offset for a local variable (C++ `localExtreme`).
    local_extreme: uintb,
    /// Boundary offset separating locals and parameters (C++ `localBoundary`).
    local_boundary: uintb,
    /// Shallowest alias (C++ `aliasBoundary`).
    alias_boundary: uintb,
    /// 1 = stack grows negative, -1 = positive (C++ `direction`).
    direction: int4,
}

impl Default for AliasChecker {
    fn default() -> AliasChecker {
        AliasChecker::new()
    }
}

impl AliasChecker {
    /// C++ constructor `AliasChecker()` (`varmap.hh:158`).
    pub fn new() -> AliasChecker {
        AliasChecker {
            space: None,
            add_base: Vec::new(),
            alias: Vec::new(),
            calculated: false,
            local_extreme: 0,
            local_boundary: 0,
            alias_boundary: 0,
            direction: 0,
        }
    }

    /// Set up basic offset boundaries for what constitutes a local variable or
    /// a parameter on the stack (C++ `deriveBoundaries`, `varmap.cc:633-655`).
    ///
    /// The boundaries can be informed by the prototype's local/param ranges.
    /// The C++ takes a `FuncProto`; here the relevant pieces are passed in
    /// pre-extracted (the prototype subsystem surface is W4).
    fn derive_boundaries(&mut self, bounds: Option<&ProtoBoundaries>) {
        self.local_extreme = !0u64; // Default settings
        self.local_boundary = 0x1000000;
        if self.direction == -1 {
            self.local_extreme = self.local_boundary;
        }

        if let Some(b) = bounds {
            // C++: only adjusts when both a first-local range and a last-param
            // range exist.
            if b.has_local_first && b.has_param_last {
                self.local_boundary = b.param_last;
                if self.direction == -1 {
                    self.local_boundary = b.param_first;
                    self.local_extreme = self.local_boundary;
                }
            }
        }
    }

    /// Gather Varnodes that point onto the stack (C++ `gather`,
    /// `varmap.cc:692-704`).
    ///
    /// For the given function and address space, gather all Varnodes that are
    /// pointers into the address space.  The actual calculation can be deferred
    /// until the first time `hasLocalAlias` is called.
    pub fn gather(
        &mut self,
        spc: Rc<AddrSpace>,
        bounds: Option<&ProtoBoundaries>,
        defer: bool,
        access: &mut dyn AliasGatherAccess,
    ) {
        let dir = if spc.stack_grows_negative() { 1 } else { -1 };
        self.space = Some(Rc::clone(&spc));
        self.calculated = false; // Defer calculation
        self.add_base.clear();
        self.alias.clear();
        self.direction = dir; // direction == 1 for normal negative stack growth
        self.derive_boundaries(bounds);
        if !defer {
            self.gather_internal(access);
        }
    }

    /// Run through Varnodes looking for pointers into the stack (C++
    /// `gatherInternal`, `varmap.cc:660-684`).
    ///
    /// STUB: `gatherAdditiveBase` (the Varnode descend walk) and the
    /// spacebase-input lookup come from [`AliasGatherAccess`].  The
    /// offset-conversion and `aliasBoundary` accumulation are transcribed
    /// exactly.
    fn gather_internal(&mut self, access: &mut dyn AliasGatherAccess) {
        self.calculated = true;
        self.alias_boundary = self.local_extreme;
        let space = match &self.space {
            Some(s) => Rc::clone(s),
            None => return,
        };
        let spacebase = match access.find_spacebase_input(&space) {
            Some(vn) => vn,
            None => return, // No possible alias
        };

        access.gather_additive_base(spacebase, &mut self.add_base);
        // C++ iterates addBase, computing one alias offset per AddBase.
        let word_size = space.get_word_size();
        let entries: Vec<crate::context::VarnodeId> = self.add_base.iter().map(|ab| ab.base).collect();
        for base in entries {
            let mut offset = access.gather_offset(base);
            offset = AddrSpace::address_to_byte(offset, word_size); // Convert to byte offset
            self.alias.push(offset);
            if self.direction == 1 {
                if offset < self.local_boundary {
                    continue; // Parameter ref
                }
            } else if offset > self.local_boundary {
                continue; // Parameter ref
            }
            // Always consider anything AFTER a pointer reference as aliased,
            // regardless of the stack direction
            if offset < self.alias_boundary {
                self.alias_boundary = offset;
            }
        }
    }

    /// Return `true` if it looks like the given Varnode is aliased by a pointer
    /// (C++ `hasLocalAlias`, `varmap.cc:711-724`).
    pub fn has_local_alias(
        &mut self,
        vn: Option<(Rc<AddrSpace>, uintb)>,
        access: &mut dyn AliasGatherAccess,
    ) -> bool {
        let (vn_space, vn_offset) = match vn {
            Some(v) => v,
            None => return false,
        };
        if !self.calculated {
            self.gather_internal(access);
        }
        let space = match &self.space {
            Some(s) => Rc::clone(s),
            None => return false,
        };
        if !Rc::ptr_eq(&vn_space, &space) {
            return false;
        }
        // For positive stack growth this is not a good test (queued subfunction
        // parameters always have offsets a bit bigger than ALL locals).
        if self.direction == -1 {
            return false;
        }
        vn_offset >= self.alias_boundary
    }

    /// Sort the alias starting offsets (C++ `sortAlias`, `varmap.cc:726-730`).
    pub fn sort_alias(&mut self) {
        self.alias.sort();
    }

    /// Get the collection of pointer Varnodes (C++ `getAddBase`,
    /// `varmap.hh:162`).
    pub fn get_add_base(&self) -> &[AddBase] {
        &self.add_base
    }

    /// Get the list of alias starting offsets (C++ `getAlias`,
    /// `varmap.hh:163`).
    pub fn get_alias(&self) -> &[uintb] {
        &self.alias
    }
}

/// Stack-layout boundary inputs extracted from a `FuncProto`
/// (C++ `AliasChecker::deriveBoundaries`'s reads of the prototype model).
///
/// STUB(W4): the C++ reads `proto.hasModel()` and the first local / last param
/// `Range`s directly off the prototype model; that surface is a later wave.
/// The caller pre-extracts the four offsets so `deriveBoundaries` is a pure
/// function of them, transcribed exactly.
#[derive(Debug, Clone, Copy)]
pub struct ProtoBoundaries {
    /// `localrange.getFirstRange() != null` (C++).
    pub has_local_first: bool,
    /// `paramrange.getLastRange() != null` (C++).
    pub has_param_last: bool,
    /// `paramrange.getLastRange()->getLast()` (C++ `localBoundary` for direction 1).
    pub param_last: uintb,
    /// `paramrange.getFirstRange()->getFirst()` (C++ `localBoundary` for direction -1).
    pub param_first: uintb,
}

/// The Varnode-graph hooks `AliasChecker` needs from `Funcdata` (C++
/// `Funcdata::findSpacebaseInput` and `AliasChecker::gatherAdditiveBase`/
/// `gatherOffset`, the latter two static but reading the def graph).
///
/// STUB(W7): the descend list and the spacebase input are `Funcdata` surfaces
/// not present at this boundary.  `Funcdata` will implement this trait, at
/// which point the alias gather runs against the real IR; the offset/sort/
/// boundary logic above is already final.
pub trait AliasGatherAccess {
    /// C++ `Funcdata::findSpacebaseInput(space)` — the input Varnode that is the
    /// stack pointer for the space, if any.
    fn find_spacebase_input(&self, space: &Rc<AddrSpace>) -> Option<crate::context::VarnodeId>;
    /// C++ `AliasChecker::gatherAdditiveBase(startvn, addbase)` — collect the
    /// roots of every additive expression tree rooted at `startvn`.
    fn gather_additive_base(&mut self, startvn: crate::context::VarnodeId, addbase: &mut Vec<AddBase>);
    /// C++ `AliasChecker::gatherOffset(vn)` — the constant portion of the sum
    /// the given Varnode is the result of.
    fn gather_offset(&mut self, vn: crate::context::VarnodeId) -> uintb;
}

/// The Symbol overlap facts `Funcdata::syncVarnodesWithSymbols` needs for one
/// storage location (the resolved Symbol's `getAllFlags`, `getSize`, and the
/// `getSizedType` for the access).  See [`ScopeLocal::sync_overlap`].
#[derive(Debug, Clone)]
pub struct SyncOverlap {
    /// C++ `entry->getAllFlags()` — `extraflags | symbol->getFlags()`.
    pub all_flags: uint4,
    /// C++ `entry->getSize()` — the size of this storage piece.
    pub entry_size: int4,
    /// C++ `entry->getSizedType(addr, size)` — the type to assign the access, or
    /// `None` when not updating types / no exact piece.
    pub sized_type: Option<Rc<Datatype>>,
    /// The owning Symbol id (so the naming pass can read its name).
    pub symbol_id: crate::database::SymbolId,
    /// C++ `entry->extraflags` — the per-entry extra flags.  For a join-piece
    /// SymbolEntry (the extra maps `Scope::addMap` registers over each piece of
    /// a join-address Symbol, database.cc:1161-1180) these carry precislo/precishi
    /// and NOT `mapped`; the kuna addrtied pre-tie consults this to avoid tying a
    /// partial (SUBPIECE-extracted) piece access, matching C++ where the heritage
    /// pre-tie only ties whole-symbol storage.
    pub extraflags: uint4,
}

/// The smallest-containing SymbolEntry metadata `Funcdata::linkSymbol`'s
/// `queryProperties(addr, 1, usepoint)` returns, plus the bits
/// `handleSymbolConflict` (`funcdata_varnode.cc:1018`) reads off the entry
/// (`entry->getAddr()` / `entry->getSize()`) to run its conflicting-HighVariable
/// scan.  Produced by [`ScopeLocal::query_container_for_link`].
#[derive(Debug, Clone)]
pub struct LinkEntryInfo {
    /// `entry->getSymbol()` — the containing Symbol's identity in THIS scope
    /// (the id `<localdb>` encodes, via [`ScopeLocal::symbol_id_and_category`]).
    pub symbol: crate::database::SymbolId,
    /// `entry->getSymbol()->getDisplayName()` (the name the high renders if the
    /// entry is reused — e.g. the recovered parameter `a`).
    pub display_name: String,
    /// `(access_addr - entry_addr) + entry_offset` — the byte offset of the
    /// access within the Symbol (0 for a whole-symbol/scalar access).
    pub sym_off: int4,
    /// `entry->getSymbol()->getType()`.
    pub sym_type: Option<Rc<Datatype>>,
    /// `entry->getAddr()` — the base of the containing entry (the conflict scan
    /// iterates `beginLoc(entry->getSize(), entry->getAddr())`).
    pub entry_addr: Address,
    /// `entry->getSize()` — the byte width of the containing entry.
    pub entry_size: int4,
    /// `entry->getSymbol()->getCategory()` (a parameter is category 0).
    pub category: int4,
    /// `entry->getSymbol()->isNameUndefined()` — the C++ `linkSpacebaseSymbol`
    /// namerec discriminator (coreaction.cc:3015): an undefined-named Symbol is
    /// the auto-created local that the C++ later renames via `buildDefaultName`.
    /// The kuna namerec rename only reaches whole-symbol-cover locals
    /// (`resolve_default_name`), so a spacebase-reference into an undefined-named
    /// composite (an unmapped array auto-local) must NOT render its raw
    /// `$$undefNN` placeholder; the caller falls back to the functional form.
    pub is_name_undefined: bool,
    /// `entry->getSymbol()->isIsolated()` — the C++ `Symbol::isIsolated` flag
    /// (database.hh:241) read by `Merge::mergeTestAdjacent` (merge.cc:198-205):
    /// an isolated covering Symbol refuses any speculative tie. (kuna L2)
    pub is_isolated: bool,
}

/// A snapshot of the SymbolEntry `RuleStringCopy`'s `queryContainer` returns,
/// holding the container entry's storage range and the owning Symbol's whole
/// data-type as owned values (so the `StringSequence` COPY driver can run its
/// mutating transform without keeping a `Database` borrow live).  Produced by
/// [`ScopeLocal::query_container`].
#[derive(Debug, Clone)]
pub struct StringContainerEntry {
    /// `entry->getAddr()` — the base address of the containing entry.
    pub addr: Address,
    /// `entry->getFirst()` — the first offset of the entry's storage.
    pub first: uintb,
    /// `entry->getSize()` — the byte width of the containing entry.
    pub size: int4,
    /// `entry->getSymbol()->getType()` — the owning Symbol's whole data-type.
    pub sym_type: Rc<Datatype>,
}

// ===========================================================================
// ScopeLocal (varmap.hh:212-269, varmap.cc:341-1620)
// ===========================================================================

/// The stack-frame symbol scope of a function (C++ `ScopeLocal`, a
/// `ScopeInternal` subclass, `varmap.hh:212`).
///
/// In C++ the `ScopeLocal` is a child of the architecture's global symbol
/// table (`glb->symboltab->attachScope(localmap, scope)`).  The merged Rust
/// tree carries the global `Database` on the *console* `Architecture`, not on
/// the `glb` [`ArchHandle`](crate::context::ArchHandle) the `Funcdata` holds, so
/// — faithful to the `ScopeInternal` self-containment (its `nametree` /
/// `maptable` / `category` / `rangetree` are all private members) — the
/// `ScopeLocal` owns its **own** single-scope [`Database`] here.  The only
/// cross-scope C++ dependency is `makeNameUnique` (consulted on the local
/// scope's own `nametree`, which IS self-contained) and `setRange`/`removeRange`
/// (which the C++ `Database` stores into this scope's own `rangetree`); both are
/// reproduced exactly against the owned `Database`.
///
/// The order-determining layout core (`RangeHint`/`MapState`/`AliasChecker`)
/// lives above this struct and is unit-tested independently; this struct ports
/// the `ScopeLocal`-specific layer that drives it: `resetLocalWindow`,
/// `restructure`, `createEntry`, `adjustFit`, `markNotMapped`,
/// `buildVariableName`, plus the `addSymbol`/`addCodeLabel` entry points the
/// console `map` commands reach.  The IR-mutating `restructureVarnode` gather
/// and `syncVarnodesWithSymbols` remain a documented stub (LOSS-109): the
/// `MapState` gather is supplied by the driver against the live IR.
pub struct ScopeLocal {
    /// The owned local symbol database (one functional scope).
    db: crate::database::Database,
    /// The id of the local scope within [`Self::db`].
    scope: crate::database::ScopeId,
    /// The address space (stack) holding this scope's variables (C++ `space`).
    space: Rc<AddrSpace>,
    /// Minimum offset of a stack parameter (C++ `minParamOffset`).
    min_param_offset: uintb,
    /// Maximum offset of a stack parameter (C++ `maxParamOffset`).
    max_param_offset: uintb,
    /// True if the layout range is locked / not auto-recovered (C++ `rangeLocked`).
    range_locked: bool,
    /// True if the stack grows toward negative offsets (C++ `stackGrowsNegative`).
    stack_grows_negative: bool,
    /// True if `restructure` could not reconcile overlaps (C++ `overlapProblems`).
    overlap_problems: bool,
    /// Storage-address -> data-type recommendations for input Varnodes (C++
    /// `list<TypeRecommend> typeRecommend`, `varmap.hh:218`).
    type_recommend: Vec<TypeRecommend>,
    /// Storage-location -> name recommendations (C++ `list<NameRecommend>
    /// nameRecommend`, `varmap.hh:216`): the names of namelocked-but-NOT-
    /// typelocked Symbols, which do not survive `clearUnlockedCategory(-1)` at
    /// restructure — C++ collects them (`collectNameRecs`) and re-applies at
    /// naming time (`recoverNameRecommendationsForSymbols`, the mechanism that
    /// makes a GUI rename of an untyped local persist).  kuna's ghidra-mode
    /// seeds these directly from the host `<localdb>` answer.
    name_recommend: Vec<NameRecommend>,
    /// Dynamic-storage name recommendations (C++ `list<DynamicRecommend>
    /// dynRecommend`, `varmap.hh:217`): the same identity carrier as
    /// [`Self::name_recommend`] for a variable whose storage is a data-flow
    /// HASH rather than an address — what the Ghidra GUI writes for any
    /// variable that `requiresDynamicStorage` (a unique-space representative,
    /// a `splitOutMergeGroup` product).  Applied through
    /// `DynamicHash::find_varnode` (`recoverNameRecommendationsForSymbols`,
    /// varmap.cc:1557-1573).
    dyn_recommend: Vec<DynamicRecommend>,
    /// (kuna) Whether any Symbol in this scope has ever been marked isolated
    /// (C++ `Symbol::setIsolated`).  Read by
    /// [`Self::has_isolated_symbols`]; see that method for why it is monotone.
    isolated_present: bool,
}

impl ScopeLocal {
    /// C++ `ScopeLocal::ScopeLocal(id,spc,fd,glb)` (`varmap.cc:341`).
    ///
    /// `num_spaces` sizes the per-space `maptable` (the C++ `ScopeInternal`
    /// constructor's `maptable.resize(numSpaces, null)`); it must cover the stack
    /// space's index so `addSymbol` on a stack address can allocate its slot.
    pub fn new(id: uint8, spc: Rc<AddrSpace>, fname: &str, num_spaces: int4) -> KunaResult<ScopeLocal> {
        let mut db = crate::database::Database::new(true);
        // The local scope is the *global* scope of this private database (empty
        // parent), but it IS functional (C++ ScopeLocal has a non-null `fd`).
        let scope = db.find_create_scope(id, "", None, num_spaces)?;
        db.scope_mut(scope).is_functional = true;
        db.scope_mut(scope).name = fname.to_string();
        db.scope_mut(scope).display_name = fname.to_string();
        Ok(ScopeLocal {
            db,
            scope,
            space: spc,
            min_param_offset: !0u64, // ~((uintb)0)
            max_param_offset: 0,
            range_locked: false,
            stack_grows_negative: true,
            overlap_problems: false,
            type_recommend: Vec::new(),
            name_recommend: Vec::new(),
            dyn_recommend: Vec::new(),
            isolated_present: false,
        })
    }

    /// C++ `ScopeLocal::markUnaliased` (`varmap.cc:1290`): mark every local stack
    /// Symbol not crossed by an alias as `nolocalalias`.  Thin wrapper over the
    /// owned database's [`Database::mark_unaliased`], passing this scope's space
    /// index and the architecture's `alias_block_level`.
    pub fn mark_unaliased(&mut self, alias: &[uintb], alias_block_level: int4) {
        let space_index = self.space.get_index() as usize;
        self.db
            .mark_unaliased(self.scope, space_index, alias, alias_block_level);
    }

    /// C++ `ScopeLocal::isUnaffectedStorage` (`varmap.hh:244`): is `vn` stored in
    /// this scope's (stack) address space?  Used by `ActionRestrictLocal` to test
    /// whether a saved-register COPY's destination is a stack slot that should be
    /// unmapped.
    pub fn is_unaffected_storage(&self, vn_space: &Rc<AddrSpace>) -> bool {
        Rc::ptr_eq(vn_space, &self.space) || vn_space.get_index() == self.space.get_index()
    }

    /// C++ `ScopeLocal::isUnmappedUnaliased` (`varmap.cc:494`): is the (unmapped)
    /// Varnode at `(vn_space, vn_offset)` provably NOT crossed by any pointer alias?
    /// True only for a stack-space location that lies outside the recovered stack-
    /// parameter window `[minParamOffset, maxParamOffset]` (or when no stack params
    /// are known).  Used by `syncVarnodesWithSymbol` to mark such a slot
    /// `nolocalalias` so `RuleIndirectCollapse` can drop the per-call INDIRECT that
    /// guards a saved-register (e.g. MIPS gp) spill that `markNotMapped` unmapped.
    /// Wired into `Funcdata::sync_varnodes_with_symbols` (the `unmapped_alias_check`
    /// arm) — it forwards Gp Test #2; the Switch suite holds because
    /// `protect_switch_paths` shields the switch INDIRECTs in the jump-table-
    /// recovery partial clone.
    pub fn is_unmapped_unaliased(&self, vn_space: &Rc<AddrSpace>, vn_offset: uintb) -> bool {
        if !(Rc::ptr_eq(vn_space, &self.space) || vn_space.get_index() == self.space.get_index()) {
            return false;
        }
        // (no known stack params)
        if self.max_param_offset < self.min_param_offset {
            return true;
        }
        vn_offset < self.min_param_offset || vn_offset > self.max_param_offset
    }

    /// C++ `ScopeLocal::markNotMapped` (`varmap.cc:510-545`): mark the range
    /// `[first, first+sz)` in `spc` as not mapped to a local Symbol, removing any
    /// Symbols already created there and dropping the range from the discovery
    /// window.  When `parameter` is set, the range start/end extend
    /// `minParamOffset`/`maxParamOffset` (so a stack parameter passed to a locked
    /// sub-function call is excised above the parameter boundary).
    ///
    /// Transcribes the C++ head (the `last` wrap/clamp + the
    /// parameter-boundary update); the symbol-removal loop and `removeRange` are the
    /// owned-database [`Database::mark_not_mapped_core`].
    pub fn mark_not_mapped(
        &mut self,
        spc: &Rc<AddrSpace>,
        first: uintb,
        sz: int4,
        parameter: bool,
    ) {
        if !self.is_unaffected_storage(spc) {
            return;
        }
        // uintb wrapping
        let mut last = first.wrapping_add(sz as uintb).wrapping_sub(1);
        // Do not allow the range to cover the split point between "negative" and
        // "positive" stack offsets.
        let highest = self.space.get_highest();
        if last < first {
            // Check for possible wrap around.
            last = highest;
        } else if last > highest {
            last = highest;
        }
        if parameter {
            // Everything above parameter.
            if first < self.min_param_offset {
                self.min_param_offset = first;
            }
            if last > self.max_param_offset {
                self.max_param_offset = last;
            }
        }
        self.db
            .mark_not_mapped_core(self.scope, Rc::clone(&self.space), first, last, sz, parameter);
    }

    /// C++ `ScopeLocal::addTypeRecommendation` (`varmap.cc:1590`): associate a
    /// data-type with a storage address.  If an input Varnode appears at this
    /// address with no other type info, the data-type is applied later by
    /// `applyTypeRecommendations`.
    pub fn add_type_recommendation(&mut self, addr: Address, dt: Rc<Datatype>) {
        self.type_recommend.push(TypeRecommend::new(addr, dt));
    }

    /// C++ `ScopeLocal::hasTypeRecommendations` (`varmap.hh:259`): are there any
    /// pending type recommendations?
    pub fn has_type_recommendations(&self) -> bool {
        !self.type_recommend.is_empty()
    }

    /// C++ `ScopeLocal::addRecommendName` (`varmap.cc:1583`): record a name
    /// recommendation for a storage location.  ghidra-mode seeds these from
    /// the host `<localdb>`'s namelocked-but-not-typelocked locals (the shape
    /// a GUI rename produces), the same identities C++ `collectNameRecs`
    /// would harvest before restructure clears the unlocked symbols.
    pub fn add_recommend_name(
        &mut self,
        addr: Address,
        usepoint: Address,
        size: int4,
        name: &str,
    ) {
        self.name_recommend.push(NameRecommend::new(
            addr,
            usepoint,
            size,
            name.to_string(),
            0,
        ));
    }

    /// The pending name recommendations (C++ `nameRecommend` list), applied by
    /// the `ActionNameVars` port (`recoverNameRecommendationsForSymbols`).
    pub fn name_recommendations(&self) -> &[NameRecommend] {
        &self.name_recommend
    }

    /// C++ `ScopeLocal::addRecommendDynamic` (`varmap.cc:1595`): record a
    /// name recommendation for a DYNAMIC (hash-addressed) storage location.
    pub fn add_recommend_dynamic(&mut self, use_point: Address, hash: u64, name: &str) {
        self.dyn_recommend
            .push(DynamicRecommend::new(use_point, hash, name.to_string(), 0));
    }

    /// The pending dynamic name recommendations (C++ `dynRecommend`).
    pub fn dynamic_recommendations(&self) -> &[DynamicRecommend] {
        &self.dyn_recommend
    }

    /// The pending `(address, type)` recommendations (C++ `typeRecommend` list),
    /// consumed by [`Funcdata::apply_type_recommendations`] (which owns
    /// `findVarnodeInput`).
    pub fn type_recommendations(&self) -> &[TypeRecommend] {
        &self.type_recommend
    }

    /// The address space holding this scope's variables (C++ `getSpaceId`).
    pub fn get_space_id(&self) -> &Rc<AddrSpace> {
        &self.space
    }

    /// Borrow the owned symbol database (read-only).
    pub fn database(&self) -> &crate::database::Database {
        &self.db
    }

    /// The local scope id within the owned database.
    pub fn scope_id(&self) -> crate::database::ScopeId {
        self.scope
    }

    /// Encode this scope as a `<localdb>` element (C++ `ScopeLocal::encode`,
    /// varmap.cc:462-470): the `main=` space + `lock=` attributes, then the
    /// inner `<scope>` document.  Also stands in for the C++
    /// `encodeRecursive(encoder,false)` call site (`Funcdata::encode`) — a
    /// `ScopeLocal`'s private database has no child scopes to recurse into.
    pub fn encode(&self, encoder: &mut dyn kuna_base::marshal::Encoder) -> KunaResult<()> {
        self.encode_with_wire_symbols(&[], encoder)
    }

    /// [`ScopeLocal::encode`] with WIRE-ONLY symbols appended to the
    /// `<symbollist>` (see [`crate::database::WireSymbol`]).
    pub fn encode_with_wire_symbols(
        &self,
        wire_symbols: &[crate::database::WireSymbol],
        encoder: &mut dyn kuna_base::marshal::Encoder,
    ) -> KunaResult<()> {
        use crate::remote_provider::{ATTRIB_LOCK, ATTRIB_MAIN, ELEM_LOCALDB};
        encoder.open_element(&ELEM_LOCALDB);
        encoder.write_space(&ATTRIB_MAIN, &self.space);
        encoder.write_bool(&ATTRIB_LOCK, self.range_locked);
        self.db.encode_scope_with_wire_symbols(self.scope, wire_symbols, encoder)?;
        encoder.close_element(&ELEM_LOCALDB);
        Ok(())
    }

    /// Reserve a fresh internal-range symbol id without creating a Symbol —
    /// the id source for [`crate::database::WireSymbol`]s.
    pub fn reserve_internal_symbol_id(&mut self) -> u64 {
        self.db.reserve_internal_symbol_id(self.scope)
    }

    /// C++ `ScopeInternal::assignDefaultNames(base)` (`database.cc:2880`) on this
    /// local scope — the catch-all rename `ActionNameVars::apply` runs as its final
    /// statement (`coreaction.cc:3079`) so no `$$undef` placeholder survives the
    /// pass.  See [`crate::kuna_undefname`].
    pub fn assign_default_names(
        &mut self,
        base: &mut int4,
        arch: &dyn crate::database::DatabaseArch,
    ) -> KunaResult<()> {
        self.db.assign_default_names(self.scope, base, arch)
    }

    /// C++ `Scope::queryProperties` (`database.cc:1268`) on the local stack scope:
    /// the Varnode boolean properties (`mapped | addrtied | typelock | ...`) of a
    /// storage range covered by — or owned by — this scope.  Returns `0` when the
    /// range is not in this scope's map.  This is the local-scope half of the
    /// `localmap->queryProperties` walk `Heritage::guard` runs to mark a mapped
    /// stack range address-tied (so its def chain is guarded across calls and
    /// survives `ActionDeadCode`); the global-scope half is
    /// [`ArchHandle::query_global_properties`](crate::context::ArchHandle::query_global_properties).
    pub fn query_properties(&self, addr: &Address, size: int4, usepoint: &Address) -> uint4 {
        let (_, flags) = self.db.query_properties(self.scope, addr, size, usepoint);
        flags
    }

    /// Did the last `restructure` leave unreconciled overlaps (C++
    /// `overlapProblems`)?
    pub fn has_overlap_problems(&self) -> bool {
        self.overlap_problems
    }

    /// C++ `ScopeLocal::resetLocalWindow` (`varmap.cc:432`): reset the discovery
    /// window for new local variables.  `local_range`/`param_range` are the
    /// function prototype's stack ranges; `grows_negative` is
    /// `proto.isStackGrowsNegative()`.  When the layout is range-locked the range
    /// is left intact (the C++ early return).
    pub fn reset_local_window(
        &mut self,
        local_range: &kuna_base::address::RangeList,
        param_range: &kuna_base::address::RangeList,
        grows_negative: bool,
    ) {
        self.stack_grows_negative = grows_negative;
        self.min_param_offset = !0u64;
        self.max_param_offset = 0;

        if self.range_locked {
            return;
        }

        let mut newrange = kuna_base::address::RangeList::new();
        for rng in local_range.iter() {
            newrange.insert_range(rng.get_space().clone(), rng.get_first(), rng.get_last());
        }
        for rng in param_range.iter() {
            newrange.insert_range(rng.get_space().clone(), rng.get_first(), rng.get_last());
        }
        // C++ glb->symboltab->setRange(this,newrange) stores into this scope's
        // own rangetree (Database::setRange -> scope->rangetree = newrange).
        self.db.set_range(self.scope, newrange);
    }

    /// C++ `ScopeLocal::addSymbol(name,ct,addr,usepoint)` — the function-local
    /// form reached by `IfcMapaddress` (`getScopeLocal()->addSymbol(...)`).
    ///
    /// In C++ this is `ScopeInternal::addSymbol`, which auto-names an empty name
    /// (`buildVariableName`) and maps the storage.  The console always passes an
    /// explicit name, so the auto-name branch is not exercised here; an empty
    /// name maps without a name (the symbol gets a name later via `restructure`).
    pub fn add_symbol(
        &mut self,
        name: &str,
        ct: Rc<Datatype>,
        addr: &Address,
        usepoint: &Address,
    ) -> KunaResult<crate::database::SymbolId> {
        let (sym, _eref) = self.db.add_symbol_mapped(self.scope, name, ct, addr, usepoint)?;
        Ok(sym)
    }

    /// Materialize a recovered/locked function parameter as a Symbol in this
    /// scope, mirroring `ProtoStoreSymbol::setInput`'s `scope->addSymbol(nm,type,
    /// addr,usepoint)` + `scope->setCategory(sym, function_parameter, i)`
    /// (`fspec.cc:3174`).  The kuna `FuncProto` stores its parameters in a
    /// `ProtoStoreInternal` (no backing symbol scope, the `setScope` stub), so the
    /// param Symbols that `Funcdata::linkSymbol`/`Scope::queryProperties` would
    /// otherwise find do not exist; this method creates them so the body Varnodes
    /// bind to the parameter names (`ptr`/`a`/`b`) instead of the raw registers.
    ///
    /// Idempotent: if a Symbol already overlaps the parameter's storage (a console
    /// `map addr`, a seeded host local, or a prior call) it is not re-created; it
    /// is categorized as this parameter slot only when its storage matches the
    /// parameter's EXACTLY (see below).
    /// Returns the new `SymbolId`, or `None` when an existing symbol was reused.
    pub fn add_param_symbol(
        &mut self,
        i: int4,
        name: &str,
        ct: Rc<Datatype>,
        addr: &Address,
        restricted_usepoint: &Address,
    ) -> KunaResult<Option<crate::database::SymbolId>> {
        if addr.is_invalid() || ct.get_size() < 1 {
            return Ok(None);
        }
        // C++ `linkSymbol`/`queryProperties` would find any existing overlapping
        // entry; only create when none exists (the `entry == 0` arm of
        // `setInput`/`linkSymbol`).
        if let Some(eref) = self.db.find_overlap(self.scope, addr, ct.get_size()) {
            // (kuna, ghidra Phase 4) An EXACT-storage match is this parameter's
            // own symbol under another guise — a host local seeded at the very
            // same address and width, or a prior call's creation — so it carries
            // the slot.  Upstream reaches the same state from the other end
            // (`ProtoStoreSymbol::setInput`, fspec.cc:3150-3183): it looks the
            // symbol up BY SLOT (`getCategorySymbol(function_parameter,i)`) and,
            // when that symbol's `getFirstWholeMap()` entry disagrees on addr OR
            // size, REMOVES it and adds a fresh one — it never promotes an
            // unrelated overlapping local.  So the exactness test is the whole
            // guard: an 8-byte local merely OVERLAPPING a 4-byte parameter must
            // stay `no_category`, or (a) `<localdb>` would advertise the local's
            // name/type/storage as cat-0 slot `i`, whose storage compare in
            // Java's `checkFullCommit` fails and force-rewrites the user's
            // signature, and (b) `HighFunctionDBUtil.getDatabaseParameter` keys
            // the DB slot off `getCategoryIndex()`, so renaming that LOCAL in
            // the GUI would rewrite DB parameter `i`.  This runs in the main
            // loop (`ActionRestructureVarnode`), standalone included, and
            // `clear_unlocked_category_negative` only clears `cat < 0` — a
            // wrongly promoted symbol would be permanent.
            let (entry_addr, entry_size) = {
                let e = self.db.entry(self.scope, eref);
                (e.get_addr().clone(), e.get_size())
            };
            if entry_addr == *addr && entry_size == ct.get_size() {
                let sym = self.db.entry(self.scope, eref).symbol;
                if self.db.symbol(sym).get_category()
                    != crate::database::symbol_category::FUNCTION_PARAMETER
                {
                    self.db.set_category(
                        self.scope,
                        sym,
                        crate::database::symbol_category::FUNCTION_PARAMETER,
                        i,
                    );
                }
            }
            return Ok(None);
        }
        // C++ `ProtoStoreSymbol::setInput` (fspec.cc:3170-3171): the usepoint is
        // empty (whole-function) only when some Scope already *owns* the
        // parameter's storage range — otherwise it is the function's
        // `restricted_usepoint` (entry-1).  A register parameter's storage is NOT
        // owned by the local stack scope, so `discoverScope` returns null and the
        // usepoint becomes `restricted_usepoint`.  This keeps the SymbolEntry's
        // `uselimit` NON-empty so `addMapInternal` does NOT mark the register
        // parameter `addrtied` (database.cc:1154) — matching C++, where `a0(i)` is
        // `mapped` but not `tied`.  Passing an invalid usepoint here (the prior
        // kuna behavior) made every register parameter `addrtied`, which let
        // `mergeAddrTied` over-merge the input register version with any later
        // same-register value (the LOSS-247 Gp-#2 `&v1`->`a0` over-merge).
        // C++ `ProtoStoreSymbol::setInput` (fspec.cc:3170): usepoint stays empty
        // (whole-function ⇒ `addrtied`) only when a Scope already *owns* the
        // parameter's storage; otherwise it is `restricted_usepoint` (entry-1),
        // keeping the SymbolEntry NON-empty ⇒ NOT `addrtied`.
        //
        // The over-merge this corrects is specifically a REGISTER input parameter
        // (the LOSS-247 Gp-#2 `&v1`->`a0`): a register's storage is owned by no
        // Scope, so `discoverScope` returns null and the register parameter is NOT
        // addrtied — which is what stops `mergeAddrTied` from grouping the input
        // register version with later same-register values.  Restricting the
        // restricted-usepoint to register storage keeps the join/stack struct
        // parameter path (`d` in Stack-spill) byte-identical to its prior behavior:
        // those go through `Scope::addMap`'s join arm (the per-piece stack entries),
        // whose kuna rendering already inlines `d.field_b`, and which the
        // single-point `restricted_usepoint` uselimit would otherwise de-tie and
        // split.  `discover_scope` already returns null for a register, so this is a
        // faithful subset of the C++ condition (it never *adds* a usepoint the C++
        // would not).
        let is_register = addr
            .get_space()
            .map(|s| s.get_type() == kuna_base::space::spacetype::IPTR_PROCESSOR)
            .unwrap_or(false);
        let usepoint = if is_register
            && self.db.discover_scope(self.scope, addr, ct.get_size(), &Address::new_invalid()).is_none()
        {
            restricted_usepoint.clone()
        } else {
            Address::new_invalid()
        };
        let (sym, _eref) = self.db.add_symbol_mapped(self.scope, name, ct, addr, &usepoint)?;
        self.db.set_category(
            self.scope,
            sym,
            crate::database::symbol_category::FUNCTION_PARAMETER,
            i,
        );
        Ok(Some(sym))
    }

    /// C++ `Scope::addCodeLabel` reached via `getScopeLocal()->addCodeLabel`
    /// (`IfcMaplabel` fd-local form).
    pub fn add_code_label(
        &mut self,
        addr: &Address,
        name: &str,
        lab_type: Rc<Datatype>,
    ) -> KunaResult<crate::database::SymbolId> {
        self.db.add_code_label(self.scope, addr, name, lab_type)
    }

    /// C++ `Scope::addDynamicSymbol` reached via
    /// `getScopeLocal()->addDynamicSymbol` (`IfcMaphash` fd-local form,
    /// `ifacedecomp.cc:603`).  A Symbol attached to a dynamic hash + code address.
    pub fn add_dynamic_symbol(
        &mut self,
        name: &str,
        ct: Rc<Datatype>,
        caddr: &Address,
        hash: uint8,
    ) -> KunaResult<crate::database::SymbolId> {
        self.db.add_dynamic_symbol(self.scope, name, ct, caddr, hash)
    }

    /// C++ `Scope::addUnionFacetSymbol` reached via
    /// `getScopeLocal()->addUnionFacetSymbol` (`IfcMapunionfacet` fd-local form,
    /// `ifacedecomp.cc:797`).  Forces a field interpretation of a union-typed
    /// Varnode at a specific PcodeOp identified by dynamic hash.
    pub fn add_union_facet_symbol(
        &mut self,
        name: &str,
        dt: Rc<Datatype>,
        field_num: int4,
        addr: &Address,
        hash: uint8,
    ) -> KunaResult<crate::database::SymbolId> {
        self.db.add_union_facet_symbol(self.scope, name, dt, field_num, addr, hash)
    }

    /// C++ `ScopeInternal::setAttribute` reached via the `map` commands
    /// (`sym->getScope()->setAttribute(sym, namelock|typelock)`).
    pub fn set_attribute(&mut self, sym: crate::database::SymbolId, attr: uint4) {
        self.db.set_attribute(sym, attr);
    }

    /// C++ `Symbol::setIsolated(val)` reached through this scope's symbol table
    /// (`IfcTypeVarnode`: `sym->setIsolated(true)`).
    pub fn set_symbol_isolated(&mut self, sym: crate::database::SymbolId, val: bool) {
        self.isolated_present |= val;
        self.db.symbol_mut(sym).set_isolated(val);
    }

    /// (kuna) Can any Symbol in this scope answer `true` to
    /// [`Symbol::is_isolated`](crate::database::Symbol::is_isolated)?
    ///
    /// `false` lets a caller skip a containment query it already knows the
    /// answer to.  [`Self::set_symbol_isolated`] is the only route by which the
    /// `ISOLATE` dispflag can ever reach a Symbol in a function-local scope —
    /// `set_attribute` writes the varnode-flag word behind a mask that excludes
    /// it, `set_display_format` masks the low format bits, the symbol
    /// constructors never set it, and the attribute is encoded (`ATTRIB_MERGE`)
    /// but never decoded.  The flag is deliberately monotone: clearing it on a
    /// `set_symbol_isolated(sym, false)` would require knowing no *other*
    /// Symbol is still isolated, and a stale `true` only costs the query it
    /// would have run anyway.
    pub fn has_isolated_symbols(&self) -> bool {
        self.isolated_present
    }

    /// C++ `ScopeInternal::setDisplayFormat(sym, attr)` (`database.cc:2246-2250`),
    /// reached via `sym->getScope()->setDisplayFormat(sym, format)` by
    /// `IfcForceFormat`: force the integer display format on a constant Symbol.
    pub fn set_display_format(&mut self, sym: crate::database::SymbolId, format: uint4) {
        self.db.symbol_mut(sym).set_display_format(format);
    }

    /// C++ `Scope::getFullName()` for this local scope (`IfcTypeVarnode`'s
    /// success message names `scope->getFullName()`).
    pub fn full_name(&self) -> String {
        self.db.get_full_name(self.scope)
    }

    /// C++ `Scope::queryByName(basename,res)` against this local scope (the
    /// `rename`/`retype`/`remove`/`isolate` commands resolve a symbol by name).
    pub fn query_by_name(&self, nm: &str) -> Vec<crate::database::SymbolId> {
        self.db.query_by_name(self.scope, nm)
    }

    /// C++ `Symbol::getCategory()` for `sym` in this scope.
    pub fn symbol_category(&self, sym: crate::database::SymbolId) -> int4 {
        self.db.symbol(sym).get_category()
    }

    /// C++ `Scope::renameSymbol(sym,newname)` against this local scope.
    pub fn rename_symbol(
        &mut self,
        sym: crate::database::SymbolId,
        newname: &str,
    ) -> KunaResult<()> {
        self.db.rename_symbol(sym, newname)
    }

    /// C++ `Scope::retypeSymbol(sym,ct)` against this local scope.
    pub fn retype_symbol(
        &mut self,
        sym: crate::database::SymbolId,
        ct: Rc<Datatype>,
    ) -> KunaResult<()> {
        self.db.retype_symbol(sym, ct)
    }

    /// C++ `Scope::addEquateSymbol(nm,format,value,addr,hash)` against this local
    /// scope (`IfcMapconvert`).  `base1` is the `getBase(1,TYPE_UNKNOWN)` type the
    /// EquateSymbol carries (resolved by the caller from the architecture).
    pub fn add_equate_symbol(
        &mut self,
        nm: &str,
        format: uint4,
        value: uintb,
        addr: &Address,
        hash: uint8,
        base1: Rc<Datatype>,
    ) -> KunaResult<crate::database::SymbolId> {
        self.db.add_equate_symbol(self.scope, nm, format, value, addr, hash, base1)
    }

    /// C++ `ScopeLocal::adjustFit` (`varmap.cc:587`): shrink `a` so it fits the
    /// mapped region and doesn't overlap an existing Symbol.  `false` if no valid
    /// adjustment is possible.
    fn adjust_fit(&self, a: &mut RangeHint) -> bool {
        if a.size == 0 {
            return false; // Nothing to fit
        }
        if a.is_type_lock() {
            return false; // Already entered
        }
        let addr = Address::new(self.space.clone(), a.start);
        let mut maxsize = self.db.scope(self.scope).get_range_tree().longest_fit(&addr, a.size as u64);
        if maxsize == 0 {
            return false;
        }
        if maxsize < a.size as u64 {
            // Suggested range doesn't fit.
            if maxsize < a.type_.get_size() as u64 {
                return false; // Can't shrink that match
            }
            a.size = maxsize as int4;
        }
        // We want ANY symbol that might be within this range.
        let entry = match self.db.find_overlap(self.scope, &addr, a.size) {
            None => return true,
            Some(e) => e,
        };
        let entry_addr = self.db.entry(self.scope, entry).get_addr().clone();
        let same_space = entry_addr
            .get_space()
            .zip(addr.get_space())
            .map(|(a, b)| a.get_index() == b.get_index())
            .unwrap_or(false);
        if entry_addr.get_offset() <= addr.get_offset() && same_space {
            // entry.getAddr() <= addr : generally shouldn't be possible.
            return false;
        }
        maxsize = entry_addr.get_offset().wrapping_sub(a.start);
        if maxsize < a.type_.get_size() as u64 {
            return false; // Can't shrink for this type
        }
        a.size = maxsize as int4;
        true
    }

    /// C++ `ScopeLocal::createEntry` (`varmap.cc:617`): construct a concrete
    /// data-type (and an array if the range holds more than one element) for the
    /// `RangeHint` and enter it as a Symbol.
    fn create_entry(&mut self, a: &RangeHint, types: &dyn TypeFactory) -> KunaResult<()> {
        let addr = Address::new(self.space.clone(), a.start);
        let usepoint = Address::new_invalid();
        let mut ct = types.concretize(Rc::clone(&a.type_))?;
        let align = ct.get_align_size();
        // C++ `a.size / ct->getAlignSize()` (int4 division); guard a 0 align.
        let num = if align != 0 { a.size / align } else { a.size };
        if num > 1 {
            ct = types.get_type_array(num, ct)?;
        }
        self.add_symbol("", ct, &addr, &usepoint)?;
        Ok(())
    }

    /// C++ `ScopeLocal::restructure` (`varmap.cc:1294`): merge the collected
    /// `RangeHint`s into a disjoint cover of Symbols.  Returns true if there were
    /// overlaps that could not be reconciled.
    pub fn restructure(&mut self, state: &mut MapState, types: &dyn TypeFactory) -> KunaResult<bool> {
        let mut overlap_problems = false;
        if !state.initialize()? {
            return Ok(overlap_problems); // No references to stack at all
        }

        let mut cur: RangeHint = state.next().clone();
        while state.get_next() {
            let next: RangeHint = state.next().clone();
            if next.sstart < cur.sstart.wrapping_add(cur.size as int8) {
                // Do the ranges intersect — union them.
                if cur.merge(&next, &self.space, types)? {
                    overlap_problems = true;
                }
            } else if !cur.attempt_join(&next) {
                if cur.range_type == RangeType::Open {
                    // C++ `cur.size = next->sstart - cur.sstart;` (intb diff
                    // truncated to int4); the gap is small and positive here.
                    cur.size = next.sstart.wrapping_sub(cur.sstart) as int4;
                }
                if self.adjust_fit(&mut cur) {
                    self.create_entry(&cur, types)?;
                }
                cur = next;
            }
        }
        // The last range is artificial so we don't build an entry for it.
        self.overlap_problems = overlap_problems;
        Ok(overlap_problems)
    }

    /// C++ `Scope::inScope(addr,size,usepoint)` reached via `lm->inScope(...)`:
    /// are all bytes of the range owned by this scope's range tree?
    pub fn in_scope(&self, addr: &Address, size: int4) -> bool {
        self.db.scope(self.scope).in_scope(addr, size, &Address::new_invalid())
    }

    /// C++ `localmap->clearUnlockedCategory(-1)` (the head of
    /// `ScopeLocal::restructureVarnode`, `varmap.cc:1259`): drop the unlocked
    /// auto-recovered stack Symbols so each restructure pass re-derives the layout
    /// from the current Varnodes.
    pub fn clear_unlocked_category_negative(&mut self) -> KunaResult<()> {
        self.db.clear_unlocked_category_negative(self.scope)
    }

    /// A clone of this scope's range tree (C++ `getRangeTree()`), the analysis
    /// range `MapState` clears the param range out of (`restructureVarnode`).
    pub fn range_tree_clone(&self) -> kuna_base::address::RangeList {
        self.db.scope(self.scope).get_range_tree().clone()
    }

    /// The console-mapped Symbol specs in this scope, so they can be re-seeded
    /// into a freshly-built `Funcdata` (the kuna console rebuilds the IR on
    /// `decompile`, where C++ reuses the same `fd`; this carries the `map addr`
    /// symbols across that rebuild).  Returns `(name, type, addr, all_flags)`
    /// per non-dynamic, address-tied SymbolEntry.
    pub fn mapped_symbol_specs(&self) -> Vec<(String, Rc<Datatype>, Address, uint4)> {
        let idx = self.space.get_index() as usize;
        let mut out = self.db.scope_space_symbol_specs(self.scope, idx);
        // The console `map addr <ramaddr> <type> <name>` form maps a global RAM/data
        // Symbol into this (functional) local scope (C++ `IfcMapaddress`,
        // `getScopeLocal()->addSymbol`).  `Scope::addMap` marks it addr-tied, but it
        // lives in the ram space, not the stack space scanned above — so it is lost
        // on the kuna IR rebuild unless carried here.  Append the addr-tied mapped
        // symbols from every NON-stack space so they are re-seeded into the rebuilt
        // local scope and `linkSymbol` binds the global Varnode's name.
        out.extend(self.db.scope_nonstack_addrtied_specs(self.scope, idx));
        out
    }

    /// The usepoint-scoped Symbol specs in this scope (across all spaces) — the
    /// register-storage locals a `type varnode %REG(pc) <type> <name>` directive
    /// creates, whose `SymbolEntry::inUse` is restricted to a code address range.
    /// Carried (WITH the use address) across the kuna console's IR rebuild so the
    /// rebuilt-IR `linkSymbol` query still binds them at the read they scope
    /// (see [`crate::database::Database::scope_usepoint_symbol_specs`]).
    pub fn usepoint_symbol_specs(
        &self,
    ) -> Vec<(String, Rc<Datatype>, Address, uint4, Address, bool)> {
        self.db.scope_usepoint_symbol_specs(self.scope)
    }

    /// The `(start, type, type_locked)` hints for every Symbol mapped into this
    /// scope's space, in EntryMap list order (C++ `MapState::gatherSymbols`).
    pub fn gather_symbol_hints(&self) -> Vec<(uintb, Rc<Datatype>, bool)> {
        let idx = self.space.get_index() as usize;
        self.db.scope_space_symbol_hints(self.scope, idx)
    }

    /// Resolve the mapped Symbol covering a Varnode for the naming pass (C++
    /// `Funcdata::linkSymbol` -> `HighVariable::getSymbol()` -> the Symbol's
    /// display name).  Returns `(display_name, symbol_offset, symbol_type)` where
    /// `symbol_offset` is the byte offset of the access within the Symbol (0 for a
    /// whole-symbol / scalar access; > 0 for an array/struct member).  `None` when
    /// no Symbol overlaps.
    pub fn name_for_varnode(
        &self,
        addr: &Address,
        size: int4,
    ) -> Option<(String, int4, Option<Rc<Datatype>>)> {
        let eref = self.db.find_overlap(self.scope, addr, size)?;
        let entry = self.db.entry(self.scope, eref);
        let entry_addr_off = entry.get_addr().get_offset();
        let entry_off = entry.get_offset();
        let sym = entry.symbol;
        let symbol = self.db.symbol(sym);
        // symbol_offset = (access_addr - entry_addr) + entry_offset.
        let sym_off = (addr.get_offset().wrapping_sub(entry_addr_off) as int4).wrapping_add(entry_off);
        Some((symbol.get_display_name().to_string(), sym_off, symbol.dtype.clone()))
    }

    /// C++ `ActionNameVars`'s namerec rename (coreaction.cc:3087-3094): if the
    /// Symbol covering `(addr, size)` has an undefined name and the access covers
    /// the whole Symbol (the high `getSymbolOffset() < 0` gate), rename it to the
    /// angr default (`v<base++>`, via `Scope::buildDefaultName`'s `kunaAngrNaming`
    /// stack/local arm) and return the new name.  Returns `None` when there is no
    /// covering Symbol; returns the existing (already-defined) name unchanged
    /// otherwise.  `base` is the running `int4 base` of `ActionNameVars`.
    pub fn resolve_default_name(
        &mut self,
        addr: &Address,
        size: int4,
        base: &mut int4,
    ) -> Option<(String, int4, Option<Rc<Datatype>>)> {
        self.resolve_default_name_override(addr, size, base, None)
    }

    /// `resolve_default_name` with an optional callee-parameter-name override
    /// (`ActionNameVars::lookForFuncParamNames`, coreaction.cc:2992): when the
    /// covered Symbol's name is undefined and the high represents the whole symbol,
    /// rename it to `makeNameUnique(override)` (the callee parameter name) INSTEAD of
    /// the `vN` default, and do NOT consume `base`.  With `override == None` this is
    /// byte-identical to the plain `resolve_default_name` `vN` path.
    pub fn resolve_default_name_override(
        &mut self,
        addr: &Address,
        size: int4,
        base: &mut int4,
        override_name: Option<&str>,
    ) -> Option<(String, int4, Option<Rc<Datatype>>)> {
        let eref = self.db.find_overlap(self.scope, addr, size)?;
        let (sym, entry_addr_off, entry_off, entry_size) = {
            let entry = self.db.entry(self.scope, eref);
            (entry.symbol, entry.get_addr().get_offset(), entry.get_offset(), entry.get_size())
        };
        let sym_off = (addr.get_offset().wrapping_sub(entry_addr_off) as int4).wrapping_add(entry_off);
        // C++ adds the Varnode to `namerec` only when `sym->isNameUndefined() &&
        // high->getSymbolOffset() < 0` — i.e. an undefined name on a high that
        // represents the WHOLE symbol (not a member access).  Here the whole-symbol
        // access is `sym_off == 0 && size == entry_size` (the entire mapped entry).
        let undefined = self.db.symbol(sym).is_name_undefined();
        if undefined && sym_off == 0 && size == entry_size {
            match override_name {
                // lookForFuncParamNames: renameSymbol(makeNameUnique(namerec)) — the
                // callee parameter name wins over `vN` and does not consume `base`.
                Some(nm) => {
                    let newname = self.db.public_make_name_unique(self.scope, nm);
                    let _ = self.db.rename_symbol(sym, &newname);
                }
                None => {
                    // newname = scope->buildDefaultName(sym, base, vn) (angr `vN` arm).
                    let newname = format!("v{}", *base);
                    *base += 1;
                    // makeNameUnique then renameSymbol.
                    let _ = self.db.rename_symbol(sym, &newname);
                }
            }
        }
        let symbol = self.db.symbol(sym);
        Some((symbol.get_display_name().to_string(), sym_off, symbol.dtype.clone()))
    }

    /// C++ `ActionNameVars::linkSpacebaseSymbol`'s namerec entry + the end-of-`apply`
    /// rename (coreaction.cc:3016 + coreaction.cc:3087-3094): the spacebase pass
    /// links a `PTRSUB(spacebase, off)` `&symbol` reference even when the covering
    /// Symbol is name-undefined, recording `offVn` in `namerec`; `apply` then renames
    /// the shared Symbol to its `buildDefaultName` (`v<base++>`).  Because the same
    /// Symbol object backs both the reference high and any body member-access high,
    /// the single rename makes BOTH render the final `vN` name.
    ///
    /// Here `addr` is the resolved symbol-reference address (C++
    /// `sb->getAddress(...)`, the same address [`query_container_for_link`] consumes).
    /// The reference targets the WHOLE symbol — the C++ namerec gate is `offVn`'s
    /// `&symbol` reference (offset 0 into the Symbol), the spacebase analogue of
    /// `high->getSymbolOffset() < 0`.  Renames the *smallest containing* undefined
    /// Symbol whose base byte the reference addresses at offset 0, consuming `base`,
    /// and returns the new name; returns `None` when there is no containing Symbol or
    /// the reference is not a whole-symbol (offset-0) reference.  A symbol that is
    /// already named is left untouched (idempotent re-run).
    pub fn name_undefined_spacebase_symbol(
        &mut self,
        addr: &Address,
        base: &mut int4,
    ) -> Option<String> {
        // queryContainer(addr,1,Address()) — the same lookup as link_symbol_reference.
        let eref = self.db.find_container(self.scope, addr, 1, &Address::default())?;
        let (sym, entry_addr_off, entry_off) = {
            let entry = self.db.entry(self.scope, eref);
            (entry.symbol, entry.get_addr().get_offset(), entry.get_offset())
        };
        // sym_off = (addr - entry_addr) + entry_off.  The reference must address the
        // base of the Symbol (sym_off == 0) — the `&symbol` whole-symbol reference.
        let sym_off = (addr.get_offset().wrapping_sub(entry_addr_off) as int4).wrapping_add(entry_off);
        if sym_off != 0 {
            return None;
        }
        if !self.db.symbol(sym).is_name_undefined() {
            return None;
        }
        // newname = scope->buildDefaultName(sym, base, vn) (angr `vN` arm); then
        // scope->renameSymbol(sym, newname).
        let newname = format!("v{}", *base);
        *base += 1;
        let _ = self.db.rename_symbol(sym, &newname);
        Some(newname)
    }

    /// Query the *smallest containing* Symbol entry for the naming/linkSymbol pass
    /// — the C++ `localmap->queryProperties(vn->getAddr(), 1, usepoint, fl)` lookup
    /// of [`Funcdata::linkSymbol`](funcdata_varnode.cc:1190).  Unlike
    /// [`name_for_varnode`] / [`resolve_default_name`] (which use the loose
    /// `findOverlap`), C++ `linkSymbol` queries `queryProperties` with **size 1**
    /// (just the base byte), and `queryProperties` returns the *smallest containing*
    /// SymbolEntry via `findContainer` (`database.cc:1268-1285`).  The caller then
    /// runs `handleSymbolConflict` against this entry's `(addr,size)` to decide
    /// whether to reuse the Symbol or spawn a fresh dynamic Symbol (the `vN` lane).
    ///
    /// Returns `None` when no Symbol *contains* the base byte (then `linkSymbol`'s
    /// `else` arm creates a fresh local Symbol — the angr `vN` path the caller's
    /// `resolve_default_name`/`vN` tail already implements).
    /// Identity of the smallest containing SymbolEntry for a storage location —
    /// the kuna analog of C++ `Varnode::getSymbolEntry()` (the `mapentry` pointer)
    /// used by `PieceNode::isLeaf` / `RulePieceStructure::determineDatatype` to
    /// decide whether two Varnodes share a Symbol.
    ///
    /// C++ compares `SymbolEntry*` pointers; the merged W3/W4 Varnode carries no
    /// stored `mapentry`, so we re-derive it with the same containment query
    /// (`findContainer(addr, 1, usepoint)`, exactly as `linkSymbol` does) and key
    /// the entry by `(SymbolId, entry-base-offset, entry-size)`.  Two Varnodes that
    /// resolve to the same key share a `SymbolEntry`; `None` (no container) is
    /// treated as a distinct "no symbol" key by the callers.
    pub fn container_entry_key(
        &self,
        addr: &Address,
        usepoint: &Address,
    ) -> Option<(crate::database::SymbolId, uintb, int4)> {
        let eref = self.db.find_container(self.scope, addr, 1, usepoint)?;
        let entry = self.db.entry(self.scope, eref);
        Some((entry.symbol, entry.get_addr().get_offset(), entry.get_size()))
    }

    pub fn query_container_for_link(
        &self,
        addr: &Address,
        usepoint: &Address,
    ) -> Option<LinkEntryInfo> {
        // C++ `Funcdata::linkSymbol` (`funcdata_varnode.cc:1190`):
        //   queryProperties(vn->getAddr(), 1, vn->getUsePoint(*this), fl) -> findContainer.
        // The `usepoint` is the Varnode's use address (def-op address if written,
        // else `fd.getAddress()-1`).  `findContainer` consults `SymbolEntry::inUse`
        // (`database.cc:115`): an addr-tied / empty-`uselimit` Symbol is in use at
        // every code address (so its container hit is usepoint-independent — passing
        // an invalid usepoint or the real one is equivalent), but a register-storage
        // local with a non-empty `uselimit` range (e.g. the `type varnode %EAX(pc)`
        // directive's usepoint-scoped Symbol) only matches when the usepoint falls in
        // its range.  Threading the real usepoint here is what lets such a Symbol bind
        // at the register read it is scoped to (the `tmp` retstruct return Symbol).
        let eref = self.db.find_container(self.scope, addr, 1, usepoint)?;
        let entry = self.db.entry(self.scope, eref);
        let sym = entry.symbol;
        let entry_addr = entry.get_addr().clone();
        let entry_size = entry.get_size();
        let entry_addr_off = entry_addr.get_offset();
        let entry_off = entry.get_offset();
        let symbol = self.db.symbol(sym);
        let sym_off =
            (addr.get_offset().wrapping_sub(entry_addr_off) as int4).wrapping_add(entry_off);
        Some(LinkEntryInfo {
            symbol: sym,
            display_name: symbol.get_display_name().to_string(),
            sym_off,
            sym_type: symbol.dtype.clone(),
            entry_addr,
            entry_size,
            category: symbol.get_category(),
            is_name_undefined: symbol.is_name_undefined(),
            is_isolated: symbol.is_isolated(),
        })
    }

    /// Read `Symbol::isIsolated()` for a SymbolId in this scope (C++
    /// `sym->isIsolated()`).  Used by the merge's `bank_symbol_isolated` for a
    /// dynamic-hash / equate Symbol bound directly on a HighVariable. (kuna L2)
    pub fn symbol_isolated(&self, sym: crate::database::SymbolId) -> bool {
        self.db.symbol(sym).is_isolated()
    }

    /// The identity of the smallest containing SymbolEntry for the Phase-4
    /// encode-time symbol link (the same `findContainer` query as
    /// [`Self::query_container_for_link`], but returning the [`SymbolId`] plus
    /// the entry geometry the `HighVariable::setSymbol` offset rule needs):
    /// `(symbol, entry_addr, entry_size, entry_offset)`.
    pub fn container_symbol_link(
        &self,
        addr: &Address,
        usepoint: &Address,
    ) -> Option<(crate::database::SymbolId, Address, int4, int4)> {
        let eref = self.db.find_container(self.scope, addr, 1, usepoint)?;
        let entry = self.db.entry(self.scope, eref);
        Some((entry.symbol, entry.get_addr().clone(), entry.get_size(), entry.get_offset()))
    }

    /// Read a symbol's `(id, category)` pair for the `<high>` encode (C++
    /// `symbol->getId()` / `symbol->getCategory()`).
    pub fn symbol_id_and_category(&self, sym: crate::database::SymbolId) -> (u64, int4) {
        let s = self.db.symbol(sym);
        (s.get_id(), s.get_category())
    }

    /// Whether [`Self::encode`] will actually emit this symbol — the O(1)
    /// single-symbol form of [`Self::encodable_symbol_ids`], for the markup's
    /// `<vardecl symref>` (which is computed per declaration on every
    /// decompile, so it must not pay for building the whole id set).
    pub fn symbol_is_encodable(&self, sym: crate::database::SymbolId) -> bool {
        self.db.symbol_encodable(sym)
    }

    /// The symbol ids [`ScopeLocal::encode`] actually emits — the set a
    /// `<high symref>` may reference (see
    /// [`Database::encodable_symbol_ids`](crate::database::Database::encodable_symbol_ids)).
    pub fn encodable_symbol_ids(&self) -> std::collections::BTreeSet<u64> {
        self.db.encodable_symbol_ids(self.scope)
    }

    /// C++ `Funcdata::linkSymbol(nameRep)` (`funcdata_varnode.cc:1177`) for the
    /// on-demand naming of a CONCAT-tree ROOT in `linkProtoPartial`
    /// (`funcdata_varnode.cc:1164-1166`).  The root's name representative is an
    /// addr-tied/mapped storage location whose smallest containing Symbol is the
    /// whole structure the pieces feed (e.g. the unified `mypiece/8` stack symbol
    /// the `propagateSpacebaseRef` seed formed).
    ///
    /// Unlike [`query_container_for_link`] (a pure lookup) this also performs the
    /// `ActionNameVars::apply` rename (coreaction.cc:3088-3092): if the containing
    /// Symbol is still name-undefined it is renamed to its `buildDefaultName`
    /// (`v<base++>`, the angr arm) ONCE — every later piece/root resolving to the
    /// SAME Symbol object then reuses that single name, exactly as the C++
    /// `namerec`/`renameSymbol` pass renames the one shared Symbol once.  This is
    /// what makes all members of one struct render through a single `v1` (e.g.
    /// `v1.a`/`v1.b`/`v1.arr[i]`) rather than a fresh `vN` per CONCAT root.
    ///
    /// Returns `(display_name, sym_off, whole_symbol_type)` where `sym_off` is the
    /// root's byte offset within the containing Symbol (the `getFirstWholeMap`
    /// base the piece offsets are measured from) and `whole_symbol_type` is the
    /// Symbol's full data-type (the struct the field render walks).  `None` when no
    /// Symbol contains the base byte (the caller keeps the `vN`-allocator fallback).
    pub fn link_symbol_root(
        &mut self,
        addr: &Address,
        base: &mut int4,
    ) -> Option<(String, int4, Option<Rc<Datatype>>)> {
        let eref = self.db.find_container(self.scope, addr, 1, &Address::default())?;
        let (sym, entry_addr_off, entry_off) = {
            let entry = self.db.entry(self.scope, eref);
            (entry.symbol, entry.get_addr().get_offset(), entry.get_offset())
        };
        let sym_off =
            (addr.get_offset().wrapping_sub(entry_addr_off) as int4).wrapping_add(entry_off);
        // ActionNameVars::apply: rename the undefined-named Symbol to `vN` once.
        if self.db.symbol(sym).is_name_undefined() {
            let newname = format!("v{}", *base);
            *base += 1;
            let _ = self.db.rename_symbol(sym, &newname);
        }
        let symbol = self.db.symbol(sym);
        Some((symbol.get_display_name().to_string(), sym_off, symbol.dtype.clone()))
    }

    /// The category of the Symbol covering a storage location (C++
    /// `Symbol::getCategory` via `findOverlap`), or `None` when none overlaps.
    /// `function_parameter` (0) marks a high the body decl block must skip — C++
    /// `emitLocalVarDecls` emits only `no_category` symbols, so parameters render
    /// in the signature, not the body (`printc.cc:2336`).
    pub fn category_for_varnode(&self, addr: &Address, size: int4) -> Option<int4> {
        let eref = self.db.find_overlap(self.scope, addr, size)?;
        let entry = self.db.entry(self.scope, eref);
        Some(self.db.symbol(entry.symbol).get_category())
    }

    /// Is `nm` the name of a Symbol in this (local) function scope?  Backs the
    /// `useScope->isNameUsed(distinguishName, terminatingScope)` collision check of
    /// `Symbol::getResolutionDepth` (`database.cc:358`): the function's local scope is
    /// detached (no parent), so the walk inspects only this scope — exactly the
    /// "parameter/local name shadows a global" predicate that drives the `::`/`ns::`
    /// namespace qualifier (the `namespace` datatest's `::spam` / `a::spam`).
    pub fn local_name_used(&self, nm: &str) -> bool {
        self.db.is_name_used(self.scope, nm, None)
    }

    /// Disambiguate `nm` against this local scope (C++ `ScopeLocal::makeNameUnique`):
    /// returns `nm` if unused, else a suffixed variant.  Used by the
    /// `lookForFuncParamNames` rename for an unmapped local that reaches the `vN`
    /// tail (no covering Symbol to rename in place).
    pub fn make_local_name_unique(&self, nm: &str) -> String {
        self.db.public_make_name_unique(self.scope, nm)
    }

    /// The identity and whole data-type of the Symbol a storage location belongs to
    /// — the smallest SymbolEntry containing its **base byte** — or `None` when no
    /// Symbol covers it (a register/unique temp that never reached the local scope).
    ///
    /// The base-byte query is C++ `Funcdata::linkSymbol`'s
    /// `queryProperties(vn->getAddr(), 1, usepoint)` (`funcdata_varnode.cc:1190`),
    /// i.e. the lookup that decides which Symbol a Varnode *is* — a sub-field read of
    /// a 4-byte slot belongs to that slot's Symbol even though it does not span it.
    /// The declaration-emission analogue of [`Self::containing_category_for_varnode`]:
    /// C++ `emitScopeVarDecls` walks the ScopeLocal **Symbol** table (printc.cc:2667)
    /// and `emitVarDecl` declares `sym->getType()` (printc.cc:1719), so the printer
    /// needs the Symbol behind a storage location to emit one declaration for the
    /// several HighVariables that share it.  The uselimit is ignored for the same
    /// reason the category query ignores it: Symbol identity is a property of the
    /// Symbol, not of a single use-point, and the declaration is shared by every use.
    pub fn containing_symbol_for_storage(
        &self,
        addr: &Address,
    ) -> Option<(crate::database::SymbolId, Option<Rc<Datatype>>)> {
        let eref = self.db.find_container_ignore_usepoint(self.scope, addr, 1)?;
        let sym = self.db.entry(self.scope, eref).symbol;
        Some((sym, self.db.symbol(sym).dtype.clone()))
    }

    /// The category of the Symbol that **contains** a storage location (C++
    /// `Scope::queryProperties`/`findContainer` semantics, `database.cc:2128`), or
    /// `None` when no Symbol entry covers the *whole* `[addr, addr+size)` range.
    ///
    /// This is the discriminator the body decl block needs that bare `findOverlap`
    /// is too loose for: a `float8` local merged onto a register that *also* holds
    /// a narrower `float4` parameter (e.g. `XMM0` holding both the `float4 a` arg
    /// and a `float8` cast result) overlaps the parameter entry but is **not
    /// contained** by it (sizes differ), so it is its own `no_category` local and
    /// must still be declared — exactly the C++ `handleSymbolConflict` outcome
    /// (`funcdata_varnode.cc:1018`: a size/high conflict spawns a fresh dynamic
    /// Symbol rather than reusing the parameter).
    pub fn containing_category_for_varnode(&self, addr: &Address, size: int4) -> Option<int4> {
        // Containment-only (ignore the SymbolEntry uselimit/`in_use`): the
        // category is a property of the Symbol, not of a single use-point.  A
        // register `function_parameter` Symbol is mapped through
        // `restricted_usepoint` (entry-1, NOT addrtied — see
        // `ScopeLocal::add_param_symbol`), so the prior usepoint-keyed
        // `find_container(&Address::default())` (invalid usepoint ⇒ `in_use`
        // false for a non-addrtied entry) missed it and the printer wrongly
        // declared the register parameter as a body local.  C++
        // `emitScopeVarDecls` finds parameters by walking the Symbol table by
        // category, never by a usepoint query — this matches that.
        let eref = self.db.find_container_ignore_usepoint(self.scope, addr, size)?;
        let entry = self.db.entry(self.scope, eref);
        Some(self.db.symbol(entry.symbol).get_category())
    }

    /// C++ `Scope::queryContainer` (`database.cc:1251`) on the local stack scope,
    /// snapshotting the matching SymbolEntry's storage range and the owning
    /// Symbol's whole data-type into owned values.  Backs `RuleStringCopy`'s
    /// `data.getScopeLocal()->queryContainer(addr, size, usepoint)` (the
    /// COPY-into-array `StringSequence` driver, `constseq.cc:990`).
    ///
    /// Returns `None` when no Symbol *contains* `[addr, addr+size)` or the owning
    /// Symbol has no data-type yet.  The snapshot lets `StringSequence` walk the
    /// container array and build the typed destination pointer without holding a
    /// borrow on the `Database` across the (mutating) transform.
    pub fn query_container(
        &self,
        addr: &Address,
        size: int4,
        usepoint: &Address,
    ) -> Option<StringContainerEntry> {
        let (escope, eref) = self.db.query_container(self.scope, addr, size, usepoint)?;
        let entry = self.db.entry(escope, eref);
        let sym_type = self.db.symbol(entry.symbol).dtype.clone()?;
        Some(StringContainerEntry {
            addr: entry.get_addr().clone(),
            first: entry.get_first(),
            size: entry.get_size(),
            sym_type,
        })
    }

    /// Information about the Symbol overlapping a storage location, for
    /// `Funcdata::syncVarnodesWithSymbols` (C++ `lm->findOverlap` + `getAllFlags`/
    /// `getSize`/`getSizedType`/`getSymbol`).
    ///
    /// Returns `None` when no Symbol overlaps `(addr, size)`.  The sized type is
    /// computed via `SymbolEntry::getSizedType` (C++ `database.cc:152`) using the
    /// owning Symbol's data-type and the type factory; it is `None` when the
    /// entry does not contain the access (the C++ "overlapping but not containing"
    /// branch handles that by the size comparison the caller does).
    pub fn sync_overlap(
        &self,
        addr: &Address,
        size: int4,
        types: &dyn TypeFactory,
    ) -> Option<SyncOverlap> {
        let eref = self.db.find_overlap(self.scope, addr, size)?;
        let entry = self.db.entry(self.scope, eref);
        let sym = entry.symbol;
        let entry_size = entry.get_size();
        let entry_off = entry.get_offset();
        let entry_addr_off = entry.get_addr().get_offset();
        let extraflags = entry.extraflags;
        let symbol = self.db.symbol(sym);
        let all_flags = extraflags | symbol.get_flags();
        // C++ SymbolEntry::getSizedType (non-dynamic): off = (inaddr - addr) + offset.
        let sized = symbol.dtype.as_ref().and_then(|cur| {
            let off = (addr.get_offset().wrapping_sub(entry_addr_off) as int4)
                .wrapping_add(entry_off);
            types.get_exact_piece(Rc::clone(cur), off, size).ok().flatten()
        });
        Some(SyncOverlap { all_flags, entry_size, sized_type: sized, symbol_id: sym, extraflags })
    }

    /// The `ActionInferTypes::buildLocaltypes` type-locked-symbol seed (C++
    /// `coreaction.cc:5275-5281`).  When a (non-type-locked) Varnode at
    /// `(addr, size)` is covered by a SymbolEntry whose owning Symbol *is*
    /// type-locked, the local data-type is seeded from the exact piece of the
    /// Symbol's type at the access offset, rather than floating from the local
    /// def/use flow (`Varnode::getLocalType`).
    ///
    /// Returns:
    /// * `Some(ct)` — the seeded exact-piece type (the caller adopts it),
    /// * `None` — no type-locked covering Symbol *or* the piece resolved to
    ///   null/`TYPE_UNKNOWN` (the caller falls through to `getLocalType`, i.e.
    ///   "let the data-type float even though the parent symbol is type-locked").
    ///
    /// `vn->getSymbolEntry()` is the SymbolEntry `linkSymbol`/`coverVarnodes`
    /// cache on the Varnode; here it is resolved freshly via `findOverlap`
    /// (`queryProperties`), the exact lookup that cache mirrors, so the seed is
    /// available on every InferTypes round without depending on the link pass
    /// having already run.
    pub fn build_localtype_seed(
        &self,
        addr: &Address,
        size: int4,
        types: &dyn TypeFactory,
    ) -> Option<Rc<Datatype>> {
        let eref = self.db.find_overlap(self.scope, addr, size)?;
        let entry = self.db.entry(self.scope, eref);
        let entry_off = entry.get_offset();
        let entry_addr_off = entry.get_addr().get_offset();
        let sym = entry.symbol;
        let symbol = self.db.symbol(sym);
        // entry->getSymbol()->isTypeLocked()
        if !symbol.is_type_locked() {
            return None;
        }
        let sym_type = Rc::clone(symbol.dtype.as_ref()?);
        let cur_off = (addr.get_offset().wrapping_sub(entry_addr_off) as int4).wrapping_add(entry_off);
        let ct = types.get_exact_piece(sym_type, cur_off, size).ok().flatten()?;
        // TYPE_UNKNOWN piece: let the type float.
        if ct.get_metatype() == crate::dtype::type_metatype::TYPE_UNKNOWN {
            return None;
        }
        Some(ct)
    }

    /// C++ `TypeSpacebase::getSubType` (`type.cc:3411-3433`), realized against this
    /// local scope's symbol table.
    ///
    /// The C++ `TypeSpacebase` carries a `glb` + a `localframe` address; `getMap()`
    /// resolves that frame to the owning function's `ScopeLocal` (which is exactly
    /// `self` here — the spacebase type seeded by `Funcdata::spacebase` uses
    /// `getAddress()` as the localframe).  Resolve the byte `off` to a stack
    /// address (`glb->resolveConstant`, which for the stack space is just
    /// `wrapOffset(byteToAddress·addressToByte)` = `wrapOffset(off)`), look up the
    /// smallest containing Symbol, and return its type with the offset *into* that
    /// Symbol.  When no Symbol contains the address, the C++ returns
    /// `getBase(1, TYPE_UNKNOWN)` with `newoff = 0` — a NON-null type, which is why
    /// `hasMatchingSubType` always succeeds off a spacebase pointer.
    pub fn spacebase_get_sub_type(
        &self,
        off: int8,
        types: &dyn TypeFactory,
    ) -> KunaResult<(Rc<Datatype>, int8)> {
        // addrOff = byteToAddress(off, wordSize); addr = resolveConstant(...) for the
        // stack space reduces to Address(space, wrapOffset(off)).
        let word_size = self.space.get_word_size();
        let addr_off = AddrSpace::byte_to_address(off as u64, word_size);
        let addr_off = AddrSpace::address_to_byte(addr_off, word_size);
        let addr_off = self.space.wrap_offset(addr_off);
        let addr = Address::new(Rc::clone(&self.space), addr_off);

        // smallest = scope->queryContainer(addr, 1, nullPoint)
        match self.db.find_container(self.scope, &addr, 1, &Address::new_invalid()) {
            Some(eref) => {
                let entry = self.db.entry(self.scope, eref);
                let entry_addr_off = entry.get_addr().get_offset();
                let entry_off = entry.get_offset();
                let sym = entry.symbol;
                let sym_type = self.db.symbol(sym).dtype.clone();
                // newoff = (addr - smallest.addr) + smallest.offset
                let newoff =
                    (addr_off.wrapping_sub(entry_addr_off) as int8) + entry_off as int8;
                match sym_type {
                    Some(t) => Ok((t, newoff)),
                    // A Symbol with no type is degenerate; fall back to UNKNOWN(1)
                    // (the C++ symbol always has a type here).
                    None => Ok((types.get_base(1, type_metatype::TYPE_UNKNOWN)?, 0)),
                }
            }
            None => Ok((types.get_base(1, type_metatype::TYPE_UNKNOWN)?, 0)),
        }
    }

    /// C++ `TypeSpacebase::nearestArrayedComponentForward` (`type.cc:3435-3480`),
    /// realized against this local scope's symbol table.
    ///
    /// Look up the Symbol containing `off`; if it starts exactly at the access
    /// (`getOffset() == 0`), defer to its type's `nearestArrayedComponentForward`.
    /// Otherwise advance to the next Symbol slot (`addr + 32` when the access is
    /// inside an offset Symbol, else past the current one) and probe there.  The
    /// returned `(distance, newoff, elSize)` matches the C++ out-params.
    pub fn spacebase_nearest_arrayed_forward(
        &self,
        off: int8,
        max: int8,
    ) -> (int8, int8, int8) {
        let word_size = self.space.get_word_size();
        let to_addr = |o: int8| -> Address {
            let a = AddrSpace::byte_to_address(o as u64, word_size);
            let a = AddrSpace::address_to_byte(a, word_size);
            Address::new(Rc::clone(&self.space), self.space.wrap_offset(a))
        };
        let addr = to_addr(off);
        let smallest =
            self.db.find_container(self.scope, &addr, 1, &Address::new_invalid());

        let mut el_size: int8 = 0;
        let next_addr: Address;
        match smallest {
            Some(eref) => {
                let entry = self.db.entry(self.scope, eref);
                let entry_off = entry.get_offset();
                if entry_off != 0 {
                    next_addr = &addr + 32;
                } else {
                    let sym = entry.symbol;
                    let symbol_type = self.db.symbol(sym).dtype.clone();
                    let entry_addr_off = entry.get_addr().get_offset();
                    let entry_size = entry.get_size();
                    let struct_off = addr.get_offset().wrapping_sub(entry_addr_off) as int8;
                    if let Some(st) = &symbol_type {
                        if let Ok((distance, _dummy, esz)) =
                            st.nearest_arrayed_component_forward(struct_off, max)
                        {
                            if distance >= 0 {
                                if distance > max {
                                    return (-1, off, 0);
                                }
                                return (distance, struct_off, esz);
                            }
                        }
                    }
                    let sz = AddrSpace::byte_to_address_int(entry_size as i64, word_size);
                    next_addr = entry.get_addr() + sz;
                }
            }
            None => {
                next_addr = &addr + 32;
            }
        }
        // Don't let the address wrap.
        if next_addr.get_offset() < addr.get_offset() {
            return (-1, off, 0);
        }
        let smallest2 =
            self.db.find_container(self.scope, &next_addr, 1, &Address::new_invalid());
        let eref2 = match smallest2 {
            Some(e) => e,
            None => return (-1, off, 0),
        };
        let entry2 = self.db.entry(self.scope, eref2);
        if entry2.get_offset() != 0 {
            return (-1, off, 0);
        }
        let sym2 = entry2.symbol;
        let symbol_type2 = self.db.symbol(sym2).dtype.clone();
        let entry2_addr_off = entry2.get_addr().get_offset();
        let newoff = addr.get_offset().wrapping_sub(entry2_addr_off) as int8;
        if let Some(st2) = &symbol_type2 {
            if let Ok((distance, _dummy, esz)) = st2.nearest_arrayed_component_forward(0, max) {
                el_size = esz;
                if distance >= 0 {
                    let distance = distance - newoff;
                    if distance > max {
                        return (-1, off, 0);
                    }
                    return (distance, newoff, el_size);
                }
            }
        }
        let _ = el_size;
        (-1, off, 0)
    }

    /// C++ `TypeSpacebase::nearestArrayedComponentBackward` (`type.cc:3482-3496`),
    /// realized against this local scope's symbol table.  Resolve the containing
    /// Symbol type (`getSubType`) then defer to its
    /// `nearestArrayedComponentBackward`.
    pub fn spacebase_nearest_arrayed_backward(
        &self,
        off: int8,
        max: int8,
        types: &dyn TypeFactory,
    ) -> (int8, int8, int8) {
        let (sub_type, newoff) = match self.spacebase_get_sub_type(off, types) {
            Ok(r) => r,
            Err(_) => return (-1, off, 0),
        };
        match sub_type.nearest_arrayed_component_backward(newoff, max) {
            Ok((distance, _dummy, esz)) if distance >= 0 => {
                if distance > max {
                    return (-1, off, 0);
                }
                (distance, newoff, esz)
            }
            _ => (-1, off, 0),
        }
    }

    /// C++ `ScopeLocal::buildVariableName` (`varmap.cc:548`): the stack-frame
    /// naming convention (`<TypeBase><Space>[XY]_<hexoffset>`) for an addr-tied,
    /// non-persistent Varnode in this scope's space whose offset lies in the
    /// function's local range; otherwise defer to the generic name builder
    /// (`ScopeInternal::buildVariableName`, the caller supplies the fallback).
    ///
    /// Returns `Some(name)` when the stack convention applies, `None` when the
    /// caller should fall through to the generic builder.
    pub fn build_variable_name_stack(
        &self,
        addr: &Address,
        ct: Option<&Datatype>,
        flags: uint4,
        type_name_base: &dyn Fn(&Datatype) -> String,
        in_local_range: bool,
    ) -> Option<String> {
        use crate::varnode::varnode_flags;
        let in_space = addr
            .get_space()
            .map(|s| s.get_index() == self.space.get_index())
            .unwrap_or(false);
        if (flags & (varnode_flags::addrtied | varnode_flags::persist)) == varnode_flags::addrtied
            && in_space
            && in_local_range
        {
            let mut start: int8 =
                AddrSpace::byte_to_address(addr.get_offset(), self.space.get_word_size()) as int8;
            start = sign_extend(start, addr.get_addr_size() * 8 - 1);
            if self.stack_grows_negative {
                start = start.wrapping_neg();
            }
            let mut s = String::new();
            if let Some(c) = ct {
                s.push_str(&type_name_base(c));
            }
            let mut spacename = capitalize_first_local(self.space.get_name());
            if start <= 0 {
                spacename.push('X'); // local stack allocated by caller
                start = start.wrapping_neg();
            } else if self.min_param_offset < self.max_param_offset {
                let unusual = if self.stack_grows_negative {
                    addr.get_offset() < self.min_param_offset
                } else {
                    addr.get_offset() > self.max_param_offset
                };
                if unusual {
                    spacename.push('Y'); // unusual region of stack
                }
            }
            s.push_str(&spacename);
            s.push('_');
            s.push_str(&format!("{:x}", start as uintb));
            return Some(self.db.public_make_name_unique(self.scope, &s));
        }
        None
    }
}

/// Uppercase the first character of `name` (C++ `spacename[0] = toupper(...)`),
/// matching the `database.rs` `capitalize_first` helper used by the generic
/// name builder.
fn capitalize_first_local(name: &str) -> String {
    let mut cs = name.chars();
    match cs.next() {
        None => String::new(),
        Some(c) => c.to_ascii_uppercase().to_string() + cs.as_str(),
    }
}

// ===========================================================================
// Recommendation records (varmap.hh:36-82)
// ===========================================================================

/// A symbol name recommendation with its associated storage location (C++
/// `NameRecommend`, `varmap.hh:36-50`).
#[derive(Debug, Clone)]
pub struct NameRecommend {
    /// The starting address of the storage location (C++ `addr`).
    pub addr: Address,
    /// The code address at the point of use (C++ `useaddr`).
    pub useaddr: Address,
    /// An optional/recommended size for the variable being stored (C++ `size`).
    pub size: int4,
    /// The local symbol name recommendation (C++ `name`).
    pub name: String,
    /// Id associated with the original Symbol (C++ `symbolId`).
    pub symbol_id: u64,
}

impl NameRecommend {
    /// C++ constructor `NameRecommend(ad,use,sz,nm,id)` (`varmap.hh:43-44`).
    pub fn new(addr: Address, useaddr: Address, size: int4, name: String, symbol_id: u64) -> NameRecommend {
        NameRecommend { addr, useaddr, size, name, symbol_id }
    }
}

/// A name recommendation for a particular dynamic storage location (C++
/// `DynamicRecommend`, `varmap.hh:56-68`).
#[derive(Debug, Clone)]
pub struct DynamicRecommend {
    /// Use point of the Symbol (C++ `usePoint`).
    pub use_point: Address,
    /// Hash encoding the Symbol's environment (C++ `hash`).
    pub hash: u64,
    /// The local symbol name recommendation (C++ `name`).
    pub name: String,
    /// Id associated with the original Symbol (C++ `symbolId`).
    pub symbol_id: u64,
}

impl DynamicRecommend {
    /// C++ constructor `DynamicRecommend(addr,h,nm,id)` (`varmap.hh:62`).
    pub fn new(use_point: Address, hash: u64, name: String, symbol_id: u64) -> DynamicRecommend {
        DynamicRecommend { use_point, hash, name, symbol_id }
    }
}

/// A data-type for a storage location when there is no Symbol yet (C++
/// `TypeRecommend`, `varmap.hh:74-82`).
#[derive(Clone)]
pub struct TypeRecommend {
    /// Storage address of the Varnode (C++ `addr`).
    pub addr: Address,
    /// Data-type to assign to the Varnode (C++ `dataType`).
    pub data_type: Rc<Datatype>,
}

impl TypeRecommend {
    /// C++ constructor `TypeRecommend(ad,dt)` (`varmap.hh:78`).
    pub fn new(addr: Address, data_type: Rc<Datatype>) -> TypeRecommend {
        TypeRecommend { addr, data_type }
    }
}


// ===========================================================================
// MapState (varmap.hh:174-203, varmap.cc:864-1249)
// ===========================================================================

/// A container for hints about the data-type layout of an address space (C++
/// `MapState`, `varmap.hh:174-203`).
///
/// Hints (`RangeHint`) are collected from Varnodes, symbols, or pointer
/// references, then sorted and iterated.  The owned `RangeHint`s are stored by
/// value (the C++ owns `RangeHint *` on the heap and `delete`s in the
/// destructor; the arena-free Rust port owns them in the `Vec`).
pub struct MapState {
    /// The address space being analyzed (C++ `spaceid`).
    spaceid: Rc<AddrSpace>,
    /// The subset of ranges, within the whole address space, to analyze (C++
    /// `range`).
    range: kuna_base::address::RangeList,
    /// The list of collected RangeHints (C++ `maplist`).
    maplist: Vec<RangeHint>,
    /// The current iterator position into the RangeHints (C++ `iter`).
    iter: usize,
    /// The default data-type to use for RangeHints (C++ `defaultType`).
    default_type: Rc<Datatype>,
    /// A collection of pointer Varnodes into our address space (C++ `checker`).
    checker: AliasChecker,
}

impl MapState {
    /// C++ constructor `MapState(spc,rn,pm,dt)` (`varmap.cc:864-879`).
    ///
    /// Clears any of the parameter ranges `pm` out of the analysis range `rn`.
    pub fn new(
        spc: Rc<AddrSpace>,
        rn: &kuna_base::address::RangeList,
        pm: &kuna_base::address::RangeList,
        dt: Rc<Datatype>,
    ) -> MapState {
        let mut range = rn.clone();
        // Clear possible input symbols.
        for r in pm.iter() {
            range.remove_range(Rc::clone(r.get_space()), r.get_first(), r.get_last());
        }
        MapState { spaceid: spc, range, maplist: Vec::new(), iter: 0, default_type: dt, checker: AliasChecker::new() }
    }

    /// Add a hint to the collection (C++ `addRange`, `varmap.cc:896-919`).
    ///
    /// A specific range of bytes is described for the hint.  A `None`/zero-size
    /// data-type falls back to the default.  Hints outside the analysis range
    /// are dropped.
    fn add_range(&mut self, st: uintb, ct: Option<Rc<Datatype>>, fl: uint4, rt: RangeType, hi: int4) {
        let ct = match ct {
            Some(c) if c.get_size() != 0 => c,
            _ => Rc::clone(&self.default_type),
        };
        let sz = ct.get_size();
        if !self.range.in_range(&Address::new(Rc::clone(&self.spaceid), st), sz) {
            return;
        }
        // intb sst = byteToAddress(st); sign_extend; addressToByte
        let word_size = self.spaceid.get_word_size();
        let addr_size = self.spaceid.get_addr_size();
        let mut sst: intb = AddrSpace::byte_to_address(st, word_size) as intb;
        sst = sign_extend(sst, (addr_size as int4) * 8 - 1);
        sst = AddrSpace::address_to_byte_int(sst, word_size); // addressToByte on a signed value
        self.maplist.push(RangeHint::new(st, sz, sst, ct, fl, rt, hi));
    }

    /// Add a fixed reference to a specific data-type (C++ `addFixedType`,
    /// `varmap.cc:926-956`).
    ///
    /// If the data-type is an array, partial struct, or partial union, the
    /// reference may be added as \e open.
    fn add_fixed_type(&mut self, start: uintb, ct: Rc<Datatype>, flags: uint4, types: &dyn TypeFactory) {
        let meta = ct.get_metatype();
        if meta == type_metatype::TYPE_PARTIALSTRUCT {
            // TypePartialStruct: getParent() is the container, getOffset() the offset.
            let parent = ct.get_partial_base().unwrap_or_else(|| Rc::clone(&ct));
            let offset = ct.get_partial_offset().unwrap_or(0);
            let pmeta = parent.get_metatype();
            if pmeta == type_metatype::TYPE_STRUCT && offset == 0 {
                // Initial fields of a TYPE_STRUCT moved here: treat as open.
                self.add_range(start, Some(Rc::clone(&parent)), 0, RangeType::Open, -1);
            } else if pmeta == type_metatype::TYPE_ARRAY {
                // Elements of an array moved here.
                if let Some(base) = parent.get_array_base() {
                    if base.get_metatype() != type_metatype::TYPE_UNKNOWN {
                        self.add_range(start, Some(base), 0, RangeType::Open, -1);
                    }
                }
            }
            // If the Varnode is a constant COPY, generate a fixed reference too.
            if flags != 0 {
                if let Ok(unk) = types.get_base(ct.get_size(), type_metatype::TYPE_UNKNOWN) {
                    self.add_range(start, Some(unk), flags, RangeType::Fixed, -1);
                }
            }
        } else if meta == type_metatype::TYPE_PARTIALUNION {
            // TypePartialUnion: getOffset(); getParentUnion() is the container.
            let offset = ct.get_partial_offset().unwrap_or(0);
            if offset == 0 {
                if let Some(parent_union) = ct.get_partial_base() {
                    self.add_range(start, Some(parent_union), 0, RangeType::Open, -1);
                }
            }
        } else {
            self.add_range(start, Some(ct), flags, RangeType::Fixed, -1);
        }
    }

    /// Decide on a data-type for RangeHints at the same address (C++
    /// `reconcileDatatypes`, `varmap.cc:960-996`).
    ///
    /// Assuming a sorted list, from among a sequence of RangeHints with the
    /// same `(start, size, flags)`, select the most specific data-type.  Set
    /// all of them to use that data-type, and eliminate duplicates.
    ///
    /// Order-determining: this walks the `stable_sort`ed `maplist` and is part
    /// of the layout output.
    fn reconcile_datatypes(&mut self) -> KunaResult<()> {
        let old = std::mem::take(&mut self.maplist);
        let mut new_list: Vec<RangeHint> = Vec::with_capacity(old.len());

        let mut start_pos: usize = 0;
        // startHint/startDatatype track the current run.
        let mut start_hint_start = old[0].start;
        let mut start_hint_size = old[0].size;
        let mut start_hint_flags = old[0].flags;
        let mut start_datatype = Rc::clone(&old[0].type_);
        new_list.push(old[0].clone());

        let mut cur_pos = 1usize;
        while cur_pos < old.len() {
            let cur_hint = &old[cur_pos];
            cur_pos += 1;
            if cur_hint.start == start_hint_start
                && cur_hint.size == start_hint_size
                && cur_hint.flags == start_hint_flags
            {
                let cur_datatype = Rc::clone(&cur_hint.type_);
                if cur_datatype.type_order(&start_datatype)? < 0 {
                    // Take the most specific variant of data-type
                    start_datatype = cur_datatype;
                }
                if cur_hint.compare(new_list.last().unwrap()) != 0 {
                    new_list.push(cur_hint.clone()); // Keep if otherwise different
                }
                // else: drop (the C++ `delete`s the heap RangeHint)
            } else {
                while start_pos < new_list.len() {
                    new_list[start_pos].type_ = Rc::clone(&start_datatype);
                    start_pos += 1;
                }
                start_hint_start = cur_hint.start;
                start_hint_size = cur_hint.size;
                start_hint_flags = cur_hint.flags;
                start_datatype = Rc::clone(&cur_hint.type_);
                new_list.push(cur_hint.clone());
            }
        }
        while start_pos < new_list.len() {
            new_list[start_pos].type_ = Rc::clone(&start_datatype);
            start_pos += 1;
        }
        self.maplist = new_list;
        Ok(())
    }

    /// Sort the collection and add a special terminating RangeHint (C++
    /// `initialize`, `varmap.cc:1063-1082`).
    ///
    /// Returns `true` if the collection isn't empty (and iteration can begin).
    /// The `stable_sort` here and the subsequent `reconcileDatatypes` fix the
    /// final layout order.
    pub fn initialize(&mut self) -> KunaResult<bool> {
        // Enforce boundaries of local variables
        let last = match self.range.get_last_signed_range(&self.spaceid) {
            Some(r) => r.get_last(),
            None => return Ok(false),
        };
        if self.maplist.is_empty() {
            return Ok(false);
        }
        let high = self.spaceid.wrap_offset(last.wadd(1));
        let word_size = self.spaceid.get_word_size();
        let addr_size = self.spaceid.get_addr_size();
        let mut sst: intb = AddrSpace::byte_to_address(high, word_size) as intb;
        sst = sign_extend(sst, (addr_size as int4) * 8 - 1);
        sst = AddrSpace::address_to_byte_int(sst, word_size);
        // Add extra range to bound any final open entry
        self.maplist.push(RangeHint::new(
            high,
            1,
            sst,
            Rc::clone(&self.default_type),
            0,
            RangeType::Endpoint,
            -2,
        ));

        // stable_sort(maplist, compareRanges)
        self.maplist.sort_by(RangeHint::compare_ranges);
        self.reconcile_datatypes()?;
        self.iter = 0;
        Ok(true)
    }

    /// Get the current RangeHint in the collection (C++ `next`,
    /// `varmap.hh:201`).
    pub fn next(&self) -> &RangeHint {
        &self.maplist[self.iter]
    }

    /// Advance the iterator; return `true` if another hint is available (C++
    /// `getNext`, `varmap.hh:202`).
    pub fn get_next(&mut self) -> bool {
        self.iter += 1;
        self.iter != self.maplist.len()
    }

    /// Sort the alias starting offsets (C++ `sortAlias`, `varmap.hh:196`).
    pub fn sort_alias(&mut self) {
        self.checker.sort_alias();
    }

    /// Get the list of alias starting offsets (C++ `getAlias`,
    /// `varmap.hh:197`).
    pub fn get_alias(&self) -> &[uintb] {
        self.checker.get_alias()
    }

    /// Add Symbol information as hints to the collection (C++ `gatherSymbols`,
    /// `varmap.cc:1044-1059`).
    ///
    /// `symbols` is the pre-extracted `(start_offset, type, type_locked)` for
    /// each `SymbolEntry` in the per-space rangemap.  The C++ reads these off
    /// the `EntryMap` list (`begin_list`/`end_list`); the iteration order is
    /// the rangemap's, which the caller preserves.
    pub fn gather_symbols(&mut self, symbols: &[(uintb, Rc<Datatype>, bool)]) {
        for (start, ct, type_locked) in symbols {
            let flags = if *type_locked { TYPELOCK } else { 0 };
            self.add_range(*start, Some(Rc::clone(ct)), flags, RangeType::Fixed, -1);
        }
    }

    /// Get a mutable handle to the alias checker (so the driver can run the
    /// access-driven gather; C++ `MapState::gatherOpen` calls `checker.gather`).
    pub fn checker_mut(&mut self) -> &mut AliasChecker {
        &mut self.checker
    }

    /// Append a fixed-type hint (the public entry the externally-driven
    /// `gatherVarnodes` uses; C++ `addFixedType` is private but is the only
    /// thing `gatherVarnodes` calls into `MapState`).
    pub fn add_fixed_type_pub(
        &mut self,
        start: uintb,
        ct: Rc<Datatype>,
        flags: uint4,
        types: &dyn TypeFactory,
    ) {
        self.add_fixed_type(start, ct, flags, types);
    }

    /// Append an open/range hint directly (the public entry `gatherOpen` and
    /// `addGuard` use; both are `MapState` methods calling the private
    /// `addRange`).
    pub fn add_range_pub(&mut self, st: uintb, ct: Option<Rc<Datatype>>, fl: uint4, rt: RangeType, hi: int4) {
        self.add_range(st, ct, fl, rt, hi);
    }
}

#[cfg(test)]
mod tests;
