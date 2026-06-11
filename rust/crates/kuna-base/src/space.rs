//! Port of `decompiler/cpp/space.hh` + `space.cc` (W1, item
//! `w1-base-space-address`) — classes for describing address spaces — plus
//! the **minimal** `AddrSpaceManager` lookup core from `translate.hh/.cc`
//! (registration + name/shortcut/index lookup), which `Decoder::readSpace`
//! requires.  The full `Translate` port arrives with the sleigh wave.
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
//! - C++ virtual dispatch over the `AddrSpace` subclass hierarchy becomes a
//!   private kind discriminant: each `virtual` method matches on the kind,
//!   one arm per C++ override.  The subclass names survive as constructor
//!   types (`ConstantSpace::new()`, ...) carrying their `NAME`/`INDEX`
//!   constants.
//! - Fields the C++ code mutates *after* a space is registered (through the
//!   manager's friend access) are `Cell`s; everything else is set during
//!   construction/decode (`&mut self`, before the `Rc` wrap).
//!
//! Deferred (recorded in the item's loss list): everything requiring
//! `JoinRecord`/`VarnodeData` (JoinSpace overlapJoin/encode/decode/read/
//! printRaw; the manager's join-record machinery), `FspecSpace`/`IopSpace`
//! printRaw/encodeAttributes payloads (need `FuncCallSpecs`/`PcodeOp`), the
//! register-name branch of `AddrSpace::read` (needs `Translate`), and
//! `SpacebaseSpace` (translate.cc).  All return
//! `Err(KunaError::Lowlevel)` with a "not yet ported" explanation where the
//! C++ has real behavior, and exact C++ error strings where the C++ throws.

use std::cell::Cell;
use std::collections::btree_map::Entry;
use std::collections::BTreeMap;
use std::rc::Rc;

use crate::address::{calc_mask, Address, AddrSpacePtr};
use crate::error::{KunaError, KunaResult};
use crate::marshal::{
    cxx_strtoul, AttributeId, Decoder, ElementId, Encoder, ATTRIB_BIGENDIAN, ATTRIB_INDEX,
    ATTRIB_NAME, ATTRIB_OFFSET, ATTRIB_SIZE, ATTRIB_SPACE, ATTRIB_WORDSIZE,
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

/// Marshaling element \<space_overlay> (from `translate.cc`, needed by
/// `OverlaySpace::decode`; the rest of the translate id table arrives with
/// the sleigh wave).
pub const ELEM_SPACE_OVERLAY: ElementId = ElementId::new("space_overlay", 34);

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

/// The dispatch discriminant standing in for the C++ `AddrSpace` subclass
/// vtable: each C++ `virtual` override becomes a match arm on this kind.
#[derive(Debug)]
enum AddrSpaceKind {
    /// A concrete base-class `AddrSpace` (also IPTR_SPACEBASE placeholders
    /// until `SpacebaseSpace` is ported)
    Base,
    /// `ConstantSpace`
    Constant,
    /// `OtherSpace`
    Other,
    /// `UniqueSpace`
    Unique,
    /// `JoinSpace`
    Join,
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
    /// (`SpacebaseSpace`, which overrides this, is not yet ported.)
    pub fn num_spacebase(&self) -> i32 {
        0
    }

    /// Return \b true if a stack in this space grows negative.
    ///
    /// For stack (or other spacebase) spaces, this routine returns \b true
    /// if the space can viewed as a stack and a \b push operation causes the
    /// spacebase pointer to be decreased (grow negative).
    /// (`SpacebaseSpace`, which overrides this, is not yet ported.)
    pub fn stack_grows_negative(&self) -> bool {
        true
    }

    /// Return this space's containing space (if any).
    ///
    /// If this space is virtual, then this routine returns the containing
    /// address space, otherwise it returns None.
    pub fn get_contain(&self) -> Option<&Rc<AddrSpace>> {
        match &self.kind {
            AddrSpaceKind::Overlay { base_space } => base_space.as_ref(),
            _ => None,
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
            AddrSpaceKind::Join => Err(KunaError::lowlevel(
                "kuna rust port: JoinSpace::overlapJoin requires JoinRecord support (translate wave)",
            )),
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
            AddrSpaceKind::Join => Err(KunaError::lowlevel(
                "kuna rust port: JoinSpace::encodeAttributes requires JoinRecord support (translate wave)",
            )),
            AddrSpaceKind::Fspec => Err(KunaError::lowlevel(
                "kuna rust port: FspecSpace::encodeAttributes requires FuncCallSpecs (fspec wave)",
            )),
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
            AddrSpaceKind::Join => self.encode_attributes(encoder, offset),
            AddrSpaceKind::Fspec => Err(KunaError::lowlevel(
                "kuna rust port: FspecSpace::encodeAttributes requires FuncCallSpecs (fspec wave)",
            )),
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
            AddrSpaceKind::Join => Err(KunaError::lowlevel(
                "kuna rust port: JoinSpace::decodeAttributes requires JoinRecord support (translate wave)",
            )),
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
            AddrSpaceKind::Join => Err(KunaError::lowlevel(
                "kuna rust port: JoinSpace::printRaw requires JoinRecord support (translate wave)",
            )),
            AddrSpaceKind::Fspec => Err(KunaError::lowlevel(
                "kuna rust port: FspecSpace::printRaw requires FuncCallSpecs (fspec wave)",
            )),
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
    /// (kuna rust) The register-name path requires `Translate` (sleigh
    /// wave); without it `getRegister` always throws, so only the C++ catch
    /// branch (absolute offset parsing) is live and transcribed here.  The
    /// `manage` back-pointer becomes an explicit parameter.
    pub fn read(&self, s: &str, size: &mut i32, manage: &AddrSpaceManager) -> KunaResult<u64> {
        if let AddrSpaceKind::Join = self.kind {
            return Err(KunaError::lowlevel(
                "kuna rust port: JoinSpace::read requires JoinRecord support (translate wave)",
            ));
        }
        let bytes = s.as_bytes();
        let append = bytes.iter().position(|&c| c == b':' || c == b'+');
        // catch(LowlevelError): name doesn't exist
        let (raw, consumed) = cxx_strtoul(bytes);
        let offset = Self::address_to_byte(raw, self.wordsize);
        if consumed == bytes.len() {
            // If no size or offset override: return "natural" size
            *size = manage.get_default_size();
            return Ok(offset);
        }
        *size = manage.get_default_size();
        if let Some(append) = append {
            let mut offset = offset;
            let expsize = get_offset_size(&bytes[append..], &mut offset);
            if expsize != -1 {
                *size = expsize;
                return Ok(offset);
            }
            return Ok(offset);
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
            AddrSpaceKind::Join => Err(KunaError::lowlevel("Should never decode join space")),
            AddrSpaceKind::Fspec => {
                Err(KunaError::lowlevel("Should never decode fspec space from stream"))
            }
            AddrSpaceKind::Iop => {
                Err(KunaError::lowlevel("Should never decode iop space from stream"))
            }
            AddrSpaceKind::Overlay { .. } => self.decode_overlay(decoder),
            _ => {
                // Multiple tags: <space>, <space_other>, <space_unique>
                let elem_id = decoder.open_element()?;
                self.decode_basic_attributes(decoder)?;
                decoder.close_element(elem_id)
            }
        }
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
    /// (C++ private MAX_PIECES; recorded for the deferred JoinRecord port)
    #[allow(dead_code)] // consumed by the deferred JoinRecord methods
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
        space.kind = AddrSpaceKind::Join;
        // This is a virtual space; never heritaged, but does dead-code
        // analysis
        space.clear_flags(fl::heritaged);
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
    /// Constructor (C++ only has the decode form)
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

/// \brief A manager for different address spaces
///
/// Allows creation, lookup by name, lookup by shortcut and iteration over
/// address spaces.
///
/// This is the **minimal** port of the C++ `AddrSpaceManager`
/// (translate.hh/.cc): registration (`insert_space`, shortcut assignment,
/// the default-space setters) and lookup (by name, shortcut, index, plus the
/// special-space accessors needed by `Decoder::readSpace`/
/// `Encoder::writeSpace`).  The join-record machinery (`findAddJoin`,
/// `findJoin`, `renormalizeJoinAddress`, ...), the resolver list, and
/// `decodeSpace`/`decodeSpaces` arrive with the full Translate port in the
/// sleigh wave.
#[derive(Debug, Default)]
pub struct AddrSpaceManager {
    /// Every space we know about for this architecture
    baselist: Vec<Option<Rc<AddrSpace>>>,
    /// Map from name -> space (C++ `map<string,AddrSpace *>`)
    name2space: BTreeMap<String, Rc<AddrSpace>>,
    /// Map from shortcut -> space (C++ `map<int4,AddrSpace *>`; the key is
    /// the C++ (signed) char converted to int4)
    shortcut2space: BTreeMap<i32, Rc<AddrSpace>>,
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
}

/// The C++ `map<int4,AddrSpace*>` shortcut key is the (signed, on x86)
/// `char` converted to `int4`: bytes >= 0x80 become negative keys.
fn cxx_char_to_int(c: u8) -> i32 {
    (c as i8) as i32
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
