//! Port of the `decompiler/cpp/type.hh` `Datatype`/`TypeFactory` **interface**.
//!
//! # Scope (W5 `w5-dtype-expand`)
//!
//! This file is the *full* `Datatype` interface surface that the 14 parallel W8
//! rule porters compile against, transcribed from `type.hh` so none of them ever
//! has to touch this module.  It carries:
//!
//!   * the `type_metatype` / `sub_metatype` / `type_class` enums, **with their
//!     explicit discriminant values** — the numeric ordering is load-bearing
//!     (`sub_metatype` drives cast/propagation ordering via [`Datatype::compare`],
//!     and `Varnode::isBooleanValue`/`updateType` compare `metatype` against
//!     specific values), so they are reproduced byte-for-byte from `type.hh`;
//!   * the metatype helper free functions (`metatype2string`/`string2metatype`/
//!     `string2typeclass`/`metatype2typeclass`/`base2sub`);
//!   * the full [`Datatype`] class surface: every query accessor (`getSize`,
//!     `getMetatype`, `getSubMeta`, `getName`, …) and every boolean property
//!     (`isCoreType`, `isPointer`, `isEnumType`, …), implemented over the stored
//!     fields; the `compare`/`compareDependency` *signatures*, with the base
//!     bodies implemented (they only read `submeta`/`size`) and the structured
//!     overrides routed to a `// SEAM(W6)` `Err`;
//!   * the subclass *kinds* (TypeBase/Char/Unicode/Pointer/Array/Struct/Union/
//!     Enum/Code/Spacebase/Unknown/Void/PartialStruct/PartialUnion/PartialEnum/
//!     PointerRel) as the [`DatatypeKind`] enum, whose per-variant payload mirrors
//!     the C++ data members of each subclass;
//!   * the [`TypeFactory`] *handle* trait — the method set the rules/actions
//!     call (`getBase`/`getTypeVoid`/`getTypePointer*`/`getTypeArray`/
//!     `getTypeCode`/`getExactPiece`/`concretize`/…).
//!
//! # What is real vs. seam (`// SEAM(W6)`)
//!
//! Everything implementable from `type.hh` **alone** — simple queries over stored
//! fields, the size/metatype/submeta accessors, the base `compare`/
//! `compareDependency` (which only read `submeta`/`size`) — is implemented for
//! real.
//!
//! **W6 item `w6-s5-type-1` (this file) fills the base classes + scalar/pointer/
//! array bodies** from `type.cc` lines ~1-1722: the `TypePointer`/`TypeArray`
//! `compare`/`compareDependency`/`getSubType`/`findCompatibleResolve` overrides,
//! `TypeArray::getHoleSize`/`nearestArrayedComponent{Forward,Backward}`,
//! `TypePointer::isPtrsubMatching` (+ `testForArraySlack`), and the
//! `resolveInFlow`/`findResolve` base "return self" (LOSS-050 restored — they now
//! take a `self: &Rc<Datatype>` receiver and hand the receiver back for every type
//! without a union override).
//!
//! What remains **W6 (type-2/type-3)**: the `TypeStruct`/`TypeUnion`/`TypeEnum`/
//! `TypeCode`/`TypePartial*`/`TypePointerRel`/`TypeSpacebase` overrides, the
//! `Funcdata`-backed union resolution (the union/pointer-to-union/array
//! `resolveInFlow`/`findResolve` paths), the `TypeFactory` construction caches and
//! decode, and `printRaw`/`hashSize`.  Those surfaces exist here (so callers link)
//! but return `Err(KunaError::lowlevel("SEAM(W6) …"))`; each is tagged
//! `// SEAM(W6)` and listed in the relevant item's `losses` output.  W6 fills the
//! bodies in place — the *signatures* are frozen by this file.
//!
//! The `Datatype::new(size, metatype)` 2-arg convenience constructor is preserved
//! verbatim from the W3 skeleton (many W3 callers — `funcdata_varnode`, `op`,
//! `cpool`, `userop`, the per-processor test helpers — build skeletal types with
//! it); the real 3-arg C++ `Datatype(s,align,m)` is [`Datatype::new_with_align`].

use std::cell::{Cell, RefCell};
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::space::AddrSpace;
use kuna_base::types::{int4, int8, uint4, uint8};

use crate::bitfield::BitRange;

// =============================================================================
// type_metatype (type.hh:80-100) — verbatim, discriminants load-bearing
// =============================================================================

/// The core meta-types supported by the decompiler (C++ `enum type_metatype`,
/// `type.hh:80-100`).
///
/// These are sizeless templates for the elements making up the type algebra.
/// **The index is important** for [`Datatype::base2sub`]'s array, and the
/// ordering is significant: the lower the number, the more \b specific the
/// data-type, affecting propagation.  Transcribed with explicit discriminants.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum type_metatype {
    /// Part of a union
    TYPE_PARTIALUNION = 0,
    /// Part of a structure, stored separately from the whole
    TYPE_PARTIALSTRUCT = 1,
    /// Part of an enumerated value (specialization of TYPE_UINT)
    TYPE_PARTIALENUM = 2,
    /// An overlapping union of multiple datatypes
    TYPE_UNION = 3,
    /// Structure data-type, made up of component datatypes
    TYPE_STRUCT = 4,
    /// Signed enumeration data-type (specialization of TYPE_INT)
    TYPE_ENUM_INT = 5,
    /// Unsigned enumeration data-type (specialization of TYPE_UINT)
    TYPE_ENUM_UINT = 6,
    /// Array data-type, made up of a sequence of "element" datatype
    TYPE_ARRAY = 7,
    /// Pointer relative to another data-type (specialization of TYPE_PTR)
    TYPE_PTRREL = 8,
    /// Pointer data-type
    TYPE_PTR = 9,
    /// Floating-point
    TYPE_FLOAT = 10,
    /// Data is actual executable code
    TYPE_CODE = 11,
    /// Boolean
    TYPE_BOOL = 12,
    /// Unsigned integer
    TYPE_UINT = 13,
    /// Signed integer. Signed is considered less specific than unsigned in C
    TYPE_INT = 14,
    /// An unknown low-level type. Treated as an unsigned integer.
    TYPE_UNKNOWN = 15,
    /// Placeholder for symbol/type look-up calculations
    TYPE_SPACEBASE = 16,
    /// Standard "void" type, absence of type
    TYPE_VOID = 17,
}

impl type_metatype {
    /// The raw discriminant as an `i32`, matching the C++ enum's integer value.
    /// Used for the `<=`/ordering comparisons the C++ does directly on the enum
    /// (e.g. `Datatype::isPieceStructured`: `metatype <= TYPE_ARRAY`).
    #[inline]
    pub fn as_i32(self) -> i32 {
        self as i32
    }
}

// =============================================================================
// sub_metatype (type.hh:104-129) — verbatim, ordering drives cast propagation
// =============================================================================

/// Specializations of the core meta-types (C++ `enum sub_metatype`,
/// `type.hh:104-129`).
///
/// Each enumeration is associated with a specific [`type_metatype`].  Ordering
/// is important: the lower the number, the more \b specific the data-type,
/// affecting propagation.  This is the value [`Datatype::compare`] /
/// [`Datatype::compare_dependency`] sort on, so the discriminants are
/// transcribed verbatim.
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum sub_metatype {
    /// Compare as a TYPE_VOID
    SUB_VOID = 23,
    /// Compare as a TYPE_SPACEBASE
    SUB_SPACEBASE = 22,
    /// Compare as a TYPE_UNKNOWN
    SUB_UNKNOWN = 21,
    /// Compare as TYPE_PARTIALSTRUCT
    SUB_PARTIALSTRUCT = 20,
    /// Signed 1-byte character, sub-type of TYPE_INT
    SUB_INT_CHAR = 19,
    /// Unsigned 1-byte character, sub-type of TYPE_UINT
    SUB_UINT_CHAR = 18,
    /// Compare as a plain TYPE_INT
    SUB_INT_PLAIN = 17,
    /// Compare as a plain TYPE_UINT
    SUB_UINT_PLAIN = 16,
    /// Signed enum, sub-type of TYPE_INT
    SUB_INT_ENUM = 15,
    /// Unsigned partial enum, sub-type of TYPE_UINT
    SUB_UINT_PARTIALENUM = 14,
    /// Unsigned enum, sub-type of TYPE_UINT
    SUB_UINT_ENUM = 13,
    /// Signed wide character, sub-type of TYPE_INT
    SUB_INT_UNICODE = 12,
    /// Unsigned wide character, sub-type of TYPE_UINT
    SUB_UINT_UNICODE = 11,
    /// Compare as TYPE_BOOL
    SUB_BOOL = 10,
    /// Compare as TYPE_CODE
    SUB_CODE = 9,
    /// Compare as TYPE_FLOAT
    SUB_FLOAT = 8,
    /// Pointer to unknown field of struct, sub-type of TYPE_PTR
    SUB_PTRREL_UNK = 7,
    /// Compare as TYPE_PTR
    SUB_PTR = 6,
    /// Pointer relative to another data-type, sub-type of TYPE_PTR
    SUB_PTRREL = 5,
    /// Pointer into struct, sub-type of TYPE_PTR
    SUB_PTR_STRUCT = 4,
    /// Compare as TYPE_ARRAY
    SUB_ARRAY = 3,
    /// Compare as TYPE_STRUCT
    SUB_STRUCT = 2,
    /// Compare as TYPE_UNION
    SUB_UNION = 1,
    /// Compare as a TYPE_PARTIALUNION
    SUB_PARTIALUNION = 0,
}

impl sub_metatype {
    /// The raw discriminant as an `i32`, matching the C++ enum's integer value.
    #[inline]
    pub fn as_i32(self) -> i32 {
        self as i32
    }
}

// =============================================================================
// type_class (type.hh:132-142) — storage-assignment classes
// =============================================================================

/// Data-type classes for the purpose of assigning storage (C++ `enum
/// type_class`, `type.hh:132-142`).
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum type_class {
    /// General purpose
    TYPECLASS_GENERAL = 0,
    /// Floating-point data-types
    TYPECLASS_FLOAT = 1,
    /// Pointer data-types
    TYPECLASS_PTR = 2,
    /// Class for hidden return values
    TYPECLASS_HIDDENRET = 3,
    /// Vector data-types
    TYPECLASS_VECTOR = 4,
    /// Architecture specific class 1
    TYPECLASS_CLASS1 = 100,
    /// Architecture specific class 2
    TYPECLASS_CLASS2 = 101,
    /// Architecture specific class 3
    TYPECLASS_CLASS3 = 102,
    /// Architecture specific class 4
    TYPECLASS_CLASS4 = 103,
}

// =============================================================================
// Metatype helper free functions (type.cc:242-436)
// =============================================================================

/// `Datatype::base2sub[18]` (type.cc:23-27): per-metatype default submeta,
/// indexed by the metatype's discriminant.
///
/// The C++ `Datatype` constructor sets `submeta = base2sub[m]`.  Transcribed
/// verbatim from `type.cc`; index `i` corresponds to the `type_metatype` whose
/// discriminant is `i`.
pub const BASE2SUB: [sub_metatype; 18] = [
    sub_metatype::SUB_PARTIALUNION,     // 0  TYPE_PARTIALUNION
    sub_metatype::SUB_PARTIALSTRUCT,    // 1  TYPE_PARTIALSTRUCT
    sub_metatype::SUB_UINT_PARTIALENUM, // 2  TYPE_PARTIALENUM
    sub_metatype::SUB_UNION,            // 3  TYPE_UNION
    sub_metatype::SUB_STRUCT,           // 4  TYPE_STRUCT
    sub_metatype::SUB_INT_ENUM,         // 5  TYPE_ENUM_INT
    sub_metatype::SUB_UINT_ENUM,        // 6  TYPE_ENUM_UINT
    sub_metatype::SUB_ARRAY,            // 7  TYPE_ARRAY
    sub_metatype::SUB_PTRREL,           // 8  TYPE_PTRREL
    sub_metatype::SUB_PTR,              // 9  TYPE_PTR
    sub_metatype::SUB_FLOAT,            // 10 TYPE_FLOAT
    sub_metatype::SUB_CODE,             // 11 TYPE_CODE
    sub_metatype::SUB_BOOL,             // 12 TYPE_BOOL
    sub_metatype::SUB_UINT_PLAIN,       // 13 TYPE_UINT
    sub_metatype::SUB_INT_PLAIN,        // 14 TYPE_INT
    sub_metatype::SUB_UNKNOWN,          // 15 TYPE_UNKNOWN
    sub_metatype::SUB_SPACEBASE,        // 16 TYPE_SPACEBASE
    sub_metatype::SUB_VOID,             // 17 TYPE_VOID
];

/// Default sub-metatype for a given meta-type (C++ `Datatype::base2sub[m]`).
#[inline]
pub fn base2sub(m: type_metatype) -> sub_metatype {
    // cast: type_metatype discriminants are 0..17, all in-bounds for BASE2SUB[18].
    BASE2SUB[m as usize]
}

/// Convert a type \b meta-type into the string name of the meta-type
/// (C++ `metatype2string`, type.cc:242-303).
pub fn metatype2string(metatype: type_metatype) -> KunaResult<String> {
    use type_metatype::*;
    let res = match metatype {
        TYPE_VOID => "void",
        TYPE_PTR => "ptr",
        TYPE_PTRREL => "ptrrel",
        TYPE_ARRAY => "array",
        TYPE_PARTIALENUM => "partenum",
        TYPE_PARTIALSTRUCT => "partstruct",
        TYPE_PARTIALUNION => "partunion",
        TYPE_ENUM_INT => "enum_int",
        TYPE_ENUM_UINT => "enum_uint",
        TYPE_STRUCT => "struct",
        TYPE_UNION => "union",
        TYPE_SPACEBASE => "spacebase",
        TYPE_UNKNOWN => "unknown",
        TYPE_UINT => "uint",
        TYPE_INT => "int",
        TYPE_BOOL => "bool",
        TYPE_CODE => "code",
        TYPE_FLOAT => "float",
        // C++ `default: throw LowlevelError("Unknown metatype");`
    };
    Ok(res.to_string())
}

/// Given a string description of a type \b meta-type, return the meta-type
/// (C++ `string2metatype`, type.cc:307-369).
///
/// The C++ dispatches on the first character then matches the full string; the
/// transcription preserves the same fall-through structure so an unrecognized
/// string under a known first character still errors (matching the C++ break +
/// final `throw`).
pub fn string2metatype(metastring: &str) -> KunaResult<type_metatype> {
    use type_metatype::*;
    let first = metastring.as_bytes().first().copied();
    let res: Option<type_metatype> = match first {
        Some(b'p') => {
            if metastring == "ptr" {
                Some(TYPE_PTR)
            } else if metastring == "ptrrel" {
                Some(TYPE_PTRREL)
            } else if metastring == "partunion" {
                Some(TYPE_PARTIALUNION)
            } else if metastring == "partstruct" {
                Some(TYPE_PARTIALSTRUCT)
            } else {
                None
            }
        }
        Some(b'a') => (metastring == "array").then_some(TYPE_ARRAY),
        Some(b'e') => {
            if metastring == "enum_int" {
                Some(TYPE_ENUM_INT)
            } else if metastring == "enum_uint" {
                Some(TYPE_ENUM_UINT)
            } else {
                None
            }
        }
        Some(b's') => {
            if metastring == "struct" {
                Some(TYPE_STRUCT)
            } else if metastring == "spacebase" {
                Some(TYPE_SPACEBASE)
            } else {
                None
            }
        }
        Some(b'u') => {
            if metastring == "unknown" {
                Some(TYPE_UNKNOWN)
            } else if metastring == "uint" {
                Some(TYPE_UINT)
            } else if metastring == "union" {
                Some(TYPE_UNION)
            } else {
                None
            }
        }
        Some(b'i') => (metastring == "int").then_some(TYPE_INT),
        Some(b'f') => (metastring == "float").then_some(TYPE_FLOAT),
        Some(b'b') => (metastring == "bool").then_some(TYPE_BOOL),
        Some(b'c') => (metastring == "code").then_some(TYPE_CODE),
        Some(b'v') => (metastring == "void").then_some(TYPE_VOID),
        _ => None,
    };
    res.ok_or_else(|| {
        KunaError::lowlevel(format!("Unknown metatype: {metastring}"))
    })
}

/// Given a description of a data-type \e class, return the [`type_class`]
/// (C++ `string2typeclass`, type.cc:375-415).
pub fn string2typeclass(classstring: &str) -> KunaResult<type_class> {
    use type_class::*;
    let first = classstring.as_bytes().first().copied();
    let res: Option<type_class> = match first {
        Some(b'c') => {
            if classstring == "class1" {
                Some(TYPECLASS_CLASS1)
            } else if classstring == "class2" {
                Some(TYPECLASS_CLASS2)
            } else if classstring == "class3" {
                Some(TYPECLASS_CLASS3)
            } else if classstring == "class4" {
                Some(TYPECLASS_CLASS4)
            } else {
                None
            }
        }
        Some(b'g') => (classstring == "general").then_some(TYPECLASS_GENERAL),
        Some(b'h') => (classstring == "hiddenret").then_some(TYPECLASS_HIDDENRET),
        Some(b'f') => (classstring == "float").then_some(TYPECLASS_FLOAT),
        Some(b'p') => {
            (classstring == "ptr" || classstring == "pointer").then_some(TYPECLASS_PTR)
        }
        Some(b'v') => (classstring == "vector").then_some(TYPECLASS_VECTOR),
        // C++: 'u' "unknown" maps to the general purpose class.
        Some(b'u') => (classstring == "unknown").then_some(TYPECLASS_GENERAL),
        _ => None,
    };
    res.ok_or_else(|| {
        KunaError::lowlevel(format!("Unknown data-type class: {classstring}"))
    })
}

/// Convert a data-type metatype to a data-type class (C++ `metatype2typeclass`,
/// type.cc:424-436).
///
///   - `TYPE_FLOAT`  -> `TYPECLASS_FLOAT`
///   - `TYPE_PTR`    -> `TYPECLASS_PTR`
///   - everything else -> `TYPECLASS_GENERAL`
pub fn metatype2typeclass(meta: type_metatype) -> type_class {
    match meta {
        type_metatype::TYPE_FLOAT => type_class::TYPECLASS_FLOAT,
        type_metatype::TYPE_PTR => type_class::TYPECLASS_PTR,
        _ => type_class::TYPECLASS_GENERAL,
    }
}

// =============================================================================
// Datatype flag bits (the anonymous enum, type.hh:171-189)
// =============================================================================

/// Boolean property bits of a [`Datatype`] (C++ `Datatype`'s anonymous protected
/// `enum`, `type.hh:171-189`).  These are the bits stored in `Datatype::flags`.
pub mod flags {
    #![allow(non_upper_case_globals)]
    use kuna_base::types::uint4;
    /// This is a basic type which will never be redefined
    pub const coretype: uint4 = 1;
    /// ASCII character data
    pub const chartype: uint4 = 2;
    /// An enumeration type (as well as an integer)
    pub const enumtype: uint4 = 4;
    /// An enumeration type where all values are of 2^^n form
    pub const poweroftwo: uint4 = 8;
    /// 16-bit wide chars in unicode UTF16
    pub const utf16: uint4 = 16;
    /// 32-bit wide chars in unicode UTF32
    pub const utf32: uint4 = 32;
    /// Structure that should be treated as a string
    pub const opaque_string: uint4 = 64;
    /// May be other structures with same name different lengths
    pub const variable_length: uint4 = 128;
    /// Datatype has a stripped form for formal declarations
    pub const has_stripped: uint4 = 0x100;
    /// Datatype is a TypePointerRel
    pub const is_ptrrel: uint4 = 0x200;
    /// Set if this (recursive) data-type has not been fully defined yet
    pub const type_incomplete: uint4 = 0x400;
    /// Datatype (union, pointer to union) needs resolution before propagation
    pub const needs_resolution: uint4 = 0x800;
    /// 3-bits encoding display format, 0=none, 1=hex, 2=dec, 3=oct, 4=bin, 5=char
    pub const force_format: uint4 = 0x7000;
    /// Pointer can be truncated and is big endian
    pub const truncate_bigendian: uint4 = 0x8000;
    /// Data-type is a pointer to an array
    pub const pointer_to_array: uint4 = 0x10000;
    /// Data-type has an associated \e warning string
    pub const warning_issued: uint4 = 0x20000;
    /// Data-type contains bitfields
    pub const has_bitfields: uint4 = 0x40000;
}

// =============================================================================
// TypeField / TypeBitField (type.hh:311-342) — light record carriers
// =============================================================================

/// A field within a structure or union (C++ `TypeField`, type.hh:311-325).
///
/// The `compare`/`compareDependency`/`encode`/decode methods are W6; this
/// carries the data members so the `TypeStruct`/`TypeUnion` payloads can hold
/// fields, and the [`Datatype::num_depend`]/[`Datatype::get_depend`] accessors
/// can walk them.
#[derive(Debug, Clone)]
pub struct TypeField {
    /// Identifier of this within its containing structure or union
    pub ident: int4,
    /// Offset (into containing structure or union) of subfield
    pub offset: int4,
    /// Name of subfield
    pub name: String,
    /// Data-type of subfield
    pub field_type: Rc<Datatype>,
}

impl TypeField {
    /// Construct from components (C++ `TypeField(int4,int4,const string&,Datatype*)`).
    pub fn new(ident: int4, offset: int4, name: impl Into<String>, ct: Rc<Datatype>) -> TypeField {
        TypeField { ident, offset, name: name.into(), field_type: ct }
    }

    /// Compare field end-point to a given offset (C++ `TypeField::compareMaxByte`,
    /// type.hh:322-324): `off < field.offset + field.type->getSize()`.  Used as
    /// the `upper_bound` comparator `comp(value, element)` in `collectBitFields`
    /// / `hasBitFieldsInRange` to skip past fields ending at or before `off`.
    fn compare_max_byte(off: int4, field: &TypeField) -> bool {
        off < field.offset + field.field_type.get_size()
    }

    /// Compare meta-data of two fields for [`Datatype::compare`] on a struct
    /// (C++ `TypeField::compare`, type.cc:803-813): offset, then name, then
    /// first-level metatype.
    fn compare(&self, op2: &TypeField) -> int4 {
        if self.offset != op2.offset {
            return if self.offset < op2.offset { -1 } else { 1 };
        }
        if self.name != op2.name {
            return if self.name < op2.name { -1 } else { 1 };
        }
        if self.field_type.get_metatype() != op2.field_type.get_metatype() {
            return if self.field_type.get_metatype() < op2.field_type.get_metatype() {
                -1
            } else {
                1
            };
        }
        0
    }

    /// Compare structure of two fields for [`Datatype::compare_dependency`]
    /// (C++ `TypeField::compareDependency`, type.cc:818-828): offset, then name,
    /// then field-type pointer identity.
    fn compare_dependency(&self, op2: &TypeField) -> int4 {
        if self.offset != op2.offset {
            return if self.offset < op2.offset { -1 } else { 1 };
        }
        if self.name != op2.name {
            return if self.name < op2.name { -1 } else { 1 };
        }
        // C++ `if (type != op2.type) return (type < op2.type) ? -1 : 1;`
        // (compare the pointers directly).
        Datatype::compare_dependency_ptr(&self.field_type, &op2.field_type)
    }
}

/// A field within a structure that is not aligned/sized on byte boundaries
/// (C++ `TypeBitField`, type.hh:328-342).
///
/// In C++ the `BitRange bits` member carries the full
/// `byteOffset`/`byteSize`/`leastSigBit`/`numBits`/`isBigEndian` tuple.  W6
/// (this item) carries them inline as the struct fields below so the comparator
/// ([`TypeBitField::compare`]/[`TypeBitField::compare_dependency`], which run
/// `BitRange::compare`) and the layout assignment
/// ([`Datatype::assign_contiguous_bitfields`]) can be transcribed faithfully.
/// `BitRange` as a standalone type is still W6 elsewhere; the type subsystem
/// only needs this flattened projection.
#[derive(Debug, Clone)]
pub struct TypeBitField {
    /// Name of bitfield
    pub name: String,
    /// Underlying (integer) data-type
    pub field_type: Rc<Datatype>,
    /// Byte offset of the bitfield's container (C++ `bits.byteOffset`).
    pub byte_offset: int4,
    /// Byte size of the bitfield's container (C++ `bits.byteSize`).
    pub byte_size: int4,
    /// Least significant bit of the range within its container (C++
    /// `bits.leastSigBit`).
    pub least_sig_bit: int4,
    /// Number of bits in the range (C++ `bits.numBits`).
    pub num_bits: int4,
    /// Is the underlying encoding big endian (C++ `bits.isBigEndian`).
    pub is_big_endian: bool,
    /// Identifier of this within containing structure
    pub ident: int4,
}

impl TypeBitField {
    /// Construct from components (C++ `TypeBitField(int4 id,int4 numBits,bool
    /// isBigEndian,const string &nm,Datatype *ct)`, type.cc:883-888).
    ///
    /// The C++ initializer is `bits(0,(numBits+7)/8,0,numBits,isBigEndian)` —
    /// `byteOffset=0`, `byteSize=(numBits+7)/8`, `leastSigBit=0`.
    pub fn new(
        id: int4,
        num_bits: int4,
        is_big_endian: bool,
        nm: impl Into<String>,
        ct: Rc<Datatype>,
    ) -> TypeBitField {
        TypeBitField {
            name: nm.into(),
            field_type: ct,
            byte_offset: 0,
            byte_size: (num_bits + 7) / 8,
            least_sig_bit: 0,
            num_bits,
            is_big_endian,
            ident: id,
        }
    }

    /// Transcribe `BitRange::compare` over the flattened bit-range fields
    /// (address.cc:643-655): byteOffset, byteSize, leastSigBit, numBits in
    /// order, each a signed `int4` tie-break.
    fn bits_compare(&self, op2: &TypeBitField) -> int4 {
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

    /// Compare meta-data of two bitfields for [`Datatype::compare`] on a struct
    /// (C++ `TypeBitField::compare`, type.cc:893-903): `bits.compare`, then name,
    /// then first-level metatype.
    fn compare(&self, op2: &TypeBitField) -> int4 {
        let res = self.bits_compare(op2);
        if res != 0 {
            return res;
        }
        if self.name != op2.name {
            return if self.name < op2.name { -1 } else { 1 };
        }
        if self.field_type.get_metatype() != op2.field_type.get_metatype() {
            return if self.field_type.get_metatype() < op2.field_type.get_metatype() {
                -1
            } else {
                1
            };
        }
        0
    }

    /// Reconstruct the bitfield's [`BitRange`] from the flattened projection
    /// (C++ `TypeBitField::bits`).  The `bits` member is the load-bearing range
    /// the bitfield transforms read; the W6 type subsystem flattened it into the
    /// five scalar fields above, so rebuild the value type on demand for the
    /// range-arithmetic callers (`collect_bit_fields` / `has_bit_fields_in_range`).
    pub fn bits(&self) -> BitRange {
        BitRange::new(
            self.byte_offset,
            self.byte_size,
            self.least_sig_bit,
            self.num_bits,
            self.is_big_endian,
        )
    }

    /// Compare byte-container end-point to a given offset (C++
    /// `TypeBitField::compareMaxByte`, type.hh:339-341):
    /// `off < bitfield.bits.byteOffset + bitfield.bits.byteSize`.  This is the
    /// `upper_bound` comparator `comp(value, element)` used in `collectBitFields`
    /// / `hasBitFieldsInRange` — `true` for the first element whose container end
    /// is strictly past `off`.
    fn compare_max_byte(off: int4, bitfield: &TypeBitField) -> bool {
        off < bitfield.byte_offset + bitfield.byte_size
    }

    /// Compare structure of two bitfields for [`Datatype::compare_dependency`]
    /// (C++ `TypeBitField::compareDependency`, type.cc:908-918): `bits.compare`,
    /// then name, then field-type pointer identity.
    fn compare_dependency(&self, op2: &TypeBitField) -> int4 {
        let res = self.bits_compare(op2);
        if res != 0 {
            return res;
        }
        if self.name != op2.name {
            return if self.name < op2.name { -1 } else { 1 };
        }
        // C++ `if (type != op2.type) return (type < op2.type) ? -1 : 1;`
        // (compare the pointers directly).
        Datatype::compare_dependency_ptr(&self.field_type, &op2.field_type)
    }
}

// =============================================================================
// BitFieldTriple (type.hh:344-357) — collectBitFields output record
// =============================================================================

/// A bitfield description, along with its offset within a root container, in one
/// record (C++ `BitFieldTriple`, type.hh:347-357).
///
/// In C++ the record carries a live `const TypeStruct *immedContainer` and a
/// `const TypeBitField *bitfield` pointer.  The only datum any caller reads off
/// the triple is `bitfield->bits` (the range, in `BitFieldTransform::
/// establishFields`, `type.cc` line 1815) plus `offset`; the immediate-container
/// pointer is never dereferenced after construction.  So the port carries an
/// owned [`TypeBitField`] clone (cheap — `Rc<Datatype>` body) and the byte
/// `offset`, which is exactly the projection the bitfield transforms consume.
#[derive(Debug, Clone)]
pub struct BitFieldTriple {
    /// Description of the bitfield (C++ `bitfield`).
    pub bitfield: TypeBitField,
    /// Byte offset of the immediate container within parent (C++ `offset`).
    pub offset: int4,
}

impl BitFieldTriple {
    /// Construct from components (C++ `BitFieldTriple(const TypeStruct*,const
    /// TypeBitField*,int4)`, type.hh:352-353).  The `immedContainer` argument is
    /// dropped — see the type doc.
    pub fn new(bits: TypeBitField, off: int4) -> BitFieldTriple {
        BitFieldTriple { bitfield: bits, offset: off }
    }

    /// Comparator putting bitfields in byte order, least to most significant
    /// (C++ `BitFieldTriple::compare`, type.cc:932-949).  Returns `true` if
    /// `op1` sorts strictly before `op2`, the strict-weak ordering `std::sort`
    /// expects.  Mixed endianness is keyed off `op1`'s bitfield exactly as in
    /// C++.
    pub fn compare(op1: &BitFieldTriple, op2: &BitFieldTriple) -> bool {
        let is_big_endian = op1.bitfield.is_big_endian;
        let byte_off1 = op1.offset + op1.bitfield.byte_offset;
        let byte_off2 = op2.offset + op2.bitfield.byte_offset;
        if byte_off1 != byte_off2 {
            if is_big_endian {
                // Return least significant container; bigger byte offset is less
                // significant.
                return byte_off1 > byte_off2;
            }
            // Smaller byte offset is less significant.
            return byte_off1 < byte_off2;
        }
        let lsb1 = op1.bitfield.least_sig_bit;
        let lsb2 = op2.bitfield.least_sig_bit;
        if lsb1 != lsb2 {
            return lsb1 < lsb2;
        }
        // fields start at the same bit
        false
    }
}

// =============================================================================
// DatatypeKind — the subclass payloads (one variant per type.hh subclass)
// =============================================================================

/// The concrete subclass of a [`Datatype`], with the per-subclass data members
/// transcribed from `type.hh`.
///
/// In C++ each subclass (`TypePointer`, `TypeArray`, …) adds its own fields and
/// overrides the virtual methods.  Rust has no inheritance, so [`Datatype`]
/// carries the shared `Datatype` members (id/size/flags/name/metatype/submeta/…)
/// plus this enum for the subclass-specific payload; [`Datatype`]'s methods
/// match on `kind` to reproduce the virtual dispatch.
///
/// Heavy per-kind behavior (compare/getSubType/downChain/resolution) is **W6**;
/// the payload fields are present so W6 can fill those bodies without touching
/// the layout.
#[derive(Debug, Clone)]
pub enum DatatypeKind {
    /// `TypeBase` — fundamental atomic type with only name/size/meta-type
    /// (also the carrier for `TypeChar`/`TypeUnicode`, which are `TypeBase`
    /// subclasses distinguished only by flag bits: `chartype`/`utf16`/`utf32`).
    Base,
    /// `TypeVoid` (type.hh:435-446): the formal "void" data-type.
    Void,
    /// `TypeUnknown` — an unknown low-level type.  In C++ `TypeUnknown` is an
    /// alias relationship via `TypeBase` with metatype `TYPE_UNKNOWN`; carried
    /// as its own kind for clarity.
    Unknown,
    /// `TypePointer` (type.hh:449-489).
    Pointer {
        /// Type being pointed to (C++ `ptrto`)
        ptrto: Rc<Datatype>,
        /// Address space this is intended to point into (C++ `spaceid`)
        spaceid: Option<Rc<AddrSpace>>,
        /// Truncated form of the pointer, if any (C++ `truncate`)  // SEAM(W6)
        truncate: Option<Rc<Datatype>>,
        /// What size unit does the pointer address (C++ `wordsize`)
        wordsize: uint4,
    },
    /// `TypeArray` (type.hh:492-523).
    Array {
        /// Type of which we have an array (C++ `arrayof`)
        arrayof: Rc<Datatype>,
        /// Number of elements in the array (C++ `arraysize`)
        arraysize: int4,
    },
    /// `TypeStruct` (type.hh:566-611).
    Struct {
        /// List of fields (C++ `field`)
        field: Vec<TypeField>,
        /// List of fields not aligned/sized on byte boundaries (C++ `bitfield`)
        bitfield: Vec<TypeBitField>,
    },
    /// `TypeUnion` (type.hh:617-640).
    Union {
        /// The list of fields (C++ `field`)
        field: Vec<TypeField>,
    },
    /// `TypeEnum` (type.hh:529-563): an integer with named values.
    Enum {
        /// Map from integer to name (C++ `namemap`)
        namemap: std::collections::BTreeMap<u64, String>,
    },
    /// `TypeCode` (type.hh:771-792): executable code / function-pointer target.
    Code {
        /// If present, describes the prototype of the underlying function (C++
        /// `proto`, a `FuncProto *`).  // SEAM(W6) — the prototype model is W4/W6.
        proto: Option<Rc<crate::seams::FuncProto>>,
    },
    /// `TypeSpacebase` (type.hh:799-824): a pointer that indexes the symbol table.
    Spacebase {
        /// The address space being treated as a structure (C++ `spaceid`)
        spaceid: Option<Rc<AddrSpace>>,
        /// Address of the function whose symbol table is indexed (C++ `localframe`)
        localframe: Address,
    },
    /// `TypePartialStruct` (type.hh:665-684): holds part of a TypeStruct/TypeArray.
    PartialStruct {
        /// The undefined data-type to use if a formal data-type is required
        /// (C++ `stripped`)
        stripped: Rc<Datatype>,
        /// Parent structure or array of which this is a part (C++ `container`)
        container: Rc<Datatype>,
        /// Byte offset within the parent where this starts (C++ `offset`)
        offset: int4,
    },
    /// `TypePartialUnion` (type.hh:692-717): a variable's relative position
    /// within a union data-type.
    PartialUnion {
        /// The undefined data-type to use if a formal data-type is required
        /// (C++ `stripped`)
        stripped: Rc<Datatype>,
        /// Union data-type containing this partial (C++ `container`, a `TypeUnion *`)
        container: Rc<Datatype>,
        /// Offset (in bytes) into the container union (C++ `offset`)
        offset: int4,
    },
    /// `TypePartialEnum` (type.hh:643-662): part of a TypeEnum plus padding.
    PartialEnum {
        /// The undefined data-type to use if a formal data-type is required
        /// (C++ `stripped`)
        stripped: Rc<Datatype>,
        /// The enumeration data-type this is based on (C++ `parent`, a `TypeEnum *`)
        parent: Rc<Datatype>,
        /// Byte offset within the parent enum where this starts (C++ `offset`)
        offset: int4,
    },
    /// `TypePointerRel` (type.hh:724-763): a pointer with a fixed offset into a
    /// container.  In C++ this *derives from* `TypePointer`, so it also carries
    /// the `Pointer` members (ptrto/spaceid/wordsize); those live in the shared
    /// `Datatype` view via the parallel `Pointer` accessors, with the relative
    /// extras here.
    PointerRel {
        /// Type being pointed to (C++ `TypePointer::ptrto`)
        ptrto: Rc<Datatype>,
        /// What size unit does the pointer address (C++ `TypePointer::wordsize`)
        wordsize: uint4,
        /// Same data-type with container info stripped (C++ `stripped`)
        stripped: Option<Rc<Datatype>>,
        /// Parent structure/array which this is pointing into (C++ `parent`)
        parent: Rc<Datatype>,
        /// Byte offset within the parent where this points to (C++ `offset`)
        offset: int4,
    },
}

// =============================================================================
// Datatype — the base class surface (type.hh:167-308)
// =============================================================================

/// A borrowed view of a `TypePointer`'s payload (`ptrto`, optional `spaceid`,
/// `wordsize`), used where the C++ casts a `Datatype &` to `TypePointer *` in
/// the pointer comparison overrides.  See [`Datatype::as_plain_pointer`].
type PlainPointerView<'a> = (&'a Rc<Datatype>, Option<&'a Rc<AddrSpace>>, uint4);

/// The base datatype class for the decompiler (C++ `Datatype`, type.hh:167-308).
///
/// Used for symbols, function prototypes, type propagation, etc.  This carries
/// the full member layout from `type.hh` plus a [`DatatypeKind`] for the
/// subclass payload, and exposes the complete query/property interface.  Heavy
/// behavior (decode, per-kind compare/resolution, encode) is **W6** and tagged
/// `// SEAM(W6)`.
#[derive(Debug, Clone)]
pub struct Datatype {
    /// A unique id for the type (or 0 if an id is not assigned)
    pub id: uint8,
    /// Size (of variable holding a value of this type)
    pub size: int4,
    /// Boolean properties of the type (see [`flags`])
    pub flags: uint4,
    /// Name of type
    pub name: String,
    /// Name to display in output
    pub display_name: String,
    /// Meta-type - type disregarding size
    pub metatype: type_metatype,
    /// Sub-type of the meta-type, for comparisons
    pub submeta: sub_metatype,
    /// The immediate data-type being typedefed by this (C++ `typedefImm`)
    pub typedef_imm: Option<Rc<Datatype>>,
    /// Byte alignment expected for this data-type in addressable memory
    pub alignment: int4,
    /// Size of data-type rounded up to a multiple of alignment
    pub align_size: int4,
    /// The concrete subclass payload
    pub kind: DatatypeKind,
}

impl Datatype {
    // -- Constructors -------------------------------------------------------

    /// Construct a base data-type providing size, alignment, and meta-type
    /// (C++ `Datatype(int4 s,int4 align,type_metatype m)`, type.hh:217-218).
    ///
    /// `submeta = base2sub[m]`, `flags = 0`, `id = 0`, `typedefImm = null`,
    /// `alignment = align`, `alignSize = s`.  The subclass payload defaults to
    /// [`DatatypeKind::Base`]; the typed factory constructors below override it.
    pub fn new_with_align(s: int4, align: int4, m: type_metatype) -> Datatype {
        Datatype {
            id: 0,
            size: s,
            flags: 0,
            name: String::new(),
            display_name: String::new(),
            metatype: m,
            submeta: base2sub(m),
            typedef_imm: None,
            alignment: align,
            align_size: s,
            kind: DatatypeKind::Base,
        }
    }

    /// Construct a minimal skeletal data-type from a size and meta-type.
    ///
    /// Preserved verbatim from the W3 skeleton (many W3 callers build skeletal
    /// types this way).  Mirrors `TypeBase(int4 s,type_metatype m) :
    /// Datatype(s,-1,m)` (type.hh:393): alignment `-1`.
    pub fn new(size: int4, metatype: type_metatype) -> Datatype {
        Datatype::new_with_align(size, -1, metatype)
    }

    /// Produce a data-type id by hashing the type name (C++ `Datatype::hashName`,
    /// type.cc:693-705).  IDs produced this way have their two top header bits set
    /// (`0xC000000000000000`) to distinguish them from other IDs.  Transcribed
    /// byte-for-byte, with `uint8` (u64) wrapping arithmetic and the `nm[i]` byte
    /// taken as an unsigned `u8` (C++ `(uint8)nm[i]` on a `char`; the type names
    /// hashed here are ASCII so the sign of `char` is moot, but we mask to a byte).
    pub fn hash_name(nm: &str) -> uint8 {
        let mut res: uint8 = 123;
        for b in nm.bytes() {
            // C++ `res = (res<<8) | (res >> 56);` — a left rotate by 8 on a u64.
            res = res.rotate_left(8);
            res = res.wrapping_add(b as uint8);
            if (res & 1) == 0 {
                res ^= 0xfeabfeab; // Some kind of feedback
            }
        }
        res |= 0xC000000000000000; // Add header bits indicating a name hash
        res
    }

    /// Reversibly hash a size into an id (C++ `Datatype::hashSize`,
    /// type.cc:713-720).  Used to uniquify variable-length data-types by their
    /// specific instance size; feeding the output back with the same size
    /// recovers the base id.
    pub fn hash_size(id: uint8, size: int4) -> uint8 {
        // C++ `uint8 sizeHash = size;` — `size` is int4; assigning to uint8
        // sign-extends then truncates to 64 bits.  Match via `as i64 as u64`.
        let size_hash: uint8 = (size as i64 as uint8).wrapping_mul(0x98251033aecbabaf);
        id ^ size_hash
    }

    // -- Core property queries (type.hh:220-249) ----------------------------

    /// Is this a core data-type (C++ `isCoreType`).
    #[inline]
    pub fn is_core_type(&self) -> bool {
        (self.flags & flags::coretype) != 0
    }
    /// Does this print as a 'char' (C++ `isCharPrint`).
    #[inline]
    pub fn is_char_print(&self) -> bool {
        (self.flags & (flags::chartype | flags::utf16 | flags::utf32 | flags::opaque_string)) != 0
    }
    /// Is this an enumerated type (C++ `isEnumType`).
    #[inline]
    pub fn is_enum_type(&self) -> bool {
        (self.flags & flags::enumtype) != 0
    }
    /// Does this print as an ASCII 'char' (C++ `isASCII`).
    #[inline]
    pub fn is_ascii(&self) -> bool {
        (self.flags & flags::chartype) != 0
    }
    /// Does this print as UTF16 'wchar' (C++ `isUTF16`).
    #[inline]
    pub fn is_utf16(&self) -> bool {
        (self.flags & flags::utf16) != 0
    }
    /// Does this print as UTF32 'wchar' (C++ `isUTF32`).
    #[inline]
    pub fn is_utf32(&self) -> bool {
        (self.flags & flags::utf32) != 0
    }
    /// Is this a variable length structure (C++ `isVariableLength`).
    #[inline]
    pub fn is_variable_length(&self) -> bool {
        (self.flags & flags::variable_length) != 0
    }
    /// Is this an opaquely encoded string (C++ `isOpaqueString`).
    #[inline]
    pub fn is_opaque_string(&self) -> bool {
        (self.flags & flags::opaque_string) != 0
    }
    /// Is this a pointer to an array (C++ `isPointerToArray`).
    #[inline]
    pub fn is_pointer_to_array(&self) -> bool {
        (self.flags & flags::pointer_to_array) != 0
    }
    /// Is this a `TypePointerRel` (C++ `isPointerRel`).
    #[inline]
    pub fn is_pointer_rel(&self) -> bool {
        (self.flags & flags::is_ptrrel) != 0
    }
    /// Is this a non-ephemeral `TypePointerRel` (C++ `isFormalPointerRel`).
    #[inline]
    pub fn is_formal_pointer_rel(&self) -> bool {
        (self.flags & (flags::is_ptrrel | flags::has_stripped)) == flags::is_ptrrel
    }
    /// Return true if this has a stripped form (C++ `hasStripped`).
    #[inline]
    pub fn has_stripped(&self) -> bool {
        (self.flags & flags::has_stripped) != 0
    }
    /// Is this an incompletely defined data-type (C++ `isIncomplete`).
    #[inline]
    pub fn is_incomplete(&self) -> bool {
        (self.flags & flags::type_incomplete) != 0
    }
    /// Is this a union or a pointer to union (C++ `needsResolution`).
    #[inline]
    pub fn needs_resolution(&self) -> bool {
        (self.flags & flags::needs_resolution) != 0
    }
    /// Has a \e warning been issued about this data-type (C++ `hasWarning`).
    #[inline]
    pub fn has_warning(&self) -> bool {
        (self.flags & flags::warning_issued) != 0
    }
    /// Return true if this contains/overlaps bitfields (C++ `hasBitfields`).
    #[inline]
    pub fn has_bitfields(&self) -> bool {
        (self.flags & flags::has_bitfields) != 0
    }
    /// Get properties pointers inherit (C++ `inheritForPointer`).
    #[inline]
    pub fn inherit_for_pointer(&self) -> uint4 {
        self.flags & (flags::coretype | flags::warning_issued)
    }
    /// Get properties \e partial data-types inherit (C++ `inheritForPartial`).
    #[inline]
    pub fn inherit_for_partial(&self) -> uint4 {
        self.flags & flags::warning_issued
    }
    /// Get the display format for constants with this data-type (C++
    /// `getDisplayFormat`, the inline at type.hh:958).
    #[inline]
    pub fn get_display_format(&self) -> uint4 {
        (self.flags & flags::force_format) >> 12
    }
    /// Set the display format for constants with this data-type (C++
    /// `Datatype::setDisplayFormat`, type.cc:201-204).  `format` is one of
    /// 0=clear, 1=hex, 2=dec, 4=oct, 8=bin, 16=char; it is stored shifted into
    /// the `force_format` bit-field, replacing any prior format.
    #[inline]
    pub fn set_display_format(&mut self, format: uint4) {
        self.flags &= !flags::force_format; // Clear preexisting
        self.flags |= format << 12;
    }
    /// Get the type \b meta-type (C++ `getMetatype`).
    #[inline]
    pub fn get_metatype(&self) -> type_metatype {
        self.metatype
    }
    /// Get the \b sub-metatype (C++ `getSubMeta`).
    #[inline]
    pub fn get_sub_meta(&self) -> sub_metatype {
        self.submeta
    }
    /// Get the type id (C++ `getId`).
    #[inline]
    pub fn get_id(&self) -> uint8 {
        self.id
    }
    /// Get the type size (C++ `getSize`).
    #[inline]
    pub fn get_size(&self) -> int4 {
        self.size
    }
    /// Get size rounded up to multiple of alignment (C++ `getAlignSize`).
    #[inline]
    pub fn get_align_size(&self) -> int4 {
        self.align_size
    }
    /// Get the expected byte alignment (C++ `getAlignment`).
    #[inline]
    pub fn get_alignment(&self) -> int4 {
        self.alignment
    }
    /// Get the type name (C++ `getName`).
    #[inline]
    pub fn get_name(&self) -> &str {
        &self.name
    }
    /// Get string to use in display (C++ `getDisplayName`).
    #[inline]
    pub fn get_display_name(&self) -> &str {
        &self.display_name
    }
    /// Get the data-type immediately typedefed by this (or `None`) (C++
    /// `getTypedef`).
    #[inline]
    pub fn get_typedef(&self) -> Option<&Rc<Datatype>> {
        self.typedef_imm.as_ref()
    }

    // -- Inline computed properties (type.hh inline section) ----------------

    /// Get the type id, without variable length size adjustment (C++
    /// `getUnsizedId`, the inline at type.hh:968).
    ///
    /// SEAM(W6): the variable-length branch needs `Datatype::hashSize`
    /// (reversible size hashing in type.cc); the non-variable-length case is
    /// implemented (returns the plain id).
    pub fn get_unsized_id(&self) -> KunaResult<uint8> {
        if (self.flags & flags::variable_length) != 0 {
            // C++: return hashSize(id, size);  // SEAM(W6)
            Err(KunaError::lowlevel(
                "SEAM(W6): Datatype::getUnsizedId hashSize not yet ported",
            ))
        } else {
            Ok(self.id)
        }
    }

    /// Does this data-type consist of separate pieces (C++ `isPieceStructured`,
    /// the inline at type.hh:998: `metatype <= TYPE_ARRAY`).
    #[inline]
    pub fn is_piece_structured(&self) -> bool {
        // C++ compares the raw enum values; TYPE_ARRAY == 7.
        self.metatype.as_i32() <= type_metatype::TYPE_ARRAY.as_i32()
    }

    /// Order this with `op` for selecting a formal high-level data-type (C++
    /// `typeOrderFormal`, the inline at type.hh:983).
    ///
    /// Deemphasizes \e partial-union and \e bool, otherwise defers to
    /// [`compare`](Datatype::compare) at level 10.
    pub fn type_order_formal(&self, op: &Datatype) -> KunaResult<int4> {
        if std::ptr::eq(self, op) {
            return Ok(0);
        }
        if self.metatype == type_metatype::TYPE_PARTIALUNION {
            return Ok(1); // Prefer partials the least
        }
        if op.metatype == type_metatype::TYPE_PARTIALUNION {
            return Ok(-1);
        }
        if self.metatype == type_metatype::TYPE_BOOL {
            return Ok(1); // Prefer bool less than integers
        }
        if op.metatype == type_metatype::TYPE_BOOL {
            return Ok(-1);
        }
        self.compare(op, 10)
    }

    /// Order this with `op` datatype (C++ `typeOrder`, the inline at type.hh:301).
    pub fn type_order(&self, op: &Datatype) -> KunaResult<int4> {
        if std::ptr::eq(self, op) {
            return Ok(0);
        }
        self.compare(op, 10)
    }

    // -- Variable-length identity (type.cc:127-135) -------------------------

    /// Are these the same variable length data-type (C++ `hasSameVariableBase`).
    ///
    /// SEAM(W6): the non-trivial path needs `Datatype::hashSize`; the
    /// short-circuits (`!isVariableLength()`) are implemented and cover the
    /// common case (returns `false`).
    pub fn has_same_variable_base(&self, ct: &Datatype) -> KunaResult<bool> {
        if !self.is_variable_length() {
            return Ok(false);
        }
        if !ct.is_variable_length() {
            return Ok(false);
        }
        // C++: uint8 thisId = hashSize(id, size); ... return thisId == themId;
        Err(KunaError::lowlevel(
            "SEAM(W6): Datatype::hasSameVariableBase hashSize not yet ported",
        ))
    }

    // -- Component / pointer structure (type.hh:252-300) --------------------

    /// Get the number of component sub-types making up this data-type (C++
    /// `numDepend`).  Base default is 0; pointer/array are 1; struct/union are
    /// the field count.
    pub fn num_depend(&self) -> int4 {
        match &self.kind {
            DatatypeKind::Pointer { .. } | DatatypeKind::PointerRel { .. } => 1,
            DatatypeKind::Array { .. } => 1,
            // cast: field count fits int4 (a struct/union with >2^31 fields is
            // impossible — C++ also stores the count as int4).
            DatatypeKind::Struct { field, .. } => field.len() as int4,
            DatatypeKind::Union { field } => field.len() as int4,
            // TypePartialUnion::numDepend is computed from the resolved field
            // structure — W6.  // SEAM(W6)
            DatatypeKind::PartialUnion { .. } => 0,
            _ => 0,
        }
    }

    /// Get a specific component sub-type by index (C++ `getDepend`).
    pub fn get_depend(&self, index: int4) -> Option<Rc<Datatype>> {
        match &self.kind {
            DatatypeKind::Pointer { ptrto, .. } => Some(Rc::clone(ptrto)),
            DatatypeKind::PointerRel { ptrto, .. } => Some(Rc::clone(ptrto)),
            DatatypeKind::Array { arrayof, .. } => Some(Rc::clone(arrayof)),
            DatatypeKind::Struct { field, .. } => {
                // cast: index is a valid component index (0..numDepend); negative
                // would wrap to a huge usize, but `get` then returns None safely.
                field.get(index as usize).map(|f| Rc::clone(&f.field_type))
            }
            DatatypeKind::Union { field } => {
                field.get(index as usize).map(|f| Rc::clone(&f.field_type))
            }
            _ => None,
        }
    }

    /// If this is a pointer, return the data-type pointed to, with any offset
    /// passed back (C++ `getPtrInto`).
    ///
    /// For an ordinary [`DatatypeKind::Pointer`], returns `(ptrto, 0)`.  The
    /// `TypePointerRel::getPtrInto` override needs the relative-pointer math —
    /// W6.  Returns `None` for non-pointers (the C++ null).
    pub fn get_ptr_into(&self) -> KunaResult<Option<(Rc<Datatype>, int4)>> {
        match &self.kind {
            DatatypeKind::Pointer { ptrto, .. } => Ok(Some((Rc::clone(ptrto), 0))),
            DatatypeKind::PointerRel { ptrto, parent, offset, .. } => {
                // C++ TypePointerRel::getPtrInto (type.cc:3060-3070): a relative
                // pointer into a STRUCT/UNION points directly at the composite
                // (off = 0); otherwise it points `offset` into the parent.
                let meta = ptrto.get_metatype();
                if meta == type_metatype::TYPE_STRUCT || meta == type_metatype::TYPE_UNION {
                    Ok(Some((Rc::clone(ptrto), 0)))
                } else {
                    Ok(Some((Rc::clone(parent), *offset)))
                }
            }
            _ => Ok(None),
        }
    }

    /// If this is a partial data-type, return the whole data-type (C++
    /// `getPartialBase`).  Base default is `None`.
    pub fn get_partial_base(&self) -> Option<Rc<Datatype>> {
        match &self.kind {
            DatatypeKind::PartialStruct { container, .. } => Some(Rc::clone(container)),
            DatatypeKind::PartialUnion { container, .. } => Some(Rc::clone(container)),
            DatatypeKind::PartialEnum { parent, .. } => Some(Rc::clone(parent)),
            _ => None,
        }
    }

    /// Get a stripped version of this for formal use in declarations (C++
    /// `getStripped`).  Base default is `None`; the partial/relative-pointer
    /// kinds return their cached stripped form.
    pub fn get_stripped(&self) -> Option<Rc<Datatype>> {
        match &self.kind {
            DatatypeKind::PartialStruct { stripped, .. } => Some(Rc::clone(stripped)),
            DatatypeKind::PartialUnion { stripped, .. } => Some(Rc::clone(stripped)),
            DatatypeKind::PartialEnum { stripped, .. } => Some(Rc::clone(stripped)),
            DatatypeKind::PointerRel { stripped, .. } => stripped.clone(),
            _ => None,
        }
    }

    /// For a `TypePartialStruct`, return the component data-type that a pointer
    /// to this partial points to (C++ `TypePartialStruct::getComponentForPtr`,
    /// type.cc:2784).  If the container is an array and the offset lands on an
    /// element boundary, the element type; otherwise the stripped form.  `None`
    /// for non-partial-struct kinds.
    pub fn get_component_for_ptr(&self) -> Option<Rc<Datatype>> {
        match &self.kind {
            DatatypeKind::PartialStruct { container, stripped, offset } => {
                if container.get_metatype() == type_metatype::TYPE_ARRAY {
                    if let Some(eltype) = container.get_array_base() {
                        if eltype.get_metatype() != type_metatype::TYPE_UNKNOWN
                            && (*offset % eltype.get_align_size()) == 0
                        {
                            return Some(eltype);
                        }
                    }
                }
                Some(Rc::clone(stripped))
            }
            _ => None,
        }
    }

    // -- Pointer accessors (TypePointer, type.hh:471-473) -------------------

    /// Get the pointed-to Datatype (C++ `TypePointer::getPtrTo`).  Returns
    /// `None` if this is not a pointer kind.
    pub fn get_ptr_to(&self) -> Option<Rc<Datatype>> {
        match &self.kind {
            DatatypeKind::Pointer { ptrto, .. } => Some(Rc::clone(ptrto)),
            DatatypeKind::PointerRel { ptrto, .. } => Some(Rc::clone(ptrto)),
            _ => None,
        }
    }

    /// Get the size of the addressable unit being pointed to (C++
    /// `TypePointer::getWordSize`).  Returns `None` for non-pointers.
    pub fn get_word_size(&self) -> Option<uint4> {
        match &self.kind {
            DatatypeKind::Pointer { wordsize, .. } => Some(*wordsize),
            DatatypeKind::PointerRel { wordsize, .. } => Some(*wordsize),
            _ => None,
        }
    }

    /// Get any address space associated with this pointer (C++
    /// `TypePointer::getSpace`).  Returns `None` for non-pointers and for
    /// pointers without a bound space.
    pub fn get_pointer_space(&self) -> Option<Rc<AddrSpace>> {
        match &self.kind {
            DatatypeKind::Pointer { spaceid, .. } => spaceid.clone(),
            _ => None,
        }
    }

    // -- Array accessors (TypeArray, type.hh:505-506) -----------------------

    /// Get the element data-type (C++ `TypeArray::getBase`).  `None` for
    /// non-arrays.
    pub fn get_array_base(&self) -> Option<Rc<Datatype>> {
        match &self.kind {
            DatatypeKind::Array { arrayof, .. } => Some(Rc::clone(arrayof)),
            _ => None,
        }
    }

    /// Get the number of elements (C++ `TypeArray::numElements`).  `None` for
    /// non-arrays.
    pub fn num_elements(&self) -> Option<int4> {
        match &self.kind {
            DatatypeKind::Array { arraysize, .. } => Some(*arraysize),
            _ => None,
        }
    }

    // -- PointerRel accessors (type.hh:741-752) -----------------------------

    /// Get the parent data-type to which this pointer is offset (C++
    /// `TypePointerRel::getParent`).  `None` for non-relative-pointers.
    pub fn get_rel_parent(&self) -> Option<Rc<Datatype>> {
        match &self.kind {
            DatatypeKind::PointerRel { parent, .. } => Some(Rc::clone(parent)),
            _ => None,
        }
    }

    /// Get offset of this relative pointer in \e byte units (C++
    /// `TypePointerRel::getByteOffset`).  `None` for non-relative-pointers.
    pub fn get_byte_offset(&self) -> Option<int4> {
        match &self.kind {
            DatatypeKind::PointerRel { offset, .. } => Some(*offset),
            _ => None,
        }
    }

    /// Get this relative pointer's offset in \e address units (C++
    /// `TypePointerRel::getAddressOffset` = `byteToAddressInt(offset, wordsize)`).
    /// `None` for non-relative-pointers.
    pub fn get_address_offset(&self) -> Option<int4> {
        match &self.kind {
            DatatypeKind::PointerRel { offset, wordsize, .. } => {
                // cast: getAddressOffset returns int4; byteToAddressInt(int4,..) fits int4.
                Some(AddrSpace::byte_to_address_int(*offset as i64, *wordsize) as int4)
            }
            _ => None,
        }
    }

    /// C++ `TypeSpacebase::getAddress` (type.cc:3522): resolve a constant offset
    /// into a concrete `Address` in this spacebase's address space.  `None` when
    /// `self` is not a `TypeSpacebase`.
    ///
    /// Faithful to the C++: a *global* spacebase (`localframe.isInvalid()`) forces
    /// `sz = -1` to suppress full-encoding recovery; the local-frame (stack) case
    /// keeps the size and resolves through the manager (`resolveConstant`).
    pub fn spacebase_get_address(
        &self,
        off: uint8,
        mut sz: int4,
        point: &Address,
        manager: &kuna_base::space::AddrSpaceManager,
    ) -> Option<Address> {
        let (spaceid, localframe) = self.as_spacebase()?;
        let spaceid = spaceid?;
        if localframe.is_invalid() {
            sz = -1; // Suppress full-encoding recovery for a global spacebase
        }
        let mut full_encoding: uint8 = 0;
        manager
            .resolve_constant(spaceid, off, sz, point, &mut full_encoding)
            .ok()
    }

    /// C++ `TypePointerRel::evaluateThruParent(addrOff)` (type.cc:3039): would a
    /// `PTRSUB(this, addrOff)` be representable as an access through the parent
    /// container (vs. the basic ptrto form)?  `None` for non-relative-pointers.
    pub fn evaluate_thru_parent(&self, addr_off: u64) -> Option<bool> {
        match &self.kind {
            DatatypeKind::PointerRel { ptrto, wordsize, parent, offset, .. } => {
                // byteOff = addressToByte(addrOff, wordsize);
                let byte_off = AddrSpace::address_to_byte(addr_off, *wordsize);
                // if (ptrto STRUCT && byteOff < ptrto.size) return false;
                if ptrto.get_metatype() == type_metatype::TYPE_STRUCT
                    && byte_off < ptrto.get_size() as u64
                {
                    return Some(false);
                }
                // byteOff = (byteOff + offset) & calc_mask(size);
                let byte_off = byte_off
                    .wrapping_add(*offset as u64)
                    & kuna_base::address::calc_mask(self.get_size());
                Some(byte_off < parent.get_size() as u64)
            }
            _ => None,
        }
    }

    // -- Partial accessors (type.hh:651,673,701) ----------------------------

    /// Get the byte offset into the containing data-type for any partial kind
    /// (C++ `getOffset` on `TypePartialStruct`/`TypePartialUnion`/
    /// `TypePartialEnum`).  `None` for non-partial kinds.
    pub fn get_partial_offset(&self) -> Option<int4> {
        match &self.kind {
            DatatypeKind::PartialStruct { offset, .. } => Some(*offset),
            DatatypeKind::PartialUnion { offset, .. } => Some(*offset),
            DatatypeKind::PartialEnum { offset, .. } => Some(*offset),
            _ => None,
        }
    }

    // -- Ordering (type.hh:287-288; type.cc base bodies) --------------------

    /// Order types for propagation (C++ `Datatype::compare`).
    ///
    /// The **base** `Datatype::compare` (type.cc:216-222) is implemented for
    /// real — it reads only `submeta` and `size`:
    ///
    /// ```text
    /// if (submeta != op.submeta) return (submeta < op.submeta) ? -1 : 1;
    /// if (size != op.size) return (op.size - size);
    /// return 0;
    /// ```
    ///
    /// The `TypePointer::compare` (type.cc:1074-1093) and `TypeArray::compare`
    /// (type.cc:1363-1375) overrides are implemented for real (W6, this item).
    /// The remaining structured overrides (`TypeStruct`/`TypeUnion`/`TypeEnum`/
    /// `TypeCode`/`TypePartial*`/`TypePointerRel`/`TypeSpacebase::compare`) are
    /// type-2/type-3 and route to a `// SEAM(W6)` `Err`.  `level` is the
    /// recursion budget the overrides decrement.
    pub fn compare(&self, op: &Datatype, level: int4) -> KunaResult<int4> {
        match &self.kind {
            // Kinds whose C++ compare is exactly the base body (this also covers
            // TypeChar/TypeUnicode, which are TypeBase subclasses that do not
            // override compare — their submeta is set at construction).
            DatatypeKind::Base | DatatypeKind::Void | DatatypeKind::Unknown => {
                Ok(self.compare_base(op))
            }
            // TypePointer::compare (type.cc:1074-1093).
            DatatypeKind::Pointer { ptrto, spaceid, wordsize, .. } => {
                // Both must be pointers (the matching submeta guarantees op is a
                // TypePointer, not a TypePointerRel: their submetas are disjoint).
                self.compare_pointer_body(op, level, ptrto, spaceid.as_ref(), *wordsize)
            }
            // TypeArray::compare (type.cc:1363-1375).
            DatatypeKind::Array { arrayof, .. } => {
                let res = self.compare_base(op);
                if res != 0 {
                    return Ok(res);
                }
                let level = level - 1;
                if level < 0 {
                    if self.id == op.get_id() {
                        return Ok(0);
                    }
                    return Ok(if self.id < op.get_id() { -1 } else { 1 });
                }
                // Both must be arrays.
                let op_arrayof = op
                    .get_array_base()
                    .ok_or_else(|| Datatype::array_invariant_err("compare"))?;
                arrayof.compare(&op_arrayof, level) // Compare array elements
            }
            // TypeStruct::compare (type.cc:1978-2032).
            DatatypeKind::Struct { field, bitfield } => {
                self.compare_struct(op, level, field, bitfield)
            }
            // TypeUnion::compare (type.cc:2461-2498).
            DatatypeKind::Union { field } => self.compare_union(op, level, field),
            // TypeEnum::compare (type.cc:1588-1592): delegates to compareDependency.
            DatatypeKind::Enum { .. } => self.compare_dependency(op),
            // TypeCode::compare (type.cc:3292-3322).
            DatatypeKind::Code { proto } => self.compare_code(op, level, proto.as_ref()),
            // TypeSpacebase::compare (type.cc:3498-3502): delegates to compareDependency.
            DatatypeKind::Spacebase { .. } => self.compare_dependency(op),
            // TypePartialStruct::compare (type.cc:2829-2843).
            DatatypeKind::PartialStruct { container, offset, .. } => {
                self.compare_partial(op, level, container, *offset)
            }
            // TypePartialUnion::compare (type.cc:2902-2916).
            DatatypeKind::PartialUnion { container, offset, .. } => {
                self.compare_partial(op, level, container, *offset)
            }
            // TypePartialEnum::compare (type.cc:2715-2729).
            DatatypeKind::PartialEnum { parent, offset, .. } => {
                self.compare_partial(op, level, parent, *offset)
            }
            // TypePointerRel::compare (type.cc:3072-3090): compare as plain
            // pointers first, then the formal-vs-ephemeral tie-break on `stripped`.
            DatatypeKind::PointerRel { ptrto, wordsize, stripped, .. } => {
                let res = self.compare_pointer_body(op, level, ptrto, None, *wordsize)?;
                if res != 0 {
                    return Ok(res);
                }
                // Both must be relative pointers.
                let (_, _, op_stripped, _, _) = op
                    .as_pointer_rel()
                    .ok_or_else(|| Datatype::pointer_invariant_err("compare(ptrrel)"))?;
                // A formal relative pointer (stripped == null) may be compared to
                // its equivalent ephemeral version (stripped != null); prefer the
                // formal version.
                if stripped.is_none() {
                    if op_stripped.is_some() {
                        return Ok(-1);
                    }
                } else if op_stripped.is_none() {
                    return Ok(1);
                }
                Ok(0)
            }
        }
    }

    /// `TypeStruct::compare` body (type.cc:1978-2032).  `field`/`bitfield` are the
    /// caller's borrowed payload; `op` is asserted (by matching submeta in the
    /// base step) to be a `TypeStruct`.
    fn compare_struct(
        &self,
        op: &Datatype,
        mut level: int4,
        field: &[TypeField],
        bitfield: &[TypeBitField],
    ) -> KunaResult<int4> {
        let res = self.compare_base(op);
        if res != 0 {
            return Ok(res);
        }
        let (op_field, op_bitfield) = op
            .as_struct_fields()
            .ok_or_else(|| Datatype::struct_invariant_err("compare"))?;
        if field.len() != op_field.len() {
            // C++ `return (ts->field.size()-field.size());` — size_t difference
            // truncated to int4 (wrapping).
            return Ok((op_field.len() as int4).wrapping_sub(field.len() as int4));
        }
        // Test only the name and first level metatype first.
        for (f1, f2) in field.iter().zip(op_field.iter()) {
            let cmp = f1.compare(f2);
            if cmp != 0 {
                return Ok(cmp);
            }
        }
        if bitfield.len() != op_bitfield.len() {
            return Ok((op_bitfield.len() as int4).wrapping_sub(bitfield.len() as int4));
        }
        for (b1, b2) in bitfield.iter().zip(op_bitfield.iter()) {
            let cmp = b1.compare(b2);
            if cmp != 0 {
                return Ok(cmp);
            }
        }
        level -= 1;
        if level < 0 {
            if self.id == op.get_id() {
                return Ok(0);
            }
            return Ok(if self.id < op.get_id() { -1 } else { 1 });
        }
        // If we are still equal, now go down deep into each field type.
        for (f1, f2) in field.iter().zip(op_field.iter()) {
            // Short circuit recursive loops (pointer identity).
            if !Rc::ptr_eq(&f1.field_type, &f2.field_type) {
                let c = f1.field_type.compare(&f2.field_type, level)?;
                if c != 0 {
                    return Ok(c);
                }
            }
        }
        for (b1, b2) in bitfield.iter().zip(op_bitfield.iter()) {
            if !Rc::ptr_eq(&b1.field_type, &b2.field_type) {
                let c = b1.field_type.compare(&b2.field_type, level)?;
                if c != 0 {
                    return Ok(c);
                }
            }
        }
        Ok(0)
    }

    /// `TypeUnion::compare` body (type.cc:2461-2498).
    fn compare_union(
        &self,
        op: &Datatype,
        mut level: int4,
        field: &[TypeField],
    ) -> KunaResult<int4> {
        let res = self.compare_base(op);
        if res != 0 {
            return Ok(res);
        }
        let op_field = op
            .as_union_fields()
            .ok_or_else(|| Datatype::union_invariant_err("compare"))?;
        if field.len() != op_field.len() {
            return Ok((op_field.len() as int4).wrapping_sub(field.len() as int4));
        }
        // Test only the name and first level metatype first.
        for (f1, f2) in field.iter().zip(op_field.iter()) {
            if f1.name != f2.name {
                return Ok(if f1.name < f2.name { -1 } else { 1 });
            }
            if f1.field_type.get_metatype() != f2.field_type.get_metatype() {
                return Ok(if f1.field_type.get_metatype() < f2.field_type.get_metatype() {
                    -1
                } else {
                    1
                });
            }
        }
        level -= 1;
        if level < 0 {
            if self.id == op.get_id() {
                return Ok(0);
            }
            return Ok(if self.id < op.get_id() { -1 } else { 1 });
        }
        // If we are still equal, now go down deep into each field type.
        for (f1, f2) in field.iter().zip(op_field.iter()) {
            if !Rc::ptr_eq(&f1.field_type, &f2.field_type) {
                let c = f1.field_type.compare(&f2.field_type, level)?;
                if c != 0 {
                    return Ok(c);
                }
            }
        }
        Ok(0)
    }

    /// `TypeCode::compare` body (type.cc:3292-3322).  The prototype walk needs the
    /// W4/W6 `FuncProto` model (`numParams`/`getParam`/`getOutputType`), so the
    /// `compareBasic == 2` ("carry on with parameters") path is a `// SEAM(W6)`;
    /// the surface comparison ([`Self::compare_code_basic`]) is implemented.
    fn compare_code(
        &self,
        op: &Datatype,
        mut level: int4,
        proto: Option<&Rc<crate::seams::FuncProto>>,
    ) -> KunaResult<int4> {
        let res = self.compare_base(op);
        if res != 0 {
            return Ok(res);
        }
        let op_proto = op
            .as_code_proto()
            .ok_or_else(|| Datatype::code_invariant_err("compare"))?;
        let res = Datatype::compare_code_basic(proto, op_proto)?;
        if res != 2 {
            return Ok(res);
        }
        level -= 1;
        if level < 0 {
            if self.id == op.get_id() {
                return Ok(0);
            }
            return Ok(if self.id < op.get_id() { -1 } else { 1 });
        }
        // The remaining per-parameter / output-type recursion (type.cc:3306-3321)
        // needs the FuncProto parameter/output model.  // SEAM(W6)
        Err(KunaError::lowlevel(
            "SEAM(W6): TypeCode::compare prototype-parameter recursion needs FuncProto model",
        ))
    }

    /// `TypePartialStruct/TypePartialUnion/TypePartialEnum::compare` body — these
    /// three share an identical shape (type.cc:2829-2843, 2902-2916, 2715-2729):
    /// base compare, then offset, then recurse into the container/parent.
    /// `whole` is `container` (struct/union) or `parent` (enum).
    fn compare_partial(
        &self,
        op: &Datatype,
        mut level: int4,
        whole: &Rc<Datatype>,
        offset: int4,
    ) -> KunaResult<int4> {
        let res = self.compare_base(op);
        if res != 0 {
            return Ok(res);
        }
        // Both must be partial — read op's offset and whole.
        let (op_whole, op_offset) = op
            .as_partial_whole()
            .ok_or_else(|| Datatype::partial_invariant_err("compare"))?;
        if offset != op_offset {
            return Ok(if offset < op_offset { -1 } else { 1 });
        }
        level -= 1;
        if level < 0 {
            if self.id == op.get_id() {
                return Ok(0);
            }
            return Ok(if self.id < op.get_id() { -1 } else { 1 });
        }
        whole.compare(op_whole, level) // Compare the underlying whole
    }

    /// Read this data-type's `Pointer`-payload fields as a plain pointer
    /// (`ptrto`, `spaceid`, `wordsize`), or `None` if it is not a plain
    /// [`DatatypeKind::Pointer`].  Used by the `TypePointer::compare`/
    /// `compareDependency` overrides where the C++ casts `&op` to `TypePointer *`.
    fn as_plain_pointer(&self) -> Option<PlainPointerView<'_>> {
        match &self.kind {
            DatatypeKind::Pointer { ptrto, spaceid, wordsize, .. } => {
                Some((ptrto, spaceid.as_ref(), *wordsize))
            }
            _ => None,
        }
    }

    /// Read this data-type's pointer-payload fields as a `TypePointer`
    /// (`ptrto`, `spaceid`, `wordsize`) for *either* a plain
    /// [`DatatypeKind::Pointer`] **or** a [`DatatypeKind::PointerRel`].  A
    /// `TypePointerRel` *is-a* `TypePointer` in C++, so its inherited
    /// `TypePointer::compare`/`compareDependency` read these same members; a
    /// relative pointer never carries a bound address space, so `spaceid` is
    /// always `None` for the `PointerRel` arm.  `None` for non-pointers.
    fn as_any_pointer(&self) -> Option<PlainPointerView<'_>> {
        match &self.kind {
            DatatypeKind::Pointer { ptrto, spaceid, wordsize, .. } => {
                Some((ptrto, spaceid.as_ref(), *wordsize))
            }
            DatatypeKind::PointerRel { ptrto, wordsize, .. } => Some((ptrto, None, *wordsize)),
            _ => None,
        }
    }

    /// Borrow a `TypePointerRel`'s relative payload (`ptrto`, `wordsize`,
    /// `stripped`, `parent`, `offset`), used where the C++ casts `&op` to
    /// `TypePointerRel *`.  `None` if not a relative pointer.
    #[allow(clippy::type_complexity)]
    fn as_pointer_rel(
        &self,
    ) -> Option<(&Rc<Datatype>, uint4, Option<&Rc<Datatype>>, &Rc<Datatype>, int4)> {
        match &self.kind {
            DatatypeKind::PointerRel { ptrto, wordsize, stripped, parent, offset } => {
                Some((ptrto, *wordsize, stripped.as_ref(), parent, *offset))
            }
            _ => None,
        }
    }

    /// `TypePointer::compare` body (type.cc:1074-1093), reusable by both the
    /// plain `TypePointer` arm and the `TypePointerRel` arm (which calls the
    /// inherited `TypePointer::compare` first).  `op` is asserted (by the matching
    /// base step) to also be a pointer-kind.
    fn compare_pointer_body(
        &self,
        op: &Datatype,
        level: int4,
        ptrto: &Rc<Datatype>,
        spaceid: Option<&Rc<AddrSpace>>,
        wordsize: uint4,
    ) -> KunaResult<int4> {
        let res = self.compare_base(op);
        if res != 0 {
            return Ok(res);
        }
        let (op_ptrto, op_spaceid, op_wordsize) = op
            .as_any_pointer()
            .ok_or_else(|| Datatype::pointer_invariant_err("compare"))?;
        if wordsize != op_wordsize {
            // C++ unsigned `wordsize` (uint4) comparison.
            return Ok(if wordsize < op_wordsize { -1 } else { 1 });
        }
        if let Some(r) = Datatype::compare_pointer_space(spaceid, op_spaceid) {
            return Ok(r);
        }
        let level = level - 1;
        if level < 0 {
            if self.id == op.get_id() {
                return Ok(0);
            }
            return Ok(if self.id < op.get_id() { -1 } else { 1 });
        }
        ptrto.compare(op_ptrto, level) // Compare whats pointed to
    }

    /// Borrow a `TypeStruct`'s `field`/`bitfield` payload, used where the C++
    /// casts `&op` to `TypeStruct *`.  `None` if not a struct.
    fn as_struct_fields(&self) -> Option<(&[TypeField], &[TypeBitField])> {
        match &self.kind {
            DatatypeKind::Struct { field, bitfield } => Some((field, bitfield)),
            _ => None,
        }
    }

    /// Borrow a `TypeUnion`'s `field` payload, used where the C++ casts `&op` to
    /// `TypeUnion *`.  `None` if not a union.
    fn as_union_fields(&self) -> Option<&[TypeField]> {
        match &self.kind {
            DatatypeKind::Union { field } => Some(field),
            _ => None,
        }
    }

    /// Borrow a `TypeCode`'s `proto` payload, used where the C++ casts `&op` to
    /// `TypeCode *`.  `None` if not a code type.
    fn as_code_proto(&self) -> Option<Option<&Rc<crate::seams::FuncProto>>> {
        match &self.kind {
            DatatypeKind::Code { proto } => Some(proto.as_ref()),
            _ => None,
        }
    }

    /// Borrow the (whole, offset) pair of a partial data-type, used where the
    /// C++ casts `&op` to `TypePartialStruct/Union/Enum *`.  The "whole" is
    /// `container` for struct/union partials, `parent` for enum partials.  `None`
    /// if not a partial kind.
    fn as_partial_whole(&self) -> Option<(&Rc<Datatype>, int4)> {
        match &self.kind {
            DatatypeKind::PartialStruct { container, offset, .. } => Some((container, *offset)),
            DatatypeKind::PartialUnion { container, offset, .. } => Some((container, *offset)),
            DatatypeKind::PartialEnum { parent, offset, .. } => Some((parent, *offset)),
            _ => None,
        }
    }

    /// Borrow a `TypeEnum`'s `namemap`, used where the C++ casts `&op` to
    /// `TypeEnum *`.  `None` if not an enum.
    fn as_enum_namemap(&self) -> Option<&std::collections::BTreeMap<u64, String>> {
        match &self.kind {
            DatatypeKind::Enum { namemap } => Some(namemap),
            _ => None,
        }
    }

    /// Borrow a `TypeSpacebase`'s `(spaceid, localframe)`, used where the C++
    /// casts `&op` to `TypeSpacebase *`.  `None` if not a spacebase.
    fn as_spacebase(&self) -> Option<(Option<&Rc<AddrSpace>>, &Address)> {
        match &self.kind {
            DatatypeKind::Spacebase { spaceid, localframe } => {
                Some((spaceid.as_ref(), localframe))
            }
            _ => None,
        }
    }

    fn enum_invariant_err(method: &str) -> KunaError {
        KunaError::lowlevel(format!(
            "Datatype::{method}: enum override reached with non-enum op \
             (submeta invariant violated)"
        ))
    }
    fn spacebase_invariant_err(method: &str) -> KunaError {
        KunaError::lowlevel(format!(
            "Datatype::{method}: spacebase override reached with non-spacebase op \
             (submeta invariant violated)"
        ))
    }

    /// Transcribe the C++ `TypeSpacebase::compareDependency` spaceid tie-break
    /// (type.cc:3510): `if (spaceid != tsb->spaceid) return (spaceid <
    /// tsb->spaceid) ? -1:1;` — a *raw pointer* comparison of the AddrSpace
    /// objects (NOT the by-index ordering used by `TypePointer`).  Returns
    /// `Some(ordering)` when the spaces differ, `None` when they are the same
    /// (C++ falls through).  The Rust analogue of the object address is
    /// [`Rc::as_ptr`].  Both spaces are always present on a TypeSpacebase.
    fn compare_spacebase_space(
        a: Option<&Rc<AddrSpace>>,
        b: Option<&Rc<AddrSpace>>,
    ) -> Option<int4> {
        match (a, b) {
            (None, None) => None,
            // Mirrors the raw-pointer `<` with a null on one side (null sorts
            // before a real pointer in C++'s flat address space).
            (None, Some(_)) => Some(-1),
            (Some(_), None) => Some(1),
            (Some(sa), Some(sb)) => {
                if Rc::ptr_eq(sa, sb) {
                    None
                } else {
                    let pa = Rc::as_ptr(sa) as usize;
                    let pb = Rc::as_ptr(sb) as usize;
                    Some(if pa < pb { -1 } else { 1 })
                }
            }
        }
    }

    /// Internal-invariant error for a structured override reached with a
    /// non-matching `op` kind (the matching-submeta precondition was violated).
    fn struct_invariant_err(method: &str) -> KunaError {
        KunaError::lowlevel(format!(
            "Datatype::{method}: struct override reached with non-struct op \
             (submeta invariant violated)"
        ))
    }
    fn union_invariant_err(method: &str) -> KunaError {
        KunaError::lowlevel(format!(
            "Datatype::{method}: union override reached with non-union op \
             (submeta invariant violated)"
        ))
    }
    fn code_invariant_err(method: &str) -> KunaError {
        KunaError::lowlevel(format!(
            "Datatype::{method}: code override reached with non-code op \
             (submeta invariant violated)"
        ))
    }
    fn partial_invariant_err(method: &str) -> KunaError {
        KunaError::lowlevel(format!(
            "Datatype::{method}: partial override reached with non-partial op \
             (submeta invariant violated)"
        ))
    }

    /// `TypeCode::compareBasic` (type.cc:3252-3282): compare surface
    /// characteristics of two code prototypes without recursing into params.
    /// Returns -1/1 if they differ, 0 if equal with no params, 2 if equal on the
    /// surface but parameters must be compared.
    ///
    /// The `hasModel`/`getModelName`/`numParams`/`getComparableFlags` accessors
    /// are part of the W4/W6 `FuncProto` model.  The proto-presence cases (one or
    /// both `proto == null`) are fully implemented; the both-present-with-model
    /// case routes to a `// SEAM(W6)` `Err`.
    fn compare_code_basic(
        proto: Option<&Rc<crate::seams::FuncProto>>,
        op_proto: Option<&Rc<crate::seams::FuncProto>>,
    ) -> KunaResult<int4> {
        match (proto, op_proto) {
            (None, None) => Ok(0),
            (None, Some(_)) => Ok(1),
            (Some(_), None) => Ok(-1),
            (Some(_), Some(_)) => {
                // hasModel/getModelName/numParams/getComparableFlags — FuncProto.
                Err(KunaError::lowlevel(
                    "SEAM(W6): TypeCode::compareBasic needs FuncProto model accessors",
                ))
            }
        }
    }

    /// Transcribe the C++ `spaceid` tie-break shared by `TypePointer::compare`
    /// and `TypePointer::compareDependency` (type.cc:1082-1086, 1102-1106):
    ///
    /// ```text
    /// if (spaceid != tp->spaceid) {
    ///   if (spaceid == (AddrSpace *)0) return 1;   // ptrs with a space come earlier
    ///   if (tp->spaceid == (AddrSpace *)0) return -1;
    ///   return (spaceid->getIndex() < tp->spaceid->getIndex()) ? -1 : 1;
    /// }
    /// ```
    ///
    /// Returns `Some(ordering)` if the spaces differ (the C++ early return) or
    /// `None` if they are the same (C++ falls through).  `AddrSpace` identity is
    /// pointer identity in C++; address spaces are unique singletons, so two
    /// spaces are "the same" iff they share an `Rc` allocation, and otherwise
    /// they are ordered by `getIndex()` (which is itself unique per space).
    fn compare_pointer_space(
        a: Option<&Rc<AddrSpace>>,
        b: Option<&Rc<AddrSpace>>,
    ) -> Option<int4> {
        match (a, b) {
            (None, None) => None,
            (None, Some(_)) => Some(1),  // self has no space -> comes later
            (Some(_), None) => Some(-1), // other has no space -> self earlier
            (Some(sa), Some(sb)) => {
                if Rc::ptr_eq(sa, sb) {
                    None // same space
                } else {
                    // getIndex() is unique per space, so it never ties here.
                    Some(if sa.get_index() < sb.get_index() { -1 } else { 1 })
                }
            }
        }
    }

    /// Order two `Rc<Datatype>` by their pointed-to object identity, transcribing
    /// the C++ raw-pointer comparison used in `compareDependency` (`ptrto <
    /// tp->ptrto`, "compare the pointers directly").
    ///
    /// In the C++ TypeFactory every interned data-type is a unique object, so
    /// this compares already-distinct sub-types by their stored address; the
    /// resulting tree order is internally consistent within a single factory
    /// instance (it is rebuilt per run, exactly as in C++).  The Rust analogue
    /// of the object address is [`Rc::as_ptr`].
    fn compare_dependency_ptr(a: &Rc<Datatype>, b: &Rc<Datatype>) -> int4 {
        if Rc::ptr_eq(a, b) {
            return 0;
        }
        // cast: `as_ptr` addresses ordered like the C++ raw pointers.
        let pa = Rc::as_ptr(a) as usize;
        let pb = Rc::as_ptr(b) as usize;
        if pa < pb {
            -1
        } else {
            1
        }
    }

    /// Internal-invariant error for a `TypePointer` override reached with a
    /// non-pointer `op` (the matching-submeta precondition was violated).
    fn pointer_invariant_err(method: &str) -> KunaError {
        KunaError::lowlevel(format!(
            "Datatype::{method}: pointer override reached with non-pointer op \
             (submeta invariant violated)"
        ))
    }

    /// Internal-invariant error for a `TypeArray` override reached with a
    /// non-array `op`.
    fn array_invariant_err(method: &str) -> KunaError {
        KunaError::lowlevel(format!(
            "Datatype::{method}: array override reached with non-array op \
             (submeta invariant violated)"
        ))
    }

    /// The base `Datatype::compare` body (type.cc:216-222), shared by the kinds
    /// that do not override it.  Pure: only reads `submeta`/`size`.
    fn compare_base(&self, op: &Datatype) -> int4 {
        if self.submeta != op.submeta {
            return if self.submeta < op.submeta { -1 } else { 1 };
        }
        if self.size != op.size {
            // C++ `return (op.size - size);` (wrapping i32 subtraction).
            return op.size.wrapping_sub(self.size);
        }
        0
    }

    /// Compare for storage in tree structure (C++ `Datatype::compareDependency`).
    ///
    /// The **base** body (type.cc:231-237) is implemented for real.  The
    /// `TypePointer::compareDependency` (type.cc:1095-1108) and
    /// `TypeArray::compareDependency` (type.cc:1377-1384) overrides are
    /// implemented here (W6, this item).  The remaining structured overrides
    /// (`TypeStruct`/`TypeUnion`/…/`TypePointerRel`/`TypeSpacebase`) are
    /// type-2/type-3.
    pub fn compare_dependency(&self, op: &Datatype) -> KunaResult<int4> {
        match &self.kind {
            DatatypeKind::Base | DatatypeKind::Void | DatatypeKind::Unknown => {
                Ok(self.compare_dependency_base(op))
            }
            // TypePointer::compareDependency (type.cc:1095-1108).  Note: unlike
            // compare(), this compares submeta then ptrto-identity/wordsize/
            // spaceid, and finishes with (op.size - size) — it does NOT call the
            // base body's size comparison up front.
            DatatypeKind::Pointer { ptrto, spaceid, wordsize, .. } => {
                if self.submeta != op.get_sub_meta() {
                    return Ok(if self.submeta < op.get_sub_meta() { -1 } else { 1 });
                }
                // Both must be pointers.
                let (op_ptrto, op_spaceid, op_wordsize) = op
                    .as_plain_pointer()
                    .ok_or_else(|| Datatype::pointer_invariant_err("compareDependency"))?;
                let ptr_cmp = Datatype::compare_dependency_ptr(ptrto, op_ptrto);
                if ptr_cmp != 0 {
                    return Ok(ptr_cmp); // Compare absolute pointers
                }
                if *wordsize != op_wordsize {
                    return Ok(if *wordsize < op_wordsize { -1 } else { 1 });
                }
                if let Some(r) = Datatype::compare_pointer_space(spaceid.as_ref(), op_spaceid) {
                    return Ok(r);
                }
                // C++ `return (op.getSize()-size);` (wrapping i32 subtraction).
                Ok(op.get_size().wrapping_sub(self.size))
            }
            // TypeArray::compareDependency (type.cc:1377-1384).
            DatatypeKind::Array { arrayof, .. } => {
                if self.submeta != op.get_sub_meta() {
                    return Ok(if self.submeta < op.get_sub_meta() { -1 } else { 1 });
                }
                // Both must be arrays.
                let op_arrayof = op
                    .get_array_base()
                    .ok_or_else(|| Datatype::array_invariant_err("compareDependency"))?;
                let arr_cmp = Datatype::compare_dependency_ptr(arrayof, &op_arrayof);
                if arr_cmp != 0 {
                    return Ok(arr_cmp); // Compare absolute pointers
                }
                Ok(op.get_size().wrapping_sub(self.size))
            }
            // TypeStruct::compareDependency (type.cc:2034-2063).
            DatatypeKind::Struct { field, bitfield } => {
                self.compare_dependency_struct(op, field, bitfield)
            }
            // TypeUnion::compareDependency (type.cc:2500-2523).
            DatatypeKind::Union { field } => self.compare_dependency_union(op, field),
            // TypeEnum::compareDependency (type.cc:1594-1617).
            DatatypeKind::Enum { namemap } => self.compare_dependency_enum(op, namemap),
            // TypeCode::compareDependency (type.cc:3324-3350).
            DatatypeKind::Code { proto } => self.compare_dependency_code(op, proto.as_ref()),
            // TypeSpacebase::compareDependency (type.cc:3504-3514).
            DatatypeKind::Spacebase { spaceid, localframe } => {
                self.compare_dependency_spacebase(op, spaceid.as_ref(), localframe)
            }
            // TypePartialStruct/Union/Enum::compareDependency — identical shape
            // (type.cc:2845-2853, 2918-2926, 2731-2739).
            DatatypeKind::PartialStruct { container, offset, .. } => {
                self.compare_dependency_partial(op, container, *offset)
            }
            DatatypeKind::PartialUnion { container, offset, .. } => {
                self.compare_dependency_partial(op, container, *offset)
            }
            DatatypeKind::PartialEnum { parent, offset, .. } => {
                self.compare_dependency_partial(op, parent, *offset)
            }
            // TypePointerRel::compareDependency (type.cc:3092-3103): submeta,
            // then ptrto-identity, offset, parent-identity, wordsize, then
            // (op.size - size).  ptrto/parent are compared by object identity
            // exactly as in C++ (raw-pointer `<`), via Rc::as_ptr.
            DatatypeKind::PointerRel { ptrto, wordsize, parent, offset, .. } => {
                if self.submeta != op.get_sub_meta() {
                    return Ok(if self.submeta < op.get_sub_meta() { -1 } else { 1 });
                }
                let (op_ptrto, op_wordsize, _, op_parent, op_offset) = op
                    .as_pointer_rel()
                    .ok_or_else(|| Datatype::pointer_invariant_err("compareDependency(ptrrel)"))?;
                let ptr_cmp = Datatype::compare_dependency_ptr(ptrto, op_ptrto);
                if ptr_cmp != 0 {
                    return Ok(ptr_cmp); // Compare absolute pointers
                }
                if *offset != op_offset {
                    return Ok(if *offset < op_offset { -1 } else { 1 });
                }
                let par_cmp = Datatype::compare_dependency_ptr(parent, op_parent);
                if par_cmp != 0 {
                    return Ok(par_cmp);
                }
                if *wordsize != op_wordsize {
                    return Ok(if *wordsize < op_wordsize { -1 } else { 1 });
                }
                // C++ `return (op.getSize()-size);` (wrapping i32 subtraction).
                Ok(op.get_size().wrapping_sub(self.size))
            }
        }
    }

    /// `TypeStruct::compareDependency` body (type.cc:2034-2063).
    fn compare_dependency_struct(
        &self,
        op: &Datatype,
        field: &[TypeField],
        bitfield: &[TypeBitField],
    ) -> KunaResult<int4> {
        let res = self.compare_dependency_base(op);
        if res != 0 {
            return Ok(res);
        }
        let (op_field, op_bitfield) = op
            .as_struct_fields()
            .ok_or_else(|| Datatype::struct_invariant_err("compareDependency"))?;
        if field.len() != op_field.len() {
            return Ok((op_field.len() as int4).wrapping_sub(field.len() as int4));
        }
        for (f1, f2) in field.iter().zip(op_field.iter()) {
            let cmp = f1.compare_dependency(f2);
            if cmp != 0 {
                return Ok(cmp);
            }
        }
        if bitfield.len() != op_bitfield.len() {
            return Ok((op_bitfield.len() as int4).wrapping_sub(bitfield.len() as int4));
        }
        for (b1, b2) in bitfield.iter().zip(op_bitfield.iter()) {
            let cmp = b1.compare_dependency(b2);
            if cmp != 0 {
                return Ok(cmp);
            }
        }
        Ok(0)
    }

    /// `TypeUnion::compareDependency` body (type.cc:2500-2523).
    fn compare_dependency_union(&self, op: &Datatype, field: &[TypeField]) -> KunaResult<int4> {
        let res = self.compare_dependency_base(op);
        if res != 0 {
            return Ok(res);
        }
        let op_field = op
            .as_union_fields()
            .ok_or_else(|| Datatype::union_invariant_err("compareDependency"))?;
        if field.len() != op_field.len() {
            return Ok((op_field.len() as int4).wrapping_sub(field.len() as int4));
        }
        for (f1, f2) in field.iter().zip(op_field.iter()) {
            if f1.name != f2.name {
                return Ok(if f1.name < f2.name { -1 } else { 1 });
            }
            // C++ `if (fld1 != fld2) return (fld1 < fld2) ? -1 : 1;`.
            let cmp = Datatype::compare_dependency_ptr(&f1.field_type, &f2.field_type);
            if cmp != 0 {
                return Ok(cmp);
            }
        }
        Ok(0)
    }

    /// `TypeEnum::compareDependency` body (type.cc:1594-1617): base step, then
    /// namemap size, then entry-by-entry (value, then name) over the ordered map.
    fn compare_dependency_enum(
        &self,
        op: &Datatype,
        namemap: &std::collections::BTreeMap<u64, String>,
    ) -> KunaResult<int4> {
        // C++ `TypeBase::compareDependency(op)` — same as the base body.
        let res = self.compare_dependency_base(op);
        if res != 0 {
            return Ok(res);
        }
        let op_namemap = op
            .as_enum_namemap()
            .ok_or_else(|| Datatype::enum_invariant_err("compareDependency"))?;
        if namemap.len() != op_namemap.len() {
            return Ok(if namemap.len() < op_namemap.len() { -1 } else { 1 });
        }
        // BTreeMap iteration is in key order, matching std::map.
        for ((v1, n1), (v2, n2)) in namemap.iter().zip(op_namemap.iter()) {
            if v1 != v2 {
                return Ok(if v1 < v2 { -1 } else { 1 });
            }
            if n1 != n2 {
                return Ok(if n1 < n2 { -1 } else { 1 });
            }
        }
        Ok(0)
    }

    /// `TypeCode::compareDependency` body (type.cc:3324-3350).  The param/output
    /// recursion needs the `FuncProto` model.
    fn compare_dependency_code(
        &self,
        op: &Datatype,
        proto: Option<&Rc<crate::seams::FuncProto>>,
    ) -> KunaResult<int4> {
        let res = self.compare_dependency_base(op);
        if res != 0 {
            return Ok(res);
        }
        let op_proto = op
            .as_code_proto()
            .ok_or_else(|| Datatype::code_invariant_err("compareDependency"))?;
        let res = Datatype::compare_code_basic(proto, op_proto)?;
        if res != 2 {
            return Ok(res);
        }
        // Per-parameter pointer comparison + output-type — FuncProto.  // SEAM(W6)
        Err(KunaError::lowlevel(
            "SEAM(W6): TypeCode::compareDependency prototype recursion needs FuncProto model",
        ))
    }

    /// `TypeSpacebase::compareDependency` body (type.cc:3504-3514): base step,
    /// then spaceid identity, then localframe (skipped for the global space base).
    fn compare_dependency_spacebase(
        &self,
        op: &Datatype,
        spaceid: Option<&Rc<AddrSpace>>,
        localframe: &Address,
    ) -> KunaResult<int4> {
        let res = self.compare_dependency_base(op);
        if res != 0 {
            return Ok(res);
        }
        let (op_spaceid, op_localframe) = op
            .as_spacebase()
            .ok_or_else(|| Datatype::spacebase_invariant_err("compareDependency"))?;
        // C++ `if (spaceid != tsb->spaceid) return (spaceid < tsb->spaceid) ? -1:1;`
        // — pointer comparison of the AddrSpace objects.
        if let Some(r) = Datatype::compare_spacebase_space(spaceid, op_spaceid) {
            return Ok(r);
        }
        // C++ `if (localframe.isInvalid()) return 0;` — Global space base.
        if localframe.is_invalid() {
            return Ok(0);
        }
        // C++ `if (localframe != tsb->localframe) return (localframe < tsb->localframe) ? -1:1;`.
        if localframe != op_localframe {
            return Ok(if localframe < op_localframe { -1 } else { 1 });
        }
        Ok(0)
    }

    /// `TypePartialStruct/Union/Enum::compareDependency` body — these three share
    /// an identical shape (type.cc:2845-2853, 2918-2926, 2731-2739): submeta,
    /// then whole-pointer identity, then offset, finishing with `op.size - size`.
    fn compare_dependency_partial(
        &self,
        op: &Datatype,
        whole: &Rc<Datatype>,
        offset: int4,
    ) -> KunaResult<int4> {
        // C++ `if (submeta != op.getSubMeta()) return (submeta < op.getSubMeta()) ? -1 : 1;`
        if self.submeta != op.get_sub_meta() {
            return Ok(if self.submeta < op.get_sub_meta() { -1 } else { 1 });
        }
        let (op_whole, op_offset) = op
            .as_partial_whole()
            .ok_or_else(|| Datatype::partial_invariant_err("compareDependency"))?;
        // C++ `if (container != tp->container) return (container < tp->container) ? -1 : 1;`
        // (compare absolute pointers).
        let cmp = Datatype::compare_dependency_ptr(whole, op_whole);
        if cmp != 0 {
            return Ok(cmp);
        }
        if offset != op_offset {
            return Ok(if offset < op_offset { -1 } else { 1 });
        }
        // C++ `return (op.getSize()-size);` (wrapping i32 subtraction).
        Ok(op.get_size().wrapping_sub(self.size))
    }

    /// The base `Datatype::compareDependency` body (type.cc:231-237).
    fn compare_dependency_base(&self, op: &Datatype) -> int4 {
        if self.submeta != op.submeta {
            return if self.submeta < op.submeta { -1 } else { 1 };
        }
        if self.size != op.size {
            return op.size.wrapping_sub(self.size);
        }
        0
    }

    // -- Struct field index helpers (type.cc:1754-1876) ---------------------

    /// `TypeStruct::getFieldIter` (type.cc:1754-1772): binary-search the field
    /// containing `off`, returning the index or -1 if `off` is not inside a
    /// field.  `field` is sorted by ascending `offset`.
    fn get_field_iter(field: &[TypeField], off: int4) -> int4 {
        let mut min: int4 = 0;
        // C++ `int4 max = field.size()-1;` — for an empty list this is -1, and
        // the loop body never runs (min=0 > max=-1).
        let mut max: int4 = field.len() as int4 - 1;
        while min <= max {
            let mid = (min + max) / 2;
            let curfield = &field[mid as usize];
            if curfield.offset > off {
                max = mid - 1;
            } else {
                // curfield.offset <= off
                if curfield.offset + curfield.field_type.get_size() > off {
                    return mid;
                }
                min = mid + 1;
            }
        }
        -1
    }

    /// `TypeStruct::getLowerBoundField` (type.cc:1858-1876): the index of the
    /// last field whose offset is <= `off` (the field may or may not contain
    /// `off`), or -1 if no field starts at or before `off`.
    fn get_lower_bound_field(field: &[TypeField], off: int4) -> int4 {
        if field.is_empty() {
            return -1;
        }
        let mut min: int4 = 0;
        let mut max: int4 = field.len() as int4 - 1;
        while min < max {
            let mid = (min + max + 1) / 2;
            if field[mid as usize].offset > off {
                max = mid - 1;
            } else {
                // curfield.offset <= off
                min = mid;
            }
        }
        if min == max && field[min as usize].offset <= off {
            return min;
        }
        -1
    }

    /// Faithful `std::upper_bound(first, last, value, comp)` over a slice with
    /// the comparator `comp(value, element)` (C++ `<algorithm>` semantics).
    /// Returns the index of the first element for which `comp(value, element)`
    /// is `true`, or `slice.len()` if none — exactly what the
    /// `collectBitFields`/`hasBitFieldsInRange` `upper_bound` calls consume.
    /// The slice must be partitioned with respect to `comp` (the bitfield/field
    /// vectors are sorted, so it is).
    fn upper_bound_idx<T>(slice: &[T], value: int4, comp: impl Fn(int4, &T) -> bool) -> usize {
        let mut first: usize = 0;
        let mut count = slice.len();
        while count > 0 {
            let step = count / 2;
            let it = first + step;
            if !comp(value, &slice[it]) {
                first = it + 1;
                count -= step + 1;
            } else {
                count = step;
            }
        }
        first
    }

    /// Collect bitfield records that overlap a given byte range (C++
    /// `TypeStruct::collectBitFields`, type.cc:1804-1826).
    ///
    /// `base_offset` is the byte offset of `self` within the root structure;
    /// `res` accumulates [`BitFieldTriple`]s; `offset`/`sz` is the byte range to
    /// find overlaps in.  Iterates `self`'s own `bitfield` list (from the first
    /// whose container end is past `offset`) collecting every record that
    /// overlaps the range, then recurses into nested struct fields.  No-op when
    /// `self` is not a `TYPE_STRUCT`.
    pub fn collect_bit_fields(
        &self,
        base_offset: int4,
        res: &mut Vec<BitFieldTriple>,
        offset: int4,
        sz: int4,
    ) {
        let (field, bitfield) = match self.as_struct_fields() {
            Some(pair) => pair,
            None => return,
        };
        // upper_bound(bitfield.begin(),bitfield.end(),offset,TypeBitField::compareMaxByte)
        let start = Datatype::upper_bound_idx(bitfield, offset, TypeBitField::compare_max_byte);
        if start != bitfield.len() {
            // BitRange range(offset,sz,(*iter).bits.isBigEndian)
            let range = BitRange::byte_range(offset, sz, bitfield[start].is_big_endian);
            for cur_bit_field in &bitfield[start..] {
                // curBitField.bits.overlapTest(range)
                let code = cur_bit_field.bits().overlap_test(&range);
                if code == 1 {
                    break;
                }
                if code == -1 {
                    continue;
                }
                res.push(BitFieldTriple::new(cur_bit_field.clone(), base_offset));
            }
        }
        // upper_bound(field.begin(),field.end(),offset,TypeField::compareMaxByte)
        let fstart = Datatype::upper_bound_idx(field, offset, TypeField::compare_max_byte);
        for cur_field in &field[fstart..] {
            if cur_field.offset >= offset + sz {
                break;
            }
            if cur_field.field_type.get_metatype() != type_metatype::TYPE_STRUCT {
                continue;
            }
            if !cur_field.field_type.has_bitfields() {
                continue;
            }
            // Recurse into nested structure
            cur_field.field_type.collect_bit_fields(
                base_offset + cur_field.offset,
                res,
                offset - cur_field.offset,
                sz,
            );
        }
    }

    /// Return `true` if any bitfield overlaps a given byte range (C++
    /// `TypeStruct::hasBitFieldsInRange`, type.cc:1828-1852).  Same walk as
    /// [`collect_bit_fields`](Self::collect_bit_fields) but short-circuits on the
    /// first overlap.  No-op (returns `false`) when `self` is not a struct.
    pub fn has_bit_fields_in_range(&self, offset: int4, sz: int4) -> bool {
        let (field, bitfield) = match self.as_struct_fields() {
            Some(pair) => pair,
            None => return false,
        };
        let start = Datatype::upper_bound_idx(bitfield, offset, TypeBitField::compare_max_byte);
        if start != bitfield.len() {
            let range = BitRange::byte_range(offset, sz, bitfield[start].is_big_endian);
            for cur_bit_field in &bitfield[start..] {
                let code = cur_bit_field.bits().overlap_test(&range);
                if code == 1 {
                    break;
                }
                if code == -1 {
                    continue;
                }
                return true;
            }
        }
        let fstart = Datatype::upper_bound_idx(field, offset, TypeField::compare_max_byte);
        for cur_field in &field[fstart..] {
            if cur_field.offset >= offset + sz {
                break;
            }
            if cur_field.field_type.get_metatype() != type_metatype::TYPE_STRUCT {
                continue;
            }
            if !cur_field.field_type.has_bitfields() {
                continue;
            }
            // Recurse into nested structure
            if cur_field
                .field_type
                .has_bit_fields_in_range(offset - cur_field.offset, sz)
            {
                return true;
            }
        }
        false
    }

    /// Return the bitfield matching the given bit range (C++
    /// `TypeStruct::findMatchingBitField`, type.cc:1777-1797).
    ///
    /// Binary-searches the (offset-sorted) `bitfield` vector using
    /// [`BitRange::overlap_test`] as the comparator: `0` is an exact match (the
    /// bitfield is returned), `-1`/`1` narrow the search window, and any partial
    /// overlap (`2`) breaks the search with no match.  Returns an owned clone of
    /// the matched record (the expression-recovery family consumes it by value,
    /// the W6 type subsystem having flattened the `const TypeBitField *` pointer
    /// into the carried scalar fields).  `None` when `self` is not a struct or no
    /// exact match exists.
    pub fn find_matching_bit_field(&self, range: &BitRange) -> Option<TypeBitField> {
        let (_field, bitfield) = self.as_struct_fields()?;
        // C++ `int4 min = 0; int4 max = bitfield.size()-1;` — note `max` is the
        // signed last index, so an empty vector gives `max = -1` and the loop is
        // skipped immediately.
        let mut min: int4 = 0;
        let mut max: int4 = bitfield.len() as int4 - 1;
        while min <= max {
            let mid = (min + max) / 2;
            let cur = &bitfield[mid as usize];
            // C++ `range.overlapTest(curfield.bits)` — the receiver is the query
            // `range`, the argument is the candidate's `bits`.
            let code = range.overlap_test(&cur.bits());
            if code == 0 {
                return Some(cur.clone());
            }
            if code == -1 {
                max = mid - 1;
            } else if code == 1 {
                min = mid + 1;
            } else {
                break; // Partial overlap
            }
        }
        None
    }

    /// Calculate the aligned size given size and alignment (C++
    /// `Datatype::calcAlignSize`, type.cc:540-547): round `sz` up to a multiple
    /// of `align`.
    pub fn calc_align_size(sz: int4, align: int4) -> int4 {
        let mod_ = sz % align;
        if mod_ != 0 {
            sz + (align - mod_)
        } else {
            sz
        }
    }

    // -- Structured-walk virtuals (type.hh:251-300) — W6 --------------------

    /// Recover component data-type one-level down (C++ `getSubType`).
    ///
    /// The base body (type.cc:175-180) returns null and `newoff = off`.  The
    /// `TypePointer::getSubType` (type.cc:1061-1072, the \e truncate window) and
    /// `TypeArray::getSubType` (type.cc:1386-1393) overrides are implemented here
    /// (W6, this item).  The `TypeStruct`/`TypeSpacebase`/`TypePartialStruct`/
    /// `TypeCode` overrides are type-2/type-3.
    pub fn get_sub_type(&self, off: int8) -> KunaResult<(Option<Rc<Datatype>>, int8)> {
        match &self.kind {
            // Base body: no subtype, newoff = off.  TypeEnum/TypeUnion do not
            // override getSubType, so they share the base body.
            DatatypeKind::Base
            | DatatypeKind::Void
            | DatatypeKind::Unknown
            | DatatypeKind::Enum { .. }
            | DatatypeKind::Union { .. } => Ok((None, off)),
            // TypePointer::getSubType (type.cc:1061-1072): if a truncated form
            // exists and `off` lands in its window, return it; else fall to base.
            DatatypeKind::Pointer { truncate, .. } => {
                if let Some(trunc) = truncate {
                    // C++: min = (flags & truncate_bigendian) ? size - trunc->getSize() : 0
                    let min: int8 = if (self.flags & flags::truncate_bigendian) != 0 {
                        (self.size - trunc.get_size()) as int8
                    } else {
                        0
                    };
                    if off >= min && off < min + trunc.get_size() as int8 {
                        return Ok((Some(Rc::clone(trunc)), off - min));
                    }
                }
                // Datatype::getSubType(off, newoff): no subtype, newoff = off.
                Ok((None, off))
            }
            // TypeArray::getSubType (type.cc:1386-1393): go down one level to the
            // element type, renormalizing the offset modulo the element align-size.
            DatatypeKind::Array { arrayof, .. } => {
                if off >= self.size as int8 {
                    // Datatype::getSubType(off, newoff): no subtype, newoff = off.
                    return Ok((None, off));
                }
                let newoff = off % arrayof.get_align_size() as int8;
                Ok((Some(Rc::clone(arrayof)), newoff))
            }
            // TypeStruct::getSubType (type.cc:1894-1904): the field containing the
            // offset, with `newoff` relative to that field's offset.
            DatatypeKind::Struct { field, .. } => {
                // C++ `i = getFieldIter(off);` — `off` is int8 but getFieldIter
                // takes int4; the C++ implicitly narrows.  Mirror that narrowing.
                let i = Datatype::get_field_iter(field, off as int4);
                if i < 0 {
                    // C++ `return Datatype::getSubType(off,newoff);` — base body.
                    return Ok((None, off));
                }
                let curfield = &field[i as usize];
                let newoff = off - curfield.offset as int8;
                Ok((Some(Rc::clone(&curfield.field_type)), newoff))
            }
            // TypePartialStruct::getSubType (type.cc:2802-2816): descend into the
            // container, going down further while the component spills past this
            // partial's range.
            DatatypeKind::PartialStruct { container, offset, .. } => {
                let size_left: int8 = self.size as int8 - off;
                let mut off = off + *offset as int8;
                let mut ct = Rc::clone(container);
                loop {
                    let (next, newoff) = ct.get_sub_type(off)?;
                    match next {
                        None => return Ok((None, newoff)),
                        Some(next_ct) => {
                            off = newoff;
                            ct = next_ct;
                            // Component can extend beyond range of this partial, in
                            // which case we go down another level.
                            if (ct.get_size() as int8 - off) <= size_left {
                                break;
                            }
                        }
                    }
                }
                Ok((Some(ct), off))
            }
            // TypeCode::getSubType (type.cc:3284-3290): if a factory is bound,
            // return `getBase(1, TYPE_CODE)` with newoff=0; else null.  The
            // factory is part of the W6 TypeFactory construction.
            DatatypeKind::Code { .. } => Err(KunaError::lowlevel(
                "SEAM(W6): TypeCode::getSubType needs the bound TypeFactory (getBase)",
            )),
            // TypeSpacebase::getSubType (type.cc:3411-3433): resolves through the
            // symbol-table Scope — needs the W6 Architecture/Scope wiring.
            DatatypeKind::Spacebase { .. } => Err(KunaError::lowlevel(
                "SEAM(W6): TypeSpacebase::getSubType needs symbol-table Scope resolution",
            )),
            // TypePointerRel does not override getSubType — it inherits
            // TypePointer::getSubType, but a relative pointer never carries a
            // `truncate`, so the result is always the base `(None, off)`.
            // TypePartialUnion/TypePartialEnum also use the base body.
            DatatypeKind::PointerRel { .. }
            | DatatypeKind::PartialUnion { .. }
            | DatatypeKind::PartialEnum { .. } => Ok((None, off)),
        }
    }

    /// Get the number of bytes at the given offset that are padding (C++
    /// `getHoleSize`).  Base default is 0.  The `TypeArray::getHoleSize`
    /// (type.cc:1415-1420) override is implemented here (W6, this item); the
    /// `TypeStruct`/`TypePartialStruct` overrides are type-2.
    pub fn get_hole_size(&self, off: int4) -> KunaResult<int4> {
        match &self.kind {
            // TypeArray::getHoleSize: renormalize into the element and recurse.
            DatatypeKind::Array { arrayof, .. } => {
                let new_off = off % arrayof.get_align_size();
                arrayof.get_hole_size(new_off)
            }
            // TypeStruct::getHoleSize (type.cc:1906-1921).
            DatatypeKind::Struct { field, .. } => {
                let mut i = Datatype::get_lower_bound_field(field, off);
                if i >= 0 {
                    let curfield = &field[i as usize];
                    let new_off = off - curfield.offset;
                    if new_off < curfield.field_type.get_size() {
                        return curfield.field_type.get_hole_size(new_off);
                    }
                }
                i += 1; // advance to first field following off
                if (i as usize) < field.len() {
                    // Distance to following field.
                    return Ok(field[i as usize].offset - off);
                }
                Ok(self.size - off) // Distance to end of structure
            }
            // TypePartialStruct::getHoleSize (type.cc:2818-2827).
            DatatypeKind::PartialStruct { container, offset, .. } => {
                let size_left = self.size - off;
                let off = off + *offset;
                let mut res = container.get_hole_size(off)?;
                if res > size_left {
                    res = size_left;
                }
                Ok(res)
            }
            _ => Ok(0),
        }
    }

    /// If this data-type is (or contains) an array starting after the given
    /// offset, return the distance in bytes to the start of the array, passing
    /// back the offset difference and the element size (C++
    /// `nearestArrayedComponentForward`).
    ///
    /// Base body (type.cc:190-194) returns `-1`.  The `TypeArray` override
    /// (type.cc:1395-1402) is implemented here (W6, this item); the
    /// `TypeStruct`/`TypeSpacebase` overrides are type-2/type-3.  On a `-1`
    /// return the passed-back values are unspecified (mirrors the C++).
    pub fn nearest_arrayed_component_forward(
        &self,
        off: int8,
        max: int8,
    ) -> KunaResult<(int8, int8, int8)> {
        match &self.kind {
            // TypeArray::nearestArrayedComponentForward (type.cc:1395-1402).
            DatatypeKind::Array { arrayof, .. } => {
                let _ = max;
                if off > 0 {
                    return Ok((-1, off, 0)); // Skip if we are in the middle of array
                }
                let new_off = off;
                let el_size = arrayof.get_align_size() as int8;
                Ok((-off, new_off, el_size))
            }
            // TypeStruct::nearestArrayedComponentForward (type.cc:1947-1976).
            DatatypeKind::Struct { field, .. } => {
                // C++ `getLowerBoundField(off)` narrows off to int4.
                let mut i = Datatype::get_lower_bound_field(field, off as int4);
                let mut remain: int8;
                if i < 0 {
                    // No component starting before off; first component after.
                    i += 1;
                    remain = 0;
                } else {
                    remain = off - field[i as usize].offset as int8;
                }
                while (i as usize) < field.len() {
                    let subfield = &field[i as usize];
                    // The first struct field examined may have a negative diff.
                    let diff = subfield.offset as int8 - off;
                    if diff + remain > max {
                        break;
                    }
                    let (distance, _suboff, sub_el) =
                        subfield.field_type.nearest_arrayed_component_forward(remain, max)?;
                    if distance >= 0 {
                        let distance = diff + remain + distance;
                        if distance > max {
                            break;
                        }
                        // C++ sets `*newoff = -diff;` and passes back elSize.
                        return Ok((distance, -diff, sub_el));
                    }
                    i += 1;
                    remain = 0;
                }
                Ok((-1, off, 0))
            }
            // TypeSpacebase::nearestArrayedComponentForward (type.cc:3435-3480):
            // walks the symbol-table Scope — needs the W6 Architecture wiring.
            DatatypeKind::Spacebase { .. } => Err(KunaError::lowlevel(
                "SEAM(W6): TypeSpacebase::nearestArrayedComponentForward needs Scope resolution",
            )),
            // Base default: return -1.
            _ => Ok((-1, off, 0)),
        }
    }

    /// If this data-type is (or contains) an array starting before the given
    /// offset, return the distance in bytes to the end of the array, passing
    /// back the offset difference and the element size (C++
    /// `nearestArrayedComponentBackward`).
    ///
    /// Base body (type.cc:205-209) returns `-1`.  The `TypeArray` override
    /// (type.cc:1404-1413) is implemented here (W6, this item); the
    /// `TypeStruct`/`TypeSpacebase` overrides are type-2/type-3.
    pub fn nearest_arrayed_component_backward(
        &self,
        off: int8,
        max: int8,
    ) -> KunaResult<(int8, int8, int8)> {
        match &self.kind {
            // TypeArray::nearestArrayedComponentBackward (type.cc:1404-1413).
            DatatypeKind::Array { arrayof, .. } => {
                let _ = max;
                if off < 0 {
                    return Ok((-1, off, 0)); // Skip if we are before array
                }
                let new_off = off;
                let el_size = arrayof.get_align_size() as int8;
                let size = self.size as int8;
                let dist = if off <= size { size - off } else { off - size };
                Ok((dist, new_off, el_size))
            }
            // TypeStruct::nearestArrayedComponentBackward (type.cc:1923-1945).
            DatatypeKind::Struct { field, .. } => {
                let first_index = Datatype::get_lower_bound_field(field, off as int4);
                let mut i = first_index;
                while i >= 0 {
                    let subfield = &field[i as usize];
                    let diff = off - subfield.offset as int8;
                    let subtype = &subfield.field_type;
                    let remain: int8 = if i == first_index {
                        diff
                    } else {
                        subtype.get_size() as int8
                    };
                    if diff - remain > max {
                        break;
                    }
                    let (distance, _suboff, el_size) =
                        subtype.nearest_arrayed_component_backward(remain, max)?;
                    if distance >= 0 {
                        let distance = (diff - remain) + distance;
                        if distance > max {
                            break;
                        }
                        // C++ sets `*newoff = diff;`.
                        return Ok((distance, diff, el_size));
                    }
                    i -= 1;
                }
                Ok((-1, off, 0))
            }
            // TypeSpacebase::nearestArrayedComponentBackward (type.cc:3482-3496):
            // resolves through getSubType (Scope) — needs the W6 Scope wiring.
            DatatypeKind::Spacebase { .. } => Err(KunaError::lowlevel(
                "SEAM(W6): TypeSpacebase::nearestArrayedComponentBackward needs Scope resolution",
            )),
            // Base default: return -1.
            _ => Ok((-1, off, 0)),
        }
    }

    /// If the given data-type is an array, or has an arrayed component, return
    /// `true` (C++ `TypePointer::testForArraySlack`, type.cc:1131-1142).
    ///
    /// A static helper on the pointed-to data-type and the out-of-bounds offset.
    fn test_for_array_slack(dt: &Datatype, off: int8) -> KunaResult<bool> {
        if dt.get_metatype() == type_metatype::TYPE_ARRAY {
            return Ok(true);
        }
        if off < 0 {
            let (dist, _newoff, _el) = dt.nearest_arrayed_component_forward(off, 128)?;
            Ok(dist >= 0)
        } else {
            let (dist, _newoff, _el) = dt.nearest_arrayed_component_backward(off, 128)?;
            Ok(dist >= 0)
        }
    }

    /// Is this data-type suitable as input to a `CPUI_PTRSUB` op (C++
    /// `isPtrsubMatching`).
    ///
    /// The base body (type.cc:555-559) returns `false` — implemented.  The
    /// `TypePointer::isPtrsubMatching` (type.cc:1260-1312) override is
    /// implemented here (W6, this item); the `TypePointerRel::isPtrsubMatching`
    /// (type.cc:3138) override is type-3.
    pub fn is_ptrsub_matching(&self, off: int8, extra: int8, multiplier: int8) -> KunaResult<bool> {
        match &self.kind {
            // TypePointer::isPtrsubMatching (type.cc:1260-1312).
            DatatypeKind::Pointer { ptrto, wordsize, .. } => {
                Datatype::is_ptrsub_matching_pointer(ptrto, *wordsize, off, extra, multiplier)
            }
            // TypePointerRel::isPtrsubMatching (type.cc:3138-3147): if a stripped
            // (formal) form exists, defer to the inherited TypePointer body;
            // otherwise the relative-pointer bound check against the parent.
            DatatypeKind::PointerRel { ptrto, wordsize, stripped, parent, offset } => {
                if stripped.is_some() {
                    return Datatype::is_ptrsub_matching_pointer(
                        ptrto, *wordsize, off, extra, multiplier,
                    );
                }
                let i_off = AddrSpace::address_to_byte_int(off, *wordsize);
                let extra = AddrSpace::address_to_byte_int(extra, *wordsize);
                let i_off = i_off + *offset as int8 + extra;
                Ok(i_off >= 0 && i_off <= parent.get_size() as int8)
            }
            _ => Ok(false), // base default
        }
    }

    /// `TypePointer::isPtrsubMatching` body (type.cc:1260-1312), factored out so
    /// the `TypePointerRel` arm (whose formal/`stripped` form re-uses the inherited
    /// `TypePointer` logic) can call it.  `extra`/`multiplier` are mutated locally
    /// in the C++ (by-value params), so they are shadowed with `let mut` to match.
    fn is_ptrsub_matching_pointer(
        ptrto: &Rc<Datatype>,
        wordsize: uint4,
        off: int8,
        extra: int8,
        multiplier: int8,
    ) -> KunaResult<bool> {
        let wordsize = &wordsize;
        let mut extra = extra;
        let mut multiplier = multiplier;
        let meta = ptrto.get_metatype();
        match meta {
            type_metatype::TYPE_SPACEBASE => {
                let new_off = AddrSpace::address_to_byte_int(off, *wordsize);
                let (sub, new_off2) = ptrto.get_sub_type(new_off)?;
                let sub_type = match sub {
                    Some(s) if new_off2 == 0 => s,
                    _ => return Ok(false),
                };
                extra = AddrSpace::address_to_byte_int(extra, *wordsize);
                // C++ nested-if; `&&` preserves short-circuit (the slack
                // test only runs when `extra` is out of bounds).
                if (extra < 0 || extra >= sub_type.get_size() as int8)
                    && !Datatype::test_for_array_slack(&sub_type, extra)?
                {
                    return Ok(false);
                }
            }
            type_metatype::TYPE_ARRAY => {
                if off != 0 {
                    return Ok(false);
                }
                multiplier = AddrSpace::address_to_byte_int(multiplier, *wordsize);
                if multiplier >= ptrto.get_align_size() as int8 {
                    return Ok(false);
                }
            }
            type_metatype::TYPE_STRUCT => {
                let typesize = ptrto.get_size() as int8;
                multiplier = AddrSpace::address_to_byte_int(multiplier, *wordsize);
                if multiplier >= ptrto.get_align_size() as int8 {
                    return Ok(false);
                }
                let new_off = AddrSpace::address_to_byte_int(off, *wordsize);
                extra = AddrSpace::address_to_byte_int(extra, *wordsize);
                let (sub, new_off2) = ptrto.get_sub_type(new_off)?;
                match sub {
                    Some(sub_type) => {
                        if new_off2 != 0 {
                            return Ok(false);
                        }
                        // C++ nested-if; `&&` preserves short-circuit.
                        if (extra < 0 || extra >= sub_type.get_size() as int8)
                            && !Datatype::test_for_array_slack(&sub_type, extra)?
                        {
                            return Ok(false);
                        }
                    }
                    None => {
                        // C++: extra += newoff; (newoff is the passed-back value)
                        extra += new_off2;
                        if (extra < 0 || extra >= typesize) && typesize != 0 {
                            return Ok(false);
                        }
                    }
                }
            }
            type_metatype::TYPE_UNION => {
                // A PTRSUB reaching here cannot be used for a union field
                // resolution; always return false.
                return Ok(false);
            }
            _ => return Ok(false), // Not a pointer to a structured data-type
        }
        Ok(true)
    }

    /// Is this made up of a single primitive (C++ `isPrimitiveWhole`,
    /// type.cc:505-518).
    ///
    /// Implemented per the C++ recursion: a non-structured type is whole; a
    /// single-component array/struct of equal size recurses; otherwise false.
    pub fn is_primitive_whole(&self) -> bool {
        if !self.is_piece_structured() {
            return true;
        }
        // C++: if (metatype == TYPE_ARRAY || metatype == TYPE_STRUCT) {
        //        if (numDepend() > 0) { Datatype *component = getDepend(0);
        //          if (component->getSize() == getSize()) return component->isPrimitiveWhole(); } }
        let is_struct_or_array = self.metatype == type_metatype::TYPE_ARRAY
            || self.metatype == type_metatype::TYPE_STRUCT;
        if is_struct_or_array && self.num_depend() > 0 {
            if let Some(component) = self.get_depend(0) {
                if component.get_size() == self.get_size() {
                    return component.is_primitive_whole();
                }
            }
        }
        false
    }

    // -- Resolution virtuals (type.hh:292-295) — W6 -------------------------

    /// Tailor data-type propagation based on Varnode use (C++ `resolveInFlow`).
    ///
    /// **LOSS-050 restored (W6, this item).** The base body (type.cc:577-581)
    /// returns `this` (the data-type unchanged) for every type without an
    /// override; with the `self: &Rc<Datatype>` receiver we can now hand the
    /// receiver back (`Rc::clone(self)`).  The overrides are:
    ///
    ///   * `TypePointer::resolveInFlow` (type.cc:1314-1333): if the pointed-to
    ///     type is a `TYPE_UNION`, score/resolve the union field against the
    ///     `PcodeOp`/slot (needs `Funcdata` wiring — `// SEAM(W6)`); otherwise it
    ///     returns `this`, which we honor here.
    ///   * `TypeArray::resolveInFlow` (type.cc:1455-1468): always does `Funcdata`
    ///     union-field scoring — `// SEAM(W6)`.
    ///   * `TypeStruct`/`TypeUnion`/`TypePartialUnion::resolveInFlow`: type-2
    ///     overrides — `// SEAM(W6)`.
    ///
    /// `op`/`slot` are opaque (`OpId`/`int4`); only the seamed union paths read
    /// them, and those still need the `Funcdata` registry that W6+ provides.
    pub fn resolve_in_flow(
        self: &Rc<Datatype>,
        _op: crate::seams::OpId,
        _slot: int4,
    ) -> KunaResult<Rc<Datatype>> {
        match &self.kind {
            // TypePointer::resolveInFlow — only pointers to unions resolve.
            DatatypeKind::Pointer { ptrto, .. } => {
                if ptrto.get_metatype() == type_metatype::TYPE_UNION {
                    Err(KunaError::lowlevel(
                        "SEAM(W6): TypePointer::resolveInFlow (pointer-to-union) needs \
                         Funcdata union-field resolution",
                    ))
                } else {
                    // C++ `return this;`
                    Ok(Rc::clone(self))
                }
            }
            // TypeArray::resolveInFlow always scores via Funcdata.
            DatatypeKind::Array { .. } => Err(KunaError::lowlevel(
                "SEAM(W6): TypeArray::resolveInFlow needs Funcdata union-field scoring",
            )),
            // type-2 structured overrides.
            DatatypeKind::Struct { .. }
            | DatatypeKind::Union { .. }
            | DatatypeKind::PartialUnion { .. } => Err(KunaError::lowlevel(
                "SEAM(W6): Datatype::resolveInFlow structured override not yet ported",
            )),
            // Base "return this": every type without an override.
            _ => Ok(Rc::clone(self)),
        }
    }

    /// Find a previously resolved sub-type (C++ `findResolve`).
    ///
    /// **LOSS-050 restored (W6, this item).** The const counterpart of
    /// [`resolve_in_flow`](Datatype::resolve_in_flow); the base body
    /// (type.cc:590-593) returns `this`.  The `TypePointer::findResolve`
    /// (type.cc:1335-1345) override only consults the `Funcdata` cache when the
    /// pointed-to type is a union (`// SEAM(W6)`), otherwise it returns `this`;
    /// `TypeArray::findResolve` (type.cc:1470-1478) and the type-2 structured
    /// overrides need the `Funcdata` cache (`// SEAM(W6)`).
    pub fn find_resolve(
        self: &Rc<Datatype>,
        _op: crate::seams::OpId,
        _slot: int4,
    ) -> KunaResult<Rc<Datatype>> {
        match &self.kind {
            DatatypeKind::Pointer { ptrto, .. } => {
                if ptrto.get_metatype() == type_metatype::TYPE_UNION {
                    Err(KunaError::lowlevel(
                        "SEAM(W6): TypePointer::findResolve (pointer-to-union) needs \
                         Funcdata union-field cache",
                    ))
                } else {
                    // C++ `return this;`
                    Ok(Rc::clone(self))
                }
            }
            DatatypeKind::Array { .. } => Err(KunaError::lowlevel(
                "SEAM(W6): TypeArray::findResolve needs Funcdata union-field cache",
            )),
            DatatypeKind::Struct { .. }
            | DatatypeKind::Union { .. }
            | DatatypeKind::PartialUnion { .. } => Err(KunaError::lowlevel(
                "SEAM(W6): Datatype::findResolve structured override not yet ported",
            )),
            _ => Ok(Rc::clone(self)),
        }
    }

    /// Find a resolution compatible with the given data-type (C++
    /// `findCompatibleResolve`).  Base default returns -1.
    ///
    /// The `TypePointer::findCompatibleResolve` (type.cc:1347-1354) and
    /// `TypeArray::findCompatibleResolve` (type.cc:1480-1490) overrides are
    /// implemented here (W6, this item); the `TypeUnion`/`TypePartialUnion`/
    /// `TypeStruct` overrides are type-2.
    pub fn find_compatible_resolve(&self, ct: &Datatype) -> KunaResult<int4> {
        match &self.kind {
            // TypePointer::findCompatibleResolve (type.cc:1347-1354): if `ct` is a
            // pointer, recurse on the pointed-to types.
            //
            // `TypePointerRel : public TypePointer` (type.hh:724) does NOT override
            // `findCompatibleResolve`, so a relative pointer dispatches the inherited
            // `TypePointer::findCompatibleResolve`.  The C++ reads `ct`'s ptrto via a
            // `(TypePointer *)ct` cast, which is valid for a `TypePointerRel` argument
            // too (the `ptrto` member is shared), so use `get_ptr_to` (both kinds).
            DatatypeKind::Pointer { ptrto, .. } | DatatypeKind::PointerRel { ptrto, .. } => {
                if ct.get_metatype() == type_metatype::TYPE_PTR {
                    // ((TypePointer *)ct)->ptrto
                    let ct_ptrto = ct
                        .get_ptr_to()
                        .ok_or_else(|| Datatype::pointer_invariant_err("findCompatibleResolve"))?;
                    return ptrto.find_compatible_resolve(&ct_ptrto);
                }
                Ok(-1)
            }
            // TypeArray::findCompatibleResolve (type.cc:1480-1490).
            DatatypeKind::Array { arrayof, .. } => {
                // C++ nested-if; `&&` preserves short-circuit (the recursive
                // resolve only runs when both resolution flags line up).
                if ct.needs_resolution()
                    && !arrayof.needs_resolution()
                    && ct.find_compatible_resolve(arrayof)? >= 0
                {
                    return Ok(0);
                }
                // C++ `if (arrayof == ct)` — pointer identity against the element.
                if std::ptr::eq(Rc::as_ptr(arrayof), ct as *const Datatype) {
                    return Ok(0);
                }
                Ok(-1)
            }
            // TypeStruct::findCompatibleResolve (type.cc:2300-2311): a struct that
            // needs resolution has exactly one field (field[0]).
            DatatypeKind::Struct { field, .. } => {
                let field_type = &field[0].field_type;
                // C++ nested-if; `&&` preserves short-circuit.
                if ct.needs_resolution()
                    && !field_type.needs_resolution()
                    && ct.find_compatible_resolve(field_type)? >= 0
                {
                    return Ok(0);
                }
                // C++ `if (fieldType == ct) return 0;` — pointer identity.
                if std::ptr::eq(Rc::as_ptr(field_type), ct as *const Datatype) {
                    return Ok(0);
                }
                Ok(-1)
            }
            // TypeUnion::findCompatibleResolve (type.cc:2629-2649).
            DatatypeKind::Union { field } => {
                if !ct.needs_resolution() {
                    for (i, f) in field.iter().enumerate() {
                        // C++ `if (field[i].type == ct && field[i].offset == 0)`.
                        if std::ptr::eq(Rc::as_ptr(&f.field_type), ct as *const Datatype)
                            && f.offset == 0
                        {
                            return Ok(i as int4);
                        }
                    }
                } else {
                    for (i, f) in field.iter().enumerate() {
                        if f.offset != 0 {
                            continue;
                        }
                        let field_type = &f.field_type;
                        if field_type.get_size() != ct.get_size() {
                            continue;
                        }
                        if field_type.needs_resolution() {
                            continue;
                        }
                        if ct.find_compatible_resolve(field_type)? >= 0 {
                            return Ok(i as int4);
                        }
                    }
                }
                Ok(-1)
            }
            // TypePartialUnion::findCompatibleResolve (type.cc:2988-2992):
            // delegate to the container union.
            DatatypeKind::PartialUnion { container, .. } => {
                container.find_compatible_resolve(ct)
            }
            _ => Ok(-1), // base default (type.cc:600-604)
        }
    }

    // -- Struct/Union field access (type.cc:1878-1904, type.hh:626) ---------

    /// Get the `i`-th field of a struct or union (C++ `TypeUnion::getField`
    /// type.hh:626, and struct field access).  Returns `None` for non-composite
    /// kinds or out-of-range indices.
    pub fn get_field(&self, i: int4) -> Option<&TypeField> {
        if i < 0 {
            return None;
        }
        match &self.kind {
            DatatypeKind::Struct { field, .. } => field.get(i as usize),
            DatatypeKind::Union { field } => field.get(i as usize),
            _ => None,
        }
    }

    /// Find the field to use for a truncated read of a struct (C++
    /// `TypeStruct::findTruncation`, type.cc:1878-1892).
    ///
    /// Returns the index of the field containing `[off, off+sz)` and passes back
    /// `newoff` (the offset into that field), or `None` if the requested piece
    /// is not inside a single field.  `op`/`slot` are accepted for signature
    /// parity (the struct override ignores them — only the `TypeUnion` override
    /// consults the `Funcdata` resolution cache, which is a W6 seam).
    ///
    /// The `TypeUnion::findTruncation` (type.cc:2613-2627) and
    /// `TypePartialUnion::findTruncation` (type.cc:2880-2884) overrides need the
    /// `Funcdata` union-resolution cache (`// SEAM(W6)`).
    pub fn find_truncation(
        &self,
        off: int8,
        sz: int4,
        _op: crate::seams::OpId,
        _slot: int4,
    ) -> KunaResult<Option<(int4, int8)>> {
        match &self.kind {
            DatatypeKind::Struct { field, .. } => {
                // C++ `i = getFieldIter(off);` — int8 narrows to int4.
                let i = Datatype::get_field_iter(field, off as int4);
                if i < 0 {
                    return Ok(None);
                }
                let curfield = &field[i as usize];
                let noff = off - curfield.offset as int8;
                // Requested piece spans more than one field.
                if noff + sz as int8 > curfield.field_type.get_size() as int8 {
                    return Ok(None);
                }
                Ok(Some((i, noff)))
            }
            // TypeUnion::findTruncation only returns a cached resolution.
            DatatypeKind::Union { .. } => Err(KunaError::lowlevel(
                "SEAM(W6): TypeUnion::findTruncation needs the Funcdata union-resolution cache",
            )),
            // TypePartialUnion::findTruncation delegates to the container union.
            DatatypeKind::PartialUnion { .. } => Err(KunaError::lowlevel(
                "SEAM(W6): TypePartialUnion::findTruncation needs the Funcdata cache",
            )),
            _ => Ok(None),
        }
    }

    /// Resolve a union field for a truncated read (C++
    /// `TypeUnion::resolveTruncation`, type.cc:2569-2605; and the
    /// `TypePartialUnion` delegate, type.cc:2994-2998).
    ///
    /// SEAM(W6): every path requires the `Funcdata` union-resolution cache and
    /// `ScoreUnionFields` (the union scoring engine), which W6 wires up.
    pub fn resolve_truncation(
        &self,
        _offset: int8,
        _op: crate::seams::OpId,
        _slot: int4,
    ) -> KunaResult<Option<(int4, int8)>> {
        match &self.kind {
            DatatypeKind::Union { .. } | DatatypeKind::PartialUnion { .. } => {
                Err(KunaError::lowlevel(
                    "SEAM(W6): Datatype::resolveTruncation needs Funcdata union scoring",
                ))
            }
            _ => Ok(None),
        }
    }

    // -- TypeEnum machinery (type.cc:1526-1721, type.hh:529-563) -------------

    /// Establish the value -> name map (C++ `TypeEnum::setNameMap`, type.hh:542).
    /// Only meaningful for an enum kind; a no-op for any other kind.
    pub fn set_name_map(&mut self, nmap: std::collections::BTreeMap<u64, String>) {
        if let DatatypeKind::Enum { namemap } = &mut self.kind {
            *namemap = nmap;
        }
    }

    /// Does this enum have a (single) name for the given value (C++
    /// `TypeEnum::hasNamedValue`, type.cc:1526-1530)?  `false` for non-enums.
    ///
    /// The `TypePartialEnum::hasNamedValue` (type.cc:2700-2705) override shifts
    /// `val` left by `8*offset` then delegates to the parent enum.
    pub fn has_named_value(&self, val: u64) -> bool {
        match &self.kind {
            DatatypeKind::Enum { namemap } => namemap.contains_key(&val),
            DatatypeKind::PartialEnum { parent, offset, .. } => {
                // C++ `val <<= 8*offset;` (wrapping u64 shift).
                let shifted = val.wrapping_shl(8u32.wrapping_mul(*offset as u32));
                parent.has_named_value(shifted)
            }
            _ => false,
        }
    }

    /// Recover the named representation of an enumeration value (C++
    /// `TypeEnum::getMatches`, type.cc:1537-1586).
    ///
    /// Returns the matched-name tokens (logically ORed) plus the `complement`
    /// flag (whether the original value or its complement was represented) and
    /// the `shift_amount`.  An empty `match_name` means no representation was
    /// possible.  The `TypePartialEnum::getMatches` (type.cc:2707-2713) override
    /// shifts `val` left by `8*offset`, sets `shift_amount = offset*8`, and
    /// delegates to the parent.
    pub fn get_matches(&self, val: u64) -> KunaResult<EnumRepresentation> {
        match &self.kind {
            DatatypeKind::Enum { namemap } => Ok(Datatype::enum_get_matches(namemap, self.size, val, 0)),
            DatatypeKind::PartialEnum { parent, offset, .. } => {
                // C++ `val <<= 8*offset; rep.shiftAmount = offset * 8;`
                let shifted = val.wrapping_shl(8u32.wrapping_mul(*offset as u32));
                let shift_amount = *offset * 8;
                // The parent must be an enum.
                let namemap = parent
                    .as_enum_namemap()
                    .ok_or_else(|| Datatype::enum_invariant_err("getMatches"))?;
                Ok(Datatype::enum_get_matches(namemap, parent.size, shifted, shift_amount))
            }
            _ => Err(Datatype::enum_invariant_err("getMatches")),
        }
    }

    /// Shared body of `TypeEnum::getMatches` (type.cc:1537-1586).  `size` is the
    /// enum's byte size, `shift_amount` seeds `rep.shiftAmount` (set non-zero by
    /// the partial-enum override).
    fn enum_get_matches(
        namemap: &std::collections::BTreeMap<u64, String>,
        size: int4,
        mut val: u64,
        shift_amount: int4,
    ) -> EnumRepresentation {
        let mut rep = EnumRepresentation { match_name: Vec::new(), complement: false, shift_amount };
        for count in 0..2 {
            let mut allmatch = true;
            if val == 0 {
                // Zero handled specially.
                if let Some(nm) = namemap.get(&val) {
                    rep.match_name.push(nm.clone());
                } else {
                    allmatch = false;
                }
            } else {
                let mut bitsleft = val;
                let mut target = val;
                while target != 0 {
                    // Find named value matching the largest number of most
                    // significant bits in bitsleft.  C++ `namemap.upper_bound(target)`
                    // -> first key strictly greater than target; the BTreeMap
                    // range `(target, ∞)` next() is the same iterator.
                    let next_greater = namemap
                        .range((std::ops::Bound::Excluded(target), std::ops::Bound::Unbounded))
                        .next();
                    // C++ `if (iter == namemap.begin()) break;` — there is no
                    // entry <= target.  upper_bound being begin() means even the
                    // smallest key is > target, so the predecessor walk fails.
                    let pred = match next_greater {
                        Some((k, _)) => {
                            // `--iter`: the largest key strictly less than this key.
                            namemap
                                .range((std::ops::Bound::Unbounded, std::ops::Bound::Excluded(*k)))
                                .next_back()
                        }
                        None => {
                            // upper_bound == end(): predecessor is the last entry,
                            // the biggest named value <= target.
                            namemap.iter().next_back()
                        }
                    };
                    let (curval, curname) = match pred {
                        Some((k, v)) => (*k, v),
                        None => break, // All named values are greater than target
                    };
                    let diff = kuna_base::address::coveringmask(bitsleft ^ curval);
                    if diff >= bitsleft {
                        break; // Could not match most significant bit of bitsleft
                    }
                    if (curval & diff) == 0 {
                        // Found a named value matching at least the MSB of bitsleft.
                        rep.match_name.push(curname.clone()); // Accept the name
                        bitsleft ^= curval; // Remove the bits from bitsleft
                        target = bitsleft;
                    } else {
                        // Not all bits of curval match into bitsleft; restrict
                        // search.  Zero out bits below this and search <= it.
                        target = curval & !diff;
                    }
                }
                allmatch = bitsleft == 0;
            }
            if allmatch {
                // We have a complete representation.
                rep.complement = count == 1;
                return rep;
            }
            // Switch value we are trying to represent (to complement).
            val ^= kuna_base::address::calc_mask(size);
            rep.match_name.clear(); // Clear out old attempt
        }
        // No representation possible — match_name is empty.
        rep
    }

    /// Establish unique enumeration values for a TypeEnum (C++
    /// `TypeEnum::assignValues`, type.cc:1688-1721).  Fills any unassigned names
    /// with the next free value (modulo the size mask) and checks for duplicates;
    /// returns the populated value -> name map.
    ///
    /// `namelist` is the list of names, `vallist` the corresponding values, and
    /// `assignlist` flags which entries the user explicitly assigned.  `size` is
    /// the enum's byte size and `name` is used in the duplicate-error message.
    pub fn assign_values(
        size: int4,
        type_name: &str,
        namelist: &[String],
        vallist: &[u64],
        assignlist: &[bool],
    ) -> KunaResult<std::collections::BTreeMap<u64, String>> {
        let mut nmap: std::collections::BTreeMap<u64, String> = std::collections::BTreeMap::new();
        let mask = kuna_base::address::calc_mask(size);
        let mut maxval: u64 = 0;
        for i in 0..namelist.len() {
            if assignlist[i] {
                // Did the user explicitly set value.
                let mut val = vallist[i];
                if val > maxval {
                    maxval = val;
                }
                val &= mask;
                if nmap.contains_key(&val) {
                    return Err(KunaError::lowlevel(format!(
                        "Enum \"{type_name}\": \"{}\" is a duplicate value",
                        namelist[i]
                    )));
                }
                nmap.insert(val, namelist[i].clone());
            }
        }
        for i in 0..namelist.len() {
            if !assignlist[i] {
                let mut val;
                loop {
                    // C++ `maxval += 1;` (wrapping u64 increment).
                    maxval = maxval.wrapping_add(1);
                    val = maxval;
                    val &= mask;
                    if !nmap.contains_key(&val) {
                        break;
                    }
                }
                nmap.insert(val, namelist[i].clone());
            }
        }
        Ok(nmap)
    }

    // -- TypeStruct / TypeUnion field layout (type.cc:1736-2409) ------------

    /// Copy a list of fields into a struct, establishing size/alignment (C++
    /// `TypeStruct::setFields`, type.cc:1736-1748).  Sets `needs_resolution`
    /// when a single field fills the whole structure, and recomputes `alignSize`.
    /// Replaces the [`DatatypeKind`] payload with [`DatatypeKind::Struct`].
    pub fn set_struct_fields(
        &mut self,
        fd: Vec<TypeField>,
        bit: Vec<TypeBitField>,
        new_size: int4,
        new_align: int4,
    ) {
        self.size = new_size;
        self.alignment = new_align;
        if fd.len() == 1 {
            // A single field that fills the whole structure needs attention.
            if fd[0].field_type.get_size() == self.size {
                self.flags |= flags::needs_resolution;
            }
        }
        self.align_size = Datatype::calc_align_size(self.size, self.alignment);
        self.kind = DatatypeKind::Struct { field: fd, bitfield: bit };
    }

    /// Copy a list of fields into a union, establishing size/alignment (C++
    /// `TypeUnion::setFields`, type.cc:2418-2425).  TypeField `offset` is assumed
    /// to be 0.  Replaces the payload with [`DatatypeKind::Union`].
    pub fn set_union_fields(&mut self, fd: Vec<TypeField>, new_size: int4, new_align: int4) {
        self.size = new_size;
        self.alignment = new_align;
        self.align_size = Datatype::calc_align_size(self.size, self.alignment);
        self.kind = DatatypeKind::Union { field: fd };
    }

    /// Assign offsets to a list of struct fields and bitfields (C++
    /// `TypeStruct::assignFieldOffsets`, type.cc:2365-2409).
    ///
    /// Each field is placed at the next offset aligned to its alignment; the
    /// passed-back `(newSize, newAlign, flags)` are the structure size (aligned),
    /// alignment, and any extra flags (`has_bitfields`).  Mutates `list` (field
    /// `offset`/`ident`) and `bitlist` (byte offset/size/bit position) in place.
    pub fn assign_field_offsets(
        list: &mut [TypeField],
        bitlist: &mut [TypeBitField],
    ) -> KunaResult<(int4, int4, uint4)> {
        let mut next_bit_pos: int4 = -1;
        let mut cur_bit_ind: int4 = -1;
        if !bitlist.is_empty() {
            cur_bit_ind = 0;
            next_bit_pos = bitlist[0].ident;
        }
        let mut offset: int4 = 0;
        let mut new_align: int4 = 1;
        let mut flags: uint4 = 0;
        // C++ indexes `list[pos]` and compares `pos` to `nextBitPos`, so the
        // position counter is load-bearing — a plain index loop transcribes it
        // directly (enumerate would also have to borrow `list[pos]` mutably).
        #[allow(clippy::needless_range_loop)]
        for pos in 0..list.len() {
            if pos as int4 == next_bit_pos {
                Datatype::assign_contiguous_bitfields(
                    bitlist,
                    &mut cur_bit_ind,
                    &mut offset,
                    &mut new_align,
                );
                // Next set of bitfields start at this offset.
                if (cur_bit_ind as usize) < bitlist.len() {
                    next_bit_pos = bitlist[cur_bit_ind as usize].ident;
                }
            }
            let cur_field = &mut list[pos];
            if cur_field.field_type.get_metatype() == type_metatype::TYPE_VOID {
                return Err(KunaError::lowlevel("Illegal field data-type: void"));
            }
            if cur_field.offset != -1 {
                continue;
            }
            let cursize = cur_field.field_type.get_align_size();
            let mut align = cur_field.field_type.get_alignment();
            if align > new_align {
                new_align = align;
            }
            align -= 1;
            if align > 0 && (offset & align) != 0 {
                offset = offset - (offset & align) + (align + 1);
            }
            cur_field.offset = offset;
            cur_field.ident = offset;
            offset += cursize;
            if cur_field.field_type.has_bitfields() {
                flags |= flags::has_bitfields;
            }
        }
        if list.len() as int4 == next_bit_pos {
            // Bitfields after any other fields.
            Datatype::assign_contiguous_bitfields(
                bitlist,
                &mut cur_bit_ind,
                &mut offset,
                &mut new_align,
            );
        }
        if !bitlist.is_empty() && cur_bit_ind as usize != bitlist.len() {
            return Err(KunaError::lowlevel("Malformed bitfield description"));
        }
        if !bitlist.is_empty() {
            flags |= flags::has_bitfields;
        }
        let new_size = Datatype::calc_align_size(offset, new_align);
        Ok((new_size, new_align, flags))
    }

    /// Assign positions to a contiguous subset of bitfields (C++
    /// `TypeStruct::assignContiguousBitfields`, type.cc:2322-2355).  The subset
    /// shares the bitfield `ident` (declaration position); this fills byte
    /// offset/size and the starting bit, then advances `pos`/`offset`/`new_align`.
    fn assign_contiguous_bitfields(
        bitlist: &mut [TypeBitField],
        pos: &mut int4,
        offset: &mut int4,
        new_align: &mut int4,
    ) {
        let mut total_size: int4 = 0;
        let start_ind = *pos;
        let next_bit_pos = bitlist[*pos as usize].ident;
        // Calculate total number of bits in contiguous bitfields.
        while (*pos as usize) < bitlist.len() && bitlist[*pos as usize].ident == next_bit_pos {
            total_size += bitlist[*pos as usize].num_bits;
            *pos += 1;
        }
        // Align the offset for bitfields.
        let mut align = bitlist[start_ind as usize].field_type.get_alignment();
        if align > *new_align {
            *new_align = align;
        }
        align -= 1;
        if align > 0 && (*offset & align) != 0 {
            *offset = *offset - (*offset & align) + (align + 1);
        }
        total_size = (total_size + 7) / 8; // Number of bytes for this set
        let mut lsb: int4 = 0;
        for i in start_ind..*pos {
            let bf = &mut bitlist[i as usize];
            bf.byte_offset = *offset; // Set byte offset
            bf.byte_size = total_size;
            bf.least_sig_bit = lsb; // Establish bit position
            lsb += bf.num_bits;
            bf.ident = i; // Identifier is position within bitfield list
        }
        *offset += total_size;
        if bitlist[start_ind as usize].is_big_endian && (*pos - start_ind) > 1 {
            // Big-endian bitfields are assigned LSB to MSB but the data-type
            // expects MSB-to-LSB order, so reverse after assignment is complete.
            bitlist[start_ind as usize..*pos as usize].reverse();
        }
    }

    /// Assign offsets to a list of union fields (C++
    /// `TypeUnion::assignFieldOffsets`, type.cc:2651-2673).  Every field is at
    /// offset 0; the union size is the max field size, alignment the max field
    /// alignment.  `type_name` is used in the validation error messages.
    pub fn assign_union_field_offsets(
        list: &mut [TypeField],
        type_name: &str,
    ) -> KunaResult<(int4, int4)> {
        let mut new_size: int4 = 0;
        let mut new_align: int4 = 1;
        for f in list.iter_mut() {
            let ct = &f.field_type;
            // Sanity checks on the field.
            if ct.get_metatype() == type_metatype::TYPE_VOID {
                return Err(KunaError::lowlevel(format!(
                    "Bad field data-type for union: {type_name}"
                )));
            }
            if f.name.is_empty() {
                return Err(KunaError::lowlevel(format!(
                    "Bad field name for union: {type_name}"
                )));
            }
            f.offset = 0;
            let end = ct.get_size();
            if end > new_size {
                new_size = end;
            }
            let cur_align = ct.get_alignment();
            if cur_align > new_align {
                new_align = cur_align;
            }
        }
        Ok((new_size, new_align))
    }

    // -- Print / encode (type.hh:250,289) — W6 ------------------------------

    /// Print a description of the type to a stream (C++ `printRaw`).
    ///
    /// SEAM(W6): printing is part of the type-rendering subsystem (W6/W9).
    pub fn print_raw(&self) -> KunaResult<String> {
        Err(KunaError::lowlevel(
            "SEAM(W6): Datatype::printRaw not yet ported",
        ))
    }
}

/// Class describing how a particular enumeration value is constructed using
/// tokens (C++ `TypeEnum::Representation`, type.hh:532-538).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct EnumRepresentation {
    /// Name tokens that are ORed together (C++ `matchname`)
    pub match_name: Vec<String>,
    /// If `true`, bitwise complement value after ORing (C++ `complement`)
    pub complement: bool,
    /// Number of bits to left-shift final value (C++ `shiftAmount`)
    pub shift_amount: int4,
}

// =============================================================================
// TypeFactory handle trait (type.hh:827-936) — the rule/action call surface
// =============================================================================

/// The container-class handle for all [`Datatype`] objects in an `Architecture`
/// (C++ `TypeFactory`, type.hh:827-936) — **interface only**.
///
/// The decompiler's rules/actions reach the type system through `glb->types`,
/// whose concrete construction (the sorted `tree`/`nametree` caches, the
/// `typecache` matrix, decode) is heavy W6 logic.  This trait freezes the
/// *method set* the W8 rule porters call so they can be written against a
/// `&dyn TypeFactory` (or a concrete W6 factory) without that factory existing
/// yet.  Every method is a `// SEAM(W6)` — a W6 type factory implements them;
/// until then a caller that needs one is itself seam-noted.
///
/// The method list is the union of the `TypeFactory::getX`/`concretize`/
/// `getExactPiece`/size-query surface that `ruleaction.cc`/`coreaction.cc`/
/// `cast.cc`/`typeop.cc` invoke (the `getBase`/`getTypePointer`/`resizePointer`/
/// `getTypeCode`/`getExactPiece`/`getBaseNoChar`/`getTypeVoid`/`getTypePointerRel`/
/// `getSizeOfPointer`/`getSizeOfInt`/`getArch` calls catalogued from those files).
pub trait TypeFactory {
    // -- Size queries (type.hh:880-885) -------------------------------------

    /// Get the size of the default "int" (C++ `getSizeOfInt`).
    fn get_size_of_int(&self) -> int4;
    /// Get the size of the default "long" (C++ `getSizeOfLong`).
    fn get_size_of_long(&self) -> int4;
    /// Get the size of the default "char" (C++ `getSizeOfChar`).
    fn get_size_of_char(&self) -> int4;
    /// Get the size of the default "wchar_t" (C++ `getSizeOfWChar`).
    fn get_size_of_wchar(&self) -> int4;
    /// Get the size of pointers (C++ `getSizeOfPointer`).
    fn get_size_of_pointer(&self) -> int4;
    /// Get size of alternate pointers, or 0 (C++ `getSizeOfAltPointer`).
    fn get_size_of_alt_pointer(&self) -> int4;
    /// Get data-type alignment based on size (C++ `getAlignment`).
    fn get_alignment(&self, size: uint4) -> KunaResult<int4>;
    /// Get the aligned size of a primitive data-type (C++ `getPrimitiveAlignSize`).
    fn get_primitive_align_size(&self, size: uint4) -> KunaResult<int4>;

    // -- Atomic / core getters (type.hh:894-899) ----------------------------

    /// Get the "void" data-type (C++ `getTypeVoid`).
    fn get_type_void(&self) -> KunaResult<Rc<Datatype>>;
    /// Get atomic type excluding "char" (C++ `getBaseNoChar`).
    fn get_base_no_char(&self, s: int4, m: type_metatype) -> KunaResult<Rc<Datatype>>;
    /// Get atomic type (C++ `getBase(int4,type_metatype)`).
    fn get_base(&self, s: int4, m: type_metatype) -> KunaResult<Rc<Datatype>>;
    /// Get named atomic type (C++ `getBase(int4,type_metatype,const string&)`).
    fn get_base_named(&self, s: int4, m: type_metatype, n: &str) -> KunaResult<Rc<Datatype>>;
    /// Get a character data-type by size (C++ `getTypeChar(int4)`).
    fn get_type_char(&self, s: int4) -> KunaResult<Rc<Datatype>>;
    /// Get an "anonymous" function data-type (C++ `getTypeCode(void)`).
    fn get_type_code(&self) -> KunaResult<Rc<Datatype>>;

    // -- Pointer construction (type.hh:900-902,913-916) ---------------------

    /// Construct a pointer data-type, stripping an ARRAY level (C++
    /// `getTypePointerStripArray`).
    fn get_type_pointer_strip_array(
        &self,
        s: int4,
        pt: Rc<Datatype>,
        ws: uint4,
    ) -> KunaResult<Rc<Datatype>>;
    /// Construct an absolute pointer data-type (C++ `getTypePointer`).
    fn get_type_pointer(&self, s: int4, pt: Rc<Datatype>, ws: uint4) -> KunaResult<Rc<Datatype>>;
    /// Construct a named pointer data-type (C++ `getTypePointer(...,const string&)`).
    fn get_type_pointer_named(
        &self,
        s: int4,
        pt: Rc<Datatype>,
        ws: uint4,
        n: &str,
    ) -> KunaResult<Rc<Datatype>>;
    /// Build a resized pointer based on the given pointer (C++ `resizePointer`).
    fn resize_pointer(&self, ptr: Rc<Datatype>, new_size: int4) -> KunaResult<Rc<Datatype>>;
    /// Get pointer offset relative to a container (C++ `getTypePointerRel`
    /// 3-arg form).
    fn get_type_pointer_rel(
        &self,
        parent_ptr: Rc<Datatype>,
        ptr_to: Rc<Datatype>,
        off: int4,
    ) -> KunaResult<Rc<Datatype>>;
    /// Get a relative pointer with explicit size/space/name (C++
    /// `getTypePointerRel` 6-arg form).
    #[allow(clippy::too_many_arguments)]
    fn get_type_pointer_rel_full(
        &self,
        sz: int4,
        parent: Rc<Datatype>,
        ptr_to: Rc<Datatype>,
        ws: int4,
        off: int4,
        nm: &str,
    ) -> KunaResult<Rc<Datatype>>;
    /// Construct a pointer into a given address space (C++
    /// `getTypePointerWithSpace`).
    fn get_type_pointer_with_space(
        &self,
        ptr_to: Rc<Datatype>,
        spc: Rc<AddrSpace>,
        nm: &str,
    ) -> KunaResult<Rc<Datatype>>;

    /// Given a containing data-type and a byte offset into it, recover the
    /// data-type that a relative pointer at that offset points directly at (C++
    /// `TypePointerRel::getPtrToFromParent`, type.cc:3157-3171).  Walks
    /// `getSubType` down the container until the offset is consumed; falls back
    /// to `getBase(1,TYPE_UNKNOWN)` when the offset is non-positive or escapes
    /// the container.  A default body, written in terms of `get_sub_type` (on
    /// `Datatype`) and `get_base` (on the factory), so every factory shares it.
    fn get_ptr_to_from_parent(
        &self,
        base: Rc<Datatype>,
        off: int4,
    ) -> KunaResult<Rc<Datatype>> {
        if off > 0 {
            let mut curoff: int8 = off as int8;
            let mut base = base;
            // do { base = base->getSubType(curoff,&curoff); }
            //   while (curoff != 0 && base != 0);
            loop {
                let (next, newoff) = base.get_sub_type(curoff)?;
                curoff = newoff;
                match next {
                    None => {
                        // base became null: getBase(1, TYPE_UNKNOWN).
                        return self.get_base(1, type_metatype::TYPE_UNKNOWN);
                    }
                    Some(next_ct) => {
                        base = next_ct;
                        if curoff == 0 {
                            break;
                        }
                    }
                }
            }
            Ok(base)
        } else {
            self.get_base(1, type_metatype::TYPE_UNKNOWN)
        }
    }

    // -- Composite construction (type.hh:903-912) ---------------------------

    /// Construct an array data-type (C++ `getTypeArray`).
    fn get_type_array(&self, as_: int4, ao: Rc<Datatype>) -> KunaResult<Rc<Datatype>>;
    /// Create an (empty) structure (C++ `getTypeStruct`).
    fn get_type_struct(&self, n: &str) -> KunaResult<Rc<Datatype>>;
    /// Create a partial structure (C++ `getTypePartialStruct`).
    fn get_type_partial_struct(
        &self,
        contain: Rc<Datatype>,
        off: int4,
        sz: int4,
    ) -> KunaResult<Rc<Datatype>>;
    /// Create an (empty) union (C++ `getTypeUnion`).
    fn get_type_union(&self, n: &str) -> KunaResult<Rc<Datatype>>;
    /// Create a partial union (C++ `getTypePartialUnion`).
    fn get_type_partial_union(
        &self,
        contain: Rc<Datatype>,
        off: int4,
        sz: int4,
    ) -> KunaResult<Rc<Datatype>>;
    /// Create a partial enumeration (C++ `getTypePartialEnum`).
    fn get_type_partial_enum(
        &self,
        contain: Rc<Datatype>,
        off: int4,
        sz: int4,
    ) -> KunaResult<Rc<Datatype>>;
    /// Create an (empty) enumeration (C++ `getTypeEnum`).
    fn get_type_enum(&self, n: &str) -> KunaResult<Rc<Datatype>>;
    /// Create a "spacebase" type (C++ `getTypeSpacebase`).
    fn get_type_spacebase(&self, id: Rc<AddrSpace>, addr: &Address) -> KunaResult<Rc<Datatype>>;

    // -- Resizing / piece extraction (type.hh:917-918) ----------------------

    /// Build a resized integer based on the given integer (C++ `resizeInteger`).
    fn resize_integer(&self, ct: Rc<Datatype>, new_size: int4) -> KunaResult<Rc<Datatype>>;
    /// Get the data-type associated with a piece of a structured data-type (C++
    /// `getExactPiece`).
    fn get_exact_piece(
        &self,
        ct: Rc<Datatype>,
        offset: int4,
        size: int4,
    ) -> KunaResult<Option<Rc<Datatype>>>;

    // -- Lookup / conversion (type.hh:887,922) ------------------------------

    /// Return type of given name (C++ `findByName`).
    fn find_by_name(&self, n: &str) -> KunaResult<Option<Rc<Datatype>>>;
    /// Convert given data-type to concrete form (C++ `concretize`).
    fn concretize(&self, ct: Rc<Datatype>) -> KunaResult<Rc<Datatype>>;

    // -- In-place construction mutators (type.cc:3919-4019, 4292, 4618-4655) --
    // These re-key an already-interned type and return the completed `Rc` (see
    // the impl on `TypeFactoryImpl` for the Rc re-keying model).  Default bodies
    // are provided so the lightweight test/seam factories (which never run the
    // C-declaration construction paths) need not implement them; the concrete
    // `TypeFactoryImpl` overrides every one.

    /// Whether the default data space is big-endian (the bitfield-layout bit C++
    /// reads via `getDefaultDataSpace()->isBigEndian()`).
    fn is_big_endian(&self) -> bool {
        false
    }
    /// Rename an interned data-type (C++ `setName`).
    fn set_name(&self, _ct: &Rc<Datatype>, _n: &str) -> KunaResult<Rc<Datatype>> {
        Err(KunaError::lowlevel("TypeFactory::setName not supported by this factory"))
    }
    /// Force the integer display format of an interned data-type (C++
    /// `TypeFactory::setDisplayFormat` → `Datatype::setDisplayFormat`,
    /// type.cc:201).  Returns the re-interned type carrying the new format.
    fn set_display_format(&self, _ct: &Rc<Datatype>, _format: uint4) -> KunaResult<Rc<Datatype>> {
        Err(KunaError::lowlevel("TypeFactory::setDisplayFormat not supported by this factory"))
    }
    /// Assign fields to an incomplete struct (C++ `assignRawFields(TypeStruct*)`).
    fn assign_raw_fields_struct(
        &self,
        _ct: &Rc<Datatype>,
        _fd: Vec<TypeField>,
        _bit: Vec<TypeBitField>,
    ) -> KunaResult<Rc<Datatype>> {
        Err(KunaError::lowlevel("TypeFactory::assignRawFields not supported by this factory"))
    }
    /// Assign fields to an incomplete union (C++ `assignRawFields(TypeUnion*)`).
    fn assign_raw_fields_union(
        &self,
        _ct: &Rc<Datatype>,
        _fd: Vec<TypeField>,
    ) -> KunaResult<Rc<Datatype>> {
        Err(KunaError::lowlevel("TypeFactory::assignRawFields not supported by this factory"))
    }
    /// Install enum value/name map on an interned enum (C++ `setEnumValues`).
    fn set_enum_values(
        &self,
        _ct: &Rc<Datatype>,
        _nmap: std::collections::BTreeMap<u64, String>,
    ) -> KunaResult<Rc<Datatype>> {
        Err(KunaError::lowlevel("TypeFactory::setEnumValues not supported by this factory"))
    }
    /// Create (or find) a typedef of a data-type (C++ `getTypedef`).
    fn get_typedef(
        &self,
        _ct: &Rc<Datatype>,
        _name: &str,
        _id: uint8,
        _format: uint4,
    ) -> KunaResult<Rc<Datatype>> {
        Err(KunaError::lowlevel("TypeFactory::getTypedef not supported by this factory"))
    }
    /// Remove a data-type from the container (C++ `destroyType`).
    fn destroy_type(&self, _ct: &Rc<Datatype>) -> KunaResult<()> {
        Err(KunaError::lowlevel("TypeFactory::destroyType not supported by this factory"))
    }
}

// =============================================================================
// TypeFactoryImpl — the concrete container (type.cc:3565-end) — item
// `w6-s5-type-3`
// =============================================================================

/// Ordering key for the factory's interning tree (C++ `DatatypeSet`,
/// `set<Datatype*,DatatypeCompare>`, type.hh:360-378).
///
/// `DatatypeCompare::operator()` orders by `compareDependency` and breaks ties on
/// `getId()`.  `compareDependency` is fallible in the Rust port only for the
/// not-yet-ported `TypeCode` *prototype recursion* (a complete code data-type
/// with a bound `FuncProto`, `// SEAM(W6)`); every data-type the construction
/// getters intern is free of that case, so the `Err` branch is unreachable in
/// practice.  To keep a *total* `Ord` (required by `BTreeSet`) we fall back to the
/// id tie-break on the (unreachable here) error, which is consistent and
/// deterministic.  The `Rc::as_ptr` identity used inside `compareDependency` is
/// stable because every interned data-type is a unique allocation, exactly as the
/// C++ tree relies on unique `Datatype*` addresses.
#[derive(Clone)]
struct TreeKey(Rc<Datatype>);

impl PartialEq for TreeKey {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}
impl Eq for TreeKey {}
impl PartialOrd for TreeKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for TreeKey {
    fn cmp(&self, other: &Self) -> Ordering {
        // C++ DatatypeCompare: compareDependency, then getId().
        let res = self.0.compare_dependency(&other.0).unwrap_or(0);
        match res.cmp(&0) {
            Ordering::Equal => self.0.get_id().cmp(&other.0.get_id()),
            o => o,
        }
    }
}

/// Mutable interning state of a [`TypeFactoryImpl`] (the C++ `tree`, `nametree`,
/// `typecache*`, `charcache`).
struct FactoryStore {
    /// Datatypes within this factory, sorted by `compareDependency` (C++ `tree`).
    tree: BTreeSet<TreeKey>,
    /// Cross-reference by (name, id) for named look-ups (C++ `nametree`, a
    /// `set<Datatype*,DatatypeNameCompare>`).  Modeled as a sorted vector keyed
    /// on `(name, id)`; only types with a non-zero id are inserted, exactly as
    /// C++ `insert` only cross-references named types.
    nametree: Vec<Rc<Datatype>>,
    /// Matrix of the most common atomic data-types (C++ `typecache[9][8]`).
    typecache: [[Option<Rc<Datatype>>; 8]; 9],
    /// Specially cached 10-byte float type (C++ `typecache10`).
    typecache10: Option<Rc<Datatype>>,
    /// Specially cached 16-byte float type (C++ `typecache16`).
    typecache16: Option<Rc<Datatype>>,
    /// Same dimensions as char but acts/displays as an INT (C++ `type_nochar`).
    type_nochar: Option<Rc<Datatype>>,
    /// Cached character data-types (C++ `charcache[5]`).
    charcache: [Option<Rc<Datatype>>; 5],
}

impl FactoryStore {
    fn new() -> FactoryStore {
        FactoryStore {
            tree: BTreeSet::new(),
            nametree: Vec::new(),
            typecache: std::array::from_fn(|_| std::array::from_fn(|_| None)),
            typecache10: None,
            typecache16: None,
            type_nochar: None,
            charcache: std::array::from_fn(|_| None),
        }
    }
}

/// The container-class for all [`Datatype`] objects in an `Architecture`
/// (C++ `TypeFactory`, type.cc:3565-end) — the concrete implementation of the
/// [`TypeFactory`] handle trait.
///
/// The C++ class is a stateful, mutating container: nearly every `getX`
/// constructor calls `findAdd`, which interns a structurally-identical instance
/// (de-duplicating by `compareDependency`) and otherwise clones-and-inserts.  In
/// Rust this requires interior mutability, so the interning state lives behind a
/// [`RefCell`]; the trait's `&self` methods borrow it mutably for the duration of
/// a single construction.  Interned data-types are shared as `Rc<Datatype>` and
/// never mutated after insertion in the paths this item ports (the
/// `setFields`/`setName`/`recalcPointerSubmeta` mutate-in-place paths belong to
/// the decode subsystem — `// SEAM(W6)`).
///
/// Size configuration (the C++ members `sizeOfInt`/`sizeOfLong`/… and the
/// `alignMap`) is carried as plain fields, set by [`TypeFactoryImpl::setup_sizes`]
/// or [`TypeFactoryImpl::decode_data_organization`].  The big-endian truncation
/// flag (the only thing `calcTruncate` reads from the `Architecture`) is carried
/// as `truncate_big_endian`, set from the default data space when known.
pub struct TypeFactoryImpl {
    /// Size of the core "int" data-type (C++ `sizeOfInt`).
    size_of_int: Cell<int4>,
    /// Size of the core "long" data-type (C++ `sizeOfLong`).
    size_of_long: Cell<int4>,
    /// Size of the core "char" data-type (C++ `sizeOfChar`).
    size_of_char: Cell<int4>,
    /// Size of the core "wchar_t" data-type (C++ `sizeOfWChar`).
    size_of_wchar: Cell<int4>,
    /// Size of pointers into the default data space (C++ `sizeOfPointer`).
    size_of_pointer: Cell<int4>,
    /// Size of alternate pointers, or 0 (C++ `sizeOfAltPointer`).
    size_of_alt_pointer: Cell<int4>,
    /// Size of an enumerated type (C++ `enumsize`).
    enumsize: Cell<int4>,
    /// Default enumeration meta-type when parsing C (C++ `enumtype`).
    enumtype: Cell<type_metatype>,
    /// Alignment of primitive data-types keyed by their size (C++ `alignMap`).
    align_map: RefCell<Vec<int4>>,
    /// Maximum "integer" size before a getBase request becomes an array of
    /// unknown bytes (C++ `glb->max_basetype_size`).
    max_basetype_size: Cell<int4>,
    /// Whether the default data space is big-endian (drives `calcTruncate`'s
    /// `truncate_bigendian` flag; C++ reads `glb->getDefaultDataSpace()`).
    truncate_big_endian: Cell<bool>,
    /// The interning state (C++ `tree`/`nametree`/`typecache*`/`charcache`).
    store: RefCell<FactoryStore>,
}

impl Default for TypeFactoryImpl {
    fn default() -> Self {
        Self::new()
    }
}

impl TypeFactoryImpl {
    /// Construct an empty container (C++ `TypeFactory::TypeFactory`,
    /// type.cc:3565-3578): all sizes 0, the cache cleared.
    pub fn new() -> TypeFactoryImpl {
        TypeFactoryImpl {
            size_of_int: Cell::new(0),
            size_of_long: Cell::new(0),
            size_of_char: Cell::new(0),
            size_of_wchar: Cell::new(0),
            size_of_pointer: Cell::new(0),
            size_of_alt_pointer: Cell::new(0),
            enumsize: Cell::new(0),
            enumtype: Cell::new(type_metatype::TYPE_ENUM_UINT),
            align_map: RefCell::new(Vec::new()),
            max_basetype_size: Cell::new(0),
            truncate_big_endian: Cell::new(false),
            store: RefCell::new(FactoryStore::new()),
        }
    }

    // -- Size configuration --------------------------------------------------

    /// Set the maximum base-type size before `getBase` builds an unknown-byte
    /// array (the C++ `glb->max_basetype_size`).
    pub fn set_max_basetype_size(&self, sz: int4) {
        self.max_basetype_size.set(sz);
    }

    /// Set whether the default data space is big-endian (drives the
    /// `truncate_bigendian` flag during `calcTruncate`).
    pub fn set_truncate_big_endian(&self, big: bool) {
        self.truncate_big_endian.set(big);
    }

    /// Provide default alignments for data-types (C++
    /// `TypeFactory::setDefaultAlignmentMap`, type.cc:5171-5183).
    pub fn set_default_alignment_map(&self) {
        let mut m = vec![1; 9];
        m[1] = 1;
        m[2] = 2;
        m[3] = 2;
        m[4] = 4;
        m[5] = 4;
        m[6] = 4;
        m[7] = 4;
        m[8] = 8;
        *self.align_map.borrow_mut() = m;
    }

    /// Set up default values for the core sizes and alignment/enum config (a
    /// faithful transcription of `TypeFactory::setupSizes`, type.cc:3596-3629,
    /// with the `Architecture`-derived defaults supplied by the caller instead of
    /// queried — the `glb` accessors the C++ reads are W4 surfaces).
    ///
    /// `stack_pointer_size` is the stack pointer width (or `None` if there is no
    /// stack space — `// SEAM(W4)` `glb->getStackSpace`); `default_data_addr_size`
    /// is the default data space address size (C++
    /// `glb->getDefaultDataSpace()->getAddrSize()`); `default_size` is the
    /// architecture default size (C++ `glb->getDefaultSize()`).
    pub fn setup_sizes(
        &self,
        stack_pointer_size: Option<int4>,
        default_data_addr_size: int4,
        default_size: int4,
    ) {
        if self.size_of_int.get() == 0 {
            let mut s = 1; // Default if we can't find a better value
            if let Some(sp) = stack_pointer_size {
                s = sp;
                if s > 4 {
                    s = 4; // "int" is rarely bigger than 4 bytes
                }
            }
            self.size_of_int.set(s);
        }
        if self.size_of_long.get() == 0 {
            self.size_of_long
                .set(if self.size_of_int.get() == 4 { 8 } else { self.size_of_int.get() });
        }
        if self.size_of_char.get() == 0 {
            self.size_of_char.set(1);
        }
        if self.size_of_wchar.get() == 0 {
            self.size_of_wchar.set(2);
        }
        if self.size_of_pointer.get() == 0 {
            self.size_of_pointer.set(default_data_addr_size);
        }
        // SEAM(W4): the segmented far-pointer adjustment (glb->getSegmentOp) is a
        // W4 surface; without it sizeOfAltPointer stays 0, as for a flat space.
        if self.align_map.borrow().is_empty() {
            self.set_default_alignment_map();
        }
        if self.enumsize.get() == 0 {
            self.enumsize.set(default_size);
            self.enumtype.set(type_metatype::TYPE_ENUM_UINT);
        }
    }

    // -- Alignment queries (type.cc:3774-3798) -------------------------------

    /// Get data-type alignment based on size (C++ `TypeFactory::getAlignment`).
    fn alignment(&self, size: uint4) -> KunaResult<int4> {
        let m = self.align_map.borrow();
        if size as usize >= m.len() {
            if m.is_empty() {
                return Err(KunaError::lowlevel("TypeFactory alignment map not initialized"));
            }
            return Ok(m[m.len() - 1]);
        }
        Ok(m[size as usize])
    }

    /// Get the aligned size of a primitive data-type (C++
    /// `TypeFactory::getPrimitiveAlignSize`).
    fn primitive_align_size(&self, size: uint4) -> KunaResult<int4> {
        let align = self.alignment(size)?;
        let mut size = size;
        // C++ `uint4 mod = size % align;` — `align` is int4 but positive here.
        let mod_ = size % align as uint4;
        if mod_ != 0 {
            size += align as uint4 - mod_;
        }
        Ok(size as int4)
    }

    // -- Interning core (type.cc:3804-3917) ----------------------------------

    /// Look up a data-type locally by name and id (C++
    /// `TypeFactory::findByIdLocal`, type.cc:3804-3822).  When `id == 0` the name
    /// may be non-unique; the first type with a matching name is returned.
    fn find_by_id_local(&self, n: &str, id: uint8) -> Option<Rc<Datatype>> {
        let store = self.store.borrow();
        if id != 0 {
            // Exact (name,id) match.
            store
                .nametree
                .iter()
                .find(|dt| dt.name == n && dt.id == id)
                .map(Rc::clone)
        } else {
            // First type with this name (nametree is kept ordered by (name,id)).
            store
                .nametree
                .iter()
                .filter(|dt| dt.name == n)
                .min_by(|a, b| a.name.cmp(&b.name).then(a.id.cmp(&b.id)))
                .map(Rc::clone)
        }
    }

    /// Search by name and/or id, applying the variable-length size hash (C++
    /// `TypeFactory::findById`, type.cc:3832-3839).
    fn find_by_id(&self, n: &str, id: uint8, sz: int4) -> Option<Rc<Datatype>> {
        let id = if sz > 0 { Datatype::hash_size(id, sz) } else { id };
        self.find_by_id_local(n, id)
    }

    /// Find a data-type without reference to its name, using the functional
    /// comparators (C++ `TypeFactory::findNoName`, type.cc:3855-3864).  The match
    /// is the tree element that compares `compareDependency`-equal to `ct`.
    fn find_no_name(&self, ct: &Rc<Datatype>) -> Option<Rc<Datatype>> {
        let store = self.store.borrow();
        store.tree.get(&TreeKey(Rc::clone(ct))).map(|k| Rc::clone(&k.0))
    }

    /// Insert a fully-built data-type pointer into the cross-reference sets (C++
    /// `TypeFactory::insert`, type.cc:3868-3884).  A duplicate id in `tree` is an
    /// error (the C++ throws after printing the clash); named types are also
    /// cross-referenced in `nametree`.
    fn insert(&self, newtype: Rc<Datatype>) -> KunaResult<()> {
        let mut store = self.store.borrow_mut();
        let key = TreeKey(Rc::clone(&newtype));
        if store.tree.contains(&key) {
            return Err(KunaError::lowlevel(format!(
                "Shared type id: {:x}",
                newtype.get_id()
            )));
        }
        store.tree.insert(key);
        if newtype.id != 0 {
            // Keep nametree ordered by (name,id) (C++ DatatypeNameCompare).
            let pos = store
                .nametree
                .binary_search_by(|dt| {
                    dt.name.cmp(&newtype.name).then(dt.id.cmp(&newtype.id))
                })
                .unwrap_or_else(|e| e);
            store.nametree.insert(pos, newtype);
        }
        Ok(())
    }

    /// Find a structurally-identical data-type in this container or add a clone
    /// of it (C++ `TypeFactory::findAdd`, type.cc:3890-3917).  Named types are
    /// matched by (name,id) — a mismatched redefinition is an error; unnamed types
    /// are matched by the functional comparator.  A newly added type with
    /// unassigned alignment (`alignment < 0`) gets its primitive alignment
    /// computed from the alignment map.
    fn find_add(&self, mut ct: Datatype) -> KunaResult<Rc<Datatype>> {
        if !ct.name.is_empty() {
            // If there is a name there must be an id.
            if ct.id == 0 {
                return Err(KunaError::lowlevel(format!(
                    "Datatype must have a valid id: {}",
                    ct.name
                )));
            }
            if let Some(res) = self.find_by_id_local(&ct.name, ct.id) {
                if res.compare_dependency(&ct)? != 0 {
                    return Err(KunaError::lowlevel(format!(
                        "Trying to alter definition of type: {}",
                        ct.name
                    )));
                }
                return Ok(res);
            }
        } else {
            let probe = Rc::new(ct.clone());
            if let Some(res) = self.find_no_name(&probe) {
                return Ok(res); // Found it
            }
        }
        // Add the new type to the trees (C++ clones; here `ct` is already owned).
        if ct.alignment < 0 {
            ct.align_size = self.primitive_align_size(ct.size as uint4)?;
            ct.alignment = self.alignment(ct.align_size as uint4)?;
        }
        let newtype = Rc::new(ct);
        self.insert(Rc::clone(&newtype))?;
        Ok(newtype)
    }

    // -- In-place mutation of interned types (type.cc:3919-4019, 4292, 4618-4655) --
    //
    // The C++ container mutates an *already-interned* `Datatype*` in place via raw
    // pointers (every reference shares the one allocation): `setName`, `setFields`,
    // `setEnumValues`, `getTypedef`, `destroyType`.  The Rust port shares interned
    // types as `Rc<Datatype>`, so an "in-place" mutation is modelled as: remove the
    // old (immutable) `Rc` from the trees by its *old* key, build a mutated clone,
    // and re-insert under its *new* key, returning the new `Rc`.  The console
    // construction flow (`parse_C` / `map`) always threads the *returned* `Rc`
    // forward (it never re-uses the pre-mutation stub), so the single-allocation
    // behavior the C++ relies on is preserved for these store-write paths.

    /// Whether the default data space is big-endian (the bit
    /// `getDefaultDataSpace()->isBigEndian()` the struct builder reads for
    /// bitfield layout).  Set by [`Self::set_truncate_big_endian`] at init.
    fn is_big_endian_impl(&self) -> bool {
        self.truncate_big_endian.get()
    }

    /// Remove an interned data-type from the trees by its current key (the
    /// `tree.erase(ct)` + optional `nametree.erase(ct)` the in-place mutators run
    /// before re-inserting).  `also_name` removes the name cross-reference too
    /// (only [`Self::set_name`] / [`Self::destroy_type`] re-key the name).
    fn erase_interned(&self, ct: &Rc<Datatype>, also_name: bool) {
        let mut store = self.store.borrow_mut();
        store.tree.remove(&TreeKey(Rc::clone(ct)));
        if also_name && ct.id != 0 {
            // C++ DatatypeNameCompare keys on (name,id); erase the matching entry.
            store
                .nametree
                .retain(|dt| !(dt.name == ct.name && dt.id == ct.id && Rc::ptr_eq(dt, ct)));
        }
    }

    /// C++ `TypeFactory::setName` (type.cc:3923-3937): rename an interned
    /// data-type, re-keying both trees.  Returns the renamed (new) `Rc`.
    fn set_name_impl(&self, ct: &Rc<Datatype>, n: &str) -> KunaResult<Rc<Datatype>> {
        self.erase_interned(ct, ct.id != 0); // nametree.erase only if it had an id
        let mut newct = (**ct).clone();
        newct.name = n.to_string();
        newct.display_name = n.to_string();
        if newct.id == 0 {
            newct.id = Datatype::hash_name(n);
        }
        let newrc = Rc::new(newct);
        self.insert(Rc::clone(&newrc))?; // tree.insert + nametree.insert
        Ok(newrc)
    }

    /// C++ `TypeFactory::setDisplayFormat(ct,format)` (type.cc) → `ct->setDisplayFormat`.
    /// The interned `Datatype` is immutable (shared `Rc`), so this mirrors
    /// [`set_name_impl`]: erase the interned instance, clone-with-the-new-format,
    /// and re-intern.  Subsequent `findByName`/`map addr` look-ups resolve the
    /// updated type, exactly as the C++ in-place mutation makes the next lookup
    /// see the new `dispflags`.
    fn set_display_format_impl(
        &self,
        ct: &Rc<Datatype>,
        format: uint4,
    ) -> KunaResult<Rc<Datatype>> {
        self.erase_interned(ct, ct.id != 0);
        let mut newct = (**ct).clone();
        newct.set_display_format(format);
        let newrc = Rc::new(newct);
        self.insert(Rc::clone(&newrc))?;
        Ok(newrc)
    }

    /// C++ `TypeFactory::assignRawFields(TypeStruct*,...)` (type.cc:4618-4626):
    /// compute the struct field offsets/size/alignment then `setFields`.  Returns
    /// the completed (new) struct `Rc`.
    fn assign_raw_fields_struct_impl(
        &self,
        ct: &Rc<Datatype>,
        mut fd: Vec<TypeField>,
        mut bit: Vec<TypeBitField>,
    ) -> KunaResult<Rc<Datatype>> {
        // TypeStruct::assignFieldOffsets(fd,bit,newSize,newAlign,flags).
        let (new_size, new_align, extra_flags) = Datatype::assign_field_offsets(&mut fd, &mut bit)?;
        self.set_fields_struct(ct, fd, bit, new_size, new_align, extra_flags)
    }

    /// C++ `TypeFactory::assignRawFields(TypeUnion*,...)` (type.cc:4633-4640):
    /// compute the union field offsets/size/alignment then `setFields`.  Returns
    /// the completed (new) union `Rc`.
    fn assign_raw_fields_union_impl(
        &self,
        ct: &Rc<Datatype>,
        mut fd: Vec<TypeField>,
    ) -> KunaResult<Rc<Datatype>> {
        // TypeUnion::assignFieldOffsets(fd,newSize,newAlign,ct).
        let (new_size, new_align) = Datatype::assign_union_field_offsets(&mut fd, ct.get_name())?;
        self.set_fields_union(ct, fd, new_size, new_align, 0)
    }

    /// C++ `TypeFactory::setFields(...,TypeStruct*,...)` (type.cc:3960-3973):
    /// re-key a completed struct into the trees.  // SEAM(W6 recalcPointerSubmeta):
    /// the C++ also recomputes the submeta of pointers that already point at this
    /// struct; the console construction flow has no such prior pointers, so that
    /// refinement is a no-op here.
    fn set_fields_struct(
        &self,
        ct: &Rc<Datatype>,
        fd: Vec<TypeField>,
        bit: Vec<TypeBitField>,
        new_size: int4,
        new_align: int4,
        extra_flags: uint4,
    ) -> KunaResult<Rc<Datatype>> {
        if !ct.is_incomplete() {
            return Err(KunaError::lowlevel("Can only set fields on an incomplete structure"));
        }
        self.erase_interned(ct, false);
        let mut newct = (**ct).clone();
        newct.set_struct_fields(fd, bit, new_size, new_align);
        newct.flags &= !flags::type_incomplete;
        newct.flags |= extra_flags
            & (flags::opaque_string
                | flags::variable_length
                | flags::type_incomplete
                | flags::has_bitfields);
        let newrc = Rc::new(newct);
        self.insert(Rc::clone(&newrc))?;
        Ok(newrc)
    }

    /// C++ `TypeFactory::setFields(...,TypeUnion*,...)` (type.cc:3981-3992):
    /// re-key a completed union into the trees.
    fn set_fields_union(
        &self,
        ct: &Rc<Datatype>,
        fd: Vec<TypeField>,
        new_size: int4,
        new_align: int4,
        extra_flags: uint4,
    ) -> KunaResult<Rc<Datatype>> {
        if !ct.is_incomplete() {
            return Err(KunaError::lowlevel("Can only set fields on an incomplete union"));
        }
        self.erase_interned(ct, false);
        let mut newct = (**ct).clone();
        newct.set_union_fields(fd, new_size, new_align);
        newct.flags &= !flags::type_incomplete;
        newct.flags |= extra_flags & (flags::variable_length | flags::type_incomplete);
        let newrc = Rc::new(newct);
        self.insert(Rc::clone(&newrc))?;
        Ok(newrc)
    }

    /// C++ `TypeFactory::setEnumValues` (type.cc:4013-4019): install the
    /// value->name map on an interned enum, re-keying the tree.  Returns the
    /// completed (new) enum `Rc`.
    fn set_enum_values_impl(
        &self,
        ct: &Rc<Datatype>,
        nmap: std::collections::BTreeMap<u64, String>,
    ) -> KunaResult<Rc<Datatype>> {
        self.erase_interned(ct, false);
        let mut newct = (**ct).clone();
        newct.set_name_map(nmap);
        let newrc = Rc::new(newct);
        self.insert(Rc::clone(&newrc))?;
        Ok(newrc)
    }

    /// C++ `TypeFactory::getTypedef` (type.cc:4292-4314): create (or find) a
    /// typedef of `ct` named `name`.  A pre-existing (name,id) must already be a
    /// typedef of `ct` or this errs.
    fn get_typedef_impl(
        &self,
        ct: &Rc<Datatype>,
        name: &str,
        id: uint8,
        format: uint4,
    ) -> KunaResult<Rc<Datatype>> {
        let id = if id == 0 { Datatype::hash_name(name) } else { id };
        if let Some(res) = self.find_by_id_local(name, id) {
            // C++ `if (ct != res->getTypedef())` throws.
            let same = matches!(res.get_typedef(), Some(t) if Rc::ptr_eq(t, ct));
            if !same {
                return Err(KunaError::lowlevel(format!(
                    "Trying to create typedef of existing type: {name}"
                )));
            }
            return Ok(res);
        }
        let mut res = (**ct).clone(); // Clone everything
        res.name = name.to_string(); // But a new name
        res.display_name = name.to_string();
        res.id = id; // and new id
        res.flags &= !flags::coretype; // Not a core type
        res.typedef_imm = Some(Rc::clone(ct));
        res.set_display_format(format);
        let resrc = Rc::new(res);
        self.insert(Rc::clone(&resrc))?;
        // C++ also stashes incomplete typedefs for later completion
        // (`incompleteTypedef`); the console parse_C flow only typedefs complete
        // types, so that deferred list is unused here.  // SEAM(W6)
        Ok(resrc)
    }

    /// C++ `TypeFactory::destroyType` (type.cc:4645-4655): remove a data-type
    /// from the container (used on the construction-error path).  Core types
    /// cannot be destroyed.
    fn destroy_type_impl(&self, ct: &Rc<Datatype>) -> KunaResult<()> {
        if ct.is_core_type() {
            return Err(KunaError::lowlevel("Cannot destroy core type"));
        }
        // C++ removeWarning(ct) on hasWarning(); the warning channel is a W5 seam,
        // so there is no warning list to prune here.
        self.erase_interned(ct, true);
        Ok(())
    }

    // -- Core-type setup (type.cc:3637-3707) ---------------------------------

    /// Manually create a "base" core type (C++ `TypeFactory::setCoreType`,
    /// type.cc:3637-3654).  Must be called before any pointers/arrays are defined
    /// off the type.  Marks the resulting type with the `coretype` flag.
    pub fn set_core_type(
        &self,
        name: &str,
        size: int4,
        meta: type_metatype,
        chartp: bool,
    ) -> KunaResult<()> {
        let ct = if chartp {
            if size == 1 {
                self.make_type_char(name)?
            } else {
                self.make_type_unicode(name, size, meta)?
            }
        } else if meta == type_metatype::TYPE_CODE {
            self.make_type_code_named(name)?
        } else if meta == type_metatype::TYPE_VOID {
            self.get_type_void()?
        } else {
            self.get_base_named(size, meta, name)?
        };
        // C++ `ct->flags |= coretype;` mutates the interned object in place.  The
        // Rust port shares interned types immutably, so re-stamp the coretype flag
        // by re-interning a flagged clone in its place.
        self.restamp_core_flag(&ct)?;
        Ok(())
    }

    /// Re-intern `ct` with the `coretype` flag set (the in-place
    /// `ct->flags |= coretype` of `setCoreType`).  Because the flag does not
    /// participate in `compareDependency`, the tree position is unchanged; we swap
    /// the stored `Rc` for a flagged clone and refresh the caches that hold it.
    fn restamp_core_flag(&self, ct: &Rc<Datatype>) -> KunaResult<()> {
        if ct.is_core_type() {
            return Ok(());
        }
        let mut flagged = (**ct).clone();
        flagged.flags |= flags::coretype;
        let flagged = Rc::new(flagged);
        let mut store = self.store.borrow_mut();
        // Replace in the tree (same ordering key; remove the old, insert flagged).
        store.tree.remove(&TreeKey(Rc::clone(ct)));
        store.tree.insert(TreeKey(Rc::clone(&flagged)));
        if flagged.id != 0 {
            for slot in store.nametree.iter_mut() {
                if Rc::ptr_eq(slot, ct) {
                    *slot = Rc::clone(&flagged);
                }
            }
        }
        Ok(())
    }

    /// Cache the most commonly accessed core data-types (C++
    /// `TypeFactory::cacheCoreTypes`, type.cc:3659-3707).  The core types must
    /// already be present in the tree.
    pub fn cache_core_types(&self) -> KunaResult<()> {
        use type_metatype::*;
        // Snapshot the tree contents to avoid holding the borrow across the
        // cache writes.
        let entries: Vec<Rc<Datatype>> = {
            let store = self.store.borrow();
            store.tree.iter().map(|k| Rc::clone(&k.0)).collect()
        };
        let float_idx = TYPE_FLOAT.as_i32();
        let mut store = self.store.borrow_mut();
        for ct in entries {
            if !ct.is_core_type() {
                continue;
            }
            if ct.get_size() > 8 {
                if ct.get_metatype() == TYPE_FLOAT {
                    if ct.get_size() == 10 {
                        store.typecache10 = Some(Rc::clone(&ct));
                    } else if ct.get_size() == 16 {
                        store.typecache16 = Some(Rc::clone(&ct));
                    }
                }
                continue;
            }
            let meta = ct.get_metatype();
            // C++ switch with fallthrough TYPE_INT -> TYPE_UINT -> common.
            let mut handled_char = false;
            if meta == TYPE_INT || meta == TYPE_UINT {
                if meta == TYPE_INT && ct.get_size() == 1 && !ct.is_ascii() {
                    store.type_nochar = Some(Rc::clone(&ct));
                }
                if ct.is_enum_type() {
                    continue; // Conceivably an enumeration
                }
                if ct.is_char_print() {
                    if ct.get_size() < 5 {
                        store.charcache[ct.get_size() as usize] = Some(Rc::clone(&ct));
                    }
                    if ct.is_ascii() {
                        // Char is preferred over other int types.
                        let col = (ct.get_metatype().as_i32() - float_idx) as usize;
                        store.typecache[ct.get_size() as usize][col] = Some(Rc::clone(&ct));
                    }
                    // Other character types (UTF16,UTF32) are not preferred.
                    continue;
                }
                handled_char = true; // fall through to the common cache step
            }
            // Common step for VOID/UNKNOWN/BOOL/CODE/FLOAT (and the INT/UINT
            // fallthrough that did not `continue` above).
            if handled_char
                || matches!(meta, TYPE_VOID | TYPE_UNKNOWN | TYPE_BOOL | TYPE_CODE | TYPE_FLOAT)
            {
                let row = ct.get_size() as usize;
                let col = (ct.get_metatype().as_i32() - float_idx) as usize;
                if store.typecache[row][col].is_none() {
                    store.typecache[row][col] = Some(Rc::clone(&ct));
                }
            }
        }
        Ok(())
    }

    // -- Atomic / core getters (type.cc:4056-4198) ---------------------------

    /// Get the unique "void" data-type (C++ `TypeFactory::getTypeVoid`,
    /// type.cc:4056-4069).
    fn get_type_void_impl(&self) -> KunaResult<Rc<Datatype>> {
        let void_col = (type_metatype::TYPE_VOID.as_i32() - type_metatype::TYPE_FLOAT.as_i32()) as usize;
        if let Some(ct) = self.store.borrow().typecache[0][void_col].clone() {
            return Ok(ct);
        }
        // TypeVoid(): Datatype(0,1,TYPE_VOID), name = "void", coretype.
        let mut tv = Datatype::new_with_align(0, 1, type_metatype::TYPE_VOID);
        tv.name = "void".to_string();
        tv.display_name = "void".to_string();
        tv.flags |= flags::coretype;
        tv.kind = DatatypeKind::Void;
        tv.id = Datatype::hash_name(&tv.name);
        let ct = Rc::new(tv);
        // C++ inserts directly into tree/nametree and the cache (not via findAdd).
        {
            let mut store = self.store.borrow_mut();
            store.tree.insert(TreeKey(Rc::clone(&ct)));
            store.nametree.push(Rc::clone(&ct));
            store.nametree.sort_by(|a, b| a.name.cmp(&b.name).then(a.id.cmp(&b.id)));
            store.typecache[0][void_col] = Some(Rc::clone(&ct));
        }
        Ok(ct)
    }

    /// Create a default 1-byte "char" type (C++ `TypeFactory::getTypeChar(name)`,
    /// type.cc:4074-4080).
    fn make_type_char(&self, n: &str) -> KunaResult<Rc<Datatype>> {
        // TypeChar(n): TypeBase(1,TYPE_INT,n), flags|=chartype, submeta=SUB_INT_CHAR.
        let mut tc = Datatype::new(1, type_metatype::TYPE_INT);
        tc.name = n.to_string();
        tc.display_name = n.to_string();
        tc.flags |= flags::chartype;
        tc.submeta = sub_metatype::SUB_INT_CHAR;
        tc.id = Datatype::hash_name(n);
        self.find_add(tc)
    }

    /// Create a default multi-byte "unicode" type (C++
    /// `TypeFactory::getTypeUnicode`, type.cc:4087-4093).
    fn make_type_unicode(&self, nm: &str, sz: int4, m: type_metatype) -> KunaResult<Rc<Datatype>> {
        // TypeUnicode(nm,sz,m): TypeBase(sz,m,nm); setflags(); submeta by INT/UINT.
        let mut tu = Datatype::new(sz, m);
        tu.name = nm.to_string();
        tu.display_name = nm.to_string();
        // setflags(): utf16/utf32/chartype by size.
        match sz {
            2 => tu.flags |= flags::utf16,
            4 => tu.flags |= flags::utf32,
            1 => tu.flags |= flags::chartype,
            _ => {}
        }
        tu.submeta = if m == type_metatype::TYPE_INT {
            sub_metatype::SUB_INT_UNICODE
        } else {
            sub_metatype::SUB_UINT_UNICODE
        };
        tu.id = Datatype::hash_name(nm);
        self.find_add(tu)
    }

    /// Get a "base" data-type by size and metatype, NOT preferring char (C++
    /// `TypeFactory::getBaseNoChar`, type.cc:4100-4106).
    fn get_base_no_char_impl(&self, s: int4, m: type_metatype) -> KunaResult<Rc<Datatype>> {
        if s == 1 && m == type_metatype::TYPE_INT {
            if let Some(nc) = self.store.borrow().type_nochar.clone() {
                return Ok(nc);
            }
        }
        self.get_base_impl(s, m)
    }

    /// Get one of the "base" datatypes, going through the cache first (C++
    /// `TypeFactory::getBase(s,m)`, type.cc:4112-4141).
    fn get_base_impl(&self, s: int4, m: type_metatype) -> KunaResult<Rc<Datatype>> {
        let float_idx = type_metatype::TYPE_FLOAT.as_i32();
        if s < 9 {
            if m.as_i32() >= float_idx {
                let cached = self.store.borrow().typecache[s as usize][(m.as_i32() - float_idx) as usize]
                    .clone();
                if let Some(ct) = cached {
                    return Ok(ct);
                }
            }
        } else if m == type_metatype::TYPE_FLOAT {
            let cached = if s == 10 {
                self.store.borrow().typecache10.clone()
            } else if s == 16 {
                self.store.borrow().typecache16.clone()
            } else {
                None
            };
            if let Some(ct) = cached {
                return Ok(ct);
            }
        }
        if s > self.max_basetype_size.get() {
            // Create array of unknown bytes to match size.
            let unk_col = (type_metatype::TYPE_UNKNOWN.as_i32() - float_idx) as usize;
            let unk = self.store.borrow().typecache[1][unk_col]
                .clone()
                .ok_or_else(|| {
                    KunaError::lowlevel("getBase: TYPE_UNKNOWN size-1 not cached for oversize array")
                })?;
            let arr = self.get_type_array_impl(s, unk)?;
            return self.find_add((*arr).clone());
        }
        // TypeBase(s,m): Datatype(s,-1,m).
        let tmp = Datatype::new(s, m);
        self.find_add(tmp)
    }

    /// Get or create a named "base" type (C++ `TypeFactory::getBase(s,m,n)`,
    /// type.cc:4148-4154).
    fn get_base_named_impl(&self, s: int4, m: type_metatype, n: &str) -> KunaResult<Rc<Datatype>> {
        let mut tmp = Datatype::new(s, m);
        tmp.name = n.to_string();
        tmp.display_name = n.to_string();
        tmp.id = Datatype::hash_name(n);
        self.find_add(tmp)
    }

    /// Return a cached core character data-type of the given size, else error
    /// (C++ `TypeFactory::getTypeChar(int4)`, type.cc:4159-4168).
    fn get_type_char_sized(&self, s: int4) -> KunaResult<Rc<Datatype>> {
        if s < 5 {
            if let Some(res) = self.store.borrow().charcache[s as usize].clone() {
                return Ok(res);
            }
        }
        Err(KunaError::lowlevel("Request for unsupported character data-type"))
    }

    /// Retrieve or create the core anonymous "code" data-type (C++
    /// `TypeFactory::getTypeCode(void)`, type.cc:4173-4182).
    fn get_type_code_impl(&self) -> KunaResult<Rc<Datatype>> {
        let code_col = (type_metatype::TYPE_CODE.as_i32() - type_metatype::TYPE_FLOAT.as_i32()) as usize;
        if let Some(ct) = self.store.borrow().typecache[1][code_col].clone() {
            return Ok(ct);
        }
        // TypeCode(): generic code object, markComplete().
        let mut tmp = Datatype::new_with_align(1, 1, type_metatype::TYPE_CODE);
        tmp.kind = DatatypeKind::Code { proto: None };
        // markComplete(): clear the type_incomplete flag (already clear here).
        tmp.flags &= !flags::type_incomplete;
        self.find_add(tmp)
    }

    /// Create a named "code" data-type (C++ `TypeFactory::getTypeCode(nm)`,
    /// type.cc:4188-4198).
    fn make_type_code_named(&self, nm: &str) -> KunaResult<Rc<Datatype>> {
        if nm.is_empty() {
            return self.get_type_code_impl();
        }
        let mut tmp = Datatype::new_with_align(1, 1, type_metatype::TYPE_CODE);
        tmp.kind = DatatypeKind::Code { proto: None };
        tmp.name = nm.to_string();
        tmp.display_name = nm.to_string();
        tmp.id = Datatype::hash_name(nm);
        tmp.flags &= !flags::type_incomplete;
        self.find_add(tmp)
    }

    // -- Pointer / composite construction (type.cc:4323-4471) ----------------

    /// Build a [`DatatypeKind::Pointer`] value with its inherited flags and
    /// computed sub-metatype (the C++ `TypePointer(s,pt,ws)` constructor +
    /// `calcSubmeta`, type.hh:466-467 / type.cc:1172-1190).
    fn build_pointer(&self, s: int4, pt: Rc<Datatype>, ws: uint4) -> Datatype {
        let mut p = Datatype::new(s, type_metatype::TYPE_PTR);
        // flags = ptrto->inheritForPointer(); spaceid = null; truncate = null.
        p.flags = pt.inherit_for_pointer();
        // calcSubmeta():
        let ptrto_meta = pt.get_metatype();
        match ptrto_meta {
            type_metatype::TYPE_STRUCT => {
                p.submeta = if pt.needs_resolution() {
                    sub_metatype::SUB_PTR
                } else {
                    sub_metatype::SUB_PTR_STRUCT
                };
            }
            type_metatype::TYPE_UNION => {
                p.submeta = sub_metatype::SUB_PTR_STRUCT;
            }
            type_metatype::TYPE_ARRAY => {
                p.flags |= flags::pointer_to_array;
            }
            _ => {}
        }
        if pt.needs_resolution() && ptrto_meta != type_metatype::TYPE_PTR {
            p.flags |= flags::needs_resolution;
        }
        p.kind = DatatypeKind::Pointer { ptrto: pt, spaceid: None, truncate: None, wordsize: ws };
        p
    }

    /// Assign a truncated pointer subcomponent if `res` has the alt-pointer size
    /// (C++ `TypePointer::calcTruncate`, type.cc:1195-1204).
    fn calc_truncate(&self, res: Rc<Datatype>) -> KunaResult<Rc<Datatype>> {
        let alt = self.size_of_alt_pointer.get();
        let has_truncate = matches!(&res.kind, DatatypeKind::Pointer { truncate: Some(_), .. });
        if has_truncate || res.size != alt {
            return Ok(res);
        }
        let smaller = self.resize_pointer_impl(&res, self.size_of_pointer.get())?;
        let mut updated = (*res).clone();
        if let DatatypeKind::Pointer { truncate, .. } = &mut updated.kind {
            *truncate = Some(smaller);
        }
        if self.truncate_big_endian.get() {
            updated.flags |= flags::truncate_bigendian;
        }
        // Re-intern the now-truncated pointer in place of the bare one.
        self.reintern_replace(&res, updated)
    }

    /// Replace an interned data-type `old` with `new_val` (recomputing alignment
    /// if unset), used by `calcTruncate` where the C++ mutates the interned object
    /// in place.  Returns the re-interned `Rc`.
    fn reintern_replace(&self, old: &Rc<Datatype>, new_val: Datatype) -> KunaResult<Rc<Datatype>> {
        let new_rc = Rc::new(new_val);
        let mut store = self.store.borrow_mut();
        store.tree.remove(&TreeKey(Rc::clone(old)));
        store.tree.insert(TreeKey(Rc::clone(&new_rc)));
        if new_rc.id != 0 {
            for slot in store.nametree.iter_mut() {
                if Rc::ptr_eq(slot, old) {
                    *slot = Rc::clone(&new_rc);
                }
            }
        }
        Ok(new_rc)
    }

    /// Construct a pointer, stripping an ARRAY level (C++
    /// `getTypePointerStripArray`, type.cc:4323-4334).
    fn get_type_pointer_strip_array_impl(
        &self,
        s: int4,
        pt: Rc<Datatype>,
        ws: uint4,
    ) -> KunaResult<Rc<Datatype>> {
        let mut pt = pt;
        if pt.has_stripped() {
            if let Some(st) = pt.get_stripped() {
                pt = st;
            }
        }
        if pt.get_metatype() == type_metatype::TYPE_ARRAY {
            if let Some(base) = pt.get_array_base() {
                pt = base; // Strip the first ARRAY type
            }
        }
        let res = self.find_add(self.build_pointer(s, pt, ws))?;
        self.calc_truncate(res)
    }

    /// Construct an absolute pointer (C++ `getTypePointer`, type.cc:4341-4350).
    fn get_type_pointer_impl(&self, s: int4, pt: Rc<Datatype>, ws: uint4) -> KunaResult<Rc<Datatype>> {
        let mut pt = pt;
        if pt.has_stripped() {
            if let Some(st) = pt.get_stripped() {
                pt = st;
            }
        }
        let res = self.find_add(self.build_pointer(s, pt, ws))?;
        self.calc_truncate(res)
    }

    /// Construct a named pointer (C++ `getTypePointer(...,n)`, type.cc:4359-4371).
    fn get_type_pointer_named_impl(
        &self,
        s: int4,
        pt: Rc<Datatype>,
        ws: uint4,
        n: &str,
    ) -> KunaResult<Rc<Datatype>> {
        let mut pt = pt;
        if pt.has_stripped() {
            if let Some(st) = pt.get_stripped() {
                pt = st;
            }
        }
        let mut tmp = self.build_pointer(s, pt, ws);
        tmp.name = n.to_string();
        tmp.display_name = n.to_string();
        tmp.id = Datatype::hash_name(n);
        let res = self.find_add(tmp)?;
        self.calc_truncate(res)
    }

    /// Build a resized pointer (C++ `resizePointer`, type.cc:4545-4553).
    fn resize_pointer_impl(&self, ptr: &Rc<Datatype>, new_size: int4) -> KunaResult<Rc<Datatype>> {
        let mut pt = ptr.get_ptr_to().ok_or_else(|| {
            KunaError::lowlevel("resizePointer: argument is not a pointer")
        })?;
        if pt.has_stripped() {
            if let Some(st) = pt.get_stripped() {
                pt = st;
            }
        }
        let ws = ptr.get_word_size().unwrap_or(1);
        self.find_add(self.build_pointer(new_size, pt, ws))
    }

    /// Build a [`DatatypeKind::Array`] value (C++ `TypeArray(n,ao)`,
    /// type.hh:1006-1015).
    fn build_array(&self, as_: int4, ao: Rc<Datatype>) -> Datatype {
        // Datatype(n*ao->getAlignSize(), ao->getAlignment(), TYPE_ARRAY).
        let size = as_ * ao.get_align_size();
        let mut a = Datatype::new_with_align(size, ao.get_alignment(), type_metatype::TYPE_ARRAY);
        // An array of size 1 is generally treated as the element data-type.
        if as_ == 1 {
            a.flags |= flags::needs_resolution;
        }
        a.kind = DatatypeKind::Array { arrayof: ao, arraysize: as_ };
        a
    }

    /// Construct an array data-type (C++ `getTypeArray`, type.cc:4376-4383).
    fn get_type_array_impl(&self, as_: int4, ao: Rc<Datatype>) -> KunaResult<Rc<Datatype>> {
        let mut ao = ao;
        if ao.has_stripped() {
            if let Some(st) = ao.get_stripped() {
                ao = st;
            }
        }
        self.find_add(self.build_array(as_, ao))
    }

    /// Create an (empty) incomplete structure (C++ `getTypeStruct`,
    /// type.cc:4388-4396).
    fn get_type_struct_impl(&self, n: &str) -> KunaResult<Rc<Datatype>> {
        // TypeStruct(): incomplete, no fields.
        let mut tmp = Datatype::new_with_align(0, -1, type_metatype::TYPE_STRUCT);
        tmp.flags |= flags::type_incomplete;
        tmp.kind = DatatypeKind::Struct { field: Vec::new(), bitfield: Vec::new() };
        tmp.name = n.to_string();
        tmp.display_name = n.to_string();
        tmp.id = Datatype::hash_name(n);
        self.find_add(tmp)
    }

    /// Create a partial structure (C++ `getTypePartialStruct`,
    /// type.cc:4403-4409).
    fn get_type_partial_struct_impl(
        &self,
        contain: Rc<Datatype>,
        off: int4,
        sz: int4,
    ) -> KunaResult<Rc<Datatype>> {
        let strip = self.get_base_impl(sz, type_metatype::TYPE_UNKNOWN)?;
        // TypePartialStruct(contain,off,sz,strip).
        let mut tps = Datatype::new_with_align(sz, -1, type_metatype::TYPE_PARTIALSTRUCT);
        tps.flags |= contain.inherit_for_partial();
        tps.kind = DatatypeKind::PartialStruct { stripped: strip, container: contain, offset: off };
        self.find_add(tps)
    }

    /// Create an (empty) incomplete union (C++ `getTypeUnion`,
    /// type.cc:4414-4422).
    fn get_type_union_impl(&self, n: &str) -> KunaResult<Rc<Datatype>> {
        let mut tmp = Datatype::new_with_align(0, -1, type_metatype::TYPE_UNION);
        // C++ `TypeUnion()` ctor (type.hh:625): flags |= (type_incomplete |
        // needs_resolution).  Every union "needs resolution" — its accessed field
        // is recovered from the data flow by `ScoreUnionFields`/`resolveInFlow`.
        tmp.flags |= flags::type_incomplete | flags::needs_resolution;
        tmp.kind = DatatypeKind::Union { field: Vec::new() };
        tmp.name = n.to_string();
        tmp.display_name = n.to_string();
        tmp.id = Datatype::hash_name(n);
        self.find_add(tmp)
    }

    /// Create a partial union (C++ `getTypePartialUnion`, type.cc:4429-4435).
    fn get_type_partial_union_impl(
        &self,
        contain: Rc<Datatype>,
        off: int4,
        sz: int4,
    ) -> KunaResult<Rc<Datatype>> {
        let strip = self.get_base_impl(sz, type_metatype::TYPE_UNKNOWN)?;
        let mut tpu = Datatype::new_with_align(sz, -1, type_metatype::TYPE_PARTIALUNION);
        tpu.flags |= contain.inherit_for_partial() | flags::needs_resolution;
        tpu.kind = DatatypeKind::PartialUnion { stripped: strip, container: contain, offset: off };
        self.find_add(tpu)
    }

    /// Create an (empty) enumeration (C++ `getTypeEnum`, type.cc:4441-4447).
    fn get_type_enum_impl(&self, n: &str) -> KunaResult<Rc<Datatype>> {
        // TypeEnum(enumsize,enumtype,n): TypeBase(s,m,nm) -> submeta = base2sub(m),
        // then flags|=enumtype and metatype = INT/UINT.
        let s = self.enumsize.get();
        let m = self.enumtype.get();
        let mut tmp = Datatype::new(s, m); // submeta = base2sub(ENUM_INT/UINT)
        tmp.name = n.to_string();
        tmp.display_name = n.to_string();
        tmp.flags |= flags::enumtype;
        tmp.metatype =
            if m == type_metatype::TYPE_ENUM_INT { type_metatype::TYPE_INT } else { type_metatype::TYPE_UINT };
        tmp.kind = DatatypeKind::Enum { namemap: std::collections::BTreeMap::new() };
        tmp.id = Datatype::hash_name(n);
        self.find_add(tmp)
    }

    /// Create a partial enumeration (C++ `getTypePartialEnum`,
    /// type.cc:4454-4460).
    fn get_type_partial_enum_impl(
        &self,
        contain: Rc<Datatype>,
        off: int4,
        sz: int4,
    ) -> KunaResult<Rc<Datatype>> {
        let strip = self.get_base_impl(sz, type_metatype::TYPE_UNKNOWN)?;
        let mut tpe = Datatype::new_with_align(sz, -1, type_metatype::TYPE_PARTIALENUM);
        tpe.flags |= contain.inherit_for_partial();
        tpe.kind = DatatypeKind::PartialEnum { stripped: strip, parent: contain, offset: off };
        self.find_add(tpe)
    }

    /// Create a "spacebase" type (C++ `getTypeSpacebase`, type.cc:4466-4471).
    fn get_type_spacebase_impl(
        &self,
        id: Rc<AddrSpace>,
        addr: &Address,
    ) -> KunaResult<Rc<Datatype>> {
        // TypeSpacebase(id,addr,glb): Datatype(0,1,TYPE_SPACEBASE).
        let mut tsb = Datatype::new_with_align(0, 1, type_metatype::TYPE_SPACEBASE);
        tsb.kind = DatatypeKind::Spacebase { spaceid: Some(id), localframe: addr.clone() };
        self.find_add(tsb)
    }

    /// Find/create an ephemeral relative pointer (C++ `getTypePointerRel`
    /// 3-arg, type.cc:4490-4497).
    fn get_type_pointer_rel_impl(
        &self,
        parent_ptr: Rc<Datatype>,
        ptr_to: Rc<Datatype>,
        off: int4,
    ) -> KunaResult<Rc<Datatype>> {
        let size = parent_ptr.get_size();
        let ws = parent_ptr.get_word_size().ok_or_else(|| {
            KunaError::lowlevel("getTypePointerRel: parentPtr is not a pointer")
        })?;
        let parent = parent_ptr
            .get_ptr_to()
            .ok_or_else(|| KunaError::lowlevel("getTypePointerRel: parentPtr is not a pointer"))?;
        let mut tp = self.build_pointer_rel(size, ptr_to, ws, parent, off);
        // markEphemeral(typegrp): stripped = getTypePointer(size,ptrto,wordsize),
        // flags|=has_stripped, submeta=SUB_PTRREL_UNK if ptrto is TYPE_UNKNOWN.
        self.mark_ephemeral(&mut tp)?;
        self.find_add(tp)
    }

    /// Build a named, non-ephemeral relative pointer (C++ `getTypePointerRel`
    /// 6-arg, type.cc:4510-4519).
    #[allow(clippy::too_many_arguments)]
    fn get_type_pointer_rel_full_impl(
        &self,
        sz: int4,
        parent: Rc<Datatype>,
        ptr_to: Rc<Datatype>,
        ws: int4,
        off: int4,
        nm: &str,
    ) -> KunaResult<Rc<Datatype>> {
        let mut tp = self.build_pointer_rel(sz, ptr_to, ws as uint4, parent, off);
        tp.name = nm.to_string();
        tp.display_name = nm.to_string();
        tp.id = Datatype::hash_name(nm);
        self.find_add(tp)
    }

    /// Build a [`DatatypeKind::PointerRel`] value (C++ `TypePointerRel(sz,ptrTo,
    /// ws,parent,off)`).  The relative pointer inherits the plain-pointer flags
    /// and sub-metatype of an equivalent `TypePointer(sz,ptrTo,ws)`, then sets
    /// `is_ptrrel` and (when `ptrto` is structured) the `SUB_PTRREL` submeta.
    fn build_pointer_rel(
        &self,
        sz: int4,
        ptr_to: Rc<Datatype>,
        ws: uint4,
        parent: Rc<Datatype>,
        off: int4,
    ) -> Datatype {
        // Start from the plain-pointer skeleton (inheritForPointer + calcSubmeta).
        let mut tp = self.build_pointer(sz, Rc::clone(&ptr_to), ws);
        // C++ TypePointerRel keeps `metatype == TYPE_PTR` internally — the
        // TypePointer base ctor sets it and the relative ctor never overrides it
        // ("Don't use TYPE_PTRREL internally", type.cc:3010).  The TYPE_PTRREL
        // metatype is ONLY a marshalling override (encodeBasic(TYPE_PTRREL,...),
        // type.cc:3109).  Using it as the live metatype mis-classifies a relative
        // pointer as a non-pointer in every `getMetatype()==TYPE_PTR` test
        // (RulePtrArith pointer-slot search, propagateType, etc.).
        tp.flags |= flags::is_ptrrel;
        // TypePointerRel uses SUB_PTRREL for the dependency ordering of a formal
        // relative pointer (markEphemeral lowers it to SUB_PTRREL_UNK).
        tp.submeta = sub_metatype::SUB_PTRREL;
        tp.kind = DatatypeKind::PointerRel {
            ptrto: ptr_to,
            wordsize: ws,
            stripped: None,
            parent,
            offset: off,
        };
        tp
    }

    /// Mark a relative pointer as ephemeral (C++ `TypePointerRel::markEphemeral`,
    /// type.hh:1025-1034): cache a stripped plain pointer and lower the submeta
    /// when it points at an unknown type.
    fn mark_ephemeral(&self, tp: &mut Datatype) -> KunaResult<()> {
        let (ptrto, ws) = match &tp.kind {
            DatatypeKind::PointerRel { ptrto, wordsize, .. } => (Rc::clone(ptrto), *wordsize),
            _ => return Err(KunaError::lowlevel("markEphemeral: not a relative pointer")),
        };
        let stripped = self.get_type_pointer_impl(tp.size, Rc::clone(&ptrto), ws)?;
        if let DatatypeKind::PointerRel { stripped: slot, .. } = &mut tp.kind {
            *slot = Some(stripped);
        }
        tp.flags |= flags::has_stripped;
        if ptrto.get_metatype() == type_metatype::TYPE_UNKNOWN {
            tp.submeta = sub_metatype::SUB_PTRREL_UNK;
        }
        Ok(())
    }

    /// Build a named pointer with an address-space attribute (C++
    /// `getTypePointerWithSpace`, type.cc:4529-4539).
    fn get_type_pointer_with_space_impl(
        &self,
        ptr_to: Rc<Datatype>,
        spc: Rc<AddrSpace>,
        nm: &str,
    ) -> KunaResult<Rc<Datatype>> {
        // TypePointer(ptrTo,spc): ws = spc->getWordSize(); spaceid = spc.
        let ws = spc.get_word_size();
        let mut tp = self.build_pointer(self.size_of_pointer.get(), ptr_to, ws);
        if let DatatypeKind::Pointer { spaceid, .. } = &mut tp.kind {
            *spaceid = Some(spc);
        }
        tp.name = nm.to_string();
        tp.display_name = nm.to_string();
        tp.id = Datatype::hash_name(nm);
        let res = self.find_add(tp)?;
        self.calc_truncate(res)
    }

    // -- Resizing / piece extraction (type.cc:4558-4610) ---------------------

    /// Build a resized integer based on the given integer (C++ `resizeInteger`,
    /// type.cc:4558-4568).
    fn resize_integer_impl(&self, ct: Rc<Datatype>, new_size: int4) -> KunaResult<Rc<Datatype>> {
        if new_size == ct.get_size() {
            return Ok(ct);
        }
        let mut meta = ct.get_metatype();
        if meta != type_metatype::TYPE_INT && meta != type_metatype::TYPE_UINT {
            meta = type_metatype::TYPE_UINT;
        }
        if ct.is_char_print() {
            self.get_base_impl(new_size, meta)
        } else {
            self.get_base_no_char_impl(new_size, meta)
        }
    }

    /// Get the data-type associated with a piece of a structured data-type (C++
    /// `getExactPiece`, type.cc:4579-4610).  Drills down through nested
    /// data-types; any union encountered yields a partial union.
    fn get_exact_piece_impl(
        &self,
        ct: Rc<Datatype>,
        offset: int4,
        size: int4,
    ) -> KunaResult<Option<Rc<Datatype>>> {
        let mut last_type: Option<Rc<Datatype>> = None;
        let mut last_off: int8 = 0;
        let mut cur_off: int8 = offset as int8;
        let mut ct = ct;
        loop {
            if (ct.get_size() as int8) < size as int8 + cur_off {
                break; // Range beyond end; construct partial around last data-type
            }
            if ct.get_size() == size {
                return Ok(Some(ct)); // Perfect size match
            }
            last_type = Some(Rc::clone(&ct));
            last_off = cur_off;
            let (sub, new_off) = self.get_sub_type_via_factory(&ct, cur_off)?;
            cur_off = new_off;
            match sub {
                Some(next) => ct = next,
                None => break,
            }
        }
        if let Some(last_type) = last_type {
            let meta = last_type.get_metatype();
            match meta {
                type_metatype::TYPE_STRUCT
                | type_metatype::TYPE_ARRAY
                | type_metatype::TYPE_PARTIALSTRUCT => {
                    return Ok(Some(self.get_type_partial_struct_impl(
                        last_type,
                        last_off as int4,
                        size,
                    )?));
                }
                type_metatype::TYPE_UNION => {
                    return Ok(Some(self.get_type_partial_union_impl(
                        last_type,
                        last_off as int4,
                        size,
                    )?));
                }
                type_metatype::TYPE_PARTIALUNION => {
                    // Truncate to a smaller partial union: re-base into the parent.
                    let parent = last_type.get_partial_base().ok_or_else(|| {
                        KunaError::lowlevel("getExactPiece: partial union missing parent")
                    })?;
                    let part_off = last_type.get_partial_offset().unwrap_or(0);
                    return Ok(Some(self.get_type_partial_union_impl(
                        parent,
                        last_off as int4 + part_off,
                        size,
                    )?));
                }
                _ => {
                    if last_type.is_enum_type() && !last_type.has_stripped() {
                        return Ok(Some(self.get_type_partial_enum_impl(
                            last_type,
                            last_off as int4,
                            size,
                        )?));
                    }
                }
            }
        }
        Ok(None)
    }

    /// `Datatype::getSubType` with the bound factory available, resolving the two
    /// factory-dependent overrides that the bare [`Datatype::get_sub_type`] routes
    /// to a `// SEAM(W6)` `Err`:
    ///
    ///   * `TypeCode::getSubType` (type.cc:3284-3290) returns
    ///     `getBase(1,TYPE_CODE)` with `newoff = 0` (this RESOLVES the type-2
    ///     leftover seam noted on the bare method);
    ///   * every other kind delegates to the bare [`Datatype::get_sub_type`]
    ///     (which is factory-independent).
    ///
    /// `TypeSpacebase::getSubType` still needs the symbol-table `Scope`
    /// (`// SEAM(W6)`), so it remains an `Err` from the bare method.
    fn get_sub_type_via_factory(
        &self,
        ct: &Rc<Datatype>,
        off: int8,
    ) -> KunaResult<(Option<Rc<Datatype>>, int8)> {
        if let DatatypeKind::Code { .. } = &ct.kind {
            // factory != null -> getBase(1, TYPE_CODE), newoff = 0.
            let code = self.get_base_impl(1, type_metatype::TYPE_CODE)?;
            return Ok((Some(code), 0));
        }
        ct.get_sub_type(off)
    }

    /// Convert a data-type to concrete form (C++ `concretize`, type.cc:4663-4673).
    fn concretize_impl(&self, ct: Rc<Datatype>) -> KunaResult<Rc<Datatype>> {
        if ct.get_metatype() == type_metatype::TYPE_CODE {
            if ct.get_size() != 1 {
                return Err(KunaError::lowlevel("Primitive code data-type that is not size 1"));
            }
            return self.get_base_impl(1, type_metatype::TYPE_UNKNOWN);
        }
        Ok(ct)
    }

    /// Find a type by name, first match (C++ `findByName`, type.cc:3844-3848).
    fn find_by_name_impl(&self, n: &str) -> Option<Rc<Datatype>> {
        self.find_by_id(n, 0, 0)
    }

    // -- Pointer drill-down (type.cc:1221-1258) ------------------------------

    /// Add a constant offset to a pointer, descending one level (C++
    /// `TypePointer::downChain` / `TypePointerRel::downChain`, type.cc:1221-1258 /
    /// 3120-3136).
    ///
    /// Returns `(result_pointer, off, par, par_off)` where `result_pointer` is the
    /// component pointer or `None` (the C++ null return), `off` is the renormalized
    /// offset, and `(par, par_off)` pass back the container (a `TYPE_STRUCT`/
    /// `TYPE_ARRAY` pointer) and the offset into it, mirroring the C++ by-reference
    /// `TypePointer *&par` / `int8 &parOff`.
    #[allow(clippy::type_complexity)]
    pub fn down_chain(
        &self,
        ptr: &Rc<Datatype>,
        off: int8,
        allow_array_wrap: bool,
    ) -> KunaResult<(Option<Rc<Datatype>>, int8, Option<Rc<Datatype>>, int8)> {
        match &ptr.kind {
            DatatypeKind::Pointer { ptrto, wordsize, .. } => {
                self.down_chain_pointer(ptr, ptrto, *wordsize, off, allow_array_wrap)
            }
            DatatypeKind::PointerRel { ptrto, wordsize, parent, offset, .. } => {
                // TypePointerRel::downChain (type.cc:3120-3136).
                let ptrto_meta = ptrto.get_metatype();
                if off >= 0
                    && off < ptrto.get_size() as int8
                    && (ptrto_meta == type_metatype::TYPE_STRUCT
                        || ptrto_meta == type_metatype::TYPE_ARRAY)
                {
                    return self.down_chain_pointer(ptr, ptrto, *wordsize, off, allow_array_wrap);
                }
                // Convert off to be relative to the parent container.  C++:
                // `int8 relOff = (off + offset) & calc_mask(size);` — the int8 `&`
                // uintb promotes to uint8, masks, then truncates back to int8.
                let rel_off = (((off + *offset as int8) as uint8)
                    & kuna_base::address::calc_mask(ptr.size)) as int8;
                if rel_off < 0 || rel_off >= parent.get_size() as int8 {
                    return Ok((None, off, None, 0)); // Don't shift beyond container
                }
                let orig_pointer =
                    self.get_type_pointer_impl(ptr.size, Rc::clone(parent), *wordsize)?;
                let off = rel_off;
                if rel_off == 0 && *offset != 0 {
                    // Recovering the start of the parent is still downchaining.
                    return Ok((Some(orig_pointer), off, None, 0));
                }
                self.down_chain(&orig_pointer, off, allow_array_wrap)
            }
            _ => Ok((None, off, None, 0)),
        }
    }

    /// The plain-`TypePointer::downChain` body (type.cc:1221-1258), shared by the
    /// `Pointer` arm and the `PointerRel` arm's struct/array fast-path.
    #[allow(clippy::type_complexity)]
    fn down_chain_pointer(
        &self,
        ptr: &Rc<Datatype>,
        ptrto: &Rc<Datatype>,
        wordsize: uint4,
        mut off: int8,
        allow_array_wrap: bool,
    ) -> KunaResult<(Option<Rc<Datatype>>, int8, Option<Rc<Datatype>>, int8)> {
        let ptrto_size = ptrto.get_align_size() as int8;
        if off < 0 || off >= ptrto_size {
            // Check if we are wrapping.
            if ptrto_size != 0 && !ptrto.is_variable_length() {
                if !allow_array_wrap {
                    return Ok((None, off, None, 0));
                }
                // intb signOff = sign_extend(off, size*8 - 1); signOff %= ptrtoSize.
                let mut sign_off =
                    kuna_base::address::sign_extend(off, ptr.size * 8 - 1) % ptrto_size;
                if sign_off < 0 {
                    sign_off += ptrto_size;
                }
                off = sign_off;
                if off == 0 {
                    // Wrapped and now at zero: consider this going down one level.
                    return Ok((Some(Rc::clone(ptr)), off, None, 0));
                }
            }
        }

        if ptrto.is_enum_type() {
            // Go "into" the enumeration.
            let tmp = self.get_base_impl(1, type_metatype::TYPE_UINT)?;
            let result = self.get_type_pointer_impl(ptr.size, tmp, wordsize)?;
            return Ok((Some(result), 0, None, 0));
        }
        let meta = ptrto.get_metatype();
        let is_array = meta == type_metatype::TYPE_ARRAY;
        let mut par: Option<Rc<Datatype>> = None;
        let mut par_off: int8 = 0;
        if is_array || meta == type_metatype::TYPE_STRUCT {
            par = Some(Rc::clone(ptr));
            par_off = off;
        }
        let (pt, new_off) = self.get_sub_type_via_factory(ptrto, off)?;
        off = new_off;
        let pt = match pt {
            Some(p) => p,
            None => return Ok((None, off, par, par_off)),
        };
        let result = if !is_array {
            self.get_type_pointer_strip_array_impl(ptr.size, pt, wordsize)?
        } else {
            self.get_type_pointer_impl(ptr.size, pt, wordsize)?
        };
        Ok((Some(result), off, par, par_off))
    }
}

impl TypeFactory for TypeFactoryImpl {
    fn get_size_of_int(&self) -> int4 {
        self.size_of_int.get()
    }
    fn get_size_of_long(&self) -> int4 {
        self.size_of_long.get()
    }
    fn get_size_of_char(&self) -> int4 {
        self.size_of_char.get()
    }
    fn get_size_of_wchar(&self) -> int4 {
        self.size_of_wchar.get()
    }
    fn get_size_of_pointer(&self) -> int4 {
        self.size_of_pointer.get()
    }
    fn get_size_of_alt_pointer(&self) -> int4 {
        self.size_of_alt_pointer.get()
    }
    fn get_alignment(&self, size: uint4) -> KunaResult<int4> {
        self.alignment(size)
    }
    fn get_primitive_align_size(&self, size: uint4) -> KunaResult<int4> {
        self.primitive_align_size(size)
    }

    fn get_type_void(&self) -> KunaResult<Rc<Datatype>> {
        self.get_type_void_impl()
    }
    fn get_base_no_char(&self, s: int4, m: type_metatype) -> KunaResult<Rc<Datatype>> {
        self.get_base_no_char_impl(s, m)
    }
    fn get_base(&self, s: int4, m: type_metatype) -> KunaResult<Rc<Datatype>> {
        self.get_base_impl(s, m)
    }
    fn get_base_named(&self, s: int4, m: type_metatype, n: &str) -> KunaResult<Rc<Datatype>> {
        self.get_base_named_impl(s, m, n)
    }
    fn get_type_char(&self, s: int4) -> KunaResult<Rc<Datatype>> {
        self.get_type_char_sized(s)
    }
    fn get_type_code(&self) -> KunaResult<Rc<Datatype>> {
        self.get_type_code_impl()
    }

    fn get_type_pointer_strip_array(
        &self,
        s: int4,
        pt: Rc<Datatype>,
        ws: uint4,
    ) -> KunaResult<Rc<Datatype>> {
        self.get_type_pointer_strip_array_impl(s, pt, ws)
    }
    fn get_type_pointer(&self, s: int4, pt: Rc<Datatype>, ws: uint4) -> KunaResult<Rc<Datatype>> {
        self.get_type_pointer_impl(s, pt, ws)
    }
    fn get_type_pointer_named(
        &self,
        s: int4,
        pt: Rc<Datatype>,
        ws: uint4,
        n: &str,
    ) -> KunaResult<Rc<Datatype>> {
        self.get_type_pointer_named_impl(s, pt, ws, n)
    }
    fn resize_pointer(&self, ptr: Rc<Datatype>, new_size: int4) -> KunaResult<Rc<Datatype>> {
        self.resize_pointer_impl(&ptr, new_size)
    }
    fn get_type_pointer_rel(
        &self,
        parent_ptr: Rc<Datatype>,
        ptr_to: Rc<Datatype>,
        off: int4,
    ) -> KunaResult<Rc<Datatype>> {
        self.get_type_pointer_rel_impl(parent_ptr, ptr_to, off)
    }
    fn get_type_pointer_rel_full(
        &self,
        sz: int4,
        parent: Rc<Datatype>,
        ptr_to: Rc<Datatype>,
        ws: int4,
        off: int4,
        nm: &str,
    ) -> KunaResult<Rc<Datatype>> {
        self.get_type_pointer_rel_full_impl(sz, parent, ptr_to, ws, off, nm)
    }
    fn get_type_pointer_with_space(
        &self,
        ptr_to: Rc<Datatype>,
        spc: Rc<AddrSpace>,
        nm: &str,
    ) -> KunaResult<Rc<Datatype>> {
        self.get_type_pointer_with_space_impl(ptr_to, spc, nm)
    }

    fn get_type_array(&self, as_: int4, ao: Rc<Datatype>) -> KunaResult<Rc<Datatype>> {
        self.get_type_array_impl(as_, ao)
    }
    fn get_type_struct(&self, n: &str) -> KunaResult<Rc<Datatype>> {
        self.get_type_struct_impl(n)
    }
    fn get_type_partial_struct(
        &self,
        contain: Rc<Datatype>,
        off: int4,
        sz: int4,
    ) -> KunaResult<Rc<Datatype>> {
        self.get_type_partial_struct_impl(contain, off, sz)
    }
    fn get_type_union(&self, n: &str) -> KunaResult<Rc<Datatype>> {
        self.get_type_union_impl(n)
    }
    fn get_type_partial_union(
        &self,
        contain: Rc<Datatype>,
        off: int4,
        sz: int4,
    ) -> KunaResult<Rc<Datatype>> {
        self.get_type_partial_union_impl(contain, off, sz)
    }
    fn get_type_partial_enum(
        &self,
        contain: Rc<Datatype>,
        off: int4,
        sz: int4,
    ) -> KunaResult<Rc<Datatype>> {
        self.get_type_partial_enum_impl(contain, off, sz)
    }
    fn get_type_enum(&self, n: &str) -> KunaResult<Rc<Datatype>> {
        self.get_type_enum_impl(n)
    }
    fn get_type_spacebase(&self, id: Rc<AddrSpace>, addr: &Address) -> KunaResult<Rc<Datatype>> {
        self.get_type_spacebase_impl(id, addr)
    }

    fn resize_integer(&self, ct: Rc<Datatype>, new_size: int4) -> KunaResult<Rc<Datatype>> {
        self.resize_integer_impl(ct, new_size)
    }
    fn get_exact_piece(
        &self,
        ct: Rc<Datatype>,
        offset: int4,
        size: int4,
    ) -> KunaResult<Option<Rc<Datatype>>> {
        self.get_exact_piece_impl(ct, offset, size)
    }

    fn find_by_name(&self, n: &str) -> KunaResult<Option<Rc<Datatype>>> {
        Ok(self.find_by_name_impl(n))
    }
    fn concretize(&self, ct: Rc<Datatype>) -> KunaResult<Rc<Datatype>> {
        self.concretize_impl(ct)
    }

    fn is_big_endian(&self) -> bool {
        self.is_big_endian_impl()
    }
    fn set_name(&self, ct: &Rc<Datatype>, n: &str) -> KunaResult<Rc<Datatype>> {
        self.set_name_impl(ct, n)
    }
    fn set_display_format(&self, ct: &Rc<Datatype>, format: uint4) -> KunaResult<Rc<Datatype>> {
        self.set_display_format_impl(ct, format)
    }
    fn assign_raw_fields_struct(
        &self,
        ct: &Rc<Datatype>,
        fd: Vec<TypeField>,
        bit: Vec<TypeBitField>,
    ) -> KunaResult<Rc<Datatype>> {
        self.assign_raw_fields_struct_impl(ct, fd, bit)
    }
    fn assign_raw_fields_union(
        &self,
        ct: &Rc<Datatype>,
        fd: Vec<TypeField>,
    ) -> KunaResult<Rc<Datatype>> {
        self.assign_raw_fields_union_impl(ct, fd)
    }
    fn set_enum_values(
        &self,
        ct: &Rc<Datatype>,
        nmap: std::collections::BTreeMap<u64, String>,
    ) -> KunaResult<Rc<Datatype>> {
        self.set_enum_values_impl(ct, nmap)
    }
    fn get_typedef(
        &self,
        ct: &Rc<Datatype>,
        name: &str,
        id: uint8,
        format: uint4,
    ) -> KunaResult<Rc<Datatype>> {
        self.get_typedef_impl(ct, name, id, format)
    }
    fn destroy_type(&self, ct: &Rc<Datatype>) -> KunaResult<()> {
        self.destroy_type_impl(ct)
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// The `type_metatype` discriminants must match `type.hh:80-100` exactly:
    /// these numeric values are load-bearing (the `<=` ordering in
    /// `isPieceStructured`, the `base2sub` index, the cast-propagation order).
    #[test]
    fn type_metatype_discriminants() {
        assert_eq!(type_metatype::TYPE_PARTIALUNION as i32, 0);
        assert_eq!(type_metatype::TYPE_PARTIALSTRUCT as i32, 1);
        assert_eq!(type_metatype::TYPE_PARTIALENUM as i32, 2);
        assert_eq!(type_metatype::TYPE_UNION as i32, 3);
        assert_eq!(type_metatype::TYPE_STRUCT as i32, 4);
        assert_eq!(type_metatype::TYPE_ENUM_INT as i32, 5);
        assert_eq!(type_metatype::TYPE_ENUM_UINT as i32, 6);
        assert_eq!(type_metatype::TYPE_ARRAY as i32, 7);
        assert_eq!(type_metatype::TYPE_PTRREL as i32, 8);
        assert_eq!(type_metatype::TYPE_PTR as i32, 9);
        assert_eq!(type_metatype::TYPE_FLOAT as i32, 10);
        assert_eq!(type_metatype::TYPE_CODE as i32, 11);
        assert_eq!(type_metatype::TYPE_BOOL as i32, 12);
        assert_eq!(type_metatype::TYPE_UINT as i32, 13);
        assert_eq!(type_metatype::TYPE_INT as i32, 14);
        assert_eq!(type_metatype::TYPE_UNKNOWN as i32, 15);
        assert_eq!(type_metatype::TYPE_SPACEBASE as i32, 16);
        assert_eq!(type_metatype::TYPE_VOID as i32, 17);
    }

    /// `sub_metatype` ordering drives cast/propagation; the numeric values
    /// MATTER (transcribed from `type.hh:104-129`).
    #[test]
    fn sub_metatype_discriminants() {
        assert_eq!(sub_metatype::SUB_VOID as i32, 23);
        assert_eq!(sub_metatype::SUB_SPACEBASE as i32, 22);
        assert_eq!(sub_metatype::SUB_UNKNOWN as i32, 21);
        assert_eq!(sub_metatype::SUB_PARTIALSTRUCT as i32, 20);
        assert_eq!(sub_metatype::SUB_INT_CHAR as i32, 19);
        assert_eq!(sub_metatype::SUB_UINT_CHAR as i32, 18);
        assert_eq!(sub_metatype::SUB_INT_PLAIN as i32, 17);
        assert_eq!(sub_metatype::SUB_UINT_PLAIN as i32, 16);
        assert_eq!(sub_metatype::SUB_INT_ENUM as i32, 15);
        assert_eq!(sub_metatype::SUB_UINT_PARTIALENUM as i32, 14);
        assert_eq!(sub_metatype::SUB_UINT_ENUM as i32, 13);
        assert_eq!(sub_metatype::SUB_INT_UNICODE as i32, 12);
        assert_eq!(sub_metatype::SUB_UINT_UNICODE as i32, 11);
        assert_eq!(sub_metatype::SUB_BOOL as i32, 10);
        assert_eq!(sub_metatype::SUB_CODE as i32, 9);
        assert_eq!(sub_metatype::SUB_FLOAT as i32, 8);
        assert_eq!(sub_metatype::SUB_PTRREL_UNK as i32, 7);
        assert_eq!(sub_metatype::SUB_PTR as i32, 6);
        assert_eq!(sub_metatype::SUB_PTRREL as i32, 5);
        assert_eq!(sub_metatype::SUB_PTR_STRUCT as i32, 4);
        assert_eq!(sub_metatype::SUB_ARRAY as i32, 3);
        assert_eq!(sub_metatype::SUB_STRUCT as i32, 2);
        assert_eq!(sub_metatype::SUB_UNION as i32, 1);
        assert_eq!(sub_metatype::SUB_PARTIALUNION as i32, 0);
    }

    /// `Ord` on the enums must follow the discriminant order (used by
    /// `compare`'s `submeta < op.submeta`).
    #[test]
    fn sub_metatype_ord_matches_discriminant() {
        assert!(sub_metatype::SUB_PARTIALUNION < sub_metatype::SUB_VOID);
        assert!(sub_metatype::SUB_PTR < sub_metatype::SUB_FLOAT);
        assert!(sub_metatype::SUB_INT_PLAIN > sub_metatype::SUB_UINT_PLAIN);
    }

    /// `type_class` discriminants (type.hh:132-142), including the 100+ class jumps.
    #[test]
    fn type_class_discriminants() {
        assert_eq!(type_class::TYPECLASS_GENERAL as i32, 0);
        assert_eq!(type_class::TYPECLASS_FLOAT as i32, 1);
        assert_eq!(type_class::TYPECLASS_PTR as i32, 2);
        assert_eq!(type_class::TYPECLASS_HIDDENRET as i32, 3);
        assert_eq!(type_class::TYPECLASS_VECTOR as i32, 4);
        assert_eq!(type_class::TYPECLASS_CLASS1 as i32, 100);
        assert_eq!(type_class::TYPECLASS_CLASS2 as i32, 101);
        assert_eq!(type_class::TYPECLASS_CLASS3 as i32, 102);
        assert_eq!(type_class::TYPECLASS_CLASS4 as i32, 103);
    }

    /// `base2sub[m]` parity with `type.cc:23-27` for every metatype.
    #[test]
    fn base2sub_parity() {
        use sub_metatype::*;
        use type_metatype::*;
        assert_eq!(base2sub(TYPE_PARTIALUNION), SUB_PARTIALUNION);
        assert_eq!(base2sub(TYPE_PARTIALSTRUCT), SUB_PARTIALSTRUCT);
        assert_eq!(base2sub(TYPE_PARTIALENUM), SUB_UINT_PARTIALENUM);
        assert_eq!(base2sub(TYPE_UNION), SUB_UNION);
        assert_eq!(base2sub(TYPE_STRUCT), SUB_STRUCT);
        assert_eq!(base2sub(TYPE_ENUM_INT), SUB_INT_ENUM);
        assert_eq!(base2sub(TYPE_ENUM_UINT), SUB_UINT_ENUM);
        assert_eq!(base2sub(TYPE_ARRAY), SUB_ARRAY);
        assert_eq!(base2sub(TYPE_PTRREL), SUB_PTRREL);
        assert_eq!(base2sub(TYPE_PTR), SUB_PTR);
        assert_eq!(base2sub(TYPE_FLOAT), SUB_FLOAT);
        assert_eq!(base2sub(TYPE_CODE), SUB_CODE);
        assert_eq!(base2sub(TYPE_BOOL), SUB_BOOL);
        assert_eq!(base2sub(TYPE_UINT), SUB_UINT_PLAIN);
        assert_eq!(base2sub(TYPE_INT), SUB_INT_PLAIN);
        assert_eq!(base2sub(TYPE_UNKNOWN), SUB_UNKNOWN);
        assert_eq!(base2sub(TYPE_SPACEBASE), SUB_SPACEBASE);
        assert_eq!(base2sub(TYPE_VOID), SUB_VOID);
    }

    /// `metatype2string` round-trips through `string2metatype` for every
    /// metatype `string2metatype` recognizes.
    ///
    /// `TYPE_PARTIALENUM` is **deliberately excluded**: upstream C++
    /// `string2metatype` (type.cc:307-369) has no `"partenum"` case (its 'p'
    /// branch only matches ptr/ptrrel/partunion/partstruct), so the string
    /// `"partenum"` does not decode back. This asymmetry is faithfully
    /// reproduced — see [`partenum_string_is_not_decodable`].
    #[test]
    fn metatype_string_roundtrip() {
        use type_metatype::*;
        for m in [
            TYPE_PARTIALUNION,
            TYPE_PARTIALSTRUCT,
            TYPE_UNION,
            TYPE_STRUCT,
            TYPE_ENUM_INT,
            TYPE_ENUM_UINT,
            TYPE_ARRAY,
            TYPE_PTRREL,
            TYPE_PTR,
            TYPE_FLOAT,
            TYPE_CODE,
            TYPE_BOOL,
            TYPE_UINT,
            TYPE_INT,
            TYPE_UNKNOWN,
            TYPE_SPACEBASE,
            TYPE_VOID,
        ] {
            let s = metatype2string(m).unwrap();
            assert_eq!(string2metatype(&s).unwrap(), m, "roundtrip failed for {s}");
        }
    }

    /// Faithful upstream asymmetry: `metatype2string(TYPE_PARTIALENUM)` yields
    /// `"partenum"`, but `string2metatype("partenum")` is not a case in the C++
    /// and therefore errors (type.cc 'p' branch omits it).
    #[test]
    fn partenum_string_is_not_decodable() {
        assert_eq!(metatype2string(type_metatype::TYPE_PARTIALENUM).unwrap(), "partenum");
        assert!(string2metatype("partenum").is_err());
    }

    /// A couple of the load-bearing exact spellings (used in encode/decode).
    #[test]
    fn metatype_string_exact_spellings() {
        assert_eq!(metatype2string(type_metatype::TYPE_PARTIALUNION).unwrap(), "partunion");
        assert_eq!(metatype2string(type_metatype::TYPE_PARTIALSTRUCT).unwrap(), "partstruct");
        assert_eq!(metatype2string(type_metatype::TYPE_PARTIALENUM).unwrap(), "partenum");
        assert_eq!(metatype2string(type_metatype::TYPE_ENUM_INT).unwrap(), "enum_int");
        assert_eq!(metatype2string(type_metatype::TYPE_ENUM_UINT).unwrap(), "enum_uint");
        assert_eq!(metatype2string(type_metatype::TYPE_SPACEBASE).unwrap(), "spacebase");
    }

    #[test]
    fn string2metatype_unknown_errors() {
        assert!(string2metatype("notatype").is_err());
        // Known first char, unknown full string still errors (C++ break + throw).
        assert!(string2metatype("ptrx").is_err());
    }

    #[test]
    fn string2typeclass_parity() {
        use type_class::*;
        assert_eq!(string2typeclass("general").unwrap(), TYPECLASS_GENERAL);
        assert_eq!(string2typeclass("float").unwrap(), TYPECLASS_FLOAT);
        assert_eq!(string2typeclass("ptr").unwrap(), TYPECLASS_PTR);
        assert_eq!(string2typeclass("pointer").unwrap(), TYPECLASS_PTR);
        assert_eq!(string2typeclass("hiddenret").unwrap(), TYPECLASS_HIDDENRET);
        assert_eq!(string2typeclass("vector").unwrap(), TYPECLASS_VECTOR);
        assert_eq!(string2typeclass("class1").unwrap(), TYPECLASS_CLASS1);
        assert_eq!(string2typeclass("class4").unwrap(), TYPECLASS_CLASS4);
        // 'u' "unknown" maps to general (type.cc:409-411).
        assert_eq!(string2typeclass("unknown").unwrap(), TYPECLASS_GENERAL);
        assert!(string2typeclass("bogus").is_err());
    }

    #[test]
    fn metatype2typeclass_parity() {
        use type_class::*;
        use type_metatype::*;
        assert_eq!(metatype2typeclass(TYPE_FLOAT), TYPECLASS_FLOAT);
        assert_eq!(metatype2typeclass(TYPE_PTR), TYPECLASS_PTR);
        assert_eq!(metatype2typeclass(TYPE_INT), TYPECLASS_GENERAL);
        assert_eq!(metatype2typeclass(TYPE_STRUCT), TYPECLASS_GENERAL);
    }

    /// The 2-arg skeletal constructor (preserved from W3) sets align=-1 and
    /// derives `submeta` from `base2sub`.
    #[test]
    fn new_skeletal_derives_submeta() {
        let dt = Datatype::new(4, type_metatype::TYPE_INT);
        assert_eq!(dt.get_size(), 4);
        assert_eq!(dt.get_metatype(), type_metatype::TYPE_INT);
        assert_eq!(dt.get_sub_meta(), sub_metatype::SUB_INT_PLAIN);
        assert_eq!(dt.alignment, -1);
        assert_eq!(dt.align_size, 4);
        assert_eq!(dt.id, 0);
        assert_eq!(dt.flags, 0);
    }

    /// The 3-arg constructor mirrors `Datatype(s,align,m)`.
    #[test]
    fn new_with_align_parity() {
        let dt = Datatype::new_with_align(8, 8, type_metatype::TYPE_FLOAT);
        assert_eq!(dt.get_alignment(), 8);
        assert_eq!(dt.get_align_size(), 8);
        assert_eq!(dt.get_sub_meta(), sub_metatype::SUB_FLOAT);
    }

    /// Kind-payload construction: a pointer carries ptrto/wordsize and the
    /// dependency/accessor methods read it.
    #[test]
    fn pointer_kind_payload() {
        let ptrto = Rc::new(Datatype::new(4, type_metatype::TYPE_INT));
        let mut p = Datatype::new_with_align(8, -1, type_metatype::TYPE_PTR);
        p.kind = DatatypeKind::Pointer {
            ptrto: Rc::clone(&ptrto),
            spaceid: None,
            truncate: None,
            wordsize: 1,
        };
        assert_eq!(p.num_depend(), 1);
        assert_eq!(p.get_word_size(), Some(1));
        assert_eq!(p.get_ptr_to().unwrap().get_size(), 4);
        assert_eq!(p.get_depend(0).unwrap().get_size(), 4);
        let (into, off) = p.get_ptr_into().unwrap().unwrap();
        assert_eq!(into.get_size(), 4);
        assert_eq!(off, 0);
    }

    /// Kind-payload construction: an array carries arrayof/arraysize.
    #[test]
    fn array_kind_payload() {
        let elem = Rc::new(Datatype::new(2, type_metatype::TYPE_INT));
        let mut a = Datatype::new_with_align(20, -1, type_metatype::TYPE_ARRAY);
        a.kind = DatatypeKind::Array { arrayof: Rc::clone(&elem), arraysize: 10 };
        assert_eq!(a.num_depend(), 1);
        assert_eq!(a.num_elements(), Some(10));
        assert_eq!(a.get_array_base().unwrap().get_size(), 2);
    }

    /// Struct field count drives num_depend/get_depend.
    #[test]
    fn struct_kind_payload() {
        let f0 = TypeField::new(0, 0, "a", Rc::new(Datatype::new(4, type_metatype::TYPE_INT)));
        let f1 = TypeField::new(1, 4, "b", Rc::new(Datatype::new(2, type_metatype::TYPE_INT)));
        let mut s = Datatype::new_with_align(8, -1, type_metatype::TYPE_STRUCT);
        s.kind = DatatypeKind::Struct { field: vec![f0, f1], bitfield: vec![] };
        assert_eq!(s.num_depend(), 2);
        assert_eq!(s.get_depend(1).unwrap().get_size(), 2);
        assert!(s.get_depend(2).is_none());
    }

    /// The base `compare`/`compareDependency` (the implemented path) sort on
    /// submeta then size, per `type.cc:216-237`.
    #[test]
    fn compare_base_orders_submeta_then_size() {
        let int_t = Datatype::new(4, type_metatype::TYPE_INT); // submeta INT_PLAIN=17
        let uint_t = Datatype::new(4, type_metatype::TYPE_UINT); // submeta UINT_PLAIN=16
        // UINT_PLAIN(16) < INT_PLAIN(17): uint precedes int -> int.compare(uint) == 1.
        assert_eq!(int_t.compare(&uint_t, 10).unwrap(), 1);
        assert_eq!(uint_t.compare(&int_t, 10).unwrap(), -1);
        // Same submeta, different size: returns op.size - size.
        let int8_t = Datatype::new(8, type_metatype::TYPE_INT);
        assert_eq!(int_t.compare(&int8_t, 10).unwrap(), 8 - 4);
        assert_eq!(int_t.compare(&int_t, 10).unwrap(), 0);
        // compareDependency shares the body.
        assert_eq!(int_t.compare_dependency(&uint_t).unwrap(), 1);
    }

    /// `is_piece_structured` follows `metatype <= TYPE_ARRAY`.
    #[test]
    fn is_piece_structured_boundary() {
        assert!(Datatype::new(4, type_metatype::TYPE_ARRAY).is_piece_structured());
        assert!(Datatype::new(4, type_metatype::TYPE_STRUCT).is_piece_structured());
        assert!(!Datatype::new(8, type_metatype::TYPE_PTR).is_piece_structured());
        assert!(!Datatype::new(4, type_metatype::TYPE_INT).is_piece_structured());
    }

    /// Boolean property flags read out of `flags` correctly.
    #[test]
    fn flag_property_queries() {
        let mut dt = Datatype::new(1, type_metatype::TYPE_INT);
        assert!(!dt.is_core_type());
        assert!(!dt.needs_resolution());
        dt.flags |= flags::coretype | flags::needs_resolution | flags::chartype;
        assert!(dt.is_core_type());
        assert!(dt.needs_resolution());
        assert!(dt.is_ascii());
        assert!(dt.is_char_print());
        // display format encoded in bits 12-14.
        dt.flags |= 2 << 12; // dec
        assert_eq!(dt.get_display_format(), 2);
    }

    /// The structured-override compare is implemented in type-2: two identical
    /// empty structs compare equal; differing field counts order by the
    /// `(op.field.size() - field.size())` finisher.
    #[test]
    fn compare_struct_implemented() {
        let mut s = Datatype::new_with_align(8, -1, type_metatype::TYPE_STRUCT);
        s.kind = DatatypeKind::Struct { field: vec![], bitfield: vec![] };
        let other = s.clone();
        assert_eq!(s.compare(&other, 10).unwrap(), 0);
        // A struct with one field vs an empty struct: base compare ties (same
        // submeta/size), then field.size() differs: ts(0) - this(1) = -1.
        let mut s1 = Datatype::new_with_align(8, -1, type_metatype::TYPE_STRUCT);
        s1.kind = DatatypeKind::Struct {
            field: vec![TypeField::new(
                0,
                0,
                "a",
                Rc::new(Datatype::new(8, type_metatype::TYPE_INT)),
            )],
            bitfield: vec![],
        };
        // s1 (1 field) vs s (0 fields): returns op.field.size() - field.size() = -1.
        assert_eq!(s1.compare(&s, 10).unwrap(), -1);
        assert_eq!(s.compare(&s1, 10).unwrap(), 1);
    }

    /// `is_ptrsub_matching` base returns false for non-pointers (type.cc:555-559),
    /// and `TypePointer::isPtrsubMatching` (type.cc:1260-1312) returns false for a
    /// pointer to a non-structured (plain) type.  A pointer to a TYPE_STRUCT now
    /// walks the real (type-2) `ptrto.get_sub_type` and resolves the field.
    #[test]
    fn is_ptrsub_matching_routing() {
        // base: non-pointer -> false.
        let int_t = Datatype::new(4, type_metatype::TYPE_INT);
        assert!(!int_t.is_ptrsub_matching(0, 0, 1).unwrap());
        // pointer to plain int: TypePointer override hits the `else` -> false.
        let mut p = Datatype::new_with_align(8, -1, type_metatype::TYPE_PTR);
        p.kind = DatatypeKind::Pointer {
            ptrto: Rc::new(Datatype::new(4, type_metatype::TYPE_INT)),
            spaceid: None,
            truncate: None,
            wordsize: 1,
        };
        assert!(!p.is_ptrsub_matching(0, 0, 1).unwrap());
        // pointer to an array of int: off==0, multiplier(1) < element align-size(4)
        // -> matches (true).
        let mut elem = Datatype::new_with_align(4, 4, type_metatype::TYPE_INT);
        elem.align_size = 4;
        let mut arr = Datatype::new_with_align(8, 4, type_metatype::TYPE_ARRAY);
        arr.kind = DatatypeKind::Array { arrayof: Rc::new(elem), arraysize: 2 };
        let mut p_arr = Datatype::new_with_align(8, -1, type_metatype::TYPE_PTR);
        p_arr.kind = DatatypeKind::Pointer {
            ptrto: Rc::new(arr),
            spaceid: None,
            truncate: None,
            wordsize: 1,
        };
        assert!(p_arr.is_ptrsub_matching(0, 0, 1).unwrap());
        // pointer to a struct: the override now calls the real (type-2)
        // ptrto.get_sub_type.  Field "a" is an int4 at offset 0 (size 4) in an
        // 8-byte struct (align 4).  is_ptrsub_matching(off=0, extra=4, mult=1):
        // multiplier(1) < align_size(4); getSubType(0) -> the int (size 4),
        // newoff==0; extra(4) >= sub.size(4) and the int has no array slack, so
        // the field check fails -> false.
        let mut st = Datatype::new_with_align(8, 4, type_metatype::TYPE_STRUCT);
        let int4_field = {
            let mut f = Datatype::new_with_align(4, 4, type_metatype::TYPE_INT);
            f.align_size = 4;
            f
        };
        st.kind = DatatypeKind::Struct {
            field: vec![TypeField::new(0, 0, "a", Rc::new(int4_field))],
            bitfield: vec![],
        };
        st.align_size = 8;
        let mut p_st = Datatype::new_with_align(8, -1, type_metatype::TYPE_PTR);
        p_st.kind = DatatypeKind::Pointer {
            ptrto: Rc::new(st),
            spaceid: None,
            truncate: None,
            wordsize: 1,
        };
        assert!(!p_st.is_ptrsub_matching(0, 4, 1).unwrap());
        // extra=0 lands inside field "a" (0 < 4), so the same struct matches.
        assert!(p_st.is_ptrsub_matching(0, 0, 1).unwrap());
    }

    /// `is_primitive_whole` follows the C++ recursion (type.cc:505-518).
    #[test]
    fn is_primitive_whole_recursion() {
        // Plain int is primitive whole.
        assert!(Datatype::new(4, type_metatype::TYPE_INT).is_primitive_whole());
        // Array of one element of equal size -> recurses to the element.
        let elem = Rc::new(Datatype::new(4, type_metatype::TYPE_INT));
        let mut a = Datatype::new_with_align(4, -1, type_metatype::TYPE_ARRAY);
        a.kind = DatatypeKind::Array { arrayof: elem, arraysize: 1 };
        assert!(a.is_primitive_whole());
        // Struct with a smaller single component -> not whole (size mismatch).
        let f0 = TypeField::new(0, 0, "a", Rc::new(Datatype::new(4, type_metatype::TYPE_INT)));
        let mut s = Datatype::new_with_align(8, -1, type_metatype::TYPE_STRUCT);
        s.kind = DatatypeKind::Struct { field: vec![f0], bitfield: vec![] };
        assert!(!s.is_primitive_whole());
    }

    // -- Pointer / Array compare overrides (type.cc:1074-1108, 1363-1384) ----

    /// Build a distinct address space with a chosen index (for the spaceid
    /// tie-break in `TypePointer::compare`).
    fn space_with_index(idx: int4) -> Rc<AddrSpace> {
        use kuna_base::space::spacetype;
        Rc::new(AddrSpace::new(
            spacetype::IPTR_PROCESSOR,
            "ram",
            false,
            8,
            1,
            idx,
            0,
            0,
            0,
        ))
    }

    fn make_pointer(
        ptrto: Rc<Datatype>,
        spaceid: Option<Rc<AddrSpace>>,
        wordsize: uint4,
    ) -> Datatype {
        let mut p = Datatype::new_with_align(8, -1, type_metatype::TYPE_PTR);
        p.kind = DatatypeKind::Pointer { ptrto, spaceid, truncate: None, wordsize };
        p
    }

    fn make_array(arrayof: Rc<Datatype>, arraysize: int4, size: int4) -> Datatype {
        let mut a = Datatype::new_with_align(size, -1, type_metatype::TYPE_ARRAY);
        a.kind = DatatypeKind::Array { arrayof, arraysize };
        a
    }

    /// `TypePointer::compare` (type.cc:1074-1093): after the base step, tie-break
    /// on wordsize, then spaceid (no-space later, else by getIndex), then recurse
    /// into ptrto with a decremented level.
    #[test]
    fn pointer_compare_tiebreaks() {
        let int4_t = Rc::new(Datatype::new(4, type_metatype::TYPE_INT));
        // wordsize tie-break: smaller wordsize earlier.
        let p_ws1 = make_pointer(Rc::clone(&int4_t), None, 1);
        let p_ws2 = make_pointer(Rc::clone(&int4_t), None, 2);
        assert_eq!(p_ws1.compare(&p_ws2, 10).unwrap(), -1);
        assert_eq!(p_ws2.compare(&p_ws1, 10).unwrap(), 1);

        // spaceid tie-break: a pointer WITH a space comes earlier than one without.
        let spc = space_with_index(3);
        let p_nospace = make_pointer(Rc::clone(&int4_t), None, 1);
        let p_space = make_pointer(Rc::clone(&int4_t), Some(Rc::clone(&spc)), 1);
        assert_eq!(p_nospace.compare(&p_space, 10).unwrap(), 1); // no space -> later
        assert_eq!(p_space.compare(&p_nospace, 10).unwrap(), -1);

        // two spaces: ordered by index.
        let spc5 = space_with_index(5);
        let p_s3 = make_pointer(Rc::clone(&int4_t), Some(Rc::clone(&spc)), 1);
        let p_s5 = make_pointer(Rc::clone(&int4_t), Some(Rc::clone(&spc5)), 1);
        assert_eq!(p_s3.compare(&p_s5, 10).unwrap(), -1);
        assert_eq!(p_s5.compare(&p_s3, 10).unwrap(), 1);

        // ptrto recursion: ptr-to-int4 vs ptr-to-int8 -> recurse, op.size - size = 4.
        let int8_t = Rc::new(Datatype::new(8, type_metatype::TYPE_INT));
        let p_int4 = make_pointer(Rc::clone(&int4_t), None, 1);
        let p_int8 = make_pointer(Rc::clone(&int8_t), None, 1);
        assert_eq!(p_int4.compare(&p_int8, 10).unwrap(), 8 - 4);

        // level == 0 short-circuits to id comparison instead of recursing.
        let mut p_a = make_pointer(Rc::clone(&int4_t), None, 1);
        let mut p_b = make_pointer(Rc::clone(&int8_t), None, 1);
        p_a.id = 100;
        p_b.id = 200;
        assert_eq!(p_a.compare(&p_b, 0).unwrap(), -1); // level-1 < 0 -> id 100 < 200
        assert_eq!(p_b.compare(&p_a, 0).unwrap(), 1);
    }

    /// `TypePointer::compareDependency` (type.cc:1095-1108): submeta, then
    /// ptrto-identity, wordsize, spaceid, then op.size - size.
    #[test]
    fn pointer_compare_dependency_uses_ptr_identity() {
        let int4_a = Rc::new(Datatype::new(4, type_metatype::TYPE_INT));
        let int4_b = Rc::new(Datatype::new(4, type_metatype::TYPE_INT)); // distinct alloc
        let p_a = make_pointer(Rc::clone(&int4_a), None, 1);
        let p_a2 = make_pointer(Rc::clone(&int4_a), None, 1); // same ptrto alloc
        let p_b = make_pointer(Rc::clone(&int4_b), None, 1); // different ptrto alloc
        // Same ptrto identity + same wordsize/space/size -> 0.
        assert_eq!(p_a.compare_dependency(&p_a2).unwrap(), 0);
        // Different ptrto identity -> nonzero, and antisymmetric.
        let ab = p_a.compare_dependency(&p_b).unwrap();
        let ba = p_b.compare_dependency(&p_a).unwrap();
        assert!(ab != 0);
        assert_eq!(ab, -ba);
    }

    /// `TypeArray::compare` (type.cc:1363-1375) recurses into the element with a
    /// decremented level, and `compareDependency` (1377-1384) uses element
    /// pointer identity.
    #[test]
    fn array_compare_and_dependency() {
        let int4_t = Rc::new(Datatype::new(4, type_metatype::TYPE_INT));
        let int8_t = Rc::new(Datatype::new(8, type_metatype::TYPE_INT));
        // array[2] of int4 (size 8) vs array[1] of int8 (size 8): same submeta,
        // same size -> base step 0; recurse into element: op.size(8)-size(4)=4.
        let a_i4 = make_array(Rc::clone(&int4_t), 2, 8);
        let a_i8 = make_array(Rc::clone(&int8_t), 1, 8);
        assert_eq!(a_i4.compare(&a_i8, 10).unwrap(), 4);
        assert_eq!(a_i8.compare(&a_i4, 10).unwrap(), -4);

        // compareDependency uses element identity; same element alloc -> 0.
        let a_same = make_array(Rc::clone(&int4_t), 2, 8);
        assert_eq!(a_i4.compare_dependency(&a_same).unwrap(), 0);

        // level == 0 short-circuits to id comparison.
        let mut a1 = make_array(Rc::clone(&int4_t), 2, 8);
        let mut a2 = make_array(Rc::clone(&int8_t), 1, 8);
        a1.id = 7;
        a2.id = 9;
        assert_eq!(a1.compare(&a2, 0).unwrap(), -1);
    }

    /// **Total-order fidelity** (the riskiest surface).  Over a corpus of the
    /// in-scope kinds (plain scalars, pointers with varying ptrto/wordsize/space,
    /// arrays), `compare`/`compareDependency` must be a total order:
    /// reflexive (x∘x==0), antisymmetric (sign(x∘y) == -sign(y∘x)), and the sign
    /// must be consistent (a strict weak ordering — equal classes are
    /// transitive).  A wrong subclass grouping silently corrupts the TypeFactory
    /// tree, so this generated matrix is the guard.
    #[test]
    fn compare_total_order_matrix() {
        let i1 = Rc::new(Datatype::new(1, type_metatype::TYPE_INT));
        let i4 = Rc::new(Datatype::new(4, type_metatype::TYPE_INT));
        let u4 = Rc::new(Datatype::new(4, type_metatype::TYPE_UINT));
        let f8 = Rc::new(Datatype::new(8, type_metatype::TYPE_FLOAT));
        let b1 = Rc::new(Datatype::new(1, type_metatype::TYPE_BOOL));
        let spc = space_with_index(2);

        let corpus: Vec<Rc<Datatype>> = vec![
            Rc::clone(&i1),
            Rc::clone(&i4),
            Rc::clone(&u4),
            Rc::clone(&f8),
            Rc::clone(&b1),
            Rc::new(make_pointer(Rc::clone(&i4), None, 1)),
            Rc::new(make_pointer(Rc::clone(&i4), None, 2)),
            Rc::new(make_pointer(Rc::clone(&i4), Some(Rc::clone(&spc)), 1)),
            Rc::new(make_pointer(Rc::clone(&f8), None, 1)),
            Rc::new(make_array(Rc::clone(&i4), 2, 8)),
            Rc::new(make_array(Rc::clone(&i1), 4, 4)),
        ];

        let sign = |x: int4| -> int4 { x.signum() };

        // Reflexivity + antisymmetry for both comparators.
        for a in &corpus {
            assert_eq!(a.compare(a, 10).unwrap(), 0, "compare not reflexive");
            assert_eq!(
                a.compare_dependency(a).unwrap(),
                0,
                "compareDependency not reflexive"
            );
            for b in &corpus {
                let ab = sign(a.compare(b, 10).unwrap());
                let ba = sign(b.compare(a, 10).unwrap());
                assert_eq!(ab, -ba, "compare not antisymmetric");
                let dab = sign(a.compare_dependency(b).unwrap());
                let dba = sign(b.compare_dependency(a).unwrap());
                assert_eq!(dab, -dba, "compareDependency not antisymmetric");
            }
        }

        // Transitivity of the strict order: if a<b and b<c then a<c, over all
        // ordered triples (compare with level 10).
        for a in &corpus {
            for b in &corpus {
                for c in &corpus {
                    let ab = sign(a.compare(b, 10).unwrap());
                    let bc = sign(b.compare(c, 10).unwrap());
                    let ac = sign(a.compare(c, 10).unwrap());
                    if ab < 0 && bc < 0 {
                        assert!(ac < 0, "compare not transitive (a<b<c => a<c)");
                    }
                    if ab == 0 && bc == 0 {
                        assert_eq!(ac, 0, "compare equality not transitive");
                    }
                }
            }
        }
    }

    /// `getSubType` for the in-scope pointer/array kinds (type.cc:1061-1072,
    /// 1386-1393).
    #[test]
    fn get_sub_type_pointer_and_array() {
        // Array: off < size -> element with renormalized offset modulo align-size.
        let mut elem = Datatype::new_with_align(4, 4, type_metatype::TYPE_INT);
        elem.align_size = 4;
        let arr = make_array(Rc::new(elem), 3, 12);
        let (sub, newoff) = arr.get_sub_type(7).unwrap();
        assert_eq!(sub.unwrap().get_size(), 4); // element
        assert_eq!(newoff, 7 % 4); // 3
        // off >= size -> base (None, off).
        let (sub, newoff) = arr.get_sub_type(12).unwrap();
        assert!(sub.is_none());
        assert_eq!(newoff, 12);

        // Pointer without truncate -> base (None, off).
        let int4_t = Rc::new(Datatype::new(4, type_metatype::TYPE_INT));
        let p = make_pointer(Rc::clone(&int4_t), None, 1);
        let (sub, newoff) = p.get_sub_type(3).unwrap();
        assert!(sub.is_none());
        assert_eq!(newoff, 3);

        // Pointer WITH a truncate window (little-endian: min == 0).
        let trunc = Rc::new(Datatype::new(4, type_metatype::TYPE_PTR));
        let mut p_tr = Datatype::new_with_align(8, -1, type_metatype::TYPE_PTR);
        p_tr.kind = DatatypeKind::Pointer {
            ptrto: Rc::clone(&int4_t),
            spaceid: None,
            truncate: Some(Rc::clone(&trunc)),
            wordsize: 1,
        };
        // off in [0,4) -> the truncated form, newoff = off - 0.
        let (sub, newoff) = p_tr.get_sub_type(2).unwrap();
        assert_eq!(sub.unwrap().get_size(), 4);
        assert_eq!(newoff, 2);
        // off >= 4 -> outside the window -> base (None, off).
        let (sub, newoff) = p_tr.get_sub_type(5).unwrap();
        assert!(sub.is_none());
        assert_eq!(newoff, 5);
    }

    /// `nearestArrayedComponent{Forward,Backward}` for the TypeArray override and
    /// the base default (type.cc:1395-1413, 190-209).
    #[test]
    fn nearest_arrayed_component() {
        let mut elem = Datatype::new_with_align(4, 4, type_metatype::TYPE_INT);
        elem.align_size = 4;
        let arr = make_array(Rc::new(elem), 3, 12);
        // forward: off<=0 -> distance -off, element size = align-size of element.
        let (dist, newoff, el) = arr.nearest_arrayed_component_forward(0, 128).unwrap();
        assert_eq!((dist, newoff, el), (0, 0, 4));
        // forward: off>0 -> -1 (in middle of array).
        let (dist, _, _) = arr.nearest_arrayed_component_forward(4, 128).unwrap();
        assert_eq!(dist, -1);
        // backward: off>=0 and off<=size -> size - off.
        let (dist, newoff, el) = arr.nearest_arrayed_component_backward(4, 128).unwrap();
        assert_eq!((dist, newoff, el), (12 - 4, 4, 4));
        // backward: off<0 -> -1.
        let (dist, _, _) = arr.nearest_arrayed_component_backward(-1, 128).unwrap();
        assert_eq!(dist, -1);

        // base default for a non-array: -1.
        let int4_t = Datatype::new(4, type_metatype::TYPE_INT);
        assert_eq!(int4_t.nearest_arrayed_component_forward(0, 128).unwrap().0, -1);
        assert_eq!(int4_t.nearest_arrayed_component_backward(0, 128).unwrap().0, -1);
    }

    /// `TypeArray::getHoleSize` (type.cc:1415-1420) renormalizes into the element
    /// and recurses; for a plain element (no holes) the result is 0.
    #[test]
    fn array_get_hole_size() {
        let mut elem = Datatype::new_with_align(4, 4, type_metatype::TYPE_INT);
        elem.align_size = 4;
        let arr = make_array(Rc::new(elem), 3, 12);
        assert_eq!(arr.get_hole_size(5).unwrap(), 0);
    }

    /// LOSS-050 restored: `resolveInFlow`/`findResolve` return the receiver
    /// unchanged for every kind without a union override; the array and
    /// pointer-to-union paths still SEAM.
    #[test]
    fn resolve_in_flow_returns_self_for_plain_kinds() {
        let int_t = Rc::new(Datatype::new(4, type_metatype::TYPE_INT));
        let op = crate::seams::OpId::default();
        let r = int_t.resolve_in_flow(op, 0).unwrap();
        assert!(Rc::ptr_eq(&r, &int_t));
        let f = int_t.find_resolve(op, -1).unwrap();
        assert!(Rc::ptr_eq(&f, &int_t));

        // Pointer to a non-union -> return self.
        let p: Rc<Datatype> = Rc::new(make_pointer(Rc::clone(&int_t), None, 1));
        let rp = p.resolve_in_flow(op, 0).unwrap();
        assert!(Rc::ptr_eq(&rp, &p));

        // Array always SEAMs (needs Funcdata scoring).
        let a: Rc<Datatype> = Rc::new(make_array(Rc::clone(&int_t), 2, 8));
        assert!(a.resolve_in_flow(op, 0).is_err());

        // Pointer to a UNION -> SEAM (needs Funcdata).
        let mut u = Datatype::new_with_align(8, -1, type_metatype::TYPE_UNION);
        u.kind = DatatypeKind::Union { field: vec![] };
        let p_u: Rc<Datatype> = Rc::new(make_pointer(Rc::new(u), None, 1));
        assert!(p_u.resolve_in_flow(op, 0).is_err());
    }

    // ----------------------------------------------------------------------
    // TypePointerRel (type-3) compare/compareDependency/isPtrsubMatching
    // ----------------------------------------------------------------------

    fn make_pointer_rel(
        ptrto: Rc<Datatype>,
        wordsize: uint4,
        stripped: Option<Rc<Datatype>>,
        parent: Rc<Datatype>,
        offset: int4,
    ) -> Datatype {
        let mut pr = Datatype::new_with_align(8, -1, type_metatype::TYPE_PTRREL);
        pr.submeta = sub_metatype::SUB_PTRREL;
        pr.kind = DatatypeKind::PointerRel { ptrto, wordsize, stripped, parent, offset };
        pr
    }

    /// `TypePointerRel::compare` (type.cc:3072-3090): compares as a plain pointer
    /// first, then prefers the *formal* version (stripped==None) over the ephemeral
    /// one (stripped==Some).
    #[test]
    fn pointer_rel_compare_prefers_formal() {
        let int4_t = Rc::new(Datatype::new(4, type_metatype::TYPE_INT));
        let parent = Rc::new(Datatype::new(16, type_metatype::TYPE_STRUCT));
        let strip: Rc<Datatype> = Rc::new(make_pointer(Rc::clone(&int4_t), None, 1));
        let formal = make_pointer_rel(Rc::clone(&int4_t), 1, None, Rc::clone(&parent), 4);
        let ephemeral =
            make_pointer_rel(Rc::clone(&int4_t), 1, Some(strip), Rc::clone(&parent), 4);
        // formal (None) vs ephemeral (Some) -> formal preferred (earlier) = -1.
        assert_eq!(formal.compare(&ephemeral, 10).unwrap(), -1);
        assert_eq!(ephemeral.compare(&formal, 10).unwrap(), 1);
        // Two formals with identical fields tie at 0.
        let formal2 = make_pointer_rel(Rc::clone(&int4_t), 1, None, Rc::clone(&parent), 4);
        assert_eq!(formal.compare(&formal2, 10).unwrap(), 0);
    }

    /// `TypePointerRel::compareDependency` (type.cc:3092-3103): submeta, ptrto
    /// identity, offset, parent identity, wordsize, then (op.size - size).
    #[test]
    fn pointer_rel_compare_dependency_fields() {
        let int4_t = Rc::new(Datatype::new(4, type_metatype::TYPE_INT));
        let parent = Rc::new(Datatype::new(16, type_metatype::TYPE_STRUCT));
        let a = make_pointer_rel(Rc::clone(&int4_t), 1, None, Rc::clone(&parent), 4);
        let a2 = make_pointer_rel(Rc::clone(&int4_t), 1, None, Rc::clone(&parent), 4);
        // Same ptrto/offset/parent/wordsize/size identity -> 0.
        assert_eq!(a.compare_dependency(&a2).unwrap(), 0);
        // Different offset -> ordered by offset.
        let b = make_pointer_rel(Rc::clone(&int4_t), 1, None, Rc::clone(&parent), 8);
        assert_eq!(a.compare_dependency(&b).unwrap(), -1);
        assert_eq!(b.compare_dependency(&a).unwrap(), 1);
    }

    /// `TypePointerRel::isPtrsubMatching` (type.cc:3138-3147): with stripped==None,
    /// the bound check is `0 <= offset+extra+iOff <= parent.getSize()`.
    #[test]
    fn pointer_rel_is_ptrsub_matching_bounds() {
        let int4_t = Rc::new(Datatype::new(4, type_metatype::TYPE_INT));
        let parent = Rc::new(Datatype::new(16, type_metatype::TYPE_STRUCT));
        let pr = make_pointer_rel(Rc::clone(&int4_t), 1, None, Rc::clone(&parent), 4);
        assert!(pr.is_ptrsub_matching(0, 0, 1).unwrap()); // 4 in [0,16]
        assert!(pr.is_ptrsub_matching(0, 12, 1).unwrap()); // 16 in [0,16]
        assert!(!pr.is_ptrsub_matching(0, 13, 1).unwrap()); // 17 > 16
        assert!(!pr.is_ptrsub_matching(-5, 0, 1).unwrap()); // -1 < 0
    }

    // ----------------------------------------------------------------------
    // TypeFactoryImpl — findAdd dedup, core-type cache, downChain/getExactPiece
    // ----------------------------------------------------------------------

    /// A minimally-configured factory: default alignment map + the size-1
    /// TYPE_UNKNOWN base cached (needed by getBase's oversize-array path and by
    /// partial-type construction), and a max-basetype-size of 8.
    fn factory() -> TypeFactoryImpl {
        let f = TypeFactoryImpl::new();
        f.set_default_alignment_map();
        f.set_max_basetype_size(8);
        f
    }

    /// `findAdd` interns by structural identity: two structurally-identical
    /// requests return the *same* `Rc` allocation; a different request does not.
    #[test]
    fn factory_find_add_dedup_identity() {
        let f = factory();
        let i4_a = f.get_base(4, type_metatype::TYPE_INT).unwrap();
        let i4_b = f.get_base(4, type_metatype::TYPE_INT).unwrap();
        assert!(Rc::ptr_eq(&i4_a, &i4_b), "identical getBase requests dedup to one object");
        let i8 = f.get_base(8, type_metatype::TYPE_INT).unwrap();
        assert!(!Rc::ptr_eq(&i4_a, &i8), "different size -> distinct object");

        // Pointer-to-int dedups, and the ptrto sub-type is itself the interned one.
        let p_a = f.get_type_pointer(8, Rc::clone(&i4_a), 1).unwrap();
        let p_b = f.get_type_pointer(8, Rc::clone(&i4_b), 1).unwrap();
        assert!(Rc::ptr_eq(&p_a, &p_b), "identical pointers dedup");
        assert!(Rc::ptr_eq(&p_a.get_ptr_to().unwrap(), &i4_a));
    }

    /// `getBaseNoChar` returns the cached non-char int for size-1 INT, and the
    /// char/int caches behave per `cacheCoreTypes`.
    #[test]
    fn factory_core_type_cache() {
        use type_metatype::*;
        let f = factory();
        // setCoreType for the primitives cacheCoreTypes reads.  `type_nochar` is
        // seeded by a *non*-char 1-byte INT (the C++ "sbyte"); the ASCII "char"
        // takes the typecache[1][INT] slot, so getBase(1,INT) prefers it.
        f.set_core_type("void", 0, TYPE_VOID, false).unwrap();
        f.set_core_type("byte", 1, TYPE_UINT, false).unwrap();
        f.set_core_type("sbyte", 1, TYPE_INT, false).unwrap(); // non-char 1-byte INT
        f.set_core_type("char", 1, TYPE_INT, true).unwrap(); // chartype -> ASCII
        f.set_core_type("undefined", 1, TYPE_UNKNOWN, false).unwrap();
        f.set_core_type("int", 4, TYPE_INT, false).unwrap();
        f.set_core_type("double", 8, TYPE_FLOAT, false).unwrap();
        f.cache_core_types().unwrap();

        // getBase(1,TYPE_INT) prefers the ASCII char (cacheCoreTypes seeds the
        // typecache[1][INT] slot with the char).
        let c = f.get_base(1, TYPE_INT).unwrap();
        assert!(c.is_ascii(), "getBase(1,INT) returns the char (preferred over nochar)");
        // getBaseNoChar(1,TYPE_INT) returns the cached type_nochar instead.
        let nc = f.get_base_no_char(1, TYPE_INT).unwrap();
        assert!(!nc.is_ascii(), "getBaseNoChar(1,INT) returns the non-char int");
        // getTypeChar(1) returns the cached 1-byte char.
        let ch = f.get_type_char(1).unwrap();
        assert!(ch.is_ascii());
        // The 4-byte int and 8-byte float are cached and dedup on re-request.
        let i4 = f.get_base(4, TYPE_INT).unwrap();
        let i4b = f.get_base(4, TYPE_INT).unwrap();
        assert!(Rc::ptr_eq(&i4, &i4b));
        let d = f.get_base(8, TYPE_FLOAT).unwrap();
        assert_eq!(d.get_metatype(), TYPE_FLOAT);
    }

    /// `concretize` collapses a size-1 TYPE_CODE to a size-1 TYPE_UNKNOWN, and is
    /// the identity on already-concrete types.
    #[test]
    fn factory_concretize() {
        let f = factory();
        // Seed a size-1 unknown so getBase(1,UNKNOWN) is available.
        let unk = f.get_base(1, type_metatype::TYPE_UNKNOWN).unwrap();
        let code = f.get_type_code().unwrap();
        assert_eq!(code.get_metatype(), type_metatype::TYPE_CODE);
        let conc = f.concretize(code).unwrap();
        assert_eq!(conc.get_metatype(), type_metatype::TYPE_UNKNOWN);
        assert!(Rc::ptr_eq(&conc, &unk));
        // Already concrete -> identity.
        let i4 = f.get_base(4, type_metatype::TYPE_INT).unwrap();
        assert!(Rc::ptr_eq(&f.concretize(Rc::clone(&i4)).unwrap(), &i4));
    }

    /// `getExactPiece` drills into a struct to the exact field, builds a partial
    /// struct when the range straddles, and returns the whole when sizes match.
    #[test]
    fn factory_get_exact_piece_matrix() {
        use type_metatype::*;
        let f = factory();
        f.get_base(1, TYPE_UNKNOWN).unwrap(); // needed for partial stripped types
        let i4 = f.get_base(4, TYPE_INT).unwrap();
        // struct { int a@0; int b@4; } size 8.
        let mut s = Datatype::new_with_align(8, 4, TYPE_STRUCT);
        s.kind = DatatypeKind::Struct {
            field: vec![
                TypeField::new(0, 0, "a", Rc::clone(&i4)),
                TypeField::new(1, 4, "b", Rc::clone(&i4)),
            ],
            bitfield: vec![],
        };
        let s = f.find_add(s).unwrap();

        // Whole-struct size match -> the struct itself.
        let whole = f.get_exact_piece(Rc::clone(&s), 0, 8).unwrap().unwrap();
        assert!(Rc::ptr_eq(&whole, &s));
        // Field b (offset 4, size 4) -> the int field exactly.
        let field_b = f.get_exact_piece(Rc::clone(&s), 4, 4).unwrap().unwrap();
        assert!(Rc::ptr_eq(&field_b, &i4));
        // A 2-byte slice wholly inside a 4-byte field -> null (only a partial field).
        assert!(f.get_exact_piece(Rc::clone(&s), 0, 2).unwrap().is_none());
        // A 4-byte slice at offset 2 crosses the a/b field boundary -> partial struct.
        let part = f.get_exact_piece(Rc::clone(&s), 2, 4).unwrap().unwrap();
        assert_eq!(part.get_metatype(), TYPE_PARTIALSTRUCT);
        assert_eq!(part.get_size(), 4);
    }

    /// `getPtrToFromParent` (type.cc:3157-3171) walks `getSubType` down a container
    /// to the data-type that a relative pointer at the given byte offset points
    /// directly at; a non-positive or escaping offset falls back to the size-1
    /// `getBase(1,TYPE_UNKNOWN)`.  This is the `pointer setting <n> <s> offset <o>`
    /// relptr ptrto resolution.
    #[test]
    fn factory_get_ptr_to_from_parent() {
        use type_metatype::*;
        let f = factory();
        let unk1 = f.get_base(1, TYPE_UNKNOWN).unwrap();
        let i4 = f.get_base(4, TYPE_INT).unwrap();
        let f4 = f.get_base(4, TYPE_FLOAT).unwrap();
        // struct mystruct { int a@0; float b@4; int c@8; int d@12; } size 16.
        let mut s = Datatype::new_with_align(16, 4, TYPE_STRUCT);
        s.kind = DatatypeKind::Struct {
            field: vec![
                TypeField::new(0, 0, "a", Rc::clone(&i4)),
                TypeField::new(1, 4, "b", Rc::clone(&f4)),
                TypeField::new(2, 8, "c", Rc::clone(&i4)),
                TypeField::new(3, 12, "d", Rc::clone(&i4)),
            ],
            bitfield: vec![],
        };
        let s = f.find_add(s).unwrap();

        // offset 8 lands exactly on field `c` (the int) — `pointer setting ...
        // offset 8` in pointerrel.xml.
        let at8 = f.get_ptr_to_from_parent(Rc::clone(&s), 8).unwrap();
        assert!(Rc::ptr_eq(&at8, &i4), "offset 8 resolves to field c (int)");
        // offset 4 lands on field `b` (the float).
        let at4 = f.get_ptr_to_from_parent(Rc::clone(&s), 4).unwrap();
        assert!(Rc::ptr_eq(&at4, &f4), "offset 4 resolves to field b (float)");
        // offset 0 is the non-positive fallback -> getBase(1,TYPE_UNKNOWN).
        let at0 = f.get_ptr_to_from_parent(Rc::clone(&s), 0).unwrap();
        assert!(Rc::ptr_eq(&at0, &unk1), "offset 0 falls back to unknown1");
        // offset 100 escapes the container -> getSubType yields None -> unknown1.
        let oob = f.get_ptr_to_from_parent(Rc::clone(&s), 100).unwrap();
        assert!(Rc::ptr_eq(&oob, &unk1), "escaping offset falls back to unknown1");
    }

    /// `downChain` on a pointer-to-struct descends to the field pointer and passes
    /// back the container; on a pointer-to-array it descends to the element.
    #[test]
    fn factory_down_chain_matrix() {
        use type_metatype::*;
        let f = factory();
        let i4 = f.get_base(4, TYPE_INT).unwrap();
        // struct { int a@0; int b@4; }
        let mut s = Datatype::new_with_align(8, 4, TYPE_STRUCT);
        s.kind = DatatypeKind::Struct {
            field: vec![
                TypeField::new(0, 0, "a", Rc::clone(&i4)),
                TypeField::new(1, 4, "b", Rc::clone(&i4)),
            ],
            bitfield: vec![],
        };
        let s = f.find_add(s).unwrap();
        let ps = f.get_type_pointer(8, Rc::clone(&s), 1).unwrap();
        // downChain at offset 4 -> pointer to int (field b), off renormalized to 0,
        // and the container (the pointer-to-struct) passed back at parOff 4.
        let (res, off, par, par_off) = f.down_chain(&ps, 4, false).unwrap();
        let res = res.expect("downChain found the field");
        assert_eq!(res.get_metatype(), TYPE_PTR);
        assert!(Rc::ptr_eq(&res.get_ptr_to().unwrap(), &i4));
        assert_eq!(off, 0);
        assert!(Rc::ptr_eq(&par.unwrap(), &ps));
        assert_eq!(par_off, 4);

        // Pointer-to-array of 4 ints; downChain at offset 8 -> element pointer.
        let arr = f.get_type_array(4, Rc::clone(&i4)).unwrap();
        let pa = f.get_type_pointer(8, Rc::clone(&arr), 1).unwrap();
        let (res2, off2, _par2, _po2) = f.down_chain(&pa, 8, false).unwrap();
        let res2 = res2.expect("downChain into array element");
        assert!(Rc::ptr_eq(&res2.get_ptr_to().unwrap(), &i4));
        assert_eq!(off2, 0);
    }

    /// `get_sub_type_via_factory` RESOLVES the type-2 `TypeCode::getSubType` seam:
    /// a bound factory returns `getBase(1,TYPE_CODE)` with newoff 0.
    #[test]
    fn factory_resolves_typecode_get_sub_type_seam() {
        let f = factory();
        let code = f.get_type_code().unwrap();
        // The bare method still seams (no bound factory on the value itself).
        assert!(code.get_sub_type(0).is_err());
        // The factory-aware path resolves to getBase(1, TYPE_CODE), newoff 0.
        let (sub, newoff) = f.get_sub_type_via_factory(&code, 0).unwrap();
        let sub = sub.expect("factory resolves the code byte");
        assert_eq!(sub.get_metatype(), type_metatype::TYPE_CODE);
        assert_eq!(sub.get_size(), 1);
        assert_eq!(newoff, 0);
    }

    /// `hashName`/`hashSize` reproduce the C++ hashes bit-for-bit (the header bits
    /// and the size reversibility property).
    #[test]
    fn factory_hash_name_and_size() {
        // Name hashes carry the two top header bits.
        let h = Datatype::hash_name("int");
        assert_eq!(h & 0xC000000000000000, 0xC000000000000000);
        // Determinism.
        assert_eq!(h, Datatype::hash_name("int"));
        assert_ne!(h, Datatype::hash_name("uint"));
        // hashSize is reversible: applying twice with the same size restores the id.
        let id = 0x1234_5678_9abc_def0u64;
        let sized = Datatype::hash_size(id, 7);
        assert_ne!(sized, id);
        assert_eq!(Datatype::hash_size(sized, 7), id);
    }

    // =========================================================================
    // collectBitFields / hasBitFieldsInRange / BitFieldTriple (type.cc:1804-1852,
    // 932-949) — the bitfield query layer the bitfield transforms read.
    // =========================================================================

    /// Build a one-byte container's three bitfields, modeling `myfoo`'s first
    /// byte (`bitfields.xml`): `uint4 field3:3` (lsb0,3b), `int4 sfield4:4`
    /// (lsb3,4b), `bool fieldb:1` (lsb7,1b).  All little-endian, byteOffset 0,
    /// byteSize 1.
    fn myfoo_byte0_bitfields() -> Vec<TypeBitField> {
        let mk = |id, lsb, nb, name: &str, meta| {
            let mut bf =
                TypeBitField::new(id, nb, false, name, Rc::new(Datatype::new(4, meta)));
            // TypeBitField::new puts the field at lsb 0; place it at the real
            // container position for the layout the struct holds.
            bf.least_sig_bit = lsb;
            bf.byte_offset = 0;
            bf.byte_size = 1;
            bf
        };
        vec![
            mk(0, 0, 3, "field3", type_metatype::TYPE_UINT),
            mk(1, 3, 4, "sfield4", type_metatype::TYPE_INT),
            mk(2, 7, 1, "fieldb", type_metatype::TYPE_BOOL),
        ]
    }

    fn struct_with_bitfields(field: Vec<TypeField>, bitfield: Vec<TypeBitField>, size: int4) -> Datatype {
        let mut s = Datatype::new_with_align(size, 4, type_metatype::TYPE_STRUCT);
        if !bitfield.is_empty() {
            s.flags |= flags::has_bitfields;
        }
        s.kind = DatatypeKind::Struct { field, bitfield };
        s
    }

    #[test]
    fn collect_bit_fields_single_byte_container() {
        let s = struct_with_bitfields(vec![], myfoo_byte0_bitfields(), 1);
        let mut res = Vec::new();
        s.collect_bit_fields(0, &mut res, 0, 1);
        // All three bitfields overlap byte [0,1).
        assert_eq!(res.len(), 3);
        let names: Vec<&str> = res.iter().map(|t| t.bitfield.name.as_str()).collect();
        assert_eq!(names, vec!["field3", "sfield4", "fieldb"]);
        // offset (container offset within parent) is 0 for all.
        assert!(res.iter().all(|t| t.offset == 0));
    }

    #[test]
    fn collect_bit_fields_range_outside_container_is_empty() {
        let s = struct_with_bitfields(vec![], myfoo_byte0_bitfields(), 4);
        // Query a byte range past the bitfield container (offset 2, size 1):
        // compareMaxByte upper_bound lands at end() -> no bitfields.
        let mut res = Vec::new();
        s.collect_bit_fields(0, &mut res, 2, 1);
        assert!(res.is_empty());
    }

    #[test]
    fn has_bit_fields_in_range_matches_collect() {
        let s = struct_with_bitfields(vec![], myfoo_byte0_bitfields(), 4);
        assert!(s.has_bit_fields_in_range(0, 1));
        assert!(!s.has_bit_fields_in_range(2, 2));
        // A non-struct never has bitfields in range.
        let i = Datatype::new(4, type_metatype::TYPE_INT);
        assert!(!i.has_bit_fields_in_range(0, 4));
    }

    #[test]
    fn collect_bit_fields_nested_struct_recurses() {
        // Outer struct: field "inner" at offset 4 is a struct holding byte0
        // bitfields; the outer also needs has_bitfields on the inner field type.
        let inner = Rc::new(struct_with_bitfields(vec![], myfoo_byte0_bitfields(), 1));
        let inner_field = TypeField::new(0, 4, "inner", inner);
        let outer = struct_with_bitfields(vec![inner_field], vec![], 8);
        // Query the whole outer range — the recursion shifts offset by the field
        // offset (4) and reports the container offset as base_offset + 4.
        let mut res = Vec::new();
        outer.collect_bit_fields(0, &mut res, 0, 8);
        assert_eq!(res.len(), 3);
        assert!(res.iter().all(|t| t.offset == 4));
    }

    #[test]
    fn bitfield_triple_compare_orders_least_to_most_significant() {
        // Little-endian: smaller byte offset is less significant; within a byte,
        // smaller leastSigBit is less significant.
        let bf = myfoo_byte0_bitfields();
        let t_field3 = BitFieldTriple::new(bf[0].clone(), 0); // lsb0
        let t_sfield4 = BitFieldTriple::new(bf[1].clone(), 0); // lsb3
        // field3 (lsb0) sorts before sfield4 (lsb3).
        assert!(BitFieldTriple::compare(&t_field3, &t_sfield4));
        assert!(!BitFieldTriple::compare(&t_sfield4, &t_field3));
        // Equal start bit -> neither strictly less.
        assert!(!BitFieldTriple::compare(&t_field3, &t_field3));
        // Different container byte offsets: smaller offset is less significant (LE).
        let mut hi = bf[0].clone();
        hi.byte_offset = 2;
        let t_hi = BitFieldTriple::new(hi, 0);
        assert!(BitFieldTriple::compare(&t_field3, &t_hi));
        assert!(!BitFieldTriple::compare(&t_hi, &t_field3));
    }
}
