//! Tests for the bitfield placement/recovery machinery (item w6-s5-bitfield).
//!
//! The "extraction/placement matrix" is exercised against ground-truth values
//! generated from the C++ `BitRange` (address.cc:630-868) so the port is pinned
//! field-by-field.  The rule layer is checked for op-list and `specs()`
//! definition-order fidelity (the part that is output-determining even while the
//! transform hand-off is a recorded W6 seam).

use super::*;
use crate::action::Rule;

// =============================================================================
// BitRange: extraction/placement matrix
// =============================================================================

#[test]
fn bitrange_byte_range_ctor() {
    let r = BitRange::byte_range(0, 8, false);
    assert_eq!(r.byte_offset, 0);
    assert_eq!(r.byte_size, 8);
    assert_eq!(r.least_sig_bit, 0);
    assert_eq!(r.num_bits, 64);
    assert!(!r.is_big_endian);
}

#[test]
fn bitrange_undefined_sentinel() {
    let r = BitRange::undefined();
    assert_eq!(r.byte_offset, -1);
    assert_eq!(r.byte_size, -1);
    assert_eq!(r.least_sig_bit, -1);
    assert_eq!(r.num_bits, -1);
    assert!(!r.is_big_endian);
}

#[test]
fn bitrange_empty() {
    assert!(BitRange::new(0, 8, 0, 0, false).empty());
    assert!(BitRange::new(0, 8, 0, -1, false).empty());
    assert!(!BitRange::new(0, 8, 0, 1, false).empty());
}

#[test]
fn bitrange_get_mask() {
    // address.cc:816-829 — oracle from /tmp/bitcheck.
    assert_eq!(BitRange::byte_range(0, 8, false).get_mask(), 0xffff_ffff_ffff_ffff);
    assert_eq!(BitRange::new(0, 8, 4, 5, false).get_mask(), 0x1f0);
    // numBits >= 64 => res starts at 0, then -1 (all ones), then << leastSigBit.
    assert_eq!(BitRange::new(0, 8, 0, 64, false).get_mask(), 0xffff_ffff_ffff_ffff);
    assert_eq!(BitRange::new(0, 8, 4, 64, false).get_mask(), 0xffff_ffff_ffff_fff0);
}

#[test]
fn bitrange_translate_lsb() {
    // address.cc:660-673 — oracle from /tmp/bitcheck.
    let le_cont = BitRange::byte_range(0, 8, false);
    let le_field = BitRange::new(2, 2, 0, 16, false);
    assert_eq!(le_cont.translate_lsb(&le_field), 16);

    let be_cont = BitRange::byte_range(0, 8, true);
    let be_field = BitRange::new(2, 2, 0, 16, true);
    assert_eq!(be_cont.translate_lsb(&be_field), 32);
}

#[test]
fn bitrange_in_container() {
    // address.cc:630-638 — oracle from /tmp/bitcheck2.
    let le_field = BitRange::new(2, 2, 0, 16, false);
    let le = BitRange::in_container(&le_field, 0, 8);
    assert_eq!((le.byte_offset, le.byte_size, le.least_sig_bit, le.num_bits), (0, 8, 16, 16));

    let be_field = BitRange::new(2, 2, 0, 16, true);
    let be = BitRange::in_container(&be_field, 0, 8);
    assert_eq!((be.byte_offset, be.byte_size, be.least_sig_bit, be.num_bits), (0, 8, 32, 16));
    assert!(be.is_big_endian);
}

#[test]
fn bitrange_overlap_test_little_endian() {
    // address.cc:685-704 — oracle from /tmp/bitcheck.
    let x = BitRange::new(0, 4, 0, 8, false); // bits [0,8)
    let y = BitRange::new(0, 4, 8, 8, false); // bits [8,16)
    assert_eq!(x.overlap_test(&x), 0); // identical
    assert_eq!(x.overlap_test(&y), -1); // x before y (LE)

    let z = BitRange::new(0, 4, 4, 8, false); // bits [4,12)
    assert_eq!(x.overlap_test(&z), 4); // partial
    assert_eq!(z.overlap_test(&x), 4); // partial

    let big = BitRange::new(0, 4, 0, 16, false);
    let small = BitRange::new(0, 4, 4, 4, false);
    assert_eq!(big.overlap_test(&small), 3); // small contained in big
    assert_eq!(small.overlap_test(&big), 2); // small contained in big
}

#[test]
fn bitrange_overlap_test_big_endian() {
    // address.cc:691-694 — oracle from /tmp/bitcheck.
    let be1 = BitRange::new(0, 4, 0, 8, true);
    let be2 = BitRange::new(0, 4, 8, 8, true);
    assert_eq!(be1.overlap_test(&be2), 1); // be1 after be2 (BE branch)
}

#[test]
fn bitrange_intersection() {
    // address.cc:709-726 — oracle from /tmp/bitcheck3.
    let mut a = BitRange::new(0, 8, 8, 16, false); // [8,24)
    a.intersection(&BitRange::new(0, 8, 12, 8, false)); // [12,20)
    assert_eq!((a.least_sig_bit, a.num_bits), (12, 8));

    let mut b = BitRange::new(0, 8, 0, 8, false); // [0,8)
    b.intersection(&BitRange::new(0, 8, 16, 8, false)); // [16,24) disjoint
    assert_eq!((b.least_sig_bit, b.num_bits), (0, 0));
}

#[test]
fn bitrange_intersect_mask() {
    // address.cc:731-750 — oracle from /tmp/bitcheck.
    let mut c = BitRange::new(0, 8, 0, 64, false);
    c.intersect_mask(0x00f0);
    assert_eq!((c.least_sig_bit, c.num_bits), (4, 4));

    let mut c2 = BitRange::new(0, 8, 0, 64, false);
    c2.intersect_mask(0x0);
    assert_eq!((c2.least_sig_bit, c2.num_bits), (0, 0));

    let mut c3 = BitRange::new(0, 8, 8, 16, false); // [8,24)
    c3.intersect_mask(0x00ff_0000); // bits [16,24)
    assert_eq!((c3.least_sig_bit, c3.num_bits), (16, 8));
}

#[test]
fn bitrange_shift() {
    // address.cc:754-770 — oracle from /tmp/bitcheck.
    let mut s1 = BitRange::new(0, 8, 0, 8, false);
    s1.shift(4);
    assert_eq!((s1.least_sig_bit, s1.num_bits), (4, 8));

    let mut s2 = BitRange::new(0, 8, 0, 8, false);
    s2.shift(-4);
    assert_eq!((s2.least_sig_bit, s2.num_bits), (0, 4));

    // shift past container end (1-byte container) clamps numBits.
    let mut s3 = BitRange::new(0, 1, 0, 8, false);
    s3.shift(4);
    assert_eq!((s3.least_sig_bit, s3.num_bits), (4, 4));
}

#[test]
fn bitrange_truncate_bytes() {
    // address.cc:774-802 — oracle from /tmp/bitcheck2.
    let mut tm = BitRange::new(0, 4, 8, 16, false);
    tm.truncate_most_sig_bytes(1);
    assert_eq!((tm.byte_offset, tm.byte_size, tm.least_sig_bit, tm.num_bits), (0, 3, 8, 16));

    let mut tl = BitRange::new(0, 4, 8, 16, false);
    tl.truncate_least_sig_bytes(1);
    assert_eq!((tl.byte_offset, tl.byte_size, tl.least_sig_bit, tl.num_bits), (1, 3, 0, 16));

    let mut tmb = BitRange::new(0, 4, 8, 16, true);
    tmb.truncate_most_sig_bytes(1);
    assert_eq!((tmb.byte_offset, tmb.byte_size, tmb.least_sig_bit, tmb.num_bits), (1, 3, 8, 16));

    let mut tlb = BitRange::new(0, 4, 8, 16, true);
    tlb.truncate_least_sig_bytes(1);
    assert_eq!((tlb.byte_offset, tlb.byte_size, tlb.least_sig_bit, tlb.num_bits), (0, 3, 0, 16));
}

#[test]
fn bitrange_extend_bytes() {
    // address.cc:806-812 — oracle from /tmp/bitcheck2.
    let mut e = BitRange::new(2, 2, 0, 16, false);
    e.extend_bytes(2);
    assert_eq!((e.byte_offset, e.byte_size), (2, 4));

    let mut eb = BitRange::new(2, 2, 0, 16, true);
    eb.extend_bytes(2);
    assert_eq!((eb.byte_offset, eb.byte_size), (0, 4));
}

#[test]
fn bitrange_is_byte_range() {
    // address.cc:832-838 — oracle from /tmp/bitcheck2.
    assert!(BitRange::new(0, 4, 8, 16, false).is_byte_range());
    assert!(!BitRange::new(0, 4, 4, 16, false).is_byte_range());
    assert!(!BitRange::new(0, 4, 8, 7, false).is_byte_range());
}

#[test]
fn bitrange_is_most_significant() {
    // address.cc:841-845 — oracle from /tmp/bitcheck2.
    assert!(BitRange::new(0, 4, 24, 8, false).is_most_significant());
    assert!(!BitRange::new(0, 4, 20, 8, false).is_most_significant());
}

#[test]
fn bitrange_minimize_container() {
    // address.cc:847-862 — oracle from /tmp/bitcheck.
    let mut m1 = BitRange::new(0, 8, 16, 8, false);
    m1.minimize_container();
    assert_eq!((m1.byte_offset, m1.byte_size, m1.least_sig_bit, m1.num_bits), (2, 1, 0, 8));

    let mut m2 = BitRange::new(0, 8, 20, 5, false);
    m2.minimize_container();
    assert_eq!((m2.byte_offset, m2.byte_size, m2.least_sig_bit, m2.num_bits), (2, 2, 4, 5));
}

#[test]
fn bitrange_expand_to_most() {
    // address.cc:864-868.
    let mut r = BitRange::new(0, 4, 8, 4, false);
    r.expand_to_most();
    // numBits = 8*4 - 8 = 24.
    assert_eq!(r.num_bits, 24);
}

#[test]
fn bitrange_compare_total_order() {
    // address.cc:643-655 — total order over (byteOffset, byteSize, leastSigBit,
    // numBits).  Each field is a strict tie-break in declaration order.
    let base = BitRange::new(1, 2, 3, 4, false);
    assert_eq!(base.compare(&base), 0);
    // byteOffset dominates.
    assert_eq!(base.compare(&BitRange::new(2, 0, 0, 0, false)), -1);
    assert_eq!(base.compare(&BitRange::new(0, 9, 9, 9, false)), 1);
    // byteSize tie-break.
    assert_eq!(base.compare(&BitRange::new(1, 3, 0, 0, false)), -1);
    assert_eq!(base.compare(&BitRange::new(1, 1, 9, 9, false)), 1);
    // leastSigBit tie-break.
    assert_eq!(base.compare(&BitRange::new(1, 2, 4, 0, false)), -1);
    assert_eq!(base.compare(&BitRange::new(1, 2, 2, 9, false)), 1);
    // numBits tie-break.
    assert_eq!(base.compare(&BitRange::new(1, 2, 3, 5, false)), -1);
    assert_eq!(base.compare(&BitRange::new(1, 2, 3, 3, false)), 1);
    // isBigEndian is NOT part of compare (matches C++).
    assert_eq!(base.compare(&BitRange::new(1, 2, 3, 4, true)), 0);
}

// =============================================================================
// BitFieldNodeState
// =============================================================================

/// A null `VarnodeId` handle — the `BitFieldNodeState` tests exercise only the
/// pure bit-range arithmetic, never dereferencing `node`.
fn vn() -> VarnodeId {
    VarnodeId::default()
}

/// Build a `TypeBitField` whose `bits()` matches `field_bits` and whose
/// underlying type has the given metatype.  The transforms only read
/// `field->type->getMetatype()` and `field->bits.numBits`/range off the field.
fn bf(field_bits: &BitRange, meta: type_metatype) -> TypeBitField {
    use crate::dtype::TypeBitField as TBF;
    let mut f = TBF::new(0, field_bits.num_bits, field_bits.is_big_endian, "f",
        Rc::new(Datatype::new(field_bits.byte_size.max(1), meta)));
    f.byte_offset = field_bits.byte_offset;
    f.byte_size = field_bits.byte_size;
    f.least_sig_bit = field_bits.least_sig_bit;
    f
}

#[test]
fn node_state_follow_field() {
    // bitfield.cc:21-28.  used container (off0,sz8); field bits (off2,sz2,lsb0,nb16).
    // bitsField = BitRange(field.bits, 0, 8) => lsb 16, nb 16 (LE oracle).
    let used = BitRange::byte_range(0, 8, false);
    let field_bits = BitRange::new(2, 2, 0, 16, false);
    let st = BitFieldNodeState::follow_field(&used, vn(), &bf(&field_bits, type_metatype::TYPE_INT));
    assert_eq!((st.bits_field.least_sig_bit, st.bits_field.num_bits), (16, 16));
    assert_eq!(st.orig_least_sig_bit, 16);
    // isMostSignificant of bitsField (8*8=64 == 16+16=32?) -> false, so no sign ext.
    assert!(!st.is_sign_extended);
    assert_eq!(st.field().unwrap().field_type.get_metatype(), type_metatype::TYPE_INT);
    // does_sign_extension_match: is_sign_extended(false) == is_int(true) -> false.
    assert!(!st.does_sign_extension_match());
}

#[test]
fn node_state_follow_field_top_aligned_signext() {
    // A field occupying the top of the container with is_int -> sign-extended.
    let used = BitRange::byte_range(0, 8, false);
    // field bits that translate to bitsField lsb 48 nb 16 in an 8-byte container:
    // a field at byteOffset 6, byteSize 2 (LE) -> translateLSB = 8*(6-0)=48.
    let field_bits = BitRange::new(6, 2, 0, 16, false);
    let st = BitFieldNodeState::follow_field(&used, vn(), &bf(&field_bits, type_metatype::TYPE_INT));
    assert_eq!((st.bits_field.least_sig_bit, st.bits_field.num_bits), (48, 16));
    assert!(st.bits_field.is_most_significant()); // 8*8 == 48+16
    assert!(st.is_sign_extended);
    assert!(st.does_sign_extension_match());
}

#[test]
fn node_state_follow_field_unsigned_no_signext() {
    let used = BitRange::byte_range(0, 8, false);
    let field_bits = BitRange::new(6, 2, 0, 16, false);
    let st = BitFieldNodeState::follow_field(&used, vn(), &bf(&field_bits, type_metatype::TYPE_UINT));
    // is_int false -> never sign extended even if most significant.
    assert!(!st.is_sign_extended);
    assert_eq!(st.field().unwrap().field_type.get_metatype(), type_metatype::TYPE_UINT);
    // does_sign_extension_match: false == false -> true.
    assert!(st.does_sign_extension_match());
}

#[test]
fn node_state_hole() {
    // bitfield.cc:30-37.  used (off0,sz8,LE); hole at leastSig 12, numBits 4.
    let used = BitRange::byte_range(0, 8, false);
    let st = BitFieldNodeState::hole(&used, vn(), 12, 4);
    assert_eq!(
        (
            st.bits_field.byte_offset,
            st.bits_field.byte_size,
            st.bits_field.least_sig_bit,
            st.bits_field.num_bits
        ),
        (0, 8, 12, 4)
    );
    assert_eq!(st.orig_least_sig_bit, 12);
    assert!(!st.is_sign_extended);
    assert!(st.field.is_hole());
    // does_sign_extension_match defensively false on a hole.
    assert!(!st.does_sign_extension_match());
}

#[test]
fn node_state_with_new_field() {
    // bitfield.cc:44-51.  field/origLeastSigBit copied; bitsField/node/signExt replaced.
    let used = BitRange::byte_range(0, 8, false);
    let field_bits = BitRange::new(2, 2, 0, 16, false);
    let base = BitFieldNodeState::follow_field(&used, vn(), &bf(&field_bits, type_metatype::TYPE_INT));
    let new_field = BitRange::new(0, 8, 0, 8, false);
    let st = BitFieldNodeState::with_new_field(&base, &new_field, vn(), /*sgn_ext*/ true);
    assert_eq!(st.bits_field, new_field);
    // field is copied from base (both follow the same TypeBitField).
    assert_eq!(
        st.field().unwrap().field_type.get_metatype(),
        base.field().unwrap().field_type.get_metatype()
    );
    assert_eq!(st.orig_least_sig_bit, base.orig_least_sig_bit);
    assert!(st.is_sign_extended);
    assert_eq!(st.bits_used, base.bits_used);
}

#[test]
fn node_state_is_field_aligned() {
    // bitfield.hh:39 — bitsField.leastSigBit == 0 && bitsField.numBits == bitsUsed.numBits.
    let used = BitRange::byte_range(0, 1, false); // numBits 8
    // Aligned: bitsField lsb 0 nb 8.
    let aligned = BitFieldNodeState::hole(&used, vn(), 0, 8);
    assert!(aligned.is_field_aligned());
    // Not aligned (lsb != 0).
    let off = BitFieldNodeState::hole(&used, vn(), 2, 6);
    assert!(!off.is_field_aligned());
    // Not aligned (numBits mismatch).
    let short = BitFieldNodeState::hole(&used, vn(), 0, 4);
    assert!(!short.is_field_aligned());
}

// =============================================================================
// BitFieldTransform (base): constructor + establish_fields
// =============================================================================

use crate::dtype::{flags, type_metatype, Datatype, DatatypeKind, TypeBitField};
use std::rc::Rc;

/// `myfoo`'s first byte (`bitfields.xml`): `uint4 field3:3` (lsb0,3b),
/// `int4 sfield4:4` (lsb3,4b), `bool fieldb:1` (lsb7,1b).  All little-endian,
/// byteOffset 0, byteSize 1.
fn myfoo_byte0_struct() -> Rc<Datatype> {
    let mk = |id, lsb, nb, name: &str, meta| {
        let mut bf = TypeBitField::new(id, nb, false, name, Rc::new(Datatype::new(4, meta)));
        bf.least_sig_bit = lsb;
        bf.byte_offset = 0;
        bf.byte_size = 1;
        bf
    };
    let bitfield = vec![
        mk(0, 0, 3, "field3", type_metatype::TYPE_UINT),
        mk(1, 3, 4, "sfield4", type_metatype::TYPE_INT),
        mk(2, 7, 1, "fieldb", type_metatype::TYPE_BOOL),
    ];
    let mut s = Datatype::new_with_align(1, 4, type_metatype::TYPE_STRUCT);
    s.flags |= flags::has_bitfields;
    s.kind = DatatypeKind::Struct { field: vec![], bitfield };
    Rc::new(s)
}

#[test]
fn transform_ctor_struct_sets_parent_and_offset() {
    let dt = myfoo_byte0_struct();
    let t = BitFieldTransform::new(&dt, 0, false);
    assert!(t.parent_struct.is_some());
    assert_eq!(t.initial_offset, 0);
    assert_eq!(t.container_size, -1);
    assert!(!t.is_big_endian);
}

#[test]
fn transform_ctor_non_struct_leaves_parent_null() {
    let dt = Rc::new(Datatype::new(4, type_metatype::TYPE_INT));
    let t = BitFieldTransform::new(&dt, 0, false);
    assert!(t.parent_struct.is_none());
    assert_eq!(t.initial_offset, -1);
}

#[test]
fn establish_fields_one_byte_no_holes() {
    // A 1-byte Varnode fully covered by field3(3)+sfield4(4)+fieldb(1) = 8 bits.
    // With follow_holes=false, only the three field records appear.
    let dt = myfoo_byte0_struct();
    let mut t = BitFieldTransform::new(&dt, 0, false);
    t.establish_fields(vn(), /*vn_size*/ 1, /*follow_holes*/ false);
    // Three field records, each a FieldRef::Field, none a hole.
    assert_eq!(t.work_list.len(), 3);
    assert!(t.work_list.iter().all(|s| matches!(s.field, FieldRef::Field(_))));
    // field3 occupies bits [0,3); least significant first.
    assert_eq!(t.work_list[0].bits_field.least_sig_bit, 0);
    assert_eq!(t.work_list[0].bits_field.num_bits, 3);
    // sfield4 [3,7).
    assert_eq!(t.work_list[1].bits_field.least_sig_bit, 3);
    assert_eq!(t.work_list[1].bits_field.num_bits, 4);
    // fieldb [7,8) — most significant, is_int? bool -> not TYPE_INT -> no signext.
    assert_eq!(t.work_list[2].bits_field.least_sig_bit, 7);
    assert_eq!(t.work_list[2].bits_field.num_bits, 1);
    // sfield4 is int4 -> FieldRef::Field { is_int: true }; its bits are not at the
    // top of the 1-byte container ([3,7) != most significant), so not sign ext.
    assert_eq!(t.work_list[1].field().unwrap().field_type.get_metatype(), type_metatype::TYPE_INT);
    assert!(!t.work_list[1].is_sign_extended);
}

#[test]
fn establish_fields_partial_coverage_emits_final_hole() {
    // Only field3(3 bits) present in a struct; a 1-byte Varnode leaves a 5-bit
    // final hole [3,8).  With follow_holes the hole record is emitted.
    let mut s = Datatype::new_with_align(1, 4, type_metatype::TYPE_STRUCT);
    s.flags |= flags::has_bitfields;
    let mut bf = TypeBitField::new(0, 3, false, "field3", Rc::new(Datatype::new(4, type_metatype::TYPE_UINT)));
    bf.byte_offset = 0;
    bf.byte_size = 1;
    s.kind = DatatypeKind::Struct { field: vec![], bitfield: vec![bf] };
    let dt = Rc::new(s);

    let mut t = BitFieldTransform::new(&dt, 0, true);
    t.establish_fields(vn(), 1, /*follow_holes*/ true);
    // field3 record + the final hole [3,8).
    assert_eq!(t.work_list.len(), 2);
    assert!(matches!(t.work_list[0].field, FieldRef::Field(_)));
    assert!(matches!(t.work_list[1].field, FieldRef::Hole));
    assert_eq!(t.work_list[1].bits_field.least_sig_bit, 3);
    assert_eq!(t.work_list[1].bits_field.num_bits, 5);

    // With follow_holes=false the hole is suppressed.
    let mut t2 = BitFieldTransform::new(&dt, 0, true);
    t2.establish_fields(vn(), 1, false);
    assert_eq!(t2.work_list.len(), 1);
    assert!(matches!(t2.work_list[0].field, FieldRef::Field(_)));
}

#[test]
fn establish_fields_non_struct_parent_collects_nothing() {
    // C++ never calls establishFields with a null parentStruct (the rule guards
    // on dt->hasBitfields() first, which implies a struct).  The port guards the
    // collectBitFields call defensively, so no field records are produced — the
    // overlap loop runs zero times.  With follow_holes=true the trailing
    // whole-Varnode hole branch (bitfield.cc:88-90) still fires, exactly as it
    // would for an empty struct (no bitfields).
    let dt = Rc::new(Datatype::new(4, type_metatype::TYPE_INT));
    let mut t = BitFieldTransform::new(&dt, 0, false);
    t.establish_fields(vn(), 4, false);
    // follow_holes=false: nothing at all.
    assert!(t.work_list.is_empty());

    let mut t2 = BitFieldTransform::new(&dt, 0, false);
    t2.establish_fields(vn(), 4, true);
    // follow_holes=true: a single whole-Varnode hole [0, 32).
    assert_eq!(t2.work_list.len(), 1);
    assert!(matches!(t2.work_list[0].field, FieldRef::Hole));
    assert_eq!(t2.work_list[0].bits_field.least_sig_bit, 0);
    assert_eq!(t2.work_list[0].bits_field.num_bits, 32);
}

// =============================================================================
// Rules: op-list + specs() definition-order fidelity
// =============================================================================

#[test]
fn rule_op_lists_match_cpp() {
    assert_eq!(RuleBitFieldStore.get_op_list(), vec![OpCode::CPUI_STORE]);
    assert_eq!(RuleBitFieldLoad.get_op_list(), vec![OpCode::CPUI_LOAD]);
    assert_eq!(
        RulePullAbsorb.get_op_list(),
        vec![OpCode::CPUI_ZPULL, OpCode::CPUI_SPULL]
    );
    assert_eq!(RuleInsertAbsorb.get_op_list(), vec![OpCode::CPUI_INSERT]);
}

#[test]
fn rule_bitfield_in_op_list_exact_order() {
    // bitfield.cc:1739-1745 — the explicit 17-entry list, in exactly that order.
    assert_eq!(
        RuleBitFieldIn.get_op_list(),
        vec![
            OpCode::CPUI_COPY,
            OpCode::CPUI_INT_EQUAL,
            OpCode::CPUI_INT_NOTEQUAL,
            OpCode::CPUI_INT_SLESS,
            OpCode::CPUI_INT_SLESSEQUAL,
            OpCode::CPUI_INT_LESS,
            OpCode::CPUI_INT_LESSEQUAL,
            OpCode::CPUI_INT_ZEXT,
            OpCode::CPUI_INT_SEXT,
            OpCode::CPUI_INT_ADD,
            OpCode::CPUI_INT_NEGATE,
            OpCode::CPUI_INT_AND,
            OpCode::CPUI_INT_LEFT,
            OpCode::CPUI_INT_RIGHT,
            OpCode::CPUI_INT_SRIGHT,
            OpCode::CPUI_INT_MULT,
            OpCode::CPUI_SUBPIECE,
        ]
    );
}

#[test]
fn rule_bitfield_out_op_list_is_allowed_writes_plus_indirect() {
    // bitfield.cc:1694-1699 — allowedFinalWrites, then CPUI_INDIRECT last.
    let oplist = RuleBitFieldOut.get_op_list();
    assert_eq!(oplist.len(), ALLOWED_FINAL_WRITES.len() + 1);
    assert_eq!(&oplist[..ALLOWED_FINAL_WRITES.len()], ALLOWED_FINAL_WRITES);
    assert_eq!(*oplist.last().unwrap(), OpCode::CPUI_INDIRECT);
    // Spot-check the head/tail of allowedFinalWrites itself.
    assert_eq!(oplist[0], OpCode::CPUI_COPY);
    assert_eq!(ALLOWED_FINAL_WRITES[0], OpCode::CPUI_COPY);
    assert_eq!(*ALLOWED_FINAL_WRITES.last().unwrap(), OpCode::CPUI_SUBPIECE);
}

#[test]
fn specs_definition_order() {
    // bitfield.hh declaration order: store, out, load, in, pull_absorb, insert_absorb.
    let specs = specs();
    assert_eq!(specs.len(), 6);
    // Build each and check its op-list signature identifies the right rule, since
    // RuleSpec carries only group + ctor.
    let mk = |i: usize| (specs[i].ctor)();
    assert_eq!(mk(0).get_op_list(), vec![OpCode::CPUI_STORE]); // bitfield_store
    assert_eq!(mk(1).get_op_list().last().copied(), Some(OpCode::CPUI_INDIRECT)); // bitfield_out
    assert_eq!(mk(2).get_op_list(), vec![OpCode::CPUI_LOAD]); // bitfield_load
    assert_eq!(mk(3).get_op_list().len(), 17); // bitfield_in
    assert_eq!(mk(4).get_op_list(), vec![OpCode::CPUI_ZPULL, OpCode::CPUI_SPULL]); // pull_absorb
    assert_eq!(mk(5).get_op_list(), vec![OpCode::CPUI_INSERT]); // insert_absorb
    // All carry the empty group placeholder (W8 assigns the real stage group).
    for s in &specs {
        assert_eq!(s.group, "");
    }
}
