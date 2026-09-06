//! Port of `decompiler/cpp/fspec.cc` lines ~1-4928 (W6, items `w6-s4-fspec-1`
//! and `w6-s4-fspec-2`): the **prototype-model subsystem**.
//!
//! `fspec-1` (lines ~1-2267) carries the **parameter-recovery foundation** —
//! the storage-model machinery that decides where parameters and return values
//! live and how data-flow trials map onto them.  `fspec-2` (lines ~2268-4928)
//! adds the **prototype-model layer** built on top of it:
//!
//!   - [`ProtoModel`] — a complete parameter-passing convention (input/output
//!     [`ParamListStandard`] wiring, `extrapop`, effect / likely-trash /
//!     internal-storage lists, local/param stack ranges, the side-effect
//!     lookups, and `assignParameterStorage`).
//!   - [`ScoreProtoModel`] — "goodness of fit" scoring of parameter trials
//!     against a [`ProtoModel`] (used by [`ProtoModelMerged::select_model`]).
//!   - [`ProtoModelMerged`] — a union of constituent [`ProtoModel`]s, with the
//!     effect/register intersection and `selectModel` resolution.
//!   - the parameter descriptions [`ProtoParameter`] / [`ParameterBasic`] and
//!     the storage interface [`ProtoStore`] / [`ProtoStoreInternal`].
//!   - [`FuncProto`] — a concrete function prototype (flag matrix, `copy`,
//!     `setModel`/`setInternal`, `updateAllTypes`, locked/unlocked semantics,
//!     and the model-delegating query surface).
//!
//! This file carries the storage-model machinery that decides where parameters
//! and return values live and how data-flow trials map onto them:
//!
//!   - [`ParamEntry`] — a contiguous (or joined) memory range usable to pass a
//!     single parameter (exclusion) or a sequence (alignment slots).  The
//!     endian-aware containment / justification / alignment / slot logic is the
//!     output-determining core (`containedBy`, `justifiedContain`,
//!     `getContainer`, `assumedExtension`, `getSlot`, `getAddrBySlot`).
//!   - [`ParamTrial`] — a putative parameter storage location seen during
//!     recovery, with the formal-parameter sort order (`operator<`,
//!     `fixedPositionCompare`).
//!   - [`ParamActive`] — the mutable collection of trials for one function, with
//!     the split/join/slot bookkeeping.
//!   - the [`ParamList`]-family struct [`ParamListStandard`] (tagged by
//!     [`ParamListKind`] for the `Standard`/`StandardOut`/`RegisterOut`/
//!     `Register`/`Merged` variants): the assignment walks (`assignMap`,
//!     `fillinMap`, `checkJoin`, ...).
//!   - the support structs [`ParameterPieces`], [`EffectRecord`],
//!     [`PrototypePieces`], and the marker [`AssignActionResponse`].
//!
//! ## Boundaries
//!
//! - `// STUB(w6-modelrules)` — [`ModelRule`] and the `AssignAction` machinery
//!   live in `modelrules.cc` (owned by a later item in this wave).  Until then
//!   `ParamListStandard` carries an **empty** `model_rules` list; the
//!   `assignAddress` walk therefore falls straight through to
//!   `assignAddressFallback` (the documented C++ behavior when there are no
//!   `<modelrule>`s), and the `<modelrule>`-affected output paths take the
//!   legacy fallback (`useFillinFallback == true`).  The `ModelRule` type is a
//!   local uninhabitable placeholder enum.
//! - `// STUB(w6-fspec-2)` — the back-pointer to the owning [`Architecture`]
//!   (C++ `ProtoModel::glb`) is **not** held: the kuna `Architecture` (W4) has
//!   no prototype-model registry / `types` / `defaultReturnAddr` / `getModel`
//!   yet.  Instead [`ProtoModel`] threads the [`AddrSpaceManager`] (for the
//!   stack space and float-extension construction) and the [`TypeFactory`]
//!   through the methods that need them; the registry-dependent paths
//!   (`decode`, `ProtoModelMerged::decode`, `setScope`) return a boundary error.
//!   [`PrototypePieces`] carries no `model` back-pointer for the same reason.
//!   The reserved [`FSPEC_SPACE_NAME`] is the only `FspecSpace` survivor (the
//!   full `FspecSpace`/`FuncCallSpecs` is `fspec-3`).
//! - `// STUB(W4)` — `decode`/`encode` paths reach fspec-owned marshaling
//!   ElementIds/AttributeIds (`<pentry>`, `<group>`, ...) and the
//!   `ProtoModel`/`Architecture` wiring that are not yet ported.  These methods
//!   return `Err(KunaError::lowlevel("STUB(W4) ..."))`; the pure-algorithm
//!   surfaces above do not depend on them and are exercised directly in tests
//!   via the `seed`/`push_entry` builder hooks.  The `FuncProto` input/output
//!   trial-update paths (`updateInputTypes`, `updateOutputTypes`, ...) and
//!   `ProtoStoreSymbol` reach `Funcdata`/`Varnode`/`Scope`/`Symbol` (W3/W4) and
//!   carry their own boundary errors.
//!
//! Integer model per ADR 0003: `uintb->u64`, `intb->i64`, `int4->i32`,
//! `uint4->u32`; arithmetic that the C++ relies on wrapping uses [`Wrap`].

use std::rc::Rc;

use kuna_base::address::{Address, RangeList};
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::space::{spacetype, AddrSpace, AddrSpaceManager, JoinRecord, VarnodeStorage};
use kuna_base::types::{int4, uint4, uintb, Wrap};
use kuna_num::opcodes::OpCode;
use kuna_num::pcoderaw::VarnodeData;

use crate::dtype::{metatype2typeclass, type_class, type_metatype, Datatype, TypeFactory};

// -----------------------------------------------------------------------------
// Wire marshaling ids owned by fspec (upstream numbers, fspec.cc:40-51;
// DECOMPILER scope — written by number, never registered on the SLEIGH
// registry; see the note in `substrate/funcdata_encode.rs`).
// -----------------------------------------------------------------------------

/// Marshaling element `<killedbycall>` (C++ `ELEM_KILLEDBYCALL`, fspec.cc:40).
pub const ELEM_KILLEDBYCALL: kuna_base::marshal::ElementId =
    kuna_base::marshal::ElementId::new("killedbycall", 162);
/// Marshaling element `<likelytrash>` (C++ `ELEM_LIKELYTRASH`, fspec.cc:41).
pub const ELEM_LIKELYTRASH: kuna_base::marshal::ElementId =
    kuna_base::marshal::ElementId::new("likelytrash", 163);
/// Marshaling element `<unaffected>` (C++ `ELEM_UNAFFECTED`, fspec.cc:51).
pub const ELEM_UNAFFECTED: kuna_base::marshal::ElementId =
    kuna_base::marshal::ElementId::new("unaffected", 173);
/// Marshaling element `<returnaddress>` re-export (defined in kuna-base,
/// upstream marshal.cc id 5).
pub use kuna_base::marshal::ELEM_RETURNADDRESS;

// =============================================================================
// AssignAction response codes (modelrules.hh:264-270)  // STUB(w6-modelrules)
// =============================================================================

/// The response code returned by `AssignAction::assignAddress` and the
/// `ParamListStandard` assignment helpers (C++ `enum` inside `AssignAction`,
/// `modelrules.hh:264-270`).
///
/// The discriminants are load-bearing: `ParamListStandard::assignMap` treats
/// `fail`/`no_assignment` as errors, and `ParamListStandardOut::assignMap`
/// branches on the three `hiddenret_*` codes.  // STUB(w6-modelrules)
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AssignActionResponse {
    /// Data-type is fully assigned.
    success = 0,
    /// Action could not be applied.
    fail = 1,
    /// Do not assign storage for this parameter.
    no_assignment = 2,
    /// Hidden return pointer as first input parameter.
    hiddenret_ptrparam = 3,
    /// Hidden return pointer in dedicated input register.
    hiddenret_specialreg = 4,
    /// Hidden return pointer, but no normal return.
    hiddenret_specialreg_void = 5,
}

/// A `<rule>` assignment rule (C++ `ModelRule`, `modelrules.hh`).
///
/// The real `ModelRule` family (and the `AssignAction` subclasses it drives)
/// lives in `modelrules.rs`; `ParamListStandard` now carries a populated
/// `Vec<ModelRule>` (decoded from the cspec `<rule>` elements, plus the synthetic
/// `pointermax` ConvertToPointer rule).  `assign_address` iterates these rules
/// first and only falls through to the fallback algorithm when every rule
/// returns `fail` — the C++ `ParamListStandard::assignAddress` behavior
/// (fspec.cc:783-792).  // (kuna) float-typeclass wave: ModelRule now wired.
pub use crate::modelrules::ModelRule;

// =============================================================================
// ParamEntry (fspec.hh:81-156, fspec.cc:62-596)
// =============================================================================

/// Boolean property flags for a [`ParamEntry`] (C++ anonymous enum,
/// `fspec.hh:84-96`).
pub mod param_entry_flags {
    use kuna_base::types::uint4;
    /// Big endian values are left justified within their slot.
    pub const FORCE_LEFT_JUSTIFY: uint4 = 1;
    /// Slots (for non-exclusion entries) are allocated in reverse order.
    pub const REVERSE_STACK: uint4 = 2;
    /// Values below max size are zero extended into this container.
    pub const SMALLSIZE_ZEXT: uint4 = 4;
    /// Values below max size are sign extended into this container.
    pub const SMALLSIZE_SEXT: uint4 = 8;
    // is_big_endian = 16 (commented out upstream)
    /// Values below max size are sign OR zero extended based on integer type.
    pub const SMALLSIZE_INTTYPE: uint4 = 0x20;
    /// Values smaller than max size are floating-point extended to full size.
    pub const SMALLSIZE_FLOATEXT: uint4 = 0x40;
    /// Extra checks during recovery on most significant portion of the double.
    pub const EXTRACHECK_HIGH: uint4 = 0x80;
    /// Extra checks during recovery on least significant portion of the double.
    pub const EXTRACHECK_LOW: uint4 = 0x100;
    /// This entry is grouped with other entries.
    pub const IS_GROUPED: uint4 = 0x200;
    /// Overlaps an earlier entry (and doesn't consume additional resource slots).
    pub const OVERLAPPING: uint4 = 0x400;
    /// Entry is first in its storage class.
    pub const FIRST_STORAGE: uint4 = 0x800;
}

/// Characterization of how a memory range relates to a [`ParamEntry`] (C++
/// anonymous enum, `fspec.hh:98-103`).  The discriminants are returned from
/// `ParamList::characterizeAsParam`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Containment {
    /// Range neither contains nor is contained by a ParamEntry.
    NoContainment = 0,
    /// ParamEntry contains range, but the range does not cover the least
    /// significant bytes.
    ContainsUnjustified = 1,
    /// ParamEntry contains range, which covers the least significant bytes.
    ContainsJustified = 2,
    /// ParamEntry is contained by the range.
    ContainedBy = 3,
}

/// A contiguous range of memory that can be used to pass a parameter or return
/// value (C++ `ParamEntry`, `fspec.hh:81-156`).
///
/// When `alignment == 0` the entry is *exclusive* (holds a single parameter);
/// otherwise it is a *resource* divided into alignment-sized slots.  A `joinrec`
/// is non-null when this entry is a logical variable built from joined pieces.
#[derive(Debug, Clone)]
pub struct ParamEntry {
    /// Boolean properties of the parameter (C++ `flags`).
    flags: uint4,
    /// Data-type storage class that this entry must match (C++ `type`).
    type_: type_class,
    /// Group(s) this entry belongs to (C++ `groupSet`).
    group_set: Vec<int4>,
    /// Address space containing the range (C++ `spaceid`).  `None` until the
    /// entry has been decoded/seeded.
    spaceid: Option<Rc<AddrSpace>>,
    /// Starting offset of the range (C++ `addressbase`).
    addressbase: uintb,
    /// Size of the range in bytes (C++ `size`).
    size: int4,
    /// Minimum bytes allowed for the logical value (C++ `minsize`).
    minsize: int4,
    /// How much alignment (0 means only 1 logical value is allowed) (C++
    /// `alignment`).
    alignment: int4,
    /// (Maximum) number of slots that can store separate parameters (C++
    /// `numslots`).
    numslots: int4,
    /// Non-null if this is a logical variable from joined pieces (C++ `joinrec`).
    joinrec: Option<Rc<JoinRecord>>,
}

impl ParamEntry {
    /// Constructor for use with decode (C++ `ParamEntry(int4 grp)`).  Seeds the
    /// group set with the single group `grp`; the remaining fields are filled
    /// in by `decode`.
    pub fn new(grp: int4) -> ParamEntry {
        ParamEntry {
            flags: 0,
            type_: type_class::TYPECLASS_GENERAL,
            group_set: vec![grp],
            spaceid: None,
            addressbase: 0,
            size: -1,
            minsize: -1,
            alignment: 0,
            numslots: 1,
            joinrec: None,
        }
    }

    /// Borrow the address space (panics on the pre-decode null, matching C++
    /// UB on a null `spaceid`).
    fn spaceid(&self) -> &Rc<AddrSpace> {
        self.spaceid
            .as_ref()
            .expect("ParamEntry::spaceid: null space (entry not decoded)")
    }

    // -- Simple accessors (fspec.hh:122-151) --------------------------------

    /// Get the group id this belongs to (C++ `getGroup`).
    pub fn get_group(&self) -> int4 {
        self.group_set[0]
    }
    /// Get all group numbers this overlaps (C++ `getAllGroups`).
    pub fn get_all_groups(&self) -> &Vec<int4> {
        &self.group_set
    }
    /// Get the size of the memory range in bytes (C++ `getSize`).
    pub fn get_size(&self) -> int4 {
        self.size
    }
    /// Get the minimum size of a logical value contained in this (C++
    /// `getMinSize`).
    pub fn get_min_size(&self) -> int4 {
        self.minsize
    }
    /// Get the alignment of this entry (C++ `getAlign`).
    pub fn get_align(&self) -> int4 {
        self.alignment
    }
    /// Get the join record describing the pieces, or `None` (C++
    /// `getJoinRecord`).
    pub fn get_join_record(&self) -> Option<&Rc<JoinRecord>> {
        self.joinrec.as_ref()
    }
    /// Get the data-type class associated with this (C++ `getType`).
    pub fn get_type(&self) -> type_class {
        self.type_
    }
    /// Return `true` if this holds a single parameter exclusively (C++
    /// `isExclusion`).
    pub fn is_exclusion(&self) -> bool {
        self.alignment == 0
    }
    /// Return `true` if parameters are allocated in reverse order (C++
    /// `isReverseStack`).
    pub fn is_reverse_stack(&self) -> bool {
        (self.flags & param_entry_flags::REVERSE_STACK) != 0
    }
    /// Return `true` if this is grouped with other entries (C++ `isGrouped`).
    pub fn is_grouped(&self) -> bool {
        (self.flags & param_entry_flags::IS_GROUPED) != 0
    }
    /// Return `true` if this overlaps another entry (C++ `isOverlap`).
    pub fn is_overlap(&self) -> bool {
        (self.flags & param_entry_flags::OVERLAPPING) != 0
    }
    /// Return `true` if this is the first entry in the storage class (C++
    /// `isFirstInClass`).
    pub fn is_first_in_class(&self) -> bool {
        (self.flags & param_entry_flags::FIRST_STORAGE) != 0
    }
    /// Get the address space containing this entry (C++ `getSpace`).
    pub fn get_space(&self) -> &Rc<AddrSpace> {
        self.spaceid()
    }
    /// Get the starting offset of this entry (C++ `getBase`).
    pub fn get_base(&self) -> uintb {
        self.addressbase
    }
    /// Return `true` if there is a high overlap (C++ `isParamCheckHigh`).
    pub fn is_param_check_high(&self) -> bool {
        (self.flags & param_entry_flags::EXTRACHECK_HIGH) != 0
    }
    /// Return `true` if there is a low overlap (C++ `isParamCheckLow`).
    pub fn is_param_check_low(&self) -> bool {
        (self.flags & param_entry_flags::EXTRACHECK_LOW) != 0
    }

    /// Is the logical value left-justified within its container (C++
    /// `isLeftJustified`).
    fn is_left_justified(&self) -> bool {
        (self.flags & param_entry_flags::FORCE_LEFT_JUSTIFY) != 0 || (!self.spaceid().is_big_endian())
    }

    // -- group / containment predicates (fspec.cc:159-365) ------------------

    /// Check if this and `op2` occupy any of the same groups (C++
    /// `groupOverlap`).  Both `group_set`s are sorted ascending; this is a
    /// merge-style intersection test.
    pub fn group_overlap(&self, op2: &ParamEntry) -> bool {
        let mut i = 0usize;
        let mut j = 0usize;
        let mut val_this = self.group_set[i];
        let mut val_other = op2.group_set[j];
        while val_this != val_other {
            if val_this < val_other {
                i += 1;
                if i >= self.group_set.len() {
                    return false;
                }
                val_this = self.group_set[i];
            } else {
                j += 1;
                if j >= op2.group_set.len() {
                    return false;
                }
                val_other = op2.group_set[j];
            }
        }
        true
    }

    /// Does this subsume the definition of `op2` (C++ `subsumesDefinition`).
    pub fn subsumes_definition(&self, op2: &ParamEntry) -> bool {
        if self.type_ != type_class::TYPECLASS_GENERAL && op2.type_ != self.type_ {
            return false;
        }
        // C++ compares the raw spaceid pointers.
        if !rc_opt_ptr_eq(&self.spaceid, &op2.spaceid) {
            return false;
        }
        if op2.addressbase < self.addressbase {
            return false;
        }
        // (op2.addressbase + op2.size - 1) > (addressbase + size - 1): uintb arith
        if op2.addressbase.wadd((op2.size - 1) as i64 as u64)
            > self.addressbase.wadd((self.size - 1) as i64 as u64)
        {
            return false;
        }
        if self.alignment != op2.alignment {
            return false;
        }
        true
    }

    /// Is the entire ParamEntry contained inside the range `[addr, addr+sz)`
    /// (C++ `containedBy`).  A join entry is never contained.
    pub fn contained_by(&self, addr: &Address, sz: int4) -> bool {
        if !rc_opt_eq_space(&self.spaceid, addr.get_space()) {
            return false;
        }
        if self.addressbase < addr.get_offset() {
            return false;
        }
        let entryoff: uintb = self.addressbase.wadd((self.size - 1) as i64 as u64);
        let rangeoff: uintb = addr.get_offset().wadd((sz - 1) as i64 as u64);
        entryoff <= rangeoff
    }

    /// Does this intersect the given range in some way (C++ `intersects`).
    pub fn intersects(&self, addr: &Address, sz: int4) -> bool {
        if let Some(jr) = &self.joinrec {
            let rangeend: uintb = addr.get_offset().wadd((sz - 1) as i64 as u64);
            for i in 0..jr.num_pieces() {
                let vdata = jr.get_piece(i);
                if !rc_opt_eq_space(&vdata.space, addr.get_space()) {
                    continue;
                }
                let vdataend: uintb = vdata.offset.wadd((vdata.size as i64 as u64).wsub(1));
                if addr.get_offset() < vdata.offset && rangeend < vdataend {
                    continue;
                }
                if addr.get_offset() > vdata.offset && rangeend > vdataend {
                    continue;
                }
                return true;
            }
        }
        if !rc_opt_eq_space(&self.spaceid, addr.get_space()) {
            return false;
        }
        let rangeend: uintb = addr.get_offset().wadd((sz - 1) as i64 as u64);
        let thisend: uintb = self.addressbase.wadd((self.size - 1) as i64 as u64);
        if addr.get_offset() < self.addressbase && rangeend < thisend {
            return false;
        }
        if addr.get_offset() > self.addressbase && rangeend > thisend {
            return false;
        }
        true
    }

    /// Endian-aware containment: if `[addr, addr+sz)` is contained in this,
    /// return the offset of the containment (0 == least significant byte),
    /// else -1 (C++ `justifiedContain`).
    pub fn justified_contain(&self, addr: &Address, sz: int4) -> int4 {
        if let Some(jr) = &self.joinrec {
            let mut res = 0;
            // Move from least significant to most.
            for i in (0..jr.num_pieces()).rev() {
                let vdata = jr.get_piece(i);
                let cur = vdata
                    .get_addr()
                    .justified_contain(vdata.size as i32, addr, sz, false);
                if cur < 0 {
                    res += vdata.size as i32; // We skipped this many less significant bytes
                } else {
                    return res + cur;
                }
            }
            return -1; // Not contained at all
        }
        if self.alignment == 0 {
            // Ordinary endian containment
            let entry = Address::new(Rc::clone(self.spaceid()), self.addressbase);
            return entry.justified_contain(
                self.size,
                addr,
                sz,
                (self.flags & param_entry_flags::FORCE_LEFT_JUSTIFY) != 0,
            );
        }
        if !rc_opt_eq_space(&self.spaceid, addr.get_space()) {
            return -1;
        }
        let mut startaddr: uintb = addr.get_offset();
        if startaddr < self.addressbase {
            return -1;
        }
        let endaddr: uintb = startaddr.wadd((sz - 1) as i64 as u64);
        if endaddr < startaddr {
            return -1; // Don't allow wrap around
        }
        if endaddr > self.addressbase.wadd((self.size - 1) as i64 as u64) {
            return -1;
        }
        startaddr = startaddr.wsub(self.addressbase);
        let endaddr = endaddr.wsub(self.addressbase);
        if !self.is_left_justified() {
            // For right justified (big endian), endaddr must be aligned
            let res = ((endaddr.wadd(1)) % (self.alignment as u64)) as i32;
            if res == 0 {
                return 0;
            }
            return self.alignment - res;
        }
        (startaddr % (self.alignment as u64)) as i32
    }

    /// Calculate the containing memory range, passing it back in `res` (C++
    /// `getContainer`).  Returns `true` if the given range is contained at all.
    pub fn get_container(&self, addr: &Address, sz: int4, res: &mut VarnodeData) -> bool {
        let endaddr = addr + ((sz - 1) as i64);
        if let Some(jr) = &self.joinrec {
            for i in (0..jr.num_pieces()).rev() {
                let vdata = jr.get_piece(i);
                if addr.overlap(0, &vdata.get_addr(), vdata.size as i32) >= 0
                    && endaddr.overlap(0, &vdata.get_addr(), vdata.size as i32) >= 0
                {
                    res.space = vdata.space.clone();
                    res.offset = vdata.offset;
                    res.size = vdata.size;
                    return true;
                }
            }
            return false; // Not contained at all
        }
        let entry = Address::new(Rc::clone(self.spaceid()), self.addressbase);
        if addr.overlap(0, &entry, self.size) < 0 {
            return false;
        }
        if endaddr.overlap(0, &entry, self.size) < 0 {
            return false;
        }
        if self.alignment == 0 {
            // Ordinary endian containment
            res.space = self.spaceid.clone();
            res.offset = self.addressbase;
            res.size = self.size as u32; // cast: int4 -> uint4 member
            return true;
        }
        let al: uintb = (addr.get_offset().wsub(self.addressbase)) % (self.alignment as u64);
        res.space = self.spaceid.clone();
        res.offset = addr.get_offset().wsub(al);
        // (int4)(endaddr.getOffset() - res.offset) + 1
        let mut size: int4 = (endaddr.get_offset().wsub(res.offset)) as i32 + 1;
        let al2: int4 = size % self.alignment;
        if al2 != 0 {
            size += self.alignment - al2; // Bump up size to nearest alignment
        }
        res.size = size as u32; // cast: int4 -> uint4 member
        true
    }

    /// Test that this (as one or more ranges) contains `op2`'s memory range
    /// (C++ `contains`).
    pub fn contains(&self, op2: &ParamEntry) -> bool {
        if op2.joinrec.is_some() {
            return false; // Assume a join entry cannot be contained
        }
        if self.joinrec.is_none() {
            let addr = Address::new(Rc::clone(self.spaceid()), self.addressbase);
            return op2.contained_by(&addr, self.size);
        }
        let jr = self.joinrec.as_ref().unwrap();
        for i in 0..jr.num_pieces() {
            let vdata = jr.get_piece(i);
            let addr = vdata.get_addr();
            if op2.contained_by(&addr, vdata.size as i32) {
                return true;
            }
        }
        false
    }

    /// Calculate the type of extension to expect for the given logical value
    /// (C++ `assumedExtension`).  Returns `CPUI_COPY` if no extension applies,
    /// otherwise passes back the container being extended in `res`.
    pub fn assumed_extension(&self, addr: &Address, sz: int4, res: &mut VarnodeData) -> OpCode {
        use param_entry_flags::*;
        if (self.flags & (SMALLSIZE_ZEXT | SMALLSIZE_SEXT | SMALLSIZE_INTTYPE)) == 0 {
            return OpCode::CPUI_COPY;
        }
        if self.alignment != 0 {
            if sz >= self.alignment {
                return OpCode::CPUI_COPY;
            }
        } else if sz >= self.size {
            return OpCode::CPUI_COPY;
        }
        if self.joinrec.is_some() {
            return OpCode::CPUI_COPY;
        }
        if self.justified_contain(addr, sz) != 0 {
            return OpCode::CPUI_COPY; // not justified properly to allow an extension
        }
        if self.alignment == 0 {
            // If exclusion, take up the whole entry
            res.space = self.spaceid.clone();
            res.offset = self.addressbase;
            res.size = self.size as u32; // cast: int4 -> uint4 member
        } else {
            // Otherwise take up whole alignment
            res.space = self.spaceid.clone();
            let align_adjust: uintb =
                (addr.get_offset().wsub(self.addressbase)) % (self.alignment as u64);
            res.offset = addr.get_offset().wsub(align_adjust);
            res.size = self.alignment as u32; // cast: int4 -> uint4 member
        }
        if (self.flags & SMALLSIZE_ZEXT) != 0 {
            return OpCode::CPUI_INT_ZEXT;
        }
        if (self.flags & SMALLSIZE_INTTYPE) != 0 {
            return OpCode::CPUI_PIECE;
        }
        OpCode::CPUI_INT_SEXT
    }

    /// Calculate the slot occupied by the byte `skip` ahead of `addr`, which is
    /// assumed already contained (C++ `getSlot`).
    pub fn get_slot(&self, addr: &Address, skip: int4) -> int4 {
        let mut res = self.group_set[0];
        if self.alignment != 0 {
            // diff = addr.getOffset() + skip - addressbase
            let diff: uintb = addr
                .get_offset()
                .wadd(skip as i64 as u64)
                .wsub(self.addressbase);
            let baseslot: int4 = (diff as i32) / self.alignment; // cast: (int4)diff
            if self.is_reverse_stack() {
                res += (self.numslots - 1) - baseslot;
            } else {
                res += baseslot;
            }
        } else if skip != 0 {
            res = *self.group_set.last().unwrap();
        }
        res
    }

    /// Calculate the storage address assigned when allocating a parameter of
    /// the given size, defaulting `justifyRight` to `!isLeftJustified()` (C++
    /// `getAddrBySlot(int4&,int4,int4)`).
    ///
    /// `manager` is the [`AddrSpaceManager`] reached through
    /// `spaceid->getManager()` in the C++ for the float-extension case.
    pub fn get_addr_by_slot(
        &self,
        slotnum: &mut int4,
        sz: int4,
        type_align: int4,
        manager: &AddrSpaceManager,
    ) -> KunaResult<Address> {
        self.get_addr_by_slot_justify(slotnum, sz, type_align, !self.is_left_justified(), manager)
    }

    /// Calculate the storage address assigned when allocating a parameter of
    /// the given size (C++ `getAddrBySlot(int4&,int4,int4,bool)`).  Returns an
    /// invalid address if the size is too small or there are not enough slots.
    pub fn get_addr_by_slot_justify(
        &self,
        slotnum: &mut int4,
        sz: int4,
        type_align: int4,
        justify_right: bool,
        manager: &AddrSpaceManager,
    ) -> KunaResult<Address> {
        let mut res = Address::new_invalid(); // Start with an invalid result
        let spaceused: int4;
        if sz < self.minsize {
            return Ok(res);
        }
        if self.alignment == 0 {
            // If not an aligned entry (allowing multiple slots)
            if *slotnum != 0 {
                return Ok(res); // Can only allocate slot 0
            }
            if sz > self.size {
                return Ok(res); // Check on maximum size
            }
            res = Address::new(Rc::clone(self.spaceid()), self.addressbase); // base of the slot
            spaceused = self.size;
            if (self.flags & param_entry_flags::SMALLSIZE_FLOATEXT) != 0 && sz != self.size {
                // implied floating-point extension
                res = manager.construct_float_extension_address(&res, self.size, sz)?;
                return Ok(res);
            }
        } else {
            if type_align > self.alignment {
                let tmp = (*slotnum * self.alignment) % type_align;
                if tmp != 0 {
                    *slotnum += (type_align - tmp) / self.alignment; // Waste slots to achieve typeAlign
                }
            }
            let mut slotsused = sz / self.alignment; // How many slots does a -sz- byte object need
            if (sz % self.alignment) != 0 {
                slotsused += 1;
            }
            if *slotnum + slotsused > self.numslots {
                return Ok(res); // Not enough slots left
            }
            spaceused = slotsused * self.alignment;
            let index: int4 = if self.is_reverse_stack() {
                self.numslots - *slotnum - slotsused
            } else {
                *slotnum
            };
            // addressbase + index * alignment
            res = Address::new(
                Rc::clone(self.spaceid()),
                self.addressbase.wadd((index * self.alignment) as i64 as u64),
            );
            *slotnum += slotsused; // Inform caller of number of slots used
        }
        if justify_right {
            // Adjust for right justified (big endian)
            res = &res + ((spaceused - sz) as i64);
        }
        Ok(res)
    }

    // -- resolution helpers run after decode (fspec.cc:62-157) --------------

    /// Find a ParamEntry in `entry_list` matching the storage triple `vn`,
    /// searching backward (C++ static `findEntryByStorage`).  Returns the index
    /// of the match in `entry_list` (the C++ returns a `ParamEntry *`).
    fn find_entry_by_storage(entry_list: &[ParamEntry], vn: &VarnodeData) -> Option<usize> {
        for i in (0..entry_list.len()).rev() {
            let entry = &entry_list[i];
            if rc_opt_ptr_eq(&entry.spaceid, &vn.space)
                && entry.addressbase == vn.offset
                && entry.size as u32 == vn.size
            {
                return Some(i);
            }
        }
        None
    }

    /// Mark this entry's `first_storage` flag based on the previous entry in
    /// `prev_list` (the entries decoded before this one) (C++ `resolveFirst`).
    /// In the C++ `--iter` reaches this entry (the last on the list) and
    /// `if (iter == begin)` tests whether it is the only entry — i.e.
    /// `prev_list` is empty here.
    fn resolve_first(&mut self, prev_list: &[ParamEntry]) {
        if prev_list.is_empty() {
            self.flags |= param_entry_flags::FIRST_STORAGE;
            return;
        }
        let prev = &prev_list[prev_list.len() - 1];
        if self.type_ != prev.type_ {
            self.flags |= param_entry_flags::FIRST_STORAGE;
        }
    }

    /// Cache the join record and adjust groups for a join entry (C++
    /// `resolveJoin`).  `prev_list` excludes `self`.
    fn resolve_join(&mut self, prev_list: &[ParamEntry], manager: &AddrSpaceManager) -> KunaResult<()> {
        if self.spaceid().get_type() != spacetype::IPTR_JOIN {
            self.joinrec = None;
            return Ok(());
        }
        let joinrec = manager.find_join(self.addressbase)?;
        self.joinrec = Some(Rc::clone(&joinrec));
        self.group_set.clear();
        for i in 0..joinrec.num_pieces() {
            let piece = piece_as_varnodedata(joinrec.get_piece(i));
            if let Some(idx) = ParamEntry::find_entry_by_storage(prev_list, &piece) {
                let entry = &prev_list[idx];
                self.group_set.extend_from_slice(&entry.group_set);
                // For output <pentry>, if the most significant part overlaps an
                // earlier entry the least significant part is extra-checked.
                self.flags |= if i == 0 {
                    param_entry_flags::EXTRACHECK_LOW
                } else {
                    param_entry_flags::EXTRACHECK_HIGH
                };
            }
        }
        if self.group_set.is_empty() {
            return Err(KunaError::lowlevel(
                "<pentry> join must overlap at least one previous entry",
            ));
        }
        self.group_set.sort_unstable();
        self.flags |= param_entry_flags::OVERLAPPING;
        Ok(())
    }

    /// Search for overlaps of this with previous entries and reassign the group
    /// if needed (C++ `resolveOverlap`).  `prev_list` excludes `self`.
    fn resolve_overlap(&mut self, prev_list: &[ParamEntry]) -> KunaResult<()> {
        if self.joinrec.is_some() {
            return Ok(()); // Overlaps with join records dealt with in resolveJoin
        }
        let mut overlap_set: Vec<int4> = Vec::new();
        let addr = Address::new(Rc::clone(self.spaceid()), self.addressbase);
        for entry in prev_list {
            if !entry.intersects(&addr, self.size) {
                continue;
            }
            if self.contains(entry) {
                if entry.is_overlap() {
                    continue; // Don't count resources (already counted overlapped entry)
                }
                overlap_set.extend_from_slice(&entry.group_set);
                if self.addressbase == entry.addressbase {
                    self.flags |= if self.spaceid().is_big_endian() {
                        param_entry_flags::EXTRACHECK_LOW
                    } else {
                        param_entry_flags::EXTRACHECK_HIGH
                    };
                } else {
                    self.flags |= if self.spaceid().is_big_endian() {
                        param_entry_flags::EXTRACHECK_HIGH
                    } else {
                        param_entry_flags::EXTRACHECK_LOW
                    };
                }
            } else {
                return Err(KunaError::lowlevel("Illegal overlap of <pentry> in compiler spec"));
            }
        }
        if overlap_set.is_empty() {
            return Ok(()); // No overlaps
        }
        overlap_set.sort_unstable();
        self.group_set = overlap_set;
        self.flags |= param_entry_flags::OVERLAPPING;
        Ok(())
    }

    /// Enforce ParamEntry group ordering rules; entries within a group must be
    /// distinguishable by size or type (C++ static `orderWithinGroup`).
    pub fn order_within_group(entry1: &ParamEntry, entry2: &ParamEntry) -> KunaResult<()> {
        if entry2.minsize > entry1.size || entry1.minsize > entry2.size {
            return Ok(());
        }
        if entry1.type_ != entry2.type_ {
            if entry1.type_ == type_class::TYPECLASS_GENERAL {
                return Err(KunaError::lowlevel(
                    "<pentry> tags with a specific type must come before the general type",
                ));
            }
            return Ok(());
        }
        Err(KunaError::lowlevel(
            "<pentry> tags within a group must be distinguished by size or type",
        ))
    }

    /// Decode a `<pentry>` element into this object (C++ `decode`).
    ///
    /// STUB(W4): reaches the fspec-owned marshaling ElementIds/AttributeIds
    /// (`<pentry>`, `minsize`, `maxsize`, `align`, ...) and `Address::decode`,
    /// which are not yet ported.  Tests build [`ParamEntry`] objects directly
    /// via [`ParamEntry::seed`].
    pub fn decode(
        &mut self,
        _normalstack: bool,
        _grouped: bool,
        _prev_list: &[ParamEntry],
    ) -> KunaResult<()> {
        Err(KunaError::lowlevel(
            "STUB(W4) ParamEntry::decode: fspec marshaling element ids not yet ported",
        ))
    }

    /// Test-and-tooling hook: build a fully-formed exclusion/resource entry
    /// without going through the (W4) decode path, running the post-decode
    /// resolution chain against the entries decoded before it (`prev_list`).
    /// Mirrors the tail of C++ `decode`.  Returns the resolved entry.
    #[allow(clippy::too_many_arguments)]
    pub fn seed(
        grp: int4,
        type_: type_class,
        space: Rc<AddrSpace>,
        addressbase: uintb,
        size: int4,
        minsize: int4,
        mut alignment: int4,
        flags: uint4,
        normalstack: bool,
        grouped: bool,
        prev_list: &[ParamEntry],
        manager: &AddrSpaceManager,
    ) -> KunaResult<ParamEntry> {
        if alignment == size {
            alignment = 0;
        }
        let mut e = ParamEntry::new(grp);
        e.flags = flags;
        e.type_ = type_;
        e.size = size;
        e.minsize = minsize;
        e.alignment = alignment;
        e.numslots = 1;
        e.spaceid = Some(Rc::clone(&space));
        e.addressbase = addressbase;
        if alignment != 0 {
            e.numslots = size / alignment;
        }
        if space.is_reverse_justified() {
            if space.is_big_endian() {
                e.flags |= param_entry_flags::FORCE_LEFT_JUSTIFY;
            } else {
                return Err(KunaError::lowlevel(
                    "No support for right justification in little endian encoding",
                ));
            }
        }
        if !normalstack {
            e.flags |= param_entry_flags::REVERSE_STACK;
            if alignment != 0 && (size % alignment) != 0 {
                return Err(KunaError::lowlevel(
                    "For positive stack growth, <pentry> size must match alignment",
                ));
            }
        }
        if grouped {
            e.flags |= param_entry_flags::IS_GROUPED;
        }
        e.resolve_first(prev_list);
        e.resolve_join(prev_list, manager)?;
        e.resolve_overlap(prev_list)?;
        Ok(e)
    }
}

/// Raw-pointer-style equality of two optional `Rc<AddrSpace>` (C++ pointer
/// compare; null == null).
fn rc_opt_ptr_eq(a: &Option<Rc<AddrSpace>>, b: &Option<Rc<AddrSpace>>) -> bool {
    match (a, b) {
        (Some(a), Some(b)) => Rc::ptr_eq(a, b),
        (None, None) => true,
        _ => false,
    }
}

/// Equality of an optional `Rc<AddrSpace>` against an `Option<&Rc<AddrSpace>>`
/// (as returned by `Address::get_space`), by pointer.
fn rc_opt_eq_space(a: &Option<Rc<AddrSpace>>, b: Option<&Rc<AddrSpace>>) -> bool {
    match (a, b) {
        (Some(a), Some(b)) => Rc::ptr_eq(a, b),
        (None, None) => true,
        _ => false,
    }
}

/// Convert a `JoinRecord` piece (`VarnodeStorage`) into the canonical
/// `kuna_num::pcoderaw::VarnodeData` triple.  The C++ `JoinRecord::getPiece`
/// returns a `VarnodeData &`; the kuna-base join record stores the equivalent
/// `VarnodeStorage`, so this is a field-for-field copy (recorded in losses).
fn piece_as_varnodedata(p: &VarnodeStorage) -> VarnodeData {
    VarnodeData { space: p.space.clone(), offset: p.offset, size: p.size }
}

// =============================================================================
// ParamTrial (fspec.hh:209-271, fspec.cc:1847-1936)
// =============================================================================

/// Boolean property flags for a [`ParamTrial`] (C++ anonymous enum,
/// `fspec.hh:211-223`).
pub mod param_trial_flags {
    use kuna_base::types::uint4;
    /// Trial has been checked.
    pub const CHECKED: uint4 = 1;
    /// Trial is definitely used (final verdict).
    pub const USED: uint4 = 2;
    /// Trial is definitely not used.
    pub const DEFNOUSE: uint4 = 4;
    /// Trial looks active (hint that it is used).
    pub const ACTIVE: uint4 = 8;
    /// There is no direct reference to this parameter trial.
    pub const UNREF: uint4 = 0x10;
    /// Data here is unlikely to flow through a func and still be a param.
    pub const KILLEDBYCALL: uint4 = 0x20;
    /// The trial is built out of a remainder operation.
    pub const REM_FORMED: uint4 = 0x40;
    /// The trial is built out of an indirect creation.
    pub const INDCREATE_FORMED: uint4 = 0x80;
    /// This trial may be affected by conditional execution.
    pub const CONDEXE_EFFECT: uint4 = 0x100;
    /// Trial has a realistic ancestor.
    pub const ANCESTOR_REALISTIC: uint4 = 0x200;
    /// Solid movement into the Varnode.
    pub const ANCESTOR_SOLID: uint4 = 0x400;
}

/// A register or memory location that may be used to pass a parameter or return
/// value (C++ `ParamTrial`, `fspec.hh:209-271`).
///
/// The link to the matching [`ParamEntry`] is modeled by `entry`, an index into
/// the owning [`ParamListStandard`]'s entry vector (the C++ holds a
/// `const ParamEntry *`).  This index is also used as the C++ "compare entry
/// pointers directly" tiebreak in [`ParamTrial::cmp`].
#[derive(Debug, Clone)]
pub struct ParamTrial {
    /// Boolean properties of the trial (C++ `flags`).
    flags: uint4,
    /// Starting address of the memory range (C++ `addr`).
    addr: Address,
    /// Number of bytes in the memory range (C++ `size`).
    size: int4,
    /// Slot assigned to this trial (C++ `slot`).
    slot: int4,
    /// PrototypeModel entry matching this trial (C++ `entry`), as an index into
    /// the owning entry vector; `None` is the C++ null pointer.
    entry: Option<usize>,
    /// "justified" offset into entry (C++ `offset`).
    offset: int4,
    /// Argument position if a fixed arg of a varargs function, else -1 (C++
    /// `fixedPosition`).
    fixed_position: int4,
}

impl ParamTrial {
    /// Construct from components (C++ `ParamTrial(const Address&,int4,int4)`).
    pub fn new(ad: Address, sz: int4, sl: int4) -> ParamTrial {
        ParamTrial {
            flags: 0,
            addr: ad,
            size: sz,
            slot: sl,
            entry: None,
            offset: -1,
            fixed_position: -1,
        }
    }

    /// Get the starting address of this trial (C++ `getAddress`).
    pub fn get_address(&self) -> &Address {
        &self.addr
    }
    /// Get the number of bytes in this trial (C++ `getSize`).
    pub fn get_size(&self) -> int4 {
        self.size
    }
    /// Get the slot associated with this trial (C++ `getSlot`).
    pub fn get_slot(&self) -> int4 {
        self.slot
    }
    /// Set the slot associated with this trial (C++ `setSlot`).
    pub fn set_slot(&mut self, val: int4) {
        self.slot = val;
    }
    /// Get the model-entry index associated with this trial (C++ `getEntry`).
    pub fn get_entry(&self) -> Option<usize> {
        self.entry
    }
    /// Get the offset associated with this trial (C++ `getOffset`).
    pub fn get_offset(&self) -> int4 {
        self.offset
    }
    /// Set the model entry (index) for this trial (C++ `setEntry`).
    pub fn set_entry(&mut self, ent: Option<usize>, off: int4) {
        self.entry = ent;
        self.offset = off;
    }
    /// Mark the trial as a formal parameter (C++ `markUsed`).
    pub fn mark_used(&mut self) {
        self.flags |= param_trial_flags::USED;
    }
    /// Mark that the trial is actively used in data-flow (C++ `markActive`).
    pub fn mark_active(&mut self) {
        self.flags |= param_trial_flags::ACTIVE | param_trial_flags::CHECKED;
    }
    /// Mark that the trial is not actively used (C++ `markInactive`).
    pub fn mark_inactive(&mut self) {
        self.flags &= !param_trial_flags::ACTIVE;
        self.flags |= param_trial_flags::CHECKED;
    }
    /// Mark trial as definitely not a parameter (C++ `markNoUse`).
    pub fn mark_no_use(&mut self) {
        self.flags &= !(param_trial_flags::ACTIVE | param_trial_flags::USED);
        self.flags |= param_trial_flags::CHECKED | param_trial_flags::DEFNOUSE;
    }
    /// Mark that this trial has no Varnode representative (C++ `markUnref`).
    pub fn mark_unref(&mut self) {
        self.flags |= param_trial_flags::UNREF | param_trial_flags::CHECKED;
        self.slot = -1;
    }
    /// Mark that this storage is killed-by-call (C++ `markKilledByCall`).
    pub fn mark_killed_by_call(&mut self) {
        self.flags |= param_trial_flags::KILLEDBYCALL;
    }
    /// Has this trial been checked (C++ `isChecked`).
    pub fn is_checked(&self) -> bool {
        (self.flags & param_trial_flags::CHECKED) != 0
    }
    /// Is this trial actively used in data-flow (C++ `isActive`).
    pub fn is_active(&self) -> bool {
        (self.flags & param_trial_flags::ACTIVE) != 0
    }
    /// Is this trial definitely not a parameter (C++ `isDefinitelyNotUsed`).
    pub fn is_definitely_not_used(&self) -> bool {
        (self.flags & param_trial_flags::DEFNOUSE) != 0
    }
    /// Is this trial a formal parameter (C++ `isUsed`).
    pub fn is_used(&self) -> bool {
        (self.flags & param_trial_flags::USED) != 0
    }
    /// Does this trial lack a Varnode representative (C++ `isUnref`).
    pub fn is_unref(&self) -> bool {
        (self.flags & param_trial_flags::UNREF) != 0
    }
    /// Is this storage killed-by-call (C++ `isKilledByCall`).
    pub fn is_killed_by_call(&self) -> bool {
        (self.flags & param_trial_flags::KILLEDBYCALL) != 0
    }
    /// Mark that this is formed by an INT_REM operation (C++ `setRemFormed`).
    pub fn set_rem_formed(&mut self) {
        self.flags |= param_trial_flags::REM_FORMED;
    }
    /// Is this formed by an INT_REM operation (C++ `isRemFormed`).
    pub fn is_rem_formed(&self) -> bool {
        (self.flags & param_trial_flags::REM_FORMED) != 0
    }
    /// Mark this trial as formed by indirect creation (C++ `setIndCreateFormed`).
    pub fn set_ind_create_formed(&mut self) {
        self.flags |= param_trial_flags::INDCREATE_FORMED;
    }
    /// Is this trial formed by indirect creation (C++ `isIndCreateFormed`).
    pub fn is_ind_create_formed(&self) -> bool {
        (self.flags & param_trial_flags::INDCREATE_FORMED) != 0
    }
    /// Mark this trial as possibly affected by conditional execution (C++
    /// `setCondExeEffect`).
    pub fn set_cond_exe_effect(&mut self) {
        self.flags |= param_trial_flags::CONDEXE_EFFECT;
    }
    /// Is this trial possibly affected by conditional execution (C++
    /// `hasCondExeEffect`).
    pub fn has_cond_exe_effect(&self) -> bool {
        (self.flags & param_trial_flags::CONDEXE_EFFECT) != 0
    }
    /// Mark this as having a realistic ancestor (C++ `setAncestorRealistic`).
    pub fn set_ancestor_realistic(&mut self) {
        self.flags |= param_trial_flags::ANCESTOR_REALISTIC;
    }
    /// Does this have a realistic ancestor (C++ `hasAncestorRealistic`).
    pub fn has_ancestor_realistic(&self) -> bool {
        (self.flags & param_trial_flags::ANCESTOR_REALISTIC) != 0
    }
    /// Mark this as showing solid movement into the Varnode (C++
    /// `setAncestorSolid`).
    pub fn set_ancestor_solid(&mut self) {
        self.flags |= param_trial_flags::ANCESTOR_SOLID;
    }
    /// Does this show solid movement into the Varnode (C++ `hasAncestorSolid`).
    pub fn has_ancestor_solid(&self) -> bool {
        (self.flags & param_trial_flags::ANCESTOR_SOLID) != 0
    }
    /// Set the fixed position (C++ `setFixedPosition`).
    pub fn set_fixed_position(&mut self, pos: int4) {
        self.fixed_position = pos;
    }
    /// Reset the memory range of this trial (C++ `setAddress`).
    pub fn set_address(&mut self, ad: Address, sz: int4) {
        self.addr = ad;
        self.size = sz;
    }

    /// Get the position of this within its parameter group (C++ `slotGroup`),
    /// resolving the entry against the owning entry vector.
    pub fn slot_group(&self, entries: &[ParamEntry]) -> int4 {
        let e = &entries[self.entry.expect("ParamTrial::slot_group on null entry")];
        e.get_slot(&self.addr, self.size - 1)
    }

    /// Create a trial representing the first part of this (C++ `splitHi`).
    pub fn split_hi(&self, sz: int4) -> ParamTrial {
        let mut res = ParamTrial::new(self.addr.clone(), sz, self.slot);
        res.flags = self.flags;
        res
    }

    /// Create a trial representing the last part of this (C++ `splitLo`).
    pub fn split_lo(&self, sz: int4) -> ParamTrial {
        let newaddr = &self.addr + ((self.size - sz) as i64);
        let mut res = ParamTrial::new(newaddr, sz, self.slot + 1);
        res.flags = self.flags;
        res
    }

    /// Test if this trial can be shrunk to the given range (C++ `testShrink`).
    pub fn test_shrink(&self, newaddr: &Address, sz: int4) -> bool {
        let testaddr = if self.addr.is_big_endian() {
            &self.addr + ((self.size - sz) as i64)
        } else {
            self.addr.clone()
        };
        if &testaddr != newaddr {
            return false;
        }
        if self.entry.is_some() {
            return false;
        }
        true
    }

    /// Compare two trials in formal parameter order (C++ `operator<`).
    ///
    /// `entries` resolves the entry index to a [`ParamEntry`]; the C++ "compare
    /// entry pointers directly" is replicated by comparing the entry indices,
    /// which preserve the storage-list order of the `list<ParamEntry>`.
    pub fn cmp(&self, b: &ParamTrial, entries: &[ParamEntry]) -> std::cmp::Ordering {
        use std::cmp::Ordering::*;
        // C++ `operator<`: when self.entry is null, `self < b` is false (line 1898);
        // when b.entry is null (self.entry non-null), `self < b` is true (line 1899).
        // Under the strict weak ordering these mean: two null-entry trials are
        // EQUIVALENT (both `a<b` and `b<a` are false) => Equal; a null-entry self
        // never sorts before a non-null b => Greater; a non-null self always sorts
        // before a null b => Less. A blanket `(None, _) => Greater` would make
        // (None, None) compare Greater in BOTH directions, breaking antisymmetry
        // and producing an unspecified `sort_unstable_by` order, so the null cases
        // are split out explicitly to keep the comparator total.
        let (ea, eb) = match (self.entry, b.entry) {
            (None, None) => return Equal, // both null: equivalent (C++ both `<` false)
            (None, Some(_)) => return Greater, // self not "<" b  (C++ line 1898 false)
            (Some(_), None) => return Less,    // self "<" b       (C++ line 1899 true)
            (Some(ea), Some(eb)) => (ea, eb),
        };
        let entry_a = &entries[ea];
        let entry_b = &entries[eb];
        let grpa = entry_a.get_group();
        let grpb = entry_b.get_group();
        if grpa != grpb {
            return grpa.cmp(&grpb);
        }
        if ea != eb {
            // Compare entry pointers directly (storage-list order).
            return ea.cmp(&eb);
        }
        if entry_a.is_exclusion() {
            return self.offset.cmp(&b.offset);
        }
        if self.addr != b.addr {
            return if entry_a.is_reverse_stack() {
                b.addr.cmp(&self.addr)
            } else {
                self.addr.cmp(&b.addr)
            };
        }
        self.size.cmp(&b.size)
    }

    /// Sort by fixed position then by [`ParamTrial::cmp`] (C++
    /// `fixedPositionCompare`).
    pub fn fixed_position_compare(
        a: &ParamTrial,
        b: &ParamTrial,
        entries: &[ParamEntry],
    ) -> std::cmp::Ordering {
        use std::cmp::Ordering::*;
        if a.fixed_position == -1 && b.fixed_position == -1 {
            return a.cmp(b, entries);
        }
        if a.fixed_position == -1 {
            return Greater; // C++ returns false (a not before b)
        }
        if b.fixed_position == -1 {
            return Less; // C++ returns true (a before b)
        }
        a.fixed_position.cmp(&b.fixed_position)
    }
}

// =============================================================================
// ParamActive (fspec.hh:285-360, fspec.cc:1938-2107)
// =============================================================================

/// The mutable collection of [`ParamTrial`] objects for one function during
/// parameter analysis (C++ `ParamActive`, `fspec.hh:285-360`).
///
/// Sorting methods take a `&[ParamEntry]` (the owning model's entry vector) to
/// resolve the entry-index tiebreak inside [`ParamTrial::cmp`], matching the
/// C++ where the trial's `entry` pointer is dereferenced by the comparator.
#[derive(Debug, Clone)]
pub struct ParamActive {
    /// The list of parameter trials (C++ `trial`).
    trial: Vec<ParamTrial>,
    /// Slot where next parameter will go (C++ `slotbase`).
    slotbase: int4,
    /// Which call input slot holds the stack placeholder (C++ `stackplaceholder`).
    stackplaceholder: int4,
    /// Number of attempts at evaluating parameters (C++ `numpasses`).
    numpasses: int4,
    /// Number of passes before we assume we have seen all params (C++ `maxpass`).
    maxpass: int4,
    /// True if all trials are fully examined (C++ `isfullychecked`).
    isfullychecked: bool,
    /// Should a final pass be made on trials (C++ `needsfinalcheck`).
    needsfinalcheck: bool,
    /// True if recovering prototypes of a sub-function call (C++ `recoversubcall`).
    recoversubcall: bool,
    /// True if varnodes should be joined in reverse order (C++ `joinReverse`).
    join_reverse: bool,
    /// (kuna) `varargstackargs`: score the stack tail of a `fillinMap` resource
    /// section separately from its register prefix, because these trials belong
    /// to a VARIADIC call whose variable arguments the ABI passes on the stack.
    /// Set by `ActionActiveParam`; see [`crate::p4_calls::kuna_varargstackargs`].
    vararg_stack_split: bool,
    /// (kuna) `inputparamgap`: are these the trials of the FUNCTION'S OWN input
    /// recovery, with the register-gap tolerance enabled?  Set by
    /// `ActionInputPrototype`; see [`crate::p4_calls::kuna_inputparamgap`].
    own_input_gap: bool,
}

impl ParamActive {
    /// Construct an empty container (C++ `ParamActive(bool)`).
    pub fn new(recoversub: bool) -> ParamActive {
        ParamActive {
            trial: Vec::new(),
            slotbase: 1,
            stackplaceholder: -1,
            numpasses: 0,
            maxpass: 0,
            isfullychecked: false,
            needsfinalcheck: false,
            recoversubcall: recoversub,
            join_reverse: false,
            vararg_stack_split: false, // (kuna) varargstackargs
            own_input_gap: false,      // (kuna) inputparamgap
        }
    }

    /// Reset to an empty container (C++ `clear`).
    pub fn clear(&mut self) {
        self.trial.clear();
        self.slotbase = 1;
        self.stackplaceholder = -1;
        self.numpasses = 0;
        self.isfullychecked = false;
        self.join_reverse = false;
    }

    /// Add a new trial to the container (C++ `registerTrial`).
    pub fn register_trial(&mut self, addr: &Address, sz: int4) {
        let mut t = ParamTrial::new(addr.clone(), sz, self.slotbase);
        // Heuristic: a non-stack location is unlikely to survive a call.
        if addr
            .get_space()
            .map(|s| s.get_type() != spacetype::IPTR_SPACEBASE)
            .unwrap_or(true)
        {
            t.mark_killed_by_call();
        }
        self.trial.push(t);
        self.slotbase += 1;
    }

    /// Get the number of trials in this container (C++ `getNumTrials`).
    pub fn get_num_trials(&self) -> int4 {
        self.trial.len() as i32 // cast: vector::size() -> int4
    }
    /// Get the i-th trial (C++ `getTrial`).
    pub fn get_trial(&self, i: int4) -> &ParamTrial {
        &self.trial[i as usize]
    }
    /// Get the i-th trial mutably (C++ `getTrial` non-const).
    pub fn get_trial_mut(&mut self, i: int4) -> &mut ParamTrial {
        &mut self.trial[i as usize]
    }

    /// Get the trial associated with the input Varnode at the given CALL/CALLIND
    /// input index (C++ inline `ParamActive::getTrialForInputVarnode`,
    /// `fspec.hh:1749`).
    ///
    /// The C++ accounts for the call-address parameter (subtract 1) and, if the
    /// index occurs \e after the index holding the stack-pointer placeholder,
    /// subtracts an additional 1:
    /// `slot -= ((stackplaceholder<0)||(slot<stackplaceholder)) ? 1 : 2;`
    pub fn get_trial_for_input_varnode(&self, slot: int4) -> &ParamTrial {
        let slot =
            slot - if (self.stackplaceholder < 0) || (slot < self.stackplaceholder) { 1 } else { 2 };
        &self.trial[slot as usize]
    }

    /// Get the (index of the) first trial overlapping the given range (C++
    /// `whichTrial`).
    pub fn which_trial(&self, addr: &Address, sz: int4) -> int4 {
        for (i, t) in self.trial.iter().enumerate() {
            if addr.overlap(0, t.get_address(), t.get_size()) >= 0 {
                return i as i32;
            }
            if sz <= 1 {
                return -1;
            }
            let endaddr = addr + ((sz - 1) as i64);
            if endaddr.overlap(0, t.get_address(), t.get_size()) >= 0 {
                return i as i32;
            }
        }
        -1
    }

    /// Is a final check required (C++ `needsFinalCheck`).
    pub fn needs_final_check(&self) -> bool {
        self.needsfinalcheck
    }
    /// Mark that a final check is required (C++ `markNeedsFinalCheck`).
    pub fn mark_needs_final_check(&mut self) {
        self.needsfinalcheck = true;
    }
    /// Do varnodes need to be joined in reverse order (C++ `isJoinReverse`).
    pub fn is_join_reverse(&self) -> bool {
        self.join_reverse
    }
    /// Mark that varnodes need to be joined in reverse order (C++ `setJoinReverse`).
    pub fn set_join_reverse(&mut self) {
        self.join_reverse = true;
    }
    /// (kuna) `varargstackargs`: are these the trials of a variadic call whose
    /// stack tail must be scored as its own `fillinMap` section?
    pub fn is_vararg_stack_split(&self) -> bool {
        self.vararg_stack_split
    }
    /// (kuna) `varargstackargs`: record that these trials belong to a variadic
    /// call site.  `clear()` deliberately leaves it alone -- like
    /// `recoversubcall` it is a property of the call, not of one pass.
    pub fn set_vararg_stack_split(&mut self, val: bool) {
        self.vararg_stack_split = val;
    }

    /// (kuna) `inputparamgap`: are these the function's OWN input trials, with
    /// the unused-register-run tolerance enabled?  Read by
    /// [`Self::force_inactive_chain`](ParamListStandard) through
    /// [`crate::p4_calls::kuna_inputparamgap::gap_slot_is_exempt`].
    pub fn is_own_input_gap(&self) -> bool {
        self.own_input_gap
    }

    /// (kuna) `inputparamgap`: record that these are the function's own input
    /// trials and the option is on.  Only `ActionInputPrototype` sets it; a call
    /// site's trials keep the upstream chain rule.
    pub fn set_own_input_gap(&mut self, val: bool) {
        self.own_input_gap = val;
    }
    /// Are these trials for a call to a sub-function (C++ `isRecoverSubcall`).
    pub fn is_recover_subcall(&self) -> bool {
        self.recoversubcall
    }
    /// Are all trials checked with no new trials expected (C++ `isFullyChecked`).
    pub fn is_fully_checked(&self) -> bool {
        self.isfullychecked
    }
    /// Mark that all trials are checked (C++ `markFullyChecked`).
    pub fn mark_fully_checked(&mut self) {
        self.isfullychecked = true;
    }
    /// Establish a stack placeholder slot (C++ `setPlaceholderSlot`).
    pub fn set_placeholder_slot(&mut self) {
        self.stackplaceholder = self.slotbase;
        self.slotbase += 1;
    }
    /// How many trial analysis passes were performed (C++ `getNumPasses`).
    pub fn get_num_passes(&self) -> int4 {
        self.numpasses
    }
    /// What is the maximum number of passes (C++ `getMaxPass`).
    pub fn get_max_pass(&self) -> int4 {
        self.maxpass
    }
    /// Set the maximum number of passes (C++ `setMaxPass`).
    pub fn set_max_pass(&mut self, val: int4) {
        self.maxpass = val;
    }
    /// Mark that an analysis pass has completed (C++ `finishPass`).
    pub fn finish_pass(&mut self) {
        self.numpasses += 1;
    }

    /// Sort the trials in formal parameter order (C++ `sortTrials`).
    pub fn sort_trials(&mut self, entries: &[ParamEntry]) {
        // std::sort is not stable; sort_unstable_by mirrors that.
        self.trial.sort_unstable_by(|a, b| a.cmp(b, entries));
    }

    /// Sort the trials by fixed position then by [`ParamTrial::cmp`] (C++
    /// `sortFixedPosition`).
    pub fn sort_fixed_position(&mut self, entries: &[ParamEntry]) {
        self.trial
            .sort_unstable_by(|a, b| ParamTrial::fixed_position_compare(a, b, entries));
    }

    /// Free the stack placeholder slot, adjusting trial slots (C++
    /// `freePlaceholderSlot`).
    pub fn free_placeholder_slot(&mut self) {
        for t in self.trial.iter_mut() {
            if t.get_slot() > self.stackplaceholder {
                t.set_slot(t.get_slot() - 1);
            }
        }
        self.stackplaceholder = -2;
        self.slotbase -= 1;
        self.maxpass = 0;
    }

    /// Delete any trial for which `isUsed()` is false, reordering slots (C++
    /// `deleteUnusedTrials`).
    pub fn delete_unused_trials(&mut self) {
        let mut newtrials: Vec<ParamTrial> = Vec::new();
        let mut slot = 1;
        for curtrial in self.trial.iter() {
            if curtrial.is_used() {
                let mut c = curtrial.clone();
                c.set_slot(slot);
                slot += 1;
                newtrials.push(c);
            }
        }
        self.trial = newtrials;
    }

    /// Split the trial at index `i` into two, the first piece having size `sz`
    /// (C++ `splitTrial`).
    pub fn split_trial(&mut self, i: int4, sz: int4) -> KunaResult<()> {
        if self.stackplaceholder >= 0 {
            return Err(KunaError::lowlevel(
                "Cannot split parameter when the placeholder has not been recovered",
            ));
        }
        let i = i as usize;
        let mut newtrials: Vec<ParamTrial> = Vec::new();
        let slot = self.trial[i].get_slot();
        for j in 0..i {
            let mut c = self.trial[j].clone();
            let oldslot = c.get_slot();
            if oldslot > slot {
                c.set_slot(oldslot + 1);
            }
            newtrials.push(c);
        }
        newtrials.push(self.trial[i].split_hi(sz));
        newtrials.push(self.trial[i].split_lo(self.trial[i].get_size() - sz));
        for j in (i + 1)..self.trial.len() {
            let mut c = self.trial[j].clone();
            let oldslot = c.get_slot();
            if oldslot > slot {
                c.set_slot(oldslot + 1);
            }
            newtrials.push(c);
        }
        self.slotbase += 1;
        self.trial = newtrials;
        Ok(())
    }

    /// Join the trial at `slot` with the trial in the next slot (C++
    /// `joinTrial`).
    pub fn join_trial(&mut self, slot: int4, addr: &Address, sz: int4) -> KunaResult<()> {
        if self.stackplaceholder >= 0 {
            return Err(KunaError::lowlevel(
                "Cannot join parameters when the placeholder has not been removed",
            ));
        }
        let mut newtrials: Vec<ParamTrial> = Vec::new();
        let mut sizecheck = 0;
        for curtrial in self.trial.iter() {
            let curslot = curtrial.get_slot();
            if curslot < slot {
                newtrials.push(curtrial.clone());
            } else if curslot == slot {
                sizecheck += curtrial.get_size();
                let mut t = ParamTrial::new(addr.clone(), sz, slot);
                t.mark_used();
                t.mark_active();
                newtrials.push(t);
            } else if curslot == slot + 1 {
                // this slot is thrown out
                sizecheck += curtrial.get_size();
            } else {
                let mut c = curtrial.clone();
                c.set_slot(curslot - 1);
                newtrials.push(c);
            }
        }
        if sizecheck != sz {
            return Err(KunaError::lowlevel("Size mismatch when joining parameters"));
        }
        self.slotbase -= 1;
        self.trial = newtrials;
        Ok(())
    }

    /// Get number of trials marked as formal parameters (assumes sorted) (C++
    /// `getNumUsed`).
    pub fn get_num_used(&self) -> int4 {
        let mut count = 0;
        while (count as usize) < self.trial.len() {
            if !self.trial[count as usize].is_used() {
                break;
            }
            count += 1;
        }
        count
    }

    /// Test if the trial at `i` can be shrunk to the given range (C++
    /// `testShrink`).
    pub fn test_shrink(&self, i: int4, addr: &Address, sz: int4) -> bool {
        self.trial[i as usize].test_shrink(addr, sz)
    }

    /// Shrink the trial at `i` to a new range (C++ `shrink`).
    pub fn shrink(&mut self, i: int4, addr: Address, sz: int4) {
        self.trial[i as usize].set_address(addr, sz);
    }
}

// =============================================================================
// ParameterPieces (fspec.hh:359-371, fspec.cc:2180-2215)
// =============================================================================

/// Property flags for a [`ParameterPieces`] (C++ anonymous enum,
/// `fspec.hh:360-366`).
pub mod parameter_pieces_flags {
    use kuna_base::types::uint4;
    /// Parameter is "this" pointer.
    pub const ISTHIS: uint4 = 1;
    /// Parameter is hidden pointer to return value.
    pub const HIDDENRETPARM: uint4 = 2;
    /// Parameter is indirect pointer to true parameter.
    pub const INDIRECTSTORAGE: uint4 = 4;
    /// Parameter's name is locked.
    pub const NAMELOCK: uint4 = 8;
    /// Parameter's data-type is locked.
    pub const TYPELOCK: uint4 = 16;
    /// Size of the parameter is locked (but not the data-type).
    pub const SIZELOCK: uint4 = 32;
}

/// Basic elements of a parameter: address, data-type, properties (C++
/// `ParameterPieces`, `fspec.hh:359-371`).
#[derive(Debug, Clone)]
pub struct ParameterPieces {
    /// Storage address of the parameter (C++ `addr`).
    pub addr: Address,
    /// The data-type of the parameter (C++ `type`); `None` is the C++ null.
    pub type_: Option<Rc<Datatype>>,
    /// Additional attributes of the parameter (C++ `flags`).
    pub flags: uint4,
}

impl Default for ParameterPieces {
    fn default() -> Self {
        ParameterPieces { addr: Address::new_invalid(), type_: None, flags: 0 }
    }
}

impl ParameterPieces {
    /// Swap data-type and flags with another parameter, leaving the storage
    /// address intact (C++ `swapMarkup`).
    pub fn swap_markup(&mut self, op: &mut ParameterPieces) {
        std::mem::swap(&mut self.flags, &mut op.flags);
        std::mem::swap(&mut self.type_, &mut op.type_);
    }

    /// Generate a parameter address from the list of Varnodes making up the
    /// parameter (C++ `ParameterPieces::assignAddressFromPieces`, fspec.cc:2196).
    ///
    /// `pieces` is assumed ordered most-significant-to-least when `most_to_least`
    /// is set; otherwise it is reversed first.  Contiguous register/stack pieces
    /// are merged (`JoinRecord::mergeSequence`); if a single piece remains its
    /// address is used directly, otherwise a JOIN-space record is found/created
    /// (`Architecture::findAddJoin`) and its unified address is the storage.
    pub fn assign_address_from_pieces(
        &mut self,
        pieces: &mut Vec<VarnodeData>,
        most_to_least: bool,
        manager: &AddrSpaceManager,
    ) -> KunaResult<()> {
        if !most_to_least && pieces.len() > 1 {
            pieces.reverse();
        }
        // The kuna join machinery operates on `VarnodeStorage`; convert the
        // `VarnodeData` triples in place, merge contiguous ranges through the
        // manager's installed RegisterLookup (the C++ `glb->translate`), then
        // continue on the merged storage list.
        let mut seq: Vec<VarnodeStorage> = pieces
            .iter()
            .map(|p| VarnodeStorage { space: p.space.clone(), offset: p.offset, size: p.size })
            .collect();
        if let Some(lookup) = manager.register_lookup() {
            let lookup = Rc::clone(lookup);
            JoinRecord::merge_sequence(&mut seq, lookup.as_ref());
        }
        if seq.len() == 1 {
            let p = &seq[0];
            self.addr = Address::new(
                p.space.clone().ok_or_else(|| {
                    KunaError::lowlevel("assignAddressFromPieces: merged piece has no space")
                })?,
                p.offset,
            );
            // Reflect the merge back into the caller's piece list.
            *pieces = seq.iter().map(|p| VarnodeData {
                space: p.space.clone(), offset: p.offset, size: p.size,
            }).collect();
            return Ok(());
        }
        let join_record = manager.find_add_join(&seq, 0)?;
        let unified = join_record.get_unified();
        self.addr = Address::new(
            unified.space.clone().ok_or_else(|| {
                KunaError::lowlevel("assignAddressFromPieces: join record has no unified space")
            })?,
            unified.offset,
        );
        // Reflect the (possibly merged) piece list back to the caller.
        *pieces = seq.iter().map(|p| VarnodeData {
            space: p.space.clone(), offset: p.offset, size: p.size,
        }).collect();
        Ok(())
    }
}

// =============================================================================
// EffectRecord (fspec.hh:387-414, fspec.cc:2217-2266)
// =============================================================================

/// The kind of indirect effect a sub-function has on a memory range (C++
/// anonymous enum inside `EffectRecord`, `fspec.hh:389-394`).
pub mod effect_type {
    use kuna_base::types::uint4;
    /// The sub-function does not change the value at all.
    pub const UNAFFECTED: uint4 = 1;
    /// The memory is changed and unrelated to its original value.
    pub const KILLEDBYCALL: uint4 = 2;
    /// The memory is being used to store the return address.
    pub const RETURN_ADDRESS: uint4 = 3;
    /// An unknown effect (indicates the absence of an EffectRecord).
    pub const UNKNOWN_EFFECT: uint4 = 4;
}

/// Description of the indirect effect a sub-function has on a memory range (C++
/// `EffectRecord`, `fspec.hh:387-414`).
#[derive(Debug, Clone)]
pub struct EffectRecord {
    /// The memory range affected (C++ `range`).
    range: VarnodeData,
    /// The type of effect (C++ `type`).
    type_: uint4,
}

impl EffectRecord {
    /// Construct a memory range with an unknown effect (C++
    /// `EffectRecord(const Address&,int4)`).
    pub fn new_unknown(addr: &Address, size: int4) -> EffectRecord {
        EffectRecord {
            range: VarnodeData {
                space: addr.get_space().cloned(),
                offset: addr.get_offset(),
                size: size as u32, // cast: int4 -> uint4 member
            },
            type_: effect_type::UNKNOWN_EFFECT,
        }
    }

    /// Construct an effect on a parameter storage location (C++
    /// `EffectRecord(const ParamEntry&,uint4)`).
    pub fn from_param_entry(entry: &ParamEntry, t: uint4) -> EffectRecord {
        EffectRecord {
            range: VarnodeData {
                space: Some(Rc::clone(entry.get_space())),
                offset: entry.get_base(),
                size: entry.get_size() as u32, // cast: int4 -> uint4 member
            },
            type_: t,
        }
    }

    /// Construct an effect on a memory range (C++
    /// `EffectRecord(const VarnodeData&,uint4)`).
    pub fn from_varnode(data: VarnodeData, t: uint4) -> EffectRecord {
        EffectRecord { range: data, type_: t }
    }

    /// Get the type of effect (C++ `getType`).
    pub fn get_type(&self) -> uint4 {
        self.type_
    }
    /// Get the starting address of the affected range (C++ `getAddress`).
    pub fn get_address(&self) -> Address {
        self.range.get_addr()
    }
    /// Get the size of the affected range (C++ `getSize`).
    pub fn get_size(&self) -> int4 {
        self.range.size as i32 // cast: uint4 -> int4
    }

    /// Compare two effect records by their starting address (C++
    /// `compareByAddress`).  The C++ compares `range.space` by index then
    /// `range.offset`; `Address::cmp` transcribes the same ordering.
    pub fn compare_by_address(op1: &EffectRecord, op2: &EffectRecord) -> std::cmp::Ordering {
        let s1 = op1.range.get_addr();
        let s2 = op2.range.get_addr();
        s1.cmp(&s2)
    }

    /// Encode this record as a sized `<addr>` element (C++
    /// `EffectRecord::encode`, fspec.cc:3560-3568).  Only the three named
    /// effect types are encodable.
    pub fn encode(&self, encoder: &mut dyn kuna_base::marshal::Encoder) -> KunaResult<()> {
        let addr = self.range.get_addr();
        if self.type_ == effect_type::UNAFFECTED
            || self.type_ == effect_type::KILLEDBYCALL
            || self.type_ == effect_type::RETURN_ADDRESS
        {
            addr.encode_sized(encoder, self.range.size as int4)
        } else {
            Err(KunaError::lowlevel("Bad EffectRecord type"))
        }
    }
}

impl PartialEq for EffectRecord {
    /// C++ `operator==`: type and range must match.
    fn eq(&self, op2: &EffectRecord) -> bool {
        if self.type_ != op2.type_ {
            return false;
        }
        self.range == op2.range
    }
}
impl Eq for EffectRecord {}

// =============================================================================
// ParamList family (fspec.hh:417-728, fspec.cc:599-1844)
// =============================================================================

/// The type discriminant of a [`ParamList`] (C++ `ParamList::enum`,
/// `fspec.hh:419-425`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParamListType {
    /// Standard input parameter model.
    Standard = 0,
    /// Standard output (return value) model.
    StandardOut = 1,
    /// Unordered parameter passing locations model.
    Register = 2,
    /// Multiple possible return value locations model.
    RegisterOut = 3,
    /// A merged model (multiple models merged together).
    Merged = 4,
}

/// The concrete kind of a [`ParamListStandard`] (which carries the data and
/// dispatches the per-kind algorithm variants).  This mirrors the C++ class
/// hierarchy `ParamListStandard` / `ParamListStandardOut` / ... as a tag, so a
/// single owned struct carries the shared `entry`/resolver state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParamListKind {
    /// `ParamListStandard`.
    Standard,
    /// `ParamListStandardOut`.
    StandardOut,
    /// `ParamListRegisterOut`.
    RegisterOut,
    /// `ParamListRegister`.
    Register,
    /// `ParamListMerged`.
    Merged,
}

/// A group of [`ParamEntry`] objects forming a complete set for passing
/// parameters in one direction (C++ `ParamListStandard` and its subclasses,
/// `fspec.hh:579-728`).
///
/// The C++ class hierarchy (`ParamListStandard` -> `ParamListStandardOut` ->
/// `ParamListRegisterOut`, `ParamListRegister`, `ParamListMerged`) is collapsed
/// into one struct tagged by [`ParamListKind`]; the per-kind method bodies
/// dispatch on `kind`.  The shared state (`entry`, `resolver_map`,
/// `resource_start`, ...) lives here exactly as in the `ParamListStandard` base.
///
/// `Debug` and `Clone` are implemented manually because [`ParamEntryResolver`]
/// (a [`kuna_base::rangemap::RangeMap`]) provides neither.  Like the C++ copy
/// constructor (which calls `populateResolver()` rather than copying the
/// resolver map), `Clone` rebuilds the resolver from the cloned entries.
pub struct ParamListStandard {
    /// Which concrete model this is.
    kind: ParamListKind,
    /// Number of groups in this convention (C++ `numgroup`).
    numgroup: int4,
    /// Maximum heritage delay across all parameters (C++ `maxdelay`).
    maxdelay: int4,
    /// Does a `this` parameter come before a hidden return parameter (C++
    /// `thisbeforeret`).
    thisbeforeret: bool,
    /// Are storage locations automatically killed-by-call (C++ `autoKilledByCall`).
    auto_killed_by_call: bool,
    /// The starting group for each resource section (C++ `resourceStart`).
    resource_start: Vec<int4>,
    /// The ordered list of parameter entries (C++ `entry`, a `list<ParamEntry>`).
    entry: Vec<ParamEntry>,
    /// Map from space index to the offset->entry resolver (C++ `resolverMap`).
    /// Each resolver maps an offset to an index into `entry`.
    resolver_map: Vec<Option<ParamEntryResolver>>,
    /// Rules to apply when assigning addresses (C++ `modelRules`).  Decoded from
    /// the cspec `<rule>` elements (plus the synthetic `pointermax`
    /// ConvertToPointer rule) by the architecture cspec loader.
    model_rules: Vec<ModelRule>,
    /// Address space containing relative offset parameters (C++ `spacebase`).
    spacebase: Option<Rc<AddrSpace>>,
    /// If true, use the legacy fillin fallback for output (C++
    /// `ParamListStandardOut::useFillinFallback`).  Stays true here: the
    /// output-side `fillinOutputMap`/`canAffectFillinOutput` wiring (C++
    /// `ParamListStandardOut::initialize`, fspec.cc:1616-1628) is a separate
    /// STUB — the output TRIAL recovery keeps the legacy fallback while only the
    /// `assignAddress` (locked-param storage) rule chain is wired.
    use_fillin_fallback: bool,
}

impl std::fmt::Debug for ParamListStandard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // The resolver map is a derived index (rebuilt from `entry` by
        // `populate_resolver`) and has no Debug; omit it.
        f.debug_struct("ParamListStandard")
            .field("kind", &self.kind)
            .field("numgroup", &self.numgroup)
            .field("maxdelay", &self.maxdelay)
            .field("thisbeforeret", &self.thisbeforeret)
            .field("auto_killed_by_call", &self.auto_killed_by_call)
            .field("resource_start", &self.resource_start)
            .field("entry", &self.entry)
            .field("model_rules", &self.model_rules)
            .field("spacebase", &self.spacebase)
            .field("use_fillin_fallback", &self.use_fillin_fallback)
            .finish_non_exhaustive()
    }
}

impl Clone for ParamListStandard {
    /// C++ `ParamListStandard(const ParamListStandard &op2)`: copy the scalar
    /// state and entries, then rebuild the resolver via `populateResolver()`
    /// (the resolver map is never copied directly).
    fn clone(&self) -> ParamListStandard {
        let mut res = ParamListStandard {
            kind: self.kind,
            numgroup: self.numgroup,
            maxdelay: self.maxdelay,
            thisbeforeret: self.thisbeforeret,
            auto_killed_by_call: self.auto_killed_by_call,
            resource_start: self.resource_start.clone(),
            entry: self.entry.clone(),
            resolver_map: Vec::new(),
            model_rules: self.model_rules.clone(),
            spacebase: self.spacebase.clone(),
            use_fillin_fallback: self.use_fillin_fallback,
        };
        res.populate_resolver();
        res
    }
}

/// A map from offset to a [`ParamEntry`] index (C++
/// `rangemap<ParamEntryRange>` = `ParamEntryResolver`).  The `ParamEntryRange`
/// record's `entry` pointer is modeled as an index into the owning entry vector.
type ParamEntryResolver = kuna_base::rangemap::RangeMap<ParamEntryRange>;

/// The record stored in a [`ParamEntryResolver`] (C++ `ParamEntryRange`,
/// `fspec.hh:159-192`).  Maps an interval `[first, last]` to the entry at index
/// `entry` within the owning `ParamListStandard::entry` vector, sub-sorted by
/// `position` (insertion order across the prototype list).
#[derive(Debug, Clone)]
pub struct ParamEntryRange {
    first: uintb,
    last: uintb,
    position: int4,
    entry: usize,
}

/// Initialization data for a [`ParamEntryRange`] (C++
/// `ParamEntryRange::InitData`).
pub struct ParamEntryRangeInit {
    position: int4,
    entry: usize,
}

impl kuna_base::rangemap::RangeRecord for ParamEntryRange {
    type LineType = uintb;
    // C++ SubsortPosition: position, with minimal=0 and maximal=1000000.
    type SubsortType = SubsortPosition;
    type InitType = ParamEntryRangeInit;

    fn create(data: ParamEntryRangeInit, a: uintb, b: uintb) -> ParamEntryRange {
        ParamEntryRange { first: a, last: b, position: data.position, entry: data.entry }
    }
    fn get_first(&self) -> uintb {
        self.first
    }
    fn get_last(&self) -> uintb {
        self.last
    }
    fn get_subsort(&self) -> SubsortPosition {
        SubsortPosition(self.position)
    }
}

/// Sub-sort key for [`ParamEntryRange`] (C++ `ParamEntryRange::SubsortPosition`,
/// `fspec.hh:174-181`): compare on `position`, with the minimal/maximal
/// sentinels being 0 and 1000000.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SubsortPosition(int4);

impl kuna_base::rangemap::Subsort for SubsortPosition {
    fn minimal() -> Self {
        SubsortPosition(0)
    }
    fn maximal() -> Self {
        SubsortPosition(1000000)
    }
}

impl ParamListStandard {
    /// Construct an empty `ParamListStandard` of the given kind.
    pub fn new(kind: ParamListKind) -> ParamListStandard {
        ParamListStandard {
            kind,
            numgroup: 0,
            maxdelay: 0,
            thisbeforeret: false,
            auto_killed_by_call: false,
            resource_start: Vec::new(),
            entry: Vec::new(),
            resolver_map: Vec::new(),
            model_rules: Vec::new(),
            spacebase: None,
            use_fillin_fallback: true,
        }
    }

    /// Get the list of parameter entries (C++ `getEntry`).
    pub fn get_entry(&self) -> &[ParamEntry] {
        &self.entry
    }

    /// Get the concrete model kind (C++ `getType`, projected to [`ParamListType`]).
    pub fn get_type(&self) -> ParamListType {
        match self.kind {
            ParamListKind::Standard => ParamListType::Standard,
            ParamListKind::StandardOut => ParamListType::StandardOut,
            ParamListKind::RegisterOut => ParamListType::RegisterOut,
            ParamListKind::Register => ParamListType::Register,
            ParamListKind::Merged => ParamListType::Merged,
        }
    }

    /// Return true if resources are big endian (C++ `isBigEndian`).
    pub fn is_big_endian(&self) -> bool {
        self.entry[0].get_space().is_big_endian()
    }

    /// Get the address space associated with any stack-based parameters (C++
    /// `getSpacebase`).
    pub fn get_spacebase(&self) -> Option<&Rc<AddrSpace>> {
        self.spacebase.as_ref()
    }

    /// Return true if the `this` pointer occurs before an indirect return
    /// pointer (C++ `isThisBeforeRetPointer`).
    pub fn is_this_before_ret_pointer(&self) -> bool {
        self.thisbeforeret
    }

    /// Return the maximum heritage delay across all parameters (C++ `getMaxDelay`).
    pub fn get_max_delay(&self) -> int4 {
        self.maxdelay
    }

    /// Return true if ParamEntry locations are automatically killed-by-call
    /// (C++ `isAutoKilledByCall`).
    pub fn is_auto_killed_by_call(&self) -> bool {
        self.auto_killed_by_call
    }

    /// Get registers of a given storage class (C++ `extractTiles`).  Passes back
    /// the indices of matching entries.
    pub fn extract_tiles(&self, tiles: &mut Vec<usize>, type_: type_class) {
        for (i, cur_entry) in self.entry.iter().enumerate() {
            if !cur_entry.is_exclusion() {
                continue;
            }
            if cur_entry.get_type() != type_ || cur_entry.get_all_groups().len() != 1 {
                continue;
            }
            tiles.push(i);
        }
    }

    /// Get the stack entry index, or `None` (C++ `getStackEntry`).
    pub fn get_stack_entry(&self) -> Option<usize> {
        if let Some(last) = self.entry.last() {
            if !last.is_exclusion() && last.get_space().get_type() == spacetype::IPTR_SPACEBASE {
                return Some(self.entry.len() - 1);
            }
        }
        None
    }

    /// Find the (first) entry containing the given memory range (C++
    /// `findEntry`).  Returns the index of the matching entry, or `None`.
    pub fn find_entry(&self, loc: &Address, size: int4, just: bool) -> Option<usize> {
        let space = loc.get_space()?;
        let index = space.get_index();
        if index < 0 || (index as usize) >= self.resolver_map.len() {
            return None;
        }
        let resolver = self.resolver_map[index as usize].as_ref()?;
        let mut iter = resolver.find(loc.get_offset());
        for ridx in iter.by_ref() {
            let test_idx = resolver.record(ridx).entry;
            let test_entry = &self.entry[test_idx];
            if test_entry.get_min_size() > size {
                continue;
            }
            if !just || test_entry.justified_contain(loc, size) == 0 {
                return Some(test_idx);
            }
        }
        None
    }

    /// Select the entry from `grp` that best matches `pref_type` (C++
    /// `selectUnreferenceEntry`).  Returns the index of the best entry.
    pub fn select_unreference_entry(&self, grp: int4, pref_type: type_class) -> Option<usize> {
        let mut best_score = -1;
        let mut best_entry: Option<usize> = None;
        for (i, cur_entry) in self.entry.iter().enumerate() {
            if cur_entry.get_group() != grp {
                continue;
            }
            let cur_score = if cur_entry.get_type() == pref_type {
                2
            } else if pref_type == type_class::TYPECLASS_GENERAL {
                1
            } else {
                0
            };
            if cur_score > best_score {
                best_score = cur_score;
                best_entry = Some(i);
            }
        }
        best_entry
    }

    /// Characterize whether the given range overlaps parameter storage (C++
    /// `characterizeAsParam`).
    pub fn characterize_as_param(&self, loc: &Address, size: int4) -> Containment {
        let space = match loc.get_space() {
            Some(s) => s,
            None => return Containment::NoContainment,
        };
        let index = space.get_index();
        if index < 0 || (index as usize) >= self.resolver_map.len() {
            return Containment::NoContainment;
        }
        let resolver = match self.resolver_map[index as usize].as_ref() {
            Some(r) => r,
            None => return Containment::NoContainment,
        };
        let mut res_contains = false;
        let mut res_contained_by = false;
        let mut iter = resolver.find(loc.get_offset());
        for ridx in iter.by_ref() {
            let test_entry = &self.entry[resolver.record(ridx).entry];
            let off = test_entry.justified_contain(loc, size);
            if off == 0 {
                return Containment::ContainsJustified;
            } else if off > 0 {
                res_contains = true;
            }
            if test_entry.is_exclusion() && test_entry.contained_by(loc, size) {
                res_contained_by = true;
            }
        }
        if res_contains {
            return Containment::ContainsUnjustified;
        }
        if res_contained_by {
            return Containment::ContainedBy;
        }
        // Second pass: the range may contain an entry whose start is past loc.
        // C++ continues from where the first `find()` ended; we re-derive the
        // window via find_begin(loc) .. find_end(loc + size - 1).
        let begin = resolver.find_begin(loc.get_offset());
        let endpoint = loc.get_offset().wadd((size - 1) as i64 as u64);
        let end = resolver.find_end(endpoint);
        let mut iter2 = resolver.iter_between(&begin, &end);
        for ridx in iter2.by_ref() {
            let test_entry = &self.entry[resolver.record(ridx).entry];
            if test_entry.is_exclusion() && test_entry.contained_by(loc, size) {
                return Containment::ContainedBy;
            }
        }
        Containment::NoContainment
    }

    /// Does the given storage location make sense as a parameter (C++
    /// `possibleParam`).  Dispatches on `kind` for the output models.
    pub fn possible_param(&self, loc: &Address, size: int4) -> bool {
        match self.kind {
            ParamListKind::StandardOut | ParamListKind::RegisterOut => {
                // ParamListStandardOut::possibleParam
                self.entry.iter().any(|e| e.justified_contain(loc, size) >= 0)
            }
            _ => self.find_entry(loc, size, true).is_some(),
        }
    }

    /// Pass back the slot and slot size for the given storage location (C++
    /// `possibleParamWithSlot`).
    pub fn possible_param_with_slot(
        &self,
        loc: &Address,
        size: int4,
        slot: &mut int4,
        slotsize: &mut int4,
    ) -> bool {
        let idx = match self.find_entry(loc, size, true) {
            Some(i) => i,
            None => return false,
        };
        let entry_num = &self.entry[idx];
        *slot = entry_num.get_slot(loc, 0);
        if entry_num.is_exclusion() {
            *slotsize = entry_num.get_all_groups().len() as i32; // cast: size() -> int4
        } else {
            *slotsize = ((size - 1) / entry_num.get_align()) + 1;
        }
        true
    }

    /// Pass back the biggest parameter contained within the given range (C++
    /// `getBiggestContainedParam`).
    pub fn get_biggest_contained_param(
        &self,
        loc: &Address,
        size: int4,
        res: &mut VarnodeData,
    ) -> bool {
        let space = match loc.get_space() {
            Some(s) => s,
            None => return false,
        };
        let index = space.get_index();
        if index < 0 || (index as usize) >= self.resolver_map.len() {
            return false;
        }
        let resolver = match self.resolver_map[index as usize].as_ref() {
            Some(r) => r,
            None => return false,
        };
        let end_loc = loc + ((size - 1) as i64);
        if end_loc.get_offset() < loc.get_offset() {
            return false; // wrapping
        }
        let mut max_entry: Option<usize> = None;
        let begin = resolver.find_begin(loc.get_offset());
        let end = resolver.find_end(end_loc.get_offset());
        let mut iter = resolver.iter_between(&begin, &end);
        for ridx in iter.by_ref() {
            let test_idx = resolver.record(ridx).entry;
            let test_entry = &self.entry[test_idx];
            if test_entry.contained_by(loc, size) {
                match max_entry {
                    None => max_entry = Some(test_idx),
                    Some(m) if test_entry.get_size() > self.entry[m].get_size() => {
                        max_entry = Some(test_idx)
                    }
                    _ => {}
                }
            }
        }
        if let Some(m) = max_entry {
            let me = &self.entry[m];
            if !me.is_exclusion() {
                return false;
            }
            res.space = Some(Rc::clone(me.get_space()));
            res.offset = me.get_base();
            res.size = me.get_size() as u32; // cast: int4 -> uint4 member
            return true;
        }
        false
    }

    /// Check if the given storage looks like an unjustified parameter (C++
    /// `unjustifiedContainer`).
    pub fn unjustified_container(&self, loc: &Address, size: int4, res: &mut VarnodeData) -> bool {
        for e in self.entry.iter() {
            if e.get_min_size() > size {
                continue;
            }
            let just = e.justified_contain(loc, size);
            if just < 0 {
                continue;
            }
            if just == 0 {
                return false;
            }
            e.get_container(loc, size, res);
            return true;
        }
        false
    }

    /// Get the type of extension and containing parameter for the given storage
    /// (C++ `assumedExtension`).
    pub fn assumed_extension(&self, addr: &Address, size: int4, res: &mut VarnodeData) -> OpCode {
        for e in self.entry.iter() {
            if e.get_min_size() > size {
                continue;
            }
            let ext = e.assumed_extension(addr, size, res);
            if ext != OpCode::CPUI_COPY {
                return ext;
            }
        }
        OpCode::CPUI_COPY
    }

    /// Collect all parameter locations within the given address space (C++
    /// `getRangeList`).
    pub fn get_range_list(&self, spc: &Rc<AddrSpace>, res: &mut RangeList) {
        for e in self.entry.iter() {
            if !Rc::ptr_eq(e.get_space(), spc) {
                continue;
            }
            let baseoff = e.get_base();
            let endoff = baseoff.wadd((e.get_size() - 1) as i64 as u64);
            res.insert_range(Rc::clone(spc), baseoff, endoff);
        }
    }

    /// Check if the two storage locations can represent a single logical
    /// parameter (C++ `checkJoin`).
    pub fn check_join(
        &self,
        hiaddr: &Address,
        hisize: int4,
        loaddr: &Address,
        losize: int4,
    ) -> bool {
        let entry_hi = match self.find_entry(hiaddr, hisize, true) {
            Some(i) => i,
            None => return false,
        };
        let entry_lo = match self.find_entry(loaddr, losize, true) {
            Some(i) => i,
            None => return false,
        };
        let e_hi = &self.entry[entry_hi];
        let e_lo = &self.entry[entry_lo];
        if e_hi.get_group() == e_lo.get_group() {
            if e_hi.is_exclusion() || e_lo.is_exclusion() {
                return false;
            }
            if !hiaddr.is_contiguous(hisize, loaddr, losize) {
                return false;
            }
            if !(hiaddr.get_offset().wsub(e_hi.get_base())).is_multiple_of(e_hi.get_align() as u64) {
                return false;
            }
            if !(loaddr.get_offset().wsub(e_lo.get_base())).is_multiple_of(e_lo.get_align() as u64) {
                return false;
            }
            true
        } else {
            let sizesum = hisize + losize;
            for e in self.entry.iter() {
                if e.get_size() < sizesum {
                    continue;
                }
                if e.justified_contain(loaddr, losize) != 0 {
                    continue;
                }
                if e.justified_contain(hiaddr, hisize) != losize {
                    continue;
                }
                return true;
            }
            false
        }
    }

    /// Check if it makes sense to split a single storage location into two
    /// parameters (C++ `checkSplit`).
    pub fn check_split(&self, loc: &Address, size: int4, splitpoint: int4) -> bool {
        let loc2 = loc + (splitpoint as i64);
        let size2 = size - splitpoint;
        if self.find_entry(loc, splitpoint, true).is_none() {
            return false;
        }
        if self.find_entry(&loc2, size2, true).is_none() {
            return false;
        }
        true
    }

    /// Calculate the maximum heritage delay for any potential parameter (C++
    /// `calcDelay`).
    pub fn calc_delay(&mut self) {
        self.maxdelay = 0;
        for e in self.entry.iter() {
            let delay = e.get_space().get_delay();
            if delay > self.maxdelay {
                self.maxdelay = delay;
            }
        }
    }

    /// Add a single address range to the resolver maps (C++ `addResolverRange`).
    fn add_resolver_range(
        &mut self,
        spc: &Rc<AddrSpace>,
        first: uintb,
        last: uintb,
        param_entry: usize,
        position: int4,
    ) {
        let index = spc.get_index();
        let index = if index < 0 { 0 } else { index as usize };
        while self.resolver_map.len() <= index {
            self.resolver_map.push(None);
        }
        if self.resolver_map[index].is_none() {
            self.resolver_map[index] = Some(ParamEntryResolver::new());
        }
        let resolver = self.resolver_map[index].as_mut().unwrap();
        resolver.insert(
            ParamEntryRangeInit { position, entry: param_entry },
            first,
            last,
        );
    }

    /// Build the ParamEntry resolver maps (C++ `populateResolver`).
    pub fn populate_resolver(&mut self) {
        self.resolver_map.clear();
        let mut position = 0;
        // Collect the resolver insertions first (immutable borrow of entry),
        // then apply them (mutable borrow of resolver_map).
        struct Ins {
            spc: Rc<AddrSpace>,
            first: uintb,
            last: uintb,
            entry: usize,
            position: int4,
        }
        let mut inserts: Vec<Ins> = Vec::new();
        for (i, param_entry) in self.entry.iter().enumerate() {
            let spc = param_entry.get_space();
            if spc.get_type() == spacetype::IPTR_JOIN {
                let join_rec = param_entry
                    .get_join_record()
                    .expect("join entry without join record");
                for k in 0..join_rec.num_pieces() {
                    let vdata = join_rec.get_piece(k);
                    let last = vdata.offset.wadd((vdata.size as i64 as u64).wsub(1));
                    inserts.push(Ins {
                        spc: vdata.space.clone().expect("join piece null space"),
                        first: vdata.offset,
                        last,
                        entry: i,
                        position,
                    });
                    position += 1;
                }
            } else {
                let first = param_entry.get_base();
                let last = first.wadd((param_entry.get_size() - 1) as i64 as u64);
                inserts.push(Ins { spc: Rc::clone(spc), first, last, entry: i, position });
                position += 1;
            }
        }
        for ins in inserts {
            self.add_resolver_range(&ins.spc, ins.first, ins.last, ins.entry, ins.position);
        }
    }

    /// Assign storage for a parameter class using the fallback algorithm (C++
    /// `assignAddressFallback`).
    pub fn assign_address_fallback(
        &self,
        resource: type_class,
        tp: &Rc<Datatype>,
        match_exact: bool,
        status: &mut [int4],
        param: &mut ParameterPieces,
        manager: &AddrSpaceManager,
    ) -> KunaResult<AssignActionResponse> {
        for cur_entry in self.entry.iter() {
            let grp = cur_entry.get_group();
            if status[grp as usize] < 0 {
                continue;
            }
            if resource != cur_entry.get_type()
                && (match_exact || cur_entry.get_type() != type_class::TYPECLASS_GENERAL)
            {
                continue; // Wrong type
            }
            param.addr = cur_entry.get_addr_by_slot(
                &mut status[grp as usize],
                tp.get_align_size(),
                tp.get_alignment(),
                manager,
            )?;
            if param.addr.is_invalid() {
                continue; // If -tp- doesn't fit
            }
            if cur_entry.is_exclusion() {
                for &g in cur_entry.get_all_groups() {
                    status[g as usize] = -1; // some groups are taken up
                }
            }
            param.type_ = Some(Rc::clone(tp));
            param.flags = 0;
            return Ok(AssignActionResponse::success);
        }
        Ok(AssignActionResponse::fail)
    }

    /// Fill in the Address and details for the given parameter (C++
    /// `assignAddress`).  With no model rules (the current boundary state) this
    /// falls straight through to the fallback.  // STUB(w6-modelrules)
    #[allow(clippy::too_many_arguments)] // mirrors C++ ParamListStandard::assignAddress
    pub fn assign_address(
        &self,
        dt: &Rc<Datatype>,
        proto: &PrototypePieces,
        pos: int4,
        tlist: &dyn TypeFactory,
        status: &mut [int4],
        res: &mut ParameterPieces,
        manager: &AddrSpaceManager,
    ) -> KunaResult<AssignActionResponse> {
        // C++ ParamListStandard::assignAddress (fspec.cc:783-792): try each
        // ModelRule in order; the first non-fail response wins.  Only when every
        // rule returns `fail` do we fall through to the metatype-keyed fallback.
        for rule in self.model_rules.iter() {
            let response_code =
                rule.assign_address(dt, proto, pos, tlist, status, res, self, manager)?;
            if response_code != AssignActionResponse::fail {
                return Ok(response_code);
            }
        }
        let store = metatype2typeclass(dt.get_metatype());
        self.assign_address_fallback(store, dt, false, status, res, manager)
    }

    /// Map a list of data-types to storage locations (C++ `assignMap`).
    /// Dispatches on `kind` for the output variants.
    pub fn assign_map(
        &self,
        proto: &PrototypePieces,
        typefactory: &dyn TypeFactory,
        res: &mut Vec<ParameterPieces>,
        manager: &AddrSpaceManager,
    ) -> KunaResult<()> {
        match self.kind {
            ParamListKind::Merged => Err(KunaError::lowlevel(
                "Cannot assign prototype before model has been resolved",
            )),
            ParamListKind::Standard | ParamListKind::Register => {
                self.assign_map_standard(proto, typefactory, res, manager)
            }
            ParamListKind::StandardOut => self.assign_map_standard_out(proto, typefactory, res, manager),
            ParamListKind::RegisterOut => self.assign_map_register_out(proto, typefactory, res, manager),
        }
    }

    /// `ParamListStandard::assignMap`.
    fn assign_map_standard(
        &self,
        proto: &PrototypePieces,
        typefactory: &dyn TypeFactory,
        res: &mut Vec<ParameterPieces>,
        manager: &AddrSpaceManager,
    ) -> KunaResult<()> {
        let mut status = vec![0i32; self.numgroup as usize];
        if res.len() == 2 {
            // Hidden parameters defined by the output list.
            let is_hidden = (res[1].flags & parameter_pieces_flags::HIDDENRETPARM) != 0;
            let dt = res[1].type_.clone().expect("hidden ret type null");
            if is_hidden {
                let mut back = res.pop().unwrap();
                let r = self.assign_address_fallback(
                    type_class::TYPECLASS_HIDDENRET,
                    &dt,
                    false,
                    &mut status,
                    &mut back,
                    manager,
                )?;
                res.push(back);
                if r == AssignActionResponse::fail {
                    return Err(unassigned_err(&dt));
                }
            } else {
                let mut back = res.pop().unwrap();
                let r = self.assign_address(&dt, proto, 0, typefactory, &mut status, &mut back, manager)?;
                res.push(back);
                if r == AssignActionResponse::fail {
                    return Err(unassigned_err(&dt));
                }
            }
            res[1].flags |= parameter_pieces_flags::HIDDENRETPARM;
        }
        for i in 0..proto.intypes.len() {
            let dt = Rc::clone(&proto.intypes[i]);
            let mut back = ParameterPieces::default();
            let response =
                self.assign_address(&dt, proto, i as i32, typefactory, &mut status, &mut back, manager)?;
            res.push(back);
            if response == AssignActionResponse::fail || response == AssignActionResponse::no_assignment {
                return Err(unassigned_err(&dt));
            }
        }
        Ok(())
    }

    /// `ParamListRegisterOut::assignMap`.
    fn assign_map_register_out(
        &self,
        proto: &PrototypePieces,
        typefactory: &dyn TypeFactory,
        res: &mut Vec<ParameterPieces>,
        manager: &AddrSpaceManager,
    ) -> KunaResult<()> {
        let mut status = vec![0i32; self.numgroup as usize];
        let mut back = ParameterPieces::default();
        let outtype = proto.outtype.clone().expect("outtype null");
        if outtype.get_metatype() != type_metatype::TYPE_VOID {
            self.assign_address(&outtype, proto, -1, typefactory, &mut status, &mut back, manager)?;
            if back.addr.is_invalid() {
                return Err(unassigned_err(&outtype));
            }
        } else {
            back.type_ = Some(outtype);
            back.flags = 0;
        }
        res.push(back);
        Ok(())
    }

    /// `ParamListStandardOut::assignMap` (fspec.cc:1571-1614).
    ///
    /// The common (assignable) path and the void path are ported faithfully.
    /// The hidden-return path (too-big return value: the output is returned via
    /// a hidden pointer parameter) is now wired through the `AddrSpaceManager`'s
    /// default data space (C++ `typefactory.getArch()->getDefaultDataSpace()`)
    /// and the type factory's `getTypePointer` — closing the w10-struct-return
    /// boundary so struct-returning prototypes apply instead of graceful-degrading.
    fn assign_map_standard_out(
        &self,
        proto: &PrototypePieces,
        typefactory: &dyn TypeFactory,
        res: &mut Vec<ParameterPieces>,
        manager: &AddrSpaceManager,
    ) -> KunaResult<()> {
        let mut status = vec![0i32; self.numgroup as usize];
        let mut back = ParameterPieces::default();
        let outtype = proto.outtype.clone().expect("outtype null");
        if outtype.get_metatype() == type_metatype::TYPE_VOID {
            back.type_ = Some(outtype);
            back.flags = 0;
            res.push(back);
            return Ok(()); // Leave the address as invalid
        }
        let mut response =
            self.assign_address(&outtype, proto, -1, typefactory, &mut status, &mut back, manager)?;
        if response == AssignActionResponse::fail {
            // Invoke default hidden return input assignment action.
            response = AssignActionResponse::hiddenret_ptrparam;
        }
        if response == AssignActionResponse::hiddenret_ptrparam
            || response == AssignActionResponse::hiddenret_specialreg
            || response == AssignActionResponse::hiddenret_specialreg_void
        {
            // Could not assign an address (too big): the return value is passed
            // back through a hidden pointer parameter (C++ fspec.cc:1589-1612).
            let spc = match self.spacebase.clone() {
                Some(s) => s,
                None => Rc::clone(manager.get_default_data_space().ok_or_else(|| {
                    KunaError::lowlevel(
                        "ParamListStandardOut::assignMap: no default data space for hidden return",
                    )
                })?),
            };
            let pointersize = spc.get_addr_size() as int4;
            let wordsize = spc.get_word_size();
            let pointertp =
                typefactory.get_type_pointer(pointersize, outtype.clone(), wordsize)?;
            if response == AssignActionResponse::hiddenret_specialreg_void {
                back.type_ = Some(typefactory.get_type_void()?);
            } else {
                back.type_ = Some(pointertp.clone());
                // C++ assignAddress(pointertp,...,res.back()): writes the
                // resolved storage address (and re-sets type/flags) onto `back`.
                if self.assign_address(
                    &pointertp,
                    proto,
                    -1,
                    typefactory,
                    &mut status,
                    &mut back,
                    manager,
                )? == AssignActionResponse::fail
                {
                    // C++ fspec.cc:1601 `throw ParamUnassignedError(...)`: a
                    // dedicated LowlevelError subclass caught by
                    // ProtoModel::assignParameterStorage(ignoreOutputError=true)
                    // (fspec.cc:2441) to degrade the output to void.
                    return Err(KunaError::param_unassigned(
                        "Cannot assign return value as a pointer",
                    ));
                }
            }
            back.flags = parameter_pieces_flags::INDIRECTSTORAGE;
            res.push(back);

            // Add extra storage location in the input params that holds a pointer
            // to where the return value should be stored.  Leave its address
            // invalid, to be filled in by the input list assignMap.  Encode
            // whether or not the hidden return should be drawn from
            // TYPECLASS_HIDDENRET.
            let is_special = response == AssignActionResponse::hiddenret_specialreg
                || response == AssignActionResponse::hiddenret_specialreg_void;
            res.push(ParameterPieces {
                type_: Some(pointertp),
                flags: if is_special { parameter_pieces_flags::HIDDENRETPARM } else { 0 },
                ..Default::default()
            });
            return Ok(());
        }
        res.push(back);
        Ok(())
    }

    // -- fillinMap family (fspec.cc:851-1315, 1544-1765) --------------------

    /// Build the map from parameter trials to model ParamEntrys (C++
    /// `buildTrialMap`).
    fn build_trial_map(&self, active: &mut ParamActive, manager: &AddrSpaceManager) -> KunaResult<()> {
        let mut hitlist: Vec<Option<usize>> = Vec::new();
        let mut float_count = 0;
        let mut int_count = 0;

        for i in 0..active.get_num_trials() {
            let (addr, size, is_active) = {
                let pt = active.get_trial(i);
                (pt.get_address().clone(), pt.get_size(), pt.is_active())
            };
            let entry_slot = self.find_entry(&addr, size, true);
            match entry_slot {
                None => active.get_trial_mut(i).mark_no_use(),
                Some(eidx) => {
                    active.get_trial_mut(i).set_entry(Some(eidx), 0);
                    if is_active {
                        if self.entry[eidx].get_type() == type_class::TYPECLASS_FLOAT {
                            float_count += 1;
                        } else {
                            int_count += 1;
                        }
                    }
                    let grp = self.entry[eidx].get_group();
                    while (hitlist.len() as i32) <= grp {
                        hitlist.push(None);
                    }
                    if hitlist[grp as usize].is_none() {
                        hitlist[grp as usize] = Some(eidx);
                    }
                }
            }
        }

        // Fill in unreferenced trials for missing groups.  `i` is the group
        // index (passed to selectUnreferenceEntry), not just a position.
        #[allow(clippy::needless_range_loop)]
        for i in 0..hitlist.len() {
            match hitlist[i] {
                None => {
                    let pref = if float_count > int_count {
                        type_class::TYPECLASS_FLOAT
                    } else {
                        type_class::TYPECLASS_GENERAL
                    };
                    let curentry = match self.select_unreference_entry(i as i32, pref) {
                        Some(c) => c,
                        None => continue,
                    };
                    let ce = &self.entry[curentry];
                    let sz = if ce.is_exclusion() { ce.get_size() } else { ce.get_align() };
                    let mut nextslot = 0;
                    let addr = ce.get_addr_by_slot(&mut nextslot, sz, 1, manager)?;
                    let trialpos = active.get_num_trials();
                    active.register_trial(&addr, sz);
                    let pt = active.get_trial_mut(trialpos);
                    pt.mark_unref();
                    pt.set_entry(Some(curentry), 0);
                }
                Some(curentry) if !self.entry[curentry].is_exclusion() => {
                    // Non-exclusion group: build a slot hitlist to find holes.
                    let mut slotlist: Vec<i32> = Vec::new();
                    for j in 0..active.get_num_trials() {
                        let (paddr, psize, pentry) = {
                            let pt = active.get_trial(j);
                            (pt.get_address().clone(), pt.get_size(), pt.get_entry())
                        };
                        if pentry != Some(curentry) {
                            continue;
                        }
                        let ce = &self.entry[curentry];
                        let mut slot = ce.get_slot(&paddr, 0) - ce.get_group();
                        let mut endslot = ce.get_slot(&paddr, psize - 1) - ce.get_group();
                        if endslot < slot {
                            std::mem::swap(&mut slot, &mut endslot);
                        }
                        while (slotlist.len() as i32) <= endslot {
                            slotlist.push(0);
                        }
                        let mut s = slot;
                        while s <= endslot {
                            slotlist[s as usize] = 1;
                            s += 1;
                        }
                    }
                    // `j` is the slot index (becomes nextslot for getAddrBySlot).
                    #[allow(clippy::needless_range_loop)]
                    for j in 0..slotlist.len() {
                        if slotlist[j] == 0 {
                            let ce = &self.entry[curentry];
                            let mut nextslot = j as i32;
                            let align = ce.get_align();
                            let addr = ce.get_addr_by_slot(&mut nextslot, align, 1, manager)?;
                            let trialpos = active.get_num_trials();
                            active.register_trial(&addr, align);
                            let pt = active.get_trial_mut(trialpos);
                            pt.mark_unref();
                            pt.set_entry(Some(curentry), 0);
                        }
                    }
                }
                Some(_) => {}
            }
        }
        active.sort_trials(&self.entry);
        Ok(())
    }

    /// Calculate the range of trials in each resource section (C++
    /// `separateSections`).
    fn separate_sections(&self, active: &ParamActive, trial_start: &mut Vec<int4>) -> KunaResult<()> {
        let numtrials = active.get_num_trials();
        let mut next_group = self.resource_start[1];
        let mut next_section = 2usize;
        trial_start.push(0);
        for current_trial in 0..numtrials {
            let curtrial = active.get_trial(current_trial);
            let entry = match curtrial.get_entry() {
                Some(e) => e,
                None => continue,
            };
            if self.entry[entry].get_group() >= next_group {
                if next_section > self.resource_start.len() {
                    return Err(KunaError::lowlevel("Missing next resource start"));
                }
                next_group = self.resource_start[next_section];
                next_section += 1;
                trial_start.push(current_trial);
            }
        }
        trial_start.push(numtrials);
        Ok(())
    }

    /// Mark all trials within the indicated groups as not-used, except for one
    /// (C++ `markGroupNoUse`).
    fn mark_group_no_use(&self, active: &mut ParamActive, active_trial: int4, trial_start: int4) {
        let num_trials = active.get_num_trials();
        let active_entry = active.get_trial(active_trial).get_entry().expect("null entry");
        for i in trial_start..num_trials {
            if i == active_trial {
                continue;
            }
            if active.get_trial(i).is_definitely_not_used() {
                continue;
            }
            let other_entry = active.get_trial(i).get_entry().expect("null entry");
            if !self.entry[other_entry].group_overlap(&self.entry[active_entry]) {
                break;
            }
            active.get_trial_mut(i).mark_no_use();
        }
    }

    /// From among multiple inactive trials, select the most likely active and
    /// mark others not-used (C++ `markBestInactive`).
    fn mark_best_inactive(
        &self,
        active: &mut ParamActive,
        group: int4,
        group_start: int4,
        pref_type: type_class,
    ) {
        let num_trials = active.get_num_trials();
        let mut best_trial = -1;
        let mut best_score = -1;
        for i in group_start..num_trials {
            let trial = active.get_trial(i);
            if trial.is_definitely_not_used() {
                continue;
            }
            let entry = &self.entry[trial.get_entry().expect("null entry")];
            if entry.get_group() != group {
                break;
            }
            if entry.get_all_groups().len() > 1 {
                continue; // Covering multiple slots -> low score
            }
            let mut score = 0;
            if trial.has_ancestor_realistic() {
                score += 5;
                if trial.has_ancestor_solid() {
                    score += 5;
                }
            }
            if entry.get_type() == pref_type {
                score += 1;
            }
            if score > best_score {
                best_score = score;
                best_trial = i;
            }
        }
        if best_trial >= 0 {
            self.mark_group_no_use(active, best_trial, group_start);
        }
    }

    /// Enforce exclusion rules for the given set of trials (C++
    /// `forceExclusionGroup`).
    fn force_exclusion_group(&self, active: &mut ParamActive) {
        let num_trials = active.get_num_trials();
        let mut cur_group = -1;
        let mut group_start = -1;
        let mut inactive_count = 0;
        for i in 0..num_trials {
            let (defnouse, exclusion, grp, is_act) = {
                let curtrial = active.get_trial(i);
                match curtrial.get_entry() {
                    None => (curtrial.is_definitely_not_used(), false, -1, false),
                    Some(e) => (
                        curtrial.is_definitely_not_used(),
                        self.entry[e].is_exclusion(),
                        self.entry[e].get_group(),
                        curtrial.is_active(),
                    ),
                }
            };
            if defnouse || !exclusion {
                continue;
            }
            if grp != cur_group {
                if inactive_count > 1 {
                    self.mark_best_inactive(active, cur_group, group_start, type_class::TYPECLASS_GENERAL);
                }
                cur_group = grp;
                group_start = i;
                inactive_count = 0;
            }
            if is_act {
                self.mark_group_no_use(active, i, group_start);
            } else {
                inactive_count += 1;
            }
        }
        if inactive_count > 1 {
            self.mark_best_inactive(active, cur_group, group_start, type_class::TYPECLASS_GENERAL);
        }
    }

    /// Mark every trial above the first "definitely not used" as inactive (C++
    /// `forceNoUse`).
    fn force_no_use(&self, active: &mut ParamActive, start: int4, stop: int4) {
        let mut seendefnouse = false;
        let mut curgroup = -1;
        let mut alldefnouse = false;
        for i in start..stop {
            let (entry, defnouse) = {
                let curtrial = active.get_trial(i);
                (curtrial.get_entry(), curtrial.is_definitely_not_used())
            };
            let entry = match entry {
                Some(e) => e,
                None => continue, // Already marked as not used
            };
            let grp = self.entry[entry].get_group();
            let exclusion = self.entry[entry].is_exclusion();
            if grp <= curgroup && exclusion {
                // Same exclusion group
                if !defnouse {
                    alldefnouse = false;
                }
            } else {
                if alldefnouse {
                    seendefnouse = true;
                }
                alldefnouse = defnouse;
                curgroup = grp;
            }
            if seendefnouse {
                active.get_trial_mut(i).mark_inactive();
            }
        }
    }

    /// Enforce rules about chains of inactive slots (C++ `forceInactiveChain`).
    fn force_inactive_chain(
        &self,
        active: &mut ParamActive,
        maxchain: int4,
        start: int4,
        stop: int4,
        groupstart: int4,
    ) {
        let mut seenchain = false;
        let mut chainlength = 0;
        let mut max = -1;
        for i in start..stop {
            let (defnouse, is_act, is_unref, addr_is_spacebase, slotgrp, protected) = {
                let trial = active.get_trial(i);
                let addr_sb = trial
                    .get_address()
                    .get_space()
                    .map(|s| s.get_type() == spacetype::IPTR_SPACEBASE)
                    .unwrap_or(false);
                (
                    trial.is_definitely_not_used(),
                    trial.is_active(),
                    trial.is_unref(),
                    addr_sb,
                    if trial.get_entry().is_some() {
                        trial.slot_group(&self.entry)
                    } else {
                        0
                    },
                    // (kuna) `inputparamgap`: in the function's OWN input
                    // recovery an unused ARGUMENT REGISTER is an ignored
                    // parameter, not evidence that a later REGISTER the body
                    // reads before writing is not a parameter.
                    crate::p4_calls::kuna_inputparamgap::trial_is_protected(
                        active,
                        trial,
                        &self.entry,
                    ),
                )
            };
            if defnouse {
                continue;
            }
            if !is_act {
                if is_unref && active.is_recover_subcall() && addr_is_spacebase {
                    seenchain = true;
                }
                if i == start {
                    chainlength += slotgrp - groupstart + 1;
                } else {
                    let prev_slotgrp = {
                        let pt = active.get_trial(i - 1);
                        if pt.get_entry().is_some() {
                            pt.slot_group(&self.entry)
                        } else {
                            0
                        }
                    };
                    chainlength += slotgrp - prev_slotgrp;
                }
                if chainlength > maxchain {
                    seenchain = true;
                }
            } else {
                chainlength = 0;
                // (kuna) `inputparamgap`: a protected trial is a REGISTER the
                // function's own body reads before writing, and the chain does
                // not get to veto it.  Trials sort into parameter order, so
                // everything before it is also a register and the tail loop's
                // hole-filling stays inside the argument-register file.
                if !seenchain || protected {
                    max = i;
                }
            }
            if seenchain && !protected {
                active.get_trial_mut(i).mark_inactive();
            }
        }
        for i in start..=max {
            let (defnouse, is_act) = {
                let trial = active.get_trial(i);
                (trial.is_definitely_not_used(), trial.is_active())
            };
            if defnouse {
                continue;
            }
            if !is_act {
                active.get_trial_mut(i).mark_active();
            }
        }
    }

    /// Given an unordered list of trials, calculate the formal prototype (C++
    /// `fillinMap` / subclasses).  Dispatches on `kind`.
    pub fn fillin_map(&self, active: &mut ParamActive, manager: &AddrSpaceManager) -> KunaResult<()> {
        match self.kind {
            ParamListKind::Merged => Err(KunaError::lowlevel(
                "Cannot determine prototype before model has been resolved",
            )),
            ParamListKind::Standard => self.fillin_map_standard(active, manager),
            ParamListKind::Register => {
                self.fillin_map_register(active);
                Ok(())
            }
            ParamListKind::StandardOut | ParamListKind::RegisterOut => {
                self.fillin_map_standard_out(active);
                Ok(())
            }
        }
    }

    /// `ParamListStandard::fillinMap`.
    fn fillin_map_standard(&self, active: &mut ParamActive, manager: &AddrSpaceManager) -> KunaResult<()> {
        if active.get_num_trials() == 0 {
            return Ok(());
        }
        if self.entry.is_empty() {
            return Err(KunaError::lowlevel(
                "Cannot derive parameter storage for prototype model without parameter entries",
            ));
        }
        self.build_trial_map(active, manager)?;
        self.force_exclusion_group(active);
        let mut trial_start: Vec<int4> = Vec::new();
        self.separate_sections(active, &mut trial_start)?;
        // (kuna) `varargstackargs`: at a variadic call site the register file
        // between the last fixed parameter and the first stack argument is
        // structurally empty, so the stack tail of a section is scored as its
        // own section.  Off (and for every non-variadic call) this leaves
        // `trial_start`/`resource_start` exactly as `separate_sections` built
        // them.  See [`crate::p4_calls::kuna_varargstackargs`].
        let mut group_start: Vec<int4> =
            (0..trial_start.len() - 1).map(|i| self.resource_start[i]).collect();
        let mut sec = 0usize;
        while sec + 1 < trial_start.len() {
            if let Some(cut) = crate::p4_calls::kuna_varargstackargs::stack_section_split(
                active,
                trial_start[sec],
                trial_start[sec + 1],
            ) {
                let cut_group = active.get_trial(cut).slot_group(&self.entry);
                trial_start.insert(sec + 1, cut);
                group_start.insert(sec + 1, cut_group);
                sec += 1; // the tail is all-stack; it never splits again
            }
            sec += 1;
        }
        let num_section = trial_start.len() - 1;
        for i in 0..num_section {
            self.force_no_use(active, trial_start[i], trial_start[i + 1]);
        }
        for i in 0..num_section {
            self.force_inactive_chain(active, 2, trial_start[i], trial_start[i + 1], group_start[i]);
        }
        for i in 0..active.get_num_trials() {
            if active.get_trial(i).is_active() {
                active.get_trial_mut(i).mark_used();
            }
        }
        Ok(())
    }

    /// `ParamListRegister::fillinMap`.
    fn fillin_map_register(&self, active: &mut ParamActive) {
        if active.get_num_trials() == 0 {
            return;
        }
        for i in 0..active.get_num_trials() {
            let (addr, size, is_act) = {
                let pt = active.get_trial(i);
                (pt.get_address().clone(), pt.get_size(), pt.is_active())
            };
            match self.find_entry(&addr, size, true) {
                None => active.get_trial_mut(i).mark_no_use(),
                Some(eidx) => {
                    let pt = active.get_trial_mut(i);
                    pt.set_entry(Some(eidx), 0);
                    if is_act {
                        pt.mark_used();
                    }
                }
            }
        }
        active.sort_trials(&self.entry);
    }

    /// `ParamListStandardOut::fillinMap` (C++ fspec.cc:1721-1763).
    ///
    /// When `use_fillin_fallback` is set (no output `<rule>`), dispatch to the
    /// legacy fallback.  Otherwise drive the decoded ModelRules: tag each active
    /// trial with the `ParamEntry` it lands in, then ask each rule's
    /// `fillinOutputMap` whether the trials form that rule's output storage.  The
    /// first rule that matches marks the covered active trials used (the SPARC
    /// `<join storage="general">` joins the o0:o1 return pair so both registers
    /// are kept used and `buildReturnOutput` emits the CONCAT44 join).  If no
    /// rule matches, fall back with `first_only == true`.
    fn fillin_map_standard_out(&self, active: &mut ParamActive) {
        if active.get_num_trials() == 0 {
            return;
        }
        if self.use_fillin_fallback {
            self.fillin_map_fallback(active, false);
            return;
        }
        for i in 0..active.get_num_trials() {
            let (addr, size, is_act, rem_or_ind) = {
                let trial = active.get_trial(i);
                (
                    trial.get_address().clone(),
                    trial.get_size(),
                    trial.is_active(),
                    trial.is_rem_formed() || trial.is_ind_create_formed(),
                )
            };
            active.get_trial_mut(i).set_entry(None, 0);
            if !is_act {
                continue;
            }
            let entry = match self.find_entry(&addr, size, false) {
                Some(e) => e,
                None => {
                    active.get_trial_mut(i).mark_no_use();
                    continue;
                }
            };
            let res = self.entry[entry].justified_contain(&addr, size);
            if rem_or_ind && !self.entry[entry].is_first_in_class() {
                active.get_trial_mut(i).mark_no_use();
                continue;
            }
            active.get_trial_mut(i).set_entry(Some(entry), res);
        }
        active.sort_trials(&self.entry);
        for rule in self.model_rules.iter() {
            if rule.fillin_output_map(active, self) {
                for i in 0..active.get_num_trials() {
                    if active.get_trial(i).is_active() {
                        active.get_trial_mut(i).mark_used();
                    } else {
                        let t = active.get_trial_mut(i);
                        t.mark_no_use();
                        t.set_entry(None, 0);
                    }
                }
                return;
            }
        }
        self.fillin_map_fallback(active, true);
    }

    /// Find the return value storage using the older fallback method (C++
    /// `ParamListStandardOut::fillinMapFallback`).
    fn fillin_map_fallback(&self, active: &mut ParamActive, first_only: bool) {
        let mut bestentry: Option<usize> = None;
        let mut bestcover = 0;
        let mut bestclass = type_class::TYPECLASS_PTR;

        for (ci, curentry) in self.entry.iter().enumerate() {
            if first_only
                && !curentry.is_first_in_class()
                && curentry.is_exclusion()
                && curentry.get_all_groups().len() == 1
            {
                continue; // Not the first entry in the storage class
            }
            let mut putativematch = false;
            for j in 0..active.get_num_trials() {
                let (is_act, addr, size) = {
                    let pt = active.get_trial(j);
                    (pt.is_active(), pt.get_address().clone(), pt.get_size())
                };
                if is_act {
                    let res = curentry.justified_contain(&addr, size);
                    if res >= 0 {
                        active.get_trial_mut(j).set_entry(Some(ci), res);
                        putativematch = true;
                    } else {
                        active.get_trial_mut(j).set_entry(None, 0);
                    }
                } else {
                    active.get_trial_mut(j).set_entry(None, 0);
                }
            }
            if !putativematch {
                continue;
            }
            active.sort_trials(&self.entry);
            // Number of least-justified contiguous bytes for this entry.
            let mut offmatch = 0;
            let mut k = 0;
            while k < active.get_num_trials() {
                let pt = active.get_trial(k);
                if pt.get_entry().is_none() {
                    k += 1;
                    continue;
                }
                if offmatch != pt.get_offset() {
                    break;
                }
                if (offmatch == 0 && curentry.is_param_check_low())
                    || (offmatch != 0 && curentry.is_param_check_high())
                {
                    if pt.is_rem_formed() {
                        break;
                    }
                    if pt.is_ind_create_formed() {
                        break;
                    }
                }
                offmatch += pt.get_size();
                k += 1;
            }
            if offmatch < curentry.get_min_size() {
                k = 0; // Don't use this entry
            }
            if k == active.get_num_trials()
                && (curentry.get_type() < bestclass || offmatch > bestcover)
            {
                bestentry = Some(ci);
                bestcover = offmatch;
                bestclass = curentry.get_type();
            }
        }
        match bestentry {
            None => {
                for i in 0..active.get_num_trials() {
                    active.get_trial_mut(i).mark_no_use();
                }
            }
            Some(be) => {
                for i in 0..active.get_num_trials() {
                    let (is_act, addr, size) = {
                        let pt = active.get_trial(i);
                        (pt.is_active(), pt.get_address().clone(), pt.get_size())
                    };
                    if is_act {
                        let res = self.entry[be].justified_contain(&addr, size);
                        if res >= 0 {
                            let pt = active.get_trial_mut(i);
                            pt.mark_used();
                            pt.set_entry(Some(be), res);
                        } else {
                            let pt = active.get_trial_mut(i);
                            pt.mark_no_use();
                            pt.set_entry(None, 0);
                        }
                    } else {
                        let pt = active.get_trial_mut(i);
                        pt.mark_no_use();
                        pt.set_entry(None, 0);
                    }
                }
                active.sort_trials(&self.entry);
            }
        }
    }

    /// Add another model to this union (C++ `ParamListMerged::foldIn`).
    pub fn fold_in(&mut self, op2: &ParamListStandard) -> KunaResult<()> {
        if self.entry.is_empty() {
            self.spacebase = op2.spacebase.clone();
            self.entry = op2.entry.clone();
            return Ok(());
        }
        if !rc_opt_ptr_eq(&self.spacebase, &op2.spacebase) && op2.spacebase.is_some() {
            return Err(KunaError::lowlevel(
                "Cannot merge prototype models with different stacks",
            ));
        }
        for opentry in op2.entry.iter() {
            let mut typeint = 0;
            let mut found: Option<usize> = None;
            for (i, e) in self.entry.iter().enumerate() {
                if e.subsumes_definition(opentry) {
                    typeint = 2;
                    found = Some(i);
                    break;
                }
                if opentry.subsumes_definition(e) {
                    typeint = 1;
                    found = Some(i);
                    break;
                }
            }
            if typeint == 2 {
                let i = found.unwrap();
                if self.entry[i].get_min_size() != opentry.get_min_size() {
                    typeint = 0;
                }
            } else if typeint == 1 {
                let i = found.unwrap();
                if self.entry[i].get_min_size() != opentry.get_min_size() {
                    typeint = 0;
                } else {
                    self.entry[i] = opentry.clone(); // Replace with the containing entry
                }
            }
            if typeint == 0 {
                self.entry.push(opentry.clone());
            }
        }
        Ok(())
    }

    /// Fold-ins are finished; finalize this (C++ `ParamListMerged::finalize`).
    pub fn finalize(&mut self) {
        self.populate_resolver();
    }

    /// Cache ModelRule information after decode (C++
    /// `ParamListStandardOut::initialize`, fspec.cc:1614-1628).
    ///
    /// Scans the decoded `model_rules` for any rule whose `AssignAction` can
    /// drive `fillinOutputMap` (a `<join>`/`<consume>`/... output rule).  If one
    /// exists, the model uses the ModelRule output-map path (the
    /// `<join storage="general">` that joins a SPARC o0:o1 return pair) and
    /// `use_fillin_fallback` becomes false.  With no such rule the legacy
    /// fallback stays on and `auto_killed_by_call` is set (legacy behavior).
    pub fn initialize(&mut self) {
        self.use_fillin_fallback = true;
        for rule in self.model_rules.iter() {
            if rule.can_affect_fillin_output() {
                self.use_fillin_fallback = false;
                break;
            }
        }
        if self.use_fillin_fallback {
            self.auto_killed_by_call = true; // Legacy behavior if there are no rules
        }
    }

    /// Restore the model from an `<input>`/`<output>` element (C++ `decode`).
    ///
    /// STUB(W4): reaches the fspec-owned marshaling ElementIds/AttributeIds and
    /// the `<modelrule>` decode (STUB(w6-modelrules)).  Not yet ported; tests
    /// construct models directly via [`ParamListStandard::push_entry`].
    pub fn decode(&mut self, _normalstack: bool) -> KunaResult<()> {
        Err(KunaError::lowlevel(
            "STUB(W4) ParamListStandard::decode: fspec marshaling element ids not yet ported",
        ))
    }

    // -- test/tooling builder hooks -----------------------------------------

    /// Append a fully-formed [`ParamEntry`], replicating the non-marshaling
    /// tail of C++ `parsePentry` (with `splitFloat == true`, the default):
    /// update the resource-section boundaries, `spacebase`, and `numgroup`.
    /// Builder hook for tests and the model builders until the W4 decode path
    /// lands.
    ///
    /// The `groupid` is the new entry's primary group; the C++ derives it from
    /// the parse position, which equals the entry's own group here.
    pub fn push_entry(&mut self, e: ParamEntry) {
        // C++ parsePentry derives lastClass here (CLASS4 when the entry list is empty).
        let last_class: type_class = match self.entry.last() {
            None => type_class::TYPECLASS_CLASS4,
            Some(back) => {
                if back.is_grouped() {
                    type_class::TYPECLASS_GENERAL
                } else {
                    back.get_type()
                }
            }
        };
        let groupid = e.get_group();
        let current_class = if e.is_grouped() {
            type_class::TYPECLASS_GENERAL
        } else {
            e.get_type()
        };
        // splitFloat is true by default: open a new resource section whenever
        // the storage class changes (entries must be ordered by storage class).
        if last_class != current_class {
            // C++ throws if lastClass < currentClass; the seed/order checks
            // guard that, so we only push the boundary on a class change.
            if last_class >= current_class {
                self.resource_start.push(groupid);
            }
        }
        let spc = Rc::clone(e.get_space());
        if spc.get_type() == spacetype::IPTR_SPACEBASE {
            self.spacebase = Some(spc);
        }
        let maxgroup = e.get_all_groups().last().unwrap() + 1;
        if maxgroup > self.numgroup {
            self.numgroup = maxgroup;
        }
        self.entry.push(e);
    }

    /// Record the end-of-decode bookkeeping that `decode` performs after the
    /// entries are present: push the final resource-section boundary, compute
    /// the heritage delay, and build the resolver maps.  Test/builder hook.
    pub fn finish_decode(&mut self) {
        self.resource_start.push(self.numgroup);
        self.calc_delay();
        self.populate_resolver();
        // C++ `ParamListStandardOut::decode` calls `initialize()` after the base
        // `ParamListStandard::decode` (fspec.cc:1604), which scans the decoded
        // `<rule>`s for an output `fillinOutputMap` action and clears
        // `useFillinFallback`.  Harmless on the input lists (their
        // `use_fillin_fallback` flag is read only by the `StandardOut`/
        // `RegisterOut` fillin path).
        self.initialize();
    }

    /// Push a resource-section boundary (the C++ `resourceStart.push_back`),
    /// for multi-section models built by tests.
    pub fn push_resource_start(&mut self, group: int4) {
        self.resource_start.push(group);
    }

    /// Append a decoded [`ModelRule`] (C++ `modelRules.emplace_back(...)` inside
    /// `ParamListStandard::decode`, fspec.cc:1496-1497).  The architecture cspec
    /// loader calls this for each `<rule>` element after the `<pentry>`/`<group>`
    /// entries are present (the C++ ordering: rules come after entries).
    pub fn push_model_rule(&mut self, rule: ModelRule) {
        self.model_rules.push(rule);
    }

    /// Append the synthetic `pointermax` ConvertToPointer rule (C++
    /// `ParamListStandard::decode`, fspec.cc:1507-1512): a `SizeRestrictedFilter`
    /// (`pointermax+1`, 0) feeding a `ConvertToPointer` action, planted at the end
    /// of `modelRules` so any data-type larger than `pointermax` is passed as a
    /// pointer.  Called by the cspec loader when the model's `pointermax > 0`.
    pub fn push_pointermax_rule(&mut self, pointermax: int4) {
        let filter = crate::modelrules::DatatypeFilter::SizeRestricted(
            crate::modelrules::SizeRestriction::new(pointermax + 1, 0),
        );
        let action = crate::modelrules::AssignAction::ConvertToPointer {
            space: self.spacebase.clone(),
        };
        self.model_rules
            .push(ModelRule::from_components(filter, action));
    }

    /// Number of decoded model rules (test/inspection hook).
    pub fn num_model_rules(&self) -> usize {
        self.model_rules.len()
    }
}

/// Build the standard "cannot assign parameter address" error for a data-type
/// (C++ `ParamUnassignedError`).
fn unassigned_err(dt: &Rc<Datatype>) -> KunaError {
    KunaError::param_unassigned(format!(
        "Cannot assign parameter address for {}",
        dt.get_name()
    ))
}

// =============================================================================
// PrototypePieces (fspec.hh:373-381)
// =============================================================================

/// Raw components of a function prototype obtained from parsing source code
/// (C++ `PrototypePieces`, `fspec.hh:373-381`).
///
/// The `model` back-pointer (C++ `ProtoModel *`) is omitted: the kuna
/// `Architecture` (W4) has no prototype-model registry, so a `PrototypePieces`
/// cannot point at a registered model.  The methods that read it in the C++
/// (`ProtoStoreInternal::decode`, `paramShift`) take the [`ProtoModel`] as an
/// explicit argument instead.  // STUB(w6-fspec-2)
#[derive(Debug, Clone, Default)]
pub struct PrototypePieces {
    /// Identifier (function name) associated with prototype (C++ `name`).
    pub name: String,
    /// Return data-type (C++ `outtype`); `None` is the C++ null.
    pub outtype: Option<Rc<Datatype>>,
    /// Input data-types (C++ `intypes`).
    pub intypes: Vec<Rc<Datatype>>,
    /// Identifiers for input types (C++ `innames`).
    pub innames: Vec<String>,
    /// First position of a variable argument, or -1 if not varargs (C++
    /// `firstVarArgSlot`).
    pub first_var_arg_slot: int4,
    /// (kuna) Explicit, model-overriding locked output storage, as established by
    /// the console `map return <addr> <type>` (`IfcMapReturn`).  C++ keeps a
    /// callee's locked `FuncProto` live on its `Funcdata` and `ActionDefaultParams`
    /// does `fc->copy(otherfunc->getFuncProto())`, carrying the custom output
    /// storage verbatim; the merged tree reconstructs callee prototypes from
    /// `PrototypePieces`, which only describe *types* (storage is re-derived from
    /// the model).  A custom stack-relative return (e.g. `s0x10`) cannot be
    /// re-derived, so it rides here and `set_pieces` re-applies it after the
    /// model-driven `update_all_types`.  `None` is the normal (model-derived)
    /// case.
    pub output_storage: Option<ParameterPieces>,
    /// (kuna) Explicit, model-overriding locked INPUT storage for individual
    /// slots, as established by the console `map param <func>::<i> <storage>
    /// <decl>` (the `--assert 'param <func>::<i> ...'` directive).  The
    /// input-side twin of [`Self::output_storage`] and it exists for the same
    /// reason: these pieces describe *types*, and storage is re-derived from the
    /// model, so a caller that states "this callee takes its first argument in
    /// ECX" has nowhere else to put that fact.  `set_pieces` re-applies each
    /// `(slot, storage)` after the model-driven `update_all_types`.  Empty is the
    /// normal (model-derived) case.
    pub input_storage: Vec<(int4, ParameterPieces)>,
}

// =============================================================================
// FspecSpace (fspec.hh:341-351, fspec.cc:2109-2178)  // STUB(W4)
// =============================================================================

/// Reserved name for the fspec space (C++ `FspecSpace::NAME`).
///
/// The full `FspecSpace` (`AddrSpace` subclass that encodes a `FuncCallSpecs`
/// pointer as an address) reaches `FuncCallSpecs` (a `fspec-3` type) and the
/// marshaling encoder, so only its reserved name is carried here.
/// // STUB(w6-fspec-2)
pub const FSPEC_SPACE_NAME: &str = "fspec";

// =============================================================================
// ProtoModel (fspec.hh:748-1017, fspec.cc:2268-2705)
// =============================================================================

/// Reserved `extrapop` value meaning the function's `extrapop` is unknown (C++
/// `ProtoModel::extrapop_unknown`, `fspec.hh:772`).
pub const EXTRAPOP_UNKNOWN: int4 = 0x8000;

/// A complete model for passing data-types between functions (C++ `ProtoModel`,
/// `fspec.hh:748-1017`).
///
/// A model holds the resource lists for input parameters and return values
/// (the [`ParamListStandard`] family, tagged by [`ParamListKind`]), the
/// `extrapop`, the side-effect / likely-trash / internal-storage lists, the
/// stack ranges for locals and parameters, and several boolean properties.
///
/// The C++ `Architecture *glb` back-pointer is **not** held (the kuna
/// `Architecture` has no prototype-model registry yet); the methods that need
/// the stack space or float-extension construction take an [`AddrSpaceManager`]
/// and the type factory explicitly.  The merged-model variant
/// ([`ProtoModelMerged`]) is folded in as the `merged` field, mirroring the C++
/// subclass.  // STUB(w6-fspec-2)
#[derive(Debug, Clone)]
pub struct ProtoModel {
    /// Name of the model (C++ `name`).
    name: String,
    /// Extra bytes popped from stack (C++ `extrapop`).
    extrapop: int4,
    /// Resource model for input parameters (C++ `input`); `None` until
    /// `build_param_list`.
    input: Option<ParamListStandard>,
    /// Resource model for output parameters (C++ `output`); `None` until
    /// `build_param_list`.
    output: Option<ParamListStandard>,
    /// `true` if `self` is a copy of another model (C++ `compatModel != 0`),
    /// recording the identity of the alias parent.  The C++ holds a
    /// `const ProtoModel *`; the kuna port carries the alias parent's identity
    /// token (see [`ProtoModelId`]) so [`ProtoModel::is_compatible`] can match.
    compat_model: Option<ProtoModelId>,
    /// Stable identity of `self`, used to match `compat_model` across copies
    /// (the C++ uses raw `this`/`compatModel` pointer identity).
    id: ProtoModelId,
    /// List of side-effects, sorted by address (C++ `effectlist`).
    effectlist: Vec<EffectRecord>,
    /// Storage locations potentially carrying trash values, sorted (C++
    /// `likelytrash`).
    likelytrash: Vec<VarnodeData>,
    /// Registers that hold internal compiler constants, sorted (C++
    /// `internalstorage`).
    internalstorage: Vec<VarnodeData>,
    /// Id of injection to perform at beginning of function, -1 if unused (C++
    /// `injectUponEntry`).
    inject_upon_entry: int4,
    /// Id of injection to perform after a call to this function, -1 if unused
    /// (C++ `injectUponReturn`).
    inject_upon_return: int4,
    /// Memory range(s) of space-based locals (C++ `localrange`).
    localrange: RangeList,
    /// Memory range(s) of space-based parameters (C++ `paramrange`).
    paramrange: RangeList,
    /// `true` if stack parameters have low->high address ordering (C++
    /// `stackgrowsnegative`).
    stackgrowsnegative: bool,
    /// `true` if this model has a `this` parameter (C++ `hasThis`).
    has_this: bool,
    /// `true` if this model is a constructor (C++ `isConstruct`).
    is_construct: bool,
    /// `true` if this model should be printed in declarations (C++ `isPrinted`).
    is_printed: bool,
    /// The merged-model state, present only for a [`ProtoModelMerged`] (C++
    /// subclass `ProtoModelMerged`).
    merged: Option<ProtoModelMerged>,
    /// `true` if this is a placeholder for an unrecognized model name (C++
    /// `UnknownProtoModel::isUnknown`).
    is_unknown: bool,
}

/// Stable identity token for a [`ProtoModel`] (replaces the C++ raw `this` /
/// `compatModel` pointer identity used by [`ProtoModel::is_compatible`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProtoModelId(u64);

impl ProtoModelId {
    /// Allocate a fresh, process-unique identity token.
    fn fresh() -> ProtoModelId {
        use std::sync::atomic::{AtomicU64, Ordering};
        static NEXT: AtomicU64 = AtomicU64::new(1);
        ProtoModelId(NEXT.fetch_add(1, Ordering::Relaxed))
    }
}

/// The merged-model state of a [`ProtoModel`] (C++ `ProtoModelMerged`,
/// `fspec.hh:1077-1090`).
///
/// In the C++ this is a subclass of `ProtoModel`; here it is an optional
/// payload (`ProtoModel::merged`).  It holds the constituent models being
/// merged.  Each constituent is owned by reference-counted clone (the C++ holds
/// borrowed `ProtoModel *` into the architecture's registry; with no registry
/// the merged model owns clones of the folded-in constituents).
#[derive(Debug, Clone)]
pub struct ProtoModelMerged {
    /// Constituent models being merged (C++ `modellist`).
    modellist: Vec<Rc<ProtoModel>>,
}

impl ProtoModel {
    /// Construct an empty model ready for `decode`/seeding (C++
    /// `ProtoModel(Architecture *g)`).
    ///
    /// The C++ seeds `localrange`/`paramrange` from the stack space via
    /// `defaultLocalRange`/`defaultParamRange`; with no `glb` the stack space is
    /// supplied by the caller (the [`AddrSpaceManager`]).  If there is no stack
    /// space the default ranges are left empty (the C++ would dereference a null
    /// stack space; the model is built with a stack space in practice).
    pub fn new(manager: &AddrSpaceManager) -> ProtoModel {
        let mut res = ProtoModel {
            name: String::new(),
            extrapop: 0,
            input: None,
            output: None,
            compat_model: None,
            id: ProtoModelId::fresh(),
            effectlist: Vec::new(),
            likelytrash: Vec::new(),
            internalstorage: Vec::new(),
            inject_upon_entry: -1,
            inject_upon_return: -1,
            localrange: RangeList::new(),
            paramrange: RangeList::new(),
            stackgrowsnegative: true, // Normal stack parameter ordering
            has_this: false,
            is_construct: false,
            is_printed: true,
            merged: None,
            is_unknown: false,
        };
        res.default_local_range(manager);
        res.default_param_range(manager);
        res
    }

    /// Copy `op2` under a new name (C++
    /// `ProtoModel(const string &nm,const ProtoModel &op2)`).
    ///
    /// Everything is copied except the name; `is_printed` is reset to `true`
    /// (not inherited), `compat_model` records `op2`'s identity, and the
    /// `__thiscall` name forces `has_this`.
    pub fn copy_named(nm: &str, op2: &ProtoModel) -> ProtoModel {
        let mut res = op2.clone();
        res.id = ProtoModelId::fresh();
        res.name = nm.to_string();
        res.is_printed = true; // Don't inherit. Always print unless setPrintInDecl called explicitly
        // input/output/effectlist/likelytrash/internalstorage/ranges already
        // copied by `clone`.
        if res.name == "__thiscall" {
            res.has_this = true;
        }
        res.compat_model = Some(op2.id);
        res.merged = None; // the named copy is not itself a merged model
        res.is_unknown = false;
        res
    }

    /// Build an [`UnknownProtoModel`]-equivalent: a named copy of `placeholder`
    /// that identifies itself as unknown (C++ `UnknownProtoModel`,
    /// `fspec.hh:1025-1032`).
    pub fn new_unknown(nm: &str, placeholder: &ProtoModel) -> ProtoModel {
        let mut res = ProtoModel::copy_named(nm, placeholder);
        res.is_unknown = true;
        res
    }

    // -- Simple accessors (fspec.hh:777-1013) -------------------------------

    /// Get the name of the prototype model (C++ `getName`).
    pub fn get_name(&self) -> &str {
        &self.name
    }
    /// Get the stack-pointer `extrapop` (C++ `getExtraPop`).
    pub fn get_extra_pop(&self) -> int4 {
        self.extrapop
    }
    /// Set the stack-pointer `extrapop` (C++ `setExtraPop`).
    pub fn set_extra_pop(&mut self, ep: int4) {
        self.extrapop = ep;
    }
    /// Get the inject `uponentry` id (C++ `getInjectUponEntry`).
    pub fn get_inject_upon_entry(&self) -> int4 {
        self.inject_upon_entry
    }
    /// Get the inject `uponreturn` id (C++ `getInjectUponReturn`).
    pub fn get_inject_upon_return(&self) -> int4 {
        self.inject_upon_return
    }
    /// Get the range of (possible) local stack variables (C++ `getLocalRange`).
    pub fn get_local_range(&self) -> &RangeList {
        &self.localrange
    }
    /// Get the range of (possible) stack parameters (C++ `getParamRange`).
    pub fn get_param_range(&self) -> &RangeList {
        &self.paramrange
    }
    /// Get the side-effect list (C++ `effectBegin`/`effectEnd`).
    pub fn effect_list(&self) -> &[EffectRecord] {
        &self.effectlist
    }
    /// Get the likely-trash list (C++ `trashBegin`/`trashEnd`).
    pub fn trash_list(&self) -> &[VarnodeData] {
        &self.likelytrash
    }
    /// Get the internal-storage list (C++ `internalBegin`/`internalEnd`).
    pub fn internal_list(&self) -> &[VarnodeData] {
        &self.internalstorage
    }
    /// Get the stack space associated with this model (C++ `getSpacebase`).
    pub fn get_spacebase(&self) -> Option<&Rc<AddrSpace>> {
        self.input.as_ref().and_then(|i| i.get_spacebase())
    }
    /// Return `true` if the stack grows toward smaller addresses (C++
    /// `isStackGrowsNegative`).
    pub fn is_stack_grows_negative(&self) -> bool {
        self.stackgrowsnegative
    }
    /// Is this a model for (non-static) class methods (C++ `hasThisPointer`).
    pub fn has_this_pointer(&self) -> bool {
        self.has_this
    }
    /// Is this model for class constructors (C++ `isConstructor`).
    pub fn is_constructor(&self) -> bool {
        self.is_construct
    }
    /// Return `true` if name should be printed in declarations (C++
    /// `printInDecl`).
    pub fn print_in_decl(&self) -> bool {
        self.is_printed
    }
    /// Set whether this name should be printed in declarations (C++
    /// `setPrintInDecl`).
    pub fn set_print_in_decl(&mut self, val: bool) {
        self.is_printed = val;
    }
    /// Maximum heritage delay across all input parameters (C++
    /// `getMaxInputDelay`).
    pub fn get_max_input_delay(&self) -> int4 {
        self.input.as_ref().map(|i| i.get_max_delay()).unwrap_or(0)
    }
    /// Maximum heritage delay across all return values (C++
    /// `getMaxOutputDelay`).
    pub fn get_max_output_delay(&self) -> int4 {
        self.output.as_ref().map(|o| o.get_max_delay()).unwrap_or(0)
    }
    /// Does this model automatically consider potential output locations as
    /// killed-by-call (C++ `isAutoKilledByCall`).
    pub fn is_auto_killed_by_call(&self) -> bool {
        self.output.as_ref().map(|o| o.is_auto_killed_by_call()).unwrap_or(false)
    }
    /// Is this a merged prototype model (C++ `isMerged`).
    pub fn is_merged(&self) -> bool {
        self.merged.is_some()
    }
    /// Is this a placeholder for an unrecognized model name (C++ `isUnknown`).
    pub fn is_unknown(&self) -> bool {
        self.is_unknown
    }
    /// Return the identity of the model `self` is an alias of, or `None` (C++
    /// `getAliasParent`).
    pub fn get_alias_parent(&self) -> Option<ProtoModelId> {
        self.compat_model
    }
    /// Borrow the input resource model (panics if not yet built — C++ would
    /// dereference a null `input`).
    pub fn input(&self) -> &ParamListStandard {
        self.input.as_ref().expect("ProtoModel::input: not built")
    }
    /// Borrow the output resource model (panics if not yet built — C++ would
    /// dereference a null `output`).
    pub fn output(&self) -> &ParamListStandard {
        self.output.as_ref().expect("ProtoModel::output: not built")
    }
    /// Borrow the merged-model state, if this is a merged model.
    pub fn merged(&self) -> Option<&ProtoModelMerged> {
        self.merged.as_ref()
    }

    // -- Query delegators (fspec.hh:812-975) --------------------------------

    /// Check if the two input storage locations can represent a single logical
    /// parameter (C++ `checkInputJoin`).
    pub fn check_input_join(
        &self,
        hiaddr: &Address,
        hisize: int4,
        loaddr: &Address,
        losize: int4,
    ) -> bool {
        self.input().check_join(hiaddr, hisize, loaddr, losize)
    }
    /// Check if the two output storage locations can represent a single logical
    /// return value (C++ `checkOutputJoin`).
    pub fn check_output_join(
        &self,
        hiaddr: &Address,
        hisize: int4,
        loaddr: &Address,
        losize: int4,
    ) -> bool {
        self.output().check_join(hiaddr, hisize, loaddr, losize)
    }
    /// Check if a single storage location can be split into two input
    /// parameters (C++ `checkInputSplit`).
    pub fn check_input_split(&self, loc: &Address, size: int4, splitpoint: int4) -> bool {
        self.input().check_split(loc, size, splitpoint)
    }
    /// Characterize whether the given range overlaps input storage (C++
    /// `characterizeAsInputParam`).
    pub fn characterize_as_input_param(&self, loc: &Address, size: int4) -> Containment {
        self.input().characterize_as_param(loc, size)
    }
    /// Characterize whether the given range overlaps output storage (C++
    /// `characterizeAsOutput`).
    pub fn characterize_as_output(&self, loc: &Address, size: int4) -> Containment {
        self.output().characterize_as_param(loc, size)
    }
    /// Does the given storage location make sense as an input parameter (C++
    /// `possibleInputParam`).
    pub fn possible_input_param(&self, loc: &Address, size: int4) -> bool {
        self.input().possible_param(loc, size)
    }
    /// Does the given storage location make sense as a return value (C++
    /// `possibleOutputParam`).
    pub fn possible_output_param(&self, loc: &Address, size: int4) -> bool {
        self.output().possible_param(loc, size)
    }
    /// Pass back the slot/slot-size for the storage as an input parameter (C++
    /// `possibleInputParamWithSlot`).
    pub fn possible_input_param_with_slot(
        &self,
        loc: &Address,
        size: int4,
        slot: &mut int4,
        slotsize: &mut int4,
    ) -> bool {
        self.input().possible_param_with_slot(loc, size, slot, slotsize)
    }
    /// Pass back the slot/slot-size for the storage as a return value (C++
    /// `possibleOutputParamWithSlot`).
    pub fn possible_output_param_with_slot(
        &self,
        loc: &Address,
        size: int4,
        slot: &mut int4,
        slotsize: &mut int4,
    ) -> bool {
        self.output().possible_param_with_slot(loc, size, slot, slotsize)
    }
    /// Check if the storage looks like an unjustified input parameter (C++
    /// `unjustifiedInputParam`).
    pub fn unjustified_input_param(&self, loc: &Address, size: int4, res: &mut VarnodeData) -> bool {
        self.input().unjustified_container(loc, size, res)
    }
    /// Get the type of extension and containing input parameter (C++
    /// `assumedInputExtension`).
    pub fn assumed_input_extension(&self, addr: &Address, size: int4, res: &mut VarnodeData) -> OpCode {
        self.input().assumed_extension(addr, size, res)
    }
    /// Get the type of extension and containing return value location (C++
    /// `assumedOutputExtension`).
    pub fn assumed_output_extension(&self, addr: &Address, size: int4, res: &mut VarnodeData) -> OpCode {
        self.output().assumed_extension(addr, size, res)
    }
    /// Pass back the biggest input parameter contained in the range (C++
    /// `getBiggestContainedInputParam`).
    pub fn get_biggest_contained_input_param(&self, loc: &Address, size: int4, res: &mut VarnodeData) -> bool {
        self.input().get_biggest_contained_param(loc, size, res)
    }
    /// Pass back the biggest output parameter contained in the range (C++
    /// `getBiggestContainedOutput`).
    pub fn get_biggest_contained_output(&self, loc: &Address, size: int4, res: &mut VarnodeData) -> bool {
        self.output().get_biggest_contained_param(loc, size, res)
    }
    /// Derive the most likely input prototype from a list of trials (C++
    /// `deriveInputMap`).
    pub fn derive_input_map(&self, active: &mut ParamActive, manager: &AddrSpaceManager) -> KunaResult<()> {
        self.input().fillin_map(active, manager)
    }
    /// Derive the most likely output prototype from a list of trials (C++
    /// `deriveOutputMap`).
    pub fn derive_output_map(&self, active: &mut ParamActive, manager: &AddrSpaceManager) -> KunaResult<()> {
        self.output().fillin_map(active, manager)
    }

    // -- Default stack ranges (fspec.cc:2268-2324) --------------------------

    /// Set the default stack range used for local variables (C++
    /// `defaultLocalRange`).
    fn default_local_range(&mut self, manager: &AddrSpaceManager) {
        let spc = match manager.get_stack_space() {
            Some(s) => Rc::clone(s),
            None => return,
        };
        let (first, last): (uintb, uintb);
        if self.stackgrowsnegative {
            // Normal stack convention: locals are negative offsets off the stack
            last = spc.get_highest();
            if spc.get_addr_size() >= 4 {
                first = last - 999999;
            } else if spc.get_addr_size() >= 2 {
                first = last - 9999;
            } else {
                first = last - 99;
            }
            self.localrange.insert_range(spc, first, last);
        } else {
            // Flipped stack convention
            first = 0;
            if spc.get_addr_size() >= 4 {
                last = 999999;
            } else if spc.get_addr_size() >= 2 {
                last = 9999;
            } else {
                last = 99;
            }
            self.localrange.insert_range(spc, first, last);
        }
    }

    /// Set the default stack range used for input parameters (C++
    /// `defaultParamRange`).
    fn default_param_range(&mut self, manager: &AddrSpaceManager) {
        let spc = match manager.get_stack_space() {
            Some(s) => Rc::clone(s),
            None => return,
        };
        let (first, last): (uintb, uintb);
        if self.stackgrowsnegative {
            // Normal stack convention: parameters are positive offsets off the stack
            first = 0;
            if spc.get_addr_size() >= 4 {
                last = 511;
            } else if spc.get_addr_size() >= 2 {
                last = 255;
            } else {
                last = 15;
            }
            self.paramrange.insert_range(spc, first, last);
        } else {
            // Flipped stack convention
            last = spc.get_highest();
            if spc.get_addr_size() >= 4 {
                first = last - 511;
            } else if spc.get_addr_size() >= 2 {
                first = last - 255;
            } else {
                first = last - 15;
            }
            self.paramrange.insert_range(spc, first, last); // Parameters are negative offsets
        }
    }

    /// Establish the main resource lists for input and output parameters (C++
    /// `buildParamList`).  `strategy` is currently `""`/`"standard"` or
    /// `"register"`.
    pub fn build_param_list(&mut self, strategy: &str) -> KunaResult<()> {
        if strategy.is_empty() || strategy == "standard" {
            self.input = Some(ParamListStandard::new(ParamListKind::Standard));
            self.output = Some(ParamListStandard::new(ParamListKind::StandardOut));
        } else if strategy == "register" {
            self.input = Some(ParamListStandard::new(ParamListKind::Register));
            self.output = Some(ParamListStandard::new(ParamListKind::RegisterOut));
        } else {
            return Err(KunaError::lowlevel(format!(
                "Unknown strategy type: {strategy}"
            )));
        }
        Ok(())
    }

    /// Test whether one [`ProtoModel`] can be substituted for another during
    /// `FuncCallSpecs::deindirect` (C++ `isCompatible`).
    ///
    /// `op2` is compatible when it is the same model, or one is a named copy of
    /// the other (matched by [`ProtoModelId`]).
    pub fn is_compatible(&self, op2: &ProtoModel) -> bool {
        if self.id == op2.id {
            return true;
        }
        if self.compat_model == Some(op2.id) {
            return true;
        }
        if op2.compat_model == Some(self.id) {
            return true;
        }
        false
    }

    /// Calculate input and output storage locations given a function prototype
    /// (C++ `assignParameterStorage`).
    ///
    /// The passed-back storage locations are ordered with the output storage
    /// first, followed by the input storage locations.  If `ignore_output_error`
    /// is set, a failure to assign the output is swallowed and the return value
    /// is assumed `void`.
    pub fn assign_parameter_storage(
        &self,
        proto: &PrototypePieces,
        res: &mut Vec<ParameterPieces>,
        ignore_output_error: bool,
        typefactory: &dyn TypeFactory,
        manager: &AddrSpaceManager,
    ) -> KunaResult<()> {
        if ignore_output_error {
            match self.output().assign_map(proto, typefactory, res, manager) {
                Ok(()) => {}
                Err(KunaError::ParamUnassigned { .. }) => {
                    // ParamUnassignedError: leave address undefined, void return
                    res.clear();
                    let p = ParameterPieces {
                        flags: 0,
                        type_: Some(typefactory.get_type_void()?),
                        ..Default::default()
                    };
                    res.push(p);
                }
                Err(e) => return Err(e),
            }
        } else {
            self.output().assign_map(proto, typefactory, res, manager)?;
        }
        self.input().assign_map(proto, typefactory, res, manager)?;

        if self.has_this && res.len() > 1 {
            let mut this_index = 1usize;
            if (res[1].flags & parameter_pieces_flags::HIDDENRETPARM) != 0 && res.len() > 2 {
                if self.input().is_this_before_ret_pointer() {
                    // pointer has been bumped by auto-return-storage; swap markup
                    // for slots 1 and 2.
                    let (left, right) = res.split_at_mut(2);
                    left[1].swap_markup(&mut right[0]);
                } else {
                    this_index = 2;
                }
            }
            res[this_index].flags |= parameter_pieces_flags::ISTHIS;
        }
        Ok(())
    }

    /// Look up an effect from the given (address-sorted) [`EffectRecord`] list
    /// (C++ static `lookupEffect`).  Returns [`effect_type::UNKNOWN_EFFECT`] if
    /// there is no match.
    pub fn lookup_effect(efflist: &[EffectRecord], addr: &Address, size: int4) -> uint4 {
        // Unique is always local to function
        if let Some(spc) = addr.get_space() {
            if spc.get_type() == spacetype::IPTR_INTERNAL {
                return effect_type::UNAFFECTED;
            }
        }
        let cur = EffectRecord::new_unknown(addr, size);
        // upper_bound: first element strictly greater than cur by address.
        let idx = upper_bound_by(efflist, &cur, EffectRecord::compare_by_address);
        if idx == 0 {
            return effect_type::UNKNOWN_EFFECT; // Can't go back one
        }
        let hitrec = &efflist[idx - 1];
        let hit = hitrec.get_address();
        let sz = hitrec.get_size();
        if sz == 0 && rc_opt_eq_space(&Some(Rc::clone(hit.get_space().unwrap())), addr.get_space()) {
            // A size of zero indicates the whole space is unaffected
            return effect_type::UNAFFECTED;
        }
        let where_ = addr.overlap(0, &hit, sz);
        if where_ >= 0 && where_ + size <= sz {
            return hitrec.get_type();
        }
        effect_type::UNKNOWN_EFFECT
    }

    /// Look up a particular [`EffectRecord`] from a list by its address and
    /// size (C++ static `lookupRecord`).  Only the first `list_size` elements
    /// are examined.  Returns the matching index, or -1 (no overlap) / -2
    /// (partial overlap).
    pub fn lookup_record(
        efflist: &[EffectRecord],
        list_size: int4,
        addr: &Address,
        size: int4,
    ) -> int4 {
        if list_size == 0 {
            return -1;
        }
        let cur = EffectRecord::new_unknown(addr, size);
        let window = &efflist[..list_size as usize];
        let idx = upper_bound_by(window, &cur, EffectRecord::compare_by_address);
        if idx == 0 {
            // C++ dereferences *iter (== efflist.begin()) here.
            let close_addr = efflist[0].get_address();
            return if close_addr.overlap(0, addr, size) < 0 { -1 } else { -2 };
        }
        let closerec = &efflist[idx - 1];
        let close_addr = closerec.get_address();
        let sz = closerec.get_size();
        if &close_addr == addr && size == sz {
            return (idx - 1) as i32; // iter - begiter
        }
        if addr.overlap(0, &close_addr, sz) < 0 {
            -1
        } else {
            -2
        }
    }

    /// Determine the side-effect of this model on the given memory range (C++
    /// `hasEffect`).
    pub fn has_effect(&self, addr: &Address, size: int4) -> uint4 {
        ProtoModel::lookup_effect(&self.effectlist, addr, size)
    }

    /// Restore this model from a `<prototype>` element (C++ `decode`).
    ///
    /// STUB(w6-fspec-2): reaches the marshaling [`kuna_base::marshal::Decoder`],
    /// the fspec ElementIds/AttributeIds, and `glb` (stack space,
    /// `defaultReturnAddr`, `pcodeinjectlib`) — the `Architecture` registry is
    /// not yet ported.  Tests build models via [`ProtoModel::build_param_list`]
    /// + the `ParamListStandard` builder hooks.
    pub fn decode(&mut self) -> KunaResult<()> {
        Err(KunaError::lowlevel(
            "STUB(w6-fspec-2) ProtoModel::decode: marshaling + Architecture registry not yet ported",
        ))
    }

    // -- merged-model operations (fspec.cc:2785-2927) -----------------------

    /// Construct an empty merged model (C++ `ProtoModelMerged(Architecture *g)`).
    pub fn new_merged(manager: &AddrSpaceManager) -> ProtoModel {
        let mut res = ProtoModel::new(manager);
        res.merged = Some(ProtoModelMerged { modellist: Vec::new() });
        res
    }

    /// Number of constituent models (C++ `ProtoModelMerged::numModels`).
    pub fn num_models(&self) -> int4 {
        self.merged.as_ref().map(|m| m.modellist.len() as i32).unwrap_or(0)
    }

    /// Get the i-th constituent model (C++ `ProtoModelMerged::getModel`).
    pub fn get_model(&self, i: int4) -> &Rc<ProtoModel> {
        &self.merged.as_ref().expect("not a merged model").modellist[i as usize]
    }

    /// Fold EffectRecords into this model, keeping the intersection (C++
    /// `ProtoModelMerged::intersectEffects`).
    fn intersect_effects(&mut self, efflist: &[EffectRecord]) {
        let mut newlist: Vec<EffectRecord> = Vec::new();
        let mut i = 0usize;
        let mut j = 0usize;
        while i < self.effectlist.len() && j < efflist.len() {
            let eff1 = &self.effectlist[i];
            let eff2 = &efflist[j];
            use std::cmp::Ordering::*;
            match EffectRecord::compare_by_address(eff1, eff2) {
                Less => i += 1,
                Greater => j += 1,
                Equal => {
                    if eff1 == eff2 {
                        newlist.push(eff1.clone());
                    }
                    i += 1;
                    j += 1;
                }
            }
        }
        self.effectlist = newlist;
    }

    /// Intersect two sorted register lists, replacing the first (C++ static
    /// `ProtoModelMerged::intersectRegisters`).
    fn intersect_registers(reg_list1: &mut Vec<VarnodeData>, reg_list2: &[VarnodeData]) {
        let mut newlist: Vec<VarnodeData> = Vec::new();
        let mut i = 0usize;
        let mut j = 0usize;
        while i < reg_list1.len() && j < reg_list2.len() {
            use std::cmp::Ordering::*;
            match reg_list1[i].cmp(&reg_list2[j]) {
                Less => i += 1,
                Greater => j += 1,
                Equal => {
                    newlist.push(reg_list1[i].clone());
                    i += 1;
                    j += 1;
                }
            }
        }
        *reg_list1 = newlist;
    }

    /// Fold in an additional prototype model (C++ `ProtoModelMerged::foldIn`).
    ///
    /// The constituent `model`'s input must be a standard or register list; on
    /// the first fold-in the merged input/output/extrapop/effects are seeded,
    /// subsequent fold-ins take the intersection.
    pub fn fold_in_model(&mut self, model: &Rc<ProtoModel>) -> KunaResult<()> {
        if !matches!(
            model.input().get_type(),
            ParamListType::Standard | ParamListType::Register
        ) {
            return Err(KunaError::lowlevel(
                "Can only resolve between standard prototype models",
            ));
        }
        if self.input.is_none() {
            // First fold in
            let mut merged_input = ParamListStandard::new(ParamListKind::Merged);
            merged_input.fold_in(model.input())?;
            self.input = Some(merged_input);
            self.output = Some(model.output().clone());
            self.extrapop = model.extrapop;
            self.effectlist = model.effectlist.clone();
            self.inject_upon_entry = model.inject_upon_entry;
            self.inject_upon_return = model.inject_upon_return;
            self.likelytrash = model.likelytrash.clone();
            self.localrange = model.localrange.clone();
            self.paramrange = model.paramrange.clone();
        } else {
            self.input.as_mut().unwrap().fold_in(model.input())?;
            // We assume here that the output models are the same, but we don't check
            if self.extrapop != model.extrapop {
                self.extrapop = EXTRAPOP_UNKNOWN;
            }
            if self.inject_upon_entry != model.inject_upon_entry
                || self.inject_upon_return != model.inject_upon_return
            {
                return Err(KunaError::lowlevel(
                    "Cannot merge prototype models with different inject ids",
                ));
            }
            self.intersect_effects(&model.effectlist);
            ProtoModel::intersect_registers(&mut self.likelytrash, &model.likelytrash);
            ProtoModel::intersect_registers(&mut self.internalstorage, &model.internalstorage);
            // Take the union of the localrange and paramrange
            for r in model.localrange.iter() {
                self.localrange
                    .insert_range(Rc::clone(r.get_space()), r.get_first(), r.get_last());
            }
            for r in model.paramrange.iter() {
                self.paramrange
                    .insert_range(Rc::clone(r.get_space()), r.get_first(), r.get_last());
            }
        }
        Ok(())
    }

    /// Fold in a constituent and record it in the merged `modellist` (the C++
    /// `ProtoModelMerged::decode` body: `foldIn(mymodel)` then
    /// `modellist.push_back(mymodel)`).  Run `finalize` after the last fold-in.
    pub fn merged_push(&mut self, model: Rc<ProtoModel>) -> KunaResult<()> {
        self.fold_in_model(&model)?;
        self.merged
            .as_mut()
            .expect("merged_push on non-merged model")
            .modellist
            .push(model);
        Ok(())
    }

    /// Finalize the merged input list after all fold-ins (C++
    /// `((ParamListMerged *)input)->finalize()`).
    pub fn merged_finalize(&mut self) {
        if let Some(input) = self.input.as_mut() {
            input.finalize();
        }
    }

    /// Select the best constituent model given a set of trials (C++
    /// `ProtoModelMerged::selectModel`).  Uses [`ScoreProtoModel`] to score
    /// each constituent.
    pub fn select_model(&self, active: &ParamActive) -> KunaResult<Rc<ProtoModel>> {
        let merged = self
            .merged
            .as_ref()
            .ok_or_else(|| KunaError::lowlevel("selectModel on non-merged model"))?;
        let mut bestscore = 500;
        let mut bestindex: i32 = -1;
        for (i, m) in merged.modellist.iter().enumerate() {
            let numtrials = active.get_num_trials();
            let mut scoremodel = ScoreProtoModel::new(true, numtrials);
            for j in 0..numtrials {
                let trial = active.get_trial(j);
                if trial.is_active() {
                    scoremodel.add_parameter(m, trial.get_address(), trial.get_size());
                }
            }
            scoremodel.do_score();
            let score = scoremodel.get_score();
            if score < bestscore {
                bestscore = score;
                bestindex = i as i32;
                if bestscore == 0 {
                    break; // Can't get any lower
                }
            }
        }
        if bestindex >= 0 {
            return Ok(Rc::clone(&merged.modellist[bestindex as usize]));
        }
        Err(KunaError::lowlevel("No model matches : missing default"))
    }

    // -- test/tooling builder hooks -----------------------------------------

    /// Set the model name (test/builder hook; the C++ sets `name` during
    /// `decode`).
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
        if self.name == "__thiscall" {
            self.has_this = true;
        }
    }
    /// Set the `has_this` flag (test/builder hook; decoded from `hasthis`).
    pub fn set_has_this(&mut self, val: bool) {
        self.has_this = val;
    }
    /// Set the `is_construct` flag (test/builder hook; decoded from
    /// `constructor`).
    pub fn set_constructor(&mut self, val: bool) {
        self.is_construct = val;
    }
    /// Mutable access to the input resource model for builder hooks.
    pub fn input_mut(&mut self) -> &mut ParamListStandard {
        self.input.as_mut().expect("ProtoModel::input_mut: not built")
    }
    /// Mutable access to the output resource model for builder hooks.
    pub fn output_mut(&mut self) -> &mut ParamListStandard {
        self.output.as_mut().expect("ProtoModel::output_mut: not built")
    }
    /// Append a side-effect record and re-sort (test/builder hook; the C++
    /// `decode` sorts `effectlist` by `compareByAddress`).
    pub fn push_effect(&mut self, eff: EffectRecord) {
        self.effectlist.push(eff);
        self.effectlist.sort_by(EffectRecord::compare_by_address);
    }
    /// Append a likely-trash register and re-sort (test/builder hook).
    pub fn push_likely_trash(&mut self, vd: VarnodeData) {
        self.likelytrash.push(vd);
        self.likelytrash.sort_unstable();
    }
    /// Append an internal-storage register and re-sort (test/builder hook).
    pub fn push_internal_storage(&mut self, vd: VarnodeData) {
        self.internalstorage.push(vd);
        self.internalstorage.sort_unstable();
    }
    /// Set the inject ids (test/builder hook).
    pub fn set_inject_ids(&mut self, upon_entry: int4, upon_return: int4) {
        self.inject_upon_entry = upon_entry;
        self.inject_upon_return = upon_return;
    }
}

/// `std::upper_bound` over a slice with a strict-weak-order comparator
/// returning [`std::cmp::Ordering`]: the index of the first element that
/// compares `Greater` than `value` (i.e. for which `cmp(value, elem) ==
/// Less`).  Replicates the C++ `upper_bound(begin,end,cur,compareByAddress)`
/// where `compareByAddress` is the `<` predicate.
fn upper_bound_by<T, F>(slice: &[T], value: &T, mut cmp: F) -> usize
where
    F: FnMut(&T, &T) -> std::cmp::Ordering,
{
    let mut lo = 0usize;
    let mut hi = slice.len();
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        // upper_bound: advance while !(value < slice[mid]), i.e. while
        // cmp(value, slice[mid]) != Less.
        if cmp(value, &slice[mid]) != std::cmp::Ordering::Less {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }
    lo
}

// =============================================================================
// ScoreProtoModel (fspec.hh:1040-1064, fspec.cc:2710-2780)
// =============================================================================

/// A record mapping a trial to a parameter entry in the prototype model (C++
/// `ScoreProtoModel::PEntry`, `fspec.hh:1042-1052`).
#[derive(Debug, Clone)]
struct PEntry {
    /// Original index of trial (C++ `origIndex`).
    #[allow(dead_code)] // recorded as in C++; not read after sort, but kept for fidelity
    orig_index: int4,
    /// Matching slot within the resource list (C++ `slot`).
    slot: int4,
    /// Number of slots occupied (C++ `size`).
    size: int4,
}

/// Calculates "goodness of fit" of parameter trials against a [`ProtoModel`]
/// (C++ `ScoreProtoModel`, `fspec.hh:1040-1064`).  A lower score is a better
/// fit.
#[derive(Debug, Clone)]
pub struct ScoreProtoModel {
    /// `true` if scoring against input parameters, `false` for outputs (C++
    /// `isinputscore`).
    isinputscore: bool,
    /// Map of parameter entries corresponding to trials (C++ `entry`).
    entry: Vec<PEntry>,
    /// The final fitness score (C++ `finalscore`).
    finalscore: int4,
    /// Number of trials that don't fit the model at all (C++ `mismatch`).
    mismatch: int4,
}

impl ScoreProtoModel {
    /// Construct a scorer (C++
    /// `ScoreProtoModel(bool isinput,const ProtoModel *mod,int4 numparam)`).
    ///
    /// The C++ holds the `ProtoModel *` for the duration; the kuna scorer takes
    /// it per [`ScoreProtoModel::add_parameter`] call instead (avoids a borrow
    /// of the model spanning the scorer's lifetime).
    pub fn new(isinput: bool, numparam: int4) -> ScoreProtoModel {
        ScoreProtoModel {
            isinputscore: isinput,
            entry: Vec::with_capacity(numparam.max(0) as usize),
            finalscore: -1,
            mismatch: 0,
        }
    }

    /// Register a trial to be scored against `model` (C++ `addParameter`).
    pub fn add_parameter(&mut self, model: &ProtoModel, addr: &Address, sz: int4) {
        let orig = self.entry.len() as i32;
        let mut slot = 0;
        let mut slotsize = 0;
        let isparam = if self.isinputscore {
            model.possible_input_param_with_slot(addr, sz, &mut slot, &mut slotsize)
        } else {
            model.possible_output_param_with_slot(addr, sz, &mut slot, &mut slotsize)
        };
        if isparam {
            self.entry.push(PEntry { orig_index: orig, slot, size: slotsize });
        } else {
            self.mismatch += 1;
        }
    }

    /// Compute the fitness score (C++ `doScore`).
    pub fn do_score(&mut self) {
        // Sort our entries via slot (C++ `PEntry::operator<` compares slot only;
        // std::sort is not stable -> sort_unstable_by_key).
        self.entry.sort_unstable_by_key(|p| p.slot);

        let mut nextfree = 0; // Next slot we expect to see
        let mut basescore = 0;
        let penalty = [16, 10, 7, 5];
        let penaltyfinal = 3;
        let mismatchpenalty = 20;

        for p in self.entry.iter() {
            if p.slot > nextfree {
                // Some kind of hole in our slot coverage
                while nextfree < p.slot {
                    if nextfree < 4 {
                        basescore += penalty[nextfree as usize];
                    } else {
                        basescore += penaltyfinal;
                    }
                    nextfree += 1;
                }
                nextfree += p.size;
            } else if nextfree > p.slot {
                // Some kind of slot duplication
                basescore += mismatchpenalty;
                if p.slot + p.size > nextfree {
                    nextfree = p.slot + p.size;
                }
            } else {
                nextfree = p.slot + p.size;
            }
        }
        self.finalscore = basescore + mismatchpenalty * self.mismatch;
    }

    /// Get the fitness score (C++ `getScore`).
    pub fn get_score(&self) -> int4 {
        self.finalscore
    }
    /// Get the number of mismatched trials (C++ `getNumMismatch`).
    pub fn get_num_mismatch(&self) -> int4 {
        self.mismatch
    }
}

// =============================================================================
// ProtoParameter / ParameterBasic (fspec.hh:1100-1191, fspec.cc:2929-2984)
// =============================================================================

/// A function parameter viewed as a name, data-type, and storage address (C++
/// abstract base `ProtoParameter`, `fspec.hh:1100-1156`).
///
/// The C++ base is abstract with `ParameterBasic`/`ParameterSymbol` subclasses.
/// `ParameterSymbol` is backed by a `Scope`/`Symbol` (W3/W4) and is a boundary; the
/// kuna trait carries the shared query surface, and [`ParameterBasic`] is the
/// stand-alone (no backing symbol) implementation.
pub trait ProtoParameter {
    /// Get the name of the parameter ("" for return value) (C++ `getName`).
    fn get_name(&self) -> &str;
    /// Get the data-type associated with this (C++ `getType`); `None` is the
    /// C++ null.
    fn get_type(&self) -> Option<&Rc<Datatype>>;
    /// Get the storage address for this parameter (C++ `getAddress`).
    fn get_address(&self) -> Address;
    /// Get the number of bytes occupied (C++ `getSize`).
    fn get_size(&self) -> int4;
    /// Is the parameter data-type locked (C++ `isTypeLocked`).
    fn is_type_locked(&self) -> bool;
    /// Is the parameter name locked (C++ `isNameLocked`).
    fn is_name_locked(&self) -> bool;
    /// Is the size of the parameter locked (C++ `isSizeTypeLocked`).
    fn is_size_type_locked(&self) -> bool;
    /// Is this the "this" pointer for a class method (C++ `isThisPointer`).
    fn is_this_pointer(&self) -> bool;
    /// Is this really a pointer to the true parameter (C++ `isIndirectStorage`).
    fn is_indirect_storage(&self) -> bool;
    /// Is this a pointer to storage for a return value (C++ `isHiddenReturn`).
    fn is_hidden_return(&self) -> bool;
    /// Is the name undefined (C++ `isNameUndefined`).
    fn is_name_undefined(&self) -> bool;
    /// Toggle the lock on the data-type (C++ `setTypeLock`).
    fn set_type_lock(&mut self, val: bool);
    /// Toggle the lock on the name (C++ `setNameLock`).
    fn set_name_lock(&mut self, val: bool);
    /// Toggle whether this is the "this" pointer (C++ `setThisPointer`).
    fn set_this_pointer(&mut self, val: bool);
    /// Change (override) the data-type of a size-locked parameter (C++
    /// `overrideSizeLockType`).
    fn override_size_lock_type(&mut self, ct: Rc<Datatype>) -> KunaResult<()>;
    /// Clear the data-type preserving any size-lock (C++ `resetSizeLockType`).
    fn reset_size_lock_type(&mut self, factory: &dyn TypeFactory) -> KunaResult<()>;
}

/// Compare storage location and data-type for equality (C++
/// `ProtoParameter::operator==`).  Two parameters are equal when they share a
/// storage address and data-type (compared by `Rc` identity, mirroring the C++
/// `Datatype *` pointer compare).
pub fn proto_parameter_eq(a: &dyn ProtoParameter, b: &dyn ProtoParameter) -> bool {
    if a.get_address() != b.get_address() {
        return false;
    }
    match (a.get_type(), b.get_type()) {
        (Some(x), Some(y)) => Rc::ptr_eq(x, y),
        (None, None) => true,
        _ => false,
    }
}

/// A stand-alone parameter with no backing symbol (C++ `ParameterBasic`,
/// `fspec.hh:1163-1191`).
#[derive(Debug, Clone)]
pub struct ParameterBasic {
    /// Name, "" for undefined or return-value parameters (C++ `name`).
    name: String,
    /// Storage address (C++ `addr`).
    addr: Address,
    /// Data-type (C++ `type`); `None` only for the rare null-typed void boundary.
    type_: Option<Rc<Datatype>>,
    /// Lock and other properties (C++ `flags`).
    flags: uint4,
}

impl ParameterBasic {
    /// Construct from components (C++
    /// `ParameterBasic(const string&,const Address&,Datatype*,uint4)`).
    pub fn new(name: &str, addr: Address, type_: Rc<Datatype>, flags: uint4) -> ParameterBasic {
        ParameterBasic { name: name.to_string(), addr, type_: Some(type_), flags }
    }

    /// Construct a void parameter (C++ `ParameterBasic(Datatype *tp)`).  The
    /// C++ leaves `addr` default-constructed (invalid).
    pub fn new_void(type_: Rc<Datatype>) -> ParameterBasic {
        ParameterBasic {
            name: String::new(),
            addr: Address::new_invalid(),
            type_: Some(type_),
            flags: 0,
        }
    }
}

impl ProtoParameter for ParameterBasic {
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_type(&self) -> Option<&Rc<Datatype>> {
        self.type_.as_ref()
    }
    fn get_address(&self) -> Address {
        self.addr.clone()
    }
    fn get_size(&self) -> int4 {
        self.type_.as_ref().map(|t| t.get_size()).unwrap_or(0)
    }
    fn is_type_locked(&self) -> bool {
        (self.flags & parameter_pieces_flags::TYPELOCK) != 0
    }
    fn is_name_locked(&self) -> bool {
        (self.flags & parameter_pieces_flags::NAMELOCK) != 0
    }
    fn is_size_type_locked(&self) -> bool {
        (self.flags & parameter_pieces_flags::SIZELOCK) != 0
    }
    fn is_this_pointer(&self) -> bool {
        (self.flags & parameter_pieces_flags::ISTHIS) != 0
    }
    fn is_indirect_storage(&self) -> bool {
        (self.flags & parameter_pieces_flags::INDIRECTSTORAGE) != 0
    }
    fn is_hidden_return(&self) -> bool {
        (self.flags & parameter_pieces_flags::HIDDENRETPARM) != 0
    }
    fn is_name_undefined(&self) -> bool {
        self.name.is_empty()
    }

    fn set_type_lock(&mut self, val: bool) {
        if val {
            self.flags |= parameter_pieces_flags::TYPELOCK;
            // Check if we are locking TYPE_UNKNOWN
            if self.type_.as_ref().map(|t| t.get_metatype()) == Some(type_metatype::TYPE_UNKNOWN) {
                self.flags |= parameter_pieces_flags::SIZELOCK;
            }
        } else {
            self.flags &= !(parameter_pieces_flags::TYPELOCK | parameter_pieces_flags::SIZELOCK);
        }
    }
    fn set_name_lock(&mut self, val: bool) {
        if val {
            self.flags |= parameter_pieces_flags::NAMELOCK;
        } else {
            self.flags &= !parameter_pieces_flags::NAMELOCK;
        }
    }
    fn set_this_pointer(&mut self, val: bool) {
        if val {
            self.flags |= parameter_pieces_flags::ISTHIS;
        } else {
            self.flags &= !parameter_pieces_flags::ISTHIS;
        }
    }
    fn override_size_lock_type(&mut self, ct: Rc<Datatype>) -> KunaResult<()> {
        let cur_size = self.type_.as_ref().map(|t| t.get_size()).unwrap_or(0);
        if cur_size == ct.get_size() {
            if !self.is_size_type_locked() {
                return Err(KunaError::lowlevel(
                    "Overriding parameter that is not size locked",
                ));
            }
            self.type_ = Some(ct);
            return Ok(());
        }
        Err(KunaError::lowlevel(
            "Overriding parameter with different type size",
        ))
    }
    fn reset_size_lock_type(&mut self, factory: &dyn TypeFactory) -> KunaResult<()> {
        if self.type_.as_ref().map(|t| t.get_metatype()) == Some(type_metatype::TYPE_UNKNOWN) {
            return Ok(()); // Nothing to do
        }
        let size = self.type_.as_ref().map(|t| t.get_size()).unwrap_or(0);
        self.type_ = Some(factory.get_base(size, type_metatype::TYPE_UNKNOWN)?);
        Ok(())
    }
}

// =============================================================================
// ProtoStore / ProtoStoreInternal (fspec.hh:1198-1330, fspec.cc:3311-3576)
// =============================================================================

/// A collection of parameter descriptions making up a function prototype (C++
/// abstract base `ProtoStore`, `fspec.hh:1198-1249`).
///
/// The symbol-backed variant `ProtoStoreSymbol` reaches `Scope`/`Symbol` (W3)
/// and is a boundary; [`ProtoStoreInternal`] is the stand-alone implementation.
pub trait ProtoStore {
    /// Establish name, data-type, storage of a specific input parameter (C++
    /// `setInput`).
    fn set_input(&mut self, i: int4, nm: &str, pieces: &ParameterPieces);
    /// Clear the input parameter at the specified slot, shifting following
    /// parameters down (C++ `clearInput`).
    fn clear_input(&mut self, i: int4);
    /// Clear all input parameters (C++ `clearAllInputs`).
    fn clear_all_inputs(&mut self);
    /// Get the number of input parameters (C++ `getNumInputs`).
    fn get_num_inputs(&self) -> int4;
    /// Get the i-th input parameter, or `None` (C++ `getInput`).
    fn get_input(&self, i: int4) -> Option<&dyn ProtoParameter>;
    /// Establish the data-type and storage of the return value (C++
    /// `setOutput`).
    fn set_output(&mut self, piece: &ParameterPieces);
    /// Clear the return value to TYPE_VOID (C++ `clearOutput`).
    fn clear_output(&mut self);
    /// Get the return-value description (C++ `getOutput`).
    fn get_output(&self) -> &dyn ProtoParameter;
    /// Clone the entire collection (C++ `clone`).
    fn clone_box(&self) -> Box<dyn ProtoStore>;
    /// Mutable access to the i-th input parameter, or `None`.
    fn get_input_mut(&mut self, i: int4) -> Option<&mut dyn ProtoParameter>;
    /// Mutable access to the return-value description.
    fn get_output_mut(&mut self) -> &mut dyn ProtoParameter;
}

/// A collection of parameter descriptions without backing symbols (C++
/// `ProtoStoreInternal`, `fspec.hh:1312-1330`).
#[derive(Debug, Clone)]
pub struct ProtoStoreInternal {
    /// Cached reference to the void data-type (C++ `voidtype`).
    voidtype: Rc<Datatype>,
    /// Descriptions of input parameters; `None` is a C++ null slot (C++
    /// `inparam`).
    inparam: Vec<Option<ParameterBasic>>,
    /// Description of the return value; `None` is the C++ null (C++ `outparam`).
    outparam: Option<ParameterBasic>,
}

impl ProtoStoreInternal {
    /// Construct with the given void data-type, seeding a void output (C++
    /// `ProtoStoreInternal(Datatype *vt)`).
    pub fn new(vt: Rc<Datatype>) -> ProtoStoreInternal {
        let mut res = ProtoStoreInternal { voidtype: Rc::clone(&vt), inparam: Vec::new(), outparam: None };
        let pieces = ParameterPieces { addr: Address::new_invalid(), type_: Some(vt), flags: 0 };
        res.set_output(&pieces);
        res
    }
}

impl ProtoStore for ProtoStoreInternal {
    fn set_input(&mut self, i: int4, nm: &str, pieces: &ParameterPieces) {
        let i = i as usize;
        while self.inparam.len() <= i {
            self.inparam.push(None);
        }
        self.inparam[i] = Some(ParameterBasic::new(
            nm,
            pieces.addr.clone(),
            pieces.type_.clone().expect("setInput with null type"),
            pieces.flags,
        ));
    }
    fn clear_input(&mut self, i: int4) {
        let sz = self.inparam.len();
        let i = i as usize;
        if i >= sz {
            return;
        }
        self.inparam[i] = None;
        // Renumber parameters with index > i
        for j in (i + 1)..sz {
            self.inparam[j - 1] = self.inparam[j].take();
        }
        while matches!(self.inparam.last(), Some(None)) {
            self.inparam.pop();
        }
    }
    fn clear_all_inputs(&mut self) {
        self.inparam.clear();
    }
    fn get_num_inputs(&self) -> int4 {
        self.inparam.len() as i32
    }
    fn get_input(&self, i: int4) -> Option<&dyn ProtoParameter> {
        if i < 0 || (i as usize) >= self.inparam.len() {
            return None;
        }
        self.inparam[i as usize].as_ref().map(|p| p as &dyn ProtoParameter)
    }
    fn get_input_mut(&mut self, i: int4) -> Option<&mut dyn ProtoParameter> {
        if i < 0 || (i as usize) >= self.inparam.len() {
            return None;
        }
        self.inparam[i as usize].as_mut().map(|p| p as &mut dyn ProtoParameter)
    }
    fn set_output(&mut self, piece: &ParameterPieces) {
        self.outparam = Some(ParameterBasic::new(
            "",
            piece.addr.clone(),
            piece.type_.clone().expect("setOutput with null type"),
            piece.flags,
        ));
    }
    fn clear_output(&mut self) {
        self.outparam = Some(ParameterBasic::new_void(Rc::clone(&self.voidtype)));
    }
    fn get_output(&self) -> &dyn ProtoParameter {
        self.outparam.as_ref().expect("ProtoStoreInternal::get_output: null")
    }
    fn get_output_mut(&mut self) -> &mut dyn ProtoParameter {
        self.outparam.as_mut().expect("ProtoStoreInternal::get_output_mut: null")
    }
    fn clone_box(&self) -> Box<dyn ProtoStore> {
        Box::new(self.clone())
    }
}

// =============================================================================
// FuncProto (fspec.hh:1343-1624, fspec.cc:3783-4628)
// =============================================================================

/// Boolean property flags for a [`FuncProto`] (C++ anonymous enum,
/// `fspec.hh:1344-1359`).
pub mod func_proto_flags {
    use kuna_base::types::uint4;
    /// Set if this prototype takes variable arguments (varargs).
    pub const DOTDOTDOT: uint4 = 1;
    /// Set if this prototype takes no inputs and is locked.
    pub const VOIDINPUTLOCK: uint4 = 2;
    /// Set if the PrototypeModel is locked for this prototype.
    pub const MODELLOCK: uint4 = 4;
    /// Should this be inlined by the decompiler.
    pub const IS_INLINE: uint4 = 8;
    /// Function does not return.
    pub const NO_RETURN: uint4 = 16;
    /// paramshift parameters have been added and removed.
    pub const PARAMSHIFT_APPLIED: uint4 = 32;
    /// Set if the input parameters are not properly represented.
    pub const ERROR_INPUTPARAM: uint4 = 64;
    /// Set if the return value(s) are not properly represented.
    pub const ERROR_OUTPUTPARAM: uint4 = 128;
    /// Parameter storage is custom (not derived from ProtoModel).
    pub const CUSTOM_STORAGE: uint4 = 256;
    /// Function is an (object-oriented) constructor.
    pub const IS_CONSTRUCTOR: uint4 = 0x200;
    /// Function is an (object-oriented) destructor.
    pub const IS_DESTRUCTOR: uint4 = 0x400;
    /// Function is a method with a 'this' pointer as an argument.
    pub const HAS_THISPTR: uint4 = 0x800;
    /// Set if this prototype is created to override a single call site.
    pub const IS_OVERRIDE: uint4 = 0x1000;
    /// Potential output storage should always be considered killed-by-call.
    pub const AUTO_KILLEDBYCALL: uint4 = 0x2000;
}

/// A function prototype: the parameters and return value for a specific
/// function (C++ `FuncProto`, `fspec.hh:1343-1624`).
///
/// The C++ holds a `ProtoModel *model` borrowed from the architecture's
/// registry; with no registry the kuna `FuncProto` owns the model by
/// reference-counted clone ([`Rc<ProtoModel>`]).  The storage interface is a
/// boxed [`ProtoStore`] (an internal store here; the symbol-backed store is a
/// `Scope` boundary).  // STUB(w6-fspec-2)
pub struct FuncProto {
    /// Model for this prototype (C++ `model`); `None` is the C++ null.
    model: Option<Rc<ProtoModel>>,
    /// Storage interface for parameters (C++ `store`); `None` is the C++ null.
    store: Option<Box<dyn ProtoStore>>,
    /// Extra bytes popped from stack (C++ `extrapop`).
    extrapop: int4,
    /// Boolean properties (C++ `flags`).
    flags: uint4,
    /// Side-effects associated with non-parameter storage (C++ `effectlist`).
    effectlist: Vec<EffectRecord>,
    /// Locations that may contain trash values (C++ `likelytrash`).
    likelytrash: Vec<VarnodeData>,
    /// (If non-negative) id of p-code snippet to replace this function (C++
    /// `injectid`).
    injectid: int4,
    /// Number of bytes of return value consumed by callers (0 = all) (C++
    /// `returnBytesConsumed`).
    return_bytes_consumed: int4,
}

impl std::fmt::Debug for FuncProto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // `store` is a trait object with no Debug; omit it.
        f.debug_struct("FuncProto")
            .field("model", &self.model.as_ref().map(|m| m.get_name()))
            .field("extrapop", &self.extrapop)
            .field("flags", &self.flags)
            .field("effectlist", &self.effectlist)
            .field("likelytrash", &self.likelytrash)
            .field("injectid", &self.injectid)
            .field("return_bytes_consumed", &self.return_bytes_consumed)
            .finish_non_exhaustive()
    }
}

impl Default for FuncProto {
    fn default() -> Self {
        FuncProto::new()
    }
}

impl FuncProto {
    /// Construct an empty prototype (C++ `FuncProto(void)`).
    pub fn new() -> FuncProto {
        FuncProto {
            model: None,
            store: None,
            extrapop: 0,
            flags: 0,
            effectlist: Vec::new(),
            likelytrash: Vec::new(),
            injectid: -1,
            return_bytes_consumed: 0,
        }
    }

    /// Copy another function prototype into this (C++ `copy`).
    pub fn copy(&mut self, op2: &FuncProto) {
        self.model = op2.model.clone();
        self.extrapop = op2.extrapop;
        self.flags = op2.flags;
        self.store = op2.store.as_ref().map(|s| s.clone_box());
        self.effectlist = op2.effectlist.clone();
        self.likelytrash = op2.likelytrash.clone();
        self.injectid = op2.injectid;
    }

    /// Copy properties that affect data-flow (C++ `copyFlowEffects`).
    pub fn copy_flow_effects(&mut self, op2: &FuncProto) {
        self.flags &= !(func_proto_flags::IS_INLINE | func_proto_flags::NO_RETURN);
        self.flags |= op2.flags & (func_proto_flags::IS_INLINE | func_proto_flags::NO_RETURN);
        self.injectid = op2.injectid;
    }

    /// If the model is a merged model, decide which one of the merged models
    /// best fits the given trials and set it as the model (C++
    /// `FuncProto::resolveModel`, `fspec.cc:3772`).
    ///
    /// Once a model is chosen it is no longer merged, so re-running is a no-op.
    /// The trials are not re-marked here (that happens in
    /// `ParamList::fillinMap` / `derive_input_map`).
    pub fn resolve_model(&mut self, active: &ParamActive) -> KunaResult<()> {
        let model = match &self.model {
            Some(m) => Rc::clone(m),
            None => return Ok(()),
        };
        if !model.is_merged() {
            return Ok(()); // Already been resolved
        }
        let newmodel = model.select_model(active)?;
        self.set_model(Some(newmodel));
        Ok(())
    }

    /// Does this prototype have a model (C++ `hasModel`).
    pub fn has_model(&self) -> bool {
        self.model.is_some()
    }
    /// Borrow the model (panics if none — C++ would dereference null).
    pub fn model(&self) -> &Rc<ProtoModel> {
        self.model.as_ref().expect("FuncProto::model: null")
    }
    /// Does this use the given model (C++ `hasMatchingModel`).
    pub fn has_matching_model(&self, op2: &Rc<ProtoModel>) -> bool {
        match &self.model {
            Some(m) => Rc::ptr_eq(m, op2),
            None => false,
        }
    }
    /// Get the prototype model name (C++ `getModelName`).
    pub fn get_model_name(&self) -> &str {
        self.model().get_name()
    }
    /// Get the extrapop of the prototype model (C++ `getModelExtraPop`).
    pub fn get_model_extra_pop(&self) -> int4 {
        self.model().get_extra_pop()
    }
    /// Return true if the prototype model is unknown (C++ `isModelUnknown`).
    pub fn is_model_unknown(&self) -> bool {
        self.model().is_unknown()
    }
    /// Return true if the name should be printed in declarations (C++
    /// `printModelInDecl`).
    pub fn print_model_in_decl(&self) -> bool {
        self.model().print_in_decl()
    }

    /// Establish a specific prototype model (C++ `setModel`).
    pub fn set_model(&mut self, m: Option<Rc<ProtoModel>>) {
        match m {
            Some(m) => {
                let expop = m.get_extra_pop();
                // If a model previously existed don't overwrite extrapop with unknown
                if self.model.is_none() || expop != EXTRAPOP_UNKNOWN {
                    self.extrapop = expop;
                }
                if m.has_this_pointer() {
                    self.flags |= func_proto_flags::HAS_THISPTR;
                }
                if m.is_constructor() {
                    self.flags |= func_proto_flags::IS_CONSTRUCTOR;
                }
                if m.is_auto_killed_by_call() {
                    self.flags |= func_proto_flags::AUTO_KILLEDBYCALL;
                }
                self.model = Some(m);
            }
            None => {
                self.model = None;
                self.extrapop = EXTRAPOP_UNKNOWN;
            }
        }
    }

    /// Set internal backing storage (C++ `setInternal`).
    pub fn set_internal(&mut self, m: Rc<ProtoModel>, vt: Rc<Datatype>) {
        self.store = Some(Box::new(ProtoStoreInternal::new(vt)));
        if self.model.is_none() {
            self.set_model(Some(m));
        }
    }

    /// Set a backing symbol `Scope` for this (C++ `setScope`).
    ///
    /// STUB(w6-fspec-2): `ProtoStoreSymbol` reaches `Scope`/`Symbol` (W3) and
    /// `glb->defaultfp` (the Architecture registry).  Use [`FuncProto::set_internal`]
    /// in the meantime.
    pub fn set_scope(&mut self) -> KunaResult<()> {
        Err(KunaError::lowlevel(
            "STUB(w6-fspec-2) FuncProto::setScope: ProtoStoreSymbol needs Scope + defaultfp",
        ))
    }

    /// Get the i-th input parameter (C++ `getParam`).
    pub fn get_param(&self, i: int4) -> Option<&dyn ProtoParameter> {
        self.store().get_input(i)
    }
    /// Set parameter storage directly (C++ `setParam`).
    pub fn set_param(&mut self, i: int4, name: &str, piece: &ParameterPieces) {
        self.store_mut().set_input(i, name, piece);
    }
    /// Remove the i-th input parameter (C++ `removeParam`).
    pub fn remove_param(&mut self, i: int4) {
        self.store_mut().clear_input(i);
    }
    /// Get the number of input parameters (C++ `numParams`).
    pub fn num_params(&self) -> int4 {
        self.store().get_num_inputs()
    }
    /// Get the return value (C++ `getOutput`).
    pub fn get_output(&self) -> &dyn ProtoParameter {
        self.store().get_output()
    }
    /// Set return value storage directly (C++ `setOutput`).
    pub fn set_output(&mut self, piece: &ParameterPieces) {
        self.store_mut().set_output(piece);
    }
    /// Clear all input parameters from the store (C++ `store->clearAllInputs()`),
    /// used by `updateInputTypes`/`updateInputNoTypes` before re-registering the
    /// recovered parameters.  A no-op if no store is attached.
    pub fn store_clear_all_inputs(&mut self) {
        if self.store.is_some() {
            self.store_mut().clear_all_inputs();
        }
    }
    /// Set the `i`-th input parameter storage directly (C++ `store->setInput`),
    /// used by `updateInputTypes`/`updateInputNoTypes`.  A no-op if no store is
    /// attached.
    pub fn store_set_input(&mut self, i: int4, nm: &str, pieces: &ParameterPieces) {
        if self.store.is_some() {
            self.store_mut().set_input(i, nm, pieces);
        }
    }
    /// Get the return value data-type (C++ `getOutputType`).
    pub fn get_output_type(&self) -> Option<&Rc<Datatype>> {
        // Borrow chain: get_output returns &dyn, get_type returns Option<&Rc>.
        // The lifetime is tied to &self via store(); safe.
        self.store().get_output().get_type()
    }

    /// Whether a backing parameter store has been attached (C++ `store != 0`).
    ///
    /// The merged-tree `Funcdata::new` cannot run the C++
    /// `funcp.setScope(localmap,...)` (`ProtoStoreSymbol` needs the symbol scope,
    /// a W4 boundary), so the recovered proto carries a model but no store.  The
    /// store-dependent queries below fall back to the unrecovered default (void,
    /// unlocked) when this is false, rather than panic.
    pub fn has_store(&self) -> bool {
        self.store.is_some()
    }

    /// Attach a stand-alone [`ProtoStoreInternal`] (the C++ store used when there
    /// is no symbol scope — `ProtoStoreInternal`, fspec.cc:3311) seeded with the
    /// given void type.  Idempotent: a no-op if a store is already present.  This
    /// is the merged-tree path for output/input recovery without the W4
    /// `ProtoStoreSymbol`/ScopeLocal.
    pub fn attach_internal_store(&mut self, void_type: Rc<Datatype>) {
        if self.store.is_none() {
            self.store = Some(Box::new(ProtoStoreInternal::new(void_type)));
        }
    }
    /// Borrow the store (panics if none).
    fn store(&self) -> &dyn ProtoStore {
        self.store.as_deref().expect("FuncProto::store: null")
    }
    /// Borrow the store mutably (panics if none).
    fn store_mut(&mut self) -> &mut dyn ProtoStore {
        self.store.as_deref_mut().expect("FuncProto::store: null")
    }

    // -- lock predicates / setters (fspec.cc:3911-3953) ---------------------

    /// Are input data-types locked (C++ `isInputLocked`).
    pub fn is_input_locked(&self) -> bool {
        if (self.flags & func_proto_flags::VOIDINPUTLOCK) != 0 {
            return true;
        }
        // No store (merged-tree setScope boundary): the unrecovered input is unlocked
        // (no parameters), so the param-lock query reads false rather than panic
        // dereferencing a null store — same convention as `is_output_locked`.
        if !self.has_store() {
            return false;
        }
        if self.num_params() == 0 {
            return false;
        }
        self.get_param(0).map(|p| p.is_type_locked()).unwrap_or(false)
    }
    /// Is the output data-type locked (C++ `isOutputLocked`).
    pub fn is_output_locked(&self) -> bool {
        // No store (merged-tree setScope boundary): the unrecovered output is void
        // and never locked.
        if !self.has_store() {
            return false;
        }
        self.store().get_output().is_type_locked()
    }
    /// Is the prototype model locked (C++ `isModelLocked`).
    pub fn is_model_locked(&self) -> bool {
        (self.flags & func_proto_flags::MODELLOCK) != 0
    }
    /// Is this a "custom" function prototype (C++ `hasCustomStorage`).
    pub fn has_custom_storage(&self) -> bool {
        (self.flags & func_proto_flags::CUSTOM_STORAGE) != 0
    }
    /// Toggle the data-type lock on input parameters (C++ `setInputLock`).
    pub fn set_input_lock(&mut self, val: bool) {
        if val {
            self.flags |= func_proto_flags::MODELLOCK; // Locking input locks the model
        }
        let num = self.num_params();
        if num == 0 {
            self.flags = if val {
                self.flags | func_proto_flags::VOIDINPUTLOCK
            } else {
                self.flags & !func_proto_flags::VOIDINPUTLOCK
            };
            return;
        }
        for i in 0..num {
            if let Some(param) = self.store_mut().get_input_mut(i) {
                param.set_type_lock(val);
                // (kuna) C++ `setInputLock` calls `param->setTypeLock(val)` where
                // `param` is a symbol-backed `ParameterSymbol`, whose
                // `setTypeLock` (fspec.cc:3052-3062) ALSO toggles the NAME lock
                // for a NAMED parameter (`if (!sym->isNameUndefined()) attrs |=
                // Varnode::namelock`).  The rust port collapses both C++ param
                // subclasses onto `ParameterBasic`, whose `set_type_lock` matches
                // only the plain `ParameterBasic::setTypeLock` (no namelock).  A
                // function prototype committed via `parse line` / `setPieces` is
                // symbol-backed in the oracle, so its named params end up
                // type+name locked (verified under gdb: the `receive` proto params
                // carry `flags 0x300` at `ActionNameVars::makeRec` time).
                // Replicate the symbol-backed `setTypeLock` here so a named,
                // committed param is name-locked — the gate
                // `ActionNameVars::lookForFuncParamNames` requires to propagate a
                // callee parameter name to its argument local.
                if !param.is_name_undefined() {
                    param.set_name_lock(val);
                }
            }
        }
    }
    /// Toggle the data-type lock on the return value (C++ `setOutputLock`).
    pub fn set_output_lock(&mut self, val: bool) {
        if val {
            self.flags |= func_proto_flags::MODELLOCK; // Locking output locks the model
        }
        self.store_mut().get_output_mut().set_type_lock(val);
    }
    /// Toggle the lock on the prototype model (C++ `setModelLock`).
    pub fn set_model_lock(&mut self, val: bool) {
        self.flags = if val {
            self.flags | func_proto_flags::MODELLOCK
        } else {
            self.flags & !func_proto_flags::MODELLOCK
        };
    }

    // -- simple flag accessors (fspec.hh:1411-1545) -------------------------

    /// Does this function get in-lined (C++ `isInline`).
    pub fn is_inline(&self) -> bool {
        (self.flags & func_proto_flags::IS_INLINE) != 0
    }
    /// Toggle the in-line setting (C++ `setInline`).
    pub fn set_inline(&mut self, val: bool) {
        self.flags = if val {
            self.flags | func_proto_flags::IS_INLINE
        } else {
            self.flags & !func_proto_flags::IS_INLINE
        };
    }
    /// Get the injection id (C++ `getInjectId`).
    pub fn get_inject_id(&self) -> int4 {
        self.injectid
    }
    /// Get the bytes consumed by callers (C++ `getReturnBytesConsumed`).
    pub fn get_return_bytes_consumed(&self) -> int4 {
        self.return_bytes_consumed
    }
    /// Does a function with this prototype never return (C++ `isNoReturn`).
    pub fn is_no_return(&self) -> bool {
        (self.flags & func_proto_flags::NO_RETURN) != 0
    }
    /// Toggle the no-return setting (C++ `setNoReturn`).
    pub fn set_no_return(&mut self, val: bool) {
        self.flags = if val {
            self.flags | func_proto_flags::NO_RETURN
        } else {
            self.flags & !func_proto_flags::NO_RETURN
        };
    }
    /// Is this a prototype for a class method, taking a this pointer (C++
    /// `hasThisPointer`).
    pub fn has_this_pointer(&self) -> bool {
        (self.flags & func_proto_flags::HAS_THISPTR) != 0
    }
    /// Is this prototype for a class constructor (C++ `isConstructor`).
    pub fn is_constructor(&self) -> bool {
        (self.flags & func_proto_flags::IS_CONSTRUCTOR) != 0
    }
    /// Toggle whether this is a constructor (C++ `setConstructor`).
    pub fn set_constructor(&mut self, val: bool) {
        self.flags = if val {
            self.flags | func_proto_flags::IS_CONSTRUCTOR
        } else {
            self.flags & !func_proto_flags::IS_CONSTRUCTOR
        };
    }
    /// Is this prototype for a class destructor (C++ `isDestructor`).
    pub fn is_destructor(&self) -> bool {
        (self.flags & func_proto_flags::IS_DESTRUCTOR) != 0
    }
    /// Toggle whether this is a destructor (C++ `setDestructor`).
    pub fn set_destructor(&mut self, val: bool) {
        self.flags = if val {
            self.flags | func_proto_flags::IS_DESTRUCTOR
        } else {
            self.flags & !func_proto_flags::IS_DESTRUCTOR
        };
    }
    /// Has this been marked as having incorrect input parameters (C++
    /// `hasInputErrors`).
    pub fn has_input_errors(&self) -> bool {
        (self.flags & func_proto_flags::ERROR_INPUTPARAM) != 0
    }
    /// Has this been marked as having an incorrect return value (C++
    /// `hasOutputErrors`).
    pub fn has_output_errors(&self) -> bool {
        (self.flags & func_proto_flags::ERROR_OUTPUTPARAM) != 0
    }
    /// Toggle the input error setting (C++ `setInputErrors`).
    pub fn set_input_errors(&mut self, val: bool) {
        self.flags = if val {
            self.flags | func_proto_flags::ERROR_INPUTPARAM
        } else {
            self.flags & !func_proto_flags::ERROR_INPUTPARAM
        };
    }
    /// Toggle the output error setting (C++ `setOutputErrors`).
    pub fn set_output_errors(&mut self, val: bool) {
        self.flags = if val {
            self.flags | func_proto_flags::ERROR_OUTPUTPARAM
        } else {
            self.flags & !func_proto_flags::ERROR_OUTPUTPARAM
        };
    }
    /// Get the general extrapop setting (C++ `getExtraPop`).
    pub fn get_extra_pop(&self) -> int4 {
        self.extrapop
    }
    /// Set the general extrapop (C++ `setExtraPop`).
    pub fn set_extra_pop(&mut self, ep: int4) {
        self.extrapop = ep;
    }
    /// Get any upon-entry injection id (C++ `getInjectUponEntry`).
    pub fn get_inject_upon_entry(&self) -> int4 {
        self.model().get_inject_upon_entry()
    }
    /// Get any upon-return injection id (C++ `getInjectUponReturn`).
    pub fn get_inject_upon_return(&self) -> int4 {
        self.model().get_inject_upon_return()
    }
    /// Return true if this takes a variable number of arguments (C++
    /// `isDotdotdot`).
    pub fn is_dotdotdot(&self) -> bool {
        (self.flags & func_proto_flags::DOTDOTDOT) != 0
    }
    /// Toggle whether this takes variable arguments (C++ `setDotdotdot`).
    pub fn set_dotdotdot(&mut self, val: bool) {
        self.flags = if val {
            self.flags | func_proto_flags::DOTDOTDOT
        } else {
            self.flags & !func_proto_flags::DOTDOTDOT
        };
    }
    /// Return true if this is a call site override (C++ `isOverride`).
    pub fn is_override(&self) -> bool {
        (self.flags & func_proto_flags::IS_OVERRIDE) != 0
    }
    /// Toggle whether this is a call site override (C++ `setOverride`).
    pub fn set_override(&mut self, val: bool) {
        self.flags = if val {
            self.flags | func_proto_flags::IS_OVERRIDE
        } else {
            self.flags & !func_proto_flags::IS_OVERRIDE
        };
    }
    /// Has a parameter shift been applied (C++ `isParamshiftApplied`).
    pub fn is_paramshift_applied(&self) -> bool {
        (self.flags & func_proto_flags::PARAMSHIFT_APPLIED) != 0
    }
    /// Toggle whether a parameter shift has been applied (C++
    /// `setParamshiftApplied`).
    pub fn set_paramshift_applied(&mut self, val: bool) {
        self.flags = if val {
            self.flags | func_proto_flags::PARAMSHIFT_APPLIED
        } else {
            self.flags & !func_proto_flags::PARAMSHIFT_APPLIED
        };
    }
    /// Get the comparable properties of this prototype (C++
    /// `getComparableFlags`).
    pub fn get_comparable_flags(&self) -> uint4 {
        self.flags
            & (func_proto_flags::DOTDOTDOT
                | func_proto_flags::IS_CONSTRUCTOR
                | func_proto_flags::IS_DESTRUCTOR
                | func_proto_flags::HAS_THISPTR)
    }

    /// Provide a hint as to how many bytes of the return value are important
    /// (C++ `setReturnBytesConsumed`).  Returns true if the smallest hint
    /// changed.
    pub fn set_return_bytes_consumed(&mut self, val: int4) -> bool {
        if val == 0 {
            return false;
        }
        if self.return_bytes_consumed == 0 || val < self.return_bytes_consumed {
            self.return_bytes_consumed = val;
            return true;
        }
        false
    }

    /// Assuming this prototype is locked, calculate the extrapop (C++
    /// `resolveExtraPop`).  Designed to work with 32-bit x86 binaries.
    pub fn resolve_extra_pop(&mut self) {
        if !self.is_input_locked() {
            return;
        }
        let numparams = self.num_params();
        if self.is_dotdotdot() {
            if numparams != 0 {
                // "standard" varargs with fixed initial parameters -> __cdecl
                self.set_extra_pop(4);
            }
            return; // otherwise we can't resolve the extrapop
        }
        let mut expop = 4; // Extrapop is at least 4 for the return address
        for i in 0..numparams {
            let param = match self.get_param(i) {
                Some(p) => p,
                None => continue,
            };
            let addr = param.get_address();
            if addr.get_space().map(|s| s.get_type()) != Some(spacetype::IPTR_SPACEBASE) {
                continue;
            }
            // (int4)addr.getOffset() + param->getSize()
            let mut cur = (addr.get_offset() as i32).wrapping_add(param.get_size());
            cur = (cur + 3) & 0xffffffc; // Must be 4-byte aligned
            if cur > expop {
                expop = cur;
            }
        }
        self.set_extra_pop(expop);
    }

    /// Clear input parameters that have not been locked (C++
    /// `clearUnlockedInput`).
    pub fn clear_unlocked_input(&mut self) {
        if self.is_input_locked() {
            return;
        }
        // No store (merged-tree setScope boundary): no inputs to clear.
        if self.store.is_some() {
            self.store_mut().clear_all_inputs();
        }
    }

    /// Clear the return value if it has not been locked (C++
    /// `clearUnlockedOutput`).
    pub fn clear_unlocked_output(&mut self, factory: &dyn TypeFactory) -> KunaResult<()> {
        let (type_locked, size_locked) = {
            let outparam = self.get_output();
            (outparam.is_type_locked(), outparam.is_size_type_locked())
        };
        if type_locked {
            if size_locked && self.model.is_some() {
                self.store_mut().get_output_mut().reset_size_lock_type(factory)?;
            }
        } else {
            self.store_mut().clear_output();
        }
        self.return_bytes_consumed = 0;
        Ok(())
    }

    /// Clear all input parameters regardless of lock (C++ `clearInput`).
    pub fn clear_input(&mut self) {
        self.store_mut().clear_all_inputs();
        self.flags &= !func_proto_flags::VOIDINPUTLOCK; // If a void was locked in clear it
    }

    /// Associate a given injection with this prototype (C++ `setInjectId`).
    pub fn set_inject_id(&mut self, id: int4) {
        if id < 0 {
            self.cancel_inject_id();
        } else {
            self.injectid = id;
            self.flags |= func_proto_flags::IS_INLINE;
        }
    }
    /// Turn-off any in-lining for this function (C++ `cancelInjectId`).
    pub fn cancel_inject_id(&mut self) {
        self.injectid = -1;
        self.flags &= !func_proto_flags::IS_INLINE;
    }

    /// Make sure any "this" parameter is properly marked (C++
    /// `updateThisPointer`).
    pub fn update_this_pointer(&mut self) {
        if !self.model().has_this_pointer() {
            return;
        }
        let num_inputs = self.store().get_num_inputs();
        if num_inputs == 0 {
            return;
        }
        let mut idx = 0;
        if self.store().get_input(0).map(|p| p.is_hidden_return()).unwrap_or(false) {
            if num_inputs < 2 {
                return;
            }
            idx = 1;
        }
        if let Some(param) = self.store_mut().get_input_mut(idx) {
            param.set_this_pointer(true);
        }
    }

    /// Copy out the raw pieces of this prototype (C++ `getPieces`).
    pub fn get_pieces(&self, pieces: &mut PrototypePieces) {
        if self.store.is_none() {
            return;
        }
        pieces.outtype = self.store().get_output().get_type().cloned();
        let num = self.store().get_num_inputs();
        for i in 0..num {
            if let Some(param) = self.store().get_input(i) {
                if let Some(t) = param.get_type() {
                    pieces.intypes.push(Rc::clone(t));
                }
                pieces.innames.push(param.get_name().to_string());
            }
        }
        pieces.first_var_arg_slot = if self.is_dotdotdot() { num } else { -1 };
    }

    /// The full prototype is (re)set from a model, names, and data-types (C++
    /// `setPieces`).  Both input and output are assumed locked.
    pub fn set_pieces(
        &mut self,
        pieces: &PrototypePieces,
        model: Option<Rc<ProtoModel>>,
        typefactory: &dyn TypeFactory,
        manager: &AddrSpaceManager,
    ) -> KunaResult<()> {
        if model.is_some() {
            self.set_model(model);
        }
        self.update_all_types(pieces, typefactory, manager)?;
        // (kuna) A console `map return <addr>` parks an explicit, model-overriding
        // locked output storage on the pieces.  `update_all_types` re-derived the
        // output from the model (e.g. RAX for an int8 return); replace it with the
        // custom storage so a stack-relative return survives the callee-proto
        // reconstruction (C++ keeps it verbatim via `fc->copy(callee FuncProto)`).
        if let Some(custom_out) = pieces.output_storage.as_ref() {
            self.store_mut().set_output(custom_out);
        }
        // (kuna) The same override on the input side, for the slots a caller
        // named explicitly (`map param <func>::<i> <storage> <decl>`).  Only a
        // slot the model actually assigned is replaced, so a stale index cannot
        // punch a hole into the parameter list.
        let assigned = self.store().get_num_inputs();
        for (slot, custom_in) in pieces.input_storage.iter() {
            if *slot < 0 || *slot >= assigned {
                continue;
            }
            let name = pieces
                .innames
                .get(*slot as usize)
                .map(String::as_str)
                .unwrap_or("");
            self.store_mut().set_input(*slot, name, custom_in);
        }
        self.set_input_lock(true);
        self.set_output_lock(true);
        self.set_model_lock(true);
        Ok(())
    }

    /// Seed an empty (fresh-`Funcdata`) prototype from a parsed C declaration and
    /// lock it (the merged-tree analogue of C++ `Architecture::setPrototype`
    /// followed by the `FuncProto::setScope`/restore that a `queryFunction`
    /// performs on a freshly built `Funcdata`).
    ///
    /// The console `parse line extern <decl>` captures a [`PrototypePieces`]; in
    /// C++ that pieces set is applied to the function symbol's `FuncProto`
    /// (already model+store seeded by `setScope`).  Here the fresh `Funcdata`
    /// carries an empty `FuncProto` (no model, no store), so this first seeds the
    /// `defaultfp` model and a stand-alone [`ProtoStoreInternal`] (the
    /// no-symbol-scope store, matching [`attach_internal_store`]) and then runs
    /// the faithful [`set_pieces`] body (which calls `update_all_types` +
    /// input/output/model lock).  After this the input/output is type-locked, so
    /// `ActionPrototypeTypes` forces the input/output Varnodes and
    /// `ActionInputPrototype` leaves the locked input untouched.
    ///
    /// (kuna `cppsig`) An `outtype` of `None` with no `output_storage` means "the
    /// source declares the PARAMETERS but not the return type" — the shape a C++
    /// mangled symbol has, since the Itanium ABI encodes a return type only for a
    /// template function. Storage assignment needs *some* output to work with, so
    /// `void` is seeded and the OUTPUT lock is released again, leaving the return
    /// type to whatever recovery finds. This is upstream's behavior:
    /// `DemangledFunction.resolveReturnType` returns null in exactly this case and
    /// `ApplyFunctionSignatureCmd` keeps the function's existing return type;
    /// inventing `void` would DELETE every recovered return value. Handled here
    /// rather than at one call site so the caller-side rebuild
    /// (`ActionDefaultParams`, which types a call's arguments from the callee's
    /// parked pieces) sees the same contract. Distinct from the `map return`
    /// output-only pieces, which carry `output_storage`.
    ///
    /// [`attach_internal_store`]: FuncProto::attach_internal_store
    /// [`set_pieces`]: FuncProto::set_pieces
    pub fn seed_locked_from_pieces(
        &mut self,
        pieces: &PrototypePieces,
        defaultfp: Rc<ProtoModel>,
        void_type: Rc<Datatype>,
        typefactory: &dyn TypeFactory,
        manager: &AddrSpaceManager,
    ) -> KunaResult<()> {
        // Seed the model + store the way setScope would, before update_all_types
        // (which needs both: it does setModel(model) and store->clearAllInputs()).
        self.attach_internal_store(Rc::clone(&void_type));
        if pieces.outtype.is_none() && pieces.output_storage.is_none() {
            let mut input_only = pieces.clone();
            input_only.outtype = Some(void_type);
            self.set_pieces(&input_only, Some(defaultfp), typefactory, manager)?;
            self.set_output_lock(false);
            return Ok(());
        }
        // (kuna) A `map return <addr> <type>` parks OUTPUT-ONLY pieces: explicit
        // storage, and no `outtype` because the directive declares no separate
        // return type. But `assignParameterStorage` dereferences `outtype`
        // unconditionally, so those pieces aborted the process the moment the
        // function was decompiled. The declared type IS the return type — adopt it
        // and the model has something to assign against, after which `set_pieces`
        // replaces the derived storage with the explicit one as before.
        if pieces.outtype.is_none() {
            if let Some(declared) = pieces.output_storage.as_ref().and_then(|p| p.type_.clone()) {
                let mut typed = pieces.clone();
                typed.outtype = Some(declared);
                return self.set_pieces(&typed, Some(defaultfp), typefactory, manager);
            }
        }
        self.set_pieces(pieces, Some(defaultfp), typefactory, manager)
    }

    /// Update input/output parameters from raw pieces (C++ `updateAllTypes`).
    ///
    /// Resets `extrapop` via `setModel(model)`, clears the store, recomputes the
    /// storage locations via `assignParameterStorage`, and re-marks the `this`
    /// pointer.  A `ParamUnassignedError` sets the input-error flag.
    pub fn update_all_types(
        &mut self,
        proto: &PrototypePieces,
        typefactory: &dyn TypeFactory,
        manager: &AddrSpaceManager,
    ) -> KunaResult<()> {
        self.set_model(self.model.clone()); // This resets extrapop
        self.store_mut().clear_all_inputs();
        self.store_mut().clear_output();
        self.flags &= !func_proto_flags::VOIDINPUTLOCK;
        self.set_dotdotdot(proto.first_var_arg_slot >= 0);

        let mut pieces: Vec<ParameterPieces> = Vec::new();
        let model = self.model().clone();
        match model.assign_parameter_storage(proto, &mut pieces, false, typefactory, manager) {
            Ok(()) => {
                self.store_mut().set_output(&pieces[0]);
                let mut j = 0usize;
                for (i, piece) in pieces.iter().enumerate().skip(1) {
                    if (piece.flags & parameter_pieces_flags::HIDDENRETPARM) != 0 {
                        self.store_mut().set_input((i - 1) as i32, "rethidden", piece);
                        continue; // increment i but not j
                    }
                    let nm = if j >= proto.innames.len() { "" } else { proto.innames[j].as_str() };
                    let nm = nm.to_string();
                    self.store_mut().set_input((i - 1) as i32, &nm, piece);
                    j += 1;
                }
            }
            Err(KunaError::ParamUnassigned { .. }) => {
                // ParamUnassignedError
                self.flags |= func_proto_flags::ERROR_INPUTPARAM;
            }
            Err(e) => return Err(e),
        }
        self.update_this_pointer();
        Ok(())
    }

    // -- effect / query delegators (fspec.cc:4239-4622) ---------------------

    /// Calculate the effect this has on a given storage location (C++
    /// `hasEffect`).
    ///
    /// C++ dereferences `model` unconditionally when `effectlist` is empty; in the
    /// real pipeline the model is always present (a proto is always constructed with
    /// one).  In the rust port the model is an `Option` (test fixtures may build a
    /// bare `FuncProto` with none); a missing model contributes no EffectRecord, so
    /// this returns `UNKNOWN_EFFECT` — the "absence of an EffectRecord" value — which
    /// is the no-op the caller (`setInputVarnode`) already treats as "no marking".
    pub fn has_effect(&self, addr: &Address, size: int4) -> uint4 {
        if self.effectlist.is_empty() {
            return match self.model.as_ref() {
                Some(m) => m.has_effect(addr, size),
                None => effect_type::UNKNOWN_EFFECT,
            };
        }
        ProtoModel::lookup_effect(&self.effectlist, addr, size)
    }

    /// Get the effect list (C++ `effectBegin`/`effectEnd`): the override list if
    /// non-empty, else the model's list.
    ///
    /// C++ iterates `model->effectlist` when the override is empty; in the rust port
    /// the model is an `Option` (a model-less test fixture would otherwise deref
    /// null), so a missing model yields the empty slice — the "no side-effect
    /// records" state, which is what an unconfigured proto reports.
    pub fn effect_list(&self) -> &[EffectRecord] {
        if self.effectlist.is_empty() {
            match self.model.as_ref() {
                Some(m) => m.effect_list(),
                None => &[],
            }
        } else {
            &self.effectlist
        }
    }
    /// (kuna `calleepreserves`) Does this proto carry its OWN effect records,
    /// rather than deferring to the model's?  A non-empty override list is a
    /// deliberate statement about this function's side effects and is never
    /// second-guessed by the callee-body probe.
    pub fn has_effect_override(&self) -> bool {
        !self.effectlist.is_empty()
    }

    /// Append an effect-record override and re-sort (test/builder hook, mirroring
    /// [`ProtoModel::push_effect`]).
    #[cfg(test)]
    pub fn push_effect_override(&mut self, eff: EffectRecord) {
        self.effectlist.push(eff);
        self.effectlist.sort_by(EffectRecord::compare_by_address);
    }

    /// Get the likely-trash list (C++ `trashBegin`/`trashEnd`).
    pub fn trash_list(&self) -> &[VarnodeData] {
        if self.likelytrash.is_empty() {
            self.model().trash_list()
        } else {
            &self.likelytrash
        }
    }
    /// Get the internal-storage list (C++ `internalBegin`/`internalEnd`).
    pub fn internal_list(&self) -> &[VarnodeData] {
        self.model().internal_list()
    }

    /// Decide whether a storage location could be, or hold, an input parameter
    /// (C++ `characterizeAsInputParam`).
    pub fn characterize_as_input_param(&self, addr: &Address, size: int4) -> Containment {
        if !self.is_dotdotdot() {
            // If varargs, go straight to the model
            if (self.flags & func_proto_flags::VOIDINPUTLOCK) != 0 {
                return Containment::NoContainment;
            }
            let num = self.num_params();
            if num > 0 {
                let mut locktest = false;
                let mut res_contains = false;
                let mut res_contained_by = false;
                for i in 0..num {
                    let param = match self.get_param(i) {
                        Some(p) => p,
                        None => continue,
                    };
                    if !param.is_type_locked() {
                        continue;
                    }
                    locktest = true;
                    let iaddr = param.get_address();
                    // Must be justified relative to space endianness, ignoring forceleft
                    let off = iaddr.justified_contain(param.get_size(), addr, size, false);
                    if off == 0 {
                        return Containment::ContainsJustified;
                    } else if off > 0 {
                        res_contains = true;
                    }
                    if iaddr.contained_by(param.get_size(), addr, size) {
                        res_contained_by = true;
                    }
                }
                if locktest {
                    if res_contains {
                        return Containment::ContainsUnjustified;
                    }
                    if res_contained_by {
                        return Containment::ContainedBy;
                    }
                    return Containment::NoContainment;
                }
            }
        }
        self.model().characterize_as_input_param(addr, size)
    }

    /// Given a list of output \e trials, derive the most likely return value for
    /// this prototype (C++ `FuncProto::deriveOutputMap`, fspec.hh:1501 —
    /// `model->deriveOutputMap(active)`).
    pub fn derive_output_map(
        &self,
        active: &mut ParamActive,
        manager: &AddrSpaceManager,
    ) -> KunaResult<()> {
        self.model().derive_output_map(active, manager)
    }

    /// Derive the most likely input prototype from a list of trials (C++
    /// `FuncProto::deriveInputMap`, fspec.hh:1494 — `model->deriveInputMap(active)`).
    pub fn derive_input_map(
        &self,
        active: &mut ParamActive,
        manager: &AddrSpaceManager,
    ) -> KunaResult<()> {
        self.model().derive_input_map(active, manager)
    }

    /// The input model's [`ParamEntry`] list (C++ `model->getInput()` entries),
    /// cloned so a `ParamActive` trial sort can run while the proto is no longer
    /// borrowed.  Used by `buildInputFromTrials`'s `sortFixedPosition`.
    pub fn input_param_entries(&self) -> Vec<ParamEntry> {
        self.model().input().get_entry().to_vec()
    }

    /// Decide whether a storage location could be, or hold, the return value
    /// (C++ `characterizeAsOutput`).
    pub fn characterize_as_output(&self, addr: &Address, size: int4) -> Containment {
        if self.is_output_locked() {
            let outparam = self.get_output();
            if outparam.get_type().map(|t| t.get_metatype()) == Some(type_metatype::TYPE_VOID) {
                return Containment::NoContainment;
            }
            let iaddr = outparam.get_address();
            let off = iaddr.justified_contain(outparam.get_size(), addr, size, false);
            if off == 0 {
                return Containment::ContainsJustified;
            } else if off > 0 {
                return Containment::ContainsUnjustified;
            }
            if iaddr.contained_by(outparam.get_size(), addr, size) {
                return Containment::ContainedBy;
            }
            return Containment::NoContainment;
        }
        self.model().characterize_as_output(addr, size)
    }

    /// Decide whether a storage location could be an input parameter (C++
    /// `possibleInputParam`).
    pub fn possible_input_param(&self, addr: &Address, size: int4) -> bool {
        if !self.is_dotdotdot() {
            if (self.flags & func_proto_flags::VOIDINPUTLOCK) != 0 {
                return false;
            }
            // (kuna) C++ FuncProto always has a ProtoStore; the merged kuna
            // FuncProto may have its `ProtoStoreInternal` un-attached at
            // main-loop time (un-recovered output/input). With no store there
            // are no locked params to test, so fall straight through to the
            // model -- mirrors C++ with `numParams()==0`. Guard the store call
            // so it never panics.
            let num = if self.has_store() { self.num_params() } else { 0 };
            if num > 0 {
                let mut locktest = false;
                for i in 0..num {
                    let param = match self.get_param(i) {
                        Some(p) => p,
                        None => continue,
                    };
                    if !param.is_type_locked() {
                        continue;
                    }
                    locktest = true;
                    let iaddr = param.get_address();
                    if iaddr.justified_contain(param.get_size(), addr, size, false) == 0 {
                        return true;
                    }
                }
                if locktest {
                    return false;
                }
            }
        }
        // (kuna) C++ always has a model here; an un-recovered kuna FuncProto may
        // not -- with no model nothing is yet a possible parameter.
        match &self.model {
            Some(m) => m.possible_input_param(addr, size),
            None => false,
        }
    }

    /// Decide whether a storage location could be a return value (C++
    /// `possibleOutputParam`).
    pub fn possible_output_param(&self, addr: &Address, size: int4) -> bool {
        if self.is_output_locked() {
            let outparam = self.get_output();
            if outparam.get_type().map(|t| t.get_metatype()) == Some(type_metatype::TYPE_VOID) {
                return false;
            }
            let iaddr = outparam.get_address();
            return iaddr.justified_contain(outparam.get_size(), addr, size, false) == 0;
        }
        self.model().possible_output_param(addr, size)
    }

    /// Check if the storage looks like an unjustified input parameter (C++
    /// `unjustifiedInputParam`).
    pub fn unjustified_input_param(&self, addr: &Address, size: int4, res: &mut VarnodeData) -> bool {
        if !self.is_dotdotdot() {
            if (self.flags & func_proto_flags::VOIDINPUTLOCK) != 0 {
                return false;
            }
            let num = self.num_params();
            if num > 0 {
                let mut locktest = false;
                for i in 0..num {
                    let param = match self.get_param(i) {
                        Some(p) => p,
                        None => continue,
                    };
                    if !param.is_type_locked() {
                        continue;
                    }
                    locktest = true;
                    let iaddr = param.get_address();
                    let just = iaddr.justified_contain(param.get_size(), addr, size, false);
                    if just == 0 {
                        return false; // Contained but not improperly
                    }
                    if just > 0 {
                        res.space = iaddr.get_space().cloned();
                        res.offset = iaddr.get_offset();
                        res.size = param.get_size() as u32;
                        return true;
                    }
                }
                if locktest {
                    return false;
                }
            }
        }
        self.model().unjustified_input_param(addr, size, res)
    }

    /// Pass back the biggest input parameter contained in the range (C++
    /// `getBiggestContainedInputParam`).
    pub fn get_biggest_contained_input_param(&self, loc: &Address, size: int4, res: &mut VarnodeData) -> bool {
        if !self.is_dotdotdot() {
            if (self.flags & func_proto_flags::VOIDINPUTLOCK) != 0 {
                return false;
            }
            let num = self.num_params();
            if num > 0 {
                let mut locktest = false;
                res.size = 0;
                for i in 0..num {
                    let param = match self.get_param(i) {
                        Some(p) => p,
                        None => continue,
                    };
                    if !param.is_type_locked() {
                        continue;
                    }
                    locktest = true;
                    let iaddr = param.get_address();
                    if iaddr.contained_by(param.get_size(), loc, size)
                        && param.get_size() as u32 > res.size
                    {
                        res.space = iaddr.get_space().cloned();
                        res.offset = iaddr.get_offset();
                        res.size = param.get_size() as u32;
                    }
                }
                if locktest {
                    return res.size == 0;
                }
            }
        }
        self.model().get_biggest_contained_input_param(loc, size, res)
    }

    /// Pass back the biggest output storage contained in the range (C++
    /// `getBiggestContainedOutput`).
    pub fn get_biggest_contained_output(&self, loc: &Address, size: int4, res: &mut VarnodeData) -> bool {
        if self.is_output_locked() {
            let outparam = self.get_output();
            if outparam.get_type().map(|t| t.get_metatype()) == Some(type_metatype::TYPE_VOID) {
                return false;
            }
            let iaddr = outparam.get_address();
            if iaddr.contained_by(outparam.get_size(), loc, size) {
                res.space = iaddr.get_space().cloned();
                res.offset = iaddr.get_offset();
                res.size = outparam.get_size() as u32;
                return true;
            }
            return false;
        }
        self.model().get_biggest_contained_output(loc, size, res)
    }

    /// Get the storage location for the "this" pointer (C++
    /// `getThisPointerStorage`).
    pub fn get_this_pointer_storage(
        &self,
        dt: Rc<Datatype>,
        typefactory: &dyn TypeFactory,
        manager: &AddrSpaceManager,
    ) -> KunaResult<Address> {
        if !self.model().has_this_pointer() {
            return Ok(Address::new_invalid());
        }
        let mut proto = PrototypePieces { first_var_arg_slot: -1, ..Default::default() };
        proto.outtype = self.get_output_type().cloned();
        proto.intypes.push(dt);
        let mut res: Vec<ParameterPieces> = Vec::new();
        self.model()
            .assign_parameter_storage(&proto, &mut res, true, typefactory, manager)?;
        for piece in res.iter().skip(1) {
            if (piece.flags & parameter_pieces_flags::HIDDENRETPARM) != 0 {
                continue;
            }
            return Ok(piece.addr.clone());
        }
        Ok(Address::new_invalid())
    }

    /// Decide if this can be safely restricted to match another prototype (C++
    /// `isCompatible`).
    pub fn is_compatible(&self, op2: &FuncProto) -> bool {
        if !self.model().is_compatible(op2.model()) {
            return false;
        }
        if op2.is_output_locked() && self.is_output_locked() {
            let out1 = self.store().get_output();
            let out2 = op2.store().get_output();
            if !proto_parameter_eq(out1, out2) {
                return false;
            }
        }
        if self.extrapop != EXTRAPOP_UNKNOWN && self.extrapop != op2.extrapop {
            return false;
        }
        if self.is_dotdotdot() != op2.is_dotdotdot() {
            // Mismatch in varargs
            if op2.is_dotdotdot() {
                // If -this- is generic, trials are still set up to recover varargs
                if self.is_input_locked() {
                    return false;
                }
            } else {
                return false;
            }
        }
        if self.injectid != op2.injectid {
            return false;
        }
        if (self.flags & (func_proto_flags::IS_INLINE | func_proto_flags::NO_RETURN))
            != (op2.flags & (func_proto_flags::IS_INLINE | func_proto_flags::NO_RETURN))
        {
            return false;
        }
        if self.effectlist.len() != op2.effectlist.len() {
            return false;
        }
        for i in 0..self.effectlist.len() {
            if self.effectlist[i] != op2.effectlist[i] {
                return false;
            }
        }
        if self.likelytrash.len() != op2.likelytrash.len() {
            return false;
        }
        for i in 0..self.likelytrash.len() {
            if self.likelytrash[i] != op2.likelytrash[i] {
                return false;
            }
        }
        true
    }

    /// Is a potential output automatically considered killed-by-call (C++
    /// `isAutoKilledByCall`).
    pub fn is_auto_killed_by_call(&self) -> bool {
        if (self.flags & func_proto_flags::AUTO_KILLEDBYCALL) != 0 {
            return true; // The ProtoModel always does killedbycall
        }
        if self.is_output_locked() {
            return true; // A locked output location is killedbycall by definition
        }
        false
    }

    /// Get the stack address space (C++ `getSpacebase`).
    pub fn get_spacebase(&self) -> Option<&Rc<AddrSpace>> {
        self.model().get_spacebase()
    }
    /// Get the range of potential local stack variables (C++ `getLocalRange`).
    pub fn get_local_range(&self) -> &RangeList {
        self.model().get_local_range()
    }
    /// Get the range of potential stack parameters (C++ `getParamRange`).
    pub fn get_param_range(&self) -> &RangeList {
        self.model().get_param_range()
    }
    /// Return true if the stack grows toward smaller addresses (C++
    /// `isStackGrowsNegative`).
    pub fn is_stack_grows_negative(&self) -> bool {
        self.model().is_stack_grows_negative()
    }
    /// Maximum heritage delay across all input parameters (C++
    /// `getMaxInputDelay`).
    pub fn get_max_input_delay(&self) -> int4 {
        self.model().get_max_input_delay()
    }
    /// Maximum heritage delay across all return values (C++
    /// `getMaxOutputDelay`).
    pub fn get_max_output_delay(&self) -> int4 {
        self.model().get_max_output_delay()
    }
    /// Check if two input storage locations can represent a single logical
    /// parameter (C++ `checkInputJoin`).
    pub fn check_input_join(&self, hiaddr: &Address, hisz: int4, loaddr: &Address, losz: int4) -> bool {
        self.model().check_input_join(hiaddr, hisz, loaddr, losz)
    }
    /// Check if a single storage location can be split into two input
    /// parameters (C++ `checkInputSplit`).
    pub fn check_input_split(&self, loc: &Address, size: int4, splitpoint: int4) -> bool {
        self.model().check_input_split(loc, size, splitpoint)
    }
    /// Get any assumed input extension and container (C++
    /// `assumedInputExtension`).
    pub fn assumed_input_extension(&self, addr: &Address, size: int4, res: &mut VarnodeData) -> OpCode {
        self.model().assumed_input_extension(addr, size, res)
    }
    /// Get any assumed output extension and container (C++
    /// `assumedOutputExtension`).
    pub fn assumed_output_extension(&self, addr: &Address, size: int4, res: &mut VarnodeData) -> OpCode {
        self.model().assumed_output_extension(addr, size, res)
    }

    // -- Funcdata-dependent updates (fspec.cc:4057-4198) --------------------

    /// Update input parameters based on Varnode trials (C++ `updateInputTypes`).
    ///
    /// STUB(w6-fspec-2): reaches `Funcdata`/`Varnode` (W3) — the trial list is a
    /// `vector<Varnode *>` and the body reads each Varnode's high data-type.
    pub fn update_input_types(&mut self) -> KunaResult<()> {
        Err(KunaError::lowlevel(
            "STUB(w6-fspec-2) FuncProto::updateInputTypes: needs Funcdata/Varnode trials",
        ))
    }
    /// Update input parameters from trials without types (C++
    /// `updateInputNoTypes`).  STUB(w6-fspec-2): needs `Funcdata`/`Varnode`.
    pub fn update_input_no_types(&mut self) -> KunaResult<()> {
        Err(KunaError::lowlevel(
            "STUB(w6-fspec-2) FuncProto::updateInputNoTypes: needs Funcdata/Varnode trials",
        ))
    }
    /// Update the return value based on Varnode trials (C++
    /// `updateOutputTypes`).  STUB(w6-fspec-2): needs `Funcdata`/`Varnode`.
    pub fn update_output_types(&mut self) -> KunaResult<()> {
        Err(KunaError::lowlevel(
            "STUB(w6-fspec-2) FuncProto::updateOutputTypes: needs Funcdata/Varnode trials",
        ))
    }
    /// Update the return value from trials without types (C++
    /// `updateOutputNoTypes`).  STUB(w6-fspec-2): needs `Funcdata`/`Varnode`.
    pub fn update_output_no_types(&mut self) -> KunaResult<()> {
        Err(KunaError::lowlevel(
            "STUB(w6-fspec-2) FuncProto::updateOutputNoTypes: needs Funcdata/Varnode trials",
        ))
    }
    /// Restore this from a `<prototype>` element (C++ `decode`).
    /// STUB(w6-fspec-2): needs the marshaling Decoder + Architecture registry.
    pub fn decode(&mut self) -> KunaResult<()> {
        Err(KunaError::lowlevel(
            "STUB(w6-fspec-2) FuncProto::decode: needs marshaling + Architecture registry",
        ))
    }

    /// Encode this to a `<prototype>` element (C++ `FuncProto::encode`,
    /// fspec.cc:4625-4668): the model + extrapop + boolean flags, the
    /// `<returnsym>` (storage + type of the output parameter), the effect /
    /// likely-trash overrides, and the call-fixup `<inject>` when
    /// `inject_name` supplies the resolved fixup name (the C++ reads it
    /// through `glb->pcodeinjectlib`, which the kuna `FuncProto` cannot
    /// reach — the caller resolves it).
    ///
    /// The C++ tail calls `store->encode` — `ProtoStoreSymbol::encode`
    /// (fspec.cc:3293) writes NOTHING, and upstream's decompileAt path always
    /// has the symbol-backed store, so the wire shape omits `<internallist>`;
    /// kuna's internal store follows the wire shape (input parameters travel
    /// as `<localdb>` category-0 symbols, `LocalSymbolMap.decodeSymbolList`).
    pub fn encode(&self, encoder: &mut dyn kuna_base::marshal::Encoder) -> KunaResult<()> {
        use kuna_base::marshal::{
            ATTRIB_CONSTRUCTOR, ATTRIB_DESTRUCTOR, ATTRIB_MODEL, ATTRIB_TYPELOCK,
        };
        use crate::remote_provider::{
            ATTRIB_CUSTOM, ATTRIB_DOTDOTDOT, ATTRIB_EXTRAPOP, ATTRIB_INLINE, ATTRIB_MODELLOCK,
            ATTRIB_NORETURN, ATTRIB_VOIDLOCK, ELEM_PROTOTYPE, ELEM_RETURNSYM,
        };
        encoder.open_element(&ELEM_PROTOTYPE);
        // C++ `model->getName()` (a null model never encodes upstream); an
        // un-modeled kuna proto degrades to the "default" spelling, which the
        // Java side maps onto the program's default model.
        let model_name = self.model.as_ref().map(|m| m.get_name()).unwrap_or("default");
        encoder.write_string(&ATTRIB_MODEL, model_name.as_bytes());
        if self.extrapop == EXTRAPOP_UNKNOWN {
            encoder.write_string(&ATTRIB_EXTRAPOP, b"unknown");
        } else {
            encoder.write_signed_integer(&ATTRIB_EXTRAPOP, self.extrapop as i64);
        }
        if self.is_dotdotdot() {
            encoder.write_bool(&ATTRIB_DOTDOTDOT, true);
        }
        if self.is_model_locked() {
            encoder.write_bool(&ATTRIB_MODELLOCK, true);
        }
        if (self.flags & func_proto_flags::VOIDINPUTLOCK) != 0 {
            encoder.write_bool(&ATTRIB_VOIDLOCK, true);
        }
        if self.is_inline() {
            encoder.write_bool(&ATTRIB_INLINE, true);
        }
        if self.is_no_return() {
            encoder.write_bool(&ATTRIB_NORETURN, true);
        }
        if self.has_custom_storage() {
            encoder.write_bool(&ATTRIB_CUSTOM, true);
        }
        if self.is_constructor() {
            encoder.write_bool(&ATTRIB_CONSTRUCTOR, true);
        }
        if self.is_destructor() {
            encoder.write_bool(&ATTRIB_DESTRUCTOR, true);
        }
        // <returnsym>: outparam storage (sized <addr>) + type.  Java requires
        // the pair (FunctionPrototype.decodePrototype).
        let store = self
            .store
            .as_ref()
            .ok_or_else(|| KunaError::lowlevel("FuncProto::encode: no parameter store"))?;
        let outparam = store.get_output();
        encoder.open_element(&ELEM_RETURNSYM);
        if outparam.is_type_locked() {
            encoder.write_bool(&ATTRIB_TYPELOCK, true);
        }
        outparam.get_address().encode_sized(encoder, outparam.get_size())?;
        match outparam.get_type() {
            Some(t) => t.encode_ref(encoder)?,
            None => {
                // A type-less output is the void state (C++ outparam type is
                // never null; ParameterBasic seeds TYPE_VOID).
                encoder.open_element(&kuna_base::marshal::ELEM_VOID);
                encoder.close_element(&kuna_base::marshal::ELEM_VOID);
            }
        }
        encoder.close_element(&ELEM_RETURNSYM);
        self.encode_effect(encoder)?;
        self.encode_likely_trash(encoder)?;
        // <inject>: only when the caller resolved the fixup name (injectid>=0).
        // Without the resolver the id is silently dropped, matching a fixup-less
        // proto (Java skips the element anyway).
        encoder.close_element(&ELEM_PROTOTYPE);
        Ok(())
    }

    /// Encode the effect-record overrides (C++ `FuncProto::encodeEffect`,
    /// fspec.cc:3589-3627): only records differing from the underlying
    /// `ProtoModel`, grouped as `<unaffected>` / `<killedbycall>` /
    /// `<returnaddress>`.
    fn encode_effect(&self, encoder: &mut dyn kuna_base::marshal::Encoder) -> KunaResult<()> {
        if self.effectlist.is_empty() {
            return Ok(());
        }
        let mut unaffected: Vec<&EffectRecord> = Vec::new();
        let mut killed: Vec<&EffectRecord> = Vec::new();
        let mut ret_addr: Option<&EffectRecord> = None;
        for cur in &self.effectlist {
            let tp = match &self.model {
                Some(m) => m.has_effect(&cur.get_address(), cur.get_size()),
                None => effect_type::UNKNOWN_EFFECT,
            };
            if tp == cur.get_type() {
                continue;
            }
            if cur.get_type() == effect_type::UNAFFECTED {
                unaffected.push(cur);
            } else if cur.get_type() == effect_type::KILLEDBYCALL {
                killed.push(cur);
            } else if cur.get_type() == effect_type::RETURN_ADDRESS {
                ret_addr = Some(cur);
            }
        }
        if !unaffected.is_empty() {
            encoder.open_element(&ELEM_UNAFFECTED);
            for r in unaffected {
                r.encode(encoder)?;
            }
            encoder.close_element(&ELEM_UNAFFECTED);
        }
        if !killed.is_empty() {
            encoder.open_element(&ELEM_KILLEDBYCALL);
            for r in killed {
                r.encode(encoder)?;
            }
            encoder.close_element(&ELEM_KILLEDBYCALL);
        }
        if let Some(r) = ret_addr {
            encoder.open_element(&ELEM_RETURNADDRESS);
            r.encode(encoder)?;
            encoder.close_element(&ELEM_RETURNADDRESS);
        }
        Ok(())
    }

    /// Encode the likely-trash overrides (C++ `FuncProto::encodeLikelyTrash`,
    /// fspec.cc:3631-3648): the entries not already in the `ProtoModel`'s list.
    fn encode_likely_trash(&self, encoder: &mut dyn kuna_base::marshal::Encoder) -> KunaResult<()> {
        if self.likelytrash.is_empty() {
            return Ok(());
        }
        let model_trash: &[VarnodeData] =
            self.model.as_ref().map(|m| m.trash_list()).unwrap_or(&[]);
        encoder.open_element(&ELEM_LIKELYTRASH);
        for cur in &self.likelytrash {
            if model_trash.binary_search_by(|p| p.cmp(cur)).is_ok() {
                continue; // Already exists in ProtoModel
            }
            encoder.open_element(&kuna_base::address::ELEM_ADDR);
            if let Some(spc) = &cur.space {
                spc.encode_attributes_sized(encoder, cur.offset, cur.size as int4)?;
            }
            encoder.close_element(&kuna_base::address::ELEM_ADDR);
        }
        encoder.close_element(&ELEM_LIKELYTRASH);
        Ok(())
    }

    // -- test/tooling builder hooks -----------------------------------------

    /// Replace the parameter store (test/builder hook; the C++ swaps `store`
    /// in `setScope`/`setInternal`/`paramShift`).
    pub fn set_store(&mut self, store: Box<dyn ProtoStore>) {
        self.store = Some(store);
    }
}

// =============================================================================
// FuncCallSpecs (fspec.hh:1645-1742, fspec.cc:4854-end) — item w6-s4-fspec-3
// =============================================================================
//
// `FuncCallSpecs` is a prototype that evolves over analysis: it derives off
// `FuncProto` (modeled here by **composition** over a [`FuncProto`] field —
// Rust has no inheritance, exactly as `ProtoModelMerged` carries its base by
// composition) and adds the call-site data-flow state used to recover a working
// prototype for one CALL/CALLIND.
//
// ## Cross-wave boundaries
//
// The C++ method bodies that *mutate* the calling `Funcdata`'s IR
// (`commitNewInputs`/`commitNewOutputs`, the success path of `deindirect`/
// `forceSet`, `createPlaceholder`, the `buildParam` truncation builds) reach a
// large surface of W4 `Funcdata` helpers that are **not yet on the W3
// `Funcdata`** (`opStackLoad`, `getOverride`, `warningHeader`, `opSetAllInput`,
// `newVarnodeOut`, `newIndirectCreation`, `opMarkCalculatedBool`, the W6 `glb->
// inst[opc]` `TypeOp` resolution for `opSetOpcode`).  Those paths are marked
// `// STUB(w6-fspec-3 W4)` and return `Err(KunaError::lowlevel(...))`, matching
// the established `fspec-2` convention (`updateInputTypes`, etc.).
//
// The **pure** state machine (the trial lifecycle, `checkInputJoin`/
// `doInputJoin` over [`ParamActive`], `lateRestriction`'s compatibility gate,
// and the **restart-recorder** paths of `deindirect`/`forceSet`) is ported
// faithfully and exercised directly in tests — `resolveSpacebaseRelative` and
// `getSpacebaseRelative` over a hand-built call site, and the restart paths via
// a [`RestartLog`].

/// "Magic" stack offset indicating the offset is unknown (C++
/// `FuncCallSpecs::offset_unknown`, `fspec.hh:1677`).
pub const OFFSET_UNKNOWN: uintb = 0xBADBEEF;

use crate::funcdata::Funcdata;
use crate::kuna_restartlog::{KunaRestartReason, RestartLog};
use crate::context::{OpId, VarnodeId};

/// A class for analyzing parameters to a sub-function call (C++
/// `FuncCallSpecs`, `fspec.hh:1645`).
///
/// Derives off [`FuncProto`] by composition (the `proto` field); the
/// `FuncProto` API is reached through [`FuncCallSpecs::proto`] /
/// [`FuncCallSpecs::proto_mut`] (and the thin forwarders this impl provides for
/// the methods the call-site logic uses).
#[derive(Debug)]
pub struct FuncCallSpecs {
    /// The `FuncProto` base (C++ inheritance `: public FuncProto`).
    proto: FuncProto,
    /// The CALL or CALLIND op in the calling function (C++ `op`,
    /// a `PcodeOp *`).  Modeled as the W3 [`OpId`].
    op: OpId,
    /// Name of the called function if present (C++ `name`).
    name: String,
    /// First executing address of the called function (C++ `entryaddress`).
    entryaddress: Address,
    /// True if `fd` (the callee `Funcdata`) is known (C++ `fd != 0`).  The
    /// callee `Funcdata` is a cross-function W4 reference; only its presence is
    /// tracked here.
    has_fd: bool,
    /// Working extrapop for the CALL (C++ `effective_extrapop`).
    effective_extrapop: int4,
    /// Relative offset of the stack-pointer at the time of this call (C++
    /// `stackoffset`).
    stackoffset: uintb,
    /// Slot containing the temporary stack-tracing placeholder, or -1 if unused
    /// (C++ `stackPlaceholderSlot`).
    stack_placeholder_slot: int4,
    /// Number of input parameters to ignore before the prototype (C++
    /// `paramshift`).
    paramshift: int4,
    /// Number of calls to this sub-function within the calling function (C++
    /// `matchCallCount`).
    match_call_count: int4,
    /// Info for recovering input parameters (C++ `activeinput`).
    activeinput: ParamActive,
    /// Info for recovering output parameters (C++ `activeoutput`).
    activeoutput: ParamActive,
    /// Number of bytes consumed by the sub-function, per input parameter (C++
    /// `inputConsume`).
    input_consume: Vec<int4>,
    /// Are we actively trying to recover input parameters (C++ `isinputactive`).
    isinputactive: bool,
    /// Are we actively trying to recover output parameters (C++
    /// `isoutputactive`).
    isoutputactive: bool,
    /// Was the call originally a jump-table we couldn't recover (C++
    /// `isbadjumptable`).
    isbadjumptable: bool,
    /// Do we have a locked output on the stack (C++ `isstackoutputlock`).
    isstackoutputlock: bool,
    /// (kuna) `calleearity`: the storage each recovered argument occupies, in
    /// prototype order, recorded when `build_input_from_trials` finalizes the
    /// list.  The CALL op's inputs carry the argument *values* (a constant, a
    /// temporary), never the location, and the trials that knew the location are
    /// cleared right after — so a later call to the same callee has no other way
    /// to ask what its sibling recovered.  See
    /// [`crate::p4_calls::kuna_calleearity`].
    final_input_storage: Vec<(Address, int4)>,
}

impl FuncCallSpecs {
    /// Construct based on a CALL or CALLIND op (C++
    /// `FuncCallSpecs::FuncCallSpecs(PcodeOp *)`, `fspec.cc:4929`).
    ///
    /// The C++ peeks at `call_op->getIn(0)` for a direct CALL to grab the entry
    /// address; if that input is already an \e fspec annotation (a cloned op for
    /// inlining) it chases through to the underlying call-spec's entry address.
    /// That chase needs the call-spec registry (W4 op-clone path) and is
    /// supplied by the caller via `entry`: pass the callee entry address for a
    /// direct CALL (or an invalid `Address` for a CALLIND).
    pub fn new(call_op: OpId, entry: Address) -> FuncCallSpecs {
        FuncCallSpecs {
            proto: FuncProto::new(),
            op: call_op,
            name: String::new(),
            entryaddress: entry,
            has_fd: false,
            effective_extrapop: EXTRAPOP_UNKNOWN, // ProtoModel::extrapop_unknown
            stackoffset: OFFSET_UNKNOWN,
            stack_placeholder_slot: -1,
            paramshift: 0,
            match_call_count: 0,
            activeinput: ParamActive::new(true),
            activeoutput: ParamActive::new(true),
            input_consume: Vec::new(),
            isinputactive: false,
            isoutputactive: false,
            isbadjumptable: false,
            isstackoutputlock: false,
            final_input_storage: Vec::new(), // (kuna) calleearity
        }
    }

    /// Clone this call spec onto a new CALL/CALLIND op (C++
    /// `FuncCallSpecs::clone(PcodeOp *newop)`, `fspec.cc:4969`).
    ///
    /// Used by `Funcdata::truncatedFlow` to re-attach the discovered call specs
    /// to the matching ops of a partial (jump-table recovery) clone.  Copies the
    /// `FuncProto` portion (incl. the effect list that drives the call's INDIRECT
    /// markers, which keep a post-call stack slot symbolic during recovery),
    /// `effective_extrapop`/`stackoffset`/`paramshift`/`isbadjumptable`, and the
    /// entry/name/funcdata identity.  `activeinput`/`activeoutput` are skipped,
    /// exactly as the C++ does.
    pub fn clone_for_op(&self, newop: OpId) -> FuncCallSpecs {
        let mut res = FuncCallSpecs::new(newop, self.entryaddress.clone());
        // setFuncdata(fd): sets op (already), name, address, fd.
        if self.has_fd {
            // Mirror setFuncdata(fd) — entry/name already on res; mark fd present.
            res.has_fd = true;
            res.name = self.name.clone();
        } else {
            res.name = self.name.clone();
        }
        res.effective_extrapop = self.effective_extrapop;
        res.stackoffset = self.stackoffset;
        res.paramshift = self.paramshift;
        // We are skipping activeinput, activeoutput (per C++).
        res.isbadjumptable = self.isbadjumptable;
        res.proto.copy(&self.proto); // Copy the FuncProto portion
        res
    }

    // -- FuncProto base access ----------------------------------------------

    /// The `FuncProto` base (C++ upcast to `FuncProto`).
    pub fn proto(&self) -> &FuncProto {
        &self.proto
    }
    /// The `FuncProto` base, mutably.
    pub fn proto_mut(&mut self) -> &mut FuncProto {
        &mut self.proto
    }

    // -- simple call-site accessors (fspec.hh:1680-1706) --------------------

    /// Set (override) the callee's entry address (C++ `setAddress`).
    pub fn set_address(&mut self, addr: Address) {
        self.entryaddress = addr;
    }
    /// Get the CALL or CALLIND corresponding to this (C++ `getOp`).
    pub fn get_op(&self) -> OpId {
        self.op
    }
    /// Is the callee `Funcdata` known (C++ `getFuncdata() != 0`).
    pub fn has_funcdata(&self) -> bool {
        self.has_fd
    }
    /// Record (the presence of) the callee `Funcdata` (C++ `setFuncdata`).
    ///
    /// The C++ pulls the entry address and display name off `fd`; the callee
    /// `Funcdata` is a W4 cross-function reference, so the caller passes the
    /// already-extracted `entry`/`name` (mirroring `fd->getAddress()` /
    /// `fd->getDisplayName()`).  Errs if a callee was already set (C++
    /// `throw LowlevelError("Setting call spec function multiple times")`).
    pub fn set_funcdata(&mut self, entry: Address, name: &str) -> KunaResult<()> {
        if self.has_fd {
            return Err(KunaError::lowlevel("Setting call spec function multiple times"));
        }
        self.has_fd = true;
        self.entryaddress = entry;
        if !name.is_empty() {
            self.name = name.to_string();
        }
        Ok(())
    }
    /// Get the function name associated with the callee (C++ `getName`).
    pub fn get_name(&self) -> &str {
        &self.name
    }
    /// Get the entry address of the callee (C++ `getEntryAddress`).
    pub fn get_entry_address(&self) -> &Address {
        &self.entryaddress
    }
    /// Set the specific extrapop for this call site (C++ `setEffectiveExtraPop`).
    pub fn set_effective_extra_pop(&mut self, epop: int4) {
        self.effective_extrapop = epop;
    }
    /// Get the specific extrapop for this call site (C++ `getEffectiveExtraPop`).
    pub fn get_effective_extra_pop(&self) -> int4 {
        self.effective_extrapop
    }
    /// Get the stack-pointer relative offset at this call site (C++
    /// `getSpacebaseOffset`).
    pub fn get_spacebase_offset(&self) -> uintb {
        self.stackoffset
    }
    /// Determine the side-effect of \b this call on a memory range, first
    /// translating a stack address into the callee's spacebase point of view
    /// (C++ `FuncCallSpecs::hasEffectTranslate`, `fspec.cc:5941`).
    ///
    /// For a non-spacebase address this is just `FuncProto::hasEffect`.  For a
    /// spacebase (stack) address: if the call's stack offset is unknown the
    /// effect is `unknown_effect`; otherwise the offset is shifted by
    /// `-stackoffset` (wrapped in the space) to land in the callee's frame, then
    /// `hasEffect` is consulted.
    pub fn has_effect_translate(&self, addr: &Address, size: int4) -> uint4 {
        let spc = addr.get_space().expect("has_effect_translate: addr space");
        if spc.get_type() != kuna_base::space::spacetype::IPTR_SPACEBASE {
            return self.proto.has_effect(addr, size);
        }
        if self.stackoffset == OFFSET_UNKNOWN {
            return effect_type::UNKNOWN_EFFECT;
        }
        // Translate to callee's spacebase point of view.
        let newoff = spc.wrap_offset(addr.get_offset().wrapping_sub(self.stackoffset));
        let translated = Address::new(Rc::clone(spc), newoff);
        self.proto.has_effect(&translated, size)
    }
    /// Set a parameter shift for this call site (C++ `setParamshift`).
    pub fn set_paramshift(&mut self, val: int4) {
        self.paramshift = val;
    }
    /// Get the parameter shift for this call site (C++ `getParamshift`).
    pub fn get_paramshift(&self) -> int4 {
        self.paramshift
    }
    /// Get the number of calls the caller makes to this sub-function (C++
    /// `getMatchCallCount`).
    pub fn get_match_call_count(&self) -> int4 {
        self.match_call_count
    }
    /// Get the slot of the stack-pointer placeholder (C++
    /// `getStackPlaceholderSlot`).
    pub fn get_stack_placeholder_slot(&self) -> int4 {
        self.stack_placeholder_slot
    }

    /// Set the slot of the stack-pointer placeholder (C++ inline
    /// `setStackPlaceholderSlot`, `fspec.hh:1671`).
    ///
    /// Its only C++ callers — `createPlaceholder` and `commitNewInputs` — reach
    /// W4 `Funcdata` factories (`opStackLoad`/`opSetAllInput`) that are not yet
    /// on the W3 `Funcdata`, so both are stubbed (`// STUB(w6-fspec-3 W4)`); the
    /// bookkeeping itself is transcribed and lands here.
    #[allow(dead_code)] // exercised once createPlaceholder/commitNewInputs de-stub (W4)
    fn set_stack_placeholder_slot(&mut self, slot: int4) {
        self.stack_placeholder_slot = slot;
        if self.isinputactive {
            self.activeinput.set_placeholder_slot();
        }
    }
    /// Release the stack-pointer placeholder (C++ inline
    /// `clearStackPlaceholderSlot`, `fspec.hh:1673`).
    fn clear_stack_placeholder_slot(&mut self) {
        self.stack_placeholder_slot = -1;
        if self.isinputactive {
            self.activeinput.free_placeholder_slot();
        }
    }

    // -- input/output recovery activation (fspec.hh:1695-1706) --------------

    /// Turn on analysis recovering input parameters (C++ `initActiveInput`,
    /// `fspec.cc:5336`).
    pub fn init_active_input(&mut self) {
        self.isinputactive = true;
        let mut maxdelay = self.proto.get_max_input_delay();
        if maxdelay > 0 {
            maxdelay = 3;
        }
        self.activeinput.set_max_pass(maxdelay);
    }
    /// Turn off analysis recovering input parameters (C++ `clearActiveInput`).
    pub fn clear_active_input(&mut self) {
        self.isinputactive = false;
    }
    /// (kuna) `calleearity`: the storage of each argument this call finally
    /// recovered, in prototype order (empty until it is finalized).
    pub fn final_input_storage(&self) -> &[(Address, int4)] {
        &self.final_input_storage
    }
    /// (kuna) `calleearity`: record the finalized argument storage.
    pub fn set_final_input_storage(&mut self, storage: Vec<(Address, int4)>) {
        self.final_input_storage = storage;
    }
    /// Turn on analysis recovering the return value (C++ `initActiveOutput`).
    pub fn init_active_output(&mut self) {
        self.isoutputactive = true;
    }
    /// Turn off analysis recovering the return value (C++ `clearActiveOutput`).
    pub fn clear_active_output(&mut self) {
        self.isoutputactive = false;
    }
    /// True if input parameter recovery analysis is active (C++ `isInputActive`).
    pub fn is_input_active(&self) -> bool {
        self.isinputactive
    }
    /// True if return value recovery analysis is active (C++ `isOutputActive`).
    pub fn is_output_active(&self) -> bool {
        self.isoutputactive
    }
    /// Toggle whether the call site looked like an indirect jump (C++
    /// `setBadJumpTable`).
    pub fn set_bad_jump_table(&mut self, val: bool) {
        self.isbadjumptable = val;
    }
    /// True if this call site looked like an indirect jump (C++
    /// `isBadJumpTable`).
    pub fn is_bad_jump_table(&self) -> bool {
        self.isbadjumptable
    }
    /// Toggle whether output is locked and on the stack (C++ `setStackOutputLock`).
    pub fn set_stack_output_lock(&mut self, val: bool) {
        self.isstackoutputlock = val;
    }
    /// True if the return value is locked and on the stack (C++ `isStackOutputLock`).
    pub fn is_stack_output_lock(&self) -> bool {
        self.isstackoutputlock
    }
    /// Resolve a merged prototype model against the recovered input trials (C++
    /// `FuncProto::resolveModel` via the `FuncCallSpecs` base, called by
    /// `ActionActiveParam`).  Delegates to the `FuncProto` base.
    pub fn resolve_model(&mut self, active: &ParamActive) -> KunaResult<()> {
        self.proto.resolve_model(active)
    }

    /// Run the C++ `ActionActiveParam` model/input-map resolution against \b this
    /// call spec's own active input (`fc->resolveModel(activeinput)` then
    /// `fc->deriveInputMap(activeinput)`).
    ///
    /// The two operate on the same `activeinput` member, which the borrow checker
    /// will not let `resolve_model(&self.activeinput)`/`derive_input_map(&mut
    /// self.activeinput)` borrow simultaneously — so the disjoint
    /// `proto`/`activeinput` field split is done here, in one place.
    pub fn resolve_and_derive_input_map(&mut self, manager: &AddrSpaceManager) -> KunaResult<()> {
        // resolveModel reads activeinput, writes proto.model.
        self.proto.resolve_model(&self.activeinput)?;
        // deriveInputMap reads proto.model, writes activeinput trials.
        self.proto.derive_input_map(&mut self.activeinput, manager)
    }

    /// Derive the most likely input prototype from the trials (C++
    /// `FuncCallSpecs::deriveInputMap` -> `FuncProto::deriveInputMap`).
    pub fn derive_input_map(
        &self,
        active: &mut ParamActive,
        manager: &AddrSpaceManager,
    ) -> KunaResult<()> {
        self.proto.derive_input_map(active, manager)
    }

    /// Derive the most likely output prototype from the trials (C++
    /// `FuncCallSpecs::deriveOutputMap` -> `FuncProto::deriveOutputMap`).
    pub fn derive_output_map(
        &self,
        active: &mut ParamActive,
        manager: &AddrSpaceManager,
    ) -> KunaResult<()> {
        self.proto.derive_output_map(active, manager)
    }

    /// Derive the output map against \b this call spec's own active output (C++
    /// `fc->deriveOutputMap(activeoutput)` in `ActionActiveReturn`).  The
    /// proto/activeoutput field split is done here (the borrow checker rejects
    /// `derive_output_map(&mut self.activeoutput)` with `&self.proto`).
    pub fn derive_output_map_self(&mut self, manager: &AddrSpaceManager) -> KunaResult<()> {
        self.proto.derive_output_map(&mut self.activeoutput, manager)
    }

    /// The analysis object for input parameter recovery (C++ `getActiveInput`).
    pub fn get_active_input(&mut self) -> &mut ParamActive {
        &mut self.activeinput
    }
    /// Immutable view of the input-parameter recovery analysis object.
    ///
    /// C++ has only the non-const `getActiveInput()`, but `checkCallDoubleUse`
    /// is a `const` Funcdata method reading another call's trials
    /// (`fc->getActiveInput()->getTrialForInputVarnode(j)`); the Rust port keeps
    /// that read-only path on a shared borrow so the cross-call lookup needs no
    /// `&mut`.
    pub fn active_input(&self) -> &ParamActive {
        &self.activeinput
    }
    /// The analysis object for return value recovery (C++ `getActiveOutput`).
    pub fn get_active_output(&mut self) -> &mut ParamActive {
        &mut self.activeoutput
    }

    /// The relative offset of the stack pointer at this call site (C++
    /// `stackoffset`; the `buildInputFromTrials` spacebase-parameter translation
    /// reads it directly).
    pub fn get_stackoffset(&self) -> uintb {
        self.stackoffset
    }

    /// Is this prototype using varargs (C++ `isDotdotdot` via the `FuncProto`
    /// base).
    pub fn is_dotdotdot(&self) -> bool {
        self.proto.is_dotdotdot()
    }

    /// Is this prototype's input list locked (C++ `isInputLocked` via the
    /// `FuncProto` base).
    pub fn is_input_locked(&self) -> bool {
        self.proto.is_input_locked()
    }

    /// The model's declared extrapop (C++ `getModelExtraPop` via the `FuncProto`
    /// base; the `checkInputTrialUse` callee-pop test reads it).
    pub fn get_model_extra_pop(&self) -> int4 {
        self.proto.get_model_extra_pop()
    }

    /// The working extrapop for this prototype (C++ `getExtraPop` via the
    /// `FuncProto` base).
    pub fn get_extra_pop(&self) -> int4 {
        self.proto.get_extra_pop()
    }

    /// Is a potential output automatically killed-by-call (C++
    /// `isAutoKilledByCall` via the `FuncProto` base; the `guardCalls` output-trial
    /// effect-type test reads it).
    pub fn is_auto_killed_by_call(&self) -> bool {
        self.proto.is_auto_killed_by_call()
    }

    // -- input-bytes-consumed hints (fspec.cc:5877-5906) --------------------

    /// Number of bytes within the given parameter consumed by the sub-function
    /// (C++ `FuncCallSpecs::getInputBytesConsumed`, `fspec.cc:5877`).
    pub fn get_input_bytes_consumed(&self, slot: int4) -> int4 {
        if slot >= self.input_consume.len() as i32 {
            return 0;
        }
        self.input_consume[slot as usize]
    }

    /// Set the estimated number of bytes within the given parameter consumed by
    /// the sub-function (C++ `FuncCallSpecs::setInputBytesConsumed`,
    /// `fspec.cc:5894`).  Only lets the value get smaller; returns `true` on a
    /// change.  (The C++ marks the method `const` and mutates a `mutable`
    /// field; here the method is `&mut self`.)
    pub fn set_input_bytes_consumed(&mut self, slot: int4, val: int4) -> bool {
        while (self.input_consume.len() as i32) <= slot {
            self.input_consume.push(0);
        }
        let old_val = self.input_consume[slot as usize];
        if old_val == 0 || val < old_val {
            // Only let the value get smaller
            self.input_consume[slot as usize] = val;
            return true;
        }
        false
    }

    // -- input join (fspec.cc:5354-5400) ------------------------------------

    /// Check if adjacent parameter trials can be combined into a single logical
    /// parameter (C++ `FuncCallSpecs::checkInputJoin`, `fspec.cc:5354`).
    ///
    /// `vn1_size`/`vn2_size` are the sizes of the Varnodes corresponding to the
    /// first and second trial (C++ `vn1->getSize()` / `vn2->getSize()`).
    pub fn check_input_join(
        &self,
        slot1: int4,
        ishislot: bool,
        vn1_size: int4,
        vn2_size: int4,
    ) -> bool {
        if self.is_input_active() {
            return false;
        }
        if slot1 >= self.activeinput.get_num_trials() {
            return false; // Not enough params
        }
        let hislot: &ParamTrial;
        let loslot: &ParamTrial;
        if ishislot {
            // slot1 looks like the high slot
            hislot = self.activeinput.get_trial_for_input_varnode(slot1);
            loslot = self.activeinput.get_trial_for_input_varnode(slot1 + 1);
            if hislot.get_size() != vn1_size {
                return false;
            }
            if loslot.get_size() != vn2_size {
                return false;
            }
        } else {
            loslot = self.activeinput.get_trial_for_input_varnode(slot1);
            hislot = self.activeinput.get_trial_for_input_varnode(slot1 + 1);
            if loslot.get_size() != vn1_size {
                return false;
            }
            if hislot.get_size() != vn2_size {
                return false;
            }
        }
        self.proto.check_input_join(
            hislot.get_address(),
            hislot.get_size(),
            loslot.get_address(),
            loslot.get_size(),
        )
    }

    /// Join two adjacent parameter trials (C++ `FuncCallSpecs::doInputJoin`,
    /// `fspec.cc:5381`).
    ///
    /// Assumes `check_input_join` returned `true`.  The C++ reaches
    /// `glb->constructJoinAddress(glb->translate, ...)`; the kuna `ProtoModel`
    /// does not hold the owning `Architecture`, so the caller supplies the same
    /// two values — the [`AddrSpaceManager`] (`glb`) and the register lookup
    /// (`glb->translate`) — explicitly.  Errs on a locked prototype (C++
    /// `throw LowlevelError`).
    pub fn do_input_join(
        &mut self,
        slot1: int4,
        ishislot: bool,
        manager: &AddrSpaceManager,
        translate: &dyn kuna_base::space::RegisterLookup,
    ) -> KunaResult<()> {
        if self.proto.is_input_locked() {
            return Err(KunaError::lowlevel(
                "Trying to join parameters on locked function prototype",
            ));
        }

        let trial1 = self.activeinput.get_trial_for_input_varnode(slot1).clone();
        let trial2 = self.activeinput.get_trial_for_input_varnode(slot1 + 1).clone();

        let addr1 = trial1.get_address().clone();
        let addr2 = trial2.get_address().clone();
        let joinaddr = if ishislot {
            manager.construct_join_address(
                translate,
                &addr1,
                trial1.get_size(),
                &addr2,
                trial2.get_size(),
            )?
        } else {
            manager.construct_join_address(
                translate,
                &addr2,
                trial2.get_size(),
                &addr1,
                trial1.get_size(),
            )?
        };

        self.activeinput.join_trial(slot1, &joinaddr, trial1.get_size() + trial2.get_size())
    }

    // -- prototype restriction / de-indirection (fspec.cc:5413-5516) --------

    /// Update this prototype to match a more specialized (locked) prototype
    /// (C++ `FuncCallSpecs::lateRestriction`, `fspec.cc:5413`).
    ///
    /// On success `this` is converted to `restricted_proto` and the new input /
    /// output Varnode lists are passed back.  When `restricted_proto` is
    /// input/output locked the transfer of the existing Varnodes reaches the W3
    /// CALL operands; that transfer (`transferLockedInput`/`Output`) is the
    /// `// STUB(w6-fspec-3 W4)` path below.  The unlocked compatibility gate
    /// (`hasModel`/`isCompatible`/dotdotdot) is ported in full.
    pub fn late_restriction(
        &mut self,
        data: &Funcdata,
        restricted_proto: &FuncProto,
        newinput: &mut Vec<Option<VarnodeId>>,
        newoutput: &mut Vec<VarnodeId>,
    ) -> KunaResult<bool> {
        if !self.proto.has_model() {
            self.proto.copy(restricted_proto);
            return Ok(true);
        }

        if !self.proto.is_compatible(restricted_proto) {
            return Ok(false);
        }
        if restricted_proto.is_dotdotdot() && !self.isinputactive {
            return Ok(false);
        }

        if restricted_proto.is_input_locked() {
            // Redo all the varnode inputs (if possible)
            if !self.transfer_locked_input(data, newinput, restricted_proto)? {
                return Ok(false);
            }
        }
        if restricted_proto.is_output_locked() {
            // Redo all the varnode outputs (if possible)
            if !self.transfer_locked_output(data, newoutput, restricted_proto)? {
                return Ok(false);
            }
        }
        self.proto.copy(restricted_proto); // Convert ourselves to restrictedProto

        Ok(true)
    }

    /// Convert this call site from an indirect to a direct function call (C++
    /// `FuncCallSpecs::deindirect`, `fspec.cc:5448`).
    ///
    /// `newfd_entry`/`newfd_name`/`newproto` mirror the callee `Funcdata`'s
    /// `getAddress()`/`getDisplayName()`/`getFuncProto()` (a cross-function W4
    /// reference).  The state mutation on `data` (the CALL op becomes a direct
    /// `CPUI_CALL` annotated with the call-spec handle, the override store
    /// gets an indirect override) reaches W4 `Funcdata` surfaces and is the
    /// `// STUB(w6-fspec-3 W4)` path.  The **decision** — whether
    /// `lateRestriction` succeeds, and the restart-recorder call when it does
    /// not — is ported in full and observable through `restartlog`.
    #[allow(clippy::too_many_arguments)]
    pub fn deindirect(
        &mut self,
        data: &mut Funcdata,
        newfd_entry: Address,
        newfd_name: &str,
        newproto: &FuncProto,
        newproto_no_return: bool,
        newproto_inline: bool,
        restartlog: &mut RestartLog,
    ) -> KunaResult<()> {
        self.entryaddress = newfd_entry.clone();
        self.name = newfd_name.to_string();
        self.has_fd = true;

        // Convert the CALLIND into a direct CALL
        // carrying this call spec's fspec annotation (the indirect target Varnode in
        // slot 0 is replaced).  The handle is the process-unique fspec offset; the
        // printed name + entry are registered in the fspec-space side table (the C++
        // raw-pointer-cast equivalent), exactly as flow.rs `build_call_specs` does
        // for a direct call.
        let handle = crate::flow::next_fspec_handle();
        let style = data.get_arch().kuna_name_style();
        self.register_in_fspec_space(handle, style);
        let fspecvn = data.new_varnode_call_specs(handle);
        data.op_set_input(self.op, fspecvn, 0)?;
        data.op_set_opcode_code(self.op, OpCode::CPUI_CALL);

        // data.getOverride().insertIndirectOverride(op->getAddr(), entryaddress);
        // Record the indirect->direct redirect so that, on the restart this method
        // schedules, `FlowInfo::setupCallindSpecs` (`applyIndirect`) rebuilds the
        // CALLIND straight as a direct CALL to the resolved target.
        let site = self.op_addr(data);
        data.get_override_mut().insert_indirect_override(site, newfd_entry);

        // Try our best to merge existing prototype with the one just handed.
        let mut newinput: Vec<Option<VarnodeId>> = Vec::new();
        let mut newoutput: Vec<VarnodeId> = Vec::new();
        if !newproto_no_return && !newproto_inline {
            if self.proto.is_override() {
                // If we are overridden at the call-site, don't use the
                // discovered function prototype.
                return Ok(());
            }

            if self.late_restriction(data, newproto, &mut newinput, &mut newoutput)? {
                // We have successfully updated the prototype: commit the new I/O.
                self.commit_new_inputs(data, &mut newinput)?;
                self.commit_new_outputs(data, &mut newoutput)?;
                return Ok(()); // don't restart
            }
        }
        data.set_restart_pending(true);
        // (kuna) restart observability
        let site = self.op_addr(data);
        restartlog.record_at(data, KunaRestartReason::ProtoDeindirect, &site);
        Ok(())
    }

    /// Force a more restrictive prototype on this call site (C++
    /// `FuncCallSpecs::forceSet`, `fspec.cc:5491`).
    ///
    /// The C++ records the recovered prototype into the override manager
    /// (`insertProtoOverride`), tries `lateRestriction`, commits or schedules a
    /// restart, then locks the input.  The override insertion and the
    /// success-commit are W4 `Funcdata` surfaces (`// STUB(w6-fspec-3 W4)`); the
    /// restart-recorder branch and the input-lock bookkeeping are ported in full.
    pub fn force_set(
        &mut self,
        data: &mut Funcdata,
        fp: &FuncProto,
        restartlog: &mut RestartLog,
    ) -> KunaResult<()> {
        let mut newinput: Vec<Option<VarnodeId>> = Vec::new();
        let mut newoutput: Vec<VarnodeId> = Vec::new();

        // data.getOverride().insertProtoOverride(op->getAddr(), copy(fp));
        // STUB(w6-fspec-3 W4): the override store is a W4 Funcdata surface.

        if self.late_restriction(data, fp, &mut newinput, &mut newoutput)? {
            // commitNewInputs/commitNewOutputs — STUB(w6-fspec-3 W4)
        } else {
            // Too late to make restrictions to correct prototype: force a restart.
            data.set_restart_pending(true);
            // (kuna) restart observability
            let site = self.op_addr(data);
            restartlog.record_at(data, KunaRestartReason::ProtoForced, &site);
        }
        // Regardless of what happened, lock the prototype so it doesn't happen again.
        self.proto.set_input_lock(true);
        self.proto.set_input_errors(fp.has_input_errors());
        self.proto.set_output_errors(fp.has_output_errors());
        Ok(())
    }

    /// The address of this call site's CALL op (C++ `op->getAddr()`).
    fn op_addr(&self, data: &Funcdata) -> Address {
        data.obank()
            .get(self.op)
            .expect("FuncCallSpecs::op_addr: stale call op")
            .get_addr()
            .clone()
    }

    // -- FspecSpace registry wiring (fspec.cc:2155-2169) --------------------

    /// Resolve the display name this call spec prints inside the \e fspec space
    /// (C++ `FspecSpace::printRaw` name/`func_`/`sub_` branch, `fspec.cc:2160`).
    ///
    /// `style` is the architecture's fallback-naming vocabulary
    /// ([`Architecture::kuna_name_style`]; the `Architecture` is visible at the
    /// call site that drives the annotation, so the policy is decided here —
    /// kuna-base, which holds the `FspecSpace` arms, cannot see it).
    pub fn fspec_printed_name(&self, style: crate::database::KunaNameStyle) -> String {
        if !self.name.is_empty() {
            return self.name.clone();
        }
        match style {
            // (kuna) angr-style: sub_<addr>
            crate::database::KunaNameStyle::Angr => {
                crate::database::kuna_function_name(&self.entryaddress)
            }
            // (kuna, Phase 3) ghidra-mode: FUN_%08x (the Java dynamic shape)
            crate::database::KunaNameStyle::Ghidra => {
                crate::database::ghidra_function_name(&self.entryaddress)
            }
            crate::database::KunaNameStyle::Func => {
                let mut s = String::from("func_");
                // printRaw on a real (processor) entry address cannot fail; on the
                // (unreachable) error path leave the prefix only.
                let _ = self.entryaddress.print_raw(&mut s);
                s
            }
        }
    }

    /// Register this call spec's printed name + entry address under the given
    /// \e fspec handle so `FspecSpace::printRaw`/`encodeAttributes` can recover
    /// them (the faithful equivalent of the C++ `(FuncCallSpecs *)offset` cast;
    /// `handle` is the offset of the \e fspec address, the same value
    /// `Funcdata::newVarnodeCallSpecs` takes).
    pub fn register_in_fspec_space(&self, handle: uintb, style: crate::database::KunaNameStyle) {
        kuna_base::space::fspec_register(
            handle,
            kuna_base::space::FspecCallInfo {
                printed_name: self.fspec_printed_name(style),
                entry: self.entryaddress.clone(),
            },
        );
    }

    // -- spacebase placeholder (fspec.cc:4854-4997) -------------------------

    /// Insert a stack-pointer placeholder into the CALL input (C++
    /// `FuncCallSpecs::createPlaceholder`, `fspec.cc:4854`).
    ///
    /// The C++ builds a LOAD-from-stack Varnode (`data.opStackLoad`), inserts it
    /// as the last CALL input, records the slot via `setStackPlaceholderSlot`,
    /// and marks it as a spacebase placeholder.  `opStackLoad` is a W4
    /// `Funcdata` surface not yet on the W3 `Funcdata`, so the build is the
    /// `// STUB(w6-fspec-3 W4)` step; the slot/insert/mark bookkeeping (the part
    /// that uses [`FuncCallSpecs::set_stack_placeholder_slot`]) is transcribed.
    pub fn create_placeholder(
        &mut self,
        data: &mut Funcdata,
        spacebase: &Rc<AddrSpace>,
    ) -> KunaResult<()> {
        let slot = data
            .obank()
            .get(self.op)
            .expect("createPlaceholder: stale call op")
            .num_input();
        let loadval = data.op_stack_load(spacebase, 0, 1, self.op, None, false)?;
        data.op_insert_input(self.op, loadval, slot)?;
        self.set_stack_placeholder_slot(slot);
        if let Some(v) = data.vbank_mut().get_mut(loadval) {
            v.set_spacebase_placeholder();
        }
        Ok(())
    }

    /// Find the active stack-pointer Varnode at this call site by examining the
    /// placeholder slot (C++ `FuncCallSpecs::getSpacebaseRelative`,
    /// `fspec.cc:4987`).
    ///
    /// Returns the LOAD's pointer input (the spacebase reference) or `None`.
    pub fn get_spacebase_relative(&self, data: &Funcdata) -> Option<VarnodeId> {
        if self.stack_placeholder_slot < 0 {
            return None;
        }
        let callop = data.obank().get(self.op)?;
        let tmpvn_id = callop.get_in(self.stack_placeholder_slot)?;
        let tmpvn = data.vbank().get(tmpvn_id)?;
        if !tmpvn.is_spacebase_placeholder() {
            return None;
        }
        if !tmpvn.is_written() {
            return None;
        }
        let loadop_id = tmpvn.get_def()?;
        let loadop = data.obank().get(loadop_id)?;
        if loadop.code() != OpCode::CPUI_LOAD {
            return None;
        }
        loadop.get_in(1) // The load input (ptr) is the reference we want
    }

    /// Calculate the relative stack offset of this call site from the
    /// placeholder Varnode (C++ `FuncCallSpecs::resolveSpacebaseRelative`,
    /// `fspec.cc:4875`).
    ///
    /// `phvn` is the Varnode in the placeholder slot.  The `data.warningHeader`
    /// emission on a non-spacebase reference is a W4 `Funcdata` surface and is
    /// noted but not emitted here (`// STUB(w6-fspec-3 W4)`); the offset
    /// arithmetic, the placeholder-abort short circuit, and the input-locked
    /// branch are ported in full.
    pub fn resolve_spacebase_relative(
        &mut self,
        data: &mut Funcdata,
        phvn: VarnodeId,
    ) -> KunaResult<()> {
        let refvn_id = {
            let phv = data.vbank().get(phvn).expect("resolveSpacebaseRelative: stale phvn");
            let def = phv.get_def().expect("resolveSpacebaseRelative: phvn not written");
            data.obank()
                .get(def)
                .expect("resolveSpacebaseRelative: stale def op")
                .get_in(0)
                .expect("resolveSpacebaseRelative: def has no input 0")
        };
        let refvn = data.vbank().get(refvn_id).expect("resolveSpacebaseRelative: stale refvn");
        let spacebase = Rc::clone(refvn.get_space());
        // STUB(w6-fspec-3 W4): when spacebase is not IPTR_SPACEBASE the C++ emits
        // data.warningHeader("This function may have set the stack pointer");
        // warningHeader is a W4 Funcdata surface, so the warning is not emitted here.
        self.stackoffset = refvn.get_offset();

        if self.stack_placeholder_slot >= 0 {
            let in_at_slot = data
                .obank()
                .get(self.op)
                .expect("resolveSpacebaseRelative: stale call op")
                .get_in(self.stack_placeholder_slot);
            if in_at_slot == Some(phvn) {
                self.abort_spacebase_relative(data);
                return Ok(());
            }
        }

        if self.proto.is_input_locked() {
            // The prototype is locked and had stack parameters; grab the
            // relative offset from this rather than from a placeholder.
            let slot = data
                .obank()
                .get(self.op)
                .expect("resolveSpacebaseRelative: stale call op")
                .get_slot(phvn)
                - 1;
            if slot >= self.proto.num_params() {
                return Err(KunaError::lowlevel(
                    "Stack placeholder does not line up with locked parameter",
                ));
            }
            let param = self
                .proto
                .get_param(slot)
                .expect("resolveSpacebaseRelative: locked param missing");
            let addr = param.get_address().clone();
            // C++'s two nested guards (space mismatch + IPTR_SPACEBASE check) collapsed to one.
            if addr.get_space().map(Rc::as_ptr) != Some(Rc::as_ptr(&spacebase))
                && spacebase.get_type() == spacetype::IPTR_SPACEBASE
            {
                return Err(KunaError::lowlevel(
                    "Stack placeholder does not match locked space",
                ));
            }
            self.stackoffset = self.stackoffset.wsub(addr.get_offset());
            self.stackoffset = spacebase.wrap_offset(self.stackoffset);
            return Ok(());
        }
        Err(KunaError::lowlevel("Unresolved stack placeholder"))
    }

    /// Abort the attempt to recover the relative stack offset (C++
    /// `FuncCallSpecs::abortSpacebaseRelative`, `fspec.cc:4915`).
    ///
    /// Removes any stack-pointer placeholder input, and the op producing it if
    /// it is a dead internal write.
    pub fn abort_spacebase_relative(&mut self, data: &mut Funcdata) {
        if self.stack_placeholder_slot >= 0 {
            let slot = self.stack_placeholder_slot;
            let vn = data
                .obank()
                .get(self.op)
                .expect("abortSpacebaseRelative: stale call op")
                .get_in(slot);
            data.op_remove_input(self.op, slot);
            self.clear_stack_placeholder_slot();
            // Remove the op producing the placeholder as well, if it is a dead
            // internal write.
            if let Some(vn_id) = vn {
                let (no_descend, internal, written, def) = {
                    let v = data.vbank().get(vn_id).expect("abortSpacebaseRelative: stale vn");
                    (
                        v.has_no_descend(),
                        v.get_space().get_type() == spacetype::IPTR_INTERNAL,
                        v.is_written(),
                        v.get_def(),
                    )
                };
                if no_descend && internal && written {
                    if let Some(def_op) = def {
                        data.op_destroy(def_op);
                    }
                }
            }
        }
    }

    // -- locked-parameter transfer helpers (fspec.cc:5043-5144) -------------

    /// Get the index of the CALL input Varnode matching `param`, or the encoded
    /// slot (C++ `FuncCallSpecs::transferLockedInputParam`, `fspec.cc:5043`).
    ///
    /// Returns `0` if the Varnode can't be built, `slot#` (>0) to reuse an
    /// input, or `-1` to build from the stack.
    fn transfer_locked_input_param(&self, param: &dyn ProtoParameter) -> int4 {
        let numtrials = self.activeinput.get_num_trials();
        let startaddr = param.get_address().clone();
        let sz = param.get_size();
        let lastaddr = &startaddr + ((sz - 1) as i64);
        for i in 0..numtrials {
            let curtrial = self.activeinput.get_trial(i);
            if startaddr < *curtrial.get_address() {
                continue;
            }
            let trialend = curtrial.get_address() + ((curtrial.get_size() - 1) as i64);
            if trialend < lastaddr {
                continue;
            }
            if curtrial.is_definitely_not_used() {
                return 0; // Trial has already been stripped
            }
            return curtrial.get_slot();
        }
        if startaddr.get_space().map(|s| s.get_type()) == Some(spacetype::IPTR_SPACEBASE) {
            return -1;
        }
        0
    }

    /// List and/or create a Varnode for each input parameter matching a source
    /// prototype (C++ `FuncCallSpecs::transferLockedInput`, `fspec.cc:5105`).
    ///
    /// `None` entries in `newinput` indicate stack parameters.  The op-input
    /// reuse path reads the CALL operands (the W3 op, via `data`); the stack
    /// path needs a spacebase placeholder.  Returns `false` only if a stack
    /// variable is needed and there is no placeholder.
    fn transfer_locked_input(
        &self,
        data: &Funcdata,
        newinput: &mut Vec<Option<VarnodeId>>,
        source: &FuncProto,
    ) -> KunaResult<bool> {
        // Always keep the call destination address (op->getIn(0)).
        let in0 = data
            .obank()
            .get(self.op)
            .expect("transferLockedInput: stale call op")
            .get_in(0);
        newinput.push(in0);
        let numparams = source.num_params();
        let mut stackref: Option<VarnodeId> = None;
        let mut stackref_resolved = false;
        for i in 0..numparams {
            let param = source.get_param(i).expect("transferLockedInput: source param missing");
            let reuse = self.transfer_locked_input_param(param);
            if reuse == 0 {
                return Ok(false);
            }
            if reuse > 0 {
                let vn = data
                    .obank()
                    .get(self.op)
                    .expect("transferLockedInput: stale call op")
                    .get_in(reuse);
                newinput.push(vn);
            } else {
                if !stackref_resolved {
                    stackref = self.get_spacebase_relative(data);
                    stackref_resolved = true;
                }
                if stackref.is_none() {
                    return Ok(false);
                }
                newinput.push(None);
            }
        }
        Ok(true)
    }

    /// Pass back the Varnode(s) matching the source prototype's return value
    /// (C++ `FuncCallSpecs::transferLockedOutput`, `fspec.cc:5135`).
    fn transfer_locked_output(
        &self,
        data: &Funcdata,
        newoutput: &mut Vec<VarnodeId>,
        source: &FuncProto,
    ) -> KunaResult<bool> {
        let param = source.get_output();
        if param.get_type().map(|t| t.get_metatype()) == Some(type_metatype::TYPE_VOID) {
            return Ok(true);
        }
        // transferLockedOutputParam(param, newoutput): the CALL's current output
        // Varnode (op->getOut()) plus any leading INDIRECT-creation outputs that the
        // (locked) return-value param justifiably contains, or that contain the
        // param.  Before output recovery runs there is no output Varnode yet (the
        // common deindirect case): the list comes back empty and the result is still
        // accurate (the C++ always returns true).
        self.transfer_locked_output_param(data, param, newoutput);
        Ok(true)
    }

    /// Collect the CALL/INDIRECT-creation Varnodes matching a locked return-value
    /// parameter (C++ `FuncCallSpecs::transferLockedOutputParam`, `fspec.cc:5073`).
    fn transfer_locked_output_param(
        &self,
        data: &Funcdata,
        param: &dyn ProtoParameter,
        newoutput: &mut Vec<VarnodeId>,
    ) {
        let paddr = param.get_address();
        let psize = param.get_size();
        if let Some(out) = data.obank().get(self.op).and_then(|o| o.get_out()) {
            if let Some(vn) = data.vbank().get(out) {
                let vaddr = vn.get_addr();
                let vsize = vn.get_size();
                if paddr.justified_contain(psize, vaddr, vsize, false) >= 0
                    || vaddr.justified_contain(vsize, &paddr, psize, false) >= 0
                {
                    newoutput.push(out);
                }
            }
        }
        let mut indop = data.op_previous_op(self.op);
        while let Some(io) = indop {
            let opref = match data.obank().get(io) {
                Some(o) => o,
                None => break,
            };
            if opref.code() != OpCode::CPUI_INDIRECT {
                break;
            }
            if opref.is_indirect_creation() {
                if let Some(out) = opref.get_out().and_then(|ov| data.vbank().get(ov).map(|_| ov)) {
                    let vn = data.vbank().get(out).expect("transferLockedOutputParam: stale out");
                    let vaddr = vn.get_addr();
                    let vsize = vn.get_size();
                    if paddr.justified_contain(psize, vaddr, vsize, false) >= 0
                        || vaddr.justified_contain(vsize, &paddr, psize, false) >= 0
                    {
                        newoutput.push(out);
                    }
                }
            }
            indop = data.op_previous_op(io);
        }
    }

    /// Build the Varnode matching a (locked) input parameter from existing
    /// data-flow (C++ `FuncCallSpecs::buildParam`, `fspec.cc:5010`).  `vn` is the
    /// reused CALL input Varnode (or `None` for a stack parameter), `stackref` the
    /// resolved spacebase Varnode.  Truncates an over-wide input through a SUBPIECE
    /// or loads a stack parameter; returns the Varnode to install as the CALL input.
    fn build_param(
        &self,
        data: &mut Funcdata,
        vn: Option<VarnodeId>,
        param_addr: &Address,
        param_size: int4,
        stackref: Option<VarnodeId>,
    ) -> KunaResult<VarnodeId> {
        let opaddr =
            data.obank().get(self.op).expect("buildParam: stale call op").get_addr().clone();
        let vn = match vn {
            None => {
                // Need to build a spacebase relative varnode.
                let spc = std::rc::Rc::clone(
                    param_addr.get_space().expect("buildParam: param has no space"),
                );
                let off = param_addr.get_offset();
                return data.op_stack_load(
                    &spc,
                    off,
                    param_size as uint4,
                    self.op,
                    stackref,
                    false,
                );
            }
            Some(v) => v,
        };
        let vsize = data.vbank().get(vn).expect("buildParam: stale input vn").get_size();
        if vsize == param_size {
            return Ok(vn);
        }
        let newop = data.new_op(2, opaddr);
        data.op_set_opcode_code(newop, OpCode::CPUI_SUBPIECE);
        let newout = data.new_unique_out(param_size, newop)?;
        // It is possible vn is free, in which case the SetInput would give it
        // multiple descendants; construct a fresh version instead.
        let use_vn = {
            let v = data.vbank().get(vn).expect("buildParam: stale input vn");
            if v.is_free() && !v.is_constant() && !v.has_no_descend() {
                let (sz, addr) = (v.get_size(), v.get_addr().clone());
                data.new_varnode(sz, &addr, None)
            } else {
                vn
            }
        };
        data.op_set_input(newop, use_vn, 0)?;
        let c0 = data.new_constant(4, 0);
        data.op_set_input(newop, c0, 1)?;
        data.op_insert_before(newop, self.op);
        Ok(newout)
    }

    /// Update the CALL input Varnodes to reflect the locked formal input parameters
    /// (C++ `FuncCallSpecs::commitNewInputs`, `fspec.cc:5155`).  `newinput` holds
    /// the old input Varnodes (slot 0 = call destination) and is rewritten in place
    /// to the new inputs before being installed with `opSetAllInput`.
    fn commit_new_inputs(
        &mut self,
        data: &mut Funcdata,
        newinput: &mut Vec<Option<VarnodeId>>,
    ) -> KunaResult<()> {
        if !self.is_input_locked() {
            return Ok(());
        }
        let stackref = self.get_spacebase_relative(data);
        let mut placeholder = if self.stack_placeholder_slot >= 0 {
            data.obank()
                .get(self.op)
                .expect("commitNewInputs: stale call op")
                .get_in(self.stack_placeholder_slot)
        } else {
            None
        };
        let mut noplacehold = true;

        // Clear activeinput and old placeholder.
        self.stack_placeholder_slot = -1;
        let num_passes = self.activeinput.get_num_passes();
        self.activeinput.clear();

        let numparams = self.proto.num_params();
        for i in 0..numparams {
            let (paddr, psize, pspace_is_spacebase) = {
                let param = self.proto.get_param(i).expect("commitNewInputs: source param missing");
                let a = param.get_address();
                let is_sb = a
                    .get_space()
                    .map(|s| s.get_type() == spacetype::IPTR_SPACEBASE)
                    .unwrap_or(false);
                (a, param.get_size(), is_sb)
            };
            let vn = self.build_param(data, newinput[(1 + i) as usize], &paddr, psize, stackref)?;
            newinput[(1 + i) as usize] = Some(vn);
            self.activeinput.register_trial(&paddr, psize);
            self.activeinput.get_trial_mut(i).mark_active(); // not optional
            if noplacehold && pspace_is_spacebase {
                // A locked stack parameter: use it to recover the stack offset.
                data.vbank_mut()
                    .get_mut(vn)
                    .expect("commitNewInputs: stale built vn")
                    .set_spacebase_placeholder();
                noplacehold = false;
                placeholder = None; // with a locked stack param, no placeholder needed
            }
        }
        if let Some(ph) = placeholder {
            // Still need a placeholder: add it at the end of the parameters.
            newinput.push(Some(ph));
            self.set_stack_placeholder_slot((newinput.len() - 1) as int4);
        }
        let all: Vec<VarnodeId> =
            newinput.iter().map(|v| v.expect("commitNewInputs: null input after build")).collect();
        data.op_set_all_input(self.op, &all)?;
        if !self.is_dotdotdot() {
            self.clear_active_input();
        } else if num_passes > 0 {
            self.activeinput.finish_pass();
        }
        Ok(())
    }

    /// Update the CALL output Varnode to reflect the locked formal return value
    /// (C++ `FuncCallSpecs::commitNewOutputs`, `fspec.cc:5206`).  `newoutput` holds
    /// the intersecting output Varnodes gathered by `transferLockedOutput`; they are
    /// merged into a single return-value Varnode (truncations/extensions/concats of
    /// the real output) so the deindirected CALL has one clean output.
    fn commit_new_outputs(
        &mut self,
        data: &mut Funcdata,
        newoutput: &mut [VarnodeId],
    ) -> KunaResult<()> {
        if !self.proto.is_output_locked() {
            return Ok(());
        }
        self.activeoutput.clear();
        if newoutput.is_empty() {
            return Ok(());
        }

        let (paddr, psize) = {
            let param = self.proto.get_output();
            (param.get_address(), param.get_size())
        };
        self.activeoutput.register_trial(&paddr, psize);
        // (The BOOL/typeRecovery opMarkCalculatedBool arm is not reached by the
        // deindirect datatests and needs a W4 surface; omitted faithfully — psize==1
        // BOOL outputs do not occur here.)

        // Find a Varnode that exactly matches the param size.
        let exact_match = newoutput
            .iter()
            .copied()
            .find(|v| data.vbank().get(*v).map(|vn| vn.get_size()) == Some(psize));

        let opaddr =
            data.obank().get(self.op).expect("commitNewOutputs: stale call op").get_addr().clone();

        let real_out: VarnodeId = if let Some(em) = exact_match {
            // Make sure the exact match is the output of the CALL.
            let ind_op = data.vbank().get(em).and_then(|v| v.get_def());
            if ind_op != Some(self.op) {
                // -op- must currently have no output.
                data.op_set_output(self.op, em)?;
                if let Some(io) = ind_op {
                    data.op_unlink(io); // an indirect creation no longer used
                }
            }
            em
        } else {
            data.op_unset_output(self.op);
            data.new_varnode_out(psize, &paddr, self.op)?
        };

        let realout_addr =
            data.vbank().get(real_out).expect("commitNewOutputs: stale realOut").get_addr().clone();
        let realout_size =
            data.vbank().get(real_out).expect("commitNewOutputs: stale realOut").get_size();

        for &old_out in newoutput.iter() {
            if Some(old_out) == exact_match {
                continue;
            }
            let oldsize = data.vbank().get(old_out).expect("commitNewOutputs: stale oldOut").get_size();
            let oldaddr =
                data.vbank().get(old_out).expect("commitNewOutputs: stale oldOut").get_addr().clone();
            let mut ind_op = data.vbank().get(old_out).and_then(|v| v.get_def());
            if ind_op == Some(self.op) {
                ind_op = None;
            }
            if oldsize < psize {
                // Truncate: oldOut is a SUBPIECE of realOut.
                let sub = if let Some(io) = ind_op {
                    data.op_uninsert(io);
                    data.op_set_opcode_code(io, OpCode::CPUI_SUBPIECE);
                    io
                } else {
                    let nop = data.new_op(2, opaddr.clone());
                    data.op_set_opcode_code(nop, OpCode::CPUI_SUBPIECE);
                    data.op_set_output(nop, old_out)?; // move oldOut from op to nop
                    nop
                };
                let overlap = {
                    let ov = data.vbank().get(old_out).expect("commitNewOutputs: stale oldOut");
                    overlap_addr(ov.get_addr(), ov.get_size(), &realout_addr, realout_size)
                };
                data.op_set_input(sub, real_out, 0)?;
                let c = data.new_constant(4, overlap as uintb);
                data.op_set_input(sub, c, 1)?;
                data.op_insert_after(sub, self.op);
            } else if psize < oldsize {
                // Extend: realOut is contained in oldOut.
                let overlap =
                    oldaddr.justified_contain(oldsize, &paddr, psize, false);
                let mut vardata = VarnodeData::default();
                let mut opc = self.proto.assumed_output_extension(&paddr, psize, &mut vardata);
                if opc != OpCode::CPUI_COPY && overlap == 0 {
                    // oldOut is a natural extension of the true output type.
                    if opc == OpCode::CPUI_PIECE {
                        opc = if self.proto.get_output().get_type().map(|t| t.get_metatype())
                            == Some(type_metatype::TYPE_INT)
                        {
                            OpCode::CPUI_INT_SEXT
                        } else {
                            OpCode::CPUI_INT_ZEXT
                        };
                    }
                    let ext = if let Some(io) = ind_op {
                        data.op_uninsert(io);
                        data.op_remove_input(io, 1);
                        data.op_set_opcode_code(io, opc);
                        data.op_set_input(io, real_out, 0)?;
                        io
                    } else {
                        let extop = data.new_op(1, opaddr.clone());
                        data.op_set_opcode_code(extop, opc);
                        data.op_set_output(extop, old_out)?; // move oldOut to extop
                        data.op_set_input(extop, real_out, 0)?;
                        extop
                    };
                    data.op_insert_after(ext, self.op);
                } else {
                    // Concatenate extra bytes from something indirectly created.
                    if let Some(io) = ind_op {
                        data.op_unlink(io);
                    }
                    let most_sig_size = oldsize - overlap - realout_size;
                    let mut last_op = self.op;
                    let is_big_endian = oldaddr.is_big_endian();
                    if overlap != 0 {
                        // Append less-significant bytes to realOut for this oldOut.
                        let lo_addr = if is_big_endian {
                            &oldaddr + (oldsize - overlap) as i64
                        } else {
                            oldaddr.clone()
                        };
                        let new_ind = data.new_indirect_creation(self.op, &lo_addr, overlap, true);
                        let new_ind_out = data
                            .obank()
                            .get(new_ind)
                            .and_then(|o| o.get_out())
                            .expect("commitNewOutputs: indirect-creation has no out");
                        let concat = data.new_op(2, opaddr.clone());
                        data.op_set_opcode_code(concat, OpCode::CPUI_PIECE);
                        data.op_set_input(concat, real_out, 0)?; // most significant
                        data.op_set_input(concat, new_ind_out, 1)?; // least significant
                        data.op_insert_after(concat, self.op);
                        if most_sig_size != 0 {
                            let mid_addr =
                                if is_big_endian { realout_addr.clone() } else { lo_addr.clone() };
                            data.new_varnode_out(overlap + realout_size, &mid_addr, concat)?;
                        }
                        last_op = concat;
                    }
                    if most_sig_size != 0 {
                        // Append more-significant bytes to realOut for this oldOut.
                        let hi_addr = if !is_big_endian {
                            &oldaddr + (realout_size + overlap) as i64
                        } else {
                            oldaddr.clone()
                        };
                        let new_ind =
                            data.new_indirect_creation(self.op, &hi_addr, most_sig_size, true);
                        let new_ind_out = data
                            .obank()
                            .get(new_ind)
                            .and_then(|o| o.get_out())
                            .expect("commitNewOutputs: indirect-creation has no out");
                        let last_out = data
                            .obank()
                            .get(last_op)
                            .and_then(|o| o.get_out())
                            .expect("commitNewOutputs: lastOp has no out");
                        let concat = data.new_op(2, opaddr.clone());
                        data.op_set_opcode_code(concat, OpCode::CPUI_PIECE);
                        data.op_set_input(concat, new_ind_out, 0)?;
                        data.op_set_input(concat, last_out, 1)?;
                        data.op_insert_after(concat, last_op);
                        last_op = concat;
                    }
                    data.op_set_output(last_op, old_out)?; // redefinition complete
                }
            }
        }
        self.clear_active_output();
        Ok(())
    }
}

/// Relative point of overlap of `[addr, addr+size)` against another range
/// (C++ `Varnode::overlap(const Address&,int4)` little-endian path,
/// `varnode.cc`).  The deindirect commit only runs on processor (non-join)
/// addresses; the big-endian justification is handled by the caller's address
/// arithmetic, matching `Address::overlap`.
fn overlap_addr(addr: &Address, size: int4, op: &Address, opsize: int4) -> int4 {
    if !addr.is_big_endian() {
        addr.overlap(0, op, opsize)
    } else {
        let over = addr.overlap(size - 1, op, opsize);
        if over != -1 {
            opsize - 1 - over
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod tests;
