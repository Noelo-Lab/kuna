//! Adversarial verifier tests for `w10-bitfield` (the bitfield type-query layer
//! and the `BitFieldTransform` base). Targets the spots the hunt list flagged as
//! most fragile and that the porter's own tests under-exercise.
//!
//! - `BitFieldTriple::compare` on the **big-endian** arm (the porter's diff only
//!   tested little-endian thoroughly) — proving the comparator is a total
//!   strict-weak order that reverses container-byte significance for BE
//!   (type.cc:932-949).
//! - `collect_bit_fields` / `has_bit_fields_in_range` `upper_bound` boundary
//!   behavior: the `compareMaxByte` skip must drop fields whose container ends at
//!   or before `offset`, and the field-range half-open `[offset, offset+sz)`
//!   overlap (type.cc:1804-1852).
//! - `BitFieldTransform::establish_fields` on a **big-endian** container and on
//!   partial-coverage clipping at the Varnode boundary (bitfield.cc:57-91).
//!
//! These are verifier-authored (independent of the porter) and land on the
//! porter branch per docs/rust-port/verification.md.

use std::rc::Rc;

use kuna_decomp::bitfield::{BitFieldTransform, FieldRef};
use kuna_decomp::dtype::{
    flags, type_metatype, BitFieldTriple, Datatype, DatatypeKind, TypeBitField, TypeField,
};
use kuna_decomp::seams::VarnodeId;

/// Build one bitfield with an explicit container layout.
fn bf(id: i32, byte_off: i32, byte_sz: i32, lsb: i32, nb: i32, big: bool, meta: type_metatype) -> TypeBitField {
    let mut b = TypeBitField::new(id, nb, big, format!("f{id}"), Rc::new(Datatype::new(4, meta)));
    b.byte_offset = byte_off;
    b.byte_size = byte_sz;
    b.least_sig_bit = lsb;
    b
}

fn struct_with(field: Vec<TypeField>, bitfield: Vec<TypeBitField>, size: i32) -> Datatype {
    let mut s = Datatype::new_with_align(size, 4, type_metatype::TYPE_STRUCT);
    if !bitfield.is_empty() {
        s.flags |= flags::has_bitfields;
    }
    s.kind = DatatypeKind::Struct { field, bitfield };
    s
}

fn vn() -> VarnodeId {
    VarnodeId::default()
}

// ---------------------------------------------------------------------------
// F-1: BitFieldTriple::compare big-endian arm + total-order self-consistency.
//
// C++ (type.cc:937-942): for differing container byte offsets, big-endian
// returns `byteOff1 > byteOff2` (bigger byte offset is LESS significant), the
// REVERSE of little-endian.  The porter's diff only covers LE; a careless port
// would drop the `is_big_endian` branch and silently mis-sort BE bitfields.
// ---------------------------------------------------------------------------
#[test]
fn w10bf_triple_compare_bigendian_reverses_byte_significance() {
    // Two bitfields in different container bytes, big-endian.
    let lo_byte = bf(0, 0, 1, 0, 3, true, type_metatype::TYPE_UINT); // byteOffset 0
    let hi_byte = bf(1, 2, 1, 0, 3, true, type_metatype::TYPE_UINT); // byteOffset 2

    let t_lo = BitFieldTriple::new(lo_byte.clone(), 0);
    let t_hi = BitFieldTriple::new(hi_byte.clone(), 0);

    // BE: bigger byte offset (2) is LESS significant -> sorts FIRST.
    // compare(op1,op2)==true means op1 strictly before op2.
    assert!(
        BitFieldTriple::compare(&t_hi, &t_lo),
        "BE: higher byte-offset container must sort as less significant (first)"
    );
    assert!(
        !BitFieldTriple::compare(&t_lo, &t_hi),
        "BE comparator must be asymmetric (not both directions true)"
    );

    // The endianness key is op1's bitfield (C++ line 935): both records carry
    // big-endian here, and the order is the BE order, opposite of LE.
    // Sanity: the SAME two byte-offsets little-endian sort the other way.
    let lo_le = bf(0, 0, 1, 0, 3, false, type_metatype::TYPE_UINT);
    let hi_le = bf(1, 2, 1, 0, 3, false, type_metatype::TYPE_UINT);
    let t_lo_le = BitFieldTriple::new(lo_le, 0);
    let t_hi_le = BitFieldTriple::new(hi_le, 0);
    assert!(
        BitFieldTriple::compare(&t_lo_le, &t_hi_le),
        "LE: lower byte-offset must sort first (opposite of BE)"
    );

    // Total-order: irreflexive on every record we built.
    for t in [&t_lo, &t_hi, &t_lo_le, &t_hi_le] {
        assert!(!BitFieldTriple::compare(t, t), "comparator must be irreflexive");
    }
    // The `offset` term folds into the container byte offset (C++ byteOff =
    // op.offset + op.bitfield.byteOffset): a record at offset+2,byteOffset 0 must
    // compare equal-keyed to one at offset 0,byteOffset 2 under BE -> same total
    // significance, then leastSigBit tie-break (here both lsb 0 -> not-less).
    let t_off = BitFieldTriple::new(bf(2, 0, 1, 0, 3, true, type_metatype::TYPE_UINT), 2);
    assert!(!BitFieldTriple::compare(&t_off, &t_hi), "equal folded byteOff + equal lsb -> not strictly less");
    assert!(!BitFieldTriple::compare(&t_hi, &t_off), "equal folded byteOff + equal lsb -> not strictly less");
}

// ---------------------------------------------------------------------------
// F-2: upper_bound(compareMaxByte) boundary in collect_bit_fields.
//
// C++ TypeBitField::compareMaxByte(off, b) = off < b.byteOffset + b.byteSize.
// upper_bound returns the first element whose container END is strictly past
// `off`.  A field whose container ends exactly AT `off` (byteOffset+byteSize ==
// off) must be skipped; a field ending one byte later must be kept.  An
// off-by-one here (>= vs >) silently includes/excludes the boundary field.
// ---------------------------------------------------------------------------
#[test]
fn w10bf_collect_upper_bound_skips_container_ending_at_offset() {
    // Two single-byte containers: byte 0 (ends at byte 1) and byte 1 (ends at
    // byte 2).  Each holds one 8-bit field.
    let b0 = bf(0, 0, 1, 0, 8, false, type_metatype::TYPE_UINT);
    let b1 = bf(1, 1, 1, 0, 8, false, type_metatype::TYPE_UINT);
    let s = struct_with(vec![], vec![b0, b1], 2);

    // Query byte range [1,2): compareMaxByte(1, b0) = 1 < (0+1) = false -> b0's
    // container ends AT 1, so upper_bound steps past it; b1 (ends at 2, 1<2) is
    // the start.  Only b1 overlaps [1,2).
    let mut res = Vec::new();
    s.collect_bit_fields(0, &mut res, 1, 1);
    assert_eq!(res.len(), 1, "only the byte-1 field overlaps [1,2)");
    assert_eq!(res[0].bitfield.byte_offset, 1);

    // Query [0,2): both containers overlap.
    let mut res2 = Vec::new();
    s.collect_bit_fields(0, &mut res2, 0, 2);
    assert_eq!(res2.len(), 2, "both fields overlap [0,2)");

    // hasBitFieldsInRange must agree at every boundary with collect.
    assert!(s.has_bit_fields_in_range(0, 1));
    assert!(s.has_bit_fields_in_range(1, 1));
    assert!(!s.has_bit_fields_in_range(2, 1), "byte 2 is past both containers");
    // Zero-size query at the very end: no overlap (half-open range is empty).
    assert!(!s.has_bit_fields_in_range(2, 0));
}

// ---------------------------------------------------------------------------
// F-3: collect_bit_fields recursion into nested struct shifts offsets by the
// field offset and reports the immediate-container offset as
// base_offset + field.offset (C++ type.cc:1822 recursion args), AND the
// field-loop `curField.offset >= offset+sz` break is half-open.
// ---------------------------------------------------------------------------
#[test]
fn w10bf_collect_nested_offset_shift_and_halfopen_break() {
    // inner struct (1 byte) with one 8-bit bitfield at container byte 0.
    let inner = Rc::new(struct_with(
        vec![],
        vec![bf(0, 0, 1, 0, 8, false, type_metatype::TYPE_UINT)],
        1,
    ));
    // outer: plain field `pad` at 0 (1 byte, not a struct -> skipped), nested
    // struct `inner` at offset 3.
    let pad = TypeField::new(0, 0, "pad", Rc::new(Datatype::new(1, type_metatype::TYPE_UINT)));
    let inner_f = TypeField::new(1, 3, "inner", Rc::clone(&inner));
    let outer = struct_with(vec![pad, inner_f], vec![], 8);

    // Query [3,4): only the nested struct at offset 3 is in range; recursion
    // reports container offset base(0)+3 = 3.
    let mut res = Vec::new();
    outer.collect_bit_fields(0, &mut res, 3, 1);
    assert_eq!(res.len(), 1, "the nested bitfield at offset 3 is collected");
    assert_eq!(res[0].offset, 3, "immediate-container offset is base+field.offset");

    // Query [0,3): the nested struct begins AT offset 3, and the field-loop
    // break is `curField.offset >= offset+sz` (3 >= 0+3) -> the nested struct is
    // excluded.  Half-open: byte 3 is NOT in [0,3).
    let mut res2 = Vec::new();
    outer.collect_bit_fields(0, &mut res2, 0, 3);
    assert!(res2.is_empty(), "nested struct starting at offset 3 is outside [0,3)");
    assert!(!outer.has_bit_fields_in_range(0, 3));
    assert!(outer.has_bit_fields_in_range(0, 4), "[0,4) reaches the nested struct at 3");
}

// ---------------------------------------------------------------------------
// F-4: establish_fields big-endian translate_lsb + partial-overlap clipping.
//
// On a big-endian 1-byte container holding field3:3(lsb0)+sfield4:4(lsb3)+
// fieldb:1(lsb7), establishFields must (a) sort via translateLSB into the BE
// frame and (b) emit field records iff overlapTest is 0/3.  A 1-byte Varnode
// fully covers all three; with follow_holes=false exactly three field records
// appear, in least-to-most-significant order.  The most-significant field
// (fieldb) being a bool (not TYPE_INT) -> not sign-extended even at the top.
// ---------------------------------------------------------------------------
#[test]
fn w10bf_establish_fields_bigendian_three_fields_no_holes() {
    let bitfield = vec![
        bf(0, 0, 1, 0, 3, true, type_metatype::TYPE_UINT), // field3
        bf(1, 0, 1, 3, 4, true, type_metatype::TYPE_INT),  // sfield4 (signed)
        bf(2, 0, 1, 7, 1, true, type_metatype::TYPE_BOOL), // fieldb
    ];
    let dt = Rc::new(struct_with(vec![], bitfield, 1));

    let mut t = BitFieldTransform::new(&dt, 0, /*big_endian*/ true);
    assert!(t.parent_struct.is_some(), "TYPE_STRUCT root sets parentStruct");
    assert_eq!(t.initial_offset, 0);
    assert!(t.is_big_endian);

    t.establish_fields(vn(), /*vn_size*/ 1, /*follow_holes*/ false);
    assert_eq!(t.work_list.len(), 3, "all three fields fully contained, no holes");
    assert!(
        t.work_list.iter().all(|s| matches!(s.field, FieldRef::Field { .. })),
        "no hole records with follow_holes=false"
    );

    // sfield4 is the only TYPE_INT; it is NOT the most-significant bit of the
    // byte (fieldb at lsb7 is), so it is not sign-extended.
    let sfield = t
        .work_list
        .iter()
        .find(|s| matches!(s.field, FieldRef::Field { is_int: true }))
        .expect("sfield4 is the one TYPE_INT field");
    assert!(
        !sfield.is_sign_extended,
        "signed field not at the container top must not be sign-extended"
    );

    // fieldb (bool, lsb7) is at the top but is not TYPE_INT -> not sign-extended.
    let topmost = t
        .work_list
        .iter()
        .max_by_key(|s| s.bits_field.least_sig_bit)
        .unwrap();
    assert!(!topmost.is_sign_extended, "bool top field is never sign-extended");
}

// ---------------------------------------------------------------------------
// F-5: establish_fields partial coverage emits a leading hole (BE/LE), and the
// follow_holes flag gates EVERY hole branch.  A struct whose single field sits
// in the MIDDLE of the Varnode leaves a hole before it and after it.
// ---------------------------------------------------------------------------
#[test]
fn w10bf_establish_fields_leading_and_trailing_holes_gated() {
    // One 2-bit field at lsb3..5 of a single byte; a 1-byte (8-bit) Varnode.
    // Holes: [0,3) before, [5,8) after.
    let dt = Rc::new(struct_with(
        vec![],
        vec![bf(0, 0, 1, 3, 2, false, type_metatype::TYPE_UINT)],
        1,
    ));

    let mut t = BitFieldTransform::new(&dt, 0, false);
    t.establish_fields(vn(), 1, /*follow_holes*/ true);
    // hole[0,3), field[3,5), hole[5,8)
    assert_eq!(t.work_list.len(), 3, "leading hole + field + trailing hole");
    assert!(matches!(t.work_list[0].field, FieldRef::Hole));
    assert_eq!(t.work_list[0].bits_field.least_sig_bit, 0);
    assert_eq!(t.work_list[0].bits_field.num_bits, 3);
    assert!(matches!(t.work_list[1].field, FieldRef::Field { .. }));
    assert!(matches!(t.work_list[2].field, FieldRef::Hole));
    assert_eq!(t.work_list[2].bits_field.least_sig_bit, 5);
    assert_eq!(t.work_list[2].bits_field.num_bits, 3);

    // follow_holes=false: only the field record, no holes at all.
    let mut t2 = BitFieldTransform::new(&dt, 0, false);
    t2.establish_fields(vn(), 1, false);
    assert_eq!(t2.work_list.len(), 1);
    assert!(matches!(t2.work_list[0].field, FieldRef::Field { .. }));
}
