//! Minimal `Datatype` seam for the W3 IR data-model.
//!
//! The full port of `decompiler/cpp/type.{hh,cc}` (`Datatype`, `TypeFactory`,
//! the metatype lattice, resolution) is **W6**.  This file carries only the
//! pieces the W3 serial chain (`varnode`, `op`, ...) actually touches, each
//! annotated `// SEAM(W6)`; W6 replaces this skeleton with the real
//! `Datatype`/`TypeFactory` and routes the W3 callers onto it.
//!
//! `type_metatype` is transcribed **verbatim** from `type.hh:80-100` — it is
//! small and stable, and `Varnode::isBooleanValue`/`updateType` compare against
//! `TYPE_BOOL`/`TYPE_UNKNOWN`, so the discriminant *values* are load-bearing.

use kuna_base::types::{int4, uint8};

/// Meta-type of a [`Datatype`], disregarding size.
///
/// Verbatim transcription of the C++ `enum type_metatype` (`type.hh:80-100`),
/// including the explicit discriminant values (ordering is significant: the
/// lower the number, the more specific the data-type, affecting propagation).
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

/// The base datatype handle for the decompiler.
///
/// SEAM(W6): a placeholder carrying only the members/methods `varnode.cc`
/// reaches for — `size` (`getSize`) and `metatype` (`getMetatype`).  W6
/// replaces this with the full `decompiler/cpp/type.{hh,cc}` `Datatype` (the
/// id/flags/name/submeta lattice, `needsResolution`/`findResolve` resolution,
/// the `TypeFactory`).  Until then, `varnode.rs` holds an `Rc<Datatype>` exactly
/// where the C++ holds a `Datatype *`.
#[derive(Debug, Clone)]
pub struct Datatype {
    /// A unique id for the type (or 0 if an id is not assigned)  // SEAM(W6)
    pub id: uint8,
    /// Size (of variable holding a value of this type)  // SEAM(W6)
    pub size: int4,
    /// Meta-type - type disregarding size  // SEAM(W6)
    pub metatype: type_metatype,
}

impl Datatype {
    /// Construct a minimal datatype skeleton (SEAM(W6)).
    pub fn new(size: int4, metatype: type_metatype) -> Datatype {
        Datatype { id: 0, size, metatype }
    }

    /// Get the size of the data-type (C++ `Datatype::getSize`).  // SEAM(W6)
    pub fn get_size(&self) -> int4 {
        self.size
    }

    /// Get the meta-type (C++ `Datatype::getMetatype`).  // SEAM(W6)
    pub fn get_metatype(&self) -> type_metatype {
        self.metatype
    }
}
