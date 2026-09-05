//! Port of `decompiler/cpp/space.hh` + `space.cc` (W1, item
//! `w1-base-space-address`) — classes for describing address spaces — plus
//! the `AddrSpaceManager` from `translate.hh/.cc`: the W1 lookup core
//! (registration + name/shortcut/index lookup, which `Decoder::readSpace`
//! requires) extended by the W2 `w2-sleigh-translate` item with the
//! `JoinRecord` machinery, `SpacebaseSpace`, the resolver list and the
//! decode entry points.  The abstract `Translate` layer itself lives in
//! `kuna-sleigh::translate`.
//!
//! Also ported here (their C++ homes need types that are not yet available,
//! but they are pure `AddrSpace` subclasses): `FspecSpace` (`fspec.hh/.cc`)
//! and `IopSpace` (`op.hh/.cc`).
//!
//! Structural mapping (vs C++):
//!
//! - C++ spaces are heap objects owned by the manager and shared by raw
//!   pointer everywhere.  Rust spaces are `Rc<AddrSpace>`; pointer equality
//!   maps to `Rc::ptr_eq`.  The `manage`/`trans` back-pointers are **not**
//!   stored (they would form reference cycles); the few methods that need
//!   them take an explicit `&AddrSpaceManager` parameter, and constructors
//!   that consulted `Translate::isBigEndian()` take a `bool` instead.
//!   The register-lookup half of the `trans` back-pointer is modelled by an
//!   explicit [`RegisterLookup`] trait object installed on the manager (set
//!   by the sleigh/architecture bootstrap); paths needing
//!   `Translate::getRegister` error out until one is installed.
//! - C++ virtual dispatch over the `AddrSpace` subclass hierarchy becomes a
//!   private kind discriminant: each `virtual` method matches on the kind,
//!   one arm per C++ override.  The subclass names survive as constructor
//!   types (`ConstantSpace::new()`, ...) carrying their `NAME`/`INDEX`
//!   constants.
//! - Fields the C++ code mutates *after* a space is registered (through the
//!   manager's friend access) are `Cell`s; everything else is set during
//!   construction/decode (`&mut self`, before the `Rc` wrap).  The mutable
//!   join-record table and `SpacebaseSpace` base-register data live in
//!   `RefCell`s inside the owning space's kind (the C++ keeps the join table
//!   on the manager; see `AddrSpaceManager::find_add_join` for why the move
//!   is observationally equivalent).
//! - [`VarnodeStorage`] mirrors the (space, offset, size) triple of C++
//!   `VarnodeData` (pcoderaw.hh) for the join/spacebase/register machinery:
//!   the canonical `VarnodeData` port is `kuna_num::pcoderaw::VarnodeData`,
//!   which cannot be named from this crate (kuna-num depends on kuna-base).
//!   `kuna-sleigh::translate` converts at the boundary.
//!
//! The `FspecSpace` printRaw/encode arms are restored (W6 `fspec-3`): the
//! call-spec layer registers the small slice of `FuncCallSpecs` state these arms
//! read ([`FspecCallInfo`]) under the same integer handle the offset of the
//! \e fspec address carries — the faithful equivalent of the C++ pointer cast.
//! Still deferred (losses ledger): `IopSpace::printRaw` (needs `PcodeOp`, W3).
//! That arm returns `Err(KunaError::Lowlevel)`; everywhere the C++ throws, the
//! exact C++ error string is kept.

use std::cell::{Cell, RefCell};
use std::collections::btree_map::Entry;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::rc::Rc;

use crate::address::{calc_mask, Address, AddrSpacePtr};
use crate::error::{KunaError, KunaResult};
use crate::marshal::{
    cxx_strtoul, AttributeId, Decoder, ElementId, Encoder, ATTRIB_BIGENDIAN, ATTRIB_INDEX,
    ATTRIB_NAME, ATTRIB_OFFSET, ATTRIB_SIZE, ATTRIB_SPACE, ATTRIB_UNKNOWN, ATTRIB_WORDSIZE,
};
use crate::types::{Wrap, HOST_ENDIAN};

/// \brief Fundemental address space types
///
/// Every address space must be one of the following core types
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum spacetype {
    /// Special space to represent constants
    IPTR_CONSTANT = 0,
    /// Normal spaces modelled by processor
    IPTR_PROCESSOR = 1,
    /// addresses = offsets off of base register
    IPTR_SPACEBASE = 2,
    /// Internally managed temporary space
    IPTR_INTERNAL = 3,
    /// Special internal FuncCallSpecs reference
    IPTR_FSPEC = 4,
    /// Special internal PcodeOp reference
    IPTR_IOP = 5,
    /// Special virtual space to represent split variables
    IPTR_JOIN = 6,
}

pub const ATTRIB_BASE: AttributeId = AttributeId::new("base", 89);
pub const ATTRIB_DEADCODEDELAY: AttributeId = AttributeId::new("deadcodedelay", 90);
pub const ATTRIB_DELAY: AttributeId = AttributeId::new("delay", 91);
pub const ATTRIB_LOGICALSIZE: AttributeId = AttributeId::new("logicalsize", 92);
pub const ATTRIB_PHYSICAL: AttributeId = AttributeId::new("physical", 93);

/// ATTRIB_PIECE is a special attribute for supporting the legacy attributes
/// "piece1", "piece2", ..., "piece9".  It is effectively a sequence of
/// indexed attributes for use with Encoder::writeStringIndexed.  The index
/// starts at the ids reserved for "piece1" thru "piece9" but can extend
/// farther.  (Open slots 94-102.)
pub const ATTRIB_PIECE: AttributeId = AttributeId::new("piece", 94);

/// Marshaling attribute "contain" (from `translate.cc`, needed by
/// `SpacebaseSpace::decode`)
pub const ATTRIB_CONTAIN: AttributeId = AttributeId::new("contain", 44);
/// Marshaling attribute "defaultspace" (from `translate.cc`, needed by
/// `AddrSpaceManager::decode_spaces`)
pub const ATTRIB_DEFAULTSPACE: AttributeId = AttributeId::new("defaultspace", 45);

/// Marshaling element \<spaces> (from `translate.cc`, needed by
/// `AddrSpaceManager::decode_spaces`)
pub const ELEM_SPACES: ElementId = ElementId::new("spaces", 31);
/// Marshaling element \<space_base> (from `translate.cc`, needed by
/// `SpacebaseSpace::decode`)
pub const ELEM_SPACE_BASE: ElementId = ElementId::new("space_base", 32);
/// Marshaling element \<space_other> (from `translate.cc`, needed by
/// `AddrSpaceManager::decode_space`)
pub const ELEM_SPACE_OTHER: ElementId = ElementId::new("space_other", 33);
/// Marshaling element \<space_overlay> (from `translate.cc`, needed by
/// `OverlaySpace::decode`; the rest of the translate id table lives in
/// `kuna-sleigh::translate`).
pub const ELEM_SPACE_OVERLAY: ElementId = ElementId::new("space_overlay", 34);
/// Marshaling element \<space_unique> (from `translate.cc`, needed by
/// `AddrSpaceManager::decode_space`)
pub const ELEM_SPACE_UNIQUE: ElementId = ElementId::new("space_unique", 35);

/// Boolean attributes (flags) of an [`AddrSpace`] (the C++ anonymous enum).
pub mod addrspace_flags {
    #![allow(non_upper_case_globals)]

    /// Space is big endian if set, little endian otherwise
    pub const big_endian: u32 = 1;
    /// This space is heritaged
    pub const heritaged: u32 = 2;
    /// Dead-code analysis is done on this space
    pub const does_deadcode: u32 = 4;
    /// Space is specific to a particular loadimage
    pub const programspecific: u32 = 8;
    /// Justification within aligned word is opposite of endianness
    pub const reverse_justification: u32 = 16;
    /// Space attached to the formal \b stack \b pointer
    pub const formal_stackspace: u32 = 0x20;
    /// This space is an overlay of another space
    pub const overlay: u32 = 0x40;
    /// This is the base space for overlay space(s)
    pub const overlaybase: u32 = 0x80;
    /// Space is truncated from its original size, expect pointers larger
    /// than this size
    pub const truncated: u32 = 0x100;
    /// Has physical memory associated with it
    pub const hasphysical: u32 = 0x200;
    /// Quick check for the OtherSpace derived class
    pub const is_otherspace: u32 = 0x400;
    /// Does there exist near pointers into this space
    pub const has_nearpointers: u32 = 0x800;
}

use addrspace_flags as fl;

// ---------------------------------------------------------------------------
// VarnodeStorage — the C++ VarnodeData triple, as needed below kuna-num
// ---------------------------------------------------------------------------

/// The (space, offset, size) storage triple of the C++ `VarnodeData`
/// (pcoderaw.hh), mirrored here for the machinery from `translate.cc/hh`
/// that must live in this crate ([`JoinRecord`] pieces, `SpacebaseSpace`
/// base registers, [`RegisterLookup`] results).
///
/// The canonical port of `VarnodeData` is `kuna_num::pcoderaw::VarnodeData`
/// — it cannot be named from kuna-base (kuna-num depends on kuna-base), so
/// the comparison operators are transcribed here a second time and
/// `kuna-sleigh::translate` provides the conversions between the two
/// representations (recorded in the rust-port losses ledger).
#[derive(Debug, Clone, Default)]
pub struct VarnodeStorage {
    /// The address space (C++ `AddrSpace *`; `None` is the null pointer)
    pub space: Option<Rc<AddrSpace>>,
    /// The offset within the space
    pub offset: u64,
    /// The number of bytes in the location
    pub size: u32,
}

/// C++ raw-pointer equality on optional space fields (null == null).
fn space_opt_ptr_eq(a: &Option<Rc<AddrSpace>>, b: &Option<Rc<AddrSpace>>) -> bool {
    match (a, b) {
        (Some(a), Some(b)) => Rc::ptr_eq(a, b),
        (None, None) => true,
        _ => false,
    }
}

impl PartialEq for VarnodeStorage {
    /// C++ `VarnodeData::operator==`: the space (by pointer), offset, and
    /// size must all be exactly equal.
    fn eq(&self, op2: &VarnodeStorage) -> bool {
        if !space_opt_ptr_eq(&self.space, &op2.space) {
            return false;
        }
        if self.offset != op2.offset {
            return false;
        }
        self.size == op2.size
    }
}
impl Eq for VarnodeStorage {}

impl Ord for VarnodeStorage {
    /// C++ `VarnodeData::operator<`: sort on the space's index, then the
    /// offset, and finally by size with BIG sizes coming first.  (Transcribed
    /// field-for-field from `kuna_num::pcoderaw::VarnodeData`.)
    fn cmp(&self, op2: &VarnodeStorage) -> std::cmp::Ordering {
        // C++ `if (space != op2.space)` is a pointer compare; distinct
        // spaces order by their index.  Dereferencing a null space here is
        // C++ UB -> panic (ADR 0004).
        if !space_opt_ptr_eq(&self.space, &op2.space) {
            let ind1 = self
                .space
                .as_ref()
                .expect("VarnodeStorage::cmp: null space pointer (C++ UB)")
                .get_index();
            let ind2 = op2
                .space
                .as_ref()
                .expect("VarnodeStorage::cmp: null space pointer (C++ UB)")
                .get_index();
            if ind1 != ind2 {
                return ind1.cmp(&ind2);
            }
            // Distinct space objects sharing an index cannot happen within
            // one manager; fall through so the order stays total.
        }
        if self.offset != op2.offset {
            return self.offset.cmp(&op2.offset);
        }
        op2.size.cmp(&self.size) // BIG sizes come first
    }
}

impl PartialOrd for VarnodeStorage {
    fn partial_cmp(&self, op2: &VarnodeStorage) -> Option<std::cmp::Ordering> {
        Some(self.cmp(op2))
    }
}

impl VarnodeStorage {
    /// Get the location of the varnode as an address (C++
    /// `VarnodeData::getAddr`).
    pub fn get_addr(&self) -> Address {
        match &self.space {
            Some(spc) => Address::new(Rc::clone(spc), self.offset),
            // C++ would construct an Address with a null space pointer
            None => Address::from_space_ptr(AddrSpacePtr::Null, self.offset),
        }
    }

    /// Is \b this contiguous (as the most significant piece) with the given
    /// triple (C++ `VarnodeData::isContiguous`).
    pub fn is_contiguous(&self, lo: &VarnodeStorage) -> bool {
        if !space_opt_ptr_eq(&self.space, &lo.space) {
            return false;
        }
        // C++ dereferences the space pointer below: null is UB -> panic
        let space = self
            .space
            .as_ref()
            .expect("VarnodeStorage::is_contiguous: null space pointer (C++ UB)");
        if space.is_big_endian() {
            let nextoff = space.wrap_offset(self.offset.wadd(u64::from(self.size)));
            if nextoff == lo.offset {
                return true;
            }
        } else {
            let nextoff = space.wrap_offset(lo.offset.wadd(u64::from(lo.size)));
            if nextoff == self.offset {
                return true;
            }
        }
        false
    }
}

// ---------------------------------------------------------------------------
// RegisterLookup / AddressResolver — abstract surfaces from translate.hh
// ---------------------------------------------------------------------------

/// The register-lookup surface of the C++ `Translate` interface
/// (translate.hh), split out so code in this crate can consult the
/// processor's registers without naming the full `Translate` trait (whose
/// home is `kuna-sleigh::translate`, where it has `RegisterLookup` as a
/// supertrait).
///
/// An implementation is installed on the [`AddrSpaceManager`] via
/// [`AddrSpaceManager::set_register_lookup`]; this stands in for the C++
/// `AddrSpace::trans` back-pointer.
pub trait RegisterLookup {
    /// C++ `Translate::getRegister`: get a register as a storage triple
    /// given its name.  Errs (C++ throws `SleighError`, a `LowlevelError`)
    /// if the register does not exist.
    fn get_register(&self, nm: &str) -> KunaResult<VarnodeStorage>;

    /// Speculative form of [`RegisterLookup::get_register`]: ask whether `nm`
    /// resolves *here*, cheaply and without side effects.
    ///
    /// `None` means "not resolvable on this lookup", never "this language has
    /// no such register".  A back end that answers register questions by
    /// asking a host — the Ghidra front end, whose exact lookup is a
    /// `getRegister` query the host answers by throwing on an undefined name —
    /// overrides this to consult only what it already knows.  Code that is
    /// merely testing "does this language happen to have X?" must use this
    /// rather than the exact lookup.
    fn probe_register(&self, nm: &str) -> Option<VarnodeStorage> {
        self.get_register(nm).ok()
    }

    /// C++ `Translate::getRegisterName`: get the name of the smallest
    /// containing register given a location and size, or an empty string.
    fn get_register_name(&self, base: &Rc<AddrSpace>, off: u64, size: i32) -> String;

    /// C++ `Translate::getExactRegisterName`: get the name of a register
    /// with an exact location and size, or an empty string.
    fn get_exact_register_name(&self, base: &Rc<AddrSpace>, off: u64, size: i32) -> String;
}

/// The shared error for paths that need `Translate::getRegister` before any
/// [`RegisterLookup`] has been installed on the manager.  (In C++ the
/// back-pointer always exists; in kuna it is absent until the sleigh wave's
/// engine — or a test stub — installs one.)
pub fn no_register_lookup_err() -> KunaError {
    KunaError::lowlevel(
        "kuna rust port: no Translate/register lookup installed in the AddrSpaceManager",
    )
}

/// \brief Abstract class for converting native constants to addresses
/// (translate.hh `AddressResolver`)
///
/// This class is used if there is a special calculation to get from a
/// constant embedded in the code being analyzed to the actual Address being
/// referred to.  This is used especially in the case of a segmented
/// architecture, where "near" pointers must be extended to a full address
/// with implied segment information.
pub trait AddressResolver {
    /// \brief The main resolver method.
    ///
    /// Given a native constant in a specific context, resolve what address
    /// is being referred to.  The constant can be a partially encoded
    /// pointer, in which case the full pointer encoding is recovered as well
    /// as the address.  Whether or not a pointer is partially encoded or not
    /// is determined by the \e sz parameter, indicating the number of bytes
    /// in the pointer. A value of -1 here indicates that the pointer is
    /// known to be a full encoding.
    /// \param val is the constant to be resolved to an address
    /// \param sz is the size of \e val in context (or -1)
    /// \param point is the address at which this constant is being used
    /// \param full_encoding holds the full pointer encoding if \b val is a
    ///        partial encoding
    /// \return the resolved Address
    fn resolve(
        &self,
        val: u64,
        sz: i32,
        point: &Address,
        full_encoding: &mut u64,
    ) -> KunaResult<Address>;
}

// ---------------------------------------------------------------------------
// JoinRecord (translate.hh/.cc)
// ---------------------------------------------------------------------------

/// \brief A record describing how logical values are split
///
/// The decompiler can describe a logical value that is stored split across
/// multiple physical memory locations.  This record describes such a split.
/// The pieces must be listed from \e most \e significant to \e least
/// \e significant.
#[derive(Debug)]
pub struct JoinRecord {
    /// All the physical pieces of the symbol, most significant to least
    pieces: Vec<VarnodeStorage>,
    /// Special entry representing entire symbol in one chunk
    unified: VarnodeStorage,
}

impl JoinRecord {
    /// Get number of pieces in this record
    pub fn num_pieces(&self) -> i32 {
        self.pieces.len() as i32 // cast: C++ returns int4 from vector::size()
    }

    /// Does this record extend a float varnode
    pub fn is_float_extension(&self) -> bool {
        self.pieces.len() == 1
    }

    /// Get the i-th piece (panics when out of range, where C++ reads out of
    /// bounds)
    pub fn get_piece(&self, i: i32) -> &VarnodeStorage {
        &self.pieces[i as usize] // cast: int4 index, C++ UB when out of range
    }

    /// Get the Varnode whole
    pub fn get_unified(&self) -> &VarnodeStorage {
        &self.unified
    }

    /// Given offset in \e join space, get equivalent address of piece.
    ///
    /// The \e join space range maps to the underlying pieces in a natural
    /// endian aware way.  Given an offset in the range, figure out what
    /// address it is mapping to.  The particular piece is passed back as an
    /// index (`pos`), and the Address is returned (invalid when the offset
    /// falls outside the record, like the C++ default `Address()`).
    pub fn get_equivalent_address(&self, offset: u64, pos: &mut i32) -> Address {
        if offset < self.unified.offset {
            return Address::new_invalid(); // offset comes before this range
        }
        // int4 smallOff = (int4)(offset - unified.offset): truncating cast
        let mut small_off = offset.wsub(self.unified.offset) as i32;
        // C++ indexes pieces[0] unconditionally: an empty record is UB
        let big_endian = self.pieces[0]
            .space
            .as_ref()
            .expect("JoinRecord::get_equivalent_address: null piece space (C++ UB)")
            .is_big_endian();
        let num = self.pieces.len() as i32; // cast: int4 loop bound
        if big_endian {
            *pos = 0;
            while *pos < num {
                let piece_size = self.pieces[*pos as usize].size as i32; // cast: uint4 -> int4
                if small_off < piece_size {
                    break;
                }
                small_off -= piece_size;
                *pos += 1;
            }
            if *pos == num {
                return Address::new_invalid(); // offset comes after this range
            }
        } else {
            *pos = num - 1;
            while *pos >= 0 {
                let piece_size = self.pieces[*pos as usize].size as i32; // cast: uint4 -> int4
                if small_off < piece_size {
                    break;
                }
                small_off -= piece_size;
                *pos -= 1;
            }
            if *pos < 0 {
                return Address::new_invalid(); // offset comes after this range
            }
        }
        let piece = &self.pieces[*pos as usize];
        Address::new(
            Rc::clone(
                piece
                    .space
                    .as_ref()
                    .expect("JoinRecord::get_equivalent_address: null piece space (C++ UB)"),
            ),
            // pieces[pos].offset + smallOff: uintb + int4 sign-extends
            piece.offset.wadd(small_off as i64 as u64),
        )
    }

    /// Merge any contiguous ranges in a sequence (C++ static
    /// `JoinRecord::mergeSequence`).
    ///
    /// Assuming the given list of triples go from most significant to least
    /// significant, merge any contiguous elements in the list.  Varnodes
    /// that are not in the \e stack address space are only merged if the
    /// resulting byte range has a formal register name.
    /// \param seq is the given list of triples
    /// \param trans is the language to use for register names
    pub fn merge_sequence(seq: &mut Vec<VarnodeStorage>, trans: &dyn RegisterLookup) {
        let mut i: usize = 1;
        while i < seq.len() {
            let hi = &seq[i - 1];
            let lo = &seq[i];
            if hi.is_contiguous(lo) {
                break;
            }
            i += 1;
        }
        if i >= seq.len() {
            return;
        }
        let mut res: Vec<VarnodeStorage> = Vec::new();
        i = 1;
        res.push(seq.first().expect("mergeSequence on empty sequence (C++ UB)").clone());
        let mut last_is_informal = false;
        while i < seq.len() {
            let lo = seq[i].clone();
            let hi = res.last_mut().expect("res starts non-empty");
            if hi.is_contiguous(&lo) {
                let hi_space = hi
                    .space
                    .as_ref()
                    .expect("mergeSequence: null space pointer (C++ UB)");
                hi.offset = if hi_space.is_big_endian() { hi.offset } else { lo.offset };
                hi.size = hi.size.wadd(lo.size);
                if hi_space.get_type() != spacetype::IPTR_SPACEBASE {
                    let hi_space = Rc::clone(hi_space);
                    last_is_informal =
                        trans.get_exact_register_name(&hi_space, hi.offset, hi.size as i32) // cast: uint4 size -> int4 parameter
                            .is_empty();
                }
            } else {
                if last_is_informal {
                    break;
                }
                res.push(lo);
            }
            i += 1;
        }
        if last_is_informal {
            // If the merge contains an informal register,
            return; // throw it out and keep the original sequence
        }
        *seq = res;
    }
}

impl PartialEq for JoinRecord {
    /// Set-equivalence under the C++ `JoinRecordCompare` (C++ defines no
    /// `operator==` for JoinRecord).
    fn eq(&self, op2: &JoinRecord) -> bool {
        self.cmp(op2) == std::cmp::Ordering::Equal
    }
}
impl Eq for JoinRecord {}

impl Ord for JoinRecord {
    /// C++ `JoinRecord::operator<`: compare records lexigraphically by
    /// pieces.  Allows sorting on JoinRecords so that a collection of pieces
    /// can be quickly mapped to its logical whole, specified with a join
    /// address.
    fn cmp(&self, op2: &JoinRecord) -> std::cmp::Ordering {
        // Some joins may have same piece but different unified size
        // (floating point)
        if self.unified.size != op2.unified.size {
            // Compare size first
            return self.unified.size.cmp(&op2.unified.size);
        }
        // Lexigraphic sort on pieces
        let mut i: usize = 0;
        loop {
            if self.pieces.len() == i {
                // If more pieces in op2, it is bigger; if same number
                // this == op2
                return if op2.pieces.len() > i {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Equal
                };
            }
            if op2.pieces.len() == i {
                // More pieces in -this-, so it is bigger
                return std::cmp::Ordering::Greater;
            }
            if self.pieces[i] != op2.pieces[i] {
                return self.pieces[i].cmp(&op2.pieces[i]);
            }
            i += 1;
        }
    }
}

impl PartialOrd for JoinRecord {
    fn partial_cmp(&self, op2: &JoinRecord) -> Option<std::cmp::Ordering> {
        Some(self.cmp(op2))
    }
}

/// The mutable join-record table of the C++ `AddrSpaceManager`
/// (`joinallocate`/`splitset`/`splitlist`).  In the Rust port it lives
/// behind a `RefCell` inside the join space's kind, so the `JoinSpace`
/// virtual methods can reach it without the C++ `manage` back-pointer.
#[derive(Debug, Default)]
struct JoinState {
    /// Next offset to be allocated in join space (C++ `joinallocate`)
    joinallocate: u64,
    /// Different splits that have been defined in join space (C++
    /// `set<JoinRecord *,JoinRecordCompare> splitset`; the comparator is
    /// `JoinRecord`'s `Ord`)
    splitset: BTreeSet<Rc<JoinRecord>>,
    /// JoinRecords indexed by join address (C++ `splitlist`)
    splitlist: Vec<Rc<JoinRecord>>,
}

impl JoinState {
    /// C++ `AddrSpaceManager::findJoinInternal`: find the JoinRecord that
    /// contains \e offset, as a range in the \e join address space.  If
    /// there is no existing record, None is returned.
    fn find_join_internal(&self, offset: u64) -> Option<Rc<JoinRecord>> {
        let mut min: i32 = 0;
        let mut max: i32 = self.splitlist.len() as i32 - 1; // cast: int4 in C++
        while min <= max {
            // Binary search
            let mid = (min + max) / 2;
            let rec = &self.splitlist[mid as usize];
            let val = rec.unified.offset;
            // val + rec->unified.size: uintb + uint4 zero-extends and wraps
            if val.wadd(u64::from(rec.unified.size)) <= offset {
                min = mid + 1;
            } else if val > offset {
                max = mid - 1;
            } else {
                return Some(Rc::clone(rec));
            }
        }
        None
    }

    /// C++ `AddrSpaceManager::findJoin`: find the JoinRecord whose unified
    /// range starts exactly at \e offset.  The offset must originally have
    /// come from a JoinRecord returned by `find_add_join`, otherwise this
    /// method errs.
    fn find_join(&self, offset: u64) -> KunaResult<Rc<JoinRecord>> {
        let mut min: i32 = 0;
        let mut max: i32 = self.splitlist.len() as i32 - 1; // cast: int4 in C++
        while min <= max {
            // Binary search
            let mid = (min + max) / 2;
            let rec = &self.splitlist[mid as usize];
            let val = rec.unified.offset;
            if val == offset {
                return Ok(Rc::clone(rec));
            }
            if val < offset {
                min = mid + 1;
            } else {
                max = mid - 1;
            }
        }
        Err(KunaError::lowlevel("Unlinked join address"))
    }
}

/// The mutable base-register data of a `SpacebaseSpace` (translate.hh).
/// C++ mutates these through the manager's friend access after the space is
/// registered, hence the `RefCell` in the kind.
#[derive(Debug, Default)]
struct SpacebaseState {
    /// true if a base register has been attached
    hasbaseregister: bool,
    /// true if stack grows in negative direction
    is_negative_stack: bool,
    /// Location data of the base register
    baseloc: VarnodeStorage,
    /// Original base register before any truncation
    base_orig: VarnodeStorage,
}

/// The dispatch discriminant standing in for the C++ `AddrSpace` subclass
/// vtable: each C++ `virtual` override becomes a match arm on this kind.
#[derive(Debug)]
enum AddrSpaceKind {
    /// A concrete base-class `AddrSpace`
    Base,
    /// `ConstantSpace`
    Constant,
    /// `OtherSpace`
    Other,
    /// `UniqueSpace`
    Unique,
    /// `JoinSpace`, carrying the manager's join-record table (see
    /// [`JoinState`])
    Join {
        /// The join-record table (C++ manager members
        /// `joinallocate`/`splitset`/`splitlist`)
        state: RefCell<JoinState>,
    },
    /// `SpacebaseSpace` (translate.hh)
    Spacebase {
        /// Containing space (C++ `contain`; null until decode for the
        /// partial constructor)
        contain: Option<Rc<AddrSpace>>,
        /// Mutable base-register data (C++
        /// `hasbaseregister`/`isNegativeStack`/`baseloc`/`baseOrig`)
        state: RefCell<SpacebaseState>,
    },
    /// `FspecSpace` (declared in fspec.hh)
    Fspec,
    /// `IopSpace` (declared in op.hh)
    Iop,
    /// `OverlaySpace`, with the space being overlayed (C++ `baseSpace`,
    /// set during decode)
    Overlay {
        /// Space being overlayed
        base_space: Option<Rc<AddrSpace>>,
    },
}

/// \brief A region where processor data is stored
///
/// An AddrSpace (Address Space) is an arbitrary sequence of bytes where a
/// processor can store data. As is usual with most processors' concept of
/// RAM, an integer offset paired with an AddrSpace forms the address (See
/// Address) of a byte.  The \e size of an AddrSpace indicates the number of
/// bytes that can be separately addressed and is usually described by the
/// number of bytes needed to encode the biggest offset.  I.e. a \e 4-byte
/// address space means that there are offsets ranging from 0x00000000 to
/// 0xffffffff within the space for a total of 2^32 addressable bytes within
/// the space.  There can be multiple address spaces, and it is typical to
/// have spaces
///     - \b ram        Modeling the main processor address bus
///     - \b register   Modeling a processors registers
///
/// The processor specification can set up any address spaces it needs in an
/// arbitrary manner, but \e all data manipulated by the processor, which the
/// specification hopes to model, must be contained in some address space,
/// including RAM, ROM, general registers, special registers, i/o ports, etc.
///
/// The analysis engine also uses additional address spaces to model special
/// concepts.  These include
///     - \b const        There is a \e constant address space for modeling
///                       constant values in p-code expressions
///                       (See ConstantSpace)
///     - \b unique       There is always a \e unique address space used as a
///                       pool for temporary registers. (See UniqueSpace)
#[derive(Debug)]
pub struct AddrSpace {
    /// Type of space (PROCESSOR, CONSTANT, INTERNAL, ...)
    type_: spacetype,
    // (kuna rust) C++ `manage`/`trans` back-pointers are not stored; see
    // module docs.
    /// Number of managers using this space
    refcount: Cell<i32>,
    /// Attributes of the space
    flags: Cell<u32>,
    /// Highest (byte) offset into this space
    highest: Cell<u64>,
    /// Offset below which we don't search for pointers
    pointer_lower_bound: Cell<u64>,
    /// Offset above which we don't search for pointers
    pointer_upper_bound: Cell<u64>,
    /// Shortcut character for printing (C++ `char`)
    shortcut: Cell<u8>,
    /// Name of this space
    name: String,
    /// Size of an address into this space in bytes
    address_size: Cell<u32>,
    /// Size of unit being addressed (1=byte)
    wordsize: u32,
    /// Smallest size of a pointer into \b this space (in bytes)
    minimum_pointer_size: Cell<i32>,
    /// An integer identifier for the space
    index: i32,
    /// Delay in heritaging this space
    delay: i32,
    /// Delay before deadcode removal is allowed on this space
    deadcodedelay: Cell<i32>,
    /// Subclass discriminant (C++ vtable)
    kind: AddrSpaceKind,
}

impl AddrSpace {
    /// Initialize an address space with its basic attributes.
    ///
    /// C++ signature carries leading `AddrSpaceManager *m, const Translate
    /// *t` back-pointer arguments, omitted here (see module docs).
    ///
    /// \param tp is the type of the new space (PROCESSOR, CONSTANT, INTERNAL,...)
    /// \param nm is the name of the new space
    /// \param big_end is \b true for big endian encoding
    /// \param size is the (offset encoding) size of the new space
    /// \param ws is the number of bytes in an addressable unit
    /// \param ind is the integer identifier for the new space
    /// \param flags can be 0 or AddrSpace::hasphysical
    /// \param dl is the number of rounds to delay heritage for the new space
    /// \param dead is the number of rounds to delay before dead code removal
    #[allow(clippy::too_many_arguments)] // C++ constructor signature
    pub fn new(
        tp: spacetype,
        nm: &str,
        big_end: bool,
        size: u32,
        ws: u32,
        ind: i32,
        flags: u32,
        dl: i32,
        dead: i32,
    ) -> AddrSpace {
        let space = AddrSpace {
            type_: tp,
            refcount: Cell::new(0), // No references to this space yet
            // These are the flags we allow to be set from constructor;
            // heritaged/does_deadcode are always on unless explicitly turned
            // off in a derived constructor
            flags: Cell::new((flags & fl::hasphysical)
                | if big_end { fl::big_endian } else { 0 }
                | (fl::heritaged | fl::does_deadcode)),
            highest: Cell::new(0),
            pointer_lower_bound: Cell::new(0),
            pointer_upper_bound: Cell::new(0),
            shortcut: Cell::new(b' '), // Placeholder meaning shortcut is unassigned
            name: nm.to_string(),
            address_size: Cell::new(size),
            wordsize: ws,
            // (initially) assume pointers must match the space size exactly
            minimum_pointer_size: Cell::new(0),
            index: ind,
            delay: dl,
            deadcodedelay: Cell::new(dead),
            kind: AddrSpaceKind::Base,
        };
        space.calc_scale_mask();
        space
    }

    /// This is a partial constructor, for initializing a space via decode
    /// (C++ `AddrSpace(AddrSpaceManager *m,const Translate *t,spacetype tp)`).
    /// Fields the C++ leaves uninitialized are zeroed here.
    pub fn new_for_decode(tp: spacetype) -> AddrSpace {
        AddrSpace {
            type_: tp,
            refcount: Cell::new(0),
            // Always on unless explicitly turned off in derived constructor;
            // we let big_endian get set by attribute
            flags: Cell::new(fl::heritaged | fl::does_deadcode),
            highest: Cell::new(0),
            pointer_lower_bound: Cell::new(0),
            pointer_upper_bound: Cell::new(0),
            shortcut: Cell::new(b' '),
            name: String::new(),
            address_size: Cell::new(0),
            wordsize: 1,
            minimum_pointer_size: Cell::new(0),
            index: 0,
            delay: 0,
            deadcodedelay: Cell::new(0),
            kind: AddrSpaceKind::Base,
        }
    }

    /// Calculate \e highest based on \e addressSize, and \e wordsize.
    /// This also calculates the default pointerLowerBound
    fn calc_scale_mask(&self) {
        let mut highest = calc_mask(self.address_size.get() as i32); // Maximum address
        // Maximum byte address (wrapping like the C++ unsigned arithmetic)
        highest = highest.wmul(self.wordsize as u64).wadd((self.wordsize as u64).wsub(1));
        self.highest.set(highest);
        let buffer_size: u64 = if self.address_size.get() < 3 { 0x100 } else { 0x1000 };
        self.pointer_lower_bound.set(buffer_size);
        self.pointer_upper_bound.set(highest.wsub(buffer_size));
    }

    /// An internal method for derived classes (and the manager) to set
    /// space attributes (C++ protected `setFlags`)
    pub fn set_flags(&self, flags: u32) {
        self.flags.set(self.flags.get() | flags);
    }

    /// An internal method for derived classes (and the manager) to clear
    /// space attributes (C++ protected `clearFlags`)
    pub fn clear_flags(&self, flags: u32) {
        self.flags.set(self.flags.get() & !flags);
    }

    /// The logical form of the space is truncated from its actual size.
    /// Pointers may refer to this original size put the most significant
    /// bytes are ignored
    /// \param newsize is the size (in bytes) of the truncated (logical) space
    pub fn truncate_space(&self, newsize: u32) {
        self.set_flags(fl::truncated);
        self.address_size.set(newsize);
        self.minimum_pointer_size.set(newsize as i32);
        self.calc_scale_mask();
    }

    /// Get the name.  Every address space has a (unique) name, which is
    /// referred to especially in configuration files via XML.
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Get the type of space.
    ///   - IPTR_CONSTANT for the constant space
    ///   - IPTR_PROCESSOR for a normal space
    ///   - IPTR_INTERNAL for the temporary register space
    ///   - IPTR_FSPEC for special FuncCallSpecs references
    ///   - IPTR_IOP for special PcodeOp references
    pub fn get_type(&self) -> spacetype {
        self.type_
    }

    /// Get number of heritage passes being delayed.
    ///
    /// If the heritage algorithms need to trace dataflow within this space,
    /// the algorithms can delay tracing this space in order to let indirect
    /// references into the space resolve themselves.  This method indicates
    /// the number of rounds of dataflow analysis that should be skipped for
    /// this space to let this resolution happen.
    pub fn get_delay(&self) -> i32 {
        self.delay
    }

    /// Get number of passes before deadcode removal is allowed.
    ///
    /// The point at which deadcode removal is performed on varnodes within a
    /// space can be set to skip some number of heritage passes, in case not
    /// all the varnodes are created within a single pass.
    pub fn get_deadcode_delay(&self) -> i32 {
        self.deadcodedelay.get()
    }

    /// Get the integer identifier.  Each address space has an associated
    /// index that can be used as an integer encoding of the space.
    pub fn get_index(&self) -> i32 {
        self.index
    }

    /// Get the addressable unit size.  This method indicates the number of
    /// bytes contained in an \e addressable \e unit of this space.  This is
    /// almost always 1, but can be any other small integer.
    pub fn get_word_size(&self) -> u32 {
        self.wordsize
    }

    /// Get the size of the space.  Return the number of bytes needed to
    /// represent an offset into this space.  A space with 2^32 bytes has an
    /// address size of 4, for instance.
    pub fn get_addr_size(&self) -> u32 {
        self.address_size.get()
    }

    /// Get the highest (byte) offset possible for this space
    pub fn get_highest(&self) -> u64 {
        self.highest.get()
    }

    /// Get lower bound for assuming an offset is a pointer.  Constant
    /// offsets are tested against \b this lower bound as a quick filter
    /// before attempting to lookup symbols.
    pub fn get_pointer_lower_bound(&self) -> u64 {
        self.pointer_lower_bound.get()
    }

    /// Get upper bound for assuming an offset is a pointer.
    pub fn get_pointer_upper_bound(&self) -> u64 {
        self.pointer_upper_bound.get()
    }

    /// Get the minimum pointer size for \b this space.  A value of 0 means
    /// the size must match exactly. If the space is truncated, or if there
    /// exists near pointers, this value may be non-zero.
    pub fn get_minimum_ptr_size(&self) -> i32 {
        self.minimum_pointer_size.get()
    }

    /// Wrap -off- to the offset that fits into this space.
    ///
    /// Calculate \e off modulo the size of this address space in order to
    /// construct the offset "equivalent" to \e off that fits properly into
    /// this space
    pub fn wrap_offset(&self, off: u64) -> u64 {
        if off <= self.highest.get() {
            // Comparison is unsigned
            return off;
        }
        let modulus = (self.highest.get().wadd(1)) as i64;
        let mut res = (off as i64).wrem(modulus); // remainder is signed
        if res < 0 {
            // Remainder may be negative
            res = res.wadd(modulus); // Adding mod guarantees res is in (0,mod)
        }
        res as u64
    }

    /// Get the shortcut character.  Return a unique short cut character that
    /// is associated with this space.  The shortcut character can be used by
    /// the read method to quickly specify the space of an address.
    pub fn get_shortcut(&self) -> char {
        self.shortcut.get() as char
    }

    /// (manager internal) Set the shortcut character (C++ friend access)
    pub(crate) fn set_shortcut_raw(&self, sc: u8) {
        self.shortcut.set(sc);
    }

    /// Return \b true if dataflow has been traced.
    ///
    /// During analysis, memory locations in most spaces need to have their
    /// data-flow traced.  For some of the special spaces, like the
    /// \e constant space, tracing data flow makes no sense, and this routine
    /// will return \b false.
    pub fn is_heritaged(&self) -> bool {
        (self.flags.get() & fl::heritaged) != 0
    }

    /// Return \b true if dead code analysis should be done on this space.
    pub fn does_deadcode(&self) -> bool {
        (self.flags.get() & fl::does_deadcode) != 0
    }

    /// Return \b true if the space has physical data in it.
    pub fn has_physical(&self) -> bool {
        (self.flags.get() & fl::hasphysical) != 0
    }

    /// Return \b true if values in this space are big endian.
    pub fn is_big_endian(&self) -> bool {
        (self.flags.get() & fl::big_endian) != 0
    }

    /// Return \b true if alignment justification does not match endianness.
    ///
    /// Certain architectures or compilers specify an alignment for accessing
    /// words within the space.  The space required for a variable must be
    /// rounded up to the alignment. For variables smaller than the
    /// alignment, there is the issue of how the variable is "justified"
    /// within the aligned word. Usually the justification depends on the
    /// endianness of the space; for certain weird cases the justification
    /// may be the opposite of the endianness.
    pub fn is_reverse_justified(&self) -> bool {
        (self.flags.get() & fl::reverse_justification) != 0
    }

    /// Return \b true if \b this is attached to the formal \b stack
    /// \b pointer.  Currently an architecture can declare only one formal
    /// stack pointer.
    pub fn is_formal_stack_space(&self) -> bool {
        (self.flags.get() & fl::formal_stackspace) != 0
    }

    /// Return \b true if this is an overlay space.
    pub fn is_overlay(&self) -> bool {
        (self.flags.get() & fl::overlay) != 0
    }

    /// Return \b true if other spaces overlay this space.
    pub fn is_overlay_base(&self) -> bool {
        (self.flags.get() & fl::overlaybase) != 0
    }

    /// Return \b true if \b this is the \e other address space.
    pub fn is_other_space(&self) -> bool {
        (self.flags.get() & fl::is_otherspace) != 0
    }

    /// Return \b true if this space is truncated from its original size.
    /// Pointers may refer to the original size but the most significant
    /// bytes are ignored.
    pub fn is_truncated(&self) -> bool {
        (self.flags.get() & fl::truncated) != 0
    }

    /// Return \b true if \e near (truncated) pointers into \b this space are
    /// possible.
    pub fn has_near_pointers(&self) -> bool {
        (self.flags.get() & fl::has_nearpointers) != 0
    }

    /// Write an address offset to a stream.  Print the \e offset as
    /// hexidecimal digits.
    pub fn print_offset(&self, s: &mut String, offset: u64) {
        s.push_str(&format!("0x{offset:x}"));
    }

    /// Number of base registers associated with this space.
    ///
    /// Some spaces are "virtual", like the stack spaces, where addresses are
    /// really relative to a base pointer stored in a register, like the
    /// stackpointer.  This routine will return non-zero if \b this space is
    /// virtual and there is 1 (or more) associated pointer registers.
    pub fn num_spacebase(&self) -> i32 {
        match &self.kind {
            // SpacebaseSpace::numSpacebase
            AddrSpaceKind::Spacebase { state, .. } => {
                if state.borrow().hasbaseregister {
                    1
                } else {
                    0
                }
            }
            _ => 0,
        }
    }

    /// Get a base register that creates this virtual space (C++
    /// `getSpacebase`, returning a `const VarnodeData &`; the Rust port
    /// returns a clone of the triple).
    pub fn get_spacebase(&self, i: i32) -> KunaResult<VarnodeStorage> {
        match &self.kind {
            // SpacebaseSpace::getSpacebase
            AddrSpaceKind::Spacebase { state, .. } => {
                let state = state.borrow();
                if !state.hasbaseregister || i != 0 {
                    return Err(KunaError::lowlevel(format!(
                        "No base register specified for space: {}",
                        self.get_name()
                    )));
                }
                Ok(state.baseloc.clone())
            }
            _ => Err(KunaError::lowlevel(format!(
                "{} space is not virtual and has no associated base register",
                self.get_name()
            ))),
        }
    }

    /// Return the original spacebase register before truncation (C++
    /// `getSpacebaseFull`; returns a clone of the triple).
    pub fn get_spacebase_full(&self, i: i32) -> KunaResult<VarnodeStorage> {
        match &self.kind {
            // SpacebaseSpace::getSpacebaseFull
            AddrSpaceKind::Spacebase { state, .. } => {
                let state = state.borrow();
                if !state.hasbaseregister || i != 0 {
                    return Err(KunaError::lowlevel(format!(
                        "No base register specified for space: {}",
                        self.get_name()
                    )));
                }
                Ok(state.base_orig.clone())
            }
            _ => Err(KunaError::lowlevel(format!(
                "{} has no truncated registers",
                self.get_name()
            ))),
        }
    }

    /// Set the base register at the time a `SpacebaseSpace` is created (C++
    /// private `SpacebaseSpace::setBaseRegister`, reached through the
    /// manager friend `addSpacebasePointer`).
    ///
    /// Errs if something tries to set two (different) base registers.
    /// \param data is the location data for the base register
    /// \param trunc_size is the size of the space covered by the register
    /// \param stack_growth is \b true if the stack which this register
    ///        manages grows in a negative direction
    pub(crate) fn set_base_register(
        &self,
        data: &VarnodeStorage,
        trunc_size: i32,
        stack_growth: bool,
    ) -> KunaResult<()> {
        let state = match &self.kind {
            AddrSpaceKind::Spacebase { state, .. } => state,
            // C++ cannot call this on a non-SpacebaseSpace (it is a private
            // member); reaching here is an internal invariant violation.
            _ => panic!("setBaseRegister on a non-spacebase space"),
        };
        let mut state = state.borrow_mut();
        if state.hasbaseregister
            && (state.baseloc != *data || state.is_negative_stack != stack_growth)
        {
            return Err(KunaError::lowlevel(format!(
                "Attempt to assign more than one base register to space: {}",
                self.get_name()
            )));
        }
        state.hasbaseregister = true;
        state.is_negative_stack = stack_growth;
        state.base_orig = data.clone();
        state.baseloc = data.clone();
        // C++ `truncSize != baseloc.size`: int4 converts up to uint4
        if trunc_size as u32 != state.baseloc.size {
            if state
                .baseloc
                .space
                .as_ref()
                .expect("setBaseRegister: null base register space (C++ UB)")
                .is_big_endian()
            {
                // baseloc.offset += (baseloc.size - truncSize): the
                // subtraction happens in uint4 (int4 converts up), the
                // result zero-extends into the uintb offset
                state.baseloc.offset = state
                    .baseloc
                    .offset
                    .wadd(u64::from(state.baseloc.size.wsub(trunc_size as u32)));
            }
            state.baseloc.size = trunc_size as u32; // cast: int4 -> uint4 member
        }
        Ok(())
    }

    /// Return \b true if a stack in this space grows negative.
    ///
    /// For stack (or other spacebase) spaces, this routine returns \b true
    /// if the space can viewed as a stack and a \b push operation causes the
    /// spacebase pointer to be decreased (grow negative).
    pub fn stack_grows_negative(&self) -> bool {
        match &self.kind {
            // SpacebaseSpace::stackGrowsNegative
            AddrSpaceKind::Spacebase { state, .. } => state.borrow().is_negative_stack,
            _ => true,
        }
    }

    /// Return this space's containing space (if any).
    ///
    /// If this space is virtual, then this routine returns the containing
    /// address space, otherwise it returns None.
    pub fn get_contain(&self) -> Option<&Rc<AddrSpace>> {
        match &self.kind {
            AddrSpaceKind::Overlay { base_space } => base_space.as_ref(),
            AddrSpaceKind::Spacebase { contain, .. } => contain.as_ref(),
            _ => None,
        }
    }

    /// The join-record table when \b this is the join space.
    fn join_state(&self) -> Option<&RefCell<JoinState>> {
        match &self.kind {
            AddrSpaceKind::Join { state } => Some(state),
            _ => None,
        }
    }

    /// Find the JoinRecord whose unified range starts exactly at \e offset
    /// within \b this (join) space (C++ `AddrSpaceManager::findJoin`, reached
    /// here through the join space's own `JoinState` rather than the C++
    /// `glb->findJoin` manager back-pointer — mirrors how [`overlap_join`]
    /// reaches the table).  Errors if \b this is not a join space or the offset
    /// is unlinked.
    pub fn find_join(&self, offset: u64) -> KunaResult<Rc<JoinRecord>> {
        match self.join_state() {
            Some(state) => state.borrow().find_join(offset),
            None => Err(KunaError::lowlevel("find_join on a non-join space")),
        }
    }

    /// \brief Determine if a given point is contained in an address range in
    /// \b this address space
    ///
    /// The point is specified as an address space and offset pair plus an
    /// additional number of bytes to "skip".  A non-negative value is
    /// returned if the point falls in the address range.  If the point falls
    /// on the first byte of the range, 0 is returned. For the second byte,
    /// 1 is returned, etc.  Otherwise -1 is returned.
    /// \param offset is the starting offset of the address range within \b this space
    /// \param size is the size of the address range in bytes
    /// \param point_space is the address space of the given point
    /// \param point_off is the offset of the given point
    /// \param point_skip is the additional bytes to skip
    pub fn overlap_join(
        &self,
        offset: u64,
        size: i32,
        point_space: &Rc<AddrSpace>,
        point_off: u64,
        point_skip: i32,
    ) -> KunaResult<i32> {
        match &self.kind {
            // ConstantSpace::overlapJoin always reports no overlap
            AddrSpaceKind::Constant => Ok(-1),
            // JoinSpace::overlapJoin
            AddrSpaceKind::Join { state } => {
                let (point_space, point_offset): (Option<Rc<AddrSpace>>, u64) =
                    if std::ptr::eq(self as *const AddrSpace, Rc::as_ptr(point_space)) {
                        // If the point is in the join space, translate the
                        // point into the piece address space
                        let piece_record = state.borrow().find_join(point_off)?;
                        let mut pos: i32 = 0;
                        // pointOffset + pointSkip: int4 sign-extends to uintb
                        let addr = piece_record.get_equivalent_address(
                            point_off.wadd(point_skip as i64 as u64),
                            &mut pos,
                        );
                        // (an invalid address leaves a null pointSpace in
                        // C++, which the pointer compares below tolerate)
                        (addr.get_space().cloned(), addr.get_offset())
                    } else {
                        if point_space.get_type() == spacetype::IPTR_CONSTANT {
                            return Ok(-1);
                        }
                        (
                            Some(Rc::clone(point_space)),
                            point_space.wrap_offset(point_off.wadd(point_skip as i64 as u64)),
                        )
                    };
                let join_record = state.borrow().find_join(offset)?;
                // Set up so we traverse pieces in data order
                let (start_piece, end_piece, dir): (i32, i32, i32) = if self.is_big_endian() {
                    (0, join_record.num_pieces(), 1)
                } else {
                    (join_record.num_pieces() - 1, -1, -1)
                };
                let mut bytes_accum: i32 = 0;
                let mut i = start_piece;
                while i != end_piece {
                    let v_data = join_record.get_piece(i);
                    // vData.offset + (vData.size-1): uint4 wrap zero-extends
                    // into the uintb add
                    if space_opt_ptr_eq(&v_data.space, &point_space)
                        && point_offset >= v_data.offset
                        && point_offset <= v_data.offset.wadd(u64::from(v_data.size.wsub(1)))
                    {
                        // (int4)(pointOffset - vData.offset) + bytesAccum
                        let res = ((point_offset.wsub(v_data.offset)) as i32).wadd(bytes_accum);
                        if res >= size {
                            return Ok(-1);
                        }
                        return Ok(res);
                    }
                    bytes_accum = bytes_accum.wadd(v_data.size as i32); // cast: int4 += uint4
                    i += dir;
                }
                Ok(-1)
            }
            _ => {
                if !std::ptr::eq(self as *const AddrSpace, Rc::as_ptr(point_space)) {
                    return Ok(-1);
                }
                // pointOff + pointSkip - offset: int4 pointSkip sign-extends
                // to uintb in the C++ arithmetic
                let dist =
                    self.wrap_offset(point_off.wadd(point_skip as i64 as u64).wsub(offset));
                // mixed comparison: uintb dist vs int4 size (size converted
                // up, sign-extended)
                if dist >= size as i64 as u64 {
                    return Ok(-1); // but must fall before op+size
                }
                Ok(dist as i32)
            }
        }
    }

    /// Encode address attributes to a stream.
    ///
    /// Write the main attributes for an address within \b this space.  The
    /// caller provides only the \e offset, and this routine fills in other
    /// details pertaining to this particular space.
    pub fn encode_attributes(&self, encoder: &mut dyn Encoder, offset: u64) -> KunaResult<()> {
        match &self.kind {
            // JoinSpace::encodeAttributes: encode a \e join address to the
            // stream.  This method in the interface only outputs attributes
            // for a single element, so we are forced to encode what should
            // probably be recursive elements into an attribute.
            AddrSpaceKind::Join { state } => {
                let rec = state.borrow().find_join(offset)?; // Record must already exist
                encoder.write_space(&ATTRIB_SPACE, self);
                let num = rec.num_pieces();
                if num > JoinSpace::MAX_PIECES {
                    return Err(KunaError::lowlevel(
                        "Exceeded maximum pieces in one join address",
                    ));
                }
                let mut i: i32 = 0;
                while i < num {
                    let vdata = rec.get_piece(i);
                    let spc = vdata
                        .space
                        .as_ref()
                        .expect("JoinSpace::encodeAttributes: null piece space (C++ UB)");
                    // ostringstream: space name, ":0x", hex offset, ':',
                    // decimal size
                    let t = format!("{}:0x{:x}:{}", spc.get_name(), vdata.offset, vdata.size);
                    encoder.write_string_indexed(&ATTRIB_PIECE, i as u32, t.as_bytes()); // cast: int4 loop index as the attribute index
                    i += 1;
                }
                if num == 1 {
                    encoder.write_unsigned_integer(
                        &ATTRIB_LOGICALSIZE,
                        u64::from(rec.get_unified().size),
                    );
                }
                Ok(())
            }
            // FspecSpace::encodeAttributes (unsized): if the callee entry is
            // invalid, emit space="fspec"; otherwise the entry's space + offset.
            AddrSpaceKind::Fspec => {
                let info = fspec_lookup(offset).ok_or_else(|| {
                    KunaError::lowlevel("FspecSpace::encodeAttributes: unregistered fspec handle")
                })?;
                if info.entry.is_invalid() {
                    encoder.write_string(&ATTRIB_SPACE, b"fspec");
                } else {
                    let id = info
                        .entry
                        .get_space()
                        .expect("FspecSpace::encodeAttributes: entry not invalid yet null space");
                    encoder.write_space(&ATTRIB_SPACE, id);
                    encoder.write_unsigned_integer(&ATTRIB_OFFSET, info.entry.get_offset());
                }
                Ok(())
            }
            AddrSpaceKind::Iop => {
                // IopSpace::encodeAttributes
                encoder.write_string(&ATTRIB_SPACE, b"iop");
                Ok(())
            }
            _ => {
                encoder.write_space(&ATTRIB_SPACE, self);
                encoder.write_unsigned_integer(&ATTRIB_OFFSET, offset);
                Ok(())
            }
        }
    }

    /// Encode an address and size attributes to a stream.
    ///
    /// Write the main attributes of an address with \b this space and a
    /// size. The caller provides the \e offset and \e size, and other
    /// details about this particular space are filled in.
    pub fn encode_attributes_sized(
        &self,
        encoder: &mut dyn Encoder,
        offset: u64,
        size: i32,
    ) -> KunaResult<()> {
        match &self.kind {
            // JoinSpace ignores the size and defers to the unsized variant
            AddrSpaceKind::Join { .. } => self.encode_attributes(encoder, offset),
            // FspecSpace::encodeAttributes (sized): same as the unsized arm but
            // also emits ATTRIB_SIZE on the valid-entry branch.
            AddrSpaceKind::Fspec => {
                let info = fspec_lookup(offset).ok_or_else(|| {
                    KunaError::lowlevel("FspecSpace::encodeAttributes: unregistered fspec handle")
                })?;
                if info.entry.is_invalid() {
                    encoder.write_string(&ATTRIB_SPACE, b"fspec");
                } else {
                    let id = info
                        .entry
                        .get_space()
                        .expect("FspecSpace::encodeAttributes: entry not invalid yet null space");
                    encoder.write_space(&ATTRIB_SPACE, id);
                    encoder.write_unsigned_integer(&ATTRIB_OFFSET, info.entry.get_offset());
                    encoder.write_signed_integer(&ATTRIB_SIZE, size as i64);
                }
                Ok(())
            }
            AddrSpaceKind::Iop => {
                // IopSpace::encodeAttributes
                encoder.write_string(&ATTRIB_SPACE, b"iop");
                Ok(())
            }
            _ => {
                encoder.write_space(&ATTRIB_SPACE, self);
                encoder.write_unsigned_integer(&ATTRIB_OFFSET, offset);
                encoder.write_signed_integer(&ATTRIB_SIZE, size as i64);
                Ok(())
            }
        }
    }

    /// Recover an offset and size.
    ///
    /// For an open element describing an address in \b this space, this
    /// routine recovers the offset and possibly the size described by the
    /// element.  `size` is only written when a size attribute is present
    /// (C++ fills a by-reference argument).
    pub fn decode_attributes(&self, decoder: &mut dyn Decoder, size: &mut u32) -> KunaResult<u64> {
        match &self.kind {
            // JoinSpace::decodeAttributes: parse the current element as a
            // join address.  Pieces of the join are encoded as a sequence of
            // ATTRIB_PIECE attributes; "piece1" corresponds to the most
            // significant piece.  `findAddJoin` is used to construct a
            // logical address within the join space.  (C++ reaches the
            // manager through the `manage` back-pointer; the decoder carries
            // the same manager here.)
            AddrSpaceKind::Join { .. } => {
                let mut pieces: Vec<VarnodeStorage> = Vec::new();
                // C++ accumulates `sizesum` but never reads it; kept for the
                // line-against-line review
                let mut _sizesum: u32 = 0;
                let mut logicalsize: u32 = 0;
                loop {
                    let mut attrib_id = decoder.get_next_attribute_id()?;
                    if attrib_id == 0 {
                        break;
                    }
                    if attrib_id == ATTRIB_LOGICALSIZE {
                        logicalsize = decoder.read_unsigned_integer()? as u32; // cast: uintb -> uint4 member
                        continue;
                    } else if attrib_id == ATTRIB_UNKNOWN {
                        attrib_id = decoder.get_indexed_attribute_id(&ATTRIB_PIECE)?;
                    }
                    if attrib_id < ATTRIB_PIECE.get_id() {
                        continue;
                    }
                    // (int4)(attribId - ATTRIB_PIECE.getId()): uint4 wrap,
                    // then truncate to int4
                    let pos = attrib_id.wsub(ATTRIB_PIECE.get_id()) as i32;
                    if pos > JoinSpace::MAX_PIECES {
                        continue;
                    }
                    while pieces.len() <= pos as usize {
                        // cast: int4 -> index, non-negative here
                        pieces.push(VarnodeStorage::default());
                    }
                    let attr_val = decoder.read_string()?;
                    let offpos = attr_val.iter().position(|&c| c == b':');
                    let vdat: VarnodeStorage = match offpos {
                        None => {
                            // Register-name piece: C++
                            // `getTrans()->getRegister(attrVal)`
                            let lookup = decoder
                                .get_addr_space_manager()
                                .register_lookup()
                                .cloned()
                                .ok_or_else(no_register_lookup_err)?;
                            lookup.get_register(&String::from_utf8_lossy(&attr_val))?
                        }
                        Some(offpos) => {
                            let szpos = attr_val[offpos + 1..]
                                .iter()
                                .position(|&c| c == b':')
                                .map(|p| p + offpos + 1);
                            let szpos = match szpos {
                                Some(szpos) => szpos,
                                None => {
                                    return Err(KunaError::lowlevel(
                                        "join address piece attribute is malformed",
                                    ))
                                }
                            };
                            let spcname = String::from_utf8_lossy(&attr_val[..offpos]);
                            // (C++ silently stores a null space pointer for
                            // an unknown name; comparing such a piece later
                            // is UB there and panics here)
                            let space = decoder
                                .get_addr_space_manager()
                                .get_space_by_name(&spcname)
                                .cloned();
                            // istringstream with unsetf(dec|hex|oct):
                            // base-0 detection, like strtoul
                            let offset = cxx_strtoul(&attr_val[offpos + 1..]).0;
                            // extraction into a uint4 saturates on overflow
                            // (num_get stores UINT_MAX)
                            let size64 = cxx_strtoul(&attr_val[szpos + 1..]).0;
                            let size = if size64 > u64::from(u32::MAX) {
                                u32::MAX
                            } else {
                                size64 as u32 // cast: checked above
                            };
                            VarnodeStorage { space, offset, size }
                        }
                    };
                    _sizesum = _sizesum.wadd(vdat.size);
                    pieces[pos as usize] = vdat; // cast: int4 -> index
                }
                let rec = decoder
                    .get_addr_space_manager()
                    .find_add_join(&pieces, logicalsize)?;
                *size = rec.get_unified().size;
                Ok(rec.get_unified().offset)
            }
            _ => {
                let mut offset: u64 = 0;
                let mut foundoffset = false;
                loop {
                    let attrib_id = decoder.get_next_attribute_id()?;
                    if attrib_id == 0 {
                        break;
                    }
                    if attrib_id == ATTRIB_OFFSET {
                        foundoffset = true;
                        offset = decoder.read_unsigned_integer()?;
                    } else if attrib_id == ATTRIB_SIZE {
                        // intb -> uint4 truncating conversion as in C++
                        *size = decoder.read_signed_integer()? as u32;
                    }
                }
                if !foundoffset {
                    return Err(KunaError::lowlevel("Address is missing offset"));
                }
                Ok(offset)
            }
        }
    }

    /// Write an address in this space to a stream.
    ///
    /// This is a printing method for the debugging routines. It prints
    /// taking into account the \e wordsize, adding a "+n" if the offset is
    /// not on-cut with wordsize. It also pads to the expected/typical size
    /// of values from this space.
    pub fn print_raw(&self, s: &mut String, offset: u64) -> KunaResult<()> {
        match &self.kind {
            // Constants are always printed as hexidecimal values in the
            // debugger and console dumps; OtherSpace prints the same way.
            AddrSpaceKind::Constant | AddrSpaceKind::Other => {
                s.push_str(&format!("0x{offset:x}"));
                Ok(())
            }
            // JoinSpace::printRaw
            AddrSpaceKind::Join { state } => {
                let rec = state.borrow().find_join(offset)?;
                let mut szsum: i32 = 0;
                let num = rec.num_pieces();
                s.push('{');
                let mut i: i32 = 0;
                while i < num {
                    let vdat = rec.get_piece(i);
                    szsum = szsum.wadd(vdat.size as i32); // cast: int4 += uint4
                    if i != 0 {
                        s.push(',');
                    }
                    vdat.space
                        .as_ref()
                        .expect("JoinSpace::printRaw: null piece space (C++ UB)")
                        .print_raw(s, vdat.offset)?;
                    i += 1;
                }
                if num == 1 {
                    szsum = rec.get_unified().size as i32; // cast: uint4 -> int4
                    s.push(':');
                    s.push_str(&szsum.to_string());
                }
                s.push('}');
                Ok(())
            }
            // FspecSpace::printRaw: the offset is a registered call-spec
            // handle; the call-spec layer has already resolved the display
            // name (the C++ name/`func_`/`sub_` branch, decided where the
            // `Architecture` is visible).
            AddrSpaceKind::Fspec => match fspec_lookup(offset) {
                Some(info) => {
                    s.push_str(&info.printed_name);
                    Ok(())
                }
                None => Err(KunaError::lowlevel(
                    "FspecSpace::printRaw: unregistered fspec handle",
                )),
            },
            AddrSpaceKind::Iop => Err(KunaError::lowlevel(
                "kuna rust port: IopSpace::printRaw requires PcodeOp (op wave)",
            )),
            _ => {
                let mut sz = self.get_addr_size() as i32;
                if sz > 4 {
                    if (offset >> 32) == 0 {
                        sz = 4; // Don't print a bunch of zeroes at front of address
                    } else if (offset >> 48) == 0 {
                        sz = 6;
                    }
                }
                s.push_str(&format!(
                    "0x{:0width$x}",
                    Self::byte_to_address(offset, self.wordsize),
                    width = (2 * sz) as usize
                ));
                if self.wordsize > 1 {
                    // int4 cut = offset % wordsize (truncating cast)
                    let cut = (offset % self.wordsize as u64) as i32;
                    if cut != 0 {
                        s.push_str(&format!("+{cut}"));
                    }
                }
                Ok(())
            }
        }
    }

    /// Read in an address (and possible size) from a string.
    ///
    /// For the console mode, an address space can tailor how it converts
    /// user strings into offsets within the space. The base routine can read
    /// and convert register names as well as absolute hex addresses.  A size
    /// can be indicated by appending a ':' and integer, i.e.  0x1000:2.
    /// Offsets within a register can be indicated by appending a '+' and
    /// integer, i.e. eax+2.
    ///
    /// (kuna rust) The C++ `trans`/`manage` back-pointers become the
    /// explicit `manage` parameter; the register-name path consults the
    /// manager's installed [`RegisterLookup`].  With none installed,
    /// `getRegister` behaves as always-throwing — identical to the C++
    /// behavior for an *unknown* register — and only the catch branch
    /// (absolute offset parsing) is live.  The C++ try/catch is a
    /// *speculative* question ("is this token a register name?"), so it goes
    /// through [`RegisterLookup::probe_register`].  That probe collapses every
    /// failure to `None`, where the exact lookup used to re-raise a
    /// non-`LowlevelError`: no lookup in the tree returns one, and the only
    /// caller is the console's address parser, where a swallowed one resurfaces
    /// as a hex-parse rejection of the same token.
    pub fn read(&self, s: &str, size: &mut i32, manage: &AddrSpaceManager) -> KunaResult<u64> {
        // JoinSpace::read: parse a comma-separated sequence of register
        // names / shortcut-prefixed offsets into a join record.
        if let AddrSpaceKind::Join { .. } = self.kind {
            let bytes = s.as_bytes();
            let mut pieces: Vec<VarnodeStorage> = Vec::new();
            let mut szsum: i32 = 0;
            let mut i: usize = 0;
            while i < bytes.len() {
                let mut token: Vec<u8> = Vec::new();
                while i < bytes.len() && bytes[i] != b',' {
                    token.push(bytes[i]);
                    i += 1;
                }
                i += 1; // Skip the comma
                // try { getRegister(token) } catch(LowlevelError): name
                // doesn't exist (no installed lookup == always throwing)
                let piece: Option<VarnodeStorage> = manage
                    .register_lookup()
                    .and_then(|lookup| lookup.probe_register(&String::from_utf8_lossy(&token)));
                let piece = match piece {
                    Some(piece) => piece,
                    None => {
                        // (an empty token reads '\0' through C++'s
                        // operator[](size()), which never names a space)
                        let try_shortcut = token.first().copied().unwrap_or(0);
                        let spc = match manage.get_space_by_shortcut(try_shortcut) {
                            Some(spc) => Rc::clone(spc),
                            None => {
                                return Err(KunaError::lowlevel("Could not parse join string"))
                            }
                        };
                        let mut subsize: i32 = 0;
                        let offset = spc.read(
                            &String::from_utf8_lossy(&token[1..]),
                            &mut subsize,
                            manage,
                        )?;
                        VarnodeStorage {
                            space: Some(spc),
                            offset,
                            size: subsize as u32, // cast: int4 -> uint4 member
                        }
                    }
                };
                szsum = szsum.wadd(piece.size as i32); // cast: int4 += uint4
                pieces.push(piece);
            }
            let rec = manage.find_add_join(&pieces, 0)?;
            *size = szsum;
            return Ok(rec.get_unified().offset);
        }
        let bytes = s.as_bytes();
        let append = bytes.iter().position(|&c| c == b':' || c == b'+');
        // try { getRegister } catch(LowlevelError) { absolute offset }
        let point: Option<VarnodeStorage> = manage.register_lookup().and_then(|lookup| {
            let name = match append {
                None => s,
                Some(append) => &s[..append],
            };
            lookup.probe_register(name)
        });
        let mut offset: u64;
        match point {
            Some(point) => {
                offset = point.offset;
                *size = point.size as i32; // cast: uint4 -> int4
            }
            None => {
                // catch(LowlevelError): name doesn't exist
                let (raw, consumed) = cxx_strtoul(bytes);
                offset = Self::address_to_byte(raw, self.wordsize);
                if consumed == bytes.len() {
                    // If no size or offset override: return "natural" size
                    *size = manage.get_default_size();
                    return Ok(offset);
                }
                *size = manage.get_default_size();
            }
        }
        if let Some(append) = append {
            let expsize = get_offset_size(&bytes[append..], &mut offset);
            if expsize != -1 {
                *size = expsize;
                return Ok(offset);
            }
        }
        Ok(offset)
    }

    /// Walk attributes of the current element and recover all the properties
    /// defining this space.  The \e type must already be filled in.
    /// (C++ protected `decodeBasicAttributes`.)
    #[allow(clippy::collapsible_if)] // nested ifs transcribe the C++ shape
    fn decode_basic_attributes(&mut self, decoder: &mut dyn Decoder) -> KunaResult<()> {
        self.deadcodedelay.set(-1);
        loop {
            let attrib_id = decoder.get_next_attribute_id()?;
            if attrib_id == 0 {
                break;
            }
            if attrib_id == ATTRIB_NAME {
                // space names are ASCII in practice; lossy conversion keeps
                // the byte-string -> String boundary explicit
                self.name = String::from_utf8_lossy(&decoder.read_string()?).into_owned();
            }
            // NOTE: the C++ checks ATTRIB_INDEX in a fresh `if` (not an
            // `else if` chained to the NAME check); transcribed as-is.
            if attrib_id == ATTRIB_INDEX {
                self.index = decoder.read_signed_integer()? as i32;
            } else if attrib_id == ATTRIB_SIZE {
                // intb -> uint4 truncating conversion
                self.address_size.set(decoder.read_signed_integer()? as u32);
            } else if attrib_id == ATTRIB_WORDSIZE {
                // uintb -> uint4 truncating conversion
                self.wordsize = decoder.read_unsigned_integer()? as u32;
            } else if attrib_id == ATTRIB_BIGENDIAN {
                if decoder.read_bool()? {
                    self.flags.set(self.flags.get() | fl::big_endian);
                }
            } else if attrib_id == ATTRIB_DELAY {
                self.delay = decoder.read_signed_integer()? as i32;
            } else if attrib_id == ATTRIB_DEADCODEDELAY {
                self.deadcodedelay.set(decoder.read_signed_integer()? as i32);
            } else if attrib_id == ATTRIB_PHYSICAL {
                if decoder.read_bool()? {
                    self.flags.set(self.flags.get() | fl::hasphysical);
                }
            }
        }
        if self.deadcodedelay.get() == -1 {
            // If deadcodedelay attribute not present, set it to delay
            self.deadcodedelay.set(self.delay);
        }
        self.calc_scale_mask();
        Ok(())
    }

    /// Recover the details of this space from a stream (the C++ virtual
    /// `decode`, dispatching to the subclass overrides).
    pub fn decode(&mut self, decoder: &mut dyn Decoder) -> KunaResult<()> {
        match &self.kind {
            // As the ConstantSpace is never saved, it should never get
            // decoded either.
            AddrSpaceKind::Constant => {
                Err(KunaError::lowlevel("Should never decode the constant space"))
            }
            AddrSpaceKind::Join { .. } => {
                Err(KunaError::lowlevel("Should never decode join space"))
            }
            AddrSpaceKind::Fspec => {
                Err(KunaError::lowlevel("Should never decode fspec space from stream"))
            }
            AddrSpaceKind::Iop => {
                Err(KunaError::lowlevel("Should never decode iop space from stream"))
            }
            AddrSpaceKind::Overlay { .. } => self.decode_overlay(decoder),
            AddrSpaceKind::Spacebase { .. } => self.decode_spacebase(decoder),
            _ => {
                // Multiple tags: <space>, <space_other>, <space_unique>
                let elem_id = decoder.open_element()?;
                self.decode_basic_attributes(decoder)?;
                decoder.close_element(elem_id)
            }
        }
    }

    /// `SpacebaseSpace::decode` (translate.cc)
    fn decode_spacebase(&mut self, decoder: &mut dyn Decoder) -> KunaResult<()> {
        let elem_id = decoder.open_element_id(&ELEM_SPACE_BASE)?;
        self.decode_basic_attributes(decoder)?;
        let new_contain = decoder.read_space_id(&ATTRIB_CONTAIN)?;
        decoder.close_element(elem_id)?;
        match &mut self.kind {
            AddrSpaceKind::Spacebase { contain, .. } => *contain = Some(new_contain),
            _ => unreachable!("decode_spacebase dispatched on a spacebase kind"),
        }
        Ok(())
    }

    /// `OverlaySpace::decode`
    fn decode_overlay(&mut self, decoder: &mut dyn Decoder) -> KunaResult<()> {
        let elem_id = decoder.open_element_id(&ELEM_SPACE_OVERLAY)?;
        self.name = String::from_utf8_lossy(&decoder.read_string_id(&ATTRIB_NAME)?).into_owned();
        self.index = decoder.read_signed_integer_id(&ATTRIB_INDEX)? as i32;

        let base_space = decoder.read_space_id(&ATTRIB_BASE)?;
        decoder.close_element(elem_id)?;
        self.address_size.set(base_space.get_addr_size());
        self.wordsize = base_space.get_word_size();
        self.delay = base_space.get_delay();
        self.deadcodedelay.set(base_space.get_deadcode_delay());
        self.calc_scale_mask();

        if base_space.is_big_endian() {
            self.set_flags(fl::big_endian);
        }
        if base_space.has_physical() {
            self.set_flags(fl::hasphysical);
        }
        self.kind = AddrSpaceKind::Overlay { base_space: Some(base_space) };
        Ok(())
    }

    /// Scale from addressable units to byte units.
    ///
    /// Given an offset into an address space based on the addressable unit
    /// size (wordsize), convert it into a byte relative offset.
    pub fn address_to_byte(val: u64, ws: u32) -> u64 {
        val.wmul(ws as u64)
    }

    /// Scale from byte units to addressable units.
    pub fn byte_to_address(val: u64, ws: u32) -> u64 {
        val / ws as u64
    }

    /// Scale int8 from addressable units to byte units.
    pub fn address_to_byte_int(val: i64, ws: u32) -> i64 {
        val.wmul(ws as i64)
    }

    /// Scale int8 from byte units to addressable units.
    pub fn byte_to_address_int(val: i64, ws: u32) -> i64 {
        val / ws as i64
    }

    /// Compare two spaces by their index, for sorting a sequence of address
    /// spaces.  Returns \b true if the first space should come before the
    /// second.
    pub fn compare_by_index(a: &AddrSpace, b: &AddrSpace) -> bool {
        a.index < b.index
    }
}

/// Get optional size and offset fields from string (C++ static
/// `get_offset_size` in space.cc)
fn get_offset_size(ptr: &[u8], offset: &mut u64) -> i32 {
    let mut val: u32 = 0; // Defaults
    let mut size: i32 = -1;
    if !ptr.is_empty() && ptr[0] == b':' {
        let (sz, consumed) = cxx_strtoul(&ptr[1..]);
        size = sz as i32; // unsigned long -> int4 truncation
        let rest = &ptr[1 + consumed..];
        if !rest.is_empty() && rest[0] == b'+' {
            val = cxx_strtoul(&rest[1..]).0 as u32;
        }
    }
    if !ptr.is_empty() && ptr[0] == b'+' {
        val = cxx_strtoul(&ptr[1..]).0 as u32;
    }
    *offset = offset.wadd(val as u64); // Adjust offset
    size
}

// ---------------------------------------------------------------------------
// AddrSpace subclasses (constructor types)
// ---------------------------------------------------------------------------

/// \brief Special AddrSpace for representing constants during analysis.
///
/// The underlying RTL (See PcodeOp) represents all data in terms of an
/// Address, which is made up of an AddrSpace and offset pair.  In order to
/// represent constants in the semantics of the RTL, there is a special
/// \e constant address space.  An \e offset within the address space encodes
/// the actual constant represented by the pair.  I.e. the pair (\b const,4)
/// represents the constant \b 4 within the RTL.  The \e size of the
/// ConstantSpace has no meaning, as we always want to be able to represent
/// an arbitrarily large constant.  In practice, the size of a constant is
/// limited by the offset field of an Address.
pub struct ConstantSpace;

impl ConstantSpace {
    /// Reserved name for the address space
    pub const NAME: &'static str = "const";
    /// Reserved index for constant space
    pub const INDEX: i32 = 0;

    /// Only constructor.  This constructs the unique constant space.  By
    /// convention, the name is always "const" and the index is always 0.
    #[allow(clippy::new_ret_no_self)] // C++ subclass constructor
    pub fn new() -> AddrSpace {
        let mut space = AddrSpace::new(
            spacetype::IPTR_CONSTANT,
            Self::NAME,
            false,
            8, // sizeof(uintb)
            1,
            Self::INDEX,
            0,
            0,
            0,
        );
        space.kind = AddrSpaceKind::Constant;
        space.clear_flags(fl::heritaged | fl::does_deadcode | fl::big_endian);
        if HOST_ENDIAN == 1 {
            // Endianness always matches host
            space.set_flags(fl::big_endian);
        }
        space
    }
}

/// \brief Special AddrSpace for special/user-defined address spaces
pub struct OtherSpace;

impl OtherSpace {
    /// Reserved name for the address space
    pub const NAME: &'static str = "OTHER";
    /// Reserved index for the other space
    pub const INDEX: i32 = 1;

    /// Constructor.  Construct the \b other space, which is automatically
    /// constructed by the compiler, and is only constructed once.  The name
    /// should always be \b OTHER.
    #[allow(clippy::new_ret_no_self)] // C++ subclass constructor
    pub fn new(_ind: i32) -> AddrSpace {
        // C++ passes `ind` but the base constructor receives INDEX
        let mut space = AddrSpace::new(
            spacetype::IPTR_PROCESSOR,
            Self::NAME,
            false,
            8, // sizeof(uintb)
            1,
            Self::INDEX,
            0,
            0,
            0,
        );
        space.kind = AddrSpaceKind::Other;
        space.clear_flags(fl::heritaged | fl::does_deadcode);
        space.set_flags(fl::is_otherspace);
        space
    }

    /// For use with decode
    pub fn new_for_decode() -> AddrSpace {
        let mut space = AddrSpace::new_for_decode(spacetype::IPTR_PROCESSOR);
        space.kind = AddrSpaceKind::Other;
        space.clear_flags(fl::heritaged | fl::does_deadcode);
        space.set_flags(fl::is_otherspace);
        space
    }
}

/// \brief The pool of temporary storage registers
///
/// It is convenient both for modelling processor instructions in an RTL and
/// for later transforming of the RTL to have a pool of temporary registers
/// that can hold data but that aren't a formal part of the state of the
/// processor. The UniqueSpace provides a specific location for this pool.
/// The analysis engine always creates exactly one of these spaces named
/// \b unique.
pub struct UniqueSpace;

impl UniqueSpace {
    /// Reserved name for the unique space
    pub const NAME: &'static str = "unique";
    /// Fixed size (in bytes) for unique space offsets
    pub const SIZE: u32 = 4;

    /// Constructor.  This is the constructor for the \b unique space, which
    /// is automatically constructed by the analysis engine, and constructed
    /// only once.  The name should always be \b unique.
    ///
    /// `big_end` replaces the C++ `t->isBigEndian()` consultation of the
    /// (unported) Translate back-pointer.
    /// \param ind is the integer identifier
    /// \param flags are attribute flags (currently unused)
    #[allow(clippy::new_ret_no_self)] // C++ subclass constructor
    pub fn new(ind: i32, flags: u32, big_end: bool) -> AddrSpace {
        let mut space = AddrSpace::new(
            spacetype::IPTR_INTERNAL,
            Self::NAME,
            big_end,
            Self::SIZE,
            1,
            ind,
            flags,
            0,
            0,
        );
        space.kind = AddrSpaceKind::Unique;
        space.set_flags(fl::hasphysical);
        space
    }

    /// For use with decode
    pub fn new_for_decode() -> AddrSpace {
        let mut space = AddrSpace::new_for_decode(spacetype::IPTR_INTERNAL);
        space.kind = AddrSpaceKind::Unique;
        space.set_flags(fl::hasphysical);
        space
    }
}

/// \brief The pool of logically joined variables
///
/// Some logical variables are split across non-contiguous regions of memory.
/// This space creates a virtual place for these logical variables to exist.
/// Any memory location within this space is backed by 2 or more memory
/// locations in other spaces that physically hold the pieces of the logical
/// value. The database controlling symbols is responsible for keeping track
/// of mapping the logical address in this space to its physical pieces.
/// Offsets into this space do not have an absolute meaning; the database may
/// vary what offset is assigned to what set of pieces.
pub struct JoinSpace;

impl JoinSpace {
    /// Reserved name for the join space
    pub const NAME: &'static str = "join";
    /// Maximum number of pieces that can be marshaled in one \e join address
    /// (C++ private MAX_PIECES)
    pub(crate) const MAX_PIECES: i32 = 64;

    /// Constructor.  This is the constructor for the \b join space, which is
    /// automatically constructed by the analysis engine, and constructed
    /// only once. The name should always be \b join.
    ///
    /// `big_end` replaces the C++ `t->isBigEndian()` consultation of the
    /// (unported) Translate back-pointer.
    /// \param ind is the integer identifier
    #[allow(clippy::new_ret_no_self)] // C++ subclass constructor
    pub fn new(ind: i32, big_end: bool) -> AddrSpace {
        let mut space = AddrSpace::new(
            spacetype::IPTR_JOIN,
            Self::NAME,
            big_end,
            4, // sizeof(uintm)
            1,
            ind,
            0,
            0,
            0,
        );
        space.kind = AddrSpaceKind::Join { state: RefCell::new(JoinState::default()) };
        // This is a virtual space; never heritaged, but does dead-code
        // analysis
        space.clear_flags(fl::heritaged);
        space
    }
}

/// \brief A virtual space \e stack space (translate.hh `SpacebaseSpace`)
///
/// In a lot of analysis situations it is convenient to extend the notion of
/// an address space to mean bytes that are indexed relative to some base
/// register.  The canonical example of this is the \b stack space, which
/// models the concept of local variables stored on the stack.  An address of
/// (\b stack, 8) might model the address of a function parameter on the
/// stack for instance, and (\b stack, 0xfffffff4) might be the address of a
/// local variable.  A space like this is inherently \e virtual and contained
/// within whatever space is being indexed into.
pub struct SpacebaseSpace;

impl SpacebaseSpace {
    /// Construct a virtual space.  This is usually used for the stack space,
    /// which is indicated by the \b is_formal parameter, but multiple such
    /// spaces are allowed.
    ///
    /// `big_end` replaces the C++ `t->isBigEndian()` consultation of the
    /// (unstored) Translate back-pointer.
    /// \param nm is the name of the space
    /// \param ind is the integer identifier
    /// \param sz is the size of the space
    /// \param base is the containing space
    /// \param dl is the heritage delay
    /// \param is_formal is the formal stack space indicator
    #[allow(clippy::new_ret_no_self)] // C++ subclass constructor
    pub fn new(
        nm: &str,
        ind: i32,
        sz: u32,
        base: &Rc<AddrSpace>,
        dl: i32,
        is_formal: bool,
        big_end: bool,
    ) -> AddrSpace {
        let mut space = AddrSpace::new(
            spacetype::IPTR_SPACEBASE,
            nm,
            big_end,
            sz,
            base.get_word_size(),
            ind,
            0,
            dl,
            dl,
        );
        space.kind = AddrSpaceKind::Spacebase {
            contain: Some(Rc::clone(base)),
            state: RefCell::new(SpacebaseState {
                hasbaseregister: false, // No base register assigned yet
                is_negative_stack: true, // default stack growth
                baseloc: VarnodeStorage::default(),
                base_orig: VarnodeStorage::default(),
            }),
        };
        if is_formal {
            space.set_flags(fl::formal_stackspace);
        }
        space
    }

    /// For use with decode.  This is a partial constructor, which must be
    /// followed up with decode in order to fill in the rest of the space's
    /// attributes.
    pub fn new_for_decode() -> AddrSpace {
        let mut space = AddrSpace::new_for_decode(spacetype::IPTR_SPACEBASE);
        space.kind = AddrSpaceKind::Spacebase {
            contain: None,
            state: RefCell::new(SpacebaseState {
                hasbaseregister: false,
                is_negative_stack: true,
                baseloc: VarnodeStorage::default(),
                base_orig: VarnodeStorage::default(),
            }),
        };
        space.set_flags(fl::programspecific);
        space
    }
}

/// \brief An overlay space.
///
/// A different code and data layout that occupies the same memory as another
/// address space.  Some compilers use this concept to increase the logical
/// size of a program without increasing its physical memory requirements.
/// An overlay space allows the same physical location to contain different
/// code and be labeled with different symbols, depending on context.  From
/// the point of view of reverse engineering, the different code and symbols
/// are viewed as a logically distinct space.
pub struct OverlaySpace;

impl OverlaySpace {
    /// C++ has only the decode form (no explicit constructor).
    #[allow(clippy::new_ret_no_self)] // C++ subclass constructor
    pub fn new() -> AddrSpace {
        let mut space = AddrSpace::new_for_decode(spacetype::IPTR_PROCESSOR);
        space.kind = AddrSpaceKind::Overlay { base_space: None };
        space.set_flags(fl::overlay);
        space
    }
}

/// \brief A special space for encoding FuncCallSpecs (declared in fspec.hh)
///
/// It is efficient and convenient to store the main subfunction object
/// (FuncCallSpecs) in the pcode operation which is actually making the call.
/// This address space allows a FuncCallSpecs to be encoded as an address
/// which replaces the formally encoded address of the function being called,
/// when manipulating the operation internally.  The space stored in the
/// encoded address is this special \b fspec space, and the offset is the
/// actual value of the pointer.
pub struct FspecSpace;

impl FspecSpace {
    /// Reserved name for the fspec space
    pub const NAME: &'static str = "fspec";

    /// Constructor for the \b fspec space.  There is only one such space,
    /// and it is considered internal to the model, i.e. the Translate engine
    /// should never generate addresses in this space.
    /// \param ind is the index associated with the space
    #[allow(clippy::new_ret_no_self)] // C++ subclass constructor
    pub fn new(ind: i32) -> AddrSpace {
        let mut space = AddrSpace::new(
            spacetype::IPTR_FSPEC,
            Self::NAME,
            false,
            core::mem::size_of::<usize>() as u32, // sizeof(void *)
            1,
            ind,
            0,
            1,
            1,
        );
        space.kind = AddrSpaceKind::Fspec;
        space.clear_flags(fl::heritaged | fl::does_deadcode | fl::big_endian);
        if HOST_ENDIAN == 1 {
            // Endianness always set by host
            space.set_flags(fl::big_endian);
        }
        space
    }
}

// -----------------------------------------------------------------------------
// FspecSpace call-spec registry (kuna rust port)
//
// In C++ the offset of an \e fspec address *is* the raw `FuncCallSpecs *`, so
// `FspecSpace::printRaw`/`encodeAttributes` simply cast the offset back to a
// `FuncCallSpecs *` and read its name / entry address.  kuna-base sits below
// kuna-decomp (where `FuncCallSpecs` lives), so it cannot hold that pointer.
// Instead the call-spec layer registers the small slice of state these two
// arms read ([`FspecCallInfo`]) under an integer handle, and the handle is the
// offset of the \e fspec address (`Funcdata::newVarnodeCallSpecs` takes the
// same handle).  This is the faithful equivalent of the pointer cast: the arms
// recover exactly the fields `FspecSpace::printRaw`/`encodeAttributes` read.
// -----------------------------------------------------------------------------

/// The slice of `FuncCallSpecs` state that `FspecSpace::printRaw` /
/// `encodeAttributes` read (C++ `fc->getName()` and `fc->getEntryAddress()`).
#[derive(Clone, Debug)]
pub struct FspecCallInfo {
    /// The display name to print (already resolved by the call-spec layer,
    /// which owns the naming policy — C++ `printRaw`'s name/`func_`/`sub_`
    /// branch is decided where the `Architecture` is visible).
    pub printed_name: String,
    /// The callee entry address (C++ `fc->getEntryAddress()`); invalid (no
    /// space) selects the `writeString(ATTRIB_SPACE,"fspec")` branch.
    pub entry: Address,
}

thread_local! {
    /// Handle -> call-spec info side table.  Process(thread)-local because the
    /// C++ offset is a process pointer; the call-spec layer owns the lifetime.
    static FSPEC_REGISTRY: RefCell<BTreeMap<u64, FspecCallInfo>> =
        const { RefCell::new(BTreeMap::new()) };
}

/// Register the call-spec state for an \e fspec handle (the offset of the
/// \e fspec address).  Called by the call-spec layer when it materializes a
/// `FuncCallSpecs` annotation Varnode.
pub fn fspec_register(handle: u64, info: FspecCallInfo) {
    FSPEC_REGISTRY.with(|r| {
        r.borrow_mut().insert(handle, info);
    });
}

/// Drop a registered \e fspec handle.
pub fn fspec_unregister(handle: u64) {
    FSPEC_REGISTRY.with(|r| {
        r.borrow_mut().remove(&handle);
    });
}

/// Look up the call-spec info for an \e fspec handle, if registered.
pub fn fspec_lookup(handle: u64) -> Option<FspecCallInfo> {
    FSPEC_REGISTRY.with(|r| r.borrow().get(&handle).cloned())
}

/// \brief Space for storing internal PcodeOp pointers as addresses (declared
/// in op.hh)
///
/// It is convenient and efficient to replace the formally encoded branch
/// target addresses with a pointer to the actual PcodeOp being branched to.
/// This special \b iop space allows a PcodeOp pointer to be encoded as an
/// address so it can be stored as part of an input varnode, in place of the
/// target address, in a \e branching operation.
pub struct IopSpace;

impl IopSpace {
    /// Reserved name for the iop space
    pub const NAME: &'static str = "iop";

    /// Constructor
    #[allow(clippy::new_ret_no_self)] // C++ subclass constructor
    pub fn new(ind: i32) -> AddrSpace {
        let mut space = AddrSpace::new(
            spacetype::IPTR_IOP,
            Self::NAME,
            false,
            core::mem::size_of::<usize>() as u32, // sizeof(void *)
            1,
            ind,
            0,
            1,
            1,
        );
        space.kind = AddrSpaceKind::Iop;
        space.clear_flags(fl::heritaged | fl::does_deadcode | fl::big_endian);
        if HOST_ENDIAN == 1 {
            // Endianness always set to host
            space.set_flags(fl::big_endian);
        }
        space
    }
}

// ---------------------------------------------------------------------------
// AddrSpaceManager (minimal lookup core from translate.hh/.cc)
// ---------------------------------------------------------------------------

/// The C++ `vector<AddressResolver *> resolvelist` (owned trait objects; the
/// newtype keeps `AddrSpaceManager`'s `Debug`/`Default` derives).
#[derive(Default)]
struct ResolverList(Vec<Option<Box<dyn AddressResolver>>>);

impl fmt::Debug for ResolverList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let set: Vec<bool> = self.0.iter().map(Option::is_some).collect();
        f.debug_tuple("ResolverList").field(&set).finish()
    }
}

/// The installed [`RegisterLookup`] (the register half of the C++
/// `AddrSpace::trans` back-pointer; the newtype keeps the manager's
/// `Debug`/`Default` derives).
#[derive(Default)]
struct RegisterLookupSlot(Option<Rc<dyn RegisterLookup>>);

impl fmt::Debug for RegisterLookupSlot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("RegisterLookupSlot").field(&self.0.is_some()).finish()
    }
}

/// \brief A manager for different address spaces
///
/// Allows creation, lookup by name, lookup by shortcut and iteration over
/// address spaces.
///
/// Port of the C++ `AddrSpaceManager` (translate.hh/.cc): the W1 core
/// (registration — `insert_space`, shortcut assignment, the default-space
/// setters — and lookup by name, shortcut and index, plus the special-space
/// accessors needed by `Decoder::readSpace`/`Encoder::writeSpace`) plus the
/// W2 translate-item extensions: the join-record machinery (`find_add_join`,
/// `find_join`, `renormalize_join_address`, ...), the resolver list, and
/// `decode_space`/`decode_spaces`.  The C++ `joinallocate`/`splitset`/
/// `splitlist` members live inside the join space's kind ([`JoinState`]) so
/// the `JoinSpace` virtuals can reach them without a back-pointer; since a
/// join space belongs to exactly one record-creating manager in practice
/// (the C++ `copySpaces` comment notwithstanding), the table is shared
/// rather than per-manager — observationally equivalent for every in-tree
/// use.
#[derive(Debug, Default)]
pub struct AddrSpaceManager {
    /// Every space we know about for this architecture
    baselist: Vec<Option<Rc<AddrSpace>>>,
    /// Special constant resolvers (C++ `resolvelist`)
    resolvelist: ResolverList,
    /// Map from name -> space (C++ `map<string,AddrSpace *>`)
    name2space: BTreeMap<String, Rc<AddrSpace>>,
    /// Map from shortcut -> space (C++ `map<int4,AddrSpace *>`; the key is
    /// the C++ (signed) char converted to int4)
    shortcut2space: BTreeMap<i32, Rc<AddrSpace>>,
    /// The installed register lookup (see [`RegisterLookup`])
    lookup: RegisterLookupSlot,
    /// Quick reference to constant space
    constantspace: Option<Rc<AddrSpace>>,
    /// Default space where code lives, generally main RAM
    defaultcodespace: Option<Rc<AddrSpace>>,
    /// Default space where data lives
    defaultdataspace: Option<Rc<AddrSpace>>,
    /// Space for internal pcode op pointers
    iopspace: Option<Rc<AddrSpace>>,
    /// Space for internal callspec pointers
    fspecspace: Option<Rc<AddrSpace>>,
    /// Space for unifying split variables
    joinspace: Option<Rc<AddrSpace>>,
    /// Stack space associated with processor
    stackspace: Option<Rc<AddrSpace>>,
    /// Temporary space associated with processor
    uniqspace: Option<Rc<AddrSpace>>,
}

impl AddrSpaceManager {
    /// Construct an empty address space manager.  All the cached space slots
    /// are set to null.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a new address space to the model.
    ///
    /// This adds a previously instantiated address space (AddrSpace) to the
    /// model for this processor.  It checks a set of indexing and naming
    /// conventions for the space and returns an error if the conventions are
    /// violated. Should only be called during initialization.
    pub fn insert_space(&mut self, spc: Rc<AddrSpace>) -> KunaResult<()> {
        // (C++ takes conditional ownership via refcount + unique_ptr; the
        // Rc handles ownership here.)
        match spc.get_type() {
            spacetype::IPTR_CONSTANT => {
                if spc.get_name() != ConstantSpace::NAME {
                    return Err(KunaError::lowlevel(format!(
                        "Space {} was initialized with wrong type",
                        spc.get_name()
                    )));
                }
                if spc.get_index() != ConstantSpace::INDEX {
                    return Err(KunaError::lowlevel("const space must be assigned index 0"));
                }
                self.constantspace = Some(Rc::clone(&spc));
            }
            spacetype::IPTR_INTERNAL => {
                if spc.get_name() != UniqueSpace::NAME {
                    return Err(KunaError::lowlevel(format!(
                        "Space {} was initialized with wrong type",
                        spc.get_name()
                    )));
                }
                if self.uniqspace.is_some() {
                    return Err(KunaError::lowlevel(format!(
                        "Space {} was initialized more than once",
                        spc.get_name()
                    )));
                }
                self.uniqspace = Some(Rc::clone(&spc));
            }
            spacetype::IPTR_FSPEC => {
                if spc.get_name() != "fspec" {
                    return Err(KunaError::lowlevel(format!(
                        "Space {} was initialized with wrong type",
                        spc.get_name()
                    )));
                }
                if self.fspecspace.is_some() {
                    return Err(KunaError::lowlevel(format!(
                        "Space {} was initialized more than once",
                        spc.get_name()
                    )));
                }
                self.fspecspace = Some(Rc::clone(&spc));
            }
            spacetype::IPTR_JOIN => {
                if spc.get_name() != JoinSpace::NAME {
                    return Err(KunaError::lowlevel(format!(
                        "Space {} was initialized with wrong type",
                        spc.get_name()
                    )));
                }
                if self.joinspace.is_some() {
                    return Err(KunaError::lowlevel(format!(
                        "Space {} was initialized more than once",
                        spc.get_name()
                    )));
                }
                self.joinspace = Some(Rc::clone(&spc));
            }
            spacetype::IPTR_IOP => {
                if spc.get_name() != "iop" {
                    return Err(KunaError::lowlevel(format!(
                        "Space {} was initialized with wrong type",
                        spc.get_name()
                    )));
                }
                if self.iopspace.is_some() {
                    return Err(KunaError::lowlevel(format!(
                        "Space {} was initialized more than once",
                        spc.get_name()
                    )));
                }
                self.iopspace = Some(Rc::clone(&spc));
            }
            // C++ falls through from IPTR_SPACEBASE into IPTR_PROCESSOR
            spacetype::IPTR_SPACEBASE | spacetype::IPTR_PROCESSOR => {
                if spc.get_type() == spacetype::IPTR_SPACEBASE && spc.get_name() == "stack" {
                    if self.stackspace.is_some() {
                        return Err(KunaError::lowlevel(format!(
                            "Space {} was initialized more than once",
                            spc.get_name()
                        )));
                    }
                    self.stackspace = Some(Rc::clone(&spc));
                }
                if spc.is_overlay() {
                    // If this is a new overlay space, mark the base as being
                    // overlayed.  (C++ dereferences a null `getContain()` if
                    // the overlay was never decoded — UB => panic, ADR 0004.)
                    spc.get_contain()
                        .expect("overlay space inserted without a base space")
                        .set_flags(fl::overlaybase);
                } else if spc.is_other_space() && spc.get_index() != OtherSpace::INDEX {
                    return Err(KunaError::lowlevel("OTHER space must be assigned index 1"));
                }
            }
        }

        // A negative index would index out of bounds in C++ (UB) => panic
        assert!(spc.get_index() >= 0, "AddrSpace with negative index");
        let index = spc.get_index() as usize;
        if self.baselist.len() <= index {
            self.baselist.resize(index + 1, None);
        }

        if let Some(existing) = &self.baselist[index] {
            return Err(KunaError::lowlevel(format!(
                "Space {} was assigned id duplicating: {}",
                spc.get_name(),
                existing.get_name()
            )));
        }

        if self.name2space.contains_key(spc.get_name()) {
            return Err(KunaError::lowlevel(format!(
                "Space {} was initialized more than once",
                spc.get_name()
            )));
        }
        self.name2space.insert(spc.get_name().to_string(), Rc::clone(&spc));

        self.baselist[index] = Some(Rc::clone(&spc));
        spc.refcount.set(spc.refcount.get() + 1);
        self.assign_shortcut(&spc);
        Ok(())
    }

    /// Select a shortcut character for a new space.
    ///
    /// This routine makes use of the desired type of the new space and info
    /// about shortcuts for spaces that already exist to pick a unique and
    /// consistent character.  This method also builds up a map from shortcut
    /// to AddrSpace object.
    fn assign_shortcut(&mut self, spc: &Rc<AddrSpace>) {
        if spc.shortcut.get() != b' ' {
            // If the shortcut is already assigned (C++ insert() is a no-op
            // on an existing key)
            self.shortcut2space
                .entry(cxx_char_to_int(spc.shortcut.get()))
                .or_insert_with(|| Rc::clone(spc));
            return;
        }
        let mut shortcut: u8 = match spc.get_type() {
            spacetype::IPTR_CONSTANT => b'#',
            spacetype::IPTR_PROCESSOR => {
                if spc.get_name() == "register" {
                    b'%'
                } else {
                    spc.get_name().as_bytes()[0]
                }
            }
            spacetype::IPTR_SPACEBASE => b's',
            spacetype::IPTR_INTERNAL => b'u',
            spacetype::IPTR_FSPEC => b'f',
            spacetype::IPTR_JOIN => b'j',
            spacetype::IPTR_IOP => b'i',
            // C++ has a default: 'x' arm; unreachable since the Rust enum is
            // exhaustive over the same variants.
        };

        if shortcut.is_ascii_uppercase() {
            shortcut += 0x20;
        }

        let mut collision_count = 0;
        loop {
            match self.shortcut2space.entry(cxx_char_to_int(shortcut)) {
                Entry::Vacant(e) => {
                    e.insert(Rc::clone(spc));
                    break;
                }
                Entry::Occupied(_) => {
                    collision_count += 1;
                    if collision_count > 26 {
                        // Could not find a unique shortcut, but we just
                        // re-use 'z' as we can always use the long form to
                        // specify the address if there are really so many
                        // spaces that need to be distinguishable (in the
                        // console mode)
                        spc.set_shortcut_raw(b'z');
                        return;
                    }
                    shortcut = shortcut.wadd(1);
                    // C++: if (shortcut < 'a' || shortcut > 'z') shortcut = 'a';
                    if !shortcut.is_ascii_lowercase() {
                        shortcut = b'a';
                    }
                }
            }
        }
        spc.set_shortcut_raw(shortcut);
    }

    /// Set the default address space (for code).
    ///
    /// Once all the address spaces have been initialized, this routine
    /// should be called once to establish the official \e default space for
    /// the processor, via its index. Should only be called during
    /// initialization.
    pub fn set_default_code_space(&mut self, index: i32) -> KunaResult<()> {
        if self.defaultcodespace.is_some() {
            return Err(KunaError::lowlevel("Default space set multiple times"));
        }
        if self.baselist.len() <= index as usize || self.baselist[index as usize].is_none() {
            return Err(KunaError::lowlevel("Bad index for default space"));
        }
        self.defaultcodespace = self.baselist[index as usize].clone();
        // By default the default data space is the same
        self.defaultdataspace = self.defaultcodespace.clone();
        Ok(())
    }

    /// Set the default address space for data.
    ///
    /// If the architecture has different code and data spaces, this routine
    /// can be called to set the \e data space after the \e code space has
    /// been set.
    pub fn set_default_data_space(&mut self, index: i32) -> KunaResult<()> {
        if self.defaultcodespace.is_none() {
            return Err(KunaError::lowlevel(
                "Default data space must be set after the code space",
            ));
        }
        if self.baselist.len() <= index as usize || self.baselist[index as usize].is_none() {
            return Err(KunaError::lowlevel("Bad index for default data space"));
        }
        self.defaultdataspace = self.baselist[index as usize].clone();
        Ok(())
    }

    /// Set reverse justified property on this space.
    ///
    /// For spaces with alignment restrictions, the address of a small
    /// variable must be justified within a larger aligned memory word,
    /// usually either to the left boundary for little endian encoding or to
    /// the right boundary for big endian encoding.  Some compilers justify
    /// small variables to the opposite side of the one indicated by the
    /// endianness. Setting this property on a space causes the decompiler to
    /// use this justification.
    pub fn set_reverse_justified(&self, spc: &AddrSpace) {
        spc.set_flags(fl::reverse_justification);
    }

    /// Mark that given space can be accessed with near pointers.
    /// \param spc is the AddrSpace to mark
    /// \param size is the (minimum) size of a near pointer in bytes
    pub fn mark_near_pointers(&self, spc: &AddrSpace, size: i32) {
        spc.set_flags(fl::has_nearpointers);
        // mixed comparison: uint4 addressSize vs int4 size (converted up)
        if spc.minimum_pointer_size.get() == 0 && spc.address_size.get() != size as u32 {
            spc.minimum_pointer_size.set(size);
        }
    }

    /// Set the number of passes for a specific AddrSpace before deadcode
    /// removal is allowed for that space.
    pub fn set_deadcode_delay(&self, spc: &AddrSpace, delaydelta: i32) {
        spc.deadcodedelay.set(delaydelta);
    }

    /// Mark a space as truncated from its original size (the body of the C++
    /// `truncateSpace(const TruncationTag &)`; the `TruncationTag` wrapper
    /// arrives with the sleigh wave).
    pub fn truncate_space(&self, space_name: &str, size: u32) -> KunaResult<()> {
        match self.get_space_by_name(space_name) {
            None => Err(KunaError::lowlevel(format!(
                "Unknown space in <truncate_space> command: {space_name}"
            ))),
            Some(spc) => {
                spc.truncate_space(size);
                Ok(())
            }
        }
    }

    /// Get size of addresses for the default space (panics if no default
    /// space has been set — C++ dereferences the null pointer).
    pub fn get_default_size(&self) -> i32 {
        self.defaultcodespace
            .as_ref()
            .expect("default code space not set")
            .get_addr_size() as i32
    }

    /// Get address space by name
    pub fn get_space_by_name(&self, nm: &str) -> Option<&Rc<AddrSpace>> {
        self.name2space.get(nm)
    }

    /// Get address space from its shortcut (the C++ `char` is a byte here)
    pub fn get_space_by_shortcut(&self, sc: u8) -> Option<&Rc<AddrSpace>> {
        self.shortcut2space.get(&cxx_char_to_int(sc))
    }

    /// Get the internal pcode op space
    pub fn get_iop_space(&self) -> Option<&Rc<AddrSpace>> {
        self.iopspace.as_ref()
    }

    /// Get the internal callspec space
    pub fn get_fspec_space(&self) -> Option<&Rc<AddrSpace>> {
        self.fspecspace.as_ref()
    }

    /// Get the joining space
    pub fn get_join_space(&self) -> Option<&Rc<AddrSpace>> {
        self.joinspace.as_ref()
    }

    /// Get the stack space for this processor
    pub fn get_stack_space(&self) -> Option<&Rc<AddrSpace>> {
        self.stackspace.as_ref()
    }

    /// Get the temporary register space for this processor
    pub fn get_unique_space(&self) -> Option<&Rc<AddrSpace>> {
        self.uniqspace.as_ref()
    }

    /// Get the default address space of this processor
    pub fn get_default_code_space(&self) -> Option<&Rc<AddrSpace>> {
        self.defaultcodespace.as_ref()
    }

    /// Get the default address space where data is stored
    pub fn get_default_data_space(&self) -> Option<&Rc<AddrSpace>> {
        self.defaultdataspace.as_ref()
    }

    /// Get the constant space
    pub fn get_constant_space(&self) -> Option<&Rc<AddrSpace>> {
        self.constantspace.as_ref()
    }

    /// Get a constant encoded as an Address (panics if the constant space
    /// has not been registered — C++ would build an Address around null).
    pub fn get_constant(&self, val: u64) -> Address {
        Address::new(
            Rc::clone(self.constantspace.as_ref().expect("constant space not registered")),
            val,
        )
    }

    /// Create a constant address encoding an address space (C++
    /// `createConstFromSpace`).
    ///
    /// This routine is used to encode a pointer to an address space as a
    /// \e constant Address, for use in \b LOAD and \b STORE operations.
    /// The C++ stores the raw `AddrSpace *` heap pointer in the offset; the
    /// Rust port stores the space's manager index, matching
    /// `kuna_num::pcoderaw` (losses ledger LOSS-015).
    pub fn create_const_from_space(&self, spc: &Rc<AddrSpace>) -> Address {
        Address::new(
            Rc::clone(self.constantspace.as_ref().expect("constant space not registered")),
            spc.get_index() as u64, // cast: non-negative space index (LOSS-015)
        )
    }

    /// Set the range of addresses that can be inferred as pointers (C++
    /// protected `setInferPtrBounds`).
    ///
    /// This method establishes for a single address space, what range of
    /// constants are checked as possible symbol starts, when it is not known
    /// apriori that a constant is a pointer.
    /// \param range is the range of values for a single address space
    pub fn set_infer_ptr_bounds(&self, range: &crate::address::Range) {
        range.get_space().pointer_lower_bound.set(range.get_first());
        range.get_space().pointer_upper_bound.set(range.get_last());
    }

    /// Get the number of address spaces for this processor
    pub fn num_spaces(&self) -> i32 {
        self.baselist.len() as i32
    }

    /// Get an address space via its index (C++ `baselist[i]`: an
    /// out-of-range index is UB there and panics here; an unregistered slot
    /// is a null pointer there and `None` here).
    pub fn get_space(&self, i: i32) -> Option<&Rc<AddrSpace>> {
        self.baselist[i as usize].as_ref()
    }

    /// Get the address space associated with the indicated \e spacebase register
    /// (C++ `Architecture::getSpaceBySpacebase`, `architecture.cc:265`).
    ///
    /// If the location of the \e stack \e pointer is passed in, this returns a
    /// pointer to the \b stack space.  `None` if no corresponding space is found
    /// (the C++ throws `LowlevelError`; the only caller — `RuleLoadVarnode::
    /// correctSpacebase` — wraps the call in a context that has already verified
    /// the Varnode is `isSpacebase`, so a miss returns `None` to mean "not the
    /// right space").
    pub fn get_space_by_spacebase(&self, loc: &Address, size: i32) -> Option<Rc<AddrSpace>> {
        let sz = self.num_spaces();
        for i in 0..sz {
            let id = match self.get_space(i) {
                Some(s) => s,
                None => continue,
            };
            let numspace = id.num_spacebase();
            for j in 0..numspace {
                let point = match id.get_spacebase(j) {
                    Ok(p) => p,
                    Err(_) => continue,
                };
                if point.size as i32 != size {
                    continue;
                }
                let point_space = match &point.space {
                    Some(s) => s,
                    None => continue,
                };
                let loc_space = match loc.get_space() {
                    Some(s) => s,
                    None => continue,
                };
                if point_space.get_index() != loc_space.get_index() {
                    continue;
                }
                if point.offset != loc.get_offset() {
                    continue;
                }
                return Some(Rc::clone(id));
            }
        }
        None
    }

    /// Get the next \e contiguous address space.
    ///
    /// Get the next space in the absolute order of addresses.  This ordering
    /// is determined by the AddrSpace index.  The C++ uses the null pointer
    /// ("before the first space") and ~0 ("after the last space") as
    /// sentinels; those map to [`AddrSpacePtr::Null`] and
    /// [`AddrSpacePtr::Max`].
    pub fn get_next_space_in_order(&self, spc: &AddrSpacePtr) -> AddrSpacePtr {
        match spc {
            AddrSpacePtr::Null => match &self.baselist[0] {
                Some(s) => AddrSpacePtr::Spc(Rc::clone(s)),
                None => AddrSpacePtr::Null,
            },
            AddrSpacePtr::Max => AddrSpacePtr::Null,
            AddrSpacePtr::Spc(s) => {
                let mut index = s.get_index() + 1;
                while (index as usize) < self.baselist.len() {
                    if let Some(res) = &self.baselist[index as usize] {
                        return AddrSpacePtr::Spc(Rc::clone(res));
                    }
                    index += 1;
                }
                AddrSpacePtr::Max
            }
        }
    }

    /// Install the [`RegisterLookup`] consulted by register-name paths (the
    /// register half of the C++ `trans` back-pointer; the C++ sets it per
    /// space at construction time, kuna sets it once on the manager).
    pub fn set_register_lookup(&mut self, lookup: Rc<dyn RegisterLookup>) {
        self.lookup.0 = Some(lookup);
    }

    /// The installed [`RegisterLookup`], if any.
    pub fn register_lookup(&self) -> Option<&Rc<dyn RegisterLookup>> {
        self.lookup.0.as_ref()
    }

    /// Copy spaces from another manager.
    ///
    /// Different managers may need to share the same spaces. I.e. if
    /// different programs being analyzed share the same processor. This
    /// routine pulls in a reference of every space in `op2` in order to
    /// manage it from within `self`.
    pub fn copy_spaces(&mut self, op2: &AddrSpaceManager) -> KunaResult<()> {
        // Insert every space in -op2- into -this- manager
        for spc in op2.baselist.iter().flatten() {
            self.insert_space(Rc::clone(spc))?;
        }
        // C++ dereferences op2's default spaces unconditionally (null is UB)
        self.set_default_code_space(
            op2.get_default_code_space()
                .expect("copySpaces: source manager has no default code space (C++ UB)")
                .get_index(),
        )?;
        self.set_default_data_space(
            op2.get_default_data_space()
                .expect("copySpaces: source manager has no default data space (C++ UB)")
                .get_index(),
        )
    }

    /// Set the base register of a spacebase space (C++ protected
    /// `addSpacebasePointer`, the \e privileged friend access to
    /// `SpacebaseSpace::setBaseRegister`).
    /// \param basespace is the virtual space (must be a spacebase space)
    /// \param ptrdata is the location data for the base register
    /// \param trunc_size is the size of the space covered by the base register
    /// \param stack_growth is true if the stack grows "normally" towards address 0
    pub fn add_spacebase_pointer(
        &self,
        basespace: &AddrSpace,
        ptrdata: &VarnodeStorage,
        trunc_size: i32,
        stack_growth: bool,
    ) -> KunaResult<()> {
        basespace.set_base_register(ptrdata, trunc_size, stack_growth)
    }

    /// Override the base resolver for a space (C++ protected
    /// `insertResolver`).  The manager takes ownership of the resolver.
    /// \param spc is the space to which the resolver is associated
    /// \param rsolv is the new resolver object
    pub fn insert_resolver(&mut self, spc: &AddrSpace, rsolv: Box<dyn AddressResolver>) {
        // A negative index would index out of bounds in C++ (UB) => panic
        assert!(spc.get_index() >= 0, "insertResolver: AddrSpace with negative index");
        let ind = spc.get_index() as usize;
        while self.resolvelist.0.len() <= ind {
            self.resolvelist.0.push(None);
        }
        // (the C++ deletes any previously installed resolver; drop here)
        self.resolvelist.0[ind] = Some(rsolv);
    }

    /// \brief Resolve a native constant into an Address
    ///
    /// If there is a special resolver for the AddrSpace, this is invoked,
    /// otherwise basic wordsize conversion and wrapping is performed. If the
    /// address encoding is partial (as in a \e near pointer) and the full
    /// encoding can be recovered, it is passed back.  The \e sz parameter
    /// indicates the number of bytes in the constant and is used to
    /// determine if the constant is a partial or full pointer encoding. A
    /// value of -1 indicates the value is known to be a full encoding.
    /// \param spc is the space to generate the address from
    /// \param val is the constant encoding of the address
    /// \param sz is the size of the constant encoding (or -1)
    /// \param point is the context address (for recovering full encoding
    ///        info if necessary)
    /// \param full_encoding is used to pass back the recovered full encoding
    ///        of the pointer
    /// \return the formal Address associated with the encoding
    pub fn resolve_constant(
        &self,
        spc: &Rc<AddrSpace>,
        val: u64,
        sz: i32,
        point: &Address,
        full_encoding: &mut u64,
    ) -> KunaResult<Address> {
        let ind = spc.get_index();
        // mixed comparison: int4 ind vs size_t list length (C++ converts ind
        // up; negative ind never enters the branch there or here)
        if ind >= 0 && (ind as usize) < self.resolvelist.0.len() {
            if let Some(resolve) = &self.resolvelist.0[ind as usize] {
                return resolve.resolve(val, sz, point, full_encoding);
            }
        }
        *full_encoding = val;
        let val = AddrSpace::address_to_byte(val, spc.get_word_size());
        let val = spc.wrap_offset(val);
        Ok(Address::new(Rc::clone(spc), val))
    }

    /// The join-record table, reached through the registered join space (in
    /// C++ the table lives directly on the manager).
    fn join_records(&self) -> KunaResult<&RefCell<JoinState>> {
        self.joinspace
            .as_ref()
            .and_then(|spc| spc.join_state())
            .ok_or_else(|| {
                KunaError::lowlevel(
                    "kuna rust port: join-record request without a registered join space",
                )
            })
    }

    /// Get (or create) JoinRecord for \e pieces (C++ `findAddJoin`).
    ///
    /// Given a list of memory locations, the \e pieces, either find a
    /// pre-existing JoinRecord or create a JoinRecord that represents the
    /// logical joining of the pieces.  The pieces must be in order from most
    /// significant to least significant.
    /// \param pieces is the list of memory locations to be joined
    /// \param logicalsize is the size of a \e single \e piece join, or zero
    /// \return the JoinRecord
    pub fn find_add_join(
        &self,
        pieces: &[VarnodeStorage],
        logicalsize: u32,
    ) -> KunaResult<Rc<JoinRecord>> {
        // Find a pre-existing split record, or create a new one
        // corresponding to the input -pieces-.  If -logicalsize- is 0,
        // calculate logical size as sum of pieces
        if pieces.is_empty() {
            return Err(KunaError::lowlevel("Cannot create a join without pieces"));
        }
        if pieces.len() == 1 && logicalsize == 0 {
            return Err(KunaError::lowlevel(
                "Cannot create a single piece join without a logical size",
            ));
        }

        let totalsize: u32;
        if logicalsize != 0 {
            if pieces.len() != 1 {
                return Err(KunaError::lowlevel(
                    "Cannot specify logical size for multiple piece join",
                ));
            }
            totalsize = logicalsize;
        } else {
            // Calculate sum of the sizes of all pieces (uint4 arithmetic)
            let mut sum: u32 = 0;
            for piece in pieces {
                sum = sum.wadd(piece.size);
            }
            if sum == 0 {
                return Err(KunaError::lowlevel("Cannot create a zero size join"));
            }
            totalsize = sum;
        }

        let state = self.join_records()?;
        let testnode = JoinRecord {
            pieces: pieces.to_vec(),
            // C++ leaves testnode.unified.space/offset default; only the
            // size participates in the comparator
            unified: VarnodeStorage { space: None, offset: 0, size: totalsize },
        };
        if let Some(rec) = state.borrow().splitset.get(&testnode) {
            // If already in the set
            return Ok(Rc::clone(rec));
        }

        // Next biggest multiple of 16 (uint4 arithmetic)
        let roundsize: u32 = totalsize.wadd(15) & !0xfu32;

        let mut state = state.borrow_mut();
        let newjoin = Rc::new(JoinRecord {
            pieces: pieces.to_vec(),
            unified: VarnodeStorage {
                space: self.joinspace.clone(),
                offset: state.joinallocate,
                size: totalsize,
            },
        });
        // joinallocate += roundsize: uintb += uint4
        state.joinallocate = state.joinallocate.wadd(u64::from(roundsize));
        state.splitset.insert(Rc::clone(&newjoin));
        state.splitlist.push(Rc::clone(&newjoin));
        Ok(newjoin)
    }

    /// Find JoinRecord for \e offset in the join space (C++ protected
    /// `findJoinInternal`): recover the JoinRecord that *contains* the
    /// offset, as a range in the \e join address space, or `None`.
    fn find_join_internal(&self, offset: u64) -> KunaResult<Option<Rc<JoinRecord>>> {
        Ok(self.join_records()?.borrow().find_join_internal(offset))
    }

    /// Find JoinRecord for \e offset in the join space (C++ `findJoin`).
    ///
    /// The offset must originally have come from a JoinRecord returned by
    /// `find_add_join`, otherwise this method errs.
    pub fn find_join(&self, offset: u64) -> KunaResult<Rc<JoinRecord>> {
        self.join_records()?.borrow().find_join(offset)
    }

    /// \brief Build a logically lower precision storage location for a
    /// bigger floating point register (C++
    /// `constructFloatExtensionAddress`)
    ///
    /// This handles the situation where we need to find a logical address to
    /// hold the lower precision floating-point value that is stored in a
    /// bigger register.  If the logicalsize (precision) requested matches
    /// the -realsize- of the register just return the real address.
    /// Otherwise construct a join address to hold the logical value.
    /// \param realaddr is the address of the real floating-point register
    /// \param realsize is the size of the real floating-point register
    /// \param logicalsize is the size (lower precision) of the logical value
    pub fn construct_float_extension_address(
        &self,
        realaddr: &Address,
        realsize: i32,
        logicalsize: i32,
    ) -> KunaResult<Address> {
        if logicalsize == realsize {
            return Ok(realaddr.clone());
        }
        let pieces = vec![VarnodeStorage {
            space: realaddr.get_space().cloned(),
            offset: realaddr.get_offset(),
            size: realsize as u32, // cast: int4 -> uint4 member
        }];
        let join = self.find_add_join(&pieces, logicalsize as u32)?; // cast: int4 -> uint4 parameter
        Ok(join.get_unified().get_addr())
    }

    /// \brief Build a logical whole from register pairs (C++
    /// `constructJoinAddress`)
    ///
    /// This handles the common case, of trying to find a join address given
    /// a high location and a low location. This may not return an address in
    /// the \e join address space.  It checks for the case where the two
    /// pieces are contiguous locations in a mappable space, in which case it
    /// just returns the containing address.
    /// \param translate is the `RegisterLookup` used to find registers (the
    ///        C++ `Translate *`)
    /// \param hiaddr is the address of the most significant piece to be joined
    /// \param hisz is the size of the most significant piece
    /// \param loaddr is the address of the least significant piece
    /// \param losz is the size of the least significant piece
    /// \return an address representing the start of the joined range
    pub fn construct_join_address(
        &self,
        translate: &dyn RegisterLookup,
        hiaddr: &Address,
        hisz: i32,
        loaddr: &Address,
        losz: i32,
    ) -> KunaResult<Address> {
        // C++ dereferences the space pointers unconditionally (null is UB)
        let hispace = hiaddr
            .get_space()
            .expect("constructJoinAddress: invalid hiaddr (C++ UB)");
        let lospace = loaddr
            .get_space()
            .expect("constructJoinAddress: invalid loaddr (C++ UB)");
        let hitp = hispace.get_type();
        let lotp = lospace.get_type();
        let mut usejoinspace = true;
        if ((hitp != spacetype::IPTR_SPACEBASE) && (hitp != spacetype::IPTR_PROCESSOR))
            || ((lotp != spacetype::IPTR_SPACEBASE) && (lotp != spacetype::IPTR_PROCESSOR))
        {
            // (sic: the C++ message reads "in appropriate")
            return Err(KunaError::lowlevel("Trying to join in appropriate locations"));
        }
        let default_code_eq = |spc: &Rc<AddrSpace>| -> bool {
            match self.get_default_code_space() {
                Some(def) => Rc::ptr_eq(spc, def),
                None => false, // C++ compares against a null pointer
            }
        };
        if (hitp == spacetype::IPTR_SPACEBASE)
            || (lotp == spacetype::IPTR_SPACEBASE)
            || default_code_eq(hispace)
            || default_code_eq(lospace)
        {
            usejoinspace = false;
        }
        if hiaddr.is_contiguous(hisz, loaddr, losz) {
            // If we are contiguous
            if !usejoinspace {
                // and in a mappable space, just return the earliest address
                if hiaddr.is_big_endian() {
                    return Ok(hiaddr.clone());
                }
                return Ok(loaddr.clone());
            } else {
                // If we are in a non-mappable (register) space, check to see
                // if a parent register exists
                if hiaddr.is_big_endian() {
                    if !translate
                        .get_register_name(hispace, hiaddr.get_offset(), hisz + losz)
                        .is_empty()
                    {
                        return Ok(hiaddr.clone());
                    }
                } else if !translate
                    .get_register_name(lospace, loaddr.get_offset(), hisz + losz)
                    .is_empty()
                {
                    return Ok(loaddr.clone());
                }
            }
        }
        // Otherwise construct a formal JoinRecord
        let pieces = vec![
            VarnodeStorage {
                space: Some(Rc::clone(hispace)),
                offset: hiaddr.get_offset(),
                size: hisz as u32, // cast: int4 -> uint4 member
            },
            VarnodeStorage {
                space: Some(Rc::clone(lospace)),
                offset: loaddr.get_offset(),
                size: losz as u32, // cast: int4 -> uint4 member
            },
        ];
        let join = self.find_add_join(&pieces, 0)?;
        Ok(join.get_unified().get_addr())
    }

    /// \brief Make sure a possibly offset \e join address has a proper
    /// JoinRecord (C++ `renormalizeJoinAddress`)
    ///
    /// If an Address in the \e join AddressSpace is shifted from its
    /// original offset, it may no longer have a valid JoinRecord.  The shift
    /// or size change may even make the address of one of the pieces a more
    /// natural representation.  Given a new Address and size, this method
    /// decides if there is a matching JoinRecord. If not it either
    /// constructs a new JoinRecord or computes the address within the
    /// containing piece.  The given Address is changed if necessary either
    /// to the offset corresponding to the new JoinRecord or to a normal
    /// \e non-join Address.
    /// \param addr is the given Address
    /// \param size is the size of the range in bytes
    pub fn renormalize_join_address(&self, addr: &mut Address, size: i32) -> KunaResult<()> {
        let join_record = match self.find_join_internal(addr.get_offset())? {
            Some(rec) => rec,
            None => {
                return Err(KunaError::lowlevel("Join address not covered by a JoinRecord"))
            }
        };
        // size == unified.size: int4 converts up to uint4 in C++
        if addr.get_offset() == join_record.unified.offset
            && size as u32 == join_record.unified.size
        {
            return Ok(()); // JoinRecord matches perfectly, no change necessary
        }
        let mut pos1: i32 = 0;
        let addr1 = join_record.get_equivalent_address(addr.get_offset(), &mut pos1);
        let mut pos2: i32 = 0;
        // addr.getOffset() + (size-1): int4 sign-extends to uintb
        let addr2 = join_record
            .get_equivalent_address(addr.get_offset().wadd((size - 1) as i64 as u64), &mut pos2);
        if addr2.is_invalid() {
            return Err(KunaError::lowlevel("Join address range not covered"));
        }
        if pos1 == pos2 {
            *addr = addr1;
            return Ok(());
        }
        let mut new_pieces: Vec<VarnodeStorage> = Vec::new();
        // (int4)(addr1.getOffset() - pieces[pos1].offset): truncating cast
        let size_trunc1 = addr1
            .get_offset()
            .wsub(join_record.pieces[pos1 as usize].offset) as i32;
        // pieces[pos2].size - (int4)(...) - 1: the int4 converts back up to
        // uint4 for the subtraction; the uint4 result is assigned to int4
        let size_trunc2 = join_record.pieces[pos2 as usize]
            .size
            .wsub(addr2.get_offset().wsub(join_record.pieces[pos2 as usize].offset) as u32)
            .wsub(1) as i32;

        if pos2 < pos1 {
            // Little endian
            new_pieces.push(join_record.pieces[pos2 as usize].clone());
            pos2 += 1;
            while pos2 <= pos1 {
                new_pieces.push(join_record.pieces[pos2 as usize].clone());
                pos2 += 1;
            }
            let back = new_pieces.last_mut().expect("non-empty");
            back.offset = addr1.get_offset();
            back.size = back.size.wsub(size_trunc1 as u32); // cast: uint4 -= int4
            let front = new_pieces.first_mut().expect("non-empty");
            front.size = front.size.wsub(size_trunc2 as u32); // cast: uint4 -= int4
        } else {
            new_pieces.push(join_record.pieces[pos1 as usize].clone());
            pos1 += 1;
            while pos1 <= pos2 {
                new_pieces.push(join_record.pieces[pos1 as usize].clone());
                pos1 += 1;
            }
            let front = new_pieces.first_mut().expect("non-empty");
            front.offset = addr1.get_offset();
            front.size = front.size.wsub(size_trunc1 as u32); // cast: uint4 -= int4
            let back = new_pieces.last_mut().expect("non-empty");
            back.size = back.size.wsub(size_trunc2 as u32); // cast: uint4 -= int4
        }
        let new_join_record = self.find_add_join(&new_pieces, 0)?;
        *addr = match &new_join_record.unified.space {
            Some(spc) => Address::new(Rc::clone(spc), new_join_record.unified.offset),
            // C++ would build the Address around a null space pointer
            None => Address::from_space_ptr(AddrSpacePtr::Null, new_join_record.unified.offset),
        };
        Ok(())
    }

    /// \brief Create an Address by stripping a piece from a JoinRecord (C++
    /// `stripJoinPiece`)
    ///
    /// If only 1 piece remains, the VarnodeStorage of that piece is
    /// returned.  Otherwise a new JoinRecord is created and its unified
    /// VarnodeStorage is returned.  (C++ returns a `const VarnodeData &`;
    /// the Rust port returns a clone of the triple.)
    /// \param join is the JoinRecord to strip
    /// \param index is the index of the piece to strip, which must be at the
    ///        front or back
    /// \return the storage triple corresponding to the remaining piece(s)
    pub fn strip_join_piece(&self, join: &JoinRecord, index: i32) -> KunaResult<VarnodeStorage> {
        let start: i32;
        let end: i32;
        if index == 0 {
            start = 1;
            end = join.num_pieces() - 1;
        } else if index == join.num_pieces() - 1 {
            start = 0;
            end = join.num_pieces() - 2;
        } else {
            return Err(KunaError::lowlevel("Stripping middle piece from JoinRecord"));
        }
        if start == end {
            return Ok(join.get_piece(start).clone());
        }
        let mut new_pieces: Vec<VarnodeStorage> = Vec::new();
        let mut i = start;
        while i <= end {
            new_pieces.push(join.get_piece(i).clone());
            i += 1;
        }
        let new_join_record = self.find_add_join(&new_pieces, 0)?;
        Ok(new_join_record.get_unified().clone())
    }

    /// \brief Parse a string with just an \e address \e space name and a hex
    /// offset (C++ `parseAddressSimple`)
    ///
    /// The string \e must contain a hexadecimal offset.  The offset may be
    /// optionally prepended with "0x".  The string may optionally start with
    /// the name of the address space to associate with the offset, followed
    /// by ':' to separate it from the offset.  If the name is not present,
    /// the default data space is assumed.
    /// \param val is the string to parse
    /// \return the parsed address
    pub fn parse_address_simple(&self, val: &str) -> KunaResult<Address> {
        let bytes = val.as_bytes();
        let col = bytes.iter().position(|&c| c == b':');
        let spc: Rc<AddrSpace>;
        let mut col = match col {
            None => {
                // C++ dereferences a null default data space below (UB)
                spc = Rc::clone(
                    self.get_default_data_space()
                        .expect("parseAddressSimple: no default data space (C++ UB)"),
                );
                0
            }
            Some(col) => {
                let spc_name = &val[..col];
                spc = match self.get_space_by_name(spc_name) {
                    Some(spc) => Rc::clone(spc),
                    None => {
                        return Err(KunaError::lowlevel(format!(
                            "Unknown address space: {spc_name}"
                        )))
                    }
                };
                col + 1
            }
        };
        if col + 2 <= bytes.len() && bytes[col] == b'0' && bytes[col + 1] == b'x' {
            col += 2;
        }
        // istringstream `s >> hex >> off`
        let off = cxx_istream_hex_u64(&bytes[col..]);
        Ok(Address::new(
            Rc::clone(&spc),
            AddrSpace::address_to_byte(off, spc.get_word_size()),
        ))
    }

    /// Add a space to the model based on a stream element (C++ protected
    /// `decodeSpace`; the `Translate *` parameter is only the C++
    /// back-pointer and is dropped, and the receiver is `&self` — the C++
    /// uses `this` only as another back-pointer for the constructors).
    ///
    /// This routine initializes a single address space from a decoder
    /// element.  It knows which class derived from AddrSpace to instantiate
    /// based on the ElementId.
    /// \param decoder is the stream decoder
    /// \return the initialized AddrSpace
    pub fn decode_space(&self, decoder: &mut dyn Decoder) -> KunaResult<Rc<AddrSpace>> {
        let elem_id = decoder.peek_element()?;
        let mut res: AddrSpace = if elem_id == ELEM_SPACE_BASE.get_id() {
            SpacebaseSpace::new_for_decode()
        } else if elem_id == ELEM_SPACE_UNIQUE.get_id() {
            UniqueSpace::new_for_decode()
        } else if elem_id == ELEM_SPACE_OTHER.get_id() {
            OtherSpace::new_for_decode()
        } else if elem_id == ELEM_SPACE_OVERLAY.get_id() {
            OverlaySpace::new()
        } else {
            AddrSpace::new_for_decode(spacetype::IPTR_PROCESSOR)
        };

        res.decode(decoder)?;
        Ok(Rc::new(res))
    }

    /// Restore address spaces in the model from a stream (C++ protected
    /// `decodeSpaces`).
    ///
    /// This routine initializes (almost) all the address spaces used for a
    /// particular processor by using a \b \<spaces\> element, which contains
    /// child elements for the specific address spaces.  This also
    /// instantiates the builtin \e constant space. It should probably also
    /// instantiate the \b iop, \b fspec, and \b join spaces, but this is
    /// currently done by the Architecture class.
    ///
    /// (kuna rust) Rust's aliasing rules make this method un-callable with a
    /// decoder constructed over `self` (the C++ usage): the decoder holds
    /// `&AddrSpaceManager` while `insert_space` needs `&mut self`.  Until
    /// the architecture wave revisits the `Decoder` manager access, callers
    /// drive the identical loop body stepwise — a fresh decoder per child
    /// element around each `decode_space`/`insert_space` pair — so each
    /// element's `read_space` resolution sees the previously inserted
    /// spaces.
    /// \param decoder is the stream decoder
    pub fn decode_spaces(&mut self, decoder: &mut dyn Decoder) -> KunaResult<()> {
        // The first space should always be the constant space
        self.insert_space(Rc::new(ConstantSpace::new()))?;

        let elem_id = decoder.open_element_id(&ELEM_SPACES)?;
        let defname =
            String::from_utf8_lossy(&decoder.read_string_id(&ATTRIB_DEFAULTSPACE)?).into_owned();
        while decoder.peek_element()? != 0 {
            let spc = self.decode_space(decoder)?;
            self.insert_space(spc)?;
        }
        decoder.close_element(elem_id)?;
        let spc = match self.get_space_by_name(&defname) {
            Some(spc) => Rc::clone(spc),
            None => {
                return Err(KunaError::lowlevel(format!(
                    "Bad 'defaultspace' attribute: {defname}"
                )))
            }
        };
        self.set_default_code_space(spc.get_index())
    }
}

/// The C++ `map<int4,AddrSpace*>` shortcut key is the (signed, on x86)
/// `char` converted to `int4`: bytes >= 0x80 become negative keys.
fn cxx_char_to_int(c: u8) -> i32 {
    (c as i8) as i32
}

/// `istringstream >> hex >> off` into a `uintb` (parseAddressSimple):
/// num_get skips leading whitespace, accepts an optional sign and an
/// optional "0x"/"0X" prefix, then hex digits.  A failed extraction stores 0
/// (C++11); overflow saturates at ULLONG_MAX; a '-' sign negates modularly.
fn cxx_istream_hex_u64(bytes: &[u8]) -> u64 {
    let mut i = 0;
    while i < bytes.len() && (bytes[i] as char).is_ascii_whitespace() {
        i += 1;
    }
    let mut negative = false;
    if i < bytes.len() && (bytes[i] == b'+' || bytes[i] == b'-') {
        negative = bytes[i] == b'-';
        i += 1;
    }
    if i + 1 < bytes.len() && bytes[i] == b'0' && (bytes[i + 1] == b'x' || bytes[i + 1] == b'X') {
        i += 2;
    }
    let mut val: u64 = 0;
    let mut overflow = false;
    let mut any = false;
    while i < bytes.len() {
        let digit = match (bytes[i] as char).to_digit(16) {
            Some(d) => u64::from(d),
            None => break,
        };
        any = true;
        let (shifted, ovf1) = val.overflowing_mul(16);
        let (next, ovf2) = shifted.overflowing_add(digit);
        overflow |= ovf1 || ovf2;
        val = next;
        i += 1;
    }
    if !any {
        return 0;
    }
    if overflow {
        return u64::MAX;
    }
    if negative {
        val.wneg()
    } else {
        val
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn ram_space(ind: i32) -> AddrSpace {
        AddrSpace::new(spacetype::IPTR_PROCESSOR, "ram", false, 8, 1, ind, fl::hasphysical, 1, 1)
    }

    #[test]
    fn test_space_calc_scale_mask() {
        let spc = ram_space(3);
        assert_eq!(spc.get_highest(), u64::MAX);
        assert_eq!(spc.get_pointer_lower_bound(), 0x1000);
        assert_eq!(spc.get_pointer_upper_bound(), u64::MAX.wrapping_sub(0x1000));
        let small = AddrSpace::new(spacetype::IPTR_PROCESSOR, "io", false, 2, 1, 4, 0, 0, 0);
        assert_eq!(small.get_highest(), 0xffff);
        assert_eq!(small.get_pointer_lower_bound(), 0x100);
        assert_eq!(small.get_pointer_upper_bound(), 0xffff - 0x100);
        // wordsize scaling: highest = mask * wordsize + (wordsize-1)
        let word = AddrSpace::new(spacetype::IPTR_PROCESSOR, "word", false, 2, 2, 5, 0, 0, 0);
        assert_eq!(word.get_highest(), 0xffffu64 * 2 + 1);
    }

    #[test]
    fn test_space_wrap_offset() {
        let small = AddrSpace::new(spacetype::IPTR_PROCESSOR, "io", false, 2, 1, 4, 0, 0, 0);
        assert_eq!(small.wrap_offset(0x10000), 0);
        assert_eq!(small.wrap_offset(0x1ffff), 0xffff);
        assert_eq!(small.wrap_offset(0xffff), 0xffff);
        // negative-as-unsigned offsets wrap through the signed remainder
        assert_eq!(small.wrap_offset(1u64.wrapping_neg()), 0xffff);
    }

    #[test]
    fn test_space_manager_insert_and_shortcuts() {
        let mut manager = AddrSpaceManager::new();
        manager.insert_space(Rc::new(ConstantSpace::new())).unwrap();
        manager.insert_space(Rc::new(OtherSpace::new(1))).unwrap();
        manager.insert_space(Rc::new(UniqueSpace::new(2, 0, false))).unwrap();
        manager.insert_space(Rc::new(ram_space(3))).unwrap();
        manager
            .insert_space(Rc::new(AddrSpace::new(
                spacetype::IPTR_PROCESSOR,
                "register",
                false,
                4,
                1,
                4,
                0,
                0,
                0,
            )))
            .unwrap();
        manager.insert_space(Rc::new(FspecSpace::new(5))).unwrap();
        manager.insert_space(Rc::new(IopSpace::new(6))).unwrap();
        manager.insert_space(Rc::new(JoinSpace::new(7, false))).unwrap();
        manager.set_default_code_space(3).unwrap();

        assert_eq!(manager.num_spaces(), 8);
        assert_eq!(manager.get_default_size(), 8);
        // Shortcut assignment: '#' const, 'o' OTHER, 'u' unique, 'r' ram,
        // '%' register, 'f' fspec, 'i' iop, 'j' join
        assert_eq!(manager.get_space(0).unwrap().get_shortcut(), '#');
        assert_eq!(manager.get_space(1).unwrap().get_shortcut(), 'o');
        assert_eq!(manager.get_space(2).unwrap().get_shortcut(), 'u');
        assert_eq!(manager.get_space(3).unwrap().get_shortcut(), 'r');
        assert_eq!(manager.get_space(4).unwrap().get_shortcut(), '%');
        assert_eq!(manager.get_space(5).unwrap().get_shortcut(), 'f');
        assert_eq!(manager.get_space(6).unwrap().get_shortcut(), 'i');
        assert_eq!(manager.get_space(7).unwrap().get_shortcut(), 'j');
        assert!(Rc::ptr_eq(
            manager.get_space_by_shortcut(b'r').unwrap(),
            manager.get_space(3).unwrap()
        ));
        assert!(Rc::ptr_eq(
            manager.get_space_by_name("OTHER").unwrap(),
            manager.get_space(1).unwrap()
        ));
        // Special-space quick references
        assert!(manager.get_constant_space().is_some());
        assert!(manager.get_unique_space().is_some());
        assert!(manager.get_fspec_space().is_some());
        assert!(manager.get_iop_space().is_some());
        assert!(manager.get_join_space().is_some());
        assert!(manager.get_stack_space().is_none());
    }

    #[test]
    fn test_space_manager_insert_errors() {
        let mut manager = AddrSpaceManager::new();
        // const space with wrong index
        let mut bad = ConstantSpace::new();
        bad.index = 5;
        let err = manager.insert_space(Rc::new(bad)).unwrap_err();
        assert_eq!(err.explain(), "const space must be assigned index 0");
        // duplicate id
        manager.insert_space(Rc::new(ram_space(3))).unwrap();
        let err = manager
            .insert_space(Rc::new(AddrSpace::new(
                spacetype::IPTR_PROCESSOR,
                "ram2",
                false,
                8,
                1,
                3,
                0,
                0,
                0,
            )))
            .unwrap_err();
        assert_eq!(err.explain(), "Space ram2 was assigned id duplicating: ram");
        // duplicate name
        let err = manager
            .insert_space(Rc::new(AddrSpace::new(
                spacetype::IPTR_PROCESSOR,
                "ram",
                false,
                8,
                1,
                9,
                0,
                0,
                0,
            )))
            .unwrap_err();
        assert_eq!(err.explain(), "Space ram was initialized more than once");
    }

    #[test]
    fn test_space_print_raw_padding() {
        let spc = ram_space(3);
        let mut s = String::new();
        spc.print_raw(&mut s, 0x1234).unwrap();
        assert_eq!(s, "0x00001234"); // 8-byte space shrinks to 4 for small offsets
        let mut s = String::new();
        spc.print_raw(&mut s, 0x123456789).unwrap();
        assert_eq!(s, "0x000123456789"); // 6-byte form
        let mut s = String::new();
        spc.print_raw(&mut s, 0x1234567890abcdef).unwrap();
        assert_eq!(s, "0x1234567890abcdef");
        let constant = ConstantSpace::new();
        let mut s = String::new();
        constant.print_raw(&mut s, 0x42).unwrap();
        assert_eq!(s, "0x42");
    }

    #[test]
    fn test_space_varnode_storage_compare() {
        let manager = {
            let mut m = AddrSpaceManager::new();
            m.insert_space(Rc::new(ConstantSpace::new())).unwrap();
            m.insert_space(Rc::new(ram_space(3))).unwrap();
            m
        };
        let r = Rc::clone(manager.get_space(3).unwrap());
        let c = Rc::clone(manager.get_constant_space().unwrap());
        let vn = |s: &Rc<AddrSpace>, off: u64, size: u32| VarnodeStorage {
            space: Some(Rc::clone(s)),
            offset: off,
            size,
        };
        // Equality requires identical space/offset/size
        assert_eq!(vn(&r, 0x10, 4), vn(&r, 0x10, 4));
        assert_ne!(vn(&r, 0x10, 4), vn(&r, 0x10, 8));
        assert_ne!(vn(&r, 0x10, 4), vn(&c, 0x10, 4));
        // Ordering: space index, then offset, then BIG sizes first
        assert!(vn(&c, 0x10, 4) < vn(&r, 0x0, 4));
        assert!(vn(&r, 0x10, 4) < vn(&r, 0x11, 4));
        assert!(vn(&r, 0x10, 8) < vn(&r, 0x10, 4));
        // little-endian contiguity: `this` is the most significant piece
        assert!(vn(&r, 0x104, 4).is_contiguous(&vn(&r, 0x100, 4)));
        assert!(!vn(&r, 0x100, 4).is_contiguous(&vn(&r, 0x104, 4)));
        assert!(!vn(&r, 0x104, 4).is_contiguous(&vn(&c, 0x100, 4)));
    }

    #[test]
    fn test_space_join_record_ord() {
        let manager = {
            let mut m = AddrSpaceManager::new();
            m.insert_space(Rc::new(ram_space(3))).unwrap();
            m
        };
        let r = Rc::clone(manager.get_space(3).unwrap());
        let vn = |off: u64, size: u32| VarnodeStorage {
            space: Some(Rc::clone(&r)),
            offset: off,
            size,
        };
        let rec = |pieces: Vec<VarnodeStorage>, usize_: u32| JoinRecord {
            pieces,
            unified: VarnodeStorage { space: None, offset: 0, size: usize_ },
        };
        // unified size compares first (floats: same piece, different size)
        assert!(rec(vec![vn(0, 8)], 4) < rec(vec![vn(0, 8)], 8));
        // lexicographic piece order
        assert!(rec(vec![vn(0, 4), vn(8, 4)], 8) < rec(vec![vn(0, 4), vn(12, 4)], 8));
        // prefix: fewer pieces sorts Less, same pieces Equal
        assert!(rec(vec![vn(0, 4)], 8) < rec(vec![vn(0, 4), vn(8, 4)], 8));
        assert!(rec(vec![vn(0, 4), vn(8, 4)], 8) > rec(vec![vn(0, 4)], 8));
        assert_eq!(rec(vec![vn(0, 4), vn(8, 4)], 8), rec(vec![vn(0, 4), vn(8, 4)], 8));
    }

    #[test]
    fn test_space_istream_hex() {
        // num_get hex semantics for parseAddressSimple
        assert_eq!(cxx_istream_hex_u64(b"1000"), 0x1000);
        assert_eq!(cxx_istream_hex_u64(b"  0xfF"), 0xff);
        assert_eq!(cxx_istream_hex_u64(b"10zz"), 0x10); // stops at non-digit
        assert_eq!(cxx_istream_hex_u64(b""), 0); // failed extraction stores 0
        assert_eq!(cxx_istream_hex_u64(b"zz"), 0);
        // overflow saturates at ULLONG_MAX
        assert_eq!(cxx_istream_hex_u64(b"1ffffffffffffffff"), u64::MAX);
        // '-' negates modularly
        assert_eq!(cxx_istream_hex_u64(b"-1"), u64::MAX);
    }

    #[test]
    fn test_space_read_offsets() {
        let mut manager = AddrSpaceManager::new();
        manager.insert_space(Rc::new(ram_space(3))).unwrap();
        manager.set_default_code_space(3).unwrap();
        let spc = Rc::clone(manager.get_space(3).unwrap());
        let mut size: i32 = 0;
        let off = spc.read("0x1000", &mut size, &manager).unwrap();
        assert_eq!(off, 0x1000);
        assert_eq!(size, 8); // "natural" size of the default space
        let off = spc.read("0x1000:2", &mut size, &manager).unwrap();
        assert_eq!(off, 0x1000);
        assert_eq!(size, 2);
        let off = spc.read("0x1000:2+4", &mut size, &manager).unwrap();
        assert_eq!(off, 0x1004);
        assert_eq!(size, 2);
        let off = spc.read("0x1000+8", &mut size, &manager).unwrap();
        assert_eq!(off, 0x1008);
        assert_eq!(size, 8);
    }
}
