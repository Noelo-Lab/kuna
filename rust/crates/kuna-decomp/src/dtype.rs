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

use std::rc::Rc;

use kuna_base::address::Address;
use kuna_base::error::{KunaError, KunaResult};
use kuna_base::space::AddrSpace;
use kuna_base::types::{int4, int8, uint4, uint8};

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
}

/// A field within a structure that is not aligned/sized on byte boundaries
/// (C++ `TypeBitField`, type.hh:328-342).
///
/// SEAM(W6): the `BitRange bits` member needs `BitRange` (an address.hh type W6
/// surfaces for the type subsystem); the byte-range projection
/// (`byteOffset`/`byteSize`) the comparator needs is carried directly.
#[derive(Debug, Clone)]
pub struct TypeBitField {
    /// Name of bitfield
    pub name: String,
    /// Underlying (integer) data-type
    pub field_type: Rc<Datatype>,
    /// Byte offset of the bitfield's container (C++ `bits.byteOffset`).  // SEAM(W6)
    pub byte_offset: int4,
    /// Byte size of the bitfield's container (C++ `bits.byteSize`).  // SEAM(W6)
    pub byte_size: int4,
    /// Identifier of this within containing structure
    pub ident: int4,
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
            DatatypeKind::PointerRel { .. } => {
                // C++ TypePointerRel::getPtrInto — relative offset math.  // SEAM(W6)
                Err(KunaError::lowlevel(
                    "SEAM(W6): TypePointerRel::getPtrInto not yet ported",
                ))
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
                let res = self.compare_base(op);
                if res != 0 {
                    return Ok(res);
                }
                // Both must be pointers (the matching submeta guarantees op is a
                // TypePointer, not a TypePointerRel: their submetas are disjoint).
                let (op_ptrto, op_spaceid, op_wordsize) = op
                    .as_plain_pointer()
                    .ok_or_else(|| Datatype::pointer_invariant_err("compare"))?;
                if *wordsize != op_wordsize {
                    // C++ unsigned `wordsize` (uint4) comparison.
                    return Ok(if *wordsize < op_wordsize { -1 } else { 1 });
                }
                if let Some(r) = Datatype::compare_pointer_space(spaceid.as_ref(), op_spaceid) {
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
            // Subclass overrides handled by type-2/type-3:
            // TypeStruct/TypeUnion/TypeEnum/TypeCode/TypePartial*/
            // TypePointerRel/TypeSpacebase::compare.
            // (TypeSpacebase::compare delegates to compareDependency, which
            // tie-breaks on spaceid then localframe after the base step —
            // type.cc:3498-3514.)  // SEAM(W6)
            _ => Err(KunaError::lowlevel(
                "SEAM(W6): Datatype::compare subclass override not yet ported",
            )),
        }
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
            // type-2/type-3 overrides.
            // (TypeSpacebase tie-breaks on spaceid then localframe after the base
            // step — type.cc:3504-3514.)  // SEAM(W6)
            _ => Err(KunaError::lowlevel(
                "SEAM(W6): Datatype::compareDependency subclass override not yet ported",
            )),
        }
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
            _ => Err(KunaError::lowlevel(
                "SEAM(W6): Datatype::getSubType override not yet ported",
            )),
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
            DatatypeKind::Struct { .. } | DatatypeKind::PartialStruct { .. } => {
                Err(KunaError::lowlevel(
                    "SEAM(W6): Datatype::getHoleSize override not yet ported",
                ))
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
        let _ = max;
        match &self.kind {
            // TypeArray::nearestArrayedComponentForward (type.cc:1395-1402).
            DatatypeKind::Array { arrayof, .. } => {
                if off > 0 {
                    return Ok((-1, off, 0)); // Skip if we are in the middle of array
                }
                let new_off = off;
                let el_size = arrayof.get_align_size() as int8;
                Ok((-off, new_off, el_size))
            }
            DatatypeKind::Struct { .. } | DatatypeKind::Spacebase { .. } => {
                Err(KunaError::lowlevel(
                    "SEAM(W6): Datatype::nearestArrayedComponentForward override not yet ported",
                ))
            }
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
        let _ = max;
        match &self.kind {
            // TypeArray::nearestArrayedComponentBackward (type.cc:1404-1413).
            DatatypeKind::Array { arrayof, .. } => {
                if off < 0 {
                    return Ok((-1, off, 0)); // Skip if we are before array
                }
                let new_off = off;
                let el_size = arrayof.get_align_size() as int8;
                let size = self.size as int8;
                let dist = if off <= size { size - off } else { off - size };
                Ok((dist, new_off, el_size))
            }
            DatatypeKind::Struct { .. } | DatatypeKind::Spacebase { .. } => {
                Err(KunaError::lowlevel(
                    "SEAM(W6): Datatype::nearestArrayedComponentBackward override not yet ported",
                ))
            }
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
            // TypePointer::isPtrsubMatching (type.cc:1260-1312).  `extra` and
            // `multiplier` are mutated locally in the C++ (by-value params), so we
            // shadow them with `let mut` to match.
            DatatypeKind::Pointer { ptrto, wordsize, .. } => {
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
            // TypePointerRel::isPtrsubMatching is type-3.  // SEAM(W6)
            DatatypeKind::PointerRel { .. } => Err(KunaError::lowlevel(
                "SEAM(W6): TypePointerRel::isPtrsubMatching not yet ported",
            )),
            _ => Ok(false), // base default
        }
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
            DatatypeKind::Pointer { ptrto, .. } => {
                if ct.get_metatype() == type_metatype::TYPE_PTR {
                    // ((TypePointer *)ct)->ptrto
                    let ct_ptrto = ct
                        .as_plain_pointer()
                        .map(|(p, _, _)| p)
                        .ok_or_else(|| Datatype::pointer_invariant_err("findCompatibleResolve"))?;
                    return ptrto.find_compatible_resolve(ct_ptrto);
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
            DatatypeKind::Union { .. }
            | DatatypeKind::PartialUnion { .. }
            | DatatypeKind::Struct { .. } => Err(KunaError::lowlevel(
                "SEAM(W6): Datatype::findCompatibleResolve override not yet ported",
            )),
            _ => Ok(-1), // base default (type.cc:600-604)
        }
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

    /// The structured-override compare is W6-seamed (errors), while simple kinds
    /// succeed — confirms the seam routing.
    #[test]
    fn compare_struct_is_seamed() {
        let mut s = Datatype::new_with_align(8, -1, type_metatype::TYPE_STRUCT);
        s.kind = DatatypeKind::Struct { field: vec![], bitfield: vec![] };
        let other = s.clone();
        assert!(s.compare(&other, 10).is_err());
    }

    /// `is_ptrsub_matching` base returns false for non-pointers (type.cc:555-559),
    /// and `TypePointer::isPtrsubMatching` (type.cc:1260-1312) returns false for a
    /// pointer to a non-structured (plain) type.  A pointer to a TYPE_STRUCT walks
    /// `ptrto.get_sub_type`, which is a type-2 SEAM, so the error propagates.
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
        // pointer to a struct: the override calls ptrto.get_sub_type (type-2 SEAM).
        let mut st = Datatype::new_with_align(8, 4, type_metatype::TYPE_STRUCT);
        st.kind = DatatypeKind::Struct {
            field: vec![TypeField::new(0, 0, "a", Rc::new(Datatype::new(4, type_metatype::TYPE_INT)))],
            bitfield: vec![],
        };
        let mut p_st = Datatype::new_with_align(8, -1, type_metatype::TYPE_PTR);
        p_st.kind = DatatypeKind::Pointer {
            ptrto: Rc::new(st),
            spaceid: None,
            truncate: None,
            wordsize: 1,
        };
        assert!(p_st.is_ptrsub_matching(0, 4, 1).is_err());
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
}
