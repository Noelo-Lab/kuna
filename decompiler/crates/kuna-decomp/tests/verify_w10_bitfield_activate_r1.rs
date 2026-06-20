//! Round-1 VERIFIER adversarial tests for `rport/w10-bitfield-activate`.
//!
//! These are authored by the independent verifier (per
//! docs/rust-port/verification.md, Adversarial pass) and target the spots the
//! hunt list flagged as most fragile for the bitfield-query primitive the
//! transforms read:
//!
//!  - R1: `BitFieldTriple::compare` must be a TOTAL strict-weak order under the
//!    big-endian byte branch (type.cc:932-949).  A non-total comparator silently
//!    corrupts `establishFields`'s `sort(overlap, BitFieldTriple::compare)` and,
//!    under Rust's debug `sort_by`, can panic ("comparator is not a total
//!    order").  We sort a pathological permutation and assert antisymmetry /
//!    transitivity hold across every pair.
//!  - R2: `find_matching_bit_field`'s binary search must follow the big-endian
//!    `overlapTest` sign across MULTIPLE bytes (a 5-field BE vector spanning two
//!    bytes), where the answer is only reached after the search narrows past the
//!    middle probe in the BE direction.  A port that kept the LE direction would
//!    match the middle by luck but miss the window extremes.
//!  - R3: the `upper_bound`/`compareMaxByte` boundary is STRICT (`off < off+sz`):
//!    a field whose container ends exactly at the query offset must be excluded.
//!    An off-by-one (`<=`) would wrongly include it and shift every subsequent
//!    overlap classification.
//!
//! The primitive is the only surface reachable without the (unwired) transform
//! pass; these pin the arithmetic the `ptr->field` render ultimately depends on.

use std::rc::Rc;

use kuna_decomp::bitfield::BitRange;
use kuna_decomp::dtype::{flags, type_metatype, BitFieldTriple, Datatype, DatatypeKind, TypeBitField};

fn bf(id: i32, byte_off: i32, byte_sz: i32, lsb: i32, nb: i32, big: bool) -> TypeBitField {
    let mut b = TypeBitField::new(
        id,
        nb,
        big,
        format!("f{id}"),
        Rc::new(Datatype::new(4, type_metatype::TYPE_UINT)),
    );
    b.byte_offset = byte_off;
    b.byte_size = byte_sz;
    b.least_sig_bit = lsb;
    b
}

fn struct_with(bitfield: Vec<TypeBitField>, size: i32) -> Datatype {
    let mut s = Datatype::new_with_align(size, 4, type_metatype::TYPE_STRUCT);
    if !bitfield.is_empty() {
        s.flags |= flags::has_bitfields;
    }
    s.kind = DatatypeKind::Struct { field: vec![], bitfield };
    s
}

// ---------------------------------------------------------------------------
// R1 — BitFieldTriple::compare is a TOTAL strict-weak order (both endians).
// Antisymmetry: never compare(a,b) && compare(b,a).  Irreflexivity:
// !compare(a,a).  Transitivity of the derived equivalence + order.  And the
// std debug sort (which asserts totality) must not panic on a pathological
// permutation.
// ---------------------------------------------------------------------------
#[test]
fn r1_triple_compare_is_total_strict_weak_order_both_endians() {
    for big in [false, true] {
        // Fields across two container bytes plus same-byte lsb variants — every
        // branch of compare (byte term both directions, lsb tie-break, exact tie).
        let triples: Vec<BitFieldTriple> = vec![
            BitFieldTriple::new(bf(0, 0, 1, 0, 2, big), 0),
            BitFieldTriple::new(bf(1, 0, 1, 2, 2, big), 0),
            BitFieldTriple::new(bf(2, 1, 1, 0, 4, big), 0),
            BitFieldTriple::new(bf(3, 1, 1, 4, 4, big), 0),
            // A duplicate-keyed pair (same byteOffset+lsb) to exercise the
            // "neither strictly less" equivalence tie.
            BitFieldTriple::new(bf(4, 0, 1, 0, 2, big), 0),
        ];
        // Irreflexive + antisymmetric over every ordered pair.
        for a in &triples {
            assert!(
                !BitFieldTriple::compare(a, a),
                "compare must be irreflexive (big={big})"
            );
            for b in &triples {
                let ab = BitFieldTriple::compare(a, b);
                let ba = BitFieldTriple::compare(b, a);
                assert!(
                    !(ab && ba),
                    "compare must be antisymmetric (big={big}): a={} b={}",
                    a.bitfield.ident,
                    b.bitfield.ident
                );
            }
        }
        // Transitivity: a<b and b<c => a<c, across all triples-of-triples.
        for a in &triples {
            for b in &triples {
                for c in &triples {
                    if BitFieldTriple::compare(a, b) && BitFieldTriple::compare(b, c) {
                        assert!(
                            BitFieldTriple::compare(a, c),
                            "compare must be transitive (big={big})"
                        );
                    }
                }
            }
        }
        // The std debug sort asserts totality — a non-total comparator panics here.
        let mut perm = vec![
            triples[3].clone(),
            triples[0].clone(),
            triples[4].clone(),
            triples[2].clone(),
            triples[1].clone(),
        ];
        perm.sort_by(|a, b| {
            if BitFieldTriple::compare(a, b) {
                std::cmp::Ordering::Less
            } else if BitFieldTriple::compare(b, a) {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Equal
            }
        });
        // Result is monotone non-decreasing under compare (no later < earlier).
        for w in perm.windows(2) {
            assert!(
                !BitFieldTriple::compare(&w[1], &w[0]),
                "sorted order must be monotone (big={big})"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// R2 — find_matching_bit_field binary search across TWO bytes, big-endian.
// 5 fields: byte 0 holds {lsb0:2, lsb2:2, lsb4:4}, byte 1 holds {lsb0:4, lsb4:4}.
// In BE the stored vector is most-significant-first; the binary search must use
// the BE overlapTest sign to narrow.  Every field must be found, and a query for
// a field that only exists in the OTHER byte must not collide.
// ---------------------------------------------------------------------------
#[test]
fn r2_find_matching_be_multibyte_window_narrowing() {
    // Build BE layout: within a byte, higher lsb is MORE significant; across
    // bytes, lower byteOffset is MORE significant (BE memory order).  The stored
    // order produced by assign_contiguous_bitfields is most- to least-significant
    // in memory: byte0 {lsb4,lsb2,lsb0} then byte1 {lsb4,lsb0}.
    let v = vec![
        bf(0, 0, 1, 4, 4, true), // byte0 lsb4
        bf(1, 0, 1, 2, 2, true), // byte0 lsb2
        bf(2, 0, 1, 0, 2, true), // byte0 lsb0
        bf(3, 1, 1, 4, 4, true), // byte1 lsb4
        bf(4, 1, 1, 0, 4, true), // byte1 lsb0
    ];
    let s = struct_with(v, 4);

    // Each (byteOffset, lsb, nb) must resolve to its own field, never a neighbour.
    let cases = [
        (0i32, 4i32, 4i32, 0i32),
        (0, 2, 2, 1),
        (0, 0, 2, 2),
        (1, 4, 4, 3),
        (1, 0, 4, 4),
    ];
    for (byte_off, lsb, nb, want_id) in cases {
        // The query range carries the container byteOffset/byteSize so overlapTest
        // sees the right byte; in_container/translate_lsb does the rest.
        let range = BitRange::new(byte_off, 1, lsb, nb, true);
        let m = s
            .find_matching_bit_field(&range)
            .unwrap_or_else(|| panic!("BE multibyte search must find byte{byte_off} lsb{lsb}"));
        assert_eq!(
            m.ident, want_id,
            "BE search returned the wrong field for byte{byte_off} lsb{lsb}"
        );
        assert_eq!((m.byte_offset, m.least_sig_bit, m.num_bits), (byte_off, lsb, nb));
    }
    // A partial range straddling byte0 lsb2 and lsb4 must return None (break on
    // overlapTest==4), not the nearest field.
    assert!(
        s.find_matching_bit_field(&BitRange::new(0, 1, 3, 2, true)).is_none(),
        "straddling query must not match a partial field"
    );
}

// ---------------------------------------------------------------------------
// R3 — compareMaxByte upper_bound boundary is STRICT.  A field whose container
// ends EXACTLY at the query offset (off == byteOffset+byteSize) must be excluded
// by upper_bound; the iteration starts at the first field whose container end is
// strictly past `off`.  An off-by-one (`<=`) would include the ended field and
// mis-shift every subsequent overlap classification.
// ---------------------------------------------------------------------------
#[test]
fn r3_collect_upper_bound_boundary_is_strict() {
    // Three 1-byte containers at byteOffset 0,1,2.  Field 0 ends at byte 1.
    let v = vec![
        bf(0, 0, 1, 0, 8, false),
        bf(1, 1, 1, 0, 8, false),
        bf(2, 2, 1, 0, 8, false),
    ];
    let s = struct_with(v, 4);

    // Query offset 1, size 1: field0's container ends at exactly 1 -> compareMaxByte
    // (1 < 0+1) is FALSE, so upper_bound skips field0 and starts at field1.
    // The collected set must be exactly {field1}, never {field0, field1}.
    let mut res = Vec::new();
    s.collect_bit_fields(0, &mut res, 1, 1);
    assert_eq!(res.len(), 1, "field ending exactly at the query offset is excluded");
    assert_eq!(res[0].bitfield.ident, 1, "iteration starts at the first not-yet-ended field");

    // Query offset 0 must include field0 (ends at 1 > 0) and break at field1
    // (byte 1 entirely above [0,1)).
    let mut res0 = Vec::new();
    s.collect_bit_fields(0, &mut res0, 0, 1);
    assert_eq!(res0.len(), 1);
    assert_eq!(res0[0].bitfield.ident, 0);

    // has_bit_fields_in_range agrees with the strict boundary at every offset.
    assert!(s.has_bit_fields_in_range(0, 1));
    assert!(s.has_bit_fields_in_range(1, 1));
    assert!(s.has_bit_fields_in_range(2, 1));
    assert!(!s.has_bit_fields_in_range(3, 1), "no container past byte 2");
}
