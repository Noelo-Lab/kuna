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
//! ## Cross-wave seams
//!
//! `ScopeLocal` is a `ScopeInternal` subclass in C++ (`varmap.hh:212`) and is
//! wired tightly to `Funcdata` (the Varnode location/def sets, the spacebase
//! input, the load guards, warning emission, varnode remapping) and to the
//! symbol `Database` (category clearing, `addSymbol`, `findOverlap`,
//! `makeNameUnique`, attribute setting).  At this wave boundary the
//! `Funcdata`/`Database` surfaces those methods need are largely still seams
//! (the descend list, `findSpacebaseInput`, `beginLoc`/`beginDef`,
//! `getLoadGuards`, `warningHeader`, `remapVarnode`, `clearUnlockedCategory`,
//! `setCategory`, the per-scope `EntryMap` iteration).  The self-contained core
//! is ported and unit-tested here; the `Funcdata`/`Database`-bound methods are
//! transcribed against documented [`ScopeLocalSeam`] hooks and otherwise
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
            // if (meta != TYPE_ARRAY || ((TypeArray *)a->type)->getBase()->getMetatype() != TYPE_UNKNOWN)
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
        // if (b->sstart+b->size-1 <= sstart+size-1) return true;
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

        // return (0 > type->typeOrder(*b->type));  -- Prefer the more specific
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
/// (ADR 0001) these are [`crate::seams::VarnodeId`]s.  `index` is `None` for
/// the C++ `(Varnode *)0`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AddBase {
    /// The Varnode holding the base pointer (C++ `base`).
    pub base: crate::seams::VarnodeId,
    /// The index value or NULL (C++ `index`).
    pub index: Option<crate::seams::VarnodeId>,
}

impl AddBase {
    /// C++ constructor `AddBase(Varnode *b,Varnode *i)` (`varmap.hh:143`).
    pub fn new(base: crate::seams::VarnodeId, index: Option<crate::seams::VarnodeId>) -> AddBase {
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
/// SEAM: the C++ `gatherInternal`/`gatherAdditiveBase` walk the Varnode
/// descend graph and call `Funcdata::findSpacebaseInput`, neither of which is
/// available at this wave boundary.  The boundary-derivation, offset-gathering
/// (`gatherOffset`), alias sort, and the `hasLocalAlias` decision are ported in
/// full; the graph walk routes through [`AliasGatherSeam`].
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
        seam: &mut dyn AliasGatherSeam,
    ) {
        let dir = if spc.stack_grows_negative() { 1 } else { -1 };
        self.space = Some(Rc::clone(&spc));
        self.calculated = false; // Defer calculation
        self.add_base.clear();
        self.alias.clear();
        self.direction = dir; // direction == 1 for normal negative stack growth
        self.derive_boundaries(bounds);
        if !defer {
            self.gather_internal(seam);
        }
    }

    /// Run through Varnodes looking for pointers into the stack (C++
    /// `gatherInternal`, `varmap.cc:660-684`).
    ///
    /// SEAM: `gatherAdditiveBase` (the Varnode descend walk) and the
    /// spacebase-input lookup come from [`AliasGatherSeam`].  The
    /// offset-conversion and `aliasBoundary` accumulation are transcribed
    /// exactly.
    fn gather_internal(&mut self, seam: &mut dyn AliasGatherSeam) {
        self.calculated = true;
        self.alias_boundary = self.local_extreme;
        let space = match &self.space {
            Some(s) => Rc::clone(s),
            None => return,
        };
        let spacebase = match seam.find_spacebase_input(&space) {
            Some(vn) => vn,
            None => return, // No possible alias
        };

        seam.gather_additive_base(spacebase, &mut self.add_base);
        // C++ iterates addBase, computing one alias offset per AddBase.
        let word_size = space.get_word_size();
        let entries: Vec<crate::seams::VarnodeId> = self.add_base.iter().map(|ab| ab.base).collect();
        for base in entries {
            let mut offset = seam.gather_offset(base);
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
        seam: &mut dyn AliasGatherSeam,
    ) -> bool {
        let (vn_space, vn_offset) = match vn {
            Some(v) => v,
            None => return false,
        };
        if !self.calculated {
            self.gather_internal(seam);
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
/// SEAM(W4): the C++ reads `proto.hasModel()` and the first local / last param
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
/// SEAM(W7): the descend list and the spacebase input are `Funcdata` surfaces
/// not present at this boundary.  `Funcdata` will implement this trait, at
/// which point the alias gather runs against the real IR; the offset/sort/
/// boundary logic above is already final.
pub trait AliasGatherSeam {
    /// C++ `Funcdata::findSpacebaseInput(space)` — the input Varnode that is the
    /// stack pointer for the space, if any.
    fn find_spacebase_input(&self, space: &Rc<AddrSpace>) -> Option<crate::seams::VarnodeId>;
    /// C++ `AliasChecker::gatherAdditiveBase(startvn, addbase)` — collect the
    /// roots of every additive expression tree rooted at `startvn`.
    fn gather_additive_base(&mut self, startvn: crate::seams::VarnodeId, addbase: &mut Vec<AddBase>);
    /// C++ `AliasChecker::gatherOffset(vn)` — the constant portion of the sum
    /// the given Varnode is the result of.
    fn gather_offset(&mut self, vn: crate::seams::VarnodeId) -> uintb;
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
/// the `glb` [`ArchHandle`](crate::seams::ArchHandle) the `Funcdata` holds, so
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
/// and `syncVarnodesWithSymbols` remain a documented seam (LOSS-109): the
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
        })
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

    /// C++ `ScopeInternal::setAttribute` reached via the `map` commands
    /// (`sym->getScope()->setAttribute(sym, namelock|typelock)`).
    pub fn set_attribute(&mut self, sym: crate::database::SymbolId, attr: uint4) {
        self.db.set_attribute(sym, attr);
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
        self.db.scope_space_symbol_specs(self.scope, idx)
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
        Some(SyncOverlap { all_flags, entry_size, sized_type: sized, symbol_id: sym })
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
        // for(pmiter : pm) range.removeRange(...)  -- Clear possible input symbols
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
        // if (ct == null || ct->getSize() == 0) ct = defaultType;
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
    /// seam-bound gather; C++ `MapState::gatherOpen` calls `checker.gather`).
    pub fn checker_mut(&mut self) -> &mut AliasChecker {
        &mut self.checker
    }

    /// Append a fixed-type hint (the public entry the seam-driven
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
