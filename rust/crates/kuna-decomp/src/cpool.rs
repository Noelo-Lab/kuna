//! Port of `decompiler/cpp/cpool.{hh,cc}` — the constant pool for \e deferred
//! compilation languages (i.e. java byte-code).
//!
//! Byte-code languages refer to system objects (string literals, class
//! methods, fields, ...) through encoded references rather than embedding them
//! in instructions.  A [`CPoolRecord`] describes one such object; a
//! [`ConstantPool`] maps a \e reference (1+ identifying integers) to its
//! record.  [`ConstantPoolInternal`] is the in-RAM implementation, keying a
//! `BTreeMap` on the lexicographically-ordered [`CheapSorter`].
//!
//! Faithful structural transcription of the C++:
//!   - the `tag`/`flags`/`token`/`value`/`type`/`byteData`/`byteDataLen`
//!     record fields and their `getTag`/`getToken`/... accessors;
//!   - the `encode`/`decode` round-trip, including the `<cpoolrec>` tag-string
//!     mapping, the `<value>`/`<data>`/`<token>` child handling, the
//!     `is_constructor`/`is_destructor` flag bits, and the `MAX_STRING_SIZE`
//!     bound check;
//!   - `ConstantPoolInternal::createRecord`'s duplicate-reference error, the
//!     `getRecord` lookup, and the container `encode`/`decode` ordering
//!     (records are emitted in `CheapSorter` order, each as a `<ref>` followed
//!     by a `<cpoolrec>`).
//!
//! SEAM(W6): the data-type half of the record (`Datatype *type`,
//! `type->encodeRef`, `typegrp.decodeType`/`decodeTypeWithCodeFlags`) belongs
//! to the W6 `type.{hh,cc}` / `TypeFactory` port.  Until then the record holds
//! an [`Rc<Datatype>`](crate::dtype::Datatype) (the W6 skeleton, exactly where
//! the C++ holds `Datatype *`) and the encode/decode of the type reference goes
//! through the local [`CPoolTypeFactory`] / [`CPoolTypeRef`] seam traits; W6
//! supplies an impl of these over the real `TypeFactory`.

use std::collections::BTreeMap;
use std::rc::Rc;

use kuna_base::error::{KunaError, KunaResult};
use kuna_base::marshal::{
    Decoder, Encoder, ATTRIB_CONSTRUCTOR, ATTRIB_CONTENT, ATTRIB_DESTRUCTOR, ELEM_DATA, ELEM_VALUE,
};
use kuna_base::types::{int4, int8, uint1, uint4, uintb};

use crate::dtype::Datatype;

// ---------------------------------------------------------------------------
// AttributeId / ElementId  (C++ `cpool.cc:20-28`)
// ---------------------------------------------------------------------------

use kuna_base::marshal::{AttributeId, ElementId};

/// Marshaling attribute "a" (C++ `ATTRIB_A`, `cpool.cc:20`).
pub const ATTRIB_A: AttributeId = AttributeId::new("a", 80);
/// Marshaling attribute "b" (C++ `ATTRIB_B`, `cpool.cc:21`).
pub const ATTRIB_B: AttributeId = AttributeId::new("b", 81);
/// Marshaling attribute "length" (C++ `ATTRIB_LENGTH`, `cpool.cc:22`).
pub const ATTRIB_LENGTH: AttributeId = AttributeId::new("length", 82);
/// Marshaling attribute "tag" (C++ `ATTRIB_TAG`, `cpool.cc:23`).
pub const ATTRIB_TAG: AttributeId = AttributeId::new("tag", 83);

/// Marshaling element \<constantpool> (C++ `ELEM_CONSTANTPOOL`, `cpool.cc:25`).
pub const ELEM_CONSTANTPOOL: ElementId = ElementId::new("constantpool", 109);
/// Marshaling element \<cpoolrec> (C++ `ELEM_CPOOLREC`, `cpool.cc:26`).
pub const ELEM_CPOOLREC: ElementId = ElementId::new("cpoolrec", 110);
/// Marshaling element \<ref> (C++ `ELEM_REF`, `cpool.cc:27`).
pub const ELEM_REF: ElementId = ElementId::new("ref", 111);
/// Marshaling element \<token> (C++ `ELEM_TOKEN`, `cpool.cc:28`).
pub const ELEM_TOKEN: ElementId = ElementId::new("token", 112);

// ---------------------------------------------------------------------------
// SEAM(W6): the TypeFactory / Datatype-reference operations cpool reaches.
// ---------------------------------------------------------------------------

/// SEAM(W6): the slice of a `Datatype` reference cpool needs to *encode*
/// (`type->encodeRef(encoder)`).
///
/// W6's real `Datatype` implements this over its `encodeRef`; until then the
/// record carries an [`Rc<Datatype>`](crate::dtype::Datatype) and the encode
/// path is the W6 seam.  The [`crate::dtype::Datatype`] skeleton itself only
/// carries `size`/`metatype`/`id`, not enough to reproduce `encodeRef`, so the
/// encode is parameterized on a borrowed `&dyn CPoolTypeRef` rather than
/// reaching into the skeleton.
pub trait CPoolTypeRef {
    /// Encode a *reference* to this data-type (C++ `Datatype::encodeRef`).
    fn encode_ref(&self, encoder: &mut dyn Encoder) -> KunaResult<()>;
}

/// SEAM(W6): the slice of `TypeFactory` cpool needs to *decode* a data-type
/// reference (`typegrp.decodeType` / `typegrp.decodeTypeWithCodeFlags`).
///
/// W6's real `TypeFactory` implements this; the W4/W6 caller supplies it to
/// [`CPoolRecord::decode`] / [`ConstantPool::decode_record`].
pub trait CPoolTypeFactory {
    /// Decode a data-type reference (C++ `TypeFactory::decodeType`).
    fn decode_type(&mut self, decoder: &mut dyn Decoder) -> KunaResult<Rc<Datatype>>;

    /// Decode a data-type reference, marking it as code with the given
    /// constructor/destructor flags (C++ `TypeFactory::decodeTypeWithCodeFlags`).
    fn decode_type_with_code_flags(
        &mut self,
        decoder: &mut dyn Decoder,
        is_constructor: bool,
        is_destructor: bool,
    ) -> KunaResult<Rc<Datatype>>;
}

// ---------------------------------------------------------------------------
// CPoolRecord  (C++ `cpool.hh:56`, `cpool.cc:30-158`)
// ---------------------------------------------------------------------------

/// Generic constant pool tag types (C++ `CPoolRecord` anonymous enum,
/// `cpool.hh:59-68`).
pub mod cpool_tag {
    use kuna_base::types::uint4;

    /// Constant \b value of data-type \b type; cpool operator can be eliminated.
    pub const PRIMITIVE: uint4 = 0;
    /// Constant reference to string (passed back as \b byteData).
    pub const STRING_LITERAL: uint4 = 1;
    /// Reference to (system level) class object; \b token holds class name.
    pub const CLASS_REFERENCE: uint4 = 2;
    /// Pointer to a method; name in \b token, signature in \b type.
    pub const POINTER_METHOD: uint4 = 3;
    /// Pointer to a field; name in \b token, data-type in \b type.
    pub const POINTER_FIELD: uint4 = 4;
    /// Integer length; \b token is language-specific indicator, \b type integral.
    pub const ARRAY_LENGTH: uint4 = 5;
    /// Boolean value; \b token is language-specific indicator, \b type boolean.
    pub const INSTANCE_OF: uint4 = 6;
    /// Pointer to object; new name in \b token, new data-type in \b type.
    pub const CHECK_CAST: uint4 = 7;
}

/// Additional boolean properties on the record (C++ `CPoolRecord` flag enum,
/// `cpool.hh:69-72`).
pub mod cpool_flag {
    use kuna_base::types::uint4;

    /// Referenced method is a constructor.
    pub const IS_CONSTRUCTOR: uint4 = 0x1;
    /// Referenced method is a destructor.
    pub const IS_DESTRUCTOR: uint4 = 0x2;
}

/// Maximum allowed bytes of string in pool (C++
/// `CPoolRecord::MAX_STRING_SIZE`, `cpool.hh:82`).
const MAX_STRING_SIZE: int8 = 0x100000;

/// A description of a byte-code object referenced by a constant
/// (C++ `CPoolRecord`).
#[derive(Debug, Clone, Default)]
pub struct CPoolRecord {
    /// Descriptor of type of the object (C++ `tag`).
    tag: uint4,
    /// Additional boolean properties on the record (C++ `flags`).
    flags: uint4,
    /// Name or token associated with the object (C++ `token`).
    ///
    /// Byte string, following the marshal-layer decision (C++ `std::string`
    /// payloads need not be valid UTF-8 and must round-trip byte-for-byte).
    token: Vec<u8>,
    /// Constant value of the object, if known (C++ `value`).
    value: uintb,
    /// Data-type associated with the object (C++ `Datatype *type`).
    ///
    /// SEAM(W6): `None` == the C++ null `type` pointer.  An empty (just
    /// `createRecord`'d) record has no type until `decode`/`putRecord` sets it.
    type_: Option<Rc<Datatype>>,
    /// For string literals, the raw byte data of the string (C++ `byteData`).
    ///
    /// `None` == the C++ null `byteData` pointer.  `byteDataLen` is `data.len()`.
    byte_data: Option<Vec<uint1>>,
}

impl CPoolRecord {
    /// Construct an empty record (C++ `CPoolRecord(void)`).
    pub fn new() -> CPoolRecord {
        CPoolRecord::default()
    }

    /// Get the type of record (C++ `getTag`).
    pub fn get_tag(&self) -> uint4 {
        self.tag
    }

    /// Get name of method or data-type (C++ `getToken`).
    pub fn get_token(&self) -> &[u8] {
        &self.token
    }

    /// Get pointer to string literal data (C++ `getByteData`).
    ///
    /// `None` == the C++ null `byteData` pointer.
    pub fn get_byte_data(&self) -> Option<&[uint1]> {
        self.byte_data.as_deref()
    }

    /// Number of bytes of string literal data (C++ `getByteDataLength`).
    ///
    /// `byteDataLen` is undefined in C++ when `byteData` is null, but callers
    /// only read it after checking `getByteData`; we report the actual length.
    pub fn get_byte_data_length(&self) -> int4 {
        // C++ stores byteDataLen as int4; the decode bound-checks against
        // MAX_STRING_SIZE so the length always fits in int4.
        match &self.byte_data {
            Some(d) => d.len() as int4,
            None => 0,
        }
    }

    /// Get the data-type associated with \b this (C++ `getType`).
    ///
    /// SEAM(W6): `None` == the C++ null `type` pointer.
    pub fn get_type(&self) -> Option<&Rc<Datatype>> {
        self.type_.as_ref()
    }

    /// Get the constant value associated with \b this (C++ `getValue`).
    pub fn get_value(&self) -> uintb {
        self.value
    }

    /// Is object a constructor method (C++ `isConstructor`).
    pub fn is_constructor(&self) -> bool {
        (self.flags & cpool_flag::IS_CONSTRUCTOR) != 0
    }

    /// Is object a destructor method (C++ `isDestructor`).
    pub fn is_destructor(&self) -> bool {
        (self.flags & cpool_flag::IS_DESTRUCTOR) != 0
    }

    /// Encode \b this CPoolRecord description as a \<cpoolrec> element
    /// (C++ `CPoolRecord::encode`).
    ///
    /// SEAM(W6): the trailing `type->encodeRef(encoder)` goes through the
    /// `type_ref` argument (the W6 `Datatype` reference encoder).  The C++
    /// dereferences `type` unconditionally here, so an encode with no type is
    /// an internal-invariant violation.
    pub fn encode(
        &self,
        encoder: &mut dyn Encoder,
        type_ref: &dyn CPoolTypeRef,
    ) -> KunaResult<()> {
        encoder.open_element(&ELEM_CPOOLREC);
        // The tag-string mapping (C++ `cpool.cc:36-51`).
        let tagstr: &[u8] = if self.tag == cpool_tag::POINTER_METHOD {
            b"method"
        } else if self.tag == cpool_tag::POINTER_FIELD {
            b"field"
        } else if self.tag == cpool_tag::INSTANCE_OF {
            b"instanceof"
        } else if self.tag == cpool_tag::ARRAY_LENGTH {
            b"arraylength"
        } else if self.tag == cpool_tag::CHECK_CAST {
            b"checkcast"
        } else if self.tag == cpool_tag::STRING_LITERAL {
            b"string"
        } else if self.tag == cpool_tag::CLASS_REFERENCE {
            b"classref"
        } else {
            b"primitive"
        };
        encoder.write_string(&ATTRIB_TAG, tagstr);
        if self.is_constructor() {
            encoder.write_bool(&ATTRIB_CONSTRUCTOR, true);
        }
        if self.is_destructor() {
            encoder.write_bool(&ATTRIB_DESTRUCTOR, true);
        }
        if self.tag == cpool_tag::PRIMITIVE {
            encoder.open_element(&ELEM_VALUE);
            encoder.write_unsigned_integer(&ATTRIB_CONTENT, self.value);
            encoder.close_element(&ELEM_VALUE);
        }
        if let Some(data) = &self.byte_data {
            encoder.open_element(&ELEM_DATA);
            // C++ `byteDataLen` is int4 here.
            encoder.write_signed_integer(&ATTRIB_LENGTH, data.len() as int8);
            // Reproduce the C++ ostringstream byte dump: two-digit hex, lower
            // case, space-separated, wrapping a newline after 16 bytes.
            let mut s: Vec<u8> = Vec::new();
            let mut wrap: int4 = 0;
            for &b in data.iter() {
                s.extend_from_slice(format!("{:02x}", b).as_bytes());
                s.push(b' ');
                wrap += 1;
                if wrap > 15 {
                    s.push(b'\n');
                    wrap = 0;
                }
            }
            encoder.write_string(&ATTRIB_CONTENT, &s);
            encoder.close_element(&ELEM_DATA);
        } else {
            encoder.open_element(&ELEM_TOKEN);
            encoder.write_string(&ATTRIB_CONTENT, &self.token);
            encoder.close_element(&ELEM_TOKEN);
        }
        type_ref.encode_ref(encoder)?;
        encoder.close_element(&ELEM_CPOOLREC);
        Ok(())
    }

    /// Initialize \b this CPoolRecord instance from a \<cpoolrec> element
    /// (C++ `CPoolRecord::decode`).
    ///
    /// SEAM(W6): the trailing data-type resolution goes through `typegrp`
    /// (the W6 `TypeFactory`).
    pub fn decode(
        &mut self,
        decoder: &mut dyn Decoder,
        typegrp: &mut dyn CPoolTypeFactory,
    ) -> KunaResult<()> {
        self.tag = cpool_tag::PRIMITIVE; // Default tag
        self.value = 0;
        self.flags = 0;
        let elem_id = decoder.open_element_id(&ELEM_CPOOLREC)?;
        loop {
            let attrib_id = decoder.get_next_attribute_id()?;
            if attrib_id == 0 {
                break;
            }
            if attrib_id == ATTRIB_TAG.get_id() {
                let tagstring = decoder.read_string()?;
                if tagstring == b"method" {
                    self.tag = cpool_tag::POINTER_METHOD;
                } else if tagstring == b"field" {
                    self.tag = cpool_tag::POINTER_FIELD;
                } else if tagstring == b"instanceof" {
                    self.tag = cpool_tag::INSTANCE_OF;
                } else if tagstring == b"arraylength" {
                    self.tag = cpool_tag::ARRAY_LENGTH;
                } else if tagstring == b"checkcast" {
                    self.tag = cpool_tag::CHECK_CAST;
                } else if tagstring == b"string" {
                    self.tag = cpool_tag::STRING_LITERAL;
                } else if tagstring == b"classref" {
                    self.tag = cpool_tag::CLASS_REFERENCE;
                }
            } else if attrib_id == ATTRIB_CONSTRUCTOR.get_id() {
                if decoder.read_bool()? {
                    self.flags |= cpool_flag::IS_CONSTRUCTOR;
                }
            } else if attrib_id == ATTRIB_DESTRUCTOR.get_id() && decoder.read_bool()? {
                self.flags |= cpool_flag::IS_DESTRUCTOR;
            }
        }
        if self.tag == cpool_tag::PRIMITIVE {
            // First tag must be value
            let sub_id = decoder.open_element_id(&ELEM_VALUE)?;
            self.value = decoder.read_unsigned_integer_id(&ATTRIB_CONTENT)?;
            decoder.close_element(sub_id)?;
        }
        let sub_id = decoder.open_element()?;
        if sub_id == ELEM_TOKEN.get_id() {
            self.token = decoder.read_string_id(&ATTRIB_CONTENT)?;
        } else {
            let val: int8 = decoder.read_signed_integer_id(&ATTRIB_LENGTH)?;
            // Faithful transcription of the C++ `if (val < 0 || val >= MAX_STRING_SIZE)`.
            #[allow(clippy::manual_range_contains)]
            if val < 0 || val >= MAX_STRING_SIZE {
                return Err(KunaError::lowlevel(
                    "Bad constant pool record: bad <data> size",
                ));
            }
            let byte_data_len = val; // int4 in C++; bounded by MAX_STRING_SIZE
            // Reproduce the C++ `istringstream >> ws >> hex >> val` loop: read
            // `byteDataLen` whitespace-separated hex bytes from the content.
            let content = decoder.read_string_id(&ATTRIB_CONTENT)?;
            let mut data: Vec<uint1> = Vec::with_capacity(byte_data_len as usize);
            let mut toks = HexByteTokens::new(&content);
            for _ in 0..byte_data_len {
                // C++ reads a uint4 then narrows to uint1 (`byteData[i] = (uint1)val`).
                let v: uint4 = toks.next_hex_u32();
                data.push(v as uint1);
            }
            self.byte_data = Some(data);
        }
        decoder.close_element(sub_id)?;
        if self.tag == cpool_tag::STRING_LITERAL && self.byte_data.is_none() {
            return Err(KunaError::lowlevel(
                "Bad constant pool record: missing <data>",
            ));
        }
        if self.flags != 0 {
            let is_constructor = (self.flags & cpool_flag::IS_CONSTRUCTOR) != 0;
            let is_destructor = (self.flags & cpool_flag::IS_DESTRUCTOR) != 0;
            self.type_ =
                Some(typegrp.decode_type_with_code_flags(decoder, is_constructor, is_destructor)?);
        } else {
            self.type_ = Some(typegrp.decode_type(decoder)?);
        }
        decoder.close_element(elem_id)?;
        Ok(())
    }
}

/// Iterator over whitespace-separated hex byte tokens, reproducing the C++
/// `istringstream >> ws >> hex >> val` extraction used by `CPoolRecord::decode`.
///
/// C++'s `>>` with `std::hex` skips leading whitespace and reads the longest
/// run of hex digits (optionally with a `0x` prefix and a sign); a failed
/// extraction leaves `val` at 0 (since C++11, on failure `val` is set to 0).
/// The decode loop never reads past `byteDataLen`, so an exhausted stream
/// produces 0 bytes, matching the C++ failbit behavior.
struct HexByteTokens<'a> {
    buf: &'a [u8],
    pos: usize,
}

impl<'a> HexByteTokens<'a> {
    fn new(buf: &'a [u8]) -> HexByteTokens<'a> {
        HexByteTokens { buf, pos: 0 }
    }

    /// Read the next whitespace-delimited hex token as a `uint4`
    /// (C++ `s3 >> ws >> hex >> val`).  Returns 0 on a failed extraction, like
    /// the post-C++11 stream behavior.
    fn next_hex_u32(&mut self) -> uint4 {
        // Skip leading whitespace (`>> ws`).
        while self.pos < self.buf.len() && self.buf[self.pos].is_ascii_whitespace() {
            self.pos += 1;
        }
        let mut val: uint4 = 0;
        let mut any = false;
        // Optional "0x"/"0X" prefix is consumed by std::hex extraction.
        if self.pos + 1 < self.buf.len()
            && self.buf[self.pos] == b'0'
            && (self.buf[self.pos + 1] == b'x' || self.buf[self.pos + 1] == b'X')
        {
            // A bare "0x" with no following digit reads the leading '0'.
            // Mirror libstdc++: consume the '0', leave 'x' for the next field.
            // (The encode path never emits a prefix, so this branch is only
            // reached by hand-authored XML.)
            self.pos += 1;
            any = true;
        }
        while self.pos < self.buf.len() {
            let c = self.buf[self.pos];
            let digit = match c {
                b'0'..=b'9' => (c - b'0') as uint4,
                b'a'..=b'f' => (c - b'a' + 10) as uint4,
                b'A'..=b'F' => (c - b'A' + 10) as uint4,
                _ => break,
            };
            // C++ `>>` saturates to the max on overflow and sets failbit; the
            // decode loop only feeds two-digit bytes, so plain wrapping mul is
            // faithful for the data this path actually sees.
            val = val.wrapping_mul(16).wrapping_add(digit);
            any = true;
            self.pos += 1;
        }
        if any {
            val
        } else {
            0
        }
    }
}

// ---------------------------------------------------------------------------
// ConstantPool  (C++ `cpool.hh:105`, `cpool.cc:160-175`)
// ---------------------------------------------------------------------------

/// An interface to the pool of \b constant objects for byte-code languages
/// (C++ abstract `ConstantPool`).
///
/// A \e reference (1 or more integer constants) maps to an individual
/// [`CPoolRecord`].  Implementors provide `create_record` (allocate a record
/// for a reference), `get_record`, `empty`, `clear`, `encode`, and `decode`;
/// the [`Self::put_record`] / [`Self::decode_record`] helpers are provided over
/// `create_record`, mirroring the non-virtual C++ methods.
pub trait ConstantPool {
    /// Allocate a new [`CPoolRecord`], given a \e reference to it
    /// (C++ pure-virtual `createRecord`).
    ///
    /// The object will still need to be initialized but is already associated
    /// with the reference.  Any issue with allocation (e.g. a duplicate
    /// reference) returns an error.  Returns a mutable handle to the new record.
    fn create_record(&mut self, refs: &[uintb]) -> KunaResult<&mut CPoolRecord>;

    /// Retrieve a constant pool record given a \e reference to it
    /// (C++ pure-virtual `getRecord`).  `None` if no record matches.
    fn get_record(&self, refs: &[uintb]) -> Option<&CPoolRecord>;

    /// Is the container empty of records (C++ pure-virtual `empty`).
    fn empty(&self) -> bool;

    /// Release any (local) resources (C++ pure-virtual `clear`).
    fn clear(&mut self);

    /// Encode all records in this container to a stream
    /// (C++ pure-virtual `encode`).
    ///
    /// SEAM(W6): the per-record `type->encodeRef` goes through `type_provider`,
    /// which resolves a record's [`Datatype`] to its W6 reference encoder.
    fn encode(
        &self,
        encoder: &mut dyn Encoder,
        type_provider: &dyn CPoolTypeRefProvider,
    ) -> KunaResult<()>;

    /// Restore constant pool records from the given stream decoder
    /// (C++ pure-virtual `decode`).
    fn decode(
        &mut self,
        decoder: &mut dyn Decoder,
        typegrp: &mut dyn CPoolTypeFactory,
    ) -> KunaResult<()>;

    /// Add a new constant pool record to \b this database
    /// (C++ non-virtual `ConstantPool::putRecord`).
    ///
    /// Given the basic constituents of the record — tag, name, and data-type —
    /// create a new [`CPoolRecord`] and associate it with the given reference.
    fn put_record(
        &mut self,
        refs: &[uintb],
        tag: uint4,
        tok: &[u8],
        ct: Rc<Datatype>,
    ) -> KunaResult<()> {
        let newrec = self.create_record(refs)?;
        newrec.tag = tag;
        newrec.token = tok.to_vec();
        newrec.type_ = Some(ct);
        Ok(())
    }

    /// Restore a [`CPoolRecord`] given a \e reference and a stream decoder
    /// (C++ non-virtual `ConstantPool::decodeRecord`).
    ///
    /// A \<cpoolrec> element initializes the new record which is immediately
    /// associated with the \e reference.
    fn decode_record(
        &mut self,
        refs: &[uintb],
        decoder: &mut dyn Decoder,
        typegrp: &mut dyn CPoolTypeFactory,
    ) -> KunaResult<()> {
        // C++ returns the new record pointer; we return Ok(()) and leave the
        // record reachable via get_record (the only borrow-safe shape here).
        let mut newrec = CPoolRecord::new();
        newrec.decode(decoder, typegrp)?;
        let slot = self.create_record(refs)?;
        *slot = newrec;
        Ok(())
    }
}

/// SEAM(W6): resolves a [`CPoolRecord`]'s [`Datatype`] to its reference encoder
/// for the container `encode` path.
///
/// The C++ calls `record.type->encodeRef(encoder)` directly; with the W6
/// `Datatype` skeleton that has no `encodeRef`, the container `encode` asks the
/// provider for a `&dyn CPoolTypeRef` per record.  W6 supplies a provider that
/// returns the `Datatype` itself.
pub trait CPoolTypeRefProvider {
    /// Encode a reference to the given record's data-type
    /// (C++ `record.type->encodeRef(encoder)`).
    fn encode_record_type_ref(
        &self,
        record: &CPoolRecord,
        encoder: &mut dyn Encoder,
    ) -> KunaResult<()>;
}

// ---------------------------------------------------------------------------
// ConstantPoolInternal  (C++ `cpool.hh:166`, `cpool.cc:177-246`)
// ---------------------------------------------------------------------------

/// A cheap (efficient) placeholder for a \e reference to a constant pool record
/// (C++ `ConstantPoolInternal::CheapSorter`).
///
/// A \b reference can be an open-ended number of (1 or more) integers; in
/// practice the most we see is two.  `a` is the first integer, `b` the second
/// (or zero if absent).  References are ordered lexicographically — the derived
/// `Ord`/`PartialOrd` here reproduce C++ `operator<` exactly (`a` first, then
/// `b`), which is the `BTreeMap` key comparator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct CheapSorter {
    /// The first integer in a \e reference (C++ `a`).
    pub a: uintb,
    /// The second integer in a \e reference, or zero (C++ `b`).
    pub b: uintb,
}

impl CheapSorter {
    /// Construct a zero reference (C++ `CheapSorter(void)`).
    pub fn new() -> CheapSorter {
        CheapSorter { a: 0, b: 0 }
    }

    /// Construct from an array of integers (C++ `CheapSorter(const vector<uintb>&)`).
    ///
    /// The C++ reads `refs[0]` unconditionally (so an empty reference is UB);
    /// we treat an empty slice as the internal-invariant violation it is.
    pub fn from_refs(refs: &[uintb]) -> CheapSorter {
        let a = refs[0];
        let b = if refs.len() > 1 { refs[1] } else { 0 };
        CheapSorter { a, b }
    }

    /// Convert the reference back to a formal array of integers
    /// (C++ `CheapSorter::apply`).  Pushes both `a` and `b`.
    pub fn apply(&self, refs: &mut Vec<uintb>) {
        refs.push(self.a);
        refs.push(self.b);
    }

    /// Encode the \e reference to a stream as a \<ref> element
    /// (C++ `CheapSorter::encode`).
    pub fn encode(&self, encoder: &mut dyn Encoder) {
        encoder.open_element(&ELEM_REF);
        encoder.write_unsigned_integer(&ATTRIB_A, self.a);
        encoder.write_unsigned_integer(&ATTRIB_B, self.b);
        encoder.close_element(&ELEM_REF);
    }

    /// Restore \b this \e reference from a \<ref> element
    /// (C++ `CheapSorter::decode`).
    pub fn decode(&mut self, decoder: &mut dyn Decoder) -> KunaResult<()> {
        let elem_id = decoder.open_element_id(&ELEM_REF)?;
        self.a = decoder.read_unsigned_integer_id(&ATTRIB_A)?;
        self.b = decoder.read_unsigned_integer_id(&ATTRIB_B)?;
        decoder.close_element(elem_id)?;
        Ok(())
    }
}

/// An implementation of [`ConstantPool`] storing records internally in RAM
/// (C++ `ConstantPoolInternal`).
///
/// The [`CPoolRecord`] objects are held directly in a [`BTreeMap`] keyed by
/// [`CheapSorter`] (the C++ `map<CheapSorter,CPoolRecord>`).  This can be a
/// stand-alone constant pool or a local cache for some other implementation.
#[derive(Debug, Default)]
pub struct ConstantPoolInternal {
    /// A map from \e reference to constant pool record (C++ `cpoolMap`).
    cpool_map: BTreeMap<CheapSorter, CPoolRecord>,
}

impl ConstantPoolInternal {
    /// Construct an empty in-RAM constant pool.
    pub fn new() -> ConstantPoolInternal {
        ConstantPoolInternal::default()
    }
}

impl ConstantPool for ConstantPoolInternal {
    fn create_record(&mut self, refs: &[uintb]) -> KunaResult<&mut CPoolRecord> {
        let sorter = CheapSorter::from_refs(refs);
        // C++ `cpoolMap.emplace(...)` returns (iterator, inserted); a duplicate
        // reference is an error reporting the existing record's token.
        if let Some(existing) = self.cpool_map.get(&sorter) {
            let mut msg =
                String::from("Creating duplicate entry in constant pool: ");
            msg.push_str(&String::from_utf8_lossy(existing.get_token()));
            return Err(KunaError::lowlevel(msg));
        }
        self.cpool_map.insert(sorter, CPoolRecord::new());
        Ok(self
            .cpool_map
            .get_mut(&sorter)
            .expect("createRecord: just-inserted entry missing (internal invariant)"))
    }

    fn get_record(&self, refs: &[uintb]) -> Option<&CPoolRecord> {
        let sorter = CheapSorter::from_refs(refs);
        self.cpool_map.get(&sorter)
    }

    fn empty(&self) -> bool {
        self.cpool_map.is_empty()
    }

    fn clear(&mut self) {
        self.cpool_map.clear();
    }

    fn encode(
        &self,
        encoder: &mut dyn Encoder,
        type_provider: &dyn CPoolTypeRefProvider,
    ) -> KunaResult<()> {
        encoder.open_element(&ELEM_CONSTANTPOOL);
        // BTreeMap iterates in CheapSorter (lexicographic) order, matching the
        // C++ `map` iteration.  Each entry emits its <ref> then its <cpoolrec>.
        for (sorter, record) in self.cpool_map.iter() {
            sorter.encode(encoder);
            // The C++ calls `record.encode(encoder)`, which ends in
            // `type->encodeRef(encoder)`.  With the W6 type skeleton lacking
            // `encodeRef`, route the record's type-ref through the provider via
            // a per-record adaptor (SEAM(W6)).
            let adaptor = ProviderTypeRef {
                provider: type_provider,
                record,
            };
            record.encode(encoder, &adaptor)?;
        }
        encoder.close_element(&ELEM_CONSTANTPOOL);
        Ok(())
    }

    fn decode(
        &mut self,
        decoder: &mut dyn Decoder,
        typegrp: &mut dyn CPoolTypeFactory,
    ) -> KunaResult<()> {
        let elem_id = decoder.open_element_id(&ELEM_CONSTANTPOOL)?;
        while decoder.peek_element()? != 0 {
            let mut sorter = CheapSorter::new();
            sorter.decode(decoder)?;
            let mut refs: Vec<uintb> = Vec::new();
            sorter.apply(&mut refs);
            self.decode_record(&refs, decoder, typegrp)?;
        }
        decoder.close_element(elem_id)?;
        Ok(())
    }
}

/// Per-record adaptor that satisfies [`CPoolTypeRef`] (needed by
/// [`CPoolRecord::encode`]) by delegating to a [`CPoolTypeRefProvider`].
///
/// SEAM(W6): bridges the C++ `record.type->encodeRef(encoder)` call — which
/// the W6 `Datatype` skeleton cannot do directly — to the provider supplied to
/// the container `encode`.
struct ProviderTypeRef<'a> {
    provider: &'a dyn CPoolTypeRefProvider,
    record: &'a CPoolRecord,
}

impl CPoolTypeRef for ProviderTypeRef<'_> {
    fn encode_ref(&self, encoder: &mut dyn Encoder) -> KunaResult<()> {
        self.provider.encode_record_type_ref(self.record, encoder)
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use kuna_base::marshal::{IdRegistry, XmlDecode, XmlEncode};
    use kuna_base::space::AddrSpaceManager;
    use crate::dtype::type_metatype;

    /// A trivial type factory/provider seam impl for round-trip tests: every
    /// data-type reference encodes as an empty `<void/>` element and decodes
    /// back to a fixed skeleton.  This lets the cpool encode/decode round-trip
    /// be exercised without the W6 `TypeFactory`.
    struct VoidTypeSeam;

    fn void_type() -> Rc<Datatype> {
        Rc::new(Datatype::new(0, type_metatype::TYPE_VOID))
    }

    impl CPoolTypeRef for VoidTypeSeam {
        fn encode_ref(&self, encoder: &mut dyn Encoder) -> KunaResult<()> {
            encoder.open_element(&kuna_base::marshal::ELEM_VOID);
            encoder.close_element(&kuna_base::marshal::ELEM_VOID);
            Ok(())
        }
    }

    impl CPoolTypeRefProvider for VoidTypeSeam {
        fn encode_record_type_ref(
            &self,
            _record: &CPoolRecord,
            encoder: &mut dyn Encoder,
        ) -> KunaResult<()> {
            self.encode_ref(encoder)
        }
    }

    impl CPoolTypeFactory for VoidTypeSeam {
        fn decode_type(&mut self, decoder: &mut dyn Decoder) -> KunaResult<Rc<Datatype>> {
            let id = decoder.open_element()?;
            decoder.close_element(id)?;
            Ok(void_type())
        }

        fn decode_type_with_code_flags(
            &mut self,
            decoder: &mut dyn Decoder,
            _is_constructor: bool,
            _is_destructor: bool,
        ) -> KunaResult<Rc<Datatype>> {
            let id = decoder.open_element()?;
            decoder.close_element(id)?;
            Ok(void_type())
        }
    }

    fn registry() -> IdRegistry {
        let mut reg = IdRegistry::with_base_ids();
        // cpool ids
        for a in [&ATTRIB_A, &ATTRIB_B, &ATTRIB_LENGTH, &ATTRIB_TAG] {
            reg.register_attribute(a);
        }
        for e in [&ELEM_CONSTANTPOOL, &ELEM_CPOOLREC, &ELEM_REF, &ELEM_TOKEN] {
            reg.register_element(e);
        }
        reg
    }

    fn roundtrip(pool: &ConstantPoolInternal) -> ConstantPoolInternal {
        let mgr = AddrSpaceManager::new();
        let reg = registry();
        let mut buf: Vec<u8> = Vec::new();
        {
            let mut enc = XmlEncode::new(&mut buf);
            pool.encode(&mut enc, &VoidTypeSeam).unwrap();
        }
        let mut dec = XmlDecode::new(&mgr, &reg);
        dec.ingest_stream(&buf).unwrap();
        let mut out = ConstantPoolInternal::new();
        out.decode(&mut dec, &mut VoidTypeSeam).unwrap();
        out
    }

    #[test]
    fn cheapsorter_lexicographic_order() {
        let s1 = CheapSorter { a: 1, b: 5 };
        let s2 = CheapSorter { a: 1, b: 9 };
        let s3 = CheapSorter { a: 2, b: 0 };
        assert!(s1 < s2);
        assert!(s2 < s3);
        assert!(s1 < s3);
        // from_refs: single-element ref has b == 0
        assert_eq!(CheapSorter::from_refs(&[7]), CheapSorter { a: 7, b: 0 });
        assert_eq!(CheapSorter::from_refs(&[7, 3]), CheapSorter { a: 7, b: 3 });
    }

    #[test]
    fn put_and_get_record() {
        let mut pool = ConstantPoolInternal::new();
        assert!(pool.empty());
        pool.put_record(&[10], cpool_tag::CLASS_REFERENCE, b"MyClass", void_type())
            .unwrap();
        assert!(!pool.empty());
        let rec = pool.get_record(&[10]).unwrap();
        assert_eq!(rec.get_tag(), cpool_tag::CLASS_REFERENCE);
        assert_eq!(rec.get_token(), b"MyClass");
        assert!(pool.get_record(&[11]).is_none());
    }

    #[test]
    fn duplicate_record_is_error() {
        let mut pool = ConstantPoolInternal::new();
        pool.put_record(&[3, 4], cpool_tag::PRIMITIVE, b"first", void_type())
            .unwrap();
        let err = pool.put_record(&[3, 4], cpool_tag::PRIMITIVE, b"second", void_type());
        assert!(err.is_err());
        // The error message echoes the existing token.
        let msg = err.unwrap_err().to_string();
        assert!(msg.contains("first"), "msg was: {msg}");
    }

    #[test]
    fn roundtrip_primitive_record() {
        let mut pool = ConstantPoolInternal::new();
        {
            let rec = pool.create_record(&[42]).unwrap();
            rec.tag = cpool_tag::PRIMITIVE;
            rec.value = 0xdeadbeef;
            rec.token = b"prim".to_vec();
            rec.type_ = Some(void_type());
        }
        let out = roundtrip(&pool);
        let rec = out.get_record(&[42]).unwrap();
        assert_eq!(rec.get_tag(), cpool_tag::PRIMITIVE);
        assert_eq!(rec.get_value(), 0xdeadbeef);
        assert_eq!(rec.get_token(), b"prim");
    }

    #[test]
    fn roundtrip_classref_with_flags() {
        let mut pool = ConstantPoolInternal::new();
        {
            let rec = pool.create_record(&[1, 2]).unwrap();
            rec.tag = cpool_tag::POINTER_METHOD;
            rec.token = b"<init>".to_vec();
            rec.flags = cpool_flag::IS_CONSTRUCTOR;
            rec.type_ = Some(void_type());
        }
        let out = roundtrip(&pool);
        let rec = out.get_record(&[1, 2]).unwrap();
        assert_eq!(rec.get_tag(), cpool_tag::POINTER_METHOD);
        assert_eq!(rec.get_token(), b"<init>");
        assert!(rec.is_constructor());
        assert!(!rec.is_destructor());
    }

    #[test]
    fn roundtrip_string_literal_bytedata() {
        let mut pool = ConstantPoolInternal::new();
        let bytes: Vec<u8> = (0u16..20).map(|x| x as u8).collect();
        {
            let rec = pool.create_record(&[100]).unwrap();
            rec.tag = cpool_tag::STRING_LITERAL;
            rec.byte_data = Some(bytes.clone());
            rec.type_ = Some(void_type());
        }
        let out = roundtrip(&pool);
        let rec = out.get_record(&[100]).unwrap();
        assert_eq!(rec.get_tag(), cpool_tag::STRING_LITERAL);
        assert_eq!(rec.get_byte_data(), Some(bytes.as_slice()));
        assert_eq!(rec.get_byte_data_length(), 20);
    }

    #[test]
    fn roundtrip_container_order() {
        // Insert out of order; the encode/decode must preserve the sorted set.
        let mut pool = ConstantPoolInternal::new();
        for &(a, b) in &[(5u64, 0u64), (1, 9), (1, 2), (3, 0)] {
            let rec = pool.create_record(&[a, b]).unwrap();
            rec.tag = cpool_tag::CLASS_REFERENCE;
            rec.token = format!("ref_{a}_{b}").into_bytes();
            rec.type_ = Some(void_type());
        }
        let out = roundtrip(&pool);
        // Verify each record survived with the right token.
        for &(a, b) in &[(5u64, 0u64), (1, 9), (1, 2), (3, 0)] {
            let rec = out.get_record(&[a, b]).unwrap();
            assert_eq!(rec.get_token(), format!("ref_{a}_{b}").as_bytes());
        }
        // Verify lexicographic iteration order is preserved.
        let keys: Vec<CheapSorter> = out.cpool_map.keys().copied().collect();
        assert_eq!(
            keys,
            vec![
                CheapSorter { a: 1, b: 2 },
                CheapSorter { a: 1, b: 9 },
                CheapSorter { a: 3, b: 0 },
                CheapSorter { a: 5, b: 0 },
            ]
        );
    }

    #[test]
    fn bad_data_size_is_error() {
        // A <data> length beyond MAX_STRING_SIZE must error on decode.
        // Build a record manually with an oversized length attribute.
        let mgr = AddrSpaceManager::new();
        let reg = registry();
        let mut buf: Vec<u8> = Vec::new();
        {
            let mut enc = XmlEncode::new(&mut buf);
            enc.open_element(&ELEM_CPOOLREC);
            enc.write_string(&ATTRIB_TAG, b"primitive");
            enc.open_element(&ELEM_VALUE);
            enc.write_unsigned_integer(&ATTRIB_CONTENT, 0);
            enc.close_element(&ELEM_VALUE);
            enc.open_element(&ELEM_DATA);
            enc.write_signed_integer(&ATTRIB_LENGTH, MAX_STRING_SIZE);
            enc.write_string(&ATTRIB_CONTENT, b"");
            enc.close_element(&ELEM_DATA);
            enc.open_element(&kuna_base::marshal::ELEM_VOID);
            enc.close_element(&kuna_base::marshal::ELEM_VOID);
            enc.close_element(&ELEM_CPOOLREC);
        }
        let mut dec = XmlDecode::new(&mgr, &reg);
        dec.ingest_stream(&buf).unwrap();
        let mut rec = CPoolRecord::new();
        assert!(rec.decode(&mut dec, &mut VoidTypeSeam).is_err());
    }
}
