//! Port of `decompiler/cpp/globalcontext.hh` + `globalcontext.cc` (W2, item
//! `w2-sleigh-context`): utilities for getting address-based context to the
//! disassembler and decompiler.
//!
//! Pieces: [`ContextBitRange`] (a value packed into the context blob),
//! [`TrackedContext`]/[`TrackedSet`] (tracked register values),
//! [`ContextDatabase`] (the abstract database interface),
//! [`ContextInternal`] (the in-memory implementation on
//! `kuna_base::partmap::PartMap`), and [`ContextCache`] (the per-address
//! cache with flow-boundary invalidation).
//!
//! Port decisions (each justified at its use site):
//!
//! - `uintm -> u32`: a context *blob* is an array of `u32` words
//!   (`8*sizeof(uintm)` is 32 everywhere below).
//! - Context-variable names are byte strings (`Vec<u8>`/`&[u8]`) per the
//!   workspace marshal convention; `BTreeMap<Vec<u8>,_>` iterates in the
//!   same byte-lexicographic order as the C++ `std::map<string,_>`.
//! - The C++ protected virtuals (`getVariable`, `getRegionForSet`,
//!   `getRegionToChangePoint`, `getDefaultValue`) are public trait methods —
//!   Rust traits have no protected visibility.  `getVariable` returns the
//!   `ContextBitRange` *by value* (it is a `Copy` POD that is immutable
//!   after registration; the C++ non-const reference is never used to
//!   mutate), which sidesteps borrow conflicts in the provided methods.
//! - C++ `getRegionForSet`/`getRegionToChangePoint` pass back a
//!   `vector<uintm *>` of raw pointers into the database which the caller
//!   then writes through.  Safe Rust cannot hand out a list of aliasing
//!   `&mut`; instead the region methods invoke a caller-supplied callback
//!   on each blob, in the same iteration order.  Every C++ caller applies
//!   one uniform mutation per returned blob, so interleaving "mark mask /
//!   mutate blob" per entry produces the identical final state (the mask
//!   array and value array of an entry never alias, and the
//!   `getRegionToChangePoint` stop-test reads only masks of *later*
//!   entries, which the callback never touches).
//! - `FreeArray`'s `Clone` transcribes the C++ `operator=` — values are
//!   copied, the mask is **zeroed** ("Copy value at split point, but not
//!   fact that value is being set").  `PartMap::split`/`clear_range` are
//!   the only cloners, exactly the C++ `database[pnt] = ...` sites.
//! - [`ContextCache`] does not own the database: the C++ encapsulated
//!   `ContextDatabase *` becomes an explicit method parameter (same
//!   precedent as `kuna_num::pcoderaw::VarnodeData::get_space_from_const`,
//!   which takes the manager the C++ pointer implied).  C++ `getDatabase()`
//!   has no equivalent.  The C++ cached `const uintm *context` pointer
//!   cannot be stored safely; on a cache hit the blob is re-fetched with
//!   the cheap single-lookup `ContextDatabase::get_context` instead.  This
//!   reproduces the C++ behavior for every flow that goes through the
//!   ContextCache API (including paints from below `first` that reach into
//!   the cached range without tripping the invalidation tests).  The only
//!   divergence is flows that mutate the database *directly* while a cache
//!   is live AND insert a new split point inside the cached range: C++
//!   serves the stale pre-split blob, the port serves the fresh one.  No
//!   in-tree flow does this (reported as a loss by the porting item).

use std::collections::BTreeMap;
use std::rc::Rc;

use kuna_base::address::{calc_mask, Address, Range};
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::marshal::{Decoder, ElementId, Encoder, IdRegistry, ATTRIB_NAME, ATTRIB_VAL};
use kuna_base::partmap::PartMap;
use kuna_base::space::AddrSpace;
use kuna_base::types::Wrap;
use kuna_num::pcoderaw::VarnodeData;

// ---------------------------------------------------------------------------
// Marshaling ids (globalcontext.cc; numeric values pinned by the packed
// wire format)
// ---------------------------------------------------------------------------

/// Marshaling element \<context_data>
pub const ELEM_CONTEXT_DATA: ElementId = ElementId::new("context_data", 120);
/// Marshaling element \<context_points>
pub const ELEM_CONTEXT_POINTS: ElementId = ElementId::new("context_points", 121);
/// Marshaling element \<context_pointset>
pub const ELEM_CONTEXT_POINTSET: ElementId = ElementId::new("context_pointset", 122);
/// Marshaling element \<context_set>
pub const ELEM_CONTEXT_SET: ElementId = ElementId::new("context_set", 123);
/// Marshaling element \<set>
pub const ELEM_SET: ElementId = ElementId::new("set", 124);
/// Marshaling element \<tracked_pointset>
pub const ELEM_TRACKED_POINTSET: ElementId = ElementId::new("tracked_pointset", 125);
/// Marshaling element \<tracked_set>
pub const ELEM_TRACKED_SET: ElementId = ElementId::new("tracked_set", 126);

/// The element ids defined by `globalcontext.cc` (in C++ these register
/// themselves via global constructors; see `kuna_base::marshal::IdRegistry`).
pub static GLOBALCONTEXT_ELEMENT_IDS: &[&ElementId] = &[
    &ELEM_CONTEXT_DATA,
    &ELEM_CONTEXT_POINTS,
    &ELEM_CONTEXT_POINTSET,
    &ELEM_CONTEXT_SET,
    &ELEM_SET,
    &ELEM_TRACKED_POINTSET,
    &ELEM_TRACKED_SET,
];

/// Register every globalcontext element id with the given registry
/// (the explicit replacement for the C++ global-constructor registration).
pub fn register_globalcontext_ids(registry: &mut IdRegistry) {
    for e in GLOBALCONTEXT_ELEMENT_IDS {
        registry.register_element(e);
    }
}

// ---------------------------------------------------------------------------
// ContextBitRange
// ---------------------------------------------------------------------------

/// \brief Description of a context variable within the disassembly context
/// \e blob
///
/// Disassembly context is stored as individual (integer) values packed into
/// a sequence of words. This class represents the info for encoding or
/// decoding a single value within this sequence.  A value is a contiguous
/// range of bits within one context word. Size can range from 1 bit up to
/// the size of a word.
///
/// (C++ `ContextBitRange(void)` leaves the fields uninitialized; `Default`
/// zeroes them here.)
#[derive(Debug, Clone, Copy, Default)]
pub struct ContextBitRange {
    /// Index of word containing this context value
    word: i32,
    /// Starting bit of the value within its word (0=most significant bit
    /// 1=least significant).  Never read after construction, as in C++.
    #[allow(dead_code)]
    startbit: i32,
    /// Ending bit of the value within its word.  Never read after
    /// construction, as in C++.
    #[allow(dead_code)]
    endbit: i32,
    /// Right-shift amount to apply when unpacking this value from its word
    shift: i32,
    /// Mask to apply (after shifting) when unpacking this value from its word
    mask: u32,
}

impl ContextBitRange {
    /// Bits within the whole context blob are labeled starting with 0 as the
    /// most significant bit in the first word in the sequence. The new
    /// context value must be contained within a single word.
    /// \param sbit is the starting (most significant) bit of the new value
    /// \param ebit is the ending (least significant) bit of the new value
    pub fn new(sbit: i32, ebit: i32) -> ContextBitRange {
        // 8*sizeof(uintm) == 32.  (C++ divides int4 by size_t — an unsigned
        // division — identical to i32 division for the non-negative bit
        // positions used here.)
        let word = sbit / 32;
        let startbit = sbit - word * 32;
        let endbit = ebit - word * 32;
        let shift = 32 - endbit - 1;
        // cast: startbit+shift is in [0,31] for any in-word range (C++
        // shifts by an int with the same value)
        let mask = (!0u32) >> ((startbit + shift) as u32);
        ContextBitRange { word, startbit, endbit, shift, mask }
    }

    /// Return the shift-amount for \b this value
    pub fn get_shift(&self) -> i32 {
        self.shift
    }

    /// Return the mask for \b this value
    pub fn get_mask(&self) -> u32 {
        self.mask
    }

    /// Return the word index for \b this value
    pub fn get_word(&self) -> i32 {
        self.word
    }

    /// \brief Set \b this value within a given context blob
    ///
    /// \param vec is the given context blob to alter (as an array of words)
    /// \param val is the integer value to set
    pub fn set_value(&self, vec: &mut [u32], val: u32) {
        let word = self.word as usize; // cast: word index, non-negative
        let mut newval = vec[word];
        newval &= !(self.mask << self.shift);
        newval |= (val & self.mask) << self.shift;
        vec[word] = newval;
    }

    /// \brief Retrieve \b this value from a given context blob
    ///
    /// \param vec is the given context blob (as an array of words)
    /// \return the recovered integer value
    pub fn get_value(&self, vec: &[u32]) -> u32 {
        (vec[self.word as usize] >> self.shift) & self.mask // cast: word index
    }
}

// ---------------------------------------------------------------------------
// TrackedContext / TrackedSet
// ---------------------------------------------------------------------------

/// \brief A tracked register (Varnode) and the value it contains
///
/// This is the object returned when querying for tracked registers, via
/// [`ContextDatabase::get_tracked_set`].  It holds the storage details of
/// the register and the actual value it holds at the point of the query.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TrackedContext {
    /// Storage details of the register being tracked
    pub loc: VarnodeData,
    /// The value of the register
    pub val: u64,
}

impl TrackedContext {
    /// The register storage and value are encoded as a \<set> element.
    /// \param encoder is the stream encoder
    pub fn encode(&self, encoder: &mut dyn Encoder) -> KunaResult<()> {
        encoder.open_element(&ELEM_SET);
        self.loc
            .space
            .as_ref()
            .expect("TrackedContext::encode: null space pointer (C++ UB)")
            // cast: VarnodeData::size is uint4, encodeAttributes takes int4
            .encode_attributes_sized(encoder, self.loc.offset, self.loc.size as i32)?;
        encoder.write_unsigned_integer(&ATTRIB_VAL, self.val);
        encoder.close_element(&ELEM_SET);
        Ok(())
    }

    /// Parse a \<set> element to fill in the storage and value details.
    /// \param decoder is the stream decoder
    pub fn decode(&mut self, decoder: &mut dyn Decoder) -> KunaResult<()> {
        let elem_id = decoder.open_element_id(&ELEM_SET)?;
        self.loc.decode_from_attributes(decoder)?;
        self.val = decoder.read_unsigned_integer_id(&ATTRIB_VAL)?;
        decoder.close_element(elem_id)
    }
}

/// A set of tracked registers and their values (at one code point)
pub type TrackedSet = Vec<TrackedContext>;

/// \brief Encode all tracked register values for a specific address to a
/// stream
///
/// Encode all the tracked register values associated with a specific target
/// address as a \<tracked_pointset> tag.
/// (C++ `ContextDatabase::encodeTracked`, a protected static method.)
/// \param encoder is the stream encoder
/// \param addr is the specific address we have tracked values for
/// \param vec is the list of tracked values
pub fn encode_tracked(
    encoder: &mut dyn Encoder,
    addr: &Address,
    vec: &TrackedSet,
) -> KunaResult<()> {
    if vec.is_empty() {
        return Ok(());
    }
    encoder.open_element(&ELEM_TRACKED_POINTSET);
    addr.get_space()
        .expect("encodeTracked: invalid address (C++ UB)")
        .encode_attributes(encoder, addr.get_offset())?;
    for tcont in vec.iter() {
        tcont.encode(encoder)?;
    }
    encoder.close_element(&ELEM_TRACKED_POINTSET);
    Ok(())
}

/// \brief Restore a sequence of tracked register values from the given
/// stream decoder
///
/// Parse a \<tracked_pointset> element, decoding each child in turn to
/// populate a list of TrackedContext objects.
/// (C++ `ContextDatabase::decodeTracked`, a protected static method.)
/// \param decoder is the given stream decoder
/// \param vec is the container that will hold the new TrackedContext objects
pub fn decode_tracked(decoder: &mut dyn Decoder, vec: &mut TrackedSet) -> KunaResult<()> {
    vec.clear(); // Clear out any old stuff
    while decoder.peek_element()? != 0 {
        let mut tcont = TrackedContext::default();
        tcont.decode(decoder)?;
        vec.push(tcont);
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// ContextDatabase
// ---------------------------------------------------------------------------

/// C++ raw-pointer equality on optional space fields (null == null), as in
/// `kuna_num::pcoderaw`.
fn space_ptr_eq(a: &Option<Rc<AddrSpace>>, b: &Option<Rc<AddrSpace>>) -> bool {
    match (a, b) {
        (Some(a), Some(b)) => Rc::ptr_eq(a, b),
        (None, None) => true,
        _ => false,
    }
}

/// \brief An interface to a database of disassembly/decompiler \b context
/// information
///
/// \b Context \b information is a set of named variables that hold concrete
/// values at specific addresses in the target executable being analyzed. A
/// variable can hold different values at different addresses, but a specific
/// value at a specific address never changes. Analysis recovers these values
/// over time, populating this database, and querying this database lets
/// analysis provide concrete values for memory locations in context.
///
/// Context variables come in two flavors:
///  - \b Low-level \b context \b variables:
///    These can affect instruction decoding. These can be as small as a
///    single bit and need to be defined in the Sleigh specification (so
///    that Sleigh knows how they effect disassembly).  These variables are
///    not mapped to normal memory locations with an address space and
///    offset (although they often have a corresponding embedding into a
///    normal memory location).  The model to keep in mind is a control
///    register with specialized bit-fields within it.
///  - \b High-level \b tracked \b variables:
///    These are normal memory locations that are to be treated as
///    constants across some range of code. These are normally registers
///    that are being tracked by the compiler outside the domain of normal
///    local and global variables. They have a specific value established
///    by the compiler coming into a function but are not supposed to be
///    interpreted as a high-level variable. Typical examples are the
///    direction flag (for \e string instructions) and segment registers.
///    All tracked variables are interpreted as a constant value at the
///    start of a function, although the memory location can be recycled
///    for other calculations later in the function.
///
/// Low-level context variables can be queried and set by name --
/// get_variable_value(), set_variable(), set_variable_region() -- but the
/// disassembler accesses all the variables at an address as a group via
/// get_context(), set_context_change_point(), set_context_region().  In this
/// setting, all the values are packed together in an array of words, a
/// context \e blob (See ContextBitRange).
///
/// Tracked variables are also queried as a group via get_tracked_set() and
/// create_set().  These return a list of TrackedContext objects.
pub trait ContextDatabase {
    /// \brief Retrieve the context variable description object by name
    ///
    /// If the variable doesn't exist an error is returned.  (C++ protected
    /// virtual returning a reference; the Rust returns the `Copy` POD by
    /// value — it is immutable after registration.)
    /// \param nm is the name of the context value
    /// \return the ContextBitRange object matching the name
    fn get_variable(&self, nm: &[u8]) -> KunaResult<ContextBitRange>;

    /// \brief Grab the context blob(s) for the given address range, marking
    /// bits that will be set
    ///
    /// This is an internal routine for obtaining the actual memory regions
    /// holding context values for the address range.  This also informs the
    /// system which bits are getting set. A split is forced at the first
    /// address, and at least one memory region is passed back. The second
    /// address can be invalid in which case the memory region passed back is
    /// valid from the first address to whatever the next split point is.
    /// (The C++ passes back a `vector<uintm *>`; the Rust invokes \b each on
    /// every blob in the same order — see module docs.)
    /// \param addr1 is the starting address of the range
    /// \param addr2 is (1 past) the last address of the range or is invalid
    /// \param num is the word index for the context value that will be set
    /// \param mask is a mask of the value being set (within its word)
    /// \param each is invoked on each memory region in the range
    fn get_region_for_set(
        &mut self,
        addr1: &Address,
        addr2: &Address,
        num: i32,
        mask: u32,
        each: &mut dyn FnMut(&mut [u32]),
    );

    /// \brief Grab the context blob(s) starting at the given address up to
    /// the first point of change
    ///
    /// This is an internal routine for obtaining the actual memory regions
    /// holding context values starting at the given address.  A specific
    /// context value is specified, and all memory regions are visited up to
    /// the first address where that particular context value changes.
    /// \param addr is the starting address of the regions to fetch
    /// \param num is the word index for the specific context value being set
    /// \param mask is a mask of the context value being set (within its word)
    /// \param each is invoked on each memory region being passed back
    fn get_region_to_change_point(
        &mut self,
        addr: &Address,
        num: i32,
        mask: u32,
        each: &mut dyn FnMut(&mut [u32]),
    );

    /// \brief Retrieve the memory region holding all default context values
    ///
    /// This fetches the active memory holding the default context values on
    /// top of which all other context values are overlaid.
    /// \return the memory region holding all the default context values
    fn get_default_value(&self) -> &[u32];

    /// \brief Retrieve the memory region holding all default context values
    /// (mutable flavor of the C++ non-const overload)
    fn get_default_value_mut(&mut self) -> &mut [u32];

    /// \brief Retrieve the number of words (u32) in a context \e blob
    ///
    /// \return the number of words
    fn get_context_size(&self) -> i32;

    /// \brief Register a new named context variable (as a bit range) with
    /// the database
    ///
    /// A new variable is registered by providing a name and the range of
    /// bits the value will occupy within the context blob.  The full blob
    /// size is automatically increased if necessary.  The variable must be
    /// contained within a single word, and all variables must be registered
    /// before any values can be set.
    /// \param nm is the name of the new variable
    /// \param sbit is the position of the variable's most significant bit
    /// within the blob
    /// \param ebit is the position of the variable's least significant bit
    /// within the blob
    fn register_variable(&mut self, nm: &[u8], sbit: i32, ebit: i32) -> KunaResult<()>;

    /// \brief Get the context blob of values associated with a given address
    ///
    /// \param addr is the given address
    /// \return the memory region holding the context values for the address
    fn get_context(&self, addr: &Address) -> &[u32];

    /// \brief Get the context blob of values associated with a given address
    /// and its bounding offsets
    ///
    /// In addition to the memory region, the range of addresses for which
    /// the region is valid is passed back as offsets into the address space.
    /// (C++ `getContext(addr,first,last)`; the Rust returns
    /// `(region, first, last)`.)
    /// \param addr is the given address
    /// \return the memory region, the starting offset of the valid range,
    /// and the ending offset of the valid range
    fn get_context_bounds(&self, addr: &Address) -> (&[u32], u64, u64);

    /// \brief Get the set of default values for all tracked registers
    ///
    /// \return the list of TrackedContext objects
    fn get_tracked_default(&mut self) -> &mut TrackedSet;

    /// \brief Get the set of tracked register values associated with the
    /// given address
    ///
    /// \param addr is the given address
    /// \return the list of TrackedContext objects
    fn get_tracked_set(&self, addr: &Address) -> &TrackedSet;

    /// Snapshot the entire tracked-register part-map (the `set track` database).
    ///
    /// The per-function `ArchHandle` is a detached skeleton that does not hold
    /// the engine's `ContextDatabase`; `ActionConstbase` needs to query the
    /// tracked set for the function's entry address through that skeleton, so
    /// `build_arch_handle` clones this map onto the seam.
    fn clone_trackbase(&self) -> PartMap<Address, TrackedSet>;

    /// \brief Create a tracked register set that is valid over the given
    /// range
    ///
    /// This really should be an internal routine.  The created set is empty,
    /// old values are blown away.  If old/default values are to be
    /// preserved, they must be copied back in.
    /// \param addr1 is the starting address of the given range
    /// \param addr2 is (1 past) the ending address of the given range
    /// \return the empty set of tracked register values
    fn create_set(&mut self, addr1: &Address, addr2: &Address) -> &mut TrackedSet;

    /// \brief Encode the entire database to a stream
    ///
    /// \param encoder is the stream encoder
    fn encode(&self, encoder: &mut dyn Encoder) -> KunaResult<()>;

    /// \brief Restore the state of \b this database object from the given
    /// stream decoder
    ///
    /// \param decoder is the given stream decoder
    fn decode(&mut self, decoder: &mut dyn Decoder) -> KunaResult<()>;

    /// \brief Add initial context state from elements in the
    /// compiler/processor specifications
    ///
    /// Parse a \<context_data> element from the given stream decoder from
    /// either the compiler or processor specification file for the
    /// architecture, initializing this database.
    /// \param decoder is the given stream decoder
    fn decode_from_spec(&mut self, decoder: &mut dyn Decoder) -> KunaResult<()>;

    // -- Non-virtual helper methods (provided) --

    /// Provide a default value for a context variable.
    ///
    /// The default value is returned for addresses that have not been
    /// overlaid with other values.
    /// \param nm is the name of the context variable
    /// \param val is the default value to establish
    fn set_variable_default(&mut self, nm: &[u8], val: u32) -> KunaResult<()> {
        let var = self.get_variable(nm)?;
        var.set_value(self.get_default_value_mut(), val);
        Ok(())
    }

    /// Retrieve the default value for a context variable
    /// (C++ `getDefaultValue(const string &)`).
    ///
    /// This will return the default value used for addresses that have not
    /// been overlaid with other values.
    /// \param nm is the name of the context variable
    /// \return the variable's default value
    fn get_default_value_by_name(&self, nm: &[u8]) -> KunaResult<u32> {
        let var = self.get_variable(nm)?;
        Ok(var.get_value(self.get_default_value()))
    }

    /// Set a context value at the given address.
    ///
    /// The variable will be changed to the new value, starting at the given
    /// address up to the next point of change.
    /// \param nm is the name of the context variable
    /// \param addr is the given address
    /// \param value is the new value to set
    fn set_variable(&mut self, nm: &[u8], addr: &Address, value: u32) -> KunaResult<()> {
        let bitrange = self.get_variable(nm)?;
        let num = bitrange.get_word();
        let mask = bitrange.get_mask() << bitrange.get_shift();

        self.get_region_to_change_point(addr, num, mask, &mut |blob| {
            bitrange.set_value(blob, value)
        });
        Ok(())
    }

    /// Retrieve a context value at the given address
    /// (C++ `getVariable(const string &,const Address &)`).
    ///
    /// If a value has not been explicitly set for an address range
    /// containing the given address, the default value for the variable is
    /// returned.
    /// \param nm is the name of the context variable
    /// \param addr is the address for which the specific value is needed
    /// \return the context variable value for the address
    fn get_variable_value(&self, nm: &[u8], addr: &Address) -> KunaResult<u32> {
        let bitrange = self.get_variable(nm)?;

        let context = self.get_context(addr);
        Ok(bitrange.get_value(context))
    }

    /// \brief Set a specific context value starting at the given address
    ///
    /// The new value is \e painted across an address range, starting with
    /// the given address up to the point where another change for the
    /// variable was specified. No other context variable is changed, inside
    /// (or outside) the range.
    /// \param addr is the given starting address
    /// \param num is the index of the word (within the context blob) of the
    /// context variable
    /// \param mask is the mask delimiting the context variable (within its
    /// word)
    /// \param value is the (already shifted) value being set
    fn set_context_change_point(&mut self, addr: &Address, num: i32, mask: u32, value: u32) {
        self.get_region_to_change_point(addr, num, mask, &mut |newcontext| {
            let word = num as usize; // cast: word index, non-negative
            let mut val = newcontext[word];
            val &= !mask; // Clear range to zero
            val |= value;
            newcontext[word] = val;
        });
    }

    /// \brief Set a context variable value over a given range of addresses
    ///
    /// The new value is \e painted over an explicit range of addresses. No
    /// other context variable is changed inside (or outside) the range.
    /// \param addr1 is the starting address of the given range
    /// \param addr2 is the ending address of the given range
    /// \param num is the index of the word (within the context blob) of the
    /// context variable
    /// \param mask is the mask delimiting the context variable (within its
    /// word)
    /// \param value is the (already shifted) value being set
    fn set_context_region(
        &mut self,
        addr1: &Address,
        addr2: &Address,
        num: i32,
        mask: u32,
        value: u32,
    ) {
        self.get_region_for_set(addr1, addr2, num, mask, &mut |vec| {
            let word = num as usize; // cast: word index, non-negative
            vec[word] = (vec[word] & !mask) | value;
        });
    }

    /// \brief Set a context variable by name over a given range of addresses
    ///
    /// The new value is \e painted over an explicit range of addresses. No
    /// other context variable is changed inside (or outside) the range.
    /// \param nm is the name of the context variable to set
    /// \param begad is the starting address of the given range
    /// \param endad is the ending address of the given range
    /// \param value is the new value to set
    fn set_variable_region(
        &mut self,
        nm: &[u8],
        begad: &Address,
        endad: &Address,
        value: u32,
    ) -> KunaResult<()> {
        let bitrange = self.get_variable(nm)?;

        self.get_region_for_set(
            begad,
            endad,
            bitrange.get_word(),
            bitrange.get_mask() << bitrange.get_shift(),
            &mut |blob| bitrange.set_value(blob, value),
        );
        Ok(())
    }

    /// \brief Get the value of a tracked register at a specific address
    ///
    /// A specific storage region and code address is given.  If the region
    /// is tracked the value at the address is retrieved.  If the specified
    /// storage region is contained in the tracked region, the retrieved
    /// value is trimmed to match the containment before returning it. If the
    /// region is not tracked, a value of 0 is returned.
    /// \param mem is the specified storage region
    /// \param point is the code address
    /// \return the tracked value or zero
    fn get_tracked_value(&self, mem: &VarnodeData, point: &Address) -> u64 {
        let tset = self.get_tracked_set(point);
        // C++ `mem.offset + mem.size - 1`: defined unsigned wraparound
        let endoff = mem.offset.wadd(u64::from(mem.size)).wsub(1);
        for tcont in tset.iter() {
            // tcont must contain -mem- (space comparison is by pointer)
            if !space_ptr_eq(&tcont.loc.space, &mem.space) {
                continue;
            }
            if tcont.loc.offset > mem.offset {
                continue;
            }
            let tendoff = tcont.loc.offset.wadd(u64::from(tcont.loc.size)).wsub(1);
            if tendoff < endoff {
                continue;
            }
            let mut res = tcont.val;
            // If we have proper containment, trim value based on endianness.
            // (C++ dereferences the space pointer here: null is UB.)
            let spc = tcont
                .loc
                .space
                .as_ref()
                .expect("getTrackedValue: null tracked space pointer (C++ UB)");
            // Shift amounts below stay < 64 for tracked registers of size
            // <= 8; anything larger is C++ UB (oversized shift).
            if spc.is_big_endian() {
                if endoff != tendoff {
                    res >>= 8 * (tendoff - mem.offset);
                }
            } else if mem.offset != tcont.loc.offset {
                res >>= 8 * (mem.offset - tcont.loc.offset);
            }
            res &= calc_mask(mem.size as i32); // Final trim based on size (cast: uint4 -> int4 as in C++)
            return res;
        }
        0
    }
}

// ---------------------------------------------------------------------------
// ContextInternal
// ---------------------------------------------------------------------------

/// \brief A context blob, holding context values across some range of code
/// addresses
///
/// This is an internal object that holds the actual "array of words" for a
/// context blob.  An associated mask array holds 1-bits for context
/// variables that were explicitly set for the specific split point.
#[derive(Debug, Default)]
struct FreeArray {
    /// The "array of words" holding context variable values
    array: Vec<u32>,
    /// The mask array indicating which variables are explicitly set
    mask: Vec<u32>,
}

impl FreeArray {
    /// Resize the context blob, preserving old values.
    ///
    /// The "array of words" and mask array are resized to the given value.
    /// Old values are preserved, chopping off the last values, or appending
    /// zeroes, as needed.
    /// \param sz is the new number of words to resize array to
    fn reset(&mut self, sz: i32) {
        let sz = sz as usize; // cast: word count, non-negative by construction
        let mut newarray = vec![0u32; sz]; // Pad new part with zero
        let mut newmask = vec![0u32; sz];
        let min = sz.min(self.array.len());
        newarray[..min].copy_from_slice(&self.array[..min]); // Copy old part
        newmask[..min].copy_from_slice(&self.mask[..min]);
        self.array = newarray;
        self.mask = newmask;
    }
}

impl Clone for FreeArray {
    /// Transcribes the C++ assignment `operator=` (the only copy path the
    /// C++ exercises, via `partmap`'s `database[pnt] = ...`): the value
    /// array is copied at the split point, "but not fact that value is
    /// being set" — the mask is zeroed.
    fn clone(&self) -> FreeArray {
        FreeArray {
            array: self.array.clone(), // Copy value at split point
            mask: vec![0u32; self.mask.len()], // but not fact that value is being set
        }
    }
}

/// \brief An in-memory implementation of the ContextDatabase interface
///
/// Context blobs are held in a partition map on addresses.  Any address
/// within the map indicates a \e split point, where the value of a context
/// variable was explicitly changed.  Sets of tracked registers are held in a
/// separate partition map.
#[derive(Debug, Default)]
pub struct ContextInternal {
    /// Number of words in a context blob (for this architecture)
    size: i32,
    /// Map from context variable name to description object
    variables: BTreeMap<Vec<u8>, ContextBitRange>,
    /// Partition map of context blobs (FreeArray)
    database: PartMap<Address, FreeArray>,
    /// Partition map of tracked register sets
    trackbase: PartMap<Address, TrackedSet>,
}

impl ContextInternal {
    /// Construct an empty context database
    pub fn new() -> ContextInternal {
        ContextInternal::default()
    }

    /// \brief Encode a single context block to a stream
    ///
    /// The blob is broken up into individual values and written out as a
    /// series of \<set> elements within a parent \<context_pointset>
    /// element.
    /// \param encoder is the stream encoder
    /// \param addr is the address of the split point where the blob is valid
    /// \param vec is the array of words holding the blob values
    fn encode_context(
        &self,
        encoder: &mut dyn Encoder,
        addr: &Address,
        vec: &[u32],
    ) -> KunaResult<()> {
        encoder.open_element(&ELEM_CONTEXT_POINTSET);
        addr.get_space()
            .expect("encodeContext: invalid address (C++ UB)")
            .encode_attributes(encoder, addr.get_offset())?;
        for (name, bitrange) in self.variables.iter() {
            let val = bitrange.get_value(vec);
            encoder.open_element(&ELEM_SET);
            encoder.write_string(&ATTRIB_NAME, name);
            encoder.write_unsigned_integer(&ATTRIB_VAL, u64::from(val));
            encoder.close_element(&ELEM_SET);
        }
        encoder.close_element(&ELEM_CONTEXT_POINTSET);
        Ok(())
    }

    /// \brief Restore a context blob for given address range from a stream
    /// decoder
    ///
    /// Parse either a \<context_pointset> or \<context_set> element. In
    /// either case, children are parsed to get context variable values.
    /// Then a context blob is reconstructed from the values.  The new blob
    /// is added to the interval map based on the address range.  If the
    /// start address is invalid, the default value of the context variables
    /// are painted.  The second address can be invalid, if only a split
    /// point is known.
    /// \param decoder is the stream decoder
    /// \param addr1 is the starting address of the given range
    /// \param addr2 is the ending address of the given range
    fn decode_context(
        &mut self,
        decoder: &mut dyn Decoder,
        addr1: &Address,
        addr2: &Address,
    ) -> KunaResult<()> {
        loop {
            let sub_id = decoder.open_element()?;
            if sub_id != ELEM_SET.get_id() {
                break;
            }
            // cast: C++ stores readUnsignedInteger (uintb) into uintm,
            // an implicit truncation
            let val = decoder.read_unsigned_integer_id(&ATTRIB_VAL)? as u32;
            let name = decoder.read_string_id(&ATTRIB_NAME)?;
            let var = self.get_variable(&name)?;
            if addr1.is_invalid() {
                // Invalid addr1, indicates we should set default value.
                // (As in C++, the default blob is re-zeroed for *each*
                // <set> child before the value is painted.)
                let default_buffer = self.get_default_value_mut();
                default_buffer.fill(0);
                var.set_value(default_buffer, val);
            } else {
                self.get_region_for_set(
                    addr1,
                    addr2,
                    var.get_word(),
                    var.get_mask() << var.get_shift(),
                    &mut |blob| var.set_value(blob, val),
                );
            }
            decoder.close_element(sub_id)?;
        }
        Ok(())
    }
}

impl ContextDatabase for ContextInternal {
    fn get_variable(&self, nm: &[u8]) -> KunaResult<ContextBitRange> {
        match self.variables.get(nm) {
            Some(bitrange) => Ok(*bitrange),
            None => Err(KunaError::lowlevel(format!(
                "Non-existent context variable: {}",
                String::from_utf8_lossy(nm)
            ))),
        }
    }

    fn get_region_for_set(
        &mut self,
        addr1: &Address,
        addr2: &Address,
        num: i32,
        mask: u32,
        each: &mut dyn FnMut(&mut [u32]),
    ) {
        self.database.split(addr1);
        // C++: aiter = begin(addr1); biter = addr2 invalid ? end()
        //      : (split(addr2), begin(addr2))
        if !addr2.is_invalid() {
            self.database.split(addr2);
        }
        for (key, freearray) in self.database.iter_from_mut(addr1) {
            if !addr2.is_invalid() && key >= addr2 {
                break; // reached biter
            }
            let word = num as usize; // cast: word index, non-negative
            freearray.mask[word] |= mask; // Mark that this value is being definitely set
            each(&mut freearray.array);
        }
    }

    fn get_region_to_change_point(
        &mut self,
        addr: &Address,
        num: i32,
        mask: u32,
        each: &mut dyn FnMut(&mut [u32]),
    ) {
        self.database.split(addr);
        let word = num as usize; // cast: word index, non-negative
        let mut iter = self.database.iter_from_mut(addr);
        match iter.next() {
            None => return, // C++ `if (aiter == biter) return`
            Some((_, freearray)) => {
                freearray.mask[word] |= mask;
                each(&mut freearray.array);
            }
        }
        for (_, freearray) in iter {
            if (freearray.mask[word] & mask) != 0 {
                break; // Reached point where this value was definitively set before
            }
            each(&mut freearray.array);
        }
    }

    fn get_default_value(&self) -> &[u32] {
        &self.database.default_value().array
    }

    fn get_default_value_mut(&mut self) -> &mut [u32] {
        &mut self.database.default_value_mut().array
    }

    fn get_context_size(&self) -> i32 {
        self.size
    }

    fn register_variable(&mut self, nm: &[u8], sbit: i32, ebit: i32) -> KunaResult<()> {
        if !self.database.empty() {
            return Err(KunaError::lowlevel(
                "Cannot register new context variables after database is initialized",
            ));
        }
        let bitrange = ContextBitRange::new(sbit, ebit);
        // 8*sizeof(uintm) == 32 (C++ unsigned division; identical for the
        // non-negative bit positions)
        let sz = sbit / 32 + 1;
        if ebit / 32 + 1 != sz {
            return Err(KunaError::lowlevel("Context variable does not fit in one word"));
        }
        if sz > self.size {
            self.size = sz;
            self.database.default_value_mut().reset(self.size);
        }
        self.variables.insert(nm.to_vec(), bitrange);
        Ok(())
    }

    fn get_context(&self, addr: &Address) -> &[u32] {
        &self.database.get_value(addr).array
    }

    fn get_context_bounds(&self, addr: &Address) -> (&[u32], u64, u64) {
        let (value, before, after, valid) = self.database.bounds(addr);
        let res = value.array.as_slice();
        // C++: if (((valid&1)!=0)||(before.getSpace() != addr.getSpace()))
        //        first = 0; else first = before.getOffset();
        let first = match before {
            Some(before) if (valid & 1) == 0 => {
                if space_eq(before, addr) {
                    before.get_offset()
                } else {
                    0
                }
            }
            _ => 0,
        };
        // C++: if (((valid&2)!=0)||(after.getSpace() != addr.getSpace()))
        //        last = addr.getSpace()->getHighest();
        //      else last = after.getOffset()-1;
        let last = match after {
            Some(after) if (valid & 2) == 0 && space_eq(after, addr) => {
                // C++ `after.getOffset()-1`: defined unsigned wraparound
                after.get_offset().wsub(1)
            }
            _ => addr
                .get_space()
                .expect("getContext: invalid address (C++ UB)")
                .get_highest(),
        };
        (res, first, last)
    }

    fn get_tracked_default(&mut self) -> &mut TrackedSet {
        self.trackbase.default_value_mut()
    }

    fn get_tracked_set(&self, addr: &Address) -> &TrackedSet {
        self.trackbase.get_value(addr)
    }

    fn clone_trackbase(&self) -> PartMap<Address, TrackedSet> {
        self.trackbase.clone()
    }

    fn create_set(&mut self, addr1: &Address, addr2: &Address) -> &mut TrackedSet {
        let res = self.trackbase.clear_range(addr1, addr2);
        res.clear();
        res
    }

    fn encode(&self, encoder: &mut dyn Encoder) -> KunaResult<()> {
        if self.database.empty() && self.trackbase.empty() {
            return Ok(());
        }

        encoder.open_element(&ELEM_CONTEXT_POINTS);

        // Save context at each changepoint
        for (addr, freearray) in self.database.iter() {
            self.encode_context(encoder, addr, &freearray.array)?;
        }

        for (addr, trackedset) in self.trackbase.iter() {
            encode_tracked(encoder, addr, trackedset)?;
        }

        encoder.close_element(&ELEM_CONTEXT_POINTS);
        Ok(())
    }

    fn decode(&mut self, decoder: &mut dyn Decoder) -> KunaResult<()> {
        let elem_id = decoder.open_element_id(&ELEM_CONTEXT_POINTS)?;
        loop {
            let sub_id = decoder.open_element()?;
            if sub_id == 0 {
                break;
            }
            if sub_id == ELEM_CONTEXT_POINTSET.get_id() {
                let attrib_id = decoder.get_next_attribute_id()?;
                decoder.rewind_attributes();
                if attrib_id == 0 {
                    // Restore the default value
                    self.decode_context(decoder, &Address::default(), &Address::default())?;
                } else {
                    let mut v_data = VarnodeData::default();
                    v_data.decode_from_attributes(decoder)?;
                    self.decode_context(decoder, &v_data.get_addr(), &Address::default())?;
                }
            } else if sub_id == ELEM_TRACKED_POINTSET.get_id() {
                let mut v_data = VarnodeData::default();
                v_data.decode_from_attributes(decoder)?;
                let addr = v_data.get_addr();
                decode_tracked(decoder, self.trackbase.split(&addr))?;
            } else {
                return Err(KunaError::lowlevel("Bad <context_points> tag"));
            }
            decoder.close_element(sub_id)?;
        }
        decoder.close_element(elem_id)
    }

    fn decode_from_spec(&mut self, decoder: &mut dyn Decoder) -> KunaResult<()> {
        let elem_id = decoder.open_element_id(&ELEM_CONTEXT_DATA)?;
        loop {
            let sub_id = decoder.open_element()?;
            if sub_id == 0 {
                break;
            }
            // There MUST be a range
            let range = Range::decode_from_attributes(decoder)?;
            let addr1 = range.get_first_addr();
            let addr2 = range.get_last_addr_open(decoder.get_addr_space_manager());
            if sub_id == ELEM_CONTEXT_SET.get_id() {
                self.decode_context(decoder, &addr1, &addr2)?;
            } else if sub_id == ELEM_TRACKED_SET.get_id() {
                decode_tracked(decoder, self.create_set(&addr1, &addr2))?;
            } else {
                return Err(KunaError::lowlevel("Bad <context_data> tag"));
            }
            decoder.close_element(sub_id)?;
        }
        decoder.close_element(elem_id)
    }
}

/// C++ `before.getSpace() != addr.getSpace()` style raw-pointer comparison
/// between the spaces of two addresses (null == null).
fn space_eq(a: &Address, b: &Address) -> bool {
    match (a.get_space(), b.get_space()) {
        (Some(a), Some(b)) => Rc::ptr_eq(a, b),
        (None, None) => true,
        _ => false,
    }
}

// ---------------------------------------------------------------------------
// ContextCache
// ---------------------------------------------------------------------------

/// \brief A helper class for caching the active context blob to minimize
/// database lookups
///
/// This caches the range of addresses over which the current context blob is
/// valid.  It exposes a minimal interface (get_context() and set_context()).
///
/// Differences from the C++ (see module docs): the encapsulated
/// `ContextDatabase *` is an explicit parameter on each method, and instead
/// of caching the raw blob pointer the cache re-fetches the blob through the
/// single-lookup [`ContextDatabase::get_context`] on a hit (the expensive
/// bounds query is what the cache skips).
#[derive(Debug)]
pub struct ContextCache {
    /// If set to \b false, any set_context() call is dropped
    allowset: bool,
    /// Address space of the current valid range (`None` = cache invalid,
    /// the C++ null `curspace`)
    curspace: Option<Rc<AddrSpace>>,
    /// Starting offset of the current valid range
    first: u64,
    /// Ending offset of the current valid range
    last: u64,
}

impl Default for ContextCache {
    fn default() -> ContextCache {
        ContextCache::new()
    }
}

impl ContextCache {
    /// Construct a context cache (C++ takes the context database that will
    /// be encapsulated; here the database is passed to each method).
    pub fn new() -> ContextCache {
        ContextCache {
            curspace: None, // Mark cache as invalid
            allowset: true,
            // C++ leaves first/last uninitialized (they are only read once
            // curspace is set); zeroed here
            first: 0,
            last: 0,
        }
    }

    /// Toggle whether set_context() calls are ignored
    pub fn allow_set(&mut self, val: bool) {
        self.allowset = val;
    }

    /// Return \b true if the cached range covers the given address (the
    /// C++ `addr.getSpace()==curspace` raw-pointer test plus offset bounds).
    fn cache_covers(&self, addr: &Address) -> bool {
        let space_hit = match (&self.curspace, addr.get_space()) {
            (Some(cur), Some(spc)) => Rc::ptr_eq(cur, spc),
            _ => false,
        };
        space_hit && self.first <= addr.get_offset() && self.last >= addr.get_offset()
    }

    /// Retrieve the context blob for the given address.
    ///
    /// Check if the address is in the current valid range. If it is not,
    /// make a (bounds) call to the database and cache a new valid range.
    /// (C++ is `const` with mutable cache fields; Rust takes `&mut self`.)
    /// \param database is the context database (the C++ encapsulated member)
    /// \param addr is the given address
    /// \param buf is where the blob should be stored
    pub fn get_context(&mut self, database: &dyn ContextDatabase, addr: &Address, buf: &mut [u32]) {
        // C++: (addr.getSpace()!=curspace)||(first>addr.getOffset())||
        //      (last<addr.getOffset())
        let n = database.get_context_size() as usize; // cast: word count
        if !self.cache_covers(addr) {
            self.curspace = addr.get_space().cloned();
            let (context, first, last) = database.get_context_bounds(addr);
            self.first = first;
            self.last = last;
            buf[..n].copy_from_slice(&context[..n]);
        } else {
            // Cache hit: the C++ copies from its cached blob pointer; the
            // port re-fetches the same blob with the cheap single lookup
            // (see module docs).
            let context = database.get_context(addr);
            buf[..n].copy_from_slice(&context[..n]);
        }
    }

    /// \brief Change the value of a context variable at the given address
    /// with no bound
    ///
    /// The context value is set starting at the given address and \e paints
    /// memory up to the next explicit change point.
    /// \param database is the context database (the C++ encapsulated member)
    /// \param addr is the given starting address
    /// \param num is the word index of the context variable
    /// \param mask is the mask delimiting the context variable
    /// \param value is the (already shifted) value to set
    pub fn set_context(
        &mut self,
        database: &mut dyn ContextDatabase,
        addr: &Address,
        num: i32,
        mask: u32,
        value: u32,
    ) {
        if !self.allowset {
            return;
        }
        database.set_context_change_point(addr, num, mask, value);
        if self.cache_covers(addr) {
            self.curspace = None; // Invalidate cache
        }
    }

    /// \brief Change the value of a context variable across an explicit
    /// address range (C++ `setContext(addr1,addr2,...)`)
    ///
    /// The context value is \e painted across the range. The context
    /// variable is marked as explicitly changing at the starting address of
    /// the range.
    /// \param database is the context database (the C++ encapsulated member)
    /// \param addr1 is the starting address of the given range
    /// \param addr2 is the ending address of the given range
    /// \param num is the word index of the context variable
    /// \param mask is the mask delimiting the context variable
    /// \param value is the (already shifted) value to set
    pub fn set_context_region(
        &mut self,
        database: &mut dyn ContextDatabase,
        addr1: &Address,
        addr2: &Address,
        num: i32,
        mask: u32,
        value: u32,
    ) {
        if !self.allowset {
            return;
        }
        database.set_context_region(addr1, addr2, num, mask, value);
        if self.cache_covers(addr1) {
            self.curspace = None; // Invalidate cache
        }
        // The C++ second and third tests compare offsets only (no space
        // check) — transcribed as-is.
        if self.first <= addr2.get_offset() && self.last >= addr2.get_offset() {
            self.curspace = None; // Invalidate cache
        }
        if self.first >= addr1.get_offset() && self.first <= addr2.get_offset() {
            self.curspace = None; // Invalidate cache
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use kuna_base::marshal::{PackedDecode, PackedEncode, XmlDecode, XmlEncode};
    use kuna_base::space::{addrspace_flags, spacetype, AddrSpaceManager, ConstantSpace};

    /// Build a manager with a constant space (index 0), two 4-byte code
    /// spaces "ram" (index 1) and "ram2" (index 2), a little-endian
    /// register space "reg" (index 3) and a big-endian register space
    /// "breg" (index 4).
    fn test_manager() -> AddrSpaceManager {
        let mut manager = AddrSpaceManager::new();
        manager.insert_space(Rc::new(ConstantSpace::new())).unwrap();
        for (i, (name, bigend)) in
            [("ram", false), ("ram2", false), ("reg", false), ("breg", true)].iter().enumerate()
        {
            let spc = AddrSpace::new(
                spacetype::IPTR_PROCESSOR,
                name,
                *bigend,
                4,
                1,
                (i + 1) as i32, // cast: small test index
                addrspace_flags::hasphysical,
                1,
                1,
            );
            manager.insert_space(Rc::new(spc)).unwrap();
        }
        manager
    }

    fn space(manager: &AddrSpaceManager, nm: &str) -> Rc<AddrSpace> {
        Rc::clone(manager.get_space_by_name(nm).unwrap())
    }

    fn addr(manager: &AddrSpaceManager, nm: &str, off: u64) -> Address {
        Address::new(space(manager, nm), off)
    }

    fn test_registry() -> kuna_base::marshal::IdRegistry {
        let mut registry = kuna_base::marshal::IdRegistry::with_base_ids();
        register_globalcontext_ids(&mut registry);
        registry
    }

    /// A database with two one-word variables and one second-word variable,
    /// mirroring a typical processor-spec registration.
    fn test_db() -> ContextInternal {
        let mut db = ContextInternal::new();
        db.register_variable(b"fldA", 0, 7).unwrap();
        db.register_variable(b"fldB", 8, 15).unwrap();
        db.register_variable(b"flag", 16, 16).unwrap();
        db.register_variable(b"wide", 32, 63).unwrap();
        db
    }

    fn get(db: &ContextInternal, manager: &AddrSpaceManager, nm: &[u8], off: u64) -> u32 {
        db.get_variable_value(nm, &addr(manager, "ram", off)).unwrap()
    }

    #[test]
    fn test_globalcontext_bitrange_pack_unpack() {
        let r = ContextBitRange::new(0, 7);
        assert_eq!((r.get_word(), r.get_shift(), r.get_mask()), (0, 24, 0xff));
        let r2 = ContextBitRange::new(4, 6);
        assert_eq!((r2.get_word(), r2.get_shift(), r2.get_mask()), (0, 25, 0x7));
        let r3 = ContextBitRange::new(32, 47);
        assert_eq!((r3.get_word(), r3.get_shift(), r3.get_mask()), (1, 16, 0xffff));
        let bit = ContextBitRange::new(8, 8);
        assert_eq!((bit.get_word(), bit.get_shift(), bit.get_mask()), (0, 23, 0x1));
        // full-word value
        let full = ContextBitRange::new(0, 31);
        assert_eq!((full.get_word(), full.get_shift(), full.get_mask()), (0, 0, 0xffff_ffff));

        let mut blob = [0u32; 2];
        r.set_value(&mut blob, 0xab);
        r3.set_value(&mut blob, 0x1234);
        bit.set_value(&mut blob, 1);
        assert_eq!(r.get_value(&blob), 0xab);
        assert_eq!(r3.get_value(&blob), 0x1234);
        assert_eq!(bit.get_value(&blob), 1);
        assert_eq!(blob[0], 0xab80_0000);
        assert_eq!(blob[1], 0x1234_0000);
        // overwrite only touches its own bits; oversized values are masked
        r.set_value(&mut blob, 0x1cd);
        assert_eq!(r.get_value(&blob), 0xcd);
        assert_eq!(bit.get_value(&blob), 1);
        assert_eq!(r3.get_value(&blob), 0x1234);
    }

    #[test]
    fn test_globalcontext_register_and_defaults() {
        let manager = test_manager();
        let mut db = test_db();
        assert_eq!(db.get_context_size(), 2);
        // default values are served everywhere before any set
        db.set_variable_default(b"fldA", 0xab).unwrap();
        db.set_variable_default(b"wide", 0xdead_beef).unwrap();
        assert_eq!(db.get_default_value_by_name(b"fldA").unwrap(), 0xab);
        assert_eq!(db.get_default_value_by_name(b"wide").unwrap(), 0xdead_beef);
        assert_eq!(get(&db, &manager, b"fldA", 0x0), 0xab);
        assert_eq!(get(&db, &manager, b"wide", 0xffff_ffff), 0xdead_beef);
        // unknown variable -> C++ LowlevelError
        let err = db.get_variable_value(b"nope", &addr(&manager, "ram", 0)).unwrap_err();
        assert_eq!(err.to_string(), "Non-existent context variable: nope");
        // a variable crossing a word boundary is rejected
        let err = db.register_variable(b"bad", 30, 33).unwrap_err();
        assert_eq!(err.to_string(), "Context variable does not fit in one word");
        // once the database has split points, registration is frozen
        db.set_variable(b"fldA", &addr(&manager, "ram", 0x100), 1).unwrap();
        let err = db.register_variable(b"late", 17, 18).unwrap_err();
        assert_eq!(
            err.to_string(),
            "Cannot register new context variables after database is initialized"
        );
    }

    /// The core change-point semantics: a set paints from its address up to
    /// the next point where the SAME variable was explicitly set, flowing
    /// across split points belonging to OTHER variables.
    #[test]
    fn test_globalcontext_region_layering_across_boundaries() {
        let manager = test_manager();
        let mut db = test_db();
        // A set at 0x100 paints from 0x100 to the end of the space
        db.set_variable(b"fldA", &addr(&manager, "ram", 0x100), 5).unwrap();
        assert_eq!(get(&db, &manager, b"fldA", 0xff), 0);
        assert_eq!(get(&db, &manager, b"fldA", 0x100), 5);
        assert_eq!(get(&db, &manager, b"fldA", 0x5000), 5);
        // B set at 0x200: A keeps its painted value across the new split
        db.set_variable(b"fldB", &addr(&manager, "ram", 0x200), 7).unwrap();
        assert_eq!(get(&db, &manager, b"fldB", 0x200), 7);
        assert_eq!(get(&db, &manager, b"fldB", 0x1ff), 0);
        assert_eq!(get(&db, &manager, b"fldA", 0x200), 5);
        // A set at 0x50 stops painting at 0x100 (A's mask is set there)
        db.set_variable(b"fldA", &addr(&manager, "ram", 0x50), 9).unwrap();
        assert_eq!(get(&db, &manager, b"fldA", 0x4f), 0);
        assert_eq!(get(&db, &manager, b"fldA", 0x50), 9);
        assert_eq!(get(&db, &manager, b"fldA", 0xff), 9);
        assert_eq!(get(&db, &manager, b"fldA", 0x100), 5);
        // B set at 0x50 flows THROUGH the 0x100 boundary (only A was set
        // there) and stops at 0x200 where B was set
        db.set_variable(b"fldB", &addr(&manager, "ram", 0x50), 3).unwrap();
        assert_eq!(get(&db, &manager, b"fldB", 0x50), 3);
        assert_eq!(get(&db, &manager, b"fldB", 0x100), 3);
        assert_eq!(get(&db, &manager, b"fldB", 0x1ff), 3);
        assert_eq!(get(&db, &manager, b"fldB", 0x200), 7);
        // ... and A was not disturbed by any B operation
        assert_eq!(get(&db, &manager, b"fldA", 0x80), 9);
        assert_eq!(get(&db, &manager, b"fldA", 0x150), 5);
    }

    /// setVariableRegion paints exactly [begad, endad); the split forced at
    /// endad copies values but NOT the explicitly-set mask (FreeArray
    /// operator=), so a later change-point set flows through endad.
    #[test]
    fn test_globalcontext_set_variable_region() {
        let manager = test_manager();
        let mut db = test_db();
        db.set_variable_region(
            b"fldA",
            &addr(&manager, "ram", 0x1000),
            &addr(&manager, "ram", 0x2000),
            4,
        )
        .unwrap();
        assert_eq!(get(&db, &manager, b"fldA", 0xfff), 0);
        assert_eq!(get(&db, &manager, b"fldA", 0x1000), 4);
        assert_eq!(get(&db, &manager, b"fldA", 0x1fff), 4);
        assert_eq!(get(&db, &manager, b"fldA", 0x2000), 0);
        // a set below the region stops at its first boundary (mask set at
        // 0x1000)
        db.set_variable(b"fldA", &addr(&manager, "ram", 0x500), 2).unwrap();
        assert_eq!(get(&db, &manager, b"fldA", 0x500), 2);
        assert_eq!(get(&db, &manager, b"fldA", 0xfff), 2);
        assert_eq!(get(&db, &manager, b"fldA", 0x1000), 4);
        // a set inside the region paints through the region's end split
        // (0x2000 has no explicitly-set mask) out to the end of the space
        db.set_variable(b"fldA", &addr(&manager, "ram", 0x1800), 6).unwrap();
        assert_eq!(get(&db, &manager, b"fldA", 0x17ff), 4);
        assert_eq!(get(&db, &manager, b"fldA", 0x1800), 6);
        assert_eq!(get(&db, &manager, b"fldA", 0x2000), 6);
        assert_eq!(get(&db, &manager, b"fldA", 0x9999), 6);
        // an invalid second address paints from the first address through
        // every remaining split point (C++ biter = database.end())
        db.set_variable_region(
            b"fldB",
            &addr(&manager, "ram", 0x1900),
            &Address::default(),
            1,
        )
        .unwrap();
        assert_eq!(get(&db, &manager, b"fldB", 0x18ff), 0);
        assert_eq!(get(&db, &manager, b"fldB", 0x1900), 1);
        assert_eq!(get(&db, &manager, b"fldB", 0x2000), 1);
        assert_eq!(get(&db, &manager, b"fldB", 0x9999), 1);
    }

    /// The raw word/mask interface used by SLEIGH context commits.
    #[test]
    fn test_globalcontext_raw_context_set() {
        let manager = test_manager();
        let mut db = test_db();
        let var = db.get_variable(b"fldB").unwrap();
        let num = var.get_word();
        let mask = var.get_mask() << var.get_shift();
        // (already shifted) value 0x42 in fldB's field
        db.set_context_change_point(&addr(&manager, "ram", 0x100), num, mask, 0x42 << var.get_shift());
        assert_eq!(get(&db, &manager, b"fldB", 0x100), 0x42);
        assert_eq!(get(&db, &manager, b"fldB", 0xff), 0);
        db.set_context_region(
            &addr(&manager, "ram", 0x40),
            &addr(&manager, "ram", 0x80),
            num,
            mask,
            0x7 << var.get_shift(),
        );
        assert_eq!(get(&db, &manager, b"fldB", 0x40), 0x7);
        assert_eq!(get(&db, &manager, b"fldB", 0x7f), 0x7);
        assert_eq!(get(&db, &manager, b"fldB", 0x80), 0);
        assert_eq!(get(&db, &manager, b"fldB", 0x100), 0x42);
    }

    #[test]
    fn test_globalcontext_get_context_bounds() {
        let manager = test_manager();
        let mut db = test_db();
        let ram_highest = space(&manager, "ram").get_highest();
        // empty database: whole space
        let (_, first, last) = db.get_context_bounds(&addr(&manager, "ram", 0x500));
        assert_eq!((first, last), (0, ram_highest));
        db.set_variable(b"fldA", &addr(&manager, "ram", 0x100), 5).unwrap();
        // below the split: no lower bound in this space
        let (blob, first, last) = db.get_context_bounds(&addr(&manager, "ram", 0x80));
        assert_eq!((first, last), (0, 0xff));
        assert_eq!(db.get_variable(b"fldA").unwrap().get_value(blob), 0);
        // at/after the split: bounded below by the split, unbounded above
        let (blob, first, last) = db.get_context_bounds(&addr(&manager, "ram", 0x100));
        assert_eq!((first, last), (0x100, ram_highest));
        assert_eq!(db.get_variable(b"fldA").unwrap().get_value(blob), 5);
        // a neighboring space: the ram split sorts before every ram2
        // address but belongs to another space, so first collapses to 0
        let ram2_highest = space(&manager, "ram2").get_highest();
        let (_, first, last) = db.get_context_bounds(&addr(&manager, "ram2", 0x500));
        assert_eq!((first, last), (0, ram2_highest));
    }

    #[test]
    fn test_globalcontext_cache_flow_boundary_invalidation() {
        let manager = test_manager();
        let mut db = test_db();
        let var = db.get_variable(b"fldA").unwrap();
        let (num, mask) = (var.get_word(), var.get_mask() << var.get_shift());
        // a pre-existing explicit set at 0x200
        db.set_variable(b"fldA", &addr(&manager, "ram", 0x200), 7).unwrap();

        let mut cache = ContextCache::new();
        let mut buf = [0u32; 2];
        cache.get_context(&db, &addr(&manager, "ram", 0x100), &mut buf);
        assert_eq!(var.get_value(&buf), 0); // default below 0x200
        cache.get_context(&db, &addr(&manager, "ram", 0x300), &mut buf);
        assert_eq!(var.get_value(&buf), 7);

        // a set within the cached range invalidates the cache, and the
        // paint stops at the 0x200 flow boundary
        cache.get_context(&db, &addr(&manager, "ram", 0x100), &mut buf);
        cache.set_context(&mut db, &addr(&manager, "ram", 0x180), num, mask, 5 << var.get_shift());
        cache.get_context(&db, &addr(&manager, "ram", 0x190), &mut buf);
        assert_eq!(var.get_value(&buf), 5);
        cache.get_context(&db, &addr(&manager, "ram", 0x17f), &mut buf);
        assert_eq!(var.get_value(&buf), 0);
        cache.get_context(&db, &addr(&manager, "ram", 0x210), &mut buf);
        assert_eq!(var.get_value(&buf), 7);

        // a paint from BELOW the cached range that reaches into it is still
        // observed (the cache serves fresh blob values; C++ reads through
        // its cached pointer the same way)
        cache.get_context(&db, &addr(&manager, "ram", 0x300), &mut buf); // cache [0x200,highest]
        cache.set_context(&mut db, &addr(&manager, "ram", 0x1c0), num, mask, 3 << var.get_shift());
        // paint [0x1c0,0x200) only -- 0x200 has fldA's mask set
        cache.get_context(&db, &addr(&manager, "ram", 0x300), &mut buf);
        assert_eq!(var.get_value(&buf), 7);
        cache.get_context(&db, &addr(&manager, "ram", 0x1f0), &mut buf);
        assert_eq!(var.get_value(&buf), 3);

        // allow_set(false) drops sets entirely
        cache.allow_set(false);
        cache.set_context(&mut db, &addr(&manager, "ram", 0x300), num, mask, 1 << var.get_shift());
        cache.get_context(&db, &addr(&manager, "ram", 0x300), &mut buf);
        assert_eq!(var.get_value(&buf), 7);
        cache.allow_set(true);

        // region flavor with its three invalidation tests
        cache.set_context_region(
            &mut db,
            &addr(&manager, "ram", 0x400),
            &addr(&manager, "ram", 0x500),
            num,
            mask,
            9 << var.get_shift(),
        );
        cache.get_context(&db, &addr(&manager, "ram", 0x450), &mut buf);
        assert_eq!(var.get_value(&buf), 9);
        cache.get_context(&db, &addr(&manager, "ram", 0x500), &mut buf);
        assert_eq!(var.get_value(&buf), 7);
    }

    #[test]
    fn test_globalcontext_tracked_value_trimming() {
        let manager = test_manager();
        let mut db = test_db();
        let point = addr(&manager, "ram", 0x1000);
        {
            let tset = db.get_tracked_default();
            tset.push(TrackedContext {
                loc: VarnodeData { space: Some(space(&manager, "reg")), offset: 0, size: 8 },
                val: 0x1122_3344_5566_7788,
            });
            tset.push(TrackedContext {
                loc: VarnodeData { space: Some(space(&manager, "breg")), offset: 0x10, size: 8 },
                val: 0x1122_3344_5566_7788,
            });
        }
        // exact match
        let mem = VarnodeData { space: Some(space(&manager, "reg")), offset: 0, size: 8 };
        assert_eq!(db.get_tracked_value(&mem, &point), 0x1122_3344_5566_7788);
        // little endian: sub-register at +4 takes the high bytes
        let mem = VarnodeData { space: Some(space(&manager, "reg")), offset: 4, size: 4 };
        assert_eq!(db.get_tracked_value(&mem, &point), 0x1122_3344);
        // little endian: sub-register at +0 takes the low bytes
        let mem = VarnodeData { space: Some(space(&manager, "reg")), offset: 0, size: 4 };
        assert_eq!(db.get_tracked_value(&mem, &point), 0x5566_7788);
        // big endian, single byte reads: last byte is unshifted, first byte
        // is shifted down by the full distance to the tracked end
        let mem = VarnodeData { space: Some(space(&manager, "breg")), offset: 0x17, size: 1 };
        assert_eq!(db.get_tracked_value(&mem, &point), 0x88);
        let mem = VarnodeData { space: Some(space(&manager, "breg")), offset: 0x10, size: 1 };
        assert_eq!(db.get_tracked_value(&mem, &point), 0x11);
        // big endian, multi-byte sub-register: the C++ shift is
        // `8*(tendoff - mem.offset)` (NOT `tendoff - endoff`), which
        // over-shifts unless the read is one byte or the whole register —
        // transcribed as-is, the C++ is the spec
        let mem = VarnodeData { space: Some(space(&manager, "breg")), offset: 0x10, size: 4 };
        assert_eq!(db.get_tracked_value(&mem, &point), 0x11);
        // big endian exact match is returned untrimmed
        let mem = VarnodeData { space: Some(space(&manager, "breg")), offset: 0x10, size: 8 };
        assert_eq!(db.get_tracked_value(&mem, &point), 0x1122_3344_5566_7788);
        // not contained / not tracked -> 0
        let mem = VarnodeData { space: Some(space(&manager, "reg")), offset: 6, size: 4 };
        assert_eq!(db.get_tracked_value(&mem, &point), 0);
        let mem = VarnodeData { space: Some(space(&manager, "ram")), offset: 0, size: 4 };
        assert_eq!(db.get_tracked_value(&mem, &point), 0);
    }

    #[test]
    fn test_globalcontext_create_set_ranges() {
        let manager = test_manager();
        let mut db = test_db();
        {
            let tset = db.create_set(&addr(&manager, "ram", 0x100), &addr(&manager, "ram", 0x200));
            tset.push(TrackedContext {
                loc: VarnodeData { space: Some(space(&manager, "reg")), offset: 0, size: 4 },
                val: 0x55,
            });
        }
        let mem = VarnodeData { space: Some(space(&manager, "reg")), offset: 0, size: 4 };
        assert_eq!(db.get_tracked_value(&mem, &addr(&manager, "ram", 0x150)), 0x55);
        // before the range and at/after its open end: the (empty) copies
        assert_eq!(db.get_tracked_value(&mem, &addr(&manager, "ram", 0xff)), 0);
        assert_eq!(db.get_tracked_value(&mem, &addr(&manager, "ram", 0x200)), 0);
        assert!(db.get_tracked_set(&addr(&manager, "ram", 0x150)).len() == 1);
        assert!(db.get_tracked_set(&addr(&manager, "ram", 0x200)).is_empty());
    }

    #[test]
    fn test_globalcontext_trackedcontext_roundtrip() {
        let manager = test_manager();
        let registry = test_registry();
        let tcont = TrackedContext {
            loc: VarnodeData { space: Some(space(&manager, "reg")), offset: 0x8, size: 4 },
            val: 0x1234_5678,
        };
        // XML
        let mut stream: Vec<u8> = Vec::new();
        {
            let mut encoder = XmlEncode::new(&mut stream);
            tcont.encode(&mut encoder).unwrap();
        }
        let mut decoder = XmlDecode::new(&manager, &registry);
        decoder.ingest_stream(&stream).unwrap();
        let mut back = TrackedContext::default();
        back.decode(&mut decoder).unwrap();
        assert_eq!(back, tcont);
        // Packed
        let mut stream: Vec<u8> = Vec::new();
        {
            let mut encoder = PackedEncode::new(&mut stream);
            tcont.encode(&mut encoder).unwrap();
        }
        let mut decoder = PackedDecode::new(&manager);
        decoder.ingest_stream(&stream).unwrap();
        let mut back = TrackedContext::default();
        back.decode(&mut decoder).unwrap();
        assert_eq!(back, tcont);
    }

    /// Full database round trip <context_points> via both codecs.
    #[test]
    fn test_globalcontext_database_roundtrip() {
        let manager = test_manager();
        let registry = test_registry();
        let mut db = test_db();
        // nothing recorded -> nothing encoded (C++ early return)
        let mut empty: Vec<u8> = Vec::new();
        {
            let mut encoder = XmlEncode::new(&mut empty);
            db.encode(&mut encoder).unwrap();
        }
        assert!(empty.is_empty());

        db.set_variable(b"fldA", &addr(&manager, "ram", 0x100), 5).unwrap();
        db.set_variable(b"fldB", &addr(&manager, "ram", 0x200), 7).unwrap();
        db.set_variable(b"wide", &addr(&manager, "ram", 0x100), 0xcafe).unwrap();
        {
            let tset = db.create_set(&addr(&manager, "ram", 0x300), &addr(&manager, "ram", 0x400));
            tset.push(TrackedContext {
                loc: VarnodeData { space: Some(space(&manager, "reg")), offset: 0, size: 4 },
                val: 0x99,
            });
        }

        for codec in ["xml", "packed"] {
            let mut stream: Vec<u8> = Vec::new();
            if codec == "xml" {
                let mut encoder = XmlEncode::new(&mut stream);
                db.encode(&mut encoder).unwrap();
            } else {
                let mut encoder = PackedEncode::new(&mut stream);
                db.encode(&mut encoder).unwrap();
            }
            // decode into a freshly-registered database
            let mut back = test_db();
            if codec == "xml" {
                let mut decoder = XmlDecode::new(&manager, &registry);
                decoder.ingest_stream(&stream).unwrap();
                back.decode(&mut decoder).unwrap();
            } else {
                let mut decoder = PackedDecode::new(&manager);
                decoder.ingest_stream(&stream).unwrap();
                back.decode(&mut decoder).unwrap();
            }
            assert_eq!(get(&back, &manager, b"fldA", 0xff), 0, "{codec}");
            assert_eq!(get(&back, &manager, b"fldA", 0x100), 5, "{codec}");
            assert_eq!(get(&back, &manager, b"fldA", 0x250), 5, "{codec}");
            assert_eq!(get(&back, &manager, b"fldB", 0x1ff), 0, "{codec}");
            assert_eq!(get(&back, &manager, b"fldB", 0x200), 7, "{codec}");
            assert_eq!(get(&back, &manager, b"wide", 0x100), 0xcafe, "{codec}");
            let mem = VarnodeData { space: Some(space(&manager, "reg")), offset: 0, size: 4 };
            assert_eq!(back.get_tracked_value(&mem, &addr(&manager, "ram", 0x350)), 0x99, "{codec}");
            assert_eq!(back.get_tracked_value(&mem, &addr(&manager, "ram", 0x2ff)), 0, "{codec}");
        }
    }

    /// decodeFromSpec: the <context_data> element from a processor spec.
    #[test]
    fn test_globalcontext_decode_from_spec() {
        let manager = test_manager();
        let registry = test_registry();
        let mut db = test_db();
        let mut decoder = XmlDecode::new(&manager, &registry);
        decoder
            .ingest_stream(
                b"<context_data>\
                  <context_set space=\"ram\" first=\"0x100\" last=\"0x1ff\">\
                    <set name=\"fldA\" val=\"3\"/>\
                    <set name=\"flag\" val=\"1\"/>\
                  </context_set>\
                  <tracked_set space=\"ram\" first=\"0x100\" last=\"0x1ff\">\
                    <set space=\"reg\" offset=\"0x0\" size=\"4\" val=\"0x55\"/>\
                  </tracked_set>\
                </context_data>",
            )
            .unwrap();
        db.decode_from_spec(&mut decoder).unwrap();
        assert_eq!(get(&db, &manager, b"fldA", 0x100), 3);
        assert_eq!(get(&db, &manager, b"fldA", 0x1ff), 3);
        assert_eq!(get(&db, &manager, b"fldA", 0x200), 0);
        assert_eq!(get(&db, &manager, b"fldA", 0xff), 0);
        assert_eq!(get(&db, &manager, b"flag", 0x180), 1);
        let mem = VarnodeData { space: Some(space(&manager, "reg")), offset: 0, size: 4 };
        assert_eq!(db.get_tracked_value(&mem, &addr(&manager, "ram", 0x150)), 0x55);
        assert_eq!(db.get_tracked_value(&mem, &addr(&manager, "ram", 0x250)), 0);

        // an unexpected child is a hard error, as in C++
        let mut db2 = test_db();
        let mut decoder = XmlDecode::new(&manager, &registry);
        decoder
            .ingest_stream(
                b"<context_data><context_pointset space=\"ram\" first=\"0\" last=\"1\"/></context_data>",
            )
            .unwrap();
        let err = db2.decode_from_spec(&mut decoder).unwrap_err();
        assert_eq!(err.to_string(), "Bad <context_data> tag");
    }

    /// The attribute-less <context_pointset> form restores the DEFAULT
    /// blob (zeroing it per <set> child, as the C++ does).
    #[test]
    fn test_globalcontext_decode_default_pointset() {
        let manager = test_manager();
        let registry = test_registry();
        let mut db = test_db();
        db.set_variable_default(b"fldB", 0x11).unwrap();
        let mut decoder = XmlDecode::new(&manager, &registry);
        decoder
            .ingest_stream(
                b"<context_points><context_pointset>\
                  <set name=\"fldA\" val=\"2\"/>\
                </context_pointset></context_points>",
            )
            .unwrap();
        db.decode(&mut decoder).unwrap();
        // default blob was zeroed wholesale, then fldA painted
        assert_eq!(db.get_default_value_by_name(b"fldA").unwrap(), 2);
        assert_eq!(db.get_default_value_by_name(b"fldB").unwrap(), 0);
        assert_eq!(get(&db, &manager, b"fldA", 0x1234), 2);
    }
}
