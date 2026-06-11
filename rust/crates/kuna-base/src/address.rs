//! Port of `decompiler/cpp/address.hh` + `address.cc` (W1, item
//! `w1-base-space-address`) — classes for specifying addresses and other
//! low-level constants.
//!
//! All addresses are absolute and there are no registers in CPUI. However,
//! all addresses are prefixed with an "immutable" pointer, which can specify
//! a separate RAM space, a register space, an i/o space etc.  Thus a
//! translation from a real machine language will typically simulate
//! registers by placing them in their own space, separate from RAM.
//! Indirection (i.e. pointers) must be simulated through the LOAD and STORE
//! ops.
//!
//! Pointer-model notes (vs C++):
//!
//! - The C++ `AddrSpace *base` member of `Address` admits two sentinel
//!   values besides real spaces: the null pointer (invalid / `m_minimal`)
//!   and `~0` (`m_maximal`).  [`AddrSpacePtr`] models all three states.
//! - C++ `Address::operator==` compares the raw space *pointer*;
//!   `operator<` orders spaces by *index*.  Both are transcribed exactly
//!   (`PartialEq` is `Rc::ptr_eq`-based, `Ord` is index-based).  They
//!   coincide — and the Rust `Ord`/`Eq` consistency contract holds — under
//!   the standing invariant that all compared spaces belong to one
//!   `AddrSpaceManager` (a registered index identifies a unique space).
//! - `SeqNum::operator==` compares only the `uniq` field while `operator<`
//!   orders by (pc, uniq); transcribed exactly with the same caveat (uniq is
//!   unique for the life of a PcodeOp, so equality implies equal pc).
//! - `Range`'s `operator<` (the `std::set` comparator) orders by (space
//!   index, first) only — `last` does not participate, so `RangeList`'s
//!   tree treats ranges with equal (space, first) as equivalent.  `Ord`/
//!   `PartialEq` transcribe that comparator; they are tree-equivalence, not
//!   structural equality (C++ defines no `operator==` at all).
//!
//! Deferred to later waves (loss-listed): `Address::decode` inlines the
//! (space,offset,size) scan of `VarnodeData::decode` (pcoderaw is a kuna-num
//! item); the register-name (`ATTRIB_NAME`) branches of the decode methods
//! and `Address::renormalize`'s join handling need `Translate`/`JoinRecord`
//! and return `Err(KunaError::Lowlevel)` "not yet ported" markers.

use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::ops::Bound;
use std::rc::Rc;

use crate::error::{KunaError, KunaResult};
use crate::marshal::{
    AttributeId, Decoder, ElementId, Encoder, ATTRIB_NAME, ATTRIB_SPACE,
};
use crate::space::{spacetype, AddrSpace, AddrSpaceManager};
use crate::types::Wrap;

pub const ATTRIB_FIRST: AttributeId = AttributeId::new("first", 27);
pub const ATTRIB_LAST: AttributeId = AttributeId::new("last", 28);
pub const ATTRIB_UNIQ: AttributeId = AttributeId::new("uniq", 29);

pub const ELEM_ADDR: ElementId = ElementId::new("addr", 11);
pub const ELEM_RANGE: ElementId = ElementId::new("range", 12);
pub const ELEM_RANGELIST: ElementId = ElementId::new("rangelist", 13);
pub const ELEM_REGISTER: ElementId = ElementId::new("register", 14);
pub const ELEM_SEQNUM: ElementId = ElementId::new("seqnum", 15);
pub const ELEM_VARNODE: ElementId = ElementId::new("varnode", 16);

// ---------------------------------------------------------------------------
// AddrSpacePtr — the C++ `AddrSpace *` of an Address, including sentinels
// ---------------------------------------------------------------------------

/// The space pointer of an [`Address`]: a real space, the null pointer
/// (invalid / minimal address), or the `(AddrSpace *)~0` sentinel (maximal
/// address).
#[derive(Debug, Clone)]
pub enum AddrSpacePtr {
    /// C++ null pointer (deliberately invalid / `m_minimal`)
    Null,
    /// A real address space
    Spc(Rc<AddrSpace>),
    /// C++ `(AddrSpace *) ~((uintp)0)` (`m_maximal` sentinel)
    Max,
}

impl PartialEq for AddrSpacePtr {
    /// Raw pointer equality, as in C++ (`Rc::ptr_eq` for real spaces).
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (AddrSpacePtr::Null, AddrSpacePtr::Null) => true,
            (AddrSpacePtr::Max, AddrSpacePtr::Max) => true,
            (AddrSpacePtr::Spc(a), AddrSpacePtr::Spc(b)) => Rc::ptr_eq(a, b),
            _ => false,
        }
    }
}
impl Eq for AddrSpacePtr {}

// ---------------------------------------------------------------------------
// Address
// ---------------------------------------------------------------------------

/// An enum for specifying extremal addresses (C++ `Address::mach_extreme`)
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum mach_extreme {
    /// Smallest possible address
    m_minimal,
    /// Biggest possible address
    m_maximal,
}

/// \brief A low-level machine address for labelling bytes and data.
///
/// All data that can be manipulated within the processor reverse engineering
/// model can be labelled with an Address. It is simply an address space
/// (AddrSpace) and an offset within that space.  Note that processor
/// registers are typically modelled by creating a dedicated address space
/// for them, as distinct from RAM say, and then specifying certain addresses
/// within the register space that correspond to particular registers.
/// However, an arbitrary address could refer to anything: RAM, ROM, cpu
/// register, data segment, coprocessor, stack, nvram, etc.  An Address
/// represents an offset \e only, not an offset and length.
#[derive(Debug, Clone)]
pub struct Address {
    /// Pointer to our address space
    base: AddrSpacePtr,
    /// Offset (in bytes)
    offset: u64,
}

impl Default for Address {
    /// C++ `Address(void)`: a deliberately invalid address.  (The C++
    /// leaves `offset` uninitialized; it is zeroed here.)
    fn default() -> Self {
        Address { base: AddrSpacePtr::Null, offset: 0 }
    }
}

impl Address {
    /// This is the basic Address constructor.
    /// \param id is the space containing the address
    /// \param off is the offset of the address
    pub fn new(id: Rc<AddrSpace>, off: u64) -> Address {
        Address { base: AddrSpacePtr::Spc(id), offset: off }
    }

    /// Create an invalid address (C++ `Address(void)`)
    pub fn new_invalid() -> Address {
        Address::default()
    }

    /// Initialize an extremal address.
    ///
    /// Some data structures sort on an Address, and it is convenient to be
    /// able to create an Address that is either bigger than or smaller than
    /// all other Addresses.
    /// \param ex is either \e m_minimal or \e m_maximal
    pub fn new_extreme(ex: mach_extreme) -> Address {
        if ex == mach_extreme::m_minimal {
            Address { base: AddrSpacePtr::Null, offset: 0 }
        } else {
            Address { base: AddrSpacePtr::Max, offset: u64::MAX }
        }
    }

    /// Construct from an explicit space pointer (which may be a sentinel)
    /// and offset — the C++ `Address(AddrSpace *,uintb)` with a raw pointer.
    pub fn from_space_ptr(base: AddrSpacePtr, off: u64) -> Address {
        Address { base, offset: off }
    }

    /// Determine if this is an invalid address. This only detects
    /// \e deliberate invalid addresses.
    pub fn is_invalid(&self) -> bool {
        matches!(self.base, AddrSpacePtr::Null)
    }

    /// The real space behind this address; panics on the null/max sentinels
    /// (where C++ would dereference an invalid pointer — UB, ADR 0004).
    fn space(&self) -> &Rc<AddrSpace> {
        match &self.base {
            AddrSpacePtr::Spc(s) => s,
            _ => panic!("Address: dereferencing a null/sentinel space pointer (C++ UB)"),
        }
    }

    /// Get the number of bytes needed to encode the \e offset for this
    /// address.
    pub fn get_addr_size(&self) -> i32 {
        self.space().get_addr_size() as i32
    }

    /// Determine if data stored at this address is big endian encoded.
    pub fn is_big_endian(&self) -> bool {
        self.space().is_big_endian()
    }

    /// Write a short-hand or debug version of this address to a stream.
    pub fn print_raw(&self, s: &mut String) -> KunaResult<()> {
        match &self.base {
            AddrSpacePtr::Null => {
                s.push_str("invalid_addr");
                Ok(())
            }
            AddrSpacePtr::Spc(base) => base.print_raw(s, self.offset),
            // C++ only checks for null and would dereference the ~0 sentinel
            AddrSpacePtr::Max => {
                panic!("Address::printRaw on m_maximal sentinel (C++ UB)")
            }
        }
    }

    /// Read in the address from a string (console syntax; see
    /// `AddrSpace::read`).  Mutates the offset and returns any size
    /// associated with the parsed string.  The `manage` parameter replaces
    /// the C++ manager back-pointer.
    pub fn read(&mut self, s: &str, manage: &AddrSpaceManager) -> KunaResult<i32> {
        let mut sz: i32 = 0;
        let off = self.space().read(s, &mut sz, manage)?;
        self.offset = off;
        Ok(sz)
    }

    /// Get the address space, or `None` for the null/max sentinels (C++
    /// returns the raw — possibly sentinel — pointer; use
    /// [`Address::base_ptr`] to observe the sentinel states).
    pub fn get_space(&self) -> Option<&Rc<AddrSpace>> {
        match &self.base {
            AddrSpacePtr::Spc(s) => Some(s),
            _ => None,
        }
    }

    /// The raw space pointer including sentinel states.
    pub fn base_ptr(&self) -> &AddrSpacePtr {
        &self.base
    }

    /// Get the offset of the address as an integer.
    pub fn get_offset(&self) -> u64 {
        self.offset
    }

    /// Get the shortcut character for the address space.
    ///
    /// Each address has a shortcut character associated with it for use with
    /// the read and printRaw methods.
    pub fn get_shortcut(&self) -> char {
        self.space().get_shortcut()
    }

    /// Determine if \e op2 range contains \b this range.
    ///
    /// Return \b true if the range starting at \b this extending the given
    /// number of bytes is contained by the second given range.
    /// \param sz is the given number of bytes in \b this range
    /// \param op2 is the start of the second given range
    /// \param sz2 is the number of bytes in the second given range
    pub fn contained_by(&self, sz: i32, op2: &Address, sz2: i32) -> bool {
        if self.base != op2.base {
            return false;
        }
        if op2.offset > self.offset {
            return false;
        }
        // offset + (sz-1): the int4 (sz-1) sign-extends into the uintb sum
        let off1 = self.offset.wadd((sz - 1) as i64 as u64);
        let off2 = op2.offset.wadd((sz2 - 1) as i64 as u64);
        off2 >= off1
    }

    /// Determine if \e op2 is the least significant part of \e this.
    ///
    /// Return -1 if (\e op2,\e sz2) is not properly contained in
    /// (\e this,\e sz).  If it is contained, return the endian aware offset
    /// of (\e op2,\e sz2).  I.e. if the least significant byte of the \e op2
    /// range falls on the least significant byte of the \e this range,
    /// return 0.  If it intersects the second least significant, return 1,
    /// etc.  The -forceleft- toggle causes the check to be made against the
    /// left (lowest address) side of the container, regardless of the
    /// endianness.  I.e. it forces a little endian interpretation.
    pub fn justified_contain(&self, sz: i32, op2: &Address, sz2: i32, forceleft: bool) -> i32 {
        if self.base != op2.base {
            return -1;
        }
        if op2.offset < self.offset {
            return -1;
        }
        let off1 = self.offset.wadd((sz - 1) as i64 as u64);
        let off2 = op2.offset.wadd((sz2 - 1) as i64 as u64);
        if off2 > off1 {
            return -1;
        }
        if self.space().is_big_endian() && !forceleft {
            return off1.wsub(off2) as i32;
        }
        op2.offset.wsub(self.offset) as i32
    }

    /// Determine how \b this address falls in a given address range.
    ///
    /// If \e this + \e skip falls in the range \e op to \e op + \e size,
    /// then a non-negative integer is returned indicating where in the
    /// interval it falls. I.e. if \e this + \e skip == \e op, then 0 is
    /// returned. Otherwise -1 is returned.
    pub fn overlap(&self, skip: i32, op: &Address, size: i32) -> i32 {
        if self.base != op.base {
            return -1; // Must be in same address space to overlap
        }
        let base = self.space();
        if base.get_type() == spacetype::IPTR_CONSTANT {
            return -1; // Must not be constants
        }
        // offset + skip - op.offset with int4 skip sign-extending
        let dist = base.wrap_offset(self.offset.wadd(skip as i64 as u64).wsub(op.offset));
        // mixed comparison: uintb dist vs int4 size (sign-extended up)
        if dist >= size as i64 as u64 {
            return -1; // but must fall before op+size
        }
        dist as i32
    }

    /// Determine how \b this falls in a possible \e join space address range.
    ///
    /// This method is equivalent to Address::overlap, but a range in the
    /// \e join space can be considered overlapped with its constituent
    /// pieces.
    pub fn overlap_join(&self, skip: i32, op: &Address, size: i32) -> KunaResult<i32> {
        op.space().overlap_join(op.get_offset(), size, self.space(), self.offset, skip)
    }

    /// Does \e this form a contiguous range with \e loaddr.
    ///
    /// Does the location \e this, \e sz form a contiguous region to
    /// \e loaddr, \e losz, where \e this forms the most significant piece of
    /// the logical whole.
    pub fn is_contiguous(&self, sz: i32, loaddr: &Address, losz: i32) -> bool {
        if self.base != loaddr.base {
            return false;
        }
        let base = self.space();
        if base.is_big_endian() {
            let nextoff = base.wrap_offset(self.offset.wadd(sz as i64 as u64));
            if nextoff == loaddr.offset {
                return true;
            }
        } else {
            let nextoff = base.wrap_offset(loaddr.offset.wadd(losz as i64 as u64));
            if nextoff == self.offset {
                return true;
            }
        }
        false
    }

    /// Is this a \e constant \e value.
    ///
    /// All constant values are represented as an offset into the \e constant
    /// \e space.
    pub fn is_constant(&self) -> bool {
        self.space().get_type() == spacetype::IPTR_CONSTANT
    }

    /// Make sure there is a backing JoinRecord if \b this is in the \e join
    /// space.
    ///
    /// If \b this is (originally) a \e join address, reevaluate it in terms
    /// of its new \e offset and \e size, changing the space and offset if
    /// necessary.
    pub fn renormalize(&mut self, _size: i32) -> KunaResult<()> {
        if self.space().get_type() == spacetype::IPTR_JOIN {
            return Err(KunaError::lowlevel(
                "kuna rust port: renormalizeJoinAddress requires JoinRecord support (translate wave)",
            ));
        }
        Ok(())
    }

    /// Is this a \e join \e value (a set of joined memory locations).
    pub fn is_join(&self) -> bool {
        self.space().get_type() == spacetype::IPTR_JOIN
    }

    /// Encode \b this to a stream.
    ///
    /// Save an \<addr\> element corresponding to this address.  The exact
    /// format is determined by the address space, but this generally has a
    /// \e space and an \e offset attribute.
    pub fn encode(&self, encoder: &mut dyn Encoder) -> KunaResult<()> {
        encoder.open_element(&ELEM_ADDR);
        match &self.base {
            AddrSpacePtr::Null => {}
            AddrSpacePtr::Spc(base) => base.encode_attributes(encoder, self.offset)?,
            // C++ only guards against null; the ~0 sentinel would be
            // dereferenced (UB)
            AddrSpacePtr::Max => panic!("Address::encode on m_maximal sentinel (C++ UB)"),
        }
        encoder.close_element(&ELEM_ADDR);
        Ok(())
    }

    /// Encode \b this and a size to a stream.
    ///
    /// The element will include an extra \e size attribute so that it can
    /// describe an entire memory range.
    pub fn encode_sized(&self, encoder: &mut dyn Encoder, size: i32) -> KunaResult<()> {
        encoder.open_element(&ELEM_ADDR);
        match &self.base {
            AddrSpacePtr::Null => {}
            AddrSpacePtr::Spc(base) => base.encode_attributes_sized(encoder, self.offset, size)?,
            AddrSpacePtr::Max => panic!("Address::encode on m_maximal sentinel (C++ UB)"),
        }
        encoder.close_element(&ELEM_ADDR);
        Ok(())
    }

    /// Decode an address from a stream.
    ///
    /// This is usually used to decode an address from an \b \<addr\>
    /// element, but any element can be used if it has the appropriate
    /// attributes
    ///    - \e space indicates the address space of the tag
    ///    - \e offset indicates the offset within the space
    ///
    /// (The C++ routes through `VarnodeData::decode` — pcoderaw is a
    /// kuna-num item, so its (space,offset,size) attribute scan is inlined
    /// here.  The \e name register-form requires `Translate` and is
    /// deferred.)
    pub fn decode(decoder: &mut dyn Decoder) -> KunaResult<Address> {
        let (space, offset, _size) = decode_varnode_attributes(decoder)?;
        Ok(match space {
            Some(spc) => Address::new(spc, offset),
            None => Address::from_space_ptr(AddrSpacePtr::Null, offset),
        })
    }

    /// Decode an address and size from a stream.
    ///
    /// If a size is recovered it is returned alongside the address.
    pub fn decode_sized(decoder: &mut dyn Decoder) -> KunaResult<(Address, i32)> {
        let (space, offset, size) = decode_varnode_attributes(decoder)?;
        let addr = match space {
            Some(spc) => Address::new(spc, offset),
            None => Address::from_space_ptr(AddrSpacePtr::Null, offset),
        };
        // uint4 -> int4 conversion as in `size = var.size`
        Ok((addr, size as i32))
    }
}

/// The (space, offset, size) attribute scan of `VarnodeData::decode`
/// (pcoderaw.cc), inlined pending the kuna-num pcoderaw port.
fn decode_varnode_attributes(
    decoder: &mut dyn Decoder,
) -> KunaResult<(Option<Rc<AddrSpace>>, u64, u32)> {
    let elem_id = decoder.open_element()?;
    let mut space: Option<Rc<AddrSpace>> = None;
    let mut size: u32 = 0;
    let mut offset: u64 = 0;
    loop {
        let attrib_id = decoder.get_next_attribute_id()?;
        if attrib_id == 0 {
            break; // It's possible to have no attributes in an <addr/> tag
        }
        if attrib_id == ATTRIB_SPACE {
            let spc = decoder.read_space()?;
            decoder.rewind_attributes();
            offset = spc.decode_attributes(decoder, &mut size)?;
            space = Some(spc);
            break;
        } else if attrib_id == ATTRIB_NAME {
            return Err(KunaError::lowlevel(
                "kuna rust port: register-name address decode requires Translate (sleigh wave)",
            ));
        }
    }
    decoder.close_element(elem_id)?;
    Ok((space, offset, size))
}

impl PartialEq for Address {
    /// Check if two addresses are equal. I.e. if their address space and
    /// offset are the same.  (Space comparison is by pointer, as in C++;
    /// see the module docs for the one-manager consistency invariant.)
    fn eq(&self, op2: &Address) -> bool {
        self.base == op2.base && self.offset == op2.offset
    }
}
impl Eq for Address {}

impl Ord for Address {
    /// Do an ordering comparison of two addresses.  Addresses are sorted
    /// first on space, then on offset.  So two addresses in the same space
    /// compare naturally based on their offset, but addresses in different
    /// spaces also compare. Different spaces are ordered by their index.
    /// The null pointer sorts before everything and the ~0 sentinel after
    /// everything, exactly as in the C++ `operator<`/`operator<=` pair.
    fn cmp(&self, op2: &Address) -> Ordering {
        // C++ `if (base != op2.base)`: distinct pointers ...
        if self.base != op2.base {
            let rank = |b: &AddrSpacePtr| -> i32 {
                match b {
                    AddrSpacePtr::Null => 0,
                    AddrSpacePtr::Spc(_) => 1,
                    AddrSpacePtr::Max => 2,
                }
            };
            let (r1, r2) = (rank(&self.base), rank(&op2.base));
            if r1 != r2 {
                return r1.cmp(&r2);
            }
            if let (AddrSpacePtr::Spc(a), AddrSpacePtr::Spc(b)) = (&self.base, &op2.base) {
                let ord = a.get_index().cmp(&b.get_index());
                if ord != Ordering::Equal {
                    return ord;
                }
                // Distinct spaces sharing an index: C++ treats these as
                // equivalent under operator< (cannot happen within one
                // manager); fall through to the offset comparison so the
                // order stays total.
            }
        }
        self.offset.cmp(&op2.offset)
    }
}

impl PartialOrd for Address {
    fn partial_cmp(&self, op2: &Address) -> Option<Ordering> {
        Some(self.cmp(op2))
    }
}

/// Increment address by a number of bytes (C++ `operator+(int8)`).
///
/// The addition takes into account the \e size of the address space, and the
/// Address will wrap around if necessary.
impl core::ops::Add<i64> for &Address {
    type Output = Address;
    fn add(self, off: i64) -> Address {
        let base = self.space();
        Address::new(Rc::clone(base), base.wrap_offset(self.offset.wadd(off as u64)))
    }
}

/// Decrement address by a number of bytes (C++ `operator-(int8)`).
impl core::ops::Sub<i64> for &Address {
    type Output = Address;
    fn sub(self, off: i64) -> Address {
        let base = self.space();
        Address::new(Rc::clone(base), base.wrap_offset(self.offset.wsub(off as u64)))
    }
}

// ---------------------------------------------------------------------------
// SeqNum
// ---------------------------------------------------------------------------

/// \brief A class for uniquely labelling and comparing PcodeOps
///
/// Different PcodeOps generated by a single machine instruction can only be
/// labelled with a single Address. But PcodeOps must be distinguishable and
/// compared for execution order.  A SeqNum extends the address for a PcodeOp
/// to include:
///   - A fixed \e time field, which is set at the time the PcodeOp is
///     created. The \e time field guarantees a unique SeqNum for the life of
///     the PcodeOp.
///   - An \e order field, which is guaranteed to be comparable for the
///     execution order of the PcodeOp within its basic block.  The \e order
///     field also provides uniqueness but may change over time if the syntax
///     tree is manipulated.
#[derive(Debug, Clone, Default)]
pub struct SeqNum {
    /// Program counter at start of instruction
    pc: Address,
    /// Number to guarantee uniqueness
    uniq: u32,
    /// Number for order comparisons within a block
    order: u32,
}

impl SeqNum {
    /// Create a sequence number with a specific \e time field
    pub fn new(a: Address, b: u32) -> SeqNum {
        // (C++ leaves `order` uninitialized here; zeroed)
        SeqNum { pc: a, uniq: b, order: 0 }
    }

    /// Create an extremal sequence number
    pub fn new_extreme(ex: mach_extreme) -> SeqNum {
        let uniq = if ex == mach_extreme::m_minimal { 0 } else { u32::MAX };
        SeqNum { pc: Address::new_extreme(ex), uniq, order: 0 }
    }

    /// Get the address portion of a sequence number
    pub fn get_addr(&self) -> &Address {
        &self.pc
    }

    /// Get the \e time field of a sequence number
    pub fn get_time(&self) -> u32 {
        self.uniq
    }

    /// Get the \e order field of a sequence number
    pub fn get_order(&self) -> u32 {
        self.order
    }

    /// Set the \e order field of a sequence number
    pub fn set_order(&mut self, ord: u32) {
        self.order = ord;
    }

    /// Encode a SeqNum to a stream
    pub fn encode(&self, encoder: &mut dyn Encoder) -> KunaResult<()> {
        encoder.open_element(&ELEM_SEQNUM);
        self.pc
            .get_space()
            .expect("SeqNum::encode on invalid address (C++ UB)")
            .encode_attributes(encoder, self.pc.get_offset())?;
        encoder.write_unsigned_integer(&ATTRIB_UNIQ, self.uniq as u64);
        encoder.close_element(&ELEM_SEQNUM);
        Ok(())
    }

    /// Decode a SeqNum from a stream
    pub fn decode(decoder: &mut dyn Decoder) -> KunaResult<SeqNum> {
        let mut uniq: u32 = u32::MAX; // ~((uintm)0)
        let elem_id = decoder.open_element_id(&ELEM_SEQNUM)?;
        let pc = Address::decode(decoder)?; // Recover address
        loop {
            let attrib_id = decoder.get_next_attribute_id()?;
            if attrib_id == 0 {
                break;
            }
            if attrib_id == ATTRIB_UNIQ {
                // uintb -> uintm truncating conversion
                uniq = decoder.read_unsigned_integer()? as u32;
                break;
            }
        }
        decoder.close_element(elem_id)?;
        Ok(SeqNum::new(pc, uniq))
    }

    /// Write out a SeqNum in human readable form to a stream
    /// (C++ `operator<<`)
    pub fn print_raw(&self, s: &mut String) -> KunaResult<()> {
        self.pc.print_raw(s)?;
        s.push(':');
        s.push_str(&format!("{}", self.uniq));
        Ok(())
    }
}

impl PartialEq for SeqNum {
    /// Compare two sequence numbers for equality — by the \e uniq field
    /// ONLY, as in C++ (`uniq` is unique for the life of a PcodeOp, so
    /// within one function equal uniq implies equal pc).
    fn eq(&self, op2: &SeqNum) -> bool {
        self.uniq == op2.uniq
    }
}
impl Eq for SeqNum {}

impl Ord for SeqNum {
    /// Compare two sequence numbers with their natural order:
    /// `if (pc == op2.pc) return uniq < op2.uniq; return pc < op2.pc;`
    /// (The `order` field never participates.)
    fn cmp(&self, op2: &SeqNum) -> Ordering {
        match self.pc.cmp(&op2.pc) {
            Ordering::Equal => self.uniq.cmp(&op2.uniq),
            ord => ord,
        }
    }
}

impl PartialOrd for SeqNum {
    fn partial_cmp(&self, op2: &SeqNum) -> Option<Ordering> {
        Some(self.cmp(op2))
    }
}

// ---------------------------------------------------------------------------
// Range / RangeProperties / RangeList
// ---------------------------------------------------------------------------

/// \brief A contiguous range of bytes in some address space
#[derive(Debug, Clone)]
pub struct Range {
    /// Space containing range
    spc: Rc<AddrSpace>,
    /// Offset of first byte in \b this Range
    first: u64,
    /// Offset of last byte in \b this Range
    last: u64,
}

impl Range {
    /// \brief Construct a Range from offsets
    ///
    /// Offsets must expressed in \e bytes as opposed to addressable \e words
    /// \param s is the address space containing the range
    /// \param f is the offset of the first byte in the range
    /// \param l is the offset of the last byte in the range
    pub fn new(s: Rc<AddrSpace>, f: u64, l: u64) -> Range {
        Range { spc: s, first: f, last: l }
    }

    /// Construct range out of basic properties
    /// (C++ `Range(const RangeProperties &,const AddrSpaceManager *)`).
    pub fn from_properties(
        properties: &RangeProperties,
        manage: &AddrSpaceManager,
    ) -> KunaResult<Range> {
        if properties.is_register {
            // Requires Translate::getRegister
            return Err(KunaError::lowlevel(
                "kuna rust port: register-name Range requires Translate (sleigh wave)",
            ));
        }
        let spc = match manage.get_space_by_name(&properties.space_name) {
            Some(spc) => Rc::clone(spc),
            None => {
                return Err(KunaError::lowlevel(format!(
                    "Undefined space: {}",
                    properties.space_name
                )))
            }
        };
        // (The C++ has a second, unreachable null check here — "No address
        // space indicated in range tag" — not transcribed.)
        let first = properties.first;
        let mut last = properties.last;
        if !properties.seen_last {
            last = spc.get_highest();
        }
        if first > spc.get_highest() || last > spc.get_highest() || last < first {
            return Err(KunaError::lowlevel("Illegal range tag"));
        }
        Ok(Range { spc, first, last })
    }

    /// Get the address space containing \b this Range
    pub fn get_space(&self) -> &Rc<AddrSpace> {
        &self.spc
    }

    /// Get the offset of the first byte in \b this Range
    pub fn get_first(&self) -> u64 {
        self.first
    }

    /// Get the offset of the last byte in \b this Range
    pub fn get_last(&self) -> u64 {
        self.last
    }

    /// Get the address of the first byte
    pub fn get_first_addr(&self) -> Address {
        Address::new(Rc::clone(&self.spc), self.first)
    }

    /// Get the address of the last byte
    pub fn get_last_addr(&self) -> Address {
        Address::new(Rc::clone(&self.spc), self.last)
    }

    /// Get address of first byte after \b this.
    ///
    /// Get the last address +1, updating the space, or returning the
    /// extremal address if necessary.
    /// \param manage is used to fetch the next address space
    pub fn get_last_addr_open(&self, manage: &AddrSpaceManager) -> Address {
        let mut curspc = AddrSpacePtr::Spc(Rc::clone(&self.spc));
        let mut curlast = self.last;
        if curlast == self.spc.get_highest() {
            curspc = manage.get_next_space_in_order(&curspc);
            curlast = 0;
        } else {
            curlast += 1;
        }
        if let AddrSpacePtr::Null = curspc {
            return Address::new_extreme(mach_extreme::m_maximal);
        }
        // NOTE: curspc may be the ~0 sentinel here, exactly as in C++
        Address::from_space_ptr(curspc, curlast)
    }

    /// Determine if the address is in \b this Range
    pub fn contains(&self, addr: &Address) -> bool {
        // C++ compares the raw space pointers
        match addr.get_space() {
            Some(spc) if Rc::ptr_eq(&self.spc, spc) => {}
            _ => return false,
        }
        if self.first > addr.get_offset() {
            return false;
        }
        if self.last < addr.get_offset() {
            return false;
        }
        true
    }

    /// Print \b this Range to a stream.
    /// Output a description of this Range like:  ram: 7f-9c
    pub fn print_bounds(&self, s: &mut String) {
        s.push_str(self.spc.get_name());
        s.push_str(": ");
        s.push_str(&format!("{:x}-{:x}", self.first, self.last));
    }

    /// Encode \b this Range to a stream as a \<range> element.
    pub fn encode(&self, encoder: &mut dyn Encoder) {
        encoder.open_element(&ELEM_RANGE);
        encoder.write_space(&ATTRIB_SPACE, &self.spc);
        encoder.write_unsigned_integer(&ATTRIB_FIRST, self.first);
        encoder.write_unsigned_integer(&ATTRIB_LAST, self.last);
        encoder.close_element(&ELEM_RANGE);
    }

    /// Restore \b this from a stream.
    /// Reconstruct this object from a \<range> or \<register> element.
    /// (The C++ mutates a default-constructed Range; the Rust constructs.)
    pub fn decode(decoder: &mut dyn Decoder) -> KunaResult<Range> {
        let elem_id = decoder.open_element()?;
        if elem_id != ELEM_RANGE && elem_id != ELEM_REGISTER {
            return Err(KunaError::decoder("Expecting <range> or <register> element"));
        }
        let range = Self::decode_from_attributes(decoder)?;
        decoder.close_element(elem_id)?;
        Ok(range)
    }

    /// Read \b this from attributes on another element.
    /// Reconstruct from attributes that may not be part of a \<range>
    /// element.
    pub fn decode_from_attributes(decoder: &mut dyn Decoder) -> KunaResult<Range> {
        let mut spc: Option<Rc<AddrSpace>> = None;
        let mut seen_last = false;
        let mut first: u64 = 0;
        let mut last: u64 = 0;
        loop {
            let attrib_id = decoder.get_next_attribute_id()?;
            if attrib_id == 0 {
                break;
            }
            if attrib_id == ATTRIB_SPACE {
                spc = Some(decoder.read_space()?);
            } else if attrib_id == ATTRIB_FIRST {
                first = decoder.read_unsigned_integer()?;
            } else if attrib_id == ATTRIB_LAST {
                last = decoder.read_unsigned_integer()?;
                seen_last = true;
            } else if attrib_id == ATTRIB_NAME {
                // Requires Translate::getRegister
                return Err(KunaError::lowlevel(
                    "kuna rust port: register-name range decode requires Translate (sleigh wave)",
                ));
            }
        }
        let spc = match spc {
            Some(spc) => spc,
            None => {
                return Err(KunaError::lowlevel("No address space indicated in range tag"))
            }
        };
        if !seen_last {
            last = spc.get_highest();
        }
        if first > spc.get_highest() || last > spc.get_highest() || last < first {
            return Err(KunaError::lowlevel("Illegal range tag"));
        }
        Ok(Range { spc, first, last })
    }
}

impl PartialEq for Range {
    /// Tree-equivalence under the C++ `std::set<Range>` comparator (space
    /// index and `first` only — `last` does not participate; C++ defines no
    /// `operator==` for Range).
    fn eq(&self, op2: &Range) -> bool {
        self.cmp(op2) == Ordering::Equal
    }
}
impl Eq for Range {}

impl Ord for Range {
    /// \brief Sorting operator for Ranges
    ///
    /// Compare based on address space, then the starting offset
    fn cmp(&self, op2: &Range) -> Ordering {
        if self.spc.get_index() != op2.spc.get_index() {
            return self.spc.get_index().cmp(&op2.spc.get_index());
        }
        self.first.cmp(&op2.first)
    }
}

impl PartialOrd for Range {
    fn partial_cmp(&self, op2: &Range) -> Option<Ordering> {
        Some(self.cmp(op2))
    }
}

/// \brief A partially parsed description of a Range
///
/// Class that allows \<range> tags to be parsed, when the address space
/// doesn't yet exist
#[derive(Debug, Clone, Default)]
pub struct RangeProperties {
    /// Name of the address space containing the range
    space_name: String,
    /// Offset of first byte in the Range
    first: u64,
    /// Offset of last byte in the Range
    last: u64,
    /// Range is specified as a register name
    is_register: bool,
    /// End of the range is actively specified
    seen_last: bool,
}

impl RangeProperties {
    /// Construct with default values (C++ `RangeProperties(void)`)
    pub fn new() -> Self {
        Self::default()
    }

    /// Decode \b this from a stream
    pub fn decode(&mut self, decoder: &mut dyn Decoder) -> KunaResult<()> {
        let elem_id = decoder.open_element()?;
        if elem_id != ELEM_RANGE && elem_id != ELEM_REGISTER {
            return Err(KunaError::decoder("Expecting <range> or <register> element"));
        }
        loop {
            let attrib_id = decoder.get_next_attribute_id()?;
            if attrib_id == 0 {
                break;
            }
            if attrib_id == ATTRIB_SPACE {
                self.space_name = String::from_utf8_lossy(&decoder.read_string()?).into_owned();
            } else if attrib_id == ATTRIB_FIRST {
                self.first = decoder.read_unsigned_integer()?;
            } else if attrib_id == ATTRIB_LAST {
                self.last = decoder.read_unsigned_integer()?;
                self.seen_last = true;
            } else if attrib_id == ATTRIB_NAME {
                self.space_name = String::from_utf8_lossy(&decoder.read_string()?).into_owned();
                self.is_register = true;
            }
        }
        decoder.close_element(elem_id)
    }
}

/// \brief A disjoint set of Ranges, possibly across multiple address spaces
///
/// This is a container for addresses. It maintains a disjoint list of Ranges
/// that cover all the addresses in the container.  Ranges can be inserted
/// and removed, but overlapping/adjacent ranges will get merged.
#[derive(Debug, Clone, Default)]
pub struct RangeList {
    /// The sorted list of Range objects
    tree: BTreeSet<Range>,
}

impl RangeList {
    /// Construct an empty container
    pub fn new() -> Self {
        Self::default()
    }

    /// Clear \b this container to empty
    pub fn clear(&mut self) {
        self.tree.clear();
    }

    /// Return \b true if \b this is empty
    pub fn empty(&self) -> bool {
        self.tree.is_empty()
    }

    /// Iterate over the disjoint Range objects (C++ begin()/end())
    pub fn iter(&self) -> impl Iterator<Item = &Range> {
        self.tree.iter()
    }

    /// Return the number of Range objects in container
    pub fn num_ranges(&self) -> i32 {
        self.tree.len() as i32
    }

    /// Insert a range of addresses.
    ///
    /// Insert a new Range merging as appropriate to maintain the disjoint
    /// cover.
    /// \param spc is the address space containing the new range
    /// \param first is the offset of the first byte in the new range
    /// \param last is the offset of the last byte in the new range
    pub fn insert_range(&mut self, spc: Rc<AddrSpace>, mut first: u64, mut last: u64) {
        // collect [iter1, iter2): C++ erases while merging bounds
        let to_erase = self.collect_overlap(&spc, first, last);
        for r in &to_erase {
            if r.first < first {
                first = r.first;
            }
            if r.last > last {
                last = r.last;
            }
            self.tree.remove(r);
        }
        self.tree.insert(Range::new(spc, first, last));
    }

    /// Remove a range of addresses.
    ///
    /// Remove/narrow/split existing Range objects to eliminate the indicated
    /// addresses while still maintaining a disjoint cover.
    pub fn remove_range(&mut self, spc: Rc<AddrSpace>, first: u64, last: u64) {
        // remove a range
        if self.tree.is_empty() {
            return; // Nothing to do
        }
        let to_erase = self.collect_overlap(&spc, first, last);
        for r in &to_erase {
            let a = r.first;
            let b = r.last;
            self.tree.remove(r);
            if a < first {
                self.tree.insert(Range::new(Rc::clone(&spc), a, first - 1));
            }
            if b > last {
                self.tree.insert(Range::new(Rc::clone(&spc), last + 1, b));
            }
        }
    }

    /// The shared `[iter1, iter2)` walk of the C++ insertRange/removeRange:
    /// `iter1` is the first range with `range.last >= first` (the
    /// upper-bound of (spc,first), possibly stepped back one), and `iter2`
    /// is the first range with `range.first > last`.
    fn collect_overlap(&self, spc: &Rc<AddrSpace>, first: u64, last: u64) -> Vec<Range> {
        let probe1 = Range::new(Rc::clone(spc), first, first);
        let probe2 = Range::new(Rc::clone(spc), last, last);
        let mut result: Vec<Range> = Vec::new();
        // we must have iter1.first > first; set iter1 to first range with
        // range.last >= first — it is either the upper bound or the one
        // before
        if let Some(prev) = self.tree.range((Bound::Unbounded, Bound::Included(&probe1))).next_back()
        {
            // C++ pointer comparison `(*iter1).spc != spc`
            if Rc::ptr_eq(&prev.spc, spc) && prev.last >= first {
                result.push(prev.clone());
            }
        }
        for r in self.tree.range((Bound::Excluded(&probe1), Bound::Unbounded)) {
            if *r > probe2 {
                break; // reached iter2
            }
            result.push(r.clone());
        }
        result
    }

    /// Merge another RangeList into \b this
    pub fn merge(&mut self, op2: &RangeList) {
        // Merge -op2- into this rangelist
        for range in op2.tree.iter() {
            self.insert_range(Rc::clone(&range.spc), range.first, range.last);
        }
    }

    /// Check containment of an address range.
    ///
    /// Make sure indicated range of addresses is \e contained in \b this
    /// RangeList.
    /// \param addr is the first Address in the target range
    /// \param size is the number of bytes in the target range
    pub fn in_range(&self, addr: &Address, size: i32) -> bool {
        if addr.is_invalid() {
            return true; // We don't really care
        }
        if self.tree.is_empty() {
            return false;
        }
        let spc = addr.get_space().expect("in_range on sentinel address (C++ UB)");
        let probe = Range::new(Rc::clone(spc), addr.get_offset(), addr.get_offset());
        // iter = first range with its first > addr, then step back to the
        // last range with range.first <= addr
        match self.tree.range((Bound::Unbounded, Bound::Included(&probe))).next_back() {
            None => false,
            Some(r) => {
                if !Rc::ptr_eq(&r.spc, spc) {
                    return false;
                }
                // last >= addr.getOffset()+size-1 (unsigned wrap, int4 size
                // sign-extending)
                r.last >= addr.get_offset().wadd(size as i64 as u64).wsub(1)
            }
        }
    }

    /// If \b this RangeList contains the specific address (spaceid,offset),
    /// return it, or None
    pub fn get_range(&self, spaceid: &Rc<AddrSpace>, offset: u64) -> Option<&Range> {
        if self.tree.is_empty() {
            return None;
        }
        let probe = Range::new(Rc::clone(spaceid), offset, offset);
        // iter = first range with its first > offset, stepped back
        match self.tree.range((Bound::Unbounded, Bound::Included(&probe))).next_back() {
            None => None,
            Some(r) => {
                if !Rc::ptr_eq(&r.spc, spaceid) {
                    return None;
                }
                if r.last >= offset {
                    return Some(r);
                }
                None
            }
        }
    }

    /// Find size of biggest Range containing given address.
    ///
    /// Return the size of the biggest contiguous sequence of addresses in
    /// \b this RangeList which contain the given address.
    /// \param addr is the given address
    /// \param maxsize is the largest range to consider before giving up
    pub fn longest_fit(&self, addr: &Address, maxsize: u64) -> u64 {
        if addr.is_invalid() {
            return 0;
        }
        if self.tree.is_empty() {
            return 0;
        }
        let spc = addr.get_space().expect("longest_fit on sentinel address (C++ UB)");
        let mut offset = addr.get_offset();
        let probe = Range::new(Rc::clone(spc), offset, offset);
        // iter = first range with its first > addr, stepped back
        let first_range =
            match self.tree.range((Bound::Unbounded, Bound::Included(&probe))).next_back() {
                None => return 0,
                Some(r) => r,
            };
        let mut sizeres: u64 = 0;
        if first_range.last < offset {
            return sizeres;
        }
        // do { ... } while(iter != tree.end()) chain
        for r in self.tree.range((Bound::Included(first_range), Bound::Unbounded)) {
            if !Rc::ptr_eq(&r.spc, spc) {
                break;
            }
            if r.first > offset {
                break;
            }
            // Size extends to end of range
            sizeres = sizeres.wadd(r.last.wadd(1).wsub(offset));
            offset = r.last.wadd(1); // Try to chain on the next range
            if sizeres >= maxsize {
                break; // Don't bother if past maxsize
            }
        }
        sizeres
    }

    /// Get the first Range, or None if empty
    pub fn get_first_range(&self) -> Option<&Range> {
        self.tree.iter().next()
    }

    /// Get the last Range, or None if empty
    pub fn get_last_range(&self) -> Option<&Range> {
        self.tree.iter().next_back()
    }

    /// Get the last Range viewing offsets as signed.
    ///
    /// Treating offsets with their high-bits set as coming \e before offsets
    /// where the high-bit is clear, return the last/latest contiguous Range
    /// within the given address space.
    pub fn get_last_signed_range(&self, spaceid: &Rc<AddrSpace>) -> Option<&Range> {
        let midway = spaceid.get_highest() / 2; // Maximal signed value
        let probe = Range::new(Rc::clone(spaceid), midway, midway);
        // First element greater than -range- (should be MOST negative),
        // stepped back one
        if let Some(r) = self.tree.range((Bound::Unbounded, Bound::Included(&probe))).next_back() {
            if Rc::ptr_eq(&r.spc, spaceid) {
                return Some(r);
            }
        }

        // If there were no "positive" ranges, search for biggest negative
        // range
        let probe = Range::new(Rc::clone(spaceid), spaceid.get_highest(), spaceid.get_highest());
        if let Some(r) = self.tree.range((Bound::Unbounded, Bound::Included(&probe))).next_back() {
            if Rc::ptr_eq(&r.spc, spaceid) {
                return Some(r);
            }
        }
        None
    }

    /// Print a description of \b this RangeList to stream.
    ///
    /// Print a one line description of each disjoint Range making up \b this
    /// RangeList.
    pub fn print_bounds(&self, s: &mut String) {
        if self.tree.is_empty() {
            s.push_str("all\n");
        } else {
            for range in self.tree.iter() {
                range.print_bounds(s);
                s.push('\n');
            }
        }
    }

    /// Encode \b this RangeList to a stream as a \<rangelist> element
    pub fn encode(&self, encoder: &mut dyn Encoder) {
        encoder.open_element(&ELEM_RANGELIST);
        for range in self.tree.iter() {
            range.encode(encoder);
        }
        encoder.close_element(&ELEM_RANGELIST);
    }

    /// Decode \b this RangeList from a \<rangelist> element.
    /// Recover each individual disjoint Range.
    pub fn decode(&mut self, decoder: &mut dyn Decoder) -> KunaResult<()> {
        let elem_id = decoder.open_element_id(&ELEM_RANGELIST)?;
        while decoder.peek_element()? != 0 {
            let range = Range::decode(decoder)?;
            self.tree.insert(range);
        }
        decoder.close_element(elem_id)
    }
}

// ---------------------------------------------------------------------------
// BitRange
// ---------------------------------------------------------------------------

/// \brief An endian aware range of bits contained in a contiguous set of
/// bytes
#[derive(Debug, Clone, Copy)]
pub struct BitRange {
    /// Byte offset of the region containing the range
    pub byte_offset: i32,
    /// Size of the region in bytes
    pub byte_size: i32,
    /// Least significant bit of the bit-range within its region
    pub least_sig_bit: i32,
    /// Number of bits in the range
    pub num_bits: i32,
    /// Is the underlying encoding big endian
    pub is_big_endian: bool,
}

impl Default for BitRange {
    /// Construct \e undefined range (C++ `BitRange(void)`)
    fn default() -> Self {
        BitRange {
            byte_offset: -1,
            byte_size: -1,
            least_sig_bit: -1,
            num_bits: -1,
            is_big_endian: false,
        }
    }
}

impl BitRange {
    /// Construct \e undefined range
    pub fn new() -> Self {
        Self::default()
    }

    /// Construct byte range
    pub fn from_bytes(b_off: i32, b_size: i32, big_endian: bool) -> Self {
        BitRange {
            byte_offset: b_off,
            byte_size: b_size,
            least_sig_bit: 0,
            num_bits: b_size * 8,
            is_big_endian: big_endian,
        }
    }

    /// Constructor with explicit bit range
    pub fn with_bits(b_off: i32, b_size: i32, least: i32, num: i32, big_endian: bool) -> Self {
        BitRange {
            byte_offset: b_off,
            byte_size: b_size,
            least_sig_bit: least,
            num_bits: num,
            is_big_endian: big_endian,
        }
    }

    /// Constructor, copy range into new container
    /// (C++ `BitRange(const BitRange &,int4 off,int4 sz)`)
    pub fn recontained(op2: &BitRange, off: i32, sz: i32) -> Self {
        let mut res = BitRange {
            byte_offset: off,
            byte_size: sz,
            least_sig_bit: 0,
            num_bits: op2.num_bits,
            is_big_endian: op2.is_big_endian,
        };
        res.least_sig_bit = res.translate_lsb(op2);
        res
    }

    /// Return \b true if \b this is an empty bit range (zero bits)
    pub fn empty(&self) -> bool {
        self.num_bits <= 0
    }

    /// Compare \b this with another as containers.
    ///
    /// Both the byte container and the bit range are compared and must be
    /// equal to return 0.
    /// \return -1, 0, or 1 to establish ordering the two ranges
    pub fn compare(&self, op2: &BitRange) -> i32 {
        if self.byte_offset != op2.byte_offset {
            return if self.byte_offset < op2.byte_offset { -1 } else { 1 };
        }
        if self.byte_size != op2.byte_size {
            return if self.byte_size < op2.byte_size { -1 } else { 1 };
        }
        if self.least_sig_bit != op2.least_sig_bit {
            return if self.least_sig_bit < op2.least_sig_bit { -1 } else { 1 };
        }
        if self.num_bits != op2.num_bits {
            return if self.num_bits < op2.num_bits { -1 } else { 1 };
        }
        0
    }

    /// Translate the \b leastSigBit of the given range into \b this
    /// reference frame.
    ///
    /// The returned result is directly comparable with \b leastSigBit for
    /// determining order/overlap.
    pub fn translate_lsb(&self, op2: &BitRange) -> i32 {
        let mut op2_sig = op2.least_sig_bit;
        if self.is_big_endian {
            let this_pos = self.byte_offset + self.byte_size;
            let op2_pos = op2.byte_offset + op2.byte_size;
            op2_sig += 8 * (this_pos - op2_pos);
        } else {
            op2_sig += 8 * (op2.byte_offset - self.byte_offset);
        }
        op2_sig
    }

    /// Characterize the type of overlap between \b this and another range.
    ///
    /// Return:
    ///   -  -1 if \b this should come before (no intersection)
    ///   -  0 if \b this and op2 are the same bitrange
    ///   -  1 if \b this should come after (no intersection)
    ///   -  2 if \b this is contained in op2
    ///   -  3 if op2 is contained in \b this
    ///   -  4 if partial overlap
    pub fn overlap_test(&self, op2: &BitRange) -> i32 {
        let op2_sig = self.translate_lsb(op2);
        let this_most = self.least_sig_bit + self.num_bits;
        let op2_most = op2_sig + op2.num_bits;
        if self.is_big_endian {
            if self.least_sig_bit >= op2_most {
                return -1;
            }
            if op2_sig >= this_most {
                return 1;
            }
        } else {
            if this_most <= op2_sig {
                return -1;
            }
            if op2_most <= self.least_sig_bit {
                return 1;
            }
        }
        // Reaching here we have some kind of intersection
        if self.least_sig_bit == op2_sig && this_most == op2_most {
            return 0;
        }
        if op2_sig <= self.least_sig_bit && op2_most >= this_most {
            return 2; // this contained in op2
        }
        if self.least_sig_bit <= op2_sig && this_most >= op2_most {
            return 3; // op2 contained in this
        }
        4
    }

    /// Replace \b this with the intersection of \b this with another
    /// BitRange.
    ///
    /// The byte container for \b this does not change, only \b leastSigBit
    /// and \b numBits.  If the intersection is empty, \b numBits is set to
    /// 0.
    pub fn intersection(&mut self, op2: &BitRange) {
        let op2_sig = self.translate_lsb(op2);
        let op2_most = op2_sig + op2.num_bits;
        let this_most = self.least_sig_bit + self.num_bits;
        if op2_sig > self.least_sig_bit {
            self.num_bits -= op2_sig - self.least_sig_bit;
            self.least_sig_bit = op2_sig;
        }
        if op2_most < this_most {
            self.num_bits -= this_most - op2_most;
        }
        if self.num_bits < 0 {
            self.least_sig_bit = 0;
            self.num_bits = 0;
        }
    }

    /// Restrict \b this with a mask that lines up with the container.
    ///
    /// The range of bits is intersected with the 1-bits of the mask.  The
    /// resulting range is the minimal cover of the bits in the intersection.
    pub fn intersect_mask(&mut self, mut mask: u64) {
        mask &= self.get_mask();
        if mask == 0 {
            self.least_sig_bit = 0;
            self.num_bits = 0;
            return;
        }
        let new_least_sig = leastsigbit_set(mask);
        let new_most_sig = mostsigbit_set(mask) + 1;
        let this_most = self.least_sig_bit + self.num_bits;
        if new_least_sig > self.least_sig_bit {
            self.num_bits -= new_least_sig - self.least_sig_bit;
            self.least_sig_bit = new_least_sig;
        }
        if new_most_sig < this_most {
            self.num_bits -= this_most - new_most_sig;
        }
    }

    /// Replace \b this with the shifted range.
    ///
    /// The bit range is shifted to the left by the given amount.
    pub fn shift(&mut self, left_shift_amount: i32) {
        self.least_sig_bit += left_shift_amount;
        let most = self.least_sig_bit + self.num_bits;
        if self.least_sig_bit < 0 {
            self.num_bits += self.least_sig_bit;
            self.least_sig_bit = 0;
        } else if most > self.byte_size * 8 {
            self.num_bits -= most - self.byte_size * 8;
        }
        if self.num_bits < 0 {
            self.least_sig_bit = 0;
            self.num_bits = 0;
        }
    }

    /// Truncate the most significant bytes in the byte container.
    /// The number of bits may be affected.
    pub fn truncate_most_sig_bytes(&mut self, num: i32) {
        if self.is_big_endian {
            self.byte_offset += num;
        }
        self.byte_size -= num;
        let max_offset = self.least_sig_bit + self.num_bits;
        if max_offset > self.byte_size * 8 {
            self.num_bits -= max_offset - self.byte_size * 8;
        }
        if self.num_bits < 0 {
            self.num_bits = 0;
        }
    }

    /// Truncate the least significant bytes in the byte container.
    pub fn truncate_least_sig_bytes(&mut self, num: i32) {
        if !self.is_big_endian {
            self.byte_offset += num;
        }
        self.byte_size -= num;
        self.least_sig_bit -= num * 8;
        if self.least_sig_bit < 0 {
            self.num_bits += self.least_sig_bit;
            self.least_sig_bit = 0;
            if self.num_bits < 0 {
                self.num_bits = 0;
            }
        }
    }

    /// Add most significant bytes to the container.
    /// Only the container is affected, the bit range itself does not change.
    pub fn extend_bytes(&mut self, num: i32) {
        if self.is_big_endian {
            self.byte_offset -= num;
        }
        self.byte_size += num;
    }

    /// Get mask representing \b this range.
    /// The bit-mask is aligned with the byte container.
    pub fn get_mask(&self) -> u64 {
        let mut res: u64;
        // mixed comparison: int4 numBits vs sizeof(uintb)*8 (size_t) — a
        // negative numBits converts to a huge unsigned value
        if self.num_bits as i64 as u64 >= 64 {
            res = 0;
        } else {
            res = 1;
            res = res.wshl(self.num_bits as u32);
        }
        res = res.wsub(1);
        res = res.wshl(self.least_sig_bit as u32);
        res
    }

    /// Return \b true if the beginning and end of the range fall on byte
    /// boundaries
    pub fn is_byte_range(&self) -> bool {
        if (self.num_bits & 7) != 0 {
            return false;
        }
        if (self.least_sig_bit & 7) != 0 {
            return false;
        }
        true
    }

    /// Return \b true if the most significant bit of the field and the
    /// container are the same
    pub fn is_most_significant(&self) -> bool {
        8 * self.byte_size == self.least_sig_bit + self.num_bits
    }

    /// Shrink the container to fit the bit range
    pub fn minimize_container(&mut self) {
        let trunc = self.least_sig_bit / 8;
        if self.is_big_endian {
            self.byte_size -= trunc;
        } else {
            self.byte_offset += trunc;
        }
        self.least_sig_bit &= 7;
        let num = self.byte_size - (self.least_sig_bit + self.num_bits + 7) / 8;
        if num > 0 {
            if self.is_big_endian {
                self.byte_offset += num;
            }
            self.byte_size -= num;
        }
    }

    /// Expand the bitrange until it includes the most significant bits of
    /// the container
    pub fn expand_to_most(&mut self) {
        // Increase number of bits to maximum that still fits
        self.num_bits = 8 * self.byte_size - self.least_sig_bit;
    }
}

// ---------------------------------------------------------------------------
// calc_mask and the other helper functions
// ---------------------------------------------------------------------------

/// Precalculated masks indexed by size (the non-UINTB4 table)
#[allow(non_upper_case_globals)]
pub const uintbmasks: [u64; 9] = [
    0,
    0xff,
    0xffff,
    0xffffff,
    0xffffffff,
    0xffffffffff,
    0xffffffffffff,
    0xffffffffffffff,
    0xffffffffffffffff,
];

/// Calculate a mask for a given byte size.
/// \param size is the desired size in bytes
/// \return a value appropriate for masking off the first \e size bytes
pub fn calc_mask(size: i32) -> u64 {
    // `((uint4)size) < 8 ? size : 8`: a negative size converts to a huge
    // uint4 and clamps to index 8
    uintbmasks[if (size as u32) < 8 { size as usize } else { 8 }]
}

/// Get the largest unsigned integer value for the given size.
/// \param size of the integer in bytes
pub fn calc_uint_max(size: i32) -> u64 {
    calc_mask(size)
}

/// Get the largest signed integer value for the given size.
/// \param size of the integer in bytes
pub fn calc_int_max(size: i32) -> u64 {
    calc_mask(size) >> 1
}

/// Get the smallest signed integer value for the given size.
/// \param size of the integer in bytes
pub fn calc_int_min(size: i32) -> u64 {
    // (uintb)1 << (size*8-1); size 0 is UB in C++ (wrapping shift here)
    1u64.wshl((size * 8 - 1) as u32)
}

/// Perform a CPUI_INT_RIGHT on the given val.
/// \param val is the value to shift
/// \param sa is the number of bits to shift
pub fn pcode_right(val: u64, sa: i32) -> u64 {
    // mixed comparison: int4 sa vs 8*sizeof(uintb) (size_t); a negative sa
    // converts huge and returns 0
    if sa as i64 as u64 >= 64 {
        return 0;
    }
    val >> sa
}

/// Perform a CPUI_INT_LEFT on the given val.
/// \param val is the value to shift
/// \param sa is the number of bits to shift
pub fn pcode_left(val: u64, sa: i32) -> u64 {
    if sa as i64 as u64 >= 64 {
        return 0;
    }
    val << sa
}

/// \brief Calculate smallest mask that covers the given value
///
/// Calculcate a mask that covers either the least significant byte, uint2,
/// uint4, or uint8, whatever is smallest.
pub fn minimalmask(val: u64) -> u64 {
    if val > 0xffffffff {
        return u64::MAX;
    }
    if val > 0xffff {
        return 0xffffffff;
    }
    if val > 0xff {
        return 0xffff;
    }
    0xff
}

/// \brief Sign extend above given bit
///
/// Sign extend \b val starting at \b bit
/// \param val is the value to be sign-extended
/// \param bit is the index of the bit to extend from (0=least significant bit)
pub fn sign_extend(val: i64, bit: i32) -> i64 {
    let sa = 64 - (bit + 1);
    val.wshl(sa as u32).wshr(sa as u32)
}

/// \brief Clear all bits above given bit
///
/// Zero extend \b val starting at \b bit
/// \param val is the value to be zero extended
/// \param bit is the index of the bit to extend from (0=least significant bit)
pub fn zero_extend(val: i64, bit: i32) -> i64 {
    let sa = 64 - (bit + 1);
    ((val as u64).wshl(sa as u32).wshr(sa as u32)) as i64
}

/// Return true if the sign-bit is set.
/// Treat the given \b val as a constant of \b size bytes.
pub fn signbit_negative(val: u64, size: i32) -> bool {
    // Return true if signbit is set (negative)
    let mask = 0x80u64.wshl((8 * (size - 1)) as u32);
    (val & mask) != 0
}

/// Negate the \e sized value.
/// Treat the given \b in as a constant of \b size bytes.  Negate this
/// constant keeping the upper bytes zero.
pub fn uintb_negate(in_: u64, size: i32) -> u64 {
    // Invert bits
    (!in_) & calc_mask(size)
}

/// Sign-extend a value between two byte sizes
/// (C++ `uintb sign_extend(uintb,int4,int4)`; renamed for the intb
/// overload above).
///
/// Take the first \b sizein bytes of the given \b in and sign-extend this to
/// \b sizeout bytes, keeping any more significant bytes zero.
pub fn sign_extend_sized(in_: u64, sizein: i32, sizeout: i32) -> u64 {
    // mixed comparisons: int4 vs sizeof(uintb) (size_t) — negative sizes
    // convert huge and clamp to 8
    let sizein = if (sizein as i64 as u64) < 8 { sizein } else { 8 };
    let sizeout = if (sizeout as i64 as u64) < 8 { sizeout } else { 8 };
    let mut sval = in_ as i64;
    // out-of-range/negative C++ shift counts resolve with x86 masking,
    // matching the wrapping shifts (ADR 0003)
    sval = sval.wshl(((8 - sizein) * 8) as u32);
    let mut res = sval.wshr(((sizeout - sizein) * 8) as u32) as u64;
    res = res.wshr(((8 - sizeout) * 8) as u32);
    res
}

/// Extend a signed value of given number of bits to a full integer.
/// \param val is the value to extend
/// \param numbits is the number of bits in the value
/// \param size is the integer size in bytes
pub fn extend_signbit(val: u64, numbits: i32, size: i32) -> u64 {
    let mut val = val;
    if numbits < size * 8 {
        let sa = 64 - numbits;
        let sval = val as i64;
        val = sval.wshl(sa as u32).wshr(sa as u32) as u64;
        val &= calc_mask(size);
    }
    val
}

/// Swap bytes in the given value (C++ `void byte_swap(intb &,int4)`).
/// Swap the least significant \b size bytes in \b val.
pub fn byte_swap_inplace(val: &mut i64, mut size: i32) {
    let mut res: i64 = 0;
    while size > 0 {
        res = res.wshl(8);
        res |= *val & 0xff;
        *val >>= 8; // arithmetic shift, as in C++
        size -= 1;
    }
    *val = res;
}

/// Return the given value with bytes swapped
/// (C++ `uintb byte_swap(uintb,int4)`).
/// Swap the least significant \b size bytes in \b val.
pub fn byte_swap(mut val: u64, mut size: i32) -> u64 {
    let mut res: u64 = 0;
    while size > 0 {
        res = res.wshl(8);
        res |= val & 0xff;
        val >>= 8;
        size -= 1;
    }
    res
}

/// Return index of least significant bit set in given value.
/// The least significant bit is index 0.
/// \return the index of the least significant set bit, or -1 if none are set
pub fn leastsigbit_set(mut val: u64) -> i32 {
    if val == 0 {
        return -1;
    }
    let mut res: i32 = 0;
    let mut sz: i32 = 32; // 4*sizeof(uintb)
    let mut mask: u64 = u64::MAX;
    loop {
        mask >>= sz;
        if (mask & val) == 0 {
            res += sz;
            val >>= sz;
        }
        sz >>= 1;
        if sz == 0 {
            break;
        }
    }
    res
}

/// Return index of most significant bit set in given value.
/// The least significant bit is index 0.
/// \return the index of the most significant set bit, or -1 if none are set
pub fn mostsigbit_set(mut val: u64) -> i32 {
    if val == 0 {
        return -1;
    }
    let mut res: i32 = 63; // 8*sizeof(uintb)-1
    let mut sz: i32 = 32; // 4*sizeof(uintb)
    let mut mask: u64 = u64::MAX;
    loop {
        mask <<= sz;
        if (mask & val) == 0 {
            res -= sz;
            val <<= sz;
        }
        sz >>= 1;
        if sz == 0 {
            break;
        }
    }
    res
}

/// Return the number of one bits in the given value.
/// Count the number (population) of bits set.
pub fn popcount(mut val: u64) -> i32 {
    val = (val & 0x5555555555555555) + ((val >> 1) & 0x5555555555555555);
    val = (val & 0x3333333333333333) + ((val >> 2) & 0x3333333333333333);
    val = (val & 0x0f0f0f0f0f0f0f0f) + ((val >> 4) & 0x0f0f0f0f0f0f0f0f);
    val = (val & 0x00ff00ff00ff00ff) + ((val >> 8) & 0x00ff00ff00ff00ff);
    val = (val & 0x0000ffff0000ffff) + ((val >> 16) & 0x0000ffff0000ffff);
    let mut res = (val & 0xff) as i32;
    res += ((val >> 32) & 0xff) as i32;
    res
}

/// Return the number of leading zero bits in the given value.
///
/// Count the number of more significant zero bits before the most
/// significant one bit in the representation of the given value.
pub fn count_leading_zeros(val: u64) -> i32 {
    if val == 0 {
        return 64; // 8*sizeof(uintb)
    }
    let mut mask: u64 = u64::MAX;
    let mut mask_size: i32 = 32; // 4*sizeof(uintb)
    mask &= mask << mask_size;
    let mut bit: i32 = 0;

    loop {
        if (mask & val) == 0 {
            bit += mask_size;
            mask_size >>= 1;
            mask |= mask >> mask_size;
        } else {
            mask_size >>= 1;
            mask &= mask << mask_size;
        }
        if mask_size == 0 {
            break;
        }
    }
    bit
}

/// Return a mask that \e covers the given value.
/// Return smallest number of form 2^n-1, bigger or equal to the given value.
pub fn coveringmask(val: u64) -> u64 {
    let mut res = val;
    let mut sz: i32 = 1;
    while sz < 64 {
        // 8*sizeof(uintb)
        res |= res >> sz;
        sz <<= 1;
    }
    res
}

/// Calculate the number of bit transitions in the sized value.
///
/// Treat \b val as a constant of size \b sz.  Scanning across the bits of
/// \b val return the number of transitions (from 0->1 or 1->0).  If there
/// are 2 or less transitions, this is an indication of a bit flag or a mask.
pub fn bit_transitions(mut val: u64, sz: i32) -> i32 {
    let mut res: i32 = 0;
    let mut last: i32 = (val & 1) as i32;
    for _i in 1..(8 * sz) {
        val >>= 1;
        let cur = (val & 1) as i32;
        if cur != last {
            res += 1;
            last = cur;
        }
        if val == 0 {
            break;
        }
    }
    res
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::space::ConstantSpace;

    fn ram() -> Rc<AddrSpace> {
        Rc::new(AddrSpace::new(
            spacetype::IPTR_PROCESSOR,
            "ram",
            false,
            8,
            1,
            3,
            crate::space::addrspace_flags::hasphysical,
            1,
            1,
        ))
    }

    fn small_space(name: &str, index: i32) -> Rc<AddrSpace> {
        Rc::new(AddrSpace::new(spacetype::IPTR_PROCESSOR, name, false, 2, 1, index, 0, 0, 0))
    }

    #[test]
    fn test_address_compare_and_extremes() {
        let ram = ram();
        let constant = Rc::new(ConstantSpace::new());
        let a = Address::new(Rc::clone(&ram), 0x1000);
        let b = Address::new(Rc::clone(&ram), 0x1001);
        let c = Address::new(Rc::clone(&constant), 0x5000);
        assert!(a < b);
        assert!(a <= b);
        assert!(a == a.clone());
        assert!(a != b);
        // const space (index 0) sorts before ram (index 3) regardless of offset
        assert!(c < a);
        // extremes
        let min = Address::new_extreme(mach_extreme::m_minimal);
        let max = Address::new_extreme(mach_extreme::m_maximal);
        assert!(min < a && min < c && min < max);
        assert!(a < max && c < max);
        assert!(min == Address::new_extreme(mach_extreme::m_minimal));
        // invalid address detection
        assert!(Address::new_invalid().is_invalid());
        assert!(!a.is_invalid());
    }

    #[test]
    fn test_address_arithmetic_wraps() {
        let small = small_space("io", 4);
        let a = Address::new(Rc::clone(&small), 0xfffe);
        let b = &a + 3;
        assert_eq!(b.get_offset(), 0x1); // wraps at the space boundary
        let c = &b - 2;
        assert_eq!(c.get_offset(), 0xffff);
    }

    #[test]
    fn test_address_overlap_and_containment() {
        let ram = ram();
        let a = Address::new(Rc::clone(&ram), 0x100);
        let b = Address::new(Rc::clone(&ram), 0x104);
        assert_eq!(b.overlap(0, &a, 8), 4);
        assert_eq!(b.overlap(0, &a, 4), -1);
        let constant = Rc::new(ConstantSpace::new());
        let k = Address::new(Rc::clone(&constant), 4);
        assert_eq!(k.overlap(0, &k, 8), -1); // constants never overlap
        assert!(b.contained_by(4, &a, 8));
        assert!(!b.contained_by(8, &a, 8));
        // justified containment (little endian: offset from the left)
        assert_eq!(a.justified_contain(8, &b, 4, false), 4);
        assert_eq!(a.justified_contain(4, &b, 8, false), -1);
        // contiguous: this(hi) at 0x104, lo at 0x100 with losz 4 (little endian)
        assert!(b.is_contiguous(4, &a, 4));
        assert!(!a.is_contiguous(4, &b, 4));
    }

    #[test]
    fn test_seqnum_ordering() {
        let ram = ram();
        let a = Address::new(Rc::clone(&ram), 0x100);
        let b = Address::new(Rc::clone(&ram), 0x200);
        let s1 = SeqNum::new(a.clone(), 5);
        let s2 = SeqNum::new(a.clone(), 9);
        let s3 = SeqNum::new(b.clone(), 1);
        assert!(s1 < s2); // same pc: uniq breaks the tie
        assert!(s2 < s3); // pc dominates uniq
        // C++ operator== compares uniq only
        let s4 = SeqNum::new(b, 5);
        assert!(s1 == s4);
        assert!(s1 != s2);
        assert_eq!(s1.get_time(), 5);
    }

    #[test]
    fn test_range_list_insert_merge() {
        let ram = ram();
        let mut list = RangeList::new();
        list.insert_range(Rc::clone(&ram), 0x100, 0x1ff);
        list.insert_range(Rc::clone(&ram), 0x300, 0x3ff);
        assert_eq!(list.num_ranges(), 2);
        // overlapping insert merges
        list.insert_range(Rc::clone(&ram), 0x180, 0x310);
        assert_eq!(list.num_ranges(), 1);
        let r = list.get_first_range().unwrap();
        assert_eq!(r.get_first(), 0x100);
        assert_eq!(r.get_last(), 0x3ff);
        // adjacent (non-overlapping) ranges do NOT merge: 0x400 starts
        // after 0x3ff but upper_bound walking only merges actual overlap
        list.insert_range(Rc::clone(&ram), 0x500, 0x5ff);
        assert_eq!(list.num_ranges(), 2);
    }

    #[test]
    fn test_range_list_remove_splits() {
        let ram = ram();
        let mut list = RangeList::new();
        list.insert_range(Rc::clone(&ram), 0x100, 0x1ff);
        list.remove_range(Rc::clone(&ram), 0x140, 0x14f);
        assert_eq!(list.num_ranges(), 2);
        let mut s = String::new();
        list.print_bounds(&mut s);
        assert_eq!(s, "ram: 100-13f\nram: 150-1ff\n");
        // removing everything empties the container
        list.remove_range(Rc::clone(&ram), 0, u64::MAX);
        assert!(list.empty());
        let mut s = String::new();
        list.print_bounds(&mut s);
        assert_eq!(s, "all\n");
    }

    #[test]
    fn test_range_list_queries() {
        let ram = ram();
        let other = small_space("io", 4);
        let mut list = RangeList::new();
        list.insert_range(Rc::clone(&ram), 0x100, 0x1ff);
        list.insert_range(Rc::clone(&ram), 0x200, 0x2ff);
        list.insert_range(Rc::clone(&other), 0x10, 0x1f);
        let addr = Address::new(Rc::clone(&ram), 0x180);
        assert!(list.in_range(&addr, 0x10));
        assert!(list.in_range(&addr, 0x80));
        assert!(!list.in_range(&addr, 0x81 + 0x100)); // would need into 0x300
        // longest_fit chains contiguous ranges (0x100-0x1ff then 0x200-0x2ff)
        assert_eq!(list.longest_fit(&addr, 0x1000), 0x180);
        assert!(list.get_range(&ram, 0x250).is_some());
        assert!(list.get_range(&ram, 0x300).is_none());
        assert!(list.get_range(&other, 0x15).is_some());
        assert_eq!(list.num_ranges(), 3);
        // first/last respect space index order (io has higher index)
        assert!(Rc::ptr_eq(list.get_first_range().unwrap().get_space(), &ram));
        assert!(Rc::ptr_eq(list.get_last_range().unwrap().get_space(), &other));
    }

    #[test]
    fn test_range_get_last_signed_range() {
        let small = small_space("io", 4);
        let mut list = RangeList::new();
        // a "negative" range (high bit set) and a "positive" one
        list.insert_range(Rc::clone(&small), 0x8000, 0x80ff);
        list.insert_range(Rc::clone(&small), 0x100, 0x1ff);
        // positive ranges win: last range with first <= 0x7fff
        let r = list.get_last_signed_range(&small).unwrap();
        assert_eq!(r.get_first(), 0x100);
        // only negative ranges: fall back to biggest negative
        let mut list = RangeList::new();
        list.insert_range(Rc::clone(&small), 0x8000, 0x80ff);
        let r = list.get_last_signed_range(&small).unwrap();
        assert_eq!(r.get_first(), 0x8000);
    }

    #[test]
    fn test_bitrange_semantics() {
        // little endian byte range
        let mut br = BitRange::from_bytes(0, 4, false);
        assert_eq!(br.num_bits, 32);
        assert_eq!(br.get_mask(), 0xffffffff);
        assert!(br.is_byte_range());
        assert!(br.is_most_significant());
        br.intersect_mask(0x00ffff00);
        assert_eq!(br.least_sig_bit, 8);
        assert_eq!(br.num_bits, 16);
        assert!(br.is_byte_range());
        br.minimize_container();
        assert_eq!(br.byte_offset, 1);
        assert_eq!(br.byte_size, 2);
        assert_eq!(br.least_sig_bit, 0);
        // overlap testing
        let a = BitRange::with_bits(0, 4, 0, 16, false);
        let b = BitRange::with_bits(0, 4, 16, 16, false);
        assert_eq!(a.overlap_test(&b), -1);
        assert_eq!(b.overlap_test(&a), 1);
        assert_eq!(a.overlap_test(&a), 0);
        let c = BitRange::with_bits(0, 4, 4, 8, false);
        assert_eq!(c.overlap_test(&a), 2);
        assert_eq!(a.overlap_test(&c), 3);
    }

    #[test]
    fn test_address_helper_functions() {
        assert_eq!(calc_mask(1), 0xff);
        assert_eq!(calc_mask(4), 0xffffffff);
        assert_eq!(calc_mask(8), u64::MAX);
        assert_eq!(calc_mask(100), u64::MAX);
        assert_eq!(calc_mask(-1), u64::MAX); // (uint4)-1 clamps to 8
        assert_eq!(calc_int_max(4), 0x7fffffff);
        assert_eq!(calc_int_min(4), 0x80000000);
        assert!(signbit_negative(0x80, 1));
        assert!(!signbit_negative(0x7f, 1));
        assert_eq!(uintb_negate(0x0f, 1), 0xf0);
        assert_eq!(sign_extend(0x80, 7) as u64, 0xffffffffffffff80);
        assert_eq!(sign_extend(0x40, 7), 0x40);
        assert_eq!(zero_extend(-1, 15), 0xffff);
        assert_eq!(sign_extend_sized(0x80, 1, 4), 0xffffff80);
        assert_eq!(sign_extend_sized(0x7f, 1, 4), 0x7f);
        assert_eq!(sign_extend_sized(0xdeadbeef, 4, 8), 0xffffffffdeadbeef);
        assert_eq!(extend_signbit(0x8, 4, 2), 0xfff8);
        assert_eq!(byte_swap(0x12345678, 4), 0x78563412);
        let mut v: i64 = 0x1234;
        byte_swap_inplace(&mut v, 2);
        assert_eq!(v, 0x3412);
        assert_eq!(leastsigbit_set(0), -1);
        assert_eq!(leastsigbit_set(0x80), 7);
        assert_eq!(leastsigbit_set(1u64 << 63), 63);
        assert_eq!(mostsigbit_set(0), -1);
        assert_eq!(mostsigbit_set(1), 0);
        assert_eq!(mostsigbit_set(u64::MAX), 63);
        assert_eq!(popcount(0), 0);
        assert_eq!(popcount(u64::MAX), 64);
        assert_eq!(popcount(0xdeadbeef), 24);
        assert_eq!(count_leading_zeros(0), 64);
        assert_eq!(count_leading_zeros(1), 63);
        assert_eq!(count_leading_zeros(u64::MAX), 0);
        assert_eq!(coveringmask(0x1001), 0x1fff);
        assert_eq!(coveringmask(0), 0);
        assert_eq!(bit_transitions(0x0ff0, 2), 2);
        assert_eq!(bit_transitions(0, 8), 0);
        assert_eq!(pcode_right(0x100, 4), 0x10);
        assert_eq!(pcode_right(0x100, 64), 0);
        assert_eq!(pcode_right(0x100, -1), 0); // negative sa converts huge
        assert_eq!(pcode_left(0x10, 4), 0x100);
        assert_eq!(pcode_left(0x10, 65), 0);
        assert_eq!(minimalmask(0), 0xff);
        assert_eq!(minimalmask(0x100), 0xffff);
        assert_eq!(minimalmask(0x10000), 0xffffffff);
        assert_eq!(minimalmask(0x100000000), u64::MAX);
    }

    #[test]
    fn test_address_encode_decode_roundtrip() {
        use crate::marshal::{IdRegistry, PackedDecode, PackedEncode, XmlDecode, XmlEncode};
        use crate::space::AddrSpaceManager;
        let mut manager = AddrSpaceManager::new();
        manager
            .insert_space(Rc::new(AddrSpace::new(
                spacetype::IPTR_PROCESSOR,
                "ram",
                false,
                8,
                1,
                3,
                crate::space::addrspace_flags::hasphysical,
                1,
                1,
            )))
            .unwrap();
        let ram = Rc::clone(manager.get_space(3).unwrap());
        let addr = Address::new(Rc::clone(&ram), 0x401000);

        // packed
        let mut buf = Vec::new();
        {
            let mut enc = PackedEncode::new(&mut buf);
            addr.encode_sized(&mut enc, 4).unwrap();
        }
        let mut dec = PackedDecode::new(&manager);
        dec.ingest_stream(&buf).unwrap();
        let (rt, size) = Address::decode_sized(&mut dec).unwrap();
        assert!(rt == addr);
        assert_eq!(size, 4);

        // xml
        let registry = IdRegistry::with_base_ids();
        let mut buf = Vec::new();
        {
            let mut enc = XmlEncode::new(&mut buf);
            addr.encode(&mut enc).unwrap();
        }
        let mut dec = XmlDecode::new(&manager, &registry);
        dec.ingest_stream(&buf).unwrap();
        let rt = Address::decode(&mut dec).unwrap();
        assert!(rt == addr);
    }

    #[test]
    fn test_rangelist_encode_decode_roundtrip() {
        use crate::marshal::{PackedDecode, PackedEncode};
        use crate::space::AddrSpaceManager;
        let mut manager = AddrSpaceManager::new();
        manager
            .insert_space(Rc::new(AddrSpace::new(
                spacetype::IPTR_PROCESSOR,
                "ram",
                false,
                8,
                1,
                3,
                0,
                1,
                1,
            )))
            .unwrap();
        let ram = Rc::clone(manager.get_space(3).unwrap());
        let mut list = RangeList::new();
        list.insert_range(Rc::clone(&ram), 0x100, 0x1ff);
        list.insert_range(Rc::clone(&ram), 0x400, 0x4ff);
        let mut buf = Vec::new();
        {
            let mut enc = PackedEncode::new(&mut buf);
            list.encode(&mut enc);
        }
        let mut dec = PackedDecode::new(&manager);
        dec.ingest_stream(&buf).unwrap();
        let mut rt = RangeList::new();
        rt.decode(&mut dec).unwrap();
        assert_eq!(rt.num_ranges(), 2);
        let mut s1 = String::new();
        let mut s2 = String::new();
        list.print_bounds(&mut s1);
        rt.print_bounds(&mut s2);
        assert_eq!(s1, s2);
    }

    #[test]
    fn test_range_last_addr_open() {
        use crate::space::AddrSpaceManager;
        let mut manager = AddrSpaceManager::new();
        manager
            .insert_space(Rc::new(AddrSpace::new(
                spacetype::IPTR_PROCESSOR,
                "ram",
                false,
                2,
                1,
                0,
                0,
                0,
                0,
            )))
            .unwrap();
        manager
            .insert_space(Rc::new(AddrSpace::new(
                spacetype::IPTR_PROCESSOR,
                "io",
                false,
                2,
                1,
                1,
                0,
                0,
                0,
            )))
            .unwrap();
        let ram = Rc::clone(manager.get_space(0).unwrap());
        let io = Rc::clone(manager.get_space(1).unwrap());
        // interior range: last+1 within the same space
        let r = Range::new(Rc::clone(&ram), 0x10, 0x1f);
        let open = r.get_last_addr_open(&manager);
        assert!(Rc::ptr_eq(open.get_space().unwrap(), &ram));
        assert_eq!(open.get_offset(), 0x20);
        // range ending at the space top: rolls into the next space at 0
        let r = Range::new(Rc::clone(&ram), 0x10, 0xffff);
        let open = r.get_last_addr_open(&manager);
        assert!(Rc::ptr_eq(open.get_space().unwrap(), &io));
        assert_eq!(open.get_offset(), 0);
        // last space ending at top: the ~0 sentinel space with offset 0
        let r = Range::new(Rc::clone(&io), 0x10, 0xffff);
        let open = r.get_last_addr_open(&manager);
        assert!(matches!(open.base_ptr(), AddrSpacePtr::Max));
        assert_eq!(open.get_offset(), 0);
        // it still sorts after every real address
        assert!(Address::new(Rc::clone(&io), 0xffff) < open);
    }
}
