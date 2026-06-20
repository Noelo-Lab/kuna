//! Verifier tests for item `w6-s5-type-2`: the structured `Datatype`
//! subclasses ported from `type.cc` lines ~1723-3564
//! (TypeStruct/TypeUnion/TypeEnum/TypeCode/TypeSpacebase + partials + field
//! resolution).
//!
//! These exercise the output-determining surfaces the hunt list flags as most
//! fragile:
//!
//!   * **enum `getMatches` matrices** — the exact-string oracle from
//!     `testtypes.cc` (`enum_matching`/`enum_matching2`), including the
//!     complement path and the `upper_bound`/predecessor walk over the namemap.
//!     (cpp: type.cc:1537-1586)
//!   * **struct field-offset assignment** vs the C++ alignment rules
//!     (`assignFieldOffsets`): each field rounds up to its alignment, the
//!     structure size rounds up to the struct alignment.  (cpp:2365-2409)
//!   * **bitfield contiguous assignment** byte offset/size + LSB placement and
//!     the big-endian reversal.  (cpp:2322-2355)
//!   * **union facet basics** — `set_union_fields`, `assign_union_field_offsets`
//!     (max size / max align, all offsets 0), and `findCompatibleResolve`.
//!     (cpp:2418-2425, 2629-2673)
//!   * **struct compare / compareDependency** field-count finisher and the deep
//!     field recursion.  (cpp:1978-2063)
//!   * **getSubType / getHoleSize** binary-search field lookup and the hole
//!     distance.  (cpp:1894-1921)
//!   * **partial-type compare** offset tie-break + container recursion.
//!     (cpp:2829-2853)
//!
//! C++ oracle: `decompiler/cpp/type.cc`, `decompiler/cpp/address.cc`
//! (`BitRange::compare`), and `decompiler/unittests/testtypes.cc`.

use std::collections::BTreeMap;
use std::rc::Rc;

use kuna_decomp::dtype::{
    flags, type_metatype, Datatype, DatatypeKind, TypeBitField, TypeField,
};

/// Build an interned scalar with explicit alignment (and align_size = aligned
/// size), the way the TypeFactory would.
fn scalar(size: i32, align: i32, m: type_metatype) -> Rc<Datatype> {
    let mut dt = Datatype::new_with_align(size, align, m);
    dt.align_size = Datatype::calc_align_size(size, align);
    Rc::new(dt)
}

/// Build an enum from a (value -> name) map, mirroring `TypeEnum` after decode.
fn make_enum(size: i32, entries: &[(u64, &str)]) -> Datatype {
    let mut dt = Datatype::new_with_align(size, size, type_metatype::TYPE_INT);
    dt.flags |= flags::enumtype;
    let mut namemap: BTreeMap<u64, String> = BTreeMap::new();
    for (v, n) in entries {
        namemap.insert(*v, (*n).to_string());
    }
    dt.kind = DatatypeKind::Enum { namemap };
    dt
}

// ---------------------------------------------------------------------------
// Enum getMatches — the testtypes.cc oracle
// ---------------------------------------------------------------------------

/// `testtypes.cc` `enum_matching` (lines 215-239): enum3 { ZERO=0, ONE=1,
/// TWO=2, FOUR=4, EIGHT=8 }.  The default enum size in the test environment
/// (x86-64) is `glb->getDefaultSize()` = 8, so the complement path masks the
/// full 64-bit value (`0xfffffffffffffff7 ^ calc_mask(8) == 8`).
#[test]
fn enum_matching_enum3() {
    let enum3 = make_enum(
        8,
        &[(0, "ZERO"), (1, "ONE"), (2, "TWO"), (4, "FOUR"), (8, "EIGHT")],
    );

    let rep = enum3.get_matches(5).unwrap();
    assert_eq!(rep.match_name, vec!["FOUR".to_string(), "ONE".to_string()]);
    assert!(!rep.complement);

    let rep = enum3.get_matches(0xfffffffffffffff7).unwrap();
    assert_eq!(rep.match_name, vec!["EIGHT".to_string()]);
    assert!(rep.complement);

    let rep = enum3.get_matches(0).unwrap();
    assert_eq!(rep.match_name, vec!["ZERO".to_string()]);
    assert!(!rep.complement);

    // 0x10 has no representation in the 4-bit enum within its size mask.
    let rep = enum3.get_matches(0x10).unwrap();
    assert!(rep.match_name.is_empty());
    assert!(!rep.complement);
}

/// `testtypes.cc` `enum_matching2` (lines 241-261): enum4 { ZERO=0, ONE=1,
/// TWO=2, FOUR=4, SIX=6, EIGHT=8, ELEVEN=11 }, default size 8.
#[test]
fn enum_matching_enum4() {
    let enum4 = make_enum(
        8,
        &[
            (0, "ZERO"),
            (1, "ONE"),
            (2, "TWO"),
            (4, "FOUR"),
            (6, "SIX"),
            (8, "EIGHT"),
            (11, "ELEVEN"),
        ],
    );

    let rep = enum4.get_matches(12).unwrap();
    assert_eq!(rep.match_name, vec!["EIGHT".to_string(), "FOUR".to_string()]);
    assert!(!rep.complement);

    let rep = enum4.get_matches(7).unwrap();
    assert_eq!(rep.match_name, vec!["SIX".to_string(), "ONE".to_string()]);
    assert!(!rep.complement);

    let rep = enum4.get_matches(11).unwrap();
    assert_eq!(rep.match_name, vec!["ELEVEN".to_string()]);
    assert!(!rep.complement);
}

/// `hasNamedValue` checks the namemap directly.
#[test]
fn enum_has_named_value() {
    let enum3 = make_enum(4, &[(0, "ZERO"), (1, "ONE"), (8, "EIGHT")]);
    assert!(enum3.has_named_value(0));
    assert!(enum3.has_named_value(8));
    assert!(!enum3.has_named_value(3));
}

/// `assignValues` fills unassigned names with the next free value and rejects
/// duplicates (type.cc:1688-1721).
#[test]
fn enum_assign_values() {
    let names = vec!["A".to_string(), "B".to_string(), "C".to_string()];
    // A=5 (assigned), B unassigned -> next free after maxval(5) = 6,
    // C unassigned -> 7.
    let vals = vec![5u64, 0, 0];
    let assigned = vec![true, false, false];
    let nmap = Datatype::assign_values(4, "E", &names, &vals, &assigned).unwrap();
    assert_eq!(nmap.get(&5), Some(&"A".to_string()));
    assert_eq!(nmap.get(&6), Some(&"B".to_string()));
    assert_eq!(nmap.get(&7), Some(&"C".to_string()));

    // Duplicate explicit value -> error.
    let names = vec!["A".to_string(), "B".to_string()];
    let vals = vec![1u64, 1u64];
    let assigned = vec![true, true];
    assert!(Datatype::assign_values(4, "E", &names, &vals, &assigned).is_err());
}

// ---------------------------------------------------------------------------
// Struct field-offset assignment (alignment)
// ---------------------------------------------------------------------------

/// `assignFieldOffsets`: { char a; int b; char c } with align(char)=1,
/// align(int)=4.  a@0 (size1), b rounds to 4 (@4, size4), c@8 (size1).
/// Structure size rounds up to align(4): offset 9 -> 12.
#[test]
fn struct_field_offsets_alignment() {
    let char_t = scalar(1, 1, type_metatype::TYPE_INT);
    let int_t = scalar(4, 4, type_metatype::TYPE_INT);
    let mut list = vec![
        TypeField::new(-1, -1, "a", Rc::clone(&char_t)),
        TypeField::new(-1, -1, "b", Rc::clone(&int_t)),
        TypeField::new(-1, -1, "c", Rc::clone(&char_t)),
    ];
    let mut bitlist: Vec<TypeBitField> = vec![];
    let (new_size, new_align, fl) =
        Datatype::assign_field_offsets(&mut list, &mut bitlist).unwrap();
    assert_eq!(list[0].offset, 0);
    assert_eq!(list[1].offset, 4); // aligned up from 1 to 4
    assert_eq!(list[2].offset, 8);
    assert_eq!(new_align, 4);
    assert_eq!(new_size, 12); // 9 rounded up to multiple of 4
    assert_eq!(fl & flags::has_bitfields, 0);
    // ident mirrors offset.
    assert_eq!(list[1].ident, 4);
}

/// A field whose offset is already set (!= -1) is left in place (type.cc:2387).
#[test]
fn struct_field_offsets_preset_skipped() {
    let int_t = scalar(4, 4, type_metatype::TYPE_INT);
    let mut list = vec![
        TypeField::new(100, 100, "fixed", Rc::clone(&int_t)),
        TypeField::new(-1, -1, "auto", Rc::clone(&int_t)),
    ];
    let mut bitlist: Vec<TypeBitField> = vec![];
    let (_size, _align, _fl) =
        Datatype::assign_field_offsets(&mut list, &mut bitlist).unwrap();
    assert_eq!(list[0].offset, 100); // untouched
    assert_eq!(list[1].offset, 0); // first auto-placed field at 0
}

/// A void field is rejected (type.cc:2385-2386).
#[test]
fn struct_field_offsets_void_rejected() {
    let void_t = Rc::new(Datatype::new_with_align(0, 1, type_metatype::TYPE_VOID));
    let mut list = vec![TypeField::new(-1, -1, "v", void_t)];
    let mut bitlist: Vec<TypeBitField> = vec![];
    assert!(Datatype::assign_field_offsets(&mut list, &mut bitlist).is_err());
}

/// `set_struct_fields`: a single field filling the whole struct flips
/// `needs_resolution` (type.cc:1743-1746).
#[test]
fn struct_single_field_needs_resolution() {
    let int8 = scalar(8, 8, type_metatype::TYPE_INT);
    let mut s = Datatype::new_with_align(8, 8, type_metatype::TYPE_STRUCT);
    s.set_struct_fields(
        vec![TypeField::new(0, 0, "only", Rc::clone(&int8))],
        vec![],
        8,
        8,
    );
    assert!(s.needs_resolution());
    assert_eq!(s.get_align_size(), 8);

    // Two fields -> no needs_resolution.
    let int4 = scalar(4, 4, type_metatype::TYPE_INT);
    let mut s2 = Datatype::new_with_align(8, 4, type_metatype::TYPE_STRUCT);
    s2.set_struct_fields(
        vec![
            TypeField::new(0, 0, "a", Rc::clone(&int4)),
            TypeField::new(4, 4, "b", Rc::clone(&int4)),
        ],
        vec![],
        8,
        4,
    );
    assert!(!s2.needs_resolution());
}

// ---------------------------------------------------------------------------
// Bitfield contiguous assignment
// ---------------------------------------------------------------------------

/// `assignContiguousBitfields`: two little-endian bitfields sharing ident 0
/// (3 bits + 5 bits = 8 bits = 1 byte) packed at offset 0, with LSB 0 and 3.
#[test]
fn bitfield_contiguous_assignment_le() {
    let int_t = scalar(4, 4, type_metatype::TYPE_INT);
    // After assignFieldOffsets, the bitfields' ident is their declaration group.
    let mut list: Vec<TypeField> = vec![];
    let mut bitlist = vec![
        TypeBitField::new(0, 3, false, "lo", Rc::clone(&int_t)),
        TypeBitField::new(0, 5, false, "hi", Rc::clone(&int_t)),
    ];
    let (new_size, new_align, fl) =
        Datatype::assign_field_offsets(&mut list, &mut bitlist).unwrap();
    // byteSize is (3+5+7)/8 = 1; both at byteOffset 0.
    assert_eq!(bitlist[0].byte_offset, 0);
    assert_eq!(bitlist[0].byte_size, 1);
    assert_eq!(bitlist[0].least_sig_bit, 0);
    assert_eq!(bitlist[1].least_sig_bit, 3); // packed above lo
    assert_eq!(bitlist[1].byte_size, 1);
    // ident becomes the position within the bitfield list.
    assert_eq!(bitlist[0].ident, 0);
    assert_eq!(bitlist[1].ident, 1);
    assert_eq!(new_align, int_t.get_alignment());
    assert_eq!(fl & flags::has_bitfields, flags::has_bitfields);
    // align(4): one byte rounds up to 4.
    assert_eq!(new_size, 4);
}

/// Big-endian contiguous bitfields reverse order after assignment
/// (type.cc:2350-2354).
#[test]
fn bitfield_contiguous_assignment_be_reversed() {
    let int_t = scalar(4, 4, type_metatype::TYPE_INT);
    let mut list: Vec<TypeField> = vec![];
    let mut bitlist = vec![
        TypeBitField::new(0, 3, true, "first", Rc::clone(&int_t)),
        TypeBitField::new(0, 5, true, "second", Rc::clone(&int_t)),
    ];
    Datatype::assign_field_offsets(&mut list, &mut bitlist).unwrap();
    // After the big-endian reverse, "second" comes first in the list.
    assert_eq!(bitlist[0].name, "second");
    assert_eq!(bitlist[1].name, "first");
}

// ---------------------------------------------------------------------------
// Union facet basics
// ---------------------------------------------------------------------------

/// `assignFieldOffsets` for a union: all offsets 0, size = max field size,
/// align = max field align (type.cc:2651-2673).
#[test]
fn union_assign_field_offsets() {
    let int_t = scalar(4, 4, type_metatype::TYPE_INT);
    let dbl_t = scalar(8, 8, type_metatype::TYPE_FLOAT);
    let mut list = vec![
        TypeField::new(0, 0, "i", Rc::clone(&int_t)),
        TypeField::new(0, 0, "d", Rc::clone(&dbl_t)),
    ];
    let (new_size, new_align) =
        Datatype::assign_union_field_offsets(&mut list, "U").unwrap();
    assert_eq!(new_size, 8); // max(4, 8)
    assert_eq!(new_align, 8); // max(4, 8)
    assert!(list.iter().all(|f| f.offset == 0));
}

/// A union field with an empty name or a void type is rejected.
#[test]
fn union_assign_field_offsets_validation() {
    let int_t = scalar(4, 4, type_metatype::TYPE_INT);
    let mut list = vec![TypeField::new(0, 0, "", Rc::clone(&int_t))];
    assert!(Datatype::assign_union_field_offsets(&mut list, "U").is_err());

    let void_t = Rc::new(Datatype::new_with_align(0, 1, type_metatype::TYPE_VOID));
    let mut list2 = vec![TypeField::new(0, 0, "v", void_t)];
    assert!(Datatype::assign_union_field_offsets(&mut list2, "U").is_err());
}

/// `set_union_fields` installs the payload and recomputes alignSize.
#[test]
fn union_set_fields() {
    let int_t = scalar(4, 4, type_metatype::TYPE_INT);
    let mut u = Datatype::new_with_align(0, -1, type_metatype::TYPE_UNION);
    u.set_union_fields(
        vec![TypeField::new(0, 0, "i", Rc::clone(&int_t))],
        4,
        4,
    );
    assert_eq!(u.get_size(), 4);
    assert_eq!(u.get_align_size(), 4);
    assert_eq!(u.get_field(0).unwrap().name, "i");
    assert!(u.get_field(1).is_none());
}

/// `TypeUnion::findCompatibleResolve` (type.cc:2629-2649): a non-resolution
/// `ct` matches the field that is == ct at offset 0.
#[test]
fn union_find_compatible_resolve_direct() {
    let int_t = scalar(4, 4, type_metatype::TYPE_INT);
    let float_t = scalar(4, 4, type_metatype::TYPE_FLOAT);
    let mut u = Datatype::new_with_align(4, 4, type_metatype::TYPE_UNION);
    u.kind = DatatypeKind::Union {
        field: vec![
            TypeField::new(0, 0, "i", Rc::clone(&int_t)),
            TypeField::new(1, 0, "f", Rc::clone(&float_t)),
        ],
    };
    // ct == field[1].type (pointer identity) at offset 0 -> returns index 1.
    assert_eq!(u.find_compatible_resolve(&float_t).unwrap(), 1);
    // A type not present -> -1.
    let other = scalar(4, 4, type_metatype::TYPE_INT);
    assert_eq!(u.find_compatible_resolve(&other).unwrap(), -1);
}

// ---------------------------------------------------------------------------
// Struct compare / compareDependency
// ---------------------------------------------------------------------------

fn make_struct(size: i32, align: i32, fields: Vec<TypeField>) -> Datatype {
    let mut s = Datatype::new_with_align(size, align, type_metatype::TYPE_STRUCT);
    s.align_size = Datatype::calc_align_size(size, align);
    s.kind = DatatypeKind::Struct { field: fields, bitfield: vec![] };
    s
}

/// Two structurally identical structs with shared field types compare equal;
/// differing field types order by the deep comparison.
#[test]
fn struct_compare_equal_and_deep() {
    let int_t = scalar(4, 4, type_metatype::TYPE_INT);
    let uint_t = scalar(4, 4, type_metatype::TYPE_UINT);

    let a = make_struct(
        8,
        4,
        vec![
            TypeField::new(0, 0, "x", Rc::clone(&int_t)),
            TypeField::new(4, 4, "y", Rc::clone(&int_t)),
        ],
    );
    let b = make_struct(
        8,
        4,
        vec![
            TypeField::new(0, 0, "x", Rc::clone(&int_t)),
            TypeField::new(4, 4, "y", Rc::clone(&int_t)),
        ],
    );
    assert_eq!(a.compare(&b, 10).unwrap(), 0);
    assert_eq!(a.compare_dependency(&b).unwrap(), 0);

    // Same names/offsets but the second field's type differs at first-level
    // metatype (int vs uint) -> TypeField::compare orders it.  uint submeta <
    // int submeta, so the uint-bearing struct sorts first.
    let c = make_struct(
        8,
        4,
        vec![
            TypeField::new(0, 0, "x", Rc::clone(&int_t)),
            TypeField::new(4, 4, "y", Rc::clone(&uint_t)),
        ],
    );
    // c (uint) vs a (int): TypeField::compare hits metatype, TYPE_UINT(13) <
    // TYPE_INT(14) -> -1.
    assert_eq!(c.compare(&a, 10).unwrap(), -1);
    assert_eq!(a.compare(&c, 10).unwrap(), 1);
}

/// Differing field counts use the `(op.field.size() - field.size())` finisher.
#[test]
fn struct_compare_field_count() {
    let int_t = scalar(4, 4, type_metatype::TYPE_INT);
    let one = make_struct(4, 4, vec![TypeField::new(0, 0, "a", Rc::clone(&int_t))]);
    let two = make_struct(
        8,
        4,
        vec![
            TypeField::new(0, 0, "a", Rc::clone(&int_t)),
            TypeField::new(4, 4, "b", Rc::clone(&int_t)),
        ],
    );
    // Different size -> base compare resolves first (op.size - size). For
    // compareDependency, identical-size structs with differing field counts
    // would use the field finisher; here sizes differ so base wins.
    // one(size4) vs two(size8): base `op.size - size` = 8 - 4 = 4.
    assert_eq!(one.compare(&two, 10).unwrap(), 4);
}

// ---------------------------------------------------------------------------
// getSubType / getHoleSize
// ---------------------------------------------------------------------------

/// `TypeStruct::getSubType` binary-searches the containing field and returns the
/// field-relative offset (type.cc:1894-1904).
#[test]
fn struct_get_sub_type() {
    let int_t = scalar(4, 4, type_metatype::TYPE_INT);
    let s = make_struct(
        8,
        4,
        vec![
            TypeField::new(0, 0, "a", Rc::clone(&int_t)),
            TypeField::new(4, 4, "b", Rc::clone(&int_t)),
        ],
    );
    // off 5 lands in field b (offset 4), newoff 1.
    let (sub, newoff) = s.get_sub_type(5).unwrap();
    assert!(sub.is_some());
    assert_eq!(newoff, 1);
    // off 0 lands in field a, newoff 0.
    let (sub0, newoff0) = s.get_sub_type(0).unwrap();
    assert!(sub0.is_some());
    assert_eq!(newoff0, 0);
}

/// `TypeStruct::getHoleSize`: distance to the next field when off is in a hole
/// (type.cc:1906-1921).
#[test]
fn struct_get_hole_size() {
    let int_t = scalar(4, 4, type_metatype::TYPE_INT);
    let char_t = scalar(1, 1, type_metatype::TYPE_INT);
    // { char a@0; int b@4 } size 8 -- a hole exists at offsets 1..4.
    let s = make_struct(
        8,
        4,
        vec![
            TypeField::new(0, 0, "a", Rc::clone(&char_t)),
            TypeField::new(4, 4, "b", Rc::clone(&int_t)),
        ],
    );
    // off 1: lower-bound field is a@0; newOff 1 >= a.size(1), so advance to b@4.
    // Distance to following field = 4 - 1 = 3.
    assert_eq!(s.get_hole_size(1).unwrap(), 3);
    // off 0: inside field a (newOff 0 < 1) -> a.getHoleSize(0) = 0 (no hole in
    // a primitive).
    assert_eq!(s.get_hole_size(0).unwrap(), 0);
}

// ---------------------------------------------------------------------------
// Partial-type compare
// ---------------------------------------------------------------------------

/// `TypePartialStruct::compare` (type.cc:2829-2843): base compare, then offset
/// tie-break, then container recursion.
#[test]
fn partial_struct_compare() {
    let int_t = scalar(4, 4, type_metatype::TYPE_INT);
    let container = Rc::new(make_struct(
        8,
        4,
        vec![
            TypeField::new(0, 0, "a", Rc::clone(&int_t)),
            TypeField::new(4, 4, "b", Rc::clone(&int_t)),
        ],
    ));
    let stripped = scalar(4, 4, type_metatype::TYPE_UNKNOWN);
    let mk = |off: i32| {
        let mut p = Datatype::new_with_align(4, 1, type_metatype::TYPE_PARTIALSTRUCT);
        p.align_size = 4;
        p.kind = DatatypeKind::PartialStruct {
            stripped: Rc::clone(&stripped),
            container: Rc::clone(&container),
            offset: off,
        };
        p
    };
    let p0 = mk(0);
    let p0b = mk(0);
    let p4 = mk(4);
    // Same offset, same container -> equal.
    assert_eq!(p0.compare(&p0b, 10).unwrap(), 0);
    // Different offset: 0 < 4 -> -1.
    assert_eq!(p0.compare(&p4, 10).unwrap(), -1);
    assert_eq!(p4.compare(&p0, 10).unwrap(), 1);
}

/// `TypePartialEnum::hasNamedValue` shifts the value into the parent enum's
/// frame (type.cc:2700-2705).
#[test]
fn partial_enum_has_named_value() {
    // parent enum at the high byte: value 8 named EIGHT.
    let parent = Rc::new(make_enum(4, &[(0x800, "HI")]));
    let stripped = scalar(1, 1, type_metatype::TYPE_UNKNOWN);
    let mut pe = Datatype::new_with_align(1, 1, type_metatype::TYPE_PARTIALENUM);
    pe.flags |= flags::enumtype;
    pe.kind = DatatypeKind::PartialEnum {
        stripped,
        parent: Rc::clone(&parent),
        offset: 1, // one byte up
    };
    // val 8 << (8*1) = 0x800 -> present in parent.
    assert!(pe.has_named_value(8));
    assert!(!pe.has_named_value(9));
}

// ---------------------------------------------------------------------------
// Enum compareDependency
// ---------------------------------------------------------------------------

/// `TypeEnum::compareDependency` (type.cc:1594-1617): equal namemaps tie, a
/// larger namemap sorts after a smaller one.
#[test]
fn enum_compare_dependency() {
    let e1 = make_enum(4, &[(0, "ZERO"), (1, "ONE")]);
    let e2 = make_enum(4, &[(0, "ZERO"), (1, "ONE")]);
    assert_eq!(e1.compare_dependency(&e2).unwrap(), 0);
    // compare() delegates to compareDependency for enums.
    assert_eq!(e1.compare(&e2, 10).unwrap(), 0);

    let e3 = make_enum(4, &[(0, "ZERO"), (1, "ONE"), (2, "TWO")]);
    // e1 (2 entries) vs e3 (3 entries): namemap size differs, 2 < 3 -> -1.
    assert_eq!(e1.compare_dependency(&e3).unwrap(), -1);
    assert_eq!(e3.compare_dependency(&e1).unwrap(), 1);

    // Same size namemaps, differing name for same value: order by name.
    let e4 = make_enum(4, &[(0, "ZERO"), (1, "UNO")]);
    // "ONE" < "UNO" -> -1.
    assert_eq!(e1.compare_dependency(&e4).unwrap(), -1);
}
