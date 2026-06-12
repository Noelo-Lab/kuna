//! Port of `decompiler/cpp/fspec.cc` lines ~1-2267 (W6, item `w6-s4-fspec-1`):
//! the **parameter-recovery foundation** of the prototype-model subsystem.
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
//! ## Seams
//!
//! - `// SEAM(w6-modelrules)` — [`ModelRule`] and the `AssignAction` machinery
//!   live in `modelrules.cc` (owned by a later item in this wave).  Until then
//!   `ParamListStandard` carries an **empty** `model_rules` list; the
//!   `assignAddress` walk therefore falls straight through to
//!   `assignAddressFallback` (the documented C++ behavior when there are no
//!   `<modelrule>`s), and the `<modelrule>`-affected output paths take the
//!   legacy fallback (`useFillinFallback == true`).  The `ModelRule` type is a
//!   local uninhabitable placeholder enum.
//! - `// SEAM(W4)` — `decode`/`encode` paths reach fspec-owned marshaling
//!   ElementIds/AttributeIds (`<pentry>`, `<group>`, ...) and the
//!   `ProtoModel`/`Architecture` wiring that are not yet ported.  These methods
//!   return `Err(KunaError::lowlevel("SEAM(W4) ..."))`; the pure-algorithm
//!   surfaces above do not depend on them and are exercised directly in tests
//!   via the `seed`/`push_entry` builder seams.  `ProtoModel` itself is
//!   `fspec-2`/`fspec-3`.
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

// =============================================================================
// AssignAction response codes (modelrules.hh:264-270)  // SEAM(w6-modelrules)
// =============================================================================

/// The response code returned by `AssignAction::assignAddress` and the
/// `ParamListStandard` assignment helpers (C++ `enum` inside `AssignAction`,
/// `modelrules.hh:264-270`).
///
/// The discriminants are load-bearing: `ParamListStandard::assignMap` treats
/// `fail`/`no_assignment` as errors, and `ParamListStandardOut::assignMap`
/// branches on the three `hiddenret_*` codes.  // SEAM(w6-modelrules)
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

/// Placeholder for a `<modelrule>` assignment rule (C++ `ModelRule`,
/// `modelrules.hh`).  // SEAM(w6-modelrules)
///
/// The real `ModelRule` family (and the `AssignAction` subclasses it drives)
/// lives in `modelrules.cc`, owned by a later item in this wave.  Until then a
/// `ParamListStandard` carries an empty `Vec<ModelRule>`, so every assignment
/// walk falls through to the fallback algorithm — exactly the documented C++
/// behavior for a model with no `<modelrule>` elements.  This empty enum is
/// uninhabitable; methods that would iterate over rules see an empty list.
#[derive(Debug, Clone)]
pub enum ModelRule {}

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
    /// SEAM(W4): reaches the fspec-owned marshaling ElementIds/AttributeIds
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
            "SEAM(W4) ParamEntry::decode: fspec marshaling element ids not yet ported",
        ))
    }

    /// Test-and-tooling seam: build a fully-formed exclusion/resource entry
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
        // if (entry == null) return false;  if (b.entry == null) return true;
        let (ea, eb) = match (self.entry, b.entry) {
            (None, _) => return Greater, // self is not "<" b  (C++ returns false)
            (_, None) => return Less,    // self "<" b          (C++ returns true)
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
    /// parameter (C++ `assignAddressFromPieces`).
    ///
    /// SEAM(W4): the C++ reaches `JoinRecord::mergeSequence` /
    /// `Architecture::findAddJoin` (the `Architecture`/`Translate` wiring is
    /// W4).  The reversal logic is faithful; the multi-piece join construction
    /// is deferred.
    pub fn assign_address_from_pieces(
        &mut self,
        pieces: &mut [VarnodeData],
        most_to_least: bool,
    ) -> KunaResult<()> {
        if !most_to_least && pieces.len() > 1 {
            pieces.reverse();
        }
        if pieces.len() == 1 {
            self.addr = pieces[0].get_addr();
            return Ok(());
        }
        Err(KunaError::lowlevel(
            "SEAM(W4) ParameterPieces::assign_address_from_pieces: join construction not yet ported",
        ))
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
    /// Rules to apply when assigning addresses (C++ `modelRules`).  Empty until
    /// `modelrules.cc` is ported.  // SEAM(w6-modelrules)
    model_rules: Vec<ModelRule>,
    /// Address space containing relative offset parameters (C++ `spacebase`).
    spacebase: Option<Rc<AddrSpace>>,
    /// If true, use the legacy fillin fallback for output (C++
    /// `ParamListStandardOut::useFillinFallback`).  Always true while there are
    /// no model rules.  // SEAM(w6-modelrules)
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
            // C++: ((hiaddr.offset - base) % align) != 0  (unsigned)
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
    /// `assignAddress`).  With no model rules (the current seam state) this
    /// falls straight through to the fallback.  // SEAM(w6-modelrules)
    #[allow(clippy::too_many_arguments)] // mirrors C++ ParamListStandard::assignAddress
    pub fn assign_address(
        &self,
        dt: &Rc<Datatype>,
        _proto: &PrototypePieces,
        _pos: int4,
        _tlist: &dyn TypeFactory,
        status: &mut [int4],
        res: &mut ParameterPieces,
        manager: &AddrSpaceManager,
    ) -> KunaResult<AssignActionResponse> {
        // SEAM(w6-modelrules): with model rules ported, iterate model_rules here
        // and return the first non-fail response.  model_rules is currently empty.
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

    /// `ParamListStandardOut::assignMap`.
    ///
    /// SEAM(W4): the too-big return-value path reaches
    /// `typefactory.getArch()->getDefaultDataSpace()` and constructs a pointer
    /// type — the `Architecture` wiring is W4.  The common (assignable) path and
    /// the void path are ported faithfully; the hidden-return fallback returns a
    /// seam error.
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
            response = AssignActionResponse::hiddenret_ptrparam;
        }
        if response == AssignActionResponse::hiddenret_ptrparam
            || response == AssignActionResponse::hiddenret_specialreg
            || response == AssignActionResponse::hiddenret_specialreg_void
        {
            res.push(back);
            return Err(KunaError::lowlevel(
                "SEAM(W4) ParamListStandardOut::assignMap hidden-return path needs Architecture wiring",
            ));
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
            let (defnouse, is_act, is_unref, addr_is_spacebase, slotgrp) = {
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
                if !seenchain {
                    max = i;
                }
            }
            if seenchain {
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
        let num_section = trial_start.len() - 1;
        for i in 0..num_section {
            self.force_no_use(active, trial_start[i], trial_start[i + 1]);
        }
        for i in 0..num_section {
            self.force_inactive_chain(active, 2, trial_start[i], trial_start[i + 1], self.resource_start[i]);
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

    /// `ParamListStandardOut::fillinMap` — with no model rules, dispatches to
    /// the legacy fallback (C++ `useFillinFallback == true`).
    /// // SEAM(w6-modelrules)
    fn fillin_map_standard_out(&self, active: &mut ParamActive) {
        if active.get_num_trials() == 0 {
            return;
        }
        // SEAM(w6-modelrules): when model rules exist, the non-fallback path
        // runs first; with none, useFillinFallback is true.
        self.fillin_map_fallback(active, false);
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
    /// `ParamListStandardOut::initialize`).  With no model rules,
    /// `use_fillin_fallback` stays true and `auto_killed_by_call` is set (legacy
    /// behavior).  // SEAM(w6-modelrules)
    pub fn initialize(&mut self) {
        self.use_fillin_fallback = true;
        // SEAM(w6-modelrules): scan model_rules for canAffectFillinOutput here.
        if self.use_fillin_fallback {
            self.auto_killed_by_call = true; // Legacy behavior if there are no rules
        }
    }

    /// Restore the model from an `<input>`/`<output>` element (C++ `decode`).
    ///
    /// SEAM(W4): reaches the fspec-owned marshaling ElementIds/AttributeIds and
    /// the `<modelrule>` decode (SEAM(w6-modelrules)).  Not yet ported; tests
    /// construct models directly via [`ParamListStandard::push_entry`].
    pub fn decode(&mut self, _normalstack: bool) -> KunaResult<()> {
        Err(KunaError::lowlevel(
            "SEAM(W4) ParamListStandard::decode: fspec marshaling element ids not yet ported",
        ))
    }

    // -- test/tooling builder seams -----------------------------------------

    /// Append a fully-formed [`ParamEntry`], replicating the non-marshaling
    /// tail of C++ `parsePentry` (with `splitFloat == true`, the default):
    /// update the resource-section boundaries, `spacebase`, and `numgroup`.
    /// Builder seam for tests and the model builders until the W4 decode path
    /// lands.
    ///
    /// The `groupid` is the new entry's primary group; the C++ derives it from
    /// the parse position, which equals the entry's own group here.
    pub fn push_entry(&mut self, e: ParamEntry) {
        // C++ parsePentry: lastClass = entry.back().isGrouped() ? GENERAL :
        // entry.back().getType(); when the list is empty lastClass = CLASS4.
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
    /// the heritage delay, and build the resolver maps.  Test/builder seam.
    pub fn finish_decode(&mut self) {
        self.resource_start.push(self.numgroup);
        self.calc_delay();
        self.populate_resolver();
    }

    /// Push a resource-section boundary (the C++ `resourceStart.push_back`),
    /// for multi-section models built by tests.
    pub fn push_resource_start(&mut self, group: int4) {
        self.resource_start.push(group);
    }
}

/// Build the standard "cannot assign parameter address" error for a data-type
/// (C++ `ParamUnassignedError`).
fn unassigned_err(dt: &Rc<Datatype>) -> KunaError {
    KunaError::lowlevel(format!(
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
/// The `model` back-pointer (C++ `ProtoModel *`) is `fspec-2`/`fspec-3`; it is
/// omitted here.  // SEAM(w6-fspec-2)
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
}

// =============================================================================
// FspecSpace (fspec.hh:341-351, fspec.cc:2109-2178)  // SEAM(W4)
// =============================================================================

/// Reserved name for the fspec space (C++ `FspecSpace::NAME`).
///
/// The full `FspecSpace` (`AddrSpace` subclass that encodes a `FuncCallSpecs`
/// pointer as an address) reaches `FuncCallSpecs` (a `fspec-2`/`fspec-3` type)
/// and the marshaling encoder, so only its reserved name is carried here.
/// // SEAM(W4)
pub const FSPEC_SPACE_NAME: &str = "fspec";

#[cfg(test)]
mod tests;
