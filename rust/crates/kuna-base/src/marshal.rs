//! Port of `decompiler/cpp/marshal.hh` + `marshal.cc` (W1, item
//! `w1-base-marshal`) — the structured-data Encoder/Decoder layer.
//!
//! Three pieces:
//!
//! - [`AttributeId`] / [`ElementId`]: annotations parallel to XML attributes
//!   and elements.  In C++ these are static objects whose constructors run
//!   before `main` and register themselves in a global list, which
//!   `AttributeId::initialize()` later folds into a name -> id hashtable.
//!   Per the porting rules (NO global ctors, deterministic registry) the
//!   Rust ids are `const` values and the name -> id table is an explicit
//!   [`IdRegistry`] instance: [`IdRegistry::with_base_ids`] registers every
//!   id defined in this crate, and later crates/waves extend the registry
//!   explicitly with their own tables (kuna's 4000+ id range included).
//! - [`Decoder`] / [`Encoder`] traits mirroring the C++ abstract classes.
//!   C++ overloads `read*(void)` / `read*(AttributeId)` become `read_*()` /
//!   `read_*_id()`.  Errors (C++ `DecoderError`) surface as
//!   `Err(KunaError::Decoder)`.
//! - [`XmlEncode`]/[`XmlDecode`] over [`crate::xml`], and
//!   [`PackedEncode`]/[`PackedDecode`], the byte-packed protocol described
//!   by [`packed_format`].  The packed protocol is **bit-exact** with the
//!   C++ implementation (`.sla` files and Ghidra save files depend on byte
//!   identity).
//!
//! Deliberate API notes (vs the C++ headers):
//!
//! - `Decoder::readOpcode` / `Encoder::writeOpcode` are **not** in the
//!   traits: `OpCode` and its name tables are ported in `kuna-num`
//!   (`w1-num-pcode-semantics`), which depends on this crate.  The opcode
//!   read/write protocol (XML: opcode name string; packed: positive signed
//!   integer) is layered on top of `read_string`/`read_signed_integer` by
//!   that wave.
//! - Strings are byte strings (`&[u8]` / `Vec<u8>`), following the
//!   `crate::xml` decision: C++ `std::string` attribute payloads need not
//!   be valid UTF-8 and must round-trip byte-for-byte.
//! - Streams are in-memory: encoders write to `&mut Vec<u8>` (the C++
//!   `ostream&`), `ingest_stream` takes `&[u8]` (the C++ `istream&`).
//! - The C++ `CPUI_DEBUG` blocks in `XmlDecode::closeElement` /
//!   `closeElementSkipping` are compiled **out** of the test oracle
//!   (`decomp_test_dbg` builds without `CPUI_DEBUG`); the port follows the
//!   test-oracle semantics and the blocks are noted in comments.
//! - `Decoder` in C++ stores a possibly-null `AddrSpaceManager*`; the Rust
//!   decoders hold a `&AddrSpaceManager` (non-null).  Contexts that pass
//!   null in C++ (parts of the SLEIGH compiler) arrive with the sleigh wave
//!   and can construct an empty manager.

use std::collections::BTreeMap;
use std::rc::Rc;

use crate::error::{KunaError, KunaResult};
use crate::space::{spacetype, AddrSpace, AddrSpaceManager};
use crate::types::Wrap;
use crate::xml::{self, Document, Element};

// ---------------------------------------------------------------------------
// AttributeId / ElementId
// ---------------------------------------------------------------------------

/// \brief An annotation for a data element to being transferred to/from a stream
///
/// This class parallels the XML concept of an \b attribute on an element. An
/// AttributeId describes a particular piece of data associated with an
/// ElementId.  The defining characteristic of the AttributeId is its name.
/// Internally this name is associated with an integer id.  The name (and id)
/// uniquely determine the data being labeled, within the context of a
/// specific ElementId.  Within this context, an AttributeId labels either
///   - An unsigned integer
///   - A signed integer
///   - A boolean value
///   - A string
///
/// The same AttributeId can be used to label a different type of data when
/// associated with a different ElementId.
#[derive(Debug, Clone, Copy)]
pub struct AttributeId {
    /// The name of the attribute
    name: &'static str,
    /// The (internal) id of the attribute
    id: u32,
}

impl AttributeId {
    /// Construct given a name and id (C++ static-object constructor; the
    /// global registration side effect is replaced by [`IdRegistry`]).
    pub const fn new(nm: &'static str, i: u32) -> Self {
        AttributeId { name: nm, id: i }
    }

    /// Get the attribute's name
    pub const fn get_name(&self) -> &'static str {
        self.name
    }

    /// Get the attribute's id
    pub const fn get_id(&self) -> u32 {
        self.id
    }
}

impl PartialEq for AttributeId {
    /// Test equality with another AttributeId (id comparison, as in C++)
    fn eq(&self, op2: &AttributeId) -> bool {
        self.id == op2.id
    }
}
impl Eq for AttributeId {}

/// Test equality of a raw integer id with an AttributeId
impl PartialEq<AttributeId> for u32 {
    fn eq(&self, op2: &AttributeId) -> bool {
        *self == op2.id
    }
}
/// Test equality of an AttributeId with a raw integer id
impl PartialEq<u32> for AttributeId {
    fn eq(&self, id: &u32) -> bool {
        self.id == *id
    }
}

/// \brief An annotation for a specific collection of hierarchical data
///
/// This class parallels the XML concept of an \b element.  An ElementId
/// describes a collection of data, where each piece is annotated by a
/// specific AttributeId.  In addition, each ElementId can contain zero or
/// more \e child ElementId objects, forming a hierarchy of annotated data.
/// Each ElementId has a name, which is unique at least within the context of
/// its parent ElementId. Internally this name is associated with an integer
/// id. A special AttributeId ATTRIB_CONTENT is used to label the XML
/// element's text content, which is traditionally not labeled as an
/// attribute.
#[derive(Debug, Clone, Copy)]
pub struct ElementId {
    /// The name of the element
    name: &'static str,
    /// The (internal) id of the element
    id: u32,
}

impl ElementId {
    /// Construct given a name and id
    pub const fn new(nm: &'static str, i: u32) -> Self {
        ElementId { name: nm, id: i }
    }

    /// Get the element's name
    pub const fn get_name(&self) -> &'static str {
        self.name
    }

    /// Get the element's id
    pub const fn get_id(&self) -> u32 {
        self.id
    }
}

impl PartialEq for ElementId {
    /// Test equality with another ElementId (id comparison, as in C++)
    fn eq(&self, op2: &ElementId) -> bool {
        self.id == op2.id
    }
}
impl Eq for ElementId {}

/// Test equality of a raw integer id with an ElementId
impl PartialEq<ElementId> for u32 {
    fn eq(&self, op2: &ElementId) -> bool {
        *self == op2.id
    }
}
/// Test equality of an ElementId with a raw integer id
impl PartialEq<u32> for ElementId {
    fn eq(&self, id: &u32) -> bool {
        self.id == *id
    }
}

// ---------------------------------------------------------------------------
// IdRegistry — the explicit, deterministic replacement for the C++ global
// static-initialization lists + AttributeId::initialize()/ElementId::initialize()
// ---------------------------------------------------------------------------

/// Explicit name -> id lookup tables for attributes and elements.
///
/// C++ builds these maps from global static objects at startup
/// (`AttributeId::initialize`, `ElementId::initialize`); the Rust port has
/// no global constructors, so the registry is an explicit value constructed
/// deterministically from registration calls.  Like the C++ `initialize()`
/// (release build), a duplicate name registration silently overwrites.
///
/// Only scope-0 ids are registered (the C++ constructors skip registration
/// for `scope != 0`, and `find` only supports reverse lookup for scope 0).
#[derive(Debug, Default, Clone)]
pub struct IdRegistry {
    /// A map of AttributeId names to their associated id
    lookup_attribute_id: BTreeMap<&'static str, u32>,
    /// A map of ElementId names to their associated id
    lookup_element_id: BTreeMap<&'static str, u32>,
}

impl IdRegistry {
    /// Construct an empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Register an attribute for name lookup (C++ static registration +
    /// `AttributeId::initialize`).
    pub fn register_attribute(&mut self, attrib: &AttributeId) {
        self.lookup_attribute_id.insert(attrib.name, attrib.id);
    }

    /// Register an element for name lookup (C++ static registration +
    /// `ElementId::initialize`).
    pub fn register_element(&mut self, elem: &ElementId) {
        self.lookup_element_id.insert(elem.name, elem.id);
    }

    /// Find the id associated with a specific attribute name
    /// (C++ `AttributeId::find`).
    ///
    /// The name is looked up in the scoped list of attributes.  If the
    /// attribute is not in the list, a special placeholder attribute,
    /// ATTRIB_UNKNOWN, is returned as a placeholder for attributes with
    /// unrecognized names.
    pub fn find_attribute(&self, nm: &str, scope: i32) -> u32 {
        if scope == 0 {
            // Currently only support reverse look up for scope 0
            if let Some(&id) = self.lookup_attribute_id.get(nm) {
                return id;
            }
        }
        ATTRIB_UNKNOWN.id
    }

    /// Find the id associated with a specific element name
    /// (C++ `ElementId::find`).
    pub fn find_element(&self, nm: &str, scope: i32) -> u32 {
        if scope == 0 {
            if let Some(&id) = self.lookup_element_id.get(nm) {
                return id;
            }
        }
        ELEM_UNKNOWN.id
    }

    /// Build a registry holding every AttributeId/ElementId defined in
    /// kuna-base (`marshal.cc`, `space.cc`, `address.cc` tables plus the
    /// one `translate.cc` element needed by `OverlaySpace::decode`).
    pub fn with_base_ids() -> Self {
        let mut reg = Self::new();
        for a in BASE_ATTRIBUTE_IDS {
            reg.register_attribute(a);
        }
        for e in BASE_ELEMENT_IDS {
            reg.register_element(e);
        }
        reg
    }
}

// Common attributes.  Attributes with multiple uses (marshal.cc)
pub const ATTRIB_CONTENT: AttributeId = AttributeId::new("XMLcontent", 1);
pub const ATTRIB_ALIGN: AttributeId = AttributeId::new("align", 2);
pub const ATTRIB_BIGENDIAN: AttributeId = AttributeId::new("bigendian", 3);
pub const ATTRIB_CONSTRUCTOR: AttributeId = AttributeId::new("constructor", 4);
pub const ATTRIB_DESTRUCTOR: AttributeId = AttributeId::new("destructor", 5);
pub const ATTRIB_EXTRAPOP: AttributeId = AttributeId::new("extrapop", 6);
pub const ATTRIB_FORMAT: AttributeId = AttributeId::new("format", 7);
pub const ATTRIB_HIDDENRETPARM: AttributeId = AttributeId::new("hiddenretparm", 8);
pub const ATTRIB_ID: AttributeId = AttributeId::new("id", 9);
pub const ATTRIB_INDEX: AttributeId = AttributeId::new("index", 10);
pub const ATTRIB_INDIRECTSTORAGE: AttributeId = AttributeId::new("indirectstorage", 11);
pub const ATTRIB_METATYPE: AttributeId = AttributeId::new("metatype", 12);
pub const ATTRIB_MODEL: AttributeId = AttributeId::new("model", 13);
pub const ATTRIB_NAME: AttributeId = AttributeId::new("name", 14);
pub const ATTRIB_NAMELOCK: AttributeId = AttributeId::new("namelock", 15);
pub const ATTRIB_OFFSET: AttributeId = AttributeId::new("offset", 16);
pub const ATTRIB_READONLY: AttributeId = AttributeId::new("readonly", 17);
pub const ATTRIB_REF: AttributeId = AttributeId::new("ref", 18);
pub const ATTRIB_SIZE: AttributeId = AttributeId::new("size", 19);
pub const ATTRIB_SPACE: AttributeId = AttributeId::new("space", 20);
pub const ATTRIB_THISPTR: AttributeId = AttributeId::new("thisptr", 21);
pub const ATTRIB_TYPE: AttributeId = AttributeId::new("type", 22);
pub const ATTRIB_TYPELOCK: AttributeId = AttributeId::new("typelock", 23);
pub const ATTRIB_VAL: AttributeId = AttributeId::new("val", 24);
pub const ATTRIB_VALUE: AttributeId = AttributeId::new("value", 25);
pub const ATTRIB_WORDSIZE: AttributeId = AttributeId::new("wordsize", 26);
pub const ATTRIB_STORAGE: AttributeId = AttributeId::new("storage", 149);
pub const ATTRIB_STACKSPILL: AttributeId = AttributeId::new("stackspill", 150);

/// Special attribute to represent an attribute with an unrecognized name.
/// (Number serves as next open index.)
pub const ATTRIB_UNKNOWN: AttributeId = AttributeId::new("XMLunknown", 159);

pub const ELEM_DATA: ElementId = ElementId::new("data", 1);
pub const ELEM_INPUT: ElementId = ElementId::new("input", 2);
pub const ELEM_OFF: ElementId = ElementId::new("off", 3);
pub const ELEM_OUTPUT: ElementId = ElementId::new("output", 4);
pub const ELEM_RETURNADDRESS: ElementId = ElementId::new("returnaddress", 5);
pub const ELEM_SYMBOL: ElementId = ElementId::new("symbol", 6);
pub const ELEM_TARGET: ElementId = ElementId::new("target", 7);
pub const ELEM_VAL: ElementId = ElementId::new("val", 8);
pub const ELEM_VALUE: ElementId = ElementId::new("value", 9);
pub const ELEM_VOID: ElementId = ElementId::new("void", 10);

/// Special element to represent an element with an unrecognized name.
/// (Number serves as next open index.)
pub const ELEM_UNKNOWN: ElementId = ElementId::new("XMLunknown", 290);

/// Every kuna-base AttributeId, for [`IdRegistry::with_base_ids`].
static BASE_ATTRIBUTE_IDS: &[&AttributeId] = &[
    // marshal.cc
    &ATTRIB_CONTENT,
    &ATTRIB_ALIGN,
    &ATTRIB_BIGENDIAN,
    &ATTRIB_CONSTRUCTOR,
    &ATTRIB_DESTRUCTOR,
    &ATTRIB_EXTRAPOP,
    &ATTRIB_FORMAT,
    &ATTRIB_HIDDENRETPARM,
    &ATTRIB_ID,
    &ATTRIB_INDEX,
    &ATTRIB_INDIRECTSTORAGE,
    &ATTRIB_METATYPE,
    &ATTRIB_MODEL,
    &ATTRIB_NAME,
    &ATTRIB_NAMELOCK,
    &ATTRIB_OFFSET,
    &ATTRIB_READONLY,
    &ATTRIB_REF,
    &ATTRIB_SIZE,
    &ATTRIB_SPACE,
    &ATTRIB_THISPTR,
    &ATTRIB_TYPE,
    &ATTRIB_TYPELOCK,
    &ATTRIB_VAL,
    &ATTRIB_VALUE,
    &ATTRIB_WORDSIZE,
    &ATTRIB_STORAGE,
    &ATTRIB_STACKSPILL,
    &ATTRIB_UNKNOWN,
    // space.cc
    &crate::space::ATTRIB_BASE,
    &crate::space::ATTRIB_DEADCODEDELAY,
    &crate::space::ATTRIB_DELAY,
    &crate::space::ATTRIB_LOGICALSIZE,
    &crate::space::ATTRIB_PHYSICAL,
    &crate::space::ATTRIB_PIECE,
    // address.cc
    &crate::address::ATTRIB_FIRST,
    &crate::address::ATTRIB_LAST,
    &crate::address::ATTRIB_UNIQ,
];

/// Every kuna-base ElementId, for [`IdRegistry::with_base_ids`].
static BASE_ELEMENT_IDS: &[&ElementId] = &[
    // marshal.cc
    &ELEM_DATA,
    &ELEM_INPUT,
    &ELEM_OFF,
    &ELEM_OUTPUT,
    &ELEM_RETURNADDRESS,
    &ELEM_SYMBOL,
    &ELEM_TARGET,
    &ELEM_VAL,
    &ELEM_VALUE,
    &ELEM_VOID,
    &ELEM_UNKNOWN,
    // address.cc
    &crate::address::ELEM_ADDR,
    &crate::address::ELEM_RANGE,
    &crate::address::ELEM_RANGELIST,
    &crate::address::ELEM_REGISTER,
    &crate::address::ELEM_SEQNUM,
    &crate::address::ELEM_VARNODE,
    // translate.cc (only the element needed by OverlaySpace::decode; the
    // full translate id table arrives with the sleigh wave)
    &crate::space::ELEM_SPACE_OVERLAY,
];

// ---------------------------------------------------------------------------
// C++ istream-style integer parsing helpers (XmlDecode read* methods)
// ---------------------------------------------------------------------------

/// Core of `strtoull`-style parsing with C++ base-0 detection, as performed
/// by `istringstream >> val` after `unsetf(ios::dec|ios::hex|ios::oct)`.
///
/// Returns `(magnitude, end_index, negative, overflow)`:
/// - leading whitespace is skipped, then an optional `+`/`-` sign;
/// - `0x`/`0X` selects base 16, else a leading `0` selects base 8, else 10;
/// - `end_index` is the index just past the last consumed digit (the C++
///   `endptr`); if no digits are consumed the magnitude is 0 (a failed C++
///   extraction stores 0 since C++11) and `end_index` is 0;
/// - on overflow of 64 bits the magnitude saturates and `overflow` is set
///   (strtoull returns ULLONG_MAX with ERANGE).
fn cxx_strtoull_core(bytes: &[u8]) -> (u64, usize, bool, bool) {
    let mut i = 0usize;
    // isspace() in the "C" locale
    while i < bytes.len() && matches!(bytes[i], b' ' | b'\t' | b'\n' | b'\x0b' | b'\x0c' | b'\r') {
        i += 1;
    }
    let mut neg = false;
    if i < bytes.len() && (bytes[i] == b'+' || bytes[i] == b'-') {
        neg = bytes[i] == b'-';
        i += 1;
    }
    let mut base: u64 = 10;
    if i + 1 < bytes.len() && bytes[i] == b'0' && (bytes[i + 1] == b'x' || bytes[i + 1] == b'X') {
        // "0x" with no following hex digit: the subject sequence is just the
        // "0" (strtoul leaves endptr at the 'x').
        if i + 2 < bytes.len() && bytes[i + 2].is_ascii_hexdigit() {
            base = 16;
            i += 2;
        } else {
            return (0, i + 1, neg, false);
        }
    } else if i < bytes.len() && bytes[i] == b'0' {
        base = 8;
    }
    let mut val: u64 = 0;
    let mut overflow = false;
    let mut any = false;
    while i < bytes.len() {
        let c = bytes[i];
        let d: u64 = match c {
            b'0'..=b'9' => (c - b'0') as u64,
            b'a'..=b'f' => (c - b'a') as u64 + 10,
            b'A'..=b'F' => (c - b'A') as u64 + 10,
            _ => break,
        };
        if d >= base {
            break;
        }
        any = true;
        let (m, o1) = val.overflowing_mul(base);
        let (s, o2) = m.overflowing_add(d);
        if o1 || o2 {
            overflow = true;
        }
        val = s;
        i += 1;
    }
    if !any {
        // No conversion: value 0 (and strtoul-style endptr == nptr)
        return (0, 0, neg, false);
    }
    (val, i, neg, overflow)
}

/// `istringstream >> uintb` with `unsetf(ios::dec|ios::hex|ios::oct)`:
/// strtoull semantics — overflow saturates to `u64::MAX`, a minus sign
/// (wrapping-)negates the magnitude.
pub(crate) fn cxx_parse_unsigned(bytes: &[u8]) -> u64 {
    let (val, _, neg, overflow) = cxx_strtoull_core(bytes);
    if overflow {
        return u64::MAX;
    }
    if neg {
        val.wrapping_neg()
    } else {
        val
    }
}

/// `istringstream >> intb` with `unsetf(ios::dec|ios::hex|ios::oct)`:
/// strtoll semantics — out-of-range values clamp to `i64::MAX`/`i64::MIN`.
pub(crate) fn cxx_parse_signed(bytes: &[u8]) -> i64 {
    let (val, _, neg, overflow) = cxx_strtoull_core(bytes);
    if neg {
        if overflow || val > (i64::MAX as u64) + 1 {
            i64::MIN
        } else {
            // magnitude <= 2^63 here; wrapping negate handles exactly 2^63
            val.wrapping_neg() as i64
        }
    } else if overflow || val > i64::MAX as u64 {
        i64::MAX
    } else {
        val as i64
    }
}

/// `istringstream >> dec >> val` into a `uint4` (libstdc++ `num_get`,
/// C++11): decimal digits ONLY — `dec` fixes the base, so there is no
/// `0x`/leading-`0` base detection — after optional leading whitespace (the
/// `operator>>` sentry; `skipws` is on by default) and an optional single
/// `+`/`-` sign.  `num_get` accumulates in the unsigned destination width:
/// on overflow of `uint4` it stores `UINT_MAX` (the most positive value,
/// since the destination is unsigned, even for a negative field), a minus
/// sign negates the magnitude modularly, and a failed extraction (no
/// digits) stores 0 (C++11).
fn cxx_num_get_u32_dec(bytes: &[u8]) -> u32 {
    let mut i = 0usize;
    // isspace() in the "C" locale
    while i < bytes.len() && matches!(bytes[i], b' ' | b'\t' | b'\n' | b'\x0b' | b'\x0c' | b'\r') {
        i += 1;
    }
    let mut neg = false;
    if i < bytes.len() && (bytes[i] == b'+' || bytes[i] == b'-') {
        neg = bytes[i] == b'-';
        i += 1;
    }
    let mut val: u32 = 0;
    let mut any = false;
    let mut overflow = false;
    while i < bytes.len() {
        let c = bytes[i];
        if !c.is_ascii_digit() {
            break;
        }
        any = true;
        // _M_extract_int checks against numeric_limits<uint4>::max() before
        // each accumulation step; digits keep being consumed after overflow.
        let (m, o1) = val.overflowing_mul(10);
        let (s, o2) = m.overflowing_add((c - b'0') as u32);
        if o1 || o2 {
            overflow = true;
        }
        val = s;
        i += 1;
    }
    if !any {
        return 0; // Failed extraction stores 0 (C++11 num_get)
    }
    if overflow {
        return u32::MAX; // Unsigned destination: most-positive value stored
    }
    if neg {
        val.wrapping_neg() // Modular negation of the magnitude
    } else {
        val
    }
}

/// `strtoul(s, &endptr, 0)` returning the value and the consumed byte count
/// (the `endptr` offset).  Used by `AddrSpace::read` and `get_offset_size`.
pub(crate) fn cxx_strtoul(bytes: &[u8]) -> (u64, usize) {
    let (val, end, neg, overflow) = cxx_strtoull_core(bytes);
    let v = if overflow {
        u64::MAX
    } else if neg {
        val.wrapping_neg()
    } else {
        val
    };
    (v, end)
}

// ---------------------------------------------------------------------------
// Decoder / Encoder traits
// ---------------------------------------------------------------------------

/// \brief A class for reading structured data from a stream
///
/// All data is loosely structured as with an XML document.  A document
/// contains a nested set of \b elements, with labels corresponding to the
/// ElementId class. A single element can hold zero or more attributes and
/// zero or more child elements.  An attribute holds a primitive data element
/// (bool, integer, string) and is labeled by an AttributeId. The document is
/// traversed using a sequence of openElement() and closeElement() calls,
/// intermixed with read*() calls to extract the data. The elements are
/// traversed in a depth first order.  Attributes within an element can be
/// traversed in order using repeated calls to the getNextAttributeId()
/// method, followed by a calls to one of the read*() methods to extract the
/// data.  Alternately a read*_id(AttributeId) call can be used to extract
/// data for an attribute known to be in the element.  There is a special
/// content attribute whose data can be extracted using a read*_id call that
/// is passed the special ATTRIB_CONTENT id.  This attribute will not be
/// traversed by getNextAttributeId().
pub trait Decoder {
    /// Get the manager used for address space decoding
    fn get_addr_space_manager(&self) -> &AddrSpaceManager;

    /// \brief Prepare to decode a given stream
    ///
    /// Called once before any decoding.  Currently this is assumed to make
    /// an internal copy of the stream data, i.e. the input stream is cleared
    /// before any decoding takes place.
    fn ingest_stream(&mut self, s: &[u8]) -> KunaResult<()>;

    /// \brief Peek at the next child element of the current parent, without
    /// traversing in (opening) it.
    ///
    /// The element id is returned, which can be compared to ElementId
    /// labels.  If there are no remaining child elements to traverse, 0 is
    /// returned.
    fn peek_element(&mut self) -> KunaResult<u32>;

    /// \brief Open (traverse into) the next child element of the current parent.
    ///
    /// The child becomes the current parent.  The list of attributes is
    /// initialized for use with getNextAttributeId.  Returns 0 if there is
    /// no child to open.
    fn open_element(&mut self) -> KunaResult<u32>;

    /// \brief Open (traverse into) the next child element, which must be of
    /// a specific type (C++ `openElement(const ElementId &)`)
    ///
    /// The child becomes the current parent, and its attributes are
    /// initialized for use with getNextAttributeId.  The child must match
    /// the given element id or an error is returned.
    fn open_element_id(&mut self, elem_id: &ElementId) -> KunaResult<u32>;

    /// \brief Close the current element
    ///
    /// The data for the current element is considered fully processed.  If
    /// the element has additional children, an error is returned.  The
    /// stream must indicate the end of the element in some way.
    fn close_element(&mut self, id: u32) -> KunaResult<()>;

    /// \brief Close the current element, skipping any child elements that
    /// have not yet been parsed
    fn close_element_skipping(&mut self, id: u32) -> KunaResult<()>;

    /// \brief Get the next attribute id for the current element
    ///
    /// Attributes are automatically set up for traversal using this method,
    /// when the element is opened.  If all attributes have been traversed
    /// (or there are no attributes), 0 is returned.
    fn get_next_attribute_id(&mut self) -> KunaResult<u32>;

    /// \brief Get the id for the (current) attribute, assuming it is indexed
    ///
    /// Assuming the previous call to getNextAttributeId() returned the id of
    /// ATTRIB_UNKNOWN, reinterpret the attribute as being an indexed form of
    /// the given attribute. If the attribute matches, return this indexed
    /// id, otherwise return ATTRIB_UNKNOWN.
    fn get_indexed_attribute_id(&mut self, attrib_id: &AttributeId) -> KunaResult<u32>;

    /// \brief Reset attribute traversal for the current element
    ///
    /// Attributes for a single element can be traversed more than once using
    /// the getNextAttributeId method.
    fn rewind_attributes(&mut self);

    /// \brief Parse the current attribute as a boolean value
    fn read_bool(&mut self) -> KunaResult<bool>;

    /// \brief Find and parse a specific attribute in the current element as
    /// a boolean value (C++ `readBool(const AttributeId &)`)
    ///
    /// Parsing via getNextAttributeId is reset.
    fn read_bool_id(&mut self, attrib_id: &AttributeId) -> KunaResult<bool>;

    /// \brief Parse the current attribute as a signed integer value
    fn read_signed_integer(&mut self) -> KunaResult<i64>;

    /// \brief Find and parse a specific attribute in the current element as
    /// a signed integer (C++ `readSignedInteger(const AttributeId &)`)
    fn read_signed_integer_id(&mut self, attrib_id: &AttributeId) -> KunaResult<i64>;

    /// \brief Parse the current attribute as either a signed integer value
    /// or a string.
    ///
    /// If the attribute is an integer, its value is returned. If the
    /// attribute is a string, it must match an expected string passed to the
    /// method, and a predetermined integer value associated with the string
    /// is returned.
    fn read_signed_integer_expect_string(
        &mut self,
        expect: &[u8],
        expectval: i64,
    ) -> KunaResult<i64>;

    /// \brief Find and parse a specific attribute in the current element as
    /// either a signed integer or a string.
    fn read_signed_integer_expect_string_id(
        &mut self,
        attrib_id: &AttributeId,
        expect: &[u8],
        expectval: i64,
    ) -> KunaResult<i64>;

    /// \brief Parse the current attribute as an unsigned integer value
    fn read_unsigned_integer(&mut self) -> KunaResult<u64>;

    /// \brief Find and parse a specific attribute in the current element as
    /// an unsigned integer (C++ `readUnsignedInteger(const AttributeId &)`)
    fn read_unsigned_integer_id(&mut self, attrib_id: &AttributeId) -> KunaResult<u64>;

    /// \brief Parse the current attribute as a string (byte string)
    fn read_string(&mut self) -> KunaResult<Vec<u8>>;

    /// \brief Find the specific attribute in the current element and return
    /// it as a string (C++ `readString(const AttributeId &)`)
    fn read_string_id(&mut self, attrib_id: &AttributeId) -> KunaResult<Vec<u8>>;

    /// \brief Parse the current attribute as an address space
    fn read_space(&mut self) -> KunaResult<Rc<AddrSpace>>;

    /// \brief Find the specific attribute in the current element and return
    /// it as an address space (C++ `readSpace(const AttributeId &)`)
    fn read_space_id(&mut self, attrib_id: &AttributeId) -> KunaResult<Rc<AddrSpace>>;

    // NOTE: C++ `readOpcode()` / `readOpcode(AttributeId&)` are layered on
    // by the kuna-num opcode wave (see module docs).

    /// \brief Skip parsing of the next element
    ///
    /// The element skipped is the one that would be opened by the next call
    /// to openElement.
    fn skip_element(&mut self) -> KunaResult<()> {
        let elem_id = self.open_element()?;
        self.close_element_skipping(elem_id)
    }
}

/// \brief A class for writing structured data to a stream
///
/// The resulting encoded data is structured similarly to an XML document.
/// The document contains a nested set of \b elements, with labels
/// corresponding to the ElementId class. A single element can hold zero or
/// more attributes and zero or more child elements.  An \b attribute holds a
/// primitive data element (bool, integer, string) and is labeled by an
/// AttributeId. The document is written using a sequence of openElement()
/// and closeElement() calls, intermixed with write*() calls to encode the
/// data primitives.  All primitives written using a write*() call are
/// associated with current open element, and all write*() calls for one
/// element must come before opening any child element.  The traditional XML
/// element text content can be written using the special ATTRIB_CONTENT
/// AttributeId, which must be the last write*() call associated with the
/// specific element.
pub trait Encoder {
    /// \brief Begin a new element in the encoding
    ///
    /// The element will have the given ElementId annotation and becomes the
    /// \e current element.
    fn open_element(&mut self, elem_id: &ElementId);

    /// \brief End the current element in the encoding
    ///
    /// The current element must match the given annotation or the encoding
    /// is corrupted.
    fn close_element(&mut self, elem_id: &ElementId);

    /// \brief Write an annotated boolean value into the encoding
    fn write_bool(&mut self, attrib_id: &AttributeId, val: bool);

    /// \brief Write an annotated signed integer value into the encoding
    fn write_signed_integer(&mut self, attrib_id: &AttributeId, val: i64);

    /// \brief Write an annotated unsigned integer value into the encoding
    fn write_unsigned_integer(&mut self, attrib_id: &AttributeId, val: u64);

    /// \brief Write an annotated string (byte string) into the encoding
    fn write_string(&mut self, attrib_id: &AttributeId, val: &[u8]);

    /// \brief Write an annotated string, using an indexed attribute, into
    /// the encoding
    ///
    /// Multiple attributes with a shared name can be written to the same
    /// element by calling this method multiple times with a different
    /// \b index value. The encoding will use attribute ids up to the base id
    /// plus the maximum index passed in.  Implementors must be careful to
    /// not use other attributes with ids bigger than the base id within the
    /// element taking the indexed attribute.
    fn write_string_indexed(&mut self, attrib_id: &AttributeId, index: u32, val: &[u8]);

    /// \brief Write an address space reference into the encoding
    fn write_space(&mut self, attrib_id: &AttributeId, spc: &AddrSpace);

    // NOTE: C++ `writeOpcode` is layered on by the kuna-num opcode wave
    // (see module docs).
}

// ---------------------------------------------------------------------------
// XmlEncode
// ---------------------------------------------------------------------------

/// Stage of writing an element tag (XmlEncode)
mod xml_tag_status {
    /// Tag has been opened, attributes can be written
    pub const TAG_START: i32 = 0;
    /// Opening tag and content have been written
    pub const TAG_CONTENT: i32 = 1;
    /// No tag is currently being written
    pub const TAG_STOP: i32 = 2;
}

/// Array of '\n' + ' ' characters for emitting indents (C++ `XmlEncode::spaces`)
const XML_ENCODE_SPACES: &[u8] = b"\n                        ";
/// Maximum number of leading spaces when indenting XML (C++ `MAX_SPACES`, 24+1)
const XML_ENCODE_MAX_SPACES: i32 = 24 + 1;

/// \brief An XML based encoder
///
/// The underlying transfer encoding is an XML document.  The encoder is
/// initialized with a stream (an in-memory byte buffer here) which will
/// receive the XML document as calls are made on the encoder.
pub struct XmlEncode<'a> {
    /// The stream receiving the encoded data
    out_stream: &'a mut Vec<u8>,
    /// Stage of writing an element tag
    tag_status: i32,
    /// Depth of open elements
    depth: i32,
    /// `true` if encoder should indent and emit newlines
    do_formatting: bool,
}

impl<'a> XmlEncode<'a> {
    /// Construct from a stream (C++ default `doFormat=true`)
    pub fn new(s: &'a mut Vec<u8>) -> Self {
        Self::new_with_format(s, true)
    }

    /// Construct from a stream with explicit formatting control
    pub fn new_with_format(s: &'a mut Vec<u8>, do_format: bool) -> Self {
        XmlEncode {
            out_stream: s,
            tag_status: xml_tag_status::TAG_STOP,
            depth: 0,
            do_formatting: do_format,
        }
    }

    /// Emit a newline and proper indenting for the next tag
    fn new_line(&mut self) {
        if !self.do_formatting {
            return;
        }
        let mut num_spaces = self.depth * 2 + 1;
        if num_spaces > XML_ENCODE_MAX_SPACES {
            num_spaces = XML_ENCODE_MAX_SPACES;
        }
        self.out_stream.extend_from_slice(&XML_ENCODE_SPACES[..num_spaces as usize]);
    }
}

impl Encoder for XmlEncode<'_> {
    fn open_element(&mut self, elem_id: &ElementId) {
        if self.tag_status == xml_tag_status::TAG_START {
            self.out_stream.push(b'>');
        } else {
            self.tag_status = xml_tag_status::TAG_START;
        }
        self.new_line();
        self.out_stream.push(b'<');
        self.out_stream.extend_from_slice(elem_id.get_name().as_bytes());
        self.depth += 1;
    }

    fn close_element(&mut self, elem_id: &ElementId) {
        self.depth -= 1;
        if self.tag_status == xml_tag_status::TAG_START {
            self.out_stream.extend_from_slice(b"/>");
            self.tag_status = xml_tag_status::TAG_STOP;
            return;
        }
        if self.tag_status != xml_tag_status::TAG_CONTENT {
            self.new_line();
        } else {
            self.tag_status = xml_tag_status::TAG_STOP;
        }
        self.out_stream.extend_from_slice(b"</");
        self.out_stream.extend_from_slice(elem_id.get_name().as_bytes());
        self.out_stream.push(b'>');
    }

    fn write_bool(&mut self, attrib_id: &AttributeId, val: bool) {
        if *attrib_id == ATTRIB_CONTENT {
            // Special id indicating, text value
            if self.tag_status == xml_tag_status::TAG_START {
                self.out_stream.push(b'>');
            }
            if val {
                self.out_stream.extend_from_slice(b"true");
            } else {
                self.out_stream.extend_from_slice(b"false");
            }
            self.tag_status = xml_tag_status::TAG_CONTENT;
            return;
        }
        xml::a_v_b(self.out_stream, attrib_id.get_name(), val);
    }

    fn write_signed_integer(&mut self, attrib_id: &AttributeId, val: i64) {
        if *attrib_id == ATTRIB_CONTENT {
            // Special id indicating, text value
            if self.tag_status == xml_tag_status::TAG_START {
                self.out_stream.push(b'>');
            }
            self.out_stream.extend_from_slice(format!("{val}").as_bytes());
            self.tag_status = xml_tag_status::TAG_CONTENT;
            return;
        }
        xml::a_v_i(self.out_stream, attrib_id.get_name(), val);
    }

    fn write_unsigned_integer(&mut self, attrib_id: &AttributeId, val: u64) {
        if *attrib_id == ATTRIB_CONTENT {
            // Special id indicating, text value
            if self.tag_status == xml_tag_status::TAG_START {
                self.out_stream.push(b'>');
            }
            self.out_stream.extend_from_slice(format!("0x{val:x}").as_bytes());
            self.tag_status = xml_tag_status::TAG_CONTENT;
            return;
        }
        xml::a_v_u(self.out_stream, attrib_id.get_name(), val);
    }

    fn write_string(&mut self, attrib_id: &AttributeId, val: &[u8]) {
        if *attrib_id == ATTRIB_CONTENT {
            // Special id indicating, text value
            if self.tag_status == xml_tag_status::TAG_START {
                self.out_stream.push(b'>');
            }
            xml::xml_escape(self.out_stream, val);
            self.tag_status = xml_tag_status::TAG_CONTENT;
            return;
        }
        xml::a_v(self.out_stream, attrib_id.get_name(), val);
    }

    fn write_string_indexed(&mut self, attrib_id: &AttributeId, index: u32, val: &[u8]) {
        self.out_stream.push(b' ');
        self.out_stream.extend_from_slice(attrib_id.get_name().as_bytes());
        // C++ `<< dec << index + 1` is uint4 arithmetic: wraps at UINT_MAX
        self.out_stream.extend_from_slice(format!("{}", index.wadd(1)).as_bytes());
        self.out_stream.extend_from_slice(b"=\"");
        xml::xml_escape(self.out_stream, val);
        self.out_stream.push(b'"');
    }

    fn write_space(&mut self, attrib_id: &AttributeId, spc: &AddrSpace) {
        if *attrib_id == ATTRIB_CONTENT {
            // Special id indicating, text value
            if self.tag_status == xml_tag_status::TAG_START {
                self.out_stream.push(b'>');
            }
            xml::xml_escape(self.out_stream, spc.get_name().as_bytes());
            self.tag_status = xml_tag_status::TAG_CONTENT;
            return;
        }
        xml::a_v(self.out_stream, attrib_id.get_name(), spc.get_name().as_bytes());
    }
}

// ---------------------------------------------------------------------------
// XmlDecode
// ---------------------------------------------------------------------------

/// \brief An XML based decoder
///
/// The underlying transfer encoding is an XML document.  The decoder can
/// either be initialized with an existing Element as the root of the data to
/// transfer, or the ingestStream() method can be invoked to read the XML
/// document from an input stream, in which case the decoder manages the
/// Document object.
///
/// Name -> id lookup goes through an explicit [`IdRegistry`] (the C++
/// version uses the global tables built by static initialization).
pub struct XmlDecode<'a> {
    /// Manager for decoding address space attributes
    spc_manager: &'a AddrSpaceManager,
    /// Name -> id tables (C++ static `AttributeId::find` / `ElementId::find`)
    registry: &'a IdRegistry,
    /// An ingested XML document, owned by \b this decoder.  (The DOM is
    /// `Rc`-shared, so holding the Document is not strictly necessary for
    /// lifetime, but mirrors the C++ ownership.)
    document: Option<Document>,
    /// The root XML element to be decoded
    root_element: Option<Rc<Element>>,
    /// Stack of currently \e open elements
    el_stack: Vec<Rc<Element>>,
    /// Index of next child for each \e open element
    iter_stack: Vec<usize>,
    /// Position of \e current attribute to parse (in \e current element)
    attribute_index: i32,
    /// Scope of element/attribute tags to look up
    scope: i32,
}

impl<'a> XmlDecode<'a> {
    /// Constructor for use with ingest_stream (C++ default scope = 0)
    pub fn new(spc: &'a AddrSpaceManager, registry: &'a IdRegistry) -> Self {
        Self::new_with_scope(spc, registry, 0)
    }

    /// Constructor for use with ingest_stream, with explicit scope
    pub fn new_with_scope(spc: &'a AddrSpaceManager, registry: &'a IdRegistry, sc: i32) -> Self {
        XmlDecode {
            spc_manager: spc,
            registry,
            document: None,
            root_element: None,
            el_stack: Vec::new(),
            iter_stack: Vec::new(),
            attribute_index: -1,
            scope: sc,
        }
    }

    /// Constructor with preparsed root
    pub fn new_with_root(
        spc: &'a AddrSpaceManager,
        registry: &'a IdRegistry,
        root: &Rc<Element>,
        sc: i32,
    ) -> Self {
        XmlDecode {
            spc_manager: spc,
            registry,
            document: None,
            root_element: Some(Rc::clone(root)),
            el_stack: Vec::new(),
            iter_stack: Vec::new(),
            attribute_index: -1,
            scope: sc,
        }
    }

    /// Get pointer to underlying XML element object
    pub fn get_current_xml_element(&self) -> &Rc<Element> {
        self.el_stack.last().expect("XmlDecode: no current element")
    }

    /// The current open element.  Panics if there is none (C++ dereferences
    /// `elStack.back()` unconditionally — UB when empty; ADR 0004).
    fn current_element(&self) -> Rc<Element> {
        Rc::clone(self.el_stack.last().expect("XmlDecode: no current element"))
    }

    /// \brief Find the attribute index, within the given element, for the
    /// given name
    ///
    /// Run through the attributes of the element until we find the one
    /// matching the name, or return an error otherwise.
    fn find_matching_attribute(el: &Element, attrib_name: &str) -> KunaResult<i32> {
        for i in 0..el.get_num_attributes() {
            if el.get_attribute_name(i) == attrib_name {
                return Ok(i);
            }
        }
        Err(KunaError::decoder(format!("Attribute missing: {attrib_name}")))
    }
}

impl Decoder for XmlDecode<'_> {
    fn get_addr_space_manager(&self) -> &AddrSpaceManager {
        self.spc_manager
    }

    fn ingest_stream(&mut self, s: &[u8]) -> KunaResult<()> {
        let document = xml::xml_tree(s)?;
        self.root_element = Some(Rc::clone(document.get_root()));
        self.document = Some(document);
        Ok(())
    }

    fn peek_element(&mut self) -> KunaResult<u32> {
        let el: Rc<Element>;
        if self.el_stack.is_empty() {
            match &self.root_element {
                None => return Ok(0),
                Some(root) => el = Rc::clone(root),
            }
        } else {
            let cur = self.current_element();
            let iter = *self.iter_stack.last().expect("XmlDecode: iterator stack empty");
            if iter == cur.get_children().len() {
                return Ok(0);
            }
            el = Rc::clone(&cur.get_children()[iter]);
        }
        Ok(self.registry.find_element(el.get_name(), self.scope))
    }

    fn open_element(&mut self) -> KunaResult<u32> {
        let el: Rc<Element>;
        if self.el_stack.is_empty() {
            match self.root_element.take() {
                None => return Ok(0), // Document already traversed
                Some(root) => el = root, // Only open once
            }
        } else {
            let cur = self.current_element();
            let iter = *self.iter_stack.last().expect("XmlDecode: iterator stack empty");
            if iter == cur.get_children().len() {
                return Ok(0); // Element already fully traversed
            }
            el = Rc::clone(&cur.get_children()[iter]);
            *self.iter_stack.last_mut().expect("XmlDecode: iterator stack empty") = iter + 1;
        }
        let id = self.registry.find_element(el.get_name(), self.scope);
        self.el_stack.push(el);
        self.iter_stack.push(0);
        self.attribute_index = -1;
        Ok(id)
    }

    fn open_element_id(&mut self, elem_id: &ElementId) -> KunaResult<u32> {
        let el: Rc<Element>;
        if self.el_stack.is_empty() {
            match self.root_element.take() {
                None => {
                    return Err(KunaError::decoder(format!(
                        "Expecting <{}> but reached end of document",
                        elem_id.get_name()
                    )))
                }
                Some(root) => el = root, // Only open document once
            }
        } else {
            let cur = self.current_element();
            let iter = *self.iter_stack.last().expect("XmlDecode: iterator stack empty");
            if iter != cur.get_children().len() {
                el = Rc::clone(&cur.get_children()[iter]);
                *self.iter_stack.last_mut().expect("XmlDecode: iterator stack empty") = iter + 1;
            } else {
                return Err(KunaError::decoder(format!(
                    "Expecting <{}> but no remaining children in current element",
                    elem_id.get_name()
                )));
            }
        }
        if el.get_name() != elem_id.get_name() {
            return Err(KunaError::decoder(format!(
                "Expecting <{}> but got <{}>",
                elem_id.get_name(),
                el.get_name()
            )));
        }
        self.el_stack.push(el);
        self.iter_stack.push(0);
        self.attribute_index = -1;
        Ok(elem_id.get_id())
    }

    fn close_element(&mut self, _id: u32) -> KunaResult<()> {
        // The C++ CPUI_DEBUG block (additional-children / id-mismatch
        // checks) is compiled out of the test oracle and is not ported.
        self.el_stack.pop();
        self.iter_stack.pop();
        self.attribute_index = 1000; // Cannot read any additional attributes
        Ok(())
    }

    fn close_element_skipping(&mut self, _id: u32) -> KunaResult<()> {
        // The C++ CPUI_DEBUG id-mismatch check is compiled out of the test
        // oracle and is not ported.
        self.el_stack.pop();
        self.iter_stack.pop();
        self.attribute_index = 1000; // We could check that id matches current element
        Ok(())
    }

    fn get_next_attribute_id(&mut self) -> KunaResult<u32> {
        let el = self.current_element();
        let next_index = self.attribute_index + 1;
        if next_index < el.get_num_attributes() {
            self.attribute_index = next_index;
            return Ok(self
                .registry
                .find_attribute(el.get_attribute_name(self.attribute_index), self.scope));
        }
        Ok(0)
    }

    fn get_indexed_attribute_id(&mut self, attrib_id: &AttributeId) -> KunaResult<u32> {
        let el = self.current_element();
        if self.attribute_index < 0 || self.attribute_index >= el.get_num_attributes() {
            return Ok(ATTRIB_UNKNOWN.get_id());
        }
        // For XML, the index is encoded directly in the attribute name.
        // Does the name start with desired attribute base name?
        let attrib_name = el.get_attribute_name(self.attribute_index);
        if !attrib_name.starts_with(attrib_id.get_name()) {
            return Ok(ATTRIB_UNKNOWN.get_id());
        }
        // Strip off the base name, decode the remaining decimal integer
        // (starting at 1).  C++ `s >> dec >> val` into a uint4 is a
        // decimal-only num_get: overflow stores UINT_MAX, a minus sign
        // negates modularly, a failed extraction leaves the initialized 0.
        let suffix = &attrib_name.as_bytes()[attrib_id.get_name().len()..];
        let val: u32 = cxx_num_get_u32_dec(suffix);
        if val == 0 {
            return Err(KunaError::lowlevel(format!(
                "Bad indexed attribute: {}",
                attrib_id.get_name()
            )));
        }
        // C++ `attribId.getId() + (val-1)` in uint4 arithmetic: the add wraps
        // mod 2^32 when val came back UINT_MAX (the subtract cannot wrap —
        // val != 0 was just checked — but stays greppable wrapping intent).
        Ok(attrib_id.get_id().wadd(val.wsub(1)))
    }

    fn rewind_attributes(&mut self) {
        self.attribute_index = -1;
    }

    fn read_bool(&mut self) -> KunaResult<bool> {
        let el = self.current_element();
        Ok(xml::xml_readbool(el.get_attribute_value_at(self.attribute_index)))
    }

    fn read_bool_id(&mut self, attrib_id: &AttributeId) -> KunaResult<bool> {
        let el = self.current_element();
        if *attrib_id == ATTRIB_CONTENT {
            return Ok(xml::xml_readbool(el.get_content()));
        }
        let index = Self::find_matching_attribute(&el, attrib_id.get_name())?;
        Ok(xml::xml_readbool(el.get_attribute_value_at(index)))
    }

    fn read_signed_integer(&mut self) -> KunaResult<i64> {
        let el = self.current_element();
        Ok(cxx_parse_signed(el.get_attribute_value_at(self.attribute_index)))
    }

    fn read_signed_integer_id(&mut self, attrib_id: &AttributeId) -> KunaResult<i64> {
        let el = self.current_element();
        if *attrib_id == ATTRIB_CONTENT {
            return Ok(cxx_parse_signed(el.get_content()));
        }
        let index = Self::find_matching_attribute(&el, attrib_id.get_name())?;
        Ok(cxx_parse_signed(el.get_attribute_value_at(index)))
    }

    fn read_signed_integer_expect_string(
        &mut self,
        expect: &[u8],
        expectval: i64,
    ) -> KunaResult<i64> {
        let el = self.current_element();
        let value = el.get_attribute_value_at(self.attribute_index);
        if value == expect {
            return Ok(expectval);
        }
        Ok(cxx_parse_signed(value))
    }

    fn read_signed_integer_expect_string_id(
        &mut self,
        attrib_id: &AttributeId,
        expect: &[u8],
        expectval: i64,
    ) -> KunaResult<i64> {
        let value = self.read_string_id(attrib_id)?;
        if value == expect {
            return Ok(expectval);
        }
        Ok(cxx_parse_signed(&value))
    }

    fn read_unsigned_integer(&mut self) -> KunaResult<u64> {
        let el = self.current_element();
        Ok(cxx_parse_unsigned(el.get_attribute_value_at(self.attribute_index)))
    }

    fn read_unsigned_integer_id(&mut self, attrib_id: &AttributeId) -> KunaResult<u64> {
        let el = self.current_element();
        if *attrib_id == ATTRIB_CONTENT {
            return Ok(cxx_parse_unsigned(el.get_content()));
        }
        let index = Self::find_matching_attribute(&el, attrib_id.get_name())?;
        Ok(cxx_parse_unsigned(el.get_attribute_value_at(index)))
    }

    fn read_string(&mut self) -> KunaResult<Vec<u8>> {
        let el = self.current_element();
        Ok(el.get_attribute_value_at(self.attribute_index).to_vec())
    }

    fn read_string_id(&mut self, attrib_id: &AttributeId) -> KunaResult<Vec<u8>> {
        let el = self.current_element();
        if *attrib_id == ATTRIB_CONTENT {
            return Ok(el.get_content().to_vec());
        }
        let index = Self::find_matching_attribute(&el, attrib_id.get_name())?;
        Ok(el.get_attribute_value_at(index).to_vec())
    }

    fn read_space(&mut self) -> KunaResult<Rc<AddrSpace>> {
        let el = self.current_element();
        let nm = el.get_attribute_value_at(self.attribute_index).to_vec();
        lookup_space_by_name(self.spc_manager, &nm)
    }

    fn read_space_id(&mut self, attrib_id: &AttributeId) -> KunaResult<Rc<AddrSpace>> {
        let el = self.current_element();
        let nm: Vec<u8> = if *attrib_id == ATTRIB_CONTENT {
            el.get_content().to_vec()
        } else {
            let index = Self::find_matching_attribute(&el, attrib_id.get_name())?;
            el.get_attribute_value_at(index).to_vec()
        };
        lookup_space_by_name(self.spc_manager, &nm)
    }
}

/// Shared name -> space lookup for `XmlDecode::readSpace` variants.
/// A non-UTF-8 name cannot equal any registered (ASCII) space name, so it
/// falls through to the same "Unknown address space name" error as in C++.
fn lookup_space_by_name(manager: &AddrSpaceManager, nm: &[u8]) -> KunaResult<Rc<AddrSpace>> {
    if let Ok(nm_str) = std::str::from_utf8(nm) {
        if let Some(spc) = manager.get_space_by_name(nm_str) {
            return Ok(Rc::clone(spc));
        }
    }
    Err(KunaError::decoder(format!(
        "Unknown address space name: {}",
        String::from_utf8_lossy(nm)
    )))
}

// ---------------------------------------------------------------------------
// PackedFormat
// ---------------------------------------------------------------------------

/// \brief Protocol format for PackedEncode and PackedDecode classes
///
/// All bytes in the encoding are expected to be non-zero.  Element encoding
/// looks like
///   - 01xiiiii is an element start
///   - 10xiiiii is an element end
///   - 11xiiiii is an attribute start
///
/// Where iiiii is the (first) 5 bits of the element/attribute id.
/// If x=0, the id is complete.  If x=1, the next byte contains 7 more bits
/// of the id:  1iiiiiii
///
/// After an attribute start, there follows a \e type byte:  ttttllll, where
/// the first 4 bits indicate the type of attribute and final 4 bits are a
/// \b length \b code.  The types are:
///   - 1 = boolean (lengthcode=0 for false, lengthcode=1 for true)
///   - 2 = positive signed integer
///   - 3 = negative signed integer (stored in negated form)
///   - 4 = unsigned integer
///   - 5 = basic address space (encoded as the integer index of the space)
///   - 6 = special address space (lengthcode 0=>stack 1=>join 2=>fspec 3=>iop)
///   - 7 = string
///
/// All attribute types except \e boolean and \e special, have an encoded
/// integer after the \e type byte.  The \b length \b code, indicates the
/// number bytes used to encode the integer, 7-bits of info per byte,
/// 1iiiiiii.  A \b length \b code of zero is used to encode an integer value
/// of 0, with no following bytes.
///
/// For strings, the integer encoded after the \e type byte, is the actual
/// length of the string.  The string data itself is stored immediately after
/// the length integer using UTF8 format.
pub mod packed_format {
    /// Bits encoding the record type
    pub const HEADER_MASK: u8 = 0xc0;
    /// Header for an element start record
    pub const ELEMENT_START: u8 = 0x40;
    /// Header for an element end record
    pub const ELEMENT_END: u8 = 0x80;
    /// Header for an attribute record
    pub const ATTRIBUTE: u8 = 0xc0;
    /// Bit indicating the id extends into the next byte
    pub const HEADEREXTEND_MASK: u8 = 0x20;
    /// Bits encoding (part of) the id in the record header
    pub const ELEMENTID_MASK: u8 = 0x1f;
    /// Bits of raw data in follow-on bytes
    pub const RAWDATA_MASK: u8 = 0x7f;
    /// Number of bits used in a follow-on byte
    pub const RAWDATA_BITSPERBYTE: i32 = 7;
    /// The unused bit in follow-on bytes. (Always set to 1)
    pub const RAWDATA_MARKER: u8 = 0x80;
    /// Bit position of the type code in the type byte
    pub const TYPECODE_SHIFT: i32 = 4;
    /// Bits in the type byte forming the length code
    pub const LENGTHCODE_MASK: u8 = 0xf;
    /// Type code for the \e boolean type
    pub const TYPECODE_BOOLEAN: u8 = 1;
    /// Type code for the \e signed \e positive \e integer type
    pub const TYPECODE_SIGNEDINT_POSITIVE: u8 = 2;
    /// Type code for the \e signed \e negative \e integer type
    pub const TYPECODE_SIGNEDINT_NEGATIVE: u8 = 3;
    /// Type code for the \e unsigned \e integer type
    pub const TYPECODE_UNSIGNEDINT: u8 = 4;
    /// Type code for the \e address \e space type
    pub const TYPECODE_ADDRESSSPACE: u8 = 5;
    /// Type code for the \e special \e address \e space type
    pub const TYPECODE_SPECIALSPACE: u8 = 6;
    /// Type code for the \e string type
    pub const TYPECODE_STRING: u8 = 7;
    /// Special code for the \e stack space
    pub const SPECIALSPACE_STACK: u32 = 0;
    /// Special code for the \e join space
    pub const SPECIALSPACE_JOIN: u32 = 1;
    /// Special code for the \e fspec space
    pub const SPECIALSPACE_FSPEC: u32 = 2;
    /// Special code for the \e iop space
    pub const SPECIALSPACE_IOP: u32 = 3;
    /// Special code for a \e spacebase space
    pub const SPECIALSPACE_SPACEBASE: u32 = 4;
}

use packed_format as pf;

// ---------------------------------------------------------------------------
// PackedDecode
// ---------------------------------------------------------------------------

/// \brief An iterator into the input stream (C++ `PackedDecode::Position`)
///
/// The C++ Position holds a chunk-list iterator plus current/end raw
/// pointers; the Rust version holds the chunk index and byte offsets.
#[derive(Debug, Clone, Copy, Default)]
struct Position {
    /// Current byte sequence (index into `in_stream`)
    seq: usize,
    /// Current position in sequence
    current: usize,
    /// End of current sequence
    end: usize,
}

/// \brief A byte-based decoder designed to marshal info to the decompiler
/// efficiently
///
/// The decoder expects an encoding as described in [`packed_format`].  When
/// ingested, the stream bytes are held in a sequence of arrays (the C++
/// ByteChunk list).  During decoding, \b this object maintains a Position in
/// the stream at the start and end of the current open element, and a
/// Position of the next attribute to read to facilitate getNextAttributeId()
/// and associated read*() methods.
pub struct PackedDecode<'a> {
    /// Manager for decoding address space attributes
    spc_manager: &'a AddrSpaceManager,
    /// Incoming raw data as a sequence of byte arrays (C++ `list<ByteChunk>`)
    in_stream: Vec<Vec<u8>>,
    /// Position at the start of the current open element
    start_pos: Position,
    /// Position of the next attribute as returned by getNextAttributeId
    cur_pos: Position,
    /// Ending position after all attributes in current open element
    end_pos: Position,
    /// Has the last attribute returned by getNextAttributeId been read
    attribute_read: bool,
}

impl<'a> PackedDecode<'a> {
    /// The size, in bytes, of a single cached chunk of the input stream
    pub const BUFFER_SIZE: i32 = 1024;

    /// Constructor
    pub fn new(spc_manager: &'a AddrSpaceManager) -> Self {
        PackedDecode {
            spc_manager,
            in_stream: Vec::new(),
            start_pos: Position::default(),
            cur_pos: Position::default(),
            end_pos: Position::default(),
            attribute_read: false,
        }
    }

    /// Get the byte at the current position, do not advance
    fn get_byte(in_stream: &[Vec<u8>], pos: &Position) -> u8 {
        in_stream[pos.seq][pos.current]
    }

    /// Get the byte following the current byte, do not advance position.
    /// An error is returned if the position currently points to the last
    /// byte in the stream.
    fn get_byte_plus1(in_stream: &[Vec<u8>], pos: &Position) -> KunaResult<u8> {
        let ptr = pos.current + 1;
        if ptr == pos.end {
            let iter = pos.seq + 1;
            if iter == in_stream.len() {
                return Err(KunaError::decoder("Unexpected end of stream"));
            }
            return Ok(in_stream[iter][0]);
        }
        Ok(in_stream[pos.seq][ptr])
    }

    /// Get the byte at the current position and advance to the next byte.
    /// An error is returned if there are no additional bytes in the stream.
    fn get_next_byte(in_stream: &[Vec<u8>], pos: &mut Position) -> KunaResult<u8> {
        let res = in_stream[pos.seq][pos.current];
        pos.current += 1;
        if pos.current != pos.end {
            return Ok(res);
        }
        pos.seq += 1;
        if pos.seq == in_stream.len() {
            return Err(KunaError::decoder("Unexpected end of stream"));
        }
        pos.current = 0;
        pos.end = in_stream[pos.seq].len();
        Ok(res)
    }

    /// Advance the position by the given number of bytes.  An error is
    /// returned if the position is advanced past the end of the stream.
    fn advance_position(
        in_stream: &[Vec<u8>],
        pos: &mut Position,
        mut skip: u32,
    ) -> KunaResult<()> {
        // C++ compares `pos.end - pos.current <= skip` with ptrdiff_t vs
        // uint4 (skip converted up); both sides are non-negative here.
        while ((pos.end - pos.current) as u64) <= skip as u64 {
            skip -= (pos.end - pos.current) as u32;
            pos.seq += 1;
            if pos.seq == in_stream.len() {
                return Err(KunaError::decoder("Unexpected end of stream"));
            }
            pos.current = 0;
            pos.end = in_stream[pos.seq].len();
        }
        pos.current += skip as usize;
        Ok(())
    }

    /// Read an integer from the \e current position given its length in
    /// bytes.  The integer is encoded, 7-bits per byte, starting with the
    /// most significant 7-bits.  The integer is decoded from the \e current
    /// position, and the position is advanced.
    fn read_integer(&mut self, mut len: i32) -> KunaResult<u64> {
        let mut res: u64 = 0;
        while len > 0 {
            res <<= pf::RAWDATA_BITSPERBYTE;
            res |= (Self::get_next_byte(&self.in_stream, &mut self.cur_pos)? & pf::RAWDATA_MASK)
                as u64;
            len -= 1;
        }
        Ok(res)
    }

    /// Extract length code from type byte
    fn read_length_code(type_byte: u8) -> u32 {
        (type_byte & pf::LENGTHCODE_MASK) as u32
    }

    /// Find attribute matching the given id in open element.
    ///
    /// The \e current position is reset to the start of the current open
    /// element. Attributes are scanned and skipped until the attribute
    /// matching the given id is found.  The \e current position is set to
    /// the start of the matching attribute, in preparation for one of the
    /// read*() methods.  If the id is not found an error is returned.
    fn find_matching_attribute(&mut self, attrib_id: &AttributeId) -> KunaResult<()> {
        self.cur_pos = self.start_pos;
        loop {
            let header1 = Self::get_byte(&self.in_stream, &self.cur_pos);
            if (header1 & pf::HEADER_MASK) != pf::ATTRIBUTE {
                break;
            }
            let mut id = (header1 & pf::ELEMENTID_MASK) as u32;
            if (header1 & pf::HEADEREXTEND_MASK) != 0 {
                id <<= pf::RAWDATA_BITSPERBYTE;
                id |= (Self::get_byte_plus1(&self.in_stream, &self.cur_pos)? & pf::RAWDATA_MASK)
                    as u32;
            }
            if attrib_id.get_id() == id {
                return Ok(()); // Found it
            }
            self.skip_attribute()?;
        }
        Err(KunaError::decoder(format!(
            "Attribute {} is not present",
            attrib_id.get_name()
        )))
    }

    /// Skip over the attribute at the current position.
    ///
    /// The attribute at the \e current position is scanned enough to
    /// determine its length, and the position is advanced to the following
    /// byte.
    fn skip_attribute(&mut self) -> KunaResult<()> {
        let header1 = Self::get_next_byte(&self.in_stream, &mut self.cur_pos)?; // Attribute header
        if (header1 & pf::HEADEREXTEND_MASK) != 0 {
            Self::get_next_byte(&self.in_stream, &mut self.cur_pos)?; // Extra byte for extended id
        }
        let type_byte = Self::get_next_byte(&self.in_stream, &mut self.cur_pos)?; // Type (and length) byte
        let attrib_type = type_byte >> pf::TYPECODE_SHIFT;
        if attrib_type == pf::TYPECODE_BOOLEAN || attrib_type == pf::TYPECODE_SPECIALSPACE {
            return Ok(()); // has no additional data
        }
        let mut length = Self::read_length_code(type_byte); // Length of data in bytes
        if attrib_type == pf::TYPECODE_STRING {
            // Read length field to get final length of string
            // cast: uint4 length -> int4 readInteger arg (length code <= 0x1f,
            // in range); uint8 result -> uint4 truncation, as the C++
            // assignment `length = readInteger(length)` (marshal.cc:653)
            length = self.read_integer(length as i32)? as u32;
        }
        Self::advance_position(&self.in_stream, &mut self.cur_pos, length) // Skip -length- data
    }

    /// Skip over remaining attribute data, after a mismatch.
    ///
    /// This assumes the header and \b type \b byte have been read.  Decode
    /// type and length info and finish skipping over the attribute so that
    /// the next call to getNextAttributeId() is on cut.
    fn skip_attribute_remaining(&mut self, type_byte: u8) -> KunaResult<()> {
        let attrib_type = type_byte >> pf::TYPECODE_SHIFT;
        if attrib_type == pf::TYPECODE_BOOLEAN || attrib_type == pf::TYPECODE_SPECIALSPACE {
            return Ok(()); // has no additional data
        }
        let mut length = Self::read_length_code(type_byte); // Length of data in bytes
        if attrib_type == pf::TYPECODE_STRING {
            // Read length field to get final length of string
            // cast: uint4 length -> int4 readInteger arg (length code <= 0x1f,
            // in range); uint8 result -> uint4 truncation, as the C++
            // assignment `length = readInteger(length)` (marshal.cc:669)
            length = self.read_integer(length as i32)? as u32;
        }
        Self::advance_position(&self.in_stream, &mut self.cur_pos, length) // Skip -length- data
    }

    /// Finish set up for reading input stream.
    ///
    /// Set decoder to beginning of the stream.  Add padding to end of the
    /// stream.  `buf_pos` is the number of bytes used by the last input
    /// buffer.
    fn end_ingest(&mut self, mut buf_pos: i32) -> KunaResult<()> {
        if self.in_stream.is_empty() {
            return Err(KunaError::decoder("Ended ingestion without any input"));
        }
        // Set position to beginning of stream
        self.end_pos.seq = 0;
        self.end_pos.current = 0;
        self.end_pos.end = self.in_stream[0].len();
        // Make sure there is at least one character after ingested buffer
        if buf_pos == Self::BUFFER_SIZE {
            // Last buffer was entirely filled; add one more buffer
            self.in_stream.push(vec![0u8; 1]);
            buf_pos = 0;
        }
        let last = self.in_stream.last_mut().expect("non-empty in_stream");
        last[buf_pos as usize] = pf::ELEMENT_END;
        Ok(())
    }
}

impl Decoder for PackedDecode<'_> {
    fn get_addr_space_manager(&self) -> &AddrSpaceManager {
        self.spc_manager
    }

    fn ingest_stream(&mut self, s: &[u8]) -> KunaResult<()> {
        // The C++ loop reads with `istream::get(buf, BUFFER_SIZE+1, '\0')`
        // while `s.peek() > 0`: the effective input ends at the first NUL
        // byte (or end of slice).
        let data = match s.iter().position(|&b| b == 0) {
            Some(idx) => &s[..idx],
            None => s,
        };
        let mut gcount: i32 = 0;
        let mut pos = 0usize;
        while pos < data.len() {
            let n = (data.len() - pos).min(Self::BUFFER_SIZE as usize);
            // Allocate the next chunk (C++ allocateNextInputBuffer; the +1
            // pad byte only held istream::get's terminating NUL and is not
            // part of the chunk's logical extent).  Bytes beyond the data
            // are zero-filled (uninitialized in C++; never validly read).
            let mut buf = vec![0u8; Self::BUFFER_SIZE as usize];
            buf[..n].copy_from_slice(&data[pos..pos + n]);
            self.in_stream.push(buf);
            gcount = n as i32;
            pos += n;
        }
        self.end_ingest(gcount)
    }

    fn peek_element(&mut self) -> KunaResult<u32> {
        let header1 = Self::get_byte(&self.in_stream, &self.end_pos);
        if (header1 & pf::HEADER_MASK) != pf::ELEMENT_START {
            return Ok(0);
        }
        let mut id = (header1 & pf::ELEMENTID_MASK) as u32;
        if (header1 & pf::HEADEREXTEND_MASK) != 0 {
            id <<= pf::RAWDATA_BITSPERBYTE;
            id |= (Self::get_byte_plus1(&self.in_stream, &self.end_pos)? & pf::RAWDATA_MASK) as u32;
        }
        Ok(id)
    }

    fn open_element(&mut self) -> KunaResult<u32> {
        let mut header1 = Self::get_byte(&self.in_stream, &self.end_pos);
        if (header1 & pf::HEADER_MASK) != pf::ELEMENT_START {
            return Ok(0);
        }
        Self::get_next_byte(&self.in_stream, &mut self.end_pos)?;
        let mut id = (header1 & pf::ELEMENTID_MASK) as u32;
        if (header1 & pf::HEADEREXTEND_MASK) != 0 {
            id <<= pf::RAWDATA_BITSPERBYTE;
            id |= (Self::get_next_byte(&self.in_stream, &mut self.end_pos)? & pf::RAWDATA_MASK)
                as u32;
        }
        self.start_pos = self.end_pos;
        self.cur_pos = self.end_pos;
        header1 = Self::get_byte(&self.in_stream, &self.cur_pos);
        while (header1 & pf::HEADER_MASK) == pf::ATTRIBUTE {
            self.skip_attribute()?;
            header1 = Self::get_byte(&self.in_stream, &self.cur_pos);
        }
        self.end_pos = self.cur_pos;
        self.cur_pos = self.start_pos;
        self.attribute_read = true; // "Last attribute was read" is vacuously true
        Ok(id)
    }

    fn open_element_id(&mut self, elem_id: &ElementId) -> KunaResult<u32> {
        let id = self.open_element()?;
        if id != elem_id.get_id() {
            if id == 0 {
                return Err(KunaError::decoder(format!(
                    "Expecting <{}> but did not scan an element",
                    elem_id.get_name()
                )));
            }
            return Err(KunaError::decoder(format!(
                "Expecting <{}> but id did not match",
                elem_id.get_name()
            )));
        }
        Ok(id)
    }

    fn close_element(&mut self, id: u32) -> KunaResult<()> {
        let header1 = Self::get_next_byte(&self.in_stream, &mut self.end_pos)?;
        if (header1 & pf::HEADER_MASK) != pf::ELEMENT_END {
            return Err(KunaError::decoder("Expecting element close"));
        }
        let mut close_id = (header1 & pf::ELEMENTID_MASK) as u32;
        if (header1 & pf::HEADEREXTEND_MASK) != 0 {
            close_id <<= pf::RAWDATA_BITSPERBYTE;
            close_id |= (Self::get_next_byte(&self.in_stream, &mut self.end_pos)?
                & pf::RAWDATA_MASK) as u32;
        }
        if id != close_id {
            return Err(KunaError::decoder("Did not see expected closing element"));
        }
        Ok(())
    }

    fn close_element_skipping(&mut self, id: u32) -> KunaResult<()> {
        let mut idstack: Vec<u32> = vec![id];
        while !idstack.is_empty() {
            let header1 = Self::get_byte(&self.in_stream, &self.end_pos) & pf::HEADER_MASK;
            if header1 == pf::ELEMENT_END {
                let top = *idstack.last().expect("idstack non-empty");
                self.close_element(top)?;
                idstack.pop();
            } else if header1 == pf::ELEMENT_START {
                idstack.push(self.open_element()?);
            } else {
                return Err(KunaError::decoder("Corrupt stream"));
            }
        }
        Ok(())
    }

    fn get_next_attribute_id(&mut self) -> KunaResult<u32> {
        if !self.attribute_read {
            self.skip_attribute()?;
        }
        let header1 = Self::get_byte(&self.in_stream, &self.cur_pos);
        if (header1 & pf::HEADER_MASK) != pf::ATTRIBUTE {
            return Ok(0);
        }
        let mut id = (header1 & pf::ELEMENTID_MASK) as u32;
        if (header1 & pf::HEADEREXTEND_MASK) != 0 {
            id <<= pf::RAWDATA_BITSPERBYTE;
            id |= (Self::get_byte_plus1(&self.in_stream, &self.cur_pos)? & pf::RAWDATA_MASK) as u32;
        }
        self.attribute_read = false;
        Ok(id)
    }

    fn get_indexed_attribute_id(&mut self, _attrib_id: &AttributeId) -> KunaResult<u32> {
        // PackedDecode never needs to reinterpret an attribute
        Ok(ATTRIB_UNKNOWN.get_id())
    }

    fn rewind_attributes(&mut self) {
        self.cur_pos = self.start_pos;
        self.attribute_read = true;
    }

    fn read_bool(&mut self) -> KunaResult<bool> {
        let header1 = Self::get_next_byte(&self.in_stream, &mut self.cur_pos)?;
        if (header1 & pf::HEADEREXTEND_MASK) != 0 {
            Self::get_next_byte(&self.in_stream, &mut self.cur_pos)?;
        }
        let type_byte = Self::get_next_byte(&self.in_stream, &mut self.cur_pos)?;
        self.attribute_read = true;
        if (type_byte >> pf::TYPECODE_SHIFT) != pf::TYPECODE_BOOLEAN {
            return Err(KunaError::decoder("Expecting boolean attribute"));
        }
        Ok((type_byte & pf::LENGTHCODE_MASK) != 0)
    }

    fn read_bool_id(&mut self, attrib_id: &AttributeId) -> KunaResult<bool> {
        self.find_matching_attribute(attrib_id)?;
        let res = self.read_bool()?;
        self.cur_pos = self.start_pos;
        Ok(res)
    }

    fn read_signed_integer(&mut self) -> KunaResult<i64> {
        let header1 = Self::get_next_byte(&self.in_stream, &mut self.cur_pos)?;
        if (header1 & pf::HEADEREXTEND_MASK) != 0 {
            Self::get_next_byte(&self.in_stream, &mut self.cur_pos)?;
        }
        let type_byte = Self::get_next_byte(&self.in_stream, &mut self.cur_pos)?;
        let type_code = type_byte >> pf::TYPECODE_SHIFT;
        let res: i64;
        if type_code == pf::TYPECODE_SIGNEDINT_POSITIVE {
            res = self.read_integer(Self::read_length_code(type_byte) as i32)? as i64;
        } else if type_code == pf::TYPECODE_SIGNEDINT_NEGATIVE {
            // Stored in negated form; wrapping negate reproduces the C++
            // `res = -res` (i64::MIN round-trips through its own negation).
            res = (self.read_integer(Self::read_length_code(type_byte) as i32)? as i64)
                .wrapping_neg();
        } else {
            self.skip_attribute_remaining(type_byte)?;
            self.attribute_read = true;
            return Err(KunaError::decoder("Expecting signed integer attribute"));
        }
        self.attribute_read = true;
        Ok(res)
    }

    fn read_signed_integer_id(&mut self, attrib_id: &AttributeId) -> KunaResult<i64> {
        self.find_matching_attribute(attrib_id)?;
        let res = self.read_signed_integer()?;
        self.cur_pos = self.start_pos;
        Ok(res)
    }

    fn read_signed_integer_expect_string(
        &mut self,
        expect: &[u8],
        expectval: i64,
    ) -> KunaResult<i64> {
        // Peek at the type byte without advancing curPos (C++ tmpPos copy)
        let mut tmp_pos = self.cur_pos;
        let header1 = Self::get_next_byte(&self.in_stream, &mut tmp_pos)?;
        if (header1 & pf::HEADEREXTEND_MASK) != 0 {
            Self::get_next_byte(&self.in_stream, &mut tmp_pos)?;
        }
        let type_byte = Self::get_next_byte(&self.in_stream, &mut tmp_pos)?;
        let type_code = type_byte >> pf::TYPECODE_SHIFT;
        if type_code == pf::TYPECODE_STRING {
            let val = self.read_string()?;
            if val != expect {
                return Err(KunaError::decoder(format!(
                    "Expecting string \"{}\" but read \"{}\"",
                    String::from_utf8_lossy(expect),
                    String::from_utf8_lossy(&val)
                )));
            }
            Ok(expectval)
        } else {
            self.read_signed_integer()
        }
    }

    fn read_signed_integer_expect_string_id(
        &mut self,
        attrib_id: &AttributeId,
        expect: &[u8],
        expectval: i64,
    ) -> KunaResult<i64> {
        self.find_matching_attribute(attrib_id)?;
        let res = self.read_signed_integer_expect_string(expect, expectval)?;
        self.cur_pos = self.start_pos;
        Ok(res)
    }

    fn read_unsigned_integer(&mut self) -> KunaResult<u64> {
        let header1 = Self::get_next_byte(&self.in_stream, &mut self.cur_pos)?;
        if (header1 & pf::HEADEREXTEND_MASK) != 0 {
            Self::get_next_byte(&self.in_stream, &mut self.cur_pos)?;
        }
        let type_byte = Self::get_next_byte(&self.in_stream, &mut self.cur_pos)?;
        let type_code = type_byte >> pf::TYPECODE_SHIFT;
        if type_code != pf::TYPECODE_UNSIGNEDINT {
            self.skip_attribute_remaining(type_byte)?;
            self.attribute_read = true;
            return Err(KunaError::decoder("Expecting unsigned integer attribute"));
        }
        let res = self.read_integer(Self::read_length_code(type_byte) as i32)?;
        self.attribute_read = true;
        Ok(res)
    }

    fn read_unsigned_integer_id(&mut self, attrib_id: &AttributeId) -> KunaResult<u64> {
        self.find_matching_attribute(attrib_id)?;
        let res = self.read_unsigned_integer()?;
        self.cur_pos = self.start_pos;
        Ok(res)
    }

    fn read_string(&mut self) -> KunaResult<Vec<u8>> {
        let header1 = Self::get_next_byte(&self.in_stream, &mut self.cur_pos)?;
        if (header1 & pf::HEADEREXTEND_MASK) != 0 {
            Self::get_next_byte(&self.in_stream, &mut self.cur_pos)?;
        }
        let type_byte = Self::get_next_byte(&self.in_stream, &mut self.cur_pos)?;
        let type_code = type_byte >> pf::TYPECODE_SHIFT;
        if type_code != pf::TYPECODE_STRING {
            self.skip_attribute_remaining(type_byte)?;
            self.attribute_read = true;
            return Err(KunaError::decoder("Expecting string attribute"));
        }
        let mut length = self.read_integer(Self::read_length_code(type_byte) as i32)?;

        self.attribute_read = true;
        let cur_len = (self.cur_pos.end - self.cur_pos.current) as u64;
        if cur_len >= length {
            let start = self.cur_pos.current;
            let res = self.in_stream[self.cur_pos.seq][start..start + length as usize].to_vec();
            Self::advance_position(&self.in_stream, &mut self.cur_pos, length as u32)?;
            return Ok(res);
        }
        let start = self.cur_pos.current;
        let mut res = self.in_stream[self.cur_pos.seq][start..start + cur_len as usize].to_vec();
        length -= cur_len;
        Self::advance_position(&self.in_stream, &mut self.cur_pos, cur_len as u32)?;
        while length > 0 {
            let mut cur_len = (self.cur_pos.end - self.cur_pos.current) as u64;
            if cur_len > length {
                cur_len = length;
            }
            let start = self.cur_pos.current;
            res.extend_from_slice(
                &self.in_stream[self.cur_pos.seq][start..start + cur_len as usize],
            );
            length -= cur_len;
            Self::advance_position(&self.in_stream, &mut self.cur_pos, cur_len as u32)?;
        }
        Ok(res)
    }

    fn read_string_id(&mut self, attrib_id: &AttributeId) -> KunaResult<Vec<u8>> {
        self.find_matching_attribute(attrib_id)?;
        let res = self.read_string()?;
        self.cur_pos = self.start_pos;
        Ok(res)
    }

    fn read_space(&mut self) -> KunaResult<Rc<AddrSpace>> {
        let header1 = Self::get_next_byte(&self.in_stream, &mut self.cur_pos)?;
        if (header1 & pf::HEADEREXTEND_MASK) != 0 {
            Self::get_next_byte(&self.in_stream, &mut self.cur_pos)?;
        }
        let type_byte = Self::get_next_byte(&self.in_stream, &mut self.cur_pos)?;
        let type_code = type_byte >> pf::TYPECODE_SHIFT;
        let spc: Rc<AddrSpace>;
        if type_code == pf::TYPECODE_ADDRESSSPACE {
            let res = self.read_integer(Self::read_length_code(type_byte) as i32)?;
            // mixed comparison: uint8 res vs int4 numSpaces (converted up)
            if res >= self.spc_manager.num_spaces() as u64 {
                return Err(KunaError::decoder("Invalid address space index"));
            }
            match self.spc_manager.get_space(res as i32) {
                Some(s) => spc = Rc::clone(s),
                None => return Err(KunaError::decoder("Unknown address space index")),
            }
        } else if type_code == pf::TYPECODE_SPECIALSPACE {
            let special_code = Self::read_length_code(type_byte);
            if special_code == pf::SPECIALSPACE_STACK {
                // C++ returns the manager's (possibly null) pointer; a null
                // here is dereferenced downstream (UB) => panic (ADR 0004).
                spc = Rc::clone(
                    self.spc_manager
                        .get_stack_space()
                        .expect("stack space not registered in AddrSpaceManager"),
                );
            } else if special_code == pf::SPECIALSPACE_JOIN {
                spc = Rc::clone(
                    self.spc_manager
                        .get_join_space()
                        .expect("join space not registered in AddrSpaceManager"),
                );
            } else {
                return Err(KunaError::decoder("Cannot marshal special address space"));
            }
        } else {
            self.skip_attribute_remaining(type_byte)?;
            self.attribute_read = true;
            return Err(KunaError::decoder("Expecting space attribute"));
        }
        self.attribute_read = true;
        Ok(spc)
    }

    fn read_space_id(&mut self, attrib_id: &AttributeId) -> KunaResult<Rc<AddrSpace>> {
        self.find_matching_attribute(attrib_id)?;
        let res = self.read_space()?;
        self.cur_pos = self.start_pos;
        Ok(res)
    }
}

// ---------------------------------------------------------------------------
// PackedEncode
// ---------------------------------------------------------------------------

/// \brief A byte-based encoder designed to marshal from the decompiler
/// efficiently
///
/// See [`PackedDecode`] for details of the encoding format.
pub struct PackedEncode<'a> {
    /// The stream receiving the encoded data
    out_stream: &'a mut Vec<u8>,
}

impl<'a> PackedEncode<'a> {
    /// Construct from a stream
    pub fn new(s: &'a mut Vec<u8>) -> Self {
        PackedEncode { out_stream: s }
    }

    /// Write a header, element or attribute, to stream
    fn write_header(&mut self, header: u8, id: u32) {
        if id > 0x1f {
            let mut header = header;
            header |= pf::HEADEREXTEND_MASK;
            // cast: the shifted uint4 id is |='d into the uint1 header, keeping
            // the low byte, as the C++ `header |= (id >> ...)` (marshal.hh:666)
            header |= (id >> pf::RAWDATA_BITSPERBYTE) as u8;
            // cast: uint4 id -> uint1 low-byte truncation before the mask, as
            // the C++ `uint1 extendByte = (id & RAWDATA_MASK) | ...`
            // (marshal.hh:667)
            let extend_byte = ((id as u8) & pf::RAWDATA_MASK) | pf::RAWDATA_MARKER;
            self.out_stream.push(header);
            self.out_stream.push(extend_byte);
        } else {
            self.out_stream.push(header | id as u8);
        }
    }

    /// Write an integer value to the stream.
    ///
    /// The value is either an unsigned integer, an address space index, or
    /// (the absolute value of) a signed integer.  A type header is passed in
    /// with the particular type code for the value already filled in.  This
    /// method then fills in the length code, outputs the full type header
    /// and the encoded bytes of the integer.
    fn write_integer(&mut self, type_byte: u8, val: u64) {
        let len_code: u8;
        let mut sa: i32;
        if val == 0 {
            len_code = 0;
            sa = -1;
        } else if val < 0x800000000 {
            if val < 0x200000 {
                if val < 0x80 {
                    len_code = 1; // 7-bits
                    sa = 0;
                } else if val < 0x4000 {
                    len_code = 2; // 14-bits
                    sa = pf::RAWDATA_BITSPERBYTE;
                } else {
                    len_code = 3; // 21-bits
                    sa = 2 * pf::RAWDATA_BITSPERBYTE;
                }
            } else if val < 0x10000000 {
                len_code = 4; // 28-bits
                sa = 3 * pf::RAWDATA_BITSPERBYTE;
            } else {
                len_code = 5; // 35-bits
                sa = 4 * pf::RAWDATA_BITSPERBYTE;
            }
        } else if val < 0x2000000000000 {
            if val < 0x40000000000 {
                len_code = 6;
                sa = 5 * pf::RAWDATA_BITSPERBYTE;
            } else {
                len_code = 7;
                sa = 6 * pf::RAWDATA_BITSPERBYTE;
            }
        } else if val < 0x100000000000000 {
            len_code = 8;
            sa = 7 * pf::RAWDATA_BITSPERBYTE;
        } else if val < 0x8000000000000000 {
            len_code = 9;
            sa = 8 * pf::RAWDATA_BITSPERBYTE;
        } else {
            len_code = 10;
            sa = 9 * pf::RAWDATA_BITSPERBYTE;
        }
        self.out_stream.push(type_byte | len_code);
        while sa >= 0 {
            let mut piece = ((val >> sa) as u8) & pf::RAWDATA_MASK;
            piece |= pf::RAWDATA_MARKER;
            self.out_stream.push(piece);
            sa -= pf::RAWDATA_BITSPERBYTE;
        }
    }
}

impl Encoder for PackedEncode<'_> {
    fn open_element(&mut self, elem_id: &ElementId) {
        self.write_header(pf::ELEMENT_START, elem_id.get_id());
    }

    fn close_element(&mut self, elem_id: &ElementId) {
        self.write_header(pf::ELEMENT_END, elem_id.get_id());
    }

    fn write_bool(&mut self, attrib_id: &AttributeId, val: bool) {
        self.write_header(pf::ATTRIBUTE, attrib_id.get_id());
        let type_byte = if val {
            (pf::TYPECODE_BOOLEAN << pf::TYPECODE_SHIFT) | 1
        } else {
            pf::TYPECODE_BOOLEAN << pf::TYPECODE_SHIFT
        };
        self.out_stream.push(type_byte);
    }

    fn write_signed_integer(&mut self, attrib_id: &AttributeId, val: i64) {
        self.write_header(pf::ATTRIBUTE, attrib_id.get_id());
        let type_byte: u8;
        let num: u64;
        if val < 0 {
            type_byte = pf::TYPECODE_SIGNEDINT_NEGATIVE << pf::TYPECODE_SHIFT;
            // C++ `num = -val` (intb negate converted to uint8); wrapping
            // negate so that i64::MIN encodes as 0x8000000000000000.
            num = val.wrapping_neg() as u64;
        } else {
            type_byte = pf::TYPECODE_SIGNEDINT_POSITIVE << pf::TYPECODE_SHIFT;
            num = val as u64;
        }
        self.write_integer(type_byte, num);
    }

    fn write_unsigned_integer(&mut self, attrib_id: &AttributeId, val: u64) {
        self.write_header(pf::ATTRIBUTE, attrib_id.get_id());
        self.write_integer(pf::TYPECODE_UNSIGNEDINT << pf::TYPECODE_SHIFT, val);
    }

    fn write_string(&mut self, attrib_id: &AttributeId, val: &[u8]) {
        let length = val.len() as u64;
        self.write_header(pf::ATTRIBUTE, attrib_id.get_id());
        self.write_integer(pf::TYPECODE_STRING << pf::TYPECODE_SHIFT, length);
        self.out_stream.extend_from_slice(val);
    }

    fn write_string_indexed(&mut self, attrib_id: &AttributeId, index: u32, val: &[u8]) {
        let length = val.len() as u64;
        // C++ `attribId.getId() + index` is uint4 arithmetic: wraps mod 2^32
        self.write_header(pf::ATTRIBUTE, attrib_id.get_id().wadd(index));
        self.write_integer(pf::TYPECODE_STRING << pf::TYPECODE_SHIFT, length);
        self.out_stream.extend_from_slice(val);
    }

    fn write_space(&mut self, attrib_id: &AttributeId, spc: &AddrSpace) {
        self.write_header(pf::ATTRIBUTE, attrib_id.get_id());
        match spc.get_type() {
            spacetype::IPTR_FSPEC => {
                self.out_stream.push(
                    (pf::TYPECODE_SPECIALSPACE << pf::TYPECODE_SHIFT)
                        | pf::SPECIALSPACE_FSPEC as u8,
                );
            }
            spacetype::IPTR_IOP => {
                self.out_stream.push(
                    (pf::TYPECODE_SPECIALSPACE << pf::TYPECODE_SHIFT) | pf::SPECIALSPACE_IOP as u8,
                );
            }
            spacetype::IPTR_JOIN => {
                self.out_stream.push(
                    (pf::TYPECODE_SPECIALSPACE << pf::TYPECODE_SHIFT) | pf::SPECIALSPACE_JOIN as u8,
                );
            }
            spacetype::IPTR_SPACEBASE => {
                if spc.is_formal_stack_space() {
                    self.out_stream.push(
                        (pf::TYPECODE_SPECIALSPACE << pf::TYPECODE_SHIFT)
                            | pf::SPECIALSPACE_STACK as u8,
                    );
                } else {
                    // A secondary register offset space
                    self.out_stream.push(
                        (pf::TYPECODE_SPECIALSPACE << pf::TYPECODE_SHIFT)
                            | pf::SPECIALSPACE_SPACEBASE as u8,
                    );
                }
            }
            _ => {
                // cast: int4 getIndex() sign-extends to uint8, as the C++
                // `uint8 spcId = spc->getIndex()` (marshal.cc:1218)
                let spc_id = spc.get_index() as u64;
                self.write_integer(pf::TYPECODE_ADDRESSSPACE << pf::TYPECODE_SHIFT, spc_id);
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::space::AddrSpace;

    // ELEM_ADDR lives in address.rs; alias for test brevity.
    const ELEM_ADDR_TEST: ElementId = crate::address::ELEM_ADDR;

    /// Mirror of testmarshal.cc's TestAddrSpaceManager: a manager holding a
    /// single big "ram" space at index 3.
    fn test_manager() -> AddrSpaceManager {
        let mut manager = AddrSpaceManager::new();
        let ram = AddrSpace::new(
            spacetype::IPTR_PROCESSOR,
            "ram",
            false, // DummyTranslate::isBigEndian()
            8,
            1,
            3,
            crate::space::addrspace_flags::hasphysical,
            1,
            1,
        );
        manager.insert_space(Rc::new(ram)).unwrap();
        manager
    }

    /// Boundary values exercising every packed length code (the 0x7f/0x80
    /// style boundaries up through 64 bits, including 2^32 and u64::MAX).
    const UNSIGNED_BOUNDARIES: &[u64] = &[
        0,
        1,
        0x7f,
        0x80,
        0x3fff,
        0x4000,
        0x1fffff,
        0x200000,
        0xfffffff,
        0x10000000,
        0xffffffff,  // 2^32 - 1
        0x100000000, // 2^32
        0x7ffffffff,
        0x800000000,
        0x3ffffffffff,
        0x40000000000,
        0x1ffffffffffff,
        0x2000000000000,
        0xffffffffffffff,
        0x100000000000000,
        0x7fffffffffffffff,
        0x8000000000000000,
        u64::MAX,
    ];

    #[test]
    fn test_marshal_packed_unsigned_roundtrip_boundaries() {
        let manager = test_manager();
        for &val in UNSIGNED_BOUNDARIES {
            let mut buf = Vec::new();
            {
                let mut enc = PackedEncode::new(&mut buf);
                enc.open_element(&ELEM_ADDR_TEST);
                enc.write_unsigned_integer(&ATTRIB_OFFSET, val);
                enc.close_element(&ELEM_ADDR_TEST);
            }
            let mut dec = PackedDecode::new(&manager);
            dec.ingest_stream(&buf).unwrap();
            let el = dec.open_element_id(&ELEM_ADDR_TEST).unwrap();
            let rt = dec.read_unsigned_integer_id(&ATTRIB_OFFSET).unwrap();
            assert_eq!(rt, val, "unsigned round trip failed for {val:#x}");
            dec.close_element(el).unwrap();
        }
    }

    #[test]
    fn test_marshal_packed_signed_roundtrip_boundaries() {
        let manager = test_manager();
        let mut vals: Vec<i64> = Vec::new();
        for &u in UNSIGNED_BOUNDARIES {
            vals.push(u as i64);
            vals.push((u as i64).wrapping_neg());
        }
        vals.push(i64::MIN);
        vals.push(i64::MAX);
        for &val in &vals {
            let mut buf = Vec::new();
            {
                let mut enc = PackedEncode::new(&mut buf);
                enc.open_element(&ELEM_ADDR_TEST);
                enc.write_signed_integer(&ATTRIB_OFFSET, val);
                enc.close_element(&ELEM_ADDR_TEST);
            }
            let mut dec = PackedDecode::new(&manager);
            dec.ingest_stream(&buf).unwrap();
            let el = dec.open_element_id(&ELEM_ADDR_TEST).unwrap();
            let rt = dec.read_signed_integer_id(&ATTRIB_OFFSET).unwrap();
            assert_eq!(rt, val, "signed round trip failed for {val:#x}");
            dec.close_element(el).unwrap();
        }
    }

    #[test]
    fn test_marshal_packed_exact_bytes() {
        // Byte-exactness probes for the packed protocol (any change here is
        // a save-file format break).  ELEM_DATA id 1; ATTRIB_OFFSET id 16.
        let mut buf = Vec::new();
        {
            let mut enc = PackedEncode::new(&mut buf);
            enc.open_element(&ELEM_DATA); // 0x40 | 1
            enc.write_bool(&ATTRIB_ALIGN, true); // header 0xc0|2, type (1<<4)|1
            enc.write_unsigned_integer(&ATTRIB_OFFSET, 0); // header 0xc0|16, type (4<<4)|0
            enc.write_unsigned_integer(&ATTRIB_OFFSET, 0x7f); // lencode 1: 0x80|0x7f
            enc.write_unsigned_integer(&ATTRIB_OFFSET, 0x80); // lencode 2: 0x81, 0x80
            enc.write_signed_integer(&ATTRIB_OFFSET, -1); // negative type 3, lencode 1
            enc.close_element(&ELEM_DATA); // 0x80 | 1
        }
        assert_eq!(
            buf,
            vec![
                0x41, // <data>
                0xc2, 0x11, // align=true
                0xd0, 0x40, // offset=0
                0xd0, 0x41, 0xff, // offset=0x7f
                0xd0, 0x42, 0x81, 0x80, // offset=0x80
                0xd0, 0x31, 0x81, // offset=-1
                0x81, // </data>
            ]
        );
        // Extended (>0x1f) id header: ATTRIB_STORAGE id 149
        let mut buf2 = Vec::new();
        {
            let mut enc = PackedEncode::new(&mut buf2);
            enc.write_bool(&ATTRIB_STORAGE, false);
        }
        assert_eq!(buf2, vec![0xc0 | 0x20 | (149 >> 7), 0x80 | (149 & 0x7f), 0x10]);
    }

    #[test]
    fn test_marshal_packed_string_and_space_roundtrip() {
        let manager = test_manager();
        let mut buf = Vec::new();
        {
            let mut enc = PackedEncode::new(&mut buf);
            enc.open_element(&ELEM_DATA);
            enc.write_string(&ATTRIB_VALUE, b"hello");
            enc.write_string(&ATTRIB_VAL, b"");
            let spc = manager.get_space(3).unwrap();
            enc.write_space(&ATTRIB_SPACE, spc);
            enc.close_element(&ELEM_DATA);
        }
        let mut dec = PackedDecode::new(&manager);
        dec.ingest_stream(&buf).unwrap();
        let el = dec.open_element_id(&ELEM_DATA).unwrap();
        assert_eq!(dec.read_string_id(&ATTRIB_VALUE).unwrap(), b"hello");
        assert_eq!(dec.read_string_id(&ATTRIB_VAL).unwrap(), b"");
        let spc = dec.read_space_id(&ATTRIB_SPACE).unwrap();
        assert!(Rc::ptr_eq(&spc, manager.get_space(3).unwrap()));
        dec.close_element(el).unwrap();
    }

    #[test]
    fn test_marshal_xml_roundtrip() {
        let manager = test_manager();
        let registry = IdRegistry::with_base_ids();
        let mut buf = Vec::new();
        {
            let mut enc = XmlEncode::new(&mut buf);
            enc.open_element(&ELEM_DATA);
            enc.write_bool(&ATTRIB_ALIGN, true);
            enc.write_signed_integer(&ATTRIB_ID, -0x1234);
            enc.write_unsigned_integer(&ATTRIB_OFFSET, 0xdeadbeef);
            enc.write_string(&ATTRIB_VALUE, b"<<&\"bl a h'\\bleh\n\t");
            let spc = manager.get_space(3).unwrap();
            enc.write_space(&ATTRIB_SPACE, spc);
            enc.close_element(&ELEM_DATA);
        }
        let mut dec = XmlDecode::new(&manager, &registry);
        dec.ingest_stream(&buf).unwrap();
        let el = dec.open_element_id(&ELEM_DATA).unwrap();
        assert!(dec.read_bool_id(&ATTRIB_ALIGN).unwrap());
        assert_eq!(dec.read_signed_integer_id(&ATTRIB_ID).unwrap(), -0x1234);
        assert_eq!(dec.read_unsigned_integer_id(&ATTRIB_OFFSET).unwrap(), 0xdeadbeef);
        assert_eq!(dec.read_string_id(&ATTRIB_VALUE).unwrap(), b"<<&\"bl a h'\\bleh\n\t");
        let spc = dec.read_space_id(&ATTRIB_SPACE).unwrap();
        assert!(Rc::ptr_eq(&spc, manager.get_space(3).unwrap()));
        dec.close_element(el).unwrap();
    }

    #[test]
    fn test_marshal_xml_encode_formatting() {
        // Exact text of a small formatted document (depth-based indenting:
        // "\n" + 2*depth spaces, capped at 25 chars of "\n<24 spaces>").
        let mut buf = Vec::new();
        {
            let mut enc = XmlEncode::new(&mut buf);
            enc.open_element(&ELEM_DATA);
            enc.write_bool(&ATTRIB_ALIGN, true);
            enc.open_element(&ELEM_INPUT);
            enc.close_element(&ELEM_INPUT);
            enc.close_element(&ELEM_DATA);
        }
        assert_eq!(
            String::from_utf8(buf).unwrap(),
            "\n<data align=\"true\">\n  <input/>\n</data>"
        );
    }

    #[test]
    fn test_marshal_packed_bufferpad() {
        // Transcription of the C++ TEST(marshal_bufferpad) setup to pin the
        // chunking/padding logic (1 + 511*2 + 1 = 1024 bytes exactly).
        assert_eq!(PackedDecode::BUFFER_SIZE, 1024);
        let manager = test_manager();
        let mut buf = Vec::new();
        {
            let mut enc = PackedEncode::new(&mut buf);
            enc.open_element(&ELEM_INPUT); // 1-byte
            for i in 0..511 {
                // 1022-bytes
                enc.write_bool(&ATTRIB_CONTENT, (i & 1) == 0);
            }
            enc.close_element(&ELEM_INPUT);
        }
        assert_eq!(buf.len(), 1024); // Encoding should exactly fill one buffer
        let mut dec = PackedDecode::new(&manager);
        dec.ingest_stream(&buf).unwrap();
        let el = dec.open_element_id(&ELEM_INPUT).unwrap();
        for i in 0..511 {
            let attrib_id = dec.get_next_attribute_id().unwrap();
            assert_eq!(attrib_id, ATTRIB_CONTENT.get_id());
            let val = dec.read_bool().unwrap();
            assert_eq!(val, (i & 1) == 0);
        }
        let nextel = dec.peek_element().unwrap();
        assert_eq!(nextel, 0);
        dec.close_element(el).unwrap();
    }

    #[test]
    fn test_marshal_packed_unexpected_eof() {
        // Truncated stream (mirrors test_unexpected_eof): drop the closing
        // element of the outer <data>.
        let manager = test_manager();
        let mut buf = Vec::new();
        {
            let mut enc = PackedEncode::new(&mut buf);
            enc.open_element(&ELEM_DATA);
            enc.open_element(&ELEM_INPUT);
            enc.write_string(&ATTRIB_NAME, b"hello");
            enc.close_element(&ELEM_INPUT);
        }
        let mut dec = PackedDecode::new(&manager);
        dec.ingest_stream(&buf).unwrap();
        let el1 = dec.open_element_id(&ELEM_DATA).unwrap();
        let el2 = dec.open_element_id(&ELEM_INPUT).unwrap();
        dec.close_element(el2).unwrap();
        let err = dec.close_element(el1).unwrap_err();
        assert!(matches!(err, KunaError::Decoder { .. }));
    }

    #[test]
    fn test_marshal_cxx_integer_parsing() {
        // istringstream-with-unsetf semantics
        assert_eq!(cxx_parse_unsigned(b"0x1000"), 0x1000);
        assert_eq!(cxx_parse_unsigned(b"1000"), 1000);
        assert_eq!(cxx_parse_unsigned(b"010"), 8); // octal
        assert_eq!(cxx_parse_unsigned(b"  42"), 42);
        assert_eq!(cxx_parse_unsigned(b"-5"), 5u64.wrapping_neg());
        assert_eq!(cxx_parse_unsigned(b"0xffffffffffffffff"), u64::MAX);
        assert_eq!(cxx_parse_unsigned(b""), 0);
        assert_eq!(cxx_parse_unsigned(b"junk"), 0);
        assert_eq!(cxx_parse_signed(b"-0x100"), -0x100);
        assert_eq!(cxx_parse_signed(b"0x7fffffffffffffff"), i64::MAX);
        assert_eq!(cxx_parse_signed(b"-9223372036854775808"), i64::MIN);
        assert_eq!(cxx_parse_signed(b"123abc"), 123);
        assert_eq!(cxx_strtoul(b"0x1000:2"), (0x1000, 6));
        assert_eq!(cxx_strtoul(b"33+1"), (33, 2));
    }

    #[test]
    fn test_marshal_cxx_num_get_u32_dec() {
        // `istringstream >> dec >> uint4` semantics (getIndexedAttributeId
        // suffix parse); decimal cases oracle-verified against g++/libstdc++.
        assert_eq!(cxx_num_get_u32_dec(b"1"), 1);
        assert_eq!(cxx_num_get_u32_dec(b"23"), 23);
        assert_eq!(cxx_num_get_u32_dec(b"010"), 10); // dec, NOT octal
        assert_eq!(cxx_num_get_u32_dec(b"08"), 8); // dec, NOT an octal error
        assert_eq!(cxx_num_get_u32_dec(b"0x2"), 0); // dec, NOT hex: stops at 'x'
        assert_eq!(cxx_num_get_u32_dec(b""), 0); // failed extraction stores 0
        assert_eq!(cxx_num_get_u32_dec(b"abc"), 0);
        assert_eq!(cxx_num_get_u32_dec(b"+7"), 7);
        assert_eq!(cxx_num_get_u32_dec(b"  12"), 12); // sentry skips whitespace
        assert_eq!(cxx_num_get_u32_dec(b"-"), 0); // sign with no digits fails
        // Overflow of the uint4 destination stores UINT_MAX (C++11 num_get)
        assert_eq!(cxx_num_get_u32_dec(b"4294967296"), u32::MAX);
        assert_eq!(cxx_num_get_u32_dec(b"4294967295"), u32::MAX); // exact max, no overflow
        // A minus sign negates the in-range magnitude modularly...
        assert_eq!(cxx_num_get_u32_dec(b"-5"), 5u32.wrapping_neg());
        assert_eq!(cxx_num_get_u32_dec(b"-4294967295"), 1);
        // ...but an overflowing negative field still stores UINT_MAX
        // (unsigned destination: num_get only stores __min for signed types)
        assert_eq!(cxx_num_get_u32_dec(b"-4294967296"), u32::MAX);
    }
}
